//! The status bar's Results segments: the plotted interval the panes show,
//! and the plot magnification in the zoom chip.

use crate::state::SimulationState;

/// The zoom chip's waveform form: whole numbers once the magnification is
/// deep, one decimal while it is shallow enough for the fraction to matter.
pub fn format_plot_zoom(zoom: f64) -> String {
    if zoom >= 10.0 {
        return format!("{zoom:.0}×");
    }
    let text = format!("{zoom:.1}");
    format!("{}×", text.strip_suffix(".0").unwrap_or(text.as_str()))
}

/// The Results coordinate segment.
///
/// The instrument bar already carries the A/B cursor readout, so this states
/// the interval the panes are showing. Sheets with no plotted interval — a
/// table, or a run that has not been opened yet — name the dataset instead.
pub fn results_view_summary(
    simulation: &SimulationState,
    results_view: Option<&crate::workbench::documents::result_document::SharedXStatus>,
) -> String {
    if let Some(view) = results_view {
        return view.span.clone();
    }
    simulation
        .view
        .active_run_idx
        .and_then(|index| simulation.retained.runs.get(index))
        .map_or_else(
            // The selection segment beside this one already reports that no
            // dataset is chosen. This segment answers a different question —
            // what interval am I reading — and has to say so in its own
            // words, or the bar prints the same sentence twice.
            || "No plotted interval".to_owned(),
            |run| run.label.clone(),
        )
}
