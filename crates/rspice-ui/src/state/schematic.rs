//! Schematic State
//!
//! Data structures for the schematic capture editor.
//! Manages components, wires, selection, and interaction state.

use std::collections::HashMap;

/// Grid-aligned point (in grid units, not pixels)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentType {
    // Passive components
    Resistor,
    Capacitor,
    Inductor,

    // Semiconductors
    Diode,
    NpnBjt,
    PnpBjt,
    Nmos,
    Pmos,

    // Sources
    VoltageSource,
    CurrentSource,
    VoltageSourceAc,
    VoltageSourcePulse,
    VoltageSourceSin,

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
            ComponentType::Diode => "D",
            ComponentType::NpnBjt | ComponentType::PnpBjt => "Q",
            ComponentType::Nmos | ComponentType::Pmos => "M",
            ComponentType::VoltageSource
            | ComponentType::VoltageSourceAc
            | ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin => "V",
            ComponentType::CurrentSource => "I",
            ComponentType::Ground => "",
        }
    }

    /// Get the display name
    pub fn display_name(&self) -> &'static str {
        match self {
            ComponentType::Resistor => "Resistor",
            ComponentType::Capacitor => "Capacitor",
            ComponentType::Inductor => "Inductor",
            ComponentType::Diode => "Diode",
            ComponentType::NpnBjt => "NPN BJT",
            ComponentType::PnpBjt => "PNP BJT",
            ComponentType::Nmos => "NMOS",
            ComponentType::Pmos => "PMOS",
            ComponentType::VoltageSource => "V DC",
            ComponentType::CurrentSource => "I DC",
            ComponentType::VoltageSourceAc => "V AC",
            ComponentType::VoltageSourcePulse => "V Pulse",
            ComponentType::VoltageSourceSin => "V Sin",
            ComponentType::Ground => "Ground",
        }
    }

    /// Get the number of terminals
    pub fn terminal_count(&self) -> usize {
        match self {
            ComponentType::Ground => 1,
            ComponentType::NpnBjt | ComponentType::PnpBjt => 3,
            ComponentType::Nmos | ComponentType::Pmos => 4,
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
            ComponentType::Ground => vec![("", Point::new(0, 0))],
        }
    }
}

/// A placed component on the schematic
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone, Default)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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

/// Wire drawing state
#[derive(Debug, Clone, Default)]
pub struct WireDrawing {
    /// Points in the current wire being drawn
    pub points: Vec<Point>,

    /// Whether currently drawing
    pub active: bool,
}

/// Net label for naming nodes in the schematic
#[derive(Debug, Clone)]
pub struct NetLabel {
    /// Unique identifier
    pub id: u64,
    /// Position on grid
    pub pos: Point,
    /// Net name (e.g., "VCC", "GND", "OUT")
    pub name: String,
}

/// Clipboard data for copy/paste operations
#[derive(Debug, Clone, Default)]
pub struct ClipboardData {
    /// Copied components (stored with relative positions)
    pub components: Vec<Component>,
    /// Copied wires (stored with relative positions)
    pub wires: Vec<Wire>,
    /// Origin point (center of copied selection)
    pub origin: Point,
}

/// Main schematic state
#[derive(Debug, Clone)]
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

    /// Next component ID
    next_id: u64,

    /// Component counters for auto-naming (R1, R2, etc.)
    component_counters: HashMap<&'static str, u32>,

    /// Clipboard for copy/paste operations
    pub clipboard: ClipboardData,

    /// Net labels for naming nodes
    pub net_labels: Vec<NetLabel>,
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
            next_id: 1,
            component_counters: HashMap::new(),
            clipboard: ClipboardData::default(),
            net_labels: Vec::new(),
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

    /// Start drawing a wire at position
    pub fn start_wire(&mut self, pos: Point) {
        self.wire_drawing.points.clear();
        self.wire_drawing.points.push(pos);
        self.wire_drawing.active = true;
    }

    /// Add a point to the current wire
    pub fn extend_wire(&mut self, pos: Point) {
        if !self.wire_drawing.active {
            return;
        }
        if let Some(last) = self.wire_drawing.points.last() {
            if *last != pos {
                self.wire_drawing.points.push(pos);
            }
        }
    }

    /// Finish drawing the current wire
    pub fn finish_wire(&mut self) -> Option<u64> {
        if !self.wire_drawing.active {
            return None;
        }
        self.wire_drawing.active = false;
        let points = std::mem::take(&mut self.wire_drawing.points);
        self.add_wire(points)
    }

    /// Cancel wire drawing
    pub fn cancel_wire(&mut self) {
        self.wire_drawing.active = false;
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
}
