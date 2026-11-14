//! Arming the placement cursor with a stimulus definition.
//!
//! The ADS sources-palette idiom: a definition is a part, so picking one arms
//! the cursor and the next click on the canvas draws an instance that has
//! already adopted it. Every door onto that — the Component shelf's Stimulus
//! library section, the Stimulus Library workspace, the schematic's own menu —
//! comes through here, so there is one refusal vocabulary and one console line
//! for all of them.
//!
//! The armed copy is the library's **saved** revision. A draft being edited in
//! the Stimulus Library workspace is not a revision anything can adopt: an
//! instance stamped from one would read `modified` against a definition the
//! library never published.

use crate::state::{PendingStimulusPlacement, Tool};
use crate::workbench::app_state::Model;
use crate::workbench::state::{WorkbenchSession, Workspace};

/// Arm the next placement with the saved revision of one definition.
///
/// Returns the refusal sentence when there is nothing to arm: the caller shows
/// it where the reader asked, rather than this pushing a console line for a
/// failure the reader is standing in front of.
pub(crate) fn arm_placement_from_definition(
    model: &mut Model,
    wb: &mut WorkbenchSession,
    name: &str,
) -> Result<(), String> {
    if model.schematic_edit_read_only() {
        return Err(format!(
            "The active schematic view is read-only, so '{name}' cannot be placed on it. Reopen \
             the view in an editable context first."
        ));
    }
    let Some(definition) = model.workspace.content.stimulus_library.get(name) else {
        return Err(format!(
            "This project defines no '{name}', so there is no stimulus to place."
        ));
    };
    let placement = PendingStimulusPlacement::of(definition);
    let reference = definition.kind().letter().to_owned();
    let armed = format!("{} r{}", placement.definition(), placement.revision());

    model
        .schematic
        .arm_tool(Tool::Place(placement.component_type));
    model.schematic.session.editor.pending_stimulus = Some(placement);
    wb.activate(Workspace::Design);
    wb.prompt(format!(
        "{reference}\u{2026} from {armed} follows the pointer; click to place, Esc cancels."
    ));
    Ok(())
}
