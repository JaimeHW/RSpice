//! Building the default session.
//!
//! The initial [`AppState`], and the default model library and analysis
//! viewers behind it; and the state a launch starts from.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::session::preferences_file::PreferencesStore;
use crate::workbench::app_state::viewer_capabilities::AnalysisWorkspaceState;
pub(crate) use rspice_app_state::workbench::workspace::default_model_library_manager;
use rspice_app_state::workbench::workspace::restore_model_library_manager;

impl AppState {
    /// The state a launch starts from: the stored session, with the
    /// preferences file read over it.
    pub(in crate::workbench) fn restore_with_preferences_file(
        storage: Option<&dyn eframe::Storage>,
    ) -> Self {
        Self::restore_with_store(storage, PreferencesStore::production())
    }

    pub fn restore_with_store(
        storage: Option<&dyn eframe::Storage>,
        store: Option<PreferencesStore>,
    ) -> Self {
        let mut state = storage
            .and_then(Self::restore_eframe_session)
            .unwrap_or_default();
        match store {
            Some(store) => state.preferences_file_parts().bind_preferences_file(store),
            None => {
                state.record(
                    LogSeverity::Warning,
                    LogSource::Studio,
                    "This system reports no folder for user settings, so preferences are kept \
                     with the session only.",
                );
            }
        }
        state
    }
}

/// Rebuild the application-scoped model catalog from durable PDK settings.
///
/// Project-owned libraries never participate in this reconstruction. The
/// caller receives a completely staged catalog, and both the live
/// configuration and catalog remain unchanged unless discovery, parsing, and
/// native ownership persistence all succeed.
pub(crate) fn restore_session_model_library_manager(
    config: &mut crate::state::pdk_config::PdkConfig,
) -> Result<(crate::state::model_library::ModelLibraryManager, usize), Vec<String>> {
    let mut next = config.clone();
    let restored = restore_model_library_manager(&mut next)?;

    #[cfg(all(not(test), not(target_arch = "wasm32")))]
    if next.managed_model_sources != config.managed_model_sources {
        crate::state::pdk_config::save(&next).map_err(|error| {
            vec![format!(
                "Configured PDK sources were validated but their ownership could not be persisted: {error}"
            )]
        })?;
    }

    *config = next;
    Ok(restored)
}

pub(in crate::workbench) fn default_analysis_viewers() -> AnalysisWorkspaceState {
    AnalysisWorkspaceState {
        bode_plot_state: rspice_results_ui::bode::BodePlotState::default(),
        nyquist_state: rspice_results_ui::nyquist::NyquistState::default(),
        eye_diagram_state: rspice_results_ui::eye_diagram::EyeDiagramState::default(),
        fft_state: rspice_results_ui::fft::FftState::default(),
        smith_chart_state: rspice_results_ui::smith_chart::SmithChartState::default(),
        histogram_state: rspice_results_ui::histogram::HistogramState::default(),
        cache_authority: Default::default(),
    }
}

pub(in crate::workbench) fn default_app_state() -> AppState {
    let analysis = default_analysis_viewers();
    #[cfg(all(not(test), not(target_arch = "wasm32")))]
    let (pdk_config, pdk_load_error) = match crate::state::pdk_config::load() {
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
    let (pdk_config, pdk_load_error) = (
        crate::state::pdk_config::PdkConfig::default(),
        None::<String>,
    );
    // Unit tests must never inherit user- or machine-owned trust state. Apart
    // from making tests nondeterministic, doing so could grant a fixture
    // authority it did not explicitly provision.
    #[cfg(test)]
    let (pdk_config, pdk_load_error) = (
        crate::state::pdk_config::PdkConfig::default(),
        None::<String>,
    );
    let model = crate::workbench::app_state::Model::session_start(
        pdk_config,
        pdk_load_error,
        restore_session_model_library_manager,
    );
    let workspace = &model.workspace;
    let snap_grid_size = model
        .schematic
        .document()
        .document_policy
        .grid_pitch
        .canvas_grid_size();
    let mut automation = None;
    if let Some(bundle) = workspace.content.project_sources.bundle_for_owner(
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
            workspace.content.project.id(),
            bundle,
            &manifest,
            Vec::new(),
        )
        .expect("the canonical bootstrapped Automation worker snapshot must be valid");
        automation = Some((manifest, runtime_snapshot));
        // Static compilation prepares editor metadata and the immutable worker
        // snapshot, but it cannot authorize Python execution. The managed
        // runtime creates the validation receipt after authoritative CPython
        // compilation.
    }

    let mut state = AppState {
        model,
        problems_view: crate::workbench::app_state::problems_view::ProblemsView::default(),
        shell: Default::default(),
        platform: Box::default(),
        design: Box::default(),
        code: Box::default(),
        simulate: Box::default(),
        project: Box::default(),
        results: Box::default(),
        hardcopy: Box::default(),
        settings: Box::default(),
        cloud: Box::default(),
        sheets: Box::default(),
        models: Box::default(),
        runs: Box::default(),
        workbench: Box::default(),
        shortcut_resolver:
            crate::workbench::app_state::session::shortcuts::ShortcutResolverState::default(),
    };
    // The slots derive their `Default`; these are the values a new session
    // was always built with, set where they always were.
    state.design.property_registry = crate::properties::PropertyEditorSchema::new();
    state.results.calculator_panel = super::calculator::CalculatorPanel::new();
    *state.models.pdk_settings_dialog = super::pdk_settings::PdkSettingsDialogState::new();
    *state.results.analysis = analysis;
    state.design.schematic_snap.grid_size = snap_grid_size;
    if let Some((manifest, runtime_snapshot)) = automation {
        state.code.code_workspace.automation.manifest = Some(manifest);
        state.code.code_workspace.automation.runtime_snapshot = Some(runtime_snapshot);
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
            .model
            .workspace
            .content
            .project_sources
            .bundle_for_owner(&crate::state::ProjectSourceOwner::code_workspace(
                ProjectSourceLanguage::RSpiceAutomation,
            ))
            .expect("bootstrapped Automation workspace");
        let snapshot = state
            .code
            .code_workspace
            .automation
            .runtime_snapshot
            .as_ref()
            .expect("bootstrapped Automation runtime snapshot");

        assert_eq!(
            snapshot.project_id,
            state.model.workspace.content.project.id().as_uuid()
        );
        assert_eq!(snapshot.workspace_revision, automation.revision().get());
        assert_eq!(
            snapshot.closure_digest.0,
            *automation.closure_digest().as_bytes()
        );
        assert!(
            state.code.code_workspace.automation.receipt.is_none(),
            "static startup parsing must not impersonate managed-CPython validation"
        );
        assert!(state.code.code_workspace.automation.manifest.is_some());
        assert!(
            state.code.code_workspace.veriloga.receipt.is_none(),
            "a new project has no Verilog-A source, so startup compiles none"
        );
    }
}
