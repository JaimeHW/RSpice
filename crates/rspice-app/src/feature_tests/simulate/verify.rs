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

/// One name per view: the inspector's title and the view's toolbar title are
/// both the view's label, the name its menu row, navigator row and palette
/// entry carry too.
#[test]
fn the_inspector_and_the_toolbar_name_each_view_by_its_label() {
    use rspice_app_simulate::workbench::surfaces::verify::raster::{
        VerifyStage, page_input, page_pass,
    };
    use rspice_app_workbench::workbench::docks::inspector::header;

    for page in VerificationPage::NAVIGATION {
        let mut app = RSpiceApp::test_instance();
        app.open(page);
        let ctx = egui::Context::default();
        rspice_app_workbench::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut headings = Vec::new();
        for _ in 0..2 {
            let (output, _) = page_pass(&ctx, page_input(Vec::new()), |ui| {
                header(ui, &mut app);
                app.draw(ui);
            });
            headings = output
                .platform_output
                .accesskit_update
                .expect("AccessKit tree update")
                .nodes
                .iter()
                .filter(|(_, node)| node.role() == egui::accesskit::Role::Heading)
                // The toolbar's heading carries its title as its value.
                .map(|(_, node)| {
                    (
                        node.level(),
                        node.label().or(node.value()).map(str::to_owned),
                    )
                })
                .collect::<Vec<_>>();
        }
        let label = Some(page.label().to_owned());
        assert!(
            headings.contains(&(Some(2), label.clone())),
            "{page:?}: the inspector's title among {headings:?}"
        );
        assert!(
            headings.contains(&(None, label)),
            "{page:?}: the toolbar's title among {headings:?}"
        );
    }
}

/// A stored session reopens on the Verify view it was left on, stored and
/// read back the way the application does it.
#[test]
fn a_stored_session_reopens_on_its_verify_view() {
    #[derive(Default)]
    struct MemoryStorage(std::collections::HashMap<String, String>);

    impl eframe::Storage for MemoryStorage {
        fn get_string(&self, key: &str) -> Option<String> {
            self.0.get(key).cloned()
        }

        fn set_string(&mut self, key: &str, value: String) {
            self.0.insert(key.to_owned(), value);
        }

        fn remove_string(&mut self, key: &str) {
            self.0.remove(key);
        }

        fn flush(&mut self) {}
    }

    for page in VerificationPage::NAVIGATION {
        let mut app = RSpiceApp::test_instance();
        app.state.simulate.verification_page = page;
        let mut storage = MemoryStorage::default();
        app.state.store_session(&mut storage);
        let restored = crate::workbench::app_state::AppState::restore_eframe_session(&storage)
            .expect("RSpice restores a session it just stored");
        assert_eq!(restored.simulate.verification_page, page);
    }
}

/// The shell's filter field narrows the open view's rows, so it names them:
/// specifications on the views listed by specification, checks on the rest.
#[test]
fn the_filter_field_names_the_rows_of_the_open_view() {
    use rspice_app_state::workbench::workspace::Workspace;
    use rspice_app_workbench::workbench::framework::host::Host;

    let mut app = RSpiceApp::test_instance();
    for (page, placeholder) in [
        (
            VerificationPage::Specifications,
            "Filter specifications\u{2026}",
        ),
        (VerificationPage::Corners, "Filter specifications\u{2026}"),
        (
            VerificationPage::MonteCarlo,
            "Filter specifications\u{2026}",
        ),
        (VerificationPage::Checks, "Filter checks\u{2026}"),
        (VerificationPage::Reference, "Filter checks\u{2026}"),
    ] {
        app.state.simulate.verification_page = page;
        assert_eq!(
            app.navigator_section(Workspace::Verify).placeholder,
            placeholder,
            "{page:?}"
        );
    }
    assert_eq!(
        app.navigator_section(Workspace::Simulate).placeholder,
        "Filter setup\u{2026}"
    );
}

/// Every seeded page opens with a 36 px toolbar whose title starts on the
/// panel inset, then a table of 24 px head and rows directly under it, with
/// every right-aligned column ending on one edge and nothing past the insets.
#[test]
fn seeded_verify_pages_follow_the_table_grammar() {
    use super::verify_fixtures::{corner_app, monte_carlo_app, soa_app};
    use rspice_app_simulate::workbench::surfaces::verify::raster::{PAGE_WIDTH, page_geometry};

    for (page, app) in [
        (VerificationPage::Specifications, corner_app()),
        (VerificationPage::MonteCarlo, monte_carlo_app()),
        (VerificationPage::Corners, corner_app()),
        (VerificationPage::Checks, soa_app()),
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
        // Band rows naming runs of columns are the head's own top rows.
        let bands = geometry.all("table.band");
        assert_eq!(
            bands.first().map_or(head.top(), |band| band.top()),
            toolbar.bottom(),
            "{page:?}: nothing between toolbar and table"
        );
        if let Some(band) = bands.last() {
            assert_eq!(head.top(), band.bottom(), "{page:?}");
        }
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
