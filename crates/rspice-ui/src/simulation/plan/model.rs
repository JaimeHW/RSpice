//! The analysis plan model.
//!
//! A persisted, revisioned plan: its analyses, their dependencies, and the
//! outputs it saves. The revision is what lets a run receipt name the exact
//! plan it executed.

mod dependencies;

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::product::{AnalysisInstanceId, ObjectRevision, RevisionError, RunId, SimulationPlanId};

use super::config::{
    AnalysisDependencyRepairContext, DependencyConfigurationIssue,
    dependency_candidate_context_issue, dependency_configuration_issue, prerequisite_draft_for,
};
use super::{AnalysisDraft, AnalysisKind};

/// Explicit, typed dependency from an analysis to one prerequisite instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalysisDependency {
    prerequisite: AnalysisKind,
    target: AnalysisInstanceId,
}

impl AnalysisDependency {
    #[must_use]
    pub const fn new(prerequisite: AnalysisKind, target: AnalysisInstanceId) -> Self {
        Self {
            prerequisite,
            target,
        }
    }

    #[must_use]
    pub const fn prerequisite(self) -> AnalysisKind {
        self.prerequisite
    }

    #[must_use]
    pub const fn target(self) -> AnalysisInstanceId {
        self.target
    }
}

/// Exact outcome of one atomic prerequisite-repair command.
///
/// Existing identities are reused whenever possible. Fresh identities are
/// reported only for prerequisite kinds that were absent from the plan, and
/// every list follows the final dependency order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisDependencyRepair {
    dependent: AnalysisInstanceId,
    inserted: Vec<AnalysisInstanceId>,
    enabled: Vec<AnalysisInstanceId>,
    moved: Vec<AnalysisInstanceId>,
    bound: Vec<AnalysisDependency>,
    removed: Vec<AnalysisDependency>,
}

impl AnalysisDependencyRepair {
    fn new(dependent: AnalysisInstanceId) -> Self {
        Self {
            dependent,
            inserted: Vec::new(),
            enabled: Vec::new(),
            moved: Vec::new(),
            bound: Vec::new(),
            removed: Vec::new(),
        }
    }

    #[must_use]
    #[cfg(test)]
    pub const fn dependent(&self) -> AnalysisInstanceId {
        self.dependent
    }

    #[must_use]
    pub fn inserted(&self) -> &[AnalysisInstanceId] {
        &self.inserted
    }

    #[must_use]
    pub fn enabled(&self) -> &[AnalysisInstanceId] {
        &self.enabled
    }

    #[must_use]
    pub fn moved(&self) -> &[AnalysisInstanceId] {
        &self.moved
    }

    #[must_use]
    pub fn bound(&self) -> &[AnalysisDependency] {
        &self.bound
    }

    #[must_use]
    pub fn removed(&self) -> &[AnalysisDependency] {
        &self.removed
    }

    #[must_use]
    #[cfg(test)]
    pub fn changed(&self) -> bool {
        !self.inserted.is_empty()
            || !self.enabled.is_empty()
            || !self.moved.is_empty()
            || !self.bound.is_empty()
            || !self.removed.is_empty()
    }

    fn sort_in_final_order(&mut self, instances: &[AnalysisInstance]) {
        let positions = instances
            .iter()
            .enumerate()
            .map(|(position, instance)| (instance.id, position))
            .collect::<HashMap<_, _>>();
        self.inserted.sort_by_key(|id| positions.get(id).copied());
        self.enabled.sort_by_key(|id| positions.get(id).copied());
        self.moved.sort_by_key(|id| positions.get(id).copied());
        self.bound
            .sort_by_key(|dependency| positions.get(&dependency.target).copied());
        self.removed
            .sort_by_key(|dependency| positions.get(&dependency.target).copied());
    }
}

/// Editable-plan lifecycle of one analysis instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AnalysisLifecycleState {
    Absent,
    Draft,
    Invalid,
    Ready,
    PreflightReady,
    Blocked,
    Queued,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
    Disabled,
    Removed,
    SameState,
}

impl AnalysisLifecycleState {
    const fn is_executing(self) -> bool {
        matches!(self, Self::Queued | Self::Running | Self::Paused)
    }
}

impl fmt::Display for AnalysisLifecycleState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Absent => "absent",
            Self::Draft => "draft",
            Self::Invalid => "invalid",
            Self::Ready => "ready",
            Self::PreflightReady => "preflight-ready",
            Self::Blocked => "blocked",
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Paused => "paused",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::Disabled => "disabled",
            Self::Removed => "removed",
            Self::SameState => "same-state",
        })
    }
}

/// Stable command vocabulary recorded by plan mutation receipts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AnalysisLifecycleCommand {
    Insert,
    Edit,
    Clone,
    Enable,
    Disable,
    Reorder,
    Dependency,
    Validate,
    Preflight,
    Execute,
    Remove,
}

impl fmt::Display for AnalysisLifecycleCommand {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Insert => "insert",
            Self::Edit => "edit",
            Self::Clone => "clone",
            Self::Enable => "enable",
            Self::Disable => "disable",
            Self::Reorder => "reorder",
            Self::Dependency => "dependency",
            Self::Validate => "validate",
            Self::Preflight => "preflight",
            Self::Execute => "execute",
            Self::Remove => "remove",
        })
    }
}

/// Immutable receipt proving one atomic plan mutation committed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalysisLifecycleReceipt {
    sequence: u64,
    command: AnalysisLifecycleCommand,
    instance_id: AnalysisInstanceId,
    kind: AnalysisKind,
    outcome: AnalysisLifecycleState,
    related_instance_id: Option<AnalysisInstanceId>,
    source_revision: ObjectRevision,
    committed_revision: ObjectRevision,
    detail: String,
}

impl AnalysisLifecycleReceipt {
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub const fn command(&self) -> AnalysisLifecycleCommand {
        self.command
    }

    #[must_use]
    pub const fn instance_id(&self) -> AnalysisInstanceId {
        self.instance_id
    }

    #[must_use]
    #[cfg(test)]
    pub const fn kind(&self) -> AnalysisKind {
        self.kind
    }

    #[must_use]
    pub const fn outcome(&self) -> AnalysisLifecycleState {
        self.outcome
    }

    #[must_use]
    pub const fn related_instance_id(&self) -> Option<AnalysisInstanceId> {
        self.related_instance_id
    }

    #[must_use]
    pub const fn source_revision(&self) -> ObjectRevision {
        self.source_revision
    }

    #[must_use]
    pub const fn committed_revision(&self) -> ObjectRevision {
        self.committed_revision
    }

    #[must_use]
    pub fn detail(&self) -> &str {
        &self.detail
    }
}

/// Immutable receipt proving that a non-analysis plan configuration domain
/// committed against one exact plan revision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationPlanConfigurationReceipt {
    sequence: u64,
    source_revision: ObjectRevision,
    committed_revision: ObjectRevision,
    detail: String,
}

impl SimulationPlanConfigurationReceipt {
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub const fn source_revision(&self) -> ObjectRevision {
        self.source_revision
    }

    #[must_use]
    pub const fn committed_revision(&self) -> ObjectRevision {
        self.committed_revision
    }

    #[must_use]
    pub fn detail(&self) -> &str {
        &self.detail
    }
}

/// One stable analysis identity in presentation and execution order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalysisInstance {
    id: AnalysisInstanceId,
    kind: AnalysisKind,
    draft: AnalysisDraft,
    enabled: bool,
    dependencies: Vec<AnalysisDependency>,
    lifecycle: AnalysisLifecycleState,
    created_revision: ObjectRevision,
    modified_revision: ObjectRevision,
}

impl AnalysisInstance {
    fn fresh(
        id: AnalysisInstanceId,
        draft: AnalysisDraft,
        enabled: bool,
        dependencies: Vec<AnalysisDependency>,
        revision: ObjectRevision,
    ) -> Self {
        Self {
            id,
            kind: draft.kind(),
            draft,
            enabled,
            dependencies,
            lifecycle: if enabled {
                AnalysisLifecycleState::Draft
            } else {
                AnalysisLifecycleState::Disabled
            },
            created_revision: revision,
            modified_revision: revision,
        }
    }

    /// Build a supplied-ID instance for deterministic migration.
    ///
    /// Whole-plan graph constraints are checked by
    /// [`SimulationPlan::from_ordered_instances`].
    pub fn supplied(
        id: AnalysisInstanceId,
        kind: AnalysisKind,
        draft: AnalysisDraft,
        enabled: bool,
        dependencies: Vec<AnalysisDependency>,
        created_revision: ObjectRevision,
        modified_revision: ObjectRevision,
    ) -> Result<Self, AnalysisPlanError> {
        if kind != draft.kind() {
            return Err(AnalysisPlanError::DraftKindMismatch {
                expected: kind,
                actual: draft.kind(),
            });
        }
        if modified_revision < created_revision {
            return Err(AnalysisPlanError::InvalidInstanceRevision { id });
        }
        Ok(Self {
            id,
            kind,
            draft,
            enabled,
            dependencies,
            lifecycle: if enabled {
                AnalysisLifecycleState::Draft
            } else {
                AnalysisLifecycleState::Disabled
            },
            created_revision,
            modified_revision,
        })
    }

    #[must_use]
    pub const fn id(&self) -> AnalysisInstanceId {
        self.id
    }

    #[must_use]
    pub const fn kind(&self) -> AnalysisKind {
        self.kind
    }

    #[must_use]
    pub const fn draft(&self) -> &AnalysisDraft {
        &self.draft
    }

    #[must_use]
    pub const fn enabled(&self) -> bool {
        self.enabled
    }

    #[must_use]
    pub fn dependencies(&self) -> &[AnalysisDependency] {
        &self.dependencies
    }

    /// Exact dependency roles required by this instance's current draft.
    #[must_use]
    pub fn prerequisite_roles(&self) -> &'static [AnalysisKind] {
        self.draft.prerequisite_roles()
    }

    #[must_use]
    pub const fn lifecycle(&self) -> AnalysisLifecycleState {
        self.lifecycle
    }

    #[must_use]
    pub const fn created_revision(&self) -> ObjectRevision {
        self.created_revision
    }

    #[must_use]
    pub const fn modified_revision(&self) -> ObjectRevision {
        self.modified_revision
    }
}

/// Durable identity record retained after an analysis is removed.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalysisTombstone {
    id: AnalysisInstanceId,
    kind: AnalysisKind,
    created_revision: ObjectRevision,
    last_modified_revision: ObjectRevision,
    removed_revision: ObjectRevision,
    prior_run_ids: Vec<RunId>,
}

impl AnalysisTombstone {
    #[must_use]
    pub const fn id(&self) -> AnalysisInstanceId {
        self.id
    }

    #[must_use]
    pub const fn kind(&self) -> AnalysisKind {
        self.kind
    }

    #[must_use]
    pub const fn created_revision(&self) -> ObjectRevision {
        self.created_revision
    }

    #[must_use]
    pub const fn removed_revision(&self) -> ObjectRevision {
        self.removed_revision
    }

    #[must_use]
    pub fn prior_run_ids(&self) -> &[RunId] {
        &self.prior_run_ids
    }
}

/// Deterministic validation diagnostic for a plan or supplied migration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum AnalysisPlanIssue {
    NoEnabledInstances,
    DuplicateInstanceId {
        id: AnalysisInstanceId,
    },
    DuplicateTombstoneId {
        id: AnalysisInstanceId,
    },
    ReusedTombstonedId {
        id: AnalysisInstanceId,
    },
    KindDraftMismatch {
        id: AnalysisInstanceId,
        expected: AnalysisKind,
        actual: AnalysisKind,
    },
    InvalidInstanceRevision {
        id: AnalysisInstanceId,
    },
    InvalidLifecycle {
        id: AnalysisInstanceId,
        state: AnalysisLifecycleState,
        enabled: bool,
    },
    MissingPrerequisite {
        dependent: AnalysisInstanceId,
        prerequisite: AnalysisKind,
    },
    UnexpectedDependencyRole {
        dependent: AnalysisInstanceId,
        prerequisite: AnalysisKind,
    },
    DuplicateDependencyRole {
        dependent: AnalysisInstanceId,
        prerequisite: AnalysisKind,
    },
    SelfDependency {
        dependent: AnalysisInstanceId,
    },
    DanglingDependency {
        dependent: AnalysisInstanceId,
        target: AnalysisInstanceId,
    },
    WrongDependencyKind {
        dependent: AnalysisInstanceId,
        prerequisite: AnalysisKind,
        target: AnalysisInstanceId,
        actual: AnalysisKind,
    },
    DisabledDependency {
        dependent: AnalysisInstanceId,
        target: AnalysisInstanceId,
    },
    DependencyNotEarlier {
        dependent: AnalysisInstanceId,
        target: AnalysisInstanceId,
    },
    IncompatibleDependencyConfiguration {
        dependent: AnalysisInstanceId,
        prerequisite: AnalysisKind,
        target: AnalysisInstanceId,
        detail: String,
    },
    DependencyCycle {
        members: Vec<AnalysisInstanceId>,
    },
    InvalidTombstoneRevision {
        id: AnalysisInstanceId,
    },
    InvalidReceiptSequence {
        sequence: u64,
    },
    InvalidReceiptRevision {
        sequence: u64,
    },
    DanglingReceiptInstance {
        sequence: u64,
        id: AnalysisInstanceId,
    },
    ReceiptKindMismatch {
        sequence: u64,
        expected: AnalysisKind,
        actual: AnalysisKind,
    },
    EmptyReceiptDetail {
        sequence: u64,
    },
    InvalidNextReceiptSequence {
        expected: u64,
        actual: u64,
    },
}

impl AnalysisPlanIssue {
    const fn is_structural(&self) -> bool {
        !matches!(
            self,
            Self::NoEnabledInstances
                | Self::MissingPrerequisite { .. }
                | Self::IncompatibleDependencyConfiguration { .. }
        )
    }
}

impl fmt::Display for AnalysisPlanIssue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoEnabledInstances => {
                formatter.write_str("Enable at least one analysis instance.")
            }
            Self::DuplicateInstanceId { id } => {
                write!(formatter, "Analysis identity {id} appears more than once.")
            }
            Self::DuplicateTombstoneId { id } => {
                write!(
                    formatter,
                    "Retired analysis identity {id} appears more than once."
                )
            }
            Self::ReusedTombstonedId { id } => write!(
                formatter,
                "Analysis identity {id} was retired and cannot be reused."
            ),
            Self::KindDraftMismatch {
                id,
                expected,
                actual,
            } => write!(
                formatter,
                "Analysis {id} is declared as {expected} but contains an {actual} draft."
            ),
            Self::InvalidInstanceRevision { id } => write!(
                formatter,
                "Analysis {id} has an invalid creation or modification revision."
            ),
            Self::InvalidLifecycle { id, state, enabled } => {
                let setting = if *enabled { "enabled" } else { "disabled" };
                write!(
                    formatter,
                    "Analysis {id} has lifecycle state {state}, which is inconsistent with its {setting} setting."
                )
            }
            Self::MissingPrerequisite {
                dependent,
                prerequisite,
            } => write!(
                formatter,
                "Analysis {dependent} has no bound {prerequisite} prerequisite."
            ),
            Self::UnexpectedDependencyRole {
                dependent,
                prerequisite,
            } => write!(
                formatter,
                "Analysis {dependent} does not accept a {prerequisite} prerequisite."
            ),
            Self::DuplicateDependencyRole {
                dependent,
                prerequisite,
            } => write!(
                formatter,
                "Analysis {dependent} binds the {prerequisite} prerequisite more than once."
            ),
            Self::SelfDependency { dependent } => {
                write!(formatter, "Analysis {dependent} cannot depend on itself.")
            }
            Self::DanglingDependency { dependent, target } => write!(
                formatter,
                "Analysis {dependent} references missing prerequisite {target}."
            ),
            Self::WrongDependencyKind {
                dependent,
                prerequisite,
                target,
                actual,
            } => write!(
                formatter,
                "Analysis {dependent} binds {target} as {prerequisite}, but that target is {actual}."
            ),
            Self::DisabledDependency { dependent, target } => write!(
                formatter,
                "Analysis {dependent} requires enabled prerequisite {target}."
            ),
            Self::DependencyNotEarlier { dependent, target } => write!(
                formatter,
                "Analysis {dependent} must follow prerequisite {target}."
            ),
            Self::IncompatibleDependencyConfiguration {
                dependent,
                prerequisite,
                target,
                detail,
            } => write!(
                formatter,
                "Analysis {dependent} cannot use {prerequisite} prerequisite {target}: {detail}."
            ),
            Self::DependencyCycle { members } => {
                formatter.write_str("The analysis dependency graph contains a cycle involving ")?;
                for (index, member) in members.iter().enumerate() {
                    if index > 0 {
                        formatter.write_str(", ")?;
                    }
                    write!(formatter, "{member}")?;
                }
                formatter.write_str(".")
            }
            Self::InvalidTombstoneRevision { id } => write!(
                formatter,
                "Retired analysis {id} has an invalid revision history."
            ),
            Self::InvalidReceiptSequence { sequence } => write!(
                formatter,
                "Analysis lifecycle receipt sequence {sequence} is out of order."
            ),
            Self::InvalidReceiptRevision { sequence } => write!(
                formatter,
                "Analysis lifecycle receipt {sequence} has an invalid revision transition."
            ),
            Self::DanglingReceiptInstance { sequence, id } => write!(
                formatter,
                "Analysis lifecycle receipt {sequence} references unknown analysis {id}."
            ),
            Self::ReceiptKindMismatch {
                sequence,
                expected,
                actual,
            } => write!(
                formatter,
                "Analysis lifecycle receipt {sequence} identifies {actual}, but its retained analysis is {expected}."
            ),
            Self::EmptyReceiptDetail { sequence } => write!(
                formatter,
                "Analysis lifecycle receipt {sequence} has no status detail."
            ),
            Self::InvalidNextReceiptSequence { expected, actual } => write!(
                formatter,
                "The next analysis lifecycle receipt must be {expected}, not {actual}."
            ),
        }
    }
}

/// Atomic plan-command failure. The receiver is unchanged for every error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnalysisPlanError {
    InstanceNotFound(AnalysisInstanceId),
    DuplicateIdentity(AnalysisInstanceId),
    RetiredIdentity(AnalysisInstanceId),
    DraftKindMismatch {
        expected: AnalysisKind,
        actual: AnalysisKind,
    },
    InvalidInstanceRevision {
        id: AnalysisInstanceId,
    },
    PositionOutOfBounds {
        position: usize,
        length: usize,
    },
    InstanceExecuting(AnalysisInstanceId),
    UnexpectedDependencyRole {
        dependent: AnalysisInstanceId,
        prerequisite: AnalysisKind,
    },
    SelfDependency {
        dependent: AnalysisInstanceId,
    },
    DependencyTargetMissing {
        dependent: AnalysisInstanceId,
        target: AnalysisInstanceId,
    },
    DependencyTargetWrongKind {
        dependent: AnalysisInstanceId,
        target: AnalysisInstanceId,
        expected: AnalysisKind,
        actual: AnalysisKind,
    },
    DependencyTargetDisabled {
        dependent: AnalysisInstanceId,
        target: AnalysisInstanceId,
    },
    DependencyTargetNotEarlier {
        dependent: AnalysisInstanceId,
        target: AnalysisInstanceId,
    },
    DependencyConfigurationInvalid {
        dependent: AnalysisInstanceId,
        prerequisite: AnalysisKind,
        detail: String,
    },
    ReferencedBy {
        target: AnalysisInstanceId,
        dependents: Vec<AnalysisInstanceId>,
    },
    InvalidConfigurationChangeDetail,
    ReceiptSequenceExhausted,
    Revision(RevisionError),
    InvalidPlan(Vec<AnalysisPlanIssue>),
}

impl fmt::Display for AnalysisPlanError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InstanceNotFound(id) => write!(formatter, "Analysis {id} does not exist."),
            Self::DuplicateIdentity(id) => {
                write!(formatter, "Analysis identity {id} is already active.")
            }
            Self::RetiredIdentity(id) => write!(
                formatter,
                "Analysis identity {id} was retired and cannot be reused."
            ),
            Self::DraftKindMismatch { expected, actual } => write!(
                formatter,
                "The {expected} analysis contains an {actual} draft."
            ),
            Self::InvalidInstanceRevision { id } => write!(
                formatter,
                "Analysis {id} has an invalid creation or modification revision."
            ),
            Self::PositionOutOfBounds { position, length } => write!(
                formatter,
                "Position {} is outside this {length}-analysis plan.",
                position.saturating_add(1)
            ),
            Self::InstanceExecuting(id) => write!(
                formatter,
                "Analysis {id} is queued or executing and cannot be changed."
            ),
            Self::UnexpectedDependencyRole {
                dependent,
                prerequisite,
            } => write!(
                formatter,
                "Analysis {dependent} does not accept a {prerequisite} prerequisite."
            ),
            Self::SelfDependency { dependent } => {
                write!(formatter, "Analysis {dependent} cannot depend on itself.")
            }
            Self::DependencyTargetMissing { dependent, target } => write!(
                formatter,
                "Analysis {dependent} references missing prerequisite {target}."
            ),
            Self::DependencyTargetWrongKind {
                dependent,
                target,
                expected,
                actual,
            } => write!(
                formatter,
                "Analysis {dependent} requires {target} to be {expected}, but it is {actual}."
            ),
            Self::DependencyTargetDisabled { dependent, target } => write!(
                formatter,
                "Analysis {dependent} requires enabled prerequisite {target}."
            ),
            Self::DependencyTargetNotEarlier { dependent, target } => write!(
                formatter,
                "Analysis {dependent} must follow prerequisite {target}."
            ),
            Self::DependencyConfigurationInvalid {
                dependent,
                prerequisite,
                detail,
            } => write!(
                formatter,
                "Analysis {dependent} cannot repair its {prerequisite} prerequisite: {detail}."
            ),
            Self::ReferencedBy { target, dependents } => {
                write!(formatter, "Analysis {target} is still required by ")?;
                for (index, dependent) in dependents.iter().enumerate() {
                    if index > 0 {
                        formatter.write_str(", ")?;
                    }
                    write!(formatter, "{dependent}")?;
                }
                formatter.write_str(".")
            }
            Self::InvalidConfigurationChangeDetail => formatter.write_str(
                "Plan configuration change detail must be a non-empty single line of at most 512 characters.",
            ),
            Self::ReceiptSequenceExhausted => {
                formatter.write_str("The analysis lifecycle receipt sequence is exhausted.")
            }
            Self::Revision(error) => error.fmt(formatter),
            Self::InvalidPlan(issues) => {
                let Some(first) = issues.first() else {
                    return formatter.write_str("The simulation plan is invalid.");
                };
                write!(formatter, "{first}")?;
                if issues.len() > 1 {
                    write!(
                        formatter,
                        " Review {} additional plan issue{}.",
                        issues.len() - 1,
                        if issues.len() == 2 { "" } else { "s" }
                    )?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for AnalysisPlanError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Revision(error) => Some(error),
            _ => None,
        }
    }
}

impl From<RevisionError> for AnalysisPlanError {
    fn from(error: RevisionError) -> Self {
        Self::Revision(error)
    }
}

/// Immutable execution projection. It contains only enabled analyses, in
/// deterministic presentation/dependency order, with deep-copied drafts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FrozenAnalysisInstance {
    order: usize,
    id: AnalysisInstanceId,
    kind: AnalysisKind,
    draft: AnalysisDraft,
    dependencies: Vec<AnalysisDependency>,
}

impl FrozenAnalysisInstance {
    #[must_use]
    pub const fn order(&self) -> usize {
        self.order
    }

    #[must_use]
    pub const fn id(&self) -> AnalysisInstanceId {
        self.id
    }

    #[must_use]
    pub const fn kind(&self) -> AnalysisKind {
        self.kind
    }

    #[must_use]
    pub const fn draft(&self) -> &AnalysisDraft {
        &self.draft
    }

    #[must_use]
    pub fn dependencies(&self) -> &[AnalysisDependency] {
        &self.dependencies
    }
}

/// Revision-pinned immutable input to preflight and execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FrozenSimulationPlan {
    plan_id: SimulationPlanId,
    revision: ObjectRevision,
    instances: Vec<FrozenAnalysisInstance>,
}

impl FrozenSimulationPlan {
    #[must_use]
    pub const fn plan_id(&self) -> SimulationPlanId {
        self.plan_id
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub fn instances(&self) -> &[FrozenAnalysisInstance] {
        &self.instances
    }
}

/// Revisioned ordered collection of stable analysis instances.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationPlan {
    id: SimulationPlanId,
    revision: ObjectRevision,
    instances: Vec<AnalysisInstance>,
    tombstones: Vec<AnalysisTombstone>,
    receipts: Vec<AnalysisLifecycleReceipt>,
    #[serde(default)]
    configuration_receipts: Vec<SimulationPlanConfigurationReceipt>,
    next_receipt_sequence: u64,
}

impl Default for SimulationPlan {
    fn default() -> Self {
        Self::new()
    }
}

impl SimulationPlan {
    /// Create a fresh, intentionally empty plan.
    ///
    /// Empty plans are valid editable documents, but cannot be frozen for
    /// execution until at least one analysis is inserted and enabled. This is
    /// used when a plan clone deliberately excludes the source analyses.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            id: SimulationPlanId::new(),
            revision: ObjectRevision::INITIAL,
            instances: Vec::new(),
            tombstones: Vec::new(),
            receipts: Vec::new(),
            configuration_receipts: Vec::new(),
            next_receipt_sequence: 1,
        }
    }

    /// Fresh plans contain one enabled transient instance.
    #[must_use]
    pub fn new() -> Self {
        let revision = ObjectRevision::INITIAL;
        Self {
            id: SimulationPlanId::new(),
            revision,
            instances: vec![AnalysisInstance::fresh(
                AnalysisInstanceId::new(),
                AnalysisDraft::for_kind(AnalysisKind::Transient),
                true,
                Vec::new(),
                revision,
            )],
            tombstones: Vec::new(),
            receipts: Vec::new(),
            configuration_receipts: Vec::new(),
            next_receipt_sequence: 1,
        }
    }

    /// Deep-copy the editable analysis graph into a new plan identity.
    ///
    /// Every analysis receives a fresh identity and every dependency edge is
    /// remapped to the corresponding clone. Revision and lifecycle history,
    /// mutation receipts, tombstones, and prior result references belong to
    /// the source plan and are intentionally not copied. Draft values,
    /// enabled state, order, and dependency roles are preserved exactly.
    pub fn clone_as_new(&self) -> Result<Self, AnalysisPlanError> {
        self.ensure_structurally_valid()?;

        let source_identities = self
            .instances
            .iter()
            .map(|instance| instance.id)
            .chain(self.tombstones.iter().map(|tombstone| tombstone.id))
            .collect::<HashSet<_>>();
        let mut cloned_identities = HashSet::with_capacity(self.instances.len());
        let mut identity_map = HashMap::with_capacity(self.instances.len());
        for instance in &self.instances {
            let id = loop {
                let candidate = AnalysisInstanceId::new();
                if !source_identities.contains(&candidate) && cloned_identities.insert(candidate) {
                    break candidate;
                }
            };
            identity_map.insert(instance.id, id);
        }

        let revision = ObjectRevision::INITIAL;
        let instances = self
            .instances
            .iter()
            .map(|source| {
                let id = identity_map[&source.id];
                let dependencies = source
                    .dependencies
                    .iter()
                    .map(|dependency| {
                        let target = identity_map.get(&dependency.target).copied().ok_or(
                            AnalysisPlanError::DependencyTargetMissing {
                                dependent: id,
                                target: dependency.target,
                            },
                        )?;
                        Ok(AnalysisDependency::new(dependency.prerequisite, target))
                    })
                    .collect::<Result<Vec<_>, AnalysisPlanError>>()?;
                Ok(AnalysisInstance::fresh(
                    id,
                    source.draft.clone(),
                    source.enabled,
                    dependencies,
                    revision,
                ))
            })
            .collect::<Result<Vec<_>, AnalysisPlanError>>()?;

        let id = loop {
            let candidate = SimulationPlanId::new();
            if candidate != self.id {
                break candidate;
            }
        };
        let cloned = Self {
            id,
            revision,
            instances,
            tombstones: Vec::new(),
            receipts: Vec::new(),
            configuration_receipts: Vec::new(),
            next_receipt_sequence: 1,
        };
        cloned.ensure_structurally_valid()?;
        Ok(cloned)
    }

    /// Validate a deterministic supplied-ID migration in its exact order.
    /// Missing prerequisite bindings remain diagnostics and do not reject the
    /// migration; corrupt identity, revision, lifecycle, or graph state does.
    pub fn from_ordered_instances(
        id: SimulationPlanId,
        revision: ObjectRevision,
        instances: Vec<AnalysisInstance>,
    ) -> Result<Self, AnalysisPlanError> {
        Self::from_persisted_parts(id, revision, instances, Vec::new(), Vec::new())
    }

    /// Reconstruct all durable plan parts while validating their identities,
    /// revisions, receipts, and dependency graph.
    pub fn from_persisted_parts(
        id: SimulationPlanId,
        revision: ObjectRevision,
        instances: Vec<AnalysisInstance>,
        tombstones: Vec<AnalysisTombstone>,
        receipts: Vec<AnalysisLifecycleReceipt>,
    ) -> Result<Self, AnalysisPlanError> {
        Self::from_all_persisted_parts(id, revision, instances, tombstones, receipts, Vec::new())
    }

    /// Reconstruct every durable plan receipt domain. This is the lossless
    /// import boundary for schema-5 and newer plan documents.
    pub fn from_all_persisted_parts(
        id: SimulationPlanId,
        revision: ObjectRevision,
        instances: Vec<AnalysisInstance>,
        tombstones: Vec<AnalysisTombstone>,
        receipts: Vec<AnalysisLifecycleReceipt>,
        configuration_receipts: Vec<SimulationPlanConfigurationReceipt>,
    ) -> Result<Self, AnalysisPlanError> {
        let last_sequence = receipts
            .iter()
            .map(AnalysisLifecycleReceipt::sequence)
            .chain(
                configuration_receipts
                    .iter()
                    .map(SimulationPlanConfigurationReceipt::sequence),
            )
            .max();
        let next_receipt_sequence = match last_sequence {
            Some(sequence) => sequence
                .checked_add(1)
                .ok_or(AnalysisPlanError::ReceiptSequenceExhausted)?,
            None => 1,
        };
        let plan = Self {
            id,
            revision,
            instances,
            tombstones,
            receipts,
            configuration_receipts,
            next_receipt_sequence,
        };
        plan.ensure_structurally_valid()?;
        Ok(plan)
    }

    #[must_use]
    pub const fn id(&self) -> SimulationPlanId {
        self.id
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub fn instances(&self) -> &[AnalysisInstance] {
        &self.instances
    }

    #[must_use]
    pub fn tombstones(&self) -> &[AnalysisTombstone] {
        &self.tombstones
    }

    #[must_use]
    pub fn receipts(&self) -> &[AnalysisLifecycleReceipt] {
        &self.receipts
    }

    #[must_use]
    #[cfg(test)]
    pub fn configuration_receipts(&self) -> &[SimulationPlanConfigurationReceipt] {
        &self.configuration_receipts
    }

    /// Whether this plan currently owns queued or executing work.
    ///
    /// Replacing an active plan while one of its instances owns runner state
    /// would sever cancellation and progress authority, so plan-catalog
    /// operations use this guard before switching or cloning the active plan.
    #[must_use]
    pub fn has_executing_instances(&self) -> bool {
        self.instances
            .iter()
            .any(|instance| instance.lifecycle.is_executing())
    }

    /// Commit a project-owned plan configuration change without fabricating
    /// an analysis-instance mutation.
    ///
    /// The plan revision advances exactly once, which invalidates preflight
    /// artifacts pinned to the previous revision. A durable receipt shares
    /// the same monotonic sequence as analysis lifecycle receipts. The method
    /// is atomic and refuses to replace configuration while this plan owns
    /// queued or executing work.
    pub fn commit_configuration_change(
        &mut self,
        detail: impl Into<String>,
    ) -> Result<SimulationPlanConfigurationReceipt, AnalysisPlanError> {
        let detail = detail.into();
        let detail = detail.trim();
        if detail.is_empty()
            || detail.chars().count() > 512
            || detail
                .chars()
                .any(|character| matches!(character, '\r' | '\n'))
        {
            return Err(AnalysisPlanError::InvalidConfigurationChangeDetail);
        }
        if let Some(instance) = self
            .instances
            .iter()
            .find(|instance| instance.lifecycle.is_executing())
        {
            return Err(AnalysisPlanError::InstanceExecuting(instance.id));
        }

        let mut candidate = self.clone();
        let source_revision = candidate.revision;
        let committed_revision = source_revision.next()?;
        let next_sequence = candidate
            .next_receipt_sequence
            .checked_add(1)
            .ok_or(AnalysisPlanError::ReceiptSequenceExhausted)?;
        let receipt = SimulationPlanConfigurationReceipt {
            sequence: candidate.next_receipt_sequence,
            source_revision,
            committed_revision,
            detail: detail.to_owned(),
        };
        candidate.revision = committed_revision;
        candidate.next_receipt_sequence = next_sequence;
        candidate.configuration_receipts.push(receipt.clone());
        candidate.ensure_structurally_valid()?;
        *self = candidate;
        Ok(receipt)
    }

    #[must_use]
    pub fn instance(&self, id: AnalysisInstanceId) -> Option<&AnalysisInstance> {
        self.instances.iter().find(|instance| instance.id == id)
    }

    /// Whether an exact target can satisfy a prerequisite without a known
    /// numerical-contract failure. Used by selectors as well as mutations so
    /// the UI cannot offer a binding that the plan will reject.
    #[must_use]
    #[cfg(test)]
    pub fn dependency_candidate_is_compatible(
        &self,
        dependent: AnalysisInstanceId,
        prerequisite: AnalysisKind,
        target: AnalysisInstanceId,
    ) -> bool {
        let (Some(dependent), Some(target)) = (self.instance(dependent), self.instance(target))
        else {
            return false;
        };
        Self::dependency_candidate_compatibility(dependent, prerequisite, target)
            .is_ok_and(|compatible| compatible)
    }

    /// Whether an exact target satisfies both the analysis execution contract
    /// and circuit-derived prerequisite identity requirements.
    #[must_use]
    pub fn dependency_candidate_is_compatible_with_context(
        &self,
        dependent: AnalysisInstanceId,
        prerequisite: AnalysisKind,
        target: AnalysisInstanceId,
        context: &AnalysisDependencyRepairContext,
    ) -> bool {
        let (Some(dependent), Some(target)) = (self.instance(dependent), self.instance(target))
        else {
            return false;
        };
        Self::dependency_candidate_compatibility(dependent, prerequisite, target)
            .is_ok_and(|compatible| compatible)
            && dependency_candidate_context_issue(prerequisite, &target.draft, context).is_none()
    }

    /// Whether quick repair can synthesize a valid draft for this role from
    /// the dependent's current configuration.
    #[must_use]
    #[cfg(test)]
    pub fn dependency_prerequisite_is_repairable(
        &self,
        dependent: AnalysisInstanceId,
        prerequisite: AnalysisKind,
    ) -> bool {
        self.dependency_prerequisite_is_repairable_with_context(
            dependent,
            prerequisite,
            &AnalysisDependencyRepairContext::default(),
        )
    }

    /// Whether quick repair can synthesize a valid prerequisite using exact
    /// circuit-derived source identity where the role requires it.
    #[must_use]
    pub fn dependency_prerequisite_is_repairable_with_context(
        &self,
        dependent: AnalysisInstanceId,
        prerequisite: AnalysisKind,
        context: &AnalysisDependencyRepairContext,
    ) -> bool {
        let Some(instance) = self.instance(dependent) else {
            return false;
        };
        instance.prerequisite_roles().contains(&prerequisite)
            && (self.instances.iter().any(|candidate| {
                self.dependency_candidate_is_compatible_with_context(
                    dependent,
                    prerequisite,
                    candidate.id,
                    context,
                )
            }) || prerequisite_draft_for(&instance.draft, prerequisite, context).is_ok())
    }

    fn dependency_candidate_compatibility(
        dependent: &AnalysisInstance,
        prerequisite: AnalysisKind,
        target: &AnalysisInstance,
    ) -> Result<bool, String> {
        if dependent.id == target.id
            || !dependent.prerequisite_roles().contains(&prerequisite)
            || target.kind != prerequisite
        {
            return Ok(false);
        }
        Ok(dependency_configuration_issue(&dependent.draft, &target.draft).is_none())
    }

    /// Repair runtime-only dialog sentinels and relinquish runner authority.
    ///
    /// A queued, running, or paused state belongs to the process that owned
    /// the plan before persistence. Restoring those states cannot recreate
    /// that process or its cancellation authority, so enabled instances
    /// return to an editable draft without changing durable identity or
    /// revision history.
    pub fn prepare_after_restore(&mut self) {
        for instance in &mut self.instances {
            instance.draft.prepare_after_restore();
            let required_roles = instance.draft.prerequisite_roles();
            // Schema migrations may retire a dependency role. Retaining such
            // an edge would make an otherwise valid saved plan structurally
            // corrupt, so restore prunes only roles the current typed draft no
            // longer owns. Required edges and immutable receipt history remain
            // untouched.
            instance
                .dependencies
                .retain(|dependency| required_roles.contains(&dependency.prerequisite));
            if instance.lifecycle.is_executing() {
                instance.lifecycle = if instance.enabled {
                    AnalysisLifecycleState::Draft
                } else {
                    AnalysisLifecycleState::Disabled
                };
            }
        }
    }

    /// All deterministic validation diagnostics, including unresolved required
    /// prerequisites that are legal while a plan is being assembled.
    #[must_use]
    pub fn validation_issues(&self) -> Vec<AnalysisPlanIssue> {
        let mut issues = Vec::new();
        if !self.instances.iter().any(|instance| instance.enabled) {
            issues.push(AnalysisPlanIssue::NoEnabledInstances);
        }

        let mut active_ids = HashSet::new();
        let mut positions = HashMap::new();
        for (position, instance) in self.instances.iter().enumerate() {
            if !active_ids.insert(instance.id) {
                issues.push(AnalysisPlanIssue::DuplicateInstanceId { id: instance.id });
            }
            positions.entry(instance.id).or_insert(position);
            if instance.kind != instance.draft.kind() {
                issues.push(AnalysisPlanIssue::KindDraftMismatch {
                    id: instance.id,
                    expected: instance.kind,
                    actual: instance.draft.kind(),
                });
            }
            if instance.created_revision > instance.modified_revision
                || instance.modified_revision > self.revision
            {
                issues.push(AnalysisPlanIssue::InvalidInstanceRevision { id: instance.id });
            }
            let lifecycle_valid = if instance.enabled {
                !matches!(
                    instance.lifecycle,
                    AnalysisLifecycleState::Absent
                        | AnalysisLifecycleState::Disabled
                        | AnalysisLifecycleState::Removed
                        | AnalysisLifecycleState::SameState
                )
            } else {
                instance.lifecycle == AnalysisLifecycleState::Disabled
            };
            if !lifecycle_valid {
                issues.push(AnalysisPlanIssue::InvalidLifecycle {
                    id: instance.id,
                    state: instance.lifecycle,
                    enabled: instance.enabled,
                });
            }
        }

        let mut tombstone_ids = HashSet::new();
        for tombstone in &self.tombstones {
            if !tombstone_ids.insert(tombstone.id) {
                issues.push(AnalysisPlanIssue::DuplicateTombstoneId { id: tombstone.id });
            }
            if active_ids.contains(&tombstone.id) {
                issues.push(AnalysisPlanIssue::ReusedTombstonedId { id: tombstone.id });
            }
            if tombstone.created_revision > tombstone.last_modified_revision
                || tombstone.last_modified_revision >= tombstone.removed_revision
                || tombstone.removed_revision > self.revision
            {
                issues.push(AnalysisPlanIssue::InvalidTombstoneRevision { id: tombstone.id });
            }
        }

        let by_id: HashMap<_, _> = self
            .instances
            .iter()
            .map(|instance| (instance.id, instance))
            .collect();
        for instance in &self.instances {
            let mut roles = HashSet::new();
            for dependency in &instance.dependencies {
                if !instance
                    .prerequisite_roles()
                    .contains(&dependency.prerequisite)
                {
                    issues.push(AnalysisPlanIssue::UnexpectedDependencyRole {
                        dependent: instance.id,
                        prerequisite: dependency.prerequisite,
                    });
                }
                if !roles.insert(dependency.prerequisite) {
                    issues.push(AnalysisPlanIssue::DuplicateDependencyRole {
                        dependent: instance.id,
                        prerequisite: dependency.prerequisite,
                    });
                }
                if dependency.target == instance.id {
                    issues.push(AnalysisPlanIssue::SelfDependency {
                        dependent: instance.id,
                    });
                    continue;
                }
                let Some(target) = by_id.get(&dependency.target) else {
                    issues.push(AnalysisPlanIssue::DanglingDependency {
                        dependent: instance.id,
                        target: dependency.target,
                    });
                    continue;
                };
                if target.kind != dependency.prerequisite {
                    issues.push(AnalysisPlanIssue::WrongDependencyKind {
                        dependent: instance.id,
                        prerequisite: dependency.prerequisite,
                        target: target.id,
                        actual: target.kind,
                    });
                } else if instance.enabled {
                    match dependency_configuration_issue(&instance.draft, &target.draft) {
                        Some(DependencyConfigurationIssue::InvalidPrerequisite(detail))
                        | Some(DependencyConfigurationIssue::Incompatible(detail)) => {
                            issues.push(AnalysisPlanIssue::IncompatibleDependencyConfiguration {
                                dependent: instance.id,
                                prerequisite: dependency.prerequisite,
                                target: target.id,
                                detail,
                            });
                        }
                        // The dependent's own form validation owns this error.
                        // Suppressing a dependency repair CTA avoids promising a
                        // prerequisite mutation that cannot repair the consumer.
                        Some(DependencyConfigurationIssue::InvalidDependent(_)) | None => {}
                    }
                }
                if instance.enabled {
                    if !target.enabled {
                        issues.push(AnalysisPlanIssue::DisabledDependency {
                            dependent: instance.id,
                            target: target.id,
                        });
                    }
                    if positions.get(&target.id) >= positions.get(&instance.id) {
                        issues.push(AnalysisPlanIssue::DependencyNotEarlier {
                            dependent: instance.id,
                            target: target.id,
                        });
                    }
                }
            }
            if instance.enabled {
                for prerequisite in instance.prerequisite_roles() {
                    if !roles.contains(prerequisite) {
                        issues.push(AnalysisPlanIssue::MissingPrerequisite {
                            dependent: instance.id,
                            prerequisite: *prerequisite,
                        });
                    }
                }
            }
        }

        if let Some(members) = self.dependency_cycle_members(&by_id) {
            issues.push(AnalysisPlanIssue::DependencyCycle { members });
        }

        enum RetainedReceipt<'a> {
            Analysis(&'a AnalysisLifecycleReceipt),
            Configuration(&'a SimulationPlanConfigurationReceipt),
        }
        impl RetainedReceipt<'_> {
            fn sequence(&self) -> u64 {
                match self {
                    Self::Analysis(receipt) => receipt.sequence,
                    Self::Configuration(receipt) => receipt.sequence,
                }
            }

            fn source_revision(&self) -> ObjectRevision {
                match self {
                    Self::Analysis(receipt) => receipt.source_revision,
                    Self::Configuration(receipt) => receipt.source_revision,
                }
            }

            fn committed_revision(&self) -> ObjectRevision {
                match self {
                    Self::Analysis(receipt) => receipt.committed_revision,
                    Self::Configuration(receipt) => receipt.committed_revision,
                }
            }

            fn detail(&self) -> &str {
                match self {
                    Self::Analysis(receipt) => &receipt.detail,
                    Self::Configuration(receipt) => &receipt.detail,
                }
            }
        }

        for pair in self.receipts.windows(2) {
            if pair[0].sequence >= pair[1].sequence {
                issues.push(AnalysisPlanIssue::InvalidReceiptSequence {
                    sequence: pair[1].sequence,
                });
            }
        }
        for pair in self.configuration_receipts.windows(2) {
            if pair[0].sequence >= pair[1].sequence {
                issues.push(AnalysisPlanIssue::InvalidReceiptSequence {
                    sequence: pair[1].sequence,
                });
            }
        }

        let mut retained_receipts = self
            .receipts
            .iter()
            .map(RetainedReceipt::Analysis)
            .chain(
                self.configuration_receipts
                    .iter()
                    .map(RetainedReceipt::Configuration),
            )
            .collect::<Vec<_>>();
        retained_receipts.sort_by_key(RetainedReceipt::sequence);

        let mut expected_sequence = 1;
        let mut previous_committed_revision = None;
        for retained in retained_receipts {
            let sequence = retained.sequence();
            let source_revision = retained.source_revision();
            let committed_revision = retained.committed_revision();
            if sequence != expected_sequence {
                issues.push(AnalysisPlanIssue::InvalidReceiptSequence { sequence });
            }
            expected_sequence = sequence.saturating_add(1);
            let revisions_are_consecutive = matches!(
                source_revision.next(),
                Ok(next) if next == committed_revision
            );
            let follows_previous_receipt = match previous_committed_revision {
                Some(previous) => previous == source_revision,
                None => true,
            };
            if source_revision >= committed_revision
                || committed_revision > self.revision
                || !revisions_are_consecutive
                || !follows_previous_receipt
            {
                issues.push(AnalysisPlanIssue::InvalidReceiptRevision { sequence });
            }
            let detail = retained.detail();
            let detail_is_empty = detail.trim().is_empty();
            let detail_is_invalid = detail.chars().count() > 512
                || detail
                    .chars()
                    .any(|character| matches!(character, '\r' | '\n'));
            if let RetainedReceipt::Analysis(receipt) = retained {
                let retained_kind = by_id
                    .get(&receipt.instance_id)
                    .map(|instance| instance.kind)
                    .or_else(|| {
                        self.tombstones
                            .iter()
                            .find(|tombstone| tombstone.id == receipt.instance_id)
                            .map(|tombstone| tombstone.kind)
                    });
                match retained_kind {
                    Some(expected) if expected != receipt.kind => {
                        issues.push(AnalysisPlanIssue::ReceiptKindMismatch {
                            sequence,
                            expected,
                            actual: receipt.kind,
                        });
                    }
                    Some(_) => {}
                    None => issues.push(AnalysisPlanIssue::DanglingReceiptInstance {
                        sequence,
                        id: receipt.instance_id,
                    }),
                }
            }
            if detail_is_empty || detail_is_invalid {
                issues.push(AnalysisPlanIssue::EmptyReceiptDetail { sequence });
            }
            previous_committed_revision = Some(committed_revision);
        }
        if self.next_receipt_sequence != expected_sequence {
            issues.push(AnalysisPlanIssue::InvalidNextReceiptSequence {
                expected: expected_sequence,
                actual: self.next_receipt_sequence,
            });
        }
        issues
    }

    /// Corruption-level diagnostics that must never cross persistence or
    /// execution boundaries. Editable incompleteness (`NoEnabledInstances`
    /// and `MissingPrerequisite`) is deliberately excluded.
    #[must_use]
    pub fn structural_issues(&self) -> Vec<AnalysisPlanIssue> {
        self.validation_issues()
            .into_iter()
            .filter(AnalysisPlanIssue::is_structural)
            .collect()
    }

    /// Reject corrupt persisted structure while allowing an incomplete draft
    /// plan to round-trip for repair in the editor.
    pub fn validate_structure(&self) -> Result<(), AnalysisPlanError> {
        let issues = self.structural_issues();
        if issues.is_empty() {
            Ok(())
        } else {
            Err(AnalysisPlanError::InvalidPlan(issues))
        }
    }

    fn dependency_cycle_members(
        &self,
        by_id: &HashMap<AnalysisInstanceId, &AnalysisInstance>,
    ) -> Option<Vec<AnalysisInstanceId>> {
        let mut indegree: HashMap<_, _> = self
            .instances
            .iter()
            .map(|instance| (instance.id, 0usize))
            .collect();
        let mut outgoing: HashMap<AnalysisInstanceId, Vec<AnalysisInstanceId>> = HashMap::new();
        for dependent in &self.instances {
            for dependency in &dependent.dependencies {
                if by_id.contains_key(&dependency.target) {
                    *indegree.entry(dependent.id).or_default() += 1;
                    outgoing
                        .entry(dependency.target)
                        .or_default()
                        .push(dependent.id);
                }
            }
        }
        let mut ready: VecDeque<_> = self
            .instances
            .iter()
            .filter(|instance| indegree[&instance.id] == 0)
            .map(|instance| instance.id)
            .collect();
        let mut visited = 0;
        while let Some(id) = ready.pop_front() {
            visited += 1;
            if let Some(dependents) = outgoing.get(&id) {
                for dependent in dependents {
                    let degree = indegree
                        .get_mut(dependent)
                        .expect("active dependent has indegree");
                    *degree -= 1;
                    if *degree == 0 {
                        ready.push_back(*dependent);
                    }
                }
            }
        }
        (visited != self.instances.len()).then(|| {
            self.instances
                .iter()
                .filter(|instance| indegree[&instance.id] > 0)
                .map(|instance| instance.id)
                .collect()
        })
    }

    fn ensure_structurally_valid(&self) -> Result<(), AnalysisPlanError> {
        self.validate_structure()
    }

    fn ensure_identity_available(&self, id: AnalysisInstanceId) -> Result<(), AnalysisPlanError> {
        if self.instances.iter().any(|instance| instance.id == id) {
            return Err(AnalysisPlanError::DuplicateIdentity(id));
        }
        if self.tombstones.iter().any(|tombstone| tombstone.id == id) {
            return Err(AnalysisPlanError::RetiredIdentity(id));
        }
        Ok(())
    }

    fn fresh_identity(&self) -> AnalysisInstanceId {
        loop {
            let id = AnalysisInstanceId::new();
            if self.ensure_identity_available(id).is_ok() {
                return id;
            }
        }
    }

    fn index_of(&self, id: AnalysisInstanceId) -> Result<usize, AnalysisPlanError> {
        self.instances
            .iter()
            .position(|instance| instance.id == id)
            .ok_or(AnalysisPlanError::InstanceNotFound(id))
    }

    fn ensure_editable(&self, index: usize) -> Result<(), AnalysisPlanError> {
        let instance = &self.instances[index];
        if instance.lifecycle.is_executing() {
            Err(AnalysisPlanError::InstanceExecuting(instance.id))
        } else {
            Ok(())
        }
    }

    fn dependents_of(&self, target: AnalysisInstanceId) -> Vec<AnalysisInstanceId> {
        self.instances
            .iter()
            .filter(|instance| {
                instance
                    .dependencies
                    .iter()
                    .any(|dependency| dependency.target == target)
            })
            .map(|instance| instance.id)
            .collect()
    }

    fn enabled_dependents_of(&self, target: AnalysisInstanceId) -> Vec<AnalysisInstanceId> {
        self.instances
            .iter()
            .filter(|instance| {
                instance.enabled
                    && instance
                        .dependencies
                        .iter()
                        .any(|dependency| dependency.target == target)
            })
            .map(|instance| instance.id)
            .collect()
    }

    fn transact<T>(
        &mut self,
        command: AnalysisLifecycleCommand,
        instance_id: AnalysisInstanceId,
        related_instance_id: Option<AnalysisInstanceId>,
        outcome: AnalysisLifecycleState,
        detail: String,
        mutation: impl FnOnce(&mut Self, ObjectRevision) -> Result<T, AnalysisPlanError>,
    ) -> Result<(T, AnalysisLifecycleReceipt), AnalysisPlanError> {
        let mut candidate = self.clone();
        let source_revision = self.revision;
        let committed_revision = source_revision.next()?;
        let result = mutation(&mut candidate, committed_revision)?;
        candidate.revision = committed_revision;
        let next_sequence = candidate
            .next_receipt_sequence
            .checked_add(1)
            .ok_or(AnalysisPlanError::ReceiptSequenceExhausted)?;
        let kind = if let Some(instance) = candidate.instance(instance_id) {
            instance.kind
        } else if let Some(tombstone) = candidate
            .tombstones
            .iter()
            .find(|tombstone| tombstone.id == instance_id)
        {
            tombstone.kind
        } else {
            return Err(AnalysisPlanError::InstanceNotFound(instance_id));
        };
        let receipt = AnalysisLifecycleReceipt {
            sequence: candidate.next_receipt_sequence,
            command,
            instance_id,
            kind,
            outcome,
            related_instance_id,
            source_revision,
            committed_revision,
            detail,
        };
        candidate.next_receipt_sequence = next_sequence;
        candidate.receipts.push(receipt.clone());
        candidate.ensure_structurally_valid()?;
        *self = candidate;
        Ok((result, receipt))
    }

    /// Append a fresh enabled analysis. Unresolved required prerequisites are
    /// retained as validation issues so insertion can precede explicit binding.
    pub fn insert(
        &mut self,
        kind: AnalysisKind,
    ) -> Result<(AnalysisInstanceId, AnalysisLifecycleReceipt), AnalysisPlanError> {
        let position = self.instances.len();
        self.insert_at(kind, position)
    }

    /// Insert a fresh enabled analysis at an exact presentation position.
    pub fn insert_at(
        &mut self,
        kind: AnalysisKind,
        position: usize,
    ) -> Result<(AnalysisInstanceId, AnalysisLifecycleReceipt), AnalysisPlanError> {
        let id = self.fresh_identity();
        self.insert_draft_with_id(id, AnalysisDraft::for_kind(kind), true, position)
    }

    /// Insert a supplied identity and draft through the normal atomic command
    /// path. Primarily useful for deterministic import and tests.
    pub fn insert_draft_with_id(
        &mut self,
        id: AnalysisInstanceId,
        draft: AnalysisDraft,
        enabled: bool,
        position: usize,
    ) -> Result<(AnalysisInstanceId, AnalysisLifecycleReceipt), AnalysisPlanError> {
        let kind = draft.kind();
        let outcome = if enabled {
            AnalysisLifecycleState::Draft
        } else {
            AnalysisLifecycleState::Disabled
        };
        let ((), receipt) = self.transact(
            AnalysisLifecycleCommand::Insert,
            id,
            None,
            outcome,
            format!(
                "{} analysis {id} was inserted at position {}.",
                kind.label(),
                position.saturating_add(1)
            ),
            move |candidate, revision| {
                candidate.ensure_identity_available(id)?;
                if position > candidate.instances.len() {
                    return Err(AnalysisPlanError::PositionOutOfBounds {
                        position,
                        length: candidate.instances.len(),
                    });
                }
                candidate.instances.insert(
                    position,
                    AnalysisInstance::fresh(id, draft, enabled, Vec::new(), revision),
                );
                debug_assert_eq!(candidate.instances[position].kind, kind);
                Ok(())
            },
        )?;
        Ok((id, receipt))
    }

    /// Atomically edit one raw draft. Replacing it with a different kind is
    /// rejected and leaves the plan byte-for-byte unchanged.
    pub fn edit<R>(
        &mut self,
        id: AnalysisInstanceId,
        edit: impl FnOnce(&mut AnalysisDraft) -> R,
    ) -> Result<(R, AnalysisLifecycleReceipt), AnalysisPlanError> {
        let instance = self
            .instance(id)
            .ok_or(AnalysisPlanError::InstanceNotFound(id))?;
        let kind = instance.kind();
        let outcome = if instance.enabled() {
            AnalysisLifecycleState::Draft
        } else {
            AnalysisLifecycleState::Disabled
        };
        self.transact(
            AnalysisLifecycleCommand::Edit,
            id,
            None,
            outcome,
            format!("{} analysis {id} draft was updated.", kind.label()),
            move |candidate, revision| {
                let index = candidate.index_of(id)?;
                candidate.ensure_editable(index)?;
                let expected = candidate.instances[index].kind;
                let output = edit(&mut candidate.instances[index].draft);
                let actual = candidate.instances[index].draft.kind();
                if expected != actual {
                    return Err(AnalysisPlanError::DraftKindMismatch { expected, actual });
                }
                let required_roles = candidate.instances[index]
                    .draft
                    .prerequisite_roles()
                    .to_vec();
                let instance = &mut candidate.instances[index];
                // A draft edit can legitimately change the dependency schema.
                // Remove obsolete edges in the same atomic edit so an ordinary
                // form change cannot create structural corruption. A newly
                // required unbound role remains a precise MissingPrerequisite
                // issue and can be resolved by bind/repair without rewriting
                // history.
                instance
                    .dependencies
                    .retain(|dependency| required_roles.contains(&dependency.prerequisite));
                instance.modified_revision = revision;
                instance.lifecycle = if instance.enabled {
                    AnalysisLifecycleState::Draft
                } else {
                    AnalysisLifecycleState::Disabled
                };
                Ok(output)
            },
        )
    }

    /// Deep-clone an instance and insert the fresh identity directly after it.
    pub fn clone_instance(
        &mut self,
        source: AnalysisInstanceId,
    ) -> Result<(AnalysisInstanceId, AnalysisLifecycleReceipt), AnalysisPlanError> {
        let id = self.fresh_identity();
        self.clone_instance_with_id(source, id)
    }

    /// Deterministic-ID clone variant with the same no-reuse checks.
    pub fn clone_instance_with_id(
        &mut self,
        source: AnalysisInstanceId,
        id: AnalysisInstanceId,
    ) -> Result<(AnalysisInstanceId, AnalysisLifecycleReceipt), AnalysisPlanError> {
        let source_instance = self
            .instance(source)
            .ok_or(AnalysisPlanError::InstanceNotFound(source))?;
        let kind = source_instance.kind();
        let outcome = if source_instance.enabled() {
            AnalysisLifecycleState::Draft
        } else {
            AnalysisLifecycleState::Disabled
        };
        let ((), receipt) = self.transact(
            AnalysisLifecycleCommand::Clone,
            id,
            Some(source),
            outcome,
            format!(
                "{} analysis {source} was cloned as {id} with independent draft values.",
                kind.label()
            ),
            move |candidate, revision| {
                candidate.ensure_identity_available(id)?;
                let source_index = candidate.index_of(source)?;
                candidate.ensure_editable(source_index)?;
                let source_instance = candidate.instances[source_index].clone();
                candidate.instances.insert(
                    source_index + 1,
                    AnalysisInstance::fresh(
                        id,
                        source_instance.draft,
                        source_instance.enabled,
                        source_instance.dependencies,
                        revision,
                    ),
                );
                Ok(())
            },
        )?;
        Ok((id, receipt))
    }

    /// Enable or disable an instance without removing its ordered position.
    /// A target referenced by an enabled dependent cannot be disabled.
    pub fn set_enabled(
        &mut self,
        id: AnalysisInstanceId,
        enabled: bool,
    ) -> Result<AnalysisLifecycleReceipt, AnalysisPlanError> {
        let kind = self
            .instance(id)
            .map(AnalysisInstance::kind)
            .ok_or(AnalysisPlanError::InstanceNotFound(id))?;
        let disposition = if enabled { "enabled" } else { "disabled" };
        let ((), receipt) = self.transact(
            if enabled {
                AnalysisLifecycleCommand::Enable
            } else {
                AnalysisLifecycleCommand::Disable
            },
            id,
            None,
            if enabled {
                AnalysisLifecycleState::Draft
            } else {
                AnalysisLifecycleState::Disabled
            },
            format!("{} analysis {id} was {disposition}.", kind.label()),
            move |candidate, revision| {
                let index = candidate.index_of(id)?;
                candidate.ensure_editable(index)?;
                if !enabled {
                    let dependents = candidate.enabled_dependents_of(id);
                    if !dependents.is_empty() {
                        return Err(AnalysisPlanError::ReferencedBy {
                            target: id,
                            dependents,
                        });
                    }
                }
                let retained_dependencies = enabled.then(|| {
                    candidate.instances[index]
                        .prerequisite_roles()
                        .iter()
                        .filter_map(|prerequisite| {
                            candidate.instances[index]
                                .dependencies
                                .iter()
                                .find(|dependency| dependency.prerequisite == *prerequisite)
                                .copied()
                                .filter(|dependency| {
                                    candidate.instances[..index].iter().any(|target| {
                                        target.id == dependency.target
                                            && target.enabled
                                            && target.kind == *prerequisite
                                            && Self::dependency_candidate_compatibility(
                                                &candidate.instances[index],
                                                *prerequisite,
                                                target,
                                            )
                                            .is_ok_and(|compatible| compatible)
                                    })
                                })
                        })
                        .collect::<Vec<_>>()
                });
                let instance = &mut candidate.instances[index];
                instance.enabled = enabled;
                instance.lifecycle = if enabled {
                    AnalysisLifecycleState::Draft
                } else {
                    AnalysisLifecycleState::Disabled
                };
                instance.modified_revision = revision;
                if let Some(dependencies) = retained_dependencies {
                    instance.dependencies = dependencies;
                }
                Ok(())
            },
        )?;
        Ok(receipt)
    }

    /// Move an instance to an exact final index. Moves that invert an explicit
    /// dependency edge are rejected atomically.
    pub fn reorder(
        &mut self,
        id: AnalysisInstanceId,
        position: usize,
    ) -> Result<AnalysisLifecycleReceipt, AnalysisPlanError> {
        let kind = self
            .instance(id)
            .map(AnalysisInstance::kind)
            .ok_or(AnalysisPlanError::InstanceNotFound(id))?;
        let ((), receipt) = self.transact(
            AnalysisLifecycleCommand::Reorder,
            id,
            None,
            AnalysisLifecycleState::SameState,
            format!(
                "{} analysis {id} was moved to position {}.",
                kind.label(),
                position.saturating_add(1)
            ),
            move |candidate, revision| {
                if position >= candidate.instances.len() {
                    return Err(AnalysisPlanError::PositionOutOfBounds {
                        position,
                        length: candidate.instances.len(),
                    });
                }
                let index = candidate.index_of(id)?;
                candidate.ensure_editable(index)?;
                let mut instance = candidate.instances.remove(index);
                instance.modified_revision = revision;
                candidate.instances.insert(position, instance);
                Ok(())
            },
        )?;
        Ok(receipt)
    }

    /// Remove an unreferenced instance and retain its identity, revisions, and
    /// prior result RunIds in a permanent tombstone.
    pub fn remove(
        &mut self,
        id: AnalysisInstanceId,
        prior_run_ids: Vec<RunId>,
    ) -> Result<AnalysisLifecycleReceipt, AnalysisPlanError> {
        let kind = self
            .instance(id)
            .map(AnalysisInstance::kind)
            .ok_or(AnalysisPlanError::InstanceNotFound(id))?;
        let ((), receipt) = self.transact(
            AnalysisLifecycleCommand::Remove,
            id,
            None,
            AnalysisLifecycleState::Removed,
            format!(
                "{} analysis {id} was removed; its identity and prior runs remain retained.",
                kind.label()
            ),
            move |candidate, revision| {
                let index = candidate.index_of(id)?;
                candidate.ensure_editable(index)?;
                let dependents = candidate.dependents_of(id);
                if !dependents.is_empty() {
                    return Err(AnalysisPlanError::ReferencedBy {
                        target: id,
                        dependents,
                    });
                }
                let instance = candidate.instances.remove(index);
                let mut seen = HashSet::new();
                let prior_run_ids = prior_run_ids
                    .into_iter()
                    .filter(|run_id| seen.insert(*run_id))
                    .collect();
                candidate.tombstones.push(AnalysisTombstone {
                    id: instance.id,
                    kind: instance.kind,
                    created_revision: instance.created_revision,
                    last_modified_revision: instance.modified_revision,
                    removed_revision: revision,
                    prior_run_ids,
                });
                Ok(())
            },
        )?;
        Ok(receipt)
    }

    /// Freeze a dispatchable deterministic projection. Any unresolved or
    /// structural diagnostic blocks the projection.
    pub fn freeze(&self) -> Result<FrozenSimulationPlan, AnalysisPlanError> {
        let issues = self.validation_issues();
        if !issues.is_empty() {
            return Err(AnalysisPlanError::InvalidPlan(issues));
        }
        let instances = self
            .instances
            .iter()
            .filter(|instance| instance.enabled)
            .enumerate()
            .map(|(index, instance)| {
                let mut dependencies = instance.dependencies.clone();
                dependencies.sort_by_key(|dependency| dependency.prerequisite.legacy_index());
                FrozenAnalysisInstance {
                    order: index + 1,
                    id: instance.id,
                    kind: instance.kind,
                    draft: instance.draft.clone(),
                    dependencies,
                }
            })
            .collect();
        Ok(FrozenSimulationPlan {
            plan_id: self.id,
            revision: self.revision,
            instances,
        })
    }
}

#[cfg(test)]
mod tests;
