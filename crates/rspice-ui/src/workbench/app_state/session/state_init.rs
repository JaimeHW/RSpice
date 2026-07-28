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
    if let Some(document) = workspace
        .project_sources
        .get(crate::state::ProjectSourceLanguage::RSpiceAutomation)
    {
        let plan = crate::automation_workflow::compile_workflow(document.content())
            .expect("the canonical bootstrapped Automation source must compile");
        ui.code_workspace.automation.receipt = Some(
            crate::workbench::documents::code_workspace::AutomationValidationReceipt {
                token: crate::workbench::documents::code_workspace::SourceOperationToken {
                    project_id: workspace.project.id(),
                    revision: document.revision().get(),
                    content_digest: document.content_digest(),
                },
                plan,
            },
        );
    }

    AppState {
        schematic,
        simulation: crate::state::SimulationState::default(),
        design_execution_epoch: 0,
        active_schematic_epoch: 0,
        project_design_history: Default::default(),
        dialogs: DialogState::default(),
        sim_setup,
        log_buffer: crate::diagnostics::LogBuffer::default(),
        script_console: crate::workbench::panels::ScriptConsoleState::default(),
        library_manager,
        workspace,
        project_lifecycle: crate::workbench::lifecycle::project_lifecycle::ProjectLifecycleState::default(),
        pending_delete_cell: None,
        pending_delete_view: None,
        tabbed_property_dialog: crate::properties::TabbedPropertyDialogState::default(),
        property_registry: crate::state::PropertyRegistry::new(),
        calculator_panel: crate::workbench::panels::calculator::CalculatorPanel::new(),
        pdk_settings_dialog: crate::workbench::panels::PdkSettingsDialogState::new(),
        pdk_config: crate::state::pdk_config::PdkConfig::load_or_default(),
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
    }
}

#[cfg(test)]
mod tests {
    use crate::state::ProjectSourceLanguage;

    #[test]
    fn bootstrapped_code_sources_have_exact_current_runtime_receipts() {
        let state = super::default_app_state();
        let automation = state
            .workspace
            .project_sources
            .get(ProjectSourceLanguage::RSpiceAutomation)
            .expect("bootstrapped Automation source");
        let receipt = state
            .ui
            .code_workspace
            .automation
            .receipt
            .expect("bootstrapped Automation receipt");

        assert_eq!(receipt.token.project_id, state.workspace.project.id());
        assert_eq!(receipt.token.revision, automation.revision().get());
        assert_eq!(receipt.token.content_digest, automation.content_digest());
        assert_eq!(receipt.plan.project_name(), "Lab characterization");
        assert!(state.ui.code_workspace.veriloga.receipt.is_some());
    }
}
