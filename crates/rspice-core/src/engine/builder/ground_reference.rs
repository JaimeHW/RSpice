//! The reference node of a deck that names no node 0.
//!
//! A deck keeps node 0 when an element names `0` or one of its aliases, or
//! when, without the deck naming it, an element the deck authors connects
//! node 0 to the rest of the circuit through a branch incident on it: a
//! Verilog-A or Verilog-AMS module's contribution or source that reaches one
//! of its electrical ports single-ended or through a `ground` net, an XSPICE
//! bridge or code model that drives or loads a net against node 0, or a
//! source or load the build makes from an authored card with node 0 at one
//! end. Choosing another net as the reference of such a deck would move the
//! very node those elements are connected to, so the build chooses one only
//! for a deck that connects node 0 nowhere.
//!
//! A converter the build inserts itself never decides: the automatic D/A and
//! A/D converters the auto-bridge planner and the connect rules put on a
//! boundary between the discrete and electrical worlds (a template's or a
//! connect module's body included), and the automatic real-to-voltage and
//! voltage-to-real converters. The build decides before it inserts them, and
//! each drives or senses against node 0, which is the deck's reference
//! whichever net that turns out to be: node 0 when the deck names or
//! connects it, the chosen net otherwise.
//!
//! What only observes node 0 does not connect it: a single-ended read of a
//! potential in an equation, a Verilog-AMS process's analog read, an A/D
//! converter's or a code model's voltage input, a B source's read, a task's
//! arguments. A read stamps nothing in node 0's row or column; it
//! differentiates with respect to the net it reads alone. Without a branch to
//! node 0 the deck as written would float.
//!
//! The build therefore runs as written first. Only when that circuit
//! connects node 0 nowhere is the deck built once more, with the negative
//! terminal of its first voltage source aliased to node 0 before any node
//! exists, so every device and internal node is built against the final
//! numbering and nothing is renumbered afterwards.
//!
//! A terminal a card leaves out, which the card's syntax ties to node 0 (a
//! three-terminal BJT's substrate, a generated model's omitted thermal node,
//! an unconnected Verilog-A port), does not count: it joins the chosen
//! reference, as it would had the deck named that net 0. Nor does a node a
//! module keeps as private state, such as a potential that carries a charge,
//! or a thermal port's tie to ambient.
//!
//! A native device is judged by its card alone. A Verilog-A or Verilog-AMS
//! module is judged by its equations, read from its canonical IR by
//! `rspice_veriloga::canonical_ir::ground_reference`: the build reads them
//! for a module it compiles, and a generated model carries the generator's
//! reading of the same IR as metadata. An equation inside an `if` counts when
//! the instance's own parameter values and connections select it, or when
//! they cannot decide it before the solve; see
//! [`rspice_veriloga_runtime::GroundReference`]. A `.va` file and the
//! generated model built from it therefore answer one card alike.

use super::*;

/// The net a deck that names no node 0 would take as its reference: the
/// negative terminal of its first voltage source. `None` when the deck names
/// ground itself, or has no voltage source to take one from.
///
/// It runs before the terminal-current meters are added, so a meter is never
/// the source it is taken from.
pub(super) fn automatic(elements: &[Element]) -> Option<&str> {
    if elements.iter().any(names_ground) {
        return None;
    }
    elements.iter().find_map(|element| match element.kind {
        ElementKind::VoltageSource(_) => element.nodes.get(1).map(String::as_str),
        _ => None,
    })
}

/// Whether the element connects node 0 on a net the build resolves: a
/// terminal, a controlled source's or switch's control node, a code model's
/// port, or a PSpice digital device's power pin.
fn names_ground(element: &Element) -> bool {
    let ground = |node: &str| crate::naming::is_spice_ground_name(node);
    element.nodes.iter().any(|node| ground(node))
        || match &element.kind {
            ElementKind::Vcvs { control_nodes, .. } | ElementKind::Vccs { control_nodes, .. } => {
                ground(&control_nodes.0) || ground(&control_nodes.1)
            }
            ElementKind::VSwitch {
                control_pos,
                control_neg,
                ..
            } => ground(control_pos) || ground(control_neg),
            ElementKind::Xspice {
                ports,
                pspice_u_timing,
                ..
            } => {
                ports
                    .iter()
                    .any(|port| port.node_names().into_iter().any(ground))
                    || pspice_u_timing
                        .as_ref()
                        .and_then(|timing| timing.power_pins.as_ref())
                        .is_some_and(|(power, reference)| ground(power) || ground(reference))
            }
            _ => false,
        }
}

/// The reference a deck that names no node 0 takes after its first build:
/// `candidate`, unless something in `circuit` connects node 0. It runs once
/// every authored element is built, before the build inserts any automatic
/// converter.
pub(super) fn to_take(candidate: Option<String>, circuit: &CircuitData) -> Option<String> {
    let candidate = candidate?;
    match referent(circuit) {
        Some(referent) => {
            log::info!(
                "The deck names no node 0, but {referent} connects to it; node 0 stays the reference"
            );
            None
        }
        None => Some(candidate),
    }
}

/// What in a built circuit connects node 0 without the deck naming it.
///
/// Three kinds of element can: a Verilog-A or Verilog-AMS instance whose own
/// contributions do (recorded while it was built, see [`note_module`]), a code
/// model or bridge that drives or loads a net against node 0, and a source or
/// load an authored card expands into that stamps node 0. A device's own
/// internal resistors and capacitors are its card's, and follow its terminals.
fn referent(circuit: &CircuitData) -> Option<String> {
    #[cfg(any(feature = "veriloga", feature = "veriloga-builtins-base"))]
    if let crate::circuit::GroundCensus::Referenced(instance) = &circuit.ground_census {
        return Some(format!("'{instance}'"));
    }
    if let Some(instance) = circuit.xspice_instances.iter().find(|instance| {
        instance
            .ports()
            .iter()
            .zip(instance.connections())
            .any(|(port, connection)| port_connects_ground(port, connection))
    }) {
        return Some(format!("'{}'", instance.name));
    }
    #[cfg(feature = "veriloga")]
    if let Some(host) = circuit
        .mixed_signal_hosts
        .iter()
        .find(|host| host.drives_against_ground())
    {
        return Some(format!("a converter of '{}'", host.instance_name()));
    }
    let grounded = |pos: crate::NodeId, neg: crate::NodeId| pos == 0 || neg == 0;
    let resistors = &circuit.resistors;
    let capacitors = &circuit.capacitors;
    let inductors = &circuit.inductors;
    let branches = &circuit.resistor_branches;
    let primitive = (0..resistors.stamps.len())
        .find(|&index| {
            resistors
                .device_owners
                .get(index)
                .is_none_or(Option::is_none)
                && grounded(
                    resistors.stamps[index].pp.row,
                    resistors.stamps[index].nn.row,
                )
        })
        .and_then(|index| resistors.names.get(index))
        .or_else(|| {
            (0..capacitors.stamps.len())
                .find(|&index| {
                    !capacitors.internal.get(index).copied().unwrap_or(false)
                        && grounded(
                            capacitors.stamps[index].pp.row,
                            capacitors.stamps[index].nn.row,
                        )
                })
                .and_then(|index| capacitors.names.get(index))
        })
        .or_else(|| {
            (0..inductors.names.len())
                .find(|&index| grounded(inductors.node_pos[index], inductors.node_neg[index]))
                .map(|index| &inductors.names[index])
        })
        .or_else(|| {
            (0..branches.names.len())
                .find(|&index| grounded(branches.node_pos[index], branches.node_neg[index]))
                .map(|index| &branches.names[index])
        })
        .or_else(|| {
            let sources = &circuit.voltage_sources;
            (0..sources.names.len())
                .find(|&index| grounded(sources.node_pos[index], sources.node_neg[index]))
                .map(|index| &sources.names[index])
        })
        .or_else(|| {
            let sources = &circuit.current_sources;
            (0..sources.names.len())
                .find(|&index| grounded(sources.node_pos[index], sources.node_neg[index]))
                .map(|index| &sources.names[index])
        })
        .or_else(|| {
            [
                (
                    &circuit.vcvs.names,
                    &circuit.vcvs.node_pos,
                    &circuit.vcvs.node_neg,
                ),
                (
                    &circuit.vccs.names,
                    &circuit.vccs.node_pos,
                    &circuit.vccs.node_neg,
                ),
                (
                    &circuit.cccs.names,
                    &circuit.cccs.node_pos,
                    &circuit.cccs.node_neg,
                ),
                (
                    &circuit.ccvs.names,
                    &circuit.ccvs.node_pos,
                    &circuit.ccvs.node_neg,
                ),
            ]
            .into_iter()
            .find_map(|(names, pos, neg)| {
                (0..names.len())
                    .find(|&index| grounded(pos[index], neg[index]))
                    .map(|index| &names[index])
            })
        })
        .or_else(|| {
            let sources = &circuit.behavioral_sources;
            sources
                .voltage_sources
                .iter()
                .find(|source| grounded(source.node_pos, source.node_neg))
                .map(|source| &source.name)
                .or_else(|| {
                    sources
                        .current_sources
                        .iter()
                        .find(|source| grounded(source.node_pos, source.node_neg))
                        .map(|source| &source.name)
                })
        });
    primitive.map(|name| format!("'{name}'"))
}

/// Whether a code-model port connects node 0 to the circuit: it stamps a
/// branch with node 0 at one end, as an output or inout port's drive, a
/// current input's zero-volt probe and a hybrid port do, single-ended or as a
/// pair with ground at one end. A voltage input only senses its nets and
/// stamps nothing at them, single-ended or not, so it does not count.
fn port_connects_ground(
    port: &crate::xspice::PortSpec,
    connection: &crate::xspice::PortConnection,
) -> bool {
    use crate::xspice::{AnalogInputConnection, PortConnection, PortDirection, PortType};

    let senses = port.direction == PortDirection::In
        && matches!(
            port.default_type,
            PortType::Voltage | PortType::DifferentialVoltage
        );
    let grounded = |pos: usize, neg: usize| pos == 0 || neg == 0;
    match connection {
        PortConnection::Analog(_) | PortConnection::AnalogVector(_) => !senses,
        PortConnection::Differential(pos, neg) => !senses && grounded(*pos, *neg),
        PortConnection::CurrentProbe { pos, neg, .. }
        | PortConnection::CurrentOutput { pos, neg }
        | PortConnection::VoltageOutput { pos, neg }
        | PortConnection::Hybrid { pos, neg, .. } => grounded(*pos, *neg),
        PortConnection::TypedAnalogVector(elements) => {
            elements.iter().any(|element| match element {
                AnalogInputConnection::Node(_) => !senses,
                AnalogInputConnection::Differential(pos, neg) => !senses && grounded(*pos, *neg),
                AnalogInputConnection::CurrentProbe { pos, neg, .. }
                | AnalogInputConnection::CurrentOutput { pos, neg }
                | AnalogInputConnection::VoltageOutput { pos, neg }
                | AnalogInputConnection::Hybrid { pos, neg, .. } => grounded(*pos, *neg),
                AnalogInputConnection::BranchCurrent { .. }
                | AnalogInputConnection::NamedBranchCurrent { .. }
                | AnalogInputConnection::NamedCurrentSource { .. } => false,
            })
        }
        PortConnection::Digital(_)
        | PortConnection::DigitalInverted(_)
        | PortConnection::Real(_)
        | PortConnection::DigitalVector(_)
        | PortConnection::DigitalVectorMapped(_)
        | PortConnection::RealVector(_)
        | PortConnection::BranchCurrent { .. }
        | PortConnection::NamedBranchCurrent { .. }
        | PortConnection::NamedCurrentSource { .. }
        | PortConnection::Null => false,
    }
}

/// Record an instance whose module's contributions connect node 0, while the
/// build still has a deck's reference to decide.
#[cfg(any(feature = "veriloga", feature = "veriloga-builtins-base"))]
fn note_instance(circuit: &mut CircuitData, instance: &str, references: impl FnOnce() -> bool) {
    if circuit.ground_census == crate::circuit::GroundCensus::Open && references() {
        circuit.ground_census = crate::circuit::GroundCensus::Referenced(instance.to_owned());
    }
}

/// Note a Verilog-A or Verilog-AMS instance compiled at run time, once its
/// device is built: its module's contributions, read from the canonical IR by
/// [`rspice_veriloga::canonical_ir::ground_reference`], decided with the
/// device's own parameter values and connections.
///
/// A module whose canonical IR is absent cannot be read, and counts as
/// connecting node 0, so the build never chooses a reference it cannot
/// justify.
#[cfg(feature = "veriloga")]
pub(super) fn note_module(
    circuit: &mut CircuitData,
    instance: &str,
    artifact: Option<&rspice_veriloga::canonical_ir::CanonicalIrArtifact>,
    device: &crate::device::veriloga::VerilogADevice,
) {
    note_instance(circuit, instance, || {
        module_references_ground(artifact, &RuntimeInstance(device))
    });
}

#[cfg(feature = "veriloga")]
fn module_references_ground(
    artifact: Option<&rspice_veriloga::canonical_ir::CanonicalIrArtifact>,
    instance: &dyn rspice_veriloga_runtime::GroundReferenceInputs,
) -> bool {
    artifact.is_none_or(|artifact| {
        rspice_veriloga::canonical_ir::ground_reference(artifact)
            .view()
            .references_ground(instance)
    })
}

/// Note a generated model's instance: the generator recorded its module's
/// answer from the same canonical IR, and the instance decides it with its
/// own parameter values. Every port of a generated device is connected, as
/// its `$port_connected` answers.
#[cfg(feature = "veriloga-builtins-base")]
pub(super) fn note_generated(
    circuit: &mut CircuitData,
    device: &crate::device::veriloga_builtins::BuiltinVerilogAInstance,
) {
    note_instance(circuit, device.instance_name(), || {
        device.references_ground()
    });
}

#[cfg(feature = "veriloga")]
struct RuntimeInstance<'a>(&'a crate::device::veriloga::VerilogADevice);

#[cfg(feature = "veriloga")]
impl rspice_veriloga_runtime::GroundReferenceInputs for RuntimeInstance<'_> {
    fn parameter(&self, name: &str) -> Option<f64> {
        self.0.parameter_state(name).map(|(value, _)| value)
    }

    fn parameter_given(&self, name: &str) -> Option<bool> {
        self.0.parameter_state(name).map(|(_, given)| given)
    }

    fn port_connected(&self, port: u32) -> Option<bool> {
        self.0.port_connected(usize::try_from(port).ok()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reference(deck: &str) -> Option<String> {
        let netlist = Netlist::parse(deck).expect("deck parses");
        automatic(&netlist.elements).map(str::to_ascii_lowercase)
    }

    #[test]
    fn the_first_voltage_sources_negative_net_is_the_reference() {
        assert_eq!(
            reference("* first source\nr1 a b 1k\ni1 b c 1m\nv2 a c 1\nv1 b a 1\n.end\n")
                .as_deref(),
            Some("c")
        );
    }

    #[test]
    fn a_deck_that_names_node_zero_anywhere_keeps_it() {
        for deck in [
            "* terminal\nv1 a b 1\nr1 b 0 1k\n.end\n",
            "* control node\nv1 a b 1\nr1 a b 1k\ne1 c b a 0 2\n.end\n",
            "* switch control\nv1 a b 1\nr1 a b 1k\ns1 a b c 0 sw\n.model sw sw\n.end\n",
            "* code model port\nv1 a b 1\nr1 a b 1k\na1 0 b amp\n.model amp gain (gain=2)\n.end\n",
        ] {
            assert_eq!(reference(deck), None, "{deck}");
        }
    }

    #[test]
    fn a_deck_without_a_voltage_source_has_no_reference_to_take() {
        assert_eq!(reference("* no source\ni1 a b 1m\nr1 a b 1k\n.end\n"), None);
    }

    #[cfg(feature = "veriloga")]
    #[test]
    fn a_module_without_its_canonical_ir_counts_as_referencing_node_zero() {
        struct Nothing;
        impl rspice_veriloga_runtime::GroundReferenceInputs for Nothing {
            fn parameter(&self, _: &str) -> Option<f64> {
                None
            }
            fn parameter_given(&self, _: &str) -> Option<bool> {
                None
            }
            fn port_connected(&self, _: u32) -> Option<bool> {
                None
            }
        }
        assert!(module_references_ground(None, &Nothing));
    }
}
