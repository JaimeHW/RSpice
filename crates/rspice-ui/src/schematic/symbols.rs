//! Symbol Library - Commercial-grade SVG symbol management for schematic components
//!
//! This module provides:
//! - SVG path parsing and conversion to egui-compatible draw commands
//! - Symbol registry with O(1) lookup by ComponentType
//! - Rotation transforms for 0°, 90°, 180°, 270° orientations
//! - Pin position definitions for wire attachment
//!
//! Follows Cadence Spectre patterns for symbol management.

#![allow(clippy::too_many_arguments)]

use crate::state::ComponentType;
use egui::{Painter, Pos2, Stroke};
use std::collections::HashMap;
use std::f32::consts::PI;

mod embedded_symbols {
    include!(concat!(env!("OUT_DIR"), "/embedded_symbols.rs"));
}

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

    /// Center point of the symbol (in normalized coordinates starting at 0,0)
    pub fn center(&self) -> (f32, f32) {
        // Use bounds center since paths are normalized to start at (0,0)
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
/// Supports orientation-specific symbols (vertical/horizontal) for components
/// that have different SVGs for different rotations.
pub struct SymbolLibrary {
    /// Default (vertical) symbols
    symbols: HashMap<ComponentType, Symbol>,
    /// Horizontal variants for components that have separate horizontal SVGs
    horizontal_symbols: HashMap<ComponentType, Symbol>,
    /// Non-default symbol variants keyed by component type and variant id
    variant_symbols: HashMap<(ComponentType, String), Symbol>,
    /// Horizontal symbol variants keyed by component type and variant id
    horizontal_variant_symbols: HashMap<(ComponentType, String), Symbol>,
    /// All embedded asset files parsed successfully and keyed by filename
    embedded_assets: HashMap<String, Symbol>,
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
            horizontal_symbols: HashMap::new(),
            variant_symbols: HashMap::new(),
            horizontal_variant_symbols: HashMap::new(),
            embedded_assets: HashMap::new(),
        }
    }

    /// Load all embedded SVG symbols from the assets directory.
    /// Returns the library with all symbols loaded, or an error if any fail.
    pub fn load_embedded() -> Result<Self, SymbolError> {
        let mut library = Self::new();

        library.embedded_assets = Self::load_all_embedded_assets()?;

        // Map ComponentType to default SVG filename.
        let default_mappings: &[(ComponentType, &str, &str)] = &[
            (ComponentType::Resistor, "resistor.svg", "Resistor"),
            (ComponentType::Capacitor, "cap_unpolarized.svg", "Capacitor"),
            (ComponentType::Inductor, "inductor.svg", "Inductor"),
            (
                ComponentType::Transformer,
                "transformer_symmetrical.svg",
                "Transformer",
            ),
            (
                ComponentType::SaturableInductor,
                "inductor.svg",
                "Saturable Inductor",
            ),
            (
                ComponentType::VoltageSource,
                "v_src_dc.svg",
                "Voltage Source",
            ),
            (
                ComponentType::VoltageSourceAc,
                "v_src_ac_vertical.svg",
                "AC Voltage Source",
            ),
            (
                ComponentType::VoltageSourceSin,
                "v_src_ac_vertical.svg",
                "Sinusoidal Voltage Source",
            ),
            (
                ComponentType::VoltageSourcePulse,
                "v_src_dc.svg",
                "Pulse Voltage Source",
            ),
            (
                ComponentType::VoltageSourcePwl,
                "v_src_dc.svg",
                "PWL Voltage Source",
            ),
            (
                ComponentType::VoltageSourceExp,
                "v_src_dc.svg",
                "Exponential Voltage Source",
            ),
            (
                ComponentType::VoltageSourceSffm,
                "v_src_dc.svg",
                "SFFM Voltage Source",
            ),
            (ComponentType::CurrentSource, "i_src.svg", "Current Source"),
            (
                ComponentType::CurrentSourceAc,
                "i_src.svg",
                "AC Current Source",
            ),
            (
                ComponentType::CurrentSourcePulse,
                "i_src.svg",
                "Pulse Current Source",
            ),
            (
                ComponentType::CurrentSourceSin,
                "i_src.svg",
                "Sinusoidal Current Source",
            ),
            (
                ComponentType::CurrentSourcePwl,
                "i_src.svg",
                "PWL Current Source",
            ),
            (
                ComponentType::CurrentSourceExp,
                "i_src.svg",
                "Exponential Current Source",
            ),
            (
                ComponentType::CurrentSourceNoise,
                "i_src.svg",
                "Noise Current Source",
            ),
            (ComponentType::Ground, "ground_signal.svg", "Ground"),
            (ComponentType::Diode, "diode.svg", "Diode"),
            (
                ComponentType::Nmos,
                "mos_n_chan_enh_no_substrate.svg",
                "NMOS",
            ),
            (
                ComponentType::Pmos,
                "mos_p_chan_enh_no_substrate.svg",
                "PMOS",
            ),
            (ComponentType::Njfet, "jfet_n_chan.svg", "N-JFET"),
            (ComponentType::Pjfet, "jfet_p_chan.svg", "P-JFET"),
            (
                ComponentType::NVdmos,
                "mos_n_chan_enh_body_diode_discrete.svg",
                "N-VDMOS",
            ),
            (
                ComponentType::PVdmos,
                "mos_p_chan_enh_body_diode_discrete.svg",
                "P-VDMOS",
            ),
            (ComponentType::NpnBjt, "bjt_npn.svg", "NPN BJT"),
            (ComponentType::PnpBjt, "bjt_pnp.svg", "PNP BJT"),
        ];

        for (component_type, filename, name) in default_mappings {
            let symbol = library.prepare_symbol(*component_type, filename, name, false)?;
            library.symbols.insert(*component_type, symbol);
        }

        // Load non-default visual variants for symbol families that already share
        // the same electrical terminals and can be treated as pure schematic skins.
        let variant_mappings: &[(ComponentType, &str, &str, &str)] = &[
            (
                ComponentType::VoltageSource,
                "battery",
                "battery.svg",
                "Battery",
            ),
            (
                ComponentType::VoltageSource,
                "battery_multi_cell",
                "battery_multi_cell.svg",
                "Battery",
            ),
            (ComponentType::Ground, "earth", "ground_earth.svg", "Ground"),
            (
                ComponentType::Ground,
                "chassis",
                "ground_chassis.svg",
                "Ground",
            ),
            (
                ComponentType::Diode,
                "schottky",
                "diode_schottky.svg",
                "Schottky Diode",
            ),
            (
                ComponentType::Diode,
                "zener",
                "diode_zener.svg",
                "Zener Diode",
            ),
            (
                ComponentType::Diode,
                "tunnel",
                "diode_tunnel.svg",
                "Tunnel Diode",
            ),
            (ComponentType::Diode, "led", "led.svg", "LED"),
            (
                ComponentType::NpnBjt,
                "discrete",
                "bjt_npn_descrete.svg",
                "NPN BJT",
            ),
            (
                ComponentType::PnpBjt,
                "discrete",
                "bjt_pnp_discrete.svg",
                "PNP BJT",
            ),
            (
                ComponentType::Njfet,
                "discrete",
                "jfet_n_chan_discrete.svg",
                "N-JFET",
            ),
            (
                ComponentType::Pjfet,
                "discrete",
                "jfet_p_chan_discrete.svg",
                "P-JFET",
            ),
        ];

        for (component_type, variant_id, filename, name) in variant_mappings {
            let symbol = library.prepare_symbol(*component_type, filename, name, false)?;
            library
                .variant_symbols
                .insert((*component_type, (*variant_id).to_string()), symbol);
        }

        // Load horizontal variants for components that have separate horizontal SVGs
        let horizontal_mappings: &[(ComponentType, &str, &str)] = &[
            (
                ComponentType::VoltageSourceAc,
                "v_src_ac_horizontal.svg",
                "AC Voltage Source",
            ),
            (
                ComponentType::VoltageSourceSin,
                "v_src_ac_horizontal.svg",
                "Sinusoidal Voltage Source",
            ),
        ];

        for (component_type, filename, name) in horizontal_mappings {
            let symbol = library.prepare_symbol(*component_type, filename, name, true)?;
            library.horizontal_symbols.insert(*component_type, symbol);
        }

        Ok(library)
    }

    fn load_all_embedded_assets() -> Result<HashMap<String, Symbol>, SymbolError> {
        let mut assets = HashMap::with_capacity(embedded_symbols::EMBEDDED_SYMBOLS.len());

        for &(filename, svg_data) in embedded_symbols::EMBEDDED_SYMBOLS {
            let mut symbol = parse_svg(svg_data).map_err(|err| {
                SymbolError::ParseError(format!(
                    "Failed to parse embedded symbol asset '{}': {}",
                    filename, err
                ))
            })?;
            symbol.name = filename.to_string();
            assets.insert(filename.to_string(), symbol);
        }

        Ok(assets)
    }

    fn prepare_symbol(
        &self,
        component_type: ComponentType,
        filename: &str,
        name: &str,
        horizontal: bool,
    ) -> Result<Symbol, SymbolError> {
        let mut symbol =
            self.embedded_assets
                .get(filename)
                .cloned()
                .ok_or_else(|| SymbolError::IoError {
                    path: filename.to_string(),
                    message: "embedded symbol asset was not loaded".to_string(),
                })?;

        symbol.name = name.to_string();

        let (target_w, target_h) = component_type.symbol_dimensions();
        if horizontal {
            symbol.target_width = target_h as f32;
            symbol.target_height = target_w as f32;
        } else {
            symbol.target_width = target_w as f32;
            symbol.target_height = target_h as f32;
        }

        add_default_pins(&mut symbol, component_type);
        Ok(symbol)
    }

    /// Load an embedded SVG file by name
    #[cfg(test)]
    pub(crate) fn load_embedded_svg(filename: &str) -> Result<String, SymbolError> {
        embedded_symbols::EMBEDDED_SYMBOLS
            .iter()
            .find_map(|(asset_name, svg)| (*asset_name == filename).then(|| (*svg).to_string()))
            .ok_or_else(|| SymbolError::IoError {
                path: filename.to_string(),
                message: "embedded symbol asset was not found".to_string(),
            })
    }

    /// Get a symbol by component type (O(1) lookup)
    pub fn get(&self, component_type: ComponentType) -> Option<&Symbol> {
        self.symbols.get(&component_type)
    }

    /// Get a parsed embedded asset by filename.
    pub fn get_asset(&self, filename: &str) -> Option<&Symbol> {
        self.embedded_assets.get(filename)
    }

    /// Return all parsed embedded asset filenames.
    pub fn asset_names(&self) -> Vec<String> {
        let mut names: Vec<_> = self.embedded_assets.keys().cloned().collect();
        names.sort();
        names
    }

    /// Get a symbol with rotation awareness.
    /// For components with horizontal variants (like AC voltage source),
    /// returns the horizontal SVG when rotated 90° or 270°, along with the
    /// adjusted rotation to apply to the symbol.
    /// Returns (symbol, adjusted_rotation_degrees).
    pub fn get_with_rotation(
        &self,
        component_type: ComponentType,
        rotation_degrees: i32,
    ) -> Option<(&Symbol, i32)> {
        // Normalize rotation to 0-359
        let normalized = rotation_degrees.rem_euclid(360);

        // For 90° or 270° rotation, use horizontal variant if available
        if (normalized == 90 || normalized == 270)
            && let Some(symbol) = self.horizontal_symbols.get(&component_type)
        {
            // Horizontal SVG is already rotated 90° from vertical.
            // For 90° requested: use horizontal SVG with 0° rotation
            // For 270° requested: use horizontal SVG with 180° rotation
            let adjusted = if normalized == 90 { 0 } else { 180 };
            return Some((symbol, adjusted));
        }

        // Fall back to default symbol with original rotation
        self.symbols
            .get(&component_type)
            .map(|s| (s, rotation_degrees))
    }

    /// Get a symbol with rotation awareness and optional symbol variant override.
    pub fn get_with_rotation_variant(
        &self,
        component_type: ComponentType,
        rotation_degrees: i32,
        variant: Option<&str>,
    ) -> Option<(&Symbol, i32)> {
        let normalized = rotation_degrees.rem_euclid(360);

        if let Some(variant_id) = variant.filter(|variant_id| !variant_id.is_empty()) {
            let variant_key = (component_type, variant_id.to_string());

            if (normalized == 90 || normalized == 270)
                && let Some(symbol) = self.horizontal_variant_symbols.get(&variant_key)
            {
                let adjusted = if normalized == 90 { 0 } else { 180 };
                return Some((symbol, adjusted));
            }

            if let Some(symbol) = self.variant_symbols.get(&variant_key) {
                return Some((symbol, rotation_degrees));
            }
        }

        self.get_with_rotation(component_type, rotation_degrees)
    }

    /// Check if a symbol exists for the given component type
    pub fn contains(&self, component_type: ComponentType) -> bool {
        self.symbols.contains_key(&component_type)
    }

    /// Check whether a parsed embedded asset exists by filename.
    pub fn contains_asset(&self, filename: &str) -> bool {
        self.embedded_assets.contains_key(filename)
    }

    /// Get all loaded component types
    pub fn loaded_types(&self) -> Vec<ComponentType> {
        self.symbols.keys().copied().collect()
    }

    /// Number of loaded symbols
    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    /// Number of parsed embedded SVG asset files.
    pub fn asset_count(&self) -> usize {
        self.embedded_assets.len()
    }

    /// Check if library is empty
    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }
}

// ============================================================================
// Symbol Rendering
// ============================================================================

/// Render a symbol to an egui painter with position, scale, rotation, mirroring, and stroke
pub fn draw_symbol(
    painter: &Painter,
    symbol: &Symbol,
    center: Pos2,
    scale: f32,
    rotation_degrees: i32,
    mirror_h: bool,
    mirror_v: bool,
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

    // Non-uniform scaling to exactly fill target dimensions:
    // This ensures terminal leads reach their expected positions on the grid.
    // For transistors, gate lead must reach -hw and D/S leads must reach ±hh.
    let view_scale_x = symbol.target_width / symbol.width().max(0.001);
    let view_scale_y = symbol.target_height / symbol.height().max(0.001);
    let total_scale_x = scale * view_scale_x;
    let total_scale_y = scale * view_scale_y;

    // Debug: log symbol dimensions
    log::debug!(
        "Symbol '{}' bounds=({:.1},{:.1},{:.1},{:.1}), target=({:.1}x{:.1}), scale=({:.2},{:.2})",
        symbol.name,
        symbol.bounds.0,
        symbol.bounds.1,
        symbol.bounds.2,
        symbol.bounds.3,
        symbol.target_width,
        symbol.target_height,
        total_scale_x,
        total_scale_y
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

                    // Transform point with non-uniform scaling and mirroring
                    let (tx, ty) = transform_point_nonuniform(
                        *x,
                        *y,
                        cx,
                        cy,
                        cos_r,
                        sin_r,
                        total_scale_x,
                        total_scale_y,
                        mirror_h,
                        mirror_v,
                        center,
                    );
                    points.push(Pos2::new(tx, ty));
                    current_pos = (*x, *y);
                }
                PathCommand::LineTo(x, y) => {
                    let (tx, ty) = transform_point_nonuniform(
                        *x,
                        *y,
                        cx,
                        cy,
                        cos_r,
                        sin_r,
                        total_scale_x,
                        total_scale_y,
                        mirror_h,
                        mirror_v,
                        center,
                    );
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
                        let (tx, ty) = transform_point_nonuniform(
                            bx,
                            by,
                            cx,
                            cy,
                            cos_r,
                            sin_r,
                            total_scale_x,
                            total_scale_y,
                            mirror_h,
                            mirror_v,
                            center,
                        );
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

    // Note: Lead lines are handled by the SVG artwork itself.
    // The SVG symbols are designed with lead lines that extend to terminal positions.
    // Uniform scaling preserves these leads proportionally.
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

/// Transform a point with rotation around symbol center and non-uniform scale.
/// Non-uniform scaling is applied BEFORE rotation to ensure the symbol fills
/// exactly its target dimensions. This guarantees terminals land on grid lines.
/// Mirroring is applied BEFORE rotation but AFTER scaling.
#[allow(dead_code)]
fn transform_point_nonuniform(
    x: f32,
    y: f32,
    cx: f32,
    cy: f32,
    cos_r: f32,
    sin_r: f32,
    scale_x: f32,
    scale_y: f32,
    mirror_h: bool,
    mirror_v: bool,
    center: Pos2,
) -> (f32, f32) {
    // Translate to symbol origin
    let dx = x - cx;
    let dy = y - cy;

    // Apply non-uniform scaling BEFORE rotation
    // This ensures the symbol exactly fills its target dimensions
    let sx = dx * scale_x;
    let sy = dy * scale_y;

    // Apply mirroring BEFORE rotation
    // mirror_h flips horizontally (negate x), mirror_v flips vertically (negate y)
    let mx = if mirror_h { -sx } else { sx };
    let my = if mirror_v { -sy } else { sy };

    // Rotate the mirrored and scaled point
    let rx = mx * cos_r - my * sin_r;
    let ry = mx * sin_r + my * cos_r;

    // Translate to screen position
    (center.x + rx, center.y + ry)
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
    // center() uses bounds-based calculation, so paths must be normalized to match
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
    let (norm_width, norm_height) = if min_x != f32::MAX {
        (max_x - offset_x, max_y - offset_y)
    } else {
        (40.0, 40.0) // Default fallback
    };

    // Get viewBox
    let view_box = tree.size();
    let vb = (0.0, 0.0, view_box.width(), view_box.height());

    // Scale paths to match viewBox coordinate space
    // usvg may extract paths at different DPI than viewBox dimensions
    let scale_x = if norm_width > 0.0 {
        vb.2 / norm_width
    } else {
        1.0
    };
    let scale_y = if norm_height > 0.0 {
        vb.3 / norm_height
    } else {
        1.0
    };

    log::info!(
        "SCALING PATHS: ({:.3}, {:.3}) - norm=({:.1}x{:.1}) viewBox=({:.1}x{:.1})",
        scale_x,
        scale_y,
        norm_width,
        norm_height,
        vb.2,
        vb.3
    );

    for path in &mut paths {
        for cmd in &mut path.commands {
            match cmd {
                PathCommand::MoveTo(x, y) => {
                    *x *= scale_x;
                    *y *= scale_y;
                }
                PathCommand::LineTo(x, y) => {
                    *x *= scale_x;
                    *y *= scale_y;
                }
                PathCommand::CurveTo { ctrl1, ctrl2, end } => {
                    ctrl1.0 *= scale_x;
                    ctrl1.1 *= scale_y;
                    ctrl2.0 *= scale_x;
                    ctrl2.1 *= scale_y;
                    end.0 *= scale_x;
                    end.1 *= scale_y;
                }
                PathCommand::Close => {}
            }
        }
    }

    // Set final bounds to viewBox dimensions (paths are now in viewBox space)
    let final_bounds = (0.0, 0.0, vb.2, vb.3);

    log::debug!(
        "Final bounds (viewBox space): ({:.1},{:.1},{:.1},{:.1})",
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
        ComponentType::Resistor
        | ComponentType::Capacitor
        | ComponentType::Inductor
        | ComponentType::SaturableInductor => {
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
        ComponentType::Transformer => {
            let pin_inset = 10.0;
            symbol.pins = vec![
                SymbolPin {
                    name: "P1".to_string(),
                    position: (cx - w + pin_inset, cy - h),
                    direction: PinDirection::Left,
                },
                SymbolPin {
                    name: "P2".to_string(),
                    position: (cx - w + pin_inset, cy + h),
                    direction: PinDirection::Left,
                },
                SymbolPin {
                    name: "S1".to_string(),
                    position: (cx + w - pin_inset, cy - h),
                    direction: PinDirection::Right,
                },
                SymbolPin {
                    name: "S2".to_string(),
                    position: (cx + w - pin_inset, cy + h),
                    direction: PinDirection::Right,
                },
            ];
        }
        ComponentType::VoltageSource
        | ComponentType::VoltageSourceAc
        | ComponentType::VoltageSourcePulse
        | ComponentType::VoltageSourceSin
        | ComponentType::VoltageSourcePwl
        | ComponentType::VoltageSourceExp
        | ComponentType::VoltageSourceSffm
        | ComponentType::CurrentSource
        | ComponentType::CurrentSourceAc
        | ComponentType::CurrentSourcePulse
        | ComponentType::CurrentSourceSin
        | ComponentType::CurrentSourcePwl
        | ComponentType::CurrentSourceExp
        | ComponentType::CurrentSourceNoise => {
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
        ComponentType::Nmos
        | ComponentType::Pmos
        | ComponentType::NVdmos
        | ComponentType::PVdmos => {
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
        ComponentType::Njfet | ComponentType::Pjfet => {
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
        let (tx, ty) = transform_point_nonuniform(
            10.0,
            20.0,
            5.0,
            10.0,
            1.0,
            0.0,
            1.0,   // scale_x
            1.0,   // scale_y
            false, // mirror_h
            false, // mirror_v
            Pos2::new(100.0, 100.0),
        );

        // Point (10,20) relative to center (5,10) = offset (5, 10)
        // No rotation (cos=1, sin=0), scale (1, 1), center at (100, 100)
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
        let (tx, ty) = transform_point_nonuniform(
            10.0,
            0.0,
            0.0,
            0.0,
            cos_r,
            sin_r,
            1.0,
            1.0,
            false, // mirror_h
            false, // mirror_v
            Pos2::new(0.0, 0.0),
        );

        // After 90 degree rotation: (x,y) -> (-y, x) conceptually
        // With non-uniform scaling BEFORE rotation:
        // 1. Scale: sx = 10*1 = 10, sy = 0*1 = 0
        // 2. Rotate: rx = sx*cos - sy*sin = 10*0 - 0*1 = 0
        //            ry = sx*sin + sy*cos = 10*1 + 0*0 = 10
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
        // Test that we can load the baseline embedded symbol set.
        let library = SymbolLibrary::load_embedded();
        assert!(
            library.is_ok(),
            "Failed to load symbol library: {:?}",
            library.err()
        );

        let library = library.unwrap();
        assert!(
            library.len() >= 11,
            "Should have at least the baseline symbol set loaded"
        );

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
    fn test_load_all_embedded_symbol_assets() {
        let library = SymbolLibrary::load_embedded().expect("Should load library");
        let asset_names = library.asset_names();

        assert!(
            asset_names.len() >= 54,
            "Expected all embedded SVG assets to be parsed, got {}",
            asset_names.len()
        );
        assert!(library.contains_asset("diode_zener.svg"));
        assert!(library.contains_asset("battery.svg"));
        assert!(library.contains_asset("transformer_symmetrical.svg"));
    }

    #[test]
    fn test_symbol_target_dimensions_commercial_grade() {
        // Commercial-grade verification: each component type should have appropriate
        // target dimensions per EDA standards (grid-aligned, multiples of 20)
        let library = SymbolLibrary::load_embedded().expect("Should load library");

        // Verify passive components have horizontal orientation (wider than tall)
        // Dimensions are 40x20 to ensure terminals at ±20 land on major grid lines
        let resistor = library.get(crate::state::ComponentType::Resistor).unwrap();
        assert_eq!(
            resistor.target_width, 40.0,
            "Resistor should have 40.0 target width"
        );
        assert_eq!(
            resistor.target_height, 20.0,
            "Resistor should have 20.0 target height"
        );
        assert!(
            resistor.target_width > resistor.target_height,
            "Resistor should be horizontal"
        );

        let inductor = library.get(crate::state::ComponentType::Inductor).unwrap();
        assert_eq!(
            inductor.target_width, 40.0,
            "Inductor should have 40.0 target width"
        );
        assert_eq!(
            inductor.target_height, 20.0,
            "Inductor should have 20.0 target height"
        );

        // Verify sources have vertical orientation (taller than wide)
        // Dimensions are 28x40 to match SVG aspect ratio while keeping terminals on grid
        let vsrc = library
            .get(crate::state::ComponentType::VoltageSource)
            .unwrap();
        assert_eq!(
            vsrc.target_width, 28.0,
            "VoltageSource should have 28.0 target width"
        );
        assert_eq!(
            vsrc.target_height, 40.0,
            "VoltageSource should have 40.0 target height"
        );
        assert!(
            vsrc.target_height > vsrc.target_width,
            "VoltageSource should be vertical"
        );

        // Verify transistors have 40x80 dimensions for consistent sizing with grid alignment
        // G at ±20, D/S at ±40 - all on major grid
        let nmos = library.get(crate::state::ComponentType::Nmos).unwrap();
        assert_eq!(
            nmos.target_width, 40.0,
            "NMOS should have 40.0 target width"
        );
        assert_eq!(
            nmos.target_height, 80.0,
            "NMOS should have 80.0 target height"
        );

        let npn = library.get(crate::state::ComponentType::NpnBjt).unwrap();
        assert_eq!(
            npn.target_width, 40.0,
            "NPN BJT should have 40.0 target width"
        );
        assert_eq!(
            npn.target_height, 80.0,
            "NPN BJT should have 80.0 target height"
        );

        // Verify ground is 20x20 for grid alignment
        let ground = library.get(crate::state::ComponentType::Ground).unwrap();
        assert_eq!(
            ground.target_width, 20.0,
            "Ground should have 20.0 target width"
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

    #[test]
    fn test_high_confidence_symbol_hookups_are_loaded() {
        let library = SymbolLibrary::load_embedded().expect("Should load library");

        let expected_types = [
            ComponentType::Transformer,
            ComponentType::SaturableInductor,
            ComponentType::VoltageSourcePulse,
            ComponentType::VoltageSourceSin,
            ComponentType::VoltageSourcePwl,
            ComponentType::VoltageSourceExp,
            ComponentType::VoltageSourceSffm,
            ComponentType::CurrentSourceAc,
            ComponentType::CurrentSourcePulse,
            ComponentType::CurrentSourceSin,
            ComponentType::CurrentSourcePwl,
            ComponentType::CurrentSourceExp,
            ComponentType::CurrentSourceNoise,
            ComponentType::Njfet,
            ComponentType::Pjfet,
            ComponentType::NVdmos,
            ComponentType::PVdmos,
        ];

        for component_type in expected_types {
            assert!(
                library.contains(component_type),
                "Expected symbol mapping for {:?}",
                component_type
            );

            let symbol = library
                .get(component_type)
                .expect("Mapped symbol should be retrievable");
            assert!(
                !symbol.paths.is_empty(),
                "Mapped symbol for {:?} should have renderable paths",
                component_type
            );
        }
    }

    #[test]
    fn test_voltage_source_sin_uses_ac_horizontal_variant() {
        let library = SymbolLibrary::load_embedded().expect("Should load library");

        let vertical = library
            .get(ComponentType::VoltageSourceSin)
            .expect("Vertical symbol should exist");
        assert_eq!(vertical.target_width, 28.0);
        assert_eq!(vertical.target_height, 40.0);

        let (horizontal_90, adjusted_90) = library
            .get_with_rotation(ComponentType::VoltageSourceSin, 90)
            .expect("Horizontal symbol should exist for 90 degree rotation");
        assert_eq!(adjusted_90, 0);
        assert_eq!(horizontal_90.target_width, 40.0);
        assert_eq!(horizontal_90.target_height, 28.0);

        let (horizontal_270, adjusted_270) = library
            .get_with_rotation(ComponentType::VoltageSourceSin, 270)
            .expect("Horizontal symbol should exist for 270 degree rotation");
        assert_eq!(adjusted_270, 180);
        assert_eq!(horizontal_270.target_width, 40.0);
        assert_eq!(horizontal_270.target_height, 28.0);
    }

    #[test]
    fn test_added_symbol_variants_have_expected_default_pins() {
        let library = SymbolLibrary::load_embedded().expect("Should load library");

        for source in [
            ComponentType::VoltageSourceSin,
            ComponentType::VoltageSourcePwl,
            ComponentType::CurrentSourceSin,
            ComponentType::CurrentSourceNoise,
        ] {
            let symbol = library
                .get(source)
                .expect("Source symbol should be available");
            assert_eq!(
                symbol.pins.len(),
                2,
                "Source {:?} should have two default pins",
                source
            );
            assert_eq!(symbol.pins[0].name, "+");
            assert_eq!(symbol.pins[1].name, "-");
        }

        for transistor in [ComponentType::Njfet, ComponentType::Pjfet] {
            let symbol = library
                .get(transistor)
                .expect("JFET symbol should be available");
            assert_eq!(symbol.pins.len(), 3);
            assert_eq!(symbol.pins[0].name, "G");
            assert_eq!(symbol.pins[1].name, "D");
            assert_eq!(symbol.pins[2].name, "S");
        }

        let transformer = library
            .get(ComponentType::Transformer)
            .expect("Transformer symbol should be available");
        let (cx, cy) = transformer.center();
        let half_width = transformer.width() / 2.0;
        let half_height = transformer.height() / 2.0;
        assert_eq!(transformer.pins.len(), 4);
        assert_eq!(transformer.pins[0].name, "P1");
        assert_eq!(transformer.pins[1].name, "P2");
        assert_eq!(transformer.pins[2].name, "S1");
        assert_eq!(transformer.pins[3].name, "S2");
        assert_eq!(
            transformer.pins[0].position,
            (cx - half_width + 10.0, cy - half_height)
        );
        assert_eq!(
            transformer.pins[1].position,
            (cx - half_width + 10.0, cy + half_height)
        );
        assert_eq!(
            transformer.pins[2].position,
            (cx + half_width - 10.0, cy - half_height)
        );
        assert_eq!(
            transformer.pins[3].position,
            (cx + half_width - 10.0, cy + half_height)
        );
    }

    #[test]
    fn test_variant_symbol_lookup_uses_named_variant() {
        let library = SymbolLibrary::load_embedded().expect("Should load library");

        let (default_symbol, _) = library
            .get_with_rotation_variant(ComponentType::Diode, 0, None)
            .expect("Default diode symbol should resolve");
        let (zener_symbol, _) = library
            .get_with_rotation_variant(ComponentType::Diode, 0, Some("zener"))
            .expect("Zener diode symbol should resolve");

        assert_eq!(default_symbol.name, "Diode");
        assert_eq!(zener_symbol.name, "Zener Diode");
        assert_eq!(zener_symbol.target_width, default_symbol.target_width);
        assert_eq!(zener_symbol.target_height, default_symbol.target_height);
    }
}
