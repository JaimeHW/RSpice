//! Contract tests for capture groups on the Save page.
//!
//! Split from the page's other route tests because these assert one claim the
//! rest do not: that the number the page shows, the owner each output resolves
//! to, and the receipt an edit writes are all the same answer. The helpers they
//! need — rendering a page, naming the active plan — come from the parent.

use super::plan_id;
use crate::workbench::app_state::Model;
use crate::workbench::simulate::test_host::SimulateTestHost;

/// Seed one plan with two outputs in different scopes and a group over the
/// first, and hand back the group's identity.
pub(crate) fn seed_capture_group(model: &mut Model) -> crate::product::CaptureGroupId {
    use crate::state::{
        CaptureGroup, CaptureGroupRule, InstancePath, SavedOutput, SavedOutputCompatibility,
        SavedOutputKind, SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming,
    };

    let id = plan_id(model);
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
        model
            .workspace
            .content
            .add_saved_output(id, output)
            .expect("the plan accepts the output");
    }
    let mut group = CaptureGroup::new("Core rails").expect("group name");
    group.rules.push(CaptureGroupRule::for_scope(
        InstancePath::parse_legacy("/x1").expect("scope"),
    ));
    group.points = Some(SavedOutputPolicy::EveryAcceptedPoint);
    model
        .workspace
        .content
        .add_capture_group(id, group)
        .expect("the plan accepts the group")
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

    let mut harness = SimulateTestHost::harness();
    seed_capture_group(&mut harness.model);
    let id = plan_id(&harness.model);
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
            &mut harness.cx(),
            id,
            "Added saved output core_m.",
            move |workspace, plan_id| {
                workspace
                    .content
                    .add_saved_output(plan_id, arriving)
                    .map_err(|error| error.to_string())
            },
        ),
        "the registry edit commits"
    );

    let detail = harness
        .model
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
    let mut harness = SimulateTestHost::harness();
    let group = seed_capture_group(&mut harness.model);
    let id = plan_id(&harness.model);

    assert!(
        super::super::workflows::commit_plan_change(
            &mut harness.cx(),
            id,
            "Moved capture group Core rails later in resolution order.",
            move |workspace, plan_id| {
                workspace
                    .content
                    .reorder_capture_group(plan_id, group, false)
                    .map_err(|error| error.to_string())
            },
        ),
        "the reorder commits"
    );

    let detail = harness
        .model
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

/// Naming an output through the page's transaction beats the rule that would
/// otherwise have claimed it, and the receipt says where it went.
#[test]
fn naming_an_output_into_a_group_outranks_the_rule_that_had_it() {
    use crate::state::CaptureGroup;

    let mut harness = SimulateTestHost::harness();
    seed_capture_group(&mut harness.model);
    let id = plan_id(&harness.model);
    let watchlist = CaptureGroup::new("Watchlist").expect("group");
    let watchlist_id = watchlist.id;
    harness
        .model
        .workspace
        .content
        .add_capture_group(id, watchlist)
        .expect("second group");
    let output_id = harness
        .model
        .workspace
        .content
        .plan_data(id)
        .expect("payload")
        .saved_outputs[0]
        .id;

    assert!(
        super::super::workflows::commit_plan_change(
            &mut harness.cx(),
            id,
            "Named saved output core_n into capture group Watchlist.",
            move |workspace, plan_id| {
                workspace
                    .content
                    .set_capture_group_member(plan_id, watchlist_id, output_id, true)
                    .map_err(|error| error.to_string())
            },
        ),
        "the naming commits"
    );

    let payload = harness
        .model
        .workspace
        .content
        .plan_data(id)
        .expect("payload");
    let membership = crate::state::CaptureGroupMembership::resolve(
        &payload.capture_groups,
        &payload.saved_outputs,
    );
    assert_eq!(
        membership.owner(0),
        watchlist_id,
        "the named group takes the output off the rule group that matched it"
    );
    let detail = harness
        .model
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

    let mut harness = SimulateTestHost::harness();
    seed_capture_group(&mut harness.model);
    let id = plan_id(&harness.model);
    let mut group = CaptureGroup::new("Two rules").expect("group");
    group.rules.push(CaptureGroupRule::for_scope(
        InstancePath::parse_legacy("/x3").expect("scope"),
    ));
    group.rules.push(CaptureGroupRule::for_kind(
        SavedOutputKind::NoiseContributor,
    ));
    let group_id = harness
        .model
        .workspace
        .content
        .add_capture_group(id, group.clone())
        .expect("the plan accepts a two-rule group");
    let output_id = harness
        .model
        .workspace
        .content
        .plan_data(id)
        .expect("payload")
        .saved_outputs[1]
        .id;
    harness
        .model
        .workspace
        .content
        .set_capture_group_member(id, group_id, output_id, true)
        .expect("named member");

    let mut draft = super::super::workflows::group_draft(&group);
    draft.name = "Renamed".to_owned();
    super::super::workflows::commit_capture_group(&mut harness.cx(), &draft)
        .expect("the edit commits");

    let stored = harness
        .model
        .workspace
        .content
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
