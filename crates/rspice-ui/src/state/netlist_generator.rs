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
fn build_connectivity_graph(
    schematic: &SchematicState,
) -> (HashMap<Point, String>, HashMap<String, Vec<Point>>) {
    let mut point_to_net: HashMap<Point, String> = HashMap::new();
    let mut net_to_points: HashMap<String, Vec<Point>> = HashMap::new();
    let mut net_counter = 0u32;

    // Collect all wire points and build adjacency
    let mut wire_points: HashSet<Point> = HashSet::new();
    for wire in &schematic.wires {
        for point in &wire.points {
            wire_points.insert(*point);
        }
        // Also add all points along segments
        for segment in wire.points.windows(2) {
            let (p1, p2) = (segment[0], segment[1]);
            if p1.x == p2.x {
                // Vertical segment
                let (y_min, y_max) = if p1.y < p2.y {
                    (p1.y, p2.y)
                } else {
                    (p2.y, p1.y)
                };
                for y in y_min..=y_max {
                    wire_points.insert(Point::new(p1.x, y));
                }
            } else if p1.y == p2.y {
                // Horizontal segment
                let (x_min, x_max) = if p1.x < p2.x {
                    (p1.x, p2.x)
                } else {
                    (p2.x, p1.x)
                };
                for x in x_min..=x_max {
                    wire_points.insert(Point::new(x, p1.y));
                }
            }
        }
    }

    // Add component terminal positions
    for comp in &schematic.components {
        for (_, pos) in comp.terminal_positions() {
            wire_points.insert(pos);
        }
    }

    // Check for net labels and assign their names first
    let mut label_names: HashMap<Point, String> = HashMap::new();
    for label in &schematic.net_labels {
        label_names.insert(label.pos, label.name.clone());
    }

    // Pre-seed ground terminals as "0" before flood-fill
    // This ensures all connected points get the ground net name
    for comp in &schematic.components {
        if comp.kind == ComponentType::Ground {
            let terminals = comp.terminal_positions();
            if let Some((_, pos)) = terminals.first() {
                label_names.insert(*pos, "0".to_string());
            }
        }
    }

    // Flood-fill to assign net names
    // IMPORTANT: Process labeled points (including ground) FIRST to ensure they propagate
    // their names before auto-generated names are assigned
    let mut visited: HashSet<Point> = HashSet::new();

    // Collect labeled points to process first (ground and net labels)
    let labeled_points: Vec<Point> = label_names.keys().cloned().collect();

    // Process labeled points first (ground, net labels), then remaining wire points
    let points_to_process: Vec<Point> = labeled_points
        .into_iter()
        .chain(wire_points.iter().cloned())
        .collect();

    for start_point in points_to_process {
        if visited.contains(&start_point) {
            continue;
        }
        if !wire_points.contains(&start_point) {
            continue; // Skip labeled points not on wires
        }

        // Determine net name - check labels first (includes ground)
        let net_name = if let Some(label) = label_names.get(&start_point) {
            label.clone()
        } else {
            net_counter += 1;
            format!("N{:03}", net_counter)
        };

        // Flood fill from this point
        let mut stack = vec![start_point];
        let mut net_points = Vec::new();

        while let Some(point) = stack.pop() {
            if visited.contains(&point) {
                continue;
            }
            if !wire_points.contains(&point) {
                continue;
            }

            visited.insert(point);
            net_points.push(point);
            point_to_net.insert(point, net_name.clone());

            // Check if this point has a label (propagate name)
            if let Some(label) = label_names.get(&point) {
                // Update all points in this net to use the label
                for p in &net_points {
                    point_to_net.insert(*p, label.clone());
                }
            }

            // Add adjacent points (on wire grid)
            for neighbor in point.neighbors() {
                if wire_points.contains(&neighbor) && !visited.contains(&neighbor) {
                    stack.push(neighbor);
                }
            }
        }

        if !net_points.is_empty() {
            // Get final net name (may have been updated by label)
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
