//! A campaign member saved with the project it ran in.

use super::session_of;
use crate::simulation::controller::SimulationController;
use crate::simulation::controller::prepared_run::fixtures::two_plan_campaign_state;
use crate::workbench::lifecycle::project_lifecycle;

#[test]
fn campaign_freezes_distinct_plan_members_without_switching_the_live_editor() {
    let (state, first_plan_id, second_plan_id) = two_plan_campaign_state();
    let mut state = session_of(state);
    let live_plan_before_dispatch = state.model.sim_setup.stable_analysis_plan().unwrap().id();
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
        state.model.sim_setup.stable_analysis_plan().unwrap().id(),
        live_plan_before_dispatch
    );
    let run = state
        .model
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
