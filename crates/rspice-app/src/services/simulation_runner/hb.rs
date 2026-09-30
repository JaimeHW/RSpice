//! Harmonic balance analysis.
//!
//! Solves for the periodic steady state directly in the frequency domain,
//! which is what makes strongly nonlinear RF circuits tractable where a
//! transient run to steady state would not be.

#![allow(clippy::type_complexity)]

use super::{
    ServiceRunError, ServiceRunResult, build_resolved_periodic_engine,
    parse_runner_netlist_with_abort,
};
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
use rspice_simulation::error::ensure_not_aborted;
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

use rspice_simulation::periodic::build_core_hb_config;
pub use rspice_simulation::periodic::{HbRunConfig, HbToneRunConfig};

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
#[allow(
    dead_code,
    reason = "retained source-path HB adapter for callers and tests"
)]
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
