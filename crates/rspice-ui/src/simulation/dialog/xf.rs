//! DC transfer-function (`.TF`) analysis configuration.
//!
//! A SPICE transfer-function analysis is a DC-linearized scalar analysis. It
//! reports the small-signal transfer from one independent source together with
//! the input and output resistances at the same operating point. Frequency
//! sweeps and group delay belong to AC/PXF and are deliberately not represented
//! here.

/// Result normalization applied after the exact DC-linearized solve.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum XfNormalization {
    /// Retain the signed engineering derivative returned by `.TF`.
    #[default]
    None,
    /// Convert the derivative to a dimensionless nominal-to-nominal ratio.
    RelativeToNominal,
    /// Report the response for one engineering unit of source perturbation.
    PerSourceUnit,
}

impl XfNormalization {
    pub const ALL: [Self; 3] = [Self::None, Self::RelativeToNominal, Self::PerSourceUnit];

    #[cfg(test)]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::None => "Disabled",
            Self::RelativeToNominal => "Relative to nominal",
            Self::PerSourceUnit => "Per source unit",
        }
    }
}

/// Numerical policy for the operating-point and zero-hertz linear solves.
///
/// The same tier as every other analysis offers; see
/// [`crate::simulation::accuracy`] for what a tier name resolves to.
pub type XfAccuracy = crate::simulation::accuracy::AnalysisAccuracy;

/// Exact transfer-function execution configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XfConfig {
    pub input_source: String,
    pub output_expression: String,
    pub transfer_gain: bool,
    pub input_resistance: bool,
    pub output_resistance: bool,
    pub normalization: XfNormalization,
    pub accuracy: XfAccuracy,
}

impl Default for XfConfig {
    fn default() -> Self {
        Self {
            // Both name the user's own circuit; `validate` reports each one
            // that is still missing rather than a default naming a stranger.
            input_source: String::new(),
            output_expression: String::new(),
            transfer_gain: true,
            input_resistance: true,
            output_resistance: true,
            normalization: XfNormalization::None,
            accuracy: XfAccuracy::Balanced,
        }
    }
}

impl XfConfig {
    pub fn validate(&self) -> Result<(), String> {
        validate_identifier(&self.input_source, "Input source")?;
        validate_output_expression(&self.output_expression)?;
        if !self.transfer_gain && !self.input_resistance && !self.output_resistance {
            return Err("Enable transfer gain, input resistance, or output resistance".to_owned());
        }
        Ok(())
    }

    #[cfg(test)]
    pub fn to_spice(&self) -> String {
        format!(
            ".tf {} {}",
            self.output_expression.trim(),
            self.input_source.trim()
        )
    }
}

/// Persisted editor state.
///
/// The retired frequency-domain fields remain deserialize-only so projects
/// produced before the DC `.TF` correction still open deterministically. They
/// are migrated by [`Self::prepare_after_restore`] and are never written again.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct XfDialogState {
    #[serde(default)]
    pub input_source: String,
    #[serde(default)]
    pub output_expression: String,
    #[serde(default = "default_true")]
    pub transfer_gain: bool,
    #[serde(default = "default_true")]
    pub input_resistance: bool,
    #[serde(default = "default_true")]
    pub output_resistance: bool,
    #[serde(default)]
    pub normalization_idx: usize,
    #[serde(default = "default_accuracy_idx")]
    pub accuracy_idx: usize,

    #[serde(default, skip_serializing)]
    pub start_freq: String,
    #[serde(default, skip_serializing)]
    pub stop_freq: String,
    #[serde(default, skip_serializing)]
    pub num_points: String,
    #[serde(default, skip_serializing)]
    pub sweep_type_idx: usize,
    #[serde(default, skip_serializing)]
    pub output_node: String,
    #[serde(default, skip_serializing)]
    pub output_ref: String,
    /// Accepted so `deny_unknown_fields` does not reject a pre-correction
    /// project, then dropped: group delay is a frequency-domain quantity and
    /// the DC `.TF` model it was saved beside no longer computes one.
    #[serde(default, skip_serializing, rename = "group_delay")]
    pub _group_delay: bool,
    #[serde(default, skip_serializing)]
    pub input_impedance: Option<bool>,
    #[serde(default, skip_serializing)]
    pub output_impedance: Option<bool>,

    #[serde(skip)]
    pub initialized: bool,
}

impl XfDialogState {
    pub fn from_config(config: &XfConfig) -> Self {
        Self {
            input_source: config.input_source.clone(),
            output_expression: config.output_expression.clone(),
            transfer_gain: config.transfer_gain,
            input_resistance: config.input_resistance,
            output_resistance: config.output_resistance,
            normalization_idx: XfNormalization::ALL
                .iter()
                .position(|value| *value == config.normalization)
                .unwrap_or(0),
            accuracy_idx: XfAccuracy::ALL
                .iter()
                .position(|value| *value == config.accuracy)
                .unwrap_or(default_accuracy_idx()),
            initialized: true,
            ..Self::default()
        }
    }

    pub fn to_config(&self) -> Result<XfConfig, String> {
        let normalization = XfNormalization::ALL
            .get(self.normalization_idx)
            .copied()
            .ok_or_else(|| "Transfer-function normalization selection is invalid".to_owned())?;
        let accuracy = XfAccuracy::ALL
            .get(self.accuracy_idx)
            .copied()
            .ok_or_else(|| "Transfer-function accuracy selection is invalid".to_owned())?;
        let config = XfConfig {
            input_source: self.input_source.trim().to_owned(),
            output_expression: self.output_expression.trim().to_owned(),
            transfer_gain: self.transfer_gain,
            input_resistance: self.input_resistance,
            output_resistance: self.output_resistance,
            normalization,
            accuracy,
        };
        config.validate()?;
        Ok(config)
    }

    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&XfConfig::default());
        }
    }

    /// Restore runtime state and migrate the retired AC-shaped `.XF` editor.
    pub fn prepare_after_restore(&mut self) {
        let legacy = !self.start_freq.is_empty()
            || !self.stop_freq.is_empty()
            || !self.num_points.is_empty()
            || !self.output_node.is_empty()
            || !self.output_ref.is_empty()
            || self.input_impedance.is_some()
            || self.output_impedance.is_some();
        if legacy && self.output_expression.trim().is_empty() {
            let node = self.output_node.trim();
            self.output_expression = if node.is_empty() {
                XfConfig::default().output_expression
            } else if self.output_ref.trim().is_empty() {
                format!("V({node})")
            } else {
                format!("V({node},{})", self.output_ref.trim())
            };
            self.input_resistance = self.input_impedance.unwrap_or(true);
            self.output_resistance = self.output_impedance.unwrap_or(true);
        }
        if self.input_source.trim().is_empty() {
            self.input_source = XfConfig::default().input_source;
        }
        if self.output_expression.trim().is_empty() {
            self.output_expression = XfConfig::default().output_expression;
        }
        self.normalization_idx = self
            .normalization_idx
            .min(XfNormalization::ALL.len().saturating_sub(1));
        self.accuracy_idx = self
            .accuracy_idx
            .min(XfAccuracy::ALL.len().saturating_sub(1));
        self.initialized = true;
    }
}

const fn default_true() -> bool {
    true
}

const fn default_accuracy_idx() -> usize {
    1
}

fn validate_identifier(value: &str, label: &str) -> Result<(), String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(format!("{label} is required"));
    }
    if value != trimmed || trimmed.chars().any(char::is_whitespace) {
        return Err(format!("{label} must be one independent-source name"));
    }
    Ok(())
}

fn validate_output_expression(expression: &str) -> Result<(), String> {
    let trimmed = expression.trim();
    if expression != trimmed {
        return Err("Output expression must not contain surrounding whitespace".to_owned());
    }
    let expression = trimmed;
    let Some(open) = expression.find('(') else {
        return Err("Output expression must use V(node), V(node,ref), or I(element)".to_owned());
    };
    if !expression.ends_with(')') || expression[open + 1..expression.len() - 1].contains(['(', ')'])
    {
        return Err("Output expression must use one balanced V(...) or I(...) call".to_owned());
    }
    let function = &expression[..open];
    let arguments = expression[open + 1..expression.len() - 1]
        .split(',')
        .collect::<Vec<_>>();
    let valid = if function.eq_ignore_ascii_case("V") {
        matches!(arguments.as_slice(), [node] if !node.is_empty())
            || matches!(arguments.as_slice(), [node, reference] if !node.is_empty() && !reference.is_empty())
    } else if function.eq_ignore_ascii_case("I") {
        matches!(arguments.as_slice(), [element] if !element.is_empty())
    } else {
        false
    };
    if !valid
        || arguments.iter().any(|argument| {
            *argument != argument.trim() || argument.chars().any(char::is_whitespace)
        })
    {
        return Err("Output expression must use V(node), V(node,ref), or I(element)".to_owned());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configuration_is_dc_scalar_and_validates_enabled_outputs() {
        let config = XfConfig::default();
        // The two circuit names a default cannot know are the two it leaves
        // for the user; everything else the analysis decides for itself.
        assert_eq!(config.input_source, "");
        assert_eq!(config.output_expression, "");
        assert!(config.validate().is_err());
        assert!(config.transfer_gain);
        assert!(config.input_resistance);
        assert!(config.output_resistance);
        assert_eq!(config.normalization, XfNormalization::None);
        assert_eq!(config.accuracy, XfAccuracy::Balanced);

        let named = XfConfig {
            input_source: "V1".to_owned(),
            output_expression: "V(out)".to_owned(),
            ..XfConfig::default()
        };
        assert_eq!(named.to_spice(), ".tf V(out) V1");
        assert!(named.validate().is_ok());

        assert_eq!(
            XfNormalization::ALL.map(XfNormalization::display_name),
            ["Disabled", "Relative to nominal", "Per source unit"]
        );
        assert_eq!(
            XfAccuracy::ALL.map(XfAccuracy::display_name),
            ["Fast", "Balanced", "Accurate", "Robust"]
        );

        let disabled = XfConfig {
            transfer_gain: false,
            input_resistance: false,
            output_resistance: false,
            ..named
        };
        assert!(disabled.validate().unwrap_err().contains("Enable"));
    }

    #[test]
    fn output_contract_accepts_voltage_differential_and_branch_current_only() {
        for valid in ["V(out)", "v(out,ref)", "I(Vsense)"] {
            assert!(validate_output_expression(valid).is_ok(), "{valid}");
        }
        for invalid in [
            "",
            "out",
            "V()",
            "V(out,)",
            "V(a,b,c)",
            "I(V1,V2)",
            "P(R1)",
            " V(out)",
            "V(out) ",
            "V (out)",
            "V( out)",
            "V(out, ref)",
            "V((out))",
            "V(out) extra",
        ] {
            assert!(validate_output_expression(invalid).is_err(), "{invalid}");
        }

        for invalid_source in ["", " VIN_DIFF", "VIN_DIFF ", "VIN DIFF", "VIN\tDIFF"] {
            let config = XfConfig {
                input_source: invalid_source.to_owned(),
                ..XfConfig::default()
            };
            assert!(config.validate().is_err(), "{invalid_source:?}");
        }
    }

    #[test]
    fn retired_frequency_state_migrates_without_reappearing_on_serialize() {
        let json = r#"{
            "start_freq":"1","stop_freq":"1G","num_points":"10","sweep_type_idx":0,
            "input_source":"VIN","output_node":"OUT","output_ref":"REF",
            "group_delay":true,"input_impedance":false,"output_impedance":true
        }"#;
        let mut restored: XfDialogState = serde_json::from_str(json).expect("legacy XF state");
        restored.prepare_after_restore();
        let config = restored.to_config().expect("migrated XF config");
        assert_eq!(config.output_expression, "V(OUT,REF)");
        assert!(!config.input_resistance);
        assert!(config.output_resistance);

        let encoded = serde_json::to_value(&restored).expect("serialize migrated state");
        let keys = encoded
            .as_object()
            .expect("XF state is an object")
            .keys()
            .map(String::as_str)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(
            keys,
            std::collections::BTreeSet::from([
                "accuracy_idx",
                "input_resistance",
                "input_source",
                "normalization_idx",
                "output_expression",
                "output_resistance",
                "transfer_gain",
            ])
        );
    }
}
