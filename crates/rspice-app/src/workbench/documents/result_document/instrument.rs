//! Active-pane authority and application effects for the Results instrument.

use super::{AppState, ResultPlotTool, Tokens, Ui, hidden_wave_strip_count, waves};
use rspice_results_ui::chrome::{
    ExportRequests,
    instrument::{
        self, InstrumentAction, InstrumentAvailability, InstrumentControls, WaveInstrumentHost,
    },
};

pub(super) fn show(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    waves::reconcile_active_pane(state, &t);
    let limits_available = waves::spec_limits_available(state, &t);
    let envelope_available = waves::family_envelope_available(state, &t);
    let marker_available = waves::marker_at_cursor_a_available(state, &t);
    if !limits_available {
        state.results.results.session.show_spec_limits = false;
    }
    if !envelope_available {
        state.results.results.session.show_family_envelope = false;
    }

    instrument::show(
        ui,
        InstrumentAvailability {
            limits_available,
            envelope_available,
            marker_available,
        },
        &mut Host { state, tokens: &t },
    );
}

struct Host<'a> {
    state: &'a mut AppState,
    tokens: &'a Tokens,
}

impl WaveInstrumentHost for Host<'_> {
    fn controls(&self) -> InstrumentControls {
        let results = &self.state.results.results;
        InstrumentControls {
            plot_tool: results.session.plot_tool,
            cursor_armed: results.session.cursor_tool.is_armed(),
            show_spec_limits: results.session.show_spec_limits,
            show_family_envelope: results.session.show_family_envelope,
            show_minor_grid: results.session.show_minor_grid,
        }
    }

    fn request(&mut self, action: InstrumentAction) {
        match action {
            InstrumentAction::Cursor => {
                let cursor_active = self.state.results.results.session.plot_tool
                    == ResultPlotTool::Cursor
                    && self.state.results.results.session.cursor_tool.is_armed();
                if cursor_active {
                    self.state.results.results.session.toggle_cursor_tool();
                } else {
                    self.state.results.results.session.plot_tool = ResultPlotTool::Cursor;
                    if !self.state.results.results.session.cursor_tool.is_armed() {
                        self.state.results.results.session.toggle_cursor_tool();
                    }
                }
            }
            InstrumentAction::Tool(tool) => self.state.results.results.session.plot_tool = tool,
            InstrumentAction::Zoom(factor) => {
                waves::zoom_active_pane(self.state, self.tokens, factor)
            }
            InstrumentAction::Fit => waves::fit_active_pane(self.state, self.tokens),
            InstrumentAction::ToggleLimits => {
                self.state.results.results.session.show_spec_limits =
                    !self.state.results.results.session.show_spec_limits
            }
            InstrumentAction::ToggleEnvelope => {
                self.state.results.results.session.show_family_envelope =
                    !self.state.results.results.session.show_family_envelope
            }
            InstrumentAction::ToggleGrid => {
                self.state.results.results.session.show_minor_grid =
                    !self.state.results.results.session.show_minor_grid
            }
            InstrumentAction::DropMarker => waves::drop_marker_at_cursor_a(self.state, self.tokens),
            InstrumentAction::RestoreStrips => {
                self.state.results.results.session.hidden_strips.clear()
            }
        }
    }

    fn export(&mut self, requests: ExportRequests) {
        super::bars::raise_exports(self.state, requests);
    }

    fn inline_cursor_readout(&mut self) -> Option<String> {
        waves::inline_cursor_readout(self.state, self.tokens)
    }
    fn hidden_strip_count(&self) -> usize {
        hidden_wave_strip_count(self.state)
    }
}
