//! Sensitivity analysis configuration.

use super::AcSweepType;

//=============================================================================

/// What a plan saved before the filter existed computed.
///
/// The Studio used to loop the engine's single-parameter primitive over every
/// finite `.PARAM` of the deck, which is exactly the set `PARAM:*` names. A
/// restored plan therefore states that in the engine's own words rather than
/// silently becoming a different run under the same digest.
pub const DESIGN_PARAMETERS_FILTER: &str = "PARAM:*";

/// The serde default for every persisted and transported `filter` field.
///
/// An absent filter is a plan or a draft written before the filter existed,
/// and what it computed is the design parameters. A new plan writes its
/// filter — empty or not — so absence never means "empty".
pub fn design_parameters_filter() -> String {
    DESIGN_PARAMETERS_FILTER.to_owned()
}

/// The one canonical spelling of a filter list.
///
/// The card's own grammar is the rule: a devspec is a run of adjacent tokens,
/// uppercased, separated by whitespace. Commas are accepted from a typist
/// because every other list field in the Studio accepts them, and the first
/// occurrence of a repeated token wins so the order a reader typed survives
/// — the same rule the Monte Carlo parameter list already uses.
pub fn canonical_sensitivity_filter(text: &str) -> String {
    let mut seen: Vec<String> = Vec::new();
    for token in text.split([',', ' ', '\t', '\n', '\r']) {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }
        let token = token.to_ascii_uppercase();
        if !seen.contains(&token) {
            seen.push(token);
        }
    }
    seen.join(" ")
}

/// Refuse a filter the `.SENS` card would read as something else.
///
/// The filter is written into the card verbatim, so a token the card's own
/// grammar claims — its `AC` and `DC` mode keywords — would silently change
/// the analysis rather than select a variable. `PARAM:` with nothing after it
/// selects no design parameter and names none, and a character the card has
/// no place for cannot survive the round trip through the deck.
pub fn validate_sensitivity_filter(filter: &str) -> Result<(), String> {
    for token in filter.split_whitespace() {
        if token.eq_ignore_ascii_case("AC") || token.eq_ignore_ascii_case("DC") {
            return Err(format!(
                "Filter item '{token}' would be read as the .SENS card's own mode keyword"
            ));
        }
        if token.eq_ignore_ascii_case("PARAM:") {
            return Err(
                "Filter item 'PARAM:' names no design parameter; write PARAM:* or PARAM:<name>"
                    .to_owned(),
            );
        }
        if let Some(bad) = token.chars().find(|character| {
            !matches!(character,
                'A'..='Z' | 'a'..='z' | '0'..='9' | '_' | '.' | ':' | '*' | '?' | '[' | ']')
        }) {
            return Err(format!(
                "Filter item '{token}' contains '{bad}', which a .SENS card cannot carry"
            ));
        }
    }
    Ok(())
}

/// An AC sensitivity sweep of more than one frequency.
///
/// The start frequency is [`SensitivityConfig::frequency`], because one
/// frequency and a sweep starting there are the same card with a different
/// count and nothing should have two spellings for the band's lower edge.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensitivitySweep {
    pub stop_frequency: f64,
    pub points: u32,
    pub variation: AcSweepType,
}

impl SensitivitySweep {
    /// The same band in the specification's own sweep vocabulary.
    ///
    /// One conversion, named once, because a sweep that meant decades on the
    /// way out and octaves on the way back would solve a different band than
    /// the plan states.
    pub fn to_spec(self) -> super::SensitivitySweepSpec {
        use super::FrequencySweep;
        super::SensitivitySweepSpec {
            stop_frequency: self.stop_frequency,
            points: self.points,
            variation: match self.variation {
                AcSweepType::Decade => FrequencySweep::Decade,
                AcSweepType::Octave => FrequencySweep::Octave,
                AcSweepType::Linear => FrequencySweep::Linear,
            },
        }
    }

    /// The inverse of [`SensitivitySweep::to_spec`].
    pub fn from_spec(spec: super::SensitivitySweepSpec) -> Self {
        use super::FrequencySweep;
        Self {
            stop_frequency: spec.stop_frequency,
            points: spec.points,
            variation: match spec.variation {
                FrequencySweep::Decade => AcSweepType::Decade,
                FrequencySweep::Octave => AcSweepType::Octave,
                FrequencySweep::Linear => AcSweepType::Linear,
            },
        }
    }
}

/// Sensitivity analysis configuration
#[derive(Debug, Clone)]
pub struct SensitivityConfig {
    /// Output variable (e.g., "V(out)", "I(R1)")
    pub output_var: String,
    /// AC analysis (if true, does AC sensitivity)
    pub ac_mode: bool,
    /// Start frequency for AC sensitivity
    pub frequency: Option<f64>,
    /// The variables to differentiate against, as the `.SENS` card spells
    /// them: whitespace-separated globs, canonical and upper-case. Empty is
    /// the engine's default — every device and model parameter, and no design
    /// parameter — and it is a statement, not a missing value.
    pub filter: String,
    /// The rest of the AC band, when the run asked for more than one point.
    pub sweep: Option<SensitivitySweep>,
}

impl Default for SensitivityConfig {
    fn default() -> Self {
        Self {
            output_var: "V(out)".to_string(),
            ac_mode: false,
            frequency: None,
            filter: String::new(),
            sweep: None,
        }
    }
}

impl SensitivityConfig {
    /// Preserve historical positive spot cards; DC-limit spots use a linear grid.
    pub fn ac_sweep_config(&self) -> Option<super::AcAnalysisConfig> {
        if !self.ac_mode {
            return None;
        }
        let start_freq = self.frequency?;
        let sweep = self.sweep.unwrap_or(SensitivitySweep {
            stop_frequency: start_freq,
            points: 1,
            variation: if start_freq == 0.0 {
                AcSweepType::Linear
            } else {
                AcSweepType::Decade
            },
        });
        Some(super::AcAnalysisConfig {
            sweep_type: sweep.variation,
            num_points: sweep.points as usize,
            start_freq,
            stop_freq: sweep.stop_frequency,
        })
    }

    /// Generate SPICE .sens command
    ///
    /// The card the Studio writes is the card `rspice run` reads: one output,
    /// the filter list if there is one, and the AC sweep if the run has a
    /// frequency. Positive spot frequencies keep the historical `AC DEC 1 f f`
    /// spelling; the zero-frequency limit is written as `AC LIN 1 0 0`.
    pub fn to_spice(&self) -> String {
        let mut card = format!(".sens {}", self.output_var);
        if !self.filter.trim().is_empty() {
            card.push(' ');
            card.push_str(self.filter.trim());
        }
        if let Some(sweep) = self.ac_sweep_config() {
            card.push_str(&format!(
                " AC {} {} {} {}",
                sweep.sweep_type.spice_name().to_uppercase(),
                sweep.num_points,
                sweep.start_freq,
                sweep.stop_freq,
            ));
        }
        card
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.output_var.trim().is_empty() {
            errors.push("Output variable is required".to_string());
        }
        if let Err(error) = validate_sensitivity_filter(&self.filter) {
            errors.push(error);
        }
        if self.ac_mode {
            match self.frequency {
                Some(freq) if !freq.is_finite() || freq < 0.0 => errors.push(
                    "Frequency must be finite and nonnegative for AC sensitivity".to_string(),
                ),
                Some(0.0)
                    if self
                        .sweep
                        .is_some_and(|sweep| sweep.variation != AcSweepType::Linear) =>
                {
                    errors.push(
                        "DEC/OCT sensitivity requires a positive start frequency".to_string(),
                    );
                }
                // An AC study with no frequency has no grid to solve on. It
                // used to run silently at 1 Hz, which answered a question
                // nobody asked.
                None => errors.push("AC sensitivity requires a start frequency".to_string()),
                Some(_) => {}
            }
            if let Some(sweep) = self.sweep {
                if !sweep.stop_frequency.is_finite() || sweep.stop_frequency < 0.0 {
                    errors.push(
                        "Stop frequency must be finite and nonnegative for an AC sensitivity sweep"
                            .to_string(),
                    );
                }
                if sweep.points == 0 {
                    errors.push("An AC sensitivity sweep needs at least one point".to_string());
                }
            }
        } else if self.frequency.is_some() {
            errors.push("A sensitivity frequency is only valid in AC mode".to_string());
        } else if self.sweep.is_some() {
            errors.push("A sensitivity sweep is only valid in AC mode".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AcSweepType, SensitivityConfig, SensitivitySweep};

    #[test]
    fn ac_sensitivity_config_rejects_non_finite_frequency() {
        let config = SensitivityConfig {
            output_var: "V(out)".to_string(),
            ac_mode: true,
            frequency: Some(f64::NAN),
            ..SensitivityConfig::default()
        };

        let errors = config
            .validate()
            .expect_err("NaN AC sensitivity frequency must be rejected");
        assert!(
            errors
                .iter()
                .any(|message| message.contains("Frequency must be finite and nonnegative"))
        );
    }

    /// The card a single-frequency run writes has not moved: a project that
    /// re-emits its deck must produce the same bytes it always did.
    #[test]
    fn a_single_frequency_card_is_written_byte_for_byte_as_before() {
        let config = SensitivityConfig {
            output_var: "V(out)".to_string(),
            ac_mode: true,
            frequency: Some(1e6),
            ..SensitivityConfig::default()
        };
        assert_eq!(config.to_spice(), ".sens V(out) AC DEC 1 1000000 1000000");

        let dc = SensitivityConfig {
            output_var: "I(V1)".to_string(),
            ..SensitivityConfig::default()
        };
        assert_eq!(dc.to_spice(), ".sens I(V1)");
    }

    #[test]
    fn a_filter_and_a_sweep_reach_the_card_in_the_order_it_reads_them() {
        let config = SensitivityConfig {
            output_var: "V(out)".to_string(),
            ac_mode: true,
            frequency: Some(10.0),
            filter: "R* PARAM:*".to_string(),
            sweep: Some(SensitivitySweep {
                stop_frequency: 1e6,
                points: 10,
                variation: AcSweepType::Decade,
            }),
        };
        assert_eq!(
            config.to_spice(),
            ".sens V(out) R* PARAM:* AC DEC 10 10 1000000"
        );
        config.validate().expect("the swept card is valid");

        let dc_with_filter = SensitivityConfig {
            output_var: "V(out)".to_string(),
            filter: "PARAM:*".to_string(),
            ..SensitivityConfig::default()
        };
        assert_eq!(dc_with_filter.to_spice(), ".sens V(out) PARAM:*");
    }

    /// An AC study with no frequency is refused rather than solved at 1 Hz.
    #[test]
    fn an_ac_study_without_a_frequency_is_refused() {
        let config = SensitivityConfig {
            output_var: "V(out)".to_string(),
            ac_mode: true,
            ..SensitivityConfig::default()
        };
        assert!(
            config
                .validate()
                .expect_err("an AC study needs a frequency")
                .iter()
                .any(|message| message.contains("requires a start frequency"))
        );
        // And a sweep without AC is a sweep of nothing.
        let sweep_without_ac = SensitivityConfig {
            output_var: "V(out)".to_string(),
            sweep: Some(SensitivitySweep {
                stop_frequency: 1e6,
                points: 10,
                variation: AcSweepType::Decade,
            }),
            ..SensitivityConfig::default()
        };
        assert!(sweep_without_ac.validate().is_err());
    }
}
