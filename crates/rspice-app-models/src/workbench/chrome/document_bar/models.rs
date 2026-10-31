//! The document bar's Models tab: the workspace's pinned root document.

use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::state::WorkspaceDocumentId;

use crate::workbench::state::workbench_session::WorkspaceDocument;

/// The Models workspace's one document, its pinned root.
pub fn models_documents() -> Vec<WorkspaceDocument> {
    vec![WorkspaceDocument {
        id: WorkspaceDocumentId::Models,
        label: "Models & PDKs".to_owned(),
        icon: WorkbenchIcon::Models,
        dirty: false,
    }]
}
