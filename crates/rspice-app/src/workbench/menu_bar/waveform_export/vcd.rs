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
//! Both project through [`rspice_core::execution::event_vcd_document`] under one `$scope module
//! events` and both serialise with [`rspice_core::io::write_vcd`], so the timescale rule,
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
use crate::workbench::app_state::AppState;
use crate::workbench::documents::result_document::view_context::ResolvedResultView;
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

use rspice_formats::vcd::encode_result_vcd;

/// VCD is ASCII text. There is no registered media type for it, and claiming
/// one would be an invention rather than a fact about the bytes.
const MIME_TYPE: &str = "text/plain;charset=utf-8";

pub(super) fn export_vcd(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
    displayed: &ResolvedResultView,
) {
    let prepared = match displayed.primary_analysis(state) {
        Some(analysis) => encode_result_vcd(analysis),
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
        AnalysisResult, AnalysisResultPayload, AnalysisType, DigitalEventPointEvidence,
        DigitalEventTraceEvidence,
    };

    #[test]
    fn event_source_vcd_export_reimport_keeps_levels_and_marks_canonical_strengths() {
        use crate::state::{ResultImportFormat, SimulationRunLifecycle};
        use crate::workbench::workflows::result_import_workflow::{
            commit_result_import_draft, stage_imported_result_dataset,
        };
        let points = [0_u8, 3, 6, 9, 1, 4, 7, 10, 2, 5, 8, 11, 12]
            .into_iter()
            .enumerate()
            .map(|(i, code)| (i as f64 * 1e-9, code))
            .collect::<Vec<_>>();
        let mut analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN");
        analysis.result_payload = Some(AnalysisResultPayload::TransientEvents {
            current_impulses: None,
            digital_traces: vec![DigitalEventTraceEvidence {
                node_name: "d".to_owned(),
                points: points
                    .iter()
                    .map(|(time_s, value_code)| DigitalEventPointEvidence {
                        time_s: *time_s,
                        value_code: *value_code,
                    })
                    .collect(),
            }],
            real_traces: Vec::new(),
            digital_buses: Vec::new(),
        });
        let prepared = encode_result_vcd(&analysis).unwrap();
        let mut state = AppState::default();
        let baseline = crate::workbench::lifecycle::project_lifecycle::snapshot(&state).unwrap();
        crate::workbench::lifecycle::project_lifecycle::accept_loaded_project(
            &mut state, baseline, None,
        );
        stage_imported_result_dataset(&mut state, "roundtrip.vcd", &prepared.bytes).unwrap();
        commit_result_import_draft(&mut state).unwrap();
        let project = crate::workbench::lifecycle::project_lifecycle::snapshot(&state).unwrap();
        let text = crate::io::project_io::serialize_project_file(&project).unwrap();
        let restored = crate::io::project_io::load_project_text(&text, None).unwrap();
        assert!(restored.simulation_results_warning.is_none());
        let restored =
            crate::io::simulation_state_from_results(restored.simulation_results).unwrap();
        let run = restored.active_run().unwrap();
        assert_eq!(run.lifecycle, SimulationRunLifecycle::LegacyUnknown);
        assert!(run.prepared_receipt().is_none());
        let result = restored.active_analysis().unwrap();
        assert_eq!(
            result.import_source.as_ref().unwrap().format,
            ResultImportFormat::Vcd
        );
        let Some(AnalysisResultPayload::TransientEvents { digital_traces, .. }) =
            &result.result_payload
        else {
            panic!("retained event payload");
        };
        assert_eq!(
            digital_traces[0]
                .points
                .iter()
                .map(|point| point.value_code)
                .collect::<Vec<_>>(),
            [0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 12]
        );
        assert_eq!(digital_traces[0].points.len(), points.len());
        for (retained, (time, _)) in digital_traces[0].points.iter().zip(&points) {
            assert!((retained.time_s - time).abs() < 1e-23);
        }
    }
}
