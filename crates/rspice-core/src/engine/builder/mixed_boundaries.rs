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
    let mut pending = Vec::new();
    for (host_index, host) in circuit.mixed_signal_hosts.iter().enumerate() {
        for (port_index, (port, signal)) in host.direct_event_ports().enumerate() {
            if port.node > 0
                && !physical.contains(&port.node)
                && endpoints.get(&port.node).copied().unwrap_or(0) >= 2
            {
                continue;
            }
            let node_label = xspice_auto_bridge_node_label(Some(&names), port.node);
            let kind = match (port.bit, port.direction) {
                (Some(_), DigitalLinkDirection::Input) => XspiceAutoBridgeKind::Adc,
                (Some(_), DigitalLinkDirection::Output) => XspiceAutoBridgeKind::Dac,
                (Some(_), DigitalLinkDirection::Inout) => XspiceAutoBridgeKind::Bidi,
                (None, DigitalLinkDirection::Input) => XspiceAutoBridgeKind::VToReal,
                (None, DigitalLinkDirection::Output) => XspiceAutoBridgeKind::RealToV,
                _ => {
                    return Err(SimulationError::Circuit(format!(
                        "mixed instance '{}' port '{signal}' on node '{node_label}' requires an explicit bidirectional real electrical conversion",
                        host.instance_name()
                    )));
                }
            };
            let selected =
                rules.select_for_boundary_node(kind, &node_label, host.instance_name(), signal)?;
            if let Some(selected) = &selected {
                connect_modules::check_delegable(selected, kind, &node_label)?;
            }
            let scoped = metadata.get(&port.node);
            let (vcc, supply) = if port.bit.is_some() {
                if let Some(level) = selected
                    .as_ref()
                    .and_then(connect_modules::PlannedConnectModule::stated_supply)
                {
                    (level, Some(boundary_supply::SupplyDerivation::Declared))
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
                host_index,
                port_index,
                event_name,
                (signal.to_string(), port.bit),
                PlannedXspiceAutoBridge {
                    node: port.node,
                    event_node: None,
                    kind,
                    vcc,
                    supply,
                    family: scoped.and_then(|item| item.family.clone()),
                    connect_module: selected,
                },
            ));
        }
    }

    let mut bridges = Vec::with_capacity(pending.len());
    for (host_index, port_index, name, (signal, bit), mut bridge) in pending {
        if circuit.get_node_by_name(&name).is_some() {
            return Err(SimulationError::Circuit(format!(
                "generated mixed event endpoint '{name}' conflicts with an existing circuit node"
            )));
        }
        if bridge.kind == XspiceAutoBridgeKind::Adc {
            // This detector never drives HDL. The common converter publishes
            // its decision (and any delayed edge); the host only localizes the
            // physical threshold before accepting the analog step.
            circuit.mixed_signal_hosts[host_index]
                .add_adc_root(
                    &signal,
                    bit.expect("logic input"),
                    (bridge.node, 0),
                    bridge.vcc / 2.0,
                    bridge.vcc / 2.0,
                )
                .map_err(|error| SimulationError::Circuit(error.to_string()))?;
        }
        let event_node = circuit.get_or_create_node(&name);
        circuit.mixed_signal_hosts[host_index]
            .rebind_event_port(port_index, event_node)
            .map_err(|error| SimulationError::Circuit(error.to_string()))?;
        bridge.event_node = Some(event_node);
        bridges.push(bridge);
    }
    Ok(bridges)
}
