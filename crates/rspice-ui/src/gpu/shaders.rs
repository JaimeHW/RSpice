//! GPU Shaders
//!
//! WGSL shaders for schematic rendering.
//! Shaders are embedded as strings for easy modification and debugging.

/// Basic vertex/fragment shader for solid colored primitives
pub const SOLID_SHADER: &str = r#"
// Camera uniform buffer
struct Camera {
    view_proj: mat4x4<f32>,
    viewport: vec4<f32>,  // width, height, 1/width, 1/height
    zoom: f32,
    grid_size: f32,
    _padding: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

// Vertex input
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
}

// Vertex output / Fragment input
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Transform world position to clip space
    out.clip_position = camera.view_proj * vec4<f32>(in.position, 0.0, 1.0);
    out.color = in.color;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
"#;

/// Wire shader with line thickness
pub const WIRE_SHADER: &str = r#"
struct Camera {
    view_proj: mat4x4<f32>,
    viewport: vec4<f32>,
    zoom: f32,
    grid_size: f32,
    _padding: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) normal: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) thickness: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Offset by normal * thickness in screen space
    let offset_world = in.normal * in.thickness / camera.zoom;
    let world_pos = in.position + offset_world;
    
    out.clip_position = camera.view_proj * vec4<f32>(world_pos, 0.0, 1.0);
    out.color = in.color;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
"#;

/// Instanced component shader
pub const COMPONENT_SHADER: &str = r#"
struct Camera {
    view_proj: mat4x4<f32>,
    viewport: vec4<f32>,
    zoom: f32,
    grid_size: f32,
    _padding: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

// Vertex (per-symbol)
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
}

// Instance (per-component)
struct InstanceInput {
    @location(4) inst_position: vec2<f32>,
    @location(5) inst_rotation: f32,
    @location(6) inst_scale: f32,
    @location(7) inst_color: vec4<f32>,
    @location(8) inst_state: u32,
    @location(9) inst_symbol_id: u32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) state: u32,
}

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Apply rotation
    let cos_r = cos(instance.inst_rotation);
    let sin_r = sin(instance.inst_rotation);
    let rotated = vec2<f32>(
        vertex.position.x * cos_r - vertex.position.y * sin_r,
        vertex.position.x * sin_r + vertex.position.y * cos_r
    );
    
    // Apply scale and translation
    let world_pos = rotated * instance.inst_scale + instance.inst_position;
    
    out.clip_position = camera.view_proj * vec4<f32>(world_pos, 0.0, 1.0);
    out.color = vertex.color * instance.inst_color;
    out.state = instance.inst_state;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Highlight selected components
    if (in.state == 1u) {
        // Selected - use accent color
        return vec4<f32>(0.3, 0.6, 1.0, 1.0);
    } else if (in.state == 2u) {
        // Hovered - brighten slightly
        return in.color * 1.2;
    }
    return in.color;
}
"#;

/// Circle/junction shader using instancing
pub const CIRCLE_SHADER: &str = r#"
struct Camera {
    view_proj: mat4x4<f32>,
    viewport: vec4<f32>,
    zoom: f32,
    grid_size: f32,
    _padding: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

// Unit quad vertex
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
}

// Circle instance
struct InstanceInput {
    @location(4) inst_position: vec2<f32>,
    @location(5) inst_radius: f32,
    @location(6) inst_color: u32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
}

fn unpack_color(packed: u32) -> vec4<f32> {
    let r = f32(packed & 0xFFu) / 255.0;
    let g = f32((packed >> 8u) & 0xFFu) / 255.0;
    let b = f32((packed >> 16u) & 0xFFu) / 255.0;
    let a = f32((packed >> 24u) & 0xFFu) / 255.0;
    return vec4<f32>(r, g, b, a);
}

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Scale quad by radius and translate
    let world_pos = vertex.position * instance.inst_radius + instance.inst_position;
    
    out.clip_position = camera.view_proj * vec4<f32>(world_pos, 0.0, 1.0);
    out.uv = vertex.uv;
    out.color = unpack_color(instance.inst_color);
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Discard pixels outside circle
    let dist = length(in.uv - vec2<f32>(0.5, 0.5)) * 2.0;
    if (dist > 1.0) {
        discard;
    }
    
    // Smooth edge for anti-aliasing
    let alpha = 1.0 - smoothstep(0.9, 1.0, dist);
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
"#;

/// Grid shader for background grid pattern
pub const GRID_SHADER: &str = r#"
struct Camera {
    view_proj: mat4x4<f32>,
    viewport: vec4<f32>,
    zoom: f32,
    grid_size: f32,
    _padding: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_pos: vec2<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(in.position, 0.0, 1.0);
    out.world_pos = in.position;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Grid line rendering using fwidth for proper anti-aliasing at all zoom levels
    // Visual grid is 2x snap grid (20px when grid_size=10) for cleaner appearance
    let minor_grid = camera.grid_size * 2.0;
    let major_grid = camera.grid_size * 10.0;
    
    // Use modulo to find distance to nearest grid line
    // abs(fract(x/grid + 0.5) - 0.5) * grid gives distance to nearest line
    let minor_dist_x = abs(fract(in.world_pos.x / minor_grid + 0.5) - 0.5) * minor_grid;
    let minor_dist_y = abs(fract(in.world_pos.y / minor_grid + 0.5) - 0.5) * minor_grid;
    let minor_dist = min(minor_dist_x, minor_dist_y);
    
    let major_dist_x = abs(fract(in.world_pos.x / major_grid + 0.5) - 0.5) * major_grid;
    let major_dist_y = abs(fract(in.world_pos.y / major_grid + 0.5) - 0.5) * major_grid;
    let major_dist = min(major_dist_x, major_dist_y);
    
    // Use fwidth for screen-space anti-aliasing - gets the pixel size in world coords
    // This ensures consistent 1-pixel lines at any zoom level
    let pixel_size = fwidth(in.world_pos.x);
    let line_width = max(pixel_size, 0.5); // At least 0.5 world units
    
    // Create anti-aliased lines using smoothstep with fwidth-based threshold
    let minor_line = 1.0 - smoothstep(0.0, line_width, minor_dist);
    let major_line = 1.0 - smoothstep(0.0, line_width, major_dist);
    
    // Colors matching SVG grid (subtle gray)
    let minor_color = vec4<f32>(0.5, 0.5, 0.5, minor_line * 0.08);
    let major_color = vec4<f32>(0.5, 0.5, 0.5, major_line * 0.2);
    
    // Blend major over minor
    let grid_alpha = max(minor_color.a, major_color.a);
    let grid_color = select(minor_color.rgb, major_color.rgb, major_line > minor_line);
    
    return vec4<f32>(grid_color, grid_alpha);
}
"#;

/// Text shader for glyph atlas-based text rendering
///
/// Uses instancing for efficient text rendering. Each instance represents
/// a single glyph with position, UV coordinates, color, and scale.
pub const TEXT_SHADER: &str = r#"
struct Camera {
    view_proj: mat4x4<f32>,
    viewport: vec4<f32>,
    zoom: f32,
    grid_size: f32,
    _padding: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

@group(1) @binding(0)
var glyph_texture: texture_2d<f32>;
@group(1) @binding(1)
var glyph_sampler: sampler;

// Vertex (unit quad)
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
}

// Instance (per-glyph)
struct InstanceInput {
    @location(4) inst_position: vec2<f32>,  // World position
    @location(5) inst_uv_offset: vec2<f32>, // UV top-left in atlas
    @location(6) inst_uv_size: vec2<f32>,   // UV size in atlas
    @location(7) inst_color: vec4<f32>,     // Glyph color
    @location(8) inst_scale: f32,           // Scale factor
    @location(9) inst_glyph_size: vec2<f32>, // Glyph size in world units
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
}

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Scale quad to glyph size
    let scaled_pos = vertex.position * instance.inst_glyph_size * instance.inst_scale;
    
    // Add world position
    let world_pos = scaled_pos + instance.inst_position;
    
    out.clip_position = camera.view_proj * vec4<f32>(world_pos, 0.0, 1.0);
    
    // Map vertex UV (0-1) to glyph UV in atlas
    out.uv = instance.inst_uv_offset + vertex.uv * instance.inst_uv_size;
    out.color = instance.inst_color;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Sample glyph from atlas
    let glyph_alpha = textureSample(glyph_texture, glyph_sampler, in.uv).r;
    
    // Apply color with glyph alpha
    if (glyph_alpha < 0.1) {
        discard;
    }
    
    return vec4<f32>(in.color.rgb, in.color.a * glyph_alpha);
}
"#;

/// Selection highlight shader for box selection overlay
pub const SELECTION_SHADER: &str = r#"
struct Camera {
    view_proj: mat4x4<f32>,
    viewport: vec4<f32>,
    zoom: f32,
    grid_size: f32,
    _padding: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_pos: vec2<f32>,
}

struct SelectionUniform {
    min_pos: vec2<f32>,
    max_pos: vec2<f32>,
    border_color: vec4<f32>,
    fill_color: vec4<f32>,
}

@group(1) @binding(0)
var<uniform> selection: SelectionUniform;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(in.position, 0.0, 1.0);
    out.world_pos = in.position;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Check if inside selection box
    let inside_x = in.world_pos.x >= selection.min_pos.x && in.world_pos.x <= selection.max_pos.x;
    let inside_y = in.world_pos.y >= selection.min_pos.y && in.world_pos.y <= selection.max_pos.y;
    
    if (!inside_x || !inside_y) {
        discard;
    }
    
    // Border detection (2 pixel width in screen space)
    let border_width = 2.0 / camera.zoom;
    let near_border_x = in.world_pos.x < selection.min_pos.x + border_width || 
                        in.world_pos.x > selection.max_pos.x - border_width;
    let near_border_y = in.world_pos.y < selection.min_pos.y + border_width || 
                        in.world_pos.y > selection.max_pos.y - border_width;
    
    if (near_border_x || near_border_y) {
        return selection.border_color;
    }
    
    return selection.fill_color;
}
"#;

/// Wire preview shader for drawing-in-progress wires
pub const WIRE_PREVIEW_SHADER: &str = r#"
struct Camera {
    view_proj: mat4x4<f32>,
    viewport: vec4<f32>,
    zoom: f32,
    grid_size: f32,
    _padding: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) normal: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) thickness: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) dash_coord: f32,
}

@vertex
fn vs_main(in: VertexInput, @builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    
    // Offset by normal * thickness
    let offset_world = in.normal * in.thickness / camera.zoom;
    let world_pos = in.position + offset_world;
    
    out.clip_position = camera.view_proj * vec4<f32>(world_pos, 0.0, 1.0);
    out.color = in.color;
    out.dash_coord = f32(vertex_index) * 0.5;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Dashed line pattern
    let dash = fract(in.dash_coord);
    if (dash > 0.5) {
        discard;
    }
    
    return in.color;
}
"#;

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solid_shader_not_empty() {
        assert!(!SOLID_SHADER.is_empty());
        assert!(SOLID_SHADER.contains("vs_main"));
        assert!(SOLID_SHADER.contains("fs_main"));
    }

    #[test]
    fn test_wire_shader_has_thickness() {
        assert!(WIRE_SHADER.contains("thickness"));
        assert!(WIRE_SHADER.contains("normal"));
    }

    #[test]
    fn test_component_shader_has_instancing() {
        assert!(COMPONENT_SHADER.contains("InstanceInput"));
        assert!(COMPONENT_SHADER.contains("inst_rotation"));
        assert!(COMPONENT_SHADER.contains("inst_scale"));
    }

    #[test]
    fn test_circle_shader_has_anti_aliasing() {
        assert!(CIRCLE_SHADER.contains("smoothstep"));
        assert!(CIRCLE_SHADER.contains("discard"));
    }

    #[test]
    fn test_grid_shader_has_major_minor() {
        assert!(GRID_SHADER.contains("major_grid"));
        assert!(GRID_SHADER.contains("minor"));
    }

    #[test]
    fn test_text_shader_has_glyph_texture() {
        assert!(TEXT_SHADER.contains("glyph_texture"));
        assert!(TEXT_SHADER.contains("glyph_sampler"));
        assert!(TEXT_SHADER.contains("inst_uv_offset"));
    }

    #[test]
    fn test_selection_shader_has_border() {
        assert!(SELECTION_SHADER.contains("border_color"));
        assert!(SELECTION_SHADER.contains("fill_color"));
        assert!(SELECTION_SHADER.contains("min_pos"));
    }

    #[test]
    fn test_wire_preview_shader_has_dashing() {
        assert!(WIRE_PREVIEW_SHADER.contains("dash"));
        assert!(WIRE_PREVIEW_SHADER.contains("discard"));
    }

    #[test]
    fn test_all_shaders_have_camera() {
        let shaders = [
            SOLID_SHADER,
            WIRE_SHADER,
            COMPONENT_SHADER,
            CIRCLE_SHADER,
            GRID_SHADER,
            TEXT_SHADER,
            SELECTION_SHADER,
            WIRE_PREVIEW_SHADER,
        ];

        for shader in shaders {
            assert!(
                shader.contains("struct Camera"),
                "Shader missing Camera struct"
            );
            assert!(
                shader.contains("camera: Camera"),
                "Shader missing camera uniform"
            );
        }
    }

    #[test]
    fn test_all_shaders_have_entry_points() {
        let shaders = [
            SOLID_SHADER,
            WIRE_SHADER,
            COMPONENT_SHADER,
            CIRCLE_SHADER,
            GRID_SHADER,
            TEXT_SHADER,
            SELECTION_SHADER,
            WIRE_PREVIEW_SHADER,
        ];

        for shader in shaders {
            assert!(shader.contains("@vertex"), "Shader missing vertex entry");
            assert!(
                shader.contains("@fragment"),
                "Shader missing fragment entry"
            );
            assert!(shader.contains("fn vs_main"), "Shader missing vs_main");
            assert!(shader.contains("fn fs_main"), "Shader missing fs_main");
        }
    }

    #[test]
    fn test_shader_syntax_basic_validation() {
        // Basic checks that shaders have balanced braces
        for shader in [
            SOLID_SHADER,
            WIRE_SHADER,
            COMPONENT_SHADER,
            CIRCLE_SHADER,
            GRID_SHADER,
            TEXT_SHADER,
        ] {
            let open = shader.chars().filter(|c| *c == '{').count();
            let close = shader.chars().filter(|c| *c == '}').count();
            assert_eq!(open, close, "Unbalanced braces in shader");
        }
    }
}
