use super::{AnalysisWorkspaceState, AppState, DialogState};

pub(super) fn default_model_library_manager() -> crate::state::model_library::ModelLibraryManager {
    let mut manager = crate::state::model_library::ModelLibraryManager::new();
    manager.load_builtin_models();
    manager
}

pub(super) fn default_analysis_viewers() -> AnalysisWorkspaceState {
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

pub(super) fn default_app_state() -> AppState {
    let analysis = default_analysis_viewers();
    let mut library_manager = crate::state::LibraryManager::with_primitives();
    let mut workspace = crate::state::ProjectWorkspace::new_bootstrapped(&mut library_manager);
    let schematic = workspace
        .active_schematic()
        .cloned()
        .unwrap_or_else(crate::state::SchematicState::default);
    workspace.save_active_schematic(&schematic);

    AppState {
        schematic,
        simulation: crate::state::SimulationState::default(),
        dialogs: DialogState::default(),
        sim_setup: super::SimSetupState::default(),
        log_buffer: crate::panels::LogBuffer::default(),
        script_console: crate::panels::ScriptConsoleState::default(),
        library_manager,
        workspace,
        pending_delete_cell: None,
        pending_delete_view: None,
        tabbed_property_dialog: crate::properties::TabbedPropertyDialogState::default(),
        property_registry: crate::state::PropertyRegistry::new(),
        calculator_panel: crate::panels::calculator::CalculatorPanel::new(),
        pdk_settings_dialog: crate::panels::PdkSettingsDialogState::new(),
        pdk_config: crate::state::pdk_config::PdkConfig::load_or_default(),
        model_library_manager: default_model_library_manager(),
        model_browser_state: crate::properties::model_browser::ModelBrowserState::default(),
        exit_requested: false,
        analysis,
        shell: crate::shell::ShellState::new(),
    }
}
