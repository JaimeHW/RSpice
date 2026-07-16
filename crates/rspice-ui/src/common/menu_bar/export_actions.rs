use crate::common::app::AppState;
use crate::common::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

pub(super) fn action_export_svg_with_io(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
) {
    use crate::schematic::export::{SvgExportConfig, export_to_svg_with_symbol_resolver};

    let config = SvgExportConfig::default();
    let resolver = crate::state::SymbolResolver::new(
        &state.library_manager,
        &state.workspace.schematic_buffers,
    );
    let svg_content = export_to_svg_with_symbol_resolver(&state.schematic, &config, &resolver);

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
            crate::common::file_actions::ensure_file_extension(&mut path, "svg");

            let export = io
                .observe_destination(&path)
                .and_then(|destination| io.write_text_file_observed(&destination, &svg_content));
            match export {
                Ok(()) => {
                    state.push_user_message(crate::common::app::ConsoleMessage::info(
                        crate::common::export_workflow::export_completion_message(
                            "SVG", &path, None, io,
                        ),
                    ));
                }
                Err(e) => {
                    state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
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
            state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
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
    if state.schematic.components.is_empty() {
        state.push_user_message(crate::common::app::ConsoleMessage::warning(
            "No circuit to export. Add components first.",
        ));
        return;
    }

    let Some(netlist_content) = build_menu_netlist(state, format) else {
        return;
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
            crate::common::file_actions::ensure_file_extension(&mut path, extension);

            let export = io.observe_destination(&path).and_then(|destination| {
                io.write_text_file_observed(&destination, &netlist_content)
            });
            match export {
                Ok(()) => {
                    state.push_user_message(crate::common::app::ConsoleMessage::info(
                        crate::common::export_workflow::export_completion_message(
                            filter_name,
                            &path,
                            None,
                            io,
                        ),
                    ));
                }
                Err(e) => {
                    state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
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
            state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                "Netlist export failed: {}",
                e
            )));
        }
    }
}

pub(crate) fn action_view_netlist(state: &mut AppState) {
    if state.schematic.components.is_empty() {
        state.push_user_message(crate::common::app::ConsoleMessage::warning(
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

    state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
        "Generated netlist ({} lines):\n{}{}",
        total_lines,
        preview,
        if total_lines > 10 { "\n..." } else { "" }
    )));
}

fn build_menu_netlist(state: &mut AppState, format: crate::io::NetlistFormat) -> Option<String> {
    let hierarchy = crate::simulation::netlist_gen::HierarchySource::from_workspace(
        &state.library_manager,
        &state.workspace.schematic_buffers,
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
        state.push_user_message(crate::common::app::ConsoleMessage::error(
            "Netlist export requires a stable active simulation plan.",
        ));
        return None;
    };
    let Some(plan_payload) = state.workspace.active_plan_data(plan_id) else {
        state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
            "Simulation plan {plan_id} has no plan-owned configuration payload."
        )));
        return None;
    };
    let generation = crate::simulation::netlist_gen::generate_netlist_hierarchical_with_variables(
        &state.schematic,
        &[],
        &hierarchy,
        &plan_payload.design_variables,
        crate::simulation::netlist_gen::DesignVariableNetlistContext {
            active_cell: &state.workspace.active_view,
            analysis_instances: &analysis_instances,
        },
    );

    if !generation.errors.is_empty() {
        for err in generation.errors {
            state.push_user_message(crate::common::app::ConsoleMessage::error(err));
        }
        return None;
    }

    for warning in generation.warnings {
        state.push_user_message(crate::common::app::ConsoleMessage::warning(warning));
    }

    let spice_netlist = generation.netlist;
    Some(match format {
        crate::io::NetlistFormat::Spectre => {
            super::netlist_compat::spice_to_ahdl_compatible_netlist(&spice_netlist)
        }
        _ => spice_netlist,
    })
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
}
