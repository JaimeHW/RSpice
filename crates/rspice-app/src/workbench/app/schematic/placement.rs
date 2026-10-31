//! Bind catalog placement payloads to the current app-owned document context.

use super::edit_authority::schematic_editor_request_source;
use crate::state::model_hub::PartPlacement;
use crate::state::{ComponentType, LibraryCellInstance, Tool};
use crate::workbench::app_state::{AppState, Model};
use rspice_schematic_editor::session::placement::{PendingLibraryCellPlacement, PendingPartModel};

/// Arm a frozen cell binding and return the label used by placement receipts.
pub(crate) fn arm_library_cell_placement(
    state: &mut AppState,
    binding: LibraryCellInstance,
) -> String {
    arm_cell_instance(&mut state.model, binding)
}

/// Arm a frozen cell binding on the model's schematic.
fn arm_cell_instance(model: &mut Model, binding: LibraryCellInstance) -> String {
    let label = format!("{}/{}", binding.library, binding.cell);
    model.schematic.session.editor.pending_library_cell = Some(PendingLibraryCellPlacement::new(
        binding,
        schematic_editor_request_source(model),
    ));
    model
        .schematic
        .arm_tool(Tool::Place(ComponentType::CellInstance));
    label
}

/// Coordinate either placement shape published by a distributed model pack.
pub(crate) fn arm_pack_part(model: &mut Model, placement: PartPlacement) -> String {
    match placement {
        PartPlacement::CellInstance(binding) => arm_cell_instance(model, *binding),
        PartPlacement::NativeDevice {
            component_type,
            variant,
            model: part_model,
        } => {
            model.schematic.session.editor.pending_library_cell = None;
            model.schematic.arm_tool(Tool::Place(component_type));
            model.schematic.session.editor.pending_part_model = Some(PendingPartModel {
                tool: Tool::Place(component_type),
                model: part_model.clone(),
                variant,
            });
            part_model
        }
    }
}
