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

pub use pac::PacAnalysisResult;
pub use pnoise::PnoiseAnalysisResult;

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
        // Validate configuration
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

        // Build circuit using SoA architecture
        let circuit = self.build_circuit(netlist)?;

        // Get node count (excluding ground)
        let num_nodes = circuit.num_nodes();
        if num_nodes == 0 {
            return Err(SimulationError::Circuit("Circuit has no nodes".to_string()));
        }

        // No reactive-element gate: junction devices carry their own charge
        // storage, and HB on a resistive nonlinear circuit is legitimate
        // distortion analysis (every harmonic system is solvable regardless).
        if let Some(summary) = Self::hb_unsupported_nonlinear_device_summary(&circuit, num_nodes) {
            return Err(HbError::UnsupportedNonlinearDevices(summary).into());
        }
        let has_supported_nonlinear = Self::hb_has_supported_nonlinear_devices(&circuit, num_nodes);
        let drive_tones = Self::hb_collect_drive_tones(&config)?;
        Self::hb_validate_drive_tone_sources(&circuit, &drive_tones)?;

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
            solver.solve_newton(&mut state).map_err(|e| match e {
                crate::analysis::HbError::ConvergenceFailed {
                    iterations,
                    residual,
                } => HbError::ConvergenceFailed {
                    iterations,
                    residual,
                }
                .into(),
                other => SimulationError::Circuit(format!("HB nonlinear solve failed: {}", other)),
            })?;
        } else {
            // Solve linear HB system
            solver
                .solve_linear(&mut state)
                .map_err(|_| SimulationError::Circuit("HB linear solve failed".to_string()))?;
        }

        // Build result
        let result = solver.build_result(&state);

        Ok(HbAnalysisResult {
            result,
            fundamental_freq: config.fundamental_freq,
            num_harmonics: config.num_harmonics,
            converged: state.converged,
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
            phase: 0.0,
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
