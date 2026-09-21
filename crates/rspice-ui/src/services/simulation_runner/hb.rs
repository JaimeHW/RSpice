//! Harmonic balance analysis.
//!
//! Solves for the periodic steady state directly in the frequency domain,
//! which is what makes strongly nonlinear RF circuits tractable where a
//! transient run to steady state would not be.

#![allow(clippy::type_complexity)]

use super::error::ensure_not_aborted;
use super::{
    ServiceRunError, ServiceRunResult, build_multi_tone_hb_layout_with_abort,
    build_resolved_periodic_engine, parse_runner_netlist_with_abort,
};
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
#[cfg(test)]
use rspice_core::abort_signal::NoAbort;
use std::collections::HashSet;
use std::path::Path;
/// One displayed spectrum in the core result's peak-amplitude convention.
#[derive(Debug, Clone)]
pub struct HbSpectrum {
    pub name: String,
    pub unit: &'static str,
    pub frequencies: Vec<Value>,
    pub coefficients: Vec<num_complex::Complex64>,
}

/// Harmonic Balance analysis data
#[derive(Debug, Clone)]
pub struct HbData {
    /// DC operating point voltages
    pub dc_voltages: Vec<(String, Value)>,
    /// Node voltages, branch currents, and device leads on the solved harmonic grid.
    pub spectra: Vec<HbSpectrum>,
    /// Exact converged state retained for HB-dependent analyses.
    pub operating_point: std::sync::Arc<rspice_core::engine::HbOperatingPoint>,
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
}

/// Harmonic Balance run configuration passed from the simulation pipeline.
#[derive(Debug, Clone)]
pub struct HbRunConfig {
    pub tones: Vec<HbToneRunConfig>,
    pub reltol: Value,
    pub abstol: Value,
    pub max_iterations: usize,
    pub damping: Value,
    pub min_damping: Value,
    pub oversample: usize,
    pub collocation_points: Option<usize>,
    pub max_mixing_order: usize,
    pub use_krylov: bool,
    pub gmres_restart: usize,
    pub source_stepping: bool,
    pub use_exact_jacobian: bool,
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
            min_damping: 0.01,
            oversample: 2,
            collocation_points: None,
            max_mixing_order: 5,
            use_krylov: false,
            gmres_restart: 30,
            source_stepping: false,
            use_exact_jacobian: true,
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
        if !self.min_damping.is_finite()
            || self.min_damping <= 0.0
            || self.min_damping > self.damping
        {
            return Err(ServiceRunError::Failure(
                "HB min_damping must be in (0, damping]".to_string(),
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

/// Run Harmonic Balance analysis with cooperative cancellation.
///
/// Test-only. The shipping path is
/// [`run_hb_analysis_with_source_path_and_abort`], reached from
/// `simulation::runner::spec::periodic`.
#[cfg(test)]
pub fn run_hb_analysis_with_abort(
    netlist_text: &str,
    config: &HbRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<HbData> {
    run_hb_analysis_with_source_path_and_abort(netlist_text, config, None, abort)
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
    ensure_not_aborted(abort)?;
    config.validate_with_abort(abort)?;
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    run_hb_analysis_on_materialized_with_abort(&netlist, config, abort)
}

/// Solve the already varied study circuit without replaying its nominal source.
pub(crate) fn run_hb_analysis_on_materialized_with_abort(
    netlist: &rspice_core::Netlist,
    config: &HbRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<HbData> {
    ensure_not_aborted(abort)?;
    run_hb_analysis_with_dc_seed_on_materialized_with_abort(netlist, config, None, abort)
}

/// Apply a bound OP seed while preserving an explicitly authored zero start.
pub(crate) fn run_hb_analysis_with_dc_seed_on_materialized_with_abort(
    netlist: &rspice_core::Netlist,
    config: &HbRunConfig,
    seed: Option<&rspice_core::engine::PeriodicDcOperatingPointSeed>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<HbData> {
    ensure_not_aborted(abort)?;
    let hb_config = build_core_hb_config(config, abort)?;
    let engine = build_resolved_periodic_engine(
        netlist,
        config.reltol,
        "HB resolved engine configuration is invalid",
    )?;
    // Run actual HB analysis
    let seed = seed.filter(|_| {
        netlist.options.hb_time_domain_mode
            != Some(rspice_core::netlist::XyceHbTimeDomainMode::Direct)
    });
    let hb_result = match seed {
        Some(seed) => engine.run_hb_with_dc_seed_and_abort(netlist, hb_config, seed, abort),
        None => engine.run_hb_with_abort(netlist, hb_config, abort),
    }
    .map_err(|error| ServiceRunError::from_core("HB error", error))?;
    validate_hb_solution(&hb_result)?;

    // Extract DC operating point from spectral data
    let mut dc_voltages = Vec::with_capacity(hb_result.result.spectral_voltages.len());
    for sv in &hb_result.result.spectral_voltages {
        ensure_not_aborted(abort)?;
        let dc_val = sv.coefficients[0].re;
        dc_voltages.push((sv.node_name.clone(), dc_val));
    }

    let mut spectra = Vec::new();
    for voltage in &hb_result.result.spectral_voltages {
        ensure_not_aborted(abort)?;
        spectra.push(HbSpectrum {
            name: format!("V({})", voltage.node_name),
            unit: "V",
            frequencies: voltage.frequencies.clone(),
            coefficients: voltage.coefficients.clone(),
        });
    }
    let mut current_names = HashSet::new();
    for branch in &hb_result.result.mna_branch_currents {
        ensure_not_aborted(abort)?;
        current_names.insert(branch.device_name.to_ascii_lowercase());
        spectra.push(HbSpectrum {
            name: format!("I({})", branch.device_name),
            unit: "A",
            frequencies: branch.frequencies.clone(),
            coefficients: branch.coefficients.clone(),
        });
    }
    for reactive in &hb_result.result.reactive_spectra {
        ensure_not_aborted(abort)?;
        // An inductor's exact MNA current is already present. A legacy state
        // without an exact DC current cannot supply a complete current trace.
        if !reactive.dc_current_is_exact
            || !current_names.insert(reactive.device_name.to_ascii_lowercase())
        {
            continue;
        }
        spectra.push(HbSpectrum {
            name: format!("I({})", reactive.device_name),
            unit: "A",
            frequencies: hb_result.result.harmonic_frequencies.clone(),
            coefficients: reactive.current_coefficients.clone(),
        });
    }

    for current in &hb_result.device_currents {
        ensure_not_aborted(abort)?;
        spectra.push(HbSpectrum {
            name: current.probe.clone(),
            unit: "A",
            frequencies: hb_result.result.harmonic_frequencies.clone(),
            coefficients: current.coefficients.clone(),
        });
    }

    ensure_not_aborted(abort)?;
    Ok(HbData {
        dc_voltages,
        spectra,
        operating_point: std::sync::Arc::new(hb_result.operating_point),
    })
}

fn validate_hb_solution(analysis: &rspice_core::engine::HbAnalysisResult) -> ServiceRunResult<()> {
    let result = &analysis.result;
    if !analysis.converged || !result.converged || !result.is_valid() {
        return Err(ServiceRunError::Failure(
            "HB engine returned an invalid or unconverged solution".to_owned(),
        ));
    }
    if !analysis.fundamental_freq.is_finite()
        || analysis.fundamental_freq <= 0.0
        || analysis.fundamental_freq.to_bits() != result.fundamental_freq.to_bits()
        || analysis.num_harmonics != result.num_harmonics
    {
        return Err(ServiceRunError::Failure(
            "HB engine returned an inconsistent solved frequency basis".to_owned(),
        ));
    }
    let expected_coefficients = result.num_harmonics.checked_add(1).ok_or_else(|| {
        ServiceRunError::Failure("HB harmonic count overflows the platform".to_owned())
    })?;
    if result.spectral_voltages.is_empty()
        || result.node_names.len() != result.spectral_voltages.len()
        || result.harmonic_frequencies.len() != expected_coefficients
    {
        return Err(ServiceRunError::Failure(
            "HB engine returned an incomplete spectral solution".to_owned(),
        ));
    }
    if result
        .harmonic_frequencies
        .iter()
        .any(|frequency| !frequency.is_finite() || *frequency < 0.0)
        || result
            .harmonic_frequencies
            .windows(2)
            .any(|pair| pair[1] <= pair[0])
    {
        return Err(ServiceRunError::Failure(
            "HB engine returned an invalid harmonic frequency grid".to_owned(),
        ));
    }

    let mut node_names = HashSet::with_capacity(result.node_names.len());
    for (index, (node_name, spectrum)) in result
        .node_names
        .iter()
        .zip(&result.spectral_voltages)
        .enumerate()
    {
        if node_name.trim().is_empty()
            || spectrum.node_name != *node_name
            || !node_names.insert(node_name.trim().to_ascii_lowercase())
        {
            return Err(ServiceRunError::Failure(format!(
                "HB engine returned an invalid node identity at spectrum {}",
                index + 1
            )));
        }
        if spectrum.coefficients.len() != expected_coefficients
            || spectrum.frequencies.len() != expected_coefficients
            || spectrum
                .frequencies
                .iter()
                .zip(&result.harmonic_frequencies)
                .any(|(actual, expected)| actual.to_bits() != expected.to_bits())
        {
            return Err(ServiceRunError::Failure(format!(
                "HB node '{}' returned an inconsistent harmonic basis",
                spectrum.node_name
            )));
        }
        if spectrum
            .coefficients
            .iter()
            .any(|value| !value.re.is_finite() || !value.im.is_finite())
        {
            return Err(ServiceRunError::Failure(format!(
                "HB node '{}' returned a non-finite coefficient",
                spectrum.node_name
            )));
        }
    }
    Ok(())
}

pub(crate) fn build_core_hb_config(
    config: &HbRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<rspice_core::analysis::HbConfig> {
    use rspice_core::analysis::{HbConfig, HbTone};

    config.validate_with_abort(abort)?;
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
    hb_config.min_damping = config.min_damping;
    hb_config.max_mixing_order = config.max_mixing_order;
    hb_config.use_krylov = config.use_krylov;
    hb_config.gmres_restart = config.gmres_restart;
    hb_config.source_stepping = config.source_stepping;
    hb_config.use_exact_jacobian = config.use_exact_jacobian;
    hb_config.verbose = config.verbose;

    ensure_not_aborted(abort)?;
    Ok(hb_config)
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

    /// Every solver control the form authors arrives at the engine's own
    /// configuration, and arrives at its own field.
    ///
    /// Each value is distinct and none is a default, so a control that stopped
    /// at this crate's run configuration, or landed on the neighbouring field
    /// of the same type, is visible here. `build_core_hb_config` is the last
    /// hop: what it returns is what `HbEngine` solves with.
    #[test]
    fn every_hb_solver_control_reaches_the_engine_config() {
        let config = HbRunConfig {
            tones: vec![HbToneRunConfig::new(2.0e9, 6)],
            reltol: 1.0e-7,
            abstol: 1.0e-11,
            max_iterations: 42,
            damping: 0.5,
            min_damping: 0.25,
            oversample: 8,
            collocation_points: Some(37),
            max_mixing_order: 7,
            use_krylov: true,
            gmres_restart: 16,
            source_stepping: true,
            use_exact_jacobian: false,
            verbose: true,
        };

        let core = build_core_hb_config(&config, &NoAbort).expect("the run configuration resolves");

        assert_eq!(core.tolerance, config.reltol);
        assert_eq!(core.abstol, config.abstol);
        assert_eq!(core.max_iterations, config.max_iterations);
        assert_eq!(core.damping, config.damping);
        assert_eq!(core.min_damping, config.min_damping);
        assert_eq!(core.oversample_factor, config.oversample);
        assert_eq!(core.collocation_points, config.collocation_points);
        assert_eq!(core.max_mixing_order, config.max_mixing_order);
        assert_eq!(core.use_krylov, config.use_krylov);
        assert_eq!(core.gmres_restart, config.gmres_restart);
        assert_eq!(core.source_stepping, config.source_stepping);
        assert_eq!(core.use_exact_jacobian, config.use_exact_jacobian);
        assert_eq!(core.verbose, config.verbose);
    }

    /// A floor above the factor it is a floor for is not a configuration the
    /// engine would accept, and the run configuration says so before a solve
    /// is ever prepared.
    #[test]
    fn run_config_rejects_a_damping_floor_above_the_damping_factor() {
        let config = HbRunConfig {
            damping: 0.2,
            min_damping: 0.5,
            ..HbRunConfig::default()
        };

        let err = config.validate().expect_err("the floor exceeds the factor");
        assert!(err.contains("min_damping must be in (0, damping]"), "{err}");
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
