//! Project-owned hardcopy source-set records and their shared validation.
//!
//! A source set is the persisted, digest-authenticated answer to "which exact
//! document revisions does this print job cover". `ProjectWorkspace` owns these
//! records directly, so they live here rather than beside the adapters that
//! resolve live documents: the adapters need the schematic symbol library and
//! the analysis viewers, and `state` must not reach up through those to
//! describe its own saved data.
//!
//! The document adapters in `workbench::hardcopy_adapters::sources` import these types
//! and the shared validation helpers back down from here.

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

use crate::product::{ContentDigest, ObjectRevision};
use crate::results::report_document::{ReportBlockId, ReportReferenceCurrentness};

use super::contract::{HardcopyDocumentId, HardcopyDocumentKind, HardcopyScope};

/// Maximum number of exact retained documents in one project-owned source
/// set. This is a persistence and resolution boundary, not a UI page limit.
pub const MAX_HARDCOPY_SOURCE_SET_MEMBERS: usize = 64;
pub const HARDCOPY_SOURCE_SET_SCHEMA_VERSION: u32 = 1;

pub(crate) const SOURCE_KEY_LIMIT: usize = 512;
pub(crate) const DISPLAY_NAME_LIMIT: usize = 256;

/// Stable source identity supplied by the owner of a schematic or symbol
/// document. The source adapter authenticates content but never fabricates an
/// application document identity or revision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HardcopySourceIdentity {
    pub source_key: String,
    pub document_id: HardcopyDocumentId,
    pub revision: ObjectRevision,
    pub display_name: String,
}

impl HardcopySourceIdentity {
    pub fn try_new(
        source_key: impl Into<String>,
        document_id: HardcopyDocumentId,
        revision: ObjectRevision,
        display_name: impl Into<String>,
    ) -> Result<Self, HardcopySourceError> {
        let source_key = source_key.into();
        let display_name = display_name.into();
        validate_label("source key", &source_key, SOURCE_KEY_LIMIT)?;
        validate_label("display name", &display_name, DISPLAY_NAME_LIMIT)?;
        Ok(Self {
            source_key,
            document_id,
            revision,
            display_name,
        })
    }
}

/// One exact, ordered member pinned into a project-owned hardcopy source set.
/// A set never follows "latest": all four authority fields must still match
/// when the aggregate is resolved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HardcopySourceSetMember {
    source_key: String,
    display_name: String,
    document_id: HardcopyDocumentId,
    revision: ObjectRevision,
    content_digest: ContentDigest,
    scope: HardcopyScope,
}

impl HardcopySourceSetMember {
    pub fn try_new(
        source_key: impl Into<String>,
        display_name: impl Into<String>,
        document_id: HardcopyDocumentId,
        revision: ObjectRevision,
        content_digest: ContentDigest,
        scope: HardcopyScope,
    ) -> Result<Self, HardcopySourceError> {
        let member = Self {
            source_key: source_key.into(),
            display_name: display_name.into(),
            document_id,
            revision,
            content_digest,
            scope,
        };
        member.validate()?;
        Ok(member)
    }


    fn validate(&self) -> Result<(), HardcopySourceError> {
        validate_label("source-set member key", &self.source_key, SOURCE_KEY_LIMIT)?;
        validate_label(
            "source-set member display name",
            &self.display_name,
            DISPLAY_NAME_LIMIT,
        )?;
        if matches!(
            self.scope,
            HardcopyScope::AllSheetsOrPanes | HardcopyScope::NamedPrintSet(_)
        ) {
            return Err(HardcopySourceError::InvalidSourceSet(
                "source-set members cannot recursively name aggregate scopes".to_owned(),
            ));
        }
        Ok(())
    }

    #[must_use]
    pub fn source_key(&self) -> &str {
        &self.source_key
    }

    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    #[must_use]
    pub const fn document_id(&self) -> HardcopyDocumentId {
        self.document_id
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub const fn content_digest(&self) -> ContentDigest {
        self.content_digest
    }

    #[must_use]
    pub const fn scope(&self) -> &HardcopyScope {
        &self.scope
    }
}

/// Versioned, digest-authenticated source-set definition intended for direct
/// ownership by `ProjectWorkspace`. Stable vector order is publication order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HardcopySourceSet {
    schema_version: u32,
    document_id: HardcopyDocumentId,
    revision: ObjectRevision,
    name: String,
    document_kind: HardcopyDocumentKind,
    scope: HardcopyScope,
    members: Vec<HardcopySourceSetMember>,
    definition_digest: ContentDigest,
}

impl HardcopySourceSet {
    pub fn try_new(
        document_id: HardcopyDocumentId,
        revision: ObjectRevision,
        name: impl Into<String>,
        document_kind: HardcopyDocumentKind,
        scope: HardcopyScope,
        members: Vec<HardcopySourceSetMember>,
    ) -> Result<Self, HardcopySourceError> {
        let mut source_set = Self {
            schema_version: HARDCOPY_SOURCE_SET_SCHEMA_VERSION,
            document_id,
            revision,
            name: name.into(),
            document_kind,
            scope,
            members,
            definition_digest: ContentDigest::from_bytes([0; 32]),
        };
        source_set.validate_shape()?;
        source_set.definition_digest = source_set.compute_definition_digest()?;
        Ok(source_set)
    }

    /// Revalidate a deserialized definition, including its exact digest.
    pub fn validate(&self) -> Result<(), HardcopySourceError> {
        self.validate_shape()?;
        let actual = self.compute_definition_digest()?;
        if actual != self.definition_digest {
            return Err(HardcopySourceError::SourceSetDigestMismatch {
                expected: self.definition_digest,
                actual,
            });
        }
        Ok(())
    }

    fn validate_shape(&self) -> Result<(), HardcopySourceError> {
        if self.schema_version != HARDCOPY_SOURCE_SET_SCHEMA_VERSION {
            return Err(HardcopySourceError::UnsupportedSourceSetSchema(
                self.schema_version,
            ));
        }
        validate_label("source-set name", &self.name, DISPLAY_NAME_LIMIT)?;
        if self.members.is_empty() || self.members.len() > MAX_HARDCOPY_SOURCE_SET_MEMBERS {
            return Err(HardcopySourceError::InvalidSourceSet(format!(
                "a source set must retain 1..={MAX_HARDCOPY_SOURCE_SET_MEMBERS} members"
            )));
        }
        match &self.scope {
            HardcopyScope::NamedPrintSet(name) if name == &self.name => {}
            HardcopyScope::AllSheetsOrPanes
                if self.document_kind != HardcopyDocumentKind::EngineeringDocument => {}
            HardcopyScope::NamedPrintSet(_) => {
                return Err(HardcopySourceError::InvalidSourceSet(
                    "named print-set scope must exactly match the source-set name".to_owned(),
                ));
            }
            _ => {
                return Err(HardcopySourceError::InvalidSourceSet(
                    "aggregate scope must be all-sheets-or-panes or the exact named print set"
                        .to_owned(),
                ));
            }
        }
        let mut keys = std::collections::BTreeSet::new();
        for member in &self.members {
            member.validate()?;
            if !keys.insert(member.source_key()) {
                return Err(HardcopySourceError::DuplicateSourceSetMember(
                    member.source_key().to_owned(),
                ));
            }
        }
        Ok(())
    }

    fn compute_definition_digest(&self) -> Result<ContentDigest, HardcopySourceError> {
        canonical_digest(
            b"rspice-hardcopy-source-set-definition-v1",
            &HardcopySourceSetDigestMaterial {
                schema_version: self.schema_version,
                document_id: self.document_id,
                revision: self.revision,
                name: &self.name,
                document_kind: self.document_kind,
                scope: &self.scope,
                members: &self.members,
            },
        )
    }

    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    #[must_use]
    pub const fn document_id(&self) -> HardcopyDocumentId {
        self.document_id
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn document_kind(&self) -> HardcopyDocumentKind {
        self.document_kind
    }

    #[must_use]
    pub const fn scope(&self) -> &HardcopyScope {
        &self.scope
    }

    #[must_use]
    pub fn members(&self) -> &[HardcopySourceSetMember] {
        &self.members
    }

    #[must_use]
    pub const fn definition_digest(&self) -> ContentDigest {
        self.definition_digest
    }

    #[must_use]
    pub fn source_key(&self) -> String {
        format!("hardcopy-source-set:{}", self.document_id)
    }
}

#[derive(Serialize)]
struct HardcopySourceSetDigestMaterial<'a> {
    schema_version: u32,
    document_id: HardcopyDocumentId,
    revision: ObjectRevision,
    name: &'a str,
    document_kind: HardcopyDocumentKind,
    scope: &'a HardcopyScope,
    members: &'a [HardcopySourceSetMember],
}


pub(crate) fn canonical_digest<T: Serialize>(
    domain: &[u8],
    value: &T,
) -> Result<ContentDigest, HardcopySourceError> {
    let mut hasher = Sha256::new();
    hasher.update(domain);
    serde_json::to_writer(&mut DigestWriter(&mut hasher), value)
        .map_err(|error| HardcopySourceError::Serialization(error.to_string()))?;
    Ok(ContentDigest::from_bytes(hasher.finalize().into()))
}

struct DigestWriter<'a>(&'a mut Sha256);

impl std::io::Write for DigestWriter<'_> {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        self.0.update(buffer);
        Ok(buffer.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub(crate) fn validate_label(
    field: &'static str,
    value: &str,
    maximum_bytes: usize,
) -> Result<(), HardcopySourceError> {
    if value.is_empty()
        || value != value.trim()
        || value.len() > maximum_bytes
        || value.chars().any(char::is_control)
    {
        return Err(HardcopySourceError::InvalidLabel {
            field,
            maximum_bytes,
        });
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum HardcopySourceError {
    #[error(
        "invalid {field}; it must be trimmed, non-blank, control-free, and at most {maximum_bytes} bytes"
    )]
    InvalidLabel {
        field: &'static str,
        maximum_bytes: usize,
    },
    #[error("active hardcopy source `{0}` is not retained")]
    SourceNotRetained(String),
    #[error("retained hardcopy source `{source_key}` is unavailable: {reason}")]
    UnavailableRetainedSource { source_key: String, reason: String },
    #[error("hardcopy source-set schema version {0} is unsupported")]
    UnsupportedSourceSetSchema(u32),
    #[error("invalid hardcopy source set: {0}")]
    InvalidSourceSet(String),
    #[error("hardcopy source set contains duplicate member `{0}`")]
    DuplicateSourceSetMember(String),
    #[error(
        "hardcopy source-set definition digest mismatch (expected {expected}, actual {actual})"
    )]
    SourceSetDigestMismatch {
        expected: ContentDigest,
        actual: ContentDigest,
    },
    #[error("hardcopy source-set member `{source_key}` no longer matches its pinned authority")]
    StaleSourceSetMember { source_key: String },
    #[error("active hardcopy source `{0}` resolves to more than one retained document")]
    AmbiguousActiveSource(String),
    #[error("the active surface has no retained {0} authority")]
    NoActiveDocumentAuthority(&'static str),
    #[error("the active document authority is stale or inconsistent: {0}")]
    StaleActiveDocumentAuthority(String),
    #[error("unsupported active document: {0}")]
    UnsupportedDocument(String),
    #[error("scope {0:?} cannot be resolved from this exact active document")]
    UnsupportedScope(HardcopyScope),
    #[error("the requested selection is empty")]
    EmptySelection,
    #[error("the selected hardcopy source contains no publishable content")]
    EmptyContent,
    #[error(
        "schematic topology changed during source resolution (expected {expected}, actual {actual})"
    )]
    StaleSchematic { expected: u64, actual: u64 },
    #[error("schematic sheet partition is invalid: {0}")]
    InvalidSheetPartition(String),
    #[error("component {component_id} has no exact symbol: {reason}")]
    UnresolvedCellSymbol { component_id: u64, reason: String },
    #[error("component {0} references invalid authored symbol metadata")]
    InvalidAuthoredSymbol(u64),
    #[error("result source is not retained: {0}")]
    UnretainedResult(String),
    #[error("dataset `{0}` resolves to more than one retained run")]
    AmbiguousRetainedDataset(String),
    #[error("analysis sequence {0} resolves to more than one retained result")]
    AmbiguousRetainedAnalysis(u64),
    #[error("retained waveform `{0}` is empty, mismatched, or non-finite")]
    InvalidRetainedWaveform(String),
    #[error("retained traces collide on stable identity {0}")]
    DuplicateStableTraceIdentity(u64),
    #[error("viewer `{0}` has no implemented semantic hardcopy adapter")]
    UnsupportedVisualizationViewer(String),
    #[error("the retained analysis contains no {0} evidence required by the active viewer")]
    MissingViewerEvidence(&'static str),
    #[error("active visualization pane cannot be published: {0}")]
    InvalidVisualizationSource(String),
    #[error("result pane has an invalid or degenerate numeric axis range")]
    InvalidResultRange,
    #[error("result pane contains a non-finite retained sample")]
    NonFiniteResultSample,
    #[error("report revision {0:?} is not retained in its authenticated history")]
    UnretainedReportRevision(ObjectRevision),
    #[error("linked report references require an exact retained source inventory")]
    ReportReferenceInventoryRequired,
    #[error(
        "report block {block_id} is not authenticated for hardcopy (reference status: {currentness:?})"
    )]
    UnauthenticatedReportReference {
        block_id: ReportBlockId,
        currentness: ReportReferenceCurrentness,
    },
    #[error("report block {block_id} ({kind}) cannot be published: {reason}")]
    UnsupportedAuthenticatedReportBlock {
        block_id: ReportBlockId,
        kind: &'static str,
        reason: String,
    },
    #[error("report source authentication failed: {0}")]
    InvalidReportSource(String),
    #[error("hardcopy source coordinates exceed the supported physical range")]
    CoordinateOverflow,
    #[error("hardcopy source serialization failed: {0}")]
    Serialization(String),
    #[error("hardcopy worker snapshot is {0} bytes and exceeds the 64 MiB transfer boundary")]
    WorkerSnapshotTooLarge(usize),
    #[error("hardcopy worker snapshot validation failed: {0}")]
    InvalidWorkerSnapshot(String),
    #[error(
        "prepared hardcopy worker snapshot is {0} bytes and exceeds the 64 MiB transfer boundary"
    )]
    PreparedWorkerSnapshotTooLarge(usize),
    #[error("prepared hardcopy worker snapshot validation failed: {0}")]
    InvalidPreparedWorkerSnapshot(String),
    #[error("hardcopy contract rejected the source: {0}")]
    HardcopyContract(String),
}

#[cfg(test)]
impl HardcopySourceSet {
    /// Reorder members without recomputing the definition digest, so a test
    /// can prove that validation rejects a tampered set. The field stays
    /// private: order is part of what the digest authenticates.
    pub(crate) fn reverse_members_for_test(&mut self) {
        self.members.reverse();
    }
}
