//! The idle-frame gate: steady-state Results frames must be O(visible).
//!
//! Every surface here is driven through its real `show` on a deliberately
//! large retained dataset, for ten consecutive frames with identical input
//! and no state change in between. What the gate asserts is not a duration —
//! wall clock on CI is noise — but a count: how many times those ten frames
//! walked the complete dataset. The answer has to be zero. The first frame is
//! allowed to build whatever it needs; frames two onward are the product's
//! steady state, and a reader who is not touching anything is entitled to
//! have nothing recomputed.

use std::collections::BTreeMap;

use super::{DatasetWalk, FrameSampleRead, WorkCounts};
// Through the module under measurement rather than around it: this drives the
// Results workspace's own surfaces with the session type that workspace
// already holds, and does not reach for the session aggregate on its own.
use super::super::{
    AppState, ResultViewer, eye, manifest, op_inspector, optimization, sensitivity, soa, table,
    waves,
};
use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisType, DcOpResult,
    OperatingPointValue, SimulationRun, SoaEvaluationEvidence, SoaParameterEvidence,
    SoaRuleVerdictEvidence, SoaViolationEvidence, SoaViolationSeverityEvidence, WaveformData,
};

/// Retained samples per transient waveform.
const TRANSIENT_SAMPLES: usize = 250_000;
/// Transient waveforms; the product of the two is the headline dataset size.
const TRANSIENT_TRACES: usize = 4;
/// Retained node-voltage rows on the operating point.
const OP_NODES: usize = 20_000;
/// Retained per-device rows on the operating point.
const OP_DEVICES: usize = 20_000;
/// Evaluated SOA rules, each with its own retained stress history.
const SOA_RULES: usize = 400;
/// Retained samples in each SOA stress history.
const SOA_SAMPLES: usize = 2_500;
/// Folded eye acquisitions.
const EYE_TRACES: usize = 500;
/// Samples per folded acquisition.
const EYE_SAMPLES: usize = 400;

/// Ranked sensitivity parameters.
const SENSITIVITY_PARAMETERS: usize = 5_000;
/// Retained optimizer candidates.
const OPTIMIZATION_ITERATIONS: usize = 5_000;

/// Frames measured after the first. Ten is enough that a per-frame walk
/// cannot hide behind an every-other-frame schedule.
const IDLE_FRAMES: usize = 10;

fn transient_analysis() -> AnalysisResult {
    let x: Vec<f64> = (0..TRANSIENT_SAMPLES)
        .map(|index| index as f64 * 1.0e-9)
        .collect();
    let waveforms = (0..TRANSIENT_TRACES)
        .map(|trace| {
            let y: Vec<f64> = (0..TRANSIENT_SAMPLES)
                .map(|index| ((index + trace) as f64 * 1.0e-3).sin())
                .collect();
            WaveformData::new(format!("V(n{trace})"), x.clone(), y, "#00aaff")
        })
        .collect();
    AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(waveforms)
}

fn operating_point_analysis() -> AnalysisResult {
    let dc = DcOpResult {
        node_voltages: (0..OP_NODES)
            .map(|index| OperatingPointValue {
                name: format!("V(x{}.n{index})", index % 32),
                value: index as f64 * 1.0e-3,
                unit: "V".to_owned(),
            })
            .collect(),
        ..DcOpResult::default()
    };
    let report = rspice_core::circuit::DeviceOpReport {
        entries: (0..OP_DEVICES)
            .map(|index| rspice_core::circuit::DeviceOpEntry {
                name: format!("x{}.m{index}", index % 32),
                device_kind: "MOSFET",
                region: Some("saturation"),
                params: vec![
                    ("id", index as f64 * 1.0e-6),
                    ("vgs", 1.2),
                    ("vds", 1.8),
                    ("gm", 1.0e-3),
                ],
            })
            .collect(),
    };
    AnalysisResult::new(2, AnalysisType::DcOp, "OP")
        .with_dc_op(dc)
        .with_device_op(report)
}

fn soa_analysis() -> AnalysisResult {
    let time: Vec<f64> = (0..SOA_SAMPLES)
        .map(|index| index as f64 * 1.0e-12)
        .collect();
    let mut waveforms = Vec::with_capacity(SOA_RULES);
    let mut evaluations = Vec::with_capacity(SOA_RULES);
    let mut violations = Vec::with_capacity(SOA_RULES);
    for rule in 0..SOA_RULES {
        let device_id = format!("M{rule:04}");
        // The worst sample is the last one, and every other sample sits below
        // it, which is what the stress-history verifier checks for.
        let y: Vec<f64> = (0..SOA_SAMPLES)
            .map(|index| 3.0 * (index as f64 + 1.0) / SOA_SAMPLES as f64)
            .collect();
        let worst_actual_value = y[SOA_SAMPLES - 1];
        let worst_time_s = time[SOA_SAMPLES - 1];
        waveforms.push(WaveformData::new(
            crate::services::safety::soa_stress_waveform_name(
                &device_id,
                crate::services::safety::SoAParameter::Vds,
            ),
            time.clone(),
            y,
            "#00aaff",
        ));
        evaluations.push(SoaEvaluationEvidence {
            device_id: device_id.clone(),
            parameter: SoaParameterEvidence::DrainSourceVoltage,
            limit_value: 3.3,
            worst_actual_value,
            worst_time_s,
            sample_count: SOA_SAMPLES as u64,
            unit: "V".to_owned(),
            description: "Maximum drain-source voltage".to_owned(),
            verdict: SoaRuleVerdictEvidence::Warning,
        });
        violations.push(SoaViolationEvidence {
            device_id,
            parameter: SoaParameterEvidence::DrainSourceVoltage,
            limit_value: 3.3,
            actual_value: worst_actual_value,
            time_s: worst_time_s,
            severity: SoaViolationSeverityEvidence::Warning,
        });
    }
    AnalysisResult::new(3, AnalysisType::Soa, "SOA")
        .with_family_metadata(AnalysisResultFamilyMetadata::Soa { time })
        .with_waveforms(waveforms)
        .with_result_payload(AnalysisResultPayload::Soa {
            evaluations,
            violations,
        })
}

fn sensitivity_analysis() -> AnalysisResult {
    AnalysisResult::new(4, AnalysisType::Sensitivity, "SENS").with_result_payload(
        AnalysisResultPayload::Sensitivity {
            output: "V(out)".to_owned(),
            result_mode: crate::state::SensitivityResultMode::Dc,
            rows: (0..SENSITIVITY_PARAMETERS)
                .map(|index| crate::state::SensitivityResultRow {
                    parameter: format!("p{index:06}"),
                    raw: index as f64,
                    normalized: (index as f64).sin(),
                })
                .collect(),
        },
    )
}

fn optimization_analysis() -> AnalysisResult {
    let axis: Vec<f64> = (0..OPTIMIZATION_ITERATIONS)
        .map(|index| index as f64)
        .collect();
    let cost: Vec<f64> = (0..OPTIMIZATION_ITERATIONS)
        .map(|index| 1.0 / (index as f64 + 1.0))
        .collect();
    let gain: Vec<f64> = (0..OPTIMIZATION_ITERATIONS)
        .map(|index| index as f64 * 0.5)
        .collect();
    AnalysisResult::new(5, AnalysisType::Optimization, "OPT")
        .with_family_metadata(AnalysisResultFamilyMetadata::Optimization {
            iterations: axis.clone(),
            best_cost: cost[OPTIMIZATION_ITERATIONS - 1],
            best_variables: std::collections::BTreeMap::from([(
                "GAIN".to_owned(),
                gain[OPTIMIZATION_ITERATIONS - 1],
            )]),
            converged: true,
        })
        .with_waveforms(vec![
            WaveformData::new("OPT_COST", axis.clone(), cost, "#0af"),
            WaveformData::new("OPT_GAIN", axis, gain, "#fa0"),
        ])
}

/// One run carrying every dataset the measured surfaces read.
fn large_state() -> AppState {
    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.add_analysis(transient_analysis());
    run.add_analysis(operating_point_analysis());
    run.add_analysis(soa_analysis());
    run.add_analysis(sensitivity_analysis());
    run.add_analysis(optimization_analysis());
    state.simulation.runs = vec![run];
    assert!(state.simulation.select_run(0));

    let mut eye = crate::analysis::eye_diagram::EyeData::new(1.0e-9, 2);
    for trace in 0..EYE_TRACES {
        let time: Vec<f64> = (0..EYE_SAMPLES)
            .map(|index| index as f64 * 2.0 / EYE_SAMPLES as f64)
            .collect();
        let amplitude: Vec<f64> = (0..EYE_SAMPLES)
            .map(|index| ((index + trace) as f64 * 0.05).sin())
            .collect();
        eye.add_trace(crate::analysis::eye_diagram::EyeTrace::new(time, amplitude));
    }
    state.analysis.eye_diagram_state.load_data(eye);
    state
}

/// Select the analysis a surface speaks for, by retained analysis type.
fn select_analysis(state: &mut AppState, analysis_type: AnalysisType) {
    let index = state
        .simulation
        .active_run()
        .expect("the fixture selects a run")
        .analyses
        .iter()
        .position(|analysis| analysis.analysis_type == analysis_type)
        .expect("the fixture retains this analysis type");
    assert!(state.simulation.select_analysis(index));
}

/// Run one frame of `surface` against an unchanged state and identical input.
fn frame(
    ctx: &egui::Context,
    state: &mut AppState,
    surface: &mut dyn FnMut(&mut egui::Ui, &mut AppState),
) {
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_680.0, 1_020.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| surface(ui, state));
        },
    );
}

/// Work counted across [`IDLE_FRAMES`] frames that follow a first frame.
fn steady_state_work(
    state: &mut AppState,
    mut surface: impl FnMut(&mut egui::Ui, &mut AppState),
) -> WorkCounts {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    // The first frame is allowed to build the memos the rest read.
    frame(&ctx, state, &mut surface);
    let baseline = WorkCounts::reset();
    for _ in 0..IDLE_FRAMES {
        frame(&ctx, state, &mut surface);
    }
    baseline.since()
}

/// Every measured surface, named for the failure message.
fn surfaces() -> Vec<(&'static str, ResultViewer)> {
    vec![
        ("Manifest", ResultViewer::Manifest),
        ("OP inspector", ResultViewer::Op),
        ("Table", ResultViewer::Table),
        ("SOA", ResultViewer::Soa),
        ("Waves", ResultViewer::Waves),
        ("Eye", ResultViewer::Eye),
        ("Sensitivity", ResultViewer::Contribution),
        ("Optimization", ResultViewer::Optimization),
    ]
}

fn show_surface(viewer: ResultViewer, ui: &mut egui::Ui, state: &mut AppState) {
    state.ui.results.viewer = viewer;
    // The document bar states the sheet's purpose on every frame, including
    // whether the retained evidence validated, and the tab strip decides on
    // every frame which sheets to offer. Both are part of the frame, and both
    // used to be where the dataset walks were.
    let _ = super::super::sheet_purpose(state);
    for candidate in ResultViewer::PRIMARY
        .into_iter()
        .chain(ResultViewer::DATASET_NATIVE)
    {
        let _ = super::super::viewer_availability(state, candidate);
    }
    match viewer {
        ResultViewer::Manifest => manifest::show(ui, state),
        ResultViewer::Op => op_inspector::show(ui, state),
        ResultViewer::Table => table::show(ui, state),
        ResultViewer::Soa => soa::show(ui, state),
        ResultViewer::Waves => waves::show(ui, state),
        ResultViewer::Eye => eye::show(ui, state),
        ResultViewer::Contribution => {
            sensitivity::show(ui, state);
            sensitivity::right_panel(ui, state);
        }
        ResultViewer::Optimization => {
            optimization::show(ui, state);
            optimization::right_panel(ui, state);
        }
        other => panic!("{other:?} is not a measured surface"),
    }
}

fn state_for(viewer: ResultViewer) -> AppState {
    let mut state = large_state();
    match viewer {
        ResultViewer::Op => select_analysis(&mut state, AnalysisType::DcOp),
        ResultViewer::Soa => select_analysis(&mut state, AnalysisType::Soa),
        ResultViewer::Contribution => select_analysis(&mut state, AnalysisType::Sensitivity),
        ResultViewer::Optimization => select_analysis(&mut state, AnalysisType::Optimization),
        _ => select_analysis(&mut state, AnalysisType::Transient),
    }
    if viewer == ResultViewer::Waves {
        add_transient_expression(&mut state);
    }
    state
}

/// Put one visible expression on the transient strip.
///
/// An expression is a second retained series the strip has to fit its axis to
/// on every frame, and resolving a strip happens twice per frame — so it is
/// where a per-frame full scan hides most easily. Without one on the fixture,
/// every gate here is measuring a Waves sheet the product rarely shows.
fn add_transient_expression(state: &mut AppState) {
    let run = state
        .simulation
        .active_run()
        .expect("the fixture selects a run");
    let analysis = run
        .analyses
        .iter()
        .find(|analysis| analysis.analysis_type == AnalysisType::Transient)
        .expect("the fixture retains a transient analysis");
    let key = super::super::AnalysisPresentationKey::new(run.dataset_id, analysis);
    state.ui.results.analysis_exprs.insert(
        key,
        vec![super::super::ExprTrace {
            text: "V(n0) * 2".to_owned(),
            visible: true,
        }],
    );
}

/// The permanent gate.
///
/// Ten consecutive frames with identical input and no state change between
/// them, on a dataset large enough that any whole-dataset work would be
/// unmissable. A surface that recomputes anything dataset-sized on an idle
/// frame fails here, named, with the class of work it repeated and how many
/// times it repeated it.
#[test]
fn idle_frames_do_no_whole_dataset_work_on_any_results_surface() {
    let mut offenders = BTreeMap::new();
    for (name, viewer) in surfaces() {
        let mut state = state_for(viewer);
        let work = steady_state_work(&mut state, |ui, state| show_surface(viewer, ui, state));
        if work.total() > 0 {
            offenders.insert(name, work.nonzero());
        }
    }
    assert!(
        offenders.is_empty(),
        "these Results surfaces repeated whole-dataset work across {IDLE_FRAMES} idle frames \
         (surface -> [(work, times)]):\n{offenders:#?}"
    );
}

/// The measurement is only meaningful if the counters can see the work
/// at all. Frame one is the proof: it has no memos to read, so it must count
/// the walks the steady state then refuses to repeat.
#[test]
fn the_first_frame_counts_the_work_the_idle_frames_must_not_repeat() {
    let mut seen = BTreeMap::new();
    for (name, viewer) in surfaces() {
        let mut state = state_for(viewer);
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let baseline = WorkCounts::reset();
        frame(&ctx, &mut state, &mut |ui, state| {
            show_surface(viewer, ui, state);
        });
        seen.insert(name, baseline.since().total());
    }
    for (name, count) in &seen {
        assert!(
            *count > 0,
            "the first {name} frame counted no whole-dataset work at all, so the idle-frame \
             gate for it is vacuous: {seen:#?}"
        );
    }
}

/// A memo that can serve stale data is worse than the cost it saved. Every
/// surface must show the new truth after the dataset behind it changes.
#[test]
fn a_new_data_version_rebuilds_every_memo() {
    for (name, viewer) in surfaces() {
        let mut state = state_for(viewer);
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        frame(&ctx, &mut state, &mut |ui, state| {
            show_surface(viewer, ui, state);
        });

        state.simulation.data_version = state.simulation.data_version.wrapping_add(1);
        let baseline = WorkCounts::reset();
        frame(&ctx, &mut state, &mut |ui, state| {
            show_surface(viewer, ui, state);
        });
        assert!(
            baseline.since().total() > 0,
            "{name} served its memo across a data-version bump without recomputing anything"
        );
    }
}

/// Around-cursor mode maps two cursors and a window centre onto the retained
/// grid on every frame. On a million-sample transient that was three scans of
/// the whole grid per frame, for a cursor the reader had already placed.
#[test]
fn the_table_maps_a_cursor_without_scanning_the_grid() {
    let mut state = state_for(ResultViewer::Table);
    state.ui.results.table.around_cursor = true;
    state.ui.results.cursors.place(1.0e-4);
    state.ui.results.cursors.place(2.0e-4);
    state.ui.results.cursor_strip = Some(0);

    let work = steady_state_work(&mut state, |ui, state| {
        show_surface(ResultViewer::Table, ui, state);
    });
    assert_eq!(
        work.get(DatasetWalk::TableCursorScan),
        0,
        "the table scanned the retained grid to place a cursor that had not moved"
    );
}

/// The retained-evidence verdict is a memo, not a snapshot.
///
/// Corrupting the evidence and declaring a new dataset generation has to
/// close every gate that reads it — the sheet bar's caution and the SOA
/// availability gate both come from the one memo, so both must move.
#[test]
fn corrupted_evidence_closes_the_gates_that_read_the_memo() {
    let mut state = state_for(ResultViewer::Soa);
    state.ui.results.viewer = ResultViewer::Soa;
    assert!(soa::active_payload_is_valid(&state));
    let purpose = super::super::sheet_purpose(&state);
    assert!(
        !purpose.contains("failed validation"),
        "the fixture starts out valid, but the bar says: {purpose}"
    );

    // A retained waveform with more coordinates than values is exactly what
    // `validate_retained_evidence` exists to refuse.
    let analysis = state.simulation.runs[0]
        .analyses
        .iter_mut()
        .find(|analysis| analysis.analysis_type == AnalysisType::Soa)
        .expect("the fixture retains an SOA analysis");
    let mut shortened = analysis.waveforms[0].y.as_ref().clone();
    shortened.pop();
    analysis.waveforms[0].y = std::sync::Arc::new(shortened);
    state.simulation.data_version = state.simulation.data_version.wrapping_add(1);

    assert!(
        !soa::active_payload_is_valid(&state),
        "the SOA gate served a verdict from the previous dataset generation"
    );
    let purpose = super::super::sheet_purpose(&state);
    assert!(
        purpose.contains("the retained evidence failed validation"),
        "the sheet bar kept the old verdict: {purpose}"
    );
}

/// Report the counts rather than asserting on them, for tuning the fixes.
/// Not a gate: `--ignored` keeps it out of the ordinary run.
///
/// ```text
/// cargo test -p rspice-ui --lib report_idle_frame_work -- --ignored --nocapture
/// ```
// The crate denies `print_stdout` because the desktop build detaches from its
// console and the browser build has no stderr, so a stray diagnostic reaches
// nobody. This is the same exception `services::license` already carries: an
// `#[ignore]`d operator entry point whose entire output *is* the printed
// table, read through `--nocapture`. Routing it through `log` would put the
// table behind a logger no test installs. The allow is per-function so the
// rule keeps holding for every other test in the crate.
#[test]
#[allow(clippy::print_stdout)]
#[ignore = "measurement harness"]
fn report_idle_frame_work() {
    for (name, viewer) in surfaces() {
        let mut state = state_for(viewer);
        let first = WorkCounts::reset();
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        frame(&ctx, &mut state, &mut |ui, state| {
            show_surface(viewer, ui, state);
        });
        let first = first.since();
        let steady = steady_state_work(&mut state, |ui, state| show_surface(viewer, ui, state));
        println!(
            "{name:14} first frame {:5} | {IDLE_FRAMES} idle frames {:6} | idle detail {:?}",
            first.total(),
            steady.total(),
            steady.nonzero()
        );
    }
}

/// Retained samples one surface may read across [`IDLE_FRAMES`] steady frames.
///
/// What is drawn sets it. The only surface that legitimately reads the dataset
/// on a steady frame is the overview lane under a Waves strip, which draws a
/// fixed 160 points (plus the partial last stride) per strip; this fixture has
/// two time-domain strips, so 3 280 over ten frames. Four thousand leaves room
/// for a third strip and is still three orders of magnitude under the
/// 5 050 000 an unmemoized lane read.
const STEADY_SAMPLE_CEILING_PER_SURFACE: u64 = 4_000;

/// The second gate: a steady frame may read the dataset, but only as much of
/// it as it draws.
///
/// The walk gate above cannot see this. Nothing here walks a *complete*
/// dataset — an overview lane reads one trace of several, an axis fit reads
/// one series — so it counted zero while the Waves sheet spent most of its
/// frame reading a quarter of a million samples to place a hundred and sixty
/// points. What makes the difference visible is counting the samples.
#[test]
fn no_steady_frame_reads_more_of_the_dataset_than_it_draws() {
    let mut reads = BTreeMap::new();
    for (name, viewer) in surfaces() {
        let mut state = state_for(viewer);
        let work = steady_state_work(&mut state, |ui, state| show_surface(viewer, ui, state));
        if work.total_samples() > STEADY_SAMPLE_CEILING_PER_SURFACE {
            reads.insert(name, (work.total_samples(), work.nonzero_samples()));
        }
    }
    assert!(
        reads.is_empty(),
        "over {IDLE_FRAMES} idle frames these surfaces read more than \
         {STEADY_SAMPLE_CEILING_PER_SURFACE} retained samples, which is more than they draw \
         (surface -> (samples, [(class, samples)])):\n{reads:#?}"
    );
}

/// The sample counter is only worth having if it can see the work. The first
/// frame proves it: with no memo to read, it must count the reads the steady
/// frames then refuse to repeat at that scale.
#[test]
fn the_first_frame_counts_the_samples_the_steady_frames_must_not_reread() {
    let mut state = state_for(ResultViewer::Waves);
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let baseline = WorkCounts::reset();
    frame(&ctx, &mut state, &mut |ui, state| {
        show_surface(ResultViewer::Waves, ui, state);
    });
    let first = baseline.since();
    assert!(
        first.samples(FrameSampleRead::TraceExtremes) >= TRANSIENT_SAMPLES as u64,
        "the first Waves frame read {} samples for trace extremes, fewer than the \
         {TRANSIENT_SAMPLES} one retained waveform holds, so the sample gate is vacuous",
        first.samples(FrameSampleRead::TraceExtremes)
    );
}

#[test]
fn the_counter_reports_exactly_the_walks_that_were_noted() {
    let baseline = WorkCounts::reset();
    super::note(DatasetWalk::WaveXRange);
    super::note(DatasetWalk::WaveXRange);
    super::note(DatasetWalk::EyeRaster);

    let delta = baseline.since();
    assert_eq!(delta.get(DatasetWalk::WaveXRange), 2);
    assert_eq!(delta.get(DatasetWalk::EyeRaster), 1);
    assert_eq!(delta.get(DatasetWalk::OpPlan), 0);
    assert_eq!(delta.total(), 3);
    assert_eq!(delta.total_samples(), 0);
    assert_eq!(
        delta.nonzero(),
        vec![(DatasetWalk::WaveXRange, 2), (DatasetWalk::EyeRaster, 1)]
    );
}

#[test]
fn the_counter_reports_exactly_the_samples_that_were_read() {
    let baseline = WorkCounts::reset();
    super::note_samples(FrameSampleRead::StripOverview, 160);
    super::note_samples(FrameSampleRead::StripOverview, 1);
    super::note_samples(FrameSampleRead::TraceExtremes, 250_000);

    let delta = baseline.since();
    assert_eq!(delta.samples(FrameSampleRead::StripOverview), 161);
    assert_eq!(delta.samples(FrameSampleRead::TraceExtremes), 250_000);
    assert_eq!(delta.total_samples(), 250_161);
    assert_eq!(delta.total(), 0);
    assert_eq!(
        delta.nonzero_samples(),
        vec![
            (FrameSampleRead::StripOverview, 161),
            (FrameSampleRead::TraceExtremes, 250_000)
        ]
    );
}

#[test]
fn a_reset_snapshot_measures_only_work_that_follows_it() {
    super::note(DatasetWalk::OpPlan);
    super::note_samples(FrameSampleRead::StripOverview, 999);
    let baseline = WorkCounts::reset();
    assert_eq!(baseline.total(), 0);
    assert_eq!(baseline.total_samples(), 0);
    assert_eq!(baseline.since().total(), 0);
    assert_eq!(baseline.since().total_samples(), 0);
}
