//! Atomic, authenticated multi-format report publication packages.
//!
//! Package construction is deliberately UI- and filesystem-independent. All
//! requested payloads, the deterministic manifest, and its receipt are built
//! and verified in memory before a package is returned. Any writer, audit,
//! naming, gate, or resource failure drops the candidate and returns only an
//! error; this API has no partial-success state.

use std::collections::HashSet;

use serde::Serialize;
use sha2::{Digest as _, Sha256};

use crate::product::{ContentDigest, ObjectRevision, ResultDocumentId};

use super::report_document::{
    ReportBlockId, ReportDocument, ReportError, ReportReferenceAudit, ReportReferenceAuditEntry,
    ReportReferenceCurrentness, ReportReferenceMode,
};
use super::report_pdfa::{
    ReportPdfAArtifact, ReportPdfAError, ReportPdfAOptions, serialize_report_pdfa_2b,
};
use super::report_publication::{
    ReportPublicationArtifact, ReportPublicationError, ReportPublicationFormat,
    publish_canonical_json, publish_selected_table_csv, publish_standalone_html,
};

/// Aggregate byte ceiling for payloads, manifest, and receipt.
pub const MAX_REPORT_PACKAGE_BYTES: usize = 256 * 1_048_576;
/// Maximum number of payload entries (including individually selected CSVs).
pub const MAX_REPORT_PACKAGE_PAYLOADS: usize = 1_027;
/// Maximum serialized manifest size.
pub const MAX_REPORT_PACKAGE_MANIFEST_BYTES: usize = 4 * 1_048_576;
/// Maximum serialized receipt size.
pub const MAX_REPORT_PACKAGE_RECEIPT_BYTES: usize = 1_048_576;

const MAX_PROFILE_NAME_BYTES: usize = 128;
const MAX_WATERMARK_BYTES: usize = 128;
const MAX_GATE_DISCLOSURE_BYTES: usize = 4_096;
const MAX_REVISION_NOTE_BYTES: usize = 16_384;
const MAX_FILE_NAME_BYTES: usize = 255;

const MANIFEST_MEDIA_TYPE: &str = "application/vnd.rspice.report-package-manifest+json";
const RECEIPT_MEDIA_TYPE: &str = "application/vnd.rspice.report-package-receipt+json";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReportPackageFormatSelection {
    pub pdfa_2b: bool,
    pub standalone_html: bool,
    pub canonical_json: bool,
    /// `None` omits CSV. `Some` requests exactly the listed table identities;
    /// an empty selection is rejected rather than silently treated as omit.
    pub selected_table_csv: Option<Vec<ReportBlockId>>,
}

impl ReportPackageFormatSelection {
    #[must_use]
    pub const fn none() -> Self {
        Self {
            pdfa_2b: false,
            standalone_html: false,
            canonical_json: false,
            selected_table_csv: None,
        }
    }

    fn validate(&self) -> Result<(), ReportPackageError> {
        if !self.pdfa_2b
            && !self.standalone_html
            && !self.canonical_json
            && self.selected_table_csv.is_none()
        {
            return Err(ReportPackageError::NoFormatsRequested);
        }
        if self.selected_table_csv.as_ref().is_some_and(Vec::is_empty) {
            return Err(ReportPackageError::EmptyCsvSelection);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case", tag = "kind", content = "label")]
pub enum ReportPackageWatermark {
    None,
    Draft,
    Confidential,
    Custom(String),
}

impl ReportPackageWatermark {
    fn validate(&self) -> Result<(), ReportPackageError> {
        if let Self::Custom(label) = self {
            validate_single_line(
                label,
                MAX_WATERMARK_BYTES,
                ReportPackageError::InvalidWatermark,
            )?;
        }
        Ok(())
    }
}

/// Governed package gate represented in the authenticated manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReportPublicationGate {
    EngineeringReview,
    SignOffCandidate,
    Released,
}

impl ReportPublicationGate {
    const fn requires_current_references(self) -> bool {
        matches!(self, Self::SignOffCandidate | Self::Released)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReportGateDisclosure {
    gate: ReportPublicationGate,
    statement: String,
}

impl ReportGateDisclosure {
    pub fn new(
        gate: ReportPublicationGate,
        statement: impl Into<String>,
    ) -> Result<Self, ReportPackageError> {
        let disclosure = Self {
            gate,
            statement: statement.into(),
        };
        disclosure.validate()?;
        Ok(disclosure)
    }

    fn validate(&self) -> Result<(), ReportPackageError> {
        validate_text(
            &self.statement,
            MAX_GATE_DISCLOSURE_BYTES,
            ReportPackageError::InvalidGateDisclosure,
        )
    }

    #[must_use]
    pub const fn gate(&self) -> ReportPublicationGate {
        self.gate
    }

    #[must_use]
    pub fn statement(&self) -> &str {
        &self.statement
    }
}

/// Deterministic package policy bound into the manifest.
///
/// `watermark` is a cryptographically bound package-classification mark. It
/// does not claim that every heterogeneous payload format contains a visual
/// overlay; consumers must display the manifest mark when presenting the
/// package as a governed set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReportPublicationProfile {
    name: String,
    formats: ReportPackageFormatSelection,
    watermark: ReportPackageWatermark,
    gate_disclosure: ReportGateDisclosure,
    revision_note: String,
}

impl ReportPublicationProfile {
    pub fn new(
        name: impl Into<String>,
        formats: ReportPackageFormatSelection,
        watermark: ReportPackageWatermark,
        gate_disclosure: ReportGateDisclosure,
        revision_note: impl Into<String>,
    ) -> Result<Self, ReportPackageError> {
        let profile = Self {
            name: name.into(),
            formats,
            watermark,
            gate_disclosure,
            revision_note: revision_note.into(),
        };
        profile.validate()?;
        Ok(profile)
    }

    fn validate(&self) -> Result<(), ReportPackageError> {
        validate_single_line(
            &self.name,
            MAX_PROFILE_NAME_BYTES,
            ReportPackageError::InvalidProfileName,
        )?;
        self.formats.validate()?;
        self.watermark.validate()?;
        self.gate_disclosure.validate()?;
        validate_text(
            &self.revision_note,
            MAX_REVISION_NOTE_BYTES,
            ReportPackageError::InvalidRevisionNote,
        )?;
        if self.gate_disclosure.gate.requires_current_references()
            && matches!(&self.watermark, ReportPackageWatermark::Draft)
        {
            return Err(ReportPackageError::DraftWatermarkAtSignOff);
        }
        Ok(())
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn formats(&self) -> &ReportPackageFormatSelection {
        &self.formats
    }

    #[must_use]
    pub const fn watermark(&self) -> &ReportPackageWatermark {
        &self.watermark
    }

    #[must_use]
    pub const fn gate_disclosure(&self) -> &ReportGateDisclosure {
        &self.gate_disclosure
    }

    #[must_use]
    pub fn revision_note(&self) -> &str {
        &self.revision_note
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportPackageFileKind {
    PdfA2b,
    StandaloneHtml,
    CanonicalJson,
    TableCsv { block_id: ReportBlockId },
    Manifest,
    Receipt,
}

impl ReportPackageFileKind {
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::PdfA2b => "pdfa-2b",
            Self::StandaloneHtml => "standalone-html",
            Self::CanonicalJson => "canonical-json",
            Self::TableCsv { .. } => "table-csv",
            Self::Manifest => "package-manifest",
            Self::Receipt => "package-receipt",
        }
    }

    const fn expected_media_type(&self) -> &'static str {
        match self {
            Self::PdfA2b => ReportPdfAArtifact::MEDIA_TYPE,
            Self::StandaloneHtml => ReportPublicationFormat::StandaloneHtml.media_type(),
            Self::CanonicalJson => ReportPublicationFormat::CanonicalJson.media_type(),
            Self::TableCsv { .. } => ReportPublicationFormat::TableCsv.media_type(),
            Self::Manifest => MANIFEST_MEDIA_TYPE,
            Self::Receipt => RECEIPT_MEDIA_TYPE,
        }
    }

    const fn block_id(&self) -> Option<ReportBlockId> {
        match self {
            Self::TableCsv { block_id } => Some(*block_id),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ReportPackageFileContent {
    Standard(Box<ReportPublicationArtifact>),
    PdfA(ReportPdfAArtifact),
    Owned {
        bytes: Vec<u8>,
        digest: ContentDigest,
    },
}

/// One immutable file in a completed package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportPackageFile {
    kind: ReportPackageFileKind,
    file_name: String,
    media_type: &'static str,
    content: ReportPackageFileContent,
}

impl ReportPackageFile {
    fn from_standard(
        kind: ReportPackageFileKind,
        artifact: ReportPublicationArtifact,
    ) -> Result<Self, ReportPackageError> {
        let expected = match &kind {
            ReportPackageFileKind::StandaloneHtml => ReportPublicationFormat::StandaloneHtml,
            ReportPackageFileKind::CanonicalJson => ReportPublicationFormat::CanonicalJson,
            ReportPackageFileKind::TableCsv { block_id } => {
                if artifact.block_id() != Some(*block_id) {
                    return Err(ReportPackageError::ArtifactMetadataMismatch);
                }
                ReportPublicationFormat::TableCsv
            }
            _ => return Err(ReportPackageError::ArtifactMetadataMismatch),
        };
        if artifact.format() != expected {
            return Err(ReportPackageError::ArtifactMetadataMismatch);
        }
        let file = Self {
            kind,
            file_name: artifact.file_name().to_owned(),
            media_type: artifact.media_type(),
            content: ReportPackageFileContent::Standard(Box::new(artifact)),
        };
        file.validate()?;
        Ok(file)
    }

    fn from_pdfa(
        document: &ReportDocument,
        artifact: ReportPdfAArtifact,
    ) -> Result<Self, ReportPackageError> {
        let file = Self {
            kind: ReportPackageFileKind::PdfA2b,
            file_name: format!(
                "rspice-report-{}-r{}.pdf",
                document.id(),
                document.revision().get()
            ),
            media_type: ReportPdfAArtifact::MEDIA_TYPE,
            content: ReportPackageFileContent::PdfA(artifact),
        };
        file.validate()?;
        Ok(file)
    }

    fn owned(
        kind: ReportPackageFileKind,
        file_name: String,
        media_type: &'static str,
        bytes: Vec<u8>,
        maximum: usize,
    ) -> Result<Self, ReportPackageError> {
        if bytes.is_empty() || bytes.len() > maximum {
            return Err(ReportPackageError::ResourceLimit {
                scope: kind.label(),
                maximum_bytes: maximum,
            });
        }
        let digest = sha256(&bytes);
        let file = Self {
            kind,
            file_name,
            media_type,
            content: ReportPackageFileContent::Owned { bytes, digest },
        };
        file.validate()?;
        Ok(file)
    }

    fn validate(&self) -> Result<(), ReportPackageError> {
        validate_safe_file_name(&self.file_name)?;
        if self.media_type != self.kind.expected_media_type() {
            return Err(ReportPackageError::InvalidMediaType {
                file_name: self.file_name.clone(),
                expected: self.kind.expected_media_type(),
                actual: self.media_type.to_owned(),
            });
        }
        if self.bytes().is_empty() || sha256(self.bytes()) != self.content_digest() {
            return Err(ReportPackageError::ArtifactDigestMismatch(
                self.file_name.clone(),
            ));
        }
        Ok(())
    }

    #[must_use]
    pub const fn kind(&self) -> &ReportPackageFileKind {
        &self.kind
    }

    #[must_use]
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    #[must_use]
    pub const fn media_type(&self) -> &'static str {
        self.media_type
    }

    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        match &self.content {
            ReportPackageFileContent::Standard(artifact) => artifact.bytes(),
            ReportPackageFileContent::PdfA(artifact) => artifact.bytes(),
            ReportPackageFileContent::Owned { bytes, .. } => bytes,
        }
    }

    #[must_use]
    pub fn size_bytes(&self) -> usize {
        self.bytes().len()
    }

    #[must_use]
    pub const fn content_digest(&self) -> ContentDigest {
        match &self.content {
            ReportPackageFileContent::Standard(artifact) => artifact.content_digest(),
            ReportPackageFileContent::PdfA(artifact) => artifact.digest(),
            ReportPackageFileContent::Owned { digest, .. } => *digest,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReportPackageManifestEntry {
    ordinal: u16,
    format: String,
    block_id: Option<ReportBlockId>,
    file_name: String,
    media_type: String,
    size_bytes: u64,
    sha256: ContentDigest,
}

impl ReportPackageManifestEntry {
    fn from_file(ordinal: usize, file: &ReportPackageFile) -> Result<Self, ReportPackageError> {
        Ok(Self {
            ordinal: u16::try_from(ordinal).map_err(|_| ReportPackageError::TooManyPayloads)?,
            format: file.kind().label().to_owned(),
            block_id: file.kind().block_id(),
            file_name: file.file_name().to_owned(),
            media_type: file.media_type().to_owned(),
            size_bytes: u64::try_from(file.size_bytes())
                .map_err(|_| ReportPackageError::SizeOverflow)?,
            sha256: file.content_digest(),
        })
    }

    #[must_use]
    pub const fn ordinal(&self) -> u16 {
        self.ordinal
    }

    #[must_use]
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    #[must_use]
    pub fn media_type(&self) -> &str {
        &self.media_type
    }

    #[must_use]
    pub const fn size_bytes(&self) -> u64 {
        self.size_bytes
    }

    #[must_use]
    pub const fn content_digest(&self) -> ContentDigest {
        self.sha256
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReportPackageManifest {
    schema_version: u16,
    domain: &'static str,
    document_id: ResultDocumentId,
    document_revision: ObjectRevision,
    reference_audit_digest: ContentDigest,
    profile: ReportPublicationProfile,
    entries: Vec<ReportPackageManifestEntry>,
    aggregate_payload_bytes: u64,
}

impl ReportPackageManifest {
    const SCHEMA_VERSION: u16 = 1;
    const DOMAIN: &'static str = "rspice-report-publication-package-v1";

    #[must_use]
    pub const fn document_id(&self) -> ResultDocumentId {
        self.document_id
    }

    #[must_use]
    pub const fn document_revision(&self) -> ObjectRevision {
        self.document_revision
    }

    #[must_use]
    pub const fn reference_audit_digest(&self) -> ContentDigest {
        self.reference_audit_digest
    }

    #[must_use]
    pub const fn profile(&self) -> &ReportPublicationProfile {
        &self.profile
    }

    #[must_use]
    pub fn entries(&self) -> &[ReportPackageManifestEntry] {
        &self.entries
    }

    #[must_use]
    pub const fn aggregate_payload_bytes(&self) -> u64 {
        self.aggregate_payload_bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct ReceiptAuthenticationMaterial {
    domain: &'static str,
    document_id: ResultDocumentId,
    document_revision: ObjectRevision,
    reference_audit_digest: ContentDigest,
    manifest_file_name: String,
    manifest_media_type: &'static str,
    manifest_size_bytes: u64,
    manifest_sha256: ContentDigest,
    payload_count: u16,
    aggregate_payload_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReportPackageReceipt {
    schema_version: u16,
    domain: &'static str,
    authentication: ReceiptAuthenticationMaterial,
    authentication_sha256: ContentDigest,
}

impl ReportPackageReceipt {
    const SCHEMA_VERSION: u16 = 1;
    const DOMAIN: &'static str = "rspice-report-publication-receipt-v1";

    #[must_use]
    pub const fn document_id(&self) -> ResultDocumentId {
        self.authentication.document_id
    }

    #[must_use]
    pub const fn document_revision(&self) -> ObjectRevision {
        self.authentication.document_revision
    }

    #[must_use]
    pub const fn reference_audit_digest(&self) -> ContentDigest {
        self.authentication.reference_audit_digest
    }

    #[must_use]
    pub const fn manifest_digest(&self) -> ContentDigest {
        self.authentication.manifest_sha256
    }

    #[must_use]
    pub const fn authentication_digest(&self) -> ContentDigest {
        self.authentication_sha256
    }
}

/// Fully constructed, internally verified publication package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportPublicationPackage {
    payloads: Vec<ReportPackageFile>,
    manifest: ReportPackageManifest,
    manifest_file: ReportPackageFile,
    receipt: ReportPackageReceipt,
    receipt_file: ReportPackageFile,
    aggregate_bytes: usize,
}

impl ReportPublicationPackage {
    #[must_use]
    pub fn payloads(&self) -> &[ReportPackageFile] {
        &self.payloads
    }

    #[must_use]
    pub const fn manifest(&self) -> &ReportPackageManifest {
        &self.manifest
    }

    #[must_use]
    pub const fn manifest_file(&self) -> &ReportPackageFile {
        &self.manifest_file
    }

    #[must_use]
    pub const fn receipt(&self) -> &ReportPackageReceipt {
        &self.receipt
    }

    #[must_use]
    pub const fn receipt_file(&self) -> &ReportPackageFile {
        &self.receipt_file
    }

    #[must_use]
    pub const fn aggregate_bytes(&self) -> usize {
        self.aggregate_bytes
    }

    fn validate(&self) -> Result<(), ReportPackageError> {
        validate_file_set(&self.payloads)?;
        self.manifest_file.validate()?;
        self.receipt_file.validate()?;
        validate_unique_file_names(
            self.payloads
                .iter()
                .chain([&self.manifest_file, &self.receipt_file]),
        )?;
        for (index, (entry, file)) in self.manifest.entries.iter().zip(&self.payloads).enumerate() {
            if usize::from(entry.ordinal) != index
                || entry.file_name != file.file_name
                || entry.media_type != file.media_type
                || entry.size_bytes
                    != u64::try_from(file.size_bytes())
                        .map_err(|_| ReportPackageError::SizeOverflow)?
                || entry.sha256 != file.content_digest()
                || entry.format != file.kind.label()
                || entry.block_id != file.kind.block_id()
            {
                return Err(ReportPackageError::ManifestEntryMismatch);
            }
        }
        let payload_bytes = aggregate_file_bytes(self.payloads.iter())?;
        let payload_bytes_u64 =
            u64::try_from(payload_bytes).map_err(|_| ReportPackageError::SizeOverflow)?;
        if self.manifest.entries.len() != self.payloads.len()
            || self.manifest.aggregate_payload_bytes != payload_bytes_u64
            || self.receipt.authentication.document_id != self.manifest.document_id
            || self.receipt.authentication.document_revision != self.manifest.document_revision
            || self.receipt.authentication.reference_audit_digest
                != self.manifest.reference_audit_digest
            || usize::from(self.receipt.authentication.payload_count) != self.payloads.len()
            || self.receipt.authentication.aggregate_payload_bytes != payload_bytes_u64
            || self.receipt.authentication.manifest_file_name != self.manifest_file.file_name
            || self.receipt.authentication.manifest_media_type != self.manifest_file.media_type
            || sha256(self.manifest_file.bytes()) != self.manifest_file.content_digest()
            || serde_json::to_vec(&self.manifest)? != self.manifest_file.bytes()
            || self.receipt.authentication.manifest_sha256 != self.manifest_file.content_digest()
            || self.receipt.authentication.manifest_size_bytes
                != u64::try_from(self.manifest_file.size_bytes())
                    .map_err(|_| ReportPackageError::SizeOverflow)?
            || receipt_authentication_digest(&self.receipt.authentication)?
                != self.receipt.authentication_sha256
            || serde_json::to_vec(&self.receipt)? != self.receipt_file.bytes()
            || sha256(self.receipt_file.bytes()) != self.receipt_file.content_digest()
        {
            return Err(ReportPackageError::ControlArtifactMismatch);
        }
        let expected_size = aggregate_file_bytes(
            self.payloads
                .iter()
                .chain([&self.manifest_file, &self.receipt_file]),
        )?;
        if expected_size != self.aggregate_bytes {
            return Err(ReportPackageError::ControlArtifactMismatch);
        }
        Ok(())
    }
}

/// Build every requested format and return a package only after complete
/// manifest/receipt verification.
pub fn publish_report_package(
    document: &ReportDocument,
    reference_audit: &ReportReferenceAudit,
    profile: &ReportPublicationProfile,
    pdfa_options: Option<&ReportPdfAOptions>,
) -> Result<ReportPublicationPackage, ReportPackageError> {
    document.validate()?;
    profile.validate()?;
    validate_reference_audit(document, reference_audit)?;
    validate_gate(profile, reference_audit)?;
    match (profile.formats.pdfa_2b, pdfa_options) {
        (true, None) => return Err(ReportPackageError::MissingPdfAOptions),
        (false, Some(_)) => return Err(ReportPackageError::UnexpectedPdfAOptions),
        _ => {}
    }

    // All values stay local until the final package validates. The order is a
    // stable package contract, independent of caller selection order.
    let mut payloads = Vec::new();
    if profile.formats.pdfa_2b {
        let pdf = serialize_report_pdfa_2b(
            document,
            pdfa_options.ok_or(ReportPackageError::MissingPdfAOptions)?,
        )?;
        payloads.push(ReportPackageFile::from_pdfa(document, pdf)?);
    }
    if profile.formats.standalone_html {
        payloads.push(ReportPackageFile::from_standard(
            ReportPackageFileKind::StandaloneHtml,
            publish_standalone_html(document)?,
        )?);
    }
    if profile.formats.canonical_json {
        payloads.push(ReportPackageFile::from_standard(
            ReportPackageFileKind::CanonicalJson,
            publish_canonical_json(document)?,
        )?);
    }
    if let Some(selected) = &profile.formats.selected_table_csv {
        for artifact in publish_selected_table_csv(document, selected)? {
            let block_id = artifact
                .block_id()
                .ok_or(ReportPackageError::ArtifactMetadataMismatch)?;
            payloads.push(ReportPackageFile::from_standard(
                ReportPackageFileKind::TableCsv { block_id },
                artifact,
            )?);
        }
    }
    if payloads.len() > MAX_REPORT_PACKAGE_PAYLOADS {
        return Err(ReportPackageError::TooManyPayloads);
    }
    validate_file_set(&payloads)?;
    let aggregate_payload_bytes = aggregate_file_bytes(payloads.iter())?;

    let entries = payloads
        .iter()
        .enumerate()
        .map(|(index, file)| ReportPackageManifestEntry::from_file(index, file))
        .collect::<Result<Vec<_>, _>>()?;
    let mut normalized_profile = profile.clone();
    if normalized_profile.formats.selected_table_csv.is_some() {
        normalized_profile.formats.selected_table_csv = Some(
            payloads
                .iter()
                .filter_map(|file| file.kind().block_id())
                .collect(),
        );
    }
    let manifest = ReportPackageManifest {
        schema_version: ReportPackageManifest::SCHEMA_VERSION,
        domain: ReportPackageManifest::DOMAIN,
        document_id: document.id(),
        document_revision: document.revision(),
        reference_audit_digest: reference_audit.audit_digest,
        profile: normalized_profile,
        entries,
        aggregate_payload_bytes: u64::try_from(aggregate_payload_bytes)
            .map_err(|_| ReportPackageError::SizeOverflow)?,
    };
    let manifest_bytes = serde_json::to_vec(&manifest)?;
    let manifest_file_name = format!(
        "rspice-report-{}-r{}-package-manifest.json",
        document.id(),
        document.revision().get()
    );
    let manifest_file = ReportPackageFile::owned(
        ReportPackageFileKind::Manifest,
        manifest_file_name.clone(),
        MANIFEST_MEDIA_TYPE,
        manifest_bytes,
        MAX_REPORT_PACKAGE_MANIFEST_BYTES,
    )?;

    let authentication = ReceiptAuthenticationMaterial {
        domain: ReportPackageReceipt::DOMAIN,
        document_id: document.id(),
        document_revision: document.revision(),
        reference_audit_digest: reference_audit.audit_digest,
        manifest_file_name,
        manifest_media_type: MANIFEST_MEDIA_TYPE,
        manifest_size_bytes: u64::try_from(manifest_file.size_bytes())
            .map_err(|_| ReportPackageError::SizeOverflow)?,
        manifest_sha256: manifest_file.content_digest(),
        payload_count: u16::try_from(payloads.len())
            .map_err(|_| ReportPackageError::TooManyPayloads)?,
        aggregate_payload_bytes: u64::try_from(aggregate_payload_bytes)
            .map_err(|_| ReportPackageError::SizeOverflow)?,
    };
    let receipt = ReportPackageReceipt {
        schema_version: ReportPackageReceipt::SCHEMA_VERSION,
        domain: ReportPackageReceipt::DOMAIN,
        authentication_sha256: receipt_authentication_digest(&authentication)?,
        authentication,
    };
    let receipt_file = ReportPackageFile::owned(
        ReportPackageFileKind::Receipt,
        format!(
            "rspice-report-{}-r{}-package-receipt.json",
            document.id(),
            document.revision().get()
        ),
        RECEIPT_MEDIA_TYPE,
        serde_json::to_vec(&receipt)?,
        MAX_REPORT_PACKAGE_RECEIPT_BYTES,
    )?;
    let aggregate_bytes =
        aggregate_file_bytes(payloads.iter().chain([&manifest_file, &receipt_file]))?;
    let package = ReportPublicationPackage {
        payloads,
        manifest,
        manifest_file,
        receipt,
        receipt_file,
        aggregate_bytes,
    };
    package.validate()?;
    Ok(package)
}

fn validate_reference_audit(
    document: &ReportDocument,
    audit: &ReportReferenceAudit,
) -> Result<(), ReportPackageError> {
    if audit.document_id != document.id() || audit.document_revision != document.revision() {
        return Err(ReportPackageError::AuditBindingMismatch {
            expected_document_id: document.id(),
            expected_revision: document.revision(),
            actual_document_id: audit.document_id,
            actual_revision: audit.document_revision,
        });
    }
    if audit.audit_digest != compute_audit_digest(audit)? {
        return Err(ReportPackageError::InvalidAuditDigest);
    }
    let referenced_blocks: Vec<_> = document
        .pages()
        .iter()
        .flat_map(|page| page.sections())
        .flat_map(|section| section.blocks())
        .filter_map(|block| block.kind().reference().map(|reference| (block, reference)))
        .collect();
    if referenced_blocks.len() != audit.entries.len() {
        return Err(ReportPackageError::AuditEntryMismatch);
    }
    let mut identities = HashSet::with_capacity(audit.entries.len());
    for ((block, reference), entry) in referenced_blocks.iter().zip(&audit.entries) {
        if !identities.insert(entry.block_id)
            || !audit_entry_matches_reference(block.id(), reference, entry)
        {
            return Err(ReportPackageError::AuditEntryMismatch);
        }
    }
    Ok(())
}

fn audit_entry_matches_reference(
    block_id: ReportBlockId,
    reference: &ReportReferenceMode,
    entry: &ReportReferenceAuditEntry,
) -> bool {
    let snapshot = reference.snapshot();
    if entry.block_id != block_id
        || entry.source != snapshot.source
        || entry.captured_revision != snapshot.source_revision
        || entry.captured_content_digest != snapshot.content_digest
    {
        return false;
    }
    let missing_unique = entry
        .missing_dataset_bindings
        .iter()
        .copied()
        .collect::<HashSet<_>>()
        .len()
        == entry.missing_dataset_bindings.len();
    if !missing_unique
        || entry
            .missing_dataset_bindings
            .iter()
            .any(|binding| !snapshot.dataset_bindings.contains(binding))
    {
        return false;
    }
    match reference {
        ReportReferenceMode::Frozen { artifact, .. } => {
            entry.currentness == ReportReferenceCurrentness::Frozen
                && entry.frozen_artifact_media_type.as_deref() == Some(artifact.media_type())
                && entry.frozen_artifact_digest == Some(artifact.content_digest())
                && entry.missing_dataset_bindings.is_empty()
        }
        ReportReferenceMode::Linked { .. } => {
            entry.currentness != ReportReferenceCurrentness::Frozen
                && entry.frozen_artifact_media_type.is_none()
                && entry.frozen_artifact_digest.is_none()
                && audit_linked_currentness_is_coherent(entry)
        }
    }
}

fn audit_linked_currentness_is_coherent(entry: &ReportReferenceAuditEntry) -> bool {
    match entry.currentness {
        ReportReferenceCurrentness::Current => {
            entry.available_revision == entry.captured_revision
                && entry.available_content_digest == Some(entry.captured_content_digest)
                && entry.missing_dataset_bindings.is_empty()
        }
        ReportReferenceCurrentness::UpdateAvailable => {
            entry.available_revision.is_some()
                && entry.available_revision != entry.captured_revision
                && entry.available_content_digest.is_some()
                && entry.missing_dataset_bindings.is_empty()
        }
        ReportReferenceCurrentness::SourceContentChanged => {
            entry.available_revision == entry.captured_revision
                && entry
                    .available_content_digest
                    .is_some_and(|digest| digest != entry.captured_content_digest)
                && entry.missing_dataset_bindings.is_empty()
        }
        ReportReferenceCurrentness::SourceMissing => {
            entry.available_revision.is_none() && entry.available_content_digest.is_none()
        }
        ReportReferenceCurrentness::DatasetMissing => {
            entry.available_content_digest.is_some() && !entry.missing_dataset_bindings.is_empty()
        }
        ReportReferenceCurrentness::Frozen => false,
    }
}

fn validate_gate(
    profile: &ReportPublicationProfile,
    audit: &ReportReferenceAudit,
) -> Result<(), ReportPackageError> {
    if profile.gate_disclosure.gate.requires_current_references()
        && !audit.is_current_for_sign_off()
    {
        return Err(ReportPackageError::ReferencesNotCurrentForSignOff);
    }
    Ok(())
}

fn compute_audit_digest(audit: &ReportReferenceAudit) -> Result<ContentDigest, ReportPackageError> {
    #[derive(Serialize)]
    struct AuditDigestMaterial<'a> {
        domain: &'static str,
        document_id: ResultDocumentId,
        document_revision: ObjectRevision,
        entries: &'a [ReportReferenceAuditEntry],
    }
    let material = AuditDigestMaterial {
        domain: "rspice-report-reference-audit-v1",
        document_id: audit.document_id,
        document_revision: audit.document_revision,
        entries: &audit.entries,
    };
    Ok(sha256(&serde_json::to_vec(&material)?))
}

fn receipt_authentication_digest(
    authentication: &ReceiptAuthenticationMaterial,
) -> Result<ContentDigest, ReportPackageError> {
    Ok(sha256(&serde_json::to_vec(authentication)?))
}

fn validate_file_set(files: &[ReportPackageFile]) -> Result<(), ReportPackageError> {
    if files.len() > MAX_REPORT_PACKAGE_PAYLOADS {
        return Err(ReportPackageError::TooManyPayloads);
    }
    for file in files {
        file.validate()?;
    }
    validate_unique_file_names(files)
}

fn validate_unique_file_names<'a>(
    files: impl IntoIterator<Item = &'a ReportPackageFile>,
) -> Result<(), ReportPackageError> {
    let mut names = HashSet::new();
    for file in files {
        let comparison_name = file.file_name.to_ascii_lowercase();
        if !names.insert(comparison_name) {
            return Err(ReportPackageError::DuplicateFileName(
                file.file_name.clone(),
            ));
        }
    }
    Ok(())
}

fn aggregate_file_bytes<'a>(
    files: impl IntoIterator<Item = &'a ReportPackageFile>,
) -> Result<usize, ReportPackageError> {
    aggregate_sizes(files.into_iter().map(ReportPackageFile::size_bytes))
}

fn aggregate_sizes(sizes: impl IntoIterator<Item = usize>) -> Result<usize, ReportPackageError> {
    let mut total = 0_usize;
    for size in sizes {
        total = total
            .checked_add(size)
            .ok_or(ReportPackageError::SizeOverflow)?;
        if total > MAX_REPORT_PACKAGE_BYTES {
            return Err(ReportPackageError::ResourceLimit {
                scope: "report publication package",
                maximum_bytes: MAX_REPORT_PACKAGE_BYTES,
            });
        }
    }
    Ok(total)
}

fn validate_safe_file_name(value: &str) -> Result<(), ReportPackageError> {
    let safe = !value.is_empty()
        && value.len() <= MAX_FILE_NAME_BYTES
        && value == value.trim()
        && !value.starts_with('.')
        && !value.ends_with('.')
        && !value.contains("..")
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'));
    if !safe {
        return Err(ReportPackageError::UnsafeFileName(value.to_owned()));
    }
    Ok(())
}

fn validate_single_line(
    value: &str,
    maximum: usize,
    error: ReportPackageError,
) -> Result<(), ReportPackageError> {
    if value.is_empty()
        || value != value.trim()
        || value.len() > maximum
        || value.chars().any(char::is_control)
    {
        return Err(error);
    }
    Ok(())
}

fn validate_text(
    value: &str,
    maximum: usize,
    error: ReportPackageError,
) -> Result<(), ReportPackageError> {
    let invalid_control = value
        .chars()
        .any(|character| character.is_control() && !matches!(character, '\n' | '\r' | '\t'));
    if value.trim().is_empty() || value != value.trim() || value.len() > maximum || invalid_control
    {
        return Err(error);
    }
    Ok(())
}

fn sha256(bytes: &[u8]) -> ContentDigest {
    ContentDigest::from_bytes(Sha256::digest(bytes).into())
}

#[derive(Debug, thiserror::Error)]
pub enum ReportPackageError {
    #[error("report document is invalid: {0}")]
    InvalidDocument(#[from] ReportError),
    #[error("report package JSON serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error(transparent)]
    Publication(#[from] ReportPublicationError),
    #[error(transparent)]
    PdfA(#[from] ReportPdfAError),
    #[error("at least one report output format must be requested")]
    NoFormatsRequested,
    #[error("CSV was requested without selecting at least one table block")]
    EmptyCsvSelection,
    #[error("the publication profile name is invalid")]
    InvalidProfileName,
    #[error("the package watermark is invalid")]
    InvalidWatermark,
    #[error("the gate disclosure is invalid")]
    InvalidGateDisclosure,
    #[error("the revision note is invalid")]
    InvalidRevisionNote,
    #[error("a draft watermark is incompatible with a sign-off or released package")]
    DraftWatermarkAtSignOff,
    #[error("PDF/A-2b was requested without exact publication options")]
    MissingPdfAOptions,
    #[error("PDF/A options were supplied even though PDF/A-2b was not requested")]
    UnexpectedPdfAOptions,
    #[error(
        "reference audit is bound to document {actual_document_id} revision {actual_revision:?}, expected {expected_document_id} revision {expected_revision:?}"
    )]
    AuditBindingMismatch {
        expected_document_id: ResultDocumentId,
        expected_revision: ObjectRevision,
        actual_document_id: ResultDocumentId,
        actual_revision: ObjectRevision,
    },
    #[error("reference audit digest does not authenticate its exact audit entries")]
    InvalidAuditDigest,
    #[error("reference audit entries do not exactly match this report revision")]
    AuditEntryMismatch,
    #[error(
        "sign-off and released packages require every report reference to be current or frozen"
    )]
    ReferencesNotCurrentForSignOff,
    #[error("a writer returned artifact metadata incompatible with the requested format")]
    ArtifactMetadataMismatch,
    #[error("artifact {0} does not match its SHA-256 metadata")]
    ArtifactDigestMismatch(String),
    #[error("unsafe package file name {0:?}")]
    UnsafeFileName(String),
    #[error("duplicate package file name {0:?}")]
    DuplicateFileName(String),
    #[error("file {file_name:?} declares media type {actual:?}; expected {expected:?}")]
    InvalidMediaType {
        file_name: String,
        expected: &'static str,
        actual: String,
    },
    #[error("report package contains too many payload files")]
    TooManyPayloads,
    #[error("report package byte accounting overflowed")]
    SizeOverflow,
    #[error("{scope} exceeds the {maximum_bytes}-byte package resource limit")]
    ResourceLimit {
        scope: &'static str,
        maximum_bytes: usize,
    },
    #[error("package manifest entries do not exactly match payload files")]
    ManifestEntryMismatch,
    #[error("package manifest or receipt authentication failed")]
    ControlArtifactMismatch,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{DatasetBinding, DatasetId};
    use crate::results::report_document::{
        DataTableBlock, ProseBlock, ProseStyle, ReportEdit, ReportReferenceInventory,
        ReportReferenceInventoryEntry, ReportReferenceSnapshot, ReportSourceId, TableCell,
        TableColumn,
    };
    use crate::results::report_pdfa::ReportPublicationDate;

    fn digest(seed: u8) -> ContentDigest {
        ContentDigest::from_bytes([seed; 32])
    }

    fn simple_report() -> ReportDocument {
        let mut report = ReportDocument::new("Package verification").unwrap();
        report
            .transact(
                report.revision(),
                vec![ReportEdit::AddPage {
                    title: "Summary".to_owned(),
                }],
                1,
            )
            .unwrap();
        let page_id = report.pages()[0].id();
        report
            .transact(
                report.revision(),
                vec![ReportEdit::AddSection {
                    page_id,
                    title: "Decision".to_owned(),
                }],
                2,
            )
            .unwrap();
        let section_id = report.pages()[0].sections()[0].id();
        report
            .transact(
                report.revision(),
                vec![ReportEdit::AddBlock {
                    section_id,
                    kind: super::super::report_document::ReportBlockKind::Prose(ProseBlock {
                        style: ProseStyle::Conclusion,
                        markdown: "All governed checks completed.".to_owned(),
                    }),
                }],
                3,
            )
            .unwrap();
        report
    }

    fn table_report() -> (ReportDocument, ReportReferenceInventory, ReportBlockId) {
        let dataset = DatasetBinding::new(DatasetId::new(), digest(2));
        let snapshot = ReportReferenceSnapshot::new(
            ReportSourceId::ExternalRecord {
                namespace: "verification".to_owned(),
                key: "table-source".to_owned(),
            },
            Some(ObjectRevision::new(4).unwrap()),
            digest(3),
            vec![dataset],
        )
        .unwrap();
        let reference = ReportReferenceMode::Linked {
            snapshot: snapshot.clone(),
        };
        let mut report = ReportDocument::new("Table package").unwrap();
        report
            .transact(
                report.revision(),
                vec![ReportEdit::AddPage {
                    title: "Measurements".to_owned(),
                }],
                1,
            )
            .unwrap();
        let page_id = report.pages()[0].id();
        report
            .transact(
                report.revision(),
                vec![ReportEdit::AddSection {
                    page_id,
                    title: "Nominal".to_owned(),
                }],
                2,
            )
            .unwrap();
        let section_id = report.pages()[0].sections()[0].id();
        report
            .transact(
                report.revision(),
                [("Operating point", 1.25), ("Guard band", 2.25)]
                    .into_iter()
                    .map(|(title, value)| ReportEdit::AddBlock {
                        section_id,
                        kind: super::super::report_document::ReportBlockKind::DataTable(
                            DataTableBlock {
                                title: title.to_owned(),
                                columns: vec![TableColumn {
                                    key: "value".to_owned(),
                                    heading: "Value".to_owned(),
                                    unit: Some("V".to_owned()),
                                }],
                                rows: vec![vec![TableCell::Number {
                                    value,
                                    unit: Some("V".to_owned()),
                                }]],
                                reference: reference.clone(),
                            },
                        ),
                    })
                    .collect(),
                3,
            )
            .unwrap();
        let block_id = report.pages()[0].sections()[0].blocks()[0].id();
        let inventory = ReportReferenceInventory {
            sources: vec![
                ReportReferenceInventoryEntry::new(
                    snapshot.source,
                    snapshot.source_revision,
                    snapshot.content_digest,
                    snapshot.dataset_bindings,
                )
                .unwrap(),
            ],
            available_datasets: vec![dataset],
        };
        (report, inventory, block_id)
    }

    fn formats(html: bool, json: bool) -> ReportPackageFormatSelection {
        ReportPackageFormatSelection {
            pdfa_2b: false,
            standalone_html: html,
            canonical_json: json,
            selected_table_csv: None,
        }
    }

    fn profile(selection: ReportPackageFormatSelection) -> ReportPublicationProfile {
        ReportPublicationProfile::new(
            "Verification release",
            selection,
            ReportPackageWatermark::Confidential,
            ReportGateDisclosure::new(
                ReportPublicationGate::EngineeringReview,
                "Engineering review package; not a released sign-off record.",
            )
            .unwrap(),
            "Regenerated after the nominal operating-point review.",
        )
        .unwrap()
    }

    #[test]
    fn package_bytes_manifest_and_receipt_are_deterministic() {
        let report = simple_report();
        let audit = report
            .audit_references(&ReportReferenceInventory::default())
            .unwrap();
        let profile = profile(formats(true, true));

        let first = publish_report_package(&report, &audit, &profile, None).unwrap();
        let second = publish_report_package(&report, &audit, &profile, None).unwrap();

        assert_eq!(first, second);
        assert_eq!(first.payloads().len(), 2);
        assert_eq!(
            first
                .payloads()
                .iter()
                .map(|file| file.kind().label())
                .collect::<Vec<_>>(),
            vec!["standalone-html", "canonical-json"]
        );
        assert_eq!(
            first.manifest().reference_audit_digest(),
            audit.audit_digest
        );
        assert_eq!(
            first.receipt().manifest_digest(),
            first.manifest_file().content_digest()
        );
    }

    #[test]
    fn semantically_identical_csv_selections_have_one_canonical_package_order() {
        let (report, inventory, first) = table_report();
        let second = report.pages()[0].sections()[0].blocks()[1].id();
        let audit = report.audit_references(&inventory).unwrap();
        let selection = |ids| ReportPackageFormatSelection {
            pdfa_2b: false,
            standalone_html: false,
            canonical_json: false,
            selected_table_csv: Some(ids),
        };

        let forward = publish_report_package(
            &report,
            &audit,
            &profile(selection(vec![first, second])),
            None,
        )
        .unwrap();
        let reverse = publish_report_package(
            &report,
            &audit,
            &profile(selection(vec![second, first])),
            None,
        )
        .unwrap();

        assert_eq!(forward, reverse);
        assert_eq!(
            forward
                .payloads()
                .iter()
                .filter_map(|file| file.kind().block_id())
                .collect::<Vec<_>>(),
            vec![first, second]
        );
    }

    #[test]
    fn manifest_metadata_authenticates_every_exact_payload() {
        let (report, inventory, block_id) = table_report();
        let audit = report.audit_references(&inventory).unwrap();
        let mut selection = formats(true, true);
        selection.selected_table_csv = Some(vec![block_id]);
        let package = publish_report_package(&report, &audit, &profile(selection), None).unwrap();

        assert_eq!(package.payloads().len(), 3);
        for (ordinal, (entry, file)) in package
            .manifest()
            .entries()
            .iter()
            .zip(package.payloads())
            .enumerate()
        {
            assert_eq!(usize::from(entry.ordinal()), ordinal);
            assert_eq!(entry.file_name(), file.file_name());
            assert_eq!(entry.media_type(), file.media_type());
            assert_eq!(entry.size_bytes(), file.size_bytes() as u64);
            assert_eq!(entry.content_digest(), sha256(file.bytes()));
        }
        assert_eq!(
            package.receipt().authentication_digest(),
            receipt_authentication_digest(&package.receipt().authentication).unwrap()
        );
        assert_eq!(
            package.manifest().document_id(),
            package.receipt().document_id()
        );
    }

    #[test]
    fn requested_formats_are_exact_and_ordered() {
        let report = simple_report();
        let audit = report
            .audit_references(&ReportReferenceInventory::default())
            .unwrap();
        let mut selection = formats(true, false);
        selection.pdfa_2b = true;
        let profile = profile(selection);
        let options =
            ReportPdfAOptions::new(ReportPublicationDate::new(2026, 7, 17, 14, 30, 0).unwrap());

        let package = publish_report_package(&report, &audit, &profile, Some(&options)).unwrap();
        assert_eq!(
            package
                .payloads()
                .iter()
                .map(|file| file.kind().label())
                .collect::<Vec<_>>(),
            vec!["pdfa-2b", "standalone-html"]
        );
        assert!(package.payloads()[0].bytes().starts_with(b"%PDF-"));
        assert!(
            package.payloads()[1]
                .bytes()
                .starts_with(b"<!doctype html>")
        );
    }

    #[test]
    fn format_options_and_gate_profile_cannot_be_ambiguous() {
        let report = simple_report();
        let audit = report
            .audit_references(&ReportReferenceInventory::default())
            .unwrap();
        let disclosure = || {
            ReportGateDisclosure::new(
                ReportPublicationGate::EngineeringReview,
                "Engineering review only.",
            )
            .unwrap()
        };
        assert!(matches!(
            ReportPublicationProfile::new(
                "Empty",
                ReportPackageFormatSelection::none(),
                ReportPackageWatermark::None,
                disclosure(),
                "No output"
            ),
            Err(ReportPackageError::NoFormatsRequested)
        ));

        let pdf_profile = profile(ReportPackageFormatSelection {
            pdfa_2b: true,
            standalone_html: false,
            canonical_json: false,
            selected_table_csv: None,
        });
        assert!(matches!(
            publish_report_package(&report, &audit, &pdf_profile, None),
            Err(ReportPackageError::MissingPdfAOptions)
        ));
        let options =
            ReportPdfAOptions::new(ReportPublicationDate::new(2026, 7, 17, 14, 30, 0).unwrap());
        assert!(matches!(
            publish_report_package(
                &report,
                &audit,
                &profile(formats(false, true)),
                Some(&options)
            ),
            Err(ReportPackageError::UnexpectedPdfAOptions)
        ));

        assert!(matches!(
            ReportPublicationProfile::new(
                "Unsafe sign-off",
                formats(false, true),
                ReportPackageWatermark::Draft,
                ReportGateDisclosure::new(
                    ReportPublicationGate::SignOffCandidate,
                    "Sign-off candidate."
                )
                .unwrap(),
                "Candidate"
            ),
            Err(ReportPackageError::DraftWatermarkAtSignOff)
        ));
    }

    #[test]
    fn writer_failure_returns_no_partial_package() {
        let report = simple_report();
        let audit = report
            .audit_references(&ReportReferenceInventory::default())
            .unwrap();
        let prose_id = report.pages()[0].sections()[0].blocks()[0].id();
        let selection = ReportPackageFormatSelection {
            pdfa_2b: false,
            standalone_html: true,
            canonical_json: true,
            selected_table_csv: Some(vec![prose_id]),
        };

        let outcome = publish_report_package(&report, &audit, &profile(selection), None);
        assert!(matches!(
            outcome,
            Err(ReportPackageError::Publication(
                ReportPublicationError::BlockIsNotDataTable(id)
            )) if id == prose_id
        ));
    }

    #[test]
    fn empty_and_duplicate_csv_selections_fail_closed() {
        let (report, inventory, block_id) = table_report();
        let audit = report.audit_references(&inventory).unwrap();
        let empty = ReportPackageFormatSelection {
            pdfa_2b: false,
            standalone_html: false,
            canonical_json: false,
            selected_table_csv: Some(Vec::new()),
        };
        assert!(matches!(
            ReportPublicationProfile::new(
                "Profile",
                empty,
                ReportPackageWatermark::None,
                ReportGateDisclosure::new(ReportPublicationGate::EngineeringReview, "Review only")
                    .unwrap(),
                "Initial revision"
            ),
            Err(ReportPackageError::EmptyCsvSelection)
        ));

        let duplicate = ReportPackageFormatSelection {
            pdfa_2b: false,
            standalone_html: false,
            canonical_json: false,
            selected_table_csv: Some(vec![block_id, block_id]),
        };
        assert!(matches!(
            publish_report_package(&report, &audit, &profile(duplicate), None),
            Err(ReportPackageError::Publication(
                ReportPublicationError::DuplicateCsvSelection
            ))
        ));
    }

    #[test]
    fn audit_binding_digest_and_signoff_gate_are_enforced() {
        let (report, inventory, _) = table_report();
        let stale = report
            .audit_references(&ReportReferenceInventory::default())
            .unwrap();
        let signoff = ReportPublicationProfile::new(
            "Sign-off",
            formats(false, true),
            ReportPackageWatermark::None,
            ReportGateDisclosure::new(
                ReportPublicationGate::SignOffCandidate,
                "Candidate requires current immutable references.",
            )
            .unwrap(),
            "Candidate revision",
        )
        .unwrap();
        let current = report.audit_references(&inventory).unwrap();
        assert!(publish_report_package(&report, &current, &signoff, None).is_ok());
        assert!(matches!(
            publish_report_package(&report, &stale, &signoff, None),
            Err(ReportPackageError::ReferencesNotCurrentForSignOff)
        ));

        let mut tampered = stale.clone();
        tampered.audit_digest = digest(99);
        assert!(matches!(
            publish_report_package(&report, &tampered, &profile(formats(false, true)), None),
            Err(ReportPackageError::InvalidAuditDigest)
        ));

        let mut incomplete = stale.clone();
        incomplete.entries.pop();
        incomplete.audit_digest = compute_audit_digest(&incomplete).unwrap();
        assert!(matches!(
            publish_report_package(&report, &incomplete, &profile(formats(false, true)), None),
            Err(ReportPackageError::AuditEntryMismatch)
        ));

        let other = simple_report();
        let other_audit = other
            .audit_references(&ReportReferenceInventory::default())
            .unwrap();
        assert!(matches!(
            publish_report_package(&report, &other_audit, &profile(formats(false, true)), None),
            Err(ReportPackageError::AuditBindingMismatch { .. })
        ));
    }

    #[test]
    fn unsafe_duplicate_names_and_media_mismatch_are_rejected() {
        let report = simple_report();
        let artifact = publish_canonical_json(&report).unwrap();
        let file = ReportPackageFile::from_standard(ReportPackageFileKind::CanonicalJson, artifact)
            .unwrap();
        assert!(matches!(
            validate_file_set(&[file.clone(), file]),
            Err(ReportPackageError::DuplicateFileName(_))
        ));
        let mut wrong_media = ReportPackageFile::from_standard(
            ReportPackageFileKind::CanonicalJson,
            publish_canonical_json(&report).unwrap(),
        )
        .unwrap();
        wrong_media.media_type = "text/html; charset=utf-8";
        assert!(matches!(
            wrong_media.validate(),
            Err(ReportPackageError::InvalidMediaType { .. })
        ));
        assert!(matches!(
            validate_safe_file_name("../report.json"),
            Err(ReportPackageError::UnsafeFileName(_))
        ));
    }

    #[test]
    fn package_resource_limits_are_checked_without_large_allocations() {
        // Size accounting itself is checked by a dedicated arithmetic helper
        // so the boundary test does not allocate hundreds of megabytes.
        assert!(aggregate_sizes([MAX_REPORT_PACKAGE_BYTES]).is_ok());
        assert!(matches!(
            aggregate_sizes([MAX_REPORT_PACKAGE_BYTES, 1]),
            Err(ReportPackageError::ResourceLimit { .. })
        ));
    }
}
