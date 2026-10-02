//! Bind catalog placement payloads to the current app-owned document context.

use super::edit_authority::schematic_editor_request_source;
use crate::state::model_hub::PartPlacement;
use crate::state::{ComponentType, LibraryCellInstance, Tool};
use crate::workbench::app_state::AppState;
use rspice_schematic_editor::session::placement::{PendingLibraryCellPlacement, PendingPartModel};

/// Arm a frozen cell binding and return the label used by placement receipts.
pub(crate) fn arm_library_cell_placement(
    state: &mut AppState,
    binding: LibraryCellInstance,
) -> String {
    let label = format!("{}/{}", binding.library, binding.cell);
    state.schematic.session.editor.pending_library_cell = Some(PendingLibraryCellPlacement::new(
        binding,
        schematic_editor_request_source(state),
    ));
    state
        .schematic
        .arm_tool(Tool::Place(ComponentType::CellInstance));
    label
}

/// Coordinate either placement shape published by a distributed model pack.
pub(crate) fn arm_pack_part(state: &mut AppState, placement: PartPlacement) -> String {
    match placement {
        PartPlacement::CellInstance(binding) => arm_library_cell_placement(state, *binding),
        PartPlacement::NativeDevice {
            component_type,
            variant,
            model,
        } => {
            state.schematic.session.editor.pending_library_cell = None;
            state.schematic.arm_tool(Tool::Place(component_type));
            state.schematic.session.editor.pending_part_model = Some(PendingPartModel {
                tool: Tool::Place(component_type),
                model: model.clone(),
                variant,
            });
            model
        }
    }
}
