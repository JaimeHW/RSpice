//! Periodic AC (PAC) analysis: small-signal conversion analysis around a
//! harmonic-balance operating point.
//!
//! The large-signal periodic solution comes from the HB solver; the
//! small-signal stimulus is applied at `f = offset + m*f0` and the linearized
//! time-varying network maps it into responses at every sideband
//! `offset + k*f0`. See `harmonic_balance::solver::periodic_ac` for the
//! conversion-matrix formulation.

#![allow(clippy::needless_range_loop)]

use super::*;
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::advanced::harmonic_balance::PeriodicAcExcitation;
use crate::analysis::advanced::pac::{PacConfig, PacResult};
use crate::analysis::{HbConfig, HbSolverState};

/// PAC analysis result with convergence info
#[derive(Debug)]
pub struct PacAnalysisResult {
    /// The PAC solution: per-node sideband spectra and the conversion matrix
    /// for the configured output node.
    pub result: PacResult,
    /// Large-signal fundamental frequency (Hz)
    pub fundamental_freq: Value,
    /// Whether the operating-point solve converged
    pub converged: bool,
}

impl Engine {
    /// Run Periodic AC analysis.
    ///
    /// Solves the large-signal periodic operating point with harmonic
    /// balance, samples the periodically time-varying small-signal
    /// linearization, and solves the sideband-coupled system at every sweep
    /// offset. The per-node spectra answer "what does the circuit do to the
    /// input source's signal"; when an output node is configured, the full
    /// conversion matrix (every input sideband to every output sideband) is
    /// filled for that node.
    pub fn run_pac(
        &self,
        netlist: &Netlist,
        config: PacConfig,
    ) -> Result<PacAnalysisResult, SimulationError> {
        self.run_pac_with_abort(netlist, config, &NoAbort)
    }

    /// Run periodic AC with cooperative cancellation between operating-point
    /// iterations and frequency/sideband solves.
    pub fn run_pac_with_abort(
        &self,
        netlist: &Netlist,
        config: PacConfig,
        abort: &dyn AbortSignal,
    ) -> Result<PacAnalysisResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if !config.fundamental_freq.is_finite() || config.fundamental_freq <= 0.0 {
            return Err(SimulationError::Circuit(
                "PAC requires a positive fundamental frequency".to_string(),
            ));
        }
        config
            .validate()
            .map_err(|e| SimulationError::Circuit(format!("Invalid PAC config: {e}")))?;

        let input_name = config
            .input_source
            .clone()
            .ok_or_else(|| SimulationError::Circuit("PAC requires an input source".to_string()))?;

        let circuit = self.build_circuit(netlist)?;
        let num_nodes = circuit.num_nodes();
        if num_nodes == 0 {
            return Err(SimulationError::Circuit("Circuit has no nodes".to_string()));
        }
        if let Some(summary) = Self::hb_unsupported_nonlinear_device_summary(&circuit, num_nodes) {
            return Err(HbError::UnsupportedNonlinearDevices(summary).into());
        }

        // The operating point needs enough harmonics that every conversion
        // coupling G[k-m] over the sideband span exists, with headroom for
        // the drive itself.
        let span = (config.sideband_max - config.sideband_min).unsigned_abs() as usize;
        let extreme = (config
            .sideband_min
            .unsigned_abs()
            .max(config.sideband_max.unsigned_abs())) as usize;
        let op_harmonics = span.max(extreme).max(8);

        let hb_config = HbConfig::new(config.fundamental_freq)
            .with_harmonics(op_harmonics)
            .with_oversample(4);
        let drive_tones = Self::hb_collect_drive_tones(&hb_config)?;

        let mut solver = HbSolver::new(hb_config.clone(), num_nodes);
        let node_names = self.hb_build_node_names(&circuit, num_nodes);
        solver.set_node_names(node_names.clone());

        self.hb_stamp_resistors(&circuit, &mut solver);
        self.hb_stamp_capacitors(&circuit, &mut solver);
        self.hb_stamp_inductors(&circuit, &mut solver);
        // Always Norton form: the small-signal system must short the
        // large-signal voltage sources, and the input excitation needs a
        // node-space current injection.
        self.hb_stamp_voltage_sources_norton(&circuit, &mut solver, &hb_config, &drive_tones)?;
        self.hb_stamp_current_sources(&circuit, &mut solver, &hb_config, &drive_tones)?;

        let has_nonlinear = Self::hb_has_supported_nonlinear_devices(&circuit, num_nodes);
        if has_nonlinear {
            self.hb_stamp_supported_nonlinear_devices(&circuit, &mut solver, num_nodes);
        }

        let mut state = HbSolverState::new(num_nodes, op_harmonics);
        if has_nonlinear {
            solver
                .solve_newton_with_abort(&mut state, abort)
                .map_err(|e| match e {
                    crate::analysis::HbError::Aborted => SimulationError::Aborted,
                    _ => SimulationError::Circuit(format!("PAC operating-point solve failed: {e}")),
                })?;
        } else {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            solver.solve_linear(&mut state).map_err(|e| {
                SimulationError::Circuit(format!("PAC operating-point solve failed: {e}"))
            })?;
        }

        // Resolve the small-signal input source to node-space injections of
        // unit amplitude.
        let injections = Self::pac_input_injections(&circuit, &input_name, num_nodes)?;

        let mut sweep_config = config.clone();
        sweep_config.fundamental_freq = config.fundamental_freq;
        let frequencies = sweep_config.try_frequency_points().map_err(|err| {
            SimulationError::Circuit(format!("Invalid PAC frequency sweep: {err}"))
        })?;
        if frequencies.is_empty() {
            return Err(SimulationError::Circuit(
                "PAC frequency sweep produced no points".to_string(),
            ));
        }

        let mut result = PacResult::new(
            config.fundamental_freq,
            frequencies.clone(),
            config.sideband_min,
            config.sideband_max,
            node_names.clone(),
            Vec::new(),
        );
        result.set_input_source(&input_name);
        if let Some(ref out) = config.output_node {
            result.set_output_node(out);
        }

        let output_idx = config
            .output_node
            .as_deref()
            .and_then(|name| node_names.iter().position(|n| n.eq_ignore_ascii_case(name)));

        // Excitation columns: the input source's own frequency (m = 0)
        // always; every input sideband when a conversion matrix is wanted.
        let excitation_sidebands: Vec<i32> = if output_idx.is_some() {
            (config.sideband_min..=config.sideband_max).collect()
        } else {
            vec![0]
        };

        for (freq_idx, &offset) in frequencies.iter().enumerate() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let excitations: Vec<PeriodicAcExcitation> = excitation_sidebands
                .iter()
                .map(|&m| PeriodicAcExcitation {
                    sideband: m,
                    injections: injections.clone(),
                })
                .collect();

            let solutions = solver
                .solve_periodic_ac(
                    &state,
                    offset,
                    config.sideband_min,
                    config.sideband_max,
                    &excitations,
                )
                .map_err(|e| {
                    SimulationError::Circuit(format!(
                        "PAC solve failed at offset {offset:.6e} Hz: {e}"
                    ))
                })?;

            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }

            for (col, &m) in excitation_sidebands.iter().enumerate() {
                let by_node = &solutions[col];

                if m == 0 {
                    for k_idx in 0..result.num_sidebands() {
                        let k = config.sideband_min + k_idx as i32;
                        if let Some(data) = result.get_sideband_data_mut(freq_idx, k) {
                            for node in 0..num_nodes {
                                data.set_voltage(node, by_node[node][k_idx]);
                            }
                        }
                    }
                }

                if let Some(out) = output_idx {
                    for k_idx in 0..(config.sideband_max - config.sideband_min + 1) as usize {
                        let k = config.sideband_min + k_idx as i32;
                        result
                            .conversion_matrix
                            .set(freq_idx, k, m, by_node[out][k_idx]);
                    }
                }
            }
        }

        result.set_convergence_info(state.iteration.max(1), state.residual_norm);

        Ok(PacAnalysisResult {
            result,
            fundamental_freq: config.fundamental_freq,
            converged: state.converged,
        })
    }

    /// Map the named small-signal source to unit-amplitude current
    /// injections. Voltage sources use their stiff Norton conversion (the
    /// parallel conductance is already stamped); current sources inject
    /// directly with the SPICE element orientation.
    pub(in crate::engine::hb) fn pac_input_injections(
        circuit: &CircuitData,
        input_name: &str,
        _num_nodes: usize,
    ) -> Result<Vec<(usize, Complex64)>, SimulationError> {
        let trimmed = input_name.trim();

        if let Some(idx) = circuit
            .voltage_sources
            .names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(trimmed))
        {
            let np = circuit.voltage_sources.node_pos[idx];
            let nn = circuit.voltage_sources.node_neg[idx];
            let mut injections = Vec::new();
            if np > 0 {
                injections.push((np - 1, Complex64::new(HB_NORTON_G, 0.0)));
            }
            if nn > 0 {
                injections.push((nn - 1, Complex64::new(-HB_NORTON_G, 0.0)));
            }
            return Ok(injections);
        }

        if let Some(idx) = circuit
            .current_sources
            .names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(trimmed))
        {
            let np = circuit.current_sources.node_pos[idx];
            let nn = circuit.current_sources.node_neg[idx];
            let mut injections = Vec::new();
            if np > 0 {
                injections.push((np - 1, Complex64::new(-1.0, 0.0)));
            }
            if nn > 0 {
                injections.push((nn - 1, Complex64::new(1.0, 0.0)));
            }
            return Ok(injections);
        }

        Err(SimulationError::Circuit(format!(
            "PAC input source '{trimmed}' not found among independent sources"
        )))
    }
}
