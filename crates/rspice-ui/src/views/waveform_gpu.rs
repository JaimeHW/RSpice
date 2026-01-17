//! GPU-Accelerated Waveform Rendering
//!
//! Uses wgpu for hardware-accelerated line rendering of waveform traces.
//! Targets 60fps pan/zoom on millions of data points.

use bytemuck::{Pod, Zeroable};
use std::sync::Arc;
use wgpu::util::DeviceExt;

/// Vertex data for waveform rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct WaveformVertex {
    /// Position (normalized device coordinates)
    position: [f32; 2],
    /// Color (RGBA)
    color: [f32; 4],
}

/// View uniforms passed to the shader
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ViewUniforms {
    /// Transform matrix (scale + translate)
    pub x_scale: f32,
    pub x_offset: f32,
    pub y_scale: f32,
    pub y_offset: f32,
}

/// GPU renderer for waveform traces
pub struct WaveformRenderer {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    vertex_buffer: Option<wgpu::Buffer>,
    vertex_count: u32,
}

impl WaveformRenderer {
    /// Create a new waveform renderer
    pub fn new(
        device: Arc<wgpu::Device>,
        queue: Arc<wgpu::Queue>,
        format: wgpu::TextureFormat,
    ) -> Self {
        // Create shader module
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Waveform Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("waveform.wgsl").into()),
        });

        // Create uniform buffer
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("View Uniforms"),
            contents: bytemuck::cast_slice(&[ViewUniforms {
                x_scale: 1.0,
                x_offset: 0.0,
                y_scale: 1.0,
                y_offset: 0.0,
            }]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("View Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // Create bind group
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("View Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Waveform Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create render pipeline
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

        Self {
            device,
            queue,
            pipeline,
            uniform_buffer,
            uniform_bind_group,
            vertex_buffer: None,
            vertex_count: 0,
        }
    }

    /// Update waveform data
    pub fn update_data(&mut self, x: &[f64], y: &[f64], color: [f32; 4]) {
        if x.is_empty() || y.is_empty() || x.len() != y.len() {
            self.vertex_buffer = None;
            self.vertex_count = 0;
            return;
        }

        // Convert to vertices
        let vertices: Vec<WaveformVertex> = x
            .iter()
            .zip(y.iter())
            .map(|(&xi, &yi)| WaveformVertex {
                position: [xi as f32, yi as f32],
                color,
            })
            .collect();

        self.vertex_count = vertices.len() as u32;

        // Create or update vertex buffer
        self.vertex_buffer = Some(self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Waveform Vertices"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            },
        ));
    }

    /// Update view transform (for pan/zoom)
    pub fn update_view(&self, x_min: f64, x_max: f64, y_min: f64, y_max: f64) {
        let x_range = (x_max - x_min) as f32;
        let y_range = (y_max - y_min) as f32;

        let uniforms = ViewUniforms {
            x_scale: 2.0 / x_range,
            x_offset: -1.0 - (x_min as f32) * (2.0 / x_range),
            y_scale: 2.0 / y_range,
            y_offset: -1.0 - (y_min as f32) * (2.0 / y_range),
        };

        self.queue
            .write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    }

    /// Render waveform to texture
    pub fn render(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let Some(vertex_buffer) = &self.vertex_buffer else {
            return;
        };

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Waveform Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load, // Don't clear, render on top
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..self.vertex_count, 0..1);
    }
}

/// Decimate waveform data for efficient rendering at different zoom levels
pub fn decimate(x: &[f64], y: &[f64], target_points: usize) -> (Vec<f64>, Vec<f64>) {
    if x.len() <= target_points {
        return (x.to_vec(), y.to_vec());
    }

    let step = x.len() as f64 / target_points as f64;
    let mut out_x = Vec::with_capacity(target_points);
    let mut out_y = Vec::with_capacity(target_points);

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

        // Add points in order (min before max or vice versa)
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
        assert!(dx.len() <= 40); // Each point might produce 2 (min/max)
        assert_eq!(dx.len(), dy.len());
    }
}
