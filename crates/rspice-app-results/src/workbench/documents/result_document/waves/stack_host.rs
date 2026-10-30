//! Source-bound operations used by the reusable waveform stack.
use super::*;
use rspice_results_ui::{session::ResultViewerState, waves::stack::StackHost};
use std::sync::Arc;

pub(super) struct WaveStackHost<'a, 'p>(pub &'a mut ResultsParts<'p>);

impl StackHost for WaveStackHost<'_, '_> {
    fn session(&self) -> &ResultViewerState {
        &self.0.results.session
    }
    fn session_mut(&mut self) -> &mut ResultViewerState {
        &mut self.0.results.session
    }
    fn expression_is_complex(&self, analysis: AnalysisPresentationKey, text: &str) -> bool {
        self.0
            .results
            .analysis_expr_cache
            .get(&(analysis, text.to_owned()))
            .is_some_and(|cached| {
                cached.series.as_ref().is_ok_and(|outputs| {
                    outputs
                        .iter()
                        .any(|output| output.waveform.complex.is_some())
                })
            })
    }
    fn edit_expression(&mut self, ui: &mut Ui, model: &StripModel) {
        expr_editor_row(ui, self.0, model.analysis_key, model.analysis_index);
    }
    fn resolve_expressions(&mut self, model: &StripModel, tokens: &Tokens) -> Vec<ResolvedExpr> {
        resolve_strip_exprs(self.0, model, tokens)
    }
    fn forget_expression(&mut self, analysis: AnalysisPresentationKey, text: String) {
        self.0.results.analysis_expr_cache.remove(&(analysis, text));
    }
    fn specification_limits(
        &self,
        model: &StripModel,
        pane: &UnitPane,
        tokens: &Tokens,
    ) -> Vec<plot::LimitLine> {
        matching_spec_limits(self.0.results_reads(), model, pane, tokens)
    }
    fn pane_header(
        &mut self,
        ui: &mut Ui,
        model: &StripModel,
        pane: &UnitPane,
        ordinal: usize,
        input: pane_header::PaneHeader,
    ) -> pane_header::UnitPaneHeaderResponse {
        show_unit_pane_header(ui, self.0, model, pane, ordinal, input)
    }
    fn family_envelopes(&mut self, model: &StripModel, pane: &UnitPane) -> Arc<FamilyEnvelopePlan> {
        let generation = self.0.results.models.generation();
        extent::family_envelopes(&mut self.0.results.plans.envelopes, generation, model, pane)
    }
    fn place_marker(&mut self, model: &StripModel, trace: &StripTrace, x: f64) {
        let placement = super::super::MarkerPlacement {
            analysis: model.analysis_key,
            anchor: anchor_key(model, trace),
            trace_name: trace.name.clone(),
            x,
            samples: trace.x.as_slice(),
        };
        if let Some(selector) = super::super::place_marker(self.0, placement) {
            marker_dialog::open(self.0, selector);
        }
    }
    fn note_sample_reads(&mut self, overview: usize, extrema: usize) {
        frame_work::note_samples(FrameSampleRead::StripOverview, overview);
        frame_work::note_samples(FrameSampleRead::TraceExtremes, extrema);
    }
}

fn show_unit_pane_header(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    model: &StripModel,
    pane: &UnitPane,
    ordinal: usize,
    input: pane_header::PaneHeader,
) -> pane_header::UnitPaneHeaderResponse {
    // Design notation changes labels only; retained source names remain identities.
    let notations = bus_notations(&state.model.workspace, &state.model.schematic);
    let pane_key = WavePanePresentationKey {
        analysis: model.analysis_key,
        unit: pane.unit.to_owned(),
    };
    struct HeaderHost<'a, 'p> {
        state: &'a mut ResultsParts<'p>,
        model: &'a StripModel,
        pane_key: WavePanePresentationKey,
    }
    impl pane_header::PaneHeaderHost for HeaderHost<'_, '_> {
        fn trace_selected(&self, trace: &StripTrace) -> bool {
            self.state
                .results
                .valid_selected_trace(&self.state.model.simulation)
                .is_some_and(|selected| {
                    selected.analysis_key() == self.model.analysis_key
                        && selected.source_name() == trace.source_waveform_name
                })
        }

        fn toggle_trace_visibility(&mut self, trace: &StripTrace) {
            if let Some(key) = trace.family_visibility_key {
                self.state.results.toggle_family_trace_visibility(key);
            } else {
                toggle_visibility(self.state, self.model.analysis_index, trace.waveform_index);
            }
        }

        fn select_trace(&mut self, trace: &StripTrace) {
            self.state.results.session.selected_trace = Some(SelectedResultTrace::from_identity(
                self.model.analysis_key,
                trace.source_waveform_name.clone(),
            ));
            self.activate_pane();
        }

        fn activate_pane(&mut self) {
            self.state.results.session.active_wave_pane = Some(self.pane_key.clone());
        }
    }
    pane_header::show_header(
        ui,
        model,
        pane,
        ordinal,
        input,
        |name| notations.display(name),
        &mut HeaderHost {
            state,
            model,
            pane_key,
        },
    )
}
