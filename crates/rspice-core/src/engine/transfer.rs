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

use super::{Engine, SimulationError};
use crate::analysis::TransferFunctionResult;
use crate::analysis::ac::AcResult;
use crate::netlist::{Element, ElementKind, SourceSpec};
use crate::{Netlist, Value};

/// Name of the temporary probe source injected for the output-impedance
/// solve; chosen to stay clear of any plausible user element name.
const ZOUT_PROBE_NAME: &str = "ITF_ZOUT_PROBE_INTERNAL";

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
        // Base deck: every independent source's AC excitation cleared, so
        // each solve below is driven purely by its own unit excitation.
        let mut base = netlist.clone();
        for element in &mut base.elements {
            if let ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) =
                &mut element.kind
            {
                *spec = spec.clone().with_ac(0.0, 0.0);
            }
        }

        let input_kind = independent_source_kind(&base, input_source).ok_or_else(|| {
            SimulationError::Netlist(format!(
                ".TF input `{input_source}` is not an independent V/I source in the netlist"
            ))
        })?;

        // Solve 1: unit drive at the input source.
        let mut driven = base.clone();
        set_source_ac(&mut driven, input_source, 1.0, 0.0);
        let drive_solution = self.single_zero_hz_solve(&driven)?;

        let gain = if output_is_current {
            branch_current(&drive_solution, output_node).ok_or_else(|| {
                SimulationError::Netlist(format!(
                    ".TF output element `{output_node}` has no branch current; probe a \
                     voltage source or inductor"
                ))
            })?
        } else {
            voltage_difference(&drive_solution, output_node, reference_node)?
        };

        let input_impedance = match input_kind {
            InputKind::Voltage => {
                // SPICE convention: a voltage source's branch current is
                // positive flowing from its + node through the source, so a
                // passive load yields a negative branch current and
                // Zin = -V/I is positive.
                let current = branch_current(&drive_solution, input_source).ok_or_else(|| {
                    SimulationError::Netlist(format!(
                        ".TF input source `{input_source}` has no branch current in the AC solution"
                    ))
                })?;
                if current.abs() < 1e-300 {
                    Value::INFINITY
                } else {
                    -1.0 / current
                }
            }
            InputKind::Current => {
                // A unit current source injects 1 A into its negative node;
                // the voltage developed across it is the input impedance.
                let (pos, neg) = source_nodes(&base, input_source)?;
                voltage_difference(&drive_solution, &neg, Some(&pos))?
            }
        };

        // Solve 2: unit current injected at the probe, input AC cleared.
        // For branch-current outputs ngspice reports the output impedance
        // as its 1e20 "infinite" sentinel rather than solving; match that.
        let output_impedance = if output_is_current {
            NGSPICE_INFINITE_IMPEDANCE
        } else {
            let probe_pos = output_node.to_string();
            let probe_neg = reference_node.unwrap_or("0").to_string();
            let mut zout_deck = base;
            zout_deck.elements.push(Element {
                name: ZOUT_PROBE_NAME.to_string(),
                kind: ElementKind::CurrentSource(SourceSpec::Ac {
                    magnitude: 1.0,
                    phase: 0.0,
                }),
                // Current flows n+ -> n- inside the source, i.e. 1 A is
                // injected into the probe's positive node.
                nodes: vec![probe_neg.clone(), probe_pos.clone()],
            });
            let zout_solution = self.single_zero_hz_solve(&zout_deck)?;
            voltage_difference(&zout_solution, &probe_pos, Some(&probe_neg))?
        };

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

    /// Linearize at the DC operating point and solve once at 0 Hz.
    fn single_zero_hz_solve(&self, netlist: &Netlist) -> Result<AcResult, SimulationError> {
        self.run_ac(netlist, &[0.0])?.pop().ok_or_else(|| {
            SimulationError::Circuit(
                "transfer-function AC solve produced no sample at 0 Hz".to_string(),
            )
        })
    }
}

enum InputKind {
    Voltage,
    Current,
}

fn independent_source_kind(netlist: &Netlist, name: &str) -> Option<InputKind> {
    netlist
        .elements
        .iter()
        .find(|element| element.name.eq_ignore_ascii_case(name))
        .and_then(|element| match &element.kind {
            ElementKind::VoltageSource(_) => Some(InputKind::Voltage),
            ElementKind::CurrentSource(_) => Some(InputKind::Current),
            _ => None,
        })
}

fn set_source_ac(netlist: &mut Netlist, name: &str, magnitude: Value, phase: Value) {
    for element in &mut netlist.elements {
        if element.name.eq_ignore_ascii_case(name)
            && let (ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec)) =
                &mut element.kind
        {
            *spec = spec.clone().with_ac(magnitude, phase);
        }
    }
}

fn source_nodes(netlist: &Netlist, name: &str) -> Result<(String, String), SimulationError> {
    let element = netlist
        .elements
        .iter()
        .find(|element| element.name.eq_ignore_ascii_case(name))
        .ok_or_else(|| {
            SimulationError::Netlist(format!(".TF element `{name}` not found"))
        })?;
    match element.nodes.as_slice() {
        [pos, neg, ..] => Ok((pos.clone(), neg.clone())),
        _ => Err(SimulationError::Netlist(format!(
            ".TF element `{name}` does not have two terminals"
        ))),
    }
}

fn is_ground(node: &str) -> bool {
    node == "0" || node.eq_ignore_ascii_case("gnd")
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
        .ok_or_else(|| {
            SimulationError::Netlist(format!(".TF references unknown node `{node}`"))
        })
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
