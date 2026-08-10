//! Project-owned executable source bundles.
//!
//! A source bundle retains the exact UTF-8 bytes and dependency graph used by
//! compilation. Logical paths are portable project identities rather than host
//! filesystem paths, so the same sealed closure is valid on native and WebAssembly
//! targets. The registry also provides a deterministic migration for the original
//! one-document Code Workspace representation.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};
use sha2::{Digest as _, Sha256};
use unicode_normalization::UnicodeNormalization;
use uuid::Uuid;

use crate::product::{ContentDigest, ObjectRevision, ProjectId};

use super::workspace::CellViewRef;

/// Persisted schema for [`ProjectSourceRegistry`].
pub const PROJECT_SOURCE_REGISTRY_SCHEMA_VERSION: u16 = 2;
/// Maximum exact UTF-8 payload accepted for one retained source file.
pub const MAX_PROJECT_CODE_SOURCE_BYTES: usize = 16 * 1024 * 1024;
/// Maximum exact UTF-8 payload accepted for a complete retained source closure.
pub const MAX_PROJECT_SOURCE_BUNDLE_BYTES: usize = 64 * 1024 * 1024;
/// Maximum number of files, including the root, in one retained closure.
pub const MAX_PROJECT_SOURCE_FILES: usize = 10_000;
/// Maximum authenticated dependency edges in one retained closure.
pub const MAX_PROJECT_SOURCE_DEPENDENCIES: usize = 16_384;
/// Maximum dependency depth accepted before compilation.
pub const MAX_PROJECT_SOURCE_DEPENDENCY_DEPTH: usize = 256;
/// Maximum UTF-8 length of one portable logical path.
pub const MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES: usize = 1024;
/// Maximum immutable Verilog-A qualification attempts retained per bundle.
pub const MAX_PROJECT_SOURCE_QUALIFICATION_RECORDS: usize = 100_000;

const LEGACY_PROJECT_SOURCE_ID_NAMESPACE: Uuid =
    Uuid::from_u128(0xb50a_3aed_047f_54f8_971b_a2aa_e1e8_4f5b);
const PROJECT_SOURCE_DOCUMENT_ID_NAMESPACE: Uuid =
    Uuid::from_u128(0x9f22_95dc_feb3_55a0_99c4_276e_7015_a08f);
const CLOSURE_DIGEST_DOMAIN: &[u8] = b"rspice.project-source-closure/v2";

/// Stable identity of one project-owned source bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct ProjectSourceId(Uuid);

impl ProjectSourceId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn try_from_uuid(value: Uuid) -> Result<Self, ProjectSourceIdError> {
        (!value.is_nil())
            .then_some(Self(value))
            .ok_or(ProjectSourceIdError::Nil)
    }

    #[must_use]
    pub const fn as_uuid(self) -> Uuid {
        self.0
    }

    fn migrated(document: &ProjectSourceDocument) -> Self {
        let mut material = Vec::new();
        append_frame(&mut material, b"rspice.project-source-legacy-id/v1");
        append_frame(&mut material, document.language.stable_name().as_bytes());
        append_frame(&mut material, document.file_name.as_bytes());
        append_frame(&mut material, document.content.as_bytes());
        append_frame(&mut material, &document.revision.get().to_be_bytes());
        Self(Uuid::new_v5(&LEGACY_PROJECT_SOURCE_ID_NAMESPACE, &material))
    }
}

impl Default for ProjectSourceId {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProjectSourceId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Uuid::deserialize(deserializer)?;
        Self::try_from_uuid(value).map_err(D::Error::custom)
    }
}

impl fmt::Display for ProjectSourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl FromStr for ProjectSourceId {
    type Err = ProjectSourceIdParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::try_from_uuid(Uuid::parse_str(value)?)?)
    }
}

/// Stable identity of one document inside a project source bundle.
///
/// Paths can change while this identity remains stable, allowing diagnostics,
/// breakpoints, editor history, and runtime events to survive a rename without
/// being rebound to a different file by display text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ProjectSourceDocumentId(Uuid);

impl ProjectSourceDocumentId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn try_from_uuid(value: Uuid) -> Result<Self, ProjectSourceDocumentIdError> {
        (!value.is_nil())
            .then_some(Self(value))
            .ok_or(ProjectSourceDocumentIdError::Nil)
    }

    #[must_use]
    pub const fn as_uuid(self) -> Uuid {
        self.0
    }

    fn migrated(bundle_id: ProjectSourceId, logical_path: &str) -> Self {
        let mut material = Vec::new();
        append_frame(&mut material, bundle_id.as_uuid().as_bytes());
        append_frame(&mut material, path_key(logical_path).as_bytes());
        Self(Uuid::new_v5(
            &PROJECT_SOURCE_DOCUMENT_ID_NAMESPACE,
            &material,
        ))
    }

    const fn missing_for_migration() -> Self {
        Self(Uuid::nil())
    }

    const fn is_missing(self) -> bool {
        self.0.is_nil()
    }
}

impl Default for ProjectSourceDocumentId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ProjectSourceDocumentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ProjectSourceDocumentIdError {
    #[error("project source document identity must not be the nil UUID")]
    Nil,
}

fn missing_project_source_document_id() -> ProjectSourceDocumentId {
    ProjectSourceDocumentId::missing_for_migration()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ProjectSourceIdError {
    #[error("project source identity must not be the nil UUID")]
    Nil,
}

#[derive(Debug, thiserror::Error)]
pub enum ProjectSourceIdParseError {
    #[error(transparent)]
    Parse(#[from] uuid::Error),
    #[error(transparent)]
    Invalid(#[from] ProjectSourceIdError),
}

/// Stable language identity for a project-owned executable source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProjectSourceLanguage {
    VerilogA,
    #[serde(rename = "rspice-automation")]
    RSpiceAutomation,
}

impl ProjectSourceLanguage {
    pub const ALL: [Self; 2] = [Self::VerilogA, Self::RSpiceAutomation];

    pub const fn label(self) -> &'static str {
        match self {
            Self::VerilogA => "Verilog-A",
            Self::RSpiceAutomation => "RSpice Automation",
        }
    }

    pub const fn required_extension(self) -> &'static str {
        match self {
            Self::VerilogA => ".va",
            Self::RSpiceAutomation => ".py",
        }
    }

    const fn stable_name(self) -> &'static str {
        match self {
            Self::VerilogA => "verilog-a",
            Self::RSpiceAutomation => "rspice-automation",
        }
    }
}

impl fmt::Display for ProjectSourceLanguage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.label())
    }
}

/// Product owner of a source bundle. Code Workspace owners preserve the
/// singleton editor contract; cell-view owners make hierarchy-bound behavioral
/// sources first-class project artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum ProjectSourceOwner {
    CodeWorkspace { language: ProjectSourceLanguage },
    CellView { reference: CellViewRef },
}

/// Locale-independent, Unicode-aware identity for one Library/Cell/View owner.
///
/// Display spelling remains untouched everywhere else. This key exists only
/// for identity comparisons, so canonically equivalent spellings and case
/// variants cannot create owners that are visually indistinguishable but
/// operationally different.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct CanonicalCellViewOwnerKey {
    library: String,
    cell: String,
    view: String,
}

impl fmt::Display for CanonicalCellViewOwnerKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}/{}/{}", self.library, self.cell, self.view)
    }
}

/// Build the sole canonical identity used for Library/Cell/View ownership.
///
/// Uppercasing before lowercasing provides locale-independent expansion for
/// characters such as German sharp-s. NFC on both sides also makes composed
/// and decomposed accented names identical without changing stored spelling.
pub(crate) fn canonical_cell_view_owner_key(
    library: &str,
    cell: &str,
    view: &str,
) -> CanonicalCellViewOwnerKey {
    CanonicalCellViewOwnerKey {
        library: canonical_owner_segment(library),
        cell: canonical_owner_segment(cell),
        view: canonical_owner_segment(view),
    }
}

fn canonical_owner_segment(value: &str) -> String {
    value
        .nfc()
        .flat_map(char::to_uppercase)
        .flat_map(char::to_lowercase)
        .collect::<String>()
        .nfc()
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CanonicalProjectSourceOwnerKey {
    CodeWorkspace(ProjectSourceLanguage),
    CellView(CanonicalCellViewOwnerKey),
}

impl fmt::Display for CanonicalProjectSourceOwnerKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CodeWorkspace(language) => {
                write!(formatter, "code-workspace:{}", language.stable_name())
            }
            Self::CellView(reference) => write!(formatter, "cell-view:{reference}"),
        }
    }
}

impl ProjectSourceOwner {
    #[must_use]
    pub const fn code_workspace(language: ProjectSourceLanguage) -> Self {
        Self::CodeWorkspace { language }
    }

    #[must_use]
    pub fn cell_view(reference: CellViewRef) -> Self {
        Self::CellView { reference }
    }

    fn canonical_key(&self) -> CanonicalProjectSourceOwnerKey {
        match self {
            Self::CodeWorkspace { language } => {
                CanonicalProjectSourceOwnerKey::CodeWorkspace(*language)
            }
            Self::CellView { reference } => CanonicalProjectSourceOwnerKey::CellView(
                canonical_cell_view_owner_key(&reference.library, &reference.cell, &reference.view),
            ),
        }
    }

    fn validate(&self, language: ProjectSourceLanguage) -> Result<(), ProjectSourceError> {
        match self {
            Self::CodeWorkspace {
                language: owner_language,
            } if *owner_language != language => Err(ProjectSourceError::OwnerLanguageMismatch {
                owner: *owner_language,
                bundle: language,
            }),
            Self::CodeWorkspace { .. } => Ok(()),
            Self::CellView { reference } => {
                reference.validate_name_segments().map_err(|error| {
                    ProjectSourceError::InvalidCellViewOwner {
                        reference: reference.display_path(),
                        message: error.to_string(),
                    }
                })?;
                if language != ProjectSourceLanguage::VerilogA {
                    return Err(ProjectSourceError::UnsupportedCellViewLanguage { language });
                }
                Ok(())
            }
        }
    }
}

/// Identity of validation evidence for exact source bytes. For a document the
/// digest covers that document; for a bundle it covers the complete framed
/// closure returned by [`ProjectSourceBundle::closure_digest`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectSourceValidationIdentity {
    revision: ObjectRevision,
    content_digest: ContentDigest,
}

impl ProjectSourceValidationIdentity {
    #[must_use]
    pub const fn revision(self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub const fn content_digest(self) -> ContentDigest {
        self.content_digest
    }

    #[must_use]
    pub const fn closure_digest(self) -> ContentDigest {
        self.content_digest
    }
}

/// One exact UTF-8 root source retained by a bundle.
///
/// `file_name` is retained as the compatibility accessor name, but its value is
/// a normalized project-relative logical path and may contain `/` separators.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectSourceDocument {
    #[serde(default = "missing_project_source_document_id")]
    id: ProjectSourceDocumentId,
    file_name: String,
    language: ProjectSourceLanguage,
    content: String,
    revision: ObjectRevision,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    validated_identity: Option<ProjectSourceValidationIdentity>,
}

impl ProjectSourceDocument {
    pub fn try_new(
        file_name: impl Into<String>,
        language: ProjectSourceLanguage,
        content: impl Into<String>,
    ) -> Result<Self, ProjectSourceError> {
        Self::try_new_with_id(ProjectSourceDocumentId::new(), file_name, language, content)
    }

    fn try_new_with_id(
        id: ProjectSourceDocumentId,
        file_name: impl Into<String>,
        language: ProjectSourceLanguage,
        content: impl Into<String>,
    ) -> Result<Self, ProjectSourceError> {
        let document = Self {
            id,
            file_name: file_name.into(),
            language,
            content: content.into(),
            revision: ObjectRevision::INITIAL,
            validated_identity: None,
        };
        document.validate()?;
        Ok(document)
    }

    #[must_use]
    pub const fn id(&self) -> ProjectSourceDocumentId {
        self.id
    }

    #[must_use]
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    #[must_use]
    pub fn logical_path(&self) -> &str {
        &self.file_name
    }

    #[must_use]
    pub const fn language(&self) -> ProjectSourceLanguage {
        self.language
    }

    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }

    #[must_use]
    pub fn exact_bytes(&self) -> &[u8] {
        self.content.as_bytes()
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    /// SHA-256 over the exact UTF-8 content bytes. No newline normalization,
    /// Unicode normalization, trimming, or transcoding is performed.
    #[must_use]
    pub fn content_digest(&self) -> ContentDigest {
        ContentDigest::from_bytes(Sha256::digest(self.content.as_bytes()).into())
    }

    #[must_use]
    pub const fn validated_identity(&self) -> Option<ProjectSourceValidationIdentity> {
        self.validated_identity
    }

    #[must_use]
    pub fn validation_is_current(&self) -> bool {
        self.validated_identity.is_some_and(|identity| {
            identity.revision == self.revision && identity.content_digest == self.content_digest()
        })
    }

    fn replace_content(&mut self, content: String) -> Result<bool, ProjectSourceError> {
        if self.content == content {
            return Ok(false);
        }
        validate_source_content(&self.file_name, &content)?;
        self.revision = next_revision(self.revision, &self.file_name)?;
        self.content = content;
        self.validated_identity = None;
        Ok(true)
    }

    fn replace_imported(
        &mut self,
        file_name: String,
        content: String,
    ) -> Result<bool, ProjectSourceError> {
        if self.file_name == file_name && self.content == content {
            return Ok(false);
        }
        let replacement = Self {
            id: self.id,
            file_name,
            language: self.language,
            content,
            revision: next_revision(self.revision, &self.file_name)?,
            validated_identity: None,
        };
        replacement.validate()?;
        *self = replacement;
        Ok(true)
    }

    fn mark_validated(&mut self) -> ProjectSourceValidationIdentity {
        let identity = ProjectSourceValidationIdentity {
            revision: self.revision,
            content_digest: self.content_digest(),
        };
        self.validated_identity = Some(identity);
        identity
    }

    fn invalidate_validation(&mut self) {
        self.validated_identity = None;
    }

    fn validate(&self) -> Result<(), ProjectSourceError> {
        if self.id.is_missing() {
            return Err(ProjectSourceError::MissingDocumentIdentity {
                logical_path: self.file_name.clone(),
            });
        }
        validate_project_source_file_name(&self.file_name, self.language)?;
        validate_source_content(&self.file_name, &self.content)?;
        if self.validated_identity.is_some() && !self.validation_is_current() {
            return Err(ProjectSourceError::StaleValidationIdentity {
                file_name: self.file_name.clone(),
            });
        }
        Ok(())
    }
}

/// One non-root file retained in a bundle's exact source closure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectSourceFile {
    #[serde(default = "missing_project_source_document_id")]
    id: ProjectSourceDocumentId,
    logical_path: String,
    content: String,
    #[serde(default)]
    revision: ObjectRevision,
}

impl ProjectSourceFile {
    pub fn try_new(
        logical_path: impl Into<String>,
        content: impl Into<String>,
    ) -> Result<Self, ProjectSourceError> {
        Self::try_new_with_id(ProjectSourceDocumentId::new(), logical_path, content)
    }

    fn try_new_with_id(
        id: ProjectSourceDocumentId,
        logical_path: impl Into<String>,
        content: impl Into<String>,
    ) -> Result<Self, ProjectSourceError> {
        let file = Self {
            id,
            logical_path: logical_path.into(),
            content: content.into(),
            revision: ObjectRevision::INITIAL,
        };
        file.validate()?;
        Ok(file)
    }

    #[must_use]
    pub const fn id(&self) -> ProjectSourceDocumentId {
        self.id
    }

    #[must_use]
    pub fn logical_path(&self) -> &str {
        &self.logical_path
    }

    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub fn exact_bytes(&self) -> &[u8] {
        self.content.as_bytes()
    }

    fn validate(&self) -> Result<(), ProjectSourceError> {
        if self.id.is_missing() {
            return Err(ProjectSourceError::MissingDocumentIdentity {
                logical_path: self.logical_path.clone(),
            });
        }
        validate_logical_path(&self.logical_path)?;
        validate_source_content(&self.logical_path, &self.content)
    }
}

/// One authenticated importer-to-imported edge in a source closure.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectSourceDependency {
    importer: String,
    imported: String,
}

/// Semantic role of one document in a project source closure.
///
/// A role is persisted project configuration. It is never inferred from a
/// mockup filename at execution time. More than one Automation run plan may be
/// retained; singleton roles are bound to exactly one portable logical path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProjectSourceRole {
    VerilogABuildProfile,
    AutomationEntry,
    AutomationRunPlan,
    AutomationEnvironmentLock,
    AutomationPermissionManifest,
}

impl ProjectSourceRole {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::VerilogABuildProfile => "Verilog-A build profile",
            Self::AutomationEntry => "Automation entry",
            Self::AutomationRunPlan => "Automation run plan",
            Self::AutomationEnvironmentLock => "Automation environment lock",
            Self::AutomationPermissionManifest => "Automation permission manifest",
        }
    }

    const fn allows_multiple(self) -> bool {
        matches!(self, Self::AutomationRunPlan)
    }

    const fn is_allowed_for(self, language: ProjectSourceLanguage) -> bool {
        matches!(
            (self, language),
            (Self::VerilogABuildProfile, ProjectSourceLanguage::VerilogA)
                | (
                    Self::AutomationEntry
                        | Self::AutomationRunPlan
                        | Self::AutomationEnvironmentLock
                        | Self::AutomationPermissionManifest,
                    ProjectSourceLanguage::RSpiceAutomation
                )
        )
    }
}

/// Persisted binding between a portable logical path and its semantic role.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectSourceRoleBinding {
    logical_path: String,
    role: ProjectSourceRole,
}

impl ProjectSourceRoleBinding {
    pub fn try_new(
        logical_path: impl Into<String>,
        role: ProjectSourceRole,
    ) -> Result<Self, ProjectSourceError> {
        let binding = Self {
            logical_path: logical_path.into(),
            role,
        };
        validate_logical_path(&binding.logical_path)?;
        Ok(binding)
    }

    #[must_use]
    pub fn logical_path(&self) -> &str {
        &self.logical_path
    }

    #[must_use]
    pub const fn role(&self) -> ProjectSourceRole {
        self.role
    }

    fn canonical_key(&self) -> (ProjectSourceRole, String) {
        (self.role, path_key(&self.logical_path))
    }
}

impl ProjectSourceDependency {
    pub fn try_new(
        importer: impl Into<String>,
        imported: impl Into<String>,
    ) -> Result<Self, ProjectSourceError> {
        let dependency = Self {
            importer: importer.into(),
            imported: imported.into(),
        };
        validate_logical_path(&dependency.importer)?;
        validate_logical_path(&dependency.imported)?;
        Ok(dependency)
    }

    #[must_use]
    pub fn importer(&self) -> &str {
        &self.importer
    }

    #[must_use]
    pub fn imported(&self) -> &str {
        &self.imported
    }

    fn canonical_key(&self) -> (String, String) {
        (path_key(&self.importer), path_key(&self.imported))
    }
}

/// A complete, portable, exact source closure owned by one product surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectSourceBundle {
    id: ProjectSourceId,
    owner: ProjectSourceOwner,
    language: ProjectSourceLanguage,
    root: ProjectSourceDocument,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    files: Vec<ProjectSourceFile>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    dependencies: Vec<ProjectSourceDependency>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    roles: Vec<ProjectSourceRoleBinding>,
    revision: ObjectRevision,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    validated_identity: Option<ProjectSourceValidationIdentity>,
    /// Bounded, content-complete revision journal. Snapshots contain source
    /// graph data only (never nested history), so project save, browser
    /// recovery, compare, and revert share one portable authority.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    history: Vec<ProjectSourceRevisionSnapshot>,
    /// Immutable build/qualification ledger. This evidence intentionally does
    /// not participate in source revision or closure identity.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    qualifications: Vec<ProjectSourceQualificationRecord>,
}

const MAX_SOURCE_HISTORY_REVISIONS: usize = 64;
const MAX_SOURCE_HISTORY_BYTES: usize = 32 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProjectSourceQualificationDisposition {
    Passed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectSourceQualificationTarget {
    pub name: String,
    pub readiness: String,
    pub maturity: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectSourceQualificationCheck {
    pub name: String,
    pub disposition: String,
    pub findings: usize,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectSourceQualificationRecord {
    pub(crate) sequence: u64,
    pub attempt_id: Uuid,
    pub recorded_at_unix_ms: u64,
    pub source_revision: u64,
    pub source_closure_digest: ContentDigest,
    pub profile_digest: ContentDigest,
    pub package_name: String,
    pub package_version: String,
    pub selected_module: String,
    pub compiler_version: String,
    pub disposition: ProjectSourceQualificationDisposition,
    pub report_digest: ContentDigest,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<ProjectSourceQualificationTarget>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub checks: Vec<ProjectSourceQualificationCheck>,
    #[serde(default)]
    pub error_count: usize,
    #[serde(default)]
    pub warning_count: usize,
}

impl ProjectSourceQualificationRecord {
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    fn validate(&self) -> Result<(), ProjectSourceError> {
        if self.sequence == 0
            || self.attempt_id.is_nil()
            || self.recorded_at_unix_ms == 0
            || self.source_revision == 0
            || self.package_name.trim().is_empty()
            || self.package_version.trim().is_empty()
            || self.selected_module.trim().is_empty()
            || self.compiler_version.trim().is_empty()
        {
            return Err(ProjectSourceError::InvalidQualificationRecord);
        }
        for value in [
            &self.package_name,
            &self.package_version,
            &self.selected_module,
            &self.compiler_version,
        ] {
            if value.len() > MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES
                || value.chars().any(char::is_control)
            {
                return Err(ProjectSourceError::InvalidQualificationRecord);
            }
        }
        if self.targets.iter().any(|target| {
            target.name.trim().is_empty()
                || target.readiness.trim().is_empty()
                || target.maturity.trim().is_empty()
                || target.name.len() > MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES
                || target.readiness.len() > MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES
                || target.maturity.len() > MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES
                || target.detail.len() > MAX_PROJECT_CODE_SOURCE_BYTES
        }) || self.checks.iter().any(|check| {
            check.name.trim().is_empty()
                || check.disposition.trim().is_empty()
                || check.name.len() > MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES
                || check.disposition.len() > MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES
                || check.detail.len() > MAX_PROJECT_CODE_SOURCE_BYTES
        }) {
            return Err(ProjectSourceError::InvalidQualificationRecord);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectSourceRevisionSnapshot {
    revision: ObjectRevision,
    closure_digest: ContentDigest,
    owner: ProjectSourceOwner,
    root: ProjectSourceDocument,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    files: Vec<ProjectSourceFile>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    dependencies: Vec<ProjectSourceDependency>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    roles: Vec<ProjectSourceRoleBinding>,
}

impl ProjectSourceRevisionSnapshot {
    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub const fn closure_digest(&self) -> ContentDigest {
        self.closure_digest
    }

    #[must_use]
    pub fn root(&self) -> &ProjectSourceDocument {
        &self.root
    }

    #[must_use]
    pub fn files(&self) -> &[ProjectSourceFile] {
        &self.files
    }

    #[must_use]
    pub fn file_content(&self, logical_path: &str) -> Option<&str> {
        let key = path_key(logical_path);
        if path_key(self.root.logical_path()) == key {
            return Some(self.root.content());
        }
        self.files
            .iter()
            .find(|file| path_key(file.logical_path()) == key)
            .map(ProjectSourceFile::content)
    }

    fn retained_bytes(&self) -> usize {
        self.root.content().len()
            + self
                .files
                .iter()
                .map(|file| file.content().len())
                .sum::<usize>()
    }
}

impl ProjectSourceBundle {
    pub fn try_new(
        owner: ProjectSourceOwner,
        language: ProjectSourceLanguage,
        root_path: impl Into<String>,
        root_content: impl Into<String>,
        files: impl IntoIterator<Item = ProjectSourceFile>,
        dependencies: impl IntoIterator<Item = ProjectSourceDependency>,
    ) -> Result<Self, ProjectSourceError> {
        let root_path = root_path.into();
        let roles = default_source_roles(language, &root_path)?;
        Self::try_new_with_id_and_roles(
            ProjectSourceId::new(),
            owner,
            language,
            root_path,
            root_content,
            files,
            dependencies,
            roles,
        )
    }

    /// Construct a source bundle with explicit persisted semantic roles.
    /// Callers creating Automation bundles should use this constructor so
    /// run-plan, environment, and permission documents never depend on path
    /// or content heuristics.
    pub fn try_new_with_roles(
        owner: ProjectSourceOwner,
        language: ProjectSourceLanguage,
        root_path: impl Into<String>,
        root_content: impl Into<String>,
        files: impl IntoIterator<Item = ProjectSourceFile>,
        dependencies: impl IntoIterator<Item = ProjectSourceDependency>,
        roles: impl IntoIterator<Item = ProjectSourceRoleBinding>,
    ) -> Result<Self, ProjectSourceError> {
        Self::try_new_with_id_and_roles(
            ProjectSourceId::new(),
            owner,
            language,
            root_path,
            root_content,
            files,
            dependencies,
            roles,
        )
    }

    pub fn try_new_with_id(
        id: ProjectSourceId,
        owner: ProjectSourceOwner,
        language: ProjectSourceLanguage,
        root_path: impl Into<String>,
        root_content: impl Into<String>,
        files: impl IntoIterator<Item = ProjectSourceFile>,
        dependencies: impl IntoIterator<Item = ProjectSourceDependency>,
    ) -> Result<Self, ProjectSourceError> {
        let root_path = root_path.into();
        let roles = default_source_roles(language, &root_path)?;
        Self::try_new_with_id_and_roles(
            id,
            owner,
            language,
            root_path,
            root_content,
            files,
            dependencies,
            roles,
        )
    }

    pub fn try_new_with_id_and_roles(
        id: ProjectSourceId,
        owner: ProjectSourceOwner,
        language: ProjectSourceLanguage,
        root_path: impl Into<String>,
        root_content: impl Into<String>,
        files: impl IntoIterator<Item = ProjectSourceFile>,
        dependencies: impl IntoIterator<Item = ProjectSourceDependency>,
        roles: impl IntoIterator<Item = ProjectSourceRoleBinding>,
    ) -> Result<Self, ProjectSourceError> {
        let root_path = root_path.into();
        let mut bundle = Self {
            id,
            owner,
            language,
            root: ProjectSourceDocument::try_new_with_id(
                ProjectSourceDocumentId::migrated(id, &root_path),
                root_path,
                language,
                root_content,
            )?,
            files: files.into_iter().collect(),
            dependencies: dependencies.into_iter().collect(),
            roles: roles.into_iter().collect(),
            revision: ObjectRevision::INITIAL,
            validated_identity: None,
            history: Vec::new(),
            qualifications: Vec::new(),
        };
        bundle.canonicalize();
        bundle.validate()?;
        Ok(bundle)
    }

    fn migrated(mut document: ProjectSourceDocument) -> Result<Self, ProjectSourceError> {
        let id = ProjectSourceId::migrated(&document);
        if document.id.is_missing() {
            document.id = ProjectSourceDocumentId::migrated(id, document.logical_path());
        }
        document.validate()?;
        let revision = document.revision;
        let was_validated = document.validation_is_current();
        let mut bundle = Self {
            id,
            owner: ProjectSourceOwner::code_workspace(document.language),
            language: document.language,
            roles: default_source_roles(document.language, document.logical_path())?,
            root: document,
            files: Vec::new(),
            dependencies: Vec::new(),
            revision,
            validated_identity: None,
            history: Vec::new(),
            qualifications: Vec::new(),
        };
        if was_validated {
            bundle.validated_identity = Some(ProjectSourceValidationIdentity {
                revision,
                content_digest: bundle.closure_digest(),
            });
        }
        bundle.validate()?;
        Ok(bundle)
    }

    #[must_use]
    pub const fn id(&self) -> ProjectSourceId {
        self.id
    }

    #[must_use]
    pub fn owner(&self) -> &ProjectSourceOwner {
        &self.owner
    }

    #[must_use]
    pub const fn language(&self) -> ProjectSourceLanguage {
        self.language
    }

    #[must_use]
    pub fn root(&self) -> &ProjectSourceDocument {
        &self.root
    }

    #[must_use]
    pub fn files(&self) -> &[ProjectSourceFile] {
        &self.files
    }

    #[must_use]
    pub fn dependencies(&self) -> &[ProjectSourceDependency] {
        &self.dependencies
    }

    #[must_use]
    pub fn roles(&self) -> &[ProjectSourceRoleBinding] {
        &self.roles
    }

    #[must_use]
    pub fn qualifications(&self) -> &[ProjectSourceQualificationRecord] {
        &self.qualifications
    }

    /// Append immutable qualification evidence for the exact current source
    /// identity. Evidence never rewrites or deletes prior attempts.
    pub fn append_qualification(
        &mut self,
        mut record: ProjectSourceQualificationRecord,
    ) -> Result<u64, ProjectSourceError> {
        if self.qualifications.len() >= MAX_PROJECT_SOURCE_QUALIFICATION_RECORDS {
            return Err(ProjectSourceError::QualificationHistoryLimitExceeded);
        }
        if record.source_revision != self.revision.get()
            || record.source_closure_digest != self.closure_digest()
        {
            return Err(ProjectSourceError::QualificationIdentityMismatch);
        }
        let sequence = self
            .qualifications
            .last()
            .map_or(1, |previous| previous.sequence.saturating_add(1));
        if sequence == 0 {
            return Err(ProjectSourceError::QualificationHistoryLimitExceeded);
        }
        record.sequence = sequence;
        record.validate()?;
        self.qualifications.push(record);
        Ok(sequence)
    }

    pub fn paths_for_role(&self, role: ProjectSourceRole) -> impl Iterator<Item = &str> {
        self.roles
            .iter()
            .filter(move |binding| binding.role == role)
            .map(ProjectSourceRoleBinding::logical_path)
    }

    #[must_use]
    pub fn role_for_path(&self, logical_path: &str) -> Option<ProjectSourceRole> {
        let key = path_key(logical_path);
        self.roles
            .iter()
            .find(|binding| path_key(binding.logical_path()) == key)
            .map(ProjectSourceRoleBinding::role)
    }

    /// Bind one retained document to a language-appropriate semantic role atomically.
    /// A document has at most one role, and singleton roles cannot be rebound
    /// without first unbinding their current document.
    pub fn bind_role(
        &mut self,
        logical_path: &str,
        role: ProjectSourceRole,
    ) -> Result<bool, ProjectSourceError> {
        validate_logical_path(logical_path)?;
        if !role.is_allowed_for(self.language) {
            return Err(ProjectSourceError::RoleNotAllowedForLanguage {
                role,
                language: self.language,
            });
        }
        let canonical_path = self
            .logical_path_with_original_case(logical_path)
            .ok_or_else(|| ProjectSourceError::MissingFile {
                logical_path: logical_path.to_owned(),
            })?
            .to_owned();
        if self.roles.iter().any(|binding| {
            binding.role == role && path_key(binding.logical_path()) == path_key(&canonical_path)
        }) {
            return Ok(false);
        }
        if let Some(binding) = self
            .roles
            .iter()
            .find(|binding| path_key(binding.logical_path()) == path_key(&canonical_path))
        {
            return Err(ProjectSourceError::FileAlreadyHasRole {
                logical_path: canonical_path,
                role: binding.role,
            });
        }
        if !role.allows_multiple()
            && let Some(binding) = self.roles.iter().find(|binding| binding.role == role)
        {
            return Err(ProjectSourceError::DuplicateSingletonRole {
                role,
                first: binding.logical_path.clone(),
                second: canonical_path,
            });
        }
        if role == ProjectSourceRole::AutomationEntry
            && path_key(&canonical_path) != path_key(self.root.logical_path())
        {
            return Err(ProjectSourceError::AutomationEntryMustBeRoot {
                logical_path: canonical_path,
                root_path: self.root.logical_path().to_owned(),
            });
        }

        let mut candidate = self.clone();
        candidate.record_revision_snapshot(self);
        candidate
            .roles
            .push(ProjectSourceRoleBinding::try_new(canonical_path, role)?);
        candidate.advance_revision()?;
        candidate.invalidate_validation();
        candidate.canonicalize();
        candidate.validate()?;
        *self = candidate;
        Ok(true)
    }

    /// Remove a non-entry Automation role atomically. The entry role is part
    /// of bundle identity and moves with root renames; it cannot be removed.
    pub fn unbind_role(&mut self, logical_path: &str) -> Result<bool, ProjectSourceError> {
        validate_logical_path(logical_path)?;
        let key = path_key(logical_path);
        let Some(binding) = self
            .roles
            .iter()
            .find(|binding| path_key(binding.logical_path()) == key)
        else {
            return Ok(false);
        };
        if binding.role == ProjectSourceRole::AutomationEntry {
            return Err(ProjectSourceError::CannotUnbindAutomationEntry {
                logical_path: binding.logical_path.clone(),
            });
        }
        let mut candidate = self.clone();
        candidate.record_revision_snapshot(self);
        candidate
            .roles
            .retain(|binding| path_key(binding.logical_path()) != key);
        candidate.advance_revision()?;
        candidate.invalidate_validation();
        candidate.canonicalize();
        candidate.validate()?;
        *self = candidate;
        Ok(true)
    }

    /// Replace the optional non-entry Automation role for one retained source
    /// as a single source-graph revision. Singleton roles move atomically from
    /// their prior document, so the project is never left in a transient state
    /// with two permission manifests or no environment lock merely because a
    /// UI operation was interrupted between unbind and bind calls.
    pub fn set_non_entry_role(
        &mut self,
        logical_path: &str,
        role: Option<ProjectSourceRole>,
    ) -> Result<bool, ProjectSourceError> {
        validate_logical_path(logical_path)?;
        if self.language != ProjectSourceLanguage::RSpiceAutomation {
            return Err(ProjectSourceError::RoleNotAllowedForLanguage {
                role: role.unwrap_or(ProjectSourceRole::AutomationEntry),
                language: self.language,
            });
        }
        if role == Some(ProjectSourceRole::AutomationEntry) {
            return Err(ProjectSourceError::CannotUnbindAutomationEntry {
                logical_path: logical_path.to_owned(),
            });
        }
        let canonical_path = self
            .logical_path_with_original_case(logical_path)
            .ok_or_else(|| ProjectSourceError::MissingFile {
                logical_path: logical_path.to_owned(),
            })?
            .to_owned();
        if canonical_path.eq_ignore_ascii_case(self.root.logical_path()) {
            return Err(ProjectSourceError::CannotUnbindAutomationEntry {
                logical_path: canonical_path,
            });
        }
        let current = self.role_for_path(&canonical_path);
        if current == role {
            return Ok(false);
        }

        let mut candidate = self.clone();
        candidate.record_revision_snapshot(self);
        let requested_key = path_key(&canonical_path);
        candidate.roles.retain(|binding| {
            let same_document = path_key(binding.logical_path()) == requested_key;
            let displaced_singleton = role
                .is_some_and(|requested| !requested.allows_multiple() && binding.role == requested);
            binding.role == ProjectSourceRole::AutomationEntry
                || (!same_document && !displaced_singleton)
        });
        if let Some(role) = role {
            candidate
                .roles
                .push(ProjectSourceRoleBinding::try_new(&canonical_path, role)?);
        }
        candidate.advance_revision()?;
        candidate.invalidate_validation();
        candidate.canonicalize();
        candidate.validate()?;
        *self = candidate;
        Ok(true)
    }

    /// Return the exact retained UTF-8 source for one portable logical path.
    /// Lookups are case-insensitive because bundle validation rejects paths
    /// that would collide on a supported desktop filesystem.
    #[must_use]
    pub fn file_content(&self, logical_path: &str) -> Option<&str> {
        let key = path_key(logical_path);
        if path_key(self.root.logical_path()) == key {
            return Some(self.root.content());
        }
        self.files
            .iter()
            .find(|file| path_key(file.logical_path()) == key)
            .map(ProjectSourceFile::content)
    }

    #[must_use]
    pub fn document_id(&self, logical_path: &str) -> Option<ProjectSourceDocumentId> {
        let key = path_key(logical_path);
        if path_key(self.root.logical_path()) == key {
            return Some(self.root.id());
        }
        self.files
            .iter()
            .find(|file| path_key(file.logical_path()) == key)
            .map(ProjectSourceFile::id)
    }

    #[must_use]
    pub fn document_revision(&self, logical_path: &str) -> Option<ObjectRevision> {
        let key = path_key(logical_path);
        if path_key(self.root.logical_path()) == key {
            return Some(self.root.revision());
        }
        self.files
            .iter()
            .find(|file| path_key(file.logical_path()) == key)
            .map(ProjectSourceFile::revision)
    }

    #[must_use]
    pub fn contains_file(&self, logical_path: &str) -> bool {
        self.file_content(logical_path).is_some()
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub fn revision_history(&self) -> &[ProjectSourceRevisionSnapshot] {
        &self.history
    }

    #[must_use]
    pub fn revision_snapshot(
        &self,
        revision: ObjectRevision,
    ) -> Option<&ProjectSourceRevisionSnapshot> {
        self.history
            .iter()
            .find(|snapshot| snapshot.revision == revision)
    }

    /// Restore a retained source-graph revision as a new monotonic revision.
    /// The expected-current check prevents a comparison dialog from applying
    /// to a newer bundle, and current ownership is intentionally preserved.
    pub fn restore_revision(
        &mut self,
        expected_current: ObjectRevision,
        retained_revision: ObjectRevision,
    ) -> Result<bool, ProjectSourceError> {
        if self.revision != expected_current {
            return Err(ProjectSourceError::StaleRevisionRestore {
                expected: expected_current.get(),
                found: self.revision.get(),
            });
        }
        let snapshot = self.revision_snapshot(retained_revision).cloned().ok_or(
            ProjectSourceError::MissingRetainedRevision {
                revision: retained_revision.get(),
            },
        )?;
        if self.root == snapshot.root
            && self.files == snapshot.files
            && self.dependencies == snapshot.dependencies
            && self.roles == snapshot.roles
        {
            return Ok(false);
        }
        let mut candidate = self.clone();
        candidate.record_revision_snapshot(self);
        candidate.root = snapshot.root;
        candidate.files = snapshot.files;
        candidate.dependencies = snapshot.dependencies;
        candidate.roles = snapshot.roles;
        candidate.advance_revision()?;
        candidate.root.revision = candidate.revision;
        for file in &mut candidate.files {
            file.revision = candidate.revision;
        }
        candidate.invalidate_validation();
        candidate.canonicalize();
        candidate.validate()?;
        *self = candidate;
        Ok(true)
    }

    #[must_use]
    pub const fn validated_identity(&self) -> Option<ProjectSourceValidationIdentity> {
        self.validated_identity
    }

    #[must_use]
    pub fn validation_is_current(&self) -> bool {
        self.validated_identity.is_some_and(|identity| {
            identity.revision == self.revision && identity.content_digest == self.closure_digest()
        })
    }

    /// SHA-256 over an explicitly versioned sequence of length-framed owner,
    /// language, path, edge, and exact UTF-8 content fields. Framing prevents
    /// concatenation ambiguity; canonical ordering makes the result independent
    /// of insertion order.
    #[must_use]
    pub fn closure_digest(&self) -> ContentDigest {
        let mut hasher = Sha256::new();
        hash_frame(&mut hasher, CLOSURE_DIGEST_DOMAIN);
        match &self.owner {
            ProjectSourceOwner::CodeWorkspace { language } => {
                hash_frame(&mut hasher, b"code-workspace");
                hash_frame(&mut hasher, language.stable_name().as_bytes());
            }
            ProjectSourceOwner::CellView { reference } => {
                hash_frame(&mut hasher, b"cell-view");
                hash_frame(&mut hasher, reference.library.as_bytes());
                hash_frame(&mut hasher, reference.cell.as_bytes());
                hash_frame(&mut hasher, reference.view.as_bytes());
            }
        }
        hash_frame(&mut hasher, self.language.stable_name().as_bytes());
        hash_frame(&mut hasher, self.root.id.as_uuid().as_bytes());
        hash_frame(&mut hasher, self.root.file_name.as_bytes());
        hash_frame(&mut hasher, self.root.content.as_bytes());
        hash_u64(&mut hasher, self.files.len() as u64);
        for file in &self.files {
            hash_frame(&mut hasher, file.id.as_uuid().as_bytes());
            hash_frame(&mut hasher, file.logical_path.as_bytes());
            hash_frame(&mut hasher, file.content.as_bytes());
        }
        hash_u64(&mut hasher, self.dependencies.len() as u64);
        for dependency in &self.dependencies {
            hash_frame(&mut hasher, dependency.importer.as_bytes());
            hash_frame(&mut hasher, dependency.imported.as_bytes());
        }
        hash_u64(&mut hasher, self.roles.len() as u64);
        for binding in &self.roles {
            hash_frame(
                &mut hasher,
                source_role_stable_name(binding.role).as_bytes(),
            );
            hash_frame(&mut hasher, binding.logical_path.as_bytes());
        }
        ContentDigest::from_bytes(hasher.finalize().into())
    }

    /// Replace exact bytes for one retained logical path atomically.
    pub fn replace_file_content(
        &mut self,
        logical_path: &str,
        content: String,
    ) -> Result<bool, ProjectSourceError> {
        validate_logical_path(logical_path)?;
        let key = path_key(logical_path);
        let mut candidate = self.clone();
        candidate.record_revision_snapshot(self);
        let changed = if path_key(&candidate.root.file_name) == key {
            candidate.root.replace_content(content)?
        } else if let Some(file) = candidate
            .files
            .iter_mut()
            .find(|file| path_key(&file.logical_path) == key)
        {
            if file.content == content {
                false
            } else {
                validate_source_content(&file.logical_path, &content)?;
                file.revision = next_revision(file.revision, &file.logical_path)?;
                file.content = content;
                true
            }
        } else {
            return Err(ProjectSourceError::MissingFile {
                logical_path: logical_path.to_owned(),
            });
        };
        if !changed {
            return Ok(false);
        }
        candidate.advance_revision()?;
        candidate.invalidate_validation();
        candidate.validate()?;
        *self = candidate;
        Ok(true)
    }

    /// Replace any number of documents as one journaled source-graph
    /// revision. Per-document revisions advance for changed documents, while
    /// the owning bundle advances exactly once and retains exactly one
    /// predecessor snapshot.
    pub fn replace_files_transactionally(
        &mut self,
        replacements: impl IntoIterator<Item = (String, String)>,
    ) -> Result<usize, ProjectSourceError> {
        let replacements = replacements.into_iter().collect::<Vec<_>>();
        let mut paths = BTreeSet::new();
        for (logical_path, _) in &replacements {
            validate_logical_path(logical_path)?;
            if !paths.insert(path_key(logical_path)) {
                return Err(ProjectSourceError::DuplicateLogicalPath {
                    logical_path: logical_path.clone(),
                });
            }
        }

        let mut candidate = self.clone();
        candidate.record_revision_snapshot(self);
        let mut changed = 0_usize;
        for (logical_path, content) in replacements {
            let key = path_key(&logical_path);
            let file_changed = if path_key(candidate.root.logical_path()) == key {
                candidate.root.replace_content(content)?
            } else if let Some(file) = candidate
                .files
                .iter_mut()
                .find(|file| path_key(file.logical_path()) == key)
            {
                if file.content == content {
                    false
                } else {
                    validate_source_content(file.logical_path(), &content)?;
                    file.revision = next_revision(file.revision, file.logical_path())?;
                    file.content = content;
                    true
                }
            } else {
                return Err(ProjectSourceError::MissingFile { logical_path });
            };
            changed += usize::from(file_changed);
        }
        if changed == 0 {
            return Ok(0);
        }
        candidate.advance_revision()?;
        candidate.invalidate_validation();
        candidate.validate()?;
        *self = candidate;
        Ok(changed)
    }

    /// Add one dependency document to the sealed closure and attach it to an
    /// existing importer. The corresponding include is inserted at the start
    /// of the importer, so the authenticated graph and executable source can
    /// never diverge as a side effect of the authoring command.
    pub fn add_file(
        &mut self,
        importer: &str,
        file: ProjectSourceFile,
    ) -> Result<bool, ProjectSourceError> {
        validate_logical_path(importer)?;
        file.validate()?;
        if self.contains_file(file.logical_path()) {
            return Err(ProjectSourceError::DuplicateLogicalPath {
                logical_path: file.logical_path().to_owned(),
            });
        }
        if !self.contains_file(importer) {
            return Err(ProjectSourceError::MissingFile {
                logical_path: importer.to_owned(),
            });
        }

        let mut candidate = self.clone();
        candidate.record_revision_snapshot(self);
        let imported = file.logical_path().to_owned();
        candidate.files.push(file);
        candidate
            .dependencies
            .push(ProjectSourceDependency::try_new(
                importer.to_owned(),
                imported.clone(),
            )?);
        if candidate.language == ProjectSourceLanguage::VerilogA {
            candidate.prepend_include(importer, &imported)?;
        }
        candidate.advance_revision()?;
        candidate.invalidate_validation();
        candidate.canonicalize();
        candidate.validate()?;
        *self = candidate;
        Ok(true)
    }

    /// Add one project document, dependency edge, and semantic role as one
    /// source-graph revision. Unlike ordinary Verilog-A dependencies, a build
    /// profile is configuration and is therefore never injected as an
    /// `` `include `` directive into source text.
    pub fn add_file_with_role(
        &mut self,
        importer: &str,
        file: ProjectSourceFile,
        role: ProjectSourceRole,
    ) -> Result<bool, ProjectSourceError> {
        validate_logical_path(importer)?;
        file.validate()?;
        if !role.is_allowed_for(self.language) {
            return Err(ProjectSourceError::RoleNotAllowedForLanguage {
                role,
                language: self.language,
            });
        }
        if self.contains_file(file.logical_path()) {
            return Err(ProjectSourceError::DuplicateLogicalPath {
                logical_path: file.logical_path().to_owned(),
            });
        }
        if !self.contains_file(importer) {
            return Err(ProjectSourceError::MissingFile {
                logical_path: importer.to_owned(),
            });
        }
        if !role.allows_multiple()
            && let Some(binding) = self.roles.iter().find(|binding| binding.role == role)
        {
            return Err(ProjectSourceError::DuplicateSingletonRole {
                role,
                first: binding.logical_path.clone(),
                second: file.logical_path().to_owned(),
            });
        }

        let mut candidate = self.clone();
        candidate.record_revision_snapshot(self);
        let imported = file.logical_path().to_owned();
        candidate.files.push(file);
        candidate
            .dependencies
            .push(ProjectSourceDependency::try_new(importer, &imported)?);
        candidate
            .roles
            .push(ProjectSourceRoleBinding::try_new(&imported, role)?);
        candidate.advance_revision()?;
        candidate.invalidate_validation();
        candidate.canonicalize();
        candidate.validate()?;
        *self = candidate;
        Ok(true)
    }

    /// Rename one source document atomically. Every authenticated dependency
    /// endpoint and every include that realizes an incoming dependency is
    /// rewritten in the same transaction. The root remains the explicit root;
    /// only its portable logical path changes.
    pub fn rename_file(
        &mut self,
        current_path: &str,
        new_path: &str,
    ) -> Result<bool, ProjectSourceError> {
        validate_logical_path(current_path)?;
        validate_logical_path(new_path)?;
        let current_key = path_key(current_path);
        let new_key = path_key(new_path);
        let current = self
            .logical_path_with_original_case(current_path)
            .ok_or_else(|| ProjectSourceError::MissingFile {
                logical_path: current_path.to_owned(),
            })?;
        if current == new_path {
            return Ok(false);
        }
        if current_key != new_key && self.contains_file(new_path) {
            return Err(ProjectSourceError::DuplicateLogicalPath {
                logical_path: new_path.to_owned(),
            });
        }

        let mut candidate = self.clone();
        candidate.record_revision_snapshot(self);
        let canonical_current = candidate
            .logical_path_with_original_case(current_path)
            .ok_or_else(|| ProjectSourceError::MissingFile {
                logical_path: current_path.to_owned(),
            })?
            .to_owned();
        if candidate.language == ProjectSourceLanguage::VerilogA {
            let incoming = candidate
                .dependencies
                .iter()
                .filter(|edge| path_key(edge.imported()) == current_key)
                .map(|edge| edge.importer().to_owned())
                .collect::<Vec<_>>();
            for importer in incoming {
                candidate.rewrite_include(&importer, &canonical_current, Some(new_path))?;
            }
        }

        if path_key(candidate.root.logical_path()) == current_key {
            candidate
                .root
                .replace_imported(new_path.to_owned(), candidate.root.content.clone())?;
        } else {
            let file = candidate
                .files
                .iter_mut()
                .find(|file| path_key(file.logical_path()) == current_key)
                .ok_or_else(|| ProjectSourceError::MissingFile {
                    logical_path: current_path.to_owned(),
                })?;
            let mut renamed =
                ProjectSourceFile::try_new_with_id(file.id, new_path, file.content.clone())?;
            renamed.revision = next_revision(file.revision, file.logical_path())?;
            *file = renamed;
        }
        for edge in &mut candidate.dependencies {
            if path_key(edge.importer()) == current_key {
                edge.importer = new_path.to_owned();
            }
            if path_key(edge.imported()) == current_key {
                edge.imported = new_path.to_owned();
            }
        }
        for binding in &mut candidate.roles {
            if path_key(binding.logical_path()) == current_key {
                binding.logical_path = new_path.to_owned();
            }
        }
        candidate.advance_revision()?;
        candidate.invalidate_validation();
        candidate.canonicalize();
        candidate.validate()?;
        *self = candidate;
        Ok(true)
    }

    /// Delete a non-root leaf document. Includes in every importer are removed
    /// atomically. A document with its own dependencies must be emptied from
    /// the leaves upward, preventing an accidental cascade that hides source
    /// loss from the user.
    pub fn remove_file(&mut self, logical_path: &str) -> Result<bool, ProjectSourceError> {
        validate_logical_path(logical_path)?;
        let key = path_key(logical_path);
        if path_key(self.root.logical_path()) == key {
            return Err(ProjectSourceError::CannotRemoveBundleRoot {
                logical_path: self.root.logical_path().to_owned(),
            });
        }
        let canonical_path = self
            .logical_path_with_original_case(logical_path)
            .ok_or_else(|| ProjectSourceError::MissingFile {
                logical_path: logical_path.to_owned(),
            })?
            .to_owned();
        if let Some(binding) = self
            .roles
            .iter()
            .find(|binding| path_key(binding.logical_path()) == key)
        {
            return Err(ProjectSourceError::RoleBoundFile {
                logical_path: canonical_path,
                role: binding.role,
            });
        }
        if let Some(edge) = self
            .dependencies
            .iter()
            .find(|edge| path_key(edge.importer()) == key)
        {
            return Err(ProjectSourceError::FileHasDependencies {
                logical_path: canonical_path,
                dependency: edge.imported().to_owned(),
            });
        }

        let mut candidate = self.clone();
        candidate.record_revision_snapshot(self);
        if candidate.language == ProjectSourceLanguage::VerilogA {
            let importers = candidate
                .dependencies
                .iter()
                .filter(|edge| path_key(edge.imported()) == key)
                .map(|edge| edge.importer().to_owned())
                .collect::<Vec<_>>();
            for importer in importers {
                candidate.rewrite_include(&importer, &canonical_path, None)?;
            }
        }
        candidate
            .files
            .retain(|file| path_key(file.logical_path()) != key);
        candidate
            .dependencies
            .retain(|edge| path_key(edge.importer()) != key && path_key(edge.imported()) != key);
        candidate.advance_revision()?;
        candidate.invalidate_validation();
        candidate.canonicalize();
        candidate.validate()?;
        *self = candidate;
        Ok(true)
    }

    fn logical_path_with_original_case(&self, logical_path: &str) -> Option<&str> {
        let key = path_key(logical_path);
        if path_key(self.root.logical_path()) == key {
            return Some(self.root.logical_path());
        }
        self.files
            .iter()
            .find(|file| path_key(file.logical_path()) == key)
            .map(ProjectSourceFile::logical_path)
    }

    /// Return the retained spelling of a case-insensitively matched logical
    /// path. Callers use this to keep command/search identity canonical.
    pub fn retained_logical_path(&self, logical_path: &str) -> Option<&str> {
        self.logical_path_with_original_case(logical_path)
    }

    fn prepend_include(
        &mut self,
        importer: &str,
        imported: &str,
    ) -> Result<(), ProjectSourceError> {
        let content =
            self.file_content(importer)
                .ok_or_else(|| ProjectSourceError::MissingFile {
                    logical_path: importer.to_owned(),
                })?;
        let updated = format!("`include \"{imported}\"\n{content}");
        self.replace_file_content_without_bundle_revision(importer, updated)
    }

    fn rewrite_include(
        &mut self,
        importer: &str,
        imported: &str,
        replacement: Option<&str>,
    ) -> Result<(), ProjectSourceError> {
        let source =
            self.file_content(importer)
                .ok_or_else(|| ProjectSourceError::MissingFile {
                    logical_path: importer.to_owned(),
                })?;
        let updated = rewrite_matching_include_lines(source, importer, imported, replacement);
        if updated != source {
            self.replace_file_content_without_bundle_revision(importer, updated)?;
        }
        Ok(())
    }

    fn replace_file_content_without_bundle_revision(
        &mut self,
        logical_path: &str,
        content: String,
    ) -> Result<(), ProjectSourceError> {
        let key = path_key(logical_path);
        if path_key(self.root.logical_path()) == key {
            self.root.replace_content(content)?;
            return Ok(());
        }
        let file = self
            .files
            .iter_mut()
            .find(|file| path_key(file.logical_path()) == key)
            .ok_or_else(|| ProjectSourceError::MissingFile {
                logical_path: logical_path.to_owned(),
            })?;
        validate_source_content(file.logical_path(), &content)?;
        file.revision = next_revision(file.revision, file.logical_path())?;
        file.content = content;
        Ok(())
    }

    fn replace_root_imported(
        &mut self,
        root_path: String,
        content: String,
    ) -> Result<bool, ProjectSourceError> {
        let retained_profile_path = (self.language == ProjectSourceLanguage::VerilogA)
            .then(|| {
                self.paths_for_role(ProjectSourceRole::VerilogABuildProfile)
                    .next()
                    .map(str::to_owned)
            })
            .flatten();
        let discarded_closure = self.files.iter().any(|file| {
            retained_profile_path
                .as_deref()
                .is_none_or(|path| !file.logical_path().eq_ignore_ascii_case(path))
        }) || self.dependencies.iter().any(|edge| {
            retained_profile_path.as_deref().is_none_or(|path| {
                !edge.imported().eq_ignore_ascii_case(path)
                    || !edge
                        .importer()
                        .eq_ignore_ascii_case(self.root.logical_path())
            })
        });
        let mut candidate = self.clone();
        candidate.record_revision_snapshot(self);
        let root_changed = candidate.root.replace_imported(root_path, content)?;
        if !root_changed && !discarded_closure {
            return Ok(false);
        }
        if let Some(profile_path) = retained_profile_path {
            candidate
                .files
                .retain(|file| file.logical_path().eq_ignore_ascii_case(&profile_path));
            candidate.dependencies.clear();
            candidate
                .dependencies
                .push(ProjectSourceDependency::try_new(
                    candidate.root.logical_path(),
                    &profile_path,
                )?);
            candidate.roles.retain(|binding| {
                binding.role == ProjectSourceRole::VerilogABuildProfile
                    && binding.logical_path().eq_ignore_ascii_case(&profile_path)
            });
        } else {
            candidate.files.clear();
            candidate.dependencies.clear();
            candidate.roles =
                default_source_roles(candidate.language, candidate.root.logical_path())?;
        }
        candidate.advance_revision()?;
        candidate.invalidate_validation();
        candidate.validate()?;
        *self = candidate;
        Ok(true)
    }

    pub fn mark_validated(
        &mut self,
    ) -> Result<ProjectSourceValidationIdentity, ProjectSourceError> {
        self.validate()?;
        self.root.mark_validated();
        let identity = ProjectSourceValidationIdentity {
            revision: self.revision,
            content_digest: self.closure_digest(),
        };
        self.validated_identity = Some(identity);
        Ok(identity)
    }

    pub fn validate(&self) -> Result<(), ProjectSourceError> {
        self.owner.validate(self.language)?;
        if self.root.language != self.language {
            return Err(ProjectSourceError::BundleLanguageMismatch {
                bundle: self.language,
                root: self.root.language,
            });
        }
        self.root.validate()?;
        if self.files.len().saturating_add(1) > MAX_PROJECT_SOURCE_FILES {
            return Err(ProjectSourceError::TooManyFiles {
                files: self.files.len().saturating_add(1),
                limit: MAX_PROJECT_SOURCE_FILES,
            });
        }
        if self.dependencies.len() > MAX_PROJECT_SOURCE_DEPENDENCIES {
            return Err(ProjectSourceError::TooManyDependencies {
                dependencies: self.dependencies.len(),
                limit: MAX_PROJECT_SOURCE_DEPENDENCIES,
            });
        }

        let root_key = path_key(&self.root.file_name);
        let mut paths = BTreeMap::new();
        paths.insert(root_key.clone(), self.root.file_name.as_str());
        let mut document_ids = BTreeSet::from([self.root.id]);
        let mut total_bytes = self.root.content.len();
        let mut previous_file_key: Option<String> = None;
        for file in &self.files {
            file.validate()?;
            if !document_ids.insert(file.id) {
                return Err(ProjectSourceError::DuplicateDocumentIdentity {
                    id: file.id,
                    logical_path: file.logical_path.clone(),
                });
            }
            total_bytes = total_bytes.checked_add(file.content.len()).ok_or(
                ProjectSourceError::BundleTooLarge {
                    bytes: usize::MAX,
                    limit: MAX_PROJECT_SOURCE_BUNDLE_BYTES,
                },
            )?;
            let key = path_key(&file.logical_path);
            if paths.insert(key.clone(), &file.logical_path).is_some() {
                return Err(ProjectSourceError::DuplicateLogicalPath {
                    logical_path: file.logical_path.clone(),
                });
            }
            if previous_file_key
                .as_ref()
                .is_some_and(|previous| previous >= &key)
            {
                return Err(ProjectSourceError::UnsortedFiles);
            }
            previous_file_key = Some(key);
        }
        if total_bytes > MAX_PROJECT_SOURCE_BUNDLE_BYTES {
            return Err(ProjectSourceError::BundleTooLarge {
                bytes: total_bytes,
                limit: MAX_PROJECT_SOURCE_BUNDLE_BYTES,
            });
        }

        let mut graph: BTreeMap<String, Vec<String>> = paths
            .keys()
            .map(|path| (path.clone(), Vec::new()))
            .collect();
        let mut edge_keys = BTreeSet::new();
        let mut previous_edge_key: Option<(String, String)> = None;
        for dependency in &self.dependencies {
            validate_logical_path(&dependency.importer)?;
            validate_logical_path(&dependency.imported)?;
            let key = dependency.canonical_key();
            if !paths.contains_key(&key.0) {
                return Err(ProjectSourceError::MissingDependencyEndpoint {
                    logical_path: dependency.importer.clone(),
                });
            }
            if !paths.contains_key(&key.1) {
                return Err(ProjectSourceError::MissingDependencyEndpoint {
                    logical_path: dependency.imported.clone(),
                });
            }
            if !edge_keys.insert(key.clone()) {
                return Err(ProjectSourceError::DuplicateDependency {
                    importer: dependency.importer.clone(),
                    imported: dependency.imported.clone(),
                });
            }
            if previous_edge_key
                .as_ref()
                .is_some_and(|previous| previous >= &key)
            {
                return Err(ProjectSourceError::UnsortedDependencies);
            }
            previous_edge_key = Some(key.clone());
            graph.entry(key.0).or_default().push(key.1);
        }

        let mut previous_role_key: Option<(ProjectSourceRole, String)> = None;
        let mut role_paths = BTreeSet::new();
        let mut singleton_roles = BTreeMap::new();
        for binding in &self.roles {
            if !binding.role.is_allowed_for(self.language) {
                return Err(ProjectSourceError::RoleNotAllowedForLanguage {
                    role: binding.role,
                    language: self.language,
                });
            }
            validate_logical_path(binding.logical_path())?;
            let path = path_key(binding.logical_path());
            if !paths.contains_key(&path) {
                return Err(ProjectSourceError::MissingRoleEndpoint {
                    logical_path: binding.logical_path.clone(),
                    role: binding.role,
                });
            }
            if !role_paths.insert(path) {
                return Err(ProjectSourceError::FileHasMultipleRoles {
                    logical_path: binding.logical_path.clone(),
                });
            }
            if !binding.role.allows_multiple()
                && let Some(first) = singleton_roles.insert(binding.role, binding.logical_path())
            {
                return Err(ProjectSourceError::DuplicateSingletonRole {
                    role: binding.role,
                    first: first.to_owned(),
                    second: binding.logical_path.clone(),
                });
            }
            let key = binding.canonical_key();
            if previous_role_key
                .as_ref()
                .is_some_and(|previous| previous >= &key)
            {
                return Err(ProjectSourceError::UnsortedRoles);
            }
            previous_role_key = Some(key);
        }
        if self.language == ProjectSourceLanguage::RSpiceAutomation {
            let entries = self
                .roles
                .iter()
                .filter(|binding| binding.role == ProjectSourceRole::AutomationEntry)
                .collect::<Vec<_>>();
            if entries.len() != 1 {
                return Err(ProjectSourceError::MissingAutomationEntryRole);
            }
            if path_key(entries[0].logical_path()) != root_key {
                return Err(ProjectSourceError::AutomationEntryMustBeRoot {
                    logical_path: entries[0].logical_path.clone(),
                    root_path: self.root.logical_path().to_owned(),
                });
            }
        }

        validate_dependency_graph(&root_key, &graph)?;
        if self.qualifications.len() > MAX_PROJECT_SOURCE_QUALIFICATION_RECORDS {
            return Err(ProjectSourceError::QualificationHistoryLimitExceeded);
        }
        let mut previous_qualification_sequence = 0_u64;
        for record in &self.qualifications {
            record.validate()?;
            if record.sequence <= previous_qualification_sequence
                || record.source_revision > self.revision.get()
            {
                return Err(ProjectSourceError::InvalidQualificationRecord);
            }
            previous_qualification_sequence = record.sequence;
        }
        if self.history.len() > MAX_SOURCE_HISTORY_REVISIONS {
            return Err(ProjectSourceError::RetainedHistoryLimitExceeded);
        }
        let retained_bytes = self
            .history
            .iter()
            .map(ProjectSourceRevisionSnapshot::retained_bytes)
            .sum::<usize>();
        if retained_bytes > MAX_SOURCE_HISTORY_BYTES && self.history.len() > 1 {
            return Err(ProjectSourceError::RetainedHistoryLimitExceeded);
        }
        let mut previous_revision = None;
        for snapshot in &self.history {
            if snapshot.revision >= self.revision
                || previous_revision.is_some_and(|revision| revision >= snapshot.revision)
            {
                return Err(ProjectSourceError::CorruptRetainedRevision {
                    revision: snapshot.revision.get(),
                });
            }
            let historical = ProjectSourceBundle {
                id: self.id,
                owner: snapshot.owner.clone(),
                language: self.language,
                root: snapshot.root.clone(),
                files: snapshot.files.clone(),
                dependencies: snapshot.dependencies.clone(),
                roles: snapshot.roles.clone(),
                revision: snapshot.revision,
                validated_identity: None,
                history: Vec::new(),
                qualifications: Vec::new(),
            };
            historical.validate()?;
            if historical.closure_digest() != snapshot.closure_digest {
                return Err(ProjectSourceError::CorruptRetainedRevision {
                    revision: snapshot.revision.get(),
                });
            }
            previous_revision = Some(snapshot.revision);
        }
        if self.validated_identity.is_some() && !self.validation_is_current() {
            return Err(ProjectSourceError::StaleBundleValidationIdentity { id: self.id });
        }
        Ok(())
    }

    fn canonicalize(&mut self) {
        self.files.sort_by(|left, right| {
            path_key(&left.logical_path)
                .cmp(&path_key(&right.logical_path))
                .then_with(|| left.logical_path.cmp(&right.logical_path))
        });
        self.dependencies.sort_by(|left, right| {
            left.canonical_key()
                .cmp(&right.canonical_key())
                .then_with(|| left.cmp(right))
        });
        self.roles.sort_by(|left, right| {
            left.canonical_key()
                .cmp(&right.canonical_key())
                .then_with(|| left.cmp(right))
        });
    }

    fn advance_revision(&mut self) -> Result<(), ProjectSourceError> {
        self.revision = next_revision(self.revision, self.root.file_name())?;
        Ok(())
    }

    fn record_revision_snapshot(&mut self, source: &ProjectSourceBundle) {
        let digest = source.closure_digest();
        if self.history.last().is_some_and(|snapshot| {
            snapshot.revision == source.revision && snapshot.closure_digest == digest
        }) {
            return;
        }
        self.history.push(ProjectSourceRevisionSnapshot {
            revision: source.revision,
            closure_digest: digest,
            owner: source.owner.clone(),
            root: source.root.clone(),
            files: source.files.clone(),
            dependencies: source.dependencies.clone(),
            roles: source.roles.clone(),
        });
        let mut retained_bytes = self
            .history
            .iter()
            .map(ProjectSourceRevisionSnapshot::retained_bytes)
            .sum::<usize>();
        while self.history.len() > MAX_SOURCE_HISTORY_REVISIONS
            || (retained_bytes > MAX_SOURCE_HISTORY_BYTES && self.history.len() > 1)
        {
            retained_bytes = retained_bytes.saturating_sub(self.history[0].retained_bytes());
            self.history.remove(0);
        }
    }

    fn invalidate_validation(&mut self) {
        self.validated_identity = None;
        self.root.invalidate_validation();
    }

    /// One-time migration from the pre-role schema. Runtime execution never
    /// calls this heuristic. It exists only while decoding schema v1, after
    /// which the resolved bindings are persisted explicitly in schema v2.
    fn migrate_schema_v1(&mut self) -> Result<(), ProjectSourceError> {
        if self.root.id.is_missing() {
            self.root.id = ProjectSourceDocumentId::migrated(self.id, self.root.logical_path());
        }
        for file in &mut self.files {
            file.revision = self.revision;
            if file.id.is_missing() {
                file.id = ProjectSourceDocumentId::migrated(self.id, file.logical_path());
            }
        }
        if self.language != ProjectSourceLanguage::RSpiceAutomation {
            self.invalidate_validation();
            return Ok(());
        }
        self.roles = default_source_roles(self.language, self.root.logical_path())?;
        if self
            .root
            .logical_path()
            .to_ascii_lowercase()
            .ends_with(".rspice")
        {
            self.invalidate_validation();
            self.canonicalize();
            return Ok(());
        }

        let run_plans = self
            .files
            .iter()
            .filter(|file| {
                let path = file.logical_path().to_ascii_lowercase();
                path.ends_with(".yaml") || path.ends_with(".yml")
            })
            .map(ProjectSourceFile::logical_path)
            .collect::<Vec<_>>();
        for path in run_plans {
            self.roles.push(ProjectSourceRoleBinding::try_new(
                path,
                ProjectSourceRole::AutomationRunPlan,
            )?);
        }
        let locks = self
            .files
            .iter()
            .filter(|file| file.content().contains("rspice-python-lock/"))
            .map(ProjectSourceFile::logical_path)
            .collect::<Vec<_>>();
        if let [path] = locks.as_slice() {
            self.roles.push(ProjectSourceRoleBinding::try_new(
                *path,
                ProjectSourceRole::AutomationEnvironmentLock,
            )?);
        }
        let permission_manifests = self
            .files
            .iter()
            .filter(|file| {
                file.content().lines().any(|line| {
                    let key = line.split('=').next().unwrap_or_default().trim();
                    matches!(
                        key,
                        "project_files" | "artifact_directory" | "network" | "process_spawn"
                    )
                })
            })
            .map(ProjectSourceFile::logical_path)
            .collect::<Vec<_>>();
        if let [path] = permission_manifests.as_slice() {
            self.roles.push(ProjectSourceRoleBinding::try_new(
                *path,
                ProjectSourceRole::AutomationPermissionManifest,
            )?);
        }
        self.invalidate_validation();
        self.canonicalize();
        Ok(())
    }
}

/// Typed registry for every project-owned source closure.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProjectSourceRegistry {
    bundles: Vec<ProjectSourceBundle>,
}

impl ProjectSourceRegistry {
    pub fn try_from_documents(
        documents: impl IntoIterator<Item = ProjectSourceDocument>,
    ) -> Result<Self, ProjectSourceError> {
        let mut registry = Self::default();
        for document in documents {
            registry.insert(document)?;
        }
        registry.validate()?;
        Ok(registry)
    }

    pub fn try_from_bundles(
        bundles: impl IntoIterator<Item = ProjectSourceBundle>,
    ) -> Result<Self, ProjectSourceError> {
        let mut registry = Self {
            bundles: bundles.into_iter().collect(),
        };
        registry.canonicalize();
        registry.validate()?;
        Ok(registry)
    }

    /// Insert a compatibility Code Workspace singleton.
    pub fn insert(&mut self, document: ProjectSourceDocument) -> Result<(), ProjectSourceError> {
        let language = document.language;
        if self.get(language).is_some() {
            return Err(ProjectSourceError::DuplicateLanguage { language });
        }
        self.insert_bundle(ProjectSourceBundle::migrated(document)?)
    }

    pub fn insert_bundle(
        &mut self,
        mut bundle: ProjectSourceBundle,
    ) -> Result<(), ProjectSourceError> {
        bundle.canonicalize();
        bundle.validate()?;
        if self.bundles.iter().any(|current| current.id == bundle.id) {
            return Err(ProjectSourceError::DuplicateIdentity { id: bundle.id });
        }
        let owner_key = bundle.owner.canonical_key();
        if self
            .bundles
            .iter()
            .any(|current| current.owner.canonical_key() == owner_key)
        {
            if let ProjectSourceOwner::CodeWorkspace { language } = bundle.owner {
                return Err(ProjectSourceError::DuplicateLanguage { language });
            }
            return Err(ProjectSourceError::DuplicateOwner {
                owner: owner_key.to_string(),
            });
        }
        self.bundles.push(bundle);
        self.canonicalize();
        Ok(())
    }

    #[must_use]
    pub fn get(&self, language: ProjectSourceLanguage) -> Option<&ProjectSourceDocument> {
        self.code_workspace_bundle(language)
            .map(ProjectSourceBundle::root)
    }

    #[must_use]
    pub fn get_bundle(&self, id: ProjectSourceId) -> Option<&ProjectSourceBundle> {
        self.bundles.iter().find(|bundle| bundle.id == id)
    }

    pub fn restore_bundle_revision(
        &mut self,
        id: ProjectSourceId,
        expected_current: ObjectRevision,
        retained_revision: ObjectRevision,
    ) -> Result<bool, ProjectSourceError> {
        self.bundles
            .iter_mut()
            .find(|bundle| bundle.id == id)
            .ok_or(ProjectSourceError::MissingIdentity { id })?
            .restore_revision(expected_current, retained_revision)
    }

    #[must_use]
    pub fn bundle_for_owner(&self, owner: &ProjectSourceOwner) -> Option<&ProjectSourceBundle> {
        let owner_key = owner.canonical_key();
        self.bundles
            .iter()
            .find(|bundle| bundle.owner.canonical_key() == owner_key)
    }

    pub fn iter(&self) -> impl Iterator<Item = &ProjectSourceDocument> {
        ProjectSourceLanguage::ALL
            .into_iter()
            .filter_map(|language| self.get(language))
    }

    pub fn iter_bundles(&self) -> impl Iterator<Item = &ProjectSourceBundle> {
        self.bundles.iter()
    }

    pub fn remove(&mut self, language: ProjectSourceLanguage) -> Option<ProjectSourceDocument> {
        let index = self
            .bundles
            .iter()
            .position(|bundle| bundle.owner == ProjectSourceOwner::code_workspace(language))?;
        Some(self.bundles.remove(index).root)
    }

    pub fn remove_bundle(&mut self, id: ProjectSourceId) -> Option<ProjectSourceBundle> {
        let index = self.bundles.iter().position(|bundle| bundle.id == id)?;
        Some(self.bundles.remove(index))
    }

    /// Remove every cell-view source owned by the exact library/cell scope.
    /// Supplying a view narrows the operation to that one view. The returned
    /// identities let callers invalidate transient receipts without guessing.
    pub fn remove_cell_view_bundles(
        &mut self,
        library: &str,
        cell: &str,
        view: Option<&str>,
    ) -> Vec<ProjectSourceId> {
        let mut removed = Vec::new();
        self.bundles.retain(|bundle| {
            let matches = matches!(
                bundle.owner(),
                ProjectSourceOwner::CellView { reference }
                    if canonical_cell_view_owner_key(
                        &reference.library,
                        &reference.cell,
                        &reference.view,
                    ) == canonical_cell_view_owner_key(
                        library,
                        cell,
                        view.unwrap_or(&reference.view),
                    )
            );
            if matches {
                removed.push(bundle.id());
            }
            !matches
        });
        removed
    }

    /// Remove cell-view bundles whose owner no longer exists in the project
    /// library/view catalog. Code Workspace sources are independent documents
    /// and are never affected. Returned identities allow callers to invalidate
    /// transient compiler evidence without relying on paths or display names.
    pub fn retain_cell_view_bundles_for(
        &mut self,
        references: impl IntoIterator<Item = CellViewRef>,
    ) -> Vec<ProjectSourceId> {
        let retained = references
            .into_iter()
            .map(|reference| {
                canonical_cell_view_owner_key(&reference.library, &reference.cell, &reference.view)
            })
            .collect::<BTreeSet<_>>();
        let mut removed = Vec::new();
        self.bundles.retain(|bundle| {
            let keep = match bundle.owner() {
                ProjectSourceOwner::CodeWorkspace { .. } => true,
                ProjectSourceOwner::CellView { reference } => {
                    retained.contains(&canonical_cell_view_owner_key(
                        &reference.library,
                        &reference.cell,
                        &reference.view,
                    ))
                }
            };
            if !keep {
                removed.push(bundle.id());
            }
            keep
        });
        removed
    }

    /// Clone every project-owned source associated with one cell into a new
    /// cell identity. Each clone receives a fresh stable identity and loses
    /// validation evidence because owner identity participates in its sealed
    /// closure digest. The registry changes atomically.
    pub fn clone_cell_view_bundles(
        &mut self,
        source_library: &str,
        source_cell: &str,
        target_library: &str,
        target_cell: &str,
    ) -> Result<Vec<ProjectSourceId>, ProjectSourceError> {
        let originals = self
            .bundles
            .iter()
            .filter_map(|bundle| match bundle.owner() {
                ProjectSourceOwner::CellView { reference }
                    if canonical_cell_view_owner_key(
                        &reference.library,
                        &reference.cell,
                        &reference.view,
                    ) == canonical_cell_view_owner_key(
                        source_library,
                        source_cell,
                        &reference.view,
                    ) =>
                {
                    Some((bundle, reference.view.clone()))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        let mut candidate = self.clone();
        let mut inserted = Vec::with_capacity(originals.len());
        for (bundle, view) in originals {
            let cloned_files = bundle
                .files
                .iter()
                .map(|file| {
                    ProjectSourceFile::try_new(file.logical_path.clone(), file.content.clone())
                })
                .collect::<Result<Vec<_>, _>>()?;
            let clone = ProjectSourceBundle::try_new_with_roles(
                ProjectSourceOwner::cell_view(CellViewRef::new(target_library, target_cell, view)),
                bundle.language,
                bundle.root.file_name.clone(),
                bundle.root.content.clone(),
                cloned_files,
                bundle.dependencies.clone(),
                bundle.roles.clone(),
            )?;
            inserted.push(clone.id());
            candidate.insert_bundle(clone)?;
        }
        candidate.validate()?;
        *self = candidate;
        Ok(inserted)
    }

    /// Move cell-view source ownership after a cell rename. Bundle identities
    /// remain stable, while revision/digest/validation advance atomically so
    /// stale compiled artifacts cannot publish under the renamed owner.
    pub fn rename_cell_view_bundles(
        &mut self,
        library: &str,
        old_cell: &str,
        new_cell: &str,
    ) -> Result<Vec<ProjectSourceId>, ProjectSourceError> {
        let mut candidate = self.clone();
        let originals = self
            .bundles
            .iter()
            .map(|bundle| (bundle.id(), bundle.clone()))
            .collect::<BTreeMap<_, _>>();
        let mut changed = Vec::new();
        for bundle in &mut candidate.bundles {
            let matches = match &bundle.owner {
                ProjectSourceOwner::CellView { reference } => {
                    canonical_cell_view_owner_key(
                        &reference.library,
                        &reference.cell,
                        &reference.view,
                    ) == canonical_cell_view_owner_key(library, old_cell, &reference.view)
                }
                ProjectSourceOwner::CodeWorkspace { .. } => false,
            };
            if !matches {
                continue;
            }
            if let Some(original) = originals.get(&bundle.id()) {
                bundle.record_revision_snapshot(original);
            }
            let ProjectSourceOwner::CellView { reference } = &mut bundle.owner else {
                unreachable!("cell-view ownership was checked above")
            };
            reference.cell = new_cell.to_owned();
            bundle.advance_revision()?;
            bundle.invalidate_validation();
            changed.push(bundle.id());
        }
        candidate.canonicalize();
        candidate.validate()?;
        *self = candidate;
        Ok(changed)
    }

    /// Synchronize one cell-view-owned bundle from another registry without
    /// touching Code Workspace or sibling cell-view sources.
    pub fn synchronize_cell_view_bundle_from(
        &mut self,
        reference: &CellViewRef,
        source: &Self,
    ) -> Result<bool, ProjectSourceError> {
        let owner = ProjectSourceOwner::cell_view(reference.clone());
        let replacement = source.bundle_for_owner(&owner).cloned();
        let mut candidate = self.clone();
        candidate.remove_cell_view_bundles(
            &reference.library,
            &reference.cell,
            Some(&reference.view),
        );
        if let Some(bundle) = replacement {
            candidate.insert_bundle(bundle)?;
        }
        candidate.validate()?;
        if candidate == *self {
            return Ok(false);
        }
        *self = candidate;
        Ok(true)
    }

    /// Synchronize only singleton Code Workspace sources. Cell-view bundles
    /// are separate lifecycle documents and are preserved exactly.
    pub fn synchronize_code_workspace_bundles_from(
        &mut self,
        source: &Self,
    ) -> Result<bool, ProjectSourceError> {
        let mut candidate = self.clone();
        candidate
            .bundles
            .retain(|bundle| !matches!(bundle.owner(), ProjectSourceOwner::CodeWorkspace { .. }));
        for bundle in source
            .bundles
            .iter()
            .filter(|bundle| matches!(bundle.owner(), ProjectSourceOwner::CodeWorkspace { .. }))
        {
            candidate.insert_bundle(bundle.clone())?;
        }
        candidate.validate()?;
        if candidate == *self {
            return Ok(false);
        }
        *self = candidate;
        Ok(true)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bundles.is_empty()
    }

    pub fn replace_content(
        &mut self,
        language: ProjectSourceLanguage,
        content: String,
    ) -> Result<bool, ProjectSourceError> {
        let bundle = self
            .code_workspace_bundle_mut(language)
            .ok_or(ProjectSourceError::MissingLanguage { language })?;
        let path = bundle.root.file_name.clone();
        bundle.replace_file_content(&path, content)
    }

    pub fn replace_bundle_file_content(
        &mut self,
        id: ProjectSourceId,
        logical_path: &str,
        content: String,
    ) -> Result<bool, ProjectSourceError> {
        self.bundles
            .iter_mut()
            .find(|bundle| bundle.id == id)
            .ok_or(ProjectSourceError::MissingIdentity { id })?
            .replace_file_content(logical_path, content)
    }

    pub fn set_bundle_non_entry_role(
        &mut self,
        id: ProjectSourceId,
        logical_path: &str,
        role: Option<ProjectSourceRole>,
    ) -> Result<bool, ProjectSourceError> {
        self.bundles
            .iter_mut()
            .find(|bundle| bundle.id == id)
            .ok_or(ProjectSourceError::MissingIdentity { id })?
            .set_non_entry_role(logical_path, role)
    }

    /// Replace multiple documents as one source-graph transaction. Every
    /// candidate is validated on a private registry clone; any missing path,
    /// duplicate path, size violation, or revision failure leaves `self`
    /// byte-for-byte unchanged.
    pub fn replace_bundle_files_transactionally(
        &mut self,
        id: ProjectSourceId,
        replacements: impl IntoIterator<Item = (String, String)>,
    ) -> Result<usize, ProjectSourceError> {
        let mut candidate = self.clone();
        let bundle = candidate
            .bundles
            .iter_mut()
            .find(|bundle| bundle.id == id)
            .ok_or(ProjectSourceError::MissingIdentity { id })?;
        let changed = bundle.replace_files_transactionally(replacements)?;
        if changed == 0 {
            return Ok(0);
        }
        candidate.validate()?;
        *self = candidate;
        Ok(changed)
    }

    pub fn add_bundle_file(
        &mut self,
        id: ProjectSourceId,
        importer: &str,
        file: ProjectSourceFile,
    ) -> Result<bool, ProjectSourceError> {
        self.bundles
            .iter_mut()
            .find(|bundle| bundle.id == id)
            .ok_or(ProjectSourceError::MissingIdentity { id })?
            .add_file(importer, file)
    }

    pub fn add_bundle_file_with_role(
        &mut self,
        id: ProjectSourceId,
        importer: &str,
        file: ProjectSourceFile,
        role: ProjectSourceRole,
    ) -> Result<bool, ProjectSourceError> {
        self.bundles
            .iter_mut()
            .find(|bundle| bundle.id == id)
            .ok_or(ProjectSourceError::MissingIdentity { id })?
            .add_file_with_role(importer, file, role)
    }

    pub fn append_bundle_qualification(
        &mut self,
        id: ProjectSourceId,
        record: ProjectSourceQualificationRecord,
    ) -> Result<u64, ProjectSourceError> {
        self.bundles
            .iter_mut()
            .find(|bundle| bundle.id == id)
            .ok_or(ProjectSourceError::MissingIdentity { id })?
            .append_qualification(record)
    }

    pub fn rename_bundle_file(
        &mut self,
        id: ProjectSourceId,
        current_path: &str,
        new_path: &str,
    ) -> Result<bool, ProjectSourceError> {
        self.bundles
            .iter_mut()
            .find(|bundle| bundle.id == id)
            .ok_or(ProjectSourceError::MissingIdentity { id })?
            .rename_file(current_path, new_path)
    }

    pub fn remove_bundle_file(
        &mut self,
        id: ProjectSourceId,
        logical_path: &str,
    ) -> Result<bool, ProjectSourceError> {
        self.bundles
            .iter_mut()
            .find(|bundle| bundle.id == id)
            .ok_or(ProjectSourceError::MissingIdentity { id })?
            .remove_file(logical_path)
    }

    pub fn replace_imported(
        &mut self,
        language: ProjectSourceLanguage,
        file_name: String,
        content: String,
    ) -> Result<bool, ProjectSourceError> {
        match self.code_workspace_bundle_mut(language) {
            Some(bundle) => bundle.replace_root_imported(file_name, content),
            None => {
                self.insert(ProjectSourceDocument::try_new(
                    file_name, language, content,
                )?)?;
                Ok(true)
            }
        }
    }

    pub fn mark_validated(
        &mut self,
        language: ProjectSourceLanguage,
    ) -> Result<ProjectSourceValidationIdentity, ProjectSourceError> {
        let bundle = self
            .code_workspace_bundle_mut(language)
            .ok_or(ProjectSourceError::MissingLanguage { language })?;
        bundle.mark_validated()?;
        Ok(bundle
            .root
            .validated_identity
            .expect("marking a bundle validates its root"))
    }

    pub fn mark_bundle_validated(
        &mut self,
        id: ProjectSourceId,
    ) -> Result<ProjectSourceValidationIdentity, ProjectSourceError> {
        self.bundles
            .iter_mut()
            .find(|bundle| bundle.id == id)
            .ok_or(ProjectSourceError::MissingIdentity { id })?
            .mark_validated()
    }

    pub fn validate(&self) -> Result<(), ProjectSourceError> {
        let mut ids = BTreeSet::new();
        let mut owners = BTreeSet::new();
        for bundle in &self.bundles {
            bundle.validate()?;
            if !ids.insert(bundle.id) {
                return Err(ProjectSourceError::DuplicateIdentity { id: bundle.id });
            }
            let owner = bundle.owner.canonical_key();
            if !owners.insert(owner.clone()) {
                if let ProjectSourceOwner::CodeWorkspace { language } = bundle.owner {
                    return Err(ProjectSourceError::DuplicateLanguage { language });
                }
                return Err(ProjectSourceError::DuplicateOwner {
                    owner: owner.to_string(),
                });
            }
        }
        Ok(())
    }

    fn code_workspace_bundle(
        &self,
        language: ProjectSourceLanguage,
    ) -> Option<&ProjectSourceBundle> {
        self.bundles
            .iter()
            .find(|bundle| bundle.owner == ProjectSourceOwner::code_workspace(language))
    }

    fn code_workspace_bundle_mut(
        &mut self,
        language: ProjectSourceLanguage,
    ) -> Option<&mut ProjectSourceBundle> {
        self.bundles
            .iter_mut()
            .find(|bundle| bundle.owner == ProjectSourceOwner::code_workspace(language))
    }

    fn canonicalize(&mut self) {
        for bundle in &mut self.bundles {
            bundle.canonicalize();
        }
        self.bundles.sort_by(|left, right| {
            left.owner
                .canonical_key()
                .cmp(&right.owner.canonical_key())
                .then_with(|| left.id.cmp(&right.id))
        });
    }
}

/// Canonical virtual source key for one selected module in a sealed project
/// Verilog-A bundle. This is the single identity authority shared by hierarchy
/// resolution, generated netlists, prepared execution, and runtime caching.
pub fn project_veriloga_bundle_source_key(
    project_id: ProjectId,
    bundle: &ProjectSourceBundle,
    module_name: &str,
) -> Result<String, ProjectSourceError> {
    validate_veriloga_runtime_selection(bundle, module_name)?;
    Ok(format!(
        "__rspice_project__/{project_id}/{}/{}/{}/{}",
        bundle.id(),
        bundle.closure_digest(),
        selected_module_digest(module_name),
        bundle.root().logical_path()
    ))
}

/// Collision-resistant, portable SPICE identifier for one selected module in
/// a project Verilog-A bundle. It deliberately contains no raw user text: the
/// complete bundle UUID and complete selected-module digest preserve identity
/// while every emitted byte remains `[A-Za-z0-9_]`.
pub fn project_veriloga_bundle_alias(
    bundle: &ProjectSourceBundle,
    module_name: &str,
) -> Result<String, ProjectSourceError> {
    validate_veriloga_runtime_selection(bundle, module_name)?;
    let id = bundle.id().to_string().replace('-', "");
    Ok(format!(
        "rspice_va_{id}_{}",
        selected_module_digest(module_name)
    ))
}

fn validate_veriloga_runtime_selection(
    bundle: &ProjectSourceBundle,
    module_name: &str,
) -> Result<(), ProjectSourceError> {
    bundle.validate()?;
    if bundle.language() != ProjectSourceLanguage::VerilogA {
        return Err(ProjectSourceError::UnsupportedRuntimeLanguage {
            language: bundle.language(),
        });
    }
    let mut characters = module_name.chars();
    let valid = characters
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        && characters
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '$'));
    if !valid {
        return Err(ProjectSourceError::InvalidModuleName {
            module_name: module_name.to_owned(),
        });
    }
    Ok(())
}

fn selected_module_digest(module_name: &str) -> ContentDigest {
    let mut hasher = Sha256::new();
    hash_frame(&mut hasher, b"rspice.project-veriloga-selected-module/v1");
    hash_frame(&mut hasher, module_name.as_bytes());
    ContentDigest::from_bytes(hasher.finalize().into())
}

#[derive(Serialize)]
#[serde(deny_unknown_fields)]
struct ProjectSourceRegistryRef<'a> {
    schema_version: u16,
    bundles: &'a [ProjectSourceBundle],
}

impl Serialize for ProjectSourceRegistry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ProjectSourceRegistryRef {
            schema_version: PROJECT_SOURCE_REGISTRY_SCHEMA_VERSION,
            bundles: &self.bundles,
        }
        .serialize(serializer)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProjectSourceRegistryCanonical {
    schema_version: u16,
    bundles: Vec<ProjectSourceBundle>,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct ProjectSourceRegistryLegacy {
    #[serde(default)]
    verilog_a: Option<ProjectSourceDocument>,
    #[serde(default)]
    automation: Option<ProjectSourceDocument>,
}

impl<'de> Deserialize<'de> for ProjectSourceRegistry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let is_canonical = value.as_object().is_some_and(|object| {
            object.contains_key("schema_version") || object.contains_key("bundles")
        });
        let mut registry = if is_canonical {
            let persisted: ProjectSourceRegistryCanonical =
                serde_json::from_value(value).map_err(D::Error::custom)?;
            if !matches!(
                persisted.schema_version,
                1 | PROJECT_SOURCE_REGISTRY_SCHEMA_VERSION
            ) {
                return Err(D::Error::custom(
                    ProjectSourceError::UnsupportedSchemaVersion {
                        found: persisted.schema_version,
                        supported: PROJECT_SOURCE_REGISTRY_SCHEMA_VERSION,
                    },
                ));
            }
            let mut registry = Self {
                bundles: persisted.bundles,
            };
            if persisted.schema_version == 1 {
                for bundle in &mut registry.bundles {
                    bundle.migrate_schema_v1().map_err(D::Error::custom)?;
                }
            }
            registry
        } else {
            let legacy: ProjectSourceRegistryLegacy =
                serde_json::from_value(value).map_err(D::Error::custom)?;
            let mut bundles = Vec::new();
            for (expected, document) in [
                (ProjectSourceLanguage::VerilogA, legacy.verilog_a),
                (ProjectSourceLanguage::RSpiceAutomation, legacy.automation),
            ] {
                if let Some(document) = document {
                    if document.language != expected {
                        return Err(D::Error::custom(
                            ProjectSourceError::RegistryLanguageMismatch {
                                slot: expected,
                                document: document.language,
                            },
                        ));
                    }
                    bundles
                        .push(ProjectSourceBundle::migrated(document).map_err(D::Error::custom)?);
                }
            }
            Self { bundles }
        };
        registry.canonicalize();
        registry.validate().map_err(D::Error::custom)?;
        Ok(registry)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProjectSourceError {
    #[error("source document '{logical_path}' has no stable identity")]
    MissingDocumentIdentity { logical_path: String },
    #[error("source document identity {id} is duplicated by '{logical_path}'")]
    DuplicateDocumentIdentity {
        id: ProjectSourceDocumentId,
        logical_path: String,
    },
    #[error("{language} source file name is empty or not trimmed")]
    InvalidFileNameWhitespace { language: ProjectSourceLanguage },
    #[error("{language} source logical path is absolute, traversing, or not normalized")]
    InvalidFileNamePath { language: ProjectSourceLanguage },
    #[error("{language} source file name contains a reserved or non-portable character")]
    InvalidFileNameCharacters { language: ProjectSourceLanguage },
    #[error("{language} source file name is reserved by a supported desktop platform")]
    ReservedFileName { language: ProjectSourceLanguage },
    #[error("{language} source file name must end in '{required_extension}'")]
    InvalidFileNameExtension {
        language: ProjectSourceLanguage,
        required_extension: &'static str,
    },
    #[error("logical source path '{logical_path}' is absolute, traversing, or not normalized")]
    InvalidLogicalPath { logical_path: String },
    #[error("logical source path '{logical_path}' contains a non-portable character")]
    InvalidLogicalPathCharacter { logical_path: String },
    #[error("logical source path '{logical_path}' contains a reserved platform name")]
    ReservedLogicalPath { logical_path: String },
    #[error("logical source path is {bytes} bytes; the supported limit is {limit} bytes")]
    LogicalPathTooLong { bytes: usize, limit: usize },
    #[error("source file '{file_name}' contains a NUL byte")]
    NulInContent { file_name: String },
    #[error("source file '{file_name}' is {bytes} bytes; the supported limit is {limit} bytes")]
    SourceTooLarge {
        file_name: String,
        bytes: usize,
        limit: usize,
    },
    #[error("source bundle is {bytes} bytes; the supported limit is {limit} bytes")]
    BundleTooLarge { bytes: usize, limit: usize },
    #[error("source bundle has {files} files; the supported limit is {limit}")]
    TooManyFiles { files: usize, limit: usize },
    #[error("source bundle has {dependencies} dependency edges; the supported limit is {limit}")]
    TooManyDependencies { dependencies: usize, limit: usize },
    #[error("source bundle repeats logical path '{logical_path}' case-insensitively")]
    DuplicateLogicalPath { logical_path: String },
    #[error("source bundle files must be strictly sorted by portable logical path")]
    UnsortedFiles,
    #[error("source dependency references missing logical path '{logical_path}'")]
    MissingDependencyEndpoint { logical_path: String },
    #[error("source dependency '{importer}' -> '{imported}' is duplicated case-insensitively")]
    DuplicateDependency { importer: String, imported: String },
    #[error("source dependencies must be strictly sorted and unique")]
    UnsortedDependencies,
    #[error("source dependency graph contains a cycle at '{logical_path}'")]
    DependencyCycle { logical_path: String },
    #[error("source dependency graph exceeds depth {limit} at '{logical_path}'")]
    DependencyDepthExceeded { logical_path: String, limit: usize },
    #[error("source file '{logical_path}' is not reachable from the bundle root")]
    UnreachableFile { logical_path: String },
    #[error("source bundle has no file named '{logical_path}'")]
    MissingFile { logical_path: String },
    #[error("source bundle root '{logical_path}' cannot be deleted; rename or replace it instead")]
    CannotRemoveBundleRoot { logical_path: String },
    #[error(
        "source file '{logical_path}' still imports '{dependency}'; remove its dependencies first"
    )]
    FileHasDependencies {
        logical_path: String,
        dependency: String,
    },
    #[error("{role:?} role is not valid for {language} source bundles")]
    RoleNotAllowedForLanguage {
        role: ProjectSourceRole,
        language: ProjectSourceLanguage,
    },
    #[error("{role:?} role references missing source file '{logical_path}'")]
    MissingRoleEndpoint {
        logical_path: String,
        role: ProjectSourceRole,
    },
    #[error("source file '{logical_path}' has more than one semantic role")]
    FileHasMultipleRoles { logical_path: String },
    #[error("source role bindings must be strictly sorted and unique")]
    UnsortedRoles,
    #[error("Automation bundle must contain exactly one entry role bound to its root")]
    MissingAutomationEntryRole,
    #[error("Automation entry '{logical_path}' must be the bundle root '{root_path}'")]
    AutomationEntryMustBeRoot {
        logical_path: String,
        root_path: String,
    },
    #[error("{role:?} is already bound to '{first}' and cannot also bind '{second}'")]
    DuplicateSingletonRole {
        role: ProjectSourceRole,
        first: String,
        second: String,
    },
    #[error("source file '{logical_path}' already has the {role:?} role")]
    FileAlreadyHasRole {
        logical_path: String,
        role: ProjectSourceRole,
    },
    #[error("source file '{logical_path}' is bound as {role:?}; unbind it before deletion")]
    RoleBoundFile {
        logical_path: String,
        role: ProjectSourceRole,
    },
    #[error("Automation entry role for '{logical_path}' cannot be unbound")]
    CannotUnbindAutomationEntry { logical_path: String },
    #[error("source file '{file_name}' has stale validation evidence")]
    StaleValidationIdentity { file_name: String },
    #[error("source bundle {id} has stale validation evidence")]
    StaleBundleValidationIdentity { id: ProjectSourceId },
    #[error("source revision {revision} is not retained in the bounded project history")]
    MissingRetainedRevision { revision: u64 },
    #[error(
        "source revision restore expected current revision {expected}, but found revision {found}"
    )]
    StaleRevisionRestore { expected: u64, found: u64 },
    #[error("retained source revision {revision} has a corrupted closure digest")]
    CorruptRetainedRevision { revision: u64 },
    #[error("retained source history exceeds its bounded storage contract")]
    RetainedHistoryLimitExceeded,
    #[error("project source qualification history exceeds its retained-record limit")]
    QualificationHistoryLimitExceeded,
    #[error("project source qualification record is malformed or out of order")]
    InvalidQualificationRecord,
    #[error("project source qualification evidence does not match the current source identity")]
    QualificationIdentityMismatch,
    #[error("source file '{file_name}' exhausted its revision space")]
    RevisionExhausted { file_name: String },
    #[error("the project already owns a {language} Code Workspace source")]
    DuplicateLanguage { language: ProjectSourceLanguage },
    #[error("the project has no {language} Code Workspace source")]
    MissingLanguage { language: ProjectSourceLanguage },
    #[error("project source identity {id} is duplicated")]
    DuplicateIdentity { id: ProjectSourceId },
    #[error("the project has no source bundle with identity {id}")]
    MissingIdentity { id: ProjectSourceId },
    #[error("project source owner '{owner}' is duplicated")]
    DuplicateOwner { owner: String },
    #[error("the {slot} registry slot contains a {document} document")]
    RegistryLanguageMismatch {
        slot: ProjectSourceLanguage,
        document: ProjectSourceLanguage,
    },
    #[error("source owner declares {owner}, but its bundle declares {bundle}")]
    OwnerLanguageMismatch {
        owner: ProjectSourceLanguage,
        bundle: ProjectSourceLanguage,
    },
    #[error("source bundle declares {bundle}, but its root declares {root}")]
    BundleLanguageMismatch {
        bundle: ProjectSourceLanguage,
        root: ProjectSourceLanguage,
    },
    #[error("cell-view source owner '{reference}' is invalid: {message}")]
    InvalidCellViewOwner { reference: String, message: String },
    #[error("cell-view source bundles do not support {language}")]
    UnsupportedCellViewLanguage { language: ProjectSourceLanguage },
    #[error("{language} source bundles cannot produce a Verilog-A runtime identity")]
    UnsupportedRuntimeLanguage { language: ProjectSourceLanguage },
    #[error("Verilog-A selected module {module_name:?} is not a valid ASCII module identifier")]
    InvalidModuleName { module_name: String },
    #[error("project source registry schema {found} is unsupported; expected {supported}")]
    UnsupportedSchemaVersion { found: u16, supported: u16 },
}

fn next_revision(
    revision: ObjectRevision,
    file_name: &str,
) -> Result<ObjectRevision, ProjectSourceError> {
    revision
        .next()
        .map_err(|_| ProjectSourceError::RevisionExhausted {
            file_name: file_name.to_owned(),
        })
}

fn validate_source_content(file_name: &str, content: &str) -> Result<(), ProjectSourceError> {
    if content.contains('\0') {
        return Err(ProjectSourceError::NulInContent {
            file_name: file_name.to_owned(),
        });
    }
    if content.len() > MAX_PROJECT_CODE_SOURCE_BYTES {
        return Err(ProjectSourceError::SourceTooLarge {
            file_name: file_name.to_owned(),
            bytes: content.len(),
            limit: MAX_PROJECT_CODE_SOURCE_BYTES,
        });
    }
    Ok(())
}

fn default_source_roles(
    language: ProjectSourceLanguage,
    root_path: &str,
) -> Result<Vec<ProjectSourceRoleBinding>, ProjectSourceError> {
    if language == ProjectSourceLanguage::RSpiceAutomation {
        Ok(vec![ProjectSourceRoleBinding::try_new(
            root_path,
            ProjectSourceRole::AutomationEntry,
        )?])
    } else {
        Ok(Vec::new())
    }
}

const fn source_role_stable_name(role: ProjectSourceRole) -> &'static str {
    match role {
        ProjectSourceRole::VerilogABuildProfile => "veriloga-build-profile",
        ProjectSourceRole::AutomationEntry => "automation-entry",
        ProjectSourceRole::AutomationRunPlan => "automation-run-plan",
        ProjectSourceRole::AutomationEnvironmentLock => "automation-environment-lock",
        ProjectSourceRole::AutomationPermissionManifest => "automation-permission-manifest",
    }
}

fn validate_project_source_file_name(
    file_name: &str,
    language: ProjectSourceLanguage,
) -> Result<(), ProjectSourceError> {
    if file_name.is_empty()
        || file_name != file_name.trim()
        || file_name.chars().any(char::is_control)
    {
        return Err(ProjectSourceError::InvalidFileNameWhitespace { language });
    }
    validate_logical_path(file_name).map_err(|error| match error {
        ProjectSourceError::InvalidLogicalPath { .. }
        | ProjectSourceError::LogicalPathTooLong { .. } => {
            ProjectSourceError::InvalidFileNamePath { language }
        }
        ProjectSourceError::InvalidLogicalPathCharacter { .. } => {
            ProjectSourceError::InvalidFileNameCharacters { language }
        }
        ProjectSourceError::ReservedLogicalPath { .. } => {
            ProjectSourceError::ReservedFileName { language }
        }
        other => other,
    })?;
    let required_extension = language.required_extension();
    let leaf = file_name.rsplit('/').next().unwrap_or_default();
    let lowercase = leaf.to_ascii_lowercase();
    // Version-one projects used a single `.rspice` workflow document. Keep
    // those exact sources loadable so the workspace can offer an explicit,
    // source-preserving migration to the Python/YAML/TOML bundle. New source
    // roots use the qualified `.py` contract exclusively.
    let legacy_automation = language == ProjectSourceLanguage::RSpiceAutomation
        && lowercase.ends_with(".rspice")
        && leaf.len() > ".rspice".len();
    if (!lowercase.ends_with(required_extension) || leaf.len() == required_extension.len())
        && !legacy_automation
    {
        return Err(ProjectSourceError::InvalidFileNameExtension {
            language,
            required_extension,
        });
    }
    Ok(())
}

fn validate_logical_path(logical_path: &str) -> Result<(), ProjectSourceError> {
    if logical_path.is_empty()
        || logical_path != logical_path.trim()
        || logical_path.starts_with('/')
        || logical_path.starts_with('\\')
        || logical_path.contains('\\')
        || logical_path.chars().nth(1) == Some(':')
        || logical_path.len() > MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES
    {
        if logical_path.len() > MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES {
            return Err(ProjectSourceError::LogicalPathTooLong {
                bytes: logical_path.len(),
                limit: MAX_PROJECT_SOURCE_LOGICAL_PATH_BYTES,
            });
        }
        return Err(ProjectSourceError::InvalidLogicalPath {
            logical_path: logical_path.to_owned(),
        });
    }
    for component in logical_path.split('/') {
        if component.is_empty()
            || matches!(component, "." | "..")
            || component != component.trim()
            || component.ends_with('.')
        {
            return Err(ProjectSourceError::InvalidLogicalPath {
                logical_path: logical_path.to_owned(),
            });
        }
        if component.chars().any(|character| {
            character.is_control()
                || matches!(character, '"' | '\'' | ':' | '*' | '?' | '<' | '>' | '|')
        }) {
            return Err(ProjectSourceError::InvalidLogicalPathCharacter {
                logical_path: logical_path.to_owned(),
            });
        }
        if is_reserved_platform_component(component) {
            return Err(ProjectSourceError::ReservedLogicalPath {
                logical_path: logical_path.to_owned(),
            });
        }
    }
    Ok(())
}

fn is_reserved_platform_component(component: &str) -> bool {
    let portable_stem = component
        .split('.')
        .next()
        .unwrap_or_default()
        .to_ascii_uppercase();
    matches!(portable_stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || portable_stem
            .strip_prefix("COM")
            .and_then(|suffix| suffix.parse::<u8>().ok())
            .is_some_and(|number| (1..=9).contains(&number))
        || portable_stem
            .strip_prefix("LPT")
            .and_then(|suffix| suffix.parse::<u8>().ok())
            .is_some_and(|number| (1..=9).contains(&number))
}

fn path_key(path: &str) -> String {
    path.to_lowercase()
}

pub(crate) fn project_source_path_key(path: &str) -> String {
    path_key(path)
}

pub(crate) fn project_source_paths_equal(first: &str, second: &str) -> bool {
    path_key(first) == path_key(second)
}

fn logical_parent(path: &str) -> Option<&str> {
    path.rsplit_once('/').map(|(parent, _)| parent)
}

fn include_resolves_to(importer: &str, requested: &str, imported: &str) -> bool {
    if requested.eq_ignore_ascii_case(imported) {
        return true;
    }
    logical_parent(importer)
        .is_some_and(|parent| format!("{parent}/{requested}").eq_ignore_ascii_case(imported))
}

fn rewrite_matching_include_lines(
    source: &str,
    importer: &str,
    imported: &str,
    replacement: Option<&str>,
) -> String {
    let mut output = String::with_capacity(source.len());
    for segment in source.split_inclusive('\n') {
        let (line, ending) = segment
            .strip_suffix("\r\n")
            .map(|line| (line, "\r\n"))
            .or_else(|| segment.strip_suffix('\n').map(|line| (line, "\n")))
            .unwrap_or((segment, ""));
        let trimmed = line.trim_start();
        let Some(rest) = trimmed.strip_prefix("`include") else {
            output.push_str(segment);
            continue;
        };
        let rest = rest.trim_start();
        let Some(quote) = rest
            .chars()
            .next()
            .filter(|quote| matches!(quote, '\'' | '"'))
        else {
            output.push_str(segment);
            continue;
        };
        let rest = &rest[quote.len_utf8()..];
        let Some(end) = rest.find(quote) else {
            output.push_str(segment);
            continue;
        };
        let requested = &rest[..end];
        if !include_resolves_to(importer, requested, imported) {
            output.push_str(segment);
            continue;
        }
        if let Some(replacement) = replacement {
            let leading = &line[..line.len() - trimmed.len()];
            output.push_str(leading);
            output.push_str("`include \"");
            output.push_str(replacement);
            output.push('"');
            output.push_str(&rest[end + quote.len_utf8()..]);
            output.push_str(ending);
        }
    }
    output
}

fn validate_dependency_graph(
    root: &str,
    graph: &BTreeMap<String, Vec<String>>,
) -> Result<(), ProjectSourceError> {
    fn visit(
        node: &str,
        depth: usize,
        graph: &BTreeMap<String, Vec<String>>,
        colors: &mut BTreeMap<String, u8>,
    ) -> Result<(), ProjectSourceError> {
        if depth > MAX_PROJECT_SOURCE_DEPENDENCY_DEPTH {
            return Err(ProjectSourceError::DependencyDepthExceeded {
                logical_path: node.to_owned(),
                limit: MAX_PROJECT_SOURCE_DEPENDENCY_DEPTH,
            });
        }
        match colors.get(node).copied().unwrap_or_default() {
            1 => {
                return Err(ProjectSourceError::DependencyCycle {
                    logical_path: node.to_owned(),
                });
            }
            2 => return Ok(()),
            _ => {}
        }
        colors.insert(node.to_owned(), 1);
        if let Some(children) = graph.get(node) {
            for child in children {
                visit(child, depth + 1, graph, colors)?;
            }
        }
        colors.insert(node.to_owned(), 2);
        Ok(())
    }

    let mut colors = BTreeMap::new();
    for node in graph.keys() {
        visit(node, 0, graph, &mut colors)?;
    }

    let mut reachable = BTreeSet::new();
    let mut pending = vec![root.to_owned()];
    while let Some(node) = pending.pop() {
        if !reachable.insert(node.clone()) {
            continue;
        }
        if let Some(children) = graph.get(&node) {
            pending.extend(children.iter().cloned());
        }
    }
    if let Some(unreachable) = graph.keys().find(|path| !reachable.contains(*path)) {
        return Err(ProjectSourceError::UnreachableFile {
            logical_path: unreachable.clone(),
        });
    }
    Ok(())
}

fn append_frame(target: &mut Vec<u8>, bytes: &[u8]) {
    target.extend_from_slice(&(bytes.len() as u64).to_be_bytes());
    target.extend_from_slice(bytes);
}

fn hash_frame(hasher: &mut Sha256, bytes: &[u8]) {
    hasher.update((bytes.len() as u64).to_be_bytes());
    hasher.update(bytes);
}

fn hash_u64(hasher: &mut Sha256, value: u64) {
    hasher.update(value.to_be_bytes());
}

#[cfg(test)]
mod tests;
