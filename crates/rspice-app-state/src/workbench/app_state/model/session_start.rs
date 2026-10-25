//! The model a new session starts with.

use crate::state::model_library::ModelLibraryManager;
use crate::state::pdk_config::PdkConfig;

use super::super::sim_setup::SimSetupState;
use super::Model;

impl Model {
    /// The model a new session starts with: the bootstrapped project, showing
    /// its active schematic, with the default simulation setup.
    ///
    /// `pdk_config` is the PDK configuration the host read at startup, and
    /// `pdk_load_error` why it could not be read, if it could not. Its
    /// installed technologies are revalidated against its own trust store, and
    /// `restore_catalog` rebuilds the model catalog from it; when that fails,
    /// the session uses the built-in models only. Each of those problems is
    /// recorded in the Console as an error.
    pub fn session_start(
        mut pdk_config: PdkConfig,
        pdk_load_error: Option<String>,
        restore_catalog: impl FnOnce(
            &mut PdkConfig,
        ) -> Result<(ModelLibraryManager, usize), Vec<String>>,
    ) -> Self {
        let mut library_manager = crate::state::LibraryManager::with_primitives();
        let mut workspace = crate::state::ProjectWorkspace::new_bootstrapped(&mut library_manager);
        let schematic = workspace
            .active_schematic()
            .map(|source| source.clone_editor())
            .unwrap_or_default();
        workspace.save_active_schematic(&schematic);
        let sim_setup = SimSetupState::new();
        if let Ok(plan) = sim_setup.stable_analysis_plan() {
            workspace.content.migrate_active_plan_data(plan.id());
        }
        let trust_store = pdk_config.publisher_trust_store.clone();
        let _ = pdk_config
            .technology_registry
            .revalidate_installed(&trust_store);
        let pdk_validation_errors = pdk_config.technology_registry.validation_errors().to_vec();
        let (model_library_manager, pdk_model_restore_errors) =
            match restore_catalog(&mut pdk_config) {
                Ok((manager, _loaded)) => (manager, Vec::new()),
                Err(errors) => (
                    crate::workbench::workspace::default_model_library_manager(),
                    errors,
                ),
            };

        let mut model = Model {
            schematic,
            simulation: crate::state::SimulationState::default(),
            design_execution_epoch: 0,
            active_schematic_epoch: 0,
            project_design_history: Default::default(),
            design_checks: Default::default(),
            diagnostics: Default::default(),
            sim_setup,
            log_buffer: crate::diagnostics::LogBuffer::default(),
            notices: Default::default(),
            netlist_artifacts: Box::default(),
            library_manager,
            library_edit_locks: crate::state::ProjectLibraryLockAuthority::default(),
            workspace,
            project_open: Default::default(),
            pdk_config,
            model_library_manager,
            preferences: Default::default(),
            safe_mode: crate::state::write_policy::LocalSafeModeState::default(),
            live_write_locks: crate::state::write_policy::LiveWriteLocks::default(),
            recorded_sync_failures: Default::default(),
        };
        let startup_errors = pdk_load_error
            .map(|error| {
                format!(
                    "PDK configuration could not be loaded; the session started without a PDK: {error}"
                )
            })
            .into_iter()
            .chain(
                pdk_validation_errors
                    .into_iter()
                    .map(|error| format!("A PDK was not trusted at startup: {error}")),
            )
            .chain(pdk_model_restore_errors.into_iter().map(|error| {
                format!(
                    "Configured PDK model catalog could not be restored; the session is using built-in models only: {error}"
                )
            }));
        for error in startup_errors {
            model.record(
                crate::diagnostics::LogSeverity::Error,
                crate::diagnostics::LogSource::Studio,
                error,
            );
        }
        model
    }
}
