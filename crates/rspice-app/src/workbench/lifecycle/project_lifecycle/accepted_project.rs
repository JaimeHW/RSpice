//! Pair immutable accepted content with editor state and its platform binding.
//!
//! This owner stays on the UI thread. Binding/permission changes do not alter
//! accepted content; an acknowledged save or load constructs a new owner.

use std::rc::Rc;

use crate::io::ProjectSnapshot;
use crate::state::{SchematicState, workspace::WorkspaceSession};
use rspice_project::ProjectFile;

use super::PersistenceBinding;
use super::registry::DocumentFingerprints;

#[derive(Debug)]
struct AcceptedContent {
    project: rspice_project::AcceptedProject,
    workspace_session: WorkspaceSession,
}

#[derive(Debug, Clone)]
pub(crate) struct AcceptedProject {
    // Canonical content, fingerprints and the matching editor session share
    // one allocation. Drafts and save candidates receive explicit copies.
    content: Rc<AcceptedContent>,
    pub(crate) binding: Option<PersistenceBinding>,
}

impl AcceptedProject {
    pub(super) fn new(baseline: ProjectSnapshot, binding: Option<PersistenceBinding>) -> Self {
        let ProjectSnapshot {
            file,
            workspace_session,
        } = baseline;
        let project = rspice_project::AcceptedProject::new(file);
        Self {
            content: Rc::new(AcceptedContent {
                project,
                workspace_session,
            }),
            binding,
        }
    }

    pub(super) fn baseline(&self) -> &ProjectFile {
        self.content.project.baseline()
    }

    pub(super) fn fingerprints(&self) -> Result<&DocumentFingerprints, String> {
        self.content.project.fingerprints()
    }

    pub(super) fn clone_snapshot(&self) -> ProjectSnapshot {
        ProjectSnapshot {
            file: self.baseline().clone(),
            workspace_session: self.content.workspace_session.clone(),
        }
    }

    pub(super) fn clone_schematic_editor(&self, key: &str) -> Option<SchematicState> {
        self.content
            .workspace_session
            .clone_schematic_editor(&self.baseline().workspace, key)
    }
}
