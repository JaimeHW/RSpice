//! Place in <schematic>: from a new Verilog-A device's first placeable
//! compile back to the schematic it was created from.
//!
//! New Verilog-A device… remembers the schematic tab it was opened from, and
//! the compile result offers to place the device there. That tab may have
//! been closed since; the design's top schematic is offered instead, and
//! nothing at all when the design has none to offer.

use crate::state::{CellViewRef, ViewType, library_cell_placement_candidates};
use crate::workbench::app_state::AppState;
use crate::workbench::documents::code_workspace::VerilogAPlacementHandoff;

impl AppState {
    /// Where `handoff`'s device is offered for placement: the schematic the
    /// form was opened from while its tab is open, else the design's top
    /// schematic. Never the device's own cell.
    pub(crate) fn veriloga_placement_target(
        &self,
        handoff: &VerilogAPlacementHandoff,
    ) -> Option<CellViewRef> {
        let schematic = |reference: &CellViewRef| {
            reference.cell != handoff.view.cell
                && self
                    .library_manager
                    .get_library(&reference.library)
                    .and_then(|library| library.get_cell(&reference.cell))
                    .and_then(|cell| cell.get_view(&reference.view))
                    .is_some_and(|view| {
                        matches!(view.view_type, ViewType::Schematic | ViewType::Testbench)
                    })
        };
        let open = |reference: &CellViewRef| {
            self.workspace
                .content
                .open_views
                .iter()
                .any(|open| &open.reference == reference)
        };
        let project = &self.workspace.content.project;
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

    /// The placement the open Verilog-A view's compile result offers: the
    /// device and where it would go.
    pub(crate) fn offered_veriloga_placement(
        &self,
    ) -> Option<(VerilogAPlacementHandoff, CellViewRef)> {
        let handoff = self
            .ui
            .code_workspace
            .veriloga
            .receipt
            .as_ref()?
            .placement
            .clone()?;
        let target = self.veriloga_placement_target(&handoff)?;
        Some((handoff, target))
    }

    /// Take the offered placement: switch to its schematic and arm the
    /// device there, as the shelf arms a cell. Returns the console line.
    pub(crate) fn place_offered_veriloga_device(&mut self) -> Result<String, String> {
        let (handoff, target) = self
            .offered_veriloga_placement()
            .ok_or_else(|| "No schematic is open to place the device in.".to_owned())?;
        let candidate = library_cell_placement_candidates(&self.library_manager, &self.workspace)
            .into_iter()
            .find(|candidate| {
                candidate.library == handoff.view.library
                    && candidate.cell == handoff.view.cell
                    && candidate.view == handoff.view.view
            })
            .ok_or_else(|| format!("{} is no longer a cell view.", handoff.view.display_path()))?;
        if !candidate.ready {
            return Err(format!(
                "{} cannot be placed yet: {}.",
                handoff.view.display_path(),
                candidate.unavailable_reason
            ));
        }
        if let Some(receipt) = self.ui.code_workspace.veriloga.receipt.as_mut() {
            receipt.placement = None;
        }
        self.open_workspace_view(target.clone());
        self.workbench
            .activate(crate::workbench::state::Workspace::Design);
        let label = crate::workbench::app::arm_library_cell_placement(self, candidate.binding);
        Ok(format!(
            "{label} is armed for placement in {}.",
            target.display_path()
        ))
    }
}
