//! Toolbar for egui Application
//!
//! Main toolbar with simulation controls, zoom, and tool selection.

use egui::{Color32, Ui, Vec2};

use crate::common::app::AppState;
use crate::state::{ComponentType, Tool};

// =============================================================================
// Toolbar Rendering
// =============================================================================

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
        let is_select = state.schematic.tool.is_select();
        if ui
            .add(tool_button("⬚", is_select))
            .on_hover_text("Select (S)")
            .clicked()
        {
            state.schematic.tool = Tool::Select;
        }

        // Wire tool
        let is_wire = state.schematic.tool.is_wire();
        if ui
            .add(tool_button("↝", is_wire))
            .on_hover_text("Wire (W)")
            .clicked()
        {
            state.schematic.tool = Tool::Wire;
        }

        // Component dropdown
        let is_place = state.schematic.tool.is_place_tool();
        ui.menu_button(egui::RichText::new("⊕").size(14.0), |ui| {
            ui.set_min_width(120.0);
            ui.label("Components");
            ui.separator();

            if ui.button("Resistor (R+Shift)").clicked() {
                state.schematic.tool = Tool::Place(ComponentType::Resistor);
                ui.close_menu();
            }
            if ui.button("Capacitor (C)").clicked() {
                state.schematic.tool = Tool::Place(ComponentType::Capacitor);
                ui.close_menu();
            }
            if ui.button("Inductor (L)").clicked() {
                state.schematic.tool = Tool::Place(ComponentType::Inductor);
                ui.close_menu();
            }

            ui.separator();

            if ui.button("Voltage Source (V)").clicked() {
                state.schematic.tool = Tool::Place(ComponentType::VoltageSource);
                ui.close_menu();
            }
            if ui.button("Current Source (I)").clicked() {
                state.schematic.tool = Tool::Place(ComponentType::CurrentSource);
                ui.close_menu();
            }
            if ui.button("Ground (G)").clicked() {
                state.schematic.tool = Tool::Place(ComponentType::Ground);
                ui.close_menu();
            }

            ui.separator();

            if ui.button("Diode (D)").clicked() {
                state.schematic.tool = Tool::Place(ComponentType::Diode);
                ui.close_menu();
            }
            if ui.button("NMOS (M)").clicked() {
                state.schematic.tool = Tool::Place(ComponentType::Nmos);
                ui.close_menu();
            }
            if ui.button("PMOS").clicked() {
                state.schematic.tool = Tool::Place(ComponentType::Pmos);
                ui.close_menu();
            }
            if ui.button("NPN BJT (Q)").clicked() {
                state.schematic.tool = Tool::Place(ComponentType::NpnBjt);
                ui.close_menu();
            }
            if ui.button("PNP BJT").clicked() {
                state.schematic.tool = Tool::Place(ComponentType::PnpBjt);
                ui.close_menu();
            }
        })
        .response
        .on_hover_text(if is_place {
            format!("Placing: {}", state.schematic.tool.display_name())
        } else {
            "Add Component".to_string()
        });

        // Probe tool
        let is_probe = matches!(state.schematic.tool, Tool::Probe);
        if ui
            .add(tool_button("🔬", is_probe))
            .on_hover_text("Probe (P)")
            .clicked()
        {
            state.schematic.tool = Tool::Probe;
        }

        // Current tool indicator
        ui.separator();
        let tool_name = state.schematic.tool.display_name();
        ui.label(format!("Mode: {}", tool_name));

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

        // Run button - log once when components become available
        static LOGGED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        if !LOGGED.load(std::sync::atomic::Ordering::Relaxed)
            && !state.schematic.components.is_empty()
        {
            log::info!(
                "Run button now enabled: {} components",
                state.schematic.components.len()
            );
            LOGGED.store(true, std::sync::atomic::Ordering::Relaxed);
        }
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
            log::info!(
                "Toolbar Run clicked! Components: {}",
                state.schematic.components.len()
            );
            state.simulation.trigger_simulation = true;
            state
                .console_messages
                .push(crate::common::app::ConsoleMessage::info(
                    "Simulation started...",
                ));
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
                .push(crate::common::app::ConsoleMessage::warning(
                    "Simulation stopped",
                ));
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
                ui.colored_label(Color32::from_rgb(100, 200, 255), "⏳ Simulating...");
            } else if !state.simulation.waveforms.is_empty() {
                ui.colored_label(Color32::from_rgb(100, 200, 100), "✓ Results ready");
            }

            // Dirty indicator
            if state.schematic.is_dirty {
                ui.colored_label(Color32::from_rgb(255, 180, 50), "●");
            }
        });
    });
}

/// Create a tool button with active highlighting
fn tool_button(icon: &str, active: bool) -> egui::Button<'_> {
    let btn = egui::Button::new(icon).min_size(Vec2::splat(28.0));
    if active {
        btn.fill(Color32::from_rgb(60, 100, 160))
    } else {
        btn
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_button_creates_button() {
        // Just verify it compiles and creates a button
        let _btn = tool_button("⬚", true);
        let _btn2 = tool_button("⬚", false);
    }

    #[test]
    fn test_tool_button_different_icons() {
        let _select = tool_button("⬚", false);
        let _wire = tool_button("↝", false);
        let _probe = tool_button("🔬", true);
    }
}
