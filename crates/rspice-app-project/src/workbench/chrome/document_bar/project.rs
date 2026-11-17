//! The document bar's Project tab: the workspace's one document. Its pages
//! are the tabs under it, which name the page in front.

use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::state::WorkspaceDocumentId;

use crate::workbench::state::workbench_session::WorkspaceDocument;

/// The Project workspace's one document, the project.
pub fn project_documents() -> Vec<WorkspaceDocument> {
    vec![WorkspaceDocument {
        id: WorkspaceDocumentId::Project,
        label: "Project".to_owned(),
        icon: WorkbenchIcon::Project,
        dirty: false,
    }]
}
