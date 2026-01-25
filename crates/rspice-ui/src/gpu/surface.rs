//! GPU Surface Integration
//!
//! Direct window surface rendering for commercial-grade real-time performance.
//!
//! # Commercial Architecture
//!
//! Professional EDA tools like Cadence render directly to the window surface
//! for immediate visual feedback. This module provides that capability by:
//!
//! 1. Obtaining the raw window handle from the desktop window
//! 2. Creating a wgpu Surface attached to that window
//! 3. Rendering directly to the surface each frame
//!
//! This eliminates the latency of offscreen rendering and provides
//! the smooth 60 FPS experience expected from professional tools.
//!
//! # Platform Support
//!
//! - Windows: D3D12 or Vulkan backend
//! - macOS: Metal backend
//! - Linux: Vulkan backend
//! - Web: WebGPU (separate implementation)

use std::sync::Arc;
use wgpu::{Surface, SurfaceConfiguration, TextureFormat};

use super::context::{GpuContext, GpuError};
use super::renderer::SchematicRenderer;

/// Surface wrapper for direct window rendering
pub struct GpuSurface<'window> {
    /// wgpu surface attached to window
    surface: Surface<'window>,

    /// Surface configuration
    config: SurfaceConfiguration,

    /// Current width
    width: u32,

    /// Current height
    height: u32,
}

impl<'window> GpuSurface<'window> {
    /// Create a new GPU surface from a window handle
    ///
    /// # Safety
    ///
    /// The window must outlive the surface. The caller must ensure
    /// the window handle remains valid for the lifetime of this surface.
    pub unsafe fn new(
        context: &GpuContext,
        window: impl Into<wgpu::SurfaceTarget<'window>>,
        width: u32,
        height: u32,
    ) -> Result<Self, GpuError> {
        let surface = context
            .instance
            .create_surface(window)
            .map_err(|e| GpuError::SurfaceCreation(e.to_string()))?;

        // Get capabilities and select best format
        let caps = surface.get_capabilities(&context.adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);

        let config = SurfaceConfiguration {
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

        Ok(Self {
            surface,
            config,
            width,
            height,
        })
    }

    /// Resize the surface
    pub fn resize(&mut self, context: &GpuContext, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.width = width;
        self.height = height;
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&context.device, &self.config);
    }

    /// Get current surface frame for rendering
    pub fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.surface.get_current_texture()
    }

    /// Get surface format
    pub fn format(&self) -> TextureFormat {
        self.config.format
    }

    /// Get current dimensions
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

/// Managed GPU canvas with surface and renderer
pub struct ManagedGpuCanvas<'window> {
    /// GPU context (device, queue)
    pub context: Arc<GpuContext>,

    /// Window surface
    pub surface: GpuSurface<'window>,

    /// Schematic renderer
    pub renderer: SchematicRenderer,

    /// Frame counter for performance metrics
    frame_count: u64,

    /// Last frame time for FPS calculation
    last_frame_time: std::time::Instant,

    /// Rolling FPS average
    fps: f32,
}

impl<'window> ManagedGpuCanvas<'window> {
    /// Create a new managed GPU canvas
    ///
    /// # Safety
    ///
    /// The window must outlive the canvas.
    pub async unsafe fn new(
        window: impl Into<wgpu::SurfaceTarget<'window>>,
        width: u32,
        height: u32,
    ) -> Result<Self, GpuError> {
        // Create shared context
        let context = Arc::new(GpuContext::new().await?);

        // Create surface
        let surface = GpuSurface::new(&context, window, width, height)?;

        // Create renderer
        let renderer = SchematicRenderer::new().await?;

        Ok(Self {
            context,
            surface,
            renderer,
            frame_count: 0,
            last_frame_time: std::time::Instant::now(),
            fps: 0.0,
        })
    }

    /// Resize canvas
    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface.resize(&self.context, width, height);
        self.renderer.resize(width, height);
    }

    /// Render a frame
    pub fn render(&mut self) -> Result<(), GpuError> {
        // Get current frame
        let output = self
            .surface
            .get_current_texture()
            .map_err(|e| GpuError::SurfaceCreation(e.to_string()))?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Render schematic
        self.renderer.render(&view)?;

        // Present
        output.present();

        // Update FPS
        self.frame_count += 1;
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.last_frame_time).as_secs_f32();
        if elapsed >= 1.0 {
            self.fps = self.frame_count as f32 / elapsed;
            self.frame_count = 0;
            self.last_frame_time = now;
        }

        Ok(())
    }

    /// Get current FPS
    pub fn fps(&self) -> f32 {
        self.fps
    }
}

// =============================================================================
// Window Handle Wrapper for Cross-Platform Support
// =============================================================================

/// Raw window handle wrapper for surface creation
#[cfg(not(target_arch = "wasm32"))]
pub mod native {
    use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

    /// Wrapper to pass window to wgpu
    pub struct WindowWrapper<'a, W: HasWindowHandle + HasDisplayHandle> {
        pub window: &'a W,
    }

    impl<'a, W: HasWindowHandle + HasDisplayHandle> WindowWrapper<'a, W> {
        pub fn new(window: &'a W) -> Self {
            Self { window }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub mod web {
    /// Web canvas wrapper for WebGPU
    pub struct CanvasWrapper {
        pub canvas: web_sys::HtmlCanvasElement,
    }
}
