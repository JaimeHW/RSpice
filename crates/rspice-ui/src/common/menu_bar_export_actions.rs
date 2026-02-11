use crate::common::app::AppState;
use crate::common::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

pub(super) fn action_export_svg(state: &mut AppState) {
    let io = crate::common::export_workflow::NativeExportWorkflowIo;
    action_export_svg_with_io(state, &io);
}

pub(super) fn action_export_svg_with_io(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
) {
    use crate::schematic::export::{export_to_svg, SvgExportConfig};

    let config = SvgExportConfig::default();
    let svg_content = export_to_svg(&state.schematic, &config);

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
        Some(mut path) => {
            super::menu_bar_file_actions::ensure_file_extension(&mut path, "svg");

            match io.write_text_file(&path, &svg_content) {
                Ok(()) => {
                    state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
                        "Exported SVG: {}",
                        path.display()
                    )));
                }
                Err(e) => {
                    state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                        "SVG export failed: {}",
                        e
                    )));
                }
            }
        }
        None => {
            // User cancelled - no message needed
        }
    }
}

pub(super) fn action_export_netlist(state: &mut AppState, format: crate::io::NetlistFormat) {
    let io = crate::common::export_workflow::NativeExportWorkflowIo;
    action_export_netlist_with_io(state, format, &io);
}

pub(super) fn action_export_netlist_with_io(
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
        Some(mut path) => {
            super::menu_bar_file_actions::ensure_file_extension(&mut path, extension);

            match io.write_text_file(&path, &netlist_content) {
                Ok(()) => {
                    state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
                        "Exported {}: {}",
                        filter_name,
                        path.display()
                    )));
                }
                Err(e) => {
                    state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                        "Netlist export failed: {}",
                        e
                    )));
                }
            }
        }
        None => {
            // User cancelled - no message needed
        }
    }
}

pub(super) fn action_view_netlist(state: &mut AppState) {
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
    let generation =
        crate::simulation::netlist_gen::generate_netlist_with_analysis(&state.schematic, &[]);

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
            super::menu_bar_netlist_compat::spice_to_spectre_compatible_netlist(&spice_netlist)
        }
        _ => spice_netlist,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::collections::VecDeque;
    use std::path::{Path, PathBuf};
    use std::rc::Rc;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct SaveDialogConfigSnapshot {
        title: String,
        default_name: String,
        filter_name: String,
        filter_extensions: Vec<String>,
    }

    #[derive(Default)]
    struct MockExportWorkflowIo {
        save_dialog_results: RefCell<VecDeque<Option<PathBuf>>>,
        write_text_results: RefCell<VecDeque<Result<(), String>>>,
        save_dialog_calls: Cell<usize>,
        write_text_calls: Cell<usize>,
        last_save_dialog_config: RefCell<Option<SaveDialogConfigSnapshot>>,
        last_write_text_path: RefCell<Option<PathBuf>>,
        last_write_text_contents: RefCell<Option<String>>,
    }

    impl MockExportWorkflowIo {
        fn push_save_dialog_result(&self, result: Option<PathBuf>) {
            self.save_dialog_results.borrow_mut().push_back(result);
        }

        fn push_write_text_result(&self, result: Result<(), String>) {
            self.write_text_results.borrow_mut().push_back(result);
        }

        fn save_dialog_calls(&self) -> usize {
            self.save_dialog_calls.get()
        }

        fn write_text_calls(&self) -> usize {
            self.write_text_calls.get()
        }

        fn last_save_dialog_config(&self) -> Option<SaveDialogConfigSnapshot> {
            self.last_save_dialog_config.borrow().clone()
        }

        fn last_write_text_path(&self) -> Option<PathBuf> {
            self.last_write_text_path.borrow().clone()
        }

        fn last_write_text_contents(&self) -> Option<String> {
            self.last_write_text_contents.borrow().clone()
        }
    }

    impl crate::common::export_workflow::ExportWorkflowIo for Rc<MockExportWorkflowIo> {
        fn show_save_dialog(
            &self,
            config: crate::common::export_workflow::SaveDialogConfig<'_>,
        ) -> Option<PathBuf> {
            self.save_dialog_calls
                .set(self.save_dialog_calls.get().saturating_add(1));
            *self.last_save_dialog_config.borrow_mut() = Some(SaveDialogConfigSnapshot {
                title: config.title.to_string(),
                default_name: config.default_name.to_string(),
                filter_name: config.filter_name.to_string(),
                filter_extensions: config
                    .filter_extensions
                    .iter()
                    .map(|ext| (*ext).to_string())
                    .collect(),
            });
            self.save_dialog_results
                .borrow_mut()
                .pop_front()
                .expect("test must provide show_save_dialog result")
        }

        fn write_text_file(&self, path: &Path, contents: &str) -> Result<(), String> {
            self.write_text_calls
                .set(self.write_text_calls.get().saturating_add(1));
            *self.last_write_text_path.borrow_mut() = Some(path.to_path_buf());
            *self.last_write_text_contents.borrow_mut() = Some(contents.to_string());
            self.write_text_results
                .borrow_mut()
                .pop_front()
                .expect("test must provide write_text_file result")
        }

        fn write_waveform_csv(
            &self,
            _dataset: &crate::io::WaveformDataset,
            _path: &Path,
        ) -> Result<(), String> {
            Err("unexpected write_waveform_csv call in menu_bar_export_actions tests".to_string())
        }
    }

    fn state_with_resistor_component() -> AppState {
        let mut state = AppState::default();
        use crate::state::{Component, ComponentType, Point};
        let comp = Component::new(1, ComponentType::Resistor, Point::new(100, 100))
            .with_name_value("R1", "1k");
        state.schematic.components.push(comp);
        state
    }

    #[test]
    fn test_action_view_netlist_uses_generated_schematic_netlist() {
        let mut state = state_with_resistor_component();

        action_view_netlist(&mut state);

        assert!(
            state.simulation.netlist_content.contains("R1"),
            "generated netlist should include the real component instance"
        );
        assert!(
            !state.simulation.netlist_content.contains("N1 N2"),
            "legacy placeholder node names must not appear"
        );
    }

    #[test]
    fn test_action_view_netlist_warns_when_schematic_is_empty() {
        let mut state = AppState::default();

        action_view_netlist(&mut state);

        assert!(
            state.simulation.netlist_content.is_empty(),
            "netlist preview should remain empty for an empty schematic"
        );
        assert!(
            state
                .console_messages
                .iter()
                .any(|message| message.message.contains("No circuit to generate netlist")),
            "an actionable warning should be emitted for empty schematics"
        );
    }

    #[test]
    fn test_action_export_svg_with_io_writes_svg_file_and_logs_success() {
        let io = Rc::new(MockExportWorkflowIo::default());
        io.push_save_dialog_result(Some(PathBuf::from("exports/schematic_output")));
        io.push_write_text_result(Ok(()));

        let mut state = state_with_resistor_component();
        state.schematic.current_file = Some(PathBuf::from("designs/opamp.rsch"));

        action_export_svg_with_io(&mut state, &io);

        assert_eq!(io.save_dialog_calls(), 1);
        assert_eq!(io.write_text_calls(), 1);
        assert_eq!(
            io.last_save_dialog_config(),
            Some(SaveDialogConfigSnapshot {
                title: "Export SVG".to_string(),
                default_name: "opamp.svg".to_string(),
                filter_name: "SVG Image".to_string(),
                filter_extensions: vec!["svg".to_string()],
            })
        );
        assert_eq!(
            io.last_write_text_path(),
            Some(PathBuf::from("exports/schematic_output.svg"))
        );
        let svg = io
            .last_write_text_contents()
            .expect("svg content should be captured");
        assert!(svg.contains("<svg"));
        assert!(
            state
                .console_messages
                .iter()
                .any(|message| message.message.contains("Exported SVG:")),
            "success export should log an informational message"
        );
    }

    #[test]
    fn test_action_export_svg_with_io_cancelled_dialog_skips_write_and_messages() {
        let io = Rc::new(MockExportWorkflowIo::default());
        io.push_save_dialog_result(None);

        let mut state = state_with_resistor_component();
        let baseline_messages = state.console_messages.len();

        action_export_svg_with_io(&mut state, &io);

        assert_eq!(io.save_dialog_calls(), 1);
        assert_eq!(io.write_text_calls(), 0);
        assert_eq!(state.console_messages.len(), baseline_messages);
    }

    #[test]
    fn test_action_export_svg_with_io_write_error_logs_failure() {
        let io = Rc::new(MockExportWorkflowIo::default());
        io.push_save_dialog_result(Some(PathBuf::from("broken.svg")));
        io.push_write_text_result(Err("disk full".to_string()));

        let mut state = state_with_resistor_component();

        action_export_svg_with_io(&mut state, &io);

        assert_eq!(io.write_text_calls(), 1);
        assert!(
            state
                .console_messages
                .iter()
                .any(|message| message.message.contains("SVG export failed: disk full")),
            "write errors should surface to the user log"
        );
    }

    #[test]
    fn test_action_export_netlist_with_io_empty_schematic_warns_and_skips_dialog() {
        let io = Rc::new(MockExportWorkflowIo::default());
        let mut state = AppState::default();

        action_export_netlist_with_io(&mut state, crate::io::NetlistFormat::Spice, &io);

        assert_eq!(io.save_dialog_calls(), 0);
        assert_eq!(io.write_text_calls(), 0);
        assert!(
            state
                .console_messages
                .iter()
                .any(|message| message.message.contains("No circuit to export")),
            "empty netlist export should emit actionable warning"
        );
    }

    #[test]
    fn test_action_export_netlist_with_io_spice_writes_file_and_logs_success() {
        let io = Rc::new(MockExportWorkflowIo::default());
        io.push_save_dialog_result(Some(PathBuf::from("netlists/run")));
        io.push_write_text_result(Ok(()));

        let mut state = state_with_resistor_component();
        state.schematic.current_file = Some(PathBuf::from("designs/oscillator.rsch"));

        action_export_netlist_with_io(&mut state, crate::io::NetlistFormat::Spice, &io);

        assert_eq!(io.save_dialog_calls(), 1);
        assert_eq!(io.write_text_calls(), 1);
        assert_eq!(
            io.last_save_dialog_config(),
            Some(SaveDialogConfigSnapshot {
                title: "Export Netlist".to_string(),
                default_name: "oscillator.spice".to_string(),
                filter_name: "SPICE Netlist".to_string(),
                filter_extensions: vec!["spice".to_string()],
            })
        );
        assert_eq!(
            io.last_write_text_path(),
            Some(PathBuf::from("netlists/run.spice"))
        );
        let netlist = io
            .last_write_text_contents()
            .expect("written netlist content should be captured");
        assert!(netlist.contains("R1"));
        assert!(
            state
                .console_messages
                .iter()
                .any(|message| message.message.contains("Exported SPICE Netlist")),
            "successful netlist export should be logged"
        );
    }

    #[test]
    fn test_action_export_netlist_with_io_write_error_logs_failure() {
        let io = Rc::new(MockExportWorkflowIo::default());
        io.push_save_dialog_result(Some(PathBuf::from("netlists/fail.scs")));
        io.push_write_text_result(Err("permission denied".to_string()));

        let mut state = state_with_resistor_component();

        action_export_netlist_with_io(&mut state, crate::io::NetlistFormat::Spectre, &io);

        assert_eq!(io.write_text_calls(), 1);
        assert!(
            state
                .console_messages
                .iter()
                .any(|message| message.message.contains("Netlist export failed: permission denied")),
            "netlist write errors should be surfaced to users"
        );
    }
}
