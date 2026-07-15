use super::error::{ensure_not_aborted, poll_periodically};
use super::{
    ServiceRunError, ServiceRunResult, TransientData,
    run_transient_analysis_with_source_path_and_abort,
};
use num_complex::Complex64;
use rspice_core::Value;
use rspice_core::abort_signal::{AbortSignal, NoAbort};
use std::path::Path;

/// Configuration for envelope analysis.
#[derive(Debug, Clone)]
pub struct EnvelopeRunConfig {
    pub fundamental_freq: Value,
    pub stop_time: Value,
    pub num_harmonics: usize,
    pub max_step: Option<Value>,
}

impl EnvelopeRunConfig {
    fn validate(&self) -> Result<(), String> {
        if !self.fundamental_freq.is_finite() || self.fundamental_freq <= 0.0 {
            return Err("Envelope fundamental frequency must be positive".to_string());
        }
        if !self.stop_time.is_finite() || self.stop_time <= 0.0 {
            return Err("Envelope stop_time must be positive".to_string());
        }
        if self.num_harmonics == 0 {
            return Err("Envelope num_harmonics must be > 0".to_string());
        }
        if let Some(step) = self.max_step
            && (!step.is_finite() || step <= 0.0)
        {
            return Err("Envelope max_step must be positive when provided".to_string());
        }
        Ok(())
    }
}

/// Envelope analysis output.
#[derive(Debug, Clone)]
pub struct EnvelopeData {
    pub time: Vec<Value>,
    pub waveforms: Vec<(String, Vec<Value>)>,
}

/// Run envelope analysis by post-processing transient data with a sliding RMS demodulation.
pub fn run_envelope_analysis(
    netlist_text: &str,
    config: &EnvelopeRunConfig,
) -> Result<EnvelopeData, String> {
    run_envelope_analysis_with_abort(netlist_text, config, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run envelope analysis with cooperative cancellation.
pub fn run_envelope_analysis_with_abort(
    netlist_text: &str,
    config: &EnvelopeRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<EnvelopeData> {
    run_envelope_analysis_with_source_path_and_abort(netlist_text, config, None, abort)
}

/// Run envelope analysis with a source path used to resolve relative includes
/// and model file references.
pub fn run_envelope_analysis_with_source_path(
    netlist_text: &str,
    config: &EnvelopeRunConfig,
    source_path: Option<&Path>,
) -> Result<EnvelopeData, String> {
    run_envelope_analysis_with_source_path_and_abort(netlist_text, config, source_path, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run envelope analysis with source-path resolution and cooperative
/// cancellation through transient solving and RMS demodulation.
pub fn run_envelope_analysis_with_source_path_and_abort(
    netlist_text: &str,
    config: &EnvelopeRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<EnvelopeData> {
    ensure_not_aborted(abort)?;
    let validation = config.validate();
    ensure_not_aborted(abort)?;
    validation.map_err(ServiceRunError::Failure)?;

    let samples_per_cycle = (config.num_harmonics.max(1) as f64 * 16.0).max(32.0);
    let carrier_step = 1.0 / (config.fundamental_freq * samples_per_cycle);
    let coarse_step = config.stop_time / 1200.0;
    let fine_floor = config.stop_time / 100_000.0;
    let mut step_time = config
        .max_step
        .unwrap_or_else(|| carrier_step.min(coarse_step).max(fine_floor));
    step_time = step_time.clamp(fine_floor, config.stop_time);
    if !step_time.is_finite() || step_time <= 0.0 {
        return Err(ServiceRunError::Failure(
            "Envelope sampling step could not be represented".to_string(),
        ));
    }
    ensure_not_aborted(abort)?;

    let transient = run_transient_analysis_with_source_path_and_abort(
        netlist_text,
        config.stop_time,
        step_time,
        source_path,
        abort,
    )?;
    if transient.time.is_empty() {
        return Err(ServiceRunError::Failure(
            "Envelope analysis produced no transient samples".to_string(),
        ));
    }
    if transient.voltages.is_empty() {
        return Err(ServiceRunError::Failure(
            "Envelope analysis found no non-ground node waveforms".to_string(),
        ));
    }

    let cycle_window = (1.0 / config.fundamental_freq / step_time).round().max(3.0) as usize;
    let mut waveforms = Vec::with_capacity(transient.voltages.len());
    for (name, values) in transient.voltages {
        ensure_not_aborted(abort)?;
        if values.is_empty() {
            continue;
        }
        let env = compute_envelope_rms_with_abort(&values, cycle_window, abort)?;
        waveforms.push((format!("ENV({})", name), env));
    }
    if waveforms.is_empty() {
        return Err(ServiceRunError::Failure(
            "Envelope analysis produced no envelope traces".to_string(),
        ));
    }

    ensure_not_aborted(abort)?;
    Ok(EnvelopeData {
        time: transient.time,
        waveforms,
    })
}

fn compute_envelope_rms_with_abort(
    values: &[Value],
    window: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Value>> {
    ensure_not_aborted(abort)?;
    if values.is_empty() {
        return Ok(Vec::new());
    }
    let window = window.max(3).min(values.len());
    let half = window / 2;
    let mut prefix_sq = Vec::with_capacity(values.len() + 1);
    prefix_sq.push(0.0);
    for (sample_idx, &sample) in values.iter().enumerate() {
        poll_periodically(abort, sample_idx)?;
        let next = prefix_sq.last().copied().unwrap_or(0.0) + sample * sample;
        prefix_sq.push(next);
    }

    let mut envelope = Vec::with_capacity(values.len());
    for idx in 0..values.len() {
        poll_periodically(abort, idx)?;
        let start = idx.saturating_sub(half);
        let end = (idx + half + 1).min(values.len());
        let denom = (end - start).max(1) as Value;
        let mean_sq = (prefix_sq[end] - prefix_sq[start]) / denom;
        envelope.push((2.0 * mean_sq).sqrt());
    }
    ensure_not_aborted(abort)?;
    Ok(envelope)
}

/// Configuration for Fourier analysis.
#[derive(Debug, Clone)]
pub struct FourierRunConfig {
    pub fundamental_freq: Value,
    pub num_harmonics: usize,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub start_time: Value,
    pub stop_time: Value,
}

impl FourierRunConfig {
    fn validate(&self) -> Result<(), String> {
        if !self.fundamental_freq.is_finite() || self.fundamental_freq <= 0.0 {
            return Err("Fourier fundamental frequency must be positive".to_string());
        }
        if self.num_harmonics == 0 {
            return Err("Fourier num_harmonics must be greater than zero".to_string());
        }
        if self.output_node.trim().is_empty() {
            return Err("Fourier output node must be specified".to_string());
        }
        if !self.start_time.is_finite() || self.start_time < 0.0 {
            return Err("Fourier start_time must be >= 0".to_string());
        }
        if !self.stop_time.is_finite() || self.stop_time <= self.start_time {
            return Err("Fourier stop_time must be greater than start_time".to_string());
        }
        Ok(())
    }
}

/// Fourier analysis output.
#[derive(Debug, Clone)]
pub struct FourierData {
    pub frequencies: Vec<Value>,
    pub response: Vec<Complex64>,
    pub thd_percent: Value,
    pub dc_component: Value,
    pub output_label: String,
}

/// Run Fourier analysis by executing transient and computing harmonic decomposition.
pub fn run_fourier_analysis(
    netlist_text: &str,
    config: &FourierRunConfig,
) -> Result<FourierData, String> {
    run_fourier_analysis_with_abort(netlist_text, config, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run Fourier analysis with cooperative cancellation.
pub fn run_fourier_analysis_with_abort(
    netlist_text: &str,
    config: &FourierRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<FourierData> {
    run_fourier_analysis_with_source_path_and_abort(netlist_text, config, None, abort)
}

/// Run Fourier analysis with a source path used to resolve relative includes
/// and model file references.
pub fn run_fourier_analysis_with_source_path(
    netlist_text: &str,
    config: &FourierRunConfig,
    source_path: Option<&Path>,
) -> Result<FourierData, String> {
    run_fourier_analysis_with_source_path_and_abort(netlist_text, config, source_path, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run Fourier analysis with source-path resolution and cooperative
/// cancellation through transient solving and harmonic integration.
pub fn run_fourier_analysis_with_source_path_and_abort(
    netlist_text: &str,
    config: &FourierRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<FourierData> {
    ensure_not_aborted(abort)?;
    let validation = config.validate();
    ensure_not_aborted(abort)?;
    validation.map_err(ServiceRunError::Failure)?;

    let window = config.stop_time - config.start_time;
    let max_harmonic_freq = config.fundamental_freq * (config.num_harmonics as f64 + 1.0);
    let nyquist_oversample = 8.0;
    let fine_step = 1.0 / (max_harmonic_freq * nyquist_oversample);
    let coarse_step = window / 1500.0;
    let floor_step = window / 200_000.0;
    let step_time = fine_step
        .min(coarse_step)
        .max(floor_step)
        .min(config.stop_time);
    if !step_time.is_finite() || step_time <= 0.0 {
        return Err(ServiceRunError::Failure(
            "Fourier sampling step could not be represented".to_string(),
        ));
    }
    ensure_not_aborted(abort)?;

    let transient = run_transient_analysis_with_source_path_and_abort(
        netlist_text,
        config.stop_time,
        step_time,
        source_path,
        abort,
    )?;
    if transient.time.len() < 3 {
        return Err(ServiceRunError::Failure(
            "Fourier analysis requires at least 3 transient samples".to_string(),
        ));
    }

    let signal = extract_transient_signal_with_abort(
        &transient,
        &config.output_node,
        config.output_ref.as_deref(),
        abort,
    )?;
    let mut window_time = Vec::new();
    let mut window_values = Vec::new();
    for (sample_idx, (&time, &value)) in transient.time.iter().zip(signal.iter()).enumerate() {
        poll_periodically(abort, sample_idx)?;
        if time >= config.start_time && time <= config.stop_time {
            window_time.push(time);
            window_values.push(value);
        }
    }
    if window_time.len() < 3 {
        return Err(ServiceRunError::Failure(
            "Fourier analysis window has insufficient samples".to_string(),
        ));
    }

    let decomposition = analyze_fourier_with_abort(
        &window_time,
        &window_values,
        config.fundamental_freq,
        config.num_harmonics,
        abort,
    )?;
    let output_label = if let Some(ref_node) = config.output_ref.as_deref() {
        if ref_node.trim().is_empty() || ref_node.eq_ignore_ascii_case("0") {
            format!("V({})", config.output_node.trim())
        } else {
            format!("V({}, {})", config.output_node.trim(), ref_node.trim())
        }
    } else {
        format!("V({})", config.output_node.trim())
    };

    ensure_not_aborted(abort)?;
    Ok(FourierData {
        frequencies: decomposition.frequencies,
        response: decomposition.response,
        thd_percent: decomposition.thd_percent,
        dc_component: decomposition.dc_component,
        output_label,
    })
}

struct FourierDecomposition {
    frequencies: Vec<Value>,
    response: Vec<Complex64>,
    thd_percent: Value,
    dc_component: Value,
}

fn analyze_fourier_with_abort(
    time: &[Value],
    values: &[Value],
    fundamental_freq: Value,
    num_harmonics: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<FourierDecomposition> {
    use std::f64::consts::PI;

    ensure_not_aborted(abort)?;
    if time.len() != values.len() || time.len() < 2 {
        return Err(ServiceRunError::Failure(
            "Fourier time/value vectors are inconsistent".to_string(),
        ));
    }

    let t_end = *time.last().expect("length checked");
    let t_start = (t_end - 1.0 / fundamental_freq).max(time[0]);
    let mut start_idx = 0;
    for (idx, sample_time) in time.iter().enumerate() {
        poll_periodically(abort, idx)?;
        if *sample_time >= t_start {
            start_idx = idx;
            break;
        }
    }
    let end_idx = time.len() - 1;
    if end_idx <= start_idx.saturating_add(1) {
        return Err(ServiceRunError::Failure(
            "Fourier analysis period has insufficient samples".to_string(),
        ));
    }
    let time = &time[start_idx..=end_idx];
    let values = &values[start_idx..=end_idx];
    let origin = time[0];
    let duration = time[time.len() - 1] - origin;
    if !duration.is_finite() || duration <= 0.0 {
        return Err(ServiceRunError::Failure(
            "Fourier analysis period has invalid duration".to_string(),
        ));
    }

    let component_count = num_harmonics.checked_add(1).ok_or_else(|| {
        ServiceRunError::Failure("Fourier harmonic count exceeds this platform".to_string())
    })?;
    let mut frequencies = Vec::new();
    frequencies
        .try_reserve_exact(component_count)
        .map_err(|error| {
            ServiceRunError::Failure(format!(
                "Fourier frequency allocation for {component_count} components failed: {error}"
            ))
        })?;
    let mut response = Vec::new();
    response
        .try_reserve_exact(component_count)
        .map_err(|error| {
            ServiceRunError::Failure(format!(
                "Fourier response allocation for {component_count} components failed: {error}"
            ))
        })?;
    let mut harmonic_sum_sq = 0.0;
    let mut fundamental_magnitude = 0.0;
    let mut dc_component = 0.0;

    for harmonic in 0..=num_harmonics {
        ensure_not_aborted(abort)?;
        let frequency = harmonic as Value * fundamental_freq;
        let (real, imaginary) = if harmonic == 0 {
            let mut integral = 0.0;
            for idx in 1..time.len() {
                poll_periodically(abort, idx)?;
                let dt = time[idx] - time[idx - 1];
                integral += 0.5 * (values[idx] + values[idx - 1]) * dt;
            }
            (integral / duration, 0.0)
        } else {
            let omega = 2.0 * PI * frequency;
            let mut cosine_integral = 0.0;
            let mut sine_integral = 0.0;
            for idx in 1..time.len() {
                poll_periodically(abort, idx)?;
                let dt = time[idx] - time[idx - 1];
                let phase0 = omega * (time[idx - 1] - origin);
                let phase1 = omega * (time[idx] - origin);
                cosine_integral +=
                    0.5 * (values[idx - 1] * phase0.cos() + values[idx] * phase1.cos()) * dt;
                sine_integral +=
                    0.5 * (values[idx - 1] * phase0.sin() + values[idx] * phase1.sin()) * dt;
            }
            (
                2.0 * cosine_integral / duration,
                -2.0 * sine_integral / duration,
            )
        };
        let value = Complex64::new(real, imaginary);
        let magnitude = value.norm();
        if harmonic == 0 {
            dc_component = real;
        } else if harmonic == 1 {
            fundamental_magnitude = magnitude;
        } else {
            harmonic_sum_sq += magnitude * magnitude;
        }
        frequencies.push(frequency);
        response.push(value);
    }

    let thd_percent = if fundamental_magnitude > 1e-15 {
        harmonic_sum_sq.sqrt() / fundamental_magnitude * 100.0
    } else {
        0.0
    };
    ensure_not_aborted(abort)?;
    Ok(FourierDecomposition {
        frequencies,
        response,
        thd_percent,
        dc_component,
    })
}

fn extract_transient_signal_with_abort(
    transient: &TransientData,
    output_node: &str,
    output_ref: Option<&str>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Value>> {
    ensure_not_aborted(abort)?;
    let node = output_node.trim();
    if node.is_empty() {
        return Err(ServiceRunError::Failure(
            "Fourier output node is empty".to_string(),
        ));
    }

    let node_waveform =
        find_transient_waveform_with_abort(transient, node, abort)?.ok_or_else(|| {
            ServiceRunError::Failure(format!(
                "Fourier output node '{}' not found in transient data",
                node
            ))
        })?;

    if let Some(ref_name) = output_ref {
        let ref_name = ref_name.trim();
        if ref_name.is_empty() || ref_name.eq_ignore_ascii_case("0") {
            return copy_values_with_abort(node_waveform, abort);
        }
        let ref_waveform = find_transient_waveform_with_abort(transient, ref_name, abort)?
            .ok_or_else(|| {
                ServiceRunError::Failure(format!(
                    "Fourier output reference node '{}' not found in transient data",
                    ref_name
                ))
            })?;
        if ref_waveform.len() != node_waveform.len() {
            return Err(ServiceRunError::Failure(
                "Fourier node/reference waveform length mismatch".to_string(),
            ));
        }
        let mut differential = Vec::with_capacity(node_waveform.len());
        for (sample_idx, (value, reference)) in node_waveform.iter().zip(ref_waveform).enumerate() {
            poll_periodically(abort, sample_idx)?;
            differential.push(value - reference);
        }
        ensure_not_aborted(abort)?;
        return Ok(differential);
    }

    copy_values_with_abort(node_waveform, abort)
}

fn copy_values_with_abort(
    values: &[Value],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Value>> {
    ensure_not_aborted(abort)?;
    let mut copied = Vec::with_capacity(values.len());
    for (sample_idx, value) in values.iter().enumerate() {
        poll_periodically(abort, sample_idx)?;
        copied.push(*value);
    }
    ensure_not_aborted(abort)?;
    Ok(copied)
}

fn find_transient_waveform_with_abort<'a>(
    transient: &'a TransientData,
    node_name: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Option<&'a [Value]>> {
    ensure_not_aborted(abort)?;
    let target = normalize_waveform_node_name(node_name);
    for (name, values) in &transient.voltages {
        ensure_not_aborted(abort)?;
        let wf_node = normalize_waveform_node_name(name);
        if wf_node == target {
            return Ok(Some(values));
        }
    }
    Ok(None)
}

fn normalize_waveform_node_name(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.len() >= 3
        && (trimmed.starts_with("V(") || trimmed.starts_with("v("))
        && trimmed.ends_with(')')
    {
        return trimmed[2..trimmed.len() - 1].trim().to_ascii_uppercase();
    }
    trimmed.to_ascii_uppercase()
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
    fn envelope_rms_polls_inside_large_sample_loops() {
        let abort = AbortOnPoll {
            abort_on: 3,
            polls: AtomicUsize::new(0),
        };
        let values = vec![1.0; 256];

        let result = compute_envelope_rms_with_abort(&values, 16, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn fourier_integration_polls_inside_harmonic_sample_loops() {
        let abort = AbortOnPoll {
            abort_on: 6,
            polls: AtomicUsize::new(0),
        };
        let time: Vec<Value> = (0..256).map(|idx| idx as Value * 1.0e-9).collect();
        let values: Vec<Value> = time
            .iter()
            .map(|time| (2.0 * std::f64::consts::PI * 10.0e6 * time).sin())
            .collect();

        let result = analyze_fourier_with_abort(&time, &values, 10.0e6, 10, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn cancellable_fourier_matches_core_harmonic_decomposition() {
        use rspice_core::analysis::{FourierAnalysis, FourierConfig};

        let fundamental = 10.0e6;
        let time: Vec<Value> = (0..512).map(|idx| idx as Value * 1.0e-9).collect();
        let values: Vec<Value> = time
            .iter()
            .map(|time| {
                0.25 + (2.0 * std::f64::consts::PI * fundamental * time).sin()
                    + 0.1 * (4.0 * std::f64::consts::PI * fundamental * time).cos()
            })
            .collect();
        let expected = FourierAnalysis::new(FourierConfig::new(fundamental).with_harmonics(6))
            .analyze(&time, &values);

        let actual = analyze_fourier_with_abort(&time, &values, fundamental, 6, &NoAbort)
            .expect("cancellable decomposition should succeed");

        assert_eq!(actual.response.len(), expected.harmonics.len());
        assert!((actual.dc_component - expected.dc_component).abs() < 1.0e-12);
        assert!((actual.thd_percent - expected.thd).abs() < 1.0e-9);
        for (value, harmonic) in actual.response.iter().zip(&expected.harmonics) {
            let expected_value =
                Complex64::from_polar(harmonic.magnitude, harmonic.phase.to_radians());
            assert!((value.re - expected_value.re).abs() < 1.0e-10);
            assert!((value.im - expected_value.im).abs() < 1.0e-10);
        }
    }

    #[test]
    fn cancellation_precedes_invalid_fourier_configuration() {
        let config = FourierRunConfig {
            fundamental_freq: -1.0,
            num_harmonics: 0,
            output_node: String::new(),
            output_ref: None,
            start_time: -1.0,
            stop_time: -1.0,
        };
        let abort = AbortOnPoll {
            abort_on: 2,
            polls: AtomicUsize::new(0),
        };

        let result = run_fourier_analysis_with_abort("invalid", &config, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }
}
