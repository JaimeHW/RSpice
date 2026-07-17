//! Canonical, UI-independent report-composer domain.
//!
//! A report is a versioned arrangement of pages, sections, and typed content
//! blocks. References retain the exact source revision, content digest, and
//! dataset bindings used to compose a revision. Linked references can be
//! audited or refreshed; frozen references remain self-contained and never
//! silently follow mutable external state.

use std::collections::{HashMap, HashSet};
use std::fmt;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FigureSizing {
    FitWidth,
    FitPage,
    Natural,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ReportTemplate {
    #[default]
    ReleaseVerification42,
    DesignReview,
    ModelQualification,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ReportPageUpdatePolicy {
    #[default]
    RefreshLinkedAutomatically,
    FreezeSelectedRevision,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlotFigureBlock {
    pub caption: String,
    pub alternative_text: String,
    pub sizing: FigureSizing,
    pub reference: ReportReferenceMode,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "value")]
pub enum TableCell {
    Empty,
    Text(String),
    Number { value: f64, unit: Option<String> },
    Integer(i64),
    Boolean(bool),
}

impl TableCell {
    fn validate(&self) -> Result<(), ReportError> {
        match self {
            Self::Text(value) => validate_text("table.cell.text", value, 16_384, true),
            Self::Number { value, unit } => {
                if !value.is_finite() {
                    return Err(ReportError::InvalidValue {
                        field: "table.cell.number",
                        message: "numeric table values must be finite".to_owned(),
                    });
                }
                if let Some(unit) = unit {
                    validate_label("table.cell.unit", unit, 64)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableColumn {
    pub key: String,
    pub heading: String,
    pub unit: Option<String>,
}

impl TableColumn {
    fn validate(&self) -> Result<(), ReportError> {
        validate_token("table.column.key", &self.key, 128)?;
        validate_label("table.column.heading", &self.heading, 256)?;
        if let Some(unit) = &self.unit {
            validate_label("table.column.unit", unit, 64)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataTableBlock {
    pub title: String,
    pub columns: Vec<TableColumn>,
    pub rows: Vec<Vec<TableCell>>,
    pub reference: ReportReferenceMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatasheetField {
    pub key: String,
    pub label: String,
    pub value: String,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatasheetBlock {
    pub title: String,
    pub fields: Vec<DatasheetField>,
    pub reference: ReportReferenceMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RequirementDisposition {
    NotEvaluated,
    Passed,
    Failed,
    Waived,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequirementEntry {
    pub requirement_id: String,
    pub statement: String,
    pub disposition: RequirementDisposition,
    pub evidence_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequirementsBlock {
    pub title: String,
    pub entries: Vec<RequirementEntry>,
    pub reference: ReportReferenceMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpecificationDisposition {
    NotEvaluated,
    InSpecification,
    OutOfSpecification,
    Informational,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecificationEntry {
    pub expression: String,
    pub limit: String,
    pub measured: Option<String>,
    pub disposition: SpecificationDisposition,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecificationsBlock {
    pub title: String,
    pub entries: Vec<SpecificationEntry>,
    pub reference: ReportReferenceMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProseStyle {
    Body,
    ExecutiveSummary,
    Method,
    Conclusion,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProseBlock {
    pub style: ProseStyle,
    pub markdown: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReviewNoteStatus {
    Open,
    Addressed,
    Accepted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewNoteBlock {
    pub author: String,
    pub status: ReviewNoteStatus,
    pub message: String,
    pub created_at_unix_ms: u64,
    pub resolved_at_unix_ms: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceBlock {
    pub title: String,
    pub summary: String,
    pub reference: ReportReferenceMode,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "block-type", content = "content")]
pub enum ReportBlockKind {
    PlotFigure(PlotFigureBlock),
    DataTable(DataTableBlock),
    Datasheet(DatasheetBlock),
    Requirements(RequirementsBlock),
    Specifications(SpecificationsBlock),
    Prose(ProseBlock),
    ReviewNote(ReviewNoteBlock),
    Evidence(EvidenceBlock),
}

impl ReportBlockKind {
    #[must_use]
    pub const fn reference(&self) -> Option<&ReportReferenceMode> {
        match self {
            Self::PlotFigure(block) => Some(&block.reference),
            Self::DataTable(block) => Some(&block.reference),
            Self::Datasheet(block) => Some(&block.reference),
            Self::Requirements(block) => Some(&block.reference),
            Self::Specifications(block) => Some(&block.reference),
            Self::Evidence(block) => Some(&block.reference),
            Self::Prose(_) | Self::ReviewNote(_) => None,
        }
    }

    fn set_reference(&mut self, reference: ReportReferenceMode) -> Result<(), ReportError> {
        match self {
            Self::PlotFigure(block) => block.reference = reference,
            Self::DataTable(block) => block.reference = reference,
            Self::Datasheet(block) => block.reference = reference,
            Self::Requirements(block) => block.reference = reference,
            Self::Specifications(block) => block.reference = reference,
            Self::Evidence(block) => block.reference = reference,
            Self::Prose(_) | Self::ReviewNote(_) => {
                return Err(ReportError::BlockHasNoExternalReference);
            }
        }
        self.validate()
    }

    fn validate(&self) -> Result<(), ReportError> {
        match self {
            Self::PlotFigure(block) => {
                validate_label("plot-figure.caption", &block.caption, 2_048)?;
                validate_text(
                    "plot-figure.alternative-text",
                    &block.alternative_text,
                    8_192,
                    false,
                )?;
                block.reference.validate()?;
                if !matches!(
                    block.reference.snapshot().source,
                    ReportSourceId::VisualizationDocument { .. }
                ) {
                    return Err(ReportError::InvalidReferenceKind {
                        block: "plot-figure",
                        expected: "visualization-document",
                    });
                }
            }
            Self::DataTable(block) => validate_data_table(block)?,
            Self::Datasheet(block) => validate_datasheet(block)?,
            Self::Requirements(block) => validate_requirements(block)?,
            Self::Specifications(block) => validate_specifications(block)?,
            Self::Prose(block) => {
                validate_text("prose.markdown", &block.markdown, MAX_TEXT_BYTES, false)?;
            }
            Self::ReviewNote(block) => validate_review_note(block)?,
            Self::Evidence(block) => {
                validate_label("evidence.title", &block.title, 512)?;
                validate_text("evidence.summary", &block.summary, 65_536, false)?;
                block.reference.validate()?;
                if !matches!(
                    block.reference.snapshot().source,
                    ReportSourceId::VerificationEvidence { .. }
                ) {
                    return Err(ReportError::InvalidReferenceKind {
                        block: "evidence",
                        expected: "verification-evidence",
                    });
                }
            }
        }
        Ok(())
    }
}

fn validate_data_table(block: &DataTableBlock) -> Result<(), ReportError> {
    validate_label("data-table.title", &block.title, 512)?;
    block.reference.validate()?;
    if block.columns.is_empty() || block.columns.len() > MAX_TABLE_COLUMNS {
        return Err(ReportError::InvalidValue {
            field: "data-table.columns",
            message: format!("a table requires 1 to {MAX_TABLE_COLUMNS} columns"),
        });
    }
    if block.rows.len() > MAX_TABLE_ROWS
        || block
            .columns
            .len()
            .checked_mul(block.rows.len())
            .is_none_or(|cells| cells > MAX_TABLE_CELLS)
    {
        return Err(ReportError::InvalidValue {
            field: "data-table.rows",
            message: format!(
                "a table may contain at most {MAX_TABLE_ROWS} rows and {MAX_TABLE_CELLS} cells"
            ),
        });
    }
    let mut keys = HashSet::with_capacity(block.columns.len());
    for column in &block.columns {
        column.validate()?;
        if !keys.insert(column.key.as_str()) {
            return Err(ReportError::DuplicateKey(column.key.clone()));
        }
    }
    for row in &block.rows {
        if row.len() != block.columns.len() {
            return Err(ReportError::InvalidValue {
                field: "data-table.rows",
                message: "every row must contain exactly one cell per declared column".to_owned(),
            });
        }
        for cell in row {
            cell.validate()?;
        }
    }
    Ok(())
}

fn validate_datasheet(block: &DatasheetBlock) -> Result<(), ReportError> {
    validate_label("datasheet.title", &block.title, 512)?;
    block.reference.validate()?;
    if block.fields.is_empty() || block.fields.len() > MAX_STRUCTURED_ENTRIES {
        return Err(ReportError::InvalidValue {
            field: "datasheet.fields",
            message: format!("a datasheet requires 1 to {MAX_STRUCTURED_ENTRIES} fields"),
        });
    }
    let mut keys = HashSet::with_capacity(block.fields.len());
    for field in &block.fields {
        validate_token("datasheet.field.key", &field.key, 128)?;
        validate_label("datasheet.field.label", &field.label, 256)?;
        validate_text("datasheet.field.value", &field.value, 16_384, true)?;
        if let Some(unit) = &field.unit {
            validate_label("datasheet.field.unit", unit, 64)?;
        }
        if !keys.insert(field.key.as_str()) {
            return Err(ReportError::DuplicateKey(field.key.clone()));
        }
    }
    Ok(())
}

fn validate_requirements(block: &RequirementsBlock) -> Result<(), ReportError> {
    validate_label("requirements.title", &block.title, 512)?;
    block.reference.validate()?;
    if block.entries.is_empty() || block.entries.len() > MAX_STRUCTURED_ENTRIES {
        return Err(ReportError::InvalidValue {
            field: "requirements.entries",
            message: format!("a requirements block requires 1 to {MAX_STRUCTURED_ENTRIES} entries"),
        });
    }
    let mut identities = HashSet::with_capacity(block.entries.len());
    for entry in &block.entries {
        validate_token("requirement.id", &entry.requirement_id, 256)?;
        validate_text("requirement.statement", &entry.statement, 65_536, false)?;
        if let Some(label) = &entry.evidence_label {
            validate_label("requirement.evidence-label", label, 512)?;
        }
        if !identities.insert(entry.requirement_id.as_str()) {
            return Err(ReportError::DuplicateKey(entry.requirement_id.clone()));
        }
    }
    Ok(())
}

fn validate_specifications(block: &SpecificationsBlock) -> Result<(), ReportError> {
    validate_label("specifications.title", &block.title, 512)?;
    block.reference.validate()?;
    if block.entries.is_empty() || block.entries.len() > MAX_STRUCTURED_ENTRIES {
        return Err(ReportError::InvalidValue {
            field: "specifications.entries",
            message: format!(
                "a specifications block requires 1 to {MAX_STRUCTURED_ENTRIES} entries"
            ),
        });
    }
    let mut expressions = HashSet::with_capacity(block.entries.len());
    for entry in &block.entries {
        validate_text("specification.expression", &entry.expression, 4_096, true)?;
        validate_text("specification.limit", &entry.limit, 4_096, true)?;
        if let Some(measured) = &entry.measured {
            validate_text("specification.measured", measured, 4_096, true)?;
        }
        if !expressions.insert(entry.expression.as_str()) {
            return Err(ReportError::DuplicateKey(entry.expression.clone()));
        }
    }
    Ok(())
}

fn validate_review_note(block: &ReviewNoteBlock) -> Result<(), ReportError> {
    validate_label("review-note.author", &block.author, 256)?;
    validate_text("review-note.message", &block.message, 65_536, false)?;
    match (block.status, block.resolved_at_unix_ms) {
        (ReviewNoteStatus::Open, None) => {}
        (ReviewNoteStatus::Addressed | ReviewNoteStatus::Accepted, Some(resolved))
            if resolved >= block.created_at_unix_ms => {}
        (ReviewNoteStatus::Open, Some(_)) => {
            return Err(ReportError::InvalidValue {
                field: "review-note.resolved-at",
                message: "an open review note must not carry a resolution timestamp".to_owned(),
            });
        }
        _ => {
            return Err(ReportError::InvalidValue {
                field: "review-note.resolved-at",
                message:
                    "addressed and accepted notes require a resolution timestamp at or after creation"
                        .to_owned(),
            });
        }
    }
    Ok(())
}

fn validate_dataset_bindings(bindings: &[DatasetBinding]) -> Result<(), ReportError> {
    if bindings.len() > MAX_DATASET_BINDINGS {
        return Err(ReportError::InvalidValue {
            field: "reference.dataset-bindings",
            message: format!("at most {MAX_DATASET_BINDINGS} dataset bindings are permitted"),
        });
    }
    let mut identities = HashSet::with_capacity(bindings.len());
    for binding in bindings {
        if !identities.insert(binding.dataset_id) {
            return Err(ReportError::DuplicateDatasetBinding(binding.dataset_id));
        }
    }
    Ok(())
}

fn validate_label(
    field: &'static str,
    value: &str,
    maximum_bytes: usize,
) -> Result<(), ReportError> {
    if value.is_empty()
        || value != value.trim()
        || value.len() > maximum_bytes
        || value.chars().any(char::is_control)
    {
        return Err(ReportError::InvalidValue {
            field,
            message: format!(
                "value must be trimmed, non-blank, contain no control characters, and not exceed {maximum_bytes} bytes"
            ),
        });
    }
    Ok(())
}

fn validate_text(
    field: &'static str,
    value: &str,
    maximum_bytes: usize,
    single_line: bool,
) -> Result<(), ReportError> {
    if value.trim().is_empty() || value.len() > maximum_bytes {
        return Err(ReportError::InvalidValue {
            field,
            message: format!("text must be non-blank and not exceed {maximum_bytes} bytes"),
        });
    }
    let invalid_control = value
        .chars()
        .any(|character| character.is_control() && !matches!(character, '\n' | '\r' | '\t'));
    if invalid_control || (single_line && value.contains(['\n', '\r'])) {
        return Err(ReportError::InvalidValue {
            field,
            message: "text contains a control character not permitted in this field".to_owned(),
        });
    }
    Ok(())
}

fn validate_token(
    field: &'static str,
    value: &str,
    maximum_bytes: usize,
) -> Result<(), ReportError> {
    if value.is_empty()
        || value.len() > maximum_bytes
        || value != value.trim()
        || value.chars().any(|character| {
            character.is_control() || character.is_whitespace() || matches!(character, '/' | '\\')
        })
    {
        return Err(ReportError::InvalidValue {
            field,
            message: format!(
                "token must be trimmed, non-blank, path-neutral, contain no whitespace or control characters, and not exceed {maximum_bytes} bytes"
            ),
        });
    }
    Ok(())
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
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
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
}

impl<'de> Deserialize<'de> for ReportDocument {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ReportDocumentWire::deserialize(deserializer)?;
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
        };
        document.migrate().map_err(serde::de::Error::custom)?;
        document.validate().map_err(serde::de::Error::custom)?;
        Ok(document)
    }
}

impl ReportDocument {
    pub const SCHEMA_VERSION: u16 = 2;

    pub fn new(title: impl Into<String>) -> Result<Self, ReportError> {
        Self::new_with_template(title, ReportTemplate::ReleaseVerification42)
    }

    pub fn new_with_template(
        title: impl Into<String>,
        template: ReportTemplate,
    ) -> Result<Self, ReportError> {
        let document = Self {
            schema_version: Self::SCHEMA_VERSION,
            id: ResultDocumentId::new(),
            revision: ObjectRevision::INITIAL,
            title: title.into(),
            template,
            pages: Vec::new(),
            receipts: Vec::new(),
            tombstones: Vec::new(),
            legacy_origin_entities: Vec::new(),
        };
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
        let committed_revision = self.revision.next()?;
        let edit_count = u16::try_from(edits.len()).map_err(|_| ReportError::InvalidValue {
            field: "transaction.edits",
            message: "edit count cannot be represented in a mutation receipt".to_owned(),
        })?;
        let mut candidate = self.clone();
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
                Ok(())
            }
            version => Err(ReportError::UnsupportedSchemaVersion(version)),
        }
    }

    fn apply_edit(
        &mut self,
        edit: ReportEdit,
        committed_revision: ObjectRevision,
        created: &mut Vec<ReportEntityRef>,
        changed: &mut Vec<ReportEntityRef>,
        tombstoned: &mut Vec<ReportEntityRef>,
    ) -> Result<(), ReportError> {
        match edit {
            ReportEdit::SetDocumentTitle { title } => {
                validate_label("report-document.title", &title, 512)?;
                if self.title == title {
                    return Err(ReportError::NoChanges);
                }
                self.title = title;
            }
            ReportEdit::SetTemplate { template } => {
                if self.template == template {
                    return Err(ReportError::NoChanges);
                }
                self.template = template;
            }
            ReportEdit::AddPage { title } => {
                validate_label("report-page.title", &title, 512)?;
                if self.pages.len() >= MAX_PAGES {
                    return Err(ReportError::CapacityExceeded("report pages"));
                }
                let page = ReportPage {
                    id: ReportPageId::new(),
                    created_at_document_revision: committed_revision,
                    revision: ObjectRevision::INITIAL,
                    title,
                    update_policy: ReportPageUpdatePolicy::RefreshLinkedAutomatically,
                    sections: Vec::new(),
                };
                created.push(ReportEntityRef::Page(page.id));
                self.pages.push(page);
            }
            ReportEdit::UpdatePageTitle {
                page_id,
                expected_page_revision,
                title,
            } => {
                validate_label("report-page.title", &title, 512)?;
                let page = self.page_mut(page_id)?;
                require_entity_revision(
                    ReportEntityRef::Page(page_id),
                    expected_page_revision,
                    page.revision,
                )?;
                if page.title == title {
                    return Err(ReportError::NoChanges);
                }
                page.title = title;
                page.revision = page.revision.next()?;
                changed.push(ReportEntityRef::Page(page_id));
            }
            ReportEdit::SetPageUpdatePolicy {
                page_id,
                expected_page_revision,
                update_policy,
            } => {
                let page = self.page_mut(page_id)?;
                require_entity_revision(
                    ReportEntityRef::Page(page_id),
                    expected_page_revision,
                    page.revision,
                )?;
                if page.update_policy == update_policy {
                    return Err(ReportError::NoChanges);
                }
                page.update_policy = update_policy;
                page.revision = page.revision.next()?;
                changed.push(ReportEntityRef::Page(page_id));
            }
            ReportEdit::MovePage {
                page_id,
                expected_page_revision,
                before,
            } => self.move_page(page_id, expected_page_revision, before, changed)?,
            ReportEdit::AddSection { page_id, title } => {
                validate_label("report-section.title", &title, 512)?;
                let page = self.page_mut(page_id)?;
                if page.sections.len() >= MAX_SECTIONS_PER_PAGE {
                    return Err(ReportError::CapacityExceeded("sections per report page"));
                }
                let section = ReportSection {
                    id: ReportSectionId::new(),
                    created_at_document_revision: committed_revision,
                    revision: ObjectRevision::INITIAL,
                    title,
                    blocks: Vec::new(),
                };
                created.push(ReportEntityRef::Section(section.id));
                page.sections.push(section);
                page.revision = page.revision.next()?;
                changed.push(ReportEntityRef::Page(page_id));
            }
            ReportEdit::UpdateSectionTitle {
                section_id,
                expected_section_revision,
                title,
            } => {
                validate_label("report-section.title", &title, 512)?;
                let (_, _, section) = self.section_mut(section_id)?;
                require_entity_revision(
                    ReportEntityRef::Section(section_id),
                    expected_section_revision,
                    section.revision,
                )?;
                if section.title == title {
                    return Err(ReportError::NoChanges);
                }
                section.title = title;
                section.revision = section.revision.next()?;
                changed.push(ReportEntityRef::Section(section_id));
            }
            ReportEdit::MoveSection {
                section_id,
                expected_section_revision,
                target_page_id,
                before,
            } => self.move_section(
                section_id,
                expected_section_revision,
                target_page_id,
                before,
                changed,
            )?,
            ReportEdit::AddBlock { section_id, kind } => {
                kind.validate()?;
                if self.block_count() >= MAX_BLOCKS_TOTAL {
                    return Err(ReportError::CapacityExceeded("report content blocks"));
                }
                let (_, _, section) = self.section_mut(section_id)?;
                if section.blocks.len() >= MAX_BLOCKS_PER_SECTION {
                    return Err(ReportError::CapacityExceeded(
                        "content blocks per report section",
                    ));
                }
                let block = ReportBlock {
                    id: ReportBlockId::new(),
                    created_at_document_revision: committed_revision,
                    revision: ObjectRevision::INITIAL,
                    kind,
                };
                created.push(ReportEntityRef::Block(block.id));
                section.blocks.push(block);
                section.revision = section.revision.next()?;
                changed.push(ReportEntityRef::Section(section_id));
            }
            ReportEdit::ReplaceBlock {
                block_id,
                expected_block_revision,
                kind,
            } => {
                kind.validate()?;
                let (_, _, _, _, block) = self.block_mut(block_id)?;
                require_entity_revision(
                    ReportEntityRef::Block(block_id),
                    expected_block_revision,
                    block.revision,
                )?;
                if block.kind == kind {
                    return Err(ReportError::NoChanges);
                }
                block.kind = kind;
                block.revision = block.revision.next()?;
                changed.push(ReportEntityRef::Block(block_id));
            }
            ReportEdit::UpdateBlockReference {
                block_id,
                expected_block_revision,
                reference,
            } => {
                reference.validate()?;
                let (_, _, _, _, block) = self.block_mut(block_id)?;
                require_entity_revision(
                    ReportEntityRef::Block(block_id),
                    expected_block_revision,
                    block.revision,
                )?;
                if block.kind.reference() == Some(&reference) {
                    return Err(ReportError::NoChanges);
                }
                block.kind.set_reference(reference)?;
                block.revision = block.revision.next()?;
                changed.push(ReportEntityRef::Block(block_id));
            }
            ReportEdit::MoveBlock {
                block_id,
                expected_block_revision,
                target_section_id,
                before,
            } => self.move_block(
                block_id,
                expected_block_revision,
                target_section_id,
                before,
                changed,
            )?,
            ReportEdit::Remove {
                entity,
                expected_entity_revision,
            } => self.remove_entity(
                entity,
                expected_entity_revision,
                committed_revision,
                changed,
                tombstoned,
            )?,
        }
        Ok(())
    }

    fn move_page(
        &mut self,
        page_id: ReportPageId,
        expected_revision: ObjectRevision,
        before: Option<ReportPageId>,
        changed: &mut Vec<ReportEntityRef>,
    ) -> Result<(), ReportError> {
        let from = self
            .pages
            .iter()
            .position(|page| page.id == page_id)
            .ok_or(ReportError::EntityNotFound(ReportEntityRef::Page(page_id)))?;
        require_entity_revision(
            ReportEntityRef::Page(page_id),
            expected_revision,
            self.pages[from].revision,
        )?;
        if before == Some(page_id) {
            return Err(ReportError::InvalidMoveTarget);
        }
        let mut page = self.pages.remove(from);
        let target = match before {
            Some(target_id) => self
                .pages
                .iter()
                .position(|candidate| candidate.id == target_id)
                .ok_or(ReportError::EntityNotFound(ReportEntityRef::Page(
                    target_id,
                )))?,
            None => self.pages.len(),
        };
        if target == from {
            // Put the removed value back at its original position before
            // reporting a no-op so the cloned candidate remains internally
            // usable for diagnostic inspection.
            self.pages.insert(from, page);
            return Err(ReportError::NoChanges);
        }
        page.revision = page.revision.next()?;
        self.pages.insert(target, page);
        changed.push(ReportEntityRef::Page(page_id));
        Ok(())
    }

    fn move_section(
        &mut self,
        section_id: ReportSectionId,
        expected_revision: ObjectRevision,
        target_page_id: ReportPageId,
        before: Option<ReportSectionId>,
        changed: &mut Vec<ReportEntityRef>,
    ) -> Result<(), ReportError> {
        if before == Some(section_id) {
            return Err(ReportError::InvalidMoveTarget);
        }
        let (source_page_index, source_section_index) = self.section_position(section_id)?;
        require_entity_revision(
            ReportEntityRef::Section(section_id),
            expected_revision,
            self.pages[source_page_index].sections[source_section_index].revision,
        )?;
        let target_page_index = self
            .pages
            .iter()
            .position(|page| page.id == target_page_id)
            .ok_or(ReportError::EntityNotFound(ReportEntityRef::Page(
                target_page_id,
            )))?;
        if source_page_index != target_page_index
            && self.pages[target_page_index].sections.len() >= MAX_SECTIONS_PER_PAGE
        {
            return Err(ReportError::CapacityExceeded("sections per report page"));
        }
        let target_index_before_removal = match before {
            Some(target_id) => {
                let (page_index, section_index) = self.section_position(target_id)?;
                if page_index != target_page_index {
                    return Err(ReportError::InvalidMoveTarget);
                }
                section_index
            }
            None => self.pages[target_page_index].sections.len(),
        };
        let no_change = source_page_index == target_page_index
            && (target_index_before_removal == source_section_index
                || (before.is_some() && target_index_before_removal == source_section_index + 1)
                || (before.is_none()
                    && source_section_index + 1 == self.pages[source_page_index].sections.len()));
        if no_change {
            return Err(ReportError::NoChanges);
        }
        let mut section = self.pages[source_page_index]
            .sections
            .remove(source_section_index);
        let target_index = if source_page_index == target_page_index
            && target_index_before_removal > source_section_index
        {
            target_index_before_removal - 1
        } else {
            target_index_before_removal
        };
        section.revision = section.revision.next()?;
        self.pages[target_page_index]
            .sections
            .insert(target_index, section);
        self.pages[source_page_index].revision = self.pages[source_page_index].revision.next()?;
        if source_page_index != target_page_index {
            self.pages[target_page_index].revision =
                self.pages[target_page_index].revision.next()?;
        }
        changed.push(ReportEntityRef::Section(section_id));
        changed.push(ReportEntityRef::Page(self.pages[source_page_index].id));
        changed.push(ReportEntityRef::Page(self.pages[target_page_index].id));
        Ok(())
    }

    fn move_block(
        &mut self,
        block_id: ReportBlockId,
        expected_revision: ObjectRevision,
        target_section_id: ReportSectionId,
        before: Option<ReportBlockId>,
        changed: &mut Vec<ReportEntityRef>,
    ) -> Result<(), ReportError> {
        if before == Some(block_id) {
            return Err(ReportError::InvalidMoveTarget);
        }
        let (source_page, source_section, source_block) = self.block_position(block_id)?;
        require_entity_revision(
            ReportEntityRef::Block(block_id),
            expected_revision,
            self.pages[source_page].sections[source_section].blocks[source_block].revision,
        )?;
        let (target_page, target_section) = self.section_position(target_section_id)?;
        if (source_page, source_section) != (target_page, target_section)
            && self.pages[target_page].sections[target_section]
                .blocks
                .len()
                >= MAX_BLOCKS_PER_SECTION
        {
            return Err(ReportError::CapacityExceeded(
                "content blocks per report section",
            ));
        }
        let target_index_before_removal = match before {
            Some(target_id) => {
                let (page, section, index) = self.block_position(target_id)?;
                if (page, section) != (target_page, target_section) {
                    return Err(ReportError::InvalidMoveTarget);
                }
                index
            }
            None => self.pages[target_page].sections[target_section]
                .blocks
                .len(),
        };
        let same_container = (source_page, source_section) == (target_page, target_section);
        let no_change = same_container
            && (target_index_before_removal == source_block
                || (before.is_some() && target_index_before_removal == source_block + 1)
                || (before.is_none()
                    && source_block + 1
                        == self.pages[source_page].sections[source_section]
                            .blocks
                            .len()));
        if no_change {
            return Err(ReportError::NoChanges);
        }
        let mut block = self.pages[source_page].sections[source_section]
            .blocks
            .remove(source_block);
        let target_index = if same_container && target_index_before_removal > source_block {
            target_index_before_removal - 1
        } else {
            target_index_before_removal
        };
        block.revision = block.revision.next()?;
        self.pages[target_page].sections[target_section]
            .blocks
            .insert(target_index, block);
        self.pages[source_page].sections[source_section].revision = self.pages[source_page]
            .sections[source_section]
            .revision
            .next()?;
        if !same_container {
            self.pages[target_page].sections[target_section].revision = self.pages[target_page]
                .sections[target_section]
                .revision
                .next()?;
        }
        changed.push(ReportEntityRef::Block(block_id));
        changed.push(ReportEntityRef::Section(
            self.pages[source_page].sections[source_section].id,
        ));
        changed.push(ReportEntityRef::Section(
            self.pages[target_page].sections[target_section].id,
        ));
        Ok(())
    }

    fn remove_entity(
        &mut self,
        entity: ReportEntityRef,
        expected_revision: ObjectRevision,
        committed_revision: ObjectRevision,
        changed: &mut Vec<ReportEntityRef>,
        tombstoned: &mut Vec<ReportEntityRef>,
    ) -> Result<(), ReportError> {
        match entity {
            ReportEntityRef::Page(page_id) => {
                let index = self
                    .pages
                    .iter()
                    .position(|page| page.id == page_id)
                    .ok_or(ReportError::EntityNotFound(entity))?;
                require_entity_revision(entity, expected_revision, self.pages[index].revision)?;
                let page = self.pages.remove(index);
                for section in page.sections {
                    for block in section.blocks {
                        self.record_tombstone(
                            ReportEntityRef::Block(block.id),
                            block.created_at_document_revision,
                            block.revision,
                            committed_revision,
                            tombstoned,
                        );
                    }
                    self.record_tombstone(
                        ReportEntityRef::Section(section.id),
                        section.created_at_document_revision,
                        section.revision,
                        committed_revision,
                        tombstoned,
                    );
                }
                self.record_tombstone(
                    ReportEntityRef::Page(page.id),
                    page.created_at_document_revision,
                    page.revision,
                    committed_revision,
                    tombstoned,
                );
            }
            ReportEntityRef::Section(section_id) => {
                let (page_index, section_index) = self.section_position(section_id)?;
                require_entity_revision(
                    entity,
                    expected_revision,
                    self.pages[page_index].sections[section_index].revision,
                )?;
                let section = self.pages[page_index].sections.remove(section_index);
                for block in section.blocks {
                    self.record_tombstone(
                        ReportEntityRef::Block(block.id),
                        block.created_at_document_revision,
                        block.revision,
                        committed_revision,
                        tombstoned,
                    );
                }
                self.record_tombstone(
                    ReportEntityRef::Section(section.id),
                    section.created_at_document_revision,
                    section.revision,
                    committed_revision,
                    tombstoned,
                );
                self.pages[page_index].revision = self.pages[page_index].revision.next()?;
                changed.push(ReportEntityRef::Page(self.pages[page_index].id));
            }
            ReportEntityRef::Block(block_id) => {
                let (page_index, section_index, block_index) = self.block_position(block_id)?;
                require_entity_revision(
                    entity,
                    expected_revision,
                    self.pages[page_index].sections[section_index].blocks[block_index].revision,
                )?;
                let block = self.pages[page_index].sections[section_index]
                    .blocks
                    .remove(block_index);
                self.record_tombstone(
                    ReportEntityRef::Block(block.id),
                    block.created_at_document_revision,
                    block.revision,
                    committed_revision,
                    tombstoned,
                );
                self.pages[page_index].sections[section_index].revision = self.pages[page_index]
                    .sections[section_index]
                    .revision
                    .next()?;
                changed.push(ReportEntityRef::Section(
                    self.pages[page_index].sections[section_index].id,
                ));
            }
        }
        Ok(())
    }

    fn record_tombstone(
        &mut self,
        entity: ReportEntityRef,
        created_at_document_revision: ObjectRevision,
        last_entity_revision: ObjectRevision,
        removed_at_document_revision: ObjectRevision,
        receipt: &mut Vec<ReportEntityRef>,
    ) {
        self.tombstones.push(ReportTombstone {
            entity,
            created_at_document_revision,
            last_entity_revision,
            removed_at_document_revision,
        });
        receipt.push(entity);
    }

    fn page_mut(&mut self, page_id: ReportPageId) -> Result<&mut ReportPage, ReportError> {
        self.pages
            .iter_mut()
            .find(|page| page.id == page_id)
            .ok_or(ReportError::EntityNotFound(ReportEntityRef::Page(page_id)))
    }

    fn section_mut(
        &mut self,
        section_id: ReportSectionId,
    ) -> Result<(usize, usize, &mut ReportSection), ReportError> {
        let (page_index, section_index) = self.section_position(section_id)?;
        let section = &mut self.pages[page_index].sections[section_index];
        Ok((page_index, section_index, section))
    }

    fn block_mut(
        &mut self,
        block_id: ReportBlockId,
    ) -> Result<(usize, usize, usize, ReportSectionId, &mut ReportBlock), ReportError> {
        let (page_index, section_index, block_index) = self.block_position(block_id)?;
        let section_id = self.pages[page_index].sections[section_index].id;
        let block = &mut self.pages[page_index].sections[section_index].blocks[block_index];
        Ok((page_index, section_index, block_index, section_id, block))
    }

    fn section_position(&self, section_id: ReportSectionId) -> Result<(usize, usize), ReportError> {
        for (page_index, page) in self.pages.iter().enumerate() {
            if let Some(section_index) = page
                .sections
                .iter()
                .position(|section| section.id == section_id)
            {
                return Ok((page_index, section_index));
            }
        }
        Err(ReportError::EntityNotFound(ReportEntityRef::Section(
            section_id,
        )))
    }

    fn block_position(
        &self,
        block_id: ReportBlockId,
    ) -> Result<(usize, usize, usize), ReportError> {
        for (page_index, page) in self.pages.iter().enumerate() {
            for (section_index, section) in page.sections.iter().enumerate() {
                if let Some(block_index) =
                    section.blocks.iter().position(|block| block.id == block_id)
                {
                    return Ok((page_index, section_index, block_index));
                }
            }
        }
        Err(ReportError::EntityNotFound(ReportEntityRef::Block(
            block_id,
        )))
    }

    fn block_count(&self) -> usize {
        self.pages
            .iter()
            .flat_map(|page| page.sections.iter())
            .map(|section| section.blocks.len())
            .sum()
    }

    pub fn validate(&self) -> Result<(), ReportError> {
        if self.schema_version != Self::SCHEMA_VERSION {
            return Err(ReportError::UnsupportedSchemaVersion(self.schema_version));
        }
        validate_label("report-document.title", &self.title, 512)?;
        if self.pages.len() > MAX_PAGES {
            return Err(ReportError::CapacityExceeded("report pages"));
        }
        if self.block_count() > MAX_BLOCKS_TOTAL {
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
        for page in &self.pages {
            if !live.insert(ReportEntityRef::Page(page.id)) {
                return Err(ReportError::DuplicateIdentity(ReportEntityRef::Page(
                    page.id,
                )));
            }
            validate_label("report-page.title", &page.title, 512)?;
            self.validate_entity_creation(
                ReportEntityRef::Page(page.id),
                page.created_at_document_revision,
            )?;
            if page.sections.len() > MAX_SECTIONS_PER_PAGE {
                return Err(ReportError::CapacityExceeded("sections per report page"));
            }
            for section in &page.sections {
                if !live.insert(ReportEntityRef::Section(section.id)) {
                    return Err(ReportError::DuplicateIdentity(ReportEntityRef::Section(
                        section.id,
                    )));
                }
                validate_label("report-section.title", &section.title, 512)?;
                self.validate_entity_creation(
                    ReportEntityRef::Section(section.id),
                    section.created_at_document_revision,
                )?;
                if section.blocks.len() > MAX_BLOCKS_PER_SECTION {
                    return Err(ReportError::CapacityExceeded(
                        "content blocks per report section",
                    ));
                }
                for block in &section.blocks {
                    if !live.insert(ReportEntityRef::Block(block.id)) {
                        return Err(ReportError::DuplicateIdentity(ReportEntityRef::Block(
                            block.id,
                        )));
                    }
                    self.validate_entity_creation(
                        ReportEntityRef::Block(block.id),
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
        &self,
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

    fn validate_legacy_origins(&self, live: &HashSet<ReportEntityRef>) -> Result<(), ReportError> {
        let tombstoned: HashSet<_> = self
            .tombstones
            .iter()
            .map(|tombstone| tombstone.entity)
            .collect();
        let mut origins = HashSet::with_capacity(self.legacy_origin_entities.len());
        for entity in &self.legacy_origin_entities {
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

    fn validate_tombstones(&self, live: &HashSet<ReportEntityRef>) -> Result<(), ReportError> {
        let mut tombstoned = HashSet::with_capacity(self.tombstones.len());
        for tombstone in &self.tombstones {
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

    fn validate_receipts(&self, live: &HashSet<ReportEntityRef>) -> Result<(), ReportError> {
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
        // Entities present in a post-creation document must have exactly one
        // creation receipt. Version-one snapshots may contain initial entities
        // without receipts and are migrated only at document revision one.
        if self.revision != ObjectRevision::INITIAL {
            for entity in live.iter().chain(tombstones.keys()) {
                if !created_ids.contains(entity) && !legacy_origins.contains(entity) {
                    return Err(invalid_receipt());
                }
            }
        }
        Ok(())
    }

    fn entity_created_at(&self, entity: ReportEntityRef) -> Option<ObjectRevision> {
        match entity {
            ReportEntityRef::Page(id) => {
                self.page(id).map(|page| page.created_at_document_revision)
            }
            ReportEntityRef::Section(id) => self
                .section(id)
                .map(|section| section.created_at_document_revision),
            ReportEntityRef::Block(id) => self
                .block(id)
                .map(|block| block.created_at_document_revision),
        }
        .or_else(|| {
            self.tombstones
                .iter()
                .find(|tombstone| tombstone.entity == entity)
                .map(|tombstone| tombstone.created_at_document_revision)
        })
    }

    fn entity_removed_at(&self, entity: ReportEntityRef) -> Option<ObjectRevision> {
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

        let mut changed = document.clone();
        let (_, _, _, _, block) = changed.block_mut(block_id).unwrap();
        let ReportBlockKind::PlotFigure(figure) = &mut block.kind else {
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
        changed.validate().unwrap();
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
        };
        let mut migrated: ReportDocument =
            serde_json::from_value(serde_json::to_value(&legacy).unwrap()).unwrap();
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
        unsafe_legacy["revision"] = serde_json::json!(2);
        assert!(serde_json::from_value::<ReportDocument>(unsafe_legacy).is_err());
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
