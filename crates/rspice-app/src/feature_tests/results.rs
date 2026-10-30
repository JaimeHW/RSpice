//! The results feature observed through a whole application: the commands
//! that reach the Results workspace and its return edges, and the cases that
//! read or write another feature's state on the way.

mod availability;
mod calculator;
mod commands;
mod create_document;
mod document_edit;
pub(super) mod histogram_publication;
mod inspector;
mod navigation;
mod op_inspector;
mod persistent_document;
mod result_import_adapters;
mod result_import_workflow;
mod retained_memo;
mod specs;
mod surface;
mod viewer_capabilities;
mod waveform_export;

use crate::simulation::SimulationController;
use crate::workbench::RSpiceApp;
use crate::workbench::documents::result_document::ResultsParts;

/// Lend `app`'s results parts, run controller and selected specification to
/// `draw`, as the shell lends them to the results surface.
fn with_surface_parts<R>(
    app: &mut RSpiceApp,
    draw: impl FnOnce(&mut ResultsParts<'_>, &mut SimulationController, Option<&str>) -> R,
) -> R {
    let selected_specification = app.state.simulate.selected_specification.clone();
    draw(
        &mut app.state.results_parts(),
        &mut app.simulation_controller,
        selected_specification.as_deref(),
    )
}
