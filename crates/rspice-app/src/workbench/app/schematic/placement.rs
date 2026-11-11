//! Bind catalog placement payloads to the current app-owned document context.

use crate::state::{LibraryCellInstance, Tool};
use crate::workbench::app::dialogs::selection::array::ArraySelectionDialogState;
use crate::workbench::app::dialogs::selection::move_selection::MoveSelectionDialogState;
use crate::workbench::app::dialogs::selection::stretch::StretchSelectionDialogState;
use crate::workbench::app_state::{AppState, Model};
use crate::workbench::state::{WorkbenchSession, Workspace};
use rspice_app_state::workbench::app::schematic::placement::arm_cell_instance;

/// Arm a frozen cell binding and return the label used by placement receipts.
pub(crate) fn arm_library_cell_placement(
    state: &mut AppState,
    binding: LibraryCellInstance,
) -> String {
    arm_cell_instance(&mut state.model, binding)
}

/// Arm `tool` on the schematic in the Design workspace, closing whichever
/// selection transform (move, stretch or array) another tool left armed.
pub(crate) fn arm_schematic_tool(
    move_selection: &mut MoveSelectionDialogState,
    stretch_selection: &mut StretchSelectionDialogState,
    array_selection: &mut ArraySelectionDialogState,
    model: &mut Model,
    workbench: &mut WorkbenchSession,
    tool: Tool,
) {
    workbench.activate(Workspace::Design);
    if move_selection.armed && tool != Tool::MoveSelection {
        move_selection.close();
    }
    if stretch_selection.armed && tool != Tool::StretchSelection {
        stretch_selection.close();
    }
    if array_selection.armed && tool != Tool::ArraySelection {
        array_selection.close();
    }
    model.schematic.arm_tool(tool);
}
