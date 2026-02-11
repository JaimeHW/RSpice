use egui::{Color32, Ui, Vec2};

use super::RSpiceApp;

/// Create an icon button for the icon rail (32x32 procedural icon)
/// Uses direct positioning to ensure pixel-perfect centering
fn rail_icon_button(
    ui: &mut Ui,
    icon: crate::schematic::toolbar::IconType,
    active: bool,
    accent: Color32,
) -> egui::Response {
    use crate::schematic::toolbar::paint_icon;

    let size = Vec2::splat(32.0);
    let fill = if active {
        accent
    } else {
        Color32::from_rgb(58, 62, 74)
    };

    // Allocate vertical space for the button height
    let (full_rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), size.y),
        egui::Sense::hover(),
    );

    // Calculate centered rect within the allocated space
    let center_x = full_rect.center().x;
    let rect = egui::Rect::from_center_size(egui::pos2(center_x, full_rect.center().y), size);

    // Register the actual button interaction area
    let response = ui.interact(
        rect,
        ui.id().with("rail_btn").with(icon as u8),
        egui::Sense::click(),
    );

    if ui.is_rect_visible(rect) {
        let visuals = ui.visuals().widgets.style(&response);
        let rounding = egui::Rounding::same(4.0);
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

/// Create a disabled icon button for the icon rail.
#[allow(dead_code)]
fn rail_icon_button_disabled(
    ui: &mut Ui,
    icon: crate::schematic::toolbar::IconType,
) -> egui::Response {
    use crate::schematic::toolbar::paint_icon;

    let size = Vec2::splat(32.0);
    let fill = Color32::from_rgb(45, 48, 56);

    // Allocate vertical space for the button height
    let (full_rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), size.y),
        egui::Sense::hover(),
    );

    // Calculate centered rect within the allocated space
    let center_x = full_rect.center().x;
    let rect = egui::Rect::from_center_size(egui::pos2(center_x, full_rect.center().y), size);

    // Register the actual button interaction area (hover only for disabled)
    let response = ui.interact(
        rect,
        ui.id().with("rail_btn_dis").with(icon as u8),
        egui::Sense::hover(),
    );

    if ui.is_rect_visible(rect) {
        let rounding = egui::Rounding::same(4.0);

        ui.painter().rect(rect, rounding, fill, egui::Stroke::NONE);

        let icon_color = ui.visuals().widgets.noninteractive.text_color();
        paint_icon(ui, rect, icon, icon_color);
    }

    response
}

impl RSpiceApp {
    /// Render the left icon rail (VSCode style).
    pub(super) fn render_icon_rail(&mut self, ui: &mut Ui) {
        use crate::schematic::toolbar::IconType;

        // Use zero item spacing for precise centering control
        ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

        ui.vertical(|ui| {
            ui.add_space(4.0);

            // Project browser toggle
            let browser_active = self.state.panels.project_browser;
            if rail_icon_button(
                ui,
                IconType::Folder,
                browser_active,
                self.state.theme.accent,
            )
            .on_hover_text("Library Browser (Ctrl+Shift+L)")
            .clicked()
            {
                self.toggle_panel_browser();
            }

            ui.add_space(4.0);

            // Results browser toggle - always clickable, shows empty state if no results
            let results_active = self.state.panels.results_browser;
            let has_results = self.state.simulation.has_results();
            let results_response = rail_icon_button(
                ui,
                IconType::Results,
                results_active,
                self.state.theme.accent,
            );
            if results_response
                .on_hover_text(if has_results {
                    "Results Browser"
                } else {
                    "Results Browser (no results yet)"
                })
                .clicked()
            {
                // Toggle results browser, and close library browser if opening results
                if !self.state.panels.results_browser {
                    self.state.panels.project_browser = false;
                }
                self.state.panels.results_browser = !self.state.panels.results_browser;
            }

            ui.add_space(4.0);

            // Spacer to push bottom items down
            ui.add_space(ui.available_height() - 40.0);

            // Unified Bottom Panel Toggle
            // Replaces separate Log and Waveform buttons for a cleaner UI
            // The active tab is managed within the panel itself
            let panel_active = self.state.panels.bottom_panel;
            if rail_icon_button(
                ui,
                IconType::PanelBottom,
                panel_active,
                self.state.theme.accent,
            )
            .on_hover_text("Toggle Bottom Panel (Ctrl+`)")
            .clicked()
            {
                self.toggle_bottom_panel();
            }
        });
    }
}
