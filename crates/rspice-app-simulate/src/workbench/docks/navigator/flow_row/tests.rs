//! What the navigator is allowed to claim about a run.
//!
//! Each view's second line is counted from what that view draws; the runs
//! list puts the newest first and marks the reference. The remaining cases
//! hold the mock's grid and spacing, and the density switch between desktop
//! and touch.

use super::super::simulate::simulate_nav_meta;
use super::super::verify::{
    VerifyNavigator, verification_flow_presentation, verification_navigator_requires_scroll, verify,
};
use super::{FLOW_DETAIL_TOP, FLOW_LABEL_TOP, FLOW_ROW_HEIGHT, FLOW_STATUS_TOP, flow_row_geometry};
use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
use crate::state::pdk_config::PdkConfig;
use crate::state::{
    AnalysisResult, AnalysisResultPayload, AnalysisType, SimulationRun, SimulationState,
};
use crate::state::{SimulationPage, VerificationPage};
use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::harness::FeatureHarness;
use crate::workbench::simulate::SimulateFeature;
use crate::workbench::simulate::test_host::SimulateTestHost;
use crate::workbench::slots::SimulateState;
use crate::workbench::surfaces::verify::raster::sealed_run;
use crate::workbench::surfaces::verify::set_reference_run;
use rspice_app_state::workbench::workspace::restore_model_library_manager;
use rspice_results::yield_analysis::{
    DistributionStats, MonteCarloSamplingMode, YieldAnalysisProvenance, YieldResult, YieldSpec,
};

fn result(trail: Vec<bool>) -> YieldResult {
    let pass_count = trail.iter().filter(|passes| **passes).count();
    YieldResult {
        spec: YieldSpec::lower("gain", 0.0, ""),
        total_runs: trail.len(),
        pass_count,
        fail_count: trail.len() - pass_count,
        yield_percent: pass_count as f64 / trail.len() as f64 * 100.0,
        stats: DistributionStats::default(),
        samples: vec![1.0; trail.len()],
        trail,
    }
}

#[test]
fn simulation_models_meta_counts_only_active_plan_bindings() {
    let mut model = Model::session_start(PdkConfig::default(), None, restore_model_library_manager);

    assert_eq!(
        simulate_nav_meta(&model, SimulationPage::Models, "unused", None, 0),
        None,
        "globally available libraries are not owned by the active simulation plan",
    );

    model
        .sim_setup
        .model_bindings
        .push(crate::state::model_library::SimulationPlanModelBinding {
            library_name: "project-process-models".to_owned(),
            source_digest: ContentDigest::from_bytes([0x6d; 32]),
            selected_corner: Some("tt".to_owned()),
        });

    assert_eq!(
        simulate_nav_meta(&model, SimulationPage::Models, "unused", None, 0),
        Some("1".to_owned()),
    );
}

#[test]
fn verification_flow_rows_match_the_mock_grid_without_text_collisions() {
    assert!(FLOW_DETAIL_TOP - (FLOW_LABEL_TOP + 11.0) >= 4.0);
    assert!(FLOW_STATUS_TOP - (FLOW_DETAIL_TOP + 11.0) >= 4.0);
    assert!(FLOW_ROW_HEIGHT - (FLOW_STATUS_TOP + 11.0) >= 7.0);
    // A label and one line of detail; each wrapped line adds one pitch.
    assert_eq!(flow_row_geometry(1), 43.0);
    assert_eq!(flow_row_geometry(2), 58.0);
    assert_eq!(flow_row_geometry(3), 73.0);
    // A row with nothing under its name keeps its siblings' height.
    assert_eq!(flow_row_geometry(0), 43.0);
}

#[test]
fn verification_navigation_lists_every_view_by_its_label() {
    let model = Model::session_start(PdkConfig::default(), None, restore_model_library_manager);
    let simulate = SimulateState::default();
    let labels = VerificationPage::NAVIGATION
        .map(|page| verification_flow_presentation(&model, &simulate, page).label);
    assert_eq!(
        labels,
        [
            "Specifications",
            "Corners",
            "Monte Carlo",
            "Checks",
            "Reference"
        ]
    );
    assert_eq!(
        labels,
        VerificationPage::NAVIGATION.map(|page| page.label().to_owned())
    );
}

/// The Checks row counts the run's checks as the Checks view lists them, and
/// leads with the worst one's mark.
#[test]
fn the_checks_flow_counts_the_checks_the_run_holds() {
    let mut model = Model::session_start(PdkConfig::default(), None, restore_model_library_manager);
    let evaluation = crate::state::SoaEvaluationEvidence {
        duration: None,
        thresholds: Default::default(),
        envelope: None,
        derating: None,
        device_id: "M1".to_owned(),
        parameter: crate::state::SoaParameterEvidence::DrainSourceVoltage,
        limit_value: 3.3,
        worst_actual_value: 3.0,
        worst_time_s: 1.0e-9,
        sample_count: 2,
        unit: "V".to_owned(),
        description: "Maximum drain-source voltage".to_owned(),
        verdict: crate::state::SoaRuleVerdictEvidence::Warning,
    };
    let analysis = AnalysisResult::new(1, AnalysisType::Soa, "SOA")
        .with_result_payload(AnalysisResultPayload::Soa {
            source_history: None,
            evaluations: vec![evaluation],
            violations: Vec::new(),
        })
        .with_provenance(
            crate::state::AnalysisResultProvenance::new(
                AnalysisInstanceId::new(),
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([0x5c; 32]),
                Vec::new(),
            )
            .expect("test provenance is valid"),
        );
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    model.simulation.retained.runs = vec![run].into();
    model.simulation.view.active_run_idx = Some(0);

    model
        .simulation
        .refresh_verification(&model.sim_setup, &model.workspace.content);
    assert_eq!(model.simulation.verification_checks().rows.len(), 1);

    let flow =
        verification_flow_presentation(&model, &SimulateState::default(), VerificationPage::Checks);
    assert_eq!(flow.detail, "1 warning \u{b7} 1 check");
    assert_eq!(flow.problem, Some(crate::ui::widgets::Problem::Warning));
}

/// The Checks line counts a failed check and one that could not be evaluated
/// as violations and a warning apart from both, leaves out a part with
/// nothing to count, and says nothing for a run that evaluated no check.
#[test]
fn the_checks_line_counts_violations_and_warnings_apart() {
    use crate::ui::widgets::Problem;
    use crate::workbench::docks::navigator::verify::checks_line;
    use rspice_results::verification::{CheckStatus, Checks};

    let mut model = Model::session_start(PdkConfig::default(), None, restore_model_library_manager);
    crate::workbench::surfaces::verify::raster::seed_soa(&mut model);
    model
        .simulation
        .refresh_verification(&model.sim_setup, &model.workspace.content);
    let row = model
        .simulation
        .verification_checks()
        .rows
        .first()
        .cloned()
        .expect("the SOA fixture evaluates checks");
    let checks = |statuses: &[CheckStatus]| Checks {
        points: Vec::new(),
        rows: statuses
            .iter()
            .map(|status| rspice_results::verification::CheckRow {
                status: *status,
                ..row.clone()
            })
            .collect(),
    };
    use CheckStatus::{Error, Fail, Info, Pass, Warning};
    assert_eq!(
        checks_line(&checks(&[
            Fail, Fail, Fail, Error, Warning, Warning, Pass, Pass, Info
        ])),
        (
            "4 violations \u{b7} 2 warnings \u{b7} 9 checks".to_owned(),
            Some(Problem::Failure)
        )
    );
    assert_eq!(
        checks_line(&checks(&[Error, Pass])),
        (
            "1 violation \u{b7} 2 checks".to_owned(),
            Some(Problem::Failure)
        )
    );
    assert_eq!(
        checks_line(&checks(&[Warning, Warning])),
        (
            "2 warnings \u{b7} 2 checks".to_owned(),
            Some(Problem::Warning)
        )
    );
    assert_eq!(
        checks_line(&checks(&[Pass, Info, Pass])),
        ("3 checks".to_owned(), None)
    );
    assert_eq!(checks_line(&Checks::default()), (String::new(), None));
}

/// On the seeded SOA run the navigator's Checks line states exactly the
/// counts of the checks the view lists.
#[test]
fn the_checks_line_states_the_soa_fixture_as_the_view_lists_it() {
    use rspice_results::verification::CheckStatus;

    let mut model = Model::session_start(PdkConfig::default(), None, restore_model_library_manager);
    crate::workbench::surfaces::verify::raster::seed_soa(&mut model);
    model
        .simulation
        .refresh_verification(&model.sim_setup, &model.workspace.content);
    let rows = &model.simulation.verification_checks().rows;
    let count = |statuses: &[CheckStatus]| {
        rows.iter()
            .filter(|row| statuses.contains(&row.status))
            .count()
    };
    let violations = count(&[CheckStatus::Fail, CheckStatus::Error]);
    let warnings = count(&[CheckStatus::Warning]);
    assert!(violations > 0, "the fixture holds a violation");
    let mut expected = vec![format!(
        "{violations} violation{}",
        if violations == 1 { "" } else { "s" }
    )];
    if warnings > 0 {
        expected.push(format!(
            "{warnings} warning{}",
            if warnings == 1 { "" } else { "s" }
        ));
    }
    expected.push(format!("{} checks", rows.len()));
    let flow =
        verification_flow_presentation(&model, &SimulateState::default(), VerificationPage::Checks);
    assert_eq!(flow.detail, expected.join(" \u{b7} "));
}

#[test]
fn verification_navigator_scrolls_when_flows_exceed_compact_height() {
    assert!(verification_navigator_requires_scroll(440.0));
    assert!(!verification_navigator_requires_scroll(441.0));
    assert!(verification_navigator_requires_scroll(330.0));
    assert!(!verification_navigator_requires_scroll(560.0));
    assert!(!verification_navigator_requires_scroll(700.0));
}

/// The longest Monte Carlo sample trail the active dataset's yield evidence
/// holds, or none when the evidence is another dataset's.
fn active_mc_sample_trail(simulation: &SimulationState) -> usize {
    simulation
        .yield_results_for_active_dataset()
        .unwrap_or(&[])
        .iter()
        .map(|result| result.trail.len())
        .max()
        .unwrap_or(0)
}

#[test]
fn mc_sample_trail_is_visible_only_for_its_active_provenance_dataset() {
    let source = SimulationRun::new(1);
    let source_run_id = source.run_id;
    let source_dataset_id = source.dataset_id;
    let other = SimulationRun::new(2);
    let mut simulation = SimulationState {
        retained: crate::state::RetainedSimulationState {
            runs: vec![source, other].into(),
            ..Default::default()
        },
        view: crate::state::SimulationViewState {
            active_run_idx: Some(0),
            ..Default::default()
        },
        ..SimulationState::default()
    };
    simulation.replace_yield_evidence(
        vec![result(vec![true, false, true])],
        Some(YieldAnalysisProvenance {
            source_run_id,
            source_dataset_id,
            seed: 7,
            runs_requested: 3,
            runs_completed: 3,
            sampling_mode: MonteCarloSamplingMode::PseudoRandom,
        }),
    );

    assert_eq!(active_mc_sample_trail(&simulation), 3);
    simulation.view.active_run_idx = Some(1);
    assert_eq!(active_mc_sample_trail(&simulation), 0);
    simulation.view.active_run_idx = None;
    assert_eq!(active_mc_sample_trail(&simulation), 0);
}

/// One pass of the Verify navigator over `harness`, with `events` arriving,
/// and what the reader asked of it.
fn navigator_pass(
    ctx: &egui::Context,
    harness: &mut FeatureHarness<SimulateFeature>,
    events: Vec<egui::Event>,
) -> (egui::FullOutput, super::super::verify::VerifyNavigatorAsk) {
    let model = &mut harness.model;
    model
        .simulation
        .refresh_verification(&model.sim_setup, &model.workspace.content);
    let mut ask = None;
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(320.0, 900.0),
            )),
            events,
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ask = Some(verify(
                    ui,
                    &VerifyNavigator {
                        model: &harness.model,
                        simulate: &harness.state,
                    },
                ));
            });
        },
    );
    (output, ask.expect("the navigator drew"))
}

/// The run rows the navigator announced, top to bottom, as (what it says,
/// where it is).
fn run_rows(output: egui::FullOutput) -> Vec<(String, egui::Rect)> {
    let mut rows = output
        .platform_output
        .accesskit_update
        .expect("AccessKit tree update")
        .nodes
        .into_iter()
        .filter_map(|(_, node)| {
            let label = node.label()?.to_string();
            let bounds = node.bounds()?;
            label.starts_with("Run ").then(|| {
                (
                    label,
                    egui::Rect::from_min_max(
                        egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                        egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                    ),
                )
            })
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.1.top().total_cmp(&right.1.top()));
    rows
}

/// The runs list holds the plan's runs newest first, each row two lines that
/// state how its run ended whether or not Verify judges it, the reference
/// marked on that line, and picking one asks for it to be the run Verify
/// judges.
#[test]
fn the_runs_list_is_newest_first_marks_the_reference_and_picks_a_run() {
    let mut harness = SimulateTestHost::harness();
    let plan_id = harness
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("default plan")
        .id();
    let source = AnalysisInstanceId::new();
    let measured = |label: &str| {
        AnalysisResult::new(1, AnalysisType::Ac, label)
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 1.0)])
    };
    let completed = |mut run: SimulationRun| {
        run.restore_lifecycle(crate::state::SimulationRunLifecycle::Completed, 1.0)
            .expect("a finite elapsed time");
        run
    };
    let older = completed(sealed_run(1, plan_id, source, 2, measured("older")));
    let newer = completed(sealed_run(2, plan_id, source, 2, measured("newer")));
    let older_id = older.run_id;
    harness.model.simulation.retained.runs = vec![older, newer].into();
    assert!(harness.model.simulation.select_run(1));
    set_reference_run(&mut harness.cx(), older_id).expect("the older run can be the reference");

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let (output, ask) = navigator_pass(&ctx, &mut harness, Vec::new());
    assert_eq!(ask, Default::default());
    let rows = run_rows(output);
    assert_eq!(rows.len(), 2, "{rows:?}");
    let label = |run: &SimulationRun| run.label.clone();
    let [newer, older] = [1, 0].map(|index| label(&harness.model.simulation.retained.runs[index]));
    // Both runs ended alike, so the selected one reads as the other does,
    // and the reference mark joins the same line.
    assert_eq!(rows[0].0, format!("{newer}. completed"), "{rows:?}");
    assert_eq!(
        rows[1].0,
        format!("{older}. completed \u{b7} reference"),
        "{rows:?}"
    );
    assert_eq!(
        rows[0].1.height(),
        flow_row_geometry(1),
        "a run row is its name and one line"
    );
    assert_eq!(rows[1].1.height(), rows[0].1.height(), "{rows:?}");

    // Selecting the other run changes neither row's words.
    assert!(harness.model.simulation.select_run(0));
    let (output, _) = navigator_pass(&ctx, &mut harness, Vec::new());
    assert_eq!(
        run_rows(output)
            .into_iter()
            .map(|(said, _)| said)
            .collect::<Vec<_>>(),
        rows.iter()
            .map(|(said, _)| said.clone())
            .collect::<Vec<_>>()
    );
    assert!(harness.model.simulation.select_run(1));

    let at = rows[1].1.center();
    let (_, ask) = navigator_pass(
        &ctx,
        &mut harness,
        vec![
            egui::Event::PointerMoved(at),
            egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::default(),
            },
            egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::default(),
            },
        ],
    );
    assert_eq!(ask.run, Some(0), "the older run is the first retained");
}
