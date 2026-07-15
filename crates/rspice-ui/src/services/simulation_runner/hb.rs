#![allow(clippy::type_complexity)]

use super::error::{ensure_not_aborted, poll_periodically};
use super::{
    ServiceRunError, ServiceRunResult, build_engine_config, build_multi_tone_hb_layout_with_abort,
    parse_runner_netlist_with_abort,
};
use rspice_core::Value;
use rspice_core::abort_signal::{AbortSignal, NoAbort};
use rspice_core::engine::Engine;
use std::path::Path;
/// Harmonic Balance analysis data
#[derive(Debug, Clone)]
pub struct HbData {
    /// Fundamental frequencies (one per tone)
    pub fundamentals: Vec<Value>,
    /// Number of harmonics per tone
    pub harmonics_per_tone: Vec<usize>,
    /// DC operating point voltages
    pub dc_voltages: Vec<(String, Value)>,
    /// Harmonic spectra: (node_name, [(freq, magnitude, phase_deg)])
    pub spectra: Vec<(String, Vec<(Value, Value, Value)>)>,
    /// Total number of frequency components
    pub num_components: usize,
    /// Whether solution converged
    pub converged: bool,
}

/// Harmonic Balance run configuration passed from the simulation pipeline.
#[derive(Debug, Clone)]
pub struct HbToneRunConfig {
    pub frequency: Value,
    pub harmonics: usize,
    pub source: Option<String>,
    pub name: Option<String>,
}

impl HbToneRunConfig {
    pub fn new(frequency: Value, harmonics: usize) -> Self {
        Self {
            frequency,
            harmonics,
            source: None,
            name: None,
        }
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        let source = source.into();
        self.source = if source.trim().is_empty() {
            None
        } else {
            Some(source)
        };
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        let name = name.into();
        self.name = if name.trim().is_empty() {
            None
        } else {
            Some(name)
        };
        self
    }
}

/// Harmonic Balance run configuration passed from the simulation pipeline.
#[derive(Debug, Clone)]
pub struct HbRunConfig {
    pub tones: Vec<HbToneRunConfig>,
    pub reltol: Value,
    pub abstol: Value,
    pub max_iterations: usize,
    pub damping: Value,
    pub oversample: usize,
    pub collocation_points: Option<usize>,
    pub max_mixing_order: usize,
    pub use_krylov: bool,
    pub gmres_restart: usize,
    pub source_stepping: bool,
    pub verbose: bool,
}

impl Default for HbRunConfig {
    fn default() -> Self {
        Self {
            tones: vec![HbToneRunConfig::new(1e9, 9)],
            reltol: 1e-6,
            abstol: 1e-12,
            max_iterations: 100,
            damping: 1.0,
            oversample: 2,
            collocation_points: None,
            max_mixing_order: 5,
            use_krylov: false,
            gmres_restart: 30,
            source_stepping: false,
            verbose: false,
        }
    }
}

impl HbRunConfig {
    #[cfg(test)]
    fn validate(&self) -> Result<(), String> {
        self.validate_with_abort(&NoAbort)
            .map_err(|error| error.to_string())
    }

    fn validate_with_abort(&self, abort: &dyn AbortSignal) -> ServiceRunResult<()> {
        ensure_not_aborted(abort)?;
        if self.tones.is_empty() {
            return Err(ServiceRunError::Failure(
                "HB requires at least one tone".to_string(),
            ));
        }
        for (idx, tone) in self.tones.iter().enumerate() {
            ensure_not_aborted(abort)?;
            if !tone.frequency.is_finite() || tone.frequency <= 0.0 {
                return Err(ServiceRunError::Failure(format!(
                    "HB tone {} frequency must be positive",
                    idx + 1
                )));
            }
            if tone.harmonics == 0 {
                return Err(ServiceRunError::Failure(format!(
                    "HB tone {} harmonics must be > 0",
                    idx + 1
                )));
            }
        }
        if !self.reltol.is_finite() || self.reltol <= 0.0 {
            return Err(ServiceRunError::Failure(
                "HB reltol must be > 0".to_string(),
            ));
        }
        if !self.abstol.is_finite() || self.abstol <= 0.0 {
            return Err(ServiceRunError::Failure(
                "HB abstol must be > 0".to_string(),
            ));
        }
        if self.max_iterations == 0 {
            return Err(ServiceRunError::Failure(
                "HB max_iterations must be > 0".to_string(),
            ));
        }
        if !self.damping.is_finite() || self.damping <= 0.0 || self.damping > 1.0 {
            return Err(ServiceRunError::Failure(
                "HB damping must be in (0, 1]".to_string(),
            ));
        }
        if self.oversample == 0 {
            return Err(ServiceRunError::Failure(
                "HB oversample must be > 0".to_string(),
            ));
        }
        if let Some(points) = self.collocation_points {
            if points == 0 || points % 2 == 0 {
                return Err(ServiceRunError::Failure(
                    "HB collocation_points must be a positive odd integer".to_string(),
                ));
            }
            let mut core_tones = Vec::with_capacity(self.tones.len());
            for tone in &self.tones {
                ensure_not_aborted(abort)?;
                core_tones.push(rspice_core::analysis::HbTone::new(
                    tone.frequency,
                    tone.harmonics,
                ));
            }
            let core_config = rspice_core::analysis::HbConfig::multi_tone(core_tones);
            let minimum = core_config.minimum_collocation_points().ok_or_else(|| {
                ServiceRunError::Failure(
                    "HB harmonic count exceeds the addressable collocation grid".to_string(),
                )
            })?;
            if points < minimum {
                return Err(ServiceRunError::Failure(format!(
                    "HB collocation_points must contain at least {minimum} points for the configured tones"
                )));
            }
        }
        if self.max_mixing_order == 0 {
            return Err(ServiceRunError::Failure(
                "HB max_mixing_order must be > 0".to_string(),
            ));
        }
        if self.gmres_restart == 0 {
            return Err(ServiceRunError::Failure(
                "HB gmres_restart must be > 0".to_string(),
            ));
        }
        ensure_not_aborted(abort)?;
        Ok(())
    }
}

/// Run Harmonic Balance analysis
///
/// Solves for the steady-state response in the frequency domain,
/// suitable for RF circuits with multiple tones.
pub fn run_hb_analysis(netlist_text: &str, config: &HbRunConfig) -> Result<HbData, String> {
    run_hb_analysis_with_abort(netlist_text, config, &NoAbort).map_err(|error| error.to_string())
}

/// Run Harmonic Balance analysis with cooperative cancellation.
pub fn run_hb_analysis_with_abort(
    netlist_text: &str,
    config: &HbRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<HbData> {
    run_hb_analysis_with_source_path_and_abort(netlist_text, config, None, abort)
}

/// Run Harmonic Balance analysis with a source path used to resolve relative
/// includes and model file references.
pub fn run_hb_analysis_with_source_path(
    netlist_text: &str,
    config: &HbRunConfig,
    source_path: Option<&Path>,
) -> Result<HbData, String> {
    run_hb_analysis_with_source_path_and_abort(netlist_text, config, source_path, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run Harmonic Balance analysis with source-path resolution and cooperative
/// cancellation through validation, layout construction, solving, and result
/// conversion.
pub fn run_hb_analysis_with_source_path_and_abort(
    netlist_text: &str,
    config: &HbRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<HbData> {
    use rspice_core::analysis::{HbConfig, HbTone};
    ensure_not_aborted(abort)?;
    let validation = config.validate_with_abort(abort);
    ensure_not_aborted(abort)?;
    validation?;

    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;

    let engine = Engine::new(build_engine_config(&netlist, None));

    let layout =
        build_multi_tone_hb_layout_with_abort(&config.tones, config.max_mixing_order, abort)?;
    let mut hb_config = HbConfig::new(layout.base_frequency).with_harmonics(layout.max_harmonic);
    let mut core_tones = Vec::with_capacity(config.tones.len());
    for (idx, tone) in config.tones.iter().enumerate() {
        ensure_not_aborted(abort)?;
        let mut hb_tone = HbTone::new(tone.frequency, tone.harmonics.max(1));
        if let Some(name) = tone
            .name
            .as_deref()
            .map(str::trim)
            .filter(|name| !name.is_empty())
        {
            hb_tone = hb_tone.with_name(name.to_string());
        } else {
            hb_tone = hb_tone.with_name(format!("tone{}", idx + 1));
        }
        if let Some(source) = tone
            .source
            .as_deref()
            .map(str::trim)
            .filter(|source| !source.is_empty())
        {
            hb_tone = hb_tone.with_source(source.to_string());
        }
        core_tones.push(hb_tone);
    }
    hb_config.tones = core_tones;
    hb_config = hb_config
        .with_tolerance(config.reltol)
        .with_max_iterations(config.max_iterations)
        .with_damping(config.damping)
        .with_oversample(config.oversample);
    if let Some(points) = config.collocation_points {
        hb_config = hb_config.with_collocation_points(points);
    }
    hb_config.abstol = config.abstol;
    hb_config.max_mixing_order = config.max_mixing_order;
    hb_config.use_krylov = config.use_krylov;
    hb_config.gmres_restart = config.gmres_restart;
    hb_config.source_stepping = config.source_stepping;
    hb_config.verbose = config.verbose;

    // Run actual HB analysis
    let hb_result = engine
        .run_hb_with_abort(&netlist, hb_config, abort)
        .map_err(|error| ServiceRunError::from_core("HB error", error))?;

    // Build fundamentals list
    let mut fundamentals = Vec::with_capacity(config.tones.len());
    let mut harmonics_per_tone = Vec::with_capacity(config.tones.len());
    for tone in &config.tones {
        ensure_not_aborted(abort)?;
        fundamentals.push(tone.frequency);
        harmonics_per_tone.push(tone.harmonics);
    }

    // Extract DC operating point from spectral data
    let mut dc_voltages = Vec::with_capacity(hb_result.result.spectral_voltages.len());
    for sv in &hb_result.result.spectral_voltages {
        ensure_not_aborted(abort)?;
        let dc_val = sv.coefficients.first().map(|c| c.re).unwrap_or(0.0);
        dc_voltages.push((sv.node_name.clone(), dc_val));
    }

    // Build spectra from HB result's spectral voltages
    let mut spectra = Vec::new();
    for sv in &hb_result.result.spectral_voltages {
        ensure_not_aborted(abort)?;
        let mut spectrum = Vec::new();

        // For each harmonic coefficient
        for (h, coeff) in sv.coefficients.iter().enumerate() {
            poll_periodically(abort, h)?;
            let freq = hb_result.fundamental_freq * h as Value;
            let magnitude = coeff.norm();
            let phase_deg = coeff.arg().to_degrees();
            spectrum.push((freq, magnitude, phase_deg));
        }

        spectra.push((format!("V({})", sv.node_name), spectrum));
    }

    // Number of frequency components
    let num_components = hb_result.num_harmonics + 1;

    ensure_not_aborted(abort)?;
    Ok(HbData {
        fundamentals,
        harmonics_per_tone,
        dc_voltages,
        spectra,
        num_components,
        converged: hb_result.converged,
    })
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    struct AbortOnPoll {
        abort_on: usize,
        polls: AtomicUsize,
    }

    impl AbortSignal for AbortOnPoll {
        fn is_aborted(&self) -> bool {
            self.polls.fetch_add(1, Ordering::Relaxed) + 1 >= self.abort_on
        }
    }

    #[test]
    fn run_config_rejects_undersized_exact_grid() {
        let config = HbRunConfig {
            tones: vec![HbToneRunConfig::new(1.0e6, 3)],
            collocation_points: Some(5),
            ..HbRunConfig::default()
        };

        let err = config.validate().expect_err("grid is undersized");
        assert!(err.contains("at least 7 points"));
    }

    #[test]
    fn hb_validation_observes_abort_while_visiting_tones() {
        let config = HbRunConfig {
            tones: vec![
                HbToneRunConfig::new(1.0e6, 3),
                HbToneRunConfig::new(2.0e6, 3),
                HbToneRunConfig::new(3.0e6, 3),
            ],
            ..HbRunConfig::default()
        };
        let abort = AbortOnPoll {
            abort_on: 3,
            polls: AtomicUsize::new(0),
        };

        let result = config.validate_with_abort(&abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn cancellation_precedes_invalid_hb_configuration() {
        let config = HbRunConfig {
            tones: Vec::new(),
            ..HbRunConfig::default()
        };
        let abort = AbortOnPoll {
            abort_on: 2,
            polls: AtomicUsize::new(0),
        };

        let result = run_hb_analysis_with_abort("invalid", &config, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }
}
