//! The schematic SVG export over the session, through an export IO that
//! records what it was asked.

use crate::workbench::app_state::AppState;
use crate::workbench::hardcopy::exports::*;
use crate::workbench::workflows::export_workflow::ExportWorkflowIo;
use crate::workbench::workflows::export_workflow::SaveDialogConfig;
use std::cell::RefCell;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
struct SaveDialogConfigSnapshot {
    title: String,
    default_name: String,
    filter_name: String,
    filter_extensions: Vec<String>,
}

#[derive(Debug)]
struct MockExportWorkflowIo {
    dialog_result: Result<Option<PathBuf>, String>,
    write_result: Result<(), String>,
    saved_paths_are_reopenable: bool,
    configs: RefCell<Vec<SaveDialogConfigSnapshot>>,
    writes: RefCell<Vec<(PathBuf, String)>>,
}

impl MockExportWorkflowIo {
    fn returning_path(path: impl Into<PathBuf>) -> Self {
        Self {
            dialog_result: Ok(Some(path.into())),
            write_result: Ok(()),
            saved_paths_are_reopenable: true,
            configs: RefCell::default(),
            writes: RefCell::default(),
        }
    }

    fn failing_dialog(message: impl Into<String>) -> Self {
        Self {
            dialog_result: Err(message.into()),
            write_result: Ok(()),
            saved_paths_are_reopenable: true,
            configs: RefCell::default(),
            writes: RefCell::default(),
        }
    }

    fn with_write_error(mut self, message: impl Into<String>) -> Self {
        self.write_result = Err(message.into());
        self
    }

    fn download_only(mut self) -> Self {
        self.saved_paths_are_reopenable = false;
        self
    }
}

impl ExportWorkflowIo for MockExportWorkflowIo {
    fn show_save_dialog(&self, config: SaveDialogConfig<'_>) -> Result<Option<PathBuf>, String> {
        self.configs.borrow_mut().push(SaveDialogConfigSnapshot {
            title: config.title.to_owned(),
            default_name: config.default_name.to_owned(),
            filter_name: config.filter_name.to_owned(),
            filter_extensions: config
                .filter_extensions
                .iter()
                .map(|extension| (*extension).to_owned())
                .collect(),
        });
        self.dialog_result.clone()
    }

    fn write_text_file(&self, path: &Path, contents: &str) -> Result<(), String> {
        self.writes
            .borrow_mut()
            .push((path.to_path_buf(), contents.to_owned()));
        self.write_result.clone()
    }

    fn write_waveform_csv(
        &self,
        _dataset: &crate::io::WaveformDataset,
        _path: &Path,
    ) -> Result<(), String> {
        Ok(())
    }

    fn saved_paths_are_reopenable(&self) -> bool {
        self.saved_paths_are_reopenable
    }
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

#[test]
fn svg_export_uses_dialog_defaults_and_writes_svg_file() {
    let mut state = AppState::default();
    state.model.schematic.session.current_file =
        Some(PathBuf::from("designs").join("rc_filter.sch"));
    let io = MockExportWorkflowIo::returning_path(PathBuf::from("exports").join("rc_filter"));

    action_export_svg_with_io(&mut state.model, &io);

    assert_eq!(
        io.configs.borrow().as_slice(),
        &[SaveDialogConfigSnapshot {
            title: "Export SVG".to_owned(),
            default_name: "rc_filter.svg".to_owned(),
            filter_name: "SVG Image".to_owned(),
            filter_extensions: vec!["svg".to_owned()],
        }]
    );
    let writes = io.writes.borrow();
    assert_eq!(writes.len(), 1);
    assert_eq!(writes[0].0, PathBuf::from("exports").join("rc_filter.svg"));
    assert!(writes[0].1.starts_with("<?xml version=\"1.0\""));
    assert!(
        writes[0]
            .1
            .contains("<svg xmlns=\"http://www.w3.org/2000/svg\"")
    );
    assert_eq!(
        last_log_message(&state),
        format!("Exported SVG: {}", writes[0].0.display())
    );
}

/// Inside a project there is no schematic file to take a name from; the
/// export proposes the cell's name, not "schematic.svg" for every cell.
#[test]
fn svg_export_without_a_file_is_named_after_the_cell() {
    let mut state = AppState::default();
    state.model.schematic.session.current_file = None;
    let cell = state
        .model
        .workspace
        .content
        .active_schematic_reference()
        .cell
        .clone();
    assert!(!cell.is_empty() && cell != "schematic");
    let io = MockExportWorkflowIo::returning_path(PathBuf::from("exports").join("out"));

    action_export_svg_with_io(&mut state.model, &io);

    assert_eq!(io.configs.borrow()[0].default_name, format!("{cell}.svg"));
}

#[test]
fn svg_export_reports_save_dialog_errors_without_writing() {
    let mut state = AppState::default();
    let io = MockExportWorkflowIo::failing_dialog("native dialog unavailable");

    action_export_svg_with_io(&mut state.model, &io);

    assert!(io.writes.borrow().is_empty());
    assert_eq!(
        last_log_message(&state),
        "SVG export failed: native dialog unavailable"
    );
}

#[test]
fn svg_export_propagates_injected_publication_failure_without_claiming_success() {
    let mut state = AppState::default();
    let io = MockExportWorkflowIo::returning_path(PathBuf::from("schematic.svg"))
        .with_write_error("disk full");

    action_export_svg_with_io(&mut state.model, &io);

    assert_eq!(io.writes.borrow().len(), 1);
    assert_eq!(last_log_message(&state), "SVG export failed: disk full");
    // An export the user asked for that did not happen is said now.
    state.drain_notices(&egui::Context::default());
    let activity = state.workbench.toasts.activity();
    assert_eq!(activity.len(), 1);
    assert_eq!(activity[0].title(), "SVG export failed");
    assert_eq!(activity[0].message(), "disk full");
}

#[test]
fn svg_export_reports_browser_download_start_without_claiming_file_written() {
    let mut state = AppState::default();
    let io = MockExportWorkflowIo::returning_path(PathBuf::from("schematic.svg")).download_only();

    action_export_svg_with_io(&mut state.model, &io);

    assert_eq!(
        last_log_message(&state),
        "SVG download started: schematic.svg (confirm the browser accepted the download)"
    );
}
