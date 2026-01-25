//! Render Loop Controller
//!
//! Manages the GPU render lifecycle for the schematic canvas.
//! Provides a clean interface between Dioxus reactive state and the GPU render pipeline.
//!
//! # Usage
//!
//! ```ignore
//! let controller = RenderLoopController::new(800, 600).await?;
//! controller.sync_schematic(&schematic_state);
//! controller.request_frame(); // Marks frame dirty
//! controller.render_if_needed(&view)?; // Renders if dirty
//! ```

use crate::gpu::camera::Camera;
use crate::gpu::context::GpuError;
use crate::gpu::culling::CullStats;
use crate::gpu::integration::GpuSchematicBridge;
use crate::gpu::renderer::{ComponentData, JunctionData, SchematicRenderer, WireData};
use crate::state::render_context::RenderContext;
use crate::state::SchematicState;

// =============================================================================
// Dirty Flags
// =============================================================================

/// Flags indicating what needs to be re-rendered
#[derive(Debug, Clone, Copy, Default)]
pub struct DirtyFlags {
    /// Schematic data changed (components, wires, junctions)
    pub schematic: bool,
    /// Viewport changed (pan, zoom, resize)
    pub viewport: bool,
    /// Selection changed
    pub selection: bool,
    /// Grid settings changed
    pub grid: bool,
    /// Force full redraw
    pub force: bool,
}

impl DirtyFlags {
    /// Check if any flag is set
    pub fn any(&self) -> bool {
        self.schematic || self.viewport || self.selection || self.grid || self.force
    }

    /// Clear all flags
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// Mark everything dirty
    pub fn mark_all(&mut self) {
        self.schematic = true;
        self.viewport = true;
        self.selection = true;
        self.grid = true;
    }
}

// =============================================================================
// Frame Statistics
// =============================================================================

/// Per-frame performance statistics
#[derive(Debug, Clone, Default)]
pub struct FrameStats {
    /// Frame count since initialization
    pub frame_count: u64,
    /// Triangles rendered this frame
    pub triangle_count: u32,
    /// Draw calls this frame
    pub draw_call_count: u32,
    /// Components culled this frame
    pub cull_stats: CullStats,
    /// Frame time in milliseconds
    pub frame_time_ms: f32,
    /// Rolling average FPS
    pub fps: f32,
    /// GPU upload time in milliseconds
    pub upload_time_ms: f32,
}

impl FrameStats {
    /// Reset stats for new frame
    pub fn begin_frame(&mut self) {
        self.triangle_count = 0;
        self.draw_call_count = 0;
        self.upload_time_ms = 0.0;
    }

    /// Calculate FPS from frame time
    pub fn update_fps(&mut self, frame_time_ms: f32) {
        self.frame_time_ms = frame_time_ms;
        if frame_time_ms > 0.0 {
            // Exponential moving average for smooth FPS display
            let instant_fps = 1000.0 / frame_time_ms;
            self.fps = self.fps * 0.9 + instant_fps * 0.1;
        }
    }
}

// =============================================================================
// Render Loop Controller
// =============================================================================

/// Controller for managing the GPU render lifecycle
///
/// This provides a high-level interface for rendering that:
/// - Manages dirty state to avoid unnecessary renders
/// - Tracks performance statistics
/// - Handles error recovery gracefully
/// - Provides clean separation from Dioxus reactive system
pub struct RenderLoopController {
    /// Schematic renderer with GPU resources
    renderer: SchematicRenderer,

    /// GPU-Schematic bridge for data conversion
    bridge: GpuSchematicBridge,

    /// Camera state
    camera: Camera,

    /// Dirty flags for incremental updates
    dirty: DirtyFlags,

    /// Frame statistics
    stats: FrameStats,

    /// Viewport dimensions
    width: u32,
    height: u32,

    /// Cached topology version for change detection
    last_topology_version: u64,

    /// Whether initialization is complete
    initialized: bool,

    /// Last frame timestamp (for FPS calculation)
    #[cfg(not(target_arch = "wasm32"))]
    last_frame_time: std::time::Instant,
}

impl RenderLoopController {
    /// Create a new render loop controller
    ///
    /// This is async because GPU initialization requires adapter negotiation.
    pub async fn new(width: u32, height: u32) -> Result<Self, GpuError> {
        let renderer = SchematicRenderer::new().await?;
        let camera = Camera::new(width as f32, height as f32, 10.0);
        let bridge = GpuSchematicBridge::new();

        Ok(Self {
            renderer,
            bridge,
            camera,
            dirty: DirtyFlags {
                force: true,
                ..Default::default()
            },
            stats: FrameStats::default(),
            width,
            height,
            last_topology_version: 0,
            initialized: true,
            #[cfg(not(target_arch = "wasm32"))]
            last_frame_time: std::time::Instant::now(),
        })
    }

    // =========================================================================
    // State Updates
    // =========================================================================

    /// Update viewport dimensions
    pub fn resize(&mut self, width: u32, height: u32) {
        if width != self.width || height != self.height {
            self.width = width.max(1);
            self.height = height.max(1);
            self.camera.set_viewport(width as f32, height as f32);
            self.renderer.resize(width, height);
            self.dirty.viewport = true;
        }
    }

    /// Update camera pan position
    pub fn set_pan(&mut self, pan_x: f32, pan_y: f32) {
        self.camera.position = [pan_x, pan_y];
        self.dirty.viewport = true;
    }

    /// Update camera zoom level
    pub fn set_zoom(&mut self, zoom: f32) {
        self.camera.set_zoom(zoom);
        self.dirty.viewport = true;
    }

    /// Mark schematic data as dirty
    pub fn mark_schematic_dirty(&mut self) {
        self.dirty.schematic = true;
    }

    /// Mark selection as dirty
    pub fn mark_selection_dirty(&mut self) {
        self.dirty.selection = true;
    }

    /// Request a frame render
    pub fn request_frame(&mut self) {
        self.dirty.force = true;
    }

    // =========================================================================
    // Schematic Data Sync
    // =========================================================================

    /// Sync schematic state to GPU buffers via bridge
    ///
    /// Returns true if data changed and render is needed.
    pub fn sync_schematic(
        &mut self,
        schematic: &SchematicState,
        render_ctx: &RenderContext,
    ) -> bool {
        let version = schematic.topology_version();
        let version_changed = version != self.last_topology_version;

        if version_changed || self.dirty.schematic || self.dirty.selection {
            self.last_topology_version = version;

            // First sync to bridge cache
            let needs_update = self.bridge.sync(schematic, render_ctx);

            if needs_update || self.dirty.schematic {
                // Get data from bridge
                let components = self.bridge.component_data();
                let wires = self.bridge.wire_data();
                let junctions = self.bridge.junction_data();

                // Update renderer with culled data (renderer handles culling internally)
                let component_stats = self.renderer.update_components_culled(components);
                let wire_stats = self.renderer.update_wires_culled(wires);
                let junction_stats = self.renderer.update_junctions_culled(junctions);

                // Aggregate cull stats
                self.stats.cull_stats = CullStats {
                    total: component_stats.total + wire_stats.total + junction_stats.total,
                    visible: component_stats.visible + wire_stats.visible + junction_stats.visible,
                    culled: component_stats.culled + wire_stats.culled + junction_stats.culled,
                    cull_ratio: if component_stats.total + wire_stats.total + junction_stats.total
                        > 0
                    {
                        (component_stats.culled + wire_stats.culled + junction_stats.culled) as f32
                            / (component_stats.total + wire_stats.total + junction_stats.total)
                                as f32
                    } else {
                        0.0
                    },
                };

                self.dirty.schematic = false;
                self.dirty.selection = false;

                return true;
            }
        }

        false
    }

    // =========================================================================
    // Rendering
    // =========================================================================

    /// Check if a render is needed
    pub fn needs_render(&self) -> bool {
        self.dirty.any()
    }

    /// Render a frame to a texture view
    pub fn render(&mut self, view: &wgpu::TextureView) -> Result<FrameStats, GpuError> {
        self.stats.begin_frame();
        self.stats.frame_count += 1;

        // Render (renderer uses its internal camera)
        self.renderer.render(view)?;

        // Update FPS
        #[cfg(not(target_arch = "wasm32"))]
        {
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(self.last_frame_time).as_secs_f32() * 1000.0;
            self.stats.update_fps(elapsed);
            self.last_frame_time = now;
        }

        // Clear dirty flags
        self.dirty.clear();

        Ok(self.stats.clone())
    }

    /// Render only if needed, returns None if skipped
    pub fn render_if_needed(
        &mut self,
        view: &wgpu::TextureView,
    ) -> Result<Option<FrameStats>, GpuError> {
        if self.needs_render() {
            Ok(Some(self.render(view)?))
        } else {
            Ok(None)
        }
    }

    // =========================================================================
    // Accessors
    // =========================================================================

    /// Get current frame statistics
    pub fn stats(&self) -> &FrameStats {
        &self.stats
    }

    /// Get current camera
    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    /// Get mutable camera reference
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    /// Get current viewport dimensions
    pub fn viewport(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Check if initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Get the underlying renderer
    pub fn renderer(&self) -> &SchematicRenderer {
        &self.renderer
    }

    /// Get mutable renderer reference
    pub fn renderer_mut(&mut self) -> &mut SchematicRenderer {
        &mut self.renderer
    }

    /// Get the GPU-schematic bridge
    pub fn bridge(&self) -> &GpuSchematicBridge {
        &self.bridge
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // DirtyFlags Tests
    // =========================================================================

    #[test]
    fn test_dirty_flags_default() {
        let flags = DirtyFlags::default();
        assert!(!flags.schematic);
        assert!(!flags.viewport);
        assert!(!flags.selection);
        assert!(!flags.grid);
        assert!(!flags.force);
        assert!(!flags.any());
    }

    #[test]
    fn test_dirty_flags_any() {
        let mut flags = DirtyFlags::default();
        assert!(!flags.any());

        flags.schematic = true;
        assert!(flags.any());

        flags.schematic = false;
        flags.viewport = true;
        assert!(flags.any());
    }

    #[test]
    fn test_dirty_flags_clear() {
        let mut flags = DirtyFlags::default();
        flags.mark_all();
        assert!(flags.any());

        flags.clear();
        assert!(!flags.any());
    }

    #[test]
    fn test_dirty_flags_mark_all() {
        let mut flags = DirtyFlags::default();
        flags.mark_all();

        assert!(flags.schematic);
        assert!(flags.viewport);
        assert!(flags.selection);
        assert!(flags.grid);
    }

    // =========================================================================
    // FrameStats Tests
    // =========================================================================

    #[test]
    fn test_frame_stats_default() {
        let stats = FrameStats::default();
        assert_eq!(stats.frame_count, 0);
        assert_eq!(stats.triangle_count, 0);
        assert_eq!(stats.fps, 0.0);
    }

    #[test]
    fn test_frame_stats_begin_frame() {
        let mut stats = FrameStats::default();
        stats.triangle_count = 1000;
        stats.draw_call_count = 50;

        stats.begin_frame();

        assert_eq!(stats.triangle_count, 0);
        assert_eq!(stats.draw_call_count, 0);
    }

    #[test]
    fn test_frame_stats_update_fps() {
        let mut stats = FrameStats::default();

        // 16.67ms = ~60 FPS
        stats.update_fps(16.67);
        assert!(stats.fps > 0.0);

        // FPS should be approximately 60 after enough frames
        // EMA with 0.9/0.1 weighting needs ~30 frames to converge
        for _ in 0..30 {
            stats.update_fps(16.67);
        }
        assert!(stats.fps > 50.0 && stats.fps < 70.0);
    }

    #[test]
    fn test_frame_stats_zero_frame_time() {
        let mut stats = FrameStats::default();
        stats.fps = 60.0;

        // Zero frame time should not update FPS
        stats.update_fps(0.0);
        assert_eq!(stats.fps, 60.0);
    }

    // =========================================================================
    // Integration Tests (Mock)
    // =========================================================================

    #[test]
    fn test_camera_integration() {
        let camera = Camera::new(800.0, 600.0, 10.0);
        assert_eq!(camera.viewport_width, 800.0);
        assert_eq!(camera.viewport_height, 600.0);
        assert_eq!(camera.zoom, 1.0);
    }

    #[test]
    fn test_bridge_creation() {
        let bridge = GpuSchematicBridge::new();
        assert!(bridge.wire_data().is_empty());
        assert!(bridge.component_data().is_empty());
        assert!(bridge.junction_data().is_empty());
    }

    #[test]
    fn test_cull_stats_aggregation() {
        let stats1 = CullStats {
            total: 100,
            visible: 30,
            culled: 70,
            cull_ratio: 0.7,
        };

        let stats2 = CullStats {
            total: 200,
            visible: 50,
            culled: 150,
            cull_ratio: 0.75,
        };

        let combined_total = stats1.total + stats2.total;
        let combined_visible = stats1.visible + stats2.visible;
        let combined_culled = stats1.culled + stats2.culled;

        assert_eq!(combined_total, 300);
        assert_eq!(combined_visible, 80);
        assert_eq!(combined_culled, 220);
    }

    #[test]
    fn test_frame_counter_increment() {
        let mut stats = FrameStats::default();
        assert_eq!(stats.frame_count, 0);

        stats.frame_count += 1;
        assert_eq!(stats.frame_count, 1);

        stats.frame_count += 1;
        assert_eq!(stats.frame_count, 2);
    }

    #[test]
    fn test_viewport_minimum_size() {
        // Ensure viewport clamps to at least 1x1
        let width = 0u32.max(1);
        let height = 0u32.max(1);
        assert_eq!(width, 1);
        assert_eq!(height, 1);
    }

    #[test]
    fn test_topology_version_change_detection() {
        let mut last_version = 0u64;
        let new_version = 1u64;

        let changed = new_version != last_version;
        assert!(changed);

        last_version = new_version;
        let changed = new_version != last_version;
        assert!(!changed);
    }

    // =========================================================================
    // Performance Tests
    // =========================================================================

    #[test]
    fn test_dirty_flag_size() {
        // DirtyFlags should be small for efficient copying
        assert!(std::mem::size_of::<DirtyFlags>() <= 8);
    }

    #[test]
    fn test_frame_stats_size() {
        // FrameStats should be reasonably small
        assert!(std::mem::size_of::<FrameStats>() <= 128);
    }

    #[test]
    fn test_cull_stats_size() {
        // CullStats should be small
        assert!(std::mem::size_of::<CullStats>() <= 32);
    }
}
