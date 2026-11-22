//! The no-connect tool: a click on a pin's end marks the pin as meant to stay
//! open, and a click on a marked pin clears the mark.
//!
//! The mark is a flag on the part's pin, as KiCad's and Virtuoso's no-connect
//! marks are: it changes no node, only what the design check says about the
//! pin and the cross the sheet draws at its end.

use rspice_schematic_editor::view::symbol_context::SchematicSymbolContext;

use super::parts::CanvasParts;
use crate::state::{Point, SchematicState};

/// A click with the no-connect tool at `position`: the pin end nearest it
/// within `radius` changes its mark, and the status line says so.
pub(super) fn handle_no_connect_click(
    state: &mut CanvasParts<'_>,
    position: Point,
    radius: i32,
    symbols: &SchematicSymbolContext,
) {
    let message = match state.design_view().pin_end_near(symbols, position, radius) {
        Some((component_id, pin)) => {
            toggle_pin_no_connect(&mut state.model.schematic, component_id, &pin)
        }
        None => "Click the end of a pin to mark it no-connect.".to_owned(),
    };
    state.prompt(message);
}

/// Mark `pin` of the part `component_id` no-connect, or clear its mark, as
/// one step of the history, and say what changed.
pub(crate) fn toggle_pin_no_connect(
    schematic: &mut SchematicState,
    component_id: u64,
    pin: &str,
) -> String {
    let Some(component) = schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)
    else {
        return "That part is no longer on the sheet.".to_owned();
    };
    let flagged = !component.is_no_connect(pin);
    let part = if component.name.trim().is_empty() {
        component.kind.display_name().to_owned()
    } else {
        component.name.clone()
    };
    let description = if flagged {
        "mark pin no-connect"
    } else {
        "clear pin no-connect mark"
    };
    schematic.with_undo(description, |schematic| {
        schematic.set_pin_no_connect(component_id, pin, flagged);
    });
    if flagged {
        format!("Marked pin {part}.{pin} no-connect.")
    } else {
        format!("Cleared the no-connect mark on pin {part}.{pin}.")
    }
}
