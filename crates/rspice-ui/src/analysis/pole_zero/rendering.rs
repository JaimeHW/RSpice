//! Pole-Zero Plot Rendering
//!
//! Commercial-grade egui rendering for pole-zero visualization.

use egui::{Color32, FontId, Pos2, Rect, Rounding, Sense, Stroke, Ui, UiBuilder, Vec2};

use super::data::{PoleZeroData, RootType};
use super::state::{DomainType, PoleZeroState};
use crate::common::app::AppState;
use crate::common::viewer_style::viewer_header_bg_color;

// =============================================================================
// Constants
// =============================================================================

fn chart_bg_color() -> Color32 {
    Color32::from_rgb(15, 17, 21)
}

fn grid_color() -> Color32 {
    Color32::from_rgb(40, 45, 55)
}

fn axis_color() -> Color32 {
    Color32::from_rgb(80, 85, 95)
}

fn pole_color() -> Color32 {
    Color32::from_rgb(255, 100, 100)
}

fn zero_color() -> Color32 {
    Color32::from_rgb(100, 200, 255)
}

fn unit_circle_color() -> Color32 {
    Color32::from_rgba_unmultiplied(255, 200, 100, 100)
}

fn stable_region_color() -> Color32 {
    Color32::from_rgba_unmultiplied(50, 150, 50, 30)
}

fn text_color() -> Color32 {
    Color32::from_rgb(180, 185, 195)
}

// =============================================================================
// Main Rendering Entry Point
// =============================================================================

/// Render the pole-zero viewer panel
pub fn render_pz_viewer(ui: &mut Ui, _app_state: &mut AppState) {
    let mut state = PoleZeroState::new();
    load_demo_data(&mut state);

    let available_rect = ui.available_rect_before_wrap();
    let layout = calculate_layout(available_rect);

    render_header(ui, &layout, &mut state);
    render_plot(ui, &layout, &state);
    render_info_panel(ui, &layout, &state);
}

/// Public render function
pub fn render_pz_plot(ui: &mut Ui, state: &PoleZeroState) {
    let available_rect = ui.available_rect_before_wrap();
    let layout = calculate_layout(available_rect);

    render_plot_core(ui, &layout, state);
}

// =============================================================================
// Layout
// =============================================================================

#[derive(Debug, Clone)]
struct PzLayout {
    header: Rect,
    plot: Rect,
    info: Rect,
}

const HEADER_HEIGHT: f32 = 32.0;
const INFO_WIDTH: f32 = 140.0;
const CHART_PADDING: f32 = 8.0;

fn calculate_layout(available: Rect) -> PzLayout {
    let total = available;

    let header = Rect::from_min_size(total.min, Vec2::new(total.width(), HEADER_HEIGHT));

    let info = Rect::from_min_size(
        Pos2::new(total.max.x - INFO_WIDTH, header.max.y),
        Vec2::new(INFO_WIDTH, total.height() - HEADER_HEIGHT),
    );

    let plot = Rect::from_min_max(
        Pos2::new(total.min.x + CHART_PADDING, header.max.y + CHART_PADDING),
        Pos2::new(info.min.x - CHART_PADDING, total.max.y - CHART_PADDING),
    );

    PzLayout { header, plot, info }
}

// =============================================================================
// Header Rendering
// =============================================================================

fn render_header(ui: &mut Ui, layout: &PzLayout, state: &mut PoleZeroState) {
    ui.painter()
        .rect_filled(layout.header, Rounding::ZERO, viewer_header_bg_color());

    let header_rect = layout.header.shrink(4.0);
    ui.allocate_new_ui(UiBuilder::new().max_rect(header_rect), |ui| {
        ui.horizontal(|ui| {
            ui.add_space(8.0);

            ui.label(
                egui::RichText::new("Pole-Zero Map")
                    .size(13.0)
                    .strong()
                    .color(Color32::from_rgb(200, 200, 210)),
            );

            ui.add_space(16.0);

            // Domain selector
            egui::ComboBox::from_id_salt("domain")
                .selected_text(state.domain.display_name())
                .show_ui(ui, |ui| {
                    for d in DomainType::all() {
                        ui.selectable_value(&mut state.domain, *d, d.display_name());
                    }
                });

            ui.separator();

            // Toggle buttons
            if ui
                .small_button(if state.show_grid { "Grid ✓" } else { "Grid" })
                .clicked()
            {
                state.toggle_grid();
            }

            if ui
                .small_button(if state.show_unit_circle {
                    "Circle ✓"
                } else {
                    "Circle"
                })
                .clicked()
            {
                state.toggle_unit_circle();
            }

            if ui
                .small_button(if state.show_stability_region {
                    "Stable ✓"
                } else {
                    "Stable"
                })
                .clicked()
            {
                state.toggle_stability_region();
            }
        });
    });
}

// =============================================================================
// Plot Rendering
// =============================================================================

fn render_plot(ui: &mut Ui, layout: &PzLayout, state: &PoleZeroState) {
    render_plot_core(ui, layout, state);
    ui.allocate_rect(layout.plot, Sense::click());
}

fn render_plot_core(ui: &mut Ui, layout: &PzLayout, state: &PoleZeroState) {
    let painter = ui.painter().clone();
    let rect = layout.plot;

    // Background
    painter.rect_filled(rect, Rounding::ZERO, chart_bg_color());

    // Stability region shading
    if state.show_stability_region {
        render_stability_region(&painter, rect, state);
    }

    // Grid
    if state.show_grid {
        render_grid(&painter, rect, state);
    }

    // Axes
    render_axes(&painter, rect, state);

    // Unit circle (for z-domain or as reference)
    if state.show_unit_circle && state.domain == DomainType::ZDomain {
        render_unit_circle(&painter, rect, state);
    }

    // Poles and zeros
    for data in &state.datasets {
        render_roots(&painter, rect, data, state);
    }

    // Border
    painter.rect_stroke(
        rect,
        Rounding::ZERO,
        Stroke::new(1.0, Color32::from_rgb(60, 65, 75)),
    );
}

fn render_stability_region(painter: &egui::Painter, rect: Rect, state: &PoleZeroState) {
    match state.domain {
        DomainType::SDomain => {
            // Left half-plane (Re < 0) is stable
            let zero_x = map_to_screen(0.0, 0.0, rect, state).x;
            if zero_x > rect.min.x && zero_x < rect.max.x {
                let stable_rect = Rect::from_min_max(rect.min, Pos2::new(zero_x, rect.max.y));
                painter.rect_filled(stable_rect, Rounding::ZERO, stable_region_color());
            }
        }
        DomainType::ZDomain => {
            // Inside unit circle is stable (already handled by unit circle rendering)
        }
    }
}

fn render_grid(painter: &egui::Painter, rect: Rect, _state: &PoleZeroState) {
    let stroke = Stroke::new(0.5, grid_color());

    let num_lines = 8;
    for i in 0..=num_lines {
        let t = i as f32 / num_lines as f32;
        let x = rect.min.x + t * rect.width();
        let y = rect.min.y + t * rect.height();
        painter.line_segment([Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)], stroke);
        painter.line_segment([Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)], stroke);
    }
}

fn render_axes(painter: &egui::Painter, rect: Rect, state: &PoleZeroState) {
    let stroke = Stroke::new(1.0, axis_color());

    // Real axis
    let zero_pos = map_to_screen(0.0, 0.0, rect, state);
    if zero_pos.y >= rect.min.y && zero_pos.y <= rect.max.y {
        painter.line_segment(
            [
                Pos2::new(rect.min.x, zero_pos.y),
                Pos2::new(rect.max.x, zero_pos.y),
            ],
            stroke,
        );
    }

    // Imaginary axis
    if zero_pos.x >= rect.min.x && zero_pos.x <= rect.max.x {
        painter.line_segment(
            [
                Pos2::new(zero_pos.x, rect.min.y),
                Pos2::new(zero_pos.x, rect.max.y),
            ],
            stroke,
        );
    }

    // Labels
    painter.text(
        Pos2::new(rect.max.x - 5.0, zero_pos.y + 12.0),
        egui::Align2::RIGHT_TOP,
        "Re",
        FontId::proportional(10.0),
        text_color(),
    );

    painter.text(
        Pos2::new(zero_pos.x + 5.0, rect.min.y + 5.0),
        egui::Align2::LEFT_TOP,
        "Im",
        FontId::proportional(10.0),
        text_color(),
    );
}

fn render_unit_circle(painter: &egui::Painter, rect: Rect, state: &PoleZeroState) {
    let center = map_to_screen(0.0, 0.0, rect, state);
    let edge = map_to_screen(1.0, 0.0, rect, state);
    let radius = (edge.x - center.x).abs();

    painter.circle_stroke(center, radius, Stroke::new(1.5, unit_circle_color()));
}

fn render_roots(painter: &egui::Painter, rect: Rect, data: &PoleZeroData, state: &PoleZeroState) {
    for root in &data.roots {
        let pos = map_to_screen(root.real, root.imag, rect, state);

        if pos.x < rect.min.x || pos.x > rect.max.x || pos.y < rect.min.y || pos.y > rect.max.y {
            continue;
        }

        let size = 6.0;

        match root.root_type {
            RootType::Pole => {
                // Draw X
                let stroke = Stroke::new(2.0, pole_color());
                painter.line_segment(
                    [
                        Pos2::new(pos.x - size, pos.y - size),
                        Pos2::new(pos.x + size, pos.y + size),
                    ],
                    stroke,
                );
                painter.line_segment(
                    [
                        Pos2::new(pos.x + size, pos.y - size),
                        Pos2::new(pos.x - size, pos.y + size),
                    ],
                    stroke,
                );
            }
            RootType::Zero => {
                // Draw O
                painter.circle_stroke(pos, size, Stroke::new(2.0, zero_color()));
            }
        }

        // Annotation
        if state.show_annotations && root.is_pole() && !root.is_real() && root.imag > 0.0
            && let Some(q) = root.q_factor() {
                painter.text(
                    Pos2::new(pos.x + 10.0, pos.y - 5.0),
                    egui::Align2::LEFT_CENTER,
                    format!("Q={:.1}", q),
                    FontId::proportional(9.0),
                    text_color(),
                );
            }
    }
}

fn map_to_screen(real: f64, imag: f64, rect: Rect, state: &PoleZeroState) -> Pos2 {
    let real_range = state.real_max - state.real_min;
    let imag_range = state.imag_max - state.imag_min;

    let x = if real_range > 0.0 {
        rect.min.x + ((real - state.real_min) / real_range) as f32 * rect.width()
    } else {
        rect.center().x
    };

    let y = if imag_range > 0.0 {
        rect.max.y - ((imag - state.imag_min) / imag_range) as f32 * rect.height()
    } else {
        rect.center().y
    };

    Pos2::new(x, y)
}

// =============================================================================
// Info Panel
// =============================================================================

fn render_info_panel(ui: &mut Ui, layout: &PzLayout, state: &PoleZeroState) {
    ui.painter()
        .rect_filled(layout.info, Rounding::ZERO, Color32::from_rgb(25, 27, 33));

    let panel_rect = layout.info.shrink(8.0);
    ui.allocate_new_ui(UiBuilder::new().max_rect(panel_rect), |ui| {
        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new("System Info")
                    .size(11.0)
                    .color(text_color()),
            );
            ui.add_space(8.0);

            if let Some(data) = state.current() {
                info_row(ui, "Poles", &format!("{}", data.pole_count()));
                info_row(ui, "Zeros", &format!("{}", data.zero_count()));
                info_row(ui, "Order", &format!("{}", data.system_order()));
                info_row(ui, "Rel Deg", &format!("{}", data.relative_degree()));

                ui.add_space(4.0);
                ui.separator();
                ui.add_space(4.0);

                // Stability
                let is_stable = data.is_stable();
                let stability_text = if is_stable { "STABLE" } else { "UNSTABLE" };
                let stability_color = if is_stable {
                    Color32::from_rgb(100, 200, 100)
                } else {
                    Color32::from_rgb(255, 100, 100)
                };

                ui.label(
                    egui::RichText::new(stability_text)
                        .size(12.0)
                        .strong()
                        .color(stability_color),
                );

                if data.is_marginally_stable() {
                    ui.label(
                        egui::RichText::new("(Marginal)")
                            .size(10.0)
                            .color(Color32::from_rgb(200, 200, 100)),
                    );
                }

                // Dominant pole info
                ui.add_space(4.0);
                let dominant = data.dominant_poles();
                if !dominant.is_empty()
                    && let Some(dom) = dominant.first() {
                        if let Some(q) = dom.q_factor() {
                            info_row(ui, "Dom Q", &format!("{:.2}", q));
                        }
                        let zeta = dom.damping_ratio();
                        if zeta > 0.0 && zeta < 1.0 {
                            info_row(ui, "ζ", &format!("{:.3}", zeta));
                        }
                    }
            } else {
                ui.label(
                    egui::RichText::new("No data")
                        .size(10.0)
                        .color(Color32::from_rgb(100, 105, 115)),
                );
            }
        });
    });
}

fn info_row(ui: &mut Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(format!("{}:", label))
                .size(10.0)
                .color(Color32::from_rgb(120, 125, 135)),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(value)
                    .size(11.0)
                    .color(Color32::from_rgb(200, 205, 215)),
            );
        });
    });
}

// =============================================================================
// Demo Data
// =============================================================================

fn load_demo_data(state: &mut PoleZeroState) {
    // Create demo second-order system
    let mut data = PoleZeroData::new("Demo System");

    // Second-order with Q = 5 (underdamped)
    // ζ = 0.1, ωn = 10 rad/s
    // Poles at -ζωn ± jωn√(1-ζ²) = -1 ± j9.95
    let zeta: f64 = 0.1;
    let wn: f64 = 10.0;
    let sigma = -zeta * wn;
    let omega = wn * (1.0_f64 - zeta * zeta).sqrt();

    data.add_pole_pair(sigma, omega);

    // Add a real zero
    data.add_real_zero(-5.0);

    state.load_data(data);
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_calculation() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0));
        let layout = calculate_layout(rect);

        assert!(layout.plot.width() > 0.0);
        assert!(layout.plot.height() > 0.0);
    }

    #[test]
    fn test_map_to_screen() {
        let rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(100.0, 100.0));
        let mut state = PoleZeroState::new();
        state.real_min = -2.0;
        state.real_max = 2.0;
        state.imag_min = -2.0;
        state.imag_max = 2.0;

        // Origin should map to center
        let center = map_to_screen(0.0, 0.0, rect, &state);
        assert!((center.x - 50.0).abs() < 0.1);
        assert!((center.y - 50.0).abs() < 0.1);
    }

    #[test]
    fn test_load_demo_data() {
        let mut state = PoleZeroState::new();
        load_demo_data(&mut state);

        assert!(!state.is_empty());
        assert!(state.current().is_some());
    }

    #[test]
    fn test_demo_data_stable() {
        let mut state = PoleZeroState::new();
        load_demo_data(&mut state);

        // Demo system should be stable
        assert_eq!(state.is_stable(), Some(true));
    }
}
