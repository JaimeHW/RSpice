//! Results Browser Panel
//!
//! Cadence Spectre-style simulation results browser with:
//! - Multi-run history (newest first, up to MAX_RUN_HISTORY runs)
//! - Analysis tree view within each run
//! - Click to select, double-click to view in appropriate viewer
//! - Right-click context menu for run management
//!
//! # Commercial Parity
//!
//! This follows Cadence Spectre/ViVA conventions:
//! - PSF-style database with timestamped runs
//! - Analysis-aware viewer switching
//! - Results retained for comparison

use egui::{CollapsingHeader, RichText, Ui};

use crate::common::analysis_navigation;
use crate::common::app::AppState;
use crate::state::AnalysisType;

// =============================================================================
// Public API
// =============================================================================

/// Render the results browser panel
///
/// This displays a tree view of simulation runs and their analyses,
/// allowing users to navigate and view results from different runs.
pub fn render_results_browser(ui: &mut Ui, state: &mut AppState) {
    // Show run count and status
    let run_count = state.simulation.run_count();
    if run_count == 0 {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.label(
                RichText::new("No simulation results")
                    .color(ui.visuals().text_color().gamma_multiply(0.5)),
            );
            ui.add_space(8.0);
            ui.label(
                RichText::new("Run a simulation to see results here")
                    .size(11.0)
                    .color(ui.visuals().text_color().gamma_multiply(0.4)),
            );
        });
        return;
    }

    // Header with Clear All button
    ui.horizontal(|ui| {
        ui.label(format!(
            "{} run{}",
            run_count,
            if run_count == 1 { "" } else { "s" }
        ));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.small_button("Clear All").clicked() {
                clear_runs_and_sync_viewers(state);
            }
        });
    });
    ui.add_space(4.0);

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            // We need to collect indices first to avoid borrow issues
            let _run_count = state.simulation.runs.len();
            let active_run = state.simulation.active_run_idx;
            let active_analysis = state.simulation.active_analysis_idx;

            // Collect run data for rendering
            let run_data: Vec<RunDisplayData> = state
                .simulation
                .runs
                .iter()
                .enumerate()
                .map(|(idx, run)| RunDisplayData {
                    index: idx,
                    label: run.label.clone(),
                    success: run.success,
                    is_current: idx == 0,
                    analyses: run
                        .analyses
                        .iter()
                        .enumerate()
                        .map(|(a_idx, a)| AnalysisDisplayData {
                            index: a_idx,
                            analysis_type: a.analysis_type,
                            label: a.label.clone(),
                            success: a.success,
                            has_data: a.has_data(),
                        })
                        .collect(),
                })
                .collect();

            // Render each run
            for run_data in run_data {
                let run_response = render_run(
                    ui,
                    &run_data,
                    active_run == Some(run_data.index),
                    active_analysis,
                    state,
                );

                // Handle run deletion (deferred to avoid borrow conflict)
                if run_response.delete_requested {
                    delete_run_and_sync_viewers(state, run_data.index);
                }

                // Handle run selection
                if run_response.run_selected {
                    let previous = (
                        state.simulation.active_run_idx,
                        state.simulation.active_analysis_idx,
                    );
                    state.simulation.select_run(run_data.index);
                    if previous
                        != (
                            state.simulation.active_run_idx,
                            state.simulation.active_analysis_idx,
                        )
                    {
                        state.clear_transient_specialized_viewer_data();
                    }
                }

                // Handle analysis selection
                if let Some(analysis_idx) = run_response.analysis_selected {
                    let previous = (
                        state.simulation.active_run_idx,
                        state.simulation.active_analysis_idx,
                    );
                    state.simulation.select_run(run_data.index);
                    state.simulation.select_analysis(analysis_idx);
                    if previous
                        != (
                            state.simulation.active_run_idx,
                            state.simulation.active_analysis_idx,
                        )
                    {
                        state.clear_transient_specialized_viewer_data();
                    }
                }

                // Handle analysis view request
                if let Some((analysis_idx, analysis_type)) = run_response.analysis_view_requested {
                    activate_analysis_view(state, run_data.index, analysis_idx, analysis_type);
                }
            }
        });
}

fn clear_runs_and_sync_viewers(state: &mut AppState) {
    state.simulation.clear_runs();
    state.clear_specialized_viewer_data();
}

fn delete_run_and_sync_viewers(state: &mut AppState, run_idx: usize) {
    // Only invalidate specialized viewer caches if the active waveform selection changed.
    // Non-active run deletions should preserve the currently loaded specialized data.
    let before_data_version = state.simulation.data_version;
    if state.simulation.delete_run(run_idx) && state.simulation.data_version != before_data_version
    {
        state.clear_specialized_viewer_data();
    }
}

fn activate_analysis_view(
    state: &mut AppState,
    run_idx: usize,
    analysis_idx: usize,
    analysis_type: AnalysisType,
) {
    let previous = (
        state.simulation.active_run_idx,
        state.simulation.active_analysis_idx,
    );
    state.simulation.select_run(run_idx);
    state.simulation.select_analysis(analysis_idx);
    if previous
        != (
            state.simulation.active_run_idx,
            state.simulation.active_analysis_idx,
        )
    {
        state.clear_transient_specialized_viewer_data();
    }
    state.open_preferred_viewer_for_analysis(analysis_type);
}

// =============================================================================
// Internal Data Structures
// =============================================================================

/// Display data for a run (extracted to avoid borrow conflicts)
struct RunDisplayData {
    index: usize,
    label: String,
    success: bool,
    is_current: bool,
    analyses: Vec<AnalysisDisplayData>,
}

/// Display data for an analysis
struct AnalysisDisplayData {
    index: usize,
    analysis_type: AnalysisType,
    label: String,
    success: bool,
    has_data: bool,
}

/// Response from rendering a run
struct RunRenderResponse {
    delete_requested: bool,
    run_selected: bool,
    analysis_selected: Option<usize>,
    analysis_view_requested: Option<(usize, AnalysisType)>,
}

// =============================================================================
// Run Rendering
// =============================================================================

/// Render a single simulation run with its analyses
fn render_run(
    ui: &mut Ui,
    run: &RunDisplayData,
    is_active: bool,
    active_analysis_idx: Option<usize>,
    state: &AppState,
) -> RunRenderResponse {
    let mut response = RunRenderResponse {
        delete_requested: false,
        run_selected: false,
        analysis_selected: None,
        analysis_view_requested: None,
    };

    // Build run header - using triangles for expand/collapse indicators
    let status_indicator = if run.is_current { ">" } else { "-" };
    let status_color = if run.success {
        egui::Color32::from_rgb(100, 200, 100)
    } else {
        egui::Color32::from_rgb(200, 100, 100)
    };

    let current_marker = if run.is_current { " [current]" } else { "" };
    let header_text = format!("{} {}{}", status_indicator, run.label, current_marker);

    // Use frame to highlight active run
    let frame = if is_active {
        egui::Frame::none()
            .fill(ui.visuals().selection.bg_fill.gamma_multiply(0.5))
            .rounding(2.0)
    } else {
        egui::Frame::none()
    };

    frame.show(ui, |ui| {
        CollapsingHeader::new(RichText::new(header_text).color(status_color))
            .default_open(run.is_current)
            .show(ui, |ui| {
                // Render analyses
                for analysis in &run.analyses {
                    let analysis_response = render_analysis(
                        ui,
                        analysis,
                        is_active && active_analysis_idx == Some(analysis.index),
                        state,
                    );

                    if analysis_response.selected {
                        response.analysis_selected = Some(analysis.index);
                    }

                    if analysis_response.view_requested {
                        response.analysis_view_requested =
                            Some((analysis.index, analysis.analysis_type));
                    }
                }

                // Run management buttons
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    if ui
                        .small_button("Delete")
                        .on_hover_text("Delete this run")
                        .clicked()
                    {
                        response.delete_requested = true;
                    }
                });
            });
    });

    response
}

// =============================================================================
// Analysis Rendering
// =============================================================================

/// Response from rendering an analysis
struct AnalysisRenderResponse {
    selected: bool,
    view_requested: bool,
}

/// Render a single analysis within a run
fn render_analysis(
    ui: &mut Ui,
    analysis: &AnalysisDisplayData,
    is_active: bool,
    _state: &AppState,
) -> AnalysisRenderResponse {
    let mut response = AnalysisRenderResponse {
        selected: false,
        view_requested: false,
    };

    // Build analysis label with type icon
    let type_icon = analysis_navigation::analysis_icon(analysis.analysis_type);
    let status_icon = if analysis.success { "" } else { " ⚠" };
    let data_icon = if analysis.has_data { "" } else { " (no data)" };

    let label = format!(
        "  {} {} {}{}{}",
        type_icon,
        analysis.analysis_type.short_label(),
        analysis.label,
        status_icon,
        data_icon
    );

    // Style for active analysis
    let text = if is_active {
        RichText::new(&label).strong()
    } else {
        RichText::new(&label)
    };

    let button = ui.selectable_label(is_active, text);

    // Single click selects
    if button.clicked() {
        response.selected = true;
    }

    // Double click opens viewer
    if button.double_clicked() && analysis.has_data {
        response.view_requested = true;
    }

    // Tooltip
    button.on_hover_text(format!(
        "{}\nClick to select, double-click to view",
        analysis.analysis_type.display_name()
    ));

    response
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analysis_type_icon_coverage() {
        // Ensure all analysis types have icons
        let types = [
            AnalysisType::DcOp,
            AnalysisType::DcSweep,
            AnalysisType::Ac,
            AnalysisType::Disto,
            AnalysisType::Transient,
            AnalysisType::Noise,
            AnalysisType::PoleZero,
            AnalysisType::Tf,
            AnalysisType::Sensitivity,
            AnalysisType::Pac,
            AnalysisType::Pnoise,
            AnalysisType::Pxf,
            AnalysisType::Pstb,
            AnalysisType::Stb,
            AnalysisType::MonteCarlo,
            AnalysisType::Parametric,
            AnalysisType::Corner,
            AnalysisType::Reliability,
            AnalysisType::Optimization,
            AnalysisType::Soa,
            AnalysisType::SParameter,
            AnalysisType::Envelope,
            AnalysisType::Fourier,
            AnalysisType::HarmonicBalance,
            AnalysisType::Pss,
        ];

        for t in types {
            let icon = analysis_navigation::analysis_icon(t);
            assert!(!icon.is_empty(), "Missing icon for {:?}", t);
        }
    }

    #[test]
    fn test_analysis_type_to_tab_coverage() {
        // Ensure all analysis types map to a tab
        let types = [
            AnalysisType::DcOp,
            AnalysisType::DcSweep,
            AnalysisType::Ac,
            AnalysisType::Disto,
            AnalysisType::Transient,
            AnalysisType::Noise,
            AnalysisType::PoleZero,
            AnalysisType::Tf,
            AnalysisType::Sensitivity,
            AnalysisType::Pac,
            AnalysisType::Pnoise,
            AnalysisType::Pxf,
            AnalysisType::Pstb,
            AnalysisType::Stb,
            AnalysisType::MonteCarlo,
            AnalysisType::Parametric,
            AnalysisType::Corner,
            AnalysisType::Reliability,
            AnalysisType::Optimization,
            AnalysisType::Soa,
            AnalysisType::SParameter,
            AnalysisType::Envelope,
            AnalysisType::Fourier,
            AnalysisType::HarmonicBalance,
            AnalysisType::Pss,
        ];

        for t in types {
            // Just verify it doesn't panic
            let _ = analysis_navigation::preferred_bottom_tab(t);
        }
    }

    #[test]
    fn test_analysis_type_to_tab_routes_dcop_to_log() {
        assert_eq!(
            analysis_navigation::preferred_bottom_tab(AnalysisType::DcOp),
            crate::common::app::BottomPanelTab::Log
        );
    }

    #[test]
    fn test_analysis_type_to_viewer_routes_specialized_views() {
        assert_eq!(
            analysis_navigation::preferred_viewer(AnalysisType::Ac),
            crate::viewers::ActiveViewer::BodePlot
        );
        assert_eq!(
            analysis_navigation::preferred_viewer(AnalysisType::Disto),
            crate::viewers::ActiveViewer::BodePlot
        );
        assert_eq!(
            analysis_navigation::preferred_viewer(AnalysisType::SParameter),
            crate::viewers::ActiveViewer::SmithChart
        );
        assert_eq!(
            analysis_navigation::preferred_viewer(AnalysisType::Fourier),
            crate::viewers::ActiveViewer::Fft
        );
        assert_eq!(
            analysis_navigation::preferred_viewer(AnalysisType::PoleZero),
            crate::viewers::ActiveViewer::PoleZero
        );
        assert_eq!(
            analysis_navigation::preferred_viewer(AnalysisType::MonteCarlo),
            crate::viewers::ActiveViewer::Histogram
        );
    }

    fn seed_run_with_analysis(state: &mut AppState, analysis_type: AnalysisType) {
        let waveform =
            crate::state::WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#00AAFF");
        let result =
            crate::state::AnalysisResult::new(1, analysis_type, analysis_type.display_name())
                .with_waveforms(vec![waveform]);
        state.simulation.start_run().add_analysis(result);
    }

    fn seed_bode_data(state: &mut AppState) {
        let mut response = crate::analysis::bode::FrequencyResponse::new("tf");
        response.add_point(crate::analysis::bode::data::FrequencyPoint::new(
            1.0, 1.0, 0.0,
        ));
        let mut bode = crate::analysis::bode::BodeData::new();
        bode.add_response(response);
        state.bode_plot_state.load_data(bode);
    }

    fn seed_fft_data(state: &mut AppState) {
        let mut fft = crate::analysis::fft::FftData::new("spec");
        fft.points
            .push(crate::analysis::fft::FftPoint::new(1.0, 1.0, 0.0));
        state.fft_state.load_data(fft);
    }

    #[test]
    fn test_activate_analysis_view_prefers_available_ac_viewer() {
        let mut state = AppState::default();
        seed_run_with_analysis(&mut state, AnalysisType::Ac);
        seed_bode_data(&mut state);

        activate_analysis_view(&mut state, 0, 0, AnalysisType::Ac);

        assert_eq!(
            state.active_viewer(),
            crate::viewers::ActiveViewer::BodePlot
        );
        assert_eq!(
            state.panels.active_bottom_tab,
            crate::common::app::BottomPanelTab::Waveform
        );
    }

    #[test]
    fn test_activate_analysis_view_falls_back_across_priority_chain() {
        let mut state = AppState::default();
        seed_run_with_analysis(&mut state, AnalysisType::SParameter);

        // No specialized data loaded: should fall back to waveform.
        activate_analysis_view(&mut state, 0, 0, AnalysisType::SParameter);
        assert_eq!(
            state.active_viewer(),
            crate::viewers::ActiveViewer::Waveform
        );

        // With Bode loaded but no Smith data, S-parameter should choose Bode fallback.
        seed_bode_data(&mut state);
        activate_analysis_view(&mut state, 0, 0, AnalysisType::SParameter);
        assert_eq!(
            state.active_viewer(),
            crate::viewers::ActiveViewer::BodePlot
        );
    }

    #[test]
    fn delete_run_sync_clears_specialized_viewers_when_active_selection_changes() {
        let mut state = AppState::default();
        seed_run_with_analysis(&mut state, AnalysisType::Transient);
        assert!(state.simulation.select_analysis(0));
        seed_fft_data(&mut state);
        assert!(state.viewer_is_available(crate::viewers::ActiveViewer::Fft));

        delete_run_and_sync_viewers(&mut state, 0);

        assert_eq!(state.simulation.run_count(), 0);
        assert!(!state.viewer_is_available(crate::viewers::ActiveViewer::Fft));
    }

    #[test]
    fn delete_run_sync_preserves_specialized_viewers_when_active_data_unchanged() {
        let mut state = AppState::default();
        seed_run_with_analysis(&mut state, AnalysisType::Transient);
        seed_run_with_analysis(&mut state, AnalysisType::Transient);
        assert!(state.simulation.select_run(0));
        assert!(state.simulation.select_analysis(0));
        seed_fft_data(&mut state);
        assert!(state.viewer_is_available(crate::viewers::ActiveViewer::Fft));

        delete_run_and_sync_viewers(&mut state, 1);

        assert_eq!(state.simulation.run_count(), 1);
        assert!(state.viewer_is_available(crate::viewers::ActiveViewer::Fft));
    }

    #[test]
    fn clear_runs_sync_clears_specialized_viewers() {
        let mut state = AppState::default();
        seed_run_with_analysis(&mut state, AnalysisType::Transient);
        assert!(state.simulation.select_analysis(0));
        seed_fft_data(&mut state);
        assert!(state.viewer_is_available(crate::viewers::ActiveViewer::Fft));

        clear_runs_and_sync_viewers(&mut state);

        assert_eq!(state.simulation.run_count(), 0);
        assert!(!state.viewer_is_available(crate::viewers::ActiveViewer::Fft));
    }
}
