//! Contract tests for capture groups on the Save page.
//!
//! Split from the page's other route tests because these assert one claim the
//! rest do not: that the number the page shows, the owner each output resolves
//! to, and the receipt an edit writes are all the same answer. The helpers they
//! need — rendering a page, naming the active plan — come from the parent.

use super::{RSpiceApp, SimulationPage, plan_id, render_with};

/// Seed one plan with two outputs in different scopes and a group over the
/// first, and hand back the group's identity.
fn seed_capture_group(app: &mut RSpiceApp) -> crate::product::CaptureGroupId {
    use crate::state::{
        CaptureGroup, CaptureGroupRule, InstancePath, SavedOutput, SavedOutputCompatibility,
        SavedOutputKind, SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming,
    };

    let id = plan_id(app);
    for (name, expression) in [("core_n", "V(x1.n)"), ("edge_n", "V(x2.n)")] {
        let output = SavedOutput::new(
            SavedOutputKind::RawVoltageOrCurrent,
            name,
            expression,
            SavedOutputCompatibility::AllCompatibleAnalyses,
            SavedOutputPolicy::SelectedAndFinalPoints,
            SavedOutputPrecision::FullSourcePrecision,
            SavedOutputStreaming::StoreOnly,
        )
        .expect("valid saved output");
        app.state
            .workspace
            .add_saved_output(id, output)
            .expect("the plan accepts the output");
    }
    let mut group = CaptureGroup::new("Core rails").expect("group name");
    group.rules.push(CaptureGroupRule::for_scope(
        InstancePath::parse_legacy("/x1").expect("scope"),
    ));
    group.points = Some(SavedOutputPolicy::EveryAcceptedPoint);
    app.state
        .workspace
        .add_capture_group(id, group)
        .expect("the plan accepts the group")
}

/// The Save page's own ledger, built exactly the way the page builds it.
fn page_capture_ledger(app: &RSpiceApp) -> crate::simulation::capture_ledger::CaptureLedger {
    let payload = app
        .state
        .workspace
        .plan_data(plan_id(app))
        .cloned()
        .expect("payload");
    let (outputs, reports, _, membership) = app
        .simulation_controller
        .effective_saved_outputs_preflight(
            &app.state,
            &payload.saved_outputs,
            &payload.capture_groups,
        )
        .expect("the test design resolves an output set");
    crate::simulation::capture_ledger::CaptureLedger::resolve(
        &payload.capture_groups,
        &outputs,
        &reports,
        &membership,
        app.state.sim_setup.save_policy.output_selection_mode,
        app.state.sim_setup.enabled_analysis_instance_count(),
        u64::try_from(app.state.sim_setup.run_set.point_count()).unwrap_or(u64::MAX),
    )
}

/// The forecast the Save page shows is the forecast preparation refuses on, so
/// it has exactly one owner: every group row, plus the plan-level allowances,
/// adds up to the total and nothing else contributes to it.
///
/// The second half is what the one-owner rule buys: the outputs counted across
/// the rows are the outputs there are, so no group's subtotal can include an
/// output another group is also charging for.
#[test]
fn the_capture_ledger_total_is_the_sum_of_its_rows_and_counts_each_output_once() {
    let mut app = RSpiceApp::test_instance();
    seed_capture_group(&mut app);
    let payload = app
        .state
        .workspace
        .plan_data(plan_id(&app))
        .cloned()
        .expect("payload");

    let ledger = page_capture_ledger(&app);

    let rows: u64 = ledger.rows().iter().map(|row| row.bytes).sum();
    assert_eq!(
        ledger.total_bytes(),
        rows + ledger.shared_source_bytes() + ledger.engine_ceiling_bytes().unwrap_or(0),
        "the total is a projection of the printed lines"
    );
    assert_eq!(
        ledger.rows().iter().map(|row| row.outputs).sum::<usize>(),
        payload.saved_outputs.len(),
        "every authored output is counted in exactly one group"
    );
    let core = ledger
        .rows()
        .iter()
        .find(|row| row.name == "Core rails")
        .expect("the authored group has a row");
    assert_eq!(core.outputs, 1, "only the /x1 output is inside the rule");
    assert_eq!(
        ledger
            .rows()
            .last()
            .expect("the fallback row is always drawn")
            .name,
        crate::state::UNGROUPED_NAME,
        "the fallback group is listed last, after every group that can claim first"
    );
}

/// A group's override is what the run will execute, not a label on a table.
///
/// The projection runs on the route preparation uses, so asserting it here is
/// asserting what the dispatched contracts carry.
#[test]
fn a_capture_group_override_reaches_the_effective_output_the_run_will_use() {
    use crate::state::SavedOutputPolicy;

    let mut app = RSpiceApp::test_instance();
    seed_capture_group(&mut app);
    let payload = app
        .state
        .workspace
        .plan_data(plan_id(&app))
        .cloned()
        .expect("payload");

    let (outputs, _, _, _) = app
        .simulation_controller
        .effective_saved_outputs_preflight(
            &app.state,
            &payload.saved_outputs,
            &payload.capture_groups,
        )
        .expect("the test design resolves an output set");

    let core = outputs
        .iter()
        .find(|output| output.name == "core_n")
        .expect("the grouped output survives selection");
    let edge = outputs
        .iter()
        .find(|output| output.name == "edge_n")
        .expect("the ungrouped output survives selection");
    assert_eq!(
        core.save_policy,
        SavedOutputPolicy::EveryAcceptedPoint,
        "the owning group's override is what the run receives"
    );
    assert_eq!(
        edge.save_policy,
        SavedOutputPolicy::SelectedAndFinalPoints,
        "and an output no group claims keeps its own contract"
    );
}

/// Membership is a function of the registry, so a registry edit can move
/// outputs between groups without anybody editing a group. The receipt for
/// that edit has to say so, or a plan's capture policy changes with no
/// evidence that it did.
#[test]
fn adding_an_output_inside_a_rule_receipts_the_group_it_joined() {
    use crate::state::{
        SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
        SavedOutputPrecision, SavedOutputStreaming,
    };

    let mut app = RSpiceApp::test_instance();
    seed_capture_group(&mut app);
    let id = plan_id(&app);
    let arriving = SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        "core_m",
        "V(x1.m)",
        SavedOutputCompatibility::AllCompatibleAnalyses,
        SavedOutputPolicy::SelectedAndFinalPoints,
        SavedOutputPrecision::FullSourcePrecision,
        SavedOutputStreaming::StoreOnly,
    )
    .expect("valid saved output");

    assert!(
        super::super::workflows::commit_plan_change(
            &mut app,
            id,
            "Added saved output core_m.",
            move |workspace, plan_id| {
                workspace
                    .add_saved_output(plan_id, arriving)
                    .map_err(|error| error.to_string())
            },
        ),
        "the registry edit commits"
    );

    let detail = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("plan")
        .configuration_receipts()
        .last()
        .expect("the edit wrote a receipt")
        .detail()
        .to_owned();
    assert!(
        detail.starts_with("Added saved output core_m."),
        "the receipt still says what the edit was: {detail}"
    );
    assert!(
        detail.contains("core_m joined Core rails"),
        "and names the group the new output joined: {detail}"
    );
}

/// The mirror: an edit that moves nothing must not decorate its receipt with a
/// membership sentence, or every receipt reads as a policy change.
#[test]
fn an_edit_that_moves_nothing_receipts_only_what_it_changed() {
    let mut app = RSpiceApp::test_instance();
    let group = seed_capture_group(&mut app);
    let id = plan_id(&app);

    assert!(
        super::super::workflows::commit_plan_change(
            &mut app,
            id,
            "Moved capture group Core rails later in resolution order.",
            move |workspace, plan_id| {
                workspace
                    .reorder_capture_group(plan_id, group, false)
                    .map_err(|error| error.to_string())
            },
        ),
        "the reorder commits"
    );

    let detail = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("plan")
        .configuration_receipts()
        .last()
        .expect("the edit wrote a receipt")
        .detail()
        .to_owned();
    assert_eq!(
        detail, "Moved capture group Core rails later in resolution order.",
        "nothing changed hands, so nothing is claimed to have"
    );
}

/// The Save page draws the groups it prices, including the one nobody authored.
#[test]
fn the_save_page_lists_every_capture_group_and_the_fallback() {
    let rendered = render_with(SimulationPage::Save, 1400.0, |app| {
        seed_capture_group(app);
    });

    assert!(
        rendered.contains("Capture groups"),
        "the card is on the page: {rendered}"
    );
    assert!(
        rendered.contains("Core rails"),
        "the authored group has a row: {rendered}"
    );
    assert!(
        rendered.contains(crate::state::UNGROUPED_NAME),
        "and so does the group holding everything unclaimed: {rendered}"
    );
    assert!(
        rendered.contains("Add group"),
        "the card can author one: {rendered}"
    );
}

/// Naming an output into a group is reachable from the page that owns the
/// output, and the control says which group holds it and how.
///
/// "By rule · X" and "X" are different answers to different questions: the
/// first says moving a rule would move this output, the second says nothing
/// but releasing it will.
#[test]
fn the_outputs_page_states_which_capture_group_holds_the_selected_output() {
    let rendered = render_with(SimulationPage::Outputs, 1400.0, |app| {
        seed_capture_group(app);
        app.state.workbench.selected_saved_output = Some("core_n".to_owned());
    });

    assert!(
        rendered.contains("Capture group"),
        "the record editor offers the membership control: {rendered}"
    );
    assert!(
        rendered.contains("By rule · Core rails"),
        "and states that a rule is what put this output there: {rendered}"
    );
}

/// Naming an output through the page's transaction beats the rule that would
/// otherwise have claimed it, and the receipt says where it went.
#[test]
fn naming_an_output_into_a_group_outranks_the_rule_that_had_it() {
    use crate::state::CaptureGroup;

    let mut app = RSpiceApp::test_instance();
    seed_capture_group(&mut app);
    let id = plan_id(&app);
    let watchlist = CaptureGroup::new("Watchlist").expect("group");
    let watchlist_id = watchlist.id;
    app.state
        .workspace
        .add_capture_group(id, watchlist)
        .expect("second group");
    let output_id = app
        .state
        .workspace
        .plan_data(id)
        .expect("payload")
        .saved_outputs[0]
        .id;

    assert!(
        super::super::workflows::commit_plan_change(
            &mut app,
            id,
            "Named saved output core_n into capture group Watchlist.",
            move |workspace, plan_id| {
                workspace
                    .set_capture_group_member(plan_id, watchlist_id, output_id, true)
                    .map_err(|error| error.to_string())
            },
        ),
        "the naming commits"
    );

    let payload = app.state.workspace.plan_data(id).expect("payload");
    let membership = crate::state::CaptureGroupMembership::resolve(
        &payload.capture_groups,
        &payload.saved_outputs,
    );
    assert_eq!(
        membership.owner(0),
        watchlist_id,
        "the named group takes the output off the rule group that matched it"
    );
    let detail = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("plan")
        .configuration_receipts()
        .last()
        .expect("receipt")
        .detail()
        .to_owned();
    assert!(
        detail.contains("core_n moved Core rails → Watchlist"),
        "and the receipt names both ends of the move: {detail}"
    );
}

/// The group editor shows one rule. A group carrying more must not lose them
/// when a name or a policy is edited through that form.
#[test]
fn editing_a_group_keeps_the_rules_and_members_the_form_never_showed() {
    use crate::state::{CaptureGroup, CaptureGroupRule, InstancePath, SavedOutputKind};

    let mut app = RSpiceApp::test_instance();
    seed_capture_group(&mut app);
    let id = plan_id(&app);
    let mut group = CaptureGroup::new("Two rules").expect("group");
    group.rules.push(CaptureGroupRule::for_scope(
        InstancePath::parse_legacy("/x3").expect("scope"),
    ));
    group.rules.push(CaptureGroupRule::for_kind(
        SavedOutputKind::NoiseContributor,
    ));
    let group_id = app
        .state
        .workspace
        .add_capture_group(id, group.clone())
        .expect("the plan accepts a two-rule group");
    let output_id = app
        .state
        .workspace
        .plan_data(id)
        .expect("payload")
        .saved_outputs[1]
        .id;
    app.state
        .workspace
        .set_capture_group_member(id, group_id, output_id, true)
        .expect("named member");

    let mut draft = super::super::workflows::group_draft(&group);
    draft.name = "Renamed".to_owned();
    super::super::workflows::commit_capture_group(&mut app, &draft).expect("the edit commits");

    let stored = app
        .state
        .workspace
        .plan_data(id)
        .expect("payload")
        .capture_groups
        .iter()
        .find(|candidate| candidate.id == group_id)
        .expect("the group survives the edit")
        .clone();
    assert_eq!(stored.name, "Renamed");
    assert_eq!(
        stored.rules.len(),
        2,
        "a rule the form does not show is not a rule the form may delete"
    );
    assert_eq!(
        stored.rules[1].kind,
        Some(SavedOutputKind::NoiseContributor)
    );
    assert_eq!(
        stored.members,
        vec![output_id],
        "and the outputs it named directly survive too"
    );
}
