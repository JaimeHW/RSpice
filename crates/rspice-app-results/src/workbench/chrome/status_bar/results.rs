//! The status bar's Results segments: the plotted interval the panes show,
//! and the plot magnification in the zoom chip.

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
/// the interval the panes are showing. A sheet with no plotted interval — a
/// table, or a run that has not been opened yet — says so: the run segment
/// at the bar's right names the run, and naming it here too printed it
/// twice.
pub fn results_view_summary(
    results_view: Option<&crate::workbench::documents::result_document::SharedXStatus>,
) -> String {
    results_view.map_or_else(
        || "No plotted interval".to_owned(),
        |view| view.span.clone(),
    )
}
