//! Semantic Analyzer for Verilog-A/AMS
//!
//! Provides comprehensive semantic analysis including:
//! - Hierarchical symbol table with nested scopes
//! - Type inference and checking
//! - Discipline validation
//! - Expression validation
//! - Parameter range checking

use crate::ast::*;
use crate::disciplines::{Discipline, DisciplineDb, Domain, Nature};
use crate::error::{CompileError, CompileResult, SemanticError, SemanticErrorKind};
use crate::integer_runtime::{IntegerBinaryOperation, integer_binary, real_to_integer};
use crate::numeric_literal::parse_integer_literal;
use crate::source::Span;
use crate::types::{FunctionRegistry, ParameterRange as TypedParameterRange, ValueType};
use smol_str::SmolStr;
use std::collections::{HashMap, HashSet};

const RSPICE_LIMITED_EXP_INTRINSIC: &str = "__rspice_limited_exp";
pub(crate) const MAX_PARAMETER_ARRAY_RANK: usize = 16;
pub(crate) const MAX_PARAMETER_ARRAY_ELEMENTS: u64 = 1_048_576;
const MAX_REPLICATION_NESTING: usize = 128;
const MAX_REPLICATION_MATERIALIZATION_WORK: usize = 4_194_304;
const MAX_ANALOG_FILTER_VECTOR_ELEMENTS: usize =
    crate::zfilter::MAX_ZI_RUNTIME_OPERANDS - crate::zfilter::ZI_FIXED_RUNTIME_OPERANDS;

/// Numeric value retained by compile-time evaluation.
///
/// Verilog-AMS arithmetic is type-sensitive: notably, `1 / 2` is integer
/// division while `1 / 2.0` is real division. Keeping only an `f64` here
/// silently erased that distinction before defaults, bounds, and loop limits
/// were validated.
#[derive(Debug, Clone, Copy, PartialEq)]
enum ConstantValue {
    Integer(i64),
    Real(f64),
}

impl ConstantValue {
    fn as_f64(self) -> f64 {
        match self {
            Self::Integer(value) => value as f64,
            Self::Real(value) => value,
        }
    }

    fn is_real(self) -> bool {
        matches!(self, Self::Real(_))
    }

    fn is_truthy(self) -> bool {
        self.as_f64() != 0.0
    }
}

/// Diagnostic code for a system task that parses and is then discarded.
const NO_EFFECT_SYSTEM_TASK_CODE: &str = "VA-SEM-NO-EFFECT-SYSTEM-TASK";

mod analyzed;
mod elaboration;
mod symbols;

pub use analyzed::*;
pub(crate) use elaboration::elaborate_executable_module;
pub use symbols::*;

// ============================================================================
// Semantic Analyzer
// ============================================================================

/// Semantic analyzer for Verilog-A modules
pub struct SemanticAnalyzer {
    disciplines: DisciplineDb,
    functions: FunctionRegistry,
    symbols: SymbolTable,
    errors: Vec<SemanticError>,
    /// User-defined analog functions of the module under analysis
    user_functions: HashMap<SmolStr, FunctionDef>,
    /// Stack of active guard conditions (innermost last)
    guard_stack: Vec<Expression>,
    /// Structured regions being built, innermost last.
    ///
    /// Runs alongside `guard_stack` and records the shape the guards are about
    /// to erase. The bottom frame is the module's analog block; each nested
    /// construct pushes a frame and pops it into an [`AnalyzedRegion`].
    region_stack: Vec<Vec<AnalyzedRegion>>,
    /// Stack of identifier substitution frames (innermost last). Used for
    /// hoisted block locals, unrolled loop variables, and inlined function
    /// locals.
    subst_stack: Vec<HashMap<SmolStr, Expression>>,
    /// Caller-visible assignments produced by analog function output/inout
    /// arguments while expression lowering is in progress.
    function_side_effects: Vec<AssignmentStmt>,
    /// Counter for generating unique hoisted local names
    local_counter: usize,
    /// Constant parameter default values (compile-time diagnostics only:
    /// instances may override parameters, so these must never influence
    /// generated code)
    param_consts: HashMap<SmolStr, ConstantValue>,
    /// Values that cannot vary per instance (localparams derived purely
    /// from literals). Safe for loop unrolling and code folding.
    invariant_consts: HashMap<SmolStr, ConstantValue>,
    /// Current function inlining depth (recursion guard)
    inline_depth: usize,
    /// Nesting depth of runtime-bounded loops (contributions inside them
    /// are not representable and must error)
    runtime_loop_depth: usize,
    /// Nesting depth of control flow whose selector can change during an
    /// analysis. Stateful analog operators must execute on every Newton
    /// iteration and are therefore illegal beneath such a guard (VAMS-2023
    /// section 4.5.15).
    dynamic_analog_operator_guard_depth: usize,
    current_default_transition: f64,
    /// Array variables of the module under analysis (name -> layout)
    arrays: HashMap<SmolStr, AnalyzedArray>,
    /// Public parameter-array declarations. Until element lowering is wired
    /// through every backend, these names must never acquire scalar semantics.
    parameter_arrays: HashSet<SmolStr>,
    /// Hidden system-task variables ($bound_step, $discontinuity)
    /// registered on first use
    task_vars: HashMap<SmolStr, usize>,
    /// Snapshotted guards for enclosing unfiltered `initial_step` events.
    unfiltered_initial_step_guards: Vec<SmolStr>,
    /// Non-fatal findings for the whole file under analysis, deduplicated by
    /// code and span so a construct inside a statically unrolled loop is
    /// reported once per source site rather than once per iteration.
    warnings: Vec<SemanticWarning>,
}

/// How an event expression lowers into the dataflow representation
enum EventLowering {
    /// Body executes when this runtime guard is nonzero
    Guard(Expression),
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticAnalyzer {
    pub const SIMULATOR_DEFAULT_TRANSITION: f64 = 1.0e-9;

    pub fn new() -> Self {
        Self {
            disciplines: DisciplineDb::with_standard(),
            functions: FunctionRegistry::new(),
            symbols: SymbolTable::new(),
            errors: Vec::new(),
            user_functions: HashMap::new(),
            guard_stack: Vec::new(),
            region_stack: vec![Vec::new()],
            subst_stack: Vec::new(),
            function_side_effects: Vec::new(),
            local_counter: 0,
            param_consts: HashMap::new(),
            invariant_consts: HashMap::new(),
            inline_depth: 0,
            runtime_loop_depth: 0,
            dynamic_analog_operator_guard_depth: 0,
            current_default_transition: Self::SIMULATOR_DEFAULT_TRANSITION,
            arrays: HashMap::new(),
            parameter_arrays: HashSet::new(),
            task_vars: HashMap::new(),
            unfiltered_initial_step_guards: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn analyze(&mut self, source: &SourceFile) -> CompileResult<AnalyzedFile> {
        let mut modules = HashMap::new();
        let mut module_spans = HashMap::new();
        self.warnings.clear();

        // First pass: register user-defined natures, then disciplines that
        // reference them. Access compatibility validation relies on this DB.
        for item in &source.items {
            if let Item::Nature(nature) = item {
                self.register_nature(nature)?;
            }
        }
        for item in &source.items {
            if let Item::Discipline(discipline) = item {
                self.register_discipline(discipline)?;
            }
        }

        // Second pass: analyze modules in declaration order while applying
        // the file-scoped default-transition setting.
        let mut default_transition = Self::SIMULATOR_DEFAULT_TRANSITION;
        for item in &source.items {
            if let Item::DefaultTransition(directive) = item {
                let Some(value) = Self::eval_const_with(&directive.value, &HashMap::new()) else {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::InvalidAnalogOperator(
                            "`default_transition` requires a numeric constant expression".into(),
                        ),
                        directive.span,
                    )));
                };
                if !value.is_finite() || value < 0.0 {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::InvalidAnalogOperator(format!(
                            "`default_transition` must be finite and non-negative, got {value}"
                        )),
                        directive.span,
                    )));
                }
                default_transition = value;
            } else if let Item::Module(module) = item {
                if let Some(first_defined) = module_spans.insert(module.name.clone(), module.span) {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::DuplicateSymbol {
                            name: module.name.clone(),
                            first_defined,
                        },
                        module.span,
                    )));
                }
                self.symbols = SymbolTable::new();
                self.errors.clear();
                self.user_functions.clear();
                self.guard_stack.clear();
                self.region_stack.clear();
                self.region_stack.push(Vec::new());
                self.subst_stack.clear();
                self.function_side_effects.clear();
                self.local_counter = 0;
                self.param_consts.clear();
                self.invariant_consts.clear();
                self.inline_depth = 0;
                self.runtime_loop_depth = 0;
                self.dynamic_analog_operator_guard_depth = 0;
                self.current_default_transition = default_transition;
                self.parameter_arrays.clear();

                match self.analyze_module(module, default_transition) {
                    Ok(analyzed) => {
                        modules.insert(module.name.clone(), analyzed);
                    }
                    Err(e) => return Err(e),
                }
            }
        }

        self.warnings.sort_by_key(|warning| warning.span.start);
        Ok(AnalyzedFile {
            source: source.clone(),
            modules,
            warnings: std::mem::take(&mut self.warnings),
        })
    }

    /// Record a non-fatal finding once per source site.
    ///
    /// Static loop unrolling and analog-function inlining re-walk the same
    /// statements, so a repeat of an identical code and span is the same
    /// construct seen twice, not a second occurrence in the source.
    fn warn(&mut self, code: &'static str, message: String, span: Span) {
        if self
            .warnings
            .iter()
            .any(|warning| warning.code == code && warning.span == span)
        {
            return;
        }
        self.warnings.push(SemanticWarning {
            code,
            message,
            span,
        });
    }

    fn register_nature(&mut self, nature: &NatureDef) -> CompileResult<()> {
        let base = nature
            .base
            .as_deref()
            .map(|base| {
                self.disciplines.get_nature(base).ok_or_else(|| {
                    CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "nature '{}' extends unknown base nature '{}'",
                            nature.name, base
                        )),
                        nature.span,
                    ))
                })
            })
            .transpose()?;

        let access = nature
            .access
            .as_ref()
            .map(|s| s.to_string())
            .or_else(|| base.map(|base| base.access.clone()))
            .ok_or_else(|| {
                CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::MissingAttribute(format!(
                        "access for nature '{}'",
                        nature.name
                    )),
                    nature.span,
                ))
            })?;
        let units = nature
            .units
            .as_ref()
            .map(|s| s.to_string())
            .or_else(|| base.map(|base| base.units.clone()))
            .unwrap_or_default();
        let abstol = nature
            .abstol
            .as_ref()
            .and_then(|expr| self.eval_const(expr))
            .or_else(|| base.map(|base| base.abstol))
            .unwrap_or(0.0);

        self.disciplines.add_nature(Nature {
            name: nature.name.to_string(),
            units,
            abstol,
            access,
            idt_nature: nature.idt_nature.as_ref().map(|s| s.to_string()),
            ddt_nature: nature.ddt_nature.as_ref().map(|s| s.to_string()),
            span: Some(nature.span),
        });
        Ok(())
    }

    fn register_discipline(&mut self, discipline: &DisciplineDef) -> CompileResult<()> {
        for nature_name in [discipline.potential.as_ref(), discipline.flow.as_ref()]
            .into_iter()
            .flatten()
        {
            if self.disciplines.get_nature(nature_name).is_none() {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::UndefinedDiscipline(format!(
                        "discipline '{}' references unknown nature '{}'",
                        discipline.name, nature_name
                    )),
                    discipline.span,
                )));
            }
        }

        let domain = match discipline.domain.unwrap_or(DomainKind::Continuous) {
            DomainKind::Continuous => Domain::Continuous,
            DomainKind::Discrete => Domain::Discrete,
        };
        self.disciplines.add_discipline(Discipline {
            name: discipline.name.to_string(),
            domain,
            potential: discipline.potential.as_ref().map(|s| s.to_string()),
            flow: discipline.flow.as_ref().map(|s| s.to_string()),
            span: Some(discipline.span),
        });
        Ok(())
    }

    fn require_discipline(&self, name: &str, span: Span) -> CompileResult<()> {
        if self.disciplines.get_discipline(name).is_none() {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UndefinedDiscipline(name.to_string()),
                span,
            )));
        }
        Ok(())
    }

    fn analyze_module(
        &mut self,
        module: &Module,
        default_transition: f64,
    ) -> CompileResult<AnalyzedModule> {
        let mut analyzed = AnalyzedModule {
            name: module.name.clone(),
            default_transition,
            ports: Vec::new(),
            parameters: Vec::new(),
            param_aliases: Vec::new(),
            variables: Vec::new(),
            branches: Vec::new(),
            contributions: Vec::new(),
            statements: Vec::new(),
            body: Vec::new(),
            internal_nodes: Vec::new(),
            ground_nodes: Vec::new(),
            arrays: HashMap::new(),
            symbol_table: SymbolTable::new(),
        };
        // Evaluation statements accumulate in a local sink so loop bodies
        // can recurse into their own sinks without aliasing the module
        let mut statements: Vec<AnalyzedStatement> = Vec::new();
        self.user_functions = module
            .functions
            .iter()
            .map(|f| (f.name.clone(), f.clone()))
            .collect();
        self.arrays.clear();
        self.task_vars.clear();

        // Phase 1: Collect port names from module header
        let port_names: Vec<SmolStr> = module.ports.iter().map(|p| p.name.clone()).collect();

        // Phase 2: Process port declarations to get direction and discipline
        let mut port_info: HashMap<SmolStr, (PortDirection, Option<SmolStr>)> = HashMap::new();
        let mut declared_ports: HashMap<SmolStr, Span> = HashMap::new();
        for decl in &module.port_declarations {
            if let Some(discipline) = &decl.discipline {
                self.require_discipline(discipline, decl.span)?;
            }
            for name in &decl.names {
                if let Some(first_defined) = declared_ports.get(name) {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::DuplicateSymbol {
                            name: name.clone(),
                            first_defined: *first_defined,
                        },
                        decl.span,
                    )));
                }
                declared_ports.insert(name.clone(), decl.span);
                port_info.insert(name.clone(), (decl.direction, decl.discipline.clone()));
            }
        }

        // Phase 3: Update port disciplines from net declarations
        for net in &module.nets {
            self.require_discipline(&net.discipline, net.span)?;
            let discipline = net.discipline.clone();
            for name in &net.names {
                // If this is a port, update its discipline
                if let Some((dir, _)) = port_info.get(name) {
                    port_info.insert(name.clone(), (*dir, Some(discipline.clone())));
                }
            }
        }

        // Phase 4: Define ports in symbol table FIRST
        for port_name in &port_names {
            let (direction, discipline) = port_info
                .get(port_name)
                .cloned()
                .unwrap_or((PortDirection::Inout, None));

            let disc_name = discipline.unwrap_or_else(|| "electrical".into());

            let disc = self.disciplines.get_discipline(&disc_name).ok_or_else(|| {
                CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::UndefinedDiscipline(disc_name.to_string()),
                    module.span,
                ))
            })?;
            let potential = disc.potential.as_ref().map(|s| SmolStr::from(s.as_str()));
            let flow = disc.flow.as_ref().map(|s| SmolStr::from(s.as_str()));

            analyzed.ports.push(AnalyzedPort {
                name: port_name.clone(),
                direction,
                discipline: disc_name.clone(),
                nature_potential: potential,
                nature_flow: flow,
            });

            self.define_symbol(Symbol {
                name: port_name.clone(),
                kind: SymbolKind::Port,
                value_type: ValueType::NatureAccess,
                span: Span::new(crate::source::SourceId::new(0), 0, 0),
                attrs: SymbolAttrs {
                    direction: Some(direction),
                    discipline: Some(disc_name),
                    ..Default::default()
                },
            })?;
        }

        // Phase 5: Define internal and ground nodes (nets that aren't ports)
        let mut internal_node_idx = 0usize;
        for net in &module.nets {
            let discipline = net.discipline.clone();
            for name in &net.names {
                // Skip if already defined as a port
                if self.symbols.lookup_local(name).is_some() {
                    continue;
                }

                if net.is_ground {
                    // Ground nets reference the global reference node and
                    // must not consume an internal node slot.
                    self.define_symbol(Symbol {
                        name: name.clone(),
                        kind: SymbolKind::Node,
                        value_type: ValueType::NatureAccess,
                        span: net.span,
                        attrs: SymbolAttrs {
                            discipline: Some(discipline.clone()),
                            is_ground: true,
                            ..Default::default()
                        },
                    })?;
                    analyzed.ground_nodes.push(name.clone());
                    continue;
                }

                // Define as internal node
                self.define_symbol(Symbol {
                    name: name.clone(),
                    kind: SymbolKind::Node,
                    value_type: ValueType::NatureAccess,
                    span: net.span,
                    attrs: SymbolAttrs {
                        discipline: Some(discipline.clone()),
                        is_internal: true,
                        internal_node_index: Some(internal_node_idx),
                        ..Default::default()
                    },
                })?;

                // Add to analyzed internal nodes
                analyzed.internal_nodes.push(AnalyzedInternalNode {
                    name: name.clone(),
                    discipline: discipline.clone(),
                    index: internal_node_idx,
                });
                internal_node_idx += 1;
            }
        }

        // Phase 6: Named branch declarations
        for branch in &module.branches {
            self.validate_node(&branch.pos, branch.span)?;
            if !branch.neg.is_empty() {
                self.validate_node(&branch.neg, branch.span)?;
            }
            let discipline = self
                .symbols
                .lookup(&branch.pos)
                .and_then(|s| s.attrs.discipline.clone())
                .unwrap_or_else(|| "electrical".into());

            analyzed.branches.push(AnalyzedBranch {
                name: branch.name.clone(),
                pos_node: branch.pos.clone(),
                neg_node: branch.neg.clone(),
                discipline: discipline.clone(),
            });

            self.define_symbol(Symbol {
                name: branch.name.clone(),
                kind: SymbolKind::Branch,
                value_type: ValueType::NatureAccess,
                span: branch.span,
                attrs: SymbolAttrs {
                    discipline: Some(discipline),
                    ..Default::default()
                },
            })?;
        }

        // Phase 7: Analyze parameters (defaults may reference earlier ones)
        let param_names: std::collections::HashSet<SmolStr> =
            module.parameters.iter().map(|p| p.name.clone()).collect();
        let parameter_scopes: Vec<_> = module
            .parameters
            .iter()
            .map(|parameter| self.parameter_scope(parameter))
            .collect();
        let parameter_also_model: Vec<_> = module
            .parameters
            .iter()
            .zip(&parameter_scopes)
            .map(|(parameter, scope)| self.parameter_also_model(parameter, *scope))
            .collect();
        let canonical_model_storage = module
            .parameters
            .iter()
            .zip(&parameter_scopes)
            .zip(&parameter_also_model)
            .map(|((parameter, scope), also_model)| {
                (
                    parameter.name.clone(),
                    *scope == ParameterScope::Model || *also_model,
                )
            })
            .collect::<std::collections::HashMap<_, _>>();
        let mut external_model_storage = canonical_model_storage
            .iter()
            .map(|(name, has_model_storage)| (name.to_ascii_lowercase(), *has_model_storage))
            .collect::<std::collections::HashMap<_, _>>();
        for alias in &module.aliasparams {
            if let Some(has_model_storage) = canonical_model_storage.get(&alias.target) {
                external_model_storage.insert(alias.alias.to_ascii_lowercase(), *has_model_storage);
            }
        }
        // Preserve the first case-sensitive declaration so duplicate names do
        // not perturb dependency order before the symbol-table diagnostic.
        // The same table serves scalar defaults and array bounds.
        let parameter_indices = module.parameters.iter().enumerate().fold(
            HashMap::new(),
            |mut indices, (index, parameter)| {
                indices.entry(parameter.name.clone()).or_insert(index);
                indices
            },
        );
        self.parameter_arrays.extend(
            module
                .parameters
                .iter()
                .filter(|parameter| !parameter.dimensions.is_empty())
                .map(|parameter| parameter.name.clone()),
        );
        self.validate_parameter_default_dependencies(
            &module.parameters,
            &module.aliasparams,
            &parameter_indices,
        );
        for (parameter_index, ((param, scope), also_model)) in module
            .parameters
            .iter()
            .zip(parameter_scopes)
            .zip(parameter_also_model)
            .enumerate()
        {
            let is_parameter_array = !param.dimensions.is_empty();
            let materialized_array_default = if is_parameter_array {
                param
                    .default
                    .as_ref()
                    .map(|default| {
                        self.materialize_replication_expression(
                            default,
                            MAX_PARAMETER_ARRAY_ELEMENTS as usize,
                            MAX_REPLICATION_MATERIALIZATION_WORK,
                            &format!("default of parameter array '{}'", param.name),
                            false,
                        )
                    })
                    .transpose()?
            } else {
                None
            };
            if is_parameter_array {
                self.validate_parameter_array_declaration(
                    param,
                    materialized_array_default.as_ref(),
                    parameter_index,
                    &module.parameters,
                    &parameter_indices,
                );
            }
            if scope == ParameterScope::Model || also_model {
                let default_reads_instance = param.default.as_ref().is_some_and(|expression| {
                    Self::references_parameter_without_model_storage(
                        expression,
                        &canonical_model_storage,
                        &external_model_storage,
                    )
                });
                let shape_reads_instance = param.dimensions.iter().any(|dimension| {
                    [&dimension.start, &dimension.end]
                        .into_iter()
                        .any(|expression| {
                            Self::references_parameter_without_model_storage(
                                expression,
                                &canonical_model_storage,
                                &external_model_storage,
                            )
                        })
                });
                // Range bounds are validation expressions, not part of the
                // model-card value. CMC models legitimately constrain a model
                // parameter against instance geometry (for example XGL <= L).
                // Those bounds are evaluated after instance overrides and do
                // not make the model parameter itself instance-owned.
                if default_reads_instance {
                    let storage = if scope == ParameterScope::Model {
                        "model parameter"
                    } else {
                        "dual-scope parameter model fallback"
                    };
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "{storage} '{}' cannot depend on an instance parameter that lacks model storage",
                            param.name,
                        )),
                        param.span,
                    );
                }
                if shape_reads_instance {
                    let storage = if scope == ParameterScope::Model {
                        "model parameter array"
                    } else {
                        "dual-scope parameter-array model fallback"
                    };
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "{storage} '{}' cannot have dimensions that depend on an instance parameter lacking model storage",
                            param.name,
                        )),
                        param.span,
                    );
                }
            }
            let value_type = match param.param_type {
                ParamType::Real => ValueType::Real,
                ParamType::Integer => ValueType::Integer,
                ParamType::String => ValueType::String,
            };

            // Resolve the declaration-time value for diagnostics even when
            // the executable default must remain symbolic. Keeping those two
            // concerns separate lets a later array bound be checked through a
            // transitive chain without baking overridable values into code.
            let declared_default_value = param
                .default
                .as_ref()
                .and_then(|expression| self.eval_const_value(expression));
            let declared_default = declared_default_value.map(ConstantValue::as_f64);

            // A default that references other parameters must stay
            // symbolic: instance overrides of those parameters change it,
            // so it is evaluated per instance at setup time.
            let default = if is_parameter_array
                || param
                    .default
                    .as_ref()
                    .is_some_and(|e| Self::references_identifiers(e, &param_names))
            {
                None
            } else {
                declared_default
            };

            if param.param_type == ParamType::Integer
                && let Some(default) = declared_default
                && (default.fract() != 0.0
                    || default < f64::from(i32::MIN)
                    || default > f64::from(i32::MAX))
            {
                self.record_error_at(
                    SemanticErrorKind::TypeMismatch {
                        expected: "32-bit integer".into(),
                        found: default.to_string(),
                        context: format!("default of parameter '{}'", param.name),
                    },
                    param.span,
                );
            }

            if !is_parameter_array && let Some(declared_range) = &param.range {
                if declared_range.bounds.len() > 1 {
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(
                            "multiple parameter 'from' ranges are not yet supported".into(),
                        ),
                        declared_range.span,
                    );
                }
                let has_unresolved_constraint = declared_range
                    .bounds
                    .iter()
                    .flat_map(|bound| [bound.lower.as_ref(), bound.upper.as_ref()])
                    .flatten()
                    .chain(declared_range.exclude.iter())
                    .any(|expression| {
                        !Self::references_identifiers(expression, &param_names)
                            && self.eval_const(expression).is_none()
                            && Self::direct_parameter_reference(expression, &param_names).is_none()
                    });
                if has_unresolved_constraint {
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(
                            "parameter range expressions must be constant or depend only on parameters"
                                .into(),
                        ),
                        declared_range.span,
                    );
                }
                let own_name = std::collections::HashSet::from([param.name.clone()]);
                let has_self_referential_constraint = declared_range
                    .bounds
                    .iter()
                    .flat_map(|bound| [bound.lower.as_ref(), bound.upper.as_ref()])
                    .flatten()
                    .chain(declared_range.exclude.iter())
                    .any(|expression| Self::references_identifiers(expression, &own_name));
                if has_self_referential_constraint {
                    self.record_error_at(
                        SemanticErrorKind::CircularDependency(format!(
                            "range of parameter '{}'",
                            param.name
                        )),
                        declared_range.span,
                    );
                }
            }

            // Parse parameter range if present
            let range = if is_parameter_array {
                None
            } else {
                param
                    .range
                    .as_ref()
                    .map(|r| self.parse_range(r, &param_names))
            };

            // Validate default against range
            if let (Some(default_val), Some(range_constraint)) = (declared_default, &range)
                && !range_constraint.contains(default_val)
            {
                self.record_error_at(
                    SemanticErrorKind::ParameterOutOfRange {
                        name: param.name.clone(),
                        value: default_val,
                        range: format!("{}", range_constraint),
                    },
                    param.span,
                );
            }

            if let Some(value) = declared_default_value
                .and_then(|value| Self::constant_for_declared_type(value, param.param_type))
            {
                self.param_consts.insert(param.name.clone(), value);
            }

            analyzed.parameters.push(AnalyzedParameter {
                name: param.name.clone(),
                is_public: true,
                scope,
                also_model,
                param_type: param.param_type,
                value_type,
                dimensions: param
                    .dimensions
                    .iter()
                    .map(|dimension| AnalyzedParameterDimension {
                        left: dimension.start.clone(),
                        right: dimension.end.clone(),
                        span: dimension.span,
                    })
                    .collect(),
                default,
                default_expr: materialized_array_default.or_else(|| param.default.clone()),
                range: range.clone(),
            });

            self.define_symbol(Symbol {
                name: param.name.clone(),
                kind: SymbolKind::Parameter,
                value_type,
                span: param.span,
                attrs: SymbolAttrs {
                    range,
                    ..Default::default()
                },
            })?;
        }

        // Phase 7b: Parameter aliases (aliasparam). The target must be a
        // declared parameter; the alias name must not collide with any
        // other declaration. The alias enters the symbol table only to
        // reserve its name - it is not a parameter, so the module body
        // cannot reference it and it gets no storage or default.
        for decl in &module.aliasparams {
            let Some(target) = analyzed
                .parameters
                .iter()
                .position(|p| p.name == decl.target)
            else {
                self.record_error_at(
                    SemanticErrorKind::UndeclaredSymbol {
                        name: decl.target.clone(),
                    },
                    decl.span,
                );
                continue;
            };

            self.define_symbol(Symbol {
                name: decl.alias.clone(),
                kind: SymbolKind::ParamAlias,
                value_type: analyzed.parameters[target].value_type,
                span: decl.span,
                attrs: Default::default(),
            })?;

            analyzed.param_aliases.push(AnalyzedParamAlias {
                alias: decl.alias.clone(),
                target,
            });
        }

        // Pre-pass for Phase 8: seed the constant environments with
        // localparam values so array bounds may reference them (their full
        // lowering to computed variables happens in Phase 9)
        for localparam in &module.localparams {
            if !localparam.dimensions.is_empty() {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "localparam array '{}' is retained with its declared dimensions, but array-valued localparam storage and indexing are not implemented",
                        localparam.name
                    )),
                    localparam.span,
                )));
            }
            if let Some(default) = &localparam.default {
                if let Some(value) = self.eval_const_value(default).and_then(|value| {
                    Self::constant_for_declared_type(value, localparam.param_type)
                }) {
                    self.param_consts.insert(localparam.name.clone(), value);
                }
                if let Some(value) = self.eval_const_invariant_value(default).and_then(|value| {
                    Self::constant_for_declared_type(value, localparam.param_type)
                }) {
                    self.invariant_consts.insert(localparam.name.clone(), value);
                }
            }
        }

        // Phase 8: Analyze variables
        for var_decl in &module.variables {
            let value_type = match var_decl.var_type {
                VarType::Real => ValueType::Real,
                VarType::Integer => ValueType::Integer,
                VarType::String => ValueType::String,
            };

            for item in &var_decl.items {
                if !item.dimensions.is_empty() {
                    let name = item.name.clone();
                    if let Some(layout) =
                        self.register_array_variable(item, var_decl.var_type, &name, &mut analyzed)
                    {
                        analyzed.arrays.insert(name.clone(), layout.clone());
                        self.arrays.insert(name.clone(), layout);
                        self.define_symbol(Symbol {
                            name,
                            kind: SymbolKind::Variable,
                            value_type,
                            span: var_decl.span,
                            attrs: Default::default(),
                        })?;
                    }
                    continue;
                }

                analyzed.variables.push(AnalyzedVariable {
                    name: item.name.clone(),
                    var_type: var_decl.var_type,
                    value_type,
                    is_state: false,
                });

                self.define_symbol(Symbol {
                    name: item.name.clone(),
                    kind: SymbolKind::Variable,
                    value_type,
                    span: var_decl.span,
                    attrs: Default::default(),
                })?;
            }
        }

        // Phase 9: Lower localparams to computed variables. Their values may
        // depend on parameters, so they are evaluated at runtime before any
        // analog-block assignment, in declaration order.
        for localparam in &module.localparams {
            let value_type = match localparam.param_type {
                ParamType::Real => ValueType::Real,
                ParamType::Integer => ValueType::Integer,
                ParamType::String => ValueType::String,
            };

            let Some(default) = &localparam.default else {
                self.record_error_at(
                    SemanticErrorKind::MissingAttribute(format!(
                        "localparam '{}' requires a value",
                        localparam.name
                    )),
                    localparam.span,
                );
                continue;
            };

            if let Some(value) = self
                .eval_const_value(default)
                .and_then(|value| Self::constant_for_declared_type(value, localparam.param_type))
            {
                self.param_consts.insert(localparam.name.clone(), value);
            }
            // A localparam derived purely from literals (and other
            // invariant localparams) cannot vary per instance, so it may
            // participate in loop unrolling and other code folding
            if let Some(value) = self
                .eval_const_invariant_value(default)
                .and_then(|value| Self::constant_for_declared_type(value, localparam.param_type))
            {
                self.invariant_consts.insert(localparam.name.clone(), value);
            }

            let var_index = analyzed.variables.len();
            analyzed.variables.push(AnalyzedVariable {
                name: localparam.name.clone(),
                var_type: match localparam.param_type {
                    ParamType::Real => VarType::Real,
                    ParamType::Integer => VarType::Integer,
                    ParamType::String => VarType::String,
                },
                value_type,
                is_state: false,
            });

            self.define_symbol(Symbol {
                name: localparam.name.clone(),
                kind: SymbolKind::Parameter,
                value_type,
                span: localparam.span,
                attrs: Default::default(),
            })?;

            let expression =
                self.lower_expression_with_side_effects(default, &mut analyzed, &mut statements)?;
            let expr_type = self.infer_type(&expression)?;
            statements.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
                target: localparam.name.clone(),
                var_index,
                index: None,
                expression,
                expr_type,
                span: localparam.span,
                unfiltered_initial_step_guard: None,
            }));
        }

        // Phase 10: Module-level variable initializers run before the
        // analog block, in declaration order.
        for var_decl in &module.variables {
            for item in &var_decl.items {
                let Some(init) = &item.init else { continue };

                if let Some(layout) = self.arrays.get(&item.name).cloned() {
                    // Array initializer: '{e0, e1, ...} fills the elements
                    // in declaration order
                    let Expression::ArrayLiteral(lit) = init else {
                        self.record_error_at(
                            SemanticErrorKind::TypeMismatch {
                                expected: "array literal".to_string(),
                                found: "scalar expression".to_string(),
                                context: format!("initializer of array '{}'", item.name),
                            },
                            item.span,
                        );
                        continue;
                    };
                    if lit.elements.len() != layout.len {
                        self.record_error_at(
                            SemanticErrorKind::TypeMismatch {
                                expected: format!("{} elements", layout.len),
                                found: format!("{} elements", lit.elements.len()),
                                context: format!("initializer of array '{}'", item.name),
                            },
                            item.span,
                        );
                        continue;
                    }
                    if let Some(replication) = lit.first_replication() {
                        self.record_error_at(
                            SemanticErrorKind::UnsupportedFeature(
                                "replication in executable array initializers is parsed but not yet supported; write the elements explicitly"
                                    .into(),
                            ),
                            replication.span,
                        );
                        continue;
                    }
                    for (offset, element) in lit.elements.iter().enumerate() {
                        let ArrayLiteralElement::Value(element) = element else {
                            unreachable!("replication was rejected before array lowering");
                        };
                        let var_index = layout.base + offset;
                        let expression = self.lower_expression_with_side_effects(
                            element,
                            &mut analyzed,
                            &mut statements,
                        )?;
                        let expr_type = self.infer_type(&expression)?;
                        statements.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
                            target: analyzed.variables[var_index].name.clone(),
                            var_index,
                            index: None,
                            expression,
                            expr_type,
                            span: item.span,
                            unfiltered_initial_step_guard: None,
                        }));
                    }
                    continue;
                }

                let var_index = analyzed
                    .variables
                    .iter()
                    .position(|v| v.name == item.name)
                    .expect("variable registered above");
                let expression =
                    self.lower_expression_with_side_effects(init, &mut analyzed, &mut statements)?;
                let expr_type = self.infer_type(&expression)?;
                statements.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
                    target: item.name.clone(),
                    var_index,
                    index: None,
                    expression,
                    expr_type,
                    span: item.span,
                    unfiltered_initial_step_guard: None,
                }));
            }
        }

        // Phase 11: analog initial runs before the main analog block
        if let Some(block) = &module.analog_initial {
            for stmt in &block.statements {
                self.analyze_statement(stmt, &mut analyzed, &mut statements)?;
            }
        }

        // Phase 12: Analyze analog block
        if let Some(block) = &module.analog_block {
            for stmt in &block.statements {
                self.analyze_statement(stmt, &mut analyzed, &mut statements)?;
            }
        }

        analyzed.statements = statements;
        analyzed.body = self.take_body();

        // Surface every recorded diagnostic instead of silently succeeding
        if !self.errors.is_empty() {
            let errors = std::mem::take(&mut self.errors)
                .into_iter()
                .map(CompileError::Semantic)
                .collect();
            return Err(CompileError::multiple(errors));
        }

        analyzed.symbol_table = self.symbols.clone();
        Ok(analyzed)
    }

    /// Interpret the CMC parameter storage convention without letting backend
    /// policy override what the Verilog-A source declares.
    fn parameter_scope(&mut self, parameter: &ParameterDecl) -> ParameterScope {
        let mut scope = ParameterScope::Model;
        let mut declared = false;
        for attribute in &parameter.attributes {
            if !attribute.name.eq_ignore_ascii_case("type") {
                continue;
            }
            let Some(Expression::StringLit(value)) = &attribute.value else {
                self.record_error_at(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "parameter '{}' attribute 'type' must be the string \"model\" or \
                         \"instance\"",
                        parameter.name
                    )),
                    attribute.span,
                );
                continue;
            };
            let parsed = if value.value.eq_ignore_ascii_case("model") {
                Some(ParameterScope::Model)
            } else if value.value.eq_ignore_ascii_case("instance") {
                Some(ParameterScope::Instance)
            } else {
                None
            };
            let Some(parsed) = parsed else {
                self.record_error_at(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "parameter '{}' has unsupported type attribute {:?}; expected \"model\" \
                         or \"instance\"",
                        parameter.name, value.value
                    )),
                    attribute.span,
                );
                continue;
            };
            if declared && scope != parsed {
                self.record_error_at(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "parameter '{}' has conflicting model/instance type attributes",
                        parameter.name
                    )),
                    attribute.span,
                );
                continue;
            }
            scope = parsed;
            declared = true;
        }
        scope
    }

    /// Interpret Xyce's CMC dual-scope extension. Xyce's ADMS templates gate
    /// this extension on attribute presence and intentionally ignore its value,
    /// so even a bare attribute or `xyceAlsoModel="no"` enables model storage.
    /// An explicit instance value remains independently given and takes
    /// precedence over the model-card fallback.
    fn parameter_also_model(&mut self, parameter: &ParameterDecl, scope: ParameterScope) -> bool {
        let present = parameter
            .attributes
            .iter()
            .any(|attribute| attribute.name.eq_ignore_ascii_case("xyceAlsoModel"));
        if present && scope != ParameterScope::Instance {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "parameter '{}' may use 'xyceAlsoModel' only with type=\"instance\"",
                    parameter.name
                )),
                parameter.span,
            );
            return false;
        }
        present
    }

    fn define_symbol(&mut self, symbol: Symbol) -> CompileResult<()> {
        if let Err(existing) = self.symbols.define(symbol.clone()) {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::DuplicateSymbol {
                    name: symbol.name,
                    first_defined: existing.span,
                },
                symbol.span,
            )));
        }
        Ok(())
    }

    /// Hard cap on array storage so a typo in a bound cannot silently
    /// allocate gigabytes of per-instance state
    const MAX_ARRAY_ELEMENTS: usize = 65_536;

    /// Register a 1-D array variable: its elements become contiguous
    /// `name[k]` slots in the variable storage (named after `storage_name`,
    /// which differs from the declared name for hoisted block locals).
    /// Bounds must fold to instance-invariant constants
    /// (parameter-dependent shapes would make the storage layout vary per
    /// instance).
    fn register_array_variable(
        &mut self,
        item: &VariableItem,
        var_type: VarType,
        storage_name: &SmolStr,
        analyzed: &mut AnalyzedModule,
    ) -> Option<AnalyzedArray> {
        if item.dimensions.len() != 1 {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "multi-dimensional array '{}' is not supported",
                    item.name
                )),
                item.span,
            );
            return None;
        }
        let dim = &item.dimensions[0];
        let (Some(start), Some(end)) = (
            self.eval_const_invariant(&dim.start),
            self.eval_const_invariant(&dim.end),
        ) else {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "array '{}' bounds must be compile-time constants",
                    item.name
                )),
                dim.span,
            );
            return None;
        };
        let (start, end) = (start.round() as i64, end.round() as i64);
        // The LRM writes ranges [lo:hi]; accept either order
        let (lower, upper) = if start <= end {
            (start, end)
        } else {
            (end, start)
        };
        let len = (upper - lower + 1) as usize;
        if len > Self::MAX_ARRAY_ELEMENTS {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "array '{}' has {len} elements (limit {})",
                    item.name,
                    Self::MAX_ARRAY_ELEMENTS
                )),
                dim.span,
            );
            return None;
        }
        let value_type = match var_type {
            VarType::Real => ValueType::Real,
            VarType::Integer => ValueType::Integer,
            VarType::String => ValueType::String,
        };
        let base = analyzed.variables.len();
        for k in lower..=upper {
            analyzed.variables.push(AnalyzedVariable {
                name: SmolStr::from(format!("{storage_name}[{k}]")),
                var_type,
                value_type,
                is_state: false,
            });
        }
        Some(AnalyzedArray { base, lower, len })
    }

    /// Fold the active guard stack into a single condition expression
    fn current_guard(&self) -> Option<Expression> {
        let mut guards = self.guard_stack.iter();
        let first = guards.next()?.clone();
        Some(guards.fold(first, |acc, g| {
            let span = g.span();
            Expression::Binary(BinaryExpr {
                op: BinaryOp::And,
                left: Box::new(acc),
                right: Box::new(g.clone()),
                span,
            })
        }))
    }

    /// Record a step in the innermost open region.
    ///
    /// Called at each point [`Self::apply_guard`] is about to fold a guard into
    /// an expression, with the expression as written.
    fn record_region(&mut self, region: AnalyzedRegion) {
        if let Some(frame) = self.region_stack.last_mut() {
            frame.push(region);
        }
    }

    /// Begin collecting a nested region.
    fn open_region(&mut self) {
        self.region_stack.push(Vec::new());
    }

    /// Finish the innermost region and return what it collected.
    fn close_region(&mut self) -> Vec<AnalyzedRegion> {
        // The bottom frame is the analog block itself and is closed by
        // `take_body`; popping it here would silently discard the module.
        debug_assert!(
            self.region_stack.len() > 1,
            "close_region beyond the analog block frame"
        );
        self.region_stack.pop().unwrap_or_default()
    }

    /// Take the module's structured body, leaving a fresh frame behind.
    fn take_body(&mut self) -> Vec<AnalyzedRegion> {
        debug_assert_eq!(
            self.region_stack.len(),
            1,
            "analog block finished with {} regions still open",
            self.region_stack.len().saturating_sub(1)
        );
        let body = self.region_stack.pop().unwrap_or_default();
        self.region_stack.push(Vec::new());
        body
    }

    /// Build `guard ? value : fallback` under the active guard (or `value`
    /// when unguarded)
    fn apply_guard(&self, value: Expression, fallback: Expression) -> Expression {
        match self.current_guard() {
            Some(guard) => {
                let span = value.span();
                Expression::Conditional(ConditionalExpr {
                    condition: Box::new(guard),
                    then_expr: Box::new(value),
                    else_expr: Box::new(fallback),
                    span,
                })
            }
            None => value,
        }
    }

    fn not_expr(expr: Expression) -> Expression {
        let span = expr.span();
        Expression::Unary(UnaryExpr {
            op: UnaryOp::Not,
            operand: Box::new(expr),
            span,
        })
    }

    fn binary_expr(op: BinaryOp, left: Expression, right: Expression) -> Expression {
        let span = left.span();
        Expression::Binary(BinaryExpr {
            op,
            left: Box::new(left),
            right: Box::new(right),
            span,
        })
    }

    fn number_expr(value: f64, span: Span) -> Expression {
        Expression::Number(NumberLit {
            value,
            raw: SmolStr::default(),
            span,
        })
    }

    fn value_type_for_var_type(var_type: VarType) -> ValueType {
        match var_type {
            VarType::Real => ValueType::Real,
            VarType::Integer => ValueType::Integer,
            VarType::String => ValueType::String,
        }
    }

    fn register_function_temp(
        &mut self,
        module: &mut AnalyzedModule,
        name: SmolStr,
        var_type: VarType,
        span: Span,
    ) -> CompileResult<()> {
        module.variables.push(AnalyzedVariable {
            name: name.clone(),
            var_type,
            value_type: Self::value_type_for_var_type(var_type),
            is_state: false,
        });
        self.define_symbol(Symbol {
            name,
            kind: SymbolKind::Variable,
            value_type: Self::value_type_for_var_type(var_type),
            span,
            attrs: Default::default(),
        })
    }

    /// Whether a call to this function *must* be lowered at a statement
    /// boundary. An output or inout argument writes a caller variable, which an
    /// expression cannot do.
    fn function_needs_materialization(func: &FunctionDef) -> bool {
        func.params
            .iter()
            .any(|param| param.direction != ParamDirection::Input)
    }

    /// Whether a call to this function *should* be lowered at a statement
    /// boundary even when it need not be.
    ///
    /// A body with control flow should, because the alternative —
    /// [`Self::inline_function`] — dissolves that control flow into
    /// `guard ? value : previous` and duplicates the whole tree built so far at
    /// every assignment. Materialising instead runs the body through the
    /// ordinary statement path, where one `if` stays one conditional.
    ///
    /// Measured on `EPFL_HEMT_10a`, whose `core` nests three arms over five
    /// chained locals: 186,444 HIR expressions from an analog block holding 191
    /// assignments.
    fn function_should_materialize(func: &FunctionDef) -> bool {
        if Self::function_needs_materialization(func) {
            return true;
        }
        // A clamped exponential is recognised and replaced by an intrinsic
        // further down, which is both smaller and differentiable in closed
        // form. Hoisting it to a statement would happen first and take that
        // away, so its conditionals are left alone.
        if Self::is_recognized_limited_exp_function(func) {
            return false;
        }
        func.body.statements.iter().any(Self::statement_branches)
    }

    fn statement_branches(statement: &AnalogStatement) -> bool {
        match statement {
            AnalogStatement::Conditional(_)
            | AnalogStatement::Case(_)
            | AnalogStatement::While(_)
            | AnalogStatement::For(_)
            | AnalogStatement::Repeat(_) => true,
            AnalogStatement::Block(block) => block.statements.iter().any(Self::statement_branches),
            _ => false,
        }
    }

    fn is_recognized_limited_exp_function(func: &FunctionDef) -> bool {
        if !func.name.eq_ignore_ascii_case("lexp")
            || func.return_type != VarType::Real
            || func.params.len() != 1
            || func.params[0].direction != ParamDirection::Input
            || func.params[0].param_type != VarType::Real
        {
            return false;
        }

        let input = &func.params[0].name;
        let mut has_exp_return = false;
        let mut has_upper_linear_return = false;
        let mut has_lower_clamp_return = false;
        Self::collect_limited_exp_return_features(
            &func.body.statements,
            &func.name,
            input,
            &mut has_exp_return,
            &mut has_upper_linear_return,
            &mut has_lower_clamp_return,
        );
        has_exp_return && has_upper_linear_return && has_lower_clamp_return
    }

    fn collect_limited_exp_return_features(
        statements: &[AnalogStatement],
        return_name: &SmolStr,
        input_name: &SmolStr,
        has_exp_return: &mut bool,
        has_upper_linear_return: &mut bool,
        has_lower_clamp_return: &mut bool,
    ) {
        for statement in statements {
            match statement {
                AnalogStatement::Assignment(assignment) if matches!(&assignment.target, LValue::Variable { name, .. } if name == return_name) =>
                {
                    let value = &assignment.value;
                    if Self::expr_contains_call(value, "exp")
                        && Self::expr_contains_identifier(value, input_name)
                    {
                        *has_exp_return = true;
                    }
                    if Self::expr_contains_identifier(value, input_name)
                        && Self::expr_contains_number_close(value, 5.540622384e34)
                    {
                        *has_upper_linear_return = true;
                    }
                    if Self::expr_contains_number_close(value, 1.804851387e-35) {
                        *has_lower_clamp_return = true;
                    }
                }
                AnalogStatement::Conditional(conditional) => {
                    Self::collect_limited_exp_return_features(
                        std::slice::from_ref(&conditional.then_branch),
                        return_name,
                        input_name,
                        has_exp_return,
                        has_upper_linear_return,
                        has_lower_clamp_return,
                    );
                    if let Some(else_branch) = &conditional.else_branch {
                        Self::collect_limited_exp_return_features(
                            std::slice::from_ref(else_branch),
                            return_name,
                            input_name,
                            has_exp_return,
                            has_upper_linear_return,
                            has_lower_clamp_return,
                        );
                    }
                }
                AnalogStatement::Block(block) => Self::collect_limited_exp_return_features(
                    &block.statements,
                    return_name,
                    input_name,
                    has_exp_return,
                    has_upper_linear_return,
                    has_lower_clamp_return,
                ),
                AnalogStatement::Case(case) => {
                    for item in &case.items {
                        Self::collect_limited_exp_return_features(
                            std::slice::from_ref(&item.statement),
                            return_name,
                            input_name,
                            has_exp_return,
                            has_upper_linear_return,
                            has_lower_clamp_return,
                        );
                    }
                    if let Some(default) = &case.default {
                        Self::collect_limited_exp_return_features(
                            std::slice::from_ref(default),
                            return_name,
                            input_name,
                            has_exp_return,
                            has_upper_linear_return,
                            has_lower_clamp_return,
                        );
                    }
                }
                _ => {}
            }
        }
    }

    fn expr_contains_identifier(expr: &Expression, expected: &SmolStr) -> bool {
        match expr {
            Expression::Identifier(identifier) => &identifier.name == expected,
            Expression::Binary(binary) => {
                Self::expr_contains_identifier(&binary.left, expected)
                    || Self::expr_contains_identifier(&binary.right, expected)
            }
            Expression::Unary(unary) => Self::expr_contains_identifier(&unary.operand, expected),
            Expression::Conditional(conditional) => {
                Self::expr_contains_identifier(&conditional.condition, expected)
                    || Self::expr_contains_identifier(&conditional.then_expr, expected)
                    || Self::expr_contains_identifier(&conditional.else_expr, expected)
            }
            Expression::Call(call) => call
                .args
                .iter()
                .any(|arg| Self::expr_contains_identifier(arg, expected)),
            Expression::SystemFunction(function) => function
                .args
                .iter()
                .any(|arg| Self::expr_contains_identifier(arg, expected)),
            Expression::ArrayAccess(access) => {
                Self::expr_contains_identifier(&access.index, expected)
            }
            Expression::ArrayLiteral(array) => array
                .elements
                .iter()
                .any(|element| Self::array_element_contains_identifier(element, expected)),
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::NullArgument(_)
            | Expression::BranchAccess(_)
            | Expression::AnalogOperator(_)
            | Expression::NoiseSource(_) => false,
        }
    }

    fn array_element_contains_identifier(
        element: &ArrayLiteralElement,
        expected: &SmolStr,
    ) -> bool {
        match element {
            ArrayLiteralElement::Value(expression) => {
                Self::expr_contains_identifier(expression, expected)
            }
            ArrayLiteralElement::Replication(replication) => {
                Self::expr_contains_identifier(&replication.count, expected)
                    || replication
                        .elements
                        .iter()
                        .any(|element| Self::array_element_contains_identifier(element, expected))
            }
        }
    }

    fn expr_contains_call(expr: &Expression, expected: &str) -> bool {
        match expr {
            Expression::Call(call) => {
                call.name.eq_ignore_ascii_case(expected)
                    || call
                        .args
                        .iter()
                        .any(|arg| Self::expr_contains_call(arg, expected))
            }
            Expression::Binary(binary) => {
                Self::expr_contains_call(&binary.left, expected)
                    || Self::expr_contains_call(&binary.right, expected)
            }
            Expression::Unary(unary) => Self::expr_contains_call(&unary.operand, expected),
            Expression::Conditional(conditional) => {
                Self::expr_contains_call(&conditional.condition, expected)
                    || Self::expr_contains_call(&conditional.then_expr, expected)
                    || Self::expr_contains_call(&conditional.else_expr, expected)
            }
            Expression::SystemFunction(function) => function
                .args
                .iter()
                .any(|arg| Self::expr_contains_call(arg, expected)),
            Expression::ArrayAccess(access) => Self::expr_contains_call(&access.index, expected),
            Expression::ArrayLiteral(array) => array
                .elements
                .iter()
                .any(|element| Self::array_element_contains_call(element, expected)),
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::NullArgument(_)
            | Expression::Identifier(_)
            | Expression::BranchAccess(_)
            | Expression::AnalogOperator(_)
            | Expression::NoiseSource(_) => false,
        }
    }

    fn array_element_contains_call(element: &ArrayLiteralElement, expected: &str) -> bool {
        match element {
            ArrayLiteralElement::Value(expression) => {
                Self::expr_contains_call(expression, expected)
            }
            ArrayLiteralElement::Replication(replication) => {
                Self::expr_contains_call(&replication.count, expected)
                    || replication
                        .elements
                        .iter()
                        .any(|element| Self::array_element_contains_call(element, expected))
            }
        }
    }

    fn expr_contains_number_close(expr: &Expression, expected: f64) -> bool {
        match expr {
            Expression::Number(number) => {
                let tolerance = expected.abs().max(1.0) * 1.0e-12;
                (number.value - expected).abs() <= tolerance
            }
            Expression::Binary(binary) => {
                Self::expr_contains_number_close(&binary.left, expected)
                    || Self::expr_contains_number_close(&binary.right, expected)
            }
            Expression::Unary(unary) => Self::expr_contains_number_close(&unary.operand, expected),
            Expression::Conditional(conditional) => {
                Self::expr_contains_number_close(&conditional.condition, expected)
                    || Self::expr_contains_number_close(&conditional.then_expr, expected)
                    || Self::expr_contains_number_close(&conditional.else_expr, expected)
            }
            Expression::Call(call) => call
                .args
                .iter()
                .any(|arg| Self::expr_contains_number_close(arg, expected)),
            Expression::SystemFunction(function) => function
                .args
                .iter()
                .any(|arg| Self::expr_contains_number_close(arg, expected)),
            Expression::ArrayAccess(access) => {
                Self::expr_contains_number_close(&access.index, expected)
            }
            Expression::ArrayLiteral(array) => array
                .elements
                .iter()
                .any(|element| Self::array_element_contains_number_close(element, expected)),
            Expression::StringLit(_)
            | Expression::NullArgument(_)
            | Expression::Identifier(_)
            | Expression::BranchAccess(_)
            | Expression::AnalogOperator(_)
            | Expression::NoiseSource(_) => false,
        }
    }

    fn array_element_contains_number_close(element: &ArrayLiteralElement, expected: f64) -> bool {
        match element {
            ArrayLiteralElement::Value(expression) => {
                Self::expr_contains_number_close(expression, expected)
            }
            ArrayLiteralElement::Replication(replication) => {
                Self::expr_contains_number_close(&replication.count, expected)
                    || replication
                        .elements
                        .iter()
                        .any(|element| Self::array_element_contains_number_close(element, expected))
            }
        }
    }

    /// Analyze a statement, lowering control flow into guarded dataflow.
    ///
    /// Assignments and contributions inside conditionals become conditional
    /// expressions (`guard ? value : previous`), so the recorded flat lists
    /// preserve branch semantics exactly. Loops whose bounds do not fold to
    /// compile-time constants lower to runtime loop statements.
    fn block_local_canonical_name_is_taken(&self, module: &AnalyzedModule, name: &SmolStr) -> bool {
        module
            .variables
            .iter()
            .any(|variable| variable.name == *name)
            || module
                .parameters
                .iter()
                .any(|parameter| parameter.name == *name)
            || module
                .param_aliases
                .iter()
                .any(|alias| alias.alias == *name)
            || module.ports.iter().any(|port| port.name == *name)
            || module.internal_nodes.iter().any(|node| node.name == *name)
            || module.branches.iter().any(|branch| branch.name == *name)
            || module.arrays.contains_key(name)
            || self.arrays.contains_key(name)
            || self.symbols.lookup(name).is_some()
    }

    fn analyze_statement(
        &mut self,
        stmt: &AnalogStatement,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        match stmt {
            AnalogStatement::Contribution(contrib) => {
                self.analyze_contribution(contrib, module, sink)?;
            }
            AnalogStatement::Assignment(assign) => {
                self.analyze_assignment(assign, module, sink)?;
            }
            AnalogStatement::Block(block) => {
                self.symbols.enter_scope();
                self.subst_stack.push(HashMap::new());

                // Hoist block-local variables to module scope under unique
                // names; expressions are rewritten through the subst frame.
                for var_decl in &block.variables {
                    let value_type = match var_decl.var_type {
                        VarType::Real => ValueType::Real,
                        VarType::Integer => ValueType::Integer,
                        VarType::String => ValueType::String,
                    };
                    for item in &var_decl.items {
                        self.local_counter += 1;
                        let hoisted = if self
                            .block_local_canonical_name_is_taken(module, &item.name)
                        {
                            loop {
                                let candidate: SmolStr =
                                    format!("{}__blk{}", item.name, self.local_counter).into();
                                if !self.block_local_canonical_name_is_taken(module, &candidate) {
                                    break candidate;
                                }
                                self.local_counter += 1;
                            }
                        } else {
                            item.name.clone()
                        };

                        if !item.dimensions.is_empty() {
                            if let Some(layout) = self.register_array_variable(
                                item,
                                var_decl.var_type,
                                &hoisted,
                                module,
                            ) {
                                module.arrays.insert(hoisted.clone(), layout.clone());
                                self.arrays.insert(hoisted.clone(), layout);
                                self.define_symbol(Symbol {
                                    name: hoisted.clone(),
                                    kind: SymbolKind::Variable,
                                    value_type,
                                    span: item.span,
                                    attrs: Default::default(),
                                })?;
                                self.subst_stack.last_mut().unwrap().insert(
                                    item.name.clone(),
                                    Expression::Identifier(Identifier {
                                        name: hoisted.clone(),
                                        span: item.span,
                                    }),
                                );
                                if item.init.is_some() {
                                    self.record_error_at(
                                        SemanticErrorKind::UnsupportedFeature(format!(
                                            "initializer on block-local array '{}'",
                                            item.name
                                        )),
                                        item.span,
                                    );
                                }
                            }
                            continue;
                        }

                        module.variables.push(AnalyzedVariable {
                            name: hoisted.clone(),
                            var_type: var_decl.var_type,
                            value_type,
                            is_state: false,
                        });
                        self.define_symbol(Symbol {
                            name: hoisted.clone(),
                            kind: SymbolKind::Variable,
                            value_type,
                            span: item.span,
                            attrs: Default::default(),
                        })?;
                        self.subst_stack.last_mut().unwrap().insert(
                            item.name.clone(),
                            Expression::Identifier(Identifier {
                                name: hoisted.clone(),
                                span: item.span,
                            }),
                        );

                        if let Some(init) = &item.init {
                            let expression =
                                self.lower_expression_with_side_effects(init, module, sink)?;
                            let expression =
                                self.apply_guard(expression, Self::number_expr(0.0, item.span));
                            let expr_type = self.infer_type(&expression)?;
                            let var_index = module
                                .variables
                                .iter()
                                .position(|v| v.name == hoisted)
                                .expect("just registered");
                            sink.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
                                target: hoisted.clone(),
                                var_index,
                                index: None,
                                expression,
                                expr_type,
                                span: item.span,
                                unfiltered_initial_step_guard: None,
                            }));
                        }
                    }
                }

                for s in &block.statements {
                    self.analyze_statement(s, module, sink)?;
                }

                self.subst_stack.pop();
                self.symbols.exit_scope();
            }
            AnalogStatement::Conditional(cond) => {
                let condition =
                    self.lower_expression_with_side_effects(&cond.condition, module, sink)?;
                let dynamic_condition = !self.expression_is_simulation_invariant(&condition);
                let cond_type = self.infer_type(&condition)?;
                if !cond_type.is_condition() {
                    self.record_error_at(
                        SemanticErrorKind::InvalidCondition {
                            found: cond_type.to_string(),
                        },
                        cond.span,
                    );
                }

                // Snapshot the condition into a temporary BEFORE either
                // branch runs: the then-branch may assign variables the
                // condition reads, and a re-evaluated else-guard would then
                // see the mutated state and fire as well.
                //
                // The structured form needs no snapshot and must not use one.
                // Its condition is evaluated once, at the branch, and every read
                // inside an arm resolves against the definitions reaching that
                // arm — so the hazard the snapshot exists for cannot arise, and
                // the snapshot variable's own assignment only ever went into the
                // flat list.
                let unsnapshotted = condition.clone();
                let condition = self.snapshot_guard(condition, cond.span, module, sink)?;

                self.guard_stack.push(condition.clone());
                self.open_region();
                if dynamic_condition {
                    self.dynamic_analog_operator_guard_depth += 1;
                }
                let then_result = self.analyze_statement(&cond.then_branch, module, sink);
                if dynamic_condition {
                    self.dynamic_analog_operator_guard_depth -= 1;
                }
                then_result?;
                let then_body = self.close_region();
                self.guard_stack.pop();

                let mut else_body = Vec::new();
                if let Some(else_branch) = &cond.else_branch {
                    self.guard_stack.push(Self::not_expr(condition.clone()));
                    self.open_region();
                    if dynamic_condition {
                        self.dynamic_analog_operator_guard_depth += 1;
                    }
                    let else_result = self.analyze_statement(else_branch, module, sink);
                    if dynamic_condition {
                        self.dynamic_analog_operator_guard_depth -= 1;
                    }
                    else_result?;
                    else_body = self.close_region();
                    self.guard_stack.pop();
                }

                self.record_region(AnalyzedRegion::Conditional {
                    condition: unsnapshotted,
                    then_body,
                    else_body,
                    span: cond.span,
                });
            }
            AnalogStatement::Case(case_stmt) => {
                // The selector and ALL match comparisons are evaluated
                // before any arm executes (LRM case semantics); snapshot
                // them so arm bodies cannot perturb later guards.
                let selector =
                    self.lower_expression_with_side_effects(&case_stmt.expr, module, sink)?;
                let mut dynamic_case = !self.expression_is_simulation_invariant(&selector);
                // The structured form keeps the comparisons as written; a case
                // arm's condition is evaluated once at its branch, so the
                // snapshot the flat form needs would only be a variable the
                // region body never assigns.
                let unsnapshotted_selector = selector.clone();
                let selector = self.snapshot_guard(selector, case_stmt.span, module, sink)?;

                let mut item_guards: Vec<Option<Expression>> = Vec::new();
                let mut unsnapshotted_guards: Vec<Option<Expression>> = Vec::new();
                for item in &case_stmt.items {
                    let mut item_match: Option<Expression> = None;
                    let mut unsnapshotted_match: Option<Expression> = None;
                    for m in &item.matches {
                        let m_lowered = self.lower_expression_with_side_effects(m, module, sink)?;
                        dynamic_case |= !self.expression_is_simulation_invariant(&m_lowered);
                        let eq =
                            Self::binary_expr(BinaryOp::Eq, selector.clone(), m_lowered.clone());
                        item_match = Some(match item_match {
                            Some(acc) => Self::binary_expr(BinaryOp::Or, acc, eq),
                            None => eq,
                        });
                        let raw_eq = Self::binary_expr(
                            BinaryOp::Eq,
                            unsnapshotted_selector.clone(),
                            m_lowered,
                        );
                        unsnapshotted_match = Some(match unsnapshotted_match {
                            Some(acc) => Self::binary_expr(BinaryOp::Or, acc, raw_eq),
                            None => raw_eq,
                        });
                    }
                    let snapshotted = match item_match {
                        Some(expr) => {
                            Some(self.snapshot_guard(expr, case_stmt.span, module, sink)?)
                        }
                        None => None,
                    };
                    item_guards.push(snapshotted);
                    unsnapshotted_guards.push(unsnapshotted_match);
                }

                // OR of all guards matched so far (case items are priority
                // ordered: the first matching item wins)
                let mut prior_match: Option<Expression> = None;
                // Structured arms, each holding the item's own match condition
                // rather than the flat form's `match AND NOT prior`. Priority
                // becomes the nesting below, which is what lets the CFG emit
                // one branch per arm instead of an accumulating conjunction.
                let mut arms: Vec<(Expression, Vec<AnalyzedRegion>)> = Vec::new();

                for ((item, item_match), unsnapshotted_match) in case_stmt
                    .items
                    .iter()
                    .zip(item_guards)
                    .zip(unsnapshotted_guards)
                {
                    let (Some(item_match), Some(unsnapshotted_match)) =
                        (item_match, unsnapshotted_match)
                    else {
                        continue;
                    };

                    let guard = match &prior_match {
                        Some(prior) => Self::binary_expr(
                            BinaryOp::And,
                            item_match.clone(),
                            Self::not_expr(prior.clone()),
                        ),
                        None => item_match.clone(),
                    };

                    self.guard_stack.push(guard);
                    self.open_region();
                    if dynamic_case {
                        self.dynamic_analog_operator_guard_depth += 1;
                    }
                    let arm_result = self.analyze_statement(&item.statement, module, sink);
                    if dynamic_case {
                        self.dynamic_analog_operator_guard_depth -= 1;
                    }
                    arm_result?;
                    let body = self.close_region();
                    self.guard_stack.pop();
                    arms.push((unsnapshotted_match, body));

                    prior_match = Some(match prior_match {
                        Some(prior) => Self::binary_expr(BinaryOp::Or, prior, item_match),
                        None => item_match,
                    });
                }

                let mut chain: Vec<AnalyzedRegion> = Vec::new();
                if let Some(default) = &case_stmt.default {
                    match prior_match {
                        Some(prior) => {
                            self.guard_stack.push(Self::not_expr(prior));
                            self.open_region();
                            if dynamic_case {
                                self.dynamic_analog_operator_guard_depth += 1;
                            }
                            let default_result = self.analyze_statement(default, module, sink);
                            if dynamic_case {
                                self.dynamic_analog_operator_guard_depth -= 1;
                            }
                            default_result?;
                            chain = self.close_region();
                            self.guard_stack.pop();
                        }
                        None => {
                            self.open_region();
                            if dynamic_case {
                                self.dynamic_analog_operator_guard_depth += 1;
                            }
                            let default_result = self.analyze_statement(default, module, sink);
                            if dynamic_case {
                                self.dynamic_analog_operator_guard_depth -= 1;
                            }
                            default_result?;
                            chain = self.close_region();
                        }
                    }
                }

                // Fold innermost-first so arm order becomes else-nesting depth.
                for (condition, then_body) in arms.into_iter().rev() {
                    chain = vec![AnalyzedRegion::Conditional {
                        condition,
                        then_body,
                        else_body: chain,
                        span: case_stmt.span,
                    }];
                }
                for region in chain {
                    self.record_region(region);
                }
            }
            AnalogStatement::For(for_stmt) => {
                self.dynamic_analog_operator_guard_depth += 1;
                let result = self.analyze_for(for_stmt, module, sink);
                self.dynamic_analog_operator_guard_depth -= 1;
                result?;
            }
            AnalogStatement::Repeat(repeat) => {
                self.dynamic_analog_operator_guard_depth += 1;
                let count_expr =
                    self.lower_expression_with_side_effects(&repeat.count, module, sink);
                let result = count_expr.and_then(|count_expr| {
                    match self.eval_const_invariant(&count_expr) {
                        Some(count) if (count as usize) <= Self::MAX_UNROLL_ITERATIONS => {
                            for _ in 0..(count as usize) {
                                self.analyze_statement(&repeat.body, module, sink)?;
                            }
                            Ok(())
                        }
                        Some(count) => Err(CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::InvalidAnalogOperator(format!(
                                "repeat count {count} exceeds the unroll limit"
                            )),
                            repeat.span,
                        ))),
                        // Runtime-dependent count: lower to a runtime loop with
                        // a synthesized counter
                        None => self.lower_runtime_repeat(repeat, count_expr, module, sink),
                    }
                });
                self.dynamic_analog_operator_guard_depth -= 1;
                result?;
            }
            AnalogStatement::While(while_stmt) => {
                self.dynamic_analog_operator_guard_depth += 1;
                let condition =
                    self.lower_expression_with_side_effects(&while_stmt.condition, module, sink);
                let result =
                    condition.and_then(|condition| match self.eval_const_invariant(&condition) {
                        Some(0.0) => Ok(()), // statically dead loop
                        Some(_) => Err(CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::InvalidAnalogOperator(
                                "while loop condition is constant-true (infinite loop)".into(),
                            ),
                            while_stmt.span,
                        ))),
                        None => {
                            // Runtime condition: lower to a runtime loop
                            let cond_type = self.infer_type(&condition)?;
                            if !cond_type.is_condition() {
                                self.record_error_at(
                                    SemanticErrorKind::InvalidCondition {
                                        found: cond_type.to_string(),
                                    },
                                    while_stmt.span,
                                );
                            }
                            // The structured loop sits inside whatever conditional
                            // region encloses it, so folding the guard in again
                            // would only add a read of a snapshot variable that the
                            // region body never assigns.
                            let unguarded = condition.clone();
                            let condition = self.fold_guard_into_condition(condition);
                            let (body, regions) =
                                self.analyze_loop_body(&while_stmt.body, None, module)?;
                            self.record_region(AnalyzedRegion::Loop {
                                condition: unguarded,
                                body: regions,
                                span: while_stmt.span,
                            });
                            sink.push(AnalyzedStatement::Loop(AnalyzedLoop {
                                condition,
                                body,
                                span: while_stmt.span,
                            }));
                            Ok(())
                        }
                    });
                self.dynamic_analog_operator_guard_depth -= 1;
                result?;
            }
            AnalogStatement::EventControl(event_ctrl) => {
                let unfiltered_initial_step = matches!(
                    &event_ctrl.event,
                    EventExpr::InitialStep { analyses, .. } if analyses.is_empty()
                );
                let EventLowering::Guard(guard) =
                    self.event_guard(&event_ctrl.event, module, sink)?;
                // Snapshot: the body must not perturb its own guard.
                let guard = self.snapshot_guard(guard, event_ctrl.span, module, sink)?;
                let initial_guard_name = match &guard {
                    Expression::Identifier(identifier) if unfiltered_initial_step => {
                        Some(identifier.name.clone())
                    }
                    _ => None,
                };
                self.guard_stack.push(guard);
                if let Some(name) = initial_guard_name {
                    self.unfiltered_initial_step_guards.push(name);
                }
                self.dynamic_analog_operator_guard_depth += 1;
                let body_result = self.analyze_statement(&event_ctrl.statement, module, sink);
                self.dynamic_analog_operator_guard_depth -= 1;
                body_result?;
                if unfiltered_initial_step {
                    self.unfiltered_initial_step_guards.pop();
                }
                self.guard_stack.pop();
            }
            AnalogStatement::IndirectContribution(stmt) => {
                self.analyze_indirect_contribution(stmt, module, sink)?;
            }
            // $bound_step and $discontinuity steer the transient stepper
            // through hidden per-evaluation variables; other system tasks
            // ($strobe, $display, ...) have no effect on the device
            // equations
            AnalogStatement::Call(call) => match call.name.as_str() {
                "$bound_step" => {
                    self.validate_system_task_arity(call, 1, Some(1))?;
                    self.analyze_bound_step(call, module, sink)?;
                }
                "$discontinuity" => {
                    self.validate_system_task_arity(call, 0, Some(1))?;
                    self.analyze_discontinuity(call, module, sink)?;
                }
                name if Self::is_no_effect_system_task(name) => {
                    self.warn_no_effect_system_task(call);
                }
                _ => return Err(Self::unknown_system_task_error(call)),
            },
            AnalogStatement::Disable(_) | AnalogStatement::Null(_) => {}
        }
        Ok(())
    }

    const MAX_STATIC_UNROLL_ITERATIONS: usize = 32;
    const MAX_UNROLL_ITERATIONS: usize = 65536;

    /// Analyze a for loop: statically unroll when the bounds fold to
    /// compile-time constants, otherwise lower to a runtime loop
    fn analyze_for(
        &mut self,
        for_stmt: &ForStmt,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        let loop_var = self.resolve_substituted_name(&for_stmt.var);
        if self.symbols.lookup(&loop_var).is_none() {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UndeclaredSymbol {
                    name: loop_var.clone(),
                },
                for_stmt.span,
            )));
        }

        if *for_stmt.update.target_name() != for_stmt.var {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(
                    "for-loop update must assign the loop variable".into(),
                ),
                for_stmt.span,
            )));
        }

        // Probe whether init, condition, and update fold to constants;
        // only then is static unrolling sound.
        let init = self.lower_expression_without_side_effects(&for_stmt.init, "for-loop init")?;
        let init_value = self.eval_const_invariant(&init);
        let static_unrollable = if let Some(value) = init_value {
            self.subst_stack.push(HashMap::from([(
                for_stmt.var.clone(),
                Self::number_expr(value, for_stmt.span),
            )]));
            let cond_probe = self
                .lower_expression_without_side_effects(&for_stmt.condition, "for-loop condition")
                .ok()
                .and_then(|c| self.eval_const_invariant(&c));
            let update_probe = self
                .lower_expression_without_side_effects(&for_stmt.update.value, "for-loop update")
                .ok()
                .and_then(|u| self.eval_const_invariant(&u));
            self.subst_stack.pop();
            cond_probe.is_some() && update_probe.is_some()
        } else {
            false
        };

        if static_unrollable {
            let init_value = init_value.expect("checked");
            let iteration_count = self.static_for_iteration_count(for_stmt, init_value)?;
            if iteration_count <= Self::MAX_STATIC_UNROLL_ITERATIONS
                || Self::statement_contains_contribution(&for_stmt.body)
            {
                self.unroll_for(for_stmt, init_value, module, sink)
            } else {
                self.lower_runtime_for(for_stmt, module, sink)
            }
        } else {
            self.lower_runtime_for(for_stmt, module, sink)
        }
    }

    fn static_for_iteration_count(
        &mut self,
        for_stmt: &ForStmt,
        init_value: f64,
    ) -> CompileResult<usize> {
        let mut value = init_value;
        let mut iterations = 0usize;
        loop {
            self.subst_stack.push(HashMap::from([(
                for_stmt.var.clone(),
                Self::number_expr(value, for_stmt.span),
            )]));

            let condition = match self
                .lower_expression_without_side_effects(&for_stmt.condition, "for-loop condition")
                .and_then(|expr| {
                    self.eval_const_invariant(&expr).ok_or_else(|| {
                        CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::InvalidAnalogOperator(
                                "for-loop condition stopped folding during unroll sizing".into(),
                            ),
                            for_stmt.span,
                        ))
                    })
                }) {
                Ok(condition) => condition,
                Err(error) => {
                    self.subst_stack.pop();
                    return Err(error);
                }
            };
            if condition == 0.0 {
                self.subst_stack.pop();
                return Ok(iterations);
            }

            let update = match self
                .lower_expression_without_side_effects(&for_stmt.update.value, "for-loop update")
                .and_then(|expr| {
                    self.eval_const_invariant(&expr).ok_or_else(|| {
                        CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::InvalidAnalogOperator(
                                "for-loop update stopped folding during unroll sizing".into(),
                            ),
                            for_stmt.span,
                        ))
                    })
                }) {
                Ok(update) => update,
                Err(error) => {
                    self.subst_stack.pop();
                    return Err(error);
                }
            };
            self.subst_stack.pop();

            iterations += 1;
            if iterations > Self::MAX_UNROLL_ITERATIONS {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidAnalogOperator(format!(
                        "for-loop exceeds the unroll limit of {} iterations",
                        Self::MAX_UNROLL_ITERATIONS
                    )),
                    for_stmt.span,
                )));
            }
            value = update;
        }
    }

    fn statement_contains_contribution(stmt: &AnalogStatement) -> bool {
        match stmt {
            AnalogStatement::Contribution(_) | AnalogStatement::IndirectContribution(_) => true,
            AnalogStatement::Block(block) => block
                .statements
                .iter()
                .any(Self::statement_contains_contribution),
            AnalogStatement::Conditional(cond) => {
                Self::statement_contains_contribution(&cond.then_branch)
                    || cond
                        .else_branch
                        .as_deref()
                        .is_some_and(Self::statement_contains_contribution)
            }
            AnalogStatement::Case(case_stmt) => {
                case_stmt
                    .items
                    .iter()
                    .any(|item| Self::statement_contains_contribution(&item.statement))
                    || case_stmt
                        .default
                        .as_deref()
                        .is_some_and(Self::statement_contains_contribution)
            }
            AnalogStatement::For(for_stmt) => Self::statement_contains_contribution(&for_stmt.body),
            AnalogStatement::Repeat(repeat) => Self::statement_contains_contribution(&repeat.body),
            AnalogStatement::While(while_stmt) => {
                Self::statement_contains_contribution(&while_stmt.body)
            }
            AnalogStatement::EventControl(event_ctrl) => {
                Self::statement_contains_contribution(&event_ctrl.statement)
            }
            AnalogStatement::Assignment(_)
            | AnalogStatement::Call(_)
            | AnalogStatement::Disable(_)
            | AnalogStatement::Null(_) => false,
        }
    }

    /// Statically unroll a for loop with compile-time-constant bounds
    fn unroll_for(
        &mut self,
        for_stmt: &ForStmt,
        init_value: f64,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        let mut value = init_value;
        let mut iterations = 0usize;
        loop {
            // Bind the loop variable to its current constant value
            self.subst_stack.push(HashMap::from([(
                for_stmt.var.clone(),
                Self::number_expr(value, for_stmt.span),
            )]));

            let condition = self
                .lower_expression_without_side_effects(&for_stmt.condition, "for-loop condition")?;
            let Some(cond_value) = self.eval_const_invariant(&condition) else {
                self.subst_stack.pop();
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidAnalogOperator(
                        "for-loop condition stopped folding during unrolling".into(),
                    ),
                    for_stmt.span,
                )));
            };
            if cond_value == 0.0 {
                self.subst_stack.pop();
                break;
            }

            self.analyze_statement(&for_stmt.body, module, sink)?;

            let update = self
                .lower_expression_without_side_effects(&for_stmt.update.value, "for-loop update")?;
            let Some(next_value) = self.eval_const_invariant(&update) else {
                self.subst_stack.pop();
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidAnalogOperator(
                        "for-loop update stopped folding during unrolling".into(),
                    ),
                    for_stmt.span,
                )));
            };
            self.subst_stack.pop();

            value = next_value;
            iterations += 1;
            if iterations > Self::MAX_UNROLL_ITERATIONS {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidAnalogOperator(format!(
                        "for-loop exceeds the unroll limit of {} iterations",
                        Self::MAX_UNROLL_ITERATIONS
                    )),
                    for_stmt.span,
                )));
            }
        }

        Ok(())
    }

    /// Materialize a guard expression into a synthesized variable assigned
    /// once at this point in the statement stream.
    ///
    /// Guards must capture the state at evaluation time: branch bodies may
    /// assign variables their own guard reads, and re-evaluating the raw
    /// expression inside each guarded assignment would observe the
    /// mutation (e.g. `if (x == UNSET) x = a; else x = x + b;` must never
    /// run both arms). Trivial guards (literals, identifiers) pass through.
    fn snapshot_guard(
        &mut self,
        condition: Expression,
        span: Span,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<Expression> {
        // Identifiers and literals are stable by construction
        if matches!(condition, Expression::Identifier(_) | Expression::Number(_)) {
            return Ok(condition);
        }

        self.local_counter += 1;
        let name: SmolStr = format!("__guard{}", self.local_counter).into();

        let var_index = module.variables.len();
        module.variables.push(AnalyzedVariable {
            name: name.clone(),
            var_type: VarType::Real,
            value_type: ValueType::Real,
            is_state: false,
        });
        self.define_symbol(Symbol {
            name: name.clone(),
            kind: SymbolKind::Variable,
            value_type: ValueType::Real,
            span,
            attrs: Default::default(),
        })?;

        // The snapshot assignment itself is unconditional: guard
        // expressions are pure, and enclosing guards already gate every
        // consumer of this variable.
        sink.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
            target: name.clone(),
            var_index,
            index: None,
            expression: condition,
            expr_type: ValueType::Real,
            span,
            unfiltered_initial_step_guard: None,
        }));

        Ok(Expression::Identifier(Identifier { name, span }))
    }

    /// AND the enclosing guard into a runtime loop condition so a guarded
    /// loop runs zero iterations when its guard is inactive
    fn fold_guard_into_condition(&self, condition: Expression) -> Expression {
        match self.current_guard() {
            Some(guard) => Self::binary_expr(BinaryOp::And, guard, condition),
            None => condition,
        }
    }

    /// Analyze loop-body statements into a fresh sink, tracking the
    /// runtime-loop nesting depth (contributions inside are rejected).
    /// An optional trailing statement (the for-loop update) is analyzed
    /// after the body.
    /// Analyze a runtime loop body, returning both forms of it.
    ///
    /// The single funnel for every runtime-bounded loop, which is why the
    /// structured capture lives here rather than at the three record sites.
    fn analyze_loop_body(
        &mut self,
        body: &AnalogStatement,
        trailing: Option<&AnalogStatement>,
        module: &mut AnalyzedModule,
    ) -> CompileResult<(Vec<AnalyzedStatement>, Vec<AnalyzedRegion>)> {
        let mut statements = Vec::new();
        self.runtime_loop_depth += 1;
        self.open_region();
        let result = self
            .analyze_statement(body, module, &mut statements)
            .and_then(|()| match trailing {
                Some(stmt) => self.analyze_statement(stmt, module, &mut statements),
                None => Ok(()),
            });
        let regions = self.close_region();
        self.runtime_loop_depth -= 1;
        result?;
        Ok((statements, regions))
    }

    /// Lower a for loop with runtime-dependent bounds (e.g. iterating to a
    /// parameter like BSIM4's nf finger count) into a runtime loop
    fn lower_runtime_for(
        &mut self,
        for_stmt: &ForStmt,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        // Loop variable initialization through the normal guarded path
        let init_stmt = AssignmentStmt {
            target: LValue::Variable {
                name: for_stmt.var.clone(),
                span: for_stmt.span,
            },
            value: for_stmt.init.clone(),
            span: for_stmt.span,
        };
        self.analyze_assignment(&init_stmt, module, sink)?;

        // Condition re-evaluated each iteration, with the enclosing guard
        // folded in
        let condition =
            self.lower_expression_without_side_effects(&for_stmt.condition, "for-loop condition")?;
        let cond_type = self.infer_type(&condition)?;
        if !cond_type.is_condition() {
            self.record_error_at(
                SemanticErrorKind::InvalidCondition {
                    found: cond_type.to_string(),
                },
                for_stmt.span,
            );
        }
        let unguarded = condition.clone();
        let condition = self.fold_guard_into_condition(condition);

        // Body, then the update assignment, inside the loop sink
        let update_stmt = AnalogStatement::Assignment((*for_stmt.update).clone());
        let (body, regions) = self.analyze_loop_body(&for_stmt.body, Some(&update_stmt), module)?;

        self.record_region(AnalyzedRegion::Loop {
            condition: unguarded,
            body: regions,
            span: for_stmt.span,
        });
        sink.push(AnalyzedStatement::Loop(AnalyzedLoop {
            condition,
            body,
            span: for_stmt.span,
        }));
        Ok(())
    }

    /// Lower a repeat loop with a runtime-dependent count into a runtime
    /// loop over a synthesized counter
    fn lower_runtime_repeat(
        &mut self,
        repeat: &RepeatStmt,
        count_expr: Expression,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        let span = repeat.span;

        // Synthesize counter and count-snapshot variables (the LRM
        // evaluates the repeat count once, before iterating)
        self.local_counter += 1;
        let idx_name: SmolStr = format!("__repeat_i{}", self.local_counter).into();
        let cnt_name: SmolStr = format!("__repeat_n{}", self.local_counter).into();
        let mut register = |this: &mut Self, name: &SmolStr| -> CompileResult<usize> {
            let var_index = module.variables.len();
            module.variables.push(AnalyzedVariable {
                name: name.clone(),
                var_type: VarType::Real,
                value_type: ValueType::Real,
                is_state: false,
            });
            this.define_symbol(Symbol {
                name: name.clone(),
                kind: SymbolKind::Variable,
                value_type: ValueType::Real,
                span,
                attrs: Default::default(),
            })?;
            Ok(var_index)
        };
        let idx_index = register(self, &idx_name)?;
        let cnt_index = register(self, &cnt_name)?;

        let ident = |name: &SmolStr| {
            Expression::Identifier(Identifier {
                name: name.clone(),
                span,
            })
        };

        // cnt = <count>; idx = 0;
        sink.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
            target: cnt_name.clone(),
            var_index: cnt_index,
            index: None,
            expression: count_expr,
            expr_type: ValueType::Real,
            span,
            unfiltered_initial_step_guard: None,
        }));
        sink.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
            target: idx_name.clone(),
            var_index: idx_index,
            index: None,
            expression: Self::number_expr(0.0, span),
            expr_type: ValueType::Real,
            span,
            unfiltered_initial_step_guard: None,
        }));

        // while (guard && idx < cnt) { body; idx = idx + 1; }
        let unguarded = Self::binary_expr(BinaryOp::Lt, ident(&idx_name), ident(&cnt_name));
        let condition = self.fold_guard_into_condition(unguarded.clone());

        let (mut body, mut regions) = self.analyze_loop_body(&repeat.body, None, module)?;
        // The synthesized counter bump closes the loop in both forms; a
        // structured body without it would describe a loop that never advances.
        let increment = AnalyzedAssignment {
            target: idx_name.clone(),
            var_index: idx_index,
            index: None,
            expression: Self::binary_expr(
                BinaryOp::Add,
                ident(&idx_name),
                Self::number_expr(1.0, span),
            ),
            expr_type: ValueType::Real,
            span,
            unfiltered_initial_step_guard: None,
        };
        regions.push(AnalyzedRegion::Assignment(increment.clone()));
        body.push(AnalyzedStatement::Assignment(increment));

        self.record_region(AnalyzedRegion::Loop {
            condition: unguarded,
            body: regions,
            span,
        });
        sink.push(AnalyzedStatement::Loop(AnalyzedLoop {
            condition,
            body,
            span,
        }));
        Ok(())
    }

    /// Lower an event expression into a runtime guard
    fn event_guard(
        &mut self,
        event: &EventExpr,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<EventLowering> {
        Ok(match event {
            EventExpr::InitialStep { analyses, span } => {
                let phase = Expression::Call(CallExpr {
                    name: "analysis".into(),
                    args: vec![Expression::StringLit(StringLit {
                        value: "__rspice_initial_step".into(),
                        span: *span,
                    })],
                    span: *span,
                });
                EventLowering::Guard(Self::filter_step_event(phase, analyses, *span))
            }
            EventExpr::FinalStep { analyses, span } => {
                let phase = Expression::Call(CallExpr {
                    name: "analysis".into(),
                    args: vec![Expression::StringLit(StringLit {
                        value: "__rspice_final_step".into(),
                        span: *span,
                    })],
                    span: *span,
                });
                EventLowering::Guard(Self::filter_step_event(phase, analyses, *span))
            }
            EventExpr::Cross {
                signal,
                direction,
                time_tol,
                expr_tol,
                enable,
                span,
            } => {
                let signal = self.lower_expression_with_side_effects(signal, module, sink)?;
                let dir_value = match direction {
                    Some(CrossDirection::Rising) => 1.0,
                    Some(CrossDirection::Falling) => -1.0,
                    Some(CrossDirection::Both) | None => 0.0,
                };
                let mut args = vec![signal, Self::number_expr(dir_value, *span)];
                if let Some(time_tol) = time_tol {
                    args.push(self.lower_expression_with_side_effects(time_tol, module, sink)?);
                } else if expr_tol.is_some() || enable.is_some() {
                    args.push(Self::number_expr(0.0, *span));
                }
                if let Some(expr_tol) = expr_tol {
                    args.push(self.lower_expression_with_side_effects(expr_tol, module, sink)?);
                } else if enable.is_some() {
                    args.push(Self::number_expr(0.0, *span));
                }
                if let Some(enable) = enable {
                    args.push(self.lower_expression_with_side_effects(enable, module, sink)?);
                }
                EventLowering::Guard(Expression::Call(CallExpr {
                    name: "cross".into(),
                    args,
                    span: *span,
                }))
            }
            EventExpr::Posedge { signal, span } => {
                let signal = self.lower_expression_with_side_effects(signal, module, sink)?;
                EventLowering::Guard(Expression::Call(CallExpr {
                    name: "cross".into(),
                    args: vec![signal, Self::number_expr(1.0, *span)],
                    span: *span,
                }))
            }
            EventExpr::Negedge { signal, span } => {
                let signal = self.lower_expression_with_side_effects(signal, module, sink)?;
                EventLowering::Guard(Expression::Call(CallExpr {
                    name: "cross".into(),
                    args: vec![signal, Self::number_expr(-1.0, *span)],
                    span: *span,
                }))
            }
            EventExpr::Above {
                signal,
                time_tol,
                expr_tol,
                enable,
                span,
            } => {
                let signal = self.lower_expression_with_side_effects(signal, module, sink)?;
                let mut args = vec![signal];
                if let Some(time_tol) = time_tol {
                    args.push(self.lower_expression_with_side_effects(time_tol, module, sink)?);
                } else if expr_tol.is_some() || enable.is_some() {
                    args.push(Self::number_expr(0.0, *span));
                }
                if let Some(expr_tol) = expr_tol {
                    args.push(self.lower_expression_with_side_effects(expr_tol, module, sink)?);
                } else if enable.is_some() {
                    args.push(Self::number_expr(0.0, *span));
                }
                if let Some(enable) = enable {
                    args.push(self.lower_expression_with_side_effects(enable, module, sink)?);
                }
                EventLowering::Guard(Expression::Call(CallExpr {
                    name: "above".into(),
                    args,
                    span: *span,
                }))
            }
            EventExpr::Timer {
                start,
                period,
                time_tol,
                enable,
                span,
            } => {
                let mut args = vec![self.lower_expression_with_side_effects(start, module, sink)?];
                if let Some(period) = period {
                    args.push(self.lower_expression_with_side_effects(period, module, sink)?);
                } else if time_tol.is_some() || enable.is_some() {
                    args.push(Self::number_expr(0.0, *span));
                }
                if let Some(time_tol) = time_tol {
                    args.push(self.lower_expression_with_side_effects(time_tol, module, sink)?);
                } else if enable.is_some() {
                    args.push(Self::number_expr(0.0, *span));
                }
                if let Some(enable) = enable {
                    args.push(self.lower_expression_with_side_effects(enable, module, sink)?);
                }
                EventLowering::Guard(Expression::Call(CallExpr {
                    name: "timer".into(),
                    args,
                    span: *span,
                }))
            }
            EventExpr::Or { left, right, .. } => {
                let left = self.event_guard(left, module, sink)?;
                let right = self.event_guard(right, module, sink)?;
                match (left, right) {
                    (EventLowering::Guard(l), EventLowering::Guard(r)) => {
                        EventLowering::Guard(Self::binary_expr(BinaryOp::Or, l, r))
                    }
                }
            }
        })
    }

    fn filter_step_event(phase: Expression, analyses: &[StringLit], span: Span) -> Expression {
        if analyses.is_empty() {
            return phase;
        }

        let analysis_filter = Expression::Call(CallExpr {
            name: "analysis".into(),
            args: analyses
                .iter()
                .cloned()
                .map(Expression::StringLit)
                .collect(),
            span,
        });
        Self::binary_expr(BinaryOp::And, phase, analysis_filter)
    }

    fn analyze_contribution(
        &mut self,
        contrib: &ContributionStmt,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        // Contributions accumulate into fixed stamp programs; a contribution
        // executed a runtime-dependent number of times is not representable
        if self.runtime_loop_depth > 0 {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidContribution(
                    "contributions inside loops require compile-time-constant bounds".into(),
                ),
                contrib.span,
            )));
        }

        let (branch_name, is_current, declared_branch) =
            self.resolve_contribution_target(&contrib.target, module, contrib.span)?;
        self.validate_branch_access_compatible(&contrib.target, contrib.span)?;

        self.validate_direct_zi_contribution(&contrib.value, contrib.span)?;

        let expression = self.lower_expression_with_side_effects(&contrib.value, module, sink)?;
        let expr_type = self.infer_type(&expression)?;
        if !expr_type.is_numeric() && expr_type != ValueType::Unknown {
            self.record_error_at(
                SemanticErrorKind::TypeMismatch {
                    expected: "numeric".to_string(),
                    found: expr_type.to_string(),
                    context: "contribution expression".to_string(),
                },
                contrib.span,
            );
        }

        self.record_region(AnalyzedRegion::Contribution(AnalyzedContribution {
            branch: branch_name.clone(),
            declared_branch: declared_branch.clone(),
            is_current,
            indirect: false,
            expression: expression.clone(),
            expr_type,
            span: contrib.span,
        }));
        // A guarded contribution contributes zero when inactive
        let expression = self.apply_guard(expression, Self::number_expr(0.0, contrib.span));

        module.contributions.push(AnalyzedContribution {
            branch: branch_name,
            declared_branch,
            is_current,
            indirect: false,
            expression,
            expr_type,
            span: contrib.span,
        });

        Ok(())
    }

    fn validate_direct_zi_contribution(
        &self,
        expression: &Expression,
        span: Span,
    ) -> CompileResult<()> {
        match expression {
            Expression::Call(call) => {
                if is_zi_operator_name(&call.name) {
                    self.validate_direct_zi_site(call.name.as_str(), call.args.get(4), span)?;
                }
                for argument in &call.args {
                    self.validate_direct_zi_contribution(argument, span)?;
                }
            }
            Expression::AnalogOperator(AnalogOperator::Zi { transition, .. }) => {
                self.validate_direct_zi_site("zi operator", transition.as_deref(), span)?;
                self.validate_direct_zi_analog_children(expression, span)?;
            }
            Expression::AnalogOperator(_) => {
                self.validate_direct_zi_analog_children(expression, span)?;
            }
            Expression::Binary(binary) => {
                self.validate_direct_zi_contribution(&binary.left, span)?;
                self.validate_direct_zi_contribution(&binary.right, span)?;
            }
            Expression::Unary(unary) => {
                self.validate_direct_zi_contribution(&unary.operand, span)?;
            }
            Expression::Conditional(conditional) => {
                self.validate_direct_zi_contribution(&conditional.condition, span)?;
                self.validate_direct_zi_contribution(&conditional.then_expr, span)?;
                self.validate_direct_zi_contribution(&conditional.else_expr, span)?;
            }
            Expression::SystemFunction(function) => {
                for argument in &function.args {
                    self.validate_direct_zi_contribution(argument, span)?;
                }
            }
            Expression::ArrayAccess(access) => {
                self.validate_direct_zi_contribution(&access.index, span)?;
            }
            Expression::ArrayLiteral(array) => {
                for element in &array.elements {
                    self.validate_direct_zi_array_element(element, span)?;
                }
            }
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::NullArgument(_)
            | Expression::Identifier(_)
            | Expression::BranchAccess(_) => {}
            Expression::NoiseSource(noise) => match noise {
                NoiseSource::White { power, .. } => {
                    self.validate_direct_zi_contribution(power, span)?;
                }
                NoiseSource::Flicker {
                    power, exponent, ..
                } => {
                    self.validate_direct_zi_contribution(power, span)?;
                    self.validate_direct_zi_contribution(exponent, span)?;
                }
                NoiseSource::Table { data, .. } => {
                    for value in data {
                        self.validate_direct_zi_contribution(value, span)?;
                    }
                }
            },
        }
        Ok(())
    }

    fn validate_direct_zi_array_element(
        &self,
        element: &ArrayLiteralElement,
        span: Span,
    ) -> CompileResult<()> {
        match element {
            ArrayLiteralElement::Value(expression) => {
                self.validate_direct_zi_contribution(expression, span)
            }
            ArrayLiteralElement::Replication(replication) => {
                self.validate_direct_zi_contribution(&replication.count, span)?;
                for element in &replication.elements {
                    self.validate_direct_zi_array_element(element, span)?;
                }
                Ok(())
            }
        }
    }

    fn validate_direct_zi_analog_children(
        &self,
        expression: &Expression,
        span: Span,
    ) -> CompileResult<()> {
        let Expression::AnalogOperator(operator) = expression else {
            return Ok(());
        };
        let visit = |child: &Expression| self.validate_direct_zi_contribution(child, span);
        match operator {
            AnalogOperator::Limit {
                proposed,
                candidate,
                type_metadata,
                ..
            } => {
                visit(proposed)?;
                visit(candidate)?;
                if let Some(value) = type_metadata {
                    visit(value)?;
                }
            }
            AnalogOperator::LimiterArgument { .. } => {}
            AnalogOperator::Ddt { expr, abstol, .. } => {
                visit(expr)?;
                if let Some(value) = abstol {
                    visit(value)?;
                }
            }
            AnalogOperator::Idt {
                expr,
                ic,
                assert_val,
                abstol,
                ..
            } => {
                visit(expr)?;
                for value in [ic, assert_val, abstol].into_iter().flatten() {
                    visit(value)?;
                }
            }
            AnalogOperator::IdtMod {
                expr,
                ic,
                modulus,
                offset,
                abstol,
                ..
            } => {
                visit(expr)?;
                for value in [ic, modulus, offset, abstol].into_iter().flatten() {
                    visit(value)?;
                }
            }
            AnalogOperator::Ddx { expr, .. }
            | AnalogOperator::Limexp { expr, .. }
            | AnalogOperator::LastCrossing { expr, .. } => visit(expr)?,
            AnalogOperator::Absdelay {
                expr,
                delay,
                max_delay,
                ..
            } => {
                visit(expr)?;
                visit(delay)?;
                if let Some(value) = max_delay {
                    visit(value)?;
                }
            }
            AnalogOperator::Transition {
                expr,
                delay,
                rise,
                fall,
                tolerance,
                ..
            } => {
                visit(expr)?;
                for value in [delay, rise, fall, tolerance].into_iter().flatten() {
                    visit(value)?;
                }
            }
            AnalogOperator::Slew {
                expr,
                max_rise,
                max_fall,
                ..
            } => {
                visit(expr)?;
                for value in [max_rise, max_fall].into_iter().flatten() {
                    visit(value)?;
                }
            }
            AnalogOperator::Laplace { expr, kind, .. } => {
                visit(expr)?;
                let (first, second) = match kind {
                    LaplaceKind::ZeroPole { zeros, poles } => (zeros, poles),
                    LaplaceKind::ZeroDenominator { zeros, denominator } => (zeros, denominator),
                    LaplaceKind::NumeratorPole { numerator, poles } => (numerator, poles),
                    LaplaceKind::NumeratorDenominator {
                        numerator,
                        denominator,
                    } => (numerator, denominator),
                };
                for value in first.iter().chain(second) {
                    visit(value)?;
                }
            }
            AnalogOperator::Zi {
                expr,
                kind,
                period,
                transition,
                first_transition,
                ..
            } => {
                visit(expr)?;
                let (first, second) = match kind {
                    ZiKind::ZeroPole { zeros, poles } => (zeros, poles),
                    ZiKind::ZeroDenominator { zeros, denominator } => (zeros, denominator),
                    ZiKind::NumeratorPole { numerator, poles } => (numerator, poles),
                    ZiKind::NumeratorDenominator {
                        numerator,
                        denominator,
                    } => (numerator, denominator),
                };
                for value in first.iter().chain(second) {
                    visit(value)?;
                }
                visit(period)?;
                if let Some(value) = transition {
                    visit(value)?;
                }
                if let Some(value) = first_transition {
                    visit(value)?;
                }
            }
        }
        Ok(())
    }

    fn validate_direct_zi_site(
        &self,
        name: &str,
        transition: Option<&Expression>,
        contribution_span: Span,
    ) -> CompileResult<()> {
        let statically_invalid = match transition {
            None => (self.current_default_transition == 0.0)
                .then_some("the effective `default_transition` is zero".to_string()),
            Some(Expression::NullArgument(_)) => {
                Some("the transition argument is null".to_string())
            }
            Some(value) => self.eval_const_invariant(value).and_then(|value| {
                (!value.is_finite() || value <= 0.0)
                    .then_some(format!("the transition expression evaluates to {value}"))
            }),
        };
        if let Some(reason) = statically_invalid {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(format!(
                    "{name} cannot be contributed directly to an analog branch because {reason}; its transition time must evaluate strictly positive, or the Zi result must first be assigned to an intermediate variable (VAMS-2023 section 4.5.12)"
                )),
                contribution_span,
            )));
        }
        Ok(())
    }

    /// Analyze an indirect contribution `V(x): lhs == rhs`: the target
    /// branch carries an unknown source whose value the solver picks so
    /// the constraint holds. The recorded expression is the residual
    /// `lhs - rhs`; under an inactive guard it degrades to `I(branch)`,
    /// pinning the unknown to zero so the branch opens.
    fn analyze_indirect_contribution(
        &mut self,
        stmt: &IndirectContributionStmt,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        if self.runtime_loop_depth > 0 {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidContribution(
                    "contributions inside loops require compile-time-constant bounds".into(),
                ),
                stmt.span,
            )));
        }

        let (branch_name, is_current, declared_branch) =
            self.resolve_contribution_target(&stmt.branch, module, stmt.span)?;
        self.validate_branch_access_compatible(&stmt.branch, stmt.span)?;

        let lhs = self.lower_expression_with_side_effects(&stmt.lhs, module, sink)?;
        let rhs = self.lower_expression_with_side_effects(&stmt.rhs, module, sink)?;
        for (side, expr) in [("left", &lhs), ("right", &rhs)] {
            let ty = self.infer_type(expr)?;
            if !ty.is_numeric() && ty != ValueType::Unknown && ty != ValueType::NatureAccess {
                self.record_error_at(
                    SemanticErrorKind::TypeMismatch {
                        expected: "numeric".to_string(),
                        found: ty.to_string(),
                        context: format!("{side} side of indirect contribution"),
                    },
                    stmt.span,
                );
            }
        }

        let residual = Self::binary_expr(BinaryOp::Sub, lhs, rhs);

        // Guard fallback: the constraint is replaced by I(branch) = 0
        let fallback = Expression::BranchAccess(match &stmt.branch {
            BranchAccess::Nodes { pos, neg, span, .. } => BranchAccess::Nodes {
                access: "I".into(),
                pos: pos.clone(),
                neg: neg.clone(),
                span: *span,
            },
            BranchAccess::Branch { name, span, .. } => BranchAccess::Branch {
                access: "I".into(),
                name: name.clone(),
                span: *span,
            },
        });
        self.record_region(AnalyzedRegion::Contribution(AnalyzedContribution {
            branch: branch_name.clone(),
            declared_branch: declared_branch.clone(),
            is_current,
            indirect: true,
            expression: residual.clone(),
            expr_type: ValueType::Real,
            span: stmt.span,
        }));
        let expression = self.apply_guard(residual, fallback);

        module.contributions.push(AnalyzedContribution {
            branch: branch_name,
            declared_branch,
            is_current,
            indirect: true,
            expression,
            expr_type: ValueType::Real,
            span: stmt.span,
        });

        Ok(())
    }

    /// `$bound_step(max_dt)`: cap the next transient step while the call
    /// is active. Lowers to `$bound_step = min($bound_step, max_dt)` on a
    /// hidden variable reset to +inf at the top of every evaluation, so
    /// multiple calls and guards compose naturally.
    fn analyze_bound_step(
        &mut self,
        call: &CallStmt,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        let Some(arg) = call.args.first() else {
            self.record_error_at(
                SemanticErrorKind::MissingAttribute(
                    "$bound_step requires a maximum-step argument".into(),
                ),
                call.span,
            );
            return Ok(());
        };
        let var_index =
            self.ensure_task_variable("$bound_step", f64::INFINITY, module, sink, call.span);
        let bound = self.lower_expression_with_side_effects(arg, module, sink)?;
        let current = Expression::Identifier(Identifier {
            name: "$bound_step".into(),
            span: call.span,
        });
        let min = Expression::Call(CallExpr {
            name: "min".into(),
            args: vec![current.clone(), bound],
            span: call.span,
        });
        let expression = self.apply_guard(min, current);
        sink.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
            target: "$bound_step".into(),
            var_index,
            index: None,
            expression,
            expr_type: ValueType::Real,
            span: call.span,
            unfiltered_initial_step_guard: None,
        }));
        Ok(())
    }

    /// `$discontinuity(degree)`: flag a topology/regime change so the
    /// transient stepper places a breakpoint. Lowers to a hidden flag
    /// reset to 0 every evaluation and set to 1 while the call is active.
    fn analyze_discontinuity(
        &mut self,
        call: &CallStmt,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        if let Some(arg) = call.args.first() {
            let degree = self.lower_expression_with_side_effects(arg, module, sink)?;
            let degree_type = self.infer_type(&degree)?;
            if !degree_type.is_numeric() {
                self.record_error_at(
                    SemanticErrorKind::TypeMismatch {
                        expected: "numeric".to_string(),
                        found: degree_type.to_string(),
                        context: "$discontinuity argument".to_string(),
                    },
                    call.span,
                );
            }
        }

        let var_index = self.ensure_task_variable("$discontinuity", 0.0, module, sink, call.span);
        let current = Expression::Identifier(Identifier {
            name: "$discontinuity".into(),
            span: call.span,
        });
        let expression = self.apply_guard(Self::number_expr(1.0, call.span), current);
        sink.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
            target: "$discontinuity".into(),
            var_index,
            index: None,
            expression,
            expr_type: ValueType::Real,
            span: call.span,
            unfiltered_initial_step_guard: None,
        }));
        Ok(())
    }

    /// Register a hidden system-task variable on first use and emit its
    /// unguarded per-evaluation reset (the `$`-prefixed name cannot
    /// collide with user identifiers)
    fn ensure_task_variable(
        &mut self,
        name: &str,
        reset: f64,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
        span: Span,
    ) -> usize {
        if let Some(&idx) = self.task_vars.get(name) {
            return idx;
        }
        let var_index = module.variables.len();
        module.variables.push(AnalyzedVariable {
            name: name.into(),
            var_type: VarType::Real,
            value_type: ValueType::Real,
            is_state: false,
        });
        self.task_vars.insert(name.into(), var_index);
        // The reset runs unconditionally: every evaluation starts neutral
        // and only active calls move the value
        sink.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
            target: name.into(),
            var_index,
            index: None,
            expression: Self::number_expr(reset, span),
            expr_type: ValueType::Real,
            span,
            unfiltered_initial_step_guard: None,
        }));
        var_index
    }

    /// Resolve a contribution target (node pair or named branch) to the
    /// IR branch string and its flow/potential kind
    fn resolve_contribution_target(
        &mut self,
        target: &BranchAccess,
        module: &AnalyzedModule,
        span: Span,
    ) -> CompileResult<(SmolStr, bool, Option<SmolStr>)> {
        match target {
            BranchAccess::Nodes {
                access, pos, neg, ..
            } => {
                let is_current = self.resolve_access_kind(access, span)?;

                // V(name)/I(name) where `name` is a declared branch resolves
                // through the branch table
                if neg.is_none()
                    && let Some(branch) = module.branches.iter().find(|b| b.name == *pos)
                {
                    let branch_str = if branch.neg_node.is_empty() {
                        branch.pos_node.to_string()
                    } else {
                        format!("{},{}", branch.pos_node, branch.neg_node)
                    };
                    Ok((
                        SmolStr::from(branch_str),
                        is_current,
                        Some(branch.name.clone()),
                    ))
                } else {
                    self.validate_node(pos, span)?;
                    if let Some(n) = neg {
                        self.validate_node(n, span)?;
                    }
                    // Format as "pos,neg" for IR parser compatibility
                    let branch = if neg.is_some() {
                        format!("{},{}", pos, neg.as_deref().unwrap())
                    } else {
                        pos.to_string()
                    };
                    Ok((branch.into(), is_current, None))
                }
            }
            BranchAccess::Branch { name, access, .. } => {
                let is_current = self.resolve_access_kind(access, span)?;
                match module.branches.iter().find(|b| b.name == *name) {
                    Some(branch) => {
                        let branch_str = if branch.neg_node.is_empty() {
                            branch.pos_node.to_string()
                        } else {
                            format!("{},{}", branch.pos_node, branch.neg_node)
                        };
                        Ok((
                            SmolStr::from(branch_str),
                            is_current,
                            Some(branch.name.clone()),
                        ))
                    }
                    None => {
                        self.validate_node(name, span)?;
                        Ok((name.to_string().into(), is_current, None))
                    }
                }
            }
        }
    }

    /// Resolve and classify an access function. Unknown access names must
    /// not silently become potential contributions.
    fn resolve_access_kind(&self, access: &str, span: Span) -> CompileResult<bool> {
        if self.disciplines.resolve_access(access).is_none() {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidContribution(format!(
                    "unknown access function '{access}'"
                )),
                span,
            )));
        }
        Ok(self.is_flow_access(access))
    }

    fn validate_branch_access_compatible(
        &self,
        access_expr: &BranchAccess,
        span: Span,
    ) -> CompileResult<()> {
        match access_expr {
            BranchAccess::Nodes {
                access, pos, neg, ..
            } => {
                self.validate_access_compatible_with_symbol(access, pos, span)?;
                if let Some(neg) = neg {
                    self.validate_access_compatible_with_symbol(access, neg, span)?;
                }
            }
            BranchAccess::Branch { access, name, .. } => {
                self.validate_access_compatible_with_symbol(access, name, span)?;
            }
        }
        Ok(())
    }

    fn validate_access_compatible_with_symbol(
        &self,
        access: &str,
        name: &str,
        span: Span,
    ) -> CompileResult<()> {
        let Some(nature) = self.disciplines.resolve_access(access) else {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidContribution(format!(
                    "unknown access function '{access}'"
                )),
                span,
            )));
        };
        let Some(symbol) = self.symbols.lookup(name) else {
            if is_global_ground_name(name) {
                return Ok(());
            }
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UndeclaredSymbol { name: name.into() },
                span,
            )));
        };
        if !matches!(
            symbol.kind,
            SymbolKind::Port | SymbolKind::Node | SymbolKind::Branch
        ) {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidNodeReference {
                    name: name.into(),
                    kind: format!("{:?}", symbol.kind),
                },
                span,
            )));
        }
        let discipline = symbol.attrs.discipline.as_deref().unwrap_or("electrical");
        let Some(discipline_def) = self.disciplines.get_discipline(discipline) else {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UndefinedDiscipline(discipline.to_string()),
                span,
            )));
        };
        if discipline_def.potential.as_deref() == Some(nature.name.as_str())
            || discipline_def.flow.as_deref() == Some(nature.name.as_str())
        {
            return Ok(());
        }

        Err(CompileError::Semantic(SemanticError::new(
            SemanticErrorKind::InvalidContribution(format!(
                "access function '{access}' is incompatible with discipline '{discipline}'"
            )),
            span,
        )))
    }

    /// Whether the access function refers to a flow (current-like) quantity
    fn is_flow_access(&self, access: &str) -> bool {
        if access == "I" {
            return true;
        }
        self.disciplines.is_flow_access(access)
    }

    fn analyze_assignment(
        &mut self,
        assign: &AssignmentStmt,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        // `symbol_name` is the declared symbol checked for kind/type;
        // `target_name` is the storage slot (for const-index array elements
        // they differ: symbol `arr`, storage `arr[k]`)
        let (symbol_name, target_name, span, dyn_index) = match &assign.target {
            LValue::Variable { name, span } => {
                let resolved = self.resolve_substituted_name(name);
                if self.symbols.lookup(&resolved).is_none() {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UndeclaredSymbol { name: name.clone() },
                        *span,
                    )));
                }
                self.symbols.mark_used(&resolved);
                (resolved.clone(), resolved, *span, None)
            }
            LValue::ArrayAccess { name, index, span } => {
                let array_name = self.resolve_substituted_name(name);
                let Some(layout) = self.arrays.get(&array_name).cloned() else {
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "'{}' is indexed but is not a declared array variable",
                            name
                        )),
                        *span,
                    );
                    return Ok(());
                };
                self.symbols.mark_used(&array_name);
                let index = self.lower_expression_with_side_effects(index, module, sink)?;
                if let Some(k) = self.eval_const_invariant(&index) {
                    // Compile-time index: target the element slot directly
                    let k = k.round() as i64;
                    self.check_array_bounds(&array_name, &layout, k, *span)?;
                    let elem = SmolStr::from(format!("{array_name}[{k}]"));
                    (array_name, elem, *span, None)
                } else {
                    (array_name.clone(), array_name, *span, Some(index))
                }
            }
        };

        // Assignments may only target variables
        if let Some(sym) = self.symbols.lookup(&symbol_name)
            && !matches!(sym.kind, SymbolKind::Variable | SymbolKind::LoopVar)
        {
            self.record_error_at(
                SemanticErrorKind::TypeMismatch {
                    expected: "variable".to_string(),
                    found: format!("{:?}", sym.kind).to_lowercase(),
                    context: format!("assignment to '{}'", target_name),
                },
                span,
            );
            return Ok(());
        }

        let expression = self.lower_expression_with_side_effects(&assign.value, module, sink)?;
        let value_type = self.infer_type(&expression)?;

        if let Some(sym) = self.symbols.lookup(&symbol_name)
            && !value_type.can_coerce_to(&sym.value_type)
        {
            self.record_error_at(
                SemanticErrorKind::TypeMismatch {
                    expected: sym.value_type.to_string(),
                    found: value_type.to_string(),
                    context: format!("assignment to '{}'", target_name),
                },
                span,
            );
        }

        if let Some(index) = dyn_index {
            return self.push_indexed_assignment(
                target_name,
                index,
                expression,
                value_type,
                span,
                sink,
            );
        }

        // Find variable index; assignments to unknown storage are an error
        let Some(var_index) = module.variables.iter().position(|v| v.name == target_name) else {
            self.record_error_at(
                SemanticErrorKind::UndeclaredSymbol {
                    name: target_name.clone(),
                },
                span,
            );
            return Ok(());
        };

        // Under a guard, the variable keeps its previous value when the
        // guard is inactive
        let fallback = Expression::Identifier(Identifier {
            name: target_name.clone(),
            span,
        });
        // Structured form first: it needs the expression as written, and the
        // next line is where that stops being available.
        self.record_region(AnalyzedRegion::Assignment(AnalyzedAssignment {
            target: target_name.clone(),
            var_index,
            index: None,
            expression: expression.clone(),
            expr_type: value_type,
            span,
            unfiltered_initial_step_guard: self.unfiltered_initial_step_guards.last().cloned(),
        }));
        let expression = self.apply_guard(expression, fallback);

        // Record the assignment for code generation
        sink.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
            target: target_name,
            var_index,
            index: None,
            expression,
            expr_type: value_type,
            span,
            unfiltered_initial_step_guard: self.unfiltered_initial_step_guards.last().cloned(),
        }));

        Ok(())
    }

    /// Record an array element assignment whose index is only known at
    /// runtime. Guards fall back to re-reading the same element, so an
    /// inactive guard leaves the array untouched.
    fn push_indexed_assignment(
        &mut self,
        array_name: SmolStr,
        index: Expression,
        expression: Expression,
        value_type: ValueType,
        span: Span,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        let layout = self.arrays.get(&array_name).cloned().expect("checked");
        let fallback = Expression::ArrayAccess(ArrayAccessExpr {
            array: array_name.clone(),
            index: Box::new(index.clone()),
            span,
        });
        self.record_region(AnalyzedRegion::Assignment(AnalyzedAssignment {
            target: array_name.clone(),
            var_index: layout.base,
            index: Some(index.clone()),
            expression: expression.clone(),
            expr_type: value_type,
            span,
            unfiltered_initial_step_guard: None,
        }));
        let expression = self.apply_guard(expression, fallback);
        sink.push(AnalyzedStatement::Assignment(AnalyzedAssignment {
            target: array_name,
            var_index: layout.base,
            index: Some(index),
            expression,
            expr_type: value_type,
            span,
            unfiltered_initial_step_guard: None,
        }));
        Ok(())
    }

    /// Validate a compile-time array index against the declared bounds
    fn check_array_bounds(
        &self,
        name: &SmolStr,
        layout: &AnalyzedArray,
        k: i64,
        span: Span,
    ) -> CompileResult<()> {
        if k < layout.lower || k >= layout.lower + layout.len as i64 {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::IndexOutOfBounds(format!(
                    "index {k} outside '{name}[{}:{}]'",
                    layout.lower,
                    layout.lower + layout.len as i64 - 1
                )),
                span,
            )));
        }
        Ok(())
    }

    /// Resolve a name through the substitution stack (innermost first).
    /// Only identity renames (identifier-to-identifier) are returned; other
    /// substitutions keep the original name.
    fn resolve_substituted_name(&self, name: &SmolStr) -> SmolStr {
        for frame in self.subst_stack.iter().rev() {
            if let Some(expr) = frame.get(name) {
                if let Expression::Identifier(id) = expr {
                    return id.name.clone();
                }
                return name.clone();
            }
        }
        name.clone()
    }

    /// Look up a substitution for an identifier
    fn lookup_substitution(&self, name: &SmolStr) -> Option<Expression> {
        for frame in self.subst_stack.iter().rev() {
            if let Some(expr) = frame.get(name) {
                return Some(expr.clone());
            }
        }
        None
    }

    fn materialize_output_function_call(
        &mut self,
        expr: &Expression,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<Option<Expression>> {
        let Expression::Call(call) = expr else {
            return Ok(None);
        };
        let Some(func) = self.user_functions.get(&call.name).cloned() else {
            return Ok(None);
        };
        if !Self::function_should_materialize(&func) {
            return Ok(None);
        }
        if self.inline_depth >= Self::MAX_INLINE_DEPTH {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::CircularDependency(format!(
                    "analog function '{}' (recursive call chain?)",
                    call.name
                )),
                call.span,
            )));
        }
        if call.args.len() != func.params.len() {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::ArgumentCountMismatch {
                    name: call.name.to_string(),
                    expected: func.params.len().to_string(),
                    got: call.args.len(),
                },
                call.span,
            )));
        }

        let mut output_bindings = Vec::new();
        for (param, arg) in func.params.iter().zip(call.args.iter()) {
            if param.direction != ParamDirection::Input {
                let target = self.function_output_lvalue(&func.name, param, arg)?;
                output_bindings.push((param.name.clone(), target, param.span));
            }
        }

        self.local_counter += 1;
        let call_id = self.local_counter;
        let prefix = format!("__fn{call_id}_{}", func.name);
        let make_name = |name: &SmolStr| -> SmolStr { SmolStr::from(format!("{prefix}__{name}")) };

        let return_name: SmolStr = format!("{prefix}__return").into();
        self.register_function_temp(module, return_name.clone(), func.return_type, func.span)?;

        let mut frame = HashMap::new();
        frame.insert(
            func.name.clone(),
            Expression::Identifier(Identifier {
                name: return_name.clone(),
                span: func.span,
            }),
        );

        let mut formal_temps = HashMap::new();
        for param in &func.params {
            let temp_name = make_name(&param.name);
            self.register_function_temp(module, temp_name.clone(), param.param_type, param.span)?;
            formal_temps.insert(param.name.clone(), temp_name.clone());
            frame.insert(
                param.name.clone(),
                Expression::Identifier(Identifier {
                    name: temp_name,
                    span: param.span,
                }),
            );
        }

        let zero_return = AssignmentStmt {
            target: LValue::Variable {
                name: return_name.clone(),
                span: func.span,
            },
            value: Self::number_expr(0.0, func.span),
            span: func.span,
        };
        self.analyze_assignment(&zero_return, module, sink)?;

        for (param, arg) in func.params.iter().zip(call.args.iter()) {
            let formal_temp = formal_temps
                .get(&param.name)
                .expect("formal temp registered")
                .clone();
            let value = match param.direction {
                ParamDirection::Input | ParamDirection::Inout => arg.clone(),
                ParamDirection::Output => Self::number_expr(0.0, param.span),
            };
            let assignment = AssignmentStmt {
                target: LValue::Variable {
                    name: formal_temp,
                    span: param.span,
                },
                value,
                span: param.span,
            };
            self.analyze_assignment(&assignment, module, sink)?;
        }

        for var_decl in &func.locals {
            for item in &var_decl.items {
                if func.params.iter().any(|param| param.name == item.name) {
                    continue;
                }
                if !item.dimensions.is_empty() {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "array local '{}' in analog function '{}'",
                            item.name, func.name
                        )),
                        item.span,
                    )));
                }
                let temp_name = make_name(&item.name);
                self.register_function_temp(
                    module,
                    temp_name.clone(),
                    var_decl.var_type,
                    item.span,
                )?;
                frame.insert(
                    item.name.clone(),
                    Expression::Identifier(Identifier {
                        name: temp_name.clone(),
                        span: item.span,
                    }),
                );
            }
        }

        self.subst_stack.push(frame);
        self.inline_depth += 1;
        let body_result = (|| -> CompileResult<()> {
            for var_decl in &func.locals {
                for item in &var_decl.items {
                    if func.params.iter().any(|param| param.name == item.name) {
                        continue;
                    }
                    let target_name = self.resolve_substituted_name(&item.name);
                    let value = item
                        .init
                        .clone()
                        .unwrap_or_else(|| Self::number_expr(0.0, item.span));
                    let assignment = AssignmentStmt {
                        target: LValue::Variable {
                            name: target_name,
                            span: item.span,
                        },
                        value,
                        span: item.span,
                    };
                    self.analyze_assignment(&assignment, module, sink)?;
                }
            }
            for statement in &func.body.statements {
                self.analyze_statement(statement, module, sink)?;
            }
            Ok(())
        })();
        self.inline_depth -= 1;
        self.subst_stack.pop().expect("function frame");
        body_result?;

        for (formal, target, span) in output_bindings {
            let formal_temp = formal_temps.get(&formal).expect("formal temp registered");
            let assignment = AssignmentStmt {
                target,
                value: Expression::Identifier(Identifier {
                    name: formal_temp.clone(),
                    span,
                }),
                span,
            };
            self.analyze_assignment(&assignment, module, sink)?;
        }

        Ok(Some(Expression::Identifier(Identifier {
            name: return_name,
            span: call.span,
        })))
    }

    fn lower_expression_with_side_effects(
        &mut self,
        expr: &Expression,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<Expression> {
        let expr = self.materialize_output_function_calls(expr, module, sink)?;
        let side_effect_start = self.function_side_effects.len();
        let lowered = self.lower_expression(&expr)?;
        let side_effects = self.function_side_effects.split_off(side_effect_start);
        for assignment in side_effects {
            self.analyze_assignment(&assignment, module, sink)?;
        }
        Ok(lowered)
    }

    fn materialize_output_function_calls(
        &mut self,
        expr: &Expression,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<Expression> {
        Ok(match expr {
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::NullArgument(_)
            | Expression::Identifier(_)
            | Expression::BranchAccess(_) => expr.clone(),
            Expression::SystemFunction(function) => Expression::SystemFunction(SystemFunction {
                name: function.name.clone(),
                args: function
                    .args
                    .iter()
                    .map(|arg| self.materialize_output_function_calls(arg, module, sink))
                    .collect::<CompileResult<Vec<_>>>()?,
                span: function.span,
            }),
            Expression::Binary(binary) => Expression::Binary(BinaryExpr {
                op: binary.op,
                left: Box::new(self.materialize_output_function_calls(
                    &binary.left,
                    module,
                    sink,
                )?),
                right: Box::new(self.materialize_output_function_calls(
                    &binary.right,
                    module,
                    sink,
                )?),
                span: binary.span,
            }),
            Expression::Unary(unary) => Expression::Unary(UnaryExpr {
                op: unary.op,
                operand: Box::new(self.materialize_output_function_calls(
                    &unary.operand,
                    module,
                    sink,
                )?),
                span: unary.span,
            }),
            Expression::Conditional(conditional) => {
                if self.expression_contains_output_function_call(expr) {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::InvalidAnalogOperator(
                            "analog function output/inout arguments are not supported inside conditional expressions".into(),
                        ),
                        conditional.span,
                    )));
                }
                Expression::Conditional(ConditionalExpr {
                    condition: Box::new(self.materialize_output_function_calls(
                        &conditional.condition,
                        module,
                        sink,
                    )?),
                    then_expr: Box::new(self.materialize_output_function_calls(
                        &conditional.then_expr,
                        module,
                        sink,
                    )?),
                    else_expr: Box::new(self.materialize_output_function_calls(
                        &conditional.else_expr,
                        module,
                        sink,
                    )?),
                    span: conditional.span,
                })
            }
            Expression::Call(call) => {
                if let Some(func) = self.user_functions.get(&call.name).cloned()
                    && Self::function_should_materialize(&func)
                {
                    let args = if call.args.len() == func.params.len() {
                        call.args
                            .iter()
                            .zip(func.params.iter())
                            .map(|(arg, param)| match param.direction {
                                ParamDirection::Input => {
                                    self.materialize_output_function_calls(arg, module, sink)
                                }
                                ParamDirection::Output | ParamDirection::Inout => Ok(arg.clone()),
                            })
                            .collect::<CompileResult<Vec<_>>>()?
                    } else {
                        call.args.clone()
                    };
                    let call = Expression::Call(CallExpr {
                        name: call.name.clone(),
                        args,
                        span: call.span,
                    });
                    return self
                        .materialize_output_function_call(&call, module, sink)?
                        .ok_or_else(|| {
                            CompileError::Semantic(SemanticError::new(
                                SemanticErrorKind::InvalidAnalogOperator(
                                    "analog function output/inout call could not be materialized"
                                        .into(),
                                ),
                                expr.span(),
                            ))
                        });
                }
                Expression::Call(CallExpr {
                    name: call.name.clone(),
                    args: call
                        .args
                        .iter()
                        .map(|arg| self.materialize_output_function_calls(arg, module, sink))
                        .collect::<CompileResult<Vec<_>>>()?,
                    span: call.span,
                })
            }
            Expression::ArrayAccess(access) => Expression::ArrayAccess(ArrayAccessExpr {
                array: access.array.clone(),
                index: Box::new(self.materialize_output_function_calls(
                    &access.index,
                    module,
                    sink,
                )?),
                span: access.span,
            }),
            Expression::ArrayLiteral(array) => Expression::ArrayLiteral(ArrayLiteralExpr {
                elements: array
                    .elements
                    .iter()
                    .map(|element| {
                        self.materialize_output_function_calls_in_array_element(
                            element, module, sink,
                        )
                    })
                    .collect::<CompileResult<Vec<_>>>()?,
                assignment_pattern: array.assignment_pattern,
                span: array.span,
            }),
            Expression::AnalogOperator(op) => Expression::AnalogOperator(
                self.materialize_output_function_calls_in_analog_operator(op, module, sink)?,
            ),
            Expression::NoiseSource(noise) => Expression::NoiseSource(
                self.materialize_output_function_calls_in_noise_source(noise, module, sink)?,
            ),
        })
    }

    fn materialize_output_function_calls_in_array_element(
        &mut self,
        element: &ArrayLiteralElement,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<ArrayLiteralElement> {
        Ok(match element {
            ArrayLiteralElement::Value(expression) => ArrayLiteralElement::Value(
                self.materialize_output_function_calls(expression, module, sink)?,
            ),
            ArrayLiteralElement::Replication(replication) => {
                ArrayLiteralElement::Replication(ReplicationExpr {
                    count: Box::new(self.materialize_output_function_calls(
                        &replication.count,
                        module,
                        sink,
                    )?),
                    elements: replication
                        .elements
                        .iter()
                        .map(|element| {
                            self.materialize_output_function_calls_in_array_element(
                                element, module, sink,
                            )
                        })
                        .collect::<CompileResult<Vec<_>>>()?,
                    span: replication.span,
                })
            }
        })
    }

    fn materialize_output_function_calls_in_analog_operator(
        &mut self,
        op: &AnalogOperator,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<AnalogOperator> {
        Ok(match op {
            AnalogOperator::Limit {
                proposed,
                candidate,
                type_metadata,
                selector,
                span,
            } => AnalogOperator::Limit {
                proposed: self.materialize_output_function_calls_box(proposed, module, sink)?,
                candidate: self.materialize_output_function_calls_box(candidate, module, sink)?,
                type_metadata: self.materialize_output_function_calls_opt_box(
                    type_metadata,
                    module,
                    sink,
                )?,
                selector: selector.clone(),
                span: *span,
            },
            AnalogOperator::LimiterArgument { argument, span } => AnalogOperator::LimiterArgument {
                argument: *argument,
                span: *span,
            },
            AnalogOperator::Ddt { expr, abstol, span } => AnalogOperator::Ddt {
                expr: self.materialize_output_function_calls_box(expr, module, sink)?,
                abstol: self.materialize_output_function_calls_opt_box(abstol, module, sink)?,
                span: *span,
            },
            AnalogOperator::Idt {
                expr,
                ic,
                assert_val,
                abstol,
                span,
            } => AnalogOperator::Idt {
                expr: self.materialize_output_function_calls_box(expr, module, sink)?,
                ic: self.materialize_output_function_calls_opt_box(ic, module, sink)?,
                assert_val: self
                    .materialize_output_function_calls_opt_box(assert_val, module, sink)?,
                abstol: self.materialize_output_function_calls_opt_box(abstol, module, sink)?,
                span: *span,
            },
            AnalogOperator::IdtMod {
                expr,
                ic,
                modulus,
                offset,
                abstol,
                span,
            } => AnalogOperator::IdtMod {
                expr: self.materialize_output_function_calls_box(expr, module, sink)?,
                ic: self.materialize_output_function_calls_opt_box(ic, module, sink)?,
                modulus: self.materialize_output_function_calls_opt_box(modulus, module, sink)?,
                offset: self.materialize_output_function_calls_opt_box(offset, module, sink)?,
                abstol: self.materialize_output_function_calls_opt_box(abstol, module, sink)?,
                span: *span,
            },
            AnalogOperator::Ddx { expr, probe, span } => AnalogOperator::Ddx {
                expr: self.materialize_output_function_calls_box(expr, module, sink)?,
                probe: probe.clone(),
                span: *span,
            },
            AnalogOperator::Limexp { expr, span } => AnalogOperator::Limexp {
                expr: self.materialize_output_function_calls_box(expr, module, sink)?,
                span: *span,
            },
            AnalogOperator::Absdelay {
                expr,
                delay,
                max_delay,
                span,
            } => AnalogOperator::Absdelay {
                expr: self.materialize_output_function_calls_box(expr, module, sink)?,
                delay: self.materialize_output_function_calls_box(delay, module, sink)?,
                max_delay: self
                    .materialize_output_function_calls_opt_box(max_delay, module, sink)?,
                span: *span,
            },
            AnalogOperator::Transition {
                expr,
                delay,
                rise,
                fall,
                tolerance,
                span,
            } => AnalogOperator::Transition {
                expr: self.materialize_output_function_calls_box(expr, module, sink)?,
                delay: self.materialize_output_function_calls_opt_box(delay, module, sink)?,
                rise: self.materialize_output_function_calls_opt_box(rise, module, sink)?,
                fall: self.materialize_output_function_calls_opt_box(fall, module, sink)?,
                tolerance: self
                    .materialize_output_function_calls_opt_box(tolerance, module, sink)?,
                span: *span,
            },
            AnalogOperator::Slew {
                expr,
                max_rise,
                max_fall,
                span,
            } => AnalogOperator::Slew {
                expr: self.materialize_output_function_calls_box(expr, module, sink)?,
                max_rise: self.materialize_output_function_calls_opt_box(max_rise, module, sink)?,
                max_fall: self.materialize_output_function_calls_opt_box(max_fall, module, sink)?,
                span: *span,
            },
            AnalogOperator::LastCrossing { expr, edge, span } => AnalogOperator::LastCrossing {
                expr: self.materialize_output_function_calls_box(expr, module, sink)?,
                edge: *edge,
                span: *span,
            },
            AnalogOperator::Laplace { kind, expr, span } => AnalogOperator::Laplace {
                kind: self.materialize_output_function_calls_in_laplace_kind(kind, module, sink)?,
                expr: self.materialize_output_function_calls_box(expr, module, sink)?,
                span: *span,
            },
            AnalogOperator::Zi {
                kind,
                expr,
                period,
                transition,
                first_transition,
                span,
            } => AnalogOperator::Zi {
                kind: self.materialize_output_function_calls_in_zi_kind(kind, module, sink)?,
                expr: self.materialize_output_function_calls_box(expr, module, sink)?,
                period: self.materialize_output_function_calls_box(period, module, sink)?,
                transition: self
                    .materialize_output_function_calls_opt_box(transition, module, sink)?,
                first_transition: self.materialize_output_function_calls_opt_box(
                    first_transition,
                    module,
                    sink,
                )?,
                span: *span,
            },
        })
    }

    fn materialize_output_function_calls_box(
        &mut self,
        expr: &Expression,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<Box<Expression>> {
        Ok(Box::new(
            self.materialize_output_function_calls(expr, module, sink)?,
        ))
    }

    fn materialize_output_function_calls_opt_box(
        &mut self,
        expr: &Option<Box<Expression>>,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<Option<Box<Expression>>> {
        expr.as_deref()
            .map(|expr| self.materialize_output_function_calls_box(expr, module, sink))
            .transpose()
    }

    fn materialize_output_function_calls_in_laplace_kind(
        &mut self,
        kind: &LaplaceKind,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<LaplaceKind> {
        Ok(match kind {
            LaplaceKind::ZeroPole { zeros, poles } => LaplaceKind::ZeroPole {
                zeros: self.materialize_output_function_calls_in_expr_list(zeros, module, sink)?,
                poles: self.materialize_output_function_calls_in_expr_list(poles, module, sink)?,
            },
            LaplaceKind::ZeroDenominator { zeros, denominator } => LaplaceKind::ZeroDenominator {
                zeros: self.materialize_output_function_calls_in_expr_list(zeros, module, sink)?,
                denominator: self.materialize_output_function_calls_in_expr_list(
                    denominator,
                    module,
                    sink,
                )?,
            },
            LaplaceKind::NumeratorPole { numerator, poles } => LaplaceKind::NumeratorPole {
                numerator: self
                    .materialize_output_function_calls_in_expr_list(numerator, module, sink)?,
                poles: self.materialize_output_function_calls_in_expr_list(poles, module, sink)?,
            },
            LaplaceKind::NumeratorDenominator {
                numerator,
                denominator,
            } => LaplaceKind::NumeratorDenominator {
                numerator: self
                    .materialize_output_function_calls_in_expr_list(numerator, module, sink)?,
                denominator: self.materialize_output_function_calls_in_expr_list(
                    denominator,
                    module,
                    sink,
                )?,
            },
        })
    }

    fn materialize_output_function_calls_in_zi_kind(
        &mut self,
        kind: &ZiKind,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<ZiKind> {
        Ok(match kind {
            ZiKind::ZeroPole { zeros, poles } => ZiKind::ZeroPole {
                zeros: self.materialize_output_function_calls_in_expr_list(zeros, module, sink)?,
                poles: self.materialize_output_function_calls_in_expr_list(poles, module, sink)?,
            },
            ZiKind::ZeroDenominator { zeros, denominator } => ZiKind::ZeroDenominator {
                zeros: self.materialize_output_function_calls_in_expr_list(zeros, module, sink)?,
                denominator: self.materialize_output_function_calls_in_expr_list(
                    denominator,
                    module,
                    sink,
                )?,
            },
            ZiKind::NumeratorPole { numerator, poles } => ZiKind::NumeratorPole {
                numerator: self
                    .materialize_output_function_calls_in_expr_list(numerator, module, sink)?,
                poles: self.materialize_output_function_calls_in_expr_list(poles, module, sink)?,
            },
            ZiKind::NumeratorDenominator {
                numerator,
                denominator,
            } => ZiKind::NumeratorDenominator {
                numerator: self
                    .materialize_output_function_calls_in_expr_list(numerator, module, sink)?,
                denominator: self.materialize_output_function_calls_in_expr_list(
                    denominator,
                    module,
                    sink,
                )?,
            },
        })
    }

    fn materialize_output_function_calls_in_noise_source(
        &mut self,
        noise: &NoiseSource,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<NoiseSource> {
        Ok(match noise {
            NoiseSource::White { power, name, span } => NoiseSource::White {
                power: Box::new(self.materialize_output_function_calls(power, module, sink)?),
                name: name.clone(),
                span: *span,
            },
            NoiseSource::Flicker {
                power,
                exponent,
                name,
                span,
            } => NoiseSource::Flicker {
                power: Box::new(self.materialize_output_function_calls(power, module, sink)?),
                exponent: Box::new(self.materialize_output_function_calls(exponent, module, sink)?),
                name: name.clone(),
                span: *span,
            },
            NoiseSource::Table { data, name, span } => NoiseSource::Table {
                data: self.materialize_output_function_calls_in_expr_list(data, module, sink)?,
                name: name.clone(),
                span: *span,
            },
        })
    }

    fn materialize_output_function_calls_in_expr_list(
        &mut self,
        exprs: &[Expression],
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<Vec<Expression>> {
        exprs
            .iter()
            .map(|expr| self.materialize_output_function_calls(expr, module, sink))
            .collect()
    }

    fn expression_contains_output_function_call(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Call(call) => {
                self.user_functions
                    .get(&call.name)
                    .is_some_and(Self::function_needs_materialization)
                    || call
                        .args
                        .iter()
                        .any(|arg| self.expression_contains_output_function_call(arg))
            }
            Expression::SystemFunction(function) => function
                .args
                .iter()
                .any(|arg| self.expression_contains_output_function_call(arg)),
            Expression::Binary(binary) => {
                self.expression_contains_output_function_call(&binary.left)
                    || self.expression_contains_output_function_call(&binary.right)
            }
            Expression::Unary(unary) => {
                self.expression_contains_output_function_call(&unary.operand)
            }
            Expression::Conditional(conditional) => {
                self.expression_contains_output_function_call(&conditional.condition)
                    || self.expression_contains_output_function_call(&conditional.then_expr)
                    || self.expression_contains_output_function_call(&conditional.else_expr)
            }
            Expression::ArrayAccess(access) => {
                self.expression_contains_output_function_call(&access.index)
            }
            Expression::ArrayLiteral(array) => array
                .elements
                .iter()
                .any(|element| self.array_element_contains_output_function_call(element)),
            Expression::AnalogOperator(op) => {
                self.analog_operator_contains_output_function_call(op)
            }
            Expression::NoiseSource(noise) => {
                self.noise_source_contains_output_function_call(noise)
            }
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::NullArgument(_)
            | Expression::Identifier(_)
            | Expression::BranchAccess(_) => false,
        }
    }

    fn array_element_contains_output_function_call(&self, element: &ArrayLiteralElement) -> bool {
        match element {
            ArrayLiteralElement::Value(expression) => {
                self.expression_contains_output_function_call(expression)
            }
            ArrayLiteralElement::Replication(replication) => {
                self.expression_contains_output_function_call(&replication.count)
                    || replication
                        .elements
                        .iter()
                        .any(|element| self.array_element_contains_output_function_call(element))
            }
        }
    }

    fn analog_operator_contains_output_function_call(&self, op: &AnalogOperator) -> bool {
        let contains_opt = |expr: &Option<Box<Expression>>| {
            expr.as_ref()
                .is_some_and(|expr| self.expression_contains_output_function_call(expr))
        };
        match op {
            AnalogOperator::Limit {
                proposed,
                candidate,
                type_metadata,
                ..
            } => {
                self.expression_contains_output_function_call(proposed)
                    || self.expression_contains_output_function_call(candidate)
                    || contains_opt(type_metadata)
            }
            AnalogOperator::LimiterArgument { .. } => false,
            AnalogOperator::Ddt { expr, abstol, .. } => {
                self.expression_contains_output_function_call(expr) || contains_opt(abstol)
            }
            AnalogOperator::Idt {
                expr,
                ic,
                assert_val,
                abstol,
                ..
            } => {
                self.expression_contains_output_function_call(expr)
                    || contains_opt(ic)
                    || contains_opt(assert_val)
                    || contains_opt(abstol)
            }
            AnalogOperator::IdtMod {
                expr,
                ic,
                modulus,
                offset,
                abstol,
                ..
            } => {
                self.expression_contains_output_function_call(expr)
                    || contains_opt(ic)
                    || contains_opt(modulus)
                    || contains_opt(offset)
                    || contains_opt(abstol)
            }
            AnalogOperator::Ddx { expr, .. }
            | AnalogOperator::Limexp { expr, .. }
            | AnalogOperator::LastCrossing { expr, .. } => {
                self.expression_contains_output_function_call(expr)
            }
            AnalogOperator::Laplace { kind, expr, .. } => {
                self.expression_contains_output_function_call(expr)
                    || self.laplace_kind_contains_output_function_call(kind)
            }
            AnalogOperator::Zi {
                kind, expr, period, ..
            } => {
                self.expression_contains_output_function_call(expr)
                    || self.expression_contains_output_function_call(period)
                    || self.zi_kind_contains_output_function_call(kind)
            }
            AnalogOperator::Absdelay {
                expr,
                delay,
                max_delay,
                ..
            } => {
                self.expression_contains_output_function_call(expr)
                    || self.expression_contains_output_function_call(delay)
                    || contains_opt(max_delay)
            }
            AnalogOperator::Transition {
                expr,
                delay,
                rise,
                fall,
                tolerance,
                ..
            } => {
                self.expression_contains_output_function_call(expr)
                    || contains_opt(delay)
                    || contains_opt(rise)
                    || contains_opt(fall)
                    || contains_opt(tolerance)
            }
            AnalogOperator::Slew {
                expr,
                max_rise,
                max_fall,
                ..
            } => {
                self.expression_contains_output_function_call(expr)
                    || contains_opt(max_rise)
                    || contains_opt(max_fall)
            }
        }
    }

    fn laplace_kind_contains_output_function_call(&self, kind: &LaplaceKind) -> bool {
        match kind {
            LaplaceKind::ZeroPole { zeros, poles } => {
                self.expr_list_contains_output_function_call(zeros)
                    || self.expr_list_contains_output_function_call(poles)
            }
            LaplaceKind::ZeroDenominator { zeros, denominator } => {
                self.expr_list_contains_output_function_call(zeros)
                    || self.expr_list_contains_output_function_call(denominator)
            }
            LaplaceKind::NumeratorPole { numerator, poles } => {
                self.expr_list_contains_output_function_call(numerator)
                    || self.expr_list_contains_output_function_call(poles)
            }
            LaplaceKind::NumeratorDenominator {
                numerator,
                denominator,
            } => {
                self.expr_list_contains_output_function_call(numerator)
                    || self.expr_list_contains_output_function_call(denominator)
            }
        }
    }

    fn zi_kind_contains_output_function_call(&self, kind: &ZiKind) -> bool {
        match kind {
            ZiKind::ZeroPole { zeros, poles } => {
                self.expr_list_contains_output_function_call(zeros)
                    || self.expr_list_contains_output_function_call(poles)
            }
            ZiKind::ZeroDenominator { zeros, denominator } => {
                self.expr_list_contains_output_function_call(zeros)
                    || self.expr_list_contains_output_function_call(denominator)
            }
            ZiKind::NumeratorPole { numerator, poles } => {
                self.expr_list_contains_output_function_call(numerator)
                    || self.expr_list_contains_output_function_call(poles)
            }
            ZiKind::NumeratorDenominator {
                numerator,
                denominator,
            } => {
                self.expr_list_contains_output_function_call(numerator)
                    || self.expr_list_contains_output_function_call(denominator)
            }
        }
    }

    fn expr_list_contains_output_function_call(&self, exprs: &[Expression]) -> bool {
        exprs
            .iter()
            .any(|expr| self.expression_contains_output_function_call(expr))
    }

    fn noise_source_contains_output_function_call(&self, noise: &NoiseSource) -> bool {
        match noise {
            NoiseSource::White { power, .. } => {
                self.expression_contains_output_function_call(power)
            }
            NoiseSource::Flicker {
                power, exponent, ..
            } => {
                self.expression_contains_output_function_call(power)
                    || self.expression_contains_output_function_call(exponent)
            }
            NoiseSource::Table { data, .. } => data
                .iter()
                .any(|expr| self.expression_contains_output_function_call(expr)),
        }
    }

    fn lower_expression_without_side_effects(
        &mut self,
        expr: &Expression,
        context: &str,
    ) -> CompileResult<Expression> {
        let side_effect_start = self.function_side_effects.len();
        let lowered = self.lower_expression(expr)?;
        if self.function_side_effects.len() != side_effect_start {
            self.function_side_effects.truncate(side_effect_start);
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(format!(
                    "analog function output/inout arguments are not supported in {context}"
                )),
                expr.span(),
            )));
        }
        Ok(lowered)
    }

    /// Rewrite an expression: apply substitutions (block locals, loop
    /// variables) and inline calls to user-defined analog functions.
    fn lower_expression(&mut self, expr: &Expression) -> CompileResult<Expression> {
        Ok(match expr {
            Expression::Identifier(id) => {
                if self.parameter_arrays.contains(&id.name) {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "parameter array '{}' cannot be used as a scalar expression",
                            id.name
                        )),
                        id.span,
                    )));
                }
                match self.lookup_substitution(&id.name) {
                    Some(subst) => subst,
                    None => expr.clone(),
                }
            }
            Expression::Number(_) | Expression::StringLit(_) | Expression::NullArgument(_) => {
                expr.clone()
            }
            Expression::BranchAccess(access) => {
                self.validate_branch_access_compatible(access, access.span())?;
                expr.clone()
            }
            Expression::Binary(b) => {
                let left = self.lower_expression(&b.left)?;
                let right = self.lower_expression(&b.right)?;
                if matches!(
                    b.op,
                    BinaryOp::BitAnd
                        | BinaryOp::BitOr
                        | BinaryOp::BitXor
                        | BinaryOp::Shl
                        | BinaryOp::Shr
                ) {
                    Self::validate_integer_operator_operand(
                        self.infer_type(&left)?,
                        "left operand of bitwise or shift operator",
                        left.span(),
                    )?;
                    Self::validate_integer_operator_operand(
                        self.infer_type(&right)?,
                        "right operand of bitwise or shift operator",
                        right.span(),
                    )?;
                }
                Expression::Binary(BinaryExpr {
                    op: b.op,
                    left: Box::new(left),
                    right: Box::new(right),
                    span: b.span,
                })
            }
            Expression::Unary(u) => {
                let operand = self.lower_expression(&u.operand)?;
                if u.op == UnaryOp::BitNot {
                    Self::validate_integer_operator_operand(
                        self.infer_type(&operand)?,
                        "operand of bitwise complement",
                        operand.span(),
                    )?;
                }
                Expression::Unary(UnaryExpr {
                    op: u.op,
                    operand: Box::new(operand),
                    span: u.span,
                })
            }
            Expression::Conditional(c) => {
                let condition = self.lower_expression(&c.condition)?;
                let dynamic_condition = !self.expression_is_simulation_invariant(&condition);
                if dynamic_condition {
                    self.dynamic_analog_operator_guard_depth += 1;
                }
                let then_result = self.lower_expression(&c.then_expr);
                let else_result = self.lower_expression(&c.else_expr);
                if dynamic_condition {
                    self.dynamic_analog_operator_guard_depth -= 1;
                }
                Expression::Conditional(ConditionalExpr {
                    condition: Box::new(condition),
                    then_expr: Box::new(then_result?),
                    else_expr: Box::new(else_result?),
                    span: c.span,
                })
            }
            Expression::SystemFunction(f) => {
                self.validate_limit_call(f)?;
                if let Some(limit) = self.lower_custom_limit_call(f)? {
                    return Ok(limit);
                }
                let args = f
                    .args
                    .iter()
                    .map(|a| self.lower_expression(a))
                    .collect::<CompileResult<Vec<_>>>()?;
                Expression::SystemFunction(SystemFunction {
                    name: f.name.clone(),
                    args,
                    span: f.span,
                })
            }
            Expression::Call(call) => {
                self.validate_builtin_call_arity(call)?;
                self.validate_null_arguments(call)?;
                self.validate_filter_vector_operands(call)?;
                let call = self.materialize_filter_call_replication(call)?;
                if is_zi_operator_name(&call.name) {
                    self.validate_zi_operand_budget(&call)?;
                    self.validate_zi_definition_purity(&call)?;
                }

                if let Some(operator) = stateful_analog_operator_call_name(&call.name) {
                    self.validate_stateful_analog_operator_placement(operator, call.span)?;
                }

                // Nature access functions other than V/I (Pwr, Temp, ...)
                // parse as calls; rewrite them into branch accesses.
                if !self.user_functions.contains_key(&call.name)
                    && self.disciplines.resolve_access(&call.name).is_some()
                    && matches!(call.args.len(), 1 | 2)
                    && call.args.iter().all(|a| {
                        matches!(a, Expression::Identifier(id)
                        if self.symbols.lookup(&id.name).is_some_and(|s| matches!(
                            s.kind,
                            SymbolKind::Port | SymbolKind::Node | SymbolKind::Branch
                        )))
                    })
                {
                    let mut nodes = call.args.iter().map(|a| match a {
                        Expression::Identifier(id) => id.name.clone(),
                        _ => unreachable!(),
                    });
                    let access_expr = BranchAccess::Nodes {
                        access: call.name.clone(),
                        pos: nodes.next().unwrap(),
                        neg: nodes.next(),
                        span: call.span,
                    };
                    self.validate_branch_access_compatible(&access_expr, call.span)?;
                    return Ok(Expression::BranchAccess(access_expr));
                }

                if let Some(func) = self.user_functions.get(&call.name) {
                    if Self::is_recognized_limited_exp_function(func) {
                        let args = call
                            .args
                            .iter()
                            .map(|a| self.lower_expression(a))
                            .collect::<CompileResult<Vec<_>>>()?;
                        return Ok(Expression::Call(CallExpr {
                            name: RSPICE_LIMITED_EXP_INTRINSIC.into(),
                            args,
                            span: call.span,
                        }));
                    }
                    if Self::function_needs_materialization(func) {
                        return Err(CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::InvalidAnalogOperator(format!(
                                "analog function '{}': output/inout calls must be lowered at a statement boundary",
                                call.name
                            )),
                            call.span,
                        )));
                    }
                    self.inline_function(&call.name, &call.args, call.span)?
                } else {
                    let mut args = call
                        .args
                        .iter()
                        .map(|a| self.lower_expression(a))
                        .collect::<CompileResult<Vec<_>>>()?;
                    // Materialize the declaration-order-scoped default now.
                    // Hierarchy flattening may later combine modules authored
                    // under different directives into one analyzed module.
                    if is_zi_operator_name(&call.name) && args.len() == 4 {
                        args.push(Self::number_expr(
                            self.current_default_transition,
                            call.span,
                        ));
                    }
                    Expression::Call(CallExpr {
                        name: call.name.clone(),
                        args,
                        span: call.span,
                    })
                }
            }
            Expression::ArrayAccess(a) => {
                let index = self.lower_expression(&a.index)?;
                let array_name = self.resolve_substituted_name(&a.array);
                if self.parameter_arrays.contains(&array_name) {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "indexed access to parameter array '{}' is represented by its declaration metadata, but parameter-array element lowering is not implemented",
                            a.array
                        )),
                        a.span,
                    )));
                }
                let Some(layout) = self.arrays.get(&array_name).cloned() else {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "'{}' is indexed but is not a declared array variable",
                            a.array
                        )),
                        a.span,
                    )));
                };
                // Indexes that fold to instance-invariant constants (literals,
                // unrolled loop variables) resolve straight to the element
                // variable; everything else stays a runtime indexed access
                if let Some(k) = self.eval_const_invariant(&index) {
                    let k = k.round() as i64;
                    self.check_array_bounds(&array_name, &layout, k, a.span)?;
                    Expression::Identifier(Identifier {
                        name: SmolStr::from(format!("{array_name}[{k}]")),
                        span: a.span,
                    })
                } else {
                    Expression::ArrayAccess(ArrayAccessExpr {
                        array: array_name,
                        index: Box::new(index),
                        span: a.span,
                    })
                }
            }
            Expression::ArrayLiteral(a) => {
                if let Some(replication) = a.first_replication() {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(
                            "replication in executable expressions is parsed but not yet supported; write the elements explicitly"
                                .into(),
                        ),
                        replication.span,
                    )));
                }
                let elements = a
                    .elements
                    .iter()
                    .map(|element| {
                        let ArrayLiteralElement::Value(expression) = element else {
                            unreachable!("replication was rejected before expression lowering");
                        };
                        self.lower_expression(expression)
                            .map(ArrayLiteralElement::Value)
                    })
                    .collect::<CompileResult<Vec<_>>>()?;
                Expression::ArrayLiteral(ArrayLiteralExpr {
                    elements,
                    assignment_pattern: a.assignment_pattern,
                    span: a.span,
                })
            }
            Expression::AnalogOperator(operator)
                if stateful_public_analog_operator_name(operator).is_some() =>
            {
                let operator_name = stateful_public_analog_operator_name(operator)
                    .expect("guard requires a classified stateful analog operator");
                self.validate_stateful_analog_operator_placement(operator_name, operator.span())?;
                self.lower_stateful_public_analog_operator(operator)?
            }
            Expression::AnalogOperator(_) | Expression::NoiseSource(_) => expr.clone(),
        })
    }

    fn validate_stateful_analog_operator_placement(
        &self,
        operator: &str,
        span: Span,
    ) -> CompileResult<()> {
        let context = match (
            self.inline_depth != 0,
            self.dynamic_analog_operator_guard_depth != 0,
        ) {
            (false, false) => return Ok(()),
            (true, true) => "inside a user-defined analog function under runtime control flow",
            (true, false) => "inside a user-defined analog function",
            (false, true) => "under runtime control flow",
        };
        Err(CompileError::Semantic(SemanticError::new(
            SemanticErrorKind::InvalidAnalogOperator(format!(
                "'{operator}' analog operator must be evaluated on every Newton iteration and cannot appear {context} (VAMS-2023 section 4.5.15)"
            )),
            span,
        )))
    }

    /// Lower all operands of the public typed stateful representation. Parsed
    /// source normally reaches this pass as [`Expression::Call`], but API
    /// callers may submit these nodes directly; they must receive the same
    /// substitution, access validation, and nested-placement checks.
    fn lower_stateful_public_analog_operator(
        &mut self,
        operator: &AnalogOperator,
    ) -> CompileResult<Expression> {
        let lower_optional = |this: &mut Self, value: &Option<Box<Expression>>| {
            value
                .as_ref()
                .map(|value| this.lower_expression(value).map(Box::new))
                .transpose()
        };
        Ok(Expression::AnalogOperator(match operator {
            AnalogOperator::Ddt { expr, abstol, span } => AnalogOperator::Ddt {
                expr: Box::new(self.lower_expression(expr)?),
                abstol: lower_optional(self, abstol)?,
                span: *span,
            },
            AnalogOperator::Idt {
                expr,
                ic,
                assert_val,
                abstol,
                span,
            } => AnalogOperator::Idt {
                expr: Box::new(self.lower_expression(expr)?),
                ic: lower_optional(self, ic)?,
                assert_val: lower_optional(self, assert_val)?,
                abstol: lower_optional(self, abstol)?,
                span: *span,
            },
            AnalogOperator::IdtMod {
                expr,
                ic,
                modulus,
                offset,
                abstol,
                span,
            } => AnalogOperator::IdtMod {
                expr: Box::new(self.lower_expression(expr)?),
                ic: lower_optional(self, ic)?,
                modulus: lower_optional(self, modulus)?,
                offset: lower_optional(self, offset)?,
                abstol: lower_optional(self, abstol)?,
                span: *span,
            },
            AnalogOperator::Absdelay {
                expr,
                delay,
                max_delay,
                span,
            } => AnalogOperator::Absdelay {
                expr: Box::new(self.lower_expression(expr)?),
                delay: Box::new(self.lower_expression(delay)?),
                max_delay: lower_optional(self, max_delay)?,
                span: *span,
            },
            AnalogOperator::Transition {
                expr,
                delay,
                rise,
                fall,
                tolerance,
                span,
            } => AnalogOperator::Transition {
                expr: Box::new(self.lower_expression(expr)?),
                delay: lower_optional(self, delay)?,
                rise: lower_optional(self, rise)?,
                fall: lower_optional(self, fall)?,
                tolerance: lower_optional(self, tolerance)?,
                span: *span,
            },
            AnalogOperator::Slew {
                expr,
                max_rise,
                max_fall,
                span,
            } => AnalogOperator::Slew {
                expr: Box::new(self.lower_expression(expr)?),
                max_rise: lower_optional(self, max_rise)?,
                max_fall: lower_optional(self, max_fall)?,
                span: *span,
            },
            AnalogOperator::LastCrossing { expr, edge, span } => AnalogOperator::LastCrossing {
                expr: Box::new(self.lower_expression(expr)?),
                edge: *edge,
                span: *span,
            },
            AnalogOperator::Laplace { kind, expr, span } => AnalogOperator::Laplace {
                kind: self.lower_public_laplace_kind(kind)?,
                expr: Box::new(self.lower_expression(expr)?),
                span: *span,
            },
            AnalogOperator::Zi {
                kind,
                expr,
                period,
                transition,
                first_transition,
                span,
            } => {
                self.validate_public_zi_operand_budget(kind, *span)?;
                AnalogOperator::Zi {
                    kind: self.lower_public_zi_kind(kind)?,
                    expr: Box::new(self.lower_expression(expr)?),
                    period: Box::new(self.lower_expression(period)?),
                    transition: Some(Box::new(match transition {
                        Some(transition) => self.lower_expression(transition)?,
                        None => Self::number_expr(self.current_default_transition, *span),
                    })),
                    first_transition: lower_optional(self, first_transition)?,
                    span: *span,
                }
            }
            AnalogOperator::Limit { .. }
            | AnalogOperator::LimiterArgument { .. }
            | AnalogOperator::Ddx { .. }
            | AnalogOperator::Limexp { .. } => {
                unreachable!("only classified stateful operators enter this lowering path")
            }
        }))
    }

    fn lower_public_laplace_kind(&mut self, kind: &LaplaceKind) -> CompileResult<LaplaceKind> {
        let lower = |this: &mut Self, values: &[Expression]| {
            values
                .iter()
                .map(|value| this.lower_expression(value))
                .collect::<CompileResult<Vec<_>>>()
        };
        Ok(match kind {
            LaplaceKind::ZeroPole { zeros, poles } => LaplaceKind::ZeroPole {
                zeros: lower(self, zeros)?,
                poles: lower(self, poles)?,
            },
            LaplaceKind::ZeroDenominator { zeros, denominator } => LaplaceKind::ZeroDenominator {
                zeros: lower(self, zeros)?,
                denominator: lower(self, denominator)?,
            },
            LaplaceKind::NumeratorPole { numerator, poles } => LaplaceKind::NumeratorPole {
                numerator: lower(self, numerator)?,
                poles: lower(self, poles)?,
            },
            LaplaceKind::NumeratorDenominator {
                numerator,
                denominator,
            } => LaplaceKind::NumeratorDenominator {
                numerator: lower(self, numerator)?,
                denominator: lower(self, denominator)?,
            },
        })
    }

    fn lower_public_zi_kind(&mut self, kind: &ZiKind) -> CompileResult<ZiKind> {
        let lower = |this: &mut Self, values: &[Expression]| {
            values
                .iter()
                .map(|value| this.lower_expression(value))
                .collect::<CompileResult<Vec<_>>>()
        };
        Ok(match kind {
            ZiKind::ZeroPole { zeros, poles } => ZiKind::ZeroPole {
                zeros: lower(self, zeros)?,
                poles: lower(self, poles)?,
            },
            ZiKind::ZeroDenominator { zeros, denominator } => ZiKind::ZeroDenominator {
                zeros: lower(self, zeros)?,
                denominator: lower(self, denominator)?,
            },
            ZiKind::NumeratorPole { numerator, poles } => ZiKind::NumeratorPole {
                numerator: lower(self, numerator)?,
                poles: lower(self, poles)?,
            },
            ZiKind::NumeratorDenominator {
                numerator,
                denominator,
            } => ZiKind::NumeratorDenominator {
                numerator: lower(self, numerator)?,
                denominator: lower(self, denominator)?,
            },
        })
    }

    fn validate_zi_definition_purity(&self, call: &CallExpr) -> CompileResult<()> {
        for (index, argument) in call.args.iter().enumerate() {
            if matches!(index, 1 | 2 | 3 | 5) {
                Self::validate_zi_freeze_expression(argument, &call.name, index)?;
            }
        }
        Ok(())
    }

    fn validate_zi_operand_budget(&self, call: &CallExpr) -> CompileResult<()> {
        let scalar_count = |expression: &Expression| match expression {
            Expression::NullArgument(_) => 0,
            Expression::ArrayLiteral(array) => array.elements.len(),
            _ => 1,
        };
        self.validate_zi_scalar_budget(
            &call.name,
            scalar_count(&call.args[1]),
            scalar_count(&call.args[2]),
            call.span,
        )
    }

    fn validate_public_zi_operand_budget(&self, kind: &ZiKind, span: Span) -> CompileResult<()> {
        let (operator, numerator, denominator) = match kind {
            ZiKind::ZeroPole { zeros, poles } => ("zi_zp", zeros.len(), poles.len()),
            ZiKind::ZeroDenominator { zeros, denominator } => {
                ("zi_zd", zeros.len(), denominator.len())
            }
            ZiKind::NumeratorPole { numerator, poles } => ("zi_np", numerator.len(), poles.len()),
            ZiKind::NumeratorDenominator {
                numerator,
                denominator,
            } => ("zi_nd", numerator.len(), denominator.len()),
        };
        self.validate_zi_scalar_budget(operator, numerator, denominator, span)
    }

    fn validate_zi_scalar_budget(
        &self,
        operator: &str,
        numerator: usize,
        denominator: usize,
        span: Span,
    ) -> CompileResult<()> {
        crate::zfilter::validate_zi_runtime_operand_budget(operator, numerator, denominator)
            .map(|_| ())
            .map_err(|error| {
                CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidAnalogOperator(error.to_string()),
                    span,
                ))
            })
    }

    fn validate_zi_freeze_expression(
        expression: &Expression,
        operator: &str,
        argument_index: usize,
    ) -> CompileResult<()> {
        let reject = |detail: String, span: Span| {
            Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(format!(
                    "{operator} constant argument {} must be side-effect-free and deterministic for analysis-start freezing: {detail}",
                    argument_index + 1
                )),
                span,
            )))
        };
        match expression {
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::NullArgument(_)
            | Expression::Identifier(_)
            | Expression::BranchAccess(_) => Ok(()),
            Expression::Binary(binary) => {
                Self::validate_zi_freeze_expression(&binary.left, operator, argument_index)?;
                Self::validate_zi_freeze_expression(&binary.right, operator, argument_index)
            }
            Expression::Unary(unary) => {
                Self::validate_zi_freeze_expression(&unary.operand, operator, argument_index)
            }
            Expression::Conditional(conditional) => {
                Self::validate_zi_freeze_expression(
                    &conditional.condition,
                    operator,
                    argument_index,
                )?;
                Self::validate_zi_freeze_expression(
                    &conditional.then_expr,
                    operator,
                    argument_index,
                )?;
                Self::validate_zi_freeze_expression(
                    &conditional.else_expr,
                    operator,
                    argument_index,
                )
            }
            Expression::ArrayAccess(access) => {
                Self::validate_zi_freeze_expression(&access.index, operator, argument_index)
            }
            Expression::ArrayLiteral(array) => {
                for element in &array.elements {
                    Self::validate_zi_freeze_array_element(element, operator, argument_index)?;
                }
                Ok(())
            }
            Expression::SystemFunction(function) => {
                let normalized = function.name.to_ascii_lowercase();
                if normalized.contains("random")
                    || normalized.starts_with("$dist_")
                    || normalized.starts_with("$rdist_")
                    || normalized == "$limit"
                {
                    return reject(
                        format!(
                            "system function '{}' is stateful or nondeterministic",
                            function.name
                        ),
                        function.span,
                    );
                }
                for argument in &function.args {
                    Self::validate_zi_freeze_expression(argument, operator, argument_index)?;
                }
                Ok(())
            }
            Expression::Call(nested) => {
                let normalized = nested.name.to_ascii_lowercase();
                if matches!(
                    normalized.as_str(),
                    "ddt"
                        | "idt"
                        | "idtmod"
                        | "transition"
                        | "slew"
                        | "absdelay"
                        | "cross"
                        | "last_crossing"
                        | "above"
                        | "timer"
                        | "laplace_zp"
                        | "laplace_zd"
                        | "laplace_np"
                        | "laplace_nd"
                        | "zi_zp"
                        | "zi_zd"
                        | "zi_np"
                        | "zi_nd"
                        | "white_noise"
                        | "flicker_noise"
                        | "noise_table"
                ) {
                    return reject(
                        format!(
                            "nested operator '{}' is stateful or nondeterministic",
                            nested.name
                        ),
                        nested.span,
                    );
                }
                for argument in &nested.args {
                    Self::validate_zi_freeze_expression(argument, operator, argument_index)?;
                }
                Ok(())
            }
            Expression::AnalogOperator(AnalogOperator::Limexp { expr, .. }) => {
                Self::validate_zi_freeze_expression(expr, operator, argument_index)
            }
            Expression::AnalogOperator(analog) => reject(
                format!("nested analog operator at {:?} is stateful", analog.span()),
                analog.span(),
            ),
            Expression::NoiseSource(noise) => reject(
                "a noise source is nondeterministic in this context".into(),
                noise.span(),
            ),
        }
    }

    fn validate_zi_freeze_array_element(
        element: &ArrayLiteralElement,
        operator: &str,
        argument_index: usize,
    ) -> CompileResult<()> {
        match element {
            ArrayLiteralElement::Value(expression) => {
                Self::validate_zi_freeze_expression(expression, operator, argument_index)
            }
            ArrayLiteralElement::Replication(replication) => {
                Self::validate_zi_freeze_expression(&replication.count, operator, argument_index)?;
                for element in &replication.elements {
                    Self::validate_zi_freeze_array_element(element, operator, argument_index)?;
                }
                Ok(())
            }
        }
    }

    /// Validate the selector and callable contract of named `$limit` forms.
    ///
    /// Numeric `$limit(value)` and `$limit(value, step)` calls intentionally
    /// remain outside this check. A named limiter receives the proposed and
    /// previous values implicitly, followed by every argument after its
    /// selector in the `$limit` call.
    fn validate_limit_call(&self, function: &SystemFunction) -> CompileResult<()> {
        if function.name != "$limit" {
            return Ok(());
        }

        let selector = match function.args.get(1) {
            Some(Expression::StringLit(selector)) => selector.value.as_str(),
            _ if function.args.len() <= 2 => return Ok(()),
            _ => {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidAnalogOperator(
                        "named $limit requires a literal string selector as its second argument"
                            .into(),
                    ),
                    function.span,
                )));
            }
        };

        // Xyce's named limiter ABI has untyped and typed selector families.
        // Typed built-ins carry one additional type/polarity argument; dummy
        // selectors retain the same shape for initialization bookkeeping even
        // though they intentionally leave the proposed value unchanged.
        let expected_total = match selector {
            "pnjlim" | "pnjlim_new" | "dummy" => Some(4),
            "typedpnjlim" | "typedpnjlim_new" | "typeddummy" => Some(5),
            _ => None,
        };
        if let Some(expected) = expected_total {
            if function.args.len() != expected {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::ArgumentCountMismatch {
                        name: format!("$limit(\"{selector}\")"),
                        expected: expected.to_string(),
                        got: function.args.len(),
                    },
                    function.span,
                )));
            }
            return Ok(());
        }

        let Some(limiter) = self.user_functions.get(selector) else {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UnknownFunction(selector.to_string()),
                function.span,
            )));
        };

        if limiter.return_type != VarType::Real {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(format!(
                    "named $limit function '{selector}' must return real"
                )),
                function.span,
            )));
        }

        // The source function's first two inputs receive the proposed and
        // previous values. In Xyce's typed custom-limiter convention, the
        // literal `"typed"` and the following type/polarity expression are
        // metadata and are not forwarded to the analog function.
        let typed_custom = function.args.get(2).is_some_and(|argument| {
            matches!(
                argument,
                Expression::StringLit(marker) if marker.value == "typed"
            )
        });
        if typed_custom && function.args.len() < 4 {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(format!(
                    "typed named $limit function '{selector}' requires a type/polarity metadata argument after \"typed\""
                )),
                function.span,
            )));
        }
        let expected_formals = if typed_custom {
            function.args.len() - 2
        } else {
            function.args.len()
        };
        if limiter.params.len() != expected_formals {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::ArgumentCountMismatch {
                    name: selector.to_string(),
                    expected: expected_formals.to_string(),
                    got: limiter.params.len(),
                },
                function.span,
            )));
        }
        if let Some(param) = limiter
            .params
            .iter()
            .find(|param| param.direction != ParamDirection::Input)
        {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(format!(
                    "named $limit function '{selector}' requires input formal '{}', found {}",
                    param.name,
                    match param.direction {
                        ParamDirection::Input => unreachable!(),
                        ParamDirection::Output => "output",
                        ParamDirection::Inout => "inout",
                    }
                )),
                function.span,
            )));
        }
        if let Some(param) = limiter
            .params
            .iter()
            .find(|param| param.param_type != VarType::Real)
        {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(format!(
                    "named $limit function '{selector}' requires real formal '{}', found {:?}",
                    param.name, param.param_type
                )),
                function.span,
            )));
        }

        Ok(())
    }

    /// Lower a user-defined named `$limit` into an explicit stateful operator.
    ///
    /// Xyce supplies the first two analog-function inputs implicitly: the
    /// (possibly polarity-oriented) proposed value and the previous Newton
    /// iterate's limited value. Typed custom limiters encode the literal
    /// `"typed"` and the following polarity expression as call metadata; those
    /// two arguments are deliberately not forwarded to the source function.
    fn lower_custom_limit_call(
        &mut self,
        function: &SystemFunction,
    ) -> CompileResult<Option<Expression>> {
        if function.name != "$limit" {
            return Ok(None);
        }
        let Some(Expression::StringLit(selector_literal)) = function.args.get(1) else {
            return Ok(None);
        };
        let selector = selector_literal.value.clone();
        if matches!(
            selector.as_str(),
            "pnjlim" | "pnjlim_new" | "dummy" | "typedpnjlim" | "typedpnjlim_new" | "typeddummy"
        ) {
            return Ok(None);
        }
        if !self.user_functions.contains_key(&selector) {
            // Validation reports the authoritative unknown-function error.
            return Ok(None);
        }

        let typed = function.args.get(2).is_some_and(
            |argument| matches!(argument, Expression::StringLit(marker) if marker.value == "typed"),
        );
        let proposed = self.lower_expression(
            function
                .args
                .first()
                .expect("validated custom $limit has a proposed value"),
        )?;
        let type_metadata = typed
            .then(|| self.lower_expression(&function.args[3]))
            .transpose()?
            .map(Box::new);
        let implicit_proposed = Expression::AnalogOperator(AnalogOperator::LimiterArgument {
            argument: LimiterArgument::Proposed,
            span: function.span,
        });
        let implicit_previous = Expression::AnalogOperator(AnalogOperator::LimiterArgument {
            argument: LimiterArgument::Previous,
            span: function.span,
        });
        let forwarded_start = if typed { 4 } else { 2 };
        let mut limiter_args = Vec::with_capacity(2 + function.args.len() - forwarded_start);
        limiter_args.extend([implicit_proposed, implicit_previous]);
        limiter_args.extend(function.args[forwarded_start..].iter().cloned());
        let candidate = self.inline_function(&selector, &limiter_args, function.span)?;

        Ok(Some(Expression::AnalogOperator(AnalogOperator::Limit {
            proposed: Box::new(proposed),
            candidate: Box::new(candidate),
            type_metadata,
            selector,
            span: function.span,
        })))
    }

    fn validate_builtin_call_arity(&self, call: &CallExpr) -> CompileResult<()> {
        let Some(signature) = self.functions.get(&call.name) else {
            return Ok(());
        };

        let min_args = signature.min_args();
        let max_args = signature.max_args();
        let got = call.args.len();
        if got < min_args || got > max_args {
            let expected = if min_args == max_args {
                min_args.to_string()
            } else {
                format!("{min_args}..{max_args}")
            };
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::ArgumentCountMismatch {
                    name: call.name.to_string(),
                    expected,
                    got,
                },
                call.span,
            )));
        }

        Ok(())
    }

    fn validate_null_arguments(&self, call: &CallExpr) -> CompileResult<()> {
        let normalized = call.name.to_ascii_lowercase();
        for (index, argument) in call.args.iter().enumerate() {
            if !matches!(argument, Expression::NullArgument(_)) {
                continue;
            }
            let authorized = index == 1
                && matches!(
                    normalized.as_str(),
                    "zi_zp" | "zi_zd" | "laplace_zp" | "laplace_zd"
                );
            if !authorized {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidAnalogOperator(format!(
                        "{} argument {} may not be null; only the zeros operand of zi_zp/zi_zd and laplace_zp/laplace_zd authorizes an adjacent-comma null",
                        call.name,
                        index + 1
                    )),
                    argument.span(),
                )));
            }
        }
        Ok(())
    }

    /// Enforce the Verilog-AMS array-value grammar used by the Laplace and Zi
    /// operators. An ordinary brace concatenation is a packed value, not an
    /// unpacked coefficient/root vector, and a bare scalar cannot be silently
    /// promoted to a one-element vector.
    fn validate_filter_vector_operands(&self, call: &CallExpr) -> CompileResult<()> {
        let normalized = call.name.to_ascii_lowercase();
        let roles = match normalized.as_str() {
            "laplace_zp" | "zi_zp" => ("zeros", "poles", true),
            "laplace_zd" | "zi_zd" => ("zeros", "denominator", true),
            "laplace_np" | "zi_np" => ("numerator", "poles", false),
            "laplace_nd" | "zi_nd" => ("numerator", "denominator", false),
            _ => return Ok(()),
        };

        for (index, role, allow_null) in [(1, roles.0, roles.2), (2, roles.1, false)] {
            let Some(argument) = call.args.get(index) else {
                // The arity validator owns the missing-argument diagnostic.
                continue;
            };
            match argument {
                Expression::NullArgument(_) if allow_null => {}
                Expression::ArrayLiteral(array) if array.assignment_pattern => {}
                Expression::ArrayLiteral(array) => {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::InvalidAnalogOperator(format!(
                            "{} {role} vector must be an assignment pattern opened with `'{{` or an array identifier; ordinary concatenation `{{...}}` is not a Verilog-AMS array value",
                            call.name
                        )),
                        array.span,
                    )));
                }
                Expression::Identifier(identifier)
                    if self.parameter_arrays.contains(&identifier.name)
                        || self.arrays.contains_key(&identifier.name) =>
                {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "{} {role} array identifier '{}' is valid Verilog-AMS syntax, but executable filter array operands are not implemented yet",
                            call.name, identifier.name
                        )),
                        identifier.span,
                    )));
                }
                other => {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::InvalidAnalogOperator(format!(
                            "{} {role} operand must be an assignment pattern opened with `'{{` or an array identifier; a scalar expression is not a filter vector",
                            call.name
                        )),
                        other.span(),
                    )));
                }
            }
        }
        Ok(())
    }

    fn materialize_filter_call_replication(&self, call: &CallExpr) -> CompileResult<CallExpr> {
        let normalized = call.name.to_ascii_lowercase();
        let roles = match normalized.as_str() {
            "laplace_zp" | "zi_zp" => ("zeros", "poles"),
            "laplace_zd" | "zi_zd" => ("zeros", "denominator"),
            "laplace_np" | "zi_np" => ("numerator", "poles"),
            "laplace_nd" | "zi_nd" => ("numerator", "denominator"),
            _ => return Ok(call.clone()),
        };

        let mut materialized = call.clone();
        for (index, role) in [(1, roles.0), (2, roles.1)] {
            let Some(argument) = materialized.args.get_mut(index) else {
                continue;
            };
            if !matches!(argument, Expression::ArrayLiteral(_)) {
                continue;
            }
            *argument = self.materialize_replication_expression(
                argument,
                MAX_ANALOG_FILTER_VECTOR_ELEMENTS,
                MAX_ANALOG_FILTER_VECTOR_ELEMENTS.saturating_mul(4),
                &format!("{} {role} vector", call.name),
                true,
            )?;
        }
        Ok(materialized)
    }

    const MAX_INLINE_DEPTH: usize = 16;

    /// Inline a call to a user-defined analog function by symbolically
    /// executing its body. The return value is the final expression bound
    /// to the function-name variable.
    ///
    /// # This is guard flattening, and it is the backend rebuild's last one
    ///
    /// A function body's control flow is dissolved here exactly as the analog
    /// block's used to be: each assignment under a guard becomes
    /// `guard ? value : previous`, where `previous` is the *entire* expression
    /// tree built so far. A chain of `n` assignments under nested conditions
    /// therefore duplicates its predecessor at every step, and the tree grows
    /// multiplicatively rather than additively.
    ///
    /// Measured on `EPFL_HEMT_10a`: its analog block holds **6 conditionals and
    /// 191 assignments**, and the front end turns them into **186,444 HIR
    /// expressions** — roughly a thousand nodes per assignment — because its
    /// `core` function nests three arms over five chained locals and is called
    /// from several places. Lowering those `?:` trees produces 8,248 CFG blocks
    /// and 6.3 MB of Rust against 78 KB from the tier being replaced. It is the
    /// single largest obstacle to the plan's size gate.
    ///
    /// The fix is the one Phase 1 applied a level up: materialise a call whose
    /// body has control flow at a statement boundary and inline the body as
    /// statements, so one `if` in a function becomes one
    /// [`AnalyzedRegion::Conditional`] rather than a duplicated expression. The
    /// machinery for statement-boundary materialisation already exists here for
    /// output/inout arguments. See `design/VERILOGA_BACKEND_PLAN.md`.
    fn inline_function(
        &mut self,
        name: &SmolStr,
        args: &[Expression],
        span: Span,
    ) -> CompileResult<Expression> {
        if self.inline_depth >= Self::MAX_INLINE_DEPTH {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::CircularDependency(format!(
                    "analog function '{}' (recursive call chain?)",
                    name
                )),
                span,
            )));
        }

        let func = self
            .user_functions
            .get(name)
            .cloned()
            .expect("checked by caller");

        if args.len() != func.params.len() {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::ArgumentCountMismatch {
                    name: name.to_string(),
                    expected: func.params.len().to_string(),
                    got: args.len(),
                },
                span,
            )));
        }

        // Bind parameters and locals in a fresh substitution frame
        let mut frame = HashMap::new();
        let mut output_bindings = Vec::new();
        for (param, arg) in func.params.iter().zip(args.iter()) {
            match param.direction {
                ParamDirection::Input => {
                    frame.insert(param.name.clone(), self.lower_expression(arg)?);
                }
                ParamDirection::Output => {
                    let target = self.function_output_lvalue(name, param, arg)?;
                    frame.insert(param.name.clone(), Self::number_expr(0.0, param.span));
                    output_bindings.push((param.name.clone(), target, param.span));
                }
                ParamDirection::Inout => {
                    let target = self.function_output_lvalue(name, param, arg)?;
                    frame.insert(param.name.clone(), self.lower_expression(arg)?);
                    output_bindings.push((param.name.clone(), target, param.span));
                }
            }
        }
        for var_decl in &func.locals {
            for item in &var_decl.items {
                if func.params.iter().any(|p| p.name == item.name) {
                    continue;
                }
                if !item.dimensions.is_empty() {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "array local '{}' in analog function '{}'",
                            item.name, name
                        )),
                        item.span,
                    )));
                }
                let init = match &item.init {
                    Some(init) => init.clone(),
                    None => Self::number_expr(0.0, item.span),
                };
                frame.insert(item.name.clone(), init);
            }
        }
        // The return value accumulates in a variable named after the function
        frame.insert(func.name.clone(), Self::number_expr(0.0, span));

        self.subst_stack.push(frame);
        self.inline_depth += 1;
        let result = self.exec_function_body(&func.body.statements, None);
        self.inline_depth -= 1;
        let frame = self.subst_stack.pop().expect("pushed above");
        result?;

        for (formal, target, span) in output_bindings {
            let value = frame
                .get(&formal)
                .cloned()
                .unwrap_or_else(|| Self::number_expr(0.0, span));
            self.function_side_effects.push(AssignmentStmt {
                target,
                value,
                span,
            });
        }

        Ok(frame
            .get(&func.name)
            .cloned()
            .unwrap_or_else(|| Self::number_expr(0.0, span)))
    }

    fn function_output_lvalue(
        &mut self,
        function_name: &SmolStr,
        param: &FunctionParam,
        arg: &Expression,
    ) -> CompileResult<LValue> {
        match arg {
            Expression::Identifier(id) => {
                let resolved = self.resolve_substituted_name(&id.name);
                Ok(LValue::Variable {
                    name: resolved,
                    span: id.span,
                })
            }
            Expression::ArrayAccess(access) => {
                let array = self.resolve_substituted_name(&access.array);
                let index = self.lower_expression_without_side_effects(
                    &access.index,
                    "analog function output argument index",
                )?;
                Ok(LValue::ArrayAccess {
                    name: array,
                    index: Box::new(index),
                    span: access.span,
                })
            }
            _ => Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(format!(
                    "analog function '{}': {} argument '{}' must be an assignable variable",
                    function_name,
                    match param.direction {
                        ParamDirection::Output => "output",
                        ParamDirection::Inout => "inout",
                        ParamDirection::Input => "input",
                    },
                    param.name
                )),
                arg.span(),
            ))),
        }
    }

    /// Symbolically execute function-body statements, updating the topmost
    /// substitution frame.
    fn exec_function_body(
        &mut self,
        statements: &[AnalogStatement],
        guard: Option<&Expression>,
    ) -> CompileResult<()> {
        for stmt in statements {
            self.exec_function_statement(stmt, guard)?;
        }
        Ok(())
    }

    fn exec_function_statement(
        &mut self,
        stmt: &AnalogStatement,
        guard: Option<&Expression>,
    ) -> CompileResult<()> {
        match stmt {
            AnalogStatement::Assignment(assign) => {
                let LValue::Variable { name, span } = &assign.target else {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::InvalidAnalogOperator(
                            "array assignment inside analog functions is not supported".into(),
                        ),
                        assign.span,
                    )));
                };
                let value = self
                    .lower_expression_without_side_effects(&assign.value, "analog function body")?;
                let prev = self
                    .lookup_substitution(name)
                    .unwrap_or_else(|| Self::number_expr(0.0, *span));
                let new_value = match guard {
                    Some(g) => Expression::Conditional(ConditionalExpr {
                        condition: Box::new(g.clone()),
                        then_expr: Box::new(value),
                        else_expr: Box::new(prev),
                        span: *span,
                    }),
                    None => value,
                };
                self.subst_stack
                    .last_mut()
                    .expect("function frame")
                    .insert(name.clone(), new_value);
            }
            AnalogStatement::Conditional(cond) => {
                let condition = self.lower_expression_without_side_effects(
                    &cond.condition,
                    "analog function body",
                )?;
                let dynamic_condition = !self.expression_is_simulation_invariant(&condition);
                let then_guard = match guard {
                    Some(g) => Self::binary_expr(BinaryOp::And, g.clone(), condition.clone()),
                    None => condition.clone(),
                };
                if dynamic_condition {
                    self.dynamic_analog_operator_guard_depth += 1;
                }
                let then_result =
                    self.exec_function_statement(&cond.then_branch, Some(&then_guard));
                if dynamic_condition {
                    self.dynamic_analog_operator_guard_depth -= 1;
                }
                then_result?;
                if let Some(else_branch) = &cond.else_branch {
                    let not_cond = Self::not_expr(condition);
                    let else_guard = match guard {
                        Some(g) => Self::binary_expr(BinaryOp::And, g.clone(), not_cond),
                        None => not_cond,
                    };
                    if dynamic_condition {
                        self.dynamic_analog_operator_guard_depth += 1;
                    }
                    let else_result = self.exec_function_statement(else_branch, Some(&else_guard));
                    if dynamic_condition {
                        self.dynamic_analog_operator_guard_depth -= 1;
                    }
                    else_result?;
                }
            }
            AnalogStatement::Case(case_stmt) => {
                let selector = self.lower_expression_without_side_effects(
                    &case_stmt.expr,
                    "analog function body",
                )?;
                let mut prior_match: Option<Expression> = None;
                for item in &case_stmt.items {
                    let mut item_match: Option<Expression> = None;
                    for m in &item.matches {
                        let m_lowered =
                            self.lower_expression_without_side_effects(m, "analog function body")?;
                        let eq = Self::binary_expr(BinaryOp::Eq, selector.clone(), m_lowered);
                        item_match = Some(match item_match {
                            Some(acc) => Self::binary_expr(BinaryOp::Or, acc, eq),
                            None => eq,
                        });
                    }
                    let Some(item_match) = item_match else {
                        continue;
                    };
                    let mut item_guard = match &prior_match {
                        Some(prior) => Self::binary_expr(
                            BinaryOp::And,
                            item_match.clone(),
                            Self::not_expr(prior.clone()),
                        ),
                        None => item_match.clone(),
                    };
                    if let Some(g) = guard {
                        item_guard = Self::binary_expr(BinaryOp::And, g.clone(), item_guard);
                    }
                    self.exec_function_statement(&item.statement, Some(&item_guard))?;
                    prior_match = Some(match prior_match {
                        Some(prior) => Self::binary_expr(BinaryOp::Or, prior, item_match),
                        None => item_match,
                    });
                }
                if let Some(default) = &case_stmt.default {
                    let default_guard = match (guard, prior_match) {
                        (Some(g), Some(prior)) => Some(Self::binary_expr(
                            BinaryOp::And,
                            g.clone(),
                            Self::not_expr(prior),
                        )),
                        (None, Some(prior)) => Some(Self::not_expr(prior)),
                        (Some(g), None) => Some(g.clone()),
                        (None, None) => None,
                    };
                    self.exec_function_statement(default, default_guard.as_ref())?;
                }
            }
            AnalogStatement::Block(block) => {
                // Function-internal blocks share the function frame; local
                // declarations bind into it
                for var_decl in &block.variables {
                    for item in &var_decl.items {
                        if !item.dimensions.is_empty() {
                            return Err(CompileError::Semantic(SemanticError::new(
                                SemanticErrorKind::UnsupportedFeature(format!(
                                    "array local '{}' in analog function",
                                    item.name
                                )),
                                item.span,
                            )));
                        }
                        let init = match &item.init {
                            Some(init) => self.lower_expression_without_side_effects(
                                init,
                                "analog function body",
                            )?,
                            None => Self::number_expr(0.0, item.span),
                        };
                        self.subst_stack
                            .last_mut()
                            .expect("function frame")
                            .insert(item.name.clone(), init);
                    }
                }
                self.exec_function_body(&block.statements, guard)?;
            }
            AnalogStatement::Null(_) => {}
            AnalogStatement::Call(call) => self.validate_no_effect_system_task(call)?,
            other => {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidAnalogOperator(format!(
                        "statement not supported inside analog functions: {:?}",
                        std::mem::discriminant(other)
                    )),
                    Span::dummy(),
                )));
            }
        }
        Ok(())
    }

    fn is_no_effect_system_task(name: &str) -> bool {
        matches!(
            name,
            "$display"
                | "$error"
                | "$fatal"
                | "$finish"
                | "$info"
                | "$monitor"
                | "$stop"
                | "$strobe"
                | "$warning"
                | "$write"
        )
    }

    /// Report one no-effect system task the analyzer is about to drop.
    ///
    /// The task is legal Verilog-A and parses, but this compiler emits device
    /// equations rather than a simulation program, so the call contributes
    /// nothing. Silence would leave the author believing the model prints.
    fn warn_no_effect_system_task(&mut self, call: &CallStmt) {
        self.warn(
            NO_EFFECT_SYSTEM_TASK_CODE,
            format!(
                "System task '{}' is parsed and discarded: it has no effect on the device equations and writes nothing at run time.",
                call.name
            ),
            call.span,
        );
    }

    fn validate_no_effect_system_task(&mut self, call: &CallStmt) -> CompileResult<()> {
        if Self::is_no_effect_system_task(call.name.as_str()) {
            self.warn_no_effect_system_task(call);
            return Ok(());
        }

        if matches!(call.name.as_str(), "$bound_step" | "$discontinuity") {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(format!(
                    "system task '{}' is not supported inside analog functions",
                    call.name
                )),
                call.span,
            )));
        }

        Err(Self::unknown_system_task_error(call))
    }

    fn validate_system_task_arity(
        &self,
        call: &CallStmt,
        min_args: usize,
        max_args: Option<usize>,
    ) -> CompileResult<()> {
        let got = call.args.len();
        let too_many = max_args.is_some_and(|max| got > max);
        if got >= min_args && !too_many {
            return Ok(());
        }

        let expected = match max_args {
            Some(max) if min_args == max => min_args.to_string(),
            Some(max) => format!("{min_args}..{max}"),
            None => format!("{min_args}+"),
        };
        Err(CompileError::Semantic(SemanticError::new(
            SemanticErrorKind::ArgumentCountMismatch {
                name: call.name.to_string(),
                expected,
                got,
            },
            call.span,
        )))
    }

    fn unknown_system_task_error(call: &CallStmt) -> CompileError {
        CompileError::Semantic(SemanticError::new(
            SemanticErrorKind::UnknownFunction(call.name.to_string()),
            call.span,
        ))
    }

    fn validate_node(&self, name: &str, span: Span) -> CompileResult<()> {
        if is_global_ground_name(name) {
            return Ok(());
        }
        if let Some(sym) = self.symbols.lookup(name) {
            match sym.kind {
                SymbolKind::Port | SymbolKind::Node | SymbolKind::Branch => Ok(()),
                _ => Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidNodeReference {
                        name: name.into(),
                        kind: format!("{:?}", sym.kind),
                    },
                    span,
                ))),
            }
        } else {
            Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UndeclaredSymbol { name: name.into() },
                span,
            )))
        }
    }

    fn infer_type(&self, expr: &Expression) -> CompileResult<ValueType> {
        match expr {
            Expression::Number(number) => Ok(if Self::integer_literal_value(number).is_some() {
                ValueType::Integer
            } else {
                ValueType::Real
            }),
            Expression::StringLit(_) => Ok(ValueType::String),
            Expression::NullArgument(span) => Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidExpression(
                    "a null argument is legal only in the zero-vector position of zi_zp, zi_zd, laplace_zp, or laplace_zd"
                        .into(),
                ),
                *span,
            ))),
            Expression::Identifier(ident) => {
                if self.parameter_arrays.contains(&ident.name) {
                    return Err(CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "parameter array '{}' cannot be used as a scalar expression",
                            ident.name
                        )),
                        ident.span,
                    )));
                }
                if let Some(sym) = self.symbols.lookup(&ident.name) {
                    Ok(sym.value_type)
                } else if let Some((base, _)) = ident.name.split_once('[')
                    && self.arrays.contains_key(base)
                    && let Some(sym) = self.symbols.lookup(base)
                {
                    // Array element slot (`arr[k]`): typed like the array
                    Ok(sym.value_type)
                } else {
                    Ok(ValueType::Unknown)
                }
            }
            Expression::BranchAccess(_) => Ok(ValueType::NatureAccess),
            Expression::SystemFunction(_) => Ok(ValueType::Real),
            Expression::Call(call) => {
                if let Some(sig) = self.functions.get(&call.name) {
                    Ok(sig.return_type)
                } else {
                    Ok(ValueType::Unknown)
                }
            }
            Expression::Unary(unary) => {
                let operand_type = self.infer_type(&unary.operand)?;
                match unary.op {
                    UnaryOp::Pos | UnaryOp::Neg => Ok(operand_type),
                    UnaryOp::Not => Ok(ValueType::Boolean),
                    UnaryOp::BitNot => {
                        Self::validate_integer_operator_operand(
                            operand_type,
                            "operand of bitwise complement",
                            unary.operand.span(),
                        )?;
                        Ok(ValueType::Integer)
                    }
                }
            }
            Expression::Binary(binary) => {
                let left = self.infer_type(&binary.left)?;
                let right = self.infer_type(&binary.right)?;

                match binary.op {
                    BinaryOp::Eq
                    | BinaryOp::Ne
                    | BinaryOp::Lt
                    | BinaryOp::Le
                    | BinaryOp::Gt
                    | BinaryOp::Ge => Ok(ValueType::Boolean),
                    BinaryOp::And | BinaryOp::Or => Ok(ValueType::Boolean),
                    BinaryOp::Add
                    | BinaryOp::Sub
                    | BinaryOp::Mul
                    | BinaryOp::Div
                    | BinaryOp::Pow
                    | BinaryOp::Mod => Ok(left.common_type(right)),
                    BinaryOp::BitAnd
                    | BinaryOp::BitOr
                    | BinaryOp::BitXor
                    | BinaryOp::Shl
                    | BinaryOp::Shr => {
                        Self::validate_integer_operator_operand(
                            left,
                            "left operand of bitwise or shift operator",
                            binary.left.span(),
                        )?;
                        Self::validate_integer_operator_operand(
                            right,
                            "right operand of bitwise or shift operator",
                            binary.right.span(),
                        )?;
                        Ok(ValueType::Integer)
                    }
                }
            }
            Expression::Conditional(cond) => {
                let then_type = self.infer_type(&cond.then_expr)?;
                let else_type = self.infer_type(&cond.else_expr)?;
                Ok(then_type.common_type(else_type))
            }
            Expression::ArrayAccess(a) => {
                if let Some(sym) = self.symbols.lookup(&a.array) {
                    Ok(sym.value_type)
                } else {
                    Ok(ValueType::Unknown)
                }
            }
            Expression::ArrayLiteral(_) => Ok(ValueType::Unknown),
            Expression::AnalogOperator(_) => Ok(ValueType::Real),
            Expression::NoiseSource(_) => Ok(ValueType::Real),
        }
    }

    fn validate_integer_operator_operand(
        operand_type: ValueType,
        context: &str,
        span: Span,
    ) -> CompileResult<()> {
        if matches!(
            operand_type,
            ValueType::Integer | ValueType::Boolean | ValueType::Unknown | ValueType::Error
        ) {
            return Ok(());
        }
        Err(CompileError::Semantic(SemanticError::new(
            SemanticErrorKind::TypeMismatch {
                expected: "integer".into(),
                found: operand_type.to_string(),
                context: context.into(),
            },
            span,
        )))
    }

    fn validate_integer_operator_expression(
        &mut self,
        expression: &Expression,
        context: &str,
    ) -> bool {
        match self.infer_type(expression) {
            Ok(operand_type) => {
                match Self::validate_integer_operator_operand(
                    operand_type,
                    context,
                    expression.span(),
                ) {
                    Ok(()) => true,
                    Err(CompileError::Semantic(error)) => {
                        self.errors.push(error);
                        false
                    }
                    Err(error) => {
                        self.record_error_at(
                            SemanticErrorKind::InvalidExpression(error.to_string()),
                            expression.span(),
                        );
                        false
                    }
                }
            }
            Err(CompileError::Semantic(error)) => {
                self.errors.push(error);
                false
            }
            Err(error) => {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(error.to_string()),
                    expression.span(),
                );
                false
            }
        }
    }

    /// Constant evaluation against parameter defaults. Suitable only for
    /// compile-time diagnostics (range checks on declared defaults):
    /// instances may override parameters.
    fn eval_const(&self, expr: &Expression) -> Option<f64> {
        self.eval_const_value(expr).map(ConstantValue::as_f64)
    }

    fn eval_const_value(&self, expr: &Expression) -> Option<ConstantValue> {
        Self::eval_const_value_with(expr, &self.param_consts)
    }

    /// Whether an expression is fixed for the duration of an analysis.
    /// Model parameters may differ between instances, but each resolved
    /// parameter value is constant while that instance is evaluated, so it is
    /// legal as an analog-operator control expression even though it is not
    /// safe for compile-time code-shape folding.
    fn expression_is_simulation_invariant(&self, expr: &Expression) -> bool {
        if self.eval_const(expr).is_some() {
            return true;
        }
        match expr {
            // Every resolved parameter is fixed on the instance even when its
            // default comes from an elaboration-time service such as
            // `$simparam` and therefore cannot be numerically folded here.
            Expression::Identifier(identifier) => self
                .symbols
                .lookup(&identifier.name)
                .is_some_and(|symbol| symbol.kind == SymbolKind::Parameter),
            Expression::Unary(unary) => self.expression_is_simulation_invariant(&unary.operand),
            Expression::Binary(binary) => {
                self.expression_is_simulation_invariant(&binary.left)
                    && self.expression_is_simulation_invariant(&binary.right)
            }
            Expression::Conditional(conditional) => {
                self.expression_is_simulation_invariant(&conditional.condition)
                    && self.expression_is_simulation_invariant(&conditional.then_expr)
                    && self.expression_is_simulation_invariant(&conditional.else_expr)
            }
            // Connectivity is fixed when an instance is elaborated. Compact
            // models use it together with model selectors to choose which
            // terminal owns a thermal `ddt`; that choice cannot change during
            // Newton or between accepted transient steps.
            Expression::SystemFunction(function)
                if function.name.eq_ignore_ascii_case("$port_connected")
                    && function.args.len() == 1 =>
            {
                matches!(
                    &function.args[0],
                    Expression::Identifier(identifier)
                        if self.symbols.lookup(&identifier.name).is_some_and(|symbol| {
                            matches!(symbol.kind, SymbolKind::Port | SymbolKind::Node)
                        })
                )
            }
            _ => false,
        }
    }

    /// Constant evaluation that only resolves instance-invariant values.
    /// Anything that shapes generated code (loop unrolling, repeat counts)
    /// must use this: folding a parameter's *default* would bake it in and
    /// break per-instance overrides.
    fn eval_const_invariant(&self, expr: &Expression) -> Option<f64> {
        self.eval_const_invariant_value(expr)
            .map(ConstantValue::as_f64)
    }

    fn eval_const_invariant_value(&self, expr: &Expression) -> Option<ConstantValue> {
        Self::eval_const_value_with(expr, &self.invariant_consts)
    }

    fn exact_const_i64(value: f64) -> Option<i64> {
        (value.is_finite()
            && value.fract() == 0.0
            && value >= i64::MIN as f64
            && value < 9_223_372_036_854_775_808.0)
            .then_some(value as i64)
    }

    fn integer_literal_value(number: &NumberLit) -> Option<i64> {
        parse_integer_literal(number.raw.as_str()).ok().flatten()
    }

    fn constant_for_declared_type(
        value: ConstantValue,
        declared_type: ParamType,
    ) -> Option<ConstantValue> {
        match declared_type {
            ParamType::Real => Some(ConstantValue::Real(value.as_f64())),
            ParamType::Integer => Self::exact_const_i64(value.as_f64()).map(ConstantValue::Integer),
            ParamType::String => None,
        }
    }

    fn constant_integer(value: ConstantValue) -> Option<i64> {
        match value {
            ConstantValue::Integer(value) => Some(value),
            ConstantValue::Real(_) => None,
        }
    }

    fn constant_add(left: ConstantValue, right: ConstantValue) -> Option<ConstantValue> {
        match (left, right) {
            (ConstantValue::Integer(left), ConstantValue::Integer(right)) => {
                left.checked_add(right).map(ConstantValue::Integer)
            }
            _ => Some(ConstantValue::Real(left.as_f64() + right.as_f64())),
        }
    }

    fn constant_sub(left: ConstantValue, right: ConstantValue) -> Option<ConstantValue> {
        match (left, right) {
            (ConstantValue::Integer(left), ConstantValue::Integer(right)) => {
                left.checked_sub(right).map(ConstantValue::Integer)
            }
            _ => Some(ConstantValue::Real(left.as_f64() - right.as_f64())),
        }
    }

    fn constant_mul(left: ConstantValue, right: ConstantValue) -> Option<ConstantValue> {
        match (left, right) {
            (ConstantValue::Integer(left), ConstantValue::Integer(right)) => {
                left.checked_mul(right).map(ConstantValue::Integer)
            }
            _ => Some(ConstantValue::Real(left.as_f64() * right.as_f64())),
        }
    }

    fn constant_div(left: ConstantValue, right: ConstantValue) -> Option<ConstantValue> {
        match (left, right) {
            (ConstantValue::Integer(left), ConstantValue::Integer(right)) => {
                left.checked_div(right).map(ConstantValue::Integer)
            }
            _ => Some(ConstantValue::Real(left.as_f64() / right.as_f64())),
        }
    }

    fn constant_mod(left: ConstantValue, right: ConstantValue) -> Option<ConstantValue> {
        match (left, right) {
            (ConstantValue::Integer(left), ConstantValue::Integer(right)) => {
                left.checked_rem(right).map(ConstantValue::Integer)
            }
            _ => Some(ConstantValue::Real(left.as_f64() % right.as_f64())),
        }
    }

    fn constant_pow(left: ConstantValue, right: ConstantValue) -> Option<ConstantValue> {
        match (left, right) {
            (ConstantValue::Integer(left), ConstantValue::Integer(right)) => {
                let exponent = u32::try_from(right).ok()?;
                left.checked_pow(exponent).map(ConstantValue::Integer)
            }
            _ => Some(ConstantValue::Real(left.as_f64().powf(right.as_f64()))),
        }
    }

    fn constant_min(left: ConstantValue, right: ConstantValue) -> ConstantValue {
        if left.is_real() || right.is_real() {
            ConstantValue::Real(left.as_f64().min(right.as_f64()))
        } else {
            ConstantValue::Integer(
                Self::constant_integer(left)
                    .expect("integer checked")
                    .min(Self::constant_integer(right).expect("integer checked")),
            )
        }
    }

    fn constant_max(left: ConstantValue, right: ConstantValue) -> ConstantValue {
        if left.is_real() || right.is_real() {
            ConstantValue::Real(left.as_f64().max(right.as_f64()))
        } else {
            ConstantValue::Integer(
                Self::constant_integer(left)
                    .expect("integer checked")
                    .max(Self::constant_integer(right).expect("integer checked")),
            )
        }
    }

    fn eval_const_value_with(
        expr: &Expression,
        env: &HashMap<SmolStr, ConstantValue>,
    ) -> Option<ConstantValue> {
        let eval = |e: &Expression| Self::eval_const_value_with(e, env);
        match expr {
            Expression::Number(number) => Some(
                Self::integer_literal_value(number)
                    .map(ConstantValue::Integer)
                    .unwrap_or(ConstantValue::Real(number.value)),
            ),
            Expression::Unary(u) => {
                let v = eval(&u.operand)?;
                Some(match u.op {
                    UnaryOp::Neg => match v {
                        ConstantValue::Integer(value) => {
                            ConstantValue::Integer(value.checked_neg()?)
                        }
                        ConstantValue::Real(value) => ConstantValue::Real(-value),
                    },
                    UnaryOp::Pos => v,
                    UnaryOp::Not => ConstantValue::Integer(i64::from(!v.is_truthy())),
                    UnaryOp::BitNot => ConstantValue::Integer(i64::from(
                        !i32::try_from(Self::constant_integer(v)?).ok()?,
                    )),
                })
            }
            Expression::Binary(b) => {
                let l = eval(&b.left)?;
                let r = eval(&b.right)?;
                Some(match b.op {
                    BinaryOp::Add => Self::constant_add(l, r)?,
                    BinaryOp::Sub => Self::constant_sub(l, r)?,
                    BinaryOp::Mul => Self::constant_mul(l, r)?,
                    BinaryOp::Div => Self::constant_div(l, r)?,
                    BinaryOp::Mod => Self::constant_mod(l, r)?,
                    BinaryOp::Pow => Self::constant_pow(l, r)?,
                    BinaryOp::Eq => ConstantValue::Integer(i64::from(l.as_f64() == r.as_f64())),
                    BinaryOp::Ne => ConstantValue::Integer(i64::from(l.as_f64() != r.as_f64())),
                    BinaryOp::Lt => ConstantValue::Integer(i64::from(l.as_f64() < r.as_f64())),
                    BinaryOp::Le => ConstantValue::Integer(i64::from(l.as_f64() <= r.as_f64())),
                    BinaryOp::Gt => ConstantValue::Integer(i64::from(l.as_f64() > r.as_f64())),
                    BinaryOp::Ge => ConstantValue::Integer(i64::from(l.as_f64() >= r.as_f64())),
                    BinaryOp::And => {
                        ConstantValue::Integer(i64::from(l.is_truthy() && r.is_truthy()))
                    }
                    BinaryOp::Or => {
                        ConstantValue::Integer(i64::from(l.is_truthy() || r.is_truthy()))
                    }
                    BinaryOp::Shl => {
                        let value = integer_binary(
                            IntegerBinaryOperation::Shl,
                            Self::constant_integer(l)? as f64,
                            Self::constant_integer(r)? as f64,
                        )
                        .ok()?;
                        ConstantValue::Integer(i64::from(real_to_integer(value).ok()?))
                    }
                    BinaryOp::Shr => {
                        let value = integer_binary(
                            IntegerBinaryOperation::Shr,
                            Self::constant_integer(l)? as f64,
                            Self::constant_integer(r)? as f64,
                        )
                        .ok()?;
                        ConstantValue::Integer(i64::from(real_to_integer(value).ok()?))
                    }
                    BinaryOp::BitAnd | BinaryOp::BitOr | BinaryOp::BitXor => {
                        let operation = match b.op {
                            BinaryOp::BitAnd => IntegerBinaryOperation::BitAnd,
                            BinaryOp::BitOr => IntegerBinaryOperation::BitOr,
                            BinaryOp::BitXor => IntegerBinaryOperation::BitXor,
                            _ => unreachable!("bitwise branch only receives bitwise operators"),
                        };
                        let value = integer_binary(
                            operation,
                            Self::constant_integer(l)? as f64,
                            Self::constant_integer(r)? as f64,
                        )
                        .ok()?;
                        ConstantValue::Integer(i64::from(real_to_integer(value).ok()?))
                    }
                })
            }
            Expression::Conditional(c) => {
                let cond = eval(&c.condition)?;
                if cond.is_truthy() {
                    eval(&c.then_expr)
                } else {
                    eval(&c.else_expr)
                }
            }
            Expression::Call(call) => {
                let args: Option<Vec<ConstantValue>> = call.args.iter().map(eval).collect();
                let args = args?;
                match (call.name.as_str(), args.as_slice()) {
                    ("abs", [ConstantValue::Integer(value)]) => {
                        Some(ConstantValue::Integer(value.checked_abs()?))
                    }
                    ("abs", [value]) => Some(ConstantValue::Real(value.as_f64().abs())),
                    ("sqrt", [value]) => Some(ConstantValue::Real(value.as_f64().sqrt())),
                    ("exp", [value]) => Some(ConstantValue::Real(value.as_f64().exp())),
                    ("ln" | "log", [value]) => Some(ConstantValue::Real(value.as_f64().ln())),
                    ("log10", [value]) => Some(ConstantValue::Real(value.as_f64().log10())),
                    ("floor", [value]) => Some(ConstantValue::Real(value.as_f64().floor())),
                    ("ceil", [value]) => Some(ConstantValue::Real(value.as_f64().ceil())),
                    ("min", [a, b]) => Some(Self::constant_min(*a, *b)),
                    ("max", [a, b]) => Some(Self::constant_max(*a, *b)),
                    ("pow", [a, b]) => Some(ConstantValue::Real(a.as_f64().powf(b.as_f64()))),
                    _ => None,
                }
            }
            Expression::Identifier(ident) => match ident.name.as_str() {
                "inf" => Some(ConstantValue::Real(f64::INFINITY)),
                name => env.get(name).copied(),
            },
            _ => None,
        }
    }

    /// Compatibility entry point for hierarchy elaboration, whose resolved
    /// scalar vector is still represented as `f64`. Literal arithmetic keeps
    /// its source type; resolved identifiers are real until the hierarchy
    /// parameter vector gains explicit type metadata.
    fn eval_const_with(expr: &Expression, env: &HashMap<SmolStr, f64>) -> Option<f64> {
        let typed_env = env
            .iter()
            .map(|(name, value)| (name.clone(), ConstantValue::Real(*value)))
            .collect();
        Self::eval_const_value_with(expr, &typed_env).map(ConstantValue::as_f64)
    }

    fn parse_range(
        &self,
        range: &ParameterRange,
        param_names: &std::collections::HashSet<SmolStr>,
    ) -> TypedParameterRange {
        // Bounds that directly reference another parameter remain dynamic;
        // they must be checked against the final instance parameter vector.
        let fold = |e: &Expression| -> Option<f64> {
            if Self::references_identifiers(e, param_names) {
                None
            } else {
                self.eval_const(e)
            }
        };
        let computed = |expression: &Expression| {
            (Self::references_identifiers(expression, param_names)
                && Self::direct_parameter_reference(expression, param_names).is_none())
            .then(|| expression.clone())
        };

        // Extract bounds from first range bound if present
        if let Some(bound) = range.bounds.first() {
            let min = bound.lower.as_ref().and_then(fold);
            let max = bound.upper.as_ref().and_then(fold);
            let min_parameter = bound
                .lower
                .as_ref()
                .and_then(|expression| Self::direct_parameter_reference(expression, param_names));
            let max_parameter = bound
                .upper
                .as_ref()
                .and_then(|expression| Self::direct_parameter_reference(expression, param_names));
            let min_expression = bound.lower.as_ref().and_then(computed);
            let max_expression = bound.upper.as_ref().and_then(computed);
            let exclude: Vec<f64> = range.exclude.iter().filter_map(fold).collect();
            let exclude_parameters = range
                .exclude
                .iter()
                .filter_map(|expression| Self::direct_parameter_reference(expression, param_names))
                .collect();
            let exclude_expressions = range.exclude.iter().filter_map(computed).collect();

            TypedParameterRange {
                min,
                max,
                min_parameter,
                max_parameter,
                min_expression,
                max_expression,
                min_exclusive: !bound.lower_inclusive,
                max_exclusive: !bound.upper_inclusive,
                exclude,
                exclude_parameters,
                exclude_expressions,
            }
        } else {
            TypedParameterRange::unrestricted()
        }
    }

    fn record_error_at(&mut self, kind: SemanticErrorKind, span: Span) {
        self.errors.push(SemanticError::new(kind, span));
    }

    /// Validate the declaration-only portion of a public parameter array.
    /// Bounds remain symbolic in analyzed output: instance overrides of an
    /// earlier scalar parameter may change the eventual shape. This pass only
    /// proves that resolving those bounds later is deterministic and numeric.
    /// Expand retained replication only for language constructs whose result
    /// is an unpacked assignment-pattern value. The source AST is never
    /// mutated; analyzed/canonical metadata receives a bounded materialized
    /// clone. A complete sizing pass runs before any expanded vector is
    /// allocated, including checked multiplication for nested replication.
    fn materialize_replication_expression(
        &self,
        expression: &Expression,
        max_elements_per_pattern: usize,
        max_work: usize,
        owner: &str,
        require_instance_invariant_count: bool,
    ) -> CompileResult<Expression> {
        let Expression::ArrayLiteral(array) = expression else {
            return Ok(expression.clone());
        };
        self.measure_replication_array(
            array,
            max_elements_per_pattern,
            max_work,
            owner,
            require_instance_invariant_count,
            0,
        )?;
        Ok(Expression::ArrayLiteral(self.build_materialized_array(
            array,
            owner,
            require_instance_invariant_count,
        )?))
    }

    fn replication_count(
        &self,
        replication: &ReplicationExpr,
        owner: &str,
        require_instance_invariant_count: bool,
    ) -> CompileResult<u64> {
        let value = if require_instance_invariant_count {
            self.eval_const_invariant_value(&replication.count)
        } else {
            self.eval_const_value(&replication.count)
        };
        let count = match value {
            Some(ConstantValue::Integer(value)) => value,
            Some(ConstantValue::Real(value)) => {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidExpression(format!(
                        "{owner} replication count must be an integer constant expression; found real value {value}"
                    )),
                    replication.count.span(),
                )));
            }
            None => {
                let requirement = if require_instance_invariant_count {
                    "an instance-invariant integer constant expression (overridable parameters cannot determine executable operand shape)"
                } else {
                    "an integer constant expression"
                };
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidExpression(format!(
                        "{owner} replication count must be {requirement}"
                    )),
                    replication.count.span(),
                )));
            }
        };
        if count < 0 {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidExpression(format!(
                    "{owner} replication count must be non-negative; found {count}"
                )),
                replication.count.span(),
            )));
        }
        u64::try_from(count).map_err(|_| {
            CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidExpression(format!(
                    "{owner} replication count {count} is not representable by the materialization contract"
                )),
                replication.count.span(),
            ))
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn measure_replication_array(
        &self,
        array: &ArrayLiteralExpr,
        max_elements_per_pattern: usize,
        max_work: usize,
        owner: &str,
        require_instance_invariant_count: bool,
        depth: usize,
    ) -> CompileResult<(usize, usize)> {
        if depth >= MAX_REPLICATION_NESTING {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "{owner} replication nesting exceeds the safety limit of {MAX_REPLICATION_NESTING}"
                )),
                array.span,
            )));
        }
        let (elements, work) = self.measure_replication_elements(
            &array.elements,
            max_elements_per_pattern,
            max_work,
            owner,
            require_instance_invariant_count,
            depth,
        )?;
        if elements > max_elements_per_pattern {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "{owner} materializes {elements} elements in one assignment-pattern dimension; the supported safety limit is {max_elements_per_pattern}"
                )),
                array.span,
            )));
        }
        if work > max_work {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "{owner} replication materialization requires {work} syntax-tree items; the work budget is {max_work}"
                )),
                array.span,
            )));
        }
        Ok((elements, work))
    }

    #[allow(clippy::too_many_arguments)]
    fn measure_replication_elements(
        &self,
        elements: &[ArrayLiteralElement],
        max_elements_per_pattern: usize,
        max_work: usize,
        owner: &str,
        require_instance_invariant_count: bool,
        depth: usize,
    ) -> CompileResult<(usize, usize)> {
        if depth >= MAX_REPLICATION_NESTING {
            let span = elements
                .first()
                .map(ArrayLiteralElement::span)
                .unwrap_or_else(|| Span::new(crate::source::SourceId::new(0), 0, 0));
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "{owner} replication nesting exceeds the safety limit of {MAX_REPLICATION_NESTING}"
                )),
                span,
            )));
        }
        let mut output_len = 0_usize;
        let mut work = 0_usize;
        for element in elements {
            let (element_len, element_work, span) = match element {
                ArrayLiteralElement::Value(Expression::ArrayLiteral(nested)) => {
                    let (_, nested_work) = self.measure_replication_array(
                        nested,
                        max_elements_per_pattern,
                        max_work,
                        owner,
                        require_instance_invariant_count,
                        depth + 1,
                    )?;
                    (1, nested_work.checked_add(1), nested.span)
                }
                ArrayLiteralElement::Value(expression) => (1, Some(1), expression.span()),
                ArrayLiteralElement::Replication(replication) => {
                    if replication.elements.is_empty() {
                        return Err(CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::InvalidExpression(format!(
                                "{owner} replication body must contain at least one element"
                            )),
                            replication.span,
                        )));
                    }
                    let count = self.replication_count(
                        replication,
                        owner,
                        require_instance_invariant_count,
                    )?;
                    let (body_len, body_work) = self.measure_replication_elements(
                        &replication.elements,
                        max_elements_per_pattern,
                        max_work,
                        owner,
                        require_instance_invariant_count,
                        depth + 1,
                    )?;
                    let expanded_len = count
                        .checked_mul(u64::try_from(body_len).map_err(|_| {
                            CompileError::Semantic(SemanticError::new(
                                SemanticErrorKind::InvalidExpression(format!(
                                    "{owner} replication body length is not representable as u64"
                                )),
                                replication.span,
                            ))
                        })?)
                        .ok_or_else(|| {
                            CompileError::Semantic(SemanticError::new(
                                SemanticErrorKind::InvalidExpression(format!(
                                    "{owner} replication element count overflows u64"
                                )),
                                replication.span,
                            ))
                        })?;
                    let expanded_work = count
                        .checked_mul(u64::try_from(body_work).map_err(|_| {
                            CompileError::Semantic(SemanticError::new(
                                SemanticErrorKind::InvalidExpression(format!(
                                    "{owner} replication body work is not representable as u64"
                                )),
                                replication.span,
                            ))
                        })?)
                        .ok_or_else(|| {
                            CompileError::Semantic(SemanticError::new(
                                SemanticErrorKind::InvalidExpression(format!(
                                    "{owner} replication work count overflows u64"
                                )),
                                replication.span,
                            ))
                        })?;
                    if expanded_len > max_elements_per_pattern as u64 {
                        return Err(CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::UnsupportedFeature(format!(
                                "{owner} materializes {expanded_len} elements in one assignment-pattern dimension; the supported safety limit is {max_elements_per_pattern}"
                            )),
                            replication.span,
                        )));
                    }
                    if expanded_work > max_work as u64 {
                        return Err(CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::UnsupportedFeature(format!(
                                "{owner} replication materialization requires {expanded_work} syntax-tree items; the work budget is {max_work}"
                            )),
                            replication.span,
                        )));
                    }
                    (
                        usize::try_from(expanded_len).map_err(|_| {
                            CompileError::Semantic(SemanticError::new(
                                SemanticErrorKind::InvalidExpression(format!(
                                    "{owner} replication element count {expanded_len} is not representable on this platform"
                                )),
                                replication.span,
                            ))
                        })?,
                        usize::try_from(expanded_work).ok(),
                        replication.span,
                    )
                }
            };
            let element_work = element_work.ok_or_else(|| {
                CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidExpression(format!(
                        "{owner} replication work count overflows usize"
                    )),
                    span,
                ))
            })?;
            output_len = output_len.checked_add(element_len).ok_or_else(|| {
                CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidExpression(format!(
                        "{owner} replication element count overflows usize"
                    )),
                    span,
                ))
            })?;
            work = work.checked_add(element_work).ok_or_else(|| {
                CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidExpression(format!(
                        "{owner} replication work count overflows usize"
                    )),
                    span,
                ))
            })?;
            if output_len > max_elements_per_pattern {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "{owner} materializes {output_len} elements in one assignment-pattern dimension; the supported safety limit is {max_elements_per_pattern}"
                    )),
                    span,
                )));
            }
            if work > max_work {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "{owner} replication materialization requires more than {max_work} syntax-tree items"
                    )),
                    span,
                )));
            }
        }
        Ok((output_len, work))
    }

    fn build_materialized_array(
        &self,
        array: &ArrayLiteralExpr,
        owner: &str,
        require_instance_invariant_count: bool,
    ) -> CompileResult<ArrayLiteralExpr> {
        let elements = self.build_materialized_elements(
            &array.elements,
            owner,
            require_instance_invariant_count,
        )?;
        Ok(ArrayLiteralExpr {
            elements,
            assignment_pattern: array.assignment_pattern,
            span: array.span,
        })
    }

    fn build_materialized_elements(
        &self,
        elements: &[ArrayLiteralElement],
        owner: &str,
        require_instance_invariant_count: bool,
    ) -> CompileResult<Vec<ArrayLiteralElement>> {
        let mut materialized = Vec::new();
        for element in elements {
            match element {
                ArrayLiteralElement::Value(Expression::ArrayLiteral(nested)) => {
                    materialized.push(ArrayLiteralElement::Value(Expression::ArrayLiteral(
                        self.build_materialized_array(
                            nested,
                            owner,
                            require_instance_invariant_count,
                        )?,
                    )));
                }
                ArrayLiteralElement::Value(expression) => {
                    materialized.push(ArrayLiteralElement::Value(expression.clone()));
                }
                ArrayLiteralElement::Replication(replication) => {
                    let count = self.replication_count(
                        replication,
                        owner,
                        require_instance_invariant_count,
                    )?;
                    let count = usize::try_from(count).map_err(|_| {
                        CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::InvalidExpression(format!(
                                "{owner} replication count is not representable on this platform during materialization"
                            )),
                            replication.span,
                        ))
                    })?;
                    let body = self.build_materialized_elements(
                        &replication.elements,
                        owner,
                        require_instance_invariant_count,
                    )?;
                    let additional = count.checked_mul(body.len()).ok_or_else(|| {
                        CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::InvalidExpression(format!(
                                "{owner} replication element count overflows usize during materialization"
                            )),
                            replication.span,
                        ))
                    })?;
                    materialized.try_reserve(additional).map_err(|_| {
                        CompileError::Semantic(SemanticError::new(
                            SemanticErrorKind::UnsupportedFeature(format!(
                                "{owner} replication materialization could not reserve storage for {additional} elements"
                            )),
                            replication.span,
                        ))
                    })?;
                    for _ in 0..count {
                        materialized.extend(body.iter().cloned());
                    }
                }
            }
        }
        Ok(materialized)
    }

    fn validate_parameter_array_declaration(
        &mut self,
        parameter: &ParameterDecl,
        materialized_default: Option<&Expression>,
        parameter_index: usize,
        parameters: &[ParameterDecl],
        parameter_indices: &HashMap<SmolStr, usize>,
    ) {
        if !parameter.type_is_explicit {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "parameter array '{}' requires an explicit integer or real element type",
                    parameter.name
                )),
                parameter.span,
            );
        }
        if parameter.default.is_none() {
            self.record_error_at(
                SemanticErrorKind::InvalidExpression(format!(
                    "parameter array '{}' requires a default value",
                    parameter.name
                )),
                parameter.span,
            );
        }
        if parameter.param_type == ParamType::String {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "string parameter array '{}' is not supported; parameter arrays must have integer or real elements",
                    parameter.name
                )),
                parameter.span,
            );
        }
        if let Some(range) = &parameter.range {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "parameter array '{}' may not use from/exclude constraints until array-valued constraint semantics are implemented",
                    parameter.name
                )),
                range.span,
            );
        }
        if parameter.dimensions.len() > MAX_PARAMETER_ARRAY_RANK {
            self.record_error_at(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "parameter array '{}' has rank {}; the supported safety limit is {}",
                    parameter.name,
                    parameter.dimensions.len(),
                    MAX_PARAMETER_ARRAY_RANK
                )),
                parameter.span,
            );
        }

        let initializer = match materialized_default.or(parameter.default.as_ref()) {
            Some(Expression::ArrayLiteral(initializer)) if initializer.assignment_pattern => {
                Some(initializer)
            }
            Some(Expression::ArrayLiteral(initializer)) => {
                self.record_error_at(
                    SemanticErrorKind::TypeMismatch {
                        expected: "constant assignment pattern opened with `'{'".into(),
                        found: "concatenation opened with '{'".into(),
                        context: format!("default of parameter array '{}'", parameter.name),
                    },
                    initializer.span,
                );
                None
            }
            Some(other) => {
                self.record_error_at(
                    SemanticErrorKind::TypeMismatch {
                        expected: "constant assignment pattern".into(),
                        found: "scalar expression".into(),
                        context: format!("default of parameter array '{}'", parameter.name),
                    },
                    other.span(),
                );
                None
            }
            None => None,
        };

        for (dimension_index, dimension) in parameter.dimensions.iter().enumerate() {
            for (side, bound) in [("left", &dimension.start), ("right", &dimension.end)] {
                let owner = format!(
                    "{side} bound of dimension {} of parameter array '{}'",
                    dimension_index + 1,
                    parameter.name
                );
                if !self.validate_parameter_array_bound_expression(
                    bound,
                    &owner,
                    parameter_index,
                    parameters,
                    parameter_indices,
                ) {
                    continue;
                }

                match self.eval_const(bound) {
                    Some(value) if !value.is_finite() => {
                        self.record_error_at(
                            SemanticErrorKind::InvalidExpression(format!(
                                "{owner} resolves to non-finite value {value}"
                            )),
                            bound.span(),
                        );
                    }
                    Some(value) if value.fract() != 0.0 => {
                        self.record_error_at(
                            SemanticErrorKind::InvalidExpression(format!(
                                "{owner} resolves to non-integral value {value}"
                            )),
                            bound.span(),
                        );
                    }
                    Some(value)
                        if !(-9_223_372_036_854_775_808.0..9_223_372_036_854_775_808.0)
                            .contains(&value) =>
                    {
                        self.record_error_at(
                            SemanticErrorKind::InvalidExpression(format!(
                                "{owner} resolves outside the signed 64-bit index range: {value}"
                            )),
                            bound.span(),
                        );
                    }
                    Some(_) => {}
                    None => self.record_error_at(
                        SemanticErrorKind::InvalidExpression(format!(
                            "{owner} does not resolve to a valid integer using declared parameter defaults"
                        )),
                        bound.span(),
                    ),
                }
            }
        }

        let constant_shape = self.resolve_parameter_array_default_shape(parameter);
        if let Some(initializer) = initializer {
            if parameter.param_type != ParamType::String {
                let owner = format!("default of parameter array '{}'", parameter.name);
                for element in &initializer.elements {
                    match element {
                        ArrayLiteralElement::Value(expression) => {
                            self.validate_parameter_array_initializer_elements(
                                expression,
                                parameter,
                                &owner,
                                parameter_index,
                                parameters,
                                parameter_indices,
                            );
                        }
                        ArrayLiteralElement::Replication(replication) => {
                            self.record_error_at(
                                SemanticErrorKind::InvalidExpression(format!(
                                    "{owner} retained replication reached parameter-array validation without bounded materialization"
                                )),
                                replication.span,
                            );
                        }
                    }
                }
            }

            if let Some(shape) = constant_shape
                && let Err(detail) =
                    Self::validate_parameter_array_initializer_shape(initializer, &shape, 0)
            {
                self.record_error_at(
                    SemanticErrorKind::TypeMismatch {
                        expected: format!("rectangular assignment pattern with shape {shape:?}"),
                        found: detail,
                        context: format!("default of parameter array '{}'", parameter.name),
                    },
                    initializer.span,
                );
            }
        }
    }

    fn resolve_parameter_array_default_shape(
        &mut self,
        parameter: &ParameterDecl,
    ) -> Option<Vec<u64>> {
        if parameter.dimensions.len() > MAX_PARAMETER_ARRAY_RANK {
            return None;
        }

        let mut shape = Vec::with_capacity(parameter.dimensions.len());
        let mut total_elements = 1_u64;
        for (dimension_index, dimension) in parameter.dimensions.iter().enumerate() {
            let left = Self::exact_const_i64(self.eval_const(&dimension.start)?)?;
            let right = Self::exact_const_i64(self.eval_const(&dimension.end)?)?;
            let Some(extent) = left.abs_diff(right).checked_add(1) else {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(format!(
                        "dimension {} of parameter array '{}' has an unrepresentable extent",
                        dimension_index + 1,
                        parameter.name
                    )),
                    dimension.span,
                );
                return None;
            };
            let Some(next_total) = total_elements.checked_mul(extent) else {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(format!(
                        "parameter array '{}' element count overflows the canonical shape representation",
                        parameter.name
                    )),
                    parameter.span,
                );
                return None;
            };
            if next_total > MAX_PARAMETER_ARRAY_ELEMENTS {
                self.record_error_at(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "parameter array '{}' declares {next_total} elements; the supported safety limit is {MAX_PARAMETER_ARRAY_ELEMENTS}",
                        parameter.name
                    )),
                    parameter.span,
                );
                return None;
            }
            total_elements = next_total;
            shape.push(extent);
        }
        Some(shape)
    }

    fn validate_parameter_array_initializer_elements(
        &mut self,
        expression: &Expression,
        parameter: &ParameterDecl,
        owner: &str,
        parameter_index: usize,
        parameters: &[ParameterDecl],
        parameter_indices: &HashMap<SmolStr, usize>,
    ) -> bool {
        if let Expression::ArrayLiteral(array) = expression {
            return array.elements.iter().fold(true, |valid, element| {
                let element_valid = match element {
                    ArrayLiteralElement::Value(expression) => {
                        self.validate_parameter_array_initializer_elements(
                            expression,
                            parameter,
                            owner,
                            parameter_index,
                            parameters,
                            parameter_indices,
                        )
                    }
                    ArrayLiteralElement::Replication(replication) => {
                        self.record_error_at(
                            SemanticErrorKind::InvalidExpression(format!(
                                "{owner} retained replication reached parameter-array validation without bounded materialization"
                            )),
                            replication.span,
                        );
                        false
                    }
                };
                element_valid && valid
            });
        }

        let valid = self.validate_parameter_array_bound_expression(
            expression,
            owner,
            parameter_index,
            parameters,
            parameter_indices,
        );
        if !valid {
            return false;
        }

        let Some(value) = self.eval_const(expression) else {
            self.record_error_at(
                SemanticErrorKind::InvalidExpression(format!(
                    "{owner} element does not resolve using declared parameter defaults"
                )),
                expression.span(),
            );
            return false;
        };
        if !value.is_finite() {
            self.record_error_at(
                SemanticErrorKind::InvalidExpression(format!(
                    "{owner} element resolves to non-finite value {value}"
                )),
                expression.span(),
            );
            return false;
        }
        if parameter.param_type == ParamType::Integer
            && (value.fract() != 0.0 || value < f64::from(i32::MIN) || value > f64::from(i32::MAX))
        {
            self.record_error_at(
                SemanticErrorKind::TypeMismatch {
                    expected: "32-bit integer array element".into(),
                    found: value.to_string(),
                    context: owner.into(),
                },
                expression.span(),
            );
            return false;
        }
        true
    }

    fn validate_parameter_array_initializer_shape(
        initializer: &crate::ast::ArrayLiteralExpr,
        shape: &[u64],
        dimension: usize,
    ) -> Result<(), String> {
        let Some(&expected_len) = shape.get(dimension) else {
            return Err(format!(
                "nested assignment pattern extends beyond declared rank {}",
                shape.len()
            ));
        };
        if u64::try_from(initializer.elements.len()).ok() != Some(expected_len) {
            return Err(format!(
                "dimension {} has {} elements",
                dimension + 1,
                initializer.elements.len()
            ));
        }

        let is_leaf = dimension + 1 == shape.len();
        for element in &initializer.elements {
            match (is_leaf, element) {
                (_, ArrayLiteralElement::Replication(_)) => {
                    return Err(format!(
                        "dimension {} contains retained replication after the materialization boundary",
                        dimension + 1
                    ));
                }
                (true, ArrayLiteralElement::Value(Expression::ArrayLiteral(_))) => {
                    return Err(format!(
                        "dimension {} contains an unexpected nested pattern",
                        dimension + 1
                    ));
                }
                (false, ArrayLiteralElement::Value(Expression::ArrayLiteral(nested))) => {
                    if !nested.assignment_pattern {
                        return Err(format!(
                            "dimension {} contains a concatenation instead of an assignment pattern",
                            dimension + 1
                        ));
                    }
                    Self::validate_parameter_array_initializer_shape(nested, shape, dimension + 1)?;
                }
                (false, ArrayLiteralElement::Value(_)) => {
                    return Err(format!(
                        "dimension {} contains a scalar before the final dimension",
                        dimension + 1
                    ));
                }
                (true, ArrayLiteralElement::Value(_)) => {}
            }
        }
        Ok(())
    }

    /// Return true when `expression` belongs to the numeric constant-expression
    /// subset used by parameter-array bounds. Diagnostics identify the first
    /// invalid leaf so malformed shapes cannot degrade into an unresolved
    /// runtime expression.
    fn validate_parameter_array_bound_expression(
        &mut self,
        expression: &Expression,
        owner: &str,
        parameter_index: usize,
        parameters: &[ParameterDecl],
        parameter_indices: &HashMap<SmolStr, usize>,
    ) -> bool {
        let validate_child = |this: &mut Self, child: &Expression| {
            this.validate_parameter_array_bound_expression(
                child,
                owner,
                parameter_index,
                parameters,
                parameter_indices,
            )
        };

        match expression {
            Expression::Number(_) => true,
            Expression::Identifier(identifier) if identifier.name == "inf" => true,
            Expression::Identifier(identifier) => {
                let Some(&referenced_index) = parameter_indices.get(&identifier.name) else {
                    self.record_error_at(
                        SemanticErrorKind::InvalidExpression(format!(
                            "{owner} references unknown identifier '{}'",
                            identifier.name
                        )),
                        identifier.span,
                    );
                    return false;
                };
                let referenced = &parameters[referenced_index];
                if referenced_index == parameter_index {
                    self.record_error_at(
                        SemanticErrorKind::CircularDependency(format!(
                            "{owner} references parameter '{}' itself",
                            parameters[parameter_index].name
                        )),
                        identifier.span,
                    );
                    false
                } else if referenced_index > parameter_index {
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "{owner} references later parameter '{}'; parameter-array bounds may reference only previously declared scalar parameters",
                            referenced.name
                        )),
                        identifier.span,
                    );
                    false
                } else if !referenced.dimensions.is_empty() {
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "{owner} references parameter array '{}'; parameter-array bounds may reference only previously declared scalar parameters",
                            referenced.name
                        )),
                        identifier.span,
                    );
                    false
                } else if referenced.param_type == ParamType::String {
                    self.record_error_at(
                        SemanticErrorKind::TypeMismatch {
                            expected: "previously declared numeric scalar parameter".into(),
                            found: format!("string parameter '{}'", referenced.name),
                            context: owner.into(),
                        },
                        identifier.span,
                    );
                    false
                } else {
                    true
                }
            }
            Expression::Unary(unary) => {
                let child = validate_child(self, &unary.operand);
                let operand = unary.op != UnaryOp::BitNot
                    || self.validate_integer_operator_expression(
                        &unary.operand,
                        "operand of bitwise complement",
                    );
                child && operand
            }
            Expression::Binary(binary) => {
                let left = validate_child(self, &binary.left);
                let right = validate_child(self, &binary.right);
                let operands = if matches!(
                    binary.op,
                    BinaryOp::BitAnd
                        | BinaryOp::BitOr
                        | BinaryOp::BitXor
                        | BinaryOp::Shl
                        | BinaryOp::Shr
                ) {
                    let left_type = self.validate_integer_operator_expression(
                        &binary.left,
                        "left operand of bitwise or shift operator",
                    );
                    let right_type = self.validate_integer_operator_expression(
                        &binary.right,
                        "right operand of bitwise or shift operator",
                    );
                    left_type && right_type
                } else {
                    true
                };
                left && right && operands
            }
            Expression::Conditional(conditional) => {
                let condition = validate_child(self, &conditional.condition);
                let then_expr = validate_child(self, &conditional.then_expr);
                let else_expr = validate_child(self, &conditional.else_expr);
                condition && then_expr && else_expr
            }
            Expression::Call(call) => {
                let expected_arity = match call.name.as_str() {
                    "abs" | "sqrt" | "exp" | "ln" | "log" | "log10" | "floor" | "ceil" => Some(1),
                    "min" | "max" | "pow" => Some(2),
                    _ => None,
                };
                let Some(expected_arity) = expected_arity else {
                    self.record_error_at(
                        SemanticErrorKind::InvalidExpression(format!(
                            "{owner} calls non-constant function '{}'",
                            call.name
                        )),
                        call.span,
                    );
                    return false;
                };
                if call.args.len() != expected_arity {
                    self.record_error_at(
                        SemanticErrorKind::ArgumentCountMismatch {
                            name: call.name.to_string(),
                            expected: expected_arity.to_string(),
                            got: call.args.len(),
                        },
                        call.span,
                    );
                    return false;
                }
                call.args.iter().fold(true, |valid, argument| {
                    validate_child(self, argument) && valid
                })
            }
            Expression::StringLit(string) => {
                self.record_error_at(
                    SemanticErrorKind::TypeMismatch {
                        expected: "numeric constant expression".into(),
                        found: "string literal".into(),
                        context: owner.into(),
                    },
                    string.span,
                );
                false
            }
            Expression::ArrayAccess(access) => {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(format!(
                        "{owner} indexes array '{}'; parameter-array bounds may reference only previously declared scalar parameters",
                        access.array
                    )),
                    access.span,
                );
                false
            }
            Expression::ArrayLiteral(array) => {
                self.record_error_at(
                    SemanticErrorKind::TypeMismatch {
                        expected: "numeric constant expression".into(),
                        found: "array literal".into(),
                        context: owner.into(),
                    },
                    array.span,
                );
                false
            }
            Expression::SystemFunction(function) => {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(format!(
                        "{owner} calls non-constant system function '{}'",
                        function.name
                    )),
                    function.span,
                );
                false
            }
            Expression::NullArgument(span) => {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(format!(
                        "{owner} contains a null argument"
                    )),
                    *span,
                );
                false
            }
            Expression::BranchAccess(access) => {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(format!(
                        "{owner} reads non-constant branch access"
                    )),
                    access.span(),
                );
                false
            }
            Expression::AnalogOperator(operator) => {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(format!(
                        "{owner} contains non-constant analog operator"
                    )),
                    operator.span(),
                );
                false
            }
            Expression::NoiseSource(noise) => {
                self.record_error_at(
                    SemanticErrorKind::InvalidExpression(format!(
                        "{owner} contains non-constant noise source"
                    )),
                    noise.span(),
                );
                false
            }
        }
    }

    /// Verilog-AMS 2.3.1 section 3.4 permits a parameter initializer to read
    /// only parameters declared before it. Enforce that language rule here,
    /// before symbolic defaults reach any backend, so composite forward and
    /// cyclic references cannot acquire order-dependent values.
    fn validate_parameter_default_dependencies(
        &mut self,
        parameters: &[ParameterDecl],
        aliases: &[AliasParamDecl],
        indices: &HashMap<SmolStr, usize>,
    ) {
        // External SPICE parameter names and the generated `$param_given`
        // resolver are intentionally case-insensitive. Reject collisions up
        // front: otherwise the backend's sorted lookup table would pick one
        // declaration nondeterministically. Aliases resolve to their exact,
        // case-sensitive Verilog-A target.
        let mut param_given_indices: std::collections::HashMap<String, (usize, SmolStr)> =
            std::collections::HashMap::new();
        for (index, parameter) in parameters.iter().enumerate() {
            Self::insert_external_parameter_name(
                &mut param_given_indices,
                parameter.name.as_str(),
                parameter.name.as_str(),
                index,
                parameter.span,
                &mut self.errors,
            );
        }
        for alias in aliases {
            let Some(&target) = indices.get(&alias.target) else {
                continue;
            };
            Self::insert_external_parameter_name(
                &mut param_given_indices,
                alias.alias.as_str(),
                parameters[target].name.as_str(),
                target,
                alias.span,
                &mut self.errors,
            );
        }

        for (index, parameter) in parameters.iter().enumerate() {
            let Some(default) = &parameter.default else {
                continue;
            };
            let mut references = Vec::new();
            Self::collect_parameter_identifier_references(
                default,
                indices,
                &param_given_indices,
                &mut references,
            );
            let mut reported = std::collections::HashSet::new();
            for (referenced_index, referenced, span) in references {
                if !reported.insert(referenced_index) {
                    continue;
                }
                if referenced_index == index {
                    self.record_error_at(
                        SemanticErrorKind::CircularDependency(format!(
                            "default of parameter '{}' references itself",
                            parameter.name
                        )),
                        span,
                    );
                } else if referenced_index > index {
                    self.record_error_at(
                        SemanticErrorKind::UnsupportedFeature(format!(
                            "default of parameter '{}' references later parameter '{}'; Verilog-AMS parameter defaults may reference only previously declared parameters",
                            parameter.name, referenced
                        )),
                        span,
                    );
                }
            }
        }
    }

    fn insert_external_parameter_name(
        names: &mut std::collections::HashMap<String, (usize, SmolStr)>,
        name: &str,
        canonical_name: &str,
        index: usize,
        span: Span,
        errors: &mut Vec<SemanticError>,
    ) {
        let folded = name.to_ascii_lowercase();
        if let Some((_, first_name)) = names.get(&folded) {
            errors.push(SemanticError::new(
                SemanticErrorKind::UnsupportedFeature(format!(
                    "parameter lookup name '{name}' conflicts case-insensitively with '{first_name}'; generated SPICE parameter names and aliases must be unique ignoring ASCII case"
                )),
                span,
            ));
            return;
        }
        names.insert(folded, (index, canonical_name.into()));
    }

    fn collect_parameter_identifier_references<'a>(
        expression: &'a Expression,
        parameter_indices: &std::collections::HashMap<SmolStr, usize>,
        param_given_indices: &std::collections::HashMap<String, (usize, SmolStr)>,
        references: &mut Vec<(usize, SmolStr, Span)>,
    ) {
        match expression {
            Expression::Identifier(identifier) => {
                if let Some(&index) = parameter_indices.get(&identifier.name) {
                    references.push((index, identifier.name.clone(), identifier.span));
                }
            }
            Expression::Binary(binary) => {
                Self::collect_parameter_identifier_references(
                    &binary.left,
                    parameter_indices,
                    param_given_indices,
                    references,
                );
                Self::collect_parameter_identifier_references(
                    &binary.right,
                    parameter_indices,
                    param_given_indices,
                    references,
                );
            }
            Expression::Unary(unary) => Self::collect_parameter_identifier_references(
                &unary.operand,
                parameter_indices,
                param_given_indices,
                references,
            ),
            Expression::Conditional(conditional) => {
                Self::collect_parameter_identifier_references(
                    &conditional.condition,
                    parameter_indices,
                    param_given_indices,
                    references,
                );
                Self::collect_parameter_identifier_references(
                    &conditional.then_expr,
                    parameter_indices,
                    param_given_indices,
                    references,
                );
                Self::collect_parameter_identifier_references(
                    &conditional.else_expr,
                    parameter_indices,
                    param_given_indices,
                    references,
                );
            }
            Expression::Call(call) => {
                for argument in &call.args {
                    Self::collect_parameter_identifier_references(
                        argument,
                        parameter_indices,
                        param_given_indices,
                        references,
                    );
                }
            }
            Expression::SystemFunction(function) => {
                if function.name.eq_ignore_ascii_case("$param_given")
                    || function.name.eq_ignore_ascii_case("param_given")
                {
                    if let [Expression::Identifier(identifier)] = function.args.as_slice()
                        && let Some((index, canonical)) =
                            param_given_indices.get(&identifier.name.to_ascii_lowercase())
                    {
                        references.push((*index, canonical.clone(), identifier.span));
                    }
                    return;
                }
                for argument in &function.args {
                    Self::collect_parameter_identifier_references(
                        argument,
                        parameter_indices,
                        param_given_indices,
                        references,
                    );
                }
            }
            Expression::ArrayAccess(access) => {
                if let Some(&index) = parameter_indices.get(&access.array) {
                    references.push((index, access.array.clone(), access.span));
                }
                Self::collect_parameter_identifier_references(
                    &access.index,
                    parameter_indices,
                    param_given_indices,
                    references,
                );
            }
            Expression::ArrayLiteral(array) => {
                for element in &array.elements {
                    Self::collect_array_element_parameter_identifier_references(
                        element,
                        parameter_indices,
                        param_given_indices,
                        references,
                    );
                }
            }
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::NullArgument(_)
            | Expression::BranchAccess(_)
            | Expression::AnalogOperator(_)
            | Expression::NoiseSource(_) => {}
        }
    }

    fn collect_array_element_parameter_identifier_references<'a>(
        element: &'a ArrayLiteralElement,
        parameter_indices: &HashMap<SmolStr, usize>,
        param_given_indices: &HashMap<String, (usize, SmolStr)>,
        references: &mut Vec<(usize, SmolStr, Span)>,
    ) {
        match element {
            ArrayLiteralElement::Value(expression) => {
                Self::collect_parameter_identifier_references(
                    expression,
                    parameter_indices,
                    param_given_indices,
                    references,
                );
            }
            ArrayLiteralElement::Replication(replication) => {
                Self::collect_parameter_identifier_references(
                    &replication.count,
                    parameter_indices,
                    param_given_indices,
                    references,
                );
                for element in &replication.elements {
                    Self::collect_array_element_parameter_identifier_references(
                        element,
                        parameter_indices,
                        param_given_indices,
                        references,
                    );
                }
            }
        }
    }

    /// Whether an expression references any identifier from the given set
    fn references_identifiers(
        expr: &Expression,
        names: &std::collections::HashSet<SmolStr>,
    ) -> bool {
        match expr {
            Expression::Identifier(id) => names.contains(&id.name),
            Expression::Number(_) | Expression::StringLit(_) | Expression::NullArgument(_) => false,
            Expression::Binary(b) => {
                Self::references_identifiers(&b.left, names)
                    || Self::references_identifiers(&b.right, names)
            }
            Expression::Unary(u) => Self::references_identifiers(&u.operand, names),
            Expression::Conditional(c) => {
                Self::references_identifiers(&c.condition, names)
                    || Self::references_identifiers(&c.then_expr, names)
                    || Self::references_identifiers(&c.else_expr, names)
            }
            Expression::Call(call) => call
                .args
                .iter()
                .any(|a| Self::references_identifiers(a, names)),
            Expression::SystemFunction(f) => f
                .args
                .iter()
                .any(|a| Self::references_identifiers(a, names)),
            Expression::ArrayAccess(a) => {
                names.contains(&a.array) || Self::references_identifiers(&a.index, names)
            }
            Expression::ArrayLiteral(a) => a
                .elements
                .iter()
                .any(|element| Self::array_element_references_identifiers(element, names)),
            Expression::BranchAccess(_)
            | Expression::AnalogOperator(_)
            | Expression::NoiseSource(_) => false,
        }
    }

    fn array_element_references_identifiers(
        element: &ArrayLiteralElement,
        names: &std::collections::HashSet<SmolStr>,
    ) -> bool {
        match element {
            ArrayLiteralElement::Value(expression) => {
                Self::references_identifiers(expression, names)
            }
            ArrayLiteralElement::Replication(replication) => {
                Self::references_identifiers(&replication.count, names)
                    || replication
                        .elements
                        .iter()
                        .any(|element| Self::array_element_references_identifiers(element, names))
            }
        }
    }

    /// Whether a model-parameter default reads a parameter that has no
    /// model-card storage. Ordinary Verilog-A identifiers resolve with exact
    /// case; `$param_given` follows the generated external lookup and accepts
    /// canonical names and aliases case-insensitively.
    fn references_parameter_without_model_storage(
        expr: &Expression,
        canonical_storage: &std::collections::HashMap<SmolStr, bool>,
        external_storage: &std::collections::HashMap<String, bool>,
    ) -> bool {
        match expr {
            Expression::Identifier(identifier) => canonical_storage
                .get(&identifier.name)
                .is_some_and(|has_model_storage| !has_model_storage),
            Expression::SystemFunction(function)
                if function.name.eq_ignore_ascii_case("$param_given")
                    || function.name.eq_ignore_ascii_case("param_given") =>
            {
                let [Expression::Identifier(identifier)] = function.args.as_slice() else {
                    return false;
                };
                external_storage
                    .get(&identifier.name.to_ascii_lowercase())
                    .is_some_and(|has_model_storage| !has_model_storage)
            }
            Expression::Binary(binary) => {
                Self::references_parameter_without_model_storage(
                    &binary.left,
                    canonical_storage,
                    external_storage,
                ) || Self::references_parameter_without_model_storage(
                    &binary.right,
                    canonical_storage,
                    external_storage,
                )
            }
            Expression::Unary(unary) => Self::references_parameter_without_model_storage(
                &unary.operand,
                canonical_storage,
                external_storage,
            ),
            Expression::Conditional(conditional) => {
                Self::references_parameter_without_model_storage(
                    &conditional.condition,
                    canonical_storage,
                    external_storage,
                ) || Self::references_parameter_without_model_storage(
                    &conditional.then_expr,
                    canonical_storage,
                    external_storage,
                ) || Self::references_parameter_without_model_storage(
                    &conditional.else_expr,
                    canonical_storage,
                    external_storage,
                )
            }
            Expression::Call(call) => call.args.iter().any(|argument| {
                Self::references_parameter_without_model_storage(
                    argument,
                    canonical_storage,
                    external_storage,
                )
            }),
            Expression::SystemFunction(function) => function.args.iter().any(|argument| {
                Self::references_parameter_without_model_storage(
                    argument,
                    canonical_storage,
                    external_storage,
                )
            }),
            Expression::ArrayAccess(access) => {
                canonical_storage
                    .get(&access.array)
                    .is_some_and(|has_model_storage| !has_model_storage)
                    || Self::references_parameter_without_model_storage(
                        &access.index,
                        canonical_storage,
                        external_storage,
                    )
            }
            Expression::ArrayLiteral(array) => array.elements.iter().any(|element| {
                Self::array_element_references_parameter_without_model_storage(
                    element,
                    canonical_storage,
                    external_storage,
                )
            }),
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::NullArgument(_)
            | Expression::BranchAccess(_)
            | Expression::AnalogOperator(_)
            | Expression::NoiseSource(_) => false,
        }
    }

    fn array_element_references_parameter_without_model_storage(
        element: &ArrayLiteralElement,
        canonical_storage: &std::collections::HashMap<SmolStr, bool>,
        external_storage: &std::collections::HashMap<String, bool>,
    ) -> bool {
        match element {
            ArrayLiteralElement::Value(expression) => {
                Self::references_parameter_without_model_storage(
                    expression,
                    canonical_storage,
                    external_storage,
                )
            }
            ArrayLiteralElement::Replication(replication) => {
                Self::references_parameter_without_model_storage(
                    &replication.count,
                    canonical_storage,
                    external_storage,
                ) || replication.elements.iter().any(|element| {
                    Self::array_element_references_parameter_without_model_storage(
                        element,
                        canonical_storage,
                        external_storage,
                    )
                })
            }
        }
    }

    fn direct_parameter_reference(
        expression: &Expression,
        names: &std::collections::HashSet<SmolStr>,
    ) -> Option<SmolStr> {
        let Expression::Identifier(identifier) = expression else {
            return None;
        };
        names
            .contains(&identifier.name)
            .then(|| identifier.name.clone())
    }
}

fn is_global_ground_name(name: &str) -> bool {
    name == "0"
}

fn is_zi_operator_name(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "zi_zp" | "zi_zd" | "zi_np" | "zi_nd"
    )
}

/// Canonical name of a source-level analog operator whose state must be
/// visited on every Newton iteration. Keep this list deliberately narrower
/// than the built-in-function registry: `ddx`, math, access, nature, and noise
/// calls are not stateful evaluation sites.
fn stateful_analog_operator_call_name(name: &str) -> Option<&'static str> {
    Some(match name.to_ascii_lowercase().as_str() {
        "ddt" => "ddt",
        "idt" => "idt",
        "idtmod" => "idtmod",
        "absdelay" => "absdelay",
        "transition" => "transition",
        "slew" => "slew",
        "cross" => "cross",
        "above" => "above",
        "last_crossing" => "last_crossing",
        "timer" => "timer",
        "laplace_zp" => "laplace_zp",
        "laplace_zd" => "laplace_zd",
        "laplace_np" => "laplace_np",
        "laplace_nd" => "laplace_nd",
        "zi_zp" => "zi_zp",
        "zi_zd" => "zi_zd",
        "zi_np" => "zi_np",
        "zi_nd" => "zi_nd",
        _ => return None,
    })
}

/// Canonical name of a public typed analog-operator node with per-evaluation
/// state. Event-control `cross`, `above`, and `timer` use [`EventExpr`] rather
/// than this enum and are intentionally validated by their enclosing event
/// semantics instead of being rejected as conditional calls.
fn stateful_public_analog_operator_name(operator: &AnalogOperator) -> Option<&'static str> {
    Some(match operator {
        AnalogOperator::Ddt { .. } => "ddt",
        AnalogOperator::Idt { .. } => "idt",
        AnalogOperator::IdtMod { .. } => "idtmod",
        AnalogOperator::Absdelay { .. } => "absdelay",
        AnalogOperator::Transition { .. } => "transition",
        AnalogOperator::Slew { .. } => "slew",
        AnalogOperator::LastCrossing { .. } => "last_crossing",
        AnalogOperator::Laplace { kind, .. } => match kind {
            LaplaceKind::ZeroPole { .. } => "laplace_zp",
            LaplaceKind::ZeroDenominator { .. } => "laplace_zd",
            LaplaceKind::NumeratorPole { .. } => "laplace_np",
            LaplaceKind::NumeratorDenominator { .. } => "laplace_nd",
        },
        AnalogOperator::Zi { kind, .. } => match kind {
            ZiKind::ZeroPole { .. } => "zi_zp",
            ZiKind::ZeroDenominator { .. } => "zi_zd",
            ZiKind::NumeratorPole { .. } => "zi_np",
            ZiKind::NumeratorDenominator { .. } => "zi_nd",
        },
        AnalogOperator::Limit { .. }
        | AnalogOperator::LimiterArgument { .. }
        | AnalogOperator::Ddx { .. }
        | AnalogOperator::Limexp { .. } => return None,
    })
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests;
