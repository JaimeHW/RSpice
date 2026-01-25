//! Native GPU Overlay Window
//!
//! Creates a native GPU window that overlays the webview canvas area,
//! providing direct 60fps rendering without PNG encoding overhead.
//!
//! # Commercial Architecture
//!
//! Professional EDA tools render directly to hardware surfaces.
//! This module provides that capability by:
//! 1. Creating a native child window positioned over the canvas area
//! 2. Attaching a wgpu surface to render directly
//! 3. Running a dedicated render loop for 60fps updates
//!
//! # Usage
//!
//! ```ignore
//! // Create overlay from Dioxus component
//! let overlay = NativeGpuOverlay::new(x, y, width, height).await?;
//! overlay.start_render_loop(schematic_signal, pan_signal, zoom_signal);
//! ```

#[cfg(not(target_arch = "wasm32"))]
pub mod native {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::thread;
    use tokio::sync::mpsc;

    use winit::application::ApplicationHandler;
    use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
    use winit::event::{ElementState, MouseButton, WindowEvent};
    use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopBuilder};
    #[cfg(target_os = "windows")]
    use winit::platform::windows::EventLoopBuilderExtWindows;
    use winit::window::{Window, WindowId};

    use crate::gpu::camera::Camera;
    use crate::gpu::canvas::schematic_to_gpu_data;
    use crate::gpu::context::GpuContext;
    use crate::gpu::renderer::SchematicRenderer;
    use crate::gpu::surface::GpuSurface;
    use crate::state::SchematicState;

    /// Commands that can be sent to the GPU overlay thread
    #[derive(Debug)]
    pub enum OverlayCommand {
        /// Update pan position
        SetPan(f64, f64),
        /// Update zoom level
        SetZoom(f64),
        /// Update schematic data for rendering
        UpdateSchematic(Box<SchematicState>),
        /// Resize the overlay window
        Resize(u32, u32),
        /// Move the overlay window
        Move(i32, i32),
        /// Request a frame render
        Render,
        /// Shutdown the overlay
        Shutdown,
    }

    /// Handle to control the GPU overlay from the main thread
    #[derive(Clone)]
    pub struct GpuOverlayHandle {
        /// Channel to send commands
        tx: mpsc::UnboundedSender<OverlayCommand>,
        /// Flag indicating if overlay is running
        running: Arc<AtomicBool>,
    }

    impl GpuOverlayHandle {
        /// Set pan position
        pub fn set_pan(&self, x: f64, y: f64) {
            let _ = self.tx.send(OverlayCommand::SetPan(x, y));
        }

        /// Set zoom level
        pub fn set_zoom(&self, zoom: f64) {
            let _ = self.tx.send(OverlayCommand::SetZoom(zoom));
        }

        /// Update schematic for rendering
        pub fn update_schematic(&self, schematic: SchematicState) {
            let _ = self
                .tx
                .send(OverlayCommand::UpdateSchematic(Box::new(schematic)));
        }

        /// Resize overlay
        pub fn resize(&self, width: u32, height: u32) {
            let _ = self.tx.send(OverlayCommand::Resize(width, height));
        }

        /// Move overlay
        pub fn move_to(&self, x: i32, y: i32) {
            let _ = self.tx.send(OverlayCommand::Move(x, y));
        }

        /// Request render
        pub fn render(&self) {
            let _ = self.tx.send(OverlayCommand::Render);
        }

        /// Check if running
        pub fn is_running(&self) -> bool {
            self.running.load(Ordering::SeqCst)
        }

        /// Shutdown the overlay
        pub fn shutdown(&self) {
            let _ = self.tx.send(OverlayCommand::Shutdown);
        }
    }

    /// Application handler for the GPU overlay window
    struct GpuOverlayApp {
        /// Window handle
        window: Option<Arc<Window>>,
        /// GPU context
        context: Option<Arc<GpuContext>>,
        /// GPU surface for rendering
        surface: Option<GpuSurface<'static>>,
        /// Schematic renderer
        renderer: Option<SchematicRenderer>,
        /// Camera for viewport
        camera: Camera,
        /// Current schematic data
        schematic: Option<SchematicState>,
        /// Command receiver
        rx: mpsc::UnboundedReceiver<OverlayCommand>,
        /// Running flag
        running: Arc<AtomicBool>,
        /// Initial position
        initial_x: i32,
        initial_y: i32,
        /// Initial size
        initial_width: u32,
        initial_height: u32,
        /// Mouse state for panning
        mouse_pos: (f64, f64),
        is_panning: bool,
        last_pan: (f64, f64),
        /// Current pan/zoom
        pan: (f64, f64),
        zoom: f64,
    }

    impl GpuOverlayApp {
        fn new(
            rx: mpsc::UnboundedReceiver<OverlayCommand>,
            running: Arc<AtomicBool>,
            x: i32,
            y: i32,
            width: u32,
            height: u32,
        ) -> Self {
            Self {
                window: None,
                context: None,
                surface: None,
                renderer: None,
                camera: Camera::new(width as f32, height as f32, 10.0),
                schematic: None,
                rx,
                running,
                initial_x: x,
                initial_y: y,
                initial_width: width,
                initial_height: height,
                mouse_pos: (0.0, 0.0),
                is_panning: false,
                last_pan: (0.0, 0.0),
                pan: (0.0, 0.0),
                zoom: 1.0,
            }
        }

        /// Process pending commands
        fn process_commands(&mut self) {
            while let Ok(cmd) = self.rx.try_recv() {
                match cmd {
                    OverlayCommand::SetPan(x, y) => {
                        self.pan = (x, y);
                    }
                    OverlayCommand::SetZoom(z) => {
                        self.zoom = z;
                    }
                    OverlayCommand::UpdateSchematic(sch) => {
                        self.schematic = Some(*sch);
                    }
                    OverlayCommand::Resize(w, h) => {
                        if let (Some(ctx), Some(surface)) = (&self.context, &mut self.surface) {
                            surface.resize(ctx, w, h);
                            self.camera.set_viewport(w as f32, h as f32);
                        }
                        if let Some(renderer) = &mut self.renderer {
                            renderer.resize(w, h);
                        }
                    }
                    OverlayCommand::Move(x, y) => {
                        if let Some(window) = &self.window {
                            window.set_outer_position(LogicalPosition::new(x, y));
                        }
                    }
                    OverlayCommand::Render => {
                        // Will render on next frame
                    }
                    OverlayCommand::Shutdown => {
                        self.running.store(false, Ordering::SeqCst);
                    }
                }
            }
        }

        /// Render a frame
        fn render_frame(&mut self) {
            let Some(surface) = &self.surface else { return };
            let Some(renderer) = &mut self.renderer else {
                return;
            };
            let Some(context) = &self.context else { return };

            // Update camera from pan/zoom
            let (width, height) = surface.dimensions();
            self.camera.set_viewport(width as f32, height as f32);
            let center_x = (width as f32 / 2.0 - self.pan.0 as f32) / self.zoom as f32;
            let center_y = (height as f32 / 2.0 - self.pan.1 as f32) / self.zoom as f32;
            self.camera.position = [center_x, center_y];
            self.camera.zoom = self.zoom as f32;

            // Update renderer with schematic data
            if let Some(sch) = &self.schematic {
                let (wires, components, junctions) = schematic_to_gpu_data(sch);
                renderer.update_camera(&self.camera);
                renderer.update_grid(&self.camera);
                renderer.update_wires(&wires);
                renderer.update_components(&components);
                renderer.update_junctions(&junctions);
            }

            // Get surface texture and render
            if let Ok(output) = surface.get_current_texture() {
                let view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                if let Ok(()) = renderer.render(&view) {
                    output.present();
                }
            }
        }
    }

    impl ApplicationHandler for GpuOverlayApp {
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            // Create window
            let window_attrs = Window::default_attributes()
                .with_title("GPU Canvas")
                .with_inner_size(LogicalSize::new(self.initial_width, self.initial_height))
                .with_position(LogicalPosition::new(self.initial_x, self.initial_y))
                .with_decorations(false)
                .with_transparent(false);

            let window = Arc::new(
                event_loop
                    .create_window(window_attrs)
                    .expect("Failed to create window"),
            );

            // Initialize GPU - this needs to be async, so we use pollster
            let context =
                pollster::block_on(GpuContext::new()).expect("Failed to create GPU context");
            let context = Arc::new(context);

            // Create surface
            let surface = unsafe {
                GpuSurface::new(
                    &context,
                    wgpu::SurfaceTarget::from(window.clone()),
                    self.initial_width,
                    self.initial_height,
                )
                .expect("Failed to create surface")
            };

            // Create renderer
            let renderer =
                pollster::block_on(SchematicRenderer::new()).expect("Failed to create renderer");

            self.window = Some(window);
            self.context = Some(context);
            self.surface = Some(surface);
            self.renderer = Some(renderer);

            self.running.store(true, Ordering::SeqCst);
            log::info!("Native GPU overlay window created");
        }

        fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            _id: WindowId,
            event: WindowEvent,
        ) {
            match event {
                WindowEvent::CloseRequested => {
                    self.running.store(false, Ordering::SeqCst);
                    event_loop.exit();
                }
                WindowEvent::Resized(size) => {
                    if size.width > 0 && size.height > 0 {
                        if let (Some(ctx), Some(surface)) = (&self.context, &mut self.surface) {
                            surface.resize(ctx, size.width, size.height);
                            self.camera
                                .set_viewport(size.width as f32, size.height as f32);
                        }
                        if let Some(renderer) = &mut self.renderer {
                            renderer.resize(size.width, size.height);
                        }
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let new_pos = (position.x, position.y);
                    if self.is_panning {
                        let dx = new_pos.0 - self.mouse_pos.0;
                        let dy = new_pos.1 - self.mouse_pos.1;
                        self.pan.0 += dx;
                        self.pan.1 += dy;
                    }
                    self.mouse_pos = new_pos;
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    if button == MouseButton::Middle {
                        self.is_panning = state == ElementState::Pressed;
                    }
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    let scroll = match delta {
                        winit::event::MouseScrollDelta::LineDelta(_, y) => y as f64 * 0.1,
                        winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y * 0.001,
                    };
                    self.zoom *= (1.0 + scroll).max(0.1).min(10.0);
                }
                WindowEvent::RedrawRequested => {
                    self.process_commands();
                    self.render_frame();

                    // Request next frame
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
                _ => {}
            }
        }

        fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
            // Request redraw for continuous rendering
            if let Some(window) = &self.window {
                window.request_redraw();
            }

            // Check for shutdown
            if !self.running.load(Ordering::SeqCst) {
                _event_loop.exit();
            }
        }
    }

    /// Global singleton for the GPU overlay
    static OVERLAY_SPAWNED: std::sync::atomic::AtomicBool =
        std::sync::atomic::AtomicBool::new(false);
    static OVERLAY_TX: once_cell::sync::OnceCell<mpsc::UnboundedSender<OverlayCommand>> =
        once_cell::sync::OnceCell::new();
    static OVERLAY_RUNNING: once_cell::sync::OnceCell<Arc<AtomicBool>> =
        once_cell::sync::OnceCell::new();

    /// Spawn the GPU overlay window (singleton - only one per process)
    ///
    /// Returns a handle to communicate with the overlay.
    /// If the overlay was already spawned, returns a handle to the existing one.
    pub fn spawn_gpu_overlay(x: i32, y: i32, width: u32, height: u32) -> GpuOverlayHandle {
        // Check if already spawned
        if OVERLAY_SPAWNED.swap(true, Ordering::SeqCst) {
            // Already spawned, return handle to existing overlay
            if let (Some(tx), Some(running)) = (OVERLAY_TX.get(), OVERLAY_RUNNING.get()) {
                return GpuOverlayHandle {
                    tx: tx.clone(),
                    running: running.clone(),
                };
            }
        }

        let (tx, rx) = mpsc::unbounded_channel();
        let running = Arc::new(AtomicBool::new(false));

        // Store in global statics for future access
        let _ = OVERLAY_TX.set(tx.clone());
        let _ = OVERLAY_RUNNING.set(running.clone());

        let running_clone = running.clone();

        thread::spawn(move || {
            // Use any_thread extension on Windows to allow event loop on background thread
            #[cfg(target_os = "windows")]
            let event_loop_result = EventLoopBuilder::default().with_any_thread(true).build();
            #[cfg(not(target_os = "windows"))]
            let event_loop_result = EventLoop::new();

            match event_loop_result {
                Ok(event_loop) => {
                    event_loop.set_control_flow(ControlFlow::Poll);
                    let mut app = GpuOverlayApp::new(rx, running_clone, x, y, width, height);
                    if let Err(e) = event_loop.run_app(&mut app) {
                        log::error!("GPU overlay event loop error: {:?}", e);
                    }
                }
                Err(e) => {
                    log::error!("Failed to create GPU overlay event loop: {:?}", e);
                }
            }
        });

        GpuOverlayHandle { tx, running }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlay_handle_creation() {
        // Just test that the module compiles
        // Actual window tests require manual verification
    }
}
