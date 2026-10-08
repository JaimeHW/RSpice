//! Elaborate mixed Verilog-AMS X-cards into one analog device and typed ports.
//!
//! Typed ports select electrical conversion after the entire deck is wired,
//! using `mixed_boundaries` and the common converter materializer. Pure event
//! connections join the shared resolver. Input threshold detectors retain the
//! host's root-localization and feedback-causality contract while the common
//! converter scheduler owns digital publication and propagation delay.
//! Packed port order and deck trace/bus labels survive private event endpoints.

use crate::xspice::event_scheduler::SchedulerLimits;
use crate::xspice::verilog::{BoundaryBus, MixedSignalHost};
use crate::{CircuitData, ElaborationError, ElaborationErrorKind, SimulationError};

use super::veriloga_instances::PreparedInstance;

/// Every refusal this module raises is about one X-card bound to one master,
/// so it is built once here rather than formatted at each site: the instance
/// and the master are the subject the rendering prints, and the `detail` each
/// site passes is the reason alone.
fn refuse(
    instance: &str,
    module: &str,
    kind: ElaborationErrorKind,
    detail: impl Into<String>,
) -> SimulationError {
    ElaborationError::new(kind, detail)
        .instance(instance)
        .module(module)
        .into()
}

/// Which binding step a host construction or update failure belongs to.
///
/// `MixedSignalError` already separates a source the compiler would not build
/// from a bridge declaration it will not execute, and both are the author's to
/// fix; everything else it can return at elaboration time is the engine
/// failing a step it expected to complete.
fn host_failure_kind(error: &crate::xspice::verilog::MixedSignalError) -> ElaborationErrorKind {
    use crate::xspice::verilog::MixedSignalError as Error;
    match error {
        Error::Compile { .. } => ElaborationErrorKind::CompileRefusal,
        Error::InvalidBridge { .. } => ElaborationErrorKind::PortDiscipline,
        _ => ElaborationErrorKind::Internal,
    }
}

/// Which way one boundary port faces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BoundaryDirection {
    /// The analog side drives and the module reads.
    AnalogToDiscrete,
    /// The module drives and the analog side reads.
    DiscreteToAnalog,
    Bidirectional,
}

impl BoundaryDirection {
    fn link_direction(self) -> rspice_veriloga::canonical_ir::digital_link::DigitalLinkDirection {
        use rspice_veriloga::canonical_ir::digital_link::DigitalLinkDirection as Direction;
        match self {
            Self::AnalogToDiscrete => Direction::Input,
            Self::DiscreteToAnalog => Direction::Output,
            Self::Bidirectional => Direction::Inout,
        }
    }
}

/// One bit of one boundary port, resolved to the deck node it landed on.
///
/// A scalar port contributes one of these and a vector port contributes one
/// per bit, because the deck names one node per conductor and a bridge carries
/// one node.
struct BoundaryPort {
    /// The module's name for the whole signal, which is what the discrete
    /// plan is keyed by — `count`, not `count[1]`.
    signal: String,
    /// Which bit of that signal this net carries, counted from the least
    /// significant end.
    bit: u32,
    node: usize,
    direction: BoundaryDirection,
    real: bool,
}

/// What one boundary port declared, once its bits are deck nodes.
struct BoundaryLayout {
    /// One node per HIR port, in port order — what the continuous half binds.
    analog_terminals: Vec<usize>,
    /// Every bridged net, in port order and declared MSB first within a port.
    ports: Vec<BoundaryPort>,
    /// One entry per vector boundary port, over the nodes its bits landed on.
    buses: Vec<BoundaryBus>,
}

/// Build a mixed module instance, or report that this model is not mixed.
///
/// `Ok(false)` means the effective artifact needs no discrete runtime. The
/// caller builds an analog device from the same prepared instance.
pub(super) fn try_build_mixed_signal_instance(
    circuit: &mut CircuitData,
    element: &crate::netlist::Element,
    prepared: &PreparedInstance<'_>,
    // Every net the flattened deck authors, for the one check that this
    // instance's own unknowns are not about to take a name already in use.
    authored_nets: &std::collections::HashSet<String>,
    abort: &dyn crate::abort_signal::AbortSignal,
) -> Result<bool, SimulationError> {
    let crate::netlist::ElementKind::Subcircuit { subckt_name, .. } = &element.kind else {
        return Ok(false);
    };
    let Some(artifact) = prepared.canonical_ir.as_deref() else {
        // Only a native build can reach here without an artifact, and it has
        // already refused for its own reason; an interpreter build without one
        // simply has no plan to consult, which is the analog route.
        return Ok(false);
    };
    if !artifact.digital.has_executable_content() {
        return Ok(false);
    }

    let model = &prepared.model;
    let declared_nodes = declared_node_count(artifact);
    if element.nodes.len() != declared_nodes {
        // `num_terminals` is one per module port and says nothing about how
        // wide a port is (`rspice-veriloga`'s `canonical_compat` pins it equal
        // to `hir.ports.len()`), so it is not the number the deck must match
        // once a vector discrete port takes one node per bit. It is still what
        // the sentence should say when nothing is a vector, because then the
        // two numbers are the same and "ports" is what an author counted.
        let shape = if declared_nodes == model.num_terminals {
            format!(
                "which declares {} ports; a mixed module's discrete ports are part of its \
                 boundary, so every port must be connected",
                model.num_terminals
            )
        } else {
            format!(
                "which declares {} ports needing {declared_nodes} nodes; a mixed module's \
                 discrete ports are part of its boundary and a vector discrete port is one net \
                 per bit, so every bit must be connected",
                model.num_terminals
            )
        };
        return Err(refuse(
            &element.name,
            subckt_name,
            ElaborationErrorKind::PortCount,
            format!("connects {} nodes to a master {shape}", element.nodes.len()),
        ));
    }

    let mut terminal_nodes = Vec::with_capacity(element.nodes.len());
    for node_name in &element.nodes {
        terminal_nodes.push(if node_name.eq_ignore_ascii_case("0") {
            0
        } else {
            circuit.get_or_create_node(node_name)
        });
    }

    let layout = classify_boundary_ports(artifact, element, subckt_name, &terminal_nodes)?;
    let boundary = &layout.ports;
    // The continuous half of a mixed instance brings the same internal nodes
    // and branch currents into the same circuit, so it takes the same names
    // and is checked against the same deck nets. A deck author moving a module
    // across the two routes by adding a process must not have to rewrite the
    // probes that read it.
    let internal_names = super::veriloga_internal_node_names(Some(artifact), model.internal_nodes);
    let branch_names = super::veriloga_branch_unknown_names(model);
    let unknown_names = super::VerilogAUnknownNames {
        internal: &internal_names,
        branches: &branch_names,
    };
    unknown_names
        .check_free(
            &element.name,
            model.internal_nodes,
            model.branch_sources.len(),
            authored_nets,
        )
        .map_err(|(kind, detail)| refuse(&element.name, subckt_name, kind, detail))?;
    // Event connections may be declared by a later A-card or a generated
    // bridge. The completed circuit validates this boundary after all of
    // them are registered; querying a partial node table here is order dependent.

    let mut host = MixedSignalHost::from_compiled_with_analog_setup(
        &element.name,
        std::sync::Arc::clone(model),
        artifact,
        &layout.analog_terminals,
        &prepared.overrides,
        SchedulerLimits::default(),
        circuit.generated_simulation_parameters,
        &super::veriloga_cache::VerilogACompileControl { abort },
        &mut |device| {
            super::bind_veriloga_solver_unknowns(circuit, &element.name, device, &unknown_names)
        },
    )
    .map_err(|error| {
        if abort.is_aborted() {
            return SimulationError::Aborted;
        }
        refuse(
            &element.name,
            subckt_name,
            host_failure_kind(&error),
            format!("could not be constructed: {error}"),
        )
    })?;

    // After the setup closure has bound this instance's solver unknowns and
    // before the analysis starts, which is where the analog route applies the
    // same factor to its own device.
    if let Some(multiplicity) = prepared.multiplicity {
        host.set_multiplicity(multiplicity).map_err(|error| {
            refuse(
                &element.name,
                subckt_name,
                host_failure_kind(&error),
                format!("multiplicity update failed: {error}"),
            )
        })?;
    }

    host.set_temperature(prepared.temperature)
        .map_err(|error| {
            refuse(
                &element.name,
                subckt_name,
                host_failure_kind(&error),
                format!("temperature update failed: {error}"),
            )
        })?;

    for port in boundary {
        host.declare_event_port(
            &port.signal,
            (!port.real).then_some(port.bit),
            port.node,
            port.direction.link_direction(),
        )
        .map_err(|error| {
            refuse(
                &element.name,
                subckt_name,
                host_failure_kind(&error),
                error.to_string(),
            )
        })?;
    }

    for bus in layout.buses {
        let name = bus.name.clone();
        host.declare_boundary_bus(bus).map_err(|error| {
            refuse(
                &element.name,
                subckt_name,
                host_failure_kind(&error),
                format!("could not declare boundary bus '{name}': {error}"),
            )
        })?;
    }

    log::info!(
        "Instantiated mixed Verilog-AMS module '{}' as '{}' with {} boundary net(s) in {} bus(es)",
        subckt_name,
        element.name,
        boundary.len(),
        host.boundary_buses().len()
    );
    circuit.add_mixed_signal_host(host)?;
    Ok(true)
}

/// How many deck nodes an X-card must name for this module.
///
/// One per module port, except that a discrete port carries one net per bit —
/// its own refusal used to say so ("a vector boundary needs one net per bit and
/// the deck names one node"), and this is that sentence answered rather than
/// refused.
///
/// A real-valued port occupies one event net, despite having no bit width.
fn declared_node_count(artifact: &rspice_veriloga::canonical_ir::CanonicalIrArtifact) -> usize {
    artifact
        .hir
        .ports
        .iter()
        .map(|port| {
            artifact
                .digital
                .signals
                .iter()
                .find(|signal| signal.name == port.name)
                .filter(|signal| !signal.kind.is_real())
                .map_or(1, |signal| signal.width.max(1) as usize)
        })
        .sum()
}

/// Split the module's ports into the analog terminals the continuous half
/// stamps and the boundary nets the bridges carry.
///
/// # Why a port is not a node any more
///
/// The X-card's node list is read with a cursor rather than by port index,
/// because a discrete port of width N occupies N consecutive entries in it.
/// The two orders that makes this work are both fixed elsewhere and are not
/// choices taken here:
///
/// * *Ports against nodes.* `rspice-veriloga` pins `CompiledModel.num_terminals`
///   and `terminal_names` to `hir.ports` name for name (`canonical_compat.rs`),
///   and `VerilogADevice` binds terminal *i* to the *i*th node it is handed, so
///   HIR port order **is** X-card node order. The cursor generalizes that to
///   "the *i*th port's nets, in order".
/// * *Bits against nodes.* A vector port's nets are the deck's, declared MSB
///   first — the order `rspice-ui`'s netlister emits a vector pin's formals in,
///   and the order `DigitalBusDeclaration` lists members in. Which bit of the
///   value each one is comes from `VectorBounds::position_of`, the discrete
///   half's own rule for what a declared index names, so a `[7:4]` port's first
///   net is its bit 7 and is stored at position 3.
///
/// The continuous half still binds one node per port, because its terminal
/// count is one per port: a vector discrete port hands it the net its declared
/// MSB landed on. That is exactly what a scalar port handed it, generalized —
/// the analog equations cannot contribute to a discrete port anyway, since the
/// discrete plan is what makes it a boundary.
fn classify_boundary_ports(
    artifact: &rspice_veriloga::canonical_ir::CanonicalIrArtifact,
    element: &crate::netlist::Element,
    subckt_name: &str,
    terminal_nodes: &[usize],
) -> Result<BoundaryLayout, SimulationError> {
    let mut layout = BoundaryLayout {
        analog_terminals: Vec::with_capacity(artifact.hir.ports.len()),
        ports: Vec::new(),
        buses: Vec::new(),
    };
    let mut cursor = 0usize;
    for port in &artifact.hir.ports {
        let signal = artifact
            .digital
            .signals
            .iter()
            .find(|signal| signal.name == port.name);
        let width = match signal {
            Some(signal) if !signal.kind.is_real() => signal.width.max(1) as usize,
            _ => 1,
        };
        let first = cursor;
        cursor += width;
        layout.analog_terminals.push(terminal_nodes[first]);
        let Some(signal) = signal else {
            continue;
        };
        let direction = match port.direction.as_str() {
            "input" => BoundaryDirection::AnalogToDiscrete,
            "output" => BoundaryDirection::DiscreteToAnalog,
            "inout" => BoundaryDirection::Bidirectional,
            other => {
                return Err(refuse(
                    &element.name,
                    subckt_name,
                    ElaborationErrorKind::PortDiscipline,
                    format!(
                        "declares discrete port '{}' as `{other}`; a bidirectional discrete \
                         boundary needs a bridge that arbitrates which side is driving, and this \
                         route has an analog-to-discrete and a discrete-to-analog bridge and no \
                         third kind",
                        port.name
                    ),
                ));
            }
        };
        let bits = (if signal.kind.is_real() {
            Some(vec![0])
        } else {
            boundary_bit_order(signal)
        })
        .ok_or_else(|| {
            refuse(
                &element.name,
                subckt_name,
                ElaborationErrorKind::Internal,
                format!(
                    "declares discrete port '{}' as `[{}:{}]`, a range naming {} bit(s), and \
                     carries a value {} bit(s) wide; the plan the boundary was handed contradicts \
                     itself",
                    port.name,
                    signal.bounds.map_or(0, |(msb, _)| msb),
                    signal.bounds.map_or(0, |(_, lsb)| lsb),
                    signal.declared_range().width(),
                    signal.width,
                ),
            )
        })?;
        for (offset, bit) in bits.iter().copied().enumerate() {
            let deck_index = first + offset;
            let node = terminal_nodes[deck_index];
            layout.ports.push(BoundaryPort {
                signal: port.name.to_string(),
                bit,
                node,
                direction,
                real: signal.kind.is_real(),
            });
        }
        if let Some((msb, lsb)) = signal.bounds {
            layout.buses.push(BoundaryBus {
                name: format!("{}.{}", element.name, port.name),
                msb,
                lsb,
                members: terminal_nodes[first..first + bits.len()].to_vec(),
            });
        }
    }
    Ok(layout)
}

/// Where each of one boundary port's bits is stored, in the order the deck
/// names its nets: declared MSB first.
///
/// The deck's first net for a vector port is the port's most significant bit,
/// which is the *left* declared bound whatever its value (IEEE 1364-2005
/// section 3.3.1). Which bit of the stored value that is comes from
/// `VectorBounds::position_of`, the discrete half's own rule, so the boundary
/// and the kernel cannot disagree about which conductor is which bit: `[1:0]`
/// yields positions 1, 0 and so does `[0:1]`, because the leading declared
/// index is the most significant bit in both.
///
/// Where the range starts does not enter into it. `[7:4]` is a four-bit port
/// whose deck nets are its bits 7, 6, 5 and 4, stored at positions 3, 2, 1 and
/// 0 — the same four conductors a `[3:0]` port has, under the names the module
/// uses for them. This function refused such a port for as long as the discrete
/// half read a declared index as a position; now that it reads one as a name,
/// there is nothing left to refuse but a plan whose width and bounds contradict
/// each other, which no front end produces.
///
/// A scalar port has no range and is bit zero, which is what it has always
/// been.
fn boundary_bit_order(signal: &rspice_veriloga::canonical_ir::DigitalSignal) -> Option<Vec<u32>> {
    if signal.bounds.is_none() {
        return Some(vec![0]);
    }
    let range = signal.declared_range();
    if range.width() != signal.width {
        return None;
    }
    range
        .indices_msb_first()
        .map(|index| u32::try_from(range.position_of(index)).ok())
        .collect()
}
