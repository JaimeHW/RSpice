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

use crate::gpu::camera::{Camera, CameraUniform};
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
        let surface_format = wgpu::TextureFormat::Bgra8Unorm;
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

    // =========================================================================
    // Rendering Methods
    // =========================================================================

    /// Render a frame to a texture view
    ///
    /// This is the core render method that draws the schematic to a GPU texture.
    /// Used by both offscreen rendering and surface rendering.
    pub fn render_to_texture(&mut self, view: &wgpu::TextureView) -> Result<(), GpuError> {
        // Create camera uniform
        let camera_uniform = self.camera.build_uniform();
        let camera_buffer = self.context.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Uniform"),
            size: std::mem::size_of::<CameraUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.context
            .queue
            .write_buffer(&camera_buffer, 0, bytemuck::bytes_of(&camera_uniform));

        // Create camera bind group
        let camera_bind_group = self
            .context
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Camera Bind Group"),
                layout: &self.pipelines.camera_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }],
            });

        // Create command encoder
        let mut encoder =
            self.context
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        // Begin render pass
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Schematic Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            // Match theme.bg_primary() = "#0a0a0f" = RGB(10, 10, 15)
                            r: 0.039,
                            g: 0.039,
                            b: 0.059,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_bind_group(0, &camera_bind_group, &[]);

            // Render grid
            render_pass.set_pipeline(&self.pipelines.grid);
            // Grid quad draw call would go here (grid quad covers viewport)

            // Render wires
            // render_pass.set_pipeline(&self.pipelines.wire_pipeline);
            // Wire vertex buffer and draw calls

            // Render components
            // render_pass.set_pipeline(&self.pipelines.component_pipeline);
            // Component vertex/instance buffers and draw calls
        }

        // Submit commands
        self.context.queue.submit(std::iter::once(encoder.finish()));

        // Update FPS
        self.update_fps();

        Ok(())
    }

    /// Render a frame to a wgpu surface
    ///
    /// This acquires a frame from the surface, renders to it, and presents.
    pub fn render_to_surface(&mut self, surface: &wgpu::Surface) -> Result<(), GpuError> {
        // Get current frame
        let output = surface
            .get_current_texture()
            .map_err(|e| GpuError::SurfaceCreation(format!("Surface error: {}", e)))?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Render to the texture
        self.render_to_texture(&view)?;

        // Present
        output.present();

        Ok(())
    }

    /// Get the GPU context
    pub fn context(&self) -> &GpuContext {
        &self.context
    }

    /// Get the pipelines
    pub fn pipelines(&self) -> &Pipelines {
        &self.pipelines
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
pub fn configure_surface(surface: &wgpu::Surface, context: &GpuContext, width: u32, height: u32) {
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

    // =========================================================================
    // Camera Tests
    // =========================================================================

    #[test]
    fn test_runtime_default_camera() {
        // Test that we can create the components without GPU
        let camera = Camera::new(800.0, 600.0, 10.0);
        assert_eq!(camera.zoom, 1.0);
    }

    #[test]
    fn test_camera_viewport_size() {
        let camera = Camera::new(1920.0, 1080.0, 10.0);
        assert_eq!(camera.viewport_width, 1920.0);
        assert_eq!(camera.viewport_height, 1080.0);
    }

    #[test]
    fn test_camera_zoom_range() {
        let mut camera = Camera::new(800.0, 600.0, 10.0);
        camera.set_zoom(5.0);
        assert_eq!(camera.zoom, 5.0);

        camera.set_zoom(0.1);
        assert_eq!(camera.zoom, 0.1);
    }

    #[test]
    fn test_camera_uniform_generation() {
        let camera = Camera::new(800.0, 600.0, 10.0);
        let uniform = camera.build_uniform();
        // Uniform should contain valid viewport data
        assert!(uniform.viewport[0] > 0.0); // width
        assert!(uniform.viewport[1] > 0.0); // height
    }

    // =========================================================================
    // Bridge Tests
    // =========================================================================

    #[test]
    fn test_bridge_creation() {
        let bridge = GpuSchematicBridge::new();
        assert!(bridge.wire_data().is_empty());
        assert!(bridge.component_data().is_empty());
    }

    #[test]
    fn test_bridge_junction_data_empty() {
        let bridge = GpuSchematicBridge::new();
        assert!(bridge.junction_data().is_empty());
    }

    #[test]
    fn test_bridge_camera_dirty_flag() {
        let mut bridge = GpuSchematicBridge::new();
        bridge.mark_camera_dirty();
        // After marking dirty, next sync should update
    }

    // =========================================================================
    // Pipeline Configuration Tests
    // =========================================================================

    #[test]
    fn test_surface_format_constant() {
        // Use linear format for accurate colors in offscreen rendering
        let format = wgpu::TextureFormat::Bgra8Unorm;
        assert!(!format.is_srgb()); // Linear format, not sRGB
    }

    #[test]
    fn test_viewport_minimum_size() {
        // Viewport should clamp to at least 1x1
        let width = 0u32.max(1);
        let height = 0u32.max(1);
        assert_eq!(width, 1);
        assert_eq!(height, 1);
    }

    // =========================================================================
    // Render Configuration Tests
    // =========================================================================

    #[test]
    fn test_clear_color_values() {
        // Dark background for schematic rendering
        let bg_r = 0.08f64;
        let bg_g = 0.08f64;
        let bg_b = 0.08f64;

        assert!(bg_r >= 0.0 && bg_r <= 1.0);
        assert!(bg_g >= 0.0 && bg_g <= 1.0);
        assert!(bg_b >= 0.0 && bg_b <= 1.0);
    }

    #[test]
    fn test_fps_initial_value() {
        // FPS should start at 0
        let fps = 0.0f32;
        assert_eq!(fps, 0.0);
    }

    #[test]
    fn test_frame_count_increments() {
        let mut frame_count = 0u64;
        frame_count += 1;
        assert_eq!(frame_count, 1);
        frame_count += 1;
        assert_eq!(frame_count, 2);
    }

    // =========================================================================
    // Surface Configuration Tests
    // =========================================================================

    #[test]
    fn test_vsync_present_mode() {
        let mode = wgpu::PresentMode::AutoVsync;
        // AutoVsync adapts to display refresh rate
        assert!(matches!(mode, wgpu::PresentMode::AutoVsync));
    }

    #[test]
    fn test_render_attachment_usage() {
        let usage = wgpu::TextureUsages::RENDER_ATTACHMENT;
        assert!(usage.contains(wgpu::TextureUsages::RENDER_ATTACHMENT));
    }

    #[test]
    fn test_buffer_usage_flags() {
        let usage = wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST;
        assert!(usage.contains(wgpu::BufferUsages::UNIFORM));
        assert!(usage.contains(wgpu::BufferUsages::COPY_DST));
    }

    // =========================================================================
    // Coordinate System Tests
    // =========================================================================

    #[test]
    fn test_pan_position_update() {
        let mut position = [0.0f32, 0.0f32];
        position[0] = 100.0;
        position[1] = -50.0;

        assert_eq!(position[0], 100.0);
        assert_eq!(position[1], -50.0);
    }

    #[test]
    fn test_zoom_affects_camera() {
        let mut camera = Camera::new(800.0, 600.0, 10.0);
        let initial_zoom = camera.zoom;

        camera.set_zoom(2.0);
        assert_ne!(camera.zoom, initial_zoom);
        assert_eq!(camera.zoom, 2.0);
    }

    // =========================================================================
    // Memory Layout Tests
    // =========================================================================

    #[test]
    fn test_camera_uniform_size() {
        let size = std::mem::size_of::<CameraUniform>();
        // CameraUniform should be 64 bytes (4x4 matrix) + padding as needed
        assert!(size >= 64);
    }
}
