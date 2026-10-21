//! Electrical conversion for typed mixed ports, after the whole deck is wired.
//! Each physical port gets its own event endpoint: an input samples the solved
//! voltage, including a driver's ramp and loading, rather than its target value.

use super::*;
use rspice_veriloga::canonical_ir::digital_link::DigitalLinkDirection;

pub(super) fn plan_conversions(
    circuit: &mut CircuitData,
    flat_elements: &[Element],
    metadata: &BTreeMap<usize, XspiceAutoBridgeNodeMetadata>,
    supplies: &boundary_supply::BoundarySupplies,
    rules: &connect_modules::DesignConnectRules,
) -> Result<Vec<PlannedXspiceAutoBridge>, SimulationError> {
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
            // The real ports driving one electrical node are one net: its
            // first port plans the one converter, and the rest join it.
            let members = match merged.get(&port.node) {
                Some(group) if is_real_output(port.bit, port.direction) => {
                    if group.first() != Some(&(host_index, port_index)) {
                        continue;
                    }
                    group.clone()
                }
                _ => vec![(host_index, port_index)],
            };
            let node_label = xspice_auto_bridge_node_label(Some(&names), port.node);
            // A converter's electrical side would land on a net that already
            // carries XSPICE event values, which is two domains on one node
            // with no contract between them. Refused by the instance and port
            // that would have brought the second domain.
            let net_kind = circuit.net_kinds.kind(port.node);
            if port.node > 0 && net_kind.is_discrete() {
                return Err(crate::ElaborationError::new(
                    crate::ElaborationErrorKind::PortDiscipline,
                    format!(
                        "connects its discrete port '{signal}' to node '{node_label}', which \
                         carries {} event-driven XSPICE values and continuous loading. This \
                         connection requires an explicit shared conversion contract for \
                         continuous loading or unlike event domains; direct shared resolution \
                         currently requires a digital-only net",
                        net_kind.description(),
                    ),
                )
                .instance(host.instance_name())
                .into());
            }
            let kind = match (port.bit, port.direction) {
                (Some(_), DigitalLinkDirection::Input) => XspiceAutoBridgeKind::Adc,
                (Some(_), DigitalLinkDirection::Output) => XspiceAutoBridgeKind::Dac,
                (Some(_), DigitalLinkDirection::Inout) => XspiceAutoBridgeKind::Bidi,
                (None, DigitalLinkDirection::Input) => XspiceAutoBridgeKind::VToReal,
                (None, DigitalLinkDirection::Output) => XspiceAutoBridgeKind::RealToV,
                (None, DigitalLinkDirection::Inout) => XspiceAutoBridgeKind::RealBidi,
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
            let (vcc, supply) = if port.bit.is_some() {
                if let Some(level) = selected
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
            let suffix = port.bit.map(|bit| format!("_{bit}")).unwrap_or_default();
            // Generated cards pass through the SPICE parser, which folds node
            // names. Allocate the same spelling here to retain node identity.
            let event_name =
                format!("{}.{}__event{suffix}", host.instance_name(), signal).to_uppercase();
            pending.push((
                members,
                event_name,
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
    for (members, name, mut bridge) in pending {
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
        bridge.event_node = Some(event_node);
        bridges.push(bridge);
    }
    Ok(bridges)
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
        for threshold in &thresholds {
            let host = output_owners
                .get(&threshold.element)
                .copied()
                .unwrap_or(fallback);
            circuit.mixed_signal_hosts[host]
                .add_converter_input_root(instance_index, &name, threshold)
                .map_err(|error| SimulationError::Circuit(error.to_string()))?;
        }
        circuit.xspice_instances[instance_index]
            .make_mut()
            .bind_mixed_input_thresholds();
    }
    Ok(())
}
