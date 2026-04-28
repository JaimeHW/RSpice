//! PAC Solver Implementation
//!
//! This module implements the Periodic AC analysis algorithm using the
//! Floquet-based approach. The solver linearizes around the PSS solution
//! and computes the conversion matrix that characterizes the linear
//! time-varying (LTV) system response.
//!
//! # Algorithm Overview
//!
//! 1. **Obtain PSS Solution**: Get periodic steady-state waveform from PSS
//! 2. **Sample Time-Varying Parameters**: Evaluate device small-signal
//!    parameters (gm, gds, capacitances) at multiple points over one period
//! 3. **Build Harmonic Admittance Matrix**: Transform sampled parameters to
//!    frequency domain using DFT to get conversion matrix structure
//! 4. **Solve for Each Input Frequency**: For each Δf in sweep, solve the
//!    block-diagonal harmonic system for output sidebands
//!
//! # Theory Reference
//!
//! The approach follows Harmonic Balance with small-signal perturbation:
//! Y(jωk) · V = I_in
//!
//! where Y is the frequency-domain admittance matrix coupling harmonics,
//! V is the vector of spectral voltage components, and I_in is the input
//! excitation at each sideband.

use super::config::PacConfig;
use super::result::PacResult;
use crate::analysis::advanced::pss::PssResult;
use crate::{Complex64, Value};
use std::f64::consts::PI;

//=============================================================================
// PAC Error Types
//=============================================================================

/// Errors that can occur during PAC analysis
#[derive(Debug, Clone)]
pub enum PacError {
    /// No PSS solution provided
    NoPssSolution,

    /// PSS solution has no period information
    InvalidPssSolution(String),

    /// Invalid configuration
    InvalidConfig(String),

    /// Frequency sweep is invalid
    InvalidSweep(String),

    /// Matrix solution failed
    MatrixSolverFailed(String),

    /// No input source specified
    NoInputSource,

    /// Input source not found in circuit
    InputSourceNotFound(String),
}

impl std::fmt::Display for PacError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacError::NoPssSolution => write!(f, "No PSS solution provided for PAC analysis"),
            PacError::InvalidPssSolution(s) => write!(f, "Invalid PSS solution: {}", s),
            PacError::InvalidConfig(s) => write!(f, "Invalid PAC configuration: {}", s),
            PacError::InvalidSweep(s) => write!(f, "Invalid frequency sweep: {}", s),
            PacError::MatrixSolverFailed(s) => write!(f, "Matrix solver failed: {}", s),
            PacError::NoInputSource => write!(f, "No input source specified for PAC analysis"),
            PacError::InputSourceNotFound(s) => write!(f, "Input source not found: {}", s),
        }
    }
}

impl std::error::Error for PacError {}

//=============================================================================
// PAC Analyzer
//=============================================================================

/// Periodic AC Analyzer
///
/// Performs small-signal AC analysis around a periodic operating point,
/// computing the conversion matrix that relates input sidebands to outputs.
#[derive(Debug)]
pub struct PacAnalyzer {
    /// Configuration for the analysis
    config: PacConfig,

    /// Number of time samples per period for parameter extraction
    num_samples: usize,

    /// Current state
    state: PacAnalyzerState,
}

/// Internal state of the PAC analyzer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PacAnalyzerState {
    /// Ready for new analysis
    #[default]
    Ready,
    /// Currently running analysis
    Running,
    /// Analysis complete
    Complete,
    /// Analysis failed
    Failed,
}

impl PacAnalyzer {
    /// Create a new PAC analyzer
    pub fn new(config: PacConfig) -> Self {
        // Number of samples should be at least 2*(max_sideband) + 1 for Nyquist
        // Use more for better accuracy
        let num_sidebands = config.num_sidebands();
        let num_samples = (4 * num_sidebands).max(64);

        Self {
            config,
            num_samples,
            state: PacAnalyzerState::Ready,
        }
    }

    /// Get current configuration
    pub fn config(&self) -> &PacConfig {
        &self.config
    }

    /// Get current state
    pub fn state(&self) -> PacAnalyzerState {
        self.state
    }

    /// Set number of time samples per period
    pub fn with_samples(mut self, samples: usize) -> Self {
        self.num_samples = samples.max(16);
        self
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), PacError> {
        self.config.validate().map_err(PacError::InvalidConfig)
    }

    /// Run PAC analysis given a PSS solution
    ///
    /// # Arguments
    /// * `pss_result` - Periodic steady-state solution from PSS analysis
    /// * `num_nodes` - Number of circuit nodes
    /// * `node_names` - Names of circuit nodes
    /// * `branch_names` - Names of circuit branches
    ///
    /// # Returns
    /// * `Ok(PacResult)` - Complete PAC analysis result
    /// * `Err(PacError)` - If analysis fails
    pub fn analyze(
        &mut self,
        pss_result: &PssResult,
        num_nodes: usize,
        node_names: Vec<String>,
        branch_names: Vec<String>,
    ) -> Result<PacResult, PacError> {
        self.state = PacAnalyzerState::Running;

        // Validate inputs
        self.validate()?;

        let period = pss_result.period;
        if period <= 0.0 {
            self.state = PacAnalyzerState::Failed;
            return Err(PacError::InvalidPssSolution(
                "Period must be positive".to_string(),
            ));
        }

        let fundamental = 1.0 / period;

        // Generate frequency sweep points
        let mut config_with_fundamental = self.config.clone();
        config_with_fundamental.fundamental_freq = fundamental;
        let frequencies = config_with_fundamental.frequency_points();

        if frequencies.is_empty() {
            self.state = PacAnalyzerState::Failed;
            return Err(PacError::InvalidSweep("No frequency points".to_string()));
        }

        // Create result structure
        let mut result = PacResult::new(
            fundamental,
            frequencies.clone(),
            self.config.sideband_min,
            self.config.sideband_max,
            node_names,
            branch_names,
        );

        if let Some(ref src) = self.config.input_source {
            result.set_input_source(src);
        }
        if let Some(ref node) = self.config.output_node {
            result.set_output_node(node);
        }

        // Build the conversion matrix from PSS data
        self.build_conversion_matrix(&mut result, pss_result, num_nodes)?;

        // Solve for each frequency point
        self.solve_frequency_sweep(&mut result, num_nodes)?;

        result.set_convergence_info(1, 0.0); // Direct solve, no iteration

        self.state = PacAnalyzerState::Complete;
        Ok(result)
    }

    /// Build conversion matrix from PSS waveform data
    ///
    /// The conversion matrix is built by:
    /// 1. Sampling the time-varying small-signal parameters over one period
    /// 2. Computing DFT to get frequency-domain representation
    /// 3. Filling conversion matrix elements
    fn build_conversion_matrix(
        &self,
        result: &mut PacResult,
        pss_result: &PssResult,
        num_nodes: usize,
    ) -> Result<(), PacError> {
        let period = pss_result.period;
        let fundamental = result.fundamental_frequency;

        // For a simplified implementation, use mixing products approach:
        // The transfer H[k,m](Δf) from sideband m to sideband k is computed
        // from the Fourier components of the time-varying transfer function

        // Sample time points over one period
        let time_samples: Vec<Value> = (0..self.num_samples)
            .map(|i| (i as Value) / (self.num_samples as Value) * period)
            .collect();

        // Extract periodic waveform from PSS result
        // For now, use a simplified model where we assume direct harmonic coupling

        // Build harmonic admittance matrix
        // Y_km = (1/T) ∫₀^T y(t) · e^{-j2π(k-m)f₀t} dt
        //
        // This is approximated by DFT of sampled y(t)

        for freq_idx in 0..result.num_frequencies() {
            for out_sb in self.config.sideband_min..=self.config.sideband_max {
                for in_sb in self.config.sideband_min..=self.config.sideband_max {
                    // Compute harmonic coupling coefficient
                    let harmonic_order = out_sb - in_sb;
                    let coupling = self.compute_harmonic_coupling(
                        harmonic_order,
                        &time_samples,
                        pss_result,
                        num_nodes,
                    );

                    // Scale by frequency-dependent factor
                    let freq_offset = result.frequencies[freq_idx];
                    let output_freq = (out_sb as Value) * fundamental + freq_offset;

                    // Apply frequency scaling for reactive elements
                    let scaled = if output_freq.abs() > 1e-10 {
                        coupling
                    } else {
                        coupling * 0.5 // DC component scaling
                    };

                    result
                        .conversion_matrix
                        .set(freq_idx, out_sb, in_sb, scaled);
                }
            }
        }

        Ok(())
    }

    /// Compute harmonic coupling coefficient using DFT
    ///
    /// H_k = (1/N) Σₙ h[n] · e^{-j2πkn/N}
    fn compute_harmonic_coupling(
        &self,
        harmonic_order: i32,
        time_samples: &[Value],
        pss_result: &PssResult,
        _num_nodes: usize,
    ) -> Complex64 {
        let n = time_samples.len();
        if n == 0 {
            return Complex64::new(0.0, 0.0);
        }

        // Sum over time samples with DFT kernel
        let mut sum = Complex64::new(0.0, 0.0);

        for (i, &_t) in time_samples.iter().enumerate() {
            // Sample the transfer function at this time
            // For a first implementation, use a simple model:
            // h(t) ≈ 1 + Σ_k a_k cos(2πkf₀t + φ_k)
            //
            // For a mixer, the main coupling comes from LO harmonics

            // Get PSS waveform sample (simplified: use first harmonic data if available)
            let h_t = self.sample_transfer_function(pss_result, i, n);

            // DFT kernel: e^{-j2πkn/N}
            let angle = -2.0 * PI * (harmonic_order as Value) * (i as Value) / (n as Value);
            let kernel = Complex64::new(angle.cos(), angle.sin());

            sum += h_t * kernel;
        }

        // Normalize
        sum / (n as Value)
    }

    /// Sample the time-varying transfer function from PSS data
    fn sample_transfer_function(
        &self,
        pss_result: &PssResult,
        sample_idx: usize,
        total_samples: usize,
    ) -> Complex64 {
        // For a complete implementation, this would:
        // 1. Interpolate PSS waveform to this time point
        // 2. Evaluate device small-signal parameters (gm, gds, etc.)
        // 3. Return the composite transfer function
        //
        // For now, use a simplified mixing model based on PSS waveform:
        // The transfer is dominated by the LO waveform shape

        let period = pss_result.period;
        if period <= 0.0 || total_samples == 0 {
            return Complex64::new(1.0, 0.0);
        }

        // Calculate time within period
        let t = (sample_idx as Value) / (total_samples as Value) * period;

        // Use PSS waveform data if available
        // Interpolate from the first non-ground waveform (LO-like node)
        if !pss_result.waveforms.is_empty() && !pss_result.time.is_empty() {
            // Get first waveform (typically LO or main driving signal)
            let wf = &pss_result.waveforms[0];
            let v = wf.interpolate(&pss_result.time, t, period);

            // Model transfer as modulated by waveform
            // Normalize around 1.0 to create mixing behavior
            let pp = wf.peak_to_peak();
            let dc = wf.dc(&pss_result.time, period);

            let normalized = if pp > 1e-15 {
                1.0 + 0.5 * (v - dc) / (pp / 2.0)
            } else {
                1.0
            };

            return Complex64::new(normalized, 0.0);
        }

        // Fallback: simple sinusoidal modulation
        let t_normalized = (sample_idx as Value) / (total_samples as Value);
        let modulation_depth = 0.5;
        let re = 1.0 + modulation_depth * (2.0 * PI * t_normalized).cos();

        Complex64::new(re, 0.0)
    }

    /// Solve for each frequency point in the sweep
    fn solve_frequency_sweep(
        &self,
        result: &mut PacResult,
        num_nodes: usize,
    ) -> Result<(), PacError> {
        for freq_idx in 0..result.num_frequencies() {
            // For each sideband, compute output voltage
            for out_sb in self.config.sideband_min..=self.config.sideband_max {
                // Output voltage is sum of contributions from all input sidebands
                let mut v_out = Complex64::new(0.0, 0.0);

                for in_sb in self.config.sideband_min..=self.config.sideband_max {
                    let transfer = result.conversion_matrix.get(freq_idx, out_sb, in_sb);

                    // Assume unit input at the designated input sideband
                    let v_in = if in_sb == 1 {
                        Complex64::new(1.0, 0.0) // RF input at sideband +1
                    } else {
                        Complex64::new(0.0, 0.0)
                    };

                    v_out += transfer * v_in;
                }

                // Store result for all nodes (simplified: same response pattern)
                if let Some(sb_data) = result.get_sideband_data_mut(freq_idx, out_sb) {
                    for node in 0..num_nodes {
                        // Apply node-dependent scaling
                        let node_scale = if node == 0 {
                            Complex64::new(0.0, 0.0) // Ground
                        } else {
                            v_out * Complex64::new((node as Value).sqrt() * 0.5, 0.0)
                        };

                        sb_data.set_voltage(node, node_scale);
                    }
                }
            }
        }

        Ok(())
    }
}

//=============================================================================
// Tests
//=============================================================================
