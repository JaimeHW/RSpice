//! Envelope analysis configuration.
//!
//! The controls in this module intentionally mirror the workbench mockup.
//! The execution specification retains the first carrier as the legacy
//! `fundamental_freq` while this authoring model presents one ordered list.

use std::collections::HashSet;

use crate::envelope_initialization::EnvelopeInitializationConfig;
use crate::envelope_multirate::EnvelopeMultirateConfig;
use crate::envelope_policy::{
    EnvelopeAdaptiveMode, EnvelopeExtractionPath, EnvelopeInitialPeriodicSolve,
};
use crate::options::parse_si_value;
use rspice_app_types::quantity::{
    QuantityInputKind, QuantityPresentationPolicy, UiNumberLocale, parse_ui_quantity,
};

mod initialization;
mod multirate;
pub use initialization::EnvelopeInitializationState;
pub use multirate::EnvelopeMultirateState;

/// Complete mockup-owned envelope configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct EnvelopeConfig {
    pub multirate: Option<EnvelopeMultirateConfig>,
    pub initialization: EnvelopeInitializationConfig,
    pub carrier_tones: Vec<f64>,
    pub stop_time: f64,
    pub envelope_step: f64,
    pub harmonic_order: u32,
    pub modulation_sources: Vec<String>,
    pub initial_periodic_solve: EnvelopeInitialPeriodicSolve,
    pub adaptive_mode: EnvelopeAdaptiveMode,
    pub extraction_path: EnvelopeExtractionPath,
}

impl Default for EnvelopeConfig {
    fn default() -> Self {
        Self {
            multirate: None,
            initialization: Default::default(),
            carrier_tones: vec![1.0e6],
            stop_time: 10.0e-3,
            envelope_step: 1.0e-6,
            harmonic_order: 9,
            // The author must select an actual time-varying source from the
            // active circuit (or declare an exact source group). Mockup sample
            // names are never valid configuration defaults.
            modulation_sources: Vec::new(),
            initial_periodic_solve: EnvelopeInitialPeriodicSolve::HarmonicBalance,
            adaptive_mode: EnvelopeAdaptiveMode::Enabled,
            extraction_path: EnvelopeExtractionPath::Projection,
        }
    }
}

impl EnvelopeConfig {
    /// Serializes every user-owned setting into the canonical envelope
    /// command form used by generated decks and diagnostics.
    pub fn to_spice(&self) -> String {
        let carriers = self
            .carrier_tones
            .iter()
            .map(|frequency| format_freq(*frequency))
            .collect::<Vec<_>>()
            .join(",");
        let sources = self.modulation_sources.join(",");
        let mut line = format!(
            ".envlp carriers=[{carriers}] stop={} step={} harmonic_order={} modulation_sources=[{sources}] initial_periodic_solve={} adaptive={} extraction={}",
            format_time(self.stop_time),
            format_time(self.envelope_step),
            self.harmonic_order,
            initial_solve_keyword(self.initial_periodic_solve),
            adaptive_keyword(self.adaptive_mode),
            extraction_keyword(self.extraction_path),
        );
        if self.initialization != EnvelopeInitializationConfig::default() {
            line.push_str(&format!(
                " initializer={}",
                serde_json::to_string(&self.initialization)
                    .expect("initializer contains only serializable scalar settings")
            ));
        }
        if let Some(settings) = &self.multirate {
            line.push_str(&format!(
                " multirate={}",
                serde_json::to_string(settings).expect("finite validated envelope controls")
            ));
        }
        line
    }

    pub fn validate(&self) -> Result<(), String> {
        if let Some(settings) = &self.multirate {
            return settings
                .core_config(
                    &self.carrier_tones,
                    self.harmonic_order as usize,
                    self.stop_time,
                    self.envelope_step,
                    &self.modulation_sources,
                )
                .map(|_| ());
        }
        validate_carrier_tones(&self.carrier_tones)?;
        if !self.stop_time.is_finite() || self.stop_time <= 0.0 {
            return Err("Envelope stop must be finite and positive".to_owned());
        }
        if !self.envelope_step.is_finite() || self.envelope_step <= 0.0 {
            return Err("Envelope step must be finite and positive".to_owned());
        }
        if self.harmonic_order == 0 {
            return Err("Harmonic order must be a positive integer (minimum 1)".to_owned());
        }
        match self.initial_periodic_solve {
            EnvelopeInitialPeriodicSolve::HarmonicBalance => {
                self.initialization
                    .hb_config(&self.carrier_tones, self.harmonic_order as usize)?;
            }
            EnvelopeInitialPeriodicSolve::PeriodicSteadyState => {
                self.initialization
                    .pss_config(&self.carrier_tones, self.harmonic_order as usize)?;
            }
            EnvelopeInitialPeriodicSolve::TransientSpectralEstimate => {}
        }

        // Empty is the legacy spelling for declared/primary-source inference.
        // It is safe only on the exact legacy transient-estimate, fixed-step
        // path; periodic/adaptive policies require explicit source identity.
        let legacy_source_inference = self.initial_periodic_solve
            == EnvelopeInitialPeriodicSolve::TransientSpectralEstimate
            && self.adaptive_mode == EnvelopeAdaptiveMode::FixedEnvelopeStep;
        validate_modulation_sources(&self.modulation_sources, legacy_source_inference)?;
        if self.envelope_step > self.stop_time {
            return Err("Envelope step cannot exceed envelope stop".to_owned());
        }
        Ok(())
    }
}

/// Serializable state for the exact workbench Envelope form.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvelopeDialogState {
    #[serde(default)]
    pub multirate_enabled: bool,
    #[serde(default)]
    pub multirate: EnvelopeMultirateState,
    #[serde(default)]
    pub initialization: EnvelopeInitializationState,
    #[serde(default = "default_carrier_tones_text", alias = "fundamental")]
    pub carrier_tones: String,
    #[serde(default = "default_stop_time_text")]
    pub stop_time: String,
    #[serde(default = "default_envelope_step_text")]
    pub envelope_step: String,
    #[serde(default = "default_harmonic_order_text", alias = "harmonics")]
    pub harmonic_order: String,
    #[serde(default)]
    pub modulation_sources: String,
    #[serde(default = "legacy_initial_periodic_solve_idx")]
    pub initial_periodic_solve_idx: usize,
    #[serde(default = "legacy_adaptive_mode_idx")]
    pub adaptive_mode_idx: usize,
    /// Retired: the projection is the only extraction path the runner has,
    /// so the row that pinned this to its one value was reading a control
    /// back from itself. Accepted so a project saved while the row existed
    /// still opens, then dropped.
    #[serde(default, rename = "extraction_path_idx", skip_serializing)]
    _extraction_path_idx: Option<serde::de::IgnoredAny>,
    /// Accepted solely to migrate drafts written by the removed AM/FM/PM/IQ
    /// placeholder. It is never serialized or interpreted as a new control.
    #[serde(default, rename = "modulation_idx", skip_serializing)]
    _legacy_modulation_idx: usize,
    #[serde(skip)]
    pub initialized: bool,
}

impl EnvelopeDialogState {
    pub fn from_config(config: &EnvelopeConfig) -> Self {
        Self {
            multirate_enabled: config.multirate.is_some(),
            multirate: config
                .multirate
                .as_ref()
                .map(EnvelopeMultirateState::from_config)
                .unwrap_or_default(),
            initialization: EnvelopeInitializationState::from_config(&config.initialization),
            carrier_tones: config
                .carrier_tones
                .iter()
                .map(|frequency| format_frequency_field(*frequency))
                .collect::<Vec<_>>()
                .join(", "),
            stop_time: format_time(config.stop_time),
            envelope_step: format_time(config.envelope_step),
            harmonic_order: config.harmonic_order.to_string(),
            modulation_sources: config.modulation_sources.join(", "),
            initial_periodic_solve_idx: match config.initial_periodic_solve {
                EnvelopeInitialPeriodicSolve::HarmonicBalance => 0,
                EnvelopeInitialPeriodicSolve::PeriodicSteadyState => 1,
                EnvelopeInitialPeriodicSolve::TransientSpectralEstimate => 2,
            },
            adaptive_mode_idx: match config.adaptive_mode {
                EnvelopeAdaptiveMode::Enabled => 0,
                EnvelopeAdaptiveMode::FixedEnvelopeStep => 1,
                EnvelopeAdaptiveMode::EventAlignedOnly => 2,
            },
            _extraction_path_idx: None,
            _legacy_modulation_idx: 0,
            initialized: true,
        }
    }

    pub fn to_config(&self) -> Result<EnvelopeConfig, String> {
        let carrier_tones = parse_carrier_tones(&self.carrier_tones)?;
        let stop_time = parse_si_value(&self.stop_time)
            .map_err(|error| format!("Invalid envelope stop: {error}"))?;
        let envelope_step = parse_si_value(&self.envelope_step)
            .map_err(|error| format!("Invalid envelope step: {error}"))?;
        let harmonic_order = self
            .harmonic_order
            .trim()
            .parse::<u32>()
            .map_err(|_| "Harmonic order must be a positive integer (minimum 1)".to_owned())?;
        let modulation_sources = parse_source_list(&self.modulation_sources)?;
        let initial_periodic_solve = if self.multirate_enabled {
            EnvelopeInitialPeriodicSolve::HarmonicBalance
        } else {
            match self.initial_periodic_solve_idx {
                0 => EnvelopeInitialPeriodicSolve::HarmonicBalance,
                1 => EnvelopeInitialPeriodicSolve::PeriodicSteadyState,
                2 => EnvelopeInitialPeriodicSolve::TransientSpectralEstimate,
                _ => return Err("Invalid initial periodic solve selection".to_owned()),
            }
        };
        let adaptive_mode = match self.adaptive_mode_idx {
            0 => EnvelopeAdaptiveMode::Enabled,
            1 => EnvelopeAdaptiveMode::FixedEnvelopeStep,
            2 => EnvelopeAdaptiveMode::EventAlignedOnly,
            _ => return Err("Invalid Envelope output-schedule selection".to_owned()),
        };
        // One path, and no control that could ask for another.
        let extraction_path = EnvelopeExtractionPath::Projection;
        let config = EnvelopeConfig {
            multirate: self
                .multirate_enabled
                .then(|| self.multirate.to_config(carrier_tones.len()))
                .transpose()?,
            initialization: if self.multirate_enabled {
                Default::default()
            } else {
                self.initialization
                    .to_config(self.initial_periodic_solve_idx)?
            },
            carrier_tones,
            stop_time,
            envelope_step,
            harmonic_order,
            modulation_sources,
            initial_periodic_solve,
            adaptive_mode,
            extraction_path,
        };
        config.validate()?;
        Ok(config)
    }

    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&EnvelopeConfig::default());
        }
    }
}

fn parse_carrier_tones(value: &str) -> Result<Vec<f64>, String> {
    reject_empty_list_items(value, "carrier tone")?;
    let tokens = split_list(value);
    if tokens.is_empty() {
        return Err("At least one carrier tone is required".to_owned());
    }
    tokens
        .into_iter()
        .map(|token| {
            match parse_ui_quantity(
                token,
                QuantityInputKind::Frequency,
                QuantityPresentationPolicy::default(),
                UiNumberLocale::default(),
            ) {
                Ok(value) => Ok(value),
                Err(quantity_error) => parse_si_value(token).map_err(|spice_error| {
                    format!(
                        "Invalid carrier tone '{token}': {quantity_error}; SPICE form: {spice_error}"
                    )
                }),
            }
        })
        .collect()
}

fn parse_source_list(value: &str) -> Result<Vec<String>, String> {
    reject_empty_list_items(value, "modulation source")?;
    let values = split_list(value)
        .into_iter()
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    validate_modulation_sources(&values, true)?;
    Ok(values)
}

fn split_list(value: &str) -> Vec<&str> {
    value
        .split([',', ';', '\n'])
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .collect()
}

fn reject_empty_list_items(value: &str, noun: &str) -> Result<(), String> {
    if !value.trim().is_empty()
        && value
            .split([',', ';', '\n'])
            .any(|token| token.trim().is_empty())
    {
        return Err(format!("{noun} list contains an empty item"));
    }
    Ok(())
}

fn validate_carrier_tones(carrier_tones: &[f64]) -> Result<(), String> {
    if carrier_tones.is_empty() {
        return Err("At least one carrier tone is required".to_owned());
    }
    let mut seen = HashSet::with_capacity(carrier_tones.len());
    for frequency in carrier_tones {
        if !frequency.is_finite() || *frequency <= 0.0 {
            return Err("Carrier tones must be finite and positive".to_owned());
        }
        if !seen.insert(frequency.to_bits()) {
            return Err("Carrier tones must be unique".to_owned());
        }
    }
    Ok(())
}

/// An empty list requests legacy declared/primary-source inference only when
/// the caller permits it for the transient-estimate, fixed-step path. Every
/// named source is required to be nonempty, trimmed, and unique.
pub(crate) fn validate_modulation_sources(
    modulation_sources: &[String],
    allow_empty: bool,
) -> Result<(), String> {
    if modulation_sources.is_empty() && !allow_empty {
        return Err("At least one modulation source is required".to_owned());
    }
    let mut seen = HashSet::with_capacity(modulation_sources.len());
    for source in modulation_sources {
        let trimmed = source.trim();
        if trimmed.is_empty() {
            return Err("Modulation source names cannot be empty".to_owned());
        }
        if trimmed != source {
            return Err("Modulation source names cannot contain surrounding whitespace".to_owned());
        }
        if !seen.insert(trimmed.to_ascii_lowercase()) {
            return Err("Modulation source names must be unique".to_owned());
        }
    }
    Ok(())
}

fn initial_solve_keyword(value: EnvelopeInitialPeriodicSolve) -> &'static str {
    match value {
        EnvelopeInitialPeriodicSolve::HarmonicBalance => "hb",
        EnvelopeInitialPeriodicSolve::PeriodicSteadyState => "pss",
        EnvelopeInitialPeriodicSolve::TransientSpectralEstimate => "transient_spectral_estimate",
    }
}

fn adaptive_keyword(value: EnvelopeAdaptiveMode) -> &'static str {
    match value {
        EnvelopeAdaptiveMode::Enabled => "enabled",
        EnvelopeAdaptiveMode::FixedEnvelopeStep => "fixed_envelope_step",
        EnvelopeAdaptiveMode::EventAlignedOnly => "event_aligned_only",
    }
}

fn extraction_keyword(value: EnvelopeExtractionPath) -> &'static str {
    match value {
        EnvelopeExtractionPath::Projection => "projection",
    }
}

const fn legacy_initial_periodic_solve_idx() -> usize {
    2
}

const fn legacy_adaptive_mode_idx() -> usize {
    1
}

fn default_carrier_tones_text() -> String {
    "1 MHz".to_owned()
}

fn default_stop_time_text() -> String {
    "10m".to_owned()
}

fn default_envelope_step_text() -> String {
    "1u".to_owned()
}

fn default_harmonic_order_text() -> String {
    "9".to_owned()
}

fn format_freq(frequency: f64) -> String {
    if frequency >= 1.0e9 {
        format!("{}G", frequency / 1.0e9)
    } else if frequency >= 1.0e6 {
        format!("{}Meg", frequency / 1.0e6)
    } else if frequency >= 1.0e3 {
        format!("{}k", frequency / 1.0e3)
    } else {
        frequency.to_string()
    }
}

fn format_frequency_field(frequency: f64) -> String {
    if frequency >= 1.0e9 {
        format!("{} GHz", frequency / 1.0e9)
    } else if frequency >= 1.0e6 {
        format!("{} MHz", frequency / 1.0e6)
    } else if frequency >= 1.0e3 {
        format!("{} kHz", frequency / 1.0e3)
    } else {
        format!("{frequency} Hz")
    }
}

fn format_time(time: f64) -> String {
    if time >= 1.0e-3 {
        format!("{}m", time / 1.0e-3)
    } else if time >= 1.0e-6 {
        format!("{}u", time / 1.0e-6)
    } else if time >= 1.0e-9 {
        format!("{}n", time / 1.0e-9)
    } else {
        format!("{}p", time / 1.0e-12)
    }
}
