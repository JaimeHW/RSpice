//! Showing one placed instance on the drawing it is in.
//!
//! Several lists name an instance that is somewhere else — the Studio's
//! Excitations ledger, the Stimulus Library's adopter cards — and every one of
//! them owes the reader the same transaction: show the design, select the
//! instance, and centre the view on it. Arriving at a selected but off-screen
//! instance is the one outcome that reads as a broken link, and a second
//! spelling of this is how one surface came to select without centring.
//!
//! An instance drawn inside another occurrence is shown but not selected.
//! Selection is a transaction against the buffer on screen and a component id
//! is unique only inside one, so selecting there would point at whichever
//! instance of the open sheet happened to carry that id. The caller states
//! which occurrence to descend to instead.

use crate::state::InstancePath;
use crate::workbench::WorkbenchSession;
use crate::workbench::app_state::{AppState, Model};
use crate::workbench::state::Workspace;

/// Show the design with this instance selected and centred.
pub(in crate::workbench) fn placed_instance(
    state: &mut AppState,
    occurrence: Option<&InstancePath>,
    component_id: u64,
) {
    placed_instance_in(
        &mut state.model,
        &mut state.workbench,
        occurrence,
        component_id,
    );
}

/// [`placed_instance`] over the parts it reads and writes.
pub(in crate::workbench) fn placed_instance_in(
    model: &mut Model,
    wb: &mut WorkbenchSession,
    occurrence: Option<&InstancePath>,
    component_id: u64,
) {
    wb.activate(Workspace::Design);
    if !reaches_in(model, occurrence) {
        return;
    }
    let position = model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)
        .map(|component| component.pos);
    model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(component_id);
    model.schematic.session.editor.net_highlight.clear();
    model.schematic.session.editor.center_request = position;
}

/// Whether the buffer on screen is the one this occurrence names.
///
/// `None` is the reading a single-sheet walk produces, which is by definition
/// the sheet in front of the reader. Every verb that edits one instance asks
/// this first, because the component id it holds means nothing anywhere else.
#[must_use]
pub(in crate::workbench) fn reaches(state: &AppState, occurrence: Option<&InstancePath>) -> bool {
    reaches_in(&state.model, occurrence)
}

/// [`reaches`] over the model it reads.
#[must_use]
fn reaches_in(model: &Model, occurrence: Option<&InstancePath>) -> bool {
    occurrence.is_none_or(|occurrence| *occurrence == model.workspace.content.occurrence_path())
}
