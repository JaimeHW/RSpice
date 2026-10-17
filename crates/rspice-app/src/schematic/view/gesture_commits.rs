//! The canvas's gesture commits, for tests outside the canvas that drive an
//! edit the way the pointer does.
//!
//! Each function is the call the canvas makes once a gesture lands: a click
//! with a placement or junction tool armed, Enter on an armed Move or
//! Stretch. A test that calls these runs the edit the user's gesture runs,
//! without painting a canvas.

use super::{
    AppState, interaction, move_interaction, schematic_symbol_context, stretch_interaction,
};
use crate::state::{ComponentType, Point};

/// A click with the placement tool for `component_type` armed, landing at
/// `at` on the grid.
pub(crate) fn place_component(state: &mut AppState, component_type: ComponentType, at: Point) {
    interaction::place_component(state, component_type, at);
}

/// A click with the junction tool at `at`: it adds a dot where two
/// conductors meet, or takes away the one already there.
pub(crate) fn click_junction(state: &mut AppState, at: Point) {
    interaction::handle_junction_click(state, at);
}

/// Enter on an armed Move whose preview stands at its current delta.
pub(crate) fn commit_move(state: &mut AppState) {
    let symbols = schematic_symbol_context(state);
    move_interaction::commit_armed_move_selection(state, &symbols);
}

/// Enter on an armed Stretch whose preview stands at its current delta.
pub(crate) fn commit_stretch(state: &mut AppState) {
    let symbols = schematic_symbol_context(state);
    stretch_interaction::commit_armed_stretch_selection(state, &symbols);
}
