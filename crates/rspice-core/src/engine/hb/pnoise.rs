//! Periodic noise (pnoise) analysis: cyclostationary noise folding around a
//! harmonic-balance operating point.
//!
//! For every sweep offset one adjoint conversion-matrix solve yields the
//! transfer from a unit current at every (node, sideband) to the output at
//! the analysis frequency; each noise source then contributes through the
//! full sideband correlation of its periodically modulated intensity. This
//! captures what the stationary approximation cannot: noise transferred
//! through the LO-modulated small-signal parameters (switching mixers,
//! choppers, samplers), and shot noise that switches on and off with its
//! bias current.

use super::*;
use crate::analysis::HbSolverState;
use crate::analysis::advanced::harmonic_balance::{HbConfig, PeriodicNoiseSource};

/// Result of periodic noise analysis.
#[derive(Debug, Clone)]
pub struct PnoiseAnalysisResult {
    /// Offset frequencies (Hz), the output analysis frequencies.
    pub frequencies: Vec<Value>,
    /// Output noise voltage PSD at each offset (V^2/Hz).
    pub output_noise: Vec<Value>,
    /// Large-signal fundamental (Hz).
    pub fundamental_freq: Value,
    /// Whether the operating-point solve converged.
    pub converged: bool,
}

impl Engine {
    /// Run periodic noise analysis at `output_node` over `offsets`.
    ///
    /// `fundamental_freq` is the large-signal periodicity; `max_sideband`
    /// bounds the folding range (sidebands -K..=K participate). Resistor
    /// thermal noise is stationary; junction shot noise and FET channel
    /// thermal noise are modulated by the periodic operating point.
    pub fn run_pnoise(
        &self,
        netlist: &Netlist,
        fundamental_freq: Value,
        offsets: &[Value],
        output_node: &str,
        max_sideband: i32,
    ) -> Result<PnoiseAnalysisResult, SimulationError> {
        if !fundamental_freq.is_finite() || fundamental_freq <= 0.0 {
            return Err(SimulationError::Circuit(
                "pnoise requires a positive fundamental frequency".to_string(),
            ));
        }
        if offsets.is_empty() {
            return Err(SimulationError::Circuit(
                "pnoise frequency sweep is empty".to_string(),
            ));
        }
        let max_sideband = max_sideband.max(1);

        let circuit = self.build_circuit(netlist)?;
        let num_nodes = circuit.num_nodes();
        if num_nodes == 0 {
            return Err(SimulationError::Circuit("Circuit has no nodes".to_string()));
        }
        if let Some(summary) = Self::hb_unsupported_nonlinear_device_summary(&circuit, num_nodes) {
            return Err(HbError::UnsupportedNonlinearDevices(summary).into());
        }

        let span = (2 * max_sideband).unsigned_abs() as usize;
        let op_harmonics = span.max(8);
        let hb_config = HbConfig::new(fundamental_freq)
            .with_harmonics(op_harmonics)
            .with_oversample(4);
        let drive_tones = Self::hb_collect_drive_tones(&hb_config)?;

        let mut solver = HbSolver::new(hb_config, num_nodes);
        let node_names = self.hb_build_node_names(&circuit, num_nodes);
        solver.set_node_names(node_names.clone());

        self.hb_stamp_resistors(&circuit, &mut solver);
        self.hb_stamp_capacitors(&circuit, &mut solver);
        self.hb_stamp_inductors(&circuit, &mut solver);
        self.hb_stamp_voltage_sources_norton(&circuit, &mut solver, &drive_tones);
        self.hb_stamp_current_sources(&circuit, &mut solver, &drive_tones);

        let has_nonlinear = Self::hb_has_supported_nonlinear_devices(&circuit, num_nodes);
        if has_nonlinear {
            self.hb_stamp_supported_nonlinear_devices(&circuit, &mut solver, num_nodes);
        }

        let mut state = HbSolverState::new(num_nodes, op_harmonics);
        if has_nonlinear {
            solver.solve_newton(&mut state).map_err(|e| {
                SimulationError::Circuit(format!("pnoise operating-point solve failed: {e}"))
            })?;
        } else {
            solver.solve_linear(&mut state).map_err(|e| {
                SimulationError::Circuit(format!("pnoise operating-point solve failed: {e}"))
            })?;
        }

        let out_idx = node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(output_node.trim()))
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "pnoise output node '{output_node}' not found in circuit nodes"
                ))
            })?;

        let temperature = self.config.temperature;
        let k_b = 1.380649e-23;

        // Stationary resistor thermal sources: 4kT*G between the resistor
        // terminals (DC-only intensity spectrum).
        let mut sources: Vec<PeriodicNoiseSource> = Vec::new();
        for i in 0..circuit.resistors.len() {
            let g = circuit.resistors.conductances[i];
            if !(g.is_finite() && g > 0.0) {
                continue;
            }
            let np = circuit.resistors.stamps[i].pp.row;
            let nn = circuit.resistors.stamps[i].nn.row;
            sources.push(PeriodicNoiseSource {
                node_pos: Self::hb_node_to_solver_index(np, num_nodes),
                node_neg: Self::hb_node_to_solver_index(nn, num_nodes),
                psd: vec![Complex64::new(4.0 * k_b * temperature * g, 0.0)],
            });
        }

        // Cyclostationary device sources from the converged waveforms.
        sources.extend(solver.device_noise_sources(&state, temperature));

        let mut output_noise = Vec::with_capacity(offsets.len());
        for &offset in offsets {
            let psd = solver
                .solve_periodic_noise(
                    &state,
                    offset,
                    -max_sideband,
                    max_sideband,
                    out_idx,
                    &sources,
                )
                .map_err(|e| {
                    SimulationError::Circuit(format!(
                        "pnoise solve failed at offset {offset:.6e} Hz: {e}"
                    ))
                })?;
            output_noise.push(psd);
        }

        Ok(PnoiseAnalysisResult {
            frequencies: offsets.to_vec(),
            output_noise,
            fundamental_freq,
            converged: state.converged,
        })
    }
}
