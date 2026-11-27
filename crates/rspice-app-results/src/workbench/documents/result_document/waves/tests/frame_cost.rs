//! Frame cost of the waveform strip on very large results.
//!
//! Two loads stand for the results that break a waveform viewer: many
//! unrelated signals on one time grid, and a family of many members of one
//! quantity. The gates count work rather than time it: how many envelopes a
//! frame rebuilt from the retained samples. Wall clock on CI is noise; a
//! rebuild count is not.

use super::*;

use rspice_ui_kit::plot::DecimationWork;

/// The window the measured frames are drawn into, as in the idle-frame gates.
const SCREEN: egui::Vec2 = egui::vec2(1_680.0, 1_020.0);

/// One transient analysis holding `traces` unrelated waveforms of `samples`
/// points on one shared time grid.
///
/// Each is its own signal, with its own gain and phase on one shape, so the
/// curves overlap as densely as a family's members do and the pane fits one
/// Y range to all of them; but no family model relates them.
fn crowded_transient(traces: usize, samples: usize) -> ResultsTestHost {
    transient_in_units(traces, samples, &["V"])
}

/// [`crowded_transient`] with `traces` members in each of `units`: one pane
/// per unit, every curve on the same time grid.
fn transient_in_units(traces: usize, samples: usize, units: &[&str]) -> ResultsTestHost {
    let time: Arc<Vec<f64>> = Arc::new((0..samples).map(|index| index as f64 * 1.0e-9).collect());
    // About three periods across the window whatever the sample count, so
    // every column of the strip holds a real minimum and maximum.
    let step = 20.0 / samples as f64;
    let waveforms = units
        .iter()
        .enumerate()
        .flat_map(|(pane, unit)| (0..traces).map(move |member| (pane, *unit, member)))
        .map(|(pane, unit, member)| {
            let spread = member as f64 / traces.max(1) as f64 - 0.5;
            let gain = 0.1f64.mul_add(spread, 1.0);
            let phase = 0.4 * spread;
            let values: Vec<f64> = (0..samples)
                .map(|index| gain * (index as f64).mul_add(step, phase).sin())
                .collect();
            RetainedTrace::new(
                format!("p{pane}m{member}"),
                Arc::clone(&time),
                values,
                "#00aaff",
            )
            .with_unit(unit)
        })
        .collect();
    let mut state = ResultsTestHost::default();
    state.model.simulation.start_run().add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(waveforms),
    );
    state
}

/// A family of `members` curves of `samples` points: one quantity, `V(out)`,
/// solved at `members` temperatures against one swept input, presented
/// through the strip's family model as one curve per member.
///
/// Each member has its own gain and phase, as [`crowded_transient`]'s curves
/// do, so the members overlap the way a real family's do.
fn family_of(members: usize, samples: usize) -> ResultsTestHost {
    family_of_signals(members, samples, &["V(out)"])
}

/// [`family_of`] answering every one of `signals`, each a quarter period
/// behind the one before it, so any two of them trace a loop.
pub(super) fn family_of_signals(
    members: usize,
    samples: usize,
    signals: &[&str],
) -> ResultsTestHost {
    let step = 20.0 / samples as f64;
    let rows = members * samples;
    let (mut x, mut ys, mut temperatures) = (
        Vec::with_capacity(rows),
        vec![Vec::with_capacity(rows); signals.len()],
        Vec::with_capacity(rows),
    );
    for member in 0..members {
        let spread = member as f64 / members.max(1) as f64 - 0.5;
        let gain = 0.1f64.mul_add(spread, 1.0);
        let phase = 0.4 * spread;
        for index in 0..samples {
            x.push(index as f64 * 1.0e-3);
            for (lag, y) in ys.iter_mut().enumerate() {
                let behind = lag as f64 * std::f64::consts::FRAC_PI_2;
                y.push(gain * (index as f64).mul_add(step, phase - behind).sin());
            }
            temperatures.push(member as f64);
        }
    }
    let mut state = ResultsTestHost::default();
    let run = state.model.simulation.start_run();
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Corner, "PVT")
            .with_waveforms(
                signals
                    .iter()
                    .zip(ys)
                    .map(|(name, y)| {
                        RetainedTrace::new(*name, x.clone(), y, "#00aaff").with_unit("V")
                    })
                    .collect(),
            )
            .with_family_metadata(AnalysisResultFamilyMetadata::Corner {
                member_measurements: Vec::new(),
                x_values: x,
                x_label: "VIN".to_owned(),
                x_unit: "V".to_owned(),
                temperatures_c: temperatures,
                corner_labels: vec!["TT".to_owned(); rows],
                failed_corners: 0,
            }),
    );
    let manifest = FamilyManifest::from_metadata(
        run.analyses[0].analysis_type,
        run.analyses[0].family_metadata.as_ref(),
    )
    .expect("the corner metadata is valid")
    .expect("a corner result is a family");
    let temperature = FamilyDimension::new("temperature", ValueType::Real).expect("dimension");
    let policy = FamilyPresentationPolicy {
        x_dimension: FamilyXDimension {
            dimension: FamilyDimension::new("VIN", ValueType::Real).expect("dimension"),
            ordering: FamilyXOrdering::Source,
        },
        family_dimensions: vec![temperature.clone()],
        facet_layout: None,
        aggregation: FamilyAggregationPolicy {
            method: FamilyAggregationMethod::None,
            over_dimensions: Vec::new(),
        },
        filter: None,
        missing_points: MissingPointPolicy::ExcludeWithOmissionRecord,
        // Labelled, not coloured: every member keeps the quantity's hue, as
        // a family drawn for its spread is.
        encodings: vec![FamilyEncodingMap::Label {
            dimension: temperature,
            prefix: None,
        }],
    };
    let selection = SourceSampleSelection::new(run.dataset_id, 1, (0..rows).collect())
        .expect("every row is selected")
        .with_family_presentation(&manifest, &policy)
        .expect("the policy presents the family");
    state.results.results.set_sample_selection(Some(selection));
    state
}

/// What one frame cost.
#[derive(Debug, Clone, Copy)]
pub(super) struct FrameCost {
    pub(super) vertices: usize,
    pub(super) work: DecimationWork,
}

pub(super) fn new_context() -> egui::Context {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx
}

/// Run one frame of the waveform strip with `events` as its only input.
pub(super) fn frame(
    ctx: &egui::Context,
    state: &mut ResultsTestHost,
    events: Vec<egui::Event>,
) -> FrameCost {
    let before = state.results.results.session.cache.work();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, SCREEN)),
            events,
            ..Default::default()
        },
        |ctx| {
            // The workspace prepares its caches once per frame before any
            // sheet draws; that is what advances the envelope cache's frame.
            crate::workbench::documents::result_document::prepare_viewer_state(
                &mut state.results_parts(),
            );
            egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut state.results_parts()));
        },
    );
    let primitives = ctx.tessellate(output.shapes, output.pixels_per_point);
    let vertices = primitives
        .iter()
        .map(|primitive| match &primitive.primitive {
            egui::epaint::Primitive::Mesh(mesh) => mesh.vertices.len(),
            egui::epaint::Primitive::Callback(_) => 0,
        })
        .sum();
    FrameCost {
        vertices,
        work: state.results.results.session.cache.work().since(before),
    }
}

/// A pointer sweep across the plot, one position per frame.
fn hover_positions(count: usize) -> Vec<egui::Pos2> {
    (0..count)
        .map(|index| {
            let fraction = (index as f32 + 0.5) / count as f32;
            egui::pos2(fraction.mul_add(1_100.0, 400.0), 420.0)
        })
        .collect()
}

/// A family too large to stroke member by member is drawn as the band it
/// occupies. Its warm frames read nothing from the retained samples, and its
/// hovered frames build at most the one envelope of the member newly under
/// the pointer — never the family's.
#[test]
fn a_large_family_draws_its_band_and_rebuilds_nothing_warm_or_hovered() {
    let mut state = family_of(100, 1_000);
    let ctx = new_context();
    let cold = frame(&ctx, &mut state, Vec::new());
    assert_eq!(
        cold.work.rebuilds, 1,
        "the cold frame folds the family into one band, not 100 envelopes"
    );
    for _ in 0..3 {
        let warm = frame(&ctx, &mut state, Vec::new());
        assert_eq!(warm.work.rebuilds, 0, "a warm frame rebuilt an envelope");
        // A hundred members of a thousand points stroked one by one are
        // hundreds of thousands of vertices; the band and its edges are
        // thousands.
        assert!(
            warm.vertices < 100_000,
            "a 100-member family tessellated {} vertices",
            warm.vertices
        );
    }
    let sweep = hover_positions(12);
    let first: Vec<FrameCost> = sweep
        .iter()
        .map(|position| frame(&ctx, &mut state, vec![egui::Event::PointerMoved(*position)]))
        .collect();
    assert!(
        first.iter().all(|cost| cost.work.rebuilds <= 1),
        "a hover rebuilt more than the curve under the pointer: {:?}",
        first
            .iter()
            .map(|cost| cost.work.rebuilds)
            .collect::<Vec<_>>()
    );
    assert!(
        first
            .iter()
            .all(|cost| cost.work.hits + cost.work.rebuilds <= 2),
        "a hovered frame reads the band and at most the one curve under the pointer: {:?}",
        first.iter().map(|cost| cost.work).collect::<Vec<_>>()
    );
    // The same sweep again finds every curve it highlights already reduced.
    for position in &sweep {
        let again = frame(&ctx, &mut state, vec![egui::Event::PointerMoved(*position)]);
        assert_eq!(
            again.work.rebuilds, 0,
            "a repeated hover rebuilt an envelope"
        );
    }
}

/// A hundred unrelated curves in one pane are stroked one by one, never
/// folded into a band: a band says its members are one quantity spread by
/// what varied between them, which is false of unrelated signals. The cold
/// frame reduces each curve's own envelope; a warm or hovered frame keeps the
/// picture it drew and reads nothing, no band included.
#[test]
fn a_hundred_unrelated_curves_are_never_drawn_as_a_band() {
    let mut state = crowded_transient(100, 1_000);
    let ctx = new_context();
    let cold = frame(&ctx, &mut state, Vec::new());
    assert_eq!(
        (cold.work.hits, cold.work.rebuilds),
        (0, 100),
        "the cold frame reduces every curve on its own"
    );
    assert_eq!(cold.work.strokes, 100, "the cold frame strokes every curve");
    let warm = frame(&ctx, &mut state, Vec::new());
    assert_eq!(
        (warm.work.hits, warm.work.rebuilds, warm.work.strokes),
        (0, 0, 0),
        "a warm frame keeps the picture it drew"
    );
    for position in hover_positions(6) {
        let hover = frame(&ctx, &mut state, vec![egui::Event::PointerMoved(position)]);
        assert_eq!(
            (hover.work.hits, hover.work.rebuilds, hover.work.strokes),
            (0, 0, 0),
            "a hovered frame keeps the picture it drew, and reads no band"
        );
    }
}

/// Unticking a signal in the data browser keeps every other trace's
/// envelope: the curves left on the strip are redrawn from what the cache
/// holds, not reduced again from every sample.
#[test]
fn unticking_a_signal_keeps_the_other_traces_envelopes() {
    use crate::workbench::documents::result_document::browser_plot::set_result_signal_plotted;
    use rspice_results::result_presentation::SourceWaveformPresentationKey;
    let mut state = crowded_transient(3, 50_000);
    // The quietest member is the one the dataset hides: plotted and taken
    // off again, it moves neither the pane's Y range nor its other curves.
    let analysis = &mut state.model.simulation.retained.runs[0].analyses[0];
    analysis.waveforms[0].visible = false;
    let name = analysis.waveforms[0].name.clone();
    let key =
        SourceWaveformPresentationKey::new(super::lanes::strip(&mut state)[0].analysis_key, name);
    let ctx = new_context();
    assert_eq!(frame(&ctx, &mut state, Vec::new()).work.rebuilds, 2);
    set_result_signal_plotted(&mut state.results_parts(), &key, true);
    assert_eq!(
        frame(&ctx, &mut state, Vec::new()).work.rebuilds,
        1,
        "the ticked signal is the only one reduced"
    );
    set_result_signal_plotted(&mut state.results_parts(), &key, false);
    let unticked = frame(&ctx, &mut state, Vec::new());
    assert_eq!(
        (unticked.work.hits, unticked.work.rebuilds),
        (2, 0),
        "the untick reduced the curves it left on the strip again"
    );
}

/// A strip that strokes more curves than the envelope cache's stale bound —
/// five panes of sixty — reuses every envelope on its warm and hovered
/// frames. With a hard 256-entry bound it evicted and rebuilt most of them on
/// every repaint.
#[test]
fn a_strip_stroking_more_curves_than_the_cache_bound_rebuilds_nothing_warm_or_hovered() {
    let mut state = transient_in_units(60, 1_000, &["V", "A", "W", "F", "H"]);
    let ctx = new_context();
    let cold = frame(&ctx, &mut state, Vec::new());
    assert_eq!(cold.work.rebuilds, 300, "every curve is reduced once");
    for _ in 0..3 {
        let warm = frame(&ctx, &mut state, Vec::new());
        assert_eq!(
            (warm.work.rebuilds, warm.work.evictions),
            (0, 0),
            "a warm frame rebuilt or evicted an envelope"
        );
    }
    for position in hover_positions(6) {
        let hover = frame(&ctx, &mut state, vec![egui::Event::PointerMoved(position)]);
        assert_eq!(hover.work.rebuilds, 0, "a hover rebuilt an envelope");
    }
}

/// One curve as a strip draws it: its signal, palette place, colour and
/// whether it is drawn.
type DrawnCurve = (String, Option<usize>, egui::Color32, bool);

/// What a strip draws: each curve, and the strip's X extent.
fn strip_drawn(model: &StripModel) -> (Vec<DrawnCurve>, String) {
    let curves = model
        .traces
        .iter()
        .map(|trace| {
            (
                trace.source_waveform_name.clone(),
                trace.palette_slot,
                trace.color,
                trace.visible,
            )
        })
        .collect();
    (curves, format!("{:?} {:?}", model.x_bounds, model.x_range))
}

/// A hundred thousand signals ticked at once change the strip models built
/// before by the ticks, as one tick does, so the strip they leave alone is
/// the one built before; and after the gesture, its Undo and its Redo, every
/// strip draws what a build from the same ticks draws.
#[test]
fn a_hundred_thousand_signals_ticked_at_once_follow_the_strips_built_before() {
    use crate::workbench::docks::navigator::results_actions::set_result_quantities_ticked;
    use crate::workbench::documents::result_document::view_edits;
    use rspice_results::result_presentation::SourceWaveformPresentationKey;
    use rspice_results_ui::selection::ResultBrowserSelectionKey;

    const SIGNALS: usize = 100_000;
    // Every signal on the run's one time axis, as a run solves them.
    let time = Arc::new(vec![0.0, 1.0]);
    let wide = (0..SIGNALS)
        .map(|index| {
            let mut trace = RetainedTrace::new(
                format!("V(n{index})"),
                Arc::clone(&time),
                vec![0.0, 1.0],
                "",
            )
            .with_unit("V");
            trace.visible = index == 0;
            trace
        })
        .collect();
    let mut state = ResultsTestHost::default();
    let run = state.model.simulation.start_run();
    run.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(wide));
    run.add_analysis(
        AnalysisResult::new(2, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            RetainedTrace::new("V(out)", vec![0.0, 1.0], vec![1.0, 2.0], "").with_unit("V"),
        ]),
    );
    state.quick_frame();
    let (ticked, other) = {
        let run = state.model.simulation.active_run().expect("the run");
        (
            AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]),
            AnalysisPresentationKey::new(run.dataset_id, &run.analyses[1]),
        )
    };
    let models = |state: &mut ResultsTestHost| {
        let presentation = state.model.preferences.result_presentation_policy();
        cached_models(
            &state.model.simulation,
            &mut state.results.results,
            presentation.complex_number_display(),
            &Tokens::default(),
        )
    };
    let strip = |models: &[StripModel], analysis| {
        models
            .iter()
            .position(|model| model.analysis_key == analysis)
            .expect("the analysis has a strip")
    };
    // The strips as a build from the ticks draws them, and the revision of
    // the one the gesture leaves alone.
    let check = |state: &mut ResultsTestHost, label: &str| {
        let followed = models(state);
        let afresh = {
            let mut fresh = state.clone();
            fresh.results.results.models.invalidate();
            let built = models(&mut fresh);
            built.iter().map(strip_drawn).collect::<Vec<_>>()
        };
        let drawn = followed.iter().map(strip_drawn).collect::<Vec<_>>();
        assert!(
            drawn == afresh,
            "{label}: the strips draw what a build from the same ticks draws"
        );
        followed[strip(&followed, other)].revision()
    };
    let before = check(&mut state, "before");
    let keys: Vec<ResultBrowserSelectionKey> = (1..SIGNALS)
        .map(|index| {
            ResultBrowserSelectionKey::Waveform(SourceWaveformPresentationKey::new(
                ticked,
                format!("V(n{index})"),
            ))
        })
        .collect();

    set_result_quantities_ticked(&mut state.results_parts(), &keys, true);
    // The frame the reader ticks in draws the strips before the view keeps
    // the ticks.
    assert_eq!(
        check(&mut state, "the tick's frame"),
        before,
        "the strip no tick touched is the one built before"
    );
    state.quick_frame();
    // The view keeping them changes none of the strips drawn from them.
    let drawn = models(&mut state);
    assert_eq!(drawn[strip(&drawn, other)].revision(), before);
    let shown = drawn[strip(&drawn, ticked)]
        .traces
        .iter()
        .filter(|trace| trace.visible)
        .count();
    assert_eq!(shown, SIGNALS, "every signal is drawn");
    drop(drawn);

    let view = state.working_view().expect("the run's view").id();
    view_edits::step(&mut state.results_parts(), view, view_edits::Step::Undo)
        .expect("the gesture is undone");
    state.quick_frame();
    let undone = check(&mut state, "undone");
    view_edits::step(&mut state.results_parts(), view, view_edits::Step::Redo)
        .expect("the gesture is redone");
    state.quick_frame();
    assert_eq!(
        check(&mut state, "redone"),
        undone,
        "the strip no step touched is the one built before"
    );
}

/// More rows than a block, so a read of a block is not a read of all: the
/// time grid and `V(out)` on it.
#[cfg(not(target_arch = "wasm32"))]
fn stored_rows() -> (Vec<f64>, Vec<f64>) {
    let rows = 3 * rspice_results::waveform::BLOCK_SAMPLES + 7;
    let time: Vec<f64> = (0..rows).map(|row| row as f64 * 1.0e-9).collect();
    let out = time.iter().map(|t| (t * 2.0e5).sin()).collect();
    (time, out)
}

/// A host with `analysis` as its one run, in volts, with `expressions` on
/// its strip; the analysis' key.
#[cfg(not(target_arch = "wasm32"))]
fn expression_host(
    mut analysis: AnalysisResult,
    expressions: &[&str],
) -> (ResultsTestHost, AnalysisPresentationKey) {
    analysis.waveforms = std::mem::take(&mut analysis.waveforms)
        .into_iter()
        .map(|trace| trace.with_unit("V"))
        .collect();
    let mut state = ResultsTestHost::default();
    state.model.simulation.start_run().add_analysis(analysis);
    state.model.simulation.complete_run();
    let run = state.model.simulation.active_run().expect("the run");
    let key = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    for text in expressions {
        state
            .results
            .results
            .add_expression_trace(&state.model.simulation, key, (*text).to_owned())
            .expect("the expression is added");
    }
    (state, key)
}

/// The stored run of [`stored_rows`] and the columns it reads, which count
/// the reads that take a column whole.
#[cfg(not(target_arch = "wasm32"))]
fn stored_run(
    time: &[f64],
    out: &[f64],
) -> (
    AnalysisResult,
    Vec<std::sync::Arc<crate::workbench::documents::result_document::test_fixtures::CountedColumn>>,
) {
    crate::workbench::documents::result_document::test_fixtures::stored_transient(
        time.to_vec(),
        vec![("V(out)".to_owned(), out.to_vec())],
        Some([8; 32]),
    )
}

/// The same samples held in memory.
#[cfg(not(target_arch = "wasm32"))]
fn held_run(time: &[f64], out: &[f64]) -> AnalysisResult {
    AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
        RetainedTrace::new("V(out)", time.to_vec(), out.to_vec(), "#55aaff"),
    ])
}

/// The expression curves the first strip resolves.
#[cfg(not(target_arch = "wasm32"))]
fn expression_curves(
    state: &mut ResultsTestHost,
) -> Vec<rspice_results_ui::waves::pane::ResolvedExpr> {
    let models = cached_models(
        &state.model.simulation,
        &mut state.results.results,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        &Tokens::default(),
    );
    resolve_strip_exprs(&mut state.results_parts(), &models[0], &Tokens::default())
}

/// One Waves frame, holding the jobs it starts while `hold`: the texts it
/// painted.
#[cfg(not(target_arch = "wasm32"))]
fn painted_frame(ctx: &egui::Context, state: &mut ResultsTestHost, hold: bool) -> Vec<String> {
    crate::workbench::results_job::hold_jobs_for_tests(hold);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, SCREEN)),
            ..Default::default()
        },
        |ctx| {
            crate::workbench::documents::result_document::prepare_viewer_state(
                &mut state.results_parts(),
            );
            egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut state.results_parts()));
        },
    );
    crate::workbench::results_job::hold_jobs_for_tests(false);
    output
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::Shape::Text(text) => Some(text.galley.text().to_owned()),
            _ => None,
        })
        .collect()
}

/// The job out for `text` on `key`, released if it was held.
#[cfg(not(target_arch = "wasm32"))]
fn released_job(
    state: &ResultsTestHost,
    key: AnalysisPresentationKey,
    text: &str,
) -> Option<
    crate::workbench::results_job::ResultsJob<
        crate::workbench::documents::result_document::ExpressionSeriesResult,
    >,
> {
    let job = state
        .results
        .results
        .expression_jobs
        .get(&(key, text.to_owned()))?
        .job
        .clone();
    job.release();
    Some(job)
}

/// Whether two series hold the same samples, bit for bit.
#[cfg(not(target_arch = "wasm32"))]
fn same_samples(
    left: &rspice_results::waveform::WaveformValues,
    right: &rspice_results::waveform::WaveformValues,
) -> bool {
    left.len() == right.len()
        && left
            .samples(..)
            .zip(right.samples(..))
            .all(|(left, right)| left.to_bits() == right.to_bits())
}

/// An expression over a stored run is evaluated on a Results job, a block at
/// a time: the frame that asks for it reads none of its operands whole and
/// names it pending on its pane, and the curve the job lands holds, sample
/// for sample, what the same samples held in memory and read whole give.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_expression_over_a_stored_run_is_evaluated_off_the_frame_a_block_at_a_time() {
    const TEXT: &str = "V(out) * 2 - avg(V(out))";
    let (time, out) = stored_rows();
    let (stored, columns) = stored_run(&time, &out);
    let (mut state, key) = expression_host(stored, &[TEXT]);
    let whole_reads = || {
        columns
            .iter()
            .map(|column| column.whole_reads())
            .sum::<u64>()
    };

    let painted = painted_frame(&new_context(), &mut state, true);
    assert_eq!(whole_reads(), 0, "the frame read an operand whole");
    assert!(
        painted.iter().any(|text| text.contains("evaluating")),
        "the pane names the expression pending: {painted:?}"
    );
    let job = released_job(&state, key, TEXT).expect("a job evaluates the expression");
    assert!(
        job.wait(std::time::Duration::from_secs(120)),
        "the job ends"
    );

    let landed = expression_curves(&mut state);
    assert_eq!(whole_reads(), 0, "the job read an operand whole");
    let (mut held, _) = expression_host(held_run(&time, &out), &[TEXT]);
    let whole = expression_curves(&mut held);
    assert_eq!((landed.len(), whole.len()), (1, 1));
    assert_eq!(landed[0].y.len(), time.len());
    assert!(
        same_samples(&landed[0].y, &whole[0].y),
        "the curve read in blocks is not the curve read whole"
    );
}

/// Dragging a cursor an expression over a stored run reads never leaves its
/// pane without the curve or marks it pending: each frame draws the curve
/// last landed while a job evaluates the new position, a job for a position
/// the cursor has left is dropped, never drawn, and the curve that lands
/// last is the one at the cursor's final position.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn dragging_a_cursor_an_expression_reads_keeps_its_curve_until_the_last_position_lands() {
    const TEXT: &str = "V(out) - xA";
    let (time, out) = stored_rows();
    let (stored, _) = stored_run(&time, &out);
    let (mut state, key) = expression_host(stored, &[TEXT]);
    let wait = |state: &ResultsTestHost| {
        if let Some(job) = released_job(state, key, TEXT) {
            assert!(
                job.wait(std::time::Duration::from_secs(120)),
                "the job ends"
            );
        }
    };
    state.results.results.session.cursors.a = Some(time[10]);
    assert!(
        expression_curves(&mut state).is_empty(),
        "nothing has landed"
    );
    wait(&state);
    assert_eq!(
        expression_curves(&mut state).len(),
        1,
        "the first curve lands"
    );

    let ctx = new_context();
    let steps = 40;
    for step in 0..steps {
        state.results.results.session.cursors.a = Some(time[100 + step * 997]);
        let painted = painted_frame(&ctx, &mut state, false);
        assert!(
            !painted.iter().any(|text| text.contains("evaluating")),
            "frame {step} marked the expression pending: {painted:?}"
        );
        assert_eq!(
            expression_curves(&mut state).len(),
            1,
            "frame {step} left the pane without the curve"
        );
    }
    // The job out when the drag stops may be for a position it left: that
    // one is dropped and the final position's is started in its place.
    for _ in 0..3 {
        wait(&state);
        assert_eq!(expression_curves(&mut state).len(), 1);
    }
    assert!(
        !state
            .results
            .results
            .expression_jobs
            .contains_key(&(key, TEXT.to_owned())),
        "the final position has landed"
    );
    let last = time[100 + (steps - 1) * 997];
    let landed = expression_curves(&mut state);
    let expected = rspice_results::waveform::WaveformValues::from(
        out.iter().map(|out| out - last).collect::<Vec<_>>(),
    );
    assert!(
        same_samples(&landed[0].y, &expected),
        "the curve drawn is not the one at the cursor's final position"
    );
}
