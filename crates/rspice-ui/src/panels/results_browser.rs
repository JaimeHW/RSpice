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

use crate::common::app::{AppState, BottomPanelTab};
use crate::state::AnalysisType;
use crate::viewers::ActiveViewer;

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
                state.simulation.clear_runs();
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
                    id: run.id,
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
                    state.simulation.delete_run(run_data.index);
                }

                // Handle run selection
                if run_response.run_selected {
                    state.simulation.select_run(run_data.index);
                }

                // Handle analysis selection
                if let Some(analysis_idx) = run_response.analysis_selected {
                    state.simulation.select_run(run_data.index);
                    state.simulation.select_analysis(analysis_idx);
                }

                // Handle analysis view request
                if let Some((analysis_idx, analysis_type)) = run_response.analysis_view_requested {
                    state.simulation.select_run(run_data.index);
                    state.simulation.select_analysis(analysis_idx);

                    // Switch to appropriate viewer tab
                    state.panels.bottom_panel = true;
                    state.panels.active_bottom_tab = analysis_type_to_tab(analysis_type);
                    state.active_viewer = analysis_type_to_viewer(analysis_type);
                }
            }
        });
}

// =============================================================================
// Internal Data Structures
// =============================================================================

/// Display data for a run (extracted to avoid borrow conflicts)
struct RunDisplayData {
    index: usize,
    id: u64,
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
    let type_icon = analysis_type_icon(analysis.analysis_type);
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
// Helper Functions
// =============================================================================

/// Get icon for analysis type (using ASCII-safe abbreviations)
fn analysis_type_icon(analysis_type: AnalysisType) -> &'static str {
    match analysis_type {
        AnalysisType::DcOp => "[OP]",
        AnalysisType::DcSweep => "[DC]",
        AnalysisType::Ac => "[AC]",
        AnalysisType::Disto => "[DI]",
        AnalysisType::Transient => "[TR]",
        AnalysisType::Noise => "[NO]",
        AnalysisType::PoleZero => "[PZ]",
        AnalysisType::Tf => "[TF]",
        AnalysisType::Sensitivity => "[SN]",
        AnalysisType::Pac => "[PAC]",
        AnalysisType::Pnoise => "[PN]",
        AnalysisType::Pxf => "[PXF]",
        AnalysisType::Pstb => "[PSB]",
        AnalysisType::Stb => "[STB]",
        AnalysisType::MonteCarlo => "[MC]",
        AnalysisType::Parametric => "[PA]",
        AnalysisType::Corner => "[CR]",
        AnalysisType::Reliability => "[REL]",
        AnalysisType::Optimization => "[OPT]",
        AnalysisType::Soa => "[SOA]",
        AnalysisType::SParameter => "[SP]",
        AnalysisType::Envelope => "[ENV]",
        AnalysisType::Fourier => "[FOU]",
        AnalysisType::HarmonicBalance => "[HB]",
        AnalysisType::Pss => "[PS]",
    }
}

/// Map analysis type to appropriate bottom panel tab
fn analysis_type_to_tab(analysis_type: AnalysisType) -> BottomPanelTab {
    match analysis_type {
        AnalysisType::DcOp => BottomPanelTab::Log,
        AnalysisType::DcSweep => BottomPanelTab::Waveform,
        AnalysisType::Ac => BottomPanelTab::Waveform, // AC uses waveform viewer for now
        AnalysisType::Disto => BottomPanelTab::Waveform,
        AnalysisType::Transient => BottomPanelTab::Waveform,
        AnalysisType::Noise => BottomPanelTab::Waveform,
        AnalysisType::PoleZero => BottomPanelTab::Waveform, // PoleZero uses waveform viewer for now
        AnalysisType::Tf => BottomPanelTab::Waveform,
        AnalysisType::Sensitivity => BottomPanelTab::Waveform,
        AnalysisType::Pac => BottomPanelTab::Waveform,
        AnalysisType::Pnoise => BottomPanelTab::Waveform,
        AnalysisType::Pxf => BottomPanelTab::Waveform,
        AnalysisType::Pstb => BottomPanelTab::Waveform,
        AnalysisType::Stb => BottomPanelTab::Waveform,
        AnalysisType::MonteCarlo => BottomPanelTab::Waveform,
        AnalysisType::Parametric => BottomPanelTab::Waveform,
        AnalysisType::Corner => BottomPanelTab::Waveform,
        AnalysisType::Reliability => BottomPanelTab::Waveform,
        AnalysisType::Optimization => BottomPanelTab::Waveform,
        AnalysisType::Soa => BottomPanelTab::Waveform,
        AnalysisType::SParameter => BottomPanelTab::Waveform,
        AnalysisType::Envelope => BottomPanelTab::Waveform,
        AnalysisType::Fourier => BottomPanelTab::Waveform,
        AnalysisType::HarmonicBalance => BottomPanelTab::Waveform,
        AnalysisType::Pss => BottomPanelTab::Waveform,
    }
}

fn analysis_type_to_viewer(analysis_type: AnalysisType) -> ActiveViewer {
    match analysis_type {
        AnalysisType::DcOp => ActiveViewer::Waveform,
        AnalysisType::DcSweep | AnalysisType::Transient | AnalysisType::Envelope => {
            ActiveViewer::Waveform
        }
        AnalysisType::Ac
        | AnalysisType::Disto
        | AnalysisType::Tf
        | AnalysisType::Pac
        | AnalysisType::Pxf => ActiveViewer::BodePlot,
        AnalysisType::Noise | AnalysisType::Pnoise => ActiveViewer::BodePlot,
        AnalysisType::PoleZero => ActiveViewer::PoleZero,
        AnalysisType::Sensitivity => ActiveViewer::Waveform,
        AnalysisType::Pstb | AnalysisType::Stb => ActiveViewer::Nyquist,
        AnalysisType::MonteCarlo | AnalysisType::Corner | AnalysisType::Parametric => {
            ActiveViewer::Histogram
        }
        AnalysisType::Reliability | AnalysisType::Optimization | AnalysisType::Soa => {
            ActiveViewer::Waveform
        }
        AnalysisType::SParameter => ActiveViewer::SmithChart,
        AnalysisType::Fourier => ActiveViewer::Fft,
        AnalysisType::HarmonicBalance | AnalysisType::Pss => ActiveViewer::Waveform,
    }
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
            let icon = analysis_type_icon(t);
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
            let _ = analysis_type_to_tab(t);
        }
    }

    #[test]
    fn test_analysis_type_to_tab_routes_dcop_to_log() {
        assert_eq!(
            analysis_type_to_tab(AnalysisType::DcOp),
            BottomPanelTab::Log
        );
    }

    #[test]
    fn test_analysis_type_to_viewer_routes_specialized_views() {
        assert_eq!(
            analysis_type_to_viewer(AnalysisType::Ac),
            ActiveViewer::BodePlot
        );
        assert_eq!(
            analysis_type_to_viewer(AnalysisType::Disto),
            ActiveViewer::BodePlot
        );
        assert_eq!(
            analysis_type_to_viewer(AnalysisType::SParameter),
            ActiveViewer::SmithChart
        );
        assert_eq!(
            analysis_type_to_viewer(AnalysisType::Fourier),
            ActiveViewer::Fft
        );
        assert_eq!(
            analysis_type_to_viewer(AnalysisType::PoleZero),
            ActiveViewer::PoleZero
        );
        assert_eq!(
            analysis_type_to_viewer(AnalysisType::MonteCarlo),
            ActiveViewer::Histogram
        );
    }
}
