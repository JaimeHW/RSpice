//! What Project revision history keeps between frames, which the project
//! slot holds: the records it shows, the ones selected, their comparison
//! and the authority a restore is checked against. The dialog itself
//! draws in the project feature's context.

use std::sync::Arc;

use crate::state::{
    CellViewRef, ValidatedRevisionJournal, ValidatedRevisionSemanticDelta,
    ValidatedSchematicRevisionId,
};
use crate::ui::widgets::table::{RowKey, TableState};
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
    /// When the record was made, as a date and time; empty for a record
    /// that keeps no time.
    pub(crate) time: String,
    /// The same moment in milliseconds since the epoch, which the table
    /// sorts by.
    pub(crate) made_unix_ms: Option<u64>,
    /// How long ago that was when the rows were built, or why it cannot be
    /// said.
    pub(crate) age: String,
    pub(crate) actor: String,
    pub(crate) change: String,
    pub(crate) affected_evidence: String,
    pub(crate) status: String,
    pub(crate) schematic_revision: Option<ValidatedSchematicRevisionId>,
    /// The schematic view a revision belongs to, which restoring it opens.
    pub(crate) view: Option<CellViewRef>,
    /// The view's drawing is this revision's as it stands.
    pub(crate) current: bool,
}

/// What the rows were built from. The dialog is modal, so what moves while
/// it is open is the schematic in front, which a restore changes and opens,
/// and the project revision, which covers a governed change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ProjectAuditRowsSource {
    pub(crate) journal_len: usize,
    pub(crate) journal_head: Option<ValidatedSchematicRevisionId>,
    pub(crate) project_revision: u64,
    pub(crate) active_schematic_epoch: u64,
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
        /// The newer side is the view's drawing as it stands, not a
        /// revision.
        with_current: bool,
    },
    RetainedMetadata(ProjectAuditComparison),
}

/// A restore that opened its revision's view and waits for it to be in
/// front before it puts the revision back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PendingRestore {
    pub(crate) view: CellViewRef,
    pub(crate) revision: ValidatedSchematicRevisionId,
}

/// What the rows, the artifact filter and the search were when
/// [`ShownRows`] was worked out.
pub(crate) type ShownSource = (
    Option<ProjectAuditRowsSource>,
    ProjectRevisionArtifactFilter,
    String,
);

/// The rows the filter and the search show, by their place in the dialog's
/// rows, with their keys; the columns any row says something in, and how
/// wide each is to hold its widest words. Worked out when the rows, the
/// filter or the search change, never on an idle frame.
#[derive(Debug, Clone)]
pub(crate) struct ShownRows {
    pub(crate) source: Option<ShownSource>,
    pub(crate) rows: Arc<[usize]>,
    pub(crate) keys: Arc<[RowKey]>,
    pub(crate) columns: Arc<[usize]>,
    pub(crate) natural: Arc<[f32]>,
}

impl Default for ShownRows {
    fn default() -> Self {
        Self {
            source: None,
            rows: Arc::from(Vec::new()),
            keys: Arc::from(Vec::new()),
            columns: Arc::from(Vec::new()),
            natural: Arc::from(Vec::new()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProjectRevisionHistoryDialogState {
    pub open: bool,
    pub(crate) artifact_filter: ProjectRevisionArtifactFilter,
    pub(crate) query: String,
    /// The table's selection, sort and sideways scroll.
    pub(crate) table: TableState,
    pub(crate) comparison: Option<ProjectRevisionComparison>,
    pub(crate) authority: Option<SchematicEditAuthority>,
    pub(crate) expected_journal: ValidatedRevisionJournal,
    pub(crate) restore_confirmation: bool,
    pub(crate) pending_restore: Option<PendingRestore>,
    pub(crate) error: Option<String>,
    pub(crate) body_scroll_offset: f32,
    /// The table's rows, built when the dialog opens and again when what
    /// they were built from moves; never on an idle frame.
    pub(crate) rows: Arc<[ProjectAuditRow]>,
    pub(crate) rows_source: Option<ProjectAuditRowsSource>,
    pub(crate) shown: ShownRows,
}

impl Default for ProjectRevisionHistoryDialogState {
    fn default() -> Self {
        Self {
            open: false,
            artifact_filter: ProjectRevisionArtifactFilter::All,
            query: String::new(),
            table: TableState::default(),
            comparison: None,
            authority: None,
            expected_journal: ValidatedRevisionJournal::default(),
            restore_confirmation: false,
            pending_restore: None,
            error: None,
            body_scroll_offset: 0.0,
            rows: Arc::from(Vec::new()),
            rows_source: None,
            shown: ShownRows::default(),
        }
    }
}

impl ProjectRevisionHistoryDialogState {
    /// The keys of the rows the history stands on, in the order the table
    /// lists them.
    pub fn selected(&self) -> Vec<String> {
        let mut selected = self
            .table
            .selection
            .iter()
            .chain(&self.table.others)
            .filter_map(|selection| match &selection.key {
                Some(RowKey::Text(key)) => Some((selection.row, key.clone())),
                _ => None,
            })
            .collect::<Vec<_>>();
        selected.sort_by_key(|(row, _)| *row);
        selected.into_iter().map(|(_, key)| key).collect()
    }

    /// Stand the history on the record `key` names, alone. The table finds
    /// its row when it next draws; a key no row carries selects nothing.
    pub(crate) fn stand_on(&mut self, key: String) {
        self.table.select(0, Some(RowKey::Text(key)));
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}
