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

/// What one staged plan removal is about to take out of the plan.
///
/// Four registries, one review. Each variant carries the plan the review was
/// staged against as well as the record, so a confirmation answered while the
/// active plan changed underneath the modal applies to nothing rather than to
/// whatever now holds that position. The analysis stack is the exception only
/// because an analysis instance is identified plan-wide already.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PlanRemovalTarget {
    Analysis(crate::product::AnalysisInstanceId),
    Variable {
        plan: crate::product::SimulationPlanId,
        id: crate::product::DesignVariableId,
    },
    Output {
        plan: crate::product::SimulationPlanId,
        id: crate::product::SavedOutputId,
    },
    CaptureGroup {
        plan: crate::product::SimulationPlanId,
        id: crate::product::CaptureGroupId,
    },
}

/// How hard one consequence row's paragraph reads.
///
/// Not a color: `Warn` is what the reader loses and has to repair, `Aside` is
/// what survives removal and is said so they do not assume otherwise. A review
/// whose every line shouted would be a review nobody finished reading.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PlanRemovalTone {
    Warn,
    Aside,
}

/// One fact about what removal costs, resolved when the review opened.
///
/// The row is always stated — "none" is an answer a reader needs as much as a
/// list is — while the paragraph is only carried when there is something to
/// explain, which is what keeps a review of a cheap removal from reading like
/// a review of an expensive one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PlanRemovalConsequence {
    pub(crate) fact: String,
    pub(crate) value: String,
    pub(crate) note: Option<(PlanRemovalTone, String)>,
}

impl PlanRemovalConsequence {
    /// A stated fact with nothing further to explain.
    pub(crate) fn stated(fact: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            fact: fact.into(),
            value: value.into(),
            note: None,
        }
    }

    pub(crate) fn explained(mut self, tone: PlanRemovalTone, note: impl Into<String>) -> Self {
        self.note = Some((tone, note.into()));
        self
    }
}

/// Destructive review for removing one record from the analysis plan.
///
/// Only raised where removal actually costs something: an analysis with
/// retained results or dependents, a variable the plan resolves into an
/// analysis, an output a capture group holds or a specification names, a
/// capture group that holds outputs. Removing a freshly-added record that
/// nothing depends on is not a decision worth a modal, and asking anyway
/// teaches the reader to dismiss the dialog without reading it.
///
/// Plan mutations are not undoable from schematic history, which is exactly
/// why the four registries share one review instead of three of them
/// committing on the first click.
#[derive(Debug, Clone, Default)]
pub(crate) struct PlanRemovalReview {
    pub(crate) target: Option<PlanRemovalTarget>,
    /// What removal takes with it, resolved when the review opened.
    pub(crate) label: String,
    pub(crate) consequences: Vec<PlanRemovalConsequence>,
    /// Set by the dialog when the reader confirms. The page that owns the
    /// registry performs the removal itself on its next frame, so the modal
    /// never reaches across into another surface's transactions.
    pub(crate) confirmed: bool,
}

impl PlanRemovalReview {
    /// Stage a review for one record.
    pub(crate) fn open(
        &mut self,
        target: PlanRemovalTarget,
        label: String,
        consequences: Vec<PlanRemovalConsequence>,
    ) {
        *self = Self {
            target: Some(target),
            label,
            consequences,
            confirmed: false,
        };
    }

    /// The analysis the reader confirmed removing, taken exactly once.
    pub(crate) fn take_confirmed_analysis(&mut self) -> Option<crate::product::AnalysisInstanceId> {
        self.take_confirmed(|target| match target {
            PlanRemovalTarget::Analysis(id) => Some(id),
            _ => None,
        })
    }

    /// The design variable the reader confirmed removing from `plan`.
    pub(crate) fn take_confirmed_variable(
        &mut self,
        plan: crate::product::SimulationPlanId,
    ) -> Option<crate::product::DesignVariableId> {
        self.take_confirmed(|target| match target {
            PlanRemovalTarget::Variable { plan: staged, id } if staged == plan => Some(id),
            _ => None,
        })
    }

    /// The saved output the reader confirmed removing from `plan`.
    pub(crate) fn take_confirmed_output(
        &mut self,
        plan: crate::product::SimulationPlanId,
    ) -> Option<crate::product::SavedOutputId> {
        self.take_confirmed(|target| match target {
            PlanRemovalTarget::Output { plan: staged, id } if staged == plan => Some(id),
            _ => None,
        })
    }

    /// The capture group the reader confirmed removing from `plan`.
    pub(crate) fn take_confirmed_capture_group(
        &mut self,
        plan: crate::product::SimulationPlanId,
    ) -> Option<crate::product::CaptureGroupId> {
        self.take_confirmed(|target| match target {
            PlanRemovalTarget::CaptureGroup { plan: staged, id } if staged == plan => Some(id),
            _ => None,
        })
    }

    /// A confirmed answer of one kind, taken exactly once.
    ///
    /// The review is only closed when `extract` claims it. A registry that
    /// asked for a kind it did not stage must leave the answer standing for
    /// the registry that did, or one page reading the review would silently
    /// discard another page's confirmed removal.
    fn take_confirmed<T>(&mut self, extract: impl Fn(PlanRemovalTarget) -> Option<T>) -> Option<T> {
        let taken = self.target.filter(|_| self.confirmed).and_then(extract)?;
        self.close();
        Some(taken)
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}
