//! Versioned, project-owned metadata for one device-model definition.
//!
//! The model editor uses this domain to present typed parameters, section
//! inheritance, statistical ownership, and temperature behavior without
//! manufacturing facts from the GUI.  Values in this module are descriptive
//! model-definition data; executable SPICE source remains the simulation
//! authority and must be committed atomically with this metadata.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, de};
use thiserror::Error;

/// Current persisted schema for [`ModelDefinitionMetadata`].
pub const MODEL_DEFINITION_METADATA_SCHEMA_VERSION: u16 = 1;

const MATRIX_ELEMENT_TOLERANCE: f64 = 1.0e-12;
const MATRIX_PSD_TOLERANCE: f64 = 1.0e-10;
const PROBABILITY_SUM_TOLERANCE: f64 = 1.0e-10;

/// An IEEE-754 value whose construction and deserialization reject NaN and
/// infinities.
///
/// Keeping the invariant in the type makes persisted metadata equatable and
/// prevents a caller from bypassing validation by deserializing a non-finite
/// value through a binary format.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct FiniteF64(f64);

impl FiniteF64 {
    pub fn new(value: f64) -> Result<Self, NonFiniteNumber> {
        if value.is_finite() {
            Ok(Self(value))
        } else {
            Err(NonFiniteNumber)
        }
    }

    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl Eq for FiniteF64 {}

impl TryFrom<f64> for FiniteF64 {
    type Error = NonFiniteNumber;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<FiniteF64> for f64 {
    fn from(value: FiniteF64) -> Self {
        value.get()
    }
}

impl fmt::Display for FiniteF64 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl<'de> Deserialize<'de> for FiniteF64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = f64::deserialize(deserializer)?;
        Self::new(value).map_err(de::Error::custom)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("number must be finite")]
pub struct NonFiniteNumber;

/// Inclusive numeric bounds. At least one endpoint must be supplied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FiniteBounds {
    pub lower: Option<FiniteF64>,
    pub upper: Option<FiniteF64>,
}

impl FiniteBounds {
    #[must_use]
    pub fn contains(self, value: FiniteF64) -> bool {
        self.lower.is_none_or(|lower| value >= lower)
            && self.upper.is_none_or(|upper| value <= upper)
    }

    fn validate(self, path: &str) -> Result<(), DefinitionMetadataError> {
        if self.lower.is_none() && self.upper.is_none() {
            return invalid(path, "at least one finite endpoint is required");
        }
        if let (Some(lower), Some(upper)) = (self.lower, self.upper)
            && lower > upper
        {
            return invalid(path, "lower endpoint must not exceed upper endpoint");
        }
        Ok(())
    }
}

/// Closed temperature interval, expressed in degrees Celsius.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TemperatureRange {
    pub minimum_c: FiniteF64,
    pub maximum_c: FiniteF64,
}

impl TemperatureRange {
    #[must_use]
    pub fn contains(self, temperature_c: FiniteF64) -> bool {
        temperature_c >= self.minimum_c && temperature_c <= self.maximum_c
    }

    fn validate(self, path: &str) -> Result<(), DefinitionMetadataError> {
        if self.minimum_c >= self.maximum_c {
            return invalid(
                path,
                "minimum temperature must be less than maximum temperature",
            );
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterDataType {
    Numeric,
    String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ParameterValue {
    Numeric(FiniteF64),
    String(String),
}

impl ParameterValue {
    #[must_use]
    pub const fn data_type(&self) -> ParameterDataType {
        match self {
            Self::Numeric(_) => ParameterDataType::Numeric,
            Self::String(_) => ParameterDataType::String,
        }
    }
}

/// Provenance of the effective parameter value shown by the editor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ParameterSource {
    /// Declared directly by a base source or model card.
    Declared { source: String },
    /// Resolved without modification from the named ancestor section.
    Inherited { from_section: String },
    /// Replaced by an override in the named section.
    Overridden { section: String },
}

/// Typed schema and current effective value for one model parameter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParameterDefinition {
    pub name: String,
    pub data_type: ParameterDataType,
    pub value: ParameterValue,
    /// Display unit. `None` means dimensionless or not declared.
    pub unit: Option<String>,
    /// Inclusive numeric bounds. String parameters must not declare bounds.
    pub bounds: Option<FiniteBounds>,
    pub source: ParameterSource,
    pub description: String,
}

/// Stable identity of a model source contributing to a named section.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelFileIdentity {
    pub source_id: String,
    pub revision: u64,
    /// Lowercase hexadecimal SHA-256 digest of the exact retained bytes.
    pub content_digest: String,
    pub display_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum ModelSectionQualification {
    Unqualified,
    Pending,
    Qualified { evidence_digest: Option<String> },
    Failed { summary: String },
}

/// One process section and its delta from an optional parent section.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelSectionDefinition {
    pub name: String,
    pub parent: Option<String>,
    pub overrides: BTreeMap<String, ParameterValue>,
    pub model_files: Vec<ModelFileIdentity>,
    pub qualification: ModelSectionQualification,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatisticalHierarchyScope {
    Global,
    Wafer,
    Die,
    Instance,
    MatchedDeviceGroup,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiscreteOutcome {
    pub value: ParameterValue,
    /// Probability in the open interval `(0, 1]`.
    pub probability: FiniteF64,
}

/// A distribution centered on the target parameter's nominal value unless the
/// variant supplies explicit endpoints or outcomes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StatisticalDistribution {
    Normal { sigma: FiniteF64 },
    Lognormal { sigma_fraction: FiniteF64 },
    Uniform { lower: FiniteF64, upper: FiniteF64 },
    Discrete { outcomes: Vec<DiscreteOutcome> },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StatisticalVariableDefinition {
    pub name: String,
    /// Parameter whose value this variable perturbs or selects.
    pub parameter: String,
    pub distribution: StatisticalDistribution,
    /// Variables in the same named group use the corresponding correlation
    /// matrix. `None` declares an independent variable.
    pub correlation_group: Option<String>,
    pub hierarchy: StatisticalHierarchyScope,
    pub description: String,
}

/// Dense, ordered correlation matrix for one named variable group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorrelationMatrix {
    pub group: String,
    pub variables: Vec<String>,
    pub coefficients: Vec<Vec<FiniteF64>>,
}

impl CorrelationMatrix {
    #[must_use]
    pub fn coefficient(&self, row: usize, column: usize) -> Option<FiniteF64> {
        self.coefficients
            .get(row)
            .and_then(|values| values.get(column))
            .copied()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct StatisticalDefinition {
    pub variables: Vec<StatisticalVariableDefinition>,
    pub correlation_matrices: Vec<CorrelationMatrix>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LookupInterpolation {
    Linear,
    LogLinear,
    Step,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TemperaturePoint {
    pub temperature_c: FiniteF64,
    pub value: FiniteF64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TemperatureLawRepresentation {
    Equation {
        expression: String,
    },
    LookupTable {
        interpolation: LookupInterpolation,
        points: Vec<TemperaturePoint>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemperatureExtrapolationPolicy {
    BlockOutsideRange,
    Warn,
    Clamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TemperatureLawDefinition {
    pub quantity: String,
    /// Explicit parameter dependencies used by the equation or table.
    pub parameters: Vec<String>,
    pub representation: TemperatureLawRepresentation,
    pub reference_temperature_c: FiniteF64,
    pub valid_range: TemperatureRange,
    pub extrapolation: TemperatureExtrapolationPolicy,
    pub description: String,
}

/// One deterministic evaluation of a temperature law. The requested and
/// effective temperatures differ only when the declared clamp policy applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TemperatureLawEvaluation {
    pub requested_temperature_c: FiniteF64,
    pub effective_temperature_c: FiniteF64,
    pub value: FiniteF64,
    pub outside_declared_range: bool,
}

/// Complete metadata rendered by the Parameters, Sections, Statistics, and
/// Temperature pages of the model editor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelDefinitionMetadata {
    pub schema_version: u16,
    /// Stable identity of this model's canonical source fragment. The model
    /// revision advances only when this model changes; it is deliberately
    /// independent from the containing library closure revision.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_identity: Option<ModelFileIdentity>,
    pub parameters: Vec<ParameterDefinition>,
    pub sections: Vec<ModelSectionDefinition>,
    pub statistics: StatisticalDefinition,
    pub temperature_laws: Vec<TemperatureLawDefinition>,
}

impl Default for ModelDefinitionMetadata {
    fn default() -> Self {
        Self {
            schema_version: MODEL_DEFINITION_METADATA_SCHEMA_VERSION,
            source_identity: None,
            parameters: Vec::new(),
            sections: Vec::new(),
            statistics: StatisticalDefinition::default(),
            temperature_laws: Vec::new(),
        }
    }
}

impl ModelDefinitionMetadata {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn validate(&self) -> Result<(), DefinitionMetadataError> {
        if self.schema_version != MODEL_DEFINITION_METADATA_SCHEMA_VERSION {
            return Err(DefinitionMetadataError::UnsupportedSchemaVersion {
                actual: self.schema_version,
                supported: MODEL_DEFINITION_METADATA_SCHEMA_VERSION,
            });
        }
        if let Some(identity) = self.source_identity.as_ref() {
            validate_model_file_identity("source_identity", identity)?;
        }

        let parameter_indices = self.validate_parameters()?;
        let section_indices = self.validate_sections(&parameter_indices)?;
        self.validate_parameter_sources(&section_indices)?;
        self.validate_statistics(&parameter_indices)?;
        self.validate_temperature_laws(&parameter_indices)?;
        Ok(())
    }

    #[must_use]
    pub fn parameter(&self, name: &str) -> Option<&ParameterDefinition> {
        self.parameters
            .iter()
            .find(|parameter| identifier_eq(&parameter.name, name))
    }

    #[must_use]
    pub fn section(&self, name: &str) -> Option<&ModelSectionDefinition> {
        self.sections
            .iter()
            .find(|section| identifier_eq(&section.name, name))
    }

    #[must_use]
    pub fn statistical_variable(&self, name: &str) -> Option<&StatisticalVariableDefinition> {
        self.statistics
            .variables
            .iter()
            .find(|variable| identifier_eq(&variable.name, name))
    }

    #[must_use]
    pub fn temperature_law(&self, quantity: &str) -> Option<&TemperatureLawDefinition> {
        self.temperature_laws
            .iter()
            .find(|law| identifier_eq(&law.quantity, quantity))
    }

    /// Evaluate one declared law using the same typed parameter values that
    /// are persisted with the definition. Equation variables are
    /// case-insensitive: `T`/`TK` are absolute kelvin, `TEMP`/`TEMPER` are
    /// degrees Celsius, and `TREF`/`TREF_C` are the declared reference values.
    pub fn evaluate_temperature_law(
        &self,
        quantity: &str,
        temperature_c: f64,
    ) -> Result<TemperatureLawEvaluation, DefinitionMetadataError> {
        self.validate()?;
        let requested_temperature_c =
            FiniteF64::new(temperature_c).map_err(|_| DefinitionMetadataError::Invalid {
                path: "temperature".to_owned(),
                message: "requested temperature must be finite".to_owned(),
            })?;
        let law =
            self.temperature_law(quantity)
                .ok_or_else(|| DefinitionMetadataError::Invalid {
                    path: "temperature_laws".to_owned(),
                    message: format!("temperature quantity '{quantity}' is not declared"),
                })?;
        evaluate_temperature_law(self, law, requested_temperature_c)
    }

    fn validate_parameters(&self) -> Result<BTreeMap<String, usize>, DefinitionMetadataError> {
        let mut indices = BTreeMap::new();
        for (index, parameter) in self.parameters.iter().enumerate() {
            let path = format!("parameters[{index}]");
            validate_identifier(&format!("{path}.name"), &parameter.name)?;
            let canonical = canonical_identifier(&parameter.name);
            if let Some(first) = indices.insert(canonical, index) {
                return invalid(
                    &format!("{path}.name"),
                    format!(
                        "duplicate case-insensitive parameter identifier; first declared at parameters[{first}]"
                    ),
                );
            }
            validate_description(&format!("{path}.description"), &parameter.description)?;
            validate_parameter_value(
                &format!("{path}.value"),
                &parameter.value,
                parameter.data_type,
            )?;

            match parameter.data_type {
                ParameterDataType::Numeric => {
                    if let Some(bounds) = parameter.bounds {
                        bounds.validate(&format!("{path}.bounds"))?;
                        if let ParameterValue::Numeric(value) = parameter.value
                            && !bounds.contains(value)
                        {
                            return invalid(
                                &format!("{path}.value"),
                                "numeric value lies outside its declared inclusive bounds",
                            );
                        }
                    }
                    if let Some(unit) = &parameter.unit {
                        validate_display_text(&format!("{path}.unit"), unit, false)?;
                    }
                }
                ParameterDataType::String => {
                    if parameter.bounds.is_some() {
                        return invalid(
                            &format!("{path}.bounds"),
                            "string parameters cannot declare numeric bounds",
                        );
                    }
                    if parameter.unit.is_some() {
                        return invalid(
                            &format!("{path}.unit"),
                            "string parameters cannot declare a physical unit",
                        );
                    }
                }
            }

            if let ParameterSource::Declared { source } = &parameter.source {
                validate_display_text(&format!("{path}.source.source"), source, false)?;
            }
        }
        Ok(indices)
    }

    fn validate_sections(
        &self,
        parameter_indices: &BTreeMap<String, usize>,
    ) -> Result<BTreeMap<String, usize>, DefinitionMetadataError> {
        let mut indices = BTreeMap::new();
        for (index, section) in self.sections.iter().enumerate() {
            let path = format!("sections[{index}]");
            validate_identifier(&format!("{path}.name"), &section.name)?;
            let canonical = canonical_identifier(&section.name);
            if let Some(first) = indices.insert(canonical, index) {
                return invalid(
                    &format!("{path}.name"),
                    format!(
                        "duplicate case-insensitive section identifier; first declared at sections[{first}]"
                    ),
                );
            }

            let mut override_names = BTreeSet::new();
            for (name, value) in &section.overrides {
                let override_path = format!("{path}.overrides[{name:?}]");
                validate_identifier(&override_path, name)?;
                let canonical_name = canonical_identifier(name);
                if !override_names.insert(canonical_name.clone()) {
                    return invalid(
                        &override_path,
                        "duplicate case-insensitive override identifier",
                    );
                }
                let Some(parameter_index) = parameter_indices.get(&canonical_name) else {
                    return invalid(&override_path, "override references an unknown parameter");
                };
                let parameter = &self.parameters[*parameter_index];
                validate_parameter_value(&override_path, value, parameter.data_type)?;
                if let (Some(bounds), ParameterValue::Numeric(value)) = (parameter.bounds, value)
                    && !bounds.contains(*value)
                {
                    return invalid(
                        &override_path,
                        "numeric override lies outside the parameter's inclusive bounds",
                    );
                }
            }

            if section.model_files.is_empty() {
                return invalid(
                    &format!("{path}.model_files"),
                    "a named section must identify at least one contributing model file",
                );
            }
            let mut file_ids = BTreeSet::new();
            for (file_index, file) in section.model_files.iter().enumerate() {
                let file_path = format!("{path}.model_files[{file_index}]");
                validate_model_file_identity(&file_path, file)?;
                if !file_ids.insert(file.source_id.clone()) {
                    return invalid(
                        &format!("{file_path}.source_id"),
                        "duplicate source identity within section",
                    );
                }
            }
            validate_qualification(&format!("{path}.qualification"), &section.qualification)?;
        }

        for (index, section) in self.sections.iter().enumerate() {
            if let Some(parent) = &section.parent {
                validate_identifier(&format!("sections[{index}].parent"), parent)?;
                if !indices.contains_key(&canonical_identifier(parent)) {
                    return invalid(
                        &format!("sections[{index}].parent"),
                        format!("unknown parent section '{parent}'"),
                    );
                }
            }
        }
        self.validate_section_cycles(&indices)?;
        Ok(indices)
    }

    fn validate_section_cycles(
        &self,
        section_indices: &BTreeMap<String, usize>,
    ) -> Result<(), DefinitionMetadataError> {
        let mut complete = vec![false; self.sections.len()];
        for start in 0..self.sections.len() {
            if complete[start] {
                continue;
            }
            let mut positions = BTreeMap::<usize, usize>::new();
            let mut trail = Vec::<usize>::new();
            let mut current = Some(start);
            while let Some(index) = current {
                if complete[index] {
                    break;
                }
                if let Some(cycle_start) = positions.get(&index).copied() {
                    let mut names = trail[cycle_start..]
                        .iter()
                        .map(|member| self.sections[*member].name.as_str())
                        .collect::<Vec<_>>();
                    names.push(self.sections[index].name.as_str());
                    return invalid(
                        &format!("sections[{index}].parent"),
                        format!("section inheritance cycle: {}", names.join(" -> ")),
                    );
                }
                positions.insert(index, trail.len());
                trail.push(index);
                current = self.sections[index]
                    .parent
                    .as_ref()
                    .map(|parent| section_indices[&canonical_identifier(parent)]);
            }
            for index in trail {
                complete[index] = true;
            }
        }
        Ok(())
    }

    fn validate_parameter_sources(
        &self,
        section_indices: &BTreeMap<String, usize>,
    ) -> Result<(), DefinitionMetadataError> {
        for (index, parameter) in self.parameters.iter().enumerate() {
            let (section_name, must_override) = match &parameter.source {
                ParameterSource::Declared { .. } => continue,
                ParameterSource::Inherited { from_section } => (from_section, false),
                ParameterSource::Overridden { section } => (section, true),
            };
            let Some(section_index) = section_indices.get(&canonical_identifier(section_name))
            else {
                return invalid(
                    &format!("parameters[{index}].source"),
                    format!("source references unknown section '{section_name}'"),
                );
            };
            if must_override
                && !self.sections[*section_index]
                    .overrides
                    .iter()
                    .any(|(name, value)| {
                        identifier_eq(name, &parameter.name) && value == &parameter.value
                    })
            {
                return invalid(
                    &format!("parameters[{index}].source"),
                    format!(
                        "parameter is marked overridden by section '{section_name}', but that section has no matching effective override"
                    ),
                );
            }
        }
        Ok(())
    }

    fn validate_statistics(
        &self,
        parameter_indices: &BTreeMap<String, usize>,
    ) -> Result<(), DefinitionMetadataError> {
        let mut variable_indices = BTreeMap::new();
        let mut group_members = BTreeMap::<String, BTreeSet<String>>::new();
        for (index, variable) in self.statistics.variables.iter().enumerate() {
            let path = format!("statistics.variables[{index}]");
            validate_identifier(&format!("{path}.name"), &variable.name)?;
            let variable_name = canonical_identifier(&variable.name);
            if let Some(first) = variable_indices.insert(variable_name.clone(), index) {
                return invalid(
                    &format!("{path}.name"),
                    format!(
                        "duplicate case-insensitive statistical-variable identifier; first declared at statistics.variables[{first}]"
                    ),
                );
            }
            validate_identifier(&format!("{path}.parameter"), &variable.parameter)?;
            let Some(parameter_index) =
                parameter_indices.get(&canonical_identifier(&variable.parameter))
            else {
                return invalid(
                    &format!("{path}.parameter"),
                    "statistical variable references an unknown parameter",
                );
            };
            let parameter = &self.parameters[*parameter_index];
            validate_distribution(
                &format!("{path}.distribution"),
                &variable.distribution,
                parameter,
            )?;
            validate_description(&format!("{path}.description"), &variable.description)?;
            if let Some(group) = &variable.correlation_group {
                validate_identifier(&format!("{path}.correlation_group"), group)?;
                group_members
                    .entry(canonical_identifier(group))
                    .or_default()
                    .insert(variable_name);
            }
        }

        let mut matrix_groups = BTreeSet::new();
        for (index, matrix) in self.statistics.correlation_matrices.iter().enumerate() {
            let path = format!("statistics.correlation_matrices[{index}]");
            validate_identifier(&format!("{path}.group"), &matrix.group)?;
            let group = canonical_identifier(&matrix.group);
            if !matrix_groups.insert(group.clone()) {
                return invalid(
                    &format!("{path}.group"),
                    "duplicate case-insensitive correlation-matrix group",
                );
            }
            let Some(expected_members) = group_members.get(&group) else {
                return invalid(
                    &format!("{path}.group"),
                    "correlation matrix has no statistical variables in its group",
                );
            };
            validate_correlation_matrix(
                path.as_str(),
                matrix,
                expected_members,
                &variable_indices,
            )?;
        }

        for (group, members) in group_members {
            if members.len() > 1 && !matrix_groups.contains(&group) {
                return invalid(
                    "statistics.correlation_matrices",
                    format!(
                        "correlation group '{group}' has {} variables but no matrix",
                        members.len()
                    ),
                );
            }
        }
        Ok(())
    }

    fn validate_temperature_laws(
        &self,
        parameter_indices: &BTreeMap<String, usize>,
    ) -> Result<(), DefinitionMetadataError> {
        let mut quantities = BTreeMap::new();
        for (index, law) in self.temperature_laws.iter().enumerate() {
            let path = format!("temperature_laws[{index}]");
            validate_display_text(&format!("{path}.quantity"), &law.quantity, false)?;
            let quantity = canonical_identifier(&law.quantity);
            if let Some(first) = quantities.insert(quantity, index) {
                return invalid(
                    &format!("{path}.quantity"),
                    format!(
                        "duplicate case-insensitive temperature quantity; first declared at temperature_laws[{first}]"
                    ),
                );
            }
            validate_description(&format!("{path}.description"), &law.description)?;
            law.valid_range.validate(&format!("{path}.valid_range"))?;
            if !law.valid_range.contains(law.reference_temperature_c) {
                return invalid(
                    &format!("{path}.reference_temperature_c"),
                    "reference temperature lies outside the valid range",
                );
            }
            if law.parameters.is_empty() {
                return invalid(
                    &format!("{path}.parameters"),
                    "a temperature law must declare at least one parameter dependency",
                );
            }
            let mut dependencies = BTreeSet::new();
            for (parameter_index, parameter) in law.parameters.iter().enumerate() {
                let dependency_path = format!("{path}.parameters[{parameter_index}]");
                validate_identifier(&dependency_path, parameter)?;
                let parameter = canonical_identifier(parameter);
                if !dependencies.insert(parameter.clone()) {
                    return invalid(
                        &dependency_path,
                        "duplicate case-insensitive parameter dependency",
                    );
                }
                let Some(index) = parameter_indices.get(&parameter) else {
                    return invalid(
                        &dependency_path,
                        "temperature law references an unknown parameter",
                    );
                };
                if self.parameters[*index].data_type != ParameterDataType::Numeric {
                    return invalid(
                        &dependency_path,
                        "temperature laws can reference only numeric parameters",
                    );
                }
            }
            validate_temperature_representation(
                &format!("{path}.representation"),
                &law.representation,
                law.valid_range,
            )?;
            match &law.representation {
                TemperatureLawRepresentation::Equation { expression } => {
                    let expression_dependencies = temperature_expression_dependencies(expression)
                        .map_err(|message| {
                        DefinitionMetadataError::Invalid {
                            path: format!("{path}.representation.expression"),
                            message,
                        }
                    })?;
                    if expression_dependencies != dependencies {
                        return invalid(
                            &format!("{path}.parameters"),
                            format!(
                                "declared dependencies {dependencies:?} do not exactly match equation references {expression_dependencies:?}"
                            ),
                        );
                    }
                    evaluate_temperature_equation(
                        self,
                        law,
                        expression,
                        law.reference_temperature_c,
                    )?;
                }
                TemperatureLawRepresentation::LookupTable {
                    interpolation,
                    points,
                } => {
                    evaluate_temperature_table(
                        *interpolation,
                        points,
                        law.reference_temperature_c,
                    )?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DefinitionMetadataError {
    #[error(
        "unsupported model-definition metadata schema {actual}; supported schema is {supported}"
    )]
    UnsupportedSchemaVersion { actual: u16, supported: u16 },
    #[error("{path}: {message}")]
    Invalid { path: String, message: String },
}

fn invalid<T>(path: &str, message: impl Into<String>) -> Result<T, DefinitionMetadataError> {
    Err(DefinitionMetadataError::Invalid {
        path: path.to_owned(),
        message: message.into(),
    })
}

fn canonical_identifier(value: &str) -> String {
    value.to_ascii_lowercase()
}

fn identifier_eq(left: &str, right: &str) -> bool {
    left.eq_ignore_ascii_case(right)
}

fn validate_identifier(path: &str, value: &str) -> Result<(), DefinitionMetadataError> {
    if value.is_empty() || value.trim() != value {
        return invalid(
            path,
            "identifier must not be empty or contain outer whitespace",
        );
    }
    let mut characters = value.chars();
    let first = characters.next().expect("non-empty identifier");
    if !(first.is_ascii_alphabetic() || first == '_')
        || !characters.all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '_' | '.' | '$' | '-')
        })
    {
        return invalid(path, format!("'{value}' is not a safe model identifier"));
    }
    Ok(())
}

fn validate_display_text(
    path: &str,
    value: &str,
    allow_empty: bool,
) -> Result<(), DefinitionMetadataError> {
    if (!allow_empty && value.trim().is_empty()) || value.trim() != value {
        return invalid(path, "text must not be empty or contain outer whitespace");
    }
    if value
        .chars()
        .any(|character| character == '\0' || (character.is_control() && character != '\n'))
    {
        return invalid(path, "text contains a forbidden control character");
    }
    Ok(())
}

fn validate_description(path: &str, value: &str) -> Result<(), DefinitionMetadataError> {
    validate_display_text(path, value, false)
}

fn validate_opaque_identity(path: &str, value: &str) -> Result<(), DefinitionMetadataError> {
    validate_display_text(path, value, false)?;
    if value.chars().any(char::is_whitespace) {
        return invalid(path, "identity must not contain whitespace");
    }
    Ok(())
}

fn validate_model_file_identity(
    path: &str,
    identity: &ModelFileIdentity,
) -> Result<(), DefinitionMetadataError> {
    validate_opaque_identity(&format!("{path}.source_id"), &identity.source_id)?;
    if identity.revision == 0 {
        return invalid(
            &format!("{path}.revision"),
            "revision must be greater than zero",
        );
    }
    validate_sha256(&format!("{path}.content_digest"), &identity.content_digest)?;
    validate_display_text(
        &format!("{path}.display_name"),
        &identity.display_name,
        false,
    )
}

fn validate_sha256(path: &str, value: &str) -> Result<(), DefinitionMetadataError> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return invalid(
            path,
            "digest must be 64 lowercase hexadecimal SHA-256 characters",
        );
    }
    Ok(())
}

fn validate_parameter_value(
    path: &str,
    value: &ParameterValue,
    expected: ParameterDataType,
) -> Result<(), DefinitionMetadataError> {
    if value.data_type() != expected {
        return invalid(
            path,
            format!(
                "value type {:?} does not match declared parameter type {expected:?}",
                value.data_type()
            ),
        );
    }
    if let ParameterValue::String(value) = value {
        validate_display_text(path, value, false)?;
    }
    Ok(())
}

fn validate_qualification(
    path: &str,
    qualification: &ModelSectionQualification,
) -> Result<(), DefinitionMetadataError> {
    match qualification {
        ModelSectionQualification::Qualified {
            evidence_digest: Some(digest),
        } => validate_sha256(&format!("{path}.evidence_digest"), digest),
        ModelSectionQualification::Failed { summary } => {
            validate_display_text(&format!("{path}.summary"), summary, false)
        }
        ModelSectionQualification::Qualified {
            evidence_digest: None,
        } => invalid(
            &format!("{path}.evidence_digest"),
            "a qualified model section requires the exact retained evidence digest",
        ),
        ModelSectionQualification::Unqualified | ModelSectionQualification::Pending => Ok(()),
    }
}

fn validate_distribution(
    path: &str,
    distribution: &StatisticalDistribution,
    parameter: &ParameterDefinition,
) -> Result<(), DefinitionMetadataError> {
    match distribution {
        StatisticalDistribution::Normal { sigma } => {
            require_numeric_parameter(path, parameter)?;
            if sigma.get() <= 0.0 {
                return invalid(
                    &format!("{path}.sigma"),
                    "normal sigma must be greater than zero",
                );
            }
        }
        StatisticalDistribution::Lognormal { sigma_fraction } => {
            require_numeric_parameter(path, parameter)?;
            if sigma_fraction.get() <= 0.0 {
                return invalid(
                    &format!("{path}.sigma_fraction"),
                    "lognormal sigma fraction must be greater than zero",
                );
            }
            if !matches!(parameter.value, ParameterValue::Numeric(value) if value.get() > 0.0) {
                return invalid(
                    path,
                    "lognormal distribution requires a positive nominal parameter value",
                );
            }
        }
        StatisticalDistribution::Uniform { lower, upper } => {
            require_numeric_parameter(path, parameter)?;
            if lower >= upper {
                return invalid(
                    path,
                    "uniform lower endpoint must be less than upper endpoint",
                );
            }
            if let Some(bounds) = parameter.bounds
                && (!bounds.contains(*lower) || !bounds.contains(*upper))
            {
                return invalid(path, "uniform endpoints lie outside the parameter bounds");
            }
        }
        StatisticalDistribution::Discrete { outcomes } => {
            if outcomes.is_empty() {
                return invalid(
                    &format!("{path}.outcomes"),
                    "at least one outcome is required",
                );
            }
            let mut probability_sum = 0.0;
            for (index, outcome) in outcomes.iter().enumerate() {
                let outcome_path = format!("{path}.outcomes[{index}]");
                validate_parameter_value(
                    &format!("{outcome_path}.value"),
                    &outcome.value,
                    parameter.data_type,
                )?;
                if let (Some(bounds), ParameterValue::Numeric(value)) =
                    (parameter.bounds, &outcome.value)
                    && !bounds.contains(*value)
                {
                    return invalid(
                        &format!("{outcome_path}.value"),
                        "discrete outcome lies outside the parameter bounds",
                    );
                }
                if outcomes[..index]
                    .iter()
                    .any(|prior| prior.value == outcome.value)
                {
                    return invalid(
                        &format!("{outcome_path}.value"),
                        "duplicate discrete outcome",
                    );
                }
                let probability = outcome.probability.get();
                if probability <= 0.0 || probability > 1.0 {
                    return invalid(
                        &format!("{outcome_path}.probability"),
                        "probability must be greater than zero and at most one",
                    );
                }
                probability_sum += probability;
            }
            if (probability_sum - 1.0).abs() > PROBABILITY_SUM_TOLERANCE {
                return invalid(
                    &format!("{path}.outcomes"),
                    format!("outcome probabilities must sum to one, got {probability_sum}"),
                );
            }
        }
    }
    Ok(())
}

fn require_numeric_parameter(
    path: &str,
    parameter: &ParameterDefinition,
) -> Result<(), DefinitionMetadataError> {
    if parameter.data_type != ParameterDataType::Numeric {
        return invalid(path, "distribution requires a numeric target parameter");
    }
    Ok(())
}

fn validate_correlation_matrix(
    path: &str,
    matrix: &CorrelationMatrix,
    expected_members: &BTreeSet<String>,
    variable_indices: &BTreeMap<String, usize>,
) -> Result<(), DefinitionMetadataError> {
    if matrix.variables.is_empty() {
        return invalid(
            &format!("{path}.variables"),
            "matrix must contain at least one variable",
        );
    }
    let mut actual_members = BTreeSet::new();
    for (index, variable) in matrix.variables.iter().enumerate() {
        let variable_path = format!("{path}.variables[{index}]");
        validate_identifier(&variable_path, variable)?;
        let canonical = canonical_identifier(variable);
        if !actual_members.insert(canonical.clone()) {
            return invalid(&variable_path, "duplicate case-insensitive matrix variable");
        }
        if !variable_indices.contains_key(&canonical) {
            return invalid(
                &variable_path,
                "matrix references an unknown statistical variable",
            );
        }
    }
    if &actual_members != expected_members {
        return invalid(
            &format!("{path}.variables"),
            "matrix membership must exactly match its correlation group",
        );
    }

    let dimension = matrix.variables.len();
    if matrix.coefficients.len() != dimension {
        return invalid(
            &format!("{path}.coefficients"),
            format!("matrix must have {dimension} rows"),
        );
    }
    for (row, coefficients) in matrix.coefficients.iter().enumerate() {
        if coefficients.len() != dimension {
            return invalid(
                &format!("{path}.coefficients[{row}]"),
                format!("matrix row must have {dimension} columns"),
            );
        }
        for (column, coefficient) in coefficients.iter().enumerate() {
            if !(-1.0..=1.0).contains(&coefficient.get()) {
                return invalid(
                    &format!("{path}.coefficients[{row}][{column}]"),
                    "correlation coefficient must lie in [-1, 1]",
                );
            }
        }
        if (coefficients[row].get() - 1.0).abs() > MATRIX_ELEMENT_TOLERANCE {
            return invalid(
                &format!("{path}.coefficients[{row}][{row}]"),
                "correlation diagonal must equal one",
            );
        }
    }
    for row in 0..dimension {
        for column in 0..row {
            let left = matrix.coefficients[row][column].get();
            let right = matrix.coefficients[column][row].get();
            if (left - right).abs() > MATRIX_ELEMENT_TOLERANCE {
                return invalid(
                    &format!("{path}.coefficients[{row}][{column}]"),
                    "correlation matrix must be symmetric",
                );
            }
        }
    }
    validate_positive_semidefinite(path, &matrix.coefficients)
}

/// Deterministic LDL^T factorization with a small numerical tolerance. A zero
/// pivot is accepted only when the corresponding residuals are also zero,
/// which admits semidefinite matrices without accepting an indefinite one.
fn validate_positive_semidefinite(
    path: &str,
    matrix: &[Vec<FiniteF64>],
) -> Result<(), DefinitionMetadataError> {
    let dimension = matrix.len();
    let tolerance = MATRIX_PSD_TOLERANCE * (dimension.max(1) as f64);
    let mut lower = vec![vec![0.0_f64; dimension]; dimension];
    let mut diagonal = vec![0.0_f64; dimension];

    for row in 0..dimension {
        lower[row][row] = 1.0;
        for column in 0..row {
            let correction = (0..column)
                .map(|k| lower[row][k] * diagonal[k] * lower[column][k])
                .sum::<f64>();
            let residual = matrix[row][column].get() - correction;
            if diagonal[column].abs() <= tolerance {
                if residual.abs() > tolerance {
                    return invalid(
                        &format!("{path}.coefficients"),
                        "correlation matrix is not positive semidefinite",
                    );
                }
                lower[row][column] = 0.0;
            } else {
                lower[row][column] = residual / diagonal[column];
            }
        }
        let correction = (0..row)
            .map(|k| lower[row][k] * lower[row][k] * diagonal[k])
            .sum::<f64>();
        let pivot = matrix[row][row].get() - correction;
        if pivot < -tolerance {
            return invalid(
                &format!("{path}.coefficients"),
                "correlation matrix is not positive semidefinite",
            );
        }
        diagonal[row] = if pivot.abs() <= tolerance { 0.0 } else { pivot };
    }
    Ok(())
}

fn validate_temperature_representation(
    path: &str,
    representation: &TemperatureLawRepresentation,
    valid_range: TemperatureRange,
) -> Result<(), DefinitionMetadataError> {
    match representation {
        TemperatureLawRepresentation::Equation { expression } => {
            validate_display_text(&format!("{path}.expression"), expression, false)
        }
        TemperatureLawRepresentation::LookupTable {
            interpolation,
            points,
        } => {
            if points.len() < 2 {
                return invalid(
                    &format!("{path}.points"),
                    "lookup table requires at least two points",
                );
            }
            for index in 1..points.len() {
                if points[index - 1].temperature_c >= points[index].temperature_c {
                    return invalid(
                        &format!("{path}.points[{index}].temperature_c"),
                        "lookup temperatures must be strictly increasing",
                    );
                }
            }
            if points[0].temperature_c > valid_range.minimum_c
                || points[points.len() - 1].temperature_c < valid_range.maximum_c
            {
                return invalid(
                    &format!("{path}.points"),
                    "lookup table must cover the complete declared valid range",
                );
            }
            if *interpolation == LookupInterpolation::LogLinear
                && points.iter().any(|point| point.value.get() <= 0.0)
            {
                return invalid(
                    &format!("{path}.points"),
                    "log-linear interpolation requires positive table values",
                );
            }
            Ok(())
        }
    }
}

fn evaluate_temperature_law(
    metadata: &ModelDefinitionMetadata,
    law: &TemperatureLawDefinition,
    requested_temperature_c: FiniteF64,
) -> Result<TemperatureLawEvaluation, DefinitionMetadataError> {
    let outside_declared_range = !law.valid_range.contains(requested_temperature_c);
    let effective_temperature_c = if outside_declared_range {
        match law.extrapolation {
            TemperatureExtrapolationPolicy::BlockOutsideRange => {
                return invalid(
                    "temperature",
                    format!(
                        "{} °C is outside the declared range {}..{} °C for '{}'",
                        requested_temperature_c,
                        law.valid_range.minimum_c,
                        law.valid_range.maximum_c,
                        law.quantity
                    ),
                );
            }
            TemperatureExtrapolationPolicy::Warn => requested_temperature_c,
            TemperatureExtrapolationPolicy::Clamp => {
                FiniteF64::new(requested_temperature_c.get().clamp(
                    law.valid_range.minimum_c.get(),
                    law.valid_range.maximum_c.get(),
                ))
                .expect("clamping finite values produces a finite value")
            }
        }
    } else {
        requested_temperature_c
    };

    let value = match &law.representation {
        TemperatureLawRepresentation::Equation { expression } => {
            evaluate_temperature_equation(metadata, law, expression, effective_temperature_c)?
        }
        TemperatureLawRepresentation::LookupTable {
            interpolation,
            points,
        } => evaluate_temperature_table(*interpolation, points, effective_temperature_c)?,
    };
    Ok(TemperatureLawEvaluation {
        requested_temperature_c,
        effective_temperature_c,
        value,
        outside_declared_range,
    })
}

fn evaluate_temperature_equation(
    metadata: &ModelDefinitionMetadata,
    law: &TemperatureLawDefinition,
    expression: &str,
    temperature_c: FiniteF64,
) -> Result<FiniteF64, DefinitionMetadataError> {
    let mut context = rspice_core::netlist::expr::ParamContext::new();
    for parameter in &metadata.parameters {
        if let ParameterValue::Numeric(value) = parameter.value {
            context.set(&parameter.name, value.get());
        }
    }
    let temperature_k = temperature_c.get() + 273.15;
    let reference_k = law.reference_temperature_c.get() + 273.15;
    context.set("T", temperature_k);
    context.set("TK", temperature_k);
    context.set("TEMP", temperature_c.get());
    context.set("TEMPER", temperature_c.get());
    context.set("TREF", reference_k);
    context.set("TREF_C", law.reference_temperature_c.get());
    let value =
        rspice_core::netlist::expr::eval_expression(expression, &context).map_err(|error| {
            DefinitionMetadataError::Invalid {
                path: format!(
                    "temperature_laws[{}].representation.expression",
                    law.quantity
                ),
                message: format!("temperature equation cannot be evaluated: {error}"),
            }
        })?;
    FiniteF64::new(value).map_err(|_| DefinitionMetadataError::Invalid {
        path: format!(
            "temperature_laws[{}].representation.expression",
            law.quantity
        ),
        message: "temperature equation produced a non-finite value".to_owned(),
    })
}

fn temperature_expression_dependencies(expression: &str) -> Result<BTreeSet<String>, String> {
    let parsed = rspice_core::netlist::expr::parse_expression(expression)
        .map_err(|error| format!("temperature equation cannot be parsed: {error}"))?;
    let mut dependencies = BTreeSet::new();
    collect_expression_parameters(&parsed, &mut dependencies);
    const RESERVED: [&str; 6] = ["t", "tk", "temp", "temper", "tref", "tref_c"];
    dependencies.retain(|name| !RESERVED.contains(&name.as_str()));
    Ok(dependencies)
}

fn collect_expression_parameters(
    expression: &rspice_core::netlist::expr::Expr,
    parameters: &mut BTreeSet<String>,
) {
    use rspice_core::netlist::expr::Expr;
    match expression {
        Expr::Param(name) => {
            parameters.insert(canonical_identifier(name));
        }
        Expr::BinOp { left, right, .. } => {
            collect_expression_parameters(left, parameters);
            collect_expression_parameters(right, parameters);
        }
        Expr::UnaryOp { operand, .. } => collect_expression_parameters(operand, parameters),
        Expr::FnCall { args, .. } => {
            for argument in args {
                collect_expression_parameters(argument, parameters);
            }
        }
        Expr::Number(_) | Expr::ComplexNumber(_) | Expr::StringLiteral(_) => {}
    }
}

fn evaluate_temperature_table(
    interpolation: LookupInterpolation,
    points: &[TemperaturePoint],
    temperature_c: FiniteF64,
) -> Result<FiniteF64, DefinitionMetadataError> {
    let target = temperature_c.get();
    let segment = if target <= points[0].temperature_c.get() {
        (&points[0], &points[1])
    } else if target >= points[points.len() - 1].temperature_c.get() {
        (&points[points.len() - 2], &points[points.len() - 1])
    } else {
        points
            .windows(2)
            .find(|pair| {
                pair[0].temperature_c.get() <= target && target <= pair[1].temperature_c.get()
            })
            .map(|pair| (&pair[0], &pair[1]))
            .ok_or_else(|| DefinitionMetadataError::Invalid {
                path: "temperature_laws[*].representation.points".to_owned(),
                message: "temperature table has no interval for the requested value".to_owned(),
            })?
    };
    let (left, right) = segment;
    let fraction = (target - left.temperature_c.get())
        / (right.temperature_c.get() - left.temperature_c.get());
    let value = match interpolation {
        LookupInterpolation::Linear => {
            left.value.get() + fraction * (right.value.get() - left.value.get())
        }
        LookupInterpolation::LogLinear => (left.value.get().ln()
            + fraction * (right.value.get().ln() - left.value.get().ln()))
        .exp(),
        LookupInterpolation::Step => {
            if target >= right.temperature_c.get() {
                right.value.get()
            } else {
                left.value.get()
            }
        }
    };
    FiniteF64::new(value).map_err(|_| DefinitionMetadataError::Invalid {
        path: "temperature_laws[*].representation.points".to_owned(),
        message: "temperature interpolation produced a non-finite value".to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn finite(value: f64) -> FiniteF64 {
        FiniteF64::new(value).expect("finite test value")
    }

    fn digest(character: char) -> String {
        std::iter::repeat_n(character, 64).collect()
    }

    fn file(name: &str, digest_character: char) -> ModelFileIdentity {
        ModelFileIdentity {
            source_id: format!("source:{name}"),
            revision: 1,
            content_digest: digest(digest_character),
            display_name: format!("{name}.lib"),
        }
    }

    fn numeric_parameter(name: &str, value: f64) -> ParameterDefinition {
        ParameterDefinition {
            name: name.to_owned(),
            data_type: ParameterDataType::Numeric,
            value: ParameterValue::Numeric(finite(value)),
            unit: Some("V".to_owned()),
            bounds: Some(FiniteBounds {
                lower: Some(finite(0.0)),
                upper: Some(finite(10.0)),
            }),
            source: ParameterSource::Declared {
                source: "base".to_owned(),
            },
            description: format!("Typed definition for {name}"),
        }
    }

    fn valid_metadata() -> ModelDefinitionMetadata {
        let mut vth0 = numeric_parameter("VTH0", 0.4821);
        vth0.bounds = Some(FiniteBounds {
            lower: Some(finite(0.35)),
            upper: Some(finite(0.65)),
        });
        vth0.source = ParameterSource::Overridden {
            section: "TT".to_owned(),
        };
        let mut u0 = numeric_parameter("U0", 0.0418);
        u0.unit = Some("m²/Vs".to_owned());
        u0.bounds = Some(FiniteBounds {
            lower: Some(finite(0.0)),
            upper: None,
        });
        let mode = ParameterDefinition {
            name: "MODE".to_owned(),
            data_type: ParameterDataType::String,
            value: ParameterValue::String("physical".to_owned()),
            unit: None,
            bounds: None,
            source: ParameterSource::Declared {
                source: "base".to_owned(),
            },
            description: "Model formulation selector".to_owned(),
        };
        let tnom = ParameterDefinition {
            name: "TNOM".to_owned(),
            data_type: ParameterDataType::Numeric,
            value: ParameterValue::Numeric(finite(300.15)),
            unit: Some("K".to_owned()),
            bounds: Some(FiniteBounds {
                lower: Some(finite(1.0)),
                upper: None,
            }),
            source: ParameterSource::Declared {
                source: "base".to_owned(),
            },
            description: "Nominal absolute model temperature".to_owned(),
        };
        let ute = ParameterDefinition {
            name: "UTE".to_owned(),
            data_type: ParameterDataType::Numeric,
            value: ParameterValue::Numeric(finite(-1.5)),
            unit: None,
            bounds: None,
            source: ParameterSource::Declared {
                source: "base".to_owned(),
            },
            description: "Mobility temperature exponent".to_owned(),
        };

        ModelDefinitionMetadata {
            schema_version: MODEL_DEFINITION_METADATA_SCHEMA_VERSION,
            source_identity: None,
            parameters: vec![vth0, u0, mode, tnom, ute],
            sections: vec![
                ModelSectionDefinition {
                    name: "BASE".to_owned(),
                    parent: None,
                    overrides: BTreeMap::new(),
                    model_files: vec![file("base", 'a')],
                    qualification: ModelSectionQualification::Qualified {
                        evidence_digest: Some(digest('b')),
                    },
                },
                ModelSectionDefinition {
                    name: "TT".to_owned(),
                    parent: Some("base".to_owned()),
                    overrides: BTreeMap::from([(
                        "vth0".to_owned(),
                        ParameterValue::Numeric(finite(0.4821)),
                    )]),
                    model_files: vec![file("tt", 'c')],
                    qualification: ModelSectionQualification::Pending,
                },
            ],
            statistics: StatisticalDefinition {
                variables: vec![
                    StatisticalVariableDefinition {
                        name: "dvth_global".to_owned(),
                        parameter: "VTH0".to_owned(),
                        distribution: StatisticalDistribution::Normal {
                            sigma: finite(0.018),
                        },
                        correlation_group: Some("process_voltage".to_owned()),
                        hierarchy: StatisticalHierarchyScope::Die,
                        description: "Die-level threshold variation".to_owned(),
                    },
                    StatisticalVariableDefinition {
                        name: "du0_global".to_owned(),
                        parameter: "U0".to_owned(),
                        distribution: StatisticalDistribution::Lognormal {
                            sigma_fraction: finite(0.042),
                        },
                        correlation_group: Some("process_voltage".to_owned()),
                        hierarchy: StatisticalHierarchyScope::Die,
                        description: "Die-level mobility variation".to_owned(),
                    },
                    StatisticalVariableDefinition {
                        name: "mode_selection".to_owned(),
                        parameter: "MODE".to_owned(),
                        distribution: StatisticalDistribution::Discrete {
                            outcomes: vec![
                                DiscreteOutcome {
                                    value: ParameterValue::String("physical".to_owned()),
                                    probability: finite(0.8),
                                },
                                DiscreteOutcome {
                                    value: ParameterValue::String("empirical".to_owned()),
                                    probability: finite(0.2),
                                },
                            ],
                        },
                        correlation_group: None,
                        hierarchy: StatisticalHierarchyScope::Global,
                        description: "Formulation selection fixture".to_owned(),
                    },
                ],
                correlation_matrices: vec![CorrelationMatrix {
                    group: "process_voltage".to_owned(),
                    variables: vec!["dvth_global".to_owned(), "du0_global".to_owned()],
                    coefficients: vec![
                        vec![finite(1.0), finite(0.25)],
                        vec![finite(0.25), finite(1.0)],
                    ],
                }],
            },
            temperature_laws: vec![
                TemperatureLawDefinition {
                    quantity: "mobility".to_owned(),
                    parameters: vec!["U0".to_owned(), "TNOM".to_owned(), "UTE".to_owned()],
                    representation: TemperatureLawRepresentation::Equation {
                        expression: "U0*(T/Tnom)^UTE".to_owned(),
                    },
                    reference_temperature_c: finite(27.0),
                    valid_range: TemperatureRange {
                        minimum_c: finite(-55.0),
                        maximum_c: finite(175.0),
                    },
                    extrapolation: TemperatureExtrapolationPolicy::BlockOutsideRange,
                    description: "Mobility temperature dependence".to_owned(),
                },
                TemperatureLawDefinition {
                    quantity: "threshold voltage".to_owned(),
                    parameters: vec!["VTH0".to_owned()],
                    representation: TemperatureLawRepresentation::LookupTable {
                        interpolation: LookupInterpolation::Linear,
                        points: vec![
                            TemperaturePoint {
                                temperature_c: finite(-55.0),
                                value: finite(0.55),
                            },
                            TemperaturePoint {
                                temperature_c: finite(27.0),
                                value: finite(0.4821),
                            },
                            TemperaturePoint {
                                temperature_c: finite(175.0),
                                value: finite(0.39),
                            },
                        ],
                    },
                    reference_temperature_c: finite(27.0),
                    valid_range: TemperatureRange {
                        minimum_c: finite(-55.0),
                        maximum_c: finite(175.0),
                    },
                    extrapolation: TemperatureExtrapolationPolicy::Warn,
                    description: "Qualified threshold interpolation".to_owned(),
                },
            ],
        }
    }

    fn assert_invalid(metadata: &ModelDefinitionMetadata, expected_path: &str) {
        let error = metadata.validate().expect_err("metadata must be rejected");
        match error {
            DefinitionMetadataError::Invalid { path, .. } => assert_eq!(path, expected_path),
            other => panic!("expected field validation error, got {other}"),
        }
    }

    #[test]
    fn complete_metadata_validates_and_round_trips_without_loss() {
        let metadata = valid_metadata();
        metadata.validate().expect("valid metadata");
        let encoded = serde_json::to_string_pretty(&metadata).expect("serialize metadata");
        let restored: ModelDefinitionMetadata =
            serde_json::from_str(&encoded).expect("deserialize metadata");
        assert_eq!(restored, metadata);
        restored
            .validate()
            .expect("restored metadata remains valid");
        assert_eq!(
            restored.parameter("vth0").map(|value| &value.name),
            Some(&"VTH0".to_owned())
        );
        assert!(restored.section("tt").is_some());
        assert!(restored.statistical_variable("DVTH_GLOBAL").is_some());
        assert!(restored.temperature_law("MOBILITY").is_some());
    }

    #[test]
    fn temperature_equations_and_tables_evaluate_with_declared_range_policy() {
        let metadata = valid_metadata();
        let reference = metadata
            .evaluate_temperature_law("mobility", 27.0)
            .expect("equation evaluates at reference");
        assert!((reference.value.get() - 0.0418).abs() < 1.0e-12);
        assert!(!reference.outside_declared_range);

        let table = metadata
            .evaluate_temperature_law("threshold voltage", 101.0)
            .expect("table interpolates");
        let expected = 0.4821 + (101.0 - 27.0) / (175.0 - 27.0) * (0.39 - 0.4821);
        assert!((table.value.get() - expected).abs() < 1.0e-12);

        let error = metadata
            .evaluate_temperature_law("mobility", 200.0)
            .expect_err("block policy rejects extrapolation");
        assert!(error.to_string().contains("outside the declared range"));

        let mut clamped = metadata.clone();
        clamped.temperature_laws[1].extrapolation = TemperatureExtrapolationPolicy::Clamp;
        let result = clamped
            .evaluate_temperature_law("threshold voltage", 250.0)
            .expect("clamp policy evaluates at boundary");
        assert!(result.outside_declared_range);
        assert_eq!(result.effective_temperature_c, finite(175.0));
        assert_eq!(result.value, finite(0.39));
    }

    #[test]
    fn unevaluable_temperature_equations_are_not_valid_metadata() {
        let mut metadata = valid_metadata();
        metadata.temperature_laws[0].representation = TemperatureLawRepresentation::Equation {
            expression: "U0 * missing_parameter".to_owned(),
        };
        let error = metadata.validate().expect_err("unbound equation must fail");
        assert!(error.to_string().contains("do not exactly match"));
    }

    #[test]
    fn finite_value_rejects_nan_and_infinities() {
        assert!(FiniteF64::new(f64::NAN).is_err());
        assert!(FiniteF64::new(f64::INFINITY).is_err());
        assert!(FiniteF64::new(f64::NEG_INFINITY).is_err());
        assert_eq!(FiniteF64::new(-0.0), FiniteF64::new(0.0));
    }

    #[test]
    fn rejects_future_schema_and_case_insensitive_parameter_duplicates() {
        let mut metadata = valid_metadata();
        metadata.schema_version += 1;
        assert!(matches!(
            metadata.validate(),
            Err(DefinitionMetadataError::UnsupportedSchemaVersion { .. })
        ));

        let mut metadata = valid_metadata();
        metadata.parameters.push(numeric_parameter("vth0", 0.5));
        assert_invalid(&metadata, "parameters[5].name");
    }

    #[test]
    fn rejects_value_type_bounds_and_override_contract_violations() {
        let mut metadata = valid_metadata();
        metadata.parameters[0].value = ParameterValue::String("wrong".to_owned());
        assert_invalid(&metadata, "parameters[0].value");

        let mut metadata = valid_metadata();
        metadata.parameters[0].bounds = Some(FiniteBounds {
            lower: Some(finite(1.0)),
            upper: Some(finite(0.0)),
        });
        assert_invalid(&metadata, "parameters[0].bounds");

        let mut metadata = valid_metadata();
        metadata.sections[1]
            .overrides
            .insert("missing".to_owned(), ParameterValue::Numeric(finite(1.0)));
        assert_invalid(&metadata, "sections[1].overrides[\"missing\"]");

        let mut metadata = valid_metadata();
        metadata.sections[1].overrides.clear();
        assert_invalid(&metadata, "parameters[0].source");
    }

    #[test]
    fn rejects_duplicate_sections_missing_parents_and_cycles() {
        let mut metadata = valid_metadata();
        let mut duplicate = metadata.sections[1].clone();
        duplicate.name = "tt".to_owned();
        metadata.sections.push(duplicate);
        assert_invalid(&metadata, "sections[2].name");

        let mut metadata = valid_metadata();
        metadata.sections[1].parent = Some("UNKNOWN".to_owned());
        assert_invalid(&metadata, "sections[1].parent");

        let mut metadata = valid_metadata();
        metadata.sections[0].parent = Some("TT".to_owned());
        assert_invalid(&metadata, "sections[0].parent");
    }

    #[test]
    fn rejects_bad_file_identity_and_qualification_evidence() {
        let mut metadata = valid_metadata();
        metadata.sections[0].model_files[0].content_digest = "ABC".to_owned();
        assert_invalid(&metadata, "sections[0].model_files[0].content_digest");

        let mut metadata = valid_metadata();
        metadata.sections[0].qualification = ModelSectionQualification::Failed {
            summary: "".to_owned(),
        };
        assert_invalid(&metadata, "sections[0].qualification.summary");
    }

    #[test]
    fn rejects_missing_statistical_references_and_invalid_distributions() {
        let mut metadata = valid_metadata();
        metadata.statistics.variables[0].parameter = "NO_SUCH_PARAMETER".to_owned();
        assert_invalid(&metadata, "statistics.variables[0].parameter");

        let mut metadata = valid_metadata();
        metadata.statistics.variables[0].distribution =
            StatisticalDistribution::Normal { sigma: finite(0.0) };
        assert_invalid(&metadata, "statistics.variables[0].distribution.sigma");

        let mut metadata = valid_metadata();
        let StatisticalDistribution::Discrete { outcomes } =
            &mut metadata.statistics.variables[2].distribution
        else {
            panic!("fixture uses discrete outcomes")
        };
        outcomes[0].probability = finite(0.7);
        assert_invalid(&metadata, "statistics.variables[2].distribution.outcomes");
    }

    #[test]
    fn rejects_missing_asymmetric_and_indefinite_correlation_matrices() {
        let mut metadata = valid_metadata();
        metadata.statistics.correlation_matrices.clear();
        assert_invalid(&metadata, "statistics.correlation_matrices");

        let mut metadata = valid_metadata();
        metadata.statistics.correlation_matrices[0].coefficients[1][0] = finite(0.5);
        assert_invalid(
            &metadata,
            "statistics.correlation_matrices[0].coefficients[1][0]",
        );

        let mut metadata = valid_metadata();
        metadata
            .statistics
            .variables
            .push(StatisticalVariableDefinition {
                name: "third".to_owned(),
                parameter: "VTH0".to_owned(),
                distribution: StatisticalDistribution::Normal {
                    sigma: finite(0.01),
                },
                correlation_group: Some("process_voltage".to_owned()),
                hierarchy: StatisticalHierarchyScope::Die,
                description: "Third correlated variable".to_owned(),
            });
        let matrix = &mut metadata.statistics.correlation_matrices[0];
        matrix.variables.push("third".to_owned());
        matrix.coefficients = vec![
            vec![finite(1.0), finite(0.9), finite(0.9)],
            vec![finite(0.9), finite(1.0), finite(-0.9)],
            vec![finite(0.9), finite(-0.9), finite(1.0)],
        ];
        assert_invalid(&metadata, "statistics.correlation_matrices[0].coefficients");
    }

    #[test]
    fn accepts_singular_positive_semidefinite_correlation_matrix() {
        let mut metadata = valid_metadata();
        metadata.statistics.correlation_matrices[0].coefficients = vec![
            vec![finite(1.0), finite(1.0)],
            vec![finite(1.0), finite(1.0)],
        ];
        metadata.validate().expect("singular PSD matrix is valid");
    }

    #[test]
    fn rejects_temperature_reference_dependencies_ranges_and_tables() {
        let mut metadata = valid_metadata();
        metadata.temperature_laws[0].parameters = vec!["UNKNOWN".to_owned()];
        assert_invalid(&metadata, "temperature_laws[0].parameters[0]");

        let mut metadata = valid_metadata();
        metadata.temperature_laws[0].reference_temperature_c = finite(200.0);
        assert_invalid(&metadata, "temperature_laws[0].reference_temperature_c");

        let mut metadata = valid_metadata();
        metadata.temperature_laws[1].valid_range = TemperatureRange {
            minimum_c: finite(100.0),
            maximum_c: finite(100.0),
        };
        assert_invalid(&metadata, "temperature_laws[1].valid_range");

        let mut metadata = valid_metadata();
        let TemperatureLawRepresentation::LookupTable { points, .. } =
            &mut metadata.temperature_laws[1].representation
        else {
            panic!("fixture uses lookup table")
        };
        points.swap(0, 1);
        assert_invalid(
            &metadata,
            "temperature_laws[1].representation.points[1].temperature_c",
        );
    }

    #[test]
    fn rejects_unknown_serialized_fields() {
        let metadata = valid_metadata();
        let mut value = serde_json::to_value(metadata).expect("serialize fixture");
        value
            .as_object_mut()
            .expect("metadata object")
            .insert("invented".to_owned(), serde_json::Value::Bool(true));
        assert!(serde_json::from_value::<ModelDefinitionMetadata>(value).is_err());
    }
}
