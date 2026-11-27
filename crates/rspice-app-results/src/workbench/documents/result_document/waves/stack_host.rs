//! Source-bound operations used by the reusable waveform stack.
use super::*;
use rspice_results_ui::{session::ResultViewerState, waves::stack::StackHost};
use std::sync::Arc;

pub(super) struct WaveStackHost<'a, 'p>(pub &'a mut ResultsParts<'p>);

impl StackHost for WaveStackHost<'_, '_> {
    fn session(&self) -> &ResultViewerState {
        &self.0.results.session
    }
    fn nothing_plotted_hint(&self) -> &'static str {
        nothing_plotted_hint(self.0)
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
    fn resolve_abscissa_expression(
        &mut self,
        model: &StripModel,
        text: &str,
    ) -> Result<rspice_results_ui::waves::xy::ResolvedAbscissa, String> {
        super::expressions::resolve_abscissa_expression(self.0, model, text)
    }
    fn expression_pending(&self, analysis: AnalysisPresentationKey, text: &str) -> bool {
        self.0
            .results
            .expression_jobs
            .contains_key(&(analysis, text.to_owned()))
    }
    fn forget_expression(&mut self, analysis: AnalysisPresentationKey, text: String) {
        let key = (analysis, text);
        if let Some(evaluating) = self.0.results.expression_jobs.remove(&key) {
            evaluating.job.cancel();
        }
        self.0.results.analysis_expr_cache.remove(&key);
    }
    fn add_expression(&mut self, model: &StripModel, text: String) {
        super::expressions::add_strip_expression(self.0, model.analysis_key, text);
    }
    fn specification_limits(
        &self,
        model: &StripModel,
        pane: &UnitPane,
        tokens: &Tokens,
    ) -> Vec<plot::LimitLine> {
        matching_spec_limits(self.0.results_reads(), model, pane, tokens)
    }
    fn margin_marks(
        &mut self,
        model: &StripModel,
        pane: &UnitPane,
        tokens: &Tokens,
    ) -> rspice_results_ui::bode::margins::MarginMarks {
        super::super::bode::margin_marks(self.0, model, pane, tokens)
    }
    fn comparison_marks(
        &mut self,
        model: &StripModel,
        pane: &UnitPane,
        expressions: &[ResolvedExpr],
        tokens: &Tokens,
    ) -> rspice_results_ui::waves::pane::ComparisonMarks {
        super::super::document_edit::compare::comparison_marks(
            self.0,
            model,
            pane,
            expressions,
            tokens,
        )
    }
    fn pane_header(
        &mut self,
        ui: &mut Ui,
        model: &StripModel,
        pane: &UnitPane,
        input: pane_header::PaneHeader,
    ) -> pane_header::UnitPaneHeaderResponse {
        show_unit_pane_header(ui, self.0, model, pane, input)
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
            samples: &trace.x,
        };
        if let Some(selector) = super::super::place_marker(self.0, placement) {
            marker_dialog::open(self.0, selector);
        }
    }
    fn note_sample_reads(&mut self, overview: usize, extrema: usize) {
        frame_work::note_samples(FrameSampleRead::StripOverview, overview);
        frame_work::note_samples(FrameSampleRead::TraceExtremes, extrema);
    }
    fn refuse(&mut self, reason: String) {
        self.0.prompt(reason);
    }
    fn offers_notes(&self, model: &StripModel) -> bool {
        self.0
            .results
            .session
            .pane_for(model.analysis_key)
            .is_some()
    }
    fn edit_note(&mut self, _model: &StripModel, edit: rspice_results_ui::waves::notes::NoteEdit) {
        super::super::notes::edit_note(self.0, edit);
    }
    fn send_to_calculator(&mut self, model: &StripModel, trace: &StripTrace) {
        run_on_trace(
            self.0,
            model,
            trace,
            crate::workbench::results::commands::ResultsCommand::SendTraceToCalculator,
        );
    }
    fn open_in_results(&mut self, model: &StripModel) {
        super::super::plot_page::open_in_results(self.0, Some(model.analysis_key));
    }
    fn event_lane_height(&self, model: &StripModel) -> f32 {
        super::lanes::lane_height(self.0, model)
    }
    fn show_event_lanes(
        &mut self,
        ui: &mut Ui,
        model: &StripModel,
        frame: rspice_results_ui::waves::stack::EventLaneFrame,
    ) {
        super::lanes::show_lanes(ui, self.0, model, frame);
    }
    fn event_marker_height(&self, model: &StripModel) -> f32 {
        super::event_markers::marker_height(model)
    }
    fn show_event_markers(
        &mut self,
        ui: &mut Ui,
        model: &StripModel,
        frame: rspice_results_ui::waves::stack::EventLaneFrame,
    ) {
        super::event_markers::show_markers(ui, self.0, model, frame);
    }
    fn real_step_traces(
        &mut self,
        model: &StripModel,
        pane: &UnitPane,
        _tokens: &Tokens,
        view: (f64, f64),
    ) -> Vec<ResolvedExpr> {
        rspice_results_ui::waves::lanes::real_step_traces(model, pane, view)
    }
}

fn show_unit_pane_header(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    model: &StripModel,
    pane: &UnitPane,
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
            let is = |selected: &SelectedResultTrace| {
                selected.analysis_key() == self.model.analysis_key
                    && selected.source_name() == trace.source_waveform_name
            };
            self.state
                .results
                .valid_selected_trace(&self.state.model.simulation)
                .is_some_and(|selected| {
                    is(selected)
                        || self
                            .state
                            .results
                            .session
                            .also_selected_traces
                            .iter()
                            .any(is)
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
            // A form waiting for a trace takes a legend entry as it takes a
            // curve clicked in the plot.
            self.state.results.session.trace_pick.take(trace);
            self.state.results.session.selected_trace = Some(SelectedResultTrace::from_identity(
                self.model.analysis_key,
                trace.source_waveform_name.clone(),
            ));
            self.state.results.session.also_selected_traces.clear();
            self.activate_pane();
        }

        fn extend_selection(&mut self, trace: &StripTrace) {
            let picked = SelectedResultTrace::from_identity(
                self.model.analysis_key,
                trace.source_waveform_name.clone(),
            );
            let session = &mut self.state.results.session;
            // The traces styled together are of one strip.
            match session.selected_trace.as_ref() {
                Some(selected) if selected.analysis_key() == self.model.analysis_key => {
                    if *selected != picked {
                        let also = &mut session.also_selected_traces;
                        if let Some(at) = also.iter().position(|held| *held == picked) {
                            also.remove(at);
                        } else {
                            also.push(picked);
                        }
                    }
                    self.activate_pane();
                }
                _ => self.select_trace(trace),
            }
        }

        fn activate_pane(&mut self) {
            self.state.results.session.active_wave_pane = Some(self.pane_key.clone());
        }

        fn add_expression(&mut self, text: String) {
            super::expressions::add_strip_expression(self.state, self.model.analysis_key, text);
        }

        fn offers_trace_commands(&self) -> bool {
            true
        }

        fn offers_pane_menu(&self) -> bool {
            self.state.results.session.persistent_pane_context.is_some()
        }

        fn pane_menu(&mut self, ui: &mut Ui) {
            let Some(context) = self.state.results.session.persistent_pane_context else {
                return;
            };
            let menu = super::super::document_edit::pane_menu(
                self.state.results_reads(),
                context.document_id,
                context.pane_id,
            );
            if let Some(action) = rspice_results_ui::chrome::persistent::pane_menu_entries(ui, menu)
            {
                self.state.results.session.pane_menu_choice = Some(action);
            }
        }

        fn trace_command(&mut self, trace: &StripTrace, command: pane_header::TraceCommand) {
            use crate::workbench::results::commands::ResultsCommand;
            if let pane_header::TraceCommand::Style(style) = command {
                // A trace among the selected ones styles them all.
                let mut signals = vec![trace.source_waveform_name.clone()];
                if self.trace_selected(trace) {
                    let session = &self.state.results.session;
                    signals.extend(
                        session
                            .selected_trace
                            .iter()
                            .chain(&session.also_selected_traces)
                            .filter(|selected| selected.analysis_key() == self.model.analysis_key)
                            .map(|selected| selected.source_name().to_owned()),
                    );
                    signals.sort();
                    signals.dedup();
                }
                // Each takes the part the menu changed and keeps its own.
                for signal in signals {
                    let own = self
                        .state
                        .results
                        .session
                        .waveform_looks
                        .get(&SourceWaveformPresentationKey::new(
                            self.model.analysis_key,
                            signal.clone(),
                        ))
                        .map(|look| look.style)
                        .unwrap_or_default();
                    if let Err(error) = super::super::set_trace_style(
                        self.state,
                        self.model.analysis_key,
                        &signal,
                        own.with_changes(trace.style, style),
                    ) {
                        self.state.prompt(error);
                    }
                }
                return;
            }
            let command = match command {
                pane_header::TraceCommand::Measure => ResultsCommand::MeasurementLibrary,
                pane_header::TraceCommand::Spectrum => ResultsCommand::TraceSpectrum,
                pane_header::TraceCommand::SendToCalculator => {
                    ResultsCommand::SendTraceToCalculator
                }
                pane_header::TraceCommand::Style(_) => return,
            };
            run_on_trace(self.state, self.model, trace, command);
        }
    }
    pane_header::show_header(
        ui,
        model,
        pane,
        input,
        |name| notations.display(name),
        &mut HeaderHost {
            state,
            model,
            pane_key,
        },
    )
}

/// Select `trace` and run `command`, which acts on the selected trace, once
/// the frame is drawn.
fn run_on_trace(
    state: &mut ResultsParts<'_>,
    model: &StripModel,
    trace: &StripTrace,
    command: crate::workbench::results::commands::ResultsCommand,
) {
    state.results.session.selected_trace = Some(SelectedResultTrace::from_identity(
        model.analysis_key,
        trace.source_waveform_name.clone(),
    ));
    state
        .workbench
        .outbox
        .push(crate::workbench::framework::feature::Execute(command.key()));
}
