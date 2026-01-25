//! Toolbar for egui Application
//!
//! Main toolbar with simulation controls, zoom, and tool selection.

use egui::{Ui, Vec2};

use super::app::AppState;

/// Tool selection mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToolMode {
    /// Selection/pointer tool
    #[default]
    Select,
    /// Wire drawing tool
    Wire,
    /// Component placement tool
    Component,
    /// Voltage probe tool
    ProbeVoltage,
    /// Current probe tool
    ProbeCurrent,
    /// Text annotation tool
    Text,
}

/// Render the main toolbar
pub fn render_toolbar(ui: &mut Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.add_space(8.0);

        // =====================================================================
        // Tool Selection
        // =====================================================================
        ui.label("Tools:");
        ui.add_space(4.0);

        // Select tool
        if ui
            .add(egui::Button::new("⬚").min_size(Vec2::splat(28.0)))
            .on_hover_text("Select (S)")
            .clicked()
        {
            state
                .console_messages
                .push(super::app::ConsoleMessage::info("Select tool active"));
        }

        // Wire tool
        if ui
            .add(egui::Button::new("↝").min_size(Vec2::splat(28.0)))
            .on_hover_text("Wire (W)")
            .clicked()
        {
            state
                .console_messages
                .push(super::app::ConsoleMessage::info("Wire tool active"));
        }

        // Component tool
        if ui
            .add(egui::Button::new("⊕").min_size(Vec2::splat(28.0)))
            .on_hover_text("Add Component (C)")
            .clicked()
        {
            state
                .console_messages
                .push(super::app::ConsoleMessage::info("Component tool active"));
        }

        // Probe tools
        if ui
            .add(egui::Button::new("🔬").min_size(Vec2::splat(28.0)))
            .on_hover_text("Voltage Probe (V)")
            .clicked()
        {
            state
                .console_messages
                .push(super::app::ConsoleMessage::info("Voltage probe active"));
        }

        ui.separator();

        // =====================================================================
        // Zoom Controls
        // =====================================================================
        ui.label("Zoom:");
        ui.add_space(4.0);

        if ui
            .add(egui::Button::new("−").min_size(Vec2::splat(28.0)))
            .on_hover_text("Zoom Out (Ctrl+-)")
            .clicked()
        {
            state.schematic.zoom = (state.schematic.zoom / 1.25).max(0.25);
        }

        // Zoom percentage display
        let zoom_pct = (state.schematic.zoom * 100.0) as i32;
        let zoom_text = format!("{}%", zoom_pct);
        ui.label(zoom_text);

        if ui
            .add(egui::Button::new("+").min_size(Vec2::splat(28.0)))
            .on_hover_text("Zoom In (Ctrl++)")
            .clicked()
        {
            state.schematic.zoom = (state.schematic.zoom * 1.25).min(4.0);
        }

        if ui
            .add(egui::Button::new("⟲").min_size(Vec2::splat(28.0)))
            .on_hover_text("Zoom to Fit")
            .clicked()
        {
            state.schematic.zoom_to_fit(800.0, 600.0);
        }

        ui.separator();

        // =====================================================================
        // Simulation Controls
        // =====================================================================
        ui.label("Simulation:");
        ui.add_space(4.0);

        // Run button
        let run_enabled = !state.schematic.components.is_empty() && !state.simulation.is_running;
        if ui
            .add_enabled(
                run_enabled,
                egui::Button::new("▶ Run").min_size(Vec2::new(60.0, 28.0)),
            )
            .on_hover_text("Run Simulation")
            .on_disabled_hover_text(if state.simulation.is_running {
                "Simulation running"
            } else {
                "Add components first"
            })
            .clicked()
        {
            state.simulation.trigger_simulation = true;
            state
                .console_messages
                .push(super::app::ConsoleMessage::info("Simulation started..."));
        }

        // Stop button
        let stop_enabled = state.simulation.is_running;
        if ui
            .add_enabled(
                stop_enabled,
                egui::Button::new("■ Stop").min_size(Vec2::new(60.0, 28.0)),
            )
            .on_hover_text("Stop Simulation")
            .clicked()
        {
            state.simulation.is_running = false;
            state
                .console_messages
                .push(super::app::ConsoleMessage::warning("Simulation stopped"));
        }

        // Setup button
        if ui
            .add(egui::Button::new("⚙ Setup").min_size(Vec2::new(60.0, 28.0)))
            .on_hover_text("Simulation Setup")
            .clicked()
        {
            state.dialogs.simulation_dialog = true;
        }

        // =====================================================================
        // Status indicator (right aligned)
        // =====================================================================
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if state.simulation.is_running {
                ui.colored_label(egui::Color32::from_rgb(100, 200, 255), "⏳ Simulating...");
            } else if !state.simulation.waveforms.is_empty() {
                ui.colored_label(egui::Color32::from_rgb(100, 200, 100), "✓ Results ready");
            }

            // Dirty indicator
            if state.schematic.is_dirty {
                ui.colored_label(egui::Color32::from_rgb(255, 180, 50), "●");
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
    fn test_tool_mode_default() {
        let mode = ToolMode::default();
        assert_eq!(mode, ToolMode::Select);
    }

    #[test]
    fn test_tool_mode_variants() {
        // Ensure all variants can be created
        let _select = ToolMode::Select;
        let _wire = ToolMode::Wire;
        let _component = ToolMode::Component;
        let _probe_v = ToolMode::ProbeVoltage;
        let _probe_i = ToolMode::ProbeCurrent;
        let _text = ToolMode::Text;
    }

    #[test]
    fn test_tool_mode_equality() {
        assert_eq!(ToolMode::Wire, ToolMode::Wire);
        assert_ne!(ToolMode::Wire, ToolMode::Select);
    }
}
