use super::build_engine_config;
use rspice_core::Value;
use rspice_core::analysis::PssConfig;
use rspice_core::engine::Engine;

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
    let netlist = rspice_core::netlist::parse_netlist(netlist_text)
        .map_err(|e| format!("Parse error: {}", e))?;

    let mut sim_config = build_engine_config(&netlist, None);
    sim_config.tolerance = tolerance;
    let engine = Engine::new(sim_config);

    let pss_config = PssConfig::new(fundamental_freq)
        .with_harmonics(num_harmonics)
        .with_tolerance(tolerance)
        .with_max_iterations(50)
        .with_tstab_periods(10);

    let pss_result = engine
        .run_pss(&netlist, pss_config)
        .map_err(|e| format!("PSS error: {}", e))?;

    let period = pss_result.period;
    let frequency = 1.0 / period;
    let time = pss_result.result.time.clone();

    let mut waveforms: Vec<(String, Vec<Value>)> = Vec::new();
    let node_names = &pss_result.result.node_names;
    for (idx, waveform) in pss_result.result.waveforms.iter().enumerate() {
        let node_name = node_names
            .get(idx)
            .cloned()
            .unwrap_or_else(|| format!("n{}", idx + 1));
        if node_name == "0" || node_name.eq_ignore_ascii_case("gnd") {
            continue;
        }
        waveforms.push((format!("V({})", node_name), waveform.values.clone()));
    }

    let mut harmonics: Vec<(String, Vec<(Value, Value, Value)>)> = Vec::new();
    for (name, waveform_values) in &waveforms {
        let node_harmonics = if waveform_values.is_empty() {
            Vec::new()
        } else {
            compute_fft_harmonics(waveform_values, frequency, num_harmonics)
        };
        harmonics.push((name.clone(), node_harmonics));
    }

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

fn compute_fft_harmonics(
    waveform: &[Value],
    fundamental_freq: Value,
    num_harmonics: usize,
) -> Vec<(Value, Value, Value)> {
    use std::f64::consts::PI;

    let n = waveform.len();
    if n == 0 {
        return Vec::new();
    }

    let mut harmonics = Vec::with_capacity(num_harmonics + 1);
    let dc = waveform.iter().sum::<Value>() / n as Value;
    harmonics.push((0.0, dc, 0.0));

    for harmonic in 1..=num_harmonics {
        let freq = fundamental_freq * harmonic as Value;
        let mut real = 0.0;
        let mut imag = 0.0;

        for (sample_idx, &sample) in waveform.iter().enumerate() {
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

    harmonics
}
