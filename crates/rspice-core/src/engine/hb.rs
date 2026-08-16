//! Harmonic Balance (HB) Analysis Engine Integration
//!
//! This module provides Harmonic Balance analysis for finding
//! periodic steady-state solutions in the frequency domain.
//!
//! # Overview
//!
//! HB solves directly for the Fourier coefficients of node voltages, making it
//! ideal for RF/MW circuits with slow time constants where transient would be
//! prohibitively slow.
//!
//! # Algorithm
//!
//! 1. **Circuit setup**: Build admittance matrices G (conductance) and C (capacitance)
//! 2. **Source stamping**: Extract DC and AC source spectra
//! 3. **Newton iteration**: Solve for spectral coefficients via Newton-Raphson
//!    - Linear part: (G + jω*C) * X
//!    - Nonlinear part: FFT ↔ time-domain evaluation ↔ IFFT
//! 4. **Result construction**: Build HbResult with spectral voltages and harmonics

use super::{Engine, SimulationError};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::harmonic_balance::HbFft;
use crate::analysis::{HbConfig, HbResult, HbSolver, HbSolverState};
use crate::circuit::CircuitData;
use crate::netlist::SourceSpec;
use crate::{Netlist, Value};
use num_complex::Complex64;
use std::collections::BTreeSet;

mod drive;
mod pac;
mod pnoise;
mod stamping;
mod state;

pub use pac::PacAnalysisResult;
pub use pnoise::PnoiseAnalysisResult;
pub use state::{HbEnvelopeContinuationState, HbEnvelopeStateGuarantee};

/// Exact converged harmonic-balance numerical state retained for dependent
/// periodic small-signal analyses.
///
/// Display spectra are not an execution contract: they may be filtered or
/// converted to magnitude/phase. This payload keeps the frozen HB basis and
/// the solver's complex Fourier coefficients in canonical node order so a
/// dependent analysis never has to re-solve or infer the large-signal state.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "veriloga", derive(serde::Serialize, serde::Deserialize))]
pub struct HbOperatingPoint {
    config: HbConfig,
    node_names: Vec<String>,
    spectral_state: Vec<Vec<Complex64>>,
    iterations: usize,
    residual_norm: Value,
}

impl HbOperatingPoint {
    /// Frozen HB configuration that produced this state.
    pub fn config(&self) -> &HbConfig {
        &self.config
    }

    /// Canonical non-ground node order used by every spectral row.
    pub fn node_names(&self) -> &[String] {
        &self.node_names
    }

    /// Solver Fourier coefficients indexed `[node][harmonic]`.
    pub fn spectral_state(&self) -> &[Vec<Complex64>] {
        &self.spectral_state
    }

    /// Number of nonlinear iterations used by the producer solve.
    pub fn iterations(&self) -> usize {
        self.iterations
    }

    /// Final producer residual norm.
    pub fn residual_norm(&self) -> Value {
        self.residual_norm
    }

    /// Highest retained positive harmonic.
    pub fn spectral_harmonic_capacity(&self) -> usize {
        self.config.num_harmonics
    }

    /// Reconstruct an HB operating point after authenticated transport.
    /// Shape, finiteness, and canonical-name checks are repeated at this
    /// boundary before the state can enter a numerical kernel.
    pub fn try_from_parts(
        config: HbConfig,
        node_names: Vec<String>,
        spectral_state: Vec<Vec<Complex64>>,
        iterations: usize,
        residual_norm: Value,
    ) -> Result<Self, SimulationError> {
        if !config.fundamental_freq.is_finite() || config.fundamental_freq <= 0.0 {
            return Err(SimulationError::Circuit(
                "retained HB state has an invalid fundamental frequency".to_owned(),
            ));
        }
        if config.num_harmonics == 0 {
            return Err(SimulationError::Circuit(
                "retained HB state has no harmonic basis".to_owned(),
            ));
        }
        if !residual_norm.is_finite() || residual_norm < 0.0 {
            return Err(SimulationError::Circuit(
                "retained HB state has an invalid residual norm".to_owned(),
            ));
        }
        if node_names.is_empty() || spectral_state.len() != node_names.len() {
            return Err(SimulationError::Circuit(format!(
                "retained HB state contains {} spectral row(s) for {} node name(s)",
                spectral_state.len(),
                node_names.len()
            )));
        }
        let expected_harmonics = config.num_harmonics.saturating_add(1);
        let mut seen = std::collections::HashSet::with_capacity(node_names.len());
        for (node, spectrum) in node_names.iter().zip(&spectral_state) {
            if node.is_empty() || node.trim() != node {
                return Err(SimulationError::Circuit(
                    "retained HB state contains a non-canonical node name".to_owned(),
                ));
            }
            if !seen.insert(node.to_ascii_uppercase()) {
                return Err(SimulationError::Circuit(format!(
                    "retained HB state contains duplicate node name '{node}'"
                )));
            }
            if spectrum.len() != expected_harmonics {
                return Err(SimulationError::Circuit(format!(
                    "retained HB node '{node}' contains {} coefficients; the frozen basis requires {expected_harmonics}",
                    spectrum.len()
                )));
            }
            if spectrum
                .iter()
                .any(|value| !value.re.is_finite() || !value.im.is_finite())
            {
                return Err(SimulationError::Circuit(format!(
                    "retained HB node '{node}' contains a non-finite coefficient"
                )));
            }
        }
        Ok(Self {
            config,
            node_names,
            spectral_state,
            iterations,
            residual_norm,
        })
    }

    fn to_solver_state(
        &self,
        expected_node_names: &[String],
    ) -> Result<HbSolverState, SimulationError> {
        if self.node_names != expected_node_names {
            return Err(SimulationError::Circuit(format!(
                "retained HB node basis does not match the elaborated circuit: expected {:?}, received {:?}",
                expected_node_names, self.node_names
            )));
        }
        let mut state = HbSolverState::new(self.node_names.len(), self.config.num_harmonics);
        state.x.clone_from(&self.spectral_state);
        state.iteration = self.iterations;
        state.total_iterations = self.iterations;
        state.residual_norm = self.residual_norm;
        state.converged = true;
        Ok(state)
    }
}

/// HB-specific error types
#[derive(Debug, Clone)]
pub enum HbError {
    /// Newton iteration did not converge
    ConvergenceFailed { iterations: usize, residual: Value },
    /// Circuit has no reactive elements
    NoReactiveElements,
    /// Invalid configuration
    InvalidConfig(String),
    /// Matrix is singular
    SingularMatrix,
    /// Circuit contains nonlinear/advanced devices not yet supported by HB runtime.
    UnsupportedNonlinearDevices(String),
}

impl std::fmt::Display for HbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConvergenceFailed {
                iterations,
                residual,
            } => {
                write!(
                    f,
                    "HB convergence failed after {} iterations (residual: {:.3e})",
                    iterations, residual
                )
            }
            Self::NoReactiveElements => write!(f, "Circuit has no capacitors or inductors"),
            Self::InvalidConfig(msg) => write!(f, "Invalid HB config: {}", msg),
            Self::SingularMatrix => write!(f, "Singular admittance matrix"),
            Self::UnsupportedNonlinearDevices(summary) => {
                write!(f, "HB runtime does not yet support {}", summary)
            }
        }
    }
}

impl std::error::Error for HbError {}

impl From<HbError> for SimulationError {
    fn from(e: HbError) -> Self {
        match e {
            HbError::ConvergenceFailed { iterations, .. } => {
                SimulationError::ConvergenceFailed(iterations)
            }
            _ => SimulationError::Circuit(e.to_string()),
        }
    }
}

/// HB analysis result with detailed info
#[derive(Debug)]
pub struct HbAnalysisResult {
    /// The HB solution
    pub result: HbResult,
    /// Fundamental frequency
    pub fundamental_freq: Value,
    /// Number of harmonics
    pub num_harmonics: usize,
    /// Whether solution converged
    pub converged: bool,
    /// Exact spectral operating point consumed by HB-dependent analyses.
    pub operating_point: HbOperatingPoint,
}

const HB_NORTON_G: Value = 1e6; // Rs = 1 uOhm for stiff source conversion in nonlinear HB.
const HB_ZERO_SENSE_TOL: Value = 1e-12;

#[derive(Debug, Clone, Copy)]
struct HbCurrentSwitchControl {
    ctrl_pos: usize,
    ctrl_neg: usize,
    control_current_bias: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct HbDriveTone {
    harmonic: usize,
    source_filter: Option<String>,
}

#[derive(Debug, Clone)]
struct HbSourceSpectrum {
    dc: Value,
    /// Physical peak amplitudes and phases for positive harmonics. The HB
    /// solver's stamping boundary converts these to one-sided Fourier
    /// coefficients.
    harmonics: Vec<(usize, Value, Value)>,
}

impl HbDriveTone {
    fn broadcast(harmonic: usize) -> Self {
        Self {
            harmonic,
            source_filter: None,
        }
    }

    fn matches_source(&self, source_name: &str) -> bool {
        match &self.source_filter {
            None => true,
            Some(filter) => filter.eq_ignore_ascii_case(source_name),
        }
    }
}

impl Engine {
    /// Project an authenticated shooting-PSS orbit into the HB spectral basis
    /// used by periodic small-signal kernels. This is a representation change,
    /// not an operating-point solve: every coefficient is sampled from the
    /// retained orbit and no Newton or linear large-signal solve is run.
    fn hb_state_from_pss_operating_point(
        &self,
        operating_point: &super::PssOperatingPoint,
        config: &HbConfig,
        node_names: &[String],
    ) -> Result<HbSolverState, SimulationError> {
        let analysis = operating_point.analysis();
        let result = &analysis.result;
        if !result.frequency.is_finite() || result.frequency <= 0.0 {
            return Err(SimulationError::Circuit(
                "periodic operating point has an invalid fundamental frequency".to_owned(),
            ));
        }
        let relative_frequency_error =
            ((result.frequency - config.fundamental_freq) / result.frequency).abs();
        if relative_frequency_error > 1.0e-9 {
            return Err(SimulationError::Circuit(format!(
                "periodic operating-point frequency {:.16e} Hz does not match the dependent analysis basis {:.16e} Hz",
                result.frequency, config.fundamental_freq
            )));
        }
        if node_names.len() != result.waveforms.len() {
            return Err(SimulationError::Circuit(format!(
                "periodic operating point contains {} node waveforms for a {}-node dependent circuit",
                result.waveforms.len(),
                node_names.len()
            )));
        }

        let mut fft = HbFft::with_size(config.num_harmonics, config.fft_size());
        let sample_count = fft.size();
        let mut state = HbSolverState::new(node_names.len(), config.num_harmonics);
        for (target_index, target_name) in node_names.iter().enumerate() {
            let source_index = result
                .node_names
                .iter()
                .position(|source_name| source_name.eq_ignore_ascii_case(target_name))
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "periodic operating point has no waveform for dependent-circuit node '{target_name}'"
                    ))
                })?;
            let waveform = &result.waveforms[source_index];
            let samples = (0..sample_count)
                .map(|sample_index| {
                    let time = analysis.period * sample_index as Value / sample_count as Value;
                    waveform.interpolate(&result.time, time, analysis.period)
                })
                .collect::<Vec<_>>();
            state.x[target_index] = fft.to_frequency_domain(&samples);
        }
        state.iteration = analysis.iterations.max(1);
        state.total_iterations = analysis.iterations;
        state.residual_norm = analysis.final_residual;
        state.converged = true;
        Ok(state)
    }

    /// Run Harmonic Balance analysis
    ///
    /// This is the main entry point for HB simulation. It builds the circuit,
    /// extracts admittance matrices, and solves for spectral coefficients.
    ///
    /// # Arguments
    /// * `netlist` - The circuit netlist
    /// * `config` - HB analysis configuration
    ///
    /// # Returns
    /// * `Ok(HbAnalysisResult)` - Successful analysis with spectral voltages
    /// * `Err(SimulationError)` - Analysis failed
    ///
    /// # Example
    /// ```ignore
    /// use rspice_core::{Engine, Netlist};
    /// use rspice_core::analysis::HbConfig;
    ///
    /// let netlist = Netlist::parse("...")?;
    /// let engine = Engine::default();
    /// let config = HbConfig::new(1e9).with_harmonics(9);
    /// let result = engine.run_hb(&netlist, config)?;
    /// ```
    pub fn run_hb(
        &self,
        netlist: &Netlist,
        config: HbConfig,
    ) -> Result<HbAnalysisResult, SimulationError> {
        self.run_hb_with_abort(netlist, config, &NoAbort)
    }

    /// Run harmonic balance with cooperative cancellation.
    pub fn run_hb_with_abort(
        &self,
        netlist: &Netlist,
        config: HbConfig,
        abort: &dyn AbortSignal,
    ) -> Result<HbAnalysisResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let engine = self.resolved_for_netlist(netlist);
        engine.hb_validate_config(&config)?;

        // Build circuit using SoA architecture
        let circuit = engine.build_circuit_with_abort(netlist, abort)?;
        engine.run_hb_with_prebuilt_circuit_abort(netlist, circuit, config, abort)
    }

    fn hb_validate_config(&self, config: &HbConfig) -> Result<(), SimulationError> {
        if !config.fundamental_freq.is_finite() || config.fundamental_freq <= 0.0 {
            return Err(HbError::InvalidConfig(
                "Fundamental frequency must be finite and positive".to_string(),
            )
            .into());
        }
        if config.num_harmonics == 0 {
            return Err(
                HbError::InvalidConfig("Must have at least one harmonic".to_string()).into(),
            );
        }

        let minimum_points = config.minimum_collocation_points().ok_or_else(|| {
            HbError::InvalidConfig(format!(
                "harmonic count {} exceeds the addressable collocation grid",
                config.num_harmonics
            ))
        })?;
        if let Some(points) = config.collocation_points
            && (points % 2 == 0 || points < minimum_points)
        {
            return Err(HbError::InvalidConfig(format!(
                "collocation grid must be odd and contain at least {} points for {} harmonics; found {points}",
                minimum_points,
                config.num_harmonics
            ))
            .into());
        }
        self.ensure_analysis_points(config.fft_size())?;
        self.ensure_analysis_points(config.num_harmonics.saturating_add(1))?;
        Ok(())
    }

    /// Solve an already elaborated circuit. HB-specific clients use this
    /// boundary when they must authenticate a source transformation before
    /// the periodic solve; ordinary callers always enter through
    /// [`Self::run_hb_with_abort`].
    fn run_hb_with_prebuilt_circuit_abort(
        &self,
        netlist: &Netlist,
        circuit: CircuitData,
        config: HbConfig,
        abort: &dyn AbortSignal,
    ) -> Result<HbAnalysisResult, SimulationError> {
        // Get node count (excluding ground)
        let num_nodes = circuit.num_nodes();
        if num_nodes == 0 {
            return Err(SimulationError::Circuit("Circuit has no nodes".to_string()));
        }
        self.ensure_result_shape(
            config.num_harmonics.saturating_add(1),
            num_nodes.saturating_mul(2),
        )?;

        // No reactive-element gate: junction devices carry their own charge
        // storage, and HB on a resistive nonlinear circuit is legitimate
        // distortion analysis (every harmonic system is solvable regardless).
        if let Some(summary) = Self::hb_unsupported_nonlinear_device_summary(&circuit, num_nodes) {
            return Err(HbError::UnsupportedNonlinearDevices(summary).into());
        }
        let has_supported_nonlinear = Self::hb_has_supported_nonlinear_devices(&circuit, num_nodes);
        let drive_tones = Self::hb_collect_drive_tones(&config)?;
        Self::hb_validate_drive_tone_sources(&circuit, &drive_tones)?;

        // Create solver
        let mut solver = HbSolver::new(config.clone(), num_nodes);

        // Set node names from circuit's node map
        let node_names = self.hb_build_node_names(&circuit, num_nodes);
        solver.set_node_names(node_names);

        // Stamp linear circuit elements into HB solver
        self.hb_stamp_resistors(&circuit, &mut solver);
        self.hb_stamp_capacitors(&circuit, &mut solver);
        self.hb_stamp_inductors(&circuit, &mut solver);
        if has_supported_nonlinear {
            self.hb_stamp_voltage_sources_norton(&circuit, &mut solver, &config, &drive_tones)?;
        } else {
            self.hb_stamp_voltage_sources(&circuit, &mut solver, &config, &drive_tones)?;
        }
        self.hb_stamp_current_sources(&circuit, &mut solver, &config, &drive_tones)?;
        if has_supported_nonlinear {
            self.hb_stamp_supported_nonlinear_devices(&circuit, &mut solver, num_nodes);
        }

        // Create solver state
        let mut state = HbSolverState::new(num_nodes, config.num_harmonics);

        // Seed harmonic 0 with the DC operating point: Newton starts on the
        // bias trajectory instead of from zero, which is the difference
        // between converging and wandering for strongly biased circuits. A
        // failed OP falls back to the zero seed with a warning — HB's own
        // continuation may still succeed.
        if has_supported_nonlinear {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            match self.run_dc_op(netlist) {
                Ok(dc) => {
                    for node in 0..num_nodes {
                        if node < state.x.len() && !state.x[node].is_empty() {
                            let v = dc.node_voltages.get(node + 1).copied().unwrap_or(0.0);
                            state.x[node][0] = Complex64::new(v, 0.0);
                        }
                    }
                }
                Err(err) => {
                    log::warn!(
                        "HB: DC operating point for the harmonic-0 seed failed ({err}); \
                         starting from zero"
                    );
                }
            }
        }

        if has_supported_nonlinear {
            solver
                .solve_newton_with_abort(&mut state, abort)
                .map_err(|e| match e {
                    crate::analysis::HbError::Aborted => SimulationError::Aborted,
                    crate::analysis::HbError::ConvergenceFailed {
                        iterations,
                        residual,
                    } => HbError::ConvergenceFailed {
                        iterations,
                        residual,
                    }
                    .into(),
                    other => {
                        SimulationError::Circuit(format!("HB nonlinear solve failed: {}", other))
                    }
                })?;
        } else {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            // Solve linear HB system
            solver
                .solve_linear(&mut state)
                .map_err(|_| SimulationError::Circuit("HB linear solve failed".to_string()))?;
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
        }

        // Build result
        let mut result = solver.build_result(&state);
        self.hb_attach_periodic_state(&circuit, &mut result, has_supported_nonlinear);

        let operating_point = HbOperatingPoint::try_from_parts(
            config.clone(),
            result.node_names.clone(),
            state.x.clone(),
            state.total_iterations.max(state.iteration),
            state.residual_norm,
        )?;
        Ok(HbAnalysisResult {
            result,
            fundamental_freq: config.fundamental_freq,
            num_harmonics: config.num_harmonics,
            converged: state.converged,
            operating_point,
        })
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SimulationConfig;

    #[test]
    fn pulse_source_uses_the_exact_configured_collocation_grid() {
        let config = HbConfig::new(10.0e3)
            .with_harmonics(50)
            .with_collocation_points(101);
        let pulse = SourceSpec::Pulse {
            v1: 1.0,
            v2: 2.0,
            delay: 0.0,
            rise: 10.0e-6,
            fall: 10.0e-6,
            width: 40.0e-6,
            period: 100.0e-6,
            pulse_count: 0.0,
            width_defaults_to_zero: false,
        };
        let spectrum = Engine::hb_source_spectrum(1.0, 0.0, 0.0, Some(&pulse), &config, &[1])
            .expect("periodic pulse spectrum");

        assert!((spectrum.dc - 1.499_950_985_197_529_7).abs() < 1.0e-14);
        let (_, h1_amplitude, h1_phase) = spectrum.harmonics[0];
        let h1 = Complex64::from_polar(h1_amplitude, h1_phase);
        assert!(
            (h1.re - -1.935_850_060_379_601_7e-1).abs() < 1.0e-12,
            "h1={h1:?}"
        );
        assert!(
            (h1.im - -5.955_525_013_998_652e-1).abs() < 1.0e-12,
            "h1={h1:?}"
        );
        let h2 = spectrum
            .harmonics
            .iter()
            .find(|(harmonic, _, _)| *harmonic == 2)
            .map(|(_, amplitude, phase)| Complex64::from_polar(*amplitude, *phase))
            .expect("minimal odd grid aliases the continuous even harmonic onto the grid");
        assert!(h2.norm() > 1.0e-4);
    }

    #[test]
    fn sin_source_is_converted_from_sine_to_cosine_reference() {
        let config = HbConfig::new(1.0e3).with_harmonics(3);
        let sin = SourceSpec::Sin {
            offset: 2.0,
            amplitude: 3.0,
            frequency: 1.0e3,
            delay: 0.0,
            damping: 0.0,
            phase: 0.0,
        };
        let spectrum = Engine::hb_source_spectrum(2.0, 0.0, 0.0, Some(&sin), &config, &[1])
            .expect("periodic sine spectrum");

        assert_eq!(spectrum.dc, 2.0);
        let (_, amplitude, phase) = spectrum.harmonics[0];
        let phasor = Complex64::from_polar(amplitude, phase);
        assert!(phasor.re.abs() < 1.0e-12, "phasor={phasor:?}");
        assert!((phasor.im + 3.0).abs() < 1.0e-12, "phasor={phasor:?}");
    }

    #[test]
    fn transient_waveform_takes_precedence_over_small_signal_ac_annotation() {
        let config = HbConfig::new(1.0e3).with_harmonics(3);
        let source = SourceSpec::DcAcTransient {
            dc_value: 0.0,
            ac_magnitude: 99.0,
            ac_phase: 1.0,
            transient: Box::new(SourceSpec::Sin {
                offset: 1.0,
                amplitude: 2.0,
                frequency: 1.0e3,
                delay: 0.0,
                damping: 0.0,
                phase: 0.0,
            }),
        };
        let spectrum = Engine::hb_source_spectrum(0.0, 99.0, 1.0, Some(&source), &config, &[1])
            .expect("periodic transient spectrum");

        assert_eq!(spectrum.dc, 1.0);
        let (_, amplitude, phase) = spectrum.harmonics[0];
        let phasor = Complex64::from_polar(amplitude, phase);
        assert!(phasor.re.abs() < 1.0e-12, "phasor={phasor:?}");
        assert!((phasor.im + 2.0).abs() < 1.0e-12, "phasor={phasor:?}");
    }

    #[test]
    fn unmatched_periodic_source_contributes_dc_but_no_harmonics() {
        let config = HbConfig::new(1.0e3).with_harmonics(3);
        let sin = SourceSpec::Sin {
            offset: 2.0,
            amplitude: 3.0,
            frequency: 1.0e3,
            delay: 0.0,
            damping: 0.0,
            phase: 0.0,
        };
        let spectrum = Engine::hb_source_spectrum(2.0, 0.0, 0.0, Some(&sin), &config, &[])
            .expect("filtered periodic source");

        assert_eq!(spectrum.dc, 2.0);
        assert!(spectrum.harmonics.is_empty());
    }

    #[test]
    fn periodic_source_rejects_non_finite_parameters() {
        let config = HbConfig::new(1.0e3).with_harmonics(3);
        let sin = SourceSpec::Sin {
            offset: 0.0,
            amplitude: f64::NAN,
            frequency: 1.0e3,
            delay: 0.0,
            damping: 0.0,
            phase: 0.0,
        };
        let err = Engine::hb_source_spectrum(0.0, 0.0, 0.0, Some(&sin), &config, &[1])
            .expect_err("non-finite source parameters must fail");
        assert!(err.to_string().contains("amplitude must be finite"));
    }

    #[test]
    fn invalid_exact_collocation_grid_fails_before_solver_construction() {
        let netlist =
            Netlist::parse("invalid HB grid\nV1 1 0 1\nR1 1 0 1k\n.end\n").expect("deck parses");
        let config = HbConfig::new(1.0e3)
            .with_harmonics(5)
            .with_collocation_points(10);
        let err = Engine::new(SimulationConfig::default())
            .run_hb(&netlist, config)
            .expect_err("even/undersized collocation grid must fail");
        assert!(err.to_string().contains("collocation grid"));
    }

    #[test]
    fn non_finite_fundamental_is_rejected() {
        let netlist = Netlist::parse("invalid HB frequency\nV1 1 0 1\nR1 1 0 1k\n.end\n")
            .expect("deck parses");
        let err = Engine::new(SimulationConfig::default())
            .run_hb(&netlist, HbConfig::new(f64::NAN))
            .expect_err("NaN fundamental must fail");
        assert!(err.to_string().contains("finite and positive"));
    }
}
