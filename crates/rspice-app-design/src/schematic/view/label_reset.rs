//! Reset label position: each selected part's name and value go back to
//! where the layout places them, clear of the part's turned body, as one undo
//! step. KiCad's Autoplace Fields does the same for a symbol's fields.

use super::{Model, WorkbenchSession, sync_active_schematic};
use crate::state::Component;
use rspice_design::schematic::label_position::LabelPosition;

/// Whether `part` letters its name or value anywhere but where the layout
/// places it: moved by hand, or set where an imported drawing put it.
pub(crate) fn has_placed_text(part: &Component) -> bool {
    part.name_label_pos != LabelPosition::Auto
        || part.value_label_pos != LabelPosition::Auto
        || !part.value_pieces.is_empty()
}

/// The selected parts the command would change, each beside its reset copy.
fn resets(model: &Model) -> Vec<(Component, Component)> {
    let selection = &model.schematic.session.editor.selection;
    model
        .schematic
        .document()
        .components
        .iter()
        .filter(|part| selection.components.contains(&part.id) && has_placed_text(part))
        .map(|part| {
            let mut reset = part.clone();
            reset.name_label_pos = LabelPosition::Auto;
            reset.value_label_pos = LabelPosition::Auto;
            reset.value_pieces.clear();
            (part.clone(), reset)
        })
        .collect()
}

/// Whether the selection holds a part whose name or value has been placed.
pub(crate) fn selection_holds_placed_text(model: &Model) -> bool {
    let selection = &model.schematic.session.editor.selection;
    model
        .schematic
        .document()
        .components
        .iter()
        .any(|part| selection.components.contains(&part.id) && has_placed_text(part))
}

/// Why Reset label position cannot run now, or `None` when it can.
pub(crate) fn reset_block_reason(model: &Model) -> Option<&'static str> {
    if model.schematic_edit_read_only() {
        Some("the active schematic is read-only")
    } else if !selection_holds_placed_text(model) {
        Some("select a part whose name or value has been moved")
    } else {
        None
    }
}

/// Reset label position as a command runs it, and say what it did.
pub(crate) fn run_reset_label_position(model: &mut Model, wb: &mut WorkbenchSession) {
    if model.schematic_edit_read_only() {
        wb.prompt("The active schematic is read-only.");
        return;
    }
    model.retain_selection_on_active_sheet();
    let edits = resets(model);
    let Some((first, _)) = edits.first() else {
        wb.prompt("The selected names and values already stand where they are placed.");
        return;
    };
    let only = (edits.len() == 1).then(|| first.name.clone());
    match crate::state::bulk_edit::replace_components(
        &mut model.schematic,
        edits,
        "reset label position",
    ) {
        Ok(changed) => {
            sync_active_schematic(model, wb);
            wb.prompt(match only {
                Some(name) => format!("{name}'s name and value are back in place."),
                None => format!("The names and values of {changed} parts are back in place."),
            });
        }
        Err(error) => wb.prompt(error),
    }
}
