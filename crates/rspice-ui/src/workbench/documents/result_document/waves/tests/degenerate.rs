//! What the instrument does with data no reader should ever see.
//!
//! A run that failed to converge retains empty vectors, ragged pairs, NaN,
//! infinities and lone samples, and the sheets draw whatever was kept. These
//! cases go all the way through layout, painting and tessellation, because a
//! coordinate that slips past an axis mapping does not trip an assertion — it
//! becomes a vertex at infinity and takes the whole draw call with it.

use super::*;

/// A run that failed to converge — or an expression that divided by zero —
/// retains exactly this: empty vectors, ragged x/y pairs, NaN, ±inf, a lone
/// sample and a flat trace. The instrument draws whatever the engine kept, so
/// every one of these reaches the painters, and a panic there takes the
/// workbench down on the one run the user most needs to read.
fn degenerate_waveforms() -> Vec<(&'static str, WaveformData)> {
    vec![
        (
            "empty",
            WaveformData::new("V(empty)", Vec::<f64>::new(), Vec::<f64>::new(), "#fff"),
        ),
        (
            "ragged",
            WaveformData::new("V(ragged)", vec![0.0, 1.0e-3, 2.0e-3], vec![1.0], "#f80"),
        ),
        (
            "all-nan",
            WaveformData::new(
                "V(nan)",
                vec![0.0, 1.0e-3],
                vec![f64::NAN, f64::NAN],
                "#0af",
            ),
        ),
        (
            "infinite",
            WaveformData::new(
                "V(inf)",
                vec![0.0, 1.0e-3],
                vec![f64::INFINITY, f64::NEG_INFINITY],
                "#8f0",
            ),
        ),
        (
            "single-sample",
            WaveformData::new("V(single)", vec![5.0e-4], vec![2.0], "#f0f"),
        ),
        (
            "flat",
            WaveformData::new("V(flat)", vec![0.0, 1.0e-3], vec![3.0, 3.0], "#ff0"),
        ),
        (
            "nan-abscissa",
            WaveformData::new("V(nanx)", vec![f64::NAN, 1.0e-3], vec![0.0, 1.0], "#0ff"),
        ),
        (
            "astronomical",
            WaveformData::new(
                "I(huge)",
                vec![0.0, 1.0e-3],
                vec![-1.0e300, 1.0e300],
                "#f00",
            ),
        ),
        (
            "denormal-span",
            WaveformData::new("V(tiny)", vec![0.0, 1.0e-3], vec![0.0, 5.0e-324], "#88f"),
        ),
        (
            "denormal-domain",
            WaveformData::new("V(dx)", vec![0.0, 5.0e-324], vec![0.0, 1.0], "#8ff"),
        ),
    ]
}

fn degenerate_transient() -> AnalysisResult {
    AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(
        degenerate_waveforms()
            .into_iter()
            .map(|(_, waveform)| waveform)
            .collect(),
    )
}

/// Draw a sheet through a real egui frame and tessellate the result, exactly
/// as the workbench does.
///
/// Layout, every painter, and mesh generation all run here. That is the only
/// harness that can prove the instrument survives its own data: a value that
/// slips past the axis mapping does not fail an assert, it becomes a vertex
/// at infinity, and the whole draw call degenerates around it.
fn draw_and_tessellate(state: &mut AppState, sheet: fn(&mut Ui, &mut AppState)) {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1440.0, 900.0),
        )),
        ..Default::default()
    };
    // Two frames: the first builds the model and derived caches, the second
    // is the cached path the user actually spends their time in.
    for _ in 0..2 {
        let output = ctx.run_ui(input.clone(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| sheet(ui, state));
        });
        for primitive in ctx.tessellate(output.shapes, output.pixels_per_point) {
            let egui::epaint::Primitive::Mesh(mesh) = primitive.primitive else {
                continue;
            };
            assert!(
                mesh.vertices
                    .iter()
                    .all(|vertex| vertex.pos.x.is_finite() && vertex.pos.y.is_finite()),
                "a non-finite vertex reached the mesh",
            );
        }
    }
}

/// Put the sheet in the state that reads values back out of the data: both
/// cursors down and the readout open. The value, delta and slope columns all
/// compute and format from the samples themselves, so a degenerate run has
/// to survive being read as well as being drawn.
fn arm_cursors(state: &mut AppState) {
    if !state.ui.results.cursor_tool.is_armed() {
        state.ui.results.toggle_cursor_tool();
    }
    state.ui.results.cursor_strip = Some(0);
    state.ui.results.cursors.a = Some(5.0e-4);
    state.ui.results.cursors.b = Some(1.0e-3);
    state.ui.results.readout_collapsed = false;
}

// The crate bans stderr printing so debugging scaffolding cannot reach a
// release build. This is the one place it earns its keep: the failures this
// test exists to catch abort the process rather than panic — the tick ladder
// once asked the allocator for 128 GiB on a denormal span — and an abort
// unwinds nothing, so a panic message would never be printed. Naming each
// case as it starts is the only record of which shape broke the instrument.
#[allow(clippy::print_stderr)]
#[test]
fn the_instrument_survives_each_degenerate_trace_alone() {
    for (label, waveform) in degenerate_waveforms() {
        eprintln!("degenerate case: {label}");
        let mut state = AppState::default();
        state.simulation.start_run().add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![waveform]),
        );
        arm_cursors(&mut state);
        draw_and_tessellate(&mut state, show);
    }
}

#[test]
fn the_instrument_survives_a_degenerate_run() {
    let mut state = AppState::default();
    state
        .simulation
        .start_run()
        .add_analysis(degenerate_transient());
    arm_cursors(&mut state);

    draw_and_tessellate(&mut state, show);

    // The domain is what every screen coordinate on the sheet derives from:
    // one NaN admitted here maps the entire run to nowhere.
    let presentation = state.ui.preferences.result_presentation_policy();
    let models = cached_models(
        &state.simulation,
        &mut state.ui.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    );
    let model = &models[0];
    let (x0, x1) = model
        .x_range
        .expect("a finite domain survives the degenerate traces");
    assert!(x0.is_finite() && x1.is_finite() && x1 > x0, "{x0}..{x1}");

    // Every pane needs a non-zero height as well: a flat trace and an
    // all-NaN one both collapse to a single value before padding.
    let mut derived = DerivedSeries::default();
    for pane in model.unit_panes() {
        let Some((lo, hi)) = pane_y_range(&mut derived, model, &pane.traces) else {
            continue;
        };
        assert!(
            lo.is_finite() && hi.is_finite(),
            "{}: {lo}..{hi}",
            pane.unit
        );
        assert!(hi > lo, "{}: a pane needs a non-zero height", pane.unit);
    }
}

#[test]
fn a_long_run_that_diverges_partway_still_draws() {
    // Enough samples to force the decimation path, which is where a real
    // transient lives. The degenerate fixtures above are all short enough to
    // stroke raw, so without this the min/max envelope — the code an actual
    // diverged run goes through — would never be drawn at all.
    let count = 20_000usize;
    let x = (0..count).map(|i| i as f64 * 1.0e-7).collect::<Vec<_>>();
    let y = (0..count)
        .map(|i| {
            if i < count * 3 / 5 {
                (i as f64 * 1.0e-3).sin()
            } else {
                f64::NAN
            }
        })
        .collect::<Vec<_>>();
    let mut state = AppState::default();
    state.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![WaveformData::new("V(out)", x, y, "#0af")]),
    );
    arm_cursors(&mut state);

    draw_and_tessellate(&mut state, show);
}

#[test]
fn the_log_frequency_sheets_survive_a_non_positive_abscissa() {
    let mut state = AppState::default();
    let run = state.simulation.start_run();
    // A sweep carrying DC, and a PSD that went negative. log10 of the first
    // is -inf and the square root of the second is NaN, and both are shapes
    // a real engine emits — an .ac list starting at 0, a subtracted noise
    // expression that undershoots.
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_waveforms(vec![WaveformData::new(
            "V(out)",
            vec![0.0, -1.0, 1.0e3],
            vec![1.0, 0.5, 0.25],
            "#0af",
        )]),
    );
    run.add_analysis(
        AnalysisResult::new(2, AnalysisType::Noise, "NOISE").with_waveforms(vec![
            WaveformData::new("onoise", vec![0.0, 1.0e3], vec![-1.0e-18, 4.0e-18], "#f80"),
        ]),
    );

    state.ui.results.viewer = super::super::super::ResultViewer::Bode;
    draw_and_tessellate(&mut state, show_bode);

    state.ui.results.viewer = super::super::super::ResultViewer::NoiseContrib;
    draw_and_tessellate(&mut state, show_noise);
}
