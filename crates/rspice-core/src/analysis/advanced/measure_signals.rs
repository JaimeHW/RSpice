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

use super::measure::{MeasureEngine, MeasureResult, MeasureStatement, MeasureType};
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
    evaluate_equation_measurements(netlist, "TRAN", &result.time, &signals, -1.0, false)
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
    evaluate_equation_measurements(netlist, "DC", series.axis(), &signals, 0.0, true)
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
    evaluate_equation_measurements(netlist, "AC", series.axis(), &signals, -1.0, false)
}

/// Shared continuous-equation evaluator for analyses with a real-valued
/// axis and real signal waveforms.
fn evaluate_equation_measurements(
    netlist: &Netlist,
    analysis: &str,
    axis: &[Value],
    signals: &HashMap<String, &[Value]>,
    implicit_default: Value,
    normalize_window: bool,
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
                default_value,
            } = &statement.measure_type
            else {
                return None;
            };
            Some((statement, expression, *from, *to, *td, *default_value))
        })
        .map(|(statement, expression, from, to, td, default_value)| {
            let expression = crate::netlist::expr::parse_expression(expression).map_err(|err| {
                format!(
                    "failed to parse continuous measure '{}': {err}",
                    statement.name
                )
            })?;
            let (from, to) = if normalize_window {
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
                current: default_value.unwrap_or(implicit_default),
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
            if equation_axis_is_in_window(axis_value, program.from, program.to, program.td) {
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
) -> bool {
    const XYCE_MEASURE_WINDOW_TOLERANCE: Value = 1.0e-12;
    td.is_none_or(|bound| axis_value >= bound * (1.0 - XYCE_MEASURE_WINDOW_TOLERANCE))
        && from.is_none_or(|bound| axis_value >= bound * (1.0 - XYCE_MEASURE_WINDOW_TOLERANCE))
        && to.is_none_or(|bound| axis_value <= bound * (1.0 + XYCE_MEASURE_WINDOW_TOLERANCE))
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
                .unwrap_or_else(|| fallback.clone());
            storage.push((raw.clone(), 'V', series.clone()));
            if raw != fallback {
                storage.push((fallback, 'V', series));
            }
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

        Some(Self { axis, storage })
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
        signals
    }
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
    /// (full signal key, series)
    storage: Vec<(String, Vec<Value>)>,
}

impl AcSweepSeries {
    /// Collect the derived real series across the sweep. Returns `None`
    /// for an empty sweep.
    pub fn from_sweep(sweep: &[AcResult]) -> Option<Self> {
        let first = sweep.first()?;
        let axis: Vec<Value> = sweep.iter().map(|point| point.frequency).collect();

        let mut storage: Vec<(String, Vec<Value>)> = Vec::new();
        let mut push_complex_series = |prefix: char, raw: &str, values: Vec<crate::Complex64>| {
            let magnitude: Vec<Value> = values.iter().map(|c| c.norm()).collect();
            let db: Vec<Value> = magnitude
                .iter()
                .map(|m| if *m > 1e-30 { 20.0 * m.log10() } else { -600.0 })
                .collect();
            let phase_deg: Vec<Value> = values.iter().map(|c| c.arg().to_degrees()).collect();
            let real: Vec<Value> = values.iter().map(|c| c.re).collect();
            let imag: Vec<Value> = values.iter().map(|c| c.im).collect();

            storage.push((format!("{prefix}({raw})"), magnitude.clone()));
            storage.push((format!("{prefix}M({raw})"), magnitude));
            storage.push((format!("{prefix}DB({raw})"), db));
            storage.push((format!("{prefix}P({raw})"), phase_deg));
            storage.push((format!("{prefix}R({raw})"), real));
            storage.push((format!("{prefix}I({raw})"), imag));
        };

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
            push_complex_series('V', &raw, values);
        }
        for (index, name) in first.branch_names.iter().enumerate() {
            if name.is_empty() {
                continue;
            }
            let values: Vec<crate::Complex64> = sweep
                .iter()
                .map(|point| point.currents.get(index).copied().unwrap_or_default())
                .collect();
            push_complex_series('I', name, values);
        }

        Some(Self { axis, storage })
    }

    /// The sweep frequencies, used as the measurement abscissa.
    pub fn axis(&self) -> &[Value] {
        &self.axis
    }

    /// Borrowed signal table over the collected series.
    pub fn signal_map(&self) -> HashMap<String, &[Value]> {
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        insert_case_variants(&mut signals, "Time", self.axis.as_slice());
        insert_case_variants(&mut signals, "Frequency", self.axis.as_slice());
        insert_case_variants(&mut signals, "Freq", self.axis.as_slice());
        for (key, series) in &self.storage {
            insert_case_variants(&mut signals, key, series.as_slice());
        }
        signals
    }

    /// Signal table used by continuous AC equation measures.
    ///
    /// Ordinary scalar AC measures historically address `V()`/`I()` as
    /// magnitude. Xyce equation expressions instead define the unqualified
    /// accessors as the real projection, so this deliberately supplies a
    /// distinct view without changing the established scalar-measure API.
    pub fn equation_signal_map(&self) -> HashMap<String, &[Value]> {
        let mut signals = self.signal_map();
        insert_case_variants(&mut signals, "Hertz", self.axis.as_slice());
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
                insert_case_variants(&mut signals, &unqualified, series.as_slice());
            }
        }
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
}

impl NoiseSweepSeries {
    /// Collect spectral-density series across the sweep. Returns `None`
    /// for an empty sweep.
    pub fn from_sweep(sweep: &[crate::analysis::NoiseResult]) -> Option<Self> {
        if sweep.is_empty() {
            return None;
        }
        Some(Self {
            axis: sweep.iter().map(|point| point.frequency).collect(),
            onoise: sweep.iter().map(|point| point.output_noise_rms()).collect(),
            inoise: sweep
                .iter()
                .map(|point| point.input_referred_rms())
                .collect(),
        })
    }

    /// The sweep frequencies, used as the measurement abscissa.
    pub fn axis(&self) -> &[Value] {
        &self.axis
    }

    /// Borrowed signal table over the collected series.
    pub fn signal_map(&self) -> HashMap<String, &[Value]> {
        let mut signals: HashMap<String, &[Value]> = HashMap::new();
        insert_case_variants(&mut signals, "Time", self.axis.as_slice());
        insert_case_variants(&mut signals, "Frequency", self.axis.as_slice());
        insert_case_variants(&mut signals, "Freq", self.axis.as_slice());
        for key in ["Onoise", "Onoise_Spectrum"] {
            insert_case_variants(&mut signals, key, self.onoise.as_slice());
        }
        for key in ["Inoise", "Inoise_Spectrum"] {
            insert_case_variants(&mut signals, key, self.inoise.as_slice());
        }
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
    let signals = series.signal_map();
    evaluate_statements(&statements, series.axis(), &signals)
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
) -> Vec<MeasureResult> {
    let mut engine = MeasureEngine::new();
    for statement in statements {
        engine.add((*statement).clone());
    }
    engine.evaluate(axis, signals)
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
    evaluate_statements(&statements, &result.time, &signals)
}

/// Evaluate the netlist's DC .MEAS statements against a sweep.
///
/// Returns an empty vector when the netlist has no DC measurements; an empty
/// sweep fails every statement explicitly rather than skipping it.
pub fn evaluate_dc_measurements(
    netlist: &Netlist,
    sweep: &[(Value, SimulationResult)],
) -> Vec<MeasureResult> {
    let statements = measurements_for_analysis(netlist, "DC");
    if statements.is_empty() {
        return Vec::new();
    }
    let Some(series) = DcSweepSeries::from_sweep(sweep) else {
        return statements
            .iter()
            .map(|m| MeasureResult::failed(&m.name, "DC sweep produced no points"))
            .collect();
    };
    let signals = series.signal_map();
    evaluate_statements(&statements, series.axis(), &signals)
}

/// Evaluate the netlist's AC .MEAS statements against a sweep.
///
/// Returns an empty vector when the netlist has no AC measurements; an
/// empty sweep fails every statement explicitly rather than skipping it.
/// Signal naming follows [`AcSweepSeries`]: magnitudes under `V(x)`/`VM(x)`,
/// `VDB`/`VP` (degrees)/`VR`/`VI` variants, and the frequency axis as
/// `TIME`/`FREQUENCY`/`FREQ`.
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
    let signals = series.signal_map();
    let mut results = evaluate_statements(&statements, series.axis(), &signals);
    match evaluate_ac_equation_measurements(netlist, sweep) {
        Ok(traces) => {
            for (statement, result) in statements.iter().zip(&mut results) {
                if !matches!(statement.measure_type, MeasureType::Equation { .. }) {
                    continue;
                }
                let Some(trace) = traces
                    .iter()
                    .find(|trace| trace.name.eq_ignore_ascii_case(&statement.name))
                else {
                    *result = MeasureResult::failed(
                        &statement.name,
                        "continuous AC equation trace was not produced",
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
                                "continuous AC equation trace is empty",
                            )
                        })
                } else {
                    MeasureResult::failed(
                        &statement.name,
                        "continuous AC equation window was never active",
                    )
                }
                .check_goal(statement);
            }
        }
        Err(err) => {
            for (statement, result) in statements.iter().zip(&mut results) {
                if matches!(statement.measure_type, MeasureType::Equation { .. }) {
                    *result = MeasureResult::failed(
                        &statement.name,
                        &format!("continuous AC equation evaluation failed: {err}"),
                    );
                }
            }
        }
    }
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
    }

    #[test]
    fn dc_equations_honor_explicit_default_and_inclusive_td() {
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

        assert_eq!(traces[0].values, vec![-7.0, -7.0, 2.0, 3.0, 4.0]);
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
}
