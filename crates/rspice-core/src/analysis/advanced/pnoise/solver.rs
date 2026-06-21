//! PNoise Solver
//!
//! Phase noise solver implementing:
//! - Time-varying linearization around periodic operating point
//! - Floquet mode decomposition
//! - Noise source projection onto phase/amplitude
//! - Integration for RMS jitter

use super::config::PnoiseConfig;
use super::floquet::FloquetAnalyzer;
use super::result::{NoiseContributor, PhaseNoisePoint, PnoiseResult};
use crate::Value;
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
        if !self.state.carrier_freq.is_finite() || self.state.carrier_freq <= 0.0 {
            return Err(PnoiseError::InvalidCarrier);
        }
        if !self.state.period.is_finite() || self.state.period <= 0.0 {
            return Err(PnoiseError::InvalidCarrier);
        }

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

        let floquet = self
            .state
            .floquet
            .as_ref()
            .ok_or(PnoiseError::NoPssSolution)?;
        if !floquet.is_initialized() {
            return Err(PnoiseError::MissingLinearization);
        }

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
    /// Invalid carrier/reference frequency
    InvalidCarrier,
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
    /// Floquet linearization has not been computed
    MissingLinearization,
}

impl std::fmt::Display for PnoiseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCarrier => write!(f, "Invalid carrier frequency"),
            Self::NoNodes => write!(f, "No nodes in circuit"),
            Self::NoNoiseSources => write!(f, "No noise sources defined"),
            Self::InvalidSweep => write!(f, "Invalid offset frequency sweep"),
            Self::FloquetFailed => write!(f, "Floquet analysis failed"),
            Self::NoPssSolution => write!(f, "No periodic steady-state solution available"),
            Self::MissingLinearization => write!(f, "Missing Floquet linearization"),
        }
    }
}

impl std::error::Error for PnoiseError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn solver_with_noise(carrier_freq: Value) -> PnoiseSolver {
        let config = PnoiseConfig::new("out", 1.0e3, 1.0e5);
        let mut solver = PnoiseSolver::new(config, carrier_freq, 1);
        solver
            .state_mut()
            .add_device_noise(DeviceNoise::thermal("R1", vec![0], 1.0e3, 300.0));
        solver
    }

    #[test]
    fn compute_rejects_missing_pss_or_floquet_data() {
        let mut solver = solver_with_noise(1.0e6);

        let err = solver
            .compute()
            .expect_err("pnoise must not fabricate Floquet data");

        assert_eq!(err, PnoiseError::NoPssSolution);
    }

    #[test]
    fn compute_rejects_invalid_carrier_frequency_without_panicking() {
        for carrier in [0.0, -1.0, Value::NAN, Value::INFINITY] {
            let mut solver = solver_with_noise(carrier);

            let err = solver
                .compute()
                .expect_err("invalid carrier frequency must be rejected");

            assert_eq!(err, PnoiseError::InvalidCarrier);
        }
    }
}

// =============================================================================
// Tests
// =============================================================================
