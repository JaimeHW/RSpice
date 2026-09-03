//! Shared .MEAS evaluation harness over simulation results.
//!
//! Builds the signal-name table the [`MeasureEngine`] expects and evaluates
//! a netlist's measurement statements against
//! transient or DC-sweep data. Both the CLI and the Python bindings consume
//! this module so measurement semantics cannot drift between frontends.
//!
//! Signal naming: voltages are reachable as `V(name)`, `name`, `V(id)`, and
//! `id` in any case; branch currents as `I(name)`; the analysis axis as
//! `TIME` (the swept value plays that role for DC sweeps, so
//! `FIND TIME WHEN V(out)=...` addresses the sweep variable).

use std::collections::{HashMap, HashSet, VecDeque};

use super::measure::{
    AcceptedRowAtMatch, ContinuousMeasureResult, DelayConditionTracker, EdgeType,
    ErrorFunctionNorm, ExtremaOutput, LegacyFracDelayTracker, MeasureConditionDirection,
    MeasureEngine, MeasureOperand, MeasureResult, MeasureStatement, MeasureType, TrigSpec,
    TriggerEvent, WhenCondition, accepted_row_at_match, accepted_row_secant_slope,
    canonical_measure_signal_name,
};
use crate::Value;
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::{AcResult, NoiseContributionKind, NoiseContributionProbe};
use crate::engine::{SimulationError, TransientDeviceOpTrace, TransientResult};
use crate::netlist::expr::{ComplexValue, Expr as NetExpr, PreparedExpression, is_real};
use crate::netlist::{
    InterfaceNodeAliases, Netlist, NetlistSourceLocation, OutputAnalysisKind, OutputDirectiveKind,
    OutputNodeNamespace, OutputOperandKind, OutputRequest, SaveSignal, canonical_symbol,
    collect_output_node_namespace_with_limits_and_abort,
    collect_requested_interface_node_aliases_with_abort, is_current_output_accessor,
    is_current_projection_accessor, is_device_lead_current_accessor,
};
use crate::resource::{ResourceKind, ResourceLimitError, ResourceLimits};
use crate::solver::SimulationResult;

/// Insert `key` plus its lower/upper-case spellings.
fn insert_case_variants<'a>(
    signals: &mut HashMap<String, &'a [Value]>,
    key: &str,
    waveform: &'a [Value],
) {
    if key.is_empty() {
        return;
    }
    signals.insert(key.to_string(), waveform);
    let lower = key.to_ascii_lowercase();
    if lower != key {
        signals.insert(lower, waveform);
    }
    let upper = key.to_ascii_uppercase();
    if upper != key {
        signals.insert(upper, waveform);
    }
}

/// Insert `P(raw)` in every case combination of prefix and name.
fn insert_wrapped_variants<'a>(
    signals: &mut HashMap<String, &'a [Value]>,
    prefix: char,
    raw: &str,
    waveform: &'a [Value],
) {
    if raw.is_empty() {
        return;
    }
    let upper_prefix = prefix.to_ascii_uppercase();
    let lower_prefix = prefix.to_ascii_lowercase();
    for key in [
        format!("{upper_prefix}({raw})"),
        format!("{upper_prefix}({})", raw.to_ascii_lowercase()),
        format!("{upper_prefix}({})", raw.to_ascii_uppercase()),
        format!("{lower_prefix}({raw})"),
        format!("{lower_prefix}({})", raw.to_ascii_lowercase()),
        format!("{lower_prefix}({})", raw.to_ascii_uppercase()),
    ] {
        signals.insert(key, waveform);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MeasurementSignalDomain {
    Real,
    Complex,
}

impl MeasurementSignalDomain {
    fn for_analysis(analysis: OutputAnalysisKind) -> Result<Self, String> {
        match analysis {
            OutputAnalysisKind::Tran | OutputAnalysisKind::Dc => Ok(Self::Real),
            OutputAnalysisKind::Ac | OutputAnalysisKind::Hb | OutputAnalysisKind::Noise => {
                Ok(Self::Complex)
            }
            other => Err(format!(
                "interface-node measurement projection does not support {other:?} analysis"
            )),
        }
    }

    fn supports_voltage_accessor(self, accessor: &str) -> bool {
        match self {
            Self::Real => accessor == "V",
            Self::Complex => matches!(accessor, "V" | "VM" | "VR" | "VI" | "VP" | "VDB"),
        }
    }
}

/// Measurement-referenced subcircuit aliases plus accessor-specific ground
/// storage. Xyce materializes only requested aliases; retaining that policy
/// avoids multiplying every result table by every unused interface port.
struct InterfaceNodeAliasProjection {
    aliases: InterfaceNodeAliases,
    requested_accessors: HashMap<String, HashMap<String, HashSet<String>>>,
    domain: MeasurementSignalDomain,
    ground_zero: Option<Vec<Value>>,
    ground_decibels: Option<Vec<Value>>,
}

enum InterfaceNodeAliasProjectionError {
    Aborted,
    Detail(String),
}

impl InterfaceNodeAliasProjection {
    fn new(
        netlist: &Netlist,
        analysis: OutputAnalysisKind,
        point_count: usize,
    ) -> Result<Self, String> {
        match Self::new_with_abort(netlist, analysis, point_count, &NoAbort) {
            Ok(projection) => Ok(projection),
            Err(InterfaceNodeAliasProjectionError::Detail(detail)) => Err(detail),
            Err(InterfaceNodeAliasProjectionError::Aborted) => {
                unreachable!("NoAbort cannot cancel interface-alias projection")
            }
        }
    }

    fn new_with_abort(
        netlist: &Netlist,
        analysis: OutputAnalysisKind,
        point_count: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Self, InterfaceNodeAliasProjectionError> {
        if abort.is_aborted() {
            return Err(InterfaceNodeAliasProjectionError::Aborted);
        }
        let domain = MeasurementSignalDomain::for_analysis(analysis)
            .map_err(InterfaceNodeAliasProjectionError::Detail)?;
        let mut requested_accessors = HashMap::<String, HashMap<String, HashSet<String>>>::new();
        for (request_index, request) in
            netlist
                .output_requests
                .iter()
                .enumerate()
                .filter(|(_, request)| {
                    request.analysis.is_none_or(|owned| owned == analysis)
                        && matches!(
                            request.directive,
                            crate::netlist::OutputDirectiveKind::Measure
                                | crate::netlist::OutputDirectiveKind::Print
                                | crate::netlist::OutputDirectiveKind::Plot
                        )
                })
        {
            if request_index.is_multiple_of(64) && abort.is_aborted() {
                return Err(InterfaceNodeAliasProjectionError::Aborted);
            }
            for (dependency_index, dependency) in request.dependencies.iter().enumerate() {
                if dependency_index.is_multiple_of(64) && abort.is_aborted() {
                    return Err(InterfaceNodeAliasProjectionError::Aborted);
                }
                let accessor = dependency.operator.to_ascii_uppercase();
                if dependency.kind == crate::netlist::OutputSymbolKind::Node
                    && domain.supports_voltage_accessor(&accessor)
                {
                    requested_accessors
                        .entry(canonical_symbol(
                            netlist.ground_policy().canonical_node(&dependency.symbol),
                        ))
                        .or_default()
                        .entry(accessor)
                        .or_default()
                        .insert(dependency.symbol.clone());
                }
            }
        }
        let requested_aliases = requested_accessors
            .keys()
            .filter(|alias| alias.as_str() != "0")
            .cloned()
            .collect::<HashSet<_>>();
        let aliases =
            collect_requested_interface_node_aliases_with_abort(netlist, &requested_aliases, abort)
                .map_err(|error| match error {
                    crate::netlist::ParseWithAbortError::Aborted => {
                        InterfaceNodeAliasProjectionError::Aborted
                    }
                    crate::netlist::ParseWithAbortError::Parse(error) => {
                        InterfaceNodeAliasProjectionError::Detail(format!(
                            "failed to collect interface-node aliases: {error}"
                        ))
                    }
                })?;
        let direct_ground_accessors = requested_accessors.get("0");
        let mut needs_ground_zero = direct_ground_accessors
            .is_some_and(|accessors| accessors.keys().any(|accessor| accessor != "VDB"));
        let mut needs_ground_decibels =
            direct_ground_accessors.is_some_and(|accessors| accessors.contains_key("VDB"));
        for (alias, _) in aliases.iter().filter(|(_, target)| *target == "0") {
            let Some(accessors) = requested_accessors.get(alias) else {
                continue;
            };
            needs_ground_zero |= accessors.keys().any(|accessor| accessor != "VDB");
            needs_ground_decibels |= accessors.contains_key("VDB");
        }
        let allocate_ground = |value: Value| {
            let mut waveform = Vec::with_capacity(point_count);
            for index in 0..point_count {
                if index.is_multiple_of(64) && abort.is_aborted() {
                    return Err(InterfaceNodeAliasProjectionError::Aborted);
                }
                waveform.push(value);
            }
            Ok(waveform)
        };
        Ok(Self {
            aliases,
            requested_accessors,
            domain,
            ground_zero: needs_ground_zero
                .then(|| allocate_ground(0.0))
                .transpose()?,
            ground_decibels: needs_ground_decibels
                .then(|| allocate_ground(Value::NEG_INFINITY))
                .transpose()?,
        })
    }

    fn ground_waveform(&self, accessor: &str) -> Option<&[Value]> {
        if !self.domain.supports_voltage_accessor(accessor) {
            return None;
        }
        if accessor == "VDB" {
            self.ground_decibels.as_deref()
        } else {
            self.ground_zero.as_deref()
        }
    }

    /// Add only referenced voltage spellings supported by the active analysis.
    /// Existing physical signals win over aliases, matching Xyce's
    /// solution-node-first lookup. Both hierarchy separators are emitted so
    /// authored Xyce `:` paths address RSpice's canonical `.` flattening.
    fn augment<'a>(&'a self, signals: &mut HashMap<String, &'a [Value]>) -> Result<(), String> {
        match self.augment_with_abort(signals, &NoAbort) {
            Ok(()) => Ok(()),
            Err(InterfaceNodeAliasProjectionError::Detail(detail)) => Err(detail),
            Err(InterfaceNodeAliasProjectionError::Aborted) => {
                unreachable!("NoAbort cannot cancel interface-alias augmentation")
            }
        }
    }

    fn augment_with_abort<'a>(
        &'a self,
        signals: &mut HashMap<String, &'a [Value]>,
        abort: &dyn AbortSignal,
    ) -> Result<(), InterfaceNodeAliasProjectionError> {
        if abort.is_aborted() {
            return Err(InterfaceNodeAliasProjectionError::Aborted);
        }
        let physical_signals = CanonicalMeasureSignalIndex::new(signals);

        if let Some(accessors) = self.requested_accessors.get("0") {
            for (index, (accessor, authored_nodes)) in accessors.iter().enumerate() {
                if index.is_multiple_of(64) && abort.is_aborted() {
                    return Err(InterfaceNodeAliasProjectionError::Aborted);
                }
                if let Some(waveform) = self.ground_waveform(accessor) {
                    insert_interface_alias_spellings(
                        signals,
                        "0",
                        Some(accessor.as_str()),
                        authored_nodes,
                        waveform,
                    );
                    if accessor == "V" && self.domain == MeasurementSignalDomain::Real {
                        insert_interface_alias_spellings(
                            signals,
                            "0",
                            None,
                            authored_nodes,
                            waveform,
                        );
                    }
                }
            }
        }

        for (alias_index, (alias, target)) in self.aliases.iter().enumerate() {
            if alias_index.is_multiple_of(64) && abort.is_aborted() {
                return Err(InterfaceNodeAliasProjectionError::Aborted);
            }
            let Some(accessors) = self.requested_accessors.get(alias) else {
                continue;
            };
            for (accessor_index, (accessor, authored_aliases)) in accessors.iter().enumerate() {
                if accessor_index.is_multiple_of(64) && abort.is_aborted() {
                    return Err(InterfaceNodeAliasProjectionError::Aborted);
                }
                let alias_probe = format!("{accessor}({alias})");
                let target_waveform = if target == "0" {
                    self.ground_waveform(accessor)
                } else {
                    let target_probe = format!("{accessor}({target})");
                    physical_signals
                        .get(&target_probe)
                        .map_err(InterfaceNodeAliasProjectionError::Detail)?
                };
                let waveform = physical_signals
                    .get(&alias_probe)
                    .map_err(InterfaceNodeAliasProjectionError::Detail)?
                    .or(target_waveform);
                if let Some(waveform) = waveform {
                    insert_interface_alias_spellings(
                        signals,
                        alias,
                        Some(accessor.as_str()),
                        authored_aliases,
                        waveform,
                    );
                }

                // Real-domain TRAN/DC differential voltages resolve their
                // nodes from bare names. AC/NOISE maps deliberately omit those
                // names because their scalar V() accessor is a projection.
                if accessor == "V" && self.domain == MeasurementSignalDomain::Real {
                    let target_waveform = if target == "0" {
                        self.ground_zero.as_deref()
                    } else {
                        physical_signals
                            .get(target)
                            .map_err(InterfaceNodeAliasProjectionError::Detail)?
                    };
                    let waveform = physical_signals
                        .get(alias)
                        .map_err(InterfaceNodeAliasProjectionError::Detail)?
                        .or(target_waveform);
                    if let Some(waveform) = waveform {
                        insert_interface_alias_spellings(
                            signals,
                            alias,
                            None,
                            authored_aliases,
                            waveform,
                        );
                    }
                }
            }
        }
        if abort.is_aborted() {
            Err(InterfaceNodeAliasProjectionError::Aborted)
        } else {
            Ok(())
        }
    }
}

struct IndexedMeasureSignal<'a> {
    waveform: &'a [Value],
    first_name: String,
    conflicting_name: Option<String>,
}

/// Canonical, ambiguity-aware snapshot of a measurement signal table.
///
/// Case, hierarchy separators, and insignificant whitespace normalize once.
/// Distinct physical columns that collapse to one key are retained as a typed
/// lookup failure instead of depending on `HashMap` iteration order.
pub(crate) struct CanonicalMeasureSignalIndex<'a> {
    signals: HashMap<String, IndexedMeasureSignal<'a>>,
}

impl<'a> CanonicalMeasureSignalIndex<'a> {
    pub(crate) fn new(signals: &HashMap<String, &'a [Value]>) -> Self {
        let mut index = HashMap::<String, IndexedMeasureSignal<'a>>::with_capacity(signals.len());
        for (name, waveform) in signals {
            let canonical = canonical_measure_signal_name(name);
            index
                .entry(canonical)
                .and_modify(|existing| {
                    if existing.waveform.len() != waveform.len()
                        || !std::ptr::eq(existing.waveform.as_ptr(), waveform.as_ptr())
                    {
                        existing
                            .conflicting_name
                            .get_or_insert_with(|| name.to_string());
                    }
                })
                .or_insert_with(|| IndexedMeasureSignal {
                    waveform,
                    first_name: name.to_string(),
                    conflicting_name: None,
                });
        }
        Self { signals: index }
    }

    fn get(&self, requested: &str) -> Result<Option<&'a [Value]>, String> {
        let canonical = canonical_measure_signal_name(requested);
        self.get_canonical(requested, &canonical)
    }

    fn get_canonical(
        &self,
        requested: &str,
        canonical: &str,
    ) -> Result<Option<&'a [Value]>, String> {
        let Some(indexed) = self.signals.get(canonical) else {
            return Ok(None);
        };
        if let Some(conflicting_name) = &indexed.conflicting_name {
            return Err(format!(
                "measurement signal '{requested}' is ambiguous: '{}' and '{}' normalize to '{canonical}'",
                indexed.first_name, conflicting_name
            ));
        }
        Ok(Some(indexed.waveform))
    }
}

fn insert_interface_alias_spellings<'a>(
    signals: &mut HashMap<String, &'a [Value]>,
    canonical_alias: &str,
    accessor: Option<&str>,
    authored_aliases: &HashSet<String>,
    waveform: &'a [Value],
) {
    let colon_alias = canonical_alias.replace('.', ":");
    for alias in std::iter::once(canonical_alias)
        .chain(std::iter::once(colon_alias.as_str()))
        .chain(authored_aliases.iter().map(String::as_str))
    {
        let key = accessor
            .map(|accessor| format!("{accessor}({alias})"))
            .unwrap_or_else(|| alias.to_string());
        insert_case_variants(signals, &key, waveform);
    }
}

fn insert_device_op_trace_spellings<'a>(
    signals: &mut HashMap<String, &'a [Value]>,
    trace: &'a TransientDeviceOpTrace,
) {
    let operator = trace.parameter.to_ascii_uppercase();
    insert_case_variants(
        signals,
        &format!("@{}[{}]", trace.device_name, trace.parameter),
        trace.values.as_slice(),
    );
    insert_case_variants(
        signals,
        &format!("{}:{}", trace.device_name, trace.parameter),
        trace.values.as_slice(),
    );
    if is_device_lead_current_accessor(&operator) {
        insert_case_variants(
            signals,
            &format!("{operator}({})", trace.device_name),
            trace.values.as_slice(),
        );
    }
}

/// Build the measurement signal table for a transient result.
pub fn transient_signal_map(result: &TransientResult) -> HashMap<String, &[Value]> {
    let mut signals: HashMap<String, &[Value]> = HashMap::new();

    // The time axis itself, so `FIND TIME WHEN V(out)=...` works.
    insert_case_variants(&mut signals, "Time", result.time.as_slice());

    let authoritative_node_names = result
        .node_names
        .iter()
        .filter(|name| !name.is_empty())
        .map(|name| name.to_ascii_lowercase())
        .collect::<HashSet<_>>();
    for (index, waveform) in result.voltages.iter().enumerate() {
        let fallback = (index + 1).to_string();
        let raw = if let Some(name) = result.node_names.get(index).filter(|name| !name.is_empty()) {
            name.clone()
        } else {
            // An ordinal is a last-resort name only when result metadata is
            // absent. It must never replace an authoritative numeric SPICE
            // node that happens to occupy a different solver position.
            if authoritative_node_names.contains(&fallback.to_ascii_lowercase()) {
                continue;
            }
            fallback
        };

        insert_wrapped_variants(&mut signals, 'V', &raw, waveform.as_slice());
        insert_case_variants(&mut signals, &raw, waveform.as_slice());
    }

    for (index, waveform) in result.branch_currents.iter().enumerate() {
        if let Some(name) = result.branch_names.get(index).filter(|n| !n.is_empty()) {
            insert_wrapped_variants(&mut signals, 'I', name, waveform.as_slice());
        }
    }

    // Xyce's exactly two-character I* operators include device-lead currents
    // (ID/IG/IS/IB, IC/IB/IE, and hierarchical variants). RSpice
    // records those canonical device outputs in the typed operating-point
    // trace namespace rather than pretending that every lead is an MNA branch.
    for trace in &result.device_op_traces {
        insert_device_op_trace_spellings(&mut signals, trace);
    }
    for trace in &result.store_traces {
        insert_case_variants(&mut signals, &trace.name, trace.values.as_slice());
    }

    signals
}

/// One continuous Xyce `PARAM`/`EQN` measurement evaluated at every
/// transient result point.
#[derive(Debug, Clone, PartialEq)]
pub struct EquationMeasureTrace {
    pub name: String,
    pub values: Vec<Value>,
    /// Per-row IEEE numeric validity, always aligned one-to-one with `values`.
    /// Xyce exposes an undefined live measure as a raw NaN and can overwrite
    /// it at a later accepted point; retaining that value and an explicit
    /// validity bit preserves both behaviors.
    pub valid: Vec<bool>,
    /// Whether at least one point landed inside the measure's time window.
    pub initialized: bool,
}

struct LiveMeasureProgram<'a> {
    statement: &'a MeasureStatement,
    canonical_name: String,
    state: Option<LiveMeasureState>,
    current: Value,
    initialized: bool,
    values: Vec<Value>,
    valid: Vec<bool>,
    store_trace: bool,
    has_file_error_dependency: bool,
    failure: Option<String>,
}

/// Mutable view of live scalar results at one accepted analysis point.
/// Reading a FileError result is observable in Xyce: the first actual getter
/// read freezes its prefix aggregate. Keeping that operation here lets lazy
/// expression evaluation trigger it at the selected branch, rather than from
/// a conservative dependency pre-pass.
struct LiveMeasureReadContext<'program, 'netlist> {
    programs: &'program mut [LiveMeasureProgram<'netlist>],
    current_values: &'program mut HashMap<String, Value>,
    program_indices: &'program HashMap<String, usize>,
    row: usize,
    axis: &'program [Value],
}

impl LiveMeasureReadContext<'_, '_> {
    fn read_measure(&mut self, canonical_name: &str) -> Option<Value> {
        if let Some(&program_index) = self.program_indices.get(canonical_name)
            && freeze_live_file_error(&mut self.programs[program_index], self.row, self.axis)
        {
            let program = &self.programs[program_index];
            *self
                .current_values
                .get_mut(&program.canonical_name)
                .expect("compiled measure value slot") = program.current;
        }
        self.current_values.get(canonical_name).copied()
    }
}

#[derive(Debug)]
struct LiveMeasureOperand {
    authored: String,
    canonical_authored: String,
    canonical_signal: String,
    is_axis_symbol: bool,
    expression: Option<Box<LivePreparedExpression>>,
    dependencies: Vec<String>,
}

#[derive(Debug)]
struct LivePreparedExpression {
    evaluator: PreparedExpression,
    probes: HashMap<String, LiveRawOutputOperator>,
    parameters: HashMap<String, LiveExpressionParameter>,
}

#[derive(Debug)]
struct LiveExpressionParameter {
    canonical_measure: String,
    canonical_signal: String,
    is_axis_symbol: bool,
    context_value: Option<ComplexValue>,
}

enum LivePreparedExpressionCompileError {
    Aborted,
    Detail(String),
}

impl LivePreparedExpression {
    fn compile(
        expression: &NetExpr,
        params: &crate::netlist::ParamContext,
    ) -> Result<Self, String> {
        match Self::compile_with_abort(expression, params, &NoAbort) {
            Ok(expression) => Ok(expression),
            Err(LivePreparedExpressionCompileError::Detail(detail)) => Err(detail),
            Err(LivePreparedExpressionCompileError::Aborted) => {
                unreachable!("NoAbort cannot cancel live expression compilation")
            }
        }
    }

    fn compile_with_abort(
        expression: &NetExpr,
        params: &crate::netlist::ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<Self, LivePreparedExpressionCompileError> {
        fn rewrite_probes(
            expression: &NetExpr,
            probes: &mut HashMap<String, LiveRawOutputOperator>,
            abort: &dyn AbortSignal,
        ) -> Result<NetExpr, LivePreparedExpressionCompileError> {
            if abort.is_aborted() {
                return Err(LivePreparedExpressionCompileError::Aborted);
            }
            Ok(match expression {
                NetExpr::UnaryOp { op, operand } => NetExpr::UnaryOp {
                    op: *op,
                    operand: Box::new(rewrite_probes(operand, probes, abort)?),
                },
                NetExpr::BinOp { op, left, right } => NetExpr::BinOp {
                    op: *op,
                    left: Box::new(rewrite_probes(left, probes, abort)?),
                    right: Box::new(rewrite_probes(right, probes, abort)?),
                },
                NetExpr::FnCall { name, args }
                    if is_equation_probe_accessor(name)
                        || is_equation_noise_accessor(name)
                        || is_equation_generic_output_accessor(name) =>
                {
                    let prefix = name.to_ascii_uppercase();
                    let arguments = args
                        .iter()
                        .map(|argument| {
                            equation_probe_argument(Some(argument)).ok_or_else(|| {
                                LivePreparedExpressionCompileError::Detail(format!(
                                    "{prefix}() in continuous measure has an invalid argument"
                                ))
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    let authored = format!("{prefix}({})", arguments.join(","));
                    // NUL cannot occur in a parsed identifier. The prepared
                    // compiler also promotes these keys to typed External
                    // nodes, keeping the internal probe namespace disjoint
                    // from every authored parameter name.
                    let key = format!("\0RSPICE_LIVE_PROBE_{}", probes.len());
                    probes.insert(
                        key.clone(),
                        LiveRawOutputOperator::compile(&authored)
                            .map_err(LivePreparedExpressionCompileError::Detail)?,
                    );
                    NetExpr::Param(key)
                }
                NetExpr::FnCall { name, args } => NetExpr::FnCall {
                    name: name.clone(),
                    args: args
                        .iter()
                        .map(|argument| rewrite_probes(argument, probes, abort))
                        .collect::<Result<Vec<_>, _>>()?,
                },
                NetExpr::Number(_)
                | NetExpr::ComplexNumber(_)
                | NetExpr::StringLiteral(_)
                | NetExpr::Param(_) => expression.clone(),
            })
        }

        let mut probes = HashMap::new();
        let rewritten = rewrite_probes(expression, &mut probes, abort)?;
        if abort.is_aborted() {
            return Err(LivePreparedExpressionCompileError::Aborted);
        }
        let external_parameters = probes.keys().cloned().collect::<HashSet<_>>();
        let evaluator = if external_parameters.is_empty() {
            PreparedExpression::compile(&rewritten, params)
        } else {
            PreparedExpression::compile_with_external_parameters(
                &rewritten,
                params,
                &external_parameters,
            )
        }
        .map_err(|error| {
            LivePreparedExpressionCompileError::Detail(format!(
                "failed to prepare live expression: {error}"
            ))
        })?;
        if abort.is_aborted() {
            return Err(LivePreparedExpressionCompileError::Aborted);
        }
        let mut parameters = HashMap::new();
        let mut compilation_aborted = false;
        evaluator.visit_runtime_parameters(|name| {
            if abort.is_aborted() {
                compilation_aborted = true;
                return;
            }
            if !probes.contains_key(name) {
                let canonical_measure = name.to_ascii_uppercase();
                parameters
                    .entry(name.to_string())
                    .or_insert_with(|| LiveExpressionParameter {
                        context_value: params.get_complex(name),
                        is_axis_symbol: matches!(
                            canonical_measure.as_str(),
                            "TIME" | "FREQ" | "FREQUENCY" | "HERTZ"
                        ),
                        canonical_signal: canonical_measure_signal_name(name),
                        canonical_measure,
                    });
            }
        });
        if compilation_aborted || abort.is_aborted() {
            return Err(LivePreparedExpressionCompileError::Aborted);
        }
        Ok(Self {
            evaluator,
            probes,
            parameters,
        })
    }

    fn value(
        &mut self,
        row: usize,
        signals: &CanonicalMeasureSignalIndex<'_>,
        reads: &mut LiveMeasureReadContext<'_, '_>,
        params: &crate::netlist::ParamContext,
    ) -> Result<ComplexValue, String> {
        let probes = &self.probes;
        let parameters = &self.parameters;
        self.evaluator
            .evaluate_with(params, &mut |name| {
                if let Some(probe) = probes.get(name) {
                    return probe
                        .value(row, signals)
                        .map(|value| Some(ComplexValue::from(value)))
                        .map_err(crate::netlist::expr::ExprError::InvalidArgument);
                }
                let Some(parameter) = parameters.get(name) else {
                    return Ok(None);
                };
                if !parameter.is_axis_symbol
                    && let Some(value) = reads.read_measure(&parameter.canonical_measure)
                {
                    return Ok(Some(ComplexValue::from(value)));
                }
                if let Some(value) = lookup_equation_signal_canonical_optional(
                    signals,
                    name,
                    &parameter.canonical_signal,
                    row,
                )
                .map_err(crate::netlist::expr::ExprError::InvalidArgument)?
                {
                    return Ok(Some(ComplexValue::from(value)));
                }
                if parameter.is_axis_symbol
                    && let Some(value) = reads.read_measure(&parameter.canonical_measure)
                {
                    return Ok(Some(ComplexValue::from(value)));
                }
                Ok(parameter.context_value)
            })
            .map_err(|error| error.to_string())
    }
}

impl LiveMeasureOperand {
    fn compile(authored: &str, params: &crate::netlist::ParamContext) -> Result<Self, String> {
        let parsed_expression = authored
            .strip_prefix('{')
            .and_then(|value| value.strip_suffix('}'))
            .map(crate::netlist::expr::parse_expression)
            .transpose()
            .map_err(|error| format!("invalid measurement expression '{authored}': {error}"))?;
        let canonical_authored = authored.to_ascii_uppercase();
        let canonical_signal = canonical_measure_signal_name(authored);
        let is_axis_symbol = matches!(
            canonical_authored.as_str(),
            "TIME" | "FREQ" | "FREQUENCY" | "HERTZ"
        );
        let mut dependencies = vec![canonical_authored.clone()];
        let expression = if let Some(expression) = parsed_expression {
            let prepared = LivePreparedExpression::compile(&expression, params)?;
            dependencies.extend(
                prepared
                    .parameters
                    .values()
                    .map(|parameter| parameter.canonical_measure.clone()),
            );
            Some(Box::new(prepared))
        } else {
            None
        };
        if expression.is_some() {
            dependencies.retain(|name| name != &canonical_authored);
        }
        dependencies.sort();
        dependencies.dedup();
        Ok(Self {
            authored: authored.to_string(),
            canonical_authored,
            canonical_signal,
            is_axis_symbol,
            expression,
            dependencies,
        })
    }

    fn value(
        &mut self,
        row: usize,
        signals: &CanonicalMeasureSignalIndex<'_>,
        reads: &mut LiveMeasureReadContext<'_, '_>,
        params: &crate::netlist::ParamContext,
    ) -> Result<Value, String> {
        if !self.is_axis_symbol
            && let Some(value) = reads.read_measure(&self.canonical_authored)
        {
            return Ok(value);
        }
        if let Some(value) = lookup_equation_signal_canonical_optional(
            signals,
            &self.authored,
            &self.canonical_signal,
            row,
        )? {
            return Ok(value);
        }
        if self.is_axis_symbol
            && let Some(value) = reads.read_measure(&self.canonical_authored)
        {
            return Ok(value);
        }
        let Some(expression) = &mut self.expression else {
            return Err(format!("Signal '{}' not found", self.authored));
        };
        let value = expression.value(row, signals, reads, params)?;
        Ok(crate::netlist::expr::normalize_xyce_expression_component(
            value.re,
        ))
    }
}

enum LiveMeasureState {
    Equation {
        source: LiveEquationSource,
        dependencies: Vec<String>,
        from: Option<Value>,
        to: Option<Value>,
        td: Option<Value>,
    },
    Extremum {
        signal: LiveMeasureOperand,
        lower: Value,
        upper: Value,
        output: ExtremaOutput,
        is_max: bool,
        selected: Option<(Value, Value)>,
    },
    PeakToPeak {
        signal: LiveMeasureOperand,
        lower: Value,
        upper: Value,
        minimum: Value,
        maximum: Value,
        initialized: bool,
    },
    IntegralStatistic {
        signal: LiveMeasureOperand,
        lower: Value,
        upper: Value,
        mode: LiveIntegralMode,
        integral: Value,
        width: Value,
        previous: Option<(Value, Value)>,
    },
    ErrorFunction {
        measured: LiveMeasureOperand,
        comparison: LiveMeasureOperand,
        norm: ErrorFunctionNorm,
        lower: Value,
        upper: Value,
        minval: Value,
        ymin: Value,
        ymax: Value,
        sum: Value,
        count: usize,
    },
    Point {
        signal: Option<LiveMeasureOperand>,
        at: Option<Value>,
        condition: Option<Box<LiveCondition>>,
        lower: Value,
        upper: Value,
        minval: Value,
        kind: LivePointKind,
        previous_signal: Option<(Value, Value)>,
        negative_results: VecDeque<LivePointCandidate>,
        complete: bool,
    },
    Delay(Box<LiveDelayState>),
    RiseFall {
        signal: LiveMeasureOperand,
        samples: Vec<Value>,
        from_pct: Value,
        to_pct: Value,
        number: usize,
        is_rise: bool,
    },
    FileError {
        signal: LiveMeasureOperand,
        samples: Vec<Value>,
        frozen: bool,
    },
    Param {
        source: LiveEquationSource,
        dependencies: Vec<String>,
    },
}

enum LiveEquationSource {
    Expression(LivePreparedExpression),
    RawReference {
        authored: String,
        canonical_measure: String,
        canonical_signal: String,
    },
    RawOutputOperator(LiveRawOutputOperator),
}

#[derive(Debug)]
struct LiveRawOutputOperator {
    authored: String,
    canonical_signal: String,
    voltage: Option<LiveVoltageOutputOperator>,
    current: Option<LiveCurrentOutputOperator>,
}

#[derive(Debug)]
struct LiveVoltageOutputOperator {
    prefix: String,
    arguments: Vec<LiveVoltageNode>,
}

#[derive(Debug)]
struct LiveVoltageNode {
    authored: String,
    voltage: String,
    real: String,
    imaginary: String,
}

#[derive(Debug)]
struct LiveCurrentOutputOperator {
    prefix: String,
    authored: String,
    canonical_signal: String,
}

impl LiveRawOutputOperator {
    fn compile(authored: &str) -> Result<Self, String> {
        let (name, arguments) = split_equation_output_operator(authored)
            .ok_or_else(|| format!("invalid raw equation output operator '{authored}'"))?;
        let prefix = name.to_ascii_uppercase();
        let valid = matches!(prefix.as_str(), "N" | "P" | "W" | "DNO" | "DNI")
            || is_current_output_accessor(&prefix)
            || is_equation_voltage_accessor(&prefix)
            || is_equation_rf_accessor(&prefix);
        if !valid {
            return Err(format!(
                "unsupported raw equation output operator '{authored}'"
            ));
        }
        let voltage = if is_equation_voltage_accessor(&prefix) {
            if !(1..=2).contains(&arguments.len()) {
                return Err(format!(
                    "{prefix}() in continuous measure requires one or two arguments"
                ));
            }
            Some(LiveVoltageOutputOperator {
                prefix: prefix.clone(),
                arguments: arguments
                    .iter()
                    .cloned()
                    .map(|authored| LiveVoltageNode {
                        voltage: canonical_measure_signal_name(&format!("V({authored})")),
                        real: canonical_measure_signal_name(&format!("VR({authored})")),
                        imaginary: canonical_measure_signal_name(&format!("VI({authored})")),
                        authored,
                    })
                    .collect(),
            })
        } else {
            None
        };
        let current = if is_current_projection_accessor(&prefix) {
            if arguments.len() != 1 {
                return Err(format!(
                    "{prefix}() in continuous measure requires exactly one argument"
                ));
            }
            let authored = format!("I({})", arguments[0]);
            Some(LiveCurrentOutputOperator {
                prefix,
                canonical_signal: canonical_measure_signal_name(&authored),
                authored,
            })
        } else {
            None
        };
        Ok(Self {
            authored: authored.to_string(),
            canonical_signal: canonical_measure_signal_name(authored),
            voltage,
            current,
        })
    }

    fn value(
        &self,
        row: usize,
        signals: &CanonicalMeasureSignalIndex<'_>,
    ) -> Result<Value, String> {
        if let Some(value) = lookup_equation_signal_canonical_optional(
            signals,
            &self.authored,
            &self.canonical_signal,
            row,
        )? {
            return Ok(value);
        }
        if let Some(current) = &self.current
            && let Some(value) = lookup_equation_signal_canonical_optional(
                signals,
                &current.authored,
                &current.canonical_signal,
                row,
            )?
        {
            let magnitude = value.abs();
            return Ok(match current.prefix.as_str() {
                "I" | "IR" => value,
                "II" => 0.0,
                "IM" => magnitude,
                "IP" => 0.0_f64.atan2(value).to_degrees(),
                "IDB" => 20.0 * magnitude.log10(),
                _ => unreachable!(),
            });
        }
        let Some(voltage) = &self.voltage else {
            return Err(format!(
                "continuous measure signal '{}' is unavailable at row {row}",
                self.authored
            ));
        };
        let node_component = |component: &str, node: &LiveVoltageNode| -> Result<Value, String> {
            if node.authored == "0" {
                return Ok(0.0);
            }
            let canonical = match component {
                "V" => &node.voltage,
                "VR" => &node.real,
                "VI" => &node.imaginary,
                _ => unreachable!(),
            };
            if let Some(value) =
                lookup_equation_signal_canonical_optional(signals, &node.authored, canonical, row)?
            {
                return Ok(value);
            }
            if component == "VR"
                && let Some(value) = lookup_equation_signal_canonical_optional(
                    signals,
                    &node.authored,
                    &node.voltage,
                    row,
                )?
            {
                return Ok(value);
            }
            Err(format!(
                "continuous measure signal '{component}({})' is unavailable at row {row}",
                node.authored
            ))
        };
        if voltage.arguments.len() == 1 {
            return if voltage.prefix == "V" {
                node_component("V", &voltage.arguments[0])
            } else {
                Err(format!(
                    "continuous measure signal '{}' is unavailable at row {row}",
                    self.authored
                ))
            };
        }
        let positive = &voltage.arguments[0];
        let negative = &voltage.arguments[1];
        match voltage.prefix.as_str() {
            "V" | "VR" => Ok(node_component("VR", positive)? - node_component("VR", negative)?),
            "VI" => Ok(node_component("VI", positive)? - node_component("VI", negative)?),
            "VM" | "VP" | "VDB" => {
                let real = node_component("VR", positive)? - node_component("VR", negative)?;
                let imaginary = node_component("VI", positive)? - node_component("VI", negative)?;
                let magnitude = real.hypot(imaginary);
                Ok(match voltage.prefix.as_str() {
                    "VM" => magnitude,
                    "VP" => imaginary.atan2(real).to_degrees(),
                    "VDB" => 20.0 * magnitude.log10(),
                    _ => unreachable!(),
                })
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum LiveIntegralMode {
    Average,
    Rms,
    Integral { direction: Value },
}

#[derive(Debug, Clone, Copy)]
enum LivePointKind {
    When,
    Find,
    Derivative,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LivePointCandidate {
    Defined(Value),
    Undefined,
}

impl LivePointCandidate {
    fn numeric_value(&self) -> Value {
        match self {
            Self::Defined(value) => *value,
            Self::Undefined => Value::NAN,
        }
    }
}

struct LiveCondition {
    left: LiveMeasureOperand,
    right: LiveConditionOperand,
    edge: EdgeType,
    number: isize,
    minval: Value,
    match_count: usize,
    negative_events: VecDeque<LiveConditionEvent>,
    previous: Option<(Value, Value, Value)>,
}

#[derive(Debug, Default, Clone, Copy)]
struct LiveConditionUpdate {
    current: Option<LiveConditionEvent>,
    selected: Option<LiveConditionEvent>,
}

#[derive(Debug, Clone, Copy)]
struct LiveConditionEvent {
    fraction: Value,
    axis: Value,
    current_within_minval: bool,
}

enum LiveConditionOperand {
    Constant(Value),
    Waveform(LiveMeasureOperand),
}

struct LiveDelayClause {
    at: Option<Value>,
    condition: Option<LiveDelayCondition>,
    td: Option<Value>,
    from: Option<Value>,
    to: Option<Value>,
    selected: Option<Value>,
    negative_events: VecDeque<Value>,
    minval: Value,
    legacy: bool,
}

struct LiveDelayState {
    trigger: LiveDelayClause,
    target: LiveDelayClause,
    frac_tracker: Option<LegacyFracDelayTracker>,
    axis_ascending: bool,
    axis_minimum: Value,
    axis_maximum: Value,
}

struct LiveDelayCondition {
    left: LiveMeasureOperand,
    right: LiveConditionOperand,
    tracker: DelayConditionTracker,
    number: isize,
}

/// Evaluate Xyce continuous equation measurements over a transient result.
///
/// Equations run in netlist statement order at each point. Consequently a
/// reference to an earlier equation observes its value from the current point,
/// while a forward reference observes that equation's previous/default value.
pub fn evaluate_tran_equation_measurements(
    netlist: &Netlist,
    result: &TransientResult,
) -> Result<Vec<EquationMeasureTrace>, String> {
    let alias_projection =
        InterfaceNodeAliasProjection::new(netlist, OutputAnalysisKind::Tran, result.time.len())?;
    let mut signals = transient_signal_map(result);
    alias_projection.augment(&mut signals)?;
    evaluate_equation_measurements(netlist, "TRAN", &result.time, &signals, -1.0, None)
        .map(|traces| retain_equation_traces(netlist, "TRAN", traces))
}

/// Evaluate Xyce continuous equation measurements over a DC sweep.
///
/// The swept value is the equation axis. Xyce initializes DC equations to
/// zero when no explicit `DEFAULT_VAL` is present and accepts window bounds
/// in either order. As with transient equations, statements execute in
/// netlist order at each sweep point and retain their last in-window value.
pub fn evaluate_dc_equation_measurements(
    netlist: &Netlist,
    sweep: &[(Value, SimulationResult)],
) -> Result<Vec<EquationMeasureTrace>, String> {
    let Some(series) = DcSweepSeries::from_sweep(sweep).map_err(|error| error.to_string())? else {
        return Ok(Vec::new());
    };
    let alias_projection =
        InterfaceNodeAliasProjection::new(netlist, OutputAnalysisKind::Dc, series.axis().len())?;
    let mut signals = series.signal_map();
    alias_projection.augment(&mut signals)?;
    evaluate_equation_measurements(
        netlist,
        "DC",
        series.axis(),
        &signals,
        0.0,
        Some(dc_primary_sweep_is_ascending(netlist, series.axis())),
    )
    .map(|traces| retain_equation_traces(netlist, "DC", traces))
}

/// Physical type of one ordered scalar output column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputColumnKind {
    Voltage,
    Current,
    Scalar,
}

/// One fully evaluated source-authored output column.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct OutputColumn {
    name: String,
    kind: OutputColumnKind,
    values: Vec<Value>,
}

impl OutputColumn {
    /// Decompose an evaluated column so output projection can describe it
    /// without reparsing the authored spelling.
    pub(crate) fn into_parts(self) -> (String, OutputColumnKind, Vec<Value>) {
        (self.name, self.kind, self.values)
    }
}

/// Typed failure at the ordered output-projection boundary.
#[derive(Debug, thiserror::Error)]
pub(crate) enum OutputProjectionError {
    #[error("output projection aborted")]
    Aborted,
    #[error(transparent)]
    ResourceLimit(#[from] ResourceLimitError),
    #[error(
        "{analysis:?} output operand {operand_index} '{operand}' at {origin} failed{row_suffix}: {detail}",
        row_suffix = row.map(|row| format!(" at row {row}")).unwrap_or_default()
    )]
    Operand {
        analysis: OutputAnalysisKind,
        origin: NetlistSourceLocation,
        operand_index: usize,
        operand: String,
        row: Option<usize>,
        detail: String,
    },
}

/// Ordered output cards that select one analysis family.
///
/// `.PLOT` carries the same operand grammar and analysis qualifier as
/// `.PRINT`, so both are evaluated by this one resolver. Treating `.PLOT` as
/// a second, weaker path is exactly how an authored column goes missing when
/// a deck mixes the two cards.
fn matching_print_requests(netlist: &Netlist, analysis: OutputAnalysisKind) -> Vec<&OutputRequest> {
    netlist
        .output_requests
        .iter()
        .filter(|request| {
            matches!(
                request.directive,
                OutputDirectiveKind::Print | OutputDirectiveKind::Plot
            ) && request.analysis.is_none_or(|owned| owned == analysis)
        })
        .collect()
}

struct RealOutputProjectionPlan {
    column_count: usize,
    wildcard_voltage_nodes: Vec<WildcardVoltageNode>,
    expand_complete_voltage_wildcard: bool,
}

struct WildcardVoltageNode {
    lookup_name: String,
    display_name: String,
}

fn ordered_wildcard_voltage_nodes(
    names: &[String],
    voltage_count: usize,
    netlist: &Netlist,
    namespace: &OutputNodeNamespace,
) -> Result<Vec<WildcardVoltageNode>, String> {
    if names.len() != voltage_count {
        return Err(format!(
            "V(*) requires complete node-name metadata, but the result has {} name(s) for {voltage_count} voltage vector(s)",
            names.len()
        ));
    }

    let mut seen = HashSet::new();
    let mut nodes = Vec::with_capacity(names.len());
    for name in names {
        if name.trim().is_empty() {
            return Err("V(*) encountered an unnamed circuit node".to_string());
        }
        if netlist.ground_policy().is_ground(name) {
            continue;
        }
        let canonical = canonical_symbol(name);
        if !namespace.external.contains(&canonical) {
            continue;
        }
        if !seen.insert(canonical.clone()) {
            return Err(format!(
                "V(*) encountered duplicate case-insensitive circuit node name '{name}'"
            ));
        }
        nodes.push(WildcardVoltageNode {
            lookup_name: name.clone(),
            display_name: if namespace.authored_top_level.contains(&canonical) {
                canonical
            } else {
                canonical.replace('.', ":")
            },
        });
    }
    Ok(nodes)
}

fn preflight_real_output_requests(
    requests: &[&OutputRequest],
    analysis: OutputAnalysisKind,
    point_count: usize,
    node_metadata: Option<(&[String], usize)>,
    netlist: &Netlist,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<RealOutputProjectionPlan, OutputProjectionError> {
    if abort.is_aborted() {
        return Err(OutputProjectionError::Aborted);
    }
    // Xyce defines direct PRINT voltage wildcards. Ngspice's `all`/`allv`
    // selectors and Spectre SAVE globs are different languages and remain
    // owned by their existing frontend paths.
    let expand_complete_voltage_wildcard =
        netlist.params.expression_dialect() == crate::config::ExpressionDialect::Xyce;
    let mut wildcard_voltage_nodes = None;
    let mut column_count = 0usize;
    for request in requests {
        if request.operands.len() != request.operand_kinds.len() {
            return Err(output_request_error(
                request,
                analysis,
                request.operands.len().min(request.operand_kinds.len()),
                None,
                format!(
                    "authored operand count {} does not match typed operand count {}",
                    request.operands.len(),
                    request.operand_kinds.len()
                ),
            ));
        }
        for (operand_index, kind) in request.operand_kinds.iter().enumerate() {
            let OutputOperandKind::Probe(signal) = kind else {
                column_count = column_count.saturating_add(1);
                continue;
            };
            if expand_complete_voltage_wildcard
                && matches!(signal, SaveSignal::Voltage(node) if node == "*")
            {
                if wildcard_voltage_nodes.is_none() {
                    let Some((names, voltage_count)) = node_metadata else {
                        return Err(output_request_error(
                            request,
                            analysis,
                            operand_index,
                            None,
                            "V(*) requires circuit node metadata".to_string(),
                        ));
                    };
                    let namespace = match collect_output_node_namespace_with_limits_and_abort(
                        netlist, limits, abort,
                    ) {
                        Ok(namespace) => namespace,
                        Err(crate::netlist::ParseWithAbortError::Aborted) => {
                            return Err(OutputProjectionError::Aborted);
                        }
                        Err(crate::netlist::ParseWithAbortError::Parse(error)) => {
                            return Err(output_request_error(
                                request,
                                analysis,
                                operand_index,
                                None,
                                format!(
                                    "V(*) external-node namespace could not be elaborated: {error}"
                                ),
                            ));
                        }
                    };
                    wildcard_voltage_nodes = Some(
                        ordered_wildcard_voltage_nodes(names, voltage_count, netlist, &namespace)
                            .map_err(|detail| {
                            output_request_error(request, analysis, operand_index, None, detail)
                        })?,
                    );
                }
                column_count = column_count.saturating_add(
                    wildcard_voltage_nodes
                        .as_ref()
                        .expect("initialized above")
                        .len(),
                );
            } else {
                column_count = column_count.saturating_add(1);
            }
        }
    }
    let requested_values = point_count.saturating_mul(column_count.saturating_add(1));
    ResourceLimitError::ensure(
        ResourceKind::ResultValues,
        requested_values,
        limits.max_result_values,
    )?;
    Ok(RealOutputProjectionPlan {
        column_count,
        wildcard_voltage_nodes: wildcard_voltage_nodes.unwrap_or_default(),
        expand_complete_voltage_wildcard,
    })
}

/// Evaluate source-authored real `.PRINT TRAN` requests in exact card and
/// operand order with cancellation and result-allocation enforcement.
fn evaluate_tran_output_columns_with_abort(
    netlist: &Netlist,
    result: &TransientResult,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<OutputColumn>, OutputProjectionError> {
    let requests = matching_print_requests(netlist, OutputAnalysisKind::Tran);
    if requests.is_empty() {
        return Ok(Vec::new());
    }
    let projection = preflight_real_output_requests(
        &requests,
        OutputAnalysisKind::Tran,
        result.time.len(),
        Some((&result.node_names, result.voltages.len())),
        netlist,
        limits,
        abort,
    )?;
    let alias_projection = InterfaceNodeAliasProjection::new_with_abort(
        netlist,
        OutputAnalysisKind::Tran,
        result.time.len(),
        abort,
    )
    .map_err(|error| match error {
        InterfaceNodeAliasProjectionError::Aborted => OutputProjectionError::Aborted,
        InterfaceNodeAliasProjectionError::Detail(detail) => {
            output_request_error(requests[0], OutputAnalysisKind::Tran, 0, None, detail)
        }
    })?;
    let mut signals = transient_signal_map(result);
    alias_projection.augment(&mut signals).map_err(|detail| {
        output_request_error(requests[0], OutputAnalysisKind::Tran, 0, None, detail)
    })?;
    evaluate_real_output_requests(
        &requests,
        OutputAnalysisKind::Tran,
        &result.time,
        &signals,
        &netlist.params,
        &projection,
        abort,
    )
}

/// Evaluate ordered real `.PRINT TRAN` columns for frontend export.
///
/// Each tuple is `(authored_name, physical_type, values)`, where
/// `physical_type` is one of `voltage`, `current`, or `parameter`.
pub fn evaluate_tran_output_requests_with_abort(
    netlist: &Netlist,
    result: &TransientResult,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<(String, &'static str, Vec<Value>)>, SimulationError> {
    evaluate_tran_output_columns_with_abort(netlist, result, limits, abort)
        .map(frontend_output_columns)
        .map_err(frontend_output_error)
}

/// Resolve and evaluate every operand on one source-authored `.FOUR` card.
///
/// Fourier post-processing deliberately reuses the ordered transient output
/// resolver. Differential voltages, branch/device currents, hierarchy
/// aliases, expressions, allocation limits, and cancellation therefore have
/// exactly the same semantics as other transient output requests.
pub fn evaluate_tran_four_output_requests_with_abort(
    netlist: &Netlist,
    result: &TransientResult,
    four_index: usize,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<(String, &'static str, Vec<Value>)>, SimulationError> {
    let request = netlist
        .output_requests
        .iter()
        .filter(|request| request.directive == OutputDirectiveKind::Four)
        .nth(four_index)
        .ok_or_else(|| {
            SimulationError::Netlist(format!(
                ".FOUR request {} has no typed output request",
                four_index + 1
            ))
        })?;
    let requests = [request];
    let projection = preflight_real_output_requests(
        &requests,
        OutputAnalysisKind::Tran,
        result.time.len(),
        Some((&result.node_names, result.voltages.len())),
        netlist,
        limits,
        abort,
    )
    .map_err(frontend_output_error)?;
    if projection.column_count == 0 {
        return Err(SimulationError::Netlist(format!(
            ".FOUR request {} has no output operands",
            four_index + 1
        )));
    }
    let alias_projection = InterfaceNodeAliasProjection::new_with_abort(
        netlist,
        OutputAnalysisKind::Tran,
        result.time.len(),
        abort,
    )
    .map_err(|error| match error {
        InterfaceNodeAliasProjectionError::Aborted => SimulationError::Aborted,
        InterfaceNodeAliasProjectionError::Detail(detail) => SimulationError::Netlist(format!(
            ".FOUR request {} alias resolution failed: {detail}",
            four_index + 1
        )),
    })?;
    let mut signals = transient_signal_map(result);
    alias_projection.augment(&mut signals).map_err(|detail| {
        SimulationError::Netlist(format!(
            ".FOUR request {} alias resolution failed: {detail}",
            four_index + 1
        ))
    })?;
    evaluate_real_output_requests(
        &requests,
        OutputAnalysisKind::Tran,
        &result.time,
        &signals,
        &netlist.params,
        &projection,
        abort,
    )
    .map(frontend_output_columns)
    .map_err(frontend_output_error)
}

/// Resolve and evaluate one typed `.FFT` operand over the accepted transient
/// history. The source request is used rather than reparsing the probe so
/// hierarchy aliases, lead-current accessors, expressions, user functions,
/// cancellation, and allocation limits exactly match ordered output cards.
pub(crate) fn evaluate_tran_fft_output_with_abort(
    netlist: &Netlist,
    result: &TransientResult,
    fft_index: usize,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<(String, &'static str, Vec<Value>), SimulationError> {
    let request = netlist
        .output_requests
        .iter()
        .filter(|request| request.directive == OutputDirectiveKind::Fft)
        .nth(fft_index)
        .ok_or_else(|| {
            SimulationError::Netlist(format!(
                ".FFT request {} has no typed output operand",
                fft_index + 1
            ))
        })?;
    let requests = [request];
    let projection = preflight_real_output_requests(
        &requests,
        OutputAnalysisKind::Tran,
        result.time.len(),
        Some((&result.node_names, result.voltages.len())),
        netlist,
        limits,
        abort,
    )
    .map_err(frontend_output_error)?;
    if projection.column_count != 1 {
        return Err(SimulationError::Netlist(format!(
            ".FFT request {} resolved to {} output columns; exactly one is required",
            fft_index + 1,
            projection.column_count
        )));
    }
    let alias_projection = InterfaceNodeAliasProjection::new_with_abort(
        netlist,
        OutputAnalysisKind::Tran,
        result.time.len(),
        abort,
    )
    .map_err(|error| match error {
        InterfaceNodeAliasProjectionError::Aborted => SimulationError::Aborted,
        InterfaceNodeAliasProjectionError::Detail(detail) => SimulationError::Netlist(format!(
            ".FFT request {} alias resolution failed: {detail}",
            fft_index + 1
        )),
    })?;
    let mut signals = transient_signal_map(result);
    alias_projection.augment(&mut signals).map_err(|detail| {
        SimulationError::Netlist(format!(
            ".FFT request {} alias resolution failed: {detail}",
            fft_index + 1
        ))
    })?;
    let mut columns = evaluate_real_output_requests(
        &requests,
        OutputAnalysisKind::Tran,
        &result.time,
        &signals,
        &netlist.params,
        &projection,
        abort,
    )
    .map_err(frontend_output_error)?;
    let column = columns.pop().ok_or_else(|| {
        SimulationError::Netlist(format!(
            ".FFT request {} produced no output column",
            fft_index + 1
        ))
    })?;
    let physical_type = match column.kind {
        OutputColumnKind::Voltage => "voltage",
        OutputColumnKind::Current => "current",
        OutputColumnKind::Scalar => "parameter",
    };
    Ok((column.name, physical_type, column.values))
}

#[cfg(test)]
pub(crate) fn evaluate_tran_output_requests(
    netlist: &Netlist,
    result: &TransientResult,
) -> Result<Vec<OutputColumn>, OutputProjectionError> {
    evaluate_tran_output_columns_with_abort(netlist, result, ResourceLimits::default(), &NoAbort)
}

/// Evaluate source-authored real `.PRINT DC` requests in exact card and
/// operand order. DC columns are rebuilt by canonical name at every point;
/// result-shape changes can therefore never be silently zero-filled.
fn evaluate_dc_output_columns_with_abort(
    netlist: &Netlist,
    sweep: &[(Value, SimulationResult)],
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<OutputColumn>, OutputProjectionError> {
    let requests = matching_print_requests(netlist, OutputAnalysisKind::Dc);
    if requests.is_empty() {
        return Ok(Vec::new());
    }
    let node_metadata = sweep
        .first()
        .map(|(_, result)| (result.node_names.as_slice(), result.node_voltages.len()));
    let projection = preflight_real_output_requests(
        &requests,
        OutputAnalysisKind::Dc,
        sweep.len(),
        node_metadata,
        netlist,
        limits,
        abort,
    )?;
    let series =
        DcOutputSeries::from_sweep_with_abort(sweep, limits, abort).map_err(
            |error| match error {
                DcOutputSeriesBuildError::Aborted => OutputProjectionError::Aborted,
                DcOutputSeriesBuildError::ResourceLimit(error) => {
                    OutputProjectionError::ResourceLimit(error)
                }
                DcOutputSeriesBuildError::Detail(detail) => {
                    output_request_error(requests[0], OutputAnalysisKind::Dc, 0, None, detail)
                }
            },
        )?;
    let alias_projection = InterfaceNodeAliasProjection::new_with_abort(
        netlist,
        OutputAnalysisKind::Dc,
        series.axis.len(),
        abort,
    )
    .map_err(|error| match error {
        InterfaceNodeAliasProjectionError::Aborted => OutputProjectionError::Aborted,
        InterfaceNodeAliasProjectionError::Detail(detail) => {
            output_request_error(requests[0], OutputAnalysisKind::Dc, 0, None, detail)
        }
    })?;
    let mut signals = series.signal_map();
    alias_projection.augment(&mut signals).map_err(|detail| {
        output_request_error(requests[0], OutputAnalysisKind::Dc, 0, None, detail)
    })?;
    evaluate_real_output_requests(
        &requests,
        OutputAnalysisKind::Dc,
        &series.axis,
        &signals,
        &netlist.params,
        &projection,
        abort,
    )
}

/// Evaluate ordered real `.PRINT DC` columns for frontend export.
///
/// Each tuple is `(authored_name, physical_type, values)`, where DC rows are
/// resolved by canonical signal name rather than result-vector position.
pub fn evaluate_dc_output_requests_with_abort(
    netlist: &Netlist,
    sweep: &[(Value, SimulationResult)],
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<(String, &'static str, Vec<Value>)>, SimulationError> {
    evaluate_dc_output_columns_with_abort(netlist, sweep, limits, abort)
        .map(frontend_output_columns)
        .map_err(frontend_output_error)
}

#[cfg(test)]
pub(crate) fn evaluate_dc_output_requests(
    netlist: &Netlist,
    sweep: &[(Value, SimulationResult)],
) -> Result<Vec<OutputColumn>, OutputProjectionError> {
    evaluate_dc_output_columns_with_abort(netlist, sweep, ResourceLimits::default(), &NoAbort)
}

fn frontend_output_columns(columns: Vec<OutputColumn>) -> Vec<(String, &'static str, Vec<Value>)> {
    columns
        .into_iter()
        .map(|column| {
            let physical_type = match column.kind {
                OutputColumnKind::Voltage => "voltage",
                OutputColumnKind::Current => "current",
                OutputColumnKind::Scalar => "parameter",
            };
            (column.name, physical_type, column.values)
        })
        .collect()
}

fn frontend_output_error(error: OutputProjectionError) -> SimulationError {
    match error {
        OutputProjectionError::Aborted => SimulationError::Aborted,
        OutputProjectionError::ResourceLimit(error) => SimulationError::ResourceLimit(error),
        error @ OutputProjectionError::Operand { .. } => {
            SimulationError::Netlist(error.to_string())
        }
    }
}

fn output_request_error(
    request: &OutputRequest,
    analysis: OutputAnalysisKind,
    operand_index: usize,
    row: Option<usize>,
    detail: String,
) -> OutputProjectionError {
    OutputProjectionError::Operand {
        analysis,
        origin: request.origin.clone(),
        operand_index,
        operand: request
            .operands
            .get(operand_index)
            .cloned()
            .unwrap_or_default(),
        row,
        detail,
    }
}

fn output_column_kind(signal: &SaveSignal) -> OutputColumnKind {
    match signal {
        SaveSignal::Voltage(_) | SaveSignal::VoltageDiff(_, _) => OutputColumnKind::Voltage,
        SaveSignal::Current(_) => OutputColumnKind::Current,
        SaveSignal::DeviceParam { .. } => OutputColumnKind::Scalar,
        SaveSignal::Raw(name) if name.contains(':') || name.starts_with('@') => {
            OutputColumnKind::Scalar
        }
        SaveSignal::Raw(_) => OutputColumnKind::Voltage,
        SaveSignal::All => OutputColumnKind::Scalar,
    }
}

fn expression_output_column_kind(expression: &NetExpr) -> OutputColumnKind {
    let NetExpr::FnCall { name, .. } = expression else {
        return OutputColumnKind::Scalar;
    };
    let operator = name.to_ascii_uppercase();
    if operator != "IF" && is_current_output_accessor(&operator) {
        OutputColumnKind::Current
    } else if matches!(operator.as_str(), "V" | "VR" | "VI" | "VM" | "VP" | "VDB") {
        OutputColumnKind::Voltage
    } else {
        OutputColumnKind::Scalar
    }
}

fn evaluate_real_output_requests(
    requests: &[&OutputRequest],
    analysis: OutputAnalysisKind,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    params: &crate::netlist::ParamContext,
    projection: &RealOutputProjectionPlan,
    abort: &dyn AbortSignal,
) -> Result<Vec<OutputColumn>, OutputProjectionError> {
    if abort.is_aborted() {
        return Err(OutputProjectionError::Aborted);
    }

    let signal_index = CanonicalMeasureSignalIndex::new(signals);
    let mut columns = Vec::with_capacity(projection.column_count);
    for request in requests {
        for (operand_index, (authored, kind)) in request
            .operands
            .iter()
            .zip(&request.operand_kinds)
            .enumerate()
        {
            if abort.is_aborted() {
                return Err(OutputProjectionError::Aborted);
            }
            if projection.expand_complete_voltage_wildcard
                && matches!(kind, OutputOperandKind::Probe(SaveSignal::Voltage(node)) if node == "*")
            {
                for node in &projection.wildcard_voltage_nodes {
                    if abort.is_aborted() {
                        return Err(OutputProjectionError::Aborted);
                    }
                    let expanded_name = format!("V({})", node.display_name);
                    let expanded_kind =
                        OutputOperandKind::Probe(SaveSignal::Voltage(node.lookup_name.clone()));
                    let evaluated = evaluate_output_operand(
                        &expanded_name,
                        &expanded_kind,
                        axis,
                        &signal_index,
                        params,
                        abort,
                    )
                    .map_err(|error| match error {
                        OutputOperandEvaluationError::Aborted => OutputProjectionError::Aborted,
                        OutputOperandEvaluationError::Detail { row, detail } => {
                            output_request_error(request, analysis, operand_index, row, detail)
                        }
                    })?;
                    columns.push(evaluated);
                }
                continue;
            }
            let evaluated =
                evaluate_output_operand(authored, kind, axis, &signal_index, params, abort)
                    .map_err(|error| match error {
                        OutputOperandEvaluationError::Aborted => OutputProjectionError::Aborted,
                        OutputOperandEvaluationError::Detail { row, detail } => {
                            output_request_error(request, analysis, operand_index, row, detail)
                        }
                    })?;
            columns.push(evaluated);
        }
    }
    if abort.is_aborted() {
        return Err(OutputProjectionError::Aborted);
    }
    Ok(columns)
}

pub(crate) enum OutputOperandEvaluationError {
    Aborted,
    Detail { row: Option<usize>, detail: String },
}

impl From<(Option<usize>, String)> for OutputOperandEvaluationError {
    fn from((row, detail): (Option<usize>, String)) -> Self {
        Self::Detail { row, detail }
    }
}

pub(crate) fn evaluate_output_operand(
    authored: &str,
    kind: &OutputOperandKind,
    axis: &[Value],
    signal_index: &CanonicalMeasureSignalIndex<'_>,
    params: &crate::netlist::ParamContext,
    abort: &dyn AbortSignal,
) -> Result<OutputColumn, OutputOperandEvaluationError> {
    match kind {
        OutputOperandKind::Probe(signal) => {
            if matches!(signal, SaveSignal::All) {
                return Err((
                    None,
                    "the ALL selector cannot be represented as one scalar output column"
                        .to_string(),
                )
                    .into());
            }
            let direct = direct_output_values(authored, signal, signal_index, axis.len(), abort)?;
            let values = match direct {
                DirectOutputValues::Borrowed(direct) => {
                    let mut values = Vec::with_capacity(axis.len());
                    for (row, value) in direct.iter().copied().enumerate() {
                        if row.is_multiple_of(64) && abort.is_aborted() {
                            return Err(OutputOperandEvaluationError::Aborted);
                        }
                        values.push(value);
                    }
                    values
                }
                DirectOutputValues::Owned(values) => values,
            };
            if abort.is_aborted() {
                return Err(OutputOperandEvaluationError::Aborted);
            }
            Ok(OutputColumn {
                name: authored.to_string(),
                kind: output_column_kind(signal),
                values,
            })
        }
        OutputOperandKind::Expression { body } => {
            if abort.is_aborted() {
                return Err(OutputOperandEvaluationError::Aborted);
            }
            let protected_identifiers = params
                .all_params()
                .into_iter()
                .map(|(name, _)| name.to_ascii_uppercase())
                .collect::<HashSet<_>>();
            if abort.is_aborted() {
                return Err(OutputOperandEvaluationError::Aborted);
            }
            let expanded = crate::netlist::expr::expand_output_user_functions_with_abort(
                body,
                params,
                &protected_identifiers,
                abort,
            )
            .map_err(|error| match error {
                crate::netlist::expr::BehavioralPreparationError::Aborted => {
                    OutputOperandEvaluationError::Aborted
                }
                crate::netlist::expr::BehavioralPreparationError::Semantic(detail) => {
                    (None, detail).into()
                }
            })?;
            let parsed = crate::netlist::expr::parse_expression_with_abort(&expanded, abort)
                .map_err(|error| match error {
                    crate::netlist::expr::ParseExpressionWithAbortError::Aborted => {
                        OutputOperandEvaluationError::Aborted
                    }
                    crate::netlist::expr::ParseExpressionWithAbortError::Parse(error) => {
                        (None, format!("failed to parse expression: {error}")).into()
                    }
                })?;
            let column_kind = expression_output_column_kind(&parsed);
            let mut prepared = LivePreparedExpression::compile_with_abort(&parsed, params, abort)
                .map_err(|error| match error {
                LivePreparedExpressionCompileError::Aborted => {
                    OutputOperandEvaluationError::Aborted
                }
                LivePreparedExpressionCompileError::Detail(detail) => (None, detail).into(),
            })?;
            let mut values = Vec::with_capacity(axis.len());
            let mut programs = Vec::new();
            let mut current_values = HashMap::new();
            let program_indices = HashMap::new();
            for row in 0..axis.len() {
                if row.is_multiple_of(64) && abort.is_aborted() {
                    return Err(OutputOperandEvaluationError::Aborted);
                }
                let mut reads = LiveMeasureReadContext {
                    programs: &mut programs,
                    current_values: &mut current_values,
                    program_indices: &program_indices,
                    row,
                    axis,
                };
                let value = prepared
                    .value(row, signal_index, &mut reads, params)
                    .map_err(|detail| (Some(row), detail))?;
                values.push(crate::netlist::expr::normalize_xyce_expression_component(
                    value.re,
                ));
            }
            Ok(OutputColumn {
                name: authored.to_string(),
                kind: column_kind,
                values,
            })
        }
    }
}

enum DirectOutputValues<'a> {
    Borrowed(&'a [Value]),
    Owned(Vec<Value>),
}

fn direct_output_values<'a>(
    authored: &str,
    signal: &SaveSignal,
    signals: &CanonicalMeasureSignalIndex<'a>,
    point_count: usize,
    abort: &dyn AbortSignal,
) -> Result<DirectOutputValues<'a>, OutputOperandEvaluationError> {
    let lookup = |name: &str| {
        signals
            .get(name)
            .map_err(|detail| (None, detail))
            .and_then(|values| {
                values
                    .map(|values| {
                        if values.len() != point_count {
                            Err((
                                None,
                                format!(
                                    "signal '{name}' has {} value(s), expected {point_count}",
                                    values.len()
                                ),
                            ))
                        } else {
                            Ok(DirectOutputValues::Borrowed(values))
                        }
                    })
                    .transpose()
            })
    };
    if let Some(values) = lookup(authored)? {
        return Ok(values);
    }
    if let SaveSignal::DeviceParam { device, param } = signal {
        for candidate in [
            format!("@{device}[{param}]"),
            format!("{device}:{param}"),
            format!("N({device}:{param})"),
        ] {
            if let Some(values) = lookup(&candidate)? {
                return Ok(values);
            }
        }
    }
    if let SaveSignal::Raw(raw) = signal {
        if let Some(values) = lookup(raw)? {
            return Ok(values);
        }
        let voltage = format!("V({raw})");
        if let Some(values) = lookup(&voltage)? {
            return Ok(values);
        }
    }
    if matches!(
        signal,
        SaveSignal::Voltage(_) | SaveSignal::VoltageDiff(_, _) | SaveSignal::Current(_)
    ) {
        let canonical_probe = match signal {
            SaveSignal::Voltage(node) => format!("V({node})"),
            SaveSignal::VoltageDiff(positive, negative) => {
                format!("V({positive},{negative})")
            }
            SaveSignal::Current(device) => format!("I({device})"),
            _ => unreachable!(),
        };
        let operator =
            LiveRawOutputOperator::compile(&canonical_probe).map_err(|detail| (None, detail))?;
        let mut values = Vec::with_capacity(point_count);
        for row in 0..point_count {
            if row.is_multiple_of(64) && abort.is_aborted() {
                return Err(OutputOperandEvaluationError::Aborted);
            }
            values.push(
                operator
                    .value(row, signals)
                    .map_err(|detail| (Some(row), detail))?,
            );
        }
        return Ok(DirectOutputValues::Owned(values));
    }
    Err((None, format!("signal '{authored}' is unavailable")).into())
}

struct DcOutputSeries {
    axis: Vec<Value>,
    columns: Vec<(String, Vec<Value>)>,
}

enum DcOutputSeriesBuildError {
    Aborted,
    ResourceLimit(ResourceLimitError),
    Detail(String),
}

impl From<ResourceLimitError> for DcOutputSeriesBuildError {
    fn from(error: ResourceLimitError) -> Self {
        Self::ResourceLimit(error)
    }
}

impl DcOutputSeries {
    fn from_sweep_with_abort(
        sweep: &[(Value, SimulationResult)],
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, DcOutputSeriesBuildError> {
        if abort.is_aborted() {
            return Err(DcOutputSeriesBuildError::Aborted);
        }
        if sweep.is_empty() {
            return Err(DcOutputSeriesBuildError::Detail(
                "DC output projection requires at least one sweep point".to_string(),
            ));
        }
        struct Slot {
            name: String,
            values: Vec<Option<Value>>,
        }
        let point_count = sweep.len();
        let mut slots = HashMap::<String, Slot>::new();
        for (row, (_, result)) in sweep.iter().enumerate() {
            if row.is_multiple_of(64) && abort.is_aborted() {
                return Err(DcOutputSeriesBuildError::Aborted);
            }
            if result.node_names.len() != result.node_voltages.len() {
                return Err(DcOutputSeriesBuildError::Detail(format!(
                    "DC row {row} has {} node name(s) and {} voltage value(s)",
                    result.node_names.len(),
                    result.node_voltages.len()
                )));
            }
            if result.branch_names.len() != result.branch_currents.len() {
                return Err(DcOutputSeriesBuildError::Detail(format!(
                    "DC row {row} has {} branch name(s) and {} current value(s)",
                    result.branch_names.len(),
                    result.branch_currents.len()
                )));
            }
            let mut row_values = Vec::<(String, Value)>::new();
            for (item_index, (name, value)) in result
                .node_names
                .iter()
                .zip(&result.node_voltages)
                .enumerate()
            {
                if item_index.is_multiple_of(64) && abort.is_aborted() {
                    return Err(DcOutputSeriesBuildError::Aborted);
                }
                if !name.is_empty() {
                    row_values.push((format!("V({name})"), *value));
                    row_values.push((name.clone(), *value));
                }
            }
            for (item_index, (name, value)) in result
                .branch_names
                .iter()
                .zip(&result.branch_currents)
                .enumerate()
            {
                if item_index.is_multiple_of(64) && abort.is_aborted() {
                    return Err(DcOutputSeriesBuildError::Aborted);
                }
                if !name.is_empty() {
                    row_values.push((format!("I({name})"), *value));
                }
            }
            for (item_index, observable) in result.dc_observables.iter().enumerate() {
                if item_index.is_multiple_of(64) && abort.is_aborted() {
                    return Err(DcOutputSeriesBuildError::Aborted);
                }
                row_values.push(observable.clone());
            }
            for (item_index, (name, value)) in row_values.into_iter().enumerate() {
                if item_index.is_multiple_of(64) && abort.is_aborted() {
                    return Err(DcOutputSeriesBuildError::Aborted);
                }
                let canonical = canonical_measure_signal_name(&name);
                if !slots.contains_key(&canonical) {
                    let retained_values = point_count.saturating_mul(slots.len().saturating_add(2));
                    ResourceLimitError::ensure(
                        ResourceKind::ResultValues,
                        retained_values,
                        limits.max_result_values,
                    )?;
                }
                let slot = slots.entry(canonical.clone()).or_insert_with(|| Slot {
                    name: name.clone(),
                    values: vec![None; point_count],
                });
                if let Some(existing) = slot.values[row]
                    && existing.to_bits() != value.to_bits()
                {
                    return Err(DcOutputSeriesBuildError::Detail(format!(
                        "DC row {row} has ambiguous signal '{name}' normalized as '{canonical}'"
                    )));
                }
                slot.values[row] = Some(value);
            }
        }
        if abort.is_aborted() {
            return Err(DcOutputSeriesBuildError::Aborted);
        }
        let mut columns = Vec::with_capacity(slots.len());
        for (slot_index, slot) in slots.into_values().enumerate() {
            if slot_index.is_multiple_of(64) && abort.is_aborted() {
                return Err(DcOutputSeriesBuildError::Aborted);
            }
            let mut values = Vec::with_capacity(point_count);
            let mut complete = true;
            for (row, value) in slot.values.into_iter().enumerate() {
                if row.is_multiple_of(64) && abort.is_aborted() {
                    return Err(DcOutputSeriesBuildError::Aborted);
                }
                let Some(value) = value else {
                    complete = false;
                    break;
                };
                values.push(value);
            }
            if complete {
                columns.push((slot.name, values));
            }
        }
        let mut axis = Vec::with_capacity(point_count);
        for (row, (value, _)) in sweep.iter().enumerate() {
            if row.is_multiple_of(64) && abort.is_aborted() {
                return Err(DcOutputSeriesBuildError::Aborted);
            }
            axis.push(*value);
        }
        Ok(Self { axis, columns })
    }

    fn signal_map(&self) -> HashMap<String, &[Value]> {
        let mut signals = HashMap::new();
        insert_case_variants(&mut signals, "Time", &self.axis);
        for (name, values) in &self.columns {
            insert_case_variants(&mut signals, name, values);
        }
        signals
    }
}

/// Evaluate Xyce continuous equation measurements over an AC sweep.
///
/// AC equation probes follow Xyce's scalar accessor semantics: bare `V()`
/// and `I()` project the real component, while `VM`/`IM`, `VR`/`IR`,
/// `VI`/`II`, `VP`/`IP`, and `VDB`/`IDB` select magnitude, real part,
/// imaginary part, phase in degrees, and decibels respectively. `FREQ` and
/// `HERTZ` both denote the current sweep frequency.
pub fn evaluate_ac_equation_measurements(
    netlist: &Netlist,
    sweep: &[AcResult],
) -> Result<Vec<EquationMeasureTrace>, String> {
    let Some(series) = AcSweepSeries::from_sweep(sweep).map_err(|error| error.to_string())? else {
        return Ok(Vec::new());
    };
    let alias_projection =
        InterfaceNodeAliasProjection::new(netlist, OutputAnalysisKind::Ac, series.axis().len())?;
    let mut signals = series.equation_signal_map();
    alias_projection.augment(&mut signals)?;
    evaluate_equation_measurements(netlist, "AC", series.axis(), &signals, -1.0, None)
        .map(|traces| retain_equation_traces(netlist, "AC", traces))
}

/// Evaluate Xyce continuous equation measurements over a NOISE sweep.
///
/// NOISE uses the same complex probe projections as AC. Bare `V()` and `I()`
/// accessors select the real component in equations, while the explicitly
/// qualified accessors select magnitude, real, imaginary, phase, or decibels.
/// `FREQ` and `HERTZ` both denote the current sweep frequency. As in Xyce,
/// equations start at -1 unless a local or global `DEFAULT_VAL` overrides it.
pub fn evaluate_noise_equation_measurements(
    netlist: &Netlist,
    sweep: &[crate::analysis::NoiseResult],
) -> Result<Vec<EquationMeasureTrace>, String> {
    let Some(series) = NoiseSweepSeries::from_sweep(sweep).map_err(|error| error.to_string())?
    else {
        return Ok(Vec::new());
    };
    let alias_projection =
        InterfaceNodeAliasProjection::new(netlist, OutputAnalysisKind::Noise, series.axis().len())?;
    let mut signals = series.equation_signal_map();
    alias_projection.augment(&mut signals)?;
    evaluate_equation_measurements(netlist, "NOISE", series.axis(), &signals, -1.0, None)
        .map(|traces| retain_equation_traces(netlist, "NOISE", traces))
}

fn retain_equation_traces(
    netlist: &Netlist,
    analysis: &str,
    traces: Vec<EquationMeasureTrace>,
) -> Vec<EquationMeasureTrace> {
    traces
        .into_iter()
        .filter(|trace| {
            netlist.measurements.iter().any(|statement| {
                statement.analysis.eq_ignore_ascii_case(analysis)
                    && statement.name.eq_ignore_ascii_case(&trace.name)
                    && matches!(statement.measure_type, MeasureType::Equation { .. })
            })
        })
        .collect()
}

fn dc_primary_sweep_is_ascending(netlist: &Netlist, axis: &[Value]) -> bool {
    netlist
        .analyses
        .iter()
        .find_map(|analysis| {
            let crate::netlist::AnalysisCommand::Dc {
                start,
                stop,
                step,
                mode,
                ..
            } = analysis
            else {
                return None;
            };
            Some(match mode {
                crate::netlist::DcSweepMode::Linear if *step != 0.0 => *step > 0.0,
                crate::netlist::DcSweepMode::List(values) => values
                    .windows(2)
                    .find_map(|pair| (pair[0] != pair[1]).then_some(pair[1] > pair[0]))
                    .unwrap_or(true),
                crate::netlist::DcSweepMode::Linear
                | crate::netlist::DcSweepMode::Decade { .. }
                | crate::netlist::DcSweepMode::Octave { .. } => *stop >= *start,
            })
        })
        .or_else(|| {
            axis.windows(2)
                .find_map(|pair| (pair[0] != pair[1]).then_some(pair[1] > pair[0]))
        })
        .unwrap_or(true)
}

/// Shared continuous-equation evaluator for analyses with a real-valued
/// axis and real signal waveforms.
fn evaluate_equation_measurements(
    netlist: &Netlist,
    analysis: &str,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    implicit_default: Value,
    dc_sweep_ascending: Option<bool>,
) -> Result<Vec<EquationMeasureTrace>, String> {
    match evaluate_equation_measurements_with_abort(
        netlist,
        analysis,
        axis,
        signals,
        implicit_default,
        dc_sweep_ascending,
        &NoAbort,
    ) {
        Ok(traces) => Ok(traces),
        Err(EquationMeasurementEvaluationError::Detail(detail)) => Err(detail),
        Err(EquationMeasurementEvaluationError::Aborted) => {
            unreachable!("NoAbort cannot cancel equation measurement evaluation")
        }
    }
}

enum EquationMeasurementEvaluationError {
    Aborted,
    Detail(String),
}

fn evaluate_equation_measurements_with_abort(
    netlist: &Netlist,
    analysis: &str,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    implicit_default: Value,
    dc_sweep_ascending: Option<bool>,
    abort: &dyn AbortSignal,
) -> Result<Vec<EquationMeasureTrace>, EquationMeasurementEvaluationError> {
    if abort.is_aborted() {
        return Err(EquationMeasurementEvaluationError::Aborted);
    }
    let mut programs = netlist
        .measurements
        .iter()
        .filter(|statement| statement.analysis.eq_ignore_ascii_case(analysis))
        .map(|statement| {
            let equation = matches!(statement.measure_type, MeasureType::Equation { .. });
            Ok::<_, String>(LiveMeasureProgram {
                statement,
                canonical_name: statement.name.to_ascii_uppercase(),
                state: Some(compile_live_measure_state(
                    statement,
                    analysis,
                    axis,
                    dc_sweep_ascending,
                    netlist.options.measure_use_lttm(),
                    &netlist.params,
                )?),
                current: netlist
                    .options
                    .measure_default_value
                    .or(statement.default_value)
                    .unwrap_or(if equation { implicit_default } else { 0.0 }),
                initialized: false,
                values: Vec::new(),
                valid: Vec::new(),
                store_trace: false,
                has_file_error_dependency: false,
                failure: None,
            })
        })
        .collect::<Result<Vec<_>, String>>()
        .map_err(EquationMeasurementEvaluationError::Detail)?;

    if programs.is_empty() {
        return Ok(Vec::new());
    }
    let all_dependencies = programs
        .iter()
        .map(|program| {
            program
                .state
                .as_ref()
                .expect("compiled live measure state")
                .all_dependencies()
        })
        .collect::<Vec<_>>();
    let referenced_measure_names = all_dependencies
        .iter()
        .flatten()
        .cloned()
        .collect::<HashSet<_>>();
    for (index, program) in programs.iter_mut().enumerate() {
        if index.is_multiple_of(64) && abort.is_aborted() {
            return Err(EquationMeasurementEvaluationError::Aborted);
        }
        program.store_trace =
            matches!(program.statement.measure_type, MeasureType::Equation { .. })
                || referenced_measure_names.contains(&program.canonical_name);
        if program.store_trace {
            program.values.reserve(axis.len());
            program.valid.reserve(axis.len());
        }
    }
    let mut current_values = programs
        .iter()
        .map(|program| (program.canonical_name.clone(), program.current))
        .collect::<HashMap<_, _>>();
    let program_indices = programs
        .iter()
        .enumerate()
        .map(|(index, program)| (program.canonical_name.clone(), index))
        .collect::<HashMap<_, _>>();
    let file_error_programs = programs
        .iter()
        .map(|program| matches!(program.state, Some(LiveMeasureState::FileError { .. })))
        .collect::<Vec<_>>();
    for (index, (program, dependencies)) in programs.iter_mut().zip(&all_dependencies).enumerate() {
        if index.is_multiple_of(64) && abort.is_aborted() {
            return Err(EquationMeasurementEvaluationError::Aborted);
        }
        program.has_file_error_dependency = dependencies.iter().any(|dependency| {
            program_indices
                .get(dependency)
                .is_some_and(|&dependency_index| file_error_programs[dependency_index])
        });
    }
    let signal_index = CanonicalMeasureSignalIndex::new(signals);

    let segment_starts = if analysis.eq_ignore_ascii_case("DC") {
        dc_primary_segment_starts(netlist, axis.len())
    } else {
        Vec::new()
    };
    for (row, &axis_value) in axis.iter().enumerate() {
        if row.is_multiple_of(64) && abort.is_aborted() {
            return Err(EquationMeasurementEvaluationError::Aborted);
        }
        let starts_segment = segment_starts.binary_search(&row).is_ok();
        for program_index in 0..programs.len() {
            if !programs[program_index].store_trace
                && !programs[program_index].has_file_error_dependency
            {
                continue;
            }
            let has_failed = programs[program_index].failure.is_some();
            let is_equation = matches!(
                programs[program_index].statement.measure_type,
                MeasureType::Equation { .. }
            );
            let mut state = programs[program_index]
                .state
                .take()
                .expect("live measure state is present outside evaluation");
            let update = if has_failed {
                Ok(None)
            } else {
                let mut reads = LiveMeasureReadContext {
                    programs: &mut programs,
                    current_values: &mut current_values,
                    program_indices: &program_indices,
                    row,
                    axis,
                };
                state.update(
                    row,
                    axis_value,
                    axis,
                    starts_segment,
                    &signal_index,
                    &mut reads,
                    &netlist.params,
                    dc_sweep_ascending,
                )
            };
            programs[program_index].state = Some(state);
            let program = &mut programs[program_index];
            match update {
                Ok(Some(value)) => {
                    program.current = value;
                    program.initialized = true;
                }
                Ok(None) => {}
                Err(error) if is_equation => {
                    return Err(EquationMeasurementEvaluationError::Detail(format!(
                        "continuous measure '{}' evaluation failed at row {row}: {error}",
                        program.statement.name
                    )));
                }
                Err(error) => program.failure = Some(error),
            }
            *current_values
                .get_mut(&program.canonical_name)
                .expect("compiled measure value slot") = program.current;
            if program.store_trace {
                program.values.push(program.current);
                program.valid.push(!program.current.is_nan());
            }
        }
    }

    if abort.is_aborted() {
        return Err(EquationMeasurementEvaluationError::Aborted);
    }
    Ok(programs
        .into_iter()
        .filter(|program| program.store_trace)
        .map(|program| EquationMeasureTrace {
            name: program.statement.name.clone(),
            values: program.values,
            valid: program.valid,
            initialized: program.initialized,
        })
        .collect())
}

fn compile_live_equation_source(
    expression: &crate::netlist::measure::MeasureExpression,
    statement_name: &str,
    params: &crate::netlist::ParamContext,
) -> Result<(LiveEquationSource, Vec<String>), String> {
    let mut dependencies = Vec::new();
    let source = match expression.kind {
        crate::netlist::measure::MeasureExpressionKind::Expression => {
            let parsed =
                crate::netlist::expr::parse_expression(&expression.text).map_err(|error| {
                    format!("failed to parse continuous measure '{statement_name}': {error}")
                })?;
            let prepared = LivePreparedExpression::compile(&parsed, params)?;
            dependencies.extend(
                prepared
                    .parameters
                    .values()
                    .map(|parameter| parameter.canonical_measure.clone()),
            );
            LiveEquationSource::Expression(prepared)
        }
        crate::netlist::measure::MeasureExpressionKind::RawReference => {
            let canonical_measure = expression.text.to_ascii_uppercase();
            dependencies.push(canonical_measure.clone());
            LiveEquationSource::RawReference {
                authored: expression.text.clone(),
                canonical_measure,
                canonical_signal: canonical_measure_signal_name(&expression.text),
            }
        }
        crate::netlist::measure::MeasureExpressionKind::RawOutputOperator => {
            LiveEquationSource::RawOutputOperator(LiveRawOutputOperator::compile(&expression.text)?)
        }
    };
    dependencies.sort();
    dependencies.dedup();
    Ok((source, dependencies))
}

fn compile_live_measure_state(
    statement: &MeasureStatement,
    analysis: &str,
    axis: &[Value],
    dc_sweep_ascending: Option<bool>,
    use_legacy_tran_trig_targ: bool,
    params: &crate::netlist::ParamContext,
) -> Result<LiveMeasureState, String> {
    let bounds = |from: Option<Value>, to: Option<Value>| {
        if analysis.eq_ignore_ascii_case("DC")
            && let (Some(from), Some(to)) = (from, to)
        {
            (from.min(to), from.max(to))
        } else {
            live_measurement_window_bounds(axis, from, to)
        }
    };
    let point_bounds = |from, to, td| {
        let (mut lower, upper) = bounds(from, to);
        if analysis.eq_ignore_ascii_case("TRAN")
            && let Some(td) = td
        {
            lower = lower.max(td);
        }
        (lower, upper)
    };
    Ok(match &statement.measure_type {
        MeasureType::Equation {
            expression,
            from,
            to,
            td,
        } => {
            let (source, dependencies) =
                compile_live_equation_source(expression, &statement.name, params)?;
            let (from, to) = if dc_sweep_ascending.is_some() {
                match (*from, *to) {
                    (Some(from), Some(to)) if from > to => (Some(to), Some(from)),
                    bounds => bounds,
                }
            } else {
                (*from, *to)
            };
            LiveMeasureState::Equation {
                source,
                dependencies,
                from,
                to,
                td: *td,
            }
        }
        MeasureType::Min {
            signal,
            from,
            to,
            output,
        }
        | MeasureType::Max {
            signal,
            from,
            to,
            output,
        } => {
            let (lower, upper) = bounds(*from, *to);
            LiveMeasureState::Extremum {
                signal: LiveMeasureOperand::compile(signal, params)?,
                lower,
                upper,
                output: *output,
                is_max: matches!(statement.measure_type, MeasureType::Max { .. }),
                selected: None,
            }
        }
        MeasureType::PeakToPeak { signal, from, to } => {
            let (lower, upper) = bounds(*from, *to);
            LiveMeasureState::PeakToPeak {
                signal: LiveMeasureOperand::compile(signal, params)?,
                lower,
                upper,
                minimum: Value::INFINITY,
                maximum: Value::NEG_INFINITY,
                initialized: false,
            }
        }
        MeasureType::Avg { signal, from, to } | MeasureType::Rms { signal, from, to } => {
            let (lower, upper) = bounds(*from, *to);
            LiveMeasureState::IntegralStatistic {
                signal: LiveMeasureOperand::compile(signal, params)?,
                lower,
                upper,
                mode: if matches!(statement.measure_type, MeasureType::Avg { .. }) {
                    LiveIntegralMode::Average
                } else {
                    LiveIntegralMode::Rms
                },
                integral: 0.0,
                width: 0.0,
                previous: None,
            }
        }
        MeasureType::Integ { signal, from, to } => {
            let sweep_direction = axis
                .first()
                .zip(axis.last())
                .map_or(1.0, |(first, last)| Value::signum(last - first));
            let (lower, upper, direction) = match (*from, *to) {
                (Some(from), Some(to)) => (from.min(to), from.max(to), Value::signum(to - from)),
                (Some(from), None) if sweep_direction >= 0.0 => {
                    (from, Value::INFINITY, sweep_direction)
                }
                (Some(from), None) => (Value::NEG_INFINITY, from, sweep_direction),
                (None, Some(to)) if sweep_direction >= 0.0 => {
                    (Value::NEG_INFINITY, to, sweep_direction)
                }
                (None, Some(to)) => (to, Value::INFINITY, sweep_direction),
                (None, None) => (Value::NEG_INFINITY, Value::INFINITY, sweep_direction),
            };
            LiveMeasureState::IntegralStatistic {
                signal: LiveMeasureOperand::compile(signal, params)?,
                lower,
                upper,
                mode: LiveIntegralMode::Integral { direction },
                integral: 0.0,
                width: 0.0,
                previous: None,
            }
        }
        MeasureType::ErrorFunction {
            measured,
            comparison,
            norm,
            from,
            to,
            minval,
            ymin,
            ymax,
            ..
        } => {
            let (lower, upper) = if analysis.eq_ignore_ascii_case("DC") {
                match (*from, *to) {
                    (Some(from), Some(to)) => (from.min(to), from.max(to)),
                    _ => bounds(*from, *to),
                }
            } else {
                bounds(*from, *to)
            };
            LiveMeasureState::ErrorFunction {
                measured: LiveMeasureOperand::compile(measured, params)?,
                comparison: LiveMeasureOperand::compile(comparison, params)?,
                norm: *norm,
                lower,
                upper,
                minval: *minval,
                ymin: *ymin,
                ymax: *ymax,
                sum: 0.0,
                count: 0,
            }
        }
        MeasureType::Find {
            signal,
            at,
            when,
            from,
            to,
            td,
            minval,
        }
        | MeasureType::Derivative {
            signal,
            at,
            when,
            from,
            to,
            td,
            minval,
        } => {
            let (lower, upper) = point_bounds(*from, *to, *td);
            LiveMeasureState::Point {
                signal: Some(LiveMeasureOperand::compile(signal, params)?),
                at: *at,
                condition: when
                    .as_ref()
                    .map(|condition| LiveCondition::compile(condition, *minval, params))
                    .transpose()?
                    .map(Box::new),
                lower,
                upper,
                minval: *minval,
                kind: if matches!(statement.measure_type, MeasureType::Find { .. }) {
                    LivePointKind::Find
                } else {
                    LivePointKind::Derivative
                },
                previous_signal: None,
                negative_results: VecDeque::new(),
                complete: false,
            }
        }
        MeasureType::When {
            condition,
            from,
            to,
            td,
            minval,
        } => {
            let (lower, upper) = point_bounds(*from, *to, *td);
            LiveMeasureState::Point {
                signal: None,
                at: None,
                condition: Some(Box::new(LiveCondition::compile(
                    condition, *minval, params,
                )?)),
                lower,
                upper,
                minval: *minval,
                kind: LivePointKind::When,
                previous_signal: None,
                negative_results: VecDeque::new(),
                complete: false,
            }
        }
        MeasureType::Delay {
            trig,
            targ,
            from,
            to,
            minval,
        } => {
            let legacy = analysis.eq_ignore_ascii_case("TRAN")
                && (use_legacy_tran_trig_targ
                    || trig.frac_max.is_some()
                    || targ.frac_max.is_some());
            if !legacy && (trig.frac_max.is_some() || targ.frac_max.is_some()) {
                return Err(format!(
                    "FRAC_MAX is supported only by scalar TRAN TRIG/TARG for measure {}",
                    statement.name
                ));
            }
            let axis_ascending = axis
                .windows(2)
                .find_map(|pair| (pair[0] != pair[1]).then_some(pair[1] > pair[0]))
                .unwrap_or(true);
            let axis_minimum = axis.iter().copied().fold(Value::INFINITY, Value::min);
            let axis_maximum = axis.iter().copied().fold(Value::NEG_INFINITY, Value::max);
            if legacy && matches!(&targ.event, TriggerEvent::At(_)) {
                return Err(format!(
                    "AT keyword not allowed in legacy TARG block for measure {}",
                    statement.name
                ));
            }
            let legacy_td = targ.td.or(trig.td);
            let trigger_td = if legacy { legacy_td } else { trig.td };
            let target_td = if legacy {
                legacy_td
            } else {
                targ.td.or(trig.td)
            };
            LiveMeasureState::Delay(Box::new(LiveDelayState {
                frac_tracker: (legacy && (trig.frac_max.is_some() || targ.frac_max.is_some()))
                    .then(|| LegacyFracDelayTracker::new(trig, targ, *minval)),
                trigger: LiveDelayClause::compile(
                    trig, trigger_td, *from, *to, *minval, legacy, params,
                )?,
                target: LiveDelayClause::compile(
                    targ, target_td, *from, *to, *minval, legacy, params,
                )?,
                axis_ascending,
                axis_minimum,
                axis_maximum,
            }))
        }
        MeasureType::RiseTime {
            signal,
            from_pct,
            to_pct,
            number,
        }
        | MeasureType::FallTime {
            signal,
            from_pct,
            to_pct,
            number,
        } => LiveMeasureState::RiseFall {
            signal: LiveMeasureOperand::compile(signal, params)?,
            samples: Vec::new(),
            from_pct: *from_pct,
            to_pct: *to_pct,
            number: *number,
            is_rise: matches!(statement.measure_type, MeasureType::RiseTime { .. }),
        },
        MeasureType::FileError { signal, .. } => LiveMeasureState::FileError {
            signal: LiveMeasureOperand::compile(signal, params)?,
            samples: Vec::new(),
            frozen: false,
        },
        MeasureType::Param { expression } => {
            let (source, dependencies) =
                compile_live_equation_source(expression, &statement.name, params)?;
            LiveMeasureState::Param {
                source,
                dependencies,
            }
        }
    })
}

fn live_measurement_window_bounds(
    axis: &[Value],
    from: Option<Value>,
    to: Option<Value>,
) -> (Value, Value) {
    let ascending = axis
        .windows(2)
        .find_map(|pair| (pair[0] != pair[1]).then_some(pair[1] > pair[0]))
        .unwrap_or(true);
    match (from, to) {
        (Some(from), Some(to)) => (from, to),
        (Some(from), None) if ascending => (from, Value::INFINITY),
        (Some(from), None) => (Value::NEG_INFINITY, from),
        (None, Some(to)) if ascending => (Value::NEG_INFINITY, to),
        (None, Some(to)) => (to, Value::INFINITY),
        (None, None) => (Value::NEG_INFINITY, Value::INFINITY),
    }
}

fn freeze_live_file_error(
    program: &mut LiveMeasureProgram<'_>,
    read_row: usize,
    axis: &[Value],
) -> bool {
    let (signal_name, sample_count) = match program.state.as_mut() {
        Some(LiveMeasureState::FileError {
            signal,
            samples,
            frozen,
            ..
        }) if !*frozen => {
            *frozen = true;
            (signal.authored.clone(), samples.len())
        }
        _ => return false,
    };
    if sample_count == 0 {
        // Xyce's ERROR getter caches the authored/default result even when a
        // forward reference reads it before the statement has accepted its
        // first point. Later updates do not change that cached scalar.
        program.initialized = true;
        return true;
    }
    let Some(LiveMeasureState::FileError { samples, .. }) = &program.state else {
        unreachable!("file ERROR state changed while freezing")
    };
    let mut signals = HashMap::new();
    signals.insert(signal_name, samples.as_slice());
    match MeasureEngine::evaluate_file_error_prefix_raw(
        program.statement,
        &axis[..sample_count],
        &signals,
    ) {
        Ok(value) => {
            program.current = value;
            program.initialized = true;
            if program.store_trace && program.values.len() == read_row + 1 {
                program.values[read_row] = value;
                program.valid[read_row] = !value.is_nan();
            }
        }
        Err(error) => program.failure = Some(error),
    }
    true
}

impl LiveCondition {
    fn compile(
        condition: &WhenCondition,
        minval: Value,
        params: &crate::netlist::ParamContext,
    ) -> Result<Self, String> {
        Ok(Self {
            left: LiveMeasureOperand::compile(&condition.left, params)?,
            right: match &condition.right {
                MeasureOperand::Constant(value) => LiveConditionOperand::Constant(*value),
                MeasureOperand::Waveform(name) => {
                    LiveConditionOperand::Waveform(LiveMeasureOperand::compile(name, params)?)
                }
            },
            edge: condition.occurrence.edge,
            number: condition.occurrence.number,
            minval,
            match_count: 0,
            negative_events: VecDeque::new(),
            previous: None,
        })
    }

    fn update(
        &mut self,
        row: usize,
        axis_value: Value,
        starts_segment: bool,
        signals: &CanonicalMeasureSignalIndex<'_>,
        reads: &mut LiveMeasureReadContext<'_, '_>,
        params: &crate::netlist::ParamContext,
        lower: Value,
        upper: Value,
        exclusive_lower: Option<Value>,
    ) -> Result<LiveConditionUpdate, String> {
        if starts_segment {
            self.previous = None;
        }
        let left = self.left.value(row, signals, reads, params)?;
        let right = match &mut self.right {
            LiveConditionOperand::Constant(value) => *value,
            LiveConditionOperand::Waveform(operand) => {
                operand.value(row, signals, reads, params)?
            }
        };
        let previous = self.previous.replace((axis_value, left, right));
        let Some((previous_axis, previous_left, previous_right)) = previous else {
            return Ok(LiveConditionUpdate::default());
        };
        let Some(crossing) = super::measure::measure_condition_crossing(
            previous_left,
            left,
            previous_right,
            right,
            self.minval,
        ) else {
            return Ok(LiveConditionUpdate::default());
        };
        let event_axis = previous_axis + crossing.fraction * (axis_value - previous_axis);
        if !super::measure::point_event_axis_in_window(event_axis, lower, upper, self.minval)
            || exclusive_lower.is_some_and(|lower| event_axis <= lower)
        {
            return Ok(LiveConditionUpdate::default());
        }
        self.match_count += match self.edge {
            EdgeType::Cross => 1,
            EdgeType::Rise => usize::from(crossing.direction == MeasureConditionDirection::Rise),
            EdgeType::Fall => usize::from(crossing.direction != MeasureConditionDirection::Rise),
        };
        let edge_matches = match self.edge {
            EdgeType::Rise => crossing.direction == MeasureConditionDirection::Rise,
            EdgeType::Fall => crossing.direction == MeasureConditionDirection::Fall,
            EdgeType::Cross => true,
        };
        if !edge_matches {
            return Ok(LiveConditionUpdate::default());
        }
        let current = LiveConditionEvent {
            fraction: crossing.fraction,
            axis: event_axis,
            current_within_minval: crossing.current_within_minval,
        };
        let selected = if self.number >= 0 {
            (self.match_count >= self.number as usize).then_some(current)
        } else if self.number < 0 {
            let Some(distance) = self.number.checked_abs().map(|distance| distance as usize) else {
                return Ok(LiveConditionUpdate {
                    current: Some(current),
                    selected: None,
                });
            };
            self.negative_events.push_back(current);
            if self.negative_events.len() > distance {
                self.negative_events.pop_front();
            }
            (self.negative_events.len() == distance)
                .then(|| self.negative_events.front().copied())
                .flatten()
        } else {
            None
        };
        Ok(LiveConditionUpdate {
            current: Some(current),
            selected,
        })
    }

    fn reset_segment(&mut self) {
        self.previous = None;
    }

    fn dependencies(&self, names: &mut Vec<String>) {
        names.extend(self.left.dependencies.iter().cloned());
        if let LiveConditionOperand::Waveform(right) = &self.right {
            names.extend(right.dependencies.iter().cloned());
        }
    }
}

impl LiveDelayClause {
    fn compile(
        clause: &TrigSpec,
        effective_td: Option<Value>,
        from: Option<Value>,
        to: Option<Value>,
        minval: Value,
        legacy: bool,
        params: &crate::netlist::ParamContext,
    ) -> Result<Self, String> {
        Ok(match &clause.event {
            TriggerEvent::At(at) => Self {
                at: Some(*at),
                condition: None,
                td: effective_td,
                from,
                to,
                selected: None,
                negative_events: VecDeque::new(),
                minval,
                legacy,
            },
            TriggerEvent::When(condition) => Self {
                at: None,
                condition: Some(LiveDelayCondition {
                    left: LiveMeasureOperand::compile(&condition.left, params)?,
                    right: match &condition.right {
                        MeasureOperand::Constant(value) => LiveConditionOperand::Constant(*value),
                        MeasureOperand::Waveform(name) => LiveConditionOperand::Waveform(
                            LiveMeasureOperand::compile(name, params)?,
                        ),
                    },
                    tracker: if legacy {
                        DelayConditionTracker::new_legacy(
                            condition.occurrence.edge,
                            condition.occurrence.number,
                            clause.occurrence_explicit,
                            minval,
                        )
                    } else {
                        DelayConditionTracker::new(
                            condition.occurrence.edge,
                            condition.occurrence.number,
                            clause.occurrence_explicit,
                            minval,
                        )
                    },
                    number: condition.occurrence.number,
                }),
                td: effective_td,
                from,
                to,
                selected: None,
                negative_events: VecDeque::new(),
                minval,
                legacy,
            },
        })
    }

    fn update(
        &mut self,
        row: usize,
        axis_value: Value,
        ascending: bool,
        axis_minimum: Value,
        axis_maximum: Value,
        starts_segment: bool,
        signals: &CanonicalMeasureSignalIndex<'_>,
        reads: &mut LiveMeasureReadContext<'_, '_>,
        params: &crate::netlist::ParamContext,
        after: Option<Value>,
        allow_selection: bool,
    ) -> Result<(), String> {
        if self.selected.is_some()
            && (self.at.is_some()
                || self
                    .condition
                    .as_ref()
                    .is_some_and(|condition| condition.number >= 0))
        {
            return Ok(());
        }
        if let Some(target) = self.at {
            if self.selected.is_none()
                && target.is_finite()
                && if self.legacy {
                    super::measure::legacy_delay_accepts_sample(
                        axis_value,
                        self.td,
                        self.from,
                        self.to,
                        self.minval,
                    ) && axis_value >= target
                } else {
                    live_delay_at_is_reached(
                        axis_value,
                        target,
                        ascending,
                        axis_minimum,
                        axis_maximum,
                        self.minval,
                    )
                }
            {
                self.selected = Some(if self.legacy { axis_value } else { target });
            }
            return Ok(());
        }
        let Some(condition) = &mut self.condition else {
            return Ok(());
        };
        if self.legacy
            && !super::measure::legacy_delay_accepts_sample(
                axis_value,
                self.td,
                self.from,
                self.to,
                condition.tracker.minval(),
            )
        {
            return Ok(());
        }
        if starts_segment {
            condition.tracker.reset_segment();
        }
        let left = condition.left.value(row, signals, reads, params)?;
        let right = match &mut condition.right {
            LiveConditionOperand::Constant(value) => *value,
            LiveConditionOperand::Waveform(operand) => {
                operand.value(row, signals, reads, params)?
            }
        };
        if let Some(event_axis) = condition.tracker.update_with_td(
            axis_value,
            left,
            right,
            (!self.legacy).then_some(self.td).flatten(),
        ) && allow_selection
            && after.is_none_or(|trigger| axis_value > trigger)
        {
            if condition.number < 0 && !self.legacy {
                let distance = condition.number.unsigned_abs();
                self.negative_events.push_back(event_axis);
                if self.negative_events.len() > distance {
                    self.negative_events.pop_front();
                }
                self.selected = (self.negative_events.len() == distance)
                    .then(|| self.negative_events.front().copied())
                    .flatten();
            } else {
                // RiseFallDelay treats every negative count as LAST and
                // overwrites the selected result on each matching window.
                self.selected = Some(event_axis);
            }
        }
        Ok(())
    }

    fn sample(
        &mut self,
        row: usize,
        signals: &CanonicalMeasureSignalIndex<'_>,
        reads: &mut LiveMeasureReadContext<'_, '_>,
        params: &crate::netlist::ParamContext,
    ) -> Result<(Value, Value), String> {
        let Some(condition) = &mut self.condition else {
            return Ok((0.0, 0.0));
        };
        let value = condition.left.value(row, signals, reads, params)?;
        let target = match &mut condition.right {
            LiveConditionOperand::Constant(value) => *value,
            LiveConditionOperand::Waveform(operand) => {
                operand.value(row, signals, reads, params)?
            }
        };
        Ok((value, target))
    }

    fn dependencies(&self, names: &mut Vec<String>) {
        if self.selected.is_some()
            && self
                .condition
                .as_ref()
                .is_some_and(|condition| condition.number >= 0)
        {
            return;
        }
        if let Some(condition) = &self.condition {
            names.extend(condition.left.dependencies.iter().cloned());
            if let LiveConditionOperand::Waveform(right) = &condition.right {
                names.extend(right.dependencies.iter().cloned());
            }
        }
    }
}

impl LiveMeasureState {
    fn all_dependencies(&self) -> Vec<String> {
        let mut names = Vec::new();
        match self {
            Self::Equation { dependencies, .. } | Self::Param { dependencies, .. } => {
                names.extend(dependencies.iter().cloned());
            }
            Self::Extremum { signal, .. }
            | Self::PeakToPeak { signal, .. }
            | Self::IntegralStatistic { signal, .. }
            | Self::RiseFall { signal, .. }
            | Self::FileError { signal, .. } => {
                names.extend(signal.dependencies.iter().cloned());
            }
            Self::ErrorFunction {
                measured,
                comparison,
                ..
            } => {
                names.extend(measured.dependencies.iter().cloned());
                names.extend(comparison.dependencies.iter().cloned());
            }
            Self::Point {
                signal, condition, ..
            } => {
                if let Some(signal) = signal {
                    names.extend(signal.dependencies.iter().cloned());
                }
                if let Some(condition) = condition {
                    condition.dependencies(&mut names);
                }
            }
            Self::Delay(state) => {
                state.trigger.dependencies(&mut names);
                state.target.dependencies(&mut names);
            }
        }
        names.sort();
        names.dedup();
        names
    }

    #[allow(clippy::too_many_arguments)]
    fn update(
        &mut self,
        row: usize,
        axis_value: Value,
        axis: &[Value],
        starts_segment: bool,
        signals: &CanonicalMeasureSignalIndex<'_>,
        reads: &mut LiveMeasureReadContext<'_, '_>,
        params: &crate::netlist::ParamContext,
        dc_sweep_ascending: Option<bool>,
    ) -> Result<Option<Value>, String> {
        match self {
            Self::Equation {
                source,
                from,
                to,
                td,
                ..
            } => {
                if !equation_axis_is_in_window(axis_value, *from, *to, *td, dc_sweep_ascending) {
                    return Ok(None);
                }
                // Raw references and raw circuit operators are deliberately
                // not reparsed as expressions. Besides preserving IEEE values,
                // this supports Xyce's legacy `I(YDEVICE BRANCH)` spelling.
                let value = match source {
                    LiveEquationSource::Expression(expression) => {
                        let value = expression.value(row, signals, reads, params)?;
                        crate::netlist::expr::normalize_xyce_expression_component(value.re)
                    }
                    LiveEquationSource::RawReference {
                        authored,
                        canonical_measure,
                        canonical_signal,
                    } => lookup_compiled_raw_equation_reference(
                        authored,
                        canonical_measure,
                        canonical_signal,
                        row,
                        signals,
                        reads,
                    )?,
                    LiveEquationSource::RawOutputOperator(operator) => {
                        operator.value(row, signals)?
                    }
                };
                Ok(Some(value))
            }
            Self::Extremum {
                signal,
                lower,
                upper,
                output,
                is_max,
                selected,
            } => {
                if !live_axis_in_window(axis_value, *lower, *upper, 1.0e-12) {
                    return Ok(None);
                }
                let value = signal.value(row, signals, reads, params)?;
                let replaces = selected.is_none_or(|(_, selected_value)| {
                    if *is_max {
                        value > selected_value
                    } else {
                        value < selected_value
                    }
                });
                if replaces {
                    *selected = Some((axis_value, value));
                }
                // Xyce's live getMeasureResult() for MIN/MAX always returns
                // the dependent extrema value. OUTPUT=SV/FREQ/TIME affects
                // terminal printing only.
                let _ = output;
                Ok(selected.map(|(_, value)| value))
            }
            Self::PeakToPeak {
                signal,
                lower,
                upper,
                minimum,
                maximum,
                initialized,
            } => {
                if !live_axis_in_window(axis_value, *lower, *upper, 1.0e-12) {
                    return Ok(None);
                }
                let value = signal.value(row, signals, reads, params)?;
                if !*initialized {
                    *minimum = value;
                    *maximum = value;
                    *initialized = true;
                } else {
                    if value < *minimum {
                        *minimum = value;
                    }
                    if value > *maximum {
                        *maximum = value;
                    }
                }
                Ok(Some(*maximum - *minimum))
            }
            Self::IntegralStatistic {
                signal,
                lower,
                upper,
                mode,
                integral,
                width,
                previous,
            } => {
                if !live_axis_in_window(axis_value, *lower, *upper, 1.0e-12) {
                    *previous = None;
                    return Ok(None);
                }
                let value = signal.value(row, signals, reads, params)?;
                if let Some((previous_axis, previous_value)) = *previous {
                    let dx = (axis_value - previous_axis).abs();
                    *width += dx;
                    *integral += match mode {
                        LiveIntegralMode::Average => 0.5 * (value + previous_value) * dx,
                        LiveIntegralMode::Rms => {
                            0.5 * (value * value + previous_value * previous_value) * dx
                        }
                        LiveIntegralMode::Integral { .. } => 0.5 * (value + previous_value) * dx,
                    };
                }
                *previous = Some((axis_value, value));
                Ok(match mode {
                    LiveIntegralMode::Average if *width > 0.0 => Some(*integral / *width),
                    LiveIntegralMode::Rms if *width > 0.0 => Some((*integral / *width).sqrt()),
                    LiveIntegralMode::Integral { direction } => Some(*integral * *direction),
                    LiveIntegralMode::Average | LiveIntegralMode::Rms => None,
                })
            }
            Self::ErrorFunction {
                measured,
                comparison,
                norm,
                lower,
                upper,
                minval,
                ymin,
                ymax,
                sum,
                count,
            } => {
                if !minval.is_finite() || !ymin.is_finite() || !ymax.is_finite() {
                    return Err("ERR limits must be finite".to_string());
                }
                if !live_axis_in_window(axis_value, *lower, *upper, *minval) {
                    return Ok(None);
                }
                let measured_value = measured.value(row, signals, reads, params)?;
                let comparison_value = comparison.value(row, signals, reads, params)?;
                let magnitude = measured_value.abs();
                if !(magnitude >= *ymin - ymin.abs() * 1.0e-12
                    && magnitude <= *ymax + ymax.abs() * 1.0e-12)
                {
                    // Xyce marks ERR initialized before applying YMIN/YMAX.
                    // With no qualifying rows its raw getter is 0/0 (NaN);
                    // once an aggregate exists, a skipped row retains it.
                    return Ok((*count == 0).then_some(Value::NAN));
                }
                let denominator = magnitude.max(*minval);
                let relative_error = (measured_value - comparison_value) / denominator;
                *sum += match norm {
                    ErrorFunctionNorm::RootMeanSquare => relative_error * relative_error,
                    ErrorFunctionNorm::MeanAbsolute => relative_error.abs(),
                };
                *count += 1;
                let mean = *sum / *count as Value;
                Ok(Some(match norm {
                    ErrorFunctionNorm::RootMeanSquare => mean.sqrt(),
                    ErrorFunctionNorm::MeanAbsolute => mean,
                }))
            }
            Self::Point {
                signal,
                at,
                condition,
                lower,
                upper,
                minval,
                kind,
                previous_signal,
                negative_results,
                complete,
            } => {
                if *complete {
                    return Ok(None);
                }
                if starts_segment {
                    if let Some(condition) = condition {
                        condition.reset_segment();
                    }
                    *previous_signal = None;
                }
                let current_signal = signal
                    .as_mut()
                    .map(|signal| signal.value(row, signals, reads, params))
                    .transpose()?;
                let prior_signal = *previous_signal;
                *previous_signal = current_signal.map(|value| (axis_value, value));
                if let Some(target) = at {
                    if !live_axis_in_window(*target, *lower, *upper, *minval) {
                        return Ok(None);
                    }
                    if matches!(kind, LivePointKind::Derivative) && row == 0 {
                        return Ok(None);
                    }
                    let Some(current_signal) = current_signal else {
                        return Ok(None);
                    };
                    let Some(relation) = accepted_row_at_match(
                        prior_signal.map(|(axis, _)| axis),
                        axis_value,
                        *target,
                        *minval,
                    ) else {
                        return Ok(None);
                    };
                    let candidate = match (*kind, relation) {
                        (LivePointKind::Find, AcceptedRowAtMatch::Current) => {
                            LivePointCandidate::Defined(current_signal)
                        }
                        (LivePointKind::Find, AcceptedRowAtMatch::PreviousSegment { fraction }) => {
                            let Some((_, previous_signal)) = prior_signal else {
                                return Ok(None);
                            };
                            let value = super::measure::interpolate_extended_real(
                                previous_signal,
                                current_signal,
                                fraction,
                            );
                            if value.is_nan() {
                                LivePointCandidate::Undefined
                            } else {
                                LivePointCandidate::Defined(value)
                            }
                        }
                        (LivePointKind::Derivative, _) => {
                            let (previous_axis, previous_signal) =
                                prior_signal.unwrap_or((axis_value, current_signal));
                            LivePointCandidate::Defined(accepted_row_secant_slope(
                                previous_axis,
                                previous_signal,
                                axis_value,
                                current_signal,
                            ))
                        }
                        (LivePointKind::When, _) => {
                            unreachable!("WHEN point state cannot carry AT")
                        }
                    };
                    *complete = true;
                    return Ok(Some(candidate.numeric_value()));
                }
                let Some(condition) = condition else {
                    return Ok(None);
                };
                let update = condition.update(
                    row,
                    axis_value,
                    starts_segment,
                    signals,
                    reads,
                    params,
                    *lower,
                    *upper,
                    None,
                )?;
                if matches!(kind, LivePointKind::When) {
                    let Some(event) = update.selected else {
                        return Ok(None);
                    };
                    if condition.number >= 0 {
                        *complete = true;
                    }
                    return Ok(Some(event.axis));
                }

                let resolve = |event: LiveConditionEvent| {
                    let Some((previous_axis, previous_value)) = prior_signal else {
                        return Err("Point event has no prior source sample".to_string());
                    };
                    let Some(current_value) = current_signal else {
                        return Err("Point event has no current source sample".to_string());
                    };
                    match kind {
                        LivePointKind::Find => {
                            let value = if event.current_within_minval {
                                current_value
                            } else {
                                super::measure::interpolate_extended_real(
                                    previous_value,
                                    current_value,
                                    event.fraction,
                                )
                            };
                            if value.is_nan() {
                                Ok(LivePointCandidate::Undefined)
                            } else {
                                Ok(LivePointCandidate::Defined(value))
                            }
                        }
                        LivePointKind::Derivative => {
                            Ok(LivePointCandidate::Defined(accepted_row_secant_slope(
                                previous_axis,
                                previous_value,
                                axis_value,
                                current_value,
                            )))
                        }
                        LivePointKind::When => unreachable!("WHEN was handled above"),
                    }
                };

                if condition.number >= 0 {
                    let Some(event) = update.selected else {
                        return Ok(None);
                    };
                    let candidate = resolve(event)?;
                    *complete = true;
                    return Ok(Some(candidate.numeric_value()));
                }
                if condition.number < 0 {
                    let Some(distance) = condition
                        .number
                        .checked_abs()
                        .map(|distance| distance as usize)
                    else {
                        return Ok(None);
                    };
                    if let Some(event) = update.current {
                        negative_results.push_back(resolve(event)?);
                        if negative_results.len() > distance {
                            negative_results.pop_front();
                        }
                    }
                    if update.selected.is_some() {
                        // Xyce's negative-RFC FIFO stores the raw candidate,
                        // including NaN, and remains active. Undefined is a
                        // recoverable numeric state, not a structural failure:
                        // later events can age it out of this bounded queue.
                        return Ok(negative_results
                            .front()
                            .map(LivePointCandidate::numeric_value));
                    }
                }
                Ok(None)
            }
            Self::Delay(state) => {
                let LiveDelayState {
                    trigger,
                    target,
                    frac_tracker,
                    axis_ascending,
                    axis_minimum,
                    axis_maximum,
                } = state.as_mut();
                if let Some(frac_tracker) = frac_tracker {
                    if !super::measure::legacy_delay_accepts_sample(
                        axis_value,
                        trigger.td,
                        trigger.from,
                        trigger.to,
                        trigger.minval,
                    ) {
                        return Ok(None);
                    }
                    let (trigger_value, trigger_target) =
                        trigger.sample(row, signals, reads, params)?;
                    let (target_value, target_target) =
                        target.sample(row, signals, reads, params)?;
                    let pair = frac_tracker.update(
                        axis_value,
                        trigger_value,
                        trigger_target,
                        target_value,
                        target_target,
                    );
                    trigger.selected = pair.map(|(trigger, _)| trigger);
                    target.selected = pair.map(|(_, target)| target);
                    return Ok(pair.map(|(trigger, target)| target - trigger));
                }
                trigger.update(
                    row,
                    axis_value,
                    *axis_ascending,
                    *axis_minimum,
                    *axis_maximum,
                    starts_segment,
                    signals,
                    reads,
                    params,
                    None,
                    true,
                )?;
                let trigger_axis = trigger.selected;
                let target_after = target.legacy.then_some(trigger_axis).flatten();
                target.update(
                    row,
                    axis_value,
                    *axis_ascending,
                    *axis_minimum,
                    *axis_maximum,
                    starts_segment,
                    signals,
                    reads,
                    params,
                    target_after,
                    !target.legacy || trigger_axis.is_some(),
                )?;
                Ok(trigger
                    .selected
                    .zip(target.selected)
                    .map(|(trigger, target)| target - trigger))
            }
            Self::RiseFall {
                signal,
                samples,
                from_pct,
                to_pct,
                number,
                is_rise,
            } => {
                samples.push(signal.value(row, signals, reads, params)?);
                if row + 1 != axis.len() {
                    return Ok(None);
                }
                Ok(super::measure::rise_fall_duration(
                    axis, samples, *from_pct, *to_pct, *number, *is_rise,
                ))
            }
            Self::FileError {
                signal,
                samples,
                frozen,
                ..
            } => {
                if !*frozen {
                    samples.push(signal.value(row, signals, reads, params)?);
                }
                Ok(None)
            }
            Self::Param { source, .. } => {
                if row + 1 != axis.len() {
                    return Ok(None);
                }
                let value = match source {
                    LiveEquationSource::Expression(expression) => {
                        let value = expression.value(row, signals, reads, params)?;
                        if params.expression_dialect() != crate::config::ExpressionDialect::Xyce
                            && (!is_real(value) || value.re.is_nan())
                        {
                            return Err(
                                "PARAM expression produced a non-real or NaN value".to_string()
                            );
                        }
                        if params.expression_dialect() == crate::config::ExpressionDialect::Xyce {
                            crate::netlist::expr::normalize_xyce_expression_component(value.re)
                        } else {
                            value.re
                        }
                    }
                    LiveEquationSource::RawReference {
                        authored,
                        canonical_measure,
                        canonical_signal,
                    } => lookup_compiled_raw_equation_reference(
                        authored,
                        canonical_measure,
                        canonical_signal,
                        row,
                        signals,
                        reads,
                    )?,
                    LiveEquationSource::RawOutputOperator(operator) => {
                        operator.value(row, signals)?
                    }
                };
                Ok(Some(value))
            }
        }
    }
}

fn live_axis_in_window(axis: Value, lower: Value, upper: Value, tolerance: Value) -> bool {
    let lower_tolerance = if lower.is_finite() {
        (lower * tolerance).abs()
    } else {
        0.0
    };
    let upper_tolerance = if upper.is_finite() {
        (upper * tolerance).abs()
    } else {
        0.0
    };
    axis >= lower - lower_tolerance && axis <= upper + upper_tolerance
}

fn live_delay_at_is_reached(
    current_axis: Value,
    target: Value,
    ascending: bool,
    axis_minimum: Value,
    axis_maximum: Value,
    minval: Value,
) -> bool {
    if target < axis_minimum || target > axis_maximum {
        return false;
    }
    if ascending {
        current_axis - minval >= target
    } else {
        current_axis - minval <= target
    }
}

fn equation_axis_is_in_window(
    axis_value: Value,
    from: Option<Value>,
    to: Option<Value>,
    td: Option<Value>,
    dc_sweep_ascending: Option<bool>,
) -> bool {
    const XYCE_MEASURE_WINDOW_TOLERANCE: Value = 1.0e-12;
    let at_or_above =
        |bound: Value| axis_value >= bound - bound.abs() * XYCE_MEASURE_WINDOW_TOLERANCE;
    let at_or_below =
        |bound: Value| axis_value <= bound + bound.abs() * XYCE_MEASURE_WINDOW_TOLERANCE;

    if let Some(ascending) = dc_sweep_ascending {
        return match (from, to) {
            (Some(from), Some(to)) => at_or_above(from.min(to)) && at_or_below(from.max(to)),
            (Some(from), None) if ascending => at_or_above(from),
            (Some(from), None) => at_or_below(from),
            (None, Some(to)) if ascending => at_or_below(to),
            (None, Some(to)) => at_or_above(to),
            (None, None) => true,
        };
    }

    td.is_none_or(at_or_above) && from.is_none_or(at_or_above) && to.is_none_or(at_or_below)
}

fn evaluate_measure_expression(
    expression: &NetExpr,
    row: usize,
    signals: &CanonicalMeasureSignalIndex<'_>,
    measures: &HashMap<String, Value>,
    params: &crate::netlist::ParamContext,
    normalize_nonfinite: bool,
    description: &str,
) -> Result<Value, String> {
    let bound = bind_equation_expression(expression, row, signals, measures)?;
    let value = crate::netlist::expr::evaluate_complex_raw(&bound, params)
        .map_err(|error| format!("{description} failed: {error}"))?;
    // Xyce evaluates measurement expressions as complex values, applies
    // fixNan/fixInf independently to both root components, then projects the
    // real component in MeasureBase::getOutputValue. Preserve that boundary:
    // bare measure getters remain raw, but every authored expression root is
    // finite before MAX/ERR/FIND/EQN/PARAM consumes it.
    if normalize_nonfinite {
        let real = crate::netlist::expr::normalize_xyce_expression_component(value.re);
        let _imaginary = crate::netlist::expr::normalize_xyce_expression_component(value.im);
        Ok(real)
    } else {
        Ok(value.re)
    }
}

fn lookup_compiled_raw_equation_reference(
    authored: &str,
    canonical_measure: &str,
    canonical_signal: &str,
    row: usize,
    signals: &CanonicalMeasureSignalIndex<'_>,
    reads: &mut LiveMeasureReadContext<'_, '_>,
) -> Result<Value, String> {
    if let Some(value) = reads.read_measure(canonical_measure) {
        return Ok(value);
    }
    lookup_equation_signal_canonical_optional(signals, authored, canonical_signal, row)?
        .ok_or_else(|| format!("raw equation reference '{authored}' is unavailable at row {row}"))
}

fn split_equation_output_operator(operator: &str) -> Option<(&str, Vec<String>)> {
    let (name, arguments) = operator.split_once('(')?;
    let arguments = arguments.strip_suffix(')')?;
    if name.is_empty() || arguments.is_empty() {
        return None;
    }
    let arguments = arguments
        .split(',')
        .map(str::trim)
        .map(str::to_string)
        .collect::<Vec<_>>();
    arguments
        .iter()
        .all(|argument| !argument.is_empty())
        .then_some((name, arguments))
}

fn is_equation_voltage_accessor(name: &str) -> bool {
    matches!(name, "V" | "VR" | "VI" | "VM" | "VP" | "VDB")
}

fn lookup_equation_voltage_operator(
    signals: &CanonicalMeasureSignalIndex<'_>,
    prefix: &str,
    arguments: &[String],
    row: usize,
) -> Result<Value, String> {
    if !(1..=2).contains(&arguments.len()) {
        return Err(format!(
            "{prefix}() in continuous measure requires one or two arguments"
        ));
    }
    let authored = format!("{prefix}({})", arguments.join(","));
    if let Some(value) = lookup_equation_signal_optional(signals, &authored, row)? {
        return Ok(value);
    }
    let node_component = |component: &str, node: &str| -> Result<Value, String> {
        if node == "0" {
            return Ok(0.0);
        }
        if let Some(value) =
            lookup_equation_signal_optional(signals, &format!("{component}({node})"), row)?
        {
            return Ok(value);
        }
        if component == "VR" {
            return lookup_equation_signal(signals, &format!("V({node})"), row);
        }
        Err(format!(
            "continuous measure signal '{component}({node})' is unavailable at row {row}"
        ))
    };
    if arguments.len() == 1 {
        return if prefix == "V" {
            lookup_equation_signal(signals, &format!("V({})", arguments[0]), row)
        } else {
            lookup_equation_signal(signals, &authored, row)
        };
    }

    let positive = &arguments[0];
    let negative = &arguments[1];
    match prefix {
        "V" | "VR" => Ok(node_component("VR", positive)? - node_component("VR", negative)?),
        "VI" => Ok(node_component("VI", positive)? - node_component("VI", negative)?),
        "VM" | "VP" | "VDB" => {
            let real = node_component("VR", positive)? - node_component("VR", negative)?;
            let imaginary = node_component("VI", positive)? - node_component("VI", negative)?;
            let magnitude = real.hypot(imaginary);
            Ok(match prefix {
                "VM" => magnitude,
                "VP" => imaginary.atan2(real).to_degrees(),
                "VDB" => 20.0 * magnitude.log10(),
                _ => unreachable!(),
            })
        }
        _ => unreachable!(),
    }
}

fn bind_equation_expression(
    expression: &NetExpr,
    row: usize,
    signals: &CanonicalMeasureSignalIndex<'_>,
    measures: &HashMap<String, Value>,
) -> Result<NetExpr, String> {
    Ok(match expression {
        NetExpr::Param(name) => {
            let is_axis_symbol = matches!(
                name.to_ascii_uppercase().as_str(),
                "TIME" | "FREQ" | "FREQUENCY" | "HERTZ"
            );
            if is_axis_symbol {
                if let Some(value) = lookup_equation_signal_optional(signals, name, row)? {
                    NetExpr::Number(value)
                } else if let Some(value) = measures.get(&name.to_ascii_uppercase()).copied() {
                    NetExpr::Number(value)
                } else {
                    expression.clone()
                }
            } else if let Some(value) = measures.get(&name.to_ascii_uppercase()).copied() {
                NetExpr::Number(value)
            } else if let Some(value) = lookup_equation_signal_optional(signals, name, row)? {
                NetExpr::Number(value)
            } else {
                expression.clone()
            }
        }
        NetExpr::UnaryOp { op, operand } => NetExpr::UnaryOp {
            op: *op,
            operand: Box::new(bind_equation_expression(operand, row, signals, measures)?),
        },
        NetExpr::BinOp { op, left, right } => NetExpr::BinOp {
            op: *op,
            left: Box::new(bind_equation_expression(left, row, signals, measures)?),
            right: Box::new(bind_equation_expression(right, row, signals, measures)?),
        },
        NetExpr::FnCall { name, args } if is_equation_probe_accessor(name) => {
            let prefix = name.to_ascii_uppercase();
            if is_equation_voltage_accessor(&prefix) {
                let arguments = args
                    .iter()
                    .map(|argument| {
                        equation_probe_argument(Some(argument)).ok_or_else(|| {
                            format!("{prefix}() in continuous measure has an invalid argument")
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                NetExpr::Number(lookup_equation_voltage_operator(
                    signals, &prefix, &arguments, row,
                )?)
            } else if args.len() == 1 {
                let first = equation_probe_argument(args.first()).ok_or_else(|| {
                    format!("{prefix}() in continuous measure has an invalid argument")
                })?;
                NetExpr::Number(lookup_equation_signal(
                    signals,
                    &format!("{prefix}({first})"),
                    row,
                )?)
            } else {
                return Err(format!(
                    "{prefix}() in continuous measure has invalid arity"
                ));
            }
        }
        NetExpr::FnCall { name, args } if is_equation_noise_accessor(name) => {
            let prefix = name.to_ascii_uppercase();
            if !(1..=2).contains(&args.len()) {
                return Err(format!(
                    "{prefix}() in continuous measure requires one or two arguments"
                ));
            }
            let arguments = args
                .iter()
                .map(|argument| {
                    equation_probe_argument(Some(argument)).ok_or_else(|| {
                        format!("{prefix}() in continuous measure has an invalid argument")
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            let probe = format!("{prefix}({})", arguments.join(","));
            NetExpr::Number(lookup_equation_signal(signals, &probe, row)?)
        }
        NetExpr::FnCall { name, args } if is_equation_generic_output_accessor(name) => {
            let prefix = name.to_ascii_uppercase();
            let arguments = args
                .iter()
                .map(|argument| {
                    equation_probe_argument(Some(argument)).ok_or_else(|| {
                        format!("{prefix}() in continuous measure has an invalid argument")
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            let probe = format!("{prefix}({})", arguments.join(","));
            NetExpr::Number(lookup_equation_signal(signals, &probe, row)?)
        }
        NetExpr::FnCall { name, args } => NetExpr::FnCall {
            name: name.clone(),
            args: args
                .iter()
                .map(|arg| bind_equation_expression(arg, row, signals, measures))
                .collect::<Result<Vec<_>, _>>()?,
        },
        NetExpr::Number(_) | NetExpr::ComplexNumber(_) | NetExpr::StringLiteral(_) => {
            expression.clone()
        }
    })
}

fn is_equation_probe_accessor(name: &str) -> bool {
    matches!(
        name.to_ascii_uppercase().as_str(),
        "V" | "VM" | "VR" | "VI" | "VP" | "VDB" | "I" | "IM" | "IR" | "II" | "IP" | "IDB"
    )
}

fn is_equation_noise_accessor(name: &str) -> bool {
    matches!(name.to_ascii_uppercase().as_str(), "DNO" | "DNI")
}

fn is_equation_generic_output_accessor(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    !is_equation_probe_accessor(&upper)
        && !is_equation_noise_accessor(&upper)
        && upper != "IF"
        && (matches!(upper.as_str(), "N" | "P" | "W")
            || is_current_output_accessor(&upper)
            || is_equation_rf_accessor(&upper))
}

fn is_equation_rf_accessor(name: &str) -> bool {
    let Some((family, suffix)) = name.split_at_checked(1) else {
        return false;
    };
    matches!(family, "S" | "Y" | "Z") && matches!(suffix, "" | "R" | "I" | "M" | "P" | "DB")
}

fn equation_probe_argument(argument: Option<&NetExpr>) -> Option<String> {
    match argument? {
        NetExpr::Param(name) => Some(name.clone()),
        NetExpr::Number(value) if value.is_finite() && value.fract() == 0.0 => {
            Some(format!("{value:.0}"))
        }
        _ => None,
    }
}

fn lookup_equation_signal(
    signals: &CanonicalMeasureSignalIndex<'_>,
    name: &str,
    row: usize,
) -> Result<Value, String> {
    lookup_equation_signal_optional(signals, name, row)?
        .ok_or_else(|| format!("continuous measure signal '{name}' is unavailable at row {row}"))
}

fn lookup_equation_signal_optional(
    signals: &CanonicalMeasureSignalIndex<'_>,
    name: &str,
    row: usize,
) -> Result<Option<Value>, String> {
    Ok(signals
        .get(name)?
        .and_then(|values| values.get(row).copied()))
}

fn lookup_equation_signal_canonical_optional(
    signals: &CanonicalMeasureSignalIndex<'_>,
    authored_name: &str,
    canonical_name: &str,
    row: usize,
) -> Result<Option<Value>, String> {
    Ok(signals
        .get_canonical(authored_name, canonical_name)?
        .and_then(|values| values.get(row).copied()))
}

fn dc_sweep_coordinate(row: usize, value: Value) -> String {
    format!("sweep point {row} ({value:.16e})")
}

fn frequency_sweep_coordinate(row: usize, frequency: Value) -> String {
    format!("frequency point {row} ({frequency:.16e} Hz)")
}

fn result_schema_mismatch(
    analysis: &str,
    coordinate: String,
    signal_family: &str,
    expected_names: &[String],
    actual_names: &[String],
    expected_value_count: usize,
    actual_value_count: usize,
) -> SimulationError {
    SimulationError::result_schema_mismatch(
        analysis,
        Some(coordinate),
        signal_family,
        expected_names.to_vec(),
        actual_names.to_vec(),
        expected_value_count,
        actual_value_count,
    )
}

fn validate_named_result_schema(
    analysis: &str,
    coordinate: String,
    signal_family: &str,
    expected_names: &[String],
    actual_names: &[String],
    actual_value_count: usize,
) -> Result<(), SimulationError> {
    let expected_value_count = expected_names.len();
    if actual_names == expected_names && actual_value_count == expected_value_count {
        return Ok(());
    }
    Err(result_schema_mismatch(
        analysis,
        coordinate,
        signal_family,
        expected_names,
        actual_names,
        expected_value_count,
        actual_value_count,
    ))
}

fn validate_dc_sweep_schema(
    sweep: &[(Value, SimulationResult)],
    first: &SimulationResult,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    for (row, (value, result)) in sweep.iter().enumerate() {
        if row.is_multiple_of(64) && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let coordinate = dc_sweep_coordinate(row, *value);
        validate_named_result_schema(
            "DC",
            coordinate.clone(),
            "node voltages",
            &first.node_names,
            &result.node_names,
            result.node_voltages.len(),
        )?;
        validate_named_result_schema(
            "DC",
            coordinate,
            "branch currents",
            &first.branch_names,
            &result.branch_names,
            result.branch_currents.len(),
        )?;
    }
    Ok(())
}

fn validate_ac_sweep_schema(
    sweep: &[AcResult],
    first: &AcResult,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    for (row, point) in sweep.iter().enumerate() {
        if row.is_multiple_of(64) && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let coordinate = frequency_sweep_coordinate(row, point.frequency);
        validate_named_result_schema(
            "AC",
            coordinate.clone(),
            "node voltages",
            &first.node_names,
            &point.node_names,
            point.voltages.len(),
        )?;
        validate_named_result_schema(
            "AC",
            coordinate,
            "branch currents",
            &first.branch_names,
            &point.branch_names,
            point.currents.len(),
        )?;
    }
    Ok(())
}

fn validate_noise_sweep_schema(
    sweep: &[crate::analysis::NoiseResult],
    first: &crate::analysis::NoiseResult,
) -> Result<(), SimulationError> {
    for (row, point) in sweep.iter().enumerate() {
        let coordinate = frequency_sweep_coordinate(row, point.frequency);
        validate_named_result_schema(
            "NOISE",
            coordinate.clone(),
            "node voltages",
            &first.node_names,
            &point.node_names,
            point.voltages.len(),
        )?;
        validate_named_result_schema(
            "NOISE",
            coordinate,
            "branch currents",
            &first.branch_names,
            &point.branch_names,
            point.currents.len(),
        )?;
    }
    Ok(())
}

fn noise_identity_name(identity: &crate::analysis::NoiseSourceIdentity) -> String {
    match &identity.mechanism {
        Some(mechanism) => format!("{}:{mechanism}", identity.device),
        None => identity.device.clone(),
    }
}

fn noise_catalog_names(point: &crate::analysis::NoiseResult) -> Vec<String> {
    point
        .contribution_catalog
        .iter()
        .map(noise_identity_name)
        .collect()
}

/// Owned per-signal series extracted from a DC sweep, from which a
/// measurement signal table can be borrowed.
pub struct DcSweepSeries {
    axis: Vec<Value>,
    /// (raw name, prefix, series)
    storage: Vec<(String, char, Vec<Value>)>,
    /// (complete SPICE probe name, series)
    observables: Vec<(String, Vec<Value>)>,
}

impl DcSweepSeries {
    /// Collect node-voltage and branch-current series across the sweep.
    /// Returns `Ok(None)` for an empty sweep. Every accepted point must retain
    /// the same named result schema; malformed or topology-dependent rows fail
    /// explicitly instead of being padded with plausible zero values.
    pub fn from_sweep(
        sweep: &[(Value, SimulationResult)],
    ) -> Result<Option<Self>, SimulationError> {
        Self::from_sweep_with_abort(sweep, &NoAbort)
    }

    fn from_sweep_with_abort(
        sweep: &[(Value, SimulationResult)],
        abort: &dyn AbortSignal,
    ) -> Result<Option<Self>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let Some((_, first)) = sweep.first() else {
            return Ok(None);
        };
        validate_dc_sweep_schema(sweep, first, abort)?;
        let mut axis = Vec::with_capacity(sweep.len());
        for (index, (value, _)) in sweep.iter().enumerate() {
            if index.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            axis.push(*value);
        }

        let mut storage: Vec<(String, char, Vec<Value>)> = Vec::new();
        for node in 1..first.node_voltages.len() {
            if node.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let mut series = Vec::with_capacity(sweep.len());
            for (row, (_, result)) in sweep.iter().enumerate() {
                if row.is_multiple_of(64) && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let value = result.node_voltages.get(node).copied().ok_or_else(|| {
                    result_schema_mismatch(
                        "DC",
                        dc_sweep_coordinate(row, sweep[row].0),
                        "node voltages",
                        &first.node_names,
                        &result.node_names,
                        first.node_voltages.len(),
                        result.node_voltages.len(),
                    )
                })?;
                series.push(value);
            }
            let fallback = node.to_string();
            let raw = first
                .node_names
                .get(node)
                .filter(|name| !name.is_empty())
                .cloned()
                .unwrap_or(fallback);
            // A numeric node name is still a canonical SPICE name, not the
            // solver's internal node index. Adding index aliases for named
            // nodes can overwrite real V(1), V(2), ... waveforms when solver
            // ordering differs from numeric-name ordering.
            storage.push((raw, 'V', series));
        }
        for (branch, name) in first.branch_names.iter().enumerate() {
            if branch.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if name.is_empty() {
                continue;
            }
            let mut series = Vec::with_capacity(sweep.len());
            for (row, (_, result)) in sweep.iter().enumerate() {
                if row.is_multiple_of(64) && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let value = result.branch_currents.get(branch).copied().ok_or_else(|| {
                    result_schema_mismatch(
                        "DC",
                        dc_sweep_coordinate(row, sweep[row].0),
                        "branch currents",
                        &first.branch_names,
                        &result.branch_names,
                        first.branch_currents.len(),
                        result.branch_currents.len(),
                    )
                })?;
                series.push(value);
            }
            storage.push((name.clone(), 'I', series));
        }

        // Parameter sweeps can rebuild the circuit at every point and may
        // change the lowering topology (for example, a resistor can cross the
        // threshold between nodal and explicit-branch forms). Form a
        // case-insensitive union before requiring a complete waveform so a
        // name first introduced after row zero is not silently omitted.
        let mut observable_names = Vec::<String>::new();
        for (row, (_, result)) in sweep.iter().enumerate() {
            if row.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            for (index, (name, _)) in result.dc_observables.iter().enumerate() {
                if index.is_multiple_of(64) && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                if !observable_names
                    .iter()
                    .any(|candidate| candidate.eq_ignore_ascii_case(name))
                {
                    observable_names.push(name.clone());
                }
            }
        }
        let mut observables = Vec::new();
        for (index, name) in observable_names.into_iter().enumerate() {
            if index.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let mut values = Vec::with_capacity(sweep.len());
            let mut complete = true;
            for (row, (_, result)) in sweep.iter().enumerate() {
                if row.is_multiple_of(64) && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let Some(value) = result.try_dc_observable_named(&name) else {
                    complete = false;
                    break;
                };
                values.push(value);
            }
            if complete {
                observables.push((name, values));
            }
        }

        Ok(Some(Self {
            axis,
            storage,
            observables,
        }))
    }

    /// The swept values, used as the measurement abscissa.
    pub fn axis(&self) -> &[Value] {
        &self.axis
    }

    /// Borrowed signal table over the collected series.
    pub fn signal_map(&self) -> HashMap<String, &[Value]> {
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        // The sweep axis, so `FIND TIME WHEN ...` addresses the swept value.
        insert_case_variants(&mut signals, "Time", self.axis.as_slice());
        for (raw, prefix, series) in &self.storage {
            insert_wrapped_variants(&mut signals, *prefix, raw, series.as_slice());
            if *prefix == 'V' {
                insert_case_variants(&mut signals, raw, series.as_slice());
            }
        }
        for (name, series) in &self.observables {
            insert_case_variants(&mut signals, name, series.as_slice());
        }
        signals
    }
}

/// Owned derived real projections of named complex signals.
///
/// AC and NOISE share Xyce's projection names and scalar-accessor semantics,
/// so the projections are materialized by one implementation. Keeping this
/// storage owned also makes every slice in the borrowed measurement map stable
/// for the duration of evaluation.
#[derive(Default)]
struct ComplexProjectionSeries {
    storage: Vec<(String, Vec<Value>)>,
}

impl ComplexProjectionSeries {
    fn push(&mut self, prefix: char, raw: &str, values: Vec<crate::Complex64>) {
        match self.push_with_abort(prefix, raw, &values, &NoAbort) {
            Ok(()) => {}
            Err(SimulationError::Aborted) => {
                unreachable!("NoAbort cannot cancel complex measurement projection")
            }
            Err(error) => unreachable!("complex measurement projection is infallible: {error}"),
        }
    }

    fn push_with_abort(
        &mut self,
        prefix: char,
        raw: &str,
        values: &[crate::Complex64],
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let mut magnitude = Vec::with_capacity(values.len());
        let mut db = Vec::with_capacity(values.len());
        let mut phase_deg = Vec::with_capacity(values.len());
        let mut real = Vec::with_capacity(values.len());
        let mut imag = Vec::with_capacity(values.len());
        for (index, value) in values.iter().enumerate() {
            if index.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let norm = value.norm();
            magnitude.push(norm);
            db.push(20.0 * norm.log10());
            phase_deg.push(value.arg().to_degrees());
            real.push(value.re);
            imag.push(value.im);
        }

        self.storage
            .push((format!("{prefix}({raw})"), magnitude.clone()));
        self.storage.push((format!("{prefix}M({raw})"), magnitude));
        self.storage.push((format!("{prefix}DB({raw})"), db));
        self.storage.push((format!("{prefix}P({raw})"), phase_deg));
        self.storage.push((format!("{prefix}R({raw})"), real));
        self.storage.push((format!("{prefix}I({raw})"), imag));
        if abort.is_aborted() {
            Err(SimulationError::Aborted)
        } else {
            Ok(())
        }
    }

    fn insert_all<'a>(&'a self, signals: &mut HashMap<String, &'a [Value]>) {
        for (key, series) in &self.storage {
            insert_case_variants(signals, key, series.as_slice());
        }
    }

    fn insert_scalar_aliases<'a>(&'a self, signals: &mut HashMap<String, &'a [Value]>) {
        for (key, series) in &self.storage {
            let upper = key.to_ascii_uppercase();
            let unqualified = if upper.starts_with("VR(") {
                Some(format!("V{}", &key[2..]))
            } else if upper.starts_with("IR(") {
                Some(format!("I{}", &key[2..]))
            } else {
                None
            };
            if let Some(unqualified) = unqualified {
                insert_case_variants(signals, &unqualified, series.as_slice());
            }
        }
    }
}

fn insert_frequency_axis_variants<'a>(
    signals: &mut HashMap<String, &'a [Value]>,
    axis: &'a [Value],
) {
    insert_case_variants(signals, "Time", axis);
    insert_case_variants(signals, "Frequency", axis);
    insert_case_variants(signals, "Freq", axis);
    insert_case_variants(signals, "Hertz", axis);
}

/// Owned per-signal series derived from an AC sweep, from which a
/// measurement signal table can be borrowed.
///
/// AC quantities are complex; measurements address the standard derived
/// real series. For a node `x`: `V(x)`/`VM(x)` magnitude, `VDB(x)`
/// 20·log10 magnitude, `VP(x)` phase in degrees, `VR(x)`/`VI(x)` real and
/// imaginary parts; branch currents use the `I` prefix the same way. The
/// frequency axis is reachable as `TIME` (the measurement abscissa),
/// `FREQUENCY`, and `FREQ`.
pub struct AcSweepSeries {
    axis: Vec<Value>,
    projections: ComplexProjectionSeries,
}

impl AcSweepSeries {
    /// Collect the derived real series across the sweep. Returns `Ok(None)`
    /// for an empty sweep and a typed schema error for malformed rows.
    pub fn from_sweep(sweep: &[AcResult]) -> Result<Option<Self>, SimulationError> {
        Self::from_sweep_with_abort(sweep, &NoAbort)
    }

    fn from_sweep_with_abort(
        sweep: &[AcResult],
        abort: &dyn AbortSignal,
    ) -> Result<Option<Self>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let Some(first) = sweep.first() else {
            return Ok(None);
        };
        validate_ac_sweep_schema(sweep, first, abort)?;
        let mut axis = Vec::with_capacity(sweep.len());
        for (index, point) in sweep.iter().enumerate() {
            if index.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            axis.push(point.frequency);
        }

        let mut projections = ComplexProjectionSeries::default();

        for (index, name) in first.node_names.iter().enumerate() {
            if index.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let raw = if name.is_empty() {
                (index + 1).to_string()
            } else {
                name.clone()
            };
            let values = sweep
                .iter()
                .enumerate()
                .map(|(row, point)| {
                    point.voltages.get(index).copied().ok_or_else(|| {
                        result_schema_mismatch(
                            "AC",
                            frequency_sweep_coordinate(row, point.frequency),
                            "node voltages",
                            &first.node_names,
                            &point.node_names,
                            first.voltages.len(),
                            point.voltages.len(),
                        )
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            projections.push_with_abort('V', &raw, &values, abort)?;
        }
        for (index, name) in first.branch_names.iter().enumerate() {
            if index.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if name.is_empty() {
                continue;
            }
            let values: Vec<crate::Complex64> = sweep
                .iter()
                .enumerate()
                .map(|(row, point)| {
                    point.currents.get(index).copied().ok_or_else(|| {
                        result_schema_mismatch(
                            "AC",
                            frequency_sweep_coordinate(row, point.frequency),
                            "branch currents",
                            &first.branch_names,
                            &point.branch_names,
                            first.currents.len(),
                            point.currents.len(),
                        )
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            projections.push_with_abort('I', name, &values, abort)?;
        }

        Ok(Some(Self { axis, projections }))
    }

    /// The sweep frequencies, used as the measurement abscissa.
    pub fn axis(&self) -> &[Value] {
        &self.axis
    }

    /// Borrowed signal table over the collected series.
    pub fn signal_map(&self) -> HashMap<String, &[Value]> {
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        insert_frequency_axis_variants(&mut signals, self.axis.as_slice());
        self.projections.insert_all(&mut signals);
        signals
    }

    /// Signal table used by continuous AC equation measures.
    ///
    /// Xyce scalar AC measures and equation expressions define unqualified
    /// `V()`/`I()` accessors as the real projection. This view overlays those
    /// aliases without changing the explicit magnitude series in `VM()`/`IM()`.
    pub(crate) fn equation_signal_map(&self) -> HashMap<String, &[Value]> {
        let mut signals = self.signal_map();
        self.projections.insert_scalar_aliases(&mut signals);
        signals
    }
}

/// Owned series derived from a noise sweep: output and input-referred
/// spectral densities addressable as `ONOISE`/`INOISE` (also with the
/// `_SPECTRUM` suffix, matching the exported column names), with the
/// frequency axis as `TIME`/`FREQUENCY`/`FREQ`.
pub struct NoiseSweepSeries {
    axis: Vec<Value>,
    onoise: Vec<Value>,
    inoise: Vec<Value>,
    projections: ComplexProjectionSeries,
    contributions: Vec<(String, Vec<Value>)>,
}

impl NoiseSweepSeries {
    /// Collect spectral-density series across the sweep. Returns `Ok(None)`
    /// for an empty sweep and fails explicitly if the device contribution
    /// catalog changes between accepted frequency points.
    pub fn from_sweep(
        sweep: &[crate::analysis::NoiseResult],
    ) -> Result<Option<Self>, SimulationError> {
        let Some(first) = sweep.first() else {
            return Ok(None);
        };
        validate_noise_sweep_schema(sweep, first)?;
        let catalog_key = |identity: &crate::analysis::NoiseSourceIdentity| {
            (
                identity.device.to_ascii_uppercase(),
                identity
                    .mechanism
                    .as_ref()
                    .map(|mechanism| mechanism.to_ascii_uppercase()),
            )
        };
        let sorted_catalog = |point: &crate::analysis::NoiseResult| {
            let mut catalog = point
                .contribution_catalog
                .iter()
                .map(catalog_key)
                .collect::<Vec<_>>();
            catalog.sort_unstable();
            catalog
        };
        let expected_catalog = sorted_catalog(first);
        for (row, point) in sweep.iter().enumerate().skip(1) {
            let actual_catalog = sorted_catalog(point);
            if actual_catalog != expected_catalog {
                let expected_names = noise_catalog_names(first);
                let actual_names = noise_catalog_names(point);
                return Err(result_schema_mismatch(
                    "NOISE",
                    frequency_sweep_coordinate(row, point.frequency),
                    "noise contribution catalog",
                    &expected_names,
                    &actual_names,
                    expected_names.len(),
                    actual_names.len(),
                ));
            }
        }

        let mut projections = ComplexProjectionSeries::default();
        for (index, name) in first.node_names.iter().enumerate() {
            let raw = if name.is_empty() {
                (index + 1).to_string()
            } else {
                name.clone()
            };
            projections.push(
                'V',
                &raw,
                sweep
                    .iter()
                    .enumerate()
                    .map(|(row, point)| {
                        point.voltages.get(index).copied().ok_or_else(|| {
                            result_schema_mismatch(
                                "NOISE",
                                frequency_sweep_coordinate(row, point.frequency),
                                "node voltages",
                                &first.node_names,
                                &point.node_names,
                                first.voltages.len(),
                                point.voltages.len(),
                            )
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            );
        }
        for (index, name) in first.branch_names.iter().enumerate() {
            if name.is_empty() {
                continue;
            }
            projections.push(
                'I',
                name,
                sweep
                    .iter()
                    .enumerate()
                    .map(|(row, point)| {
                        point.currents.get(index).copied().ok_or_else(|| {
                            result_schema_mismatch(
                                "NOISE",
                                frequency_sweep_coordinate(row, point.frequency),
                                "branch currents",
                                &first.branch_names,
                                &point.branch_names,
                                first.currents.len(),
                                point.currents.len(),
                            )
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            );
        }
        let mut contribution_probes = Vec::new();
        let mut seen_devices = HashSet::new();
        let mut seen_mechanisms = HashSet::new();
        for identity in &first.contribution_catalog {
            let device_key = identity.device.to_ascii_uppercase();
            if seen_devices.insert(device_key) {
                contribution_probes.push((identity.device.clone(), None));
            }
            if let Some(mechanism) = &identity.mechanism {
                let key = (
                    identity.device.to_ascii_uppercase(),
                    mechanism.to_ascii_uppercase(),
                );
                if seen_mechanisms.insert(key) {
                    contribution_probes.push((identity.device.clone(), Some(mechanism.clone())));
                }
            }
        }
        let mut contributions = Vec::with_capacity(contribution_probes.len() * 2);
        for (device, mechanism) in contribution_probes {
            for (prefix, kind) in [
                ("DNO", NoiseContributionKind::Output),
                ("DNI", NoiseContributionKind::Input),
            ] {
                let probe = NoiseContributionProbe {
                    kind,
                    device: device.clone(),
                    mechanism: mechanism.clone(),
                };
                let name = match &mechanism {
                    Some(mechanism) => format!("{prefix}({device},{mechanism})"),
                    None => format!("{prefix}({device})"),
                };
                let values = sweep
                    .iter()
                    .enumerate()
                    .map(|(row, point)| {
                        point.contribution(&probe).map_err(|_| {
                            let actual_names = point
                                .contributions
                                .iter()
                                .map(|contribution| noise_identity_name(&contribution.identity))
                                .collect::<Vec<_>>();
                            result_schema_mismatch(
                                "NOISE",
                                frequency_sweep_coordinate(row, point.frequency),
                                "noise contributions",
                                &noise_catalog_names(point),
                                &actual_names,
                                point.contribution_catalog.len(),
                                point.contributions.len(),
                            )
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                contributions.push((name, values));
            }
        }

        Ok(Some(Self {
            axis: sweep.iter().map(|point| point.frequency).collect(),
            // Xyce passes the total one-sided power spectral densities directly
            // to the ONOISE and INOISE operators.  The exported
            // `*_SPECTRUM` columns therefore have units of V^2/Hz (or the
            // corresponding input-referred units), not amplitude-density
            // units.  Keep square-root conversion confined to the explicit
            // `NoiseResult::*_rms` convenience methods.
            onoise: sweep
                .iter()
                .map(|point| point.output_noise_density)
                .collect(),
            inoise: sweep
                .iter()
                .map(|point| point.input_referred_density)
                .collect(),
            projections,
            contributions,
        }))
    }

    /// The sweep frequencies, used as the measurement abscissa.
    pub fn axis(&self) -> &[Value] {
        &self.axis
    }

    /// Borrowed signal table over the collected series.
    pub fn signal_map(&self) -> HashMap<String, &[Value]> {
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        insert_frequency_axis_variants(&mut signals, self.axis.as_slice());
        self.projections.insert_all(&mut signals);
        for key in ["Onoise", "Onoise_Spectrum"] {
            insert_case_variants(&mut signals, key, self.onoise.as_slice());
        }
        for key in ["Inoise", "Inoise_Spectrum"] {
            insert_case_variants(&mut signals, key, self.inoise.as_slice());
        }
        for (name, waveform) in &self.contributions {
            insert_case_variants(&mut signals, name, waveform.as_slice());
        }
        signals
    }

    /// Signal table used by continuous NOISE equation measures. Bare complex
    /// probes are overlaid with their real-component projection.
    pub(crate) fn equation_signal_map(&self) -> HashMap<String, &[Value]> {
        let mut signals = self.signal_map();
        self.projections.insert_scalar_aliases(&mut signals);
        signals
    }
}

/// Evaluate the netlist's NOISE .MEAS statements against a sweep.
///
/// Returns an empty vector when the netlist has no NOISE measurements; an
/// empty sweep fails every statement explicitly rather than skipping it.
pub fn evaluate_noise_measurements(
    netlist: &Netlist,
    sweep: &[crate::analysis::NoiseResult],
) -> Vec<MeasureResult> {
    match evaluate_noise_measurements_with_abort(netlist, sweep, &NoAbort) {
        Ok(results) => results,
        Err(SimulationError::Aborted) => {
            unreachable!("NoAbort cannot cancel noise measurement projection")
        }
        Err(error) => {
            let statements = measurements_for_analysis(netlist, "NOISE");
            failed_measurements(&statements, &error.to_string())
        }
    }
}

/// Evaluate scalar NOISE measurements with cooperative cancellation.
pub fn evaluate_noise_measurements_with_abort(
    netlist: &Netlist,
    sweep: &[crate::analysis::NoiseResult],
    abort: &dyn AbortSignal,
) -> Result<Vec<MeasureResult>, SimulationError> {
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    let statements = measurements_for_analysis(netlist, "NOISE");
    if statements.is_empty() {
        return Ok(Vec::new());
    }
    let series = match NoiseSweepSeries::from_sweep(sweep) {
        Ok(Some(series)) => series,
        Ok(None) => {
            return Ok(failed_measurements(
                &statements,
                "noise sweep produced no points",
            ));
        }
        Err(error) => return Err(error),
    };
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    let alias_projection = match InterfaceNodeAliasProjection::new_with_abort(
        netlist,
        OutputAnalysisKind::Noise,
        series.axis().len(),
        abort,
    ) {
        Ok(projection) => projection,
        Err(InterfaceNodeAliasProjectionError::Aborted) => {
            return Err(SimulationError::Aborted);
        }
        Err(InterfaceNodeAliasProjectionError::Detail(error)) => {
            return Ok(failed_measurements(&statements, &error));
        }
    };
    let mut signals = series.equation_signal_map();
    match alias_projection.augment_with_abort(&mut signals, abort) {
        Ok(()) => {}
        Err(InterfaceNodeAliasProjectionError::Aborted) => {
            return Err(SimulationError::Aborted);
        }
        Err(InterfaceNodeAliasProjectionError::Detail(error)) => {
            return Ok(failed_measurements(&statements, &error));
        }
    }
    // NOISE equations participate in the accepted-point stream just like AC
    // equations. A later WHEN/FIND-WHEN statement must see the equation's
    // current value as a waveform, rather than only its final scalar result.
    let equation_traces = match evaluate_equation_measurements_with_abort(
        netlist,
        "NOISE",
        series.axis(),
        &signals,
        -1.0,
        None,
        abort,
    ) {
        Ok(traces) => Ok(traces),
        Err(EquationMeasurementEvaluationError::Aborted) => {
            return Err(SimulationError::Aborted);
        }
        Err(EquationMeasurementEvaluationError::Detail(error)) => Err(error),
    };
    let mut results = match &equation_traces {
        Ok(traces) => evaluate_statements_with_equation_traces(
            &statements,
            series.axis(),
            &signals,
            &netlist.params,
            &[],
            traces,
            netlist.options.measure_default_value,
            -1.0,
            false,
        ),
        Err(_) => evaluate_statements(&statements, series.axis(), &signals, &netlist.params, false),
    };
    overlay_continuous_equation_results(&statements, &mut results, equation_traces, "NOISE");
    if abort.is_aborted() {
        Err(SimulationError::Aborted)
    } else {
        Ok(results)
    }
}

/// Evaluate vector-valued `.MEASURE NOISE_CONT` point-event statements.
///
/// The returned records retain all qualifying event rows; they are not
/// collapsed to the first scalar result as ordinary NOISE measurements are.
pub fn evaluate_noise_continuous_measurements(
    netlist: &Netlist,
    sweep: &[crate::analysis::NoiseResult],
) -> Vec<ContinuousMeasureResult> {
    let statements = measurements_for_analysis(netlist, "NOISE_CONT");
    if statements.is_empty() {
        return Vec::new();
    }
    let series = match NoiseSweepSeries::from_sweep(sweep) {
        Ok(Some(series)) => series,
        Ok(None) => {
            return statements
                .iter()
                .map(|statement| ContinuousMeasureResult {
                    name: statement.name.clone(),
                    records: Vec::new(),
                    failure: Some("noise sweep produced no points".to_string()),
                    failure_metadata: None,
                })
                .collect();
        }
        Err(error) => {
            return failed_continuous_measurements(&statements, &error.to_string());
        }
    };
    let alias_projection = match InterfaceNodeAliasProjection::new(
        netlist,
        OutputAnalysisKind::Noise,
        series.axis().len(),
    ) {
        Ok(projection) => projection,
        Err(error) => return failed_continuous_measurements(&statements, &error),
    };
    let mut signals = series.equation_signal_map();
    if let Err(error) = alias_projection.augment(&mut signals) {
        return failed_continuous_measurements(&statements, &error);
    }
    evaluate_continuous_statements(&statements, series.axis(), signals, &netlist.params, &[])
}

fn evaluate_continuous_statements(
    statements: &[&MeasureStatement],
    axis: &[Value],
    signals: HashMap<String, &[Value]>,
    params: &crate::netlist::ParamContext,
    segment_starts: &[usize],
) -> Vec<ContinuousMeasureResult> {
    let derived = materialize_measure_expression_signals(statements, axis, &signals, params);
    let mut augmented_signals = signals;
    for (name, waveform) in &derived {
        augmented_signals.insert(name.clone(), waveform.as_slice());
    }
    let mut engine = MeasureEngine::new();
    for statement in statements {
        engine.add((*statement).clone());
    }
    engine.evaluate_continuous(axis, &augmented_signals, segment_starts)
}

/// The netlist's measurement statements for one analysis kind
/// (`"TRAN"`, `"DC"`, `"AC"`, ...).
pub fn measurements_for_analysis<'a>(
    netlist: &'a Netlist,
    analysis: &str,
) -> Vec<&'a MeasureStatement> {
    netlist
        .measurements
        .iter()
        .filter(|m| m.analysis.eq_ignore_ascii_case(analysis))
        .collect()
}

fn failed_measurements(statements: &[&MeasureStatement], reason: &str) -> Vec<MeasureResult> {
    statements
        .iter()
        .map(|statement| MeasureResult::failed_for_statement(statement, reason))
        .collect()
}

fn failed_continuous_measurements(
    statements: &[&MeasureStatement],
    reason: &str,
) -> Vec<ContinuousMeasureResult> {
    statements
        .iter()
        .map(|statement| ContinuousMeasureResult {
            name: statement.name.clone(),
            records: Vec::new(),
            failure: Some(reason.to_string()),
            failure_metadata: None,
        })
        .collect()
}

fn evaluate_statements(
    statements: &[&MeasureStatement],
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    params: &crate::netlist::ParamContext,
    use_legacy_tran_trig_targ: bool,
) -> Vec<MeasureResult> {
    evaluate_statements_with_segment_starts(
        statements,
        axis,
        signals,
        params,
        &[],
        use_legacy_tran_trig_targ,
    )
}

fn evaluate_statements_with_segment_starts(
    statements: &[&MeasureStatement],
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    params: &crate::netlist::ParamContext,
    segment_starts: &[usize],
    use_legacy_tran_trig_targ: bool,
) -> Vec<MeasureResult> {
    let derived = materialize_measure_expression_signals(statements, axis, signals, params);
    let mut augmented_signals = signals.clone();
    for (name, waveform) in &derived {
        augmented_signals.insert(name.clone(), waveform.as_slice());
    }
    let mut engine = MeasureEngine::new();
    engine.set_use_legacy_tran_trig_targ(use_legacy_tran_trig_targ);
    for statement in statements {
        engine.add((*statement).clone());
    }
    engine.evaluate_with_segment_starts_and_context(
        axis,
        &augmented_signals,
        segment_starts,
        params,
    )
}

fn evaluate_statements_with_equation_traces(
    statements: &[&MeasureStatement],
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    params: &crate::netlist::ParamContext,
    segment_starts: &[usize],
    traces: &[EquationMeasureTrace],
    global_default: Option<Value>,
    equation_default: Value,
    use_legacy_tran_trig_targ: bool,
) -> Vec<MeasureResult> {
    let statement_dependencies = statements
        .iter()
        .map(|statement| statement_live_dependencies(statement, params))
        .collect::<Vec<_>>();
    let equation_positions = traces
        .iter()
        .map(|trace| {
            statements
                .iter()
                .position(|statement| statement.name.eq_ignore_ascii_case(&trace.name))
        })
        .collect::<Vec<_>>();
    let previous_values = traces
        .iter()
        .zip(&equation_positions)
        .map(|(trace, position)| {
            let statement = position.map(|position| statements[position]);
            let local_default = statement.and_then(|statement| statement.default_value);
            let implicit_default = statement.map_or(equation_default, |statement| {
                if matches!(statement.measure_type, MeasureType::Equation { .. }) {
                    equation_default
                } else {
                    0.0
                }
            });
            let default = global_default.or(local_default).unwrap_or(implicit_default);
            std::iter::once(default)
                .chain(
                    trace
                        .values
                        .iter()
                        .copied()
                        .take(axis.len().saturating_sub(1)),
                )
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Xyce updates every measurement sequentially at each accepted point. A
    // later consumer sees the producer's current-point value; a forward
    // consumer sees its previous-point value (or DEFAULT_VAL at row zero).
    let mut signal_maps = statements
        .iter()
        .enumerate()
        .map(|(statement_index, _)| {
            let mut map = signals.clone();
            for (trace_index, trace) in traces.iter().enumerate() {
                if !statement_dependencies[statement_index]
                    .contains(&trace.name.to_ascii_uppercase())
                {
                    continue;
                }
                let values = if equation_positions[trace_index]
                    .is_some_and(|equation_index| statement_index > equation_index)
                {
                    trace.values.as_slice()
                } else {
                    previous_values[trace_index].as_slice()
                };
                insert_case_variants(&mut map, &trace.name, values);
            }
            map
        })
        .collect::<Vec<_>>();

    let expression_signals = statements
        .iter()
        .zip(&signal_maps)
        .map(|(statement, map)| {
            materialize_measure_expression_signals(&[*statement], axis, map, params)
        })
        .collect::<Vec<_>>();
    for (map, derived) in signal_maps.iter_mut().zip(&expression_signals) {
        for (name, waveform) in derived {
            map.insert(name.clone(), waveform.as_slice());
        }
    }

    let mut engine = MeasureEngine::new();
    engine.set_use_legacy_tran_trig_targ(use_legacy_tran_trig_targ);
    for statement in statements {
        engine.add((*statement).clone());
    }
    engine.evaluate_with_segment_starts_and_signal_maps_and_context(
        axis,
        &signal_maps,
        segment_starts,
        params,
    )
}

fn statement_live_dependencies(
    statement: &MeasureStatement,
    params: &crate::netlist::ParamContext,
) -> HashSet<String> {
    let mut names = Vec::new();
    match &statement.measure_type {
        MeasureType::Delay { trig, targ, .. } => {
            for clause in [trig, targ] {
                if let TriggerEvent::When(when) = &clause.event {
                    add_live_condition_dependencies(when, &mut names, params);
                }
            }
        }
        MeasureType::Find { signal, when, .. } | MeasureType::Derivative { signal, when, .. } => {
            add_live_operand_dependencies(signal, &mut names, params);
            if let Some(when) = when {
                add_live_condition_dependencies(when, &mut names, params);
            }
        }
        MeasureType::When {
            condition: when, ..
        } => add_live_condition_dependencies(when, &mut names, params),
        MeasureType::Param { expression } | MeasureType::Equation { expression, .. } => {
            if let Ok((_, dependencies)) =
                compile_live_equation_source(expression, &statement.name, params)
            {
                names.extend(dependencies);
            }
        }
        MeasureType::ErrorFunction {
            measured,
            comparison,
            ..
        } => {
            add_live_operand_dependencies(measured, &mut names, params);
            add_live_operand_dependencies(comparison, &mut names, params);
        }
        MeasureType::FileError { signal, .. }
        | MeasureType::Min { signal, .. }
        | MeasureType::Max { signal, .. }
        | MeasureType::PeakToPeak { signal, .. }
        | MeasureType::Avg { signal, .. }
        | MeasureType::Rms { signal, .. }
        | MeasureType::RiseTime { signal, .. }
        | MeasureType::FallTime { signal, .. }
        | MeasureType::Integ { signal, .. } => {
            add_live_operand_dependencies(signal, &mut names, params)
        }
    }
    names.into_iter().collect()
}

fn add_live_operand_dependencies(
    authored: &str,
    names: &mut Vec<String>,
    params: &crate::netlist::ParamContext,
) {
    if let Ok(operand) = LiveMeasureOperand::compile(authored, params) {
        names.extend(operand.dependencies);
    } else {
        names.push(authored.to_ascii_uppercase());
    }
}

fn add_live_condition_dependencies(
    condition: &WhenCondition,
    names: &mut Vec<String>,
    params: &crate::netlist::ParamContext,
) {
    add_live_operand_dependencies(&condition.left, names, params);
    if let MeasureOperand::Waveform(right) = &condition.right {
        add_live_operand_dependencies(right, names, params);
    }
}

fn overlay_continuous_equation_results(
    statements: &[&MeasureStatement],
    results: &mut [MeasureResult],
    traces: Result<Vec<EquationMeasureTrace>, String>,
    analysis: &str,
) {
    match traces {
        Ok(traces) => {
            for (statement, result) in statements.iter().zip(results) {
                let equation = matches!(statement.measure_type, MeasureType::Equation { .. });
                let cached_file_error =
                    matches!(statement.measure_type, MeasureType::FileError { .. })
                        && traces.iter().any(|trace| {
                            trace.name.eq_ignore_ascii_case(&statement.name) && trace.initialized
                        });
                if !equation && !cached_file_error {
                    continue;
                }
                let Some(trace) = traces
                    .iter()
                    .find(|trace| trace.name.eq_ignore_ascii_case(&statement.name))
                else {
                    *result = MeasureResult::failed_for_statement(
                        statement,
                        &format!("continuous {analysis} equation trace was not produced"),
                    );
                    continue;
                };
                *result = if trace.initialized {
                    trace
                        .values
                        .last()
                        .copied()
                        .map(|value| MeasureResult::success(&statement.name, value))
                        .unwrap_or_else(|| {
                            MeasureResult::failed(
                                &statement.name,
                                &format!("continuous {analysis} equation trace is empty"),
                            )
                        })
                } else if equation {
                    MeasureResult::failed(
                        &statement.name,
                        &format!("continuous {analysis} equation window was never active"),
                    )
                } else {
                    continue;
                }
                .check_contract(statement);
            }
        }
        Err(err) => {
            for (statement, result) in statements.iter().zip(results) {
                if matches!(statement.measure_type, MeasureType::Equation { .. }) {
                    *result = MeasureResult::failed_for_statement(
                        statement,
                        &format!("continuous {analysis} equation evaluation failed: {err}"),
                    );
                }
            }
        }
    }
}

fn materialize_measure_expression_signals(
    statements: &[&MeasureStatement],
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    params: &crate::netlist::ParamContext,
) -> Vec<(String, Vec<Value>)> {
    let mut names = Vec::new();
    let mut add = |name: &str| {
        if name.starts_with('{')
            && name.ends_with('}')
            && !names.iter().any(|candidate| candidate == name)
        {
            names.push(name.to_string());
        }
    };
    for statement in statements {
        match &statement.measure_type {
            MeasureType::Delay { trig, targ, .. } => {
                for clause in [trig, targ] {
                    if let TriggerEvent::When(condition) = &clause.event {
                        add(&condition.left);
                        if let MeasureOperand::Waveform(right) = &condition.right {
                            add(right);
                        }
                    }
                }
            }
            MeasureType::Find { signal, when, .. }
            | MeasureType::Derivative { signal, when, .. } => {
                add(signal);
                if let Some(when) = when {
                    add(&when.left);
                    if let MeasureOperand::Waveform(right) = &when.right {
                        add(right);
                    }
                }
            }
            MeasureType::When { condition, .. } => {
                add(&condition.left);
                if let MeasureOperand::Waveform(right) = &condition.right {
                    add(right);
                }
            }
            MeasureType::ErrorFunction {
                measured,
                comparison,
                ..
            } => {
                add(measured);
                add(comparison);
            }
            MeasureType::FileError { signal, .. }
            | MeasureType::Min { signal, .. }
            | MeasureType::Max { signal, .. }
            | MeasureType::PeakToPeak { signal, .. }
            | MeasureType::Avg { signal, .. }
            | MeasureType::Rms { signal, .. }
            | MeasureType::RiseTime { signal, .. }
            | MeasureType::FallTime { signal, .. }
            | MeasureType::Integ { signal, .. } => add(signal),
            MeasureType::Param { .. } | MeasureType::Equation { .. } => {}
        }
    }

    let signal_index = CanonicalMeasureSignalIndex::new(signals);
    names
        .into_iter()
        .filter_map(|name| {
            let expression = name.strip_prefix('{')?.strip_suffix('}')?;
            let expression = crate::netlist::expr::parse_expression(expression).ok()?;
            let mut waveform = Vec::with_capacity(axis.len());
            let measures = HashMap::new();
            for row in 0..axis.len() {
                waveform.push(
                    evaluate_measure_expression(
                        &expression,
                        row,
                        &signal_index,
                        &measures,
                        params,
                        true,
                        &format!("expression '{name}'"),
                    )
                    .ok()?,
                );
            }
            Some((name, waveform))
        })
        .collect()
}

fn materialize_differential_voltage_signals(
    statements: &[&MeasureStatement],
    point_count: usize,
    signals: &HashMap<String, &[Value]>,
) -> Result<Vec<(String, Vec<Value>)>, String> {
    let mut names = Vec::new();
    let mut seen_names = HashSet::new();
    let mut add = |name: &str| {
        let trimmed = name.trim();
        if differential_voltage_nodes(trimmed).is_some()
            && seen_names.insert(canonical_measure_signal_name(trimmed))
        {
            names.push(trimmed.to_string());
        }
    };
    for statement in statements {
        match &statement.measure_type {
            MeasureType::Delay { trig, targ, .. } => {
                for clause in [trig, targ] {
                    if let TriggerEvent::When(condition) = &clause.event {
                        add(&condition.left);
                        if let MeasureOperand::Waveform(right) = &condition.right {
                            add(right);
                        }
                    }
                }
            }
            MeasureType::Find { signal, when, .. }
            | MeasureType::Derivative { signal, when, .. } => {
                add(signal);
                if let Some(when) = when {
                    add(&when.left);
                    if let MeasureOperand::Waveform(right) = &when.right {
                        add(right);
                    }
                }
            }
            MeasureType::When { condition, .. } => {
                add(&condition.left);
                if let MeasureOperand::Waveform(right) = &condition.right {
                    add(right);
                }
            }
            MeasureType::ErrorFunction {
                measured,
                comparison,
                ..
            } => {
                add(measured);
                add(comparison);
            }
            MeasureType::FileError { signal, .. }
            | MeasureType::Min { signal, .. }
            | MeasureType::Max { signal, .. }
            | MeasureType::PeakToPeak { signal, .. }
            | MeasureType::Avg { signal, .. }
            | MeasureType::Rms { signal, .. }
            | MeasureType::RiseTime { signal, .. }
            | MeasureType::FallTime { signal, .. }
            | MeasureType::Integ { signal, .. } => add(signal),
            MeasureType::Param { .. } | MeasureType::Equation { .. } => {}
        }
    }

    let signal_index = CanonicalMeasureSignalIndex::new(signals);
    let mut materialized = Vec::with_capacity(names.len());
    for name in names {
        let Some((positive, negative)) = differential_voltage_nodes(&name) else {
            continue;
        };
        let Some(positive) = measurement_node_waveform(positive, point_count, &signal_index)?
        else {
            continue;
        };
        let Some(negative) = measurement_node_waveform(negative, point_count, &signal_index)?
        else {
            continue;
        };
        let waveform = positive
            .iter()
            .zip(negative)
            .map(|(positive, negative)| positive - negative)
            .collect();
        materialized.push((name, waveform));
    }
    Ok(materialized)
}

fn differential_voltage_nodes(signal: &str) -> Option<(&str, &str)> {
    let (operator, arguments) = signal.split_once('(')?;
    if !operator.eq_ignore_ascii_case("V") {
        return None;
    }
    let arguments = arguments.strip_suffix(')')?;
    let (positive, negative) = arguments.split_once(',')?;
    let positive = positive.trim();
    let negative = negative.trim();
    (!positive.is_empty() && !negative.is_empty()).then_some((positive, negative))
}

fn measurement_node_waveform<'a>(
    node: &str,
    point_count: usize,
    signals: &CanonicalMeasureSignalIndex<'a>,
) -> Result<Option<Vec<Value>>, String> {
    if node == "0" {
        return Ok(Some(vec![0.0; point_count]));
    }
    let Some(waveform) = signals.get(node)? else {
        return Ok(None);
    };
    if waveform.len() != point_count {
        return Err(format!(
            "measurement node '{node}' has {} points, expected {point_count}",
            waveform.len()
        ));
    }
    Ok(Some(waveform.to_owned()))
}

/// Evaluate the netlist's transient .MEAS statements against a result.
///
/// Returns an empty vector when the netlist has no transient measurements.
pub fn evaluate_tran_measurements(
    netlist: &Netlist,
    result: &TransientResult,
) -> Vec<MeasureResult> {
    match evaluate_tran_measurements_with_abort(netlist, result, &NoAbort) {
        Ok(results) => results,
        Err(SimulationError::Aborted) => {
            unreachable!("NoAbort cannot cancel transient measurement projection")
        }
        Err(error) => {
            unreachable!("transient measurement projection failed unexpectedly: {error}")
        }
    }
}

/// Evaluate scalar transient measurements with cooperative cancellation.
pub fn evaluate_tran_measurements_with_abort(
    netlist: &Netlist,
    result: &TransientResult,
    abort: &dyn AbortSignal,
) -> Result<Vec<MeasureResult>, SimulationError> {
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    let signals = transient_signal_map(result);
    evaluate_tran_measurements_with_signals_and_abort(netlist, &result.time, &signals, abort)
}

/// Re-evaluate the netlist's transient `.MEAS` statements over a serialized
/// point stream, such as a PRN or CSV file read by a `-remeasure` workflow.
///
/// The caller owns column parsing and supplies each serialized variable under
/// its authored header name. The independent axis is authoritative: `TIME`
/// spellings in `signals` are replaced with `time`. Signal lookup retains the
/// same case-insensitive, hierarchy-aware, ambiguity-checking semantics used
/// for native transient results.
pub fn evaluate_tran_remeasurements(
    netlist: &Netlist,
    time: &[Value],
    signals: &HashMap<String, &[Value]>,
) -> Vec<MeasureResult> {
    let mut signals = signals.clone();
    insert_case_variants(&mut signals, "Time", time);
    if let Err(error) = augment_remeasure_voltage_spellings(&mut signals) {
        let statements = measurements_for_analysis(netlist, "TRAN");
        return failed_measurements(&statements, &error);
    }
    evaluate_tran_measurements_with_signals(netlist, time, &signals)
}

/// Xyce registers a serialized `V(node)` column as solution symbol `node`.
/// Retain both spellings so measurements resolve identically whether a replay
/// producer wrote the wrapped probe or the underlying solution name. Distinct
/// columns that describe the same voltage fail closed instead of depending on
/// input-column or hash iteration order.
fn augment_remeasure_voltage_spellings<'a>(
    signals: &mut HashMap<String, &'a [Value]>,
) -> Result<(), String> {
    let index = CanonicalMeasureSignalIndex::new(signals);
    let aliases = signals
        .iter()
        .filter_map(|(name, waveform)| {
            remeasure_voltage_alias(name).map(|alias| (alias, *waveform))
        })
        .collect::<Vec<_>>();

    for (alias, waveform) in aliases {
        if let Some(existing) = index.get(&alias)? {
            if existing.len() != waveform.len()
                || !std::ptr::eq(existing.as_ptr(), waveform.as_ptr())
            {
                return Err(format!(
                    "serialized voltage column '{alias}' conflicts with an equivalent column"
                ));
            }
        }
        insert_case_variants(signals, &alias, waveform);
    }
    Ok(())
}

fn remeasure_voltage_alias(name: &str) -> Option<String> {
    let name = name.trim();
    if name.is_empty() || name.eq_ignore_ascii_case("TIME") {
        return None;
    }

    if name.len() >= 4
        && name.as_bytes()[0].eq_ignore_ascii_case(&b'v')
        && name.as_bytes()[1] == b'('
        && name.ends_with(')')
    {
        let node = name[2..name.len() - 1].trim();
        if !node.is_empty() && !node.contains(',') {
            return Some(node.to_string());
        }
        return None;
    }

    (!name.contains(['(', ')', '{', '}', ','])).then(|| format!("V({name})"))
}

fn evaluate_tran_measurements_with_signals(
    netlist: &Netlist,
    time: &[Value],
    source_signals: &HashMap<String, &[Value]>,
) -> Vec<MeasureResult> {
    match evaluate_tran_measurements_with_signals_and_abort(netlist, time, source_signals, &NoAbort)
    {
        Ok(results) => results,
        Err(SimulationError::Aborted) => {
            unreachable!("NoAbort cannot cancel transient measurement projection")
        }
        Err(error) => {
            unreachable!("transient measurement projection failed unexpectedly: {error}")
        }
    }
}

fn evaluate_tran_measurements_with_signals_and_abort(
    netlist: &Netlist,
    time: &[Value],
    source_signals: &HashMap<String, &[Value]>,
    abort: &dyn AbortSignal,
) -> Result<Vec<MeasureResult>, SimulationError> {
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    let statements = measurements_for_analysis(netlist, "TRAN");
    if statements.is_empty() {
        return Ok(Vec::new());
    }
    let alias_projection = match InterfaceNodeAliasProjection::new_with_abort(
        netlist,
        OutputAnalysisKind::Tran,
        time.len(),
        abort,
    ) {
        Ok(projection) => projection,
        Err(InterfaceNodeAliasProjectionError::Aborted) => {
            return Err(SimulationError::Aborted);
        }
        Err(InterfaceNodeAliasProjectionError::Detail(error)) => {
            return Ok(failed_measurements(&statements, &error));
        }
    };
    // Reborrow caller-owned slices into a map whose lifetime is scoped to this
    // evaluation. Alias and differential projections own local waveforms, so
    // pinning the map to the caller's longer lifetime would make inserting
    // those safe local borrows impossible.
    let mut signals = source_signals
        .iter()
        .map(|(name, waveform)| (name.clone(), &**waveform))
        .collect::<HashMap<String, &[Value]>>();
    match alias_projection.augment_with_abort(&mut signals, abort) {
        Ok(()) => {}
        Err(InterfaceNodeAliasProjectionError::Aborted) => {
            return Err(SimulationError::Aborted);
        }
        Err(InterfaceNodeAliasProjectionError::Detail(error)) => {
            return Ok(failed_measurements(&statements, &error));
        }
    }
    let differential_signals =
        match materialize_differential_voltage_signals(&statements, time.len(), &signals) {
            Ok(signals) => signals,
            Err(error) => return Ok(failed_measurements(&statements, &error)),
        };
    for (name, waveform) in &differential_signals {
        insert_case_variants(&mut signals, name, waveform);
    }
    let live_traces = match evaluate_equation_measurements_with_abort(
        netlist, "TRAN", time, &signals, -1.0, None, abort,
    ) {
        Ok(traces) => Ok(traces),
        Err(EquationMeasurementEvaluationError::Aborted) => {
            return Err(SimulationError::Aborted);
        }
        Err(EquationMeasurementEvaluationError::Detail(error)) => Err(error),
    };
    let mut results = match &live_traces {
        Ok(traces) => evaluate_statements_with_equation_traces(
            &statements,
            time,
            &signals,
            &netlist.params,
            &[],
            traces,
            netlist.options.measure_default_value,
            -1.0,
            netlist.options.measure_use_lttm(),
        ),
        Err(_) => evaluate_statements(
            &statements,
            time,
            &signals,
            &netlist.params,
            netlist.options.measure_use_lttm(),
        ),
    };
    overlay_continuous_equation_results(&statements, &mut results, live_traces, "TRAN");
    if abort.is_aborted() {
        Err(SimulationError::Aborted)
    } else {
        Ok(results)
    }
}

/// Evaluate vector-valued `.MEASURE TRAN_CONT` point-event statements.
///
/// Every qualifying WHEN, FIND, DERIV, or TRIG/TARG event is retained with
/// its interpolated event metadata. A single transient run has one continuous
/// axis segment; stepped runs invoke this adapter independently per step.
pub fn evaluate_tran_continuous_measurements(
    netlist: &Netlist,
    result: &TransientResult,
) -> Vec<ContinuousMeasureResult> {
    let statements = measurements_for_analysis(netlist, "TRAN_CONT");
    if statements.is_empty() {
        return Vec::new();
    }
    if result.time.is_empty() {
        return statements
            .iter()
            .map(|statement| ContinuousMeasureResult {
                name: statement.name.clone(),
                records: Vec::new(),
                failure: Some("transient analysis produced no accepted points".to_string()),
                failure_metadata: None,
            })
            .collect();
    }
    let alias_projection = match InterfaceNodeAliasProjection::new(
        netlist,
        OutputAnalysisKind::Tran,
        result.time.len(),
    ) {
        Ok(projection) => projection,
        Err(error) => return failed_continuous_measurements(&statements, &error),
    };
    let mut signals = transient_signal_map(result);
    if let Err(error) = alias_projection.augment(&mut signals) {
        return failed_continuous_measurements(&statements, &error);
    }
    let differential_signals =
        match materialize_differential_voltage_signals(&statements, result.time.len(), &signals) {
            Ok(signals) => signals,
            Err(error) => return failed_continuous_measurements(&statements, &error),
        };
    for (name, waveform) in &differential_signals {
        insert_case_variants(&mut signals, name, waveform);
    }
    evaluate_continuous_statements(&statements, &result.time, signals, &netlist.params, &[])
}

/// Evaluate vector-valued `.MEASURE DC_CONT` point-event statements.
///
/// The sweep value is the event abscissa. Nested DC sweeps are divided at
/// primary-sweep restarts so no event is interpolated across the synthetic
/// jump between secondary-sweep cycles.
pub fn evaluate_dc_continuous_measurements(
    netlist: &Netlist,
    sweep: &[(Value, SimulationResult)],
) -> Vec<ContinuousMeasureResult> {
    evaluate_dc_continuous_measurements_with_parameter_contexts(netlist, sweep, &[])
}

/// Evaluate vector-valued `.MEASURE DC_CONT` statements with an optional
/// parameter context for every accepted sweep point.
///
/// Point-local contexts preserve `.DC DATA` semantics for expressions that
/// reference table-driven parameters or their dependent parameters.
pub fn evaluate_dc_continuous_measurements_with_parameter_contexts(
    netlist: &Netlist,
    sweep: &[(Value, SimulationResult)],
    point_params: &[crate::netlist::ParamContext],
) -> Vec<ContinuousMeasureResult> {
    let statements = measurements_for_analysis(netlist, "DC_CONT");
    if statements.is_empty() {
        return Vec::new();
    }
    let normalized_statements = statements
        .into_iter()
        .cloned()
        .map(normalize_dc_measurement_window)
        .collect::<Vec<_>>();
    let statements = normalized_statements.iter().collect::<Vec<_>>();
    let series = match DcSweepSeries::from_sweep(sweep) {
        Ok(Some(series)) => series,
        Err(error) => {
            return failed_continuous_measurements(&statements, &error.to_string());
        }
        Ok(None) => {
            return statements
                .iter()
                .map(|statement| ContinuousMeasureResult {
                    name: statement.name.clone(),
                    records: Vec::new(),
                    failure: Some("DC sweep produced no points".to_string()),
                    failure_metadata: None,
                })
                .collect();
        }
    };
    let alias_projection = match InterfaceNodeAliasProjection::new(
        netlist,
        OutputAnalysisKind::Dc,
        series.axis().len(),
    ) {
        Ok(projection) => projection,
        Err(error) => return failed_continuous_measurements(&statements, &error),
    };
    let mut signals = series.signal_map();
    if let Err(error) = alias_projection.augment(&mut signals) {
        return failed_continuous_measurements(&statements, &error);
    }
    let parameter_series = if point_params.is_empty() {
        Vec::new()
    } else if point_params.len() != series.axis().len() {
        return statements
            .iter()
            .map(|statement| ContinuousMeasureResult {
                name: statement.name.clone(),
                records: Vec::new(),
                failure: Some(
                    "DC point-parameter context count does not match sweep length".to_string(),
                ),
                failure_metadata: None,
            })
            .collect();
    } else {
        dc_parameter_context_series(point_params)
    };
    for (name, waveform) in &parameter_series {
        insert_case_variants(&mut signals, name, waveform);
    }
    let differential_signals = match materialize_differential_voltage_signals(
        &statements,
        series.axis().len(),
        &signals,
    ) {
        Ok(signals) => signals,
        Err(error) => return failed_continuous_measurements(&statements, &error),
    };
    for (name, waveform) in &differential_signals {
        insert_case_variants(&mut signals, name, waveform);
    }
    let segment_starts = dc_primary_segment_starts(netlist, series.axis().len());
    evaluate_continuous_statements(
        &statements,
        series.axis(),
        signals,
        &netlist.params,
        &segment_starts,
    )
}

/// Evaluate the netlist's DC .MEAS statements against a sweep.
///
/// Returns an empty vector when the netlist has no DC measurements; an empty
/// sweep fails every statement explicitly rather than skipping it.
pub fn evaluate_dc_measurements(
    netlist: &Netlist,
    sweep: &[(Value, SimulationResult)],
) -> Vec<MeasureResult> {
    match evaluate_dc_measurements_with_abort(netlist, sweep, &NoAbort) {
        Ok(results) => results,
        Err(SimulationError::Aborted) => {
            unreachable!("NoAbort cannot cancel DC measurement projection")
        }
        Err(error) => {
            let statements = measurements_for_analysis(netlist, "DC");
            failed_measurements(&statements, &error.to_string())
        }
    }
}

/// Evaluate scalar DC measurements with cooperative cancellation.
pub fn evaluate_dc_measurements_with_abort(
    netlist: &Netlist,
    sweep: &[(Value, SimulationResult)],
    abort: &dyn AbortSignal,
) -> Result<Vec<MeasureResult>, SimulationError> {
    evaluate_dc_measurements_with_parameter_contexts_and_abort(netlist, sweep, &[], abort)
}

/// Evaluate DC measurements with an optional parameter context for every
/// accepted point. This preserves `.DC DATA` semantics when table columns
/// change parameters (and dependent parameters) from row to row.
pub fn evaluate_dc_measurements_with_parameter_contexts(
    netlist: &Netlist,
    sweep: &[(Value, SimulationResult)],
    point_params: &[crate::netlist::ParamContext],
) -> Vec<MeasureResult> {
    match evaluate_dc_measurements_with_parameter_contexts_and_abort(
        netlist,
        sweep,
        point_params,
        &NoAbort,
    ) {
        Ok(results) => results,
        Err(SimulationError::Aborted) => {
            unreachable!("NoAbort cannot cancel DC measurement projection")
        }
        Err(error) => {
            let statements = measurements_for_analysis(netlist, "DC");
            failed_measurements(&statements, &error.to_string())
        }
    }
}

/// Evaluate DC measurements with point-local parameter contexts and
/// cooperative cancellation.
pub fn evaluate_dc_measurements_with_parameter_contexts_and_abort(
    netlist: &Netlist,
    sweep: &[(Value, SimulationResult)],
    point_params: &[crate::netlist::ParamContext],
    abort: &dyn AbortSignal,
) -> Result<Vec<MeasureResult>, SimulationError> {
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    let statements = measurements_for_analysis(netlist, "DC");
    if statements.is_empty() {
        return Ok(Vec::new());
    }
    let normalized_statements = statements
        .into_iter()
        .cloned()
        .map(normalize_dc_measurement_window)
        .collect::<Vec<_>>();
    let statements = normalized_statements.iter().collect::<Vec<_>>();
    let Some(series) = DcSweepSeries::from_sweep_with_abort(sweep, abort)? else {
        return Ok(failed_measurements(
            &statements,
            "DC sweep produced no points",
        ));
    };
    let alias_projection = match InterfaceNodeAliasProjection::new_with_abort(
        netlist,
        OutputAnalysisKind::Dc,
        series.axis().len(),
        abort,
    ) {
        Ok(projection) => projection,
        Err(InterfaceNodeAliasProjectionError::Aborted) => {
            return Err(SimulationError::Aborted);
        }
        Err(InterfaceNodeAliasProjectionError::Detail(error)) => {
            return Ok(failed_measurements(&statements, &error));
        }
    };
    let mut signals = series.signal_map();
    match alias_projection.augment_with_abort(&mut signals, abort) {
        Ok(()) => {}
        Err(InterfaceNodeAliasProjectionError::Aborted) => {
            return Err(SimulationError::Aborted);
        }
        Err(InterfaceNodeAliasProjectionError::Detail(error)) => {
            return Ok(failed_measurements(&statements, &error));
        }
    }
    let parameter_series = if point_params.is_empty() {
        Vec::new()
    } else if point_params.len() != series.axis().len() {
        return Ok(failed_measurements(
            &statements,
            "DC point-parameter context count does not match sweep length",
        ));
    } else {
        dc_parameter_context_series(point_params)
    };
    for (name, waveform) in &parameter_series {
        insert_case_variants(&mut signals, name, waveform);
    }
    let differential_signals = match materialize_differential_voltage_signals(
        &statements,
        series.axis().len(),
        &signals,
    ) {
        Ok(signals) => signals,
        Err(error) => return Ok(failed_measurements(&statements, &error)),
    };
    for (name, waveform) in &differential_signals {
        insert_case_variants(&mut signals, name, waveform);
    }
    // Continuous equation measures are live waveforms, not merely final
    // scalars. Their visibility at each statement depends on netlist order.
    let equation_traces = match evaluate_equation_measurements_with_abort(
        netlist,
        "DC",
        series.axis(),
        &signals,
        0.0,
        Some(dc_primary_sweep_is_ascending(netlist, series.axis())),
        abort,
    ) {
        Ok(traces) => Ok(traces),
        Err(EquationMeasurementEvaluationError::Aborted) => {
            return Err(SimulationError::Aborted);
        }
        Err(EquationMeasurementEvaluationError::Detail(error)) => Err(error),
    };
    let segment_starts = dc_primary_segment_starts(netlist, series.axis().len());
    let mut results = match &equation_traces {
        Ok(traces) => evaluate_statements_with_equation_traces(
            &statements,
            series.axis(),
            &signals,
            &netlist.params,
            &segment_starts,
            traces,
            netlist.options.measure_default_value,
            0.0,
            false,
        ),
        Err(_) => evaluate_statements_with_segment_starts(
            &statements,
            series.axis(),
            &signals,
            &netlist.params,
            &segment_starts,
            false,
        ),
    };
    overlay_continuous_equation_results(&statements, &mut results, equation_traces, "DC");
    if abort.is_aborted() {
        Err(SimulationError::Aborted)
    } else {
        Ok(results)
    }
}

fn dc_parameter_context_series(
    point_params: &[crate::netlist::ParamContext],
) -> Vec<(String, Vec<Value>)> {
    let mut names = std::collections::BTreeSet::new();
    for context in point_params {
        names.extend(
            context
                .numeric_parameters()
                .into_iter()
                .map(|(name, _)| name),
        );
    }
    names
        .into_iter()
        .filter_map(|name| {
            point_params
                .iter()
                .map(|context| context.get(&name))
                .collect::<Option<Vec<_>>>()
                .map(|values| (name, values))
        })
        .collect()
}

fn dc_primary_segment_starts(netlist: &Netlist, point_count: usize) -> Vec<usize> {
    let primary_point_counts = netlist.analyses.iter().filter_map(|analysis| {
        let crate::netlist::AnalysisCommand::Dc {
            start,
            stop,
            step,
            mode,
            sweep2: Some(sweep2),
            ..
        } = analysis
        else {
            return None;
        };
        let primary_points = crate::netlist::DcSweepSpec {
            start: *start,
            stop: *stop,
            step: *step,
            mode: mode.clone(),
        }
        .points()
        .len();
        let secondary_points = sweep2.spec().points().len();
        (primary_points > 0
            && secondary_points > 0
            && primary_points.checked_mul(secondary_points) == Some(point_count))
        .then_some(primary_points)
    });
    let mut matching_primary_points = primary_point_counts.collect::<Vec<_>>();
    matching_primary_points.sort_unstable();
    matching_primary_points.dedup();
    let [primary_points] = matching_primary_points.as_slice() else {
        return Vec::new();
    };
    (*primary_points..point_count)
        .step_by(*primary_points)
        .collect()
}

fn normalize_dc_measurement_window(mut statement: MeasureStatement) -> MeasureStatement {
    let bounds = match &mut statement.measure_type {
        MeasureType::Equation { from, to, .. }
        | MeasureType::Min { from, to, .. }
        | MeasureType::Max { from, to, .. }
        | MeasureType::PeakToPeak { from, to, .. }
        | MeasureType::Avg { from, to, .. }
        | MeasureType::Rms { from, to, .. }
        | MeasureType::Find { from, to, .. }
        | MeasureType::Derivative { from, to, .. }
        | MeasureType::When { from, to, .. }
        | MeasureType::ErrorFunction { from, to, .. } => Some((from, to)),
        MeasureType::Delay { .. }
        | MeasureType::FileError { .. }
        | MeasureType::Param { .. }
        | MeasureType::RiseTime { .. }
        | MeasureType::FallTime { .. }
        | MeasureType::Integ { .. } => None,
    };
    if let Some((Some(from), Some(to))) = bounds {
        if *from > *to {
            std::mem::swap(from, to);
        }
    }
    statement
}

/// Evaluate vector-valued `.MEASURE AC_CONT` point-event statements.
///
/// Complex probes use the same canonical projections as scalar AC measures:
/// bare `V()`/`I()` select the real component, with the explicit `VM`/`IM`,
/// `VR`/`IR`, `VI`/`II`, `VP`/`IP`, and `VDB`/`IDB` accessors available for
/// magnitude, real, imaginary, phase, and decibel projections.
pub fn evaluate_ac_continuous_measurements(
    netlist: &Netlist,
    sweep: &[AcResult],
) -> Vec<ContinuousMeasureResult> {
    let statements = measurements_for_analysis(netlist, "AC_CONT");
    if statements.is_empty() {
        return Vec::new();
    }
    let series = match AcSweepSeries::from_sweep(sweep) {
        Ok(Some(series)) => series,
        Err(error) => {
            return failed_continuous_measurements(&statements, &error.to_string());
        }
        Ok(None) => {
            return statements
                .iter()
                .map(|statement| ContinuousMeasureResult {
                    name: statement.name.clone(),
                    records: Vec::new(),
                    failure: Some("AC sweep produced no points".to_string()),
                    failure_metadata: None,
                })
                .collect();
        }
    };
    let alias_projection = match InterfaceNodeAliasProjection::new(
        netlist,
        OutputAnalysisKind::Ac,
        series.axis().len(),
    ) {
        Ok(projection) => projection,
        Err(error) => return failed_continuous_measurements(&statements, &error),
    };
    let mut signals = series.equation_signal_map();
    if let Err(error) = alias_projection.augment(&mut signals) {
        return failed_continuous_measurements(&statements, &error);
    }
    evaluate_continuous_statements(&statements, series.axis(), signals, &netlist.params, &[])
}

/// Evaluate the netlist's AC .MEAS statements against a sweep.
///
/// Returns an empty vector when the netlist has no AC measurements; an
/// empty sweep fails every statement explicitly rather than skipping it.
/// Xyce scalar AC semantics project bare `V(x)`/`I(x)` to the real component;
/// `VM`/`IM`, `VDB`/`IDB`, `VP`/`IP` (degrees), `VR`/`IR`, and `VI`/`II`
/// select the explicit derived quantities. The frequency axis is available as
/// `TIME`/`FREQUENCY`/`FREQ`/`HERTZ`.
pub fn evaluate_ac_measurements(netlist: &Netlist, sweep: &[AcResult]) -> Vec<MeasureResult> {
    match evaluate_ac_measurements_with_abort(netlist, sweep, &NoAbort) {
        Ok(results) => results,
        Err(SimulationError::Aborted) => {
            unreachable!("NoAbort cannot cancel AC measurement projection")
        }
        Err(error) => {
            let statements = measurements_for_analysis(netlist, "AC");
            failed_measurements(&statements, &error.to_string())
        }
    }
}

/// Evaluate scalar AC measurements with cooperative cancellation.
pub fn evaluate_ac_measurements_with_abort(
    netlist: &Netlist,
    sweep: &[AcResult],
    abort: &dyn AbortSignal,
) -> Result<Vec<MeasureResult>, SimulationError> {
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    let statements = measurements_for_analysis(netlist, "AC");
    if statements.is_empty() {
        return Ok(Vec::new());
    }
    let Some(series) = AcSweepSeries::from_sweep_with_abort(sweep, abort)? else {
        return Ok(failed_measurements(
            &statements,
            "AC sweep produced no points",
        ));
    };
    let alias_projection = match InterfaceNodeAliasProjection::new_with_abort(
        netlist,
        OutputAnalysisKind::Ac,
        series.axis().len(),
        abort,
    ) {
        Ok(projection) => projection,
        Err(InterfaceNodeAliasProjectionError::Aborted) => {
            return Err(SimulationError::Aborted);
        }
        Err(InterfaceNodeAliasProjectionError::Detail(error)) => {
            return Ok(failed_measurements(&statements, &error));
        }
    };
    let mut signals = series.equation_signal_map();
    match alias_projection.augment_with_abort(&mut signals, abort) {
        Ok(()) => {}
        Err(InterfaceNodeAliasProjectionError::Aborted) => {
            return Err(SimulationError::Aborted);
        }
        Err(InterfaceNodeAliasProjectionError::Detail(error)) => {
            return Ok(failed_measurements(&statements, &error));
        }
    }
    // Continuous equation measures participate in the accepted-point stream.
    // A later WHEN/FIND-WHEN statement therefore observes the equation's
    // current value as a waveform, not only its final scalar result.
    let equation_traces = match evaluate_equation_measurements_with_abort(
        netlist,
        "AC",
        series.axis(),
        &signals,
        -1.0,
        None,
        abort,
    ) {
        Ok(traces) => Ok(traces),
        Err(EquationMeasurementEvaluationError::Aborted) => {
            return Err(SimulationError::Aborted);
        }
        Err(EquationMeasurementEvaluationError::Detail(error)) => Err(error),
    };
    let mut results = match &equation_traces {
        Ok(traces) => evaluate_statements_with_equation_traces(
            &statements,
            series.axis(),
            &signals,
            &netlist.params,
            &[],
            traces,
            netlist.options.measure_default_value,
            -1.0,
            false,
        ),
        Err(_) => evaluate_statements(&statements, series.axis(), &signals, &netlist.params, false),
    };
    overlay_continuous_equation_results(&statements, &mut results, equation_traces, "AC");
    if abort.is_aborted() {
        Err(SimulationError::Aborted)
    } else {
        Ok(results)
    }
}

/// Explicit not-evaluated entries for measurements whose analysis did not
/// run (or is not supported by the caller), so automation fails loudly
/// instead of silently skipping checks.
pub fn unevaluated_measurements(
    netlist: &Netlist,
    analysis: &str,
    reason: &str,
) -> Vec<MeasureResult> {
    measurements_for_analysis(netlist, analysis)
        .iter()
        .map(|m| MeasureResult::failed_for_statement(m, reason))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn xyce_netlist(source: &str) -> Netlist {
        Netlist::parse_with_options(
            source,
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("Xyce test netlist parses")
    }

    #[test]
    fn unevaluated_scalar_measurement_retains_authored_contract_metadata() {
        let netlist = xyce_netlist(
            "unevaluated measurement contract\n\
             V1 out 0 1\n\
             .tran 1n 2n\n\
             .measure tran sample MAX V(out) GOAL=3 FAILVALUE=5\n\
             .end\n",
        );

        let results = unevaluated_measurements(&netlist, "TRAN", "analysis did not run");

        assert_eq!(results.len(), 1);
        let result = &results[0];
        assert!(!result.passed);
        assert_eq!(result.value, None);
        assert_eq!(result.raw_value, None);
        assert_eq!(result.expected, Some(3.0));
        assert_eq!(result.tolerance, Some(0.03));
        assert_eq!(result.failure_limit, Some(5.0));
        assert!(!result.failure_limit_exceeded);
        assert_eq!(result.error.as_deref(), Some("analysis did not run"));
    }

    #[test]
    fn harmonic_balance_measurement_signal_domain_is_complex() {
        assert_eq!(
            MeasurementSignalDomain::for_analysis(OutputAnalysisKind::Hb),
            Ok(MeasurementSignalDomain::Complex)
        );
    }

    #[test]
    fn standard_scalar_measurement_projections_propagate_typed_abort() {
        let netlist = Netlist::parse(
            "measurement cancellation\n\
             V1 out 0 0\n\
             .TRAN 1 2\n\
             .AC LIN 2 1 2\n\
             .NOISE V(out) V1 LIN 2 1 2\n\
             .MEASURE TRAN mt MAX V(out)\n\
             .MEASURE DC md MAX V(out)\n\
             .MEASURE AC ma MAX VM(out)\n\
             .MEASURE NOISE mn MAX ONOISE\n\
             .END\n",
        )
        .expect("measurement cancellation deck parses");

        let tran_abort = crate::abort_signal::CountingAbort::new(0);
        assert!(matches!(
            evaluate_tran_measurements_with_abort(&netlist, &tran_result(), &tran_abort),
            Err(SimulationError::Aborted)
        ));
        assert_eq!(tran_abort.count(), 1);

        let dc_abort = crate::abort_signal::CountingAbort::new(0);
        assert!(matches!(
            evaluate_dc_measurements_with_abort(&netlist, &[], &dc_abort),
            Err(SimulationError::Aborted)
        ));
        assert_eq!(dc_abort.count(), 1);

        let ac_abort = crate::abort_signal::CountingAbort::new(0);
        assert!(matches!(
            evaluate_ac_measurements_with_abort(&netlist, &[], &ac_abort),
            Err(SimulationError::Aborted)
        ));
        assert_eq!(ac_abort.count(), 1);

        let noise_abort = crate::abort_signal::CountingAbort::new(0);
        assert!(matches!(
            evaluate_noise_measurements_with_abort(&netlist, &[], &noise_abort),
            Err(SimulationError::Aborted)
        ));
        assert_eq!(noise_abort.count(), 1);
    }

    #[test]
    fn transient_equation_projection_polls_abort_at_bounded_row_intervals() {
        let netlist = Netlist::parse(
            "long measurement projection\n\
             V1 out 0 0\n\
             .TRAN 1 4095\n\
             .MEASURE TRAN tracked EQN {V(out)}\n\
             .END\n",
        )
        .expect("long measurement projection deck parses");
        let point_count = 4096;
        let result = tran_waveform(
            (0..point_count).map(|index| index as Value).collect(),
            (0..point_count)
                .map(|index| (index as Value).sin())
                .collect(),
        );
        let abort = crate::abort_signal::CountingAbort::new(20);
        let error = evaluate_tran_measurements_with_abort(&netlist, &result, &abort)
            .expect_err("equation projection must honor cancellation");
        assert!(matches!(error, SimulationError::Aborted));
        assert!(
            abort.count() <= 22,
            "measurement projection polled {} times after its deterministic threshold",
            abort.count()
        );
    }

    fn tran_result() -> TransientResult {
        TransientResult {
            time: vec![0.0, 1.0, 2.0, 3.0],
            step_sizes: vec![0.0; 4],
            voltages: vec![vec![0.0, 1.0, 2.0, 3.0]],
            branch_currents: vec![vec![0.0, -1.0, -2.0, -3.0]],
            num_nodes: 1,
            node_names: vec!["out".to_string()],
            branch_names: vec!["v1".to_string()],
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        }
    }

    fn tran_waveform(time: Vec<Value>, voltage: Vec<Value>) -> TransientResult {
        assert_eq!(time.len(), voltage.len());
        TransientResult {
            step_sizes: vec![0.0; time.len()],
            time,
            voltages: vec![voltage],
            branch_currents: Vec::new(),
            num_nodes: 1,
            node_names: vec!["out".to_string()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        }
    }

    #[test]
    fn transient_map_exposes_time_voltage_and_current() {
        let result = tran_result();
        let signals = transient_signal_map(&result);
        assert!(signals.contains_key("TIME"));
        assert!(signals.contains_key("V(out)"));
        assert!(signals.contains_key("v(OUT)"));
        assert!(signals.contains_key("I(v1)"));
        assert_eq!(signals["TIME"], result.time.as_slice());
    }

    #[test]
    fn transient_output_operands_preserve_authored_order_and_expressions() {
        let netlist = Netlist::parse(
            "transient output operands\n\
             V1 out 0 0\n\
             R1 out 0 1k\n\
             .TRAN 1 3\n\
             .PRINT TRAN I ( V1 ) V ( out , 0 ) IR(V1) {V(out)-I(V1)} V(out)\n\
             .END\n",
        )
        .expect("transient output deck parses");
        let projected = evaluate_tran_output_requests(&netlist, &tran_result())
            .expect("transient operands evaluate");
        assert_eq!(
            projected
                .iter()
                .map(|column| column.name.as_str())
                .collect::<Vec<_>>(),
            [
                "I ( V1 )",
                "V ( out , 0 )",
                "IR(V1)",
                "{V(out)-I(V1)}",
                "V(out)"
            ]
        );
        assert_eq!(projected[0].kind, OutputColumnKind::Current);
        assert_eq!(projected[1].kind, OutputColumnKind::Voltage);
        assert_eq!(projected[2].kind, OutputColumnKind::Current);
        assert_eq!(projected[3].kind, OutputColumnKind::Scalar);
        assert_eq!(projected[0].values, vec![0.0, -1.0, -2.0, -3.0]);
        assert_eq!(projected[1].values, vec![0.0, 1.0, 2.0, 3.0]);
        assert_eq!(projected[2].values, projected[0].values);
        assert_eq!(projected[3].values, vec![0.0, 2.0, 4.0, 6.0]);
        assert_eq!(projected[4].values, projected[1].values);
    }

    #[test]
    fn complete_voltage_wildcard_expands_in_node_order_and_preserves_explicit_probes() {
        let netlist = xyce_netlist(
            "complete voltage wildcard\n\
             V1 Zed 0 0\n\
             R1 Zed alpha 1k\n\
             R2 alpha Top.Dot 1k\n\
             R3 Top.Dot 0 1k\n\
             X1 alpha CELL\n\
             .SUBCKT CELL in\n\
             R4 in Inner 1k\n\
             R5 Inner 0 1k\n\
             .ENDS CELL\n\
             .TRAN 1 3\n\
             .PRINT TRAN V(alpha) V(*) V(Zed)\n\
             .END\n",
        );
        let mut result = tran_result();
        result.num_nodes = 5;
        result.node_names = vec![
            "Zed".into(),
            "alpha".into(),
            "M1.__dint".into(),
            "Top.Dot".into(),
            "X1.Inner".into(),
        ];
        result.voltages = vec![
            vec![0.0, 1.0, 2.0, 3.0],
            vec![10.0, 11.0, 12.0, 13.0],
            vec![30.0, 31.0, 32.0, 33.0],
            vec![20.0, 21.0, 22.0, 23.0],
            vec![40.0, 41.0, 42.0, 43.0],
        ];

        let projected = evaluate_tran_output_requests(&netlist, &result)
            .expect("V(*) expands to ordinary voltage columns");
        assert_eq!(
            projected
                .iter()
                .map(|column| column.name.as_str())
                .collect::<Vec<_>>(),
            [
                "V(alpha)",
                "V(ZED)",
                "V(ALPHA)",
                "V(TOP.DOT)",
                "V(X1:INNER)",
                "V(Zed)"
            ]
        );
        assert!(
            projected
                .iter()
                .all(|column| column.kind == OutputColumnKind::Voltage)
        );
        assert_eq!(projected[0].values, result.voltages[1]);
        assert_eq!(projected[1].values, result.voltages[0]);
        assert_eq!(projected[2].values, result.voltages[1]);
        assert_eq!(projected[3].values, result.voltages[3]);
        assert_eq!(projected[4].values, result.voltages[4]);
        assert_eq!(projected[5].values, result.voltages[0]);
    }

    #[test]
    fn xyce_complete_voltage_wildcard_counts_columns_without_claiming_partial_patterns() {
        let netlist = xyce_netlist(
            "bounded voltage wildcard\n\
             V1 a 0 0\n\
             R1 a b 1k\n\
             R2 b 0 1k\n\
             .TRAN 1 3\n\
             .PRINT TRAN V(*)\n\
             .END\n",
        );
        let mut result = tran_result();
        result.num_nodes = 2;
        result.node_names = vec!["a".into(), "b".into()];
        result.voltages = vec![vec![0.0; 4], vec![1.0; 4]];
        let mut limits = ResourceLimits::default();
        limits.max_result_values = 11;
        let error = evaluate_tran_output_requests_with_abort(&netlist, &result, limits, &NoAbort)
            .expect_err("axis plus two expanded columns require twelve values");
        assert!(matches!(
            error,
            SimulationError::ResourceLimit(ResourceLimitError {
                resource: ResourceKind::ResultValues,
                requested: 12,
                limit: 11,
            })
        ));

        for unsupported in ["V(a*)", "V(*,a)"] {
            let unsupported_netlist = xyce_netlist(&format!(
                "future voltage wildcard\nV1 a 0 0\n.TRAN 1 3\n.PRINT TRAN {unsupported}\n.END\n"
            ));
            let error = evaluate_tran_output_requests(&unsupported_netlist, &result)
                .expect_err("unimplemented pattern must remain fail-closed at projection");
            assert!(matches!(
                error,
                OutputProjectionError::Operand {
                    analysis: OutputAnalysisKind::Tran,
                    operand_index: 0,
                    detail,
                    ..
                } if detail.contains("unavailable")
            ));
        }

        let ngspice =
            Netlist::parse("ngspice literal star\nV1 a 0 0\n.TRAN 1 3\n.PRINT TRAN V(*)\n.END\n")
                .expect("ngspice literal-star deck parses");
        let error = evaluate_tran_output_requests(&ngspice, &result)
            .expect_err("Xyce V(*) expansion must not leak into ngspice mode");
        assert!(matches!(
            error,
            OutputProjectionError::Operand { detail, .. } if detail.contains("unavailable")
        ));

        let empty_result = TransientResult {
            time: result.time.clone(),
            step_sizes: result.step_sizes.clone(),
            voltages: Vec::new(),
            branch_currents: Vec::new(),
            num_nodes: 0,
            node_names: Vec::new(),
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        };
        assert!(
            evaluate_tran_output_requests(&netlist, &empty_result)
                .expect("Xyce empty wildcard expansion succeeds")
                .is_empty(),
            "an empty Xyce wildcard expansion must omit columns"
        );
    }

    #[test]
    fn dc_complete_voltage_wildcard_excludes_ground_and_uses_first_row_node_order() {
        let netlist = xyce_netlist(
            "DC voltage wildcard\n\
             V1 Zed 0 0\n\
             R1 Zed alpha 1k\n\
             R2 alpha 0 1k\n\
             .DC V1 0 1 1\n\
             .PRINT DC V(*)\n\
             .END\n",
        );
        let point = |names: Vec<&str>, values: Vec<Value>| {
            let mut result = SimulationResult::new(2, 0);
            result.node_names = names.into_iter().map(str::to_string).collect();
            result.node_voltages = values;
            result
        };
        let sweep = vec![
            (0.0, point(vec!["0", "Zed", "alpha"], vec![0.0, 1.0, 2.0])),
            (1.0, point(vec!["0", "alpha", "Zed"], vec![0.0, 20.0, 10.0])),
        ];
        let projected = evaluate_dc_output_requests(&netlist, &sweep)
            .expect("DC V(*) expands and aligns later rows by name");
        assert_eq!(
            projected
                .iter()
                .map(|column| column.name.as_str())
                .collect::<Vec<_>>(),
            ["V(ZED)", "V(ALPHA)"]
        );
        assert_eq!(projected[0].values, vec![1.0, 10.0]);
        assert_eq!(projected[1].values, vec![2.0, 20.0]);
    }

    #[test]
    fn dc_output_operands_include_named_device_observables() {
        let netlist = Netlist::parse(
            "DC output operands\n\
             V1 out 0 0\n\
             R1 out 0 1k\n\
             .DC V1 0 1 1\n\
             .PRINT DC R1:R I(V1) {V(out)/I(V1)}\n\
             .END\n",
        )
        .expect("DC output deck parses");
        let point = |voltage: Value, current: Value, resistance: Value| {
            let mut result = SimulationResult::new(1, 1);
            result.node_names = vec!["0".to_string(), "out".to_string()];
            result.node_voltages = vec![0.0, voltage];
            result.branch_names = vec!["V1".to_string()];
            result.branch_currents = vec![current];
            result.push_dc_observable("R1:R".to_string(), resistance);
            result
        };
        let sweep = vec![
            (0.0, point(0.0, -1.0, 1_000.0)),
            (1.0, point(1.0, -2.0, 2_000.0)),
        ];
        let projected =
            evaluate_dc_output_requests(&netlist, &sweep).expect("DC operands evaluate");
        assert_eq!(projected[0].values, vec![1_000.0, 2_000.0]);
        assert_eq!(projected[1].values, vec![-1.0, -2.0]);
        assert_eq!(projected[2].values, vec![0.0, -0.5]);
    }

    #[test]
    fn ordered_output_projection_enforces_abort_and_result_limits() {
        let netlist = Netlist::parse(
            "bounded output projection\n\
             V1 out 0 0\n\
             .TRAN 1 3\n\
             .PRINT TRAN V(out) {V(out)*2}\n\
             .END\n",
        )
        .expect("bounded output deck parses");
        let mut limits = ResourceLimits::default();
        limits.max_result_values = 11;
        let error =
            evaluate_tran_output_requests_with_abort(&netlist, &tran_result(), limits, &NoAbort)
                .expect_err("axis plus two columns require twelve values");
        assert!(matches!(
            error,
            SimulationError::ResourceLimit(ResourceLimitError {
                resource: ResourceKind::ResultValues,
                requested: 12,
                limit: 11,
            })
        ));

        let error = evaluate_tran_output_requests_with_abort(
            &netlist,
            &tran_result(),
            ResourceLimits::default(),
            &crate::abort_signal::ImmediateAbort,
        )
        .expect_err("pre-aborted projection must stop");
        assert!(matches!(error, SimulationError::Aborted));

        let counting_abort = crate::abort_signal::CountingAbort::new(3);
        let error = evaluate_tran_output_requests_with_abort(
            &netlist,
            &tran_result(),
            ResourceLimits::default(),
            &counting_abort,
        )
        .expect_err("interface-alias setup must poll cancellation");
        assert!(matches!(error, SimulationError::Aborted));
        assert!(counting_abort.count() >= 4);

        let mut misaligned = netlist.clone();
        misaligned.output_requests[0]
            .operands
            .push("V(out)".to_string());
        let error = evaluate_tran_output_requests(&misaligned, &tran_result())
            .expect_err("public AST edits cannot desynchronize typed output metadata");
        assert!(matches!(
            error,
            OutputProjectionError::Operand { detail, .. }
                if detail.contains("does not match typed operand count")
        ));
    }

    #[test]
    fn dc_output_projection_bounds_and_cancels_series_materialization() {
        let netlist = Netlist::parse(
            "bounded DC projection\n\
             V1 out 0 0\n\
             R1 out 0 1k\n\
             .DC V1 0 3 1\n\
             .PRINT DC V(out)\n\
             .END\n",
        )
        .expect("bounded DC deck parses");
        let point = |value: Value| {
            let mut result = SimulationResult::new(2, 1);
            result.node_names = vec!["0".into(), "out".into(), "extra".into()];
            result.node_voltages = vec![0.0, value, value * 2.0];
            result.branch_names = vec!["V1".into()];
            result.branch_currents = vec![-value];
            result
        };
        let sweep = (0..4)
            .map(|value| (Value::from(value), point(Value::from(value))))
            .collect::<Vec<_>>();

        let mut limits = ResourceLimits::default();
        limits.max_result_values = 8;
        let error = evaluate_dc_output_requests_with_abort(&netlist, &sweep, limits, &NoAbort)
            .expect_err("intermediate DC projection storage must share the result-value budget");
        assert!(matches!(
            error,
            SimulationError::ResourceLimit(ResourceLimitError {
                resource: ResourceKind::ResultValues,
                requested: 12,
                limit: 8,
            })
        ));

        let counting_abort = crate::abort_signal::CountingAbort::new(4);
        let error = evaluate_dc_output_requests_with_abort(
            &netlist,
            &sweep,
            ResourceLimits::default(),
            &counting_abort,
        )
        .expect_err("DC series materialization must poll cancellation");
        assert!(matches!(error, SimulationError::Aborted));
        assert!(counting_abort.count() >= 5);
    }

    #[test]
    fn dc_output_projection_resolves_each_row_by_canonical_name() {
        let netlist = Netlist::parse(
            "name-aligned DC output\n\
             V1 a 0 0\n\
             R1 a b 1k\n\
             R2 b 0 1k\n\
             .DC V1 0 1 1\n\
             .PRINT DC V(a) V(b)\n\
             .END\n",
        )
        .expect("name-aligned output deck parses");
        let mut first = SimulationResult::new(2, 0);
        first.node_names = vec!["0".into(), "a".into(), "b".into()];
        first.node_voltages = vec![0.0, 1.0, 2.0];
        let mut second = SimulationResult::new(2, 0);
        second.node_names = vec!["0".into(), "b".into(), "a".into()];
        second.node_voltages = vec![0.0, 20.0, 10.0];
        let projected =
            evaluate_dc_output_requests(&netlist, &[(0.0, first.clone()), (1.0, second)])
                .expect("reordered rows project by name");
        assert_eq!(projected[0].values, vec![1.0, 10.0]);
        assert_eq!(projected[1].values, vec![2.0, 20.0]);

        let mut missing = first;
        missing.node_names = vec!["0".into(), "b".into()];
        missing.node_voltages = vec![0.0, 20.0];
        let error = evaluate_dc_output_requests(&netlist, &[(0.0, missing)])
            .expect_err("missing requested node must fail closed");
        assert!(matches!(
            error,
            OutputProjectionError::Operand {
                analysis: OutputAnalysisKind::Dc,
                operand_index: 0,
                ..
            }
        ));
    }

    #[test]
    fn transient_remeasurement_uses_the_serialized_point_stream() {
        let netlist = Netlist::parse(
            "serialized transient measurement\n\
             .MEASURE TRAN ERR1MV1.5 ERR1 V(1) V(2) MINVAL=1.5\n\
             .END\n",
        )
        .expect("remeasure deck parses");
        let time = (0..=10)
            .map(|index| index as Value * 0.1)
            .collect::<Vec<_>>();
        let first = time.iter().map(|time| 5.0 * time).collect::<Vec<_>>();
        let second = time.iter().map(|time| 3.75 * time).collect::<Vec<_>>();
        let authored_time = vec![99.0; time.len()];
        let signals = HashMap::from([
            ("TIME".to_string(), authored_time.as_slice()),
            ("V(1)".to_string(), first.as_slice()),
            ("V(2)".to_string(), second.as_slice()),
        ]);

        let results = evaluate_tran_remeasurements(&netlist, &time, &signals);
        assert_eq!(results.len(), 1);
        assert!(results[0].passed, "{:?}", results[0].error);
        let value = results[0].value.expect("serialized ERR1 result");
        assert!(
            (value - 2.312_406e-1).abs() <= 5.0e-8,
            "serialized ERR1 mismatch: {:.12e}",
            value
        );

        let bare_signals = HashMap::from([
            ("1".to_string(), first.as_slice()),
            ("2".to_string(), second.as_slice()),
        ]);
        let bare_results = evaluate_tran_remeasurements(&netlist, &time, &bare_signals);
        assert!(bare_results[0].passed, "{:?}", bare_results[0].error);
        assert_eq!(bare_results[0].value, results[0].value);
    }

    #[test]
    fn transient_remeasurement_rejects_conflicting_voltage_spellings() {
        let netlist = Netlist::parse(
            "conflicting serialized columns\n\
             .MEASURE TRAN VMAX MAX V(out)\n\
             .END\n",
        )
        .expect("remeasure deck parses");
        let time = [0.0, 1.0];
        let bare = [1.0, 2.0];
        let wrapped = [3.0, 4.0];
        let signals = HashMap::from([
            ("out".to_string(), bare.as_slice()),
            ("V(out)".to_string(), wrapped.as_slice()),
        ]);

        let results = evaluate_tran_remeasurements(&netlist, &time, &signals);
        assert_eq!(results.len(), 1);
        assert!(!results[0].passed);
        assert!(
            results[0]
                .error
                .as_deref()
                .is_some_and(|error| error.contains("conflicts")),
            "{:?}",
            results[0].error
        );
    }

    #[test]
    fn transient_map_keeps_numeric_node_names_authoritative_over_solver_ordinals() {
        let mut result = tran_result();
        result.voltages = vec![
            vec![1.0, 1.1, 1.2, 1.3],
            vec![3.0, 3.1, 3.2, 3.3],
            vec![4.0, 4.1, 4.2, 4.3],
        ];
        result.node_names = vec!["1".to_string(), "3".to_string(), "4".to_string()];
        result.num_nodes = 3;

        let signals = transient_signal_map(&result);
        assert_eq!(signals["V(1)"], result.voltages[0].as_slice());
        assert_eq!(signals["V(3)"], result.voltages[1].as_slice());
        assert_eq!(signals["V(4)"], result.voltages[2].as_slice());
        assert!(
            !signals.contains_key("V(2)"),
            "solver position two is not an authored node alias"
        );
    }

    #[test]
    fn transient_map_uses_ordinals_only_for_missing_collision_free_metadata() {
        let mut missing = tran_result();
        missing.node_names.clear();
        let signals = transient_signal_map(&missing);
        assert_eq!(signals["V(1)"], missing.voltages[0].as_slice());

        let mut partial = tran_result();
        partial.voltages = vec![vec![9.0; 4], vec![1.0; 4]];
        partial.node_names = vec![String::new(), "1".to_string()];
        partial.num_nodes = 2;
        let signals = transient_signal_map(&partial);
        assert_eq!(
            signals["V(1)"],
            partial.voltages[1].as_slice(),
            "a missing-name fallback must not shadow authoritative numeric node 1"
        );
    }

    #[test]
    fn transient_measurements_keep_sparse_numeric_nodes_distinct() {
        let netlist = Netlist::parse(
            "* sparse numeric transient measurement names\n\
             V1 1 0 0\n\
             V3 3 0 0\n\
             V4 4 0 0\n\
             .tran 1 3\n\
             .measure tran crossing WHEN V(3)=V(4)\n\
             .measure tran difference MAX {V(3)-V(4)}\n\
             .end\n",
        )
        .expect("sparse numeric measurements parse");
        let result = TransientResult {
            time: vec![0.0, 1.0, 2.0, 3.0],
            step_sizes: vec![0.0; 4],
            voltages: vec![
                vec![0.0; 4],
                vec![0.0, 0.25, 0.5, 0.75],
                vec![1.0, 0.75, 0.5, 0.25],
            ],
            branch_currents: Vec::new(),
            num_nodes: 3,
            node_names: vec!["1".to_string(), "3".to_string(), "4".to_string()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        };

        let measurements = evaluate_tran_measurements(&netlist, &result);
        assert_eq!(measurements.len(), 2);
        assert!(measurements.iter().all(|measurement| measurement.passed));
        assert_eq!(measurements[0].value, Some(2.0));
        assert_eq!(measurements[1].value, Some(0.5));
    }

    #[test]
    fn transient_measurements_resolve_direct_expression_and_nested_interface_aliases() {
        let netlist = Netlist::parse(
            "BUG 1962 interface measurements\n\
             V1 1 0 0\n\
             X1 1 2 CELL1\n\
             X2 1 4 CELL2\n\
             X3 1 5 OUTER\n\
             .SUBCKT CELL1 A C\n\
             R1 A B 1\n\
             R2 B C 1\n\
             .ENDS\n\
             .SUBCKT CELL2 D F\n\
             R1 D E 1\n\
             R2 E F 1\n\
             .ENDS\n\
             .SUBCKT OUTER G J\n\
             R1 G H 1\n\
             X1 H I CELL1\n\
             R2 I J 1\n\
             .ENDS\n\
             .TRAN 1 1\n\
             .MEASURE TRAN maxExp MAX {V(X1:a)*V(X1:c)}\n\
             .MEASURE TRAN maxNonExp MAX V(X2:d)\n\
             .MEASURE TRAN maxRecursive MAX {V(X3:X1.a)}\n\
             .MEASURE TRAN liveAlias EQN {V(X3.X1:a)+1}\n\
             .MEASURE TRAN_CONT aliasCrossing WHEN V(X3:X1.a)=0.4\n\
             .END\n",
        )
        .expect("hierarchical measurement deck parses");
        let result = TransientResult {
            time: vec![0.0, 1.0],
            step_sizes: vec![0.0; 2],
            voltages: vec![vec![0.0, 1.0], vec![0.0, 1.0 / 3.0], vec![0.0, 0.8]],
            branch_currents: Vec::new(),
            num_nodes: 3,
            node_names: vec!["1".to_string(), "2".to_string(), "X3.H".to_string()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        };

        let measurements = evaluate_tran_measurements(&netlist, &result);
        assert_eq!(measurements.len(), 4);
        assert!(
            measurements.iter().all(|measurement| measurement.passed),
            "{measurements:?}"
        );
        assert_eq!(measurements[0].value, Some(1.0 / 3.0));
        assert_eq!(measurements[1].value, Some(1.0));
        assert_eq!(measurements[2].value, Some(0.8));
        assert_eq!(measurements[3].value, Some(1.8));

        let continuous = evaluate_tran_continuous_measurements(&netlist, &result);
        assert_eq!(continuous.len(), 1);
        assert_eq!(continuous[0].failure, None);
        assert_eq!(continuous[0].records[0].event_axis, Some(0.5));
    }

    #[test]
    fn live_legacy_frac_max_revises_results_and_global_window_gates_histories() {
        let frac_netlist = Netlist::parse(
            "live frac max\n\
             V1 1 0 0\n\
             .tran 1 4\n\
             .measure tran dynamic TRIG V(1) FRAC_MAX=0.5 TARG V(1) FRAC_MAX=0.75\n\
             .measure tran echoed EQN {dynamic}\n\
             .end\n",
        )
        .expect("FRAC_MAX deck parses");
        let frac_result = TransientResult {
            time: vec![0.0, 1.0, 2.0, 3.0, 4.0],
            step_sizes: vec![0.0; 5],
            voltages: vec![vec![0.0, 1.0, 2.0, 1.0, 0.0]],
            branch_currents: Vec::new(),
            num_nodes: 1,
            node_names: vec!["1".to_string()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        };
        let results = evaluate_tran_measurements(&frac_netlist, &frac_result);
        assert_eq!(results[0].value, Some(0.5));
        assert_eq!(results[1].value, Some(0.5));

        let window_netlist = Netlist::parse(
            "legacy global delay window\n\
             V1 1 0 0\n\
             V2 2 0 0\n\
             .options measure use_lttm=1\n\
             .tran 1 5\n\
             .measure tran windowed TRIG V(1)=0 RISE=1 FROM=2 TARG V(2)=0 RISE=1 TO=5\n\
             .measure tran echoed EQN {windowed}\n\
             .end\n",
        )
        .expect("legacy global window deck parses");
        let window_result = TransientResult {
            time: vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
            step_sizes: vec![0.0; 6],
            voltages: vec![
                vec![-1.0, 1.0, 1.0, -1.0, 1.0, 1.0],
                vec![-1.0, -1.0, -1.0, 1.0, -1.0, 1.0],
            ],
            branch_currents: Vec::new(),
            num_nodes: 2,
            node_names: vec!["1".to_string(), "2".to_string()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        };
        let results = evaluate_tran_measurements(&window_netlist, &window_result);
        assert_eq!(results[0].value, Some(0.0));
        assert_eq!(results[1].value, Some(0.0));
    }

    #[test]
    fn live_err_distinguishes_inactive_windows_from_ieee_aggregates() {
        let netlist = Netlist::parse(
            "live ERR IEEE results\n\
             V1 measured 0 0\n\
             V2 zero 0 0\n\
             V3 one 0 0\n\
             .tran 1 1\n\
             .measure tran filtered ERR V(measured) V(zero) MINVAL=0 YMIN=1 YMAX=2\n\
             .measure tran filtered_live EQN filtered\n\
             .measure tran inactive ERR V(measured) V(zero) FROM=3 TO=4 MINVAL=0 YMIN=0 YMAX=1\n\
             .measure tran inactive_live EQN inactive\n\
             .measure tran zero_zero ERR V(measured) V(zero) MINVAL=0 YMIN=0 YMAX=1\n\
             .measure tran zero_zero_live EQN zero_zero\n\
             .measure tran nonzero_zero ERR V(measured) V(one) MINVAL=0 YMIN=0 YMAX=1\n\
             .measure tran nonzero_zero_live EQN nonzero_zero\n\
             .end\n",
        )
        .expect("live ERR IEEE deck parses");
        let result = TransientResult {
            time: vec![0.0, 1.0],
            step_sizes: vec![0.0; 2],
            voltages: vec![vec![0.0; 2], vec![0.0; 2], vec![1.0; 2]],
            branch_currents: Vec::new(),
            num_nodes: 3,
            node_names: vec![
                "measured".to_string(),
                "zero".to_string(),
                "one".to_string(),
            ],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        };
        let traces = evaluate_equation_measurements(
            &netlist,
            "TRAN",
            &result.time,
            &transient_signal_map(&result),
            -1.0,
            None,
        )
        .expect("live ERR IEEE traces evaluate");
        let trace = |name: &str| {
            traces
                .iter()
                .find(|trace| trace.name.eq_ignore_ascii_case(name))
                .expect("named live ERR trace")
        };
        assert!(trace("filtered").initialized);
        assert!(trace("filtered").values.iter().all(|value| value.is_nan()));
        assert!(!trace("inactive").initialized);
        assert!(trace("zero_zero").initialized);
        assert!(trace("zero_zero").values.iter().all(|value| value.is_nan()));
        assert!(trace("nonzero_zero").initialized);
        assert_eq!(trace("nonzero_zero").values, vec![Value::INFINITY; 2]);

        let terminal = evaluate_tran_measurements(&netlist, &result);
        let terminal = |name: &str| {
            terminal
                .iter()
                .find(|result| result.name.eq_ignore_ascii_case(name))
                .expect("named terminal ERR result")
        };
        assert!(terminal("filtered").value.is_some_and(Value::is_nan));
        assert!(!terminal("inactive").passed && terminal("inactive").value.is_none());
        assert!(terminal("zero_zero").value.is_some_and(Value::is_nan));
        assert_eq!(terminal("nonzero_zero").value, Some(Value::INFINITY));
    }

    #[test]
    fn ground_wired_interface_aliases_support_real_voltage_operands() {
        let netlist = Netlist::parse(
            "ground interface aliases\n\
             X1 1 0 CELL\n\
             .SUBCKT CELL A B\n\
             R1 A B 1\n\
             .ENDS\n\
             .TRAN 1 1\n\
             .MEASURE TRAN literalGround MAX V(0)\n\
             .MEASURE TRAN direct MAX V(X1:B)\n\
             .MEASURE TRAN expression MAX {V(X1:B)+1}\n\
             .MEASURE TRAN differential MAX V(X1:A,X1:B)\n\
             .END\n",
        )
        .expect("ground alias deck parses");
        let mut result = tran_result();
        result.time = vec![0.0, 1.0];
        result.step_sizes = vec![0.0; 2];
        result.voltages = vec![vec![0.0, 2.0]];
        result.node_names = vec!["1".to_string()];
        result.num_nodes = 1;
        result.branch_currents.clear();
        result.branch_names.clear();

        let measurements = evaluate_tran_measurements(&netlist, &result);
        assert_eq!(
            measurements
                .iter()
                .map(|measurement| measurement.value)
                .collect::<Vec<_>>(),
            vec![Some(0.0), Some(0.0), Some(1.0), Some(2.0)]
        );
        assert!(
            measurements.iter().all(|measurement| measurement.passed),
            "{measurements:?}"
        );
    }

    #[test]
    fn physical_node_wins_interface_alias_collision() {
        let netlist = Netlist::parse(
            "physical node precedence\n\
             VREAL X1.A 0 0\n\
             X1 1 0 CELL\n\
             X2 X1.A 0 CELL\n\
             .SUBCKT CELL A B\n\
             R1 A B 1\n\
             .ENDS\n\
             .TRAN 1 1\n\
             .MEASURE TRAN directCollision MAX V(X1:A)\n\
             .MEASURE TRAN targetCollision MAX V(X2:A)\n\
             .END\n",
        )
        .expect("collision deck parses");
        let mut result = tran_result();
        result.time = vec![0.0, 1.0];
        result.step_sizes = vec![0.0; 2];
        result.voltages = vec![vec![0.0, 1.0], vec![0.0, 9.0]];
        result.node_names = vec!["1".to_string(), "X1.A".to_string()];
        result.num_nodes = 2;
        result.branch_currents.clear();
        result.branch_names.clear();

        let measurements = evaluate_tran_measurements(&netlist, &result);
        assert_eq!(measurements.len(), 2);
        assert_eq!(measurements[0].value, Some(9.0));
        assert_eq!(measurements[1].value, Some(9.0));
        assert!(
            measurements.iter().all(|measurement| measurement.passed),
            "{measurements:?}"
        );
    }

    #[test]
    fn canonical_signal_index_reports_hierarchy_spelling_ambiguity() {
        let first = [1.0, 2.0];
        let second = [3.0, 4.0];
        let signals = HashMap::from([
            ("V(X1.A)".to_string(), first.as_slice()),
            ("v(x1:a)".to_string(), second.as_slice()),
        ]);

        let error = CanonicalMeasureSignalIndex::new(&signals)
            .get("V(X1:A)")
            .expect_err("distinct canonical columns are ambiguous");

        assert!(error.contains("ambiguous"), "{error}");
        assert!(error.contains("V(X1.A)"), "{error}");
        assert!(error.contains("v(x1:a)"), "{error}");
    }

    #[test]
    fn interface_projection_is_requested_and_analysis_scoped() {
        let netlist = Netlist::parse(
            "scoped interface projection\n\
             X1 1 0 CELL\n\
             X2 2 0 CELL\n\
             .SUBCKT CELL A B\n\
             R1 A B 1\n\
             .ENDS\n\
             .TRAN 1 1\n\
             .AC LIN 2 1 2\n\
             .MEASURE TRAN used MAX V(X1:A)\n\
             .MEASURE AC other MAX V(X2:A)\n\
             .END\n",
        )
        .expect("multi-analysis projection deck parses");
        let first = [1.0, 2.0];
        let second = [3.0, 4.0];
        let mut signals = HashMap::from([
            ("V(1)".to_string(), first.as_slice()),
            ("1".to_string(), first.as_slice()),
            ("V(2)".to_string(), second.as_slice()),
            ("2".to_string(), second.as_slice()),
        ]);
        let projection =
            InterfaceNodeAliasProjection::new(&netlist, OutputAnalysisKind::Tran, first.len())
                .expect("TRAN projection builds");

        projection
            .augment(&mut signals)
            .expect("selected TRAN alias projects");

        assert_eq!(signals["V(X1:A)"], first);
        assert!(!signals.contains_key("V(X1:B)"));
        assert!(!signals.contains_key("V(X2:A)"));
    }

    #[test]
    fn transient_measurement_alias_collection_fails_recursive_hierarchy_without_recursing() {
        let netlist = Netlist::parse(
            "recursive measurement aliases\n\
             X1 1 0 LOOP\n\
             .SUBCKT LOOP A B\n\
             XSELF A B LOOP\n\
             .ENDS\n\
             .TRAN 1 1\n\
             .MEASURE TRAN maximum MAX V(X1:XSELF:A)\n\
             .END\n",
        )
        .expect("syntactic parser retains recursive hierarchy");
        let result = TransientResult {
            time: vec![0.0, 1.0],
            step_sizes: vec![0.0; 2],
            voltages: vec![vec![0.0, 1.0]],
            branch_currents: Vec::new(),
            num_nodes: 1,
            node_names: vec!["1".to_string()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        };

        let measurements = evaluate_tran_measurements(&netlist, &result);
        assert_eq!(measurements.len(), 1);
        assert!(!measurements[0].passed);
        assert!(
            measurements[0]
                .error
                .as_deref()
                .is_some_and(|error| error.contains("Recursive subcircuit instantiation")),
            "{measurements:?}"
        );
    }

    #[test]
    fn dc_interface_aliases_reach_direct_expression_equation_and_continuous_paths() {
        let netlist = Netlist::parse(
            "DC interface measurement paths\n\
             V1 1 0 0\n\
             X1 1 0 CELL\n\
             .SUBCKT CELL A B\n\
             R1 A B 1\n\
             .ENDS\n\
             .DC V1 0 1 1\n\
             .MEASURE DC direct MAX V(X1:A)\n\
             .MEASURE DC expression MAX {V(X1:A)*2}\n\
             .MEASURE DC equation EQN {V(X1:A)+1}\n\
             .MEASURE DC ground MAX V(0)\n\
             .MEASURE DC_CONT crossing WHEN V(X1:A)=1.5\n\
             .END\n",
        )
        .expect("DC interface deck parses");
        let sweep = [1.0, 2.0]
            .into_iter()
            .enumerate()
            .map(|(axis, voltage)| {
                let mut point = SimulationResult::new(1, 0);
                point.node_voltages = vec![0.0, voltage];
                point.node_names = vec!["0".to_string(), "1".to_string()];
                (axis as Value, point)
            })
            .collect::<Vec<_>>();

        let scalar = evaluate_dc_measurements(&netlist, &sweep);
        assert_eq!(
            scalar.iter().map(|result| result.value).collect::<Vec<_>>(),
            vec![Some(2.0), Some(4.0), Some(3.0), Some(0.0)]
        );
        assert!(scalar.iter().all(|result| result.passed), "{scalar:?}");

        let continuous = evaluate_dc_continuous_measurements(&netlist, &sweep);
        assert_eq!(continuous.len(), 1);
        assert_eq!(continuous[0].failure, None);
        assert_eq!(continuous[0].records[0].event_axis, Some(0.5));
    }

    #[test]
    fn transient_equation_scalar_results_match_final_trace_values() {
        let netlist = Netlist::parse(
            "* continuous transient equations\n\
             V1 out 0 0\n\
             .tran 1 3\n\
             .meas tran bounded EQN {V(out)+1} FROM=1 TO=2\n\
             .meas tran invalid EQN {V(out)} FROM=4 TO=5 DEFAULT_VAL=-9\n\
             .end\n",
        )
        .expect("transient equations parse");

        let results = evaluate_tran_measurements(&netlist, &tran_result());
        assert_eq!(results[0].value, Some(3.0));
        assert!(results[0].passed);
        assert_eq!(results[1].value, None);
        assert!(!results[1].passed);
    }

    #[test]
    fn transient_equations_observe_live_scalar_measures_in_declaration_order() {
        let netlist = Netlist::parse(
            "* declaration-ordered live scalar visibility\n\
             V1 out 0 0\n\
             .tran 0.5 1\n\
             .measure tran before EQN peak\n\
             .measure tran peak MAX V(out) DEFAULT_VAL=-7\n\
             .measure tran after EQN peak\n\
             .end\n",
        )
        .expect("ordered live-measure deck parses");
        let result = TransientResult {
            time: vec![0.0, 0.5, 1.0],
            step_sizes: vec![0.0; 3],
            voltages: vec![vec![0.0, 2.0, 1.0]],
            branch_currents: Vec::new(),
            num_nodes: 1,
            node_names: vec!["out".to_string()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        };

        let traces = evaluate_tran_equation_measurements(&netlist, &result)
            .expect("equation traces evaluate");
        assert_eq!(traces.len(), 2, "public API returns equation traces only");
        assert_eq!(traces[0].name, "BEFORE");
        assert_eq!(traces[0].values, vec![-7.0, 0.0, 2.0]);
        assert_eq!(traces[1].name, "AFTER");
        assert_eq!(traces[1].values, vec![0.0, 2.0, 2.0]);

        let results = evaluate_tran_measurements(&netlist, &result);
        assert_eq!(
            results
                .iter()
                .map(|result| result.value)
                .collect::<Vec<_>>(),
            vec![Some(2.0), Some(2.0), Some(2.0)]
        );
        assert!(results.iter().all(|result| result.passed), "{results:?}");
    }

    #[test]
    fn transient_point_measures_consume_prior_equation_traces() {
        let netlist = Netlist::parse(
            "* transient equation consumers\n\
             V1 out 0 0\n\
             .tran 1 3\n\
             .measure tran before_crossing WHEN shifted=1.5\n\
             .measure tran before_sample FIND V(out) WHEN shifted=1.5\n\
             .measure tran shifted EQN V(out)\n\
             .measure tran after_crossing WHEN shifted=1.5\n\
             .measure tran after_sample FIND V(out) WHEN shifted=1.5\n\
             .end\n",
        )
        .expect("transient equation-consumer deck parses");

        let results = evaluate_tran_measurements(&netlist, &tran_result());
        assert_eq!(
            results
                .iter()
                .map(|result| result.value)
                .collect::<Vec<_>>(),
            vec![Some(2.5), Some(2.5), Some(3.0), Some(1.5), Some(1.5)]
        );
        assert!(results.iter().all(|result| result.passed), "{results:?}");
    }

    #[test]
    fn delay_occurrences_are_counted_after_effective_td() {
        let netlist = Netlist::parse(
            "* delay TD occurrence ordering\n\
             V1 out 0 0\n\
             .tran 1 5\n\
             .measure tran trig_delay TRIG V(out)=0.5 TD=1.5 RISE=1 TARG AT=4\n\
             .measure tran targ_delay TRIG AT=0 TARG V(out)=0.5 TD=1.5 RISE=1\n\
             .end\n",
        )
        .expect("delay TD deck parses");
        let result = tran_waveform(
            vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
            vec![0.0, 1.0, 0.0, 1.0, 0.0, 0.0],
        );

        let results = evaluate_tran_measurements(&netlist, &result);
        assert_eq!(results[0].value, Some(1.5));
        assert_eq!(results[1].value, Some(2.5));
        assert!(results.iter().all(|result| result.passed), "{results:?}");
    }

    #[test]
    fn delay_at_live_reach_matches_directional_minval_contract() {
        let netlist = Netlist::parse(
            "* delay AT accepted-point reach\n\
             V1 out 0 0\n\
             .tran 1 1\n\
             .measure tran endpoint TRIG AT=0 TARG AT=1 DEFAULT_VAL=-7\n\
             .measure tran endpoint_live EQN endpoint\n\
             .measure tran interior TRIG AT=0 TARG AT=0.5 DEFAULT_VAL=-7\n\
             .measure tran interior_live EQN interior\n\
             .end\n",
        )
        .expect("delay AT reach deck parses");
        let result = tran_waveform(vec![0.0, 1.0], vec![0.0, 0.0]);

        let traces = evaluate_tran_equation_measurements(&netlist, &result)
            .expect("delay AT live traces evaluate");
        assert_eq!(traces[0].values, vec![-7.0, -7.0]);
        assert_eq!(traces[1].values, vec![-7.0, 0.5]);
        let results = evaluate_tran_measurements(&netlist, &result);
        assert!(!results[0].passed, "exact final endpoint is not overshot");
        assert_eq!(results[2].value, Some(0.5));
    }

    #[test]
    fn find_at_first_sample_publishes_before_derivative() {
        let netlist = Netlist::parse(
            "* first-sample point publication\n\
             V1 out 0 0\n\
             .tran 1 1\n\
             .measure tran found FIND V(out) AT=0 DEFAULT_VAL=-7\n\
             .measure tran found_live EQN found\n\
             .measure tran slope DERIV V(out) AT=0 DEFAULT_VAL=-9\n\
             .measure tran slope_live EQN slope\n\
             .end\n",
        )
        .expect("first-sample point deck parses");
        let result = tran_waveform(vec![0.0, 1.0], vec![2.0, 5.0]);

        let traces = evaluate_tran_equation_measurements(&netlist, &result)
            .expect("first-sample equation traces evaluate");
        let trace = |name: &str| {
            &traces
                .iter()
                .find(|trace| trace.name.eq_ignore_ascii_case(name))
                .expect("named trace")
                .values
        };
        assert_eq!(trace("found_live"), &vec![2.0, 2.0]);
        assert_eq!(trace("slope_live"), &vec![-9.0, 3.0]);

        let results = evaluate_tran_measurements(&netlist, &result);
        assert_eq!(results[0].value, Some(2.0));
        assert_eq!(results[2].value, Some(3.0));
    }

    #[test]
    fn find_at_uses_authored_minval_in_scalar_continuous_and_live_paths() {
        let netlist = Netlist::parse(
            "* FIND AT accepted-row MINVAL parity\n\
             V1 out 0 0\n\
             .tran 1 1\n\
             .measure tran default_near FIND V(out) AT=5e-13 DEFAULT_VAL=-7\n\
             .measure tran default_live EQN default_near\n\
             .measure tran small FIND V(out) AT=5e-13 MINVAL=1e-14 DEFAULT_VAL=-7\n\
             .measure tran small_live EQN small\n\
             .measure tran_cont small_cont FIND V(out) AT=5e-13 MINVAL=1e-14\n\
             .end\n",
        )
        .expect("FIND AT MINVAL parity deck parses");
        let result = tran_waveform(vec![0.0, 1.0], vec![0.0, 1.0e12]);

        let traces = evaluate_tran_equation_measurements(&netlist, &result)
            .expect("FIND AT MINVAL live traces evaluate");
        let trace = |name: &str| {
            &traces
                .iter()
                .find(|trace| trace.name.eq_ignore_ascii_case(name))
                .expect("named trace")
                .values
        };
        assert_eq!(trace("default_live"), &vec![0.0, 0.0]);
        assert_eq!(trace("small_live"), &vec![-7.0, 0.5]);

        let scalar = evaluate_tran_measurements(&netlist, &result);
        assert_eq!(scalar[0].value, Some(0.0));
        assert_eq!(scalar[1].value, Some(0.0));
        assert_eq!(scalar[2].value, Some(0.5));
        assert_eq!(scalar[3].value, Some(0.5));

        let continuous = evaluate_tran_continuous_measurements(&netlist, &result);
        assert_eq!(continuous.len(), 1);
        assert_eq!(continuous[0].records[0].value, 0.5);
        assert_eq!(continuous[0].records[0].event_axis, Some(5.0e-13));
    }

    #[test]
    fn find_and_derivative_at_share_accepted_rows_across_all_tran_adapters() {
        let netlist = Netlist::parse(
            "* FIND and DERIV accepted-row parity\n\
             V1 out 0 0\n\
             .tran 1 2\n\
             .measure tran found FIND V(out) AT=1.05 MINVAL=0.1 DEFAULT_VAL=-7\n\
             .measure tran found_live EQN found\n\
             .measure tran slope DERIV V(out) AT=1.05 MINVAL=0.1 DEFAULT_VAL=-7\n\
             .measure tran slope_live EQN slope\n\
             .measure tran_cont found_cont FIND V(out) AT=1.05 MINVAL=0.1\n\
             .measure tran_cont slope_cont DERIV V(out) AT=1.05 MINVAL=0.1\n\
             .end\n",
        )
        .expect("FIND and DERIV accepted-row parity deck parses");
        let result = tran_waveform(vec![0.0, 1.0, 2.0], vec![0.0, 10.0, 40.0]);

        let traces = evaluate_tran_equation_measurements(&netlist, &result)
            .expect("FIND and DERIV accepted-row live traces evaluate");
        let trace = |name: &str| {
            &traces
                .iter()
                .find(|trace| trace.name.eq_ignore_ascii_case(name))
                .expect("named trace")
                .values
        };
        assert_eq!(trace("found_live"), &vec![-7.0, 10.0, 10.0]);
        assert_eq!(trace("slope_live"), &vec![-7.0, 10.0, 10.0]);

        let scalar = evaluate_tran_measurements(&netlist, &result);
        assert_eq!(scalar[0].value, Some(10.0));
        assert_eq!(scalar[1].value, Some(10.0));
        assert_eq!(scalar[2].value, Some(10.0));
        assert_eq!(scalar[3].value, Some(10.0));

        let continuous = evaluate_tran_continuous_measurements(&netlist, &result);
        assert_eq!(continuous.len(), 2);
        assert_eq!(continuous[0].records[0].value, 10.0);
        assert_eq!(continuous[1].records[0].value, 10.0);
    }

    #[test]
    fn live_getters_preserve_extrema_and_first_statistic_semantics() {
        let netlist = Netlist::parse(
            "* live getter details\n\
             V1 out 0 0\n\
             .options measure default_val=-11\n\
             .tran 1 2\n\
             .measure tran maximum MAX V(out) OUTPUT=TIME DEFAULT_VAL=-7\n\
             .measure tran maximum_live EQN maximum\n\
             .measure tran average AVG V(out) DEFAULT_VAL=-7\n\
             .measure tran average_live EQN average\n\
             .measure tran rms_value RMS V(out) DEFAULT_VAL=-7\n\
             .measure tran rms_live EQN rms_value\n\
             .measure tran integral_value INTEG V(out) DEFAULT_VAL=-7\n\
             .measure tran integral_live EQN integral_value\n\
             .end\n",
        )
        .expect("live getter detail deck parses");
        let result = tran_waveform(vec![0.0, 1.0, 2.0], vec![2.0, 4.0, 3.0]);

        let traces = evaluate_tran_equation_measurements(&netlist, &result)
            .expect("live getter traces evaluate");
        let trace = |name: &str| {
            &traces
                .iter()
                .find(|trace| trace.name.eq_ignore_ascii_case(name))
                .expect("named trace")
                .values
        };
        assert_eq!(trace("maximum_live"), &vec![2.0, 4.0, 4.0]);
        assert_eq!(trace("average_live"), &vec![-11.0, 3.0, 3.25]);
        assert_eq!(trace("rms_live")[0], -11.0);
        assert!((trace("rms_live")[1] - 10.0_f64.sqrt()).abs() < 1.0e-12);
        assert!((trace("rms_live")[2] - 11.25_f64.sqrt()).abs() < 1.0e-12);
        assert_eq!(trace("integral_live"), &vec![0.0, 3.0, 6.5]);

        let results = evaluate_tran_measurements(&netlist, &result);
        assert_eq!(results[0].value, Some(1.0), "terminal OUTPUT=TIME");
        assert_eq!(results[1].value, Some(4.0), "live MAX getter is VALUE");
    }

    #[test]
    fn last_event_updates_live_value_with_bounded_history() {
        let netlist = Netlist::parse(
            "* LAST event live state\n\
             V1 out 0 0\n\
             .tran 1 4\n\
             .measure tran latest WHEN V(out)=0.5 RISE=LAST DEFAULT_VAL=-7\n\
             .measure tran latest_live EQN latest\n\
             .end\n",
        )
        .expect("LAST event deck parses");
        let result = tran_waveform(vec![0.0, 1.0, 2.0, 3.0, 4.0], vec![0.0, 1.0, 0.0, 1.0, 0.0]);

        let traces = evaluate_tran_equation_measurements(&netlist, &result)
            .expect("LAST event traces evaluate");
        assert_eq!(traces[0].values, vec![-7.0, 0.5, 0.5, 2.5, 2.5]);
        let results = evaluate_tran_measurements(&netlist, &result);
        assert_eq!(results[0].value, Some(2.5));
        assert_eq!(results[1].value, Some(2.5));

        let axis = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let waveform = [0.0, 1.0, 0.0, 1.0, 0.0, 1.0];
        let signals = HashMap::from([("V(out)".to_string(), waveform.as_slice())]);
        let signal_index = CanonicalMeasureSignalIndex::new(&signals);
        let params = crate::netlist::ParamContext::new();
        let mut selector = LiveCondition::compile(
            &WhenCondition {
                left: "V(out)".to_string(),
                right: MeasureOperand::Constant(0.5),
                occurrence: crate::netlist::measure::EventOccurrence {
                    edge: EdgeType::Rise,
                    number: -2,
                },
            },
            1.0e-12,
            &params,
        )
        .expect("bounded selector compiles");
        let mut programs = Vec::<LiveMeasureProgram<'_>>::new();
        let mut current_values = HashMap::new();
        let program_indices = HashMap::new();
        let mut selected = None;
        for (row, axis_value) in axis.iter().copied().enumerate() {
            let mut reads = LiveMeasureReadContext {
                programs: &mut programs,
                current_values: &mut current_values,
                program_indices: &program_indices,
                row,
                axis: &axis,
            };
            if let Some(event) = selector
                .update(
                    row,
                    axis_value,
                    false,
                    &signal_index,
                    &mut reads,
                    &params,
                    Value::NEG_INFINITY,
                    Value::INFINITY,
                    None,
                )
                .expect("bounded selector updates")
                .selected
            {
                selected = Some(event);
            }
            assert!(selector.negative_events.len() <= 2);
        }
        assert_eq!(selected.map(|event| event.axis), Some(2.5));
    }

    #[test]
    fn find_last_recovers_after_an_undefined_candidate_and_preserves_raw_nan() {
        let netlist = Netlist::parse(
            "* FIND LAST recoverable undefined candidate\n\
             V1 out 0 0\n\
             V2 cond 0 0\n\
             .tran 1 3\n\
             .measure tran latest FIND V(out) WHEN V(cond)=0 CROSS=LAST DEFAULT_VAL=-7\n\
             .measure tran observer EQN latest DEFAULT_VAL=-8\n\
             .measure tran latest_live EQN latest FROM=3 DEFAULT_VAL=-9\n\
             .measure tran param_value PARAM='latest'\n\
             .measure tran param_live EQN param_value FROM=3 DEFAULT_VAL=-10\n\
             .measure tran independent EQN 42\n\
             .end\n",
        )
        .expect("FIND LAST recovery deck parses");
        let result = TransientResult {
            time: vec![0.0, 1.0, 2.0, 3.0],
            step_sizes: vec![0.0; 4],
            voltages: vec![
                vec![Value::INFINITY, Value::NEG_INFINITY, 0.0, 2.0],
                vec![-1.0, 1.0, -1.0, 1.0],
            ],
            branch_currents: Vec::new(),
            num_nodes: 2,
            node_names: vec!["out".to_string(), "cond".to_string()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        };

        let traces = evaluate_equation_measurements(
            &netlist,
            "TRAN",
            &result.time,
            &transient_signal_map(&result),
            -1.0,
            None,
        )
        .expect("FIND LAST live traces recover");
        let trace = |name: &str| {
            traces
                .iter()
                .find(|trace| trace.name.eq_ignore_ascii_case(name))
                .expect("named live trace")
        };
        let latest = trace("latest");
        assert_eq!(latest.values[0], -7.0);
        assert!(latest.values[1].is_nan());
        assert_eq!(latest.values[2], Value::NEG_INFINITY);
        assert_eq!(latest.values[3], 1.0);
        assert_eq!(latest.valid, vec![true, false, true, true]);
        let observer = trace("observer");
        assert_eq!(observer.values[0], -7.0);
        assert!(observer.values[1].is_nan());
        assert_eq!(observer.values[2], Value::NEG_INFINITY);
        assert_eq!(observer.values[3], 1.0);
        assert_eq!(observer.valid, vec![true, false, true, true]);
        assert_eq!(trace("latest_live").values, vec![-9.0, -9.0, -9.0, 1.0]);
        let param = trace("param_value");
        assert_eq!(param.values[0..3], [0.0, 0.0, 0.0]);
        assert_eq!(param.values[3], 1.0);
        assert_eq!(param.valid, vec![true; 4]);
        assert_eq!(trace("param_live").values, vec![-10.0, -10.0, -10.0, 1.0]);
        assert_eq!(trace("independent").values, vec![42.0; 4]);

        let terminal = evaluate_tran_measurements(&netlist, &result);
        assert_eq!(terminal[0].value, Some(1.0));
        assert_eq!(terminal[1].value, Some(1.0));
        assert_eq!(terminal[2].value, Some(1.0));
        assert_eq!(terminal[3].value, Some(1.0));
        assert_eq!(terminal[4].value, Some(1.0));
        assert_eq!(terminal[5].value, Some(42.0));
        assert!(terminal.iter().all(|result| result.passed));
    }

    #[test]
    fn braced_measure_operands_normalize_raw_nonfinite_getters_before_consumption() {
        let netlist = Netlist::parse(
            "* Xyce ExpressionOp root normalization\n\
             V1 out 0 0\n\
             V2 cond 0 0\n\
             V3 ref 0 0\n\
             .tran 1 3\n\
             .measure tran latest FIND V(out) WHEN V(cond)=0 CROSS=LAST DEFAULT_VAL=-7\n\
             .measure tran raw_eq EQN latest\n\
             .measure tran braced_eq EQN {latest}\n\
             .measure tran physical_expr EQN {V(out)}\n\
             .measure tran raw_max MAX latest FROM=1\n\
             .measure tran raw_max_live EQN raw_max\n\
             .measure tran braced_max MAX {latest} FROM=1\n\
             .measure tran braced_max_live EQN braced_max\n\
             .measure tran raw_pp PP latest FROM=1\n\
             .measure tran raw_pp_live EQN raw_pp\n\
             .measure tran braced_pp PP {latest} FROM=1\n\
             .measure tran braced_pp_live EQN braced_pp\n\
             .measure tran raw_measured ERR latest V(ref) FROM=1 YMIN=0 YMAX=1e100\n\
             .measure tran raw_measured_live EQN raw_measured\n\
             .measure tran braced_measured ERR {latest} V(ref) FROM=1 YMIN=0 YMAX=1e100\n\
             .measure tran braced_measured_live EQN braced_measured\n\
             .measure tran raw_comparison ERR V(ref) latest FROM=1 YMIN=0 YMAX=1e100\n\
             .measure tran raw_comparison_live EQN raw_comparison\n\
             .measure tran braced_comparison ERR V(ref) {latest} FROM=1 YMIN=0 YMAX=1e100\n\
             .measure tran braced_comparison_live EQN braced_comparison\n\
             .measure tran independent EQN 42\n\
             .end\n",
        )
        .expect("raw-vs-braced consumer deck parses");
        let result = TransientResult {
            time: vec![0.0, 1.0, 2.0, 3.0],
            step_sizes: vec![0.0; 4],
            voltages: vec![
                vec![Value::INFINITY, Value::NEG_INFINITY, 0.0, 2.0],
                vec![-1.0, 1.0, -1.0, 1.0],
                vec![1.0; 4],
            ],
            branch_currents: Vec::new(),
            num_nodes: 3,
            node_names: vec!["out".to_string(), "cond".to_string(), "ref".to_string()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        };

        let traces = evaluate_equation_measurements(
            &netlist,
            "TRAN",
            &result.time,
            &transient_signal_map(&result),
            -1.0,
            None,
        )
        .expect("raw-vs-braced consumers evaluate");
        let trace = |name: &str| {
            &traces
                .iter()
                .find(|trace| trace.name.eq_ignore_ascii_case(name))
                .expect("named raw-vs-braced trace")
                .values
        };
        assert!(trace("raw_eq")[1].is_nan());
        assert_eq!(trace("raw_eq")[2], Value::NEG_INFINITY);
        assert_eq!(trace("braced_eq"), &vec![-7.0, 1.0e50, -1.0e50, 1.0]);
        assert_eq!(trace("physical_expr"), &vec![1.0e50, -1.0e50, 0.0, 2.0]);

        assert!(
            trace("raw_max_live")[1..]
                .iter()
                .all(|value| value.is_nan())
        );
        assert_eq!(trace("braced_max_live"), &vec![0.0, 1.0e50, 1.0e50, 1.0e50]);
        assert!(trace("raw_pp_live")[1..].iter().all(|value| value.is_nan()));
        assert_eq!(trace("braced_pp_live"), &vec![0.0, 0.0, 2.0e50, 2.0e50]);

        assert!(trace("raw_measured_live")[1].is_nan());
        assert!(trace("raw_measured_live")[2].is_nan());
        assert_eq!(trace("raw_measured_live")[3], 0.0);
        let expected_braced_error = (2.0_f64 / 3.0).sqrt();
        assert_eq!(trace("braced_measured_live")[1], 1.0);
        assert_eq!(trace("braced_measured_live")[2], 1.0);
        assert!((trace("braced_measured_live")[3] - expected_braced_error).abs() < 1.0e-12);
        assert!(
            trace("raw_comparison_live")[1..]
                .iter()
                .all(|value| value.is_nan())
        );
        assert!(
            (trace("braced_comparison_live")[3] / 1.0e50 - expected_braced_error).abs() < 1.0e-12
        );
        assert_eq!(trace("independent"), &vec![42.0; 4]);

        let terminal = evaluate_tran_measurements(&netlist, &result);
        let terminal = |name: &str| {
            terminal
                .iter()
                .find(|result| result.name.eq_ignore_ascii_case(name))
                .expect("named raw-vs-braced terminal result")
        };
        assert!(terminal("raw_max").value.is_some_and(Value::is_nan));
        assert!(terminal("raw_pp").value.is_some_and(Value::is_nan));
        assert!(terminal("raw_comparison").value.is_some_and(Value::is_nan));
        assert_eq!(terminal("braced_max").value, Some(1.0e50));
        assert_eq!(terminal("braced_pp").value, Some(2.0e50));
        assert_eq!(terminal("independent").value, Some(42.0));
    }

    #[test]
    fn raw_nan_when_events_count_snap_find_and_preserve_terminal_invariants() {
        let netlist = Netlist::parse(
            "* raw NaN WHEN event provenance\n\
             V1 source 0 0\n\
             V2 producer_cond 0 0\n\
             V3 sample 0 0\n\
             .tran 1 5\n\
             .measure tran latest FIND V(source) WHEN V(producer_cond)=0 CROSS=LAST DEFAULT_VAL=-7\n\
             .measure tran raw_when WHEN latest=0 CROSS=1\n\
             .measure tran raw_when_live EQN raw_when\n\
             .measure tran braced_when WHEN {latest}=0 CROSS=1\n\
             .measure tran braced_when_live EQN braced_when\n\
             .measure tran snapped FIND V(sample) WHEN latest=0 CROSS=1\n\
             .measure tran snapped_live EQN snapped\n\
             .measure tran raw_derivative DERIV V(sample) WHEN latest=0 CROSS=1\n\
             .measure tran raw_derivative_live EQN raw_derivative\n\
             .measure tran second_cross WHEN latest=0 CROSS=2\n\
             .measure tran second_cross_live EQN second_cross\n\
             .measure tran first_fall WHEN latest=0 FALL=1\n\
             .measure tran first_fall_live EQN first_fall\n\
             .end\n",
        )
        .expect("raw NaN WHEN deck parses");
        let result = TransientResult {
            time: vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
            step_sizes: vec![0.0; 6],
            voltages: vec![
                vec![Value::INFINITY, Value::NEG_INFINITY, 0.0, 2.0, 0.0, -2.0],
                vec![-1.0, 1.0, 0.0, 1.0, -1.0, 1.0],
                vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0],
            ],
            branch_currents: Vec::new(),
            num_nodes: 3,
            node_names: vec![
                "source".to_string(),
                "producer_cond".to_string(),
                "sample".to_string(),
            ],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        };

        let traces = evaluate_equation_measurements(
            &netlist,
            "TRAN",
            &result.time,
            &transient_signal_map(&result),
            -1.0,
            None,
        )
        .expect("raw NaN WHEN traces evaluate");
        let trace = |name: &str| {
            &traces
                .iter()
                .find(|trace| trace.name.eq_ignore_ascii_case(name))
                .expect("named raw NaN WHEN trace")
                .values
        };
        assert!(
            trace("raw_when_live")[2..]
                .iter()
                .all(|value| value.is_nan())
        );
        assert!(
            trace("braced_when_live")[1..]
                .iter()
                .all(|value| (*value - 7.0e-50).abs() < 1.0e-62)
        );
        assert_eq!(trace("snapped_live")[2..], [30.0; 4]);
        assert_eq!(trace("raw_derivative_live")[2..], [10.0; 4]);
        assert_eq!(trace("second_cross_live")[5], 4.5);
        assert_eq!(trace("first_fall_live")[5], 4.5);

        let terminal = evaluate_tran_measurements(&netlist, &result);
        let terminal = |name: &str| {
            terminal
                .iter()
                .find(|result| result.name.eq_ignore_ascii_case(name))
                .expect("named raw NaN WHEN result")
        };
        assert!(terminal("raw_when").value.is_some_and(Value::is_nan));
        assert_eq!(terminal("raw_derivative").value, Some(10.0));
        assert!(
            terminal("raw_derivative")
                .event_axis
                .is_some_and(Value::is_nan)
        );
        assert!(
            (terminal("braced_when").value.expect("braced WHEN value") - 7.0e-50).abs() < 1.0e-62
        );
        assert_eq!(terminal("snapped").value, Some(30.0));
        assert_eq!(terminal("second_cross").value, Some(4.5));
        assert_eq!(terminal("first_fall").value, Some(4.5));
    }

    #[test]
    fn modern_trig_targ_delay_retains_raw_nan_history_in_explicit_rfc_window() {
        let netlist = Netlist::parse(
            "* Xyce TrigTarg raw NaN history\n\
             V1 source 0 0\n\
             V2 producer_cond 0 0\n\
             .tran 1 5\n\
             .measure tran latest FIND V(source) WHEN V(producer_cond)=0 CROSS=LAST DEFAULT_VAL=-7\n\
             .measure tran raw_delay TRIG latest=0 TARG AT=4.5\n\
             .measure tran raw_delay_live EQN raw_delay\n\
             .measure tran explicit_delay TRIG latest=0 CROSS=1 TARG AT=4.5\n\
             .measure tran explicit_delay_live EQN explicit_delay\n\
             .measure tran braced_delay TRIG {latest}=0 TARG AT=4.5\n\
             .measure tran braced_delay_live EQN braced_delay\n\
             .end\n",
        )
        .expect("raw delay history deck parses");
        let result = TransientResult {
            time: vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
            step_sizes: vec![0.0; 6],
            voltages: vec![
                vec![Value::INFINITY, Value::NEG_INFINITY, 0.0, 2.0, 0.0, -2.0],
                vec![-1.0, 1.0, 0.0, 1.0, -1.0, 1.0],
            ],
            branch_currents: Vec::new(),
            num_nodes: 2,
            node_names: vec!["source".to_string(), "producer_cond".to_string()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        };

        let traces = evaluate_equation_measurements(
            &netlist,
            "TRAN",
            &result.time,
            &transient_signal_map(&result),
            -1.0,
            None,
        )
        .expect("raw delay history traces evaluate");
        let trace = |name: &str| {
            &traces
                .iter()
                .find(|trace| trace.name.eq_ignore_ascii_case(name))
                .expect("named raw delay trace")
                .values
        };
        assert!(trace("raw_delay_live")[5].is_nan());
        assert!(trace("explicit_delay_live")[5].is_nan());
        let expected_braced = 4.5 - 7.0e-50;
        assert_eq!(trace("braced_delay_live")[5], expected_braced);

        let terminal = evaluate_tran_measurements(&netlist, &result);
        let terminal = |name: &str| {
            terminal
                .iter()
                .find(|result| result.name.eq_ignore_ascii_case(name))
                .expect("named raw delay result")
        };
        assert!(terminal("raw_delay").value.is_some_and(Value::is_nan));
        assert!(terminal("explicit_delay").value.is_some_and(Value::is_nan));
        assert_eq!(terminal("braced_delay").value, Some(expected_braced));
    }

    #[test]
    fn final_undefined_last_candidate_retains_nan_and_isolates_independent_results() {
        let netlist = Netlist::parse(
            "* terminal undefined LAST candidate isolation\n\
             V1 out 0 0\n\
             V2 cond 0 0\n\
             .tran 1 3\n\
             .measure tran latest FIND V(out) WHEN V(cond)=0 CROSS=LAST DEFAULT_VAL=-7\n\
             .measure tran observer EQN latest\n\
             .measure tran independent EQN 42\n\
             .end\n",
        )
        .expect("terminal undefined LAST deck parses");
        let result = TransientResult {
            time: vec![0.0, 1.0, 2.0, 3.0],
            step_sizes: vec![0.0; 4],
            voltages: vec![
                vec![0.0, 2.0, Value::INFINITY, Value::NEG_INFINITY],
                vec![-1.0, 1.0, -1.0, 1.0],
            ],
            branch_currents: Vec::new(),
            num_nodes: 2,
            node_names: vec!["out".to_string(), "cond".to_string()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        };

        let traces = evaluate_equation_measurements(
            &netlist,
            "TRAN",
            &result.time,
            &transient_signal_map(&result),
            -1.0,
            None,
        )
        .expect("terminal undefined LAST traces evaluate");
        let trace = |name: &str| {
            traces
                .iter()
                .find(|trace| trace.name.eq_ignore_ascii_case(name))
                .expect("named live trace")
        };
        assert!(trace("latest").values[3].is_nan());
        assert!(trace("observer").values[3].is_nan());
        assert!(!trace("latest").valid[3]);
        assert!(!trace("observer").valid[3]);
        assert_eq!(trace("independent").values, vec![42.0; 4]);
        assert_eq!(trace("independent").valid, vec![true; 4]);

        let terminal = evaluate_tran_measurements(&netlist, &result);
        assert!(terminal[0].value.is_some_and(Value::is_nan));
        assert_eq!(terminal[0].event_axis, Some(2.5));
        assert!(terminal[0].error.is_none());
        assert!(terminal[0].passed);
        assert!(terminal[1].value.is_some_and(Value::is_nan));
        assert!(terminal[1].error.is_none());
        assert!(terminal[1].passed);
        assert_eq!(terminal[2].value, Some(42.0));
        assert!(terminal[2].error.is_none());
        assert!(terminal[2].passed);
    }

    #[test]
    fn derivative_negative_two_ages_out_undefined_candidate_and_recovers() {
        let netlist = Netlist::parse(
            "* DERIV -2 recoverable undefined candidate\n\
             V1 out 0 0\n\
             V2 cond 0 0\n\
             .tran 1 5\n\
             .measure tran previous_slope DERIV V(out) WHEN V(cond)=0 CROSS=-2 DEFAULT_VAL=-7\n\
             .measure tran slope_live EQN previous_slope FROM=4 DEFAULT_VAL=-9\n\
             .end\n",
        )
        .expect("DERIV -2 recovery deck parses");
        let result = TransientResult {
            time: vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
            step_sizes: vec![0.0; 6],
            voltages: vec![
                vec![Value::INFINITY, Value::INFINITY, 0.0, 1.0, 3.0, 6.0],
                vec![-1.0, 1.0, -1.0, 1.0, -1.0, 1.0],
            ],
            branch_currents: Vec::new(),
            num_nodes: 2,
            node_names: vec!["out".to_string(), "cond".to_string()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        };

        let traces = evaluate_equation_measurements(
            &netlist,
            "TRAN",
            &result.time,
            &transient_signal_map(&result),
            -1.0,
            None,
        )
        .expect("DERIV -2 live traces recover");
        let trace = |name: &str| {
            traces
                .iter()
                .find(|trace| trace.name.eq_ignore_ascii_case(name))
                .expect("named live trace")
        };
        let slope = trace("previous_slope");
        assert_eq!(slope.values[0..2], [-7.0, -7.0]);
        assert!(slope.values[2].is_nan());
        assert_eq!(slope.values[3], Value::NEG_INFINITY);
        assert_eq!(slope.values[4..], [1.0, 2.0]);
        assert_eq!(slope.valid, vec![true, true, false, true, true, true]);
        assert_eq!(
            trace("slope_live").values,
            vec![-9.0, -9.0, -9.0, -9.0, 1.0, 2.0]
        );

        let terminal = evaluate_tran_measurements(&netlist, &result);
        assert_eq!(terminal[0].value, Some(2.0));
        assert_eq!(terminal[1].value, Some(2.0));
        assert!(terminal.iter().all(|result| result.passed));
    }

    #[test]
    fn point_state_remains_bounded_over_a_large_accepted_point_stream() {
        let point_count = 10_001;
        let axis = (0..point_count).map(|row| row as Value).collect::<Vec<_>>();
        let waveform = (0..point_count)
            .map(|row| if row % 2 == 0 { 0.0 } else { 1.0 })
            .collect::<Vec<_>>();
        let condition = |number| WhenCondition {
            left: "V(out)".to_string(),
            right: MeasureOperand::Constant(0.5),
            occurrence: crate::netlist::measure::EventOccurrence {
                edge: EdgeType::Cross,
                number,
            },
        };
        let statement = |name: &str, measure_type| MeasureStatement {
            default_value: Some(-7.0),
            fail_value: None,
            print_policy: crate::netlist::measure::MeasurePrintPolicy::All,
            name: name.to_string(),
            measure_type,
            analysis: "TRAN".to_string(),
            goal: None,
            tolerance: None,
        };
        let find = statement(
            "latest",
            MeasureType::Find {
                signal: "V(out)".to_string(),
                at: None,
                when: Some(condition(-1)),
                from: None,
                to: None,
                td: None,
                minval: 1.0e-12,
            },
        );
        let derivative = statement(
            "previous_slope",
            MeasureType::Derivative {
                signal: "V(out)".to_string(),
                at: None,
                when: Some(condition(-2)),
                from: None,
                to: None,
                td: None,
                minval: 1.0e-12,
            },
        );
        let params = crate::netlist::ParamContext::new();
        let mut find_state = compile_live_measure_state(&find, "TRAN", &axis, None, false, &params)
            .expect("large FIND state compiles");
        let mut derivative_state =
            compile_live_measure_state(&derivative, "TRAN", &axis, None, false, &params)
                .expect("large DERIV state compiles");
        let signals = HashMap::from([("V(out)".to_string(), waveform.as_slice())]);
        let signal_index = CanonicalMeasureSignalIndex::new(&signals);
        let mut programs = Vec::<LiveMeasureProgram<'_>>::new();
        let mut current_values = HashMap::new();
        let program_indices = HashMap::new();
        let mut latest_find = None;
        let mut latest_derivative = None;
        for (row, axis_value) in axis.iter().copied().enumerate() {
            let mut reads = LiveMeasureReadContext {
                programs: &mut programs,
                current_values: &mut current_values,
                program_indices: &program_indices,
                row,
                axis: &axis,
            };
            if let Some(value) = find_state
                .update(
                    row,
                    axis_value,
                    &axis,
                    false,
                    &signal_index,
                    &mut reads,
                    &params,
                    None,
                )
                .expect("large FIND state updates")
            {
                latest_find = Some(value);
            }
            let mut reads = LiveMeasureReadContext {
                programs: &mut programs,
                current_values: &mut current_values,
                program_indices: &program_indices,
                row,
                axis: &axis,
            };
            if let Some(value) = derivative_state
                .update(
                    row,
                    axis_value,
                    &axis,
                    false,
                    &signal_index,
                    &mut reads,
                    &params,
                    None,
                )
                .expect("large DERIV state updates")
            {
                latest_derivative = Some(value);
            }
        }
        assert_eq!(latest_find, Some(0.5));
        assert_eq!(latest_derivative, Some(1.0));

        let assert_point_bound = |state: &LiveMeasureState, distance: usize| match state {
            LiveMeasureState::Point {
                previous_signal,
                negative_results,
                condition: Some(condition),
                ..
            } => {
                assert!(previous_signal.is_some());
                assert!(negative_results.len() <= distance);
                assert!(condition.negative_events.len() <= distance);
            }
            _ => panic!("expected conditional Point state"),
        };
        assert_point_bound(&find_state, 1);
        assert_point_bound(&derivative_state, 2);

        let rise = statement(
            "unused_rise",
            MeasureType::RiseTime {
                signal: "V(out)".to_string(),
                from_pct: 0.1,
                to_pct: 0.9,
                number: 1,
            },
        );
        let file_error = statement(
            "unused_file_error",
            MeasureType::FileError {
                signal: "V(out)".to_string(),
                file: "virtual://measure/not-read.prn".to_string(),
                norm: crate::netlist::measure::FileErrorNorm::L2,
                independent_column: Some(1),
                dependent_column: 2,
            },
        );
        match compile_live_measure_state(&rise, "TRAN", &axis, None, false, &params)
            .expect("unreferenced RiseFall state compiles")
        {
            LiveMeasureState::RiseFall { samples, .. } => {
                assert!(samples.is_empty());
                assert_eq!(samples.capacity(), 0);
            }
            _ => panic!("expected RiseFall state"),
        }
        match compile_live_measure_state(&file_error, "TRAN", &axis, None, false, &params)
            .expect("unreferenced FileError state compiles")
        {
            LiveMeasureState::FileError { samples, .. } => {
                assert!(samples.is_empty());
                assert_eq!(samples.capacity(), 0);
            }
            _ => panic!("expected FileError state"),
        }

        let mut netlist = Netlist::parse(
            "large selective live state\n\
             V1 out 0 0\n\
             .tran 1 10000\n\
             .measure tran equation EQN V(out)\n\
             .end\n",
        )
        .expect("large selective-state deck parses");
        netlist
            .measurements
            .splice(0..0, [find.clone(), rise.clone(), file_error.clone()]);
        let internal =
            evaluate_equation_measurements(&netlist, "TRAN", &axis, &signals, -1.0, None)
                .expect("large selective live state evaluates");
        assert_eq!(internal.len(), 1);
        assert_eq!(internal[0].name, "EQUATION");
        assert_eq!(internal[0].values.len(), point_count);
    }

    #[test]
    fn file_error_freezes_only_on_a_real_measure_dependency_read() {
        let file = "virtual://measure/live-file-error.prn";
        let _ = crate::xspice::unregister_data_file(file);
        crate::xspice::register_data_file(
            file,
            "Index TIME REF\n0 0 0\n1 1 0\n2 2 0\nEnd of Xyce(TM) Simulation\n",
        )
        .expect("register live ERROR comparison table");
        let netlist = Netlist::parse(&format!(
            "live ERROR dependency reads\n\
             V1 out 0 0\n\
             .tran 1 2\n\
             .measure tran OUT ERROR V(out) FILE=\"{file}\" COMP_FUNCTION=L2NORM INDEPVARCOL=1 DEPVARCOL=2\n\
             .measure tran voltage EQN V(out)\n\
             .measure tran backward EQN OUT FROM=2 TO=2\n\
             .measure tran forward EQN LATER FROM=2 TO=2\n\
             .measure tran LATER ERROR V(out) FILE=\"{file}\" COMP_FUNCTION=L2NORM INDEPVARCOL=1 DEPVARCOL=2\n\
             .measure tran gated ERR OUT V(out) FROM=2 TO=2 MINVAL=1e-12\n\
             .measure tran gated_live EQN gated FROM=2 TO=2\n\
             .end\n"
        ))
        .expect("live ERROR dependency deck parses");
        let result = tran_waveform(vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 4.0]);

        let traces = evaluate_tran_equation_measurements(&netlist, &result)
            .expect("live ERROR traces evaluate");
        let trace = |name: &str| {
            &traces
                .iter()
                .find(|trace| trace.name.eq_ignore_ascii_case(name))
                .expect("named trace")
                .values
        };
        assert_eq!(trace("voltage"), &vec![0.0, 1.0, 4.0]);
        assert!((trace("backward")[2] - 17.0_f64.sqrt()).abs() < 1.0e-12);
        assert!(
            (trace("forward")[2] - 2.0_f64.sqrt()).abs() < 1.0e-12,
            "{traces:?}"
        );
        let expected_error = (17.0_f64.sqrt() - 4.0).abs() / 17.0_f64.sqrt();
        assert!((trace("gated_live")[2] - expected_error).abs() < 1.0e-12);

        let results = evaluate_tran_measurements(&netlist, &result);
        let result = |name: &str| {
            results
                .iter()
                .find(|result| result.name.eq_ignore_ascii_case(name))
                .expect("named result")
        };
        assert!(
            (result("OUT").value.unwrap() - 17.0_f64.sqrt()).abs() < 1.0e-12,
            "{results:?}"
        );
        assert!((result("LATER").value.unwrap() - 2.0_f64.sqrt()).abs() < 1.0e-12);
        crate::xspice::unregister_data_file(file).expect("unregister live ERROR table");
    }

    #[test]
    fn untraced_consumers_still_freeze_file_error_on_first_active_read() {
        let file = "virtual://measure/untraced-live-file-error.prn";
        let _ = crate::xspice::unregister_data_file(file);
        crate::xspice::register_data_file(
            file,
            "Index TIME REF\n0 0 0\n1 1 0\n2 2 0\nEnd of Xyce(TM) Simulation\n",
        )
        .expect("register untraced-consumer ERROR table");
        let netlist = Netlist::parse(&format!(
            "untraced ERROR dependency reads\n\
             V1 out 0 0\n\
             .tran 1 2\n\
             .measure tran EARLY ERROR V(out) FILE=\"{file}\" COMP_FUNCTION=L2NORM INDEPVARCOL=1 DEPVARCOL=2\n\
             .measure tran early_reader ERR EARLY V(out) FROM=0 TO=0 MINVAL=1e-12\n\
             .measure tran forward_reader ERR LATER V(out) FROM=0 TO=0 MINVAL=1e-12\n\
             .measure tran LATER ERROR V(out) FILE=\"{file}\" COMP_FUNCTION=L2NORM INDEPVARCOL=1 DEPVARCOL=2\n\
             .end\n"
        ))
        .expect("untraced-consumer ERROR deck parses");
        let result = tran_waveform(vec![0.0, 1.0, 2.0], vec![1.0, 2.0, 4.0]);

        let signals = transient_signal_map(&result);
        let internal =
            evaluate_equation_measurements(&netlist, "TRAN", &result.time, &signals, -1.0, None)
                .expect("untraced consumers preserve ERROR side effects");
        assert_eq!(
            internal
                .iter()
                .map(|trace| trace.name.as_str())
                .collect::<Vec<_>>(),
            vec!["EARLY", "LATER"]
        );
        let early_cached = 3.0_f64.sqrt();
        assert!(
            internal[0]
                .values
                .iter()
                .all(|value| (value - early_cached).abs() < 1.0e-12)
        );
        assert_eq!(internal[1].values, vec![0.0, 0.0, 0.0]);

        let results = evaluate_tran_measurements(&netlist, &result);
        let result = |name: &str| {
            results
                .iter()
                .find(|result| result.name.eq_ignore_ascii_case(name))
                .expect("named untraced-consumer result")
        };
        assert!(
            (result("EARLY").value.expect("EARLY cached value") - early_cached).abs() < 1.0e-12
        );
        assert_eq!(result("LATER").value, Some(0.0));
        crate::xspice::unregister_data_file(file)
            .expect("unregister untraced-consumer ERROR table");
    }

    #[test]
    fn lazy_expression_reads_freeze_only_the_condition_and_selected_arm() {
        let file = "virtual://measure/lazy-live-file-error.prn";
        let _ = crate::xspice::unregister_data_file(file);
        crate::xspice::register_data_file(
            file,
            "Index TIME REF\n0 0 0\n1 1 0\n2 2 0\nEnd of Xyce(TM) Simulation\n",
        )
        .expect("register lazy ERROR comparison table");
        let netlist = Netlist::parse(&format!(
            "lazy ERROR dependency reads\n\
             V1 out 0 0\n\
             .tran 1 2\n\
             .measure tran NEVER ERROR V(out) FILE=\"{file}\" COMP_FUNCTION=L2NORM INDEPVARCOL=1 DEPVARCOL=2\n\
             .measure tran never_reader EQN {{IF(TIME<0,NEVER,0)}}\n\
             .measure tran ARM ERROR V(out) FILE=\"{file}\" COMP_FUNCTION=L2NORM INDEPVARCOL=1 DEPVARCOL=2\n\
             .measure tran arm_reader EQN {{TIME>=1 ? ARM : 0}}\n\
             .measure tran CONDITION ERROR V(out) FILE=\"{file}\" COMP_FUNCTION=L2NORM INDEPVARCOL=1 DEPVARCOL=2\n\
             .measure tran condition_reader EQN {{IF(CONDITION>=0,1,0)}} FROM=1\n\
             .end\n"
        ))
        .expect("lazy ERROR dependency deck parses");
        let result = tran_waveform(vec![0.0, 1.0, 2.0], vec![1.0, 2.0, 4.0]);
        let signals = transient_signal_map(&result);
        let traces =
            evaluate_equation_measurements(&netlist, "TRAN", &result.time, &signals, -1.0, None)
                .expect("lazy ERROR dependencies evaluate");
        let trace = |name: &str| {
            &traces
                .iter()
                .find(|trace| trace.name.eq_ignore_ascii_case(name))
                .expect("named lazy ERROR trace")
                .values
        };

        assert_eq!(trace("NEVER"), &[0.0, 0.0, 0.0]);
        assert_eq!(trace("never_reader"), &[0.0, 0.0, 0.0]);
        for producer in ["ARM", "CONDITION"] {
            let values = trace(producer);
            assert_eq!(values[0], 0.0, "{producer} froze before its first read");
            assert!(values[1].is_finite() && values[1] > 0.0, "{values:?}");
            assert_eq!(values[2], values[1], "{producer} did not remain frozen");
        }
        assert_eq!(trace("arm_reader")[0], 0.0);
        assert_eq!(trace("arm_reader")[1], trace("ARM")[1]);
        assert_eq!(trace("condition_reader"), &[-1.0, 1.0, 1.0]);
        crate::xspice::unregister_data_file(file).expect("unregister lazy ERROR table");
    }

    #[test]
    fn file_error_forward_and_backward_getters_cache_raw_nan() {
        let file = "virtual://measure/raw-nan-live-file-error.prn";
        let _ = crate::xspice::unregister_data_file(file);
        crate::xspice::register_data_file(
            file,
            "Index TIME REF\n0 0 0\n1 1 0\nEnd of Xyce(TM) Simulation\n",
        )
        .expect("register raw-NaN ERROR table");
        let netlist = Netlist::parse(&format!(
            "raw NaN ERROR dependency reads\n\
             V1 source 0 0\n\
             V2 cond 0 0\n\
             .tran 1 2\n\
             .measure tran latest FIND V(source) WHEN V(cond)=0 CROSS=LAST DEFAULT_VAL=-7\n\
             .measure tran BACK ERROR latest FILE=\"{file}\" COMP_FUNCTION=L2NORM INDEPVARCOL=1 DEPVARCOL=2\n\
             .measure tran back_reader EQN BACK FROM=1\n\
             .measure tran braced_back EQN {{BACK}} FROM=1\n\
             .measure tran forward_reader EQN LATER FROM=2\n\
             .measure tran LATER ERROR latest FILE=\"{file}\" COMP_FUNCTION=L2NORM INDEPVARCOL=1 DEPVARCOL=2\n\
             .measure tran braced_later EQN {{LATER}} FROM=2\n\
             .measure tran independent EQN 42\n\
             .end\n"
        ))
        .expect("raw-NaN ERROR dependency deck parses");
        let result = TransientResult {
            time: vec![0.0, 1.0, 2.0],
            step_sizes: vec![0.0; 3],
            voltages: vec![
                vec![Value::INFINITY, Value::NEG_INFINITY, 0.0],
                vec![-1.0, 1.0, 0.0],
            ],
            branch_currents: Vec::new(),
            num_nodes: 2,
            node_names: vec!["source".to_string(), "cond".to_string()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        };

        let traces = evaluate_equation_measurements(
            &netlist,
            "TRAN",
            &result.time,
            &transient_signal_map(&result),
            -1.0,
            None,
        )
        .expect("raw-NaN ERROR getters evaluate");
        let trace = |name: &str| {
            &traces
                .iter()
                .find(|trace| trace.name.eq_ignore_ascii_case(name))
                .expect("named raw-NaN ERROR trace")
                .values
        };
        assert!(trace("BACK")[1..].iter().all(|value| value.is_nan()));
        assert!(trace("back_reader")[1..].iter().all(|value| value.is_nan()));
        assert_eq!(trace("braced_back")[1..], [1.0e50; 2]);
        assert!(trace("LATER")[2].is_nan());
        assert!(trace("forward_reader")[2].is_nan());
        assert_eq!(trace("braced_later")[2], 1.0e50);
        assert_eq!(trace("independent"), &vec![42.0; 3]);

        let terminal = evaluate_tran_measurements(&netlist, &result);
        let terminal = |name: &str| {
            terminal
                .iter()
                .find(|result| result.name.eq_ignore_ascii_case(name))
                .expect("named raw-NaN ERROR result")
        };
        assert!(terminal("BACK").value.is_some_and(Value::is_nan));
        assert!(terminal("back_reader").value.is_some_and(Value::is_nan));
        assert!(terminal("LATER").value.is_some_and(Value::is_nan));
        assert!(terminal("forward_reader").value.is_some_and(Value::is_nan));
        assert_eq!(terminal("braced_back").value, Some(1.0e50));
        assert_eq!(terminal("braced_later").value, Some(1.0e50));
        assert_eq!(terminal("independent").value, Some(42.0));
        crate::xspice::unregister_data_file(file).expect("unregister raw-NaN ERROR table");
    }

    #[test]
    fn issue_277_operator_letter_measure_names_feed_equations() {
        let netlist = Netlist::parse(
            "* Xyce issue 277\n\
             V1 1 0 0\n\
             .tran 0.5 1\n\
             .measure tran DMAX MAX V(1)\n\
             .measure tran IMAX MAX V(1)\n\
             .measure tran NMAX MAX V(1)\n\
             .measure tran PMAX MAX V(1)\n\
             .measure tran SMAX MAX V(1)\n\
             .measure tran VMAX MAX V(1)\n\
             .measure tran WMAX MAX V(1)\n\
             .measure tran YMAX1 MAX V(1)\n\
             .measure tran ZMAX MAX V(1)\n\
             .measure tran EQN1 EQN DMAX\n\
             .measure tran EQN2 EQN IMAX\n\
             .measure tran EQN3 EQN NMAX\n\
             .measure tran EQN4 EQN PMAX\n\
             .measure tran EQN5 EQN SMAX\n\
             .measure tran EQN6 EQN VMAX\n\
             .measure tran EQN7 EQN WMAX\n\
             .measure tran EQN8 EQN YMAX1\n\
             .measure tran EQN9 EQN ZMAX\n\
             .end\n",
        )
        .expect("issue 277 deck parses");
        let result = TransientResult {
            time: vec![0.0, 0.5, 1.0],
            step_sizes: vec![0.0; 3],
            voltages: vec![vec![0.0, 1.0, 0.0]],
            branch_currents: Vec::new(),
            num_nodes: 1,
            node_names: vec!["1".to_string()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        };

        let results = evaluate_tran_measurements(&netlist, &result);
        assert_eq!(results.len(), 18);
        assert!(results.iter().all(|result| result.passed), "{results:?}");
        assert!(
            results.iter().all(|result| result.value == Some(1.0)),
            "{results:?}"
        );
    }

    #[test]
    fn live_trace_storage_is_limited_to_equations_and_referenced_producers() {
        let netlist = Netlist::parse(
            "selective live traces\n\
             V1 out 0 0\n\
             .tran 1 1\n\
             .measure tran unused MAX V(out)\n\
             .measure tran used MAX V(out)\n\
             .measure tran consumer EQN used\n\
             .end\n",
        )
        .expect("selective trace deck parses");
        let result = tran_waveform(vec![0.0, 1.0], vec![1.0, 2.0]);
        let signals = transient_signal_map(&result);
        let internal =
            evaluate_equation_measurements(&netlist, "TRAN", &result.time, &signals, -1.0, None)
                .expect("internal live traces evaluate");
        assert_eq!(
            internal
                .iter()
                .map(|trace| trace.name.as_str())
                .collect::<Vec<_>>(),
            vec!["USED", "CONSUMER"]
        );
        let public = evaluate_tran_equation_measurements(&netlist, &result)
            .expect("public equation traces evaluate");
        assert_eq!(public.len(), 1);
        assert_eq!(public[0].name, "CONSUMER");

        let expression = crate::netlist::expr::parse_expression(
            "V(out)+VM(out)+I(V1)+DNO(M1,thermal)+DNI(M1)+gain",
        )
        .expect("probe dependency expression parses");
        let prepared =
            LivePreparedExpression::compile(&expression, &crate::netlist::ParamContext::new())
                .expect("probe dependency expression prepares");
        let mut dependencies = prepared
            .parameters
            .values()
            .map(|parameter| parameter.canonical_measure.clone())
            .collect::<Vec<_>>();
        dependencies.sort();
        dependencies.dedup();
        assert_eq!(dependencies, vec!["GAIN"]);
    }

    #[test]
    fn transient_continuous_adapter_retains_interpolated_events() {
        let netlist = Netlist::parse(
            "* continuous transient events\n\
             V1 out 0 0\n\
             .tran 1 3\n\
             .meas tran_cont crossing WHEN V(out)=1.5\n\
             .meas tran_cont slope DERIV V(out) AT=1.5\n\
             .end\n",
        )
        .expect("continuous transient measures parse");

        let results = evaluate_tran_continuous_measurements(&netlist, &tran_result());
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|result| result.failure.is_none()));
        assert_eq!(results[0].records.len(), 1);
        assert_eq!(results[0].records[0].value, 1.5);
        assert_eq!(results[0].records[0].event_axis, Some(1.5));
        assert_eq!(results[1].records.len(), 1);
        assert_eq!(results[1].records[0].value, 1.0);
        assert_eq!(results[1].records[0].event_axis, Some(1.5));
    }

    #[test]
    fn transient_measurements_materialize_direct_differential_voltage_operands() {
        let netlist = Netlist::parse(
            "* differential transient measurements\n\
             V1 X1.INTERNAL 0 0\n\
             V2 reference 0 0\n\
             .tran 1 3\n\
             .meas tran scalar WHEN V(X1:internal,reference)=1.5\n\
             .meas tran_cont continuous FIND V(X1:internal,0) WHEN V(X1:internal,reference)=1.5\n\
             .end\n",
        )
        .expect("differential transient measures parse");
        let mut result = tran_result();
        result.node_names[0] = "X1.INTERNAL".to_string();
        result.voltages.push(vec![0.0, 0.0, 0.0, 0.0]);
        result.node_names.push("reference".to_string());
        result.num_nodes = 2;

        let scalar = evaluate_tran_measurements(&netlist, &result);
        assert_eq!(scalar.len(), 1);
        assert!(scalar[0].passed, "{:?}", scalar[0].error);
        assert_eq!(scalar[0].value, Some(1.5));

        let continuous = evaluate_tran_continuous_measurements(&netlist, &result);
        assert_eq!(continuous.len(), 1);
        assert!(continuous[0].failure.is_none());
        assert_eq!(continuous[0].records.len(), 1);
        assert_eq!(continuous[0].records[0].value, 1.5);
        assert_eq!(continuous[0].records[0].event_axis, Some(1.5));
    }

    #[test]
    fn ac_series_exposes_derived_real_quantities() {
        let point = |freq: f64, voltage: crate::Complex64| AcResult {
            frequency: freq,
            node_names: vec!["out".to_string()],
            branch_names: vec![],
            voltages: vec![voltage],
            currents: vec![],
        };
        let sweep = vec![
            point(1.0, crate::Complex64::new(1.0, 0.0)),
            point(10.0, crate::Complex64::new(0.0, -1.0)),
        ];

        let series = AcSweepSeries::from_sweep(&sweep)
            .expect("AC result schema is valid")
            .expect("non-empty sweep");
        assert_eq!(series.axis(), &[1.0, 10.0]);
        let signals = series.signal_map();
        assert_eq!(signals["V(out)"], &[1.0, 1.0][..], "magnitude");
        assert_eq!(signals["VDB(out)"], &[0.0, 0.0][..], "decibels");
        assert_eq!(signals["VR(out)"], &[1.0, 0.0][..], "real part");
        assert_eq!(signals["VI(out)"], &[0.0, -1.0][..], "imaginary part");
        assert_eq!(signals["VP(out)"][1], -90.0, "phase in degrees");
        assert!(signals.contains_key("FREQUENCY"));
    }

    #[test]
    fn ac_series_rejects_missing_branch_values_instead_of_zero_filling() {
        let point = |frequency, current| AcResult {
            frequency,
            node_names: vec!["out".to_string()],
            branch_names: vec!["V1".to_string()],
            voltages: vec![crate::Complex64::new(1.0, 0.0)],
            currents: vec![current],
        };
        let first = point(1.0, crate::Complex64::new(2.0, 0.0));
        let mut malformed = point(2.0, crate::Complex64::new(3.0, 0.0));
        malformed.currents.clear();

        let error = AcSweepSeries::from_sweep(&[first, malformed])
            .err()
            .expect("a missing branch value must be a schema error");
        assert_eq!(error.descriptor().code.as_str(), "result_schema_mismatch");
        let SimulationError::ResultSchemaMismatch(detail) = error else {
            panic!("typed result-schema detail was lost");
        };
        assert_eq!(detail.analysis, "AC");
        assert_eq!(detail.signal_family, "branch currents");
        assert_eq!(detail.expected_value_count, 1);
        assert_eq!(detail.actual_value_count, 0);
        assert_eq!(
            detail.coordinate.as_deref(),
            Some("frequency point 1 (2.0000000000000000e0 Hz)")
        );
    }

    #[test]
    fn ac_series_rejects_changed_signal_names_with_the_same_shape() {
        let first = AcResult {
            frequency: 1.0,
            node_names: vec!["out".to_string()],
            branch_names: Vec::new(),
            voltages: vec![crate::Complex64::new(1.0, 0.0)],
            currents: Vec::new(),
        };
        let mut changed = first.clone();
        changed.frequency = 2.0;
        changed.node_names[0] = "other".to_string();

        let error = AcSweepSeries::from_sweep(&[first, changed])
            .err()
            .expect("a renamed signal must be a schema error");
        let SimulationError::ResultSchemaMismatch(detail) = error else {
            panic!("typed result-schema detail was lost");
        };
        assert_eq!(detail.signal_family, "node voltages");
        assert_eq!(detail.expected_names, ["out"]);
        assert_eq!(detail.actual_names, ["other"]);
    }

    #[test]
    fn ac_equations_resolve_hierarchical_derived_current_spellings() {
        let netlist = Netlist::parse(
            "hierarchical AC current equation\n\
             .AC LIN 2 1 2\n\
             .MEASURE AC currentEquation EQN {IR(X1:V1)+IM(X1:V1)}\n\
             .END\n",
        )
        .expect("hierarchical current equation parses");
        let point = |frequency, current| AcResult {
            frequency,
            node_names: Vec::new(),
            branch_names: vec!["X1.V1".to_string()],
            voltages: Vec::new(),
            currents: vec![current],
        };
        let sweep = vec![
            point(1.0, crate::Complex64::new(3.0, 4.0)),
            point(2.0, crate::Complex64::new(5.0, 12.0)),
        ];

        let traces = evaluate_ac_equation_measurements(&netlist, &sweep)
            .expect("hierarchical current equation evaluates");
        assert_eq!(traces[0].values, vec![8.0, 18.0]);

        let scalar = evaluate_ac_measurements(&netlist, &sweep);
        assert_eq!(scalar[0].value, Some(18.0));
        assert!(scalar[0].passed, "{scalar:?}");
    }

    #[test]
    fn ac_continuous_measurements_use_canonical_complex_projections() {
        let netlist = Netlist::parse(
            "AC continuous projections\n\
             V1 out 0 AC 1\n\
             .ac lin 3 1 3\n\
             .meas ac_cont samples FIND VR(out) WHEN VI(out)=0 CROSS=1\n\
             .end\n",
        )
        .expect("AC_CONT deck parses");
        let point = |frequency, real, imaginary| AcResult {
            frequency,
            node_names: vec!["out".to_string()],
            branch_names: Vec::new(),
            voltages: vec![crate::Complex64::new(real, imaginary)],
            currents: Vec::new(),
        };
        let sweep = vec![
            point(1.0, 0.0, -1.0),
            point(2.0, 10.0, 1.0),
            point(3.0, 40.0, -1.0),
        ];

        let results = evaluate_ac_continuous_measurements(&netlist, &sweep);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].failure, None);
        assert_eq!(
            results[0]
                .records
                .iter()
                .map(|record| (record.event_axis, record.value))
                .collect::<Vec<_>>(),
            vec![(Some(1.5), 5.0), (Some(2.5), 25.0)]
        );
    }

    #[test]
    fn ac_find_and_derivative_at_use_authored_minval_across_adapters() {
        let netlist = Netlist::parse(
            "AC accepted-row MINVAL parity\n\
             V1 out 0 AC 1\n\
             .ac lin 3 1 3\n\
             .measure ac found FIND VR(out) AT=2.05 MINVAL=0.1 DEFAULT_VAL=-7\n\
             .measure ac found_live EQN found\n\
             .measure ac slope DERIV VR(out) AT=2.05 MINVAL=0.1 DEFAULT_VAL=-7\n\
             .measure ac slope_live EQN slope\n\
             .measure ac_cont found_cont FIND VR(out) AT=2.05 MINVAL=0.1\n\
             .measure ac_cont slope_cont DERIV VR(out) AT=2.05 MINVAL=0.1\n\
             .end\n",
        )
        .expect("AC accepted-row MINVAL deck parses");
        let point = |frequency, real| AcResult {
            frequency,
            node_names: vec!["out".to_string()],
            branch_names: Vec::new(),
            voltages: vec![crate::Complex64::new(real, 0.0)],
            currents: Vec::new(),
        };
        let sweep = vec![point(1.0, 0.0), point(2.0, 10.0), point(3.0, 40.0)];

        let traces = evaluate_ac_equation_measurements(&netlist, &sweep)
            .expect("AC accepted-row live traces evaluate");
        assert_eq!(traces[0].values, vec![-7.0, 10.0, 10.0]);
        assert_eq!(traces[1].values, vec![-7.0, 10.0, 10.0]);

        let scalar = evaluate_ac_measurements(&netlist, &sweep);
        assert_eq!(scalar[0].value, Some(10.0));
        assert_eq!(scalar[1].value, Some(10.0));
        assert_eq!(scalar[2].value, Some(10.0));
        assert_eq!(scalar[3].value, Some(10.0));

        let continuous = evaluate_ac_continuous_measurements(&netlist, &sweep);
        assert_eq!(continuous.len(), 2);
        assert_eq!(continuous[0].records[0].value, 10.0);
        assert_eq!(continuous[1].records[0].value, 10.0);
    }

    #[test]
    fn ac_interface_aliases_cover_complex_equation_and_continuous_projections() {
        let netlist = Netlist::parse(
            "AC interface measurement paths\n\
             V1 1 0 AC 1\n\
             X1 1 0 CELL\n\
             .SUBCKT CELL A B\n\
             R1 A B 1\n\
             .ENDS\n\
             .AC LIN 2 1 2\n\
             .MEASURE AC real MAX V(X1:A)\n\
             .MEASURE AC magnitude MAX VM(X1:A)\n\
             .MEASURE AC imaginary MAX VI(X1:A)\n\
             .MEASURE AC phase MAX VP(X1:A)\n\
             .MEASURE AC decibels MAX VDB(X1:A)\n\
             .MEASURE AC groundVoltage MAX V(X1:B)\n\
             .MEASURE AC groundDecibels MAX VDB(X1:B)\n\
             .MEASURE AC equation EQN {V(X1:A)+VM(X1:A)}\n\
             .MEASURE AC spacedEquation EQN {VM( X1 : A )}\n\
             .MEASURE AC groundDbEquation EQN {VDB(X1:B)}\n\
             .MEASURE AC_CONT crossing WHEN VR(X1:A)=2\n\
             .END\n",
        )
        .expect("AC interface deck parses");
        let point = |frequency, real, imaginary| AcResult {
            frequency,
            node_names: vec!["1".to_string()],
            branch_names: Vec::new(),
            voltages: vec![crate::Complex64::new(real, imaginary)],
            currents: Vec::new(),
        };
        let sweep = vec![point(1.0, 1.0, 0.0), point(2.0, 3.0, 4.0)];

        let traces = evaluate_ac_equation_measurements(&netlist, &sweep)
            .expect("AC interface equation evaluates");
        assert_eq!(traces[0].values, vec![2.0, 8.0]);
        assert_eq!(traces[1].values, vec![1.0, 5.0]);
        assert_eq!(traces[2].values, vec![-1.0e50, -1.0e50]);

        let scalar = evaluate_ac_measurements(&netlist, &sweep);
        let result = |name: &str| {
            scalar
                .iter()
                .find(|result| result.name.eq_ignore_ascii_case(name))
                .expect("named AC interface measurement")
        };
        assert_eq!(result("real").value, Some(3.0));
        assert_eq!(result("magnitude").value, Some(5.0));
        assert_eq!(result("imaginary").value, Some(4.0));
        assert_eq!(result("phase").value, Some(4.0_f64.atan2(3.0).to_degrees()));
        assert_eq!(result("decibels").value, Some(20.0 * 5.0_f64.log10()));
        assert_eq!(result("groundVoltage").value, Some(0.0));
        assert_eq!(result("groundDecibels").value, Some(Value::NEG_INFINITY));
        assert!(result("groundDecibels").passed);
        assert_eq!(result("equation").value, Some(8.0));
        assert_eq!(result("spacedEquation").value, Some(5.0));
        assert_eq!(result("groundDbEquation").value, Some(-1.0e50));
        assert!(scalar.iter().all(|result| result.passed), "{scalar:?}");

        let continuous = evaluate_ac_continuous_measurements(&netlist, &sweep);
        assert_eq!(continuous.len(), 1);
        assert_eq!(continuous[0].failure, None);
        assert_eq!(continuous[0].records[0].event_axis, Some(1.5));
    }

    #[test]
    fn xyce_ac_ground_db_preserves_raw_infinity_and_saturates_equations() {
        let netlist = Netlist::parse_with_options(
            "Xyce ground decibel semantics\n\
             V1 1 0 AC 1\n\
             X1 1 0 CELL\n\
             .SUBCKT CELL A B\n\
             R1 A B 1\n\
             .ENDS\n\
             .AC LIN 2 1 2\n\
             .MEASURE AC raw MAX VDB(0)\n\
             .MEASURE AC bare_equation EQN VDB(0)\n\
             .MEASURE AC braced_equation EQN {VDB(0)}\n\
             .MEASURE AC bare_param PARAM VDB(0)\n\
             .MEASURE AC quoted_param PARAM='VDB(0)'\n\
             .END\n",
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("Xyce AC ground-decibel deck parses");
        let point = |frequency| AcResult {
            frequency,
            node_names: vec!["1".to_string()],
            branch_names: Vec::new(),
            voltages: vec![crate::Complex64::new(1.0, 0.0)],
            currents: Vec::new(),
        };
        let results = evaluate_ac_measurements(&netlist, &[point(1.0), point(2.0)]);
        let result = |name: &str| {
            results
                .iter()
                .find(|result| result.name.eq_ignore_ascii_case(name))
                .expect("named Xyce AC measurement")
        };

        assert_eq!(result("raw").value, Some(Value::NEG_INFINITY));
        assert_eq!(
            result("bare_equation").value,
            Some(Value::NEG_INFINITY),
            "{results:?}"
        );
        assert_eq!(
            result("braced_equation").value,
            Some(-1.0e50),
            "{results:?}"
        );
        assert_eq!(
            result("bare_param").value,
            Some(Value::NEG_INFINITY),
            "{results:?}"
        );
        assert_eq!(result("quoted_param").value, Some(-1.0e50), "{results:?}");
        assert!(results.iter().all(|result| result.passed), "{results:?}");
    }

    #[test]
    fn xyce_unbraced_equations_bind_raw_output_operator_families() {
        let netlist = Netlist::parse_with_options(
            "Xyce raw equation output operators\n\
             V1 1 0 AC 1\n\
             .AC LIN 2 1 2\n\
             .MEASURE AC raw_voltage EQN VDB(0) FROM=1 TO=2\n\
             .MEASURE AC raw_current EQN IDB(V1)\n\
             .MEASURE AC raw_dno EQN DNO(M1,thermal)\n\
             .MEASURE AC raw_dni EQN DNI(M1)\n\
             .MEASURE AC raw_device EQN N(X1:M1:id)\n\
             .MEASURE AC raw_power EQN P(R1)\n\
             .MEASURE AC raw_watt EQN W(R1)\n\
             .MEASURE AC raw_network EQN SDB(1,2)\n\
             .MEASURE AC braced_voltage EQN {VDB(0)}\n\
             .MEASURE AC quoted_voltage PARAM='VDB(0)'\n\
             .END\n",
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("Xyce raw equation operator deck parses");
        let axis = [1.0, 2.0];
        let negative_infinity = [Value::NEG_INFINITY; 2];
        let positive_infinity = [Value::INFINITY; 2];
        let device = [3.0, 4.0];
        let power = [5.0, 6.0];
        let watt = [7.0, 8.0];
        let network = [Value::INFINITY; 2];
        let signals = HashMap::from([
            ("VDB(0)".to_string(), negative_infinity.as_slice()),
            ("IDB(V1)".to_string(), positive_infinity.as_slice()),
            ("DNO(M1,thermal)".to_string(), positive_infinity.as_slice()),
            ("DNI(M1)".to_string(), negative_infinity.as_slice()),
            ("N(X1:M1:id)".to_string(), device.as_slice()),
            ("P(R1)".to_string(), power.as_slice()),
            ("W(R1)".to_string(), watt.as_slice()),
            ("SDB(1,2)".to_string(), network.as_slice()),
        ]);

        let traces = evaluate_equation_measurements(&netlist, "AC", &axis, &signals, -1.0, None)
            .expect("raw output-operator equations evaluate");
        let trace = |name: &str| {
            &traces
                .iter()
                .find(|trace| trace.name.eq_ignore_ascii_case(name))
                .expect("named raw output-operator trace")
                .values
        };
        assert_eq!(trace("raw_voltage"), &vec![Value::NEG_INFINITY; 2]);
        assert_eq!(trace("raw_current"), &vec![Value::INFINITY; 2]);
        assert_eq!(trace("raw_dno"), &vec![Value::INFINITY; 2]);
        assert_eq!(trace("raw_dni"), &vec![Value::NEG_INFINITY; 2]);
        assert_eq!(trace("raw_device"), &device);
        assert_eq!(trace("raw_power"), &power);
        assert_eq!(trace("raw_watt"), &watt);
        assert_eq!(trace("raw_network"), &vec![1.0e50; 2]);
        assert_eq!(trace("braced_voltage"), &vec![-1.0e50; 2]);
        assert_eq!(trace("quoted_voltage"), &vec![-1.0e50; 2]);
    }

    #[test]
    fn i_prefix_expression_builtins_bind_as_functions_not_current_accessors() {
        let netlist = Netlist::parse_with_options(
            "I-prefix builtins remain expression functions\n\
             V1 out 0 0\n\
             .TRAN 1 1\n\
             .MEASURE TRAN collision EQN {IF(V(out)>0,INT(V(out)),IMG(V(out)))}\n\
             .END\n",
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("I-prefix builtin equation parses");
        let axis = [0.0, 1.0];
        let waveform = [1.75, -2.25];
        let signals = HashMap::from([("V(out)".to_string(), waveform.as_slice())]);

        let traces = evaluate_equation_measurements(&netlist, "TRAN", &axis, &signals, -1.0, None)
            .expect("IF, INT, and IMG bind and evaluate as expression builtins");
        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].values, [1.0, 0.0]);
    }

    #[test]
    fn xyce_raw_current_spellings_use_direct_live_and_terminal_lookups() {
        let netlist = Netlist::parse_with_options(
            "typed raw current lookup\n\
             .tran 1 2\n\
             .measure tran legacy EQN I(YPDE BRANCH)\n\
             .measure tran collector EQN IC(Q1)\n\
             .measure tran hierarchical EQN IC(X1:Q1)\n\
             .end\n",
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("typed raw-current deck parses");
        let result = TransientResult {
            time: vec![0.0, 1.0, 2.0],
            step_sizes: vec![0.0; 3],
            voltages: Vec::new(),
            branch_currents: vec![vec![1.0, 2.0, 3.0]],
            num_nodes: 0,
            node_names: Vec::new(),
            branch_names: vec!["YPDE BRANCH".to_string()],
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: vec![
                TransientDeviceOpTrace {
                    device_name: "Q1".to_string(),
                    parameter: "ic".to_string(),
                    values: vec![4.0, 5.0, 6.0],
                },
                TransientDeviceOpTrace {
                    device_name: "X1:Q1".to_string(),
                    parameter: "IC".to_string(),
                    values: vec![7.0, 8.0, 9.0],
                },
            ],
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        };

        let traces = evaluate_tran_equation_measurements(&netlist, &result)
            .expect("typed raw currents evaluate live");
        assert_eq!(traces[0].values, [1.0, 2.0, 3.0]);
        assert_eq!(traces[1].values, [4.0, 5.0, 6.0]);
        assert_eq!(traces[2].values, [7.0, 8.0, 9.0]);
        let terminal = evaluate_tran_measurements(&netlist, &result);
        assert_eq!(terminal[0].value, Some(3.0));
        assert_eq!(terminal[1].value, Some(6.0));
        assert_eq!(terminal[2].value, Some(9.0));
        assert!(
            netlist
                .output_requests
                .iter()
                .any(|request| { request.selects_transient_device_current("X1:Q1") })
        );
    }

    #[test]
    fn xyce_differential_voltage_families_work_live_and_terminal() {
        let netlist = Netlist::parse_with_options(
            "differential voltage operators\n\
             .ac lin 2 1 2\n\
             .measure ac raw_vm EQN VM(a,b)\n\
             .measure ac raw_vdb EQN VDB(a,b)\n\
             .measure ac expression_vdb EQN {VDB(a,b)}\n\
             .end\n",
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("differential voltage equation deck parses");
        let point = |frequency, a: crate::Complex64, b: crate::Complex64| AcResult {
            frequency,
            node_names: vec!["a".to_string(), "b".to_string()],
            branch_names: Vec::new(),
            voltages: vec![a, b],
            currents: Vec::new(),
        };
        let sweep = [
            point(
                1.0,
                crate::Complex64::new(3.0, 4.0),
                crate::Complex64::new(1.0, 1.0),
            ),
            point(
                2.0,
                crate::Complex64::new(1.0, 1.0),
                crate::Complex64::new(1.0, 1.0),
            ),
        ];
        let traces = evaluate_ac_equation_measurements(&netlist, &sweep)
            .expect("differential voltage equations evaluate live");
        let trace = |name: &str| {
            &traces
                .iter()
                .find(|trace| trace.name.eq_ignore_ascii_case(name))
                .expect("named differential trace")
                .values
        };
        assert!((trace("raw_vm")[0] - 13.0_f64.sqrt()).abs() < 1.0e-12);
        assert_eq!(trace("raw_vm")[1], 0.0);
        assert!(trace("raw_vdb")[1].is_infinite() && trace("raw_vdb")[1].is_sign_negative());
        assert_eq!(trace("expression_vdb")[1], -1.0e50);

        let terminal = evaluate_ac_measurements(&netlist, &sweep);
        let value = |name: &str| {
            terminal
                .iter()
                .find(|result| result.name.eq_ignore_ascii_case(name))
                .expect("named differential terminal result")
                .value
        };
        assert_eq!(value("raw_vm"), Some(0.0));
        assert_eq!(value("raw_vdb"), Some(Value::NEG_INFINITY));
        assert_eq!(value("expression_vdb"), Some(-1.0e50));
    }

    #[test]
    fn ac_file_error_uses_akima_interpolation_and_extrapolation() {
        let file = "virtual://measure/ac-file-error.prn";
        let _ = crate::xspice::unregister_data_file(file);
        crate::xspice::register_data_file(
            file,
            "Index FREQ REF\n0 5 0.5\n1 15 1.5\n2 55 5.5\nEnd of Xyce(TM) Simulation\n",
        )
        .expect("register AC ERROR comparison table");
        let netlist = Netlist::parse(&format!(
            "AC file error interpolation\n\
             V1 out 0 AC 1\n\
             .ac lin 5 10 50\n\
             .measure ac fit ERROR VM(out) FILE=\"{file}\" INDEPVARCOL=1 DEPVARCOL=2\n\
             .end\n"
        ))
        .expect("AC ERROR deck parses");
        let point = |frequency, magnitude| AcResult {
            frequency,
            node_names: vec!["out".to_string()],
            branch_names: Vec::new(),
            voltages: vec![crate::Complex64::new(magnitude, 0.0)],
            currents: Vec::new(),
        };
        let results = evaluate_ac_measurements(
            &netlist,
            &[
                point(10.0, 1.0),
                point(20.0, 2.0),
                point(30.0, 3.0),
                point(40.0, 4.0),
                point(50.0, 5.0),
            ],
        );

        assert_eq!(results.len(), 1);
        assert!(results[0].value.is_some_and(|value| value.abs() < 1.0e-14));
        assert!(results[0].passed);
        crate::xspice::unregister_data_file(file).expect("unregister AC ERROR table");
    }

    #[test]
    fn ac_equations_use_xyce_accessors_frequency_aliases_and_windows() {
        let netlist = Netlist::parse(
            "* continuous AC equations\n\
             V1 out 0 AC 1\n\
             .ac lin 2 10 20\n\
             .meas ac bare EQN {1+V(out)}\n\
             .meas ac mag EQN {1+VM(out)}\n\
             .meas ac real EQN {1+VR(out)}\n\
             .meas ac imag EQN {1+VI(out)}\n\
             .meas ac phase EQN {1+VP(out)}\n\
             .meas ac db EQN {1+VDB(out)}\n\
             .meas ac current EQN {1+IM(V1)}\n\
             .meas ac freq EQN {FREQ}\n\
             .meas ac hertz EQN {HERTZ}\n\
             .meas ac bounded EQN {VM(out)} FROM=20 TO=20\n\
             .meas ac invalid EQN {VM(out)} FROM=30 TO=40\n\
             .meas ac expravg AVG {1+VR(out)}\n\
             .end\n",
        )
        .expect("AC equations parse");
        let point = |frequency, voltage, current| AcResult {
            frequency,
            node_names: vec!["out".to_string()],
            branch_names: vec!["V1".to_string()],
            voltages: vec![voltage],
            currents: vec![current],
        };
        let sweep = vec![
            point(
                10.0,
                crate::Complex64::new(3.0, 4.0),
                crate::Complex64::new(0.0, -2.0),
            ),
            point(
                20.0,
                crate::Complex64::new(0.0, -2.0),
                crate::Complex64::new(3.0, 4.0),
            ),
        ];

        let traces =
            evaluate_ac_equation_measurements(&netlist, &sweep).expect("AC equations evaluate");
        let trace = |name: &str| {
            traces
                .iter()
                .find(|trace| trace.name.eq_ignore_ascii_case(name))
                .expect("named trace")
        };

        assert_eq!(trace("bare").values, vec![4.0, 1.0]);
        assert_eq!(trace("mag").values, vec![6.0, 3.0]);
        assert_eq!(trace("real").values, vec![4.0, 1.0]);
        assert_eq!(trace("imag").values, vec![5.0, -1.0]);
        assert_eq!(
            trace("phase").values,
            vec![1.0 + 4.0_f64.atan2(3.0).to_degrees(), -89.0]
        );
        assert_eq!(
            trace("db").values,
            vec![1.0 + 20.0 * 5.0_f64.log10(), 1.0 + 20.0 * 2.0_f64.log10()]
        );
        assert_eq!(trace("current").values, vec![3.0, 6.0]);
        assert_eq!(trace("freq").values, vec![10.0, 20.0]);
        assert_eq!(trace("hertz").values, vec![10.0, 20.0]);
        assert_eq!(trace("bounded").values, vec![-1.0, 2.0]);
        assert!(!trace("invalid").initialized);
        assert_eq!(trace("invalid").values, vec![-1.0, -1.0]);

        let results = evaluate_ac_measurements(&netlist, &sweep);
        let result = |name: &str| {
            results
                .iter()
                .find(|result| result.name.eq_ignore_ascii_case(name))
                .expect("named measurement result")
        };
        assert_eq!(result("mag").value, Some(3.0));
        assert_eq!(result("bounded").value, Some(2.0));
        assert_eq!(result("invalid").value, None);
        assert_eq!(result("expravg").value, Some(2.5));
    }

    #[test]
    fn ac_point_events_observe_prior_continuous_equation_measures() {
        let netlist = Netlist::parse(
            "* prior AC equation references\n\
             V1 out 0 AC 1\n\
             .ac lin 3 1 3\n\
             .measure ac forward_crossing WHEN level=-0.5\n\
             .measure ac forward_found FIND VM(out) WHEN level=-0.5\n\
             .measure ac level EQN {VM(out)}\n\
             .measure ac crossing WHEN level=0.1\n\
             .measure ac found FIND VM(out) WHEN level=0.1\n\
             .end\n",
        )
        .expect("AC equation reference deck parses");
        let point = |frequency, magnitude| AcResult {
            frequency,
            node_names: vec!["out".to_string()],
            branch_names: Vec::new(),
            voltages: vec![crate::Complex64::new(magnitude, 0.0)],
            currents: Vec::new(),
        };
        let sweep = vec![point(1.0, 0.0), point(2.0, 0.05), point(3.0, 0.15)];

        let results = evaluate_ac_measurements(&netlist, &sweep);
        let result = |name: &str| {
            results
                .iter()
                .find(|result| result.name.eq_ignore_ascii_case(name))
                .expect("named AC measurement")
        };
        assert_eq!(result("forward_crossing").value, Some(1.5));
        assert_eq!(result("forward_found").value, Some(0.025));
        assert_eq!(result("level").value, Some(0.15));
        assert_eq!(result("crossing").value, Some(2.5));
        assert_eq!(result("found").value, Some(0.1));
        assert!(results.iter().all(|result| result.passed));
    }

    fn noise_point(
        frequency: Value,
        voltage: crate::Complex64,
        current: crate::Complex64,
    ) -> crate::analysis::NoiseResult {
        crate::analysis::NoiseResult {
            frequency,
            output_noise_density: 4.0,
            input_referred_density: 9.0,
            input_gain_squared: 4.0 / 9.0,
            contribution_catalog: Vec::new(),
            mechanisms_unavailable: Vec::new(),
            contributions: Vec::new(),
            node_names: vec!["out".to_string()],
            branch_names: vec!["V1".to_string()],
            voltages: vec![voltage],
            currents: vec![current],
        }
    }

    fn noise_point_with_contributions(
        frequency: Value,
        scale: Value,
    ) -> crate::analysis::NoiseResult {
        use crate::analysis::NoiseContribution;
        use crate::analysis::{NoiseSourceIdentity, NoiseSourceType};

        crate::analysis::NoiseResult {
            frequency,
            output_noise_density: 10.0 * scale,
            input_referred_density: 2.5 * scale,
            input_gain_squared: 4.0,
            contribution_catalog: vec![
                NoiseSourceIdentity::device("R4"),
                NoiseSourceIdentity::mechanism("Q1", "IB"),
                NoiseSourceIdentity::mechanism("Q1", "FN"),
            ],
            mechanisms_unavailable: Vec::new(),
            contributions: vec![
                NoiseContribution {
                    identity: NoiseSourceIdentity::device("r4"),
                    noise_type: NoiseSourceType::Thermal,
                    output_contribution: 4.0 * scale,
                    input_contribution: scale,
                    percentage: 0.0,
                },
                NoiseContribution {
                    identity: NoiseSourceIdentity::mechanism("Q1", "IB"),
                    noise_type: NoiseSourceType::Shot,
                    output_contribution: 2.0 * scale,
                    input_contribution: 0.5 * scale,
                    percentage: 0.0,
                },
                NoiseContribution {
                    identity: NoiseSourceIdentity::mechanism("q1", "ib"),
                    noise_type: NoiseSourceType::Shot,
                    output_contribution: 3.0 * scale,
                    input_contribution: 0.75 * scale,
                    percentage: 0.0,
                },
            ],
            node_names: Vec::new(),
            branch_names: Vec::new(),
            voltages: Vec::new(),
            currents: Vec::new(),
        }
    }

    #[test]
    fn noise_interface_aliases_cover_scalar_equation_and_continuous_paths() {
        let netlist = Netlist::parse(
            "NOISE interface measurement paths\n\
             X1 1 0 CELL\n\
             .SUBCKT CELL A B\n\
             R1 A B 1\n\
             .ENDS\n\
             .MEASURE NOISE real MAX V(X1:A)\n\
             .MEASURE NOISE magnitude MAX VM(X1:A)\n\
             .MEASURE NOISE equation EQN {V(X1:A)+VM(X1:A)}\n\
             .MEASURE NOISE groundDb MAX VDB(0)\n\
             .MEASURE NOISE_CONT crossing WHEN VR(X1:A)=2\n\
             .END\n",
        )
        .expect("NOISE interface deck parses");
        let mut first = noise_point(
            10.0,
            crate::Complex64::new(1.0, 0.0),
            crate::Complex64::new(0.0, 0.0),
        );
        first.node_names = vec!["1".to_string()];
        let mut second = noise_point(
            20.0,
            crate::Complex64::new(3.0, 4.0),
            crate::Complex64::new(0.0, 0.0),
        );
        second.node_names = vec!["1".to_string()];
        let sweep = vec![first, second];

        let traces = evaluate_noise_equation_measurements(&netlist, &sweep)
            .expect("NOISE interface equation evaluates");
        assert_eq!(traces[0].values, vec![2.0, 8.0]);

        let scalar = evaluate_noise_measurements(&netlist, &sweep);
        assert_eq!(
            scalar.iter().map(|result| result.value).collect::<Vec<_>>(),
            vec![Some(3.0), Some(5.0), Some(8.0), Some(Value::NEG_INFINITY)]
        );
        assert!(scalar.iter().all(|result| result.passed), "{scalar:?}");

        let continuous = evaluate_noise_continuous_measurements(&netlist, &sweep);
        assert_eq!(continuous.len(), 1);
        assert_eq!(continuous[0].failure, None);
        assert_eq!(continuous[0].records[0].event_axis, Some(15.0));
    }

    #[test]
    fn noise_series_exposes_complex_projections_and_noise_aliases() {
        let sweep = vec![
            noise_point(
                10.0,
                crate::Complex64::new(3.0, 4.0),
                crate::Complex64::new(0.0, -2.0),
            ),
            noise_point(
                20.0,
                crate::Complex64::new(0.0, -2.0),
                crate::Complex64::new(3.0, 4.0),
            ),
        ];

        let series = NoiseSweepSeries::from_sweep(&sweep)
            .expect("noise series is valid")
            .expect("non-empty noise sweep");
        let signals = series.signal_map();
        assert_eq!(signals["VM(out)"], &[5.0, 2.0]);
        assert_eq!(signals["VR(out)"], &[3.0, 0.0]);
        assert_eq!(signals["VI(out)"], &[4.0, -2.0]);
        assert_eq!(signals["VP(out)"][0], 4.0_f64.atan2(3.0).to_degrees());
        assert_eq!(signals["VDB(out)"][0], 20.0 * 5.0_f64.log10());
        assert_eq!(signals["IM(V1)"], &[2.0, 5.0]);
        assert_eq!(signals["IR(V1)"], &[0.0, 3.0]);
        assert_eq!(signals["II(V1)"], &[-2.0, 4.0]);
        assert_eq!(signals["IP(V1)"], &[-90.0, 4.0_f64.atan2(3.0).to_degrees()]);
        assert_eq!(
            signals["IDB(V1)"],
            &[20.0 * 2.0_f64.log10(), 20.0 * 5.0_f64.log10()]
        );
        assert_eq!(signals["ONOISE"], &[4.0, 4.0]);
        assert_eq!(signals["ONOISE_SPECTRUM"], &[4.0, 4.0]);
        assert_eq!(signals["INOISE"], &[9.0, 9.0]);
        assert_eq!(signals["INOISE_SPECTRUM"], &[9.0, 9.0]);
        assert_eq!(signals["HERTZ"], &[10.0, 20.0]);

        let equation_signals = series.equation_signal_map();
        assert_eq!(equation_signals["V(out)"], &[3.0, 0.0]);
        assert_eq!(equation_signals["I(V1)"], &[0.0, 3.0]);
    }

    #[test]
    fn noise_series_rejects_missing_node_values_instead_of_zero_filling() {
        let first = noise_point(
            10.0,
            crate::Complex64::new(1.0, 0.0),
            crate::Complex64::new(2.0, 0.0),
        );
        let mut malformed = noise_point(
            20.0,
            crate::Complex64::new(3.0, 0.0),
            crate::Complex64::new(4.0, 0.0),
        );
        malformed.voltages.clear();

        let error = NoiseSweepSeries::from_sweep(&[first, malformed])
            .err()
            .expect("a missing node value must be a schema error");
        assert_eq!(error.descriptor().code.as_str(), "result_schema_mismatch");
        let SimulationError::ResultSchemaMismatch(detail) = error else {
            panic!("typed result-schema detail was lost");
        };
        assert_eq!(detail.analysis, "NOISE");
        assert_eq!(detail.signal_family, "node voltages");
        assert_eq!(detail.expected_value_count, 1);
        assert_eq!(detail.actual_value_count, 0);
    }

    #[test]
    fn noise_series_exposes_dno_dni_device_and_mechanism_contributions() {
        let sweep = vec![
            noise_point_with_contributions(10.0, 1.0),
            noise_point_with_contributions(20.0, 2.0),
        ];
        let series = NoiseSweepSeries::from_sweep(&sweep)
            .expect("contribution catalogs remain stable")
            .expect("non-empty noise sweep");
        let signals = series.signal_map();

        assert_eq!(signals["DNO(R4)"], &[4.0, 8.0]);
        assert_eq!(signals["DNI(R4)"], &[1.0, 2.0]);
        assert_eq!(signals["DNO(Q1)"], &[5.0, 10.0]);
        assert_eq!(signals["DNI(Q1)"], &[1.25, 2.5]);
        assert_eq!(signals["DNO(Q1,IB)"], &[5.0, 10.0]);
        assert_eq!(signals["DNI(Q1,IB)"], &[1.25, 2.5]);
        assert_eq!(signals["DNO(Q1,FN)"], &[0.0, 0.0]);
        assert!(!signals.contains_key("DNO(R4,R4)"));
    }

    #[test]
    fn noise_measurements_bind_direct_braced_and_equation_dno_dni() {
        let netlist = Netlist::parse(
            "* noise contribution measures\n\
             .measure noise direct AVG DNI(r4)\n\
             .measure noise braced AVG {DNI(R4)}\n\
             .measure noise mechanism AVG {DNO(q1, ib)}\n\
             .measure noise equation EQN {DNO(Q1,IB)+DNI(q1,ib)}\n\
             .end\n",
        )
        .expect("noise contribution measurement deck parses");
        let sweep = vec![
            noise_point_with_contributions(10.0, 1.0),
            noise_point_with_contributions(20.0, 2.0),
        ];

        let results = evaluate_noise_measurements(&netlist, &sweep);
        let result = |name: &str| {
            results
                .iter()
                .find(|result| result.name.eq_ignore_ascii_case(name))
                .expect("named noise measurement")
        };
        assert_eq!(result("direct").value, Some(1.5));
        assert_eq!(result("braced").value, Some(1.5));
        assert_eq!(result("mechanism").value, Some(7.5));
        assert_eq!(result("equation").value, Some(12.5));
        assert!(results.iter().all(|result| result.passed), "{results:#?}");
    }

    #[test]
    fn noise_measurements_resolve_hierarchical_contribution_spellings() {
        let netlist = Netlist::parse(
            "hierarchical noise contribution measures\n\
             .MEASURE NOISE direct AVG DNO(X1:R4)\n\
             .MEASURE NOISE equation EQN {DNO(X1:R4)+DNI(X1:R4)}\n\
             .END\n",
        )
        .expect("hierarchical noise measurement deck parses");
        let mut sweep = vec![
            noise_point_with_contributions(10.0, 1.0),
            noise_point_with_contributions(20.0, 2.0),
        ];
        for point in &mut sweep {
            for identity in &mut point.contribution_catalog {
                identity.device = format!("X1.{}", identity.device);
            }
            for contribution in &mut point.contributions {
                contribution.identity.device = format!("X1.{}", contribution.identity.device);
            }
        }

        let results = evaluate_noise_measurements(&netlist, &sweep);

        assert_eq!(results[0].value, Some(6.0));
        assert_eq!(results[1].value, Some(10.0));
        assert!(results.iter().all(|result| result.passed), "{results:#?}");
    }

    #[test]
    fn noise_series_rejects_frequency_dependent_contribution_catalogs() {
        let mut changed = noise_point_with_contributions(20.0, 2.0);
        changed.contribution_catalog.pop();
        let error =
            NoiseSweepSeries::from_sweep(&[noise_point_with_contributions(10.0, 1.0), changed])
                .err()
                .expect("catalog mismatch must fail closed");
        let SimulationError::ResultSchemaMismatch(detail) = error else {
            panic!("catalog mismatch must retain typed schema detail");
        };
        assert_eq!(detail.analysis, "NOISE");
        assert_eq!(detail.signal_family, "noise contribution catalog");
        assert_eq!(
            detail.coordinate.as_deref(),
            Some("frequency point 1 (2.0000000000000000e1 Hz)")
        );
    }

    #[test]
    fn noise_equations_use_frequency_accessors_defaults_and_statement_order() {
        let netlist = Netlist::parse(
            "* continuous NOISE equations\n\
             .meas noise first EQN {VR(out)+FREQ}\n\
             .meas noise follows EQN {first+HERTZ}\n\
             .meas noise forward EQN {later+1}\n\
             .meas noise later EQN {FREQ}\n\
             .meas noise when_first WHEN first=16.5\n\
             .meas noise find_first FIND VM(out) WHEN first=16.5\n\
             .meas noise bounded EQN {VM(out)} FROM=20 TO=20\n\
             .meas noise invalid EQN {VM(out)} FROM=30 TO=40 DEFAULT_VAL=-9\n\
             .end\n",
        )
        .expect("NOISE equations parse");
        let sweep = vec![
            noise_point(
                10.0,
                crate::Complex64::new(3.0, 4.0),
                crate::Complex64::new(0.0, -2.0),
            ),
            noise_point(
                20.0,
                crate::Complex64::new(0.0, -2.0),
                crate::Complex64::new(3.0, 4.0),
            ),
        ];

        let traces = evaluate_noise_equation_measurements(&netlist, &sweep)
            .expect("NOISE equations evaluate");
        let trace = |name: &str| {
            traces
                .iter()
                .find(|trace| trace.name.eq_ignore_ascii_case(name))
                .expect("named trace")
        };
        assert_eq!(trace("first").values, vec![13.0, 20.0]);
        assert_eq!(trace("follows").values, vec![23.0, 40.0]);
        assert_eq!(trace("forward").values, vec![0.0, 11.0]);
        assert_eq!(trace("later").values, vec![10.0, 20.0]);
        assert_eq!(trace("bounded").values, vec![-1.0, 2.0]);
        assert_eq!(trace("invalid").values, vec![-9.0, -9.0]);
        assert!(!trace("invalid").initialized);

        let results = evaluate_noise_measurements(&netlist, &sweep);
        let result = |name: &str| {
            results
                .iter()
                .find(|result| result.name.eq_ignore_ascii_case(name))
                .expect("named measurement result")
        };
        assert_eq!(result("follows").value, Some(40.0));
        assert_eq!(result("forward").value, Some(11.0));
        assert_eq!(result("when_first").value, Some(15.0));
        assert_eq!(result("find_first").value, Some(3.5));
        assert_eq!(result("bounded").value, Some(2.0));
        assert!(!result("invalid").passed);
        assert_eq!(result("invalid").value, None);
    }

    #[test]
    fn dc_series_uses_sweep_as_axis() {
        let mut point = SimulationResult::new(1, 0);
        point.node_voltages = vec![0.0, 2.5];
        point.node_names = vec!["0".to_string(), "out".to_string()];
        let sweep = vec![(0.0, point.clone()), (5.0, point)];

        let series = DcSweepSeries::from_sweep(&sweep)
            .expect("DC result schema is valid")
            .expect("non-empty sweep");
        assert_eq!(series.axis(), &[0.0, 5.0]);
        let signals = series.signal_map();
        assert!(signals.contains_key("TIME"));
        assert_eq!(signals["V(out)"], &[2.5, 2.5][..]);
    }

    #[test]
    fn dc_series_rejects_missing_node_values_instead_of_zero_filling() {
        let mut first = SimulationResult::new(1, 0);
        first.node_names = vec!["0".to_string(), "out".to_string()];
        first.node_voltages = vec![0.0, 1.0];
        let mut malformed = first.clone();
        malformed.node_voltages.pop();

        let error = DcSweepSeries::from_sweep(&[(0.0, first), (1.0, malformed)])
            .err()
            .expect("a missing node value must be a schema error");
        assert_eq!(error.descriptor().code.as_str(), "result_schema_mismatch");
        let SimulationError::ResultSchemaMismatch(detail) = error else {
            panic!("typed result-schema detail was lost");
        };
        assert_eq!(detail.analysis, "DC");
        assert_eq!(detail.signal_family, "node voltages");
        assert_eq!(detail.expected_value_count, 2);
        assert_eq!(detail.actual_value_count, 1);
        assert_eq!(
            detail.coordinate.as_deref(),
            Some("sweep point 1 (1.0000000000000000e0)")
        );
    }

    #[test]
    fn dc_series_rejects_changed_branch_schema_with_the_same_shape() {
        let mut first = SimulationResult::new(0, 1);
        first.branch_names = vec!["V1".to_string()];
        first.branch_currents = vec![1.0];
        let mut changed = first.clone();
        changed.branch_names[0] = "V2".to_string();

        let error = DcSweepSeries::from_sweep(&[(0.0, first), (1.0, changed)])
            .err()
            .expect("a renamed branch must be a schema error");
        let SimulationError::ResultSchemaMismatch(detail) = error else {
            panic!("typed result-schema detail was lost");
        };
        assert_eq!(detail.signal_family, "branch currents");
        assert_eq!(detail.expected_names, ["V1"]);
        assert_eq!(detail.actual_names, ["V2"]);
    }

    #[test]
    fn global_measure_default_overrides_equation_local_default() {
        let netlist = Netlist::parse(
            "global equation default\n\
             V1 out 0 0\n\
             .options measure default_val=-10\n\
             .dc V1 0 1 1\n\
             .measure dc outside EQN {V(out)} FROM=2 TO=3 DEFAULT_VAL=2\n\
             .end\n",
        )
        .expect("global measurement default deck parses");
        let sweep = [0.0, 1.0]
            .into_iter()
            .map(|axis| {
                let mut point = SimulationResult::new(1, 0);
                point.node_voltages = vec![0.0, axis];
                point.node_names = vec!["0".to_string(), "out".to_string()];
                (axis, point)
            })
            .collect::<Vec<_>>();

        let traces =
            evaluate_dc_equation_measurements(&netlist, &sweep).expect("equation trace evaluates");
        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].values, vec![-10.0, -10.0]);
        assert!(!traces[0].initialized);
    }

    #[test]
    fn programmatic_dc_sweeps_do_not_require_a_deck_dc_card() {
        let netlist = Netlist::parse(
            "OP-only DC measurement\n\
             V1 out 0 1\n\
             .op\n\
             .measure dc maximum MAX V(out)\n\
             .end\n",
        )
        .expect("OP-only measurement deck parses");
        let mut point = SimulationResult::new(1, 0);
        point.node_voltages = vec![0.0, 1.0];
        point.node_names = vec!["0".to_string(), "out".to_string()];

        let results = evaluate_dc_measurements(&netlist, &[(0.0, point)]);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].value, Some(1.0));
        assert!(results[0].passed);
    }

    #[test]
    fn dc_measurements_observe_point_local_parameter_contexts() {
        let netlist = Netlist::parse(
            "row-local parameters\n\
             .param P=0\n\
             V1 out 0 0\n\
             .dc V1 1 2 1\n\
             .measure dc average AVG {P}\n\
             .end\n",
        )
        .expect("parameter measurement deck parses");
        let sweep = [1.0, 2.0]
            .into_iter()
            .map(|axis| {
                let mut point = SimulationResult::new(1, 0);
                point.node_voltages = vec![0.0, 0.0];
                point.node_names = vec!["0".to_string(), "out".to_string()];
                (axis, point)
            })
            .collect::<Vec<_>>();
        let contexts = [1.0, 2.0]
            .into_iter()
            .map(|value| {
                let mut params = netlist.params.clone();
                params.set("P", value);
                params
            })
            .collect::<Vec<_>>();

        let results = evaluate_dc_measurements_with_parameter_contexts(&netlist, &sweep, &contexts);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].value, Some(1.5));
        assert!(results[0].passed);
    }

    #[test]
    fn dc_continuous_measurements_observe_point_local_parameter_contexts() {
        let netlist = Netlist::parse(
            "DC continuous row-local parameters\n\
             .param P=0\n\
             V1 out 0 0\n\
             .dc V1 0 2 1\n\
             .meas dc_cont sampled FIND {P*2} WHEN V(out)=0 CROSS=1\n\
             .end\n",
        )
        .expect("DC_CONT parameter deck parses");
        let voltages = [-1.0, 1.0, -1.0];
        let sweep = voltages
            .into_iter()
            .enumerate()
            .map(|(axis, voltage)| {
                let mut point = SimulationResult::new(1, 0);
                point.node_voltages = vec![0.0, voltage];
                point.node_names = vec!["0".to_string(), "out".to_string()];
                (axis as Value, point)
            })
            .collect::<Vec<_>>();
        let contexts = [1.0, 3.0, 5.0]
            .into_iter()
            .map(|value| {
                let mut params = netlist.params.clone();
                params.set("P", value);
                params
            })
            .collect::<Vec<_>>();

        let results = evaluate_dc_continuous_measurements_with_parameter_contexts(
            &netlist, &sweep, &contexts,
        );

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].failure, None);
        assert_eq!(
            results[0]
                .records
                .iter()
                .map(|record| (record.event_axis, record.value))
                .collect::<Vec<_>>(),
            vec![(Some(0.5), 4.0), (Some(1.5), 8.0)]
        );
    }

    #[test]
    fn dc_continuous_measurements_do_not_interpolate_across_nested_sweep_restarts() {
        let netlist = Netlist::parse(
            "nested DC continuous events\n\
             V1 out 0 0\n\
             V2 bias 0 0\n\
             .dc V1 0 2 1 V2 0 1 1\n\
             .meas dc_cont crossings WHEN V(out)=0 CROSS=1\n\
             .end\n",
        )
        .expect("nested DC_CONT deck parses");
        let sweep = [
            (0.0, -1.0),
            (1.0, 1.0),
            (2.0, 1.0),
            (0.0, -1.0),
            (1.0, 1.0),
            (2.0, 1.0),
        ]
        .into_iter()
        .map(|(axis, voltage)| {
            let mut point = SimulationResult::new(1, 0);
            point.node_voltages = vec![0.0, voltage];
            point.node_names = vec!["0".to_string(), "out".to_string()];
            (axis, point)
        })
        .collect::<Vec<_>>();

        let results = evaluate_dc_continuous_measurements(&netlist, &sweep);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].failure, None);
        assert_eq!(
            results[0]
                .records
                .iter()
                .map(|record| record.value)
                .collect::<Vec<_>>(),
            vec![0.5, 0.5]
        );
    }

    #[test]
    fn dc_average_supports_differential_ground_and_reversed_decreasing_windows() {
        let netlist = Netlist::parse(
            "differential DC average\n\
             V1 out 0 0\n\
             .dc V1 3 1 -1\n\
             .meas dc reversed AVG V(GND,out) FROM=2 TO=1\n\
             .meas dc from_only AVG V(GND,out) FROM=2\n\
             .meas dc to_only AVG V(GND,out) TO=2\n\
             .end\n",
        )
        .expect("DC average parses");
        let sweep = (1..=3)
            .rev()
            .map(|axis| {
                let mut point = SimulationResult::new(1, 0);
                point.node_voltages = vec![0.0, axis as Value];
                point.node_names = vec!["0".to_string(), "out".to_string()];
                (axis as Value, point)
            })
            .collect::<Vec<_>>();

        let results = evaluate_dc_measurements(&netlist, &sweep);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].value, Some(-1.5));
        assert_eq!(results[1].value, Some(-1.5));
        assert_eq!(results[2].value, Some(-2.5));
        assert!(results.iter().all(|result| result.passed));
    }

    #[test]
    fn dc_rms_uses_absolute_width_with_descending_one_sided_windows() {
        let netlist = Netlist::parse(
            "descending DC RMS\n\
             V1 out 0 0\n\
             .dc V1 3 1 -1\n\
             .meas dc all RMS V(out)\n\
             .meas dc from_only RMS V(out) FROM=2\n\
             .meas dc to_only RMS V(out) TO=2\n\
             .meas dc reversed RMS V(out) FROM=2 TO=3\n\
             .end\n",
        )
        .expect("DC RMS parses");
        let sweep = (1..=3)
            .rev()
            .map(|axis| {
                let mut point = SimulationResult::new(1, 0);
                point.node_voltages = vec![0.0, axis as Value];
                point.node_names = vec!["0".to_string(), "out".to_string()];
                (axis as Value, point)
            })
            .collect::<Vec<_>>();

        let results = evaluate_dc_measurements(&netlist, &sweep);
        let expected = [
            4.5_f64.sqrt(),
            2.5_f64.sqrt(),
            6.5_f64.sqrt(),
            6.5_f64.sqrt(),
        ];
        for (result, expected) in results.iter().zip(expected) {
            assert_eq!(result.value, Some(expected));
            assert!(result.passed);
        }
    }

    #[test]
    fn dc_integration_preserves_requested_and_sweep_directions() {
        let netlist = Netlist::parse(
            "directional DC integration\n\
             V1 out 0 0\n\
             .dc V1 5 1 -1\n\
             .meas dc all INTEG V(out)\n\
             .meas dc from_only INTEG V(out) FROM=4\n\
             .meas dc to_only INTEG V(out) TO=3\n\
             .meas dc forward INTEG V(out) FROM=4 TO=2\n\
             .meas dc reverse INTEG V(out) FROM=2 TO=4\n\
             .end\n",
        )
        .expect("DC integration parses");
        let sweep = (1..=5)
            .rev()
            .map(|axis| {
                let mut point = SimulationResult::new(1, 0);
                point.node_voltages = vec![0.0, axis as Value];
                point.node_names = vec!["0".to_string(), "out".to_string()];
                (axis as Value, point)
            })
            .collect::<Vec<_>>();

        let results = evaluate_dc_measurements(&netlist, &sweep);
        assert_eq!(
            results
                .iter()
                .map(|result| result.value)
                .collect::<Vec<_>>(),
            vec![Some(-12.0), Some(-7.5), Some(-8.0), Some(-6.0), Some(6.0)]
        );
        assert!(results.iter().all(|result| result.passed));
    }

    #[test]
    fn dc_average_retains_state_across_secondary_sweep_cycles() {
        let netlist = Netlist::parse(
            "nested DC average\n\
             V1 out 0 0\n\
             V2 bias 0 0\n\
             .dc V1 3 1 -1 V2 0 1 1\n\
             .meas dc combined AVG V(out)\n\
             .end\n",
        )
        .expect("nested DC average parses");
        let sweep = [
            (3.0, 9.0),
            (2.0, 4.0),
            (1.0, 1.0),
            (3.0, 19.0),
            (2.0, 14.0),
            (1.0, 11.0),
        ]
        .into_iter()
        .map(|(axis, voltage)| {
            let mut point = SimulationResult::new(1, 0);
            point.node_voltages = vec![0.0, voltage];
            point.node_names = vec!["0".to_string(), "out".to_string()];
            (axis, point)
        })
        .collect::<Vec<_>>();

        let results = evaluate_dc_measurements(&netlist, &sweep);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].value, Some(58.0 / 6.0));
        assert!(results[0].passed);
    }

    #[test]
    fn nested_dc_live_aggregates_continue_but_events_respect_segment_barriers() {
        let netlist = Netlist::parse(
            "nested DC live state\n\
             V1 out 0 0\n\
             V2 bias 0 0\n\
             .dc V1 3 1 -1 V2 0 1 1\n\
             .measure dc combined AVG V(out)\n\
             .measure dc combined_live EQN combined\n\
             .measure dc barrier WHEN V(out)=10 DEFAULT_VAL=-7\n\
             .measure dc barrier_live EQN barrier\n\
             .end\n",
        )
        .expect("nested DC live-state deck parses");
        let sweep = [
            (3.0, 9.0),
            (2.0, 4.0),
            (1.0, 1.0),
            (3.0, 19.0),
            (2.0, 14.0),
            (1.0, 11.0),
        ]
        .into_iter()
        .map(|(axis, voltage)| {
            let mut point = SimulationResult::new(1, 0);
            point.node_voltages = vec![0.0, voltage];
            point.node_names = vec!["0".to_string(), "out".to_string()];
            (axis, point)
        })
        .collect::<Vec<_>>();

        let traces = evaluate_dc_equation_measurements(&netlist, &sweep)
            .expect("nested DC equation traces evaluate");
        let trace = |name: &str| {
            &traces
                .iter()
                .find(|trace| trace.name.eq_ignore_ascii_case(name))
                .expect("named trace")
                .values
        };
        let expected_average = [0.0, 6.5, 4.5, 7.25, 9.1, 58.0 / 6.0];
        for (actual, expected) in trace("combined_live").iter().zip(expected_average) {
            assert!((actual - expected).abs() < 1.0e-12);
        }
        assert_eq!(trace("barrier_live"), &vec![-7.0; 6]);

        let results = evaluate_dc_measurements(&netlist, &sweep);
        assert_eq!(results[0].value, Some(58.0 / 6.0));
        assert_eq!(results[1].value, Some(58.0 / 6.0));
        assert!(!results[2].passed, "barrier event must not bridge segments");
        assert_eq!(results[3].value, Some(-7.0));
    }

    #[test]
    fn dc_find_at_exact_later_segment_start_publishes_current_sample() {
        let netlist = Netlist::parse(
            "nested DC segment-start FIND\n\
             V1 out 0 0\n\
             V2 bias 0 0\n\
             .dc V1 0 1 1 V2 0 1 1\n\
             .measure dc sample FIND V(out) AT=2 DEFAULT_VAL=-7\n\
             .measure dc sample_live EQN sample\n\
             .measure dc near FIND V(out) AT=2.0000000000005 DEFAULT_VAL=-7\n\
             .measure dc near_live EQN near\n\
             .end\n",
        )
        .expect("nested DC segment-start FIND deck parses");
        let sweep = [(0.0, 10.0), (1.0, 11.0), (2.0, 20.0), (3.0, 21.0)]
            .into_iter()
            .map(|(axis, voltage)| {
                let mut point = SimulationResult::new(1, 0);
                point.node_voltages = vec![0.0, voltage];
                point.node_names = vec!["0".to_string(), "out".to_string()];
                (axis, point)
            })
            .collect::<Vec<_>>();

        let traces = evaluate_dc_equation_measurements(&netlist, &sweep)
            .expect("nested DC segment-start FIND traces evaluate");
        assert_eq!(traces.len(), 2);
        assert_eq!(traces[0].values, vec![-7.0, -7.0, 20.0, 20.0]);
        assert_eq!(traces[1].values, vec![-7.0, -7.0, 20.0, 20.0]);

        let results = evaluate_dc_measurements(&netlist, &sweep);
        assert_eq!(results[0].value, Some(20.0));
        assert_eq!(results[1].value, Some(20.0));
        assert_eq!(results[2].value, Some(20.0));
        assert_eq!(results[3].value, Some(20.0));
        assert!(results.iter().all(|result| result.passed));
    }

    #[test]
    fn reversed_dc_windows_apply_to_live_extrema_and_statistics() {
        let netlist = Netlist::parse(
            "reversed DC live windows\n\
             V1 out 0 0\n\
             .dc V1 3 1 -1\n\
             .measure dc maximum MAX V(out) FROM=3 TO=2\n\
             .measure dc maximum_live EQN maximum\n\
             .measure dc average AVG V(out) FROM=3 TO=2\n\
             .measure dc average_live EQN average\n\
             .end\n",
        )
        .expect("reversed DC live-window deck parses");
        let sweep = [(3.0, 9.0), (2.0, 4.0), (1.0, 1.0)]
            .into_iter()
            .map(|(axis, voltage)| {
                let mut point = SimulationResult::new(1, 0);
                point.node_voltages = vec![0.0, voltage];
                point.node_names = vec!["0".to_string(), "out".to_string()];
                (axis, point)
            })
            .collect::<Vec<_>>();

        let traces = evaluate_dc_equation_measurements(&netlist, &sweep)
            .expect("reversed DC live traces evaluate");
        assert_eq!(traces[0].values, vec![9.0, 9.0, 9.0]);
        assert_eq!(traces[1].values, vec![0.0, 6.5, 6.5]);
        let results = evaluate_dc_measurements(&netlist, &sweep);
        assert_eq!(results[0].value, Some(9.0));
        assert_eq!(results[2].value, Some(6.5));
    }

    #[test]
    fn dc_extrema_filter_every_repeated_descending_window_segment() {
        let netlist = Netlist::parse(
            "nested DC extrema\n\
             V1 out 0 0\n\
             V2 bias 0 0\n\
             .dc V1 3 1 -1 V2 0 1 1\n\
             .meas dc maximum MAX V(out) FROM=2 TO=3\n\
             .meas dc minimum MIN V(out) FROM=2 TO=3\n\
             .meas dc span PP V(out) FROM=2 TO=3\n\
             .end\n",
        )
        .expect("nested DC extrema parse");
        let sweep = [
            (3.0, 9.0),
            (2.0, 4.0),
            (1.0, 1.0),
            (3.0, 19.0),
            (2.0, 14.0),
            (1.0, 11.0),
        ]
        .into_iter()
        .map(|(axis, voltage)| {
            let mut point = SimulationResult::new(1, 0);
            point.node_voltages = vec![0.0, voltage];
            point.node_names = vec!["0".to_string(), "out".to_string()];
            (axis, point)
        })
        .collect::<Vec<_>>();

        let results = evaluate_dc_measurements(&netlist, &sweep);
        assert_eq!(
            results
                .iter()
                .map(|result| result.value)
                .collect::<Vec<_>>(),
            vec![Some(19.0), Some(4.0), Some(15.0)]
        );
        assert!(results.iter().all(|result| result.passed));
    }

    fn dc_equation_sweep() -> Vec<(Value, SimulationResult)> {
        (0..=4)
            .map(|axis| {
                let mut point = SimulationResult::new(1, 0);
                point.node_voltages = vec![0.0, axis as Value];
                point.node_names = vec!["0".to_string(), "out".to_string()];
                (axis as Value, point)
            })
            .collect()
    }

    #[test]
    fn dc_equations_use_zero_default_ordered_dependencies_and_held_values() {
        let netlist = Netlist::parse(
            "* continuous DC equations\n\
             V1 out 0 0\n\
             .dc V1 0 4 1\n\
             .meas dc bounded EQN {V(out)+1} FROM=3 TO=1\n\
             .meas dc derived EQN {bounded*2}\n\
             .end\n",
        )
        .expect("DC equations parse");

        let traces = evaluate_dc_equation_measurements(&netlist, &dc_equation_sweep())
            .expect("DC equations evaluate");

        assert_eq!(traces.len(), 2);
        assert_eq!(traces[0].name, "BOUNDED");
        assert_eq!(traces[0].values, vec![0.0, 2.0, 3.0, 4.0, 4.0]);
        assert!(traces[0].initialized);
        assert_eq!(traces[1].name, "DERIVED");
        assert_eq!(traces[1].values, vec![0.0, 4.0, 6.0, 8.0, 8.0]);

        let results = evaluate_dc_measurements(&netlist, &dc_equation_sweep());
        assert_eq!(results[0].value, Some(4.0));
        assert_eq!(results[1].value, Some(8.0));
        assert!(results.iter().all(|result| result.passed));
    }

    #[test]
    fn dc_point_events_observe_equations_in_netlist_update_order() {
        let netlist = Netlist::parse(
            "DC equation visibility order\n\
             V1 out 0 0\n\
             .dc V1 0 4 1\n\
             .meas dc before WHEN tracked=2.5\n\
             .meas dc tracked EQN {V(out)}\n\
             .meas dc after WHEN tracked=2.5\n\
             .end\n",
        )
        .expect("ordered DC equation consumers parse");

        let results = evaluate_dc_measurements(&netlist, &dc_equation_sweep());

        assert_eq!(results[0].value, Some(3.5));
        assert_eq!(results[1].value, Some(4.0));
        assert_eq!(results[2].value, Some(2.5));
        assert!(results.iter().all(|result| result.passed));
    }

    #[test]
    fn nested_dc_boundaries_require_an_exact_unambiguous_sweep_shape() {
        let matching = Netlist::parse(
            "nested DC shape\n\
             V1 one 0 0\n\
             V2 two 0 0\n\
             .dc V1 0 2 1 V2 0 1 1\n\
             .end\n",
        )
        .expect("nested DC parses");
        assert_eq!(dc_primary_segment_starts(&matching, 6), vec![3]);
        assert!(dc_primary_segment_starts(&matching, 5).is_empty());

        let ambiguous = Netlist::parse(
            "ambiguous nested DC shapes\n\
             V1 one 0 0\n\
             V2 two 0 0\n\
             .dc V1 0 2 1 V2 0 1 1\n\
             .dc V1 0 1 1 V2 0 2 1\n\
             .end\n",
        )
        .expect("multiple nested DC analyses parse");
        assert!(dc_primary_segment_starts(&ambiguous, 6).is_empty());
    }

    #[test]
    fn dc_equations_ignore_time_delay_qualifiers() {
        let netlist = Netlist::parse(
            "* continuous DC equation default\n\
             V1 out 0 0\n\
             .dc V1 0 4 1\n\
             .meas dc delayed EQN {V(out)} TD=2 DEFAULT_VAL=-7\n\
             .end\n",
        )
        .expect("DC equation parses");

        let traces = evaluate_dc_equation_measurements(&netlist, &dc_equation_sweep())
            .expect("DC equation evaluates");

        assert_eq!(traces[0].values, vec![0.0, 1.0, 2.0, 3.0, 4.0]);
        assert!(traces[0].initialized);
    }

    #[test]
    fn dc_equations_orient_one_sided_windows_to_descending_sweeps() {
        let netlist = Netlist::parse(
            "descending DC equations\n\
             V1 out 0 0\n\
             .dc V1 4 0 -1\n\
             .meas dc from_only EQN {V(out)} FROM=2\n\
             .meas dc to_only EQN {V(out)} TO=2\n\
             .meas dc paired EQN {V(out)} FROM=3 TO=1\n\
             .end\n",
        )
        .expect("descending DC equations parse");
        let sweep = dc_equation_sweep().into_iter().rev().collect::<Vec<_>>();

        let traces = evaluate_dc_equation_measurements(&netlist, &sweep)
            .expect("descending DC equations evaluate");

        assert_eq!(traces[0].values, vec![0.0, 0.0, 2.0, 1.0, 0.0]);
        assert_eq!(traces[1].values, vec![4.0, 3.0, 2.0, 2.0, 2.0]);
        assert_eq!(traces[2].values, vec![0.0, 3.0, 2.0, 1.0, 1.0]);
        assert!(traces.iter().all(|trace| trace.initialized));
    }

    #[test]
    fn dc_equation_singleton_uses_declared_sweep_direction() {
        let netlist = Netlist::parse(
            "singleton descending DC equation\n\
             V1 out 0 0\n\
             .dc V1 1 1 -1\n\
             .meas dc bounded EQN {V(out)} FROM=2\n\
             .end\n",
        )
        .expect("singleton descending DC equation parses");
        let mut point = SimulationResult::new(1, 0);
        point.node_voltages = vec![0.0, 1.0];
        point.node_names = vec!["0".to_string(), "out".to_string()];

        let traces = evaluate_dc_equation_measurements(&netlist, &[(1.0, point)])
            .expect("singleton descending DC equation evaluates");

        assert_eq!(traces[0].values, vec![1.0]);
        assert!(traces[0].initialized);
    }

    #[test]
    fn dc_measure_goal_failure_preserves_contract() {
        let netlist = Netlist::parse(
            "measure goal\n\
             V1 in 0 10\n\
             R1 in out 1k\n\
             R2 out 0 1k\n\
             .dc V1 0 10 1\n\
             .meas dc vout MAX V(out) GOAL=4 TOL=0.1\n\
             .end\n",
        )
        .expect("aggregate .MEAS with GOAL/TOL parses");

        let mut low = SimulationResult::new(2, 0);
        low.node_voltages = vec![0.0, 0.0, 0.0];
        low.node_names = vec!["0".to_string(), "in".to_string(), "out".to_string()];

        let mut high = low.clone();
        high.node_voltages[2] = 5.0;

        let results = evaluate_dc_measurements(&netlist, &[(0.0, low), (10.0, high)]);

        assert_eq!(results.len(), 1);
        let result = &results[0];
        assert_eq!(result.name, "VOUT");
        assert_eq!(result.value, Some(5.0));
        assert_eq!(result.expected, Some(4.0));
        assert_eq!(result.tolerance, Some(0.1));
        assert!(!result.passed);
        assert!(
            result
                .error
                .as_deref()
                .is_some_and(|message| message.contains("GOAL"))
        );
    }

    #[test]
    fn dc_measure_goal_failure_preserves_contract_with_engine_sweep() {
        let netlist = Netlist::parse(
            "* dc measurement with failing goal\n\
             V1 in 0 10\n\
             R1 in out 1k\n\
             R2 out 0 1k\n\
             .dc V1 0 10 1\n\
             .meas dc vout MAX V(out) GOAL=4 TOL=0.1\n\
             .end\n",
        )
        .expect("aggregate .MEAS with GOAL/TOL parses");
        let engine = crate::Engine::default();
        let sweep = engine
            .run_dc_sweep(&netlist, "V1", 0.0, 10.0, 1.0)
            .expect("DC sweep runs");

        let results = evaluate_dc_measurements(&netlist, &sweep);

        assert_eq!(results.len(), 1);
        let result = &results[0];
        assert_eq!(result.value, Some(4.9999999999975));
        assert_eq!(result.expected, Some(4.0));
        assert_eq!(result.tolerance, Some(0.1));
        assert!(!result.passed);
    }

    #[test]
    fn dc_error_functions_filter_the_measured_operand_and_ignore_weight() {
        let netlist = Netlist::parse(
            "DC relative error functions\n\
             V3 3 0 2.5\n\
             R3 3 0 1\n\
             V1 1 0 1\n\
             R1 1 2 1\n\
             R2 2 0 3\n\
             .dc V1 -5 5 1\n\
             .measure dc baseline ERR1 V(1) V(2) WEIGHT=9\n\
             .measure dc filtered ERR1 V(1) V(3) YMIN=2.5\n\
             .measure dc filtered_abs ERR2 V(1) V(3) IGNORE=2.5\n\
             .end\n",
        )
        .expect("DC ERR deck parses");
        let engine = crate::Engine::default();
        let sweep = engine
            .run_dc_sweep(&netlist, "V1", -5.0, 5.0, 1.0)
            .expect("DC ERR sweep runs");

        let results = evaluate_dc_measurements(&netlist, &sweep);

        assert!((results[0].value.expect("baseline ERR1") - 0.25).abs() < 1.0e-12);
        assert!(
            (results[1].value.expect("filtered ERR1") - 1.202091156338881).abs() < 1.0e-12,
            "unexpected filtered ERR1 result: {:?}",
            results[1]
        );
        assert!((results[2].value.expect("filtered ERR2") - 1.0).abs() < 1.0e-12);
    }
}
