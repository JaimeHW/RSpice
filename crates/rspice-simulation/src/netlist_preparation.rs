//! Netlist preparation for exact source composition and process/supply corners.

use rspice_core::abort_signal::AbortSignal;
#[cfg(test)]
use rspice_core::abort_signal::NoAbort;
use rspice_core::netlist::{ElementKind, SourceSpec};
use rspice_core::{SimulationConfig, SimulationConfigOverrides, Value, resolve_simulation_config};

use crate::error::{ServiceRunError, ServiceRunResult, ensure_not_aborted, poll_periodically};
use crate::sweeps::CornerRunConfig;

pub mod dependencies;
mod include_search;
pub use include_search::{IncludeSearchChain, IncludeSearchEntry};

pub mod measurements;
pub mod owned_source;

mod external_sources;
pub use external_sources::{
    contains_external_include_directive, deferred_external_source_reason, executable_logical_lines,
    reject_deferred_external_sources_with_project_runtimes,
};

mod parsing;
pub use parsing::{
    parse_analysis_netlist_with_abort, parse_runner_netlist_with_abort,
    parse_runner_netlist_with_options_and_abort,
    parse_runner_netlist_with_resource_limits_and_abort, validated_executable_hierarchy,
};
pub(crate) use parsing::{
    validated_executable_hierarchy_with_limits_and_abort,
    validated_parsed_hierarchy_with_limits_and_abort,
};

pub const REFERENCE_MODEL_BINDING_BEGIN: &str = "* RSPICE REFERENCE MODEL BINDING BEGIN";
pub const REFERENCE_MODEL_BINDING_END: &str = "* RSPICE REFERENCE MODEL BINDING END";

/// Locate the first executable `.end` in a title-bearing SPICE deck.
/// The UI's generated and materialized execution decks use Ngspice syntax.
pub fn terminal_end_card_offset(source: &str) -> Option<usize> {
    terminal_end_card_offset_with_abort(source, &rspice_core::NoAbort)
        .expect("NoAbort source scanning cannot be cancelled")
}

fn terminal_end_card_offset_with_abort(
    source: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Option<usize>> {
    ensure_not_aborted(abort)?;
    let mut offset = 0;
    for (index, line) in source.split_inclusive('\n').enumerate() {
        poll_periodically(abort, index)?;
        if index > 0
            && rspice_core::netlist::is_spice_end_card(
                line,
                rspice_core::config::ExpressionDialect::Ngspice,
            )
        {
            return Ok(Some(offset));
        }
        offset += line.len();
    }
    ensure_not_aborted(abort)?;
    Ok(None)
}

/// Insert generated cards before the first `.end`, or at EOF when absent.
/// Authored bytes remain unchanged. New records use the first line's newline
/// convention, and the original trailing-newline choice is retained.
pub fn splice_before_terminal_end_card(source: &str, block: &str) -> String {
    splice_before_terminal_end_card_with_abort(source, block, &rspice_core::NoAbort)
        .expect("NoAbort card insertion cannot be cancelled")
}

fn splice_before_terminal_end_card_with_abort(
    source: &str,
    block: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<String> {
    ensure_not_aborted(abort)?;
    if block.is_empty() {
        return Ok(source.to_owned());
    }
    let end = terminal_end_card_offset_with_abort(source, abort)?.unwrap_or(source.len());
    let newline = if source
        .find('\n')
        .is_some_and(|index| source[..index].ends_with('\r'))
    {
        "\r\n"
    } else {
        "\n"
    };
    let (prefix, suffix) = source.split_at(end);
    let mut result = String::with_capacity(source.len() + block.len() + 2 * newline.len());
    result.push_str(prefix);
    // An empty root still needs its blank title before executable cards.
    if !prefix.ends_with('\n') {
        result.push_str(newline);
    }
    for (index, line) in block.lines().enumerate() {
        poll_periodically(abort, index)?;
        result.push_str(line);
        result.push_str(newline);
    }
    if suffix.is_empty() && !source.ends_with('\n') {
        result.truncate(result.len() - newline.len());
    }
    result.push_str(suffix);
    ensure_not_aborted(abort)?;
    Ok(result)
}

/// Resolve run temperature before parsing so TEMP/TEMPER/VT-dependent
/// parameters and conditional source selection see the selected environment.
/// Callers retain their original source separately for request provenance.
pub fn source_with_run_temperature_with_abort(
    source: &str,
    temperature_celsius: Value,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<String> {
    if !temperature_celsius.is_finite() || temperature_celsius <= -273.15 {
        return Err(ServiceRunError::Failure(
            "Run temperature must be finite and above absolute zero".into(),
        ));
    }
    splice_before_terminal_end_card_with_abort(
        source,
        &format!(".options TEMP={temperature_celsius}"),
        abort,
    )
}

/// Freeze the exact executable source for one process corner. This is what
/// keeps a process axis from being retained as metadata while the solver
/// silently uses the reference model cards.
pub fn materialize_corner_process_source(
    source: &str,
    config: &CornerRunConfig,
    process: rspice_app_types::product::ProcessCorner,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<String> {
    ensure_not_aborted(abort)?;
    config.validate().map_err(ServiceRunError::Failure)?;
    if !config.process_corners.contains(&process) {
        return Err(ServiceRunError::Failure(format!(
            "{} is not an enabled point in the prepared corner contract",
            process.short_name()
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
    process: rspice_app_types::product::ProcessCorner,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<String> {
    let mut model_cards = Vec::new();
    for (binding_index, binding) in config.model_bindings.iter().enumerate() {
        poll_periodically(abort, binding_index)?;
        if binding.process == process {
            model_cards.push(format!(
                "{}{}\n{}",
                rspice_model_library::SEALED_MODEL_SOURCE_MARKER,
                binding.source_label,
                binding.materialized_model_cards
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
    let mut block = String::new();
    for (index, card) in model_cards.iter().enumerate() {
        poll_periodically(abort, index)?;
        if index > 0 {
            block.push('\n');
        }
        block.push_str(card);
    }
    splice_before_terminal_end_card_with_abort(source, &block, abort)
}

pub fn apply_voltage_corner(
    netlist: &mut rspice_core::Netlist,
    corner_voltage: Value,
    nominal_voltage: Value,
    supply_source_names: &[String],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<()> {
    rspice_core::engine::apply_supply_voltage_scale_with_abort(
        netlist,
        corner_voltage,
        nominal_voltage,
        supply_source_names,
        abort,
    )
    .map_err(|error| match error {
        rspice_core::SimulationError::Aborted => ServiceRunError::Aborted,
        error => ServiceRunError::Failure(error.to_string()),
    })
}

pub fn infer_nominal_supply_voltage(
    netlist: &rspice_core::Netlist,
    supply_source_names: &[String],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Option<Value>> {
    ensure_not_aborted(abort)?;
    if supply_source_names.is_empty() {
        return Err(ServiceRunError::Failure(
            "Nominal supply resolution requires at least one explicitly bound source".to_owned(),
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    let mut values = Vec::with_capacity(supply_source_names.len());
    for (index, source_name) in supply_source_names.iter().enumerate() {
        poll_periodically(abort, index)?;
        if source_name.trim().is_empty()
            || source_name != source_name.trim()
            || source_name.chars().any(char::is_control)
            || !seen.insert(source_name.to_ascii_lowercase())
        {
            return Err(ServiceRunError::Failure(format!(
                "Supply source binding {source_name:?} is malformed or duplicated"
            )));
        }
        let Some(element) = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(source_name))
        else {
            return Err(ServiceRunError::Failure(format!(
                "Bound supply source {source_name:?} is absent from the executable netlist"
            )));
        };
        let ElementKind::VoltageSource(spec) = &element.kind else {
            return Err(ServiceRunError::Failure(format!(
                "Bound supply source {source_name:?} is not an independent voltage source"
            )));
        };
        let Some(dc) = dc_value_from_source(spec) else {
            return Err(ServiceRunError::Failure(format!(
                "Bound supply source {source_name:?} has no scalable DC value"
            )));
        };
        let abs_dc = dc.abs();
        if abs_dc <= 1e-15 {
            return Err(ServiceRunError::Failure(format!(
                "Bound supply source {source_name:?} has a zero nominal magnitude"
            )));
        }
        values.push(abs_dc);
    }
    ensure_not_aborted(abort)?;
    Ok(values.into_iter().max_by(|a, b| a.total_cmp(b)))
}

fn dc_value_from_source(spec: &SourceSpec) -> Option<Value> {
    match spec {
        SourceSpec::Dc(v) => Some(*v),
        SourceSpec::DcAc { dc_value, .. } => Some(*dc_value),
        _ => None,
    }
}

/// Insert one sealed Verilog-A directive before the terminal `.end` card.
/// The exact same helper is used by the retained generated artifact and the
/// immutable prepared-run source, preventing display/execution drift.
pub fn project_veriloga_directive(source_key: &str, netlist_alias: &str) -> String {
    format!(".veriloga \"{source_key}\" {netlist_alias}")
}

pub fn append_project_veriloga_directive(
    source: &mut String,
    source_key: &str,
    netlist_alias: &str,
) {
    let directive = project_veriloga_directive(source_key, netlist_alias);
    let end = terminal_end_card_offset(source).unwrap_or(source.len());
    if source[..end]
        .lines()
        .skip(1)
        .any(|line| line.trim().eq_ignore_ascii_case(&directive))
    {
        return;
    }
    *source = splice_before_terminal_end_card(source, &directive);
}

pub fn build_engine_config(
    netlist: &rspice_core::Netlist,
    options: Option<&rspice_simulation_contract::options::SimulationOptions>,
) -> SimulationConfig {
    match options {
        Some(opts) => opts.resolve_simulation_config(Some(&netlist.options)),
        None => resolve_simulation_config(
            &SimulationConfig::default(),
            Some(&netlist.options),
            &SimulationConfigOverrides::default(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;
    use crate::sweeps::CornerBaseMode;
    use rspice_app_types::product::ProcessCorner;
    use rspice_model_library::CornerModelBinding;

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
    fn corner_cards_reach_the_parser_after_an_end_title_and_before_annotations() {
        for terminal in [".end; done", ".END // done", ".end"] {
            let source = format!(".end\r\nR1 1 0 1k\r\n{terminal}\r\n");
            let bound = inject_model_cards(&source, &[".model selected D (IS=1e-12)".to_owned()]);
            let parsed = rspice_core::Netlist::parse(&bound).expect("corner deck parses");
            assert_eq!(parsed.title, ".end", "{bound}");
            assert!(
                parsed
                    .models
                    .iter()
                    .any(|model| model.name.eq_ignore_ascii_case("selected")),
                "{bound}"
            );
            assert!(bound.starts_with(".end\r\nR1 1 0 1k\r\n"), "{bound:?}");
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
            process_corners: vec![ProcessCorner::FF],
            voltages: vec![1.0],
            temperatures_c: vec![27.0],
            nominal_voltage: Some(1.0),
            base_mode: CornerBaseMode::Op,
            model_bindings: vec![CornerModelBinding {
                process: ProcessCorner::FF,
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

        let bound = materialize_corner_process_source(&deck, &config, ProcessCorner::FF, &NoAbort)
            .expect("the selected FF section supplies DFAST");

        assert!(bound.contains(".model DFAST D (IS=1e-12)"), "{bound}");
        assert!(!bound.contains("IS=1e-9"), "{bound}");
        assert!(
            materialize_corner_process_source(&deck, &config, ProcessCorner::TT, &NoAbort)
                .expect_err("TT is not a point of this contract")
                .to_string()
                .contains("not an enabled point")
        );
    }

    #[test]
    fn process_binding_honors_an_abort_raised_while_it_reads_the_deck() {
        let abort = AbortOnPoll::new(1);
        let config = CornerRunConfig {
            process_corners: vec![ProcessCorner::FF],
            model_bindings: vec![CornerModelBinding {
                process: ProcessCorner::FF,
                source_label: "models.lib [FF]".to_owned(),
                section: Some("FF".to_owned()),
                materialized_model_cards: ".model DFAST D (IS=1e-12)".to_owned(),
            }],
            ..CornerRunConfig::default()
        };

        let result = materialize_corner_process_source(
            "abort\nR1 1 0 1k\n.op\n.end\n",
            &config,
            ProcessCorner::FF,
            &abort,
        );

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
        assert!(abort.polls.load(Ordering::Relaxed) >= 1);
    }

    #[test]
    fn generated_card_insertion_retains_body_records_and_optional_termination() {
        for source in [
            "",
            ".end",
            ".end\n",
            "μ title\r\nR1 1 0 1k",
            "μ title\r\nR1 1 0 1k\n.end; done\r\nignored tail",
        ] {
            let composed = splice_before_terminal_end_card(
                source,
                ".param selected=7\n.options reltol=0.012345",
            );
            let parsed = rspice_core::Netlist::parse(&composed).expect("inserted cards execute");
            assert_eq!(parsed.params.get("selected"), Some(7.0), "{composed:?}");
            assert_eq!(parsed.options.reltol, Some(0.012345), "{composed:?}");
            assert_eq!(parsed.title, source.lines().next().unwrap_or_default());
            assert_eq!(composed.ends_with('\n'), source.ends_with('\n'));
            let end = terminal_end_card_offset(source).unwrap_or(source.len());
            assert!(composed.starts_with(&source[..end]), "{composed:?}");
            assert!(composed.ends_with(&source[end..]), "{composed:?}");
            assert_eq!(splice_before_terminal_end_card(source, ""), source);
        }
    }

    #[test]
    fn generated_card_insertion_cancels_during_source_and_payload_scans() {
        let source = format!("title\n{}\n.end\n", "* comment\n".repeat(256));
        let source_abort = AbortOnPoll::new(5);
        assert!(matches!(
            splice_before_terminal_end_card_with_abort(&source, ".op", &source_abort),
            Err(ServiceRunError::Aborted)
        ));
        let block = "* model payload\n".repeat(256);
        let payload_abort = AbortOnPoll::new(6);
        assert!(matches!(
            splice_before_terminal_end_card_with_abort("title\n.end\n", &block, &payload_abort),
            Err(ServiceRunError::Aborted)
        ));
    }

    #[test]
    fn projected_veriloga_cards_reach_the_parser_before_commented_termination() {
        for title in [".end", ".veriloga \"sealed.va\" device", "title"] {
            for terminal in [".end; done", ".END // done", ".end"] {
                let mut source = format!("{title}\r\nR1 1 0 1k\r\n{terminal}\r\n");
                super::append_project_veriloga_directive(&mut source, "sealed.va", "device");
                super::append_project_veriloga_directive(&mut source, "sealed.va", "device");
                let parsed = rspice_core::Netlist::parse(&source).expect("projected deck parses");
                assert_eq!(parsed.title, title, "{source}");
                assert_eq!(parsed.veriloga_includes.len(), 1, "{source}");
                assert_eq!(
                    parsed.veriloga_includes[0].model_name.as_deref(),
                    Some("device")
                );
                assert!(
                    source.starts_with(&format!("{title}\r\nR1 1 0 1k\r\n")),
                    "{source:?}"
                );
            }
        }
        let mut source = "title\n.end; done\n.veriloga \"sealed.va\" device\n".to_owned();
        super::append_project_veriloga_directive(&mut source, "sealed.va", "device");
        let parsed =
            rspice_core::Netlist::parse(&source).expect("a tail directive is not executable");
        assert_eq!(parsed.veriloga_includes.len(), 1, "{source}");
    }
}
