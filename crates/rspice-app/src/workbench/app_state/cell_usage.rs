//! Where a library cell is placed: the schematic instances that bind it.
//!
//! The library's Where used report, the inspector's consumer count and the
//! Verilog-A navigator's Used in section all ask this one question, so it is
//! answered once. Every schematic of the project is in the workspace's
//! buffers; the one open in the editor is read from the editor instead,
//! because its buffer is only brought up to date when the view is left.

use rspice_design::schematic::document::SchematicDocument;

use crate::state::{CellViewRef, ViewType};
use crate::workbench::app_state::model::Model;

/// One placed instance of a library cell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellInstanceUse {
    /// The schematic the instance is placed in.
    pub schematic: CellViewRef,
    /// The instance's name in that schematic, `I3`.
    pub instance: String,
    /// The instance's component identity in that schematic.
    pub component_id: u64,
}

impl Model {
    /// Every instance in the project that binds `library`/`cell`, ordered by
    /// schematic and then by instance name.
    ///
    /// With `view` it counts only instances bound to that view. Without one it
    /// counts every view of the cell: an instance placed from a cell's symbol
    /// is still an instance of the cell its Verilog-A view describes.
    pub fn cell_instances(
        &self,
        library: &str,
        cell: &str,
        view: Option<&str>,
    ) -> Vec<CellInstanceUse> {
        let content = &self.workspace.content;
        let live = matches!(
            content.active_view_type(),
            ViewType::Schematic | ViewType::Testbench
        )
        .then(|| content.active_schematic_reference());
        let mut uses = Vec::new();
        if let Some(reference) = &live {
            collect(
                self.schematic.as_ref(),
                reference,
                (library, cell, view),
                &mut uses,
            );
        }
        for (key, schematic) in &content.schematic_buffers {
            let Some(reference) = reference_from_key(key) else {
                continue;
            };
            if live
                .as_ref()
                .is_some_and(|live| live.key().eq_ignore_ascii_case(key))
            {
                continue;
            }
            collect(
                schematic.as_ref(),
                &reference,
                (library, cell, view),
                &mut uses,
            );
        }
        uses.sort_by(|left, right| {
            (
                left.schematic.key().to_ascii_lowercase(),
                left.instance.to_ascii_lowercase(),
            )
                .cmp(&(
                    right.schematic.key().to_ascii_lowercase(),
                    right.instance.to_ascii_lowercase(),
                ))
        });
        uses
    }

    /// Take a removed cell out of every workspace collection that names it,
    /// and say what that left undecided.
    pub fn drop_workspace_references_to_cell(
        &mut self,
        library: &str,
        cell: &str,
    ) -> CellRemovalScope {
        let active_removed = self.workspace.content.active_view.library == library
            && self.workspace.content.active_view.cell == cell;
        let project_root_removed = self.workspace.content.project.root_library == library
            && self.workspace.content.project.top_cell == cell;
        let prefix = format!("{library}/{cell}/");
        self.workspace
            .content
            .schematic_buffers
            .retain(|key, _| !key.starts_with(&prefix));
        self.workspace
            .session
            .schematic_sessions
            .retain(|key, _| !key.starts_with(&prefix));
        let orphaned_layouts = self
            .workspace
            .content
            .physical_layout_documents()
            .values()
            .filter(|document| document.owner().library == library && document.owner().cell == cell)
            .map(|document| document.owner().clone())
            .collect::<Vec<_>>();
        for owner in orphaned_layouts {
            self.workspace
                .content
                .remove_physical_layout_document(&owner);
        }
        self.workspace
            .content
            .open_views
            .retain(|open| open.reference.library != library || open.reference.cell != cell);
        let libraries = &self.library_manager;
        for schematic in self.workspace.content.schematic_buffers.values_mut() {
            schematic.revalidate_instance_bindings(libraries.catalog());
        }
        let hierarchy_pruned = self
            .workspace
            .content
            .retain_valid_occurrences(|reference| {
                reference.library != library || reference.cell != cell
            });
        CellRemovalScope {
            active_removed,
            project_root_removed,
            hierarchy_pruned,
        }
    }
}

/// What taking one cell out of the workspace left undecided: the focus, the
/// project root, and the occurrence trail all name cell views, and any of the
/// three can have named the cell that just went.
pub struct CellRemovalScope {
    pub active_removed: bool,
    pub project_root_removed: bool,
    pub hierarchy_pruned: bool,
}

fn collect(
    document: &SchematicDocument,
    schematic: &CellViewRef,
    (library, cell, view): (&str, &str, Option<&str>),
    uses: &mut Vec<CellInstanceUse>,
) {
    for component in &document.components {
        let Some(binding) = component.library_cell.as_ref() else {
            continue;
        };
        if binding.library.eq_ignore_ascii_case(library)
            && binding.cell.eq_ignore_ascii_case(cell)
            && view.is_none_or(|view| binding.view.eq_ignore_ascii_case(view))
        {
            uses.push(CellInstanceUse {
                schematic: schematic.clone(),
                instance: component.name.clone(),
                component_id: component.id,
            });
        }
    }
}

/// The reference a `library/cell/view` buffer key names. Names cannot hold a
/// slash, so the key splits exactly.
fn reference_from_key(key: &str) -> Option<CellViewRef> {
    let mut parts = key.split('/');
    let (Some(library), Some(cell), Some(view), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return None;
    };
    Some(CellViewRef::new(library, cell, view))
}
