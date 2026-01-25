//! GPU Context
//!
//! Manages wgpu device, queue, and surface for schematic rendering.
//! This is the core GPU infrastructure that other modules build upon.

use std::sync::Arc;
use wgpu::{
    Adapter, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor, Limits,
    PowerPreference, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureFormat,
    TextureUsages,
};

/// GPU context containing all wgpu resources needed for rendering.
///
/// This struct owns the wgpu instance, adapter, device, and queue.
/// It provides a centralized place for GPU resource management.
#[derive(Debug)]
pub struct GpuContext {
    /// wgpu instance - entry point to the graphics API
    pub instance: Instance,

    /// Graphics adapter (physical GPU)
    pub adapter: Adapter,

    /// Logical device for creating resources
    pub device: Arc<Device>,

    /// Command queue for submitting work
    pub queue: Arc<Queue>,

    /// Preferred texture format for this surface
    pub surface_format: TextureFormat,
}

impl GpuContext {
    /// Create a new GPU context, selecting the best available adapter.
    ///
    /// This function initializes wgpu and selects a suitable GPU.
    /// It prefers discrete GPUs for better performance.
    pub async fn new() -> Result<Self, GpuError> {
        // Create instance with all available backends
        let instance = Instance::new(InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Request a high-performance adapter
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok_or(GpuError::NoAdapter)?;

        log::info!("Selected GPU adapter: {:?}", adapter.get_info().name);

        // Request device with reasonable limits
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("RSpice Schematic Renderer"),
                    required_features: Features::empty(),
                    required_limits: Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .map_err(|e| GpuError::DeviceCreation(e.to_string()))?;

        // Prefer sRGB format for accurate colors
        let surface_format = TextureFormat::Bgra8UnormSrgb;

        Ok(Self {
            instance,
            adapter,
            device: Arc::new(device),
            queue: Arc::new(queue),
            surface_format,
        })
    }

    /// Create a surface configuration for the given dimensions.
    pub fn surface_config(&self, width: u32, height: u32) -> SurfaceConfiguration {
        SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            width: width.max(1),
            height: height.max(1),
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
    }

    /// Configure a surface for rendering.
    pub fn configure_surface(&self, surface: &Surface, width: u32, height: u32) {
        let config = self.surface_config(width, height);
        surface.configure(&self.device, &config);
    }
}

/// Errors that can occur during GPU initialization
#[derive(Debug, Clone)]
pub enum GpuError {
    /// No suitable GPU adapter found
    NoAdapter,
    /// Failed to create device
    DeviceCreation(String),
    /// Surface creation failed
    SurfaceCreation(String),
    /// Shader compilation failed
    ShaderCompilation(String),
    /// Pipeline creation failed
    PipelineCreation(String),
    /// Buffer mapping failed
    BufferMap(String),
    /// Texture readback failed
    TextureRead(String),
    /// Generic error
    Other(String),
}

impl std::fmt::Display for GpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GpuError::NoAdapter => write!(f, "No suitable GPU adapter found"),
            GpuError::DeviceCreation(e) => write!(f, "Failed to create GPU device: {}", e),
            GpuError::SurfaceCreation(e) => write!(f, "Failed to create surface: {}", e),
            GpuError::ShaderCompilation(e) => write!(f, "Shader compilation failed: {}", e),
            GpuError::PipelineCreation(e) => write!(f, "Pipeline creation failed: {}", e),
            GpuError::BufferMap(e) => write!(f, "Buffer mapping failed: {}", e),
            GpuError::TextureRead(e) => write!(f, "Texture readback failed: {}", e),
            GpuError::Other(e) => write!(f, "GPU error: {}", e),
        }
    }
}

impl std::error::Error for GpuError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_error_display() {
        let err = GpuError::NoAdapter;
        assert!(err.to_string().contains("adapter"));

        let err = GpuError::DeviceCreation("test error".to_string());
        assert!(err.to_string().contains("test error"));
    }
}
