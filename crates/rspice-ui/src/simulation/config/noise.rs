//! Exact noise-analysis configuration shared by plan drafts and execution.

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

use super::AcSweepType;

/// Sweep mode exposed by the Simulation Studio noise form.
///
/// `Unsupported` exists only to retain an invalid legacy numeric index long
/// enough for validation to reject it. It is never offered by the UI and is
/// never executable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoiseSweepType {
    #[default]
    Decade,
    Octave,
    Linear,
    ExplicitFrequencyList,
    Unsupported(u64),
}

impl NoiseSweepType {
    pub const OPTIONS: [&'static str; 4] =
        ["Decade", "Octave", "Linear", "Explicit frequency list"];

    #[must_use]
    pub const fn selection_index(self) -> Option<usize> {
        match self {
            Self::Decade => Some(0),
            Self::Octave => Some(1),
            Self::Linear => Some(2),
            Self::ExplicitFrequencyList => Some(3),
            Self::Unsupported(_) => None,
        }
    }

    #[must_use]
    pub const fn from_selection_index(index: usize) -> Self {
        match index {
            0 => Self::Decade,
            1 => Self::Octave,
            2 => Self::Linear,
            3 => Self::ExplicitFrequencyList,
            unsupported => Self::Unsupported(unsupported as u64),
        }
    }

    #[must_use]
    pub const fn legacy_index(self) -> Option<usize> {
        match self {
            Self::Unsupported(value) if value <= usize::MAX as u64 => Some(value as usize),
            Self::Unsupported(_) => None,
            mode => mode.selection_index(),
        }
    }

    fn schema_name(self) -> Option<&'static str> {
        match self {
            Self::Decade => Some("decade"),
            Self::Octave => Some("octave"),
            Self::Linear => Some("linear"),
            Self::ExplicitFrequencyList => Some("explicit_frequency_list"),
            Self::Unsupported(_) => None,
        }
    }
}

impl Serialize for NoiseSweepType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Self::Unsupported(value) => serializer.serialize_u64(value),
            mode => serializer.serialize_str(
                mode.schema_name()
                    .expect("all supported noise sweep modes have schema names"),
            ),
        }
    }
}

impl<'de> Deserialize<'de> for NoiseSweepType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl de::Visitor<'_> for Visitor {
            type Value = NoiseSweepType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a named noise sweep mode or legacy numeric sweep index")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(match value {
                    0 => NoiseSweepType::Decade,
                    1 => NoiseSweepType::Octave,
                    2 => NoiseSweepType::Linear,
                    3 => NoiseSweepType::ExplicitFrequencyList,
                    value => NoiseSweepType::Unsupported(value),
                })
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let value = u64::try_from(value)
                    .map_err(|_| E::custom("noise sweep index must not be negative"))?;
                self.visit_u64(value)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value.trim().to_ascii_lowercase().as_str() {
                    "decade" | "dec" => Ok(NoiseSweepType::Decade),
                    "octave" | "oct" => Ok(NoiseSweepType::Octave),
                    "linear" | "lin" => Ok(NoiseSweepType::Linear),
                    "explicit_frequency_list" | "explicit frequency list" | "explicit" => {
                        Ok(NoiseSweepType::ExplicitFrequencyList)
                    }
                    _ => Err(E::unknown_variant(
                        value,
                        &["decade", "octave", "linear", "explicit_frequency_list"],
                    )),
                }
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

/// Retained contributor detail requested by the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum NoiseContributionDetail {
    #[default]
    Top50,
    AllContributors,
    Top20,
    SummaryOnly,
}

impl NoiseContributionDetail {
    pub const OPTIONS: [&'static str; 4] = ["Top 50", "All contributors", "Top 20", "Summary only"];

    #[must_use]
    pub const fn selection_index(self) -> usize {
        match self {
            Self::Top50 => 0,
            Self::AllContributors => 1,
            Self::Top20 => 2,
            Self::SummaryOnly => 3,
        }
    }

    #[must_use]
    pub const fn from_selection_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Top50),
            1 => Some(Self::AllContributors),
            2 => Some(Self::Top20),
            3 => Some(Self::SummaryOnly),
            _ => None,
        }
    }
}

/// Integrated-noise values retained alongside the spectral result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum NoiseIntegrationMode {
    #[default]
    Enabled,
    OutputNoiseOnly,
    Disabled,
}

impl NoiseIntegrationMode {
    pub const OPTIONS: [&'static str; 3] = ["Enabled", "Output noise only", "Disabled"];

    #[must_use]
    pub const fn selection_index(self) -> usize {
        match self {
            Self::Enabled => 0,
            Self::OutputNoiseOnly => 1,
            Self::Disabled => 2,
        }
    }

    #[must_use]
    pub const fn from_selection_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Enabled),
            1 => Some(Self::OutputNoiseOnly),
            2 => Some(Self::Disabled),
            _ => None,
        }
    }
}

/// Noise analysis configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct NoiseAnalysisConfig {
    /// Output node (voltage probe point).
    pub output_node: String,
    /// Reference node (usually ground).
    pub reference_node: String,
    /// Independent input source used for input-referred noise.
    pub input_source: String,
    /// Fixed-grid sweep type. Ignored when `explicit_frequencies` is present.
    pub sweep_type: AcSweepType,
    /// Number of points per decade/octave, or total points for linear.
    pub num_points: usize,
    /// Start frequency (Hz) for fixed-grid sweeps.
    pub start_freq: f64,
    /// Stop frequency (Hz) for fixed-grid sweeps.
    pub stop_freq: f64,
    /// Exact frequency axis for an explicit-list sweep.
    pub explicit_frequencies: Option<Vec<f64>>,
    /// Imported manual-deck `.DATA` table. When present, row-local parameter
    /// and temperature overrides are resolved by the core data-sweep runner;
    /// `explicit_frequencies` retains the exact authored axis for validation
    /// and immutable execution identity.
    pub data_table_name: Option<String>,
    /// Requested contributor retention policy.
    pub contribution_detail: NoiseContributionDetail,
    /// Requested integrated-noise retention policy.
    pub integration_mode: NoiseIntegrationMode,
    /// Resolved simulation temperature in kelvin.
    pub temperature_kelvin: f64,
}

impl Default for NoiseAnalysisConfig {
    fn default() -> Self {
        Self {
            // Both name the user's own circuit, so the default names neither
            // and `validate` reports the two that are still required.
            output_node: String::new(),
            reference_node: "0".to_owned(),
            input_source: String::new(),
            sweep_type: AcSweepType::Decade,
            num_points: 30,
            start_freq: 10.0,
            stop_freq: 1.0e6,
            explicit_frequencies: None,
            data_table_name: None,
            contribution_detail: NoiseContributionDetail::Top50,
            integration_mode: NoiseIntegrationMode::Enabled,
            temperature_kelvin: rspice_core::constants::TEMP_REFERENCE,
        }
    }
}

impl NoiseAnalysisConfig {
    /// Generate an executable SPICE deck fragment for this exact frequency axis.
    #[must_use]
    pub fn to_spice(&self) -> String {
        let output = if self.reference_node.trim().is_empty() || self.reference_node.trim() == "0" {
            format!("V({})", self.output_node.trim())
        } else {
            format!(
                "V({},{})",
                self.output_node.trim(),
                self.reference_node.trim()
            )
        };
        if let Some(table_name) = self.data_table_name.as_deref() {
            return format!(
                ".noise {output} {} DATA={}",
                self.input_source.trim(),
                table_name.trim()
            );
        }
        if let Some(frequencies) = &self.explicit_frequencies {
            let mut deck = format!(
                ".noise {output} {} DATA=rspice_noise_frequency\n.DATA rspice_noise_frequency\n+ HERTZ",
                self.input_source.trim()
            );
            for frequency in frequencies {
                deck.push_str(&format!("\n+ {frequency:.17e}"));
            }
            deck.push_str("\n.ENDDATA");
            return deck;
        }
        format!(
            ".noise {output} {} {} {} {} {}",
            self.input_source.trim(),
            self.sweep_type.spice_name(),
            self.num_points,
            self.start_freq,
            self.stop_freq
        )
    }

    /// Validate the complete, executable noise configuration.
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.output_node.trim().is_empty() {
            errors.push("Output node is required".to_owned());
        } else if !is_single_spice_node_token(&self.output_node) {
            errors.push("Output node must be one valid SPICE node token".to_owned());
        }
        if !self.reference_node.trim().is_empty()
            && !is_single_spice_node_token(&self.reference_node)
        {
            errors.push("Reference node must be one valid SPICE node token".to_owned());
        }
        if self.input_source.trim().is_empty() {
            errors.push("Input source is required".to_owned());
        } else if !is_single_spice_identifier_token(&self.input_source) {
            errors.push("Input source must be one valid SPICE identifier".to_owned());
        }
        if !self.temperature_kelvin.is_finite() || self.temperature_kelvin <= 0.0 {
            errors.push("Temperature must be finite and greater than zero kelvin".to_owned());
        }
        if let Some(table_name) = self.data_table_name.as_deref() {
            if table_name.trim().is_empty() {
                errors.push("Noise DATA table name must not be empty".to_owned());
            } else if !is_single_spice_identifier_token(table_name) {
                errors.push("Noise DATA table name must be one valid SPICE identifier".to_owned());
            }
        }

        if let Some(frequencies) = &self.explicit_frequencies {
            if frequencies.is_empty() {
                errors.push("Explicit frequency list must contain at least one value".to_owned());
            }
            let mut previous = None;
            for (index, frequency) in frequencies.iter().copied().enumerate() {
                if !frequency.is_finite() || frequency <= 0.0 {
                    errors.push(format!(
                        "Explicit frequency {} must be finite and greater than zero",
                        index + 1
                    ));
                    break;
                }
                if self.data_table_name.is_none()
                    && previous.is_some_and(|previous| frequency <= previous)
                {
                    errors.push(
                        "Explicit frequencies must be strictly increasing without duplicates"
                            .to_owned(),
                    );
                    break;
                }
                previous = Some(frequency);
            }
        } else {
            if !self.start_freq.is_finite() || self.start_freq <= 0.0 {
                errors.push("Start frequency must be finite and positive".to_owned());
            }
            if !self.stop_freq.is_finite() || self.stop_freq <= 0.0 {
                errors.push("Stop frequency must be finite and positive".to_owned());
            }
            if self.start_freq.is_finite()
                && self.stop_freq.is_finite()
                && self.start_freq >= self.stop_freq
            {
                errors.push("Start frequency must be less than stop frequency".to_owned());
            }
            if self.num_points == 0 {
                errors.push("Number of points must be positive".to_owned());
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Generate the exact frequency points represented by this configuration.
    #[must_use]
    pub fn generate_frequencies(&self) -> Vec<f64> {
        if let Some(frequencies) = &self.explicit_frequencies {
            return frequencies.clone();
        }
        rspice_core::analysis::ac::ac_sweep_frequencies(
            self.sweep_type.freq_variation(),
            self.num_points,
            self.start_freq,
            self.stop_freq,
        )
    }

    /// Resolved temperature used by the native noise engine.
    #[must_use]
    pub const fn default_temperature(&self) -> f64 {
        self.temperature_kelvin
    }
}

fn is_single_spice_identifier_token(value: &str) -> bool {
    use rspice_core::netlist::lexer::TokenKind;

    let value = value.trim();
    if value.is_empty()
        || value
            .chars()
            .any(|character| character.is_whitespace() || character.is_control())
    {
        return false;
    }
    let Ok(tokens) = rspice_core::netlist::lexer::tokenize(value) else {
        return false;
    };
    tokens.len() == 2
        && matches!(tokens[0].kind, TokenKind::Ident(_))
        && tokens[0].span.start == 0
        && tokens[0].span.end == value.len()
        && matches!(tokens[1].kind, TokenKind::Eof)
}

fn is_single_spice_node_token(value: &str) -> bool {
    use rspice_core::netlist::lexer::TokenKind;

    let value = value.trim();
    if value.is_empty()
        || value
            .chars()
            .any(|character| character.is_whitespace() || character.is_control())
    {
        return false;
    }
    let Ok(tokens) = rspice_core::netlist::lexer::tokenize(value) else {
        return false;
    };
    let Some((eof, pieces)) = tokens.split_last() else {
        return false;
    };
    if !matches!(eof.kind, TokenKind::Eof) || pieces.is_empty() {
        return false;
    }

    let mut expected_start = 0;
    for token in pieces {
        if token.span.start != expected_start
            || !matches!(
                token.kind,
                TokenKind::Ident(_)
                    | TokenKind::Number(_)
                    | TokenKind::Plus
                    | TokenKind::Minus
                    | TokenKind::Slash
                    | TokenKind::AtSign
                    | TokenKind::Tilde
                    | TokenKind::LBracket
                    | TokenKind::RBracket
                    | TokenKind::Other(':')
                    | TokenKind::Other('`')
                    | TokenKind::Other('!')
                    | TokenKind::Other('$')
                    | TokenKind::Other('^')
                    | TokenKind::Other('&')
                    | TokenKind::Other('|')
                    | TokenKind::Other('\\')
                    | TokenKind::Other('<')
                    | TokenKind::Other('>')
                    | TokenKind::Other('?')
            )
        {
            return false;
        }
        expected_start = token.span.end;
    }
    expected_start == value.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweep_schema_accepts_legacy_indices_and_writes_named_modes() {
        for (legacy, expected) in [
            (0, NoiseSweepType::Decade),
            (1, NoiseSweepType::Octave),
            (2, NoiseSweepType::Linear),
            (3, NoiseSweepType::ExplicitFrequencyList),
        ] {
            let decoded: NoiseSweepType =
                serde_json::from_str(&legacy.to_string()).expect("legacy mode decodes");
            assert_eq!(decoded, expected);
            assert!(serde_json::to_string(&decoded).unwrap().starts_with('"'));
        }
        let unsupported: NoiseSweepType = serde_json::from_str("99").unwrap();
        assert_eq!(unsupported, NoiseSweepType::Unsupported(99));
        assert!(serde_json::from_str::<NoiseSweepType>("\"invented\"").is_err());
    }

    /// The frequency axis and the retention policies are decisions this
    /// analysis can make on its own. The output node and the input source
    /// name the user's circuit, so the default names neither and `validate`
    /// asks for both.
    #[test]
    fn defaults_state_the_axis_and_ask_for_the_two_circuit_names() {
        let config = NoiseAnalysisConfig::default();
        assert_eq!(config.output_node, "");
        assert_eq!(config.reference_node, "0");
        assert_eq!(config.input_source, "");
        let errors = config.validate().expect_err("two names are still required");
        assert!(
            errors
                .iter()
                .any(|error| error == "Output node is required")
        );
        assert!(
            errors
                .iter()
                .any(|error| error == "Input source is required")
        );
        assert_eq!(config.num_points, 30);
        assert_eq!(config.start_freq, 10.0);
        assert_eq!(config.stop_freq, 1.0e6);
        assert_eq!(config.contribution_detail, NoiseContributionDetail::Top50);
        assert_eq!(config.integration_mode, NoiseIntegrationMode::Enabled);
        assert_eq!(config.temperature_kelvin, 300.15);
    }

    #[test]
    fn explicit_axis_is_validated_and_emitted_as_noise_data() {
        let named = || NoiseAnalysisConfig {
            output_node: "out".to_owned(),
            input_source: "V1".to_owned(),
            ..NoiseAnalysisConfig::default()
        };
        let config = NoiseAnalysisConfig {
            explicit_frequencies: Some(vec![10.0, 1.0e3, 1.0e6]),
            ..named()
        };
        assert!(config.validate().is_ok());
        assert_eq!(config.generate_frequencies(), vec![10.0, 1.0e3, 1.0e6]);
        let deck = config.to_spice();
        assert!(deck.contains(".noise V(out) V1 DATA=rspice_noise_frequency"));
        assert!(deck.contains(".DATA rspice_noise_frequency"));
        assert!(deck.contains("+ 1.00000000000000000e6"));
        assert!(deck.ends_with(".ENDDATA"));

        for frequencies in [vec![], vec![10.0, 10.0], vec![10.0, 1.0]] {
            let invalid = NoiseAnalysisConfig {
                explicit_frequencies: Some(frequencies),
                ..named()
            };
            assert!(invalid.validate().is_err());
        }
    }

    #[test]
    fn emitted_explicit_axis_round_trips_through_parser_and_noise_runner() {
        let config = NoiseAnalysisConfig {
            output_node: "out".to_owned(),
            input_source: "V1".to_owned(),
            explicit_frequencies: Some(vec![10.0, 1.0e3, 1.0e6]),
            ..NoiseAnalysisConfig::default()
        };
        let netlist = rspice_core::Netlist::parse(&format!(
            "exact explicit noise axis\nV1 in 0 AC 1\nR1 in out 1k\nR2 out 0 1k\n{}\n.end\n",
            config.to_spice()
        ))
        .expect("generated explicit noise fragment parses");
        assert!(netlist.analyses.iter().any(|analysis| matches!(
            analysis,
            rspice_core::netlist::AnalysisCommand::NoiseData { table_name, .. }
                if table_name.eq_ignore_ascii_case("rspice_noise_frequency")
        )));
        let (_, results) = rspice_core::Engine::default()
            .run_noise_data_named_with_input_source(
                &netlist,
                "out",
                None,
                "V1",
                "rspice_noise_frequency",
                config.temperature_kelvin,
            )
            .expect("generated explicit axis executes");
        assert_eq!(
            results
                .iter()
                .map(|result| result.frequency)
                .collect::<Vec<_>>(),
            vec![10.0, 1.0e3, 1.0e6]
        );
    }

    #[test]
    fn fixed_axis_rejects_non_finite_and_zero_density() {
        let invalid = NoiseAnalysisConfig {
            num_points: 0,
            start_freq: f64::NAN,
            temperature_kelvin: f64::INFINITY,
            ..NoiseAnalysisConfig::default()
        };
        let errors = invalid.validate().expect_err("invalid config is rejected");
        assert!(errors.iter().any(|error| error.contains("points")));
        assert!(errors.iter().any(|error| error.contains("Start frequency")));
        assert!(errors.iter().any(|error| error.contains("Temperature")));
    }

    #[test]
    fn deck_fields_require_one_exact_spice_token() {
        let valid = NoiseAnalysisConfig {
            output_node: "XAFE/out[3]".to_owned(),
            reference_node: "0".to_owned(),
            input_source: "VIN_DIFF".to_owned(),
            data_table_name: Some("noise_points".to_owned()),
            explicit_frequencies: Some(vec![10.0, 1.0]),
            ..NoiseAnalysisConfig::default()
        };
        assert!(valid.validate().is_ok());

        for invalid in [
            NoiseAnalysisConfig {
                output_node: "out extra".to_owned(),
                ..NoiseAnalysisConfig::default()
            },
            NoiseAnalysisConfig {
                reference_node: "0\n.end".to_owned(),
                ..NoiseAnalysisConfig::default()
            },
            NoiseAnalysisConfig {
                input_source: "VIN\n.op".to_owned(),
                ..NoiseAnalysisConfig::default()
            },
            NoiseAnalysisConfig {
                data_table_name: Some("points;comment".to_owned()),
                ..NoiseAnalysisConfig::default()
            },
        ] {
            assert!(invalid.validate().is_err(), "accepted {invalid:?}");
        }
    }
}
