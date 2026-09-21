//! Periodic noise analysis.
//!
//! Noise about a periodic steady state rather than a DC operating point.
//! This is the analysis that gives oscillator phase noise and mixer noise
//! figure, where noise at every sideband folds onto the output.

use super::error::{ensure_not_aborted, poll_periodically};
use super::periodic_carrier::{PeriodicCarrier, PeriodicCarrierState};
use super::{
    ServiceRunError, ServiceRunResult, build_resolved_periodic_engine,
    generate_freq_points_with_abort, is_ground_like,
    netlist_has_independent_source_named_with_abort, parse_runner_netlist_with_abort,
    run_pss_analysis_with_source_path_and_abort,
};
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
#[cfg(test)]
use rspice_core::abort_signal::NoAbort;
use rspice_core::engine::Engine;
use std::fmt;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
enum PnoiseRunError {
    Validation(String),
    Resolution(String),
    Data(String),
}

impl fmt::Display for PnoiseRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Validation(message) | Self::Resolution(message) | Self::Data(message) => {
                f.write_str(message)
            }
        }
    }
}

impl std::error::Error for PnoiseRunError {}

impl From<PnoiseRunError> for ServiceRunError {
    fn from(error: PnoiseRunError) -> Self {
        Self::Failure(error.to_string())
    }
}

/// Frequency sweep type for periodic-noise analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PnoiseFrequencySweep {
    Decade,
    Octave,
    Linear,
}

impl PnoiseFrequencySweep {
    fn keyword(self) -> &'static str {
        match self {
            Self::Decade => "dec",
            Self::Octave => "oct",
            Self::Linear => "lin",
        }
    }
}

/// PNoise noise-reference mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PnoiseReference {
    Output,
    Input,
    Phase,
}

/// Explicit configuration for PNoise execution.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PnoiseRunConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sampling: Option<rspice_core::analysis::pnoise::PeriodicNoiseSampling>,
    pub input_sideband: i32,
    pub output_sideband: i32,
    pub pss_fundamental_freq: Value,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: Value,
    pub start_freq: Value,
    pub stop_freq: Value,
    pub points_per_unit: usize,
    pub sweep: PnoiseFrequencySweep,
    pub max_sideband: i32,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub input_source: String,
    pub noise_ref: PnoiseReference,
    pub integrated_noise: bool,
    pub noise_summary: bool,
    pub reltol: Value,
    pub abstol: Value,
    /// Which periodic solve this run folds noise around; the card's `FROM=`.
    pub carrier: PeriodicCarrier,
}

impl Default for PnoiseRunConfig {
    fn default() -> Self {
        Self {
            sampling: None,
            input_sideband: 0,
            output_sideband: 0,
            pss_fundamental_freq: 1e6,
            pss_num_harmonics: 10,
            pss_tolerance: 1e-3,
            start_freq: 1.0,
            stop_freq: 1e6,
            points_per_unit: 10,
            sweep: PnoiseFrequencySweep::Decade,
            max_sideband: 5,
            output_node: "VOUT".to_string(),
            output_ref: None,
            input_source: "VIN".to_string(),
            noise_ref: PnoiseReference::Output,
            integrated_noise: false,
            noise_summary: true,
            reltol: 1e-3,
            abstol: 1e-18,
            carrier: PeriodicCarrier::Preceding,
        }
    }
}

impl PnoiseRunConfig {
    pub(crate) fn validate_conversion_channels(&self) -> Result<(), String> {
        if let Some(sampling) = &self.sampling {
            sampling.validate()?;
            if self.noise_ref == PnoiseReference::Phase || self.output_sideband != 0 {
                return Err(
                    "Sampled noise requires output or input reference and output sideband zero"
                        .into(),
                );
            }
        }
        super::validate_noise_sidebands(
            self.input_sideband,
            self.output_sideband,
            usize::try_from(self.max_sideband)
                .map_err(|_| "Maximum sideband must be nonnegative")?,
        )?;
        if (self.noise_ref == PnoiseReference::Phase && self.output_sideband != 0)
            || (self.noise_ref != PnoiseReference::Input && self.input_sideband != 0)
        {
            return Err("Conversion sidebands apply to driven noise; input sideband requires input-referred noise".into());
        }
        Ok(())
    }
    fn validate(&self) -> Result<(), PnoiseRunError> {
        if !self.pss_fundamental_freq.is_finite() || self.pss_fundamental_freq <= 0.0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE requires a positive PSS fundamental frequency".to_string(),
            ));
        }
        if self.pss_num_harmonics == 0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE requires at least one PSS harmonic".to_string(),
            ));
        }
        if !self.pss_tolerance.is_finite() || self.pss_tolerance <= 0.0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE requires a positive PSS tolerance".to_string(),
            ));
        }
        if !self.start_freq.is_finite() || self.start_freq <= 0.0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE start frequency must be positive".to_string(),
            ));
        }
        if !self.stop_freq.is_finite() || self.stop_freq < self.start_freq {
            return Err(PnoiseRunError::Validation(
                "PNOISE stop frequency must be >= start frequency".to_string(),
            ));
        }
        if self.points_per_unit == 0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE points per unit must be greater than zero".to_string(),
            ));
        }
        if self.integrated_noise
            && (self.start_freq == self.stop_freq
                || (self.sweep == PnoiseFrequencySweep::Linear && self.points_per_unit <= 2))
        {
            return Err(PnoiseRunError::Validation(
                "PNOISE integrated noise requires at least two distinct frequency points".into(),
            ));
        }
        if self.max_sideband < 0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE max sideband must be non-negative".to_string(),
            ));
        }
        self.validate_conversion_channels()
            .map_err(PnoiseRunError::Validation)?;
        if self.output_node.trim().is_empty() {
            return Err(PnoiseRunError::Validation(
                "PNOISE output node must be specified".to_string(),
            ));
        }
        if !self.reltol.is_finite() || self.reltol <= 0.0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE relative tolerance must be positive".to_string(),
            ));
        }
        if !self.abstol.is_finite() || self.abstol < 0.0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE absolute tolerance must be non-negative".to_string(),
            ));
        }
        // The carrier itself is not a range check: it names a family, and
        // whether the state this run was handed belongs to that family is
        // `PeriodicCarrierState::accepted_by`, asked where both are in hand.
        Ok(())
    }
}

/// PNoise analysis data.
#[derive(Debug, Clone)]
pub struct PnoiseData {
    pub input_quantity: Option<rspice_core::analysis::noise::NoiseInputQuantity>,
    pub conversion: Option<crate::state::PeriodicNoiseConversionEvidence>,
    /// Offset frequencies (Hz).
    pub frequencies: Vec<Value>,
    /// Noise values. Units depend on `reference`:
    /// - Output/Input: V^2/Hz
    /// - Phase: dBc/Hz
    pub output_noise: Vec<Value>,
    /// Optional input-referred noise vector (V²/Hz or A²/Hz, per `input_quantity`).
    pub input_noise: Option<Vec<Value>>,
    /// Device contributors (name, percentage) at the measured output port.
    pub contributors: Vec<(String, Value)>,
    /// Driven output-noise densities in V²/Hz. Phase-mode contributor shares
    /// are scalar percentages, never voltage-density curves.
    pub contributor_spectra: Vec<(String, Vec<Value>)>,
    /// Total output noise over the swept band in volts RMS, when the run was
    /// asked to integrate. `None` means the question was not asked — or that
    /// the reference has no answer in volts, which is the phase-noise case:
    /// its band total is an RMS phase error in radians, and the retained
    /// [`crate::state::NoiseSummary`] states volts.
    pub output_rms: Option<Value>,
    /// Total input-referred noise over the swept band in volts or amperes RMS, on the
    /// same terms.
    pub input_rms: Option<Value>,
    /// Integrated phase error in radians; distinct from the voltage totals.
    pub phase_rms_rad: Option<Value>,
    /// RMS timing displacement, phase RMS divided by the solved angular frequency.
    pub timing_jitter_rms_s: Option<Value>,
}

/// Run PNoise analysis standalone -- computing its own periodic solution
/// rather than receiving one -- with explicit configuration and cancellation.
///
/// Test-only. PNOISE ships as a dependent task through
/// [`run_pnoise_analysis_from_pss_with_source_path_and_abort`].
#[cfg(test)]
pub fn run_pnoise_analysis_with_config_and_abort(
    netlist_text: &str,
    config: &PnoiseRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PnoiseData> {
    run_pnoise_analysis_impl(netlist_text, config, None, None, abort)
}

/// Run PNOISE from an exact retained PSS state while resolving any unsealed
/// direct-call source references relative to `source_path`.
#[cfg(test)]
pub fn run_pnoise_analysis_from_pss_with_source_path_and_abort(
    netlist_text: &str,
    config: &PnoiseRunConfig,
    operating_point: &rspice_core::engine::PssOperatingPoint,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PnoiseData> {
    run_pnoise_analysis_impl(
        netlist_text,
        config,
        source_path,
        Some(PeriodicCarrierState::Shooting(operating_point)),
        abort,
    )
}

/// Run PNOISE from an exact retained harmonic-balance state.
///
/// Driven periodic noise about the other carrier the engine accepts, through
/// `Engine::run_pnoise_from_hb_with_abort`. There is no oscillator arm here:
/// a harmonic-balance orbit's period is its authored tone rather than a solver
/// unknown, so it has no free phase to diffuse, and a phase-referred request
/// is refused before this point by the plan's dependency contract and by the
/// engine's own `check_pnoise_card_carrier`.
pub fn run_pnoise_analysis_from_hb_with_source_path_and_abort(
    netlist_text: &str,
    config: &PnoiseRunConfig,
    operating_point: &rspice_core::engine::HbOperatingPoint,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PnoiseData> {
    run_pnoise_analysis_impl(
        netlist_text,
        config,
        source_path,
        Some(PeriodicCarrierState::HarmonicBalance(operating_point)),
        abort,
    )
}

fn run_pnoise_analysis_impl(
    netlist_text: &str,
    config: &PnoiseRunConfig,
    source_path: Option<&Path>,
    carrier: Option<PeriodicCarrierState<'_>>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PnoiseData> {
    ensure_not_aborted(abort)?;
    config.validate()?;
    if let Some(carrier) = carrier {
        carrier
            .accepted_by(config.carrier, ".PNOISE")
            .map_err(|reason| ServiceRunError::Failure(reason))?;
    }
    ensure_not_aborted(abort)?;

    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    let owned;
    let carrier = match carrier {
        Some(carrier) => carrier,
        None => {
            owned = run_pss_analysis_with_source_path_and_abort(
                netlist_text,
                config.pss_fundamental_freq,
                config.pss_num_harmonics,
                config.pss_tolerance,
                source_path,
                abort,
            )?;
            PeriodicCarrierState::Shooting(&owned.operating_point)
        }
    };
    run_pnoise_analysis_on_materialized_with_abort(&netlist, config, carrier, abort)
}

pub(crate) fn run_pnoise_analysis_on_materialized_with_abort(
    netlist: &rspice_core::Netlist,
    config: &PnoiseRunConfig,
    carrier: PeriodicCarrierState<'_>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PnoiseData> {
    ensure_not_aborted(abort)?;
    config.validate()?;
    carrier
        .accepted_by(config.carrier, ".PNOISE")
        .map_err(ServiceRunError::Failure)?;
    if config.noise_ref == PnoiseReference::Input {
        let source_name = config.input_source.trim();
        if !source_name.is_empty()
            && !netlist_has_independent_source_named_with_abort(&netlist, source_name, abort)?
        {
            return Err(PnoiseRunError::Resolution(format!(
                "PNOISE input source '{}' is not an independent voltage/current source in the netlist",
                source_name
            )).into());
        }
    }

    let engine = build_resolved_periodic_engine(
        &netlist,
        carrier.engine_tolerance(config.pss_tolerance),
        "PNOISE resolved producer configuration is invalid",
    )?;

    let frequencies = generate_freq_points_with_abort(
        config.start_freq,
        config.stop_freq,
        config.points_per_unit,
        config.sweep.keyword(),
        abort,
    )?;

    run_pnoise_from_retained_state(&engine, netlist, config, frequencies, carrier, abort)
}

fn run_pnoise_from_retained_state(
    engine: &Engine,
    netlist: &rspice_core::Netlist,
    config: &PnoiseRunConfig,
    frequencies: Vec<Value>,
    carrier: PeriodicCarrierState<'_>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PnoiseData> {
    if config.integrated_noise && frequencies.len() < 2 {
        return Err(ServiceRunError::Failure(
            "PNOISE integrated noise requires at least two distinct frequency points".into(),
        ));
    }
    // Validates `max_sideband` before the exact solve; the stride itself is
    // applied inside the engine call below.
    validate_pnoise_sideband_count(config.max_sideband, abort)?;
    let output_ref = config
        .output_ref
        .as_deref()
        .map(str::trim)
        .filter(|node| !node.is_empty() && !is_ground_like(node));

    if config.noise_ref == PnoiseReference::Phase {
        // Only a shooting orbit holds its period as an unknown, so only a
        // shooting orbit has a phase that diffuses. A harmonic-balance carrier
        // is refused here in the engine's own terms rather than folded onto
        // the driven arm, which would publish an output spectrum under a
        // phase-noise heading.
        let PeriodicCarrierState::Shooting(operating_point) = carrier else {
            return Err(ServiceRunError::Failure(
                "`.PNOISE NOISEREF=PHASE` needs an autonomous carrier: a harmonic-balance orbit is \
                 driven by its authored tones and has no free phase to diffuse, so author `.PSS \
                 AUTONOMOUS=YES` or ask for output-referred noise"
                    .to_owned(),
            ));
        };
        let mut oscillator = engine
            .run_pnoise_oscillator_from_pss_with_abort(
                netlist,
                operating_point.config().clone(),
                &frequencies,
                operating_point,
                abort,
            )
            .map_err(|error| ServiceRunError::from_core("exact retained-state PNOISE", error))?;
        if config.integrated_noise {
            oscillator.integrate_band().map_err(|error| {
                ServiceRunError::from_core("phase-noise band integration", error)
            })?;
        }
        let phase_rms_rad = oscillator.integrated_phase_noise;
        let timing_jitter_rms_s =
            phase_rms_rad.map(|rms| rms * oscillator.period / std::f64::consts::TAU);
        if timing_jitter_rms_s.is_some_and(|value| !value.is_finite()) {
            return Err(ServiceRunError::Failure(
                "integrated timing jitter is outside the representable range".into(),
            ));
        }
        let contributors = if config.noise_summary {
            contributor_percentages_with_abort(
                &frequencies,
                &oscillator.phase_noise_contributors,
                &oscillator.phase_error_psd,
                abort,
            )?
        } else {
            Vec::new()
        };
        ensure_not_aborted(abort)?;
        return Ok(PnoiseData {
            input_quantity: None,
            contributor_spectra: Vec::new(),
            conversion: None,
            frequencies,
            output_noise: oscillator.phase_noise_dbc,
            input_noise: None,
            contributors,
            phase_rms_rad,
            timing_jitter_rms_s,
            output_rms: None,
            input_rms: None,
        });
    }

    let input_source = (config.noise_ref == PnoiseReference::Input)
        .then(|| config.input_source.trim())
        .filter(|name| !name.is_empty());
    let request = rspice_core::engine::PeriodicNoiseRequest {
        sampling: config.sampling.as_ref(),
        offsets: &frequencies,
        output_node: config.output_node.trim(),
        output_ref,
        input_source,
        max_sideband: config.max_sideband,
        sidebands: rspice_core::engine::PeriodicNoiseSidebands {
            input: config.input_sideband,
            output: config.output_sideband,
        },
    };
    let exact =
        match carrier {
            PeriodicCarrierState::Shooting(operating_point) => engine
                .run_pnoise_from_pss_request_with_abort(netlist, &request, operating_point, abort),
            PeriodicCarrierState::HarmonicBalance(operating_point) => engine
                .run_pnoise_from_hb_request_with_abort(netlist, &request, operating_point, abort),
        }
        .map_err(|error| ServiceRunError::from_core("exact retained-state PNOISE", error))?;

    let timing = exact
        .sampling
        .as_ref()
        .is_some_and(|sampling| sampling.request.is_timing());
    let input_noise = match config.noise_ref {
        PnoiseReference::Input => Some(exact.input_noise.ok_or_else(|| {
            PnoiseRunError::Data(
                "exact retained-state PNOISE did not produce the requested input-referred spectrum"
                    .to_owned(),
            )
        })?),
        PnoiseReference::Output => None,
        PnoiseReference::Phase => unreachable!("phase reference returned through the PPV path"),
    };
    let contributors = if config.noise_summary {
        contributor_percentages_with_abort(
            &frequencies,
            &exact.contributors,
            &exact.output_noise,
            abort,
        )?
    } else {
        Vec::new()
    };
    // Integrate the actual measured quantity: voltage, timing, or input source noise.
    let (output_rms, input_rms) = if config.integrated_noise {
        let output = integrate_psd_power_with_abort(
            &frequencies,
            &exact.output_noise,
            "exact PNOISE output spectrum",
            abort,
        )?
        .sqrt();
        let input = input_noise
            .as_ref()
            .map(|density| {
                integrate_psd_power_with_abort(
                    &frequencies,
                    density,
                    "exact PNOISE input-referred spectrum",
                    abort,
                )
            })
            .transpose()?
            .map(Value::sqrt);
        (Some(output), input)
    } else {
        (None, None)
    };
    ensure_not_aborted(abort)?;
    Ok(PnoiseData {
        input_quantity: exact.input_quantity,
        contributor_spectra: if config.noise_summary {
            exact.contributors
        } else {
            Vec::new()
        },
        conversion: Some(crate::state::PeriodicNoiseConversionEvidence {
            sampling: exact.sampling,
            input_source: input_source.unwrap_or_default().into(),
            carrier_hz: exact.fundamental_freq,
            input_sideband: config.input_sideband,
            output_sideband: config.output_sideband,
            max_sideband: config.max_sideband,
        }),
        frequencies,
        output_noise: exact.output_noise,
        input_noise,
        contributors,
        output_rms: if timing { None } else { output_rms },
        input_rms,
        phase_rms_rad: None,
        timing_jitter_rms_s: if timing { output_rms } else { None },
    })
}

fn validate_pnoise_sideband_count(
    max_sideband: i32,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<()> {
    ensure_not_aborted(abort)?;
    let non_negative = u64::try_from(max_sideband).map_err(|_| {
        ServiceRunError::Failure(format!("PNOISE max sideband '{max_sideband}' is invalid"))
    })?;
    let count = non_negative
        .checked_mul(2)
        .and_then(|value| value.checked_add(1))
        .ok_or_else(|| {
            ServiceRunError::Failure(format!(
                "PNOISE sideband count overflow for max sideband '{max_sideband}'"
            ))
        })?;
    usize::try_from(count).map_err(|_| {
        ServiceRunError::Failure(format!(
            "PNOISE sideband count '{count}' is unsupported on this platform"
        ))
    })?;
    ensure_not_aborted(abort)
}

fn contributor_percentages_with_abort(
    frequencies: &[Value],
    contributors: &[(String, Vec<Value>)],
    output_noise: &[Value],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<(String, Value)>> {
    let total = integrate_psd_power_with_abort(
        frequencies,
        output_noise,
        "exact PNOISE output spectrum",
        abort,
    )?;
    let mut percentages = Vec::with_capacity(contributors.len());
    for (contributor_index, (name, values)) in contributors.iter().enumerate() {
        poll_periodically(abort, contributor_index)?;
        let share = integrate_psd_power_with_abort(
            frequencies,
            values,
            &format!("exact PNOISE contributor '{name}'"),
            abort,
        )?;
        percentages.push((
            name.clone(),
            if total > 0.0 {
                100.0 * share / total
            } else {
                0.0
            },
        ));
    }
    percentages.sort_by(|lhs, rhs| rhs.1.total_cmp(&lhs.1).then_with(|| lhs.0.cmp(&rhs.0)));
    ensure_not_aborted(abort)?;
    Ok(percentages)
}

fn integrate_psd_power_with_abort(
    frequencies: &[Value],
    psd: &[Value],
    label: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Value> {
    ensure_not_aborted(abort)?;
    if frequencies.len() != psd.len() || frequencies.is_empty() {
        return Err(PnoiseRunError::Data(format!(
            "{label} has {} samples for {} frequency points",
            psd.len(),
            frequencies.len()
        ))
        .into());
    }
    if frequencies
        .iter()
        .any(|frequency| !frequency.is_finite() || *frequency < 0.0)
        || psd.iter().any(|value| !value.is_finite() || *value < 0.0)
        || frequencies.windows(2).any(|pair| pair[1] <= pair[0])
    {
        return Err(PnoiseRunError::Data(format!(
            "{label} contains non-finite, negative, or non-monotonic numerical data"
        ))
        .into());
    }
    if frequencies.len() == 1 {
        return Ok(psd[0]);
    }

    let mut power = 0.0;
    for (index, (frequency_pair, psd_pair)) in
        frequencies.windows(2).zip(psd.windows(2)).enumerate()
    {
        poll_periodically(abort, index)?;
        power += 0.5 * (psd_pair[0] + psd_pair[1]) * (frequency_pair[1] - frequency_pair[0]);
    }
    if !power.is_finite() || power < 0.0 {
        return Err(
            PnoiseRunError::Data(format!("{label} integration produced an invalid power")).into(),
        );
    }
    ensure_not_aborted(abort)?;
    Ok(power)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::{CountingAbort, ImmediateAbort};

    #[test]
    fn pnoise_service_preserves_typed_entry_abort() {
        let result = run_pnoise_analysis_with_config_and_abort(
            "not a netlist",
            &PnoiseRunConfig::default(),
            &ImmediateAbort,
        );

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn pnoise_integration_honors_in_loop_abort() {
        let frequencies = (1..257).map(|index| index as Value).collect::<Vec<_>>();
        let psd = vec![1e-18; frequencies.len()];
        let abort = CountingAbort::new(1);

        let result =
            integrate_psd_power_with_abort(&frequencies, &psd, "abort fixture spectrum", &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
        assert!(abort.count() > 1);
    }

    /// Both carriers fold the same noise through the same network.
    ///
    /// The fixture is an RC low-pass driven by one tone. Its small-signal
    /// linearization does not depend on the operating point, so the
    /// cyclostationary fold degenerates: the periodically time-varying system
    /// is time invariant, no sideband couples to any other, and the folded
    /// spectrum is the network's stationary output noise. That number is a
    /// property of the circuit, not of which solver froze the large-signal
    /// state, so the two carriers must coincide.
    ///
    /// The closed form is stated too, not just the coincidence: a single noisy
    /// resistor R feeding C has output PSD `4*k*T*R*|H(f)|^2`, and asserting
    /// against it is what distinguishes "the two agree" from "the two share
    /// one defect". The temperature is the deck's nominal 27 degrees C, and
    /// the band around that value is wide (a factor of 1.05) because this
    /// assertion is about the scale being the physical one — the sharp
    /// statement is the agreement below it, at `1e-9` relative, which is a
    /// round-off budget rather than either solver's convergence tolerance.
    #[test]
    fn pnoise_around_hb_and_pnoise_around_pss_agree_on_a_linear_circuit() {
        use crate::services::simulation_runner::hb::{
            HbRunConfig, HbToneRunConfig, run_hb_analysis_with_abort,
        };

        // R = 1 kOhm, C = 159.154943091895 pF, corner at 1 MHz.
        const DECK: &str = "pnoise carrier agreement fixture\n\
                            V1 in 0 SIN(0 0.001 1Meg)\n\
                            R1 in out 1k\n\
                            C1 out 0 159.154943091895p\n\
                            .end\n";
        const FUNDAMENTAL: Value = 1.0e6;
        const RESISTANCE: Value = 1.0e3;
        const CAPACITANCE: Value = 159.154_943_091_895e-12;
        const HARMONICS: usize = 8;
        const BOLTZMANN: Value = 1.380_649e-23;
        const NOMINAL_KELVIN: Value = 300.15;
        const BOUND: Value = 1.0e-9;
        const SCALE_BAND: Value = 1.05;

        let config = PnoiseRunConfig {
            pss_fundamental_freq: FUNDAMENTAL,
            pss_num_harmonics: HARMONICS,
            pss_tolerance: 1.0e-9,
            start_freq: 1.0e5,
            stop_freq: 1.0e7,
            points_per_unit: 2,
            max_sideband: 1,
            output_node: "out".to_owned(),
            input_source: String::new(),
            noise_summary: false,
            ..PnoiseRunConfig::default()
        };

        let netlist =
            parse_runner_netlist_with_abort(DECK, None, &NoAbort).expect("the deck parses");
        let engine = build_resolved_periodic_engine(&netlist, config.pss_tolerance, "fixture")
            .expect("the fixture engine resolves");
        let shooting = engine
            .run_pss_operating_point_with_abort(
                &netlist,
                rspice_core::analysis::PssConfig::new(FUNDAMENTAL)
                    .with_harmonics(HARMONICS)
                    .with_points_per_period(64)
                    .with_tstab_periods(2)
                    .with_tolerance(config.pss_tolerance),
                &NoAbort,
            )
            .expect("the driven RC orbit converges");
        let harmonic_balance = run_hb_analysis_with_abort(
            DECK,
            &HbRunConfig {
                tones: vec![HbToneRunConfig::new(FUNDAMENTAL, HARMONICS)],
                reltol: 1.0e-10,
                ..HbRunConfig::default()
            },
            &NoAbort,
        )
        .expect("the harmonic-balance carrier converges")
        .operating_point;

        for (sideband, reference) in [
            (0, PnoiseReference::Output),
            (1, PnoiseReference::Output),
            (-1, PnoiseReference::Input),
        ] {
            let mut config = config.clone();
            config.output_sideband = sideband;
            config.noise_ref = reference;
            if reference == PnoiseReference::Input {
                config.input_sideband = sideband;
                config.input_source = "V1".into();
            }
            let from_pss = run_pnoise_analysis_from_pss_with_source_path_and_abort(
                DECK, &config, &shooting, None, &NoAbort,
            )
            .expect("the shooting-carried run completes");
            let from_hb = run_pnoise_analysis_from_hb_with_source_path_and_abort(
                DECK,
                &config,
                harmonic_balance.as_ref(),
                None,
                &NoAbort,
            )
            .expect("the harmonic-balance-carried run completes");

            assert_eq!(from_pss.frequencies, from_hb.frequencies);
            assert!(!from_pss.frequencies.is_empty());
            for (index, frequency) in from_pss.frequencies.iter().copied().enumerate() {
                let frequency = frequency + f64::from(sideband) * FUNDAMENTAL;
                let transfer_power = 1.0
                    / (std::f64::consts::TAU * frequency * RESISTANCE * CAPACITANCE).mul_add(
                        std::f64::consts::TAU * frequency * RESISTANCE * CAPACITANCE,
                        1.0,
                    );
                let thermal = 4.0 * BOLTZMANN * NOMINAL_KELVIN * RESISTANCE * transfer_power;
                for (label, value) in [
                    ("shooting", from_pss.output_noise[index]),
                    ("harmonic balance", from_hb.output_noise[index]),
                ] {
                    assert!(
                        value > thermal / SCALE_BAND && value < thermal * SCALE_BAND,
                        "the {label} carrier reports {value} V^2/Hz at {frequency} Hz, and the \
                     resistor's own thermal spectrum through this network is {thermal}"
                    );
                }
                let between = (from_pss.output_noise[index] - from_hb.output_noise[index]).abs()
                    / thermal.max(Value::MIN_POSITIVE);
                assert!(
                    between <= BOUND,
                    "the two carriers disagree by {between:e} at {frequency} Hz: {} versus {}",
                    from_pss.output_noise[index],
                    from_hb.output_noise[index]
                );
            }
            assert_eq!(from_pss.conversion, from_hb.conversion);
            let channels = from_hb.conversion.as_ref().unwrap();
            assert_eq!(channels.output_sideband, sideband);
            assert_eq!(channels.carrier_hz, FUNDAMENTAL);
            if reference == PnoiseReference::Input {
                let expected = 4.0 * BOLTZMANN * NOMINAL_KELVIN * RESISTANCE;
                for density in from_hb
                    .input_noise
                    .unwrap()
                    .into_iter()
                    .chain(from_pss.input_noise.unwrap())
                {
                    assert!((density / expected - 1.0).abs() < BOUND);
                }
            }
        }
    }

    /// A phase-referred request has no phase to read off a driven orbit,
    /// whichever family froze it.
    #[test]
    fn a_phase_referred_pnoise_run_refuses_a_harmonic_balance_carrier() {
        use crate::services::simulation_runner::hb::{
            HbRunConfig, HbToneRunConfig, run_hb_analysis_with_abort,
        };

        const DECK: &str = "pnoise phase refusal fixture\n\
                            V1 in 0 SIN(0 0.001 1Meg)\n\
                            R1 in out 1k\n\
                            C1 out 0 1n\n\
                            .end\n";
        let harmonic_balance = run_hb_analysis_with_abort(
            DECK,
            &HbRunConfig {
                tones: vec![HbToneRunConfig::new(1.0e6, 8)],
                ..HbRunConfig::default()
            },
            &NoAbort,
        )
        .expect("the harmonic-balance carrier converges")
        .operating_point;

        let error = run_pnoise_analysis_from_hb_with_source_path_and_abort(
            DECK,
            &PnoiseRunConfig {
                pss_fundamental_freq: 1.0e6,
                pss_num_harmonics: 8,
                start_freq: 1.0e3,
                stop_freq: 1.0e5,
                points_per_unit: 2,
                max_sideband: 1,
                output_node: "out".to_owned(),
                input_source: String::new(),
                noise_ref: PnoiseReference::Phase,
                ..PnoiseRunConfig::default()
            },
            harmonic_balance.as_ref(),
            None,
            &NoAbort,
        )
        .expect_err("a driven orbit has no free phase to diffuse");
        let detail = error.to_string();
        assert!(
            detail.contains("NOISEREF=PHASE") && detail.contains("harmonic-balance"),
            "the refusal must name the reference and the carrier: {detail}"
        );
    }

    /// `integratedNoise` reaches the band total it asks for.
    ///
    /// The flag reached `PnoiseRunConfig` and was read by nothing: a run
    /// configured to integrate produced the same result as one configured not
    /// to, and the studio's own checkbox was inert. Now that a deck can state
    /// it, an unread flag would be a card that says something and means
    /// nothing.
    #[test]
    fn an_integrating_pnoise_run_reports_the_band_total_and_a_plain_one_does_not() {
        const DECK: &str = "* pnoise integration fixture\n\
                            V1 in 0 SIN(0 1 1Meg)\n\
                            R1 in out 1k\n\
                            C1 out 0 1n\n\
                            .end\n";
        let base = PnoiseRunConfig {
            pss_fundamental_freq: 1.0e6,
            pss_num_harmonics: 3,
            pss_tolerance: 1.0e-6,
            start_freq: 1.0e3,
            stop_freq: 1.0e5,
            points_per_unit: 3,
            max_sideband: 1,
            output_node: "out".to_owned(),
            input_source: String::new(),
            ..PnoiseRunConfig::default()
        };

        let plain = run_pnoise_analysis_with_config_and_abort(DECK, &base, &NoAbort)
            .expect("the periodic-noise fixture runs");
        assert_eq!(plain.output_rms, None);

        let integrating = run_pnoise_analysis_with_config_and_abort(
            DECK,
            &PnoiseRunConfig {
                integrated_noise: true,
                ..base
            },
            &NoAbort,
        )
        .expect("the periodic-noise fixture runs");
        let total = integrating
            .output_rms
            .expect("an integrating run reports its band total");
        assert!(
            total.is_finite() && total > 0.0,
            "the band total of a noisy resistor is {total}"
        );
        // The spectra themselves are untouched by the request to integrate.
        assert_eq!(integrating.output_noise, plain.output_noise);
    }

    #[test]
    fn retained_oscillator_noise_reports_phase_timing_and_device_shares() {
        let deck = "LC phase reporting\nl1 osc 0 1u\nc1 osc 0 1u\nb1 osc 0 i=-0.051*v(osc)+0.025*v(osc)*v(osc)*v(osc)\ni1 0 osc pulse(0 1 10u 10n 10n 1u 1)\n.options rshunt=1k temp=127\n.end\n";
        let netlist = rspice_core::Netlist::parse(deck).unwrap();
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let pss = engine
            .run_pss_operating_point_with_abort(
                &netlist,
                rspice_core::analysis::PssConfig::autonomous()
                    .with_period_guess(6.3e-6)
                    .with_tstab_periods(30)
                    .with_tolerance(1e-6)
                    .with_max_iterations(60),
                &NoAbort,
            )
            .unwrap();
        let mut config = PnoiseRunConfig {
            noise_ref: PnoiseReference::Phase,
            integrated_noise: true,
            noise_summary: true,
            output_node: "osc".into(),
            ..Default::default()
        };
        // Inside the voltage-noise linewidth, the unwrapped phase continues
        // to diffuse. Jitter must not be integrated from the flattened L(f).
        let offsets = vec![1e-16, 1e-15, 1e-14];
        let reported = run_pnoise_from_retained_state(
            &engine,
            &netlist,
            &config,
            offsets.clone(),
            PeriodicCarrierState::Shooting(&pss),
            &NoAbort,
        )
        .unwrap();
        let phase = reported.phase_rms_rad.unwrap();
        let jitter = reported.timing_jitter_rms_s.unwrap();
        assert!(phase > 0.0 && jitter > 0.0);
        let voltage_equivalent = integrate_psd_power_with_abort(
            &offsets,
            &reported
                .output_noise
                .iter()
                .map(|value| 2.0 * 10.0_f64.powf(*value / 10.0))
                .collect::<Vec<_>>(),
            "bounded voltage spectrum",
            &NoAbort,
        )
        .unwrap()
        .sqrt();
        assert!(
            phase > 100.0 * voltage_equivalent,
            "{phase} vs voltage-equivalent {voltage_equivalent}"
        );
        assert!(
            (jitter / (phase * pss.analysis().result.period / std::f64::consts::TAU) - 1.0).abs()
                < 1e-12
        );
        assert!(
            (reported
                .contributors
                .iter()
                .map(|(_, share)| share)
                .sum::<f64>()
                - 100.0)
                .abs()
                < 1e-8
        );
        assert_eq!(reported.output_rms, None);
        assert_eq!(reported.input_rms, None);
        assert!(reported.contributor_spectra.is_empty());
        config.integrated_noise = false;
        config.noise_summary = false;
        let plain = run_pnoise_from_retained_state(
            &engine,
            &netlist,
            &config,
            offsets,
            PeriodicCarrierState::Shooting(&pss),
            &NoAbort,
        )
        .unwrap();
        assert_eq!(plain.phase_rms_rad, None);
        assert_eq!(plain.timing_jitter_rms_s, None);
        assert!(plain.contributors.is_empty());
        assert_eq!(plain.output_noise, reported.output_noise);
    }

    #[test]
    fn exact_contributor_percentages_integrate_on_the_nonuniform_frequency_axis() {
        let frequencies = vec![1.0, 2.0, 10.0];
        let output = vec![2.0, 2.0, 2.0];
        let contributors = vec![("M1".to_owned(), vec![0.0, 0.0, 2.0])];

        let percentages =
            contributor_percentages_with_abort(&frequencies, &contributors, &output, &NoAbort)
                .expect("valid nonuniform spectra integrate");

        assert_eq!(percentages.len(), 1);
        assert!((percentages[0].1 - 100.0 * 8.0 / 18.0).abs() < 1.0e-12);
    }
}
