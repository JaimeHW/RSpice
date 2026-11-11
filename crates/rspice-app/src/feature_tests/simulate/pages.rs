//! What a setup route states about the preflight report its plan was frozen
//! against: whether it is current, expired, blocked or absent, the rerun an
//! expired one offers, and the dispatch contract only an authorized current
//! report carries.

use crate::state::SimulationPage;
use crate::state::{
    PreflightIssue, PreflightRemediation, PreflightReport, PreparedPreflightContract,
};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::run_preflight::PreflightCurrency;
use crate::workbench::surfaces::simulate::pages::preflight_currency;
use crate::workbench::surfaces::simulate::show_on_stage;

fn current_report(
    state: &crate::workbench::AppState,
    blockers: Vec<PreflightIssue>,
    prepared: Option<PreparedPreflightContract>,
) -> PreflightReport {
    let (topology_root, topology_revision, topology_closure) =
        state.model.configured_topology_revision();
    let plan = state.model.active_plan_revision();
    PreflightReport {
        project_revision: state.model.workspace.content.project.revision().get(),
        topology_root,
        topology_revision,
        topology_closure,
        simulation_plan_id: plan.map(|(id, _)| id),
        simulation_plan_revision: plan.map(|(_, revision)| revision),
        blockers,
        advisories: Vec::new(),
        prepared,
    }
}

fn prepared_contract(task_count: usize) -> PreparedPreflightContract {
    PreparedPreflightContract {
        snapshot_digest: crate::product::ContentDigest::from_bytes([1; 32]),
        source_digest: crate::product::ContentDigest::from_bytes([2; 32]),
        receipt_digest: crate::product::ContentDigest::from_bytes([3; 32]),
        receipt_label: "receipt",
        analysis_ids: vec![crate::product::ContentDigest::from_bytes([4; 32])],
        task_count,
        saved_output_contract_count: 0,
        pvt_point_count: 1,
        target: "Desktop background thread",
        save_policy: "Retain engine-produced results",
        model_identity_count: 1,
    }
}

fn blocker() -> PreflightIssue {
    PreflightIssue {
        check: "Run set".to_owned(),
        observed: "No analysis is enabled".to_owned(),
        required: "One enabled analysis".to_owned(),
        remediation: PreflightRemediation::SimulationPlan { analysis: None },
    }
}

#[test]
fn every_setup_route_states_the_preflight_report_it_was_frozen_against() {
    let mut app = RSpiceApp::test_instance();
    assert_eq!(
        preflight_currency(&app.state.model, &app.state.runs),
        PreflightCurrency::Absent
    );
    assert!(!PreflightCurrency::Absent.wants_rerun());

    app.state.runs.preflight.report = Some(current_report(
        &app.state,
        Vec::new(),
        Some(prepared_contract(3)),
    ));
    assert_eq!(
        preflight_currency(&app.state.model, &app.state.runs),
        PreflightCurrency::Authorized { tasks: 3 }
    );
    assert!(!preflight_currency(&app.state.model, &app.state.runs).wants_rerun());

    app.state.runs.preflight.report = Some(current_report(
        &app.state,
        vec![blocker()],
        Some(prepared_contract(3)),
    ));
    assert_eq!(
        preflight_currency(&app.state.model, &app.state.runs),
        PreflightCurrency::Blocked { blockers: 1 }
    );

    // Anything the report was frozen against moving expires it, and the
    // routes that are not Analyses are exactly where that editing happens.
    let mut stale = current_report(&app.state, Vec::new(), Some(prepared_contract(3)));
    stale.project_revision += 1;
    app.state.runs.preflight.report = Some(stale);
    assert_eq!(
        preflight_currency(&app.state.model, &app.state.runs),
        PreflightCurrency::Expired
    );
    assert!(preflight_currency(&app.state.model, &app.state.runs).wants_rerun());
}

/// Render one route through the surface entry point and return its
/// AccessKit node labels.
///
/// Deliberately the surface as the shell draws it, not this module's `show`:
/// Analyses is routed past this module entirely and paints its heading in
/// `super`, so a helper that called `pages::show` could only ever ask
/// eight of the nine routes whether they state their preflight standing —
/// and the ninth is the one that went without it.
fn route_announcements(
    page: SimulationPage,
    seed: impl FnOnce(&mut crate::workbench::AppState),
) -> Vec<String> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = RSpiceApp::test_instance();
    app.state.simulate.simulation_page = page;
    seed(&mut app.state);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1200.0, 1600.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| show_on_stage(ui, &mut app));
        },
    );
    output
        .platform_output
        .accesskit_update
        .expect("AccessKit update")
        .nodes
        .iter()
        .filter_map(|(_, node)| node.label())
        .map(str::to_owned)
        .collect()
}

/// Every route, including Analyses.
///
/// The loop used to name three routes under a title that claimed all of
/// them, and the route it most needed to name was the one it could not
/// reach: Analyses paints its own heading and had no chip at all. Walking
/// `NAVIGATION` is what makes the title true and what stops the next route
/// from being added without one.
#[test]
fn an_expired_report_offers_its_rerun_from_the_route_the_reader_is_on() {
    for page in SimulationPage::NAVIGATION {
        let labels = route_announcements(page, |state| {
            let mut stale = current_report(state, Vec::new(), Some(prepared_contract(2)));
            stale.project_revision += 1;
            state.runs.preflight.report = Some(stale);
        });
        assert!(
            labels
                .iter()
                .any(|label| label.contains("Preflight expired")
                    && label.contains("Rerun preflight checks")),
            "{page:?} must state the expired report and offer its rerun: {labels:?}"
        );
    }
}

/// One route from each of the two headings that can paint the chip.
#[test]
fn a_route_states_nothing_when_no_preflight_has_been_run() {
    for page in [SimulationPage::Solver, SimulationPage::Analyses] {
        let labels = route_announcements(page, |_| {});
        assert!(
            !labels.iter().any(|label| label.contains("Preflight")),
            "an unrun preflight is the ordinary state and is not an announcement on \
             {page:?}: {labels:?}"
        );
    }
}

/// Everything the Analyses route states, by name and by value: a label
/// widget's text is its AccessKit value, not its name.
fn analyses_text(seed: impl FnOnce(&mut crate::workbench::AppState)) -> Vec<String> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = RSpiceApp::test_instance();
    app.state.simulate.simulation_page = SimulationPage::Analyses;
    seed(&mut app.state);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1200.0, 1600.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| show_on_stage(ui, &mut app));
        },
    );
    output
        .platform_output
        .accesskit_update
        .expect("AccessKit update")
        .nodes
        .iter()
        .flat_map(|(_, node)| [node.label(), node.value()])
        .flatten()
        .map(str::to_owned)
        .collect()
}

/// The dispatch contract states what an authorized, current report
/// authorizes — under the plan it authorizes — and nothing for a report
/// that authorizes nothing: one that is blocked, one that has expired,
/// or none at all.
#[test]
fn the_dispatch_contract_is_stated_only_for_an_authorized_current_report() {
    const TARGET: &str = "Desktop background thread";

    let authorized = analyses_text(|state| {
        state.runs.preflight.report = Some(current_report(
            state,
            Vec::new(),
            Some(prepared_contract(3)),
        ));
    });
    // The card's rows, by name and value. Its title is painted, not a
    // widget, so it is not among what the route announces.
    for expected in ["Analysis identities", "1 stable instance", TARGET] {
        assert!(
            authorized.iter().any(|text| text.contains(expected)),
            "an authorized report states {expected:?}: {authorized:?}"
        );
    }

    let blocked = analyses_text(|state| {
        state.runs.preflight.report = Some(current_report(
            state,
            vec![blocker()],
            Some(prepared_contract(3)),
        ));
    });
    let expired = analyses_text(|state| {
        let mut stale = current_report(state, Vec::new(), Some(prepared_contract(3)));
        stale.project_revision += 1;
        state.runs.preflight.report = Some(stale);
    });
    let absent = analyses_text(|_| {});
    for (standing, text) in [
        ("blocked", blocked),
        ("expired", expired),
        ("absent", absent),
    ] {
        assert!(
            !text
                .iter()
                .any(|text| text.contains("Analysis identities") || text.contains(TARGET)),
            "a {standing} report authorizes no dispatch and states none: {text:?}"
        );
    }
}

/// A report that authorizes dispatch is a report with nothing to say.
///
/// It painted a green "PREFLIGHT CURRENT · 2 tasks ready" chip on every
/// setup route, which is success reported as evidence. Success is the
/// absence of a problem, so the chip is gone and only the two standings
/// that owe the reader a rerun paint one.
#[test]
fn an_authorized_report_paints_no_chip() {
    for page in [SimulationPage::Solver, SimulationPage::Analyses] {
        let labels = route_announcements(page, |state| {
            state.runs.preflight.report = Some(current_report(
                state,
                Vec::new(),
                Some(prepared_contract(2)),
            ));
        });
        assert!(
            !labels.iter().any(|label| label.contains("Preflight")),
            "an authorized report states nothing on {page:?}: {labels:?}"
        );
    }
}
