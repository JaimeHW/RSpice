use super::*;

pub(in crate::engine::builder) fn resolve_xspice_node(
    circuit: &mut CircuitData,
    name: &str,
) -> usize {
    if name.eq_ignore_ascii_case("0") {
        0
    } else {
        circuit.get_or_create_node(name)
    }
}

#[cfg(test)]
pub(in crate::engine::builder) fn coerce_xspice_connection(
    circuit: &mut CircuitData,
    port_spec: &crate::xspice::PortSpec,
    parsed_port: &crate::netlist::XspicePort,
) -> Result<crate::xspice::PortConnection, SimulationError> {
    coerce_xspice_connection_named(circuit, port_spec, parsed_port, "Axspice#port")
}

fn coerce_xspice_connection_named(
    circuit: &mut CircuitData,
    port_spec: &crate::xspice::PortSpec,
    parsed_port: &crate::netlist::XspicePort,
    branch_name: &str,
) -> Result<crate::xspice::PortConnection, SimulationError> {
    use crate::netlist::XspicePort;
    use crate::xspice::{DigitalPortConnection, PortConnection};

    if let Some(explicit_type) = explicit_xspice_port_type(parsed_port)
        && !port_spec_allows_type(port_spec, explicit_type)
    {
        return Err(SimulationError::Circuit(format!(
            "XSPICE port '{}' does not allow explicit {:?} connection {:?}",
            port_spec.name, explicit_type, parsed_port
        )));
    }

    let expects_event = port_spec.default_type.is_event_driven();
    let expects_real = port_spec.default_type == crate::xspice::PortType::Real;

    let make_hybrid = |circuit: &mut CircuitData, pos: usize, neg: usize| {
        let branch_ordinal = circuit.allocate_branch_named(branch_name);
        PortConnection::Hybrid {
            pos,
            neg,
            branch_ordinal,
        }
    };

    let convert_scalar = |circuit: &mut CircuitData, node_name: &str| {
        let node = resolve_xspice_node(circuit, node_name);
        if port_spec.is_vector {
            if expects_real {
                PortConnection::RealVector(vec![node])
            } else if expects_event {
                PortConnection::DigitalVector(vec![node])
            } else {
                PortConnection::AnalogVector(vec![node])
            }
        } else if port_spec.default_type == crate::xspice::PortType::Hybrid {
            make_hybrid(circuit, node, 0)
        } else if expects_real {
            PortConnection::Real(node)
        } else if expects_event {
            PortConnection::Digital(node)
        } else {
            PortConnection::Analog(node)
        }
    };

    let convert_vector = |circuit: &mut CircuitData, node_names: &[String]| {
        let nodes: Vec<usize> = node_names
            .iter()
            .map(|name| resolve_xspice_node(circuit, name))
            .collect();

        if port_spec.is_vector {
            if expects_real {
                Ok(PortConnection::RealVector(nodes))
            } else if expects_event {
                Ok(PortConnection::DigitalVector(nodes))
            } else {
                Ok(PortConnection::AnalogVector(nodes))
            }
        } else if nodes.len() == 1 {
            if expects_real {
                Ok(PortConnection::Real(nodes[0]))
            } else if expects_event {
                Ok(PortConnection::Digital(nodes[0]))
            } else {
                Ok(PortConnection::Analog(nodes[0]))
            }
        } else {
            Err(SimulationError::Circuit(format!(
                "XSPICE port '{}' expects a scalar connection but got {} nodes",
                port_spec.name,
                nodes.len()
            )))
        }
    };

    let convert_digital_inverted = |circuit: &mut CircuitData,
                                    node_name: &str|
     -> Result<crate::xspice::PortConnection, SimulationError> {
        if !expects_event {
            return Err(SimulationError::Circuit(format!(
                "XSPICE inverted digital port '{}' requires an event-driven model port",
                port_spec.name
            )));
        }

        let node = resolve_xspice_node(circuit, node_name);
        Ok(if port_spec.is_vector {
            PortConnection::DigitalVectorMapped(vec![DigitalPortConnection::new(node, true)])
        } else {
            PortConnection::DigitalInverted(node)
        })
    };

    let convert_mixed_digital_vector =
        |circuit: &mut CircuitData,
         node_refs: &[crate::netlist::XspiceDigitalNode]|
         -> Result<crate::xspice::PortConnection, SimulationError> {
            if !expects_event {
                return Err(SimulationError::Circuit(format!(
                    "XSPICE inverted digital vector port '{}' requires an event-driven model port",
                    port_spec.name
                )));
            }

            let nodes: Vec<DigitalPortConnection> = node_refs
                .iter()
                .map(|node_ref| {
                    DigitalPortConnection::new(
                        resolve_xspice_node(circuit, &node_ref.name),
                        node_ref.inverted,
                    )
                })
                .collect();

            if port_spec.is_vector {
                Ok(PortConnection::DigitalVectorMapped(nodes))
            } else if nodes.len() == 1 {
                let node = nodes[0];
                Ok(if node.inverted {
                    PortConnection::DigitalInverted(node.node)
                } else {
                    PortConnection::Digital(node.node)
                })
            } else {
                Err(SimulationError::Circuit(format!(
                    "XSPICE port '{}' expects a scalar connection but got {} nodes",
                    port_spec.name,
                    nodes.len()
                )))
            }
        };

    match parsed_port {
        XspicePort::Digital(name) if expects_real => Err(SimulationError::Circuit(format!(
            "XSPICE real port '{}' must use a bare real node name, got bracketed digital node '{}'",
            port_spec.name, name
        ))),
        XspicePort::DigitalInverted(name) => convert_digital_inverted(circuit, name),
        XspicePort::Analog(name)
        | XspicePort::Digital(name)
        | XspicePort::ExplicitDigital(name)
        | XspicePort::Conductance(name) => Ok(convert_scalar(circuit, name)),
        XspicePort::Current(name) => {
            if port_spec.direction == crate::xspice::PortDirection::Out {
                Ok(PortConnection::CurrentOutput {
                    pos: resolve_xspice_node(circuit, name),
                    neg: 0,
                })
            } else {
                let pos = resolve_xspice_node(circuit, name);
                let branch_ordinal = circuit.allocate_branch_named(branch_name);
                Ok(PortConnection::CurrentProbe {
                    pos,
                    neg: 0,
                    branch_ordinal,
                })
            }
        }
        XspicePort::VoltageName(source_name) => Ok(PortConnection::NamedBranchCurrent {
            source_name: source_name.clone(),
            branch_ordinal: None,
        }),
        XspicePort::AnalogVector(names) | XspicePort::DigitalVector(names) => {
            convert_vector(circuit, names)
        }
        XspicePort::DigitalVectorMixed(nodes) => convert_mixed_digital_vector(circuit, nodes),
        XspicePort::DifferentialVoltage { pos, neg }
        | XspicePort::DifferentialConductance { pos, neg } => {
            let pos_node = resolve_xspice_node(circuit, pos);
            let neg_node = resolve_xspice_node(circuit, neg);
            Ok(PortConnection::Differential(pos_node, neg_node))
        }
        XspicePort::Hybrid(name) => {
            let node = resolve_xspice_node(circuit, name);
            Ok(make_hybrid(circuit, node, 0))
        }
        XspicePort::DifferentialHybrid { pos, neg } => {
            let pos_node = resolve_xspice_node(circuit, pos);
            let neg_node = resolve_xspice_node(circuit, neg);
            Ok(make_hybrid(circuit, pos_node, neg_node))
        }
        XspicePort::DifferentialCurrent { pos, neg } => {
            let pos_node = resolve_xspice_node(circuit, pos);
            let neg_node = resolve_xspice_node(circuit, neg);
            if port_spec.direction == crate::xspice::PortDirection::Out {
                Ok(PortConnection::CurrentOutput {
                    pos: pos_node,
                    neg: neg_node,
                })
            } else {
                let branch_ordinal = circuit.allocate_branch_named(branch_name);
                Ok(PortConnection::CurrentProbe {
                    pos: pos_node,
                    neg: neg_node,
                    branch_ordinal,
                })
            }
        }
        XspicePort::Null => Ok(PortConnection::Null),
    }
}

pub(in crate::engine::builder) fn coerce_xspice_connections(
    circuit: &mut CircuitData,
    port_specs: &[crate::xspice::PortSpec],
    parsed_ports: &[crate::netlist::XspicePort],
    element_name: &str,
    model_name: &str,
) -> Result<Vec<crate::xspice::PortConnection>, SimulationError> {
    let mut connections = Vec::with_capacity(port_specs.len());
    let mut cursor = 0usize;

    for (spec_idx, port_spec) in port_specs.iter().enumerate() {
        if cursor >= parsed_ports.len() {
            return Err(SimulationError::Circuit(format!(
                "XSPICE element '{}' provides too few connections ({}) for model '{}' ports ({})",
                element_name,
                parsed_ports.len(),
                model_name,
                port_specs.len()
            )));
        }

        if port_spec.null_allowed
            && matches!(parsed_ports[cursor], crate::netlist::XspicePort::Null)
        {
            connections.push(crate::xspice::PortConnection::Null);
            cursor += 1;
            continue;
        }

        if port_spec.is_vector && !parsed_ports[cursor].is_vector_connection() {
            let remaining_specs = port_specs.len() - spec_idx - 1;
            let remaining_ports = parsed_ports.len() - cursor;
            if remaining_ports <= remaining_specs {
                return Err(SimulationError::Circuit(format!(
                    "XSPICE element '{}' vector port '{}' has no connections",
                    element_name, port_spec.name
                )));
            }
            let take = remaining_ports - remaining_specs;
            connections.push(coerce_xspice_vector_connection(
                circuit,
                port_spec,
                &parsed_ports[cursor..cursor + take],
                element_name,
            )?);
            cursor += take;
        } else {
            let remaining_specs = port_specs.len() - spec_idx - 1;
            let remaining_ports = parsed_ports.len() - cursor;
            if remaining_ports > remaining_specs + 1
                && let Some(packed) =
                    try_pack_default_differential_port(port_spec, &parsed_ports[cursor..])
            {
                let branch_name = format!("{element_name}#{}#sense", port_spec.name);
                connections.push(coerce_xspice_connection_named(
                    circuit,
                    port_spec,
                    &packed,
                    &branch_name,
                )?);
                cursor += 2;
            } else {
                let branch_name = format!("{element_name}#{}#sense", port_spec.name);
                connections.push(coerce_xspice_connection_named(
                    circuit,
                    port_spec,
                    &parsed_ports[cursor],
                    &branch_name,
                )?);
                cursor += 1;
            }
        }
    }

    if cursor != parsed_ports.len() {
        return Err(SimulationError::Circuit(format!(
            "XSPICE element '{}' provides more connections ({}) than model '{}' ports ({})",
            element_name,
            parsed_ports.len(),
            model_name,
            port_specs.len()
        )));
    }

    Ok(connections)
}

fn explicit_xspice_port_type(
    parsed_port: &crate::netlist::XspicePort,
) -> Option<crate::xspice::PortType> {
    use crate::netlist::XspicePort;
    use crate::xspice::PortType;

    match parsed_port {
        XspicePort::Conductance(_) => Some(PortType::Conductance),
        XspicePort::ExplicitDigital(_) => Some(PortType::Digital),
        XspicePort::Current(_) => Some(PortType::Current),
        XspicePort::VoltageName(_) => Some(PortType::VoltageName),
        XspicePort::DifferentialVoltage { .. } => Some(PortType::DifferentialVoltage),
        XspicePort::DifferentialCurrent { .. } => Some(PortType::DifferentialCurrent),
        XspicePort::DifferentialConductance { .. } => Some(PortType::DifferentialConductance),
        XspicePort::Hybrid(_) => Some(PortType::Hybrid),
        XspicePort::DifferentialHybrid { .. } => Some(PortType::DifferentialHybrid),
        _ => None,
    }
}

fn port_spec_allows_type(
    port_spec: &crate::xspice::PortSpec,
    port_type: crate::xspice::PortType,
) -> bool {
    port_spec.allowed_types.is_empty() && port_spec.default_type == port_type
        || port_spec.allowed_types.contains(&port_type)
}

fn pack_scalar_vector_port(
    port_spec: &crate::xspice::PortSpec,
    parsed_ports: &[crate::netlist::XspicePort],
) -> Result<crate::netlist::XspicePort, SimulationError> {
    use crate::netlist::XspicePort;

    let expects_event = port_spec.default_type.is_event_driven();
    let expects_real = port_spec.default_type == crate::xspice::PortType::Real;
    let mut nodes = Vec::with_capacity(parsed_ports.len());
    for port in parsed_ports {
        match port {
            XspicePort::Analog(name)
            | XspicePort::Digital(name)
            | XspicePort::ExplicitDigital(name)
            | XspicePort::Conductance(name)
            | XspicePort::Hybrid(name) => {
                nodes.push(crate::netlist::XspiceDigitalNode::new(name.clone(), false))
            }
            XspicePort::DigitalInverted(name) => {
                nodes.push(crate::netlist::XspiceDigitalNode::new(name.clone(), true))
            }
            XspicePort::AnalogVector(names) | XspicePort::DigitalVector(names)
                if parsed_ports.len() == 1 =>
            {
                return Ok(if expects_real {
                    XspicePort::AnalogVector(names.clone())
                } else if expects_event {
                    XspicePort::DigitalVector(names.clone())
                } else {
                    XspicePort::AnalogVector(names.clone())
                });
            }
            XspicePort::DigitalVectorMixed(nodes) if parsed_ports.len() == 1 => {
                return Ok(XspicePort::DigitalVectorMixed(nodes.clone()));
            }
            other => {
                return Err(SimulationError::Circuit(format!(
                    "XSPICE vector port '{}' cannot group connection {:?}",
                    port_spec.name, other
                )));
            }
        }
    }

    Ok(if expects_real {
        XspicePort::AnalogVector(nodes.into_iter().map(|node| node.name).collect())
    } else if expects_event {
        if nodes.iter().any(|node| node.inverted) {
            XspicePort::DigitalVectorMixed(nodes)
        } else {
            XspicePort::DigitalVector(nodes.into_iter().map(|node| node.name).collect())
        }
    } else {
        XspicePort::AnalogVector(nodes.into_iter().map(|node| node.name).collect())
    })
}

fn coerce_xspice_vector_connection(
    circuit: &mut CircuitData,
    port_spec: &crate::xspice::PortSpec,
    parsed_ports: &[crate::netlist::XspicePort],
    element_name: &str,
) -> Result<crate::xspice::PortConnection, SimulationError> {
    use crate::netlist::XspicePort;
    use crate::xspice::{AnalogInputConnection, PortConnection, PortDirection};

    let expects_event = port_spec.default_type.is_event_driven();
    if expects_event {
        let packed = pack_scalar_vector_port(port_spec, parsed_ports)?;
        return coerce_xspice_connection_named(
            circuit,
            port_spec,
            &packed,
            &format!("{element_name}#{}#sense", port_spec.name),
        );
    }

    if parsed_ports.len() == 1
        && let XspicePort::AnalogVector(names) = &parsed_ports[0]
    {
        return Ok(PortConnection::AnalogVector(
            names
                .iter()
                .map(|name| resolve_xspice_node(circuit, name))
                .collect(),
        ));
    }

    let mut simple_nodes = Vec::with_capacity(parsed_ports.len());
    let mut typed_elements = Vec::with_capacity(parsed_ports.len());
    let mut all_simple_nodes = true;

    for (index, port) in parsed_ports.iter().enumerate() {
        if let Some(explicit_type) = explicit_xspice_port_type(port)
            && !port_spec_allows_type(port_spec, explicit_type)
        {
            return Err(SimulationError::Circuit(format!(
                "XSPICE port '{}' does not allow explicit {:?} connection {:?}",
                port_spec.name, explicit_type, port
            )));
        }

        let element = match port {
            XspicePort::DigitalInverted(_) | XspicePort::DigitalVectorMixed(_) => {
                return Err(SimulationError::Circuit(format!(
                    "XSPICE analog vector port '{}' cannot group inverted digital connection {:?}",
                    port_spec.name, port
                )));
            }
            XspicePort::Analog(name)
            | XspicePort::Digital(name)
            | XspicePort::ExplicitDigital(name)
            | XspicePort::Conductance(name) => {
                let node = resolve_xspice_node(circuit, name);
                simple_nodes.push(node);
                AnalogInputConnection::Node(node)
            }
            XspicePort::Current(name) => {
                all_simple_nodes = false;
                if port_spec.direction == PortDirection::Out {
                    AnalogInputConnection::CurrentOutput {
                        pos: resolve_xspice_node(circuit, name),
                        neg: 0,
                    }
                } else {
                    let pos = resolve_xspice_node(circuit, name);
                    let branch_ordinal = circuit.allocate_branch_named(&format!(
                        "{element_name}#{}[{index}]#sense",
                        port_spec.name
                    ));
                    AnalogInputConnection::CurrentProbe {
                        pos,
                        neg: 0,
                        branch_ordinal,
                    }
                }
            }
            XspicePort::DifferentialVoltage { pos, neg }
            | XspicePort::DifferentialConductance { pos, neg } => {
                all_simple_nodes = false;
                AnalogInputConnection::Differential(
                    resolve_xspice_node(circuit, pos),
                    resolve_xspice_node(circuit, neg),
                )
            }
            XspicePort::Hybrid(name) => {
                all_simple_nodes = false;
                let node = resolve_xspice_node(circuit, name);
                let branch_ordinal = circuit.allocate_branch_named(&format!(
                    "{element_name}#{}[{index}]#hybrid",
                    port_spec.name
                ));
                AnalogInputConnection::Hybrid {
                    pos: node,
                    neg: 0,
                    branch_ordinal,
                }
            }
            XspicePort::DifferentialHybrid { pos, neg } => {
                all_simple_nodes = false;
                let pos_node = resolve_xspice_node(circuit, pos);
                let neg_node = resolve_xspice_node(circuit, neg);
                let branch_ordinal = circuit.allocate_branch_named(&format!(
                    "{element_name}#{}[{index}]#hybrid",
                    port_spec.name
                ));
                AnalogInputConnection::Hybrid {
                    pos: pos_node,
                    neg: neg_node,
                    branch_ordinal,
                }
            }
            XspicePort::DifferentialCurrent { pos, neg } => {
                all_simple_nodes = false;
                let pos_node = resolve_xspice_node(circuit, pos);
                let neg_node = resolve_xspice_node(circuit, neg);
                if port_spec.direction == PortDirection::Out {
                    AnalogInputConnection::CurrentOutput {
                        pos: pos_node,
                        neg: neg_node,
                    }
                } else {
                    let branch_ordinal = circuit.allocate_branch_named(&format!(
                        "{element_name}#{}[{index}]#sense",
                        port_spec.name
                    ));
                    AnalogInputConnection::CurrentProbe {
                        pos: pos_node,
                        neg: neg_node,
                        branch_ordinal,
                    }
                }
            }
            XspicePort::VoltageName(source_name) => {
                all_simple_nodes = false;
                AnalogInputConnection::NamedBranchCurrent {
                    source_name: source_name.clone(),
                    branch_ordinal: None,
                }
            }
            XspicePort::AnalogVector(names) | XspicePort::DigitalVector(names)
                if parsed_ports.len() == 1 =>
            {
                return Ok(PortConnection::AnalogVector(
                    names
                        .iter()
                        .map(|name| resolve_xspice_node(circuit, name))
                        .collect(),
                ));
            }
            XspicePort::DigitalVector(_) => {
                return Err(SimulationError::Circuit(format!(
                    "XSPICE analog vector port '{}' cannot group digital connection {:?}",
                    port_spec.name, port
                )));
            }
            XspicePort::AnalogVector(_) => {
                return Err(SimulationError::Circuit(format!(
                    "XSPICE analog vector port '{}' cannot mix nested vector connection {:?}",
                    port_spec.name, port
                )));
            }
            XspicePort::Null => {
                return Err(SimulationError::Circuit(format!(
                    "XSPICE analog vector port '{}' cannot group null connection",
                    port_spec.name
                )));
            }
        };
        typed_elements.push(element);
    }

    if all_simple_nodes {
        Ok(PortConnection::AnalogVector(simple_nodes))
    } else {
        Ok(PortConnection::TypedAnalogVector(typed_elements))
    }
}

fn try_pack_default_differential_port(
    port_spec: &crate::xspice::PortSpec,
    parsed_ports: &[crate::netlist::XspicePort],
) -> Option<crate::netlist::XspicePort> {
    use crate::netlist::XspicePort;
    use crate::xspice::PortType;

    if port_spec.is_vector || parsed_ports.len() < 2 {
        return None;
    }

    let (pos, neg) = match (&parsed_ports[0], &parsed_ports[1]) {
        (XspicePort::Analog(pos), XspicePort::Analog(neg)) => (pos.clone(), neg.clone()),
        _ => return None,
    };

    match port_spec.default_type {
        PortType::DifferentialVoltage => Some(XspicePort::DifferentialVoltage { pos, neg }),
        PortType::DifferentialConductance => Some(XspicePort::DifferentialConductance { pos, neg }),
        PortType::DifferentialHybrid => Some(XspicePort::DifferentialHybrid { pos, neg }),
        _ => None,
    }
}

trait XspicePortExt {
    fn is_vector_connection(&self) -> bool;
}

impl XspicePortExt for crate::netlist::XspicePort {
    fn is_vector_connection(&self) -> bool {
        matches!(
            self,
            crate::netlist::XspicePort::AnalogVector(_)
                | crate::netlist::XspicePort::DigitalVector(_)
                | crate::netlist::XspicePort::DigitalVectorMixed(_)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::netlist::{XspiceDigitalNode, XspicePort};
    use crate::xspice::{AnalogInputConnection, PortConnection, PortDirection, PortSpec, PortType};

    fn gd_port(name: &str) -> PortSpec {
        PortSpec {
            name: name.to_string(),
            direction: PortDirection::InOut,
            default_type: PortType::DifferentialConductance,
            allowed_types: vec![PortType::Conductance, PortType::DifferentialConductance],
            is_vector: false,
            null_allowed: false,
            vector_min_len: None,
            vector_max_len: None,
            description: String::new(),
        }
    }

    #[test]
    fn default_differential_conductance_ports_group_bare_node_pairs() {
        let mut circuit = CircuitData::new();
        let ports = vec![gd_port("cntl_in"), gd_port("out")];
        let parsed_ports = vec![
            XspicePort::Analog("CTRL".to_string()),
            XspicePort::Analog("0".to_string()),
            XspicePort::Analog("OUT".to_string()),
            XspicePort::Analog("0".to_string()),
        ];

        let connections =
            coerce_xspice_connections(&mut circuit, &ports, &parsed_ports, "A1", "pswitch")
                .expect("bare node pairs should group into default gd ports");

        assert_eq!(connections.len(), 2);
        assert!(matches!(
            connections[0],
            PortConnection::Differential(ctrl, 0) if ctrl > 0
        ));
        assert!(matches!(
            connections[1],
            PortConnection::Differential(out, 0) if out > 0
        ));
    }

    #[test]
    fn explicit_conductance_connection_must_be_allowed_by_port_spec() {
        let mut circuit = CircuitData::new();
        let port = PortSpec::input("in", PortType::Voltage);
        let parsed = XspicePort::DifferentialConductance {
            pos: "IN".to_string(),
            neg: "0".to_string(),
        };

        let err = coerce_xspice_connection(&mut circuit, &port, &parsed)
            .expect_err("explicit gd must not satisfy a voltage-only port");

        assert!(
            err.to_string().contains("does not allow explicit"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn explicit_digital_connection_must_be_allowed_by_port_spec() {
        let mut circuit = CircuitData::new();
        let port = PortSpec::input("in", PortType::Voltage);
        let parsed = XspicePort::ExplicitDigital("IN".to_string());

        let err = coerce_xspice_connection(&mut circuit, &port, &parsed)
            .expect_err("explicit %d must not satisfy a voltage-only port");

        assert!(
            err.to_string().contains("does not allow explicit Digital"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn explicit_scalar_current_does_not_satisfy_differential_current_port() {
        let mut circuit = CircuitData::new();
        let port = PortSpec::input("in", PortType::DifferentialCurrent);
        let parsed = XspicePort::Current("vmon".to_string());

        let err = coerce_xspice_connection(&mut circuit, &port, &parsed)
            .expect_err("explicit scalar current must not satisfy an id-only port");

        assert!(
            err.to_string().contains("does not allow explicit"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn explicit_scalar_current_input_allocates_single_ended_probe() {
        let mut circuit = CircuitData::new();
        let port = PortSpec::input("in", PortType::Current);
        let parsed = XspicePort::Current("sense".to_string());

        let connection = coerce_xspice_connection(&mut circuit, &port, &parsed)
            .expect("scalar %i input should allocate a current probe");

        assert!(matches!(
            connection,
            PortConnection::CurrentProbe {
                pos,
                neg: 0,
                branch_ordinal
            } if pos > 0 && branch_ordinal > 0
        ));
    }

    #[test]
    fn explicit_differential_current_satisfies_differential_current_port() {
        let mut circuit = CircuitData::new();
        let port = PortSpec::input("in", PortType::DifferentialCurrent);
        let parsed = XspicePort::DifferentialCurrent {
            pos: "sense".to_string(),
            neg: "0".to_string(),
        };

        let connection = coerce_xspice_connection(&mut circuit, &port, &parsed)
            .expect("explicit differential current should satisfy an id port");

        assert!(matches!(
            connection,
            PortConnection::CurrentProbe {
                pos,
                neg: 0,
                branch_ordinal
            } if pos > 0 && branch_ordinal > 0
        ));
    }

    #[test]
    fn nullable_vector_port_accepts_null_connection() {
        let mut circuit = CircuitData::new();
        let ports = vec![PortSpec::vector_input("dir", PortType::Digital).nullable()];
        let parsed_ports = vec![XspicePort::Null];

        let connections =
            coerce_xspice_connections(&mut circuit, &ports, &parsed_ports, "A1", "bidi_bridge")
                .expect("nullable vector port should accept null");

        assert_eq!(connections.len(), 1);
        assert!(matches!(connections[0], PortConnection::Null));
    }

    #[test]
    fn digital_vector_port_preserves_ngspice_inverted_nodes() {
        let mut circuit = CircuitData::new();
        let ports = vec![PortSpec::vector_input("in", PortType::Digital)];
        let parsed_ports = vec![XspicePort::DigitalVectorMixed(vec![
            XspiceDigitalNode::new("A", false),
            XspiceDigitalNode::new("B", true),
            XspiceDigitalNode::new("C", false),
        ])];

        let connections =
            coerce_xspice_connections(&mut circuit, &ports, &parsed_ports, "A1", "d_and")
                .expect("inverted digital vector should coerce to event connections");

        assert_eq!(connections.len(), 1);
        match &connections[0] {
            PortConnection::DigitalVectorMapped(nodes) => {
                assert_eq!(nodes.len(), 3);
                assert!(!nodes[0].inverted);
                assert!(nodes[1].inverted);
                assert!(!nodes[2].inverted);
            }
            other => panic!("expected mapped digital vector, got {other:?}"),
        }
    }

    #[test]
    fn bracketed_typed_current_entries_group_into_typed_vector_connection() {
        let mut circuit = CircuitData::new();
        let ports = vec![PortSpec::vector_input(
            "sense",
            PortType::DifferentialCurrent,
        )];
        let parsed_ports = vec![
            XspicePort::DifferentialCurrent {
                pos: "VDD".to_string(),
                neg: "VBIASP".to_string(),
            },
            XspicePort::DifferentialCurrent {
                pos: "VDD".to_string(),
                neg: "VOP".to_string(),
            },
        ];

        let connections =
            coerce_xspice_connections(&mut circuit, &ports, &parsed_ports, "A1", "seemod2")
                .expect("typed current entries should group into one vector port");

        assert_eq!(connections.len(), 1);
        match &connections[0] {
            PortConnection::TypedAnalogVector(elements) => {
                assert_eq!(elements.len(), 2);
                assert!(matches!(
                    &elements[0],
                    AnalogInputConnection::CurrentProbe {
                        pos,
                        neg,
                        branch_ordinal
                    } if *pos > 0 && *neg > 0 && *branch_ordinal > 0
                ));
                assert!(matches!(
                    &elements[1],
                    AnalogInputConnection::CurrentProbe {
                        pos,
                        neg,
                        branch_ordinal
                    } if *pos > 0 && *neg > 0 && *branch_ordinal > 0
                ));
            }
            other => panic!("expected typed analog vector, got {other:?}"),
        }
    }

    #[test]
    fn analog_vector_port_accepts_bracketed_single_node_when_model_expects_analog() {
        let mut circuit = CircuitData::new();
        let ports = vec![PortSpec::vector_output("out", PortType::Voltage)];
        let parsed_ports = vec![XspicePort::Digital("OUT".to_string())];

        let connections =
            coerce_xspice_connections(&mut circuit, &ports, &parsed_ports, "A1", "dac_bridge")
                .expect("bracketed one-node analog vector should coerce from model port type");

        assert_eq!(connections.len(), 1);
        assert!(matches!(
            &connections[0],
            PortConnection::AnalogVector(nodes) if nodes.len() == 1 && nodes[0] > 0
        ));
    }
}
