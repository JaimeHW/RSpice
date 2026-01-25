//! wgpu Canvas Backend
//!
//! Connects the wgpu SchematicRenderer to the Dioxus GPU canvas component.
//! This is the glue layer that enables real-time GPU-accelerated schematic rendering.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │              GpuSchematicCanvas (Dioxus Component)          │
//! │                         │                                   │
//! │                         ▼                                   │
//! │ ┌─────────────────────────────────────────────────────────┐ │
//! │ │                  WgpuCanvasBackend                      │ │
//! │ │   - Manages SchematicRenderer lifecycle                 │ │
//! │ │   - Syncs SchematicState → GPU buffers                  │ │
//! │ │   - Issues render commands                              │ │
//! │ └─────────────────────────────────────────────────────────┘ │
//! │                         │                                   │
//! │                         ▼                                   │
//! │ ┌─────────────────────────────────────────────────────────┐ │
//! │ │                 SchematicRenderer                       │ │
//! │ │   - GPU pipelines, buffers, shaders                     │ │
//! │ │   - Render to texture or surface                        │ │
//! │ └─────────────────────────────────────────────────────────┘ │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Usage
//!
//! The backend is initialized lazily on first render and then reused:
//!
//! ```ignore
//! // Initialize backend (async, call once)
//! let backend = WgpuCanvasBackend::new(800, 600).await?;
//!
//! // Each frame:
//! backend.sync(&schematic, &render_ctx)?;
//! let image_data = backend.render_to_data_url()?;
//! ```

use std::sync::Arc;

use crate::gpu::camera::Camera;
use crate::gpu::context::{GpuContext, GpuError};
use crate::gpu::gpu_cache::GpuRenderCache;
use crate::gpu::integration::GpuSchematicBridge;
use crate::gpu::pipeline::Pipelines;
use crate::gpu::renderer::SchematicRenderer;
use crate::state::render_context::RenderContext;
use crate::state::SchematicState;

// =============================================================================
// wgpu Canvas Backend
// =============================================================================

/// Backend state for wgpu-accelerated canvas rendering.
///
/// Encapsulates the SchematicRenderer and provides a high-level API
/// for rendering schematics to textures or data URLs.
pub struct WgpuCanvasBackend {
    /// The main schematic renderer
    renderer: SchematicRenderer,

    /// GPU schematic bridge for state sync
    bridge: GpuSchematicBridge,

    /// Camera state
    camera: Camera,

    /// Current viewport dimensions
    width: u32,
    height: u32,

    /// Last rendered topology version
    last_topology_version: u64,

    /// Frame counter
    frame_count: u64,

    /// Whether renderer needs update
    needs_render: bool,
}

impl WgpuCanvasBackend {
    /// Create a new wgpu canvas backend.
    ///
    /// This is an async operation that initializes GPU resources.
    pub async fn new(width: u32, height: u32) -> Result<Self, GpuError> {
        let renderer = SchematicRenderer::new().await?;
        let bridge = GpuSchematicBridge::new();
        let camera = Camera::new(width as f32, height as f32, 10.0);

        Ok(Self {
            renderer,
            bridge,
            camera,
            width,
            height,
            last_topology_version: 0,
            frame_count: 0,
            needs_render: true,
        })
    }

    /// Check if backend is initialized
    pub fn is_initialized(&self) -> bool {
        true // If we get here, we're initialized
    }

    /// Resize the viewport
    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.width = width;
        self.height = height;
        self.camera.set_viewport(width as f32, height as f32);
        self.renderer.resize(width, height);
        self.needs_render = true;
    }

    /// Update camera pan (in world coordinates)
    pub fn set_camera_center(&mut self, center_x: f32, center_y: f32) {
        self.camera.position = [center_x, center_y];
        self.renderer.update_camera(&self.camera);
        self.needs_render = true;
    }

    /// Update camera zoom
    pub fn set_zoom(&mut self, zoom: f32) {
        self.camera.set_zoom(zoom);
        self.renderer.update_camera(&self.camera);
        self.needs_render = true;
    }

    /// Pan by screen pixels
    pub fn pan_by_pixels(&mut self, dx: f32, dy: f32) {
        self.camera.pan_by_pixels(dx, dy);
        self.renderer.update_camera(&self.camera);
        self.needs_render = true;
    }

    /// Get current camera
    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    /// Get current zoom level
    pub fn zoom(&self) -> f32 {
        self.camera.zoom
    }

    /// Synchronize schematic state to GPU.
    ///
    /// Returns true if render is needed.
    pub fn sync(&mut self, schematic: &SchematicState, render_ctx: &RenderContext) -> bool {
        let topology_changed = schematic.topology_version() != self.last_topology_version;

        if topology_changed {
            self.bridge.sync(schematic, render_ctx);
            self.last_topology_version = schematic.topology_version();

            // Update renderer with new data
            self.renderer.update_wires(self.bridge.wire_data());
            self.renderer.update_components(self.bridge.component_data());
            self.renderer.update_junctions(self.bridge.junction_data());
            self.renderer.update_grid(&self.camera);

            // Generate and update labels
            let component_labels = self.bridge.generate_component_labels(schematic);
            let net_labels = self.bridge.generate_net_labels(schematic);
            self.renderer.update_labels(&component_labels, &net_labels);

            self.needs_render = true;
        }

        self.needs_render
    }

    /// Get component labels for current schematic
    pub fn component_labels(&self, schematic: &SchematicState) -> Vec<crate::gpu::text::LabelData> {
        self.bridge.generate_component_labels(schematic)
    }

    /// Get net labels for current schematic
    pub fn net_labels(&self, schematic: &SchematicState) -> Vec<crate::gpu::text::LabelData> {
        self.bridge.generate_net_labels(schematic)
    }

    /// Render to a data URL (base64 PNG).
    ///
    /// This is suitable for displaying in an <img> tag.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn render_to_data_url(&mut self) -> Result<String, GpuError> {
        if !self.needs_render {
            return Ok(String::new());
        }

        // Render to image bytes
        let pixels = self.renderer.render_to_image(self.width, self.height)?;

        if pixels.is_empty() {
            return Ok(String::new());
        }

        // Encode as PNG
        let mut png_data = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut png_data, self.width, self.height);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder
                .write_header()
                .map_err(|e| GpuError::SurfaceCreation(e.to_string()))?;
            writer
                .write_image_data(&pixels)
                .map_err(|e| GpuError::SurfaceCreation(e.to_string()))?;
        }

        // Encode as base64 data URL
        let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &png_data);

        self.needs_render = false;
        self.frame_count += 1;

        Ok(format!("data:image/png;base64,{}", b64))
    }

    /// Render to a data URL (WASM stub).
    #[cfg(target_arch = "wasm32")]
    pub fn render_to_data_url(&mut self) -> Result<String, GpuError> {
        // WebGPU rendering to data URL not yet implemented
        // For web, we render directly to canvas surface instead
        Err(GpuError::SurfaceCreation(
            "WebGPU direct canvas rendering should be used on web".to_string(),
        ))
    }

    /// Get frame count
    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    /// Get GPU context (if needed for surface creation)
    pub fn context(&self) -> &GpuContext {
        self.renderer.context()
    }
}

// =============================================================================
// Lazy Initialization Helper
// =============================================================================

use std::sync::OnceLock;
use tokio::sync::Mutex;

/// Global backend instance for sharing across renders.
///
/// Using OnceLock + Mutex ensures thread-safe lazy initialization.
static BACKEND: OnceLock<Mutex<Option<WgpuCanvasBackend>>> = OnceLock::new();

/// Get or initialize the global wgpu backend.
///
/// This is used for efficient rendering across multiple frames
/// without re-creating GPU resources each time.
pub async fn get_or_init_backend(width: u32, height: u32) -> Result<&'static Mutex<Option<WgpuCanvasBackend>>, GpuError> {
    let mutex = BACKEND.get_or_init(|| Mutex::new(None));

    {
        let mut guard = mutex.lock().await;
        if guard.is_none() {
            *guard = Some(WgpuCanvasBackend::new(width, height).await?);
        }
    }

    Ok(mutex)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Component, ComponentType, Point, Wire};

    // =========================================================================
    // Camera Tests
    // =========================================================================

    #[test]
    fn test_camera_default_construction() {
        let camera = Camera::new(800.0, 600.0, 10.0);
        assert_eq!(camera.zoom, 1.0, "Default zoom should be 1.0");
        assert_eq!(camera.viewport_width, 800.0, "Viewport width mismatch");
        assert_eq!(camera.viewport_height, 600.0, "Viewport height mismatch");
        assert_eq!(camera.position, [0.0, 0.0], "Default position should be origin");
        assert_eq!(camera.grid_size, 10.0, "Grid size mismatch");
    }

    #[test]
    fn test_camera_viewport_update() {
        let mut camera = Camera::new(800.0, 600.0, 10.0);
        camera.set_viewport(1920.0, 1080.0);
        assert_eq!(camera.viewport_width, 1920.0);
        assert_eq!(camera.viewport_height, 1080.0);
    }

    #[test]
    fn test_camera_zoom_clamping() {
        let mut camera = Camera::new(800.0, 600.0, 10.0);
        
        // Test max zoom clamping
        camera.set_zoom(100.0);
        assert!(camera.zoom <= camera.max_zoom, "Zoom should be clamped to max");
        
        // Test min zoom clamping
        camera.set_zoom(0.001);
        assert!(camera.zoom >= camera.min_zoom, "Zoom should be clamped to min");
    }

    #[test]
    fn test_camera_zoom_by_factor() {
        let mut camera = Camera::new(800.0, 600.0, 10.0);
        let initial_zoom = camera.zoom;
        
        camera.zoom_by(1.1); // 10% zoom in
        assert!(camera.zoom > initial_zoom, "Zoom should increase");
        
        camera.zoom_by(0.9); // 10% zoom out
        // Should be approximately back to initial (allowing for floating point)
    }

    #[test]
    fn test_camera_pan_by_pixels() {
        let mut camera = Camera::new(800.0, 600.0, 10.0);
        let initial_pos = camera.position;
        
        camera.pan_by_pixels(100.0, 50.0);
        
        // Position should change (exact values depend on zoom)
        assert!(
            camera.position[0] != initial_pos[0] || camera.position[1] != initial_pos[1],
            "Position should change after pan"
        );
    }

    #[test]
    fn test_camera_screen_to_world_at_zoom_1() {
        let camera = Camera::new(800.0, 600.0, 10.0);
        
        // At zoom 1, screen center should map to world origin
        let [world_x, world_y] = camera.screen_to_world(400.0, 300.0);
        
        // Allow small floating point tolerance
        assert!((world_x - 0.0).abs() < 0.1, "Center X should be ~0");
        assert!((world_y - 0.0).abs() < 0.1, "Center Y should be ~0");
    }

    #[test]
    fn test_camera_world_to_screen_round_trip() {
        let camera = Camera::new(800.0, 600.0, 10.0);
        
        // Test round-trip conversion
        let world_point = [100.0f32, 50.0f32];
        let [screen_x, screen_y] = camera.world_to_screen(world_point[0], world_point[1]);
        let [back_x, back_y] = camera.screen_to_world(screen_x, screen_y);
        
        assert!((back_x - world_point[0]).abs() < 0.01, "X round-trip failed");
        assert!((back_y - world_point[1]).abs() < 0.01, "Y round-trip failed");
    }

    #[test]
    fn test_camera_world_bounds() {
        let camera = Camera::new(800.0, 600.0, 10.0);
        let bounds = camera.world_bounds();
        
        // Bounds should be centered on origin for default camera
        assert!(bounds.min_x < 0.0, "Min X should be negative");
        assert!(bounds.max_x > 0.0, "Max X should be positive");
        assert!(bounds.min_y < 0.0, "Min Y should be negative");
        assert!(bounds.max_y > 0.0, "Max Y should be positive");
    }

    #[test]
    fn test_camera_build_uniform() {
        let camera = Camera::new(800.0, 600.0, 10.0);
        let uniform = camera.build_uniform();
        
        // Verify uniform has reasonable values
        assert_eq!(uniform.viewport[0], 800.0);
        assert_eq!(uniform.viewport[1], 600.0);
        assert_eq!(uniform.zoom, 1.0);
        assert_eq!(uniform.grid_size, 10.0);
    }

    // =========================================================================
    // GpuSchematicBridge Tests
    // =========================================================================

    #[test]
    fn test_bridge_initial_sync_needed() {
        let bridge = GpuSchematicBridge::new();
        let mut sch = SchematicState::default();
        // Bump version so schematic has non-zero version
        sch.bump_topology_version();
        
        assert!(bridge.needs_sync(&sch), "Should need sync initially");
    }

    #[test]
    fn test_bridge_empty_data_initially() {
        let bridge = GpuSchematicBridge::new();
        
        assert!(bridge.wire_data().is_empty(), "No wires initially");
        assert!(bridge.component_data().is_empty(), "No components initially");
        assert!(bridge.junction_data().is_empty(), "No junctions initially");
    }

    #[test]
    fn test_bridge_sync_marks_not_needed() {
        let mut bridge = GpuSchematicBridge::new();
        let sch = SchematicState::default();
        let ctx = RenderContext::new();
        
        bridge.sync(&sch, &ctx);
        
        assert!(!bridge.needs_sync(&sch), "Should not need sync after syncing");
    }

    #[test]
    fn test_bridge_sync_with_components() {
        let mut bridge = GpuSchematicBridge::new();
        let mut sch = SchematicState::default();
        let ctx = RenderContext::new();
        
        // Add a component
        sch.components.push(Component::new(1, ComponentType::Resistor, Point::new(100, 200)));
        sch.bump_topology_version();
        
        bridge.sync(&sch, &ctx);
        
        assert_eq!(bridge.component_data().len(), 1, "Should have one component");
    }

    #[test]
    fn test_bridge_sync_with_wires() {
        let mut bridge = GpuSchematicBridge::new();
        let mut sch = SchematicState::default();
        let ctx = RenderContext::new();
        
        // Add a wire
        sch.wires.push(Wire::new(1, vec![Point::new(0, 0), Point::new(100, 0)]));
        sch.bump_topology_version();
        
        bridge.sync(&sch, &ctx);
        
        assert_eq!(bridge.wire_data().len(), 1, "Should have one wire");
    }

    #[test]
    fn test_bridge_dirty_flags() {
        let mut bridge = GpuSchematicBridge::new();
        
        bridge.mark_camera_dirty();
        assert!(bridge.dirty_flags().camera, "Camera should be dirty");
        
        bridge.mark_grid_dirty();
        assert!(bridge.dirty_flags().grid, "Grid should be dirty");
        
        bridge.clear_dirty();
        assert!(!bridge.dirty_flags().any(), "No flags should be dirty after clear");
    }

    #[test]
    fn test_bridge_hit_test_empty_schematic() {
        let bridge = GpuSchematicBridge::new();
        let sch = SchematicState::default();
        
        let hit = bridge.hit_test(&sch, 0.0, 0.0);
        assert!(!hit.is_hit(), "No hit on empty schematic");
    }

    #[test]
    fn test_bridge_hit_test_component() {
        let bridge = GpuSchematicBridge::new();
        let mut sch = SchematicState::default();
        
        sch.components.push(Component::new(1, ComponentType::Resistor, Point::new(100, 100)));
        
        let hit = bridge.hit_test(&sch, 100.0, 100.0);
        assert!(hit.is_hit(), "Should hit component at its position");
    }

    #[test]
    fn test_bridge_rect_select() {
        let bridge = GpuSchematicBridge::new();
        let mut sch = SchematicState::default();
        
        sch.components.push(Component::new(1, ComponentType::Resistor, Point::new(50, 50)));
        sch.components.push(Component::new(2, ComponentType::Capacitor, Point::new(200, 200)));
        
        // Select region containing only first component
        let (comps, wires) = bridge.rect_select(&sch, 0.0, 0.0, 100.0, 100.0);
        
        assert_eq!(comps.len(), 1, "Should select one component");
        assert!(comps.contains(&1), "Should select component ID 1");
        assert!(wires.is_empty(), "No wires in selection");
    }

    #[test]
    fn test_bridge_world_to_grid() {
        let bridge = GpuSchematicBridge::new();
        
        // Test snapping to grid
        let point = bridge.world_to_grid(12.3, 17.9, 10);
        assert_eq!(point.x, 10, "X should snap to 10");
        assert_eq!(point.y, 20, "Y should snap to 20");
    }

    #[test]
    fn test_bridge_world_to_grid_negative() {
        let bridge = GpuSchematicBridge::new();
        
        let point = bridge.world_to_grid(-12.3, -17.9, 10);
        assert_eq!(point.x, -10, "X should snap to -10");
        assert_eq!(point.y, -20, "Y should snap to -20");
    }

    // =========================================================================
    // Coordinate Conversion Tests
    // =========================================================================

    #[test]
    fn test_screen_to_world_at_origin() {
        use crate::gpu::integration::screen_to_world;
        
        let (wx, wy) = screen_to_world(0.0, 0.0, 0.0, 0.0, 1.0);
        assert_eq!(wx, 0.0);
        assert_eq!(wy, 0.0);
    }

    #[test]
    fn test_screen_to_world_with_pan() {
        use crate::gpu::integration::screen_to_world;
        
        let (wx, wy) = screen_to_world(100.0, 100.0, 50.0, 50.0, 1.0);
        // screen - pan = world at zoom 1
        assert_eq!(wx, 50.0);
        assert_eq!(wy, 50.0);
    }

    #[test]
    fn test_screen_to_world_with_zoom() {
        use crate::gpu::integration::screen_to_world;
        
        let (wx, wy) = screen_to_world(100.0, 100.0, 0.0, 0.0, 2.0);
        // screen / zoom = world
        assert_eq!(wx, 50.0);
        assert_eq!(wy, 50.0);
    }

    #[test]
    fn test_world_to_screen_at_origin() {
        use crate::gpu::integration::world_to_screen;
        
        let (sx, sy) = world_to_screen(0.0, 0.0, 0.0, 0.0, 1.0);
        assert_eq!(sx, 0.0);
        assert_eq!(sy, 0.0);
    }

    #[test]
    fn test_world_to_screen_with_pan() {
        use crate::gpu::integration::world_to_screen;
        
        let (sx, sy) = world_to_screen(50.0, 50.0, 50.0, 50.0, 1.0);
        // world * zoom + pan = screen
        assert_eq!(sx, 100.0);
        assert_eq!(sy, 100.0);
    }

    #[test]
    fn test_world_to_screen_with_zoom() {
        use crate::gpu::integration::world_to_screen;
        
        let (sx, sy) = world_to_screen(50.0, 50.0, 0.0, 0.0, 2.0);
        // world * zoom = screen
        assert_eq!(sx, 100.0);
        assert_eq!(sy, 100.0);
    }

    #[test]
    fn test_screen_world_round_trip() {
        use crate::gpu::integration::{screen_to_world, world_to_screen};
        
        let original_x = 123.456f64;
        let original_y = 789.012f64;
        let pan_x = 100.0f64;
        let pan_y = 200.0f64;
        let zoom = 1.5f64;
        
        let (wx, wy) = screen_to_world(original_x, original_y, pan_x, pan_y, zoom);
        let (sx, sy) = world_to_screen(wx, wy, pan_x, pan_y, zoom);
        
        assert!((sx - original_x).abs() < 0.001, "X round-trip failed");
        assert!((sy - original_y).abs() < 0.001, "Y round-trip failed");
    }

    #[test]
    fn test_snap_to_grid() {
        use crate::gpu::integration::snap_to_grid;
        
        assert_eq!(snap_to_grid(12.3, 10), 10.0);
        assert_eq!(snap_to_grid(17.9, 10), 20.0);
        assert_eq!(snap_to_grid(15.0, 10), 20.0); // Rounds up at midpoint
        assert_eq!(snap_to_grid(14.9, 10), 10.0);
    }

    #[test]
    fn test_snap_to_grid_negative() {
        use crate::gpu::integration::snap_to_grid;
        
        assert_eq!(snap_to_grid(-12.3, 10), -10.0);
        assert_eq!(snap_to_grid(-17.9, 10), -20.0);
    }

    #[test]
    fn test_snap_to_grid_various_sizes() {
        use crate::gpu::integration::snap_to_grid;
        
        // Grid size 5
        assert_eq!(snap_to_grid(12.3, 5), 10.0);
        assert_eq!(snap_to_grid(12.6, 5), 15.0);
        
        // Grid size 20
        assert_eq!(snap_to_grid(35.0, 20), 40.0);
        assert_eq!(snap_to_grid(29.9, 20), 20.0);
    }

    // =========================================================================
    // Edge Case Tests
    // =========================================================================

    #[test]
    fn test_camera_zero_viewport() {
        let camera = Camera::new(0.0, 0.0, 10.0);
        // Should not panic, should have safe defaults
        let _uniform = camera.build_uniform();
    }

    #[test]
    fn test_camera_very_large_viewport() {
        let camera = Camera::new(10000.0, 10000.0, 10.0);
        let bounds = camera.world_bounds();
        
        // Should have reasonable bounds
        assert!(bounds.max_x > bounds.min_x);
        assert!(bounds.max_y > bounds.min_y);
    }

    #[test]
    fn test_camera_extreme_zoom_in() {
        let mut camera = Camera::new(800.0, 600.0, 10.0);
        camera.set_zoom(1000.0);
        
        let bounds = camera.world_bounds();
        // At high zoom, visible world should be small
        let width = bounds.max_x - bounds.min_x;
        assert!(width < 10.0, "High zoom should show small area");
    }

    #[test]
    fn test_camera_extreme_zoom_out() {
        let mut camera = Camera::new(800.0, 600.0, 10.0);
        camera.set_zoom(0.1); // Use min_zoom value
        
        let bounds = camera.world_bounds();
        // At low zoom (0.1), visible world should be larger than viewport
        let width = bounds.max_x - bounds.min_x;
        // At zoom 0.1, width should be ~10x viewport = ~8000
        assert!(width > 500.0, "Low zoom ({}) should show large area, got width {}", camera.zoom, width);
    }

    #[test]
    fn test_bridge_large_schematic() {
        let mut bridge = GpuSchematicBridge::new();
        let mut sch = SchematicState::default();
        let ctx = RenderContext::new();
        
        // Add many components
        for i in 0..1000 {
            sch.components.push(Component::new(
                i as u64,
                ComponentType::Resistor,
                Point::new((i * 10) as i32, (i * 10) as i32),
            ));
        }
        sch.bump_topology_version();
        
        bridge.sync(&sch, &ctx);
        
        assert_eq!(bridge.component_data().len(), 1000);
    }

    #[test]
    fn test_bridge_complex_wire_path() {
        let mut bridge = GpuSchematicBridge::new();
        let mut sch = SchematicState::default();
        let ctx = RenderContext::new();
        
        // Wire with many segments
        let points: Vec<Point> = (0..50)
            .map(|i| Point::new(i * 10, if i % 2 == 0 { 0 } else { 10 }))
            .collect();
        
        sch.wires.push(Wire::new(1, points));
        sch.bump_topology_version();
        
        bridge.sync(&sch, &ctx);
        
        assert_eq!(bridge.wire_data().len(), 1);
    }

    // =========================================================================
    // Render State Tests
    // =========================================================================

    #[test]
    fn test_schematic_state_topology_version() {
        let mut sch = SchematicState::default();
        let initial = sch.topology_version();
        
        sch.bump_topology_version();
        assert!(sch.topology_version() > initial, "Version should increase");
        
        sch.bump_topology_version();
        sch.bump_topology_version();
        assert_eq!(sch.topology_version(), initial + 3, "Version should be +3");
    }

    #[test]
    fn test_component_position_integrity() {
        let comp = Component::new(42, ComponentType::Capacitor, Point::new(-500, 300));
        
        assert_eq!(comp.id, 42);
        assert_eq!(comp.pos.x, -500);
        assert_eq!(comp.pos.y, 300);
    }

    #[test]
    fn test_wire_points_integrity() {
        let points = vec![
            Point::new(0, 0),
            Point::new(100, 0),
            Point::new(100, 100),
        ];
        let wire = Wire::new(99, points.clone());
        
        assert_eq!(wire.id, 99);
        assert_eq!(wire.points.len(), 3);
        assert_eq!(wire.points[1], Point::new(100, 0));
    }

    // =========================================================================
    // Label Tests
    // =========================================================================

    #[test]
    fn test_label_data_construction() {
        use crate::gpu::text::LabelData;

        let label = LabelData::new("R1", 100.0, 200.0);
        assert_eq!(label.text, "R1");
        assert_eq!(label.x, 100.0);
        assert_eq!(label.y, 200.0);
    }

    #[test]
    fn test_label_data_with_color() {
        use crate::gpu::text::LabelData;

        let label = LabelData::new("C1", 50.0, 50.0)
            .with_color([1.0, 0.0, 0.0, 1.0]);
        assert_eq!(label.color, [1.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_label_data_with_scale() {
        use crate::gpu::text::LabelData;

        let label = LabelData::new("VCC", 0.0, 0.0)
            .with_scale(2.0);
        assert_eq!(label.scale, 2.0);
    }

    #[test]
    fn test_label_data_with_align() {
        use crate::gpu::text::{LabelData, TextAlign};

        let label = LabelData::new("GND", 0.0, 0.0)
            .with_align(TextAlign::Center);
        assert_eq!(label.align, TextAlign::Center);
    }

    #[test]
    fn test_label_to_instances() {
        use crate::gpu::text::{GlyphAtlas, LabelData};

        let atlas = GlyphAtlas::new();
        let label = LabelData::new("AB", 10.0, 20.0);
        let instances = label.to_instances(&atlas);

        // Should have 2 instances for 2 characters
        assert_eq!(instances.len(), 2);
    }

    #[test]
    fn test_label_to_instances_empty_text() {
        use crate::gpu::text::{GlyphAtlas, LabelData};

        let atlas = GlyphAtlas::new();
        let label = LabelData::new("", 0.0, 0.0);
        let instances = label.to_instances(&atlas);

        assert!(instances.is_empty(), "Empty text should produce no instances");
    }

    #[test]
    fn test_glyph_atlas_has_ascii() {
        use crate::gpu::text::GlyphAtlas;

        let atlas = GlyphAtlas::new();

        // Check common characters exist
        assert!(atlas.contains('A'), "Should contain A");
        assert!(atlas.contains('0'), "Should contain 0");
        assert!(atlas.contains(' '), "Should contain space");
        assert!(atlas.contains('.'), "Should contain period");
    }

    #[test]
    fn test_glyph_atlas_get_or_default() {
        use crate::gpu::text::GlyphAtlas;

        let atlas = GlyphAtlas::new();

        // Known character
        let glyph = atlas.get_or_default('R');
        assert!(glyph.advance > 0.0, "Glyph should have advance");

        // Unknown character should return space fallback
        let fallback = atlas.get_or_default('€');
        assert_eq!(fallback.advance, atlas.get_or_default(' ').advance);
    }

    #[test]
    fn test_text_instance_construction() {
        use crate::gpu::text::{GlyphAtlas, TextInstance};

        let atlas = GlyphAtlas::new();
        let glyph = atlas.get_or_default('M');
        let inst = TextInstance::new(100.0, 50.0, &glyph, [1.0, 1.0, 1.0, 1.0], 1.0);

        assert_eq!(inst.position[0], 100.0);
        assert_eq!(inst.position[1], 50.0);
        assert_eq!(inst.color, [1.0, 1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_text_layout() {
        use crate::gpu::text::{layout_text, GlyphAtlas, TextLayout};

        let atlas = GlyphAtlas::new();
        let layout = TextLayout::default();
        let instances = layout_text("Test", &layout, &atlas);

        // Should have 4 instances for 4 characters
        assert_eq!(instances.len(), 4);
    }

    #[test]
    fn test_text_width() {
        use crate::gpu::text::text_width;

        let width = text_width("Hello", 1.0);
        assert!(width > 0.0, "Text should have positive width");

        let double_width = text_width("Hello", 2.0);
        assert!((double_width - width * 2.0).abs() < 0.1, "Scaling should double width");
    }

    #[test]
    fn test_is_text_visible() {
        use crate::gpu::text::is_text_visible;

        // At high zoom, small text should be visible
        assert!(is_text_visible(1.0, 5.0), "Text visible at high zoom");

        // At low zoom, small text may be culled
        // (depends on implementation's MIN_VISIBLE_SCALE)
    }

    #[test]
    fn test_bridge_generate_component_labels() {
        let bridge = GpuSchematicBridge::new();
        let mut sch = SchematicState::default();

        // Components with names get labels
        let mut comp1 = Component::new(1, ComponentType::Resistor, Point::new(100, 100));
        comp1.name = "R1".to_string();
        sch.components.push(comp1);

        let mut comp2 = Component::new(2, ComponentType::Capacitor, Point::new(200, 200));
        comp2.name = "C1".to_string();
        sch.components.push(comp2);

        // Component without name - should not get label
        let comp3 = Component::new(3, ComponentType::Ground, Point::new(300, 300));
        sch.components.push(comp3);

        let labels = bridge.generate_component_labels(&sch);

        // Should have labels for named components only
        assert_eq!(labels.len(), 2, "Should generate label for each named component");
        assert!(labels.iter().any(|l| l.text == "R1"), "Should have R1 label");
        assert!(labels.iter().any(|l| l.text == "C1"), "Should have C1 label");
    }

    #[test]
    fn test_bridge_generate_net_labels_empty() {
        let bridge = GpuSchematicBridge::new();
        let sch = SchematicState::default();

        let labels = bridge.generate_net_labels(&sch);

        assert!(labels.is_empty(), "No nets should mean no net labels");
    }

    #[test]
    fn test_bridge_glyph_atlas() {
        let bridge = GpuSchematicBridge::new();
        let atlas = bridge.glyph_atlas();

        assert!(atlas.glyph_count() > 0, "Atlas should have glyphs");
        assert!(atlas.contains('R'), "Should contain R for resistor labels");
    }
}
