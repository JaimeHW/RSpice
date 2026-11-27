//! Whole application frames over large data, timed: a transient of five million samples in
//! memory and in the results store, and a project of two hundred retained runs, each in Results
//! and on Design with the dock's Plot page in front. Each case reports its frames' median and
//! 99th percentile, gesture by gesture, the time until what a gesture draws or measures off the
//! frame is in, the process's peak working set and the stored columns it read whole.
//!
//! Run by hand, each case alone, so the peak working set is the case's own, for instance
//! `cargo test -p rspice-app --lib large_data::five_million_samples_in_results -- --ignored
//! --exact --nocapture` with the case's whole path.

use std::io::Write as _;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use crate::workbench::RSpiceApp;
use rspice_app_results::workbench::documents::result_document::document_edit::{
    commit_open_dialog, compare,
};
use rspice_app_results::workbench::documents::result_document::frame_work::WorkCounts;
use rspice_app_results::workbench::documents::result_document::test_fixtures::peak_working_set;
use rspice_app_runs::simulation::controller::test_execution::{
    run_manual_batch_session, run_manual_deck_in,
};
use rspice_app_state::state::{AnalysisResult, AnalysisType, RetainedTrace, SimulationRun};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_results::result_presentation::AnalysisPresentationKey;
use rspice_results::waveform::column_work::ColumnWork;
use rspice_results_ui::document_dialogs::ResultDocumentDialog;
use rspice_results_ui::document_dialogs::measurement::MeasurementDraft;
use rspice_results_ui::waves::stack::set_shared_x_view;
use rspice_ui_kit::plot::CursorPair;

/// The window the frames are laid out in.
const SIZE: egui::Vec2 = egui::vec2(1_680.0, 1_020.0);
/// The longest a gesture's off-frame work may take to come in.
const SETTLE_LIMIT: Duration = Duration::from_secs(300);

/// One whole application frame, as the shell paints it, built and tessellated.
fn app_frame(ctx: &egui::Context, app: &mut RSpiceApp) -> Duration {
    let started = Instant::now();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, SIZE)),
            ..Default::default()
        },
        |ui| {
            let ctx = ui.ctx().clone();
            app.prepare_frame(&ctx);
            app.render_frame_chrome(ui);
            app.render_frame_dialogs(&ctx);
        },
    );
    let _ = ctx.tessellate(output.shapes, output.pixels_per_point);
    started.elapsed()
}

/// Frames until nothing is drawn or measured off the frame: how long the gesture's picture
/// took to come in.
fn settle(ctx: &egui::Context, app: &mut RSpiceApp) -> Duration {
    let started = Instant::now();
    loop {
        app_frame(ctx, app);
        let busy = rspice_ui_kit::plot::drawing_off_frame(ctx)
            || rspice_results_ui::waves::readout::measuring_off_frame(ctx)
            || app.state.results.results.validating_off_frame()
            || app.state.platform.results_save.is_some();
        if !busy {
            return started.elapsed();
        }
        assert!(started.elapsed() < SETTLE_LIMIT, "the frame settled");
        std::thread::sleep(Duration::from_millis(1));
    }
}

/// The frames and settling times of one case, gesture by gesture, and the walks of the
/// dataset each gesture's frame made.
#[derive(Default)]
struct Timings {
    frames: Vec<(&'static str, Vec<Duration>)>,
    settled: Vec<(&'static str, Duration)>,
    walks: Vec<(&'static str, String)>,
}

impl Timings {
    fn frame(&mut self, gesture: &'static str, took: Duration) {
        match self.frames.iter_mut().find(|(name, _)| *name == gesture) {
            Some((_, frames)) => frames.push(took),
            None => self.frames.push((gesture, vec![took])),
        }
    }

    /// One gesture's frame, then the time until its picture is in.
    fn gesture(
        &mut self,
        gesture: &'static str,
        ctx: &egui::Context,
        app: &mut RSpiceApp,
    ) -> Duration {
        let work = WorkCounts::reset();
        let took = app_frame(ctx, app);
        let walks = work.since().nonzero();
        if !walks.is_empty() && !self.walks.iter().any(|(name, _)| *name == gesture) {
            self.walks.push((gesture, format!("{walks:?}")));
        }
        self.frame(gesture, took);
        let settled = settle(ctx, app);
        match self.settled.iter_mut().find(|(name, _)| *name == gesture) {
            Some((_, slowest)) => *slowest = (*slowest).max(settled),
            None => self.settled.push((gesture, settled)),
        }
        took
    }

    fn report(&self, case: &str, columns: &ColumnWork) {
        let millis = |duration: Duration| duration.as_secs_f64() * 1e3;
        let quantiles = |frames: &[Duration]| {
            let mut sorted = frames.to_vec();
            sorted.sort_unstable();
            let at = |q: f64| sorted[((q * sorted.len() as f64).ceil() as usize).max(1) - 1];
            (millis(at(0.5)), millis(at(0.99)))
        };
        let mut text = format!("{case}\n");
        for (gesture, frames) in &self.frames {
            let (p50, p99) = quantiles(frames);
            text.push_str(&format!(
                "  {gesture}: {} frames, p50 {p50:.1} ms, p99 {p99:.1} ms\n",
                frames.len()
            ));
        }
        let all: Vec<Duration> = self
            .frames
            .iter()
            .flat_map(|(_, frames)| frames.iter().copied())
            .collect();
        let (p50, p99) = quantiles(&all);
        text.push_str(&format!(
            "  every frame: {} frames, p50 {p50:.1} ms, p99 {p99:.1} ms\n",
            all.len()
        ));
        for (gesture, settled) in &self.settled {
            text.push_str(&format!(
                "  {gesture}: in after {:.1} ms at the slowest\n",
                millis(*settled)
            ));
        }
        for (gesture, walks) in &self.walks {
            text.push_str(&format!("  {gesture} walked {walks}\n"));
        }
        let peak = peak_working_set().map_or_else(
            || "unmeasured".to_owned(),
            |bytes| format!("{:.0} MiB", bytes as f64 / f64::from(1 << 20)),
        );
        text.push_str(&format!(
            "  peak working set {peak} | stored columns read whole {}\n",
            columns.stored_reads
        ));
        // Through a locked handle: the crate denies the printing macros.
        writeln!(std::io::stdout().lock(), "{text}").expect("the report is written");
    }
}

/// Where a case is drawn: the Results workspace, or Design with the dock's Plot page in front.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Place {
    Results,
    PlotPage,
}

impl Place {
    fn show(self, app: &mut RSpiceApp) {
        match self {
            Self::Results => app.state.workbench.activate(Workspace::Results),
            Self::PlotPage => {
                app.state.workbench.activate(Workspace::Design);
                app.state.workbench.show_plot_page();
            }
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::Results => "Results",
            Self::PlotPage => "Design, the dock's Plot page in front",
        }
    }
}

/// The analysis shown and its strip's X extent.
fn shown(app: &RSpiceApp) -> (AnalysisPresentationKey, f64, f64) {
    let run = app
        .state
        .model
        .simulation
        .active_run()
        .expect("a run is shown");
    let analysis = &run.analyses[0];
    let (x0, x1) = analysis.waveforms[0].x.ends().expect("rows");
    (
        AnalysisPresentationKey::new(run.dataset_id, analysis),
        x0,
        x1,
    )
}

#[test]
#[ignore = "times whole frames over five million samples; run by hand"]
fn five_million_samples_in_results() {
    five_million_samples(Place::Results, Held::Memory);
}

#[test]
#[ignore = "times whole frames over five million samples; run by hand"]
fn five_million_samples_on_the_plot_page() {
    five_million_samples(Place::PlotPage, Held::Memory);
}

#[test]
#[ignore = "times whole frames over five million samples; run by hand"]
fn five_million_stored_samples_in_results() {
    five_million_samples(Place::Results, Held::Store);
}

#[test]
#[ignore = "times whole frames over five million samples; run by hand"]
fn five_million_stored_samples_on_the_plot_page() {
    five_million_samples(Place::PlotPage, Held::Store);
}

/// Where a run's samples are held: in memory, as a run the store refused keeps them, or in the
/// results store, as a desktop run keeps them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Held {
    Memory,
    Store,
}

/// A transient of five million samples, four signals of 1.25 million rows, `held` where it says
/// and shown in `place`: opened, panned, zoomed, its cursor dragged, and a measurement added.
fn five_million_samples(place: Place, held: Held) {
    const ROWS: usize = 1_250_000;
    const SIGNALS: usize = 4;
    let x: Vec<f64> = (0..ROWS).map(|row| row as f64 * 1.0e-9).collect();
    let waveforms = (0..SIGNALS)
        .map(|signal| {
            let rate = 1.0e5 * (signal + 1) as f64;
            let y: Vec<f64> = x.iter().map(|time| (time * rate).sin()).collect();
            RetainedTrace::new(format!("V(s{signal})"), x.clone(), y, "#fff")
        })
        .collect();
    drop(x);
    let mut analysis =
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(waveforms);
    if held == Held::Store {
        rspice_app_state::state::run_store::store_analysis(&mut analysis.data)
            .expect("the store keeps the samples");
    }
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    // What the open frame's walks of the whole record cost on their own.
    let started = Instant::now();
    let _ = run.dataset_content_digest();
    let digest = started.elapsed();
    let started = Instant::now();
    let _ = run.analyses[0].validate_retained_evidence();
    let validation = started.elapsed();
    writeln!(
        std::io::stdout().lock(),
        "the run's digest alone {:.1} ms, its evidence validation alone {:.1} ms",
        digest.as_secs_f64() * 1e3,
        validation.as_secs_f64() * 1e3
    )
    .expect("the line is written");

    let mut app = RSpiceApp::test_instance();
    app.state.model.simulation.retained.runs = vec![run].into();
    assert!(app.state.model.simulation.select_run(0));
    assert!(app.state.model.simulation.select_analysis(0));
    place.show(&mut app);
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let columns = ColumnWork::read();
    let mut timings = Timings::default();

    timings.gesture("open", &ctx, &mut app);
    let (key, x0, x1) = shown(&app);
    let span = x1 - x0;
    for index in 0..20 {
        let left = (f64::from(index) * 0.02).mul_add(span, x0);
        let session = &mut app.state.results.results.session;
        set_shared_x_view(session, key, Some((left, 0.01f64.mul_add(span, left))));
        timings.gesture("pan", &ctx, &mut app);
    }
    let anchor = 0.37f64.mul_add(span, x0);
    for power in 0..8 {
        let half = 0.5 * span / 10f64.powi(power);
        let session = &mut app.state.results.results.session;
        set_shared_x_view(session, key, Some((anchor - half, anchor + half)));
        timings.gesture("zoom", &ctx, &mut app);
    }
    set_shared_x_view(&mut app.state.results.results.session, key, None);
    timings.gesture("fit", &ctx, &mut app);
    for step in 0..40 {
        let at = (f64::from(step) / 40.0).mul_add(span, x0);
        let session = &mut app.state.results.results.session;
        session.cursor_strip = Some(0);
        session.cursors = CursorPair {
            a: Some(at),
            b: None,
        };
        // A drag is a frame per pointer move; the picture under it is not drawn again.
        let took = app_frame(&ctx, &mut app);
        timings.frame("cursor drag", took);
    }
    app.state.results.result_document_dialog =
        Some(ResultDocumentDialog::Measurement(MeasurementDraft {
            expression: "max(V(s0))".to_owned(),
            ..MeasurementDraft::default()
        }));
    let started = Instant::now();
    commit_open_dialog(&mut app.state.results_parts());
    timings.frame("add measurement (commit)", started.elapsed());
    timings.gesture("add measurement", &ctx, &mut app);
    for _ in 0..20 {
        let took = app_frame(&ctx, &mut app);
        timings.frame("idle", took);
    }
    timings.report(
        &format!(
            "five million samples {}, {}",
            match held {
                Held::Memory => "in memory",
                Held::Store => "in the results store",
            },
            place.label()
        ),
        &columns.since(),
    );
}

/// The deck of run `index` of the two hundred: an RC whose resistor each run steps. The newest
/// two are one deck, so the newest has a baseline of its own analysis to compare against.
fn deck(index: usize) -> String {
    format!(
        "two hundred runs, run {index}\nV1 in 0 PULSE(0 1 0 1u 1u 50u 100u)\nR1 in out {}\nC1 \
         out 0 10n\n.tran 0.1u 300u\n.end\n",
        1_000 + 10 * index
    )
}

#[test]
#[ignore = "solves and times two hundred runs; run by hand"]
fn two_hundred_runs_in_results() {
    two_hundred_runs(Place::Results);
}

#[test]
#[ignore = "solves and times two hundred runs; run by hand"]
fn two_hundred_runs_on_the_plot_page() {
    two_hundred_runs(Place::PlotPage);
}

/// A project of two hundred stored runs, saved and opened again, shown in `place`: the project
/// opened, its data browser drawn, runs switched between, and, where Compare is offered, the
/// newest compared against a baseline.
fn two_hundred_runs(place: Place) {
    const RUNS: usize = 200;
    let folder = std::env::temp_dir().join(format!("rspice-two-hundred-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&folder).unwrap();
    let file: PathBuf = folder.join("two-hundred.rspiceproj");
    {
        let mut state: crate::workbench::app_state::AppState = run_manual_batch_session(&deck(0));
        state.open_project_session();
        state.model.simulation.retained.retained_dataset_limit = Some(RUNS);
        for index in 1..RUNS {
            let _ = run_manual_deck_in(&mut state, &deck(index.min(RUNS - 2)));
        }
        assert_eq!(state.model.simulation.retained.runs.len(), RUNS);
        assert!(state.save_project_to_path(&file), "the project saves");
    }

    let mut app = RSpiceApp::test_instance();
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let columns = ColumnWork::read();
    let mut timings = Timings::default();
    let started = Instant::now();
    assert!(app.state.load_project_from_path(&file), "the project opens");
    timings.frame("open the project (load)", started.elapsed());
    assert_eq!(app.state.model.simulation.retained.runs.len(), RUNS);
    place.show(&mut app);
    assert!(app.state.model.simulation.select_run(0));
    timings.gesture("open the project (first frame)", &ctx, &mut app);
    for _ in 0..20 {
        let took = app_frame(&ctx, &mut app);
        timings.frame("data browser, idle", took);
    }
    for step in 0..40 {
        assert!(app.state.model.simulation.select_run(step * (RUNS / 40)));
        timings.gesture("switch run", &ctx, &mut app);
    }

    assert!(app.state.model.simulation.select_run(0));
    timings.gesture("show the newest", &ctx, &mut app);
    let offered = compare::compare_available(app.state.results_reads());
    if !offered {
        writeln!(
            std::io::stdout().lock(),
            "compare not offered: {:?}",
            compare::compare_availability(app.state.results_reads())
        )
        .expect("the line is written");
    }
    if place == Place::Results && offered {
        compare::open_compare(&mut app.state.results_parts());
        let Some(ResultDocumentDialog::Compare(mut draft)) =
            app.state.results.result_document_dialog.clone()
        else {
            panic!("Compare datasets opens its form");
        };
        let choices = compare::choices(app.state.results_reads(), &mut draft);
        assert_eq!(choices.refusal, None);
        app.state.results.result_document_dialog = Some(ResultDocumentDialog::Compare(draft));
        let started = Instant::now();
        commit_open_dialog(&mut app.state.results_parts());
        compare::settle_comparison(&mut app.state.results_parts());
        timings.frame("compare (commit and settle)", started.elapsed());
        timings.gesture("compare", &ctx, &mut app);
    }
    for _ in 0..20 {
        let took = app_frame(&ctx, &mut app);
        timings.frame("idle", took);
    }
    timings.report(
        &format!("two hundred stored runs, {}", place.label()),
        &columns.since(),
    );
    drop(app);
    let _ = std::fs::remove_dir_all(&folder);
}
