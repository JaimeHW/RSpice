//! Executable revision boundary for one project-owned model definition.
//!
//! [`ModelDefinitionMetadata`] is deliberately capable of describing model
//! sets assembled from several source files. A project-authored `.model`
//! revision is narrower: one retained source owns the base card and every
//! named section. This module joins those two representations only after the
//! complete source can be rendered deterministically and reparsed without
//! losing a parameter, section, or type distinction.

use std::collections::{BTreeMap, BTreeSet};

use sha2::{Digest as _, Sha256};
use thiserror::Error;

use super::{
    DefinitionMetadataError, ModelDefinitionMetadata, ModelFileIdentity, ParameterDataType,
    ParameterValue, ProjectModelDefinition,
};
use crate::product::{ContentDigest, ModelSourceId, ObjectRevision};

/// A typed projection of the single retained source allowed to own a
/// project-authored model revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectModelSourceIdentity {
    pub source_id: ModelSourceId,
    pub revision: ObjectRevision,
    pub content_digest: ContentDigest,
    pub display_name: String,
}

/// One base `.model` card plus the metadata that defines its executable named
/// sections. Neither half is publishable through this type unless the other
/// half agrees exactly.
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectModelRevisionDefinition {
    pub base: ProjectModelDefinition,
    pub metadata: ModelDefinitionMetadata,
    source_identity: Option<ModelFileIdentity>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ProjectModelRevisionError {
    #[error(transparent)]
    Metadata(#[from] DefinitionMetadataError),
    #[error("base model definition is invalid: {0}")]
    BaseDefinition(String),
    #[error("{path}: {message}")]
    Invalid { path: String, message: String },
    #[error("canonical model source did not parse: {0}")]
    Parse(String),
    #[error("canonical model source round-trip failed for {scope}: {message}")]
    RoundTrip { scope: String, message: String },
}

#[derive(Debug, Clone, PartialEq)]
struct EffectiveParameters {
    numeric: BTreeMap<String, f64>,
    strings: BTreeMap<String, String>,
}

impl ProjectModelRevisionDefinition {
    #[must_use]
    pub fn new(base: ProjectModelDefinition, metadata: ModelDefinitionMetadata) -> Self {
        let source_identity = metadata.source_identity.clone();
        Self {
            base,
            metadata,
            source_identity,
        }
    }

    /// Bind manager-owned identity to this candidate and derive the exact
    /// digest from its canonical source. Callers provide identity and revision;
    /// users never type or copy UUIDs and digests into section rows.
    pub fn bind_project_source_identity(
        mut self,
        source_id: ModelSourceId,
        revision: ObjectRevision,
        display_name: impl Into<String>,
    ) -> Result<Self, ProjectModelRevisionError> {
        let identity = ModelFileIdentity {
            source_id: source_id.to_string(),
            revision: revision.get(),
            // A syntactically valid placeholder permits metadata and parser
            // validation before the content-derived digest is available.
            content_digest: "0".repeat(64),
            display_name: display_name.into(),
        };
        validate_source_display_name(&identity.display_name)?;
        self.source_identity = Some(identity.clone());
        self.metadata.source_identity = Some(identity.clone());
        for section in &mut self.metadata.sections {
            section.model_files = vec![identity.clone()];
        }

        let digest = self.expected_source_digest()?.to_string();
        let Some(bound_identity) = self.source_identity.as_mut() else {
            return invalid(
                "source_identity",
                "project source identity was not retained during binding",
            );
        };
        bound_identity.content_digest.clone_from(&digest);
        let Some(metadata_identity) = self.metadata.source_identity.as_mut() else {
            return invalid(
                "metadata.source_identity",
                "project source identity was not retained in model metadata",
            );
        };
        metadata_identity.content_digest.clone_from(&digest);
        for section in &mut self.metadata.sections {
            section.model_files[0].content_digest.clone_from(&digest);
        }
        self.validate()?;
        Ok(self)
    }

    /// Validate the complete revision, including the declared source digest
    /// and a parser-backed proof of the exact emitted source.
    pub fn validate(&self) -> Result<(), ProjectModelRevisionError> {
        self.canonical_source().map(|_| ())
    }

    /// Render the only source bytes that may be committed for this revision.
    /// Sections are ordered case-insensitively by name and contain complete
    /// effective cards, so their behavior is independent of declaration order.
    pub fn canonical_source(&self) -> Result<String, ProjectModelRevisionError> {
        let prepared = self.prepare()?;
        let source = self.render_prepared(&prepared)?;
        let digest = source_digest(source.as_bytes());
        if let Some(identity) = prepared.source_identity.as_ref()
            && identity.content_digest != digest
        {
            return invalid(
                "metadata.sections[*].model_files[0].content_digest",
                format!(
                    "declared digest {} does not match canonical source digest {digest}",
                    identity.content_digest
                ),
            );
        }
        self.verify_parsed_source(&source, &prepared)?;
        Ok(source)
    }

    /// Calculate the digest a caller must assign to every section source
    /// identity after editing the base card or any section override. Existing
    /// identity syntax and cross-section coherence are still validated, but a
    /// stale declared digest is intentionally ignored by this calculation.
    pub fn expected_source_digest(&self) -> Result<ContentDigest, ProjectModelRevisionError> {
        let prepared = self.prepare()?;
        let source = self.render_prepared(&prepared)?;
        self.verify_parsed_source(&source, &prepared)?;
        Ok(source_digest(source.as_bytes()))
    }

    /// Re-run the full canonical and parser contract against supplied bytes.
    /// This is intended for manager publication paths that already own a byte
    /// buffer and must prove it is exactly the source authorized by the draft.
    pub fn verify_source_round_trip(&self, source: &str) -> Result<(), ProjectModelRevisionError> {
        let prepared = self.prepare()?;
        let canonical = self.render_prepared(&prepared)?;
        if source != canonical {
            return invalid(
                "source",
                "supplied source is not the deterministic canonical rendering",
            );
        }
        let digest = source_digest(source.as_bytes());
        if let Some(identity) = prepared.source_identity.as_ref()
            && identity.content_digest != digest
        {
            return invalid(
                "metadata.sections[*].model_files[0].content_digest",
                format!(
                    "declared digest {} does not match supplied source digest {digest}",
                    identity.content_digest
                ),
            );
        }
        self.verify_parsed_source(source, &prepared)
    }

    /// Render the exact standalone model card executed by a qualification
    /// vector. `None` selects the top-level base card; a named section selects
    /// that section's fully resolved effective card. The returned bytes are a
    /// literal contiguous slice of [`Self::canonical_source`], allowing the
    /// qualification domain to prove section selection without consulting
    /// mutable manager state or reconstructing an override graph.
    pub fn qualification_model_source(
        &self,
        section: Option<&str>,
    ) -> Result<String, ProjectModelRevisionError> {
        let prepared = self.prepare()?;
        let source = match section {
            None => self
                .base
                .canonical_source()
                .map_err(ProjectModelRevisionError::BaseDefinition)?,
            Some(section_name) => {
                let section = prepared
                    .sections
                    .iter()
                    .find(|candidate| candidate.name.eq_ignore_ascii_case(section_name))
                    .ok_or_else(|| ProjectModelRevisionError::Invalid {
                        path: "qualification.section".to_owned(),
                        message: format!("model section {section_name:?} does not exist"),
                    })?;
                ProjectModelDefinition {
                    name: self.base.name.clone(),
                    spice_type: self.base.spice_type.clone(),
                    description: String::new(),
                    numeric_parameters: section.parameters.numeric.clone(),
                    string_parameters: section.parameters.strings.clone(),
                }
                .canonical_source()
                .map_err(ProjectModelRevisionError::BaseDefinition)?
            }
        };
        let canonical = self.render_prepared(&prepared)?;
        if !canonical
            .as_bytes()
            .windows(source.len())
            .any(|bytes| bytes == source.as_bytes())
        {
            return invalid(
                "qualification.model_source",
                "qualification model card is not a literal member of the canonical source",
            );
        }
        Ok(source)
    }

    /// Return the one typed source identity shared by every section. A
    /// base-only definition has no section source projection and returns
    /// `None`; its manager-owned source authority remains authoritative.
    pub fn project_source_identity(
        &self,
    ) -> Result<Option<ProjectModelSourceIdentity>, ProjectModelRevisionError> {
        self.canonical_source()?;
        self.prepare().map(|prepared| prepared.source_identity)
    }

    fn prepare(&self) -> Result<PreparedRevision, ProjectModelRevisionError> {
        // Metadata is deliberately the first validation boundary. Callers can
        // never use a valid base card to bypass a malformed metadata graph.
        self.metadata.validate()?;
        self.base
            .validate()
            .map_err(ProjectModelRevisionError::BaseDefinition)?;
        self.validate_base_parameter_contract()?;
        let source_identity = self.validate_source_identity_contract()?;
        let sections = self.resolve_sections()?;
        Ok(PreparedRevision {
            source_identity,
            sections,
        })
    }

    fn validate_base_parameter_contract(&self) -> Result<(), ProjectModelRevisionError> {
        let mut base_types = BTreeMap::<String, ParameterDataType>::new();
        for (name, value) in &self.base.numeric_parameters {
            let canonical = canonical_identifier(name);
            base_types.insert(canonical.clone(), ParameterDataType::Numeric);
            let Some(parameter) = self.metadata.parameter(name) else {
                return invalid(
                    "metadata.parameters",
                    format!("base numeric parameter '{name}' is not declared in metadata"),
                );
            };
            if parameter.data_type != ParameterDataType::Numeric {
                return invalid(
                    format!("metadata.parameters[{name:?}].data_type"),
                    "type does not match the base numeric parameter",
                );
            }
            let ParameterValue::Numeric(metadata_value) = parameter.value else {
                return invalid(
                    format!("metadata.parameters[{name:?}].value"),
                    "value does not match the base numeric parameter type",
                );
            };
            if metadata_value.get().to_bits() != value.to_bits() {
                return invalid(
                    format!("metadata.parameters[{name:?}].value"),
                    format!(
                        "value {} does not exactly match base value {value}",
                        metadata_value.get()
                    ),
                );
            }
        }
        for (name, value) in &self.base.string_parameters {
            let canonical = canonical_identifier(name);
            base_types.insert(canonical.clone(), ParameterDataType::String);
            let Some(parameter) = self.metadata.parameter(name) else {
                return invalid(
                    "metadata.parameters",
                    format!("base string parameter '{name}' is not declared in metadata"),
                );
            };
            if parameter.data_type != ParameterDataType::String {
                return invalid(
                    format!("metadata.parameters[{name:?}].data_type"),
                    "type does not match the base string parameter",
                );
            }
            let ParameterValue::String(metadata_value) = &parameter.value else {
                return invalid(
                    format!("metadata.parameters[{name:?}].value"),
                    "value does not match the base string parameter type",
                );
            };
            if metadata_value != value {
                return invalid(
                    format!("metadata.parameters[{name:?}].value"),
                    format!("value {metadata_value:?} does not exactly match base value {value:?}"),
                );
            }
        }
        if self.metadata.parameters.len() != base_types.len() {
            let extras = self
                .metadata
                .parameters
                .iter()
                .filter(|parameter| {
                    !base_types.contains_key(&canonical_identifier(&parameter.name))
                })
                .map(|parameter| parameter.name.as_str())
                .collect::<Vec<_>>();
            return invalid(
                "metadata.parameters",
                if extras.is_empty() {
                    "metadata parameter count does not match the base definition".to_owned()
                } else {
                    format!(
                        "metadata declares parameters absent from the base definition: {}",
                        extras.join(", ")
                    )
                },
            );
        }
        Ok(())
    }

    fn validate_source_identity_contract(
        &self,
    ) -> Result<Option<ProjectModelSourceIdentity>, ProjectModelRevisionError> {
        let mut shared = self
            .source_identity
            .as_ref()
            .or(self.metadata.source_identity.as_ref());
        if let (Some(internal), Some(metadata)) = (
            self.source_identity.as_ref(),
            self.metadata.source_identity.as_ref(),
        ) && internal != metadata
        {
            return invalid(
                "metadata.source_identity",
                "metadata identity does not match the revision source identity",
            );
        }
        for (index, section) in self.metadata.sections.iter().enumerate() {
            if section.model_files.len() != 1 {
                return invalid(
                    format!("metadata.sections[{index}].model_files"),
                    format!(
                        "project-authored section '{}' must reference exactly one project-owned source, found {}",
                        section.name,
                        section.model_files.len()
                    ),
                );
            }
            let identity = &section.model_files[0];
            if let Some(expected) = shared {
                if identity != expected {
                    return invalid(
                        format!("metadata.sections[{index}].model_files[0]"),
                        format!(
                            "section '{}' does not reference the same project-owned source identity as every other section",
                            section.name
                        ),
                    );
                }
            } else {
                shared = Some(identity);
            }
        }

        shared.map(parse_source_identity).transpose()
    }

    fn resolve_sections(&self) -> Result<Vec<ResolvedSection>, ProjectModelRevisionError> {
        let indices = self
            .metadata
            .sections
            .iter()
            .enumerate()
            .map(|(index, section)| (canonical_identifier(&section.name), index))
            .collect::<BTreeMap<_, _>>();
        let mut memo = BTreeMap::<String, EffectiveParameters>::new();
        let mut visiting = BTreeSet::<String>::new();
        for section in &self.metadata.sections {
            let name = canonical_identifier(&section.name);
            self.resolve_section(&name, &indices, &mut memo, &mut visiting)?;
        }

        let mut sections = self
            .metadata
            .sections
            .iter()
            .map(|section| ResolvedSection {
                name: section.name.clone(),
                parameters: memo[&canonical_identifier(&section.name)].clone(),
            })
            .collect::<Vec<_>>();
        sections.sort_by(|left, right| {
            canonical_identifier(&left.name)
                .cmp(&canonical_identifier(&right.name))
                .then_with(|| left.name.cmp(&right.name))
        });
        Ok(sections)
    }

    fn resolve_section(
        &self,
        name: &str,
        indices: &BTreeMap<String, usize>,
        memo: &mut BTreeMap<String, EffectiveParameters>,
        visiting: &mut BTreeSet<String>,
    ) -> Result<EffectiveParameters, ProjectModelRevisionError> {
        if let Some(parameters) = memo.get(name) {
            return Ok(parameters.clone());
        }
        if !visiting.insert(name.to_owned()) {
            return invalid(
                "metadata.sections[*].parent",
                format!("section inheritance cycle reached '{name}' during source rendering"),
            );
        }
        let index =
            indices
                .get(name)
                .copied()
                .ok_or_else(|| ProjectModelRevisionError::Invalid {
                    path: "metadata.sections[*].parent".to_owned(),
                    message: format!("unknown parent section '{name}'"),
                })?;
        let section = &self.metadata.sections[index];
        let mut effective = if let Some(parent) = &section.parent {
            self.resolve_section(&canonical_identifier(parent), indices, memo, visiting)?
        } else {
            EffectiveParameters {
                numeric: self.base.numeric_parameters.clone(),
                strings: self.base.string_parameters.clone(),
            }
        };
        for (parameter, value) in &section.overrides {
            let canonical = canonical_identifier(parameter);
            match value {
                ParameterValue::Numeric(value) => {
                    let Some(existing) = find_key(&effective.numeric, &canonical) else {
                        return invalid(
                            format!("metadata.sections[{index}].overrides[{parameter:?}]"),
                            "numeric override does not resolve to a base parameter",
                        );
                    };
                    effective.numeric.insert(existing, value.get());
                }
                ParameterValue::String(value) => {
                    let Some(existing) = find_key(&effective.strings, &canonical) else {
                        return invalid(
                            format!("metadata.sections[{index}].overrides[{parameter:?}]"),
                            "string override does not resolve to a base parameter",
                        );
                    };
                    effective.strings.insert(existing, value.clone());
                }
            }
        }
        visiting.remove(name);
        memo.insert(name.to_owned(), effective.clone());
        Ok(effective)
    }

    fn render_prepared(
        &self,
        prepared: &PreparedRevision,
    ) -> Result<String, ProjectModelRevisionError> {
        let mut source = self
            .base
            .canonical_source()
            .map_err(ProjectModelRevisionError::BaseDefinition)?;
        for section in &prepared.sections {
            source.push_str(".lib ");
            source.push_str(&section.name);
            source.push('\n');
            let definition = ProjectModelDefinition {
                name: self.base.name.clone(),
                spice_type: self.base.spice_type.clone(),
                description: String::new(),
                numeric_parameters: section.parameters.numeric.clone(),
                string_parameters: section.parameters.strings.clone(),
            };
            source.push_str(
                &definition
                    .canonical_source()
                    .map_err(ProjectModelRevisionError::BaseDefinition)?,
            );
            source.push_str(".endl ");
            source.push_str(&section.name);
            source.push('\n');
        }
        Ok(source)
    }

    fn verify_parsed_source(
        &self,
        source: &str,
        prepared: &PreparedRevision,
    ) -> Result<(), ProjectModelRevisionError> {
        let mut parser = rspice_core::library::LibParser::new("/");
        let parsed = parser.parse_string(source);
        if !parsed.is_ok() {
            return Err(ProjectModelRevisionError::Parse(
                parsed
                    .errors
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("; "),
            ));
        }
        if parsed.top_level_models.len() != 1 {
            return round_trip(
                "top-level model",
                format!(
                    "expected exactly one model, parsed {}",
                    parsed.top_level_models.len()
                ),
            );
        }
        if parsed.subcircuit_count() != 0 {
            return round_trip(
                "source",
                format!(
                    "expected no subcircuits, parsed {}",
                    parsed.subcircuit_count()
                ),
            );
        }
        if parsed.model_count() != prepared.sections.len() + 1 {
            return round_trip(
                "source",
                format!(
                    "expected {} total models, parsed {}",
                    prepared.sections.len() + 1,
                    parsed.model_count()
                ),
            );
        }

        let expected_section_names = prepared
            .sections
            .iter()
            .map(|section| section.name.as_str())
            .collect::<Vec<_>>();
        let parsed_section_names = parsed.section_names();
        if parsed_section_names != expected_section_names {
            return round_trip(
                "sections",
                format!(
                    "expected exact names {expected_section_names:?}, parsed {parsed_section_names:?}"
                ),
            );
        }

        let base_parameters = EffectiveParameters {
            numeric: self.base.numeric_parameters.clone(),
            strings: self.base.string_parameters.clone(),
        };
        self.verify_parsed_model(
            "top-level model",
            &parsed.top_level_models[0],
            &base_parameters,
        )?;
        for (parsed_section, expected_section) in parsed.sections.iter().zip(&prepared.sections) {
            if !parsed_section.subcircuits.is_empty() {
                return round_trip(
                    format!("section '{}'", expected_section.name),
                    format!(
                        "expected no subcircuits, parsed {}",
                        parsed_section.subcircuits.len()
                    ),
                );
            }
            if parsed_section.models.len() != 1 {
                return round_trip(
                    format!("section '{}'", expected_section.name),
                    format!(
                        "expected exactly one model, parsed {}",
                        parsed_section.models.len()
                    ),
                );
            }
            self.verify_parsed_model(
                &format!("section '{}'", expected_section.name),
                &parsed_section.models[0],
                &expected_section.parameters,
            )?;
        }
        Ok(())
    }

    fn verify_parsed_model(
        &self,
        scope: &str,
        parsed: &rspice_core::library::ParsedModel,
        expected: &EffectiveParameters,
    ) -> Result<(), ProjectModelRevisionError> {
        if parsed.name != self.base.name {
            return round_trip(
                scope,
                format!(
                    "model name '{}' does not match '{}'",
                    parsed.name, self.base.name
                ),
            );
        }
        let expected_type = rspice_core::library::ModelType::from_spice_type(&self.base.spice_type);
        if parsed.model_type != expected_type {
            return round_trip(
                scope,
                format!(
                    "parsed model type {:?} does not match expected {:?}",
                    parsed.model_type, expected_type
                ),
            );
        }
        if parsed.parameters.len() != expected.numeric.len()
            || expected.numeric.iter().any(|(name, value)| {
                parsed
                    .parameters
                    .get(&canonical_identifier(name))
                    .is_none_or(|parsed| parsed.to_bits() != value.to_bits())
            })
        {
            return round_trip(scope, "numeric parameters did not round-trip exactly");
        }
        if parsed.string_params.len() != expected.strings.len()
            || expected.strings.iter().any(|(name, value)| {
                parsed
                    .string_params
                    .get(&canonical_identifier(name))
                    .is_none_or(|parsed| parsed != value)
            })
        {
            return round_trip(scope, "string parameters did not round-trip exactly");
        }
        Ok(())
    }
}

#[derive(Debug)]
struct PreparedRevision {
    source_identity: Option<ProjectModelSourceIdentity>,
    sections: Vec<ResolvedSection>,
}

#[derive(Debug)]
struct ResolvedSection {
    name: String,
    parameters: EffectiveParameters,
}

fn parse_source_identity(
    identity: &ModelFileIdentity,
) -> Result<ProjectModelSourceIdentity, ProjectModelRevisionError> {
    validate_source_display_name(&identity.display_name)?;
    let source_id = identity
        .source_id
        .parse::<ModelSourceId>()
        .map_err(|error| ProjectModelRevisionError::Invalid {
            path: "metadata.sections[*].model_files[0].source_id".to_owned(),
            message: format!("source is not a project-owned model-source UUID: {error}"),
        })?;
    let revision = ObjectRevision::new(identity.revision).map_err(|error| {
        ProjectModelRevisionError::Invalid {
            path: "metadata.sections[*].model_files[0].revision".to_owned(),
            message: error.to_string(),
        }
    })?;
    let content_digest = identity
        .content_digest
        .parse::<ContentDigest>()
        .map_err(|error| ProjectModelRevisionError::Invalid {
            path: "metadata.sections[*].model_files[0].content_digest".to_owned(),
            message: error.to_string(),
        })?;
    Ok(ProjectModelSourceIdentity {
        source_id,
        revision,
        content_digest,
        display_name: identity.display_name.clone(),
    })
}

fn validate_source_display_name(value: &str) -> Result<(), ProjectModelRevisionError> {
    if value.trim().is_empty() || value.trim() != value {
        return invalid(
            "source_identity.display_name",
            "display name must not be empty or contain outer whitespace",
        );
    }
    if value
        .chars()
        .any(|character| character == '\0' || (character.is_control() && character != '\n'))
    {
        return invalid(
            "source_identity.display_name",
            "display name contains a forbidden control character",
        );
    }
    Ok(())
}

fn find_key<T>(values: &BTreeMap<String, T>, canonical: &str) -> Option<String> {
    values
        .keys()
        .find(|name| canonical_identifier(name) == canonical)
        .cloned()
}

fn canonical_identifier(value: &str) -> String {
    value.to_ascii_lowercase()
}

fn source_digest(bytes: &[u8]) -> ContentDigest {
    ContentDigest::from_bytes(Sha256::digest(bytes).into())
}

fn invalid<T>(
    path: impl Into<String>,
    message: impl Into<String>,
) -> Result<T, ProjectModelRevisionError> {
    Err(ProjectModelRevisionError::Invalid {
        path: path.into(),
        message: message.into(),
    })
}

fn round_trip<T>(
    scope: impl Into<String>,
    message: impl Into<String>,
) -> Result<T, ProjectModelRevisionError> {
    Err(ProjectModelRevisionError::RoundTrip {
        scope: scope.into(),
        message: message.into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::model_library::{
        FiniteF64, MODEL_DEFINITION_METADATA_SCHEMA_VERSION, ModelSectionDefinition,
        ModelSectionQualification, ParameterDefinition, ParameterSource, StatisticalDefinition,
    };

    fn finite(value: f64) -> FiniteF64 {
        FiniteF64::new(value).expect("finite fixture value")
    }

    fn parameter(name: &str, value: ParameterValue) -> ParameterDefinition {
        ParameterDefinition {
            name: name.to_owned(),
            data_type: value.data_type(),
            value,
            unit: None,
            bounds: None,
            source: ParameterSource::Declared {
                source: "project-owned base model".to_owned(),
            },
            description: format!("Project-owned {name} model parameter."),
        }
    }

    fn source_identity() -> ModelFileIdentity {
        ModelFileIdentity {
            source_id: ModelSourceId::new().to_string(),
            revision: 3,
            content_digest: "0".repeat(64),
            display_name: "definition.model".to_owned(),
        }
    }

    fn section(
        name: &str,
        parent: Option<&str>,
        overrides: impl IntoIterator<Item = (&'static str, ParameterValue)>,
        source: &ModelFileIdentity,
    ) -> ModelSectionDefinition {
        ModelSectionDefinition {
            name: name.to_owned(),
            parent: parent.map(str::to_owned),
            overrides: overrides
                .into_iter()
                .map(|(name, value)| (name.to_owned(), value))
                .collect(),
            model_files: vec![source.clone()],
            qualification: ModelSectionQualification::Unqualified,
        }
    }

    fn revision_fixture() -> ProjectModelRevisionDefinition {
        let base = ProjectModelDefinition {
            name: "owned_nch".to_owned(),
            spice_type: "nmos".to_owned(),
            description: "Project-owned model".to_owned(),
            numeric_parameters: BTreeMap::from([
                ("level".to_owned(), 1.0),
                ("vth0".to_owned(), 0.45),
            ]),
            string_parameters: BTreeMap::from([("mode".to_owned(), "physical".to_owned())]),
        };
        let source = source_identity();
        let metadata = ModelDefinitionMetadata {
            schema_version: MODEL_DEFINITION_METADATA_SCHEMA_VERSION,
            source_identity: None,
            parameters: vec![
                parameter("vth0", ParameterValue::Numeric(finite(0.45))),
                parameter("mode", ParameterValue::String("physical".to_owned())),
                parameter("level", ParameterValue::Numeric(finite(1.0))),
            ],
            // Intentionally reverse lexical order. Canonical source sorting is
            // part of the executable digest contract.
            sections: vec![
                section(
                    "SS",
                    Some("TT"),
                    [("mode", ParameterValue::String("empirical".to_owned()))],
                    &source,
                ),
                section(
                    "TT",
                    None,
                    [("vth0", ParameterValue::Numeric(finite(0.5)))],
                    &source,
                ),
            ],
            statistics: StatisticalDefinition::default(),
            temperature_laws: Vec::new(),
        };
        let mut revision = ProjectModelRevisionDefinition::new(base, metadata);
        let digest = revision
            .expected_source_digest()
            .expect("fixture source round-trips")
            .to_string();
        for section in &mut revision.metadata.sections {
            section.model_files[0].content_digest.clone_from(&digest);
        }
        revision
    }

    #[test]
    fn canonical_rendering_is_deterministic_and_resolves_inheritance() {
        let revision = revision_fixture();
        let source = revision.canonical_source().expect("valid revision source");
        assert_eq!(
            source,
            "* Project-owned model\n\
.model owned_nch NMOS ( LEVEL=1 VTH0=0.45 MODE=\"physical\" )\n\
.lib SS\n\
.model owned_nch NMOS ( LEVEL=1 VTH0=0.5 MODE=\"empirical\" )\n\
.endl SS\n\
.lib TT\n\
.model owned_nch NMOS ( LEVEL=1 VTH0=0.5 MODE=\"physical\" )\n\
.endl TT\n"
        );
        assert_eq!(source, revision.canonical_source().expect("repeat render"));
    }

    #[test]
    fn parser_round_trip_proves_base_and_every_exact_section() {
        let revision = revision_fixture();
        let source = revision.canonical_source().expect("valid revision source");
        revision
            .verify_source_round_trip(&source)
            .expect("canonical source round-trips");

        let mut parser = rspice_core::library::LibParser::new("/");
        let parsed = parser.parse_string(&source);
        assert!(parsed.is_ok());
        assert_eq!(parsed.top_level_models.len(), 1);
        assert_eq!(parsed.section_names(), ["SS", "TT"]);
        assert_eq!(parsed.model_count(), 3);
        assert_eq!(parsed.subcircuit_count(), 0);
        assert_eq!(parsed.sections[0].models[0].parameters["vth0"], 0.5);
        assert_eq!(
            parsed.sections[0].models[0].string_params["mode"],
            "empirical"
        );
    }

    #[test]
    fn metadata_value_or_type_mismatch_fails_closed() {
        let mut revision = revision_fixture();
        revision
            .metadata
            .parameters
            .iter_mut()
            .find(|parameter| parameter.name == "vth0")
            .expect("vth0 metadata")
            .value = ParameterValue::Numeric(finite(0.46));
        let error = revision.validate().expect_err("value mismatch");
        assert!(
            error
                .to_string()
                .contains("does not exactly match base value")
        );

        let mut revision = revision_fixture();
        let parameter = revision
            .metadata
            .parameters
            .iter_mut()
            .find(|parameter| parameter.name == "mode")
            .expect("mode metadata");
        parameter.data_type = ParameterDataType::Numeric;
        let error = revision.validate().expect_err("type mismatch");
        assert!(matches!(error, ProjectModelRevisionError::Metadata(_)));
    }

    #[test]
    fn source_identity_and_digest_mismatches_fail_closed() {
        let mut revision = revision_fixture();
        revision.metadata.sections[1].model_files[0].source_id = ModelSourceId::new().to_string();
        let error = revision.validate().expect_err("source identity mismatch");
        assert!(
            error
                .to_string()
                .contains("same project-owned source identity")
        );

        let mut revision = revision_fixture();
        let stale = "f".repeat(64);
        for section in &mut revision.metadata.sections {
            section.model_files[0].content_digest.clone_from(&stale);
        }
        let error = revision.validate().expect_err("digest mismatch");
        assert!(
            error
                .to_string()
                .contains("does not match canonical source digest")
        );
    }

    #[test]
    fn inherited_cycle_is_rejected_before_rendering() {
        let mut revision = revision_fixture();
        revision.metadata.sections[1].parent = Some("SS".to_owned());
        let error = revision.validate().expect_err("cycle");
        assert!(matches!(error, ProjectModelRevisionError::Metadata(_)));
        assert!(error.to_string().contains("cycle"));
    }

    #[test]
    fn quoted_numeric_looking_string_values_preserve_their_type() {
        let mut revision = revision_fixture();
        let source_identity = revision
            .project_source_identity()
            .expect("fixture identity validates")
            .expect("fixture is project-bound");
        revision
            .base
            .string_parameters
            .insert("mode".to_owned(), "1".to_owned());
        revision
            .metadata
            .parameters
            .iter_mut()
            .find(|parameter| parameter.name == "mode")
            .expect("mode metadata")
            .value = ParameterValue::String("1".to_owned());
        revision = revision
            .bind_project_source_identity(
                source_identity.source_id,
                source_identity.revision,
                source_identity.display_name,
            )
            .expect("changed source is rebound to its content-derived identity");
        revision
            .expected_source_digest()
            .expect("quoted numeric-looking string round-trips as a string");
        let source = revision
            .canonical_source()
            .expect("numeric-looking string renders canonically");
        assert!(source.contains("MODE=\"1\""));
        revision
            .verify_source_round_trip(&source)
            .expect("quoted numeric-looking string preserves its type");
    }

    #[test]
    fn supplied_noncanonical_source_is_rejected_even_if_it_parses() {
        let revision = revision_fixture();
        let mut source = revision.canonical_source().expect("canonical source");
        source.push('\n');
        let error = revision
            .verify_source_round_trip(&source)
            .expect_err("extra byte is not canonical");
        assert!(
            error
                .to_string()
                .contains("not the deterministic canonical")
        );
    }

    #[test]
    fn manager_binding_retains_identity_for_a_base_only_definition() {
        let base = ProjectModelDefinition {
            name: "owned_diode".to_owned(),
            spice_type: "d".to_owned(),
            description: "Project-owned diode".to_owned(),
            numeric_parameters: BTreeMap::from([("is".to_owned(), 1.0e-14)]),
            string_parameters: BTreeMap::new(),
        };
        let metadata = ModelDefinitionMetadata {
            schema_version: MODEL_DEFINITION_METADATA_SCHEMA_VERSION,
            source_identity: None,
            parameters: vec![parameter("is", ParameterValue::Numeric(finite(1.0e-14)))],
            sections: Vec::new(),
            statistics: StatisticalDefinition::default(),
            temperature_laws: Vec::new(),
        };
        let source_id = ModelSourceId::new();
        let revision = ProjectModelRevisionDefinition::new(base, metadata)
            .bind_project_source_identity(
                source_id,
                ObjectRevision::new(9).expect("valid revision"),
                "owned-diode.model",
            )
            .expect("manager binding succeeds");
        let identity = revision
            .project_source_identity()
            .expect("valid identity")
            .expect("base-only identity retained");
        assert_eq!(identity.source_id, source_id);
        assert_eq!(identity.revision.get(), 9);
        assert_eq!(identity.display_name, "owned-diode.model");
        assert_eq!(
            identity.content_digest,
            revision.expected_source_digest().expect("source digest")
        );
        revision.validate().expect("bound base-only revision");
    }

    #[test]
    fn manager_binding_replaces_every_section_identity_with_one_exact_source() {
        let source_id = ModelSourceId::new();
        let revision = revision_fixture()
            .bind_project_source_identity(
                source_id,
                ObjectRevision::new(12).expect("valid revision"),
                "owned-nch.model",
            )
            .expect("manager binding succeeds");
        let identity = revision
            .project_source_identity()
            .expect("valid identity")
            .expect("sectioned identity retained");
        assert_eq!(identity.source_id, source_id);
        assert_eq!(identity.revision.get(), 12);
        assert_eq!(identity.display_name, "owned-nch.model");
        assert_eq!(
            identity.content_digest,
            revision.expected_source_digest().expect("source digest")
        );
        for section in &revision.metadata.sections {
            assert_eq!(section.model_files.len(), 1);
            let file = &section.model_files[0];
            assert_eq!(file.source_id, source_id.to_string());
            assert_eq!(file.revision, 12);
            assert_eq!(file.display_name, "owned-nch.model");
            assert_eq!(file.content_digest, identity.content_digest.to_string());
        }
        revision.validate().expect("bound sectioned revision");
    }
}
