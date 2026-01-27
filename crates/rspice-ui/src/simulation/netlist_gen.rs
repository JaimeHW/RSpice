//! Netlist Generator - Schematic to SPICE Netlist Conversion
//!
//! Commercial-grade netlist generation following Cadence Spectre conventions:
//! - Node connectivity extraction via wire tracing
//! - Ground detection (GND nets → node 0)
//! - SPICE instance line generation for all component types
//! - Subcircuit and model statement generation
//! - Analysis command generation from UI configuration
//!
//! # Architecture
//!
//! The netlist generator works in phases:
//! 1. **Node extraction**: Build connectivity graph from wires and terminals
//! 2. **Node naming**: Assign node numbers/names (with ground = 0)
//! 3. **Instance generation**: Generate SPICE lines for each component
//! 4. **Analysis generation**: Generate analysis commands from config
//!
//! # Example
//!
//! ```rust,ignore
//! use rspice_ui::state::schematic::SchematicState;
//! use rspice_ui::app::simulation::netlist_gen::NetlistGenerator;
//!
//! let schematic = SchematicState::default();
//! let generator = NetlistGenerator::new(&schematic);
//! let netlist = generator.generate();
//! ```

use std::collections::{BTreeMap, HashMap, HashSet};

use crate::state::{Component, ComponentType, Point, SchematicState, Wire};

//=============================================================================
// NetlistResult (Compatibility API)
//=============================================================================

/// Result of netlist generation (compatibility API)
///
/// This provides the same interface as the legacy netlist generator
/// for backward compatibility with existing code.
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

/// Generate a SPICE netlist from a schematic (convenience function)
///
/// This provides the same API as the legacy netlist generator for
/// backward compatibility. For more control, use `NetlistGenerator` directly.
pub fn generate_netlist(schematic: &SchematicState) -> NetlistResult {
    let mut gen = NetlistGenerator::new(schematic);
    let netlist = gen.generate();

    // Build the nets map from the generator's data
    let mut nets: HashMap<String, Vec<Point>> = HashMap::new();
    let mut point_to_net: HashMap<Point, String> = HashMap::new();

    for net in gen.nets() {
        let name = net.spice_name();
        let points: Vec<Point> = net.points.iter().copied().collect();
        for &p in &points {
            point_to_net.insert(p, name.clone());
        }
        nets.insert(name, points);
    }

    NetlistResult {
        netlist,
        nets,
        point_to_net,
        warnings: Vec::new(),
        errors: Vec::new(),
    }
}

//=============================================================================
// Node Net
//=============================================================================

/// Represents an electrical net (set of connected points)

#[derive(Debug, Clone)]
pub struct Net {
    /// Unique identifier for this net
    pub id: usize,

    /// All grid points that belong to this net
    pub points: HashSet<Point>,

    /// User-assigned label (if any, from net label component)
    pub label: Option<String>,
}

impl Net {
    /// Create a new empty net
    pub fn new(id: usize) -> Self {
        Self {
            id,
            points: HashSet::new(),
            label: None,
        }
    }

    /// Check if this net contains a point
    pub fn contains(&self, point: Point) -> bool {
        self.points.contains(&point)
    }

    /// Add a point to this net
    pub fn add_point(&mut self, point: Point) {
        self.points.insert(point);
    }

    /// Merge another net into this one
    pub fn merge(&mut self, other: &Net) {
        self.points.extend(&other.points);
        if self.label.is_none() {
            self.label = other.label.clone();
        }
    }

    /// Get SPICE node name
    ///
    /// Returns:
    /// - "0" for ground nets
    /// - User label if provided
    /// - "net{id}" otherwise
    pub fn spice_name(&self) -> String {
        if let Some(ref label) = self.label {
            if label.eq_ignore_ascii_case("0")
                || label.eq_ignore_ascii_case("gnd")
                || label.eq_ignore_ascii_case("ground")
            {
                return "0".to_string();
            }
            return label.clone();
        }
        format!("net{}", self.id)
    }
}

//=============================================================================
// Netlist Generator
//=============================================================================

/// Commercial-grade SPICE netlist generator
///
/// Extracts node connectivity from schematic and generates standard SPICE netlist.
pub struct NetlistGenerator<'a> {
    /// Reference to the schematic
    schematic: &'a SchematicState,

    /// Extracted nets (connected node groups)
    nets: Vec<Net>,

    /// Map from grid point to net ID
    point_to_net: HashMap<Point, usize>,

    /// Ground net ID (if found)
    ground_net: Option<usize>,

    /// Generated netlist lines
    lines: Vec<String>,

    /// Model definitions needed
    models: BTreeMap<String, String>,

    /// Subcircuit definitions needed
    subcircuits: Vec<String>,
}

impl<'a> NetlistGenerator<'a> {
    /// Create a new netlist generator for the given schematic
    pub fn new(schematic: &'a SchematicState) -> Self {
        Self {
            schematic,
            nets: Vec::new(),
            point_to_net: HashMap::new(),
            ground_net: None,
            lines: Vec::new(),
            models: BTreeMap::new(),
            subcircuits: Vec::new(),
        }
    }

    /// Generate complete SPICE netlist
    ///
    /// Returns the netlist as a string.
    pub fn generate(&mut self) -> String {
        // Phase 1: Extract node connectivity
        self.extract_nets();

        // Phase 2: Identify ground
        self.identify_ground();

        // Phase 3: Generate header
        self.generate_header();

        // Phase 4: Generate component instances
        self.generate_instances();

        // Phase 5: Add models if needed
        self.generate_models();

        // Phase 6: Add analysis commands placeholder
        self.lines.push(String::new());
        self.lines.push("* Analysis commands".to_string());
        self.lines.push(".op".to_string());

        // Phase 7: End statement
        self.lines.push(String::new());
        self.lines.push(".end".to_string());

        self.lines.join("\n")
    }

    /// Generate netlist with custom analysis commands
    pub fn generate_with_analysis(&mut self, analysis_lines: &[String]) -> String {
        // Phase 1: Extract node connectivity
        self.extract_nets();

        // Phase 2: Identify ground
        self.identify_ground();

        // Phase 3: Generate header
        self.generate_header();

        // Phase 4: Generate component instances
        self.generate_instances();

        // Phase 5: Add models if needed
        self.generate_models();

        // Phase 6: Add analysis commands
        if !analysis_lines.is_empty() {
            self.lines.push(String::new());
            self.lines.push("* Analysis commands".to_string());
            for line in analysis_lines {
                self.lines.push(line.clone());
            }
        }

        // Phase 7: End statement
        self.lines.push(String::new());
        self.lines.push(".end".to_string());

        self.lines.join("\n")
    }

    //-------------------------------------------------------------------------
    // Phase 1: Net Extraction
    //-------------------------------------------------------------------------

    /// Extract electrical nets from wire connectivity
    fn extract_nets(&mut self) {
        // Build point graph from wires
        let mut point_graph: HashMap<Point, HashSet<Point>> = HashMap::new();

        // Add all wire segments to the graph
        for wire in &self.schematic.wires {
            self.add_wire_to_graph(wire, &mut point_graph);
        }

        // Add component terminals to graph
        for component in &self.schematic.components {
            for (_, terminal_pos) in component.terminal_positions() {
                point_graph.entry(terminal_pos).or_default();
            }
        }

        // Connect terminals to wires at matching points
        for component in &self.schematic.components {
            for (_, terminal_pos) in component.terminal_positions() {
                // Check if any wire passes through this terminal
                for wire in &self.schematic.wires {
                    if wire.contains_point(terminal_pos) {
                        // Connect terminal to wire endpoints
                        for point in &wire.points {
                            if *point != terminal_pos {
                                point_graph.entry(terminal_pos).or_default().insert(*point);
                                point_graph.entry(*point).or_default().insert(terminal_pos);
                            }
                        }
                    }
                }
            }
        }

        // Flood-fill to find connected components (nets)
        let mut visited: HashSet<Point> = HashSet::new();
        let mut net_id = 1;

        let all_points: Vec<Point> = point_graph.keys().copied().collect();
        for start_point in all_points {
            if visited.contains(&start_point) {
                continue;
            }

            let mut net = Net::new(net_id);
            let mut stack = vec![start_point];

            while let Some(point) = stack.pop() {
                if visited.insert(point) {
                    net.add_point(point);
                    self.point_to_net.insert(point, net_id);

                    if let Some(neighbors) = point_graph.get(&point) {
                        for neighbor in neighbors {
                            if !visited.contains(neighbor) {
                                stack.push(*neighbor);
                            }
                        }
                    }
                }
            }

            if !net.points.is_empty() {
                self.nets.push(net);
                net_id += 1;
            }
        }
    }

    /// Add wire points and connections to the graph
    fn add_wire_to_graph(&self, wire: &Wire, graph: &mut HashMap<Point, HashSet<Point>>) {
        // Connect consecutive points
        for i in 0..wire.points.len() {
            let point = wire.points[i];
            graph.entry(point).or_default();

            // Connect to previous point
            if i > 0 {
                let prev = wire.points[i - 1];
                graph.entry(point).or_default().insert(prev);
                graph.entry(prev).or_default().insert(point);

                // Also add all points along the segment (for T-junctions)
                self.add_segment_points(prev, point, graph);
            }
        }
    }

    /// Add intermediate points along a wire segment for T-junction detection
    fn add_segment_points(
        &self,
        start: Point,
        end: Point,
        graph: &mut HashMap<Point, HashSet<Point>>,
    ) {
        // Only handle orthogonal segments
        if start.x == end.x {
            // Vertical segment
            let (min_y, max_y) = if start.y < end.y {
                (start.y, end.y)
            } else {
                (end.y, start.y)
            };
            for y in min_y..=max_y {
                let p = Point::new(start.x, y);
                graph.entry(p).or_default();
                // Connect to segment endpoints
                if p != start {
                    graph.entry(p).or_default().insert(start);
                    graph.entry(start).or_default().insert(p);
                }
                if p != end {
                    graph.entry(p).or_default().insert(end);
                    graph.entry(end).or_default().insert(p);
                }
            }
        } else if start.y == end.y {
            // Horizontal segment
            let (min_x, max_x) = if start.x < end.x {
                (start.x, end.x)
            } else {
                (end.x, start.x)
            };
            for x in min_x..=max_x {
                let p = Point::new(x, start.y);
                graph.entry(p).or_default();
                // Connect to segment endpoints
                if p != start {
                    graph.entry(p).or_default().insert(start);
                    graph.entry(start).or_default().insert(p);
                }
                if p != end {
                    graph.entry(p).or_default().insert(end);
                    graph.entry(end).or_default().insert(p);
                }
            }
        }
    }

    //-------------------------------------------------------------------------
    // Phase 2: Ground Identification
    //-------------------------------------------------------------------------

    /// Identify ground net from Ground components
    fn identify_ground(&mut self) {
        for component in &self.schematic.components {
            if component.kind == ComponentType::Ground {
                // Find the net connected to this ground symbol
                let terminals = component.terminal_positions();
                if let Some((_, terminal_pos)) = terminals.first() {
                    if let Some(&net_id) = self.point_to_net.get(terminal_pos) {
                        self.ground_net = Some(net_id);
                        // Update the net's label
                        if let Some(net) = self.nets.iter_mut().find(|n| n.id == net_id) {
                            net.label = Some("0".to_string());
                        }
                        return;
                    }
                }
            }
        }

        // If no explicit ground, create one at net 0 if it exists
        if !self.nets.is_empty() && self.ground_net.is_none() {
            // Warn: no ground found (in real use, this would be an error)
        }
    }

    //-------------------------------------------------------------------------
    // Phase 3: Header Generation
    //-------------------------------------------------------------------------

    /// Generate netlist header
    fn generate_header(&mut self) {
        self.lines.push("* RSpice Netlist".to_string());
        self.lines
            .push(format!("* Generated: {}", chrono_lite_timestamp()));
        self.lines
            .push(format!("* Components: {}", self.schematic.components.len()));
        self.lines.push(format!("* Nets: {}", self.nets.len()));
        self.lines.push(String::new());
    }

    //-------------------------------------------------------------------------
    // Phase 4: Instance Generation
    //-------------------------------------------------------------------------

    /// Generate SPICE instance lines for all components
    fn generate_instances(&mut self) {
        self.lines.push("* Circuit netlist".to_string());

        for component in &self.schematic.components {
            if component.kind == ComponentType::Ground {
                // Ground symbol is implicit (node 0)
                continue;
            }

            if let Some(line) = self.generate_instance_line(component) {
                self.lines.push(line);
            }
        }
    }

    /// Generate a single SPICE instance line
    fn generate_instance_line(&mut self, component: &Component) -> Option<String> {
        let terminals = component.terminal_positions();
        let node_names: Vec<String> = terminals
            .iter()
            .map(|(_, pos)| self.get_node_name(*pos))
            .collect();

        match component.kind {
            // Two-terminal passive components: X name node+ node- value
            ComponentType::Resistor
            | ComponentType::Capacitor
            | ComponentType::Inductor
            | ComponentType::Diode => {
                let prefix = component.kind.spice_prefix();
                let nodes = self.format_nodes(&node_names, 2);
                let value = self.format_value(&component.value);
                Some(format!("{}{} {} {}", prefix, component.name, nodes, value))
            }

            // Two-terminal voltage sources
            ComponentType::VoltageSource
            | ComponentType::VoltageSourceAc
            | ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin
            | ComponentType::VoltageSourcePwl
            | ComponentType::VoltageSourceExp
            | ComponentType::VoltageSourceSffm => {
                let nodes = self.format_nodes(&node_names, 2);
                let value = self.format_source_value(component);
                Some(format!("V{} {} {}", component.name, nodes, value))
            }

            // Two-terminal current sources
            ComponentType::CurrentSource
            | ComponentType::CurrentSourceAc
            | ComponentType::CurrentSourcePulse
            | ComponentType::CurrentSourceSin
            | ComponentType::CurrentSourcePwl
            | ComponentType::CurrentSourceExp
            | ComponentType::CurrentSourceNoise => {
                let nodes = self.format_nodes(&node_names, 2);
                let value = self.format_source_value(component);
                Some(format!("I{} {} {}", component.name, nodes, value))
            }

            // Three-terminal BJT: Q name C B E model
            ComponentType::NpnBjt | ComponentType::PnpBjt => {
                let nodes = self.format_nodes(&node_names, 3);
                let model = self.get_bjt_model(component);
                Some(format!("Q{} {} {}", component.name, nodes, model))
            }

            // Four-terminal MOSFET: M name D G S B model
            ComponentType::Nmos | ComponentType::Pmos => {
                let nodes = self.format_nodes(&node_names, 4);
                let model = self.get_mosfet_model(component);
                Some(format!("M{} {} {}", component.name, nodes, model))
            }

            // Three-terminal JFET: J name D G S model
            ComponentType::Njfet | ComponentType::Pjfet => {
                let nodes = self.format_nodes(&node_names, 3);
                let model = self.get_jfet_model(component);
                Some(format!("J{} {} {}", component.name, nodes, model))
            }

            // Controlled sources (4 terminals: + - control+ control-)
            ComponentType::Vcvs => {
                let nodes = self.format_nodes(&node_names, 4);
                let gain = self.format_value(&component.value);
                Some(format!("E{} {} {}", component.name, nodes, gain))
            }

            ComponentType::Vccs => {
                let nodes = self.format_nodes(&node_names, 4);
                let gain = self.format_value(&component.value);
                Some(format!("G{} {} {}", component.name, nodes, gain))
            }

            ComponentType::Ccvs => {
                let nodes = self.format_nodes(&node_names, 4);
                let gain = self.format_value(&component.value);
                Some(format!("H{} {} {}", component.name, nodes, gain))
            }

            ComponentType::Cccs => {
                let nodes = self.format_nodes(&node_names, 4);
                let gain = self.format_value(&component.value);
                Some(format!("F{} {} {}", component.name, nodes, gain))
            }

            // Ground - handled separately
            ComponentType::Ground => None,

            // XSPICE components
            _ if component.kind.is_xspice() => {
                let nodes = node_names.join(" ");
                let model = format!("{}_model", component.name.to_lowercase());
                Some(format!("A{} {} {}", component.name, nodes, model))
            }

            // Catch-all for unhandled types
            _ => {
                let prefix = component.kind.spice_prefix();
                let nodes = node_names.join(" ");
                let value = &component.value;
                Some(format!("{}{} {} {}", prefix, component.name, nodes, value))
            }
        }
    }

    /// Get node name for a grid point
    fn get_node_name(&self, point: Point) -> String {
        if let Some(&net_id) = self.point_to_net.get(&point) {
            if let Some(net) = self.nets.iter().find(|n| n.id == net_id) {
                return net.spice_name();
            }
        }
        // Floating terminal - assign a unique net
        format!(
            "float_{}",
            point.x.abs() as u32 * 10000 + point.y.abs() as u32
        )
    }

    /// Format node list for SPICE line
    fn format_nodes(&self, nodes: &[String], expected: usize) -> String {
        if nodes.len() >= expected {
            nodes[..expected].join(" ")
        } else {
            // Pad with ground if not enough terminals
            let mut result = nodes.to_vec();
            while result.len() < expected {
                result.push("0".to_string());
            }
            result.join(" ")
        }
    }

    /// Format component value with SI prefixes
    fn format_value(&self, value: &str) -> String {
        if value.is_empty() {
            return "1".to_string();
        }
        // Already has SPICE-compatible format
        value.to_string()
    }

    /// Format source value specification
    fn format_source_value(&self, component: &Component) -> String {
        let value = &component.value;

        match component.kind {
            ComponentType::VoltageSource | ComponentType::CurrentSource => {
                format!("DC {}", if value.is_empty() { "0" } else { value })
            }
            ComponentType::VoltageSourceAc | ComponentType::CurrentSourceAc => {
                format!("AC {}", if value.is_empty() { "1" } else { value })
            }
            ComponentType::VoltageSourcePulse | ComponentType::CurrentSourcePulse => {
                // PULSE(V1 V2 TD TR TF PW PER)
                format!(
                    "PULSE({})",
                    if value.is_empty() {
                        "0 1 0 1n 1n 10n 20n"
                    } else {
                        value
                    }
                )
            }
            ComponentType::VoltageSourceSin | ComponentType::CurrentSourceSin => {
                // SIN(VO VA FREQ TD THETA)
                format!("SIN({})", if value.is_empty() { "0 1 1k" } else { value })
            }
            ComponentType::VoltageSourcePwl | ComponentType::CurrentSourcePwl => {
                // PWL(T1 V1 T2 V2 ...)
                format!("PWL({})", if value.is_empty() { "0 0 1n 1" } else { value })
            }
            ComponentType::VoltageSourceExp | ComponentType::CurrentSourceExp => {
                // EXP(V1 V2 TD1 TAU1 TD2 TAU2)
                format!(
                    "EXP({})",
                    if value.is_empty() {
                        "0 1 0 1n 10n 1n"
                    } else {
                        value
                    }
                )
            }
            ComponentType::VoltageSourceSffm => {
                // SFFM(VO VA FC MDI FS)
                format!(
                    "SFFM({})",
                    if value.is_empty() {
                        "0 1 1k 1 10"
                    } else {
                        value
                    }
                )
            }
            _ => {
                if value.is_empty() {
                    "DC 0".to_string()
                } else {
                    value.to_string()
                }
            }
        }
    }

    /// Get BJT model name and add to models
    fn get_bjt_model(&mut self, component: &Component) -> String {
        let polarity = if component.kind == ComponentType::NpnBjt {
            "NPN"
        } else {
            "PNP"
        };
        let model_name = format!("{}_{}", polarity.to_lowercase(), component.name);

        // Add default model if not already present
        if !self.models.contains_key(&model_name) {
            self.models.insert(
                model_name.clone(),
                format!(".MODEL {} {} (BF=100 IS=1e-15)", model_name, polarity),
            );
        }

        model_name
    }

    /// Get MOSFET model name and add to models
    fn get_mosfet_model(&mut self, component: &Component) -> String {
        let polarity = if component.kind == ComponentType::Nmos {
            "NMOS"
        } else {
            "PMOS"
        };
        let model_name = format!("{}_{}", polarity.to_lowercase(), component.name);

        // Add default model if not already present
        if !self.models.contains_key(&model_name) {
            self.models.insert(
                model_name.clone(),
                format!(
                    ".MODEL {} {} (LEVEL=1 VTO={} KP=2e-5)",
                    model_name,
                    polarity,
                    if component.kind == ComponentType::Nmos {
                        "0.7"
                    } else {
                        "-0.7"
                    }
                ),
            );
        }

        model_name
    }

    /// Get JFET model name and add to models
    fn get_jfet_model(&mut self, component: &Component) -> String {
        let polarity = if component.kind == ComponentType::Njfet {
            "NJF"
        } else {
            "PJF"
        };
        let model_name = format!("{}_{}", polarity.to_lowercase(), component.name);

        // Add default model if not already present
        if !self.models.contains_key(&model_name) {
            self.models.insert(
                model_name.clone(),
                format!(".MODEL {} {} (VTO=-2 BETA=1e-4)", model_name, polarity),
            );
        }

        model_name
    }

    //-------------------------------------------------------------------------
    // Phase 5: Model Generation
    //-------------------------------------------------------------------------

    /// Generate model statements
    fn generate_models(&mut self) {
        if !self.models.is_empty() {
            self.lines.push(String::new());
            self.lines.push("* Models".to_string());
            for (_, model_line) in &self.models {
                self.lines.push(model_line.clone());
            }
        }
    }

    //-------------------------------------------------------------------------
    // Public Accessors
    //-------------------------------------------------------------------------

    /// Get the extracted nets
    pub fn nets(&self) -> &[Net] {
        &self.nets
    }

    /// Get net by ID
    pub fn net(&self, id: usize) -> Option<&Net> {
        self.nets.iter().find(|n| n.id == id)
    }

    /// Get net for a point
    pub fn net_at(&self, point: Point) -> Option<&Net> {
        self.point_to_net.get(&point).and_then(|&id| self.net(id))
    }

    /// Get ground net ID
    pub fn ground_net_id(&self) -> Option<usize> {
        self.ground_net
    }

    /// Check if schematic has ground
    pub fn has_ground(&self) -> bool {
        self.ground_net.is_some()
    }
}

/// Get a lightweight timestamp without external deps
fn chrono_lite_timestamp() -> String {
    // Use system time for a basic timestamp
    use std::time::{SystemTime, UNIX_EPOCH};
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => format!("{}", d.as_secs()),
        Err(_) => "unknown".to_string(),
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Point;

    // -------------------------------------------------------------------------
    // Net Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_net_new() {
        let net = Net::new(1);
        assert_eq!(net.id, 1);
        assert!(net.points.is_empty());
        assert!(net.label.is_none());
    }

    #[test]
    fn test_net_add_point() {
        let mut net = Net::new(1);
        net.add_point(Point::new(0, 0));
        net.add_point(Point::new(10, 0));
        assert_eq!(net.points.len(), 2);
        assert!(net.contains(Point::new(0, 0)));
        assert!(net.contains(Point::new(10, 0)));
        assert!(!net.contains(Point::new(5, 5)));
    }

    #[test]
    fn test_net_merge() {
        let mut net1 = Net::new(1);
        net1.add_point(Point::new(0, 0));
        net1.add_point(Point::new(10, 0));

        let mut net2 = Net::new(2);
        net2.add_point(Point::new(10, 0));
        net2.add_point(Point::new(10, 10));
        net2.label = Some("VDD".to_string());

        net1.merge(&net2);

        assert_eq!(net1.points.len(), 3); // 0,0 + 10,0 + 10,10 (10,0 deduped)
        assert_eq!(net1.label, Some("VDD".to_string()));
    }

    #[test]
    fn test_net_spice_name_default() {
        let net = Net::new(5);
        assert_eq!(net.spice_name(), "net5");
    }

    #[test]
    fn test_net_spice_name_with_label() {
        let mut net = Net::new(1);
        net.label = Some("VCC".to_string());
        assert_eq!(net.spice_name(), "VCC");
    }

    #[test]
    fn test_net_spice_name_ground() {
        let mut net = Net::new(1);
        net.label = Some("GND".to_string());
        assert_eq!(net.spice_name(), "0");

        net.label = Some("ground".to_string());
        assert_eq!(net.spice_name(), "0");

        net.label = Some("0".to_string());
        assert_eq!(net.spice_name(), "0");
    }

    // -------------------------------------------------------------------------
    // Generator Construction Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_generator_new() {
        let schematic = SchematicState::default();
        let gen = NetlistGenerator::new(&schematic);
        assert!(gen.nets.is_empty());
        assert!(gen.point_to_net.is_empty());
        assert!(gen.ground_net.is_none());
    }

    #[test]
    fn test_generator_empty_schematic() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);
        let netlist = gen.generate();

        assert!(netlist.contains("* RSpice Netlist"));
        assert!(netlist.contains("* Components: 0"));
        assert!(netlist.contains(".end"));
    }

    // -------------------------------------------------------------------------
    // Value Formatting Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_format_value_empty() {
        let schematic = SchematicState::default();
        let gen = NetlistGenerator::new(&schematic);
        assert_eq!(gen.format_value(""), "1");
    }

    #[test]
    fn test_format_value_with_si_prefix() {
        let schematic = SchematicState::default();
        let gen = NetlistGenerator::new(&schematic);
        assert_eq!(gen.format_value("1k"), "1k");
        assert_eq!(gen.format_value("10u"), "10u");
        assert_eq!(gen.format_value("100n"), "100n");
        assert_eq!(gen.format_value("1.5meg"), "1.5meg");
    }

    #[test]
    fn test_format_value_numeric() {
        let schematic = SchematicState::default();
        let gen = NetlistGenerator::new(&schematic);
        assert_eq!(gen.format_value("1000"), "1000");
        assert_eq!(gen.format_value("1e-9"), "1e-9");
    }

    // -------------------------------------------------------------------------
    // Node Formatting Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_format_nodes_exact() {
        let schematic = SchematicState::default();
        let gen = NetlistGenerator::new(&schematic);
        let nodes = vec!["1".to_string(), "2".to_string()];
        assert_eq!(gen.format_nodes(&nodes, 2), "1 2");
    }

    #[test]
    fn test_format_nodes_more_than_expected() {
        let schematic = SchematicState::default();
        let gen = NetlistGenerator::new(&schematic);
        let nodes = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        assert_eq!(gen.format_nodes(&nodes, 2), "1 2");
    }

    #[test]
    fn test_format_nodes_less_than_expected() {
        let schematic = SchematicState::default();
        let gen = NetlistGenerator::new(&schematic);
        let nodes = vec!["1".to_string()];
        assert_eq!(gen.format_nodes(&nodes, 2), "1 0");
    }

    #[test]
    fn test_format_nodes_empty() {
        let schematic = SchematicState::default();
        let gen = NetlistGenerator::new(&schematic);
        let nodes: Vec<String> = vec![];
        assert_eq!(gen.format_nodes(&nodes, 2), "0 0");
    }

    // -------------------------------------------------------------------------
    // Source Value Formatting Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_format_source_value_dc() {
        let schematic = SchematicState::default();
        let gen = NetlistGenerator::new(&schematic);

        let mut comp = Component::new(1, ComponentType::VoltageSource, Point::new(0, 0));
        comp.value = "5".to_string();
        assert!(gen.format_source_value(&comp).contains("DC 5"));
    }

    #[test]
    fn test_format_source_value_ac() {
        let schematic = SchematicState::default();
        let gen = NetlistGenerator::new(&schematic);

        let mut comp = Component::new(1, ComponentType::VoltageSourceAc, Point::new(0, 0));
        comp.value = "1".to_string();
        assert!(gen.format_source_value(&comp).contains("AC 1"));
    }

    #[test]
    fn test_format_source_value_pulse() {
        let schematic = SchematicState::default();
        let gen = NetlistGenerator::new(&schematic);

        let comp = Component::new(1, ComponentType::VoltageSourcePulse, Point::new(0, 0));
        assert!(gen.format_source_value(&comp).contains("PULSE("));
    }

    #[test]
    fn test_format_source_value_sin() {
        let schematic = SchematicState::default();
        let gen = NetlistGenerator::new(&schematic);

        let comp = Component::new(1, ComponentType::VoltageSourceSin, Point::new(0, 0));
        assert!(gen.format_source_value(&comp).contains("SIN("));
    }

    // -------------------------------------------------------------------------
    // Model Generation Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_get_bjt_model_npn() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let comp =
            Component::new(1, ComponentType::NpnBjt, Point::new(0, 0)).with_name_value("Q1", "");
        let model = gen.get_bjt_model(&comp);

        assert!(model.contains("npn"));
        assert!(gen.models.values().any(|m| m.contains("NPN")));
    }

    #[test]
    fn test_get_bjt_model_pnp() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let comp =
            Component::new(1, ComponentType::PnpBjt, Point::new(0, 0)).with_name_value("Q2", "");
        let model = gen.get_bjt_model(&comp);

        assert!(model.contains("pnp"));
        assert!(gen.models.values().any(|m| m.contains("PNP")));
    }

    #[test]
    fn test_get_mosfet_model_nmos() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let comp =
            Component::new(1, ComponentType::Nmos, Point::new(0, 0)).with_name_value("M1", "");
        let model = gen.get_mosfet_model(&comp);

        assert!(model.contains("nmos"));
        assert!(gen.models.values().any(|m| m.contains("NMOS")));
        assert!(gen.models.values().any(|m| m.contains("VTO=0.7")));
    }

    #[test]
    fn test_get_mosfet_model_pmos() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let comp =
            Component::new(1, ComponentType::Pmos, Point::new(0, 0)).with_name_value("M2", "");
        let model = gen.get_mosfet_model(&comp);

        assert!(model.contains("pmos"));
        assert!(gen.models.values().any(|m| m.contains("PMOS")));
        assert!(gen.models.values().any(|m| m.contains("VTO=-0.7")));
    }

    #[test]
    fn test_get_jfet_model() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let comp =
            Component::new(1, ComponentType::Njfet, Point::new(0, 0)).with_name_value("J1", "");
        let model = gen.get_jfet_model(&comp);

        assert!(model.contains("njf"));
        assert!(gen.models.values().any(|m| m.contains("NJF")));
    }

    // -------------------------------------------------------------------------
    // Accessor Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_generator_nets_accessor() {
        let schematic = SchematicState::default();
        let gen = NetlistGenerator::new(&schematic);
        assert!(gen.nets().is_empty());
    }

    #[test]
    fn test_generator_has_ground_initially_false() {
        let schematic = SchematicState::default();
        let gen = NetlistGenerator::new(&schematic);
        assert!(!gen.has_ground());
    }

    #[test]
    fn test_generator_ground_net_id_initially_none() {
        let schematic = SchematicState::default();
        let gen = NetlistGenerator::new(&schematic);
        assert!(gen.ground_net_id().is_none());
    }

    // -------------------------------------------------------------------------
    // Integration Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_generate_with_analysis_commands() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let analysis = vec![".tran 1n 100n".to_string(), ".ac dec 10 1 1meg".to_string()];
        let netlist = gen.generate_with_analysis(&analysis);

        assert!(netlist.contains(".tran 1n 100n"));
        assert!(netlist.contains(".ac dec 10 1 1meg"));
        assert!(netlist.contains(".end"));
    }

    #[test]
    fn test_generate_with_empty_analysis() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let netlist = gen.generate_with_analysis(&[]);

        assert!(netlist.contains("* RSpice Netlist"));
        assert!(netlist.contains(".end"));
        // Should not have analysis section when empty
        assert!(!netlist.contains("* Analysis commands"));
    }

    // -------------------------------------------------------------------------
    // Timestamp Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_chrono_lite_timestamp() {
        let ts = chrono_lite_timestamp();
        // Should be numeric (unix timestamp)
        assert!(!ts.is_empty());
        // Should be parseable as a number
        assert!(ts.parse::<u64>().is_ok() || ts == "unknown");
    }

    // -------------------------------------------------------------------------
    // Node Name Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_get_node_name_floating() {
        let schematic = SchematicState::default();
        let gen = NetlistGenerator::new(&schematic);

        // Point not in any net should return float_XXX
        let name = gen.get_node_name(Point::new(5, 3));
        assert!(name.starts_with("float_"));
    }
}
