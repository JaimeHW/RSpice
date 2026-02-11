use crate::common::app::AppState;

pub(super) fn action_export_svg(state: &mut AppState) {
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

    let dialog = rfd::FileDialog::new()
        .add_filter("SVG Image", &["svg"])
        .set_file_name(&default_name)
        .set_title("Export SVG");

    match dialog.save_file() {
        Some(mut path) => {
            super::menu_bar_file_actions::ensure_file_extension(&mut path, "svg");

            match std::fs::write(&path, &svg_content) {
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

    let dialog = rfd::FileDialog::new()
        .add_filter(filter_name, &[format.extension()])
        .set_file_name(&default_name)
        .set_title("Export Netlist");

    match dialog.save_file() {
        Some(mut path) => {
            super::menu_bar_file_actions::ensure_file_extension(&mut path, format.extension());

            match std::fs::write(&path, &netlist_content) {
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

    #[test]
    fn test_action_view_netlist_uses_generated_schematic_netlist() {
        let mut state = AppState::default();
        use crate::state::{Component, ComponentType, Point};
        let comp = Component::new(1, ComponentType::Resistor, Point::new(100, 100))
            .with_name_value("R1", "1k");
        state.schematic.components.push(comp);

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
}
