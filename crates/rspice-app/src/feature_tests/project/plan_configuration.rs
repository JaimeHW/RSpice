//! The configuration a simulation plan runs, through whole application
//! frames: a plan follows the configuration marked for runs until it names
//! another, its deck and its run are that configuration's, Undo puts the
//! choice back, one naming a deleted configuration is refused before
//! anything is solved, and every result names the configuration it ran.

#![cfg(not(target_arch = "wasm32"))]

use std::time::{Duration, Instant};

use egui::Context;

use super::configuration_editing::{nodes, pick, settle};
use super::recovery_frames::{offscreen, painted};
use crate::workbench::examples::hierarchy_reference;
use crate::workbench::{RSpiceApp, features};
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_project::workbench::surfaces::project::overview::{
    OverviewIntent, OverviewSnapshot,
};
use rspice_app_runs::simulation::controller::SimulationController;
use rspice_app_simulate::workbench::simulate::commands::SimulateCommand;
use rspice_app_state::product::SimulationPlanId;
use rspice_app_state::state::{
    ConfigurationPlatform, ConfigurationSetId, ConfigurationSetOverride, SimulationPage,
    SimulationRunIntent,
};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::ids;
use rspice_simulation_contract::design_configuration::DesignConfiguration;
use rspice_simulation_contract::plan_catalog::SimulationPlanCloneOptions;

const REFERENCE: &str = "Reference hierarchy";
const VENDOR: &str = "Vendor filter";

/// The reference project on Simulate > Analyses, open, with a second
/// configuration beside the one runs use: [`VENDOR`], which runs `XF1` as
/// the vendor's filter. Returns the two configurations, the marked one first.
fn analyses_app() -> (RSpiceApp, ConfigurationSetId, ConfigurationSetId) {
    let reference = hierarchy_reference::build();
    let mut app = RSpiceApp::test_instance();
    app.state = reference.state;
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Simulate);
    app.state.simulate.simulation_page = SimulationPage::Analyses;
    let vendor = vendor_configuration(&mut app);
    (app, reference.configuration, vendor)
}

fn vendor_configuration(app: &mut RSpiceApp) -> ConfigurationSetId {
    let mut catalog = (*app.state.model.workspace.content.configuration_sets).clone();
    let marked = catalog
        .active_configuration_id()
        .expect("one is used for runs");
    let mut definition = catalog
        .find(marked)
        .expect("the marked configuration exists")
        .definition()
        .clone();
    definition.name = VENDOR.to_owned();
    definition.overrides.push(ConfigurationSetOverride {
        instance_path: "/XF1".to_owned(),
        executable_views: vec!["schematic".to_owned()],
        stop_view: None,
        model_section: None,
        eligible_platforms: ConfigurationPlatform::ALL.to_vec(),
        target: Some(rspice_design::configuration_set::BindingTarget::new(
            "vendor", "filter",
        )),
    });
    let vendor = catalog
        .create(definition)
        .expect("the vendor configuration holds");
    catalog.activate(marked).expect("runs keep the reference");
    publish(app, catalog);
    vendor
}

fn publish(app: &mut RSpiceApp, catalog: rspice_app_state::state::ConfigurationSetCatalog) {
    app.state
        .model
        .workspace
        .content
        .replace_configuration_sets(catalog)
        .expect("the catalog publishes");
}

/// Mark `id` as the configuration used for runs, as its page's Use for runs
/// does.
fn mark_for_runs(app: &mut RSpiceApp, id: ConfigurationSetId) {
    let mut catalog = (*app.state.model.workspace.content.configuration_sets).clone();
    catalog.activate(id).expect("the configuration exists");
    publish(app, catalog);
}

/// The design deck the active plan's run executes, as it stands.
fn deck(app: &RSpiceApp) -> String {
    SimulationController::prepare_design_netlist_for_inspection(&app.state.model)
        .unwrap_or_else(|error| panic!("the design netlists: {error}"))
}

/// The configuration the deck's provenance line names.
fn deck_configuration(deck: &str) -> String {
    deck.lines()
        .find_map(|line| line.strip_prefix("* RSpice configuration-set "))
        .and_then(|rest| rest.split_whitespace().next())
        .unwrap_or_else(|| panic!("the deck names no configuration:\n{deck}"))
        .to_owned()
}

/// The master the top-level card of `instance` names.
fn master_of(deck: &str, instance: &str) -> String {
    let needle = format!("{} ", instance.to_ascii_lowercase());
    let mut depth = 0_usize;
    deck.lines()
        .find(|line| {
            let folded = line.trim_start().to_ascii_lowercase();
            if folded.starts_with(".subckt ") {
                depth += 1;
                return false;
            }
            if folded.starts_with(".ends") {
                depth = depth.saturating_sub(1);
                return false;
            }
            depth == 0 && folded.starts_with(&needle)
        })
        .and_then(|line| line.split_whitespace().last())
        .unwrap_or_else(|| panic!("no card for {instance} in:\n{deck}"))
        .to_owned()
}

/// Whether the frame painted `text` on one line.
fn shows(output: &egui::FullOutput, text: &str) -> bool {
    painted(output).lines().any(|line| line == text)
}

fn design(app: &RSpiceApp) -> DesignConfiguration {
    app.state.model.sim_setup.design_configuration
}

fn marked(app: &RSpiceApp) -> Option<ConfigurationSetId> {
    app.state
        .model
        .workspace
        .content
        .configuration_sets
        .active_configuration_id()
}

/// A plan that never chose runs whatever the project marks for runs, and the
/// select says which: marking another configuration moves the plan's deck
/// with it.
#[test]
fn a_plan_follows_the_configuration_marked_for_runs() {
    let ctx = offscreen();
    let (mut app, reference, vendor) = analyses_app();
    assert_eq!(design(&app), DesignConfiguration::UsedForRuns);

    let output = settle(&ctx, &mut app, Vec::new());
    assert!(
        shows(&output, &format!("Used for runs \u{b7} {REFERENCE}")),
        "the select states what the plan follows:\n{}",
        painted(&output)
    );
    let placed = deck(&app);
    assert_eq!(deck_configuration(&placed), reference.to_string());
    assert_ne!(master_of(&placed, "XF1"), master_of(&placed, "XF2"));

    mark_for_runs(&mut app, vendor);
    let output = settle(&ctx, &mut app, Vec::new());
    assert!(
        shows(&output, &format!("Used for runs \u{b7} {VENDOR}")),
        "{}",
        painted(&output)
    );
    let bound = deck(&app);
    assert_eq!(deck_configuration(&bound), vendor.to_string());
    assert_eq!(
        master_of(&bound, "XF1"),
        master_of(&bound, "XF2"),
        "XF1 runs the vendor's filter, as XF2 does"
    );
    assert_eq!(design(&app), DesignConfiguration::UsedForRuns);
}

/// Picked in the select, a configuration is the plan's own: the plan's deck
/// is that configuration's while the project's mark stays where it was. The
/// pick is one step of the Simulate page's Undo, which puts the plan and its
/// deck back exactly, and Redo takes it again.
#[test]
fn a_picked_configuration_is_the_plans_and_undo_restores_the_choice() {
    let ctx = offscreen();
    let (mut app, reference, vendor) = analyses_app();
    let before = deck(&app);
    let output = settle(&ctx, &mut app, Vec::new());

    let output = pick(&ctx, &mut app, &output, "Configuration", VENDOR);
    assert_eq!(design(&app), DesignConfiguration::Named(vendor));
    assert_eq!(marked(&app), Some(reference), "the project's mark stays");
    assert!(shows(&output, VENDOR), "{}", painted(&output));
    let picked = deck(&app);
    assert_eq!(deck_configuration(&picked), vendor.to_string());
    assert_eq!(master_of(&picked, "XF1"), master_of(&picked, "XF2"));

    app.execute_command_key(ids::UNDO);
    assert_eq!(design(&app), DesignConfiguration::UsedForRuns);
    assert_eq!(deck(&app), before, "the deck is the one before the pick");
    assert!(
        !app.state.model.can_undo_simulation_setup(),
        "the pick was one step"
    );
    let output = settle(&ctx, &mut app, Vec::new());
    assert!(
        shows(&output, &format!("Used for runs \u{b7} {REFERENCE}")),
        "{}",
        painted(&output)
    );

    app.execute_command_key(ids::REDO);
    assert_eq!(design(&app), DesignConfiguration::Named(vendor));
    assert_eq!(deck(&app), picked);
}

/// Run presses the whole preflight. A plan whose configuration was deleted
/// since it was picked is refused there, with the reason and where to choose
/// another, before anything is prepared or solved: no run is created, and
/// preparing the run directly is refused with the same reason.
#[test]
fn a_plan_naming_a_deleted_configuration_is_refused_before_solving() {
    let ctx = offscreen();
    let (mut app, _, vendor) = analyses_app();
    let output = settle(&ctx, &mut app, Vec::new());
    pick(&ctx, &mut app, &output, "Configuration", VENDOR);
    let mut catalog = (*app.state.model.workspace.content.configuration_sets).clone();
    let revision = catalog.find(vendor).expect("it exists").revision();
    catalog
        .remove(vendor, revision)
        .expect("a configuration not marked for runs can be deleted");
    publish(&mut app, catalog);

    let output = settle(&ctx, &mut app, Vec::new());
    assert!(
        shows(&output, "Deleted configuration"),
        "the select says the plan's configuration is gone:\n{}",
        painted(&output)
    );

    let runs = app.state.model.simulation.retained.run_count();
    app.simulation_controller
        .update(&mut app.state, &*app.export_workflow_io);
    app.execute_command_key(SimulateCommand::Run.key());
    assert!(app.state.runs.preflight.take_run_and_queue_request());
    rspice_app_runs::workbench::preflight::start(
        &mut app.state,
        &mut app.simulation_controller,
        true,
    );
    app.settle_preflight();
    let report = app
        .state
        .runs
        .preflight
        .report
        .as_ref()
        .expect("Run leaves its report");
    assert!(
        report
            .blockers
            .iter()
            .any(|blocker| blocker.check == "Plan configuration"
                && blocker
                    .observed
                    .starts_with("The configuration this plan runs was deleted.")
                && blocker.remediation
                    == rspice_app_state::state::PreflightRemediation::SimulationPlan {
                        analysis: None
                    }),
        "{:#?}",
        report.blockers
    );
    assert!(!app.state.model.simulation.execution.trigger_simulation);
    app.simulation_controller
        .update(&mut app.state, &*app.export_workflow_io);
    assert_eq!(app.state.model.simulation.retained.run_count(), runs);

    let refused = SimulationController::build_prepared_snapshot(
        &app.state,
        SimulationRunIntent::SimulateRunSet,
    )
    .expect_err("the run is refused");
    assert!(
        refused
            .to_string()
            .contains("The configuration this plan runs was deleted."),
        "{refused}"
    );
}

/// A second plan cloned from the first, as Clone plan does, and made the
/// active one.
fn clone_active_plan(app: &mut RSpiceApp, name: &str) -> SimulationPlanId {
    let model = &mut app.state.model;
    let source = model
        .sim_setup
        .stable_analysis_plan()
        .expect("a plan is active")
        .id();
    model.workspace.content.migrate_active_plan_data(source);
    let outcome = model
        .sim_setup
        .clone_active_plan(name, SimulationPlanCloneOptions::default())
        .expect("the plan clones");
    model
        .workspace
        .content
        .clone_plan_data(
            outcome.source_plan_id,
            outcome.cloned_plan_id,
            outcome.contents.copy_variables_outputs_and_specifications,
            outcome.contents.copy_regression_baseline_ownership,
            &outcome.analysis_identity_map,
        )
        .expect("its data clones");
    outcome.cloned_plan_id
}

/// Run `plans` as one campaign, to its end.
fn run_campaign(app: &mut RSpiceApp, plans: &[SimulationPlanId]) {
    app.simulation_controller
        .prepare_and_start_campaign(&mut app.state, "Two designs", plans)
        .expect("each plan prepares under its own configuration");
    let deadline = Instant::now() + Duration::from_secs(300);
    while app.simulation_controller.active_campaign.is_some()
        || app
            .state
            .model
            .simulation
            .retained
            .runs
            .iter()
            .any(|run| !run.lifecycle.is_terminal())
    {
        assert!(Instant::now() < deadline, "the campaign never finished");
        std::thread::sleep(Duration::from_millis(1));
        app.simulation_controller
            .update(&mut app.state, &*app.export_workflow_io);
    }
}

/// The configuration the retained run of `plan` recorded.
fn recorded(
    app: &RSpiceApp,
    plan: SimulationPlanId,
) -> rspice_results::run_receipt::RunConfiguration {
    app.state
        .model
        .simulation
        .retained
        .runs
        .iter()
        .filter_map(|run| run.prepared_receipt())
        .find(|receipt| receipt.simulation_plan_id() == Some(plan))
        .unwrap_or_else(|| panic!("plan {plan} ran"))
        .configuration()
        .expect("a design run records its configuration")
        .clone()
}

/// The configuration each run row on the Overview's Activity card names, as
/// the page keeps it: its Configuration column, which the card keeps at
/// every width.
fn activity(ctx: &Context, app: &mut RSpiceApp) -> Vec<String> {
    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Overview;
    settle(ctx, app, Vec::new());
    OverviewSnapshot::capture(ctx, &features::cx_with_foreign_answers(app, |_| true))
        .operations
        .iter()
        .filter(|operation| matches!(operation.intent, Some(OverviewIntent::OpenRun(_))))
        .map(|operation| operation.configuration.clone())
        .collect()
}

/// That a run row names `words` as its configuration.
fn row_naming(rows: &[String], words: &str) {
    assert!(
        rows.iter().any(|configuration| configuration == words),
        "no run row names {words:?}: {rows:#?}"
    );
}

/// One session, two plans on two configurations, run side by side as one
/// campaign: each run elaborates its own plan's configuration and its result
/// names it by identity, name and revision, the Overview's activity rows and
/// the run's inspector say which, and a configuration changed or deleted
/// since reads so. The record survives the project's save and reopen.
#[test]
fn two_plans_on_two_configurations_run_together_and_each_result_names_its_own() {
    let ctx = offscreen();
    let (mut app, reference, vendor) = analyses_app();
    let first = app
        .state
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("a plan is active")
        .id();
    let second = clone_active_plan(&mut app, "Vendor filter run");
    assert_eq!(
        design(&app),
        DesignConfiguration::UsedForRuns,
        "a clone keeps what its source runs"
    );
    let output = settle(&ctx, &mut app, Vec::new());
    pick(&ctx, &mut app, &output, "Configuration", VENDOR);
    assert_eq!(design(&app), DesignConfiguration::Named(vendor));

    run_campaign(&mut app, &[first, second]);

    let catalog = app.state.model.workspace.content.configuration_sets.clone();
    let revision_of = |id| catalog.find(id).expect("it exists").revision();
    let on_reference = recorded(&app, first);
    assert_eq!(
        (
            on_reference.id,
            on_reference.name.as_str(),
            on_reference.revision
        ),
        (Some(reference), REFERENCE, revision_of(reference))
    );
    let on_vendor = recorded(&app, second);
    assert_eq!(
        (on_vendor.id, on_vendor.name.as_str(), on_vendor.revision),
        (Some(vendor), VENDOR, revision_of(vendor))
    );
    assert_eq!(marked(&app), Some(reference), "neither run moved the mark");

    let rows = activity(&ctx, &mut app);
    row_naming(&rows, REFERENCE);
    row_naming(&rows, VENDOR);

    // Saved and reopened, each run still names its configuration.
    let project =
        rspice_app_services::workbench::lifecycle::project_lifecycle::snapshot(&app.state)
            .expect("the project snapshots");
    let json = rspice_app_services::io::project_io::serialize_project_file(&project)
        .expect("the project serializes");
    let loaded = rspice_app_services::io::project_io::load_project_text(&json, None)
        .expect("the project reloads");
    let restored =
        rspice_app_services::io::simulation_state_from_results(loaded.file.simulation_results)
            .expect("its results restore");
    let mut names = restored
        .retained
        .runs
        .iter()
        .filter_map(|run| run.prepared_receipt()?.configuration())
        .map(|recorded| recorded.name.clone())
        .collect::<Vec<_>>();
    names.sort();
    assert_eq!(names, [REFERENCE, VENDOR]);

    // The reference edited since: its run says so.
    let mut catalog = (*app.state.model.workspace.content.configuration_sets).clone();
    let mut definition = catalog.find(reference).unwrap().definition().clone();
    definition.stop_views = vec!["schematic".to_owned()];
    catalog
        .update(reference, revision_of(reference), definition)
        .expect("the edit holds");
    publish(&mut app, catalog);
    // The vendor's deleted since: its run says that.
    let mut catalog = (*app.state.model.workspace.content.configuration_sets).clone();
    catalog
        .remove(vendor, revision_of(vendor))
        .expect("a configuration not marked for runs can be deleted");
    publish(&mut app, catalog);
    let rows = activity(&ctx, &mut app);
    row_naming(&rows, &format!("{REFERENCE} (changed)"));
    row_naming(&rows, &format!("{VENDOR} (deleted)"));

    // The run's details in the Results inspector say the same.
    let index = app
        .state
        .model
        .simulation
        .retained
        .runs
        .iter()
        .position(|run| {
            run.prepared_receipt()
                .is_some_and(|receipt| receipt.simulation_plan_id() == Some(second))
        })
        .expect("the vendor run is retained");
    app.state.model.simulation.view.active_run_idx = Some(index);
    app.state.workbench.activate(Workspace::Results);
    let output = settle(&ctx, &mut app, Vec::new());
    assert!(
        announces(&output, "Configuration", &format!("{VENDOR} (deleted)")),
        "the inspector names the run's configuration"
    );

    // Run details… states it by identity and revision as well.
    let run_id = app.state.model.simulation.retained.runs[index].id;
    rspice_app_results::workbench::docks::inspector::run_details::open(
        &mut app.state.results.run_details,
        &app.state.model.simulation,
        run_id,
    );
    let output = settle(&ctx, &mut app, Vec::new());
    assert!(
        announces(
            &output,
            "Configuration ID",
            &format!("{vendor} \u{b7} revision {}", on_vendor.revision)
        ),
        "Run details names the configuration by identity and revision"
    );
}

/// Whether the frame announces a row `label` holding `value`.
fn announces(output: &egui::FullOutput, label: &str, value: &str) -> bool {
    nodes(output)
        .iter()
        .any(|node| node.label == label && node.value == value)
}

/// The Overview names the configuration the next run uses: once the plan
/// names one of its own, its Configuration row says that one, as the
/// Analyses page's select says, while the project's mark stays where it was
/// and the inspector's Configuration for runs names the mark.
#[test]
fn the_overview_names_the_configuration_the_plan_runs() {
    let ctx = offscreen();
    let (mut app, reference, _) = analyses_app();
    let output = settle(&ctx, &mut app, Vec::new());
    pick(&ctx, &mut app, &output, "Configuration", VENDOR);
    assert_eq!(marked(&app), Some(reference), "the project's mark stays");

    app.state.workbench.activate(Workspace::Project);
    app.state.project.project_page = ProjectPage::Overview;
    let shown = painted(&settle(&ctx, &mut app, Vec::new()));
    let lines: Vec<&str> = shown.lines().collect();
    // The row states the configuration first, then what it binds.
    assert!(
        lines
            .iter()
            .any(|line| line.starts_with(&format!("{VENDOR} \u{b7} "))),
        "the row names the plan's configuration:\n{shown}"
    );
    assert!(
        lines.contains(&REFERENCE),
        "the inspector names the one marked for runs:\n{shown}"
    );
}
