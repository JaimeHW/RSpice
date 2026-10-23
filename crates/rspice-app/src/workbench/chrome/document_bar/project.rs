//! The document bar's Project tab: the workspace's one document, its
//! overview.

use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::state::WorkspaceDocumentId;

use super::WorkspaceDocument;

/// The Project workspace's one document, its overview.
pub(super) fn project_documents() -> Vec<WorkspaceDocument> {
    vec![WorkspaceDocument {
        id: WorkspaceDocumentId::Project,
        label: "Project overview".to_owned(),
        icon: WorkbenchIcon::Project,
        dirty: false,
    }]
}
