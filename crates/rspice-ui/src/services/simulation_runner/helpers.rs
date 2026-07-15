use std::path::{Path, PathBuf};

use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::netlist::ElementKind;

use super::error::{ensure_not_aborted, poll_periodically};
use super::{ServiceRunError, ServiceRunResult};

pub(crate) fn parse_runner_netlist_with_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<rspice_core::Netlist> {
    ensure_not_aborted(abort)?;
    let parse_source = runner_parse_source(source_path);
    let parsed =
        rspice_core::Netlist::parse_with_path_and_abort(netlist_text, &parse_source, abort)
            .map_err(|error| match error {
                rspice_core::netlist::ParseWithAbortError::Aborted => ServiceRunError::Aborted,
                rspice_core::netlist::ParseWithAbortError::Parse(error) => {
                    ServiceRunError::Failure(format!("Parse error: {error}"))
                }
            });
    ensure_not_aborted(abort)?;
    parsed
}

fn runner_parse_source(source_path: Option<&Path>) -> PathBuf {
    const GENERATED_NETLIST_NAME: &str = "__rspice_ui_runner_generated__.cir";

    match source_path {
        Some(path) if path.is_dir() => path.join(GENERATED_NETLIST_NAME),
        Some(path) => path.to_path_buf(),
        None => std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(GENERATED_NETLIST_NAME),
    }
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

pub(super) fn is_ground_like(name: &str) -> bool {
    matches!(
        name.trim().to_ascii_lowercase().as_str(),
        "0" | "gnd" | "ground"
    )
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

    let sweep_type = sweep_type.to_lowercase();
    ensure_not_aborted(abort)?;
    match sweep_type.as_str() {
        "dec" | "decade" => {
            let num_decades = (stop / start).log10();
            generate_log_frequency_points(start, stop, points, num_decades, abort)
        }
        "oct" | "octave" => {
            let num_octaves = (stop / start).log2();
            generate_log_frequency_points(start, stop, points, num_octaves, abort)
        }
        "lin" | "linear" => {
            let mut frequencies = frequency_buffer(points)?;
            for idx in 0..points {
                poll_periodically(abort, idx)?;
                let t = idx as f64 / (points - 1).max(1) as f64;
                frequencies.push(start + t * (stop - start));
            }
            ensure_not_aborted(abort)?;
            Ok(frequencies)
        }
        _ => {
            ensure_not_aborted(abort)?;
            Err(ServiceRunError::Failure(format!(
                "unknown frequency sweep type '{sweep_type}'; expected lin, dec, or oct"
            )))
        }
    }
}

fn generate_log_frequency_points(
    start: Value,
    stop: Value,
    points_per_unit: usize,
    units: Value,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Value>> {
    ensure_not_aborted(abort)?;
    let requested = (points_per_unit as f64) * units;
    if !requested.is_finite() || requested > usize::MAX as f64 {
        return Err(ServiceRunError::Failure(
            "frequency sweep requests too many points".to_string(),
        ));
    }
    let total_points = (requested.round() as usize).max(2);
    let mut frequencies = frequency_buffer(total_points)?;
    for idx in 0..total_points {
        poll_periodically(abort, idx)?;
        let t = idx as f64 / (total_points - 1) as f64;
        frequencies.push(start * (stop / start).powf(t));
    }
    ensure_not_aborted(abort)?;
    Ok(frequencies)
}

fn frequency_buffer(capacity: usize) -> ServiceRunResult<Vec<Value>> {
    let mut frequencies = Vec::new();
    frequencies.try_reserve_exact(capacity).map_err(|error| {
        ServiceRunError::Failure(format!(
            "frequency sweep allocation for {capacity} points failed: {error}"
        ))
    })?;
    Ok(frequencies)
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
    fn voltage_signal_normalization_accepts_arbitrary_unicode_without_panicking() {
        assert_eq!(normalize_voltage_signal_name("é"), "é");
        assert_eq!(normalize_voltage_signal_name("💥)"), "💥)");
        assert_eq!(normalize_voltage_signal_name("💥V(out)"), "💥V(OUT)");
        assert_eq!(normalize_voltage_signal_name("V(Δnode)"), "ΔNODE");
    }
}
