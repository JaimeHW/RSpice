//! The exact occurrence each open document is editing.
//!
//! A document is not just a master: it is that master reached through a
//! definite chain of instances. Two tabs on two masters reached through
//! different parents are two different occurrences, so the chain belongs to
//! the tab and not to the session. Switching tabs then restores the
//! occurrence the tab was opened at instead of whatever the last navigation
//! happened to leave behind.
//!
//! One invariant holds the model together: [`DocumentOccurrence::terminal_master`]
//! is always the `reference` of the open document that owns the occurrence.
//! Every mutation here restates it with a debug assertion, and
//! [`DocumentOccurrence::debug_assert_opens`] states it at the owning tab.
//!
//! Nothing in this module invents a step. An occurrence that can no longer be
//! spelled truncates to the deepest prefix that can, because a fabricated
//! instance name addresses a different instance than the one on screen.

use serde::{Deserialize, Serialize};

use crate::state::{CellViewRef, InstancePath};

/// One descent: the instance stepped through, and the master it opened.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OccurrenceStep {
    pub instance_name: String,
    pub master: CellViewRef,
}

/// The occurrence one open document is editing: the design root it was opened
/// at, and the instances descended through since.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentOccurrence {
    pub root: CellViewRef,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub steps: Vec<OccurrenceStep>,
}

/// What pruning one occurrence against the surviving library left behind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OccurrencePrune {
    /// Every master still resolves; the occurrence is unchanged.
    Intact,
    /// Descents below a master that is gone were dropped, so the occurrence
    /// now opens an ancestor.
    Truncated,
    /// The root itself is gone, so there is no occurrence left to keep.
    Rootless,
}

impl Default for DocumentOccurrence {
    /// An unrooted occurrence, which only a tab record written before
    /// documents owned their occurrence can produce.
    /// [`DocumentOccurrence::is_unrooted`] recognizes it so migration can
    /// root it against the tab's own reference.
    fn default() -> Self {
        Self {
            root: CellViewRef::new("", "", ""),
            steps: Vec::new(),
        }
    }
}

impl DocumentOccurrence {
    /// A document opened as a design root: nothing was descended through to
    /// reach it.
    pub fn rooted(root: CellViewRef) -> Self {
        Self {
            root,
            steps: Vec::new(),
        }
    }

    /// Whether this record carries no root at all.
    pub fn is_unrooted(&self) -> bool {
        self.root.cell.is_empty()
    }

    /// The master this occurrence opens — the root while nothing has been
    /// descended through.
    pub fn terminal_master(&self) -> &CellViewRef {
        self.steps.last().map_or(&self.root, |step| &step.master)
    }

    /// Levels counting the root, which is the length the session-global
    /// breadcrumb had for the same occurrence.
    pub fn depth(&self) -> usize {
        self.steps.len() + 1
    }

    /// Breadcrumb labels: the root cell, then the instance stepped through at
    /// each level below it.
    pub fn labels(&self) -> Vec<String> {
        std::iter::once(self.root.cell.clone())
            .chain(self.steps.iter().map(|step| step.instance_name.clone()))
            .collect()
    }

    /// Every master on the occurrence, outermost first.
    pub fn masters(&self) -> impl Iterator<Item = &CellViewRef> {
        std::iter::once(&self.root).chain(self.steps.iter().map(|step| &step.master))
    }

    /// Every master on the occurrence, for a rename that moves one of them.
    pub fn masters_mut(&mut self) -> impl Iterator<Item = &mut CellViewRef> {
        let Self { root, steps } = self;
        std::iter::once(root).chain(steps.iter_mut().map(|step| &mut step.master))
    }

    /// This occurrence as an instance path.
    ///
    /// The design root is implicit, so the root cell owns no segment and only
    /// the instances below it do. A step the path grammar cannot name resolves
    /// to the root rather than to a truncated path, because a prefix of an
    /// occurrence addresses a different instance than the one on screen.
    pub fn instance_path(&self) -> InstancePath {
        let mut path = InstancePath::root();
        for step in &self.steps {
            let Ok(child) = path.child(&step.instance_name) else {
                return InstancePath::root();
            };
            path = child;
        }
        path
    }

    /// Descend into `instance_name`, whose master is `master`.
    pub fn descend(&mut self, instance_name: String, master: CellViewRef) {
        self.steps.push(OccurrenceStep {
            instance_name,
            master,
        });
        debug_assert!(
            self.steps
                .last()
                .is_some_and(|step| &step.master == self.terminal_master()),
            "a descent opens the master it just pushed"
        );
    }

    /// Keep `steps` descents, so `truncate_to(0)` is the root document.
    pub fn truncate_to(&mut self, steps: usize) {
        let before = self.depth();
        self.steps.truncate(steps);
        debug_assert_eq!(
            self.depth(),
            before.min(steps + 1),
            "truncation keeps exactly the requested prefix"
        );
    }

    /// Keep the deepest prefix whose masters all still resolve.
    pub fn retain_valid_prefix(
        &mut self,
        is_valid: impl Fn(&CellViewRef) -> bool,
    ) -> OccurrencePrune {
        if !is_valid(&self.root) {
            return OccurrencePrune::Rootless;
        }
        let keep = self
            .steps
            .iter()
            .position(|step| !is_valid(&step.master))
            .unwrap_or(self.steps.len());
        if keep == self.steps.len() {
            return OccurrencePrune::Intact;
        }
        self.truncate_to(keep);
        debug_assert!(
            self.masters().all(is_valid),
            "the retained prefix resolves at every level"
        );
        OccurrencePrune::Truncated
    }

    /// Restate the invariant this module exists to hold at the document that
    /// owns the occurrence.
    pub fn debug_assert_opens(&self, reference: &CellViewRef) {
        debug_assert_eq!(
            self.terminal_master(),
            reference,
            "an open document's occurrence must end at the document it shows"
        );
    }
}

#[cfg(test)]
mod tests;
