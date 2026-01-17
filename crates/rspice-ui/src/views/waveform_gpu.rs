//! GPU-Accelerated Waveform Rendering
//!
//! Uses wgpu for hardware-accelerated line rendering of waveform traces.
//! Implements Dioxus native CustomPaintSource for direct GPU integration.

use bytemuck::{Pod, Zeroable};
use std::sync::{Arc, Mutex};
use wgpu::util::DeviceExt;

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
    /// Render texture
    texture: Option<wgpu::Texture>,
    texture_view: Option<wgpu::TextureView>,
    texture_size: (u32, u32),
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
            multisample: wgpu::MultisampleState::default(),
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
