use egui::Ui;

use super::RSpiceApp;

impl RSpiceApp {
    /// Render the waveform panel.
    pub(super) fn render_waveform_panel(&mut self, ui: &mut Ui) {
        use crate::viewers::ActiveViewer;

        match self.state.active_viewer {
            ActiveViewer::Waveform => crate::waveform::render_waveform_panel(ui, &mut self.state),
            ActiveViewer::SmithChart => self.render_smith_panel(ui),
            ActiveViewer::EyeDiagram => self.render_eye_panel(ui),
            ActiveViewer::Histogram => self.render_histogram_panel(ui),
            ActiveViewer::BodePlot => self.render_bode_panel(ui),
            ActiveViewer::Nyquist => self.render_nyquist_panel(ui),
            ActiveViewer::Fft => self.render_fft_panel(ui),
            ActiveViewer::PoleZero => self.render_polezero_panel(ui),
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
