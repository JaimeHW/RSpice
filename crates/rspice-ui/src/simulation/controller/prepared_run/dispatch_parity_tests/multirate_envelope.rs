//! Saved multirate-envelope plans must keep their solver selection through authorized dispatch.

use super::*;
use crate::simulation::plan::AnalysisDraft;

#[test]
fn multirate_envelope_saved_plan_retains_its_solver_through_authorized_dispatch() {
    let mut state = preflight_ready_state();
    let id = only(&mut state, &[AnalysisKind::Envelope])[0];
    plan_mut(&mut state)
        .edit(id, |draft| {
            let AnalysisDraft::Envelope(draft) = draft else {
                unreachable!()
            };
            draft.multirate_enabled = true;
            draft.modulation_sources = "VCC".into();
            draft.multirate.relative_tolerance = "0.002".into();
            draft.multirate.maximum_step = "100u".into();
            draft.multirate.event_charge = "2e-15".into();
        })
        .unwrap();
    let saved = serde_json::to_vec(plan_mut(&mut state)).unwrap();
    state.sim_setup.analysis_plan = Some(serde_json::from_slice(&saved).unwrap());
    plan_mut(&mut state).prepare_after_restore();
    let mut controller = SimulationController::new();
    let snapshot = controller
        .build_prepared_snapshot(&state, SimulationRunIntent::SimulateRunSet)
        .unwrap();
    let receipt = snapshot.prepared_run_receipt().unwrap();
    controller.authorize_snapshot(snapshot).unwrap();
    let dispatch = controller
        .consume_snapshot_for_dispatch(&mut state)
        .unwrap();
    assert_eq!(
        receipt,
        dispatch
            .prepared_run_receipt(crate::state::AnalysisResultSourceDomain::SimulationPlan)
            .unwrap()
    );
    let tasks = dispatch.into_tasks();
    let task = tasks.iter().find(|task| task.instance_id() == id).unwrap();
    let AnalysisSpec::Envelope {
        multirate: Some(settings),
        ..
    } = task.spec()
    else {
        panic!("lost multirate solver")
    };
    assert_eq!(settings.relative_tolerance, 0.002);
    assert_eq!(settings.maximum_step, Some(0.0001));
    assert_eq!(settings.events.charge_coulombs, 2e-15);
}
