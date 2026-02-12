use egui::{RichText, Ui};

use super::RSpiceApp;

impl RSpiceApp {
    /// Render the waveform panel.
    pub(super) fn render_waveform_panel(&mut self, ui: &mut Ui) {
        self.render_viewer_workspace_tabs(ui);

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

    fn render_viewer_workspace_tabs(&mut self, ui: &mut Ui) {
        use crate::viewers::ActiveViewer;

        let tabs = self.state.viewer_workspace.tabs().to_vec();
        let active_viewer = self.state.active_viewer();
        let can_close_tabs = tabs.len() > 1;

        let mut focus_request: Option<ActiveViewer> = None;
        let mut close_request: Option<ActiveViewer> = None;
        let mut open_request: Option<ActiveViewer> = None;

        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;

            for viewer in tabs {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        let selected = viewer == active_viewer;
                        let text = if selected {
                            RichText::new(viewer.name()).strong()
                        } else {
                            RichText::new(viewer.name())
                        };

                        if ui.selectable_label(selected, text).clicked() {
                            focus_request = Some(viewer);
                        }

                        if can_close_tabs && ui.small_button("x").clicked() {
                            close_request = Some(viewer);
                        }
                    });
                });
            }

            ui.add_space(8.0);
            ui.menu_button("Add Viewer", |ui| {
                for viewer in ActiveViewer::all() {
                    let is_open = self.state.viewer_workspace.contains(*viewer);
                    let label = if is_open {
                        format!("[open] {}", viewer.name())
                    } else {
                        viewer.name().to_string()
                    };

                    if ui.button(label).clicked() {
                        open_request = Some(*viewer);
                        ui.close_menu();
                    }
                }
            });
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

        ui.separator();
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
