//! Persisted transient-noise and DC mismatch authoring drafts.

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransientNoiseDraft {
    pub stop_time: String,
    pub step_time: String,
    pub start_time: String,
    pub max_step: String,
    /// Empty inherits `.OPTIONS SEED` or the engine's reproducible default.
    pub seed: String,
    pub noise_fmax: String,
    /// Lowest flicker frequency the run represents, in hertz. Empty is the
    /// engine's own derivation — `1/tstop`, the longest period the window can
    /// resolve — and is the default, because a run that has not been told
    /// otherwise should represent every period it can.
    ///
    /// Defaulted on read so a plan saved before this field existed opens with
    /// the derivation it was running under rather than refusing to load.
    #[serde(default)]
    pub noise_fmin: String,
    pub scale: String,
    pub use_initial_conditions: bool,
}

impl Default for TransientNoiseDraft {
    fn default() -> Self {
        Self {
            stop_time: "1u".to_owned(),
            step_time: "1n".to_owned(),
            start_time: "0".to_owned(),
            max_step: "10n".to_owned(),
            seed: "1".to_owned(),
            noise_fmax: "10G".to_owned(),
            noise_fmin: String::new(),
            scale: "1".to_owned(),
            use_initial_conditions: false,
        }
    }
}

impl TransientNoiseDraft {
    pub fn parsed_seed(&self) -> Result<Option<u64>, String> {
        let seed = self.seed.trim();
        if seed.is_empty() {
            Ok(None)
        } else {
            seed.parse::<u64>().map(Some).map_err(|_| {
                "TNOISE seed must be an integer from 0 to 18446744073709551615, or blank to inherit"
                    .to_owned()
            })
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DcMismatchDraft {
    #[serde(default)]
    pub moment_relative_tolerance: String,
    #[serde(default)]
    pub moment_max_points: String,
    pub output_expression: String,
    pub sigma_multiplier: String,
    pub contributor_limit: String,
    /// Smallest variance share a contributor must carry to be listed. Empty
    /// is the card's own default of zero, which retains every contributor the
    /// limit above allows, and is what a plan saved before this control
    /// existed opens with.
    #[serde(default)]
    pub share_threshold: String,
    pub include_process: bool,
    pub include_mismatch: bool,
    pub normalized_contributions: bool,
}

/// A fresh draft is the bare card `.DCMATCH OUT=V(out)`.
///
/// Every default here is the engine card's own, read from `DcMatchCard`
/// rather than copied: a default Studio run and a hand-written `.DCMATCH
/// OUT=V(out)` are then the same analysis, and the engine remains the one
/// place a default is decided. The two numbers were `3` and `25`, which named
/// a study the card never described.
impl Default for DcMismatchDraft {
    fn default() -> Self {
        Self {
            moment_relative_tolerance: String::new(),
            moment_max_points: String::new(),
            output_expression: "V(out)".to_owned(),
            sigma_multiplier: "1".to_owned(),
            contributor_limit: rspice_core::netlist::DcMatchCard::DEFAULT_CONTRIBUTORS.to_string(),
            share_threshold: String::new(),
            include_process: false,
            include_mismatch: true,
            normalized_contributions: true,
        }
    }
}

impl DcMismatchDraft {
    /// Resolve blank numerical controls to the engine's defaults.
    pub fn moment_options(&self) -> Result<rspice_core::netlist::StatisticalMomentOptions, String> {
        let mut options = rspice_core::netlist::StatisticalMomentOptions::default();
        if !self.moment_relative_tolerance.trim().is_empty() {
            options.relative_tolerance =
                crate::options::parse_si_value(&self.moment_relative_tolerance)
                    .map_err(|error| format!("Invalid moment relative tolerance: {error}"))?;
        }
        if !self.moment_max_points.trim().is_empty() {
            options.max_points = self
                .moment_max_points
                .trim()
                .parse()
                .map_err(|_| "Moment integration point budget must be an integer".to_owned())?;
        }
        options.validate().map_err(|error| error.to_string())?;
        Ok(options)
    }
}
