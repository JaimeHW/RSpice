//! GPU Render Integration
//!
//! Integrates the wgpu backend with the Dioxus canvas component.
//! Provides async initialization and render state management.
//!
//! # Architecture
//!
//! The render integration bridges the async wgpu initialization with
//! the synchronous Dioxus component rendering. It uses:
//!
//! - `use_future` for async backend initialization
//! - Signals for reactive state management
//! - Data URLs for displaying GPU-rendered content
//!
//! # Usage
//!
//! ```ignore
//! let render_state = use_gpu_renderer(props.width, props.height);
//!
//! // In render:
//! if let Some(image_data) = &*render_state.read().image_data {
//!     img { src: "{image_data}" }
//! }
//! ```

use dioxus::prelude::*;
use std::sync::Arc;

use crate::gpu::camera::Camera;
use crate::gpu::integration::GpuSchematicBridge;
use crate::state::render_context::RenderContext;
use crate::state::SchematicState;

use super::wgpu_backend::WgpuCanvasBackend;

// =============================================================================
// Render State
// =============================================================================

/// Render state exposed to the canvas component
#[derive(Default, Clone)]
pub struct GpuRenderState {
    /// Rendered image as base64 data URL
    pub image_data: Option<String>,

    /// Whether backend is initialized
    pub initialized: bool,

    /// Whether a render is pending
    pub pending: bool,

    /// Error message if initialization failed
    pub error: Option<String>,

    /// Frame counter
    pub frame_count: u64,

    /// Current FPS (approximate)
    pub fps: f32,

    /// Last rendered topology version
    pub last_topology_version: u64,
}

impl GpuRenderState {
    /// Check if render is needed
    pub fn needs_render(&self, schematic: &SchematicState) -> bool {
        !self.initialized || schematic.topology_version() != self.last_topology_version
    }
}

// =============================================================================
// Render Request
// =============================================================================

/// A request to render the schematic
#[derive(Clone)]
pub struct RenderRequest {
    pub width: u32,
    pub height: u32,
    pub pan: (f64, f64),
    pub zoom: f64,
    pub topology_version: u64,
}

impl RenderRequest {
    /// Create from schematic state
    pub fn from_schematic(schematic: &SchematicState, width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pan: schematic.pan,
            zoom: schematic.zoom,
            topology_version: schematic.topology_version(),
        }
    }
}

// =============================================================================
// Render Manager
// =============================================================================

/// Manages GPU rendering for a canvas
///
/// This is a lightweight manager that coordinates:
/// - Backend lifecycle (lazy initialization)
/// - Render requests (debounced)
/// - State synchronization
pub struct RenderManager {
    /// Current state
    pub state: GpuRenderState,

    /// Cached camera
    camera: Camera,

    /// Bridge for schematic data
    bridge: GpuSchematicBridge,

    /// Last request parameters (for change detection)
    last_request: Option<RenderRequest>,
}

impl Default for RenderManager {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderManager {
    /// Create a new render manager
    pub fn new() -> Self {
        Self {
            state: GpuRenderState::default(),
            camera: Camera::new(800.0, 600.0, 10.0),
            bridge: GpuSchematicBridge::new(),
            last_request: None,
        }
    }

    /// Check if render is needed for a request
    pub fn needs_render(&self, request: &RenderRequest) -> bool {
        match &self.last_request {
            None => true,
            Some(last) => {
                request.topology_version != last.topology_version
                    || request.width != last.width
                    || request.height != last.height
                    || (request.pan.0 - last.pan.0).abs() > 0.5
                    || (request.pan.1 - last.pan.1).abs() > 0.5
                    || (request.zoom - last.zoom).abs() > 0.001
            }
        }
    }

    /// Update camera from request
    pub fn update_camera(&mut self, request: &RenderRequest) {
        self.camera.set_viewport(request.width as f32, request.height as f32);
        // Convert pan to camera position
        // Pan is in screen space, camera position is in world space
        let center_x = (request.width as f64 / 2.0 - request.pan.0) / request.zoom;
        let center_y = (request.height as f64 / 2.0 - request.pan.1) / request.zoom;
        self.camera.position = [center_x as f32, center_y as f32];
        self.camera.set_zoom(request.zoom as f32);
    }

    /// Sync schematic data
    pub fn sync(&mut self, schematic: &SchematicState, render_ctx: &RenderContext) -> bool {
        self.bridge.sync(schematic, render_ctx)
    }

    /// Get bridge for renderer updates
    pub fn bridge(&self) -> &GpuSchematicBridge {
        &self.bridge
    }

    /// Get camera for renderer updates
    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    /// Record that a request was processed
    pub fn record_request(&mut self, request: RenderRequest) {
        self.last_request = Some(request);
        self.state.frame_count += 1;
    }

    /// Mark as initialized
    pub fn set_initialized(&mut self) {
        self.state.initialized = true;
    }

    /// Mark with error
    pub fn set_error(&mut self, error: String) {
        self.state.error = Some(error);
    }

    /// Set rendered image
    pub fn set_image_data(&mut self, data: String) {
        self.state.image_data = Some(data);
        self.state.pending = false;
    }

    /// Mark render as pending
    pub fn set_pending(&mut self) {
        self.state.pending = true;
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Component, ComponentType, Point, Wire};

    // =========================================================================
    // RenderRequest Tests
    // =========================================================================

    #[test]
    fn test_render_request_from_schematic() {
        let mut sch = SchematicState::default();
        sch.pan = (100.0, 200.0);
        sch.zoom = 1.5;
        sch.bump_topology_version();

        let req = RenderRequest::from_schematic(&sch, 800, 600);

        assert_eq!(req.width, 800);
        assert_eq!(req.height, 600);
        assert_eq!(req.pan, (100.0, 200.0));
        assert_eq!(req.zoom, 1.5);
        assert_eq!(req.topology_version, sch.topology_version());
    }

    // =========================================================================
    // RenderManager Tests
    // =========================================================================

    #[test]
    fn test_render_manager_new() {
        let mgr = RenderManager::new();

        assert!(!mgr.state.initialized);
        assert!(mgr.state.image_data.is_none());
        assert!(mgr.state.error.is_none());
        assert_eq!(mgr.state.frame_count, 0);
    }

    #[test]
    fn test_render_manager_needs_render_initially() {
        let mgr = RenderManager::new();
        let sch = SchematicState::default();
        let req = RenderRequest::from_schematic(&sch, 800, 600);

        assert!(mgr.needs_render(&req), "Should need render initially");
    }

    #[test]
    fn test_render_manager_needs_render_after_record() {
        let mut mgr = RenderManager::new();
        let sch = SchematicState::default();
        let req = RenderRequest::from_schematic(&sch, 800, 600);

        mgr.record_request(req.clone());

        assert!(!mgr.needs_render(&req), "Should not need render after recording same request");
    }

    #[test]
    fn test_render_manager_needs_render_topology_change() {
        let mut mgr = RenderManager::new();
        let mut sch = SchematicState::default();
        let req1 = RenderRequest::from_schematic(&sch, 800, 600);
        mgr.record_request(req1);

        sch.bump_topology_version();
        let req2 = RenderRequest::from_schematic(&sch, 800, 600);

        assert!(mgr.needs_render(&req2), "Should need render after topology change");
    }

    #[test]
    fn test_render_manager_needs_render_size_change() {
        let mut mgr = RenderManager::new();
        let sch = SchematicState::default();
        let req1 = RenderRequest::from_schematic(&sch, 800, 600);
        mgr.record_request(req1);

        let req2 = RenderRequest {
            width: 1024,
            height: 768,
            pan: sch.pan,
            zoom: sch.zoom,
            topology_version: sch.topology_version(),
        };

        assert!(mgr.needs_render(&req2), "Should need render after size change");
    }

    #[test]
    fn test_render_manager_needs_render_pan_change() {
        let mut mgr = RenderManager::new();
        let sch = SchematicState::default();
        let req1 = RenderRequest::from_schematic(&sch, 800, 600);
        mgr.record_request(req1);

        let req2 = RenderRequest {
            width: 800,
            height: 600,
            pan: (100.0, 50.0), // Changed pan
            zoom: sch.zoom,
            topology_version: sch.topology_version(),
        };

        assert!(mgr.needs_render(&req2), "Should need render after pan change");
    }

    #[test]
    fn test_render_manager_needs_render_zoom_change() {
        let mut mgr = RenderManager::new();
        let sch = SchematicState::default();
        let req1 = RenderRequest::from_schematic(&sch, 800, 600);
        mgr.record_request(req1);

        let req2 = RenderRequest {
            width: 800,
            height: 600,
            pan: sch.pan,
            zoom: 2.0, // Changed zoom
            topology_version: sch.topology_version(),
        };

        assert!(mgr.needs_render(&req2), "Should need render after zoom change");
    }

    #[test]
    fn test_render_manager_needs_render_small_pan_ignored() {
        let mut mgr = RenderManager::new();
        let sch = SchematicState::default();
        let req1 = RenderRequest::from_schematic(&sch, 800, 600);
        mgr.record_request(req1);

        let req2 = RenderRequest {
            width: 800,
            height: 600,
            pan: (0.1, 0.1), // Very small change
            zoom: sch.zoom,
            topology_version: sch.topology_version(),
        };

        assert!(!mgr.needs_render(&req2), "Should not need render for tiny pan change");
    }

    #[test]
    fn test_render_manager_update_camera() {
        let mut mgr = RenderManager::new();
        let req = RenderRequest {
            width: 1024,
            height: 768,
            pan: (100.0, 50.0),
            zoom: 2.0,
            topology_version: 0,
        };

        mgr.update_camera(&req);

        assert_eq!(mgr.camera.viewport_width, 1024.0);
        assert_eq!(mgr.camera.viewport_height, 768.0);
        assert_eq!(mgr.camera.zoom, 2.0);
    }

    #[test]
    fn test_render_manager_sync() {
        let mut mgr = RenderManager::new();
        let mut sch = SchematicState::default();
        let ctx = RenderContext::new();

        sch.components.push(Component::new(1, ComponentType::Resistor, Point::new(100, 100)));
        sch.wires.push(Wire::new(1, vec![Point::new(0, 0), Point::new(200, 0)]));
        sch.bump_topology_version();

        let updated = mgr.sync(&sch, &ctx);

        assert!(updated, "Should indicate update occurred");
        assert_eq!(mgr.bridge.component_data().len(), 1);
        assert_eq!(mgr.bridge.wire_data().len(), 1);
    }

    #[test]
    fn test_render_manager_state_transitions() {
        let mut mgr = RenderManager::new();

        // Initial state
        assert!(!mgr.state.initialized);
        assert!(mgr.state.image_data.is_none());

        // Set initialized
        mgr.set_initialized();
        assert!(mgr.state.initialized);

        // Set pending
        mgr.set_pending();
        assert!(mgr.state.pending);

        // Set image data
        mgr.set_image_data("data:image/png;base64,ABC".to_string());
        assert!(!mgr.state.pending);
        assert!(mgr.state.image_data.is_some());

        // Set error
        mgr.set_error("Test error".to_string());
        assert!(mgr.state.error.is_some());
    }

    #[test]
    fn test_render_manager_frame_count() {
        let mut mgr = RenderManager::new();
        let sch = SchematicState::default();

        for i in 0..10 {
            let req = RenderRequest::from_schematic(&sch, 800, 600);
            mgr.record_request(req);
            assert_eq!(mgr.state.frame_count, i + 1);
        }
    }

    // =========================================================================
    // GpuRenderState Tests
    // =========================================================================

    #[test]
    fn test_render_state_default() {
        let state = GpuRenderState::default();

        assert!(!state.initialized);
        assert!(!state.pending);
        assert!(state.image_data.is_none());
        assert!(state.error.is_none());
        assert_eq!(state.frame_count, 0);
        assert_eq!(state.fps, 0.0);
        assert_eq!(state.last_topology_version, 0);
    }

    #[test]
    fn test_render_state_needs_render_uninitialized() {
        let state = GpuRenderState::default();
        let sch = SchematicState::default();

        assert!(state.needs_render(&sch), "Uninitialized state should need render");
    }

    #[test]
    fn test_render_state_needs_render_version_mismatch() {
        let mut state = GpuRenderState::default();
        state.initialized = true;
        state.last_topology_version = 1;

        let mut sch = SchematicState::default();
        sch.bump_topology_version();
        sch.bump_topology_version(); // version = 2

        assert!(state.needs_render(&sch), "Version mismatch should need render");
    }

    #[test]
    fn test_render_state_no_render_when_synced() {
        let mut state = GpuRenderState::default();
        state.initialized = true;

        let sch = SchematicState::default();
        state.last_topology_version = sch.topology_version();

        assert!(!state.needs_render(&sch), "Synced state should not need render");
    }
}
