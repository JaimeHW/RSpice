//! Component Struct
//!
//! A placed component on the schematic with position, rotation, and properties.

use super::component_type::ComponentType;
use super::point::{LabelPosition, Point};
use super::rotation::Rotation;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// =============================================================================
// Component
// =============================================================================

/// Library/cell/view binding for a generic hierarchical instance.
///
/// This mirrors commercial LCV instance binding where a placed instance points
/// to a library master and selected view.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LibraryCellInstance {
    /// Source library name.
    pub library: String,
    /// Source cell name.
    pub cell: String,
    /// Bound view name (for example: "schematic", "symbol", "veriloga").
    pub view: String,
    /// Optional Verilog-A source file path for netlisting.
    #[serde(default)]
    pub source_path: Option<PathBuf>,
    /// Optional master name override used during netlisting
    /// (for example: Verilog-A module or subcircuit name).
    #[serde(default)]
    pub module_name: Option<String>,
    /// Terminal order used for schematic connectivity and netlist emission.
    #[serde(default)]
    pub terminal_order: Vec<String>,
}

impl LibraryCellInstance {
    /// Create a new library/cell/view binding.
    pub fn new(
        library: impl Into<String>,
        cell: impl Into<String>,
        view: impl Into<String>,
    ) -> Self {
        Self {
            library: library.into(),
            cell: cell.into(),
            view: view.into(),
            source_path: None,
            module_name: None,
            terminal_order: Vec::new(),
        }
    }
}

/// A placed component on the schematic
///
/// Components are circuit elements placed on the schematic canvas.
/// Each component has a position, rotation, reference designator (name),
/// value, and optional parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    /// Unique identifier within the schematic
    pub id: u64,

    /// Component type (resistor, capacitor, etc.)
    pub kind: ComponentType,

    /// Position on grid (in grid units, not pixels)
    pub pos: Point,

    /// Rotation (0, 90, 180, or 270 degrees)
    pub rotation: Rotation,

    /// Component reference designator (e.g., "R1", "C2", "U3")
    pub name: String,

    /// Component value (e.g., "1k", "10u", "LM741")
    pub value: String,

    /// Additional SPICE parameters (e.g., "TC=0.01,0.001")
    pub params: String,

    /// Optional schematic symbol variant identifier.
    ///
    /// This is a UI-only appearance choice and is never emitted into the
    /// generated SPICE netlist.
    #[serde(default)]
    pub symbol_variant: Option<String>,

    /// Name label position (Auto for smart placement, Custom for user-defined)
    #[serde(default)]
    pub name_label_pos: LabelPosition,

    /// Value label position (Auto for smart placement, Custom for user-defined)
    #[serde(default)]
    pub value_label_pos: LabelPosition,

    /// Horizontal mirror (flip left/right)
    ///
    /// When true, the component is mirrored horizontally (about Y-axis).
    /// This is essential for proper transistor orientation (e.g., NMOS with
    /// drain on left vs right). Matches Cadence Virtuoso 'H' key behavior.
    #[serde(default)]
    pub mirror_h: bool,

    /// Vertical mirror (flip up/down)
    ///
    /// When true, the component is mirrored vertically (about X-axis).
    /// Matches Cadence Virtuoso 'V' key behavior.
    #[serde(default)]
    pub mirror_v: bool,

    /// Optional bound library/cell/view metadata for hierarchical instances.
    ///
    /// When present, this component is a generic cell instance (X element style)
    /// and uses dynamic terminal layout derived from `terminal_order`.
    #[serde(default)]
    pub library_cell: Option<LibraryCellInstance>,
}

impl Component {
    /// Create a new component with default values
    ///
    /// # Arguments
    /// * `id` - Unique identifier
    /// * `kind` - Component type
    /// * `pos` - Position on the grid
    pub fn new(id: u64, kind: ComponentType, pos: Point) -> Self {
        Self {
            id,
            kind,
            pos,
            rotation: Rotation::default(),
            name: String::new(),
            value: String::new(),
            params: String::new(),
            symbol_variant: None,
            name_label_pos: LabelPosition::Auto,
            value_label_pos: LabelPosition::Auto,
            mirror_h: false,
            mirror_v: false,
            library_cell: None,
        }
    }

    /// Create a component with name and value
    pub fn with_name_value(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.name = name.into();
        self.value = value.into();
        self
    }

    /// Create a component with a specific schematic symbol variant.
    pub fn with_symbol_variant(mut self, symbol_variant: impl Into<String>) -> Self {
        self.symbol_variant = Some(symbol_variant.into());
        self
    }

    /// Create a component with rotation
    pub fn with_rotation(mut self, rotation: Rotation) -> Self {
        self.rotation = rotation;
        self
    }

    /// Create a component with horizontal mirror
    ///
    /// Horizontal mirror flips the component about the Y-axis (left/right swap).
    pub fn with_mirror_h(mut self, mirror_h: bool) -> Self {
        self.mirror_h = mirror_h;
        self
    }

    /// Create a component with vertical mirror
    ///
    /// Vertical mirror flips the component about the X-axis (up/down swap).
    pub fn with_mirror_v(mut self, mirror_v: bool) -> Self {
        self.mirror_v = mirror_v;
        self
    }

    /// Attach a library/cell/view binding to this instance.
    pub fn with_library_cell(mut self, library_cell: LibraryCellInstance) -> Self {
        self.library_cell = Some(library_cell);
        self
    }

    /// Toggle horizontal mirror
    pub fn toggle_mirror_h(&mut self) {
        self.mirror_h = !self.mirror_h;
    }

    /// Toggle vertical mirror
    pub fn toggle_mirror_v(&mut self) {
        self.mirror_v = !self.mirror_v;
    }

    /// Get terminal positions in world coordinates
    ///
    /// Returns terminal positions accounting for component position, rotation, and mirror.
    /// Each terminal is identified by name (e.g., "+", "-", "B", "C", "E").
    ///
    /// This is used for:
    /// - Wire snapping to terminals
    /// - Netlist generation
    /// - Rubber-banding during component moves
    pub fn terminal_positions(&self) -> Vec<(&'static str, Point)> {
        if let Some(cell) = &self.library_cell {
            let pin_count = if cell.terminal_order.is_empty() {
                self.kind.terminal_count()
            } else {
                cell.terminal_order.len()
            };
            return self.dynamic_terminal_positions(pin_count);
        }

        // Terminal offsets are component-type specific and defined in ComponentType
        self.kind
            .terminal_offsets()
            .into_iter()
            .map(|(name, offset)| {
                let transformed = self.transform_point(offset);
                (
                    name,
                    Point::new(self.pos.x + transformed.x, self.pos.y + transformed.y),
                )
            })
            .collect()
    }

    fn dynamic_terminal_positions(&self, pin_count: usize) -> Vec<(&'static str, Point)> {
        let offsets = self.dynamic_terminal_offsets(pin_count);
        offsets
            .into_iter()
            .enumerate()
            .map(|(idx, offset)| {
                let transformed = self.transform_point(offset);
                (
                    Self::terminal_label(idx),
                    Point::new(self.pos.x + transformed.x, self.pos.y + transformed.y),
                )
            })
            .collect()
    }

    fn dynamic_terminal_offsets(&self, pin_count: usize) -> Vec<Point> {
        if pin_count == 0 {
            return Vec::new();
        }

        let (w, h) = self.symbol_dimensions();
        let hw = w / 2;
        let hh = h / 2;

        if pin_count == 1 {
            return vec![Point::new(-hw, 0)];
        }
        if pin_count == 2 {
            return vec![Point::new(-hw, 0), Point::new(hw, 0)];
        }

        let left_count = pin_count.div_ceil(2);
        let right_count = pin_count - left_count;
        let left_y = Self::distributed_pin_axis(left_count, hh);
        let right_y = Self::distributed_pin_axis(right_count, hh);

        let mut points = Vec::with_capacity(pin_count);
        for y in left_y {
            points.push(Point::new(-hw, y));
        }
        for y in right_y {
            points.push(Point::new(hw, y));
        }
        points
    }

    fn distributed_pin_axis(count: usize, half_height: i32) -> Vec<i32> {
        if count == 0 {
            return Vec::new();
        }
        if count == 1 {
            return vec![0];
        }

        let span = half_height * 2;
        (0..count)
            .map(|idx| -half_height + ((idx as i32 * span) / ((count - 1) as i32)))
            .collect()
    }

    fn terminal_label(idx: usize) -> &'static str {
        const LABELS: [&str; 24] = [
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
            "17", "18", "19", "20", "21", "22", "23", "24",
        ];
        LABELS.get(idx).copied().unwrap_or("P")
    }

    /// Effective symbol dimensions, allowing generic block scaling for bound library cells.
    fn symbol_dimensions(&self) -> (i32, i32) {
        if let Some(cell) = &self.library_cell {
            let pin_count = if cell.terminal_order.is_empty() {
                2
            } else {
                cell.terminal_order.len()
            };
            let rows = pin_count.div_ceil(2).max(2) as i32;
            let height = (rows * 20).max(40);
            return (60, height);
        }
        self.kind.symbol_dimensions()
    }

    /// Transform a point from component-local coordinates to world coordinates
    ///
    /// Applies both mirror and rotation transforms in the correct order:
    /// 1. Apply mirror (about component origin)
    /// 2. Apply rotation (about component origin)
    ///
    /// This matches the standard EDA convention (Cadence Virtuoso, etc.)
    pub fn transform_point(&self, p: Point) -> Point {
        // Step 1: Apply mirror transforms
        let x = if self.mirror_h { -p.x } else { p.x };
        let y = if self.mirror_v { -p.y } else { p.y };
        let mirrored = Point::new(x, y);

        // Step 2: Apply rotation
        match self.rotation {
            Rotation::R0 => mirrored,
            Rotation::R90 => Point::new(-mirrored.y, mirrored.x),
            Rotation::R180 => Point::new(-mirrored.x, -mirrored.y),
            Rotation::R270 => Point::new(mirrored.y, -mirrored.x),
        }
    }

    /// Rotate a point by the component's rotation (legacy method)
    ///
    /// NOTE: For new code, prefer transform_point() which also applies mirror.
    /// This method is kept for backward compatibility.
    #[inline]
    pub fn rotate_point(&self, p: Point) -> Point {
        match self.rotation {
            Rotation::R0 => p,
            Rotation::R90 => Point::new(-p.y, p.x),
            Rotation::R180 => Point::new(-p.x, -p.y),
            Rotation::R270 => Point::new(p.y, -p.x),
        }
    }

    /// Get the bounding box of this component in grid coordinates
    ///
    /// Returns (min_x, min_y, max_x, max_y) representing the
    /// footprint of the component symbol, matching the actual rendered size.
    pub fn bounding_box(&self) -> (i32, i32, i32, i32) {
        // Use actual symbol dimensions for accurate hit detection
        let (width, height) = self.symbol_dimensions();
        let half_w = width / 2;
        let half_h = height / 2;

        // Swap dimensions if rotated 90 or 270 degrees
        let (hw, hh) = if self.rotation.is_vertical() {
            (half_h, half_w)
        } else {
            (half_w, half_h)
        };

        (
            self.pos.x - hw,
            self.pos.y - hh,
            self.pos.x + hw,
            self.pos.y + hh,
        )
    }

    /// Check if a grid point is within this component's bounding box
    pub fn contains_point(&self, p: Point) -> bool {
        let (min_x, min_y, max_x, max_y) = self.bounding_box();
        p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y
    }

    /// Get the SPICE netlist line for this component
    ///
    /// Note: This is a simplified version. Full netlist generation
    /// requires node connectivity information.
    pub fn spice_instance_name(&self) -> String {
        if self.name.is_empty() {
            format!("{}{}", self.kind.spice_prefix(), self.id)
        } else {
            self.name.clone()
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

