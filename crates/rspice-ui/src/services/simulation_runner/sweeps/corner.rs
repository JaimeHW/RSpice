//! Binding a process corner's model section into a deck.
//!
//! A corner declaration is solved one PVT point at a time, and each point's
//! task carries its own deck. This is where that deck gets the process
//! corner's real model cards in place of the reference ones.

use super::super::error::{
    ServiceRunError, ServiceRunResult, ensure_not_aborted, poll_periodically,
};
use super::types::{CornerRunConfig, REFERENCE_MODEL_BINDING_BEGIN, REFERENCE_MODEL_BINDING_END};
use rspice_core::abort_signal::AbortSignal;
#[cfg(test)]
use rspice_core::abort_signal::NoAbort;

/// Freeze the exact executable source for one process corner. This is what
/// keeps a process axis from being retained as metadata while the solver
/// silently uses the reference model cards.
pub(crate) fn materialize_corner_process_source(
    source: &str,
    config: &CornerRunConfig,
    process: super::types::CornerProcess,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<String> {
    ensure_not_aborted(abort)?;
    config.validate().map_err(ServiceRunError::Failure)?;
    if !config.process_corners.contains(&process) {
        return Err(ServiceRunError::Failure(format!(
            "{} is not an enabled point in the prepared corner contract",
            process.as_keyword()
        )));
    }
    if config.model_bindings.is_empty() {
        return Ok(source.to_owned());
    }
    let stripped = strip_reference_model_binding_with_abort(source, abort)?;
    materialize_corner_process_source_from_stripped(&stripped, config, process, abort)
}

fn materialize_corner_process_source_from_stripped(
    stripped_source: &str,
    config: &CornerRunConfig,
    process: super::types::CornerProcess,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<String> {
    let mut model_cards = Vec::new();
    for (binding_index, binding) in config.model_bindings.iter().enumerate() {
        poll_periodically(abort, binding_index)?;
        if binding.process == process {
            model_cards.push(format!(
                "* RSpice sealed model source: {}\n{}",
                binding.source_label, binding.materialized_model_cards
            ));
        }
    }
    inject_model_cards_with_abort(stripped_source, &model_cards, abort)
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

    use super::super::execution::expand_corner_points;
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
        let deck = format!(
            "binding test\nV1 in 0 1\nR1 in out 1k\nD1 out 0 DFAST\n\
             {REFERENCE_MODEL_BINDING_BEGIN} 1\n.model DFAST D (IS=1e-9)\n\
             {REFERENCE_MODEL_BINDING_END}\n.op\n.end\n"
        );

        let bound = materialize_corner_process_source(&deck, &config, CornerProcess::FF, &NoAbort)
            .expect("the selected FF section supplies DFAST");

        assert!(bound.contains(".model DFAST D (IS=1e-12)"), "{bound}");
        assert!(!bound.contains("IS=1e-9"), "{bound}");
        assert!(
            materialize_corner_process_source(&deck, &config, CornerProcess::TT, &NoAbort)
                .expect_err("TT is not a point of this contract")
                .to_string()
                .contains("not an enabled point")
        );
    }

    #[test]
    fn a_diagonal_sweep_refuses_unequal_axes_rather_than_cycling_the_shorter_one() {
        let unequal = CornerRunConfig {
            process_corners: vec![CornerProcess::TT],
            voltages: vec![0.9, 1.0],
            temperatures_c: vec![-40.0, 27.0, 125.0],
            full_matrix: false,
            ..CornerRunConfig::default()
        };

        let error = unequal
            .validate()
            .expect_err("2 and 3 have no index-by-index pairing");
        assert!(error.contains("equal non-scalar axis lengths"), "{error}");

        // A single-valued axis is shared by every point, which is a pairing.
        let shared = CornerRunConfig {
            voltages: vec![1.0],
            ..unequal
        };
        shared
            .validate()
            .expect("a scalar axis pairs with any length");
        let points = expand_corner_points(&shared, 64).expect("diagonal expansion");
        assert_eq!(points.len(), 3);
        assert!(points.iter().all(|point| point.voltage == 1.0));
        assert_eq!(points[2].temperature_c, 125.0);
    }

    #[test]
    fn process_binding_honors_an_abort_raised_while_it_reads_the_deck() {
        let abort = AbortOnPoll::new(1);
        let config = CornerRunConfig {
            process_corners: vec![CornerProcess::FF],
            model_bindings: vec![CornerModelBinding {
                process: CornerProcess::FF,
                source_label: "models.lib [FF]".to_owned(),
                section: Some("FF".to_owned()),
                materialized_model_cards: ".model DFAST D (IS=1e-12)".to_owned(),
            }],
            ..CornerRunConfig::default()
        };

        let result = materialize_corner_process_source(
            "abort\nR1 1 0 1k\n.op\n.end\n",
            &config,
            CornerProcess::FF,
            &abort,
        );

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
        assert!(abort.polls.load(Ordering::Relaxed) >= 1);
    }
}
