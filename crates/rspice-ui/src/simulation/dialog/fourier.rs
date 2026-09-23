//! Fourier Analysis Configuration
//!
//! Configuration for Fourier/spectral analysis of transient waveforms.
//! Computes harmonic distortion (THD), spectral content, and fundamental metrics.

use super::options::parse_si_value;

/// Fourier analysis configuration
#[derive(Debug, Clone)]
pub struct FourierConfig {
    /// Fundamental frequency (Hz)
    pub fundamental_freq: f64,
    /// Number of harmonics to compute
    pub num_harmonics: u32,
    pub num_periods: u32,
    /// Output node to analyze
    pub output_node: String,
    /// Reference node (ground if empty)
    pub output_ref: String,
    /// Every further output the same card asks for, in authored order, each
    /// spelled the way the card spells it: `V(node)`, `V(node+, node-)`, or
    /// `I(device)`. Empty is a one-output card.
    pub additional_outputs: Vec<String>,
    /// Analysis window start time
    pub start_time: f64,
    /// Analysis window stop time
    pub stop_time: f64,
    /// Compute THD
    pub compute_thd: bool,
    /// Normalize to fundamental
    pub normalize: bool,
}

impl Default for FourierConfig {
    fn default() -> Self {
        Self {
            fundamental_freq: 1e6,
            num_harmonics: 10,
            num_periods: 1,
            output_node: "VOUT".to_string(),
            output_ref: String::new(),
            additional_outputs: Vec::new(),
            start_time: 0.0,
            stop_time: 10e-6,
            compute_thd: true,
            normalize: true,
        }
    }
}

impl FourierConfig {
    #[cfg(test)]
    pub fn new(fundamental: f64, harmonics: u32) -> Self {
        Self {
            fundamental_freq: fundamental,
            num_harmonics: harmonics,
            ..Default::default()
        }
    }

    #[cfg(test)]
    pub fn with_window(mut self, start: f64, stop: f64) -> Self {
        self.start_time = start;
        self.stop_time = stop;
        self
    }

    /// Number of periods in analysis window
    pub fn periods_in_window(&self) -> f64 {
        if self.fundamental_freq > 0.0 {
            (self.stop_time - self.start_time) * self.fundamental_freq
        } else {
            0.0
        }
    }

    /// Every output this card asks for, in authored order, spelled the way the
    /// card spells it.
    pub fn outputs(&self) -> Vec<String> {
        let mut outputs = Vec::with_capacity(1 + self.additional_outputs.len());
        if !self.output_node.trim().is_empty() {
            outputs.push(crate::services::simulation_runner::fourier_card_output(
                &self.output_node,
                &self.output_ref,
            ));
        }
        outputs.extend(
            self.additional_outputs
                .iter()
                .map(|output| output.trim().to_owned()),
        );
        outputs
    }

    pub fn to_spice(&self) -> String {
        let mut cmd = format!(".four {} {}", self.fundamental_freq, self.num_harmonics);
        for output in self.outputs() {
            cmd.push(' ');
            cmd.push_str(&output);
        }
        cmd.push_str(&format!(
            " PERIODS={} FROM={} TO={}",
            self.num_periods, self.start_time, self.stop_time
        ));
        cmd
    }

    pub fn validate(&self) -> Result<(), String> {
        if !self.fundamental_freq.is_finite() || self.fundamental_freq <= 0.0 {
            return Err("Fundamental frequency must be finite and positive".into());
        }
        if self.num_harmonics == 0 {
            return Err("Number of harmonics must be at least 1".into());
        }
        if self.output_node.is_empty() {
            return Err("Output node must be specified".into());
        }
        for (index, output) in self.additional_outputs.iter().enumerate() {
            crate::services::simulation_runner::split_fourier_output(output)
                .map_err(|error| format!("Output {}: {error}", index + 2))?;
        }
        if !self.start_time.is_finite() || !self.stop_time.is_finite() {
            return Err("Analysis window times must be finite".into());
        }
        if self.start_time < 0.0 {
            return Err("Start time must be non-negative".into());
        }
        if self.stop_time <= self.start_time {
            return Err("Stop time must be after start time".into());
        }
        if self.num_periods == 0 {
            return Err("Number of periods must be at least 1".into());
        }
        let available = self.periods_in_window();
        if !available.is_finite()
            || available + 16.0 * f64::EPSILON * available.abs() < self.num_periods as f64
        {
            return Err("Analysis window must contain the requested complete periods".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FourierDialogState {
    pub fundamental: String,
    pub harmonics: String,
    #[serde(default = "default_period_text")]
    pub periods: String,
    pub output_node: String,
    /// Reference node for the primary voltage output; empty means ground.
    ///
    /// Older drafts did not expose this field and therefore deserialize with
    /// the canonical empty reference.
    #[serde(default)]
    pub output_ref: String,
    /// The outputs authored beside the first one, each one row. A draft saved
    /// before the list existed carries none, which is the one-output card it
    /// was.
    #[serde(default)]
    pub additional_outputs: Vec<String>,
    pub start_time: String,
    pub stop_time: String,
    pub compute_thd: bool,
    pub normalize: bool,
    #[serde(skip)]
    pub initialized: bool,
}

impl FourierDialogState {
    pub fn from_config(config: &FourierConfig) -> Self {
        Self {
            fundamental: format_freq(config.fundamental_freq),
            harmonics: config.num_harmonics.to_string(),
            periods: config.num_periods.to_string(),
            output_node: config.output_node.clone(),
            output_ref: config.output_ref.clone(),
            additional_outputs: config.additional_outputs.clone(),
            start_time: format_time(config.start_time),
            stop_time: format_time(config.stop_time),
            compute_thd: config.compute_thd,
            normalize: config.normalize,
            initialized: true,
        }
    }

    pub fn to_config(&self) -> Result<FourierConfig, String> {
        let fund = parse_si_value(&self.fundamental).map_err(|e| format!("Bad freq: {}", e))?;
        let harm: u32 = self.harmonics.parse().map_err(|_| "Bad harmonics")?;
        let start = parse_si_value(&self.start_time).map_err(|e| format!("Bad start: {}", e))?;
        let stop = parse_si_value(&self.stop_time).map_err(|e| format!("Bad stop: {}", e))?;

        let config = FourierConfig {
            fundamental_freq: fund,
            num_harmonics: harm,
            num_periods: self
                .periods
                .parse()
                .map_err(|_| "Periods must be a positive integer")?,
            output_node: self.output_node.clone(),
            output_ref: self.output_ref.trim().to_owned(),
            additional_outputs: self
                .additional_outputs
                .iter()
                .map(|output| output.trim().to_owned())
                .filter(|output| !output.is_empty())
                .collect(),
            start_time: start,
            stop_time: stop,
            compute_thd: self.compute_thd,
            normalize: self.normalize,
        };
        config.validate()?;
        Ok(config)
    }

    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&FourierConfig::default());
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

fn format_time(t: f64) -> String {
    if t == 0.0 {
        "0".into()
    } else if t >= 1e-3 {
        format!("{}m", t / 1e-3)
    } else if t >= 1e-6 {
        format!("{}u", t / 1e-6)
    } else if t >= 1e-9 {
        format!("{}n", t / 1e-9)
    } else {
        format!("{}p", t / 1e-12)
    }
}

#[cfg(test)]
#[allow(clippy::items_after_test_module)]
mod tests {
    use super::{FourierConfig, FourierDialogState};

    #[test]
    fn fourier_dialog_rejects_invalid_start_time_text() {
        let mut state = FourierDialogState::from_config(&FourierConfig::default());
        state.start_time = "bad-start".to_string();

        let err = state
            .to_config()
            .expect_err("invalid Fourier start time must not silently default");
        assert!(err.contains("start"));
    }

    /// The card the form writes is the card the engine reads: every output in
    /// authored order, on one `.FOUR` line, because one transient serves them
    /// all.
    #[test]
    fn the_fourier_card_carries_every_output_it_was_given() {
        let mut config = FourierConfig::new(1.0e3, 9).with_window(0.0, 5.0e-3);
        config.output_node = "out".to_owned();
        config.output_ref = "ref".to_owned();
        config.additional_outputs = vec![
            "V(mid,out)".to_owned(),
            " I(V1) ".to_owned(),
            "V(in)".to_owned(),
        ];
        config.validate().expect("every output is well spelled");
        let card = config.to_spice();
        assert_eq!(
            card,
            ".four 1000 9 V(out,ref) V(mid,out) I(V1) V(in) PERIODS=1 FROM=0 TO=0.005"
        );

        let netlist = rspice_core::Netlist::parse(&format!(
            "Fourier card\nV1 in 0 SIN(0 1 1k)\nR1 in mid 1k\nR2 mid out 1k\nR3 out 0 1k\n{card}\n.tran 10u 5m\n.end\n"
        ))
        .expect("the engine parses the card the form wrote");
        let outputs = netlist
            .analyses
            .iter()
            .find_map(|command| match command {
                rspice_core::netlist::AnalysisCommand::Four { outputs, .. } => {
                    Some(outputs.clone())
                }
                _ => None,
            })
            .expect("the deck carries a .four card");
        let spelled = outputs
            .iter()
            .map(|output| {
                output
                    .chars()
                    .filter(|character| !character.is_whitespace())
                    .flat_map(char::to_uppercase)
                    .collect::<String>()
            })
            .collect::<Vec<_>>();
        assert_eq!(
            spelled,
            vec!["V(OUT)", "V(MID,OUT)", "I(V1)", "V(IN)"],
            "{outputs:?}"
        );
    }

    #[test]
    fn a_fourier_primary_reference_round_trips_through_the_draft() {
        let mut config = FourierConfig::default();
        config.output_node = "out".to_owned();
        config.output_ref = "sense".to_owned();
        let state = FourierDialogState::from_config(&config);
        let restored = state.to_config().expect("the differential output is valid");
        assert_eq!(restored.output_ref, "sense");
        assert_eq!(restored.outputs(), vec!["V(out,sense)".to_owned()]);

        let legacy: FourierDialogState = serde_json::from_str(
            r#"{"fundamental":"1Meg","harmonics":"10","output_node":"out","start_time":"0","stop_time":"10u","compute_thd":true,"normalize":true}"#,
        )
        .expect("a pre-reference draft restores");
        assert!(legacy.output_ref.is_empty());
    }

    /// A draft that names one output restores as a list of exactly one, and a
    /// draft saved before the list existed is such a draft.
    #[test]
    fn a_fourier_draft_saved_with_one_output_restores_as_a_list_of_one() {
        let mut state = FourierDialogState::from_config(&FourierConfig::default());
        state.output_node = "out".to_owned();
        assert!(state.additional_outputs.is_empty());
        let restored: FourierDialogState =
            serde_json::from_str(&serde_json::to_string(&state).expect("a draft serializes"))
                .expect("a draft restores");
        assert!(restored.additional_outputs.is_empty());
        assert_eq!(
            restored.to_config().expect("the draft is valid").outputs(),
            vec!["V(out)".to_owned()]
        );

        // The key did not exist when older drafts were written, and its
        // absence is the one-output card they were.
        let legacy: FourierDialogState = serde_json::from_str(
            r#"{"fundamental":"1Meg","harmonics":"10","output_node":"out","start_time":"0","stop_time":"10u","compute_thd":true,"normalize":true}"#,
        )
        .expect("a draft written before the list existed restores");
        assert_eq!(
            legacy.to_config().expect("the draft is valid").outputs(),
            vec!["V(out)".to_owned()]
        );

        let mut two = state.clone();
        two.additional_outputs = vec!["V(mid)".to_owned()];
        assert_eq!(
            two.to_config().expect("the draft is valid").outputs(),
            vec!["V(out)".to_owned(), "V(mid)".to_owned()]
        );
    }

    #[test]
    fn a_fourier_draft_refuses_an_unspellable_added_output() {
        let mut state = FourierDialogState::from_config(&FourierConfig::default());
        state.additional_outputs = vec!["mid".to_owned()];
        let error = state
            .to_config()
            .expect_err("an added output must be spelled the way the card spells it");
        assert!(error.contains("Output 2"), "{error}");
    }

    #[test]
    fn fourier_config_rejects_non_finite_window() {
        let config = FourierConfig::new(1e6, 10).with_window(f64::NAN, 10e-6);

        let err = config
            .validate()
            .expect_err("NaN Fourier start time must be rejected");
        assert!(err.contains("finite"));
    }
}

fn default_period_text() -> String {
    "1".to_owned()
}
