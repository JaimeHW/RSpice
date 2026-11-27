//! Arm the schematic's placement cursor from a catalog placement payload: a
//! frozen cell binding, or either shape a library part places as. Each is an
//! operation on the model alone, so every feature that places a part arms it
//! the same way.

use rspice_schematic_editor::session::placement::{PendingLibraryCellPlacement, PendingPartModel};

use super::edit_authority::schematic_editor_request_source;
use crate::state::part_placement::PartPlacement;
use crate::state::{ComponentType, LibraryCellInstance, Tool};
use crate::workbench::app_state::model::Model;

/// Arm a frozen cell binding on the model's schematic.
pub fn arm_cell_instance(model: &mut Model, binding: LibraryCellInstance) -> String {
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

/// Arm either shape a library part places as: a cell instance over a
/// subcircuit's ports, or a native device carrying its card.
pub fn arm_part_placement(model: &mut Model, placement: PartPlacement) -> String {
    let label = placement.armed_label();
    match placement {
        PartPlacement::CellInstance(binding) => {
            arm_cell_instance(model, *binding);
        }
        PartPlacement::NativeDevice {
            component_type,
            variant,
            model: part_model,
        } => {
            model.schematic.session.editor.pending_library_cell = None;
            model.schematic.arm_tool(Tool::Place(component_type));
            model.schematic.session.editor.pending_part_model = Some(PendingPartModel {
                tool: Tool::Place(component_type),
                model: part_model,
                variant,
            });
        }
    }
    label
}
