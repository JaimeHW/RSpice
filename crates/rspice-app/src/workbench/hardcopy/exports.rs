//! The hardcopy feature's file exports: the open schematic as SVG, and the
//! sealed publication snapshot.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::Cx;
use crate::workbench::hardcopy_adapters::publication_snapshot::{
    PublicationReads, publication_timestamp_utc,
};
use crate::workbench::hardcopy_adapters::sources::SourceReads;
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

use super::HardcopyFeature;

/// Export the open schematic as SVG, through the export IO.
pub(super) fn action_export_svg_with_io(model: &mut Model, io: &(impl ExportWorkflowIo + ?Sized)) {
    use rspice_schematic_editor::export::{
        SvgDesignContext, SvgExportConfig, export_to_svg_with_symbol_resolver_and_context,
    };

    let config = SvgExportConfig::default();
    let resolver = rspice_design::symbol_resolver::SymbolResolver::new(
        model.library_manager.catalog(),
        &model.workspace.content.schematic_buffers,
    );
    let view_path = model.workspace.content.active_view.display_path();
    let sheet_key = model.workspace.content.active_schematic_reference().key();
    let svg_content = export_to_svg_with_symbol_resolver_and_context(
        model.schematic.document(),
        &config,
        &resolver,
        SvgDesignContext {
            view_path: &view_path,
            // What the canvas shows: the active sheet.
            sheet_catalog: model
                .workspace
                .content
                .design_management
                .sheet_catalog(&sheet_key),
        },
    );

    let default_name = format!("{}.svg", model.export_file_stem("schematic"));

    match io.show_save_dialog(SaveDialogConfig {
        title: "Export SVG",
        default_name: &default_name,
        filter_name: "SVG Image",
        filter_extensions: &["svg"],
    }) {
        Ok(Some(mut path)) => {
            crate::io::file_exchange::ensure_file_extension(&mut path, "svg");

            let export = io
                .observe_destination(&path)
                .and_then(|destination| io.write_text_file_observed(&destination, &svg_content));
            match export {
                Ok(()) => {
                    model.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        crate::workbench::workflows::export_workflow::export_completion_message(
                            "SVG", &path, None, io,
                        ),
                    );
                }
                Err(e) => model.report_failure("SVG export failed", e.to_string()),
            }
        }
        Ok(None) => {
            // User cancelled - no message needed
        }
        Err(e) => model.report_failure("SVG export failed", e.to_string()),
    }
}

/// Export the sealed publication snapshot: the exact canonical bytes the
/// publish pipeline consumes, built from the current sheets, plot panes,
/// active-run results, and effective deck.
pub(super) fn action_export_publication_snapshot_with_io(cx: &mut Cx<'_, HardcopyFeature>) {
    let io = cx.handles.export_io;
    let stem = cx
        .model
        .schematic
        .session
        .current_file
        .as_ref()
        .and_then(|p| p.file_stem())
        .map(|s| s.to_string_lossy().into_owned());
    let draft = crate::workbench::framework::feature::PublicationDraft {
        title: stem
            .clone()
            .unwrap_or_else(|| "Untitled project".to_string()),
        description: String::new(),
        author_display: "Local export".to_string(),
        created_utc: publication_timestamp_utc(),
        license: rspice_publication_contract::ContentLicense::AllRightsReserved,
        ..Default::default()
    };
    let built =
        crate::workbench::hardcopy_adapters::publication_snapshot::build_publication_snapshot(
            publication_reads(cx),
            &draft,
        )
        .map_err(|error| error.to_string())
        .and_then(|snapshot| {
            snapshot
                .canonical_bytes()
                .map_err(|error| error.to_string())
        })
        .and_then(|bytes| String::from_utf8(bytes).map_err(|error| error.to_string()));
    let content = match built {
        Ok(content) => content,
        Err(error) => {
            cx.model
                .report_failure("Publication snapshot export failed", error);
            return;
        }
    };
    let default_name = stem
        .map(|s| format!("{s}.rspicepub"))
        .unwrap_or_else(|| "publication.rspicepub".to_string());

    match io.show_save_dialog(SaveDialogConfig {
        title: "Export publication snapshot",
        default_name: &default_name,
        filter_name: "RSpice publication snapshot",
        filter_extensions: &["rspicepub"],
    }) {
        Ok(Some(mut path)) => {
            crate::io::file_exchange::ensure_file_extension(&mut path, "rspicepub");
            let export = io
                .observe_destination(&path)
                .and_then(|destination| io.write_text_file_observed(&destination, &content));
            match export {
                Ok(()) => {
                    cx.model.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        crate::workbench::workflows::export_workflow::export_completion_message(
                            "publication snapshot",
                            &path,
                            None,
                            io,
                        ),
                    );
                }
                Err(error) => {
                    cx.model
                        .report_failure("Publication snapshot export failed", error.to_string());
                }
            }
        }
        Ok(None) => {
            // User cancelled - no message needed
        }
        Err(error) => {
            cx.model
                .report_failure("Publication snapshot export failed", error.to_string());
        }
    }
}

/// What the snapshot is built from, read from the feature's context.
fn publication_reads<'a>(cx: &'a Cx<'_, HardcopyFeature>) -> PublicationReads<'a> {
    PublicationReads {
        sources: SourceReads {
            model: cx.model,
            workbench: cx.wb,
            report_authoring: &cx.state.report_authoring,
            results: cx.peers.results(),
        },
        runs: cx.runs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::app_state::AppState;

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
        fn show_save_dialog(
            &self,
            config: SaveDialogConfig<'_>,
        ) -> Result<Option<PathBuf>, String> {
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
        let io =
            MockExportWorkflowIo::returning_path(PathBuf::from("schematic.svg")).download_only();

        action_export_svg_with_io(&mut state.model, &io);

        assert_eq!(
            last_log_message(&state),
            "SVG download started: schematic.svg (confirm the browser accepted the download)"
        );
    }
}
