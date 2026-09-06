//! Periodic noise analysis.
//!
//! Noise about a periodic steady state rather than a DC operating point.
//! This is the analysis that gives oscillator phase noise and mixer noise
//! figure, where noise at every sideband folds onto the output.

use super::error::{ensure_not_aborted, poll_periodically};
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PnoiseReference {
    Output,
    Input,
    Phase,
}

/// Explicit configuration for PNoise execution.
#[derive(Debug, Clone)]
pub struct PnoiseRunConfig {
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
}

impl Default for PnoiseRunConfig {
    fn default() -> Self {
        Self {
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
        }
    }
}

impl PnoiseRunConfig {
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
        if self.max_sideband < 0 {
            return Err(PnoiseRunError::Validation(
                "PNOISE max sideband must be non-negative".to_string(),
            ));
        }
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
        Ok(())
    }
}

/// PNoise analysis data.
#[derive(Debug, Clone)]
pub struct PnoiseData {
    /// Offset frequencies (Hz).
    pub frequencies: Vec<Value>,
    /// Noise values. Units depend on `reference`:
    /// - Output/Input: V^2/Hz
    /// - Phase: dBc/Hz
    pub output_noise: Vec<Value>,
    /// Optional input-referred noise vector (V^2/Hz), when available.
    pub input_noise: Option<Vec<Value>>,
    /// Device contributors (name, percentage) at the measured output port.
    pub contributors: Vec<(String, Value)>,
    /// Total output noise over the swept band in volts RMS, when the run was
    /// asked to integrate. `None` means the question was not asked — or that
    /// the reference has no answer in volts, which is the phase-noise case:
    /// its band total is an RMS phase error in radians, and the retained
    /// [`crate::state::NoiseSummary`] states volts.
    pub output_rms: Option<Value>,
    /// Total input-referred noise over the swept band in volts RMS, on the
    /// same terms.
    pub input_rms: Option<Value>,
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
        Some(operating_point),
        abort,
    )
}

fn run_pnoise_analysis_impl(
    netlist_text: &str,
    config: &PnoiseRunConfig,
    source_path: Option<&Path>,
    operating_point: Option<&rspice_core::engine::PssOperatingPoint>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PnoiseData> {
    ensure_not_aborted(abort)?;
    config.validate()?;
    ensure_not_aborted(abort)?;

    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
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
        config.pss_tolerance,
        "PNOISE resolved producer configuration is invalid",
    )?;

    let frequencies = generate_freq_points_with_abort(
        config.start_freq,
        config.stop_freq,
        config.points_per_unit,
        config.sweep.keyword(),
        abort,
    )?;

    if let Some(operating_point) = operating_point {
        return run_pnoise_from_retained_state(
            &engine,
            &netlist,
            config,
            frequencies,
            operating_point,
            abort,
        );
    }

    // Standalone callers solve PSS themselves. They still must publish only
    // exact cyclostationary/PPV results; a stationary sideband fold is not an
    // equivalent PNOISE analysis and is never an execution fallback.
    let pss_data = run_pss_analysis_with_source_path_and_abort(
        netlist_text,
        config.pss_fundamental_freq,
        config.pss_num_harmonics,
        config.pss_tolerance,
        source_path,
        abort,
    )?;

    run_pnoise_from_retained_state(
        &engine,
        &netlist,
        config,
        frequencies,
        &pss_data.operating_point,
        abort,
    )
}

fn run_pnoise_from_retained_state(
    engine: &Engine,
    netlist: &rspice_core::Netlist,
    config: &PnoiseRunConfig,
    frequencies: Vec<Value>,
    operating_point: &rspice_core::engine::PssOperatingPoint,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PnoiseData> {
    // Validates `max_sideband` before the exact solve; the stride itself is
    // applied inside the engine call below.
    validate_pnoise_sideband_count(config.max_sideband, abort)?;
    let output_ref = config
        .output_ref
        .as_deref()
        .map(str::trim)
        .filter(|node| !node.is_empty() && !is_ground_like(node));

    if config.noise_ref == PnoiseReference::Phase {
        let oscillator = engine
            .run_pnoise_oscillator_from_pss_with_abort(
                netlist,
                operating_point.config().clone(),
                &frequencies,
                operating_point,
                abort,
            )
            .map_err(|error| ServiceRunError::from_core("exact retained-state PNOISE", error))?;
        ensure_not_aborted(abort)?;
        return Ok(PnoiseData {
            frequencies,
            output_noise: oscillator.phase_noise_dbc,
            input_noise: None,
            contributors: Vec::new(),
            // A phase-noise band total is an RMS phase error in radians, and
            // the only place this pipeline retains a band total says volts.
            // Reporting radians there would be a mislabelled number, so the
            // total is withheld rather than renamed; the engine's own
            // `.PNOISE` route publishes it as `integrated_phase_noise`.
            output_rms: None,
            input_rms: None,
        });
    }

    let input_source = (config.noise_ref == PnoiseReference::Input)
        .then(|| config.input_source.trim())
        .filter(|name| !name.is_empty());
    let exact = engine
        .run_pnoise_from_pss_with_abort(
            netlist,
            &frequencies,
            config.output_node.trim(),
            output_ref,
            input_source,
            config.max_sideband,
            operating_point,
            abort,
        )
        .map_err(|error| ServiceRunError::from_core("exact retained-state PNOISE", error))?;

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
    // The band totals the card asked for. Both are volts RMS: the driven
    // spectra are power densities in V^2/Hz, so the integral over the swept
    // band is a mean square and its root is the total.
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
        frequencies,
        output_noise: exact.output_noise,
        input_noise,
        contributors,
        output_rms,
        input_rms,
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
