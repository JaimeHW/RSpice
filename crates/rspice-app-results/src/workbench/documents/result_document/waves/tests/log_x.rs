//! The shared X axis switches between linear and logarithmic: a startup
//! transient read over decades of time, a frequency sweep read linearly. A
//! domain a logarithmic axis cannot show refuses it with the reason.

use super::*;
use rspice_results_ui::waves::stack::log_x_refusal;

fn strip_host(kind: AnalysisType, x: Vec<f64>) -> ResultsTestHost {
    let y: Vec<f64> = x.iter().map(|x| x * 2.0).collect();
    let mut run = crate::state::SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, kind, "Analysis")
            .with_waveforms(vec![RetainedTrace::new("V(out)", x, y, "#fff")]),
    );
    run.restore_provenance(crate::state::SimulationRunProvenance::LegacyUnattributed)
        .expect("legacy provenance");
    run.restore_lifecycle(crate::state::SimulationRunLifecycle::LegacyUnknown, 0.0)
        .expect("legacy lifecycle");
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs.push(run);
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);
    state
}

fn model(state: &mut ResultsTestHost) -> Arc<Vec<StripModel>> {
    let presentation = state.model.preferences.result_presentation_policy();
    cached_models(
        &state.model.simulation,
        &mut state.results.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    )
}

fn key(state: &ResultsTestHost) -> AnalysisPresentationKey {
    AnalysisPresentationKey::new(
        state.model.simulation.retained.runs[0].dataset_id,
        &state.model.simulation.retained.runs[0].analyses[0],
    )
}

/// The X axis's log toggle as the drawn waves view publishes it: whether it
/// is lit and whether it can be pressed. `None` when the view draws none.
fn x_log_toggle(state: &mut ResultsTestHost) -> Option<(bool, bool)> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let output = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            show(ui, &mut state.results_parts());
        });
    });
    output.platform_output.accesskit_update.and_then(|update| {
        update.nodes.iter().find_map(|(_, node)| {
            (node.label() == Some("Logarithmic X axis")).then(|| {
                (
                    node.toggled() == Some(egui::accesskit::Toggled::True),
                    !node.is_disabled(),
                )
            })
        })
    })
}

#[test]
fn a_startup_transient_reads_over_decades_of_time_from_its_first_positive_sample() {
    let x = vec![0.0, 1.0e-9, 1.0e-8, 1.0e-7, 1.0e-6, 1.0e-5, 1.0e-4, 1.0e-3];
    let mut state = strip_host(AnalysisType::Transient, x);
    assert_eq!(model(&mut state)[0].x_scale, XScale::Linear);
    assert_eq!(
        x_log_toggle(&mut state),
        Some((false, true)),
        "the axis offers its scale, unlit while linear"
    );
    assert_eq!(log_x_refusal(&model(&mut state)[0]), None);

    let analysis = key(&state);
    state.quick_frame();
    state
        .results
        .results
        .session
        .x_scales
        .insert(analysis, XScale::Log10);
    let models = model(&mut state);
    let strip = &models[0];
    assert_eq!(strip.x_scale, XScale::Log10);
    // t = 0 has no place on a logarithmic axis; the axis starts at the
    // first positive sample.
    assert_eq!(strip.x_range, Some((1.0e-9, 1.0e-3)));
    // Ticks at each decade, as the reference viewers place them.
    let axis = rspice_results_ui::waves::navigation::model_x_axis(
        strip,
        1.0e-9,
        1.0e-3,
        Default::default(),
    );
    let ticks: Vec<f64> = axis.ticks.iter().map(|(value, _)| *value).collect();
    assert_eq!(ticks.len(), 7, "{ticks:?}");
    for (tick, decade) in ticks.iter().zip(-9..=-3) {
        assert!(
            (tick / 10f64.powi(decade) - 1.0).abs() < 1.0e-12,
            "{tick} is not 1e{decade}"
        );
    }
    drop(models);
    assert_eq!(
        x_log_toggle(&mut state),
        Some((true, true)),
        "lit while logarithmic"
    );
    // The choice is the project's: its working view keeps it.
    state.quick_frame();
    let mut reopened = state.reopened();
    reopened.quick_frame();
    assert_eq!(model(&mut reopened)[0].x_scale, XScale::Log10);
}

/// One rule per trace: on a log-frequency axis the line runs straight in
/// log f between samples, and a cursor between two samples reads that line,
/// as its closed form gives it, under the default cursor interpolation.
#[test]
fn a_cursor_between_samples_on_a_log_frequency_axis_reads_the_drawn_line() {
    let pole = |f: f64| -10.0 * (1.0 + (f / 1.0e4).powi(2)).log10();
    let x = vec![1.0e3, 1.0e4, 1.0e5];
    let y: Vec<f64> = x.iter().map(|f| pole(*f)).collect();
    let mut state = ResultsTestHost::default();
    state.model.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![
            RetainedTrace::new("V(out)", x, y, "#fff").with_unit("dB"),
        ]),
    );
    state.quick_frame();
    let interpolation = rspice_results_ui::waves::cursor_interpolation(
        state
            .model
            .preferences
            .result_presentation_policy()
            .cursor_interpolation(),
    );
    let models = model(&mut state);
    let strip = &models[0];
    assert_eq!(strip.x_scale, XScale::Log10);
    let trace = strip.traces.first().unwrap_or_else(|| {
        panic!(
            "the strip draws no trace: {} strips, {} complex",
            models.len(),
            strip.complex_sources.len()
        )
    });
    for decades in [0.5, 1.0 / 3.0, 0.9] {
        let f = 1.0e4 * 10f64.powf(decades);
        let line = pole(1.0e4) + decades * (pole(1.0e5) - pole(1.0e4));
        let read = strip.trace_value(trace, f, interpolation);
        assert!(
            (read - line).abs() < 1e-12,
            "at {f} Hz the cursor reads {read} where the line is drawn at {line}"
        );
    }
    // On a sample it reads the sample.
    assert_eq!(strip.trace_value(trace, 1.0e4, interpolation), pole(1.0e4));
}

#[test]
fn a_frequency_sweep_reads_linearly_when_the_reader_asks() {
    let x = vec![1.0e3, 2.0e3, 3.0e3, 4.0e3];
    let mut state = strip_host(AnalysisType::Ac, x);
    state.results.results.session.viewer = crate::workbench::ResultViewer::Bode;
    assert_eq!(model(&mut state)[0].x_scale, XScale::Log10);
    let analysis = key(&state);
    state
        .results
        .results
        .session
        .x_scales
        .insert(analysis, XScale::Linear);
    assert_eq!(model(&mut state)[0].x_scale, XScale::Linear);
}

#[test]
fn a_signed_sweep_refuses_a_logarithmic_axis_and_says_why() {
    let x = vec![-1.0, -0.5, 0.0, 0.5, 1.0];
    let mut state = strip_host(AnalysisType::DcSweep, x);
    state.results.results.session.viewer = crate::workbench::ResultViewer::DcSweep;
    let reason = log_x_refusal(&model(&mut state)[0]).expect("a signed sweep is refused");
    assert!(reason.contains("reaches \u{2212}1"), "{reason}");
    assert!(reason.contains("positive values only"), "{reason}");
    // A choice that reaches the strip anyway, from a file, is not applied.
    let analysis = key(&state);
    state
        .results
        .results
        .session
        .x_scales
        .insert(analysis, XScale::Log10);
    assert_eq!(model(&mut state)[0].x_scale, XScale::Linear);
    assert_eq!(
        x_log_toggle(&mut state),
        Some((false, false)),
        "the toggle is unlit and refused"
    );
}

/// A saved view keeps the X scale its strip drew when it was saved and draws
/// its own while it is shown. The quick view's choice stays in its working
/// view, and the strip has it back when the view is left.
#[test]
fn a_saved_view_draws_its_own_x_scale_and_hands_the_quick_view_its_choice_back() {
    use super::super::super::persistent_document::{
        capture_pane_presentation, project_pane_presentation, projection,
    };
    use super::expression_units::{document_frame, save_single_pane_view};
    use rspice_results::visualization_document::{AxisOrientation, AxisScale};
    let x = vec![0.0, 1.0e-9, 1.0e-8, 1.0e-7, 1.0e-6, 1.0e-5, 1.0e-4, 1.0e-3];
    let mut state = strip_host(AnalysisType::Transient, x);
    let analysis = key(&state);
    state.quick_frame();
    state
        .results
        .results
        .session
        .x_scales
        .insert(analysis, XScale::Log10);
    state.quick_frame();
    let document_id = save_single_pane_view(&mut state);
    let kept = |state: &ResultsTestHost| {
        state
            .model
            .workspace
            .content
            .visualization_document(document_id)
            .expect("the project keeps the document")
            .axes()
            .iter()
            .find(|axis| axis.orientation == AxisOrientation::Horizontal)
            .map(|axis| axis.scale)
    };
    assert_eq!(
        kept(&state),
        Some(AxisScale::Logarithmic),
        "the view keeps the scale its strip drew"
    );
    document_frame(&mut state, document_id);

    // The reader switches the view's axis to linear: the toggle acts inside
    // the pane's frame, between its projection and its capture.
    let pane = projection(state.results_reads(), document_id)
        .expect("the document projects")
        .pages[0]
        .panes[0]
        .clone();
    let viewer = crate::workbench::ResultViewer::from_viewer_document_id(&pane.viewer_id)
        .expect("the pane names a Results viewer");
    project_pane_presentation(&mut state.results_parts(), &pane, viewer)
        .expect("the pane projects onto its strip");
    state
        .results
        .results
        .session
        .x_scales
        .insert(analysis, XScale::Linear);
    capture_pane_presentation(&mut state.results_parts(), &pane, viewer);
    assert_eq!(kept(&state), Some(AxisScale::Linear));
    document_frame(&mut state, document_id);
    assert_eq!(
        state.results.results.session.x_scales.get(&analysis),
        Some(&XScale::Linear),
        "the strip draws the view's scale"
    );
    assert_eq!(
        state
            .working_view()
            .expect("the run's working view")
            .axes()
            .iter()
            .find(|axis| axis.orientation == AxisOrientation::Horizontal)
            .map(|axis| axis.scale),
        Some(AxisScale::Logarithmic),
        "the working view keeps the quick view's choice"
    );

    state.quick_frame();
    assert_eq!(
        state.results.results.session.x_scales.get(&analysis),
        Some(&XScale::Log10),
        "the quick view has its choice back"
    );
    assert_eq!(model(&mut state)[0].x_scale, XScale::Log10);
    // Shown again, the view still reads linearly, and leaving it again
    // hands the quick view its choice back again.
    document_frame(&mut state, document_id);
    assert_eq!(
        state.results.results.session.x_scales.get(&analysis),
        Some(&XScale::Linear)
    );
    state.quick_frame();
    assert_eq!(
        state.results.results.session.x_scales.get(&analysis),
        Some(&XScale::Log10)
    );
}

/// On log time the overview's miniature starts at the first positive sample,
/// as the panes do; t = 0 does not drag a line out past the overview's left
/// edge into the gutter.
#[test]
fn a_log_time_overview_starts_at_the_first_positive_sample() {
    fn walk(shape: &egui::Shape, out: &mut Vec<egui::Pos2>) {
        match shape {
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, out)),
            egui::Shape::Path(path) => out.extend(path.points.iter().copied()),
            egui::Shape::LineSegment { points, .. } => out.extend(points.iter().copied()),
            _ => {}
        }
    }
    let x = vec![0.0, 1.0e-9, 1.0e-8, 1.0e-7, 1.0e-6, 1.0e-5, 1.0e-4, 1.0e-3];
    let mut state = strip_host(AnalysisType::Transient, x);
    let analysis = key(&state);
    state
        .results
        .results
        .session
        .x_scales
        .insert(analysis, XScale::Log10);
    let ctx = super::lanes::theme_context();
    let mut shapes = Vec::new();
    for _ in 0..2 {
        shapes = super::lanes::frame(&ctx, &mut state, Vec::new());
    }
    let mut points = Vec::new();
    for clipped in &shapes {
        walk(&clipped.shape, &mut points);
    }
    // The overview's row is its track's. A pane clips its own curve, so only
    // the overview's row is checked.
    let row = super::cursors::overview_track(&shapes, Tokens::get(&ctx).color.canvas_bg)
        .center()
        .y;
    let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, super::lanes::FRAME);
    let in_row: Vec<_> = points
        .iter()
        .filter(|point| (point.y - row).abs() < 12.0)
        .collect();
    assert!(!in_row.is_empty(), "the overview draws its miniature");
    let outside: Vec<_> = in_row
        .iter()
        .filter(|point| !screen.contains(***point))
        .collect();
    assert!(
        outside.is_empty(),
        "the overview reaches off the window: {outside:?}"
    );
}

/// `yval` reads a trace where the cursor on it reads: on the decade axis an
/// `.AC LIN 1k 4k` sweep opens on, straight in log f between two samples,
/// and straight in f once the reader reads the axis linearly.
#[test]
fn yval_reads_a_linear_sweep_where_the_cursor_reads_it_on_either_axis() {
    let x = vec![1.0e3, 2.0e3, 3.0e3, 4.0e3];
    let mut state = strip_host(AnalysisType::Ac, x);
    state.results.results.session.viewer = crate::workbench::ResultViewer::Bode;
    assert_eq!(model(&mut state)[0].x_scale, XScale::Log10);
    let analysis = key(&state);
    state
        .results
        .results
        .add_expression_trace(
            &state.model.simulation,
            analysis,
            "yval(V(out), 1500)".to_owned(),
        )
        .expect("the expression is added");
    for scale in [XScale::Log10, XScale::Linear] {
        state
            .results
            .results
            .session
            .x_scales
            .insert(analysis, scale);
        let models = model(&mut state);
        let strip = &models[0];
        assert_eq!(strip.x_scale, scale);
        let trace = strip
            .traces
            .iter()
            .find(|trace| !trace.overlay)
            .expect("V(out) is drawn");
        let cursor = strip.trace_value(trace, 1500.0, SampleInterpolation::Linear);
        let curves = resolve_strip_exprs(&mut state.results_parts(), strip, &Tokens::default());
        let read = curves[0].y.sample(0).expect("yval is drawn");
        assert!(
            (read - cursor).abs() <= 1e-9 * cursor.abs(),
            "on a {scale:?} axis yval reads {read} where the cursor reads {cursor}"
        );
    }
}
