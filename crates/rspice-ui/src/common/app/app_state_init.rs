use super::{
    AnalysisWorkspaceState, AppState, DialogState, PanelSizes, PanelVisibility, RSpiceTheme,
};

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

    AppState {
        schematic: crate::state::SchematicState::default(),
        simulation: crate::state::SimulationState::default(),
        panels: PanelVisibility::default(),
        panel_sizes: PanelSizes::default(),
        dialogs: DialogState::default(),
        theme: RSpiceTheme::dark(),
        console_messages: Vec::new(),
        log_buffer: crate::panels::LogBuffer::default(),
        log_panel_state: crate::panels::LogPanelState::default(),
        property_editor: crate::properties::dialog::PropertyEditorState::default(),
        script_console: crate::panels::ScriptConsoleState::default(),
        viewer_workspace: crate::viewers::ViewerWorkspace::default(),
        waveform_viewer: crate::waveform::WaveformViewerState::default(),
        library_manager: crate::state::LibraryManager::with_primitives(),
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
    }
}

