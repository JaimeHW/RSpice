//! Studio section routing, retained-source views, and application effects.

use super::*;

use rspice_results_ui::studio::sections::{
    self as presentation, AxisSource, FamilyRow, MeasurementsView, SectionAction, SectionHost,
};

struct Host<'a>(&'a mut RSpiceApp);

impl SectionHost for Host<'_> {
    fn panes(&self) -> &[VisualizationPane] {
        &self.0.state.workbench.visualization_studio.panes
    }
    fn axes(&self) -> AxisSource<'_> {
        let Some(analysis) = self.0.state.simulation.active_analysis() else {
            return AxisSource::MissingAnalysis;
        };
        let Some(waveform) = analysis.waveforms.iter().find(|waveform| waveform.visible) else {
            return AxisSource::NoVisibleWaveform;
        };
        AxisSource::Waveform {
            name: &waveform.name,
            x_range: waveform.x_range(),
            y_range: waveform.y_range(),
            frequency: matches!(
                analysis.analysis_type,
                AnalysisType::Ac | AnalysisType::Noise
            ),
            complex: waveform.complex.is_some(),
        }
    }
    fn autoscale(&mut self) -> &mut VisualizationAutoscale {
        &mut self.0.state.workbench.visualization_studio.autoscale
    }
    fn autoscale_available(&self, value: VisualizationAutoscale) -> bool {
        match value {
            VisualizationAutoscale::RobustVisible => true,
            VisualizationAutoscale::ExactExtrema => {
                self.0.state.ui.results.session.viewer == ResultViewer::Waves
            }
            VisualizationAutoscale::SpecificationBounds => {
                specification_bound_fit(&self.0.state).is_some()
            }
        }
    }
    fn complex_projection(&mut self) -> &mut ComplexProjection {
        &mut self
            .0
            .state
            .workbench
            .visualization_studio
            .complex_projection
    }
    fn fit_block_reason(&self) -> Option<&'static str> {
        fit_block_reason(&self.0.state)
    }
    fn families(&self) -> Vec<FamilyRow> {
        let app = &self.0;
        let active_dataset = app.state.simulation.active_run().map(|run| run.dataset_id);
        app.state
            .simulation
            .retained
            .runs
            .iter()
            .map(|run| {
                let samples = run
                    .analyses
                    .iter()
                    .flat_map(|analysis| &analysis.waveforms)
                    .map(|waveform| waveform.x.len().min(waveform.y.len()))
                    .sum::<usize>();
                FamilyRow {
                    dataset_id: run.dataset_id,
                    label: run.label.clone(),
                    analyses: run.analyses.len(),
                    samples,
                    active: Some(run.dataset_id) == active_dataset,
                    overlaid: app.state.simulation.is_dataset_overlaid(run.dataset_id),
                }
            })
            .collect()
    }
    fn measurements(&self) -> MeasurementsView<'_> {
        let state = &self.0.state;
        let expressions = active_analysis_expressions(state);
        let studio = &state.workbench.visualization_studio;
        MeasurementsView {
            measurements: &studio.measurements,
            expressions,
            cursor_a: state.ui.results.session.cursors.a.is_some(),
            cursor_b: state.ui.results.session.cursors.b.is_some(),
            linked_cursors: state.ui.results.session.linked_cursors,
            markers: &studio.markers,
            annotations: &studio.annotations,
        }
    }
    fn display_lod(&mut self) -> &mut DisplayLodPolicy {
        &mut self.0.state.workbench.visualization_studio.display_lod
    }
    fn tile_memory_mib(&mut self) -> &mut u32 {
        &mut self.0.state.workbench.visualization_studio.tile_memory_mib
    }
    fn exact_export_available(&self) -> bool {
        active_studio_exact_export_available(&self.0.state)
    }
    fn figure_export_available(&self) -> bool {
        active_studio_figure_export_available(&self.0.state)
    }
    fn request(&mut self, action: SectionAction) {
        let app = &mut self.0;
        match action {
            SectionAction::OpenDock(dock) => open_dock(app, dock),
            SectionAction::ToggleOverlay(dataset_id) => {
                app.state.simulation.toggle_dataset_overlay(dataset_id);
            }
            SectionAction::Fit => fit_active_view(app),
            SectionAction::ApplyLod => apply_lod_policy(app),
            SectionAction::ExportData => app.state.ui.export_csv_requested = true,
            SectionAction::ExportFigure => app.state.ui.export_figure_requested = true,
        }
    }
}

pub(super) fn show_active_section(ui: &mut Ui, app: &mut RSpiceApp, compact: bool) {
    if compact {
        match app.state.workbench.visualization_studio.touch_pane {
            VisualizationTouchPane::Sections => return,
            VisualizationTouchPane::Inspector => {
                viewer_inspector(ui, app, true);
                return;
            }
            VisualizationTouchPane::Actions => {
                actions_sheet(ui, app);
                return;
            }
            VisualizationTouchPane::Stage => {}
        }
    }

    match app.state.workbench.visualization_studio.section {
        VisualizationSection::Document => presentation::document_section(ui, &mut Host(app)),
        VisualizationSection::Viewers => viewers_section(ui, app, compact),
        VisualizationSection::Axes => presentation::axes_section(ui, &mut Host(app)),
        VisualizationSection::Families => presentation::families_section(ui, &mut Host(app)),
        VisualizationSection::Measurements => {
            presentation::measurements_section(ui, &mut Host(app))
        }
        VisualizationSection::LargeData => presentation::large_data_section(ui, &mut Host(app)),
        VisualizationSection::ExportReport => presentation::export_section(ui, &mut Host(app)),
    }
}

/// Whether the toolbar's `+` and `−` can act on the sheet in the stage.
///
/// Magnification is not a waveform-coordinate question. Every unit-pane sheet
/// carries the retained extents a zoom step is computed against, which is why
/// `Command::ZoomIn` is offered on all of them — so this asks the command's
/// own gate rather than a second, narrower one that greyed the buttons on
/// three sheets that answer the gesture.
pub(super) fn magnification_available(state: &AppState) -> bool {
    result_document::zoom_gesture_available(state)
}

/// The toolbar's magnification readout.
///
/// A function of the viewport and nothing else. The readout used to print
/// `visualization_studio.zoom`, a studio-local float multiplied by 1.25 and
/// 0.8 as the buttons were pressed: it reported 125% for a sheet whose
/// viewport had not moved, kept reporting it after a fit released the
/// viewport, and reported 100% for a sheet the reader had zoomed with the
/// mouse. The unit is the status bar's, from the status bar's own formatter,
/// so one product does not state the same viewport two ways.
pub(super) fn magnification_readout(status: Option<&result_document::SharedXStatus>) -> String {
    status.map_or_else(
        || "—".to_owned(),
        |status| crate::workbench::chrome::status_bar::format_plot_zoom(status.zoom),
    )
}

/// The expression traces bound to the active analysis.
///
/// `ResultsState::exprs` is the ordinal compatibility projection, and only
/// the waveform sheet's own build pass reconciles it — so read on this
/// surface it can be a projection of a previous run's analysis order, or
/// empty because no waveform sheet has been drawn yet. `analysis_exprs` is
/// where the stable state lives, and it is keyed by the identity the active
/// analysis actually has.
pub(super) fn active_analysis_expressions(state: &AppState) -> &[result_document::ExprTrace] {
    state
        .simulation
        .active_run()
        .zip(state.simulation.view.active_analysis_idx)
        .and_then(|(run, index)| Some((run.dataset_id, run.analyses.get(index)?)))
        .and_then(|(dataset_id, analysis)| {
            state.ui.results.session.analysis_exprs.get(
                &result_document::AnalysisPresentationKey::new(dataset_id, analysis),
            )
        })
        .map_or(&[][..], Vec::as_slice)
}

pub(super) fn resolved_viewer_availability(
    state: &AppState,
    definition: &ViewerDocumentDefinition,
    capabilities: ViewerCapabilities<'_>,
) -> Result<ResultViewer, String> {
    // Answered first, because it answers a different question. Everything below
    // says the retained dataset cannot feed this view, and names what would:
    // run that analysis, retain that capability, and the row lights up. A view
    // no sheet draws never lights up for any dataset, so telling the reader to
    // go and produce photonics data would send them after something that could
    // not help. The manifest's own release scope says which it is — planned,
    // preview, deferred, or owned by an external producer.
    let Some(viewer) = ResultViewer::from_viewer_document_id(definition.id) else {
        return Err(definition.unavailable_reason().into_owned());
    };
    if definition.release != crate::results::viewer_catalog::ViewerReleaseClass::ReleaseTarget {
        return Err(definition.unavailable_reason().into_owned());
    }
    match viewer_compatibility(definition.id, capabilities) {
        ViewerCompatibility::Compatible => {}
        ViewerCompatibility::MissingAnalysis {
            accepted_analysis_ids,
        } => {
            return Err(format!(
                "Requires {} analysis data",
                accepted_analysis_ids.join(" / ")
            ));
        }
        ViewerCompatibility::MissingExternalCapability { capability_id } => {
            return Err(format!("Requires {capability_id} result capability"));
        }
        ViewerCompatibility::UnknownDocument => {
            return Err("Viewer identity is not registered".to_owned());
        }
    }
    if !result_document::viewer_is_available(state, viewer) {
        return Err(result_document::viewer_unavailability_reason(state, viewer)
            .unwrap_or("The retained result does not satisfy this viewer contract")
            .to_owned());
    }
    Ok(viewer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisResult, AnalysisType, SimulationRun, WaveformData};

    fn retained_transient_state() -> AppState {
        let mut state = AppState::default();
        let mut run = SimulationRun::new(1);
        run.add_analysis(
            AnalysisResult::new(17, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("V(out)", vec![0.0, 0.5, 1.0], vec![-1.25, 2.5, 4.0], "#0af"),
            ]),
        );
        state.simulation.retained.runs = vec![run].into();
        assert!(state.simulation.select_run(0));
        assert!(state.simulation.select_analysis(0));
        state
    }

    /// The toolbar's magnification buttons follow the command's own gate.
    ///
    /// They were enabled for the waveform sheet alone, while `Command::ZoomIn`
    /// is offered — and carried out — on every unit-pane sheet. A reader on
    /// the DC, Bode or ordinary-noise sheet saw two greyed buttons for a
    /// gesture the product performs.
    #[test]
    fn the_toolbar_offers_magnification_wherever_the_command_does() {
        let mut state = retained_transient_state();
        for viewer in [
            ResultViewer::Waves,
            ResultViewer::DcSweep,
            ResultViewer::Bode,
            ResultViewer::NoiseContrib,
        ] {
            state.ui.results.session.viewer = viewer;
            assert!(
                result_document::zoom_gesture_available(&state),
                "{viewer:?} answers the zoom gesture"
            );
            assert!(
                magnification_available(&state),
                "{viewer:?} must offer the toolbar's magnification buttons"
            );
        }
        // A sheet that owns its own canvas is still not offered the unit-pane
        // gesture, so this is not a blanket enable.
        state.ui.results.session.viewer = ResultViewer::Smith;
        assert!(!magnification_available(&state));
    }

    /// The readout states the viewport, in the status bar's unit.
    ///
    /// It printed `visualization_studio.zoom * 100` as a percentage — a
    /// counter of button presses, not a fact about the sheet. It is now a
    /// function of the shared-X status alone, which is why the studio's own
    /// accumulator cannot appear in its signature.
    #[test]
    fn the_magnification_readout_reports_the_viewport_not_the_button_count() {
        use result_document::SharedXStatus;

        assert_eq!(magnification_readout(None), "—");
        assert_eq!(
            magnification_readout(Some(&SharedXStatus {
                span: "0 s … 1 s".to_owned(),
                zoom: 1.0,
            })),
            "1×"
        );
        assert_eq!(
            magnification_readout(Some(&SharedXStatus {
                span: "0 s … 250 ms".to_owned(),
                zoom: 4.0,
            })),
            "4×"
        );
        assert_eq!(
            magnification_readout(Some(&SharedXStatus {
                span: "0 s … 8 ms".to_owned(),
                zoom: 125.0,
            })),
            "125×"
        );
    }

    /// The Measurements section reads the stable expression state.
    ///
    /// It read `ResultsState::exprs`, the ordinal compatibility projection
    /// that only the waveform sheet's build pass reconciles — so opening the
    /// section without having drawn that sheet listed nothing at all.
    #[test]
    fn the_measurements_section_reads_expressions_the_document_actually_holds() {
        let mut state = retained_transient_state();
        let key = {
            let run = state.simulation.active_run().expect("active run");
            let analysis = state.simulation.active_analysis().expect("active analysis");
            result_document::AnalysisPresentationKey::new(run.dataset_id, analysis)
        };

        assert!(active_analysis_expressions(&state).is_empty());

        state.ui.results.session.analysis_exprs.insert(
            key,
            vec![result_document::ExprTrace {
                text: "V(out)*2".to_owned(),
                complex_policy: crate::state::ComplexExpressionPolicy::Rectangular,
                visible: true,
            }],
        );
        // The ordinal projection is deliberately left empty: that is the
        // state the section is opened in when no waveform sheet has run.
        assert!(state.ui.results.session.exprs.is_empty());

        let expressions = active_analysis_expressions(&state);
        assert_eq!(expressions.len(), 1);
        assert_eq!(expressions[0].text, "V(out)*2");
    }
}
