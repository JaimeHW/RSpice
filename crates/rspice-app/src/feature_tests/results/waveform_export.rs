//! Result exports observed through a whole application: what the status
//! bar, the Console and the toasts say about an export, and an exported VCD
//! carried through a project reload.

use std::cell::RefCell;
use std::path::{Path, PathBuf};

use rspice_formats::vcd::encode_result_vcd;

use crate::state::{
    AnalysisResult, AnalysisResultPayload, AnalysisType, DigitalEventPointEvidence,
    DigitalEventTraceEvidence, RetainedTrace, SimulationRun,
};
use crate::workbench::app_state::AppState;
use crate::workbench::menu_bar::action_export_csv_with_io;
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

#[derive(Debug)]
struct MockExportWorkflowIo {
    datasets: RefCell<Vec<crate::io::WaveformDataset>>,
    paths: RefCell<Vec<PathBuf>>,
    text_files: RefCell<Vec<(PathBuf, String)>>,
    byte_files: RefCell<Vec<(PathBuf, Vec<u8>, String)>>,
    dialog_titles: RefCell<Vec<String>>,
    saved_paths_are_reopenable: bool,
    /// Why the save dialog could not open, when it cannot.
    dialog_error: Option<String>,
}

impl Default for MockExportWorkflowIo {
    fn default() -> Self {
        Self {
            datasets: RefCell::default(),
            paths: RefCell::default(),
            text_files: RefCell::default(),
            byte_files: RefCell::default(),
            dialog_titles: RefCell::default(),
            saved_paths_are_reopenable: true,
            dialog_error: None,
        }
    }
}

impl ExportWorkflowIo for MockExportWorkflowIo {
    fn show_save_dialog(&self, config: SaveDialogConfig<'_>) -> Result<Option<PathBuf>, String> {
        self.dialog_titles
            .borrow_mut()
            .push(config.title.to_owned());
        if let Some(error) = &self.dialog_error {
            return Err(error.clone());
        }
        Ok(Some(PathBuf::from(config.default_name)))
    }

    fn write_text_file(&self, path: &Path, contents: &str) -> Result<(), String> {
        self.text_files
            .borrow_mut()
            .push((path.to_path_buf(), contents.to_owned()));
        Ok(())
    }

    fn write_waveform_csv(
        &self,
        dataset: &crate::io::WaveformDataset,
        path: &Path,
    ) -> Result<(), String> {
        self.datasets.borrow_mut().push(dataset.clone());
        self.paths.borrow_mut().push(path.to_path_buf());
        Ok(())
    }

    fn write_bytes_file_observed(
        &self,
        destination: &crate::workbench::workflows::export_workflow::ObservedExportDestination,
        contents: &[u8],
        mime_type: &str,
    ) -> Result<(), String> {
        self.byte_files.borrow_mut().push((
            destination.path().to_path_buf(),
            contents.to_vec(),
            mime_type.to_owned(),
        ));
        Ok(())
    }

    fn saved_paths_are_reopenable(&self) -> bool {
        self.saved_paths_are_reopenable
    }
}

fn waveform(name: &str, x: Vec<f64>, y: Vec<f64>) -> RetainedTrace {
    RetainedTrace::new(name.to_owned(), x, y, "#4f81bd")
}

fn last_log_message(state: &AppState) -> String {
    state
        .model
        .log_buffer
        .entries()
        .last()
        .expect("a user-facing log line is emitted")
        .message
        .clone()
}

fn state_with_typed_result(analysis: AnalysisResult) -> AppState {
    let viewer = match analysis.analysis_type {
        AnalysisType::DcOp => crate::workbench::ResultViewer::Op,
        AnalysisType::PoleZero => crate::workbench::ResultViewer::PoleZero,
        AnalysisType::Sensitivity => crate::workbench::ResultViewer::Contribution,
        AnalysisType::Tf => crate::workbench::ResultViewer::TransferFunction,
        AnalysisType::Soa => crate::workbench::ResultViewer::Soa,
        AnalysisType::Optimization => crate::workbench::ResultViewer::Optimization,
        kind if kind.is_time_domain() => crate::workbench::ResultViewer::Waves,
        kind if kind.is_bode_response() || kind.is_raw_frequency_curve() => {
            crate::workbench::ResultViewer::Bode
        }
        _ => crate::workbench::ResultViewer::Table,
    };
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    let mut state = AppState::default();
    state.model.simulation.retained.runs = vec![run].into();
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);
    activate_result_document(&mut state, viewer);
    state
}

fn activate_result_document(state: &mut AppState, viewer: crate::workbench::ResultViewer) {
    let dataset_id = state
        .model
        .simulation
        .active_run()
        .expect("test retains an active result run")
        .dataset_id;
    state.workbench.documents.activate(
        crate::workbench::state::WorkspaceDocumentId::ResultDataset(dataset_id),
    );
    state.results.results.session.viewer = viewer;
}

/// The file the reader just chose is named on the status bar, and the Console
/// keeps the same line. Nothing is toasted for an export that worked.
#[test]
fn a_result_export_names_its_file_on_the_status_bar() {
    let mut state = state_with_typed_result(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![waveform(
            "V(out)",
            vec![0.0, 1.0],
            vec![0.0, 2.0],
        )]),
    );
    state.results.results.session.viewer = crate::workbench::ResultViewer::Waves;
    let io = MockExportWorkflowIo::default();

    action_export_csv_with_io(&mut state.results_parts(), &io);
    state.drain_notices(&egui::Context::default());

    let line = last_log_message(&state);
    assert!(line.contains("waveforms.csv"), "{line}");
    assert_eq!(state.prompt_text(), Some(line.as_str()));
    assert!(state.workbench.toasts.activity().is_empty());
}

/// An export the user asked for that did not happen is said now, with the
/// way to its Console line; the reader is not left to find it under the bell.
#[test]
fn a_result_export_that_fails_is_toasted() {
    let mut state = state_with_typed_result(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![waveform(
            "V(out)",
            vec![0.0, 1.0],
            vec![0.0, 2.0],
        )]),
    );
    state.results.results.session.viewer = crate::workbench::ResultViewer::Waves;
    let io = MockExportWorkflowIo {
        dialog_error: Some("the file picker is unavailable".to_owned()),
        ..MockExportWorkflowIo::default()
    };

    action_export_csv_with_io(&mut state.results_parts(), &io);
    state.drain_notices(&egui::Context::default());

    assert_eq!(
        last_log_message(&state),
        "CSV export failed: the file picker is unavailable"
    );
    let activity = state.workbench.toasts.activity();
    assert_eq!(activity.len(), 1);
    assert_eq!(activity[0].title(), "CSV export failed");
    assert_eq!(activity[0].kind(), crate::ui::widgets::ToastKind::Error);
}

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
        voltage_impulses: None,
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
    crate::workbench::app_state::services_access::accept_loaded_project(&mut state, baseline, None);
    stage_imported_result_dataset(&mut state.results_parts(), "roundtrip.vcd", &prepared.bytes)
        .unwrap();
    commit_result_import_draft(&mut state.results_parts()).unwrap();
    let project = crate::workbench::lifecycle::project_lifecycle::snapshot(&state).unwrap();
    let text = crate::io::project_io::serialize_project_file(&project).unwrap();
    let restored = crate::io::project_io::load_project_text(&text, None).unwrap();
    assert!(restored.file.simulation_results_warning.is_none());
    let restored =
        crate::io::simulation_state_from_results(restored.file.simulation_results).unwrap();
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
