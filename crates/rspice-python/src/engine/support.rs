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

/// Validate the nominal value and perturbation a finite-difference
/// sensitivity will differentiate around.
///
/// A non-finite nominal value has no neighbourhood to difference over, and a
/// non-positive step would divide by zero or walk backwards, so both are
/// rejected before any solve is scheduled.
pub(super) fn validate_sensitivity_perturbation(
    param_value: f64,
    delta: Option<f64>,
) -> PyResult<()> {
    if !param_value.is_finite() {
        return Err(crate::errors::value_error(format!(
            "param_value must be finite, got {param_value}"
        )));
    }
    if let Some(d) = delta
        && (!d.is_finite() || d <= 0.0)
    {
        return Err(crate::errors::value_error(format!(
            "delta must be a positive finite number, got {d}"
        )));
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
        AnalysisCommand::Pxf(_) => "pxf",
        AnalysisCommand::Pstb(_) => "pstb",
        AnalysisCommand::Pnoise(_) => "pnoise",
        AnalysisCommand::Envelope(_) => "envelope",
        AnalysisCommand::DcMatch(_) => "dcmatch",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::netlist::describe_analysis;
    use crate::results::PyDcMatchResult;

    /// The Spectre statistics library the mismatch deck below carries,
    /// declared the way a PDK declares one.
    const DIVIDER_STATISTICS: &str = "\
// Resistor divider mismatch.
parameters r1v=1000 r2v=2000
statistics {
 mismatch {
  vary r1v dist=gauss std=10
  vary r2v dist=gauss std=10
 }
}
";

    /// `V1 -- R1 -- out -- R2 -- 0`, both resistances varied independently.
    ///
    /// The statistics block travels the path a user's deck takes: it is
    /// lowered by the same public Spectre adapter the include expander runs a
    /// `.scs` library through, then spliced into the deck. Nothing in this
    /// crate can reach a file the browser and the notebook may not have.
    fn divider_with_mismatch(card: &str) -> rspice_core::Netlist {
        let lowered = rspice_core::library::adapt_spectre_model_library(
            std::path::Path::new("statistics.scs"),
            DIVIDER_STATISTICS,
        )
        .expect("the statistics library lowers to executable SPICE");
        rspice_core::Netlist::parse(&format!(
            "DC mismatch directive\n{lowered}V1 in 0 1\nR1 in out {{r1v}}\nR2 out 0 {{r2v}}\n\
             {card}\n.end\n"
        ))
        .expect("the lowered statistical deck parses")
    }

    /// The one authored `.DCMATCH` card of a deck.
    fn dcmatch_card(netlist: &rspice_core::Netlist) -> rspice_core::netlist::DcMatchCard {
        netlist
            .analyses
            .iter()
            .find_map(|analysis| match analysis {
                AnalysisCommand::DcMatch(card) => Some((**card).clone()),
                _ => None,
            })
            .expect("the deck authors one .DCMATCH card")
    }

    /// What the directive loop's `.DCMATCH` arm does, minus the one step that
    /// needs a Python interpreter: it runs the card through
    /// `run_dc_match_with_abort`, projects the result the binding hands to
    /// Python, and records it under the family's own tag. The push into the
    /// run report is the GIL-bound half and is covered by the Python suite.
    #[test]
    fn a_dcmatch_directive_executes_and_records_its_result() {
        let netlist = divider_with_mismatch(".DCMATCH OUT=V(out) CONTRIBUTORS=0 SIGMA=3");
        let card = dcmatch_card(&netlist);
        let result = rspice_core::Engine::new(rspice_core::SimulationConfig::default())
            .run_dc_match(&netlist, &card)
            .expect("the divider's mismatch variance solves");
        let projected = PyDcMatchResult::from_core(&result);

        // V(out) = Vs*R2/(R1+R2), so d/dR1 = -Vs*R2/(R1+R2)^2 and
        // d/dR2 = Vs*R1/(R1+R2)^2: algebra computed here from the deck's own
        // declared values rather than compared against a recorded number.
        let (source, r1, r2, sigma) = (1.0_f64, 1.0e3_f64, 2.0e3_f64, 10.0_f64);
        let sum = r1 + r2;
        let from_r1 = source * r2 / (sum * sum) * sigma;
        let from_r2 = source * r1 / (sum * sum) * sigma;
        let expected = (from_r1 * from_r1 + from_r2 * from_r2).sqrt();

        assert_eq!(projected.output, "V(OUT)");
        let error = (projected.sigma_total - expected).abs() / expected;
        assert!(
            error < 1.0e-3,
            "sigma_total {} against the analytic {expected} (relative error {error})",
            projected.sigma_total
        );
        assert!(
            (projected.nominal_value - source * r2 / sum).abs() < 1.0e-9,
            "nominal V(out) is {}",
            projected.nominal_value
        );
        assert_eq!(projected.sigma_mismatch, projected.sigma_total);
        assert_eq!(projected.sigma_process, 0.0);
        assert_eq!(projected.sigma_multiplier, 3.0);
        assert_eq!(projected.evaluated_contributors, 6);

        let analysis = netlist
            .analyses
            .iter()
            .find(|analysis| matches!(analysis, AnalysisCommand::DcMatch(_)))
            .expect("the authored card");
        assert_eq!(analysis_record_kind(analysis), "dcmatch");
    }

    #[test]
    fn a_dcmatch_card_describes_its_non_default_keys() {
        let describe = |card: &str| {
            let netlist = divider_with_mismatch(card);
            let analysis = netlist
                .analyses
                .iter()
                .find(|analysis| matches!(analysis, AnalysisCommand::DcMatch(_)))
                .expect("the authored card")
                .clone();
            describe_analysis(&analysis)
        };

        // A card that asked for nothing but the probe describes nothing but
        // the probe: every other key is at its documented default.
        assert_eq!(describe(".DCMATCH OUT=V(out)"), ".dcmatch out=v(OUT)");
        assert_eq!(
            describe(
                ".DCMATCH OUT=V(out,in) MISMATCH=no PROCESS=yes CONTRIBUTORS=3 THRESHOLD=0.01 SIGMA=3"
            ),
            ".dcmatch out=v(OUT,IN) mismatch=no process=yes contributors=3 threshold=0.01 sigma=3"
        );
        assert_eq!(describe(".DCMATCH OUT=I(V1)"), ".dcmatch out=i(V1)");
    }
}
