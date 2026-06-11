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
        if config.fundamental_freq <= 0.0 {
            return Err(HbError::InvalidConfig(
                "Fundamental frequency must be positive".to_string(),
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
            self.hb_stamp_voltage_sources_norton(&circuit, &mut solver, &drive_tones);
        } else {
            self.hb_stamp_voltage_sources(&circuit, &mut solver, &drive_tones);
        }
        self.hb_stamp_current_sources(&circuit, &mut solver, &drive_tones);
        if has_supported_nonlinear {
            self.hb_stamp_supported_nonlinear_devices(&circuit, &mut solver, num_nodes);
        }

        // Create solver state
        let mut state = HbSolverState::new(num_nodes, config.num_harmonics);

        // Initialize DC components to zero (proper approach would use DC OP first)
        for node in 0..num_nodes {
            if node < state.x.len() && !state.x[node].is_empty() {
                state.x[node][0] = Complex64::new(0.0, 0.0);
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
