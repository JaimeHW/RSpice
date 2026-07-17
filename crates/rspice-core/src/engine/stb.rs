//! STB (loop stability) analysis: true loop gain via Tian's method.
//!
//! The probe is a designated 0 V voltage source placed in the feedback loop
//! (the Spectre `stb` probe convention — its MNA branch already carries the
//! loop current). At every sweep frequency two small-signal solves run
//! against the same linearized matrix, with every other independent source
//! AC-dead:
//!
//! 1. **Voltage injection**: the probe branch drives a unit series voltage.
//! 2. **Current injection**: a unit current is injected into the probe's
//!    sense terminal.
//!
//! With `v` the sense-terminal voltage and `i` the probe branch current
//! (positive from the + terminal through the source), the loop gain follows
//! Tian, Visvanathan, Hantgan, Kundert, "Striving for Small-Signal
//! Stability", IEEE Circuits & Devices Magazine 17(1), Jan 2001:
//!
//! ```text
//! T = -1 / (1 - 1/(2*(i1*v2 - v1*i2) + v1 + i2))
//! ```
//!
//! implemented in the algebraically equivalent, singularity-free form
//! `T = D/(1 - D)` with `D = 2*(i1*v2 - v1*i2) + v1 + i2`. Unlike single
//! voltage injection, the combination is exact for arbitrary loading and
//! bidirectional transmission at the break, and is independent of the probe
//! orientation. The convention yields phase 0 at DC for a stable inverting
//! loop, so phase margin is `180 deg + arg T` at unity crossover — exactly
//! what `StbAnalyzer` extracts.

use super::{Engine, SimulationError};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::advanced::stb::{StbAnalyzer, StbConfig, StbResult};
use crate::{Complex64, Netlist, Value};
use std::f64::consts::PI;

/// STB analysis result: the Tian loop gain sweep plus extracted margins.
#[derive(Debug, Clone)]
pub struct StbAnalysisResult {
    /// Sweep frequencies (Hz)
    pub frequencies: Vec<Value>,
    /// Complex loop gain at each frequency
    pub loop_gains: Vec<Complex64>,
    /// Bode/Nyquist data and stability margins
    pub result: StbResult,
    /// Name of the probe voltage source
    pub probe_name: String,
}

impl Engine {
    /// Run loop-stability analysis at the probe source named in
    /// `config.probe_node`.
    ///
    /// The probe must be an existing voltage source with zero DC value: a
    /// nonzero value would mean the element biases the circuit and was
    /// almost certainly not meant as a probe. Any AC specification on the
    /// probe or on other independent sources is ignored — loop gain is a
    /// property of the linearized network, not of the stimuli.
    pub fn run_stb(
        &self,
        netlist: &Netlist,
        config: StbConfig,
    ) -> Result<StbAnalysisResult, SimulationError> {
        self.run_stb_with_abort(netlist, config, &NoAbort)
    }

    /// Cancellable form of [`Self::run_stb`].
    pub fn run_stb_with_abort(
        &self,
        netlist: &Netlist,
        config: StbConfig,
        abort: &dyn AbortSignal,
    ) -> Result<StbAnalysisResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        config
            .validate()
            .map_err(|err| SimulationError::Circuit(format!("Invalid STB config: {err}")))?;

        let probe_name = config
            .probe_node
            .clone()
            .ok_or_else(|| {
                SimulationError::Circuit(
                    "STB requires a probe: name a 0 V voltage source placed in the loop"
                        .to_string(),
                )
            })?
            .trim()
            .to_string();
        if probe_name.is_empty() {
            return Err(SimulationError::Circuit(
                "STB probe name is empty".to_string(),
            ));
        }

        let engine = self.resolved_for_netlist(netlist);
        let mut circuit = engine.build_circuit(netlist)?;
        if circuit.num_nodes() == 0 {
            return Err(SimulationError::Circuit("Circuit has no nodes".to_string()));
        }
        Self::ensure_supported_dynamic_charges(&circuit, "STB")?;

        let probe_idx = circuit
            .voltage_sources
            .names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(&probe_name))
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "STB probe '{probe_name}' is not a voltage source in the circuit; \
                     insert a 0 V source in the loop and name it as the probe"
                ))
            })?;

        let dc_value = circuit.voltage_sources.dc_values[probe_idx];
        if dc_value != 0.0 {
            return Err(SimulationError::Circuit(format!(
                "STB probe '{probe_name}' has DC value {dc_value} V; the probe must be \
                 a 0 V source so it only senses the loop"
            )));
        }

        let node_pos = circuit.voltage_sources.node_pos[probe_idx];
        let node_neg = circuit.voltage_sources.node_neg[probe_idx];
        if node_pos == 0 || node_neg == 0 {
            // With one terminal on the reference node the method is blind,
            // not merely inaccurate: the zero-volt probe shorts the current
            // injection straight to ground, so the second experiment carries
            // no loop information and the formula degenerates to T = 0 for
            // any circuit. Ground is the nodal reference, not a branch of
            // the loop — there is nothing to break there.
            return Err(SimulationError::Circuit(format!(
                "STB probe '{probe_name}' has a grounded terminal; place the probe in \
                 series with the loop's signal path (both terminals on circuit nodes), \
                 e.g. between the output and the feedback network"
            )));
        }

        // Loop gain is independent of the probe orientation (Tian's method
        // is symmetric); measure at the + terminal by convention.
        let sense_node = node_pos;

        let br_ordinal = circuit.voltage_sources.branch_indices[probe_idx];

        let mut matrix = engine.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);

        let has_nonlinear = circuit.has_nonlinear_devices();
        let dc_solution = engine.solve_dc_operating_point_with_abort(
            netlist,
            &mut circuit,
            &mut matrix,
            abort,
        )?;
        circuit.refresh_jiles_atherton_inductances(&dc_solution);
        if has_nonlinear {
            circuit.update_nonlinear(&dc_solution);
        }
        circuit.prepare_behavioral_small_signal(&dc_solution);

        let size = circuit.matrix_size();
        let br = circuit.get_branch_matrix_index(br_ordinal);

        let frequencies = config.frequency_points();
        let mut loop_gains = Vec::with_capacity(frequencies.len());

        for (frequency_index, &freq) in frequencies.iter().enumerate() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let omega = 2.0 * PI * freq;
            circuit.prepare_behavioral_small_signal_at_frequency(&dc_solution, freq);
            let mut ac_matrix =
                Self::try_build_small_signal_ac_matrix(&circuit, &matrix, &dc_solution, omega)?;

            // Experiment 1: unit series voltage at the probe, all other
            // sources dead.
            let mut rhs_v = vec![Complex64::new(0.0, 0.0); size];
            rhs_v[br - 1] = Complex64::new(1.0, 0.0);
            let sol_v = ac_matrix.solve(&rhs_v).map_err(SimulationError::Solver)?;
            let v1 = sol_v[sense_node - 1];
            let i1 = sol_v[br - 1];

            // Experiment 2: unit current into the sense terminal, probe at
            // 0 V (its branch row keeps enforcing V(+) - V(-) = 0).
            let mut rhs_i = vec![Complex64::new(0.0, 0.0); size];
            rhs_i[sense_node - 1] = Complex64::new(1.0, 0.0);
            let sol_i = ac_matrix.solve(&rhs_i).map_err(SimulationError::Solver)?;
            let v2 = sol_i[sense_node - 1];
            let i2 = sol_i[br - 1];

            // Tian: T = -1/(1 - 1/D) = D/(1 - D); D -> 0 (no loop) gives
            // T -> 0 without the intermediate division blowing up.
            let d = 2.0 * (i1 * v2 - v1 * i2) + v1 + i2;
            let denom = Complex64::new(1.0, 0.0) - d;
            let t = if denom.norm() < 1e-30 {
                Complex64::new(f64::INFINITY, 0.0)
            } else {
                d / denom
            };
            loop_gains.push(t);
            abort.observe_progress((frequency_index + 1) as f64 / frequencies.len() as f64);
        }

        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }

        let analyzer = StbAnalyzer::new(config);
        let result = analyzer.analyze(&frequencies, &loop_gains);

        Ok(StbAnalysisResult {
            frequencies,
            loop_gains,
            result,
            probe_name,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::advanced::stb::StbSweepType;

    #[test]
    fn stb_refreshes_frequency_dependent_behavioral_conductance() {
        let netlist = Netlist::parse_with_options(
            "live FREQ STB operator\n\
             .PARAM RUNTIME_R={FREQ}\n\
             EAMP out 0 in 0 10\n\
             VPROBE out fb 0\n\
             RF fb in {RUNTIME_R}\n\
             RIN in 0 1k\n\
             .END\n",
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::netlist::ExpressionDialect::Xyce,
                ..crate::netlist::NetlistParseOptions::default()
            },
        )
        .expect("frequency-dependent STB deck parses");
        let engine = Engine::new(
            crate::engine::SimulationConfig::default()
                .with_spice_dialect(crate::engine::SpiceDialect::Xyce),
        );
        let config = StbConfig::new()
            .with_sweep(10.0, 100.0, 2)
            .with_sweep_type(StbSweepType::Linear)
            .with_probe("VPROBE");
        let result = engine
            .run_stb(&netlist, config)
            .expect("frequency-dependent STB operators solve");
        assert_eq!(result.loop_gains.len(), 2);
        assert!(
            (result.loop_gains[0] - result.loop_gains[1]).norm() > 1.0e-3,
            "STB operator retained a stale FREQ conductance: {:?}",
            result.loop_gains
        );
    }
}
