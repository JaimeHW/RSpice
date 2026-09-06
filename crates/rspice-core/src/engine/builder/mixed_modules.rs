//! Elaborating an X-card that names a mixed Verilog-AMS module.
//!
//! The `.VERILOGA` route already turns a `.va` file into a compiled model and a
//! canonical artifact, and an X-card into a device with its terminals bound to
//! deck nodes. A mixed module is the same route with one branch taken
//! differently: its canonical artifact carries a non-empty discrete plan, and
//! that plan is what decides it goes to
//! [`MixedSignalHost`](crate::xspice::verilog::MixedSignalHost) rather than to
//! [`VerilogADevice`](crate::device::veriloga::VerilogADevice). Nothing else
//! about the route changes — the same cache entry, the same compile, the same
//! terminal binding.
//!
//! # Which ports are the boundary
//!
//! The compiled artifact's HIR lists every module port with the direction its
//! author wrote; the discrete plan lists every signal the discrete domain
//! declares. A port whose name is in the plan is a *boundary* port, and the
//! plan is the authority rather than the port's declared discipline, because
//! the plan is also what `add_adc_bridge` and `add_dac_bridge` resolve a name
//! against — so the classification here and the lookup there cannot disagree.
//!
//! A boundary port faces one way, and clause 7's own vocabulary names which:
//!
//! * `input` — the module reads the net, so the analog side drives and the
//!   boundary is analog-to-discrete. It takes an A/D bridge.
//! * `output` — the module drives the net, so the boundary is
//!   discrete-to-analog. It takes a D/A bridge.
//! * `inout` — bidirectional, which needs a bridge that arbitrates who is
//!   driving. The host has A/D and D/A and no third kind, so this is refused by
//!   name rather than approximated with one of them.
//!
//! # A vector boundary port
//!
//! A discrete port may be a vector, and a vector boundary is one net per bit:
//! the deck names N nodes for an N-bit port, in the order the port declares its
//! bits — MSB first — and each gets its own bridge. That is not a convention
//! invented here. It is the only spelling a deck has, because a node list is
//! flat; it is what `rspice-ui`'s netlister already emits for a vector pin; and
//! it is what makes the port's bits recordable at all, since every value that
//! reaches a result is keyed by circuit node.
//!
//! The bits are still one word, and saying so is the bus declaration this
//! builder attaches to the host: `<instance>.<port>`, the range the module
//! declared, and the member nodes MSB first. The discrete half never sees the
//! split — its own transitions stay whole-vector, because the A/D settle
//! composes a port's bit drives into one write.
//!
//! # Where the bridge's numbers come from
//!
//! From the same two places the XSPICE auto-bridge's do, and by the same rule.
//! With no `connectrules` block the thresholds and levels derive from the
//! deck's supply exactly as `add_planned_xspice_auto_bridge` derives them: an
//! A/D bridge switches at half supply, a D/A bridge drives zero and supply. A
//! design that *does* name connect rules gets them from clause 7's selection —
//! `connect_modules::select_for_boundary` picks the module and
//! `connect_modules::delegated_parameters` folds section 7.7.3's overrides —
//! which is the same delegation table the XSPICE bridges take, read here rather
//! than re-derived.
//!
//! Two things the code-model bridges have and this boundary does not are
//! transition ramps and propagation delays. `d2a`'s `trise`/`tfall` and `a2d`'s
//! `tdrise`/`tdfall` are executed by the code models' own transition machinery,
//! which this Thevenin bridge is not; a connect statement that asks for one is
//! therefore refused rather than silently ignored.

use crate::xspice::event_scheduler::SchedulerLimits;
use crate::xspice::verilog::{BoundaryBus, MixedSignalHost};
use crate::{CircuitData, SimulationError};

use super::connect_modules::{self, DesignConnectRules};
use super::veriloga_cache::CachedVerilogAModel;

/// The Thevenin source resistance a D/A boundary drives through.
///
/// `bidi_bridge`'s `r_stl`/`r_sth` — the strong-drive resistances the XSPICE
/// bridge library already states for exactly this job — rather than a number
/// invented here. It is small against any load a deck is likely to hang on a
/// logic output, so the node settles to the driven level, and finite so the
/// node's row is never singular when nothing else is attached to it.
const MIXED_DAC_SOURCE_RESISTANCE: crate::Value = 20.0;

/// Which way one boundary port faces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BoundaryDirection {
    /// The analog side drives and the module reads.
    AnalogToDiscrete,
    /// The module drives and the analog side reads.
    DiscreteToAnalog,
}

impl BoundaryDirection {
    fn auto_bridge_kind(self) -> super::XspiceAutoBridgeKind {
        match self {
            Self::AnalogToDiscrete => super::XspiceAutoBridgeKind::Adc,
            Self::DiscreteToAnalog => super::XspiceAutoBridgeKind::Dac,
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
    /// Position of this net in the X-card's own node list, so a diagnostic can
    /// quote the node the deck wrote rather than searching for one that
    /// matches.
    deck_index: usize,
    direction: BoundaryDirection,
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
/// `Ok(false)` means the model's canonical artifact carries no discrete plan,
/// which is every analog `.va` there has ever been: the caller takes the device
/// route it always took.
pub(super) fn try_build_mixed_signal_instance(
    circuit: &mut CircuitData,
    netlist: &crate::Netlist,
    element: &crate::netlist::Element,
    subckt_name: &str,
    params: &[(String, crate::netlist::ParametricValue)],
    entry: &CachedVerilogAModel,
    connect_rules: &DesignConnectRules,
) -> Result<bool, SimulationError> {
    let Some(artifact) = entry.canonical_ir.as_deref() else {
        // Only a native build can reach here without an artifact, and it has
        // already refused for its own reason; an interpreter build without one
        // simply has no plan to consult, which is the analog route.
        return Ok(false);
    };
    if artifact.digital.is_empty() {
        return Ok(false);
    }

    let model = &entry.model;
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
        return Err(SimulationError::Circuit(format!(
            "mixed Verilog-AMS instance '{}' connects {} nodes to model '{}', {shape}",
            element.name,
            element.nodes.len(),
            subckt_name,
        )));
    }
    if !params.is_empty() {
        return Err(SimulationError::Circuit(format!(
            "mixed Verilog-AMS instance '{}' passes instance parameters, which this route does \
             not carry into the module yet: the analog half's parameter defaults are resolved at \
             construction and the discrete half's are folded into its compiled plan, so an \
             override would reach one and not the other",
            element.name
        )));
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
    for port in boundary {
        if circuit.is_discrete_net(port.node) {
            return Err(SimulationError::Circuit(format!(
                "mixed Verilog-AMS instance '{}' connects its discrete port '{}' to node '{}', \
                 which is already an event-driven XSPICE net. A mixed module's discrete port is a \
                 discipline boundary onto an analog net; joining it to another device's event net \
                 needs the module to share the circuit event queue, which this route does not do",
                element.name, port.signal, element.nodes[port.deck_index]
            )));
        }
    }

    let mut host = MixedSignalHost::from_compiled(
        &element.name,
        std::sync::Arc::clone(model),
        artifact,
        &layout.analog_terminals,
        SchedulerLimits::default(),
    )
    .map_err(|error| {
        SimulationError::Circuit(format!(
            "mixed Verilog-AMS instance '{}' of model '{}' could not be started: {error}",
            element.name, subckt_name
        ))
    })?;

    let vcc = super::xspice_auto_bridge_vcc(netlist);
    let node_names = circuit.node_names_sorted();
    for port in boundary {
        let node_label = super::xspice_auto_bridge_node_label(Some(&node_names), port.node);
        let kind = port.direction.auto_bridge_kind();
        let selected = connect_rules.select_for_boundary_node(
            kind,
            &node_label,
            &element.name,
            &port.signal,
        )?;
        let parameters = match selected.as_ref() {
            Some(selected) => {
                connect_modules::check_delegable(selected, kind, &node_label)?;
                let folded = connect_modules::delegated_parameters(selected, kind, vcc)?;
                refuse_timed_connect_parameters(&element.name, &port.signal, selected, &folded)?;
                log::info!(
                    "Mixed module port '{}' on node '{}' bridges through connect module '{}' as \
                     instance '{}'",
                    port.signal,
                    node_label,
                    selected.name,
                    selected.instance
                );
                folded
            }
            None => Vec::new(),
        };

        match port.direction {
            BoundaryDirection::AnalogToDiscrete => {
                let low = parameter_or(&parameters, "in_low", vcc / 2.0);
                let high = parameter_or(&parameters, "in_high", vcc / 2.0);
                host.add_adc_bridge(&port.signal, port.bit, (port.node, 0), low, high)
            }
            BoundaryDirection::DiscreteToAnalog => {
                let low = parameter_or(&parameters, "out_low", 0.0);
                let high = parameter_or(&parameters, "out_high", vcc);
                host.add_dac_bridge(
                    &port.signal,
                    port.bit,
                    (port.node, 0),
                    low,
                    high,
                    MIXED_DAC_SOURCE_RESISTANCE,
                )
            }
        }
        .map_err(|error| {
            SimulationError::Circuit(format!(
                "mixed Verilog-AMS instance '{}' could not bridge port '{}': {error}",
                element.name, port.signal
            ))
        })?;
    }

    for bus in layout.buses {
        let name = bus.name.clone();
        host.declare_boundary_bus(bus).map_err(|error| {
            SimulationError::Circuit(format!(
                "mixed Verilog-AMS instance '{}' could not declare boundary bus '{name}': {error}",
                element.name
            ))
        })?;
    }

    log::info!(
        "Instantiated mixed Verilog-AMS module '{}' as '{}' with {} boundary net(s) in {} bus(es)",
        subckt_name,
        element.name,
        boundary.len(),
        host.boundary_buses().len()
    );
    circuit.add_mixed_signal_host(host);
    Ok(true)
}

/// How many deck nodes an X-card must name for this module.
///
/// One per module port, except that a discrete port carries one net per bit —
/// its own refusal used to say so ("a vector boundary needs one net per bit and
/// the deck names one node"), and this is that sentence answered rather than
/// refused.
///
/// A `wreal` port counts as one, not as its zero-bit width: it is refused by
/// name in [`classify_boundary_ports`], and counting it as nothing here would
/// make the node count disagree first and refuse it for the wrong reason.
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
///   and the order `DigitalBusDeclaration` lists members in. Bit *k* of the
///   value is `FourStateValue::bit(k)`, counted from the least significant end,
///   which is where the discrete half's own bit selects land.
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
        if signal.kind.is_real() {
            return Err(SimulationError::Circuit(format!(
                "mixed Verilog-AMS instance '{}' of model '{}' declares real-valued port '{}'. A \
                 `wreal` boundary carries a real number rather than a discipline's potential and \
                 flow, so it is not the A/D or D/A boundary this route bridges",
                element.name, subckt_name, port.name
            )));
        }
        let direction = match port.direction.as_str() {
            "input" => BoundaryDirection::AnalogToDiscrete,
            "output" => BoundaryDirection::DiscreteToAnalog,
            other => {
                return Err(SimulationError::Circuit(format!(
                    "mixed Verilog-AMS instance '{}' of model '{}' declares discrete port '{}' as \
                     `{other}`; a bidirectional discrete boundary needs a bridge that arbitrates \
                     which side is driving, and this route has an analog-to-discrete and a \
                     discrete-to-analog bridge and no third kind",
                    element.name, subckt_name, port.name
                )));
            }
        };
        let bits = boundary_bit_order(signal).ok_or_else(|| {
            SimulationError::Circuit(format!(
                "mixed Verilog-AMS instance '{}' of model '{}' declares discrete port '{}' as \
                 `[{}:{}]`, a range that does not reach bit zero; the discrete half numbers a bit \
                 select from the least significant end, so a boundary net for its declared \
                 most-significant bit would carry a bit the module itself cannot read",
                element.name,
                subckt_name,
                port.name,
                signal.bounds.map_or(0, |(msb, _)| msb),
                signal.bounds.map_or(0, |(_, lsb)| lsb),
            ))
        })?;
        for (offset, bit) in bits.iter().copied().enumerate() {
            let deck_index = first + offset;
            let node = terminal_nodes[deck_index];
            if node == 0 {
                return Err(SimulationError::Circuit(format!(
                    "mixed Verilog-AMS instance '{}' connects discrete port '{}' to ground; a \
                     boundary net carries a logic value and ground is the voltage reference, not \
                     a net",
                    element.name, port.name
                )));
            }
            layout.ports.push(BoundaryPort {
                signal: port.name.to_string(),
                bit,
                node,
                deck_index,
                direction,
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

/// The bits of one boundary port, in the order the deck names its nets:
/// declared MSB first.
///
/// `None` when the declared range does not reach bit zero. The discrete half
/// resolves a bit select by treating the declared index *as* the position from
/// the least significant end — `digital_eval`'s `patch` and
/// `digital_value::part_select` both say so in as many words — so `[7:4]` is a
/// four-bit value whose own `x[7]` reads off the end of it. Bridging such a
/// port would bind four deck nets to four bits the module cannot name, which is
/// a wrong answer rather than a missing one, so the boundary refuses it.
///
/// A scalar port has no range and is bit zero, which is what it has always
/// been.
fn boundary_bit_order(signal: &rspice_veriloga::canonical_ir::DigitalSignal) -> Option<Vec<u32>> {
    let Some((msb, lsb)) = signal.bounds else {
        return Some(vec![0]);
    };
    if msb.min(lsb) != 0 {
        return None;
    }
    let high = u32::try_from(msb.max(lsb)).ok()?;
    if high.checked_add(1)? != signal.width {
        return None;
    }
    Some(if msb >= lsb {
        (0..=high).rev().collect()
    } else {
        (0..=high).collect()
    })
}

fn parameter_or(
    parameters: &[(String, crate::Value)],
    name: &str,
    fallback: crate::Value,
) -> crate::Value {
    parameters
        .iter()
        .find(|(parameter, _)| parameter.eq_ignore_ascii_case(name))
        .map_or(fallback, |(_, value)| *value)
}

/// Refuse a connect statement asking for a transition time this boundary cannot
/// produce.
///
/// `delegated_parameters` maps clause 7's `trise`/`tfall`/`tdrise`/`tdfall` onto
/// the XSPICE bridge code models' own transition parameters, which those models
/// execute. This boundary is a Thevenin source and a threshold comparator with
/// no transition machinery of either kind, so carrying the numbers would be
/// accepting a request and not honouring it.
fn refuse_timed_connect_parameters(
    instance: &str,
    signal: &str,
    selected: &connect_modules::PlannedConnectModule,
    folded: &[(String, crate::Value)],
) -> Result<(), SimulationError> {
    const TIMED: &[&str] = &["rise_delay", "fall_delay", "t_rise", "t_fall"];
    for (name, _) in folded {
        if TIMED.iter().any(|timed| name.eq_ignore_ascii_case(timed)) {
            return Err(SimulationError::Circuit(format!(
                "mixed Verilog-AMS instance '{instance}' port '{signal}' selects connect module \
                 '{}', whose connect statement sets a transition time. The mixed boundary drives \
                 through a source resistance and samples against a threshold, with no transition \
                 or delay stage to apply one to",
                selected.name
            )));
        }
    }
    Ok(())
}
