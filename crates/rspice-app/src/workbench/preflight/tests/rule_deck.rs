//! The project's rule deck is one configuration: the Problems page, preflight
//! and the run gate judge the configured root under it alike.

use super::*;
use crate::state::Point;
use crate::workbench::app_state::diagnostic_store::{DiagnosticInputs, DiagnosticLevel};
use rspice_design::drc::{DrcSeverity, DrcViolationType};
use rspice_simulation::execution::PreparationStage;

/// The voltage divider, runnable as it stands, with the project technology
/// the run gate asks for.
fn runnable_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    disable_global_process_axis(&mut app.state);
    crate::workbench::examples::load_example("Voltage Divider", &mut app.state.schematic);
    app.state.provision_test_project_technology_contract();
    app.state.sync_active_schematic_to_workspace();
    app
}

/// Run the Check command and return the levels the page lists for `rule`.
fn page_levels(state: &mut AppState, rule: DrcViolationType) -> Vec<DiagnosticLevel> {
    let result = state
        .run_active_design_checks()
        .expect("the configured design checks");
    let subject = state.workspace.content.active_schematic_reference();
    let records = state
        .diagnostics
        .batch(&DiagnosticInputs::DesignCheck { subject })
        .expect("the checks published their findings")
        .records
        .clone();
    result
        .violations()
        .iter()
        .zip(records)
        .filter(|(violation, _)| violation.violation_type == rule)
        .map(|(_, record)| record.level)
        .collect()
}

fn preflight_blocks_on_design_rules(app: &RSpiceApp) -> bool {
    collect_report(&app.state)
        .blockers
        .iter()
        .any(|issue| issue.check == "Schematic validation")
}

/// What the run gate does with the design: `None` where it prepares, the
/// stage it refused at otherwise.
fn run_gate(
    controller: &mut crate::simulation::SimulationController,
    state: &AppState,
) -> Option<PreparationStage> {
    controller
        .prepare_run_set_for_preflight(state)
        .err()
        .map(|error| error.stage())
}

#[test]
fn a_rule_the_deck_promotes_blocks_the_page_preflight_and_the_run_alike() {
    let mut app = runnable_app();
    // A label on nothing: a warning by default, and nothing the engine minds.
    app.state
        .schematic
        .add_net_label(Point::new(600, 600), "spare".to_owned());
    app.state.sync_active_schematic_to_workspace();

    assert_eq!(
        page_levels(&mut app.state, DrcViolationType::OrphanNetLabel),
        [DiagnosticLevel::Warning]
    );
    assert!(
        !preflight_blocks_on_design_rules(&app),
        "{:?}",
        collect_report(&app.state).blockers
    );
    assert_eq!(
        run_gate(&mut app.simulation_controller, &app.state),
        None,
        "a warning travels with the run"
    );

    app.state
        .workspace
        .content
        .design_rules
        .set_severity(DrcViolationType::OrphanNetLabel, DrcSeverity::Error)
        .expect("an orphan label may be promoted");

    assert_eq!(
        page_levels(&mut app.state, DrcViolationType::OrphanNetLabel),
        [DiagnosticLevel::Error],
        "the page states the promoted severity"
    );
    assert!(
        preflight_blocks_on_design_rules(&app),
        "preflight blocks on what the page calls an error"
    );
    assert_eq!(
        run_gate(&mut app.simulation_controller, &app.state),
        Some(PreparationStage::DesignChecks),
        "the run gate refuses what the deck made an error, whether or not \
         preflight was run first"
    );
}

#[test]
fn a_rule_the_deck_demotes_lets_the_page_preflight_and_the_run_through_alike() {
    let mut app = runnable_app();
    // Two labels the deck folds into one node: an error by default.
    app.state
        .schematic
        .add_net_label(Point::new(600, 600), "sense".to_owned());
    app.state
        .schematic
        .add_net_label(Point::new(600, 700), "SENSE".to_owned());
    app.state.sync_active_schematic_to_workspace();

    let levels = page_levels(&mut app.state, DrcViolationType::CaseCollidingNetNames);
    assert_eq!(levels, [DiagnosticLevel::Error]);
    assert!(preflight_blocks_on_design_rules(&app));
    assert_eq!(
        run_gate(&mut app.simulation_controller, &app.state),
        Some(PreparationStage::DesignChecks)
    );

    app.state
        .workspace
        .content
        .design_rules
        .set_severity(
            DrcViolationType::CaseCollidingNetNames,
            DrcSeverity::Warning,
        )
        .expect("a case collision may be demoted");

    assert_eq!(
        page_levels(&mut app.state, DrcViolationType::CaseCollidingNetNames),
        [DiagnosticLevel::Warning],
        "the page states the demoted severity"
    );
    assert!(
        !preflight_blocks_on_design_rules(&app),
        "preflight does not block on what the page calls a warning"
    );
    assert_eq!(
        run_gate(&mut app.simulation_controller, &app.state),
        None,
        "the run gate passes what the deck demoted: it used to check the root \
         under the shipped defaults and refuse it"
    );
}

/// Press Run on the divider with a label on nothing, change what the rules
/// say about it with `edit` while the worker prepares the run, and finish the
/// pass.
fn run_with_the_rules_edited_in_flight(edit: impl FnOnce(&mut AppState)) -> RSpiceApp {
    let mut app = runnable_app();
    app.state
        .schematic
        .add_net_label(Point::new(600, 600), "spare".to_owned());
    app.state.sync_active_schematic_to_workspace();
    let release = app.simulation_controller.hold_next_preflight_worker();
    run_and_queue(&mut app);
    edit(&mut app.state);
    release.send(()).expect("the worker waits to prepare");
    app.settle_preflight();
    app
}

/// A deck or a waiver edited while a pass builds its decks moves none of the
/// revisions the pass inspected, and the dispatch refuses the run over it.
/// The pass is held to the dispatch's identity of what it prepared, so it
/// concludes stale: nothing is authorized or queued under rules that no
/// longer stand.
#[test]
fn rules_edited_during_a_pass_authorize_nothing() {
    let promoted = run_with_the_rules_edited_in_flight(|state| {
        state
            .workspace
            .content
            .design_rules
            .set_severity(DrcViolationType::OrphanNetLabel, DrcSeverity::Error)
            .expect("an orphan label may be promoted");
        // As applying the Rule Setup dialog does.
        state.workbench.preflight.invalidate();
    });
    let waived = run_with_the_rules_edited_in_flight(|state| {
        let target = page_waiver_target(state, DrcViolationType::OrphanNetLabel);
        state
            .waive_finding(&target, "a spare label")
            .expect("the finding is waived");
    });
    for (edit, app) in [("deck", promoted), ("waiver", waived)] {
        assert!(
            !app.state.simulation.execution.trigger_simulation,
            "{edit}: nothing is queued"
        );
        assert!(
            app.simulation_controller.retained_run_deck().is_none(),
            "{edit}: nothing is authorized"
        );
        assert!(
            app.state.workbench.preflight.report.is_none(),
            "{edit}: a stale pass retains no report"
        );
    }
}

/// A Run press after the rules changed under a pass in flight is not
/// absorbed into it: that pass will authorize nothing, so a press that
/// joined it would be lost with it. The pass stops, the press checks the
/// design under the rules as they stand, and its run is queued.
#[test]
fn a_run_press_after_the_rules_change_in_flight_restarts_the_pass() {
    let demoted = run_again_with_the_rules_edited_in_flight(|state| {
        state
            .workspace
            .content
            .design_rules
            .set_severity(DrcViolationType::OrphanNetLabel, DrcSeverity::Info)
            .expect("an orphan label may be set as a note");
        // As applying the Rule Setup dialog does.
        state.workbench.preflight.invalidate();
    });
    let waived = run_again_with_the_rules_edited_in_flight(|state| {
        let target = page_waiver_target(state, DrcViolationType::OrphanNetLabel);
        state
            .waive_finding(&target, "a spare label")
            .expect("the finding is waived");
    });
    for (edit, app) in [("deck", demoted), ("waiver", waived)] {
        assert!(
            app.state
                .log_buffer
                .entries()
                .any(|entry| entry.message.starts_with("Preflight restarted")),
            "{edit}: the restart is stated"
        );
        assert!(
            app.state.simulation.execution.trigger_simulation,
            "{edit}: the press after the edit queues its run"
        );
        assert!(
            app.simulation_controller.retained_run_deck().is_some(),
            "{edit}: the run under the edited rules is authorized"
        );
    }
}

/// Press Run on the divider with a label on nothing, change what the rules
/// say about it with `edit` while the worker prepares the run, press Run
/// again, and finish whatever pass that leaves.
fn run_again_with_the_rules_edited_in_flight(edit: impl FnOnce(&mut AppState)) -> RSpiceApp {
    let mut app = runnable_app();
    app.state
        .schematic
        .add_net_label(Point::new(600, 600), "spare".to_owned());
    app.state.sync_active_schematic_to_workspace();
    let release = app.simulation_controller.hold_next_preflight_worker();
    run_and_queue(&mut app);
    edit(&mut app.state);
    run_and_queue(&mut app);
    release.send(()).expect("the stale worker waits to prepare");
    app.settle_preflight();
    app
}

/// What waives the page's finding of `rule`, as its row offers it.
fn page_waiver_target(
    state: &mut AppState,
    rule: DrcViolationType,
) -> crate::workbench::app_state::waivers::WaiverTarget {
    use crate::workbench::app_state::diagnostic_store::listing;
    use crate::workbench::app_state::waivers::WaiverAction;

    state
        .run_active_design_checks()
        .expect("the configured design checks");
    let listed = listing::listing(state);
    (0..listed.len())
        .filter_map(|index| listed.row(index))
        .filter(|row| row.code == rule.rule_code())
        .find_map(|row| match row.waiver_action {
            Some(WaiverAction::Waive(target)) => Some(target.clone()),
            _ => None,
        })
        .expect("the page offers to waive the finding")
}
