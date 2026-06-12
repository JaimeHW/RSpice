use super::{build_engine_config, parse_runner_netlist};
use rspice_core::Value;
use rspice_core::engine::Engine;
use std::path::Path;

/// STB analysis data for feedback loop stability
#[derive(Debug, Clone)]
pub struct StbData {
    /// Frequency points (Hz)
    pub frequencies: Vec<Value>,
    /// Loop gain magnitude (dB)
    pub loop_gain_db: Vec<Value>,
    /// Loop gain phase (degrees)
    pub loop_phase_deg: Vec<Value>,
    /// Phase margin (degrees)
    pub phase_margin: Value,
    /// Gain margin (dB)
    pub gain_margin: Value,
    /// Unity gain frequency (Hz)
    pub unity_gain_freq: Value,
    /// 180-degree phase crossover frequency (Hz)
    pub phase_crossover_freq: Value,
    /// Whether the loop is stable
    pub is_stable: bool,
}

impl StbData {
    /// Calculate stability from loop gain data
    pub fn calculate_stability(
        frequencies: &[Value],
        gain_db: &[Value],
        phase_deg: &[Value],
    ) -> Self {
        let mut unity_gain_freq = 0.0;
        let mut phase_crossover_freq = 0.0;
        let mut phase_at_unity = 0.0;
        let mut gain_at_phase_cross = 0.0;

        for idx in 1..gain_db.len() {
            if gain_db[idx - 1] >= 0.0 && gain_db[idx] < 0.0 {
                let t = -gain_db[idx - 1] / (gain_db[idx] - gain_db[idx - 1]);
                unity_gain_freq =
                    frequencies[idx - 1] + t * (frequencies[idx] - frequencies[idx - 1]);
                phase_at_unity = phase_deg[idx - 1] + t * (phase_deg[idx] - phase_deg[idx - 1]);
                break;
            }
        }

        for idx in 1..phase_deg.len() {
            if (phase_deg[idx - 1] > -180.0 && phase_deg[idx] <= -180.0)
                || (phase_deg[idx - 1] >= -180.0 && phase_deg[idx] < -180.0)
            {
                let t = (-180.0 - phase_deg[idx - 1]) / (phase_deg[idx] - phase_deg[idx - 1]);
                phase_crossover_freq =
                    frequencies[idx - 1] + t * (frequencies[idx] - frequencies[idx - 1]);
                gain_at_phase_cross = gain_db[idx - 1] + t * (gain_db[idx] - gain_db[idx - 1]);
                break;
            }
        }

        let phase_margin = 180.0 + phase_at_unity;
        let gain_margin = -gain_at_phase_cross;
        let is_stable = phase_margin > 0.0 && gain_margin > 0.0;

        Self {
            frequencies: frequencies.to_vec(),
            loop_gain_db: gain_db.to_vec(),
            loop_phase_deg: phase_deg.to_vec(),
            phase_margin,
            gain_margin,
            unity_gain_freq,
            phase_crossover_freq,
            is_stable,
        }
    }
}

/// Run STB (loop stability) analysis
///
/// Measures the loop gain and phase of a feedback system to determine
/// phase margin and gain margin using AC analysis data.
pub fn run_stb_analysis(
    netlist_text: &str,
    probe: &str,
    start_freq: Value,
    stop_freq: Value,
    points_per_decade: usize,
) -> Result<StbData, String> {
    run_stb_analysis_with_source_path(
        netlist_text,
        probe,
        start_freq,
        stop_freq,
        points_per_decade,
        None,
    )
}

/// Run STB (loop stability) analysis with a source path used to resolve
/// relative includes and model file references.
///
/// `probe` names a 0 V voltage source placed in the feedback loop; the
/// engine measures the true loop gain at that break via Tian's
/// double-injection method. An unknown probe is a hard error — there is no
/// meaningful fallback quantity.
pub fn run_stb_analysis_with_source_path(
    netlist_text: &str,
    probe: &str,
    start_freq: Value,
    stop_freq: Value,
    points_per_decade: usize,
    source_path: Option<&Path>,
) -> Result<StbData, String> {
    use rspice_core::analysis::advanced::stb::StbConfig;

    let netlist = parse_runner_netlist(netlist_text, source_path)?;
    let engine = Engine::new(build_engine_config(&netlist, None));

    let stb_config = StbConfig::new()
        .with_sweep(start_freq, stop_freq, points_per_decade)
        .with_probe(probe)
        .with_nyquist(true);

    let analysis = engine
        .run_stb(&netlist, stb_config)
        .map_err(|e| format!("STB analysis error: {}", e))?;

    let result_frequencies = analysis.frequencies;
    let stb_result = analysis.result;

    let loop_gain_db: Vec<Value> = stb_result
        .bode_points
        .iter()
        .map(|point| point.magnitude_db)
        .collect();
    let loop_phase_deg: Vec<Value> = stb_result
        .bode_points
        .iter()
        .map(|point| point.phase_deg)
        .collect();

    Ok(StbData {
        frequencies: result_frequencies,
        loop_gain_db,
        loop_phase_deg,
        phase_margin: stb_result.margins.phase_margin_deg,
        gain_margin: stb_result.margins.gain_margin_db,
        unity_gain_freq: stb_result.margins.unity_gain_bandwidth,
        phase_crossover_freq: stb_result.margins.gain_margin_freq,
        is_stable: stb_result.margins.is_stable(),
    })
}
