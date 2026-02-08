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
    generate_netlist_with_analysis(schematic, &[])
}

/// Generate a SPICE netlist from a schematic with explicit analysis directives.
pub fn generate_netlist_with_analysis(
    schematic: &SchematicState,
    analysis_lines: &[String],
) -> NetlistResult {
    let mut gen = NetlistGenerator::new(schematic);
    let netlist = gen.generate_with_analysis(analysis_lines);

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
        self.generate_with_analysis(&[])
    }

    fn reset_generation_state(&mut self) {
        self.nets.clear();
        self.point_to_net.clear();
        self.ground_net = None;
        self.lines.clear();
        self.models.clear();
        self.subcircuits.clear();
    }

    /// Generate netlist with custom analysis commands
    pub fn generate_with_analysis(&mut self, analysis_lines: &[String]) -> String {
        self.reset_generation_state();

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

        // Phase 6: Add analysis commands (if requested)
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
            // Two-terminal passive components: X name node+ node- value [params]
            // Spectre format: R1 net1 net2 1k m=2 tc1=0.01
            ComponentType::Resistor
            | ComponentType::Capacitor
            | ComponentType::Inductor
            | ComponentType::Diode => {
                let prefix = component.kind.spice_prefix();
                let nodes = self.format_nodes(&node_names, 2);
                let value_with_params =
                    self.format_value_with_params(&component.value, &component.params);
                Some(format!(
                    "{}{} {} {}",
                    prefix, component.name, nodes, value_with_params
                ))
            }

            // Two-terminal voltage sources: V name node+ node- value [params]
            // Spectre format: V1 net1 0 DC 5 acmag=1 acphase=0
            ComponentType::VoltageSource | ComponentType::VoltageSourceAc => {
                let nodes = self.format_nodes(&node_names, 2);
                let source_value = self.format_source_value(component);
                let params = self.format_params(&component.params);
                Some(format!(
                    "{} {} {}{}",
                    component.name, nodes, source_value, params
                ))
            }
            // Voltage sources with positional params (SIN, PULSE, etc.) - no extra params needed
            ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin
            | ComponentType::VoltageSourcePwl
            | ComponentType::VoltageSourceExp
            | ComponentType::VoltageSourceSffm => {
                let nodes = self.format_nodes(&node_names, 2);
                let source_value = self.format_source_value(component);
                Some(format!("{} {} {}", component.name, nodes, source_value))
            }

            // Two-terminal current sources: I name node+ node- value [params]
            // Spectre format: I1 net1 0 DC 1m acmag=1
            ComponentType::CurrentSource | ComponentType::CurrentSourceAc => {
                let nodes = self.format_nodes(&node_names, 2);
                let source_value = self.format_source_value(component);
                let params = self.format_params(&component.params);
                Some(format!(
                    "{} {} {}{}",
                    component.name, nodes, source_value, params
                ))
            }
            // Current sources with positional params (SIN, PULSE, etc.) - no extra params needed
            ComponentType::CurrentSourcePulse
            | ComponentType::CurrentSourceSin
            | ComponentType::CurrentSourcePwl
            | ComponentType::CurrentSourceExp
            | ComponentType::CurrentSourceNoise => {
                let nodes = self.format_nodes(&node_names, 2);
                let source_value = self.format_source_value(component);
                Some(format!("{} {} {}", component.name, nodes, source_value))
            }

            // Three-terminal BJT: Q name C B E model [params]
            // Spectre format: Q1 coll base emit npn_Q1 area=1 m=1
            ComponentType::NpnBjt | ComponentType::PnpBjt => {
                let nodes = self.format_nodes(&node_names, 3);
                let (explicit_model, params_without_model) =
                    Self::extract_model_override(component);
                let model = self.get_bjt_model(component, explicit_model.as_deref());
                let params = self.format_params(&params_without_model);
                Some(format!("{} {} {}{}", component.name, nodes, model, params))
            }

            // Four-terminal MOSFET: M name D G S B model [params]
            // Spectre format: M1 drain gate source bulk nmos_M1 w=1u l=180n as=1p ad=1p
            ComponentType::Nmos | ComponentType::Pmos => {
                let nodes = self.format_nodes(&node_names, 4);
                let model = self.get_mosfet_model(component);
                let params = self.format_params(&component.params);
                Some(format!("{} {} {}{}", component.name, nodes, model, params))
            }

            // Three-terminal JFET: J name D G S model [params]
            // Spectre format: J1 drain gate source njf_J1 area=1 m=1
            ComponentType::Njfet | ComponentType::Pjfet => {
                let nodes = self.format_nodes(&node_names, 3);
                let model = self.get_jfet_model(component);
                let params = self.format_params(&component.params);
                Some(format!("{} {} {}{}", component.name, nodes, model, params))
            }

            // Controlled sources (4 terminals: + - control+ control-)
            // Spectre format: E1 out+ out- in+ in- gain [params]
            ComponentType::Vcvs => {
                let nodes = self.format_nodes(&node_names, 4);
                let gain_with_params =
                    self.format_value_with_params(&component.value, &component.params);
                Some(format!("{} {} {}", component.name, nodes, gain_with_params))
            }

            ComponentType::Vccs => {
                let nodes = self.format_nodes(&node_names, 4);
                let gain_with_params =
                    self.format_value_with_params(&component.value, &component.params);
                Some(format!("{} {} {}", component.name, nodes, gain_with_params))
            }

            ComponentType::Ccvs => {
                let nodes = self.format_nodes(&node_names, 4);
                let gain_with_params =
                    self.format_value_with_params(&component.value, &component.params);
                Some(format!("{} {} {}", component.name, nodes, gain_with_params))
            }

            ComponentType::Cccs => {
                let nodes = self.format_nodes(&node_names, 4);
                let gain_with_params =
                    self.format_value_with_params(&component.value, &component.params);
                Some(format!("{} {} {}", component.name, nodes, gain_with_params))
            }

            // Ground - handled separately
            ComponentType::Ground => None,

            // XSPICE components: A name nodes model [params]
            _ if component.kind.is_xspice() => {
                let nodes = node_names.join(" ");
                let model = format!("{}_model", component.name.to_lowercase());
                let params = self.format_params(&component.params);
                Some(format!("{} {} {}{}", component.name, nodes, model, params))
            }

            // Catch-all for unhandled types
            // Include params for forward compatibility
            _ => {
                let prefix = component.kind.spice_prefix();
                let nodes = node_names.join(" ");
                let value_with_params =
                    self.format_value_with_params(&component.value, &component.params);
                Some(format!(
                    "{}{} {} {}",
                    prefix, component.name, nodes, value_with_params
                ))
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

    /// Format component parameters for SPICE netlist
    ///
    /// Converts the component.params string into proper SPICE format.
    /// Parameters are appended after the value/model in the netlist line.
    ///
    /// # SPICE Parameter Format (Cadence Spectre Parity)
    ///
    /// Passive components: `R1 net1 net2 1k m=2 tc1=0.01`
    /// Sources: `V1 net1 0 DC 5 acmag=1`
    /// MOSFETs: `M1 d g s b nmos w=1u l=180n`
    ///
    /// # Arguments
    /// * `params` - The component.params string (e.g., "m=2 tc1=0.01")
    ///
    /// # Returns
    /// Formatted parameter string with leading space if non-empty
    fn format_params(&self, params: &str) -> String {
        let trimmed = params.trim();
        if trimmed.is_empty() {
            String::new()
        } else {
            // Ensure single space separation from previous content
            format!(" {}", trimmed)
        }
    }

    /// Format component value with optional parameters appended
    ///
    /// Commercial-grade helper that combines value and params into
    /// proper SPICE format: `value [params]`
    fn format_value_with_params(&self, value: &str, params: &str) -> String {
        let formatted_value = self.format_value(value);
        let formatted_params = self.format_params(params);
        format!("{}{}", formatted_value, formatted_params)
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
                let params = crate::properties::parse_params_string(&component.params);
                let v1 = Self::get_param_owned(&params, "v1", value, "0");
                let v2 = Self::get_param_owned(&params, "v2", "", "1");
                let td = Self::get_param_owned(&params, "td", "", "0");
                let tr = Self::get_param_owned(&params, "tr", "", "1n");
                let tf = Self::get_param_owned(&params, "tf", "", "1n");
                let pw = Self::get_param_owned(&params, "pw", "", "10n");
                let per = Self::get_param_owned(&params, "period", "", "20n");
                format!("PULSE({} {} {} {} {} {} {})", v1, v2, td, tr, tf, pw, per)
            }
            ComponentType::VoltageSourceSin | ComponentType::CurrentSourceSin => {
                // SIN(VO VA FREQ TD THETA PHASE)
                let params = crate::properties::parse_params_string(&component.params);
                let vo = Self::get_param_owned(&params, "vo", value, "0");
                let va = Self::get_param_owned(&params, "va", "", "1");
                let freq = Self::get_param_owned(&params, "freq", "", "1k");
                let td = Self::get_param_owned(&params, "td", "", "0");
                let theta = Self::get_param_owned(&params, "theta", "", "0");
                let phase = Self::get_param_owned(&params, "phase", "", "0");
                format!("SIN({} {} {} {} {} {})", vo, va, freq, td, theta, phase)
            }
            ComponentType::VoltageSourcePwl | ComponentType::CurrentSourcePwl => {
                // PWL(T1 V1 T2 V2 ...)
                let params = crate::properties::parse_params_string(&component.params);
                let pwl_data = Self::get_param_owned(&params, "pwl_data", value, "0 0 1n 1");
                format!("PWL({})", pwl_data)
            }
            ComponentType::VoltageSourceExp | ComponentType::CurrentSourceExp => {
                // EXP(V1 V2 TD1 TAU1 TD2 TAU2)
                let params = crate::properties::parse_params_string(&component.params);
                let v1 = Self::get_param_owned(&params, "v1", value, "0");
                let v2 = Self::get_param_owned(&params, "v2", "", "1");
                let td1 = Self::get_param_owned(&params, "td1", "", "0");
                let tau1 = Self::get_param_owned(&params, "tau1", "", "1n");
                let td2 = Self::get_param_owned(&params, "td2", "", "10n");
                let tau2 = Self::get_param_owned(&params, "tau2", "", "1n");
                format!("EXP({} {} {} {} {} {})", v1, v2, td1, tau1, td2, tau2)
            }
            ComponentType::VoltageSourceSffm => {
                // SFFM(VO VA FC MDI FS)
                let params = crate::properties::parse_params_string(&component.params);
                let vo = Self::get_param_owned(&params, "vo", value, "0");
                let va = Self::get_param_owned(&params, "va", "", "1");
                let fc = Self::get_param_owned(&params, "fc", "", "1k");
                let mdi = Self::get_param_owned(&params, "mdi", "", "1");
                let fs = Self::get_param_owned(&params, "fs", "", "10");
                format!("SFFM({} {} {} {} {})", vo, va, fc, mdi, fs)
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

    /// Helper to get param value with fallbacks (returns owned String)
    fn get_param_owned(
        params: &HashMap<String, String>,
        key: &str,
        value_fallback: &str,
        default: &str,
    ) -> String {
        if let Some(v) = params.get(key) {
            if !v.is_empty() {
                return v.clone();
            }
        }
        if !value_fallback.is_empty() {
            value_fallback.to_string()
        } else {
            default.to_string()
        }
    }

    /// Extract optional explicit model name and params string with model= removed.
    ///
    /// Users can provide model either in the primary value field (e.g. "2N2222")
    /// or as `model=<name>` in params. When both are present, params wins.
    fn extract_model_override(component: &Component) -> (Option<String>, String) {
        let mut params_map = crate::properties::parse_params_string(&component.params);
        let explicit_from_params = params_map
            .remove("model")
            .map(|m| m.trim().to_string())
            .filter(|m| !m.is_empty());
        let params_without_model =
            crate::properties::property_bridge::format_params_string(&params_map);

        let explicit_model = explicit_from_params.or_else(|| {
            let value_model = component.value.trim();
            if value_model.is_empty() {
                None
            } else {
                Some(value_model.to_string())
            }
        });

        (explicit_model, params_without_model)
    }

    /// Get BJT model name and add auto-generated default model when needed.
    fn get_bjt_model(&mut self, component: &Component, explicit_model: Option<&str>) -> String {
        if let Some(model_name) = explicit_model.map(str::trim).filter(|s| !s.is_empty()) {
            // Explicit model selected by user: trust it and do NOT inject a generic
            // .MODEL card that could silently override a library model.
            return model_name.to_string();
        }

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
        let model = gen.get_bjt_model(&comp, None);

        assert!(model.contains("npn"));
        assert!(gen.models.values().any(|m| m.contains("NPN")));
    }

    #[test]
    fn test_get_bjt_model_pnp() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let comp =
            Component::new(1, ComponentType::PnpBjt, Point::new(0, 0)).with_name_value("Q2", "");
        let model = gen.get_bjt_model(&comp, None);

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

    #[test]
    fn test_generate_without_analysis_has_no_placeholder_op() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let netlist = gen.generate();

        assert!(netlist.contains("* RSpice Netlist"));
        assert!(netlist.contains(".end"));
        assert!(!netlist.contains("* Analysis commands"));
        assert!(!netlist.contains("\n.op\n"));
    }

    #[test]
    fn test_generate_resets_internal_state_between_calls() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let first = gen.generate_with_analysis(&[".ac dec 10 1 1meg".to_string()]);
        assert!(first.contains(".ac dec 10 1 1meg"));

        let second = gen.generate_with_analysis(&[]);
        assert!(!second.contains(".ac dec 10 1 1meg"));
        assert!(!second.contains("* Analysis commands"));
    }

    #[test]
    fn test_generate_netlist_with_analysis_convenience_api() {
        let schematic = SchematicState::default();
        let analysis = vec![".tran 1n 100n".to_string()];

        let result = generate_netlist_with_analysis(&schematic, &analysis);

        assert!(result.netlist.contains(".tran 1n 100n"));
        assert!(result.netlist.contains(".end"));
        assert!(result.errors.is_empty());
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

    // =========================================================================
    // Parameter Serialization Tests (Spectre Parity)
    // =========================================================================
    //
    // These tests verify that component.params are correctly appended to
    // netlist lines, following Cadence Spectre conventions.

    // -------------------------------------------------------------------------
    // Helper Function Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_format_params_empty() {
        let schematic = SchematicState::default();
        let gen = NetlistGenerator::new(&schematic);
        assert_eq!(gen.format_params(""), "");
        assert_eq!(gen.format_params("   "), "");
    }

    #[test]
    fn test_format_params_single() {
        let schematic = SchematicState::default();
        let gen = NetlistGenerator::new(&schematic);
        assert_eq!(gen.format_params("m=2"), " m=2");
    }

    #[test]
    fn test_format_params_multiple() {
        let schematic = SchematicState::default();
        let gen = NetlistGenerator::new(&schematic);
        assert_eq!(gen.format_params("m=2 tc1=0.01"), " m=2 tc1=0.01");
    }

    #[test]
    fn test_format_params_with_whitespace() {
        let schematic = SchematicState::default();
        let gen = NetlistGenerator::new(&schematic);
        // Leading/trailing whitespace should be trimmed
        assert_eq!(gen.format_params("  m=2  "), " m=2");
    }

    #[test]
    fn test_format_value_with_params_value_only() {
        let schematic = SchematicState::default();
        let gen = NetlistGenerator::new(&schematic);
        assert_eq!(gen.format_value_with_params("1k", ""), "1k");
    }

    #[test]
    fn test_format_value_with_params_both() {
        let schematic = SchematicState::default();
        let gen = NetlistGenerator::new(&schematic);
        assert_eq!(gen.format_value_with_params("1k", "m=2"), "1k m=2");
    }

    #[test]
    fn test_format_value_with_params_complex() {
        let schematic = SchematicState::default();
        let gen = NetlistGenerator::new(&schematic);
        assert_eq!(
            gen.format_value_with_params("4.7k", "m=2 tc1=0.01 tc2=0.001"),
            "4.7k m=2 tc1=0.01 tc2=0.001"
        );
    }

    // -------------------------------------------------------------------------
    // Passive Component Parameter Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_resistor_with_params() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let mut comp = Component::new(1, ComponentType::Resistor, Point::new(0, 0))
            .with_name_value("R1", "1k");
        comp.params = "m=2 tc1=0.01".to_string();

        let line = gen.generate_instance_line(&comp).unwrap();

        assert!(line.contains("R1"));
        assert!(line.contains("1k"));
        assert!(line.contains("m=2"));
        assert!(line.contains("tc1=0.01"));
    }

    #[test]
    fn test_resistor_without_params() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let comp = Component::new(1, ComponentType::Resistor, Point::new(0, 0))
            .with_name_value("R1", "1k");

        let line = gen.generate_instance_line(&comp).unwrap();

        assert!(line.contains("R1"));
        assert!(line.contains("1k"));
        // Should not have spurious whitespace at end
        assert!(!line.ends_with(' '));
    }

    #[test]
    fn test_capacitor_with_params() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let mut comp = Component::new(1, ComponentType::Capacitor, Point::new(0, 0))
            .with_name_value("C1", "100p");
        comp.params = "ic=0 m=4".to_string();

        let line = gen.generate_instance_line(&comp).unwrap();

        assert!(line.contains("C1"));
        assert!(line.contains("100p"));
        assert!(line.contains("ic=0"));
        assert!(line.contains("m=4"));
    }

    #[test]
    fn test_inductor_with_params() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let mut comp = Component::new(1, ComponentType::Inductor, Point::new(0, 0))
            .with_name_value("L1", "10u");
        comp.params = "ic=0 m=1".to_string();

        let line = gen.generate_instance_line(&comp).unwrap();

        assert!(line.contains("L1"));
        assert!(line.contains("10u"));
        assert!(line.contains("ic=0"));
    }

    #[test]
    fn test_diode_with_params() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let mut comp =
            Component::new(1, ComponentType::Diode, Point::new(0, 0)).with_name_value("D1", "1n");
        comp.params = "area=2 m=1".to_string();

        let line = gen.generate_instance_line(&comp).unwrap();

        assert!(line.contains("D1"));
        assert!(line.contains("area=2"));
    }

    // -------------------------------------------------------------------------
    // Source Parameter Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_voltage_source_dc_with_params() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let mut comp = Component::new(1, ComponentType::VoltageSource, Point::new(0, 0))
            .with_name_value("V1", "5");
        comp.params = "acmag=1 acphase=0".to_string();

        let line = gen.generate_instance_line(&comp).unwrap();

        assert!(line.contains("V1"));
        assert!(line.contains("DC 5"));
        assert!(line.contains("acmag=1"));
        assert!(line.contains("acphase=0"));
    }

    #[test]
    fn test_voltage_source_ac_with_params() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let mut comp = Component::new(1, ComponentType::VoltageSourceAc, Point::new(0, 0))
            .with_name_value("V2", "1");
        comp.params = "phase=45".to_string();

        let line = gen.generate_instance_line(&comp).unwrap();

        assert!(line.contains("V2"));
        assert!(line.contains("AC 1"));
        assert!(line.contains("phase=45"));
    }

    #[test]
    fn test_voltage_source_pulse_with_params() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        // Pulse sources use structured parameters for V1, V2, TD, TR, TF, PW, PER
        // The component.params contains key=value pairs for these parameters
        let mut comp = Component::new(1, ComponentType::VoltageSourcePulse, Point::new(0, 0))
            .with_name_value("V3", "");
        comp.params = "v1=0 v2=5 td=0 tr=1n tf=1n pw=10n period=20n".to_string();

        let line = gen.generate_instance_line(&comp).unwrap();

        assert!(line.contains("V3"));
        assert!(line.contains("PULSE("));
        // Check that structured parameters are parsed correctly
        assert!(line.contains("PULSE(0 5 0 1n 1n 10n 20n)"));
    }

    #[test]
    fn test_current_source_with_params() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let mut comp = Component::new(1, ComponentType::CurrentSource, Point::new(0, 0))
            .with_name_value("I1", "1m");
        comp.params = "acmag=100u".to_string();

        let line = gen.generate_instance_line(&comp).unwrap();

        assert!(line.contains("I1"));
        assert!(line.contains("DC 1m"));
        assert!(line.contains("acmag=100u"));
    }

    // -------------------------------------------------------------------------
    // Transistor Parameter Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_nmos_with_dimension_params() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let mut comp =
            Component::new(1, ComponentType::Nmos, Point::new(0, 0)).with_name_value("M1", "");
        comp.params = "w=1u l=180n as=1p ad=1p ps=2u pd=2u".to_string();

        let line = gen.generate_instance_line(&comp).unwrap();

        assert!(line.starts_with("M"));
        assert!(line.contains("M1"));
        assert!(line.contains("nmos"));
        assert!(line.contains("w=1u"));
        assert!(line.contains("l=180n"));
        assert!(line.contains("as=1p"));
        assert!(line.contains("ad=1p"));
    }

    #[test]
    fn test_pmos_with_dimension_params() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let mut comp =
            Component::new(1, ComponentType::Pmos, Point::new(0, 0)).with_name_value("M2", "");
        comp.params = "w=2u l=180n m=4".to_string();

        let line = gen.generate_instance_line(&comp).unwrap();

        assert!(line.contains("M2"));
        assert!(line.contains("pmos"));
        assert!(line.contains("w=2u"));
        assert!(line.contains("l=180n"));
        assert!(line.contains("m=4"));
    }

    #[test]
    fn test_npn_bjt_with_params() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let mut comp =
            Component::new(1, ComponentType::NpnBjt, Point::new(0, 0)).with_name_value("Q1", "");
        comp.params = "area=2 m=1".to_string();

        let line = gen.generate_instance_line(&comp).unwrap();

        assert!(line.starts_with("Q"));
        assert!(line.contains("Q1"));
        assert!(line.contains("npn"));
        assert!(line.contains("area=2"));
    }

    #[test]
    fn test_bjt_uses_value_as_explicit_model_name() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let mut comp = Component::new(1, ComponentType::NpnBjt, Point::new(0, 0))
            .with_name_value("Q1", "2N2222");
        comp.params = "area=2".to_string();

        let line = gen.generate_instance_line(&comp).unwrap();

        assert!(line.contains(" 2N2222 "));
        assert!(line.contains("area=2"));
        assert!(!line.contains("model="));
        assert!(!gen.models.contains_key("2N2222"));
    }

    #[test]
    fn test_bjt_uses_model_param_and_removes_duplicate_model_key() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let mut comp =
            Component::new(1, ComponentType::NpnBjt, Point::new(0, 0)).with_name_value("Q1", "");
        comp.params = "model=2N2222 area=2 m=1".to_string();

        let line = gen.generate_instance_line(&comp).unwrap();

        assert!(line.contains(" 2N2222 "));
        assert!(line.contains("area=2"));
        assert!(line.contains("m=1"));
        assert!(!line.contains("model=2N2222"));
        assert!(!gen.models.contains_key("2N2222"));
    }

    #[test]
    fn test_pnp_bjt_with_params() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let mut comp =
            Component::new(1, ComponentType::PnpBjt, Point::new(0, 0)).with_name_value("Q2", "");
        comp.params = "area=1.5".to_string();

        let line = gen.generate_instance_line(&comp).unwrap();

        assert!(line.contains("Q2"));
        assert!(line.contains("pnp"));
        assert!(line.contains("area=1.5"));
    }

    #[test]
    fn test_njfet_with_params() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let mut comp =
            Component::new(1, ComponentType::Njfet, Point::new(0, 0)).with_name_value("J1", "");
        comp.params = "area=1 m=2".to_string();

        let line = gen.generate_instance_line(&comp).unwrap();

        assert!(line.starts_with("J"));
        assert!(line.contains("J1"));
        assert!(line.contains("njf"));
        assert!(line.contains("area=1"));
        assert!(line.contains("m=2"));
    }

    // -------------------------------------------------------------------------
    // Controlled Source Parameter Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_vcvs_with_params() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let mut comp =
            Component::new(1, ComponentType::Vcvs, Point::new(0, 0)).with_name_value("E1", "10");
        comp.params = "max=5 min=-5".to_string();

        let line = gen.generate_instance_line(&comp).unwrap();

        assert!(line.starts_with("E"));
        assert!(line.contains("E1"));
        assert!(line.contains("10"));
        assert!(line.contains("max=5"));
        assert!(line.contains("min=-5"));
    }

    #[test]
    fn test_vccs_with_params() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let mut comp =
            Component::new(1, ComponentType::Vccs, Point::new(0, 0)).with_name_value("G1", "1m");
        comp.params = "ic=0".to_string();

        let line = gen.generate_instance_line(&comp).unwrap();

        assert!(line.starts_with("G"));
        assert!(line.contains("G1"));
        assert!(line.contains("1m"));
        assert!(line.contains("ic=0"));
    }

    #[test]
    fn test_ccvs_with_params() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let mut comp =
            Component::new(1, ComponentType::Ccvs, Point::new(0, 0)).with_name_value("H1", "1k");
        comp.params = "max=10".to_string();

        let line = gen.generate_instance_line(&comp).unwrap();

        assert!(line.starts_with("H"));
        assert!(line.contains("H1"));
        assert!(line.contains("1k"));
        assert!(line.contains("max=10"));
    }

    #[test]
    fn test_cccs_with_params() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let mut comp =
            Component::new(1, ComponentType::Cccs, Point::new(0, 0)).with_name_value("F1", "100");
        comp.params = "m=2".to_string();

        let line = gen.generate_instance_line(&comp).unwrap();

        assert!(line.starts_with("F"));
        assert!(line.contains("F1"));
        assert!(line.contains("100"));
        assert!(line.contains("m=2"));
    }

    // -------------------------------------------------------------------------
    // Edge Case Parameter Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_params_with_negative_values() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let mut comp = Component::new(1, ComponentType::Resistor, Point::new(0, 0))
            .with_name_value("R1", "1k");
        comp.params = "tc1=-0.01 tc2=-0.001".to_string();

        let line = gen.generate_instance_line(&comp).unwrap();

        assert!(line.contains("tc1=-0.01"));
        assert!(line.contains("tc2=-0.001"));
    }

    #[test]
    fn test_params_with_scientific_notation() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let mut comp =
            Component::new(1, ComponentType::Nmos, Point::new(0, 0)).with_name_value("M1", "");
        comp.params = "w=1e-6 l=1.8e-7".to_string();

        let line = gen.generate_instance_line(&comp).unwrap();

        assert!(line.contains("w=1e-6"));
        assert!(line.contains("l=1.8e-7"));
    }

    #[test]
    fn test_params_with_expressions() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let mut comp = Component::new(1, ComponentType::Resistor, Point::new(0, 0))
            .with_name_value("R1", "1k");
        // Spectre supports expressions in parameters
        comp.params = "m='2*scale'".to_string();

        let line = gen.generate_instance_line(&comp).unwrap();

        assert!(line.contains("m='2*scale'"));
    }

    #[test]
    fn test_params_preserves_order() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let mut comp =
            Component::new(1, ComponentType::Nmos, Point::new(0, 0)).with_name_value("M1", "");
        comp.params = "w=1u l=180n as=1p ad=1p".to_string();

        let line = gen.generate_instance_line(&comp).unwrap();

        // Parameters should maintain their order
        let w_pos = line.find("w=1u").unwrap();
        let l_pos = line.find("l=180n").unwrap();
        let as_pos = line.find("as=1p").unwrap();
        let ad_pos = line.find("ad=1p").unwrap();

        assert!(w_pos < l_pos);
        assert!(l_pos < as_pos);
        assert!(as_pos < ad_pos);
    }

    #[test]
    fn test_empty_params_no_trailing_space() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let comp = Component::new(1, ComponentType::Resistor, Point::new(0, 0))
            .with_name_value("R1", "1k");

        let line = gen.generate_instance_line(&comp).unwrap();

        // Line should not end with trailing space when params are empty
        assert!(!line.ends_with(' '));
        assert!(line.ends_with("1k") || line.ends_with("0 0 1k") || line.contains("1k"));
    }

    #[test]
    fn test_params_with_quoted_values() {
        let schematic = SchematicState::default();
        let mut gen = NetlistGenerator::new(&schematic);

        let mut comp = Component::new(1, ComponentType::Resistor, Point::new(0, 0))
            .with_name_value("R1", "1k");
        comp.params = "model=\"res_hi\"".to_string();

        let line = gen.generate_instance_line(&comp).unwrap();

        assert!(line.contains("model=\"res_hi\""));
    }
}
