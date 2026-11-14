//! What Project revision history keeps between frames, which the project
//! slot holds: the records it shows, the ones selected, their comparison
//! and the authority a restore is checked against. The dialog itself
//! draws in the project feature's context.

use crate::state::{ValidatedRevisionJournal, ValidatedRevisionSemanticDelta};
use crate::workbench::app::SchematicEditAuthority;

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
pub(crate) struct ProjectRevisionHistoryDialogState {
    pub(crate) open: bool,
    pub(crate) artifact_filter: ProjectRevisionArtifactFilter,
    pub(crate) query: String,
    pub(crate) selected: Vec<String>,
    pub(crate) comparison: Option<ProjectRevisionComparison>,
    pub(crate) authority: Option<SchematicEditAuthority>,
    pub(crate) expected_journal: ValidatedRevisionJournal,
    pub(crate) restore_confirmation: bool,
    pub(crate) error: Option<String>,
    pub(crate) body_scroll_offset: f32,
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
        }
    }
}

impl ProjectRevisionHistoryDialogState {
    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}
