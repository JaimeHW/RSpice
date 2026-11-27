//! The Models page's binding actions.
//!
//! A binding action commits exactly the order, corner or removal it names,
//! and a refused one leaves the plan and its revision as they were.

use crate::workbench::RSpiceApp;
use rspice_app_simulate::workbench::surfaces::simulate::page_models::{
    ModelBindingAction, apply_model_binding_action,
};

fn active_plan_revision(app: &RSpiceApp) -> rspice_app_state::product::ObjectRevision {
    app.state
        .model
        .sim_setup
        .analysis_plan
        .as_ref()
        .expect("test instance has an active plan")
        .revision()
}

fn add_sectioned_library(app: &mut RSpiceApp, source_name: &str, model: &str) -> String {
    app.state
        .model
        .model_library_manager
        .load_library_bytes(
            source_name,
            format!(
                ".lib TT\n.model {model} NMOS (LEVEL=1 KP=1e-3)\n.endl TT\n\
                 .lib FF\n.model {model} NMOS (LEVEL=1 KP=2e-3)\n.endl FF\n"
            )
            .into_bytes(),
            None,
        )
        .expect("sectioned test library is executable")
}

#[test]
fn model_binding_actions_commit_exact_order_corner_and_removal() {
    let mut app = RSpiceApp::test_instance();
    app.state.model.model_library_manager =
        rspice_app_state::state::model_library::ModelLibraryManager::new();
    let first = add_sectioned_library(&mut app, "models-first.lib", "first_nch");
    let second = add_sectioned_library(&mut app, "models-second.lib", "second_nch");
    let initial_revision = active_plan_revision(&app);
    app.state.runs.preflight.report =
        Some(rspice_app_state::state::PreflightReport::retained_for_test(
            app.state.model.workspace.content.project.revision().get(),
        ));

    apply_model_binding_action(
        &mut crate::workbench::features::cx(&mut app),
        ModelBindingAction::Attach(first.clone()),
    );
    assert_eq!(
        app.state.model.sim_setup.model_bindings[0].library_name,
        first
    );
    assert_ne!(active_plan_revision(&app), initial_revision);
    assert!(app.state.runs.preflight.report.is_none());

    apply_model_binding_action(
        &mut crate::workbench::features::cx(&mut app),
        ModelBindingAction::Attach(second.clone()),
    );
    assert_eq!(
        app.state
            .model
            .sim_setup
            .model_bindings
            .iter()
            .map(|binding| binding.library_name.as_str())
            .collect::<Vec<_>>(),
        [first.as_str(), second.as_str()]
    );

    apply_model_binding_action(
        &mut crate::workbench::features::cx(&mut app),
        ModelBindingAction::MoveUp(1),
    );
    assert_eq!(
        app.state
            .model
            .sim_setup
            .model_bindings
            .iter()
            .map(|binding| binding.library_name.as_str())
            .collect::<Vec<_>>(),
        [second.as_str(), first.as_str()]
    );

    apply_model_binding_action(
        &mut crate::workbench::features::cx(&mut app),
        ModelBindingAction::SetCorner {
            index: 0,
            corner: Some("FF".to_owned()),
        },
    );
    assert_eq!(
        app.state.model.sim_setup.model_bindings[0]
            .selected_corner
            .as_deref(),
        Some("FF")
    );

    apply_model_binding_action(
        &mut crate::workbench::features::cx(&mut app),
        ModelBindingAction::Remove(1),
    );
    assert_eq!(app.state.model.sim_setup.model_bindings.len(), 1);
    assert_eq!(
        app.state.model.sim_setup.model_bindings[0].library_name,
        second
    );
    assert!(!app.state.model.analysis_lifecycle_status.is_refusal());
}

#[test]
fn refused_model_binding_actions_preserve_plan_state_and_revision() {
    let mut app = RSpiceApp::test_instance();
    app.state.model.model_library_manager =
        rspice_app_state::state::model_library::ModelLibraryManager::new();
    let library = add_sectioned_library(&mut app, "models-valid.lib", "valid_nch");
    apply_model_binding_action(
        &mut crate::workbench::features::cx(&mut app),
        ModelBindingAction::Attach(library),
    );
    let before_bindings = app.state.model.sim_setup.model_bindings.clone();
    let before_revision = active_plan_revision(&app);

    apply_model_binding_action(
        &mut crate::workbench::features::cx(&mut app),
        ModelBindingAction::SetCorner {
            index: 0,
            corner: Some("NONEXISTENT".to_owned()),
        },
    );
    assert_eq!(app.state.model.sim_setup.model_bindings, before_bindings);
    assert_eq!(active_plan_revision(&app), before_revision);
    assert!(app.state.model.analysis_lifecycle_status.is_refusal());

    apply_model_binding_action(
        &mut crate::workbench::features::cx(&mut app),
        ModelBindingAction::Attach("missing-library".to_owned()),
    );
    assert_eq!(app.state.model.sim_setup.model_bindings, before_bindings);
    assert_eq!(active_plan_revision(&app), before_revision);
    assert!(app.state.model.analysis_lifecycle_status.is_refusal());
}
