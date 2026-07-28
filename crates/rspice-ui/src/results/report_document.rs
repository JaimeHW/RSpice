//! Canonical, UI-independent report-composer domain.
//!
//! A report is a versioned arrangement of pages, sections, and typed content
//! blocks. References retain the exact source revision, content digest, and
//! dataset bindings used to compose a revision. Linked references can be
//! audited or refreshed; frozen references remain self-contained and never
//! silently follow mutable external state.

mod blocks;
mod mutation;

pub use blocks::*;

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io;

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest as _, Sha256};
use uuid::Uuid;

use crate::product::{
    ContentDigest, DatasetBinding, DatasetId, ObjectRevision, ResultDocumentId, RevisionError,
    VerificationEvidenceId,
};

const MAX_PAGES: usize = 256;
const MAX_SECTIONS_PER_PAGE: usize = 256;
const MAX_BLOCKS_PER_SECTION: usize = 1_024;
const MAX_BLOCKS_TOTAL: usize = 8_192;
const MAX_TRANSACTION_EDITS: usize = 256;
pub const MAX_REPORT_REVISION_HISTORY_RECORDS: usize = 8_192;
pub const MAX_REPORT_REVISION_HISTORY_SNAPSHOT_BYTES: u64 = 512 * 1_048_576;
const REPORT_MIGRATION_REVISION_ID_NAMESPACE: Uuid =
    Uuid::from_u128(0x7273_7069_6365_2d72_6576_2d68_6973_742d);
const MAX_DATASET_BINDINGS: usize = 4_096;
const MAX_TABLE_COLUMNS: usize = 256;
const MAX_TABLE_ROWS: usize = 100_000;
const MAX_TABLE_CELLS: usize = 1_000_000;
const MAX_STRUCTURED_ENTRIES: usize = 10_000;
const MAX_TEXT_BYTES: usize = 1_048_576;
pub const MAX_FROZEN_ARTIFACT_BYTES: usize = 32 * 1_048_576;
const MAX_FROZEN_ARTIFACT_BASE64_BYTES: usize = MAX_FROZEN_ARTIFACT_BYTES.div_ceil(3) * 4;
const MAX_FROZEN_ARTIFACT_BYTES_PER_DOCUMENT: usize = 256 * 1_048_576;

macro_rules! stable_uuid_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            #[must_use]
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }

            pub fn try_from_uuid(value: Uuid) -> Result<Self, ReportError> {
                (!value.is_nil())
                    .then_some(Self(value))
                    .ok_or(ReportError::NilIdentity)
            }

            #[must_use]
            pub const fn as_uuid(self) -> Uuid {
                self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                Self::try_from_uuid(Uuid::deserialize(deserializer)?)
                    .map_err(serde::de::Error::custom)
            }
        }
    };
}

stable_uuid_id!(ReportPageId);
stable_uuid_id!(ReportSectionId);
stable_uuid_id!(ReportBlockId);
stable_uuid_id!(ReportRevisionId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind", content = "id")]
pub enum ReportEntityRef {
    Page(ReportPageId),
    Section(ReportSectionId),
    Block(ReportBlockId),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind")]
pub enum ReportSourceId {
    VisualizationDocument { document_id: ResultDocumentId },
    Dataset { dataset_id: DatasetId },
    VerificationEvidence { evidence_id: VerificationEvidenceId },
    ExternalRecord { namespace: String, key: String },
}

impl ReportSourceId {
    pub fn validate(&self) -> Result<(), ReportError> {
        if let Self::ExternalRecord { namespace, key } = self {
            validate_token("reference.source.namespace", namespace, 64)?;
            validate_token("reference.source.key", key, 256)?;
        }
        Ok(())
    }
}

/// Exact source state captured when a block was linked or frozen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReportReferenceSnapshot {
    pub source: ReportSourceId,
    pub source_revision: Option<ObjectRevision>,
    pub content_digest: ContentDigest,
    pub dataset_bindings: Vec<DatasetBinding>,
}

/// Authenticated, self-contained payload retained by a frozen report block.
///
/// The digest is always SHA-256 over the exact payload bytes. Construction and
/// deserialization both recompute it, so persisted payload bytes cannot change
/// without failing validation. The normalized media type is retained as
/// explicit rendering metadata and is included by the reference-audit digest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrozenReportArtifact {
    media_type: String,
    payload: Vec<u8>,
    content_digest: ContentDigest,
}

#[derive(Serialize, Deserialize)]
struct FrozenReportArtifactWire {
    media_type: String,
    payload_base64: String,
    content_digest: ContentDigest,
}

impl Serialize for FrozenReportArtifact {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        FrozenReportArtifactWire {
            media_type: self.media_type.clone(),
            payload_base64: BASE64_STANDARD.encode(&self.payload),
            content_digest: self.content_digest,
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for FrozenReportArtifact {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = FrozenReportArtifactWire::deserialize(deserializer)?;
        if wire.payload_base64.len() > MAX_FROZEN_ARTIFACT_BASE64_BYTES {
            return Err(serde::de::Error::custom(ReportError::InvalidValue {
                field: "frozen-artifact.payload-base64",
                message: format!(
                    "encoded frozen artifacts must not exceed {MAX_FROZEN_ARTIFACT_BASE64_BYTES} bytes"
                ),
            }));
        }
        let payload = BASE64_STANDARD
            .decode(wire.payload_base64.as_bytes())
            .map_err(|error| {
                serde::de::Error::custom(ReportError::InvalidValue {
                    field: "frozen-artifact.payload-base64",
                    message: format!("payload is not canonical base64: {error}"),
                })
            })?;
        if BASE64_STANDARD.encode(&payload) != wire.payload_base64 {
            return Err(serde::de::Error::custom(ReportError::InvalidValue {
                field: "frozen-artifact.payload-base64",
                message: "payload must use canonical padded base64 encoding".to_owned(),
            }));
        }
        let artifact = Self {
            media_type: wire.media_type,
            payload,
            content_digest: wire.content_digest,
        };
        artifact.validate().map_err(serde::de::Error::custom)?;
        Ok(artifact)
    }
}

impl FrozenReportArtifact {
    pub fn new(media_type: impl Into<String>, payload: Vec<u8>) -> Result<Self, ReportError> {
        let content_digest = ContentDigest::from_bytes(Sha256::digest(&payload).into());
        let artifact = Self {
            media_type: media_type.into(),
            payload,
            content_digest,
        };
        artifact.validate()?;
        Ok(artifact)
    }

    #[must_use]
    pub fn media_type(&self) -> &str {
        &self.media_type
    }

    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    #[must_use]
    pub const fn content_digest(&self) -> ContentDigest {
        self.content_digest
    }

    fn validate(&self) -> Result<(), ReportError> {
        let valid_media_type = !self.media_type.is_empty()
            && self.media_type.len() <= 128
            && self.media_type.bytes().filter(|byte| *byte == b'/').count() == 1
            && self.media_type.bytes().all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || matches!(byte, b'/' | b'+' | b'-' | b'.')
            });
        if !valid_media_type {
            return Err(ReportError::InvalidValue {
                field: "frozen-artifact.media-type",
                message: "media type must be a normalized lowercase type/subtype token of at most 128 bytes".to_owned(),
            });
        }
        if self.payload.is_empty() || self.payload.len() > MAX_FROZEN_ARTIFACT_BYTES {
            return Err(ReportError::InvalidValue {
                field: "frozen-artifact.payload",
                message: format!(
                    "frozen artifacts require 1 to {MAX_FROZEN_ARTIFACT_BYTES} payload bytes"
                ),
            });
        }
        let computed = ContentDigest::from_bytes(Sha256::digest(&self.payload).into());
        if computed != self.content_digest {
            return Err(ReportError::InvalidValue {
                field: "frozen-artifact.content-digest",
                message: "artifact content digest does not authenticate the exact payload bytes"
                    .to_owned(),
            });
        }
        Ok(())
    }
}

impl ReportReferenceSnapshot {
    pub fn new(
        source: ReportSourceId,
        source_revision: Option<ObjectRevision>,
        content_digest: ContentDigest,
        dataset_bindings: Vec<DatasetBinding>,
    ) -> Result<Self, ReportError> {
        let snapshot = Self {
            source,
            source_revision,
            content_digest,
            dataset_bindings,
        };
        snapshot.validate()?;
        Ok(snapshot)
    }

    fn validate(&self) -> Result<(), ReportError> {
        self.source.validate()?;
        if matches!(self.source, ReportSourceId::VisualizationDocument { .. })
            && self.source_revision.is_none()
        {
            return Err(ReportError::InvalidValue {
                field: "reference.source-revision",
                message: "visualization document references require an exact source revision"
                    .to_owned(),
            });
        }
        validate_dataset_bindings(&self.dataset_bindings)?;
        if let ReportSourceId::Dataset { dataset_id } = self.source {
            let exact_binding = self.dataset_bindings.iter().any(|binding| {
                binding.dataset_id == dataset_id && binding.content_digest == self.content_digest
            });
            if !exact_binding {
                return Err(ReportError::InvalidValue {
                    field: "reference.dataset-bindings",
                    message: "a dataset source must include its exact immutable dataset binding"
                        .to_owned(),
                });
            }
        }
        Ok(())
    }
}

/// Whether a source should be followed or preserved as an immutable artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "mode")]
pub enum ReportReferenceMode {
    Linked {
        snapshot: ReportReferenceSnapshot,
    },
    Frozen {
        snapshot: ReportReferenceSnapshot,
        artifact: FrozenReportArtifact,
    },
}

impl ReportReferenceMode {
    #[must_use]
    pub const fn snapshot(&self) -> &ReportReferenceSnapshot {
        match self {
            Self::Linked { snapshot } | Self::Frozen { snapshot, .. } => snapshot,
        }
    }

    #[must_use]
    pub const fn is_frozen(&self) -> bool {
        matches!(self, Self::Frozen { .. })
    }

    #[must_use]
    pub const fn frozen_artifact(&self) -> Option<&FrozenReportArtifact> {
        match self {
            Self::Frozen { artifact, .. } => Some(artifact),
            Self::Linked { .. } => None,
        }
    }

    fn validate(&self) -> Result<(), ReportError> {
        self.snapshot().validate()?;
        if let Self::Frozen { artifact, .. } = self {
            artifact.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportBlock {
    id: ReportBlockId,
    #[serde(default)]
    created_at_document_revision: ObjectRevision,
    revision: ObjectRevision,
    kind: ReportBlockKind,
}

impl ReportBlock {
    #[must_use]
    pub const fn id(&self) -> ReportBlockId {
        self.id
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub const fn created_at_document_revision(&self) -> ObjectRevision {
        self.created_at_document_revision
    }

    #[must_use]
    pub const fn kind(&self) -> &ReportBlockKind {
        &self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportSection {
    id: ReportSectionId,
    #[serde(default)]
    created_at_document_revision: ObjectRevision,
    revision: ObjectRevision,
    title: String,
    blocks: Vec<ReportBlock>,
}

impl ReportSection {
    #[must_use]
    pub const fn id(&self) -> ReportSectionId {
        self.id
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub const fn created_at_document_revision(&self) -> ObjectRevision {
        self.created_at_document_revision
    }

    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    #[must_use]
    pub fn blocks(&self) -> &[ReportBlock] {
        &self.blocks
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportPage {
    id: ReportPageId,
    #[serde(default)]
    created_at_document_revision: ObjectRevision,
    revision: ObjectRevision,
    title: String,
    update_policy: ReportPageUpdatePolicy,
    sections: Vec<ReportSection>,
}

impl ReportPage {
    #[must_use]
    pub const fn id(&self) -> ReportPageId {
        self.id
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub const fn created_at_document_revision(&self) -> ObjectRevision {
        self.created_at_document_revision
    }

    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    #[must_use]
    pub const fn update_policy(&self) -> ReportPageUpdatePolicy {
        self.update_policy
    }

    #[must_use]
    pub fn sections(&self) -> &[ReportSection] {
        &self.sections
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "edit")]
pub enum ReportEdit {
    SetDocumentTitle {
        title: String,
    },
    SetTemplate {
        template: ReportTemplate,
    },
    AddPage {
        title: String,
    },
    UpdatePageTitle {
        page_id: ReportPageId,
        expected_page_revision: ObjectRevision,
        title: String,
    },
    SetPageUpdatePolicy {
        page_id: ReportPageId,
        expected_page_revision: ObjectRevision,
        update_policy: ReportPageUpdatePolicy,
    },
    MovePage {
        page_id: ReportPageId,
        expected_page_revision: ObjectRevision,
        before: Option<ReportPageId>,
    },
    AddSection {
        page_id: ReportPageId,
        title: String,
    },
    UpdateSectionTitle {
        section_id: ReportSectionId,
        expected_section_revision: ObjectRevision,
        title: String,
    },
    MoveSection {
        section_id: ReportSectionId,
        expected_section_revision: ObjectRevision,
        target_page_id: ReportPageId,
        before: Option<ReportSectionId>,
    },
    AddBlock {
        section_id: ReportSectionId,
        kind: ReportBlockKind,
    },
    ReplaceBlock {
        block_id: ReportBlockId,
        expected_block_revision: ObjectRevision,
        kind: ReportBlockKind,
    },
    UpdateBlockReference {
        block_id: ReportBlockId,
        expected_block_revision: ObjectRevision,
        reference: ReportReferenceMode,
    },
    MoveBlock {
        block_id: ReportBlockId,
        expected_block_revision: ObjectRevision,
        target_section_id: ReportSectionId,
        before: Option<ReportBlockId>,
    },
    Remove {
        entity: ReportEntityRef,
        expected_entity_revision: ObjectRevision,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReportTombstone {
    pub entity: ReportEntityRef,
    pub created_at_document_revision: ObjectRevision,
    pub last_entity_revision: ObjectRevision,
    pub removed_at_document_revision: ObjectRevision,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReportMutationReceipt {
    pub operation_id: Uuid,
    pub previous_document_revision: ObjectRevision,
    pub committed_document_revision: ObjectRevision,
    pub timestamp_unix_ms: u64,
    pub edit_count: u16,
    pub created: Vec<ReportEntityRef>,
    pub changed: Vec<ReportEntityRef>,
    pub tombstoned: Vec<ReportEntityRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReportReferenceInventoryEntry {
    pub source: ReportSourceId,
    pub source_revision: Option<ObjectRevision>,
    pub content_digest: ContentDigest,
    pub dataset_bindings: Vec<DatasetBinding>,
}

impl ReportReferenceInventoryEntry {
    pub fn new(
        source: ReportSourceId,
        source_revision: Option<ObjectRevision>,
        content_digest: ContentDigest,
        dataset_bindings: Vec<DatasetBinding>,
    ) -> Result<Self, ReportError> {
        let entry = Self {
            source,
            source_revision,
            content_digest,
            dataset_bindings,
        };
        ReportReferenceSnapshot {
            source: entry.source.clone(),
            source_revision: entry.source_revision,
            content_digest: entry.content_digest,
            dataset_bindings: entry.dataset_bindings.clone(),
        }
        .validate()?;
        Ok(entry)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ReportReferenceInventory {
    pub sources: Vec<ReportReferenceInventoryEntry>,
    pub available_datasets: Vec<DatasetBinding>,
}

impl ReportReferenceInventory {
    pub fn validate(&self) -> Result<(), ReportError> {
        if self.sources.len() > MAX_BLOCKS_TOTAL
            || self.available_datasets.len() > MAX_DATASET_BINDINGS
        {
            return Err(ReportError::InvalidValue {
                field: "reference-inventory",
                message: "reference inventory exceeds the bounded report audit capacity".to_owned(),
            });
        }
        let mut source_ids = HashSet::with_capacity(self.sources.len());
        for source in &self.sources {
            if !source_ids.insert(&source.source) {
                return Err(ReportError::DuplicateReferenceSource(source.source.clone()));
            }
            ReportReferenceSnapshot {
                source: source.source.clone(),
                source_revision: source.source_revision,
                content_digest: source.content_digest,
                dataset_bindings: source.dataset_bindings.clone(),
            }
            .validate()?;
        }
        validate_dataset_bindings(&self.available_datasets)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReportReferenceCurrentness {
    Current,
    UpdateAvailable,
    SourceContentChanged,
    SourceMissing,
    DatasetMissing,
    Frozen,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReportReferenceAuditEntry {
    pub block_id: ReportBlockId,
    pub source: ReportSourceId,
    pub currentness: ReportReferenceCurrentness,
    pub captured_revision: Option<ObjectRevision>,
    pub available_revision: Option<ObjectRevision>,
    pub captured_content_digest: ContentDigest,
    pub available_content_digest: Option<ContentDigest>,
    pub frozen_artifact_media_type: Option<String>,
    pub frozen_artifact_digest: Option<ContentDigest>,
    pub missing_dataset_bindings: Vec<DatasetBinding>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReportReferenceAudit {
    pub document_id: ResultDocumentId,
    pub document_revision: ObjectRevision,
    pub audit_digest: ContentDigest,
    pub entries: Vec<ReportReferenceAuditEntry>,
}

impl ReportReferenceAudit {
    /// True only when every linked block resolves exactly and every frozen
    /// block retains its embedded artifact. A frozen source being absent from
    /// the live inventory does not invalidate its immutable artifact.
    #[must_use]
    pub fn is_current_for_sign_off(&self) -> bool {
        self.entries.iter().all(|entry| {
            matches!(
                entry.currentness,
                ReportReferenceCurrentness::Current | ReportReferenceCurrentness::Frozen
            )
        })
    }
}

/// Provenance of the first reconstructable source revision retained by a
/// report document.
///
/// Native documents retain revision one and every successor. Documents loaded
/// from an older schema retain an explicit baseline instead: this preserves
/// their exact current content without manufacturing revisions that the older
/// format never stored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReportRevisionHistoryOrigin {
    Native,
    ImportedSchemaOneBaseline,
    ImportedSchemaTwoBaseline,
}

/// Complete report source at one committed document revision.
///
/// The snapshot deliberately excludes the history ledger itself. Its fields
/// are otherwise sufficient to reconstruct and validate the report source at
/// the named revision, including its receipts and tombstones.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReportDocumentSnapshot {
    document_id: ResultDocumentId,
    revision: ObjectRevision,
    title: String,
    template: ReportTemplate,
    pages: Vec<ReportPage>,
    receipts: Vec<ReportMutationReceipt>,
    tombstones: Vec<ReportTombstone>,
    legacy_origin_entities: Vec<ReportEntityRef>,
}

impl ReportDocumentSnapshot {
    #[must_use]
    pub const fn document_id(&self) -> ResultDocumentId {
        self.document_id
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    #[must_use]
    pub const fn template(&self) -> ReportTemplate {
        self.template
    }

    #[must_use]
    pub fn pages(&self) -> &[ReportPage] {
        &self.pages
    }

    #[must_use]
    pub fn receipts(&self) -> &[ReportMutationReceipt] {
        &self.receipts
    }

    #[must_use]
    pub fn tombstones(&self) -> &[ReportTombstone] {
        &self.tombstones
    }

    #[must_use]
    pub fn legacy_origin_entities(&self) -> &[ReportEntityRef] {
        &self.legacy_origin_entities
    }
}

/// Immutable audit record for a complete report source revision.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReportRevisionRecord {
    revision_identity: ReportRevisionId,
    document_id: ResultDocumentId,
    revision: ObjectRevision,
    prior_revision_identity: Option<ReportRevisionId>,
    prior_record_digest: Option<ContentDigest>,
    timestamp_unix_ms: u64,
    actor: String,
    revision_note: String,
    snapshot_serialized_bytes: u64,
    snapshot_digest: ContentDigest,
    record_digest: ContentDigest,
    snapshot: ReportDocumentSnapshot,
}

impl ReportRevisionRecord {
    #[must_use]
    pub const fn revision_identity(&self) -> ReportRevisionId {
        self.revision_identity
    }

    #[must_use]
    pub const fn document_id(&self) -> ResultDocumentId {
        self.document_id
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub const fn prior_revision_identity(&self) -> Option<ReportRevisionId> {
        self.prior_revision_identity
    }

    #[must_use]
    pub const fn prior_record_digest(&self) -> Option<ContentDigest> {
        self.prior_record_digest
    }

    #[must_use]
    pub const fn timestamp_unix_ms(&self) -> u64 {
        self.timestamp_unix_ms
    }

    #[must_use]
    pub fn actor(&self) -> &str {
        &self.actor
    }

    #[must_use]
    pub fn revision_note(&self) -> &str {
        &self.revision_note
    }

    #[must_use]
    pub const fn snapshot_serialized_bytes(&self) -> u64 {
        self.snapshot_serialized_bytes
    }

    #[must_use]
    pub const fn snapshot_digest(&self) -> ContentDigest {
        self.snapshot_digest
    }

    #[must_use]
    pub const fn record_digest(&self) -> ContentDigest {
        self.record_digest
    }

    #[must_use]
    pub fn snapshot(&self) -> &ReportDocumentSnapshot {
        &self.snapshot
    }
}

/// Ordered, append-only source-revision ledger owned by one report document.
/// The canonical serialized snapshots are bounded in aggregate so malformed or
/// pathologically large projects cannot create unbounded memory or file growth.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReportRevisionHistory {
    origin: ReportRevisionHistoryOrigin,
    records: Vec<ReportRevisionRecord>,
}

#[derive(Debug, Clone, PartialEq)]
struct BoundedReportRevisionRecords(Vec<ReportRevisionRecord>);

impl<'de> Deserialize<'de> for BoundedReportRevisionRecords {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RecordsVisitor;

        impl<'de> serde::de::Visitor<'de> for RecordsVisitor {
            type Value = BoundedReportRevisionRecords;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    formatter,
                    "at most {MAX_REPORT_REVISION_HISTORY_RECORDS} authenticated report revision snapshots"
                )
            }

            fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                if sequence
                    .size_hint()
                    .is_some_and(|hint| hint > MAX_REPORT_REVISION_HISTORY_RECORDS)
                {
                    return Err(serde::de::Error::custom(invalid_revision_history()));
                }
                let mut records = Vec::with_capacity(
                    sequence
                        .size_hint()
                        .unwrap_or(0)
                        .min(MAX_REPORT_REVISION_HISTORY_RECORDS),
                );
                let mut aggregate_snapshot_bytes = 0_u64;
                loop {
                    if records.len() >= MAX_REPORT_REVISION_HISTORY_RECORDS {
                        if sequence.next_element::<serde::de::IgnoredAny>()?.is_some() {
                            return Err(serde::de::Error::custom(invalid_revision_history()));
                        }
                        break;
                    }
                    let Some(record) = sequence.next_element::<ReportRevisionRecord>()? else {
                        break;
                    };
                    let (snapshot_digest, snapshot_serialized_bytes) =
                        report_snapshot_digest_and_size(&record.snapshot)
                            .map_err(serde::de::Error::custom)?;
                    aggregate_snapshot_bytes = aggregate_snapshot_bytes
                        .checked_add(snapshot_serialized_bytes)
                        .ok_or_else(|| {
                            serde::de::Error::custom(ReportError::CapacityExceeded(
                                "report source revision snapshot bytes",
                            ))
                        })?;
                    if aggregate_snapshot_bytes > MAX_REPORT_REVISION_HISTORY_SNAPSHOT_BYTES
                        || snapshot_serialized_bytes != record.snapshot_serialized_bytes
                        || snapshot_digest != record.snapshot_digest
                    {
                        return Err(serde::de::Error::custom(invalid_revision_history()));
                    }
                    records.push(record);
                }
                Ok(BoundedReportRevisionRecords(records))
            }
        }

        deserializer.deserialize_seq(RecordsVisitor)
    }
}

impl<'de> Deserialize<'de> for ReportRevisionHistory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Wire {
            origin: ReportRevisionHistoryOrigin,
            records: BoundedReportRevisionRecords,
        }

        let wire = Wire::deserialize(deserializer)?;
        for record in &wire.records.0 {
            let record_digest = report_revision_record_digest(
                record.revision_identity,
                record.document_id,
                record.revision,
                wire.origin,
                record.prior_revision_identity,
                record.prior_record_digest,
                record.timestamp_unix_ms,
                &record.actor,
                &record.revision_note,
                record.snapshot_serialized_bytes,
                record.snapshot_digest,
            )
            .map_err(serde::de::Error::custom)?;
            if record_digest != record.record_digest {
                return Err(serde::de::Error::custom(invalid_revision_history()));
            }
        }
        Ok(Self {
            origin: wire.origin,
            records: wire.records.0,
        })
    }
}

impl ReportRevisionHistory {
    #[must_use]
    pub const fn origin(&self) -> ReportRevisionHistoryOrigin {
        self.origin
    }

    #[must_use]
    pub fn records(&self) -> &[ReportRevisionRecord] {
        &self.records
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ReportDocument {
    schema_version: u16,
    id: ResultDocumentId,
    revision: ObjectRevision,
    title: String,
    template: ReportTemplate,
    pages: Vec<ReportPage>,
    receipts: Vec<ReportMutationReceipt>,
    tombstones: Vec<ReportTombstone>,
    legacy_origin_entities: Vec<ReportEntityRef>,
    revision_history: ReportRevisionHistory,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReportDocumentWire {
    schema_version: u16,
    id: ResultDocumentId,
    revision: ObjectRevision,
    title: String,
    template: ReportTemplate,
    #[serde(default)]
    pages: Vec<ReportPage>,
    #[serde(default)]
    receipts: Vec<ReportMutationReceipt>,
    #[serde(default)]
    tombstones: Vec<ReportTombstone>,
    #[serde(default)]
    legacy_origin_entities: Vec<ReportEntityRef>,
    #[serde(default)]
    revision_history: ReportRevisionHistoryWireField,
}

#[derive(Debug, Clone, PartialEq, Default)]
enum ReportRevisionHistoryWireField {
    #[default]
    Missing,
    Null,
    Value(ReportRevisionHistory),
}

impl<'de> Deserialize<'de> for ReportRevisionHistoryWireField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<ReportRevisionHistory>::deserialize(deserializer).map(|value| match value {
            Some(history) => Self::Value(history),
            None => Self::Null,
        })
    }
}

impl<'de> Deserialize<'de> for ReportDocument {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ReportDocumentWire::deserialize(deserializer)?;
        let revision_history = match (wire.schema_version, wire.revision_history) {
            (ReportDocument::SCHEMA_VERSION, ReportRevisionHistoryWireField::Value(history)) => {
                history
            }
            (ReportDocument::SCHEMA_VERSION, ReportRevisionHistoryWireField::Missing) => {
                return Err(serde::de::Error::custom(ReportError::InvalidValue {
                    field: "report-document.revision-history",
                    message: "current-schema documents must retain their revision history"
                        .to_owned(),
                }));
            }
            (1 | 2, ReportRevisionHistoryWireField::Missing) => ReportRevisionHistory {
                origin: ReportRevisionHistoryOrigin::Native,
                records: Vec::new(),
            },
            (_, ReportRevisionHistoryWireField::Missing) => ReportRevisionHistory {
                origin: ReportRevisionHistoryOrigin::Native,
                records: Vec::new(),
            },
            (version, ReportRevisionHistoryWireField::Value(_)) => {
                return Err(serde::de::Error::custom(ReportError::InvalidValue {
                    field: "report-document.revision-history",
                    message: format!(
                        "schema {version} must not contain a revision history introduced by a later schema"
                    ),
                }));
            }
            (_, ReportRevisionHistoryWireField::Null) => {
                return Err(serde::de::Error::custom(ReportError::InvalidValue {
                    field: "report-document.revision-history",
                    message: "revision history must not be null".to_owned(),
                }));
            }
        };
        let mut document = Self {
            schema_version: wire.schema_version,
            id: wire.id,
            revision: wire.revision,
            title: wire.title,
            template: wire.template,
            pages: wire.pages,
            receipts: wire.receipts,
            tombstones: wire.tombstones,
            legacy_origin_entities: wire.legacy_origin_entities,
            revision_history,
        };
        document.migrate().map_err(serde::de::Error::custom)?;
        document.validate().map_err(serde::de::Error::custom)?;
        Ok(document)
    }
}

impl ReportDocument {
    pub const SCHEMA_VERSION: u16 = 3;

    pub fn new(title: impl Into<String>) -> Result<Self, ReportError> {
        Self::new_with_template(title, ReportTemplate::ReleaseVerification42)
    }

    pub fn new_with_template(
        title: impl Into<String>,
        template: ReportTemplate,
    ) -> Result<Self, ReportError> {
        let mut document = Self {
            schema_version: Self::SCHEMA_VERSION,
            id: ResultDocumentId::new(),
            revision: ObjectRevision::INITIAL,
            title: title.into(),
            template,
            pages: Vec::new(),
            receipts: Vec::new(),
            tombstones: Vec::new(),
            legacy_origin_entities: Vec::new(),
            revision_history: ReportRevisionHistory {
                origin: ReportRevisionHistoryOrigin::Native,
                records: Vec::new(),
            },
        };
        document.append_revision_record(
            0,
            "rspice-local-session".to_owned(),
            "Create report document".to_owned(),
        )?;
        document.validate()?;
        Ok(document)
    }

    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
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
    pub fn title(&self) -> &str {
        &self.title
    }

    #[must_use]
    pub const fn template(&self) -> ReportTemplate {
        self.template
    }

    #[must_use]
    pub fn pages(&self) -> &[ReportPage] {
        &self.pages
    }

    #[must_use]
    pub fn receipts(&self) -> &[ReportMutationReceipt] {
        &self.receipts
    }

    #[must_use]
    pub fn tombstones(&self) -> &[ReportTombstone] {
        &self.tombstones
    }

    #[must_use]
    pub const fn revision_history(&self) -> &ReportRevisionHistory {
        &self.revision_history
    }

    /// Resolve one immutable source record by the composite document/revision
    /// identity used by persisted report references.
    #[must_use]
    pub fn revision_record(
        &self,
        document_id: ResultDocumentId,
        revision: ObjectRevision,
    ) -> Option<&ReportRevisionRecord> {
        if document_id != self.id {
            return None;
        }
        self.revision_history
            .records
            .iter()
            .find(|record| record.revision == revision)
    }

    /// Reconstruct an independently valid report document at a retained
    /// revision. The reconstructed document carries only the history prefix
    /// that was known at that point in time.
    pub fn reconstruct_revision(
        &self,
        document_id: ResultDocumentId,
        revision: ObjectRevision,
    ) -> Result<Self, ReportError> {
        if document_id != self.id {
            return Err(ReportError::RevisionNotRetained {
                document_id,
                revision,
            });
        }
        let record_index = self
            .revision_history
            .records
            .iter()
            .position(|record| record.revision == revision)
            .ok_or(ReportError::RevisionNotRetained {
                document_id,
                revision,
            })?;
        let snapshot = &self.revision_history.records[record_index].snapshot;
        let reconstructed = Self {
            schema_version: Self::SCHEMA_VERSION,
            id: snapshot.document_id,
            revision: snapshot.revision,
            title: snapshot.title.clone(),
            template: snapshot.template,
            pages: snapshot.pages.clone(),
            receipts: snapshot.receipts.clone(),
            tombstones: snapshot.tombstones.clone(),
            legacy_origin_entities: snapshot.legacy_origin_entities.clone(),
            revision_history: ReportRevisionHistory {
                origin: self.revision_history.origin,
                records: self.revision_history.records[..=record_index].to_vec(),
            },
        };
        reconstructed.validate()?;
        Ok(reconstructed)
    }

    #[must_use]
    pub fn page(&self, page_id: ReportPageId) -> Option<&ReportPage> {
        self.pages.iter().find(|page| page.id == page_id)
    }

    #[must_use]
    pub fn section(&self, section_id: ReportSectionId) -> Option<&ReportSection> {
        self.pages
            .iter()
            .flat_map(|page| page.sections.iter())
            .find(|section| section.id == section_id)
    }

    #[must_use]
    pub fn block(&self, block_id: ReportBlockId) -> Option<&ReportBlock> {
        self.pages
            .iter()
            .flat_map(|page| page.sections.iter())
            .flat_map(|section| section.blocks.iter())
            .find(|block| block.id == block_id)
    }

    pub fn transact(
        &mut self,
        expected_document_revision: ObjectRevision,
        edits: Vec<ReportEdit>,
        timestamp_unix_ms: u64,
    ) -> Result<ReportMutationReceipt, ReportError> {
        let edit_count = edits.len();
        self.transact_with_context(
            expected_document_revision,
            edits,
            timestamp_unix_ms,
            "rspice-local-session",
            format!("Apply {edit_count} atomic report edit(s)"),
        )
    }

    /// Commit one atomic edit transaction with explicit revision provenance.
    ///
    /// The actor and note are persisted with the complete post-commit source
    /// snapshot; neither is inferred during reload or reconstruction.
    pub fn transact_with_context(
        &mut self,
        expected_document_revision: ObjectRevision,
        edits: Vec<ReportEdit>,
        timestamp_unix_ms: u64,
        actor: impl Into<String>,
        revision_note: impl Into<String>,
    ) -> Result<ReportMutationReceipt, ReportError> {
        if expected_document_revision != self.revision {
            return Err(ReportError::DocumentRevisionConflict {
                expected: expected_document_revision,
                actual: self.revision,
            });
        }
        if edits.is_empty() || edits.len() > MAX_TRANSACTION_EDITS {
            return Err(ReportError::InvalidValue {
                field: "transaction.edits",
                message: format!(
                    "a transaction requires 1 to {MAX_TRANSACTION_EDITS} atomic edits"
                ),
            });
        }
        if self.revision_history.records.len() >= MAX_REPORT_REVISION_HISTORY_RECORDS {
            return Err(ReportError::CapacityExceeded(
                "report source revision history",
            ));
        }
        let actor = actor.into();
        let revision_note = revision_note.into();
        validate_label("report-revision.actor", &actor, 256)?;
        validate_label("report-revision.note", &revision_note, 4_096)?;
        let committed_revision = self.revision.next()?;
        let edit_count = u16::try_from(edits.len()).map_err(|_| ReportError::InvalidValue {
            field: "transaction.edits",
            message: "edit count cannot be represented in a mutation receipt".to_owned(),
        })?;
        // Build the candidate source without cloning the potentially large
        // immutable ledger. The new complete snapshot is measured against the
        // retained aggregate before that ledger is copied, keeping an
        // over-capacity failure atomic and memory-bounded.
        let mut candidate = Self {
            schema_version: self.schema_version,
            id: self.id,
            revision: self.revision,
            title: self.title.clone(),
            template: self.template,
            pages: self.pages.clone(),
            receipts: self.receipts.clone(),
            tombstones: self.tombstones.clone(),
            legacy_origin_entities: self.legacy_origin_entities.clone(),
            revision_history: ReportRevisionHistory {
                origin: self.revision_history.origin,
                records: Vec::new(),
            },
        };
        let mut created = Vec::new();
        let mut changed = Vec::new();
        let mut tombstoned = Vec::new();
        for edit in edits {
            candidate.apply_edit(
                edit,
                committed_revision,
                &mut created,
                &mut changed,
                &mut tombstoned,
            )?;
        }
        deduplicate_entity_receipt(&mut created);
        deduplicate_entity_receipt(&mut changed);
        deduplicate_entity_receipt(&mut tombstoned);
        changed.retain(|entity| !created.contains(entity) && !tombstoned.contains(entity));
        let receipt = ReportMutationReceipt {
            operation_id: Uuid::new_v4(),
            previous_document_revision: self.revision,
            committed_document_revision: committed_revision,
            timestamp_unix_ms,
            edit_count,
            created,
            changed,
            tombstoned,
        };
        candidate.revision = committed_revision;
        candidate.receipts.push(receipt.clone());
        let candidate_snapshot = candidate.current_snapshot();
        let (candidate_snapshot_digest, candidate_snapshot_bytes) =
            report_snapshot_digest_and_size(&candidate_snapshot)?;
        validate_revision_history_snapshot_capacity(
            self.revision_history
                .records
                .iter()
                .map(|record| record.snapshot_serialized_bytes)
                .chain(std::iter::once(candidate_snapshot_bytes)),
        )?;
        candidate.revision_history = self.revision_history.clone();
        candidate.append_prepared_revision_record(
            ReportRevisionId::new(),
            timestamp_unix_ms,
            actor,
            revision_note,
            candidate_snapshot,
            candidate_snapshot_digest,
            candidate_snapshot_bytes,
        )?;
        candidate.validate()?;
        *self = candidate;
        Ok(receipt)
    }

    pub fn audit_references(
        &self,
        inventory: &ReportReferenceInventory,
    ) -> Result<ReportReferenceAudit, ReportError> {
        inventory.validate()?;
        let sources: HashMap<_, _> = inventory
            .sources
            .iter()
            .map(|source| (&source.source, source))
            .collect();
        let mut entries = Vec::new();
        for block in self
            .pages
            .iter()
            .flat_map(|page| page.sections.iter())
            .flat_map(|section| section.blocks.iter())
        {
            let Some(reference) = block.kind.reference() else {
                continue;
            };
            let captured = reference.snapshot();
            let available = sources.get(&captured.source).copied();
            let missing_dataset_bindings = if reference.is_frozen() {
                Vec::new()
            } else {
                captured
                    .dataset_bindings
                    .iter()
                    .filter(|expected| {
                        !inventory.available_datasets.contains(expected)
                            || available
                                .is_some_and(|source| !source.dataset_bindings.contains(expected))
                    })
                    .copied()
                    .collect()
            };
            let currentness = if reference.is_frozen() {
                ReportReferenceCurrentness::Frozen
            } else if available.is_none() {
                ReportReferenceCurrentness::SourceMissing
            } else if !missing_dataset_bindings.is_empty() {
                ReportReferenceCurrentness::DatasetMissing
            } else {
                let available = available.expect("checked above");
                if available.source_revision != captured.source_revision {
                    ReportReferenceCurrentness::UpdateAvailable
                } else if available.content_digest != captured.content_digest {
                    ReportReferenceCurrentness::SourceContentChanged
                } else {
                    ReportReferenceCurrentness::Current
                }
            };
            entries.push(ReportReferenceAuditEntry {
                block_id: block.id,
                source: captured.source.clone(),
                currentness,
                captured_revision: captured.source_revision,
                available_revision: available.and_then(|source| source.source_revision),
                captured_content_digest: captured.content_digest,
                available_content_digest: available.map(|source| source.content_digest),
                frozen_artifact_media_type: reference
                    .frozen_artifact()
                    .map(|artifact| artifact.media_type().to_owned()),
                frozen_artifact_digest: match reference {
                    ReportReferenceMode::Frozen { artifact, .. } => Some(artifact.content_digest()),
                    ReportReferenceMode::Linked { .. } => None,
                },
                missing_dataset_bindings,
            });
        }
        let audit_digest = audit_digest(self.id, self.revision, &entries)?;
        Ok(ReportReferenceAudit {
            document_id: self.id,
            document_revision: self.revision,
            audit_digest,
            entries,
        })
    }

    fn current_snapshot(&self) -> ReportDocumentSnapshot {
        ReportDocumentSnapshot {
            document_id: self.id,
            revision: self.revision,
            title: self.title.clone(),
            template: self.template,
            pages: self.pages.clone(),
            receipts: self.receipts.clone(),
            tombstones: self.tombstones.clone(),
            legacy_origin_entities: self.legacy_origin_entities.clone(),
        }
    }

    fn append_revision_record(
        &mut self,
        timestamp_unix_ms: u64,
        actor: String,
        revision_note: String,
    ) -> Result<(), ReportError> {
        let snapshot = self.current_snapshot();
        let (snapshot_digest, snapshot_serialized_bytes) =
            report_snapshot_digest_and_size(&snapshot)?;
        self.append_prepared_revision_record(
            ReportRevisionId::new(),
            timestamp_unix_ms,
            actor,
            revision_note,
            snapshot,
            snapshot_digest,
            snapshot_serialized_bytes,
        )
    }

    fn append_migrated_revision_record(
        &mut self,
        source_schema_version: u16,
        timestamp_unix_ms: u64,
        revision_note: String,
    ) -> Result<(), ReportError> {
        let snapshot = self.current_snapshot();
        let (snapshot_digest, snapshot_serialized_bytes) =
            report_snapshot_digest_and_size(&snapshot)?;
        let mut identity_material = Vec::with_capacity(16 + 8 + 2 + 32);
        identity_material.extend_from_slice(self.id.as_uuid().as_bytes());
        identity_material.extend_from_slice(&self.revision.get().to_be_bytes());
        identity_material.extend_from_slice(&source_schema_version.to_be_bytes());
        identity_material.extend_from_slice(snapshot_digest.as_bytes());
        let revision_identity = migrated_report_revision_id(&identity_material)?;
        self.append_prepared_revision_record(
            revision_identity,
            timestamp_unix_ms,
            "rspice-schema-migration".to_owned(),
            revision_note,
            snapshot,
            snapshot_digest,
            snapshot_serialized_bytes,
        )
    }

    fn append_prepared_revision_record(
        &mut self,
        revision_identity: ReportRevisionId,
        timestamp_unix_ms: u64,
        actor: String,
        revision_note: String,
        snapshot: ReportDocumentSnapshot,
        snapshot_digest: ContentDigest,
        snapshot_serialized_bytes: u64,
    ) -> Result<(), ReportError> {
        if self.revision_history.records.len() >= MAX_REPORT_REVISION_HISTORY_RECORDS {
            return Err(ReportError::CapacityExceeded(
                "report source revision history",
            ));
        }
        validate_label("report-revision.actor", &actor, 256)?;
        validate_label("report-revision.note", &revision_note, 4_096)?;
        validate_revision_history_snapshot_capacity(
            self.revision_history
                .records
                .iter()
                .map(|record| record.snapshot_serialized_bytes)
                .chain(std::iter::once(snapshot_serialized_bytes)),
        )?;
        let prior_revision_identity = self
            .revision_history
            .records
            .last()
            .map(|record| record.revision_identity);
        let prior_record_digest = self
            .revision_history
            .records
            .last()
            .map(|record| record.record_digest);
        let record_digest = report_revision_record_digest(
            revision_identity,
            self.id,
            self.revision,
            self.revision_history.origin,
            prior_revision_identity,
            prior_record_digest,
            timestamp_unix_ms,
            &actor,
            &revision_note,
            snapshot_serialized_bytes,
            snapshot_digest,
        )?;
        self.revision_history.records.push(ReportRevisionRecord {
            revision_identity,
            document_id: self.id,
            revision: self.revision,
            prior_revision_identity,
            prior_record_digest,
            timestamp_unix_ms,
            actor,
            revision_note,
            snapshot_serialized_bytes,
            snapshot_digest,
            record_digest,
            snapshot,
        });
        Ok(())
    }

    fn migrate(&mut self) -> Result<(), ReportError> {
        match self.schema_version {
            Self::SCHEMA_VERSION => Ok(()),
            1 => {
                // Version one predates mutation receipts and tombstones. It is
                // safe to import only an initial immutable snapshot; accepting
                // a later revision would fabricate the missing audit chain.
                if self.revision != ObjectRevision::INITIAL
                    || !self.receipts.is_empty()
                    || !self.tombstones.is_empty()
                    || !self.legacy_origin_entities.is_empty()
                {
                    return Err(ReportError::UnsafeLegacyMigration {
                        version: self.schema_version,
                    });
                }
                self.legacy_origin_entities = self
                    .pages
                    .iter()
                    .flat_map(|page| {
                        std::iter::once(ReportEntityRef::Page(page.id)).chain(
                            page.sections.iter().flat_map(|section| {
                                std::iter::once(ReportEntityRef::Section(section.id)).chain(
                                    section
                                        .blocks
                                        .iter()
                                        .map(|block| ReportEntityRef::Block(block.id)),
                                )
                            }),
                        )
                    })
                    .collect();
                self.schema_version = Self::SCHEMA_VERSION;
                self.revision_history.origin =
                    ReportRevisionHistoryOrigin::ImportedSchemaOneBaseline;
                self.append_migrated_revision_record(
                    1,
                    0,
                    "Import schema 1 report source baseline".to_owned(),
                )?;
                Ok(())
            }
            2 => {
                // Schema two validates a complete current source plus a
                // contiguous mutation-receipt chain, but it did not retain
                // the prior source bodies. Preserve only the exact source it
                // actually contains and identify it as an imported baseline.
                self.schema_version = Self::SCHEMA_VERSION;
                self.revision_history.origin =
                    ReportRevisionHistoryOrigin::ImportedSchemaTwoBaseline;
                let timestamp_unix_ms = self
                    .receipts
                    .last()
                    .map_or(0, |receipt| receipt.timestamp_unix_ms);
                self.append_migrated_revision_record(
                    2,
                    timestamp_unix_ms,
                    "Import schema 2 report source baseline".to_owned(),
                )?;
                Ok(())
            }
            version => Err(ReportError::UnsupportedSchemaVersion(version)),
        }
    }

}

#[derive(Clone, Copy)]
struct ReportDocumentContentView<'a> {
    revision: ObjectRevision,
    title: &'a str,
    pages: &'a [ReportPage],
    receipts: &'a [ReportMutationReceipt],
    tombstones: &'a [ReportTombstone],
    legacy_origin_entities: &'a [ReportEntityRef],
}

impl<'a> ReportDocumentContentView<'a> {
    fn from_document(document: &'a ReportDocument) -> Self {
        Self {
            revision: document.revision,
            title: &document.title,
            pages: &document.pages,
            receipts: &document.receipts,
            tombstones: &document.tombstones,
            legacy_origin_entities: &document.legacy_origin_entities,
        }
    }

    fn from_snapshot(snapshot: &'a ReportDocumentSnapshot) -> Self {
        Self {
            revision: snapshot.revision,
            title: &snapshot.title,
            pages: &snapshot.pages,
            receipts: &snapshot.receipts,
            tombstones: &snapshot.tombstones,
            legacy_origin_entities: &snapshot.legacy_origin_entities,
        }
    }

    fn validate(self) -> Result<(), ReportError> {
        validate_label("report-document.title", self.title, 512)?;
        if self.pages.len() > MAX_PAGES {
            return Err(ReportError::CapacityExceeded("report pages"));
        }
        let block_count = self
            .pages
            .iter()
            .flat_map(|page| page.sections.iter())
            .map(|section| section.blocks.len())
            .try_fold(0_usize, usize::checked_add)
            .ok_or(ReportError::CapacityExceeded("report content blocks"))?;
        if block_count > MAX_BLOCKS_TOTAL {
            return Err(ReportError::CapacityExceeded("report content blocks"));
        }
        validate_aggregate_frozen_payload_bytes(
            self.pages
                .iter()
                .flat_map(|page| page.sections.iter())
                .flat_map(|section| section.blocks.iter())
                .filter_map(|block| block.kind.reference())
                .filter_map(ReportReferenceMode::frozen_artifact)
                .map(|artifact| artifact.payload().len()),
        )?;

        let mut live = HashSet::new();
        for page in self.pages {
            let page_entity = ReportEntityRef::Page(page.id);
            if !live.insert(page_entity) {
                return Err(ReportError::DuplicateIdentity(page_entity));
            }
            validate_label("report-page.title", &page.title, 512)?;
            self.validate_entity_creation(page_entity, page.created_at_document_revision)?;
            if page.sections.len() > MAX_SECTIONS_PER_PAGE {
                return Err(ReportError::CapacityExceeded("sections per report page"));
            }
            for section in &page.sections {
                let section_entity = ReportEntityRef::Section(section.id);
                if !live.insert(section_entity) {
                    return Err(ReportError::DuplicateIdentity(section_entity));
                }
                validate_label("report-section.title", &section.title, 512)?;
                self.validate_entity_creation(
                    section_entity,
                    section.created_at_document_revision,
                )?;
                if section.blocks.len() > MAX_BLOCKS_PER_SECTION {
                    return Err(ReportError::CapacityExceeded(
                        "content blocks per report section",
                    ));
                }
                for block in &section.blocks {
                    let block_entity = ReportEntityRef::Block(block.id);
                    if !live.insert(block_entity) {
                        return Err(ReportError::DuplicateIdentity(block_entity));
                    }
                    self.validate_entity_creation(
                        block_entity,
                        block.created_at_document_revision,
                    )?;
                    block.kind.validate()?;
                }
            }
        }
        self.validate_tombstones(&live)?;
        self.validate_legacy_origins(&live)?;
        self.validate_receipts(&live)
    }

    fn validate_entity_creation(
        self,
        entity: ReportEntityRef,
        created_at: ObjectRevision,
    ) -> Result<(), ReportError> {
        if created_at > self.revision
            || (created_at == ObjectRevision::INITIAL
                && !self.legacy_origin_entities.contains(&entity))
        {
            return Err(ReportError::InvalidValue {
                field: "report-document.entity-created-at",
                message: "entity creation revisions must name their creation receipt or a migrated version-one origin".to_owned(),
            });
        }
        Ok(())
    }

    fn validate_legacy_origins(self, live: &HashSet<ReportEntityRef>) -> Result<(), ReportError> {
        let tombstoned: HashSet<_> = self
            .tombstones
            .iter()
            .map(|tombstone| tombstone.entity)
            .collect();
        let mut origins = HashSet::with_capacity(self.legacy_origin_entities.len());
        for entity in self.legacy_origin_entities {
            if !origins.insert(*entity) || (!live.contains(entity) && !tombstoned.contains(entity))
            {
                return Err(ReportError::InvalidValue {
                    field: "report-document.legacy-origin-entities",
                    message:
                        "migrated origin identities must be unique and remain live or tombstoned"
                            .to_owned(),
                });
            }
        }
        Ok(())
    }

    fn validate_tombstones(self, live: &HashSet<ReportEntityRef>) -> Result<(), ReportError> {
        let mut tombstoned = HashSet::with_capacity(self.tombstones.len());
        for tombstone in self.tombstones {
            if !tombstoned.insert(tombstone.entity)
                || live.contains(&tombstone.entity)
                || tombstone.created_at_document_revision > tombstone.removed_at_document_revision
                || tombstone.removed_at_document_revision > self.revision
                || tombstone.removed_at_document_revision == ObjectRevision::INITIAL
            {
                return Err(ReportError::InvalidValue {
                    field: "report-document.tombstones",
                    message: "tombstones must be unique, identify no live entity, and name a committed removal revision".to_owned(),
                });
            }
            self.validate_entity_creation(
                tombstone.entity,
                tombstone.created_at_document_revision,
            )?;
        }
        Ok(())
    }

    fn validate_receipts(self, live: &HashSet<ReportEntityRef>) -> Result<(), ReportError> {
        let expected_count = self
            .revision
            .get()
            .checked_sub(1)
            .ok_or(ReportError::RevisionSpaceExhausted)?;
        if usize::try_from(expected_count).ok() != Some(self.receipts.len()) {
            return Err(invalid_receipt());
        }
        let tombstones: HashMap<_, _> = self
            .tombstones
            .iter()
            .map(|tombstone| (tombstone.entity, tombstone))
            .collect();
        let mut operation_ids = HashSet::with_capacity(self.receipts.len());
        let mut created_ids = HashSet::new();
        let mut receipt_tombstones = HashSet::new();
        let legacy_origins: HashSet<_> = self.legacy_origin_entities.iter().copied().collect();
        for (index, receipt) in self.receipts.iter().enumerate() {
            let previous = ObjectRevision::new(
                u64::try_from(index)
                    .map_err(|_| ReportError::RevisionSpaceExhausted)?
                    .checked_add(1)
                    .ok_or(ReportError::RevisionSpaceExhausted)?,
            )?;
            if receipt.operation_id.is_nil()
                || !operation_ids.insert(receipt.operation_id)
                || receipt.previous_document_revision != previous
                || receipt.committed_document_revision != previous.next()?
                || receipt.edit_count == 0
                || usize::from(receipt.edit_count) > MAX_TRANSACTION_EDITS
            {
                return Err(invalid_receipt());
            }
            let mut within_receipt = HashSet::new();
            for entity in receipt
                .created
                .iter()
                .chain(receipt.changed.iter())
                .chain(receipt.tombstoned.iter())
            {
                if !within_receipt.insert(*entity)
                    || (!live.contains(entity) && !tombstones.contains_key(entity))
                {
                    return Err(invalid_receipt());
                }
            }
            for entity in &receipt.created {
                if !created_ids.insert(*entity)
                    || self.entity_created_at(*entity) != Some(receipt.committed_document_revision)
                    || self
                        .entity_removed_at(*entity)
                        .is_some_and(|removed_at| removed_at <= receipt.committed_document_revision)
                {
                    return Err(invalid_receipt());
                }
            }
            for entity in &receipt.changed {
                if self
                    .entity_created_at(*entity)
                    .is_none_or(|created_at| created_at >= receipt.committed_document_revision)
                    || self
                        .entity_removed_at(*entity)
                        .is_some_and(|removed_at| removed_at <= receipt.committed_document_revision)
                {
                    return Err(invalid_receipt());
                }
            }
            for entity in &receipt.tombstoned {
                if !receipt_tombstones.insert(*entity)
                    || tombstones.get(entity).is_none_or(|tombstone| {
                        tombstone.removed_at_document_revision
                            != receipt.committed_document_revision
                            || tombstone.created_at_document_revision
                                >= receipt.committed_document_revision
                    })
                {
                    return Err(invalid_receipt());
                }
            }
        }
        if receipt_tombstones.len() != tombstones.len() {
            return Err(invalid_receipt());
        }
        if self.revision != ObjectRevision::INITIAL {
            for entity in live.iter().chain(tombstones.keys()) {
                if !created_ids.contains(entity) && !legacy_origins.contains(entity) {
                    return Err(invalid_receipt());
                }
            }
        }
        Ok(())
    }

    fn entity_created_at(self, entity: ReportEntityRef) -> Option<ObjectRevision> {
        let live_created_at = match entity {
            ReportEntityRef::Page(id) => self
                .pages
                .iter()
                .find(|page| page.id == id)
                .map(|page| page.created_at_document_revision),
            ReportEntityRef::Section(id) => self
                .pages
                .iter()
                .flat_map(|page| page.sections.iter())
                .find(|section| section.id == id)
                .map(|section| section.created_at_document_revision),
            ReportEntityRef::Block(id) => self
                .pages
                .iter()
                .flat_map(|page| page.sections.iter())
                .flat_map(|section| section.blocks.iter())
                .find(|block| block.id == id)
                .map(|block| block.created_at_document_revision),
        };
        live_created_at.or_else(|| {
            self.tombstones
                .iter()
                .find(|tombstone| tombstone.entity == entity)
                .map(|tombstone| tombstone.created_at_document_revision)
        })
    }

    fn entity_removed_at(self, entity: ReportEntityRef) -> Option<ObjectRevision> {
        self.tombstones
            .iter()
            .find(|tombstone| tombstone.entity == entity)
            .map(|tombstone| tombstone.removed_at_document_revision)
    }
}

fn validate_aggregate_frozen_payload_bytes(
    payload_lengths: impl IntoIterator<Item = usize>,
) -> Result<(), ReportError> {
    let mut aggregate = 0_usize;
    for length in payload_lengths {
        aggregate = aggregate
            .checked_add(length)
            .ok_or(ReportError::CapacityExceeded(
                "aggregate frozen report artifact bytes",
            ))?;
        if aggregate > MAX_FROZEN_ARTIFACT_BYTES_PER_DOCUMENT {
            return Err(ReportError::CapacityExceeded(
                "aggregate frozen report artifact bytes",
            ));
        }
    }
    Ok(())
}

fn require_entity_revision(
    entity: ReportEntityRef,
    expected: ObjectRevision,
    actual: ObjectRevision,
) -> Result<(), ReportError> {
    if expected != actual {
        return Err(ReportError::EntityRevisionConflict {
            entity,
            expected,
            actual,
        });
    }
    Ok(())
}

fn deduplicate_entity_receipt(entities: &mut Vec<ReportEntityRef>) {
    let mut seen = HashSet::with_capacity(entities.len());
    entities.retain(|entity| seen.insert(*entity));
}

fn report_snapshot_digest_and_size(
    snapshot: &ReportDocumentSnapshot,
) -> Result<(ContentDigest, u64), ReportError> {
    struct DigestWriter {
        hasher: Sha256,
        serialized_bytes: u64,
        exceeded_capacity: bool,
    }

    impl DigestWriter {
        fn new() -> Self {
            let mut hasher = Sha256::new();
            hasher.update(b"rspice-report-source-snapshot-v1\0");
            Self {
                hasher,
                serialized_bytes: 0,
                exceeded_capacity: false,
            }
        }
    }

    impl io::Write for DigestWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            let length = u64::try_from(bytes.len()).map_err(|_| {
                self.exceeded_capacity = true;
                io::Error::other("report revision snapshot exceeds addressable size")
            })?;
            let Some(next_size) = self.serialized_bytes.checked_add(length) else {
                self.exceeded_capacity = true;
                return Err(io::Error::other("report revision snapshot size overflowed"));
            };
            if next_size > MAX_REPORT_REVISION_HISTORY_SNAPSHOT_BYTES {
                self.exceeded_capacity = true;
                return Err(io::Error::other(
                    "report revision snapshot exceeds history capacity",
                ));
            }
            self.hasher.update(bytes);
            self.serialized_bytes = next_size;
            Ok(bytes.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = DigestWriter::new();
    if let Err(error) = serde_json::to_writer(&mut writer, snapshot) {
        if writer.exceeded_capacity {
            return Err(ReportError::CapacityExceeded(
                "report source revision snapshot bytes",
            ));
        }
        return Err(ReportError::HistorySerialization(error));
    }
    Ok((
        ContentDigest::from_bytes(writer.hasher.finalize().into()),
        writer.serialized_bytes,
    ))
}

fn validate_revision_history_snapshot_capacity(
    snapshot_lengths: impl IntoIterator<Item = u64>,
) -> Result<(), ReportError> {
    let mut aggregate = 0_u64;
    for length in snapshot_lengths {
        aggregate = aggregate
            .checked_add(length)
            .ok_or(ReportError::CapacityExceeded(
                "report source revision snapshot bytes",
            ))?;
        if aggregate > MAX_REPORT_REVISION_HISTORY_SNAPSHOT_BYTES {
            return Err(ReportError::CapacityExceeded(
                "report source revision snapshot bytes",
            ));
        }
    }
    Ok(())
}

fn migrated_report_revision_id(identity_material: &[u8]) -> Result<ReportRevisionId, ReportError> {
    ReportRevisionId::try_from_uuid(Uuid::new_v5(
        &REPORT_MIGRATION_REVISION_ID_NAMESPACE,
        identity_material,
    ))
}

#[allow(clippy::too_many_arguments)]
fn report_revision_record_digest(
    revision_identity: ReportRevisionId,
    document_id: ResultDocumentId,
    revision: ObjectRevision,
    history_origin: ReportRevisionHistoryOrigin,
    prior_revision_identity: Option<ReportRevisionId>,
    prior_record_digest: Option<ContentDigest>,
    timestamp_unix_ms: u64,
    actor: &str,
    revision_note: &str,
    snapshot_serialized_bytes: u64,
    snapshot_digest: ContentDigest,
) -> Result<ContentDigest, ReportError> {
    #[derive(Serialize)]
    struct RecordDigestMaterial<'a> {
        domain: &'static str,
        revision_identity: ReportRevisionId,
        document_id: ResultDocumentId,
        revision: ObjectRevision,
        history_origin: ReportRevisionHistoryOrigin,
        prior_revision_identity: Option<ReportRevisionId>,
        prior_record_digest: Option<ContentDigest>,
        timestamp_unix_ms: u64,
        actor: &'a str,
        revision_note: &'a str,
        snapshot_serialized_bytes: u64,
        snapshot_digest: ContentDigest,
    }
    let bytes = serde_json::to_vec(&RecordDigestMaterial {
        domain: "rspice-report-source-revision-record-v1",
        revision_identity,
        document_id,
        revision,
        history_origin,
        prior_revision_identity,
        prior_record_digest,
        timestamp_unix_ms,
        actor,
        revision_note,
        snapshot_serialized_bytes,
        snapshot_digest,
    })
    .map_err(ReportError::HistorySerialization)?;
    Ok(ContentDigest::from_bytes(Sha256::digest(bytes).into()))
}

fn audit_digest(
    document_id: ResultDocumentId,
    document_revision: ObjectRevision,
    entries: &[ReportReferenceAuditEntry],
) -> Result<ContentDigest, ReportError> {
    #[derive(Serialize)]
    struct AuditDigestMaterial<'a> {
        domain: &'static str,
        document_id: ResultDocumentId,
        document_revision: ObjectRevision,
        entries: &'a [ReportReferenceAuditEntry],
    }
    let material = AuditDigestMaterial {
        domain: "rspice-report-reference-audit-v1",
        document_id,
        document_revision,
        entries,
    };
    let bytes = serde_json::to_vec(&material).map_err(ReportError::AuditSerialization)?;
    Ok(ContentDigest::from_bytes(Sha256::digest(bytes).into()))
}

fn invalid_receipt() -> ReportError {
    ReportError::InvalidValue {
        field: "report-document.receipts",
        message: "mutation receipts must be complete, contiguous, unique, and agree with live identities and tombstones".to_owned(),
    }
}

fn invalid_revision_history() -> ReportError {
    ReportError::InvalidValue {
        field: "report-document.revision-history",
        message: "revision records must be bounded, contiguous, digest-authenticated complete snapshots linked by unique immutable identities".to_owned(),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ReportError {
    #[error("report identity must not be the nil UUID")]
    NilIdentity,
    #[error("unsupported report document schema version {0}")]
    UnsupportedSchemaVersion(u16),
    #[error("schema version {version} cannot be migrated without fabricating audit history")]
    UnsafeLegacyMigration { version: u16 },
    #[error("invalid {field}: {message}")]
    InvalidValue {
        field: &'static str,
        message: String,
    },
    #[error("duplicate key '{0}'")]
    DuplicateKey(String),
    #[error("duplicate immutable binding for dataset {0}")]
    DuplicateDatasetBinding(DatasetId),
    #[error("duplicate reference inventory source {0:?}")]
    DuplicateReferenceSource(ReportSourceId),
    #[error("duplicate report entity identity {0:?}")]
    DuplicateIdentity(ReportEntityRef),
    #[error("{block} blocks require a {expected} source reference")]
    InvalidReferenceKind {
        block: &'static str,
        expected: &'static str,
    },
    #[error("this content block has no external reference")]
    BlockHasNoExternalReference,
    #[error("report entity {0:?} was not found")]
    EntityNotFound(ReportEntityRef),
    #[error("transaction expected document revision {expected:?}, current revision is {actual:?}")]
    DocumentRevisionConflict {
        expected: ObjectRevision,
        actual: ObjectRevision,
    },
    #[error(
        "report entity {entity:?} expected revision {expected:?}, current revision is {actual:?}"
    )]
    EntityRevisionConflict {
        entity: ReportEntityRef,
        expected: ObjectRevision,
        actual: ObjectRevision,
    },
    #[error("report edit would make no change")]
    NoChanges,
    #[error("move target must be a distinct sibling in the requested destination")]
    InvalidMoveTarget,
    #[error("report capacity exceeded for {0}")]
    CapacityExceeded(&'static str),
    #[error("report revision space is exhausted")]
    RevisionSpaceExhausted,
    #[error("report source revision {revision:?} is not retained for document {document_id}")]
    RevisionNotRetained {
        document_id: ResultDocumentId,
        revision: ObjectRevision,
    },
    #[error("failed to serialize report revision-history digest material: {0}")]
    HistorySerialization(serde_json::Error),
    #[error("failed to serialize reference audit digest material: {0}")]
    AuditSerialization(serde_json::Error),
}

impl From<RevisionError> for ReportError {
    fn from(error: RevisionError) -> Self {
        match error {
            RevisionError::Zero => Self::InvalidValue {
                field: "revision",
                message: "object revisions must be greater than zero".to_owned(),
            },
            RevisionError::Exhausted => Self::RevisionSpaceExhausted,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn digest(seed: u8) -> ContentDigest {
        ContentDigest::from_bytes([seed; 32])
    }

    fn artifact(media_type: &str, seed: u8) -> FrozenReportArtifact {
        FrozenReportArtifact::new(media_type, vec![seed; 32]).unwrap()
    }

    fn dataset_snapshot(seed: u8) -> (ReportReferenceSnapshot, DatasetBinding) {
        let binding = DatasetBinding::new(DatasetId::new(), digest(seed));
        (
            ReportReferenceSnapshot::new(
                ReportSourceId::Dataset {
                    dataset_id: binding.dataset_id,
                },
                None,
                binding.content_digest,
                vec![binding],
            )
            .unwrap(),
            binding,
        )
    }

    fn visualization_snapshot(
        revision: ObjectRevision,
        seed: u8,
    ) -> (ReportReferenceSnapshot, DatasetBinding) {
        let binding = DatasetBinding::new(DatasetId::new(), digest(seed));
        (
            ReportReferenceSnapshot::new(
                ReportSourceId::VisualizationDocument {
                    document_id: ResultDocumentId::new(),
                },
                Some(revision),
                digest(seed.wrapping_add(1)),
                vec![binding],
            )
            .unwrap(),
            binding,
        )
    }

    fn external_snapshot(namespace: &str, seed: u8) -> ReportReferenceSnapshot {
        ReportReferenceSnapshot::new(
            ReportSourceId::ExternalRecord {
                namespace: namespace.to_owned(),
                key: format!("record-{seed}"),
            },
            Some(ObjectRevision::INITIAL),
            digest(seed),
            Vec::new(),
        )
        .unwrap()
    }

    fn evidence_snapshot(seed: u8) -> ReportReferenceSnapshot {
        ReportReferenceSnapshot::new(
            ReportSourceId::VerificationEvidence {
                evidence_id: VerificationEvidenceId::new(),
            },
            Some(ObjectRevision::INITIAL),
            digest(seed),
            Vec::new(),
        )
        .unwrap()
    }

    fn document_with_section() -> (ReportDocument, ReportPageId, ReportSectionId) {
        let mut document = ReportDocument::new("Release verification 4.2").unwrap();
        let receipt = document
            .transact(
                document.revision(),
                vec![ReportEdit::AddPage {
                    title: "Performance".to_owned(),
                }],
                10,
            )
            .unwrap();
        let page_id = match receipt.created[0] {
            ReportEntityRef::Page(id) => id,
            _ => unreachable!(),
        };
        let receipt = document
            .transact(
                document.revision(),
                vec![ReportEdit::AddSection {
                    page_id,
                    title: "Nominal and corners".to_owned(),
                }],
                11,
            )
            .unwrap();
        let section_id = match receipt.created[0] {
            ReportEntityRef::Section(id) => id,
            _ => unreachable!(),
        };
        (document, page_id, section_id)
    }

    fn all_block_kinds() -> Vec<ReportBlockKind> {
        let (plot_snapshot, _) = visualization_snapshot(ObjectRevision::INITIAL, 10);
        let (table_snapshot, _) = dataset_snapshot(20);
        let (datasheet_snapshot, _) = dataset_snapshot(21);
        vec![
            ReportBlockKind::PlotFigure(PlotFigureBlock {
                caption: "Closed-loop gain".to_owned(),
                alternative_text: "Gain and phase across the requested frequency range.".to_owned(),
                sizing: FigureSizing::FitWidth,
                reference: ReportReferenceMode::Frozen {
                    snapshot: plot_snapshot,
                    artifact: FrozenReportArtifact::new(
                        "image/svg+xml",
                        b"<svg xmlns=\"http://www.w3.org/2000/svg\"></svg>".to_vec(),
                    )
                    .unwrap(),
                },
            }),
            ReportBlockKind::DataTable(DataTableBlock {
                title: "Corner summary".to_owned(),
                columns: vec![
                    TableColumn {
                        key: "corner".to_owned(),
                        heading: "Corner".to_owned(),
                        unit: None,
                    },
                    TableColumn {
                        key: "gain".to_owned(),
                        heading: "Gain".to_owned(),
                        unit: Some("dB".to_owned()),
                    },
                ],
                rows: vec![vec![
                    TableCell::Text("tt".to_owned()),
                    TableCell::Number {
                        value: 42.25,
                        unit: Some("dB".to_owned()),
                    },
                ]],
                reference: ReportReferenceMode::Frozen {
                    snapshot: table_snapshot,
                    artifact: artifact("application/json", 22),
                },
            }),
            ReportBlockKind::Datasheet(DatasheetBlock {
                title: "Device summary".to_owned(),
                fields: vec![DatasheetField {
                    key: "vdd".to_owned(),
                    label: "Supply voltage".to_owned(),
                    value: "1.8".to_owned(),
                    unit: Some("V".to_owned()),
                }],
                reference: ReportReferenceMode::Linked {
                    snapshot: datasheet_snapshot,
                },
            }),
            ReportBlockKind::Requirements(RequirementsBlock {
                title: "Requirements".to_owned(),
                entries: vec![RequirementEntry {
                    requirement_id: "REQ-AMP-001".to_owned(),
                    statement: "Nominal gain shall exceed 40 dB.".to_owned(),
                    disposition: RequirementDisposition::Passed,
                    evidence_label: Some("AC gain measurement".to_owned()),
                }],
                reference: ReportReferenceMode::Linked {
                    snapshot: external_snapshot("requirements", 30),
                },
            }),
            ReportBlockKind::Specifications(SpecificationsBlock {
                title: "Specifications".to_owned(),
                entries: vec![SpecificationEntry {
                    expression: "max(gain_db)".to_owned(),
                    limit: ">= 40 dB".to_owned(),
                    measured: Some("42.25 dB".to_owned()),
                    disposition: SpecificationDisposition::InSpecification,
                }],
                reference: ReportReferenceMode::Frozen {
                    snapshot: external_snapshot("specifications", 31),
                    artifact: artifact("application/json", 32),
                },
            }),
            ReportBlockKind::Prose(ProseBlock {
                style: ProseStyle::Method,
                markdown: "The amplifier was evaluated over the governed corner plan.".to_owned(),
            }),
            ReportBlockKind::ReviewNote(ReviewNoteBlock {
                author: "A. Reviewer".to_owned(),
                status: ReviewNoteStatus::Open,
                message: "Confirm the hot-corner phase margin.".to_owned(),
                created_at_unix_ms: 100,
                resolved_at_unix_ms: None,
            }),
            ReportBlockKind::Evidence(EvidenceBlock {
                title: "Verification receipt".to_owned(),
                summary: "Immutable evidence retained by the verification plan.".to_owned(),
                reference: ReportReferenceMode::Frozen {
                    snapshot: evidence_snapshot(40),
                    artifact: artifact("application/json", 41),
                },
            }),
        ]
    }

    #[test]
    fn complete_typed_composer_graph_round_trips_and_retains_exact_bindings() {
        let (mut document, _, section_id) = document_with_section();
        let kinds = all_block_kinds();
        let expected_bindings: Vec<_> = kinds
            .iter()
            .filter_map(ReportBlockKind::reference)
            .flat_map(|reference| reference.snapshot().dataset_bindings.iter().copied())
            .collect();
        let receipt = document
            .transact(
                document.revision(),
                kinds
                    .into_iter()
                    .map(|kind| ReportEdit::AddBlock { section_id, kind })
                    .collect(),
                12,
            )
            .unwrap();
        assert_eq!(receipt.created.len(), 8);
        assert_eq!(document.section(section_id).unwrap().blocks().len(), 8);

        let encoded = serde_json::to_vec(&document).unwrap();
        let decoded: ReportDocument = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(decoded, document);
        let decoded_bindings: Vec<_> = decoded
            .pages()
            .iter()
            .flat_map(|page| page.sections())
            .flat_map(|section| section.blocks())
            .filter_map(|block| block.kind().reference())
            .flat_map(|reference| reference.snapshot().dataset_bindings.iter().copied())
            .collect();
        assert_eq!(decoded_bindings, expected_bindings);
    }

    #[test]
    fn transactions_are_optimistic_atomic_and_reject_stale_entity_revisions() {
        let (mut document, page_id, section_id) = document_with_section();
        let before = document.clone();
        let error = document
            .transact(
                ObjectRevision::INITIAL,
                vec![ReportEdit::AddBlock {
                    section_id,
                    kind: all_block_kinds().remove(0),
                }],
                20,
            )
            .unwrap_err();
        assert!(matches!(
            error,
            ReportError::DocumentRevisionConflict { .. }
        ));
        assert_eq!(document, before);

        let page_revision = document.page(page_id).unwrap().revision();
        let before = document.clone();
        let error = document
            .transact(
                document.revision(),
                vec![
                    ReportEdit::UpdatePageTitle {
                        page_id,
                        expected_page_revision: page_revision,
                        title: "Updated".to_owned(),
                    },
                    ReportEdit::UpdatePageTitle {
                        page_id,
                        expected_page_revision: page_revision,
                        title: "Stale second edit".to_owned(),
                    },
                ],
                21,
            )
            .unwrap_err();
        assert!(matches!(error, ReportError::EntityRevisionConflict { .. }));
        assert_eq!(document, before);
    }

    #[test]
    fn linked_and_frozen_reference_currentness_is_explicit_and_auditable() {
        let (mut document, _, section_id) = document_with_section();
        let (linked_snapshot, linked_dataset) = visualization_snapshot(ObjectRevision::INITIAL, 50);
        let linked_source = linked_snapshot.source.clone();
        let linked_digest = linked_snapshot.content_digest;
        let (frozen_snapshot, _) = dataset_snapshot(60);
        let receipt = document
            .transact(
                document.revision(),
                vec![
                    ReportEdit::AddBlock {
                        section_id,
                        kind: ReportBlockKind::PlotFigure(PlotFigureBlock {
                            caption: "Gain".to_owned(),
                            alternative_text: "Gain versus frequency.".to_owned(),
                            sizing: FigureSizing::FitWidth,
                            reference: ReportReferenceMode::Linked {
                                snapshot: linked_snapshot.clone(),
                            },
                        }),
                    },
                    ReportEdit::AddBlock {
                        section_id,
                        kind: ReportBlockKind::Datasheet(DatasheetBlock {
                            title: "Frozen operating point".to_owned(),
                            fields: vec![DatasheetField {
                                key: "id".to_owned(),
                                label: "Drain current".to_owned(),
                                value: "2.4".to_owned(),
                                unit: Some("mA".to_owned()),
                            }],
                            reference: ReportReferenceMode::Frozen {
                                snapshot: frozen_snapshot,
                                artifact: artifact("application/json", 61),
                            },
                        }),
                    },
                ],
                30,
            )
            .unwrap();
        let linked_block = match receipt.created[0] {
            ReportEntityRef::Block(id) => id,
            _ => unreachable!(),
        };
        let inventory = ReportReferenceInventory {
            sources: vec![
                ReportReferenceInventoryEntry::new(
                    linked_source.clone(),
                    Some(ObjectRevision::new(2).unwrap()),
                    linked_digest,
                    vec![linked_dataset],
                )
                .unwrap(),
            ],
            available_datasets: vec![linked_dataset],
        };
        let first = document.audit_references(&inventory).unwrap();
        assert_eq!(
            first.entries[0].currentness,
            ReportReferenceCurrentness::UpdateAvailable
        );
        assert_eq!(
            first.entries[1].currentness,
            ReportReferenceCurrentness::Frozen
        );
        assert!(!first.is_current_for_sign_off());

        let refreshed = ReportReferenceSnapshot::new(
            linked_source,
            Some(ObjectRevision::new(2).unwrap()),
            linked_digest,
            vec![linked_dataset],
        )
        .unwrap();
        let block_revision = document.block(linked_block).unwrap().revision();
        document
            .transact(
                document.revision(),
                vec![ReportEdit::UpdateBlockReference {
                    block_id: linked_block,
                    expected_block_revision: block_revision,
                    reference: ReportReferenceMode::Linked {
                        snapshot: refreshed,
                    },
                }],
                31,
            )
            .unwrap();
        let second = document.audit_references(&inventory).unwrap();
        assert!(second.is_current_for_sign_off());
        assert_ne!(first.audit_digest, second.audit_digest);
    }

    #[test]
    fn audit_distinguishes_missing_source_dataset_and_changed_content() {
        let (mut document, _, section_id) = document_with_section();
        let (snapshot, binding) = visualization_snapshot(ObjectRevision::INITIAL, 70);
        let source = snapshot.source.clone();
        let receipt = document
            .transact(
                document.revision(),
                vec![ReportEdit::AddBlock {
                    section_id,
                    kind: ReportBlockKind::PlotFigure(PlotFigureBlock {
                        caption: "Noise".to_owned(),
                        alternative_text: "Input-referred noise density.".to_owned(),
                        sizing: FigureSizing::FitWidth,
                        reference: ReportReferenceMode::Linked {
                            snapshot: snapshot.clone(),
                        },
                    }),
                }],
                40,
            )
            .unwrap();
        assert!(matches!(receipt.created[0], ReportEntityRef::Block(_)));

        let missing = document
            .audit_references(&ReportReferenceInventory::default())
            .unwrap();
        assert_eq!(
            missing.entries[0].currentness,
            ReportReferenceCurrentness::SourceMissing
        );

        let changed_entry = ReportReferenceInventoryEntry::new(
            source,
            snapshot.source_revision,
            digest(99),
            vec![binding],
        )
        .unwrap();
        let changed = document
            .audit_references(&ReportReferenceInventory {
                sources: vec![changed_entry.clone()],
                available_datasets: vec![binding],
            })
            .unwrap();
        assert_eq!(
            changed.entries[0].currentness,
            ReportReferenceCurrentness::SourceContentChanged
        );

        let missing_dataset = document
            .audit_references(&ReportReferenceInventory {
                sources: vec![ReportReferenceInventoryEntry {
                    content_digest: snapshot.content_digest,
                    ..changed_entry
                }],
                available_datasets: Vec::new(),
            })
            .unwrap();
        assert_eq!(
            missing_dataset.entries[0].currentness,
            ReportReferenceCurrentness::DatasetMissing
        );
        assert_eq!(
            missing_dataset.entries[0].missing_dataset_bindings,
            vec![binding]
        );
    }

    #[test]
    fn cascade_removal_records_every_identity_and_complete_receipt_chain() {
        let (mut document, page_id, section_id) = document_with_section();
        document
            .transact(
                document.revision(),
                vec![ReportEdit::AddBlock {
                    section_id,
                    kind: all_block_kinds().remove(5),
                }],
                50,
            )
            .unwrap();
        let page_revision = document.page(page_id).unwrap().revision();
        let receipt = document
            .transact(
                document.revision(),
                vec![ReportEdit::Remove {
                    entity: ReportEntityRef::Page(page_id),
                    expected_entity_revision: page_revision,
                }],
                51,
            )
            .unwrap();
        assert_eq!(receipt.tombstoned.len(), 3);
        assert!(document.pages().is_empty());
        assert_eq!(document.tombstones().len(), 3);
        let decoded: ReportDocument =
            serde_json::from_slice(&serde_json::to_vec(&document).unwrap()).unwrap();
        assert_eq!(decoded, document);
    }

    #[test]
    fn pages_sections_and_blocks_can_be_reordered_without_identity_loss() {
        let (mut document, first_page, first_section) = document_with_section();
        let page_receipt = document
            .transact(
                document.revision(),
                vec![ReportEdit::AddPage {
                    title: "Appendix".to_owned(),
                }],
                60,
            )
            .unwrap();
        let second_page = match page_receipt.created[0] {
            ReportEntityRef::Page(id) => id,
            _ => unreachable!(),
        };
        let section_receipt = document
            .transact(
                document.revision(),
                vec![ReportEdit::AddSection {
                    page_id: second_page,
                    title: "Raw evidence".to_owned(),
                }],
                61,
            )
            .unwrap();
        let second_section = match section_receipt.created[0] {
            ReportEntityRef::Section(id) => id,
            _ => unreachable!(),
        };
        let blocks = document
            .transact(
                document.revision(),
                vec![
                    ReportEdit::AddBlock {
                        section_id: first_section,
                        kind: all_block_kinds().remove(5),
                    },
                    ReportEdit::AddBlock {
                        section_id: first_section,
                        kind: all_block_kinds().remove(6),
                    },
                ],
                62,
            )
            .unwrap();
        let first_block = match blocks.created[0] {
            ReportEntityRef::Block(id) => id,
            _ => unreachable!(),
        };
        let first_block_revision = document.block(first_block).unwrap().revision();
        document
            .transact(
                document.revision(),
                vec![ReportEdit::MoveBlock {
                    block_id: first_block,
                    expected_block_revision: first_block_revision,
                    target_section_id: second_section,
                    before: None,
                }],
                63,
            )
            .unwrap();
        assert_eq!(
            document.section(second_section).unwrap().blocks()[0].id(),
            first_block
        );

        let section_revision = document.section(first_section).unwrap().revision();
        document
            .transact(
                document.revision(),
                vec![ReportEdit::MoveSection {
                    section_id: first_section,
                    expected_section_revision: section_revision,
                    target_page_id: second_page,
                    before: Some(second_section),
                }],
                64,
            )
            .unwrap();
        assert_eq!(
            document.page(second_page).unwrap().sections()[0].id(),
            first_section
        );

        let page_revision = document.page(second_page).unwrap().revision();
        document
            .transact(
                document.revision(),
                vec![ReportEdit::MovePage {
                    page_id: second_page,
                    expected_page_revision: page_revision,
                    before: Some(first_page),
                }],
                65,
            )
            .unwrap();
        assert_eq!(document.pages()[0].id(), second_page);
    }

    #[test]
    fn page_move_handles_forward_one_forward_many_backward_and_end_positions() {
        let mut document = ReportDocument::new("Ordering").unwrap();
        let receipt = document
            .transact(
                document.revision(),
                ["A", "B", "C", "D"]
                    .into_iter()
                    .map(|title| ReportEdit::AddPage {
                        title: title.to_owned(),
                    })
                    .collect(),
                66,
            )
            .unwrap();
        let ids: Vec<_> = receipt
            .created
            .iter()
            .map(|entity| match entity {
                ReportEntityRef::Page(id) => *id,
                _ => unreachable!(),
            })
            .collect();
        let titles = |document: &ReportDocument| {
            document
                .pages()
                .iter()
                .map(|page| page.title().to_owned())
                .collect::<Vec<_>>()
        };

        document
            .transact(
                document.revision(),
                vec![ReportEdit::MovePage {
                    page_id: ids[0],
                    expected_page_revision: document.page(ids[0]).unwrap().revision(),
                    before: Some(ids[2]),
                }],
                67,
            )
            .unwrap();
        assert_eq!(titles(&document), ["B", "A", "C", "D"]);

        document
            .transact(
                document.revision(),
                vec![ReportEdit::MovePage {
                    page_id: ids[0],
                    expected_page_revision: document.page(ids[0]).unwrap().revision(),
                    before: None,
                }],
                68,
            )
            .unwrap();
        assert_eq!(titles(&document), ["B", "C", "D", "A"]);

        document
            .transact(
                document.revision(),
                vec![ReportEdit::MovePage {
                    page_id: ids[0],
                    expected_page_revision: document.page(ids[0]).unwrap().revision(),
                    before: Some(ids[2]),
                }],
                69,
            )
            .unwrap();
        assert_eq!(titles(&document), ["B", "A", "C", "D"]);

        document
            .transact(
                document.revision(),
                vec![ReportEdit::MovePage {
                    page_id: ids[0],
                    expected_page_revision: document.page(ids[0]).unwrap().revision(),
                    before: Some(ids[1]),
                }],
                70,
            )
            .unwrap();
        assert_eq!(titles(&document), ["A", "B", "C", "D"]);

        let before = document.clone();
        assert!(matches!(
            document.transact(
                document.revision(),
                vec![ReportEdit::MovePage {
                    page_id: ids[2],
                    expected_page_revision: document.page(ids[2]).unwrap().revision(),
                    before: Some(ids[3]),
                }],
                71,
            ),
            Err(ReportError::NoChanges)
        ));
        assert_eq!(document, before);
    }

    #[test]
    fn section_and_block_moves_use_pre_removal_indices_correctly() {
        let mut document = ReportDocument::new("Nested ordering").unwrap();
        let page_receipt = document
            .transact(
                document.revision(),
                vec![ReportEdit::AddPage {
                    title: "Page".to_owned(),
                }],
                71,
            )
            .unwrap();
        let page_id = match page_receipt.created[0] {
            ReportEntityRef::Page(id) => id,
            _ => unreachable!(),
        };
        let sections = document
            .transact(
                document.revision(),
                ["A", "B", "C"]
                    .into_iter()
                    .map(|title| ReportEdit::AddSection {
                        page_id,
                        title: title.to_owned(),
                    })
                    .collect(),
                72,
            )
            .unwrap();
        let section_ids: Vec<_> = sections
            .created
            .iter()
            .map(|entity| match entity {
                ReportEntityRef::Section(id) => *id,
                _ => unreachable!(),
            })
            .collect();
        document
            .transact(
                document.revision(),
                vec![ReportEdit::MoveSection {
                    section_id: section_ids[0],
                    expected_section_revision: document.section(section_ids[0]).unwrap().revision(),
                    target_page_id: page_id,
                    before: Some(section_ids[2]),
                }],
                73,
            )
            .unwrap();
        assert_eq!(
            document
                .page(page_id)
                .unwrap()
                .sections()
                .iter()
                .map(ReportSection::title)
                .collect::<Vec<_>>(),
            ["B", "A", "C"]
        );
        document
            .transact(
                document.revision(),
                vec![ReportEdit::MoveSection {
                    section_id: section_ids[0],
                    expected_section_revision: document.section(section_ids[0]).unwrap().revision(),
                    target_page_id: page_id,
                    before: None,
                }],
                74,
            )
            .unwrap();
        assert_eq!(
            document
                .page(page_id)
                .unwrap()
                .sections()
                .iter()
                .map(ReportSection::title)
                .collect::<Vec<_>>(),
            ["B", "C", "A"]
        );

        let target_section = section_ids[1];
        let blocks = document
            .transact(
                document.revision(),
                (0..3)
                    .map(|index| ReportEdit::AddBlock {
                        section_id: target_section,
                        kind: ReportBlockKind::Prose(ProseBlock {
                            style: ProseStyle::Body,
                            markdown: format!("Block {index}"),
                        }),
                    })
                    .collect(),
                75,
            )
            .unwrap();
        let block_ids: Vec<_> = blocks
            .created
            .iter()
            .map(|entity| match entity {
                ReportEntityRef::Block(id) => *id,
                _ => unreachable!(),
            })
            .collect();
        document
            .transact(
                document.revision(),
                vec![ReportEdit::MoveBlock {
                    block_id: block_ids[0],
                    expected_block_revision: document.block(block_ids[0]).unwrap().revision(),
                    target_section_id: target_section,
                    before: Some(block_ids[2]),
                }],
                76,
            )
            .unwrap();
        assert_eq!(
            document
                .section(target_section)
                .unwrap()
                .blocks()
                .iter()
                .map(ReportBlock::id)
                .collect::<Vec<_>>(),
            [block_ids[1], block_ids[0], block_ids[2]]
        );
        document
            .transact(
                document.revision(),
                vec![ReportEdit::MoveBlock {
                    block_id: block_ids[0],
                    expected_block_revision: document.block(block_ids[0]).unwrap().revision(),
                    target_section_id: target_section,
                    before: None,
                }],
                77,
            )
            .unwrap();
        assert_eq!(
            document
                .section(target_section)
                .unwrap()
                .blocks()
                .iter()
                .map(ReportBlock::id)
                .collect::<Vec<_>>(),
            [block_ids[1], block_ids[2], block_ids[0]]
        );
    }

    #[test]
    fn document_template_and_page_update_policy_are_transactional() {
        let (mut document, page_id, _) = document_with_section();
        assert_eq!(document.template(), ReportTemplate::ReleaseVerification42);
        assert_eq!(
            document.page(page_id).unwrap().update_policy(),
            ReportPageUpdatePolicy::RefreshLinkedAutomatically
        );
        let page_revision = document.page(page_id).unwrap().revision();
        document
            .transact(
                document.revision(),
                vec![
                    ReportEdit::SetTemplate {
                        template: ReportTemplate::ModelQualification,
                    },
                    ReportEdit::SetPageUpdatePolicy {
                        page_id,
                        expected_page_revision: page_revision,
                        update_policy: ReportPageUpdatePolicy::FreezeSelectedRevision,
                    },
                ],
                72,
            )
            .unwrap();
        assert_eq!(document.template(), ReportTemplate::ModelQualification);
        assert_eq!(
            document.page(page_id).unwrap().update_policy(),
            ReportPageUpdatePolicy::FreezeSelectedRevision
        );
    }

    #[test]
    fn invalid_tables_notes_sources_and_duplicate_bindings_fail_closed() {
        let (snapshot, binding) = dataset_snapshot(80);
        let invalid_table = ReportBlockKind::DataTable(DataTableBlock {
            title: "Broken".to_owned(),
            columns: vec![TableColumn {
                key: "value".to_owned(),
                heading: "Value".to_owned(),
                unit: None,
            }],
            rows: vec![vec![TableCell::Integer(1), TableCell::Integer(2)]],
            reference: ReportReferenceMode::Linked { snapshot },
        });
        assert!(invalid_table.validate().is_err());

        let invalid_note = ReportBlockKind::ReviewNote(ReviewNoteBlock {
            author: "Reviewer".to_owned(),
            status: ReviewNoteStatus::Accepted,
            message: "Accepted without a time.".to_owned(),
            created_at_unix_ms: 1,
            resolved_at_unix_ms: None,
        });
        assert!(invalid_note.validate().is_err());

        let wrong_plot = ReportBlockKind::PlotFigure(PlotFigureBlock {
            caption: "Wrong source".to_owned(),
            alternative_text: "This must fail source-kind validation.".to_owned(),
            sizing: FigureSizing::Natural,
            reference: ReportReferenceMode::Linked {
                snapshot: dataset_snapshot(81).0,
            },
        });
        assert!(matches!(
            wrong_plot.validate(),
            Err(ReportError::InvalidReferenceKind { .. })
        ));

        assert!(matches!(
            ReportReferenceSnapshot::new(
                ReportSourceId::Dataset {
                    dataset_id: binding.dataset_id
                },
                None,
                binding.content_digest,
                vec![binding, binding],
            ),
            Err(ReportError::DuplicateDatasetBinding(_))
        ));
    }

    #[test]
    fn serde_rejects_broken_receipts_nil_ids_and_unsupported_versions() {
        let (mut document, _, section_id) = document_with_section();
        document
            .transact(
                document.revision(),
                vec![ReportEdit::AddBlock {
                    section_id,
                    kind: all_block_kinds().remove(5),
                }],
                70,
            )
            .unwrap();
        let value = serde_json::to_value(&document).unwrap();

        let mut broken = value.clone();
        broken["receipts"][0]["committed_document_revision"] = serde_json::json!(99);
        assert!(serde_json::from_value::<ReportDocument>(broken).is_err());

        let mut nil = value.clone();
        nil["pages"][0]["id"] = serde_json::json!(Uuid::nil());
        assert!(serde_json::from_value::<ReportDocument>(nil).is_err());

        let mut future = value;
        future["schema_version"] = serde_json::json!(u16::MAX);
        assert!(serde_json::from_value::<ReportDocument>(future).is_err());
    }

    #[test]
    fn receipt_validation_rejects_changes_before_creation_and_after_tombstone() {
        let mut future_change = ReportDocument::new("Temporal audit").unwrap();
        future_change
            .transact(
                future_change.revision(),
                vec![ReportEdit::SetDocumentTitle {
                    title: "Temporal audit updated".to_owned(),
                }],
                73,
            )
            .unwrap();
        let created = future_change
            .transact(
                future_change.revision(),
                vec![ReportEdit::AddPage {
                    title: "Created later".to_owned(),
                }],
                74,
            )
            .unwrap();
        let page = created.created[0];
        let mut corrupted = serde_json::to_value(&future_change).unwrap();
        corrupted["receipts"][0]["changed"] = serde_json::json!([page]);
        assert!(serde_json::from_value::<ReportDocument>(corrupted).is_err());

        let page_id = match page {
            ReportEntityRef::Page(id) => id,
            _ => unreachable!(),
        };
        let page_revision = future_change.page(page_id).unwrap().revision();
        future_change
            .transact(
                future_change.revision(),
                vec![ReportEdit::Remove {
                    entity: page,
                    expected_entity_revision: page_revision,
                }],
                75,
            )
            .unwrap();
        future_change
            .transact(
                future_change.revision(),
                vec![ReportEdit::SetDocumentTitle {
                    title: "Changed after deletion".to_owned(),
                }],
                76,
            )
            .unwrap();
        let mut corrupted = serde_json::to_value(&future_change).unwrap();
        corrupted["receipts"][3]["changed"] = serde_json::json!([page]);
        assert!(serde_json::from_value::<ReportDocument>(corrupted).is_err());
    }

    #[test]
    fn frozen_artifacts_use_bounded_authenticated_base64_wire_payloads() {
        let artifact = FrozenReportArtifact::new("image/svg+xml", b"<svg/>".to_vec()).unwrap();
        let encoded = serde_json::to_value(&artifact).unwrap();
        assert!(encoded["payload_base64"].is_string());
        assert!(encoded.get("payload").is_none());
        let decoded: FrozenReportArtifact = serde_json::from_value(encoded.clone()).unwrap();
        assert_eq!(decoded, artifact);

        let mut tampered = encoded;
        tampered["payload_base64"] = serde_json::json!(BASE64_STANDARD.encode(b"<svg>x</svg>"));
        assert!(serde_json::from_value::<FrozenReportArtifact>(tampered).is_err());

        let oversized = "A".repeat(MAX_FROZEN_ARTIFACT_BASE64_BYTES + 1);
        let value = serde_json::json!({
            "media_type": "application/octet-stream",
            "payload_base64": oversized,
            "content_digest": digest(1),
        });
        assert!(serde_json::from_value::<FrozenReportArtifact>(value).is_err());
    }

    #[test]
    fn aggregate_frozen_payload_capacity_uses_checked_addition() {
        assert!(
            validate_aggregate_frozen_payload_bytes([
                MAX_FROZEN_ARTIFACT_BYTES_PER_DOCUMENT / 2,
                MAX_FROZEN_ARTIFACT_BYTES_PER_DOCUMENT / 2,
            ])
            .is_ok()
        );
        assert!(matches!(
            validate_aggregate_frozen_payload_bytes([MAX_FROZEN_ARTIFACT_BYTES_PER_DOCUMENT, 1,]),
            Err(ReportError::CapacityExceeded(_))
        ));
        assert!(matches!(
            validate_aggregate_frozen_payload_bytes([usize::MAX, 1]),
            Err(ReportError::CapacityExceeded(_))
        ));
    }

    #[test]
    fn frozen_plot_is_publishable_without_inventory_and_artifact_changes_affect_audit() {
        let (mut document, _, section_id) = document_with_section();
        let block_receipt = document
            .transact(
                document.revision(),
                vec![ReportEdit::AddBlock {
                    section_id,
                    kind: all_block_kinds().remove(0),
                }],
                77,
            )
            .unwrap();
        let block_id = match block_receipt.created[0] {
            ReportEntityRef::Block(id) => id,
            _ => unreachable!(),
        };
        let first = document
            .audit_references(&ReportReferenceInventory::default())
            .unwrap();
        assert!(first.is_current_for_sign_off());
        assert_eq!(
            first.entries[0].frozen_artifact_media_type.as_deref(),
            Some("image/svg+xml")
        );
        assert!(first.entries[0].frozen_artifact_digest.is_some());

        let block = document.block(block_id).unwrap();
        let block_revision = block.revision();
        let mut changed_kind = block.kind().clone();
        let ReportBlockKind::PlotFigure(figure) = &mut changed_kind else {
            unreachable!()
        };
        let ReportReferenceMode::Frozen { artifact, .. } = &mut figure.reference else {
            unreachable!()
        };
        *artifact = FrozenReportArtifact::new(
            "image/svg+xml",
            b"<svg xmlns=\"http://www.w3.org/2000/svg\"><path/></svg>".to_vec(),
        )
        .unwrap();
        let mut changed = document.clone();
        changed
            .transact(
                changed.revision(),
                vec![ReportEdit::ReplaceBlock {
                    block_id,
                    expected_block_revision: block_revision,
                    kind: changed_kind,
                }],
                78,
            )
            .unwrap();
        let second = changed
            .audit_references(&ReportReferenceInventory::default())
            .unwrap();
        assert_ne!(
            first.entries[0].frozen_artifact_digest,
            second.entries[0].frozen_artifact_digest
        );
        assert_ne!(first.audit_digest, second.audit_digest);
    }

    #[test]
    fn version_one_initial_snapshots_migrate_without_fabricating_history() {
        let page = ReportPage {
            id: ReportPageId::new(),
            created_at_document_revision: ObjectRevision::INITIAL,
            revision: ObjectRevision::INITIAL,
            title: "Imported page".to_owned(),
            update_policy: ReportPageUpdatePolicy::RefreshLinkedAutomatically,
            sections: vec![ReportSection {
                id: ReportSectionId::new(),
                created_at_document_revision: ObjectRevision::INITIAL,
                revision: ObjectRevision::INITIAL,
                title: "Imported section".to_owned(),
                blocks: vec![ReportBlock {
                    id: ReportBlockId::new(),
                    created_at_document_revision: ObjectRevision::INITIAL,
                    revision: ObjectRevision::INITIAL,
                    kind: all_block_kinds().remove(5),
                }],
            }],
        };
        let legacy = ReportDocument {
            schema_version: 1,
            id: ResultDocumentId::new(),
            revision: ObjectRevision::INITIAL,
            title: "Legacy report".to_owned(),
            template: ReportTemplate::ReleaseVerification42,
            pages: vec![page],
            receipts: Vec::new(),
            tombstones: Vec::new(),
            legacy_origin_entities: Vec::new(),
            revision_history: ReportRevisionHistory {
                origin: ReportRevisionHistoryOrigin::Native,
                records: Vec::new(),
            },
        };
        let mut legacy_value = serde_json::to_value(&legacy).unwrap();
        legacy_value
            .as_object_mut()
            .unwrap()
            .remove("revision_history");
        let mut migrated: ReportDocument = serde_json::from_value(legacy_value).unwrap();
        assert_eq!(migrated.schema_version(), ReportDocument::SCHEMA_VERSION);
        assert_eq!(migrated.legacy_origin_entities.len(), 3);
        migrated
            .transact(
                migrated.revision(),
                vec![ReportEdit::SetDocumentTitle {
                    title: "Migrated report".to_owned(),
                }],
                80,
            )
            .unwrap();

        let mut unsafe_legacy = serde_json::to_value(&legacy).unwrap();
        unsafe_legacy
            .as_object_mut()
            .unwrap()
            .remove("revision_history");
        unsafe_legacy["revision"] = serde_json::json!(2);
        assert!(serde_json::from_value::<ReportDocument>(unsafe_legacy).is_err());
    }

    #[test]
    fn revision_history_reconstructs_complete_source_at_every_native_revision() {
        let mut document = ReportDocument::new("Verification report").unwrap();
        let initial_id = document.revision_history().records()[0].revision_identity();
        document
            .transact_with_context(
                document.revision(),
                vec![ReportEdit::AddPage {
                    title: "Summary".to_owned(),
                }],
                101,
                "james@example.com",
                "Add governed report summary page",
            )
            .unwrap();
        document
            .transact_with_context(
                document.revision(),
                vec![ReportEdit::SetDocumentTitle {
                    title: "Release verification".to_owned(),
                }],
                102,
                "james@example.com",
                "Name the release report",
            )
            .unwrap();

        let records = document.revision_history().records();
        assert_eq!(
            document.revision_history().origin(),
            ReportRevisionHistoryOrigin::Native
        );
        assert_eq!(records.len(), 3);
        assert_eq!(records[0].prior_revision_identity(), None);
        assert_eq!(records[1].prior_revision_identity(), Some(initial_id));
        assert_eq!(
            records[1].prior_record_digest(),
            Some(records[0].record_digest())
        );
        assert_eq!(
            records[2].prior_revision_identity(),
            Some(records[1].revision_identity())
        );
        assert_eq!(
            records[2].prior_record_digest(),
            Some(records[1].record_digest())
        );
        assert_eq!(records[1].actor(), "james@example.com");
        assert_eq!(
            records[1].revision_note(),
            "Add governed report summary page"
        );
        assert_eq!(records[1].timestamp_unix_ms(), 101);
        assert!(records[1].snapshot_serialized_bytes() > 0);
        assert_ne!(records[1].snapshot_digest(), records[2].snapshot_digest());

        let initial = document
            .reconstruct_revision(document.id(), ObjectRevision::INITIAL)
            .unwrap();
        assert_eq!(initial.title(), "Verification report");
        assert!(initial.pages().is_empty());
        assert!(initial.receipts().is_empty());
        assert_eq!(initial.revision_history().records().len(), 1);

        let revision_two = document
            .reconstruct_revision(document.id(), ObjectRevision::new(2).unwrap())
            .unwrap();
        assert_eq!(revision_two.title(), "Verification report");
        assert_eq!(revision_two.pages().len(), 1);
        assert_eq!(revision_two.receipts().len(), 1);
        assert_eq!(revision_two.revision_history().records().len(), 2);

        let wrong_document = ResultDocumentId::new();
        assert!(
            document
                .revision_record(wrong_document, ObjectRevision::INITIAL)
                .is_none()
        );
        assert!(matches!(
            document.reconstruct_revision(wrong_document, ObjectRevision::INITIAL),
            Err(ReportError::RevisionNotRetained { .. })
        ));

        let encoded = serde_json::to_vec(&document).unwrap();
        let restored: ReportDocument = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(restored, document);
        assert_eq!(
            restored.revision_history().records()[2].record_digest(),
            records[2].record_digest()
        );
    }

    #[test]
    fn revision_history_deserialization_fails_closed_on_snapshot_or_chain_tampering() {
        let mut document = ReportDocument::new("Audit report").unwrap();
        document
            .transact(
                document.revision(),
                vec![ReportEdit::AddPage {
                    title: "Evidence".to_owned(),
                }],
                200,
            )
            .unwrap();
        let value = serde_json::to_value(&document).unwrap();

        let mut changed_snapshot = value.clone();
        changed_snapshot["revision_history"]["records"][1]["snapshot"]["title"] =
            serde_json::json!("Tampered report");
        assert!(serde_json::from_value::<ReportDocument>(changed_snapshot).is_err());

        let mut changed_actor = value.clone();
        changed_actor["revision_history"]["records"][1]["actor"] = serde_json::json!("other-actor");
        assert!(serde_json::from_value::<ReportDocument>(changed_actor).is_err());

        let mut falsified_origin = value.clone();
        falsified_origin["revision_history"]["origin"] =
            serde_json::json!("imported-schema-one-baseline");
        assert!(serde_json::from_value::<ReportDocument>(falsified_origin).is_err());

        let mut broken_chain = value.clone();
        broken_chain["revision_history"]["records"][1]["prior_revision_identity"] =
            serde_json::json!(ReportRevisionId::new());
        assert!(serde_json::from_value::<ReportDocument>(broken_chain).is_err());

        let mut false_size = value.clone();
        false_size["revision_history"]["records"][1]["snapshot_serialized_bytes"] =
            serde_json::json!(1);
        assert!(serde_json::from_value::<ReportDocument>(false_size).is_err());

        let mut unknown_current_field = value.clone();
        unknown_current_field["future_history_contract"] = serde_json::json!({"version": 4});
        assert!(serde_json::from_value::<ReportDocument>(unknown_current_field).is_err());

        let mut missing_history = value;
        missing_history
            .as_object_mut()
            .unwrap()
            .remove("revision_history");
        assert!(serde_json::from_value::<ReportDocument>(missing_history).is_err());

        let mut downgraded_with_null_history = serde_json::to_value(&document).unwrap();
        downgraded_with_null_history["schema_version"] = serde_json::json!(2);
        downgraded_with_null_history["revision_history"] = serde_json::Value::Null;
        assert!(serde_json::from_value::<ReportDocument>(downgraded_with_null_history).is_err());
    }

    #[test]
    fn schema_two_migration_retains_an_explicit_current_source_baseline() {
        let (document, _, _) = document_with_section();
        assert_eq!(document.revision().get(), 3);
        let expected_pages = document.pages().to_vec();
        let expected_receipts = document.receipts().to_vec();
        let mut schema_two = serde_json::to_value(&document).unwrap();
        schema_two["schema_version"] = serde_json::json!(2);
        schema_two
            .as_object_mut()
            .unwrap()
            .remove("revision_history");

        let first_migration: ReportDocument = serde_json::from_value(schema_two.clone()).unwrap();
        let second_migration: ReportDocument = serde_json::from_value(schema_two).unwrap();
        assert_eq!(first_migration, second_migration);
        assert_eq!(
            serde_json::to_vec(&first_migration).unwrap(),
            serde_json::to_vec(&second_migration).unwrap()
        );
        let mut migrated = first_migration;
        assert_eq!(migrated.schema_version(), ReportDocument::SCHEMA_VERSION);
        assert_eq!(migrated.pages(), expected_pages);
        assert_eq!(migrated.receipts(), expected_receipts);
        assert_eq!(
            migrated.revision_history().origin(),
            ReportRevisionHistoryOrigin::ImportedSchemaTwoBaseline
        );
        assert_eq!(migrated.revision_history().records().len(), 1);
        assert_eq!(
            migrated.revision_history().records()[0].revision(),
            migrated.revision()
        );
        assert_eq!(
            migrated
                .reconstruct_revision(migrated.id(), migrated.revision())
                .unwrap()
                .pages(),
            expected_pages
        );

        migrated
            .transact_with_context(
                migrated.revision(),
                vec![ReportEdit::SetDocumentTitle {
                    title: "Migrated and edited".to_owned(),
                }],
                300,
                "migration-reviewer",
                "Continue editing from imported baseline",
            )
            .unwrap();
        assert_eq!(migrated.revision_history().records().len(), 2);
        assert_eq!(
            migrated.revision_history().records()[1].prior_revision_identity(),
            Some(migrated.revision_history().records()[0].revision_identity())
        );
    }

    #[test]
    fn revision_history_snapshot_capacity_uses_checked_aggregate_bytes() {
        assert!(
            validate_revision_history_snapshot_capacity([
                MAX_REPORT_REVISION_HISTORY_SNAPSHOT_BYTES / 2,
                MAX_REPORT_REVISION_HISTORY_SNAPSHOT_BYTES / 2,
            ])
            .is_ok()
        );
        assert!(matches!(
            validate_revision_history_snapshot_capacity([
                MAX_REPORT_REVISION_HISTORY_SNAPSHOT_BYTES,
                1,
            ]),
            Err(ReportError::CapacityExceeded(
                "report source revision snapshot bytes"
            ))
        ));
        assert!(matches!(
            validate_revision_history_snapshot_capacity([u64::MAX, 1]),
            Err(ReportError::CapacityExceeded(
                "report source revision snapshot bytes"
            ))
        ));
    }

    #[test]
    fn streaming_snapshot_digest_matches_canonical_json_bytes() {
        let document = ReportDocument::new("Streaming digest").unwrap();
        let snapshot = document.current_snapshot();
        let canonical = serde_json::to_vec(&snapshot).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(b"rspice-report-source-snapshot-v1\0");
        hasher.update(&canonical);
        let (digest, serialized_bytes) = report_snapshot_digest_and_size(&snapshot).unwrap();
        assert_eq!(serialized_bytes, canonical.len() as u64);
        assert_eq!(digest, ContentDigest::from_bytes(hasher.finalize().into()));
    }

    #[test]
    fn revision_history_sequence_deserializer_enforces_record_limit() {
        let document = ReportDocument::new("Bounded history").unwrap();
        let mut value = serde_json::to_value(&document).unwrap();
        let record = value["revision_history"]["records"][0].clone();
        value["revision_history"]["records"] =
            serde_json::Value::Array(vec![record; MAX_REPORT_REVISION_HISTORY_RECORDS + 1]);
        assert!(serde_json::from_value::<ReportDocument>(value).is_err());
    }

    #[test]
    fn revision_context_validation_fails_atomically() {
        let mut document = ReportDocument::new("Atomic report").unwrap();
        let before = document.clone();
        let error = document
            .transact_with_context(
                document.revision(),
                vec![ReportEdit::SetDocumentTitle {
                    title: "Must not commit".to_owned(),
                }],
                400,
                " ",
                "Invalid actor must reject the transaction",
            )
            .unwrap_err();
        assert!(matches!(
            error,
            ReportError::InvalidValue {
                field: "report-revision.actor",
                ..
            }
        ));
        assert_eq!(document, before);
    }

    #[test]
    fn reference_audit_digest_is_deterministic_for_identical_inputs() {
        let (document, _, _) = document_with_section();
        let inventory = ReportReferenceInventory::default();
        let first = document.audit_references(&inventory).unwrap();
        let second = document.audit_references(&inventory).unwrap();
        assert_eq!(first.audit_digest, second.audit_digest);
        assert!(first.is_current_for_sign_off());
    }
}
