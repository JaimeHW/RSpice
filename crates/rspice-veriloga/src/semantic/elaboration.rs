//! Executable elaboration of structural Verilog-A module instances.
//!
//! Semantic analysis deliberately keeps every module independent.  This pass
//! selects a top module and flattens its instance tree into one analyzed model
//! before either executable backend sees it.  Keeping the pass here gives the
//! bytecode and canonical-IR paths exactly the same ports, parameters, state,
//! nodes, and equations.

use super::{
    AnalogSiteId, AnalyzedArray, AnalyzedAssignment, AnalyzedBranch, AnalyzedContribution,
    AnalyzedFile, AnalyzedInternalNode, AnalyzedLoop, AnalyzedModule, AnalyzedParameter,
    AnalyzedRegion, AnalyzedStatement, ConstantValue, MAX_PARAMETER_ARRAY_ELEMENTS,
    MAX_PARAMETER_ARRAY_RANK, SemanticAnalyzer,
};
use crate::ast::{
    AnalogOperator, ArrayAccessExpr, ArrayLiteralElement, ArrayLiteralExpr, BinaryExpr,
    BranchAccess, CallExpr, ConditionalExpr, Connection, Expression, Identifier, Item, Module,
    ModuleInstance, NoiseSource, NumberLit, SystemFunction, UnaryExpr,
};
use crate::error::{CompileError, CompileResult, SemanticError, SemanticErrorKind};
use crate::source::Span;
use smol_str::SmolStr;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

/// Return the selected module itself when it is structural-leaf, otherwise a
/// faithfully flattened owned module.  Unsupported or ambiguous structure is
/// rejected before code generation; it is never omitted.
pub(crate) fn elaborate_executable_module<'a>(
    analyzed: &'a AnalyzedFile,
    selected: &'a AnalyzedModule,
) -> CompileResult<Cow<'a, AnalyzedModule>> {
    // The selected module's own digital content is no longer refused here.
    // Processes have a canonical form, and each executable path now refuses
    // at the point it would have to run one — the bytecode IR builder because
    // it has no representation for a process at all, the Rust backend because
    // it emits a device the solver calls rather than a coroutine the event
    // kernel resumes.
    //
    // A *digital child* instance is no longer refused either: it is elaborated
    // by [`digital_elaborate`](super::digital_elaborate), which runs first so
    // that its refusals reach the author before any analog one. The two passes
    // partition the instance tree with one shared predicate — a child with
    // discrete-domain content belongs to that pass and to this one otherwise —
    // so no instance can be claimed by both, and none by neither. Below the
    // compiled module the analog flattening owns everything, and a digital
    // module found there is refused as it always was, because a mixed-signal
    // hierarchy has no elaborated form yet.
    let source_modules = source_modules(analyzed)?;
    let root = source_modules.get(&selected.name).copied().ok_or_else(|| {
        internal_error(format!(
            "selected module '{}' has no retained source module",
            selected.name
        ))
    })?;
    if root.instances.is_empty() {
        return Ok(Cow::Borrowed(selected));
    }

    let digital_instances = super::digital_elaborate::elaborate_digital_hierarchy(
        analyzed,
        &source_modules,
        root,
        selected,
    )?;

    let mut elaborator = HierarchyElaborator::new(analyzed, source_modules, selected.clone());
    elaborator.flattened.digital.instances = digital_instances;
    let root_scope = ScopeMap::for_root(selected);
    let mut module_stack = vec![selected.name.clone()];
    elaborator.append_instances(
        root,
        &root_scope,
        &mut module_stack,
        selected.name.as_str(),
        true,
    )?;
    Ok(Cow::Owned(elaborator.finish()))
}

fn source_modules<'a>(analyzed: &'a AnalyzedFile) -> CompileResult<HashMap<SmolStr, &'a Module>> {
    let mut modules = HashMap::new();
    for item in &analyzed.source.items {
        let Item::Module(module) = item else { continue };
        if let Some(previous) = modules.insert(module.name.clone(), module) {
            return Err(semantic_error(
                SemanticErrorKind::DuplicateSymbol {
                    name: module.name.clone(),
                    first_defined: previous.span,
                },
                module.span,
            ));
        }
    }
    Ok(modules)
}

#[derive(Clone)]
struct NodeBinding {
    name: SmolStr,
    discipline: Option<SmolStr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EffectiveParameterArrayShape {
    /// Ordered left/right bounds. Comparing these, rather than normalized
    /// extents alone, preserves an override that reverses an array's direction.
    bounds: Vec<(i64, i64)>,
    extents: Vec<u64>,
}

#[derive(Default)]
struct ScopeMap {
    nodes: HashMap<SmolStr, NodeBinding>,
    parameters: HashMap<SmolStr, SmolStr>,
    parameter_given: HashMap<SmolStr, bool>,
    variables: HashMap<SmolStr, SmolStr>,
    arrays: HashMap<SmolStr, SmolStr>,
    branches: HashMap<SmolStr, SmolStr>,
    port_connected: HashMap<SmolStr, bool>,
    instance_path: Option<SmolStr>,
    /// `(global base, local count)` for this concrete module occurrence.
    /// The root is not rewritten and therefore leaves this unset.
    noise_process_range: Option<(u32, u32)>,
}

impl ScopeMap {
    fn for_root(module: &AnalyzedModule) -> Self {
        let mut scope = Self::default();
        for port in &module.ports {
            scope.nodes.insert(
                port.name.clone(),
                NodeBinding {
                    name: port.name.clone(),
                    discipline: Some(port.discipline.clone()),
                },
            );
        }
        for node in &module.internal_nodes {
            scope.nodes.insert(
                node.name.clone(),
                NodeBinding {
                    name: node.name.clone(),
                    discipline: Some(node.discipline.clone()),
                },
            );
        }
        for ground in &module.ground_nodes {
            scope.nodes.insert(
                ground.clone(),
                NodeBinding {
                    name: ground.clone(),
                    discipline: None,
                },
            );
        }
        scope.nodes.insert(
            "0".into(),
            NodeBinding {
                name: "0".into(),
                discipline: None,
            },
        );
        for parameter in &module.parameters {
            scope
                .parameters
                .insert(parameter.name.clone(), parameter.name.clone());
        }
        for variable in &module.variables {
            scope
                .variables
                .insert(variable.name.clone(), variable.name.clone());
        }
        for array in module.arrays.keys() {
            scope.arrays.insert(array.clone(), array.clone());
        }
        for branch in &module.branches {
            scope
                .branches
                .insert(branch.name.clone(), branch.name.clone());
        }
        scope
    }
}

struct HierarchyElaborator<'a> {
    analyzed: &'a AnalyzedFile,
    source_modules: HashMap<SmolStr, &'a Module>,
    flattened: AnalyzedModule,
    used_names: HashSet<SmolStr>,
    next_name: usize,
    next_noise_process: u32,
}

impl<'a> HierarchyElaborator<'a> {
    fn new(
        analyzed: &'a AnalyzedFile,
        source_modules: HashMap<SmolStr, &'a Module>,
        flattened: AnalyzedModule,
    ) -> Self {
        let mut used_names = HashSet::new();
        used_names.extend(flattened.ports.iter().map(|item| item.name.clone()));
        used_names.extend(flattened.parameters.iter().map(|item| item.name.clone()));
        used_names.extend(flattened.variables.iter().map(|item| item.name.clone()));
        used_names.extend(
            flattened
                .internal_nodes
                .iter()
                .map(|item| item.name.clone()),
        );
        used_names.extend(flattened.ground_nodes.iter().cloned());
        used_names.extend(flattened.branches.iter().map(|item| item.name.clone()));
        used_names.extend(flattened.arrays.keys().cloned());
        let next_noise_process = flattened.noise_process_count;
        Self {
            analyzed,
            source_modules,
            flattened,
            used_names,
            next_name: 0,
            next_noise_process,
        }
    }

    fn finish(mut self) -> AnalyzedModule {
        self.flattened.noise_process_count = self.next_noise_process;
        self.flattened
    }

    /// Flatten one module's instances.
    ///
    /// `digital_children_elaborated` says whether a child with discrete-domain
    /// content is one [`super::digital_elaborate`] has already taken. It is
    /// true only for the compiled module's own instances — that pass walks the
    /// digital tree from there — and false everywhere below, where a digital
    /// module is refused rather than skipped, because nothing would elaborate
    /// it and skipping it would drop it.
    fn append_instances(
        &mut self,
        source_module: &Module,
        parent_scope: &ScopeMap,
        module_stack: &mut Vec<SmolStr>,
        parent_path: &str,
        digital_children_elaborated: bool,
    ) -> CompileResult<()> {
        let mut instance_names = HashSet::new();
        for instance in &source_module.instances {
            if !instance_names.insert(instance.name.clone()) {
                return Err(semantic_error(
                    SemanticErrorKind::DuplicateSymbol {
                        name: instance.name.clone(),
                        first_defined: instance.span,
                    },
                    instance.span,
                ));
            }
            let path = format!("{parent_path}.{}", instance.name);
            self.append_instance(
                instance,
                parent_scope,
                module_stack,
                &path,
                digital_children_elaborated,
            )?;
        }
        Ok(())
    }

    fn append_instance(
        &mut self,
        instance: &ModuleInstance,
        parent_scope: &ScopeMap,
        module_stack: &mut Vec<SmolStr>,
        path: &str,
        digital_children_elaborated: bool,
    ) -> CompileResult<()> {
        let child_source = self
            .source_modules
            .get(&instance.module)
            .copied()
            .ok_or_else(|| {
                semantic_error(
                    SemanticErrorKind::UndefinedModule(instance.module.to_string()),
                    instance.span,
                )
            })?;
        let child = self.analyzed.modules.get(&instance.module).ok_or_else(|| {
            internal_error(format!(
                "module '{}' was retained but not semantically analyzed",
                instance.module
            ))
        })?;
        // A digital child of the compiled module belongs to the digital
        // elaboration, which has already taken it. Anywhere else it is as
        // unexecutable as it ever was, and flattening would otherwise drop it
        // without a word.
        if super::digital_elaborate::is_digital_child(child) {
            if digital_children_elaborated {
                return Ok(());
            }
            super::reject_digital_content(child)?;
        }
        if module_stack.contains(&instance.module) {
            let mut cycle = module_stack.iter().map(SmolStr::as_str).collect::<Vec<_>>();
            cycle.push(instance.module.as_str());
            return Err(semantic_error(
                SemanticErrorKind::CircularDependency(format!(
                    "module hierarchy {} at instance '{path}'",
                    cycle.join(" -> ")
                )),
                instance.span,
            ));
        }

        let connections = self.bind_connections(instance, child, parent_scope, path)?;
        let overrides = bind_parameter_overrides(instance, child, path)?;
        self.validate_parameter_array_overrides(child, parent_scope, &overrides, path)?;
        let noise_process_base = self.next_noise_process;
        self.next_noise_process = self
            .next_noise_process
            .checked_add(child.noise_process_count)
            .ok_or_else(|| {
                internal_error(format!(
                    "module hierarchy at instance '{path}' exceeds the noise-process identity range"
                ))
            })?;
        let mut scope = ScopeMap {
            instance_path: Some(path.into()),
            noise_process_range: Some((noise_process_base, child.noise_process_count)),
            ..ScopeMap::default()
        };
        scope.nodes.insert(
            "0".into(),
            NodeBinding {
                name: "0".into(),
                discipline: None,
            },
        );
        for (port, connection) in child.ports.iter().zip(connections) {
            let (binding, connected) = match connection {
                Some(binding) => (binding, true),
                None => {
                    let name = self.fresh_name(&port.name);
                    let index = self.flattened.internal_nodes.len();
                    self.flattened.internal_nodes.push(AnalyzedInternalNode {
                        name: name.clone(),
                        discipline: port.discipline.clone(),
                        index,
                    });
                    (
                        NodeBinding {
                            name,
                            discipline: Some(port.discipline.clone()),
                        },
                        false,
                    )
                }
            };
            scope.nodes.insert(port.name.clone(), binding);
            scope.port_connected.insert(port.name.clone(), connected);
        }
        for ground in &child.ground_nodes {
            scope.nodes.insert(
                ground.clone(),
                NodeBinding {
                    name: "0".into(),
                    discipline: None,
                },
            );
        }
        for node in &child.internal_nodes {
            let name = self.fresh_name(&node.name);
            let index = self.flattened.internal_nodes.len();
            self.flattened.internal_nodes.push(AnalyzedInternalNode {
                name: name.clone(),
                discipline: node.discipline.clone(),
                index,
            });
            scope.nodes.insert(
                node.name.clone(),
                NodeBinding {
                    name,
                    discipline: Some(node.discipline.clone()),
                },
            );
        }

        // Allocate all parameter names before rewriting defaults and ranges;
        // this makes forward references diagnostic-preserving instead of
        // accidentally binding to a similarly named parent parameter.
        let parameter_base = self.flattened.parameters.len();
        for (index, parameter) in child.parameters.iter().enumerate() {
            let name = self.fresh_name(&parameter.name);
            scope.parameters.insert(parameter.name.clone(), name);
            scope
                .parameter_given
                .insert(parameter.name.clone(), overrides.contains_key(&index));
        }
        for (index, parameter) in child.parameters.iter().enumerate() {
            let mut parameter = parameter.clone();
            parameter.name = scope.parameters[&parameter.name].clone();
            parameter.is_public = false;
            parameter.default_expr = if let Some(override_expr) = overrides.get(&index) {
                parameter.default = None;
                Some(rewrite_expression(override_expr, parent_scope)?)
            } else {
                parameter
                    .default_expr
                    .as_ref()
                    .map(|expr| rewrite_expression(expr, &scope))
                    .transpose()?
            };
            for dimension in &mut parameter.dimensions {
                dimension.left = rewrite_expression(&dimension.left, &scope)?;
                dimension.right = rewrite_expression(&dimension.right, &scope)?;
            }
            if let Some(range) = &mut parameter.range {
                range.min_parameter =
                    mapped_optional_parameter(range.min_parameter.as_ref(), &scope, instance.span)?;
                range.max_parameter =
                    mapped_optional_parameter(range.max_parameter.as_ref(), &scope, instance.span)?;
                range.exclude_parameters = range
                    .exclude_parameters
                    .iter()
                    .map(|name| {
                        scope.parameters.get(name).cloned().ok_or_else(|| {
                            semantic_error(
                                SemanticErrorKind::UndeclaredSymbol { name: name.clone() },
                                instance.span,
                            )
                        })
                    })
                    .collect::<CompileResult<Vec<_>>>()?;
                range.min_expression = range
                    .min_expression
                    .as_ref()
                    .map(|expr| rewrite_expression(expr, &scope))
                    .transpose()?;
                range.max_expression = range
                    .max_expression
                    .as_ref()
                    .map(|expr| rewrite_expression(expr, &scope))
                    .transpose()?;
                range.exclude_expressions = range
                    .exclude_expressions
                    .iter()
                    .map(|expr| rewrite_expression(expr, &scope))
                    .collect::<CompileResult<Vec<_>>>()?;
            }
            self.flattened.parameters.push(parameter);
        }
        debug_assert_eq!(
            self.flattened.parameters.len(),
            parameter_base + child.parameters.len()
        );

        let variable_base = self.flattened.variables.len();
        for variable in &child.variables {
            let mut variable = variable.clone();
            let original = variable.name.clone();
            variable.name = self.fresh_name(&original);
            scope.variables.insert(original, variable.name.clone());
            self.flattened.variables.push(variable);
        }
        for &slot in &child.event_state_variables {
            if slot >= child.variables.len() {
                return Err(internal_error(format!(
                    "child module '{}' event-state variable slot {slot} exceeds its {} variable slots",
                    child.name,
                    child.variables.len()
                )));
            }
            self.flattened.event_state_variables.push(
                variable_base
                    .checked_add(slot)
                    .ok_or_else(|| internal_error("hierarchy event-state index overflow".into()))?,
            );
        }
        self.flattened.event_state_variables.sort_unstable();
        self.flattened.event_state_variables.dedup();
        // By name, because `fresh_name` draws from one counter shared by every
        // renamed item: walking the child's array map would hand a different
        // hoisted name to each array — and to everything renamed after it —
        // on every process that flattens this hierarchy.
        let mut child_arrays: Vec<_> = child.arrays.iter().collect();
        child_arrays.sort_unstable_by(|(left, _), (right, _)| left.cmp(right));
        for (name, array) in child_arrays {
            let mapped_name = self.fresh_name(name);
            scope.arrays.insert(name.clone(), mapped_name.clone());
            self.flattened.arrays.insert(
                mapped_name,
                AnalyzedArray {
                    base: variable_base + array.base,
                    lower: array.lower,
                    len: array.len,
                },
            );
        }
        for branch in &child.branches {
            let mapped_name = self.fresh_name(&branch.name);
            scope
                .branches
                .insert(branch.name.clone(), mapped_name.clone());
            self.flattened.branches.push(AnalyzedBranch {
                name: mapped_name,
                pos_node: mapped_node_name(&scope, &branch.pos_node, instance.span)?,
                neg_node: if branch.neg_node.is_empty() {
                    SmolStr::default()
                } else {
                    mapped_node_name(&scope, &branch.neg_node, instance.span)?
                },
                discipline: branch.discipline.clone(),
            });
        }

        // One base for both spaces, taken before anything is appended, so the
        // two lowerings of an inlined instance keep naming each other.
        let base = InstanceBase {
            variables: variable_base,
            sites: self.flattened.analog_site_count,
        };
        self.flattened.analog_site_count = self
            .flattened
            .analog_site_count
            .checked_add(child.analog_site_count)
            .ok_or_else(|| internal_error("hierarchy analog site count overflow".to_string()))?;
        // The child's statements are appended after everything already
        // flattened, so its prologue indices move by that many. Rebased before
        // the append, while the offset is still the parent's own length.
        let statement_base = self.flattened.statements.len();
        self.flattened.statements.extend(
            child
                .statements
                .iter()
                .map(|statement| rewrite_statement(statement, &scope, base))
                .collect::<CompileResult<Vec<_>>>()?,
        );
        self.flattened.prologue_statements.extend(
            child
                .prologue_statements
                .iter()
                .map(|index| index + statement_base),
        );
        self.flattened.body.extend(
            child
                .body
                .iter()
                .map(|region| rewrite_region(region, &scope, base))
                .collect::<CompileResult<Vec<_>>>()?,
        );
        self.flattened.contributions.extend(
            child
                .contributions
                .iter()
                .map(|contribution| rewrite_contribution(contribution, &scope, base))
                .collect::<CompileResult<Vec<_>>>()?,
        );
        module_stack.push(instance.module.clone());
        let nested = self.append_instances(child_source, &scope, module_stack, path, false);
        module_stack.pop();
        nested
    }

    /// Validate array-valued overrides before any child state is appended to
    /// the flattened module. Array dimensions are evaluated twice: once with
    /// the child's declared scalar defaults and once with this instance's
    /// effective scalar overrides. If any dimension's extent changes, the
    /// array must be replaced by the same instance declaration so an inherited
    /// default can never silently acquire a different shape. Ordered bounds
    /// remain preserved independently in the flattened parameter metadata.
    fn validate_parameter_array_overrides(
        &self,
        child: &AnalyzedModule,
        parent_scope: &ScopeMap,
        overrides: &HashMap<usize, Expression>,
        path: &str,
    ) -> CompileResult<()> {
        if !child
            .parameters
            .iter()
            .any(|parameter| !parameter.dimensions.is_empty())
        {
            return Ok(());
        }

        // Parent-side override expressions are resolved in their declaring
        // scope. Previously flattened child parameters already carry rewritten
        // names and expressions, so this also handles nested hierarchies.
        let mut parent_values = HashMap::new();
        for parameter in &self.flattened.parameters {
            if !parameter.dimensions.is_empty() {
                continue;
            }
            let value = parameter
                .default_expr
                .as_ref()
                .and_then(|expression| {
                    SemanticAnalyzer::eval_const_value_with(expression, &parent_values)
                })
                .or_else(|| parameter.default.map(ConstantValue::Real))
                .and_then(|value| {
                    SemanticAnalyzer::constant_for_declared_type(value, parameter.param_type)
                });
            if let Some(value) = value {
                parent_values.insert(parameter.name.clone(), value);
            }
        }

        let mut declared_values = HashMap::new();
        let mut effective_values = HashMap::new();
        for (index, parameter) in child.parameters.iter().enumerate() {
            if !parameter.dimensions.is_empty() {
                continue;
            }

            let declared = parameter
                .default_expr
                .as_ref()
                .and_then(|expression| {
                    SemanticAnalyzer::eval_const_value_with(expression, &declared_values)
                })
                .or_else(|| parameter.default.map(ConstantValue::Real))
                .and_then(|value| {
                    SemanticAnalyzer::constant_for_declared_type(value, parameter.param_type)
                });
            if let Some(value) = declared {
                declared_values.insert(parameter.name.clone(), value);
            }

            let effective = if let Some(override_expression) = overrides.get(&index) {
                let expression = rewrite_expression(override_expression, parent_scope)?;
                SemanticAnalyzer::eval_const_value_with(&expression, &parent_values).and_then(
                    |value| {
                        SemanticAnalyzer::constant_for_declared_type(value, parameter.param_type)
                    },
                )
            } else {
                parameter
                    .default_expr
                    .as_ref()
                    .and_then(|expression| {
                        SemanticAnalyzer::eval_const_value_with(expression, &effective_values)
                    })
                    .or_else(|| parameter.default.map(ConstantValue::Real))
                    .and_then(|value| {
                        SemanticAnalyzer::constant_for_declared_type(value, parameter.param_type)
                    })
            };
            if let Some(value) = effective {
                effective_values.insert(parameter.name.clone(), value);
            }
        }

        for (index, parameter) in child.parameters.iter().enumerate() {
            if parameter.dimensions.is_empty() {
                continue;
            }
            let declared =
                resolve_parameter_array_shape(parameter, &declared_values, path, "declared")?;
            let effective =
                resolve_parameter_array_shape(parameter, &effective_values, path, "effective")?;
            let replacement = overrides.get(&index);

            if declared.extents != effective.extents && replacement.is_none() {
                return Err(semantic_error(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "instance '{path}' changes parameter array '{}' bounds from {} to {}; the array must be replaced in the same instance parameter override list",
                        parameter.name,
                        parameter_bounds_label(&declared.bounds),
                        parameter_bounds_label(&effective.bounds),
                    )),
                    parameter.dimensions[0].span,
                ));
            }

            let Some(replacement) = replacement else {
                continue;
            };
            let Expression::ArrayLiteral(initializer) = replacement else {
                return Err(semantic_error(
                    SemanticErrorKind::TypeMismatch {
                        expected: "constant assignment pattern opened with `'{'".into(),
                        found: "scalar expression".into(),
                        context: format!(
                            "override of parameter array '{}' at instance '{path}'",
                            parameter.name
                        ),
                    },
                    replacement.span(),
                ));
            };
            if !initializer.assignment_pattern {
                return Err(semantic_error(
                    SemanticErrorKind::TypeMismatch {
                        expected: "constant assignment pattern opened with `'{'".into(),
                        found: "concatenation opened with '{'".into(),
                        context: format!(
                            "override of parameter array '{}' at instance '{path}'",
                            parameter.name
                        ),
                    },
                    initializer.span,
                ));
            }
            if let Err(detail) = SemanticAnalyzer::validate_parameter_array_initializer_shape(
                initializer,
                &effective.extents,
                0,
            ) {
                return Err(semantic_error(
                    SemanticErrorKind::TypeMismatch {
                        expected: format!(
                            "rectangular assignment pattern with effective shape {:?}",
                            effective.extents
                        ),
                        found: detail,
                        context: format!(
                            "override of parameter array '{}' at instance '{path}'",
                            parameter.name
                        ),
                    },
                    initializer.span,
                ));
            }
        }

        Ok(())
    }

    fn bind_connections(
        &mut self,
        instance: &ModuleInstance,
        child: &AnalyzedModule,
        parent_scope: &ScopeMap,
        path: &str,
    ) -> CompileResult<Vec<Option<NodeBinding>>> {
        let has_named = instance
            .connections
            .iter()
            .any(|connection| matches!(connection, Connection::Named { .. }));
        let has_ordered = instance
            .connections
            .iter()
            .any(|connection| matches!(connection, Connection::Ordered { .. }));
        if has_named && has_ordered {
            return Err(semantic_error(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "instance '{path}' mixes named and ordered port connections"
                )),
                instance.span,
            ));
        }
        let mut bound = vec![None; child.ports.len()];
        let mut seen = vec![false; child.ports.len()];
        if has_named {
            for connection in &instance.connections {
                let Connection::Named { port, signal, span } = connection else {
                    unreachable!()
                };
                let Some(index) = child
                    .ports
                    .iter()
                    .position(|candidate| candidate.name == *port)
                else {
                    return Err(semantic_error(
                        SemanticErrorKind::UndeclaredSymbol { name: port.clone() },
                        *span,
                    ));
                };
                if seen[index] {
                    return Err(semantic_error(
                        SemanticErrorKind::DuplicateSymbol {
                            name: port.clone(),
                            first_defined: *span,
                        },
                        *span,
                    ));
                }
                seen[index] = true;
                bound[index] = signal
                    .as_ref()
                    .map(|signal| {
                        resolve_connection(signal, parent_scope, path, &child.ports[index])
                    })
                    .transpose()?;
            }
        } else {
            if instance.connections.len() > child.ports.len() {
                return Err(semantic_error(
                    SemanticErrorKind::ArgumentCountMismatch {
                        name: path.to_string(),
                        expected: format!("at most {} port connections", child.ports.len()),
                        got: instance.connections.len(),
                    },
                    instance.span,
                ));
            }
            for (index, connection) in instance.connections.iter().enumerate() {
                let Connection::Ordered { signal, .. } = connection else {
                    unreachable!()
                };
                bound[index] = signal
                    .as_ref()
                    .map(|signal| {
                        resolve_connection(signal, parent_scope, path, &child.ports[index])
                    })
                    .transpose()?;
            }
        }
        Ok(bound)
    }

    fn fresh_name(&mut self, leaf: &str) -> SmolStr {
        loop {
            let name = SmolStr::from(format!("__rspice_h{}_{}", self.next_name, leaf));
            self.next_name += 1;
            if self.used_names.insert(name.clone()) {
                return name;
            }
        }
    }
}

fn resolve_parameter_array_shape(
    parameter: &AnalyzedParameter,
    scalar_values: &HashMap<SmolStr, ConstantValue>,
    path: &str,
    value_kind: &str,
) -> CompileResult<EffectiveParameterArrayShape> {
    if parameter.dimensions.len() > MAX_PARAMETER_ARRAY_RANK {
        return Err(semantic_error(
            SemanticErrorKind::UnsupportedFeature(format!(
                "parameter array '{}' at instance '{path}' has rank {}; the supported safety limit is {MAX_PARAMETER_ARRAY_RANK}",
                parameter.name,
                parameter.dimensions.len(),
            )),
            parameter.dimensions[MAX_PARAMETER_ARRAY_RANK].span,
        ));
    }

    let mut bounds = Vec::with_capacity(parameter.dimensions.len());
    let mut extents = Vec::with_capacity(parameter.dimensions.len());
    let mut total_elements = 1_u64;
    for (dimension_index, dimension) in parameter.dimensions.iter().enumerate() {
        let resolve_bound = |side: &str, expression: &Expression| {
            SemanticAnalyzer::eval_const_value_with(expression, scalar_values)
                .and_then(|value| SemanticAnalyzer::exact_const_i64(value.as_f64()))
                .ok_or_else(|| {
                    semantic_error(
                        SemanticErrorKind::InvalidExpression(format!(
                            "{value_kind} {side} bound of dimension {} of parameter array '{}' at instance '{path}' does not resolve to a finite signed integer",
                            dimension_index + 1,
                            parameter.name,
                        )),
                        expression.span(),
                    )
                })
        };
        let left = resolve_bound("left", &dimension.left)?;
        let right = resolve_bound("right", &dimension.right)?;
        let extent = left.abs_diff(right).checked_add(1).ok_or_else(|| {
            semantic_error(
                SemanticErrorKind::InvalidExpression(format!(
                    "{value_kind} dimension {} of parameter array '{}' at instance '{path}' has an unrepresentable extent",
                    dimension_index + 1,
                    parameter.name,
                )),
                dimension.span,
            )
        })?;
        total_elements = total_elements.checked_mul(extent).ok_or_else(|| {
            semantic_error(
                SemanticErrorKind::InvalidExpression(format!(
                    "{value_kind} element count of parameter array '{}' at instance '{path}' overflows the canonical shape representation",
                    parameter.name,
                )),
                dimension.span,
            )
        })?;
        if total_elements > MAX_PARAMETER_ARRAY_ELEMENTS {
            return Err(semantic_error(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "{value_kind} shape of parameter array '{}' at instance '{path}' declares {total_elements} elements; the supported safety limit is {MAX_PARAMETER_ARRAY_ELEMENTS}",
                    parameter.name,
                )),
                dimension.span,
            ));
        }
        bounds.push((left, right));
        extents.push(extent);
    }

    Ok(EffectiveParameterArrayShape { bounds, extents })
}

fn parameter_bounds_label(bounds: &[(i64, i64)]) -> String {
    bounds
        .iter()
        .map(|(left, right)| format!("[{left}:{right}]"))
        .collect::<String>()
}

fn bind_parameter_overrides(
    instance: &ModuleInstance,
    child: &AnalyzedModule,
    path: &str,
) -> CompileResult<HashMap<usize, Expression>> {
    let has_named = instance
        .parameters
        .iter()
        .any(|parameter| parameter.name.is_some());
    let has_ordered = instance
        .parameters
        .iter()
        .any(|parameter| parameter.name.is_none());
    if has_named && has_ordered {
        return Err(semantic_error(
            SemanticErrorKind::UnsupportedFeature(format!(
                "instance '{path}' mixes named and ordered parameter overrides"
            )),
            instance.span,
        ));
    }
    if has_ordered && instance.parameters.len() > child.parameters.len() {
        return Err(semantic_error(
            SemanticErrorKind::ArgumentCountMismatch {
                name: path.to_string(),
                expected: format!("at most {} parameter overrides", child.parameters.len()),
                got: instance.parameters.len(),
            },
            instance.span,
        ));
    }
    let aliases: HashMap<&str, usize> = child
        .param_aliases
        .iter()
        .map(|alias| (alias.alias.as_str(), alias.target))
        .collect();
    let mut overrides = HashMap::new();
    for (ordered_index, parameter) in instance.parameters.iter().enumerate() {
        let index = match &parameter.name {
            Some(name) => child
                .parameters
                .iter()
                .position(|candidate| candidate.name == *name)
                .or_else(|| aliases.get(name.as_str()).copied())
                .ok_or_else(|| {
                    semantic_error(
                        SemanticErrorKind::UndeclaredSymbol { name: name.clone() },
                        parameter.span,
                    )
                })?,
            None => ordered_index,
        };
        if overrides.insert(index, parameter.value.clone()).is_some() {
            let name = child.parameters[index].name.clone();
            return Err(semantic_error(
                SemanticErrorKind::DuplicateSymbol {
                    name,
                    first_defined: parameter.span,
                },
                parameter.span,
            ));
        }
    }
    Ok(overrides)
}

fn resolve_connection(
    expression: &Expression,
    parent_scope: &ScopeMap,
    path: &str,
    child_port: &super::AnalyzedPort,
) -> CompileResult<NodeBinding> {
    let source_name = match expression {
        Expression::Identifier(identifier) => identifier.name.as_str(),
        Expression::Number(number) if number.value == 0.0 => "0",
        _ => {
            return Err(semantic_error(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "analog port '{}' of instance '{path}' must connect to a net identifier or ground",
                    child_port.name
                )),
                expression.span(),
            ));
        }
    };
    let binding = parent_scope
        .nodes
        .get(source_name)
        .cloned()
        .ok_or_else(|| {
            semantic_error(
                SemanticErrorKind::UndeclaredSymbol {
                    name: source_name.into(),
                },
                expression.span(),
            )
        })?;
    if let Some(parent_discipline) = &binding.discipline
        && parent_discipline != &child_port.discipline
    {
        return Err(semantic_error(
            SemanticErrorKind::UnsupportedFeature(format!(
                "discipline mismatch on instance '{path}' port '{}': expected '{}', connected net uses '{}'",
                child_port.name, child_port.discipline, parent_discipline
            )),
            expression.span(),
        ));
    }
    Ok(binding)
}

/// Where one child's ids land in the flattened parent.
///
/// Both bases have to travel together: an inlined instance's variables and its
/// analog sites are each renumbered onto the parent's spaces, and rebasing one
/// without the other would leave a site pointing at another instance's copy.
#[derive(Debug, Clone, Copy)]
struct InstanceBase {
    variables: usize,
    sites: u32,
}

impl InstanceBase {
    fn site(self, site: AnalogSiteId) -> CompileResult<AnalogSiteId> {
        self.sites
            .checked_add(site.0)
            .map(AnalogSiteId)
            .ok_or_else(|| internal_error("hierarchy analog site index overflow".to_string()))
    }
}

fn rewrite_statement(
    statement: &AnalyzedStatement,
    scope: &ScopeMap,
    base: InstanceBase,
) -> CompileResult<AnalyzedStatement> {
    Ok(match statement {
        AnalyzedStatement::Assignment(assignment) => {
            AnalyzedStatement::Assignment(rewrite_assignment(assignment, scope, base)?)
        }
        AnalyzedStatement::Loop(loop_) => AnalyzedStatement::Loop(AnalyzedLoop {
            condition: rewrite_expression(&loop_.condition, scope)?,
            site: base.site(loop_.site)?,
            condition_guard: loop_.condition_guard,
            body: loop_
                .body
                .iter()
                .map(|statement| rewrite_statement(statement, scope, base))
                .collect::<CompileResult<Vec<_>>>()?,
            span: loop_.span,
        }),
    })
}

fn rewrite_region(
    region: &AnalyzedRegion,
    scope: &ScopeMap,
    base: InstanceBase,
) -> CompileResult<AnalyzedRegion> {
    Ok(match region {
        AnalyzedRegion::Assignment(assignment) => {
            AnalyzedRegion::Assignment(rewrite_assignment(assignment, scope, base)?)
        }
        AnalyzedRegion::Contribution(contribution) => {
            AnalyzedRegion::Contribution(rewrite_contribution(contribution, scope, base)?)
        }
        AnalyzedRegion::Conditional {
            condition,
            condition_site,
            then_body,
            else_body,
            span,
        } => AnalyzedRegion::Conditional {
            condition: rewrite_expression(condition, scope)?,
            condition_site: condition_site.map(|site| base.site(site)).transpose()?,
            then_body: then_body
                .iter()
                .map(|region| rewrite_region(region, scope, base))
                .collect::<CompileResult<Vec<_>>>()?,
            else_body: else_body
                .iter()
                .map(|region| rewrite_region(region, scope, base))
                .collect::<CompileResult<Vec<_>>>()?,
            span: *span,
        },
        AnalyzedRegion::Loop {
            condition,
            site,
            body,
            span,
        } => AnalyzedRegion::Loop {
            condition: rewrite_expression(condition, scope)?,
            site: base.site(*site)?,
            body: body
                .iter()
                .map(|region| rewrite_region(region, scope, base))
                .collect::<CompileResult<Vec<_>>>()?,
            span: *span,
        },
    })
}

fn rewrite_assignment(
    assignment: &AnalyzedAssignment,
    scope: &ScopeMap,
    base: InstanceBase,
) -> CompileResult<AnalyzedAssignment> {
    let variable_base = base.variables;
    let target = scope
        .variables
        .get(&assignment.target)
        .or_else(|| scope.arrays.get(&assignment.target))
        .cloned()
        .unwrap_or_else(|| assignment.target.clone());
    Ok(AnalyzedAssignment {
        target,
        var_index: variable_base
            .checked_add(assignment.var_index)
            .ok_or_else(|| internal_error("hierarchy variable index overflow".to_string()))?,
        index: assignment
            .index
            .as_ref()
            .map(|expression| rewrite_expression(expression, scope))
            .transpose()?,
        expression: rewrite_expression(&assignment.expression, scope)?,
        site: base.site(assignment.site)?,
        expression_guard: assignment.expression_guard,
        expr_type: assignment.expr_type,
        span: assignment.span,
        unfiltered_initial_step_guard: assignment.unfiltered_initial_step_guard.as_ref().map(
            |name| {
                scope
                    .variables
                    .get(name)
                    .cloned()
                    .unwrap_or_else(|| name.clone())
            },
        ),
    })
}

fn rewrite_contribution(
    contribution: &AnalyzedContribution,
    scope: &ScopeMap,
    base: InstanceBase,
) -> CompileResult<AnalyzedContribution> {
    let mut endpoints = contribution.branch.split(',');
    let pos = endpoints.next().unwrap_or_default();
    let neg = endpoints.next();
    if endpoints.next().is_some() || pos.is_empty() {
        return Err(internal_error(format!(
            "invalid analyzed contribution branch '{}'",
            contribution.branch
        )));
    }
    let pos = mapped_node_name(scope, pos, contribution.span)?;
    let branch = if let Some(neg) = neg {
        let neg = mapped_node_name(scope, neg, contribution.span)?;
        SmolStr::from(format!("{pos},{neg}"))
    } else {
        pos
    };
    Ok(AnalyzedContribution {
        branch,
        declared_branch: contribution
            .declared_branch
            .as_ref()
            .map(|name| {
                scope.branches.get(name).cloned().ok_or_else(|| {
                    semantic_error(
                        SemanticErrorKind::UndeclaredSymbol { name: name.clone() },
                        contribution.span,
                    )
                })
            })
            .transpose()?,
        is_current: contribution.is_current,
        indirect: contribution.indirect,
        expression: rewrite_expression(&contribution.expression, scope)?,
        site: base.site(contribution.site)?,
        expression_guard: contribution.expression_guard,
        expr_type: contribution.expr_type,
        span: contribution.span,
    })
}

fn mapped_node_name(scope: &ScopeMap, name: &str, span: Span) -> CompileResult<SmolStr> {
    scope
        .nodes
        .get(name)
        .map(|binding| binding.name.clone())
        .ok_or_else(|| {
            semantic_error(
                SemanticErrorKind::UndeclaredSymbol { name: name.into() },
                span,
            )
        })
}

fn mapped_optional_parameter(
    parameter: Option<&SmolStr>,
    scope: &ScopeMap,
    span: Span,
) -> CompileResult<Option<SmolStr>> {
    parameter
        .map(|name| {
            scope.parameters.get(name).cloned().ok_or_else(|| {
                semantic_error(
                    SemanticErrorKind::UndeclaredSymbol { name: name.clone() },
                    span,
                )
            })
        })
        .transpose()
}

fn rewrite_expression(expression: &Expression, scope: &ScopeMap) -> CompileResult<Expression> {
    Ok(match expression {
        Expression::Number(_) | Expression::StringLit(_) | Expression::NullArgument(_) => {
            expression.clone()
        }
        // Hierarchy elaboration rewrites the continuous-domain body of a
        // module that has already been accepted by semantic analysis, which
        // refuses discrete-domain expressions. Failing here rather than
        // cloning keeps that guarantee explicit.
        Expression::Digital(digital) => {
            return Err(semantic_error(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "a {} cannot be elaborated into a continuous-domain module body",
                    digital.construct()
                )),
                digital.span(),
            ));
        }
        Expression::Identifier(identifier) => Expression::Identifier(Identifier {
            name: scope
                .parameters
                .get(&identifier.name)
                .or_else(|| scope.variables.get(&identifier.name))
                .cloned()
                .unwrap_or_else(|| identifier.name.clone()),
            span: identifier.span,
        }),
        Expression::SystemFunction(function) => {
            if let Some(value) = rewritten_connectivity_predicate(
                &function.name,
                &function.args,
                function.span,
                scope,
            ) {
                value
            } else {
                Expression::SystemFunction(SystemFunction {
                    name: function.name.clone(),
                    args: rewrite_expressions(&function.args, scope)?,
                    span: function.span,
                })
            }
        }
        Expression::Binary(binary) => Expression::Binary(BinaryExpr {
            op: binary.op,
            left: Box::new(rewrite_expression(&binary.left, scope)?),
            right: Box::new(rewrite_expression(&binary.right, scope)?),
            span: binary.span,
        }),
        Expression::Unary(unary) => Expression::Unary(UnaryExpr {
            op: unary.op,
            operand: Box::new(rewrite_expression(&unary.operand, scope)?),
            span: unary.span,
        }),
        Expression::Conditional(conditional) => Expression::Conditional(ConditionalExpr {
            condition: Box::new(rewrite_expression(&conditional.condition, scope)?),
            then_expr: Box::new(rewrite_expression(&conditional.then_expr, scope)?),
            else_expr: Box::new(rewrite_expression(&conditional.else_expr, scope)?),
            span: conditional.span,
        }),
        Expression::Call(call) => {
            if let Some(value) =
                rewritten_connectivity_predicate(&call.name, &call.args, call.span, scope)
            {
                value
            } else {
                let mut args = rewrite_expressions(&call.args, scope)?;
                qualify_noise_call_name(&call.name, &mut args, scope);
                Expression::Call(CallExpr {
                    name: call.name.clone(),
                    args,
                    span: call.span,
                })
            }
        }
        Expression::BranchAccess(access) => {
            Expression::BranchAccess(rewrite_branch_access(access, scope)?)
        }
        Expression::ArrayAccess(access) => Expression::ArrayAccess(ArrayAccessExpr {
            array: scope
                .arrays
                .get(&access.array)
                .cloned()
                .unwrap_or_else(|| access.array.clone()),
            index: Box::new(rewrite_expression(&access.index, scope)?),
            span: access.span,
        }),
        Expression::ArrayLiteral(array) => {
            if let Some(replication) = array.first_replication() {
                return Err(semantic_error(
                    SemanticErrorKind::UnsupportedFeature(
                        "replication is retained by the parser but hierarchical elaboration does not yet support it; write the elements explicitly"
                            .into(),
                    ),
                    replication.span,
                ));
            }
            Expression::ArrayLiteral(ArrayLiteralExpr {
                elements: array
                    .elements
                    .iter()
                    .map(|element| {
                        let ArrayLiteralElement::Value(expression) = element else {
                            unreachable!("replication was rejected before elaboration");
                        };
                        rewrite_expression(expression, scope).map(ArrayLiteralElement::Value)
                    })
                    .collect::<CompileResult<Vec<_>>>()?,
                assignment_pattern: array.assignment_pattern,
                span: array.span,
            })
        }
        Expression::AnalogOperator(operator) => {
            Expression::AnalogOperator(rewrite_analog_operator(operator, scope)?)
        }
        Expression::NoiseSource(noise) => {
            Expression::NoiseSource(rewrite_noise_source(noise, scope)?)
        }
    })
}

fn qualify_noise_call_name(name: &str, arguments: &mut [Expression], scope: &ScopeMap) {
    let Some(path) = &scope.instance_path else {
        return;
    };
    let name_index = match name {
        "white_noise" => Some(1),
        "flicker_noise" => Some(2),
        "noise_table" | "noise_table_log" => Some(1),
        _ => None,
    };
    let Some(Expression::StringLit(label)) = name_index.and_then(|index| arguments.get_mut(index))
    else {
        return;
    };
    label.value = SmolStr::from(format!("{path}:{}", label.value));
}

fn rewritten_connectivity_predicate(
    name: &str,
    arguments: &[Expression],
    span: Span,
    scope: &ScopeMap,
) -> Option<Expression> {
    let Expression::Identifier(identifier) = arguments.first()? else {
        return None;
    };
    if arguments.len() != 1 {
        return None;
    }
    let value = if name.eq_ignore_ascii_case("$param_given") {
        scope.parameter_given.get(&identifier.name).copied()
    } else if name.eq_ignore_ascii_case("$port_connected") {
        scope.port_connected.get(&identifier.name).copied()
    } else {
        None
    }?;
    Some(number_expression(if value { 1.0 } else { 0.0 }, span))
}

fn rewrite_expressions(
    expressions: &[Expression],
    scope: &ScopeMap,
) -> CompileResult<Vec<Expression>> {
    expressions
        .iter()
        .map(|expression| rewrite_expression(expression, scope))
        .collect()
}

fn rewrite_branch_access(access: &BranchAccess, scope: &ScopeMap) -> CompileResult<BranchAccess> {
    Ok(match access {
        BranchAccess::Nodes {
            access,
            pos,
            neg,
            span,
        } => BranchAccess::Nodes {
            access: access.clone(),
            pos: if neg.is_none() {
                scope
                    .branches
                    .get(pos)
                    .cloned()
                    .map(Ok)
                    .unwrap_or_else(|| mapped_node_name(scope, pos, *span))?
            } else {
                mapped_node_name(scope, pos, *span)?
            },
            neg: neg
                .as_ref()
                .map(|name| mapped_node_name(scope, name, *span))
                .transpose()?,
            span: *span,
        },
        BranchAccess::Branch { access, name, span } => BranchAccess::Branch {
            access: access.clone(),
            name: scope.branches.get(name).cloned().ok_or_else(|| {
                semantic_error(
                    SemanticErrorKind::UndeclaredSymbol { name: name.clone() },
                    *span,
                )
            })?,
            span: *span,
        },
    })
}

fn rewrite_analog_operator(
    operator: &AnalogOperator,
    scope: &ScopeMap,
) -> CompileResult<AnalogOperator> {
    let optional = |expression: &Option<Box<Expression>>| {
        expression
            .as_ref()
            .map(|expression| rewrite_expression(expression, scope).map(Box::new))
            .transpose()
    };
    Ok(match operator {
        AnalogOperator::Limit {
            proposed,
            candidate,
            type_metadata,
            selector,
            span,
        } => AnalogOperator::Limit {
            proposed: Box::new(rewrite_expression(proposed, scope)?),
            candidate: Box::new(rewrite_expression(candidate, scope)?),
            type_metadata: optional(type_metadata)?,
            selector: selector.clone(),
            span: *span,
        },
        AnalogOperator::LimiterArgument { .. } => operator.clone(),
    })
}

fn rewrite_noise_source(noise: &NoiseSource, scope: &ScopeMap) -> CompileResult<NoiseSource> {
    let qualified_name = |name: &Option<SmolStr>| {
        name.as_ref().map(|name| match &scope.instance_path {
            Some(path) => SmolStr::from(format!("{path}:{name}")),
            None => name.clone(),
        })
    };
    let remap_process_id = |process_id: Option<u32>| -> CompileResult<Option<u32>> {
        let Some(process_id) = process_id else {
            return Ok(None);
        };
        let Some((base, count)) = scope.noise_process_range else {
            return Ok(Some(process_id));
        };
        if process_id >= count {
            return Err(internal_error(format!(
                "noise process {process_id} exceeds module-local process count {count} at instance '{}'",
                scope.instance_path.as_deref().unwrap_or("<root>")
            )));
        }
        Ok(Some(base.checked_add(process_id).ok_or_else(|| {
            internal_error("hierarchy noise-process identity overflow".into())
        })?))
    };
    Ok(match noise {
        NoiseSource::White {
            process_id,
            power,
            name,
            span,
        } => NoiseSource::White {
            process_id: remap_process_id(*process_id)?,
            power: Box::new(rewrite_expression(power, scope)?),
            name: qualified_name(name),
            span: *span,
        },
        NoiseSource::Flicker {
            process_id,
            power,
            exponent,
            name,
            span,
        } => NoiseSource::Flicker {
            process_id: remap_process_id(*process_id)?,
            power: Box::new(rewrite_expression(power, scope)?),
            exponent: Box::new(rewrite_expression(exponent, scope)?),
            name: qualified_name(name),
            span: *span,
        },
        NoiseSource::Table {
            process_id,
            data,
            log_interp,
            name,
            span,
        } => NoiseSource::Table {
            process_id: remap_process_id(*process_id)?,
            data: rewrite_expressions(data, scope)?,
            log_interp: *log_interp,
            name: qualified_name(name),
            span: *span,
        },
    })
}

fn number_expression(value: f64, span: Span) -> Expression {
    Expression::Number(NumberLit {
        value,
        raw: if value == 0.0 { "0.0" } else { "1.0" }.into(),
        span,
    })
}

fn semantic_error(kind: SemanticErrorKind, span: Span) -> CompileError {
    SemanticError::new(kind, span).into()
}

fn internal_error(message: String) -> CompileError {
    crate::error::CodeGenError::new(crate::error::CodeGenErrorKind::Internal(message)).into()
}
