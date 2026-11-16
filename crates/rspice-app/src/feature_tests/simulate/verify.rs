//! The Verify surface over the whole application: the device-safety pane and
//! rows, which read the Results workspace's memoized evidence verdict, and the
//! seeded pages' table grammar.

use super::verify_fixtures::soa_app;
use crate::workbench::RSpiceApp;
use rspice_app_simulate::workbench::surfaces::verify::latest_validated_analysis;
use rspice_app_simulate::workbench::surfaces::verify::raster::sealed_run;
use rspice_app_simulate::workbench::surfaces::verify::safety::device_safety;
use rspice_app_simulate::workbench::surfaces::verify::safety::soa_rows;
use rspice_app_simulate::workbench::surfaces::verify::safety::soa_summary;
use rspice_app_simulate::workbench::surfaces::verify::table::Verdict;
use rspice_app_simulate::workbench::surfaces::verify::toolbar::Summary;
use rspice_app_simulate::workbench::surfaces::verify::toolbar::TOOLBAR_H;
use rspice_app_state::product::AnalysisInstanceId;
use rspice_app_state::state::VerificationPage;
use rspice_app_state::state::{AnalysisResult, AnalysisType, RetainedTrace};

/// Retained samples in the fixture SOA stress histories. Large enough that a
/// per-frame revalidation is real work, small enough to build in a test.
const EVIDENCE_SAMPLES: usize = 512;

/// Evaluated rules, and so retained stress waveforms, in the fixture.
const EVIDENCE_RULES: usize = 8;

/// Frames measured after the first, matching the Results idle-frame gate: ten
/// is enough that a per-frame walk cannot hide behind an alternating schedule.
const EVIDENCE_IDLE_FRAMES: usize = 10;

/// An SOA analysis whose retained evidence validates, built in the shape the
/// validator accepts: one exact worst-point event per warned rule.
fn soa_evidence_analysis() -> AnalysisResult {
    use rspice_app_state::state::{
        SoaEvaluationEvidence, SoaParameterEvidence, SoaRuleVerdictEvidence, SoaViolationEvidence,
        SoaViolationSeverityEvidence,
    };

    let time: Vec<f64> = (0..EVIDENCE_SAMPLES)
        .map(|index| index as f64 * 1.0e-9)
        .collect();
    let mut waveforms = Vec::with_capacity(EVIDENCE_RULES);
    let mut evaluations = Vec::with_capacity(EVIDENCE_RULES);
    let mut violations = Vec::with_capacity(EVIDENCE_RULES);
    for rule in 0..EVIDENCE_RULES {
        let device_id = format!("M{rule:03}");
        let y: Vec<f64> = (0..EVIDENCE_SAMPLES)
            .map(|index| 3.0 * (index as f64 + 1.0) / EVIDENCE_SAMPLES as f64)
            .collect();
        let worst_actual_value = y[EVIDENCE_SAMPLES - 1];
        let worst_time_s = time[EVIDENCE_SAMPLES - 1];
        waveforms.push(RetainedTrace::new(
            rspice_app_state::results::safety::soa_stress_waveform_name(
                &device_id,
                rspice_app_state::results::safety::SoAParameter::Vds,
            ),
            time.clone(),
            y,
            "#00aaff",
        ));
        evaluations.push(SoaEvaluationEvidence {
            duration: None,
            thresholds: Default::default(),
            envelope: None,
            derating: None,
            device_id: device_id.clone(),
            parameter: SoaParameterEvidence::DrainSourceVoltage,
            limit_value: 3.3,
            worst_actual_value,
            worst_time_s,
            sample_count: EVIDENCE_SAMPLES as u64,
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
    AnalysisResult::new(1, AnalysisType::Soa, "SOA")
        .with_family_metadata(rspice_app_state::state::AnalysisResultFamilyMetadata::Soa { time })
        .with_waveforms(waveforms)
        .with_result_payload(rspice_app_state::state::AnalysisResultPayload::Soa {
            source_history: None,
            evaluations,
            violations,
        })
}

fn app_retaining_soa_evidence() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let plan_id = app
        .state
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("default plan")
        .id();
    let run = sealed_run(
        1,
        plan_id,
        AnalysisInstanceId::new(),
        9,
        soa_evidence_analysis(),
    );
    app.state.model.simulation.retained.runs = vec![run].into();
    assert!(app.state.model.simulation.select_run(0));
    app
}

#[test]
fn the_soa_pane_validate_retained_evidence_once_per_dataset_generation() {
    use rspice_app_results::workbench::documents::result_document::frame_work::{
        DatasetWalk, WorkCounts,
    };

    let mut app = app_retaining_soa_evidence();
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    // One frame of the SOA tab against unchanged state and identical
    // input, drawn through the pane's own entry point.
    let mut frame = || {
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1_680.0, 1_020.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    device_safety(ui, &mut crate::workbench::features::cx(&mut app))
                });
            },
        );
    };

    // The first frame is allowed to build the memo the rest read — but it must
    // build it through the counted owner. A pane that validates on its own is
    // uncounted, so this half of the gate is what keeps the other half from
    // passing by measuring nothing.
    let opening = WorkCounts::reset();
    frame();
    assert_eq!(
        opening.since().get(DatasetWalk::EvidenceValidation),
        1,
        "the SOA gate must ask the memoized owner once on the \
         opening frame; a gate that walks the dataset itself is not counted here \
         and would leave the idle-frame assertion below measuring nothing"
    );

    let baseline = WorkCounts::reset();
    for _ in 0..EVIDENCE_IDLE_FRAMES {
        frame();
    }
    let work = baseline.since();

    assert_eq!(
        work.get(DatasetWalk::EvidenceValidation),
        0,
        "the SOA pane revalidated the retained dataset on an idle frame; counted work was {:?}",
        work.nonzero()
    );
}

#[test]
fn the_soa_evidence_gate_rejects_source_edits_before_a_version_bump() {
    let mut app = app_retaining_soa_evidence();
    let version = app.state.model.simulation.view.data_version;
    assert!(
        latest_validated_analysis(&app.state.model, &*app.state.results, AnalysisType::Soa)
            .is_some(),
        "the fixture retains validated SOA evidence"
    );

    // A retained waveform with more coordinates than values is exactly what
    // `validate_retained_evidence` exists to refuse.
    let analysis = app.state.model.simulation.retained.runs[0]
        .analyses
        .iter_mut()
        .find(|analysis| analysis.analysis_type == AnalysisType::Soa)
        .expect("the fixture retains an SOA analysis");
    let mut shortened = analysis.waveforms[0].y.as_ref().clone();
    shortened.pop();
    analysis.waveforms[0].y = std::sync::Arc::new(shortened);

    assert!(
        latest_validated_analysis(&app.state.model, &*app.state.results, AnalysisType::Soa)
            .is_none(),
        "the gate kept a valid verdict after its retained source was corrupted"
    );
    assert_eq!(app.state.model.simulation.view.data_version, version);

    app.state.model.simulation.view.data_version =
        app.state.model.simulation.view.data_version.wrapping_add(1);

    assert!(
        latest_validated_analysis(&app.state.model, &*app.state.results, AnalysisType::Soa)
            .is_none(),
        "the gate served a verdict from the previous dataset generation"
    );
}

#[test]
fn soa_rows_put_violations_first_and_the_summary_counts_devices() {
    let app = soa_app();
    let analysis =
        latest_validated_analysis(&app.state.model, &*app.state.results, AnalysisType::Soa)
            .expect("the SOA fixture's evidence validates");
    let Some(rspice_app_state::state::AnalysisResultPayload::Soa { evaluations, .. }) =
        &analysis.result_payload
    else {
        panic!("the fixture holds an SOA payload");
    };
    let rows = soa_rows(evaluations);
    assert_eq!(
        rows.iter()
            .map(|row| (row.device.as_str(), row.verdict))
            .collect::<Vec<_>>(),
        [
            ("M3", Verdict::Fail),
            ("M1", Verdict::Fail),
            ("M2", Verdict::Near),
            ("M1", Verdict::Pass),
            ("M4", Verdict::Pass),
        ]
    );
    assert_eq!(
        soa_summary(&rows),
        Some(Summary::failing("2 violations on 2 devices"))
    );
    assert_eq!(soa_summary(&rows[3..]), None, "a clean run says nothing");
}

/// Every seeded page opens with a 36 px toolbar whose title starts on the
/// panel inset, then a table of 24 px head and rows directly under it, with
/// every right-aligned column ending on one edge and nothing past the insets.
#[test]
fn seeded_verify_pages_follow_the_table_grammar() {
    use super::verify_fixtures::{corner_app, monte_carlo_app, optimization_app, soa_app};
    use rspice_app_simulate::workbench::surfaces::verify::raster::{PAGE_WIDTH, page_geometry};

    for (page, app) in [
        (VerificationPage::Yield, monte_carlo_app()),
        (VerificationPage::Corners, corner_app()),
        (VerificationPage::DeviceSafety, soa_app()),
        (VerificationPage::Optimization, optimization_app()),
    ] {
        let geometry = page_geometry(app, page);
        let toolbar = geometry.one("toolbar").expect("toolbar");
        assert_eq!(toolbar.height(), TOOLBAR_H, "{page:?}");
        assert_eq!(toolbar.left(), 0.0, "{page:?}");
        assert_eq!(
            geometry.one("toolbar.title").expect("title").left(),
            10.0,
            "{page:?}"
        );
        let head = geometry.one("table.head").expect("table head");
        assert_eq!(head.height(), 24.0, "{page:?}");
        assert_eq!(
            head.top(),
            toolbar.bottom(),
            "{page:?}: nothing between toolbar and table"
        );
        let rows = geometry.all("table.row");
        assert!(!rows.is_empty(), "{page:?}");
        assert!(rows.iter().all(|row| row.height() == 24.0), "{page:?}");
        assert_eq!(rows[0].top(), head.bottom(), "{page:?}");
        for (column, edges) in geometry.right_edges() {
            assert_eq!(
                edges.len(),
                1,
                "{page:?} {column}: numbers end on one edge, {edges:?}"
            );
        }
        assert_eq!(
            geometry.outside_insets(PAGE_WIDTH),
            Vec::<String>::new(),
            "{page:?}"
        );
        assert!(
            geometry.one("toolbar.summary").is_some(),
            "{page:?} states a summary"
        );
    }
}
