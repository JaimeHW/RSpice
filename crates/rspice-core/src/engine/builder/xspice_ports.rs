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

pub(in crate::engine::builder) fn coerce_xspice_connection(
    circuit: &mut CircuitData,
    port_spec: &crate::xspice::PortSpec,
    parsed_port: &crate::netlist::XspicePort,
) -> Result<crate::xspice::PortConnection, SimulationError> {
    use crate::netlist::XspicePort;
    use crate::xspice::{PortConnection, PortType};

    let expects_event = matches!(
        port_spec.default_type,
        PortType::Digital | PortType::Real | PortType::Integer
    );

    let convert_scalar = |circuit: &mut CircuitData, node_name: &str| {
        let node = resolve_xspice_node(circuit, node_name);
        if port_spec.is_vector {
            if expects_event {
                PortConnection::DigitalVector(vec![node])
            } else {
                PortConnection::AnalogVector(vec![node])
            }
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
            if expects_event {
                Ok(PortConnection::DigitalVector(nodes))
            } else {
                Ok(PortConnection::AnalogVector(nodes))
            }
        } else if nodes.len() == 1 {
            if expects_event {
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

    match parsed_port {
        XspicePort::Analog(name) | XspicePort::Digital(name) => Ok(convert_scalar(circuit, name)),
        XspicePort::AnalogVector(names) | XspicePort::DigitalVector(names) => {
            convert_vector(circuit, names)
        }
        XspicePort::DifferentialVoltage { pos, neg }
        | XspicePort::DifferentialCurrent { pos, neg } => {
            let pos_node = resolve_xspice_node(circuit, pos);
            let neg_node = resolve_xspice_node(circuit, neg);
            Ok(PortConnection::Differential(pos_node, neg_node))
        }
        XspicePort::Null => Ok(PortConnection::Null),
    }
}
