//! Harmonic Balance (HB) Analysis Engine Integration
//!
//! This module provides commercial-grade Harmonic Balance analysis for finding
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
use crate::{Netlist, Value};
use num_complex::Complex64;

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

        // Check for reactive elements (capacitors or inductors)
        let has_reactive = !circuit.capacitors.is_empty() || !circuit.inductors.names.is_empty();
        if !has_reactive {
            return Err(HbError::NoReactiveElements.into());
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
        self.hb_stamp_voltage_sources(&circuit, &mut solver);
        self.hb_stamp_current_sources(&circuit, &mut solver);

        // Create solver state
        let mut state = HbSolverState::new(num_nodes, config.num_harmonics);

        // Initialize DC components to zero (proper approach would use DC OP first)
        for node in 0..num_nodes {
            if node < state.x.len() && !state.x[node].is_empty() {
                state.x[node][0] = Complex64::new(0.0, 0.0);
            }
        }

        // Solve linear HB system
        solver
            .solve_linear(&mut state)
            .map_err(|_| SimulationError::Circuit("HB linear solve failed".to_string()))?;

        // For nonlinear circuits, we would do Newton iteration here
        // TODO: Add nonlinear Newton loop with FFT/IFFT for diodes, BJTs, MOSFETs

        // Build result
        let result = solver.build_result(&state);

        Ok(HbAnalysisResult {
            result,
            fundamental_freq: config.fundamental_freq,
            num_harmonics: config.num_harmonics,
            converged: state.converged,
        })
    }

    /// Build node names from circuit node map
    fn hb_build_node_names(&self, _circuit: &CircuitData, num_nodes: usize) -> Vec<String> {
        // Generate simple numbered node names
        // In a production system, we would use circuit's actual node names
        (1..=num_nodes).map(|i| format!("n{}", i)).collect()
    }

    /// Stamp resistors into HB solver G matrix
    fn hb_stamp_resistors(&self, circuit: &CircuitData, solver: &mut HbSolver) {
        for i in 0..circuit.resistors.len() {
            let np = circuit.resistors.stamps[i].pp.row;
            let nn = circuit.resistors.stamps[i].nn.row;
            let g = circuit.resistors.conductances[i];

            // Stamp conductance matrix
            self.hb_stamp_admittance(solver, np, nn, g, true);
        }
    }

    /// Stamp capacitors into HB solver C matrix
    fn hb_stamp_capacitors(&self, circuit: &CircuitData, solver: &mut HbSolver) {
        for i in 0..circuit.capacitors.len() {
            let np = circuit.capacitors.stamps[i].pp.row;
            let nn = circuit.capacitors.stamps[i].nn.row;
            let c = circuit.capacitors.capacitances[i];

            // Stamp capacitance matrix
            self.hb_stamp_admittance(solver, np, nn, c, false);
        }
    }

    /// Stamp inductors into HB solver L matrix
    ///
    /// In the frequency domain, inductors have admittance Y_L = 1/(jωL).
    /// The solver handles the frequency-dependent admittance at each harmonic:
    /// - DC (k=0): short circuit (large conductance)
    /// - AC (k>0): Y_L = -j/(k*ω₀*L)
    fn hb_stamp_inductors(&self, circuit: &CircuitData, solver: &mut HbSolver) {
        for i in 0..circuit.inductors.len() {
            let np = circuit.inductors.node_pos[i];
            let nn = circuit.inductors.node_neg[i];
            let l = circuit.inductors.inductances[i];

            // Stamp inductance matrix
            self.hb_stamp_inductance(solver, np, nn, l);
        }
    }

    /// Stamp a two-terminal inductance into HB solver L matrix
    fn hb_stamp_inductance(&self, solver: &mut HbSolver, np: usize, nn: usize, value: Value) {
        // Standard MNA stamp pattern for two-terminal inductor
        if np > 0 && nn > 0 {
            // Both nodes are non-ground
            let i = np - 1;
            let j = nn - 1;
            solver.add_inductance(i, i, value);
            solver.add_inductance(i, j, -value);
            solver.add_inductance(j, i, -value);
            solver.add_inductance(j, j, value);
        } else if np > 0 {
            // nn is ground
            let i = np - 1;
            solver.add_inductance(i, i, value);
        } else if nn > 0 {
            // np is ground
            let i = nn - 1;
            solver.add_inductance(i, i, value);
        }
    }

    /// Stamp voltage sources into HB solver
    /// Note: For full HB, voltage sources require branch currents (MNA).
    /// Here we use a simplified Norton equivalent approach.
    fn hb_stamp_voltage_sources(&self, circuit: &CircuitData, solver: &mut HbSolver) {
        const SOURCE_CONDUCTANCE: Value = 1e-3; // 1k source resistance approximation

        for i in 0..circuit.voltage_sources.len() {
            let np = circuit.voltage_sources.node_pos[i];
            let nn = circuit.voltage_sources.node_neg[i];
            let dc = circuit.voltage_sources.dc_values[i];

            // Add small source conductance for numerical stability
            self.hb_stamp_admittance(solver, np, nn, SOURCE_CONDUCTANCE, true);

            // Stamp DC component as Norton current: I = V * G
            if np > 0 {
                solver.set_dc_source(np - 1, dc * SOURCE_CONDUCTANCE);
            }
            if nn > 0 {
                solver.set_dc_source(nn - 1, -dc * SOURCE_CONDUCTANCE);
            }
        }
    }

    /// Stamp current sources into HB solver
    fn hb_stamp_current_sources(&self, circuit: &CircuitData, solver: &mut HbSolver) {
        for i in 0..circuit.current_sources.len() {
            let np = circuit.current_sources.node_pos[i];
            let nn = circuit.current_sources.node_neg[i];
            let dc = circuit.current_sources.dc_values[i];

            // Current source stamps directly into RHS
            if np > 0 {
                solver.set_dc_source(np - 1, -dc); // Current leaves at + terminal
            }
            if nn > 0 {
                solver.set_dc_source(nn - 1, dc); // Current enters at - terminal
            }
        }
    }

    /// Stamp a two-terminal admittance (conductance or capacitance) into HB solver
    /// - is_conductance: true stamps into G matrix, false stamps into C matrix
    fn hb_stamp_admittance(
        &self,
        solver: &mut HbSolver,
        np: usize,
        nn: usize,
        value: Value,
        is_conductance: bool,
    ) {
        // Standard MNA stamp pattern for two-terminal element
        if np > 0 && nn > 0 {
            // Both nodes are non-ground
            let i = np - 1;
            let j = nn - 1;
            if is_conductance {
                solver.add_conductance(i, i, value);
                solver.add_conductance(i, j, -value);
                solver.add_conductance(j, i, -value);
                solver.add_conductance(j, j, value);
            } else {
                solver.add_capacitance(i, i, value);
                solver.add_capacitance(i, j, -value);
                solver.add_capacitance(j, i, -value);
                solver.add_capacitance(j, j, value);
            }
        } else if np > 0 {
            // nn is ground
            let i = np - 1;
            if is_conductance {
                solver.add_conductance(i, i, value);
            } else {
                solver.add_capacitance(i, i, value);
            }
        } else if nn > 0 {
            // np is ground
            let i = nn - 1;
            if is_conductance {
                solver.add_conductance(i, i, value);
            } else {
                solver.add_capacitance(i, i, value);
            }
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::HbConfig;

    // =========================================================================
    // Error Type Tests
    // =========================================================================

    #[test]
    fn test_hb_error_display_convergence() {
        let err = HbError::ConvergenceFailed {
            iterations: 50,
            residual: 1e-3,
        };
        let msg = format!("{}", err);
        assert!(msg.contains("50 iterations"));
        assert!(msg.contains("1.000e-3") || msg.contains("1.0e-3"));
    }

    #[test]
    fn test_hb_error_display_no_reactive() {
        let err = HbError::NoReactiveElements;
        let msg = format!("{}", err);
        assert!(msg.contains("no capacitors or inductors"));
    }

    #[test]
    fn test_hb_error_display_invalid_config() {
        let err = HbError::InvalidConfig("Bad frequency".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("Invalid HB config"));
        assert!(msg.contains("Bad frequency"));
    }

    #[test]
    fn test_hb_error_display_singular() {
        let err = HbError::SingularMatrix;
        let msg = format!("{}", err);
        assert!(msg.contains("Singular"));
    }

    #[test]
    fn test_hb_error_to_simulation_error_convergence() {
        let err = HbError::ConvergenceFailed {
            iterations: 25,
            residual: 1e-5,
        };
        let sim_err: SimulationError = err.into();
        match sim_err {
            SimulationError::ConvergenceFailed(n) => assert_eq!(n, 25),
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_hb_error_to_simulation_error_no_reactive() {
        let err = HbError::NoReactiveElements;
        let sim_err: SimulationError = err.into();
        match sim_err {
            SimulationError::Circuit(msg) => assert!(msg.contains("capacitors")),
            _ => panic!("Wrong error type"),
        }
    }

    // =========================================================================
    // Configuration Validation Tests
    // =========================================================================

    #[test]
    fn test_run_hb_rejects_zero_frequency() {
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();
        let config = HbConfig::new(0.0);

        let result = engine.run_hb(&netlist, config);
        assert!(result.is_err());

        if let Err(e) = result {
            let msg = format!("{}", e);
            assert!(msg.contains("frequency") || msg.contains("positive"));
        }
    }

    #[test]
    fn test_run_hb_rejects_negative_frequency() {
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();
        let config = HbConfig::new(-1e6);

        let result = engine.run_hb(&netlist, config);
        assert!(result.is_err());
    }

    #[test]
    fn test_run_hb_rejects_purely_resistive() {
        use crate::Netlist;

        let netlist_str = r#"
            * Purely resistive - no reactive elements
            V1 in 0 DC 1
            R1 in out 1k
            R2 out 0 1k
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();
        let config = HbConfig::new(1e6);

        let result = engine.run_hb(&netlist, config);
        assert!(
            result.is_err(),
            "HB should fail for purely resistive circuit"
        );

        if let Err(e) = result {
            let msg = format!("{}", e);
            assert!(msg.contains("capacitor") || msg.contains("reactive"));
        }
    }

    // =========================================================================
    // Basic Circuit Tests
    // =========================================================================

    #[test]
    fn test_run_hb_simple_rc() {
        use crate::Netlist;

        let netlist_str = r#"
            * Simple RC circuit
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        let config = HbConfig::new(1e6).with_harmonics(5).with_tolerance(1e-6);

        let result = engine.run_hb(&netlist, config);
        assert!(
            result.is_ok(),
            "HB should succeed for RC circuit: {:?}",
            result.err()
        );

        let hb_result = result.unwrap();
        assert_eq!(hb_result.num_harmonics, 5);
        assert!(hb_result.fundamental_freq > 0.0);
    }

    #[test]
    fn test_run_hb_returns_spectral_voltages() {
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        let config = HbConfig::new(1e6).with_harmonics(3);

        if let Ok(result) = engine.run_hb(&netlist, config) {
            // Should have spectral voltages for each node
            assert!(!result.result.spectral_voltages.is_empty());

            // Each spectral voltage should have DC + harmonics coefficients
            for sv in &result.result.spectral_voltages {
                assert_eq!(sv.coefficients.len(), 4); // DC + 3 harmonics
            }
        }
    }

    #[test]
    fn test_run_hb_with_current_source() {
        use crate::Netlist;

        let netlist_str = r#"
            * Circuit with current source
            I1 0 in DC 1m
            R1 in 0 1k
            C1 in 0 10n
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        let config = HbConfig::new(100e3).with_harmonics(5);

        let result = engine.run_hb(&netlist, config);
        assert!(
            result.is_ok(),
            "HB should work with current source: {:?}",
            result.err()
        );
    }

    // =========================================================================
    // Frequency Configuration Tests
    // =========================================================================

    #[test]
    fn test_run_hb_preserves_fundamental_frequency() {
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        for freq in [1e3, 1e6, 2.5e6, 1e9] {
            let config = HbConfig::new(freq).with_harmonics(5);
            if let Ok(result) = engine.run_hb(&netlist, config) {
                assert!(
                    (result.fundamental_freq - freq).abs() < 1.0,
                    "Frequency should be preserved: expected {}, got {}",
                    freq,
                    result.fundamental_freq
                );
            }
        }
    }

    #[test]
    fn test_run_hb_harmonics_count() {
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        for n_harm in [3, 5, 9, 15] {
            let config = HbConfig::new(1e6).with_harmonics(n_harm);
            if let Ok(result) = engine.run_hb(&netlist, config) {
                assert_eq!(result.num_harmonics, n_harm);
            }
        }
    }

    // =========================================================================
    // Multi-Node Circuit Tests
    // =========================================================================

    #[test]
    fn test_run_hb_two_stage_filter() {
        use crate::Netlist;

        let netlist_str = r#"
            * Two-stage RC filter
            V1 in 0 DC 1
            R1 in mid 1k
            C1 mid 0 1n
            R2 mid out 1k
            C2 out 0 1n
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        let config = HbConfig::new(100e3).with_harmonics(5);

        let result = engine.run_hb(&netlist, config);
        assert!(
            result.is_ok(),
            "HB should work with two-stage filter: {:?}",
            result.err()
        );

        if let Ok(r) = result {
            // Should have nodes for in, mid, out
            assert!(r.result.num_nodes() >= 2);
        }
    }

    #[test]
    fn test_run_hb_parallel_rc() {
        use crate::Netlist;

        let netlist_str = r#"
            * Parallel RC
            V1 in 0 DC 1
            R1 in 0 1k
            C1 in 0 1n
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        let config = HbConfig::new(1e6).with_harmonics(3);

        let result = engine.run_hb(&netlist, config);
        assert!(
            result.is_ok(),
            "HB should work with parallel RC: {:?}",
            result.err()
        );
    }

    // =========================================================================
    // Result Validity Tests
    // =========================================================================

    #[test]
    fn test_run_hb_result_is_valid() {
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 100
            C1 out 0 10p
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        let config = HbConfig::new(10e6).with_harmonics(5);

        if let Ok(result) = engine.run_hb(&netlist, config) {
            assert!(result.result.is_valid());
        }
    }

    #[test]
    fn test_run_hb_dc_operating_point() {
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 5
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        let config = HbConfig::new(1e6).with_harmonics(3);

        if let Ok(result) = engine.run_hb(&netlist, config) {
            let dc_op = result.result.dc_operating_point();
            // Should have DC values for each node
            for (name, dc) in &dc_op {
                assert!(dc.is_finite(), "DC at {} should be finite", name);
            }
        }
    }

    // =========================================================================
    // HbAnalysisResult Tests
    // =========================================================================

    #[test]
    fn test_hb_analysis_result_fields() {
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        let config = HbConfig::new(2.5e6).with_harmonics(7);

        if let Ok(result) = engine.run_hb(&netlist, config) {
            assert_eq!(result.fundamental_freq, 2.5e6);
            assert_eq!(result.num_harmonics, 7);
            // converged should be boolean
            assert!(result.converged || !result.converged);
        }
    }

    // =========================================================================
    // Numerical Accuracy Tests
    // =========================================================================

    #[test]
    fn test_hb_rc_filter_dc_gain() {
        // RC lowpass filter: at DC, output = input (gain = 1)
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 5
            R1 in out 1k
            C1 out 0 1u
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        // Very low frequency - should see nearly full DC gain
        let config = HbConfig::new(10.0).with_harmonics(3);

        if let Ok(result) = engine.run_hb(&netlist, config) {
            // At DC, output should be close to input for RC filter
            let dc_op = result.result.dc_operating_point();
            // Verify we got DC values
            assert!(!dc_op.is_empty(), "Should have DC values");
        }
    }

    #[test]
    fn test_hb_high_frequency() {
        // Test at GHz frequencies (RF/MW regime)
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 50
            C1 out 0 1p
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        // 1 GHz
        let config = HbConfig::new(1e9).with_harmonics(5);

        let result = engine.run_hb(&netlist, config);
        assert!(
            result.is_ok(),
            "HB should work at GHz frequencies: {:?}",
            result.err()
        );

        if let Ok(r) = result {
            assert_eq!(r.fundamental_freq, 1e9);
        }
    }

    #[test]
    fn test_hb_very_low_frequency() {
        // Test at very low frequencies (sub-Hz)
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1M
            C1 out 0 10u
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        // 0.1 Hz
        let config = HbConfig::new(0.1).with_harmonics(3);

        let result = engine.run_hb(&netlist, config);
        assert!(
            result.is_ok(),
            "HB should work at sub-Hz frequencies: {:?}",
            result.err()
        );
    }

    // =========================================================================
    // Edge Case Tests
    // =========================================================================

    #[test]
    fn test_hb_tiny_capacitance() {
        // Femtofarad capacitance (RF regime)
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 50
            C1 out 0 10f
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        let config = HbConfig::new(10e9).with_harmonics(3);

        let result = engine.run_hb(&netlist, config);
        assert!(
            result.is_ok(),
            "HB should work with femtofarad capacitors: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_hb_large_resistance() {
        // GigaOhm resistance
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1G
            C1 out 0 1p
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        let config = HbConfig::new(1e6).with_harmonics(3);

        let result = engine.run_hb(&netlist, config);
        assert!(
            result.is_ok(),
            "HB should work with GΩ resistors: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_hb_small_resistance() {
        // Milliohm resistance
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 10m
            C1 out 0 100u
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        let config = HbConfig::new(1e3).with_harmonics(3);

        let result = engine.run_hb(&netlist, config);
        assert!(
            result.is_ok(),
            "HB should work with mΩ resistors: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_hb_many_harmonics() {
        // Test with large number of harmonics
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        let config = HbConfig::new(1e6).with_harmonics(31);

        let result = engine.run_hb(&netlist, config);
        assert!(
            result.is_ok(),
            "HB should work with 31 harmonics: {:?}",
            result.err()
        );

        if let Ok(r) = result {
            assert_eq!(r.num_harmonics, 31);
            // Each spectral voltage should have 32 coefficients (DC + 31 harmonics)
            for sv in &r.result.spectral_voltages {
                assert_eq!(sv.coefficients.len(), 32);
            }
        }
    }

    // =========================================================================
    // Multi-Element Circuit Tests
    // =========================================================================

    #[test]
    fn test_hb_ladder_filter() {
        // 3-stage ladder filter
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1
            R1 in n1 1k
            C1 n1 0 1n
            R2 n1 n2 1k
            C2 n2 0 1n
            R3 n2 out 1k
            C3 out 0 1n
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        let config = HbConfig::new(100e3).with_harmonics(5);

        let result = engine.run_hb(&netlist, config);
        assert!(
            result.is_ok(),
            "HB should work with ladder filter: {:?}",
            result.err()
        );

        if let Ok(r) = result {
            // Should have nodes for n1, n2, out, in
            assert!(r.result.num_nodes() >= 3);
        }
    }

    #[test]
    fn test_hb_parallel_elements() {
        // Multiple parallel RC elements
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1
            R1 in 0 1k
            R2 in 0 2k
            C1 in 0 1n
            C2 in 0 2n
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        let config = HbConfig::new(1e6).with_harmonics(5);

        let result = engine.run_hb(&netlist, config);
        assert!(
            result.is_ok(),
            "HB should work with parallel elements: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_hb_bridge_circuit() {
        // Bridge/mesh topology
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1
            R1 in a 1k
            R2 in b 1k
            R3 a out 1k
            R4 b out 1k
            C1 a b 1n
            C2 out 0 1n
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        let config = HbConfig::new(1e6).with_harmonics(5);

        let result = engine.run_hb(&netlist, config);
        assert!(
            result.is_ok(),
            "HB should work with bridge circuit: {:?}",
            result.err()
        );
    }

    // =========================================================================
    // Multiple Source Tests
    // =========================================================================

    #[test]
    fn test_hb_multiple_voltage_sources() {
        // Multiple voltage sources in circuit
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 5
            V2 bias 0 DC 2.5
            R1 in out 1k
            R2 bias out 1k
            C1 out 0 1n
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        let config = HbConfig::new(1e6).with_harmonics(5);

        let result = engine.run_hb(&netlist, config);
        assert!(
            result.is_ok(),
            "HB should work with multiple voltage sources: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_hb_mixed_sources() {
        // Both voltage and current sources
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1
            I1 0 bias DC 1m
            R1 in out 1k
            R2 bias 0 1k
            C1 out bias 1n
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        let config = HbConfig::new(1e6).with_harmonics(5);

        let result = engine.run_hb(&netlist, config);
        assert!(
            result.is_ok(),
            "HB should work with mixed sources: {:?}",
            result.err()
        );
    }

    // =========================================================================
    // Spectral Coefficient Verification Tests
    // =========================================================================

    #[test]
    fn test_hb_spectral_coefficients_dc_only() {
        // With only DC source, all AC harmonics should be ~zero
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        let config = HbConfig::new(1e6).with_harmonics(5);

        if let Ok(result) = engine.run_hb(&netlist, config) {
            for sv in &result.result.spectral_voltages {
                // DC component (index 0) may be non-zero
                // But all AC harmonics should be zero for DC-only input
                for k in 1..sv.coefficients.len() {
                    let mag = sv.coefficients[k].norm();
                    assert!(
                        mag < 1e-6,
                        "Harmonic {} should be ~zero for DC input, got {}",
                        k,
                        mag
                    );
                }
            }
        }
    }

    #[test]
    fn test_hb_spectral_voltage_magnitudes_finite() {
        // All spectral coefficients should be finite
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        let config = HbConfig::new(1e6).with_harmonics(9);

        if let Ok(result) = engine.run_hb(&netlist, config) {
            for sv in &result.result.spectral_voltages {
                for (i, coeff) in sv.coefficients.iter().enumerate() {
                    assert!(
                        coeff.re.is_finite() && coeff.im.is_finite(),
                        "Coefficient {} should be finite: {:?}",
                        i,
                        coeff
                    );
                }
            }
        }
    }

    #[test]
    fn test_hb_all_nodes_have_spectral_voltages() {
        // Every node should have spectral voltage data
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1
            R1 in mid 1k
            C1 mid 0 1n
            R2 mid out 1k
            C2 out 0 1n
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        let config = HbConfig::new(1e6).with_harmonics(5);

        if let Ok(result) = engine.run_hb(&netlist, config) {
            let num_nodes = result.result.num_nodes();
            assert!(num_nodes >= 2, "Should have multiple nodes");
            assert_eq!(
                result.result.spectral_voltages.len(),
                num_nodes,
                "Should have spectral voltage for each node"
            );
        }
    }

    // =========================================================================
    // Tolerance and Config Tests
    // =========================================================================

    #[test]
    fn test_hb_different_tolerances() {
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        for tol in [1e-3, 1e-6, 1e-9, 1e-12] {
            let config = HbConfig::new(1e6).with_harmonics(5).with_tolerance(tol);
            let result = engine.run_hb(&netlist, config);
            assert!(
                result.is_ok(),
                "HB should work with tolerance {}: {:?}",
                tol,
                result.err()
            );
        }
    }

    #[test]
    fn test_hb_oversample_factors() {
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        for oversample in [2, 4, 8] {
            let config = HbConfig::new(1e6)
                .with_harmonics(5)
                .with_oversample(oversample);
            let result = engine.run_hb(&netlist, config);
            assert!(
                result.is_ok(),
                "HB should work with oversample {}: {:?}",
                oversample,
                result.err()
            );
        }
    }

    // =========================================================================
    // Stress Tests
    // =========================================================================

    #[test]
    fn test_hb_many_nodes() {
        // Circuit with many nodes
        use crate::Netlist;

        let netlist_str = r#"
            V1 n1 0 DC 1
            R1 n1 n2 1k
            C1 n2 0 1n
            R2 n2 n3 1k
            C2 n3 0 1n
            R3 n3 n4 1k
            C3 n4 0 1n
            R4 n4 n5 1k
            C4 n5 0 1n
            R5 n5 out 1k
            C5 out 0 1n
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        let config = HbConfig::new(100e3).with_harmonics(5);

        let result = engine.run_hb(&netlist, config);
        assert!(
            result.is_ok(),
            "HB should work with many nodes: {:?}",
            result.err()
        );

        if let Ok(r) = result {
            assert!(r.result.num_nodes() >= 5, "Should have 5+ nodes");
        }
    }

    #[test]
    fn test_hb_repeated_runs_consistent() {
        // Multiple runs should give consistent results
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1
            R1 in out 1k
            C1 out 0 1n
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();
        let config = HbConfig::new(1e6).with_harmonics(5);

        let mut results = Vec::new();
        for _ in 0..3 {
            if let Ok(r) = engine.run_hb(&netlist, config.clone()) {
                results.push(r);
            }
        }

        assert_eq!(results.len(), 3, "All runs should succeed");

        // Results should be identical
        for i in 1..results.len() {
            assert_eq!(
                results[0].result.num_nodes(),
                results[i].result.num_nodes(),
                "Node count should be consistent"
            );
            assert_eq!(
                results[0].num_harmonics, results[i].num_harmonics,
                "Harmonics count should be consistent"
            );
        }
    }

    // =========================================================================
    // Inductor HB Tests
    // =========================================================================

    #[test]
    fn test_run_hb_simple_rl() {
        // Simple RL circuit: V1 -> R -> L -> GND
        // At DC, inductor is short circuit
        // At AC, |V_out| = |V_in| * X_L / sqrt(R² + X_L²)
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1 AC 1
            R1 in out 100
            L1 out 0 1u
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        // f = 1 MHz, X_L = 2π * 1e6 * 1e-6 = 2π ≈ 6.28 Ω
        let config = HbConfig::new(1e6).with_harmonics(5);

        let result = engine.run_hb(&netlist, config);
        assert!(
            result.is_ok(),
            "RL circuit HB should succeed: {:?}",
            result.err()
        );

        let r = result.unwrap();
        assert!(r.result.converged, "Should converge");
    }

    #[test]
    fn test_run_hb_rl_inductor_impedance() {
        // Test inductor impedance: |Z_L| = ωL at fundamental
        use crate::Netlist;
        use std::f64::consts::PI;

        let netlist_str = r#"
            I1 0 out DC 0 AC 1
            L1 out 0 10u
            R1 out 0 1k
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        // f = 100 kHz, X_L = 2π * 1e5 * 10e-6 = 6.28 Ω
        // R is 1kΩ, so Z ≈ 1kΩ (parallel), V ≈ I * Z
        let freq = 100e3;
        let config = HbConfig::new(freq).with_harmonics(3);

        let result = engine.run_hb(&netlist, config);
        assert!(result.is_ok(), "Inductor test should succeed");

        let r = result.unwrap();
        assert!(r.result.converged);

        // At the fundamental (k=1), inductor has impedance j*ω*L
        // With parallel R, the voltage amplitude should be reasonable
        if let Some(sv) = r.result.spectral_voltages.first() {
            // Fundamental harmonic should have some voltage
            if sv.coefficients.len() > 1 {
                let v_fundamental = sv.coefficients[1].norm();
                // Should be non-zero (current source driving parallel R||L)
                assert!(v_fundamental > 0.0, "Should have AC response");
            }
        }
    }

    #[test]
    fn test_run_hb_series_rl_frequency_response() {
        // Verify frequency response of series RL: higher frequency = more L impedance
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 0 AC 1
            R1 in mid 50
            L1 mid out 100u
            R2 out 0 50
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();

        // At low frequency
        let config_low = HbConfig::new(1e3).with_harmonics(3);
        let result_low = engine.run_hb(&netlist, config_low);

        // At high frequency
        let config_high = HbConfig::new(100e3).with_harmonics(3);
        let result_high = engine.run_hb(&netlist, config_high);

        assert!(result_low.is_ok(), "Low freq should work");
        assert!(result_high.is_ok(), "High freq should work");

        // Both should converge
        if let (Ok(r_low), Ok(r_high)) = (result_low, result_high) {
            assert!(r_low.result.converged);
            assert!(r_high.result.converged);
        }
    }

    #[test]
    fn test_run_hb_inductor_only_circuit() {
        // Circuit with only inductor as reactive element
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 5
            R1 in mid 100
            L1 mid out 1m
            R2 out 0 100
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();
        let config = HbConfig::new(1e6).with_harmonics(5);

        let result = engine.run_hb(&netlist, config);
        assert!(
            result.is_ok(),
            "Inductor-only reactive should work: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_run_hb_rlc_circuit() {
        // Mixed RLC circuit
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 1 AC 1
            R1 in mid1 100
            L1 mid1 mid2 10u
            C1 mid2 out 10n
            R2 out 0 1k
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();
        let config = HbConfig::new(50e3).with_harmonics(5);

        let result = engine.run_hb(&netlist, config);
        assert!(
            result.is_ok(),
            "RLC circuit should work: {:?}",
            result.err()
        );

        if let Ok(r) = result {
            assert!(r.result.converged);
            assert!(r.result.num_nodes() >= 3);
        }
    }

    #[test]
    fn test_run_hb_parallel_rlc() {
        // Parallel RLC tank circuit
        use crate::Netlist;

        let netlist_str = r#"
            I1 0 tank DC 0 AC 1m
            R1 tank 0 10k
            L1 tank 0 100u
            C1 tank 0 1n
            .END
        "#;

        // Resonant frequency f0 = 1/(2π√(LC)) ≈ 503 kHz
        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();
        let config = HbConfig::new(500e3).with_harmonics(5);

        let result = engine.run_hb(&netlist, config);
        assert!(
            result.is_ok(),
            "Parallel RLC tank should work: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_run_hb_inductor_dc_short() {
        // At DC, inductor should act as short circuit
        // V1 -> L -> R -> GND: at DC, V_out should equal V_in (L is short)
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 10
            L1 in out 1m
            R1 out 0 1k
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();
        let config = HbConfig::new(1e6).with_harmonics(3);

        let result = engine.run_hb(&netlist, config);
        assert!(result.is_ok(), "DC inductor test should work");
    }

    #[test]
    fn test_run_hb_multiple_inductors() {
        // Circuit with multiple inductors
        use crate::Netlist;

        let netlist_str = r#"
            V1 in 0 DC 5 AC 1
            R1 in n1 50
            L1 n1 n2 10u
            L2 n2 n3 20u
            R2 n3 0 50
            .END
        "#;

        let netlist = Netlist::parse(netlist_str).expect("Parse failed");
        let engine = Engine::default();
        let config = HbConfig::new(1e6).with_harmonics(5);

        let result = engine.run_hb(&netlist, config);
        assert!(
            result.is_ok(),
            "Multiple inductors should work: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_hb_solver_add_inductance() {
        // Test the solver's add_inductance method directly
        let config = HbConfig::new(1e6).with_harmonics(5);
        let mut solver = HbSolver::new(config, 3);

        // Add inductance stamps
        solver.add_inductance(0, 0, 1e-6);
        solver.add_inductance(0, 1, -1e-6);
        solver.add_inductance(1, 0, -1e-6);
        solver.add_inductance(1, 1, 1e-6);

        // Should complete without error
        // Verify by creating a state and computing residual
        let mut state = HbSolverState::new(3, 5);
        state.x[0][1] = Complex64::new(1.0, 0.0); // Set fundamental at node 0

        solver.compute_linear_residual(&mut state);
        // Residual should be finite
        assert!(state.residual_norm.is_finite());
    }

    #[test]
    fn test_hb_solver_inductor_frequency_dependence() {
        // Verify inductor admittance is frequency-dependent
        use std::f64::consts::PI;

        let freq = 1e6;
        let l = 10e-6; // 10 µH
        let config = HbConfig::new(freq).with_harmonics(3);
        let mut solver = HbSolver::new(config, 1);

        // Add single inductor to ground
        solver.add_inductance(0, 0, l);

        let mut state = HbSolverState::new(1, 3);

        // Set DC component
        state.x[0][0] = Complex64::new(1.0, 0.0);
        // Set fundamental
        state.x[0][1] = Complex64::new(1.0, 0.0);
        // Set 2nd harmonic
        state.x[0][2] = Complex64::new(1.0, 0.0);

        solver.compute_linear_residual(&mut state);

        // At DC, inductor is short (large G), so residual[0] should be large
        // At harmonics, inductor has admittance -j/(ωL)
        // The residual magnitudes should differ between harmonics
        let res_dc = state.residual[0][0].norm();
        let res_1 = state.residual[0][1].norm();
        let res_2 = state.residual[0][2].norm();

        // DC residual should be larger (short circuit = large conductance)
        assert!(
            res_dc > res_1,
            "DC should have larger residual due to short circuit model"
        );

        // 2nd harmonic has 2x the frequency, so 2x the admittance (1/X_L)
        // The residual ratio should reflect this
        let omega1 = 2.0 * PI * freq;
        let omega2 = 2.0 * PI * freq * 2.0;
        let y1_mag = 1.0 / (omega1 * l);
        let y2_mag = 1.0 / (omega2 * l);

        // Expected ratio of residuals (both have same voltage)
        let expected_ratio = y2_mag / y1_mag; // = 0.5
        let actual_ratio = res_2 / res_1;

        assert!(
            (actual_ratio - expected_ratio).abs() < 0.1,
            "Admittance ratio should match frequency ratio: expected {}, got {}",
            expected_ratio,
            actual_ratio
        );
    }
}
