//! Place in <schematic>: from a new Verilog-A device's first placeable
//! compile back to the schematic it was created from.
//!
//! New Verilog-A device… remembers the schematic tab it was opened from, and
//! the compile result offers to place the device there. That tab may have
//! been closed since; the design's top schematic is offered instead, and
//! nothing at all when the design has none to offer.

use crate::state::VerilogAPlacementHandoff;
use crate::state::{CellViewRef, ViewType};
use crate::workbench::app_state::Model;

/// Where `handoff`'s device is offered for placement: the schematic the form
/// was opened from while its tab is open, else the design's top schematic.
/// Never the device's own cell.
pub fn veriloga_placement_target(
    model: &Model,
    handoff: &VerilogAPlacementHandoff,
) -> Option<CellViewRef> {
    let schematic = |reference: &CellViewRef| {
        reference.cell != handoff.view.cell
            && model
                .library_manager
                .get_library(&reference.library)
                .and_then(|library| library.get_cell(&reference.cell))
                .and_then(|cell| cell.get_view(&reference.view))
                .is_some_and(|view| {
                    matches!(view.view_type, ViewType::Schematic | ViewType::Testbench)
                })
    };
    let open = |reference: &CellViewRef| {
        model
            .workspace
            .content
            .open_views
            .iter()
            .any(|open| &open.reference == reference)
    };
    let project = &model.workspace.content.project;
    let top = CellViewRef::new(
        &project.root_library,
        &project.top_cell,
        crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
    );
    handoff
        .schematic
        .clone()
        .filter(|remembered| open(remembered) && schematic(remembered))
        .or_else(|| schematic(&top).then_some(top))
}
