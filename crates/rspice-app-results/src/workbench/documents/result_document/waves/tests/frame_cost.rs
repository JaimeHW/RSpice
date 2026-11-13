//! Frame cost of the waveform strip on very large results.
//!
//! Two loads stand for the results that break a waveform viewer: a Monte
//! Carlo family, many members of one signal on one time grid, and one long
//! transient, few signals of millions of samples each. The gates count work
//! rather than time it: how many envelopes a frame rebuilt from the retained
//! samples. Wall clock on CI is noise; a rebuild count is not.

use super::*;

use std::io::Write as _;
use std::time::{Duration, Instant};

use rspice_ui_kit::plot::DecimationWork;

use crate::workbench::documents::result_document::frame_work::WorkCounts;

/// The window the measured frames are drawn into, as in the idle-frame gates.
const SCREEN: egui::Vec2 = egui::vec2(1_680.0, 1_020.0);

/// One transient analysis holding `traces` waveforms of `samples` points on
/// one shared time grid.
///
/// Each member is the same signal with its own gain and phase, as a Monte
/// Carlo trial is, so the members overlap the way a real family does and the
/// pane fits one Y range to all of them.
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

/// What one frame cost.
#[derive(Debug, Clone, Copy)]
struct FrameCost {
    /// Building the frame: layout, envelopes and shapes.
    build: Duration,
    /// Turning the frame's shapes into triangles.
    tessellate: Duration,
    vertices: usize,
    work: DecimationWork,
}

impl FrameCost {
    fn total(&self) -> Duration {
        self.build + self.tessellate
    }
}

fn new_context() -> egui::Context {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx
}

/// Run one frame of the waveform strip with `events` as its only input.
fn frame(ctx: &egui::Context, state: &mut ResultsTestHost, events: Vec<egui::Event>) -> FrameCost {
    let before = state.results.results.session.cache.work();
    let started = Instant::now();
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
    let build = started.elapsed();
    let started = Instant::now();
    let primitives = ctx.tessellate(output.shapes, output.pixels_per_point);
    let tessellate = started.elapsed();
    let vertices = primitives
        .iter()
        .map(|primitive| match &primitive.primitive {
            egui::epaint::Primitive::Mesh(mesh) => mesh.vertices.len(),
            egui::epaint::Primitive::Callback(_) => 0,
        })
        .sum();
    FrameCost {
        build,
        tessellate,
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

/// The cold frame, then `warm` frames with no input, then one frame per
/// hover position.
fn measure(
    state: &mut ResultsTestHost,
    warm: usize,
    hover: usize,
) -> (FrameCost, Vec<FrameCost>, Vec<FrameCost>) {
    let ctx = new_context();
    let cold = frame(&ctx, state, Vec::new());
    let warm = (0..warm).map(|_| frame(&ctx, state, Vec::new())).collect();
    let hover = hover_positions(hover)
        .into_iter()
        .map(|position| frame(&ctx, state, vec![egui::Event::PointerMoved(position)]))
        .collect();
    (cold, warm, hover)
}

/// Frames that each pan a half-width window one hundredth of the span on.
fn measure_pan(state: &mut ResultsTestHost, frames: usize) -> Vec<FrameCost> {
    let ctx = new_context();
    let _ = frame(&ctx, state, Vec::new());
    let model = &state
        .results
        .results
        .models
        .last_built()
        .expect("the first frame built the strip")[0];
    let analysis = model.analysis_key;
    let (x0, x1) = model.x_range.expect("the strip has an X extent");
    let span = x1 - x0;
    (0..frames)
        .map(|index| {
            let left = (index as f64 * 0.01).mul_add(span, x0);
            set_shared_x_view(
                &mut state.results.results.session,
                analysis,
                1,
                Some((left, 0.5f64.mul_add(span, left))),
            );
            frame(&ctx, state, Vec::new())
        })
        .collect()
}

fn report(load: &str, state: &mut ResultsTestHost) {
    fn line(label: &str, frames: &[FrameCost]) -> String {
        let mut totals: Vec<Duration> = frames.iter().map(FrameCost::total).collect();
        totals.sort_unstable();
        let median = totals[totals.len() / 2];
        let worst = totals[totals.len() - 1];
        let pick = frames
            .iter()
            .min_by_key(|cost| cost.total().abs_diff(median))
            .copied()
            .expect("at least one frame");
        let rebuilds: u64 = frames.iter().map(|cost| cost.work.rebuilds).sum();
        let evictions: u64 = frames.iter().map(|cost| cost.work.evictions).sum();
        format!(
            "  {label:5} median {:8.2} ms (build {:8.2}, tessellate {:8.2}) worst {:8.2} ms | \
             vertices {:9} | rebuilds/frame {:6.1} evictions/frame {:6.1}",
            median.as_secs_f64() * 1e3,
            pick.build.as_secs_f64() * 1e3,
            pick.tessellate.as_secs_f64() * 1e3,
            worst.as_secs_f64() * 1e3,
            pick.vertices,
            rebuilds as f64 / frames.len() as f64,
            evictions as f64 / frames.len() as f64,
        )
    }
    // What one full decimation of every trace costs on its own, at the
    // column count a 1 680-px strip asks for.
    let decimation = {
        let run = state
            .model
            .simulation
            .active_run()
            .expect("the fixture holds a run");
        let waveforms = &run.analyses[0].waveforms;
        let (x0, x1) = (waveforms[0].x[0], *waveforms[0].x.last().expect("samples"));
        let view = rspice_ui_kit::plot::TraceView {
            x0,
            x1,
            y0: -1.0,
            y1: 1.0,
            x_scale: rspice_ui_kit::plot::XScale::Linear,
            y_scale: rspice_ui_kit::plot::XScale::Linear,
            columns: 1_600,
            rows: 960,
        };
        let mut cache = rspice_ui_kit::plot::DecimationCache::default();
        let started = Instant::now();
        for (key, waveform) in waveforms.iter().enumerate() {
            let _ = cache.series(key as u64, &waveform.x, &waveform.y, view, false, None);
        }
        started.elapsed()
    };
    let walks = WorkCounts::reset();
    let (cold, warm, hover) = measure(state, 10, 20);
    let walks = walks.since();
    let pan = measure_pan(state, 10);
    let text = [
        load.to_owned(),
        line("cold", &[cold]),
        line("warm", &warm),
        line("hover", &hover),
        line("pan", &pan),
        format!(
            "  decimating every trace once: {:.2} ms | dataset walks {:?} | samples read {:?}",
            decimation.as_secs_f64() * 1e3,
            walks.nonzero(),
            walks.nonzero_samples()
        ),
    ]
    .join("\n");
    // Through a locked handle: the crate denies the printing macros.
    let mut out = std::io::stdout().lock();
    writeln!(out, "{text}").expect("the report is written");
}

/// Times the two large loads. Run in release, one load per process, under
/// `/usr/bin/time -l` for the peak resident set:
///
/// `cargo test --release -p rspice-app-results frame_cost::measure_ -- --ignored --nocapture`
#[test]
#[ignore = "measurement harness"]
fn measure_monte_carlo_family_frame_cost() {
    let mut state = crowded_transient(500, 100_000);
    report("500 traces x 100k points", &mut state);
}

#[test]
#[ignore = "measurement harness"]
fn measure_long_transient_frame_cost() {
    let mut state = crowded_transient(20, 5_000_000);
    report("20 traces x 5M points", &mut state);
}

/// The widest pane that is still stroked curve by curve.
#[test]
#[ignore = "measurement harness"]
fn measure_crowd_limit_frame_cost() {
    let mut state = crowded_transient(64, 100_000);
    report("64 traces x 100k points", &mut state);
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

/// Review pictures of a 500-member family: the strip as drawn, and with the
/// pointer over the family. Written only when `RSPICE_PICTURES` names a
/// folder.
#[test]
#[ignore = "writes review pictures"]
fn write_the_family_pictures() {
    /// The picture, in points; it is drawn at twice that, as a 2x display does.
    const SIZE: egui::Vec2 = egui::vec2(1280.0, 820.0);
    let Ok(directory) = std::env::var("RSPICE_PICTURES") else {
        return;
    };
    let mut state = crowded_transient(500, 4_000);
    let ctx = new_context();
    let draw = |state: &mut ResultsTestHost, events: Vec<egui::Event>| {
        let mut input = rspice_ui_kit::raster::display_input(SIZE, 2.0);
        input.events = events;
        ctx.run_ui(input, |ctx| {
            crate::workbench::documents::result_document::prepare_viewer_state(
                &mut state.results_parts(),
            );
            egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut state.results_parts()));
        })
    };
    let pointer = egui::Event::PointerMoved(egui::pos2(700.0, 300.0));
    for (name, events) in [("family", Vec::new()), ("family-pointer", vec![pointer])] {
        draw(&mut state, events);
        for _ in 0..4 {
            draw(&mut state, Vec::new());
        }
        let output = draw(&mut state, Vec::new());
        let canvas = rspice_ui_kit::raster::rasterize(&ctx, SIZE, &output);
        std::fs::write(
            format!("{directory}/{name}.png"),
            canvas.png(canvas.height()),
        )
        .expect("the picture is written");
    }
}
