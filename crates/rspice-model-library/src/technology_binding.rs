//! Portable model-source attachment and exact signed-package pin contracts.

use serde::{Deserialize, Serialize};
use sha2::Digest as _;
use std::path::{Path, PathBuf};

/// Persisted schema for an exact project-owned technology binding.
pub const PROJECT_TECHNOLOGY_BINDING_SCHEMA_VERSION: u16 = 1;
pub const PROJECT_SIGNED_TECHNOLOGY_PIN_SCHEMA_VERSION: u16 = 1;

/// Persisted, content-addressed project pin to one signed PDK package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectSignedTechnologyPin {
    schema_version: u16,
    package_id: String,
    revision: String,
    manifest_digest: rspice_app_types::product::ContentDigest,
    archive_digest: rspice_app_types::product::ContentDigest,
    technology_name: String,
    publisher_id: String,
    signing_key_id: String,
    process_node_nm: u32,
    stack_name: String,
    execution_targets: Vec<crate::pdk::contracts::PdkExecutionTarget>,
}

impl ProjectSignedTechnologyPin {
    pub fn from_package_metadata(
        package: &crate::pdk::package::PdkTechnologyPackageMetadata,
    ) -> Result<Self, TechnologyBindingError> {
        let manifest = package.manifest();
        let mut execution_targets = manifest.compatibility.targets.clone();
        execution_targets.sort();
        let pin = Self {
            schema_version: PROJECT_SIGNED_TECHNOLOGY_PIN_SCHEMA_VERSION,
            package_id: manifest.package_id.clone(),
            revision: manifest.revision.clone(),
            manifest_digest: package.manifest_digest(),
            archive_digest: package.archive_digest(),
            technology_name: manifest.technology_name.clone(),
            publisher_id: manifest.publisher_id.clone(),
            signing_key_id: manifest.signing_key_id.clone(),
            process_node_nm: manifest.process_node_nm,
            stack_name: manifest.stack_name.clone(),
            execution_targets,
        };
        pin.validate()?;
        Ok(pin)
    }

    pub fn validate(&self) -> Result<(), TechnologyBindingError> {
        if self.schema_version != PROJECT_SIGNED_TECHNOLOGY_PIN_SCHEMA_VERSION {
            return Err(TechnologyBindingError::UnsupportedSignedPackageSchema {
                found: self.schema_version,
                supported: PROJECT_SIGNED_TECHNOLOGY_PIN_SCHEMA_VERSION,
            });
        }
        for (field, value) in [
            ("signed_package.package_id", self.package_id.as_str()),
            ("signed_package.revision", self.revision.as_str()),
            (
                "signed_package.technology_name",
                self.technology_name.as_str(),
            ),
            ("signed_package.publisher_id", self.publisher_id.as_str()),
            (
                "signed_package.signing_key_id",
                self.signing_key_id.as_str(),
            ),
            ("signed_package.stack_name", self.stack_name.as_str()),
        ] {
            validate_technology_text(field, value)?;
        }
        if self.process_node_nm == 0 {
            return Err(TechnologyBindingError::InvalidSignedPackage(
                "process node must be greater than zero".to_owned(),
            ));
        }
        if self.execution_targets.is_empty() {
            return Err(TechnologyBindingError::InvalidSignedPackage(
                "at least one execution target is required".to_owned(),
            ));
        }
        for index in 1..self.execution_targets.len() {
            if self.execution_targets[index - 1] >= self.execution_targets[index] {
                return Err(TechnologyBindingError::InvalidSignedPackage(
                    "execution targets must be strictly sorted and unique".to_owned(),
                ));
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn package_id(&self) -> &str {
        &self.package_id
    }

    #[must_use]
    pub fn revision(&self) -> &str {
        &self.revision
    }

    #[must_use]
    pub const fn manifest_digest(&self) -> rspice_app_types::product::ContentDigest {
        self.manifest_digest
    }

    #[must_use]
    pub const fn archive_digest(&self) -> rspice_app_types::product::ContentDigest {
        self.archive_digest
    }

    #[must_use]
    pub fn technology_name(&self) -> &str {
        &self.technology_name
    }

    #[must_use]
    pub fn publisher_id(&self) -> &str {
        &self.publisher_id
    }

    #[must_use]
    pub fn signing_key_id(&self) -> &str {
        &self.signing_key_id
    }

    #[must_use]
    pub const fn process_node_nm(&self) -> u32 {
        self.process_node_nm
    }

    #[must_use]
    pub fn stack_name(&self) -> &str {
        &self.stack_name
    }

    #[must_use]
    pub fn display_label(&self) -> String {
        format!("{} {}", self.package_id, self.revision)
    }
}

/// Exact project-owned attachment to a content-pinned model technology and an
/// optional immutable signed PDK package revision. Runtime use requires the
/// signed pin to resolve against the currently trusted registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectTechnologyBinding {
    schema_version: u16,
    package_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    package_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    technology_node: Option<String>,
    model_library: String,
    root_source: PathBuf,
    source_closure: Vec<crate::ModelSourcePin>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    source_edges: Vec<crate::ModelSourceEdge>,
    model_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    process_sections: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    signed_package: Option<ProjectSignedTechnologyPin>,
}

impl ProjectTechnologyBinding {
    pub fn from_model_library(
        library: &crate::ModelLibrary,
    ) -> Result<Self, TechnologyBindingError> {
        validate_retained_model_sources(library)?;
        let root_source = library
            .root_path
            .clone()
            .ok_or(TechnologyBindingError::MissingRootSource)?;
        let package_name = if library.pdk_name.trim().is_empty() {
            library.name.clone()
        } else {
            library.pdk_name.clone()
        };
        let package_version = nonempty_owned(&library.version);
        let technology_node = nonempty_owned(&library.technology_node);
        let mut process_sections = library.corners.keys().cloned().collect::<Vec<_>>();
        process_sections.sort();
        let binding = Self {
            schema_version: PROJECT_TECHNOLOGY_BINDING_SCHEMA_VERSION,
            package_name,
            package_version,
            technology_node,
            model_library: library.name.clone(),
            root_source,
            source_closure: library.source_closure.clone(),
            source_edges: library.source_edges.clone(),
            model_count: library.models.len(),
            process_sections,
            signed_package: None,
        };
        binding.validate()?;
        Ok(binding)
    }

    pub fn validate(&self) -> Result<(), TechnologyBindingError> {
        if self.schema_version != PROJECT_TECHNOLOGY_BINDING_SCHEMA_VERSION {
            return Err(TechnologyBindingError::UnsupportedSchema {
                found: self.schema_version,
                supported: PROJECT_TECHNOLOGY_BINDING_SCHEMA_VERSION,
            });
        }
        validate_technology_text("package_name", &self.package_name)?;
        validate_technology_text("model_library", &self.model_library)?;
        if let Some(version) = &self.package_version {
            validate_technology_text("package_version", version)?;
        }
        if let Some(node) = &self.technology_node {
            validate_technology_text("technology_node", node)?;
        }
        if self.root_source.as_os_str().is_empty() {
            return Err(TechnologyBindingError::MissingRootSource);
        }
        if !crate::is_portable_absolute_path(&self.root_source) {
            return Err(TechnologyBindingError::NonAbsoluteSource(
                self.root_source.clone(),
            ));
        }
        if self.source_closure.is_empty() {
            return Err(TechnologyBindingError::EmptySourceClosure);
        }
        let mut paths = std::collections::HashSet::with_capacity(self.source_closure.len());
        for (index, source) in self.source_closure.iter().enumerate() {
            if !crate::is_portable_absolute_path(&source.path) {
                return Err(TechnologyBindingError::NonAbsoluteSource(
                    source.path.clone(),
                ));
            }
            if !paths.insert(source.path.clone()) {
                return Err(TechnologyBindingError::DuplicateSource(source.path.clone()));
            }
            if index > 0 && self.source_closure[index - 1].path >= source.path {
                return Err(TechnologyBindingError::UnsortedSourceClosure);
            }
        }
        if !paths.contains(&self.root_source) {
            return Err(TechnologyBindingError::RootAbsentFromClosure(
                self.root_source.clone(),
            ));
        }
        if self.source_closure.len() > 1 && self.source_edges.is_empty() {
            return Err(TechnologyBindingError::MissingSourceEdges);
        }
        for (index, edge) in self.source_edges.iter().enumerate() {
            if index > 0 && self.source_edges[index - 1] >= *edge {
                return Err(TechnologyBindingError::UnsortedSourceEdges);
            }
            if !paths.contains(&edge.owner) || !paths.contains(&edge.target) {
                return Err(TechnologyBindingError::SourceEdgeOutsideClosure);
            }
            rspice_core::netlist::normalize_source_path_literal(&edge.requested_path)
                .map_err(|_| TechnologyBindingError::InvalidSourceEdge)?;
        }
        if let Some(unreachable) = crate::first_unreachable_source(
            &self.root_source,
            &self.source_closure,
            &self.source_edges,
        ) {
            return Err(TechnologyBindingError::UnreachableSource(
                unreachable.to_path_buf(),
            ));
        }
        if self.model_count == 0 {
            return Err(TechnologyBindingError::NoModels);
        }
        for (index, section) in self.process_sections.iter().enumerate() {
            validate_technology_text("process_sections", section)?;
            if index > 0 && self.process_sections[index - 1] >= *section {
                return Err(TechnologyBindingError::UnsortedProcessSections);
            }
        }
        if let Some(pin) = &self.signed_package {
            pin.validate()?;
        }
        Ok(())
    }

    #[must_use]
    pub fn display_label(&self) -> String {
        let mut label = self.package_name.clone();
        if let Some(version) = &self.package_version {
            label.push_str(" · ");
            label.push_str(version);
        }
        if let Some(node) = &self.technology_node {
            label.push_str(" · ");
            label.push_str(node);
        }
        if let Some(pin) = &self.signed_package {
            label.push_str(" · signed ");
            label.push_str(&pin.display_label());
        }
        label
    }

    #[must_use]
    pub fn package_name(&self) -> &str {
        &self.package_name
    }

    #[must_use]
    pub fn package_version(&self) -> Option<&str> {
        self.package_version.as_deref()
    }

    #[must_use]
    pub fn technology_node(&self) -> Option<&str> {
        self.technology_node.as_deref()
    }

    #[must_use]
    pub fn model_library(&self) -> &str {
        &self.model_library
    }

    #[must_use]
    pub fn root_source(&self) -> &Path {
        &self.root_source
    }

    #[must_use]
    pub fn source_closure(&self) -> &[crate::ModelSourcePin] {
        &self.source_closure
    }

    #[must_use]
    pub fn source_edges(&self) -> &[crate::ModelSourceEdge] {
        &self.source_edges
    }

    /// Prove that the mutable execution catalog still contains exactly the
    /// technology contract accepted by the project. Re-parsing, refreshing,
    /// or replacing a library must therefore invalidate the attachment until
    /// the user explicitly accepts the new contract.
    pub fn validate_model_library(
        &self,
        library: &crate::ModelLibrary,
    ) -> Result<(), TechnologyBindingError> {
        let mut observed = Self::from_model_library(library)?;
        // The live model catalog can prove only its own authenticated source
        // closure. The signed PDK pin is independently resolved against the
        // trusted technology registry, so carry the accepted pin across this
        // comparison instead of treating its deliberate absence from
        // `from_model_library` as catalog drift.
        observed.signed_package = self.signed_package.clone();
        if &observed != self {
            return Err(TechnologyBindingError::CatalogDrift {
                library: self.model_library.clone(),
            });
        }
        Ok(())
    }

    #[must_use]
    pub const fn model_count(&self) -> usize {
        self.model_count
    }

    #[must_use]
    pub fn process_sections(&self) -> &[String] {
        &self.process_sections
    }

    #[must_use]
    pub fn signed_package(&self) -> Option<&ProjectSignedTechnologyPin> {
        self.signed_package.as_ref()
    }

    pub fn with_signed_package_metadata(
        mut self,
        package: &crate::pdk::package::PdkTechnologyPackageMetadata,
    ) -> Result<Self, TechnologyBindingError> {
        self.signed_package = Some(ProjectSignedTechnologyPin::from_package_metadata(package)?);
        self.validate()?;
        Ok(self)
    }
}

fn validate_retained_model_sources(
    library: &crate::ModelLibrary,
) -> Result<(), TechnologyBindingError> {
    if library.source_contents.len() != library.source_closure.len() {
        return Err(TechnologyBindingError::MissingRetainedSourceBytes);
    }
    for (pin, content) in library.source_closure.iter().zip(&library.source_contents) {
        if pin.path != content.path {
            return Err(TechnologyBindingError::RetainedSourceIdentityMismatch);
        }
        let digest = rspice_app_types::product::ContentDigest::from_bytes(
            sha2::Sha256::digest(&content.bytes).into(),
        );
        if digest != pin.digest {
            return Err(TechnologyBindingError::RetainedSourceDigestMismatch(
                pin.path.clone(),
            ));
        }
    }
    Ok(())
}

fn nonempty_owned(value: &str) -> Option<String> {
    (!value.trim().is_empty()).then(|| value.to_owned())
}

fn validate_technology_text(
    field: &'static str,
    value: &str,
) -> Result<(), TechnologyBindingError> {
    if value.is_empty() {
        return Err(TechnologyBindingError::RequiredField(field));
    }
    if value != value.trim() {
        return Err(TechnologyBindingError::SurroundingWhitespace(field));
    }
    if value.chars().count() > 240 {
        return Err(TechnologyBindingError::FieldTooLong(field));
    }
    if value.chars().any(char::is_control) {
        return Err(TechnologyBindingError::ControlCharacter(field));
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TechnologyBindingError {
    #[error("technology binding schema {found} is unsupported; this build supports {supported}")]
    UnsupportedSchema { found: u16, supported: u16 },
    #[error("technology binding field {0} is required")]
    RequiredField(&'static str),
    #[error("technology binding field {0} must not begin or end with whitespace")]
    SurroundingWhitespace(&'static str),
    #[error("technology binding field {0} exceeds 240 Unicode scalar values")]
    FieldTooLong(&'static str),
    #[error("technology binding field {0} contains a control character")]
    ControlCharacter(&'static str),
    #[error("technology binding has no canonical root model source")]
    MissingRootSource,
    #[error("technology binding has no pinned model-source closure")]
    EmptySourceClosure,
    #[error("technology binding repeats pinned source {0}")]
    DuplicateSource(PathBuf),
    #[error("technology binding root {0} is absent from its pinned source closure")]
    RootAbsentFromClosure(PathBuf),
    #[error("technology binding source {0} is not an absolute portable path identity")]
    NonAbsoluteSource(PathBuf),
    #[error("technology binding source closure must be strictly sorted by canonical path")]
    UnsortedSourceClosure,
    #[error("multi-file technology binding has no authenticated dependency-resolution edges")]
    MissingSourceEdges,
    #[error("technology binding source edges must be strictly sorted and unique")]
    UnsortedSourceEdges,
    #[error("technology binding source edge references a source outside its pinned closure")]
    SourceEdgeOutsideClosure,
    #[error("technology binding contains an invalid source include path")]
    InvalidSourceEdge,
    #[error("technology binding source {0} is unreachable from its root")]
    UnreachableSource(PathBuf),
    #[error("technology binding process sections must be strictly sorted and unique")]
    UnsortedProcessSections,
    #[error("technology binding contains no parsed device models")]
    NoModels,
    #[error("technology binding does not retain exact bytes for every pinned model source")]
    MissingRetainedSourceBytes,
    #[error("technology binding retained source identities do not match their pinned closure")]
    RetainedSourceIdentityMismatch,
    #[error("technology binding retained bytes for {0} do not match their accepted digest")]
    RetainedSourceDigestMismatch(PathBuf),
    #[error(
        "attached model library '{library}' no longer matches the accepted technology contract"
    )]
    CatalogDrift { library: String },
    #[error("signed technology pin schema {found} is unsupported; this build supports {supported}")]
    UnsupportedSignedPackageSchema { found: u16, supported: u16 },
    #[error("signed technology package pin is invalid: {0}")]
    InvalidSignedPackage(String),
    #[error("project technology binding has no signed PDK package pin")]
    MissingSignedPackage,
    #[error("signed PDK package '{package_id}' revision '{revision}' is not currently trusted")]
    SignedPackageUnavailable {
        package_id: String,
        revision: String,
    },
    #[error("signed PDK package '{package_id}' revision '{revision}' metadata has drifted")]
    SignedPackageMetadataDrift {
        package_id: String,
        revision: String,
    },
    #[error("signed PDK package is incompatible with this runtime: {0}")]
    SignedPackageRuntime(String),
}
