//! The netlist document.
//!
//! A deck as an editable document: its identity, its text, and the revision
//! that a validation receipt is pinned to.

use std::collections::{HashMap, HashSet};
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest as _, Sha256};
use uuid::Uuid;

use crate::product::{ContentDigest, ObjectRevision, RevisionError};

use super::outline::{IncludeDirective, NetlistOutline, parse_include_directives};
use super::search::{FindError, FindOptions, ReplaceOutcome, ReplaceScope, replace_in_source};

const DOCUMENT_SCHEMA_VERSION: u32 = 1;

/// Non-nil identity retained across every ownership and source transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct NetlistDocumentId(Uuid);

impl NetlistDocumentId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn try_from_uuid(value: Uuid) -> Result<Self, DocumentError> {
        if value.is_nil() {
            Err(DocumentError::NilDocumentIdentity)
        } else {
            Ok(Self(value))
        }
    }

    #[must_use]
    pub const fn as_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for NetlistDocumentId {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for NetlistDocumentId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Uuid::deserialize(deserializer)?;
        Self::try_from_uuid(value).map_err(serde::de::Error::custom)
    }
}

impl fmt::Display for NetlistDocumentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DocumentOwnership {
    /// Immutable source projected from project inputs.
    Generated,
    /// Exact source bytes most recently loaded from an external origin.
    Imported,
    /// User-owned buffer; it may descend from generated or imported source.
    Editable,
}

/// Versioned project inputs from which a generated deck was produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenerationInput {
    revision: ObjectRevision,
    digest: ContentDigest,
}

impl GenerationInput {
    #[must_use]
    pub const fn new(revision: ObjectRevision, digest: ContentDigest) -> Self {
        Self { revision, digest }
    }

    #[must_use]
    pub const fn revision(self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub const fn digest(self) -> ContentDigest {
        self.digest
    }
}

/// Caller-owned path, URI, or browser file-handle identity. No path
/// normalization is performed because exact browser and remote locators are
/// not necessarily host filesystem paths.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct SourceLocator {
    logical_identity: String,
    display_name: String,
    native_origin: Option<String>,
}

impl SourceLocator {
    pub fn try_new(
        locator: impl Into<String>,
        display_name: impl Into<String>,
    ) -> Result<Self, DocumentError> {
        let value = Self {
            logical_identity: locator.into(),
            display_name: display_name.into(),
            native_origin: None,
        };
        value.validate()?;
        Ok(value)
    }

    pub fn with_native_origin(
        mut self,
        native_origin: impl Into<String>,
    ) -> Result<Self, DocumentError> {
        let native_origin = native_origin.into();
        validate_nonempty_text("native source origin", &native_origin)?;
        self.native_origin = Some(native_origin);
        Ok(self)
    }

    /// Preserve the logical include identity while replacing the user-granted
    /// physical authority. Browser relinks deliberately pass `None` because
    /// immutable picker bytes do not grant a reopenable native path.
    pub fn with_relinked_origin(
        mut self,
        display_name: impl Into<String>,
        native_origin: Option<String>,
    ) -> Result<Self, DocumentError> {
        let display_name = display_name.into();
        validate_nonempty_text("source display name", &display_name)?;
        if let Some(origin) = native_origin.as_deref() {
            validate_nonempty_text("native source origin", origin)?;
        }
        self.display_name = display_name;
        self.native_origin = native_origin;
        self.validate()?;
        Ok(self)
    }

    fn exact(locator: &str) -> Self {
        Self {
            logical_identity: locator.to_owned(),
            display_name: locator.to_owned(),
            native_origin: None,
        }
    }

    fn validate(&self) -> Result<(), DocumentError> {
        validate_nonempty_text("source logical identity", &self.logical_identity)?;
        validate_nonempty_text("source display name", &self.display_name)?;
        if let Some(native_origin) = &self.native_origin {
            validate_nonempty_text("native source origin", native_origin)?;
        }
        Ok(())
    }

    #[must_use]
    pub fn locator(&self) -> &str {
        &self.logical_identity
    }

    #[must_use]
    pub fn logical_identity(&self) -> &str {
        &self.logical_identity
    }

    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    #[must_use]
    pub fn native_origin(&self) -> Option<&str> {
        self.native_origin.as_deref()
    }
}

#[derive(Deserialize)]
struct SourceLocatorData {
    logical_identity: String,
    display_name: String,
    native_origin: Option<String>,
}

impl<'de> Deserialize<'de> for SourceLocator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = SourceLocatorData::deserialize(deserializer)?;
        let mut locator = Self::try_new(value.logical_identity, value.display_name)
            .map_err(serde::de::Error::custom)?;
        if let Some(native_origin) = value.native_origin {
            locator = locator
                .with_native_origin(native_origin)
                .map_err(serde::de::Error::custom)?;
        }
        Ok(locator)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedProvenance {
    generator: String,
    input: GenerationInput,
}

impl GeneratedProvenance {
    pub fn try_new(
        generator: impl Into<String>,
        input: GenerationInput,
    ) -> Result<Self, DocumentError> {
        let generator = generator.into();
        validate_nonempty_text("generator identity", &generator)?;
        Ok(Self { generator, input })
    }

    fn validate(&self) -> Result<(), DocumentError> {
        validate_nonempty_text("generator identity", &self.generator)
    }

    #[must_use]
    pub fn generator(&self) -> &str {
        &self.generator
    }

    #[must_use]
    pub const fn input(&self) -> GenerationInput {
        self.input
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedProvenance {
    origin: SourceLocator,
    imported_digest: ContentDigest,
}

impl ImportedProvenance {
    #[must_use]
    pub fn origin(&self) -> &SourceLocator {
        &self.origin
    }

    #[must_use]
    pub const fn imported_digest(&self) -> ContentDigest {
        self.imported_digest
    }
}

/// Audit provenance survives a Generated -> Imported -> Editable sequence.
/// `last_saved` is historical; the document's current persisted target is
/// exposed separately by [`NetlistDocument::save_acknowledgement`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceProvenance {
    generated: GeneratedProvenance,
    imported: Option<ImportedProvenance>,
    last_saved: Option<SaveAcknowledgement>,
}

impl SourceProvenance {
    #[must_use]
    pub const fn generated(&self) -> &GeneratedProvenance {
        &self.generated
    }

    #[must_use]
    pub const fn imported(&self) -> Option<&ImportedProvenance> {
        self.imported.as_ref()
    }

    #[must_use]
    pub const fn last_saved(&self) -> Option<&SaveAcknowledgement> {
        self.last_saved.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveAcknowledgement {
    origin: SourceLocator,
    digest: ContentDigest,
}

impl SaveAcknowledgement {
    #[must_use]
    pub fn origin(&self) -> &SourceLocator {
        &self.origin
    }

    #[must_use]
    pub const fn digest(&self) -> ContentDigest {
        self.digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "status")]
pub enum DependencyResolution {
    Unresolved,
    Resolved {
        content_digest: ContentDigest,
        source: String,
    },
    Missing {
        reason: String,
    },
}

/// External authority that owns a retained dependency before an explicit
/// copy-to-project transition. The authority is durable provenance and UI
/// policy; execution always consumes the authenticated retained bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum DependencySourceAuthority {
    #[default]
    External,
    Vendor,
    TechnologyPackage,
    StandardLibrary,
}

impl DependencySourceAuthority {
    pub const fn label(self) -> &'static str {
        match self {
            Self::External => "external reference",
            Self::Vendor => "vendor source",
            Self::TechnologyPackage => "technology package",
            Self::StandardLibrary => "standard library",
        }
    }
}

impl DependencyResolution {
    #[must_use]
    pub const fn content_digest(&self) -> Option<ContentDigest> {
        match self {
            Self::Resolved { content_digest, .. } => Some(*content_digest),
            Self::Unresolved | Self::Missing { .. } => None,
        }
    }

    #[must_use]
    pub fn source(&self) -> Option<&str> {
        match self {
            Self::Resolved { source, .. } => Some(source),
            Self::Unresolved | Self::Missing { .. } => None,
        }
    }

    #[must_use]
    pub fn source_bytes(&self) -> Option<&[u8]> {
        self.source().map(str::as_bytes)
    }

    #[must_use]
    pub fn byte_length(&self) -> Option<usize> {
        self.source_bytes().map(<[u8]>::len)
    }
}

/// One direct or transitive source dependency. Direct indices refer to
/// [`NetlistDocument::include_directives`]; transitive records identify their
/// exact parent locator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyMetadata {
    requested_locator: String,
    locator: SourceLocator,
    direct_include_index: Option<usize>,
    parent: Option<SourceLocator>,
    parent_include_index: Option<usize>,
    #[serde(default)]
    authority: DependencySourceAuthority,
    resolution: DependencyResolution,
}

/// Stable design identity attached to one exact generated source line.
/// Identities are logical project identities, never transient egui handles or
/// host pointers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedSourceMapEntry {
    generated_line: usize,
    cell_identity: String,
    view_identity: String,
    instance_identity: Option<String>,
    component_identity: Option<String>,
}

impl GeneratedSourceMapEntry {
    pub fn try_new(
        generated_line: usize,
        cell_identity: impl Into<String>,
        view_identity: impl Into<String>,
        instance_identity: Option<String>,
        component_identity: Option<String>,
    ) -> Result<Self, DocumentError> {
        if generated_line == 0 {
            return Err(DocumentError::InvalidSourceMap(
                "generated source-map lines are one-based".to_owned(),
            ));
        }
        let value = Self {
            generated_line,
            cell_identity: cell_identity.into(),
            view_identity: view_identity.into(),
            instance_identity,
            component_identity,
        };
        value.validate_identity()?;
        Ok(value)
    }

    fn validate_identity(&self) -> Result<(), DocumentError> {
        validate_source_map_identity("cell identity", &self.cell_identity)?;
        validate_source_map_identity("view identity", &self.view_identity)?;
        if let Some(identity) = &self.instance_identity {
            validate_source_map_identity("instance identity", identity)?;
        }
        if let Some(identity) = &self.component_identity {
            validate_source_map_identity("component identity", identity)?;
        }
        Ok(())
    }

    #[must_use]
    pub const fn generated_line(&self) -> usize {
        self.generated_line
    }

    #[must_use]
    pub fn cell_identity(&self) -> &str {
        &self.cell_identity
    }

    #[must_use]
    pub fn view_identity(&self) -> &str {
        &self.view_identity
    }

    #[must_use]
    pub fn instance_identity(&self) -> Option<&str> {
        self.instance_identity.as_deref()
    }

    #[must_use]
    pub fn component_identity(&self) -> Option<&str> {
        self.component_identity.as_deref()
    }
}

impl DependencyMetadata {
    #[must_use]
    pub fn unresolved_direct(include_index: usize, locator: SourceLocator) -> Self {
        let requested_locator = locator.logical_identity.clone();
        Self {
            requested_locator,
            locator,
            direct_include_index: Some(include_index),
            parent: None,
            parent_include_index: None,
            authority: DependencySourceAuthority::External,
            resolution: DependencyResolution::Unresolved,
        }
    }

    pub fn unresolved_direct_to(
        include_index: usize,
        requested_locator: impl Into<String>,
        locator: SourceLocator,
    ) -> Result<Self, DocumentError> {
        let requested_locator = requested_locator.into();
        validate_nonempty_text("requested dependency locator", &requested_locator)
            .map_err(|error| DocumentError::InvalidDependency(error.to_string()))?;
        Ok(Self {
            requested_locator,
            locator,
            direct_include_index: Some(include_index),
            parent: None,
            parent_include_index: None,
            authority: DependencySourceAuthority::External,
            resolution: DependencyResolution::Unresolved,
        })
    }

    #[must_use]
    pub fn unresolved_transitive(
        parent: SourceLocator,
        parent_include_index: usize,
        locator: SourceLocator,
    ) -> Self {
        let requested_locator = locator.logical_identity.clone();
        Self {
            requested_locator,
            locator,
            direct_include_index: None,
            parent: Some(parent),
            parent_include_index: Some(parent_include_index),
            authority: DependencySourceAuthority::External,
            resolution: DependencyResolution::Unresolved,
        }
    }

    pub fn unresolved_transitive_to(
        parent: SourceLocator,
        parent_include_index: usize,
        requested_locator: impl Into<String>,
        locator: SourceLocator,
    ) -> Result<Self, DocumentError> {
        let requested_locator = requested_locator.into();
        validate_nonempty_text("requested dependency locator", &requested_locator)
            .map_err(|error| DocumentError::InvalidDependency(error.to_string()))?;
        Ok(Self {
            requested_locator,
            locator,
            direct_include_index: None,
            parent: Some(parent),
            parent_include_index: Some(parent_include_index),
            authority: DependencySourceAuthority::External,
            resolution: DependencyResolution::Unresolved,
        })
    }

    pub fn resolve_utf8(mut self, source_bytes: Vec<u8>) -> Result<Self, DocumentError> {
        let source = decode_utf8(source_bytes)?;
        self.resolution = DependencyResolution::Resolved {
            content_digest: digest(source.as_bytes()),
            source,
        };
        Ok(self)
    }

    pub fn with_resolution(
        mut self,
        resolution: DependencyResolution,
    ) -> Result<Self, DocumentError> {
        validate_resolution(&resolution)?;
        self.resolution = resolution;
        Ok(self)
    }

    #[must_use]
    pub fn with_authority(mut self, authority: DependencySourceAuthority) -> Self {
        self.authority = authority;
        self
    }

    #[must_use]
    pub fn locator(&self) -> &SourceLocator {
        &self.locator
    }

    #[must_use]
    pub fn requested_locator(&self) -> &str {
        &self.requested_locator
    }

    #[must_use]
    pub const fn direct_include_index(&self) -> Option<usize> {
        self.direct_include_index
    }

    #[must_use]
    pub const fn parent(&self) -> Option<&SourceLocator> {
        self.parent.as_ref()
    }

    #[must_use]
    pub const fn parent_include_index(&self) -> Option<usize> {
        self.parent_include_index
    }

    #[must_use]
    pub const fn resolution(&self) -> &DependencyResolution {
        &self.resolution
    }

    #[must_use]
    pub const fn authority(&self) -> DependencySourceAuthority {
        self.authority
    }

    #[must_use]
    pub fn source(&self) -> Option<&str> {
        self.resolution.source()
    }

    #[must_use]
    pub fn source_bytes(&self) -> Option<&[u8]> {
        self.resolution.source_bytes()
    }
}

/// Exact generated deck and its authenticated input identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GeneratedArtifact {
    provenance: GeneratedProvenance,
    source: String,
    content_digest: ContentDigest,
    dependencies: Vec<DependencyMetadata>,
    source_map: Vec<GeneratedSourceMapEntry>,
}

impl GeneratedArtifact {
    pub fn try_from_utf8(
        provenance: GeneratedProvenance,
        source_bytes: Vec<u8>,
        dependencies: Vec<DependencyMetadata>,
        mut source_map: Vec<GeneratedSourceMapEntry>,
    ) -> Result<Self, DocumentError> {
        provenance.validate()?;
        let source = decode_utf8(source_bytes)?;
        let includes = parse_include_directives(&source);
        let dependencies = normalize_dependencies(&includes, dependencies)?;
        source_map.sort_by_key(GeneratedSourceMapEntry::generated_line);
        validate_source_map(&source, &source_map)?;
        Ok(Self {
            provenance,
            content_digest: digest(source.as_bytes()),
            source,
            dependencies,
            source_map,
        })
    }

    #[must_use]
    pub const fn provenance(&self) -> &GeneratedProvenance {
        &self.provenance
    }

    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    #[must_use]
    pub fn source_bytes(&self) -> &[u8] {
        self.source.as_bytes()
    }

    #[must_use]
    pub const fn content_digest(&self) -> ContentDigest {
        self.content_digest
    }

    #[must_use]
    pub fn dependencies(&self) -> &[DependencyMetadata] {
        &self.dependencies
    }

    #[must_use]
    pub fn source_map(&self) -> &[GeneratedSourceMapEntry] {
        &self.source_map
    }

    #[must_use]
    pub fn source_map_entry(&self, generated_line: usize) -> Option<&GeneratedSourceMapEntry> {
        self.source_map
            .binary_search_by_key(&generated_line, GeneratedSourceMapEntry::generated_line)
            .ok()
            .and_then(|index| self.source_map.get(index))
    }

    #[must_use]
    pub fn dependency_graph_is_sealed(&self) -> bool {
        self.dependencies.iter().all(|dependency| {
            matches!(
                &dependency.resolution,
                DependencyResolution::Resolved { .. }
            )
        })
    }

    fn validate(&self) -> Result<(), DocumentError> {
        self.provenance.validate()?;
        verify_digest(
            "generated artifact",
            self.content_digest,
            self.source.as_bytes(),
        )?;
        let includes = parse_include_directives(&self.source);
        validate_dependencies(&includes, &self.dependencies, true)?;
        validate_source_map(&self.source, &self.source_map)
    }
}

#[derive(Deserialize)]
struct GeneratedArtifactData {
    provenance: GeneratedProvenance,
    source: String,
    content_digest: ContentDigest,
    dependencies: Vec<DependencyMetadata>,
    source_map: Vec<GeneratedSourceMapEntry>,
}

impl<'de> Deserialize<'de> for GeneratedArtifact {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = GeneratedArtifactData::deserialize(deserializer)?;
        let value = Self {
            provenance: data.provenance,
            source: data.source,
            content_digest: data.content_digest,
            dependencies: data.dependencies,
            source_map: data.source_map,
        };
        value.validate().map_err(serde::de::Error::custom)?;
        Ok(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

/// One-based Unicode scalar source coordinate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourcePosition {
    line: usize,
    column: usize,
}

impl SourcePosition {
    pub fn try_new(line: usize, column: usize) -> Result<Self, DocumentError> {
        if line == 0 || column == 0 {
            return Err(DocumentError::InvalidDiagnostic(
                "diagnostic lines and columns are one-based".to_owned(),
            ));
        }
        Ok(Self { line, column })
    }

    #[must_use]
    pub const fn line(self) -> usize {
        self.line
    }

    #[must_use]
    pub const fn column(self) -> usize {
        self.column
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationDiagnostic {
    severity: DiagnosticSeverity,
    code: Option<String>,
    message: String,
    position: SourcePosition,
}

impl ValidationDiagnostic {
    pub fn try_new(
        severity: DiagnosticSeverity,
        message: impl Into<String>,
        line: usize,
        column: usize,
    ) -> Result<Self, DocumentError> {
        let message = message.into();
        validate_nonempty_text("diagnostic message", &message)
            .map_err(|error| DocumentError::InvalidDiagnostic(error.to_string()))?;
        Ok(Self {
            severity,
            code: None,
            message,
            position: SourcePosition::try_new(line, column)?,
        })
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Result<Self, DocumentError> {
        let code = code.into();
        validate_nonempty_text("diagnostic code", &code)
            .map_err(|error| DocumentError::InvalidDiagnostic(error.to_string()))?;
        self.code = Some(code);
        Ok(self)
    }

    #[must_use]
    pub const fn severity(&self) -> DiagnosticSeverity {
        self.severity
    }

    #[must_use]
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[must_use]
    pub const fn position(&self) -> SourcePosition {
        self.position
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationReport {
    content_digest: ContentDigest,
    diagnostics: Vec<ValidationDiagnostic>,
}

impl ValidationReport {
    #[must_use]
    pub const fn content_digest(&self) -> ContentDigest {
        self.content_digest
    }

    #[must_use]
    pub fn diagnostics(&self) -> &[ValidationDiagnostic] {
        &self.diagnostics
    }

    #[must_use]
    pub fn error_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
            .count()
    }

    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.severity == DiagnosticSeverity::Warning)
            .count()
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.error_count() == 0
    }
}

/// Auditable result of an atomic document transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionReceipt {
    document_id: NetlistDocumentId,
    previous_revision: ObjectRevision,
    revision: ObjectRevision,
    previous_ownership: DocumentOwnership,
    ownership: DocumentOwnership,
    previous_digest: ContentDigest,
    content_digest: ContentDigest,
}

impl TransitionReceipt {
    #[must_use]
    pub const fn document_id(self) -> NetlistDocumentId {
        self.document_id
    }

    #[must_use]
    pub const fn previous_revision(self) -> ObjectRevision {
        self.previous_revision
    }

    #[must_use]
    pub const fn revision(self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub const fn previous_ownership(self) -> DocumentOwnership {
        self.previous_ownership
    }

    #[must_use]
    pub const fn ownership(self) -> DocumentOwnership {
        self.ownership
    }

    #[must_use]
    pub const fn previous_digest(self) -> ContentDigest {
        self.previous_digest
    }

    #[must_use]
    pub const fn content_digest(self) -> ContentDigest {
        self.content_digest
    }
}

/// Canonical document model. Derived outline/include data is rebuilt from the
/// exact source after every successful source transition and after restore.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetlistDocument {
    id: NetlistDocumentId,
    revision: ObjectRevision,
    ownership: DocumentOwnership,
    source: String,
    content_digest: ContentDigest,
    generated_artifact: GeneratedArtifact,
    provenance: SourceProvenance,
    save_acknowledgement: Option<SaveAcknowledgement>,
    dependencies: Vec<DependencyMetadata>,
    validation: Option<ValidationReport>,
    outline: NetlistOutline,
    include_directives: Vec<IncludeDirective>,
}

impl NetlistDocument {
    pub fn from_generated(
        id: NetlistDocumentId,
        generated_artifact: GeneratedArtifact,
    ) -> Result<Self, DocumentError> {
        generated_artifact.validate()?;
        let source = generated_artifact.source.clone();
        let include_directives = parse_include_directives(&source);
        let value = Self {
            id,
            revision: ObjectRevision::INITIAL,
            ownership: DocumentOwnership::Generated,
            content_digest: generated_artifact.content_digest,
            provenance: SourceProvenance {
                generated: generated_artifact.provenance.clone(),
                imported: None,
                last_saved: None,
            },
            save_acknowledgement: None,
            dependencies: generated_artifact.dependencies.clone(),
            validation: None,
            outline: NetlistOutline::parse(&source),
            include_directives,
            source,
            generated_artifact,
        };
        value.validate_invariants()?;
        Ok(value)
    }

    #[must_use]
    pub const fn id(&self) -> NetlistDocumentId {
        self.id
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub const fn ownership(&self) -> DocumentOwnership {
        self.ownership
    }

    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    #[must_use]
    pub fn source_bytes(&self) -> &[u8] {
        self.source.as_bytes()
    }

    #[must_use]
    pub const fn content_digest(&self) -> ContentDigest {
        self.content_digest
    }

    #[must_use]
    pub const fn generated_artifact(&self) -> &GeneratedArtifact {
        &self.generated_artifact
    }

    #[must_use]
    pub const fn provenance(&self) -> &SourceProvenance {
        &self.provenance
    }

    #[must_use]
    pub const fn save_acknowledgement(&self) -> Option<&SaveAcknowledgement> {
        self.save_acknowledgement.as_ref()
    }

    #[must_use]
    pub fn saved_digest(&self) -> Option<ContentDigest> {
        self.save_acknowledgement
            .as_ref()
            .map(SaveAcknowledgement::digest)
    }

    #[must_use]
    pub fn is_dirty(&self) -> bool {
        self.ownership != DocumentOwnership::Generated
            && self.saved_digest() != Some(self.content_digest)
    }

    #[must_use]
    pub fn dependencies(&self) -> &[DependencyMetadata] {
        &self.dependencies
    }

    #[must_use]
    pub fn dependency_graph_is_sealed(&self) -> bool {
        self.dependencies.iter().all(|dependency| {
            matches!(
                &dependency.resolution,
                DependencyResolution::Resolved { .. }
            )
        })
    }

    #[must_use]
    pub const fn validation(&self) -> Option<&ValidationReport> {
        self.validation.as_ref()
    }

    #[must_use]
    pub const fn outline(&self) -> &NetlistOutline {
        &self.outline
    }

    #[must_use]
    pub fn include_directives(&self) -> &[IncludeDirective] {
        &self.include_directives
    }

    pub fn make_editable(
        &mut self,
        expected_digest: ContentDigest,
    ) -> Result<TransitionReceipt, DocumentError> {
        self.ensure_current(expected_digest)?;
        if self.ownership == DocumentOwnership::Editable {
            return Ok(self.noop_receipt());
        }
        let next_revision = self.revision.next()?;
        let before = self.receipt_before();
        self.ownership = DocumentOwnership::Editable;
        self.revision = next_revision;
        Ok(self.finish_receipt(before))
    }

    /// Create a separate, unsaved owned document without mutating the source
    /// document. Workbench document registries use this for the mockup's
    /// generated-primary plus editable-copy workflow.
    pub fn create_editable_copy(
        &self,
        id: NetlistDocumentId,
        expected_digest: ContentDigest,
    ) -> Result<Self, DocumentError> {
        self.ensure_current(expected_digest)?;
        let mut copy = self.clone();
        copy.id = id;
        copy.revision = ObjectRevision::INITIAL;
        copy.ownership = DocumentOwnership::Editable;
        copy.save_acknowledgement = None;
        copy.validation = None;
        copy.validate_invariants()?;
        Ok(copy)
    }

    pub fn replace_editable_source(
        &mut self,
        expected_digest: ContentDigest,
        source_bytes: Vec<u8>,
    ) -> Result<TransitionReceipt, DocumentError> {
        self.ensure_current(expected_digest)?;
        if self.ownership != DocumentOwnership::Editable {
            return Err(DocumentError::SourceIsNotEditable(self.ownership));
        }
        let source = decode_utf8(source_bytes)?;
        if source == self.source {
            return Ok(self.noop_receipt());
        }
        let next_revision = self.revision.next()?;
        let before = self.receipt_before();
        self.install_unowned_source(source)?;
        self.revision = next_revision;
        Ok(self.finish_receipt(before))
    }

    /// Apply literal or regular-expression replacement to an owned source.
    /// Generated and un-promoted imported documents fail closed as read-only.
    pub fn replace_editable_matches(
        &mut self,
        expected_digest: ContentDigest,
        query: &str,
        replacement: &str,
        options: FindOptions,
        scope: ReplaceScope,
    ) -> Result<(TransitionReceipt, ReplaceOutcome), DocumentError> {
        self.ensure_current(expected_digest)?;
        if self.ownership != DocumentOwnership::Editable {
            return Err(DocumentError::SourceIsNotEditable(self.ownership));
        }
        let outcome = replace_in_source(&self.source, query, replacement, options, scope)?;
        let receipt =
            self.replace_editable_source(expected_digest, outcome.source().as_bytes().to_vec())?;
        Ok((receipt, outcome))
    }

    pub fn import_source(
        &mut self,
        expected_digest: ContentDigest,
        origin: SourceLocator,
        source_bytes: Vec<u8>,
    ) -> Result<TransitionReceipt, DocumentError> {
        self.ensure_current(expected_digest)?;
        origin.validate()?;
        let source = decode_utf8(source_bytes)?;
        let next_revision = self.revision.next()?;
        let before = self.receipt_before();
        let imported_digest = digest(source.as_bytes());
        self.install_unowned_source(source)?;
        self.ownership = DocumentOwnership::Imported;
        let acknowledgement = SaveAcknowledgement {
            origin: origin.clone(),
            digest: imported_digest,
        };
        self.provenance.imported = Some(ImportedProvenance {
            origin,
            imported_digest,
        });
        self.provenance.last_saved = Some(acknowledgement.clone());
        self.save_acknowledgement = Some(acknowledgement);
        self.revision = next_revision;
        Ok(self.finish_receipt(before))
    }

    pub fn return_to_generated(
        &mut self,
        expected_digest: ContentDigest,
    ) -> Result<TransitionReceipt, DocumentError> {
        self.ensure_current(expected_digest)?;
        if self.ownership == DocumentOwnership::Generated
            && self.source == self.generated_artifact.source
        {
            return Ok(self.noop_receipt());
        }
        let next_revision = self.revision.next()?;
        let before = self.receipt_before();
        self.source.clone_from(&self.generated_artifact.source);
        self.content_digest = self.generated_artifact.content_digest;
        self.dependencies
            .clone_from(&self.generated_artifact.dependencies);
        self.provenance
            .generated
            .clone_from(&self.generated_artifact.provenance);
        self.ownership = DocumentOwnership::Generated;
        self.save_acknowledgement = None;
        self.validation = None;
        self.rebuild_derived();
        self.revision = next_revision;
        Ok(self.finish_receipt(before))
    }

    /// Replace the canonical generated backing. User-owned source is never
    /// overwritten; returning to generated later selects the new artifact.
    pub fn update_generated_artifact(
        &mut self,
        expected_generated_digest: ContentDigest,
        generated_artifact: GeneratedArtifact,
    ) -> Result<TransitionReceipt, DocumentError> {
        if self.generated_artifact.content_digest != expected_generated_digest {
            return Err(DocumentError::GeneratedArtifactConflict {
                expected: expected_generated_digest,
                found: self.generated_artifact.content_digest,
            });
        }
        generated_artifact.validate()?;
        if generated_artifact == self.generated_artifact {
            return Ok(self.noop_receipt());
        }
        let next_revision = self.revision.next()?;
        let before = self.receipt_before();
        let active_generated = self.ownership == DocumentOwnership::Generated;
        self.generated_artifact = generated_artifact;
        if active_generated {
            self.source.clone_from(&self.generated_artifact.source);
            self.content_digest = self.generated_artifact.content_digest;
            self.dependencies
                .clone_from(&self.generated_artifact.dependencies);
            self.provenance
                .generated
                .clone_from(&self.generated_artifact.provenance);
            self.save_acknowledgement = None;
            self.validation = None;
            self.rebuild_derived();
        }
        self.revision = next_revision;
        Ok(self.finish_receipt(before))
    }

    /// Accept a completed save only if its bytes still identify the current
    /// buffer. This prevents a slow native or browser save from clearing the
    /// dirty flag after a later edit.
    pub fn acknowledge_save(
        &mut self,
        expected_digest: ContentDigest,
        origin: SourceLocator,
    ) -> Result<TransitionReceipt, DocumentError> {
        self.ensure_current(expected_digest)?;
        if self.ownership == DocumentOwnership::Generated {
            return Err(DocumentError::GeneratedSourceIsReadOnly);
        }
        origin.validate()?;
        let acknowledgement = SaveAcknowledgement {
            origin,
            digest: self.content_digest,
        };
        if self.save_acknowledgement.as_ref() == Some(&acknowledgement) {
            return Ok(self.noop_receipt());
        }
        let next_revision = self.revision.next()?;
        let before = self.receipt_before();
        self.save_acknowledgement = Some(acknowledgement.clone());
        self.provenance.last_saved = Some(acknowledgement);
        self.revision = next_revision;
        Ok(self.finish_receipt(before))
    }

    /// Apply exact dependency resolution metadata to the current source.
    pub fn acknowledge_dependencies(
        &mut self,
        expected_digest: ContentDigest,
        dependencies: Vec<DependencyMetadata>,
    ) -> Result<TransitionReceipt, DocumentError> {
        self.ensure_current(expected_digest)?;
        let dependencies = normalize_dependencies(&self.include_directives, dependencies)?;
        if dependencies == self.dependencies {
            return Ok(self.noop_receipt());
        }
        let next_revision = self.revision.next()?;
        let before = self.receipt_before();
        self.dependencies = dependencies;
        self.validation = None;
        self.revision = next_revision;
        Ok(self.finish_receipt(before))
    }

    /// Replace one dependency's retained bytes and physical grant without
    /// changing the logical include edge. The expected document revision
    /// closes picker races even though the root source digest itself does not
    /// change when another dependency is edited or relinked.
    pub fn relink_dependency_source(
        &mut self,
        expected_revision: ObjectRevision,
        logical_identity: &str,
        locator: SourceLocator,
        source_bytes: Vec<u8>,
    ) -> Result<TransitionReceipt, DocumentError> {
        if self.revision != expected_revision {
            return Err(DocumentError::DocumentRevisionConflict {
                expected: expected_revision.get(),
                found: self.revision.get(),
            });
        }
        locator.validate()?;
        if locator.logical_identity() != logical_identity {
            return Err(DocumentError::InvalidDependency(
                "a relink cannot change the canonical logical include identity".to_owned(),
            ));
        }
        let Some(index) = self
            .dependencies
            .iter()
            .position(|dependency| dependency.locator.logical_identity() == logical_identity)
        else {
            return Err(DocumentError::InvalidDependency(format!(
                "dependency {logical_identity:?} is no longer in the canonical closure"
            )));
        };

        let mut dependencies = self.dependencies.clone();
        let source = decode_utf8(source_bytes)?;
        dependencies[index].locator = locator.clone();
        dependencies[index].resolution = DependencyResolution::Resolved {
            content_digest: digest(source.as_bytes()),
            source,
        };
        for dependency in &mut dependencies {
            if dependency
                .parent
                .as_ref()
                .is_some_and(|parent| parent.logical_identity() == logical_identity)
            {
                dependency.parent = Some(locator.clone());
            }
        }

        if self.ownership == DocumentOwnership::Generated {
            let backing = &self.generated_artifact;
            let artifact = GeneratedArtifact::try_from_utf8(
                backing.provenance().clone(),
                backing.source_bytes().to_vec(),
                dependencies,
                backing.source_map().to_vec(),
            )?;
            self.update_generated_artifact(backing.content_digest(), artifact)
        } else {
            self.acknowledge_dependencies(self.content_digest(), dependencies)
        }
    }

    /// Apply diagnostics produced for the exact current content identity.
    pub fn acknowledge_validation(
        &mut self,
        expected_digest: ContentDigest,
        diagnostics: Vec<ValidationDiagnostic>,
    ) -> Result<TransitionReceipt, DocumentError> {
        self.ensure_current(expected_digest)?;
        validate_diagnostics(&self.source, &diagnostics)?;
        let report = ValidationReport {
            content_digest: self.content_digest,
            diagnostics,
        };
        if self.validation.as_ref() == Some(&report) {
            return Ok(self.noop_receipt());
        }
        let next_revision = self.revision.next()?;
        let before = self.receipt_before();
        self.validation = Some(report);
        self.revision = next_revision;
        Ok(self.finish_receipt(before))
    }

    /// Revoke validation evidence for the exact current bytes when an
    /// out-of-buffer dependency, environment, or execution contract changes.
    pub fn invalidate_validation(
        &mut self,
        expected_digest: ContentDigest,
    ) -> Result<TransitionReceipt, DocumentError> {
        self.ensure_current(expected_digest)?;
        if self.validation.is_none() {
            return Ok(self.noop_receipt());
        }
        let next_revision = self.revision.next()?;
        let before = self.receipt_before();
        self.validation = None;
        self.revision = next_revision;
        Ok(self.finish_receipt(before))
    }

    /// Run the engine's syntax parser over the exact active source. Include
    /// resolution remains a separate dependency operation and is not claimed
    /// by this report.
    pub fn validate_syntax(&mut self) -> Result<TransitionReceipt, DocumentError> {
        let diagnostics = match rspice_core::Netlist::parse(&self.source) {
            Ok(_) => Vec::new(),
            Err(error) => {
                let line = parse_error_line(&error)
                    .filter(|line| *line > 0 && *line <= source_line_count(&self.source))
                    .unwrap_or(1);
                vec![ValidationDiagnostic::try_new(
                    DiagnosticSeverity::Error,
                    error.to_string(),
                    line,
                    1,
                )?]
            }
        };
        self.acknowledge_validation(self.content_digest, diagnostics)
    }

    fn install_unowned_source(&mut self, source: String) -> Result<(), DocumentError> {
        let include_directives = parse_include_directives(&source);
        let dependencies = normalize_dependencies(&include_directives, Vec::new())?;
        self.content_digest = digest(source.as_bytes());
        self.outline = NetlistOutline::parse(&source);
        self.source = source;
        self.include_directives = include_directives;
        self.dependencies = dependencies;
        self.save_acknowledgement = None;
        self.validation = None;
        Ok(())
    }

    fn rebuild_derived(&mut self) {
        self.outline = NetlistOutline::parse(&self.source);
        self.include_directives = parse_include_directives(&self.source);
    }

    fn ensure_current(&self, expected: ContentDigest) -> Result<(), DocumentError> {
        if self.content_digest == expected {
            Ok(())
        } else {
            Err(DocumentError::ContentConflict {
                expected,
                found: self.content_digest,
            })
        }
    }

    fn receipt_before(&self) -> (ObjectRevision, DocumentOwnership, ContentDigest) {
        (self.revision, self.ownership, self.content_digest)
    }

    fn finish_receipt(
        &self,
        before: (ObjectRevision, DocumentOwnership, ContentDigest),
    ) -> TransitionReceipt {
        TransitionReceipt {
            document_id: self.id,
            previous_revision: before.0,
            revision: self.revision,
            previous_ownership: before.1,
            ownership: self.ownership,
            previous_digest: before.2,
            content_digest: self.content_digest,
        }
    }

    fn noop_receipt(&self) -> TransitionReceipt {
        self.finish_receipt(self.receipt_before())
    }

    fn validate_invariants(&self) -> Result<(), DocumentError> {
        verify_digest("active source", self.content_digest, self.source.as_bytes())?;
        self.generated_artifact.validate()?;
        self.provenance.generated.validate()?;
        validate_dependencies(&self.include_directives, &self.dependencies, true)?;
        if let Some(validation) = &self.validation {
            if validation.content_digest != self.content_digest {
                return Err(DocumentError::InvalidPersistedDocument(
                    "validation digest does not identify the active source".to_owned(),
                ));
            }
            validate_diagnostics(&self.source, &validation.diagnostics)?;
        }
        if self.ownership == DocumentOwnership::Generated
            && (self.source != self.generated_artifact.source
                || self.dependencies != self.generated_artifact.dependencies
                || self.provenance.generated != self.generated_artifact.provenance)
        {
            return Err(DocumentError::InvalidPersistedDocument(
                "generated ownership does not match the canonical generated artifact".to_owned(),
            ));
        }
        if self.ownership == DocumentOwnership::Generated && self.save_acknowledgement.is_some() {
            return Err(DocumentError::InvalidPersistedDocument(
                "generated ownership cannot carry an active save acknowledgement".to_owned(),
            ));
        }
        if let Some(acknowledgement) = &self.save_acknowledgement
            && self.provenance.last_saved.as_ref() != Some(acknowledgement)
        {
            return Err(DocumentError::InvalidPersistedDocument(
                "active save acknowledgement is not the latest provenance save".to_owned(),
            ));
        }
        if self.ownership == DocumentOwnership::Imported {
            let Some(imported) = &self.provenance.imported else {
                return Err(DocumentError::InvalidPersistedDocument(
                    "imported ownership requires imported provenance".to_owned(),
                ));
            };
            if imported.imported_digest != self.content_digest {
                return Err(DocumentError::InvalidPersistedDocument(
                    "an unedited imported source must match its import digest".to_owned(),
                ));
            }
            if self.saved_digest() != Some(self.content_digest) {
                return Err(DocumentError::InvalidPersistedDocument(
                    "an unedited imported source must retain its persisted digest".to_owned(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Serialize)]
struct NetlistDocumentRef<'a> {
    schema_version: u32,
    id: NetlistDocumentId,
    revision: ObjectRevision,
    ownership: DocumentOwnership,
    source: &'a str,
    content_digest: ContentDigest,
    generated_artifact: &'a GeneratedArtifact,
    provenance: &'a SourceProvenance,
    save_acknowledgement: &'a Option<SaveAcknowledgement>,
    dependencies: &'a [DependencyMetadata],
    validation: &'a Option<ValidationReport>,
}

impl Serialize for NetlistDocument {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        NetlistDocumentRef {
            schema_version: DOCUMENT_SCHEMA_VERSION,
            id: self.id,
            revision: self.revision,
            ownership: self.ownership,
            source: &self.source,
            content_digest: self.content_digest,
            generated_artifact: &self.generated_artifact,
            provenance: &self.provenance,
            save_acknowledgement: &self.save_acknowledgement,
            dependencies: &self.dependencies,
            validation: &self.validation,
        }
        .serialize(serializer)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NetlistDocumentData {
    schema_version: u32,
    id: NetlistDocumentId,
    revision: ObjectRevision,
    ownership: DocumentOwnership,
    source: String,
    content_digest: ContentDigest,
    generated_artifact: GeneratedArtifact,
    provenance: SourceProvenance,
    save_acknowledgement: Option<SaveAcknowledgement>,
    dependencies: Vec<DependencyMetadata>,
    validation: Option<ValidationReport>,
}

impl<'de> Deserialize<'de> for NetlistDocument {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = NetlistDocumentData::deserialize(deserializer)?;
        if data.schema_version != DOCUMENT_SCHEMA_VERSION {
            return Err(serde::de::Error::custom(format!(
                "unsupported netlist document schema {}, expected {DOCUMENT_SCHEMA_VERSION}",
                data.schema_version
            )));
        }
        let include_directives = parse_include_directives(&data.source);
        let value = Self {
            id: data.id,
            revision: data.revision,
            ownership: data.ownership,
            outline: NetlistOutline::parse(&data.source),
            include_directives,
            source: data.source,
            content_digest: data.content_digest,
            generated_artifact: data.generated_artifact,
            provenance: data.provenance,
            save_acknowledgement: data.save_acknowledgement,
            dependencies: data.dependencies,
            validation: data.validation,
        };
        value
            .validate_invariants()
            .map_err(serde::de::Error::custom)?;
        Ok(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DocumentError {
    #[error("netlist document identity must not be the nil UUID")]
    NilDocumentIdentity,
    #[error("source is not valid UTF-8 at byte {valid_up_to} (invalid length {error_len:?})")]
    InvalidUtf8 {
        valid_up_to: usize,
        error_len: Option<usize>,
    },
    #[error("{field} must contain non-control text")]
    InvalidText { field: &'static str },
    #[error("source changed before the operation completed (expected {expected}, found {found})")]
    ContentConflict {
        expected: ContentDigest,
        found: ContentDigest,
    },
    #[error(
        "document revision changed before the operation completed (expected {expected}, found {found})"
    )]
    DocumentRevisionConflict { expected: u64, found: u64 },
    #[error(
        "generated artifact changed before regeneration completed (expected {expected}, found {found})"
    )]
    GeneratedArtifactConflict {
        expected: ContentDigest,
        found: ContentDigest,
    },
    #[error("source ownership {0:?} is not editable")]
    SourceIsNotEditable(DocumentOwnership),
    #[error("generated source is read-only; make an editable copy before saving")]
    GeneratedSourceIsReadOnly,
    #[error("invalid dependency metadata: {0}")]
    InvalidDependency(String),
    #[error("invalid validation diagnostic: {0}")]
    InvalidDiagnostic(String),
    #[error("invalid generated source map: {0}")]
    InvalidSourceMap(String),
    #[error(transparent)]
    Find(#[from] FindError),
    #[error("{scope} digest mismatch (expected {expected}, found {found})")]
    DigestMismatch {
        scope: &'static str,
        expected: ContentDigest,
        found: ContentDigest,
    },
    #[error("invalid persisted netlist document: {0}")]
    InvalidPersistedDocument(String),
    #[error(transparent)]
    Revision(#[from] RevisionError),
}

fn decode_utf8(source_bytes: Vec<u8>) -> Result<String, DocumentError> {
    String::from_utf8(source_bytes).map_err(|error| {
        let utf8_error = error.utf8_error();
        DocumentError::InvalidUtf8 {
            valid_up_to: utf8_error.valid_up_to(),
            error_len: utf8_error.error_len(),
        }
    })
}

fn digest(bytes: &[u8]) -> ContentDigest {
    ContentDigest::from_bytes(Sha256::digest(bytes).into())
}

/// Deterministic SHA-256 identity of exact UTF-8 source text.
#[must_use]
pub fn content_digest(source: &str) -> ContentDigest {
    digest(source.as_bytes())
}

fn verify_digest(
    scope: &'static str,
    expected: ContentDigest,
    bytes: &[u8],
) -> Result<(), DocumentError> {
    let found = digest(bytes);
    if found == expected {
        Ok(())
    } else {
        Err(DocumentError::DigestMismatch {
            scope,
            expected,
            found,
        })
    }
}

fn validate_nonempty_text(field: &'static str, value: &str) -> Result<(), DocumentError> {
    if value.trim().is_empty() || value.chars().any(|ch| matches!(ch, '\0' | '\r' | '\n')) {
        Err(DocumentError::InvalidText { field })
    } else {
        Ok(())
    }
}

fn validate_source_map_identity(field: &'static str, value: &str) -> Result<(), DocumentError> {
    validate_nonempty_text(field, value)
        .map_err(|error| DocumentError::InvalidSourceMap(error.to_string()))
}

fn validate_source_map(
    source: &str,
    source_map: &[GeneratedSourceMapEntry],
) -> Result<(), DocumentError> {
    let maximum_line = source.lines().count().max(1);
    let mut previous_line = None;
    for entry in source_map {
        entry.validate_identity()?;
        if entry.generated_line == 0 || entry.generated_line > maximum_line {
            return Err(DocumentError::InvalidSourceMap(format!(
                "generated line {} is outside the {maximum_line}-line source",
                entry.generated_line
            )));
        }
        if previous_line.is_some_and(|line| line >= entry.generated_line) {
            return Err(DocumentError::InvalidSourceMap(format!(
                "generated line {} is duplicated or not in canonical source order",
                entry.generated_line
            )));
        }
        previous_line = Some(entry.generated_line);
    }
    Ok(())
}

fn validate_resolution(resolution: &DependencyResolution) -> Result<(), DocumentError> {
    match resolution {
        DependencyResolution::Unresolved => {}
        DependencyResolution::Resolved {
            content_digest,
            source,
        } => {
            verify_digest("dependency member", *content_digest, source.as_bytes())?;
        }
        DependencyResolution::Missing { reason } => {
            validate_nonempty_text("missing dependency reason", reason)
                .map_err(|error| DocumentError::InvalidDependency(error.to_string()))?;
        }
    }
    Ok(())
}

fn normalize_dependencies(
    includes: &[IncludeDirective],
    mut dependencies: Vec<DependencyMetadata>,
) -> Result<Vec<DependencyMetadata>, DocumentError> {
    for (index, include) in includes.iter().enumerate() {
        if !dependencies
            .iter()
            .any(|dependency| dependency.direct_include_index == Some(index))
        {
            dependencies.push(DependencyMetadata::unresolved_direct(
                index,
                SourceLocator::exact(include.locator()),
            ));
        }
    }
    validate_dependencies(includes, &dependencies, true)?;
    Ok(dependencies)
}

fn validate_dependencies(
    includes: &[IncludeDirective],
    dependencies: &[DependencyMetadata],
    require_direct_coverage: bool,
) -> Result<(), DocumentError> {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum EdgeSlot {
        Root(usize),
        Member(String, usize),
    }

    let mut edge_slots = HashSet::new();
    let mut direct_indices = HashSet::new();
    let mut members = HashMap::<&str, (&SourceLocator, &DependencyResolution)>::new();
    for dependency in dependencies {
        validate_nonempty_text(
            "requested dependency locator",
            &dependency.requested_locator,
        )
        .map_err(|error| DocumentError::InvalidDependency(error.to_string()))?;
        dependency.locator.validate()?;
        validate_resolution(&dependency.resolution)?;
        if let Some((prior_locator, prior_resolution)) = members.insert(
            dependency.locator.logical_identity(),
            (&dependency.locator, &dependency.resolution),
        ) && (prior_locator != &dependency.locator || prior_resolution != &dependency.resolution)
        {
            return Err(DocumentError::InvalidDependency(format!(
                "logical member {:?} has ambiguous origins or source content",
                dependency.locator.logical_identity()
            )));
        }

        let edge_slot = match (
            dependency.direct_include_index,
            dependency.parent.as_ref(),
            dependency.parent_include_index,
        ) {
            (Some(index), None, None) => {
                let Some(include) = includes.get(index) else {
                    return Err(DocumentError::InvalidDependency(format!(
                        "direct include index {index} is outside the {} parsed directives",
                        includes.len()
                    )));
                };
                if dependency.requested_locator != include.locator() {
                    return Err(DocumentError::InvalidDependency(format!(
                        "requested dependency locator {:?} does not match direct include {:?}",
                        dependency.requested_locator,
                        include.locator()
                    )));
                }
                direct_indices.insert(index);
                EdgeSlot::Root(index)
            }
            (None, Some(parent), Some(parent_include_index)) => {
                parent.validate()?;
                EdgeSlot::Member(parent.logical_identity.clone(), parent_include_index)
            }
            _ => {
                return Err(DocumentError::InvalidDependency(
                    "a dependency edge must identify either one root include or one parent member include"
                        .to_owned(),
                ));
            }
        };
        if !edge_slots.insert(edge_slot) {
            return Err(DocumentError::InvalidDependency(
                "multiple members resolve the same include edge".to_owned(),
            ));
        }
    }
    if require_direct_coverage && direct_indices.len() != includes.len() {
        return Err(DocumentError::InvalidDependency(format!(
            "{} direct include directives have {} dependency records",
            includes.len(),
            direct_indices.len()
        )));
    }

    // Authenticate every transitive edge against the exact retained parent
    // member source, and require every include in every resolved member to
    // have one and only one edge.
    for dependency in dependencies {
        let (Some(parent), Some(parent_include_index)) =
            (&dependency.parent, dependency.parent_include_index)
        else {
            continue;
        };
        let Some((parent_locator, parent_resolution)) = members.get(parent.logical_identity())
        else {
            return Err(DocumentError::InvalidDependency(format!(
                "transitive edge references missing parent member {:?}",
                parent.logical_identity()
            )));
        };
        if *parent_locator != parent {
            return Err(DocumentError::InvalidDependency(format!(
                "transitive edge parent {:?} has an ambiguous source origin",
                parent.logical_identity()
            )));
        }
        let DependencyResolution::Resolved { source, .. } = parent_resolution else {
            return Err(DocumentError::InvalidDependency(format!(
                "transitive edge parent {:?} has no authenticated source bytes",
                parent.logical_identity()
            )));
        };
        let parent_includes = parse_include_directives(source);
        let Some(include) = parent_includes.get(parent_include_index) else {
            return Err(DocumentError::InvalidDependency(format!(
                "parent member {:?} include index {parent_include_index} is outside its {} parsed directives",
                parent.logical_identity(),
                parent_includes.len()
            )));
        };
        if dependency.requested_locator != include.locator() {
            return Err(DocumentError::InvalidDependency(format!(
                "transitive requested locator {:?} does not match parent include {:?}",
                dependency.requested_locator,
                include.locator()
            )));
        }
    }
    for (logical_identity, (_, resolution)) in &members {
        let DependencyResolution::Resolved { source, .. } = resolution else {
            continue;
        };
        let member_includes = parse_include_directives(source);
        for index in 0..member_includes.len() {
            if !edge_slots.contains(&EdgeSlot::Member((*logical_identity).to_owned(), index)) {
                return Err(DocumentError::InvalidDependency(format!(
                    "resolved member {logical_identity:?} include index {index} has no resolution edge"
                )));
            }
        }
    }
    reject_dependency_cycles(dependencies)?;
    Ok(())
}

fn reject_dependency_cycles(dependencies: &[DependencyMetadata]) -> Result<(), DocumentError> {
    let mut adjacency = HashMap::<&str, Vec<&str>>::new();
    for dependency in dependencies {
        if let Some(parent) = &dependency.parent {
            adjacency
                .entry(parent.logical_identity())
                .or_default()
                .push(dependency.locator.logical_identity());
        }
    }
    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();
    for node in adjacency.keys().copied().collect::<Vec<_>>() {
        if dependency_cycle_from(node, &adjacency, &mut visiting, &mut visited) {
            return Err(DocumentError::InvalidDependency(format!(
                "dependency graph contains a cycle involving {node:?}"
            )));
        }
    }
    Ok(())
}

fn dependency_cycle_from<'a>(
    node: &'a str,
    adjacency: &HashMap<&'a str, Vec<&'a str>>,
    visiting: &mut HashSet<&'a str>,
    visited: &mut HashSet<&'a str>,
) -> bool {
    if visited.contains(node) {
        return false;
    }
    if !visiting.insert(node) {
        return true;
    }
    if adjacency.get(node).is_some_and(|children| {
        children
            .iter()
            .any(|child| dependency_cycle_from(child, adjacency, visiting, visited))
    }) {
        return true;
    }
    visiting.remove(node);
    visited.insert(node);
    false
}

fn validate_diagnostics(
    source: &str,
    diagnostics: &[ValidationDiagnostic],
) -> Result<(), DocumentError> {
    let lines = source.split('\n').collect::<Vec<_>>();
    for diagnostic in diagnostics {
        validate_nonempty_text("diagnostic message", &diagnostic.message)
            .map_err(|error| DocumentError::InvalidDiagnostic(error.to_string()))?;
        if let Some(code) = &diagnostic.code {
            validate_nonempty_text("diagnostic code", code)
                .map_err(|error| DocumentError::InvalidDiagnostic(error.to_string()))?;
        }
        let line = diagnostic.position.line;
        let column = diagnostic.position.column;
        let Some(source_line) = line.checked_sub(1).and_then(|index| lines.get(index)) else {
            return Err(DocumentError::InvalidDiagnostic(format!(
                "line {line} is outside the {}-line source",
                lines.len()
            )));
        };
        let source_line = source_line.strip_suffix('\r').unwrap_or(source_line);
        let max_column = source_line.chars().count() + 1;
        if column == 0 || column > max_column {
            return Err(DocumentError::InvalidDiagnostic(format!(
                "column {column} is outside line {line} (maximum {max_column})"
            )));
        }
    }
    Ok(())
}

fn source_line_count(source: &str) -> usize {
    source.bytes().filter(|byte| *byte == b'\n').count() + 1
}

fn parse_error_line(error: &rspice_core::netlist::ParseError) -> Option<usize> {
    use rspice_core::netlist::{DeviceInitialConditionError, ParseError};

    match error {
        ParseError::Syntax { line, .. } => Some(*line),
        ParseError::DuplicateName { duplicate_line, .. } => Some(*duplicate_line),
        ParseError::MissingSubcircuitEnds(error) => Some(error.opened_at.line),
        ParseError::UndefinedMutualInductorReference(error) => Some(error.origin.line),
        ParseError::DeviceInitialCondition(error) => Some(match error.as_ref() {
            DeviceInitialConditionError::DuplicateDirective { duplicate, .. } => duplicate.line,
            DeviceInitialConditionError::MissingInformation { origin }
            | DeviceInitialConditionError::MalformedDirective { origin, .. }
            | DeviceInitialConditionError::SourceUnavailable { origin, .. }
            | DeviceInitialConditionError::MalformedSource { origin, .. }
            | DeviceInitialConditionError::NonFiniteValue { origin, .. }
            | DeviceInitialConditionError::UnresolvedSource { origin, .. }
            | DeviceInitialConditionError::InvalidArity { origin, .. }
            | DeviceInitialConditionError::UnsupportedTarget { origin, .. } => origin.line,
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests;
