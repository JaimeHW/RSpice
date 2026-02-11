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
        warnings: gen.warnings().to_vec(),
        errors: gen.errors().to_vec(),
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
    /// Netlist generation warnings surfaced to simulation controller.
    warnings: Vec<String>,
    /// Netlist generation errors that should block simulation.
    errors: Vec<String>,
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
            warnings: Vec::new(),
            errors: Vec::new(),
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
        self.warnings.clear();
        self.errors.clear();
    }

    /// Read-only warnings collected during generation.
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    /// Read-only errors collected during generation.
    pub fn errors(&self) -> &[String] {
        &self.errors
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

        // Phase 4: Generate include directives for library-bound instances
        self.generate_library_view_includes();

        // Phase 5: Generate component instances
        self.generate_instances();

        // Phase 6: Add models if needed
        self.generate_models();

        // Phase 7: Add analysis commands (if requested)
        if !analysis_lines.is_empty() {
            self.lines.push(String::new());
            self.lines.push("* Analysis commands".to_string());
            for line in analysis_lines {
                self.lines.push(line.clone());
            }
        }

        // Phase 8: End statement
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

    /// Emit include directives for placed library cell instances.
    ///
    /// - Verilog-A bindings emit `.VERILOGA "path" [module]`
    /// - Other source-backed bindings emit `.include "path"`
    fn generate_library_view_includes(&mut self) {
        let mut includes = std::collections::BTreeMap::<String, Option<String>>::new();
        let mut generic_includes = std::collections::BTreeSet::<String>::new();

        for component in &self.schematic.components {
            let Some(binding) = component.library_cell.as_ref() else {
                continue;
            };

            let Some(source_path) = binding.source_path.as_ref() else {
                continue;
            };

            let key = source_path.to_string_lossy().to_string();
            if binding.view.eq_ignore_ascii_case("veriloga") {
                let model = binding
                    .module_name
                    .as_ref()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty());
                if model.is_none() {
                    self.warnings.push(format!(
                        "Cell instance '{}' ({}/{}/{}) has no explicit Verilog-A module name; falling back to cell name during netlisting",
                        component.name, binding.library, binding.cell, binding.view
                    ));
                }
                if let Some(existing_model) = includes.get(&key) {
                    if existing_model.as_deref() != model.as_deref() {
                        self.warnings.push(format!(
                            "Conflicting Verilog-A module bindings for include '{}': keeping '{}' and ignoring '{}'",
                            key,
                            existing_model.as_deref().unwrap_or("<none>"),
                            model.as_deref().unwrap_or("<none>")
                        ));
                    }
                } else {
                    includes.insert(key, model);
                }
            } else {
                generic_includes.insert(key);
            }
        }

        if includes.is_empty() && generic_includes.is_empty() {
            return;
        }

        self.lines.push("* Library includes".to_string());
        for path in generic_includes {
            let quoted_path = Self::quote_path_for_netlist(&path);
            self.lines.push(format!(".include {}", quoted_path));
        }
        for (path, model) in includes {
            let quoted_path = Self::quote_path_for_netlist(&path);
            if let Some(model_name) = model {
                self.lines
                    .push(format!(".VERILOGA {} {}", quoted_path, model_name));
            } else {
                self.lines.push(format!(".VERILOGA {}", quoted_path));
            }
        }
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

            // Generic library/cell/view instance.
            // Emits a standard X-instance referring to the bound master name
            // (Verilog-A module or subcircuit/cell fallback).
            ComponentType::CellInstance => {
                let Some(binding) = component.library_cell.as_ref() else {
                    self.errors.push(format!(
                        "Cell instance '{}' is missing library binding metadata",
                        component.name
                    ));
                    return None;
                };

                if binding.terminal_order.is_empty() {
                    self.errors.push(format!(
                        "Cell instance '{}' ({}/{}/{}) is missing terminal order metadata (netlist.ports/netlist.terminals)",
                        component.name, binding.library, binding.cell, binding.view
                    ));
                    return None;
                }
                if node_names.len() != binding.terminal_order.len() {
                    self.errors.push(format!(
                        "Cell instance '{}' ({}/{}/{}) terminal mismatch: schematic has {} nodes but binding defines {} terminals",
                        component.name,
                        binding.library,
                        binding.cell,
                        binding.view,
                        node_names.len(),
                        binding.terminal_order.len()
                    ));
                    return None;
                }
                if binding.source_path.is_none() {
                    self.errors.push(format!(
                        "Cell instance '{}' ({}/{}/{}) is missing source path metadata",
                        component.name, binding.library, binding.cell, binding.view
                    ));
                    return None;
                }

                let subckt_name = binding
                    .module_name
                    .as_ref()
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .unwrap_or(binding.cell.as_str());
                if subckt_name.is_empty() {
                    self.errors.push(format!(
                        "Cell instance '{}' ({}/{}/{}) has no netlist master/module name",
                        component.name, binding.library, binding.cell, binding.view
                    ));
                    return None;
                }

                let mut instance_name = component.spice_instance_name();
                if !instance_name.starts_with('X') && !instance_name.starts_with('x') {
                    instance_name = format!("X{}", instance_name);
                }

                let nodes = node_names.join(" ");
                let params = self.format_params(&component.params);
                Some(format!(
                    "{} {} {}{}",
                    instance_name, nodes, subckt_name, params
                ))
            }

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

    fn quote_path_for_netlist(path: &str) -> String {
        let escaped = path.replace('"', "\\\"");
        format!("\"{}\"", escaped)
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
mod tests;
