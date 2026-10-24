//! The document bar's Simulate and Verify tabs: each workspace is one
//! document.

use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::state::WorkspaceDocumentId;

use super::WorkspaceDocument;

/// The Simulate workspace's one document, its plan.
pub(in crate::workbench) fn simulation_plan_documents() -> Vec<WorkspaceDocument> {
    // One plan, one document. The Studio used to open a "<name> · setup"
    // tab per analysis instance too, and activating one only selected that
    // instance on the Analyses page — which the plan's own analysis list
    // does, permanently, on the left. A plan with seven analyses therefore
    // paid a strip of eight tabs for nothing the list did not already say,
    // so the strip is gone: `document_strip_visible` hides a lone
    // non-Design document.
    vec![WorkspaceDocument {
        id: WorkspaceDocumentId::SimulationPlan,
        label: "Simulation plan".to_owned(),
        icon: WorkbenchIcon::Simulate,
        dirty: false,
    }]
}

/// The Verify workspace's one document.
pub(in crate::workbench) fn verification_documents() -> Vec<WorkspaceDocument> {
    vec![WorkspaceDocument {
        id: WorkspaceDocumentId::Verification,
        label: "Verification".to_owned(),
        icon: WorkbenchIcon::Verify,
        dirty: false,
    }]
}
