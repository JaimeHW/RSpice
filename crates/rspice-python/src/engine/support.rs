//! Argument validation and sweep-axis construction.
//!
//! Every analysis entry point validates its arguments before releasing the
//! GIL, so a caller's mistake surfaces as a `ValueError` naming the offending
//! value rather than as a solver failure several seconds later.

use super::*;

/// Validate that every frequency is finite and non-negative.
pub(super) fn validate_frequencies(frequencies: &[f64]) -> PyResult<()> {
    if frequencies.is_empty() {
        return Err(crate::errors::value_error("frequencies must not be empty"));
    }
    for &f in frequencies {
        if !f.is_finite() || f < 0.0 {
            return Err(crate::errors::value_error(format!(
                "frequencies must be finite and non-negative, got {f}"
            )));
        }
    }
    Ok(())
}

/// Build the `.MC` card a direct Monte Carlo request describes.
pub(super) fn monte_carlo_card(
    num_runs: usize,
    seed: Option<u64>,
    distribution: &str,
    spread: f64,
    params: Option<Vec<String>>,
) -> PyResult<AnalysisCommand> {
    if num_runs == 0 {
        return Err(crate::errors::value_error("num_runs must be at least 1"));
    }
    if !spread.is_finite() || spread < 0.0 {
        return Err(crate::errors::value_error(format!(
            "spread must be finite and non-negative, got {spread}"
        )));
    }
    let distribution = match distribution.to_ascii_lowercase().as_str() {
        "gaussian" | "normal" => MonteCarloDistribution::Gaussian,
        "uniform" => MonteCarloDistribution::Uniform,
        "worst_case" | "worstcase" | "worst-case" => MonteCarloDistribution::WorstCase,
        other => {
            return Err(crate::errors::value_error(format!(
                "distribution must be 'gaussian', 'uniform', or 'worst_case', got '{other}'"
            )));
        }
    };
    Ok(AnalysisCommand::MonteCarlo(
        rspice_core::netlist::MonteCarloCommand {
            runs: num_runs,
            seed,
            distribution,
            relative_spread: spread,
            params: params.unwrap_or_default(),
        },
    ))
}

/// Build the `.PZ` card a direct pole-zero request describes.
///
/// An omitted differential terminal is ground, which is exactly what a `.PZ`
/// card spells as node "0": the card carries all four terminals, so the
/// omission is resolved here rather than becoming a second meaning of "no
/// reference" further down.
pub(super) fn pole_zero_card(
    input_node: &NodeIdentifier,
    input_negative: Option<&NodeIdentifier>,
    output_node: &NodeIdentifier,
    output_negative: Option<&NodeIdentifier>,
    input_type: &str,
    analysis: &str,
) -> PyResult<AnalysisCommand> {
    let transfer_type = match input_type.to_ascii_lowercase().as_str() {
        "current" | "cur" | "i" => PoleZeroTransferType::Current,
        "voltage" | "vol" | "v" => PoleZeroTransferType::Voltage,
        other => {
            return Err(crate::errors::value_error(format!(
                "input_type must be 'current' or 'voltage', got '{other}'"
            )));
        }
    };
    let analysis_type = match analysis.to_ascii_lowercase().as_str() {
        "pz" | "pole_zero" | "poles_zeros" => PoleZeroAnalysisType::PoleZero,
        "pol" | "poles" => PoleZeroAnalysisType::PolesOnly,
        "zer" | "zeros" => PoleZeroAnalysisType::ZerosOnly,
        other => {
            return Err(crate::errors::value_error(format!(
                "analysis must be 'pz', 'poles', or 'zeros', got '{other}'"
            )));
        }
    };
    Ok(AnalysisCommand::PoleZero {
        input_pos: node_identifier_name(input_node),
        input_neg: input_negative.map_or_else(|| "0".to_string(), node_identifier_name),
        output_pos: node_identifier_name(output_node),
        output_neg: output_negative.map_or_else(|| "0".to_string(), node_identifier_name),
        transfer_type,
        analysis_type,
    })
}

/// Validate a `(stop_time, max_step)` pair and resolve the default ceiling.
///
/// The default is `stop_time / 50`, the same fraction of the analysis window
/// SPICE's `.TRAN` uses when a deck authors no `TMAX`.
pub(super) fn solver_window(stop_time: f64, max_step: Option<f64>) -> PyResult<f64> {
    if !stop_time.is_finite() || stop_time <= 0.0 {
        return Err(crate::errors::value_error(format!(
            "stop_time must be a positive finite number of seconds, got {stop_time}"
        )));
    }
    if let Some(step) = max_step
        && (!step.is_finite() || step <= 0.0)
    {
        return Err(crate::errors::value_error(format!(
            "max_step must be a positive finite number of seconds, got {step}"
        )));
    }
    Ok(max_step.unwrap_or(stop_time / 50.0))
}

/// Spell a node identifier the way an authored card names it.
///
/// A numeric index is a legal node name in SPICE, and `resolve_node` reads it
/// back as one, so the round trip is exact rather than an approximation.
pub(super) fn node_identifier_name(node: &NodeIdentifier) -> String {
    match node {
        NodeIdentifier::Index(index) => index.to_string(),
        NodeIdentifier::Name(name) => name.clone(),
    }
}

pub(super) fn parse_variation(variation: &str) -> PyResult<FreqVariation> {
    match variation.to_ascii_lowercase().as_str() {
        "dec" | "decade" => Ok(FreqVariation::Dec),
        "oct" | "octave" => Ok(FreqVariation::Oct),
        "lin" | "linear" => Ok(FreqVariation::Lin),
        other => Err(crate::errors::value_error(format!(
            "variation must be 'dec', 'oct', or 'lin', got '{other}'"
        ))),
    }
}

/// Generate frequency points for an analysis directive's sweep spec.
pub(super) fn sweep_frequencies(
    variation: FreqVariation,
    points: usize,
    start: f64,
    stop: f64,
    max_points: usize,
) -> PyResult<Vec<f64>> {
    rspice_core::analysis::ac::try_ac_sweep_frequencies_bounded_with_abort(
        variation,
        points,
        start,
        stop,
        max_points,
        &rspice_core::abort_signal::NoAbort,
    )
    .map_err(|error| match error {
        rspice_core::analysis::FrequencyGridError::LimitExceeded { requested, limit } => {
            crate::errors::simulation_error_to_pyerr(
                rspice_core::engine::SimulationError::ResourceLimit(
                    rspice_core::resource::ResourceLimitError {
                        resource: rspice_core::resource::ResourceKind::AnalysisPoints,
                        requested,
                        limit,
                    },
                ),
            )
        }
        _ => crate::errors::value_error(format!("invalid frequency sweep: {error}")),
    })
}

pub(super) fn ac_data_frequencies(
    netlist: &rspice_core::Netlist,
    table_name: &str,
) -> PyResult<Vec<f64>> {
    let table = netlist
        .data_tables
        .iter()
        .find(|table| table.name.eq_ignore_ascii_case(table_name))
        .ok_or_else(|| {
            crate::errors::value_error(format!("AC DATA table '{table_name}' not found"))
        })?;
    let frequency_column = table
        .params
        .iter()
        .position(|param| param.eq_ignore_ascii_case("FREQ"))
        .ok_or_else(|| {
            crate::errors::value_error(format!(
                "AC DATA table '{}' must contain a FREQ column",
                table.name
            ))
        })?;
    if table.rows.is_empty() {
        return Err(crate::errors::value_error(format!(
            "AC DATA table '{}' has no rows",
            table.name
        )));
    }
    let mut frequencies = Vec::with_capacity(table.rows.len());
    for (row_index, row) in table.rows.iter().enumerate() {
        if row.len() != table.params.len() {
            return Err(crate::errors::value_error(format!(
                "AC DATA table '{}' row {} has {} values, expected {}",
                table.name,
                row_index + 1,
                row.len(),
                table.params.len()
            )));
        }
        let Some(&frequency) = row.get(frequency_column) else {
            return Err(crate::errors::value_error(format!(
                "AC DATA table '{}' row {} has no frequency column",
                table.name,
                row_index + 1
            )));
        };
        if !frequency.is_finite() || frequency < 0.0 {
            return Err(crate::errors::value_error(format!(
                "AC DATA table '{}' row {} has invalid frequency {frequency}",
                table.name,
                row_index + 1
            )));
        }
        frequencies.push(frequency);
    }
    Ok(frequencies)
}

/// Validate the bounds a linear `.DC` sweep needs.
pub(super) fn require_linear_bounds(
    start: Option<f64>,
    stop: Option<f64>,
    step: Option<f64>,
) -> PyResult<(f64, f64, f64)> {
    let (start, stop, step) = match (start, stop, step) {
        (Some(start), Some(stop), Some(step)) => (start, stop, step),
        _ => {
            return Err(crate::errors::value_error(
                "mode='linear' requires start, stop, and step",
            ));
        }
    };
    if !start.is_finite() || !stop.is_finite() || !step.is_finite() {
        return Err(crate::errors::value_error(format!(
            "sweep bounds must be finite, got start={start}, stop={stop}, step={step}"
        )));
    }
    if step == 0.0 {
        return Err(crate::errors::value_error("sweep step must be non-zero"));
    }
    if (stop > start && step < 0.0) || (stop < start && step > 0.0) {
        return Err(crate::errors::value_error(format!(
            "sweep step sign must move from start toward stop, got start={start}, stop={stop}, step={step}"
        )));
    }
    Ok((start, stop, step))
}

/// Validate the bounds a logarithmic `.DC` sweep needs.
pub(super) fn require_log_bounds(start: Option<f64>, stop: Option<f64>) -> PyResult<(f64, f64)> {
    let (start, stop) = match (start, stop) {
        (Some(start), Some(stop)) => (start, stop),
        _ => {
            return Err(crate::errors::value_error(
                "logarithmic sweeps require start and stop",
            ));
        }
    };
    if !start.is_finite() || !stop.is_finite() {
        return Err(crate::errors::value_error(format!(
            "sweep bounds must be finite, got start={start}, stop={stop}"
        )));
    }
    if start <= 0.0 || stop <= 0.0 {
        return Err(crate::errors::value_error(format!(
            "logarithmic sweep bounds must be positive, got start={start}, stop={stop}"
        )));
    }
    Ok((start, stop))
}

/// Stable `AnalysisRecord.kind` tag for a directive.
///
/// Executed records are tagged where they are pushed; this mirrors those tags
/// for a directive that failed before it could push one.
pub(super) fn analysis_record_kind(analysis: &AnalysisCommand) -> &'static str {
    match analysis {
        AnalysisCommand::Op => "op",
        AnalysisCommand::Dc { .. } => "dc",
        AnalysisCommand::Tran { .. } => "tran",
        AnalysisCommand::Ac { .. } => "ac",
        AnalysisCommand::AcData { .. } => "ac_data",
        AnalysisCommand::Hb { .. } => "hb",
        AnalysisCommand::Disto { .. } => "disto",
        AnalysisCommand::Sp { .. } => "sp",
        AnalysisCommand::Noise { .. } => "noise",
        AnalysisCommand::NoiseData { .. } => "noise_data",
        AnalysisCommand::Tf { .. } => "tf",
        AnalysisCommand::Stb { .. } => "stb",
        AnalysisCommand::PoleZero { .. } => "pz",
        AnalysisCommand::MonteCarlo(_) => "mc",
        AnalysisCommand::Step(_) => "step",
        AnalysisCommand::Temp { .. } => "temp",
        AnalysisCommand::Sensitivity { ac_sweep, .. } => {
            if ac_sweep.is_some() {
                "sens_ac"
            } else {
                "sens"
            }
        }
        AnalysisCommand::Four { .. } => "four",
        AnalysisCommand::Pss(_) => "pss",
        AnalysisCommand::Pac(_) => "pac",
        AnalysisCommand::Pnoise(_) => "pnoise",
        AnalysisCommand::Envelope(_) => "envelope",
    }
}
