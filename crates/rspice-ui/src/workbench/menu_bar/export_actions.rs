//! Export menu actions.
//!
//! Wires the File ▸ Export items to the export workflows.

use crate::workbench::app_state::AppState;
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

pub(super) fn action_export_svg_with_io(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
) {
    use crate::schematic::export::{
        SvgDesignContext, SvgExportConfig, export_to_svg_with_symbol_resolver_and_context,
    };

    let config = SvgExportConfig::default();
    let resolver = crate::state::SymbolResolver::new(
        &state.library_manager,
        &state.workspace.schematic_buffers,
    );
    let view_path = state.workspace.active_view.display_path();
    let svg_content = export_to_svg_with_symbol_resolver_and_context(
        &state.schematic,
        &config,
        &resolver,
        SvgDesignContext {
            view_path: &view_path,
        },
    );

    let default_name = state
        .schematic
        .current_file
        .as_ref()
        .and_then(|p| p.file_stem())
        .map(|s| format!("{}.svg", s.to_string_lossy()))
        .unwrap_or_else(|| "schematic.svg".to_string());

    match io.show_save_dialog(SaveDialogConfig {
        title: "Export SVG",
        default_name: &default_name,
        filter_name: "SVG Image",
        filter_extensions: &["svg"],
    }) {
        Ok(Some(mut path)) => {
            crate::workbench::workflows::file_actions::ensure_file_extension(&mut path, "svg");

            let export = io
                .observe_destination(&path)
                .and_then(|destination| io.write_text_file_observed(&destination, &svg_content));
            match export {
                Ok(()) => {
                    state.push_user_message(crate::diagnostics::ConsoleMessage::info(
                        crate::workbench::workflows::export_workflow::export_completion_message(
                            "SVG", &path, None, io,
                        ),
                    ));
                }
                Err(e) => {
                    state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                        "SVG export failed: {}",
                        e
                    )));
                }
            }
        }
        Ok(None) => {
            // User cancelled - no message needed
        }
        Err(e) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "SVG export failed: {}",
                e
            )));
        }
    }
}

pub(crate) fn action_export_netlist_with_io(
    state: &mut AppState,
    format: crate::io::NetlistFormat,
    io: &(impl ExportWorkflowIo + ?Sized),
) {
    let netlist_content =
        if state.workbench.workspace == crate::workbench::state::Workspace::Netlist {
            if state.ui.netlist.generated_source.is_empty() {
                state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
                    "No retained generated artifact is available to export.",
                ));
                return;
            }
            if state.ui.netlist.generation_error.is_some()
                || state.ui.netlist.generated_input_digest
                    != state.ui.netlist.current_generation_input_digest
            {
                state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
                    "Generated export is blocked because the retained artifact is stale.",
                ));
                return;
            }
            match format {
                crate::io::NetlistFormat::Spectre => {
                    super::netlist_compat::spice_to_ahdl_compatible_netlist(
                        &state.ui.netlist.generated_source,
                    )
                }
                _ => state.ui.netlist.generated_source.clone(),
            }
        } else {
            if state.schematic.components.is_empty() {
                state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
                    "No circuit to export. Add components first.",
                ));
                return;
            }
            let Some(content) = build_menu_netlist(state, format) else {
                return;
            };
            content
        };

    let default_name = state
        .schematic
        .current_file
        .as_ref()
        .and_then(|p| p.file_stem())
        .map(|s| format!("{}.{}", s.to_string_lossy(), format.extension()))
        .unwrap_or_else(|| format!("circuit.{}", format.extension()));

    let filter_name = match format {
        crate::io::NetlistFormat::Spectre => "Spectre Netlist",
        crate::io::NetlistFormat::Spice => "SPICE Netlist",
        crate::io::NetlistFormat::Hspice => "HSPICE Netlist",
        crate::io::NetlistFormat::Xyce => "Xyce Netlist",
    };

    let extension = format.extension();

    match io.show_save_dialog(SaveDialogConfig {
        title: "Export Netlist",
        default_name: &default_name,
        filter_name,
        filter_extensions: &[extension],
    }) {
        Ok(Some(mut path)) => {
            crate::workbench::workflows::file_actions::ensure_file_extension(&mut path, extension);

            let export = io.observe_destination(&path).and_then(|destination| {
                io.write_text_file_observed(&destination, &netlist_content)
            });
            match export {
                Ok(()) => {
                    state.push_user_message(crate::diagnostics::ConsoleMessage::info(
                        crate::workbench::workflows::export_workflow::export_completion_message(
                            filter_name,
                            &path,
                            None,
                            io,
                        ),
                    ));
                }
                Err(e) => {
                    state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                        "Netlist export failed: {}",
                        e
                    )));
                }
            }
        }
        Ok(None) => {
            // User cancelled - no message needed
        }
        Err(e) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "Netlist export failed: {}",
                e
            )));
        }
    }
}

pub(crate) fn action_export_generated_netlist_with_options(
    state: &mut AppState,
    format: crate::io::NetlistFormat,
    bundle_dependencies: bool,
    include_source_map: bool,
    io: &(impl ExportWorkflowIo + ?Sized),
) -> bool {
    let Some(document) = state.ui.netlist.generated_document.as_ref() else {
        state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
            "No retained generated artifact is available to export.",
        ));
        return false;
    };
    let artifact = document.generated_artifact().clone();
    if state.ui.netlist.generation_error.is_some()
        || state.ui.netlist.generated_input_digest
            != state.ui.netlist.current_generation_input_digest
        || state.ui.netlist.generated_input_digest != Some(artifact.provenance().input().digest())
    {
        state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
            "Generated export is blocked because the retained artifact is stale or inconsistent with its authenticated inputs.",
        ));
        return false;
    }
    if include_source_map && !bundle_dependencies {
        state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
            "Source-map export requires a self-contained bundle.",
        ));
        return false;
    }
    if include_source_map && format != crate::io::NetlistFormat::Spice {
        state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
            "Generated source maps identify exact SPICE lines and cannot be attached to a translated dialect.",
        ));
        return false;
    }
    if !bundle_dependencies && !artifact.dependencies().is_empty() {
        state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
            "Generated decks with project or external source dependencies must be exported as a self-contained bundle.",
        ));
        return false;
    }
    if !bundle_dependencies {
        return publish_generated_source(state, &artifact, format, io);
    }

    let bundle =
        match crate::io::build_generated_bundle(&artifact, format, include_source_map, |source| {
            generated_export_source(source, format)
        }) {
            Ok(bundle) => bundle,
            Err(error) => {
                state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                    "Generated bundle export failed: {error}"
                )));
                return false;
            }
        };
    let default_name = format!(
        "generated-{}.zip",
        &artifact.content_digest().to_string()[..12]
    );
    let picked = io.show_save_dialog(SaveDialogConfig {
        title: "Export Generated Netlist Bundle",
        default_name: &default_name,
        filter_name: "RSpice Netlist Bundle",
        filter_extensions: &["zip"],
    });
    let Some(mut path) = (match picked {
        Ok(path) => path,
        Err(error) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "Generated bundle export failed: {error}"
            )));
            return false;
        }
    }) else {
        return false;
    };
    crate::workbench::workflows::file_actions::ensure_file_extension(&mut path, "zip");
    let result = io.observe_destination(&path).and_then(|destination| {
        io.write_bytes_file_observed(&destination, &bundle, "application/zip")
    });
    match result {
        Ok(()) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::info(
                crate::workbench::workflows::export_workflow::export_completion_message(
                    "Generated netlist bundle",
                    &path,
                    None,
                    io,
                ),
            ));
            true
        }
        Err(error) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "Generated bundle export failed: {error}"
            )));
            false
        }
    }
}

fn publish_generated_source(
    state: &mut AppState,
    artifact: &crate::state::GeneratedArtifact,
    format: crate::io::NetlistFormat,
    io: &(impl ExportWorkflowIo + ?Sized),
) -> bool {
    let source = generated_export_source(artifact.source(), format);
    let extension = format.extension();
    let filter_name = match format {
        crate::io::NetlistFormat::Spectre => "Spectre Netlist",
        crate::io::NetlistFormat::Spice => "SPICE Netlist",
        crate::io::NetlistFormat::Hspice => "HSPICE Netlist",
        crate::io::NetlistFormat::Xyce => "Xyce Netlist",
    };
    let default_name = format!(
        "generated-{}.{}",
        &artifact.content_digest().to_string()[..12],
        extension
    );
    let Some(mut path) = (match io.show_save_dialog(SaveDialogConfig {
        title: "Export Generated Netlist",
        default_name: &default_name,
        filter_name,
        filter_extensions: &[extension],
    }) {
        Ok(path) => path,
        Err(error) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "Generated netlist export failed: {error}"
            )));
            return false;
        }
    }) else {
        return false;
    };
    crate::workbench::workflows::file_actions::ensure_file_extension(&mut path, extension);
    let result = io
        .observe_destination(&path)
        .and_then(|destination| io.write_text_file_observed(&destination, &source));
    match result {
        Ok(()) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::info(
                crate::workbench::workflows::export_workflow::export_completion_message(
                    filter_name,
                    &path,
                    None,
                    io,
                ),
            ));
            true
        }
        Err(error) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "Generated netlist export failed: {error}"
            )));
            false
        }
    }
}

fn generated_export_source(source: &str, format: crate::io::NetlistFormat) -> String {
    match format {
        crate::io::NetlistFormat::Spectre => {
            super::netlist_compat::spice_to_ahdl_compatible_netlist(source)
        }
        crate::io::NetlistFormat::Spice
        | crate::io::NetlistFormat::Hspice
        | crate::io::NetlistFormat::Xyce => source.to_owned(),
    }
}

pub(crate) fn action_view_netlist(state: &mut AppState) {
    if state.schematic.components.is_empty() {
        state.push_user_message(crate::diagnostics::ConsoleMessage::warning(
            "No circuit to generate netlist. Add components first.",
        ));
        return;
    }

    let Some(netlist_content) = build_menu_netlist(state, crate::io::NetlistFormat::Spice) else {
        return;
    };

    state.simulation.netlist_content = netlist_content.clone();

    let preview_lines: Vec<&str> = netlist_content.lines().take(10).collect();
    let preview = preview_lines.join("\n");
    let total_lines = netlist_content.lines().count();

    state.push_user_message(crate::diagnostics::ConsoleMessage::info(format!(
        "Generated netlist ({} lines):\n{}{}",
        total_lines,
        preview,
        if total_lines > 10 { "\n..." } else { "" }
    )));
}

pub(crate) fn build_menu_netlist(
    state: &mut AppState,
    format: crate::io::NetlistFormat,
) -> Option<String> {
    let execution_projection = match state.workspace.configuration_execution_projection(
        &state.library_manager,
        &state.workspace.active_view,
        &state.schematic,
    ) {
        Ok(projection) => projection,
        Err(error) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(error.to_string()));
            return None;
        }
    };
    let root_reference = execution_projection.root().clone();
    let root_schematic = execution_projection
        .root_schematic()
        .expect("a successful execution projection has a materialized root");
    let hierarchy = crate::simulation::netlist_gen::HierarchySource::from_execution_projection(
        &state.library_manager,
        &execution_projection,
    );
    let analysis_instances = state
        .sim_setup
        .analysis_plan
        .as_ref()
        .into_iter()
        .flat_map(|plan| plan.instances())
        .filter(|instance| instance.enabled())
        .map(crate::simulation::plan::AnalysisInstance::id)
        .collect::<Vec<_>>();
    let Some(plan_id) = state
        .sim_setup
        .analysis_plan
        .as_ref()
        .map(crate::simulation::plan::SimulationPlan::id)
    else {
        state.push_user_message(crate::diagnostics::ConsoleMessage::error(
            "Netlist export requires a stable active simulation plan.",
        ));
        return None;
    };
    let Some(plan_payload) = state.workspace.active_plan_data(plan_id) else {
        state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
            "Simulation plan {plan_id} has no plan-owned configuration payload."
        )));
        return None;
    };
    let generation = crate::simulation::netlist_gen::generate_netlist_hierarchical_with_variables(
        root_schematic,
        &[],
        &hierarchy,
        &plan_payload.design_variables,
        crate::simulation::netlist_gen::DesignVariableNetlistContext {
            active_cell: &root_reference,
            analysis_instances: &analysis_instances,
        },
    );

    if !generation.errors.is_empty() {
        for err in generation.errors {
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(err));
        }
        return None;
    }

    for warning in generation.warnings {
        state.push_user_message(crate::diagnostics::ConsoleMessage::warning(warning));
    }

    let spice_netlist = state
        .workspace
        .bind_generated_netlist_provenance(generation.netlist);
    Some(match format {
        crate::io::NetlistFormat::Spectre => {
            super::netlist_compat::spice_to_ahdl_compatible_netlist(&spice_netlist)
        }
        _ => spice_netlist,
    })
}

/// Export the sealed publication snapshot: the exact canonical bytes the
/// publish pipeline consumes, built from the current sheets, plot panes,
/// active-run results, and effective deck.
pub(super) fn action_export_publication_snapshot_with_io(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
) {
    let stem = state
        .schematic
        .current_file
        .as_ref()
        .and_then(|p| p.file_stem())
        .map(|s| s.to_string_lossy().into_owned());
    let draft = crate::workbench::publication_snapshot::PublicationDraft {
        title: stem
            .clone()
            .unwrap_or_else(|| "Untitled project".to_string()),
        description: String::new(),
        author_display: "Local export".to_string(),
        created_utc: crate::workbench::publication_snapshot::publication_timestamp_utc(),
        license: rspice_publication_contract::ContentLicense::AllRightsReserved,
        ..Default::default()
    };
    let built = crate::workbench::publication_snapshot::build_publication_snapshot(state, &draft)
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
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "Publication snapshot export failed: {error}"
            )));
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
            crate::workbench::workflows::file_actions::ensure_file_extension(
                &mut path,
                "rspicepub",
            );
            let export = io
                .observe_destination(&path)
                .and_then(|destination| io.write_text_file_observed(&destination, &content));
            match export {
                Ok(()) => {
                    state.push_user_message(crate::diagnostics::ConsoleMessage::info(
                        crate::workbench::workflows::export_workflow::export_completion_message(
                            "Publication snapshot",
                            &path,
                            None,
                            io,
                        ),
                    ));
                }
                Err(error) => {
                    state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                        "Publication snapshot export failed: {error}"
                    )));
                }
            }
        }
        Ok(None) => {
            // User cancelled - no message needed
        }
        Err(error) => {
            state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
                "Publication snapshot export failed: {error}"
            )));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        state.schematic.current_file = Some(PathBuf::from("designs").join("rc_filter.sch"));
        let io = MockExportWorkflowIo::returning_path(PathBuf::from("exports").join("rc_filter"));

        action_export_svg_with_io(&mut state, &io);

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

    #[test]
    fn svg_export_reports_save_dialog_errors_without_writing() {
        let mut state = AppState::default();
        let io = MockExportWorkflowIo::failing_dialog("native dialog unavailable");

        action_export_svg_with_io(&mut state, &io);

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

        action_export_svg_with_io(&mut state, &io);

        assert_eq!(io.writes.borrow().len(), 1);
        assert_eq!(last_log_message(&state), "SVG export failed: disk full");
    }

    #[test]
    fn svg_export_reports_browser_download_start_without_claiming_file_written() {
        let mut state = AppState::default();
        let io =
            MockExportWorkflowIo::returning_path(PathBuf::from("schematic.svg")).download_only();

        action_export_svg_with_io(&mut state, &io);

        assert_eq!(
            last_log_message(&state),
            "SVG download started: schematic.svg (confirm the browser accepted the download)"
        );
    }

    #[test]
    fn non_spectre_generated_exports_preserve_exact_source_bytes() {
        let source = "title Î¼\r\nR1 out 0 1k\r\n.end\r\n";
        for format in [
            crate::io::NetlistFormat::Spice,
            crate::io::NetlistFormat::Hspice,
            crate::io::NetlistFormat::Xyce,
        ] {
            assert_eq!(
                generated_export_source(source, format).as_bytes(),
                source.as_bytes()
            );
        }
    }
}
