//! WebGPU Runtime
//!
//! Web-compatible GPU runtime for direct canvas rendering.
//! This module provides the integration layer between wgpu and the browser's
//! HTMLCanvasElement for true real-time GPU-accelerated schematic rendering.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │                     Dioxus Component                    │
//! │  ┌────────────────────────────────────────────────────┐ │
//! │  │              <canvas id="gpu-canvas">              │ │
//! │  └──────────────────────┬─────────────────────────────┘ │
//! │                         │ wgpu::Surface                 │
//! │  ┌──────────────────────▼─────────────────────────────┐ │
//! │  │                WebGpuRuntime                       │ │
//! │  │  - context: GpuContext                             │ │
//! │  │  - surface: wgpu::Surface                          │ │
//! │  │  - pipelines: Pipelines                            │ │
//! │  │  - bridge: GpuSchematicBridge                      │ │
//! │  └────────────────────────────────────────────────────┘ │
//! └─────────────────────────────────────────────────────────┘
//! ```
//!
//! # Web Initialization
//!
//! On web targets, initialization is async and uses web_sys to get the canvas:
//! 1. Get HTMLCanvasElement by ID from the DOM
//! 2. Create wgpu surface from canvas
//! 3. Initialize pipelines and buffers
//! 4. Start requestAnimationFrame render loop

use std::sync::Arc;

use crate::gpu::camera::Camera;
use crate::gpu::context::{GpuContext, GpuError};
use crate::gpu::gpu_cache::GpuRenderCache;
use crate::gpu::integration::GpuSchematicBridge;
use crate::gpu::pipeline::Pipelines;
use crate::gpu::renderer::{ComponentData, JunctionData, WireData};
use crate::state::render_context::RenderContext;
use crate::state::SchematicState;

// =============================================================================
// WebGPU Runtime State
// =============================================================================

/// Runtime state for WebGPU canvas rendering.
///
/// This struct holds all GPU resources needed for rendering and is designed
/// to be stored in a Dioxus signal or static for the render loop.
pub struct WebGpuRuntime {
    /// GPU context (device, queue, adapter)
    pub context: Arc<GpuContext>,

    /// Render pipelines
    pub pipelines: Arc<Pipelines>,

    /// Schematic bridge with cache
    pub bridge: GpuSchematicBridge,

    /// Camera state
    pub camera: Camera,

    /// Surface configuration
    pub surface_format: wgpu::TextureFormat,

    /// Current viewport dimensions
    pub width: u32,
    pub height: u32,

    /// Frame counter for performance monitoring
    frame_count: u64,

    /// Last frame time for FPS calculation
    #[cfg(not(target_arch = "wasm32"))]
    last_frame_time: std::time::Instant,

    /// Rolling FPS
    fps: f32,
}

impl WebGpuRuntime {
    /// Create a new WebGPU runtime.
    ///
    /// This is an async operation that initializes the GPU context and pipelines.
    pub async fn new(width: u32, height: u32) -> Result<Self, GpuError> {
        // Initialize GPU context
        let context = Arc::new(GpuContext::new().await?);

        // Create pipelines
        let surface_format = wgpu::TextureFormat::Bgra8UnormSrgb;
        let pipelines = Arc::new(Pipelines::new(&context.device, surface_format)?);

        // Create bridge
        let bridge = GpuSchematicBridge::new();

        // Create camera
        let camera = Camera::new(width as f32, height as f32, 10.0);

        Ok(Self {
            context,
            pipelines,
            bridge,
            camera,
            surface_format,
            width,
            height,
            frame_count: 0,
            #[cfg(not(target_arch = "wasm32"))]
            last_frame_time: std::time::Instant::now(),
            fps: 0.0,
        })
    }

    /// Resize the viewport
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width.max(1);
        self.height = height.max(1);
        self.camera.set_viewport(width as f32, height as f32);
    }

    /// Update camera pan (sets camera center position)
    pub fn set_pan(&mut self, pan_x: f32, pan_y: f32) {
        self.camera.position = [pan_x, pan_y];
        self.bridge.mark_camera_dirty();
    }

    /// Update camera zoom
    pub fn set_zoom(&mut self, zoom: f32) {
        self.camera.set_zoom(zoom);
        self.bridge.mark_camera_dirty();
    }

    /// Synchronize schematic state to GPU cache
    ///
    /// Returns true if render is needed (data changed)
    pub fn sync(&mut self, schematic: &SchematicState, render_ctx: &RenderContext) -> bool {
        self.bridge.sync(schematic, render_ctx)
    }

    /// Get wire data for rendering
    pub fn wire_data(&self) -> &[WireData] {
        self.bridge.wire_data()
    }

    /// Get component data for rendering
    pub fn component_data(&self) -> &[ComponentData] {
        self.bridge.component_data()
    }

    /// Get junction data for rendering
    pub fn junction_data(&self) -> &[JunctionData] {
        self.bridge.junction_data()
    }

    /// Get current camera
    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    /// Get FPS
    pub fn fps(&self) -> f32 {
        self.fps
    }

    /// Update FPS counter (call once per frame)
    pub fn update_fps(&mut self) {
        self.frame_count += 1;

        #[cfg(not(target_arch = "wasm32"))]
        {
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(self.last_frame_time).as_secs_f32();
            if elapsed >= 1.0 {
                self.fps = self.frame_count as f32 / elapsed;
                self.frame_count = 0;
                self.last_frame_time = now;
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            // On web, use performance.now() for timing
            // For now, just count frames
            if self.frame_count % 60 == 0 {
                self.fps = 60.0; // Approximate
            }
        }
    }
}

// =============================================================================
// Web-specific Surface Creation
// =============================================================================

/// Create a wgpu surface from an HTML canvas element ID.
///
/// # Web Usage
///
/// ```ignore
/// let surface = create_surface_from_canvas("gpu-canvas", &runtime.context)?;
/// ```
#[cfg(target_arch = "wasm32")]
pub fn create_surface_from_canvas(
    canvas_id: &str,
    context: &GpuContext,
) -> Result<wgpu::Surface<'static>, GpuError> {
    use wasm_bindgen::JsCast;
    use web_sys::HtmlCanvasElement;

    // Get document
    let window = web_sys::window().ok_or(GpuError::SurfaceCreation("No window".into()))?;
    let document = window
        .document()
        .ok_or(GpuError::SurfaceCreation("No document".into()))?;

    // Get canvas element
    let canvas = document
        .get_element_by_id(canvas_id)
        .ok_or(GpuError::SurfaceCreation(format!(
            "Canvas '{}' not found",
            canvas_id
        )))?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| GpuError::SurfaceCreation("Element is not a canvas".into()))?;

    // Create wgpu surface target
    let surface_target = wgpu::SurfaceTarget::Canvas(canvas);

    // Create surface
    context
        .instance
        .create_surface(surface_target)
        .map_err(|e| GpuError::SurfaceCreation(e.to_string()))
}

/// Configure a surface for rendering
#[cfg(target_arch = "wasm32")]
pub fn configure_surface(
    surface: &wgpu::Surface,
    context: &GpuContext,
    width: u32,
    height: u32,
) {
    let caps = surface.get_capabilities(&context.adapter);
    let format = caps
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(caps.formats[0]);

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: width.max(1),
        height: height.max(1),
        present_mode: wgpu::PresentMode::AutoVsync,
        alpha_mode: caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };

    surface.configure(&context.device, &config);
}

// =============================================================================
// Native Surface Creation (Desktop)
// =============================================================================

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use super::*;

    /// Create a wgpu surface from a native window handle
    ///
    /// # Safety
    ///
    /// The window must remain valid for the lifetime of the returned surface.
    pub unsafe fn create_surface_from_window<'w>(
        window: impl Into<wgpu::SurfaceTarget<'w>>,
        context: &GpuContext,
    ) -> Result<wgpu::Surface<'w>, GpuError> {
        context
            .instance
            .create_surface(window)
            .map_err(|e| GpuError::SurfaceCreation(e.to_string()))
    }

    /// Configure a surface for rendering
    pub fn configure_surface(
        surface: &wgpu::Surface,
        context: &GpuContext,
        width: u32,
        height: u32,
    ) {
        let caps = surface.get_capabilities(&context.adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: width.max(1),
            height: height.max(1),
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&context.device, &config);
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_default_camera() {
        // Test that we can create the components without GPU
        let camera = Camera::new(800.0, 600.0, 10.0);
        assert_eq!(camera.zoom, 1.0);
    }

    #[test]
    fn test_bridge_creation() {
        let bridge = GpuSchematicBridge::new();
        assert!(bridge.wire_data().is_empty());
        assert!(bridge.component_data().is_empty());
    }
}
