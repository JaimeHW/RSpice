//! Saved-output records, and the field validators the project model shares.
//!
//! A saved output names a signal, how precisely to keep it, whether it may
//! stream, and what makes a retained result compatible with the request.
//! It also owns the bounded-text, parameter-name and design-quantity
//! validators that the rest of the project model reuses. These are project
//! data with no dependency on `ProjectWorkspace` itself,
//! so they live beside it rather than inside it.

use std::collections::HashMap;

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

use super::DesignVariableQuantity;

use crate::product::{AnalysisInstanceId, ObjectRevision, SavedOutputId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedOutputKind {
    RawVoltageOrCurrent,
    DerivedExpression,
    DeviceOperatingPointQuantity,
    NoiseContributor,
    RfPortQuantity,
}

impl SavedOutputKind {
    pub const ALL: [Self; 5] = [
        Self::RawVoltageOrCurrent,
        Self::DerivedExpression,
        Self::DeviceOperatingPointQuantity,
        Self::NoiseContributor,
        Self::RfPortQuantity,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::RawVoltageOrCurrent => "Raw voltage / current",
            Self::DerivedExpression => "Derived expression",
            Self::DeviceOperatingPointQuantity => "Device operating-point quantity",
            Self::NoiseContributor => "Noise contributor",
            Self::RfPortQuantity => "RF port quantity",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SavedOutputCompatibility {
    OpTranAc,
    AllCompatibleAnalyses,
    SelectedAnalysis { analysis_id: AnalysisInstanceId },
}

impl SavedOutputCompatibility {
    pub const fn label(&self) -> &'static str {
        match self {
            Self::OpTranAc => "OP + TRAN + AC",
            Self::AllCompatibleAnalyses => "All compatible analyses",
            Self::SelectedAnalysis { .. } => "Selected analysis only",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedOutputPolicy {
    EveryAcceptedPoint,
    SelectedAndFinalPoints,
    OnDemandFromRetainedState,
    FailureDiagnosticsOnly,
}

impl SavedOutputPolicy {
    pub const ALL: [Self; 4] = [
        Self::EveryAcceptedPoint,
        Self::SelectedAndFinalPoints,
        Self::OnDemandFromRetainedState,
        Self::FailureDiagnosticsOnly,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::EveryAcceptedPoint => "Every accepted point",
            Self::SelectedAndFinalPoints => "Selected + final points",
            Self::OnDemandFromRetainedState => "On demand from retained state",
            Self::FailureDiagnosticsOnly => "Failure diagnostics only",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedOutputPrecision {
    FullSourcePrecision,
    DisplayCacheWithFullSourcePrecision,
}

impl SavedOutputPrecision {
    pub const ALL: [Self; 2] = [
        Self::FullSourcePrecision,
        Self::DisplayCacheWithFullSourcePrecision,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::FullSourcePrecision => "f64 / complex128",
            Self::DisplayCacheWithFullSourcePrecision => {
                "f32 display cache + full source precision"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedOutputStreaming {
    LivePlotAdaptiveDisplayDecimation,
    StoreOnly,
}

impl SavedOutputStreaming {
    pub const ALL: [Self; 2] = [Self::LivePlotAdaptiveDisplayDecimation, Self::StoreOnly];

    pub const fn label(self) -> &'static str {
        match self {
            Self::LivePlotAdaptiveDisplayDecimation => "Live plot · adaptive display decimation",
            Self::StoreOnly => "Store only",
        }
    }
}

/// Persisted waveform/data contract owned by the simulation plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SavedOutput {
    pub id: SavedOutputId,
    pub revision: ObjectRevision,
    pub kind: SavedOutputKind,
    pub name: String,
    pub source_expression: String,
    pub compatible_analyses: SavedOutputCompatibility,
    pub save_policy: SavedOutputPolicy,
    pub stored_precision: SavedOutputPrecision,
    pub streaming: SavedOutputStreaming,
}

impl SavedOutput {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        kind: SavedOutputKind,
        name: impl Into<String>,
        source_expression: impl Into<String>,
        compatible_analyses: SavedOutputCompatibility,
        save_policy: SavedOutputPolicy,
        stored_precision: SavedOutputPrecision,
        streaming: SavedOutputStreaming,
    ) -> Result<Self, String> {
        let output = Self {
            id: SavedOutputId::new(),
            revision: ObjectRevision::INITIAL,
            kind,
            name: name.into(),
            source_expression: source_expression.into(),
            compatible_analyses,
            save_policy,
            stored_precision,
            streaming,
        };
        output.validate()?;
        Ok(output)
    }

    pub fn validate(&self) -> Result<(), String> {
        validate_bounded_text("name", &self.name, 256, false)?;
        validate_bounded_text(
            "source or expression",
            &self.source_expression,
            8_192,
            false,
        )?;
        validate_saved_output_expression(self.kind, &self.source_expression)
    }

    /// Unit preview derived from the output schema without inspecting a
    /// mutable result dataset. Derived expressions remain explicit until the
    /// calculator's dimensional resolver evaluates their dependencies.
    pub fn inferred_unit(&self) -> &'static str {
        match self.kind {
            SavedOutputKind::RawVoltageOrCurrent
                if self
                    .source_expression
                    .trim()
                    .get(..2)
                    .is_some_and(|prefix| prefix.eq_ignore_ascii_case("V(")) =>
            {
                "volts"
            }
            SavedOutputKind::RawVoltageOrCurrent => "amperes",
            SavedOutputKind::DerivedExpression => "resolved from expression",
            SavedOutputKind::DeviceOperatingPointQuantity => "from device quantity",
            SavedOutputKind::NoiseContributor => "V²/Hz or A²/Hz",
            SavedOutputKind::RfPortQuantity => "dimensionless",
        }
    }

    pub const fn status_label(&self) -> &'static str {
        match self.save_policy {
            SavedOutputPolicy::EveryAcceptedPoint => "full capture",
            SavedOutputPolicy::SelectedAndFinalPoints => "selected + final",
            SavedOutputPolicy::OnDemandFromRetainedState => "retained-state derivation",
            SavedOutputPolicy::FailureDiagnosticsOnly => "failure diagnostics",
        }
    }

    pub(super) fn cloned_for_new_plan(
        &self,
        analysis_identity_map: &HashMap<AnalysisInstanceId, AnalysisInstanceId>,
    ) -> Result<Self, AnalysisInstanceId> {
        let mut cloned = self.clone();
        cloned.id = SavedOutputId::new();
        cloned.revision = ObjectRevision::INITIAL;
        if let SavedOutputCompatibility::SelectedAnalysis { analysis_id } =
            &mut cloned.compatible_analyses
        {
            *analysis_id = analysis_identity_map
                .get(analysis_id)
                .copied()
                .ok_or(*analysis_id)?;
        }
        Ok(cloned)
    }
}

const LEGACY_SAVED_OUTPUT_ID_NAMESPACE: Uuid =
    Uuid::from_u128(0x75b5_6a7e_614a_5b71_b2dc_32cb_7624_1038);

impl<'de> Deserialize<'de> for SavedOutput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Wire {
            #[serde(default = "missing_identity_sentinel")]
            id: serde_json::Value,
            #[serde(default)]
            revision: ObjectRevision,
            kind: SavedOutputKind,
            name: String,
            source_expression: String,
            compatible_analyses: SavedOutputCompatibility,
            save_policy: SavedOutputPolicy,
            stored_precision: SavedOutputPrecision,
            streaming: SavedOutputStreaming,
        }

        let wire = Wire::deserialize(deserializer)?;
        let identity = serde_json::to_vec(&(
            wire.kind,
            &wire.name,
            &wire.source_expression,
            &wire.compatible_analyses,
            wire.save_policy,
            wire.stored_precision,
            wire.streaming,
        ))
        .map_err(D::Error::custom)?;
        let id = deserialize_or_migrate_identity::<SavedOutputId, D::Error>(
            wire.id,
            LEGACY_SAVED_OUTPUT_ID_NAMESPACE,
            &identity,
            SavedOutputId::from_namespace,
        )?;
        Ok(Self {
            id,
            revision: wire.revision,
            kind: wire.kind,
            name: wire.name,
            source_expression: wire.source_expression,
            compatible_analyses: wire.compatible_analyses,
            save_policy: wire.save_policy,
            stored_precision: wire.stored_precision,
            streaming: wire.streaming,
        })
    }
}

const MISSING_IDENTITY_SENTINEL: &str = "__rspice_missing_stable_identity__";

pub(super) fn missing_identity_sentinel() -> serde_json::Value {
    serde_json::Value::String(MISSING_IDENTITY_SENTINEL.to_owned())
}

pub(super) fn deserialize_or_migrate_identity<I, E>(
    value: serde_json::Value,
    namespace: Uuid,
    identity: &[u8],
    migrate: fn(Uuid, &[u8]) -> I,
) -> Result<I, E>
where
    I: serde::de::DeserializeOwned,
    E: serde::de::Error,
{
    if value == missing_identity_sentinel() {
        Ok(migrate(namespace, identity))
    } else if value.is_null() {
        Err(E::custom("stable identity must not be null"))
    } else {
        serde_json::from_value(value).map_err(E::custom)
    }
}

pub(super) fn parse_design_quantity(
    expression: &str,
    quantity: DesignVariableQuantity,
) -> Result<f64, String> {
    use crate::quantity::{
        QuantityInputKind, QuantityPresentationPolicy, UiNumberLocale, parse_ui_quantity,
    };

    let text = expression.trim();
    let (numeric, kind) = match quantity {
        DesignVariableQuantity::Resistance => (
            strip_required_unit(text, &["ohm", "Ω"])
                .ok_or_else(|| "explicit resistance unit required (ohm or Ω)".to_owned())?,
            QuantityInputKind::EngineeringScalar,
        ),
        DesignVariableQuantity::Capacitance => (
            text.strip_suffix('F')
                .ok_or_else(|| "explicit capacitance unit required (F)".to_owned())?,
            QuantityInputKind::EngineeringScalar,
        ),
        DesignVariableQuantity::Voltage => (
            text.strip_suffix('V')
                .ok_or_else(|| "explicit voltage unit required (V)".to_owned())?,
            QuantityInputKind::EngineeringScalar,
        ),
        DesignVariableQuantity::Current => (
            text.strip_suffix('A')
                .ok_or_else(|| "explicit current unit required (A)".to_owned())?,
            QuantityInputKind::EngineeringScalar,
        ),
        DesignVariableQuantity::Temperature => (text, QuantityInputKind::Temperature),
        DesignVariableQuantity::Dimensionless => (text, QuantityInputKind::EngineeringScalar),
    };
    let numeric = if quantity == DesignVariableQuantity::Dimensionless
        || quantity == DesignVariableQuantity::Temperature
    {
        numeric.trim().to_owned()
    } else {
        normalize_explicit_unit_scalar(numeric.trim())
    };
    parse_ui_quantity(
        &numeric,
        kind,
        QuantityPresentationPolicy::default(),
        UiNumberLocale::default(),
    )
    .map_err(|error| error.to_string())
}

fn normalize_explicit_unit_scalar(value: &str) -> String {
    if let Some(prefix) = value.strip_suffix('M') {
        format!("{}Meg", prefix.trim_end())
    } else {
        value.to_owned()
    }
}

fn strip_required_unit<'a>(value: &'a str, units: &[&str]) -> Option<&'a str> {
    units.iter().find_map(|unit| {
        if unit.is_ascii() {
            value
                .get(value.len().saturating_sub(unit.len())..)
                .filter(|suffix| suffix.eq_ignore_ascii_case(unit))
                .map(|_| &value[..value.len() - unit.len()])
        } else {
            value.strip_suffix(unit)
        }
    })
}

fn validate_saved_output_expression(kind: SavedOutputKind, expression: &str) -> Result<(), String> {
    let expression = expression.trim();
    match kind {
        SavedOutputKind::RawVoltageOrCurrent => validate_raw_probe(expression),
        SavedOutputKind::DerivedExpression => parse_calculator_expression(expression),
        SavedOutputKind::DeviceOperatingPointQuantity => validate_device_op_probe(expression),
        SavedOutputKind::NoiseContributor => {
            if validate_hierarchical_token(expression).is_ok() {
                Ok(())
            } else {
                parse_calculator_expression(expression)
            }
        }
        SavedOutputKind::RfPortQuantity => {
            if !expression
                .get(..2)
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case("S("))
            {
                return Err("RF port quantity must use S(port, port) syntax".to_owned());
            }
            parse_calculator_expression(expression)
        }
    }
}

fn parse_calculator_expression(expression: &str) -> Result<(), String> {
    crate::analysis::calculator::parser::Parser::new(expression)
        .try_parse()
        .map(|_| ())
        .map_err(|error| format!("expression is invalid: {error}"))
}

pub(crate) fn validate_raw_probe(expression: &str) -> Result<(), String> {
    let open = expression
        .find('(')
        .ok_or_else(|| "raw output must use V(node), V(node+, node-), or I(source)".to_owned())?;
    if !expression.ends_with(')') {
        return Err("raw output has an unterminated probe".to_owned());
    }
    let function = &expression[..open];
    let arguments = &expression[open + 1..expression.len() - 1];
    let arguments = arguments.split(',').map(str::trim).collect::<Vec<_>>();
    if function.eq_ignore_ascii_case("V") && matches!(arguments.len(), 1 | 2)
        || function.eq_ignore_ascii_case("I") && arguments.len() == 1
    {
        for argument in arguments {
            validate_hierarchical_token(argument).map_err(|error| {
                format!("raw output must use V(node), V(node+, node-), or I(source): {error}")
            })?;
        }
        Ok(())
    } else {
        Err("raw output must use V(node), V(node+, node-), or I(source)".to_owned())
    }
}

fn validate_device_op_probe(expression: &str) -> Result<(), String> {
    let Some(body) = expression.strip_prefix('@') else {
        return Err("device operating-point quantity must use @device[quantity] syntax".to_owned());
    };
    let Some(open) = body.find('[') else {
        return Err("device operating-point quantity must use @device[quantity] syntax".to_owned());
    };
    if !body.ends_with(']') {
        return Err("device operating-point quantity has an unterminated quantity".to_owned());
    }
    validate_hierarchical_token(&body[..open])?;
    validate_parameter_name(&body[open + 1..body.len() - 1])
        .map_err(|error| format!("device quantity is invalid: {error}"))
}

fn validate_hierarchical_token(value: &str) -> Result<(), String> {
    if value.is_empty() {
        return Err("probe target is required".to_owned());
    }
    if let Some(character) = value.chars().find(|character| {
        !character.is_ascii_alphanumeric()
            && !matches!(character, '_' | '.' | ':' | '/' | '$' | '+' | '-')
    }) {
        return Err(format!(
            "probe target contains unsupported character {character:?}"
        ));
    }
    Ok(())
}

pub(super) fn validate_parameter_name(value: &str) -> Result<(), String> {
    if value.is_empty() {
        return Err("name is required".to_owned());
    }
    if value.len() > 128 {
        return Err("name exceeds 128 bytes".to_owned());
    }
    let mut characters = value.chars();
    let first = characters.next().expect("empty handled above");
    if !first.is_ascii_alphabetic() && first != '_' {
        return Err("name must begin with an ASCII letter or underscore".to_owned());
    }
    if let Some(character) =
        characters.find(|character| !character.is_ascii_alphanumeric() && *character != '_')
    {
        return Err(format!(
            "name contains unsupported character {character:?}; use ASCII letters, digits, and underscores"
        ));
    }
    Ok(())
}

pub(super) fn validate_single_line_expression(label: &str, value: &str) -> Result<(), String> {
    validate_bounded_text(label, value, 8_192, false)?;
    if value
        .chars()
        .any(|character| matches!(character, '\r' | '\n'))
    {
        return Err(format!("{label} must be a single line"));
    }
    Ok(())
}

pub(super) fn validate_bounded_text(
    label: &str,
    value: &str,
    maximum_bytes: usize,
    allow_empty: bool,
) -> Result<(), String> {
    if !allow_empty && value.trim().is_empty() {
        return Err(format!("{label} is required"));
    }
    if value.len() > maximum_bytes {
        return Err(format!("{label} exceeds {maximum_bytes} bytes"));
    }
    if let Some(character) = value
        .chars()
        .find(|character| character.is_control() && !matches!(character, '\n' | '\r' | '\t'))
    {
        return Err(format!("{label} contains control character {character:?}"));
    }
    Ok(())
}

