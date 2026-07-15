use super::super::error::{
    ServiceRunError, ServiceRunResult, ensure_not_aborted, poll_periodically,
};
use super::execution::{expand_corner_points_with_abort, run_corner_sweep};
use super::mapping::map_corner_results;
use super::netlist_mutation::infer_nominal_supply_voltage;
use super::sweep_points::extract_temp_points_with_abort;
use super::types::{
    CornerData, CornerRunConfig, REFERENCE_MODEL_BINDING_BEGIN, REFERENCE_MODEL_BINDING_END,
};
use rspice_core::abort_signal::{AbortSignal, NoAbort};
use std::collections::HashMap;
use std::path::Path;

/// Run corner analysis from `.TEMP` commands in the netlist.
///
/// This compatibility entry point executes temperature-only TT/nominal sweeps.
pub fn run_corner_analysis(netlist_text: &str) -> Result<CornerData, String> {
    run_corner_analysis_with_abort(netlist_text, &NoAbort).map_err(|error| error.to_string())
}

/// Run corner analysis with cooperative cancellation.
pub fn run_corner_analysis_with_abort(
    netlist_text: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<CornerData> {
    run_corner_analysis_with_source_path_and_abort(netlist_text, None, abort)
}

/// Run corner analysis from `.TEMP` commands in the netlist, resolving relative
/// includes from the source path when provided.
pub fn run_corner_analysis_with_source_path(
    netlist_text: &str,
    source_path: Option<&Path>,
) -> Result<CornerData, String> {
    run_corner_analysis_with_source_path_and_abort(netlist_text, source_path, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run corner analysis with source-path resolution and cooperative
/// cancellation through parsing, point execution, and result mapping.
pub fn run_corner_analysis_with_source_path_and_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<CornerData> {
    let netlist = super::super::parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    let temperatures = extract_temp_points_with_abort(&netlist, abort)?;

    if temperatures.is_empty() {
        return Err(ServiceRunError::Failure(
            "Corner analysis requires at least one .TEMP command".to_string(),
        ));
    }

    let config = CornerRunConfig {
        temperatures_c: temperatures,
        ..Default::default()
    };
    run_corner_analysis_with_netlist(&netlist, &config, abort)
}

/// Run corner analysis with explicit process/voltage/temperature configuration.
pub fn run_corner_analysis_with_config(
    netlist_text: &str,
    config: &CornerRunConfig,
) -> Result<CornerData, String> {
    run_corner_analysis_with_config_and_abort(netlist_text, config, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run an explicitly configured corner analysis with cooperative
/// cancellation.
pub fn run_corner_analysis_with_config_and_abort(
    netlist_text: &str,
    config: &CornerRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<CornerData> {
    run_corner_analysis_with_config_and_source_path_and_abort(netlist_text, config, None, abort)
}

/// Run corner analysis with explicit process/voltage/temperature configuration
/// and a source path used to resolve relative includes and model file
/// references.
pub fn run_corner_analysis_with_config_and_source_path(
    netlist_text: &str,
    config: &CornerRunConfig,
    source_path: Option<&Path>,
) -> Result<CornerData, String> {
    run_corner_analysis_with_config_and_source_path_and_abort(
        netlist_text,
        config,
        source_path,
        &NoAbort,
    )
    .map_err(|error| error.to_string())
}

/// Run an explicitly configured corner analysis with source-path resolution
/// and cooperative cancellation.
pub fn run_corner_analysis_with_config_and_source_path_and_abort(
    netlist_text: &str,
    config: &CornerRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<CornerData> {
    ensure_not_aborted(abort)?;
    config.validate().map_err(ServiceRunError::Failure)?;
    ensure_not_aborted(abort)?;
    if config.model_bindings.is_empty() {
        let netlist =
            super::super::parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
        return run_corner_analysis_with_netlist(&netlist, config, abort);
    }
    run_corner_analysis_with_bound_models(netlist_text, config, source_path, abort)
}

fn run_corner_analysis_with_bound_models(
    netlist_text: &str,
    config: &CornerRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<CornerData> {
    let points = expand_corner_points_with_abort(config, abort)?;
    if points.is_empty() {
        return Err(ServiceRunError::Failure(
            "Corner analysis produced no corner points".to_owned(),
        ));
    }
    let source_without_reference = strip_reference_model_binding_with_abort(netlist_text, abort)?;
    let mut process_netlists = HashMap::new();
    for (process_index, process) in config.process_corners.iter().enumerate() {
        poll_periodically(abort, process_index)?;
        if process_netlists.contains_key(process) {
            continue;
        }
        let mut model_cards = Vec::new();
        for (binding_index, binding) in config.model_bindings.iter().enumerate() {
            poll_periodically(abort, binding_index)?;
            if binding.process == *process {
                model_cards.push(format!(
                    "* RSpice sealed model source: {}\n{}",
                    binding.source_label, binding.materialized_model_cards
                ));
            }
        }
        let source = inject_model_cards_with_abort(&source_without_reference, &model_cards, abort)?;
        process_netlists.insert(*process, source);
    }

    let mut parsed_by_process = HashMap::new();
    for (process_index, (process, source)) in process_netlists.into_iter().enumerate() {
        poll_periodically(abort, process_index)?;
        let parsed = super::super::parse_runner_netlist_with_abort(&source, source_path, abort)
            .map_err(|error| match error {
                ServiceRunError::Aborted => ServiceRunError::Aborted,
                ServiceRunError::Failure(message) => ServiceRunError::Failure(format!(
                    "{} model-section binding failed: {message}",
                    process.as_keyword()
                )),
            })?;
        parsed_by_process.insert(process, parsed);
    }

    let mut results = Vec::with_capacity(points.len());
    for (point_index, point) in points.iter().enumerate() {
        poll_periodically(abort, point_index)?;
        let netlist = parsed_by_process.get(&point.process).ok_or_else(|| {
            ServiceRunError::Failure(format!(
                "No parsed model binding exists for {}",
                point.process.as_keyword()
            ))
        })?;
        let nominal_voltage = match config.nominal_voltage {
            Some(voltage) => voltage,
            None => infer_nominal_supply_voltage(netlist, abort)?.unwrap_or(1.0),
        };
        results.extend(run_corner_sweep(
            netlist,
            std::slice::from_ref(point),
            config,
            nominal_voltage,
            abort,
        )?);
    }
    finish_corner_data(&points, results, config, abort)
}

fn run_corner_analysis_with_netlist(
    netlist: &rspice_core::Netlist,
    config: &CornerRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<CornerData> {
    ensure_not_aborted(abort)?;
    config.validate().map_err(ServiceRunError::Failure)?;
    ensure_not_aborted(abort)?;
    let points = expand_corner_points_with_abort(config, abort)?;
    if points.is_empty() {
        return Err(ServiceRunError::Failure(
            "Corner analysis produced no corner points".to_string(),
        ));
    }

    let nominal_voltage = match config.nominal_voltage {
        Some(voltage) => voltage,
        None => infer_nominal_supply_voltage(netlist, abort)?.unwrap_or(1.0),
    };
    let results = run_corner_sweep(netlist, &points, config, nominal_voltage, abort)?;
    if results.is_empty() {
        return Err(ServiceRunError::Failure(
            "Corner analysis produced no converged corner points".to_string(),
        ));
    }

    finish_corner_data(&points, results, config, abort)
}

fn finish_corner_data(
    points: &[super::types::CornerPoint],
    results: Vec<(super::types::CornerPoint, super::types::SweepPointResult)>,
    config: &CornerRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<CornerData> {
    ensure_not_aborted(abort)?;
    if results.is_empty() {
        return Err(ServiceRunError::Failure(
            "Corner analysis produced no converged corner points".to_owned(),
        ));
    }
    let num_failures = points.len().saturating_sub(results.len());
    let metric = config.base_mode.metric_label();
    let (x_values, x_label, x_unit, temperatures_c, corner_labels, voltages) =
        map_corner_results(&results, metric, abort)?;

    Ok(CornerData {
        x_values,
        x_label,
        x_unit,
        num_points: temperatures_c.len(),
        temperatures_c,
        corner_labels,
        voltages,
        num_failures,
    })
}

#[cfg(test)]
fn strip_reference_model_binding(source: &str) -> Result<String, String> {
    strip_reference_model_binding_with_abort(source, &NoAbort).map_err(|error| error.to_string())
}

fn strip_reference_model_binding_with_abort(
    source: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<String> {
    ensure_not_aborted(abort)?;
    let mut lines = Vec::new();
    for (line_index, line) in source.lines().enumerate() {
        poll_periodically(abort, line_index)?;
        lines.push(line);
    }
    ensure_not_aborted(abort)?;
    let mut result = Vec::new();
    let mut saw_binding = false;
    let mut index = 0usize;
    while index < lines.len() {
        poll_periodically(abort, index)?;
        let line = lines[index];
        let trimmed = line.trim();
        if let Some(count) = trimmed.strip_prefix(REFERENCE_MODEL_BINDING_BEGIN) {
            if saw_binding {
                return Err(ServiceRunError::Failure(
                    "Malformed reference model-binding block".to_owned(),
                ));
            }
            saw_binding = true;
            let count = count.trim().parse::<usize>().map_err(|_| {
                ServiceRunError::Failure(
                    "Reference model-binding block has an invalid line count".to_string(),
                )
            })?;
            let end_index = index
                .checked_add(count)
                .and_then(|index| index.checked_add(1))
                .ok_or_else(|| {
                    ServiceRunError::Failure(
                        "Reference model-binding block line count overflows".to_string(),
                    )
                })?;
            if end_index >= lines.len() || lines[end_index].trim() != REFERENCE_MODEL_BINDING_END {
                return Err(ServiceRunError::Failure(
                    "Reference model-binding block line count does not reach its end marker"
                        .to_owned(),
                ));
            }
            index = end_index + 1;
            continue;
        }
        if trimmed == REFERENCE_MODEL_BINDING_END {
            return Err(ServiceRunError::Failure(
                "Reference model-binding block ends without a start marker".to_owned(),
            ));
        }
        result.push(line);
        index += 1;
    }
    let mut stripped = result.join("\n");
    if source.ends_with('\n') {
        stripped.push('\n');
    }
    ensure_not_aborted(abort)?;
    Ok(stripped)
}

#[cfg(test)]
fn inject_model_cards(source: &str, model_cards: &[String]) -> String {
    inject_model_cards_with_abort(source, model_cards, &NoAbort)
        .expect("NoAbort model-card injection cannot be cancelled")
}

fn inject_model_cards_with_abort(
    source: &str,
    model_cards: &[String],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<String> {
    ensure_not_aborted(abort)?;
    if model_cards.is_empty() {
        return Ok(source.to_owned());
    }
    let mut lines = Vec::new();
    for (index, line) in source.lines().enumerate() {
        poll_periodically(abort, index)?;
        lines.push(line.to_owned());
    }
    let mut insertion_idx = lines.len();
    for (index, line) in lines.iter().enumerate() {
        poll_periodically(abort, index)?;
        if line
            .split_whitespace()
            .next()
            .is_some_and(|directive| directive.eq_ignore_ascii_case(".end"))
        {
            insertion_idx = index;
            break;
        }
    }
    let mut copied_cards = Vec::with_capacity(model_cards.len());
    for (index, card) in model_cards.iter().enumerate() {
        poll_periodically(abort, index)?;
        copied_cards.push(card.clone());
    }
    lines.splice(insertion_idx..insertion_idx, copied_cards);
    ensure_not_aborted(abort)?;
    let mut merged = lines.join("\n");
    if source.ends_with('\n') {
        merged.push('\n');
    }
    ensure_not_aborted(abort)?;
    Ok(merged)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::super::types::{CornerBaseMode, CornerModelBinding, CornerProcess};
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
    }

    impl AbortSignal for AbortOnPoll {
        fn is_aborted(&self) -> bool {
            self.polls.fetch_add(1, Ordering::Relaxed) + 1 >= self.abort_on
        }
    }

    #[test]
    fn process_binding_replaces_reference_binding_block() {
        let source = format!(
            "title\nR1 in 0 1k\n{REFERENCE_MODEL_BINDING_BEGIN} 1\n.model old D\n{REFERENCE_MODEL_BINDING_END}\n.op\n.end\n"
        );

        let stripped = strip_reference_model_binding(&source).expect("marker block is valid");
        let rebound = inject_model_cards(&stripped, &[".model new D".to_owned()]);

        assert!(!rebound.contains(".model old"));
        assert!(rebound.contains(".model new D"));
        assert!(rebound.find(".model new").unwrap() < rebound.find(".end").unwrap());
    }

    #[test]
    fn process_binding_is_inserted_after_hierarchical_subcircuits() {
        let source = "hierarchical\n.subckt child in out\nR1 in out 1k\n.ends child\nX1 in out child\n.op\n.end\n";
        let rebound = inject_model_cards(source, &[".model new D".to_owned()]);

        let subckt_end = rebound.find(".ends child").expect("subcircuit end remains");
        let binding = rebound.find(".model new D").expect("model cards inserted");
        let terminal_end = rebound.rfind("\n.end\n").expect("terminal end remains");
        assert!(subckt_end < binding, "{rebound}");
        assert!(binding < terminal_end, "{rebound}");
    }

    #[test]
    fn process_binding_precedes_annotated_terminal_end_cards() {
        for terminal in [".end ; terminal comment", ".END $ terminal comment"] {
            let source = format!("annotated terminal\nR1 1 0 1k\n{terminal}\n");
            let rebound = inject_model_cards(&source, &[".model new D".to_owned()]);

            let binding = rebound.find(".model new D").expect("model cards inserted");
            let end = rebound.find(terminal).expect("terminal retained");
            assert!(binding < end, "{rebound}");
        }
    }

    #[test]
    fn reference_binding_line_count_ignores_hostile_marker_text_inside_model_cards() {
        let source = format!(
            "title\n{REFERENCE_MODEL_BINDING_BEGIN} 3\n{REFERENCE_MODEL_BINDING_END}\n{REFERENCE_MODEL_BINDING_BEGIN} 999\n.model hostile D\n{REFERENCE_MODEL_BINDING_END}\nR1 1 0 1k\n.end\n"
        );

        let stripped = strip_reference_model_binding(&source)
            .expect("payload marker text is data under the exact line-count contract");
        assert!(!stripped.contains("hostile"), "{stripped}");
        assert!(
            !stripped.contains(REFERENCE_MODEL_BINDING_BEGIN),
            "{stripped}"
        );
        assert!(stripped.contains("R1 1 0 1k"), "{stripped}");

        let malformed = format!(
            "title\n{REFERENCE_MODEL_BINDING_BEGIN} 2\n.model only_one D\n{REFERENCE_MODEL_BINDING_END}\n.end\n"
        );
        assert!(
            strip_reference_model_binding(&malformed)
                .expect_err("incorrect payload count must fail")
                .contains("line count")
        );
    }

    #[test]
    fn explicit_library_section_drives_non_typical_corner() {
        let config = CornerRunConfig {
            process_corners: vec![CornerProcess::FF],
            voltages: vec![1.0],
            temperatures_c: vec![27.0],
            nominal_voltage: Some(1.0),
            base_mode: CornerBaseMode::Op,
            model_bindings: vec![CornerModelBinding {
                process: CornerProcess::FF,
                source_label: "models.lib [FF]".to_owned(),
                section: Some("FF".to_owned()),
                materialized_model_cards: ".model DFAST D (IS=1e-12)".to_owned(),
            }],
            ..CornerRunConfig::default()
        };
        let deck = "binding test\nV1 in 0 1\nR1 in out 1k\nD1 out 0 DFAST\n.op\n.end\n";

        let result = run_corner_analysis_with_config(deck, &config)
            .expect("the selected FF section supplies DFAST");

        assert_eq!(result.num_points, 1);
        assert_eq!(result.corner_labels, vec!["FF_1.000000V_27.000000C"]);
    }

    #[test]
    fn corner_honors_early_abort_before_invalid_input() {
        let abort = AbortOnPoll::new(1);
        let result = run_corner_analysis_with_abort("invalid", &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn corner_honors_abort_during_nested_point_expansion() {
        let config = CornerRunConfig {
            temperatures_c: (0..128).map(|index| index as f64).collect(),
            ..CornerRunConfig::default()
        };
        let abort = AbortOnPoll::new(10);
        let result = run_corner_analysis_with_config_and_abort(
            "corner cancellation\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n",
            &config,
            &abort,
        );

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
        assert!(abort.polls.load(Ordering::Relaxed) >= 10);
    }
}
