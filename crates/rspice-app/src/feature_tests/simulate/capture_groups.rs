//! Capture groups over the whole application: the Save page lists every
//! group and the fallback, the Outputs page states which group holds the
//! selected output, and the group editor, the capture ledger and the
//! registry's size price each output once, as the application's run
//! controller resolves it.

use super::page_contracts::render_with;
use crate::state::SimulationPage;
use crate::workbench::RSpiceApp;
use crate::workbench::surfaces::simulate::fixtures::plan_id;
use crate::workbench::surfaces::simulate::fixtures::seed_capture_group;
use crate::workbench::surfaces::simulate::page_outputs;
use crate::workbench::surfaces::simulate::page_save;
use crate::workbench::surfaces::simulate::workflows;

/// The group as the plan currently stores it.
fn stored_group(app: &RSpiceApp, id: crate::product::CaptureGroupId) -> crate::state::CaptureGroup {
    app.state
        .model
        .workspace
        .content
        .plan_data(plan_id(&app.state.model))
        .expect("payload")
        .capture_groups
        .iter()
        .find(|group| group.id == id)
        .cloned()
        .expect("the group is still in the plan")
}

/// The editor commits every rule it showed, and shows every rule the group has.
///
/// A group's rules are a disjunction read in authored order. The dialog used to
/// bind only `rules[0]`: it opened on the first rule and rebuilt the group by
/// re-appending `rules[1..]` untouched. Editing the second rule of a group was
/// therefore impossible, and clearing the first deleted it while leaving the
/// rest in force — a membership change nothing on screen described, on the one
/// surface whose whole job is to say what each group holds.
#[test]
fn the_group_editor_edits_every_rule_of_a_disjunction() {
    use crate::state::{CaptureGroupRule, SavedOutputKind};
    use workflows::{commit_capture_group, group_draft};

    let mut app = RSpiceApp::test_instance();
    let id = seed_capture_group(&mut app.state.model);
    let member = app
        .state
        .model
        .workspace
        .content
        .plan_data(plan_id(&app.state.model))
        .expect("payload")
        .saved_outputs[1]
        .id;
    {
        let mut group = stored_group(&app, id);
        group.rules.push(CaptureGroupRule::for_kind(
            SavedOutputKind::NoiseContributor,
        ));
        // A directly named output is a decision made elsewhere; the editor must
        // carry it through untouched however the rules change.
        group.members.push(member);
        app.state
            .model
            .workspace
            .content
            .replace_capture_group(plan_id(&app.state.model), id, group)
            .expect("the plan accepts a second rule");
    }

    let mut draft = group_draft(&stored_group(&app, id));
    assert_eq!(draft.rules.len(), 2, "{draft:?}");
    let scope = draft.rules[0].scope.clone();
    assert!(!scope.is_empty(), "the first rule's scope reaches the form");
    let contributor = SavedOutputKind::ALL
        .iter()
        .position(|kind| *kind == SavedOutputKind::NoiseContributor)
        .expect("NoiseContributor occupies a kind slot");
    assert_eq!(draft.rules[1].kind, Some(contributor), "{draft:?}");

    // Edit the *second* rule, which the old editor could not reach at all, and
    // append a third.
    let expression = SavedOutputKind::ALL
        .iter()
        .position(|kind| *kind == SavedOutputKind::DerivedExpression)
        .expect("DerivedExpression occupies a kind slot");
    draft.rules[1].kind = Some(expression);
    draft
        .rules
        .push(crate::workbench::state::CaptureGroupRuleDraft {
            scope: "/x2".to_owned(),
            kind: None,
        });
    commit_capture_group(&mut crate::workbench::features::cx(&mut app), &draft)
        .expect("the edited group commits");

    let committed = stored_group(&app, id);
    assert_eq!(committed.rules.len(), 3, "{committed:?}");
    assert_eq!(
        committed.rules[0].scope.as_ref().map(ToString::to_string),
        Some(scope)
    );
    assert_eq!(
        committed.rules[1].kind,
        Some(SavedOutputKind::DerivedExpression)
    );
    assert!(committed.rules[2].scope.is_some(), "{committed:?}");
    assert_eq!(
        committed.members,
        vec![member],
        "a directly named output is not a rule and survives every rule edit"
    );

    // Reordering is a policy edit — the group's rules are read in this order —
    // and removing one removes exactly one.
    let mut draft = group_draft(&committed);
    draft.rules.swap(0, 1);
    draft.rules.remove(2);
    commit_capture_group(&mut crate::workbench::features::cx(&mut app), &draft)
        .expect("the reordered group commits");

    let reordered = stored_group(&app, id);
    assert_eq!(reordered.rules.len(), 2, "{reordered:?}");
    assert_eq!(
        reordered.rules[0].kind,
        Some(SavedOutputKind::DerivedExpression)
    );
    assert_eq!(
        reordered.rules[1].scope.as_ref().map(ToString::to_string),
        committed.rules[0].scope.as_ref().map(ToString::to_string)
    );
}

/// The Save page's own ledger, built exactly the way the page builds it.
fn page_capture_ledger(app: &RSpiceApp) -> rspice_simulation::capture_ledger::CaptureLedger {
    let payload = app
        .state
        .model
        .workspace
        .content
        .plan_data(plan_id(&app.state.model))
        .cloned()
        .expect("payload");
    let (outputs, reports, _, membership) = app
        .simulation_controller
        .effective_saved_outputs_preflight(
            &app.state.model,
            &payload.saved_outputs,
            &payload.capture_groups,
        )
        .expect("the test design resolves an output set");
    rspice_simulation::capture_ledger::CaptureLedger::resolve(
        &payload.capture_groups,
        &outputs,
        &reports,
        &membership,
        app.state.model.sim_setup.save_policy.output_selection_mode,
        &page_save::capture_workload(&app.state.model).0,
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
    seed_capture_group(&mut app.state.model);
    let payload = app
        .state
        .model
        .workspace
        .content
        .plan_data(plan_id(&app.state.model))
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
    seed_capture_group(&mut app.state.model);
    let payload = app
        .state
        .model
        .workspace
        .content
        .plan_data(plan_id(&app.state.model))
        .cloned()
        .expect("payload");

    let (outputs, _, _, _) = app
        .simulation_controller
        .effective_saved_outputs_preflight(
            &app.state.model,
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

/// The Save page draws the groups it prices, including the one nobody authored.
#[test]
fn the_save_page_lists_every_capture_group_and_the_fallback() {
    let rendered = render_with(SimulationPage::Save, 1400.0, |app| {
        seed_capture_group(&mut app.state.model);
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
        seed_capture_group(&mut app.state.model);
        app.state.simulate.selected_saved_output = Some("core_n".to_owned());
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

/// The Outputs registry's `Est. size` cell and the Save ledger's row are one
/// number.
///
/// They were two. The registry multiplied the *authored* record's estimate by
/// the declared point count; the ledger priced the *group-projected* output at
/// each producing analysis's own participation. So a plan whose group overrode
/// a member's precision, or whose analysis ran nominal-only, read one size on
/// the page that lists its outputs and another on the page that totals them —
/// under a comment on the first claiming "nothing here is a second forecast".
///
/// Both halves are exercised: the group in this fixture rewrites its member's
/// point policy, and the run set narrows one analysis to the nominal point.
#[test]
fn the_registry_size_and_the_ledger_row_price_one_output_once() {
    use crate::simulation::plan::{AnalysisInstance, AnalysisKind};
    use rspice_simulation_contract::analysis_run_at::AnalysisRunAt;
    use rspice_simulation_contract::run_set::RunSetDimensionKind;

    let mut app = RSpiceApp::test_instance();
    seed_capture_group(&mut app.state.model);

    // A declared space with more than one point, and one analysis narrowed to
    // the nominal one — the case a scalar point count prices wrongly.
    for dimension in &mut app.state.model.sim_setup.run_set.dimensions {
        dimension.enabled = dimension.kind == RunSetDimensionKind::Temperature;
    }
    let nominal_only = app
        .state
        .model
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .and_then(|plan| {
            plan.instances()
                .iter()
                .find(|instance| instance.kind() == AnalysisKind::Transient)
                .map(AnalysisInstance::id)
        })
        .expect("the test plan runs a transient");
    if let Some(plan) = app.state.model.sim_setup.analysis_plan.as_mut() {
        let _ = plan.set_run_at(nominal_only, AnalysisRunAt::NominalPoint);
    }
    assert!(
        app.state.model.sim_setup.run_set.point_count() > 1,
        "the space has to multiply, or the two arithmetics cannot disagree"
    );

    let payload = app
        .state
        .model
        .workspace
        .content
        .plan_data(plan_id(&app.state.model))
        .cloned()
        .expect("payload");
    let ledger = page_capture_ledger(&app);
    let registry = page_outputs::projected_output_bytes_for(
        &app.state.model,
        &app.simulation_controller,
        &payload,
        &page_save::capture_workload(&app.state.model).0,
    );
    assert!(
        !registry.is_empty(),
        "the registry has to be pricing something for this to mean anything"
    );

    // The two arithmetics have to actually differ here, or this pins nothing:
    // the old cell was the authored record's own estimate times the declared
    // point count, and at least one output must not price that way.
    let matrix =
        u64::try_from(app.state.model.sim_setup.run_set.point_count().max(1)).unwrap_or(u64::MAX);
    let flat: Vec<u64> = app
        .simulation_controller
        .saved_outputs_preflight(&app.state.model, &payload.saved_outputs)
        .iter()
        .filter_map(|report| match report.storage_estimate() {
            rspice_simulation::output_contract::SavedOutputStorageEstimate::ExactBytes(bytes) => {
                Some(bytes.saturating_mul(matrix))
            }
            rspice_simulation::output_contract::SavedOutputStorageEstimate::Indeterminate {
                ..
            } => None,
        })
        .collect();
    let projected: Vec<u64> = payload
        .saved_outputs
        .iter()
        .filter_map(|output| registry.get(&output.id).copied().flatten())
        .collect();
    assert_ne!(
        flat, projected,
        "the fixture must be one the old per-record arithmetic gets wrong"
    );

    // Every group row is the sum of the registry cells of the outputs it owns,
    // so no output is priced twice and none is priced two ways.
    let membership = crate::state::CaptureGroupMembership::resolve(
        &payload.capture_groups,
        &payload.saved_outputs,
    );
    for row in ledger.rows() {
        let from_registry: u64 = payload
            .saved_outputs
            .iter()
            .enumerate()
            .filter(|(index, _)| membership.owner(*index) == row.group)
            .filter_map(|(_, output)| registry.get(&output.id).copied().flatten())
            .sum();
        assert_eq!(
            row.bytes, from_registry,
            "the group '{}' totals {} bytes while its rows' cells add to {from_registry}",
            row.name, row.bytes
        );
    }
}
