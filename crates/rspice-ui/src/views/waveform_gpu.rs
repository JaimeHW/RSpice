//! GPU-Accelerated Waveform Rendering
//!
//! Uses wgpu for hardware-accelerated line rendering of waveform traces.
//! Device and queue are lazily initialized on first use.

use bytemuck::{Pod, Zeroable};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use wgpu::util::DeviceExt;

/// Global GPU device and queue (lazily initialized)
static GPU_CONTEXT: Lazy<Option<(Arc<wgpu::Device>, Arc<wgpu::Queue>)>> = Lazy::new(|| {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
    }))?;

    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: Some("Waveform GPU"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: Default::default(),
        },
        None,
    ))
    .ok()?;

    Some((Arc::new(device), Arc::new(queue)))
});

/// Check if GPU is available
pub fn is_gpu_available() -> bool {
    GPU_CONTEXT.is_some()
}

/// Get the GPU device and queue (if available)
pub fn get_gpu_context() -> Option<(Arc<wgpu::Device>, Arc<wgpu::Queue>)> {
    GPU_CONTEXT.clone()
}

/// Vertex data for waveform rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct WaveformVertex {
    /// Position in data coordinates
    pub position: [f32; 2],
    /// Color (RGBA)
    pub color: [f32; 4],
}

/// View uniforms passed to the shader
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ViewUniforms {
    /// Transform: x_scale, x_offset, y_scale, y_offset
    pub x_scale: f32,
    pub x_offset: f32,
    pub y_scale: f32,
    pub y_offset: f32,
    /// Background color
    pub bg_r: f32,
    pub bg_g: f32,
    pub bg_b: f32,
    pub bg_a: f32,
}

/// Waveform data to be rendered
#[derive(Clone, Debug)]
pub struct WaveformTrace {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub color: [f32; 4],
    pub name: String,
}

/// Shared state for waveform rendering
#[derive(Clone)]
pub struct WaveformGpuState {
    pub traces: Vec<WaveformTrace>,
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub dirty: bool,
}

impl Default for WaveformGpuState {
    fn default() -> Self {
        Self {
            traces: Vec::new(),
            x_min: 0.0,
            x_max: 5e-3,
            y_min: -1.5,
            y_max: 1.5,
            dirty: true,
        }
    }
}

/// GPU Waveform Painter implementing CustomPaintSource
pub struct WaveformPainter {
    /// Shared state with the Dioxus component
    pub state: Arc<Mutex<WaveformGpuState>>,
    /// wgpu device (set on resume)
    device: Option<Arc<wgpu::Device>>,
    queue: Option<Arc<wgpu::Queue>>,
    /// Render pipeline
    pipeline: Option<wgpu::RenderPipeline>,
    /// Uniform buffer
    uniform_buffer: Option<wgpu::Buffer>,
    uniform_bind_group: Option<wgpu::BindGroup>,
    /// Vertex buffers (one per trace)
    vertex_buffers: Vec<(wgpu::Buffer, u32)>,
    /// Render texture (resolve target for MSAA)
    texture: Option<wgpu::Texture>,
    texture_view: Option<wgpu::TextureView>,
    texture_size: (u32, u32),
    /// MSAA multisampled texture (render target)
    msaa_texture: Option<wgpu::Texture>,
    msaa_texture_view: Option<wgpu::TextureView>,
}

impl WaveformPainter {
    pub fn new(state: Arc<Mutex<WaveformGpuState>>) -> Self {
        Self {
            state,
            device: None,
            queue: None,
            pipeline: None,
            uniform_buffer: None,
            uniform_bind_group: None,
            vertex_buffers: Vec::new(),
            texture: None,
            texture_view: None,
            texture_size: (0, 0),
            msaa_texture: None,
            msaa_texture_view: None,
        }
    }

    fn create_pipeline(&mut self, device: &wgpu::Device, format: wgpu::TextureFormat) {
        // Create shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Waveform Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("waveform.wgsl").into()),
        });

        // Bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Waveform Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // Uniform buffer
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("View Uniforms"),
            contents: bytemuck::cast_slice(&[ViewUniforms {
                x_scale: 1.0,
                x_offset: 0.0,
                y_scale: 1.0,
                y_offset: 0.0,
                bg_r: 0.1,
                bg_g: 0.1,
                bg_b: 0.12,
                bg_a: 1.0,
            }]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Waveform Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // Pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Waveform Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Render pipeline
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Waveform Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<WaveformVertex>() as wgpu::BufferAddress,
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
                            format: wgpu::VertexFormat::Float32x4,
                        },
                    ],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 4, // 4x MSAA for smooth lines
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        self.pipeline = Some(pipeline);
        self.uniform_buffer = Some(uniform_buffer);
        self.uniform_bind_group = Some(bind_group);
    }

    fn update_vertex_buffers(&mut self, device: &wgpu::Device) {
        let state = self.state.lock().unwrap();

        self.vertex_buffers.clear();

        // Create gridlines first (so they render behind traces)
        let grid_color = [0.25f32, 0.25, 0.30, 1.0]; // Subtle grey
        let x_range = state.x_max - state.x_min;
        let y_range = state.y_max - state.y_min;

        // Use same linear spacing as axis labels (6 x-divisions, 5 y-divisions)
        let x_count = 6;
        let y_count = 5;
        let x_step = x_range / (x_count - 1) as f64;
        let y_step = y_range / (y_count - 1) as f64;

        // Vertical gridlines (X axis divisions)
        for i in 0..x_count {
            let x = state.x_min + i as f64 * x_step;
            let vertices = vec![
                WaveformVertex {
                    position: [x as f32, state.y_min as f32],
                    color: grid_color,
                },
                WaveformVertex {
                    position: [x as f32, state.y_max as f32],
                    color: grid_color,
                },
            ];
            let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Grid Vertical"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            self.vertex_buffers.push((buffer, vertices.len() as u32));
        }

        // Horizontal gridlines (Y axis divisions)
        for i in 0..y_count {
            let y = state.y_min + i as f64 * y_step;
            let vertices = vec![
                WaveformVertex {
                    position: [state.x_min as f32, y as f32],
                    color: grid_color,
                },
                WaveformVertex {
                    position: [state.x_max as f32, y as f32],
                    color: grid_color,
                },
            ];
            let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Grid Horizontal"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            self.vertex_buffers.push((buffer, vertices.len() as u32));
        }

        // Then add waveform traces
        for trace in &state.traces {
            if trace.x.is_empty() {
                continue;
            }

            let vertices: Vec<WaveformVertex> = trace
                .x
                .iter()
                .zip(trace.y.iter())
                .map(|(&x, &y)| WaveformVertex {
                    position: [x as f32, y as f32],
                    color: trace.color,
                })
                .collect();

            let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Trace Vertices"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

            self.vertex_buffers.push((buffer, vertices.len() as u32));
        }
    }

    fn update_uniforms(&self, queue: &wgpu::Queue) {
        let state = self.state.lock().unwrap();

        let x_range = (state.x_max - state.x_min) as f32;
        let y_range = (state.y_max - state.y_min) as f32;

        let uniforms = ViewUniforms {
            x_scale: 2.0 / x_range,
            x_offset: -1.0 - (state.x_min as f32) * (2.0 / x_range),
            y_scale: 2.0 / y_range,
            y_offset: -1.0 - (state.y_min as f32) * (2.0 / y_range),
            bg_r: 0.08,
            bg_g: 0.08,
            bg_b: 0.10,
            bg_a: 1.0,
        };

        if let Some(buffer) = &self.uniform_buffer {
            queue.write_buffer(buffer, 0, bytemuck::cast_slice(&[uniforms]));
        }
    }

    /// Ensure GPU resources are initialized
    fn ensure_resources(&mut self, width: u32, height: u32) {
        // Use global GPU context (lazily initialized)
        if self.device.is_none() {
            if let Some((device, queue)) = get_gpu_context() {
                self.device = Some(device);
                self.queue = Some(queue);
            } else {
                return; // GPU not available
            }
        }

        let device = self.device.clone().unwrap();
        let format = wgpu::TextureFormat::Rgba8Unorm;

        // Create pipeline if needed
        if self.pipeline.is_none() {
            self.create_pipeline(&device, format);
        }

        // Create/resize texture if needed
        if self.texture.is_none() || self.texture_size != (width, height) {
            let texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Waveform Render Texture"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
                view_formats: &[],
            });

            self.texture_view = Some(texture.create_view(&wgpu::TextureViewDescriptor::default()));
            self.texture = Some(texture);
            self.texture_size = (width, height);

            // Create MSAA multisampled texture (4x samples for smooth lines)
            let msaa_texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Waveform MSAA Texture"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 4, // 4x MSAA
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });

            self.msaa_texture_view =
                Some(msaa_texture.create_view(&wgpu::TextureViewDescriptor::default()));
            self.msaa_texture = Some(msaa_texture);
        }
    }

    /// Render waveforms to base64 PNG data URL
    pub fn render_to_base64(&mut self, width: u32, height: u32) -> Option<String> {
        if width == 0 || height == 0 {
            return None;
        }

        self.ensure_resources(width, height);

        // Clone Arc references to avoid borrow issues
        let device = self.device.clone()?;
        let queue = self.queue.clone()?;

        // Update data if dirty
        {
            let state = self.state.lock().unwrap();
            if state.dirty {
                drop(state);
                self.update_vertex_buffers(&device);
                self.state.lock().unwrap().dirty = false;
            }
        }

        self.update_uniforms(&queue);

        let texture_view = self.texture_view.as_ref()?;
        let msaa_view = self.msaa_texture_view.as_ref()?;
        let pipeline = self.pipeline.as_ref()?;
        let bind_group = self.uniform_bind_group.as_ref()?;

        // Create command encoder
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Waveform Render Encoder"),
        });

        // Render pass with MSAA - render to multisampled texture, resolve to regular texture
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Waveform Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: msaa_view,                    // Render to MSAA texture
                    resolve_target: Some(texture_view), // Resolve to regular texture for readback
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.08,
                            g: 0.08,
                            b: 0.10,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, bind_group, &[]);

            for (buffer, count) in &self.vertex_buffers {
                pass.set_vertex_buffer(0, buffer.slice(..));
                pass.draw(0..*count, 0..1);
            }
        }

        // Create buffer to read pixels
        let bytes_per_row = (width * 4 + 255) & !255; // Align to 256
        let buffer_size = (bytes_per_row * height) as u64;

        let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Pixel Readback Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                texture: self.texture.as_ref()?,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyBuffer {
                buffer: &output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(bytes_per_row),
                    rows_per_image: Some(height),
                },
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        queue.submit(std::iter::once(encoder.finish()));

        // Map buffer and read pixels
        let buffer_slice = output_buffer.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);
        rx.recv().ok()?.ok()?;

        let data = buffer_slice.get_mapped_range();

        // Extract actual pixel data (removing padding)
        let mut pixels = Vec::with_capacity((width * height * 4) as usize);
        for y in 0..height {
            let start = (y * bytes_per_row) as usize;
            let end = start + (width * 4) as usize;
            pixels.extend_from_slice(&data[start..end]);
        }

        drop(data);
        output_buffer.unmap();

        // Encode as PNG
        let mut png_data = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut png_data, width, height);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().ok()?;
            writer.write_image_data(&pixels).ok()?;
        }

        // Convert to base64 data URL
        use base64::Engine;
        let b64 = base64::engine::general_purpose::STANDARD.encode(&png_data);
        Some(format!("data:image/png;base64,{}", b64))
    }
}

/// Calculate a nice grid step size for the given range (targets 5-10 divisions)
fn calculate_grid_step(range: f64) -> f64 {
    let target_divisions = 6.0;
    let raw_step = range / target_divisions;

    // Find the order of magnitude
    let magnitude = 10f64.powf(raw_step.log10().floor());
    let normalized = raw_step / magnitude;

    // Round to nearest nice value (1, 2, 5)
    let nice = if normalized < 1.5 {
        1.0
    } else if normalized < 3.5 {
        2.0
    } else if normalized < 7.5 {
        5.0
    } else {
        10.0
    };

    nice * magnitude
}

/// Decimate waveform data for efficient rendering at different zoom levels
pub fn decimate(x: &[f64], y: &[f64], target_points: usize) -> (Vec<f64>, Vec<f64>) {
    if x.len() <= target_points {
        return (x.to_vec(), y.to_vec());
    }

    let step = x.len() as f64 / target_points as f64;
    let mut out_x = Vec::with_capacity(target_points * 2);
    let mut out_y = Vec::with_capacity(target_points * 2);

    // Min-max decimation to preserve peaks
    let mut i = 0.0;
    while (i as usize) < x.len() {
        let start = i as usize;
        let end = ((i + step) as usize).min(x.len());

        if end <= start {
            break;
        }

        // Find min and max in this window
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        let mut min_idx = start;
        let mut max_idx = start;

        for j in start..end {
            if y[j] < min_y {
                min_y = y[j];
                min_idx = j;
            }
            if y[j] > max_y {
                max_y = y[j];
                max_idx = j;
            }
        }

        // Add points in order
        if min_idx <= max_idx {
            out_x.push(x[min_idx]);
            out_y.push(min_y);
            if min_idx != max_idx {
                out_x.push(x[max_idx]);
                out_y.push(max_y);
            }
        } else {
            out_x.push(x[max_idx]);
            out_y.push(max_y);
            out_x.push(x[min_idx]);
            out_y.push(min_y);
        }

        i += step;
    }

    (out_x, out_y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimate() {
        let x: Vec<f64> = (0..100).map(|i| i as f64).collect();
        let y: Vec<f64> = x.iter().map(|x| x.sin()).collect();

        let (dx, dy) = decimate(&x, &y, 20);
        assert!(dx.len() <= 40);
        assert_eq!(dx.len(), dy.len());
    }

    #[test]
    fn test_waveform_state() {
        let state = WaveformGpuState::default();
        assert!(state.traces.is_empty());
        assert!(state.dirty);
    }
}
