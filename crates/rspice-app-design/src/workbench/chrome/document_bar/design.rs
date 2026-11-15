//! The document bar's Design tabs: one per open cell view, each wearing its
//! view type's mark.

use crate::state::{CellViewRef, ViewType};
use crate::workbench::app::actions::workspace::NavigationParts;
use crate::workbench::app_state::Model;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::state::WorkspaceDocumentId;

use crate::workbench::state::workbench_session::WorkspaceDocument;

/// One tab per cell view open in Design.
pub(in crate::workbench) fn design_documents(model: &Model) -> Vec<WorkspaceDocument> {
    model
        .workspace
        .content
        .open_views
        .iter()
        .map(|document| WorkspaceDocument {
            id: WorkspaceDocumentId::CellView(document.reference.clone()),
            label: format!(
                "{} · {}",
                document.reference.cell,
                document.view_type.caption_for(&document.reference.view)
            ),
            icon: icon_for_view(document.view_type),
            dirty: document.dirty,
        })
        .collect()
}

/// Show a cell view that is open in Design.
pub(in crate::workbench) fn activate_cell_view(
    navigation: &mut NavigationParts<'_>,
    reference: &CellViewRef,
) -> bool {
    if navigation
        .model
        .workspace
        .content
        .open_views
        .iter()
        .any(|open| open.reference == *reference)
    {
        navigation.open_workspace_view(reference.clone());
        navigation.model.workspace.content.active_view == *reference
    } else {
        false
    }
}

pub(crate) const fn icon_for_view(view_type: ViewType) -> WorkbenchIcon {
    match view_type {
        ViewType::Schematic | ViewType::Testbench => WorkbenchIcon::Design,
        ViewType::Symbol => WorkbenchIcon::Models,
        // A Verilog-A module is edited in its own tab beside the schematic it
        // describes; borrowing the netlist mark made it read as a deck.
        ViewType::VerilogA => WorkbenchIcon::VerilogA,
        ViewType::Spice | ViewType::Verilog => WorkbenchIcon::Netlist,
        _ => WorkbenchIcon::File,
    }
}
