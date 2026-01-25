//! Render Pipeline Management
//!
//! Creates and manages wgpu render pipelines for different rendering passes.

use std::sync::Arc;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, BlendState, Buffer, BufferBindingType, ColorTargetState,
    ColorWrites, Device, FragmentState, MultisampleState, PipelineLayoutDescriptor, PolygonMode,
    PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor,
    ShaderModuleDescriptor, ShaderSource, ShaderStages, TextureFormat, VertexState,
    SamplerBindingType, TextureSampleType, TextureViewDimension,
};

use super::context::GpuError;
use super::shaders;
use super::vertex::{ComponentInstance, JunctionInstance, Vertex, WireVertex, QuadVertex, TextInstance, SelectionVertex};

/// Collection of all render pipelines for schematic rendering
pub struct Pipelines {
    /// Pipeline for solid colored primitives
    pub solid: RenderPipeline,

    /// Pipeline for thick lines (wires)
    pub wire: RenderPipeline,

    /// Pipeline for instanced components
    pub component: RenderPipeline,

    /// Pipeline for circle/junction rendering
    pub circle: RenderPipeline,

    /// Pipeline for grid background
    pub grid: RenderPipeline,

    /// Pipeline for text rendering (glyph atlas)
    pub text: RenderPipeline,

    /// Pipeline for selection box overlay
    pub selection: RenderPipeline,

    /// Pipeline for wire preview (dashed)
    pub wire_preview: RenderPipeline,

    /// Camera uniform bind group layout (shared)
    pub camera_bind_group_layout: BindGroupLayout,

    /// Texture bind group layout for text rendering
    pub text_bind_group_layout: BindGroupLayout,

    /// Selection uniform bind group layout
    pub selection_bind_group_layout: BindGroupLayout,
}

impl Pipelines {
    /// Create all render pipelines
    pub fn new(device: &Device, format: TextureFormat) -> Result<Self, GpuError> {
        // Shared camera bind group layout
        let camera_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Camera Bind Group Layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        // Text bind group layout (texture + sampler)
        let text_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Text Bind Group Layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: true },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        // Selection uniform bind group layout
        let selection_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Selection Bind Group Layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Schematic Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        let text_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Text Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout, &text_bind_group_layout],
            push_constant_ranges: &[],
        });

        let selection_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Selection Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout, &selection_bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create shader modules
        let solid_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Solid Shader"),
            source: ShaderSource::Wgsl(shaders::SOLID_SHADER.into()),
        });

        let wire_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Wire Shader"),
            source: ShaderSource::Wgsl(shaders::WIRE_SHADER.into()),
        });

        let component_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Component Shader"),
            source: ShaderSource::Wgsl(shaders::COMPONENT_SHADER.into()),
        });

        let circle_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Circle Shader"),
            source: ShaderSource::Wgsl(shaders::CIRCLE_SHADER.into()),
        });

        let grid_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Grid Shader"),
            source: ShaderSource::Wgsl(shaders::GRID_SHADER.into()),
        });

        let text_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Text Shader"),
            source: ShaderSource::Wgsl(shaders::TEXT_SHADER.into()),
        });

        let selection_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Selection Shader"),
            source: ShaderSource::Wgsl(shaders::SELECTION_SHADER.into()),
        });

        let wire_preview_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Wire Preview Shader"),
            source: ShaderSource::Wgsl(shaders::WIRE_PREVIEW_SHADER.into()),
        });

        // Common color target state with alpha blending
        let color_target = ColorTargetState {
            format,
            blend: Some(BlendState::ALPHA_BLENDING),
            write_mask: ColorWrites::ALL,
        };

        // Solid pipeline (triangles)
        let solid = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Solid Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &solid_shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::layout()],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &solid_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(color_target.clone())],
                compilation_options: Default::default(),
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                polygon_mode: PolygonMode::Fill,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // Wire pipeline (triangles from line strips)
        let wire = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Wire Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &wire_shader,
                entry_point: Some("vs_main"),
                buffers: &[WireVertex::layout()],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &wire_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(color_target.clone())],
                compilation_options: Default::default(),
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                polygon_mode: PolygonMode::Fill,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // Component pipeline (instanced triangles)
        let component = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Component Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &component_shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::layout(), ComponentInstance::layout()],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &component_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(color_target.clone())],
                compilation_options: Default::default(),
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                polygon_mode: PolygonMode::Fill,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // Circle pipeline (instanced quads with circle shader)
        let circle = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Circle Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &circle_shader,
                entry_point: Some("vs_main"),
                buffers: &[QuadVertex::layout(), JunctionInstance::layout()],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &circle_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(color_target.clone())],
                compilation_options: Default::default(),
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                polygon_mode: PolygonMode::Fill,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // Grid pipeline (full-screen quad)
        let grid_vertex_layout = wgpu::VertexBufferLayout {
            array_stride: 8,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x2,
            }],
        };

        let grid = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Grid Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &grid_shader,
                entry_point: Some("vs_main"),
                buffers: &[grid_vertex_layout],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &grid_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(color_target.clone())],
                compilation_options: Default::default(),
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                polygon_mode: PolygonMode::Fill,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // Text pipeline (instanced quads with glyph sampler)
        let text = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Text Pipeline"),
            layout: Some(&text_pipeline_layout),
            vertex: VertexState {
                module: &text_shader,
                entry_point: Some("vs_main"),
                buffers: &[QuadVertex::layout(), TextInstance::layout()],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &text_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(color_target.clone())],
                compilation_options: Default::default(),
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                polygon_mode: PolygonMode::Fill,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // Selection pipeline (box overlay)
        let selection = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Selection Pipeline"),
            layout: Some(&selection_pipeline_layout),
            vertex: VertexState {
                module: &selection_shader,
                entry_point: Some("vs_main"),
                buffers: &[SelectionVertex::layout()],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &selection_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(color_target.clone())],
                compilation_options: Default::default(),
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                polygon_mode: PolygonMode::Fill,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // Wire preview pipeline (dashed lines)
        let wire_preview = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Wire Preview Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &wire_preview_shader,
                entry_point: Some("vs_main"),
                buffers: &[WireVertex::layout()],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &wire_preview_shader,
                entry_point: Some("fs_main"),
                targets: &[Some(color_target)],
                compilation_options: Default::default(),
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                polygon_mode: PolygonMode::Fill,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Ok(Self {
            solid,
            wire,
            component,
            circle,
            grid,
            text,
            selection,
            wire_preview,
            camera_bind_group_layout,
            text_bind_group_layout,
            selection_bind_group_layout,
        })
    }

    /// Create camera bind group for a uniform buffer
    pub fn create_camera_bind_group(&self, device: &Device, camera_buffer: &Buffer) -> BindGroup {
        device.create_bind_group(&BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &self.camera_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        })
    }

    /// Create text bind group for glyph atlas texture
    pub fn create_text_bind_group(
        &self,
        device: &Device,
        texture_view: &wgpu::TextureView,
        sampler: &wgpu::Sampler,
    ) -> BindGroup {
        device.create_bind_group(&BindGroupDescriptor {
            label: Some("Text Bind Group"),
            layout: &self.text_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(texture_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
        })
    }

    /// Create selection bind group for selection uniform buffer
    pub fn create_selection_bind_group(
        &self,
        device: &Device,
        selection_buffer: &Buffer,
    ) -> BindGroup {
        device.create_bind_group(&BindGroupDescriptor {
            label: Some("Selection Bind Group"),
            layout: &self.selection_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: selection_buffer.as_entire_binding(),
            }],
        })
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_struct_fields() {
        // Verify Pipelines struct has all expected fields
        // (compile-time check - if struct changes, this will fail to compile)
        let _field_check = |p: &Pipelines| {
            let _ = &p.solid;
            let _ = &p.wire;
            let _ = &p.component;
            let _ = &p.circle;
            let _ = &p.grid;
            let _ = &p.text;
            let _ = &p.selection;
            let _ = &p.wire_preview;
            let _ = &p.camera_bind_group_layout;
            let _ = &p.text_bind_group_layout;
            let _ = &p.selection_bind_group_layout;
        };
    }
}
