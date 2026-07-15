#![allow(clippy::type_complexity)]

use super::error::{ensure_not_aborted, poll_periodically};
use super::{
    ServiceRunError, ServiceRunResult, build_engine_config, parse_runner_netlist_with_abort,
};
use rspice_core::Value;
use rspice_core::abort_signal::{AbortSignal, NoAbort};
use rspice_core::analysis::PssConfig;
use rspice_core::engine::Engine;
use std::path::Path;

/// PSS analysis data
#[derive(Debug, Clone)]
pub struct PssData {
    /// Fundamental period found (seconds)
    pub period: Value,
    /// Fundamental frequency (Hz)
    pub frequency: Value,
    /// Time points within one period
    pub time: Vec<Value>,
    /// Periodic waveforms: (node_name, values)
    pub waveforms: Vec<(String, Vec<Value>)>,
    /// Harmonic content: (node_name, [(frequency, magnitude, phase_deg)])
    pub harmonics: Vec<(String, Vec<(Value, Value, Value)>)>,
    /// Whether solution converged
    pub converged: bool,
    /// Number of cycles to reach steady state
    pub settling_cycles: usize,
}

/// Run PSS analysis
///
/// Finds the periodic steady-state solution of a circuit with autonomous
/// or driven oscillations. Uses the shooting method with Newton iteration.
pub fn run_pss_analysis(
    netlist_text: &str,
    fundamental_freq: Value,
    num_harmonics: usize,
    tolerance: Value,
) -> Result<PssData, String> {
    run_pss_analysis_with_abort(
        netlist_text,
        fundamental_freq,
        num_harmonics,
        tolerance,
        &NoAbort,
    )
    .map_err(|error| error.to_string())
}

/// Run PSS analysis with cooperative cancellation.
pub fn run_pss_analysis_with_abort(
    netlist_text: &str,
    fundamental_freq: Value,
    num_harmonics: usize,
    tolerance: Value,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PssData> {
    run_pss_analysis_with_source_path_and_abort(
        netlist_text,
        fundamental_freq,
        num_harmonics,
        tolerance,
        None,
        abort,
    )
}

/// Run PSS analysis with a source path used to resolve relative includes and
/// model file references.
pub fn run_pss_analysis_with_source_path(
    netlist_text: &str,
    fundamental_freq: Value,
    num_harmonics: usize,
    tolerance: Value,
    source_path: Option<&Path>,
) -> Result<PssData, String> {
    run_pss_analysis_with_source_path_and_abort(
        netlist_text,
        fundamental_freq,
        num_harmonics,
        tolerance,
        source_path,
        &NoAbort,
    )
    .map_err(|error| error.to_string())
}

/// Run PSS analysis with source-path resolution and cooperative cancellation.
pub fn run_pss_analysis_with_source_path_and_abort(
    netlist_text: &str,
    fundamental_freq: Value,
    num_harmonics: usize,
    tolerance: Value,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PssData> {
    ensure_not_aborted(abort)?;
    let validation = validate_pss_parameters(fundamental_freq, num_harmonics, tolerance);
    ensure_not_aborted(abort)?;
    validation.map_err(ServiceRunError::Failure)?;

    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;

    let mut sim_config = build_engine_config(&netlist, None);
    sim_config.tolerance = tolerance;
    let engine = Engine::new(sim_config);

    let pss_config = PssConfig::new(fundamental_freq)
        .with_harmonics(num_harmonics)
        .with_tolerance(tolerance)
        .with_max_iterations(50)
        .with_tstab_periods(10);

    let pss_result = engine
        .run_pss_with_abort(&netlist, pss_config, abort)
        .map_err(|error| ServiceRunError::from_core("PSS error", error))?;

    let period = pss_result.period;
    if !period.is_finite() || period <= 0.0 {
        return Err(ServiceRunError::Failure(
            "PSS solver returned an invalid period".to_string(),
        ));
    }
    let frequency = 1.0 / period;
    let mut time = Vec::with_capacity(pss_result.result.time.len());
    for (sample_idx, sample) in pss_result.result.time.iter().enumerate() {
        poll_periodically(abort, sample_idx)?;
        time.push(*sample);
    }

    let mut waveforms: Vec<(String, Vec<Value>)> = Vec::new();
    let node_names = &pss_result.result.node_names;
    for (idx, waveform) in pss_result.result.waveforms.iter().enumerate() {
        ensure_not_aborted(abort)?;
        let node_name = node_names
            .get(idx)
            .cloned()
            .unwrap_or_else(|| format!("n{}", idx + 1));
        if node_name == "0" || node_name.eq_ignore_ascii_case("gnd") {
            continue;
        }
        let mut values = Vec::with_capacity(waveform.values.len());
        for (sample_idx, sample) in waveform.values.iter().enumerate() {
            poll_periodically(abort, sample_idx)?;
            values.push(*sample);
        }
        waveforms.push((format!("V({})", node_name), values));
    }

    let mut harmonics: Vec<(String, Vec<(Value, Value, Value)>)> = Vec::new();
    for (name, waveform_values) in &waveforms {
        ensure_not_aborted(abort)?;
        let node_harmonics = if waveform_values.is_empty() {
            Vec::new()
        } else {
            compute_fft_harmonics_with_abort(waveform_values, frequency, num_harmonics, abort)?
        };
        harmonics.push((name.clone(), node_harmonics));
    }

    ensure_not_aborted(abort)?;
    Ok(PssData {
        period,
        frequency,
        time,
        waveforms,
        harmonics,
        converged: true,
        settling_cycles: 10,
    })
}

fn validate_pss_parameters(
    fundamental_freq: Value,
    num_harmonics: usize,
    tolerance: Value,
) -> Result<(), String> {
    if !fundamental_freq.is_finite() || fundamental_freq <= 0.0 {
        return Err("PSS fundamental frequency must be positive".to_string());
    }
    if num_harmonics == 0 {
        return Err("PSS num_harmonics must be greater than zero".to_string());
    }
    if !tolerance.is_finite() || tolerance <= 0.0 {
        return Err("PSS tolerance must be positive".to_string());
    }
    Ok(())
}

fn compute_fft_harmonics_with_abort(
    waveform: &[Value],
    fundamental_freq: Value,
    num_harmonics: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<(Value, Value, Value)>> {
    use std::f64::consts::PI;

    ensure_not_aborted(abort)?;
    let n = waveform.len();
    if n == 0 {
        return Ok(Vec::new());
    }

    let harmonic_count = num_harmonics.checked_add(1).ok_or_else(|| {
        ServiceRunError::Failure("PSS harmonic count exceeds this platform".to_string())
    })?;
    let mut harmonics = Vec::new();
    harmonics
        .try_reserve_exact(harmonic_count)
        .map_err(|error| {
            ServiceRunError::Failure(format!(
                "PSS harmonic allocation for {harmonic_count} components failed: {error}"
            ))
        })?;
    let mut dc = 0.0;
    for (sample_idx, sample) in waveform.iter().enumerate() {
        poll_periodically(abort, sample_idx)?;
        dc += *sample;
    }
    dc /= n as Value;
    harmonics.push((0.0, dc, 0.0));

    for harmonic in 1..=num_harmonics {
        ensure_not_aborted(abort)?;
        let freq = fundamental_freq * harmonic as Value;
        let mut real = 0.0;
        let mut imag = 0.0;

        for (sample_idx, &sample) in waveform.iter().enumerate() {
            poll_periodically(abort, sample_idx)?;
            let phase = 2.0 * PI * harmonic as Value * sample_idx as Value / n as Value;
            real += sample * phase.cos();
            imag -= sample * phase.sin();
        }

        real *= 2.0 / n as Value;
        imag *= 2.0 / n as Value;

        let magnitude = (real * real + imag * imag).sqrt();
        let phase_deg = imag.atan2(real).to_degrees();
        harmonics.push((freq, magnitude, phase_deg));
    }

    ensure_not_aborted(abort)?;
    Ok(harmonics)
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
    fn pss_fft_observes_abort_inside_nested_sample_loop() {
        let abort = AbortOnPoll {
            abort_on: 9,
            polls: AtomicUsize::new(0),
        };
        let waveform = vec![1.0; 128];

        let result = compute_fft_harmonics_with_abort(&waveform, 1.0e6, 20, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn cancellation_precedes_invalid_pss_parameters() {
        let abort = AbortOnPoll {
            abort_on: 2,
            polls: AtomicUsize::new(0),
        };

        let result = run_pss_analysis_with_abort("invalid", -1.0, 0, -1.0, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }
}
