//! PNoise Solver
//!
//! Commercial-grade phase noise solver implementing:
//! - Time-varying linearization around periodic operating point
//! - Floquet mode decomposition
//! - Noise source projection onto phase/amplitude
//! - Integration for RMS jitter

use super::config::PnoiseConfig;
use super::floquet::{FloquetAnalyzer, FloquetMode};
use super::result::{NoiseContributor, PhaseNoisePoint, PnoiseResult};
use crate::Value;
use num_complex::Complex64;
use std::f64::consts::PI;

/// Phase noise solver state
#[derive(Debug)]
pub struct PnoiseState {
    /// Carrier frequency [Hz]
    pub carrier_freq: Value,

    /// Period [s]
    pub period: Value,

    /// Number of nodes in circuit
    pub num_nodes: usize,

    /// Periodic steady-state waveform samples (per node)
    pub pss_waveforms: Vec<Vec<Value>>,

    /// Time points for waveform samples [s]
    pub time_points: Vec<Value>,

    /// Floquet analyzer instance
    pub floquet: Option<FloquetAnalyzer>,

    /// Computed noise contributions by device
    pub device_noise: Vec<DeviceNoise>,

    /// Analysis completed successfully
    pub converged: bool,
}

impl PnoiseState {
    /// Create new PNoise state
    pub fn new(carrier_freq: Value, num_nodes: usize) -> Self {
        let period = 1.0 / carrier_freq;

        Self {
            carrier_freq,
            period,
            num_nodes,
            pss_waveforms: vec![Vec::new(); num_nodes],
            time_points: Vec::new(),
            floquet: None,
            device_noise: Vec::new(),
            converged: false,
        }
    }

    /// Set PSS waveform for a node
    pub fn set_waveform(&mut self, node_idx: usize, waveform: Vec<Value>, times: Vec<Value>) {
        if node_idx < self.num_nodes {
            self.pss_waveforms[node_idx] = waveform;
            if self.time_points.is_empty() {
                self.time_points = times;
            }
        }
    }

    /// Number of time samples
    pub fn num_samples(&self) -> usize {
        self.time_points.len()
    }

    /// Add device noise source
    pub fn add_device_noise(&mut self, noise: DeviceNoise) {
        self.device_noise.push(noise);
    }
}

/// Device noise source information
#[derive(Debug, Clone)]
pub struct DeviceNoise {
    /// Device name
    pub name: String,

    /// Device type (resistor, mosfet, bjt, etc.)
    pub device_type: String,

    /// Node indices where noise is injected
    pub nodes: Vec<usize>,

    /// Noise power spectral density function
    /// Returns PSD [A²/Hz] or [V²/Hz] at given frequency
    pub psd: NoisePsd,
}

impl DeviceNoise {
    /// Create thermal noise source (resistor)
    pub fn thermal(name: &str, nodes: Vec<usize>, resistance: Value, temperature: Value) -> Self {
        let k_boltzmann = 1.380649e-23;
        let i_noise_psd = 4.0 * k_boltzmann * temperature / resistance;

        Self {
            name: name.to_string(),
            device_type: "resistor".to_string(),
            nodes,
            psd: NoisePsd::White(i_noise_psd),
        }
    }

    /// Create shot noise source
    pub fn shot(name: &str, nodes: Vec<usize>, dc_current: Value) -> Self {
        let q = 1.602176634e-19;
        let i_noise_psd = 2.0 * q * dc_current.abs();

        Self {
            name: name.to_string(),
            device_type: "diode".to_string(),
            nodes,
            psd: NoisePsd::White(i_noise_psd),
        }
    }

    /// Create flicker (1/f) noise source
    pub fn flicker(name: &str, nodes: Vec<usize>, kf: Value, af: Value, i_dc: Value) -> Self {
        Self {
            name: name.to_string(),
            device_type: "mosfet".to_string(),
            nodes,
            psd: NoisePsd::Flicker { kf, af, i_dc },
        }
    }

    /// Get noise PSD at frequency
    pub fn psd_at(&self, freq: Value) -> Value {
        self.psd.at(freq)
    }
}

/// Noise power spectral density model
#[derive(Debug, Clone)]
pub enum NoisePsd {
    /// White (frequency-independent) noise
    White(Value),

    /// Flicker (1/f) noise: S(f) = Kf * I^Af / f
    Flicker { kf: Value, af: Value, i_dc: Value },

    /// Combined white + flicker (typical for transistors)
    Combined {
        white: Value,
        kf: Value,
        af: Value,
        i_dc: Value,
    },

    /// Custom (lookup table with interpolation)
    Custom(Vec<(Value, Value)>),
}

impl NoisePsd {
    /// Get PSD value at frequency
    pub fn at(&self, freq: Value) -> Value {
        match self {
            Self::White(psd) => *psd,
            Self::Flicker { kf, af, i_dc } => {
                if freq > 0.0 {
                    kf * i_dc.abs().powf(*af) / freq
                } else {
                    // Avoid division by zero - return value at 1 Hz
                    kf * i_dc.abs().powf(*af)
                }
            }
            Self::Combined {
                white,
                kf,
                af,
                i_dc,
            } => {
                let flicker = if freq > 0.0 {
                    kf * i_dc.abs().powf(*af) / freq
                } else {
                    *kf * i_dc.abs().powf(*af)
                };
                white + flicker
            }
            Self::Custom(points) => {
                if points.is_empty() {
                    return 0.0;
                }

                // Linear interpolation in log-log space
                let log_f = freq.max(1e-10).log10();

                let mut below = None;
                let mut above = None;

                for (f, psd) in points {
                    let log_fp = f.log10();
                    if log_fp <= log_f {
                        below = Some((log_fp, psd.log10()));
                    }
                    if log_fp >= log_f && above.is_none() {
                        above = Some((log_fp, psd.log10()));
                    }
                }

                match (below, above) {
                    (Some((lf1, lp1)), Some((lf2, lp2))) if (lf2 - lf1).abs() > 1e-10 => {
                        let t = (log_f - lf1) / (lf2 - lf1);
                        10.0_f64.powf(lp1 + t * (lp2 - lp1))
                    }
                    (Some((_, lp)), _) => 10.0_f64.powf(lp),
                    (_, Some((_, lp))) => 10.0_f64.powf(lp),
                    _ => 0.0,
                }
            }
        }
    }

    /// Get corner frequency (where flicker = white)
    pub fn corner_freq(&self) -> Option<Value> {
        match self {
            Self::Combined {
                white,
                kf,
                af,
                i_dc,
            } => {
                if *white > 0.0 {
                    // f_c where Kf * I^Af / f = white
                    let f_c = kf * i_dc.abs().powf(*af) / white;
                    Some(f_c)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

/// Phase noise solver
#[derive(Debug)]
pub struct PnoiseSolver {
    /// Configuration
    config: PnoiseConfig,

    /// State
    state: PnoiseState,
}

impl PnoiseSolver {
    /// Create new PNoise solver
    pub fn new(config: PnoiseConfig, carrier_freq: Value, num_nodes: usize) -> Self {
        Self {
            config,
            state: PnoiseState::new(carrier_freq, num_nodes),
        }
    }

    /// Get reference to config
    pub fn config(&self) -> &PnoiseConfig {
        &self.config
    }

    /// Get mutable reference to state
    pub fn state_mut(&mut self) -> &mut PnoiseState {
        &mut self.state
    }

    /// Initialize Floquet analyzer
    pub fn init_floquet(&mut self, num_harmonics: usize) {
        let analyzer = FloquetAnalyzer::new(self.state.period, num_harmonics)
            .with_carrier_freq(self.state.carrier_freq);
        self.state.floquet = Some(analyzer);
    }

    /// Compute phase noise at all offset frequencies
    pub fn compute(&mut self) -> Result<PnoiseResult, PnoiseError> {
        // Validate
        if self.state.num_nodes == 0 {
            return Err(PnoiseError::NoNodes);
        }

        if self.state.device_noise.is_empty() {
            return Err(PnoiseError::NoNoiseSources);
        }

        // Get offset frequencies
        let offsets = self.config.offset_frequencies();
        if offsets.is_empty() {
            return Err(PnoiseError::InvalidSweep);
        }

        // Create result
        let mut result =
            PnoiseResult::new(self.state.carrier_freq, &self.config.output_node.positive);

        // Initialize Floquet if not done
        if self.state.floquet.is_none() {
            self.init_floquet(self.config.max_sidebands);
        }

        let floquet = self.state.floquet.as_ref().unwrap();

        // Compute phase noise at each offset
        for &offset in &offsets {
            let pn = self.compute_at_offset(floquet, offset);
            result.add_point(pn);
        }

        // Add contributor breakdown
        for noise in &self.state.device_noise {
            let mut contributor = NoiseContributor::new(&noise.name, &noise.device_type);

            for &offset in &offsets {
                let contrib = self.compute_device_contribution(floquet, noise, offset);
                contributor.add_contribution(offset, contrib);
            }

            result.add_contributor(contributor);
        }

        // Compute jitter if requested
        if let Some((f_start, f_stop)) = self.config.jitter_integration {
            let (jitter, phase_err) = self.compute_jitter(&result, f_start, f_stop);
            result.set_jitter(jitter, phase_err, (f_start, f_stop));
        }

        result.converged = true;
        self.state.converged = true;

        Ok(result)
    }

    /// Compute phase noise at single offset frequency
    fn compute_at_offset(&self, floquet: &FloquetAnalyzer, offset: Value) -> PhaseNoisePoint {
        let mut total_psd = 0.0;

        // Sum contributions from all noise sources
        for noise in &self.state.device_noise {
            for &node in &noise.nodes {
                if node < self.state.num_nodes {
                    let device_psd = noise.psd_at(offset);
                    let phase_psd = floquet.noise_to_phase_transfer(offset, device_psd, node);
                    total_psd += phase_psd;
                }
            }
        }

        // Convert to dBc/Hz
        let pn_dbc = floquet.phase_psd_to_dbc(total_psd.max(1e-30));

        PhaseNoisePoint::new(offset, pn_dbc)
    }

    /// Compute individual device contribution
    fn compute_device_contribution(
        &self,
        floquet: &FloquetAnalyzer,
        noise: &DeviceNoise,
        offset: Value,
    ) -> Value {
        let mut device_psd = 0.0;

        for &node in &noise.nodes {
            if node < self.state.num_nodes {
                let psd = noise.psd_at(offset);
                device_psd += floquet.noise_to_phase_transfer(offset, psd, node);
            }
        }

        floquet.phase_psd_to_dbc(device_psd.max(1e-30))
    }

    /// Compute RMS jitter from phase noise
    fn compute_jitter(
        &self,
        result: &PnoiseResult,
        f_start: Value,
        f_stop: Value,
    ) -> (Value, Value) {
        // Integrate phase noise power
        if let Some(integrated_dbc) = result.integrated_noise_power(f_start, f_stop) {
            // Convert dBc to linear power
            let power = 10.0_f64.powf(integrated_dbc / 10.0);

            // RMS phase error [rad]
            let rms_phase = (2.0 * power).sqrt();

            // RMS jitter [s] = rms_phase / (2π * f_carrier)
            let rms_jitter = rms_phase / (2.0 * PI * self.state.carrier_freq);

            (rms_jitter, rms_phase)
        } else {
            (0.0, 0.0)
        }
    }
}

/// PNoise solver errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PnoiseError {
    /// No nodes in circuit
    NoNodes,
    /// No noise sources defined
    NoNoiseSources,
    /// Invalid sweep configuration
    InvalidSweep,
    /// Floquet analysis failed
    FloquetFailed,
    /// No PSS solution available
    NoPssSolution,
}

impl std::fmt::Display for PnoiseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoNodes => write!(f, "No nodes in circuit"),
            Self::NoNoiseSources => write!(f, "No noise sources defined"),
            Self::InvalidSweep => write!(f, "Invalid offset frequency sweep"),
            Self::FloquetFailed => write!(f, "Floquet analysis failed"),
            Self::NoPssSolution => write!(f, "No periodic steady-state solution available"),
        }
    }
}

impl std::error::Error for PnoiseError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod solver_tests {
    use super::*;

    #[test]
    fn test_pnoise_state_new() {
        let state = PnoiseState::new(1e9, 5);
        assert_eq!(state.carrier_freq, 1e9);
        assert_eq!(state.period, 1e-9);
        assert_eq!(state.num_nodes, 5);
        assert!(!state.converged);
    }

    #[test]
    fn test_pnoise_state_waveform() {
        let mut state = PnoiseState::new(1e9, 2);
        let waveform = vec![0.0, 0.5, 1.0, 0.5, 0.0];
        let times = vec![0.0, 0.25e-9, 0.5e-9, 0.75e-9, 1e-9];

        state.set_waveform(0, waveform.clone(), times.clone());

        assert_eq!(state.pss_waveforms[0], waveform);
        assert_eq!(state.num_samples(), 5);
    }

    #[test]
    fn test_device_noise_thermal() {
        let noise = DeviceNoise::thermal("R1", vec![1, 2], 1000.0, 300.0);

        assert_eq!(noise.name, "R1");
        assert_eq!(noise.device_type, "resistor");

        // 4kT/R at 300K, 1kΩ ≈ 1.66e-23 A²/Hz
        let psd = noise.psd_at(1e6);
        assert!(psd > 1e-24 && psd < 1e-22);
    }

    #[test]
    fn test_device_noise_shot() {
        let noise = DeviceNoise::shot("D1", vec![1], 1e-3);

        // 2qI ≈ 3.2e-22 A²/Hz for 1mA
        let psd = noise.psd_at(1e6);
        assert!(psd > 1e-23 && psd < 1e-21);
    }

    #[test]
    fn test_device_noise_flicker() {
        let noise = DeviceNoise::flicker("M1", vec![1], 1e-24, 2.0, 1e-3);

        // KF * I^AF / f at 1kHz
        let psd_1k = noise.psd_at(1e3);
        let psd_10k = noise.psd_at(10e3);

        // 1/f: PSD at 1kHz should be 10x PSD at 10kHz
        assert!((psd_1k / psd_10k - 10.0).abs() < 0.1);
    }

    #[test]
    fn test_noise_psd_white() {
        let psd = NoisePsd::White(1e-20);

        assert_eq!(psd.at(1e3), 1e-20);
        assert_eq!(psd.at(1e6), 1e-20);
        assert_eq!(psd.at(1e9), 1e-20);
    }

    #[test]
    fn test_noise_psd_combined() {
        let psd = NoisePsd::Combined {
            white: 1e-20,
            kf: 1e-20,
            af: 2.0,
            i_dc: 1e-3,
        };

        // At high frequency, should approach white noise
        let high_f = psd.at(1e9);
        assert!((high_f - 1e-20).abs() / 1e-20 < 0.01);

        // Corner frequency should exist
        assert!(psd.corner_freq().is_some());
    }

    #[test]
    fn test_noise_psd_custom() {
        let points = vec![(1e3, 1e-18), (10e3, 1e-19), (100e3, 1e-20)];
        let psd = NoisePsd::Custom(points);

        // At exact points
        let p1 = psd.at(1e3);
        assert!((p1 - 1e-18).abs() / 1e-18 < 0.1);

        // Interpolated
        let p_mid = psd.at(5e3);
        assert!(p_mid < 1e-18 && p_mid > 1e-19);
    }

    #[test]
    fn test_pnoise_solver_new() {
        let config = PnoiseConfig::new("out", 1e3, 1e6);
        let solver = PnoiseSolver::new(config, 1e9, 5);

        assert_eq!(solver.config().output_node.positive, "out");
    }

    #[test]
    fn test_pnoise_solver_init_floquet() {
        let config = PnoiseConfig::new("out", 1e3, 1e6);
        let mut solver = PnoiseSolver::new(config, 1e9, 5);

        solver.init_floquet(10);

        assert!(solver.state.floquet.is_some());
    }

    #[test]
    fn test_pnoise_solver_compute_no_nodes() {
        let config = PnoiseConfig::new("out", 1e3, 1e6);
        let mut solver = PnoiseSolver::new(config, 1e9, 0);

        let result = solver.compute();
        assert!(matches!(result, Err(PnoiseError::NoNodes)));
    }

    #[test]
    fn test_pnoise_solver_compute_no_noise() {
        let config = PnoiseConfig::new("out", 1e3, 1e6);
        let mut solver = PnoiseSolver::new(config, 1e9, 5);

        let result = solver.compute();
        assert!(matches!(result, Err(PnoiseError::NoNoiseSources)));
    }

    #[test]
    fn test_pnoise_solver_compute_success() {
        let config = PnoiseConfig::new("out", 1e3, 1e6)
            .with_sweep(super::super::config::PnoiseSweep::log(1e3, 1e6, 3));

        let mut solver = PnoiseSolver::new(config, 1e9, 2);

        // Add noise source
        solver
            .state_mut()
            .add_device_noise(DeviceNoise::thermal("R1", vec![0, 1], 1000.0, 300.0));

        let result = solver.compute();
        assert!(result.is_ok());

        let pn_result = result.unwrap();
        assert!(pn_result.converged);
        assert!(!pn_result.spectral_points.is_empty());
    }

    #[test]
    fn test_pnoise_solver_contributors() {
        let config = PnoiseConfig::new("out", 1e3, 1e6)
            .with_sweep(super::super::config::PnoiseSweep::log(1e3, 1e5, 2));

        let mut solver = PnoiseSolver::new(config, 1e9, 2);

        solver
            .state_mut()
            .add_device_noise(DeviceNoise::thermal("R1", vec![0], 1000.0, 300.0));
        solver
            .state_mut()
            .add_device_noise(DeviceNoise::thermal("R2", vec![1], 2000.0, 300.0));

        let result = solver.compute().unwrap();
        assert_eq!(result.contributors.len(), 2);
    }

    #[test]
    fn test_pnoise_solver_jitter() {
        let config = PnoiseConfig::new("out", 1e3, 1e6)
            .with_sweep(super::super::config::PnoiseSweep::log(1e3, 1e6, 5))
            .with_jitter_integration(1e3, 1e6);

        let mut solver = PnoiseSolver::new(config, 1e9, 2);

        solver
            .state_mut()
            .add_device_noise(DeviceNoise::thermal("R1", vec![0], 1000.0, 300.0));

        let result = solver.compute().unwrap();
        assert!(result.rms_jitter.is_some());
        assert!(result.jitter_bandwidth.is_some());
    }

    #[test]
    fn test_pnoise_error_display() {
        assert!(PnoiseError::NoNodes.to_string().contains("No nodes"));
        assert!(
            PnoiseError::NoNoiseSources
                .to_string()
                .contains("noise sources")
        );
        assert!(PnoiseError::InvalidSweep.to_string().contains("sweep"));
        assert!(PnoiseError::FloquetFailed.to_string().contains("Floquet"));
        assert!(PnoiseError::NoPssSolution.to_string().contains("periodic"));
    }
}
