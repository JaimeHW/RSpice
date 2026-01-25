//! Vertex Types
//!
//! Vertex and instance data structures for GPU rendering.
//! All types use #[repr(C)] and derive Pod/Zeroable for direct GPU upload.

use bytemuck::{Pod, Zeroable};

// =============================================================================
// Basic Vertex Types
// =============================================================================

/// Simple vertex with position and color
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    /// Position in world space (x, y)
    pub position: [f32; 2],
    /// Color (r, g, b, a)
    pub color: [f32; 4],
}

impl Vertex {
    pub const fn new(x: f32, y: f32, color: [f32; 4]) -> Self {
        Self {
            position: [x, y],
            color,
        }
    }

    /// Vertex buffer layout for wgpu
    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // Color
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

// =============================================================================
// Wire Vertex
// =============================================================================

/// Vertex for wire rendering with thickness support
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct WireVertex {
    /// Position in world space (x, y)
    pub position: [f32; 2],
    /// Normal direction for line thickness (nx, ny)
    pub normal: [f32; 2],
    /// Color (r, g, b, a)
    pub color: [f32; 4],
    /// Line thickness in pixels
    pub thickness: f32,
    /// Padding for alignment
    pub _padding: f32,
}

impl WireVertex {
    pub const fn new(x: f32, y: f32, nx: f32, ny: f32, color: [f32; 4], thickness: f32) -> Self {
        Self {
            position: [x, y],
            normal: [nx, ny],
            color,
            thickness,
            _padding: 0.0,
        }
    }

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<WireVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 8,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 16,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 32,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}

// =============================================================================
// Component Instance
// =============================================================================

/// Instance data for component rendering (one per component)
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct ComponentInstance {
    /// World position (x, y)
    pub position: [f32; 2],
    /// Rotation in radians
    pub rotation: f32,
    /// Scale factor (usually 1.0)
    pub scale: f32,
    /// Color tint (r, g, b, a)
    pub color: [f32; 4],
    /// Selection state (0 = normal, 1 = selected, 2 = hovered)
    pub state: u32,
    /// Symbol type index
    pub symbol_id: u32,
    /// Padding for alignment
    pub _padding: [u32; 2],
}

impl ComponentInstance {
    pub fn new(x: f32, y: f32, rotation: f32, symbol_id: u32) -> Self {
        Self {
            position: [x, y],
            rotation,
            scale: 1.0,
            color: [1.0, 1.0, 1.0, 1.0],
            state: 0,
            symbol_id,
            _padding: [0; 2],
        }
    }

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ComponentInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // Position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // Rotation
                wgpu::VertexAttribute {
                    offset: 8,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32,
                },
                // Scale
                wgpu::VertexAttribute {
                    offset: 12,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32,
                },
                // Color
                wgpu::VertexAttribute {
                    offset: 16,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // State
                wgpu::VertexAttribute {
                    offset: 32,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Uint32,
                },
                // Symbol ID
                wgpu::VertexAttribute {
                    offset: 36,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
        }
    }
}

// =============================================================================
// Junction Instance
// =============================================================================

/// Instance data for junction dot rendering
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct JunctionInstance {
    /// World position (x, y)
    pub position: [f32; 2],
    /// Radius in world units
    pub radius: f32,
    /// Color (r, g, b, a) - packed as u32
    pub color: u32,
}

impl JunctionInstance {
    pub fn new(x: f32, y: f32, radius: f32, color: [u8; 4]) -> Self {
        let color_packed = (color[0] as u32)
            | ((color[1] as u32) << 8)
            | ((color[2] as u32) << 16)
            | ((color[3] as u32) << 24);
        Self {
            position: [x, y],
            radius,
            color: color_packed,
        }
    }

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<JunctionInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 8,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: 12,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
        }
    }
}

// =============================================================================
// Quad Vertex
// =============================================================================

/// Simple quad vertex for text and overlay rendering
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct QuadVertex {
    /// Position (x, y)
    pub position: [f32; 2],
    /// UV coordinates (u, v)
    pub uv: [f32; 2],
}

impl QuadVertex {
    pub const fn new(x: f32, y: f32, u: f32, v: f32) -> Self {
        Self {
            position: [x, y],
            uv: [u, v],
        }
    }

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<QuadVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 8,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }

    /// Generate unit quad vertices (0-1 range)
    pub fn unit_quad() -> [QuadVertex; 6] {
        [
            QuadVertex::new(0.0, 0.0, 0.0, 0.0), // bottom-left
            QuadVertex::new(1.0, 0.0, 1.0, 0.0), // bottom-right
            QuadVertex::new(0.0, 1.0, 0.0, 1.0), // top-left
            QuadVertex::new(1.0, 0.0, 1.0, 0.0), // bottom-right
            QuadVertex::new(1.0, 1.0, 1.0, 1.0), // top-right
            QuadVertex::new(0.0, 1.0, 0.0, 1.0), // top-left
        ]
    }
}

// =============================================================================
// Text Instance
// =============================================================================

/// Instance data for text glyph rendering (one per glyph)
///
/// Total size: 48 bytes (16-byte aligned for GPU)
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct TextInstance {
    /// World position (x, y)
    pub position: [f32; 2],
    /// UV offset in glyph atlas (u, v)
    pub uv_offset: [f32; 2],
    /// UV size in glyph atlas (u_size, v_size)
    pub uv_size: [f32; 2],
    /// Glyph size in world units (width, height)
    pub glyph_size: [f32; 2],
    /// Color (r, g, b, a)
    pub color: [f32; 4],
    /// Scale factor
    pub scale: f32,
    /// Padding for alignment
    pub _padding: [f32; 3],
}

impl TextInstance {
    pub fn new(
        x: f32,
        y: f32,
        uv_offset: [f32; 2],
        uv_size: [f32; 2],
        glyph_size: [f32; 2],
        color: [f32; 4],
        scale: f32,
    ) -> Self {
        Self {
            position: [x, y],
            uv_offset,
            uv_size,
            glyph_size,
            color,
            scale,
            _padding: [0.0; 3],
        }
    }

    /// Create from a GlyphInfo (convenience method)
    pub fn from_glyph(
        x: f32,
        y: f32,
        u: f32,
        v: f32,
        u_size: f32,
        v_size: f32,
        color: [f32; 4],
        scale: f32,
    ) -> Self {
        Self::new(
            x, y,
            [u, v],
            [u_size, v_size],
            [u_size * 8.0, v_size * 8.0], // Approximate glyph size
            color,
            scale,
        )
    }

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TextInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // Position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // UV offset
                wgpu::VertexAttribute {
                    offset: 8,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // UV size
                wgpu::VertexAttribute {
                    offset: 16,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // Glyph size
                wgpu::VertexAttribute {
                    offset: 24,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // Color
                wgpu::VertexAttribute {
                    offset: 32,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // Scale
                wgpu::VertexAttribute {
                    offset: 48,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}

// =============================================================================
// Selection Box Vertex
// =============================================================================

/// Vertex for selection box rendering
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct SelectionVertex {
    /// Position in world space
    pub position: [f32; 2],
}

impl SelectionVertex {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { position: [x, y] }
    }

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<SelectionVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }

    /// Generate selection box vertices
    pub fn selection_quad(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> [SelectionVertex; 6] {
        [
            SelectionVertex::new(min_x, min_y),
            SelectionVertex::new(max_x, min_y),
            SelectionVertex::new(min_x, max_y),
            SelectionVertex::new(max_x, min_y),
            SelectionVertex::new(max_x, max_y),
            SelectionVertex::new(min_x, max_y),
        ]
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Vertex Size Tests (GPU alignment)
    // =========================================================================

    #[test]
    fn test_vertex_size() {
        // Ensure vertex is properly aligned for GPU
        assert_eq!(std::mem::size_of::<Vertex>(), 24);
    }

    #[test]
    fn test_wire_vertex_size() {
        // 2 position + 2 normal + 4 color + 1 thickness + 1 padding = 10 floats = 40 bytes
        assert_eq!(std::mem::size_of::<WireVertex>(), 40);
    }

    #[test]
    fn test_component_instance_size() {
        // Instance should be 48 bytes (multiple of 16 for alignment)
        assert_eq!(std::mem::size_of::<ComponentInstance>(), 48);
    }

    #[test]
    fn test_junction_instance_size() {
        assert_eq!(std::mem::size_of::<JunctionInstance>(), 16);
    }

    #[test]
    fn test_quad_vertex_size() {
        // 2 position + 2 uv = 16 bytes
        assert_eq!(std::mem::size_of::<QuadVertex>(), 16);
    }

    #[test]
    fn test_text_instance_size() {
        // 2 pos + 2 uv_off + 2 uv_size + 2 glyph_size + 4 color + 1 scale + 3 pad = 16 floats = 64 bytes
        assert_eq!(std::mem::size_of::<TextInstance>(), 64);
    }

    #[test]
    fn test_selection_vertex_size() {
        assert_eq!(std::mem::size_of::<SelectionVertex>(), 8);
    }

    // =========================================================================
    // Vertex Construction Tests
    // =========================================================================

    #[test]
    fn test_vertex_construction() {
        let v = Vertex::new(10.0, 20.0, [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(v.position, [10.0, 20.0]);
        assert_eq!(v.color, [1.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_wire_vertex_construction() {
        let v = WireVertex::new(5.0, 10.0, 0.0, 1.0, [0.0, 1.0, 0.0, 1.0], 2.0);
        assert_eq!(v.position, [5.0, 10.0]);
        assert_eq!(v.normal, [0.0, 1.0]);
        assert_eq!(v.thickness, 2.0);
    }

    #[test]
    fn test_component_instance_construction() {
        let inst = ComponentInstance::new(100.0, 200.0, std::f32::consts::FRAC_PI_2, 5);
        assert_eq!(inst.position, [100.0, 200.0]);
        assert_eq!(inst.rotation, std::f32::consts::FRAC_PI_2);
        assert_eq!(inst.symbol_id, 5);
        assert_eq!(inst.state, 0);
        assert_eq!(inst.scale, 1.0);
    }

    #[test]
    fn test_junction_instance_construction() {
        let inst = JunctionInstance::new(50.0, 75.0, 3.0, [255, 128, 0, 255]);
        assert_eq!(inst.position, [50.0, 75.0]);
        assert_eq!(inst.radius, 3.0);
        // Check color packing
        assert_eq!(inst.color & 0xFF, 255);          // R
        assert_eq!((inst.color >> 8) & 0xFF, 128);   // G
        assert_eq!((inst.color >> 16) & 0xFF, 0);    // B
        assert_eq!((inst.color >> 24) & 0xFF, 255);  // A
    }

    #[test]
    fn test_quad_vertex_construction() {
        let v = QuadVertex::new(0.5, 0.5, 0.25, 0.75);
        assert_eq!(v.position, [0.5, 0.5]);
        assert_eq!(v.uv, [0.25, 0.75]);
    }

    #[test]
    fn test_text_instance_construction() {
        let inst = TextInstance::new(
            10.0, 20.0,
            [0.1, 0.2],
            [0.05, 0.1],
            [4.0, 8.0],
            [1.0, 1.0, 1.0, 1.0],
            1.5,
        );
        assert_eq!(inst.position, [10.0, 20.0]);
        assert_eq!(inst.uv_offset, [0.1, 0.2]);
        assert_eq!(inst.uv_size, [0.05, 0.1]);
        assert_eq!(inst.glyph_size, [4.0, 8.0]);
        assert_eq!(inst.scale, 1.5);
    }

    #[test]
    fn test_text_instance_from_glyph() {
        let inst = TextInstance::from_glyph(
            5.0, 10.0,
            0.0, 0.0,
            0.0625, 0.125,
            [0.0, 1.0, 0.0, 1.0],
            2.0,
        );
        assert_eq!(inst.position, [5.0, 10.0]);
        assert_eq!(inst.scale, 2.0);
    }

    #[test]
    fn test_selection_vertex_construction() {
        let v = SelectionVertex::new(100.0, 200.0);
        assert_eq!(v.position, [100.0, 200.0]);
    }

    // =========================================================================
    // Quad Generation Tests
    // =========================================================================

    #[test]
    fn test_unit_quad_has_6_vertices() {
        let quad = QuadVertex::unit_quad();
        assert_eq!(quad.len(), 6);
    }

    #[test]
    fn test_unit_quad_uv_range() {
        let quad = QuadVertex::unit_quad();
        for v in quad {
            assert!(v.uv[0] >= 0.0 && v.uv[0] <= 1.0);
            assert!(v.uv[1] >= 0.0 && v.uv[1] <= 1.0);
        }
    }

    #[test]
    fn test_selection_quad_has_6_vertices() {
        let quad = SelectionVertex::selection_quad(0.0, 0.0, 100.0, 50.0);
        assert_eq!(quad.len(), 6);
    }

    #[test]
    fn test_selection_quad_bounds() {
        let quad = SelectionVertex::selection_quad(10.0, 20.0, 30.0, 40.0);
        // Check all vertices are within bounds
        for v in quad {
            assert!(v.position[0] >= 10.0 && v.position[0] <= 30.0);
            assert!(v.position[1] >= 20.0 && v.position[1] <= 40.0);
        }
    }

    // =========================================================================
    // Layout Tests
    // =========================================================================

    #[test]
    fn test_vertex_layout() {
        let layout = Vertex::layout();
        assert_eq!(layout.array_stride, 24);
        assert_eq!(layout.attributes.len(), 2);
    }

    #[test]
    fn test_wire_vertex_layout() {
        let layout = WireVertex::layout();
        assert_eq!(layout.array_stride, 40);
        assert_eq!(layout.attributes.len(), 4);
    }

    #[test]
    fn test_component_instance_layout() {
        let layout = ComponentInstance::layout();
        assert_eq!(layout.array_stride, 48);
        assert_eq!(layout.attributes.len(), 6);
    }

    #[test]
    fn test_junction_instance_layout() {
        let layout = JunctionInstance::layout();
        assert_eq!(layout.array_stride, 16);
        assert_eq!(layout.attributes.len(), 3);
    }

    #[test]
    fn test_quad_vertex_layout() {
        let layout = QuadVertex::layout();
        assert_eq!(layout.array_stride, 16);
        assert_eq!(layout.attributes.len(), 2);
    }

    #[test]
    fn test_text_instance_layout() {
        let layout = TextInstance::layout();
        assert_eq!(layout.array_stride, 64);
        assert_eq!(layout.attributes.len(), 6);
    }

    #[test]
    fn test_selection_vertex_layout() {
        let layout = SelectionVertex::layout();
        assert_eq!(layout.array_stride, 8);
        assert_eq!(layout.attributes.len(), 1);
    }

    // =========================================================================
    // Pod/Zeroable Tests
    // =========================================================================

    #[test]
    fn test_vertex_pod() {
        let v = Vertex::new(1.0, 2.0, [0.0; 4]);
        let bytes: &[u8] = bytemuck::bytes_of(&v);
        assert_eq!(bytes.len(), 24);
    }

    #[test]
    fn test_text_instance_pod() {
        let inst = TextInstance::new(
            0.0, 0.0, [0.0; 2], [0.0; 2], [0.0; 2], [0.0; 4], 1.0
        );
        let bytes: &[u8] = bytemuck::bytes_of(&inst);
        assert_eq!(bytes.len(), 64);
    }

    #[test]
    fn test_zeroed_vertex() {
        let v: Vertex = bytemuck::Zeroable::zeroed();
        assert_eq!(v.position, [0.0, 0.0]);
        assert_eq!(v.color, [0.0, 0.0, 0.0, 0.0]);
    }
}
