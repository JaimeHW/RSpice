//! Destructive-review state for removing library objects and configured
//! analyses.
//!
//! These are not drafts. Each one names an object the reader is about to lose
//! and the exact revision or dependency evidence the review was staged
//! against, so a catalog that moved underneath the modal invalidates the
//! confirmation rather than deleting something else.

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LibraryDeletionTarget {
    Cell {
        library: String,
        cell: String,
    },
    View {
        library: String,
        cell: String,
        view: String,
    },
}

impl LibraryDeletionTarget {
    pub(crate) fn library(&self) -> &str {
        match self {
            Self::Cell { library, .. } | Self::View { library, .. } => library,
        }
    }

    pub(crate) fn cell(&self) -> &str {
        match self {
            Self::Cell { cell, .. } | Self::View { cell, .. } => cell,
        }
    }

    pub(crate) fn view(&self) -> Option<&str> {
        match self {
            Self::Cell { .. } => None,
            Self::View { view, .. } => Some(view),
        }
    }

    pub(crate) fn display_path(&self) -> String {
        match self {
            Self::Cell { library, cell } => format!("{library}/{cell}"),
            Self::View {
                library,
                cell,
                view,
            } => format!("{library}/{cell}/{view}"),
        }
    }

    pub(crate) const fn kind_label(&self) -> &'static str {
        match self {
            Self::Cell { .. } => "Cell",
            Self::View { .. } => "View",
        }
    }
}

/// What becomes of the placements of a library object that is being deleted.
///
/// Removing a master is a library decision; what the drawings that placed it
/// should do about it is a separate design decision, and one the reader has to
/// make rather than inherit. Nothing is deleted until they have.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum DeletionInstanceResolution {
    /// Leave every placement drawn where it is. Each keeps naming the master
    /// it was placed from and reads as unresolved until it is rebound or
    /// removed, which is what a design review needs to be able to see.
    #[default]
    KeepUnresolved,
    /// Take every placement of the object out of every drawing in the project,
    /// as a single step that can be undone as a single step.
    RemoveInstances,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct LibraryDeletionReviewState {
    pub(crate) target: Option<LibraryDeletionTarget>,
    pub(crate) expected_library_revision: u64,
    pub(crate) error: Option<String>,
    /// The choice the reader confirmed for the object's placements. Set when
    /// the deletion is staged and taken exactly once by the staged deletion,
    /// so the two halves of one decision cannot come apart.
    pub(crate) resolution: Option<DeletionInstanceResolution>,
}

impl LibraryDeletionReviewState {
    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}

/// Destructive review for removing one configured analysis.
///
/// Only raised where removal actually costs something: the analysis has
/// retained results attributed to it, or other analyses are bound to it as a
/// prerequisite. Removing a freshly-added instance that nothing depends on is
/// not a decision worth a modal, and asking anyway teaches the reader to
/// dismiss the dialog without reading it.
#[derive(Debug, Clone, Default)]
pub(crate) struct AnalysisRemovalReviewState {
    pub(crate) target: Option<crate::product::AnalysisInstanceId>,
    /// What removal takes with it, resolved when the review opened.
    pub(crate) label: String,
    pub(crate) retained_runs: usize,
    pub(crate) dependent_analyses: Vec<String>,
    /// Set by the dialog when the reader confirms. The Simulate surface owns
    /// the removal itself and performs it on the next frame, so the modal
    /// never reaches across into another surface's lifecycle.
    pub(crate) confirmed: bool,
}

impl AnalysisRemovalReviewState {
    /// Stage a review for one analysis.
    pub(crate) fn open(
        &mut self,
        target: crate::product::AnalysisInstanceId,
        label: String,
        retained_runs: usize,
        dependent_analyses: Vec<String>,
    ) {
        *self = Self {
            target: Some(target),
            label,
            retained_runs,
            dependent_analyses,
            confirmed: false,
        };
    }

    /// The analysis the reader confirmed removing, taken exactly once.
    pub(crate) fn take_confirmed(&mut self) -> Option<crate::product::AnalysisInstanceId> {
        let target = self.target.filter(|_| self.confirmed)?;
        self.close();
        Some(target)
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}
