//! The recovery guard's whole-session cases: a restored session, and a
//! project the application saved.

use crate::workbench::app_state::AppState;
use crate::workbench::lifecycle::recovery::recovery_replacement_block_reason;
#[cfg(not(target_arch = "wasm32"))]
use crate::workbench::lifecycle::recovery::unique_temp_dir;

#[test]
fn replacement_guard_compares_initial_plan_draft_to_the_documented_default() {
    let state = AppState::default();
    let mut session = serde_json::to_value(&state).expect("session serializes");
    let execution_context_json = session["execution_context_json"]
        .as_str()
        .expect("current session stores an isolated execution-context envelope");
    let mut execution_context: serde_json::Value = serde_json::from_str(execution_context_json)
        .expect("execution-context envelope is valid JSON");
    execution_context["simulation_plan"]["analysis_plan"]["instances"][0]["draft"]["draft"]["stop"] =
        serde_json::json!("41u");
    session["execution_context_json"] = serde_json::Value::String(
        serde_json::to_string(&execution_context)
            .expect("updated execution-context envelope serializes"),
    );
    let restored: AppState = serde_json::from_value(session).expect("current session restores");
    let plan = restored
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("stable plan restored");
    assert_eq!(plan.revision(), crate::product::ObjectRevision::INITIAL);
    assert!(plan.receipts().is_empty());
    let crate::simulation::plan::AnalysisDraft::Transient(draft) = plan.instances()[0].draft()
    else {
        panic!("default instance remains transient");
    };
    assert_eq!(draft.stop, "41u");

    let reason = recovery_replacement_block_reason(&restored.model)
        .expect("a nondefault initial draft must block destructive replacement");
    assert!(reason.contains("simulation plan or model-library bindings"));
    assert!(reason.contains("pristine unsaved-project defaults"));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn replacement_guard_compares_project_owned_state_to_saved_snapshot() {
    let root = unique_temp_dir("recovery-project-boundary");
    let project_path = root.join("guard.rspiceproj");
    let mut state = AppState::default();
    assert!(
        crate::workbench::workflows::project_workflow::save_project_to_path(
            &mut state,
            &project_path
        )
    );
    assert_eq!(recovery_replacement_block_reason(&state.model), None);

    let transient = state
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("saved stable plan")
        .instances()[0]
        .id();
    state
        .model
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("saved stable plan")
        .edit(transient, |draft| {
            let crate::simulation::plan::AnalysisDraft::Transient(draft) = draft else {
                panic!("default instance is transient");
            };
            draft.stop = "17u".to_owned();
        })
        .expect("transient edit commits");
    let reason = recovery_replacement_block_reason(&state.model)
        .expect("execution state changed after save must block replacement");
    assert!(reason.contains("last saved project snapshot"));
    assert!(reason.contains("simulation plan or model-library bindings"));

    std::fs::remove_dir_all(root).expect("remove fixture");
}
