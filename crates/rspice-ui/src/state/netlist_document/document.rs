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
mod tests {
    use serde_json::Value;

    use super::*;

    const GENERATED: &str = "RSpice generated deck\r\n.include \"models/core.lib\"\r\nV1 in 0 1\r\nR1 in 0 1k\r\n.op\r\n.end\r\n";

    fn content_digest(bytes: &[u8]) -> ContentDigest {
        digest(bytes)
    }

    fn locator(value: &str) -> SourceLocator {
        SourceLocator::try_new(value, value).expect("valid locator")
    }

    fn generated(source: &str, input_marker: &[u8]) -> GeneratedArtifact {
        let provenance = GeneratedProvenance::try_new(
            "rspice-netlist-generator-v1",
            GenerationInput::new(ObjectRevision::INITIAL, content_digest(input_marker)),
        )
        .expect("valid provenance");
        GeneratedArtifact::try_from_utf8(
            provenance,
            source.as_bytes().to_vec(),
            Vec::new(),
            vec![
                GeneratedSourceMapEntry::try_new(1, "user/top", "schematic", None, None)
                    .expect("source map"),
            ],
        )
        .expect("valid generated artifact")
    }

    fn document() -> NetlistDocument {
        let id = NetlistDocumentId::try_from_uuid(
            Uuid::parse_str("c6ec5125-95dc-4a0c-93cc-24f5fef1d820").expect("UUID"),
        )
        .expect("non-nil");
        NetlistDocument::from_generated(id, generated(GENERATED, b"input-a"))
            .expect("valid document")
    }

    #[test]
    fn generated_document_preserves_exact_utf8_and_crlf_bytes() {
        let document = document();
        assert_eq!(document.source_bytes(), GENERATED.as_bytes());
        assert_eq!(
            document.content_digest(),
            content_digest(GENERATED.as_bytes())
        );
        assert_eq!(document.ownership(), DocumentOwnership::Generated);
        assert!(!document.is_dirty());
        assert_eq!(document.include_directives().len(), 1);
        assert_eq!(document.dependencies().len(), 1);
        assert!(matches!(
            document.dependencies()[0].resolution(),
            DependencyResolution::Unresolved
        ));
    }

    #[test]
    fn invalid_utf8_never_partially_mutates_a_document() {
        let mut document = document();
        let before = document.clone();
        let error = document
            .import_source(
                document.content_digest(),
                locator("bad.cir"),
                vec![b'R', 0xff],
            )
            .expect_err("invalid UTF-8");
        assert!(matches!(error, DocumentError::InvalidUtf8 { .. }));
        assert_eq!(document, before);
    }

    #[test]
    fn imported_origin_survives_make_editable_edit_and_save_as() {
        let mut document = document();
        let imported = b"Imported deck\nR9 x 0 9k\n.end\n".to_vec();
        document
            .import_source(
                document.content_digest(),
                locator("imports/a.cir"),
                imported,
            )
            .expect("import");
        assert_eq!(document.ownership(), DocumentOwnership::Imported);
        assert!(!document.is_dirty());
        let import_origin = document
            .provenance()
            .imported()
            .expect("import provenance")
            .clone();

        document
            .make_editable(document.content_digest())
            .expect("make editable");
        document
            .replace_editable_source(
                document.content_digest(),
                b"Imported deck\nR9 x 0 10k\n.end\n".to_vec(),
            )
            .expect("edit");
        assert!(document.is_dirty());
        document
            .acknowledge_save(document.content_digest(), locator("saved/b.cir"))
            .expect("save as");

        assert_eq!(document.provenance().imported(), Some(&import_origin));
        assert_eq!(
            document
                .save_acknowledgement()
                .expect("save acknowledgement")
                .origin()
                .locator(),
            "saved/b.cir"
        );
        assert!(!document.is_dirty());
    }

    #[test]
    fn stale_edit_save_validation_and_dependency_results_are_rejected() {
        let mut document = document();
        let generated_digest = document.content_digest();
        document.make_editable(generated_digest).expect("editable");
        document
            .replace_editable_source(
                generated_digest,
                b"edited\n.include new.lib\nR1 1 0 2k\n.end\n".to_vec(),
            )
            .expect("edit");
        let after_edit = document.clone();

        assert!(matches!(
            document.acknowledge_save(generated_digest, locator("stale.cir")),
            Err(DocumentError::ContentConflict { .. })
        ));
        assert!(matches!(
            document.acknowledge_validation(generated_digest, Vec::new()),
            Err(DocumentError::ContentConflict { .. })
        ));
        assert!(matches!(
            document.acknowledge_dependencies(generated_digest, Vec::new()),
            Err(DocumentError::ContentConflict { .. })
        ));
        assert_eq!(document, after_edit);
    }

    #[test]
    fn generated_refresh_never_overwrites_user_owned_source() {
        let mut document = document();
        document
            .make_editable(document.content_digest())
            .expect("editable");
        let owned_source = "owned\nR7 n 0 7k\n.end\n";
        document
            .replace_editable_source(document.content_digest(), owned_source.as_bytes().to_vec())
            .expect("edit");
        let replacement = generated("new generated\nR2 n 0 2k\n.end\n", b"input-b");
        let replacement_digest = replacement.content_digest();
        document
            .update_generated_artifact(document.generated_artifact().content_digest(), replacement)
            .expect("refresh backing");

        assert_eq!(document.source(), owned_source);
        assert_eq!(document.ownership(), DocumentOwnership::Editable);
        assert_eq!(
            document.generated_artifact().content_digest(),
            replacement_digest
        );
        document
            .return_to_generated(document.content_digest())
            .expect("return generated");
        assert_eq!(document.content_digest(), replacement_digest);
        assert_eq!(document.ownership(), DocumentOwnership::Generated);
    }

    #[test]
    fn editable_copy_is_a_distinct_unsaved_document_and_primary_is_unchanged() {
        let primary = document();
        let primary_before = primary.clone();
        let owned_id = NetlistDocumentId::try_from_uuid(
            Uuid::parse_str("9702bcf0-e52b-4eb5-bdc1-1421d11205c1").expect("UUID"),
        )
        .expect("identity");
        let owned = primary
            .create_editable_copy(owned_id, primary.content_digest())
            .expect("editable copy");

        assert_eq!(primary, primary_before);
        assert_eq!(owned.id(), owned_id);
        assert_ne!(owned.id(), primary.id());
        assert_eq!(owned.ownership(), DocumentOwnership::Editable);
        assert_eq!(owned.source_bytes(), primary.source_bytes());
        assert_eq!(
            owned.generated_artifact().content_digest(),
            primary.generated_artifact().content_digest()
        );
        assert!(owned.is_dirty());
        assert!(owned.validation().is_none());
    }

    #[test]
    fn stale_generated_refresh_is_transactional() {
        let mut document = document();
        let before = document.clone();
        let error = document
            .update_generated_artifact(
                content_digest(b"stale"),
                generated("new\n.end\n", b"input-b"),
            )
            .expect_err("conflict");
        assert!(matches!(
            error,
            DocumentError::GeneratedArtifactConflict { .. }
        ));
        assert_eq!(document, before);
    }

    #[test]
    fn source_changes_invalidate_validation_and_dependency_resolution() {
        let mut document = document();
        let resolved = document.dependencies()[0]
            .clone()
            .resolve_utf8(b"model member\n.model core r r=1k\n".to_vec())
            .expect("resolved dependency");
        document
            .acknowledge_dependencies(document.content_digest(), vec![resolved])
            .expect("dependency acknowledgement");
        document.validate_syntax().expect("syntax validation");
        assert!(document.validation().is_some());

        document
            .make_editable(document.content_digest())
            .expect("editable");
        document
            .replace_editable_source(
                document.content_digest(),
                b"changed\n.include other.lib\nR1 1 0 1k\n.end\n".to_vec(),
            )
            .expect("edit");
        assert!(document.validation().is_none());
        assert!(matches!(
            document.dependencies()[0].resolution(),
            DependencyResolution::Unresolved
        ));
        assert_eq!(document.dependencies()[0].locator().locator(), "other.lib");
    }

    #[test]
    fn syntax_validation_is_bound_to_exact_content() {
        let mut document = document();
        document
            .make_editable(document.content_digest())
            .expect("editable");
        document
            .replace_editable_source(
                document.content_digest(),
                b"broken\nR1 only-one-node\n.end\n".to_vec(),
            )
            .expect("edit");
        document.validate_syntax().expect("validation transaction");
        let report = document.validation().expect("report");
        assert_eq!(report.content_digest(), document.content_digest());
        assert!(!report.is_valid());
        assert_eq!(report.error_count(), 1);
        assert!(report.diagnostics()[0].position().line() >= 1);
    }

    #[test]
    fn diagnostic_positions_are_unicode_scalar_columns_and_checked() {
        let mut document = document();
        document
            .make_editable(document.content_digest())
            .expect("editable");
        document
            .replace_editable_source(
                document.content_digest(),
                "unicode\nRμ μ 0 1k\n.end\n".as_bytes().to_vec(),
            )
            .expect("edit");
        let diagnostic =
            ValidationDiagnostic::try_new(DiagnosticSeverity::Warning, "review μ", 2, 4)
                .expect("diagnostic");
        document
            .acknowledge_validation(document.content_digest(), vec![diagnostic])
            .expect("valid position");
        let before = document.clone();
        assert!(matches!(
            document.acknowledge_validation(
                document.content_digest(),
                vec![
                    ValidationDiagnostic::try_new(DiagnosticSeverity::Error, "outside", 2, 99)
                        .expect("shape")
                ]
            ),
            Err(DocumentError::InvalidDiagnostic(_))
        ));
        assert_eq!(document, before);
    }

    #[test]
    fn dependency_metadata_requires_exact_direct_relationships() {
        let mut document = document();
        let wrong = DependencyMetadata::unresolved_direct(0, locator("models/wrong.lib"));
        let before = document.clone();
        assert!(matches!(
            document.acknowledge_dependencies(document.content_digest(), vec![wrong]),
            Err(DocumentError::InvalidDependency(_))
        ));
        assert_eq!(document, before);

        let parent = locator("models/core.lib");
        let resolved_parent = document.dependencies()[0]
            .clone()
            .resolve_utf8(b"core models\n.include models/transistors.lib\n".to_vec())
            .expect("resolved parent");
        let child =
            DependencyMetadata::unresolved_transitive(parent, 0, locator("models/transistors.lib"))
                .with_resolution(DependencyResolution::Missing {
                    reason: "not present in sealed source bundle".to_owned(),
                })
                .expect("missing dependency");
        document
            .acknowledge_validation(document.content_digest(), Vec::new())
            .expect("validation receipt");
        assert!(document.validation().is_some());
        document
            .acknowledge_dependencies(document.content_digest(), vec![resolved_parent, child])
            .expect("valid transitive metadata");
        assert_eq!(document.dependencies().len(), 2);
        assert!(
            document.validation().is_none(),
            "dependency identity changes must revoke source validation"
        );
    }

    #[test]
    fn sealed_dependency_graph_retains_exact_member_bytes_and_edges() {
        let root_source = "sealed root\n.include a.lib\n.end\n";
        let a_source = b"a member\r\n.include b.lib\r\n.model a r r=1k\r\n";
        let b_source = "b member μ\n.model b r r=2k\n".as_bytes();
        let a = DependencyMetadata::unresolved_direct(0, locator("a.lib"))
            .resolve_utf8(a_source.to_vec())
            .expect("resolved a");
        let b = DependencyMetadata::unresolved_transitive(locator("a.lib"), 0, locator("b.lib"))
            .resolve_utf8(b_source.to_vec())
            .expect("resolved b");
        let artifact = GeneratedArtifact::try_from_utf8(
            GeneratedProvenance::try_new(
                "generator",
                GenerationInput::new(ObjectRevision::INITIAL, content_digest(b"inputs")),
            )
            .expect("provenance"),
            root_source.as_bytes().to_vec(),
            vec![a, b],
            Vec::new(),
        )
        .expect("sealed artifact");

        assert!(artifact.dependency_graph_is_sealed());
        assert_eq!(
            artifact.dependencies()[0].source_bytes(),
            Some(a_source.as_slice())
        );
        assert_eq!(artifact.dependencies()[1].source_bytes(), Some(b_source));
        assert_eq!(artifact.dependencies()[1].parent_include_index(), Some(0));
        assert_eq!(
            artifact.dependencies()[0].resolution().byte_length(),
            Some(a_source.len())
        );
    }

    #[test]
    fn resolution_edges_separate_requested_paths_from_portable_member_identity() {
        let root_source = "root\n.include ../vendor/a.lib\n.end\n";
        let a_locator = SourceLocator::try_new("vendor/a.lib", "a.lib")
            .expect("logical member")
            .with_native_origin(r"C:\cache\vendor\a.lib")
            .expect("native origin");
        let b_locator = SourceLocator::try_new("shared/b.lib", "b.lib").expect("logical member");
        let a_source = b"a\n.include ../shared/b.lib\n";
        let a = DependencyMetadata::unresolved_direct_to(0, "../vendor/a.lib", a_locator.clone())
            .expect("direct edge")
            .resolve_utf8(a_source.to_vec())
            .expect("resolved a");
        let b = DependencyMetadata::unresolved_transitive_to(
            a_locator,
            0,
            "../shared/b.lib",
            b_locator,
        )
        .expect("transitive edge")
        .resolve_utf8(b"b\n.model b r r=1k\n".to_vec())
        .expect("resolved b");
        let artifact = GeneratedArtifact::try_from_utf8(
            GeneratedProvenance::try_new(
                "generator",
                GenerationInput::new(ObjectRevision::INITIAL, content_digest(b"inputs")),
            )
            .expect("provenance"),
            root_source.as_bytes().to_vec(),
            vec![a, b],
            Vec::new(),
        )
        .expect("resolved graph");

        assert_eq!(
            artifact.dependencies()[0].requested_locator(),
            "../vendor/a.lib"
        );
        assert_eq!(
            artifact.dependencies()[0].locator().logical_identity(),
            "vendor/a.lib"
        );
        assert_eq!(
            artifact.dependencies()[0].locator().native_origin(),
            Some(r"C:\cache\vendor\a.lib")
        );
        assert_eq!(
            artifact.dependencies()[1].requested_locator(),
            "../shared/b.lib"
        );
    }

    #[test]
    fn generated_source_map_is_canonical_and_rejects_duplicate_or_out_of_range_lines() {
        let provenance = || {
            GeneratedProvenance::try_new(
                "generator",
                GenerationInput::new(ObjectRevision::INITIAL, content_digest(b"inputs")),
            )
            .expect("provenance")
        };
        let line_two = GeneratedSourceMapEntry::try_new(
            2,
            "user/top",
            "schematic",
            Some("XAMP".to_owned()),
            Some("RLOAD".to_owned()),
        )
        .expect("mapping");
        let line_one = GeneratedSourceMapEntry::try_new(1, "user/top", "schematic", None, None)
            .expect("mapping");
        let artifact = GeneratedArtifact::try_from_utf8(
            provenance(),
            b"mapped\nR1 in 0 1k\n.end\n".to_vec(),
            Vec::new(),
            vec![line_two.clone(), line_one],
        )
        .expect("canonical source map");
        assert_eq!(
            artifact
                .source_map()
                .iter()
                .map(GeneratedSourceMapEntry::generated_line)
                .collect::<Vec<_>>(),
            vec![1, 2]
        );
        let mapped = artifact.source_map_entry(2).expect("line mapping");
        assert_eq!(mapped.cell_identity(), "user/top");
        assert_eq!(mapped.view_identity(), "schematic");
        assert_eq!(mapped.instance_identity(), Some("XAMP"));
        assert_eq!(mapped.component_identity(), Some("RLOAD"));

        let duplicate = GeneratedArtifact::try_from_utf8(
            provenance(),
            b"mapped\nR1 in 0 1k\n.end\n".to_vec(),
            Vec::new(),
            vec![line_two.clone(), line_two],
        );
        assert!(matches!(duplicate, Err(DocumentError::InvalidSourceMap(_))));
        let outside = GeneratedArtifact::try_from_utf8(
            provenance(),
            b"one line\n".to_vec(),
            Vec::new(),
            vec![
                GeneratedSourceMapEntry::try_new(2, "user/top", "schematic", None, None)
                    .expect("entry shape"),
            ],
        );
        assert!(matches!(outside, Err(DocumentError::InvalidSourceMap(_))));
        assert!(GeneratedSourceMapEntry::try_new(
            1,
            "user/top",
            "schematic",
            Some(String::new()),
            None,
        )
        .is_err());
    }

    #[test]
    fn dependency_graph_rejects_missing_ambiguous_and_cyclic_edges() {
        let root_source = "root\n.include a.lib\n.end\n";
        let provenance = || {
            GeneratedProvenance::try_new(
                "generator",
                GenerationInput::new(ObjectRevision::INITIAL, content_digest(b"inputs")),
            )
            .expect("provenance")
        };
        let a_source = b"a\n.include b.lib\n";
        let a = DependencyMetadata::unresolved_direct(0, locator("a.lib"))
            .resolve_utf8(a_source.to_vec())
            .expect("resolved a");

        let missing = GeneratedArtifact::try_from_utf8(
            provenance(),
            root_source.as_bytes().to_vec(),
            vec![a.clone()],
            Vec::new(),
        );
        assert!(matches!(missing, Err(DocumentError::InvalidDependency(_))));

        let ambiguous = GeneratedArtifact::try_from_utf8(
            provenance(),
            root_source.as_bytes().to_vec(),
            vec![a.clone(), a.clone()],
            Vec::new(),
        );
        assert!(matches!(
            ambiguous,
            Err(DocumentError::InvalidDependency(_))
        ));

        let b_source = b"b\n.include a.lib\n";
        let b = DependencyMetadata::unresolved_transitive(locator("a.lib"), 0, locator("b.lib"))
            .resolve_utf8(b_source.to_vec())
            .expect("resolved b");
        let back_to_a =
            DependencyMetadata::unresolved_transitive(locator("b.lib"), 0, locator("a.lib"))
                .resolve_utf8(a_source.to_vec())
                .expect("resolved a again");
        let cyclic = GeneratedArtifact::try_from_utf8(
            provenance(),
            root_source.as_bytes().to_vec(),
            vec![a, b, back_to_a],
            Vec::new(),
        );
        assert!(matches!(cyclic, Err(DocumentError::InvalidDependency(_))));
    }

    #[test]
    fn revision_and_identity_are_stable_and_noops_do_not_create_revisions() {
        let mut document = document();
        let id = document.id();
        let initial_revision = document.revision();
        let receipt = document
            .return_to_generated(document.content_digest())
            .expect("no-op return");
        assert_eq!(receipt.previous_revision(), initial_revision);
        assert_eq!(receipt.revision(), initial_revision);
        assert_eq!(document.revision(), initial_revision);

        let receipt = document
            .make_editable(document.content_digest())
            .expect("transition");
        assert_eq!(receipt.document_id(), id);
        assert_eq!(document.id(), id);
        assert_eq!(receipt.previous_ownership(), DocumentOwnership::Generated);
        assert_eq!(receipt.ownership(), DocumentOwnership::Editable);
        assert!(receipt.revision() > receipt.previous_revision());
    }

    #[test]
    fn serde_round_trip_rebuilds_derived_navigation_without_losing_state() {
        let mut document = document();
        document
            .import_source(
                document.content_digest(),
                locator("imports/unicode.cir"),
                "Imported μ\n.include μ.lib\nRμ μ 0 1k\n.end\n"
                    .as_bytes()
                    .to_vec(),
            )
            .expect("import");
        document
            .make_editable(document.content_digest())
            .expect("editable");
        let json = serde_json::to_string(&document).expect("serialize");
        let restored: NetlistDocument = serde_json::from_str(&json).expect("restore");

        assert_eq!(restored, document);
        assert_eq!(restored.outline().entries(), document.outline().entries());
        assert_eq!(restored.include_directives(), document.include_directives());
        assert_eq!(restored.source_bytes(), document.source_bytes());
    }

    #[test]
    fn deserialization_rejects_tampered_active_and_generated_bytes() {
        let document = document();
        let mut value = serde_json::to_value(&document).expect("serialize");
        value["source"] = Value::String("tampered\n.end\n".to_owned());
        assert!(serde_json::from_value::<NetlistDocument>(value).is_err());

        let mut value = serde_json::to_value(&document).expect("serialize");
        value["generated_artifact"]["source"] =
            Value::String("tampered generated\n.end\n".to_owned());
        assert!(serde_json::from_value::<NetlistDocument>(value).is_err());
    }

    #[test]
    fn deserialization_rejects_tampered_dependency_member_bytes() {
        let root_source = "root\n.include a.lib\n.end\n";
        let a = DependencyMetadata::unresolved_direct(0, locator("a.lib"))
            .resolve_utf8(b"member\n.model a r r=1k\n".to_vec())
            .expect("resolved member");
        let artifact = GeneratedArtifact::try_from_utf8(
            GeneratedProvenance::try_new(
                "generator",
                GenerationInput::new(ObjectRevision::INITIAL, content_digest(b"inputs")),
            )
            .expect("provenance"),
            root_source.as_bytes().to_vec(),
            vec![a],
            Vec::new(),
        )
        .expect("artifact");
        let document =
            NetlistDocument::from_generated(NetlistDocumentId::new(), artifact).expect("document");
        let mut value = serde_json::to_value(document).expect("serialize");
        value["dependencies"][0]["resolution"]["source"] =
            Value::String("tampered member\n".to_owned());
        assert!(serde_json::from_value::<NetlistDocument>(value).is_err());
    }

    #[test]
    fn deserialization_rejects_tampered_generated_source_map() {
        let document = document();
        let mut value = serde_json::to_value(document).expect("serialize");
        value["generated_artifact"]["source_map"][0]["generated_line"] = Value::from(999);
        assert!(serde_json::from_value::<NetlistDocument>(value).is_err());
    }

    #[test]
    fn deserialization_rejects_future_schema_and_stale_validation() {
        let mut document = document();
        document.validate_syntax().expect("validate");
        let mut value = serde_json::to_value(&document).expect("serialize");
        value["schema_version"] = Value::from(99);
        assert!(serde_json::from_value::<NetlistDocument>(value).is_err());

        let mut value = serde_json::to_value(&document).expect("serialize");
        value["validation"]["content_digest"] = Value::String(content_digest(b"other").to_string());
        assert!(serde_json::from_value::<NetlistDocument>(value).is_err());
    }

    #[test]
    fn generated_source_cannot_be_saved_or_edited_directly() {
        let mut document = document();
        assert_eq!(
            document
                .acknowledge_save(document.content_digest(), locator("out.cir"))
                .expect_err("read only"),
            DocumentError::GeneratedSourceIsReadOnly
        );
        assert!(matches!(
            document.replace_editable_source(document.content_digest(), b"replacement".to_vec()),
            Err(DocumentError::SourceIsNotEditable(
                DocumentOwnership::Generated
            ))
        ));
        assert!(matches!(
            document.replace_editable_matches(
                document.content_digest(),
                "R1",
                "R2",
                FindOptions::default(),
                ReplaceScope::All,
            ),
            Err(DocumentError::SourceIsNotEditable(
                DocumentOwnership::Generated
            ))
        ));
    }

    #[test]
    fn editable_regex_replacement_is_one_atomic_source_transition() {
        let mut document = document();
        document
            .make_editable(document.content_digest())
            .expect("editable");
        let before_revision = document.revision();
        let before_digest = document.content_digest();
        let (receipt, outcome) = document
            .replace_editable_matches(
                before_digest,
                r"R(?P<number>\d+)",
                "X${number}",
                FindOptions {
                    regular_expression: true,
                    ..FindOptions::default()
                },
                ReplaceScope::All,
            )
            .expect("replace");
        assert_eq!(outcome.replacement_count(), 1);
        assert!(document.source().contains("X1 in 0 1k"));
        assert_eq!(receipt.previous_revision(), before_revision);
        assert_eq!(receipt.previous_digest(), before_digest);
        assert_eq!(receipt.content_digest(), document.content_digest());
        assert!(receipt.revision() > before_revision);
        assert!(document.validation().is_none());
    }

    #[test]
    fn nil_identity_and_invalid_locator_are_rejected() {
        assert_eq!(
            NetlistDocumentId::try_from_uuid(Uuid::nil()),
            Err(DocumentError::NilDocumentIdentity)
        );
        assert!(SourceLocator::try_new("\n", "bad").is_err());
        assert!(SourceLocator::try_new("valid", "\0").is_err());
    }

    #[test]
    fn portable_logical_identity_is_separate_from_optional_native_origin() {
        let locator = SourceLocator::try_new("models/vendor/core.lib", "core.lib")
            .expect("portable identity")
            .with_native_origin(r"C:\foundry\models\core.lib")
            .expect("native origin");
        assert_eq!(locator.logical_identity(), "models/vendor/core.lib");
        assert_eq!(locator.display_name(), "core.lib");
        assert_eq!(locator.native_origin(), Some(r"C:\foundry\models\core.lib"));
        let json = serde_json::to_string(&locator).expect("serialize");
        assert_eq!(
            serde_json::from_str::<SourceLocator>(&json).expect("restore"),
            locator
        );
    }
}
