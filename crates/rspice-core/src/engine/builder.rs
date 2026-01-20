//! Circuit builder - constructs CircuitData from Netlist
//!
//! This module handles the conversion from parsed netlist elements
//! to the runtime circuit representation.

use super::{Engine, SimulationError, extract_dc_value};
use crate::netlist::{ElementKind, flatten_netlist};
use crate::{CircuitData, Netlist};

impl Engine {
    /// Build circuit from netlist (flattens subcircuits first)
    pub fn build_circuit(&self, netlist: &Netlist) -> Result<CircuitData, SimulationError> {
        let mut circuit = CircuitData::new();

        // Flatten subcircuit instances into top-level elements
        let flat_elements = flatten_netlist(netlist)
            .map_err(|e| SimulationError::Netlist(format!("Flattening error: {}", e)))?;

        for element in &flat_elements {
            match &element.kind {
                ElementKind::Resistor { value } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    circuit.resistors.add(element.name.clone(), np, nn, *value);
                }
                ElementKind::Capacitor { value, .. } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    circuit.capacitors.add(element.name.clone(), np, nn, *value);
                }
                ElementKind::Inductor { value, .. } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let branch = circuit.allocate_branch_named(&element.name);
                    circuit
                        .inductors
                        .add(element.name.clone(), np, nn, branch, *value);
                }
                ElementKind::VoltageSource(spec) => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let branch = circuit.allocate_branch_named(&element.name);
                    let dc_value = extract_dc_value(spec);
                    circuit
                        .voltage_sources
                        .add(element.name.clone(), np, nn, branch, dc_value);
                }
                ElementKind::CurrentSource(spec) => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let dc_value = extract_dc_value(spec);
                    circuit
                        .current_sources
                        .add(element.name.clone(), np, nn, dc_value);
                }
                ElementKind::Diode { model: _ } => {
                    let anode = circuit.get_or_create_node(&element.nodes[0]);
                    let cathode = circuit.get_or_create_node(&element.nodes[1]);
                    let diode = crate::device::Diode::new(element.name.clone(), anode, cathode);
                    circuit.diodes.add(diode);
                }
                ElementKind::Bjt { model: _, bjt_type } => {
                    let collector = circuit.get_or_create_node(&element.nodes[0]);
                    let base = circuit.get_or_create_node(&element.nodes[1]);
                    let emitter = circuit.get_or_create_node(&element.nodes[2]);
                    let bjt = match bjt_type {
                        crate::netlist::BjtType::Npn => crate::device::Bjt::new_npn(
                            element.name.clone(),
                            collector,
                            base,
                            emitter,
                        ),
                        crate::netlist::BjtType::Pnp => crate::device::Bjt::new_pnp(
                            element.name.clone(),
                            collector,
                            base,
                            emitter,
                        ),
                    };
                    circuit.bjts.add(bjt);
                }
                ElementKind::Mosfet { model, mos_type } => {
                    let drain = circuit.get_or_create_node(&element.nodes[0]);
                    let gate = circuit.get_or_create_node(&element.nodes[1]);
                    let source = circuit.get_or_create_node(&element.nodes[2]);
                    let bulk = circuit.get_or_create_node(&element.nodes[3]);
                    let mut mosfet = match mos_type {
                        crate::netlist::MosType::Nmos => crate::device::Mosfet::new_nmos(
                            element.name.clone(),
                            drain,
                            gate,
                            source,
                            bulk,
                        ),
                        crate::netlist::MosType::Pmos => crate::device::Mosfet::new_pmos(
                            element.name.clone(),
                            drain,
                            gate,
                            source,
                            bulk,
                        ),
                    };

                    // Look up model and apply parameters including LEVEL
                    let model_upper = model.to_uppercase();
                    if let Some(device_model) = netlist
                        .models
                        .iter()
                        .find(|m| m.name.to_uppercase() == model_upper)
                    {
                        // Convert Vec<(String, f64)> to HashMap for with_params
                        let params_map: std::collections::HashMap<String, f64> =
                            device_model.params.iter().cloned().collect();

                        // Extract LEVEL from params (default to 1)
                        let level = params_map.get("LEVEL").copied().unwrap_or(1.0) as i32;
                        mosfet = mosfet.with_level(level);

                        // Apply all model parameters (VTO, KP, GAMMA, KC, NC, etc.)
                        mosfet = mosfet.with_params(&params_map);
                    }

                    circuit.mosfets.add(mosfet);
                }
                ElementKind::Jfet {
                    model: _,
                    jfet_type,
                } => {
                    let drain = circuit.get_or_create_node(&element.nodes[0]);
                    let gate = circuit.get_or_create_node(&element.nodes[1]);
                    let source = circuit.get_or_create_node(&element.nodes[2]);
                    let jfet = match jfet_type {
                        crate::netlist::JfetType::Njf => {
                            crate::device::Jfet::njf(&element.name, drain, gate, source)
                        }
                        crate::netlist::JfetType::Pjf => {
                            crate::device::Jfet::pjf(&element.name, drain, gate, source)
                        }
                    };
                    circuit.jfets.push(jfet);
                }
                // MESFET (GaAs FET) - treat as JFET for now since physics are similar
                ElementKind::Mesfet {
                    model: _,
                    mesfet_type,
                } => {
                    let drain = circuit.get_or_create_node(&element.nodes[0]);
                    let gate = circuit.get_or_create_node(&element.nodes[1]);
                    let source = circuit.get_or_create_node(&element.nodes[2]);
                    // MESFET uses similar equations to JFET - treat as N-channel JFET
                    let jfet = match mesfet_type {
                        crate::netlist::MesfetType::Nmf => {
                            crate::device::Jfet::njf(&element.name, drain, gate, source)
                        }
                        crate::netlist::MesfetType::Pmf => {
                            crate::device::Jfet::pjf(&element.name, drain, gate, source)
                        }
                    };
                    circuit.jfets.push(jfet);
                }
                // Controlled sources
                ElementKind::Vcvs {
                    gain,
                    control_nodes,
                } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let cp = circuit.get_or_create_node(&control_nodes.0);
                    let cn = circuit.get_or_create_node(&control_nodes.1);
                    let branch = circuit.allocate_branch();
                    circuit
                        .vcvs
                        .add(element.name.clone(), np, nn, cp, cn, branch, *gain);
                }
                ElementKind::Vccs {
                    transconductance,
                    control_nodes,
                } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let cp = circuit.get_or_create_node(&control_nodes.0);
                    let cn = circuit.get_or_create_node(&control_nodes.1);
                    circuit
                        .vccs
                        .add(element.name.clone(), np, nn, cp, cn, *transconductance);
                }
                ElementKind::Cccs {
                    gain,
                    control_element,
                } => {
                    // CCCS needs the branch of a controlling voltage source
                    // Register for deferred resolution after all elements are added
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let cccs_idx = circuit.cccs.len();
                    // Add with placeholder branch (will be resolved later)
                    circuit.cccs.add(element.name.clone(), np, nn, 0, *gain);
                    circuit.add_cccs_pending(cccs_idx, control_element.clone());
                }
                ElementKind::Ccvs {
                    transresistance,
                    control_element,
                } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let branch = circuit.allocate_branch_named(&element.name);
                    let ccvs_idx = circuit.ccvs.len();
                    // Add with placeholder control branch (will be resolved later)
                    circuit
                        .ccvs
                        .add(element.name.clone(), np, nn, branch, 0, *transresistance);
                    circuit.add_ccvs_pending(ccvs_idx, control_element.clone());
                }
                // Behavioral sources
                ElementKind::BehavioralVoltage { expression } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let branch = circuit.allocate_branch_named(&element.name);

                    let bvs = crate::device::BehavioralVoltageSource::new(
                        element.name.clone(),
                        np,
                        nn,
                        branch,
                        expression,
                    );
                    circuit.behavioral_sources.add_voltage(bvs);
                }
                ElementKind::BehavioralCurrent { expression } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);

                    let bcs = crate::device::BehavioralCurrentSource::new(
                        element.name.clone(),
                        np,
                        nn,
                        expression,
                    );
                    circuit.behavioral_sources.add_current(bcs);
                }
                // Subcircuit instances should be flattened before reaching here
                ElementKind::Subcircuit { .. } => {}

                // New element types
                ElementKind::VSwitch {
                    control_pos,
                    control_neg,
                    model: _,
                    initial_state: _,
                } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let cp = circuit.get_or_create_node(control_pos);
                    let cn = circuit.get_or_create_node(control_neg);
                    // Create voltage-controlled switch
                    let sw = crate::device::VoltageSwitch::new(
                        element.name.clone(),
                        np,
                        nn, // Switch terminals
                        cp,
                        cn, // Control terminals
                    );
                    circuit.vswitches.push(sw);
                }
                ElementKind::ISwitch {
                    control_element,
                    model: _,
                    initial_state: _,
                } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    // Create current-controlled switch
                    let sw = crate::device::CurrentSwitch::new(
                        element.name.clone(),
                        np,
                        nn,
                        control_element.clone(), // Control source name
                    );
                    circuit.iswitches.push(sw);
                }
                ElementKind::TransmissionLine { z0, td, freq, nl } => {
                    let p1p = circuit.get_or_create_node(&element.nodes[0]);
                    let p1n = circuit.get_or_create_node(&element.nodes[1]);
                    let p2p = circuit.get_or_create_node(&element.nodes[2]);
                    let p2n = circuit.get_or_create_node(&element.nodes[3]);

                    // Calculate delay from TD or F/NL
                    let delay = if let Some(t) = td {
                        *t
                    } else if let (Some(f), Some(n)) = (freq, nl) {
                        // TD = NL / F
                        n / f
                    } else {
                        1e-9 // Default 1ns
                    };

                    let tline = crate::device::TransmissionLine::new(
                        element.name.clone(),
                        p1p,
                        p1n,
                        p2p,
                        p2n,
                        *z0,
                        delay,
                    );
                    circuit.tlines.push(tline);
                }
                ElementKind::Coupling {
                    inductors,
                    coefficient,
                } => {
                    // Store coupling for later resolution
                    circuit.couplings.push(crate::device::InductorCoupling::new(
                        element.name.clone(),
                        inductors.clone(),
                        *coefficient,
                    ));
                }

                // XSPICE code model instances
                ElementKind::Xspice {
                    model,
                    ports,
                    params,
                } => {
                    // Convert parsed XspicePort to PortConnection with resolved node IDs
                    let mut connections: Vec<crate::xspice::PortConnection> = Vec::new();
                    for port in ports {
                        let connection = match port {
                            crate::netlist::XspicePort::Analog(name) => {
                                let node = if name.eq_ignore_ascii_case("0") {
                                    0
                                } else {
                                    circuit.get_or_create_node(name)
                                };
                                crate::xspice::PortConnection::Analog(node)
                            }
                            crate::netlist::XspicePort::Digital(name) => {
                                let node = if name.eq_ignore_ascii_case("0") {
                                    0
                                } else {
                                    circuit.get_or_create_node(name)
                                };
                                crate::xspice::PortConnection::Digital(node)
                            }
                            crate::netlist::XspicePort::AnalogVector(names) => {
                                let nodes: Vec<usize> = names
                                    .iter()
                                    .map(|n| {
                                        if n.eq_ignore_ascii_case("0") {
                                            0
                                        } else {
                                            circuit.get_or_create_node(n)
                                        }
                                    })
                                    .collect();
                                crate::xspice::PortConnection::AnalogVector(nodes)
                            }
                            crate::netlist::XspicePort::DigitalVector(names) => {
                                let nodes: Vec<usize> = names
                                    .iter()
                                    .map(|n| {
                                        if n.eq_ignore_ascii_case("0") {
                                            0
                                        } else {
                                            circuit.get_or_create_node(n)
                                        }
                                    })
                                    .collect();
                                crate::xspice::PortConnection::DigitalVector(nodes)
                            }
                            crate::netlist::XspicePort::DifferentialVoltage { pos, neg }
                            | crate::netlist::XspicePort::DifferentialCurrent { pos, neg } => {
                                let pos_node = if pos.eq_ignore_ascii_case("0") {
                                    0
                                } else {
                                    circuit.get_or_create_node(pos)
                                };
                                let neg_node = if neg.eq_ignore_ascii_case("0") {
                                    0
                                } else {
                                    circuit.get_or_create_node(neg)
                                };
                                crate::xspice::PortConnection::Differential(pos_node, neg_node)
                            }
                            crate::netlist::XspicePort::Null => crate::xspice::PortConnection::Null,
                        };
                        connections.push(connection);
                    }

                    // Look up the model in the registry and create instance
                    if let Some(code_model) = circuit.xspice_registry.get(model) {
                        match crate::xspice::XspiceInstance::new(
                            element.name.clone(),
                            code_model.clone(),
                            connections,
                            params,
                        ) {
                            Ok(instance) => {
                                circuit.xspice_instances.push(instance);
                                log::debug!(
                                    "Created XSPICE instance {}: model={}, ports={}",
                                    element.name,
                                    model,
                                    ports.len()
                                );
                            }
                            Err(e) => {
                                log::warn!(
                                    "Failed to create XSPICE instance {}: {}",
                                    element.name,
                                    e
                                );
                            }
                        }
                    } else {
                        log::warn!(
                            "Unknown XSPICE model '{}' for element {}",
                            model,
                            element.name
                        );
                    }
                }
            }
        }

        // Resolve all pending CCCS/CCVS control element references
        circuit
            .resolve_control_elements()
            .map_err(|e| SimulationError::Circuit(e.to_string()))?;

        // Load Verilog-A models from .VERILOGA includes
        #[cfg(feature = "veriloga")]
        {
            for include in &netlist.veriloga_includes {
                // Compile the Verilog-A source file
                let compiler = rspice_veriloga::VerilogACompiler::default();
                let model = compiler.compile_file(&include.file_path).map_err(|e| {
                    SimulationError::Netlist(format!(
                        "Failed to compile Verilog-A '{}': {}",
                        include.file_path.display(),
                        e
                    ))
                })?;

                log::info!(
                    "Loaded Verilog-A model '{}' from {}",
                    model.name,
                    include.file_path.display()
                );

                // Store the compiled model for later device instantiation
                // Note: Actual device instances are created based on .MODEL and X statements
                // For now, we just validate that the VA file compiles successfully
                let _ = model; // TODO: Store models for lookup during device instantiation
            }
        }

        // Ensure ground reference exists (LTspice-compatible behavior)
        // If no node "0" was specified, auto-select a reference node
        circuit.ensure_ground_reference();

        Ok(circuit)
    }
}
