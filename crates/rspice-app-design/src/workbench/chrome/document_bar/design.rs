//! The document bar's Design tabs: one per open cell view, each wearing its
//! view type's mark.

use crate::state::{CellViewRef, ViewType};
use crate::workbench::app::actions::workspace::NavigationParts;
use crate::workbench::app_state::Model;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::state::WorkspaceDocumentId;

use crate::workbench::state::workbench_session::WorkspaceDocument;

/// One tab per cell view open in Design, named by its cell and view. Tabs
/// that would read the same, one cell's name in two libraries, name the
/// library too, so no two tabs read alike. A project whose simulation is its
/// deck and that has no schematic has no schematic tab: the empty top cell
/// it keeps is not a document anyone drew.
pub fn design_documents(model: &Model) -> Vec<WorkspaceDocument> {
    let no_schematic = model.is_netlist_first_without_schematic();
    let open = model
        .workspace
        .content
        .open_views
        .iter()
        .filter(|document| {
            !(no_schematic
                && matches!(
                    document.view_type,
                    ViewType::Schematic | ViewType::Testbench
                ))
        })
        .collect::<Vec<_>>();
    let short = |document: &crate::state::OpenCellView| {
        format!(
            "{} · {}",
            document.reference.cell,
            document.view_type.caption_for(&document.reference.view)
        )
    };
    let labels = open.iter().copied().map(short).collect::<Vec<_>>();
    open.iter()
        .zip(&labels)
        .map(|(document, label)| WorkspaceDocument {
            id: WorkspaceDocumentId::CellView(document.reference.clone()),
            label: if labels.iter().filter(|other| *other == label).count() > 1 {
                format!("{}/{label}", document.reference.library)
            } else {
                label.clone()
            },
            icon: icon_for_view(document.view_type),
            dirty: document.dirty,
        })
        .collect()
}

/// Show a cell view that is open in Design.
pub fn activate_cell_view(navigation: &mut NavigationParts<'_>, reference: &CellViewRef) -> bool {
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

pub const fn icon_for_view(view_type: ViewType) -> WorkbenchIcon {
    match view_type {
        ViewType::Schematic | ViewType::Testbench => WorkbenchIcon::Design,
        ViewType::Symbol => WorkbenchIcon::Models,
        // A Verilog-A module is edited in its own tab beside the schematic it
        // describes; borrowing the netlist mark made it read as a deck.
        // Verilog-AMS carries the same analog behaviour; a digital Verilog
        // module is a structural netlist, so it keeps the deck's mark.
        ViewType::VerilogA | ViewType::VerilogAms => WorkbenchIcon::VerilogA,
        ViewType::Spice | ViewType::Verilog => WorkbenchIcon::Netlist,
        _ => WorkbenchIcon::File,
    }
}
