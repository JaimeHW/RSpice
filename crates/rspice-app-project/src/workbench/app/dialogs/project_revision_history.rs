//! What Project revision history keeps between frames, which the project
//! slot holds: the records it shows, the ones selected, their comparison
//! and the authority a restore is checked against. The dialog itself
//! draws in the project feature's context.

use std::sync::Arc;

use crate::state::{
    ValidatedRevisionJournal, ValidatedRevisionSemanticDelta, ValidatedSchematicRevisionId,
};
use crate::workbench::app::SchematicEditAuthority;

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Times this thread built the history table's rows.
    pub static AUDIT_ROW_BUILDS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// One row of the history table: a retained record as the dialog shows it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProjectAuditRow {
    pub(crate) key: String,
    pub(crate) kind: ProjectAuditArtifactKind,
    pub(crate) revision: String,
    pub(crate) time: String,
    pub(crate) actor: String,
    pub(crate) change: String,
    pub(crate) affected_evidence: String,
    pub(crate) status: String,
    pub(crate) schematic_revision: Option<ValidatedSchematicRevisionId>,
    pub(crate) restorable_schematic_revision: Option<ValidatedSchematicRevisionId>,
}

/// What the rows were built from. The dialog is modal, so the one record
/// that moves while it is open is the active schematic's journal, which a
/// restore extends; the project revision covers a governed change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ProjectAuditRowsSource {
    pub(crate) journal_len: usize,
    pub(crate) journal_head: Option<ValidatedSchematicRevisionId>,
    pub(crate) project_revision: u64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum ProjectRevisionArtifactFilter {
    #[default]
    All,
    Schematic,
    SimulationPlans,
    ModelsAndPdk,
    GovernedRecords,
}

impl ProjectRevisionArtifactFilter {
    pub(crate) const ALL: [Self; 5] = [
        Self::All,
        Self::Schematic,
        Self::SimulationPlans,
        Self::ModelsAndPdk,
        Self::GovernedRecords,
    ];

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::All => "All project artifacts",
            Self::Schematic => "Schematic",
            Self::SimulationPlans => "Simulation plans",
            Self::ModelsAndPdk => "Models and PDK",
            Self::GovernedRecords => "Governed records",
        }
    }

    pub(crate) const fn accepts(self, kind: ProjectAuditArtifactKind) -> bool {
        match self {
            Self::All => true,
            Self::Schematic => matches!(kind, ProjectAuditArtifactKind::Schematic),
            Self::SimulationPlans => {
                matches!(kind, ProjectAuditArtifactKind::SimulationPlan)
            }
            Self::ModelsAndPdk => matches!(kind, ProjectAuditArtifactKind::ModelOrPdk),
            Self::GovernedRecords => matches!(kind, ProjectAuditArtifactKind::GovernedRecord),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProjectAuditArtifactKind {
    Schematic,
    SimulationPlan,
    ModelOrPdk,
    GovernedRecord,
}

impl ProjectAuditArtifactKind {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Schematic => "Schematic",
            Self::SimulationPlan => "Simulation plan",
            Self::ModelOrPdk => "Model / PDK",
            Self::GovernedRecord => "Governed record",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ProjectAuditComparison {
    pub(crate) label: String,
    pub(crate) fields: Vec<(&'static str, String, String)>,
}

#[derive(Debug, Clone)]
pub(crate) enum ProjectRevisionComparison {
    Schematic {
        label: String,
        delta: ValidatedRevisionSemanticDelta,
    },
    RetainedMetadata(ProjectAuditComparison),
}

#[derive(Debug, Clone)]
pub struct ProjectRevisionHistoryDialogState {
    pub open: bool,
    pub(crate) artifact_filter: ProjectRevisionArtifactFilter,
    pub(crate) query: String,
    pub(crate) selected: Vec<String>,
    pub(crate) comparison: Option<ProjectRevisionComparison>,
    pub(crate) authority: Option<SchematicEditAuthority>,
    pub(crate) expected_journal: ValidatedRevisionJournal,
    pub(crate) restore_confirmation: bool,
    pub(crate) error: Option<String>,
    pub(crate) body_scroll_offset: f32,
    /// The table's rows, built when the dialog opens and again when what
    /// they were built from moves; never on an idle frame.
    pub(crate) rows: Arc<[ProjectAuditRow]>,
    pub(crate) rows_source: Option<ProjectAuditRowsSource>,
}

impl Default for ProjectRevisionHistoryDialogState {
    fn default() -> Self {
        Self {
            open: false,
            artifact_filter: ProjectRevisionArtifactFilter::All,
            query: String::new(),
            selected: Vec::new(),
            comparison: None,
            authority: None,
            expected_journal: ValidatedRevisionJournal::default(),
            restore_confirmation: false,
            error: None,
            body_scroll_offset: 0.0,
            rows: Arc::from(Vec::new()),
            rows_source: None,
        }
    }
}

impl ProjectRevisionHistoryDialogState {
    /// The keys of the rows the history stands on.
    pub fn selected(&self) -> &[String] {
        &self.selected
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}
