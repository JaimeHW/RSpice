//! Shared .MEAS evaluation harness over simulation results.
//!
//! Builds the signal-name table the [`MeasureEngine`](super::MeasureEngine)
//! expects and evaluates a netlist's measurement statements against
//! transient or DC-sweep data. Both the CLI and the Python bindings consume
//! this module so measurement semantics cannot drift between frontends.
//!
//! Signal naming: voltages are reachable as `V(name)`, `name`, `V(id)`, and
//! `id` in any case; branch currents as `I(name)`; the analysis axis as
//! `TIME` (the swept value plays that role for DC sweeps, so
//! `FIND TIME WHEN V(out)=...` addresses the sweep variable).

use std::collections::HashMap;

use super::measure::{
    ContinuousMeasureResult, MeasureEngine, MeasureOperand, MeasureResult, MeasureStatement,
    MeasureType, TriggerEvent,
};
use crate::Value;
use crate::analysis::AcResult;
use crate::engine::TransientResult;
use crate::netlist::Netlist;
use crate::netlist::expr::Expr as NetExpr;
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

/// Build the measurement signal table for a transient result.
pub fn transient_signal_map(result: &TransientResult) -> HashMap<String, &[Value]> {
    let mut signals: HashMap<String, &[Value]> = HashMap::new();

    // The time axis itself, so `FIND TIME WHEN V(out)=...` works.
    insert_case_variants(&mut signals, "Time", result.time.as_slice());

    for (index, waveform) in result.voltages.iter().enumerate() {
        let fallback = (index + 1).to_string();
        let raw = result
            .node_names
            .get(index)
            .filter(|name| !name.is_empty())
            .cloned()
            .unwrap_or_else(|| fallback.clone());

        insert_wrapped_variants(&mut signals, 'V', &raw, waveform.as_slice());
        insert_case_variants(&mut signals, &raw, waveform.as_slice());
        if raw != fallback {
            insert_wrapped_variants(&mut signals, 'V', &fallback, waveform.as_slice());
            insert_case_variants(&mut signals, &fallback, waveform.as_slice());
        }
    }

    for (index, waveform) in result.branch_currents.iter().enumerate() {
        if let Some(name) = result.branch_names.get(index).filter(|n| !n.is_empty()) {
            insert_wrapped_variants(&mut signals, 'I', name, waveform.as_slice());
        }
    }

    signals
}

/// One continuous Xyce `PARAM`/`EQN` measurement evaluated at every
/// transient result point.
#[derive(Debug, Clone, PartialEq)]
pub struct EquationMeasureTrace {
    pub name: String,
    pub values: Vec<Value>,
    /// Whether at least one point landed inside the measure's time window.
    pub initialized: bool,
}

struct EquationProgram<'a> {
    statement: &'a MeasureStatement,
    expression: NetExpr,
    from: Option<Value>,
    to: Option<Value>,
    td: Option<Value>,
    current: Value,
    initialized: bool,
    values: Vec<Value>,
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
    let signals = transient_signal_map(result);
    evaluate_equation_measurements(netlist, "TRAN", &result.time, &signals, -1.0, None)
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
    let Some(series) = DcSweepSeries::from_sweep(sweep) else {
        return Ok(Vec::new());
    };
    let signals = series.signal_map();
    evaluate_equation_measurements(
        netlist,
        "DC",
        series.axis(),
        &signals,
        0.0,
        Some(dc_primary_sweep_is_ascending(netlist, series.axis())),
    )
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
    let Some(series) = AcSweepSeries::from_sweep(sweep) else {
        return Ok(Vec::new());
    };
    let signals = series.equation_signal_map();
    evaluate_equation_measurements(netlist, "AC", series.axis(), &signals, -1.0, None)
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
    let Some(series) = NoiseSweepSeries::from_sweep(sweep) else {
        return Ok(Vec::new());
    };
    let signals = series.equation_signal_map();
    evaluate_equation_measurements(netlist, "NOISE", series.axis(), &signals, -1.0, None)
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
    let mut programs = netlist
        .measurements
        .iter()
        .filter(|statement| statement.analysis.eq_ignore_ascii_case(analysis))
        .filter_map(|statement| {
            let MeasureType::Equation {
                expression,
                from,
                to,
                td,
            } = &statement.measure_type
            else {
                return None;
            };
            Some((statement, expression, *from, *to, *td))
        })
        .map(|(statement, expression, from, to, td)| {
            let expression = crate::netlist::expr::parse_expression(expression).map_err(|err| {
                format!(
                    "failed to parse continuous measure '{}': {err}",
                    statement.name
                )
            })?;
            let (from, to) = if dc_sweep_ascending.is_some() {
                match (from, to) {
                    (Some(from), Some(to)) if from > to => (Some(to), Some(from)),
                    bounds => bounds,
                }
            } else {
                (from, to)
            };
            Ok(EquationProgram {
                statement,
                expression,
                from,
                to,
                td,
                current: netlist
                    .options
                    .measure_default_value
                    .or(statement.default_value)
                    .unwrap_or(implicit_default),
                initialized: false,
                values: Vec::with_capacity(axis.len()),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;

    if programs.is_empty() {
        return Ok(Vec::new());
    }
    let mut current_values = programs
        .iter()
        .map(|program| (program.statement.name.to_ascii_uppercase(), program.current))
        .collect::<HashMap<_, _>>();

    for (row, &axis_value) in axis.iter().enumerate() {
        for program in &mut programs {
            if equation_axis_is_in_window(
                axis_value,
                program.from,
                program.to,
                program.td,
                dc_sweep_ascending,
            ) {
                let bound =
                    bind_equation_expression(&program.expression, row, signals, &current_values)?;
                let value = crate::netlist::expr::evaluate_complex(&bound, &netlist.params)
                    .map_err(|err| {
                        format!(
                            "continuous measure '{}' evaluation failed at row {row}: {err}",
                            program.statement.name
                        )
                    })?;
                if !value.is_real() || !value.re.is_finite() {
                    return Err(format!(
                        "continuous measure '{}' produced non-real or non-finite value at row {row}",
                        program.statement.name
                    ));
                }
                program.current = value.re;
                program.initialized = true;
                current_values.insert(program.statement.name.to_ascii_uppercase(), program.current);
            }
            program.values.push(program.current);
        }
    }

    Ok(programs
        .into_iter()
        .map(|program| EquationMeasureTrace {
            name: program.statement.name.clone(),
            values: program.values,
            initialized: program.initialized,
        })
        .collect())
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

fn bind_equation_expression(
    expression: &NetExpr,
    row: usize,
    signals: &HashMap<String, &[Value]>,
    measures: &HashMap<String, Value>,
) -> Result<NetExpr, String> {
    Ok(match expression {
        NetExpr::Param(name) => {
            let is_axis_symbol = matches!(
                name.to_ascii_uppercase().as_str(),
                "TIME" | "FREQ" | "FREQUENCY" | "HERTZ"
            );
            if is_axis_symbol
                && let Some(value) = lookup_equation_signal_optional(signals, name, row)
            {
                NetExpr::Number(value)
            } else if let Some(value) = measures.get(&name.to_ascii_uppercase()).copied() {
                NetExpr::Number(value)
            } else if let Some(value) = lookup_equation_signal_optional(signals, name, row) {
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
            let first = equation_probe_argument(args.first()).ok_or_else(|| {
                format!("{prefix}() in continuous measure has an invalid first argument")
            })?;
            let first_value = lookup_equation_signal(signals, &format!("{prefix}({first})"), row)?;
            if prefix == "V" && args.len() == 2 {
                let second = equation_probe_argument(args.get(1)).ok_or_else(|| {
                    "V() in continuous measure has an invalid second argument".to_string()
                })?;
                let second_value = lookup_equation_signal(signals, &format!("V({second})"), row)?;
                NetExpr::Number(first_value - second_value)
            } else if args.len() == 1 {
                NetExpr::Number(first_value)
            } else {
                return Err(format!(
                    "{prefix}() in continuous measure has invalid arity"
                ));
            }
        }
        NetExpr::FnCall { name, args } => NetExpr::FnCall {
            name: name.clone(),
            args: args
                .iter()
                .map(|arg| bind_equation_expression(arg, row, signals, measures))
                .collect::<Result<Vec<_>, _>>()?,
        },
        NetExpr::Number(_) | NetExpr::ComplexNumber(_) => expression.clone(),
    })
}

fn is_equation_probe_accessor(name: &str) -> bool {
    matches!(
        name.to_ascii_uppercase().as_str(),
        "V" | "VM" | "VR" | "VI" | "VP" | "VDB" | "I" | "IM" | "IR" | "II" | "IP" | "IDB"
    )
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
    signals: &HashMap<String, &[Value]>,
    name: &str,
    row: usize,
) -> Result<Value, String> {
    lookup_equation_signal_optional(signals, name, row)
        .ok_or_else(|| format!("continuous measure signal '{name}' is unavailable at row {row}"))
}

fn lookup_equation_signal_optional(
    signals: &HashMap<String, &[Value]>,
    name: &str,
    row: usize,
) -> Option<Value> {
    signals
        .iter()
        .find_map(|(candidate, values)| candidate.eq_ignore_ascii_case(name).then_some(*values))
        .and_then(|values| values.get(row).copied())
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
    /// Returns `None` for an empty sweep.
    pub fn from_sweep(sweep: &[(Value, SimulationResult)]) -> Option<Self> {
        let (_, first) = sweep.first()?;
        let axis: Vec<Value> = sweep.iter().map(|(v, _)| *v).collect();

        let mut storage: Vec<(String, char, Vec<Value>)> = Vec::new();
        for node in 1..first.node_voltages.len() {
            let series: Vec<Value> = sweep
                .iter()
                .map(|(_, r)| r.node_voltages.get(node).copied().unwrap_or(0.0))
                .collect();
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
            if name.is_empty() {
                continue;
            }
            let series: Vec<Value> = sweep
                .iter()
                .map(|(_, r)| r.branch_currents.get(branch).copied().unwrap_or(0.0))
                .collect();
            storage.push((name.clone(), 'I', series));
        }

        // Parameter sweeps can rebuild the circuit at every point and may
        // change the lowering topology (for example, a resistor can cross the
        // threshold between nodal and explicit-branch forms). Form a
        // case-insensitive union before requiring a complete waveform so a
        // name first introduced after row zero is not silently omitted.
        let mut observable_names = Vec::<String>::new();
        for (_, result) in sweep {
            for (name, _) in &result.dc_observables {
                if !observable_names
                    .iter()
                    .any(|candidate| candidate.eq_ignore_ascii_case(name))
                {
                    observable_names.push(name.clone());
                }
            }
        }
        let observables = observable_names
            .into_iter()
            .filter_map(|name| {
                let values = sweep
                    .iter()
                    .map(|(_, result)| result.try_dc_observable_named(&name))
                    .collect::<Option<Vec<_>>>()?;
                Some((name, values))
            })
            .collect();

        Some(Self {
            axis,
            storage,
            observables,
        })
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
        let magnitude: Vec<Value> = values.iter().map(|c| c.norm()).collect();
        let db: Vec<Value> = magnitude
            .iter()
            .map(|m| if *m > 1e-30 { 20.0 * m.log10() } else { -600.0 })
            .collect();
        let phase_deg: Vec<Value> = values.iter().map(|c| c.arg().to_degrees()).collect();
        let real: Vec<Value> = values.iter().map(|c| c.re).collect();
        let imag: Vec<Value> = values.iter().map(|c| c.im).collect();

        self.storage
            .push((format!("{prefix}({raw})"), magnitude.clone()));
        self.storage.push((format!("{prefix}M({raw})"), magnitude));
        self.storage.push((format!("{prefix}DB({raw})"), db));
        self.storage.push((format!("{prefix}P({raw})"), phase_deg));
        self.storage.push((format!("{prefix}R({raw})"), real));
        self.storage.push((format!("{prefix}I({raw})"), imag));
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
    /// Collect the derived real series across the sweep. Returns `None`
    /// for an empty sweep.
    pub fn from_sweep(sweep: &[AcResult]) -> Option<Self> {
        let first = sweep.first()?;
        let axis: Vec<Value> = sweep.iter().map(|point| point.frequency).collect();

        let mut projections = ComplexProjectionSeries::default();

        for (index, name) in first.node_names.iter().enumerate() {
            let raw = if name.is_empty() {
                (index + 1).to_string()
            } else {
                name.clone()
            };
            let values: Vec<crate::Complex64> = sweep
                .iter()
                .map(|point| point.voltages.get(index).copied().unwrap_or_default())
                .collect();
            projections.push('V', &raw, values);
        }
        for (index, name) in first.branch_names.iter().enumerate() {
            if name.is_empty() {
                continue;
            }
            let values: Vec<crate::Complex64> = sweep
                .iter()
                .map(|point| point.currents.get(index).copied().unwrap_or_default())
                .collect();
            projections.push('I', name, values);
        }

        Some(Self { axis, projections })
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
    pub fn equation_signal_map(&self) -> HashMap<String, &[Value]> {
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
}

impl NoiseSweepSeries {
    /// Collect spectral-density series across the sweep. Returns `None`
    /// for an empty sweep.
    pub fn from_sweep(sweep: &[crate::analysis::NoiseResult]) -> Option<Self> {
        let first = sweep.first()?;
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
                    .map(|point| point.voltages.get(index).copied().unwrap_or_default())
                    .collect(),
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
                    .map(|point| point.currents.get(index).copied().unwrap_or_default())
                    .collect(),
            );
        }
        Some(Self {
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
        })
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
        signals
    }

    /// Signal table used by continuous NOISE equation measures. Bare complex
    /// probes are overlaid with their real-component projection.
    pub fn equation_signal_map(&self) -> HashMap<String, &[Value]> {
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
    let statements = measurements_for_analysis(netlist, "NOISE");
    if statements.is_empty() {
        return Vec::new();
    }
    let Some(series) = NoiseSweepSeries::from_sweep(sweep) else {
        return statements
            .iter()
            .map(|m| MeasureResult::failed(&m.name, "noise sweep produced no points"))
            .collect();
    };
    let signals = series.equation_signal_map();
    // NOISE equations participate in the accepted-point stream just like AC
    // equations. A later WHEN/FIND-WHEN statement must see the equation's
    // current value as a waveform, rather than only its final scalar result.
    let equation_traces = evaluate_noise_equation_measurements(netlist, sweep);
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
        ),
        Err(_) => evaluate_statements(&statements, series.axis(), &signals, &netlist.params),
    };
    overlay_continuous_equation_results(&statements, &mut results, equation_traces, "NOISE");
    results
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
    let Some(series) = NoiseSweepSeries::from_sweep(sweep) else {
        return statements
            .iter()
            .map(|statement| ContinuousMeasureResult {
                name: statement.name.clone(),
                records: Vec::new(),
                failure: Some("noise sweep produced no points".to_string()),
            })
            .collect();
    };
    let signals = series.equation_signal_map();
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

fn evaluate_statements(
    statements: &[&MeasureStatement],
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    params: &crate::netlist::ParamContext,
) -> Vec<MeasureResult> {
    evaluate_statements_with_segment_starts(statements, axis, signals, params, &[])
}

fn evaluate_statements_with_segment_starts(
    statements: &[&MeasureStatement],
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    params: &crate::netlist::ParamContext,
    segment_starts: &[usize],
) -> Vec<MeasureResult> {
    let derived = materialize_measure_expression_signals(statements, axis, signals, params);
    let mut augmented_signals = signals.clone();
    for (name, waveform) in &derived {
        augmented_signals.insert(name.clone(), waveform.as_slice());
    }
    let mut engine = MeasureEngine::new();
    for statement in statements {
        engine.add((*statement).clone());
    }
    engine.evaluate_with_segment_starts(axis, &augmented_signals, segment_starts)
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
) -> Vec<MeasureResult> {
    let equation_positions = traces
        .iter()
        .map(|trace| {
            statements.iter().position(|statement| {
                matches!(statement.measure_type, MeasureType::Equation { .. })
                    && statement.name.eq_ignore_ascii_case(&trace.name)
            })
        })
        .collect::<Vec<_>>();
    let previous_values = traces
        .iter()
        .zip(&equation_positions)
        .map(|(trace, position)| {
            let local_default = position.and_then(|position| statements[position].default_value);
            let default = global_default.or(local_default).unwrap_or(equation_default);
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

    // Xyce updates measurements sequentially at every accepted point. A
    // consumer after an equation sees its current-point value; a forward
    // consumer sees the previous-point value (or DEFAULT_VAL at row zero).
    let mut signal_maps = statements
        .iter()
        .enumerate()
        .map(|(statement_index, _)| {
            let mut map = signals.clone();
            for (trace_index, trace) in traces.iter().enumerate() {
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
    for statement in statements {
        engine.add((*statement).clone());
    }
    engine.evaluate_with_segment_starts_and_signal_maps(axis, &signal_maps, segment_starts)
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
                if !matches!(statement.measure_type, MeasureType::Equation { .. }) {
                    continue;
                }
                let Some(trace) = traces
                    .iter()
                    .find(|trace| trace.name.eq_ignore_ascii_case(&statement.name))
                else {
                    *result = MeasureResult::failed(
                        &statement.name,
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
                } else {
                    MeasureResult::failed(
                        &statement.name,
                        &format!("continuous {analysis} equation window was never active"),
                    )
                }
                .check_goal(statement);
            }
        }
        Err(err) => {
            for (statement, result) in statements.iter().zip(results) {
                if matches!(statement.measure_type, MeasureType::Equation { .. }) {
                    *result = MeasureResult::failed(
                        &statement.name,
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
            MeasureType::Delay { trig, targ } => {
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

    names
        .into_iter()
        .filter_map(|name| {
            let expression = name.strip_prefix('{')?.strip_suffix('}')?;
            let expression = crate::netlist::expr::parse_expression(expression).ok()?;
            let mut waveform = Vec::with_capacity(axis.len());
            let measures = HashMap::new();
            for row in 0..axis.len() {
                let bound = bind_equation_expression(&expression, row, signals, &measures).ok()?;
                let value = crate::netlist::expr::evaluate_complex(&bound, params).ok()?;
                if !value.is_real() || !value.re.is_finite() {
                    return None;
                }
                waveform.push(value.re);
            }
            Some((name, waveform))
        })
        .collect()
}

fn materialize_differential_voltage_signals(
    statements: &[&MeasureStatement],
    point_count: usize,
    signals: &HashMap<String, &[Value]>,
) -> Vec<(String, Vec<Value>)> {
    let mut names = Vec::new();
    let mut add = |name: &str| {
        let trimmed = name.trim();
        if differential_voltage_nodes(trimmed).is_some()
            && !names
                .iter()
                .any(|candidate: &String| candidate.eq_ignore_ascii_case(trimmed))
        {
            names.push(trimmed.to_string());
        }
    };
    for statement in statements {
        match &statement.measure_type {
            MeasureType::Delay { trig, targ } => {
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

    names
        .into_iter()
        .filter_map(|name| {
            let (positive, negative) = differential_voltage_nodes(&name)?;
            let positive = measurement_node_waveform(positive, point_count, signals)?;
            let negative = measurement_node_waveform(negative, point_count, signals)?;
            let waveform = positive
                .iter()
                .zip(negative)
                .map(|(positive, negative)| positive - negative)
                .collect();
            Some((name, waveform))
        })
        .collect()
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

fn measurement_node_waveform(
    node: &str,
    point_count: usize,
    signals: &HashMap<String, &[Value]>,
) -> Option<Vec<Value>> {
    if matches!(node.to_ascii_lowercase().as_str(), "0" | "gnd" | "ground") {
        return Some(vec![0.0; point_count]);
    }
    signals
        .iter()
        .find_map(|(candidate, waveform)| candidate.eq_ignore_ascii_case(node).then_some(*waveform))
        .map(ToOwned::to_owned)
}

/// Evaluate the netlist's transient .MEAS statements against a result.
///
/// Returns an empty vector when the netlist has no transient measurements.
pub fn evaluate_tran_measurements(
    netlist: &Netlist,
    result: &TransientResult,
) -> Vec<MeasureResult> {
    let statements = measurements_for_analysis(netlist, "TRAN");
    if statements.is_empty() {
        return Vec::new();
    }
    let signals = transient_signal_map(result);
    let mut results = evaluate_statements(&statements, &result.time, &signals, &netlist.params);
    overlay_continuous_equation_results(
        &statements,
        &mut results,
        evaluate_tran_equation_measurements(netlist, result),
        "TRAN",
    );
    results
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
    let Some(series) = DcSweepSeries::from_sweep(sweep) else {
        return statements
            .iter()
            .map(|statement| ContinuousMeasureResult {
                name: statement.name.clone(),
                records: Vec::new(),
                failure: Some("DC sweep produced no points".to_string()),
            })
            .collect();
    };
    let mut signals = series.signal_map();
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
            })
            .collect();
    } else {
        dc_parameter_context_series(point_params)
    };
    for (name, waveform) in &parameter_series {
        insert_case_variants(&mut signals, name, waveform);
    }
    let differential_signals =
        materialize_differential_voltage_signals(&statements, series.axis().len(), &signals);
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
    evaluate_dc_measurements_with_parameter_contexts(netlist, sweep, &[])
}

/// Evaluate DC measurements with an optional parameter context for every
/// accepted point. This preserves `.DC DATA` semantics when table columns
/// change parameters (and dependent parameters) from row to row.
pub fn evaluate_dc_measurements_with_parameter_contexts(
    netlist: &Netlist,
    sweep: &[(Value, SimulationResult)],
    point_params: &[crate::netlist::ParamContext],
) -> Vec<MeasureResult> {
    let statements = measurements_for_analysis(netlist, "DC");
    if statements.is_empty() {
        return Vec::new();
    }
    let normalized_statements = statements
        .into_iter()
        .cloned()
        .map(normalize_dc_measurement_window)
        .collect::<Vec<_>>();
    let statements = normalized_statements.iter().collect::<Vec<_>>();
    let Some(series) = DcSweepSeries::from_sweep(sweep) else {
        return statements
            .iter()
            .map(|m| MeasureResult::failed(&m.name, "DC sweep produced no points"))
            .collect();
    };
    let mut signals = series.signal_map();
    let parameter_series = if point_params.is_empty() {
        Vec::new()
    } else if point_params.len() != series.axis().len() {
        return statements
            .iter()
            .map(|statement| {
                MeasureResult::failed(
                    &statement.name,
                    "DC point-parameter context count does not match sweep length",
                )
            })
            .collect();
    } else {
        dc_parameter_context_series(point_params)
    };
    for (name, waveform) in &parameter_series {
        insert_case_variants(&mut signals, name, waveform);
    }
    let differential_signals =
        materialize_differential_voltage_signals(&statements, series.axis().len(), &signals);
    for (name, waveform) in &differential_signals {
        insert_case_variants(&mut signals, name, waveform);
    }
    // Continuous equation measures are live waveforms, not merely final
    // scalars. Their visibility at each statement depends on netlist order.
    let equation_traces = evaluate_equation_measurements(
        netlist,
        "DC",
        series.axis(),
        &signals,
        0.0,
        Some(dc_primary_sweep_is_ascending(netlist, series.axis())),
    );
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
        ),
        Err(_) => evaluate_statements_with_segment_starts(
            &statements,
            series.axis(),
            &signals,
            &netlist.params,
            &segment_starts,
        ),
    };
    overlay_continuous_equation_results(&statements, &mut results, equation_traces, "DC");
    results
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
    let Some(series) = AcSweepSeries::from_sweep(sweep) else {
        return statements
            .iter()
            .map(|statement| ContinuousMeasureResult {
                name: statement.name.clone(),
                records: Vec::new(),
                failure: Some("AC sweep produced no points".to_string()),
            })
            .collect();
    };
    evaluate_continuous_statements(
        &statements,
        series.axis(),
        series.equation_signal_map(),
        &netlist.params,
        &[],
    )
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
    let statements = measurements_for_analysis(netlist, "AC");
    if statements.is_empty() {
        return Vec::new();
    }
    let Some(series) = AcSweepSeries::from_sweep(sweep) else {
        return statements
            .iter()
            .map(|m| MeasureResult::failed(&m.name, "AC sweep produced no points"))
            .collect();
    };
    let signals = series.equation_signal_map();
    // Continuous equation measures participate in the accepted-point stream.
    // A later WHEN/FIND-WHEN statement therefore observes the equation's
    // current value as a waveform, not only its final scalar result.
    let equation_traces = evaluate_ac_equation_measurements(netlist, sweep);
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
        ),
        Err(_) => evaluate_statements(&statements, series.axis(), &signals, &netlist.params),
    };
    overlay_continuous_equation_results(&statements, &mut results, equation_traces, "AC");
    results
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
        .map(|m| MeasureResult::failed(&m.name, reason))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tran_result() -> TransientResult {
        TransientResult {
            time: vec![0.0, 1.0, 2.0, 3.0],
            voltages: vec![vec![0.0, 1.0, 2.0, 3.0]],
            branch_currents: vec![vec![0.0, -1.0, -2.0, -3.0]],
            num_nodes: 1,
            node_names: vec!["out".to_string()],
            branch_names: vec!["v1".to_string()],
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
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

        let series = AcSweepSeries::from_sweep(&sweep).expect("non-empty sweep");
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
            contributions: Vec::new(),
            node_names: vec!["out".to_string()],
            branch_names: vec!["V1".to_string()],
            voltages: vec![voltage],
            currents: vec![current],
        }
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

        let series = NoiseSweepSeries::from_sweep(&sweep).expect("non-empty noise sweep");
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

        let series = DcSweepSeries::from_sweep(&sweep).expect("non-empty sweep");
        assert_eq!(series.axis(), &[0.0, 5.0]);
        let signals = series.signal_map();
        assert!(signals.contains_key("TIME"));
        assert_eq!(signals["V(out)"], &[2.5, 2.5][..]);
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
