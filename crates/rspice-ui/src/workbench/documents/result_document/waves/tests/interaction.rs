//! What the reader's gestures do to the strip they land on.
//!
//! Two things decide whether an interaction is right, and neither of them is
//! whether it ran without panicking. A gesture on one pane has to leave the
//! strip's panes agreeing about the window they show, because they draw one
//! sweep between them. And a gesture that means "the curve under my pointer"
//! has to be resolved through the same value→screen mapping the painter used,
//! or "nearest" is measured in a space nobody is looking at.

use super::*;

/// An AC run whose one analysis projects a magnitude pane over a phase pane —
/// the two-pane strip every shared-abscissa question needs.
fn bode_strip() -> AppState {
    let mut state = AppState::default();
    state.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![
            WaveformData::new(
                "|V(out)|",
                vec![1.0, 10.0, 100.0, 1000.0],
                vec![1.0, 0.7, 0.3, 0.1],
                "#fff",
            ),
            WaveformData::new(
                "phase(V(out))",
                vec![1.0, 10.0, 100.0, 1000.0],
                vec![0.0, -30.0, -60.0, -90.0],
                "#0af",
            ),
        ]),
    );
    state.ui.results.viewer = super::super::super::ResultViewer::Bode;
    state
}

fn strip_key(state: &mut AppState) -> AnalysisPresentationKey {
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    models[0].analysis_key
}

fn pane_units(state: &mut AppState) -> Vec<String> {
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    models[0]
        .unit_panes()
        .iter()
        .map(|pane| pane.unit.to_owned())
        .collect()
}

fn pane_x(
    state: &AppState,
    analysis: AnalysisPresentationKey,
    ordinal: usize,
) -> Option<(f64, f64)> {
    state
        .ui
        .results
        .analysis_plot_view_pane(super::super::super::ResultViewer::Waves, analysis, ordinal)
        .x
}

/// The panes of one strip draw one sweep, so a zoom taken on any of them is a
/// statement about all of them. Written through a single pane's ordinal, the
/// magnitude pane went on showing the whole sweep while the phase pane under
/// it showed a decade — two axes, one abscissa, disagreeing.
#[test]
fn zoom_from_a_companion_pane_moves_the_strips_shared_abscissa() {
    let mut state = bode_strip();
    let analysis = strip_key(&mut state);
    let units = pane_units(&mut state);
    assert_eq!(units.len(), 2, "the fixture is a magnitude/phase pair");
    let phase_ordinal = units
        .iter()
        .position(|unit| unit == "\u{b0}")
        .expect("the phase pane");
    assert_ne!(phase_ordinal, 0, "the phase pane is the companion");

    state.ui.results.active_wave_pane = Some(WavePanePresentationKey {
        analysis,
        unit: units[phase_ordinal].clone(),
    });
    zoom_active_pane(&mut state, &Tokens::default(), 0.5);

    let magnitude = pane_x(&state, analysis, 0).expect("the magnitude pane took the zoom");
    let phase = pane_x(&state, analysis, phase_ordinal).expect("the phase pane took the zoom");
    assert_eq!(
        magnitude, phase,
        "a zoom from one pane left the strip's panes on different windows"
    );
    assert!(
        magnitude.0 > 1.0 && magnitude.1 < 1000.0,
        "the zoom did not narrow the retained span: {magnitude:?}"
    );
}

/// A pane drawing decades is zoomed in decades. Scaling its endpoints
/// linearly moves the geometric centre the reader was looking at, so zooming
/// in and back out does not return to where it started.
#[test]
fn a_logarithmic_pane_zooms_about_its_geometric_centre() {
    let mut state = AppState::default();
    state.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            WaveformData::new(
                "V(out)",
                vec![0.0, 1.0, 2.0],
                vec![1.0, 10.0, 100.0],
                "#fff",
            ),
        ]),
    );
    let analysis = strip_key(&mut state);
    let units = pane_units(&mut state);
    state.ui.results.active_wave_pane = Some(WavePanePresentationKey {
        analysis,
        unit: units[0].clone(),
    });
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    let model = &models[0];
    let pane = model.unit_panes().into_iter().next().expect("one pane");
    set_pane_log_y(&mut state.ui.results, model, &pane, true);
    drop(models);
    let before = (1.0, 100.0);
    state
        .ui
        .results
        .analysis_plot_view_pane_mut(super::super::super::ResultViewer::Waves, analysis, 0)
        .y = Some(before);

    zoom_active_pane(&mut state, &Tokens::default(), 0.5);

    let after = state
        .ui
        .results
        .analysis_plot_view_pane(super::super::super::ResultViewer::Waves, analysis, 0)
        .y
        .expect("the zoom pinned a Y window");
    let centre = |range: (f64, f64)| (range.0.log10() + range.1.log10()) * 0.5;
    assert!(
        (centre(after) - centre(before)).abs() < 1.0e-9,
        "a decade pane's zoom moved its geometric centre: {before:?} -> {after:?}"
    );
    let decades = |range: (f64, f64)| range.1.log10() - range.0.log10();
    assert!(
        (decades(after) - decades(before) * 0.5).abs() < 1.0e-9,
        "half a zoom factor did not halve the decades shown: {before:?} -> {after:?}"
    );
    assert!(after.0 > 0.0, "a decade pane cannot reach zero: {after:?}");
}

/// Fit means the pane returns to its data. The abscissa it returns is the
/// strip's, so every ordinal that pinned one has to release it — including an
/// ordinal the strip has stopped drawing, which otherwise goes on reporting
/// the strip as zoomed with no pane left to fit it from.
#[test]
fn fitting_a_pane_releases_every_ordinal_the_strip_pinned() {
    let mut state = bode_strip();
    let analysis = strip_key(&mut state);
    let units = pane_units(&mut state);
    let viewer = super::super::super::ResultViewer::Waves;
    for ordinal in [0usize, 1, 5] {
        state
            .ui
            .results
            .analysis_plot_view_pane_mut(viewer, analysis, ordinal)
            .x = Some((10.0, 100.0));
    }
    state
        .ui
        .results
        .analysis_plot_view_pane_mut(viewer, analysis, 1)
        .y = Some((-90.0, 0.0));
    state.ui.results.active_wave_pane = Some(WavePanePresentationKey {
        analysis,
        unit: units[0].clone(),
    });

    fit_active_pane(&mut state, &Tokens::default());

    assert!(
        !state.ui.results.analysis_strip_axis_is_pinned(
            viewer,
            analysis,
            super::super::super::PaneAxis::X
        ),
        "an ordinal the strip no longer draws kept the abscissa pinned"
    );
    assert_eq!(
        state
            .ui
            .results
            .analysis_plot_view_pane(viewer, analysis, 1)
            .y,
        Some((-90.0, 0.0)),
        "fitting one pane took a sibling pane's own Y with it"
    );
}

/// "Nearest" is a claim about the screen. The painter maps a value through
/// the pane's own scale, so a hit test that maps it linearly picks a curve
/// the pointer is nowhere near — by most of the window, on a decade pane.
///
/// Two traces a decade and a half apart, and a pointer between them where the
/// two mappings disagree about which is closer: the logarithmic answer is the
/// one that is drawn.
#[test]
fn the_nearest_trace_is_measured_where_the_pane_draws_it() {
    let mut state = AppState::default();
    state.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            WaveformData::new("V(a)", vec![0.0, 1.0], vec![1.0, 1.0], "#fff"),
            WaveformData::new("V(b)", vec![0.0, 1.0], vec![100.0, 100.0], "#0af"),
        ]),
    );
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    let model = &models[0];
    let pane_traces: Vec<(usize, &StripTrace)> =
        model.traces.iter().enumerate().collect::<Vec<_>>();
    let rect = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(100.0, 100.0));

    // v=1 draws at y=100 on both scales. v=100 draws at y≈33.3 logarithmically
    // and at y≈90.1 linearly, so a pointer at 70 is nearer V(a) on the scale
    // the pane is drawn on and nearer V(b) on the one it is not.
    let drawn = nearest_drawn_trace(
        &pane_traces,
        0.5,
        Some(70.0),
        rect,
        XScale::Log10,
        (1.0, 1000.0),
        SampleInterpolation::Linear,
    )
    .expect("a trace is under the pointer");
    assert_eq!(
        drawn.name, "V(a)",
        "the hit test picked a curve the pointer is not near"
    );

    let linear = nearest_drawn_trace(
        &pane_traces,
        0.5,
        Some(70.0),
        rect,
        XScale::Linear,
        (1.0, 1000.0),
        SampleInterpolation::Linear,
    )
    .expect("a trace is under the pointer");
    assert_eq!(
        linear.name, "V(b)",
        "the fixture no longer separates the two mappings"
    );
}

/// Past a certain zoom the shared strip's tick row stops being values and
/// becomes differences. A row of "−40n … 0 … +40n" with nothing beside it to
/// subtract them from states a window near zero, which is not where the
/// reader is.
#[test]
fn a_deeply_zoomed_shared_axis_states_the_value_its_ticks_are_offsets_from() {
    let mut state = AppState::default();
    state.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran").with_waveforms(vec![
            WaveformData::new(
                "V(out)",
                vec![1.0e-3, 1.000_02e-3, 1.000_04e-3, 1.000_06e-3, 1.000_08e-3],
                vec![0.0, 1.0, 0.0, 1.0, 0.0],
                "#fff",
            ),
        ]),
    );
    let presentation = state.ui.preferences.result_presentation_policy();
    let quantity_policy = state.ui.preferences.quantity_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    let model = &models[0];
    let (x0, x1) = model.x_range.expect("the strip has an extent");
    let axis = model_x_axis(model, x0, x1, quantity_policy);
    let anchor = axis
        .offset_anchor()
        .expect("this window is deep enough for offset labels")
        .to_owned();
    drop(models);

    let painted = painted_texts(&mut state, show);
    assert!(
        painted.iter().any(|text| text == &anchor),
        "the shared axis drew offsets without stating the {anchor} they are offsets from: \
         {painted:?}"
    );
}

/// Every string one sheet paints through a real egui frame.
pub(super) fn painted_texts(
    state: &mut AppState,
    sheet: fn(&mut Ui, &mut AppState),
) -> Vec<String> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1440.0, 900.0),
        )),
        ..Default::default()
    };
    let mut texts = Vec::new();
    // Two frames: the first builds the caches, the second is the path the
    // reader actually spends their time in.
    for _ in 0..2 {
        texts.clear();
        let output = ctx.run_ui(input.clone(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| sheet(ui, state));
        });
        collect_texts(&output.shapes, &mut texts);
    }
    texts
}

fn collect_texts(shapes: &[egui::epaint::ClippedShape], out: &mut Vec<String>) {
    fn walk(shape: &egui::Shape, out: &mut Vec<String>) {
        match shape {
            egui::Shape::Text(text) => out.push(text.galley.text().to_owned()),
            egui::Shape::Vec(inner) => {
                for shape in inner {
                    walk(shape, out);
                }
            }
            _ => {}
        }
    }
    for clipped in shapes {
        walk(&clipped.shape, out);
    }
}

/// Total points in every stroked path one sheet paints.
///
/// A curve that reduces to nothing still lays out its axes and its legend, so
/// "the pane drew something" is not the question — "the pane drew a curve" is.
pub(super) fn painted_path_points(
    state: &mut AppState,
    sheet: fn(&mut Ui, &mut AppState),
) -> usize {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1440.0, 900.0),
        )),
        ..Default::default()
    };
    let mut points = 0;
    for _ in 0..2 {
        points = 0;
        let output = ctx.run_ui(input.clone(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| sheet(ui, state));
        });
        for clipped in &output.shapes {
            points += path_points(&clipped.shape);
        }
    }
    points
}

fn path_points(shape: &egui::Shape) -> usize {
    match shape {
        egui::Shape::Path(path) => path.points.len(),
        egui::Shape::Vec(inner) => inner.iter().map(path_points).sum(),
        _ => 0,
    }
}
