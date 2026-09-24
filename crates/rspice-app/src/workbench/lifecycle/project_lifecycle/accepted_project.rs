//! Immutable accepted content and the fingerprints of that exact content.
//!
//! This owner stays on the UI thread. Binding/permission changes do not alter
//! accepted content; an acknowledged save or load constructs a new owner.

use std::rc::Rc;

use crate::io::ProjectFile;

use super::PersistenceBinding;
use super::registry::DocumentFingerprints;

#[derive(Debug)]
struct AcceptedContent {
    baseline: ProjectFile,
    fingerprints: Result<DocumentFingerprints, String>,
}

#[derive(Debug, Clone)]
pub(crate) struct AcceptedProject {
    // There is intentionally no mutable accessor. Drafts and save candidates
    // get an explicit copy, so neither can invalidate cached accepted content.
    content: Rc<AcceptedContent>,
    pub(crate) binding: Option<PersistenceBinding>,
}

impl AcceptedProject {
    pub(super) fn new(baseline: ProjectFile, binding: Option<PersistenceBinding>) -> Self {
        let fingerprints = DocumentFingerprints::new(&baseline);
        Self {
            content: Rc::new(AcceptedContent {
                baseline,
                fingerprints,
            }),
            binding,
        }
    }

    pub(super) fn baseline(&self) -> &ProjectFile {
        &self.content.baseline
    }

    pub(super) fn fingerprints(&self) -> Result<&DocumentFingerprints, String> {
        self.content.fingerprints.as_ref().map_err(Clone::clone)
    }
}
