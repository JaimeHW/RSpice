//! Durable validated-save history for schematic documents.
//!
//! Each record owns the exact restorable design state that was validated and
//! canonically saved. Runtime editor state, selection, undo history, and view
//! transforms never enter the journal. The hash-linked records are validated
//! on project load, so corrupt or reordered history fails closed.

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use thiserror::Error;
use uuid::Uuid;

use crate::product::ContentDigest;

use super::{
    Bus, BusTap, Component, DesignNote, DocumentationShape, Junction, NetLabel,
    SchematicDocumentPolicy, SchematicSnapshot, SchematicState, Wire, WireConnection,
};

pub const MAX_VALIDATED_REVISION_NOTE_LEN: usize = 240;
pub const MAX_VALIDATED_REVISION_IDENTITY_LEN: usize = 512;
pub const MAX_ADVISORY_DISPOSITION_REASON_LEN: usize = 1_024;
pub const VALIDATED_REVISION_JOURNAL_SCHEMA_VERSION: u32 = 1;

const fn current_journal_schema_version() -> u32 {
    VALIDATED_REVISION_JOURNAL_SCHEMA_VERSION
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ValidatedSchematicRevisionId(Uuid);

impl ValidatedSchematicRevisionId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    #[must_use]
    pub const fn as_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for ValidatedSchematicRevisionId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidationFindingCounts {
    pub blockers: u32,
    pub advisories: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdvisoryDispositionKind {
    AcceptedForRevision,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdvisoryDisposition {
    pub finding_identity: String,
    pub kind: AdvisoryDispositionKind,
    pub reason: String,
}

impl AdvisoryDisposition {
    pub fn accepted(finding_identity: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            finding_identity: finding_identity.into(),
            kind: AdvisoryDispositionKind::AcceptedForRevision,
            reason: reason.into(),
        }
    }

    fn validate(&self) -> Result<(), ValidatedRevisionError> {
        validate_identity("advisory finding", &self.finding_identity)?;
        let reason = self.reason.trim();
        if reason.is_empty() {
            return Err(ValidatedRevisionError::EmptyAdvisoryReason);
        }
        if reason.chars().count() > MAX_ADVISORY_DISPOSITION_REASON_LEN {
            return Err(ValidatedRevisionError::AdvisoryReasonTooLong);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidatedRevisionDependency {
    pub identity: String,
    pub digest: ContentDigest,
}

impl ValidatedRevisionDependency {
    pub fn new(identity: impl Into<String>, digest: ContentDigest) -> Self {
        Self {
            identity: identity.into(),
            digest,
        }
    }

    fn validate(&self) -> Result<(), ValidatedRevisionError> {
        validate_identity("dependency", &self.identity)
    }
}

#[derive(Debug, Clone)]
pub struct ValidatedRevisionRequest {
    pub project_id: String,
    pub project_revision: u64,
    pub view_identity: String,
    pub revision_note: String,
    pub author: String,
    pub validation_receipt_digest: ContentDigest,
    pub finding_counts: ValidationFindingCounts,
    pub dependencies: Vec<ValidatedRevisionDependency>,
    pub advisory_dispositions: Vec<AdvisoryDisposition>,
}

impl ValidatedRevisionRequest {
    fn validate(&self) -> Result<(), ValidatedRevisionError> {
        validate_project_id(&self.project_id)?;
        validate_identity("cell/view", &self.view_identity)?;
        validate_identity("author", &self.author)?;
        let note = self.revision_note.trim();
        if note.is_empty() {
            return Err(ValidatedRevisionError::EmptyRevisionNote);
        }
        if note.chars().count() > MAX_VALIDATED_REVISION_NOTE_LEN {
            return Err(ValidatedRevisionError::RevisionNoteTooLong);
        }
        if self.finding_counts.blockers != 0 {
            return Err(ValidatedRevisionError::BlockingFindings(
                self.finding_counts.blockers,
            ));
        }
        validate_dependencies(&self.dependencies)?;
        validate_dispositions(&self.advisory_dispositions, self.finding_counts.advisories)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ValidatedRevisionKind {
    AcceptedBaseline,
    ValidatedSave,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ValidatedRevisionSnapshot {
    grid_size: i32,
    document_policy: SchematicDocumentPolicy,
    components: Vec<Component>,
    wires: Vec<Wire>,
    buses: Vec<Bus>,
    bus_taps: Vec<BusTap>,
    junctions: Vec<Junction>,
    net_labels: Vec<NetLabel>,
    design_notes: Vec<DesignNote>,
    documentation_shapes: Vec<DocumentationShape>,
    connections: Vec<WireConnection>,
}

impl ValidatedRevisionSnapshot {
    fn capture(state: &SchematicState) -> Self {
        Self {
            grid_size: state.grid_size,
            document_policy: state.document_policy,
            components: state.components.clone(),
            wires: state.wires.clone(),
            buses: state.buses.clone(),
            bus_taps: state.bus_taps.clone(),
            junctions: state.junctions.clone(),
            net_labels: state.net_labels.clone(),
            design_notes: state.design_notes.clone(),
            documentation_shapes: state.documentation_shapes.clone(),
            connections: state.connections.clone(),
        }
    }

    fn digest(&self) -> Result<ContentDigest, ValidatedRevisionError> {
        let bytes = serde_json::to_vec(self)
            .map_err(|error| ValidatedRevisionError::Serialization(error.to_string()))?;
        Ok(digest(&bytes))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidatedSchematicRevision {
    id: ValidatedSchematicRevisionId,
    sequence: u64,
    kind: ValidatedRevisionKind,
    parent_revision_digest: Option<ContentDigest>,
    revision_digest: ContentDigest,
    design_content_digest: ContentDigest,
    project_id: String,
    project_revision: u64,
    view_identity: String,
    revision_note: String,
    author: String,
    created_unix_ms: u64,
    validation_receipt_digest: ContentDigest,
    finding_counts: ValidationFindingCounts,
    dependencies: Vec<ValidatedRevisionDependency>,
    advisory_dispositions: Vec<AdvisoryDisposition>,
    snapshot: ValidatedRevisionSnapshot,
}

/// Bounded semantic change counts between two validated schematic revisions.
///
/// Stable object identities distinguish additions/removals from edits. The
/// comparison deliberately reports schematic-owned domains only; it never
/// implies that project plans, models, requirements, or result manifests are
/// part of a schematic snapshot.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ValidatedRevisionObjectDelta {
    pub added: usize,
    pub removed: usize,
    pub modified: usize,
}

impl ValidatedRevisionObjectDelta {
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.added == 0 && self.removed == 0 && self.modified == 0
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ValidatedRevisionSemanticDelta {
    pub components: ValidatedRevisionObjectDelta,
    pub wires: ValidatedRevisionObjectDelta,
    pub buses: ValidatedRevisionObjectDelta,
    pub bus_taps: ValidatedRevisionObjectDelta,
    pub junctions: ValidatedRevisionObjectDelta,
    pub net_labels: ValidatedRevisionObjectDelta,
    pub design_notes: ValidatedRevisionObjectDelta,
    pub documentation_shapes: ValidatedRevisionObjectDelta,
    pub connections_added: usize,
    pub connections_removed: usize,
    pub grid_changed: bool,
    pub document_policy_changed: bool,
}

impl ValidatedRevisionSemanticDelta {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
            && self.wires.is_empty()
            && self.buses.is_empty()
            && self.bus_taps.is_empty()
            && self.junctions.is_empty()
            && self.net_labels.is_empty()
            && self.design_notes.is_empty()
            && self.documentation_shapes.is_empty()
            && self.connections_added == 0
            && self.connections_removed == 0
            && !self.grid_changed
            && !self.document_policy_changed
    }
}

impl ValidatedSchematicRevision {
    #[must_use]
    pub const fn id(&self) -> ValidatedSchematicRevisionId {
        self.id
    }

    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub const fn revision_digest(&self) -> ContentDigest {
        self.revision_digest
    }

    #[must_use]
    pub const fn design_content_digest(&self) -> ContentDigest {
        self.design_content_digest
    }

    #[must_use]
    pub fn revision_note(&self) -> &str {
        &self.revision_note
    }

    #[must_use]
    pub const fn finding_counts(&self) -> ValidationFindingCounts {
        self.finding_counts
    }

    #[must_use]
    pub fn author(&self) -> &str {
        &self.author
    }

    #[must_use]
    pub fn view_identity(&self) -> &str {
        &self.view_identity
    }

    #[must_use]
    pub const fn project_revision(&self) -> u64 {
        self.project_revision
    }

    #[must_use]
    pub const fn created_unix_ms(&self) -> u64 {
        self.created_unix_ms
    }

    #[must_use]
    pub const fn dependency_count(&self) -> usize {
        self.dependencies.len()
    }

    #[must_use]
    pub const fn is_accepted_baseline(&self) -> bool {
        matches!(self.kind, ValidatedRevisionKind::AcceptedBaseline)
    }

    /// Compare exact retained schematic semantics without exposing or
    /// duplicating the immutable snapshots owned by the journal.
    #[must_use]
    pub fn semantic_delta_to(&self, newer: &Self) -> ValidatedRevisionSemanticDelta {
        let older = &self.snapshot;
        let newer = &newer.snapshot;
        let (connections_added, connections_removed) =
            unordered_delta(&older.connections, &newer.connections);
        ValidatedRevisionSemanticDelta {
            components: identity_delta(&older.components, &newer.components, |value| value.id),
            wires: identity_delta(&older.wires, &newer.wires, |value| value.id),
            buses: identity_delta(&older.buses, &newer.buses, |value| value.id),
            bus_taps: identity_delta(&older.bus_taps, &newer.bus_taps, |value| value.id),
            junctions: identity_delta(&older.junctions, &newer.junctions, |value| value.id),
            net_labels: identity_delta(&older.net_labels, &newer.net_labels, |value| value.id),
            design_notes: identity_delta(&older.design_notes, &newer.design_notes, |value| {
                value.id
            }),
            documentation_shapes: identity_delta(
                &older.documentation_shapes,
                &newer.documentation_shapes,
                |value| value.id,
            ),
            connections_added,
            connections_removed,
            grid_changed: older.grid_size != newer.grid_size,
            document_policy_changed: older.document_policy != newer.document_policy,
        }
    }

    fn computed_revision_digest(&self) -> Result<ContentDigest, ValidatedRevisionError> {
        #[derive(Serialize)]
        struct DigestMaterial<'a> {
            id: ValidatedSchematicRevisionId,
            sequence: u64,
            kind: ValidatedRevisionKind,
            parent_revision_digest: Option<ContentDigest>,
            design_content_digest: ContentDigest,
            project_id: &'a str,
            project_revision: u64,
            view_identity: &'a str,
            revision_note: &'a str,
            author: &'a str,
            created_unix_ms: u64,
            validation_receipt_digest: ContentDigest,
            finding_counts: ValidationFindingCounts,
            dependencies: &'a [ValidatedRevisionDependency],
            advisory_dispositions: &'a [AdvisoryDisposition],
        }
        let material = DigestMaterial {
            id: self.id,
            sequence: self.sequence,
            kind: self.kind,
            parent_revision_digest: self.parent_revision_digest,
            design_content_digest: self.design_content_digest,
            project_id: &self.project_id,
            project_revision: self.project_revision,
            view_identity: &self.view_identity,
            revision_note: &self.revision_note,
            author: &self.author,
            created_unix_ms: self.created_unix_ms,
            validation_receipt_digest: self.validation_receipt_digest,
            finding_counts: self.finding_counts,
            dependencies: &self.dependencies,
            advisory_dispositions: &self.advisory_dispositions,
        };
        let bytes = serde_json::to_vec(&material)
            .map_err(|error| ValidatedRevisionError::Serialization(error.to_string()))?;
        Ok(digest(&bytes))
    }

    fn validate(&self) -> Result<(), ValidatedRevisionError> {
        if self.id.0.is_nil() {
            return Err(ValidatedRevisionError::NilRevisionIdentity);
        }
        if self.sequence == 0 {
            return Err(ValidatedRevisionError::InvalidSequence);
        }
        validate_project_id(&self.project_id)?;
        validate_identity("cell/view", &self.view_identity)?;
        validate_identity("author", &self.author)?;
        if self.kind == ValidatedRevisionKind::ValidatedSave {
            let note = self.revision_note.trim();
            if note.is_empty() {
                return Err(ValidatedRevisionError::EmptyRevisionNote);
            }
            if note.chars().count() > MAX_VALIDATED_REVISION_NOTE_LEN {
                return Err(ValidatedRevisionError::RevisionNoteTooLong);
            }
            if self.finding_counts.blockers != 0 {
                return Err(ValidatedRevisionError::BlockingFindings(
                    self.finding_counts.blockers,
                ));
            }
            validate_dispositions(&self.advisory_dispositions, self.finding_counts.advisories)?;
        } else if self.finding_counts != ValidationFindingCounts::default()
            || !self.advisory_dispositions.is_empty()
        {
            return Err(ValidatedRevisionError::InvalidBaselineEvidence);
        }
        validate_dependencies(&self.dependencies)?;
        if self.snapshot.digest()? != self.design_content_digest {
            return Err(ValidatedRevisionError::DesignDigestMismatch);
        }
        if self.computed_revision_digest()? != self.revision_digest {
            return Err(ValidatedRevisionError::RevisionDigestMismatch);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidatedRevisionJournal {
    #[serde(default = "current_journal_schema_version")]
    schema_version: u32,
    #[serde(default)]
    records: Vec<ValidatedSchematicRevision>,
}

impl Default for ValidatedRevisionJournal {
    fn default() -> Self {
        Self {
            schema_version: VALIDATED_REVISION_JOURNAL_SCHEMA_VERSION,
            records: Vec::new(),
        }
    }
}

impl ValidatedRevisionJournal {
    #[must_use]
    pub fn records(&self) -> &[ValidatedSchematicRevision] {
        &self.records
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn validate(&self) -> Result<(), ValidatedRevisionError> {
        if self.schema_version != VALIDATED_REVISION_JOURNAL_SCHEMA_VERSION {
            return Err(ValidatedRevisionError::UnsupportedJournalSchema(
                self.schema_version,
            ));
        }
        let mut prior = None;
        for (index, record) in self.records.iter().enumerate() {
            record.validate()?;
            let expected_sequence = u64::try_from(index)
                .unwrap_or(u64::MAX)
                .checked_add(1)
                .ok_or(ValidatedRevisionError::SequenceExhausted)?;
            if record.sequence != expected_sequence {
                return Err(ValidatedRevisionError::InvalidSequence);
            }
            if record.parent_revision_digest != prior {
                return Err(ValidatedRevisionError::BrokenRevisionChain);
            }
            if index > 0 && record.kind == ValidatedRevisionKind::AcceptedBaseline {
                return Err(ValidatedRevisionError::MisplacedBaseline);
            }
            prior = Some(record.revision_digest);
        }
        Ok(())
    }

    fn append_record(
        &mut self,
        mut record: ValidatedSchematicRevision,
    ) -> Result<ValidatedSchematicRevisionId, ValidatedRevisionError> {
        self.validate()?;
        record.sequence = u64::try_from(self.records.len())
            .unwrap_or(u64::MAX)
            .checked_add(1)
            .ok_or(ValidatedRevisionError::SequenceExhausted)?;
        record.parent_revision_digest = self.records.last().map(|prior| prior.revision_digest);
        record.revision_digest = record.computed_revision_digest()?;
        record.validate()?;
        let id = record.id;
        self.records.push(record);
        Ok(id)
    }

    pub fn remove_unpublished_tail(
        &mut self,
        id: ValidatedSchematicRevisionId,
    ) -> Result<(), ValidatedRevisionError> {
        self.validate()?;
        let tail = self
            .records
            .last()
            .ok_or(ValidatedRevisionError::RevisionNotFound)?;
        if tail.id != id || tail.kind != ValidatedRevisionKind::ValidatedSave {
            return Err(ValidatedRevisionError::NotUnpublishedTail);
        }
        self.records.pop();
        Ok(())
    }
}

impl SchematicState {
    pub(crate) fn validated_design_content_digest(
        &self,
    ) -> Result<ContentDigest, ValidatedRevisionError> {
        ValidatedRevisionSnapshot::capture(self).digest()
    }

    pub fn seed_accepted_revision_baseline(
        &mut self,
        accepted: &SchematicState,
        project_id: &str,
        project_revision: u64,
        view_identity: &str,
    ) -> Result<Option<ValidatedSchematicRevisionId>, ValidatedRevisionError> {
        if !self.validated_revisions.is_empty() {
            self.validated_revisions.validate()?;
            return Ok(None);
        }
        validate_project_id(project_id)?;
        validate_identity("cell/view", view_identity)?;
        let snapshot = ValidatedRevisionSnapshot::capture(accepted);
        let design_content_digest = snapshot.digest()?;
        let mut record = ValidatedSchematicRevision {
            id: ValidatedSchematicRevisionId::new(),
            sequence: 0,
            kind: ValidatedRevisionKind::AcceptedBaseline,
            parent_revision_digest: None,
            revision_digest: digest(b"pending accepted baseline revision"),
            design_content_digest,
            project_id: project_id.to_owned(),
            project_revision,
            view_identity: view_identity.to_owned(),
            revision_note: String::new(),
            author: "RSpice accepted-baseline migration".to_owned(),
            created_unix_ms: unix_time_ms(),
            validation_receipt_digest: digest(b"accepted baseline predates validation receipts"),
            finding_counts: ValidationFindingCounts::default(),
            dependencies: Vec::new(),
            advisory_dispositions: Vec::new(),
            snapshot,
        };
        record.revision_digest = record.computed_revision_digest()?;
        self.validated_revisions.append_record(record).map(Some)
    }

    pub fn append_validated_revision(
        &mut self,
        request: ValidatedRevisionRequest,
    ) -> Result<ValidatedSchematicRevisionId, ValidatedRevisionError> {
        request.validate()?;
        self.validated_revisions.validate()?;
        let snapshot = ValidatedRevisionSnapshot::capture(self);
        let design_content_digest = snapshot.digest()?;
        let mut record = ValidatedSchematicRevision {
            id: ValidatedSchematicRevisionId::new(),
            sequence: 0,
            kind: ValidatedRevisionKind::ValidatedSave,
            parent_revision_digest: None,
            revision_digest: digest(b"pending validated schematic revision"),
            design_content_digest,
            project_id: request.project_id,
            project_revision: request.project_revision,
            view_identity: request.view_identity,
            revision_note: request.revision_note.trim().to_owned(),
            author: request.author.trim().to_owned(),
            created_unix_ms: unix_time_ms(),
            validation_receipt_digest: request.validation_receipt_digest,
            finding_counts: request.finding_counts,
            dependencies: request.dependencies,
            advisory_dispositions: request.advisory_dispositions,
            snapshot,
        };
        record.revision_digest = record.computed_revision_digest()?;
        let id = self.validated_revisions.append_record(record)?;
        self.is_dirty = true;
        Ok(id)
    }

    pub fn remove_unpublished_validated_revision(
        &mut self,
        id: ValidatedSchematicRevisionId,
    ) -> Result<(), ValidatedRevisionError> {
        self.validated_revisions.remove_unpublished_tail(id)
    }

    pub fn restore_validated_revision(
        &mut self,
        id: ValidatedSchematicRevisionId,
    ) -> Result<(), ValidatedRevisionError> {
        if self.read_only {
            return Err(ValidatedRevisionError::ReadOnly);
        }
        self.validated_revisions.validate()?;
        let snapshot = self
            .validated_revisions
            .records
            .iter()
            .find(|record| record.id == id)
            .map(|record| record.snapshot.clone())
            .ok_or(ValidatedRevisionError::RevisionNotFound)?;
        let target = SchematicSnapshot {
            document_policy: snapshot.document_policy,
            grid_size: snapshot.grid_size,
            components: snapshot.components,
            wires: snapshot.wires,
            buses: snapshot.buses,
            bus_taps: snapshot.bus_taps,
            junctions: snapshot.junctions,
            net_labels: snapshot.net_labels,
            design_notes: snapshot.design_notes,
            documentation_shapes: snapshot.documentation_shapes,
            // Probe flags are simulation-output requests rather than
            // validated electrical topology. A design revision restore must
            // therefore preserve the live output markers instead of silently
            // deleting them.
            probes: self.probes.clone(),
            connections: snapshot.connections,
            // A stored revision restores the drawing. Sheet membership belongs
            // to the project catalog, which still holds the live one.
            sheet_assignments: std::collections::BTreeMap::new(),
        };
        if target.is_equal_state(self) {
            return Err(ValidatedRevisionError::AlreadyCurrent);
        }
        let changed = self.with_undo("restore validated schematic revision", move |state| {
            target.apply(state);
        });
        if changed {
            Ok(())
        } else {
            Err(ValidatedRevisionError::AlreadyCurrent)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ValidatedRevisionError {
    #[error("validated revision journal schema {0} is unsupported")]
    UnsupportedJournalSchema(u32),
    #[error("validated revision identity must not be nil")]
    NilRevisionIdentity,
    #[error("validated revision sequence is invalid")]
    InvalidSequence,
    #[error("validated revision sequence space is exhausted")]
    SequenceExhausted,
    #[error("validated revision chain is broken")]
    BrokenRevisionChain,
    #[error("an accepted baseline may only be the first revision record")]
    MisplacedBaseline,
    #[error("accepted baseline contains validation evidence it cannot own")]
    InvalidBaselineEvidence,
    #[error("revision note is required")]
    EmptyRevisionNote,
    #[error("revision note exceeds {MAX_VALIDATED_REVISION_NOTE_LEN} characters")]
    RevisionNoteTooLong,
    #[error("validated save still contains {0} blocking findings")]
    BlockingFindings(u32),
    #[error("project identity is invalid")]
    InvalidProjectIdentity,
    #[error("{field} identity is empty")]
    EmptyIdentity { field: &'static str },
    #[error("{field} identity is too long")]
    IdentityTooLong { field: &'static str },
    #[error("dependency identities must be sorted and unique")]
    UnsortedDependencies,
    #[error("advisory dispositions must be sorted and unique")]
    UnsortedAdvisoryDispositions,
    #[error("advisory disposition reason is required")]
    EmptyAdvisoryReason,
    #[error("advisory disposition reason is too long")]
    AdvisoryReasonTooLong,
    #[error("{expected} advisory dispositions were required, but {actual} were supplied")]
    AdvisoryDispositionCount { expected: u32, actual: usize },
    #[error("validated design snapshot digest does not match its record")]
    DesignDigestMismatch,
    #[error("validated revision digest does not match its record")]
    RevisionDigestMismatch,
    #[error("validated revision was not found")]
    RevisionNotFound,
    #[error("the schematic is read-only")]
    ReadOnly,
    #[error("the selected validated revision already matches the active schematic")]
    AlreadyCurrent,
    #[error("only the exact unpublished tail revision may be removed")]
    NotUnpublishedTail,
    #[error("validated revision serialization failed: {0}")]
    Serialization(String),
}

fn identity_delta<T, F>(older: &[T], newer: &[T], identity: F) -> ValidatedRevisionObjectDelta
where
    T: PartialEq,
    F: Fn(&T) -> u64,
{
    let mut delta = ValidatedRevisionObjectDelta::default();
    for old in older {
        match newer.iter().find(|new| identity(new) == identity(old)) {
            None => delta.removed += 1,
            Some(new) if new != old => delta.modified += 1,
            Some(_) => {}
        }
    }
    for new in newer {
        if !older.iter().any(|old| identity(old) == identity(new)) {
            delta.added += 1;
        }
    }
    delta
}

fn unordered_delta<T: PartialEq>(older: &[T], newer: &[T]) -> (usize, usize) {
    let added = newer
        .iter()
        .filter(|candidate| !older.contains(candidate))
        .count();
    let removed = older
        .iter()
        .filter(|candidate| !newer.contains(candidate))
        .count();
    (added, removed)
}

fn validate_project_id(project_id: &str) -> Result<(), ValidatedRevisionError> {
    Uuid::parse_str(project_id)
        .ok()
        .filter(|id| !id.is_nil())
        .map(|_| ())
        .ok_or(ValidatedRevisionError::InvalidProjectIdentity)
}

fn validate_identity(field: &'static str, value: &str) -> Result<(), ValidatedRevisionError> {
    if value.trim().is_empty() {
        return Err(ValidatedRevisionError::EmptyIdentity { field });
    }
    if value.chars().count() > MAX_VALIDATED_REVISION_IDENTITY_LEN {
        return Err(ValidatedRevisionError::IdentityTooLong { field });
    }
    Ok(())
}

fn validate_dependencies(
    dependencies: &[ValidatedRevisionDependency],
) -> Result<(), ValidatedRevisionError> {
    let mut prior: Option<&str> = None;
    for dependency in dependencies {
        dependency.validate()?;
        if prior.is_some_and(|prior| prior >= dependency.identity.as_str()) {
            return Err(ValidatedRevisionError::UnsortedDependencies);
        }
        prior = Some(&dependency.identity);
    }
    Ok(())
}

fn validate_dispositions(
    dispositions: &[AdvisoryDisposition],
    expected: u32,
) -> Result<(), ValidatedRevisionError> {
    if dispositions.len() != usize::try_from(expected).unwrap_or(usize::MAX) {
        return Err(ValidatedRevisionError::AdvisoryDispositionCount {
            expected,
            actual: dispositions.len(),
        });
    }
    let mut prior: Option<&str> = None;
    for disposition in dispositions {
        disposition.validate()?;
        if prior.is_some_and(|prior| prior >= disposition.finding_identity.as_str()) {
            return Err(ValidatedRevisionError::UnsortedAdvisoryDispositions);
        }
        prior = Some(&disposition.finding_identity);
    }
    Ok(())
}

fn digest(bytes: &[u8]) -> ContentDigest {
    ContentDigest::from_bytes(Sha256::digest(bytes).into())
}

fn unix_time_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .try_into()
        .unwrap_or(u64::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ComponentType, NetNamingPolicy, Point};

    fn request(state: &SchematicState, project_id: Uuid) -> ValidatedRevisionRequest {
        let snapshot = ValidatedRevisionSnapshot::capture(state);
        ValidatedRevisionRequest {
            project_id: project_id.to_string(),
            project_revision: 3,
            view_identity: "user/top/schematic".to_owned(),
            revision_note: "Validate compensation network".to_owned(),
            author: "Local project editor".to_owned(),
            validation_receipt_digest: snapshot.digest().expect("snapshot digest"),
            finding_counts: ValidationFindingCounts {
                blockers: 0,
                advisories: 1,
            },
            dependencies: vec![ValidatedRevisionDependency::new(
                "schematic:user/top/schematic",
                snapshot.digest().expect("snapshot digest"),
            )],
            advisory_dispositions: vec![AdvisoryDisposition::accepted(
                "drc:unconnected_pin:R1.+",
                "Validated compensation network",
            )],
        }
    }

    #[test]
    fn journal_retains_restorable_hash_linked_revisions() {
        let project_id = Uuid::new_v4();
        let mut state = SchematicState::default();
        state.add_component(ComponentType::Resistor, Point::new(10, 10));
        let first = state
            .append_validated_revision(request(&state, project_id))
            .expect("first revision");
        state.components[0].value = "2k".to_owned();
        let second = state
            .append_validated_revision(request(&state, project_id))
            .expect("second revision");

        state.validated_revisions.validate().expect("valid journal");
        assert_eq!(state.validated_revisions.records().len(), 2);
        assert_ne!(first, second);
        state
            .restore_validated_revision(first)
            .expect("restore first");
        assert_eq!(state.components[0].value, "1k");
    }

    #[test]
    fn restore_recovers_document_semantics_and_grid() {
        let project_id = Uuid::new_v4();
        let mut state = SchematicState::default();
        state.add_component(ComponentType::Resistor, Point::new(10, 10));
        state.document_policy.net_naming = NetNamingPolicy::SpiceCompatibleRelaxed;
        state.grid_size = 4;
        let saved = state
            .append_validated_revision(request(&state, project_id))
            .expect("validated revision");

        state.document_policy.net_naming = NetNamingPolicy::StrictCaseSensitive;
        state.grid_size = 10;
        state.restore_validated_revision(saved).expect("restore");

        assert_eq!(
            state.document_policy.net_naming,
            NetNamingPolicy::SpiceCompatibleRelaxed
        );
        assert_eq!(state.grid_size, 4);
    }

    #[test]
    fn restore_fails_closed_for_read_only_and_already_current_documents() {
        let project_id = Uuid::new_v4();
        let mut state = SchematicState::default();
        state.add_component(ComponentType::Resistor, Point::new(10, 10));
        let saved = state
            .append_validated_revision(request(&state, project_id))
            .expect("validated revision");
        assert_eq!(
            state.restore_validated_revision(saved),
            Err(ValidatedRevisionError::AlreadyCurrent)
        );

        state.components[0].value = "3k".to_owned();
        let changed = state.components.clone();
        state.read_only = true;
        assert_eq!(
            state.restore_validated_revision(saved),
            Err(ValidatedRevisionError::ReadOnly)
        );
        assert_eq!(state.components, changed);
    }

    #[test]
    fn semantic_delta_reports_stable_object_changes() {
        let project_id = Uuid::new_v4();
        let mut state = SchematicState::default();
        state.add_component(ComponentType::Resistor, Point::new(10, 10));
        let first = state
            .append_validated_revision(request(&state, project_id))
            .expect("first revision");
        state.components[0].value = "2k".to_owned();
        state.add_component(ComponentType::Capacitor, Point::new(20, 10));
        state.grid_size = 8;
        let second = state
            .append_validated_revision(request(&state, project_id))
            .expect("second revision");
        let first = state
            .validated_revisions
            .records()
            .iter()
            .find(|record| record.id() == first)
            .unwrap();
        let second = state
            .validated_revisions
            .records()
            .iter()
            .find(|record| record.id() == second)
            .unwrap();
        let delta = first.semantic_delta_to(second);
        assert_eq!(delta.components.added, 1);
        assert_eq!(delta.components.modified, 1);
        assert!(delta.grid_changed);
    }

    #[test]
    fn journal_serialization_declares_its_schema() {
        let encoded =
            serde_json::to_value(ValidatedRevisionJournal::default()).expect("journal serializes");
        assert_eq!(
            encoded
                .get("schema_version")
                .and_then(serde_json::Value::as_u64),
            Some(u64::from(VALIDATED_REVISION_JOURNAL_SCHEMA_VERSION))
        );
        assert!(
            encoded
                .get("records")
                .is_some_and(serde_json::Value::is_array)
        );

        let migrated: ValidatedRevisionJournal =
            serde_json::from_value(serde_json::json!({ "records": [] }))
                .expect("pre-version empty journal migrates");
        migrated.validate().expect("migrated journal validates");

        let unsupported: ValidatedRevisionJournal = serde_json::from_value(serde_json::json!({
            "schema_version": VALIDATED_REVISION_JOURNAL_SCHEMA_VERSION + 1,
            "records": []
        }))
        .expect("future journal parses for an explicit compatibility error");
        assert_eq!(
            unsupported.validate(),
            Err(ValidatedRevisionError::UnsupportedJournalSchema(
                VALIDATED_REVISION_JOURNAL_SCHEMA_VERSION + 1
            ))
        );
    }

    #[test]
    fn unpublished_removal_is_tail_only() {
        let project_id = Uuid::new_v4();
        let mut state = SchematicState::default();
        state.add_component(ComponentType::Resistor, Point::origin());
        let first = state
            .append_validated_revision(request(&state, project_id))
            .expect("first revision");
        let second = state
            .append_validated_revision(request(&state, project_id))
            .expect("second revision");
        assert_eq!(
            state.remove_unpublished_validated_revision(first),
            Err(ValidatedRevisionError::NotUnpublishedTail)
        );
        state
            .remove_unpublished_validated_revision(second)
            .expect("remove exact tail");
        assert_eq!(state.validated_revisions.records().len(), 1);
    }

    #[test]
    fn migrated_baseline_preserves_prior_accepted_design() {
        let project_id = Uuid::new_v4();
        let mut accepted = SchematicState::default();
        accepted.add_component(ComponentType::Capacitor, Point::new(20, 20));
        let mut working = accepted.clone();
        working.components[0].value = "2p".to_owned();
        let baseline = working
            .seed_accepted_revision_baseline(
                &accepted,
                &project_id.to_string(),
                7,
                "user/top/schematic",
            )
            .expect("seed baseline")
            .expect("baseline id");
        working
            .append_validated_revision(request(&working, project_id))
            .expect("validated successor");
        working
            .restore_validated_revision(baseline)
            .expect("restore baseline");
        assert_eq!(working.components[0].value, "1u");
    }

    #[test]
    fn blockers_and_undisposed_advisories_fail_closed() {
        let project_id = Uuid::new_v4();
        let mut state = SchematicState::default();
        let mut blocked = request(&state, project_id);
        blocked.finding_counts.blockers = 1;
        assert_eq!(
            state.append_validated_revision(blocked),
            Err(ValidatedRevisionError::BlockingFindings(1))
        );
        let mut undisposed = request(&state, project_id);
        undisposed.advisory_dispositions.clear();
        assert_eq!(
            state.append_validated_revision(undisposed),
            Err(ValidatedRevisionError::AdvisoryDispositionCount {
                expected: 1,
                actual: 0,
            })
        );
    }
}
