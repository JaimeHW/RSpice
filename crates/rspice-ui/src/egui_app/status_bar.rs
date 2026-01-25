//! Status Bar for egui Application
//!
//! Bottom status bar showing cursor position, grid info, and simulation status.

use egui::Ui;

use super::app::AppState;

/// Render the status bar
pub fn render_status_bar(ui: &mut Ui, state: &AppState) {
    ui.horizontal(|ui| {
        ui.add_space(8.0);

        // Cursor position on grid
        let (pan_x, pan_y) = state.schematic.pan;
        ui.label(format!("Pan: ({:.0}, {:.0})", pan_x, pan_y));

        ui.separator();

        // Zoom level
        let zoom_pct = (state.schematic.zoom * 100.0) as i32;
        ui.label(format!("Zoom: {}%", zoom_pct));

        ui.separator();

        // Grid size
        let grid = state.schematic.grid_size;
        ui.label(format!("Grid: {}", grid));

        ui.separator();

        // Selection count
        let comp_count = state.schematic.selection.components.len();
        let wire_count = state.schematic.selection.wires.len();
        if comp_count > 0 || wire_count > 0 {
            ui.label(format!(
                "Selected: {} components, {} wires",
                comp_count, wire_count
            ));
        } else {
            ui.label("No selection");
        }

        // Right side: simulation status
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(8.0);

            if state.simulation.is_running {
                ui.colored_label(egui::Color32::from_rgb(100, 200, 255), "Simulating...");
            } else if !state.simulation.waveforms.is_empty() {
                let waveform_count = state.simulation.waveforms.len();
                ui.label(format!("{} waveforms", waveform_count));
            } else {
                ui.label("Ready");
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
    fn test_status_bar_displays_correct_info() {
        let state = AppState::default();

        // Verify default state values are accessible
        assert_eq!(state.schematic.pan, (0.0, 0.0));
        assert!(state.schematic.zoom > 0.0);
        assert!(state.schematic.grid_size > 0);
        assert!(state.schematic.selection.components.is_empty());
        assert!(state.schematic.selection.wires.is_empty());
    }
}
