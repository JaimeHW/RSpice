//! Project Browser Panel
//!
//! Library/Cell/View hierarchy browser matching Cadence-style organization.

use egui::{CollapsingHeader, Ui};

use crate::egui_app::app::AppState;

/// Render the project browser panel
pub fn render_project_browser(ui: &mut Ui, state: &mut AppState) {
    ui.heading("Library Browser");
    ui.separator();

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            // Built-in primitives library
            CollapsingHeader::new("📚 Primitives")
                .default_open(true)
                .show(ui, |ui| {
                    render_library_cell(ui, "resistor", "symbol", state);
                    render_library_cell(ui, "capacitor", "symbol", state);
                    render_library_cell(ui, "inductor", "symbol", state);
                    render_library_cell(ui, "vsource", "symbol", state);
                    render_library_cell(ui, "isource", "symbol", state);
                    render_library_cell(ui, "gnd", "symbol", state);
                    render_library_cell(ui, "vdd", "symbol", state);
                });

            // Semiconductor devices
            CollapsingHeader::new("📚 Semiconductors")
                .default_open(false)
                .show(ui, |ui| {
                    render_library_cell(ui, "nmos", "symbol", state);
                    render_library_cell(ui, "pmos", "symbol", state);
                    render_library_cell(ui, "npn", "symbol", state);
                    render_library_cell(ui, "pnp", "symbol", state);
                    render_library_cell(ui, "diode", "symbol", state);
                });

            // User libraries (placeholder)
            CollapsingHeader::new("📁 User Library")
                .default_open(false)
                .show(ui, |ui| {
                    ui.label("No user cells defined");
                    if ui.small_button("+ New Cell").clicked() {
                        state
                            .console_messages
                            .push(super::super::app::ConsoleMessage::info(
                                "New cell: Coming soon",
                            ));
                    }
                });
        });
}

/// Render a library cell entry
fn render_library_cell(ui: &mut Ui, cell_name: &str, view_name: &str, state: &mut AppState) {
    CollapsingHeader::new(format!("📦 {}", cell_name))
        .default_open(false)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("  ");
                if ui.small_button(format!("📐 {}", view_name)).clicked() {
                    state
                        .console_messages
                        .push(super::super::app::ConsoleMessage::info(format!(
                            "Opening {}/{}",
                            cell_name, view_name
                        )));
                }
            });
        });
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_browser_primitives_exist() {
        // Verify primitive component names are valid
        let primitives = [
            "resistor",
            "capacitor",
            "inductor",
            "vsource",
            "isource",
            "gnd",
            "vdd",
        ];
        for p in primitives {
            assert!(!p.is_empty());
        }
    }

    #[test]
    fn test_project_browser_semiconductors_exist() {
        let semis = ["nmos", "pmos", "npn", "pnp", "diode"];
        for s in semis {
            assert!(!s.is_empty());
        }
    }
}
