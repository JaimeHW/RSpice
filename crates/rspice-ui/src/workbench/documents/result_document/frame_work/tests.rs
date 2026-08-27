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

use super::{DatasetWalk, WorkCounts};
use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisType, DcOpResult,
    OperatingPointValue, SimulationRun, SoaEvaluationEvidence, SoaParameterEvidence,
    SoaRuleVerdictEvidence, SoaViolationEvidence, SoaViolationSeverityEvidence, WaveformData,
};
use crate::workbench::AppState;
use crate::workbench::documents::result_document::{
    ResultViewer, eye, manifest, op_inspector, soa, table, waves,
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

/// One run carrying every dataset the measured surfaces read.
fn large_state() -> AppState {
    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.add_analysis(transient_analysis());
    run.add_analysis(operating_point_analysis());
    run.add_analysis(soa_analysis());
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
    ]
}

fn show_surface(viewer: ResultViewer, ui: &mut egui::Ui, state: &mut AppState) {
    state.ui.results.viewer = viewer;
    // The document bar states the sheet's purpose on every frame, including
    // whether the retained evidence validated. It is part of the frame.
    let _ = super::super::sheet_purpose(state);
    match viewer {
        ResultViewer::Manifest => manifest::show(ui, state),
        ResultViewer::Op => op_inspector::show(ui, state),
        ResultViewer::Table => table::show(ui, state),
        ResultViewer::Soa => soa::show(ui, state),
        ResultViewer::Waves => waves::show(ui, state),
        ResultViewer::Eye => eye::show(ui, state),
        other => panic!("{other:?} is not a measured surface"),
    }
}

fn state_for(viewer: ResultViewer) -> AppState {
    let mut state = large_state();
    match viewer {
        ResultViewer::Op => select_analysis(&mut state, AnalysisType::DcOp),
        ResultViewer::Soa => select_analysis(&mut state, AnalysisType::Soa),
        _ => select_analysis(&mut state, AnalysisType::Transient),
    }
    state
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

/// Report the counts rather than asserting on them, for tuning the fixes.
/// Not a gate: `--ignored` keeps it out of the ordinary run.
#[test]
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
    assert_eq!(
        delta.nonzero(),
        vec![(DatasetWalk::WaveXRange, 2), (DatasetWalk::EyeRaster, 1)]
    );
}

#[test]
fn a_reset_snapshot_measures_only_work_that_follows_it() {
    super::note(DatasetWalk::OpPlan);
    let baseline = WorkCounts::reset();
    assert_eq!(baseline.total(), 0);
    assert_eq!(baseline.since().total(), 0);
}
