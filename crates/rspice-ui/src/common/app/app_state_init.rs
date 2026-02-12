use super::{AppState, DialogState, PanelSizes, PanelVisibility, RSpiceTheme};

pub(super) struct AnalysisViewerDefaults {
    pub pole_zero_state: crate::analysis::pole_zero::PoleZeroState,
    pub bode_plot_state: crate::analysis::bode::BodePlotState,
    pub nyquist_state: crate::analysis::nyquist::NyquistState,
    pub eye_diagram_state: crate::analysis::eye_diagram::EyeDiagramState,
    pub fft_state: crate::analysis::fft::FftState,
    pub smith_chart_state: crate::analysis::smith_chart::SmithChartState,
    pub histogram_state: crate::analysis::histogram::HistogramState,
}

pub(super) fn default_model_library_manager() -> crate::state::model_library::ModelLibraryManager {
    let mut manager = crate::state::model_library::ModelLibraryManager::new();
    manager.load_builtin_models();
    manager
}

pub(super) fn default_analysis_viewers() -> AnalysisViewerDefaults {
    AnalysisViewerDefaults {
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
        op_annotation_renderer: crate::schematic::op_annotation::OpAnnotationRenderer::new(),
        pdk_settings_dialog: crate::panels::PdkSettingsDialogState::new(),
        pdk_config: crate::state::pdk_config::PdkConfig::load_or_default(),
        model_library_manager: default_model_library_manager(),
        model_browser_state: crate::properties::model_browser::ModelBrowserState::default(),
        exit_requested: false,
        pole_zero_state: analysis.pole_zero_state,
        bode_plot_state: analysis.bode_plot_state,
        nyquist_state: analysis.nyquist_state,
        eye_diagram_state: analysis.eye_diagram_state,
        fft_state: analysis.fft_state,
        smith_chart_state: analysis.smith_chart_state,
        histogram_state: analysis.histogram_state,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_model_library_manager_loads_builtin_models() {
        let manager = default_model_library_manager();
        assert!(
            manager.library_count() > 0,
            "built-in model libraries should be loaded at startup"
        );
        assert!(
            manager.total_model_count() > 0,
            "built-in model library set should include models"
        );
    }

    #[test]
    fn test_default_app_state_initializes_expected_subsystems() {
        let state = default_app_state();
        assert!(!state.library_manager.libraries_sorted().is_empty());
        assert!(state.model_library_manager.library_count() > 0);
        assert!(state.model_library_manager.total_model_count() > 0);
        assert!(!state.exit_requested);
    }
}
