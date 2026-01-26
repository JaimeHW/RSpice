//! Symbol Library - Commercial-grade SVG symbol management for schematic components
//!
//! This module provides:
//! - SVG path parsing and conversion to egui-compatible draw commands
//! - Symbol registry with O(1) lookup by ComponentType
//! - Rotation transforms for 0°, 90°, 180°, 270° orientations
//! - Pin position definitions for wire attachment
//!
//! Follows Cadence Spectre patterns for symbol management.

use crate::state::ComponentType;
use egui::{Color32, Painter, Pos2, Stroke, Vec2};
use std::collections::HashMap;
use std::f32::consts::PI;

// ============================================================================
// Symbol Path Commands
// ============================================================================

/// A single path command for rendering symbol graphics.
/// These are pre-parsed from SVG paths for efficient runtime rendering.
#[derive(Debug, Clone, PartialEq)]
pub enum PathCommand {
    /// Move to absolute position
    MoveTo(f32, f32),
    /// Draw line to absolute position
    LineTo(f32, f32),
    /// Cubic bezier curve with control points
    CurveTo {
        ctrl1: (f32, f32),
        ctrl2: (f32, f32),
        end: (f32, f32),
    },
    /// Close the current path
    Close,
}

/// A complete path within a symbol (may have multiple paths per symbol)
#[derive(Debug, Clone)]
pub struct SymbolPath {
    /// The sequence of drawing commands
    pub commands: Vec<PathCommand>,
    /// Whether to fill this path (vs stroke only)
    pub filled: bool,
}

// ============================================================================
// Symbol Pin Definitions
// ============================================================================

/// Direction a pin faces for wire attachment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinDirection {
    Left,
    Right,
    Up,
    Down,
}

/// A connection point on a symbol
#[derive(Debug, Clone)]
pub struct SymbolPin {
    /// Pin name (e.g., "1", "2", "G", "D", "S")
    pub name: String,
    /// Position relative to symbol center (in normalized coords)
    pub position: (f32, f32),
    /// Direction the pin faces (for wire routing)
    pub direction: PinDirection,
}

// ============================================================================
// Symbol Definition
// ============================================================================

/// A complete schematic symbol definition
#[derive(Debug, Clone)]
pub struct Symbol {
    /// Human-readable name
    pub name: String,
    /// All paths that make up this symbol
    pub paths: Vec<SymbolPath>,
    /// Connection points
    pub pins: Vec<SymbolPin>,
    /// Bounding box (min_x, min_y, max_x, max_y) in normalized coords
    pub bounds: (f32, f32, f32, f32),
    /// Original SVG viewBox for scaling
    pub view_box: (f32, f32, f32, f32),
    /// Target width for rendering (in grid units) - commercial-grade per-component sizing
    pub target_width: f32,
    /// Target height for rendering (in grid units) - commercial-grade per-component sizing
    pub target_height: f32,
}

impl Symbol {
    /// Get symbol width in normalized coordinates
    pub fn width(&self) -> f32 {
        self.bounds.2 - self.bounds.0
    }

    /// Get symbol height in normalized coordinates
    pub fn height(&self) -> f32 {
        self.bounds.3 - self.bounds.1
    }

    /// Center point of the symbol
    pub fn center(&self) -> (f32, f32) {
        (
            (self.bounds.0 + self.bounds.2) / 2.0,
            (self.bounds.1 + self.bounds.3) / 2.0,
        )
    }
}

// ============================================================================
// Symbol Library
// ============================================================================

/// SVG Symbol Library with O(1) lookup by component type.
/// Loads and caches parsed symbols for efficient rendering.
pub struct SymbolLibrary {
    symbols: HashMap<ComponentType, Symbol>,
}

impl Default for SymbolLibrary {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolLibrary {
    /// Create a new empty symbol library
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    /// Load all embedded SVG symbols from the assets directory.
    /// Returns the library with all symbols loaded, or an error if any fail.
    pub fn load_embedded() -> Result<Self, SymbolError> {
        let mut library = Self::new();

        // Map ComponentType to (SVG filename, Name, target_width, target_height)
        // Target dimensions follow commercial EDA standards (Cadence Spectre-style sizing):
        // - Passive components (R, L, C): horizontal orientation, ~30x10 grid units
        // - Sources: vertical orientation, ~20x40 grid units
        // - Semiconductors (transistors): vertical orientation, ~20x40 grid units
        // - Ground: compact, ~15x20 grid units
        let symbol_mappings: &[(ComponentType, &str, &str, f32, f32)] = &[
            // Passive components - horizontal orientation
            (
                ComponentType::Resistor,
                "resistor.svg",
                "Resistor",
                30.0,
                10.0,
            ),
            (
                ComponentType::Capacitor,
                "cap_unpolarized.svg",
                "Capacitor",
                20.0,
                30.0,
            ),
            (
                ComponentType::Inductor,
                "inductor.svg",
                "Inductor",
                30.0,
                10.0,
            ),
            // Sources - vertical orientation with leads
            (
                ComponentType::VoltageSource,
                "v_src_dc.svg",
                "Voltage Source",
                20.0,
                40.0,
            ),
            (
                ComponentType::CurrentSource,
                "i_src.svg",
                "Current Source",
                20.0,
                40.0,
            ),
            // Ground - compact
            (
                ComponentType::Ground,
                "ground_signal.svg",
                "Ground",
                15.0,
                20.0,
            ),
            // Discrete semiconductors
            (ComponentType::Diode, "diode.svg", "Diode", 25.0, 15.0),
            // Transistors - vertical orientation
            (
                ComponentType::Nmos,
                "mos_n_chan_enh_no_substrate.svg",
                "NMOS",
                20.0,
                40.0,
            ),
            (
                ComponentType::Pmos,
                "mos_p_chan_enh_no_substrate.svg",
                "PMOS",
                20.0,
                40.0,
            ),
            (ComponentType::NpnBjt, "bjt_npn.svg", "NPN BJT", 20.0, 40.0),
            (ComponentType::PnpBjt, "bjt_pnp.svg", "PNP BJT", 20.0, 40.0),
        ];

        for (component_type, filename, name, target_w, target_h) in symbol_mappings {
            // Load SVG from embedded assets
            let svg_data = Self::load_embedded_svg(filename)?;
            let mut symbol = parse_svg(&svg_data)?;
            symbol.name = name.to_string();
            symbol.target_width = *target_w;
            symbol.target_height = *target_h;

            // Add default pins based on component type
            add_default_pins(&mut symbol, *component_type);

            library.symbols.insert(*component_type, symbol);
        }

        Ok(library)
    }

    /// Load an embedded SVG file by name
    pub(crate) fn load_embedded_svg(filename: &str) -> Result<String, SymbolError> {
        // Path relative to crate root - assets is inside the crate directory
        let svg_path = format!(
            "{}/assets/component_symbols/{}",
            env!("CARGO_MANIFEST_DIR"),
            filename
        );

        std::fs::read_to_string(&svg_path).map_err(|e| SymbolError::IoError {
            path: svg_path,
            message: e.to_string(),
        })
    }

    /// Get a symbol by component type (O(1) lookup)
    pub fn get(&self, component_type: ComponentType) -> Option<&Symbol> {
        self.symbols.get(&component_type)
    }

    /// Check if a symbol exists for the given component type
    pub fn contains(&self, component_type: ComponentType) -> bool {
        self.symbols.contains_key(&component_type)
    }

    /// Get all loaded component types
    pub fn loaded_types(&self) -> Vec<ComponentType> {
        self.symbols.keys().copied().collect()
    }

    /// Number of loaded symbols
    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    /// Check if library is empty
    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }
}

// ============================================================================
// Symbol Rendering
// ============================================================================

/// Render a symbol to an egui painter with position, scale, rotation, and stroke
pub fn draw_symbol(
    painter: &Painter,
    symbol: &Symbol,
    center: Pos2,
    scale: f32,
    rotation_degrees: i32,
    stroke: Stroke,
) {
    // Debug: log when drawing
    log::trace!(
        "Drawing SVG symbol '{}' at {:?}, {} paths, scale={}",
        symbol.name,
        center,
        symbol.paths.len(),
        scale
    );

    let rotation_rad = (rotation_degrees as f32) * PI / 180.0;
    let cos_r = rotation_rad.cos();
    let sin_r = rotation_rad.sin();

    // Symbol center for rotation pivot
    let (cx, cy) = symbol.center();

    // Commercial-grade per-component scaling:
    // Calculate scale factors to fit symbol into its target dimensions
    // This ensures each component type renders at its intended size
    let scale_x = symbol.target_width / symbol.width().max(0.001);
    let scale_y = symbol.target_height / symbol.height().max(0.001);
    // Use uniform scaling to preserve aspect ratio (take minimum to fit within target box)
    let view_scale = scale_x.min(scale_y);
    let total_scale = scale * view_scale;

    // Debug: log symbol dimensions
    log::debug!(
        "Symbol '{}' bounds=({:.1},{:.1},{:.1},{:.1}), target=({:.1}x{:.1}), view_scale={:.2}, total_scale={:.2}",
        symbol.name, symbol.bounds.0, symbol.bounds.1, symbol.bounds.2, symbol.bounds.3,
        symbol.target_width, symbol.target_height, view_scale, total_scale
    );

    let mut total_points_drawn = 0;

    for path in &symbol.paths {
        let mut points: Vec<Pos2> = Vec::new();
        let mut current_pos = (0.0f32, 0.0f32);

        for cmd in &path.commands {
            match cmd {
                PathCommand::MoveTo(x, y) => {
                    // Draw accumulated points first
                    if points.len() >= 2 {
                        draw_path_segment(painter, &points, stroke);
                        total_points_drawn += points.len();
                    }
                    points.clear();

                    // Transform point
                    let (tx, ty) =
                        transform_point(*x, *y, cx, cy, cos_r, sin_r, total_scale, center);
                    points.push(Pos2::new(tx, ty));
                    current_pos = (*x, *y);
                }
                PathCommand::LineTo(x, y) => {
                    let (tx, ty) =
                        transform_point(*x, *y, cx, cy, cos_r, sin_r, total_scale, center);
                    points.push(Pos2::new(tx, ty));
                    current_pos = (*x, *y);
                }
                PathCommand::CurveTo { ctrl1, ctrl2, end } => {
                    // Approximate bezier with line segments
                    // Start from t=0 to ensure we don't skip the starting connection
                    let segments = 16; // Use more segments for smoother curves
                    for i in 0..=segments {
                        let t = i as f32 / segments as f32;
                        let (bx, by) = cubic_bezier(current_pos, *ctrl1, *ctrl2, *end, t);
                        let (tx, ty) =
                            transform_point(bx, by, cx, cy, cos_r, sin_r, total_scale, center);
                        // Skip adding t=0 point if it would duplicate the previous MoveTo point
                        if i == 0 && !points.is_empty() {
                            // Check if we'd be adding a duplicate
                            let last = points.last().unwrap();
                            let new_point = Pos2::new(tx, ty);
                            if (last.x - new_point.x).abs() < 0.1
                                && (last.y - new_point.y).abs() < 0.1
                            {
                                continue;
                            }
                        }
                        points.push(Pos2::new(tx, ty));
                    }
                    current_pos = *end;
                }
                PathCommand::Close => {
                    if let Some(first) = points.first() {
                        points.push(*first);
                    }
                }
            }
        }

        // Draw remaining points
        if points.len() >= 2 {
            draw_path_segment(painter, &points, stroke);
            total_points_drawn += points.len();
        }
    }

    // Debug: if no points were drawn, log a warning
    if total_points_drawn == 0 {
        log::warn!("SVG symbol '{}' had 0 points to draw!", symbol.name);
    }
}

/// Draw a sequence of connected line segments using individual line calls
fn draw_path_segment(painter: &Painter, points: &[Pos2], stroke: Stroke) {
    if points.len() < 2 {
        return;
    }

    // Draw circles FIRST (behind lines) at vertices where direction changes
    // This fills gaps at corners without visible circle edges (lines cover them)
    let radius = stroke.width / 2.0;

    // Check middle vertices for direction changes
    for i in 1..points.len().saturating_sub(1) {
        let prev = points[i - 1];
        let curr = points[i];
        let next = points[i + 1];

        // Calculate direction vectors
        let d1 = (curr.x - prev.x, curr.y - prev.y);
        let d2 = (next.x - curr.x, next.y - curr.y);

        // Normalize and check if directions are different (not collinear)
        let len1 = (d1.0 * d1.0 + d1.1 * d1.1).sqrt();
        let len2 = (d2.0 * d2.0 + d2.1 * d2.1).sqrt();

        if len1 > 0.001 && len2 > 0.001 {
            let n1 = (d1.0 / len1, d1.1 / len1);
            let n2 = (d2.0 / len2, d2.1 / len2);

            // Dot product - if close to 1, segments are collinear
            let dot = n1.0 * n2.0 + n1.1 * n2.1;

            // Only draw at sharp corners (angle > ~30 degrees)
            if dot.abs() < 0.87 {
                painter.circle_filled(curr, radius, stroke.color);
            }
        }
    }

    // Draw line segments ON TOP of circles
    for pair in points.windows(2) {
        painter.line_segment([pair[0], pair[1]], stroke);
    }
}

/// Transform a point with rotation around symbol center and scale
fn transform_point(
    x: f32,
    y: f32,
    cx: f32,
    cy: f32,
    cos_r: f32,
    sin_r: f32,
    scale: f32,
    center: Pos2,
) -> (f32, f32) {
    // Translate to origin
    let dx = x - cx;
    let dy = y - cy;

    // Rotate
    let rx = dx * cos_r - dy * sin_r;
    let ry = dx * sin_r + dy * cos_r;

    // Scale and translate to screen position
    (center.x + rx * scale, center.y + ry * scale)
}

/// Evaluate cubic bezier at parameter t
fn cubic_bezier(
    p0: (f32, f32),
    p1: (f32, f32),
    p2: (f32, f32),
    p3: (f32, f32),
    t: f32,
) -> (f32, f32) {
    let t2 = t * t;
    let t3 = t2 * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let mt3 = mt2 * mt;

    let x = mt3 * p0.0 + 3.0 * mt2 * t * p1.0 + 3.0 * mt * t2 * p2.0 + t3 * p3.0;
    let y = mt3 * p0.1 + 3.0 * mt2 * t * p1.1 + 3.0 * mt * t2 * p2.1 + t3 * p3.1;
    (x, y)
}

// ============================================================================
// SVG Parsing
// ============================================================================

/// Parse an SVG string into a Symbol
pub fn parse_svg(svg_data: &str) -> Result<Symbol, SymbolError> {
    // Preprocess SVG to remove broken filter references
    // Many Inkscape SVGs have filter:url(#filterXXX) references to non-existent filters
    // which causes usvg to skip those elements entirely
    let sanitized_svg = svg_data
        .replace(";filter:url(#filter5647)", "")
        .replace("filter:url(#filter5647);", "")
        .replace("filter:url(#filter5647)", "");

    // Parse using usvg
    let options = usvg::Options::default();
    let tree = usvg::Tree::from_str(&sanitized_svg, &options)
        .map_err(|e| SymbolError::ParseError(format!("Failed to parse SVG: {}", e)))?;

    let mut paths = Vec::new();
    let mut min_x = f32::MAX;
    let mut min_y = f32::MAX;
    let mut max_x = f32::MIN;
    let mut max_y = f32::MIN;

    // Extract paths from the tree
    extract_paths(
        tree.root(),
        &mut paths,
        &mut min_x,
        &mut min_y,
        &mut max_x,
        &mut max_y,
    );

    // Log number of paths found (before normalization)
    log::debug!(
        "Parsed SVG: {} paths, raw bounds=({:.1},{:.1},{:.1},{:.1})",
        paths.len(),
        min_x,
        min_y,
        max_x,
        max_y
    );

    // Normalize coordinates: translate so bounds start at (0,0)
    // This is necessary because usvg preserves SVG coordinates with transforms applied
    let offset_x = if min_x != f32::MAX { min_x } else { 0.0 };
    let offset_y = if min_y != f32::MAX { min_y } else { 0.0 };

    for path in &mut paths {
        for cmd in &mut path.commands {
            match cmd {
                PathCommand::MoveTo(x, y) => {
                    *x -= offset_x;
                    *y -= offset_y;
                }
                PathCommand::LineTo(x, y) => {
                    *x -= offset_x;
                    *y -= offset_y;
                }
                PathCommand::CurveTo { ctrl1, ctrl2, end } => {
                    ctrl1.0 -= offset_x;
                    ctrl1.1 -= offset_y;
                    ctrl2.0 -= offset_x;
                    ctrl2.1 -= offset_y;
                    end.0 -= offset_x;
                    end.1 -= offset_y;
                }
                PathCommand::Close => {}
            }
        }
    }

    // Update bounds to reflect normalization
    let (norm_min_x, norm_min_y, norm_max_x, norm_max_y) = if min_x != f32::MAX {
        (0.0, 0.0, max_x - offset_x, max_y - offset_y)
    } else {
        (min_x, min_y, max_x, max_y)
    };

    // Get viewBox
    let view_box = tree.size();
    let vb = (0.0, 0.0, view_box.width() as f32, view_box.height() as f32);

    // Use viewBox if no paths found bounds
    let final_bounds = if norm_min_x == f32::MAX {
        (vb.0, vb.1, vb.0 + vb.2, vb.1 + vb.3)
    } else {
        (norm_min_x, norm_min_y, norm_max_x, norm_max_y)
    };

    log::debug!(
        "Normalized bounds: ({:.1},{:.1},{:.1},{:.1})",
        final_bounds.0,
        final_bounds.1,
        final_bounds.2,
        final_bounds.3
    );

    Ok(Symbol {
        name: String::new(),
        paths,
        pins: Vec::new(),
        bounds: final_bounds,
        view_box: vb,
        // Default target dimensions - will be overridden by load_embedded for known components
        target_width: 30.0,
        target_height: 30.0,
    })
}

/// Recursively extract paths from usvg tree (usvg 0.44 API)
/// IMPORTANT: usvg does NOT automatically apply group transforms to path data.
/// We must manually apply path.abs_transform() to get coordinates in document space.
fn extract_paths(
    group: &usvg::Group,
    paths: &mut Vec<SymbolPath>,
    min_x: &mut f32,
    min_y: &mut f32,
    max_x: &mut f32,
    max_y: &mut f32,
) {
    for node in group.children() {
        match node {
            usvg::Node::Path(path) => {
                let mut commands = Vec::new();
                let data = path.data();

                // Get the absolute transform (includes all parent group transforms)
                // This is REQUIRED because usvg preserves group transforms separately
                let t = path.abs_transform();

                // Transform helper: applies the 2D affine transformation
                let transform = |x: f32, y: f32| -> (f32, f32) {
                    (t.sx * x + t.kx * y + t.tx, t.ky * x + t.sy * y + t.ty)
                };

                for segment in data.segments() {
                    match segment {
                        usvg::tiny_skia_path::PathSegment::MoveTo(p) => {
                            let (x, y) = transform(p.x, p.y);
                            update_bounds(x, y, min_x, min_y, max_x, max_y);
                            commands.push(PathCommand::MoveTo(x, y));
                        }
                        usvg::tiny_skia_path::PathSegment::LineTo(p) => {
                            let (x, y) = transform(p.x, p.y);
                            update_bounds(x, y, min_x, min_y, max_x, max_y);
                            commands.push(PathCommand::LineTo(x, y));
                        }
                        usvg::tiny_skia_path::PathSegment::CubicTo(p1, p2, p) => {
                            let (x1, y1) = transform(p1.x, p1.y);
                            let (x2, y2) = transform(p2.x, p2.y);
                            let (x, y) = transform(p.x, p.y);
                            update_bounds(x, y, min_x, min_y, max_x, max_y);
                            update_bounds(x1, y1, min_x, min_y, max_x, max_y);
                            update_bounds(x2, y2, min_x, min_y, max_x, max_y);
                            commands.push(PathCommand::CurveTo {
                                ctrl1: (x1, y1),
                                ctrl2: (x2, y2),
                                end: (x, y),
                            });
                        }
                        usvg::tiny_skia_path::PathSegment::QuadTo(p1, p) => {
                            let (x1, y1) = transform(p1.x, p1.y);
                            let (x, y) = transform(p.x, p.y);
                            update_bounds(x, y, min_x, min_y, max_x, max_y);
                            update_bounds(x1, y1, min_x, min_y, max_x, max_y);
                            // Convert quadratic to cubic bezier approximation
                            commands.push(PathCommand::CurveTo {
                                ctrl1: (x1, y1),
                                ctrl2: (x1, y1),
                                end: (x, y),
                            });
                        }
                        usvg::tiny_skia_path::PathSegment::Close => {
                            commands.push(PathCommand::Close);
                        }
                    }
                }

                if !commands.is_empty() {
                    paths.push(SymbolPath {
                        commands,
                        filled: path.fill().is_some(),
                    });
                }
            }
            usvg::Node::Group(nested_group) => {
                extract_paths(nested_group, paths, min_x, min_y, max_x, max_y);
            }
            _ => {}
        }
    }
}

fn update_bounds(
    x: f32,
    y: f32,
    min_x: &mut f32,
    min_y: &mut f32,
    max_x: &mut f32,
    max_y: &mut f32,
) {
    *min_x = min_x.min(x);
    *min_y = min_y.min(y);
    *max_x = max_x.max(x);
    *max_y = max_y.max(y);
}

/// Add default pin positions based on component type
fn add_default_pins(symbol: &mut Symbol, component_type: ComponentType) {
    let (cx, cy) = symbol.center();
    let w = symbol.width() / 2.0;
    let h = symbol.height() / 2.0;

    match component_type {
        ComponentType::Resistor | ComponentType::Capacitor | ComponentType::Inductor => {
            // Two-terminal horizontal component
            symbol.pins = vec![
                SymbolPin {
                    name: "1".to_string(),
                    position: (cx - w, cy),
                    direction: PinDirection::Left,
                },
                SymbolPin {
                    name: "2".to_string(),
                    position: (cx + w, cy),
                    direction: PinDirection::Right,
                },
            ];
        }
        ComponentType::VoltageSource | ComponentType::CurrentSource => {
            // Vertical source
            symbol.pins = vec![
                SymbolPin {
                    name: "+".to_string(),
                    position: (cx, cy - h),
                    direction: PinDirection::Up,
                },
                SymbolPin {
                    name: "-".to_string(),
                    position: (cx, cy + h),
                    direction: PinDirection::Down,
                },
            ];
        }
        ComponentType::Ground => {
            symbol.pins = vec![SymbolPin {
                name: "0".to_string(),
                position: (cx, cy - h),
                direction: PinDirection::Up,
            }];
        }
        ComponentType::Diode => {
            symbol.pins = vec![
                SymbolPin {
                    name: "A".to_string(),
                    position: (cx - w, cy),
                    direction: PinDirection::Left,
                },
                SymbolPin {
                    name: "K".to_string(),
                    position: (cx + w, cy),
                    direction: PinDirection::Right,
                },
            ];
        }
        ComponentType::Nmos | ComponentType::Pmos => {
            symbol.pins = vec![
                SymbolPin {
                    name: "G".to_string(),
                    position: (cx - w, cy),
                    direction: PinDirection::Left,
                },
                SymbolPin {
                    name: "D".to_string(),
                    position: (cx, cy - h),
                    direction: PinDirection::Up,
                },
                SymbolPin {
                    name: "S".to_string(),
                    position: (cx, cy + h),
                    direction: PinDirection::Down,
                },
            ];
        }
        ComponentType::NpnBjt | ComponentType::PnpBjt => {
            symbol.pins = vec![
                SymbolPin {
                    name: "B".to_string(),
                    position: (cx - w, cy),
                    direction: PinDirection::Left,
                },
                SymbolPin {
                    name: "C".to_string(),
                    position: (cx, cy - h),
                    direction: PinDirection::Up,
                },
                SymbolPin {
                    name: "E".to_string(),
                    position: (cx, cy + h),
                    direction: PinDirection::Down,
                },
            ];
        }
        _ => {}
    }
}

// ============================================================================
// Error Types
// ============================================================================

/// Errors that can occur during symbol loading or parsing
#[derive(Debug, Clone)]
pub enum SymbolError {
    /// Failed to read file
    IoError { path: String, message: String },
    /// Failed to parse SVG content
    ParseError(String),
    /// Symbol not found in library
    NotFound(ComponentType),
}

impl std::fmt::Display for SymbolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolError::IoError { path, message } => {
                write!(f, "Failed to read '{}': {}", path, message)
            }
            SymbolError::ParseError(msg) => write!(f, "SVG parse error: {}", msg),
            SymbolError::NotFound(t) => write!(f, "Symbol not found for {:?}", t),
        }
    }
}

impl std::error::Error for SymbolError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_command_equality() {
        let cmd1 = PathCommand::MoveTo(10.0, 20.0);
        let cmd2 = PathCommand::MoveTo(10.0, 20.0);
        let cmd3 = PathCommand::LineTo(10.0, 20.0);

        assert_eq!(cmd1, cmd2);
        assert_ne!(cmd1, cmd3);
    }

    #[test]
    fn test_symbol_bounds() {
        let symbol = Symbol {
            name: "Test".to_string(),
            paths: vec![],
            pins: vec![],
            bounds: (0.0, 0.0, 100.0, 50.0),
            view_box: (0.0, 0.0, 100.0, 50.0),
            target_width: 30.0,
            target_height: 30.0,
        };

        assert_eq!(symbol.width(), 100.0);
        assert_eq!(symbol.height(), 50.0);
        assert_eq!(symbol.center(), (50.0, 25.0));
    }

    #[test]
    fn test_symbol_library_creation() {
        let library = SymbolLibrary::new();
        assert!(library.is_empty());
        assert_eq!(library.len(), 0);
    }

    #[test]
    fn test_cubic_bezier_endpoints() {
        let p0 = (0.0, 0.0);
        let p1 = (1.0, 2.0);
        let p2 = (3.0, 2.0);
        let p3 = (4.0, 0.0);

        // At t=0, should be at p0
        let (x, y) = cubic_bezier(p0, p1, p2, p3, 0.0);
        assert!((x - p0.0).abs() < 0.001);
        assert!((y - p0.1).abs() < 0.001);

        // At t=1, should be at p3
        let (x, y) = cubic_bezier(p0, p1, p2, p3, 1.0);
        assert!((x - p3.0).abs() < 0.001);
        assert!((y - p3.1).abs() < 0.001);
    }

    #[test]
    fn test_transform_point_no_rotation() {
        let (tx, ty) = transform_point(
            10.0,
            20.0,
            5.0,
            10.0,
            1.0,
            0.0,
            1.0,
            Pos2::new(100.0, 100.0),
        );

        // Point (10,20) relative to center (5,10) = offset (5, 10)
        // No rotation (cos=1, sin=0), scale 1, center at (100, 100)
        // Result: (100 + 5, 100 + 10) = (105, 110)
        assert!((tx - 105.0).abs() < 0.001);
        assert!((ty - 110.0).abs() < 0.001);
    }

    #[test]
    fn test_transform_point_90_degree_rotation() {
        use std::f32::consts::FRAC_PI_2;
        let cos_r = FRAC_PI_2.cos(); // ~0
        let sin_r = FRAC_PI_2.sin(); // ~1

        // Point at (10, 0) relative to center (0, 0), rotated 90 degrees
        let (tx, ty) = transform_point(10.0, 0.0, 0.0, 0.0, cos_r, sin_r, 1.0, Pos2::new(0.0, 0.0));

        // After 90 degree rotation: (x,y) -> (-y, x) conceptually
        // Actually: rx = x*cos - y*sin = 10*0 - 0*1 = 0
        //           ry = x*sin + y*cos = 10*1 + 0*0 = 10
        assert!(tx.abs() < 0.001);
        assert!((ty - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_parse_simple_svg() {
        let svg = r#"<?xml version="1.0"?>
        <svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100">
            <path d="M 10,10 L 90,10 L 90,90 L 10,90 Z" fill="none" stroke="black"/>
        </svg>"#;

        let symbol = parse_svg(svg).expect("Failed to parse SVG");

        assert!(!symbol.paths.is_empty(), "Should have at least one path");
        assert!(symbol.width() > 0.0, "Width should be positive");
        assert!(symbol.height() > 0.0, "Height should be positive");
    }

    #[test]
    fn test_symbol_error_display() {
        let err = SymbolError::IoError {
            path: "/test/path.svg".to_string(),
            message: "File not found".to_string(),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("/test/path.svg"));
        assert!(msg.contains("File not found"));
    }

    #[test]
    fn test_pin_direction_variants() {
        // Ensure all variants can be created and compared
        let dirs = [
            PinDirection::Left,
            PinDirection::Right,
            PinDirection::Up,
            PinDirection::Down,
        ];

        for dir in &dirs {
            assert_eq!(*dir, *dir);
        }
        assert_ne!(PinDirection::Left, PinDirection::Right);
    }

    #[test]
    fn test_symbol_path_clone() {
        let path = SymbolPath {
            commands: vec![
                PathCommand::MoveTo(0.0, 0.0),
                PathCommand::LineTo(10.0, 10.0),
            ],
            filled: false,
        };

        let cloned = path.clone();
        assert_eq!(path.commands.len(), cloned.commands.len());
        assert_eq!(path.filled, cloned.filled);
    }

    #[test]
    fn test_load_embedded_resistor_svg() {
        // Test that we can load the resistor SVG from assets
        let result = SymbolLibrary::load_embedded_svg("resistor.svg");
        assert!(
            result.is_ok(),
            "Failed to load resistor.svg: {:?}",
            result.err()
        );

        let svg_content = result.unwrap();
        assert!(svg_content.contains("<svg"), "SVG should contain <svg tag");
        assert!(
            svg_content.contains("path"),
            "SVG should contain path elements"
        );
    }

    #[test]
    fn test_parse_resistor_svg_has_paths() {
        // Test that parsing the resistor SVG extracts paths
        let svg_result = SymbolLibrary::load_embedded_svg("resistor.svg");
        assert!(svg_result.is_ok(), "Failed to load resistor.svg");

        let symbol = parse_svg(&svg_result.unwrap());
        assert!(symbol.is_ok(), "Failed to parse SVG: {:?}", symbol.err());

        let symbol = symbol.unwrap();
        assert!(
            !symbol.paths.is_empty(),
            "Resistor symbol should have paths, got 0"
        );
        assert!(symbol.width() > 0.0, "Symbol should have positive width");
        assert!(symbol.height() > 0.0, "Symbol should have positive height");
    }

    #[test]
    fn test_load_all_embedded_symbols() {
        // Test that we can load all 11 mapped component symbols
        let library = SymbolLibrary::load_embedded();
        assert!(
            library.is_ok(),
            "Failed to load symbol library: {:?}",
            library.err()
        );

        let library = library.unwrap();
        assert_eq!(library.len(), 11, "Should have 11 symbols loaded");

        // Verify each symbol has paths
        for component_type in library.loaded_types() {
            let symbol = library.get(component_type).expect("Symbol should exist");
            assert!(
                !symbol.paths.is_empty(),
                "Symbol {:?} should have paths, got 0",
                component_type
            );
        }
    }

    #[test]
    fn test_symbol_target_dimensions_commercial_grade() {
        // Commercial-grade verification: each component type should have appropriate
        // target dimensions per EDA standards (Cadence Spectre-style sizing)
        let library = SymbolLibrary::load_embedded().expect("Should load library");

        // Verify passive components have horizontal orientation (wider than tall)
        let resistor = library.get(crate::state::ComponentType::Resistor).unwrap();
        assert_eq!(
            resistor.target_width, 30.0,
            "Resistor should have 30.0 target width"
        );
        assert_eq!(
            resistor.target_height, 10.0,
            "Resistor should have 10.0 target height"
        );
        assert!(
            resistor.target_width > resistor.target_height,
            "Resistor should be horizontal"
        );

        let inductor = library.get(crate::state::ComponentType::Inductor).unwrap();
        assert_eq!(
            inductor.target_width, 30.0,
            "Inductor should have 30.0 target width"
        );
        assert_eq!(
            inductor.target_height, 10.0,
            "Inductor should have 10.0 target height"
        );

        // Verify sources have vertical orientation (taller than wide)
        let vsrc = library
            .get(crate::state::ComponentType::VoltageSource)
            .unwrap();
        assert_eq!(
            vsrc.target_width, 20.0,
            "VoltageSource should have 20.0 target width"
        );
        assert_eq!(
            vsrc.target_height, 40.0,
            "VoltageSource should have 40.0 target height"
        );
        assert!(
            vsrc.target_height > vsrc.target_width,
            "VoltageSource should be vertical"
        );

        // Verify transistors have vertical orientation
        let nmos = library.get(crate::state::ComponentType::Nmos).unwrap();
        assert_eq!(
            nmos.target_width, 20.0,
            "NMOS should have 20.0 target width"
        );
        assert_eq!(
            nmos.target_height, 40.0,
            "NMOS should have 40.0 target height"
        );

        let npn = library.get(crate::state::ComponentType::NpnBjt).unwrap();
        assert_eq!(
            npn.target_width, 20.0,
            "NPN BJT should have 20.0 target width"
        );
        assert_eq!(
            npn.target_height, 40.0,
            "NPN BJT should have 40.0 target height"
        );

        // Verify ground is compact
        let ground = library.get(crate::state::ComponentType::Ground).unwrap();
        assert_eq!(
            ground.target_width, 15.0,
            "Ground should have 15.0 target width"
        );
        assert_eq!(
            ground.target_height, 20.0,
            "Ground should have 20.0 target height"
        );

        // Verify all symbols have positive target dimensions
        for component_type in library.loaded_types() {
            let symbol = library.get(component_type).unwrap();
            assert!(
                symbol.target_width > 0.0,
                "{:?} should have positive target_width",
                component_type
            );
            assert!(
                symbol.target_height > 0.0,
                "{:?} should have positive target_height",
                component_type
            );
        }
    }
}
