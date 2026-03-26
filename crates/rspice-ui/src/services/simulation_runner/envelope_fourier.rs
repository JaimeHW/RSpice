use super::{TransientData, run_transient_analysis};
use num_complex::Complex64;
use rspice_core::Value;
use rspice_core::analysis::{FourierAnalysis, FourierConfig};

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
    config.validate()?;

    let samples_per_cycle = (config.num_harmonics.max(1) as f64 * 16.0).max(32.0);
    let carrier_step = 1.0 / (config.fundamental_freq * samples_per_cycle);
    let coarse_step = config.stop_time / 1200.0;
    let fine_floor = config.stop_time / 100_000.0;
    let mut step_time = config
        .max_step
        .unwrap_or_else(|| carrier_step.min(coarse_step).max(fine_floor));
    step_time = step_time.clamp(fine_floor, config.stop_time);

    let transient = run_transient_analysis(netlist_text, config.stop_time, step_time)?;
    if transient.time.is_empty() {
        return Err("Envelope analysis produced no transient samples".to_string());
    }
    if transient.voltages.is_empty() {
        return Err("Envelope analysis found no non-ground node waveforms".to_string());
    }

    let cycle_window = (1.0 / config.fundamental_freq / step_time).round().max(3.0) as usize;
    let mut waveforms = Vec::with_capacity(transient.voltages.len());
    for (name, values) in transient.voltages {
        if values.is_empty() {
            continue;
        }
        let env = compute_envelope_rms(&values, cycle_window);
        waveforms.push((format!("ENV({})", name), env));
    }
    if waveforms.is_empty() {
        return Err("Envelope analysis produced no envelope traces".to_string());
    }

    Ok(EnvelopeData {
        time: transient.time,
        waveforms,
    })
}

fn compute_envelope_rms(values: &[Value], window: usize) -> Vec<Value> {
    if values.is_empty() {
        return Vec::new();
    }
    let window = window.max(3).min(values.len());
    let half = window / 2;
    let mut prefix_sq = Vec::with_capacity(values.len() + 1);
    prefix_sq.push(0.0);
    for &sample in values {
        let next = prefix_sq.last().copied().unwrap_or(0.0) + sample * sample;
        prefix_sq.push(next);
    }

    let mut envelope = Vec::with_capacity(values.len());
    for idx in 0..values.len() {
        let start = idx.saturating_sub(half);
        let end = (idx + half + 1).min(values.len());
        let denom = (end - start).max(1) as Value;
        let mean_sq = (prefix_sq[end] - prefix_sq[start]) / denom;
        envelope.push((2.0 * mean_sq).sqrt());
    }
    envelope
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
    config.validate()?;

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

    let transient = run_transient_analysis(netlist_text, config.stop_time, step_time)?;
    if transient.time.len() < 3 {
        return Err("Fourier analysis requires at least 3 transient samples".to_string());
    }

    let signal = extract_transient_signal(
        &transient,
        &config.output_node,
        config.output_ref.as_deref(),
    )?;
    let mut window_time = Vec::new();
    let mut window_values = Vec::new();
    for (&time, &value) in transient.time.iter().zip(signal.iter()) {
        if time >= config.start_time && time <= config.stop_time {
            window_time.push(time);
            window_values.push(value);
        }
    }
    if window_time.len() < 3 {
        return Err("Fourier analysis window has insufficient samples".to_string());
    }

    let analysis = FourierAnalysis::new(
        FourierConfig::new(config.fundamental_freq).with_harmonics(config.num_harmonics),
    );
    let result = analysis.analyze(&window_time, &window_values);

    let frequencies: Vec<Value> = result
        .harmonics
        .iter()
        .map(|harmonic| harmonic.frequency)
        .collect();
    let response: Vec<Complex64> = result
        .harmonics
        .iter()
        .map(|harmonic| Complex64::from_polar(harmonic.magnitude, harmonic.phase.to_radians()))
        .collect();
    let output_label = if let Some(ref_node) = config.output_ref.as_deref() {
        if ref_node.trim().is_empty() || ref_node.eq_ignore_ascii_case("0") {
            format!("V({})", config.output_node.trim())
        } else {
            format!("V({}, {})", config.output_node.trim(), ref_node.trim())
        }
    } else {
        format!("V({})", config.output_node.trim())
    };

    Ok(FourierData {
        frequencies,
        response,
        thd_percent: result.thd,
        dc_component: result.dc_component,
        output_label,
    })
}

fn extract_transient_signal(
    transient: &TransientData,
    output_node: &str,
    output_ref: Option<&str>,
) -> Result<Vec<Value>, String> {
    let node = output_node.trim();
    if node.is_empty() {
        return Err("Fourier output node is empty".to_string());
    }

    let node_waveform = find_transient_waveform(transient, node)
        .ok_or_else(|| format!("Fourier output node '{}' not found in transient data", node))?;

    if let Some(ref_name) = output_ref {
        let ref_name = ref_name.trim();
        if ref_name.is_empty() || ref_name.eq_ignore_ascii_case("0") {
            return Ok(node_waveform.to_vec());
        }
        let ref_waveform = find_transient_waveform(transient, ref_name).ok_or_else(|| {
            format!(
                "Fourier output reference node '{}' not found in transient data",
                ref_name
            )
        })?;
        if ref_waveform.len() != node_waveform.len() {
            return Err("Fourier node/reference waveform length mismatch".to_string());
        }
        return Ok(node_waveform
            .iter()
            .zip(ref_waveform.iter())
            .map(|(v, r)| v - r)
            .collect());
    }

    Ok(node_waveform.to_vec())
}

fn find_transient_waveform<'a>(
    transient: &'a TransientData,
    node_name: &str,
) -> Option<&'a [Value]> {
    let target = normalize_waveform_node_name(node_name);
    transient.voltages.iter().find_map(|(name, values)| {
        let wf_node = normalize_waveform_node_name(name);
        (wf_node == target).then_some(values.as_slice())
    })
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
