//! What a saved test holds of its plan: the analyses as they stand, and no
//! record of the edits that made them. The Console logs each edit as it is
//! made; the project file keeps only the result.

use crate::workbench::RSpiceApp;
use rspice_app_services::workbench::lifecycle::recovery::unique_temp_dir;
use rspice_app_state::simulation::plan::AnalysisDraft;

/// The stop time of the plan's transient set to `stop`, as a committed edit.
fn set_stop(app: &mut RSpiceApp, stop: &str) {
    let plan = app
        .state
        .model
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("a stable plan");
    let transient = plan
        .instances()
        .iter()
        .find(|instance| matches!(instance.draft(), AnalysisDraft::Transient(_)))
        .expect("the plan holds a transient")
        .id();
    plan.edit(transient, |draft| {
        if let AnalysisDraft::Transient(setup) = draft {
            stop.clone_into(&mut setup.stop);
        }
    })
    .expect("the transient takes its stop time");
}

#[test]
fn a_saved_test_holds_no_receipt_list() {
    let root = unique_temp_dir("saved-plan-record");
    let project = root.join("record.rspiceproj");
    let mut app = RSpiceApp::test_instance();
    for stop in ["1u", "2u", "3u"] {
        set_stop(&mut app, stop);
    }
    app.state
        .model
        .sim_setup
        .commit_active_plan_configuration_change("Saved outputs were updated.")
        .expect("a configuration change commits");
    assert!(
        app.state.save_project_to_path(&project),
        "the project saves"
    );

    let stored: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&project).expect("the project file"))
            .expect("the project file is JSON");
    let plan = &stored["execution_context"]["simulation_plan"]["analysis_plan"];
    let fields: Vec<&str> = plan
        .as_object()
        .expect("the saved plan is an object")
        .keys()
        .map(String::as_str)
        .collect();
    assert!(
        fields.contains(&"instances") && fields.contains(&"revision"),
        "the plan is saved: {fields:?}"
    );
    for record in [
        "receipts",
        "configuration_receipts",
        "next_receipt_sequence",
    ] {
        assert!(
            !fields.contains(&record),
            "the saved plan holds no {record}: {fields:?}"
        );
    }
    let _ = std::fs::remove_dir_all(root);
}
