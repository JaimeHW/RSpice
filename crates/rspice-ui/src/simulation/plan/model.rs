use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::product::{AnalysisInstanceId, ObjectRevision, RevisionError, RunId, SimulationPlanId};

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
    pub const fn last_modified_revision(&self) -> ObjectRevision {
        self.last_modified_revision
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
            Self::NoEnabledInstances | Self::MissingPrerequisite { .. }
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
    ReferencedBy {
        target: AnalysisInstanceId,
        dependents: Vec<AnalysisInstanceId>,
    },
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
    next_receipt_sequence: u64,
}

impl Default for SimulationPlan {
    fn default() -> Self {
        Self::new()
    }
}

impl SimulationPlan {
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
            next_receipt_sequence: 1,
        }
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
        let next_receipt_sequence = match receipts.last() {
            Some(receipt) => receipt
                .sequence
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
    pub fn instance(&self, id: AnalysisInstanceId) -> Option<&AnalysisInstance> {
        self.instances.iter().find(|instance| instance.id == id)
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
                    .kind
                    .prerequisites()
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
                }
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
            if instance.enabled {
                for prerequisite in instance.kind.prerequisites() {
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

        let mut expected_sequence = 1;
        let mut previous_committed_revision = None;
        for receipt in &self.receipts {
            if receipt.sequence != expected_sequence {
                issues.push(AnalysisPlanIssue::InvalidReceiptSequence {
                    sequence: receipt.sequence,
                });
            }
            expected_sequence = receipt.sequence.saturating_add(1);
            let revisions_are_consecutive = matches!(
                receipt.source_revision.next(),
                Ok(next) if next == receipt.committed_revision
            );
            let follows_previous_receipt = match previous_committed_revision {
                Some(previous) => previous == receipt.source_revision,
                None => true,
            };
            if receipt.source_revision >= receipt.committed_revision
                || receipt.committed_revision > self.revision
                || !revisions_are_consecutive
                || !follows_previous_receipt
            {
                issues.push(AnalysisPlanIssue::InvalidReceiptRevision {
                    sequence: receipt.sequence,
                });
            }
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
                        sequence: receipt.sequence,
                        expected,
                        actual: receipt.kind,
                    });
                }
                Some(_) => {}
                None => issues.push(AnalysisPlanIssue::DanglingReceiptInstance {
                    sequence: receipt.sequence,
                    id: receipt.instance_id,
                }),
            }
            if receipt.detail.trim().is_empty() {
                issues.push(AnalysisPlanIssue::EmptyReceiptDetail {
                    sequence: receipt.sequence,
                });
            }
            previous_committed_revision = Some(receipt.committed_revision);
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
                let instance = &mut candidate.instances[index];
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
    /// A referenced target cannot be disabled.
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
            AnalysisLifecycleCommand::Disable,
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
                    let dependents = candidate.dependents_of(id);
                    if !dependents.is_empty() {
                        return Err(AnalysisPlanError::ReferencedBy {
                            target: id,
                            dependents,
                        });
                    }
                }
                let instance = &mut candidate.instances[index];
                instance.enabled = enabled;
                instance.lifecycle = if enabled {
                    AnalysisLifecycleState::Draft
                } else {
                    AnalysisLifecycleState::Disabled
                };
                instance.modified_revision = revision;
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

    /// Bind or replace one exact prerequisite role.
    pub fn bind_dependency(
        &mut self,
        dependent: AnalysisInstanceId,
        prerequisite: AnalysisKind,
        target: AnalysisInstanceId,
    ) -> Result<AnalysisLifecycleReceipt, AnalysisPlanError> {
        let dependent_instance = self
            .instance(dependent)
            .ok_or(AnalysisPlanError::InstanceNotFound(dependent))?;
        let kind = dependent_instance.kind();
        let outcome = if dependent_instance.enabled() {
            AnalysisLifecycleState::Draft
        } else {
            AnalysisLifecycleState::Disabled
        };
        let ((), receipt) = self.transact(
            AnalysisLifecycleCommand::Dependency,
            dependent,
            Some(target),
            outcome,
            format!(
                "{} analysis {dependent} bound {target} as its {} prerequisite.",
                kind.label(),
                prerequisite.label()
            ),
            move |candidate, revision| {
                let dependent_index = candidate.index_of(dependent)?;
                candidate.ensure_editable(dependent_index)?;
                if !candidate.instances[dependent_index]
                    .kind
                    .prerequisites()
                    .contains(&prerequisite)
                {
                    return Err(AnalysisPlanError::UnexpectedDependencyRole {
                        dependent,
                        prerequisite,
                    });
                }
                if dependent == target {
                    return Err(AnalysisPlanError::SelfDependency { dependent });
                }
                let target_index = candidate.index_of(target).map_err(|_| {
                    AnalysisPlanError::DependencyTargetMissing { dependent, target }
                })?;
                let target_instance = &candidate.instances[target_index];
                if target_instance.kind != prerequisite {
                    return Err(AnalysisPlanError::DependencyTargetWrongKind {
                        dependent,
                        target,
                        expected: prerequisite,
                        actual: target_instance.kind,
                    });
                }
                if !target_instance.enabled {
                    return Err(AnalysisPlanError::DependencyTargetDisabled { dependent, target });
                }
                if target_index >= dependent_index {
                    return Err(AnalysisPlanError::DependencyTargetNotEarlier {
                        dependent,
                        target,
                    });
                }
                let instance = &mut candidate.instances[dependent_index];
                instance
                    .dependencies
                    .retain(|dependency| dependency.prerequisite != prerequisite);
                instance
                    .dependencies
                    .push(AnalysisDependency::new(prerequisite, target));
                instance
                    .dependencies
                    .sort_by_key(|dependency| dependency.prerequisite.legacy_index());
                instance.modified_revision = revision;
                instance.lifecycle = if instance.enabled {
                    AnalysisLifecycleState::Draft
                } else {
                    AnalysisLifecycleState::Disabled
                };
                Ok(())
            },
        )?;
        Ok(receipt)
    }

    /// Deterministically bind every required role to the latest matching,
    /// enabled instance that appears earlier. Unresolved roles remain explicit
    /// validation issues rather than making this convenience command partial.
    pub fn auto_bind_dependencies(
        &mut self,
        dependent: AnalysisInstanceId,
    ) -> Result<AnalysisLifecycleReceipt, AnalysisPlanError> {
        let dependent_instance = self
            .instance(dependent)
            .ok_or(AnalysisPlanError::InstanceNotFound(dependent))?;
        let kind = dependent_instance.kind();
        let outcome = if dependent_instance.enabled() {
            AnalysisLifecycleState::Draft
        } else {
            AnalysisLifecycleState::Disabled
        };
        let ((), receipt) = self.transact(
            AnalysisLifecycleCommand::Dependency,
            dependent,
            None,
            outcome,
            format!(
                "Dependency bindings for {} analysis {dependent} were refreshed from enabled earlier instances.",
                kind.label()
            ),
            move |candidate, revision| {
                let dependent_index = candidate.index_of(dependent)?;
                candidate.ensure_editable(dependent_index)?;
                let prerequisites = candidate.instances[dependent_index]
                    .kind
                    .prerequisites()
                    .to_vec();
                let dependencies = prerequisites
                    .into_iter()
                    .filter_map(|prerequisite| {
                        candidate.instances[..dependent_index]
                            .iter()
                            .rev()
                            .find(|instance| instance.enabled && instance.kind == prerequisite)
                            .map(|target| AnalysisDependency::new(prerequisite, target.id))
                    })
                    .collect();
                let instance = &mut candidate.instances[dependent_index];
                instance.dependencies = dependencies;
                instance.modified_revision = revision;
                instance.lifecycle = if instance.enabled {
                    AnalysisLifecycleState::Draft
                } else {
                    AnalysisLifecycleState::Disabled
                };
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
mod tests {
    use super::*;

    fn snapshot(plan: &SimulationPlan) -> String {
        serde_json::to_string(plan).expect("plan serializes")
    }

    #[test]
    fn lifecycle_vocabulary_matches_the_normative_mockup_ids() {
        let states = [
            (AnalysisLifecycleState::Absent, "absent"),
            (AnalysisLifecycleState::Draft, "draft"),
            (AnalysisLifecycleState::Invalid, "invalid"),
            (AnalysisLifecycleState::Ready, "ready"),
            (AnalysisLifecycleState::PreflightReady, "preflight-ready"),
            (AnalysisLifecycleState::Blocked, "blocked"),
            (AnalysisLifecycleState::Queued, "queued"),
            (AnalysisLifecycleState::Running, "running"),
            (AnalysisLifecycleState::Paused, "paused"),
            (AnalysisLifecycleState::Completed, "completed"),
            (AnalysisLifecycleState::Failed, "failed"),
            (AnalysisLifecycleState::Cancelled, "cancelled"),
            (AnalysisLifecycleState::Disabled, "disabled"),
            (AnalysisLifecycleState::Removed, "removed"),
            (AnalysisLifecycleState::SameState, "same-state"),
        ];
        for (state, stable_id) in states {
            assert_eq!(
                serde_json::to_string(&state).expect("state serializes"),
                format!("\"{stable_id}\"")
            );
            assert_eq!(state.to_string(), stable_id);
        }

        let commands = [
            (AnalysisLifecycleCommand::Insert, "insert"),
            (AnalysisLifecycleCommand::Edit, "edit"),
            (AnalysisLifecycleCommand::Clone, "clone"),
            (AnalysisLifecycleCommand::Disable, "disable"),
            (AnalysisLifecycleCommand::Reorder, "reorder"),
            (AnalysisLifecycleCommand::Dependency, "dependency"),
            (AnalysisLifecycleCommand::Validate, "validate"),
            (AnalysisLifecycleCommand::Preflight, "preflight"),
            (AnalysisLifecycleCommand::Execute, "execute"),
            (AnalysisLifecycleCommand::Remove, "remove"),
        ];
        for (command, stable_id) in commands {
            assert_eq!(
                serde_json::to_string(&command).expect("command serializes"),
                format!("\"{stable_id}\"")
            );
            assert_eq!(command.to_string(), stable_id);
        }
    }

    #[test]
    fn plan_diagnostics_render_as_concise_product_language() {
        let dependent = AnalysisInstanceId::new();
        let issue = AnalysisPlanIssue::MissingPrerequisite {
            dependent,
            prerequisite: AnalysisKind::OperatingPoint,
        };
        let text = issue.to_string();
        assert!(text.contains(&dependent.to_string()));
        assert!(text.contains("op prerequisite"));
        assert!(!text.contains("MissingPrerequisite"));

        let error =
            AnalysisPlanError::InvalidPlan(vec![issue, AnalysisPlanIssue::NoEnabledInstances]);
        let text = error.to_string();
        assert!(text.contains("Review 1 additional plan issue."));
        assert!(!text.contains('['));
        assert!(!text.contains("MissingPrerequisite"));
    }

    #[test]
    fn fresh_plan_has_one_enabled_transient() {
        let plan = SimulationPlan::new();
        assert_eq!(plan.revision(), ObjectRevision::INITIAL);
        assert_eq!(plan.instances().len(), 1);
        let instance = &plan.instances()[0];
        assert_eq!(instance.kind(), AnalysisKind::Transient);
        assert!(instance.enabled());
        assert!(plan.validation_issues().is_empty());
    }

    #[test]
    fn restore_relinquishes_execution_authority_without_changing_identity_or_revision() {
        for lifecycle in [
            AnalysisLifecycleState::Queued,
            AnalysisLifecycleState::Running,
            AnalysisLifecycleState::Paused,
        ] {
            let mut plan = SimulationPlan::new();
            let id = plan.instances[0].id;
            let revision = plan.revision;
            plan.instances[0].lifecycle = lifecycle;

            plan.prepare_after_restore();

            assert_eq!(plan.instances[0].id, id);
            assert_eq!(plan.revision, revision);
            assert_eq!(plan.instances[0].lifecycle, AnalysisLifecycleState::Draft);
            plan.edit(id, |_| ())
                .expect("restored instance is editable without stale runner authority");
        }
    }

    #[test]
    fn insertion_allows_missing_prerequisite_but_freeze_rejects_it() {
        let mut plan = SimulationPlan::new();
        let (ac, _) = plan.insert(AnalysisKind::Ac).expect("AC inserts");
        assert!(
            plan.validation_issues()
                .contains(&AnalysisPlanIssue::MissingPrerequisite {
                    dependent: ac,
                    prerequisite: AnalysisKind::OperatingPoint,
                })
        );
        plan.validate_structure()
            .expect("editable missing bindings are not corruption");
        assert!(matches!(
            plan.freeze(),
            Err(AnalysisPlanError::InvalidPlan(_))
        ));
    }

    #[test]
    fn deep_clone_is_inserted_after_source_and_edits_do_not_alias() {
        let mut plan = SimulationPlan::new();
        let source = plan.instances()[0].id();
        plan.edit(source, |draft| {
            let AnalysisDraft::Transient(draft) = draft else {
                panic!("expected transient");
            };
            draft.stop = "7u".to_owned();
        })
        .expect("source edits");
        let (clone, _) = plan.clone_instance(source).expect("clone succeeds");
        assert_eq!(plan.instances()[1].id(), clone);
        plan.edit(clone, |draft| {
            let AnalysisDraft::Transient(draft) = draft else {
                panic!("expected transient");
            };
            draft.stop = "9u".to_owned();
        })
        .expect("clone edits");
        let AnalysisDraft::Transient(source_draft) = plan.instance(source).unwrap().draft() else {
            panic!("expected transient");
        };
        assert_eq!(source_draft.stop, "7u");
    }

    #[test]
    fn failed_kind_changing_edit_is_fully_atomic() {
        let mut plan = SimulationPlan::new();
        let id = plan.instances()[0].id();
        let before = snapshot(&plan);
        let revision = plan.revision();
        let error = plan
            .edit(id, |draft| {
                *draft = AnalysisDraft::for_kind(AnalysisKind::Ac);
            })
            .expect_err("kind replacement must fail");
        assert!(matches!(error, AnalysisPlanError::DraftKindMismatch { .. }));
        assert_eq!(plan.revision(), revision);
        assert_eq!(snapshot(&plan), before);
    }

    #[test]
    fn referenced_target_cannot_be_disabled_removed_or_reordered_after_dependent() {
        let mut plan = SimulationPlan::new();
        let (op, _) = plan
            .insert_at(AnalysisKind::OperatingPoint, 0)
            .expect("OP inserts");
        let (ac, _) = plan.insert(AnalysisKind::Ac).expect("AC inserts");
        plan.bind_dependency(ac, AnalysisKind::OperatingPoint, op)
            .expect("binds");

        for action in [0, 1, 2] {
            let before = snapshot(&plan);
            let result = match action {
                0 => plan.set_enabled(op, false).map(|_| ()),
                1 => plan.remove(op, Vec::new()).map(|_| ()),
                _ => plan.reorder(op, plan.instances().len() - 1).map(|_| ()),
            };
            assert!(result.is_err());
            assert_eq!(snapshot(&plan), before);
        }
    }

    #[test]
    fn binding_rejects_self_dangling_wrong_disabled_and_later_targets() {
        let mut plan = SimulationPlan::new();
        let transient = plan.instances()[0].id();
        let (op, _) = plan
            .insert_at(AnalysisKind::OperatingPoint, 0)
            .expect("OP inserts");
        let (disabled_op, _) = plan
            .insert_at(AnalysisKind::OperatingPoint, 1)
            .expect("OP inserts");
        plan.set_enabled(disabled_op, false).expect("disables");
        let (ac, _) = plan.insert(AnalysisKind::Ac).expect("AC inserts");
        let (later_op, _) = plan
            .insert(AnalysisKind::OperatingPoint)
            .expect("OP inserts");

        assert!(matches!(
            plan.bind_dependency(ac, AnalysisKind::OperatingPoint, ac),
            Err(AnalysisPlanError::SelfDependency { .. })
        ));
        assert!(matches!(
            plan.bind_dependency(ac, AnalysisKind::OperatingPoint, AnalysisInstanceId::new()),
            Err(AnalysisPlanError::DependencyTargetMissing { .. })
        ));
        assert!(matches!(
            plan.bind_dependency(ac, AnalysisKind::OperatingPoint, transient),
            Err(AnalysisPlanError::DependencyTargetWrongKind { .. })
        ));
        assert!(matches!(
            plan.bind_dependency(ac, AnalysisKind::OperatingPoint, disabled_op),
            Err(AnalysisPlanError::DependencyTargetDisabled { .. })
        ));
        assert!(matches!(
            plan.bind_dependency(ac, AnalysisKind::OperatingPoint, later_op),
            Err(AnalysisPlanError::DependencyTargetNotEarlier { .. })
        ));
        plan.bind_dependency(ac, AnalysisKind::OperatingPoint, op)
            .expect("valid target binds");
    }

    #[test]
    fn auto_bind_chooses_latest_enabled_matching_predecessor() {
        let mut plan = SimulationPlan::new();
        let (first, _) = plan
            .insert_at(AnalysisKind::OperatingPoint, 0)
            .expect("OP inserts");
        let (latest, _) = plan
            .insert_at(AnalysisKind::OperatingPoint, 1)
            .expect("OP inserts");
        let (ac, _) = plan.insert(AnalysisKind::Ac).expect("AC inserts");
        let receipt = plan.auto_bind_dependencies(ac).expect("auto-bind succeeds");
        assert_eq!(receipt.command(), AnalysisLifecycleCommand::Dependency);
        assert_eq!(receipt.kind(), AnalysisKind::Ac);
        assert!(!receipt.detail().is_empty());
        assert_eq!(
            plan.instance(ac).unwrap().dependencies()[0].target(),
            latest
        );
        assert_ne!(first, latest);
    }

    #[test]
    fn remove_tombstone_prevents_identity_reuse_and_retains_runs() {
        let mut plan = SimulationPlan::new();
        let id = plan.instances()[0].id();
        let run = RunId::new();
        let receipt = plan.remove(id, vec![run, run]).expect("remove succeeds");
        assert_eq!(receipt.command(), AnalysisLifecycleCommand::Remove);
        assert_eq!(receipt.kind(), AnalysisKind::Transient);
        assert_eq!(receipt.outcome(), AnalysisLifecycleState::Removed);
        assert!(!receipt.detail().is_empty());
        let tombstone = &plan.tombstones()[0];
        assert_eq!(tombstone.id(), id);
        assert_eq!(tombstone.prior_run_ids(), &[run]);
        assert!(
            plan.validation_issues()
                .contains(&AnalysisPlanIssue::NoEnabledInstances)
        );
        plan.validate_structure()
            .expect("zero enabled instances remains an editable plan");
        let before = snapshot(&plan);
        assert!(matches!(
            plan.insert_draft_with_id(
                id,
                AnalysisDraft::for_kind(AnalysisKind::Transient),
                true,
                0
            ),
            Err(AnalysisPlanError::RetiredIdentity(retired)) if retired == id
        ));
        assert_eq!(snapshot(&plan), before);
    }

    #[test]
    fn supplied_plan_ids_and_instance_ids_are_preserved_and_validated() {
        let plan_id = SimulationPlanId::new();
        let instance_id = AnalysisInstanceId::new();
        let instance = AnalysisInstance::supplied(
            instance_id,
            AnalysisKind::Transient,
            AnalysisDraft::for_kind(AnalysisKind::Transient),
            true,
            Vec::new(),
            ObjectRevision::INITIAL,
            ObjectRevision::INITIAL,
        )
        .expect("supplied instance is valid");
        let plan = SimulationPlan::from_ordered_instances(
            plan_id,
            ObjectRevision::INITIAL,
            vec![instance],
        )
        .expect("supplied plan is valid");
        assert_eq!(plan.id(), plan_id);
        assert_eq!(plan.instances()[0].id(), instance_id);
    }

    #[test]
    fn supplied_corrupt_cycle_is_rejected_fail_closed() {
        let op_id = AnalysisInstanceId::new();
        let ac_id = AnalysisInstanceId::new();
        let op = AnalysisInstance::supplied(
            op_id,
            AnalysisKind::OperatingPoint,
            AnalysisDraft::for_kind(AnalysisKind::OperatingPoint),
            true,
            vec![AnalysisDependency::new(AnalysisKind::Ac, ac_id)],
            ObjectRevision::INITIAL,
            ObjectRevision::INITIAL,
        )
        .expect("local instance shape is valid before graph validation");
        let ac = AnalysisInstance::supplied(
            ac_id,
            AnalysisKind::Ac,
            AnalysisDraft::for_kind(AnalysisKind::Ac),
            true,
            vec![AnalysisDependency::new(AnalysisKind::OperatingPoint, op_id)],
            ObjectRevision::INITIAL,
            ObjectRevision::INITIAL,
        )
        .expect("local instance shape is valid before graph validation");

        let error = SimulationPlan::from_ordered_instances(
            SimulationPlanId::new(),
            ObjectRevision::INITIAL,
            vec![op, ac],
        )
        .expect_err("cyclic supplied graph must be rejected");
        let AnalysisPlanError::InvalidPlan(issues) = error else {
            panic!("expected structural validation diagnostics");
        };
        assert!(
            issues
                .iter()
                .any(|issue| matches!(issue, AnalysisPlanIssue::DependencyCycle { .. }))
        );
    }

    #[test]
    fn exhausted_revision_fails_without_any_mutation() {
        let revision = ObjectRevision::new(u64::MAX).expect("maximum revision is representable");
        let instance = AnalysisInstance::supplied(
            AnalysisInstanceId::new(),
            AnalysisKind::Transient,
            AnalysisDraft::for_kind(AnalysisKind::Transient),
            true,
            Vec::new(),
            revision,
            revision,
        )
        .expect("instance is locally valid");
        let mut plan = SimulationPlan::from_ordered_instances(
            SimulationPlanId::new(),
            revision,
            vec![instance],
        )
        .expect("maximum persisted revision is structurally valid");
        let before = snapshot(&plan);

        assert!(matches!(
            plan.insert(AnalysisKind::DcSweep),
            Err(AnalysisPlanError::Revision(RevisionError::Exhausted))
        ));
        assert_eq!(snapshot(&plan), before);
    }

    #[test]
    fn frozen_projection_is_deterministic_and_excludes_disabled_positions() {
        let mut plan = SimulationPlan::new();
        let transient = plan.instances()[0].id();
        let (disabled, insert_receipt) = plan.insert(AnalysisKind::DcSweep).expect("DC inserts");
        let disable_receipt = plan.set_enabled(disabled, false).expect("DC disables");
        assert_eq!(insert_receipt.sequence(), 1);
        assert_eq!(insert_receipt.command(), AnalysisLifecycleCommand::Insert);
        assert_eq!(insert_receipt.kind(), AnalysisKind::DcSweep);
        assert_eq!(insert_receipt.outcome(), AnalysisLifecycleState::Draft);
        assert!(!insert_receipt.detail().is_empty());
        assert_eq!(insert_receipt.source_revision(), ObjectRevision::INITIAL);
        assert_eq!(disable_receipt.sequence(), 2);
        assert_eq!(disable_receipt.command(), AnalysisLifecycleCommand::Disable);
        assert_eq!(disable_receipt.outcome(), AnalysisLifecycleState::Disabled);
        let persisted_receipt = serde_json::to_value(&disable_receipt).expect("receipt serializes");
        assert_eq!(persisted_receipt["command"], "disable");
        assert_eq!(persisted_receipt["kind"], "dc");
        assert_eq!(persisted_receipt["outcome"], "disabled");
        assert!(
            persisted_receipt["detail"]
                .as_str()
                .is_some_and(|detail| !detail.is_empty())
        );
        assert_eq!(
            insert_receipt.committed_revision(),
            disable_receipt.source_revision()
        );
        let frozen_a = plan.freeze().expect("plan freezes");
        let frozen_b = plan.freeze().expect("plan freezes again");
        assert_eq!(
            serde_json::to_string(&frozen_a).unwrap(),
            serde_json::to_string(&frozen_b).unwrap()
        );
        assert_eq!(frozen_a.instances().len(), 1);
        assert_eq!(frozen_a.instances()[0].id(), transient);
        assert_eq!(frozen_a.instances()[0].order(), 1);
    }
}
