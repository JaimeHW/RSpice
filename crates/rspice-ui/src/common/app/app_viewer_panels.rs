use egui::{Color32, Pos2, RichText, Stroke, Ui};

use super::RSpiceApp;

const VIEWER_TAB_STRIP_MIN_HEIGHT: f32 = VIEWER_TAB_ROW_HEIGHT;
const VIEWER_TAB_STRIP_PAD_X: f32 = 8.0;
const VIEWER_TAB_STRIP_PAD_Y: f32 = 0.0;
const VIEWER_TAB_SPACING: f32 = 4.0;
const VIEWER_TAB_ROW_HEIGHT: f32 = 22.0;
const VIEWER_TAB_CLOSE_SIZE: f32 = 12.0;
const VIEWER_ADD_BUTTON_WIDTH: f32 = 104.0;
const VIEWER_ADD_BUTTON_HEIGHT: f32 = 20.0;
const VIEWER_TAB_ROUNDING: f32 = 2.0;
const VIEWER_TAB_INNER_X: f32 = 9.0;
const VIEWER_TAB_INNER_Y: f32 = 3.0;
const VIEWER_TAB_UNDERLINE_HEIGHT: f32 = 2.0;

fn tab_strip_fill() -> Color32 {
    Color32::from_rgb(23, 26, 32)
}

fn tab_fill(selected: bool) -> Color32 {
    let _ = selected;
    Color32::TRANSPARENT
}

fn tab_stroke(selected: bool) -> Stroke {
    let _ = selected;
    Stroke::new(0.0, Color32::TRANSPARENT)
}

fn tab_underline_color() -> Color32 {
    Color32::from_rgb(92, 154, 245)
}

fn tab_text_color(selected: bool, available: bool) -> Color32 {
    if selected {
        Color32::from_rgb(229, 236, 247)
    } else if available {
        Color32::from_rgb(182, 189, 201)
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

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .small_button("Close Active")
                    .on_hover_text("Close the active viewer tab")
                    .clicked()
                {
                    self.state.close_active_viewer();
                }
            });
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
                ui.spacing_mut().item_spacing = egui::vec2(VIEWER_TAB_SPACING, 0.0);

                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), VIEWER_TAB_STRIP_MIN_HEIGHT),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        for index in 0..tab_count {
                            let Some(viewer) = self.state.viewer_workspace.tab_at(index) else {
                                continue;
                            };
                            let capability = self.state.viewer_capability(viewer);
                            let selected = viewer == active_viewer;

                            let tab_response = egui::Frame::none()
                                .fill(tab_fill(selected))
                                .stroke(tab_stroke(selected))
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

                                            let mut response = ui.add(
                                                egui::Label::new(text).sense(egui::Sense::click()),
                                            );
                                            if !capability.available {
                                                response =
                                                    response.on_hover_text(capability.reason);
                                            }
                                            if response.clicked() {
                                                focus_request = Some(viewer);
                                            }

                                            if can_close_tabs {
                                                ui.add_space(3.0);
                                                if selected {
                                                    if ui
                                                        .add(
                                                            egui::Button::new(
                                                                RichText::new("x")
                                                                    .size(10.0)
                                                                    .color(close_text_color(
                                                                        selected,
                                                                    )),
                                                            )
                                                            .frame(false)
                                                            .min_size(egui::vec2(
                                                                VIEWER_TAB_CLOSE_SIZE,
                                                                VIEWER_TAB_CLOSE_SIZE,
                                                            )),
                                                        )
                                                        .clicked()
                                                    {
                                                        close_request = Some(viewer);
                                                    }
                                                } else {
                                                    ui.add_space(VIEWER_TAB_CLOSE_SIZE);
                                                }
                                            }
                                        },
                                    );
                                });

                            if selected {
                                let rect = tab_response.response.rect;
                                let y = ui.max_rect().max.y - 1.0;
                                ui.painter().line_segment(
                                    [
                                        Pos2::new(rect.min.x + 5.0, y),
                                        Pos2::new(rect.max.x - 5.0, y),
                                    ],
                                    Stroke::new(VIEWER_TAB_UNDERLINE_HEIGHT, tab_underline_color()),
                                );
                            }
                        }

                        ui.add_space(8.0);
                        let add_menu_id = ui.make_persistent_id("viewer_add_menu");
                        let add_response = ui.add_sized(
                            egui::vec2(VIEWER_ADD_BUTTON_WIDTH, VIEWER_ADD_BUTTON_HEIGHT),
                            egui::Button::new(
                                RichText::new("Add Viewer")
                                    .color(tab_text_color(false, true))
                                    .size(11.0),
                            )
                            .fill(Color32::from_rgb(30, 34, 42))
                            .stroke(Stroke::new(1.0, Color32::from_rgb(56, 63, 76)))
                            .rounding(3.0),
                        );
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
                    },
                );
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
        crate::analysis::pole_zero::render_pz_plot(ui, &mut self.state.pole_zero_state);
    }

    /// Render the Nyquist panel.
    pub(super) fn render_nyquist_panel(&mut self, ui: &mut Ui) {
        crate::analysis::nyquist::render_nyquist_panel(ui, &mut self.state);
    }

    /// Render the FFT panel.
    pub(super) fn render_fft_panel(&mut self, ui: &mut Ui) {
        crate::analysis::fft::render_fft_panel(ui, &mut self.state);
    }

    /// Render the Eye diagram panel.
    pub(super) fn render_eye_panel(&mut self, ui: &mut Ui) {
        crate::analysis::eye_diagram::render_eye_diagram_panel(ui, &mut self.state);
    }

    /// Render the Smith chart panel.
    pub(super) fn render_smith_panel(&mut self, ui: &mut Ui) {
        crate::analysis::smith_chart::render_smith_chart(ui, &mut self.state.smith_chart_state);
    }

    /// Render the Histogram panel (Monte Carlo/corners).
    pub(super) fn render_histogram_panel(&mut self, ui: &mut Ui) {
        crate::analysis::histogram::render_histogram_panel(ui, &mut self.state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn luma(color: Color32) -> f32 {
        0.2126 * color.r() as f32 + 0.7152 * color.g() as f32 + 0.0722 * color.b() as f32
    }

    #[test]
    fn tab_frames_are_flat_to_avoid_button_chrome() {
        assert_eq!(tab_fill(true), Color32::TRANSPARENT);
        assert_eq!(tab_fill(false), Color32::TRANSPARENT);
        assert_eq!(tab_stroke(true).width, 0.0);
        assert_eq!(tab_stroke(false).width, 0.0);
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
    fn strip_and_tab_colors_are_not_identical() {
        assert_ne!(tab_strip_fill(), tab_fill(true));
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
