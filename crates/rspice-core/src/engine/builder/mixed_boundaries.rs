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
                (None, DigitalLinkDirection::Inout) => XspiceAutoBridgeKind::RealBidi,
            };
            let selected =
                rules.select_for_boundary_node(kind, &node_label, host.instance_name(), signal)?;
            if let Some(selected) = &selected {
                connect_modules::check_execution(selected, kind, &node_label)?;
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
    for (host_index, port_index, name, mut bridge) in pending {
        if circuit.get_node_by_name(&name).is_some() {
            return Err(SimulationError::Circuit(format!(
                "generated mixed event endpoint '{name}' conflicts with an existing circuit node"
            )));
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

/// Construction resolves template parameters and expands subcircuits. Bind
/// their declared thresholds only now, using the model's connected input values
/// at runtime so differential/current inputs and internal nodes share one law.
pub(super) fn bind_converter_thresholds(
    circuit: &mut CircuitData,
    first_instance: usize,
) -> Result<(), SimulationError> {
    let mut owners = BTreeMap::new();
    for (host_index, host) in circuit.mixed_signal_hosts.iter().enumerate() {
        for (port, signal) in host.direct_event_ports() {
            if let Some(bit) = port.bit {
                owners
                    .entry(port.node)
                    .or_insert((host_index, signal.to_string(), bit));
            }
        }
    }
    for instance_index in first_instance..circuit.xspice_instances.len() {
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
                output_owners.insert(output.driver_index, owner.clone());
            }
        });
        // An internal comparator may feed gates inside a template. Its root
        // still constrains the whole circuit; the host is only its ledger owner.
        let fallback = output_owners
            .values()
            .next()
            .or_else(|| owners.values().next())
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "converter '{}' declares analog thresholds without a mixed logic host",
                    instance.name
                ))
            })?;
        let name = instance.name.clone();
        for threshold in &thresholds {
            let (host, signal, bit) = output_owners.get(&threshold.element).unwrap_or(fallback);
            circuit.mixed_signal_hosts[*host]
                .add_converter_input_root(signal, *bit, instance_index, &name, threshold)
                .map_err(|error| SimulationError::Circuit(error.to_string()))?;
        }
        circuit.xspice_instances[instance_index]
            .make_mut()
            .bind_mixed_input_thresholds();
    }
    Ok(())
}
