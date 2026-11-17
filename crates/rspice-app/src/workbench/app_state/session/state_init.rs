//! Building the default session.
//!
//! The initial [`AppState`], and the default model library and analysis
//! viewers behind it; and the state a launch starts from.

use crate::workbench::app_state::AppState;
use rspice_app_models::workbench::app_state::session::pdk_settings::PdkSettingsDialogState;
use rspice_app_results::workbench::app_state::session::calculator::CalculatorPanel;
use rspice_app_results::workbench::app_state::viewer_capabilities::AnalysisWorkspaceState;
use rspice_app_settings::workbench::app_state::session::preferences_file::PreferencesStore;
use rspice_app_state::diagnostics::{LogSeverity, LogSource};
use rspice_app_state::workbench::workspace::restore_session_model_library_manager;

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
    let (pdk_config, pdk_load_error) = match rspice_app_state::state::pdk_config::load() {
        Ok(config) => (config, None),
        Err(error) => (
            rspice_app_state::state::pdk_config::PdkConfig::default(),
            Some(error.to_string()),
        ),
    };
    // Browser PDK state is hydrated asynchronously from IndexedDB by the
    // application persistence owner. Until that exact read completes, no
    // persisted trust or package authority is present in the live state.
    #[cfg(all(not(test), target_arch = "wasm32"))]
    let (pdk_config, pdk_load_error) = (
        rspice_app_state::state::pdk_config::PdkConfig::default(),
        None::<String>,
    );
    // Unit tests must never inherit user- or machine-owned trust state. Apart
    // from making tests nondeterministic, doing so could grant a fixture
    // authority it did not explicitly provision.
    #[cfg(test)]
    let (pdk_config, pdk_load_error) = (
        rspice_app_state::state::pdk_config::PdkConfig::default(),
        None::<String>,
    );
    let model = rspice_app_state::workbench::app_state::model::Model::session_start(
        pdk_config,
        pdk_load_error,
        restore_session_model_library_manager,
    );
    let snap_grid_size = model
        .schematic
        .document()
        .document_policy
        .grid_pitch
        .canvas_grid_size();

    let mut state = AppState {
        model,
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
            rspice_app_workbench::workbench::app_state::session::shortcuts::ShortcutResolverState::default(),
    };
    // The slots derive their `Default`; these are the values a new session
    // was always built with, set where they always were.
    state.design.property_registry = rspice_app_design::properties::PropertyEditorSchema::new();
    state.results.calculator_panel = CalculatorPanel::new();
    *state.models.pdk_settings_dialog = PdkSettingsDialogState::new();
    *state.results.analysis = analysis;
    state.design.schematic_snap.grid_size = snap_grid_size;
    // A first start opens on this untitled project: until it is edited,
    // New project, Close project and Exit have nothing of it to save.
    rspice_app_services::workbench::lifecycle::project_lifecycle::mark_pristine(&mut state);
    state
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_new_session_compiles_no_veriloga_source() {
        let state = super::default_app_state();
        assert!(
            state.code.code_workspace.veriloga.receipt.is_none(),
            "a new project has no Verilog-A source, so startup compiles none"
        );
    }
}
