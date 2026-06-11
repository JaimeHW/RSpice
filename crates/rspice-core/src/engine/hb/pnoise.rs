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
    /// Total output noise voltage PSD at each offset (V^2/Hz).
    pub output_noise: Vec<Value>,
    /// Per-source contributions: `(label, psd per offset)`, summing to the
    /// total at every offset.
    pub contributors: Vec<(String, Vec<Value>)>,
    /// Large-signal fundamental (Hz).
    pub fundamental_freq: Value,
    /// Whether the operating-point solve converged.
    pub converged: bool,
}

impl Engine {
    /// Run periodic noise analysis at `output_node` (optionally referenced
    /// to `output_ref` for a differential output) over `offsets`.
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
        output_ref: Option<&str>,
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
        let ref_idx = output_ref
            .map(|name| {
                node_names
                    .iter()
                    .position(|n| n.eq_ignore_ascii_case(name.trim()))
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "pnoise output reference node '{name}' not found in circuit nodes"
                        ))
                    })
            })
            .transpose()?;

        let temperature = self.config.temperature;
        let k_b = 1.380649e-23;

        // Stationary resistor thermal sources: 4kT*G between the resistor
        // terminals (DC-only intensity spectrum).
        let hb_dc_voltage = |row: usize| -> Value {
            if row == 0 {
                0.0
            } else {
                state
                    .x
                    .get(row - 1)
                    .and_then(|s| s.first())
                    .map(|c| c.re)
                    .unwrap_or(0.0)
            }
        };

        let mut sources: Vec<PeriodicNoiseSource> = Vec::new();
        for i in 0..circuit.resistors.len() {
            if !circuit.resistors.noisy.get(i).copied().unwrap_or(true) {
                continue;
            }
            let g = circuit.resistors.conductances[i];
            if !(g.is_finite() && g > 0.0) {
                continue;
            }
            let np = circuit.resistors.stamps[i].pp.row;
            let nn = circuit.resistors.stamps[i].nn.row;
            let name = circuit
                .resistors
                .names
                .get(i)
                .cloned()
                .unwrap_or_else(|| format!("R#{i}"));
            sources.push(PeriodicNoiseSource {
                name: format!("{name} thermal"),
                node_pos: Self::hb_node_to_solver_index(np, num_nodes),
                node_neg: Self::hb_node_to_solver_index(nn, num_nodes),
                psd: vec![Complex64::new(4.0 * k_b * temperature * g, 0.0)],
                flicker: None,
            });

            // Model-card resistor flicker rides on the DC bias current,
            // matching the stationary .noise treatment.
            if let Some(&Some((coefficient, af, ef))) = circuit.resistors.flicker.get(i) {
                let i_dc = g * (hb_dc_voltage(np) - hb_dc_voltage(nn));
                if i_dc.abs() > 1e-18 {
                    sources.push(PeriodicNoiseSource {
                        name: format!("{name} flicker"),
                        node_pos: Self::hb_node_to_solver_index(np, num_nodes),
                        node_neg: Self::hb_node_to_solver_index(nn, num_nodes),
                        psd: vec![Complex64::new(0.0, 0.0)],
                        flicker: Some((coefficient * i_dc.abs().powf(af), ef)),
                    });
                }
            }
        }

        // Diode flicker (KF * |Id|^AF / f) at the periodic-average bias.
        for diode in &circuit.diodes.devices {
            if diode.kf > 0.0 {
                let va = hb_dc_voltage(diode.node_anode);
                let vc = hb_dc_voltage(diode.node_cathode);
                let arg = ((va - vc) / (diode.n * diode.vt)).min(40.0);
                let i_dc = diode.is * (arg.exp() - 1.0);
                if i_dc.abs() > 1e-18 {
                    sources.push(PeriodicNoiseSource {
                        name: format!("{} flicker", diode.name),
                        node_pos: Self::hb_node_to_solver_index(diode.node_anode, num_nodes),
                        node_neg: Self::hb_node_to_solver_index(diode.node_cathode, num_nodes),
                        psd: vec![Complex64::new(0.0, 0.0)],
                        flicker: Some((diode.kf * i_dc.abs().powf(diode.af), 1.0)),
                    });
                }
            }
        }

        // Cyclostationary device sources from the converged waveforms.
        sources.extend(solver.device_noise_sources(&state, temperature));

        let mut output_noise = Vec::with_capacity(offsets.len());
        let mut contributors: Vec<(String, Vec<Value>)> = sources
            .iter()
            .map(|s| (s.name.clone(), Vec::with_capacity(offsets.len())))
            .collect();
        for &offset in offsets {
            let per_source = solver
                .solve_periodic_noise(
                    &state,
                    offset,
                    -max_sideband,
                    max_sideband,
                    out_idx,
                    ref_idx,
                    &sources,
                )
                .map_err(|e| {
                    SimulationError::Circuit(format!(
                        "pnoise solve failed at offset {offset:.6e} Hz: {e}"
                    ))
                })?;
            output_noise.push(per_source.iter().sum());
            for (slot, &value) in contributors.iter_mut().zip(&per_source) {
                slot.1.push(value);
            }
        }

        Ok(PnoiseAnalysisResult {
            frequencies: offsets.to_vec(),
            output_noise,
            contributors,
            fundamental_freq,
            converged: state.converged,
        })
    }
}
