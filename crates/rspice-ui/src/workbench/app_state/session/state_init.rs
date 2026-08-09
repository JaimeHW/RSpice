//! Building the default session.
//!
//! The initial [`AppState`], and the default model library and analysis
//! viewers behind it.

use crate::workbench::app::DialogState;
use crate::workbench::app_state::{AnalysisWorkspaceState, AppState};

pub(crate) fn default_model_library_manager() -> crate::state::model_library::ModelLibraryManager {
    let mut manager = crate::state::model_library::ModelLibraryManager::new();
    manager.load_builtin_models();
    // The packs ship beside the binary; absence is normal (browser build).
    manager.discover_spice_packs();
    manager
}

pub(in crate::workbench) fn default_analysis_viewers() -> AnalysisWorkspaceState {
    AnalysisWorkspaceState {
        pole_zero_state: crate::analysis::pole_zero::PoleZeroState::default(),
        bode_plot_state: crate::analysis::bode::BodePlotState::default(),
        nyquist_state: crate::analysis::nyquist::NyquistState::default(),
        eye_diagram_state: crate::analysis::eye_diagram::EyeDiagramState::default(),
        fft_state: crate::analysis::fft::FftState::default(),
        smith_chart_state: crate::analysis::smith_chart::SmithChartState::default(),
        histogram_state: crate::analysis::histogram::HistogramState::default(),
        cache_authority: Default::default(),
    }
}

pub(in crate::workbench) fn default_app_state() -> AppState {
    let analysis = default_analysis_viewers();
    let mut library_manager = crate::state::LibraryManager::with_primitives();
    let mut workspace = crate::state::ProjectWorkspace::new_bootstrapped(&mut library_manager);
    let schematic = workspace
        .active_schematic()
        .cloned()
        .unwrap_or_else(crate::state::SchematicState::default);
    workspace.save_active_schematic(&schematic);
    let sim_setup = crate::workbench::app_state::SimSetupState::new();
    if let Ok(plan) = sim_setup.stable_analysis_plan() {
        workspace.migrate_active_plan_data(plan.id());
    }
    let mut ui = crate::workbench::UiSessionState::new();
    ui.schematic_snap.grid_size = schematic.document_policy.grid_pitch.canvas_grid_size();
    if let Some(bundle) = workspace.project_sources.bundle_for_owner(
        &crate::state::ProjectSourceOwner::code_workspace(
            crate::state::ProjectSourceLanguage::VerilogA,
        ),
    ) {
        ui.code_workspace.veriloga.receipt = Some(
            crate::workbench::documents::code_workspace::compile_project_bundle_receipt(
                workspace.project.id(),
                bundle,
                None,
            )
            .expect("the canonical bootstrapped Verilog-A source must compile"),
        );
    }
    if let Some(bundle) = workspace.project_sources.bundle_for_owner(
        &crate::state::ProjectSourceOwner::code_workspace(
            crate::state::ProjectSourceLanguage::RSpiceAutomation,
        ),
    ) {
        let mut documents = vec![crate::automation_workflow::AutomationSourceDocument {
            path: bundle.root().logical_path(),
            source: bundle.root().content(),
        }];
        documents.extend(bundle.files().iter().map(|file| {
            crate::automation_workflow::AutomationSourceDocument {
                path: file.logical_path(),
                source: file.content(),
            }
        }));
        let roles = bundle
            .roles()
            .iter()
            .filter_map(|binding| {
                let role = match binding.role() {
                    crate::state::ProjectSourceRole::VerilogABuildProfile
                    | crate::state::ProjectSourceRole::AutomationEntry => return None,
                    crate::state::ProjectSourceRole::AutomationRunPlan => {
                        crate::automation_workflow::AutomationSourceRole::RunPlan
                    }
                    crate::state::ProjectSourceRole::AutomationEnvironmentLock => {
                        crate::automation_workflow::AutomationSourceRole::EnvironmentLock
                    }
                    crate::state::ProjectSourceRole::AutomationPermissionManifest => {
                        crate::automation_workflow::AutomationSourceRole::PermissionManifest
                    }
                };
                Some(crate::automation_workflow::AutomationRoleBinding {
                    path: binding.logical_path(),
                    role,
                })
            })
            .collect::<Vec<_>>();
        let (_plan, manifest) = crate::automation_workflow::compile_automation_documents(
            bundle.root().logical_path(),
            &documents,
            &roles,
        )
        .expect("the canonical bootstrapped Automation workspace must compile");
        let runtime_snapshot = crate::automation_workflow::build_automation_runtime_snapshot(
            workspace.project.id(),
            bundle,
            &manifest,
            Vec::new(),
        )
        .expect("the canonical bootstrapped Automation worker snapshot must be valid");
        ui.code_workspace.automation.manifest = Some(manifest);
        ui.code_workspace.automation.runtime_snapshot = Some(runtime_snapshot);
        // Static compilation prepares editor metadata and the immutable worker
        // snapshot, but it cannot authorize Python execution. The managed
        // runtime creates the validation receipt after authoritative CPython
        // compilation.
    }
    #[cfg(all(not(test), not(target_arch = "wasm32")))]
    let (mut pdk_config, pdk_load_error) = match crate::state::pdk_config::PdkConfig::load() {
        Ok(config) => (config, None),
        Err(error) => (
            crate::state::pdk_config::PdkConfig::default(),
            Some(error.to_string()),
        ),
    };
    // Browser PDK state is hydrated asynchronously from IndexedDB by the
    // application persistence owner. Until that exact read completes, no
    // persisted trust or package authority is present in the live state.
    #[cfg(all(not(test), target_arch = "wasm32"))]
    let (mut pdk_config, pdk_load_error) = (
        crate::state::pdk_config::PdkConfig::default(),
        None::<String>,
    );
    // Unit tests must never inherit user- or machine-owned trust state. Apart
    // from making tests nondeterministic, doing so could grant a fixture
    // authority it did not explicitly provision.
    #[cfg(test)]
    let (mut pdk_config, pdk_load_error) = (
        crate::state::pdk_config::PdkConfig::default(),
        None::<String>,
    );
    let trust_store = pdk_config.publisher_trust_store.clone();
    let _ = pdk_config
        .technology_registry
        .revalidate_installed(&trust_store);
    let pdk_validation_errors = pdk_config.technology_registry.validation_errors().to_vec();

    let mut state = AppState {
        schematic,
        simulation: crate::state::SimulationState::default(),
        design_execution_epoch: 0,
        active_schematic_epoch: 0,
        project_design_history: Default::default(),
        design_checks: Default::default(),
        dialogs: DialogState::default(),
        sim_setup,
        log_buffer: crate::diagnostics::LogBuffer::default(),
        script_console: super::script_console::ScriptConsoleState::default(),
        library_manager,
        library_edit_locks: crate::state::ProjectLibraryLockAuthority::default(),
        workspace,
        project_lifecycle: crate::workbench::lifecycle::project_lifecycle::ProjectLifecycleState::default(),
        pending_delete_cell: None,
        pending_delete_view: None,
        tabbed_property_dialog: crate::properties::TabbedPropertyDialogState::default(),
        property_registry: crate::state::PropertyRegistry::new(),
        calculator_panel: super::calculator::CalculatorPanel::new(),
        pdk_settings_dialog: super::pdk_settings::PdkSettingsDialogState::new(),
        pdk_config,
        model_library_manager: default_model_library_manager(),
        model_browser_state: crate::properties::model_browser::ModelBrowserState::default(),
        exit_requested: false,
        recent_files: Vec::new(),
        browser_schematic_save_name: None,
        browser_project_save_name: None,
        native_project_binding_receipt: None,
        browser_project_binding_receipt: None,
        license_key: None,
        license: None,
        analysis,
        ui,
        workbench: crate::workbench::WorkbenchState::default(),
        shortcut_resolver: crate::workbench::app_state::session::shortcuts::ShortcutResolverState::default(),
        shortcut_library_persistence:
            crate::workbench::app_state::session::shortcut_library::ShortcutLibraryPersistenceRuntime::default(),
        shortcut_library_publication_continuation: None,
    };
    if let Some(error) = pdk_load_error {
        state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
            "PDK configuration could not be loaded; the session started with no configured PDK authority: {error}"
        )));
    }
    for error in pdk_validation_errors {
        state.push_user_message(crate::diagnostics::ConsoleMessage::error(format!(
            "PDK technology trust blocked during startup: {error}"
        )));
    }
    state
}

#[cfg(test)]
mod tests {
    use crate::state::ProjectSourceLanguage;

    #[test]
    fn bootstrapped_code_sources_prepare_an_exact_snapshot_without_execution_authority() {
        let state = super::default_app_state();
        let automation = state
            .workspace
            .project_sources
            .bundle_for_owner(&crate::state::ProjectSourceOwner::code_workspace(
                ProjectSourceLanguage::RSpiceAutomation,
            ))
            .expect("bootstrapped Automation workspace");
        let snapshot = state
            .ui
            .code_workspace
            .automation
            .runtime_snapshot
            .as_ref()
            .expect("bootstrapped Automation runtime snapshot");

        assert_eq!(snapshot.project_id, state.workspace.project.id().as_uuid());
        assert_eq!(snapshot.workspace_revision, automation.revision().get());
        assert_eq!(
            snapshot.closure_digest.0,
            *automation.closure_digest().as_bytes()
        );
        assert!(
            state.ui.code_workspace.automation.receipt.is_none(),
            "static startup parsing must not impersonate managed-CPython validation"
        );
        assert!(state.ui.code_workspace.automation.manifest.is_some());
        assert!(state.ui.code_workspace.veriloga.receipt.is_some());
    }
}
