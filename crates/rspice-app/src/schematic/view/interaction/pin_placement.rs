//! Placement and document authority for a named pin sequence.

use super::{AppState, ConsoleMessage, Point};
use crate::state::PendingPortSequence;

/// Place the next name of the armed sequence, and stay armed until the names
/// run out.
///
/// A refusal — a name taken since the form was filled, an order that moved —
/// keeps the sequence: the reader can undo whatever took the name, or click
/// again somewhere the model accepts. A *document* change ends it, because the
/// pins were named for a cell that is no longer the one on screen.
pub(super) fn place_pending_port(state: &mut AppState, grid_pos: Point) {
    let Some(sequence) = state.schematic.pending_port_sequence.clone() else {
        state.push_user_message(ConsoleMessage::warning(
            "Pin placement ended: no names are armed.".to_owned(),
        ));
        state.schematic.cancel_tool();
        return;
    };
    if state.schematic_edit_read_only() || !placement_authority_matches(state, &sequence) {
        state.push_user_message(ConsoleMessage::warning(
            "Pin placement ended: the active schematic changed.".to_owned(),
        ));
        state.schematic.cancel_tool();
        return;
    }
    let Some(pending) = sequence.next_placement(&state.schematic) else {
        state.schematic.cancel_tool();
        return;
    };
    let name = pending.name.clone();
    match state.schematic.place_pending_port(grid_pos, pending) {
        Ok(stable_id) => {
            let more = state
                .schematic
                .pending_port_sequence
                .as_mut()
                .is_some_and(PendingPortSequence::advance);
            if !more {
                state.schematic.cancel_tool();
            }
            state.sync_active_schematic_to_workspace();
            state.push_user_message(ConsoleMessage::info(format!(
                "Placed pin {name} at ({}, {}).",
                grid_pos.x, grid_pos.y
            )));
            log::info!("Placed interface port {name} as {stable_id} at {grid_pos:?}");
        }
        Err(error) => {
            state.push_user_message(ConsoleMessage::warning(format!(
                "Pin {name} was not placed: {error}."
            )));
        }
    }
}

/// `true` when the armed sequence names the document the click landed in.
fn placement_authority_matches(state: &AppState, sequence: &PendingPortSequence) -> bool {
    sequence.authority.as_ref().is_some_and(|authority| {
        authority.matches(
            state.design_execution_epoch,
            state.active_schematic_epoch,
            &state.workspace.active_view.display_path(),
        )
    })
}
