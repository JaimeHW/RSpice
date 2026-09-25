//! App-facing capture group aliases and project transaction integration tests.

pub(crate) use rspice_simulation_contract::capture_group::validate_plan_groups;
pub use rspice_simulation_contract::capture_group::{
    CaptureGroup, CaptureGroupError, CaptureGroupMembership, CaptureGroupRule, MembershipMove,
    UNGROUPED_NAME, collation_key, group_namer, normalize_name,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::SimulationPlanId;
    use crate::state::workspace::SimulationConfigurationError;
    use crate::state::{
        InstancePath, ProjectWorkspace, SavedOutput, SavedOutputCompatibility, SavedOutputKind,
        SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming, SimulationPlanPayload,
        SimulationPlanPayloadRecord,
    };

    fn capture_output(name: &str, expression: &str) -> SavedOutput {
        SavedOutput::new(
            SavedOutputKind::RawVoltageOrCurrent,
            name,
            expression,
            SavedOutputCompatibility::OpTranAc,
            SavedOutputPolicy::SelectedAndFinalPoints,
            SavedOutputPrecision::FullSourcePrecision,
            SavedOutputStreaming::StoreOnly,
        )
        .expect("saved output")
    }

    /// A plan holding two outputs and one scoped group over the first.
    fn workspace_with_capture_group() -> (ProjectWorkspace, SimulationPlanId, CaptureGroup) {
        let plan_id = SimulationPlanId::new();
        let mut workspace = ProjectWorkspace::default();
        workspace
            .simulation_plan_payloads
            .push(SimulationPlanPayloadRecord {
                plan_id,
                payload: SimulationPlanPayload::default(),
            });
        workspace
            .add_saved_output(plan_id, capture_output("core", "V(x1.n)"))
            .expect("first output");
        workspace
            .add_saved_output(plan_id, capture_output("edge", "V(x2.n)"))
            .expect("second output");
        let mut group = CaptureGroup::new("Core rails").expect("group name");
        group.rules.push(CaptureGroupRule::for_scope(
            InstancePath::parse_legacy("/x1").expect("scope"),
        ));
        group.points = Some(SavedOutputPolicy::EveryAcceptedPoint);
        workspace
            .add_capture_group(plan_id, group.clone())
            .expect("group is added");
        (workspace, plan_id, group)
    }

    #[test]
    fn capture_groups_survive_a_project_round_trip_unchanged() {
        let (workspace, plan_id, group) = workspace_with_capture_group();
        let payload = workspace.plan_data(plan_id).expect("payload").clone();

        let wire = serde_json::to_string(&payload).expect("serialize");
        let restored: SimulationPlanPayload = serde_json::from_str(&wire).expect("deserialize");

        assert_eq!(restored.capture_groups.len(), 1);
        assert_eq!(restored.capture_groups[0].id, group.id);
        assert_eq!(restored.capture_groups[0].name, "Core rails");
        assert_eq!(
            restored.capture_groups[0].points,
            Some(SavedOutputPolicy::EveryAcceptedPoint)
        );
        assert_eq!(
            restored.capture_groups[0].rules[0]
                .scope
                .as_ref()
                .map(ToString::to_string),
            Some("/x1".to_owned())
        );
        assert_eq!(restored, payload, "the whole payload round-trips");
    }

    /// A project written before capture groups existed carries no such field.
    ///
    /// It must load, and every output it holds must land in the fallback group
    /// with its own contract untouched — which is the whole migration: there is
    /// nothing to rewrite, because "no groups" already means what it should.
    #[test]
    fn a_project_written_before_capture_groups_loads_with_everything_ungrouped() {
        let (workspace, plan_id, _) = workspace_with_capture_group();
        let payload = workspace.plan_data(plan_id).expect("payload").clone();
        let mut wire = serde_json::to_value(&payload).expect("serialize");
        wire.as_object_mut()
            .expect("payload object")
            .remove("capture_groups")
            .expect("the field is present before it is removed");

        let restored: SimulationPlanPayload =
            serde_json::from_value(wire).expect("a payload with no capture groups loads");

        assert!(restored.capture_groups.is_empty());
        let membership = CaptureGroupMembership::resolve(&[], &restored.saved_outputs);
        for (index, output) in restored.saved_outputs.iter().enumerate() {
            assert_eq!(
                membership.owner(index),
                CaptureGroup::ungrouped_id(),
                "{} belongs to the fallback group",
                output.name
            );
            assert_eq!(
                output.save_policy,
                SavedOutputPolicy::SelectedAndFinalPoints,
                "and keeps the contract the project stored"
            );
        }
    }

    #[test]
    fn a_capture_group_name_that_collides_is_refused_and_changes_nothing() {
        let (mut workspace, plan_id, _) = workspace_with_capture_group();
        let before = workspace.clone();

        let collision = CaptureGroup::new("core RAILS").expect("name is well formed on its own");
        let refusal = workspace
            .add_capture_group(plan_id, collision)
            .expect_err("a case-insensitive collision is refused");

        assert!(
            matches!(
                refusal,
                SimulationConfigurationError::CaptureGroup {
                    source: CaptureGroupError::NameConflict { .. },
                    ..
                }
            ),
            "{refusal}"
        );
        assert_eq!(
            workspace.plan_data(plan_id),
            before.plan_data(plan_id),
            "a refused add leaves the plan untouched"
        );
    }

    #[test]
    fn an_empty_or_reserved_capture_group_name_is_refused_before_a_group_exists() {
        assert!(CaptureGroup::new("   ").is_err());
        assert!(CaptureGroup::new("line\nbreak").is_err());
        assert!(
            CaptureGroup::new("Ungrouped").is_err(),
            "the fallback group's name is not available to author"
        );
    }

    #[test]
    fn one_output_cannot_be_named_by_two_groups() {
        let (mut workspace, plan_id, first) = workspace_with_capture_group();
        let output_id = workspace.plan_data(plan_id).expect("payload").saved_outputs[0].id;
        workspace
            .set_capture_group_member(plan_id, first.id, output_id, true)
            .expect("the first group names it");
        let mut second = CaptureGroup::new("Watchlist").expect("group");
        second.members.push(output_id);

        let refusal = workspace
            .add_capture_group(plan_id, second)
            .expect_err("a second explicit claim on one output is refused");

        assert!(
            matches!(
                refusal,
                SimulationConfigurationError::CaptureGroup {
                    source: CaptureGroupError::MemberClaimed { .. },
                    ..
                }
            ),
            "{refusal}"
        );
    }

    #[test]
    fn naming_an_output_moves_it_off_whichever_group_held_it() {
        let (mut workspace, plan_id, core) = workspace_with_capture_group();
        let edge_id = workspace.plan_data(plan_id).expect("payload").saved_outputs[1].id;
        let watchlist = CaptureGroup::new("Watchlist").expect("group");
        let watchlist_id = watchlist.id;
        workspace
            .add_capture_group(plan_id, watchlist)
            .expect("second group");
        workspace
            .set_capture_group_member(plan_id, core.id, edge_id, true)
            .expect("core names the sibling-scope output");

        workspace
            .set_capture_group_member(plan_id, watchlist_id, edge_id, true)
            .expect("the watchlist takes it over");

        let payload = workspace.plan_data(plan_id).expect("payload");
        assert!(
            payload.capture_groups[0].members.is_empty(),
            "the previous holder released it rather than both holding it"
        );
        assert_eq!(payload.capture_groups[1].members, vec![edge_id]);
    }

    #[test]
    fn reordering_groups_changes_which_rule_takes_a_contested_output() {
        let (mut workspace, plan_id, core) = workspace_with_capture_group();
        let mut wide = CaptureGroup::new("Everything").expect("group");
        wide.rules.push(CaptureGroupRule::for_kind(
            SavedOutputKind::RawVoltageOrCurrent,
        ));
        let wide_id = wide.id;
        workspace.add_capture_group(plan_id, wide).expect("added");
        let outputs = workspace
            .plan_data(plan_id)
            .expect("payload")
            .saved_outputs
            .clone();
        let before = CaptureGroupMembership::resolve(
            &workspace
                .plan_data(plan_id)
                .expect("payload")
                .capture_groups,
            &outputs,
        );
        assert_eq!(before.owner(0), core.id, "the earlier group takes it first");

        workspace
            .reorder_capture_group(plan_id, wide_id, true)
            .expect("the wide group is raised");

        let after = CaptureGroupMembership::resolve(
            &workspace
                .plan_data(plan_id)
                .expect("payload")
                .capture_groups,
            &outputs,
        );
        assert_eq!(
            after.owner(0),
            wide_id,
            "raising a group is a policy edit: it now takes the contested output"
        );
    }

    #[test]
    fn cloning_a_plan_rebinds_named_members_onto_the_cloned_outputs() {
        let (mut workspace, plan_id, core) = workspace_with_capture_group();
        let source_output = workspace.plan_data(plan_id).expect("payload").saved_outputs[0].id;
        workspace
            .set_capture_group_member(plan_id, core.id, source_output, true)
            .expect("named member");
        let cloned_plan = SimulationPlanId::new();

        workspace
            .clone_plan_data(plan_id, cloned_plan, true, false, &[])
            .expect("clone");

        let cloned = workspace.plan_data(cloned_plan).expect("cloned payload");
        assert_eq!(cloned.capture_groups.len(), 1);
        assert_ne!(
            cloned.capture_groups[0].id, core.id,
            "the clone owns its own group identity"
        );
        assert_eq!(
            cloned.capture_groups[0].members,
            vec![cloned.saved_outputs[0].id],
            "and its named member is the cloned output, not the source one"
        );
    }
}
