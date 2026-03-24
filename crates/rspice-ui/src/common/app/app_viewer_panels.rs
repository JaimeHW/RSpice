use egui::{Color32, RichText, Stroke, Ui};

use super::RSpiceApp;
use crate::common::viewer_style::viewer_header_bg_color;
use crate::simulation::controller::DerivedViewerLoadState;

const VIEWER_TAB_CHIP_OUTER_HEIGHT: f32 = VIEWER_TAB_ROW_HEIGHT + 2.0 * VIEWER_TAB_INNER_Y;
const VIEWER_TAB_CHIP_TOP_SPACE: f32 = 5.0;
const VIEWER_TAB_CHIP_BOTTOM_SPACE: f32 = 2.0;
const VIEWER_TAB_STRIP_MIN_HEIGHT: f32 =
    VIEWER_TAB_CHIP_OUTER_HEIGHT + VIEWER_TAB_CHIP_TOP_SPACE + VIEWER_TAB_CHIP_BOTTOM_SPACE;
const VIEWER_TAB_STRIP_PAD_X: f32 = 8.0;
const VIEWER_TAB_STRIP_PAD_Y: f32 = 1.0;
const VIEWER_TAB_SPACING: f32 = 6.0;
const VIEWER_TAB_ROW_HEIGHT: f32 = 16.0;
const VIEWER_TAB_CLOSE_SIZE: f32 = 12.0;
const VIEWER_ADD_BUTTON_WIDTH: f32 = 104.0;
const VIEWER_TAB_ROUNDING: f32 = 8.0;
const VIEWER_TAB_INNER_X: f32 = 9.0;
const VIEWER_TAB_INNER_Y: f32 = 2.0;
const VIEWER_TAB_HOVER_OVERLAY_ALPHA: u8 = 26;
const VIEWER_TAB_HOVER_STROKE_BOOST: f32 = 0.35;
const VIEWER_TAB_CLOSE_HOVER_BG_ALPHA: u8 = 52;

fn tab_strip_fill() -> Color32 {
    viewer_header_bg_color()
}

fn tab_fill(selected: bool) -> Color32 {
    if selected {
        Color32::from_rgb(52, 74, 110)
    } else {
        Color32::from_rgb(28, 32, 40)
    }
}

fn tab_stroke(selected: bool) -> Stroke {
    if selected {
        Stroke::new(1.0, Color32::from_rgb(112, 152, 210))
    } else {
        Stroke::new(1.0, Color32::from_rgb(64, 72, 86))
    }
}

fn tab_hover_overlay() -> Color32 {
    Color32::from_rgba_unmultiplied(255, 255, 255, VIEWER_TAB_HOVER_OVERLAY_ALPHA)
}

fn tab_hover_stroke(stroke: Stroke) -> Stroke {
    Stroke::new(
        (stroke.width + VIEWER_TAB_HOVER_STROKE_BOOST).max(stroke.width),
        stroke.color,
    )
}

fn tab_text_color(selected: bool, available: bool) -> Color32 {
    if selected {
        Color32::from_rgb(238, 243, 252)
    } else if available {
        Color32::from_rgb(186, 194, 208)
    } else {
        Color32::from_rgb(120, 126, 138)
    }
}

fn close_text_color(selected: bool) -> Color32 {
    if selected {
        Color32::from_rgb(171, 178, 191)
    } else {
        Color32::from_rgb(120, 126, 138)
    }
}

fn close_hover_bg_color() -> Color32 {
    Color32::from_rgba_unmultiplied(255, 80, 80, VIEWER_TAB_CLOSE_HOVER_BG_ALPHA)
}

fn close_hover_text_color() -> Color32 {
    Color32::from_rgb(255, 170, 170)
}

fn close_glyph_color(selected: bool, hovered: bool) -> Color32 {
    if hovered {
        close_hover_text_color()
    } else {
        close_text_color(selected)
    }
}

fn should_focus_tab(close_clicked: bool) -> bool {
    !close_clicked
}

fn render_tab_strip_action_button(ui: &mut Ui, label: &str, width: f32) -> egui::Response {
    egui::Frame::none()
        .fill(Color32::from_rgb(30, 34, 42))
        .stroke(Stroke::new(1.0, Color32::from_rgb(56, 63, 76)))
        .rounding(egui::Rounding::same(4.0))
        .inner_margin(egui::Margin::symmetric(
            VIEWER_TAB_INNER_X,
            VIEWER_TAB_INNER_Y,
        ))
        .show(ui, |ui| {
            let label_width = (width - 2.0 * VIEWER_TAB_INNER_X).max(1.0);
            ui.add_sized(
                egui::vec2(label_width, VIEWER_TAB_ROW_HEIGHT),
                egui::Label::new(
                    RichText::new(label)
                        .color(tab_text_color(false, true))
                        .size(11.0),
                )
                .selectable(false)
                .sense(egui::Sense::click()),
            )
            .on_hover_cursor(egui::CursorIcon::PointingHand)
        })
        .inner
}

fn viewer_workspace_status(state: &crate::common::app::AppState) -> (&'static str, Color32) {
    let active = state.active_viewer();
    let capability = state.viewer_capability(active);

    if active == crate::viewers::ActiveViewer::Waveform && state.simulation.waveforms.is_empty() {
        return ("No waveform data loaded", Color32::from_rgb(220, 180, 90));
    }

    if capability.available {
        ("Data ready", Color32::from_rgb(100, 210, 120))
    } else {
        (capability.reason, Color32::from_rgb(220, 180, 90))
    }
}

impl RSpiceApp {
    /// Render the waveform panel.
    pub(super) fn render_waveform_panel(&mut self, ui: &mut Ui) {
        ui.scope(|ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            self.render_viewer_workspace_tabs(ui);
            self.render_viewer_workspace_header(ui);
        });

        match self.state.active_viewer() {
            crate::viewers::ActiveViewer::Waveform => {
                crate::waveform::render_waveform_panel(ui, &mut self.state)
            }
            crate::viewers::ActiveViewer::SmithChart => self.render_smith_panel(ui),
            crate::viewers::ActiveViewer::EyeDiagram => self.render_eye_panel(ui),
            crate::viewers::ActiveViewer::Histogram => self.render_histogram_panel(ui),
            crate::viewers::ActiveViewer::BodePlot => self.render_bode_panel(ui),
            crate::viewers::ActiveViewer::Nyquist => self.render_nyquist_panel(ui),
            crate::viewers::ActiveViewer::Fft => self.render_fft_panel(ui),
            crate::viewers::ActiveViewer::PoleZero => self.render_polezero_panel(ui),
        }
    }

    fn render_viewer_workspace_header(&mut self, ui: &mut Ui) {
        let active = self.state.active_viewer();
        let (status_text, status_color) = viewer_workspace_status(&self.state);

        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.label(
                RichText::new(active.name())
                    .strong()
                    .size(12.0)
                    .color(Color32::from_rgb(220, 225, 235)),
            );

            ui.label(RichText::new(status_text).size(10.0).color(status_color));
        });
    }
    fn render_viewer_workspace_tabs(&mut self, ui: &mut Ui) {
        use crate::viewers::ActiveViewer;

        let active_viewer = self.state.active_viewer();
        let tab_count = self.state.viewer_workspace.tab_count();
        let can_close_tabs = tab_count > 1;

        let mut focus_request: Option<ActiveViewer> = None;
        let mut close_request: Option<ActiveViewer> = None;
        let mut open_request: Option<ActiveViewer> = None;

        egui::Frame::none()
            .fill(tab_strip_fill())
            .inner_margin(egui::Margin::symmetric(
                VIEWER_TAB_STRIP_PAD_X,
                VIEWER_TAB_STRIP_PAD_Y,
            ))
            .show(ui, |ui| {
                ui.set_min_height(VIEWER_TAB_STRIP_MIN_HEIGHT);
                ui.spacing_mut().item_spacing = egui::vec2(VIEWER_TAB_SPACING, 0.0);
                ui.add_space(VIEWER_TAB_CHIP_TOP_SPACE);

                ui.horizontal(|ui| {
                    for index in 0..tab_count {
                        let Some(viewer) = self.state.viewer_workspace.tab_at(index) else {
                            continue;
                        };
                        let capability = self.state.viewer_capability(viewer);
                        let selected = viewer == active_viewer;
                        let base_fill = tab_fill(selected);
                        let base_stroke = tab_stroke(selected);

                        let mut close_rect: Option<egui::Rect> = None;
                        let tab_response = egui::Frame::none()
                            .fill(base_fill)
                            .stroke(base_stroke)
                            .rounding(VIEWER_TAB_ROUNDING)
                            .inner_margin(egui::Margin::symmetric(
                                VIEWER_TAB_INNER_X,
                                VIEWER_TAB_INNER_Y,
                            ))
                            .show(ui, |ui| {
                                ui.set_min_height(VIEWER_TAB_ROW_HEIGHT);
                                ui.with_layout(
                                    egui::Layout::left_to_right(egui::Align::Center),
                                    |ui| {
                                        let text = if selected {
                                            RichText::new(viewer.name()).strong().color(
                                                tab_text_color(selected, capability.available),
                                            )
                                        } else {
                                            RichText::new(viewer.name()).color(tab_text_color(
                                                selected,
                                                capability.available,
                                            ))
                                        };

                                        ui.add(egui::Label::new(text).selectable(false));

                                        if can_close_tabs {
                                            ui.add_space(3.0);
                                            let (close_hit_rect, _close_response) = ui
                                                .allocate_exact_size(
                                                    egui::vec2(
                                                        VIEWER_TAB_CLOSE_SIZE,
                                                        VIEWER_TAB_CLOSE_SIZE,
                                                    ),
                                                    egui::Sense::hover(),
                                                );
                                            close_rect = Some(close_hit_rect);
                                            let close_hovered =
                                                ui.rect_contains_pointer(close_hit_rect);
                                            if close_hovered {
                                                ui.painter().rect_filled(
                                                    close_hit_rect,
                                                    egui::Rounding::same(3.0),
                                                    close_hover_bg_color(),
                                                );
                                            }

                                            let close_color =
                                                close_glyph_color(selected, close_hovered);
                                            let cross_half = 3.0;
                                            let center = close_hit_rect.center();
                                            let stroke = Stroke::new(1.4, close_color);
                                            ui.painter().line_segment(
                                                [
                                                    center + egui::vec2(-cross_half, -cross_half),
                                                    center + egui::vec2(cross_half, cross_half),
                                                ],
                                                stroke,
                                            );
                                            ui.painter().line_segment(
                                                [
                                                    center + egui::vec2(cross_half, -cross_half),
                                                    center + egui::vec2(-cross_half, cross_half),
                                                ],
                                                stroke,
                                            );
                                        }
                                    },
                                );
                            });

                        let tab_id = ui.make_persistent_id(("viewer_tab_focus", viewer.id()));
                        let mut tab_focus_response = ui
                            .interact(tab_response.response.rect, tab_id, egui::Sense::click())
                            .on_hover_cursor(egui::CursorIcon::PointingHand);
                        if !capability.available {
                            tab_focus_response =
                                tab_focus_response.on_hover_text(capability.reason);
                        }

                        if tab_focus_response.clicked() {
                            let close_clicked = can_close_tabs
                                && close_rect
                                    .and_then(|rect| {
                                        tab_focus_response
                                            .interact_pointer_pos()
                                            .map(|pos| rect.contains(pos))
                                    })
                                    .unwrap_or(false);
                            if should_focus_tab(close_clicked) {
                                focus_request = Some(viewer);
                            } else {
                                close_request = Some(viewer);
                            }
                        }

                        if tab_focus_response.hovered() {
                            ui.painter().rect_filled(
                                tab_response.response.rect,
                                egui::Rounding::same(VIEWER_TAB_ROUNDING),
                                tab_hover_overlay(),
                            );
                            ui.painter().rect_stroke(
                                tab_response.response.rect,
                                egui::Rounding::same(VIEWER_TAB_ROUNDING),
                                tab_hover_stroke(base_stroke),
                            );
                        }
                    }

                    ui.add_space(8.0);
                    let add_menu_id = ui.make_persistent_id("viewer_add_menu");
                    let add_response =
                        render_tab_strip_action_button(ui, "Add Viewer", VIEWER_ADD_BUTTON_WIDTH);
                    if add_response.clicked() {
                        ui.memory_mut(|mem| mem.toggle_popup(add_menu_id));
                    }

                    egui::popup_below_widget(
                        ui,
                        add_menu_id,
                        &add_response,
                        egui::PopupCloseBehavior::CloseOnClickOutside,
                        |ui| {
                            ui.set_min_width(170.0);
                            for viewer in ActiveViewer::all() {
                                let is_open = self.state.viewer_workspace.contains(*viewer);
                                let capability = self.state.viewer_capability(*viewer);
                                let enabled = capability.available;

                                let label = if is_open {
                                    format!("[open] {}", viewer.name())
                                } else {
                                    viewer.name().to_string()
                                };

                                let mut response =
                                    ui.add_enabled(enabled, egui::Button::new(label));
                                if !enabled {
                                    response = response.on_hover_text(capability.reason);
                                }

                                if response.clicked() {
                                    open_request = Some(*viewer);
                                    ui.memory_mut(|mem| mem.close_popup());
                                }
                            }
                        },
                    );
                });

                ui.add_space(VIEWER_TAB_CHIP_BOTTOM_SPACE);
            });

        if let Some(viewer) = focus_request {
            self.state.viewer_workspace.focus(viewer);
        }

        if let Some(viewer) = close_request {
            self.state.viewer_workspace.close_viewer(viewer);
        }

        if let Some(viewer) = open_request {
            self.state.open_viewer(viewer);
        }
    }

    /// Render the structured log panel.
    pub(super) fn render_log_panel(&mut self, ui: &mut Ui) {
        let cleared = crate::panels::render_log_panel(
            ui,
            &mut self.state.log_buffer,
            &mut self.state.log_panel_state,
        );
        if cleared {
            self.state.clear_primary_log();
        }
    }

    /// Render the automation/scripting panel.
    pub(super) fn render_automation_panel(&mut self, ui: &mut Ui) {
        // Delegate to the existing script console renderer
        crate::panels::render_script_console(
            ui,
            &mut self.state.script_console,
            &mut self.state.simulation,
        );
    }

    /// Render the Bode plot panel (AC analysis magnitude/phase).
    pub(super) fn render_bode_panel(&mut self, ui: &mut Ui) {
        crate::analysis::bode::render_bode_panel(ui, &mut self.state);
    }

    /// Render the Pole-Zero map panel.
    pub(super) fn render_polezero_panel(&mut self, ui: &mut Ui) {
        crate::analysis::pole_zero::render_pz_plot(ui, &self.state.pole_zero_state);
    }

    /// Render the Nyquist panel.
    pub(super) fn render_nyquist_panel(&mut self, ui: &mut Ui) {
        crate::analysis::nyquist::render_nyquist_panel(ui, &mut self.state);
    }

    /// Render the FFT panel.
    pub(super) fn render_fft_panel(&mut self, ui: &mut Ui) {
        if let Some(state) = self.ensure_transient_viewer_ready(ui, crate::viewers::ActiveViewer::Fft)
        {
            if !matches!(state, DerivedViewerLoadState::Ready) {
                return;
            }
        }
        crate::analysis::fft::render_fft_panel(ui, &mut self.state);
        self.simulation_controller
            .mark_transient_view_ready(&self.state, crate::viewers::ActiveViewer::Fft);
    }

    /// Render the Eye diagram panel.
    pub(super) fn render_eye_panel(&mut self, ui: &mut Ui) {
        if let Some(state) =
            self.ensure_transient_viewer_ready(ui, crate::viewers::ActiveViewer::EyeDiagram)
        {
            if !matches!(state, DerivedViewerLoadState::Ready) {
                return;
            }
        }
        crate::analysis::eye_diagram::render_eye_diagram_panel(ui, &mut self.state);
        self.simulation_controller
            .mark_transient_view_ready(&self.state, crate::viewers::ActiveViewer::EyeDiagram);
    }

    /// Render the Smith chart panel.
    pub(super) fn render_smith_panel(&mut self, ui: &mut Ui) {
        crate::analysis::smith_chart::render_smith_chart(ui, &mut self.state.smith_chart_state);
    }

    /// Render the Histogram panel (Monte Carlo/corners).
    pub(super) fn render_histogram_panel(&mut self, ui: &mut Ui) {
        crate::analysis::histogram::render_histogram_panel(ui, &mut self.state);
    }

    fn ensure_transient_viewer_ready(
        &mut self,
        ui: &mut Ui,
        viewer: crate::viewers::ActiveViewer,
    ) -> Option<DerivedViewerLoadState> {
        let analysis_type = self
            .state
            .simulation
            .active_analysis()
            .map(|analysis| analysis.analysis_type)?;
        if !crate::simulation::SimulationController::analysis_supports_transient_derivation(
            analysis_type,
        ) {
            return None;
        }

        let load_state = self
            .simulation_controller
            .ensure_transient_viewer_data(&mut self.state, viewer);
        match load_state {
            DerivedViewerLoadState::Ready => Some(load_state),
            DerivedViewerLoadState::Loading => {
                render_viewer_status_placeholder(
                    ui,
                    viewer.name(),
                    "Preparing derived data from the active transient waveform...",
                );
                Some(load_state)
            }
            DerivedViewerLoadState::Unavailable => {
                render_viewer_status_placeholder(
                    ui,
                    viewer.name(),
                    "The active analysis does not contain a usable derived-view source.",
                );
                Some(load_state)
            }
        }
    }
}

fn render_viewer_status_placeholder(ui: &mut Ui, title: &str, message: &str) {
    egui::Frame::none()
        .fill(Color32::from_rgb(24, 28, 34))
        .stroke(Stroke::new(1.0, Color32::from_rgb(54, 60, 72)))
        .inner_margin(egui::Margin::symmetric(18.0, 18.0))
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(24.0);
                ui.label(
                    RichText::new(title)
                        .strong()
                        .size(14.0)
                        .color(Color32::from_rgb(220, 225, 235)),
                );
                ui.add_space(8.0);
                ui.label(
                    RichText::new(message)
                        .size(11.0)
                        .color(Color32::from_rgb(180, 185, 195)),
                );
                ui.add_space(24.0);
            });
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::viewer_style::viewer_header_bg_color;

    fn luma(color: Color32) -> f32 {
        0.2126 * color.r() as f32 + 0.7152 * color.g() as f32 + 0.0722 * color.b() as f32
    }

    #[test]
    fn viewer_tabs_have_distinct_active_surface() {
        let selected_fill = tab_fill(true);
        let unselected_fill = tab_fill(false);
        let selected_stroke = tab_stroke(true);
        let unselected_stroke = tab_stroke(false);

        assert_ne!(selected_fill, unselected_fill);
        assert!(selected_stroke.width >= 1.0);
        assert!(unselected_stroke.width >= 1.0);
    }

    #[test]
    fn tab_text_color_order_preserves_readability_priority() {
        let selected = tab_text_color(true, true);
        let available = tab_text_color(false, true);
        let unavailable = tab_text_color(false, false);

        assert!(luma(selected) > luma(available));
        assert!(luma(available) > luma(unavailable));
    }

    #[test]
    fn close_glyph_contrast_tracks_selected_state() {
        let selected_close = close_text_color(true);
        let unselected_close = close_text_color(false);
        assert!(luma(selected_close) > luma(unselected_close));
    }

    #[test]
    fn close_glyph_color_prioritizes_hover_feedback() {
        assert_eq!(close_glyph_color(false, true), close_hover_text_color(),);
        assert_eq!(close_glyph_color(true, false), close_text_color(true));
        assert_eq!(close_glyph_color(false, false), close_text_color(false));
    }

    #[test]
    fn tab_focus_is_blocked_when_close_hotspot_was_clicked() {
        assert!(should_focus_tab(false));
        assert!(!should_focus_tab(true));
    }

    #[test]
    fn strip_and_tab_colors_are_not_identical() {
        assert_ne!(tab_strip_fill(), tab_fill(true));
    }

    #[test]
    fn strip_fill_matches_viewer_header_fill() {
        assert_eq!(tab_strip_fill(), viewer_header_bg_color());
    }

    #[test]
    fn chip_height_leaves_vertical_breathing_room_in_strip() {
        assert_eq!(
            VIEWER_TAB_STRIP_MIN_HEIGHT,
            VIEWER_TAB_CHIP_OUTER_HEIGHT + VIEWER_TAB_CHIP_TOP_SPACE + VIEWER_TAB_CHIP_BOTTOM_SPACE
        );
        assert!(VIEWER_TAB_CHIP_TOP_SPACE > 0.0);
        assert!(VIEWER_TAB_CHIP_BOTTOM_SPACE > 0.0);
    }

    #[test]
    fn waveform_status_reports_no_data_when_empty() {
        let state = crate::common::app::AppState::default();
        let (text, _color) = viewer_workspace_status(&state);
        assert_eq!(text, "No waveform data loaded");
    }

    #[test]
    fn waveform_status_reports_data_ready_when_waveforms_exist() {
        let mut state = crate::common::app::AppState::default();
        state
            .simulation
            .waveforms
            .push(crate::state::WaveformData::new(
                "V(out)",
                vec![0.0, 1.0],
                vec![0.0, 1.0],
                "#00AAFF",
            ));

        let (text, _color) = viewer_workspace_status(&state);
        assert_eq!(text, "Data ready");
    }
}
