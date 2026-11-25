//! The results feature observed through a whole application: the commands
//! that reach the Results workspace and its return edges, and the cases that
//! read or write another feature's state on the way.

mod availability;
mod calculator;
mod clear_history;
mod commands;
mod create_document;
mod cross_probe;
mod deck_outputs;
mod document_edit;
mod expression_diagnostics;
#[cfg(not(target_arch = "wasm32"))]
mod full_disk;
pub(super) mod histogram_publication;
mod inspector;
mod keys;
#[cfg(not(target_arch = "wasm32"))]
mod large_data;
mod loop_gain;
mod navigation;
mod navigator_browser;
mod op_inspector;
mod optimization_apply;
mod persistent_document;
mod result_import_adapters;
mod result_import_workflow;
mod retained_memo;
mod runs;
mod saved_documents;
mod sheet_print;
mod shown_analysis;
mod sweep_sheet;
mod two_windows;
mod viewer_capabilities;
mod waveform_export;
mod working_view;

use crate::workbench::RSpiceApp;
use rspice_app_results::workbench::documents::result_document::ResultsParts;
use rspice_app_runs::simulation::controller::SimulationController;

/// Lend `app`'s results parts and run controller to `draw`, as the shell
/// lends them to the results surface.
fn with_surface_parts<R>(
    app: &mut RSpiceApp,
    draw: impl FnOnce(&mut ResultsParts<'_>, &mut SimulationController) -> R,
) -> R {
    draw(
        &mut app.state.results_parts(),
        &mut app.simulation_controller,
    )
}
