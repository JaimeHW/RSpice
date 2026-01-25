//! Geometry Generation
//!
//! Commercial-grade GPU vertex generation for schematic elements.
//! Each symbol is designed to match standard IEEE/ANSI schematic symbols
//! used in professional EDA tools like Cadence and Altium.
//!
//! # Architecture
//!
//! All symbols are generated as triangulated vertex arrays, ready for GPU upload.
//! Symbols are designed for instanced rendering:
//! - Centered at origin
//! - Horizontal orientation (R0)
//! - Consistent terminal positions at ±2 grid units
//!
//! The vertex shader applies per-instance transforms (position, rotation, scale).

use super::vertex::{Vertex, WireVertex};

/// Color constants matching professional schematic themes
///
/// These colors are designed for maximum contrast on dark backgrounds
/// while remaining easy on the eyes for extended viewing.
pub mod colors {
    /// Standard wire color (green like oscilloscope traces)
    pub const WIRE_NORMAL: [f32; 4] = [0.0, 0.8, 0.0, 1.0];
    /// Selected wire color (blue for visibility)
    pub const WIRE_SELECTED: [f32; 4] = [0.3, 0.6, 1.0, 1.0];
    /// Component outline and body
    pub const COMPONENT_BODY: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
    /// Component fill (slightly darker)
    pub const COMPONENT_FILL: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
    /// Junction dot color
    pub const JUNCTION_NORMAL: [f32; 4] = [0.0, 0.8, 0.0, 1.0];
    /// Selected junction
    pub const JUNCTION_SELECTED: [f32; 4] = [0.3, 0.6, 1.0, 1.0];
    /// Background grid
    pub const BACKGROUND: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
    /// Source positive terminal (red)
    pub const SOURCE_POSITIVE: [f32; 4] = [0.9, 0.3, 0.3, 1.0];
    /// Source negative terminal
    pub const SOURCE_NEGATIVE: [f32; 4] = [0.3, 0.3, 0.9, 1.0];
    /// Semiconductor (cyan for visibility)
    pub const SEMICONDUCTOR: [f32; 4] = [0.3, 0.8, 0.8, 1.0];
    /// Digital logic (purple)
    pub const DIGITAL: [f32; 4] = [0.7, 0.4, 0.9, 1.0];
}

// =============================================================================
// Common Geometry Helpers
// =============================================================================

/// Unit quad vertices for instanced rendering (centered, size 2x2)
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct QuadVertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
}

/// Generate unit quad vertices
pub fn unit_quad_vertices() -> [QuadVertex; 6] {
    [
        QuadVertex { position: [-1.0, -1.0], uv: [0.0, 1.0] },
        QuadVertex { position: [1.0, -1.0], uv: [1.0, 1.0] },
        QuadVertex { position: [1.0, 1.0], uv: [1.0, 0.0] },
        QuadVertex { position: [-1.0, -1.0], uv: [0.0, 1.0] },
        QuadVertex { position: [1.0, 1.0], uv: [1.0, 0.0] },
        QuadVertex { position: [-1.0, 1.0], uv: [0.0, 0.0] },
    ]
}

/// Generate grid quad vertices
pub fn grid_quad_vertices(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> [[f32; 2]; 6] {
    [
        [min_x, min_y], [max_x, min_y], [max_x, max_y],
        [min_x, min_y], [max_x, max_y], [min_x, max_y],
    ]
}

/// Create a horizontal line segment as a quad
fn line_h(x1: f32, x2: f32, y: f32, thickness: f32, color: [f32; 4]) -> [Vertex; 6] {
    let t = thickness / 2.0;
    [
        Vertex::new(x1, y - t, color), Vertex::new(x2, y - t, color), Vertex::new(x2, y + t, color),
        Vertex::new(x1, y - t, color), Vertex::new(x2, y + t, color), Vertex::new(x1, y + t, color),
    ]
}

/// Create a vertical line segment as a quad
fn line_v(x: f32, y1: f32, y2: f32, thickness: f32, color: [f32; 4]) -> [Vertex; 6] {
    let t = thickness / 2.0;
    [
        Vertex::new(x - t, y1, color), Vertex::new(x + t, y1, color), Vertex::new(x + t, y2, color),
        Vertex::new(x - t, y1, color), Vertex::new(x + t, y2, color), Vertex::new(x - t, y2, color),
    ]
}

/// Create a diagonal line as a quad
fn line_diag(x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: [f32; 4]) -> [Vertex; 6] {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let len = (dx * dx + dy * dy).sqrt().max(0.001);
    let nx = -dy / len * thickness / 2.0;
    let ny = dx / len * thickness / 2.0;

    [
        Vertex::new(x1 + nx, y1 + ny, color), Vertex::new(x1 - nx, y1 - ny, color),
        Vertex::new(x2 + nx, y2 + ny, color), Vertex::new(x1 - nx, y1 - ny, color),
        Vertex::new(x2 - nx, y2 - ny, color), Vertex::new(x2 + nx, y2 + ny, color),
    ]
}

/// Create a filled triangle
fn triangle(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, color: [f32; 4]) -> [Vertex; 3] {
    [
        Vertex::new(x1, y1, color),
        Vertex::new(x2, y2, color),
        Vertex::new(x3, y3, color),
    ]
}

/// Create a filled rectangle
fn rect(x1: f32, y1: f32, x2: f32, y2: f32, color: [f32; 4]) -> [Vertex; 6] {
    [
        Vertex::new(x1, y1, color), Vertex::new(x2, y1, color), Vertex::new(x2, y2, color),
        Vertex::new(x1, y1, color), Vertex::new(x2, y2, color), Vertex::new(x1, y2, color),
    ]
}

/// Create a circle approximation (n-gon, filled)
fn circle_filled(cx: f32, cy: f32, radius: f32, segments: usize, color: [f32; 4]) -> Vec<Vertex> {
    let mut verts = Vec::with_capacity(segments * 3);
    let step = std::f32::consts::TAU / segments as f32;

    for i in 0..segments {
        let a1 = i as f32 * step;
        let a2 = (i + 1) as f32 * step;
        verts.push(Vertex::new(cx, cy, color));
        verts.push(Vertex::new(cx + radius * a1.cos(), cy + radius * a1.sin(), color));
        verts.push(Vertex::new(cx + radius * a2.cos(), cy + radius * a2.sin(), color));
    }
    verts
}

/// Create a circle outline (ring)
fn circle_outline(cx: f32, cy: f32, radius: f32, thickness: f32, segments: usize, color: [f32; 4]) -> Vec<Vertex> {
    let mut verts = Vec::with_capacity(segments * 6);
    let step = std::f32::consts::TAU / segments as f32;
    let r_inner = radius - thickness / 2.0;
    let r_outer = radius + thickness / 2.0;

    for i in 0..segments {
        let a1 = i as f32 * step;
        let a2 = (i + 1) as f32 * step;

        let (s1, c1) = (a1.sin(), a1.cos());
        let (s2, c2) = (a2.sin(), a2.cos());

        // Inner edge to outer edge quad
        verts.push(Vertex::new(cx + r_inner * c1, cy + r_inner * s1, color));
        verts.push(Vertex::new(cx + r_outer * c1, cy + r_outer * s1, color));
        verts.push(Vertex::new(cx + r_outer * c2, cy + r_outer * s2, color));

        verts.push(Vertex::new(cx + r_inner * c1, cy + r_inner * s1, color));
        verts.push(Vertex::new(cx + r_outer * c2, cy + r_outer * s2, color));
        verts.push(Vertex::new(cx + r_inner * c2, cy + r_inner * s2, color));
    }
    verts
}

/// Create a half-circle (arc) outlined
fn arc_outline(cx: f32, cy: f32, radius: f32, start_angle: f32, end_angle: f32, thickness: f32, segments: usize, color: [f32; 4]) -> Vec<Vertex> {
    let mut verts = Vec::with_capacity(segments * 6);
    let angle_span = end_angle - start_angle;
    let step = angle_span / segments as f32;
    let r_inner = radius - thickness / 2.0;
    let r_outer = radius + thickness / 2.0;

    for i in 0..segments {
        let a1 = start_angle + i as f32 * step;
        let a2 = start_angle + (i + 1) as f32 * step;

        let (s1, c1) = (a1.sin(), a1.cos());
        let (s2, c2) = (a2.sin(), a2.cos());

        verts.push(Vertex::new(cx + r_inner * c1, cy + r_inner * s1, color));
        verts.push(Vertex::new(cx + r_outer * c1, cy + r_outer * s1, color));
        verts.push(Vertex::new(cx + r_outer * c2, cy + r_outer * s2, color));

        verts.push(Vertex::new(cx + r_inner * c1, cy + r_inner * s1, color));
        verts.push(Vertex::new(cx + r_outer * c2, cy + r_outer * s2, color));
        verts.push(Vertex::new(cx + r_inner * c2, cy + r_inner * s2, color));
    }
    verts
}

// =============================================================================
// Selection Box Geometry
// =============================================================================

/// Generate selection box border vertices (hollow rectangle)
///
/// Creates 4 border quads (top, bottom, left, right) for the selection overlay.
/// The fill is semi-transparent and rendered separately.
pub fn selection_box_border_vertices(
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32,
    border_width: f32,
    color: [f32; 4],
) -> Vec<Vertex> {
    // Normalize coordinates (handle inverted selection)
    let x1 = min_x.min(max_x);
    let y1 = min_y.min(max_y);
    let x2 = min_x.max(max_x);
    let y2 = min_y.max(max_y);
    
    let mut verts = Vec::with_capacity(24); // 4 sides × 6 vertices
    
    // Top border
    verts.extend_from_slice(&rect(x1, y2 - border_width, x2, y2, color));
    
    // Bottom border
    verts.extend_from_slice(&rect(x1, y1, x2, y1 + border_width, color));
    
    // Left border (between top and bottom)
    verts.extend_from_slice(&rect(x1, y1 + border_width, x1 + border_width, y2 - border_width, color));
    
    // Right border (between top and bottom)
    verts.extend_from_slice(&rect(x2 - border_width, y1 + border_width, x2, y2 - border_width, color));
    
    verts
}

/// Generate selection box fill vertices (semi-transparent interior)
pub fn selection_box_fill_vertices(
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32,
    color: [f32; 4],
) -> [Vertex; 6] {
    let x1 = min_x.min(max_x);
    let y1 = min_y.min(max_y);
    let x2 = min_x.max(max_x);
    let y2 = min_y.max(max_y);
    
    rect(x1, y1, x2, y2, color)
}

// =============================================================================
// Wire Geometry
// =============================================================================

/// Generate wire geometry as thick line segments
///
/// Each wire segment becomes a quad (2 triangles, 6 vertices)
/// with proper normals for thickness expansion in the shader.
pub fn generate_wire_vertices(
    points: &[[f32; 2]],
    color: [f32; 4],
    thickness: f32,
) -> Vec<WireVertex> {
    if points.len() < 2 {
        return Vec::new();
    }

    let mut vertices = Vec::with_capacity((points.len() - 1) * 6);

    for window in points.windows(2) {
        let p0 = window[0];
        let p1 = window[1];

        let dx = p1[0] - p0[0];
        let dy = p1[1] - p0[1];
        let len = (dx * dx + dy * dy).sqrt();

        if len < 0.0001 {
            continue;
        }

        let nx = -dy / len;
        let ny = dx / len;

        let v0 = WireVertex::new(p0[0], p0[1], nx, ny, color, thickness);
        let v1 = WireVertex::new(p0[0], p0[1], -nx, -ny, color, thickness);
        let v2 = WireVertex::new(p1[0], p1[1], nx, ny, color, thickness);
        let v3 = WireVertex::new(p1[0], p1[1], -nx, -ny, color, thickness);

        vertices.push(v0);
        vertices.push(v1);
        vertices.push(v2);
        vertices.push(v1);
        vertices.push(v3);
        vertices.push(v2);
    }

    vertices
}

// =============================================================================
// Passive Components
// =============================================================================

/// Generate resistor symbol (IEEE zigzag style)
pub fn resistor_symbol_vertices() -> Vec<Vertex> {
    let color = colors::COMPONENT_BODY;
    let mut verts = Vec::new();

    // Leads
    verts.extend(line_h(-5.0, -3.0, 0.0, 0.15, color));
    verts.extend(line_h(3.0, 5.0, 0.0, 0.15, color));

    // Zigzag body (6 segments)
    let points: [(f32, f32); 8] = [
        (-3.0, 0.0), (-2.0, 1.0), (-1.0, -1.0), (0.0, 1.0),
        (1.0, -1.0), (2.0, 1.0), (3.0, 0.0), (3.0, 0.0),
    ];
    for i in 0..6 {
        verts.extend(line_diag(points[i].0, points[i].1, points[i+1].0, points[i+1].1, 0.15, color));
    }

    verts
}

/// Generate capacitor symbol (two parallel plates)
pub fn capacitor_symbol_vertices() -> Vec<Vertex> {
    let color = colors::COMPONENT_BODY;
    let mut verts = Vec::new();

    // Leads
    verts.extend(line_h(-5.0, -0.5, 0.0, 0.15, color));
    verts.extend(line_h(0.5, 5.0, 0.0, 0.15, color));

    // Plates
    verts.extend(line_v(-0.5, -1.5, 1.5, 0.2, color));
    verts.extend(line_v(0.5, -1.5, 1.5, 0.2, color));

    verts
}

/// Generate polarized capacitor symbol (with curved plate)
pub fn polarized_capacitor_symbol_vertices() -> Vec<Vertex> {
    let color = colors::COMPONENT_BODY;
    let mut verts = Vec::new();

    // Leads
    verts.extend(line_h(-5.0, -0.5, 0.0, 0.15, color));
    verts.extend(line_h(0.5, 5.0, 0.0, 0.15, color));

    // Straight plate (positive)
    verts.extend(line_v(-0.5, -1.5, 1.5, 0.2, color));

    // Curved plate approximation (arc)
    verts.extend(arc_outline(0.8, 0.0, 0.5, std::f32::consts::FRAC_PI_2, -std::f32::consts::FRAC_PI_2, 0.15, 8, color));

    // Plus sign
    verts.extend(line_h(-2.0, -1.2, 1.0, 0.1, color));
    verts.extend(line_v(-1.6, 0.6, 1.4, 0.1, color));

    verts
}

/// Generate inductor symbol (coil)
pub fn inductor_symbol_vertices() -> Vec<Vertex> {
    let color = colors::COMPONENT_BODY;
    let mut verts = Vec::new();

    // Leads
    verts.extend(line_h(-5.0, -3.0, 0.0, 0.15, color));
    verts.extend(line_h(3.0, 5.0, 0.0, 0.15, color));

    // Coil humps (4 semi-circles)
    for i in 0..4 {
        let cx = -2.25 + i as f32 * 1.5;
        verts.extend(arc_outline(cx, 0.0, 0.75, 0.0, std::f32::consts::PI, 0.15, 8, color));
    }

    verts
}

// =============================================================================
// Semiconductors
// =============================================================================

/// Generate diode symbol (triangle with bar)
pub fn diode_symbol_vertices() -> Vec<Vertex> {
    let color = colors::SEMICONDUCTOR;
    let mut verts = Vec::new();

    // Leads
    verts.extend(line_h(-5.0, -1.0, 0.0, 0.15, color));
    verts.extend(line_h(1.0, 5.0, 0.0, 0.15, color));

    // Triangle (anode pointing right)
    verts.extend(triangle(-1.0, -1.2, -1.0, 1.2, 1.0, 0.0, color));

    // Cathode bar
    verts.extend(line_v(1.0, -1.2, 1.2, 0.2, color));

    verts
}

/// Generate NPN BJT symbol
pub fn npn_bjt_symbol_vertices() -> Vec<Vertex> {
    let color = colors::SEMICONDUCTOR;
    let mut verts = Vec::new();

    // Base lead
    verts.extend(line_h(-5.0, -1.0, 0.0, 0.15, color));

    // Base line (vertical)
    verts.extend(line_v(-1.0, -1.5, 1.5, 0.2, color));

    // Emitter (with arrow)
    verts.extend(line_diag(-1.0, -0.8, 1.5, -2.5, 0.15, color));
    // Arrow head
    verts.extend(triangle(1.5, -2.5, 0.8, -2.0, 1.0, -1.7, color));

    // Collector
    verts.extend(line_diag(-1.0, 0.8, 1.5, 2.5, 0.15, color));

    // Collector lead
    verts.extend(line_v(1.5, 2.5, 5.0, 0.15, color));
    // Emitter lead
    verts.extend(line_v(1.5, -2.5, -5.0, 0.15, color));

    verts
}

/// Generate PNP BJT symbol
pub fn pnp_bjt_symbol_vertices() -> Vec<Vertex> {
    let color = colors::SEMICONDUCTOR;
    let mut verts = Vec::new();

    // Base lead
    verts.extend(line_h(-5.0, -1.0, 0.0, 0.15, color));

    // Base line (vertical)
    verts.extend(line_v(-1.0, -1.5, 1.5, 0.2, color));

    // Emitter (with arrow pointing in)
    verts.extend(line_diag(-1.0, -0.8, 1.5, -2.5, 0.15, color));
    // Arrow head (pointing inward)
    verts.extend(triangle(-0.8, -0.9, -0.2, -1.5, -0.5, -0.5, color));

    // Collector
    verts.extend(line_diag(-1.0, 0.8, 1.5, 2.5, 0.15, color));

    // Leads
    verts.extend(line_v(1.5, 2.5, 5.0, 0.15, color));
    verts.extend(line_v(1.5, -2.5, -5.0, 0.15, color));

    verts
}

/// Generate NMOS symbol (enhancement mode)
pub fn nmos_symbol_vertices() -> Vec<Vertex> {
    let color = colors::SEMICONDUCTOR;
    let mut verts = Vec::new();

    // Gate lead
    verts.extend(line_h(-5.0, -2.0, 0.0, 0.15, color));
    // Gate line (vertical)
    verts.extend(line_v(-2.0, -1.5, 1.5, 0.15, color));

    // Channel with gap (3 dashed segments)
    verts.extend(line_v(-1.0, -1.5, -0.5, 0.15, color));
    verts.extend(line_v(-1.0, -0.3, 0.3, 0.15, color));
    verts.extend(line_v(-1.0, 0.5, 1.5, 0.15, color));

    // Drain (top)
    verts.extend(line_h(-1.0, 1.5, 1.0, 0.15, color));
    verts.extend(line_v(1.5, 1.0, 5.0, 0.15, color));

    // Source (bottom)
    verts.extend(line_h(-1.0, 1.5, -1.0, 0.15, color));
    verts.extend(line_v(1.5, -1.0, -5.0, 0.15, color));

    // Bulk connection (center)
    verts.extend(line_h(-1.0, 1.5, 0.0, 0.15, color));

    // Arrow on source (pointing in for N-type)
    verts.extend(triangle(0.0, 0.0, 0.5, 0.3, 0.5, -0.3, color));

    verts
}

/// Generate PMOS symbol
pub fn pmos_symbol_vertices() -> Vec<Vertex> {
    let color = colors::SEMICONDUCTOR;
    let mut verts = Vec::new();

    // Similar to NMOS but with bubble on gate and arrow pointing out
    verts.extend(line_h(-5.0, -2.5, 0.0, 0.15, color));
    // Bubble (circle) on gate
    verts.extend(circle_outline(-2.25, 0.0, 0.25, 0.1, 12, color));
    // Gate line
    verts.extend(line_v(-2.0, -1.5, 1.5, 0.15, color));

    // Channel
    verts.extend(line_v(-1.0, -1.5, 1.5, 0.15, color));

    // Drain/Source
    verts.extend(line_h(-1.0, 1.5, 1.0, 0.15, color));
    verts.extend(line_v(1.5, 1.0, 5.0, 0.15, color));
    verts.extend(line_h(-1.0, 1.5, -1.0, 0.15, color));
    verts.extend(line_v(1.5, -1.0, -5.0, 0.15, color));
    verts.extend(line_h(-1.0, 1.5, 0.0, 0.15, color));

    // Arrow pointing out
    verts.extend(triangle(0.5, 0.0, 0.0, 0.3, 0.0, -0.3, color));

    verts
}

// =============================================================================
// Sources
// =============================================================================

/// Generate DC voltage source symbol (circle with +/-)
pub fn voltage_source_symbol_vertices() -> Vec<Vertex> {
    let color = colors::COMPONENT_BODY;
    let mut verts = Vec::new();

    // Circle
    verts.extend(circle_outline(0.0, 0.0, 2.0, 0.15, 24, color));

    // Leads
    verts.extend(line_v(0.0, 2.0, 5.0, 0.15, color));
    verts.extend(line_v(0.0, -2.0, -5.0, 0.15, color));

    // Plus sign (top)
    verts.extend(line_h(-0.4, 0.4, 1.0, 0.1, colors::SOURCE_POSITIVE));
    verts.extend(line_v(0.0, 0.6, 1.4, 0.1, colors::SOURCE_POSITIVE));

    // Minus sign (bottom)
    verts.extend(line_h(-0.4, 0.4, -1.0, 0.1, colors::SOURCE_NEGATIVE));

    verts
}

/// Generate DC current source symbol (circle with arrow)
pub fn current_source_symbol_vertices() -> Vec<Vertex> {
    let color = colors::COMPONENT_BODY;
    let mut verts = Vec::new();

    // Circle
    verts.extend(circle_outline(0.0, 0.0, 2.0, 0.15, 24, color));

    // Leads
    verts.extend(line_v(0.0, 2.0, 5.0, 0.15, color));
    verts.extend(line_v(0.0, -2.0, -5.0, 0.15, color));

    // Arrow (pointing up)
    verts.extend(line_v(0.0, -1.2, 1.2, 0.15, color));
    verts.extend(triangle(0.0, 1.5, -0.4, 0.8, 0.4, 0.8, color));

    verts
}

/// Generate AC source symbol (circle with sine wave)
pub fn ac_source_symbol_vertices() -> Vec<Vertex> {
    let color = colors::COMPONENT_BODY;
    let mut verts = Vec::new();

    // Circle
    verts.extend(circle_outline(0.0, 0.0, 2.0, 0.15, 24, color));

    // Leads
    verts.extend(line_v(0.0, 2.0, 5.0, 0.15, color));
    verts.extend(line_v(0.0, -2.0, -5.0, 0.15, color));

    // Sine wave inside
    verts.extend(arc_outline(-0.5, 0.0, 0.5, 0.0, std::f32::consts::PI, 0.1, 8, color));
    verts.extend(arc_outline(0.5, 0.0, 0.5, std::f32::consts::PI, std::f32::consts::TAU, 0.1, 8, color));

    verts
}

/// Generate controlled source symbol (diamond shape)
pub fn controlled_source_symbol_vertices() -> Vec<Vertex> {
    let color = colors::COMPONENT_BODY;
    let mut verts = Vec::new();

    // Diamond outline
    let size = 2.0;
    verts.extend(line_diag(0.0, -size, -size, 0.0, 0.15, color));
    verts.extend(line_diag(-size, 0.0, 0.0, size, 0.15, color));
    verts.extend(line_diag(0.0, size, size, 0.0, 0.15, color));
    verts.extend(line_diag(size, 0.0, 0.0, -size, 0.15, color));

    // Output leads (left)
    verts.extend(line_h(-size, -5.0, 0.5, 0.15, color));
    verts.extend(line_h(-size, -5.0, -0.5, 0.15, color));

    // Control leads (right)
    verts.extend(line_h(size, 5.0, 0.5, 0.15, color));
    verts.extend(line_h(size, 5.0, -0.5, 0.15, color));

    verts
}

// =============================================================================
// Special Symbols
// =============================================================================

/// Generate ground symbol (3 horizontal lines)
pub fn ground_symbol_vertices() -> Vec<Vertex> {
    let color = colors::COMPONENT_BODY;
    let mut verts = Vec::new();

    // Vertical lead
    verts.extend(line_v(0.0, 0.0, 2.0, 0.15, color));

    // Three horizontal lines
    verts.extend(line_h(-1.5, 1.5, 0.0, 0.15, color));
    verts.extend(line_h(-1.0, 1.0, -0.5, 0.15, color));
    verts.extend(line_h(-0.5, 0.5, -1.0, 0.15, color));

    verts
}

// =============================================================================
// Digital Logic Gates
// =============================================================================

/// Generate inverter/buffer symbol (triangle with optional bubble)
pub fn inverter_symbol_vertices(with_bubble: bool) -> Vec<Vertex> {
    let color = colors::DIGITAL;
    let mut verts = Vec::new();

    // Input lead
    verts.extend(line_h(-5.0, -2.0, 0.0, 0.15, color));

    // Triangle body
    verts.extend(line_diag(-2.0, -1.5, 2.0, 0.0, 0.15, color));
    verts.extend(line_diag(-2.0, 1.5, 2.0, 0.0, 0.15, color));
    verts.extend(line_v(-2.0, -1.5, 1.5, 0.15, color));

    if with_bubble {
        // Bubble for inverter
        verts.extend(circle_outline(2.4, 0.0, 0.4, 0.1, 12, color));
        verts.extend(line_h(2.8, 5.0, 0.0, 0.15, color));
    } else {
        verts.extend(line_h(2.0, 5.0, 0.0, 0.15, color));
    }

    verts
}

/// Generate AND gate symbol
pub fn and_gate_symbol_vertices() -> Vec<Vertex> {
    let color = colors::DIGITAL;
    let mut verts = Vec::new();

    // Input leads
    verts.extend(line_h(-5.0, -2.0, 1.0, 0.15, color));
    verts.extend(line_h(-5.0, -2.0, -1.0, 0.15, color));

    // Body - left side and top/bottom
    verts.extend(line_v(-2.0, -1.5, 1.5, 0.15, color));
    verts.extend(line_h(-2.0, 0.0, 1.5, 0.15, color));
    verts.extend(line_h(-2.0, 0.0, -1.5, 0.15, color));

    // Curved right side (semicircle)
    verts.extend(arc_outline(0.0, 0.0, 1.5, -std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2, 0.15, 12, color));

    // Output lead
    verts.extend(line_h(1.5, 5.0, 0.0, 0.15, color));

    verts
}

/// Generate OR gate symbol
pub fn or_gate_symbol_vertices() -> Vec<Vertex> {
    let color = colors::DIGITAL;
    let mut verts = Vec::new();

    // Input leads
    verts.extend(line_h(-5.0, -1.5, 1.0, 0.15, color));
    verts.extend(line_h(-5.0, -1.5, -1.0, 0.15, color));

    // Curved back
    verts.extend(arc_outline(-1.5, 0.0, 0.5, std::f32::consts::FRAC_PI_2, -std::f32::consts::FRAC_PI_2 + std::f32::consts::TAU, 0.15, 8, color));

    // Curved sides meeting at point
    verts.extend(arc_outline(-0.5, 2.5, 2.5, -std::f32::consts::FRAC_PI_2 - 0.5, -std::f32::consts::FRAC_PI_2 + 0.5, 0.15, 12, color));
    verts.extend(arc_outline(-0.5, -2.5, 2.5, std::f32::consts::FRAC_PI_2 - 0.5, std::f32::consts::FRAC_PI_2 + 0.5, 0.15, 12, color));

    // Output lead
    verts.extend(line_h(2.0, 5.0, 0.0, 0.15, color));

    verts
}

// =============================================================================
// Symbol Registry
// =============================================================================

/// Get vertices for a component type by GPU symbol ID
///
/// This maps the symbol IDs from GpuRenderCache to actual geometry.
pub fn get_symbol_vertices(symbol_id: u32) -> Vec<Vertex> {
    match symbol_id {
        // Passives
        0 => resistor_symbol_vertices(),
        1 => capacitor_symbol_vertices(),
        2 => inductor_symbol_vertices(),
        3 => inductor_symbol_vertices(), // CoupledInductor placeholder

        // Sources
        10 => voltage_source_symbol_vertices(),
        11 => current_source_symbol_vertices(),
        12 => ac_source_symbol_vertices(),
        13..=17 => voltage_source_symbol_vertices(), // Pulse, Sin, PWL, Exp, SFFM
        18..=23 => current_source_symbol_vertices(), // AC, Pulse, Sin variants

        // Controlled sources
        30..=33 => controlled_source_symbol_vertices(),

        // Semiconductors
        40 => diode_symbol_vertices(),
        41 => npn_bjt_symbol_vertices(),
        42 => pnp_bjt_symbol_vertices(),
        43 => nmos_symbol_vertices(),
        44 => pmos_symbol_vertices(),
        45 => npn_bjt_symbol_vertices(), // NJFET placeholder
        46 => pnp_bjt_symbol_vertices(), // PJFET placeholder
        47 => nmos_symbol_vertices(),    // NVdmos
        48 => pmos_symbol_vertices(),    // PVdmos
        49 => inductor_symbol_vertices(), // SaturableInductor

        // Special
        60 => ground_symbol_vertices(),

        // XSPICE Analog
        70 => inverter_symbol_vertices(false), // Gain as buffer
        71..=76 => controlled_source_symbol_vertices(), // Summer, Mult, etc.

        // XSPICE Digital
        80 => inverter_symbol_vertices(true),  // Inverter
        81 => inverter_symbol_vertices(false), // Buffer
        82 => and_gate_symbol_vertices(),      // AND
        83 => or_gate_symbol_vertices(),       // OR
        84 => and_gate_symbol_vertices(),      // NAND (TODO: add bubble)
        85 => or_gate_symbol_vertices(),       // NOR (TODO: add bubble)
        86 => or_gate_symbol_vertices(),       // XOR placeholder
        87..=92 => inverter_symbol_vertices(false), // Tri-state, flip-flops, etc.

        // Fallback - draw a simple box
        _ => {
            let color = colors::COMPONENT_BODY;
            let mut verts = Vec::new();
            verts.extend(rect(-2.0, -1.5, 2.0, 1.5, color));
            verts.extend(line_h(-5.0, -2.0, 0.0, 0.15, color));
            verts.extend(line_h(2.0, 5.0, 0.0, 0.15, color));
            verts
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Utility Tests
    // =========================================================================

    #[test]
    fn test_unit_quad() {
        let quad = unit_quad_vertices();
        assert_eq!(quad.len(), 6);
    }

    #[test]
    fn test_line_h_creates_6_vertices() {
        let line = line_h(0.0, 10.0, 0.0, 0.5, colors::WIRE_NORMAL);
        assert_eq!(line.len(), 6);
    }

    #[test]
    fn test_line_v_creates_6_vertices() {
        let line = line_v(0.0, 0.0, 10.0, 0.5, colors::WIRE_NORMAL);
        assert_eq!(line.len(), 6);
    }

    #[test]
    fn test_line_diag_creates_6_vertices() {
        let line = line_diag(0.0, 0.0, 10.0, 10.0, 0.5, colors::WIRE_NORMAL);
        assert_eq!(line.len(), 6);
    }

    #[test]
    fn test_triangle_creates_3_vertices() {
        let tri = triangle(0.0, 0.0, 1.0, 0.0, 0.5, 1.0, colors::WIRE_NORMAL);
        assert_eq!(tri.len(), 3);
    }

    #[test]
    fn test_rect_creates_6_vertices() {
        let r = rect(0.0, 0.0, 2.0, 2.0, colors::WIRE_NORMAL);
        assert_eq!(r.len(), 6);
    }

    #[test]
    fn test_circle_filled() {
        let circle = circle_filled(0.0, 0.0, 1.0, 12, colors::WIRE_NORMAL);
        assert_eq!(circle.len(), 12 * 3); // segments * 3 vertices per triangle
    }

    #[test]
    fn test_circle_outline() {
        let circle = circle_outline(0.0, 0.0, 1.0, 0.1, 12, colors::WIRE_NORMAL);
        assert_eq!(circle.len(), 12 * 6); // segments * 6 vertices per quad
    }

    #[test]
    fn test_arc_outline() {
        let arc = arc_outline(0.0, 0.0, 1.0, 0.0, std::f32::consts::PI, 0.1, 8, colors::WIRE_NORMAL);
        assert_eq!(arc.len(), 8 * 6);
    }

    // =========================================================================
    // Wire Tests
    // =========================================================================

    #[test]
    fn test_wire_vertices_two_points() {
        let points = [[0.0, 0.0], [10.0, 0.0]];
        let verts = generate_wire_vertices(&points, colors::WIRE_NORMAL, 1.0);
        assert_eq!(verts.len(), 6);
    }

    #[test]
    fn test_wire_vertices_three_points() {
        let points = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0]];
        let verts = generate_wire_vertices(&points, colors::WIRE_NORMAL, 1.0);
        assert_eq!(verts.len(), 12);
    }

    #[test]
    fn test_wire_vertices_single_point() {
        let points = [[0.0, 0.0]];
        let verts = generate_wire_vertices(&points, colors::WIRE_NORMAL, 1.0);
        assert_eq!(verts.len(), 0);
    }

    #[test]
    fn test_wire_vertices_empty() {
        let points: [[f32; 2]; 0] = [];
        let verts = generate_wire_vertices(&points, colors::WIRE_NORMAL, 1.0);
        assert_eq!(verts.len(), 0);
    }

    #[test]
    fn test_wire_skips_zero_length_segments() {
        let points = [[0.0, 0.0], [0.0, 0.0], [10.0, 0.0]];
        let verts = generate_wire_vertices(&points, colors::WIRE_NORMAL, 1.0);
        assert_eq!(verts.len(), 6); // Only one valid segment
    }

    // =========================================================================
    // Passive Component Tests
    // =========================================================================

    #[test]
    fn test_resistor_symbol() {
        let verts = resistor_symbol_vertices();
        assert!(!verts.is_empty());
        assert_eq!(verts.len() % 3, 0);
    }

    #[test]
    fn test_capacitor_symbol() {
        let verts = capacitor_symbol_vertices();
        assert!(!verts.is_empty());
        assert_eq!(verts.len() % 3, 0);
    }

    #[test]
    fn test_polarized_capacitor_symbol() {
        let verts = polarized_capacitor_symbol_vertices();
        assert!(!verts.is_empty());
        assert_eq!(verts.len() % 3, 0);
    }

    #[test]
    fn test_inductor_symbol() {
        let verts = inductor_symbol_vertices();
        assert!(!verts.is_empty());
        assert_eq!(verts.len() % 3, 0);
    }

    // =========================================================================
    // Semiconductor Tests
    // =========================================================================

    #[test]
    fn test_diode_symbol() {
        let verts = diode_symbol_vertices();
        assert!(!verts.is_empty());
        assert_eq!(verts.len() % 3, 0);
    }

    #[test]
    fn test_npn_bjt_symbol() {
        let verts = npn_bjt_symbol_vertices();
        assert!(!verts.is_empty());
        assert_eq!(verts.len() % 3, 0);
    }

    #[test]
    fn test_pnp_bjt_symbol() {
        let verts = pnp_bjt_symbol_vertices();
        assert!(!verts.is_empty());
        assert_eq!(verts.len() % 3, 0);
    }

    #[test]
    fn test_nmos_symbol() {
        let verts = nmos_symbol_vertices();
        assert!(!verts.is_empty());
        assert_eq!(verts.len() % 3, 0);
    }

    #[test]
    fn test_pmos_symbol() {
        let verts = pmos_symbol_vertices();
        assert!(!verts.is_empty());
        assert_eq!(verts.len() % 3, 0);
    }

    // =========================================================================
    // Source Tests
    // =========================================================================

    #[test]
    fn test_voltage_source_symbol() {
        let verts = voltage_source_symbol_vertices();
        assert!(!verts.is_empty());
        assert_eq!(verts.len() % 3, 0);
    }

    #[test]
    fn test_current_source_symbol() {
        let verts = current_source_symbol_vertices();
        assert!(!verts.is_empty());
        assert_eq!(verts.len() % 3, 0);
    }

    #[test]
    fn test_ac_source_symbol() {
        let verts = ac_source_symbol_vertices();
        assert!(!verts.is_empty());
        assert_eq!(verts.len() % 3, 0);
    }

    #[test]
    fn test_controlled_source_symbol() {
        let verts = controlled_source_symbol_vertices();
        assert!(!verts.is_empty());
        assert_eq!(verts.len() % 3, 0);
    }

    // =========================================================================
    // Special Symbol Tests
    // =========================================================================

    #[test]
    fn test_ground_symbol() {
        let verts = ground_symbol_vertices();
        assert!(!verts.is_empty());
        assert_eq!(verts.len() % 3, 0);
    }

    // =========================================================================
    // Digital Logic Tests
    // =========================================================================

    #[test]
    fn test_inverter_symbol() {
        let verts = inverter_symbol_vertices(true);
        assert!(!verts.is_empty());
        assert_eq!(verts.len() % 3, 0);
    }

    #[test]
    fn test_buffer_symbol() {
        let verts = inverter_symbol_vertices(false);
        assert!(!verts.is_empty());
        assert_eq!(verts.len() % 3, 0);
    }

    #[test]
    fn test_and_gate_symbol() {
        let verts = and_gate_symbol_vertices();
        assert!(!verts.is_empty());
        assert_eq!(verts.len() % 3, 0);
    }

    #[test]
    fn test_or_gate_symbol() {
        let verts = or_gate_symbol_vertices();
        assert!(!verts.is_empty());
        assert_eq!(verts.len() % 3, 0);
    }

    // =========================================================================
    // Symbol Registry Tests
    // =========================================================================

    #[test]
    fn test_get_symbol_vertices_resistor() {
        let verts = get_symbol_vertices(0);
        assert!(!verts.is_empty());
    }

    #[test]
    fn test_get_symbol_vertices_capacitor() {
        let verts = get_symbol_vertices(1);
        assert!(!verts.is_empty());
    }

    #[test]
    fn test_get_symbol_vertices_inductor() {
        let verts = get_symbol_vertices(2);
        assert!(!verts.is_empty());
    }

    #[test]
    fn test_get_symbol_vertices_voltage_source() {
        let verts = get_symbol_vertices(10);
        assert!(!verts.is_empty());
    }

    #[test]
    fn test_get_symbol_vertices_diode() {
        let verts = get_symbol_vertices(40);
        assert!(!verts.is_empty());
    }

    #[test]
    fn test_get_symbol_vertices_ground() {
        let verts = get_symbol_vertices(60);
        assert!(!verts.is_empty());
    }

    #[test]
    fn test_get_symbol_vertices_digital() {
        let verts = get_symbol_vertices(80);
        assert!(!verts.is_empty());
    }

    #[test]
    fn test_get_symbol_vertices_unknown_returns_fallback() {
        let verts = get_symbol_vertices(999);
        assert!(!verts.is_empty()); // Should return fallback box
    }

    #[test]
    fn test_all_registered_symbols() {
        // Test all documented symbol IDs produce valid geometry
        let ids = [
            0, 1, 2, 3,           // Passives
            10, 11, 12, 13, 14,   // Sources
            30, 31, 32, 33,       // Controlled
            40, 41, 42, 43, 44,   // Semiconductors
            60,                   // Ground
            70, 71,               // XSPICE analog
            80, 81, 82, 83,       // XSPICE digital
        ];

        for id in ids {
            let verts = get_symbol_vertices(id);
            assert!(!verts.is_empty(), "Symbol ID {} produced empty vertices", id);
            assert_eq!(verts.len() % 3, 0, "Symbol ID {} vertex count not multiple of 3", id);
        }
    }

    // =========================================================================
    // Color Constants Tests
    // =========================================================================

    #[test]
    fn test_color_constants() {
        assert_eq!(colors::WIRE_NORMAL[3], 1.0); // Alpha = 1
        assert_eq!(colors::COMPONENT_BODY[3], 1.0);
        assert_eq!(colors::BACKGROUND[3], 1.0);
    }

    #[test]
    fn test_color_values_normalized() {
        let all_colors = [
            colors::WIRE_NORMAL, colors::WIRE_SELECTED,
            colors::COMPONENT_BODY, colors::COMPONENT_FILL,
            colors::JUNCTION_NORMAL, colors::JUNCTION_SELECTED,
            colors::BACKGROUND, colors::SOURCE_POSITIVE,
            colors::SOURCE_NEGATIVE, colors::SEMICONDUCTOR,
            colors::DIGITAL,
        ];

        for color in all_colors {
            for component in color {
                assert!(component >= 0.0 && component <= 1.0,
                    "Color component {} out of range [0,1]", component);
            }
        }
    }

    // =========================================================================
    // Selection Box Geometry Tests
    // =========================================================================

    #[test]
    fn test_selection_box_border_vertices_count() {
        let verts = selection_box_border_vertices(0.0, 0.0, 100.0, 100.0, 2.0, [1.0, 1.0, 1.0, 1.0]);
        assert_eq!(verts.len(), 24); // 4 sides × 6 vertices
    }

    #[test]
    fn test_selection_box_border_vertices_inverted() {
        // Inverted coordinates (drag right-to-left)
        let verts = selection_box_border_vertices(100.0, 100.0, 0.0, 0.0, 2.0, [1.0, 1.0, 1.0, 1.0]);
        assert_eq!(verts.len(), 24);
    }

    #[test]
    fn test_selection_box_fill_vertices_count() {
        let verts = selection_box_fill_vertices(10.0, 20.0, 50.0, 80.0, [0.2, 0.4, 0.8, 0.3]);
        assert_eq!(verts.len(), 6); // 1 quad = 2 triangles = 6 vertices
    }

    #[test]
    fn test_selection_box_fill_vertices_inverted() {
        let verts = selection_box_fill_vertices(50.0, 80.0, 10.0, 20.0, [0.2, 0.4, 0.8, 0.3]);
        assert_eq!(verts.len(), 6);
    }

    #[test]
    fn test_selection_box_border_zero_width() {
        let verts = selection_box_border_vertices(50.0, 50.0, 50.0, 50.0, 2.0, [1.0, 1.0, 1.0, 1.0]);
        assert_eq!(verts.len(), 24); // Still generates geometry (degenerate case)
    }

    #[test]
    fn test_selection_box_color_preserved() {
        let color = [0.5, 0.6, 0.7, 0.8];
        let verts = selection_box_fill_vertices(0.0, 0.0, 10.0, 10.0, color);
        for v in verts.iter() {
            assert_eq!(v.color, color);
        }
    }

    #[test]
    fn test_selection_box_border_color_preserved() {
        let color = [0.1, 0.2, 0.3, 1.0];
        let verts = selection_box_border_vertices(0.0, 0.0, 100.0, 100.0, 2.0, color);
        for v in verts.iter() {
            assert_eq!(v.color, color);
        }
    }
}

