//! Lowering the analyzed discrete-domain half of a module into CFG process
//! functions.
//!
//! The analog counterpart is [`cfg_lower`](super::cfg_lower), which turns a
//! HIR body into one function. This does the same job for processes, from the
//! analyzed syntax tree rather than from HIR: a process body is already
//! source-shaped nested control flow, and routing it through a second
//! source-shaped level would copy it without resolving anything.
//!
//! # What state lives where
//!
//! A module-level `reg` or `wire` is never an SSA value. It is a signal, read
//! by a [`CfgValueKind::DigitalSignalRead`] node and written by a write node.
//! That is not a simplification of the model — it *is* the model. A process
//! that suspends and resumes must see whatever the signal holds when it wakes,
//! and a value carried across the suspension in a register would see what it
//! held when the process went to sleep.
//!
//! Static locals with deferred writes or event subscriptions use the same
//! signal store and scheduling operations as module variables. Declaration
//! identity and lexical scope remain separate from their storage identity;
//! linking relocates the owning process and storage together.
//!
//! Other locals stay in SSA: joins use block parameters, suspension uses resume
//! arguments. Blocking packed updates replace selected bits in the current SSA
//! value. All static locals initialize once at process startup and survive
//! suspension and process re-entry.
//!
//! Temporaries, SSA locals and captured blocking-assignment RHS values cross a
//! `Wait` as resume arguments. Stored locals are read again after resumption,
//! so deferred updates cannot be hidden by a stale carried value.
//!
//! This machinery is typed. Process-local `real` values use real storage or
//! real-typed SSA parameters, with initial value `0.0`; integral locals preserve
//! four-state bits and their declared width.
//!
//! # Numeric variable ownership
//!
//! Semantic analysis assigns module-level real/integer storage to its writing
//! context, excludes lexical shadows and refuses writes from both domains
//! (VAMS-2023 7.2.2). Analog reads of digital variables use state-input bindings;
//! digital reads of analog variables use retained evaluation probes.
//!
//! A digital real has no bits. A digital integer has signed [31:0] four-state
//! storage and retains its authored integer identity independently of a packed
//! reg's digital signedness. This distinction also survives hierarchy flattening.
//! Variables are written procedurally, without resolved net drivers. Unwritten
//! module numeric variables retain the compiler's existing analog-domain choice.
//!
//! # The subset
//!
//! Everything the front end parses is not everything this lowers. Refusals are
//! by name and with a span, so a model that crosses one is told what is
//! missing rather than compiled into a device that is quietly short of what
//! its author wrote. What still refuses here:
//!
//! Analog-owned scalar and array-element reads carry explicit typed probes.
//! The runtime binds them to values retained by normal analog evaluation.
//! Analog event subscriptions lowered by semantic analysis carry an occurrence
//! signal and a retained counter probe through the same hierarchy mappings.
//!
//! - A process-local `string`: a process computes in four-state and real
//!   values, and a string is neither.
//! - Partial unpacked-array values and array-valued function arguments.
//! - A non-constant part-select bound.
//!
//! Refused before this pass, and still refused: tasks and functions,
//! `fork`/`join`, `wait`, `disable`, and `force`/`release` — the parser stops
//! each on its own keyword, so none of them reaches a lowering decision.
//!
//! Nor does a generate region. IEEE 1364-2005 section 12.4 makes one an
//! elaboration-time construct, and the parser unrolls it into ordinary module
//! items at `endmodule`, so what arrives here is the flat result and there is
//! no generate anything to lower.
//!
//! # Hierarchy
//!
//! A hierarchical instance is no longer among them. It is resolved before this
//! pass by [`digital_elaborate`](crate::semantic::digital_elaborate), which
//! turns the instance tree into a flat list of frames, and this pass turns the
//! frames into one plan: one signal table, one process list, one driver list.
//! Nothing downstream sees a hierarchy — the interpreter is unchanged, and the
//! event kernel that follows it will be too.
//!
//! What survives flattening is identity. Each frame is lowered against its own
//! scope with its own freshly allocated process ids, so two instances of one
//! module are two sets of processes a scheduler can resume individually, and
//! two sets of drivers a resolver can tell apart, rather than one body lowered
//! twice.

mod checks;
mod connection_shapes;
mod constant_functions;
mod constants;
mod disable;
mod expressions;
mod local_arrays;
mod local_storage;
mod overrides;
mod references;
mod routines;
mod system_calls;
pub(crate) use connection_shapes::ConnectionShapes;
use constants::ResolvedConstants;
use system_calls::{CallTables, PlanCalls, ScopeFrame};

use super::cfg::{CfgTerminator, CfgValueKind, CfgValueType, CfgVariable, DigitalWait, SsaBuilder};
use super::diagnostic::{CompilerPhase, IrDiagnostic, SourceSpanRef};
use super::digital::{
    CanonicalDigitalPlan, CfgDigitalProcess, DigitalAnalogProbe, DigitalDriver, DigitalDriverId,
    DigitalEdge, DigitalProcessKind, DigitalRealResolution, DigitalSchedulingRegion,
    DigitalScopeKind, DigitalSensitivityOrigin, DigitalSensitivityTerm, DigitalSignal,
    DigitalSignalKind, DigitalStaticSensitivity, DigitalWriteSelect, DigitalWriteTarget,
};
use super::digital_value::{
    ArithmeticOp, BitwiseOp, DigitalCaseMatch, FourStateValue, LogicalOp, RealArithmeticOp,
    RealCompareOp, RelationalOp, ShiftOp,
};
use super::ids::{
    BlockId, DigitalAnalogProbeId, DigitalLocalId, DigitalProcessId, DigitalSignalId, ValueId,
};
use crate::analog_tasks::CDS_GET_ANALOG_VALUE;
use crate::ast::DigitalProcessKind as AstKind;
use crate::ast::{
    ArrayLiteralElement, BinaryOp, BranchAccess, DigitalAssign, DigitalCase, DigitalLValue,
    DigitalStatement, EdgeKind, Expression, IndexedPartSelectExpr, ReductionOp, TimingControl,
    UnaryOp, WrealResolution,
};
use crate::four_state::FourStateBit;
use crate::semantic::{
    AnalyzedDigital, AnalyzedDigitalProcess, AnalyzedDigitalSignal, DigitalConstants,
    INTEGER_BOUNDS, VectorBounds,
};
use crate::source::Span;
use smol_str::SmolStr;
use std::collections::{BTreeSet, HashMap, HashSet};

/// Front-end select validation uses the same typed constant rules as lowering.
/// Only the scalar index crosses this boundary; executable values remain here.
pub(crate) fn selector_constant(
    expression: &Expression,
    source: &DigitalConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
) -> Option<crate::numeric_literal::NumericLiteralValue> {
    let resolved = constants::resolve(source, time_scale, &[], &[], &[], [expression]).ok()?;
    constants::scalar(expression, &resolved, time_scale)
}

/// Constant evaluation for source elaboration, not procedural digital code.
pub(crate) fn elaboration_constant(
    expression: &Expression,
    source: &DigitalConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
) -> Option<crate::numeric_literal::NumericLiteralValue> {
    let (expression, _) = source.given.fold(expression).ok()?;
    selector_constant(&expression, source, time_scale)
}

pub(crate) fn parameter_override_literal(
    declaration: &crate::ast::ParameterDecl,
    source: &DigitalConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
) -> Result<Expression, String> {
    constants::override_literal(declaration, source, time_scale)
}

pub(crate) use constants::ElaborationDependencies;

pub(crate) fn digital_dependencies<'a>(
    digital: &AnalyzedDigital,
    shapes: impl IntoIterator<Item = &'a Expression>,
) -> Result<ElaborationDependencies, String> {
    constants::digital_dependencies(digital, shapes)
}

pub(crate) fn expression_dependencies<'a>(
    source: &DigitalConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
    expressions: impl IntoIterator<Item = &'a Expression>,
) -> Result<ElaborationDependencies, String> {
    constants::expression_dependencies(source, time_scale, expressions)
}

pub(crate) use constants::ParameterAssignment;

pub(crate) fn parameter_assignments(
    declarations: &[&crate::ast::ParameterDecl],
    given: &crate::semantic::parameter_given::GivenParameters,
    functions: &crate::semantic::ConstantFunctions,
    time_scale: crate::time_scale::ModuleTimeScale,
) -> Result<Vec<ParameterAssignment>, Vec<DigitalLoweringDiagnostic>> {
    constants::parameter_assignments(declarations, given, functions, time_scale)
}

/// Whether a diagnostic is the author's to fix or the compiler's.
///
/// The distinction decides what the author is told. A *refusal* is a construct
/// they wrote: legal enough to reach this pass, and one the discrete-domain
/// lowering has no form for. It is reported the way every other refused
/// construct is — a semantic error naming the construct at its offset. An
/// *invariant* is a property this pass established itself: a graph it built, a
/// scope it prepared, a classification it ran. Violating one can only mean the
/// compiler is broken, so it keeps the `Internal error` rendering that says so.
///
/// Telling an author that their `string` declaration is an internal error
/// tells them the wrong thing to do about it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DigitalLoweringClass {
    Refusal(DigitalRefusalKind),
    Invariant,
}

/// Which kind of refused program this is.
///
/// Two different defects wear the word "refused" and must not read the same.
/// A program that violates the standard is illegal in every tool that reads
/// it, and the author has to change it. A construct this lowering does not
/// build yet is legal Verilog that this compiler is behind on, and the author
/// may reasonably expect it to work one day. The analyzer already spells that
/// difference — [`SemanticErrorKind::InvalidContribution`] for what the
/// standard does not let drive a net, [`SemanticErrorKind::UnsupportedFeature`]
/// for what is not built — so a refusal raised here carries the same kind
/// rather than inventing a third, and one construct does not change its error
/// class according to which pass happened to catch it.
///
/// [`SemanticErrorKind::InvalidContribution`]: crate::error::SemanticErrorKind::InvalidContribution
/// [`SemanticErrorKind::UnsupportedFeature`]: crate::error::SemanticErrorKind::UnsupportedFeature
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DigitalRefusalKind {
    /// Legal Verilog the discrete-domain lowering has no form for.
    NotLowered,
    /// A driver the standard does not admit on the net it drives.
    InvalidContribution,
}

/// One diagnostic from the discrete-domain lowering, with the class that
/// decides how it reaches the author.
#[derive(Debug, Clone)]
pub(crate) struct DigitalLoweringDiagnostic {
    pub(crate) class: DigitalLoweringClass,
    pub(crate) diagnostic: IrDiagnostic,
}

impl DigitalLoweringDiagnostic {
    /// Refuse a construct this lowering has no form for yet.
    fn refusal(message: impl Into<String>, span: SourceSpanRef) -> Self {
        Self::refusal_with_kind(DigitalRefusalKind::NotLowered, message, span)
    }

    /// Refuse a construct the standard does not admit, naming which kind of
    /// illegality it is so the author is not told it is merely unsupported.
    fn refusal_with_kind(
        kind: DigitalRefusalKind,
        message: impl Into<String>,
        span: SourceSpanRef,
    ) -> Self {
        Self {
            class: DigitalLoweringClass::Refusal(kind),
            diagnostic: IrDiagnostic::error(CompilerPhase::CfgLowering, message, span),
        }
    }

    fn invariant(message: impl Into<String>, span: SourceSpanRef) -> Self {
        Self {
            class: DigitalLoweringClass::Invariant,
            diagnostic: IrDiagnostic::error(CompilerPhase::CfgLowering, message, span),
        }
    }

    /// Adopt a diagnostic raised by a pass that only checks its own output.
    fn from_invariant(diagnostic: IrDiagnostic) -> Self {
        Self {
            class: DigitalLoweringClass::Invariant,
            diagnostic,
        }
    }
}

/// Lower the analyzed discrete-domain content of a module.
///
/// Returns the plan on success, or every refusal at once — the same
/// accumulate-then-report discipline the rest of the front end uses, so an
/// author with three unsupported constructs learns about three.
///
/// The classification is dropped here: this entry point reports the raw
/// diagnostics, and the compiler's own path ([`lower_module`]) keeps it so the
/// author's constructs and the compiler's invariants are rendered apart.
pub fn lower(digital: &AnalyzedDigital) -> Result<CanonicalDigitalPlan, Vec<IrDiagnostic>> {
    lower_with_analog_variables(digital, &HashMap::new(), "top").map_err(|diagnostics| {
        diagnostics
            .into_iter()
            .map(|entry| entry.diagnostic)
            .collect()
    })
}

#[derive(Clone)]
struct AnalogVariable {
    declared_dimensions: Vec<(i64, i64)>,
    target: SmolStr,
    event_signal: Option<SmolStr>,
    event_assigned: bool,
    immutable: bool,
    quantity: super::digital::DigitalAnalogQuantity,
    array: Option<(i64, u32)>,
    dimensions: Vec<(i64, i64)>,
}

impl AnalogVariable {
    fn array_layout(&self) -> Option<crate::array_index::UnpackedArrayLayout> {
        let (lower, len) = self.array?;
        let dimensions = if self.dimensions.is_empty() {
            vec![(lower, lower.checked_add(i64::from(len) - 1)?)]
        } else {
            self.dimensions.clone()
        };
        let layout = crate::array_index::UnpackedArrayLayout::new(
            &dimensions,
            crate::semantic::MAX_PARAMETER_ARRAY_ELEMENTS as usize,
        )
        .ok()?;
        (layout.len() == len as usize).then_some(layout)
    }
}

pub(crate) fn lower_module(
    module: &crate::semantic::AnalyzedModule,
) -> Result<CanonicalDigitalPlan, Vec<DigitalLoweringDiagnostic>> {
    lower_with_analog_variables(&module.digital, &analog_variables(module), &module.name)
}

fn analog_variables(module: &crate::semantic::AnalyzedModule) -> HashMap<SmolStr, AnalogVariable> {
    use super::digital::DigitalAnalogQuantity;
    let mut variables: HashMap<_, _> = module
        .variables
        .iter()
        .filter_map(|variable| {
            if module
                .digital
                .signals
                .iter()
                .any(|signal| signal.name == variable.name)
            {
                return None;
            }
            let quantity = match variable.value_type {
                crate::ValueType::Real => DigitalAnalogQuantity::RealVariable,
                crate::ValueType::Integer => DigitalAnalogQuantity::IntegerVariable,
                _ => return None,
            };
            Some((
                variable.name.clone(),
                AnalogVariable {
                    declared_dimensions: Vec::new(),
                    target: variable.name.clone(),
                    event_signal: None,
                    event_assigned: module
                        .digital
                        .event_assigned_variables
                        .contains(&variable.name),
                    immutable: module
                        .digital
                        .immutable_analog_variables
                        .contains(&variable.name),
                    quantity,
                    array: None,
                    dimensions: Vec::new(),
                },
            ))
        })
        .collect();
    for (name, array) in &module.arrays {
        if let Some(element) = module.variables.get(array.base)
            && let Some(variable) = variables.get(&element.name).cloned()
        {
            variables.insert(
                name.clone(),
                AnalogVariable {
                    declared_dimensions: array.declared_dimensions.clone(),
                    target: name.clone(),
                    event_signal: None,
                    event_assigned: false,
                    immutable: false,
                    quantity: variable.quantity,
                    array: Some((array.lower, array.len as u32)),
                    dimensions: array.dimensions.clone(),
                },
            );
        }
    }
    bind_variable_events(
        &mut variables,
        &module.digital.analog_events,
        &module.digital.event_assigned_variables,
        &module.digital.immutable_analog_variables,
    );
    variables
}

fn bind_variable_events(
    variables: &mut HashMap<SmolStr, AnalogVariable>,
    bindings: &[crate::semantic::AnalogEventBinding],
    event_assigned: &[SmolStr],
    immutable: &[SmolStr],
) {
    for name in immutable {
        if let Some(variable) = variables.get_mut(name) {
            variable.immutable = true;
        }
    }
    for name in event_assigned {
        if let Some(variable) = variables.get_mut(name) {
            variable.event_assigned = true;
        }
    }
    for binding in bindings {
        if let Some(source) = &binding.source_variable {
            if let Some(variable) = variables.get_mut(source) {
                variable.event_signal = Some(binding.signal.clone());
            }
        }
    }
}

fn lower_with_analog_variables(
    digital: &AnalyzedDigital,
    analog_variables: &HashMap<SmolStr, AnalogVariable>,
    root_name: &str,
) -> Result<CanonicalDigitalPlan, Vec<DigitalLoweringDiagnostic>> {
    if digital.is_empty() {
        return Ok(CanonicalDigitalPlan::default());
    }

    let mut diagnostics = Vec::new();
    let timing = crate::time_scale::DigitalTiming {
        root: digital.time_scale,
        precision_exponent: digital.instances.iter().fold(
            digital.time_scale.precision_exponent(),
            |precision, instance| precision.min(instance.time_scale.precision_exponent()),
        ),
    };

    // ------------------------------------------------------------------
    // The elaborated signal table.
    //
    // Built whole before anything is lowered, because a process function
    // carries signal ids and a scope that hands out ids has to know all of
    // them. The compiled module's own signals keep their names and their
    // positions, so a design with no hierarchy produces exactly the table it
    // always did. Each instance frame then contributes the signals it declares
    // under the elaborated name the front end gave them — except a net port
    // that collapsed onto the net it connects to, which the front end named
    // after *that* net and which is therefore already in the table. Reusing
    // the entry is what collapsing is; there is no separate merge step.
    // ------------------------------------------------------------------
    let module_constants = constants::resolve(
        &digital.constants,
        digital.time_scale,
        &digital.processes,
        &digital.continuous_assigns,
        &digital.routines,
        digital
            .signals
            .iter()
            .flat_map(AnalyzedDigitalSignal::elaboration_expressions)
            .chain(digital.paths.iter().flat_map(|path| &path.delays))
            .chain(digital.timing_checks.iter().flat_map(check_constants)),
    )?;
    let instance_constants = digital
        .instances
        .iter()
        .map(|instance| {
            constants::resolve(
                &instance.constants,
                instance.time_scale,
                &instance.processes,
                &instance.continuous_assigns,
                &instance.routines,
                instance
                    .signals
                    .iter()
                    .flat_map(|signal| signal.declared.elaboration_expressions())
                    .chain(instance.paths.iter().flat_map(|path| &path.delays))
                    .chain(instance.timing_checks.iter().flat_map(check_constants)),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    // A declaration borrowed for a hierarchical reference takes no storage: it
    // is bound to its target's signal once every scope's signals exist.
    let root_references = &digital.signal_references;
    let (mut signals, mut arrays, mut root_signal_ids) = lower_signals(&digital.signals, |name| {
        references::is_reference(root_references, name)
    })?;
    let mut elaborated: HashMap<SmolStr, DigitalSignalId> = digital
        .signals
        .iter()
        .zip(&root_signal_ids)
        .filter(|(signal, _)| !references::is_reference(root_references, &signal.name))
        .map(|(signal, id)| (signal.name.clone(), *id))
        .collect();
    elaborated.extend(
        signals
            .iter()
            .map(|signal| (signal.name.clone(), signal.id)),
    );
    let root_names: HashSet<&str> = digital
        .signals
        .iter()
        .map(|signal| signal.name.as_str())
        .collect();
    let mut real_aliases = Vec::new();
    let mut frame_signal_ids = Vec::with_capacity(digital.instances.len());
    for instance in &digital.instances {
        let mut ids = vec![DigitalSignalId::new(0); instance.signals.len()];
        for (index, signal) in instance.signals.iter().enumerate() {
            if signal.declared.element_alias.is_some()
                || references::is_reference(&instance.signal_references, &signal.declared.name)
            {
                continue;
            }
            // An escaped root identifier may itself contain a period
            // (IEEE 1364-2005 section 3.7.1): `\bot.r ` is not the `r` of
            // instance `bot`, so an instance's own signal never reuses it.
            let own = signal
                .name
                .strip_prefix(instance.path.as_str())
                .and_then(|rest| rest.strip_prefix('.'))
                == Some(signal.declared.name.as_str());
            let shadowed = own && root_names.contains(signal.name.as_str());
            if shadowed {
                // The root's signal takes its escaped spelling, so the plan's
                // names stay unique and print as the author wrote them.
                let name = signal.name.as_str();
                let escaped = format!("\\{name} ");
                for cell in &mut signals {
                    if cell.name == name {
                        cell.name = escaped.clone().into();
                    } else if let Some(rest) = cell.name.strip_prefix(name)
                        && rest.starts_with('[')
                    {
                        cell.name = format!("{escaped}{rest}").into();
                    }
                }
                for array in &mut arrays {
                    if array.name == name {
                        array.name = escaped.clone().into();
                    }
                }
            }
            let existing = elaborated.get(&signal.name).filter(|_| !shadowed);
            let id = match existing {
                Some(existing) => *existing,
                None => {
                    let start = signals.len();
                    let id = append_signal(
                        &signal.declared,
                        signal.name.clone(),
                        &mut signals,
                        &mut arrays,
                    );
                    elaborated.insert(signal.name.clone(), id);
                    elaborated.extend(
                        signals[start..]
                            .iter()
                            .map(|cell| (cell.name.clone(), cell.id)),
                    );
                    id
                }
            };
            ids[index] = id;
        }
        for (index, signal) in instance.signals.iter().enumerate() {
            let Some(alias) = &signal.declared.element_alias else {
                continue;
            };
            let base = elaborated.get(&alias.array).copied().ok_or_else(|| {
                vec![DigitalLoweringDiagnostic::invariant(
                    format!("real array view '{}' has no allocated array", signal.name),
                    signal.declared.span.into(),
                )]
            })?;
            let array = arrays.iter().find(|array| array.storage.base == base);
            if array.is_none_or(|array| alias.offset >= array.storage.len) {
                return Err(vec![DigitalLoweringDiagnostic::invariant(
                    "real array view is outside its storage",
                    signal.declared.span.into(),
                )]);
            }
            let cell = DigitalSignalId::new(base.index() + alias.offset);
            let id = *elaborated.entry(signal.name.clone()).or_insert(cell);
            if id != cell {
                real_aliases.push(super::digital::DigitalRealAlias {
                    left: cell,
                    right: id,
                    span: signal.declared.span.into(),
                });
            }
            ids[index] = id;
        }
        frame_signal_ids.push(ids);
    }
    references::bind(digital, &mut root_signal_ids, &mut frame_signal_ids)?;
    // Net types other than `wire` (IEEE 1364-2005 section 4.6), by the signal
    // each declaration lowered to; `uwire`s are checked once drivers exist.
    let mut typed_nets = Vec::new();
    let mut uwires = Vec::new();
    for (declared, id, time_scale) in digital
        .signals
        .iter()
        .zip(&root_signal_ids)
        .filter(|(declared, _)| !references::is_reference(root_references, &declared.name))
        .map(|(declared, id)| (declared, id, digital.time_scale))
        .chain(
            digital
                .instances
                .iter()
                .zip(&frame_signal_ids)
                .flat_map(|(instance, ids)| {
                    instance
                        .signals
                        .iter()
                        .map(|signal| &signal.declared)
                        .zip(ids)
                        .filter(|(declared, _)| declared.element_alias.is_none())
                        .map(|(declared, id)| (declared, id, instance.time_scale))
                }),
        )
    {
        let crate::semantic::DigitalSignalClass::Net(kind) = declared.class else {
            continue;
        };
        use super::digital::DigitalNetType;
        let net_type = match kind {
            crate::ast::DigitalNetKind::Trireg { charge, decay } => {
                // Section 7.14.2: the decay time in module units, rounded at
                // the module's precision like any delay. One no run reaches
                // never decays.
                let ticks = decay
                    .map(|decay| time_scale.delay_ticks(decay.units(), timing.precision_exponent));
                let decay = match ticks {
                    None | Some(Ok(crate::time_scale::DELAY_BEYOND_HORIZON)) => None,
                    Some(Ok(ticks)) if ticks > 0 => Some(super::digital::DigitalChargeDecay {
                        ticks: ticks as u64,
                        exponent: timing.precision_exponent,
                    }),
                    Some(_) => {
                        diagnostics.push(DigitalLoweringDiagnostic::refusal(
                            format!(
                                "the charge decay time of `{}` rounds to no time at its \
                                 module's precision (IEEE 1364-2005 section 7.14.2)",
                                declared.name
                            ),
                            declared.span.into(),
                        ));
                        continue;
                    }
                };
                DigitalNetType::Trireg { charge, decay }
            }
            crate::ast::DigitalNetKind::Wand => DigitalNetType::Wand,
            crate::ast::DigitalNetKind::Wor => DigitalNetType::Wor,
            crate::ast::DigitalNetKind::Tri0 => DigitalNetType::Tri0,
            crate::ast::DigitalNetKind::Tri1 => DigitalNetType::Tri1,
            crate::ast::DigitalNetKind::Supply0 => DigitalNetType::Supply0,
            crate::ast::DigitalNetKind::Supply1 => DigitalNetType::Supply1,
            crate::ast::DigitalNetKind::Uwire => {
                uwires.push(*id);
                continue;
            }
            _ => continue,
        };
        if declared.unpacked.is_some() {
            diagnostics.push(DigitalLoweringDiagnostic::refusal(
                format!(
                    "`{}` is an array of `{}` nets, which has no lowered form; declare each \
                     element as its own net",
                    declared.name,
                    kind.keyword()
                ),
                declared.span.into(),
            ));
            continue;
        }
        typed_nets.push(super::digital::DigitalTypedNet {
            signal: *id,
            net_type,
        });
    }

    let mut net_delays = Vec::new();
    for (declared, id) in digital.signals.iter().zip(&root_signal_ids) {
        if references::is_reference(root_references, &declared.name) {
            continue;
        }
        initialize_signal(
            declared,
            *id,
            &module_constants,
            digital.time_scale,
            &mut signals,
        )?;
        net_delay(
            declared,
            *id,
            (&module_constants, digital.time_scale),
            timing.precision_exponent,
            &mut net_delays,
        )
        .map_err(|diagnostic| vec![diagnostic])?;
    }
    for ((instance, ids), constants) in digital
        .instances
        .iter()
        .zip(&frame_signal_ids)
        .zip(&instance_constants)
    {
        for (declared, id) in instance.signals.iter().zip(ids) {
            if declared.declared.initializer.is_some() {
                initialize_signal(
                    &declared.declared,
                    *id,
                    constants,
                    instance.time_scale,
                    &mut signals,
                )?;
            }
            if declared.declared.element_alias.is_none() {
                net_delay(
                    &declared.declared,
                    *id,
                    (constants, instance.time_scale),
                    timing.precision_exponent,
                    &mut net_delays,
                )
                .map_err(|diagnostic| vec![diagnostic])?;
            }
        }
    }

    // One scope per frame, each mapping the names *that frame's* body writes to
    // the elaborated ids they resolve to. The instance's body is lowered
    // unmodified against its own scope, which is what keeps two instances of
    // one module two separately addressable things rather than one body
    // rewritten twice.
    let module_scope: HashMap<&str, DigitalSignalId> = digital
        .signals
        .iter()
        .zip(&root_signal_ids)
        .map(|(analyzed, id)| (analyzed.name.as_str(), *id))
        .collect();
    let frame_scopes: Vec<HashMap<&str, DigitalSignalId>> = digital
        .instances
        .iter()
        .zip(&frame_signal_ids)
        .map(|(instance, ids)| {
            instance
                .signals
                .iter()
                .zip(ids)
                .map(|(signal, id)| (signal.declared.name.as_str(), *id))
                .collect()
        })
        .collect();
    // The scope an implicit port driver resolves in. It names both sides of a
    // connection, which live in two different instances, so it is the only one
    // keyed by elaborated name.
    let elaborated_names: Vec<_> = signals
        .iter()
        .map(|signal| (signal.name.clone(), signal.id))
        .collect();
    let elaborated_scope: HashMap<&str, DigitalSignalId> = elaborated_names
        .iter()
        .map(|(name, id)| (name.as_str(), *id))
        .collect();

    // Module path delays (IEEE 1364-2005 section 14), each frame's against
    // its own scope and constants.
    let mut paths = Vec::new();
    let mut timing_checks = Vec::new();
    let frames = std::iter::once((
        (&digital.paths, &digital.timing_checks, ""),
        &module_scope,
        &[][..],
        (&module_constants, digital.time_scale),
    ))
    .chain(
        digital
            .instances
            .iter()
            .zip(&frame_scopes)
            .zip(&instance_constants)
            .map(|((instance, scope), constants)| {
                (
                    (
                        &instance.paths,
                        &instance.timing_checks,
                        instance.path.as_str(),
                    ),
                    scope,
                    &instance.port_drivers[..],
                    (constants, instance.time_scale),
                )
            }),
    );
    for ((frame_paths, frame_checks, instance), scope, port_drivers, constants) in frames {
        lower_paths(
            frame_paths,
            scope,
            port_drivers,
            constants,
            (&signals, &elaborated_scope),
            timing.precision_exponent,
            &mut paths,
        )
        .map_err(|diagnostic| vec![diagnostic])?;
        checks::lower_timing_checks(
            frame_checks,
            (scope, instance, &signals),
            constants,
            timing.precision_exponent,
            (&mut timing_checks, &mut paths),
        )
        .map_err(|diagnostic| vec![diagnostic])?;
    }

    // ------------------------------------------------------------------
    // Processes and drivers.
    // ------------------------------------------------------------------
    // Allocate dense canonical IDs after elaboration: generate expansion can
    // leave gaps in source IDs. The fixed order is root processes, root
    // assignments, then each frame's processes, assignments and port drivers.
    let mut next_id = 0usize;
    let mut allocate = move || {
        let id = DigitalProcessId::from(next_id);
        next_id += 1;
        id
    };

    // An instance frame's body is lowered against *its own* module's constants,
    // never against the module the artifact is being built for. The two tables
    // never meet: folding a child's `WIDTH` with the parent's would be a wrong
    // answer rather than a refused one, and a child whose `{WIDTH{1'b0}}`
    // resolved to the parent's number would compile into a device silently the
    // wrong width.
    //
    // A synthesized port driver is lowered against no constants at all, because
    // it belongs to neither scope: this pass wrote it, in elaborated names, and
    // the only expressions in one are a name and a select whose bounds are
    // already literals.
    let array_storage: HashMap<_, _> = arrays
        .iter()
        .map(|array| (array.storage.base, array.clone()))
        .collect();
    let module_array_names: HashSet<_> = digital
        .signals
        .iter()
        .filter(|signal| signal.unpacked.is_some())
        .map(|signal| signal.name.clone())
        .collect();
    let elaborated_array_names: HashSet<_> =
        arrays.iter().map(|array| array.name.clone()).collect();
    let no_constants = ResolvedConstants::default();
    let no_analog_variables = HashMap::new();

    let mut processes = Vec::new();
    let mut drivers = Vec::new();
    // Drive strengths and gated drivers; the net types join them below.
    let mut driver_strengths = super::digital::DigitalStrengthPlan::default();
    let mut driver_delays = DriverDelays {
        precision: timing.precision_exponent,
        delays: Vec::new(),
    };
    // One probe table for the whole plan, in first-appearance order over the
    // same fixed traversal the process and driver numbering follows, so a
    // probe keeps its id across a recompilation for the reason a driver keeps
    // its index.
    let mut probes = Vec::new();
    // The scopes `%m` names: the root module, then each instance frame in the
    // elaboration's depth-first order, so a parent always precedes its child.
    let mut calls = PlanCalls {
        disabled: std::sync::Arc::new(disable::disabled_names(
            std::iter::once((&digital.processes, &digital.routines))
                .chain(
                    digital
                        .instances
                        .iter()
                        .map(|instance| (&instance.processes, &instance.routines)),
                )
                .flat_map(|(processes, routines)| {
                    processes
                        .iter()
                        .map(|process| &process.body)
                        .chain(routines.iter().map(|routine| &routine.routine.body))
                }),
        )),
        ..PlanCalls::default()
    };
    // A library binding's root is known by its `library.cell` (`%l`).
    let root_definition = digital
        .definition
        .clone()
        .unwrap_or_else(|| root_name.into());
    let root_scope =
        calls.add_module_scope(None, root_name.into(), root_definition, digital.time_scale);
    let frame_scope_frames: Vec<ScopeFrame> = digital
        .instances
        .iter()
        .map(|instance| {
            let path: SmolStr = format!("{root_name}.{}", instance.path).into();
            let parent = path
                .rsplit_once('.')
                .and_then(|(parent, _)| calls.module_scopes.get(parent).copied());
            calls.add_module_scope(parent, path, instance.module.clone(), instance.time_scale)
        })
        .collect();
    // Tasks and functions, numbered per scope before any body is lowered so
    // that a body may call any routine of its scope, itself included.
    let mut routine_count = 0;
    let mut plan_routines = Vec::new();
    let mut root_routines = routines::signatures(
        &digital.routines,
        &mut routine_count,
        &module_constants,
        digital.time_scale,
        &mut diagnostics,
    );
    let mut instance_routines: Vec<_> = digital
        .instances
        .iter()
        .zip(&instance_constants)
        .map(|(instance, constants)| {
            routines::signatures(
                &instance.routines,
                &mut routine_count,
                constants,
                instance.time_scale,
                &mut diagnostics,
            )
        })
        .collect();
    routines::bind_downward_names(digital, &mut root_routines, &mut instance_routines);
    for routine in &digital.routines {
        match routines::lower_routine(
            routine,
            routines::RoutineScope {
                signals: &mut signals,
                arrays: &array_storage,
                index: &module_scope,
                array_names: &module_array_names,
                constants: &module_constants,
                analog_variables,
                probes: &mut probes,
                time_scale: digital.time_scale,
                calls: &mut calls,
                scope: &root_scope,
                routines: &root_routines,
                prefix: "",
                constant: false,
            },
        ) {
            Ok(lowered) => plan_routines.push(lowered),
            Err(mut errors) => diagnostics.append(&mut errors),
        }
    }
    for process in &digital.processes {
        match lower_process(
            process,
            allocate(),
            &mut signals,
            &array_storage,
            &mut arrays,
            &module_scope,
            &module_array_names,
            &module_constants,
            analog_variables,
            &mut probes,
            digital.time_scale,
            &mut calls,
            &root_scope,
            &root_routines,
        ) {
            Ok(lowered) => processes.push(lowered),
            Err(mut errors) => diagnostics.append(&mut errors),
        }
    }
    for assignment in &digital.continuous_assigns {
        match lower_continuous_assign(
            assignment,
            &mut signals,
            &array_storage,
            &module_scope,
            &module_array_names,
            &module_constants,
            analog_variables,
            allocate(),
            (&mut drivers, &mut driver_strengths, &mut driver_delays),
            &mut probes,
            digital.time_scale,
            &root_routines,
        ) {
            Ok(lowered) => processes.push(lowered),
            Err(mut errors) => diagnostics.append(&mut errors),
        }
    }
    for ((((instance, scope), constants), frame_scope), frame_routines) in digital
        .instances
        .iter()
        .zip(&frame_scopes)
        .zip(&instance_constants)
        .zip(&frame_scope_frames)
        .zip(&instance_routines)
    {
        let frame_array_names: HashSet<_> = instance
            .signals
            .iter()
            .filter(|signal| signal.declared.unpacked.is_some())
            .map(|signal| signal.declared.name.clone())
            .collect();
        let mut frame_variables: HashMap<_, _> = instance
            .analog_variables
            .iter()
            .filter_map(|(local, global)| {
                analog_variables
                    .get(global)
                    .map(|variable| (local.clone(), variable.clone()))
            })
            .collect();
        bind_variable_events(
            &mut frame_variables,
            &instance.analog_events,
            &instance.event_assigned_variables,
            &instance.immutable_analog_variables,
        );
        let prefix = format!("{}.", instance.path);
        for routine in &instance.routines {
            match routines::lower_routine(
                routine,
                routines::RoutineScope {
                    signals: &mut signals,
                    arrays: &array_storage,
                    index: scope,
                    array_names: &frame_array_names,
                    constants,
                    analog_variables: &frame_variables,
                    probes: &mut probes,
                    time_scale: instance.time_scale,
                    calls: &mut calls,
                    scope: frame_scope,
                    routines: frame_routines,
                    prefix: &prefix,
                    constant: false,
                },
            ) {
                Ok(lowered) => plan_routines.push(lowered),
                Err(mut errors) => diagnostics.append(&mut errors),
            }
        }
        for process in &instance.processes {
            match lower_process(
                process,
                allocate(),
                &mut signals,
                &array_storage,
                &mut arrays,
                scope,
                &frame_array_names,
                constants,
                &frame_variables,
                &mut probes,
                instance.time_scale,
                &mut calls,
                frame_scope,
                frame_routines,
            ) {
                Ok(lowered) => processes.push(lowered),
                Err(mut errors) => diagnostics.append(&mut errors),
            }
        }
        for assignment in &instance.continuous_assigns {
            match lower_continuous_assign(
                assignment,
                &mut signals,
                &array_storage,
                scope,
                &frame_array_names,
                constants,
                &frame_variables,
                allocate(),
                (&mut drivers, &mut driver_strengths, &mut driver_delays),
                &mut probes,
                instance.time_scale,
                frame_routines,
            ) {
                Ok(lowered) => processes.push(lowered),
                Err(mut errors) => diagnostics.append(&mut errors),
            }
        }
        for assignment in &instance.port_drivers {
            match lower_continuous_assign(
                assignment,
                &mut signals,
                &array_storage,
                &elaborated_scope,
                &elaborated_array_names,
                &no_constants,
                &no_analog_variables,
                allocate(),
                (&mut drivers, &mut driver_strengths, &mut driver_delays),
                &mut probes,
                instance.time_scale,
                &routines::NO_ROUTINES,
            ) {
                Ok(lowered) => processes.push(lowered),
                Err(mut errors) => diagnostics.append(&mut errors),
            }
        }
    }

    disable::resolve_disables(
        &mut processes,
        &mut plan_routines,
        &calls.scopes,
        std::mem::take(&mut calls.pending_disables),
        &mut diagnostics,
    );
    overrides::subscribe_routine_reads(&mut calls.overrides, &plan_routines);

    let mut bit_aliases = Vec::new();
    for alias in digital.bit_aliases.iter().chain(
        digital
            .instances
            .iter()
            .flat_map(|instance| &instance.bit_aliases),
    ) {
        let (Some(&left), Some(&right)) = (
            elaborated_scope.get(alias.left.as_str()),
            if alias.right_element.is_some() {
                elaborated.get(&alias.right)
            } else {
                elaborated_scope.get(alias.right.as_str())
            },
        ) else {
            diagnostics.push(DigitalLoweringDiagnostic::invariant(
                "wire-bit alias has no elaborated signal",
                alias.span.into(),
            ));
            continue;
        };
        let right = if let Some(offset) = alias.right_element {
            let Some(array) = array_storage
                .get(&right)
                .filter(|array| offset < array.storage.len)
            else {
                diagnostics.push(DigitalLoweringDiagnostic::invariant(
                    "wire-bit alias has no in-range unpacked element",
                    alias.span.into(),
                ));
                continue;
            };
            DigitalSignalId::new(array.storage.base.index() + offset)
        } else {
            right
        };
        bit_aliases.push(super::digital::DigitalBitAlias {
            left: super::digital::DigitalNetBit {
                signal: left,
                bit: alias.left_bit,
            },
            right: super::digital::DigitalNetBit {
                signal: right,
                bit: alias.right_bit,
            },
            span: alias.span.into(),
        });
    }

    let mut absdelta = Vec::new();
    let mut add_events = |bindings: &[crate::semantic::AnalogEventBinding],
                          scope: &HashMap<&str, DigitalSignalId>,
                          variables: &HashMap<SmolStr, AnalogVariable>,
                          time_scale: crate::time_scale::ModuleTimeScale| {
        for binding in bindings {
            if let Some(operands) = &binding.observation {
                let Some(signal) = scope.get(binding.signal.as_str()).copied() else {
                    diagnostics.push(DigitalLoweringDiagnostic::invariant(
                        "absdelta has no occurrence signal",
                        binding.span.into(),
                    ));
                    continue;
                };
                let mut ids = [DigitalAnalogProbeId::from(0); 5];
                let mut complete = true;
                for (name, id) in operands.iter().zip(&mut ids) {
                    let Some(variable) = variables.get(name) else {
                        diagnostics.push(DigitalLoweringDiagnostic::invariant(
                            "absdelta operand has no relocated analog storage",
                            binding.span.into(),
                        ));
                        complete = false;
                        break;
                    };
                    *id = DigitalAnalogProbeId::from(probes.len());
                    probes.push(DigitalAnalogProbe {
                        id: *id,
                        retained: false,
                        event_signal: None,
                        access: "$absdelta_operand".into(),
                        quantity: super::digital::DigitalAnalogQuantity::RealVariable,
                        target: super::digital::DigitalAnalogProbeTarget::Variable {
                            name: variable.target.clone(),
                        },
                        span: binding.span.into(),
                    });
                }
                if complete {
                    absdelta.push(super::digital::DigitalAbsDeltaObserver {
                        time_scale,
                        signal,
                        operands: ids,
                        span: binding.span.into(),
                    });
                }
                continue;
            }
            let Some(variable) = variables.get(&binding.variable) else {
                diagnostics.push(DigitalLoweringDiagnostic::invariant(
                    "analog event counter has no relocated storage",
                    binding.span.into(),
                ));
                continue;
            };
            let Some(signal) = scope.get(binding.signal.as_str()).copied() else {
                diagnostics.push(DigitalLoweringDiagnostic::invariant(
                    "analog event counter has no signal",
                    binding.span.into(),
                ));
                continue;
            };
            let (_, len) = variable.array.unwrap_or((0, 1));
            let layout = variable.array_layout();
            for offset in 0..len {
                let signal = DigitalSignalId::new(signal.index() + offset);
                signals[usize::from(signal)].initial_value = Some(
                    super::digital::DigitalInitialValue::FourState(FourStateValue::from_u64(32, 0)),
                );
                let target = if binding.array {
                    crate::array_index::element_name(
                        &variable.target,
                        layout.as_ref().expect("validated occurrence array"),
                        offset as usize,
                    )
                    .into()
                } else {
                    variable.target.clone()
                };
                probes.push(DigitalAnalogProbe {
                    retained: true,
                    event_signal: Some(signal),
                    id: DigitalAnalogProbeId::from(probes.len()),
                    access: "$analog_event".into(),
                    quantity: super::digital::DigitalAnalogQuantity::IntegerVariable,
                    target: super::digital::DigitalAnalogProbeTarget::Variable { name: target },
                    span: binding.span.into(),
                });
            }
        }
    };
    add_events(
        &digital.analog_events,
        &module_scope,
        analog_variables,
        digital.time_scale,
    );
    for (instance, scope) in digital.instances.iter().zip(&frame_scopes) {
        let variables = instance
            .analog_variables
            .iter()
            .filter_map(|(local, global)| {
                analog_variables
                    .get(global)
                    .map(|v| (local.clone(), v.clone()))
            })
            .collect();
        add_events(
            &instance.analog_events,
            scope,
            &variables,
            instance.time_scale,
        );
    }
    diagnostics.extend(reject_overdriven_real_nets(&signals, &drivers));
    // IEEE 1364-2005 section 4.6.2: a `uwire` admits one driver per bit, so
    // two drivers of different bits are legal.
    for uwire in uwires {
        let range = signals[usize::from(uwire)].declared_range();
        let mut driven = BTreeSet::new();
        let second = drivers
            .iter()
            .filter(|driver| driver.id.signal == uwire)
            .find(|driver| {
                let low = driver.target.select.low_position(range);
                let width = driver.target.select.checked_width(range).unwrap_or(0);
                (low..low + i64::from(width))
                    .map(|bit| driven.insert(bit))
                    .fold(false, |clash, inserted| clash | !inserted)
            });
        if let Some(second) = second {
            diagnostics.push(DigitalLoweringDiagnostic::refusal_with_kind(
                DigitalRefusalKind::InvalidContribution,
                format!(
                    "`{}` is a `uwire` with more than one driver; IEEE 1364-2005 section 4.6.2 \
                     permits a `uwire` one driver of each bit, so declare it `wire` to resolve \
                     several",
                    signals[usize::from(uwire)].name
                ),
                second.span,
            ));
        }
    }

    // A `trireg` joined to another net would resolve as part of that net,
    // without its charge (IEEE 1364-2005 section 4.6.3), so it is refused
    // rather than run as a plain wire.
    for net in &typed_nets {
        if !matches!(net.net_type, super::digital::DigitalNetType::Trireg { .. }) {
            continue;
        }
        if let Some(alias) = bit_aliases
            .iter()
            .find(|alias| alias.left.signal == net.signal || alias.right.signal == net.signal)
        {
            diagnostics.push(DigitalLoweringDiagnostic::refusal(
                format!(
                    "`{}` is a `trireg` connected to another net through a port; a charge \
                     storage net joined to another net has no lowered form yet, so keep the \
                     `trireg` and what drives it in one module",
                    signals[usize::from(net.signal)].name
                ),
                alias.span,
            ));
        }
    }

    // A net delay is a stage on the value the net's own drivers resolve to
    // (IEEE 1364-2005 section 6.1.3). A net joined to another through a port
    // or a bidirectional switch resolves with that net, so it has no stage of
    // its own and is refused rather than run undelayed.
    let joined = |signal: DigitalSignalId| {
        bit_aliases
            .iter()
            .find(|alias| alias.left.signal == signal || alias.right.signal == signal)
            .map(|alias| alias.span)
            .or_else(|| {
                driver_strengths
                    .gates
                    .iter()
                    .filter(|gate| {
                        matches!(gate.kind, super::digital::DigitalGateKind::Tran { .. })
                            && (gate.driver.signal == signal
                                || gate
                                    .source
                                    .as_ref()
                                    .is_some_and(|source| source.signal == signal))
                    })
                    .find_map(|gate| drivers.iter().find(|driver| driver.id == gate.driver))
                    .map(|driver| driver.span)
            })
    };
    // A module path is the same stage, and one stage per net: a path to a
    // net with a net delay of its own is refused too.
    for path in &paths {
        let name = &signals[usize::from(path.destination)].name;
        if let Some(span) = joined(path.destination) {
            diagnostics.push(DigitalLoweringDiagnostic::refusal(
                format!(
                    "module path destination `{name}` is joined to another net through a port \
                     or a bidirectional switch; a path to a joined net has no lowered form"
                ),
                span,
            ));
        } else if net_delays
            .iter()
            .any(|delayed| delayed.signal == path.destination)
        {
            diagnostics.push(DigitalLoweringDiagnostic::refusal(
                format!(
                    "module path destination `{name}` has a net delay; a net with both has no \
                     lowered form, so delay its drivers instead"
                ),
                signals[usize::from(path.destination)].span,
            ));
        }
    }
    for delayed in &net_delays {
        if let Some(span) = joined(delayed.signal) {
            diagnostics.push(DigitalLoweringDiagnostic::refusal(
                format!(
                    "`{}` has a net delay and is joined to another net through a port or a \
                     bidirectional switch; a net delay on a joined net has no lowered form, \
                     so delay its drivers instead",
                    signals[usize::from(delayed.signal)].name
                ),
                span,
            ));
        }
    }

    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }
    CanonicalDigitalPlan {
        timing,
        content_identity: [0; 32],
        elaboration_parameters: std::iter::once(("", &digital.elaboration_parameters))
            .chain(
                digital
                    .instances
                    .iter()
                    .map(|instance| (instance.path.as_str(), &instance.elaboration_parameters)),
            )
            .flat_map(|(path, parameters)| {
                parameters.iter().map(move |parameter| {
                    let qualify = |name: &SmolStr| {
                        if path.is_empty() {
                            name.clone()
                        } else {
                            format!("{path}.{name}").into()
                        }
                    };
                    super::digital::DigitalElaborationParameter {
                        is_given: parameter.is_given,
                        name: qualify(&parameter.name),
                        aliases: parameter.aliases.iter().map(qualify).collect(),
                        is_public: path.is_empty() && parameter.is_public,
                        scope: parameter.scope,
                        also_model: parameter.also_model,
                        value: super::digital_value::FourStateValue::from_literal(&parameter.value),
                        signed: parameter.value.signed,
                        bounds: parameter.bounds,
                    }
                })
            })
            .collect(),
        arrays,
        bit_aliases,
        real_aliases,
        signals,
        processes,
        drivers,
        analog_probes: probes,
        absdelta,
        scopes: calls.scopes,
        system_calls: calls.sites,
        observers: calls.observers,
        routines: plan_routines,
        overrides: calls.overrides,
        strength: super::digital::DigitalStrengthPlan {
            nets: typed_nets,
            ..driver_strengths
        },
        delays: super::digital::DigitalDelayPlan {
            drivers: driver_delays.delays,
            nets: net_delays,
            paths,
            checks: timing_checks,
        },
    }
    .seal()
    // Structural validation of the plan this pass just built, reached only
    // after every construct in it was accepted: an author cannot write their
    // way to one of these.
    .map_err(|diagnostics| {
        diagnostics
            .into_iter()
            .map(DigitalLoweringDiagnostic::from_invariant)
            .collect()
    })
}

/// Refuse a plain `wreal`, or a net of a real nettype, that more than one
/// driver drives.
///
/// Verilog-AMS LRM 2.4 section 6.5.3: "There can be a maximum of one driver of
/// a real-valued net." Section 3.7 gives no resolution function to combine two
/// with — the LRM has none — so a second driver is a refusal and not a
/// resolution problem, and refusing it here is the same kind of check as
/// refusing a driver on an `input` port: a property of the declarations, fixed
/// before anything runs.
///
/// Which is why this is *not* the store's job, and does not contradict the
/// split that keeps IEEE 1364-2005 table 4-1 in the kernel. The store owns
/// what value a net takes from the drivers it legally has; this owns whether
/// the net may have them. A net declared `wrealsum`, `wrealavg`, `wrealmin` or
/// `wrealmax` says it may, and is not checked here at all — its fold is the
/// store's, exactly as a `wire`'s is.
fn reject_overdriven_real_nets(
    signals: &[DigitalSignal],
    drivers: &[DigitalDriver],
) -> Vec<DigitalLoweringDiagnostic> {
    let mut diagnostics = Vec::new();
    for signal in signals {
        if signal.kind != DigitalSignalKind::Real(DigitalRealResolution::Single) {
            continue;
        }
        // A real *variable* is not a net and has no drivers to count: section
        // 6.2 keeps a continuous assignment off it, and the analyzer already
        // refuses one by name. Skipping it here keeps this check about what it
        // says it is about — how many drivers a net may have.
        if signal.procedurally_assignable {
            continue;
        }
        let on_this_net: Vec<&DigitalDriver> = drivers
            .iter()
            .filter(|driver| driver.id.signal == signal.id)
            .collect();
        let Some(second) = on_this_net.get(1) else {
            continue;
        };
        let count = on_this_net.len();
        // A real nettype (IEEE 1800-2017 section 6.6.7) declared without a
        // resolution function admits one driver, and `wreal_resolution` does
        // not reach it, so the refusal names each driver where it stands.
        if signal.nettype {
            for (index, driver) in on_this_net.iter().enumerate() {
                diagnostics.push(DigitalLoweringDiagnostic::refusal_with_kind(
                    DigitalRefusalKind::InvalidContribution,
                    format!(
                        "`{}` is a net of a real nettype with no resolution function, and this \
                         is driver {} of its {count}; IEEE 1800-2017 section 6.6.7 permits such \
                         a net one driver, and `wreal_resolution` does not apply to it",
                        signal.name,
                        index + 1,
                    ),
                    driver.span,
                ));
            }
            continue;
        }
        // Not `UnsupportedFeature`: the LRM admits one driver of a real-valued
        // net and this design has two, so the program is illegal rather than
        // ahead of the compiler.
        diagnostics.push(DigitalLoweringDiagnostic::refusal_with_kind(
            DigitalRefusalKind::InvalidContribution,
            format!(
                "cannot contribute to `{}`, which is a `wreal` with {count} drivers; Verilog-AMS LRM 2.4 section \
                 6.5.3 permits a maximum of one driver of a real-valued net, and the standard \
                 defines no resolution to combine two — declare it `wrealsum`, `wrealavg`, \
                 `wrealmin` or `wrealmax`, or resolve every plain `wreal` with `.options \
                 wreal_resolution=sum|avg|min|max`, to say which one you want",
                signal.name
            ),
            second.span,
        ));
    }
    diagnostics
}

/// A net's own delay (IEEE 1364-2005 section 6.1.3), as the plan's entry for
/// the signal the net lowered to.
fn net_delay(
    declared: &AnalyzedDigitalSignal,
    signal: DigitalSignalId,
    (constants, time_scale): (&ResolvedConstants, crate::time_scale::ModuleTimeScale),
    precision: i8,
    delays: &mut Vec<super::digital::DigitalNetDelay>,
) -> Result<(), DigitalLoweringDiagnostic> {
    let Some(delay) = &declared.delay else {
        return Ok(());
    };
    if declared.class.is_real() || declared.unpacked.is_some() {
        return Err(DigitalLoweringDiagnostic::refusal(
            format!(
                "`{}` is a real net or an array of nets with a net delay; IEEE 1364-2005 \
                 section 7.14 gives transition delays to four-state values, and a net \
                 delay here belongs to a single net",
                declared.name
            ),
            delay.span.into(),
        ));
    }
    if let Some(delay) = transition_delay(delay, constants, time_scale, precision)? {
        delays.push(super::digital::DigitalNetDelay { signal, delay });
    }
    Ok(())
}

/// One delay value in ticks of `precision`: an elaboration constant in its
/// module's time units, rounded to the module's precision like any delay.
/// `section` is the IEEE 1364-2005 section a refusal cites.
fn delay_value_ticks(
    value: &Expression,
    (constants, time_scale): (&ResolvedConstants, crate::time_scale::ModuleTimeScale),
    precision: i8,
    section: &str,
) -> Result<u64, DigitalLoweringDiagnostic> {
    use crate::numeric_literal::NumericLiteralValue::{Integer, Real};
    let refuse = |what: &str| {
        DigitalLoweringDiagnostic::refusal(
            format!("{what} (IEEE 1364-2005 section {section})"),
            value.span().into(),
        )
    };
    let converted = match constants::scalar(value, constants, time_scale) {
        Some(Integer(units)) if units >= 0 => {
            time_scale.integer_delay_ticks(units as u64, precision)
        }
        Some(Real(units)) if units >= 0.0 => time_scale.delay_ticks(units, precision),
        Some(_) => return Err(refuse("a negative delay")),
        None => {
            return Err(refuse(
                "a delay that is not a constant expression; a driver's, a net's or a module \
                 path's delay is fixed at elaboration",
            ));
        }
    };
    match converted {
        Ok(value) if value != crate::time_scale::DELAY_BEYOND_HORIZON => Ok(value as u64),
        _ => Err(refuse("a delay too long for the design's time")),
    }
}

/// A module path's one, two, three, six or twelve delays as one per
/// transition (IEEE 1364-2005 section 14.3.1, table 14-3), and the delays
/// to and from `x` that fewer than twelve leave out as section 14.3.2 makes
/// them.
fn path_delay(
    (values, span): (&[Expression], crate::source::Span),
    constants: (&ResolvedConstants, crate::time_scale::ModuleTimeScale),
    precision: i8,
) -> Result<super::digital::DigitalPathDelay, DigitalLoweringDiagnostic> {
    let ticks = values
        .iter()
        .map(|value| delay_value_ticks(value, constants, precision, "14.3.1"))
        .collect::<Result<Vec<_>, _>>()?;
    let transitions = match *ticks.as_slice() {
        [t01, t10, t0z, tz1, t1z, tz0, t0x, tx1, t1x, tx0, txz, tzx] => {
            [t01, t10, t0z, tz1, t1z, tz0, t0x, tx1, t1x, tx0, txz, tzx]
        }
        _ => {
            let [t01, t10, t0z, tz1, t1z, tz0] = match *ticks.as_slice() {
                [t] => [t; 6],
                [rise, fall] => [rise, fall, rise, rise, fall, fall],
                [rise, fall, off] => [rise, fall, off, rise, off, fall],
                [t01, t10, t0z, tz1, t1z, tz0] => [t01, t10, t0z, tz1, t1z, tz0],
                _ => {
                    return Err(DigitalLoweringDiagnostic::refusal(
                        "a module path takes one, two, three, six or twelve delays (IEEE \
                         1364-2005 section 14.3.1)",
                        span.into(),
                    ));
                }
            };
            [
                t01,
                t10,
                t0z,
                tz1,
                t1z,
                tz0,
                t01.min(t0z),
                t01.max(tz1),
                t10.min(t1z),
                t10.max(tz0),
                t0z.max(t1z),
                tz1.min(tz0),
            ]
        }
    };
    Ok(super::digital::DigitalPathDelay {
        transitions,
        exponent: precision,
    })
}

/// The constant expressions of a timing check: its limits, `$timeskew`'s
/// and `$fullskew`'s flags, and its events' selects (IEEE 1364-2005 A.7.5.2
/// and A.7.5.3).
fn check_constants(check: &crate::ast::SpecifyTimingCheck) -> impl Iterator<Item = &Expression> {
    use crate::ast::TerminalSelect;
    let selects = std::iter::once(&check.reference)
        .chain(&check.data)
        .flat_map(|event| match &event.terminal.select {
            Some(TerminalSelect::Bit { index, .. }) => vec![index.as_ref()],
            Some(TerminalSelect::Part { first, second, .. }) => {
                vec![first.as_ref(), second.as_ref()]
            }
            None => Vec::new(),
        });
    check
        .limits
        .iter()
        .chain(&check.event_based_flag)
        .chain(&check.remain_active_flag)
        .chain(selects)
}

/// A frame's module path delays (IEEE 1364-2005 section 14), its names
/// resolved in `scope`.
///
/// A destination that is a variable output port is the net its port
/// connection drives: the path stands between the variable and that net. A
/// variable port connected to nothing has no observer outside its module,
/// and no path.
fn lower_paths(
    paths: &[crate::semantic::AnalyzedSpecifyPath],
    scope: &HashMap<&str, DigitalSignalId>,
    port_drivers: &[crate::semantic::AnalyzedContinuousAssign],
    constants: (&ResolvedConstants, crate::time_scale::ModuleTimeScale),
    (signals, elaborated): (&[DigitalSignal], &HashMap<&str, DigitalSignalId>),
    precision: i8,
    lowered: &mut Vec<super::digital::DigitalModulePath>,
) -> Result<(), DigitalLoweringDiagnostic> {
    use super::digital::DigitalEdge;
    use crate::ast::{DigitalLValue, EdgeKind};
    let resolve = |name: &SmolStr, span: crate::source::Span| {
        scope.get(name.as_str()).copied().ok_or_else(|| {
            DigitalLoweringDiagnostic::refusal(
                format!("module path terminal `{name}` names no discrete signal"),
                span.into(),
            )
        })
    };
    for path in paths {
        let source = resolve(&path.source, path.span)?;
        let written = resolve(&path.destination, path.span)?;
        let declared = &signals[usize::from(written)];
        let destination = if declared.procedurally_assignable {
            let connection = port_drivers.iter().find(|driver| {
                matches!(&driver.assignment.value, Expression::Identifier(own) if own.name == declared.name)
            });
            match connection.map(|driver| &driver.assignment.target) {
                None => continue,
                // A port driver is written in elaborated names.
                Some(DigitalLValue::Identifier { name, .. }) => {
                    elaborated.get(name.as_str()).copied().ok_or_else(|| {
                        DigitalLoweringDiagnostic::refusal(
                            format!(
                                "module path destination `{}` drives no net",
                                path.destination
                            ),
                            path.span.into(),
                        )
                    })?
                }
                Some(_) => {
                    return Err(DigitalLoweringDiagnostic::refusal(
                        format!(
                            "module path destination `{}` is a variable port connected to a \
                             select or a concatenation; a path here delays a whole net, so \
                             connect the port to one",
                            path.destination
                        ),
                        path.span.into(),
                    ));
                }
            }
        } else {
            written
        };
        let condition = path
            .condition
            .as_ref()
            .map(|name| resolve(name, path.span))
            .transpose()?;
        lowered.push(super::digital::DigitalModulePath {
            source,
            destination,
            edge: path.edge.map(|edge| match edge {
                EdgeKind::Posedge => DigitalEdge::Posedge,
                EdgeKind::Negedge => DigitalEdge::Negedge,
            }),
            condition,
            ifnone: path.ifnone,
            delay: path_delay((&path.delays, path.span), constants, precision)?,
            pulse_limits: path
                .pulse_limits
                .as_ref()
                .map(|(reject, error)| {
                    let limit = |value: &Expression| {
                        delay_value_ticks(value, constants, precision, "14.6.1")
                    };
                    let reject = limit(reject)?;
                    // An error limit under the reject limit is the reject
                    // limit, as SDF's PATHPULSE reads it (OVI SDF 3.0).
                    Ok::<_, DigitalLoweringDiagnostic>([reject, limit(error)?.max(reject)])
                })
                .transpose()?,
            on_detect: path.on_detect,
            show_cancelled: path.show_cancelled,
            delayed_signal: false,
        });
    }
    Ok(())
}

/// The driver delays lowered so far, in ticks of the design's precision.
struct DriverDelays {
    precision: i8,
    delays: Vec<super::digital::DigitalDriverDelay>,
}

/// A gate's, a continuous assignment's or a net's delay as one delay per
/// transition (IEEE 1364-2005 sections 6.1.3 and 7.14), or `None` for one
/// that is zero for every transition, which delays nothing.
///
/// Each value is an elaboration constant in its module's time units, rounded
/// to the module's precision like any delay. One value is every transition's;
/// with two, turning off takes the lesser.
fn transition_delay(
    delay: &crate::ast::Delay3,
    constants: &ResolvedConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
    precision: i8,
) -> Result<Option<super::digital::DigitalTransitionDelay>, DigitalLoweringDiagnostic> {
    let ticks = delay
        .values
        .iter()
        .map(|value| delay_value_ticks(value, (constants, time_scale), precision, "6.1.3"))
        .collect::<Result<Vec<_>, _>>()?;
    let (rise, fall) = match ticks.as_slice() {
        [delay] => (*delay, *delay),
        [rise, fall, ..] => (*rise, *fall),
        [] => return Ok(None),
    };
    let turn_off = ticks.get(2).copied().unwrap_or(rise.min(fall));
    Ok(
        ((rise, fall, turn_off) != (0, 0, 0)).then_some(super::digital::DigitalTransitionDelay {
            rise,
            fall,
            turn_off,
            exponent: precision,
        }),
    )
}

/// Lower one continuous assignment into a driver process.
///
/// The shape is the design of a driver, so it is worth reading as one. The
/// entry block evaluates the right-hand side and publishes it as this driver's
/// contribution; the entry block *then* suspends on the operands it read. A
/// driver is active from the start of the simulation rather than from the first
/// change of an operand (IEEE 1364-2005 section 6.1), and evaluating before
/// waiting is how that is spelled in a graph.
///
/// The sensitivity is derived from the right-hand side's read set, the same
/// rule section 9.7.5 gives `@*`, and reported as
/// [`DigitalSensitivityOrigin::Implicit`] because that is what it is.
///
/// A driver with no operands — `assign y = 1'b0;` — has no list to wait on and
/// returns instead of looping. Its value cannot change, so a process that woke
/// for it would have nothing to do.
///
/// A delay (sections 6.1.3 and 7.14) is not part of the process: the process
/// computes the driver's value, and the delay stands between that value and
/// the net, where the runtime applies it to every driver the assignment has.
fn lower_continuous_assign(
    assignment: &crate::semantic::AnalyzedContinuousAssign,
    signals: &mut Vec<DigitalSignal>,
    arrays: &HashMap<DigitalSignalId, super::digital::DigitalArray>,
    index: &HashMap<&str, DigitalSignalId>,
    array_names: &HashSet<SmolStr>,
    constants: &ResolvedConstants,
    analog_variables: &HashMap<SmolStr, AnalogVariable>,
    id: DigitalProcessId,
    (drivers, strengths, delays): (
        &mut Vec<DigitalDriver>,
        &mut super::digital::DigitalStrengthPlan,
        &mut DriverDelays,
    ),
    probes: &mut Vec<DigitalAnalogProbe>,
    time_scale: crate::time_scale::ModuleTimeScale,
    routines: &routines::RoutineTable,
) -> Result<CfgDigitalProcess, Vec<DigitalLoweringDiagnostic>> {
    let mut lowerer = ProcessLowerer {
        calls: Default::default(),
        analog_local_inputs: HashMap::new(),
        process: None,
        routines,
        local_arrays: Vec::new(),
        constant_expression: false,
        time_scale,
        signals,
        arrays,
        index,
        array_names,
        constants,
        analog_variables,
        probes,
        builder: ProcessBuilder::new(),
        diagnostics: Vec::new(),
        locals: Vec::new(),
        scopes: Vec::new(),
        static_scopes: HashMap::new(),
        static_local_count: 0,
    };

    let delay = match &assignment.assignment.delay {
        Some(delay) if lowerer.lvalue_is_real(&assignment.assignment.target) => {
            lowerer.error(
                "a delay on a driver of a real net; IEEE 1364-2005 section 7.14 gives \
                 transition delays to four-state values only",
                delay.span,
            );
            None
        }
        Some(delay) => match transition_delay(delay, constants, time_scale, delays.precision) {
            Ok(delay) => delay,
            Err(diagnostic) => {
                lowerer.diagnostics.push(diagnostic);
                None
            }
        },
        None => None,
    };

    let entry = lowerer.builder.create_block();
    // The driven net is the assignment's left-hand side, and section 5.4.1
    // does not distinguish a continuous assignment from a procedural one:
    // `assign p = a * b;` with an eight-bit `p` multiplies at eight bits.
    //
    // Except when the net is a `wreal`, which has no width to impose. Then the
    // right-hand side is lowered in the real domain, which is what Verilog-AMS
    // LRM 2.4 section 3.7's `assign wrstim = stim;` means.
    // A gate with an enable wrote its value as `{enable, data}` (see
    // `ContinuousAssign::gate`).
    let gated = assignment.assignment.gate.and_then(|kind| {
        let Expression::ArrayLiteral(pair) = &assignment.assignment.value else {
            return None;
        };
        match pair.elements.as_slice() {
            [
                crate::ast::ArrayLiteralElement::Value(enable),
                crate::ast::ArrayLiteralElement::Value(data),
            ] => Some((kind, enable, data)),
            _ => None,
        }
    });
    let (value, enable) = if let Some((_, enable, data)) = gated {
        let context = lowerer.lvalue_width(&assignment.assignment.target);
        let value = lowerer.assigned_value(entry, data, context);
        (value, Some(lowerer.assigned_value(entry, enable, 1)))
    } else if lowerer.lvalue_is_real(&assignment.assignment.target) {
        let value = lowerer.real_expression(entry, &assignment.assignment.value);
        (value, None)
    } else {
        let context = lowerer.lvalue_width(&assignment.assignment.target);
        let value = lowerer.assigned_value(entry, &assignment.assignment.value, context);
        (value, None)
    };
    lowerer.drive(
        entry,
        &assignment.assignment.target,
        (value, enable),
        id,
        drivers,
    );
    if let Some(strength) = assignment.assignment.strength {
        strengths
            .drivers
            .extend(
                drivers
                    .iter()
                    .filter(|driver| driver.process == id)
                    .map(|driver| super::digital::DigitalDriverStrength {
                        driver: driver.id,
                        strength,
                    }),
            );
    }
    if let Some(delay) = delay {
        delays.delays.extend(
            drivers
                .iter()
                .filter(|driver| driver.process == id)
                .map(|driver| super::digital::DigitalDriverDelay {
                    driver: driver.id,
                    delay,
                }),
        );
    }
    let bidirectional = matches!(
        gated,
        Some((super::digital::DigitalGateKind::Tran { .. }, _, _))
    );
    if let Some((kind, _, data)) = gated {
        let source = matches!(
            kind,
            super::digital::DigitalGateKind::Mos { .. }
                | super::digital::DigitalGateKind::Tran { .. }
        )
        .then(|| lowerer.passed_net_bit(data))
        .flatten();
        if bidirectional && source.is_none() {
            lowerer.error(
                "a bidirectional switch terminal must be a scalar net or a bit select of a \
                 vector net (IEEE 1364-2005 section 7.6)",
                data.span(),
            );
        }
        strengths
            .gates
            .extend(
                drivers
                    .iter()
                    .filter(|driver| driver.process == id)
                    .map(|driver| super::digital::DigitalGatedDriver {
                        driver: driver.id,
                        kind,
                        source: source.clone(),
                    }),
            );
    }

    let mut reads = BTreeSet::new();
    // A bidirectional switch's driver wakes on its control alone: the store
    // resolves the island whenever one of its nets changes.
    match gated {
        Some((_, enable, _)) if bidirectional => collect_expression_reads(enable, &mut reads),
        _ => collect_expression_reads(&assignment.assignment.value, &mut reads),
    }
    lowerer.validate_analog_event_reads(&assignment.assignment.value);
    let terms: Vec<DigitalSensitivityTerm> = reads
        .into_iter()
        .flat_map(|name| lowerer.read_dependencies(&name))
        .map(|signal| DigitalSensitivityTerm { signal, edge: None })
        .collect();

    let static_sensitivity = if terms.is_empty() {
        lowerer.builder.set_terminator(entry, CfgTerminator::Return);
        None
    } else {
        let resume = lowerer.builder.create_block();
        lowerer.builder.set_terminator(
            entry,
            CfgTerminator::Wait {
                wait: DigitalWait::Event(terms.clone()),
                resume,
                resume_args: Vec::new(),
            },
        );
        lowerer.builder.seal_block(resume);
        lowerer.builder.set_terminator(
            resume,
            CfgTerminator::Jump {
                target: entry,
                args: Vec::new(),
            },
        );
        Some(DigitalStaticSensitivity {
            terms,
            origin: DigitalSensitivityOrigin::Implicit,
        })
    };
    lowerer.builder.seal_all_blocks();

    if !lowerer.diagnostics.is_empty() {
        return Err(lowerer.diagnostics);
    }
    let function = lowerer.builder.finish(entry).map_err(|error| {
        vec![DigitalLoweringDiagnostic::invariant(
            format!("lowering a continuous assignment produced an invalid graph: {error}"),
            assignment.span.into(),
        )]
    })?;

    Ok(CfgDigitalProcess {
        id,
        kind: DigitalProcessKind::ContinuousAssign,
        time_scale,
        function,
        static_sensitivity,
        span: assignment.span.into(),
        origin: None,
    })
}

fn lower_signals(
    declarations: &[AnalyzedDigitalSignal],
    borrowed: impl Fn(&str) -> bool,
) -> Result<
    (
        Vec<DigitalSignal>,
        Vec<super::digital::DigitalArray>,
        Vec<DigitalSignalId>,
    ),
    Vec<DigitalLoweringDiagnostic>,
> {
    let mut signals = Vec::new();
    let mut arrays = Vec::new();
    let mut ids = vec![DigitalSignalId::new(0); declarations.len()];
    let mut allocated = HashMap::new();
    for (index, signal) in declarations.iter().enumerate() {
        if signal.element_alias.is_none() && !borrowed(&signal.name) {
            let id = append_signal(signal, signal.name.clone(), &mut signals, &mut arrays);
            ids[index] = id;
            allocated.insert(signal.name.as_str(), id);
        }
    }
    for (index, signal) in declarations.iter().enumerate() {
        let Some(alias) = &signal.element_alias else {
            continue;
        };
        let invalid = || {
            vec![DigitalLoweringDiagnostic::invariant(
                format!(
                    "real array element view '{}' has an invalid target or type",
                    signal.name
                ),
                signal.span.into(),
            )]
        };
        let base = allocated.get(alias.array.as_str()).ok_or_else(invalid)?;
        let array = arrays
            .iter()
            .find(|array| array.storage.base == *base)
            .ok_or_else(invalid)?;
        if alias.offset >= array.storage.len || signal.unpacked.is_some() {
            return Err(invalid());
        }
        let id = DigitalSignalId::new(base.index() + alias.offset);
        let actual = &signals[usize::from(id)];
        let expected = lower_signal(signal, id, actual.name.clone());
        if !matches!(actual.kind, DigitalSignalKind::Real(_))
            || actual.kind != expected.kind
            || actual.procedurally_assignable
            || expected.procedurally_assignable
            || expected.width != 0
            || expected.bounds.is_some()
        {
            return Err(invalid());
        }
        ids[index] = id;
    }
    Ok((signals, arrays, ids))
}

fn append_signal(
    signal: &AnalyzedDigitalSignal,
    name: SmolStr,
    signals: &mut Vec<DigitalSignal>,
    arrays: &mut Vec<super::digital::DigitalArray>,
) -> DigitalSignalId {
    let base = DigitalSignalId::from(signals.len());
    if let Some(bounds) = signal.unpacked {
        let storage = super::digital::DigitalArrayRef {
            base,
            lower: bounds.msb.min(bounds.lsb),
            len: bounds.width(),
        };
        let array = super::digital::DigitalArray {
            dimensions: if signal.dimensions.len() > 1 {
                signal
                    .dimensions
                    .iter()
                    .map(|axis| (axis.msb, axis.lsb))
                    .collect()
            } else {
                Vec::new()
            },
            name,
            bounds: (bounds.msb, bounds.lsb),
            storage,
        };
        let layout = array.layout().expect("analyzed array layout");
        for offset in 0..storage.len {
            let id = DigitalSignalId::from(signals.len());
            signals.push(lower_signal(
                signal,
                id,
                array
                    .element_name_with_layout(&layout, offset as usize)
                    .expect("array element"),
            ));
        }
        arrays.push(array);
    } else {
        signals.push(lower_signal(signal, base, name));
    }
    base
}

fn initialize_signal(
    declared: &AnalyzedDigitalSignal,
    base: DigitalSignalId,
    constants: &ResolvedConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
    signals: &mut [DigitalSignal],
) -> Result<(), Vec<DigitalLoweringDiagnostic>> {
    for (offset, value) in constants::initializers(declared, constants, time_scale)?
        .into_iter()
        .enumerate()
    {
        signals[usize::from(base) + offset].initial_value = value;
    }
    Ok(())
}

/// One declaration, under the identity and name the elaborated scope gives it.
///
/// The name is a parameter rather than read off the declaration because an
/// instance's signal is named by its instance path, and a port that collapsed
/// is named by the net it joined.
fn lower_signal(
    signal: &AnalyzedDigitalSignal,
    id: DigitalSignalId,
    name: SmolStr,
) -> DigitalSignal {
    DigitalSignal {
        local: None,
        initial_value: None,
        id,
        name,
        kind: match (signal.class.is_real(), signal.class.wreal_resolution()) {
            (false, _) => DigitalSignalKind::FourState,
            (true, Some(resolution)) => DigitalSignalKind::Real(match resolution {
                WrealResolution::Single => DigitalRealResolution::Single,
                WrealResolution::Sum => DigitalRealResolution::Sum,
                WrealResolution::Average => DigitalRealResolution::Average,
                WrealResolution::Minimum => DigitalRealResolution::Minimum,
                WrealResolution::Maximum => DigitalRealResolution::Maximum,
            }),
            // A real *variable*: no net-type keyword and so no resolution. It
            // carries [`DigitalRealResolution::Single`] because that is the
            // truth about it — one writer at a time and no fold — but nothing
            // ever consults the field, because a variable has no drivers to
            // fold. `procedurally_assignable` below is what tells the two
            // apart everywhere it matters.
            (true, None) => DigitalSignalKind::Real(DigitalRealResolution::Single),
        },
        nettype: matches!(
            signal.class,
            crate::semantic::DigitalSignalClass::Net(crate::ast::DigitalNetKind::Nettype)
        ),
        width: signal.width,
        bounds: signal.range.map(|range| (range.msb, range.lsb)),
        signed: signal.signedness.is_signed(),
        integer: matches!(
            signal.class,
            crate::semantic::DigitalSignalClass::Variable(crate::ast::DigitalVariableKind::Integer)
        ),
        procedurally_assignable: signal.class.is_variable(),
        span: signal.span.into(),
        origin: None,
    }
}

/// Lower one process under the identity the plan gives it.
///
/// `id` is the plan's, not the source's. They are the same number for a
/// process of the compiled module, and deliberately not for one of an instance
/// frame: two instances of a module have the same source process and must have
/// two identities, because a scheduler resumes one of them.
fn lower_process(
    process: &AnalyzedDigitalProcess,
    id: DigitalProcessId,
    signals: &mut Vec<DigitalSignal>,
    arrays: &HashMap<DigitalSignalId, super::digital::DigitalArray>,
    local_arrays: &mut Vec<super::digital::DigitalArray>,
    index: &HashMap<&str, DigitalSignalId>,
    array_names: &HashSet<SmolStr>,
    constants: &ResolvedConstants,
    analog_variables: &HashMap<SmolStr, AnalogVariable>,
    probes: &mut Vec<DigitalAnalogProbe>,
    time_scale: crate::time_scale::ModuleTimeScale,
    calls: &mut PlanCalls,
    scope: &ScopeFrame,
    routines: &routines::RoutineTable,
) -> Result<CfgDigitalProcess, Vec<DigitalLoweringDiagnostic>> {
    let mut lowerer = ProcessLowerer {
        calls: calls.tables_for(Some(scope)),
        analog_local_inputs: process.analog_local_inputs.clone(),
        process: None,
        routines,
        local_arrays: Vec::new(),
        constant_expression: false,
        time_scale,
        signals,
        arrays,
        index,
        array_names,
        constants,
        analog_variables,
        probes,
        builder: ProcessBuilder::new(),
        diagnostics: Vec::new(),
        locals: Vec::new(),
        scopes: Vec::new(),
        static_scopes: HashMap::new(),
        static_local_count: 0,
    };

    lowerer.process = Some(id);
    // IEEE 1364-2005 section 12.4: each generate block the process was
    // written in is a scope, which `%m` names: `main.L[0].genblk1`.
    if let Some(path) = &process.generate_scope {
        for name in path.split('.') {
            lowerer.enter_scope(&SmolStr::from(name), DigitalScopeKind::Block);
        }
    }
    let kind = match process.kind {
        AstKind::Always => DigitalProcessKind::Always,
        AstKind::Initial => DigitalProcessKind::Initial,
    };

    let entry = lowerer.builder.create_block();
    lowerer.initialize_static_locals(entry, &process.body);
    lowerer.static_local_count = lowerer.locals.len();
    lowerer.prepare_local_storage(entry, id, &process.body);
    // Declaration initialization belongs to process startup. The restart edge
    // must enter the body after it, preserving named-block variables (IEEE
    // 1364-2005 section 9.8.1) even when a wait precedes their lexical scope.
    let body = if lowerer.static_local_count == 0 {
        entry
    } else {
        let body = lowerer.builder.create_block();
        lowerer.builder.set_terminator(
            entry,
            CfgTerminator::Jump {
                target: body,
                args: Vec::new(),
            },
        );
        lowerer.builder.seal_block(entry);
        body
    };
    if !kind.restarts() {
        lowerer.builder.seal_block(body);
    }
    let exit = lowerer.statement(body, &process.body);

    // IEEE 1364-2005 sections 9.9.1 and 9.9.2, as a difference in the graph
    // rather than a flag: `always` loops back to its own entry, `initial`
    // returns. Nothing else has to be told which kind it is looking at.
    let terminator = if kind.restarts() {
        CfgTerminator::Jump {
            target: body,
            args: Vec::new(),
        }
    } else {
        CfgTerminator::Return
    };
    lowerer.builder.set_terminator(exit, terminator);
    lowerer.builder.seal_block(body);
    // Every construct seals the blocks it creates as soon as their
    // predecessors are known. This is the backstop for the paths that stopped
    // early: a construct that refused left its blocks behind, and an unsealed
    // block reaching `finish` holds parameters whose arguments never arrived.
    lowerer.builder.seal_all_blocks();

    if !lowerer.diagnostics.is_empty() {
        return Err(lowerer.diagnostics);
    }

    local_arrays.extend(lowerer.local_arrays);
    calls.absorb(lowerer.calls, disable::DisableOwner::Process(id));
    let function = lowerer.builder.finish(entry).map_err(|error| {
        vec![DigitalLoweringDiagnostic::invariant(
            format!("lowering process {id} produced an invalid graph: {error}"),
            process.span.into(),
        )]
    })?;

    // Read the static list back off the body entry's `Wait` rather than
    // computing it a second time. The metadata and the terminator then cannot
    // disagree, which is the failure a separately-derived copy invites — and
    // an `@*` list would otherwise be derived twice and reported twice.
    let static_sensitivity = match (&process.body, &function.block(body).terminator) {
        (
            DigitalStatement::Timing(timing),
            CfgTerminator::Wait {
                wait: DigitalWait::Event(terms),
                ..
            },
        ) => match &timing.control {
            TimingControl::Event(event) => Some(DigitalStaticSensitivity {
                terms: terms.clone(),
                origin: match event.sensitivity {
                    crate::ast::Sensitivity::Implicit => DigitalSensitivityOrigin::Implicit,
                    crate::ast::Sensitivity::Explicit(_) => DigitalSensitivityOrigin::Explicit,
                },
            }),
            TimingControl::Delay(_) => None,
        },
        _ => None,
    };

    Ok(CfgDigitalProcess {
        id,
        kind,
        time_scale,
        function,
        static_sensitivity,
        span: process.span.into(),
        origin: None,
    })
}

/// Width of the bit pattern `$realtobits` produces and `$bitstoreal` consumes.
///
/// A property of the format rather than of the expression: Verilog-AMS LRM 2.4
/// section 3.7 speaks of "explicitly declared 64-bit wires", and IEEE 754
/// double precision is 64 bits. Not context-determined, and not configurable.
const REAL_BIT_PATTERN_WIDTH: u32 = 64;

/// What a loop runs at the end of each pass.
enum LoopUpdate<'a> {
    /// A `for` loop's third clause, written by the author.
    Assign(&'a DigitalAssign),
    /// A `repeat` loop's decrement of the counter the lowering invented.
    Decrement(DigitalLocalId),
}

/// A lexical local represented by SSA values or shared scalar/array storage.
struct ProcessLocal {
    /// `None` for a counter the lowering invented, which no source name can
    /// reach and which therefore cannot be shadowed or read by mistake.
    name: Option<SmolStr>,
    /// Stored scalar, or first cell of an array; None for an SSA local.
    shared: Option<DigitalSignalId>,
    /// An automatic routine's frame slot; None outside one.
    slot: Option<u32>,
    array: Option<(VectorBounds, super::digital::DigitalArrayRef)>,
    integer: bool,
    packed: bool,
    /// Whether the variable holds a real rather than four-state bits.
    ///
    /// A `real` declared inside a process (IEEE 1364-2005 section 3.9.1), which
    /// is what the Verilog-AMS LRM 2.4 section 6.5.3 example reads a `wreal`
    /// into. Its `width` is zero, the way every real quantity's is here, and
    /// its `bounds` mean nothing — a real has no bits to name.
    real: bool,
    width: u32,
    /// The range its bits are named over, exactly as declared.
    ///
    /// Carried beside the width rather than folded into it, because a local is
    /// bit-selected by the same rule a signal is: `reg [7:4] t; t[4]` names the
    /// least significant bit of a four-bit variable, and only the bounds say
    /// so. [`VectorBounds::SCALAR`] for a `reg` declared without a range, and
    /// [`INTEGER_BOUNDS`] for an `integer`. The two cannot disagree: every
    /// four-state local takes its width from `bounds.width()`.
    bounds: VectorBounds,
    /// Whether reading it yields a signed value, IEEE 1364-2005 table 5-21.
    ///
    /// True for an `integer`, which the table makes signed without any
    /// qualifier, and for a `reg signed`. False for a plain `reg` and for the
    /// invented `repeat` counter, which counts down to zero and is nobody's
    /// operand.
    signed: bool,
    /// Where it was declared, for a diagnostic about it.
    span: Span,
}

/// The width and signedness an enclosing expression imposes on an operand.
///
/// The two halves of IEEE 1364-2005's top-down pass, carried together because
/// section 5.5 determines them together: "the size and the signedness of the
/// expression are determined from the whole context before evaluation". Sizing
/// an operand without also deciding how it is extended answers half the
/// question, and the half it leaves out is the one that decides whether
/// `-1` reaches an eight-bit operator as -1 or as 15.
#[derive(Clone, Copy)]
struct Context {
    /// Section 5.4.1's context size. Zero asks for nothing.
    width: u32,
    /// Whether the *enclosing* expression is signed.
    ///
    /// Not a claim about the operand. Section 5.4.2 rule (j) makes an
    /// expression signed only when every one of its context-determined
    /// operands is, so this flag travels downward as a permission that is
    /// `and`ed with each operand's own classification: one unsigned operand
    /// makes the shared context unsigned, and every other operand in it is
    /// then extended and interpreted as unsigned however it was declared.
    signed: bool,
}

impl Context {
    /// The context of an expression that nothing outside sizes or signs.
    ///
    /// `signed: true` is the absence of an imposition rather than a claim:
    /// nothing outside is forcing unsignedness, so the expression's own
    /// classification stands. Every self-determined position of table 5-22
    /// uses this — a `case` selector, a branch condition, a `repeat` count, an
    /// event term, a concatenation operand, a shift count, a comparison
    /// operand, a reduction operand.
    const SELF_DETERMINED: Self = Self {
        width: 0,
        signed: true,
    };
}

#[derive(Clone, Copy)]
enum ConditionalDomain {
    Real,
    FourState(Context),
}

/// Expression lowering can split a statement's block. Keep its entry identity
/// for loop back edges, but route subsequent statements through its current
/// continuation. Target IDs and sealing always refer to the original entry.
struct ProcessBuilder {
    ssa: SsaBuilder,
    continuations: HashMap<BlockId, BlockId>,
}

impl std::ops::Deref for ProcessBuilder {
    type Target = SsaBuilder;
    fn deref(&self) -> &SsaBuilder {
        &self.ssa
    }
}
impl std::ops::DerefMut for ProcessBuilder {
    fn deref_mut(&mut self) -> &mut SsaBuilder {
        &mut self.ssa
    }
}
impl ProcessBuilder {
    fn new() -> Self {
        Self {
            ssa: SsaBuilder::new(),
            continuations: HashMap::new(),
        }
    }
    fn tail(&mut self, entry: BlockId) -> BlockId {
        let mut block = entry;
        while let Some(next) = self.continuations.get(&block) {
            block = *next;
        }
        // Sequential conditionals repeatedly append to the same source block.
        // Compress that lookup so a long statement list does not walk every
        // preceding expression's continuation on every instruction.
        if block != entry {
            self.continuations.insert(entry, block);
        }
        block
    }
    fn continue_at(&mut self, block: BlockId, continuation: BlockId) {
        let tail = self.tail(block);
        self.continuations.insert(tail, continuation);
    }
    fn push(&mut self, block: BlockId, ty: CfgValueType, kind: CfgValueKind) -> ValueId {
        let block = self.tail(block);
        self.ssa.push(block, ty, kind)
    }
    fn set_terminator(&mut self, block: BlockId, terminator: CfgTerminator) {
        let block = self.tail(block);
        self.ssa.set_terminator(block, terminator);
    }
    fn read_variable(&mut self, variable: CfgVariable, block: BlockId) -> Option<ValueId> {
        let block = self.tail(block);
        self.ssa.read_variable(variable, block)
    }
    fn write_variable(&mut self, variable: CfgVariable, block: BlockId, value: ValueId) {
        let block = self.tail(block);
        self.ssa.write_variable(variable, block, value);
    }
    fn carry_value(&mut self, value: ValueId, from: BlockId, to: BlockId) -> ValueId {
        let from = self.tail(from);
        self.ssa.carry_value(value, from, to)
    }
    fn carry_variable(
        &mut self,
        variable: CfgVariable,
        from: BlockId,
        to: BlockId,
    ) -> Option<ValueId> {
        let from = self.tail(from);
        self.ssa.carry_variable(variable, from, to)
    }
    fn merge_values(&mut self, to: BlockId, incoming: &[(BlockId, ValueId)]) -> ValueId {
        let incoming: Vec<_> = incoming
            .iter()
            .map(|&(from, value)| (self.tail(from), value))
            .collect();
        self.ssa.merge_values(to, &incoming)
    }
    fn finish(self, entry: BlockId) -> Result<super::CfgFunction, super::cfg::CfgValidationError> {
        self.ssa.finish(entry)
    }
    fn finish_with_outputs(
        self,
        entry: BlockId,
        outputs: &[ValueId],
    ) -> Result<(super::CfgFunction, Vec<ValueId>), super::cfg::CfgValidationError> {
        self.ssa.finish_with_outputs(entry, outputs)
    }
}

struct ProcessLowerer<'a> {
    /// System call sites, observers and named-block scopes this lowering
    /// produces, and the scope it is in.
    calls: CallTables,
    analog_local_inputs: HashMap<Span, SmolStr>,
    process: Option<DigitalProcessId>,
    /// The tasks and functions a body in this scope can call.
    routines: &'a routines::RoutineTable,
    local_arrays: Vec<super::digital::DigitalArray>,
    /// Closed parameter expressions may use pure analog math intrinsics.
    constant_expression: bool,
    time_scale: crate::time_scale::ModuleTimeScale,
    signals: &'a mut Vec<DigitalSignal>,
    arrays: &'a HashMap<DigitalSignalId, super::digital::DigitalArray>,
    index: &'a HashMap<&'a str, DigitalSignalId>,
    /// Lexical shape is independent of identity: a scalar port can share the
    /// first cell of an array without denoting the whole array.
    array_names: &'a HashSet<SmolStr>,
    /// The elaboration-time constants a name in this body may denote.
    ///
    /// IEEE 1364-2005 section 12.2 fixes a parameter's value at elaboration, so
    /// `reg [WIDTH-1:0] q` and `{WIDTH{1'b0}}` are constant expressions. The
    /// table is the *declaring* module's, which is why it travels with the
    /// scope rather than being read out of one place: an instance frame's body
    /// is lowered against an empty table so a child's `WIDTH` can never be
    /// folded with a parent's.
    constants: &'a ResolvedConstants,
    analog_variables: &'a HashMap<SmolStr, AnalogVariable>,
    /// The plan's continuous-net probe table, appended to as probes appear.
    ///
    /// Plan-wide rather than per-process, because it is what a host resolves
    /// against and two processes probing one net should not make it resolve
    /// the net twice. Threaded through by `&mut` rather than returned, because
    /// a process is lowered in one pass and the table has to be shared with
    /// the processes lowered before and after it.
    probes: &'a mut Vec<DigitalAnalogProbe>,
    builder: ProcessBuilder,
    diagnostics: Vec<DigitalLoweringDiagnostic>,
    /// Every variable declared in the process, by id.
    locals: Vec<ProcessLocal>,
    /// Declarative regions, innermost last (IEEE 1364-2005 section 9.8.1).
    scopes: Vec<Vec<DigitalLocalId>>,
    /// Declaration identities prepared before any suspension is lowered.
    static_scopes: HashMap<Span, Vec<DigitalLocalId>>,
    /// Compiler-generated loop counters follow these persistent declarations.
    static_local_count: usize,
}

impl ProcessLowerer<'_> {
    /// Refuse a construct the author wrote. See [`DigitalLoweringClass`]: this
    /// reaches them as a semantic error at `span`, naming what is missing.
    fn error(&mut self, message: impl Into<String>, span: Span) {
        self.diagnostics.push(DigitalLoweringDiagnostic::refusal(
            message,
            SourceSpanRef::from(span),
        ));
    }

    /// Report a violation of something this pass established itself. Nothing
    /// an author can write reaches one, so it stays an internal error.
    fn invariant(&mut self, message: impl Into<String>, span: Span) {
        self.diagnostics.push(DigitalLoweringDiagnostic::invariant(
            message,
            SourceSpanRef::from(span),
        ));
    }

    fn width_of(&self, signal: DigitalSignalId) -> u32 {
        self.signals
            .get(usize::from(signal))
            .map_or(1, |signal| signal.width)
    }

    /// Whether a signal was declared `signed`, IEEE 1364-2005 table 5-21.
    fn signed_signal(&self, signal: DigitalSignalId) -> bool {
        self.signals
            .get(usize::from(signal))
            .is_some_and(|signal| signal.signed)
    }

    // ------------------------------------------------------------------
    // Process-local variables
    // ------------------------------------------------------------------

    /// The local a name resolves to, innermost region first.
    ///
    /// The same order the analyzer resolves in, and for the same reason: a
    /// name that meant the local in one pass and the module signal in the
    /// other would be two compilers disagreeing about one program.
    fn lookup_local(&self, name: &str) -> Option<DigitalLocalId> {
        self.scopes.iter().rev().find_map(|scope| {
            scope
                .iter()
                .rev()
                .copied()
                .find(|id| self.locals[usize::from(*id)].name.as_deref() == Some(name))
        })
    }

    fn resolved_signal(&self, name: &str) -> Option<DigitalSignalId> {
        match self.lookup_local(name) {
            Some(local) => self.locals[usize::from(local)].shared,
            None => self.index.get(name).copied(),
        }
    }

    fn ssa_local(&self, name: &str) -> Option<DigitalLocalId> {
        self.lookup_local(name)
            .filter(|local| self.locals[usize::from(*local)].shared.is_none())
    }

    fn local_width(&self, id: DigitalLocalId) -> u32 {
        self.locals[usize::from(id)].width
    }

    fn local_signed(&self, id: DigitalLocalId) -> bool {
        self.locals[usize::from(id)].signed
    }

    fn local_is_real(&self, id: DigitalLocalId) -> bool {
        self.locals[usize::from(id)].real
    }

    /// Whether a signal the plan declares carries a real value.
    fn real_signal(&self, signal: DigitalSignalId) -> bool {
        self.signals
            .get(usize::from(signal))
            .is_some_and(|signal| signal.kind.is_real())
    }

    /// The range the name a select is written against numbers its bits over.
    ///
    /// Resolve in the same scope order as [`Self::named_value`]. Parameters
    /// and analog integers retain their own fixed ranges. Unknown names are
    /// reported by the value read; the scalar range is only its placeholder.
    fn declared_range_of(&self, name: &str) -> VectorBounds {
        if let Some(local) = self.lookup_local(name) {
            return self.locals[usize::from(local)].bounds;
        }
        match self.index.get(name) {
            Some(signal) => self.signals[usize::from(*signal)].declared_range(),
            None => {
                // Parameters retain their authored width; analog integers use
                // the same [31:0] numbering as digital integer storage.
                if let Some(bounds) = self.constants.bounds.get(name) {
                    *bounds
                } else if let Some((value, _)) = self.constants.bits.get(name) {
                    VectorBounds {
                        msb: i64::from(value.width()) - 1,
                        lsb: 0,
                    }
                } else if self.analog_variables.get(name).is_some_and(|variable| {
                    variable.array.is_none()
                        && variable.quantity
                            == super::digital::DigitalAnalogQuantity::IntegerVariable
                }) {
                    INTEGER_BOUNDS
                } else {
                    VectorBounds::SCALAR
                }
            }
        }
    }

    /// Declare a local and initialize it in `block`. Source declarations use
    /// the startup block; synthesized repeat counters use their loop entry.
    fn declare_local(
        &mut self,
        block: BlockId,
        name: Option<SmolStr>,
        bounds: VectorBounds,
        signed: bool,
        span: Span,
        initial: Option<ValueId>,
    ) -> DigitalLocalId {
        let id = DigitalLocalId::from(self.locals.len());
        let width = bounds.width();
        self.locals.push(ProcessLocal {
            name,
            shared: None,
            slot: None,
            array: None,
            integer: false,
            packed: true,
            real: false,
            width,
            bounds,
            signed,
            span,
        });
        let variable = CfgVariable::DigitalLocal(id);
        self.builder
            .declare_variable(variable, CfgValueType::FourState { width });
        let initial = match initial {
            Some(value) => self.resize(block, value, width, false),
            None => self.builder.push_leaf(
                CfgValueType::FourState { width },
                CfgValueKind::FourStateConstant(FourStateValue::splat(
                    width,
                    FourStateBit::Unknown,
                )),
            ),
        };
        self.builder.write_variable(variable, block, initial);
        if let Some(scope) = self.scopes.last_mut() {
            scope.push(id);
        }
        id
    }

    /// Declare a process-local `real` and give it its initial value in `block`.
    ///
    /// The four-state twin of [`Self::declare_local`], separate because almost
    /// every line of it differs: the SSA type is [`CfgValueType::Real`], the
    /// initial value is a real rather than an all-`x` pattern, and there is no
    /// width or signedness to carry.
    ///
    /// The unwritten value is `0.0`. IEEE 1364-2005 section 3.9.1 makes a
    /// `real` variable's initial value zero — it has no `x` to start at, which
    /// is exactly why it needs its own answer here rather than the section
    /// 4.2.2 one a `reg` gets.
    fn declare_real_local(
        &mut self,
        block: BlockId,
        name: Option<SmolStr>,
        span: Span,
        initial: Option<ValueId>,
    ) -> DigitalLocalId {
        let id = DigitalLocalId::from(self.locals.len());
        self.locals.push(ProcessLocal {
            name,
            shared: None,
            slot: None,
            array: None,
            integer: false,
            packed: false,
            real: true,
            width: 0,
            bounds: VectorBounds::SCALAR,
            signed: false,
            span,
        });
        let variable = CfgVariable::DigitalLocal(id);
        self.builder.declare_variable(variable, CfgValueType::Real);
        let initial = initial.unwrap_or_else(|| self.real_constant(0.0));
        self.builder.write_variable(variable, block, initial);
        if let Some(scope) = self.scopes.last_mut() {
            scope.push(id);
        }
        id
    }

    /// Read a local's current value in `block`.
    ///
    /// A local always has a definition — the declaration wrote one — so a
    /// miss is a lowering bug rather than a program error, and it reports as
    /// one instead of producing a value nothing defined.
    fn read_local(&mut self, block: BlockId, id: DigitalLocalId) -> ValueId {
        if let Some(slot) = self.locals[usize::from(id)].slot {
            let ty = if self.local_is_real(id) {
                CfgValueType::Real
            } else {
                CfgValueType::FourState {
                    width: self.local_width(id),
                }
            };
            return self
                .builder
                .push(block, ty, CfgValueKind::FrameSlotRead { slot });
        }
        if let Some(signal) = self.locals[usize::from(id)].shared {
            let (ty, kind) = if self.local_is_real(id) {
                (
                    CfgValueType::Real,
                    CfgValueKind::DigitalRealSignalRead { signal },
                )
            } else {
                (
                    CfgValueType::FourState {
                        width: self.local_width(id),
                    },
                    CfgValueKind::DigitalSignalRead { signal },
                )
            };
            return self.builder.push(block, ty, kind);
        }
        let variable = CfgVariable::DigitalLocal(id);
        match self.builder.read_variable(variable, block) {
            Some(value) => value,
            None => {
                let local = &self.locals[usize::from(id)];
                let (real, width, span) = (local.real, local.width, local.span);
                let name = local.name.clone();
                self.error(
                    format!(
                        "process-local `{}` is read on a path that never defines it",
                        name.as_deref().unwrap_or("<loop counter>")
                    ),
                    span,
                );
                if real {
                    self.real_constant(0.0)
                } else {
                    self.unknown(width)
                }
            }
        }
    }

    /// Write a local, resizing per IEEE 1364-2005 section 5.2.1.
    ///
    /// Zero-fill, and correct because it is never reached with a value that
    /// needed the other kind: an assignment's right-hand side is sign-extended
    /// to the target's width by [`Self::assigned_value`] before it gets here,
    /// so a narrower value arriving at this point is an unsigned one.
    fn write_local(&mut self, block: BlockId, id: DigitalLocalId, value: ValueId) {
        if let Some(slot) = self.locals[usize::from(id)].slot {
            let value = if self.local_is_real(id) {
                value
            } else {
                self.resize(block, value, self.local_width(id), false)
            };
            self.builder.push(
                block,
                CfgValueType::Effect,
                CfgValueKind::FrameSlotWrite { slot, value },
            );
            return;
        }
        if let Some(signal) = self.locals[usize::from(id)].shared {
            let value = if self.local_is_real(id) {
                value
            } else {
                self.resize(block, value, self.local_width(id), false)
            };
            self.builder.push(
                block,
                CfgValueType::Effect,
                CfgValueKind::DigitalBlockingWrite {
                    target: DigitalWriteTarget {
                        signal,
                        select: DigitalWriteSelect::Whole,
                    },
                    value,
                },
            );
            return;
        }
        // A real local has no width to resize to: section 5.2.1's rule is
        // about bits, and the value arriving here has none.
        if self.local_is_real(id) {
            self.builder
                .write_variable(CfgVariable::DigitalLocal(id), block, value);
            return;
        }
        let width = self.local_width(id);
        let value = self.resize(block, value, width, false);
        self.builder
            .write_variable(CfgVariable::DigitalLocal(id), block, value);
    }

    /// `counter = counter - 1` at the counter's own width, which is what makes
    /// a `repeat` of `2'b00` passes stop rather than wrap forever.
    fn decrement(&mut self, block: BlockId, id: DigitalLocalId) {
        let width = self.local_width(id);
        let current = self.read_local(block, id);
        let one = self.builder.push_leaf(
            CfgValueType::FourState { width },
            CfgValueKind::FourStateConstant(FourStateValue::from_u64(width, 1)),
        );
        let next = self.builder.push(
            block,
            CfgValueType::FourState { width },
            CfgValueKind::DigitalArithmetic {
                op: ArithmeticOp::Sub,
                left: current,
                right: one,
                // The counter is the lowering's own, counts down to zero, and
                // is nobody's operand; unsigned is what "how many passes are
                // left" means.
                signed: false,
            },
        );
        self.builder
            .write_variable(CfgVariable::DigitalLocal(id), block, next);
    }

    /// Every local currently in scope, outermost region first.
    ///
    /// The order is declaration order, which is what makes the parameter list
    /// of a resume block reproducible across runs.
    fn locals_in_scope(&self) -> Vec<DigitalLocalId> {
        self.scopes.iter().flatten().copied().collect()
    }

    /// Allocate and initialize static declarations before lowering control
    /// flow. Lexical visibility is restored separately when lowering each
    /// block; leaving that scope must not end the declaration's lifetime.
    fn initialize_static_locals(&mut self, entry: BlockId, statement: &DigitalStatement) {
        match statement {
            DigitalStatement::Block(block) | DigitalStatement::Fork(block) => {
                self.scopes.push(Vec::new());
                self.declare_block_locals(entry, block);
                let scope = self.scopes.last().cloned().unwrap_or_default();
                self.static_scopes.insert(block.span, scope);
                for child in &block.statements {
                    self.initialize_static_locals(entry, child);
                }
                self.scopes.pop();
            }
            DigitalStatement::Conditional(conditional) => {
                self.initialize_static_locals(entry, &conditional.then_branch);
                if let Some(branch) = &conditional.else_branch {
                    self.initialize_static_locals(entry, branch);
                }
            }
            DigitalStatement::Case(case) => {
                for item in &case.items {
                    self.initialize_static_locals(entry, &item.statement);
                }
                if let Some(default) = &case.default {
                    self.initialize_static_locals(entry, default);
                }
            }
            DigitalStatement::For(statement) => {
                self.initialize_static_locals(entry, &statement.body)
            }
            DigitalStatement::While(statement) => {
                self.initialize_static_locals(entry, &statement.body)
            }
            DigitalStatement::Repeat(statement) => {
                self.initialize_static_locals(entry, &statement.body)
            }
            DigitalStatement::Forever(statement) => {
                self.initialize_static_locals(entry, &statement.body)
            }
            DigitalStatement::Timing(timing) => {
                if let Some(statement) = &timing.statement {
                    self.initialize_static_locals(entry, statement);
                }
            }
            DigitalStatement::Wait(wait) => {
                if let Some(statement) = &wait.statement {
                    self.initialize_static_locals(entry, statement);
                }
            }
            DigitalStatement::BlockingAssign(_)
            | DigitalStatement::NonblockingAssign(_)
            | DigitalStatement::SystemTask(_)
            | DigitalStatement::TaskEnable(_)
            | DigitalStatement::Disable(_)
            | DigitalStatement::EventTrigger(_)
            | DigitalStatement::ProceduralContinuous(_)
            | DigitalStatement::Null(_) => {}
        }
    }

    /// Allocate a complete block scope before evaluating its initializers.
    ///
    /// Name binding is lexical (VAMS-2023 6.8), including a declaration's own
    /// name and forward references. All locals first have their type's default
    /// value. RSpice's local-initializer extension then evaluates declaration
    /// assignments in source order, interleaving numeric and packed declarations.
    fn declare_block_locals(&mut self, block: BlockId, inner: &crate::ast::DigitalBlock) {
        let mut initializers = Vec::new();
        let names: BTreeSet<_> =
            inner
                .variables
                .iter()
                .flat_map(|declaration| declaration.items.iter().map(|item| item.name.to_string()))
                .chain(inner.digital_variables.iter().flat_map(|declaration| {
                    declaration.items.iter().map(|item| item.name.to_string())
                }))
                .collect();
        for declaration in &inner.variables {
            // A `real` is not a narrow four-state value and does not go through
            // the width machinery at all: it is declared, initialized and read
            // in the real domain, which is what lets a process hold what a
            // `wreal` gave it (Verilog-AMS LRM 2.4 section 6.5.3's own example
            // reads one into a `real residue`).
            if matches!(declaration.var_type, crate::ast::VarType::Real) {
                for item in &declaration.items {
                    let local =
                        self.declare_real_local(block, Some(item.name.clone()), item.span, None);
                    self.declare_local_array(block, local, &item.dimensions, &names);
                    if let Some(init) = &item.init {
                        initializers.push((item.span.start, local, init));
                    }
                }
                continue;
            }
            match declaration.var_type {
                // IEEE 1364-2005 section 3.9: an `integer` is a 32-bit
                // variable. It is four-state here rather than the IR's own
                // `Integer`, which has no `x` — and section 4.2.2 gives an
                // unwritten `integer` exactly that.
                crate::ast::VarType::Integer => {}
                crate::ast::VarType::Real | crate::ast::VarType::String => {
                    self.error(
                        "a process-local `string` has no lowered form yet: a process \
                         computes in four-state and real values",
                        declaration.span,
                    );
                    continue;
                }
            };
            for item in &declaration.items {
                // IEEE 1364-2005 table 5-21 makes an `integer` signed, and
                // gives it no qualifier with which to say otherwise. So a loop
                // counter compares signed, and `i < 0` can be true.
                //
                // Section 3.9 also numbers its bits [31:0], which is what makes
                // `i[31]` the sign bit rather than a read off the end.
                let local = self.declare_local(
                    block,
                    Some(item.name.clone()),
                    INTEGER_BOUNDS,
                    true,
                    item.span,
                    None,
                );
                self.locals[usize::from(local)].integer = true;
                self.declare_local_array(block, local, &item.dimensions, &names);
                if let Some(init) = &item.init {
                    initializers.push((item.span.start, local, init));
                }
            }
        }

        for declaration in &inner.digital_variables {
            let signed = declaration.signedness.is_signed();
            let bounds = match &declaration.range {
                None => Some(VectorBounds::SCALAR),
                Some(range) => match (self.constant(&range.msb), self.constant(&range.lsb)) {
                    (Some(msb), Some(lsb)) => Some(VectorBounds { msb, lsb }),
                    _ => {
                        self.error(
                            "the bounds of a process-local `reg` must be literal in this wave",
                            range.span,
                        );
                        None
                    }
                },
            };
            let Some(bounds) = bounds else { continue };
            for item in &declaration.items {
                let local = self.declare_local(
                    block,
                    Some(item.name.clone()),
                    bounds,
                    signed,
                    item.span,
                    None,
                );
                self.locals[usize::from(local)].packed = declaration.range.is_some();
                self.declare_local_array(block, local, &item.dimensions, &names);
                if let Some(init) = &item.init {
                    initializers.push((item.span.start, local, init));
                }
            }
        }
        // These offsets belong to the same preprocessed source stream. Keep
        // the authored order even though the AST separates declaration kinds.
        initializers.sort_by_key(|(offset, _, _)| *offset);
        for (_, local, init) in initializers {
            if self.locals[usize::from(local)].array.is_some() {
                self.initialize_local_array(block, local, init);
                continue;
            }
            let value = if self.local_is_real(local) {
                self.real_expression(block, init)
            } else {
                self.assigned_value(block, init, self.local_width(local))
            };
            self.write_local(block, local, value);
        }
    }

    // ------------------------------------------------------------------
    // Statements
    // ------------------------------------------------------------------

    /// Lower `statement` starting in `block`; returns the block execution
    /// continues in, which differs from `block` whenever the statement
    /// branched or suspended.
    fn statement(&mut self, block: BlockId, statement: &DigitalStatement) -> BlockId {
        match statement {
            DigitalStatement::Null(_) => block,
            DigitalStatement::Block(inner) => {
                let Some(scope) = self.static_scopes.get(&inner.span).cloned() else {
                    self.invariant(
                        "process block has no prepared declaration scope",
                        inner.span,
                    );
                    return block;
                };
                self.scopes.push(scope);
                let named = inner.name.as_ref().map(|name| {
                    let scope = self.enter_scope(name, DigitalScopeKind::Block);
                    let forks = inner.statements.iter().any(local_storage::contains_fork);
                    (scope.is_some(), self.open_unwind(scope, name, forks))
                });
                let mut current = block;
                for statement in &inner.statements {
                    current = self.statement(current, statement);
                }
                if let Some((entered, unwinds)) = named {
                    if unwinds {
                        current = self.close_unwind(current);
                    }
                    if entered {
                        self.leave_named_scope();
                    }
                }
                self.scopes.pop();
                current
            }
            DigitalStatement::SystemTask(task) => self.system_task_statement(block, task),
            DigitalStatement::TaskEnable(enable) => self.task_enable(block, enable),
            DigitalStatement::Wait(wait) => self.wait_statement(block, wait),
            DigitalStatement::EventTrigger(trigger) => self.event_trigger(block, trigger),
            DigitalStatement::ProceduralContinuous(statement) => {
                self.procedural_continuous(block, statement);
                block
            }
            DigitalStatement::Fork(fork) => self.fork_statement(block, fork),
            DigitalStatement::Disable(disable) => self.disable_statement(block, disable),
            DigitalStatement::BlockingAssign(assign) => self.assign(block, assign, false),
            DigitalStatement::NonblockingAssign(assign) => self.assign(block, assign, true),
            DigitalStatement::Conditional(conditional) => {
                let condition = self.condition(block, &conditional.condition);
                let then_entry = self.builder.create_block();
                let else_entry = self.builder.create_block();
                let join = self.builder.create_block();
                self.builder.set_terminator(
                    block,
                    CfgTerminator::Branch {
                        condition,
                        then_target: then_entry,
                        then_args: Vec::new(),
                        else_target: else_entry,
                        else_args: Vec::new(),
                    },
                );
                // Both arms have their one predecessor now, so they are sealed
                // before anything inside them reads a variable.
                self.builder.seal_block(then_entry);
                self.builder.seal_block(else_entry);
                let then_exit = self.statement(then_entry, &conditional.then_branch);
                self.builder.set_terminator(
                    then_exit,
                    CfgTerminator::Jump {
                        target: join,
                        args: Vec::new(),
                    },
                );
                let else_exit = match &conditional.else_branch {
                    Some(branch) => self.statement(else_entry, branch),
                    None => else_entry,
                };
                self.builder.set_terminator(
                    else_exit,
                    CfgTerminator::Jump {
                        target: join,
                        args: Vec::new(),
                    },
                );
                self.builder.seal_block(join);
                join
            }
            DigitalStatement::Case(case) => self.case(block, case),
            DigitalStatement::Timing(timing) => {
                let resume =
                    self.wait(block, &timing.control, timing.statement.as_deref(), &mut []);
                match &timing.statement {
                    Some(statement) => self.statement(resume, statement),
                    None => resume,
                }
            }
            DigitalStatement::Forever(forever) => {
                // A `forever` body is entered once and re-entered from its own
                // exit. Whatever follows it in source is unreachable, so the
                // continuation block is fresh and never jumped to — the graph
                // says the statement does not fall through, which is true.
                let body = self.builder.create_block();
                self.builder.set_terminator(
                    block,
                    CfgTerminator::Jump {
                        target: body,
                        args: Vec::new(),
                    },
                );
                let exit = self.statement(body, &forever.body);
                self.builder.set_terminator(
                    exit,
                    CfgTerminator::Jump {
                        target: body,
                        args: Vec::new(),
                    },
                );
                // Sealed only now: the back edge is the body's second
                // predecessor, and a variable read inside the body has to see
                // both of them or it merges with half the loop missing.
                self.builder.seal_block(body);
                let unreachable = self.builder.create_block();
                self.builder.seal_block(unreachable);
                unreachable
            }
            DigitalStatement::While(statement) => {
                self.loop_statement(block, &statement.body, None, |lowerer, header| {
                    lowerer.condition(header, &statement.condition)
                })
            }
            // IEEE 1364-2005 section 9.6.2. The initialization runs once
            // before the loop and the update at the end of each pass, which is
            // exactly where they are placed here; the counter is an ordinary
            // process-local, so nothing about the loop needs a mechanism of its
            // own once one exists.
            DigitalStatement::For(statement) => {
                let block = self.assign(block, &statement.init, false);
                self.loop_statement(
                    block,
                    &statement.body,
                    Some(LoopUpdate::Assign(&statement.update)),
                    |lowerer, header| lowerer.condition(header, &statement.condition),
                )
            }
            // Section 9.6.2 evaluates the count *once*, before the loop, and
            // runs the body that many times. A count with an `x` or `z` bit
            // has no number of passes. Normalize it and signed negative counts
            // before testing truth, which alone would treat 2'b1x as true.
            DigitalStatement::Repeat(statement) => {
                let count = self.repeat_count(block, &statement.count);
                let width = self.value_width(count);
                // The counter lives in a region of its own so that it crosses
                // a suspension inside the body like any other local, and so
                // that nothing in the body can name it.
                self.scopes.push(Vec::new());
                let counter = self.declare_local(
                    block,
                    None,
                    VectorBounds {
                        msb: i64::from(width) - 1,
                        lsb: 0,
                    },
                    false,
                    statement.span,
                    Some(count),
                );
                let exit = self.loop_statement(
                    block,
                    &statement.body,
                    Some(LoopUpdate::Decrement(counter)),
                    |lowerer, header| {
                        let value = lowerer.read_local(header, counter);
                        lowerer.truth_value(header, value)
                    },
                );
                self.scopes.pop();
                exit
            }
        }
    }

    /// Lower a loop whose header tests a condition before each pass.
    ///
    /// `while`, `for`, and `repeat` differ only in what the header tests and
    /// what runs at the end of a pass, so they share the graph shape: the
    /// header is a merge point with two predecessors, and it cannot be sealed
    /// until the back edge exists — which is the one place SSA construction
    /// genuinely needs two passes.
    fn loop_statement(
        &mut self,
        block: BlockId,
        body_statement: &DigitalStatement,
        update: Option<LoopUpdate<'_>>,
        condition_of: impl FnOnce(&mut Self, BlockId) -> ValueId,
    ) -> BlockId {
        let header = self.builder.create_block();
        let body = self.builder.create_block();
        let exit = self.builder.create_block();
        self.builder.set_terminator(
            block,
            CfgTerminator::Jump {
                target: header,
                args: Vec::new(),
            },
        );

        let condition = condition_of(self, header);
        self.builder.set_terminator(
            header,
            CfgTerminator::Branch {
                condition,
                then_target: body,
                then_args: Vec::new(),
                else_target: exit,
                else_args: Vec::new(),
            },
        );
        self.builder.seal_block(body);
        self.builder.seal_block(exit);

        let mut body_exit = self.statement(body, body_statement);
        match update {
            Some(LoopUpdate::Assign(assign)) => {
                body_exit = self.assign(body_exit, assign, false);
            }
            Some(LoopUpdate::Decrement(counter)) => self.decrement(body_exit, counter),
            None => {}
        }
        self.builder.set_terminator(
            body_exit,
            CfgTerminator::Jump {
                target: header,
                args: Vec::new(),
            },
        );
        self.builder.seal_block(header);
        exit
    }

    /// Lower a `case`, `casez`, or `casex` as a chain of match tests.
    ///
    /// The test is [`CfgValueKind::DigitalCaseMatch`], not `==`. IEEE
    /// 1364-2005 section 9.5 compares a case item against the selector *bit by
    /// bit including `x` and `z`*, so `case (sel) 2'bx0:` matches a selector of
    /// `x0` — where `==` yields `x` and would send it to the default. Section
    /// 9.5.1 then adds the wildcard forms, which ignore the positions where
    /// either operand holds a don't-care value. All three are the same
    /// operator with a different ignore set, which is why they are one node
    /// and not a lowering trick.
    fn case(&mut self, block: BlockId, case: &DigitalCase) -> BlockId {
        let match_kind = match case.kind {
            crate::ast::CaseKind::Exact => DigitalCaseMatch::Exact,
            crate::ast::CaseKind::WildcardZ => DigitalCaseMatch::WildcardZ,
            crate::ast::CaseKind::WildcardXZ => DigitalCaseMatch::WildcardXZ,
        };
        let selector = self.expression(block, &case.selector);
        let join = self.builder.create_block();
        let mut current = block;

        for item in &case.items {
            let mut matched: Option<ValueId> = None;
            for label in &item.labels {
                // Section 9.5 extends every case expression to the width of the
                // widest, and the extension is section 5.4.2's: signed when the
                // selector and *this* label both are. Decided per label rather
                // than once for the arm, because two labels of one arm may be
                // classified differently and each is its own comparison.
                let signed = self.comparison_is_signed(&case.selector, label);
                let label_value = self.expression(current, label);
                let test = self.builder.push(
                    current,
                    CfgValueType::FourState { width: 1 },
                    CfgValueKind::DigitalCaseMatch {
                        selector,
                        label: label_value,
                        kind: match_kind,
                        signed,
                    },
                );
                matched = Some(match matched {
                    None => test,
                    Some(previous) => self.builder.push(
                        current,
                        CfgValueType::FourState { width: 1 },
                        CfgValueKind::DigitalLogical {
                            op: LogicalOp::Or,
                            left: previous,
                            right: test,
                        },
                    ),
                });
            }
            let Some(matched) = matched else {
                continue;
            };
            let arm = self.builder.create_block();
            let next = self.builder.create_block();
            self.builder.set_terminator(
                current,
                CfgTerminator::Branch {
                    condition: matched,
                    then_target: arm,
                    then_args: Vec::new(),
                    else_target: next,
                    else_args: Vec::new(),
                },
            );
            self.builder.seal_block(arm);
            self.builder.seal_block(next);
            let arm_exit = self.statement(arm, &item.statement);
            self.builder.set_terminator(
                arm_exit,
                CfgTerminator::Jump {
                    target: join,
                    args: Vec::new(),
                },
            );
            current = next;
        }

        let default_exit = match &case.default {
            Some(statement) => self.statement(current, statement),
            None => current,
        };
        self.builder.set_terminator(
            default_exit,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );
        self.builder.seal_block(join);
        join
    }

    /// Capture the RHS at encounter. A nonblocking assignment queues
    /// the captured value and continues; a blocking one suspends until delivery.
    fn assign(&mut self, block: BlockId, assign: &DigitalAssign, nonblocking: bool) -> BlockId {
        let (targets, mut carried) = if let Some(array) = self.array_assignment(block, assign) {
            match array {
                Ok(assignment) => assignment,
                Err(message) => {
                    self.error(message, assign.span);
                    return block;
                }
            }
        } else {
            let value = if self.lvalue_is_real(&assign.target) {
                self.real_expression(block, &assign.value)
            } else {
                let context = self.lvalue_width(&assign.target);
                self.assigned_value(block, &assign.value, context)
            };
            (vec![assign.target.clone()], vec![value])
        };
        if let Some(TimingControl::Event(event)) = &assign.timing
            && let Some(count) = &event.repeat
        {
            return self.repeated_assignment(
                block,
                assign,
                event,
                count,
                &targets,
                &carried,
                nonblocking,
            );
        }
        if nonblocking && let Some(control) = &assign.timing {
            let wait = match control {
                TimingControl::Delay(delay) => DigitalWait::Delay(self.delay(block, &delay.value)),
                TimingControl::Event(event) => self.event_wait(
                    block,
                    event,
                    Some(&DigitalStatement::BlockingAssign(assign.clone())),
                ),
            };
            for (target, value) in targets.iter().zip(&carried) {
                self.write_with_wait(block, target, *value, true, Some(wait.clone()));
            }
            return block;
        }
        let block = match &assign.timing {
            Some(control) => self.wait(block, control, None, &mut carried),
            None => block,
        };
        for (target, value) in targets.iter().zip(carried) {
            self.write(block, target, value, nonblocking);
        }
        block
    }

    fn repeated_assignment(
        &mut self,
        block: BlockId,
        assign: &DigitalAssign,
        event: &crate::ast::EventControl,
        count: &Expression,
        targets: &[DigitalLValue],
        captured: &[ValueId],
        nonblocking: bool,
    ) -> BlockId {
        let count = self.repeat_count(block, count);
        let condition = self.truth_value(block, count);
        let waiting = self.builder.create_block();
        let immediate = self.builder.create_block();
        let join = self.builder.create_block();
        self.builder.set_terminator(
            block,
            CfgTerminator::Branch {
                condition,
                then_target: waiting,
                then_args: Vec::new(),
                else_target: immediate,
                else_args: Vec::new(),
            },
        );
        let mut waiting_value: Vec<_> = captured
            .iter()
            .map(|&value| self.builder.carry_value(value, block, waiting))
            .collect();
        let waiting_count = self.builder.carry_value(count, block, waiting);
        let immediate_value: Vec<_> = captured
            .iter()
            .map(|&value| self.builder.carry_value(value, block, immediate))
            .collect();
        self.builder.seal_block(waiting);
        self.builder.seal_block(immediate);
        // A zero count bypasses evaluation of the event expression entirely.
        // The RHS was already sampled, so both paths use the same captured value.
        let mut guarded = assign.clone();
        guarded.timing = None;
        let event_wait = self.event_wait(
            waiting,
            event,
            Some(&DigitalStatement::BlockingAssign(guarded)),
        );
        let wait = DigitalWait::Repeat {
            count: waiting_count,
            event: Box::new(event_wait),
        };
        let waiting_exit = if nonblocking {
            for (target, &value) in targets.iter().zip(&waiting_value) {
                self.write_with_wait(waiting, target, value, true, Some(wait.clone()));
            }
            waiting
        } else {
            let resume = self.suspend(waiting, wait, &mut waiting_value);
            for (target, value) in targets.iter().zip(waiting_value) {
                self.write(resume, target, value, false);
            }
            resume
        };
        for (target, value) in targets.iter().zip(immediate_value) {
            self.write(immediate, target, value, nonblocking);
        }
        for exit in [waiting_exit, immediate] {
            self.builder.set_terminator(
                exit,
                CfgTerminator::Jump {
                    target: join,
                    args: Vec::new(),
                },
            );
        }
        self.builder.seal_block(join);
        join
    }

    fn repeat_count(&mut self, block: BlockId, expression: &Expression) -> ValueId {
        let real = self.is_real_expression(expression);
        let signed = real || self.self_signed(expression);
        let input = if real {
            self.real_expression(block, expression)
        } else {
            self.expression(block, expression)
        };
        let width = if real { 32 } else { self.value_width(input) };
        self.builder.push(
            block,
            CfgValueType::FourState { width },
            CfgValueKind::DigitalRepeatCount { input, signed },
        )
    }

    /// Lower an assignment's right-hand side under the context its target
    /// gives it, IEEE 1364-2005 sections 5.4.1, 5.4.2 and 5.2.1.
    ///
    /// The target's *width* seeds the sizing, because section 5.4.1 puts the
    /// left-hand side in the right-hand side's context. Its *signedness* does
    /// not enter: section 5.4.2 rule (a) says an expression's type depends only
    /// on its operands and not on what it is assigned to, so an unsigned target
    /// cannot make a signed right-hand side compute unsigned, and a signed
    /// target cannot rescue an unsigned one.
    ///
    /// The extension afterwards is the one step the write below cannot do. A
    /// value narrower than its target is padded wherever it lands — at the
    /// concatenation split, at [`Self::write_local`], in the interpreter's
    /// signal write — and every one of those zero-fills, which is section
    /// 5.2.1's rule for an unsigned expression and the wrong answer for a
    /// signed one. So only the signed half is emitted here; stating the
    /// unsigned half as well would put a node where the rule already applies.
    fn assigned_value(&mut self, block: BlockId, value: &Expression, width: u32) -> ValueId {
        if self.is_real_expression(value) {
            let input = self.real_expression(block, value);
            return self.builder.push(
                block,
                CfgValueType::FourState { width },
                CfgValueKind::DigitalRealToInteger { input, width },
            );
        }
        let signed = self.self_signed(value);
        let lowered = self.sized(block, value, Context { width, signed });
        if signed && self.value_width(lowered) < width {
            return self.resize(block, lowered, width, true);
        }
        lowered
    }

    /// Emit the write nodes for one target.
    ///
    /// A concatenation target becomes one write per element, over slices of
    /// the right-hand side taken from the most significant end down, which is
    /// what `{carry, sum} = ...` means.
    ///
    /// The right-hand side is resized to the concatenation's *total* width
    /// first. IEEE 1364-2005 section 5.2.1 makes the assignment context the
    /// whole left-hand side, so `{carry, sum} = src` with a one-bit `src`
    /// zero-extends and gives `carry` a 0. Slicing an unresized value instead
    /// reads bits that are not there, and section 5.2.1 makes those `x` — so
    /// the defect this fixes did not fail loudly, it wrote `x` into the top of
    /// every concatenation target narrower than the sum of its parts.
    fn write(&mut self, block: BlockId, target: &DigitalLValue, value: ValueId, nonblocking: bool) {
        self.write_with_wait(block, target, value, nonblocking, None);
    }

    fn write_with_wait(
        &mut self,
        block: BlockId,
        target: &DigitalLValue,
        value: ValueId,
        nonblocking: bool,
        wait: Option<DigitalWait>,
    ) {
        if self.refuse_real_in_concatenation(target) {
            return;
        }
        match target {
            DigitalLValue::Concat { elements, .. } => {
                let widths: Vec<u32> = elements
                    .iter()
                    .map(|part| self.lvalue_width(part))
                    .collect();
                let total: u32 = widths.iter().sum();
                let value = self.resize(block, value, total, false);
                let mut offset = total;
                for (element, width) in elements.iter().zip(widths) {
                    offset -= width;
                    let slice = self.builder.push(
                        block,
                        CfgValueType::FourState { width },
                        CfgValueKind::DigitalPartSelect {
                            input: value,
                            msb: i64::from(offset + width - 1),
                            lsb: i64::from(offset),
                        },
                    );
                    self.write_with_wait(block, element, slice, nonblocking, wait.clone());
                }
            }
            DigitalLValue::IndexedPartSelect(select) => {
                self.write_indexed_part(block, select, value, nonblocking, wait);
            }
            // Unobserved blocking-only locals remain SSA definitions. Deferred
            // or observed locals were promoted before lowering and use the
            // stored-write paths below.
            DigitalLValue::Identifier { name, .. } if self.ssa_local(name).is_some() => {
                let local = self.lookup_local(name).expect("just resolved");
                if nonblocking {
                    self.error(
                        format!(
                            "a nonblocking assignment to the process-local `{name}` has no \
                             lowered form yet: a deferred update needs a store to defer it \
                             into, and a process-local lives in the function"
                        ),
                        target.span(),
                    );
                    return;
                }
                self.write_local(block, local, value);
            }
            DigitalLValue::BitSelect { name, .. } | DigitalLValue::PartSelect { name, .. }
                if self.ssa_local(name).is_some() =>
            {
                let local = self.lookup_local(name).expect("just resolved");
                if nonblocking {
                    self.error(
                        format!("a nonblocking assignment to the process-local `{name}` requires shared local storage"),
                        target.span(),
                    );
                    return;
                }
                if self.local_is_real(local) {
                    self.error("packed assignment requires an integral local; a real has no selectable bits", target.span());
                    return;
                }
                let range = self.locals[usize::from(local)].bounds;
                let select = match target {
                    DigitalLValue::BitSelect { index, .. } => {
                        super::digital::DigitalArrayWriteSelect::Bit {
                            signed: self.self_signed(index),
                            index: self.array_index_value(block, index),
                        }
                    }
                    DigitalLValue::PartSelect { msb, lsb, span, .. } => {
                        let Some(selected) = self.part_select_bounds(msb, lsb, range, *span) else {
                            return;
                        };
                        super::digital::DigitalArrayWriteSelect::Part {
                            msb: selected.msb,
                            lsb: selected.lsb,
                        }
                    }
                    _ => unreachable!("partial local target"),
                };
                let width = self.lvalue_width(target);
                let value = self.resize(block, value, width, false);
                let input = self.read_local(block, local);
                let updated = self.builder.push(
                    block,
                    CfgValueType::FourState {
                        width: self.local_width(local),
                    },
                    CfgValueKind::DigitalPackedUpdate {
                        input,
                        bounds: (range.msb, range.lsb),
                        select,
                        value,
                    },
                );
                self.write_local(block, local, updated);
            }
            DigitalLValue::ArraySelect(access) => {
                let Some(array) = self.digital_array(&access.name) else {
                    self.error(
                        "packed selection requires a discrete unpacked array",
                        access.span,
                    );
                    return;
                };
                let rank = self.array_rank(&access.name);
                let Some((indices, packed)) = access.split(rank) else {
                    self.error(
                        "array subscript count does not match its declaration",
                        access.span,
                    );
                    return;
                };
                let (index, signed) = self.array_element_index(block, &access.name, &indices);
                let range = self.signals[usize::from(array.base)].declared_range();
                let select = match packed {
                    None => super::digital::DigitalArrayWriteSelect::Whole,
                    Some(crate::ast::PackedSelect::Bit(bit)) => {
                        super::digital::DigitalArrayWriteSelect::Bit {
                            signed: self.self_signed(bit),
                            index: self.array_index_value(block, bit),
                        }
                    }
                    Some(crate::ast::PackedSelect::Part { msb, lsb }) => {
                        let Some(selected) = self.part_select_bounds(msb, lsb, range, access.span)
                        else {
                            return;
                        };
                        super::digital::DigitalArrayWriteSelect::Part {
                            msb: selected.msb,
                            lsb: selected.lsb,
                        }
                    }
                };
                let value = if packed.is_none() && self.real_signal(array.base) {
                    value
                } else {
                    let width = packed.map_or_else(
                        || self.width_of(array.base),
                        |select| self.packed_select_width(select),
                    );
                    self.resize(block, value, width, false)
                };
                let kind = if nonblocking {
                    CfgValueKind::DigitalArrayNonblockingWrite {
                        array,
                        bounds: (range.msb, range.lsb),
                        index,
                        signed,
                        select,
                        value,
                        region: DigitalSchedulingRegion::NonBlockingAssign,
                        wait,
                    }
                } else {
                    CfgValueKind::DigitalArrayBlockingWrite {
                        array,
                        bounds: (range.msb, range.lsb),
                        index,
                        signed,
                        select,
                        value,
                    }
                };
                self.builder.push(block, CfgValueType::Effect, kind);
            }
            // IEEE 1364 9.2.1/9.2.2: blocking targets are evaluated after
            // the intra-assignment wait; NBA targets are captured at scheduling.
            DigitalLValue::BitSelect { name, index, .. } if self.digital_array(name).is_some() => {
                let array = self.digital_array(name).expect("resolved discrete array");
                let range = self.signals[usize::from(array.base)].declared_range();
                let signed = self.self_signed(index);
                let index = self.array_index_value(block, index);
                let value = if self.real_signal(array.base) {
                    value
                } else {
                    self.resize(block, value, self.width_of(array.base), false)
                };
                let kind = if nonblocking {
                    CfgValueKind::DigitalArrayNonblockingWrite {
                        select: super::digital::DigitalArrayWriteSelect::Whole,
                        array,
                        bounds: (range.msb, range.lsb),
                        index,
                        signed,
                        value,
                        region: DigitalSchedulingRegion::NonBlockingAssign,
                        wait,
                    }
                } else {
                    CfgValueKind::DigitalArrayBlockingWrite {
                        select: super::digital::DigitalArrayWriteSelect::Whole,
                        array,
                        bounds: (range.msb, range.lsb),
                        index,
                        signed,
                        value,
                    }
                };
                self.builder.push(block, CfgValueType::Effect, kind);
            }
            DigitalLValue::BitSelect { name, index, span } => {
                let Some(signal) = self.resolved_signal(name) else {
                    self.error(
                        "a procedural bit write requires digital variable storage",
                        *span,
                    );
                    return;
                };
                let range = self.signals[usize::from(signal)].declared_range();
                let select = super::digital::DigitalArrayWriteSelect::Bit {
                    signed: self.self_signed(index),
                    index: self.array_index_value(block, index),
                };
                let value = self.resize(block, value, 1, false);
                let bounds = (range.msb, range.lsb);
                let kind = if nonblocking {
                    CfgValueKind::DigitalPackedNonblockingWrite {
                        signal,
                        select,
                        bounds,
                        value,
                        region: DigitalSchedulingRegion::NonBlockingAssign,
                        wait,
                    }
                } else {
                    CfgValueKind::DigitalPackedBlockingWrite {
                        signal,
                        select,
                        bounds,
                        value,
                    }
                };
                self.builder.push(block, CfgValueType::Effect, kind);
            }
            _ => {
                let Some(resolved) = self.write_target(target) else {
                    return;
                };
                let kind = if nonblocking {
                    CfgValueKind::DigitalNonblockingWrite {
                        target: resolved,
                        value,
                        region: DigitalSchedulingRegion::NonBlockingAssign,
                        wait,
                    }
                } else {
                    CfgValueKind::DigitalBlockingWrite {
                        target: resolved,
                        value,
                    }
                };
                self.builder.push(block, CfgValueType::Effect, kind);
            }
        }
    }

    /// The element index of a write to a discrete array, with whether it is
    /// signed. Each coordinate is captured once, in source order, before a
    /// delayed NBA.
    fn array_element_index(
        &mut self,
        block: BlockId,
        name: &str,
        indices: &[&Expression],
    ) -> (ValueId, bool) {
        if let [index] = indices {
            return (
                self.array_index_value(block, index),
                self.self_signed(index),
            );
        }
        let dimensions = self
            .array_declaration(name)
            .expect("discrete array")
            .dimensions
            .clone();
        let indices = indices
            .iter()
            .map(|index| {
                let signed = self.self_signed(index);
                (self.array_index_value(block, index), signed)
            })
            .collect();
        (
            self.builder.push(
                block,
                CfgValueType::FourState { width: 64 },
                CfgValueKind::DigitalArrayOffset {
                    dimensions,
                    indices,
                },
            ),
            false,
        )
    }

    /// The constant width of an indexed part-select (IEEE 1364-2005 section
    /// 5.2.1), reported when it is not a positive bounded constant.
    fn indexed_part_width(&mut self, select: &IndexedPartSelectExpr) -> Option<u32> {
        let width = self
            .constant(&select.width)
            .and_then(|width| u32::try_from(width).ok())
            .filter(|width| (1..=crate::semantic::MAX_DIGITAL_VECTOR_WIDTH).contains(width));
        if width.is_none() {
            self.error(
                "the width of an indexed part-select must be a positive constant expression",
                select.width.span(),
            );
        }
        width
    }

    /// A procedural write through `name[base +: width]` or `name[base -:
    /// width]`, of a vector or of an array element (IEEE 1364-2005 section
    /// 5.2.1). The base is evaluated at the write, so a nonblocking write
    /// captures it when scheduled; bits outside the target, or every bit when
    /// the base has `x` or `z`, are left unwritten.
    fn write_indexed_part(
        &mut self,
        block: BlockId,
        select: &IndexedPartSelectExpr,
        value: ValueId,
        nonblocking: bool,
        wait: Option<DigitalWait>,
    ) {
        let Some(width) = self.indexed_part_width(select) else {
            return;
        };
        let name = &select.name;
        let value = self.resize(block, value, width, false);
        let packed = |lowerer: &mut Self, block| super::digital::DigitalArrayWriteSelect::Indexed {
            signed: lowerer.self_signed(&select.base),
            base: lowerer.array_index_value(block, &select.base),
            width,
            ascending: select.ascending,
        };
        if !select.indices.is_empty() {
            let Some(array) = self.digital_array(name) else {
                self.error(
                    "an indexed part-select after array coordinates requires a discrete unpacked array",
                    select.span,
                );
                return;
            };
            if self.array_rank(name) != select.indices.len() || self.real_signal(array.base) {
                self.error(
                    "an indexed part-select requires every unpacked coordinate of an integral array",
                    select.span,
                );
                return;
            }
            let indices: Vec<&Expression> = select.indices.iter().collect();
            let (index, signed) = self.array_element_index(block, name, &indices);
            let range = self.signals[usize::from(array.base)].declared_range();
            let bounds = (range.msb, range.lsb);
            let select = packed(self, block);
            let kind = if nonblocking {
                CfgValueKind::DigitalArrayNonblockingWrite {
                    array,
                    bounds,
                    index,
                    signed,
                    select,
                    value,
                    region: DigitalSchedulingRegion::NonBlockingAssign,
                    wait,
                }
            } else {
                CfgValueKind::DigitalArrayBlockingWrite {
                    array,
                    bounds,
                    index,
                    signed,
                    select,
                    value,
                }
            };
            self.builder.push(block, CfgValueType::Effect, kind);
            return;
        }
        if let Some(local) = self.ssa_local(name) {
            if nonblocking {
                self.error(
                    format!(
                        "a nonblocking assignment to the process-local `{name}` requires shared local storage"
                    ),
                    select.span,
                );
                return;
            }
            if self.local_is_real(local) {
                self.error(
                    "packed assignment requires an integral local; a real has no selectable bits",
                    select.span,
                );
                return;
            }
            let range = self.locals[usize::from(local)].bounds;
            let select = packed(self, block);
            let input = self.read_local(block, local);
            let updated = self.builder.push(
                block,
                CfgValueType::FourState {
                    width: self.local_width(local),
                },
                CfgValueKind::DigitalPackedUpdate {
                    input,
                    bounds: (range.msb, range.lsb),
                    select,
                    value,
                },
            );
            self.write_local(block, local, updated);
            return;
        }
        let Some(signal) = self.resolved_signal(name) else {
            self.error(
                "a procedural indexed part write requires digital variable storage",
                select.span,
            );
            return;
        };
        if self.real_signal(signal) {
            self.error(
                "packed assignment requires integral storage; a real has no selectable bits",
                select.span,
            );
            return;
        }
        let range = self.signals[usize::from(signal)].declared_range();
        let bounds = (range.msb, range.lsb);
        let select = packed(self, block);
        let kind = if nonblocking {
            CfgValueKind::DigitalPackedNonblockingWrite {
                signal,
                select,
                bounds,
                value,
                region: DigitalSchedulingRegion::NonBlockingAssign,
                wait,
            }
        } else {
            CfgValueKind::DigitalPackedBlockingWrite {
                signal,
                select,
                bounds,
                value,
            }
        };
        self.builder.push(block, CfgValueType::Effect, kind);
    }

    /// Emit the driver-write nodes for one continuous assignment's target.
    ///
    /// The same split a procedural concatenation target gets, and for the same
    /// reason — `assign {cout, sum} = a + b;` resizes to the total width and
    /// then distributes — but each element becomes a *separate driver*. That is
    /// what it is: two nets, each driven by one expression, and a resolver
    /// working on `cout` has no business being handed `sum`'s contribution.
    /// The net bit a MOS switch's data terminal names, when it names one: a
    /// scalar, or a constant bit select of a vector.
    fn passed_net_bit(&self, data: &Expression) -> Option<DigitalWriteTarget> {
        let (name, select) = match data {
            Expression::Identifier(identifier) => {
                (identifier.name.as_str(), DigitalWriteSelect::Whole)
            }
            Expression::ArrayAccess(access) if access.packed.is_none() => (
                access.array.as_str(),
                DigitalWriteSelect::Bit(self.constant(&access.index)?),
            ),
            // A one-bit constant part select names one bit too.
            Expression::Digital(crate::ast::DigitalExpr::PartSelect(select)) => {
                let msb = self.constant(&select.msb)?;
                (msb == self.constant(&select.lsb)?)
                    .then_some((select.name.as_str(), DigitalWriteSelect::Bit(msb)))?
            }
            _ => return None,
        };
        if self.lookup_local(name).is_some() || self.digital_array(name).is_some() {
            return None;
        }
        let signal = *self.index.get(name)?;
        let declared = &self.signals[usize::from(signal)];
        let scalar = declared.declared_range().width() == 1;
        (!declared.kind.is_real() && (scalar || select != DigitalWriteSelect::Whole))
            .then_some(DigitalWriteTarget { signal, select })
    }

    fn drive(
        &mut self,
        block: BlockId,
        target: &DigitalLValue,
        (value, enable): (ValueId, Option<ValueId>),
        process: DigitalProcessId,
        drivers: &mut Vec<DigitalDriver>,
    ) {
        if self.refuse_real_in_concatenation(target) {
            return;
        }
        match target {
            DigitalLValue::Concat { elements, .. } => {
                let widths: Vec<u32> = elements
                    .iter()
                    .map(|part| self.lvalue_width(part))
                    .collect();
                let total: u32 = widths.iter().sum();
                let value = self.resize(block, value, total, false);
                let mut offset = total;
                for (element, width) in elements.iter().zip(widths) {
                    offset -= width;
                    let slice = self.builder.push(
                        block,
                        CfgValueType::FourState { width },
                        CfgValueKind::DigitalPartSelect {
                            input: value,
                            msb: i64::from(offset + width - 1),
                            lsb: i64::from(offset),
                        },
                    );
                    self.drive(block, element, (slice, enable), process, drivers);
                }
            }
            _ => {
                let Some(resolved) = self.write_target(target) else {
                    return;
                };
                // Declaration order among this net's drivers, which is what
                // makes the identity stable across a recompilation.
                let index = drivers
                    .iter()
                    .filter(|driver| driver.id.signal == resolved.signal)
                    .count() as u32;
                let driver = DigitalDriverId {
                    signal: resolved.signal,
                    index,
                };
                drivers.push(DigitalDriver {
                    id: driver,
                    target: resolved.clone(),
                    process,
                    span: target.span().into(),
                });
                self.builder.push(
                    block,
                    CfgValueType::Effect,
                    CfgValueKind::DigitalDriverWrite {
                        driver,
                        target: resolved,
                        value,
                        enable,
                    },
                );
            }
        }
    }

    /// Resize a value to `width`, extending by `signed`.
    ///
    /// Built from the nodes the IR already has rather than from a resize node
    /// of its own: truncation is a part select of the low bits, zero-extension
    /// is a concatenation with a zero constant, and an exact fit is nothing at
    /// all. A dedicated node would need its own semantics in every consumer,
    /// and these already have theirs.
    ///
    /// Sign extension is built from them too, and is the standard's own idiom:
    /// `{{n{value[msb]}}, value}`, a concatenation of `n` copies of the sign
    /// bit above the value. That is section 5.4.1's extension of a signed
    /// operand and, because a part select copies the bit rather than testing
    /// it, section 4.3.2's rule for a sign position holding `x` or `z` — such a
    /// value extends with that bit and is unknown all the way up, which is what
    /// makes the extension honest about not knowing the sign.
    ///
    /// A zero-fill is section 5.2.1's assignment-context resizing, and does
    /// *not* propagate a leading `x` the way section 3.5.1 propagates one in a
    /// literal.
    fn resize(&mut self, block: BlockId, value: ValueId, width: u32, signed: bool) -> ValueId {
        let current = self.value_width(value);
        if current == width {
            return value;
        }
        if current > width {
            return self.builder.push(
                block,
                CfgValueType::FourState { width },
                CfgValueKind::DigitalPartSelect {
                    input: value,
                    msb: i64::from(width) - 1,
                    lsb: 0,
                },
            );
        }
        let mut parts = Vec::with_capacity((width - current + 1) as usize);
        if signed {
            let sign = self.builder.push(
                block,
                CfgValueType::FourState { width: 1 },
                CfgValueKind::DigitalPartSelect {
                    input: value,
                    msb: i64::from(current) - 1,
                    lsb: i64::from(current) - 1,
                },
            );
            parts.resize(usize::try_from(width - current).unwrap_or(0), sign);
        } else {
            parts.push(self.builder.push_leaf(
                CfgValueType::FourState {
                    width: width - current,
                },
                CfgValueKind::FourStateConstant(FourStateValue::zero(width - current)),
            ));
        }
        parts.push(value);
        self.builder.push(
            block,
            CfgValueType::FourState { width },
            CfgValueKind::DigitalConcat { parts },
        )
    }

    fn write_target(&mut self, target: &DigitalLValue) -> Option<DigitalWriteTarget> {
        match target {
            DigitalLValue::BitSelect { name, index, span }
                if self.digital_array(name).is_some() =>
            {
                return self.array_driver_target(name, &[index.as_ref()], None, *span);
            }
            DigitalLValue::ArraySelect(access) => {
                let Some((indices, packed)) = access.split(self.array_rank(&access.name)) else {
                    self.error(
                        "continuous array target requires all unpacked coordinates",
                        access.span,
                    );
                    return None;
                };
                return self.array_driver_target(&access.name, &indices, packed, access.span);
            }
            // A continuous target's base is constant (IEEE 1364-2005 section
            // A.8.5), so the select names one constant part.
            DigitalLValue::IndexedPartSelect(select) => {
                let width = self.indexed_part_width(select)?;
                let base = self.constant_index(&select.base)?;
                let Some((msb, lsb)) = super::digital::indexed_part_bounds(
                    self.declared_range_of(&select.name),
                    base,
                    width,
                    select.ascending,
                ) else {
                    self.error(
                        "an indexed part-select names indices outside the signed 64-bit range",
                        select.span,
                    );
                    return None;
                };
                let part = DigitalWriteSelect::Part { msb, lsb };
                if !select.indices.is_empty() {
                    let indices: Vec<&Expression> = select.indices.iter().collect();
                    let element =
                        self.array_driver_target(&select.name, &indices, None, select.span)?;
                    return Some(DigitalWriteTarget {
                        signal: element.signal,
                        select: part,
                    });
                }
                return match self.resolved_signal(&select.name) {
                    Some(signal) => Some(DigitalWriteTarget {
                        signal,
                        select: part,
                    }),
                    None => {
                        self.error(
                            "an indexed part-select target requires a discrete-domain signal",
                            select.span,
                        );
                        None
                    }
                };
            }
            _ => {}
        }
        let (name, span, select) = match target {
            DigitalLValue::Identifier { name, span } => (name, *span, DigitalWriteSelect::Whole),
            DigitalLValue::BitSelect { name, index, span } => {
                let index = self.constant_index(index)?;
                (name, *span, DigitalWriteSelect::Bit(index))
            }
            DigitalLValue::PartSelect {
                name,
                msb,
                lsb,
                span,
            } => {
                let selected =
                    self.part_select_bounds(msb, lsb, self.declared_range_of(name), *span)?;
                (
                    name,
                    *span,
                    DigitalWriteSelect::Part {
                        msb: selected.msb,
                        lsb: selected.lsb,
                    },
                )
            }
            DigitalLValue::ArraySelect(_) | DigitalLValue::IndexedPartSelect(_) => {
                unreachable!("array and indexed targets resolved above")
            }
            DigitalLValue::Concat { .. } => unreachable!("a concatenation is split before here"),
        };
        match self.resolved_signal(name) {
            Some(signal) => Some(DigitalWriteTarget { signal, select }),
            None => {
                self.error(
                    format!(
                        "`{name}` is not a discrete-domain signal; assigning a module-level \
                         analog variable from a process has no lowered form yet — declare \
                         the variable inside the process instead"
                    ),
                    span,
                );
                None
            }
        }
    }

    fn array_driver_target(
        &mut self,
        name: &str,
        indices: &[&Expression],
        packed: Option<&crate::ast::PackedSelect>,
        span: Span,
    ) -> Option<DigitalWriteTarget> {
        let array = self.array_declaration(name)?.clone();
        let indices: Vec<_> = indices
            .iter()
            .map(|index| self.constant_index(index))
            .collect::<Option<_>>()?;
        let layout = array.layout().ok()?;
        let Ok(offset) = layout.slot(&indices, 0) else {
            self.error(
                "continuous net array target requires in-range unpacked coordinates",
                span,
            );
            return None;
        };
        let select = match packed {
            None => DigitalWriteSelect::Whole,
            Some(crate::ast::PackedSelect::Bit(index)) => {
                DigitalWriteSelect::Bit(self.constant_index(index)?)
            }
            Some(crate::ast::PackedSelect::Part { msb, lsb }) => {
                let bounds =
                    self.part_select_bounds(msb, lsb, self.declared_range_of(name), span)?;
                DigitalWriteSelect::Part {
                    msb: bounds.msb,
                    lsb: bounds.lsb,
                }
            }
        };
        Some(DigitalWriteTarget {
            signal: DigitalSignalId::new(array.storage.base.index() + offset as u32),
            select,
        })
    }

    fn lvalue_width(&mut self, target: &DigitalLValue) -> u32 {
        match target {
            DigitalLValue::ArraySelect(access) => {
                if self.selects_array_element(access) {
                    self.digital_array(&access.name)
                        .map_or(1, |array| self.width_of(array.base))
                } else {
                    self.packed_select_width(&access.select)
                }
            }
            DigitalLValue::Identifier { name, .. } => match self.lookup_local(name) {
                Some(local) => self.local_width(local),
                None => self
                    .index
                    .get(name.as_str())
                    .map_or(1, |signal| self.width_of(*signal)),
            },
            DigitalLValue::BitSelect { name, .. } => self
                .digital_array(name)
                .map_or(1, |array| self.width_of(array.base)),
            DigitalLValue::PartSelect { msb, lsb, .. } => self.part_select_width(msb, lsb),
            DigitalLValue::IndexedPartSelect(select) => self.indexed_select_width(select),
            DigitalLValue::Concat { elements, .. } => {
                elements.iter().map(|part| self.lvalue_width(part)).sum()
            }
        }
    }

    /// Lower a timing control into a `Wait` and return the resume block.
    ///
    /// Everything the resumed half of the process needs travels through
    /// `resume_args`, because a suspension does not preserve the value table:
    /// the process stopped, and the kernel that starts it again does so from a
    /// resume state and nothing else. Static declarations cross even outside
    /// their lexical scope; synthesized counters cross while in scope.
    /// Values explicitly named by `carried` cross too, which is how
    /// `q = #5 d` gets the `d` it read *before* the delay (IEEE 1364-2005
    /// section 9.2.2) to the write that lands after it.
    ///
    /// Every such local crosses, not only the ones the resumed half reads.
    /// A liveness analysis would carry fewer; carrying one that is never read
    /// again costs a bound parameter and nothing else, and getting liveness
    /// wrong costs correctness.
    fn wait(
        &mut self,
        block: BlockId,
        control: &TimingControl,
        guarded: Option<&DigitalStatement>,
        carried: &mut [ValueId],
    ) -> BlockId {
        let wait = match control {
            TimingControl::Event(event) => self.event_wait(block, event, guarded),
            TimingControl::Delay(delay) => {
                let value = self.delay(block, &delay.value);
                DigitalWait::Delay(value)
            }
        };
        self.suspend(block, wait, carried)
    }

    fn suspend(&mut self, block: BlockId, wait: DigitalWait, carried: &mut [ValueId]) -> BlockId {
        let resume = self.builder.create_block();
        self.builder.set_terminator(
            block,
            CfgTerminator::Wait {
                wait,
                resume,
                resume_args: Vec::new(),
            },
        );
        for value in carried.iter_mut() {
            *value = self.builder.carry_value(*value, block, resume);
        }
        let carried_locals: Vec<_> = (0..self.static_local_count)
            .map(DigitalLocalId::from)
            .chain(
                self.locals_in_scope()
                    .into_iter()
                    .filter(|local| usize::from(*local) >= self.static_local_count),
            )
            .collect();
        for local in carried_locals {
            let declared = &self.locals[usize::from(local)];
            if declared.shared.is_some() || declared.slot.is_some() {
                continue;
            }
            self.builder
                .carry_variable(CfgVariable::DigitalLocal(local), block, resume);
        }
        self.builder.seal_block(resume);
        self.unwind_tests(resume)
    }

    /// Evaluate once at encounter, with the same sizing for constants and signals.
    fn delay(&mut self, block: BlockId, expression: &Expression) -> ValueId {
        let signed = self.self_signed(expression);
        let input = if self.is_real_expression(expression) {
            self.real_expression(block, expression)
        } else {
            self.expression(block, expression)
        };
        self.builder.push(
            block,
            CfgValueType::FourState { width: 64 },
            CfgValueKind::DigitalDelayTicks { input, signed },
        )
    }

    fn event_wait(
        &mut self,
        block: BlockId,
        event: &crate::ast::EventControl,
        guarded: Option<&DigitalStatement>,
    ) -> DigitalWait {
        let crate::ast::Sensitivity::Explicit(terms) = &event.sensitivity else {
            return DigitalWait::Event(self.sensitivity_terms(
                &event.sensitivity,
                guarded,
                event.span,
            ));
        };
        if terms.iter().all(|term| {
            signal_name(&term.signal).is_some_and(|name| {
                self.resolved_signal(name).is_some()
                    && (self.digital_array(name).is_none()
                        || self.assignment_array_source(name).is_some())
            })
        }) {
            return DigitalWait::Event(self.sensitivity_terms(
                &event.sensitivity,
                guarded,
                event.span,
            ));
        }
        let expressions = terms.iter().map(|term| {
            let mut reads = BTreeSet::new();
            collect_expression_reads(&term.signal, &mut reads);
            if reads.iter().any(|name| self.ssa_local(name).is_some()) {
                self.error(
                    "event expressions reading process-local storage require shared local event bindings",
                    term.span,
                );
            }
            let assignment = self.assignment_event_selection(&term.signal);
            let real = assignment.as_ref().map_or_else(|| self.is_real_expression(&term.signal), |(_,indices,_)| indices.len() == 1 && self.is_real_expression(indices[0]));
            if real && term.edge.is_some() {
                self.error(
                    "posedge/negedge require a bit-valued event expression; use value-change control for a real expression",
                    term.span,
                );
            }
            // Event observations re-run only this expression, including its
            // control flow, never the surrounding process or its writes.
            let outer = std::mem::replace(&mut self.builder, ProcessBuilder::new());
            let entry = self.builder.create_block();
            let value = if let Some((array, indices, _)) = &assignment {
                if indices.len() == 1 {
                    self.array_index_value(entry, indices[0])
                } else {
                    let dimensions = self.arrays[&array.base].dimensions.clone();
                    let indices = indices.iter().map(|index| {
                        let signed = self.self_signed(index);
                        (self.array_index_value(entry, index), signed)
                    }).collect();
                    self.builder.push(entry, CfgValueType::FourState { width: 64 }, CfgValueKind::DigitalArrayOffset { dimensions, indices })
                }
            } else if real { self.real_expression(entry, &term.signal) }
                else { self.expression(entry, &term.signal) };
            let ty = self.builder.value_type_of(value).expect("expression type");
            self.builder.set_terminator(entry, CfgTerminator::Return);
            self.builder.seal_all_blocks();
            let expression_builder = std::mem::replace(&mut self.builder, outer);
            let value = match expression_builder.finish_with_outputs(entry, &[value]) {
                Ok((function, outputs)) => self.builder.push(block, ty,
                    CfgValueKind::DigitalExpression { function: Box::new(function), result: outputs[0] }),
                Err(detail) => {
                    self.invariant(format!("invalid event expression CFG: {detail}"), term.span);
                    self.real_constant(0.0)
                }
            };
            super::digital::DigitalEventExpression {
                assignment: assignment.map(|(array,_,signed)| super::digital::DigitalAssignmentEventSelection { array, signed }),
                value, edge: term.edge.map(|edge| match edge {
                    EdgeKind::Posedge => DigitalEdge::Posedge,
                    EdgeKind::Negedge => DigitalEdge::Negedge,
                }),
            }
        }).collect();
        DigitalWait::Expressions(expressions)
    }

    fn assignment_event_selection<'expr>(
        &mut self,
        expression: &'expr Expression,
    ) -> Option<(
        super::digital::DigitalArrayRef,
        Vec<&'expr Expression>,
        bool,
    )> {
        let (name, indices) = match expression {
            Expression::ArrayAccess(access) => (&access.array, vec![access.index.as_ref()]),
            Expression::Digital(crate::ast::DigitalExpr::ArraySelect(access)) => {
                let (indices, packed) = access.split(self.array_rank(&access.name))?;
                if packed.is_some() {
                    return None;
                }
                (&access.name, indices)
            }
            _ => return None,
        };
        let source = self.assignment_array_source(name)?.clone();
        if !self.retained_analog_coordinates(&source, &indices) {
            self.error(format!("analog array `{source}` selection is not assigned exclusively in analog event statements"), expression.span());
        }
        let array = self.digital_array(name)?;
        let signed = indices.len() == 1 && self.self_signed(indices[0]);
        Some((array, indices, signed))
    }

    /// Resolve a sensitivity list to signal terms.
    ///
    /// `@*` is computed here from the guarded statement's read set, per IEEE
    /// 1364-2005 section 9.7.5. The front end deliberately does not
    /// materialize it: doing so needs the statement, and a stale copy stored
    /// beside the source would be worse than none.
    fn sensitivity_terms(
        &mut self,
        sensitivity: &crate::ast::Sensitivity,
        guarded: Option<&DigitalStatement>,
        span: Span,
    ) -> Vec<DigitalSensitivityTerm> {
        match sensitivity {
            crate::ast::Sensitivity::Explicit(terms) => terms.iter().flat_map(|term| {
                let name = signal_name(&term.signal);
                let Some(signal) = name.and_then(|name| self.resolved_signal(name)) else {
                    self.error("event expression has no executable signal dependency: computed and selected event expressions require lowering",term.signal.span());
                    return Vec::new();
                };
                if let Some(source) = name.and_then(|name| self.assignment_array_source(name)).cloned() {
                    if !self.retained_analog_read(&source,None) {
                        self.error(format!("analog array `{source}` is not assigned exclusively in analog event statements"),term.span);
                    }
                    return self.arrays[&signal].storage.cell_range().expect("validated occurrence array")
                        .map(|signal| DigitalSensitivityTerm { signal: DigitalSignalId::new(signal), edge: None }).collect();
                }
                vec![DigitalSensitivityTerm { signal, edge: term.edge.map(|edge| match edge {
                    EdgeKind::Posedge => DigitalEdge::Posedge, EdgeKind::Negedge => DigitalEdge::Negedge,
                }) }]
            }).collect(),
            crate::ast::Sensitivity::Implicit => {
                let terms = guarded.map(|statement| self.scoped_read_dependencies(statement))
                    .unwrap_or_default();
                if terms.is_empty() {
                    self.error(
                        "`@*` names no signal: the statement it guards reads none, \
                         so the process could never resume",
                        span,
                    );
                }
                terms.into_iter()
                    .map(|signal| DigitalSensitivityTerm { signal, edge: None })
                    .collect()
            }
        }
    }

    // ------------------------------------------------------------------
    // Real expressions
    // ------------------------------------------------------------------
    //
    // # Where the section 5.4.1 machinery hands off
    //
    // It does not run at all on a real. IEEE 1364-2005 sections 5.4.1 and
    // 5.4.2 size and sign an expression in bits, and section 5.1's table of
    // operators excludes real operands from every operator whose answer
    // depends on a bit pattern. A real has no width to maximise, no sign bit to
    // extend from, and no truncation to apply at an assignment — so the
    // two-pass `self_width` / `sized` walk is not "trivially satisfied" for one
    // but inapplicable, and giving it a real to size would invite a zero width
    // to propagate through it.
    //
    // The seam is therefore a *classification*, made once per expression by
    // [`Self::is_real_expression`], and every caller that could receive either
    // branches on it before entering `sized`:
    //
    //   * an assignment or driver whose target is real lowers its right-hand
    //     side with [`Self::real_expression`] and never calls `assigned_value`;
    //   * `sized` refuses a real that reached a position wanting bits;
    //   * a comparison inspects its operands first, because `a > 0.5` is a real
    //     expression's operands under a four-state result;
    //   * a branch condition converts a real with `!= 0.0`, section 9.4's
    //     "nonzero known value".
    //
    // VAMS-2023 4.2.1 defines numeric conversion independently of the real-net
    // connection rules in 3.7. Integral subexpressions keep their own sizing and
    // arithmetic, then convert numerically when a real operand/target requires it.

    /// Whether an expression's value is a real rather than four-state bits.
    ///
    /// Pure, and the counterpart of [`Self::self_width`] for the other domain:
    /// consulted before anything is emitted, so that the choice of which
    /// lowering an expression gets is made once.
    ///
    /// A comparison is deliberately *not* real however real its operands are —
    /// section 5.4.2 rule (g) makes every comparison one unsigned bit — and
    /// neither is a logical operator or a reduction.
    fn is_real_expression(&self, expression: &Expression) -> bool {
        expressions::shape(self, expression).real
    }

    fn is_real_expression_leaf(&self, expression: &Expression) -> bool {
        match expression {
            Expression::Call(call) if self.user_function(call).is_some() => self
                .user_function(call)
                .and_then(|function| function.result)
                .is_some_and(|result| result.real),
            Expression::Identifier(identifier)
                if self.constant_expression && identifier.name == "inf" =>
            {
                true
            }
            Expression::Call(call) if self.constant_expression => {
                constants::math_call(&call.name).is_some()
            }
            Expression::Number(number) => is_real_literal(&number.raw),
            Expression::Identifier(identifier) => match self.lookup_local(&identifier.name) {
                Some(local) => self.local_is_real(local),
                None => match self.index.get(identifier.name.as_str()) {
                    Some(signal) => self.real_signal(*signal),
                    // A `parameter real` is an elaboration constant (IEEE
                    // 1364-2005 section 12.2), so it classifies by the type its
                    // declaration wrote and folds to a literal below. Consulted
                    // after the signal table for the reason `constant` is: a
                    // name that denotes a signal is a runtime value and is
                    // never a constant, whatever else shares its spelling.
                    None => {
                        !self.constants.bits.contains_key(&identifier.name)
                            && (self
                                .analog_variables
                                .get(&identifier.name)
                                .filter(|v| v.array.is_none())
                                .map(|v| v.quantity)
                                == Some(super::digital::DigitalAnalogQuantity::RealVariable)
                                || self.constants.real(&identifier.name).is_some()
                                || self.constants.non_finite_real(&identifier.name).is_some())
                    }
                },
            },
            // `$bitstoreal` produces a real whatever its operand is, which is
            // the whole point of it: it is the standard's own crossing, and
            // classifying it by its operand would send it down the four-state
            // path it exists to leave.
            Expression::SystemFunction(function) => {
                function.name == "$bitstoreal"
                    || function.name == CDS_GET_ANALOG_VALUE
                    || self.module_time_query(function).is_some()
                    || super::digital::DigitalTimeQuery::from_name(&function.name)
                        .is_some_and(|query| query.bit_width().is_none())
            }
            // A probe of a continuous net is a real, whichever net it names
            // (Verilog-AMS LRM 2.4 section 7.3.3, and Table 7-1's converse —
            // a continuous quantity crossing into the discrete domain arrives
            // as the real it already is, because "the discrete domain can
            // fully represent all continuous types", section 7.3.3). It is
            // classified without looking at the operand for the same reason
            // `$bitstoreal` is: it is a crossing, not a computation over what
            // is on this side of it.
            Expression::ArrayAccess(access) => {
                self.digital_array(&access.array)
                    .is_some_and(|array| self.real_signal(array.base))
                    || self
                        .analog_array(&access.array)
                        .is_some_and(|(quantity, _, _)| {
                            quantity == super::digital::DigitalAnalogQuantity::RealVariable
                        })
            }
            Expression::BranchAccess(_) => true,
            _ => false,
        }
    }

    fn array_declaration(&self, name: &str) -> Option<&super::digital::DigitalArray> {
        if let Some(local) = self.lookup_local(name) {
            let (_, array) = self.locals[usize::from(local)].array?;
            return self
                .local_arrays
                .iter()
                .find(|item| item.storage.base == array.base)
                .or_else(|| self.arrays.get(&array.base));
        }
        if !self.array_names.contains(name) {
            return None;
        }
        self.index.get(name).and_then(|id| self.arrays.get(id))
    }

    fn digital_array(&self, name: &str) -> Option<super::digital::DigitalArrayRef> {
        self.array_declaration(name).map(|array| array.storage)
    }

    fn array_dimensions(&self, name: &str) -> Option<&[(i64, i64)]> {
        if let Some(array) = self.array_declaration(name) {
            return Some(&array.dimensions);
        }
        self.analog_array(name)?;
        Some(&self.analog_variables[name].dimensions)
    }

    fn array_rank(&self, name: &str) -> usize {
        self.array_dimensions(name)
            .map_or(0, |dimensions| dimensions.len().max(1))
    }

    fn selects_array_element(&self, access: &crate::ast::ArraySelectExpr) -> bool {
        access
            .split(self.array_rank(&access.name))
            .is_some_and(|(_, packed)| packed.is_none())
    }

    fn read_dependencies(&self, name: &str) -> Vec<DigitalSignalId> {
        if let Some(local) = self.lookup_local(name) {
            return self.local_read_dependencies(local);
        }
        self.module_read_dependencies(name)
    }

    fn module_read_dependencies(&self, name: &str) -> Vec<DigitalSignalId> {
        let Some(signal) = self
            .index
            .get(name)
            .or_else(|| {
                self.analog_variables
                    .get(name)
                    .and_then(|variable| variable.event_signal.as_ref())
                    .and_then(|signal| self.index.get(signal.as_str()))
            })
            .copied()
        else {
            return Vec::new();
        };
        self.array_declaration(name)
            .or_else(|| {
                self.analog_variables
                    .get(name)
                    .and_then(|variable| variable.event_signal.as_deref())
                    .and_then(|event| self.array_declaration(event))
            })
            .map_or_else(
                || vec![signal],
                |array| {
                    array
                        .storage
                        .cell_range()
                        .expect("validated array shape")
                        .map(DigitalSignalId::new)
                        .collect()
                },
            )
    }

    fn validate_analog_event_reads(&mut self, expression: &Expression) {
        crate::semantic::visit_expression(expression, &mut |expression| {
            let (name, indices) = match expression {
                Expression::Identifier(id) => (&id.name, None),
                Expression::ArrayAccess(access) => {
                    (&access.array, Some(vec![access.index.as_ref()]))
                }
                Expression::Digital(crate::ast::DigitalExpr::ArraySelect(access)) => {
                    let Some((indices, _)) = access.split(self.array_rank(&access.name)) else {
                        return;
                    };
                    (&access.name, Some(indices))
                }
                _ => return,
            };
            if self.lookup_local(name).is_none()
                && !self.index.contains_key(name.as_str())
                && self.analog_variables.contains_key(name)
                // Numeric localparams also have analog prologue variables. The
                // digital expression uses their elaborated constants and needs
                // no analog event subscription.
                && !self.constants.contains(name)
                && !indices.as_ref().map_or_else(
                    || self.retained_analog_read(name, None),
                    |indices| self.retained_analog_coordinates(name, indices),
                )
            {
                self.error(format!("analog variable `{name}` is not assigned exclusively in analog event statements and cannot provide an event dependency"),expression.span());
            }
        });
    }

    fn assignment_array_source(&self, name: &str) -> Option<&SmolStr> {
        self.analog_variables
            .iter()
            .find(|(_, variable)| {
                variable.array.is_some() && variable.event_signal.as_deref() == Some(name)
            })
            .map(|(name, _)| name)
    }

    fn retained_analog_read(&self, name: &str, index: Option<&Expression>) -> bool {
        if let Some(index) = index {
            return self.retained_analog_coordinates(name, &[index]);
        }
        self.analog_variables
            .get(name)
            .is_some_and(|variable| variable.event_assigned || variable.immutable)
    }

    fn retained_analog_coordinates(&self, name: &str, indices: &[&Expression]) -> bool {
        let Some(variable) = self.analog_variables.get(name) else {
            return false;
        };
        let Some(layout) = variable.array_layout() else {
            return false;
        };
        if layout.axes().len() != indices.len() {
            return false;
        }
        let coordinates: Vec<_> = indices
            .iter()
            .map(|index| {
                let mut reads = BTreeSet::new();
                collect_expression_reads(index, &mut reads);
                if reads.iter().any(|name| {
                    self.lookup_local(name).is_some() || self.index.contains_key(name.as_str())
                }) {
                    return None;
                }
                constants::scalar(index, self.constants, self.time_scale).and_then(|value| {
                    match value {
                        crate::numeric_literal::NumericLiteralValue::Integer(value) => Some(value),
                        crate::numeric_literal::NumericLiteralValue::Real(value) => {
                            crate::array_index::checked_rounded_i64(value).ok()
                        }
                    }
                })
            })
            .collect();
        if layout
            .axes()
            .iter()
            .zip(&coordinates)
            .any(|(axis, coordinate)| {
                coordinate.is_some_and(|coordinate| {
                    crate::array_index::checked_integer_array_slot(
                        coordinate,
                        0,
                        axis.len(),
                        axis.left.min(axis.right),
                    )
                    .is_err()
                })
            })
        {
            return false;
        }
        if variable.event_assigned || variable.immutable {
            return true;
        }
        // A fixed row with a dynamic column depends only on that row. A
        // continuously written cell outside the possible selection is irrelevant.
        (0..layout.len())
            .filter(|&slot| {
                layout
                    .indices(slot)
                    .expect("array cell")
                    .iter()
                    .zip(&coordinates)
                    .all(|(actual, expected)| expected.is_none_or(|expected| expected == *actual))
            })
            .all(|slot| {
                let element: SmolStr = crate::array_index::element_name(name, &layout, slot).into();
                self.analog_variables
                    .get(&element)
                    .is_some_and(|cell| cell.event_assigned || cell.immutable)
            })
    }

    fn bounded_part_width(msb: i64, lsb: i64) -> Option<u32> {
        let width = VectorBounds { msb, lsb }.width();
        (width <= crate::semantic::MAX_DIGITAL_VECTOR_WIDTH).then_some(width)
    }

    /// Inference uses a bounded placeholder on failure; actual lowering reports
    /// the source diagnostic before any value or assignment can be published.
    fn part_select_width(&self, msb: &Expression, lsb: &Expression) -> u32 {
        self.constant(msb)
            .zip(self.constant(lsb))
            .and_then(|(msb, lsb)| Self::bounded_part_width(msb, lsb))
            .unwrap_or(1)
    }

    /// An indexed part-select's width for inference; actual lowering reports
    /// a width that is not a positive bounded constant.
    fn indexed_select_width(&self, select: &IndexedPartSelectExpr) -> u32 {
        self.constant(&select.width)
            .and_then(|width| u32::try_from(width).ok())
            .filter(|width| (1..=crate::semantic::MAX_DIGITAL_VECTOR_WIDTH).contains(width))
            .unwrap_or(1)
    }

    fn packed_select_width(&self, select: &crate::ast::PackedSelect) -> u32 {
        match select {
            crate::ast::PackedSelect::Bit(_) => 1,
            crate::ast::PackedSelect::Part { msb, lsb } => self.part_select_width(msb, lsb),
        }
    }

    fn part_select_bounds(
        &mut self,
        msb: &Expression,
        lsb: &Expression,
        range: VectorBounds,
        span: Span,
    ) -> Option<VectorBounds> {
        let selected = VectorBounds {
            msb: self.constant_index(msb)?,
            lsb: self.constant_index(lsb)?,
        };
        self.validate_part_bounds(selected, range, span)
    }

    fn validate_part_bounds(
        &mut self,
        selected: VectorBounds,
        range: VectorBounds,
        span: Span,
    ) -> Option<VectorBounds> {
        if Self::bounded_part_width(selected.msb, selected.lsb).is_none() {
            self.error(
                "packed part-select width exceeds the supported vector width",
                span,
            );
            return None;
        }
        if selected.msb != selected.lsb && (selected.msb > selected.lsb) != (range.msb >= range.lsb)
        {
            self.error(
                "packed part-select runs against its declared direction",
                span,
            );
            return None;
        }
        Some(selected)
    }

    fn packed_named_value(
        &mut self,
        block: BlockId,
        name: &SmolStr,
        span: Span,
    ) -> Option<ValueId> {
        let named = Expression::Identifier(crate::ast::Identifier {
            name: name.clone(),
            span,
        });
        if self.is_real_expression(&named) {
            self.error(
                "packed selection requires an integral value; a real has no selectable bits",
                span,
            );
            return None;
        }
        Some(self.named_value(block, name, span))
    }

    fn digital_array_read_value(
        &mut self,
        block: BlockId,
        name: &str,
        index: ValueId,
        signed: bool,
    ) -> ValueId {
        let array = self.digital_array(name).expect("resolved discrete array");
        let value_type = if self.real_signal(array.base) {
            CfgValueType::Real
        } else {
            CfgValueType::FourState {
                width: self.width_of(array.base),
            }
        };
        self.builder.push(
            block,
            value_type,
            CfgValueKind::DigitalArrayRead {
                array,
                index,
                signed,
            },
        )
    }

    fn array_index_value(&mut self, block: BlockId, index: &Expression) -> ValueId {
        if self.is_real_expression(index) {
            self.real_expression(block, index)
        } else {
            self.expression(block, index)
        }
    }

    /// Whether an assignment target holds a real.
    fn lvalue_is_real(&self, target: &DigitalLValue) -> bool {
        match target {
            DigitalLValue::ArraySelect(access) => {
                self.selects_array_element(access)
                    && self
                        .digital_array(&access.name)
                        .is_some_and(|array| self.real_signal(array.base))
            }
            DigitalLValue::Identifier { name, .. } => match self.lookup_local(name) {
                Some(local) => self.local_is_real(local),
                None => self
                    .index
                    .get(name.as_str())
                    .is_some_and(|signal| self.real_signal(*signal)),
            },
            DigitalLValue::BitSelect { name, .. } => self
                .digital_array(name)
                .is_some_and(|array| self.real_signal(array.base)),
            // A select names bits, and a real has none. The refusal is the
            // analyzer's; reporting `false` here sends the target down the
            // four-state path, which is where that refusal already lives.
            _ => false,
        }
    }

    /// A real literal, as a leaf.
    fn real_constant(&mut self, value: f64) -> ValueId {
        self.builder
            .push_leaf(CfgValueType::Real, CfgValueKind::RealConstant(value))
    }

    /// A process read stays ordered, while its binding is shared plan-wide.
    /// Flow-source reads have already acquired simultaneous equations; reads
    /// of voltage-source currents retain their physical branch identity.
    fn analog_probe(&mut self, block: BlockId, access: &BranchAccess) -> ValueId {
        use super::digital::DigitalAnalogProbeTarget;
        if self.constant_expression {
            self.error(
                "an analog probe is not a constant expression",
                access.span(),
            );
            return self.real_constant(0.0);
        }
        let Some(quantity) = access.kind() else {
            self.invariant(
                "analog probe has no resolved physical quantity",
                access.span(),
            );
            return self.real_constant(0.0);
        };
        let (function, target) = match access {
            BranchAccess::Nodes {
                access, pos, neg, ..
            } => (
                access.clone(),
                DigitalAnalogProbeTarget::Nodes {
                    positive: pos.clone(),
                    negative: neg.clone(),
                },
            ),
            BranchAccess::Branch { access, name, .. } => (
                access.clone(),
                DigitalAnalogProbeTarget::Branch { name: name.clone() },
            ),
        };
        self.solution_probe(block, function, quantity.into(), target, access.span())
    }

    /// `$cds_get_analog_value(path [, 0 [, quantity]])`: a read of a deck
    /// object outside the module, resolved by the host. The analyzer has
    /// already refused every other argument shape, naming the call.
    fn external_probe(&mut self, block: BlockId, function: &crate::ast::SystemFunction) -> ValueId {
        use super::digital::{DigitalAnalogProbeTarget, DigitalAnalogQuantity};
        if self.constant_expression {
            self.error(
                format!("`{CDS_GET_ANALOG_VALUE}` is not a constant expression"),
                function.span,
            );
            return self.real_constant(0.0);
        }
        let Some(Expression::StringLit(path)) = function.args.first() else {
            self.invariant(
                format!("`{CDS_GET_ANALOG_VALUE}` reached lowering without a literal path"),
                function.span,
            );
            return self.real_constant(0.0);
        };
        let quantity = match function.args.get(2) {
            Some(Expression::StringLit(quantity)) if quantity.value == "flow" => {
                DigitalAnalogQuantity::Flow
            }
            _ => DigitalAnalogQuantity::Potential,
        };
        let target = DigitalAnalogProbeTarget::External {
            path: path.value.clone(),
        };
        self.solution_probe(
            block,
            function.name.clone(),
            quantity,
            target,
            function.span,
        )
    }

    /// Intern one read of the circuit solution plan-wide and read it here.
    fn solution_probe(
        &mut self,
        block: BlockId,
        function: SmolStr,
        quantity: super::digital::DigitalAnalogQuantity,
        target: super::digital::DigitalAnalogProbeTarget,
        span: Span,
    ) -> ValueId {
        let id = match self.probes.iter().position(|probe| {
            probe.access == function && probe.quantity == quantity && probe.target == target
        }) {
            Some(index) => DigitalAnalogProbeId::from(index),
            None => {
                let id = DigitalAnalogProbeId::from(self.probes.len());
                self.probes.push(DigitalAnalogProbe {
                    retained: false,
                    event_signal: None,
                    id,
                    access: function,
                    quantity,
                    target,
                    span: SourceSpanRef::from(span),
                });
                id
            }
        };
        let kind = match quantity {
            super::digital::DigitalAnalogQuantity::Flow => {
                CfgValueKind::DigitalAnalogFlow { probe: id }
            }
            _ => CfgValueKind::DigitalAnalogPotential { probe: id },
        };
        self.builder.push(block, CfgValueType::Real, kind)
    }

    fn analog_variable(&mut self, block: BlockId, name: &str, span: Span) -> ValueId {
        use super::digital::{DigitalAnalogProbeTarget, DigitalAnalogQuantity};
        let variable = self.analog_variables[name].clone();
        if variable.array.is_some() {
            self.error(
                format!("analog array `{name}` requires an element index"),
                span,
            );
        }
        let quantity = variable.quantity;
        let target = DigitalAnalogProbeTarget::Variable {
            name: variable.target,
        };
        let id = match self
            .probes
            .iter()
            .position(|probe| probe.target == target && probe.quantity == quantity)
        {
            Some(index) => DigitalAnalogProbeId::from(index),
            None => {
                let id = DigitalAnalogProbeId::from(self.probes.len());
                self.probes.push(DigitalAnalogProbe {
                    retained: variable.event_assigned || variable.immutable,
                    event_signal: None,
                    id,
                    access: name.into(),
                    quantity,
                    target,
                    span: span.into(),
                });
                id
            }
        };
        let value_type = if quantity == DigitalAnalogQuantity::IntegerVariable {
            CfgValueType::FourState { width: 32 }
        } else {
            CfgValueType::Real
        };
        self.builder.push(
            block,
            value_type,
            CfgValueKind::DigitalAnalogVariable {
                probe: id,
                array_index: None,
            },
        )
    }

    fn analog_array(
        &self,
        name: &str,
    ) -> Option<(super::digital::DigitalAnalogQuantity, i64, u32)> {
        if self.lookup_local(name).is_some() || self.index.contains_key(name) {
            return None;
        }
        let variable = self.analog_variables.get(name)?;
        let (lower, len) = variable.array?;
        Some((variable.quantity, lower, len))
    }

    fn analog_array_read_value(
        &mut self,
        block: BlockId,
        name: &str,
        span: Span,
        index: ValueId,
        signed: bool,
    ) -> ValueId {
        use super::digital::{DigitalAnalogProbeTarget, DigitalAnalogQuantity};
        let (quantity, lower, len) = self.analog_array(name).expect("array classified");
        let variable = &self.analog_variables[name];
        let layout = variable
            .array_layout()
            .expect("validated analog array shape");
        let element_name = |name: &str, offset: usize| -> SmolStr {
            crate::array_index::element_name(name, &layout, offset).into()
        };
        let target_name = &variable.target;
        // Reserve one contiguous binding group, shared by every read of this
        // array. Selection is O(1); no conditional chain or analog re-evaluation.
        let base = if let Some(base) = self.probes.windows(len as usize).position(|group| {
            group.iter().enumerate().all(|(offset, probe)| {
                probe.quantity == quantity
                    && probe.target
                        == DigitalAnalogProbeTarget::Variable {
                            name: element_name(target_name, offset),
                        }
            })
        }) {
            base
        } else {
            let base = self.probes.len();
            for offset in 0..len as usize {
                let local = element_name(name, offset);
                let name = element_name(target_name, offset);
                let retained = self
                    .analog_variables
                    .get(&local)
                    .is_some_and(|variable| variable.event_assigned || variable.immutable);
                self.probes.push(DigitalAnalogProbe {
                    retained,
                    event_signal: None,
                    id: DigitalAnalogProbeId::from(self.probes.len()),
                    access: name.clone(),
                    quantity,
                    target: DigitalAnalogProbeTarget::Variable { name },
                    span: span.into(),
                });
            }
            base
        };
        let value_type = if quantity == DigitalAnalogQuantity::IntegerVariable {
            CfgValueType::FourState { width: 32 }
        } else {
            CfgValueType::Real
        };
        self.builder.push(
            block,
            value_type,
            CfgValueKind::DigitalAnalogVariable {
                probe: DigitalAnalogProbeId::from(base),
                array_index: Some(super::cfg::DigitalAnalogArrayIndex {
                    index,
                    signed,
                    lower,
                    len,
                    dimensions: self.analog_variables[name].dimensions.clone(),
                }),
            },
        )
    }

    /// Resolve constant module declarations independently of the process clock.
    fn module_time_query(&self, function: &crate::ast::SystemFunction) -> Option<f64> {
        if !function.name.eq_ignore_ascii_case("$simparam")
            || !(1..=2).contains(&function.args.len())
        {
            return None;
        }
        let Expression::StringLit(name) = function.args.first()? else {
            return None;
        };
        self.time_scale.parameter_value(&name.value).ok().flatten()
    }

    /// Lower an expression that must produce a real.
    fn real_expression(&mut self, block: BlockId, expression: &Expression) -> ValueId {
        expressions::lower(self, block, expression, expressions::Mode::Real)
    }

    fn real_expression_leaf(&mut self, block: BlockId, expression: &Expression) -> ValueId {
        if self.constant_expression
            && matches!(expression, Expression::Identifier(identifier) if identifier.name == "inf")
        {
            return self.real_constant(f64::INFINITY);
        }
        if !self.is_real_expression(expression) {
            let signed = self.self_signed(expression);
            let input = self.expression(block, expression);
            return self.builder.push(
                block,
                CfgValueType::Real,
                CfgValueKind::DigitalIntegerToReal { input, signed },
            );
        }
        if let Expression::SystemFunction(function) = expression
            && let Some(query) = super::digital::DigitalTimeQuery::from_name(&function.name)
            && query.bit_width().is_none()
        {
            if self.constant_expression {
                self.error(
                    "a runtime clock query is not a constant expression",
                    function.span,
                );
                return self.real_constant(0.0);
            }
            return self.builder.push(
                block,
                query.value_type(),
                CfgValueKind::DigitalTime { query },
            );
        }
        if let Expression::SystemFunction(function) = expression
            && let Some(value) = self.module_time_query(function)
        {
            return self.real_constant(value);
        }
        if let Expression::SystemFunction(function) = expression
            && function.name == CDS_GET_ANALOG_VALUE
        {
            return self.external_probe(block, function);
        }
        match expression {
            Expression::Number(number) if is_real_literal(&number.raw) => {
                self.real_constant(number.value)
            }
            Expression::BranchAccess(access) => self.analog_probe(block, access),
            Expression::ArrayAccess(_) => unreachable!("iterative array read lowering"),
            Expression::Identifier(identifier) => {
                if self.digital_array(&identifier.name).is_some() {
                    self.error(
                        format!(
                            "unpacked array `{}` requires an element index",
                            identifier.name
                        ),
                        identifier.span,
                    );
                    return self.real_constant(0.0);
                }
                if let Some(local) = self.lookup_local(&identifier.name) {
                    if self.local_is_real(local) {
                        return self.read_local(block, local);
                    }
                    return self.not_a_real(
                        &identifier.name,
                        "a four-state process-local",
                        identifier.span,
                    );
                }
                match self.index.get(identifier.name.as_str()).copied() {
                    Some(signal) if self.real_signal(signal) => self.builder.push(
                        block,
                        CfgValueType::Real,
                        CfgValueKind::DigitalRealSignalRead { signal },
                    ),
                    Some(_) => {
                        self.not_a_real(&identifier.name, "a four-state signal", identifier.span)
                    }
                    // A `parameter real`. Section 12.2 fixes its value at
                    // elaboration, so it becomes the literal it denotes and no
                    // runtime machinery is involved — the same folding a
                    // replication count or a part-select bound already gets,
                    // in the other value domain.
                    None => match self.constants.real(&identifier.name) {
                        Some(value) => self.real_constant(value),
                        // A `parameter real` whose default folds to an
                        // infinity or a NaN is a legal declaration that the
                        // continuous domain accepts, and it is only here that
                        // it has nowhere to go. Saying it is not a signal
                        // would point at the declaration, which is fine; the
                        // refusal names the value instead.
                        None => match self.constants.non_finite_real(&identifier.name) {
                            Some(value) => {
                                self.error(
                                    format!(
                                        "`{}` folds to {value}, and a non-finite real has no \
                                         discrete-domain form; section 12.2 fixes a parameter's \
                                         value at elaboration, and no discrete-domain operation \
                                         defines one over an infinity or a NaN",
                                        identifier.name
                                    ),
                                    identifier.span,
                                );
                                self.real_constant(0.0)
                            }
                            None if self.analog_variables.contains_key(&identifier.name) => {
                                self.analog_variable(block, &identifier.name, identifier.span)
                            }
                            None => {
                                self.error(
                                    format!(
                                        "`{}` is not a discrete-domain signal",
                                        identifier.name
                                    ),
                                    identifier.span,
                                );
                                self.real_constant(0.0)
                            }
                        },
                    },
                }
            }
            Expression::Binary(_) | Expression::Unary(_) => {
                unreachable!("operators use the iterative expression lowerer")
            }
            Expression::Conditional(_) => unreachable!("iterative conditional lowering"),
            // `$bitstoreal(b)`: the crossing in the other direction. The
            // operand is sized to 64 bits here rather than taken as written,
            // because the pattern the standard names is a 64-bit one and a
            // narrower operand has to be extended to *be* one — section 5.2.1's
            // rule, applied at the only place that knows the width.
            Expression::SystemFunction(function) if function.name == "$bitstoreal" => {
                let Some(argument) = function.args.first() else {
                    return self.real_constant(0.0);
                };
                let input = self.sized(
                    block,
                    argument,
                    Context {
                        width: REAL_BIT_PATTERN_WIDTH,
                        signed: false,
                    },
                );
                let input = self.resize(block, input, REAL_BIT_PATTERN_WIDTH, false);
                self.builder.push(
                    block,
                    CfgValueType::Real,
                    CfgValueKind::DigitalBitsToReal { input },
                )
            }
            other => {
                self.error(
                    "this expression form has no real-valued lowering",
                    other.span(),
                );
                self.real_constant(0.0)
            }
        }
    }

    /// Apply the same numeric conversion at assignments and real operators.
    fn real_operand(&mut self, block: BlockId, expression: &Expression) -> ValueId {
        self.real_expression(block, expression)
    }

    /// Refuse a concatenation target with a real element, reporting whether it
    /// was refused.
    ///
    /// IEEE 1364-2005 section 5.1 does not admit a real operand in a
    /// concatenation, and the reason is the same on the left-hand side as on
    /// the right: a concatenation is a statement about bit positions, and a
    /// real occupies none. `{a, r} = ...` would have to invent a width for `r`
    /// to slice the right-hand side at.
    fn refuse_real_in_concatenation(&mut self, target: &DigitalLValue) -> bool {
        let DigitalLValue::Concat { elements, .. } = target else {
            return false;
        };
        let mut refused = false;
        for element in elements {
            if self.lvalue_is_real(element) {
                self.error(
                    "a real-valued name cannot be part of a concatenation target: IEEE \
                     1364-2005 section 5.1 admits no real operand in a concatenation, which \
                     divides a value by bit position",
                    element.span(),
                );
                refused = true;
            }
        }
        refused
    }

    /// A real-classified identifier must resolve to real storage or a constant.
    fn not_a_real(&mut self, name: &str, what: &str, span: Span) -> ValueId {
        self.invariant(
            format!("`{name}` resolved to {what} after real type classification"),
            span,
        );
        self.real_constant(0.0)
    }

    // ------------------------------------------------------------------
    // Expressions
    // ------------------------------------------------------------------

    fn value_width(&self, value: ValueId) -> u32 {
        self.builder
            .value_type_of(value)
            .and_then(CfgValueType::width)
            .unwrap_or(1)
    }

    /// Lower an expression used as a branch condition.
    ///
    /// The CFG's `Branch` reads a truth value, so a wider four-state value is
    /// reduced to one bit here rather than at every branch site.
    fn condition(&mut self, block: BlockId, expression: &Expression) -> ValueId {
        // IEEE 1364-2005 section 9.4: a condition is true when it evaluates to
        // a nonzero known value. For a real that is the whole rule — there is
        // no `x` to fall to the `else` for — so `!= 0.0` is the conversion, and
        // it is exact rather than tolerance-based because the standard's test
        // is an equality with zero and not a nearness to it.
        if self.is_real_expression(expression) {
            let value = self.real_expression(block, expression);
            let zero = self.real_constant(0.0);
            return self.builder.push(
                block,
                CfgValueType::FourState { width: 1 },
                CfgValueKind::DigitalRealCompare {
                    op: RealCompareOp::Ne,
                    left: value,
                    right: zero,
                },
            );
        }
        let value = self.expression(block, expression);
        self.truth_value(block, value)
    }

    /// Reduce a value to the one bit a `Branch` reads.
    fn truth_value(&mut self, block: BlockId, value: ValueId) -> ValueId {
        if self.value_width(value) == 1 {
            return value;
        }
        // `!!x` is the standard reduction to a truth value: the inner `!`
        // collapses the width and the outer one restores the sense.
        let negated = self.builder.push(
            block,
            CfgValueType::FourState { width: 1 },
            CfgValueKind::DigitalLogicalNot { input: value },
        );
        self.builder.push(
            block,
            CfgValueType::FourState { width: 1 },
            CfgValueKind::DigitalLogicalNot { input: negated },
        )
    }

    /// Lower an expression that no outer expression sizes.
    ///
    /// The self-determined case of [`Self::sized`], and the only one a caller
    /// outside the expression machinery wants: a `case` selector, a branch
    /// condition, a `repeat` count and an event term are all self-determined
    /// per IEEE 1364-2005 table 5-22. An assignment's right-hand side is not —
    /// see [`Self::assigned_value`] — and goes through `sized` with the
    /// target's width.
    fn expression(&mut self, block: BlockId, expression: &Expression) -> ValueId {
        self.sized(block, expression, Context::SELF_DETERMINED)
    }

    /// Lower `expression` under `context`, IEEE 1364-2005 sections 5.4.1 and
    /// 5.4.2.
    ///
    /// # The rule this implements
    ///
    /// Sizing an expression is two passes over one tree, and doing it in one
    /// is the defect this exists to prevent. The first pass is bottom-up:
    /// [`Self::self_width`] gives every expression its *self-determined* size
    /// from its operands alone, and [`Self::self_signed`] its self-determined
    /// signedness the same way. The second is top-down and is this function:
    /// the context size is the larger of the self-determined size and whatever
    /// the enclosing expression asks for, the context signedness is the
    /// enclosing expression's `and`ed with this expression's own, and both are
    /// pushed back down into the operands the standard calls
    /// *context-determined*, which are extended to them **before** the
    /// operation runs.
    ///
    /// Section 5.4.1 puts the assignment's left-hand side in that context. So
    /// `p = a * b` with four-bit operands and an eight-bit `p` multiplies at
    /// eight bits and yields 225, where multiplying at the operand width and
    /// widening the product afterwards yields 1. Both are total answers; only
    /// one is the language's.
    ///
    /// # Why the signedness rides the same context
    ///
    /// Section 5.5 settles both before evaluation and from the whole context,
    /// and section 5.4.2 rule (j) makes an expression signed only when *every*
    /// one of its context-determined operands is. So one unsigned operand makes
    /// the shared context unsigned, and that decision travels back down into
    /// the other operands exactly as the width does: in `p = (a + b) + c` with
    /// `a` and `b` signed and `c` a plain `reg`, the inner `a + b` is computed
    /// unsigned too. Carrying the signedness separately from the width would
    /// mean two walks that can disagree about which subexpression they are
    /// describing; carrying it in the same [`Context`] means the pair is
    /// decided once, at each node, and used together.
    ///
    /// # What is returned
    ///
    /// A value of exactly `max(self_width(expression), context.width)` bits
    /// when the expression is context-determined, and of exactly `self_width`
    /// when it is not. A self-determined expression is *not* padded here:
    /// whether its value needs extending depends on what consumes it, and the
    /// consumer that needs it — a context-determined operator — does it through
    /// [`expressions::Mode::Operand`], which is also the only place that knows whether to
    /// pad with zeros or with the sign bit.
    ///
    /// # The classification (table 5-22 and section 5.4.2)
    ///
    /// Context-determined operands, which receive the context: both sides of
    /// `+ - * / %`, of the bitwise `& | ^ ~^`, the operand of unary `~ + -`,
    /// the *left* operand of `<< >> >>>`, and both arms of `?:`. Each of those
    /// operators takes the context size as its result size, and is signed iff
    /// all of those operands are.
    ///
    /// Self-determined, which receive nothing: the right operand of a shift,
    /// every operand of a concatenation and its replication count, a reduction
    /// operand, the condition of `?:`, and both operands of a logical
    /// `&& || !`. A comparison's two operands size to each other and to nothing
    /// outside; the result of a comparison, a logical operator or a reduction
    /// is one *unsigned* bit whatever surrounds it, which is rules (g) and (h).
    ///
    /// Unsigned whatever their operands, per rules (d), (e) and (f): a
    /// bit-select, a part-select even of a whole vector, and a concatenation or
    /// replication. Those three are why a signed context can be lost inside an
    /// expression that reads nothing but signed declarations.
    ///
    /// Comparison operands receive their common width and signedness before
    /// either is evaluated. Extending only their final results would lose a
    /// carry from a narrower arithmetic operand.
    fn sized(&mut self, block: BlockId, expression: &Expression, context: Context) -> ValueId {
        expressions::lower(self, block, expression, expressions::Mode::Bits(context))
    }

    fn sized_leaf(&mut self, block: BlockId, expression: &Expression, context: Context) -> ValueId {
        // A real that reached a position wanting bits. Everything below sizes
        // and extends in bits, and a real has none — so it stops here by name
        // rather than being sized to zero and silently disappearing.
        if self.is_real_expression(expression) {
            self.error(
                "a real value has no four-state form here: Verilog-AMS LRM 2.4 section 3.7 \
                 converts one to bits with the explicit `$realtobits`, and this position needs \
                 bits",
                expression.span(),
            );
            return self.unknown(context.width.max(1));
        }
        let width = self.self_width(expression).max(context.width);
        match expression {
            Expression::Digital(crate::ast::DigitalExpr::FourState(literal)) => {
                // A sized literal keeps the width its author wrote and is
                // extended, if at all, as an ordinary operand. An unsized one
                // takes the context, padded by section 3.5.1's rule rather than
                // with zeros — `'bx` in a wide context is wide `x`.
                let value = match literal.value.declared_width {
                    Some(_) => FourStateValue::from_literal(&literal.value),
                    None => FourStateValue::from_bits_msb_first(&literal.value.bits_at(width)),
                };
                let width = value.width();
                self.builder.push_leaf(
                    CfgValueType::FourState { width },
                    CfgValueKind::FourStateConstant(value),
                )
            }
            Expression::StringLit(literal) => {
                let value = string_literal_value(&literal.value);
                self.builder.push_leaf(
                    CfgValueType::FourState {
                        width: value.width(),
                    },
                    CfgValueKind::FourStateConstant(value),
                )
            }
            Expression::Digital(
                crate::ast::DigitalExpr::ArraySelect(_)
                | crate::ast::DigitalExpr::PartSelect(_)
                | crate::ast::DigitalExpr::IndexedPartSelect(_),
            ) => {
                unreachable!("iterative packed selection lowering")
            }
            Expression::Digital(
                crate::ast::DigitalExpr::Xnor(_)
                | crate::ast::DigitalExpr::ArithmeticShiftRight(_)
                | crate::ast::DigitalExpr::CaseEquality(_)
                | crate::ast::DigitalExpr::Reduction(_),
            ) => unreachable!("iterative operator lowering"),
            Expression::Number(number) => {
                // IEEE 1364-2005 section 3.5.1: a *sized* literal is exactly as
                // wide as its author wrote it, and an unsized one is at least
                // 32 bits — and section 5.4.1 gives it the context's width when
                // that is larger, which is what `width` already is.
                //
                // The size is recovered from the literal's own source spelling,
                // because that is the only place it survives: the lexer routes a
                // based literal whose digits are all `0`/`1` to the integer
                // decoder, which keeps the number and drops the width.
                //
                // Reading it back is not cosmetic. Every operator that combines
                // widths depends on it, and a concatenation depends on nothing
                // else: `{a, b, c, 1'b1}` is four bits, while the same
                // concatenation holding a 32-bit `1` is thirty-five, whose low
                // four bits are a different value entirely.
                if let Ok(literal) = crate::four_state::decode(&number.raw) {
                    let value = match literal.declared_width {
                        Some(_) => FourStateValue::from_literal(&literal),
                        None => FourStateValue::from_bits_msb_first(&literal.bits_at(width)),
                    };
                    let width = value.width();
                    return self.builder.push_leaf(
                        CfgValueType::FourState { width },
                        CfgValueKind::FourStateConstant(value),
                    );
                }
                // A plain decimal with no base marker. Section 3.5.1 sizes one
                // as an unsized literal, so it too takes a wider context.
                // Recover the exact decimal integer from its spelling. The
                // scalar lexer cache is f64 and can lose low bits above 2^53.
                let value = match crate::numeric_literal::parse_integer_literal(&number.raw) {
                    Ok(Some(value)) => value,
                    _ => {
                        self.error(
                            "a discrete integer literal must have exact integer syntax",
                            number.span,
                        );
                        0
                    }
                };
                self.builder.push_leaf(
                    CfgValueType::FourState { width },
                    CfgValueKind::FourStateConstant(FourStateValue::from_integer(
                        width,
                        i128::from(value),
                    )),
                )
            }
            Expression::Identifier(identifier) => {
                self.named_value(block, &identifier.name, identifier.span)
            }
            Expression::ArrayAccess(_) | Expression::ArrayLiteral(_) => {
                unreachable!("iterative array and concatenation lowering")
            }
            // Both arms retain the common width and sign; the condition is
            // self-determined. Evaluation skips the unselected source arm.
            Expression::Conditional(_) => unreachable!("iterative conditional lowering"),
            Expression::Unary(_) | Expression::Binary(_) => {
                unreachable!("operators use the iterative expression lowerer")
            }
            Expression::SystemFunction(function)
                if super::digital::DigitalTimeQuery::from_name(&function.name).is_some() =>
            {
                if self.constant_expression {
                    self.error(
                        "a runtime clock query is not a constant expression",
                        function.span,
                    );
                    return self.unknown(width);
                }
                let query = super::digital::DigitalTimeQuery::from_name(&function.name).unwrap();
                self.builder.push(
                    block,
                    query.value_type(),
                    CfgValueKind::DigitalTime { query },
                )
            }
            // `$realtobits(x)` produces the real's explicit bit representation.
            Expression::SystemFunction(function) if function.name == "$realtobits" => {
                let Some(argument) = function.args.first() else {
                    // The analyzer refuses the arity; producing a
                    // width-correct unknown keeps this pass reporting its own
                    // findings rather than panicking on that one.
                    return self.unknown(REAL_BIT_PATTERN_WIDTH);
                };
                let input = self.real_operand(block, argument);
                self.builder.push(
                    block,
                    CfgValueType::FourState {
                        width: REAL_BIT_PATTERN_WIDTH,
                    },
                    CfgValueKind::DigitalRealToBits { input },
                )
            }
            other => {
                self.error(
                    "this expression form has no discrete-domain lowering",
                    other.span(),
                );
                self.unknown(1)
            }
        }
    }

    /// The self-determined signedness of an expression, IEEE 1364-2005 section
    /// 5.4.2.
    ///
    /// The bottom-up half of the signing, and the exact counterpart of
    /// [`Self::self_width`]: pure, emitting nothing, consulted by
    /// [`Self::sized`] for every expression it lowers so the two halves cannot
    /// disagree about which subexpression they describe.
    ///
    /// # The clause, rule by rule
    ///
    /// * **(a)** The type depends only on the operands, never on the left-hand
    ///   side. That is why this takes no context: an assignment cannot make its
    ///   right-hand side signed, and cannot make a signed one unsigned either.
    /// * **(b)** A decimal number with no base is signed. `-1` is therefore a
    ///   signed 32-bit value, which is the whole reason `a == -1` behaves
    ///   differently from `a == 32'hFFFFFFFF`.
    /// * **(c)** A based number is unsigned *unless* its base carries the `s`
    ///   marker: `4'd9` is unsigned, `4'sd9` is signed, and the two spell the
    ///   same four bits.
    /// * **(d), (e), (f)** A bit-select, a part-select, and a concatenation or
    ///   replication are unsigned regardless of their operands — a part-select
    ///   of a whole `reg signed` included. These three are how a signed
    ///   expression stops being one without any unsigned declaration in sight.
    /// * **(g), (h)** A comparison and a reduction yield an unsigned bit, and
    ///   so does a logical operator, whatever they were given.
    /// * **(j)** For everything with context-determined operands, the result is
    ///   signed iff *every* one of those operands is. One unsigned operand
    ///   makes the whole expression unsigned, and [`Self::sized`] then carries
    ///   that decision back down into the signed siblings.
    ///
    /// A form this cannot classify does not lower either, and answers unsigned
    /// — the classification of the all-`x` placeholder left after the refusal.
    fn self_signed(&self, expression: &Expression) -> bool {
        expressions::shape(self, expression).signed
    }

    fn self_signed_leaf(&self, expression: &Expression) -> bool {
        match expression {
            // Section 10.4.1: a call is as signed as the function's result.
            Expression::Call(call) if self.user_function(call).is_some() => self
                .user_function(call)
                .and_then(|function| function.result)
                .is_some_and(|result| result.signed),
            // Rule (c), from the source spelling: the marker survives decoding
            // into `FourStateLiteral::signed`.
            Expression::Digital(crate::ast::DigitalExpr::FourState(literal)) => {
                literal.value.signed
            }
            // Rules (b) and (c) together. A number that carries a base marker
            // is signed only with `s`; one that carries none is a plain decimal
            // and is signed. Read from the raw spelling because that is where
            // both facts live — `crate::four_state::decode` cannot be asked, as
            // an analog literal's raw text may not decode at all.
            Expression::Number(number) => {
                !number.raw.contains('\'') || crate::four_state::has_signed_marker(&number.raw)
            }
            // Table 5-21: a declaration is signed only when it says so, and an
            // `integer` says so by being one.
            Expression::Identifier(identifier) => match self.lookup_local(&identifier.name) {
                Some(local) => self.local_signed(local),
                None => self.index.get(identifier.name.as_str()).map_or_else(
                    || {
                        self.constants
                            .bits
                            .get(&identifier.name)
                            .is_some_and(|(_, signed)| *signed)
                            || self
                                .analog_variables
                                .get(&identifier.name)
                                .filter(|v| v.array.is_none())
                                .map(|v| v.quantity)
                                == Some(super::digital::DigitalAnalogQuantity::IntegerVariable)
                    },
                    |signal| self.signed_signal(*signal),
                ),
            },
            // Rules (d), (e) and (f).
            Expression::ArrayAccess(access) => {
                self.digital_array(&access.array)
                    .is_some_and(|array| self.signed_signal(array.base))
                    || self
                        .analog_array(&access.array)
                        .is_some_and(|(quantity, _, _)| {
                            quantity == super::digital::DigitalAnalogQuantity::IntegerVariable
                        })
            }
            Expression::Digital(crate::ast::DigitalExpr::PartSelect(_))
            | Expression::Digital(crate::ast::DigitalExpr::IndexedPartSelect(_))
            | Expression::Digital(crate::ast::DigitalExpr::ArraySelect(_))
            | Expression::ArrayLiteral(_) => false,
            // Rules (g) and (h).
            Expression::Digital(crate::ast::DigitalExpr::CaseEquality(_))
            | Expression::Digital(crate::ast::DigitalExpr::Reduction(_)) => false,
            _ => false,
        }
    }

    /// The self-determined size of an expression, IEEE 1364-2005 table 5-22.
    ///
    /// The bottom-up half of the sizing, and pure: it reads declarations and
    /// literals and emits nothing, so it can be asked before a single node
    /// exists. [`Self::sized`] consults it for every expression it lowers,
    /// which is what keeps the two halves from disagreeing — the width a node
    /// is emitted at is `max(self_width, context)` by construction rather than
    /// by a second derivation that happens to match.
    ///
    /// A form this cannot size is one that does not lower either; each such
    /// arm answers 1, which is the width of the all-`x` placeholder
    /// [`Self::unknown`] leaves behind after the refusal.
    fn self_width(&self, expression: &Expression) -> u32 {
        expressions::shape(self, expression).width
    }

    fn self_width_leaf(&self, expression: &Expression) -> u32 {
        match expression {
            // Section 10.4.1: a call is as wide as the function's result.
            Expression::Call(call) if self.user_function(call).is_some() => self
                .user_function(call)
                .and_then(|function| function.result)
                .map_or(1, |result| result.width()),
            // Unsized literals have a 32-bit floor and must also retain all
            // their significant bits before any enclosing context is applied.
            Expression::Digital(crate::ast::DigitalExpr::FourState(literal)) => {
                literal.value.width()
            }
            Expression::Digital(
                crate::ast::DigitalExpr::ArraySelect(_)
                | crate::ast::DigitalExpr::PartSelect(_)
                | crate::ast::DigitalExpr::IndexedPartSelect(_),
            ) => {
                unreachable!("iterative packed selection typing")
            }
            // Section 4.1.8: an identity comparison is one bit, and so is a
            // reduction of section 5.1.10.
            Expression::Digital(crate::ast::DigitalExpr::CaseEquality(_))
            | Expression::Digital(crate::ast::DigitalExpr::Reduction(_)) => 1,
            Expression::Number(number) => match crate::four_state::decode(&number.raw) {
                Ok(literal) => literal.width(),
                Err(_) => crate::numeric_literal::parse_integer_literal(&number.raw)
                    .ok()
                    .flatten()
                    .map(crate::numeric_literal::unsized_integer_width)
                    .unwrap_or(crate::four_state::UNSIZED_FOUR_STATE_WIDTH),
            },
            Expression::Identifier(identifier) => match self.lookup_local(&identifier.name) {
                Some(local) => self.local_width(local),
                None => self.index.get(identifier.name.as_str()).map_or_else(
                    || {
                        if let Some((value, _)) = self.constants.bits.get(&identifier.name) {
                            value.width()
                        } else if self.analog_variables.contains_key(&identifier.name) {
                            32
                        } else {
                            1
                        }
                    },
                    |signal| self.width_of(*signal),
                ),
            },
            Expression::ArrayAccess(access) => {
                if let Some(array) = self.digital_array(&access.array) {
                    self.width_of(array.base)
                } else if self.analog_array(&access.array).is_some() {
                    32
                } else {
                    1
                }
            }
            Expression::StringLit(literal) => string_literal_value(&literal.value).width(),
            Expression::ArrayLiteral(_) => unreachable!("iterative concatenation typing"),
            // `$realtobits` is 64 bits by the format it names, not by the
            // context it sits in: it is double-precision's own pattern, and a
            // narrower one would be a different pattern rather than a shorter
            // spelling of this one.
            Expression::SystemFunction(function) if function.name == "$realtobits" => {
                REAL_BIT_PATTERN_WIDTH
            }
            Expression::SystemFunction(function) => {
                super::digital::DigitalTimeQuery::from_name(&function.name)
                    .and_then(|query| query.bit_width())
                    .unwrap_or(1)
            }
            _ => 1,
        }
    }

    /// Lower a reduction operator, IEEE 1364-2005 section 5.1.10.
    ///
    /// # Why this is a desugaring rather than a node
    ///
    /// Section 4.1.10 does not define reduction as a new function. It defines
    /// it as *the section 5.1.9 bitwise operator applied successively across
    /// the bits of one operand*, and the `nand`/`nor`/`xnor` forms as the
    /// `and`/`or`/`xor` fold with the single-bit result inverted. So the
    /// faithful lowering is that iteration written out — one bit select per
    /// bit, one existing binary node per step — and a `CfgValueKind` of its own
    /// would be a second place to state a rule the tables already state.
    ///
    /// That is not merely cheaper. A new kind is four edits that must land
    /// together (`leaf_class`, the `is_digital` anchor, the AD guard, and the
    /// interpreter), and `leaf_class`'s catch-all would cache a reduction at
    /// module scope if the arm were missed — a defect no type error catches.
    ///
    /// # What it does to `x` and `z`
    ///
    /// Exactly what the tables do, which is the whole reason to build it out of
    /// them. `&{1'b0, 1'bx}` is `0`, not `x`, because `0` is AND's controlling
    /// value; `^{1'b0, 1'bx}` is `x`, because XOR has none. A lowering that
    /// poisoned the result whenever any operand bit was unknown would get the
    /// first of those wrong.
    ///
    /// A one-bit operand folds against the operator's identity (`1` for AND,
    /// `0` for OR and XOR), so `0`, `1` and `x` reduce to themselves and `z`
    /// to `x`: a reduction's result is a bitwise operator's output, and no row
    /// of those tables yields `z` (IEEE 1364-2005 section 5.1.11). Folding
    /// nothing would hand the `z` through unchanged.
    fn reduce_value(&mut self, block: BlockId, reduction: ReductionOp, input: ValueId) -> ValueId {
        let width = self.value_width(input);
        let op = match reduction {
            ReductionOp::And | ReductionOp::Nand => BitwiseOp::And,
            ReductionOp::Or | ReductionOp::Nor => BitwiseOp::Or,
            ReductionOp::Xor | ReductionOp::Xnor => BitwiseOp::Xor,
        };

        let bit = |lowerer: &mut Self, index: u32| {
            lowerer.builder.push(
                block,
                CfgValueType::FourState { width: 1 },
                CfgValueKind::DigitalPartSelect {
                    input,
                    msb: i64::from(index),
                    lsb: i64::from(index),
                },
            )
        };

        // Least significant bit first, so the fold reads the way the value is
        // indexed. The operators are associative and commutative over the
        // section 5.1.9 tables, so the direction is a readability choice.
        let mut folded = bit(self, 0);
        if width == 1 {
            let identity = self.builder.push_leaf(
                CfgValueType::FourState { width: 1 },
                CfgValueKind::FourStateConstant(FourStateValue::from_u64(
                    1,
                    u64::from(op == BitwiseOp::And),
                )),
            );
            folded = self.builder.push(
                block,
                CfgValueType::FourState { width: 1 },
                CfgValueKind::DigitalBitwise {
                    op,
                    left: identity,
                    right: folded,
                },
            );
        }
        for index in 1..width {
            let next = bit(self, index);
            folded = self.builder.push(
                block,
                CfgValueType::FourState { width: 1 },
                CfgValueKind::DigitalBitwise {
                    op,
                    left: folded,
                    right: next,
                },
            );
        }

        if !reduction.inverts() {
            return folded;
        }
        self.builder.push(
            block,
            CfgValueType::FourState { width: 1 },
            CfgValueKind::DigitalBitwiseNot { input: folded },
        )
    }

    /// Whether a comparison is made on signed numbers, IEEE 1364-2005 sections
    /// 5.1.6 and 5.4.2.
    ///
    /// The operands are context-determined *with respect to each other* and to
    /// nothing outside, so the comparison forms its own context — and rule (j)
    /// applies inside it: signed only when both operands are. A single
    /// unsigned operand makes the comparison unsigned, which is why `-1 < 0`
    /// stops holding the moment one side is a plain `reg`.
    ///
    /// The enclosing expression takes no part in either direction. Rule (g)
    /// makes the result an unsigned bit however the comparison was made, so a
    /// signed context outside cannot reach in, and an unsigned one cannot
    /// suppress a signed comparison within.
    fn comparison_is_signed(&self, left: &Expression, right: &Expression) -> bool {
        self.self_signed(left) && self.self_signed(right)
    }

    fn named_value(&mut self, block: BlockId, name: &str, span: Span) -> ValueId {
        if self.digital_array(name).is_some() {
            self.error(
                format!("unpacked array `{name}` requires an element index"),
                span,
            );
            return self.unknown(1);
        }
        // A process-local shadows a module signal of the same name, per IEEE
        // 1364-2005 section 9.8.1, so the innermost region is asked first.
        if let Some(local) = self.lookup_local(name) {
            return self.read_local(block, local);
        }
        match self.index.get(name) {
            Some(signal) => {
                let width = self.width_of(*signal);
                self.builder.push(
                    block,
                    CfgValueType::FourState { width },
                    CfgValueKind::DigitalSignalRead { signal: *signal },
                )
            }
            None if self.constants.bits.contains_key(name) => {
                let (value, _) = &self.constants.bits[name];
                self.builder.push_leaf(
                    CfgValueType::FourState {
                        width: value.width(),
                    },
                    CfgValueKind::FourStateConstant(value.clone()),
                )
            }
            None if self.analog_variables.contains_key(name) => {
                self.analog_variable(block, name, span)
            }
            None => {
                self.error(
                    format!(
                        "`{name}` is not a discrete-domain signal; reading a module-level \
                         analog variable from a process has no lowered form yet — declare \
                         the variable inside the process instead"
                    ),
                    span,
                );
                self.unknown(1)
            }
        }
    }

    /// A placeholder for an expression that failed to lower.
    ///
    /// Lowering continues after an error so that a second one is reported in
    /// the same pass; the value is all-`x` so that anything built on it is
    /// visibly unknown rather than accidentally plausible.
    fn unknown(&mut self, width: u32) -> ValueId {
        self.builder.push_leaf(
            CfgValueType::FourState { width },
            CfgValueKind::FourStateConstant(FourStateValue::splat(width, FourStateBit::Unknown)),
        )
    }

    /// The constant value of an expression in *this* body's scope.
    ///
    /// A literal, or a name the declaring module gave an integer parameter or
    /// localparam — IEEE 1364-2005 section 12.2 fixes both at elaboration. A
    /// name that also denotes a signal is never folded: a signal is a runtime
    /// value, and reading one as a constant would replace a whole design's
    /// behaviour with one number.
    fn constant(&self, expression: &Expression) -> Option<i64> {
        let mut reads = BTreeSet::new();
        collect_expression_reads(expression, &mut reads);
        if reads
            .iter()
            .any(|name| self.lookup_local(name).is_some() || self.index.contains_key(name.as_str()))
        {
            return None;
        }
        match constants::scalar(expression, self.constants, self.time_scale)? {
            crate::numeric_literal::NumericLiteralValue::Integer(value) => Some(value),
            crate::numeric_literal::NumericLiteralValue::Real(value) => {
                crate::semantic::SemanticAnalyzer::exact_const_i64(value)
            }
        }
    }

    fn constant_index(&mut self, expression: &Expression) -> Option<i64> {
        match self.constant(expression) {
            Some(index) => Some(index),
            None => {
                self.error(
                    "a bit or part select must have constant bounds representable as signed 64-bit integers",
                    expression.span(),
                );
                None
            }
        }
    }
}

/// Whether a numeric literal's source spelling is a *real* constant.
///
/// IEEE 1364-2005 section 3.5.2: a real constant is written either with a
/// decimal point between digits or with an exponent. Read from the raw
/// spelling, not from the decoded value, because that is where the distinction
/// survives — `2.0` and `2` decode to the same `f64` and are different
/// constants, one real and one a 32-bit integer.
///
/// A based literal is never one: `4'd2` carries a base marker, and section
/// 2.5.2 gives real constants no bases.
fn is_real_literal(raw: &str) -> bool {
    matches!(
        crate::numeric_literal::parse_numeric_literal(raw),
        Ok(crate::numeric_literal::NumericLiteralValue::Real(_))
    )
}

/// The real comparison an operator spells, if it spells one.
const fn real_compare_op(op: BinaryOp) -> Option<RealCompareOp> {
    Some(match op {
        BinaryOp::Lt => RealCompareOp::Lt,
        BinaryOp::Le => RealCompareOp::Le,
        BinaryOp::Gt => RealCompareOp::Gt,
        BinaryOp::Ge => RealCompareOp::Ge,
        BinaryOp::Eq => RealCompareOp::Eq,
        BinaryOp::Ne => RealCompareOp::Ne,
        _ => return None,
    })
}

/// The signal an event term names, if it names one directly.
fn signal_name(expression: &Expression) -> Option<&str> {
    match expression {
        Expression::Identifier(identifier) => Some(identifier.name.as_str()),
        _ => None,
    }
}

/// IEEE 1364-2005 section 3.6: a string operand is an unsigned constant of
/// eight bits per character, the first character most significant. The empty
/// string is one zero byte.
fn string_literal_value(text: &str) -> FourStateValue {
    let bytes = text.as_bytes();
    let width = u32::try_from(bytes.len().max(1))
        .ok()
        .and_then(|count| count.checked_mul(8))
        .filter(|width| *width <= crate::semantic::MAX_DIGITAL_VECTOR_WIDTH)
        .unwrap_or(crate::semantic::MAX_DIGITAL_VECTOR_WIDTH);
    let mut value = FourStateValue::zero(width);
    for (position, byte) in bytes.iter().rev().enumerate() {
        for bit in 0..8 {
            let at = position * 8 + bit;
            if (byte >> bit) & 1 == 1 && at < width as usize {
                value.set_bit(at as u32, FourStateBit::One);
            }
        }
    }
    value
}

fn collect_lvalue_index_reads(target: &DigitalLValue, reads: &mut BTreeSet<String>) {
    match target {
        DigitalLValue::Identifier { .. } => {}
        DigitalLValue::BitSelect { index, .. } => collect_expression_reads(index, reads),
        DigitalLValue::ArraySelect(select) => {
            for child in select.children() {
                collect_expression_reads(child, reads);
            }
        }
        DigitalLValue::PartSelect { msb, lsb, .. } => {
            collect_expression_reads(msb, reads);
            collect_expression_reads(lsb, reads);
        }
        DigitalLValue::IndexedPartSelect(select) => {
            for child in select.children() {
                collect_expression_reads(child, reads);
            }
        }
        DigitalLValue::Concat { elements, .. } => {
            for element in elements {
                collect_lvalue_index_reads(element, reads);
            }
        }
    }
}

fn collect_expression_reads(expression: &Expression, reads: &mut BTreeSet<String>) {
    let mut pending = vec![expression];
    while let Some(expression) = pending.pop() {
        match expression {
            Expression::Identifier(identifier) => {
                reads.insert(identifier.name.to_string());
            }
            Expression::SystemFunction(function) => pending.extend(&function.args),
            Expression::Call(call) => pending.extend(&call.args),
            Expression::ArrayAccess(access) => {
                reads.insert(access.array.to_string());
                pending.extend(access.children());
            }
            Expression::Digital(digital) => {
                if let Some(name) = digital.base_name() {
                    reads.insert(name.to_string());
                }
                pending.extend(digital.children());
            }
            Expression::Binary(binary) => {
                pending.push(&binary.right);
                pending.push(&binary.left);
            }
            Expression::Unary(unary) => pending.push(&unary.operand),
            Expression::Conditional(conditional) => {
                pending.extend([
                    &*conditional.condition,
                    &*conditional.then_expr,
                    &*conditional.else_expr,
                ]);
            }
            Expression::ArrayLiteral(literal) => {
                let mut elements: Vec<_> = literal.elements.iter().collect();
                while let Some(element) = elements.pop() {
                    match element {
                        ArrayLiteralElement::Value(expression) => pending.push(expression),
                        ArrayLiteralElement::Replication(replication) => {
                            pending.push(&replication.count);
                            elements.extend(&replication.elements);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
