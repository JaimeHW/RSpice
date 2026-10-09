//! Executable elaboration of structural Verilog-A module instances.
//!
//! Semantic analysis deliberately keeps every module independent.  This pass
//! selects a top module and flattens its instance tree into one analyzed model
//! before either executable backend sees it.  Keeping the pass here gives the
//! bytecode and canonical-IR paths exactly the same ports, parameters, state,
//! nodes, and equations.

use super::{
    AnalogSiteGuard, AnalogSiteId, AnalyzedArray, AnalyzedAssignment, AnalyzedBranch,
    AnalyzedContribution, AnalyzedFile, AnalyzedInternalNode, AnalyzedLoop, AnalyzedModule,
    AnalyzedParameter, AnalyzedRegion, AnalyzedStatement, AnalyzedVariable, ConstantValue,
    MAX_PARAMETER_ARRAY_ELEMENTS, MAX_PARAMETER_ARRAY_RANK, SemanticAnalyzer, ValueType,
};
use crate::ast::{
    AccessKind, AnalogOperator, ArrayAccessExpr, ArrayLiteralElement, ArrayLiteralExpr, BinaryExpr,
    BinaryOp, BranchAccess, CallExpr, ConditionalExpr, Expression, Identifier, Item,
    Module, ModuleInstance, NoiseSource, NumberLit, SystemFunction, UnaryExpr, UnaryOp, VarType,
};
use crate::error::{CompileError, CompileResult, SemanticError, SemanticErrorKind};
use crate::source::Span;
use smol_str::SmolStr;
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap, HashSet};

#[path = "elaboration_parameters.rs"]
pub(super) mod parameters;
#[path = "elaboration_references.rs"]
mod references;

/// Return the selected module itself when it has no hierarchy or retained
/// generate structure, otherwise a faithfully flattened owned module.  Unsupported or ambiguous structure is
/// rejected before code generation; it is never omitted.
pub(crate) fn elaborate_executable_module<'a>(
    analyzed: &'a AnalyzedFile,
    selected: &'a AnalyzedModule,
) -> CompileResult<Cow<'a, AnalyzedModule>> {
    // Resolve the discrete hierarchy first, retaining the concrete occurrence
    // specializations. The analog pass consumes the same source and analysis,
    // then binds that occurrence's processes to the relocated analog storage.
    let source_modules = source_modules(analyzed)?;
    let root = source_modules.get(&selected.name).copied().ok_or_else(|| {
        internal_error(format!(
            "selected module '{}' has no retained source module",
            selected.name
        ))
    })?;
    if root.instances.is_empty() && root.generate_template.is_none() {
        return Ok(Cow::Borrowed(selected));
    }

    let hierarchy = super::digital_elaborate::elaborate_digital_hierarchy(
        analyzed,
        &source_modules,
        root,
        selected,
    )?;

    let prepared_root = hierarchy.root;
    let (root, selected) = prepared_root
        .as_deref()
        .map(|module| (&module.source, &module.analyzed))
        .unwrap_or((root, selected));
    let mut elaborator = HierarchyElaborator::new(analyzed, source_modules, selected.clone());
    elaborator.flattened.elaboration_warnings.extend(hierarchy.warnings);
    elaborator.shared_occurrences = hierarchy.occurrences;
    elaborator.digital_frames = hierarchy
        .instances
        .iter()
        .enumerate()
        .map(|(index, frame)| (frame.path.clone(), index))
        .collect();
    elaborator.flattened.digital.instances = hierarchy.instances;
    let root_scope = ScopeMap::for_root(root, selected);
    let inventory = super::flow_probes::hierarchy_branches(selected);
    elaborator.references.request(root, selected, "", &inventory);
    elaborator.references.register(root, selected, "", &root_scope)?;
    elaborator
        .parameter_hierarchy
        .register(root, selected, &root_scope, None)?;
    let mut module_stack = vec![selected.name.clone()];
    elaborator.append_instances(root, &root_scope, &mut module_stack, selected.name.as_str())?;
    Ok(Cow::Owned(elaborator.finish()?))
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
    /// Probed module boundaries traversed by this connection.
    port_boundaries: Vec<SmolStr>,
    name: SmolStr,
    discipline: Option<SmolStr>,
}

struct PortFlow {
    owner: SmolStr,
    span: Span,
    terms: BTreeMap<SmolStr, i8>,
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
    connections: super::node_vectors::ConnectionScope,
    discrete_nets: HashSet<SmolStr>,
    nodes: HashMap<SmolStr, NodeBinding>,
    parameters: HashMap<SmolStr, SmolStr>,
    parameter_given: HashMap<SmolStr, bool>,
    parameter_locals: std::sync::Arc<crate::semantic::parameter_defaults::LocalDefaults>,
    variables: HashMap<SmolStr, SmolStr>,
    arrays: HashMap<SmolStr, SmolStr>,
    branches: HashMap<SmolStr, SmolStr>,
    unnamed_branches: HashMap<(SmolStr, SmolStr), AnalyzedBranch>,
    node_reference_aliases: HashMap<SmolStr, SmolStr>,
    ground_nodes: Vec<SmolStr>,
    port_flows: HashMap<SmolStr, SmolStr>,
    port_connected: HashMap<SmolStr, bool>,
    instance_path: Option<SmolStr>,
    /// `(global base, local count)` for this concrete module occurrence.
    /// The root is not rewritten and therefore leaves this unset.
    noise_process_range: Option<(u32, u32)>,
}

impl ScopeMap {
    fn unnamed_branch(&self, pos: &str, neg: &str) -> Option<(&AnalyzedBranch, f64)> {
        let pos = self.node_reference_aliases.get(pos).map_or(pos, SmolStr::as_str);
        let neg = self.node_reference_aliases.get(neg).map_or(neg, SmolStr::as_str);
        let (pos, neg, sign) =
            super::flow_probes::canonical_node_pair(pos, neg, &self.ground_nodes);
        self.unnamed_branches
            .get(&(pos, neg))
            .map(|branch| (branch, sign))
    }

    fn for_root(source: &Module, module: &AnalyzedModule) -> Self {
        let mut scope = Self {
            connections: super::node_vectors::ConnectionScope::new(source, module),
            parameter_locals: module.parameter_locals.clone(),
            discrete_nets: module
                .digital
                .signals
                .iter()
                .map(|signal| signal.name.clone())
                .collect(),
            ..Self::default()
        };
        for port in &module.ports {
            if scope.discrete_nets.contains(&port.name) {
                continue;
            }
            scope.nodes.insert(
                port.name.clone(),
                NodeBinding {
                    port_boundaries: Vec::new(),
                    name: port.name.clone(),
                    discipline: Some(port.discipline.clone()),
                },
            );
        }
        for node in &module.internal_nodes {
            scope.nodes.insert(
                node.name.clone(),
                NodeBinding {
                    port_boundaries: Vec::new(),
                    name: node.name.clone(),
                    discipline: Some(node.discipline.clone()),
                },
            );
        }
        for ground in &module.ground_nodes {
            scope.nodes.insert(
                ground.clone(),
                NodeBinding {
                    port_boundaries: Vec::new(),
                    name: ground.clone(),
                    discipline: None,
                },
            );
        }
        scope.nodes.insert(
            "0".into(),
            NodeBinding {
                port_boundaries: Vec::new(),
                name: "0".into(),
                discipline: None,
            },
        );
        for parameter in &module.parameters {
            scope
                .parameters
                .insert(parameter.name.clone(), parameter.name.clone());
        }
        for parameter in &module.digital.elaboration_parameters {
            scope
                .parameters
                .insert(parameter.name.clone(), parameter.name.clone());
            for alias in &parameter.aliases {
                scope
                    .parameters
                    .insert(alias.clone(), parameter.name.clone());
            }
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
    shared_occurrences:
        HashMap<SmolStr, std::sync::Arc<super::digital_elaborate::SpecializedModule>>,
    digital_frames: HashMap<SmolStr, usize>,
    analyzed: &'a AnalyzedFile,
    source_modules: HashMap<SmolStr, &'a Module>,
    flattened: AnalyzedModule,
    used_names: HashSet<SmolStr>,
    next_name: usize,
    next_noise_process: u32,
    child_control_variables: [Vec<SmolStr>; 2],
    port_flows: BTreeMap<SmolStr, PortFlow>,
    references: references::PhysicalReferences,
    pending_port_terms: Vec<(SmolStr, NodeBinding, SmolStr, i8)>,
    parameter_hierarchy: parameters::ParameterHierarchy,
    specialization_modules: HashSet<SmolStr>,
    specializations:
        HashMap<parameters::SpecializationKey, std::sync::Arc<parameters::SpecializedModule>>,
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
        for parameter in &flattened.digital.elaboration_parameters {
            used_names.insert(parameter.name.clone());
            used_names.extend(parameter.aliases.iter().cloned());
        }
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
        let specialization_modules = parameters::specialization_modules(&source_modules, analyzed);
        Self {
            analyzed,
            shared_occurrences: HashMap::new(),
            digital_frames: HashMap::new(),
            source_modules,
            flattened,
            used_names,
            next_name: 0,
            next_noise_process,
            child_control_variables: Default::default(),
            port_flows: BTreeMap::new(),
            references: Default::default(),
            pending_port_terms: Vec::new(),
            parameter_hierarchy: Default::default(),
            specialization_modules,
            specializations: HashMap::new(),
        }
    }

    fn finish(mut self) -> CompileResult<AnalyzedModule> {
        self.references.finish(
            &mut self.flattened,
            &mut self.port_flows,
            &self.pending_port_terms,
        )?;
        let span = self.source_modules[&self.flattened.name].span;
        self.parameter_hierarchy
            .protect(&mut self.flattened, span)?;
        self.flattened.noise_process_count = self.next_noise_process;
        // Child control variables remain independent, including their resets.
        // Publish one aggregate under the names executable backends consume.
        for (task, children) in crate::analog_tasks::SIMULATOR_CONTROL_TASK_VARIABLES
            .into_iter()
            .zip(self.child_control_variables)
        {
            if children.is_empty() {
                continue;
            }
            let span = self.source_modules[&self.flattened.name].span;
            let existing = self
                .flattened
                .variables
                .iter()
                .position(|variable| variable.name == task);
            let var_index = existing.unwrap_or_else(|| {
                let index = self.flattened.variables.len();
                self.flattened.variables.push(AnalyzedVariable {
                    name: task.into(),
                    var_type: VarType::Real,
                    value_type: ValueType::Real,
                    is_state: false,
                    retains_input: false,
                    is_event_controlled: false,
                });
                index
            });
            let mut expressions = children
                .into_iter()
                .chain(existing.map(|_| SmolStr::from(task)))
                .map(|name| Expression::Identifier(Identifier { name, span }))
                .collect::<Vec<_>>();
            // Balance large hierarchies so their reduction does not introduce
            // expression depth proportional to the number of instances.
            while expressions.len() > 1 {
                let mut values = expressions.into_iter();
                let mut next = Vec::with_capacity(values.len().div_ceil(2));
                while let Some(left) = values.next() {
                    next.push(if let Some(right) = values.next() {
                        if task == "$bound_step" {
                            Expression::Call(CallExpr {
                                resolved_builtin: false,
                                name: "min".into(),
                                args: vec![left, right],
                                span,
                            })
                        } else {
                            Expression::Binary(BinaryExpr {
                                op: BinaryOp::BitOr,
                                left: Box::new(left),
                                right: Box::new(right),
                                span,
                            })
                        }
                    } else {
                        left
                    });
                }
                expressions = next;
            }
            let site = AnalogSiteId(self.flattened.analog_site_count);
            self.flattened.analog_site_count = site
                .0
                .checked_add(1)
                .ok_or_else(|| internal_error("hierarchy control-task site overflow".into()))?;
            let assignment = AnalyzedAssignment {
                occurrence_source: None,
                target: task.into(),
                var_index,
                index: None,
                expression: expressions.pop().expect("nonempty child control reduction"),
                site,
                expression_guard: AnalogSiteGuard::None,
                expr_type: ValueType::Real,
                span,
                unfiltered_initial_step_guard: None,
            };
            self.flattened
                .body
                .push(AnalyzedRegion::Assignment(assignment.clone()));
            self.flattened
                .statements
                .push(AnalyzedStatement::Assignment(assignment));
        }
        let ports = self
            .port_flows
            .into_iter()
            .map(|(name, port)| {
                let terms = port
                    .terms
                    .into_iter()
                    .map(|(branch, sign)| {
                        super::flow_probes::signed(
                            Expression::BranchAccess(BranchAccess::Branch {
                                access: "I".into(),
                                kind: Some(AccessKind::Flow),
                                name: branch,
                                span: port.span,
                                index: None,
                            }),
                            f64::from(sign),
                        )
                    })
                    .collect();
                (name, super::flow_probes::sum_expressions(terms, port.span))
            })
            .collect();
        super::flow_probes::expand_port_flows(&mut self.flattened, &ports);
        Ok(self.flattened)
    }

    fn record_port_flow(&mut self, scope: &ScopeMap, pos: &str, neg: &str, branch: &SmolStr) {
        for (endpoint, sign) in [(pos, 1), (neg, -1)] {
            let Some(binding) = scope.nodes.get(endpoint) else {
                continue;
            };
            if binding.port_boundaries.is_empty()
                && scope.node_reference_aliases.is_empty()
                && !self.references.has_node_aliases()
            {
                continue;
            }
            self.pending_port_terms.push((
                scope.instance_path.clone().unwrap_or_else(|| self.flattened.name.clone()),
                binding.clone(), branch.clone(), sign,
            ));
        }
    }

    /// Flatten the analog content of each concrete occurrence. Digital processes
    /// remain in independently named frames with bindings to this same storage.
    fn append_instances(
        &mut self,
        source_module: &Module,
        parent_scope: &ScopeMap,
        module_stack: &mut Vec<SmolStr>,
        parent_path: &str,
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
            self.append_instance(instance, source_module, parent_scope, module_stack, &path)?;
        }
        Ok(())
    }

    fn append_instance(
        &mut self,
        instance: &ModuleInstance,
        parent_source: &Module,
        parent_scope: &ScopeMap,
        module_stack: &mut Vec<SmolStr>,
        path: &str,
    ) -> CompileResult<()> {
        let relative = path
            .strip_prefix(self.flattened.name.as_str())
            .and_then(|suffix| suffix.strip_prefix('.'))
            .ok_or_else(|| {
                internal_error("hierarchy path does not name the selected root".into())
            })?;
        let shared = self.shared_occurrences.get(relative).cloned();
        let child_source = self
            .source_modules
            .get(&instance.module)
            .copied()
            .or_else(|| shared.as_deref().map(|module| &module.source))
            .ok_or_else(|| {
                semantic_error(
                    SemanticErrorKind::UndefinedModule(instance.module.to_string()),
                    instance.span,
                )
            })?;
        let child = self
            .analyzed
            .modules
            .get(&instance.module)
            .or_else(|| shared.as_deref().map(|module| &module.analyzed))
            .ok_or_else(|| {
                internal_error(format!(
                    "module '{}' was retained but not semantically analyzed",
                    instance.module
                ))
            })?;
        // Shared occurrences already passed specialization-aware cycle and
        // resource checks, so finite parameter-recursive hierarchies are legal.
        if shared.is_none() && module_stack.contains(&instance.module) {
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

        let overrides = bind_parameter_overrides(instance, child, path)?;
        self.validate_parameter_array_overrides(
            child_source,
            child,
            parent_source,
            &overrides,
            path,
        )?;
        let specialized = if shared.is_none() {
            self.specialize_parameters(child_source, child, parent_source, &overrides, path)?
        } else {
            None
        };
        let (child_source, child) = if let Some(shared) = shared.as_deref() {
            (&shared.source, &shared.analyzed)
        } else {
            specialized
                .as_deref()
                .map(|value| (&value.source, &value.analyzed))
                .unwrap_or((child_source, child))
        };
        self.flattened.hierarchical_connections |= child.hierarchical_connections;
        let mut branch_inventory = super::flow_probes::hierarchy_branches(child);
        let node_reference_aliases = references::canonicalize_node_references(
            child_source, child, &mut branch_inventory,
        );
        self.references.request(child_source, child, relative, &branch_inventory);
        self.references.extend_inventory(child_source, child, relative, &mut branch_inventory);
        let connections = self.bind_connections(instance, child, parent_scope, path)?;
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
            node_reference_aliases,
            connections: super::node_vectors::ConnectionScope::new(child_source, child),
            parameter_locals: child.parameter_locals.clone(),
            discrete_nets: child
                .digital
                .signals
                .iter()
                .map(|signal| signal.name.clone())
                .collect(),
            ground_nodes: child.ground_nodes.clone(),
            instance_path: Some(path.into()),
            noise_process_range: Some((noise_process_base, child.noise_process_count)),
            ..ScopeMap::default()
        };
        for (port, span) in &branch_inventory.port_flows {
            let token = self.fresh_name("port_flow");
            scope.port_flows.insert(port.clone(), token.clone());
            self.port_flows.insert(
                token,
                PortFlow {
                    owner: path.into(),
                    span: *span,
                    terms: BTreeMap::new(),
                },
            );
        }
        scope.nodes.insert(
            "0".into(),
            NodeBinding {
                port_boundaries: Vec::new(),
                name: "0".into(),
                discipline: None,
            },
        );
        for (port, connection) in child.ports.iter().zip(connections) {
            if child
                .digital
                .signals
                .iter()
                .any(|signal| signal.name == port.name)
            {
                continue;
            }
            let (mut binding, connected) = match connection {
                Some(binding) => (binding, true),
                None => {
                    let name = self.fresh_name(&port.name);
                    let index = self.flattened.internal_nodes.len();
                    self.flattened.internal_nodes.push(AnalyzedInternalNode {
                        name: name.clone(),
                        discipline: port.discipline.clone(),
                        is_state: false,
                        index,
                    });
                    (
                        NodeBinding {
                            port_boundaries: Vec::new(),
                            name,
                            discipline: Some(port.discipline.clone()),
                        },
                        false,
                    )
                }
            };
            if let Some(boundary) = scope.port_flows.get(&port.name) {
                binding.port_boundaries.push(boundary.clone());
            }
            scope.nodes.insert(port.name.clone(), binding);
            scope.port_connected.insert(port.name.clone(), connected);
        }
        for ground in &child.ground_nodes {
            scope.nodes.insert(
                ground.clone(),
                NodeBinding {
                    port_boundaries: Vec::new(),
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
                is_state: node.is_state,
                index,
            });
            scope.nodes.insert(
                node.name.clone(),
                NodeBinding {
                    port_boundaries: Vec::new(),
                    name,
                    discipline: Some(node.discipline.clone()),
                },
            );
        }

        // Allocate all parameter names before rewriting defaults and ranges;
        // this makes forward references diagnostic-preserving instead of
        // accidentally binding to a similarly named parent parameter.
        for parameter in &child.digital.elaboration_parameters {
            let mut retained = parameter.clone();
            retained.name = self.fresh_name(&parameter.name);
            retained.is_public = false;
            scope
                .parameters
                .insert(parameter.name.clone(), retained.name.clone());
            retained.aliases = parameter
                .aliases
                .iter()
                .map(|alias| {
                    scope
                        .parameters
                        .insert(alias.clone(), retained.name.clone());
                    self.fresh_name(alias)
                })
                .collect();
            self.flattened.digital.elaboration_parameters.push(retained);
        }
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
            // Child supplied state is immutable in the hierarchy. Its query
            // has already been resolved in that scope; this hidden numeric slot
            // may still have a symbolic default driven by parent values.
            parameter.elaboration_given = None;
            parameter.default_expr = if parameters::is_packed(&parameter)
                || (!parameter.dimensions.is_empty() && overrides.contains_key(&index))
            {
                parameter.default_expr.clone()
            } else if parameter.elaboration_value.is_some() && overrides.contains_key(&index) {
                // This input is fixed by source elaboration. Its parent inputs
                // are protected through the original override's provenance.
                parameter.default_expr.clone()
            } else if let Some(override_expr) = overrides.get(&index) {
                parameter.default = None;
                let expanded = parent_scope.parameter_locals.expand(override_expr)?;
                Some(rewrite_expression(&expanded, parent_scope)?)
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

        self.parameter_hierarchy.register(
            child_source,
            child,
            &scope,
            Some((parent_source, parent_scope, &overrides)),
        )?;

        let variable_base = self.flattened.variables.len();
        // Backends locate array lanes by base[index]. Preserve that relationship
        // while relocating both the semantic layout and its numeric slots.
        // Sort maps before allocating names to keep artifact identities stable.
        let mut child_arrays: Vec<_> = child.arrays.iter().collect();
        child_arrays.sort_unstable_by(|(left, _), (right, _)| left.cmp(right));
        let mut array_slots = HashMap::new();
        for (name, array) in child_arrays {
            let (mapped_name, names) = loop {
                let mapped_name = self.fresh_name(name);
                let mut names = Vec::with_capacity(array.len);
                let dimensions = if array.dimensions.is_empty() {
                    vec![(
                        array.lower,
                        array
                            .lower
                            .checked_add(array.len as i64 - 1)
                            .ok_or_else(|| {
                                internal_error("hierarchy array index overflow".into())
                            })?,
                    )]
                } else {
                    array.dimensions.clone()
                };
                let layout = crate::array_index::UnpackedArrayLayout::new(
                    &dimensions,
                    MAX_PARAMETER_ARRAY_ELEMENTS as usize,
                )
                .map_err(|_| {
                    internal_error("hierarchy array shape exceeds supported storage".into())
                })?;
                for offset in 0..array.len {
                    let mut name = mapped_name.to_string();
                    for index in layout.indices(offset).expect("array element") {
                        name.push_str(&format!("[{index}]"));
                    }
                    names.push(SmolStr::from(name));
                }
                if names.iter().all(|name| !self.used_names.contains(name)) {
                    break (mapped_name, names);
                }
            };
            for (offset, name) in names.into_iter().enumerate() {
                let slot = array
                    .base
                    .checked_add(offset)
                    .filter(|slot| *slot < child.variables.len())
                    .ok_or_else(|| {
                        internal_error("hierarchy array slot exceeds child storage".into())
                    })?;
                if array_slots.insert(slot, name.clone()).is_some() {
                    return Err(internal_error("overlapping hierarchy array layouts".into()));
                }
                self.used_names.insert(name);
            }
            scope.arrays.insert(name.clone(), mapped_name.clone());
            self.flattened.arrays.insert(
                mapped_name,
                AnalyzedArray {
                    declared_dimensions: array.declared_dimensions.clone(),
                    dimensions: array.dimensions.clone(),
                    base: variable_base + array.base,
                    lower: array.lower,
                    len: array.len,
                },
            );
        }
        for (slot, variable) in child.variables.iter().enumerate() {
            let mut variable = variable.clone();
            let original = variable.name.clone();
            variable.name = array_slots
                .remove(&slot)
                .unwrap_or_else(|| self.fresh_name(&original));
            if let Some(task) = crate::analog_tasks::SIMULATOR_CONTROL_TASK_VARIABLES
                .iter()
                .position(|name| *name == original)
            {
                self.child_control_variables[task].push(variable.name.clone());
            }
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
        for &(value, validity) in &child.discrete_inputs {
            if value >= child.variables.len() || validity >= child.variables.len() {
                return Err(internal_error(
                    "invalid child discrete input storage".into(),
                ));
            }
            self.flattened
                .discrete_inputs
                .push((variable_base + value, variable_base + validity));
            let signal = child
                .discrete_selections
                .iter()
                .find(|selection| selection.value == value)
                .map(|selection| &selection.signal)
                .or_else(|| child.discrete_bindings.get(&value))
                .unwrap_or(&child.variables[value].name);
            let mapped = self.digital_signal_name(relative, signal)?;
            self.flattened
                .discrete_bindings
                .insert(variable_base + value, mapped);
        }
        for selection in &child.discrete_selections {
            let mut selection = selection.clone();
            selection.value += variable_base;
            selection.signal = self.digital_signal_name(relative, &selection.signal)?;
            self.flattened
                .discrete_bindings
                .insert(selection.value, selection.signal.clone());
            self.flattened.discrete_selections.push(selection);
        }
        self.flattened.event_state_variables.sort_unstable();
        self.flattened.event_state_variables.dedup();
        for &slot in &child.switch_branch_variables {
            if child.event_state_variables.binary_search(&slot).is_err() {
                return Err(internal_error(
                    "switch-branch variable is not an event-state slot".into(),
                ));
            }
            self.flattened.switch_branch_variables.push(
                variable_base.checked_add(slot).ok_or_else(|| {
                    internal_error("hierarchy switch-branch index overflow".into())
                })?,
            );
        }
        self.flattened.switch_branch_variables.sort_unstable();
        self.flattened.switch_branch_variables.dedup();
        for branch in &child.branches {
            let mapped_name = self.fresh_name(&branch.name);
            scope
                .branches
                .insert(branch.name.clone(), mapped_name.clone());
            if branch_inventory.conducting_named.contains(&branch.name)
                && !references::is_borrowed_branch(child_source, child, &branch.name)
            {
                self.record_port_flow(
                    &scope,
                    &branch.pos_node,
                    if branch.neg_node.is_empty() {
                        "0"
                    } else {
                        &branch.neg_node
                    },
                    &mapped_name,
                );
            }
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

        // A module owns its unnamed branches even when another instance
        // binds its ports to the same nets. Give those branches private names
        // before endpoint substitution can erase that ownership.
        for ((pos, neg), span) in branch_inventory.unnamed {
            let branch = AnalyzedBranch {
                name: self.fresh_name("branch"),
                pos_node: mapped_node_name(&scope, &pos, span)?,
                neg_node: mapped_node_name(&scope, &neg, span)?,
                discipline: scope
                    .nodes
                    .get(&pos)
                    .and_then(|node| node.discipline.clone())
                    .or_else(|| {
                        scope
                            .nodes
                            .get(&neg)
                            .and_then(|node| node.discipline.clone())
                    })
                    .unwrap_or_else(|| "electrical".into()),
            };
            if branch_inventory
                .conducting_unnamed
                .contains(&(pos.clone(), neg.clone()))
            {
                self.record_port_flow(&scope, &pos, &neg, &branch.name);
            }
            scope.unnamed_branches.insert((pos, neg), branch.clone());
            self.flattened.branches.push(branch);
        }

        self.references.register(child_source, child, relative, &scope)?;

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
                .map(|contribution| rewrite_contribution(contribution, &scope, base, true))
                .collect::<CompileResult<Vec<_>>>()?,
        );
        if let Some(&index) = self.digital_frames.get(relative) {
            let frame = &mut self.flattened.digital.instances[index];
            frame.analog_variables = scope
                .variables
                .iter()
                .chain(&scope.arrays)
                .map(|(local, global)| (local.clone(), global.clone()))
                .collect();
            let mut result = Ok(());
            let mut rewrite = |expression: &mut Expression| {
                if result.is_ok() {
                    result = rewrite_digital_probes(expression, &scope);
                }
            };
            for process in &mut frame.processes {
                super::digital_walk::rewrite_roots(&mut process.body, &mut rewrite);
            }
            for assign in &mut frame.continuous_assigns {
                rewrite(&mut assign.assignment.value);
                if let Some(delay) = &mut assign.assignment.delay {
                    rewrite(delay);
                }
            }
            result?;
        }
        module_stack.push(instance.module.clone());
        let nested = self.append_instances(child_source, &scope, module_stack, path);
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
        child_source: &Module,
        child: &AnalyzedModule,
        parent_source: &Module,
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

        let parent_constants = super::instance_parameters::constants(parent_source);

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
                let expression = super::instance_parameters::close_override(
                    &child_source.parameters[index],
                    override_expression.clone(),
                    &parent_constants,
                    parent_source.time_scale,
                )
                .map_err(|message| {
                    semantic_error(
                        SemanticErrorKind::InvalidExpression(format!(
                            "parameter '{}' of instance '{path}': {message}",
                            parameter.name
                        )),
                        override_expression.span(),
                    )
                })?;
                crate::canonical_ir::digital_lower::elaboration_constant(
                    &expression,
                    &parent_constants,
                    parent_source.time_scale,
                )
                .map(|value| match value {
                    crate::numeric_literal::NumericLiteralValue::Integer(value) => {
                        ConstantValue::Integer(value)
                    }
                    crate::numeric_literal::NumericLiteralValue::Real(value) => {
                        ConstantValue::Real(value)
                    }
                })
                .and_then(|value| {
                    SemanticAnalyzer::constant_for_declared_type(value, parameter.param_type)
                })
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
            // Resolve parent names before materializing counts. The child's
            // effective shape is checked only after all scalar overrides are
            // known, and the retained source will be specialized atomically.
            let closed = super::instance_parameters::close_override(
                &child_source.parameters[index],
                replacement.clone(),
                &parent_constants,
                parent_source.time_scale,
            )
            .map_err(|message| {
                semantic_error(
                    SemanticErrorKind::InvalidExpression(format!(
                        "override of parameter array '{}' at instance '{path}': {message}",
                        parameter.name
                    )),
                    replacement.span(),
                )
            })?;
            let materialized = SemanticAnalyzer::new().materialize_replication_expression(
                &closed,
                MAX_PARAMETER_ARRAY_ELEMENTS as usize,
                super::MAX_REPLICATION_MATERIALIZATION_WORK,
                &format!(
                    "override of parameter array '{}' at instance '{path}'",
                    parameter.name
                ),
                false,
            )?;
            let Expression::ArrayLiteral(initializer) = &materialized else {
                unreachable!("closed array pattern")
            };
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
        let connections = super::node_vectors::bind_connections(
            instance,
            child,
            &parent_scope.connections,
            path,
        )?;
        connections
            .iter()
            .zip(&child.ports)
            .map(|(actual, port)| {
                if child
                    .digital
                    .signals
                    .iter()
                    .any(|signal| signal.name == port.name)
                {
                    Ok(None)
                } else {
                    actual
                        .as_ref()
                        .map(|actual| {
                            resolve_connection(self.analyzed, actual, parent_scope, path, port)
                        })
                        .transpose()
                }
            })
            .collect()
    }

    fn digital_signal_name(&self, path: &str, local: &str) -> CompileResult<SmolStr> {
        let frame = self
            .digital_frames
            .get(path)
            .map(|&index| &self.flattened.digital.instances[index])
            .ok_or_else(|| {
                internal_error(format!("mixed occurrence '{path}' has no digital frame"))
            })?;
        for signal in &frame.signals {
            if local == signal.declared.name {
                return Ok(signal.name.clone());
            }
            if signal.declared.unpacked.is_some()
                && let Some(suffix) = local.strip_prefix(signal.declared.name.as_str())
                && suffix.starts_with('[')
            {
                return Ok(format!("{}{suffix}", signal.name).into());
            }
        }
        Err(internal_error(format!(
            "mixed occurrence '{path}' has no signal '{local}'"
        )))
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

pub(super) fn bind_parameter_overrides(
    instance: &ModuleInstance,
    child: &AnalyzedModule,
    path: &str,
) -> CompileResult<HashMap<usize, Expression>> {
    let names: Vec<_> = child.parameters.iter().map(|parameter| parameter.name.clone()).collect();
    let aliases = child.param_aliases.iter().map(|alias| (alias.alias.clone(), alias.target)).collect();
    super::instance_parameters::bind_overrides(instance, &names, &aliases, path)
}

fn resolve_connection(
    analyzed: &AnalyzedFile,
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
    if parent_scope.discrete_nets.contains(source_name) {
        return Err(semantic_error(
            SemanticErrorKind::UnsupportedFeature(format!(
                "analog port '{}' of instance '{path}' connects discrete net '{source_name}' without a connect module",
                child_port.name
            )),
            expression.span(),
        ));
    }
    let mut binding = parent_scope
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
    if let Some(parent_discipline) = &binding.discipline {
        analyzed
            .connect_rules
            .check_net_compatibility(
                &analyzed.disciplines,
                parent_discipline,
                &child_port.discipline,
                source_name,
            )
            .map_err(|cause| {
                semantic_error(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "discipline mismatch on instance '{path}' port '{}': {cause}",
                        child_port.name,
                    )),
                    expression.span(),
                )
            })?;
    }
    // The shared physical identity does not erase this segment's declaration.
    // Its access functions, branch natures and descendant connections use the
    // discipline authored on the child port, including a port tied to ground.
    binding.discipline = Some(child_port.discipline.clone());
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
        AnalyzedStatement::Initialization {
            phase,
            site,
            body,
            span,
        } => AnalyzedStatement::Initialization {
            phase: *phase,
            site: base.site(*site)?,
            body: body
                .iter()
                .map(|statement| rewrite_statement(statement, scope, base))
                .collect::<CompileResult<Vec<_>>>()?,
            span: *span,
        },
        AnalyzedStatement::Task(task) => {
            let mut rewritten = task.try_map(task.span, |expression| {
                rewrite_expression(expression, scope)
            })?;
            rewritten.site = base.site(AnalogSiteId(task.site))?.0;
            AnalyzedStatement::Task(rewritten)
        }
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
        AnalyzedRegion::Initialization { phase, body, span } => AnalyzedRegion::Initialization {
            phase: *phase,
            body: body
                .iter()
                .map(|region| rewrite_region(region, scope, base))
                .collect::<CompileResult<Vec<_>>>()?,
            span: *span,
        },
        AnalyzedRegion::Task(task) => {
            let mut rewritten = task.try_map(task.span, |expression| {
                rewrite_expression(expression, scope)
            })?;
            rewritten.site = base.site(AnalogSiteId(task.site))?.0;
            AnalyzedRegion::Task(rewritten)
        }
        AnalyzedRegion::Assignment(assignment) => {
            AnalyzedRegion::Assignment(rewrite_assignment(assignment, scope, base)?)
        }
        AnalyzedRegion::Contribution(contribution) => {
            AnalyzedRegion::Contribution(rewrite_contribution(contribution, scope, base, false)?)
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
        occurrence_source: assignment
            .occurrence_source
            .map(|source| -> CompileResult<_> {
                Ok(crate::analog_occurrences::AnalogOccurrenceSource {
                    group: base.site(AnalogSiteId(source.group))?.0,
                    member: source.member,
                })
            })
            .transpose()?,
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
    flat: bool,
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
    let scoped_branch = if contribution.declared_branch.is_none() {
        scope.unnamed_branch(pos, neg.unwrap_or("0"))
    } else {
        None
    };
    let pos = mapped_node_name(scope, pos, contribution.span)?;
    let branch = if let Some(neg) = neg {
        let neg = mapped_node_name(scope, neg, contribution.span)?;
        SmolStr::from(format!("{pos},{neg}"))
    } else {
        pos
    };
    let mut rewritten = AnalyzedContribution {
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
        equation_abstol: contribution
            .equation_abstol
            .as_ref()
            .map(|expr| rewrite_expression(expr, scope))
            .transpose()?,
        expression: rewrite_expression(&contribution.expression, scope)?,
        site: base.site(contribution.site)?,
        expression_guard: contribution.expression_guard,
        expr_type: contribution.expr_type,
        span: contribution.span,
    };
    if let Some((branch, sign)) = scoped_branch {
        rewritten.branch = format!("{},{}", branch.pos_node, branch.neg_node).into();
        rewritten.declared_branch = Some(branch.name.clone());
        if sign < 0.0 {
            super::flow_probes::negate_contribution(&mut rewritten, flat);
        }
    }
    Ok(rewritten)
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
    let mut rewritten = match expression {
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
                    resolved_builtin: call.resolved_builtin,
                    name: call.name.clone(),
                    args,
                    span: call.span,
                })
            }
        }
        Expression::BranchAccess(access) => {
            if let BranchAccess::Nodes {
                pos_indices,
                neg_indices,
                access: name,
                kind,
                pos,
                neg,
                span,
            } = access
                && pos_indices.is_empty()
                && neg_indices.is_empty()
                && !(neg.is_none() && scope.branches.contains_key(pos))
                && let Some((branch, sign)) =
                    scope.unnamed_branch(pos, neg.as_deref().unwrap_or("0"))
            {
                super::flow_probes::signed(
                    Expression::BranchAccess(BranchAccess::Branch {
                        access: name.clone(),
                        kind: *kind,
                        name: branch.name.clone(),
                        span: *span,
                        index: None,
                    }),
                    sign,
                )
            } else {
                Expression::BranchAccess(rewrite_branch_access(access, scope)?)
            }
        }
        Expression::ArrayAccess(access) => Expression::ArrayAccess(ArrayAccessExpr {
            normalized: access.normalized,
            packed: access
                .packed
                .as_ref()
                .map(|packed| {
                    Ok::<_, CompileError>(crate::ast::PackedArrayIndex {
                        bit: Box::new(rewrite_expression(&packed.bit, scope)?),
                        layout: packed.layout,
                    })
                })
                .transpose()?,
            discrete_validity: access.discrete_validity.as_ref().map(|name| {
                scope
                    .arrays
                    .get(name)
                    .cloned()
                    .unwrap_or_else(|| name.clone())
            }),
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
    };
    // A reversed probe becomes -access(private_branch). Keep ddx's axis a
    // branch probe by moving that sign onto the derivative result.
    let args = match &mut rewritten {
        Expression::Call(call) if call.name == "ddx" => Some(&mut call.args),
        Expression::SystemFunction(call) if call.name == "ddx" => Some(&mut call.args),
        _ => None,
    };
    if let Some(args) = args
        && args.len() == 2
        && matches!(&args[1], Expression::Unary(unary) if unary.op == UnaryOp::Neg)
        && let Expression::Unary(unary) = args.remove(1)
    {
        args.push(*unary.operand);
        rewritten = super::flow_probes::signed(rewritten, -1.0);
    }
    Ok(rewritten)
}

/// Relocate only analog probes. Digital names retain lexical/process scope and
/// are resolved by the frame's existing signal and constant tables.
fn rewrite_digital_probes(expression: &mut Expression, scope: &ScopeMap) -> CompileResult<()> {
    let mut pending = vec![expression];
    while let Some(expression) = pending.pop() {
        if matches!(expression, Expression::BranchAccess(_)) {
            *expression = rewrite_expression(expression, scope)?;
        } else {
            super::flow_probes::for_child_mut(expression, &mut |child| pending.push(child));
        }
    }
    Ok(())
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
    if matches!(access, BranchAccess::Nodes { pos_indices, neg_indices, .. } if !pos_indices.is_empty() || !neg_indices.is_empty())
    {
        return Err(internal_error(
            "unresolved physical vector selector reached hierarchy rewriting".into(),
        ));
    }
    Ok(match access {
        BranchAccess::Nodes {
            pos_indices: _,
            neg_indices: _,
            access,
            kind,
            pos,
            neg: None,
            span,
        } if scope.branches.contains_key(pos) => BranchAccess::Branch {
            access: access.clone(),
            kind: *kind,
            name: scope.branches[pos].clone(),
            span: *span,
            index: None,
        },
        BranchAccess::Nodes {
            pos_indices: _,
            neg_indices: _,
            access,
            kind,
            pos,
            neg,
            span,
        } => BranchAccess::Nodes {
            pos_indices: Vec::new(),
            neg_indices: Vec::new(),
            access: access.clone(),
            kind: *kind,
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
        BranchAccess::Branch {
            access,
            kind,
            name,
            span,
            index: _,
        } => BranchAccess::Branch {
            index: None,
            access: access.clone(),
            kind: *kind,
            name: scope
                .branches
                .get(name)
                .or_else(|| scope.port_flows.get(name))
                .cloned()
                .ok_or_else(|| {
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
