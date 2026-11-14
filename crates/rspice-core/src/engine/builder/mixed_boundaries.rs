//! Electrical conversion for typed mixed ports, after the whole deck is wired.
//! Each physical port gets its own event endpoint: an input samples the solved
//! voltage, including a driver's ramp and loading, rather than its target value.
//!
//! A net that also carries XSPICE event values splits as Verilog-AMS splits it
//! (LRM 2.4 section 7.8): the module's ports and the code models' event ports
//! form one digital segment on a shared endpoint, and one converter between
//! that segment and the electrical node serves the analog side.

use super::*;
use rspice_veriloga::canonical_ir::digital_link::DigitalLinkDirection;

pub(super) fn plan_conversions(
    circuit: &mut CircuitData,
    flat_elements: &[Element],
    metadata: &BTreeMap<usize, XspiceAutoBridgeNodeMetadata>,
    supplies: &boundary_supply::BoundarySupplies,
    rules: &connect_modules::DesignConnectRules,
) -> Result<Vec<PlannedXspiceAutoBridge>, SimulationError> {
    if circuit.mixed_signal_hosts.is_empty() {
        return Ok(Vec::new());
    }
    let mut physical = BTreeSet::new();
    collect_flat_analog_nodes(&mut physical, circuit, flat_elements);
    let mut endpoints = BTreeMap::new();
    for host in &circuit.mixed_signal_hosts {
        for node in host.boundary_port_nodes() {
            *endpoints.entry(node).or_insert(0usize) += 1;
        }
    }
    for instance in &circuit.xspice_instances {
        for connection in instance.connections() {
            collect_analog_connection_nodes(&mut physical, connection);
            count_discrete_connection_endpoints(&mut endpoints, connection);
        }
    }

    let names = circuit.node_names_sorted();
    let segments = split_segments(circuit, &physical, &names)?;
    let merged = merged_real_drivers(circuit, &physical)?;
    let mut pending = Vec::new();
    for (host_index, host) in circuit.mixed_signal_hosts.iter().enumerate() {
        for (port_index, (port, signal)) in host.direct_event_ports().enumerate() {
            if port.node > 0
                && !physical.contains(&port.node)
                && endpoints.get(&port.node).copied().unwrap_or(0) >= 2
            {
                continue;
            }
            let segment = segments.get(&port.node);
            // The ports of one split net are one digital segment, and the
            // real ports driving one electrical node are one net: either way
            // its first port plans the one converter, and the rest join it.
            let members = match (segment, merged.get(&port.node)) {
                (Some(segment), _) => {
                    if segment.members.first() != Some(&(host_index, port_index)) {
                        continue;
                    }
                    segment.members.clone()
                }
                (None, Some(group)) if is_real_output(port.bit, port.direction) => {
                    if group.first() != Some(&(host_index, port_index)) {
                        continue;
                    }
                    group.clone()
                }
                _ => vec![(host_index, port_index)],
            };
            let node_label = xspice_auto_bridge_node_label(Some(&names), port.node);
            let kind = match segment {
                Some(segment) => segment.kind(),
                None => converter_kind(port.bit.is_some(), port.direction.into()),
            };
            let selected =
                rules.select_for_boundary_node(kind, &node_label, host.instance_name(), signal)?;
            if let Some(selected) = &selected {
                connect_modules::check_execution(
                    selected,
                    kind,
                    &node_label,
                    host.instance_name(),
                )?;
            }
            let scoped = metadata.get(&port.node);
            // A built-in converter binds live to the net the HDL port's
            // supplySensitivity names, above `.param vcc` and a numeric
            // `vsup`; a connect statement naming a net outranks it, and an
            // authored module's supply ports bind it themselves.
            let attributed = if port.bit.is_some()
                && selected.as_ref().is_none_or(|selected| {
                    selected.signature().is_some() && selected.supply_net().is_none()
                }) {
                connect_modules::supply_ports::sensitivity_net(
                    circuit,
                    &node_label,
                    host.instance_name(),
                    signal,
                )?
            } else {
                None
            };
            let (vcc, supply) = if port.bit.is_some() {
                if let Some(net) = attributed {
                    let level = supplies.rail_level(&net).unwrap_or_else(|| {
                        selected
                            .as_ref()
                            .and_then(connect_modules::PlannedConnectModule::stated_supply)
                            .unwrap_or_else(|| {
                                supplies
                                    .resolve(&node_label, scoped.and_then(|item| item.vcc))
                                    .level
                            })
                    });
                    (
                        level,
                        Some(boundary_supply::SupplyDerivation::Attribute {
                            attribute: "supplySensitivity".to_string(),
                            net,
                        }),
                    )
                } else if let Some(level) = selected
                    .as_ref()
                    .and_then(connect_modules::PlannedConnectModule::stated_supply)
                {
                    (
                        level,
                        Some(boundary_supply::SupplyDerivation::Declared {
                            parameter: "vsup".to_string(),
                        }),
                    )
                } else {
                    let resolved = supplies.resolve(&node_label, scoped.and_then(|item| item.vcc));
                    (resolved.level, Some(resolved.derivation))
                }
            } else {
                (0.0, None)
            };
            let mut selected = selected;
            if let Some(selected) = selected.as_mut() {
                connect_modules::bind_supply_ports(
                    circuit,
                    supplies,
                    selected,
                    &connect_modules::supply_ports::Boundary {
                        node_label: &node_label,
                        instance: host.instance_name(),
                        port: signal,
                        level: vcc,
                    },
                )?;
            }
            let suffix = port.bit.map(|bit| format!("_{bit}")).unwrap_or_default();
            // Generated cards pass through the SPICE parser, which folds node
            // names. Allocate the same spelling here to retain node identity.
            let event_name =
                format!("{}.{}__event{suffix}", host.instance_name(), signal).to_uppercase();
            pending.push((
                members,
                event_name,
                segment.is_some(),
                PlannedXspiceAutoBridge {
                    node: port.node,
                    event_node: None,
                    kind,
                    vcc,
                    supply,
                    family: scoped.and_then(|item| item.family.clone()),
                    port: Some((host.instance_name().to_string(), signal.to_string())),
                    connect_module: selected,
                },
            ));
        }
    }

    let mut bridges = Vec::with_capacity(pending.len());
    for (members, name, split, mut bridge) in pending {
        if circuit.get_node_by_name(&name).is_some() {
            return Err(SimulationError::Circuit(format!(
                "generated mixed event endpoint '{name}' conflicts with an existing circuit node"
            )));
        }
        let event_node = circuit.get_or_create_node(&name);
        for (host_index, port_index) in members {
            circuit.mixed_signal_hosts[host_index]
                .rebind_event_port(port_index, event_node)
                .map_err(|error| SimulationError::Circuit(error.to_string()))?;
        }
        // The code models' event ports on a split net join the module's
        // ports on the segment, so the converter is the net's only crossing.
        if split {
            circuit.rebind_xspice_event_node(bridge.node, event_node);
        }
        bridge.event_node = Some(event_node);
        bridges.push(bridge);
    }
    Ok(bridges)
}

/// Which way a digital segment's ports carry values, gathered over every port
/// on it: a module's ports and the code models' event ports alike.
#[derive(Debug, Default, Clone, Copy)]
struct SegmentTraffic {
    drives: bool,
    receives: bool,
    both: bool,
}

impl From<DigitalLinkDirection> for SegmentTraffic {
    fn from(direction: DigitalLinkDirection) -> Self {
        Self {
            drives: direction == DigitalLinkDirection::Output,
            receives: direction == DigitalLinkDirection::Input,
            both: direction == DigitalLinkDirection::Inout,
        }
    }
}

impl From<crate::xspice::PortDirection> for SegmentTraffic {
    fn from(direction: crate::xspice::PortDirection) -> Self {
        use crate::xspice::PortDirection;
        Self {
            drives: direction == PortDirection::Out,
            receives: direction == PortDirection::In,
            both: direction == PortDirection::InOut,
        }
    }
}

impl SegmentTraffic {
    fn join(&mut self, other: Self) {
        self.drives |= other.drives;
        self.receives |= other.receives;
        self.both |= other.both;
    }
}

/// The converter between a segment and its electrical node. A segment with a
/// port both ways converts both ways. One that drives meets the analog side
/// through a D/A: its receivers read the drivers on the segment itself, as
/// auto-insertion keeps digital receivers on the digital net. One that only
/// receives samples the node through an A/D.
fn converter_kind(logic: bool, traffic: SegmentTraffic) -> XspiceAutoBridgeKind {
    match (logic, traffic.both, traffic.drives) {
        (true, true, _) => XspiceAutoBridgeKind::Bidi,
        (true, false, true) => XspiceAutoBridgeKind::Dac,
        (true, false, false) => XspiceAutoBridgeKind::Adc,
        (false, true, _) => XspiceAutoBridgeKind::RealBidi,
        (false, false, true) => XspiceAutoBridgeKind::RealToV,
        (false, false, false) => XspiceAutoBridgeKind::VToReal,
    }
}

/// One split net: the module ports on it, `(host, port)` in instance order,
/// their event domain, which way every port on its segment carries values,
/// and whether the module ports drive it.
struct SplitSegment {
    members: Vec<(usize, usize)>,
    logic: bool,
    traffic: SegmentTraffic,
    module_drives: bool,
}

impl SplitSegment {
    /// The converter between the segment and its electrical node.
    fn kind(&self) -> XspiceAutoBridgeKind {
        converter_kind(self.logic, self.traffic)
    }
}

/// The nets a module's port shares with XSPICE event ports and an analog
/// element: each splits into a digital segment and its electrical node.
///
/// A net whose module ports and code-model ports carry unlike event domains,
/// four-state on one side and real-valued on the other, is refused: the net
/// holds no one value a converter could carry, and the deck declares no
/// conversion between the two.
///
/// So is a net a code model drives alongside a module's driver or an inout
/// port. Its converter reads one side's drive: a module port's design-level
/// strengths, or the code models' drivers. The two are resolved together only
/// on the event net itself, so a converter here would carry half the net.
fn split_segments(
    circuit: &CircuitData,
    physical: &BTreeSet<usize>,
    names: &[String],
) -> Result<BTreeMap<usize, SplitSegment>, SimulationError> {
    let mut segments: BTreeMap<usize, SplitSegment> = BTreeMap::new();
    for (host_index, host) in circuit.mixed_signal_hosts.iter().enumerate() {
        for (port_index, (port, signal)) in host.direct_event_ports().enumerate() {
            let net_kind = circuit.net_kinds.kind(port.node);
            if port.node == 0 || !physical.contains(&port.node) || !net_kind.is_discrete() {
                continue;
            }
            let logic = port.bit.is_some();
            let port_kind = if logic {
                crate::circuit::NetKind::Digital
            } else {
                crate::circuit::NetKind::Real
            };
            if net_kind != port_kind {
                let node_label = xspice_auto_bridge_node_label(Some(names), port.node);
                return Err(crate::ElaborationError::new(
                    crate::ElaborationErrorKind::PortDiscipline,
                    format!(
                        "connects its {} port '{signal}' to node '{node_label}', where XSPICE \
                         code models carry {} event-driven values and analog elements read the \
                         node's voltage. The net splits into an event segment and its \
                         electrical node only when every event port on it shares one domain; \
                         these are unlike event domains, and the deck declares no conversion \
                         between them",
                        port_kind.description(),
                        net_kind.description(),
                    ),
                )
                .instance(host.instance_name())
                .into());
            }
            let segment = segments.entry(port.node).or_insert_with(|| SplitSegment {
                members: Vec::new(),
                logic,
                traffic: SegmentTraffic::default(),
                module_drives: false,
            });
            segment.members.push((host_index, port_index));
            segment.traffic.join(port.direction.into());
            segment.module_drives |= port.direction != DigitalLinkDirection::Input;
        }
    }
    if segments.is_empty() {
        return Ok(segments);
    }
    let mut model_drivers: BTreeMap<usize, &str> = BTreeMap::new();
    for instance in &circuit.xspice_instances {
        for (port_index, port) in instance.ports().iter().enumerate() {
            let Some(connection) = instance.connection_at(port_index) else {
                continue;
            };
            for_each_event_connection_node(connection, |node| {
                if let Some(segment) = segments.get_mut(&node) {
                    segment.traffic.join(port.direction.into());
                    if port.direction != crate::xspice::PortDirection::In {
                        model_drivers.entry(node).or_insert(instance.name.as_str());
                    }
                }
            });
        }
    }
    for (node, model) in model_drivers {
        let segment = &segments[&node];
        if !segment.module_drives && !segment.traffic.both {
            continue;
        }
        let (host_index, port_index) = segment.members[0];
        let host = &circuit.mixed_signal_hosts[host_index];
        let signal = host
            .direct_event_ports()
            .nth(port_index)
            .map(|(_, signal)| signal)
            .unwrap_or_default();
        let node_label = xspice_auto_bridge_node_label(Some(names), node);
        return Err(crate::ElaborationError::new(
            crate::ElaborationErrorKind::PortDiscipline,
            format!(
                "connects its port '{signal}' to node '{node_label}', which code model \
                 '{model}' also drives and analog elements read. A net that a code model \
                 drives alongside a module's driver or an inout port is resolved only on its \
                 event segment, and the converter to its electrical node would read one \
                 side's drive alone; such a net needs an explicit converter on the code \
                 model's side"
            ),
        )
        .instance(host.instance_name())
        .into());
    }
    Ok(segments)
}

/// Every event node one XSPICE connection reaches.
fn for_each_event_connection_node(
    connection: &crate::xspice::PortConnection,
    mut visit: impl FnMut(usize),
) {
    use crate::xspice::PortConnection;
    match connection {
        PortConnection::Digital(node)
        | PortConnection::DigitalInverted(node)
        | PortConnection::Real(node) => visit(*node),
        PortConnection::DigitalVector(nodes) | PortConnection::RealVector(nodes) => {
            nodes.iter().copied().for_each(visit);
        }
        PortConnection::DigitalVectorMapped(nodes) => {
            for connection in nodes {
                visit(connection.node);
            }
        }
        _ => {}
    }
}

/// A real-valued `output` port. A real `inout` converts on its own, because
/// its receivers read the node through its own converter.
fn is_real_output(bit: Option<u32>, direction: DigitalLinkDirection) -> bool {
    bit.is_none() && direction == DigitalLinkDirection::Output
}

/// The real-valued ports that drive each electrical node, where more than one
/// does: `(host, port)` in instance order.
///
/// Verilog-AMS LRM 2.4 section 7.8.3 merges the discrete ports on one node
/// into a single connection. Real ports that drive one node therefore join one
/// real net, which their declared resolution combines, and one converter
/// carries the resolved value onto the node. A converter per port would put
/// several ideal sources in parallel on it. A net that two plain `wreal`
/// ports drive, or that ports of different resolutions join, is refused here,
/// naming both instances and the node.
fn merged_real_drivers(
    circuit: &CircuitData,
    physical: &BTreeSet<usize>,
) -> Result<BTreeMap<usize, Vec<(usize, usize)>>, SimulationError> {
    let mut groups: BTreeMap<usize, Vec<(usize, usize)>> = BTreeMap::new();
    let mut ports = Vec::new();
    for (host_index, host) in circuit.mixed_signal_hosts.iter().enumerate() {
        for (port_index, (port, signal)) in host.direct_event_ports().enumerate() {
            if port.node == 0
                || !physical.contains(&port.node)
                || !is_real_output(port.bit, port.direction)
            {
                continue;
            }
            let Some((resolution, drives)) = host.real_output_net(port) else {
                continue;
            };
            groups
                .entry(port.node)
                .or_default()
                .push((host_index, port_index));
            ports.push((port.node, host.instance_name(), signal, resolution, drives));
        }
    }
    groups.retain(|_, group| group.len() > 1);
    circuit.refuse_unresolvable_real_nets(
        ports
            .into_iter()
            .filter(|(node, ..)| groups.contains_key(node)),
    )?;
    Ok(groups)
}

/// Construction resolves template parameters and expands subcircuits. Bind
/// their declared thresholds only now, using the model's connected input values
/// at runtime so differential/current inputs and internal nodes share one law.
pub(super) fn bind_converter_thresholds(circuit: &mut CircuitData) -> Result<(), SimulationError> {
    if circuit.mixed_signal_hosts.is_empty() {
        return Ok(());
    }
    let mut owners = BTreeMap::new();
    for (host_index, host) in circuit.mixed_signal_hosts.iter().enumerate() {
        for (port, _) in host.direct_event_ports() {
            owners.entry(port.node).or_insert(host_index);
        }
    }
    for instance_index in 0..circuit.xspice_instances.len() {
        let instance = &circuit.xspice_instances[instance_index];
        let thresholds = instance
            .analog_input_thresholds()
            .map_err(|error| SimulationError::Circuit(error.to_string()))?;
        if thresholds.is_empty() {
            continue;
        }
        let mut output_owners = BTreeMap::new();
        instance.for_each_digital_output_driver(|output| {
            if let Some(owner) = owners.get(&output.node_id) {
                output_owners.insert(output.driver_index, *owner);
            }
        });
        // An internal comparator may feed gates inside a template. Its root
        // still constrains the whole circuit; the host is only its ledger owner.
        let fallback = output_owners.values().next().copied().unwrap_or(0);
        let name = instance.name.clone();
        let decides_by_located = instance.decides_by_located_crossing();
        for threshold in &thresholds {
            let host = output_owners
                .get(&threshold.element)
                .copied()
                .unwrap_or(fallback);
            circuit.mixed_signal_hosts[host]
                .add_converter_input_root(instance_index, &name, threshold, decides_by_located)
                .map_err(|error| SimulationError::Circuit(error.to_string()))?;
        }
        circuit.xspice_instances[instance_index]
            .make_mut()
            .bind_mixed_input_thresholds();
    }
    Ok(())
}
