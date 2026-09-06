//! Value Change Dump publication.
//!
//! A dump is the transient's *event* timelines, not its waveform table: the
//! sparse schedule the event solver accepted, at the times it accepted it. It
//! is therefore written from the retained `TransientEvents` evidence and never
//! from the analog grid, and a result that carries no event history is refused
//! by name rather than handed an empty file.
//!
//! # Agreement with the command line
//!
//! `rspice run -f vcd` and this export are the same bytes for the same run.
//! Both project through [`event_vcd_document`] under one `$scope module
//! events` and both serialise with [`write_vcd`], so the timescale rule,
//! identifier assignment and byte layout are the core's single implementation
//! rather than two that happen to agree today.
//!
//! # What a dump does not carry
//!
//! VCD has four bit states and no drive strength, so the twelve XSPICE
//! resolved states collapse onto `0`, `1`, `x` and `z`: a resistive one and a
//! strong one are both `1`. That is a property of the format, not of this
//! encoder, and it is stated to the reader on every successful publication.
//! A reader who needs the strength band wants the RSpice Result Bundle.

use super::{NO_ACTIVE_ANALYSIS_MESSAGE, note_result_export_failure, note_result_export_success};
use crate::state::AnalysisResultPayload;
use crate::workbench::app_state::AppState;
use crate::workbench::documents::result_document::view_context::ResolvedResultView;
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

use rspice_core::engine::{DigitalTrace, DigitalTracePoint, RealTrace, RealTracePoint};
use rspice_core::execution::event_vcd_document;
use rspice_core::io::write_vcd;
use rspice_core::xspice::DigitalValue;

/// The single scope every RSpice dump declares. `rspice-cli`'s `vcd_io.rs`
/// uses this exact string; the two exports differ the moment they disagree.
const EVENT_SCOPE: &str = "events";

/// VCD is ASCII text. There is no registered media type for it, and claiming
/// one would be an invention rather than a fact about the bytes.
const MIME_TYPE: &str = "text/plain;charset=utf-8";

const NO_EVENT_EVIDENCE_MESSAGE: &str = "A Value Change Dump carries the digital and real event timelines a transient's event \
     solver accepted. The active result retains none, so there is nothing to dump. Select CSV, \
     TSV, or an RSpice bundle to publish its waveform table instead.";

const EMPTY_EVENT_EVIDENCE_MESSAGE: &str = "This transient retained an event history with no node in it, so a Value Change Dump would \
     declare no signal and record no change. Publishing it would be indistinguishable from a \
     failed export.";

/// What a published dump contains, for the completion message.
#[derive(Debug)]
pub(super) struct PreparedVcd {
    pub(super) bytes: Vec<u8>,
    pub(super) node_count: usize,
    pub(super) change_count: usize,
}

/// Build the core event traces this crate's retained evidence stands for.
///
/// Refuses an unrecognised event code by name. The retained-evidence check
/// already bounds `value_code` at 12, so reaching this is a corrupted
/// dataset — which is exactly the case that must not be written out as if it
/// were a level.
fn core_traces(
    digital_traces: &[crate::state::DigitalEventTraceEvidence],
    real_traces: &[crate::state::RealEventTraceEvidence],
) -> Result<(Vec<DigitalTrace>, Vec<RealTrace>), String> {
    let mut digital = Vec::with_capacity(digital_traces.len());
    for trace in digital_traces {
        let mut points = Vec::with_capacity(trace.points.len());
        for point in &trace.points {
            let value = DigitalValue::from_event_code(point.value_code).ok_or_else(|| {
                format!(
                    "node '{}' records event code {} at {} s, which is not one of the thirteen \
                     XSPICE event codes",
                    trace.node_name, point.value_code, point.time_s
                )
            })?;
            points.push(DigitalTracePoint {
                time: point.time_s,
                value,
            });
        }
        digital.push(DigitalTrace {
            node_name: trace.node_name.clone(),
            points,
        });
    }
    let real = real_traces
        .iter()
        .map(|trace| RealTrace {
            node_name: trace.node_name.clone(),
            points: trace
                .points
                .iter()
                .map(|point| RealTracePoint {
                    time: point.time_s,
                    value: point.value,
                })
                .collect(),
        })
        .collect();
    Ok((digital, real))
}

/// Serialise one analysis's retained event history as a dump.
///
/// Everything that can refuse does so before a save picker is opened, so a
/// refused export never asks the reader for a destination it cannot fill.
pub(super) fn prepare_vcd(analysis: &crate::state::AnalysisResult) -> Result<PreparedVcd, String> {
    let Some(AnalysisResultPayload::TransientEvents {
        digital_traces,
        real_traces,
    }) = analysis.result_payload.as_ref()
    else {
        return Err(NO_EVENT_EVIDENCE_MESSAGE.to_owned());
    };
    if digital_traces.is_empty() && real_traces.is_empty() {
        return Err(EMPTY_EVENT_EVIDENCE_MESSAGE.to_owned());
    }
    let (digital, real) = core_traces(digital_traces, real_traces)?;
    let document = event_vcd_document(EVENT_SCOPE, &digital, &real, &[])
        .map_err(|error| format!("The event history cannot be dumped exactly: {error}"))?;
    let node_count = document.signals.len();
    let change_count = document
        .signals
        .iter()
        .map(|signal| signal.changes.len())
        .sum();
    let mut bytes = Vec::new();
    write_vcd(&mut bytes, &document)
        .map_err(|error| format!("The Value Change Dump could not be written: {error}"))?;
    Ok(PreparedVcd {
        bytes,
        node_count,
        change_count,
    })
}

pub(super) fn export_vcd(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
    displayed: &ResolvedResultView,
) {
    let prepared = match displayed.primary_analysis(state) {
        Some(analysis) => prepare_vcd(analysis),
        None => Err(NO_ACTIVE_ANALYSIS_MESSAGE.to_owned()),
    };
    let prepared = match prepared {
        Ok(prepared) => prepared,
        Err(message) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::warning(message));
            return;
        }
    };

    let (published_path, export) = match io.show_save_dialog(SaveDialogConfig {
        title: "Export Value Change Dump",
        default_name: "events.vcd",
        filter_name: "Value Change Dump",
        filter_extensions: &["vcd"],
    }) {
        Ok(Some(mut path)) => {
            crate::workbench::workflows::file_actions::ensure_file_extension(&mut path, "vcd");
            let export = io.observe_destination(&path).and_then(|destination| {
                io.write_bytes_file_observed(&destination, &prepared.bytes, MIME_TYPE)
            });
            (path, export)
        }
        Ok(None) => return,
        Err(error) => (std::path::PathBuf::from("events.vcd"), Err(error)),
    };
    match export {
        Ok(()) => {
            note_result_export_success(state, "VCD");
            let detail = format!(
                "{} event nodes, {} changes; four bit states, XSPICE drive strength dropped \
                 because VCD has no strength band",
                prepared.node_count, prepared.change_count
            );
            state.push_user_message(crate::diagnostics::ConsoleMessage::info(
                crate::workbench::workflows::export_workflow::export_completion_message(
                    "VCD",
                    &published_path,
                    Some(detail),
                    io,
                ),
            ));
        }
        Err(error) => {
            note_result_export_failure(state, format!("VCD export failed: {error}"));
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "VCD export failed: {error}"
            )));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        AnalysisResult, AnalysisType, DigitalEventPointEvidence, DigitalEventTraceEvidence,
        RealEventPointEvidence, RealEventTraceEvidence,
    };

    /// `with_result_payload` debug-asserts the payload is valid for the
    /// analysis, which is exactly what the corrupt-code test has to defeat, so
    /// the field is set directly.
    fn analysis(payload: Option<AnalysisResultPayload>) -> AnalysisResult {
        let mut analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN");
        analysis.result_payload = payload;
        analysis
    }

    fn digital(node: &str, points: &[(f64, u8)]) -> DigitalEventTraceEvidence {
        DigitalEventTraceEvidence {
            node_name: node.to_owned(),
            points: points
                .iter()
                .map(|(time_s, value_code)| DigitalEventPointEvidence {
                    time_s: *time_s,
                    value_code: *value_code,
                })
                .collect(),
        }
    }

    fn real(node: &str, points: &[(f64, f64)]) -> RealEventTraceEvidence {
        RealEventTraceEvidence {
            node_name: node.to_owned(),
            points: points
                .iter()
                .map(|(time_s, value)| RealEventPointEvidence {
                    time_s: *time_s,
                    value: *value,
                })
                .collect(),
        }
    }

    fn events(
        digital_traces: Vec<DigitalEventTraceEvidence>,
        real_traces: Vec<RealEventTraceEvidence>,
    ) -> AnalysisResultPayload {
        AnalysisResultPayload::TransientEvents {
            digital_traces,
            real_traces,
        }
    }

    #[test]
    fn a_dump_is_byte_identical_to_the_core_projection_of_the_same_events() {
        let digital_evidence = vec![
            digital("d", &[(0.0, 0), (1e-8, 1), (2e-8, 2), (3e-8, 12)]),
            digital("clk", &[(0.0, 1), (1e-8, 0)]),
        ];
        let real_evidence = vec![real("rnode", &[(0.0, 0.0), (2e-8, 1.5)])];
        let prepared = prepare_vcd(&analysis(Some(events(
            digital_evidence.clone(),
            real_evidence.clone(),
        ))))
        .expect("a transient with event evidence dumps");

        // The same events, projected by the core the way `rspice run -f vcd`
        // projects them. Anything but equality means the two exports of one
        // run disagree.
        let (digital, real) = core_traces(&digital_evidence, &real_evidence).expect("valid codes");
        let document = event_vcd_document(EVENT_SCOPE, &digital, &real, &[]).expect("projects");
        let mut expected = Vec::new();
        write_vcd(&mut expected, &document).expect("writes");
        assert_eq!(prepared.bytes, expected);

        let text = String::from_utf8(prepared.bytes).expect("a dump is ASCII text");
        assert!(text.contains("$scope module events $end"), "{text}");
        assert!(text.contains("$var wire 1 ! d $end"), "{text}");
        assert!(text.contains("$var wire 1 \" clk $end"), "{text}");
        assert!(text.contains("$var real 64 # rnode $end"), "{text}");
        assert!(text.contains("$timescale\n\t10 ns\n$end"), "{text}");
        assert_eq!(prepared.node_count, 3);
        assert_eq!(prepared.change_count, 8);
    }

    #[test]
    fn every_strength_band_collapses_onto_the_bit_its_level_names() {
        // Zero at four strengths, one at four, unknown at four, then high-Z.
        let codes = [0_u8, 3, 6, 9, 1, 4, 7, 10, 2, 5, 8, 11, 12];
        let points = codes
            .iter()
            .enumerate()
            .map(|(index, code)| (index as f64 * 1e-9, *code))
            .collect::<Vec<_>>();
        let prepared = prepare_vcd(&analysis(Some(events(
            vec![digital("n", &points)],
            Vec::new(),
        ))))
        .expect("dumps");
        let text = String::from_utf8(prepared.bytes).expect("ASCII");
        let bits = text
            .lines()
            .filter(|line| line.len() == 2 && line.ends_with('!'))
            .map(|line| &line[..1])
            .collect::<Vec<_>>();
        assert_eq!(
            bits,
            [
                "0", "0", "0", "0", "1", "1", "1", "1", "x", "x", "x", "x", "z"
            ]
        );
    }

    #[test]
    fn the_same_events_publish_the_same_bytes_twice() {
        let payload = events(
            vec![digital("d", &[(0.0, 0), (5e-9, 1)])],
            vec![real("r", &[(0.0, 2.5)])],
        );
        let first = prepare_vcd(&analysis(Some(payload.clone()))).expect("dumps");
        let second = prepare_vcd(&analysis(Some(payload))).expect("dumps");
        assert_eq!(first.bytes, second.bytes);
    }

    #[test]
    fn a_result_with_no_event_history_is_refused_by_what_a_dump_carries() {
        let error = prepare_vcd(&analysis(None)).expect_err("nothing to dump");
        assert_eq!(error, NO_EVENT_EVIDENCE_MESSAGE);
        assert!(error.contains("event timelines"), "{error}");
    }

    #[test]
    fn an_event_history_with_no_node_is_refused_rather_than_written_empty() {
        let error = prepare_vcd(&analysis(Some(events(Vec::new(), Vec::new()))))
            .expect_err("an empty dump is not an export");
        assert_eq!(error, EMPTY_EVENT_EVIDENCE_MESSAGE);
    }

    #[test]
    fn an_unrecognised_event_code_is_refused_by_node_and_time() {
        let error = prepare_vcd(&analysis(Some(events(
            vec![digital("d", &[(0.0, 0), (1e-9, 13)])],
            Vec::new(),
        ))))
        .expect_err("13 is not an event code");
        assert!(error.contains("node 'd'"), "{error}");
        assert!(error.contains("event code 13"), "{error}");
    }

    #[test]
    fn an_event_time_no_timescale_carries_is_refused_rather_than_quantised() {
        let error = prepare_vcd(&analysis(Some(events(
            vec![digital("d", &[(0.0, 0), (1.5e-16, 1)])],
            Vec::new(),
        ))))
        .expect_err("half a femtosecond has no tick");
        assert!(error.contains("cannot be dumped exactly"), "{error}");
    }
}
