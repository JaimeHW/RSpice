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
        Some(mut path) => {
            crate::common::file_actions::ensure_file_extension(&mut path, "svg");

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
        Some(mut path) => {
            crate::common::file_actions::ensure_file_extension(&mut path, extension);

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
    let generation = crate::simulation::netlist_gen::generate_netlist_hierarchical(
        &state.schematic,
        &[],
        &hierarchy,
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
