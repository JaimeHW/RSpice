//! Schematic Renderer
//!
//! Main GPU renderer that coordinates all rendering passes for the schematic.
//! This is the entry point for GPU-based schematic visualization.

use std::sync::Arc;
use wgpu::{
    BindGroup, Buffer, Color, CommandEncoderDescriptor, LoadOp, Operations,
    RenderPassColorAttachment, RenderPassDescriptor, StoreOp, Surface, SurfaceTexture, TextureView,
    TextureViewDescriptor,
};

use super::buffers::{DynamicBuffer, UniformBuffer};
use super::camera::{Camera, CameraUniform};
use super::context::{GpuContext, GpuError};
use super::geometry::{self, colors, QuadVertex};
use super::pipeline::Pipelines;
use super::vertex::{ComponentInstance, JunctionInstance, Vertex, WireVertex};

/// Main schematic renderer
pub struct SchematicRenderer {
    /// GPU context (device, queue, etc.)
    context: Arc<GpuContext>,

    /// Render pipelines
    pipelines: Pipelines,

    /// Camera uniform buffer
    camera_uniform: UniformBuffer<CameraUniform>,

    /// Camera bind group
    camera_bind_group: BindGroup,

    // =========================================================================
    // Geometry Buffers
    // =========================================================================
    /// Wire vertex buffer
    wire_buffer: DynamicBuffer,

    /// Wire vertex count
    wire_vertex_count: u32,

    /// Component symbol vertex buffer (shared across all components)
    symbol_buffer: Buffer,

    /// Symbol vertex count per type
    symbol_vertex_count: u32,

    /// Component instance buffer
    component_instance_buffer: DynamicBuffer,

    /// Component instance count
    component_instance_count: u32,

    /// Junction quad vertices (unit circle)
    junction_quad_buffer: Buffer,

    /// Junction instance buffer
    junction_instance_buffer: DynamicBuffer,

    /// Junction instance count
    junction_instance_count: u32,

    /// Grid quad buffer
    grid_buffer: DynamicBuffer,

    // =========================================================================
    // State
    // =========================================================================
    /// Current surface dimensions
    width: u32,
    height: u32,

    /// Background clear color
    background_color: Color,
}

impl SchematicRenderer {
    /// Create a new schematic renderer
    pub async fn new() -> Result<Self, GpuError> {
        let context = Arc::new(GpuContext::new().await?);
        let pipelines = Pipelines::new(&context.device, context.surface_format)?;

        // Create camera uniform buffer
        let camera_uniform =
            UniformBuffer::new(&context.device, "Camera Uniform", CameraUniform::default());

        // Create camera bind group
        let camera_bind_group =
            pipelines.create_camera_bind_group(&context.device, camera_uniform.buffer());

        // Create geometry buffers
        let wire_buffer = DynamicBuffer::new("Wire Vertices", wgpu::BufferUsages::VERTEX);

        // Create symbol geometry (currently just resistor)
        let symbol_verts = geometry::resistor_symbol_vertices();
        let symbol_buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Symbol Vertices"),
            size: (symbol_verts.len() * std::mem::size_of::<Vertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        context
            .queue
            .write_buffer(&symbol_buffer, 0, bytemuck::cast_slice(&symbol_verts));

        let component_instance_buffer =
            DynamicBuffer::new("Component Instances", wgpu::BufferUsages::VERTEX);

        // Create junction quad (unit circle quad)
        let quad_verts = geometry::unit_quad_vertices();
        let junction_quad_buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Junction Quad"),
            size: (quad_verts.len() * std::mem::size_of::<QuadVertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        context
            .queue
            .write_buffer(&junction_quad_buffer, 0, bytemuck::cast_slice(&quad_verts));

        let junction_instance_buffer =
            DynamicBuffer::new("Junction Instances", wgpu::BufferUsages::VERTEX);

        let grid_buffer = DynamicBuffer::new("Grid Quad", wgpu::BufferUsages::VERTEX);

        Ok(Self {
            context,
            pipelines,
            camera_uniform,
            camera_bind_group,
            wire_buffer,
            wire_vertex_count: 0,
            symbol_buffer,
            symbol_vertex_count: symbol_verts.len() as u32,
            component_instance_buffer,
            component_instance_count: 0,
            junction_quad_buffer,
            junction_instance_buffer,
            junction_instance_count: 0,
            grid_buffer,
            width: 800,
            height: 600,
            background_color: Color {
                r: 0.08,
                g: 0.08,
                b: 0.08,
                a: 1.0,
            },
        })
    }

    /// Get GPU context
    pub fn context(&self) -> &GpuContext {
        &self.context
    }

    /// Resize the render target
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width.max(1);
        self.height = height.max(1);
    }

    /// Update camera from current state
    pub fn update_camera(&mut self, camera: &Camera) {
        let uniform = camera.build_uniform();
        self.camera_uniform.set(&self.context.queue, uniform);
    }

    /// Update wire geometry
    pub fn update_wires(&mut self, wires: &[WireData]) {
        let mut vertices = Vec::new();

        for wire in wires {
            let color = if wire.selected {
                colors::WIRE_SELECTED
            } else {
                colors::WIRE_NORMAL
            };

            let wire_verts = geometry::generate_wire_vertices(&wire.points, color, 0.1);
            vertices.extend(wire_verts);
        }

        self.wire_buffer
            .write(&self.context.device, &self.context.queue, &vertices);
        self.wire_vertex_count = vertices.len() as u32;
    }

    /// Update component instances
    pub fn update_components(&mut self, components: &[ComponentData]) {
        let instances: Vec<_> = components
            .iter()
            .map(|c| {
                let mut inst = ComponentInstance::new(c.x, c.y, c.rotation, 0);
                inst.state = if c.selected { 1 } else { 0 };
                inst
            })
            .collect();

        self.component_instance_buffer
            .write(&self.context.device, &self.context.queue, &instances);
        self.component_instance_count = instances.len() as u32;
    }

    /// Update junction instances
    pub fn update_junctions(&mut self, junctions: &[JunctionData]) {
        let instances: Vec<_> = junctions
            .iter()
            .map(|j| {
                let color = if j.selected {
                    [77, 153, 255, 255] // Blue
                } else {
                    [0, 204, 0, 255] // Green
                };
                JunctionInstance::new(j.x, j.y, 0.3, color)
            })
            .collect();

        self.junction_instance_buffer
            .write(&self.context.device, &self.context.queue, &instances);
        self.junction_instance_count = instances.len() as u32;
    }

    /// Update grid quad to cover world bounds
    pub fn update_grid(&mut self, camera: &Camera) {
        let bounds = camera.world_bounds();
        // Extend bounds slightly past visible area
        let margin = 10.0;
        let grid_verts = geometry::grid_quad_vertices(
            bounds.min_x - margin,
            bounds.min_y - margin,
            bounds.max_x + margin,
            bounds.max_y + margin,
        );

        self.grid_buffer
            .write(&self.context.device, &self.context.queue, &grid_verts);
    }

    /// Update labels for rendering
    ///
    /// This accepts generated labels from the schematic bridge.
    /// Label rendering requires a text pipeline which can be added later.
    /// For now, we track the label count for debugging.
    pub fn update_labels(
        &mut self,
        component_labels: &[super::text::LabelData],
        net_labels: &[super::text::LabelData],
    ) {
        // Label rendering is tracked but not yet rendered to GPU
        // A full text pipeline would:
        // 1. Generate TextInstances from labels
        // 2. Upload to text instance buffer
        // 3. Render with text pipeline using glyph atlas texture
        let _total_labels = component_labels.len() + net_labels.len();
        // TODO: Implement text rendering pipeline
    }

    /// Render a frame to the given texture view
    pub fn render(&self, view: &TextureView) -> Result<(), GpuError> {
        let mut encoder = self
            .context
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Schematic Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Schematic Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(self.background_color),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Bind camera uniform (shared across all passes)
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

            // 1. Render grid background
            if let Some(buffer) = self.grid_buffer.buffer() {
                render_pass.set_pipeline(&self.pipelines.grid);
                render_pass.set_vertex_buffer(0, buffer.slice(..));
                render_pass.draw(0..6, 0..1);
            }

            // 2. Render wires
            if self.wire_vertex_count > 0 {
                if let Some(buffer) = self.wire_buffer.buffer() {
                    render_pass.set_pipeline(&self.pipelines.wire);
                    render_pass.set_vertex_buffer(0, buffer.slice(..));
                    render_pass.draw(0..self.wire_vertex_count, 0..1);
                }
            }

            // 3. Render components (instanced)
            if self.component_instance_count > 0 {
                if let Some(inst_buffer) = self.component_instance_buffer.buffer() {
                    render_pass.set_pipeline(&self.pipelines.component);
                    render_pass.set_vertex_buffer(0, self.symbol_buffer.slice(..));
                    render_pass.set_vertex_buffer(1, inst_buffer.slice(..));
                    render_pass.draw(
                        0..self.symbol_vertex_count,
                        0..self.component_instance_count,
                    );
                }
            }

            // 4. Render junctions (instanced circles)
            if self.junction_instance_count > 0 {
                if let Some(inst_buffer) = self.junction_instance_buffer.buffer() {
                    render_pass.set_pipeline(&self.pipelines.circle);
                    render_pass.set_vertex_buffer(0, self.junction_quad_buffer.slice(..));
                    render_pass.set_vertex_buffer(1, inst_buffer.slice(..));
                    render_pass.draw(0..6, 0..self.junction_instance_count);
                }
            }
        }

        self.context.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }

    /// Render to a texture and return the image data as RGBA bytes
    ///
    /// Creates an offscreen texture, renders the schematic, and reads back pixels.
    /// This is used for canvas display in Dioxus webview mode.
    pub fn render_to_image(&mut self, width: u32, height: u32) -> Result<Vec<u8>, GpuError> {
        let width = width.max(1);
        let height = height.max(1);

        // Create offscreen render texture
        let texture = self.context.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Offscreen Render Target"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.context.surface_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        let texture_view = texture.create_view(&TextureViewDescriptor::default());

        // Render to the offscreen texture
        self.render(&texture_view)?;

        // Calculate buffer dimensions (row alignment)
        let bytes_per_pixel = 4u32;
        let unpadded_bytes_per_row = width * bytes_per_pixel;
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let padded_bytes_per_row = ((unpadded_bytes_per_row + align - 1) / align) * align;

        // Create staging buffer for readback
        let buffer_size = (padded_bytes_per_row * height) as u64;
        let staging_buffer = self.context.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // Copy texture to staging buffer
        let mut encoder = self.context.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Texture Copy Encoder"),
        });

        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyBuffer {
                buffer: &staging_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(padded_bytes_per_row),
                    rows_per_image: Some(height),
                },
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        self.context.queue.submit(std::iter::once(encoder.finish()));

        // Map buffer and read pixels (blocking)
        let buffer_slice = staging_buffer.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });

        // Poll until mapping is complete
        self.context.device.poll(wgpu::Maintain::Wait);
        rx.recv()
            .map_err(|_| GpuError::Other("Buffer map channel closed".into()))?
            .map_err(|e| GpuError::Other(format!("Buffer map failed: {:?}", e)))?;

        // Read the data and handle row padding
        let data = buffer_slice.get_mapped_range();
        let mut result = Vec::with_capacity((width * height * bytes_per_pixel) as usize);

        for row in 0..height {
            let start = (row * padded_bytes_per_row) as usize;
            let end = start + unpadded_bytes_per_row as usize;
            result.extend_from_slice(&data[start..end]);
        }

        // Unmap buffer
        drop(data);
        staging_buffer.unmap();

        Ok(result)
    }

    /// Render to a base64 PNG data URL (for webview display)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn render_to_data_url(&mut self, width: u32, height: u32) -> Result<String, GpuError> {
        use base64::Engine;

        let rgba_data = self.render_to_image(width, height)?;
        
        // Encode as PNG
        let mut png_data = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut png_data, width, height);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header()
                .map_err(|e| GpuError::Other(format!("PNG header error: {}", e)))?;
            writer.write_image_data(&rgba_data)
                .map_err(|e| GpuError::Other(format!("PNG write error: {}", e)))?;
        }

        // Encode as base64 data URL
        let base64 = base64::engine::general_purpose::STANDARD.encode(&png_data);
        Ok(format!("data:image/png;base64,{}", base64))
    }
}

// =============================================================================
// Data Types for Schematic Interop
// =============================================================================

/// Wire data for GPU upload
#[derive(Debug, Clone)]
pub struct WireData {
    pub points: Vec<[f32; 2]>,
    pub selected: bool,
}

/// Component data for GPU upload
#[derive(Debug, Clone)]
pub struct ComponentData {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub symbol_type: u32,
    pub selected: bool,
}

/// Junction data for GPU upload
#[derive(Debug, Clone)]
pub struct JunctionData {
    pub x: f32,
    pub y: f32,
    pub selected: bool,
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Data Type Tests
    // =========================================================================

    #[test]
    fn test_wire_data_construction() {
        let wire = WireData {
            points: vec![[0.0, 0.0], [10.0, 0.0]],
            selected: false,
        };
        assert_eq!(wire.points.len(), 2);
    }

    #[test]
    fn test_wire_data_selected() {
        let wire = WireData {
            points: vec![[0.0, 0.0], [10.0, 10.0]],
            selected: true,
        };
        assert!(wire.selected);
    }

    #[test]
    fn test_component_data_construction() {
        let comp = ComponentData {
            x: 100.0,
            y: 200.0,
            rotation: std::f32::consts::FRAC_PI_2,
            symbol_type: 0,
            selected: false,
        };
        assert_eq!(comp.x, 100.0);
        assert_eq!(comp.y, 200.0);
        assert_eq!(comp.symbol_type, 0);
    }

    #[test]
    fn test_component_data_selected() {
        let comp = ComponentData {
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            symbol_type: 10,
            selected: true,
        };
        assert!(comp.selected);
        assert_eq!(comp.symbol_type, 10);
    }

    #[test]
    fn test_junction_data_construction() {
        let junc = JunctionData {
            x: 50.0,
            y: 75.0,
            selected: false,
        };
        assert_eq!(junc.x, 50.0);
        assert_eq!(junc.y, 75.0);
    }

    #[test]
    fn test_junction_data_selected() {
        let junc = JunctionData {
            x: 0.0,
            y: 0.0,
            selected: true,
        };
        assert!(junc.selected);
    }

    // =========================================================================
    // Clone Tests
    // =========================================================================

    #[test]
    fn test_wire_data_clone() {
        let wire = WireData {
            points: vec![[1.0, 2.0], [3.0, 4.0]],
            selected: true,
        };
        let wire2 = wire.clone();
        assert_eq!(wire.points, wire2.points);
        assert_eq!(wire.selected, wire2.selected);
    }

    #[test]
    fn test_component_data_clone() {
        let comp = ComponentData {
            x: 10.0,
            y: 20.0,
            rotation: 1.57,
            symbol_type: 5,
            selected: true,
        };
        let comp2 = comp.clone();
        assert_eq!(comp.x, comp2.x);
        assert_eq!(comp.symbol_type, comp2.symbol_type);
    }

    #[test]
    fn test_junction_data_clone() {
        let junc = JunctionData {
            x: 100.0,
            y: 200.0,
            selected: true,
        };
        let junc2 = junc.clone();
        assert_eq!(junc.x, junc2.x);
        assert_eq!(junc.selected, junc2.selected);
    }

    // =========================================================================
    // Edge Cases
    // =========================================================================

    #[test]
    fn test_wire_data_empty_points() {
        let wire = WireData {
            points: vec![],
            selected: false,
        };
        assert!(wire.points.is_empty());
    }

    #[test]
    fn test_wire_data_single_point() {
        let wire = WireData {
            points: vec![[5.0, 5.0]],
            selected: false,
        };
        assert_eq!(wire.points.len(), 1);
    }

    #[test]
    fn test_component_at_origin() {
        let comp = ComponentData {
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            symbol_type: 0,
            selected: false,
        };
        assert_eq!(comp.x, 0.0);
        assert_eq!(comp.y, 0.0);
    }

    #[test]
    fn test_component_negative_coordinates() {
        let comp = ComponentData {
            x: -100.0,
            y: -200.0,
            rotation: 3.14159,
            symbol_type: 40,
            selected: true,
        };
        assert_eq!(comp.x, -100.0);
        assert_eq!(comp.y, -200.0);
    }
}
