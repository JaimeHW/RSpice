//! Active-pane authority and application effects for the Results instrument.

use super::{ResultPlotTool, ResultsParts, Tokens, Ui, hidden_wave_strip_count, waves};
use rspice_results_ui::chrome::{
    ExportRequests,
    instrument::{
        self, InstrumentAction, InstrumentAvailability, InstrumentControls, StripTools,
        WaveInstrumentHost,
    },
};

pub(super) fn show(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    let t = Tokens::get(ui.ctx());
    waves::reconcile_active_pane(state, &t);
    let limits_available = waves::spec_limits_available(state, &t);
    let envelope_available = waves::family_envelope_available(state, &t);
    let marker_available = waves::marker_at_cursor_a_available(state, &t);
    if !limits_available {
        state.results.session.show_spec_limits = false;
    }
    if !envelope_available {
        state.results.session.show_family_envelope = false;
    }
    let active_strip = waves::active_strip_name(state, &t);
    // A sheet of strips carries the active strip's own tools on its bar.
    let strip_tools = super::viewer_uses_wave_stack(state.results.session.viewer)
        .then(|| waves::active_strip_tools(state, &t));

    instrument::show(
        ui,
        InstrumentAvailability {
            limits_available,
            envelope_available,
            marker_available,
        },
        &mut Host {
            state,
            tokens: &t,
            ctx: ui.ctx().clone(),
            active_strip,
            strip_tools,
        },
    );
}

/// The Plot page's tools on its band: the cursor tools, zoom and fit, over
/// the same active pane the Results instrument acts on.
pub(super) fn show_plot_tools(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    let t = Tokens::get(ui.ctx());
    waves::reconcile_active_pane(state, &t);
    let active_strip = waves::active_strip_name(state, &t);
    instrument::show_plot_tools(
        ui,
        &mut Host {
            state,
            tokens: &t,
            ctx: ui.ctx().clone(),
            active_strip,
            strip_tools: None,
        },
    );
}

/// The rule between two groups of a bar's controls.
pub(super) fn separator(ui: &mut Ui) {
    rspice_results_ui::chrome::instrument_separator(ui);
}

struct Host<'a, 'p> {
    state: &'a mut ResultsParts<'p>,
    tokens: &'a Tokens,
    ctx: egui::Context,
    active_strip: Option<String>,
    strip_tools: Option<StripTools>,
}

impl WaveInstrumentHost for Host<'_, '_> {
    fn controls(&self) -> InstrumentControls {
        let results = &self.state.results;
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
                let cursor_active = self.state.results.session.plot_tool == ResultPlotTool::Cursor
                    && self.state.results.session.cursor_tool.is_armed();
                if cursor_active {
                    self.state.results.session.toggle_cursor_tool();
                } else {
                    self.state.results.session.plot_tool = ResultPlotTool::Cursor;
                    if !self.state.results.session.cursor_tool.is_armed() {
                        self.state.results.session.toggle_cursor_tool();
                    }
                }
            }
            InstrumentAction::Tool(ResultPlotTool::HorizontalCursor) => {
                self.state.results.session.toggle_horizontal_cursor_tool()
            }
            InstrumentAction::Tool(tool) => self.state.results.session.plot_tool = tool,
            InstrumentAction::Zoom(factor) => {
                waves::zoom_active_pane(self.state, self.tokens, factor)
            }
            InstrumentAction::Fit => waves::fit_active_pane(self.state, self.tokens),
            InstrumentAction::ToggleLimits => {
                self.state.results.session.show_spec_limits =
                    !self.state.results.session.show_spec_limits
            }
            InstrumentAction::ToggleEnvelope => {
                self.state.results.session.show_family_envelope =
                    !self.state.results.session.show_family_envelope
            }
            InstrumentAction::ToggleGrid => {
                self.state.results.session.show_minor_grid =
                    !self.state.results.session.show_minor_grid
            }
            InstrumentAction::DropMarker => waves::drop_marker_at_cursor_a(self.state, self.tokens),
            InstrumentAction::RestoreStrips => self.state.results.session.hidden_strips.clear(),
            InstrumentAction::AddExpression => {
                waves::act_on_active_strip(self.state, self.tokens, waves::StripTool::AddExpression)
            }
            InstrumentAction::ToggleLogX => {
                waves::act_on_active_strip(self.state, self.tokens, waves::StripTool::LogX)
            }
            InstrumentAction::ToggleMaximize => {
                waves::act_on_active_strip(self.state, self.tokens, waves::StripTool::Maximize)
            }
        }
    }

    fn export(&mut self, requests: ExportRequests) {
        super::bars::raise_exports(&self.ctx, self.state, requests);
    }

    fn inline_cursor_readout(&mut self) -> Option<String> {
        waves::inline_cursor_readout(self.state, self.tokens)
    }
    fn sheet_controls(&mut self, ui: &mut Ui) -> bool {
        super::s_parameters::plot_menu(ui, self.state)
    }

    fn hidden_strip_count(&self) -> usize {
        hidden_wave_strip_count(self.state.results_reads())
    }
    fn active_strip(&self) -> Option<String> {
        self.active_strip.clone()
    }
    fn strip_tools(&self) -> Option<StripTools> {
        self.strip_tools.clone()
    }
}
