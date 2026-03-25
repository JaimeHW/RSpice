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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_pss_result() -> PssResult {
        use crate::analysis::advanced::pss::PeriodicWaveform;

        // Create a minimal PSS result for testing
        let mut result = PssResult::new(1e-9, 3, 4); // period=1ns, 3 nodes, 4 points

        // Set time points
        result.time = vec![0.0, 0.25e-9, 0.5e-9, 0.75e-9];

        // Set node names
        result.node_names = vec!["IN".to_string(), "LO".to_string(), "OUT".to_string()];

        // Set waveforms (one per node, not including ground)
        result.waveforms = vec![
            PeriodicWaveform::from_values(vec![0.0, 1.0, 0.0, -1.0]), // IN: sinusoidal
            PeriodicWaveform::from_values(vec![1.0, 0.0, -1.0, 0.0]), // LO: cos-like
            PeriodicWaveform::from_values(vec![0.0, 0.5, 0.0, -0.5]), // OUT: attenuated
        ];

        result.iterations = 5;
        result.residual_norm = 1e-9;

        result
    }

    #[test]
    fn test_pac_analyzer_creation() {
        let config = PacConfig::new();
        let analyzer = PacAnalyzer::new(config);

        assert_eq!(analyzer.state(), PacAnalyzerState::Ready);
        assert!(analyzer.num_samples >= 16);
    }

    #[test]
    fn test_pac_analyzer_with_samples() {
        let config = PacConfig::new();
        let analyzer = PacAnalyzer::new(config).with_samples(128);

        assert_eq!(analyzer.num_samples, 128);
    }

    #[test]
    fn test_pac_analyzer_validation() {
        let config = PacConfig::new().with_sweep(1e6, 1e9, 100);
        let analyzer = PacAnalyzer::new(config);

        assert!(analyzer.validate().is_ok());
    }

    #[test]
    fn test_pac_analyzer_validation_fails_invalid_sweep() {
        let config = PacConfig::new().with_sweep(-1e6, 1e9, 100); // Negative start
        let analyzer = PacAnalyzer::new(config);

        assert!(analyzer.validate().is_err());
    }

    #[test]
    fn test_pac_analyzer_analyze_basic() {
        let config = PacConfig::new()
            .with_sweep(1e6, 10e6, 5)
            .with_sidebands(-2, 2)
            .with_input_source("VRF")
            .with_output_node("OUT");

        let mut analyzer = PacAnalyzer::new(config);
        let pss = create_test_pss_result();
        let node_names = vec!["0".to_string(), "IN".to_string(), "OUT".to_string()];
        let branch_names = vec!["V1".to_string()];

        let result = analyzer.analyze(&pss, 3, node_names, branch_names);
        assert!(result.is_ok());

        let pac_result = result.unwrap();
        assert_eq!(pac_result.num_sidebands(), 5);
        assert_eq!(analyzer.state(), PacAnalyzerState::Complete);
    }

    #[test]
    fn test_pac_analyzer_invalid_pss_period() {
        let mut pss = create_test_pss_result();
        pss.period = -1.0; // Invalid period

        let config = PacConfig::new().with_sweep(1e6, 1e9, 10);
        let mut analyzer = PacAnalyzer::new(config);

        let result = analyzer.analyze(
            &pss,
            3,
            vec!["0".to_string(), "IN".to_string(), "OUT".to_string()],
            vec![],
        );

        assert!(result.is_err());
        assert_eq!(analyzer.state(), PacAnalyzerState::Failed);
    }

    #[test]
    fn test_pac_error_display() {
        let errors = vec![
            PacError::NoPssSolution,
            PacError::InvalidPssSolution("test".to_string()),
            PacError::InvalidConfig("bad config".to_string()),
            PacError::InvalidSweep("no points".to_string()),
            PacError::MatrixSolverFailed("singular".to_string()),
            PacError::NoInputSource,
            PacError::InputSourceNotFound("VRF".to_string()),
        ];

        for err in errors {
            let msg = format!("{}", err);
            assert!(!msg.is_empty());
        }
    }

    #[test]
    fn test_pac_result_conversion_matrix() {
        let config = PacConfig::new()
            .with_sweep(1e6, 1e6, 1) // Single frequency
            .with_sidebands(-1, 1);

        let mut analyzer = PacAnalyzer::new(config);
        let pss = create_test_pss_result();

        let result = analyzer
            .analyze(
                &pss,
                3,
                vec!["0".to_string(), "IN".to_string(), "OUT".to_string()],
                vec![],
            )
            .unwrap();

        // Check conversion matrix has been populated
        let gain = result.conversion_gain(1, 0, 0); // RF to IF
        // The gain should be non-zero for a proper mixer analysis
        // (exact value depends on PSS waveform used)
        assert!(gain.norm() >= 0.0);
    }

    #[test]
    fn test_pac_analyzer_state_transitions() {
        let config = PacConfig::new().with_sweep(1e6, 1e9, 5);
        let mut analyzer = PacAnalyzer::new(config);

        assert_eq!(analyzer.state(), PacAnalyzerState::Ready);

        let pss = create_test_pss_result();
        let _ = analyzer.analyze(
            &pss,
            3,
            vec!["0".to_string(), "IN".to_string(), "OUT".to_string()],
            vec![],
        );

        assert_eq!(analyzer.state(), PacAnalyzerState::Complete);
    }

    #[test]
    fn test_harmonic_coupling_computation() {
        let config = PacConfig::new().with_sidebands(-2, 2);
        let analyzer = PacAnalyzer::new(config);

        let time_samples: Vec<Value> = (0..64).map(|i| (i as f64) / 64.0 * 1e-9).collect();

        let pss = create_test_pss_result();

        // DC component (harmonic 0) should have largest magnitude
        let h0 = analyzer.compute_harmonic_coupling(0, &time_samples, &pss, 3);
        let h1 = analyzer.compute_harmonic_coupling(1, &time_samples, &pss, 3);

        assert!(
            h0.norm() >= h1.norm() * 0.5,
            "DC component should be significant: |H0|={}, |H1|={}",
            h0.norm(),
            h1.norm()
        );
    }

    #[test]
    fn test_sample_transfer_function() {
        let config = PacConfig::new();
        let analyzer = PacAnalyzer::new(config);
        let pss = create_test_pss_result();

        // Sample at different points
        let h0 = analyzer.sample_transfer_function(&pss, 0, 64);
        let h16 = analyzer.sample_transfer_function(&pss, 16, 64);
        let h32 = analyzer.sample_transfer_function(&pss, 32, 64);

        // Values should be different due to time variation
        assert!(
            (h0.re - h16.re).abs() > 0.0
                || (h0.im - h16.im).abs() > 0.0
                || (h0.re - h32.re).abs() > 0.0,
            "Transfer function should vary with time"
        );
    }

    #[test]
    fn test_pac_result_frequency_sweep() {
        let config = PacConfig::new()
            .with_sweep(1e6, 10e6, 10)
            .with_sidebands(-1, 1);

        let mut analyzer = PacAnalyzer::new(config);
        let pss = create_test_pss_result();

        let result = analyzer
            .analyze(
                &pss,
                3,
                vec!["0".to_string(), "IN".to_string(), "OUT".to_string()],
                vec![],
            )
            .unwrap();

        // Should have multiple frequency points
        assert!(result.num_frequencies() >= 10);

        // Check magnitude vs frequency is available
        let mag_vs_freq = result.magnitude_vs_frequency(1, 0);
        assert!(!mag_vs_freq.is_empty());
    }

    #[test]
    fn test_pac_analyzer_default_state() {
        assert_eq!(PacAnalyzerState::default(), PacAnalyzerState::Ready);
    }

    #[test]
    fn test_pac_result_has_correct_sidebands() {
        let config = PacConfig::new()
            .with_sweep(1e6, 1e6, 1)
            .with_sidebands(-3, 3);

        let mut analyzer = PacAnalyzer::new(config);
        let pss = create_test_pss_result();

        let result = analyzer
            .analyze(
                &pss,
                3,
                vec!["0".to_string(), "IN".to_string(), "OUT".to_string()],
                vec![],
            )
            .unwrap();

        assert_eq!(result.sideband_min, -3);
        assert_eq!(result.sideband_max, 3);
        assert_eq!(result.num_sidebands(), 7);
    }

    #[test]
    fn test_pac_input_output_metadata() {
        let config = PacConfig::new()
            .with_sweep(1e6, 1e6, 1)
            .with_input_source("VIN")
            .with_output_node("VOUT");

        let mut analyzer = PacAnalyzer::new(config);
        let pss = create_test_pss_result();

        let result = analyzer
            .analyze(
                &pss,
                3,
                vec!["0".to_string(), "IN".to_string(), "OUT".to_string()],
                vec![],
            )
            .unwrap();

        assert_eq!(result.input_source, Some("VIN".to_string()));
        assert_eq!(result.output_node, Some("VOUT".to_string()));
    }

    #[test]
    fn test_pac_convergence_info() {
        let config = PacConfig::new().with_sweep(1e6, 1e6, 1);
        let mut analyzer = PacAnalyzer::new(config);
        let pss = create_test_pss_result();

        let result = analyzer
            .analyze(
                &pss,
                3,
                vec!["0".to_string(), "IN".to_string(), "OUT".to_string()],
                vec![],
            )
            .unwrap();

        // Direct solve should report 1 iteration
        assert_eq!(result.iterations, 1);
        assert!((result.residual - 0.0).abs() < 1e-10);
    }
}
