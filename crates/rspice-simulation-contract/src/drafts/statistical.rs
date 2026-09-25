//! Persisted transient-noise and DC mismatch authoring drafts.

use super::parse::{parse_nonnegative, parse_positive};

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

/// Read an authored DC mismatch share threshold, or `None` for the card's own
/// default.
///
/// Two spellings mean the same card and are canonicalized to one: an empty
/// field, and a threshold authored as exactly zero. The engine's default IS
/// zero (`DcMatchCard::threshold`), so `THRESHOLD=0` is the unauthored card —
/// and if the two specifications differed, one analysis would have two plan
/// digests and a saved plan would re-run as a different request.
///
/// Both the plan draft and the specification builder read a threshold through
/// here so there is one account of that identity.
pub fn dc_mismatch_share_threshold(text: &str) -> Result<Option<f64>, String> {
    if text.trim().is_empty() {
        return Ok(None);
    }
    let value = crate::options::parse_si_value(text)
        .map_err(|error| format!("invalid share threshold: {error}"))?;
    if value == 0.0 {
        return Ok(None);
    }
    Ok(Some(value))
}

pub fn validate_transient_noise(draft: &TransientNoiseDraft) -> Option<String> {
    (|| {
        let stop = parse_positive(&draft.stop_time, "stop time")?;
        let step = parse_positive(&draft.step_time, "step time")?;
        let start = parse_nonnegative(&draft.start_time, "start time")?;
        let max_step = parse_positive(&draft.max_step, "maximum step")?;
        if start >= stop {
            return Err("start time must be less than stop time".to_owned());
        }
        if step > stop || max_step > stop {
            return Err("time steps must not exceed stop time".to_owned());
        }
        draft.parsed_seed()?;
        let fmax = parse_positive(&draft.noise_fmax, "maximum noise frequency")?;
        // An empty floor is the engine's `1/tstop` derivation, not a missing
        // value, so it is not an error. An authored one has to sit inside the
        // band the ceiling opens.
        if !draft.noise_fmin.trim().is_empty() {
            let fmin = parse_positive(&draft.noise_fmin, "minimum noise frequency")?;
            if fmin >= fmax {
                return Err(
                    "minimum noise frequency must be below the maximum noise frequency".to_owned(),
                );
            }
        }
        parse_nonnegative(&draft.scale, "noise scale")?;
        Ok(())
    })()
    .err()
}

/// What a DC mismatch draft refuses on, in the engine card's own words.
///
/// Text that is not a number at all is this layer's own to answer — the
/// engine never sees a half-typed field — but every *range* belongs to the
/// card, so the parsed draft is assembled into the specification the run
/// would carry and that specification is asked. There is then exactly one
/// account of what `.DCMATCH` refuses on, and the form and a hand-written
/// deck are refused by the same sentence.
///
/// A contributor limit of zero is legal here because it is legal on the card:
/// zero is how a deck asks for every contributor.
pub fn validate_dc_mismatch(draft: &DcMismatchDraft) -> Option<String> {
    (|| {
        let sigma_multiplier = crate::options::parse_si_value(&draft.sigma_multiplier)
            .map_err(|error| format!("invalid sigma multiplier: {error}"))?;
        let contributor_limit = draft
            .contributor_limit
            .trim()
            .parse::<usize>()
            .map_err(|_| "contributor limit must be a non-negative integer".to_owned())?;
        let contribution_threshold = dc_mismatch_share_threshold(&draft.share_threshold)?;
        crate::analysis_spec::AnalysisSpec::DcMismatch {
            moment_options: draft.moment_options()?,
            output_expression: draft.output_expression.trim().to_owned(),
            sigma_multiplier,
            contributor_limit,
            include_process: draft.include_process,
            include_mismatch: draft.include_mismatch,
            normalized_contributions: draft.normalized_contributions,
            contribution_threshold,
        }
        .validate()
    })()
    .err()
}
