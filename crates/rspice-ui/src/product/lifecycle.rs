use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::{
    CommandId, ContentDigest, DatasetId, ObjectRevision, ProjectId, ReleaseCandidateId,
    ResultDocumentId, RunId, ValidationCode, VerificationEvidenceId,
};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LifecycleError {
    #[error("invalid {object} transition from {from} using event {event}")]
    InvalidTransition {
        object: &'static str,
        from: &'static str,
        event: &'static str,
    },
    #[error("invalid value for {field}: {message}")]
    InvalidInput {
        field: &'static str,
        message: String,
    },
    #[error("{code}: transition is blocked by {missing:?}")]
    Blocked {
        code: ValidationCode,
        missing: Vec<String>,
    },
    #[error("lifecycle timestamp {next} precedes the last recorded timestamp {previous}")]
    TimestampRegression { previous: u64, next: u64 },
    #[error(transparent)]
    Revision(#[from] super::identity::RevisionError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum JobState {
    Queued,
    Dispatching,
    Running,
    Cancelling,
    Cancelled,
    Failed,
    Completed,
}

impl JobState {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Dispatching => "dispatching",
            Self::Running => "running",
            Self::Cancelling => "cancelling",
            Self::Cancelled => "cancelled",
            Self::Failed => "failed",
            Self::Completed => "completed",
        }
    }

    pub fn transition(self, event: JobEvent) -> Result<Self, LifecycleError> {
        match (self, event) {
            (Self::Queued, JobEvent::Dispatch) => Ok(Self::Dispatching),
            (Self::Dispatching, JobEvent::Start) => Ok(Self::Running),
            (Self::Queued | Self::Dispatching | Self::Running, JobEvent::RequestCancel) => {
                Ok(Self::Cancelling)
            }
            (Self::Cancelling, JobEvent::ConfirmCancelled) => Ok(Self::Cancelled),
            (Self::Dispatching | Self::Running | Self::Cancelling, JobEvent::Fail) => {
                Ok(Self::Failed)
            }
            (Self::Running, JobEvent::Complete) => Ok(Self::Completed),
            (Self::Cancelled | Self::Failed, JobEvent::Retry) => Ok(Self::Queued),
            _ => Err(LifecycleError::InvalidTransition {
                object: "job",
                from: self.label(),
                event: event.label(),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobEvent {
    Dispatch,
    Start,
    RequestCancel,
    ConfirmCancelled,
    Fail,
    Complete,
    Retry,
}

impl JobEvent {
    const fn label(self) -> &'static str {
        match self {
            Self::Dispatch => "dispatch",
            Self::Start => "start",
            Self::RequestCancel => "request-cancel",
            Self::ConfirmCancelled => "confirm-cancelled",
            Self::Fail => "fail",
            Self::Complete => "complete",
            Self::Retry => "retry",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RunState {
    Pending,
    Running,
    Cancelling,
    Cancelled,
    Failed,
    CompletedPartial,
    Completed,
}

impl RunState {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Running => "running",
            Self::Cancelling => "cancelling",
            Self::Cancelled => "cancelled",
            Self::Failed => "failed",
            Self::CompletedPartial => "completed-partial",
            Self::Completed => "completed",
        }
    }

    pub fn transition(self, event: RunEvent) -> Result<Self, LifecycleError> {
        match (self, event) {
            (Self::Pending, RunEvent::Start) => Ok(Self::Running),
            (Self::Pending | Self::Running, RunEvent::RequestCancel) => Ok(Self::Cancelling),
            (Self::Cancelling, RunEvent::ConfirmCancelled) => Ok(Self::Cancelled),
            (Self::Running | Self::Cancelling, RunEvent::Fail) => Ok(Self::Failed),
            (Self::Running, RunEvent::CompletePartial) => Ok(Self::CompletedPartial),
            (Self::Running, RunEvent::Complete) => Ok(Self::Completed),
            (Self::Cancelled | Self::Failed | Self::CompletedPartial, RunEvent::Retry) => {
                Ok(Self::Pending)
            }
            _ => Err(LifecycleError::InvalidTransition {
                object: "run",
                from: self.label(),
                event: event.label(),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunEvent {
    Start,
    RequestCancel,
    ConfirmCancelled,
    Fail,
    CompletePartial,
    Complete,
    Retry,
}

impl RunEvent {
    const fn label(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::RequestCancel => "request-cancel",
            Self::ConfirmCancelled => "confirm-cancelled",
            Self::Fail => "fail",
            Self::CompletePartial => "complete-partial",
            Self::Complete => "complete",
            Self::Retry => "retry",
        }
    }
}

/// Immutable identity of a completed dataset. No mutation methods are
/// exposed; new processing creates a new dataset and digest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "DatasetManifestData")]
pub struct DatasetManifest {
    schema_version: u16,
    id: DatasetId,
    run_id: RunId,
    input_digest: ContentDigest,
    content_digest: ContentDigest,
    analysis_ids: Vec<String>,
    created_at_unix_ms: u64,
}

#[derive(Deserialize)]
struct DatasetManifestData {
    schema_version: u16,
    id: DatasetId,
    run_id: RunId,
    input_digest: ContentDigest,
    content_digest: ContentDigest,
    analysis_ids: Vec<String>,
    created_at_unix_ms: u64,
}

impl DatasetManifest {
    pub const SCHEMA_VERSION: u16 = 1;

    pub fn new(
        id: DatasetId,
        run_id: RunId,
        input_digest: ContentDigest,
        content_digest: ContentDigest,
        analysis_ids: Vec<String>,
        created_at_unix_ms: u64,
    ) -> Result<Self, LifecycleError> {
        validate_analysis_ids(&analysis_ids)?;
        Ok(Self {
            schema_version: Self::SCHEMA_VERSION,
            id,
            run_id,
            input_digest,
            content_digest,
            analysis_ids,
            created_at_unix_ms,
        })
    }

    #[must_use]
    pub const fn id(&self) -> DatasetId {
        self.id
    }

    #[must_use]
    pub const fn run_id(&self) -> RunId {
        self.run_id
    }

    #[must_use]
    pub const fn content_digest(&self) -> ContentDigest {
        self.content_digest
    }

    #[must_use]
    pub fn analysis_ids(&self) -> &[String] {
        &self.analysis_ids
    }
}

impl TryFrom<DatasetManifestData> for DatasetManifest {
    type Error = LifecycleError;

    fn try_from(data: DatasetManifestData) -> Result<Self, Self::Error> {
        if data.schema_version != Self::SCHEMA_VERSION {
            return Err(LifecycleError::InvalidInput {
                field: "dataset.schema-version",
                message: format!(
                    "unsupported dataset schema {}; expected {}",
                    data.schema_version,
                    Self::SCHEMA_VERSION
                ),
            });
        }
        Self::new(
            data.id,
            data.run_id,
            data.input_digest,
            data.content_digest,
            data.analysis_ids,
            data.created_at_unix_ms,
        )
    }
}

fn validate_analysis_ids(analysis_ids: &[String]) -> Result<(), LifecycleError> {
    if analysis_ids.is_empty() {
        return Err(LifecycleError::InvalidInput {
            field: "dataset.analysis-ids",
            message: "a dataset must declare at least one analysis".to_owned(),
        });
    }
    let mut seen = HashSet::with_capacity(analysis_ids.len());
    for id in analysis_ids {
        CommandId::new(id.clone()).map_err(|error| LifecycleError::InvalidInput {
            field: "dataset.analysis-ids",
            message: error.to_string(),
        })?;
        if !seen.insert(id) {
            return Err(LifecycleError::InvalidInput {
                field: "dataset.analysis-ids",
                message: format!("duplicate analysis ID {id}"),
            });
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DatasetBinding {
    pub dataset_id: DatasetId,
    pub content_digest: ContentDigest,
}

impl DatasetBinding {
    #[must_use]
    pub const fn new(dataset_id: DatasetId, content_digest: ContentDigest) -> Self {
        Self {
            dataset_id,
            content_digest,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResultDocumentLayout {
    SinglePane,
    TwoLinkedPanes,
    TwoByTwoEngineeringSheet,
    FreeformReviewPage,
}

/// Persistent presentation over immutable datasets. Dataset rebinding and
/// layout changes advance one object revision and never alter source samples.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "ResultDocumentData")]
pub struct ResultDocument {
    schema_version: u16,
    id: ResultDocumentId,
    project_id: ProjectId,
    revision: ObjectRevision,
    family_id: String,
    layout: ResultDocumentLayout,
    dataset_bindings: Vec<DatasetBinding>,
    updated_at_unix_ms: u64,
}

#[derive(Deserialize)]
struct ResultDocumentData {
    schema_version: u16,
    id: ResultDocumentId,
    project_id: ProjectId,
    revision: ObjectRevision,
    family_id: String,
    layout: ResultDocumentLayout,
    dataset_bindings: Vec<DatasetBinding>,
    updated_at_unix_ms: u64,
}

impl ResultDocument {
    pub const SCHEMA_VERSION: u16 = 1;

    pub fn new(
        project_id: ProjectId,
        family_id: impl Into<String>,
        layout: ResultDocumentLayout,
        dataset_bindings: Vec<DatasetBinding>,
        timestamp_unix_ms: u64,
    ) -> Result<Self, LifecycleError> {
        let family_id = family_id.into();
        validate_result_document(&family_id, &dataset_bindings)?;
        Ok(Self {
            schema_version: Self::SCHEMA_VERSION,
            id: ResultDocumentId::new(),
            project_id,
            revision: ObjectRevision::INITIAL,
            family_id,
            layout,
            dataset_bindings,
            updated_at_unix_ms: timestamp_unix_ms,
        })
    }

    #[must_use]
    pub const fn id(&self) -> ResultDocumentId {
        self.id
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub fn family_id(&self) -> &str {
        &self.family_id
    }

    #[must_use]
    pub fn dataset_bindings(&self) -> &[DatasetBinding] {
        &self.dataset_bindings
    }

    pub fn rebind_datasets(
        &mut self,
        dataset_bindings: Vec<DatasetBinding>,
        timestamp_unix_ms: u64,
    ) -> Result<ObjectRevision, LifecycleError> {
        validate_timestamp(self.updated_at_unix_ms, timestamp_unix_ms)?;
        validate_result_document(&self.family_id, &dataset_bindings)?;
        self.dataset_bindings = dataset_bindings;
        self.advance(timestamp_unix_ms)
    }

    pub fn change_layout(
        &mut self,
        layout: ResultDocumentLayout,
        timestamp_unix_ms: u64,
    ) -> Result<ObjectRevision, LifecycleError> {
        validate_timestamp(self.updated_at_unix_ms, timestamp_unix_ms)?;
        self.layout = layout;
        self.advance(timestamp_unix_ms)
    }

    fn advance(&mut self, timestamp_unix_ms: u64) -> Result<ObjectRevision, LifecycleError> {
        self.revision = self.revision.next()?;
        self.updated_at_unix_ms = timestamp_unix_ms;
        Ok(self.revision)
    }
}

impl TryFrom<ResultDocumentData> for ResultDocument {
    type Error = LifecycleError;

    fn try_from(data: ResultDocumentData) -> Result<Self, Self::Error> {
        if data.schema_version != Self::SCHEMA_VERSION {
            return Err(LifecycleError::InvalidInput {
                field: "result-document.schema-version",
                message: format!("unsupported result-document schema {}", data.schema_version),
            });
        }
        validate_result_document(&data.family_id, &data.dataset_bindings)?;
        Ok(Self {
            schema_version: data.schema_version,
            id: data.id,
            project_id: data.project_id,
            revision: data.revision,
            family_id: data.family_id,
            layout: data.layout,
            dataset_bindings: data.dataset_bindings,
            updated_at_unix_ms: data.updated_at_unix_ms,
        })
    }
}

fn validate_result_document(
    family_id: &str,
    bindings: &[DatasetBinding],
) -> Result<(), LifecycleError> {
    CommandId::new(family_id.to_owned()).map_err(|error| LifecycleError::InvalidInput {
        field: "result-document.family",
        message: error.to_string(),
    })?;
    if bindings.is_empty() {
        return Err(LifecycleError::InvalidInput {
            field: "result-document.dataset-bindings",
            message: "at least one immutable dataset binding is required".to_owned(),
        });
    }
    let unique: HashSet<_> = bindings.iter().map(|binding| binding.dataset_id).collect();
    if unique.len() != bindings.len() {
        return Err(LifecycleError::InvalidInput {
            field: "result-document.dataset-bindings",
            message: "duplicate dataset bindings are not allowed".to_owned(),
        });
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VerificationDisposition {
    Passed,
    Failed,
    AcceptedException,
    NotApplicable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationDispositionEntry {
    sequence: u64,
    disposition: VerificationDisposition,
    actor: String,
    reason: String,
    timestamp_unix_ms: u64,
}

impl VerificationDispositionEntry {
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub const fn disposition(&self) -> VerificationDisposition {
        self.disposition
    }
}

/// Source-owned verification truth. Dispositions can only be appended; release
/// closure consumes this record but has no mutation API for it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "VerificationEvidenceData")]
pub struct VerificationEvidence {
    schema_version: u16,
    id: VerificationEvidenceId,
    dataset: DatasetBinding,
    source_digest: ContentDigest,
    dispositions: Vec<VerificationDispositionEntry>,
}

#[derive(Deserialize)]
struct VerificationEvidenceData {
    schema_version: u16,
    id: VerificationEvidenceId,
    dataset: DatasetBinding,
    source_digest: ContentDigest,
    dispositions: Vec<VerificationDispositionEntry>,
}

impl VerificationEvidence {
    pub const SCHEMA_VERSION: u16 = 1;

    #[must_use]
    pub fn new(dataset: DatasetBinding, source_digest: ContentDigest) -> Self {
        Self {
            schema_version: Self::SCHEMA_VERSION,
            id: VerificationEvidenceId::new(),
            dataset,
            source_digest,
            dispositions: Vec::new(),
        }
    }

    pub fn append_disposition(
        &mut self,
        disposition: VerificationDisposition,
        actor: impl Into<String>,
        reason: impl Into<String>,
        timestamp_unix_ms: u64,
    ) -> Result<u64, LifecycleError> {
        let actor = actor.into();
        let reason = reason.into();
        if actor.trim().is_empty() {
            return Err(LifecycleError::InvalidInput {
                field: "verification-evidence.actor",
                message: "an attributable actor is required".to_owned(),
            });
        }
        if reason.trim().is_empty() {
            return Err(LifecycleError::InvalidInput {
                field: "verification-evidence.reason",
                message: "a disposition reason is required".to_owned(),
            });
        }
        if let Some(last) = self.dispositions.last() {
            validate_timestamp(last.timestamp_unix_ms, timestamp_unix_ms)?;
        }
        let sequence = self
            .dispositions
            .len()
            .checked_add(1)
            .and_then(|value| u64::try_from(value).ok())
            .ok_or_else(|| LifecycleError::InvalidInput {
                field: "verification-evidence.dispositions",
                message: "disposition sequence space is exhausted".to_owned(),
            })?;
        self.dispositions.push(VerificationDispositionEntry {
            sequence,
            disposition,
            actor,
            reason,
            timestamp_unix_ms,
        });
        Ok(sequence)
    }

    #[must_use]
    pub fn dispositions(&self) -> &[VerificationDispositionEntry] {
        &self.dispositions
    }
}

impl TryFrom<VerificationEvidenceData> for VerificationEvidence {
    type Error = LifecycleError;

    fn try_from(data: VerificationEvidenceData) -> Result<Self, Self::Error> {
        if data.schema_version != Self::SCHEMA_VERSION {
            return Err(LifecycleError::InvalidInput {
                field: "verification-evidence.schema-version",
                message: format!(
                    "unsupported verification evidence schema {}",
                    data.schema_version
                ),
            });
        }
        for (index, entry) in data.dispositions.iter().enumerate() {
            let expected = u64::try_from(index + 1).map_err(|_| LifecycleError::InvalidInput {
                field: "verification-evidence.dispositions",
                message: "disposition sequence is too large".to_owned(),
            })?;
            if entry.sequence != expected
                || entry.actor.trim().is_empty()
                || entry.reason.trim().is_empty()
            {
                return Err(LifecycleError::InvalidInput {
                    field: "verification-evidence.dispositions",
                    message:
                        "dispositions must be attributable, reasoned, and consecutively sequenced"
                            .to_owned(),
                });
            }
            if index > 0 {
                validate_timestamp(
                    data.dispositions[index - 1].timestamp_unix_ms,
                    entry.timestamp_unix_ms,
                )?;
            }
        }
        Ok(Self {
            schema_version: data.schema_version,
            id: data.id,
            dataset: data.dataset,
            source_digest: data.source_digest,
            dispositions: data.dispositions,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReleaseCandidateState {
    Draft,
    Frozen,
    Promoted,
    Superseded,
    RevocationPending,
    Revoked,
    RolledBack,
}

impl ReleaseCandidateState {
    const fn label(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Frozen => "frozen",
            Self::Promoted => "promoted",
            Self::Superseded => "superseded",
            Self::RevocationPending => "revocation-pending",
            Self::Revoked => "revoked",
            Self::RolledBack => "rolled-back",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseGateSnapshot {
    pub technical_gates_current: bool,
    pub approvals_complete: bool,
    pub report_binding_current: bool,
    pub distribution_approved: bool,
    pub actor_reauthenticated: bool,
    pub online: bool,
}

impl ReleaseGateSnapshot {
    #[must_use]
    pub fn missing_promotion_requirements(self) -> Vec<String> {
        let mut missing = Vec::new();
        for (passed, label) in [
            (self.technical_gates_current, "current technical gates"),
            (self.approvals_complete, "independent approvals"),
            (self.report_binding_current, "current report binding"),
            (self.distribution_approved, "approved distribution"),
            (self.actor_reauthenticated, "current re-authentication"),
            (self.online, "online authority check"),
        ] {
            if !passed {
                missing.push(label.to_owned());
            }
        }
        missing
    }
}

/// Fail-closed release candidate. Freezing permanently binds scope and source
/// gate digests. Later methods can change lifecycle state but cannot edit those
/// inputs or technical evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "ReleaseCandidateData")]
pub struct ReleaseCandidate {
    schema_version: u16,
    id: ReleaseCandidateId,
    project_id: ProjectId,
    revision: ObjectRevision,
    state: ReleaseCandidateState,
    scope_digest: Option<ContentDigest>,
    source_gate_digest: Option<ContentDigest>,
    last_event_unix_ms: u64,
}

#[derive(Deserialize)]
struct ReleaseCandidateData {
    schema_version: u16,
    id: ReleaseCandidateId,
    project_id: ProjectId,
    revision: ObjectRevision,
    state: ReleaseCandidateState,
    scope_digest: Option<ContentDigest>,
    source_gate_digest: Option<ContentDigest>,
    last_event_unix_ms: u64,
}

impl ReleaseCandidate {
    pub const SCHEMA_VERSION: u16 = 1;

    #[must_use]
    pub fn new(project_id: ProjectId, timestamp_unix_ms: u64) -> Self {
        Self {
            schema_version: Self::SCHEMA_VERSION,
            id: ReleaseCandidateId::new(),
            project_id,
            revision: ObjectRevision::INITIAL,
            state: ReleaseCandidateState::Draft,
            scope_digest: None,
            source_gate_digest: None,
            last_event_unix_ms: timestamp_unix_ms,
        }
    }

    #[must_use]
    pub const fn state(&self) -> ReleaseCandidateState {
        self.state
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub const fn scope_digest(&self) -> Option<ContentDigest> {
        self.scope_digest
    }

    pub fn freeze(
        &mut self,
        scope_digest: ContentDigest,
        source_gate_digest: ContentDigest,
        timestamp_unix_ms: u64,
    ) -> Result<ObjectRevision, LifecycleError> {
        self.require_state(ReleaseCandidateState::Draft, "freeze")?;
        self.scope_digest = Some(scope_digest);
        self.source_gate_digest = Some(source_gate_digest);
        self.transition_to(ReleaseCandidateState::Frozen, timestamp_unix_ms)
    }

    pub fn promote(
        &mut self,
        gates: ReleaseGateSnapshot,
        timestamp_unix_ms: u64,
    ) -> Result<ObjectRevision, LifecycleError> {
        self.require_state(ReleaseCandidateState::Frozen, "promote")?;
        let missing = gates.missing_promotion_requirements();
        if !missing.is_empty() {
            return Err(LifecycleError::Blocked {
                code: ValidationCode::new("GUI-RELEASE-PROMOTION-BLOCKED")
                    .expect("static validation code"),
                missing,
            });
        }
        self.transition_to(ReleaseCandidateState::Promoted, timestamp_unix_ms)
    }

    pub fn supersede(&mut self, timestamp_unix_ms: u64) -> Result<ObjectRevision, LifecycleError> {
        self.require_state(ReleaseCandidateState::Promoted, "supersede")?;
        self.transition_to(ReleaseCandidateState::Superseded, timestamp_unix_ms)
    }

    pub fn request_revocation(
        &mut self,
        timestamp_unix_ms: u64,
    ) -> Result<ObjectRevision, LifecycleError> {
        self.require_state(ReleaseCandidateState::Promoted, "request-revocation")?;
        self.transition_to(ReleaseCandidateState::RevocationPending, timestamp_unix_ms)
    }

    pub fn finalize_revocation(
        &mut self,
        independent_authority: bool,
        timestamp_unix_ms: u64,
    ) -> Result<ObjectRevision, LifecycleError> {
        self.require_state(
            ReleaseCandidateState::RevocationPending,
            "finalize-revocation",
        )?;
        if !independent_authority {
            return Err(LifecycleError::Blocked {
                code: ValidationCode::new("GUI-RELEASE-REVOCATION-AUTHORITY")
                    .expect("static validation code"),
                missing: vec!["independent final authority".to_owned()],
            });
        }
        self.transition_to(ReleaseCandidateState::Revoked, timestamp_unix_ms)
    }

    pub fn record_rollback(
        &mut self,
        timestamp_unix_ms: u64,
    ) -> Result<ObjectRevision, LifecycleError> {
        if !matches!(
            self.state,
            ReleaseCandidateState::Promoted | ReleaseCandidateState::Superseded
        ) {
            return Err(self.invalid_transition("record-rollback"));
        }
        self.transition_to(ReleaseCandidateState::RolledBack, timestamp_unix_ms)
    }

    fn require_state(
        &self,
        required: ReleaseCandidateState,
        event: &'static str,
    ) -> Result<(), LifecycleError> {
        if self.state == required {
            Ok(())
        } else {
            Err(self.invalid_transition(event))
        }
    }

    fn invalid_transition(&self, event: &'static str) -> LifecycleError {
        LifecycleError::InvalidTransition {
            object: "release-candidate",
            from: self.state.label(),
            event,
        }
    }

    fn transition_to(
        &mut self,
        state: ReleaseCandidateState,
        timestamp_unix_ms: u64,
    ) -> Result<ObjectRevision, LifecycleError> {
        validate_timestamp(self.last_event_unix_ms, timestamp_unix_ms)?;
        self.state = state;
        self.revision = self.revision.next()?;
        self.last_event_unix_ms = timestamp_unix_ms;
        Ok(self.revision)
    }
}

impl TryFrom<ReleaseCandidateData> for ReleaseCandidate {
    type Error = LifecycleError;

    fn try_from(data: ReleaseCandidateData) -> Result<Self, Self::Error> {
        if data.schema_version != Self::SCHEMA_VERSION {
            return Err(LifecycleError::InvalidInput {
                field: "release-candidate.schema-version",
                message: format!(
                    "unsupported release candidate schema {}",
                    data.schema_version
                ),
            });
        }
        let frozen = data.state != ReleaseCandidateState::Draft;
        if frozen != (data.scope_digest.is_some() && data.source_gate_digest.is_some()) {
            return Err(LifecycleError::InvalidInput {
                field: "release-candidate.scope",
                message: "non-draft candidates require both frozen scope and source-gate digests"
                    .to_owned(),
            });
        }
        Ok(Self {
            schema_version: data.schema_version,
            id: data.id,
            project_id: data.project_id,
            revision: data.revision,
            state: data.state,
            scope_digest: data.scope_digest,
            source_gate_digest: data.source_gate_digest,
            last_event_unix_ms: data.last_event_unix_ms,
        })
    }
}

fn validate_timestamp(previous: u64, next: u64) -> Result<(), LifecycleError> {
    if next < previous {
        Err(LifecycleError::TimestampRegression { previous, next })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{FieldId, JobId, ProductObjectKind};

    fn digest(byte: u8) -> ContentDigest {
        ContentDigest::from_bytes([byte; 32])
    }

    #[test]
    fn job_and_run_state_machines_reject_skipped_or_terminal_transitions() {
        assert_eq!(
            JobState::Queued
                .transition(JobEvent::Dispatch)
                .and_then(|state| state.transition(JobEvent::Start))
                .and_then(|state| state.transition(JobEvent::Complete)),
            Ok(JobState::Completed)
        );
        assert!(JobState::Queued.transition(JobEvent::Complete).is_err());
        assert!(JobState::Completed.transition(JobEvent::Retry).is_err());

        assert_eq!(
            RunState::Pending
                .transition(RunEvent::Start)
                .and_then(|state| state.transition(RunEvent::CompletePartial))
                .and_then(|state| state.transition(RunEvent::Retry)),
            Ok(RunState::Pending)
        );
        assert!(
            RunState::Completed
                .transition(RunEvent::RequestCancel)
                .is_err()
        );
    }

    #[test]
    fn dataset_manifest_is_immutable_and_validates_deserialization() {
        let dataset = DatasetManifest::new(
            DatasetId::new(),
            RunId::new(),
            digest(1),
            digest(2),
            vec!["tran".to_owned(), "ac".to_owned()],
            20,
        )
        .expect("valid dataset");
        let json = serde_json::to_string(&dataset).expect("serialize dataset");
        let restored: DatasetManifest = serde_json::from_str(&json).expect("restore dataset");
        assert_eq!(restored, dataset);

        let duplicate = json.replace("\"ac\"", "\"tran\"");
        assert!(serde_json::from_str::<DatasetManifest>(&duplicate).is_err());
        let future = json.replace("\"schema_version\":1", "\"schema_version\":99");
        assert!(serde_json::from_str::<DatasetManifest>(&future).is_err());
    }

    #[test]
    fn result_document_changes_are_versioned_without_mutating_datasets() {
        let binding_a = DatasetBinding::new(DatasetId::new(), digest(3));
        let binding_b = DatasetBinding::new(DatasetId::new(), digest(4));
        let mut document = ResultDocument::new(
            ProjectId::new(),
            "viewer-waveform",
            ResultDocumentLayout::TwoLinkedPanes,
            vec![binding_a],
            10,
        )
        .expect("valid document");

        assert_eq!(
            document
                .rebind_datasets(vec![binding_a, binding_b], 11)
                .expect("rebind"),
            ObjectRevision::new(2).expect("revision")
        );
        assert_eq!(document.dataset_bindings(), &[binding_a, binding_b]);
        assert!(
            document
                .rebind_datasets(vec![binding_a, binding_a], 12)
                .is_err()
        );
        assert_eq!(
            document.revision().get(),
            2,
            "failed mutation changed revision"
        );
    }

    #[test]
    fn verification_dispositions_are_append_only_and_attributable() {
        let mut evidence =
            VerificationEvidence::new(DatasetBinding::new(DatasetId::new(), digest(5)), digest(6));
        assert_eq!(
            evidence
                .append_disposition(
                    VerificationDisposition::Failed,
                    "engine",
                    "limit exceeded",
                    10
                )
                .expect("append failure"),
            1
        );
        assert_eq!(
            evidence
                .append_disposition(
                    VerificationDisposition::AcceptedException,
                    "reviewer-17",
                    "bounded exception EX-19",
                    11,
                )
                .expect("append review"),
            2
        );
        assert!(
            evidence
                .append_disposition(VerificationDisposition::Passed, "", "", 12)
                .is_err()
        );
        assert_eq!(evidence.dispositions().len(), 2);
    }

    #[test]
    fn release_candidate_is_fail_closed_and_frozen_scope_never_changes() {
        let mut candidate = ReleaseCandidate::new(ProjectId::new(), 1);
        candidate.freeze(digest(7), digest(8), 2).expect("freeze");
        let frozen_scope = candidate.scope_digest();

        let blocked = candidate.promote(
            ReleaseGateSnapshot {
                technical_gates_current: false,
                approvals_complete: true,
                report_binding_current: true,
                distribution_approved: true,
                actor_reauthenticated: true,
                online: true,
            },
            3,
        );
        assert!(matches!(blocked, Err(LifecycleError::Blocked { .. })));
        assert_eq!(candidate.state(), ReleaseCandidateState::Frozen);
        assert_eq!(candidate.scope_digest(), frozen_scope);

        candidate
            .promote(
                ReleaseGateSnapshot {
                    technical_gates_current: true,
                    approvals_complete: true,
                    report_binding_current: true,
                    distribution_approved: true,
                    actor_reauthenticated: true,
                    online: true,
                },
                4,
            )
            .expect("eligible promotion");
        assert_eq!(candidate.state(), ReleaseCandidateState::Promoted);
        assert!(candidate.freeze(digest(9), digest(9), 5).is_err());
        assert_eq!(candidate.scope_digest(), frozen_scope);
    }

    #[test]
    fn release_candidate_rejects_tampered_persistence() {
        let candidate = ReleaseCandidate::new(ProjectId::new(), 1);
        let json = serde_json::to_string(&candidate).expect("serialize candidate");
        let tampered = json.replace("\"draft\"", "\"promoted\"");
        assert!(serde_json::from_str::<ReleaseCandidate>(&tampered).is_err());
    }

    #[test]
    fn canonical_kind_names_remain_bound_to_lifecycle_records() {
        assert_eq!(ProductObjectKind::Job.stable_id(), "job");
        assert_eq!(ProductObjectKind::Run.stable_id(), "run");
        assert_eq!(ProductObjectKind::Dataset.stable_id(), "dataset");
        assert_eq!(
            ProductObjectKind::ResultDocument.stable_id(),
            "result-document"
        );
        assert_eq!(
            FieldId::new("release-candidate.scope-attestation")
                .expect("canonical field")
                .as_str(),
            "release-candidate.scope-attestation"
        );
        let _job_id = JobId::new();
    }
}
