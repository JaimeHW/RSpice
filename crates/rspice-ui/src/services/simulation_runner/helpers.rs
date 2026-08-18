//! Shared runner plumbing.
//!
//! Parsing a deck under the run resource limits, building the output
//! expressions an analysis asks for, and the abort-aware wrappers every
//! runner shares.

use std::path::Path;

use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::netlist::{ElementKind, FreqVariation, StatisticalParamMode};

use super::error::{ensure_not_aborted, poll_periodically};
use super::{ServiceRunError, ServiceRunResult};

pub(crate) fn parse_runner_netlist_with_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<rspice_core::Netlist> {
    parse_runner_netlist_with_resource_limits_and_abort(
        netlist_text,
        source_path,
        rspice_core::ResourceLimits::default(),
        abort,
    )
}

pub(crate) fn parse_runner_netlist_with_resource_limits_and_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    resource_limits: rspice_core::ResourceLimits,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<rspice_core::Netlist> {
    parse_runner_netlist_with_mode_resource_limits_and_abort(
        netlist_text,
        source_path,
        StatisticalParamMode::Nominal,
        resource_limits,
        abort,
    )
}

pub(crate) fn parse_runner_netlist_with_statistical_sampling_and_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<rspice_core::Netlist> {
    parse_runner_netlist_with_mode_resource_limits_and_abort(
        netlist_text,
        source_path,
        StatisticalParamMode::Sample,
        rspice_core::ResourceLimits::default(),
        abort,
    )
}

fn parse_runner_netlist_with_mode_resource_limits_and_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    statistical_mode: StatisticalParamMode,
    resource_limits: rspice_core::ResourceLimits,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<rspice_core::Netlist> {
    ensure_not_aborted(abort)?;
    let options = rspice_core::netlist::NetlistParseOptions {
        statistical_mode,
        resource_limits,
        ..Default::default()
    };
    let parsed = match source_path {
        Some(path) => rspice_core::Netlist::parse_with_path_and_options_and_abort(
            netlist_text,
            path,
            options,
            abort,
        ),
        None => rspice_core::Netlist::parse_with_options_and_abort(netlist_text, options, abort),
    }
    .map_err(|error| match error {
        rspice_core::netlist::ParseWithAbortError::Aborted => ServiceRunError::Aborted,
        rspice_core::netlist::ParseWithAbortError::Parse(
            rspice_core::netlist::ParseError::ResourceLimit(error),
        ) => ServiceRunError::ResourceLimit(error),
        rspice_core::netlist::ParseWithAbortError::Parse(error) => {
            ServiceRunError::Failure(format!("Parse error: {error}"))
        }
    });
    ensure_not_aborted(abort)?;
    parsed
}

pub(super) fn build_voltage_output_expr(output_node: &str, output_ref: Option<&str>) -> String {
    let output_node = output_node.trim();
    let output_ref = output_ref
        .map(str::trim)
        .filter(|name| !name.is_empty() && !is_ground_like(name));
    match output_ref {
        Some(reference) => format!("V({},{})", output_node, reference),
        None => format!("V({})", output_node),
    }
}

/// The runner asks the same question the editor does, and gets the same answer
/// from the same authority: a node the engine folds onto `0` is ground here.
pub(super) fn is_ground_like(name: &str) -> bool {
    crate::state::is_ground_reference(name)
}

pub(crate) fn infer_primary_source_name_with_abort(
    netlist: &rspice_core::Netlist,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Option<String>> {
    ensure_not_aborted(abort)?;
    for element in &netlist.elements {
        ensure_not_aborted(abort)?;
        if matches!(
            &element.kind,
            ElementKind::VoltageSource(_) | ElementKind::CurrentSource(_)
        ) {
            return Ok(Some(element.name.clone()));
        }
    }
    Ok(None)
}

pub(crate) fn netlist_has_independent_source_named_with_abort(
    netlist: &rspice_core::Netlist,
    source_name: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<bool> {
    ensure_not_aborted(abort)?;
    for element in &netlist.elements {
        ensure_not_aborted(abort)?;
        if matches!(
            &element.kind,
            ElementKind::VoltageSource(_) | ElementKind::CurrentSource(_)
        ) && element.name.eq_ignore_ascii_case(source_name)
        {
            return Ok(true);
        }
    }
    Ok(false)
}

pub(crate) fn infer_primary_output_node_with_abort(
    node_names: &[String],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Option<String>> {
    ensure_not_aborted(abort)?;
    for name in node_names.iter().rev() {
        ensure_not_aborted(abort)?;
        if !is_ground_like(name) {
            return Ok(Some(name.clone()));
        }
    }
    Ok(None)
}

pub(super) fn normalize_voltage_signal_name(name: &str) -> String {
    let trimmed = name.trim();
    let voltage_body = trimmed
        .get(..2)
        .filter(|head| head.eq_ignore_ascii_case("V("))
        .and_then(|_| trimmed.strip_suffix(')'))
        .and_then(|body| body.get(2..));
    if let Some(body) = voltage_body {
        return body.trim().to_ascii_uppercase();
    }
    trimmed.to_ascii_uppercase()
}

pub(crate) fn generate_freq_points_with_abort(
    start: Value,
    stop: Value,
    points: usize,
    sweep_type: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Value>> {
    generate_freq_points_with_limit_and_abort(
        start,
        stop,
        points,
        sweep_type,
        rspice_core::ResourceLimits::default().max_analysis_points,
        abort,
    )
}

pub(crate) fn generate_freq_points_with_limit_and_abort(
    start: Value,
    stop: Value,
    points: usize,
    sweep_type: &str,
    max_analysis_points: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Value>> {
    ensure_not_aborted(abort)?;
    let validation = if points == 0 {
        Err(ServiceRunError::Failure(
            "frequency sweep must request at least one point".to_string(),
        ))
    } else if !start.is_finite() || !stop.is_finite() || start <= 0.0 || stop <= 0.0 {
        Err(ServiceRunError::Failure(format!(
            "frequency sweep bounds must be finite and positive (start={start}, stop={stop})"
        )))
    } else if stop < start {
        Err(ServiceRunError::Failure(format!(
            "frequency sweep stop frequency ({stop}) must be greater than or equal to start frequency ({start})"
        )))
    } else {
        Ok(())
    };
    ensure_not_aborted(abort)?;
    validation?;

    let variation = match sweep_type.to_ascii_lowercase().as_str() {
        "dec" | "decade" => FreqVariation::Dec,
        "oct" | "octave" => FreqVariation::Oct,
        "lin" | "linear" => FreqVariation::Lin,
        _ => {
            ensure_not_aborted(abort)?;
            return Err(ServiceRunError::Failure(format!(
                "unknown frequency sweep type '{sweep_type}'; expected lin, dec, or oct"
            )));
        }
    };
    generate_spice_frequency_points_with_limit_and_abort(
        variation,
        points,
        start,
        stop,
        max_analysis_points,
        abort,
    )
}

/// Generate the same grid as the exported SPICE `.ac` request while retaining
/// service-layer cancellation and resource-limit enforcement. Keeping one
/// sweep definition prevents PAC/PNOISE/RF analyses from solving a different
/// set of frequencies than AC, Noise, CLI, or an exported deck.
fn generate_spice_frequency_points_with_limit_and_abort(
    variation: FreqVariation,
    points: usize,
    start: Value,
    stop: Value,
    max_analysis_points: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Value>> {
    ensure_not_aborted(abort)?;
    if variation == FreqVariation::Lin && points > 2 {
        ensure_analysis_point_limit(points, max_analysis_points)?;
    }

    const SWEEP_RELTOL: Value = 1.0e-3;
    let delta = match variation {
        FreqVariation::Dec => {
            if stop / 10.0 < start {
                if stop == start {
                    1.0
                } else {
                    (std::f64::consts::LN_10 / points as Value).exp()
                }
            } else {
                let num_steps = ((stop / start).log10().abs() * points as Value).floor();
                ((stop / start).ln() / num_steps).exp()
            }
        }
        FreqVariation::Oct => (std::f64::consts::LN_2 / points as Value).exp(),
        FreqVariation::Lin => {
            if points > 2 {
                (stop - start) / (points - 1) as Value
            } else {
                0.0
            }
        }
    };
    let frequency_tolerance = match variation {
        FreqVariation::Lin => delta * SWEEP_RELTOL,
        _ => delta * stop * SWEEP_RELTOL,
    };

    let mut frequencies = Vec::new();
    let mut frequency = start;
    while frequency <= stop + frequency_tolerance {
        let requested = frequencies.len().saturating_add(1);
        ensure_analysis_point_limit(requested, max_analysis_points)?;
        poll_periodically(abort, frequencies.len())?;
        frequencies.try_reserve(1).map_err(|error| {
            ServiceRunError::Failure(format!(
                "frequency sweep allocation for {requested} points failed: {error}"
            ))
        })?;
        frequencies.push(frequency);
        match variation {
            FreqVariation::Lin => {
                if delta == 0.0 {
                    break;
                }
                frequency += delta;
            }
            _ => {
                if delta == 1.0 {
                    break;
                }
                frequency *= delta;
            }
        }
    }
    ensure_not_aborted(abort)?;
    debug_assert_eq!(
        frequencies,
        rspice_core::analysis::ac::ac_sweep_frequencies(variation, points, start, stop),
        "service frequency grid must match the exported SPICE sweep"
    );
    Ok(frequencies)
}

fn ensure_analysis_point_limit(requested: usize, limit: usize) -> ServiceRunResult<()> {
    if requested <= limit {
        Ok(())
    } else {
        Err(ServiceRunError::resource_limit(
            rspice_core::ResourceKind::AnalysisPoints,
            requested,
            limit,
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    struct AbortOnPoll {
        abort_on: usize,
        polls: AtomicUsize,
    }

    impl AbortOnPoll {
        fn new(abort_on: usize) -> Self {
            Self {
                abort_on,
                polls: AtomicUsize::new(0),
            }
        }

        fn polls(&self) -> usize {
            self.polls.load(Ordering::Relaxed)
        }
    }

    impl AbortSignal for AbortOnPoll {
        fn is_aborted(&self) -> bool {
            self.polls.fetch_add(1, Ordering::Relaxed) + 1 >= self.abort_on
        }
    }

    #[test]
    fn frequency_generation_polls_inside_the_point_loop() {
        let abort = AbortOnPoll::new(5);
        let result = generate_freq_points_with_abort(1.0, 100.0, 100, "lin", &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn cancellation_precedes_invalid_frequency_configuration() {
        let abort = AbortOnPoll::new(2);
        let result = generate_freq_points_with_abort(0.0, -1.0, 0, "invalid", &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn frequency_generation_enforces_configured_analysis_point_limit() {
        let result = generate_freq_points_with_limit_and_abort(
            1.0,
            10.0,
            3,
            "lin",
            2,
            &rspice_core::NoAbort,
        );

        assert!(matches!(
            result,
            Err(ServiceRunError::ResourceLimit(
                rspice_core::ResourceLimitError {
                    resource: rspice_core::ResourceKind::AnalysisPoints,
                    requested: 3,
                    limit: 2,
                }
            ))
        ));
    }

    #[test]
    fn service_frequency_grids_match_exported_spice_semantics() {
        for (variation, name, points, start, stop) in [
            (FreqVariation::Dec, "dec", 10, 1.0, 1.0e6),
            (FreqVariation::Dec, "dec", 10, 100.0, 300.0),
            (FreqVariation::Oct, "oct", 3, 10.0, 80.0),
            (FreqVariation::Lin, "lin", 7, 1.0e3, 2.0e3),
            (FreqVariation::Lin, "lin", 2, 1.0e3, 2.0e3),
        ] {
            let service =
                generate_freq_points_with_abort(start, stop, points, name, &rspice_core::NoAbort)
                    .expect("valid service sweep");
            let exported =
                rspice_core::analysis::ac::ac_sweep_frequencies(variation, points, start, stop);
            assert_eq!(service, exported, "{name} sweep drifted from SPICE");
        }
    }

    #[test]
    fn logarithmic_frequency_generation_enforces_the_limit_during_expansion() {
        let result = generate_freq_points_with_limit_and_abort(
            1.0,
            1.0e6,
            10,
            "dec",
            60,
            &rspice_core::NoAbort,
        );

        assert!(matches!(
            result,
            Err(ServiceRunError::ResourceLimit(
                rspice_core::ResourceLimitError {
                    resource: rspice_core::ResourceKind::AnalysisPoints,
                    requested: 61,
                    limit: 60,
                }
            ))
        ));
    }

    #[test]
    fn cancellation_precedes_a_parse_failure() {
        let abort = AbortOnPoll::new(2);
        let result = parse_runner_netlist_with_abort("not a valid deck", None, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn cancellation_during_parser_work_remains_typed() {
        let mut source = String::from("typed parser cancellation\n");
        for index in 0..4_096 {
            source.push_str(&format!("R{index} n{index} 0 1k\n"));
        }
        source.push_str(".end\n");
        let abort = AbortOnPoll::new(100);

        let result = parse_runner_netlist_with_abort(&source, None, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
        assert!(
            abort.polls() >= 100,
            "service adapter must preserve a mid-parse abort"
        );
    }

    #[test]
    fn parser_resource_limit_remains_typed() {
        let source = "typed service limit\nR1 in 0 1k\n.end\n";
        let mut limits = rspice_core::ResourceLimits::default();
        limits.max_netlist_bytes = source.len() - 1;

        let result = parse_runner_netlist_with_resource_limits_and_abort(
            source,
            None,
            limits,
            &rspice_core::NoAbort,
        );

        assert!(matches!(
            result,
            Err(ServiceRunError::ResourceLimit(
                rspice_core::ResourceLimitError {
                    resource: rspice_core::ResourceKind::NetlistBytes,
                    requested,
                    limit,
                }
            )) if requested == source.len() && limit == source.len() - 1
        ));
    }

    #[test]
    fn pathless_service_parse_remains_in_memory_only() {
        let source = "pathless prepared service deck\nV1 in 0 1\nR1 in 0 1k\n.end\n";
        let parsed = parse_runner_netlist_with_abort(source, None, &rspice_core::NoAbort)
            .expect("self-contained specialized-analysis deck parses without filesystem authority");

        assert_eq!(parsed.source_path, None);
    }

    #[test]
    fn ordinary_runner_parses_statistics_nominally_and_mc_sampling_is_explicit() {
        let source = "statistical runner mode\n.options seed=7\n.param delta={agauss(2,1,1)}\nR1 in 0 {1k+delta}\n.end\n";
        let nominal = parse_runner_netlist_with_abort(source, None, &rspice_core::NoAbort)
            .expect("ordinary runner deck parses");
        let sampled = parse_runner_netlist_with_statistical_sampling_and_abort(
            source,
            None,
            &rspice_core::NoAbort,
        )
        .expect("statistical Monte Carlo trial parses");

        assert_eq!(nominal.params.get("delta"), Some(2.0));
        assert_ne!(sampled.params.get("delta"), Some(2.0));
    }

    #[test]
    fn voltage_signal_normalization_accepts_arbitrary_unicode_without_panicking() {
        assert_eq!(normalize_voltage_signal_name("é"), "é");
        assert_eq!(normalize_voltage_signal_name("💥)"), "💥)");
        assert_eq!(normalize_voltage_signal_name("💥V(out)"), "💥V(OUT)");
        assert_eq!(normalize_voltage_signal_name("V(Δnode)"), "ΔNODE");
    }
}
