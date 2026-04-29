use egui::{Color32, Rect, RichText, Ui, Vec2, pos2};

use super::icons::{IconType, paint_icon};
use crate::common::app::AppState;
use crate::schematic::component_palette::component_palette;
use crate::state::{ComponentType, Tool};

/// Standard font size for toolbar labels
const TOOLBAR_FONT_SIZE: f32 = 14.0;

/// Render the main toolbar
pub fn render_toolbar(ui: &mut Ui, state: &mut AppState) {
    // Use horizontal_centered for proper vertical alignment of all elements
    ui.horizontal_centered(|ui| {
        ui.add_space(8.0);

        // =====================================================================
        // Tool Selection
        // =====================================================================
        ui.label(RichText::new("Tools:").size(TOOLBAR_FONT_SIZE));
        ui.add_space(4.0);

        // Select tool
        let is_select = state.schematic.tool.is_select();
        if icon_button(ui, IconType::Select, is_select)
            .on_hover_text("Select (S)")
            .clicked()
        {
            state.schematic.tool = Tool::Select;
        }

        // Wire tool
        let is_wire = state.schematic.tool.is_wire();
        if icon_button(ui, IconType::Wire, is_wire)
            .on_hover_text("Wire (W)")
            .clicked()
        {
            state.schematic.tool = Tool::Wire;
        }

        // Component dropdown - use icon_button for consistent sizing
        let is_place = state.schematic.tool.is_place_tool();
        let component_id = ui.make_persistent_id("component_menu");
        let component_response = icon_button(ui, IconType::Component, is_place);
        if component_response.clicked() {
            ui.memory_mut(|mem| mem.toggle_popup(component_id));
        }
        let hover_text = if is_place {
            format!("Placing: {}", state.schematic.tool.display_name())
        } else {
            "Add Component".to_string()
        };

        egui::popup_below_widget(
            ui,
            component_id,
            &component_response,
            egui::PopupCloseBehavior::CloseOnClickOutside,
            |ui| {
                ui.set_min_width(220.0);

                for (section_idx, section) in component_palette().iter().enumerate() {
                    ui.label(section.title);
                    ui.separator();

                    for entry in section.entries {
                        let button_label = if entry.kind == ComponentType::Resistor {
                            format!("{} (R+Shift)", entry.label)
                        } else if let Some(shortcut) = Tool::Place(entry.kind).shortcut() {
                            format!("{} ({})", entry.label, shortcut.to_ascii_uppercase())
                        } else {
                            entry.label.to_string()
                        };

                        if ui.button(button_label).clicked() {
                            state.schematic.tool = Tool::Place(entry.kind);
                            ui.memory_mut(|mem| mem.close_popup());
                        }
                    }

                    if section_idx + 1 < component_palette().len() {
                        ui.separator();
                    }
                }
            },
        );
        component_response.on_hover_text(hover_text);

        // Probe tool
        let is_probe = matches!(state.schematic.tool, Tool::Probe);
        if icon_button(ui, IconType::Probe, is_probe)
            .on_hover_text("Probe (P)")
            .clicked()
        {
            state.schematic.tool = Tool::Probe;
        }

        // Current tool indicator
        ui.separator();
        let tool_name = state.schematic.tool.display_name();
        ui.label(RichText::new(format!("Mode: {}", tool_name)).size(TOOLBAR_FONT_SIZE));

        ui.separator();

        // =====================================================================
        // Zoom Controls
        // =====================================================================
        ui.label(RichText::new("Zoom:").size(TOOLBAR_FONT_SIZE));
        ui.add_space(4.0);

        if icon_button(ui, IconType::ZoomOut, false)
            .on_hover_text("Zoom Out (Ctrl+-)")
            .clicked()
        {
            state.schematic.zoom = (state.schematic.zoom / 1.25).max(0.25);
        }

        // Zoom percentage display
        let zoom_pct = (state.schematic.zoom * 100.0) as i32;
        let zoom_text = format!("{}%", zoom_pct);
        ui.label(RichText::new(zoom_text).size(TOOLBAR_FONT_SIZE));

        if icon_button(ui, IconType::ZoomIn, false)
            .on_hover_text("Zoom In (Ctrl++)")
            .clicked()
        {
            state.schematic.zoom = (state.schematic.zoom * 1.25).min(4.0);
        }

        if icon_button(ui, IconType::ZoomFit, false)
            .on_hover_text("Zoom to Fit")
            .clicked()
        {
            // Set flag to trigger zoom-to-fit on next render with actual canvas dimensions
            state.schematic.needs_fit = true;
        }

        ui.separator();

        // =====================================================================
        // Simulation Controls
        // =====================================================================
        ui.label(RichText::new("Simulation:").size(TOOLBAR_FONT_SIZE));
        ui.add_space(4.0);

        // Run button
        let run_enabled = !state.schematic.components.is_empty() && !state.simulation.is_running;
        if icon_text_button(ui, IconType::Play, "Run", run_enabled)
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
            state.push_user_message(crate::common::app::ConsoleMessage::info(
                "Simulation started...",
            ));
        }

        // Stop button
        let stop_enabled = state.simulation.is_running;
        if icon_text_button(ui, IconType::Stop, "Stop", stop_enabled)
            .on_hover_text("Stop Simulation")
            .clicked()
        {
            // Set the abort trigger - SimulationController will call runner.abort()
            state.simulation.trigger_abort = true;
            state.push_user_message(crate::common::app::ConsoleMessage::warning(
                "Stopping simulation...",
            ));
        }

        // Setup button
        if icon_text_button(ui, IconType::Settings, "Setup", true)
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
                status_indicator(
                    ui,
                    IconType::Spinner,
                    "Simulating...",
                    Color32::from_rgb(100, 200, 255),
                );
            } else if !state.simulation.waveforms.is_empty() {
                status_indicator(
                    ui,
                    IconType::Checkmark,
                    "Results ready",
                    Color32::from_rgb(100, 200, 100),
                );
            }

            // Dirty indicator
            if state.schematic.is_dirty {
                status_indicator(ui, IconType::DirtyDot, "", Color32::from_rgb(255, 180, 50));
            }
        });
    });
}

/// Create a tool button with custom vector icon and active highlighting
fn icon_button(ui: &mut Ui, icon: IconType, active: bool) -> egui::Response {
    let size = Vec2::splat(28.0);
    let fill = if active {
        Color32::from_rgb(60, 100, 160)
    } else {
        ui.visuals().widgets.inactive.bg_fill
    };

    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let visuals = ui.visuals().widgets.style(&response);
        let rounding = visuals.rounding;
        let stroke = visuals.bg_stroke;

        ui.painter().rect(rect, rounding, fill, stroke);

        let icon_color = if active {
            Color32::WHITE
        } else {
            visuals.text_color()
        };
        paint_icon(ui, rect, icon, icon_color);
    }

    response
}

/// Create a button with both icon and text label
fn icon_text_button(ui: &mut Ui, icon: IconType, text: &str, enabled: bool) -> egui::Response {
    let icon_size = 20.0; // Larger icons for visibility
    let padding = 6.0;
    let font_size = 14.0; // Explicit font size for readability
    let text_galley = ui.fonts(|fonts| {
        fonts.layout_no_wrap(
            text.to_string(),
            egui::FontId::proportional(font_size),
            if enabled {
                ui.visuals().widgets.inactive.text_color()
            } else {
                ui.visuals().widgets.noninteractive.text_color()
            },
        )
    });
    let total_width = icon_size + padding + text_galley.size().x + padding * 2.0;
    let height = 28.0;

    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(total_width, height),
        if enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );

    if ui.is_rect_visible(rect) {
        let visuals = if enabled {
            ui.visuals().widgets.style(&response)
        } else {
            &ui.visuals().widgets.noninteractive
        };
        let rounding = visuals.rounding;
        let fill = visuals.bg_fill;
        let stroke = visuals.bg_stroke;

        ui.painter().rect(rect, rounding, fill, stroke);

        // Draw icon on left
        let icon_rect = Rect::from_min_size(
            pos2(rect.left() + padding, rect.center().y - icon_size * 0.5),
            Vec2::splat(icon_size),
        );
        let icon_color = if enabled {
            visuals.text_color()
        } else {
            ui.visuals().widgets.noninteractive.text_color()
        };
        paint_icon(ui, icon_rect, icon, icon_color);

        // Draw text on right
        let text_pos = pos2(
            icon_rect.right() + padding,
            rect.center().y - text_galley.size().y * 0.5,
        );
        ui.painter().galley(text_pos, text_galley, Color32::WHITE);
    }

    response
}

/// Display a status indicator with icon and optional text
fn status_indicator(ui: &mut Ui, icon: IconType, text: &str, color: Color32) {
    let icon_size = 14.0;

    // Allocate space for icon
    let (icon_rect, _) = ui.allocate_exact_size(Vec2::splat(icon_size), egui::Sense::hover());

    if ui.is_rect_visible(icon_rect) {
        paint_icon(ui, icon_rect, icon, color);
    }

    // Add text label if provided
    if !text.is_empty() {
        ui.add_space(2.0);
        ui.colored_label(color, text);
    }
}
