//! The PDK workflow's project transaction observed through a whole
//! application: the configured sources it publishes are undone by the
//! application's project history step.

use crate::state::pdk_config::PdkConfig;
use crate::workbench::app_state::AppState;
use crate::workbench::models::pdk_workflow::{
    apply_pdk_configuration_with_persistence, configured_root,
};
use crate::workbench::models::tests::harness;

#[test]
fn configured_sources_publish_and_unload_as_one_guarded_project_transaction() {
    let alpha_root = configured_root("alpha", "alpha.lib", "alpha_n");
    let beta_root = configured_root("beta", "beta.lib", "beta_n");
    let mut state = AppState::default();
    state.open_project_session();
    state.model.model_library_manager.clear();
    state.model.pdk_config = PdkConfig::default();
    let initial_revision = state.model.workspace.content.project.revision();
    let initial_epoch = state.model.design_execution_epoch;
    let mut config = PdkConfig::default();
    config.add_library_path(alpha_root.to_string_lossy().into_owned());
    config.add_library_path(beta_root.to_string_lossy().into_owned());

    assert_eq!(
        apply_pdk_configuration_with_persistence(
            &mut state.model,
            &state.platform,
            config,
            |_| Ok(())
        )
        .expect("configured sources publish"),
        2
    );
    let first_revision = state.model.workspace.content.project.revision();
    assert!(first_revision > initial_revision);
    assert_eq!(
        state.model.design_execution_epoch,
        initial_epoch.wrapping_add(1)
    );
    assert!(state.model.workspace.content.project_metadata_dirty);
    assert!(
        state
            .model
            .model_library_manager
            .get_library("alpha")
            .is_some()
    );
    assert!(
        state
            .model
            .model_library_manager
            .get_library("beta")
            .is_some()
    );
    assert_eq!(state.model.pdk_config.managed_model_sources.len(), 2);
    assert!(state.model.can_undo_project_design());

    let mut disabled = state.model.pdk_config.clone();
    disabled.toggle_path_enabled(0);
    assert_eq!(
        apply_pdk_configuration_with_persistence(
            &mut state.model,
            &state.platform,
            disabled,
            |_| Ok(())
        )
        .expect("disabled source publishes"),
        1
    );
    assert!(state.model.workspace.content.project.revision() > first_revision);
    assert_eq!(
        state.model.design_execution_epoch,
        initial_epoch.wrapping_add(2)
    );
    assert!(
        state
            .model
            .model_library_manager
            .get_library("alpha")
            .is_none()
    );
    assert!(
        state
            .model
            .model_library_manager
            .get_library("beta")
            .is_some()
    );

    assert_eq!(
        state
            .undo_project_design()
            .expect("undo configured source set"),
        Some("apply configured PDK model sources".to_owned())
    );
    assert!(
        state
            .model
            .model_library_manager
            .get_library("alpha")
            .is_some()
    );
    assert!(
        state
            .model
            .model_library_manager
            .get_library("beta")
            .is_some()
    );
    assert_eq!(
        state.model.design_execution_epoch,
        initial_epoch.wrapping_add(3)
    );

    std::fs::remove_dir_all(alpha_root).expect("remove alpha source root");
    std::fs::remove_dir_all(beta_root).expect("remove beta source root");
}

#[test]
fn startup_service_hydrates_persisted_sources_without_dirtying_a_closed_project() {
    let root = configured_root("startup", "startup.lib", "startup_n");
    let mut state = harness();
    state.model.model_library_manager.clear();
    let revision = state.model.workspace.content.project.revision();
    let epoch = state.model.design_execution_epoch;
    let mut config = PdkConfig::default();
    config.add_library_path(root.to_string_lossy().into_owned());

    state.model.pdk_config = config;
    let (manager, loaded) = crate::workbench::app_state::restore_session_model_library_manager(
        &mut state.model.pdk_config,
    )
    .expect("startup configured source hydration");
    state.model.model_library_manager = manager;
    assert_eq!(loaded, 1);
    assert!(
        state
            .model
            .model_library_manager
            .get_library("startup")
            .is_some()
    );
    assert_eq!(state.model.workspace.content.project.revision(), revision);
    assert_eq!(state.model.design_execution_epoch, epoch);
    assert!(!state.model.workspace.content.project_metadata_dirty);
    assert!(!state.model.can_undo_project_design());

    std::fs::remove_dir_all(root).expect("remove startup source root");
}
