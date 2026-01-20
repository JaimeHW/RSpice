//! Netlist Generator
//!
//! Converts a schematic into a SPICE netlist by:
//! 1. Building a connectivity graph from wires
//! 2. Assigning net names (using labels or auto-generated N001, N002...)
//! 3. Generating component lines with proper node connections
//! 4. Appending analysis commands

use super::schematic::{ComponentType, Point, SchematicState};
use std::collections::{HashMap, HashSet};

/// Result of netlist generation
#[derive(Debug, Clone)]
pub struct NetlistResult {
    /// The generated SPICE netlist text
    pub netlist: String,

    /// Net name assignments for cross-probing: net_name -> [points]
    pub nets: HashMap<String, Vec<Point>>,

    /// Point to net name mapping for probe lookup: grid_point -> net_name
    pub point_to_net: HashMap<Point, String>,

    /// Any warnings during generation
    pub warnings: Vec<String>,

    /// Any errors that prevent simulation
    pub errors: Vec<String>,
}

/// Generate a SPICE netlist from a schematic
pub fn generate_netlist(schematic: &SchematicState) -> NetlistResult {
    let mut warnings = Vec::new();
    let mut errors = Vec::new();

    // 1. Build connectivity graph
    let (point_to_net, net_to_points) = build_connectivity_graph(schematic);

    // 2. Generate component lines
    let mut lines = Vec::new();
    lines.push("* RSpice Generated Netlist".to_string());
    lines.push(format!(
        "* {} components, {} nets",
        schematic.components.len(),
        net_to_points.len()
    ));
    lines.push(String::new());

    for comp in &schematic.components {
        match generate_component_line(comp, schematic, &point_to_net, &mut warnings) {
            Ok(line) => {
                if !line.is_empty() {
                    lines.push(line);
                }
            }
            Err(e) => errors.push(e),
        }
    }

    // 3. Add default analysis if none specified
    lines.push(String::new());
    lines.push("* Analysis".to_string());
    lines.push(".tran 1u 10m".to_string());
    lines.push(String::new());
    lines.push(".end".to_string());

    // Convert net_to_points to use String keys for return
    let nets: HashMap<String, Vec<Point>> = net_to_points
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();

    NetlistResult {
        netlist: lines.join("\n"),
        nets,
        point_to_net,
        warnings,
        errors,
    }
}

/// Build connectivity graph using flood-fill algorithm
/// Returns: (point -> net_name, net_name -> points)
///
/// IMPORTANT: Professional simulator behavior (LTspice-style):
/// - Crossing wires are NOT electrically connected
/// - Wires only connect at shared endpoints OR explicit junctions
fn build_connectivity_graph(
    schematic: &SchematicState,
) -> (HashMap<Point, String>, HashMap<String, Vec<Point>>) {
    let mut point_to_net: HashMap<Point, String> = HashMap::new();
    let mut net_to_points: HashMap<String, Vec<Point>> = HashMap::new();
    let mut net_counter = 0u32;

    // Collect connection points - ONLY endpoints and junctions, NOT intermediate segment points
    // This is the key difference from unprofessional simulators that auto-connect crossings
    let mut connection_points: HashSet<Point> = HashSet::new();

    // Add wire ENDPOINTS only (first and last point of each wire)
    for wire in &schematic.wires {
        if let Some(first) = wire.points.first() {
            connection_points.insert(*first);
        }
        if let Some(last) = wire.points.last() {
            connection_points.insert(*last);
        }
    }

    // Add explicit junction positions - these connect crossing wires
    for junction in &schematic.junctions {
        connection_points.insert(junction.pos);
    }

    // Add component terminal positions
    for comp in &schematic.components {
        for (_, pos) in comp.terminal_positions() {
            connection_points.insert(pos);
        }
    }

    // Build wire adjacency map: point -> set of connected points
    // Two points are connected if they are endpoints of the same wire
    let mut adjacency: HashMap<Point, HashSet<Point>> = HashMap::new();
    for wire in &schematic.wires {
        if wire.points.len() >= 2 {
            let first = wire.points.first().unwrap();
            let last = wire.points.last().unwrap();
            // Wire endpoints are connected to each other
            adjacency.entry(*first).or_default().insert(*last);
            adjacency.entry(*last).or_default().insert(*first);
        }
    }

    // Check for net labels and assign their names first
    let mut label_names: HashMap<Point, String> = HashMap::new();
    for label in &schematic.net_labels {
        label_names.insert(label.pos, label.name.clone());
    }

    // Pre-seed ground terminals as "0" before flood-fill
    for comp in &schematic.components {
        if comp.kind == ComponentType::Ground {
            let terminals = comp.terminal_positions();
            if let Some((_, pos)) = terminals.first() {
                label_names.insert(*pos, "0".to_string());
            }
        }
    }

    // Flood-fill to assign net names using wire-based connectivity
    let mut visited: HashSet<Point> = HashSet::new();

    // Collect labeled points to process first (ground and net labels)
    let labeled_points: Vec<Point> = label_names.keys().cloned().collect();

    // Process labeled points first, then remaining connection points
    let points_to_process: Vec<Point> = labeled_points
        .into_iter()
        .chain(connection_points.iter().cloned())
        .collect();

    for start_point in points_to_process {
        if visited.contains(&start_point) {
            continue;
        }
        if !connection_points.contains(&start_point) {
            continue;
        }

        // Determine net name - check labels first (includes ground)
        let net_name = if let Some(label) = label_names.get(&start_point) {
            label.clone()
        } else {
            net_counter += 1;
            format!("N{:03}", net_counter)
        };

        // Flood fill from this point using wire-based connectivity
        let mut stack = vec![start_point];
        let mut net_points = Vec::new();

        while let Some(point) = stack.pop() {
            if visited.contains(&point) {
                continue;
            }
            if !connection_points.contains(&point) {
                continue;
            }

            visited.insert(point);
            net_points.push(point);
            point_to_net.insert(point, net_name.clone());

            // Check if this point has a label (propagate name)
            if let Some(label) = label_names.get(&point) {
                for p in &net_points {
                    point_to_net.insert(*p, label.clone());
                }
            }

            // Add wire-connected points (via adjacency map)
            if let Some(neighbors) = adjacency.get(&point) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        stack.push(*neighbor);
                    }
                }
            }

            // Add points at same position (shared endpoints, component terminals, junctions)
            for other in &connection_points {
                if *other == point {
                    continue;
                }
                if *other == point && !visited.contains(other) {
                    stack.push(*other);
                }
            }
        }

        if !net_points.is_empty() {
            let final_name = point_to_net.get(&net_points[0]).unwrap().clone();
            net_to_points.insert(final_name, net_points);
        }
    }

    // Ensure ground net exists for any Ground components
    for comp in &schematic.components {
        if comp.kind == ComponentType::Ground {
            let terminals = comp.terminal_positions();
            if let Some((_, pos)) = terminals.first() {
                point_to_net.insert(*pos, "0".to_string());
                net_to_points.entry("0".to_string()).or_default().push(*pos);
            }
        }
    }

    (point_to_net, net_to_points)
}

/// Generate SPICE line for a single component
fn generate_component_line(
    comp: &super::schematic::Component,
    _schematic: &SchematicState,
    point_to_net: &HashMap<Point, String>,
    warnings: &mut Vec<String>,
) -> Result<String, String> {
    let terminals = comp.terminal_positions();

    // Get net names for each terminal
    let mut nodes: Vec<String> = Vec::new();
    for (term_name, pos) in &terminals {
        match point_to_net.get(pos) {
            Some(net) => nodes.push(net.clone()),
            None => {
                warnings.push(format!(
                    "{}: Terminal {} at ({},{}) not connected",
                    comp.name, term_name, pos.x, pos.y
                ));
                // Use a floating node
                nodes.push(format!("NC_{}_{}_{}", comp.name, pos.x, pos.y));
            }
        }
    }

    // Generate SPICE line based on component type
    let line = match comp.kind {
        ComponentType::Ground => {
            // Ground doesn't generate a SPICE line, it just defines net 0
            String::new()
        }

        ComponentType::Resistor => {
            if nodes.len() >= 2 {
                format!("{} {} {} {}", comp.name, nodes[0], nodes[1], comp.value)
            } else {
                return Err(format!("{}: Resistor needs 2 terminals", comp.name));
            }
        }

        ComponentType::Capacitor => {
            if nodes.len() >= 2 {
                format!("{} {} {} {}", comp.name, nodes[0], nodes[1], comp.value)
            } else {
                return Err(format!("{}: Capacitor needs 2 terminals", comp.name));
            }
        }

        ComponentType::Inductor => {
            if nodes.len() >= 2 {
                format!("{} {} {} {}", comp.name, nodes[0], nodes[1], comp.value)
            } else {
                return Err(format!("{}: Inductor needs 2 terminals", comp.name));
            }
        }

        ComponentType::Diode => {
            if nodes.len() >= 2 {
                let model = if comp.params.is_empty() {
                    "D"
                } else {
                    &comp.params
                };
                format!("{} {} {} {}", comp.name, nodes[0], nodes[1], model)
            } else {
                return Err(format!("{}: Diode needs 2 terminals", comp.name));
            }
        }

        ComponentType::VoltageSource
        | ComponentType::VoltageSourceAc
        | ComponentType::VoltageSourcePulse
        | ComponentType::VoltageSourceSin => {
            if nodes.len() >= 2 {
                let value = if comp.value.is_empty() {
                    "0"
                } else {
                    &comp.value
                };
                format!("{} {} {} {}", comp.name, nodes[0], nodes[1], value)
            } else {
                return Err(format!("{}: Voltage source needs 2 terminals", comp.name));
            }
        }

        ComponentType::CurrentSource => {
            if nodes.len() >= 2 {
                let value = if comp.value.is_empty() {
                    "0"
                } else {
                    &comp.value
                };
                format!("{} {} {} {}", comp.name, nodes[0], nodes[1], value)
            } else {
                return Err(format!("{}: Current source needs 2 terminals", comp.name));
            }
        }

        ComponentType::NpnBjt | ComponentType::PnpBjt => {
            if nodes.len() >= 3 {
                let model = if comp.params.is_empty() {
                    "QN"
                } else {
                    &comp.params
                };
                // BJT: Qname C B E model
                format!(
                    "{} {} {} {} {}",
                    comp.name, nodes[1], nodes[0], nodes[2], model
                )
            } else {
                return Err(format!("{}: BJT needs 3 terminals (C, B, E)", comp.name));
            }
        }

        ComponentType::Nmos | ComponentType::Pmos => {
            if nodes.len() >= 4 {
                let model = if comp.params.is_empty() {
                    if comp.kind == ComponentType::Nmos {
                        "NMOS"
                    } else {
                        "PMOS"
                    }
                } else {
                    &comp.params
                };
                // MOSFET: Mname D G S B model
                format!(
                    "{} {} {} {} {} {}",
                    comp.name, nodes[1], nodes[0], nodes[2], nodes[3], model
                )
            } else if nodes.len() >= 3 {
                // 3-terminal MOSFET (bulk tied to source)
                let model = if comp.params.is_empty() {
                    if comp.kind == ComponentType::Nmos {
                        "NMOS"
                    } else {
                        "PMOS"
                    }
                } else {
                    &comp.params
                };
                format!(
                    "{} {} {} {} {} {}",
                    comp.name, nodes[1], nodes[0], nodes[2], nodes[2], model
                )
            } else {
                return Err(format!(
                    "{}: MOSFET needs at least 3 terminals (D, G, S)",
                    comp.name
                ));
            }
        }

        ComponentType::Njfet | ComponentType::Pjfet => {
            if nodes.len() >= 3 {
                let model = if comp.params.is_empty() {
                    if comp.kind == ComponentType::Njfet {
                        "NJF"
                    } else {
                        "PJF"
                    }
                } else {
                    &comp.params
                };
                // JFET: Jname D G S model
                format!(
                    "{} {} {} {} {}",
                    comp.name, nodes[1], nodes[0], nodes[2], model
                )
            } else {
                return Err(format!("{}: JFET needs 3 terminals (D, G, S)", comp.name));
            }
        }

        ComponentType::CoupledInductor => {
            // K statement: Kname L1 L2 coupling_coefficient
            // The value field should contain "L1 L2 0.99" format
            format!("{} {}", comp.name, comp.value)
        }

        ComponentType::Vcvs => {
            // VCVS: Ename out+ out- ctrl+ ctrl- gain
            if nodes.len() >= 4 {
                let gain = if comp.value.is_empty() {
                    "1"
                } else {
                    &comp.value
                };
                format!(
                    "{} {} {} {} {} {}",
                    comp.name, nodes[0], nodes[1], nodes[2], nodes[3], gain
                )
            } else {
                return Err(format!("{}: VCVS needs 4 terminals", comp.name));
            }
        }

        ComponentType::Vccs => {
            // VCCS: Gname out+ out- ctrl+ ctrl- transconductance
            if nodes.len() >= 4 {
                let gm = if comp.value.is_empty() {
                    "1m"
                } else {
                    &comp.value
                };
                format!(
                    "{} {} {} {} {} {}",
                    comp.name, nodes[0], nodes[1], nodes[2], nodes[3], gm
                )
            } else {
                return Err(format!("{}: VCCS needs 4 terminals", comp.name));
            }
        }

        ComponentType::Ccvs => {
            // CCVS: Hname out+ out- Vsense transresistance
            if nodes.len() >= 2 {
                let rm = if comp.value.is_empty() {
                    "1k"
                } else {
                    &comp.value
                };
                let vsense = if comp.params.is_empty() {
                    "Vsense"
                } else {
                    &comp.params
                };
                format!("{} {} {} {} {}", comp.name, nodes[0], nodes[1], vsense, rm)
            } else {
                return Err(format!("{}: CCVS needs 2 output terminals", comp.name));
            }
        }

        ComponentType::Cccs => {
            // CCCS: Fname out+ out- Vsense current_gain
            if nodes.len() >= 2 {
                let gain = if comp.value.is_empty() {
                    "1"
                } else {
                    &comp.value
                };
                let vsense = if comp.params.is_empty() {
                    "Vsense"
                } else {
                    &comp.params
                };
                format!(
                    "{} {} {} {} {}",
                    comp.name, nodes[0], nodes[1], vsense, gain
                )
            } else {
                return Err(format!("{}: CCCS needs 2 output terminals", comp.name));
            }
        }

        // XSPICE Analog Behavioral Models
        ComponentType::XspiceGain => {
            // A-device: Aname [in] [out] gain
            if nodes.len() >= 2 {
                let gain_val = if comp.value.is_empty() {
                    "1"
                } else {
                    &comp.value
                };
                format!(
                    "{} [{}] [{}] gain gain={}",
                    comp.name, nodes[0], nodes[1], gain_val
                )
            } else {
                return Err(format!("{}: Gain block needs 2 terminals", comp.name));
            }
        }

        ComponentType::XspiceSummer => {
            // A-device: Aname [in1 in2] [out] summer
            if nodes.len() >= 3 {
                format!(
                    "{} [{} {}] [{}] summer",
                    comp.name, nodes[0], nodes[1], nodes[2]
                )
            } else {
                return Err(format!("{}: Summer needs 3 terminals", comp.name));
            }
        }

        ComponentType::XspiceMultiplier => {
            // A-device: Aname [in1 in2] [out] mult
            if nodes.len() >= 3 {
                format!(
                    "{} [{} {}] [{}] mult",
                    comp.name, nodes[0], nodes[1], nodes[2]
                )
            } else {
                return Err(format!("{}: Multiplier needs 3 terminals", comp.name));
            }
        }

        ComponentType::XspiceDivider => {
            // A-device: Aname [num denom] [out] divide
            if nodes.len() >= 3 {
                format!(
                    "{} [{} {}] [{}] divide",
                    comp.name, nodes[0], nodes[1], nodes[2]
                )
            } else {
                return Err(format!("{}: Divider needs 3 terminals", comp.name));
            }
        }

        ComponentType::XspiceLimiter => {
            // A-device: Aname [in] [out] limit
            if nodes.len() >= 2 {
                let params = if comp.params.is_empty() {
                    "out_lower_limit=-1 out_upper_limit=1"
                } else {
                    &comp.params
                };
                format!(
                    "{} [{}] [{}] limit {}",
                    comp.name, nodes[0], nodes[1], params
                )
            } else {
                return Err(format!("{}: Limiter needs 2 terminals", comp.name));
            }
        }

        ComponentType::XspiceIntegrator => {
            // A-device: Aname [in] [out] integrate
            if nodes.len() >= 2 {
                let ic = if comp.value.is_empty() {
                    "0"
                } else {
                    &comp.value
                };
                format!(
                    "{} [{}] [{}] integrate ic={}",
                    comp.name, nodes[0], nodes[1], ic
                )
            } else {
                return Err(format!("{}: Integrator needs 2 terminals", comp.name));
            }
        }

        ComponentType::XspiceDifferentiator => {
            // A-device: Aname [in] [out] d_dt
            if nodes.len() >= 2 {
                format!("{} [{}] [{}] d_dt", comp.name, nodes[0], nodes[1])
            } else {
                return Err(format!("{}: Differentiator needs 2 terminals", comp.name));
            }
        }

        // XSPICE Digital Gates
        ComponentType::XspiceInverter => {
            if nodes.len() >= 2 {
                format!("{} [{}] [{}] d_inverter", comp.name, nodes[0], nodes[1])
            } else {
                return Err(format!("{}: Inverter needs 2 terminals", comp.name));
            }
        }

        ComponentType::XspiceBuffer => {
            if nodes.len() >= 2 {
                format!("{} [{}] [{}] d_buffer", comp.name, nodes[0], nodes[1])
            } else {
                return Err(format!("{}: Buffer needs 2 terminals", comp.name));
            }
        }

        ComponentType::XspiceAndGate => {
            if nodes.len() >= 3 {
                format!(
                    "{} [{} {}] [{}] d_and",
                    comp.name, nodes[0], nodes[1], nodes[2]
                )
            } else {
                return Err(format!("{}: AND gate needs 3 terminals", comp.name));
            }
        }

        ComponentType::XspiceOrGate => {
            if nodes.len() >= 3 {
                format!(
                    "{} [{} {}] [{}] d_or",
                    comp.name, nodes[0], nodes[1], nodes[2]
                )
            } else {
                return Err(format!("{}: OR gate needs 3 terminals", comp.name));
            }
        }

        ComponentType::XspiceNandGate => {
            if nodes.len() >= 3 {
                format!(
                    "{} [{} {}] [{}] d_nand",
                    comp.name, nodes[0], nodes[1], nodes[2]
                )
            } else {
                return Err(format!("{}: NAND gate needs 3 terminals", comp.name));
            }
        }

        ComponentType::XspiceNorGate => {
            if nodes.len() >= 3 {
                format!(
                    "{} [{} {}] [{}] d_nor",
                    comp.name, nodes[0], nodes[1], nodes[2]
                )
            } else {
                return Err(format!("{}: NOR gate needs 3 terminals", comp.name));
            }
        }

        ComponentType::XspiceXorGate => {
            if nodes.len() >= 3 {
                format!(
                    "{} [{} {}] [{}] d_xor",
                    comp.name, nodes[0], nodes[1], nodes[2]
                )
            } else {
                return Err(format!("{}: XOR gate needs 3 terminals", comp.name));
            }
        }

        ComponentType::XspiceTristate => {
            if nodes.len() >= 3 {
                format!(
                    "{} [{}] [{}] [{}] d_tristate",
                    comp.name, nodes[0], nodes[1], nodes[2]
                )
            } else {
                return Err(format!("{}: Tri-state needs 3 terminals", comp.name));
            }
        }

        // XSPICE Sequential Logic
        ComponentType::XspiceDFlipFlop => {
            if nodes.len() >= 4 {
                format!(
                    "{} [{}] [{}] [{}] [{}] d_dff",
                    comp.name, nodes[0], nodes[1], nodes[2], nodes[3]
                )
            } else {
                return Err(format!("{}: D Flip-Flop needs 4 terminals", comp.name));
            }
        }

        ComponentType::XspiceJkFlipFlop => {
            if nodes.len() >= 5 {
                format!(
                    "{} [{}] [{}] [{}] [{}] [{}] d_jkff",
                    comp.name, nodes[0], nodes[1], nodes[2], nodes[3], nodes[4]
                )
            } else {
                return Err(format!("{}: JK Flip-Flop needs 5 terminals", comp.name));
            }
        }

        ComponentType::XspiceSrLatch => {
            if nodes.len() >= 4 {
                format!(
                    "{} [{}] [{}] [{}] [{}] d_srlatch",
                    comp.name, nodes[0], nodes[1], nodes[2], nodes[3]
                )
            } else {
                return Err(format!("{}: SR Latch needs 4 terminals", comp.name));
            }
        }

        // XSPICE Bridges
        ComponentType::XspiceAdcBridge => {
            if nodes.len() >= 2 {
                let params = if comp.params.is_empty() {
                    "in_low=0.5 in_high=2.5"
                } else {
                    &comp.params
                };
                format!(
                    "{} [{}] [{}] adc_bridge {}",
                    comp.name, nodes[0], nodes[1], params
                )
            } else {
                return Err(format!("{}: ADC Bridge needs 2 terminals", comp.name));
            }
        }

        ComponentType::XspiceDacBridge => {
            if nodes.len() >= 2 {
                let params = if comp.params.is_empty() {
                    "out_low=0 out_high=5"
                } else {
                    &comp.params
                };
                format!(
                    "{} [{}] [{}] dac_bridge {}",
                    comp.name, nodes[0], nodes[1], params
                )
            } else {
                return Err(format!("{}: DAC Bridge needs 2 terminals", comp.name));
            }
        }
    };

    Ok(line)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::schematic::{Component, ComponentType, Point, SchematicState, Wire};

    #[test]
    fn test_simple_netlist() {
        let mut schematic = SchematicState::default();

        // Add a voltage source and resistor
        let mut v1 = Component::new(1, ComponentType::VoltageSource, Point::new(0, 0));
        v1.name = "V1".to_string();
        v1.value = "5".to_string();
        schematic.components.push(v1);

        let mut r1 = Component::new(2, ComponentType::Resistor, Point::new(5, 0));
        r1.name = "R1".to_string();
        r1.value = "1k".to_string();
        schematic.components.push(r1);

        // Add ground
        let ground = Component::new(3, ComponentType::Ground, Point::new(0, 5));
        schematic.components.push(ground);

        // Generate netlist
        let result = generate_netlist(&schematic);

        assert!(
            result.errors.is_empty(),
            "Should have no errors: {:?}",
            result.errors
        );
        assert!(result.netlist.contains("V1"));
        assert!(result.netlist.contains("R1"));
    }
}
