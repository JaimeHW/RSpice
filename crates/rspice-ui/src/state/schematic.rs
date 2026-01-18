//! Schematic State
//!
//! Data structures for the schematic capture editor.
//! Manages components, wires, selection, and interaction state.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Grid-aligned point (in grid units, not pixels)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Convert to pixel coordinates
    pub fn to_pixels(self, grid_size: i32) -> (f64, f64) {
        ((self.x * grid_size) as f64, (self.y * grid_size) as f64)
    }

    /// Create from pixel coordinates (snaps to grid)
    pub fn from_pixels(px: f64, py: f64, grid_size: i32) -> Self {
        Self {
            x: (px / grid_size as f64).round() as i32,
            y: (py / grid_size as f64).round() as i32,
        }
    }

    /// Get the 4 adjacent points
    pub fn neighbors(self) -> [Point; 4] {
        [
            Point::new(self.x - 1, self.y),
            Point::new(self.x + 1, self.y),
            Point::new(self.x, self.y - 1),
            Point::new(self.x, self.y + 1),
        ]
    }
}

/// Component rotation (clockwise)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Rotation {
    #[default]
    R0,
    R90,
    R180,
    R270,
}

impl Rotation {
    /// Rotate 90 degrees clockwise
    pub fn rotate_cw(self) -> Self {
        match self {
            Rotation::R0 => Rotation::R90,
            Rotation::R90 => Rotation::R180,
            Rotation::R180 => Rotation::R270,
            Rotation::R270 => Rotation::R0,
        }
    }

    /// Get rotation angle in degrees
    pub fn degrees(self) -> i32 {
        match self {
            Rotation::R0 => 0,
            Rotation::R90 => 90,
            Rotation::R180 => 180,
            Rotation::R270 => 270,
        }
    }
}

/// Component types available in the schematic
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentType {
    // Passive components
    Resistor,
    Capacitor,
    Inductor,
    CoupledInductor, // For transformers (K statement)

    // Semiconductors - Diodes
    Diode,

    // Semiconductors - Bipolar Transistors
    NpnBjt,
    PnpBjt,

    // Semiconductors - MOSFETs
    Nmos,
    Pmos,

    // Semiconductors - JFETs
    Njfet,
    Pjfet,

    // Independent Sources
    VoltageSource,
    CurrentSource,
    VoltageSourceAc,
    VoltageSourcePulse,
    VoltageSourceSin,

    // Controlled (Dependent) Sources
    Vcvs, // E - Voltage-Controlled Voltage Source
    Vccs, // G - Voltage-Controlled Current Source
    Ccvs, // H - Current-Controlled Voltage Source
    Cccs, // F - Current-Controlled Current Source

    // Special
    Ground,
}

impl ComponentType {
    /// Get the SPICE prefix for this component type
    pub fn spice_prefix(&self) -> &'static str {
        match self {
            ComponentType::Resistor => "R",
            ComponentType::Capacitor => "C",
            ComponentType::Inductor => "L",
            ComponentType::CoupledInductor => "K",
            ComponentType::Diode => "D",
            ComponentType::NpnBjt | ComponentType::PnpBjt => "Q",
            ComponentType::Nmos | ComponentType::Pmos => "M",
            ComponentType::Njfet | ComponentType::Pjfet => "J",
            ComponentType::VoltageSource
            | ComponentType::VoltageSourceAc
            | ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin => "V",
            ComponentType::CurrentSource => "I",
            ComponentType::Vcvs => "E",
            ComponentType::Vccs => "G",
            ComponentType::Ccvs => "H",
            ComponentType::Cccs => "F",
            ComponentType::Ground => "",
        }
    }

    /// Get the display name
    pub fn display_name(&self) -> &'static str {
        match self {
            ComponentType::Resistor => "Resistor",
            ComponentType::Capacitor => "Capacitor",
            ComponentType::Inductor => "Inductor",
            ComponentType::CoupledInductor => "Coupled Inductor",
            ComponentType::Diode => "Diode",
            ComponentType::NpnBjt => "NPN BJT",
            ComponentType::PnpBjt => "PNP BJT",
            ComponentType::Nmos => "NMOS",
            ComponentType::Pmos => "PMOS",
            ComponentType::Njfet => "N-JFET",
            ComponentType::Pjfet => "P-JFET",
            ComponentType::VoltageSource => "V DC",
            ComponentType::CurrentSource => "I DC",
            ComponentType::VoltageSourceAc => "V AC",
            ComponentType::VoltageSourcePulse => "V Pulse",
            ComponentType::VoltageSourceSin => "V Sin",
            ComponentType::Vcvs => "VCVS (E)",
            ComponentType::Vccs => "VCCS (G)",
            ComponentType::Ccvs => "CCVS (H)",
            ComponentType::Cccs => "CCCS (F)",
            ComponentType::Ground => "Ground",
        }
    }

    /// Get the number of terminals
    pub fn terminal_count(&self) -> usize {
        match self {
            ComponentType::Ground => 1,
            ComponentType::NpnBjt | ComponentType::PnpBjt => 3,
            ComponentType::Njfet | ComponentType::Pjfet => 3,
            ComponentType::Nmos | ComponentType::Pmos => 4,
            // Controlled sources have 4 terminals: output+, output-, control+, control-
            ComponentType::Vcvs
            | ComponentType::Vccs
            | ComponentType::Ccvs
            | ComponentType::Cccs => 4,
            // Coupled inductor is special - refers to two inductors
            ComponentType::CoupledInductor => 0,
            _ => 2, // Most components have 2 terminals
        }
    }

    /// Get terminal offsets relative to component position
    /// Returns (name, offset) pairs for each terminal
    pub fn terminal_offsets(&self) -> Vec<(&'static str, Point)> {
        match self {
            ComponentType::Resistor | ComponentType::Capacitor | ComponentType::Inductor => {
                vec![("+", Point::new(-2, 0)), ("-", Point::new(2, 0))]
            }
            ComponentType::Diode => vec![
                ("A", Point::new(-2, 0)), // Anode
                ("K", Point::new(2, 0)),  // Cathode
            ],
            ComponentType::VoltageSource
            | ComponentType::VoltageSourceAc
            | ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin => {
                vec![("+", Point::new(0, -2)), ("-", Point::new(0, 2))]
            }
            ComponentType::CurrentSource => vec![("+", Point::new(0, -2)), ("-", Point::new(0, 2))],
            ComponentType::NpnBjt => vec![
                ("B", Point::new(-2, 0)), // Base
                ("C", Point::new(1, -2)), // Collector
                ("E", Point::new(1, 2)),  // Emitter
            ],
            ComponentType::PnpBjt => vec![
                ("B", Point::new(-2, 0)),
                ("C", Point::new(1, 2)),
                ("E", Point::new(1, -2)),
            ],
            ComponentType::Nmos | ComponentType::Pmos => vec![
                ("G", Point::new(-2, 0)), // Gate
                ("D", Point::new(2, -1)), // Drain
                ("S", Point::new(2, 1)),  // Source
                ("B", Point::new(2, 0)),  // Bulk (usually tied to source)
            ],
            ComponentType::Njfet | ComponentType::Pjfet => vec![
                ("G", Point::new(-2, 0)), // Gate
                ("D", Point::new(2, -1)), // Drain
                ("S", Point::new(2, 1)),  // Source
            ],
            // Controlled sources: output on left, control on right
            ComponentType::Vcvs
            | ComponentType::Vccs
            | ComponentType::Ccvs
            | ComponentType::Cccs => vec![
                ("O+", Point::new(-2, -1)), // Output +
                ("O-", Point::new(-2, 1)),  // Output -
                ("C+", Point::new(2, -1)),  // Control +
                ("C-", Point::new(2, 1)),   // Control -
            ],
            // Coupled inductor doesn't have terminals (it's a coupling statement)
            ComponentType::CoupledInductor => vec![],
            ComponentType::Ground => vec![("GND", Point::new(0, -2))],
        }
    }
}

/// A placed component on the schematic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    /// Unique identifier
    pub id: u64,

    /// Component type
    pub kind: ComponentType,

    /// Position on grid
    pub pos: Point,

    /// Rotation
    pub rotation: Rotation,

    /// Component reference designator (e.g., "R1", "C2")
    pub name: String,

    /// Component value (e.g., "1k", "10u")
    pub value: String,

    /// Additional SPICE parameters
    pub params: String,
}

impl Component {
    /// Create a new component
    pub fn new(id: u64, kind: ComponentType, pos: Point) -> Self {
        Self {
            id,
            kind,
            pos,
            rotation: Rotation::default(),
            name: String::new(),
            value: String::new(),
            params: String::new(),
        }
    }

    /// Get terminal positions in world coordinates (accounting for rotation)
    pub fn terminal_positions(&self) -> Vec<(&'static str, Point)> {
        self.kind
            .terminal_offsets()
            .into_iter()
            .map(|(name, offset)| {
                let rotated = self.rotate_point(offset);
                (
                    name,
                    Point::new(self.pos.x + rotated.x, self.pos.y + rotated.y),
                )
            })
            .collect()
    }

    /// Rotate a point by the component's rotation
    fn rotate_point(&self, p: Point) -> Point {
        match self.rotation {
            Rotation::R0 => p,
            Rotation::R90 => Point::new(-p.y, p.x),
            Rotation::R180 => Point::new(-p.x, -p.y),
            Rotation::R270 => Point::new(p.y, -p.x),
        }
    }
}

/// A wire segment connecting two points
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wire {
    /// Unique identifier
    pub id: u64,

    /// Wire path (sequence of connected points)
    pub points: Vec<Point>,
}

impl Wire {
    /// Create a new wire
    pub fn new(id: u64, points: Vec<Point>) -> Self {
        Self { id, points }
    }

    /// Check if wire contains a point
    pub fn contains_point(&self, p: Point) -> bool {
        // Check vertices
        if self.points.contains(&p) {
            return true;
        }

        // Check segments
        for segment in self.points.windows(2) {
            if Self::point_on_segment(p, segment[0], segment[1]) {
                return true;
            }
        }
        false
    }

    /// Check if point lies on a horizontal or vertical segment
    fn point_on_segment(p: Point, a: Point, b: Point) -> bool {
        // Horizontal segment
        if a.y == b.y && p.y == a.y {
            let (min_x, max_x) = if a.x < b.x { (a.x, b.x) } else { (b.x, a.x) };
            return p.x >= min_x && p.x <= max_x;
        }
        // Vertical segment
        if a.x == b.x && p.x == a.x {
            let (min_y, max_y) = if a.y < b.y { (a.y, b.y) } else { (b.y, a.y) };
            return p.y >= min_y && p.y <= max_y;
        }
        false
    }
}

/// Selection state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Selection {
    /// Selected component IDs
    pub components: Vec<u64>,

    /// Selected wire IDs
    pub wires: Vec<u64>,
}

impl Selection {
    /// Check if selection is empty
    pub fn is_empty(&self) -> bool {
        self.components.is_empty() && self.wires.is_empty()
    }

    /// Clear selection
    pub fn clear(&mut self) {
        self.components.clear();
        self.wires.clear();
    }

    /// Check if a component is selected
    pub fn has_component(&self, id: u64) -> bool {
        self.components.contains(&id)
    }

    /// Check if a wire is selected
    pub fn has_wire(&self, id: u64) -> bool {
        self.wires.contains(&id)
    }
}

/// Current interaction tool
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Tool {
    /// Select and move components/wires
    #[default]
    Select,

    /// Draw wires
    Wire,

    /// Place a specific component type
    Place(ComponentType),

    /// Probe voltage/current at nodes
    Probe,

    /// Place net labels
    Label,
}

/// Wire routing mode for orthogonal drawing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum WireRoutingMode {
    /// Horizontal first, then vertical (L-shape)
    #[default]
    HorizontalFirst,
    /// Vertical first, then horizontal (inverted L-shape)
    VerticalFirst,
}

impl WireRoutingMode {
    /// Toggle between routing modes
    pub fn toggle(self) -> Self {
        match self {
            WireRoutingMode::HorizontalFirst => WireRoutingMode::VerticalFirst,
            WireRoutingMode::VerticalFirst => WireRoutingMode::HorizontalFirst,
        }
    }
}

/// Wire drawing state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WireDrawing {
    /// Points in the current wire being drawn (committed vertices)
    pub points: Vec<Point>,

    /// Whether currently drawing
    pub active: bool,

    /// Current mouse position for preview (grid-aligned)
    pub preview_pos: Option<Point>,

    /// Routing mode for orthogonal wires
    pub routing_mode: WireRoutingMode,
}

impl WireDrawing {
    /// Get intermediate points for orthogonal routing from last point to target
    /// Returns the corner point for L-shaped routing
    pub fn get_route_corner(&self, target: Point) -> Option<Point> {
        let last = self.points.last()?;
        if last.x == target.x || last.y == target.y {
            // Already aligned - no corner needed
            return None;
        }

        match self.routing_mode {
            WireRoutingMode::HorizontalFirst => {
                // Go horizontal first, then vertical
                Some(Point::new(target.x, last.y))
            }
            WireRoutingMode::VerticalFirst => {
                // Go vertical first, then horizontal
                Some(Point::new(last.x, target.y))
            }
        }
    }

    /// Get preview path from last committed point to mouse position
    pub fn get_preview_path(&self) -> Vec<Point> {
        let mut path = Vec::new();

        if let (Some(&last), Some(target)) = (self.points.last(), self.preview_pos) {
            path.push(last);

            if let Some(corner) = self.get_route_corner(target) {
                path.push(corner);
            }

            path.push(target);
        }

        path
    }
}

/// Net label for naming nodes in the schematic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetLabel {
    /// Unique identifier
    pub id: u64,
    /// Position on grid
    pub pos: Point,
    /// Net name (e.g., "VCC", "GND", "OUT")
    pub name: String,
}

/// Clipboard data for copy/paste operations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClipboardData {
    /// Copied components (stored with relative positions)
    pub components: Vec<Component>,
    /// Copied wires (stored with relative positions)
    pub wires: Vec<Wire>,
    /// Origin point (center of copied selection)
    pub origin: Point,
}

/// Represents a connection between a wire endpoint and a component terminal
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WireConnection {
    /// Wire ID
    pub wire_id: u64,
    /// Index in wire's points array (0 = start, len-1 = end)
    pub point_index: usize,
    /// Connected component ID
    pub component_id: u64,
    /// Terminal name ("+", "-", "C", "E", etc.)
    pub terminal_name: String,
}

/// Main schematic state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchematicState {
    /// All placed components
    pub components: Vec<Component>,

    /// All wires
    pub wires: Vec<Wire>,

    /// Current selection
    pub selection: Selection,

    /// Current tool
    pub tool: Tool,

    /// Wire drawing state
    pub wire_drawing: WireDrawing,

    /// Grid size in pixels
    pub grid_size: i32,

    /// Zoom level (1.0 = 100%)
    pub zoom: f64,

    /// Pan offset in pixels
    pub pan: (f64, f64),

    /// Current schematic file path (for save without dialog)
    #[serde(skip)]
    pub current_file: Option<std::path::PathBuf>,

    /// Next component ID (runtime state, not persisted)
    #[serde(skip)]
    next_id: u64,

    /// Component counters for auto-naming (runtime state, not persisted)
    #[serde(skip)]
    component_counters: HashMap<&'static str, u32>,

    /// Clipboard for copy/paste operations
    pub clipboard: ClipboardData,

    /// Net labels for naming nodes
    pub net_labels: Vec<NetLabel>,

    /// Preview rotation for component placement
    pub preview_rotation: Rotation,

    /// Wire-to-terminal connections (for rubber-banding)
    pub connections: Vec<WireConnection>,

    /// Cached point-to-net mapping from last netlist generation (for probe lookup)
    /// Updated after each simulation run
    #[serde(skip)]
    pub net_mapping: HashMap<Point, String>,
}

impl Default for SchematicState {
    fn default() -> Self {
        Self {
            components: Vec::new(),
            wires: Vec::new(),
            selection: Selection::default(),
            tool: Tool::default(),
            wire_drawing: WireDrawing::default(),
            grid_size: 10,
            zoom: 1.0,
            pan: (0.0, 0.0),
            current_file: None,
            next_id: 1,
            component_counters: HashMap::new(),
            clipboard: ClipboardData::default(),
            net_labels: Vec::new(),
            preview_rotation: Rotation::default(),
            connections: Vec::new(),
            net_mapping: HashMap::new(),
        }
    }
}

impl SchematicState {
    /// Generate a unique ID
    pub fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Recalculate runtime state after loading from file
    /// This MUST be called after deserialization to prevent ID collisions
    pub fn recalculate_runtime_state(&mut self) {
        // Find the maximum ID currently in use
        let max_component_id = self.components.iter().map(|c| c.id).max().unwrap_or(0);
        let max_wire_id = self.wires.iter().map(|w| w.id).max().unwrap_or(0);
        self.next_id = max_component_id.max(max_wire_id) + 1;

        // Rebuild component counters from existing component names
        self.component_counters.clear();
        for comp in &self.components {
            let prefix = comp.kind.spice_prefix();
            if !prefix.is_empty() {
                // Extract number from name like "R1", "C5", etc.
                if let Some(num_str) = comp.name.strip_prefix(prefix) {
                    if let Ok(num) = num_str.parse::<u32>() {
                        let counter = self.component_counters.entry(prefix).or_insert(0);
                        *counter = (*counter).max(num);
                    }
                }
            }
        }
    }

    /// Generate a unique component name
    pub fn generate_name(&mut self, kind: ComponentType) -> String {
        let prefix = kind.spice_prefix();
        if prefix.is_empty() {
            return String::new();
        }
        let counter = self.component_counters.entry(prefix).or_insert(0);
        *counter += 1;
        format!("{}{}", prefix, counter)
    }

    /// Add a component at the given position
    pub fn add_component(&mut self, kind: ComponentType, pos: Point) -> u64 {
        let id = self.next_id();
        let name = self.generate_name(kind);
        let mut component = Component::new(id, kind, pos);
        component.name = name;
        component.rotation = self.preview_rotation; // Apply preview rotation

        // Set default values
        component.value = match kind {
            ComponentType::Resistor => "1k".to_string(),
            ComponentType::Capacitor => "1u".to_string(),
            ComponentType::Inductor => "1m".to_string(),
            ComponentType::VoltageSource => "5".to_string(),
            ComponentType::CurrentSource => "1m".to_string(),
            _ => String::new(),
        };

        self.components.push(component);
        id
    }

    /// Add a wire
    pub fn add_wire(&mut self, points: Vec<Point>) -> Option<u64> {
        if points.len() < 2 {
            return None;
        }
        let id = self.next_id();
        self.wires.push(Wire::new(id, points));
        Some(id)
    }

    /// Add a net label at the given position
    pub fn add_net_label(&mut self, pos: Point, name: String) -> u64 {
        let id = self.next_id();
        self.net_labels.push(NetLabel { id, pos, name });
        id
    }

    /// Remove selected components and wires
    pub fn delete_selection(&mut self) {
        self.components
            .retain(|c| !self.selection.has_component(c.id));
        self.wires.retain(|w| !self.selection.has_wire(w.id));
        self.selection.clear();
    }

    /// Rotate selected components
    pub fn rotate_selection(&mut self) {
        for id in &self.selection.components {
            if let Some(c) = self.components.iter_mut().find(|c| c.id == *id) {
                c.rotation = c.rotation.rotate_cw();
            }
        }
    }

    /// Find component at grid position
    pub fn component_at(&self, pos: Point) -> Option<u64> {
        // Check terminals and component bounds
        for comp in &self.components {
            let terminals = comp.terminal_positions();
            for (_, term_pos) in terminals {
                if term_pos == pos {
                    return Some(comp.id);
                }
            }
            // Check if within component bounding box (simplified)
            let dx = (pos.x - comp.pos.x).abs();
            let dy = (pos.y - comp.pos.y).abs();
            if dx <= 2 && dy <= 2 {
                return Some(comp.id);
            }
        }
        None
    }

    /// Find wire at grid position
    pub fn wire_at(&self, pos: Point) -> Option<u64> {
        for wire in &self.wires {
            if wire.contains_point(pos) {
                return Some(wire.id);
            }
        }
        None
    }

    /// Find all wire points at a grid position
    /// Returns (wire_id, point_index) pairs for junction detection
    pub fn wire_points_at(&self, pos: Point) -> Vec<(u64, usize)> {
        let mut result = Vec::new();
        for wire in &self.wires {
            for (idx, point) in wire.points.iter().enumerate() {
                if *point == pos {
                    result.push((wire.id, idx));
                }
            }
        }
        result
    }

    /// Find all wire ENDPOINTS at a grid position
    /// Unlike wire_points_at, this only returns first/last points of wires
    /// Used for proper junction detection (junctions are where wire endpoints meet)
    pub fn wire_endpoints_at(&self, pos: Point) -> Vec<(u64, usize)> {
        let mut result = Vec::new();
        for wire in &self.wires {
            if let Some(first) = wire.points.first() {
                if *first == pos {
                    result.push((wire.id, 0));
                }
            }
            if wire.points.len() > 1 {
                if let Some(last) = wire.points.last() {
                    if *last == pos {
                        result.push((wire.id, wire.points.len() - 1));
                    }
                }
            }
        }
        result
    }

    /// Start drawing a wire at position
    pub fn start_wire(&mut self, pos: Point) {
        log::info!("[Wire] start_wire at {:?}", pos);
        self.wire_drawing.points.clear();
        self.wire_drawing.points.push(pos);
        self.wire_drawing.preview_pos = None;
        self.wire_drawing.active = true;
    }

    /// Update the wire preview position (called on mouse move)
    pub fn update_wire_preview(&mut self, pos: Point) {
        if self.wire_drawing.active {
            self.wire_drawing.preview_pos = Some(pos);
        }
    }

    /// Toggle wire routing mode (horizontal-first vs vertical-first)
    pub fn toggle_wire_routing(&mut self) {
        self.wire_drawing.routing_mode = self.wire_drawing.routing_mode.toggle();
    }

    /// Add a point to the current wire using orthogonal routing
    /// This commits the current preview path segment
    pub fn extend_wire(&mut self, pos: Point) {
        log::info!(
            "[Wire] extend_wire called with pos {:?}, active={}, points={:?}",
            pos,
            self.wire_drawing.active,
            self.wire_drawing.points
        );
        if !self.wire_drawing.active {
            log::warn!("[Wire] extend_wire: not active, returning");
            return;
        }

        if let Some(last) = self.wire_drawing.points.last().copied() {
            if last == pos {
                log::info!("[Wire] extend_wire: same point, skipping");
                return; // Same point, skip
            }

            // Add corner point for orthogonal routing if needed
            if let Some(corner) = self.wire_drawing.get_route_corner(pos) {
                // Only add corner if it's different from last and target
                if corner != last && corner != pos {
                    log::info!("[Wire] extend_wire: adding corner {:?}", corner);
                    self.wire_drawing.points.push(corner);
                }
            }

            log::info!("[Wire] extend_wire: adding point {:?}", pos);
            self.wire_drawing.points.push(pos);
        }
        log::info!(
            "[Wire] extend_wire: points now {:?}",
            self.wire_drawing.points
        );
    }

    /// Finish drawing the current wire
    pub fn finish_wire(&mut self) -> Option<u64> {
        log::info!(
            "[Wire] finish_wire called, active={}, points={:?}",
            self.wire_drawing.active,
            self.wire_drawing.points
        );
        if !self.wire_drawing.active {
            log::warn!("[Wire] finish_wire: not active, returning None");
            return None;
        }
        self.wire_drawing.active = false;
        self.wire_drawing.preview_pos = None;

        // Get the committed points
        let points = std::mem::take(&mut self.wire_drawing.points);
        log::info!("[Wire] finish_wire: raw points = {:?}", points);

        // Simplify wire by removing collinear points
        let simplified = Self::simplify_wire_path(points);
        log::info!("[Wire] finish_wire: simplified points = {:?}", simplified);

        if simplified.len() < 2 {
            log::warn!("[Wire] finish_wire: too few points, returning None");
            return None;
        }

        // Split the path into individual 2-point wire segments
        // This matches professional simulator behavior (LTspice) where each wire
        // is a single straight line between two endpoints
        let mut last_wire_id = None;
        for i in 0..simplified.len() - 1 {
            let segment = vec![simplified[i], simplified[i + 1]];
            log::info!("[Wire] finish_wire: adding segment {:?}", segment);
            if let Some(wire_id) = self.add_wire(segment) {
                // Snap endpoints to nearby terminals and register connections
                self.snap_wire_to_terminals(wire_id);
                last_wire_id = Some(wire_id);
            }
        }

        log::info!(
            "[Wire] finish_wire: created wire(s), last_id={:?}",
            last_wire_id
        );
        last_wire_id
    }

    /// Simplify wire path by removing intermediate points on straight segments
    fn simplify_wire_path(points: Vec<Point>) -> Vec<Point> {
        if points.len() <= 2 {
            return points;
        }

        let mut result = Vec::with_capacity(points.len());
        result.push(points[0]);

        for i in 1..points.len() - 1 {
            let prev = result.last().unwrap();
            let curr = &points[i];
            let next = &points[i + 1];

            // Check if curr is collinear with prev and next
            let same_x = prev.x == curr.x && curr.x == next.x;
            let same_y = prev.y == curr.y && curr.y == next.y;

            // Only keep if not collinear (it's a corner)
            if !same_x && !same_y {
                result.push(*curr);
            }
        }

        result.push(*points.last().unwrap());
        result
    }

    /// Cancel wire drawing
    pub fn cancel_wire(&mut self) {
        self.wire_drawing.active = false;
        self.wire_drawing.preview_pos = None;
        self.wire_drawing.points.clear();
    }

    /// Copy selected components and wires to clipboard
    pub fn copy_selection(&mut self) {
        if self.selection.is_empty() {
            return;
        }

        // Get selected components
        let selected_comps: Vec<Component> = self
            .components
            .iter()
            .filter(|c| self.selection.has_component(c.id))
            .cloned()
            .collect();

        // Get selected wires
        let selected_wires: Vec<Wire> = self
            .wires
            .iter()
            .filter(|w| self.selection.has_wire(w.id))
            .cloned()
            .collect();

        // Calculate center of selection
        let mut cx = 0i32;
        let mut cy = 0i32;
        let mut count = 0;
        for comp in &selected_comps {
            cx += comp.pos.x;
            cy += comp.pos.y;
            count += 1;
        }
        for wire in &selected_wires {
            if let Some(first) = wire.points.first() {
                cx += first.x;
                cy += first.y;
                count += 1;
            }
        }
        let origin = if count > 0 {
            Point::new(cx / count, cy / count)
        } else {
            Point::new(0, 0)
        };

        self.clipboard = ClipboardData {
            components: selected_comps,
            wires: selected_wires,
            origin,
        };
    }

    /// Check if clipboard has content
    pub fn can_paste(&self) -> bool {
        !self.clipboard.components.is_empty() || !self.clipboard.wires.is_empty()
    }

    /// Paste clipboard contents at the given position
    pub fn paste_at(&mut self, pos: Point) {
        if !self.can_paste() {
            return;
        }

        // Clone clipboard data to avoid borrow issues
        let clipboard_components = self.clipboard.components.clone();
        let clipboard_wires = self.clipboard.wires.clone();
        let origin = self.clipboard.origin;

        // Calculate offset from clipboard origin to paste position
        let offset_x = pos.x - origin.x;
        let offset_y = pos.y - origin.y;

        // Clear current selection
        self.selection.clear();

        // Paste components with new IDs
        for comp in clipboard_components {
            let new_id = self.next_id();
            let mut new_comp = comp;
            new_comp.id = new_id;
            new_comp.pos.x += offset_x;
            new_comp.pos.y += offset_y;
            // Generate new name for pasted component
            new_comp.name = self.generate_name(new_comp.kind);
            self.components.push(new_comp);
            self.selection.components.push(new_id);
        }

        // Paste wires with new IDs
        for wire in clipboard_wires {
            let new_id = self.next_id();
            let new_points: Vec<Point> = wire
                .points
                .iter()
                .map(|p| Point::new(p.x + offset_x, p.y + offset_y))
                .collect();
            let new_wire = Wire {
                id: new_id,
                points: new_points,
            };
            self.wires.push(new_wire);
            self.selection.wires.push(new_id);
        }
    }

    // =========================================================================
    // Wire Connection Management (for rubber-banding)
    // =========================================================================

    /// Snap distance in grid units for terminal connections
    const SNAP_DISTANCE: i32 = 1;

    /// Find a component terminal at or near a grid position
    /// Returns (component_id, terminal_name, terminal_position) if found
    pub fn find_terminal_at(&self, pos: Point) -> Option<(u64, String, Point)> {
        for comp in &self.components {
            for (term_name, term_pos) in comp.terminal_positions() {
                let dx = (pos.x - term_pos.x).abs();
                let dy = (pos.y - term_pos.y).abs();
                if dx <= Self::SNAP_DISTANCE && dy <= Self::SNAP_DISTANCE {
                    return Some((comp.id, term_name.to_string(), term_pos));
                }
            }
        }
        None
    }

    /// Rebuild all wire connections based on current positions
    /// Called after any operation that might affect connections
    pub fn rebuild_connections(&mut self) {
        self.connections.clear();

        // Phase 1: Collect all wire endpoint data (immutable borrow of self.wires)
        let wire_endpoints: Vec<(u64, Point, usize)> = self
            .wires
            .iter()
            .filter(|w| !w.points.is_empty())
            .flat_map(|w| {
                let mut endpoints = vec![(w.id, w.points[0], 0usize)];
                let end_idx = w.points.len() - 1;
                if end_idx > 0 {
                    endpoints.push((w.id, w.points[end_idx], end_idx));
                }
                endpoints
            })
            .collect();

        // Phase 2: Find terminal connections for each endpoint
        for (wire_id, pos, point_index) in wire_endpoints {
            if let Some((comp_id, term_name, _)) = self.find_terminal_at(pos) {
                self.connections.push(WireConnection {
                    wire_id,
                    point_index,
                    component_id: comp_id,
                    terminal_name: term_name,
                });
            }
        }
    }

    /// Find all connections for a specific component
    pub fn connections_for_component(&self, component_id: u64) -> Vec<&WireConnection> {
        self.connections
            .iter()
            .filter(|c| c.component_id == component_id)
            .collect()
    }

    /// Move a component and update all attached wire endpoints (rubber-banding)
    ///
    /// This uses junction-aware moving like professional simulators:
    /// - Find all terminal positions before moving
    /// - Find ALL wire points (from any wire) at those positions  
    /// - Move ALL those points together, preserving junctions
    pub fn move_component_with_wires(&mut self, component_id: u64, delta: Point) {
        // Get the component's terminal positions BEFORE moving
        let terminals: Vec<Point> = {
            if let Some(comp) = self.components.iter().find(|c| c.id == component_id) {
                comp.terminal_positions()
                    .into_iter()
                    .map(|(_, pos)| pos)
                    .collect()
            } else {
                return;
            }
        };

        // Find ALL wire points that are at ANY terminal position
        // This includes junction points where multiple wires meet
        let mut wire_updates: Vec<(u64, usize, Point)> = Vec::new();

        for wire in &self.wires {
            for (point_idx, point) in wire.points.iter().enumerate() {
                for term_pos in &terminals {
                    if *point == *term_pos {
                        // This wire point is at a terminal - it needs to move
                        let new_pos = Point::new(term_pos.x + delta.x, term_pos.y + delta.y);
                        wire_updates.push((wire.id, point_idx, new_pos));
                        break; // Only match one terminal per point
                    }
                }
            }
        }

        // Move the component
        if let Some(comp) = self.components.iter_mut().find(|c| c.id == component_id) {
            comp.pos.x += delta.x;
            comp.pos.y += delta.y;
        }

        // Apply wire updates
        for (wire_id, point_idx, new_pos) in wire_updates {
            if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
                if point_idx < wire.points.len() {
                    wire.points[point_idx] = new_pos;
                }
            }
        }
    }

    /// Move all points of a wire by a delta
    pub fn move_wire(&mut self, wire_id: u64, delta: Point) {
        if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
            for point in &mut wire.points {
                point.x += delta.x;
                point.y += delta.y;
            }
        }
    }

    /// Move all selected components and wires by a delta
    ///
    /// This implements professional multi-selection movement:
    /// - Components are moved with rubber-banding for attached wires NOT in selection
    /// - Wires in selection are moved entirely  
    /// - Avoids double-moving wires that are both selected AND attached to selected components
    pub fn move_selection(&mut self, delta: Point) {
        let selection = self.selection.clone();

        // Track which wires are explicitly selected (to avoid double-moving)
        let selected_wire_ids: std::collections::HashSet<u64> =
            selection.wires.iter().copied().collect();

        // Move all selected components with rubber-banding
        // (but only rubber-band to wires NOT in selection)
        for comp_id in &selection.components {
            // Get terminal positions before moving
            let terminals: Vec<Point> = {
                if let Some(comp) = self.components.iter().find(|c| c.id == *comp_id) {
                    comp.terminal_positions()
                        .into_iter()
                        .map(|(_, pos)| pos)
                        .collect()
                } else {
                    continue;
                }
            };

            // Find wire points to update (only for wires NOT in selection)
            let mut wire_updates: Vec<(u64, usize, Point)> = Vec::new();
            for wire in &self.wires {
                if selected_wire_ids.contains(&wire.id) {
                    continue; // Skip - this wire will be moved entirely
                }
                for (point_idx, point) in wire.points.iter().enumerate() {
                    for term_pos in &terminals {
                        if *point == *term_pos {
                            let new_pos = Point::new(term_pos.x + delta.x, term_pos.y + delta.y);
                            wire_updates.push((wire.id, point_idx, new_pos));
                            break;
                        }
                    }
                }
            }

            // Move the component
            if let Some(comp) = self.components.iter_mut().find(|c| c.id == *comp_id) {
                comp.pos.x += delta.x;
                comp.pos.y += delta.y;
            }

            // Apply wire updates
            for (wire_id, point_idx, new_pos) in wire_updates {
                if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
                    if point_idx < wire.points.len() {
                        wire.points[point_idx] = new_pos;
                    }
                }
            }
        }

        // Move all selected wires entirely
        for wire_id in &selection.wires {
            if let Some(wire) = self.wires.iter_mut().find(|w| w.id == *wire_id) {
                for point in &mut wire.points {
                    point.x += delta.x;
                    point.y += delta.y;
                }
            }
        }
    }

    /// Move all wire points at a junction to a new position
    /// This enables junction stretching - dragging a junction moves all connected wire endpoints
    pub fn move_junction(&mut self, old_pos: Point, new_pos: Point) {
        for wire in &mut self.wires {
            for point in &mut wire.points {
                if *point == old_pos {
                    *point = new_pos;
                }
            }
        }
    }

    /// Get wire points adjusted for a component drag preview
    ///
    /// Returns the wire's points as they would appear if the component were at the
    /// preview position. Used for live wire updates during drag.
    ///
    /// Uses stretch behavior like professional simulators:
    /// - Only endpoints directly at terminals move
    /// - Rest of wire stays anchored (may create diagonal segments)
    pub fn get_wire_preview_points(
        &self,
        wire: &Wire,
        dragging_component_id: Option<u64>,
        delta: Point,
    ) -> Vec<Point> {
        let mut points = wire.points.clone();

        let comp_id = match dragging_component_id {
            Some(id) => id,
            None => return points,
        };

        // Get the component's current terminal positions
        let terminals: Vec<Point> =
            if let Some(comp) = self.components.iter().find(|c| c.id == comp_id) {
                comp.terminal_positions()
                    .into_iter()
                    .map(|(_, pos)| pos)
                    .collect()
            } else {
                return points;
            };

        // Move any wire point that's at a terminal position
        // (only that specific point moves, rest of wire stays anchored)
        for (_point_idx, point) in points.iter_mut().enumerate() {
            for term_pos in &terminals {
                if *point == *term_pos {
                    *point = Point::new(term_pos.x + delta.x, term_pos.y + delta.y);
                    break;
                }
            }
        }

        points
    }

    /// Snap wire endpoints to nearby terminals and rebuild connections
    /// Called when finishing wire placement
    pub fn snap_wire_to_terminals(&mut self, wire_id: u64) {
        // First, get the wire's current endpoints (read-only)
        let (start_pos, end_pos, end_idx) = {
            if let Some(wire) = self.wires.iter().find(|w| w.id == wire_id) {
                if wire.points.is_empty() {
                    return;
                }
                let end_idx = wire.points.len().saturating_sub(1);
                (
                    Some(wire.points[0]),
                    if end_idx > 0 {
                        Some(wire.points[end_idx])
                    } else {
                        None
                    },
                    end_idx,
                )
            } else {
                return;
            }
        };

        // Find snap targets for each endpoint
        let snap_start = start_pos.and_then(|p| self.find_terminal_at(p).map(|(_, _, pos)| pos));
        let snap_end = end_pos.and_then(|p| self.find_terminal_at(p).map(|(_, _, pos)| pos));

        // Now apply the snaps (mutable borrow)
        if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
            if let Some(term_pos) = snap_start {
                wire.points[0] = term_pos;
            }
            if let Some(term_pos) = snap_end {
                if end_idx > 0 {
                    wire.points[end_idx] = term_pos;
                }
            }
        }

        // Rebuild connections to register the new connections
        self.rebuild_connections();
    }
}
