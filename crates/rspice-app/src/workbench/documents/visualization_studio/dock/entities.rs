//! Source-qualified dock drafts and immediate result-document effects.

use super::*;
use rspice_results_ui::studio::dock::entities::{
    self as presentation, AnnotationAnchor, AnnotationHost, CursorHost, ExportHost,
    MeasurementHost, TraceBinding, TraceHost, TraceRows,
};

struct Host<'a>(&'a mut RSpiceApp);

pub(super) fn trace_manager_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    presentation::traces(ui, &mut Host(app))
}
pub(super) fn cursor_manager_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    presentation::cursors(ui, &mut Host(app))
}
pub(super) fn measurement_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    presentation::measurement(ui, &mut Host(app))
}
pub(super) fn annotation_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    presentation::annotation(ui, &mut Host(app))
}
pub(super) fn export_dock(ui: &mut Ui, app: &mut RSpiceApp) -> bool {
    presentation::export(ui, &mut Host(app))
}

impl TraceHost for Host<'_> {
    fn binding(&self) -> TraceBinding {
        let app = &self.0;
        let dataset_id = app.state.workbench.visualization_studio.draft_trace_dataset;
        let analysis_id = app
            .state
            .workbench
            .visualization_studio
            .draft_trace_analysis;
        let binding_exists = dataset_id
            .zip(analysis_id)
            .is_some_and(|(dataset, analysis)| {
                app.state.simulation.runs.iter().any(|run| {
                    run.dataset_id == dataset
                        && run
                            .analyses
                            .iter()
                            .any(|candidate| candidate.id == analysis)
                })
            });
        TraceBinding {
            dataset_id,
            analysis_id,
            exists: binding_exists,
        }
    }
    fn rows(&mut self) -> TraceRows<'_> {
        let app = &mut self.0;
        let labels = if app
            .state
            .workbench
            .visualization_studio
            .draft_trace_visibility
            .is_empty()
        {
            Vec::new()
        } else {
            traces::row_labels(&app.state)
        };
        TraceRows {
            visibility: &mut app
                .state
                .workbench
                .visualization_studio
                .draft_trace_visibility,
            labels,
        }
    }
    fn open_measurement(&mut self) {
        open_dock(self.0, VisualizationDock::Measurement);
    }
    fn apply(&mut self, binding: TraceBinding) {
        apply_trace_changes(self.0, binding.dataset_id, binding.analysis_id);
    }
}

fn apply_trace_changes(
    app: &mut RSpiceApp,
    dataset_id: Option<DatasetId>,
    analysis_id: Option<u64>,
) {
    let visibility = app
        .state
        .workbench
        .visualization_studio
        .draft_trace_visibility
        .clone();
    if let Some(document_id) = active_project_visualization_document_id(&app.state) {
        let active_pane = app.state.workbench.visualization_studio.active_pane;
        let project = &app.state.workspace.content;
        let edits = project
            .visualization_document(document_id)
            .map(|document| {
                document
                    .traces()
                    .iter()
                    .filter(|trace| active_pane == Some(trace.pane_id.get()))
                    .filter_map(|trace| {
                        visibility
                            .iter()
                            .find(|(name, _)| name == &trace.label)
                            .filter(|(_, visible)| *visible != trace.visible)
                            .map(|(_, visible)| DocumentEdit::SetTraceVisibility {
                                trace_id: trace.id,
                                visible: *visible,
                            })
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if !edits.is_empty() {
            match transact_active_project_document(app, edits) {
                Ok(_) => reconcile_document(app),
                Err(error) => app.state.push_user_message(ConsoleMessage::error(error)),
            }
        }
        return;
    }
    if let Some((dataset_id, analysis_id)) = dataset_id.zip(analysis_id)
        && let Some(analysis) = app
            .state
            .simulation
            .runs
            .iter_mut()
            .find(|run| run.dataset_id == dataset_id)
            .and_then(|run| {
                run.analyses
                    .iter_mut()
                    .find(|analysis| analysis.id == analysis_id)
            })
    {
        for waveform in &mut analysis.waveforms {
            if let Some((_, visible)) = visibility.iter().find(|(name, _)| name == &waveform.name) {
                waveform.visible = *visible;
            }
        }
        commit_visualization_revision(app);
    }
}

impl CursorHost for Host<'_> {
    fn canonical(&self) -> bool {
        active_project_visualization_document_id(&self.0.state).is_some()
    }
    fn linked(&self) -> bool {
        self.0.state.ui.results.session.linked_cursors
    }
    fn set_linked(&mut self, canonical: bool, linked_cursors: bool) {
        let app = &mut self.0;

        if canonical {
            set_active_project_cursor_links(app, linked_cursors);
        } else {
            app.state.ui.results.session.linked_cursors = linked_cursors;
        }
    }
    fn positions(&self) -> (Option<f64>, Option<f64>) {
        (
            self.0.state.ui.results.session.cursors.a,
            self.0.state.ui.results.session.cursors.b,
        )
    }
    fn place_cursor(&mut self) {
        add_cursor_at_midpoint(self.0);
    }
    fn place_marker(&mut self) {
        add_marker_at_midpoint(self.0);
    }
    fn clear_cursors(&mut self, canonical: bool) {
        let app = &mut self.0;

        if canonical {
            if let Some(pane_id) = app.state.workbench.visualization_studio.active_pane {
                commit_active_project_cursor_pair(app, pane_id, (None, None));
            }
        } else {
            app.state.ui.results.session.clear_cursors();
        }
    }
    fn clear_markers(&mut self) {
        let app = &mut self.0;

        if let Some(document_id) = active_project_visualization_document_id(&app.state) {
            let has_markers = app
                .state
                .workspace
                .content
                .visualization_document(document_id)
                .is_some_and(|document| !document.markers().is_empty());
            if has_markers {
                match transact_active_project_document(
                    app,
                    vec![DocumentEdit::ClearMarkers { pane_id: None }],
                ) {
                    Ok(_) => reconcile_document(app),
                    Err(error) => {
                        app.state.push_user_message(ConsoleMessage::error(error));
                    }
                }
            }
        } else {
            let result = app.state.workbench.visualization_studio.transact(|studio| {
                studio.markers.clear();
                Ok(())
            });
            report_visualization_commit(app, result);
        }
    }
}

impl MeasurementHost for Host<'_> {
    fn expression(&mut self) -> &mut String {
        &mut self
            .0
            .state
            .workbench
            .visualization_studio
            .draft_measurement
    }
    fn evaluate(&self, definition: &str) -> Result<(DatasetId, u64, f64), String> {
        evaluate_scalar_measurement(&self.0.state, definition)
    }
    fn create(&mut self, measurement: (DatasetId, u64, f64), expression: String) {
        create_measurement(self.0, measurement, expression);
    }
}

fn create_measurement(
    app: &mut RSpiceApp,
    (dataset_id, analysis_sequence, value): (DatasetId, u64, f64),
    expression: String,
) {
    if active_project_visualization_document_id(&app.state).is_some() {
        let context = canonical_measurement_trace_ids(&app.state, &expression);
        match context.and_then(|(pane_id, trace_ids)| {
            transact_active_project_document(
                app,
                vec![DocumentEdit::AddScalarMeasurement {
                    pane_id,
                    trace_ids,
                    expression,
                    value,
                }],
            )
        }) {
            Ok(_) => {
                app.state
                    .workbench
                    .visualization_studio
                    .draft_measurement
                    .clear();
                reconcile_document(app);
            }
            Err(error) => app.state.push_user_message(ConsoleMessage::error(error)),
        }
        return;
    }
    let result = app.state.workbench.visualization_studio.transact(|studio| {
        let id = studio
            .allocate_identity()
            .ok_or_else(|| "Visualization measurement identity space is exhausted".to_owned())?;
        studio.measurements.push(VisualizationMeasurement {
            id,
            dataset_id,
            analysis_sequence,
            expression,
            value,
        });
        Ok(())
    });
    if report_visualization_commit(app, result) {
        app.state
            .workbench
            .visualization_studio
            .draft_measurement
            .clear();
    }
}

impl AnnotationHost for Host<'_> {
    fn text(&mut self) -> &mut String {
        &mut self.0.state.workbench.visualization_studio.draft_annotation
    }
    fn anchor(&self) -> Option<AnnotationAnchor> {
        source_midpoint(&self.0.state).map(
            |(dataset_id, analysis_sequence, waveform_name, sample_index, x, _)| AnnotationAnchor {
                dataset_id,
                analysis_sequence,
                waveform_name,
                sample_index,
                x,
            },
        )
    }
    fn create(&mut self, anchor: AnnotationAnchor, text: String) {
        create_annotation(self.0, anchor, text);
    }
}

fn create_annotation(app: &mut RSpiceApp, anchor: AnnotationAnchor, text: String) {
    let AnnotationAnchor {
        dataset_id,
        analysis_sequence,
        waveform_name,
        x,
        ..
    } = anchor;
    if active_project_visualization_document_id(&app.state).is_some() {
        let context = active_project_pane_and_trace(&app.state, Some(&waveform_name));
        match context.and_then(|(pane_id, trace_id)| {
            transact_active_project_document(
                app,
                vec![DocumentEdit::AddAnnotation {
                    pane_id,
                    anchor: crate::results::visualization_document::AnnotationAnchor::Trace {
                        trace_id,
                        coordinate: TypedValue::Real(x),
                    },
                    text,
                }],
            )
        }) {
            Ok(_) => {
                app.state
                    .workbench
                    .visualization_studio
                    .draft_annotation
                    .clear();
                reconcile_document(app);
            }
            Err(error) => app.state.push_user_message(ConsoleMessage::error(error)),
        }
        return;
    }
    let result = app.state.workbench.visualization_studio.transact(|studio| {
        let id = studio
            .allocate_identity()
            .ok_or_else(|| "Visualization annotation identity space is exhausted".to_owned())?;
        studio.annotations.push(VisualizationAnnotation {
            id,
            dataset_id,
            analysis_sequence,
            x,
            text,
        });
        Ok(())
    });
    if report_visualization_commit(app, result) {
        app.state
            .workbench
            .visualization_studio
            .draft_annotation
            .clear();
    }
}

impl ExportHost for Host<'_> {
    fn exact_available(&self) -> bool {
        active_studio_exact_export_available(&self.0.state)
    }
    fn figure_available(&self) -> bool {
        active_studio_figure_export_available(&self.0.state)
    }
    fn request_data(&mut self) {
        self.0.state.ui.export_csv_requested = true;
    }
    fn request_figure(&mut self) {
        self.0.state.ui.export_figure_requested = true;
    }
}
