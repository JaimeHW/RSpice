//! Sensitivity Analysis Configuration
//!
//! Configuration for sensitivity analysis (.sens).
//! Computes the sensitivity of an output to all circuit parameters.

/// Sensitivity analysis type
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SensType {
    #[default]
    Dc,
    Ac,
}

/// Sensitivity analysis configuration
///
/// `.SENS` differentiates one output against the variables one filter list
/// selects, out of a universe that is the union of two namespaces: every
/// device and model parameter of the flattened circuit, and every root-scope
/// design parameter, which a filter reaches through the `PARAM:` prefix. An
/// empty filter is the engine's own default — every device and model
/// parameter, and no design parameter — and it is what a new analysis has, so
/// the Studio and `rspice run` read one card the same way.
#[derive(Debug, Clone)]
pub struct SensConfig {
    /// Output expression (node voltage or current)
    pub output_expr: String,
    /// Analysis type (DC or AC)
    pub sens_type: SensType,
    /// AC start frequency (only used for AC sens)
    pub ac_freq: f64,
    /// Canonical filter list, as the card spells it.
    pub filter: String,
}

impl Default for SensConfig {
    fn default() -> Self {
        Self {
            output_expr: "V(OUT)".into(),
            sens_type: SensType::Dc,
            ac_freq: 1e6,
            // A new analysis asks the engine's own default question, and
            // writes the card `rspice run` reads for it: `.sens V(OUT)`.
            filter: String::new(),
        }
    }
}

impl SensConfig {
    #[cfg(test)]
    pub fn new(output: &str) -> Self {
        Self {
            output_expr: output.to_string(),
            ..Default::default()
        }
    }

    #[cfg(test)]
    pub fn with_type(mut self, t: SensType) -> Self {
        self.sens_type = t;
        self
    }
    #[cfg(test)]
    pub fn with_ac_freq(mut self, f: f64) -> Self {
        self.ac_freq = f;
        self
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.output_expr.trim().is_empty() {
            return Err("Output expression required".into());
        }
        if self.sens_type == SensType::Ac && (!self.ac_freq.is_finite() || self.ac_freq <= 0.0) {
            return Err("AC frequency must be finite and positive".into());
        }
        crate::simulation::config::validate_sensitivity_filter(&self.filter)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct SensDialogState {
    pub output_expr: String,
    pub sens_type_idx: usize,
    pub ac_freq: String,
    /// The filter as typed. Canonicalized on its way to the configuration,
    /// never in the field, so a reader's own spacing survives editing.
    pub filter: String,
    #[serde(skip)]
    pub initialized: bool,
}

/// Persisted editor state. New fields serialize; retired fields only decode.
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct PersistedSensDialogState {
    #[serde(default)]
    output_expr: String,
    #[serde(default)]
    sens_type_idx: usize,
    #[serde(default)]
    ac_freq: String,
    /// Absent in a draft saved before the filter existed, which computed the
    /// deck's design parameters. Restored as the engine's own spelling of
    /// that set, so the row shows what the plan runs and a reader can change
    /// it. An empty filter saved deliberately is written and restores empty.
    #[serde(default = "crate::simulation::config::design_parameters_filter")]
    filter: String,
    /// Retired. These named two checkboxes over a report the engine had no
    /// way to narrow. The `filter` above is the engine's own selection and
    /// takes their place; these are accepted so earlier projects still open,
    /// and never written back.
    #[serde(default)]
    #[allow(dead_code)]
    include_params: serde::de::IgnoredAny,
    #[serde(default)]
    #[allow(dead_code)]
    include_devices: serde::de::IgnoredAny,
}

impl<'de> serde::Deserialize<'de> for SensDialogState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let persisted = PersistedSensDialogState::deserialize(deserializer)?;
        Ok(Self {
            output_expr: persisted.output_expr,
            sens_type_idx: persisted.sens_type_idx,
            ac_freq: persisted.ac_freq,
            filter: persisted.filter,
            initialized: false,
        })
    }
}

impl SensDialogState {
    pub fn from_config(config: &SensConfig) -> Self {
        Self {
            output_expr: config.output_expr.clone(),
            sens_type_idx: match config.sens_type {
                SensType::Dc => 0,
                SensType::Ac => 1,
            },
            ac_freq: format_freq(config.ac_freq),
            filter: config.filter.clone(),
            initialized: true,
        }
    }

    pub fn to_config(&self) -> Result<SensConfig, String> {
        let sens_type = match self.sens_type_idx {
            0 => SensType::Dc,
            1 => SensType::Ac,
            _ => return Err("Select a valid sensitivity mode (DC or AC)".to_owned()),
        };
        let freq = if sens_type == SensType::Ac {
            super::options::parse_si_value(&self.ac_freq)
                .map_err(|err| format!("Invalid AC frequency: {}", err))?
        } else {
            // A disabled AC field has no effect on a DC solve. Keep its draft
            // text intact so switching back to AC restores the authored value.
            SensConfig::default().ac_freq
        };
        let config = SensConfig {
            output_expr: self.output_expr.clone(),
            sens_type,
            ac_freq: freq,
            filter: crate::simulation::config::canonical_sensitivity_filter(&self.filter),
        };
        config.validate()?;
        Ok(config)
    }

    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&SensConfig::default());
        }
    }
}

fn format_freq(f: f64) -> String {
    if f >= 1e9 {
        format!("{}G", f / 1e9)
    } else if f >= 1e6 {
        format!("{}Meg", f / 1e6)
    } else if f >= 1e3 {
        format!("{}k", f / 1e3)
    } else {
        format!("{}", f)
    }
}

#[cfg(test)]
mod tests {
    use super::{SensConfig, SensDialogState, SensType};

    #[test]
    fn dc_sensitivity_ignores_disabled_frequency_without_changing_the_draft() {
        for text in ["", "not-a-frequency", "-1", "NaN"] {
            let mut state = SensDialogState::from_config(&SensConfig::new("I(V1)"));
            state.ac_freq = text.to_owned();
            assert_eq!(state.to_config().unwrap().sens_type, SensType::Dc);
            assert_eq!(state.ac_freq, text);
            state.sens_type_idx = 1;
            assert!(state.to_config().is_err());
        }
    }

    #[test]
    fn sensitivity_rejects_unknown_modes_and_blank_outputs() {
        let mut state = SensDialogState::from_config(&SensConfig::default());
        state.sens_type_idx = 2;
        assert!(state.to_config().unwrap_err().contains("mode"));
        state.sens_type_idx = 0;
        state.output_expr = " \t ".to_owned();
        assert!(state.to_config().unwrap_err().contains("Output expression"));
    }

    /// A draft saved before the filter existed restores as the set it
    /// computed, in the engine's own words, visible and editable in the row.
    #[test]
    fn a_sensitivity_plan_saved_before_filters_still_differentiates_its_design_parameters() {
        let legacy: SensDialogState =
            serde_json::from_str(r#"{"output_expr":"V(OUT)","sens_type_idx":0,"ac_freq":"1Meg"}"#)
                .expect("a draft written before filters existed still opens");
        assert_eq!(legacy.filter, "PARAM:*");
        assert_eq!(legacy.to_config().unwrap().filter, "PARAM:*");

        // The two retired checkbox keys are still accepted, and still ignored.
        let retired: SensDialogState = serde_json::from_str(
            r#"{"output_expr":"V(OUT)","include_params":true,"include_devices":false}"#,
        )
        .expect("a draft with the retired keys still opens");
        assert_eq!(retired.filter, "PARAM:*");
    }

    /// An empty filter saved deliberately restores empty. Absence means
    /// "saved before filters"; empty means "the engine's own default".
    #[test]
    fn an_empty_filter_survives_a_save_as_empty() {
        let mut state = SensDialogState::from_config(&SensConfig::default());
        assert_eq!(state.filter, "");
        let text = serde_json::to_string(&state).expect("a draft serializes");
        assert!(text.contains("\"filter\":\"\""), "{text}");
        let restored: SensDialogState = serde_json::from_str(&text).expect("a draft restores");
        assert_eq!(restored.filter, "");

        // And what a reader types is canonicalized on its way to the card,
        // never in the field.
        state.filter = "r1, m1:w  param:gain r1".to_owned();
        assert_eq!(state.to_config().unwrap().filter, "R1 M1:W PARAM:GAIN");
        assert_eq!(state.filter, "r1, m1:w  param:gain r1");
    }

    /// A filter item the `.SENS` card would read as something else is
    /// refused, rather than silently changing the analysis that runs.
    #[test]
    fn a_filter_the_card_would_misread_is_refused_by_the_form() {
        for (filter, fragment) in [
            ("R1 AC", "mode keyword"),
            ("dc", "mode keyword"),
            ("PARAM:", "names no design parameter"),
            ("R1 {gain}", "cannot carry"),
            ("R1,,,M1:W", ""),
        ] {
            let mut state = SensDialogState::from_config(&SensConfig::default());
            state.filter = filter.to_owned();
            let outcome = state.to_config();
            if fragment.is_empty() {
                // Repeated separators are a typist's, not an error.
                assert_eq!(outcome.unwrap().filter, "R1 M1:W");
            } else {
                let error = outcome.expect_err("{filter} must be refused");
                assert!(error.contains(fragment), "{filter}: {error}");
            }
        }
    }

    #[test]
    fn ac_sensitivity_dialog_rejects_invalid_frequency_text() {
        let mut state = SensDialogState::from_config(
            &SensConfig::new("V(out)")
                .with_type(SensType::Ac)
                .with_ac_freq(1e6),
        );
        state.ac_freq = "not-a-frequency".to_string();

        let err = state
            .to_config()
            .expect_err("invalid AC frequency text must not silently default");
        assert!(err.contains("AC frequency"));
    }
}
