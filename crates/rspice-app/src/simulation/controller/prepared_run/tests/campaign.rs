//! Campaigns: plans frozen together and run one after another.
//!
//! A campaign freezes every member before any of them runs, files a member
//! that cannot start where its failures would have been filed, and hands the
//! shell the outcomes no frame can watch: a member that never became a run,
//! and the campaign's own end.

use super::*;

/// A runnable state holding two plans a campaign can freeze. The second,
/// created last, is the active one.
fn two_plan_campaign_state() -> (
    AppState,
    crate::product::SimulationPlanId,
    crate::product::SimulationPlanId,
) {
    let mut state = runnable_state();
    let first_plan_id = state.sim_setup.stable_analysis_plan().unwrap().id();
    state
        .workspace
        .content
        .migrate_active_plan_data(first_plan_id);
    let second_plan_id = state
        .sim_setup
        .create_plan("Second campaign plan")
        .expect("create campaign plan");
    state
        .workspace
        .content
        .migrate_inactive_plan_data(second_plan_id);
    state
        .workspace
        .content
        .sync_legacy_specs_projection(second_plan_id);
    (state, first_plan_id, second_plan_id)
}

#[test]
fn campaign_freezes_distinct_plan_members_without_switching_the_live_editor() {
    let (mut state, first_plan_id, second_plan_id) = two_plan_campaign_state();
    let live_plan_before_dispatch = state.sim_setup.stable_analysis_plan().unwrap().id();
    let mut controller = SimulationController::new();

    let receipt = controller
        .prepare_and_start_campaign(
            &mut state,
            "Nightly characterization",
            &[first_plan_id, second_plan_id],
        )
        .expect("freeze and dispatch campaign");

    assert_eq!(receipt.member_count, 2);
    assert_eq!(
        state.sim_setup.stable_analysis_plan().unwrap().id(),
        live_plan_before_dispatch
    );
    let run = state
        .simulation
        .active_run()
        .expect("first member dispatched");
    assert_eq!(
        run.prepared_receipt().unwrap().simulation_plan_id(),
        Some(first_plan_id)
    );
    let membership = run
        .campaign_membership()
        .expect("campaign identity retained");
    assert_eq!(membership.campaign_id(), receipt.campaign_id);
    assert_eq!(membership.member_index(), 1);
    assert_eq!(membership.member_count(), 2);
    assert_eq!(
        controller
            .active_campaign
            .as_ref()
            .expect("campaign remains active")
            .pending
            .len(),
        1
    );

    let project = project_lifecycle::snapshot(&state).expect("campaign member project snapshot");
    let json = crate::io::project_io::serialize_project_file(&project)
        .expect("campaign member serializes");
    let loaded =
        crate::io::project_io::load_project_text(&json, None).expect("campaign member reloads");
    let restored = crate::io::simulation_state_from_results(loaded.file.simulation_results)
        .expect("campaign result history restores");
    let restored_membership = restored.retained.runs[0]
        .campaign_membership()
        .expect("campaign identity survives project round trip");
    assert_eq!(restored_membership.campaign_id(), receipt.campaign_id);
    assert_eq!(restored_membership.member_index(), 1);

    controller.abort();
}

/// A campaign member that cannot be started is filed as a finding about
/// that member, where its failures would have been filed had it run, and the
/// campaign run again supersedes it. It used to reach only the Console, so
/// a campaign none of whose members ran left the Problems page empty.
#[test]
fn a_campaign_member_that_cannot_start_is_filed_until_a_rerun_supersedes_it() {
    let (mut state, first_plan_id, second_plan_id) = two_plan_campaign_state();
    let next_run_id = state.simulation.retained.next_run_id;
    state.simulation.retained.next_run_id = u64::MAX;
    let mut controller = SimulationController::new();

    controller
        .prepare_and_start_campaign(
            &mut state,
            "Nightly characterization",
            &[first_plan_id, second_plan_id],
        )
        .expect("the campaign freezes; its members are refused one by one");

    assert!(state.simulation.active_run().is_none());
    let refused = state.diagnostics.published_member_refusals();
    assert_eq!(refused.len(), 2, "{refused:?}");
    for (index, (scope, subject, message)) in refused.iter().enumerate() {
        let member = index + 1;
        assert_eq!(*scope, format!("simulation-run/member-{member}"));
        assert!(
            subject.ends_with(&format!(" · campaign member {member}")),
            "{subject}"
        );
        assert!(
            message.starts_with("The member could not be started: ")
                && message.contains("run sequence is exhausted"),
            "{message}"
        );
    }
    assert!(
        refused[1].1.starts_with("Second campaign plan"),
        "the member is named by the plan it was to run: {}",
        refused[1].1
    );

    state.simulation.retained.next_run_id = next_run_id;
    controller
        .prepare_and_start_campaign(
            &mut state,
            "Nightly characterization",
            &[first_plan_id, second_plan_id],
        )
        .expect("the campaign runs again");

    assert!(state.simulation.active_run().is_some());
    assert!(
        state.diagnostics.published_member_refusals().is_empty(),
        "the campaign run again is a new measurement"
    );

    controller.abort();
}

/// A member that never became a run is reported to the shell as a member
/// that failed, and the campaign's end as one summary that counts each
/// member once. The shell cannot watch either reach an end, so the
/// controller hands it both, each with the Console line that says it.
#[test]
fn a_campaign_reports_its_refused_members_and_its_end_to_the_shell() {
    let (mut state, first_plan_id, second_plan_id) = two_plan_campaign_state();
    let first_plan = state
        .sim_setup
        .inactive_plans()
        .iter()
        .find(|plan| plan.id() == first_plan_id)
        .expect("the first plan is stored")
        .name()
        .to_string();
    state.simulation.retained.next_run_id = u64::MAX;
    let mut controller = SimulationController::new();

    controller
        .prepare_and_start_campaign(
            &mut state,
            "Nightly characterization",
            &[first_plan_id, second_plan_id],
        )
        .expect("the campaign freezes; its members are refused one by one");

    let outcomes = state.ui.run_watch.take_campaign_outcomes();
    assert_eq!(outcomes.len(), 3, "{outcomes:?}");
    let console_line = |log_id: u64| {
        state
            .log_buffer
            .entries()
            .find(|entry| entry.id == log_id)
            .map(|entry| entry.message.clone())
            .expect("the outcome's Console line")
    };
    for (outcome, expected_plan) in outcomes
        .iter()
        .zip([first_plan.as_str(), "Second campaign plan"])
    {
        let CampaignOutcome::MemberRefused {
            campaign,
            plan,
            reason,
            log_id,
        } = outcome
        else {
            panic!("a refused member first: {outcome:?}");
        };
        assert_eq!(campaign, "Nightly characterization");
        assert_eq!(plan, expected_plan);
        assert!(
            reason.starts_with("It could not be started: ")
                && reason.contains("run sequence is exhausted"),
            "{reason}"
        );
        let line = console_line(*log_id);
        assert!(
            line.starts_with(&format!("Campaign member '{expected_plan}' could not")),
            "{line}"
        );
    }
    let CampaignOutcome::Finished {
        name,
        completed,
        failed,
        log_id,
        ..
    } = &outcomes[2]
    else {
        panic!("the campaign's end last: {:?}", outcomes[2]);
    };
    assert_eq!(
        (name.as_str(), *completed, *failed),
        ("Nightly characterization", 0, 2),
        "a failed member is not also a completed one"
    );
    assert_eq!(
        console_line(*log_id),
        "Campaign 'Nightly characterization' completed with errors: 0 completed, 2 failed"
    );
}

/// A campaign the reader cancelled is not news to them. Its summary is in
/// the Console, and the shell is handed nothing to announce.
#[test]
fn a_cancelled_campaign_reports_nothing_to_the_shell() {
    let (mut state, first_plan_id, second_plan_id) = two_plan_campaign_state();
    let mut controller = SimulationController::new();
    controller
        .prepare_and_start_campaign(
            &mut state,
            "Nightly characterization",
            &[first_plan_id, second_plan_id],
        )
        .expect("freeze and dispatch campaign");

    // What a Stop does to a campaign: nothing after the member running, and
    // that member ends without succeeding.
    let campaign = controller
        .active_campaign
        .as_mut()
        .expect("campaign is active");
    campaign.pending.clear();
    campaign.cancelled = true;
    controller.complete_campaign_member(&mut state, false);

    assert!(
        state.log_buffer.entries().any(|entry| entry.message
            == "Campaign 'Nightly characterization' cancelled: 0 completed, 1 failed"),
        "the Console still sums it up"
    );
    assert!(state.ui.run_watch.take_campaign_outcomes().is_empty());

    controller.abort();
}
