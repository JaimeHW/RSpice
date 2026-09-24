//! What the plan refuses, and what it reports about itself.
//!
//! Two diagnostic vocabularies, kept together and kept apart from the plan.
//! [`AnalysisPlanIssue`] is a *finding*: a deterministic statement about a plan
//! that already exists, produced by validation and serialized into a project so
//! a reload reports what the save reported. [`AnalysisPlanError`] is a
//! *refusal*: a command the plan declined, leaving the receiver unchanged.
//!
//! Neither reads a field of the plan. They are the words the plan speaks, and
//! separating them from the machinery that speaks them is what keeps a file
//! that owns identity, dependency edges and lifecycle transactions from also
//! owning four hundred lines of formatting.

use super::*;
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
    InvalidInstanceName {
        id: AnalysisInstanceId,
    },
    DuplicateInstanceName {
        id: AnalysisInstanceId,
        name: String,
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
    /// Whether this finding is about the plan's *shape* rather than about what
    /// it has been configured to do. Visible to the plan because that is what
    /// separates a diagnostic a save must refuse from one a page may show.
    pub(super) const fn is_structural(&self) -> bool {
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
            Self::InvalidInstanceName { id } => {
                write!(formatter, "Analysis {id} has an unusable name.")
            }
            Self::DuplicateInstanceName { id, name } => write!(
                formatter,
                "Analysis {id} shares the name \"{name}\" with another analysis in this plan."
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
    EmptyInstanceName,
    InstanceNameAlreadyDefault,
    InstanceNameTooLong {
        limit: usize,
    },
    InstanceNameNotSingleLine,
    InstanceNameTaken {
        name: String,
        holder: AnalysisInstanceId,
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
    NumericOverrideNotApplicable {
        id: AnalysisInstanceId,
        kind: AnalysisKind,
        option: NumericOverrideOption,
        reason: &'static str,
    },
    RunSetParticipationInvalid {
        id: AnalysisInstanceId,
        kind: AnalysisKind,
        reason: String,
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
            Self::EmptyInstanceName => {
                formatter.write_str("An analysis name cannot be empty or only whitespace.")
            }
            Self::InstanceNameAlreadyDefault => formatter
                .write_str("This analysis has no name of its own; it already shows its kind."),
            Self::InstanceNameTooLong { limit } => write!(
                formatter,
                "An analysis name cannot exceed {limit} characters."
            ),
            Self::InstanceNameNotSingleLine => {
                formatter.write_str("An analysis name must be a single line.")
            }
            Self::InstanceNameTaken { name, holder } => write!(
                formatter,
                "Another analysis in this plan is already shown as \"{name}\" ({holder})."
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
            Self::NumericOverrideNotApplicable {
                id,
                kind,
                option,
                reason,
            } => write!(
                formatter,
                "{} analysis {id} cannot carry {}: {reason}.",
                kind.label(),
                option.key()
            ),
            Self::RunSetParticipationInvalid { id, kind, reason } => write!(
                formatter,
                "{} analysis {id} cannot take that run-set participation: {reason}",
                kind.label()
            ),
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
