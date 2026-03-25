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

        // Check for reactive elements (capacitors or inductors)
        let has_reactive = !circuit.capacitors.is_empty() || !circuit.inductors.names.is_empty();
        if !has_reactive {
            return Err(HbError::NoReactiveElements.into());
        }
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

    /// Build node names from circuit node map
    fn hb_build_node_names(&self, circuit: &CircuitData, num_nodes: usize) -> Vec<String> {
        let mut node_names = circuit.node_names_sorted();
        if node_names.len() > num_nodes {
            node_names.truncate(num_nodes);
        } else if node_names.len() < num_nodes {
            let mut synthetic_index = node_names.len() + 1;
            while node_names.len() < num_nodes {
                node_names.push(format!("n{}", synthetic_index));
                synthetic_index += 1;
            }
        }
        node_names
    }

    fn hb_collect_drive_tones(config: &HbConfig) -> Result<Vec<HbDriveTone>, SimulationError> {
        if config.tones.is_empty() {
            return Ok(vec![HbDriveTone::broadcast(1)]);
        }

        if !config.fundamental_freq.is_finite() || config.fundamental_freq <= 0.0 {
            return Err(HbError::InvalidConfig(
                "HB multi-tone requires a positive basis fundamental frequency".to_string(),
            )
            .into());
        }

        let mut tones: BTreeSet<(usize, Option<String>)> = BTreeSet::new();
        for tone in &config.tones {
            if !tone.frequency.is_finite() || tone.frequency <= 0.0 {
                return Err(HbError::InvalidConfig(format!(
                    "HB tone '{}' has invalid frequency {}",
                    tone.name, tone.frequency
                ))
                .into());
            }

            let ratio = tone.frequency / config.fundamental_freq;
            let harmonic = ratio.round();
            let abs_error = (ratio - harmonic).abs();
            let rel_error = abs_error / harmonic.abs().max(1.0);

            if !harmonic.is_finite() || harmonic < 1.0 {
                return Err(HbError::InvalidConfig(format!(
                    "HB tone '{}' does not map to a positive harmonic of f0={:.6e} Hz",
                    tone.name, config.fundamental_freq
                ))
                .into());
            }
            if rel_error > 1e-9 {
                return Err(HbError::InvalidConfig(format!(
                    "HB tone '{}' at {:.6e} Hz is not an integer harmonic of f0={:.6e} Hz",
                    tone.name, tone.frequency, config.fundamental_freq
                ))
                .into());
            }

            let harmonic = harmonic as usize;
            if harmonic > config.num_harmonics {
                return Err(HbError::InvalidConfig(format!(
                    "HB tone '{}' maps to harmonic {} but num_harmonics is {}",
                    tone.name, harmonic, config.num_harmonics
                ))
                .into());
            }
            let source_filter = tone
                .source_name
                .as_ref()
                .map(|name| name.trim())
                .filter(|name| !name.is_empty())
                .map(|name| name.to_ascii_lowercase());
            tones.insert((harmonic, source_filter));
        }

        let collected: Vec<HbDriveTone> = tones
            .into_iter()
            .map(|(harmonic, source_filter)| HbDriveTone {
                harmonic,
                source_filter,
            })
            .collect();
        if collected.is_empty() {
            Ok(vec![HbDriveTone::broadcast(1)])
        } else {
            Ok(collected)
        }
    }

    fn hb_validate_drive_tone_sources(
        circuit: &CircuitData,
        drive_tones: &[HbDriveTone],
    ) -> Result<(), SimulationError> {
        for tone in drive_tones {
            let Some(source_filter) = tone.source_filter.as_deref() else {
                continue;
            };
            let present_in_voltage = circuit
                .voltage_sources
                .names
                .iter()
                .any(|name| source_filter.eq_ignore_ascii_case(name));
            let present_in_current = circuit
                .current_sources
                .names
                .iter()
                .any(|name| source_filter.eq_ignore_ascii_case(name));
            if !(present_in_voltage || present_in_current) {
                return Err(HbError::InvalidConfig(format!(
                    "HB tone source '{}' is not present in circuit independent sources",
                    source_filter
                ))
                .into());
            }
        }
        Ok(())
    }

    fn hb_drive_harmonics_for_source(drive_tones: &[HbDriveTone], source_name: &str) -> Vec<usize> {
        let mut harmonics: Vec<usize> = drive_tones
            .iter()
            .filter(|tone| tone.matches_source(source_name))
            .map(|tone| tone.harmonic)
            .collect();
        harmonics.sort_unstable();
        harmonics.dedup();
        harmonics
    }

    fn hb_has_supported_nonlinear_devices(circuit: &CircuitData, num_nodes: usize) -> bool {
        !circuit.diodes.is_empty()
            || !circuit.bjts.is_empty()
            || !circuit.mosfets.is_empty()
            || !circuit.jfets.is_empty()
            || !circuit.vswitches.is_empty()
            || circuit
                .iswitches
                .iter()
                .any(|sw| Self::hb_resolve_iswitch_control(circuit, sw, num_nodes).is_ok())
            || {
                #[cfg(feature = "veriloga")]
                {
                    circuit.has_veriloga_devices()
                }
                #[cfg(not(feature = "veriloga"))]
                {
                    false
                }
            }
    }

    fn hb_unsupported_nonlinear_device_summary(
        circuit: &CircuitData,
        num_nodes: usize,
    ) -> Option<String> {
        let mut kinds: Vec<String> = Vec::new();

        let unsupported_iswitch = circuit
            .iswitches
            .iter()
            .filter(|sw| Self::hb_resolve_iswitch_control(circuit, sw, num_nodes).is_err())
            .count();
        if unsupported_iswitch > 0 {
            kinds.push(format!(
                "{} current switch(es) (HB requires static control-source waveforms for ISwitch control branches)",
                unsupported_iswitch
            ));
        }
        if kinds.is_empty() {
            None
        } else {
            Some(kinds.join(", "))
        }
    }

    fn hb_extract_static_source_voltage(
        spec: Option<&SourceSpec>,
        fallback_dc: Value,
    ) -> Option<Value> {
        match spec {
            None => Some(fallback_dc),
            Some(SourceSpec::Dc(v)) => Some(*v),
            Some(SourceSpec::DcAc {
                dc_value,
                ac_magnitude,
                ..
            }) if ac_magnitude.abs() <= HB_ZERO_SENSE_TOL => Some(*dc_value),
            Some(SourceSpec::DcTransient {
                dc_value,
                transient,
            }) => Self::hb_extract_static_source_voltage(Some(transient), *dc_value),
            Some(SourceSpec::Ac { magnitude, .. }) if magnitude.abs() <= HB_ZERO_SENSE_TOL => {
                Some(0.0)
            }
            Some(SourceSpec::Sin {
                offset, amplitude, ..
            }) if amplitude.abs() <= HB_ZERO_SENSE_TOL => Some(*offset),
            Some(SourceSpec::Pulse { v1, v2, .. }) if (v2 - v1).abs() <= HB_ZERO_SENSE_TOL => {
                Some(*v1)
            }
            Some(SourceSpec::Exp { v1, v2, .. }) if (v2 - v1).abs() <= HB_ZERO_SENSE_TOL => {
                Some(*v1)
            }
            Some(SourceSpec::Pwl { points }) => {
                let first = points.first().map(|(_, value)| *value)?;
                if points
                    .iter()
                    .all(|(_, value)| (*value - first).abs() <= HB_ZERO_SENSE_TOL)
                {
                    Some(first)
                } else {
                    None
                }
            }
            Some(SourceSpec::PwlFile { .. }) => None,
            _ => None,
        }
    }

    fn hb_resolve_iswitch_control(
        circuit: &CircuitData,
        sw: &crate::device::CurrentSwitch,
        num_nodes: usize,
    ) -> Result<HbCurrentSwitchControl, ()> {
        let Some(ctrl_branch_matrix_idx) = sw.ctrl_branch else {
            return Err(());
        };
        if ctrl_branch_matrix_idx <= num_nodes {
            return Err(());
        }
        let ctrl_branch_ordinal = ctrl_branch_matrix_idx - num_nodes;
        let Some(vsrc_idx) = circuit
            .voltage_sources
            .branch_indices
            .iter()
            .position(|&ordinal| ordinal == ctrl_branch_ordinal)
        else {
            return Err(());
        };

        let dc = circuit
            .voltage_sources
            .dc_values
            .get(vsrc_idx)
            .copied()
            .unwrap_or(0.0);
        let ac_mag = circuit
            .voltage_sources
            .ac_magnitudes
            .get(vsrc_idx)
            .copied()
            .unwrap_or(0.0);
        let spec = circuit
            .voltage_sources
            .source_specs
            .get(vsrc_idx)
            .and_then(|s| s.as_ref());
        if ac_mag.abs() > HB_ZERO_SENSE_TOL {
            return Err(());
        }
        let static_voltage = Self::hb_extract_static_source_voltage(spec, dc).ok_or(())?;

        let ctrl_pos =
            Self::hb_node_to_solver_index(circuit.voltage_sources.node_pos[vsrc_idx], num_nodes);
        let ctrl_neg =
            Self::hb_node_to_solver_index(circuit.voltage_sources.node_neg[vsrc_idx], num_nodes);
        Ok(HbCurrentSwitchControl {
            ctrl_pos,
            ctrl_neg,
            control_current_bias: static_voltage * HB_NORTON_G,
        })
    }

    #[inline]
    fn hb_node_to_solver_index(node: usize, num_nodes: usize) -> usize {
        if node == 0 { num_nodes } else { node - 1 }
    }

    fn hb_stamp_supported_nonlinear_devices(
        &self,
        circuit: &CircuitData,
        solver: &mut HbSolver,
        num_nodes: usize,
    ) {
        for diode in &circuit.diodes.devices {
            let anode = Self::hb_node_to_solver_index(diode.node_anode, num_nodes);
            let cathode = Self::hb_node_to_solver_index(diode.node_cathode, num_nodes);
            solver.add_diode(anode, cathode, diode.is, diode.n);
        }

        for bjt in &circuit.bjts.devices {
            let collector = Self::hb_node_to_solver_index(bjt.node_collector, num_nodes);
            let base = Self::hb_node_to_solver_index(bjt.node_base, num_nodes);
            let emitter = Self::hb_node_to_solver_index(bjt.node_emitter, num_nodes);
            match bjt.bjt_type {
                crate::device::BjtType::Npn => {
                    solver.add_npn_bjt(collector, base, emitter, bjt.is, bjt.bf);
                }
                crate::device::BjtType::Pnp => {
                    solver.add_pnp_bjt(collector, base, emitter, bjt.is, bjt.bf);
                }
            }
        }

        for mos in &circuit.mosfets.devices {
            let drain = Self::hb_node_to_solver_index(mos.node_drain, num_nodes);
            let gate = Self::hb_node_to_solver_index(mos.node_gate, num_nodes);
            let source = Self::hb_node_to_solver_index(mos.node_source, num_nodes);
            let bulk = Self::hb_node_to_solver_index(mos.node_bulk, num_nodes);
            let kp = mos.kp.max(1e-18);
            match mos.mos_type {
                crate::device::MosType::Nmos => {
                    solver.add_nmos(drain, gate, source, bulk, kp, mos.vto);
                }
                crate::device::MosType::Pmos => {
                    solver.add_pmos(drain, gate, source, bulk, kp, mos.vto.abs());
                }
            }
        }

        for jfet in &circuit.jfets {
            let drain = Self::hb_node_to_solver_index(jfet.drain, num_nodes);
            let gate = Self::hb_node_to_solver_index(jfet.gate, num_nodes);
            let source = Self::hb_node_to_solver_index(jfet.source, num_nodes);
            let beta = jfet.params.beta.max(1e-18);
            match jfet.jfet_type {
                crate::device::JfetType::NJF => {
                    solver.add_njfet(
                        drain,
                        gate,
                        source,
                        jfet.params.vto,
                        beta,
                        jfet.params.lambda,
                    );
                }
                crate::device::JfetType::PJF => {
                    solver.add_pjfet(
                        drain,
                        gate,
                        source,
                        jfet.params.vto,
                        beta,
                        jfet.params.lambda,
                    );
                }
            }
        }

        for sw in &circuit.vswitches {
            let node_pos = Self::hb_node_to_solver_index(sw.node_pos, num_nodes);
            let node_neg = Self::hb_node_to_solver_index(sw.node_neg, num_nodes);
            let ctrl_pos = Self::hb_node_to_solver_index(sw.ctrl_pos, num_nodes);
            let ctrl_neg = Self::hb_node_to_solver_index(sw.ctrl_neg, num_nodes);
            solver.add_voltage_switch(
                node_pos, node_neg, ctrl_pos, ctrl_neg, sw.vt, sw.vh, sw.ron, sw.roff, sw.smooth,
            );
        }

        for sw in &circuit.iswitches {
            let Ok(ctrl) = Self::hb_resolve_iswitch_control(circuit, sw, num_nodes) else {
                continue;
            };
            let node_pos = Self::hb_node_to_solver_index(sw.node_pos, num_nodes);
            let node_neg = Self::hb_node_to_solver_index(sw.node_neg, num_nodes);
            solver.add_current_switch(
                node_pos,
                node_neg,
                ctrl.ctrl_pos,
                ctrl.ctrl_neg,
                sw.it + ctrl.control_current_bias,
                sw.ih,
                sw.ron,
                sw.roff,
                sw.smooth,
                HB_NORTON_G,
            );
        }

        #[cfg(feature = "veriloga")]
        for device in circuit.veriloga_devices().iter() {
            solver.add_veriloga_device(device.clone());
        }
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

    /// Stamp ideal voltage sources into HB solver using MNA branch equations.
    fn hb_stamp_voltage_sources(
        &self,
        circuit: &CircuitData,
        solver: &mut HbSolver,
        drive_tones: &[HbDriveTone],
    ) {
        for i in 0..circuit.voltage_sources.len() {
            let np = circuit.voltage_sources.node_pos[i];
            let nn = circuit.voltage_sources.node_neg[i];
            let dc = circuit.voltage_sources.dc_values[i];
            let ac_mag = circuit
                .voltage_sources
                .ac_magnitudes
                .get(i)
                .copied()
                .unwrap_or(0.0);
            let ac_phase = circuit
                .voltage_sources
                .ac_phases
                .get(i)
                .copied()
                .unwrap_or(0.0);
            let source_name = circuit
                .voltage_sources
                .names
                .get(i)
                .map(|name| name.as_str())
                .unwrap_or("");
            let harmonics = Self::hb_drive_harmonics_for_source(drive_tones, source_name);
            if ac_mag.abs() <= 1e-30 || harmonics.is_empty() {
                solver.add_voltage_source_branch(np, nn, dc);
                continue;
            }

            let harmonic_terms: Vec<(usize, Value, Value)> = harmonics
                .iter()
                .copied()
                .map(|harmonic| (harmonic, ac_mag, ac_phase))
                .collect();
            solver.add_voltage_source_branch_harmonics(np, nn, dc, &harmonic_terms);
        }
    }

    /// Stamp ideal voltage sources as stiff Norton equivalents for nonlinear HB.
    ///
    /// Nonlinear HB Newton currently solves in node-voltage space only. Converting
    /// ideal voltage sources to Norton form avoids branch-current unknowns while
    /// preserving source waveforms with a very small equivalent source resistance.
    fn hb_stamp_voltage_sources_norton(
        &self,
        circuit: &CircuitData,
        solver: &mut HbSolver,
        drive_tones: &[HbDriveTone],
    ) {
        for i in 0..circuit.voltage_sources.len() {
            let np = circuit.voltage_sources.node_pos[i];
            let nn = circuit.voltage_sources.node_neg[i];
            let dc = circuit.voltage_sources.dc_values[i];
            let ac_mag = circuit
                .voltage_sources
                .ac_magnitudes
                .get(i)
                .copied()
                .unwrap_or(0.0);
            let ac_phase = circuit
                .voltage_sources
                .ac_phases
                .get(i)
                .copied()
                .unwrap_or(0.0);
            let source_name = circuit
                .voltage_sources
                .names
                .get(i)
                .map(|name| name.as_str())
                .unwrap_or("");
            let harmonics = Self::hb_drive_harmonics_for_source(drive_tones, source_name);

            self.hb_stamp_admittance(solver, np, nn, HB_NORTON_G, true);

            let i_dc = dc * HB_NORTON_G;
            if np > 0 {
                solver.add_dc_source(np - 1, -i_dc);
            }
            if nn > 0 {
                solver.add_dc_source(nn - 1, i_dc);
            }

            let i_ac = ac_mag * HB_NORTON_G;
            if i_ac.abs() > 1e-30 && !harmonics.is_empty() {
                for harmonic in harmonics {
                    if np > 0 {
                        solver.add_harmonic_source(np - 1, harmonic, -i_ac, ac_phase);
                    }
                    if nn > 0 {
                        solver.add_harmonic_source(nn - 1, harmonic, i_ac, ac_phase);
                    }
                }
            }
        }
    }

    /// Stamp current sources into HB solver
    ///
    /// Stamps both DC and AC components:
    /// - DC component goes into harmonic 0
    /// - AC component is applied to configured HB drive harmonics with magnitude and phase
    fn hb_stamp_current_sources(
        &self,
        circuit: &CircuitData,
        solver: &mut HbSolver,
        drive_tones: &[HbDriveTone],
    ) {
        for i in 0..circuit.current_sources.len() {
            let np = circuit.current_sources.node_pos[i];
            let nn = circuit.current_sources.node_neg[i];
            let dc = circuit.current_sources.dc_values[i];

            // Stamp DC component (harmonic 0)
            if np > 0 {
                solver.add_dc_source(np - 1, -dc); // Current leaves at + terminal
            }
            if nn > 0 {
                solver.add_dc_source(nn - 1, dc); // Current enters at - terminal
            }

            // Stamp AC component across configured drive harmonics.
            let ac_mag = circuit
                .current_sources
                .ac_magnitudes
                .get(i)
                .copied()
                .unwrap_or(0.0);
            let ac_phase = circuit
                .current_sources
                .ac_phases
                .get(i)
                .copied()
                .unwrap_or(0.0);
            let source_name = circuit
                .current_sources
                .names
                .get(i)
                .map(|name| name.as_str())
                .unwrap_or("");
            let harmonics = Self::hb_drive_harmonics_for_source(drive_tones, source_name);

            if ac_mag.abs() > 1e-30 {
                for harmonic in harmonics {
                    if np > 0 {
                        // Current leaves at + terminal.
                        solver.add_harmonic_source(np - 1, harmonic, -ac_mag, ac_phase);
                    }
                    if nn > 0 {
                        // Current enters at - terminal.
                        solver.add_harmonic_source(nn - 1, harmonic, ac_mag, ac_phase);
                    }
                }
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
mod tests;
