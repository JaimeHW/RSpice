//! DC small-signal transfer-function analysis (`.TF`).
//!
//! Computes the three quantities ngspice reports for `.TF output insrc`:
//! the transfer function (gain), the input impedance seen by the input
//! source, and the output impedance at the probe. All three come from
//! linear solves of the DC-linearized circuit at 0 Hz:
//!
//! 1. Drive the input source with a unit AC excitation (all other AC
//!    excitations cleared): the probe value is the gain, and the input
//!    source's own branch voltage/current gives the input impedance.
//! 2. Drive the probe with a unit AC current (input source's AC cleared):
//!    the probe voltage is the output impedance.

use super::ac::{AcExcitation, PreparedAc};
use super::{Engine, SimulationError};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::TransferFunctionResult;
use crate::analysis::ac::AcResult;
use crate::solver::ComplexMatrix;
use crate::{CircuitData, Complex64};
use crate::{Netlist, Value};

/// ngspice's sentinel for an effectively infinite impedance (tfanal.c
/// reports 1e20 for the output impedance of branch-current probes).
const NGSPICE_INFINITE_IMPEDANCE: Value = 1.0e20;

impl Engine {
    /// Run a `.TF` analysis: DC small-signal gain, input impedance, and
    /// output impedance from `input_source` to the probe.
    ///
    /// The probe is `V(output_node[,reference_node])`, or the branch
    /// current of element `output_node` when `output_is_current` is set
    /// (the element must add a branch, e.g. a voltage source or inductor).
    pub fn run_transfer_function(
        &self,
        netlist: &Netlist,
        output_node: &str,
        reference_node: Option<&str>,
        output_is_current: bool,
        input_source: &str,
    ) -> Result<TransferFunctionResult, SimulationError> {
        self.run_transfer_function_with_abort(
            netlist,
            output_node,
            reference_node,
            output_is_current,
            input_source,
            &NoAbort,
        )
    }

    /// Cancellable form of [`Self::run_transfer_function`].
    pub fn run_transfer_function_with_abort(
        &self,
        netlist: &Netlist,
        output_node: &str,
        reference_node: Option<&str>,
        output_is_current: bool,
        input_source: &str,
        abort: &dyn AbortSignal,
    ) -> Result<TransferFunctionResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let engine = self.resolved_for_netlist(netlist);
        engine.ensure_analysis_points(1)?;
        let run_scope = crate::abort_signal::ModelRunSignal::if_needed(abort);
        let abort: &dyn AbortSignal = run_scope.as_ref().map_or(abort, |scope| scope);
        Self::ensure_model_run_active(abort)?;
        let circuit = engine.build_circuit_with_abort(netlist, abort)?;
        let voltage_input = circuit
            .voltage_sources
            .names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(input_source));
        let current_input = circuit
            .current_sources
            .names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(input_source));
        let excitation = if voltage_input.is_some() {
            AcExcitation::for_port(&circuit, input_source)?
        } else if let Some(index) = current_input {
            AcExcitation::current_probe(
                &circuit,
                circuit.current_sources.node_neg[index],
                circuit.current_sources.node_pos[index],
            )?
        } else {
            return Err(SimulationError::Netlist(format!(
                ".TF input `{input_source}` is not an independent V/I source in the elaborated circuit"
            )));
        };
        let node_id = |name: &str| {
            let name = netlist.ground_policy().canonical_node(name);
            circuit.get_node_by_name(name).ok_or_else(|| {
                SimulationError::Netlist(format!(".TF references unknown node `{name}`"))
            })
        };
        let output_probe = if output_is_current {
            if circuit.get_branch_by_name(output_node).is_none() {
                return Err(SimulationError::Netlist(format!(
                    ".TF output element `{output_node}` has no branch current"
                )));
            }
            None
        } else {
            Some(AcExcitation::current_probe(
                &circuit,
                node_id(output_node)?,
                node_id(reference_node.unwrap_or("0"))?,
            )?)
        };
        // One elaboration and bias state, with independent right-hand sides.
        // Authored excitations at every hierarchy level are excluded, and no
        // synthetic source name can collide with an authored element.
        let PreparedAc {
            mut circuit,
            mut matrix,
            linearization,
            excitation,
        } = engine.prepare_ac_circuit_with_excitation(netlist, circuit, Some(excitation), abort)?;
        engine.ensure_result_shape(1, 3)?;
        let mut workspace = ComplexMatrix::from_real_structure(&matrix);
        let solve = |circuit: &mut CircuitData, final_step| {
            linearization.prepare_frequency(circuit, &mut workspace, 0.0, final_step, abort)?;
            let solution = linearization.solve(&mut workspace, &excitation, abort)?;
            let drive = ac_result(circuit, solution);
            abort.observe_progress(0.5);
            let gain = if output_is_current {
                branch_current(&drive, output_node).ok_or_else(|| {
                    SimulationError::Netlist(format!(
                        ".TF output `{output_node}` has no branch current"
                    ))
                })?
            } else {
                voltage_difference(&drive, output_node, reference_node)?
            };
            let input_impedance = if voltage_input.is_some() {
                let current = branch_current(&drive, input_source).ok_or_else(|| {
                    SimulationError::Netlist(format!(
                        ".TF input `{input_source}` has no branch current"
                    ))
                })?;
                if current == 0.0 {
                    Value::INFINITY
                } else {
                    -1.0 / current
                }
            } else {
                let index = current_input.expect("validated independent current source");
                let value = |node: usize| {
                    if node == 0 {
                        0.0
                    } else {
                        drive.voltages[node - 1].re
                    }
                };
                value(circuit.current_sources.node_neg[index])
                    - value(circuit.current_sources.node_pos[index])
            };
            let output_impedance = if let Some(probe) = &output_probe {
                let solution = linearization.solve(&mut workspace, probe, abort)?;
                voltage_difference(&ac_result(circuit, solution), output_node, reference_node)?
            } else {
                NGSPICE_INFINITE_IMPEDANCE
            };
            Ok((gain, input_impedance, output_impedance))
        };
        let (gain, input_impedance, output_impedance) = Self::solve_accepted_frequency_point(
            &mut circuit,
            &mut matrix,
            &linearization.bias,
            super::analog_tasks::FrequencyModelPoint {
                analysis: 1,
                frequency: 0.0,
                final_step: true,
            },
            abort,
            solve,
        )?;
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        abort.observe_progress(1.0);

        let probe_label = if output_is_current {
            format!("I({output_node})")
        } else {
            match reference_node {
                Some(reference) => format!("V({output_node},{reference})"),
                None => format!("V({output_node})"),
            }
        };

        Ok(TransferFunctionResult::new(
            &probe_label,
            input_source,
            gain,
            input_impedance,
            output_impedance,
        ))
    }
}

fn ac_result(circuit: &CircuitData, solution: Vec<Complex64>) -> AcResult {
    let num_nodes = circuit.num_nodes();
    let mut currents = solution[num_nodes..].to_vec();
    circuit
        .capacitors
        .project_complex_ic_branch_currents(&solution, &mut currents, 0.0);
    AcResult {
        frequency: 0.0,
        node_names: circuit.node_names_sorted(),
        branch_names: circuit.branch_names_sorted(),
        voltages: solution[..num_nodes].to_vec(),
        currents,
    }
}

fn is_ground(node: &str) -> bool {
    node == "0"
}

fn node_voltage(solution: &AcResult, node: &str) -> Result<Value, SimulationError> {
    if is_ground(node) {
        return Ok(0.0);
    }
    solution
        .node_names
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(node))
        .and_then(|idx| solution.voltages.get(idx))
        .map(|value| value.re)
        .ok_or_else(|| SimulationError::Netlist(format!(".TF references unknown node `{node}`")))
}

fn voltage_difference(
    solution: &AcResult,
    positive: &str,
    reference: Option<&str>,
) -> Result<Value, SimulationError> {
    let pos = node_voltage(solution, positive)?;
    let neg = match reference {
        Some(node) => node_voltage(solution, node)?,
        None => 0.0,
    };
    Ok(pos - neg)
}

fn branch_current(solution: &AcResult, element: &str) -> Option<Value> {
    solution
        .branch_names
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(element))
        .and_then(|idx| solution.currents.get(idx))
        .map(|value| value.re)
}
