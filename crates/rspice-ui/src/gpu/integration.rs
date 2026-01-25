//! GPU Integration Layer
//!
//! Connects the GPU rendering modules with the schematic editor.
//! Provides a unified interface for schematic → GPU data conversion,
//! hit testing, and render cache synchronization.

use crate::gpu::gpu_cache::{CacheStats, DirtyFlags, GpuRenderCache};
use crate::gpu::hit_test::{BoundingBox, HitResult, HitTestConfig, HitTester};
use crate::gpu::renderer::{ComponentData, JunctionData, WireData};
use crate::gpu::text::{GlyphAtlas, LabelData, TextAlign};
use crate::state::render_context::RenderContext;
use crate::state::{Point, SchematicState};

// =============================================================================
// GPU Schematic Bridge
// =============================================================================

/// Bridge between schematic state and GPU renderer
///
/// This is the main integration point that coordinates:
/// - Converting SchematicState to GPU-compatible data
/// - Maintaining render cache with dirty tracking
/// - Providing hit testing for selection
/// - Managing text/label rendering
pub struct GpuSchematicBridge {
    /// Render cache with dirty tracking
    pub cache: GpuRenderCache,

    /// Hit tester for picking
    hit_tester: HitTester,

    /// Glyph atlas for text rendering
    glyph_atlas: GlyphAtlas,

    /// Last synced topology version
    last_sync_version: u64,
}

impl Default for GpuSchematicBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl GpuSchematicBridge {
    /// Create a new GPU bridge
    pub fn new() -> Self {
        Self {
            cache: GpuRenderCache::new(),
            hit_tester: HitTester::default(),
            glyph_atlas: GlyphAtlas::new(),
            last_sync_version: 0,
        }
    }

    /// Create with custom hit test config
    pub fn with_hit_config(config: HitTestConfig) -> Self {
        Self {
            cache: GpuRenderCache::new(),
            hit_tester: HitTester::new(config),
            glyph_atlas: GlyphAtlas::new(),
            last_sync_version: 0,
        }
    }

    /// Synchronize GPU cache with schematic state
    ///
    /// Returns true if cache was updated (needs re-render)
    pub fn sync(&mut self, schematic: &SchematicState, render_ctx: &RenderContext) -> bool {
        let was_dirty = self.cache.dirty.any();
        let old_version = self.cache.topology_version();
        
        self.cache.synchronize(schematic, render_ctx);
        
        let updated = was_dirty || self.cache.topology_version() != old_version || self.cache.dirty.any();
        if updated {
            self.last_sync_version = schematic.topology_version();
        }
        updated
    }

    /// Check if sync is needed
    pub fn needs_sync(&self, schematic: &SchematicState) -> bool {
        schematic.topology_version() != self.last_sync_version
    }

    /// Get cached wire data for GPU upload
    pub fn wire_data(&self) -> &[WireData] {
        &self.cache.wires
    }

    /// Get cached component data for GPU upload
    pub fn component_data(&self) -> &[ComponentData] {
        &self.cache.components
    }

    /// Get cached junction data for GPU upload
    pub fn junction_data(&self) -> &[JunctionData] {
        &self.cache.junctions
    }

    /// Get dirty flags indicating what needs updating
    pub fn dirty_flags(&self) -> DirtyFlags {
        self.cache.dirty
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        self.cache.stats
    }

    /// Perform hit test at world coordinates
    pub fn hit_test(&self, schematic: &SchematicState, x: f32, y: f32) -> HitResult {
        self.hit_tester.test_all(&schematic.components, &schematic.wires, x, y)
    }

    /// Perform rectangle selection
    pub fn rect_select(
        &self,
        schematic: &SchematicState,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
    ) -> (Vec<u64>, Vec<u64>) {
        let rect = BoundingBox::new(
            x1.min(x2), y1.min(y2),
            x1.max(x2), y1.max(y2),
        );
        self.hit_tester.test_rect(&schematic.components, &schematic.wires, &rect)
    }

    /// Convert world coordinates to grid point
    pub fn world_to_grid(&self, x: f32, y: f32, grid_size: i32) -> Point {
        Point::new(
            ((x / grid_size as f32).round() as i32) * grid_size,
            ((y / grid_size as f32).round() as i32) * grid_size,
        )
    }

    /// Generate labels for all visible components
    pub fn generate_component_labels(&self, schematic: &SchematicState) -> Vec<LabelData> {
        let mut labels = Vec::new();

        for comp in &schematic.components {
            // Component name label
            if !comp.name.is_empty() {
                labels.push(
                    LabelData::new(&comp.name, comp.pos.x as f32, comp.pos.y as f32 + 2.0)
                        .with_scale(0.1)
                        .with_color([1.0, 1.0, 1.0, 1.0])
                );
            }
        }

        labels
    }

    /// Generate net labels
    pub fn generate_net_labels(&self, schematic: &SchematicState) -> Vec<LabelData> {
        // Wire color: green
        let wire_color = [0.0, 0.8, 0.0, 1.0];
        
        schematic.net_labels.iter().map(|nl| {
            LabelData::new(&nl.name, nl.pos.x as f32, nl.pos.y as f32)
                .with_scale(0.1)
                .with_color(wire_color)
                .with_align(TextAlign::Left)
        }).collect()
    }

    /// Get the glyph atlas for text rendering
    pub fn glyph_atlas(&self) -> &GlyphAtlas {
        &self.glyph_atlas
    }

    /// Mark grid as dirty (e.g., when grid size changes)
    pub fn mark_grid_dirty(&mut self) {
        self.cache.dirty.grid = true;
    }

    /// Mark camera as dirty (e.g., when pan/zoom changes)
    pub fn mark_camera_dirty(&mut self) {
        self.cache.dirty.camera = true;
    }

    /// Clear dirty flags after GPU upload
    pub fn clear_dirty(&mut self) {
        self.cache.dirty.clear();
    }
}

// =============================================================================
// Coordinate Conversion Helpers
// =============================================================================

/// Convert screen coordinates to world coordinates
pub fn screen_to_world(
    screen_x: f64,
    screen_y: f64,
    pan_x: f64,
    pan_y: f64,
    zoom: f64,
) -> (f32, f32) {
    let world_x = ((screen_x - pan_x) / zoom) as f32;
    let world_y = ((screen_y - pan_y) / zoom) as f32;
    (world_x, world_y)
}

/// Convert world coordinates to screen coordinates
pub fn world_to_screen(
    world_x: f32,
    world_y: f32,
    pan_x: f64,
    pan_y: f64,
    zoom: f64,
) -> (f64, f64) {
    let screen_x = world_x as f64 * zoom + pan_x;
    let screen_y = world_y as f64 * zoom + pan_y;
    (screen_x, screen_y)
}

/// Snap world coordinate to grid
pub fn snap_to_grid(world: f32, grid_size: i32) -> f32 {
    (world / grid_size as f32).round() * grid_size as f32
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Component, ComponentType, Wire};

    fn make_test_schematic() -> SchematicState {
        let mut sch = SchematicState::default();
        sch.components.push(Component::new(1, ComponentType::Resistor, Point::new(10, 10)));
        sch.wires.push(Wire::new(1, vec![Point::new(0, 0), Point::new(20, 0)]));
        sch.bump_topology_version();
        sch
    }

    #[test]
    fn test_gpu_bridge_new() {
        let bridge = GpuSchematicBridge::new();
        assert_eq!(bridge.last_sync_version, 0);
    }

    #[test]
    fn test_gpu_bridge_sync() {
        let mut bridge = GpuSchematicBridge::new();
        let sch = make_test_schematic();
        let render_ctx = RenderContext::new();

        let updated = bridge.sync(&sch, &render_ctx);
        assert!(updated);
        assert!(!bridge.needs_sync(&sch));
    }

    #[test]
    fn test_gpu_bridge_hit_test() {
        let bridge = GpuSchematicBridge::new();
        let sch = make_test_schematic();

        let hit = bridge.hit_test(&sch, 10.0, 10.0);
        assert!(hit.is_hit());
    }

    #[test]
    fn test_gpu_bridge_rect_select() {
        let bridge = GpuSchematicBridge::new();
        let sch = make_test_schematic();

        let (comps, wires) = bridge.rect_select(&sch, -5.0, -5.0, 25.0, 15.0);
        assert!(!comps.is_empty() || !wires.is_empty());
    }

    #[test]
    fn test_screen_to_world() {
        let (wx, wy) = screen_to_world(100.0, 100.0, 50.0, 50.0, 2.0);
        assert_eq!(wx, 25.0);
        assert_eq!(wy, 25.0);
    }

    #[test]
    fn test_world_to_screen() {
        let (sx, sy) = world_to_screen(25.0, 25.0, 50.0, 50.0, 2.0);
        assert_eq!(sx, 100.0);
        assert_eq!(sy, 100.0);
    }

    #[test]
    fn test_snap_to_grid() {
        assert_eq!(snap_to_grid(12.3, 10), 10.0);
        assert_eq!(snap_to_grid(17.9, 10), 20.0);
    }

    #[test]
    fn test_world_to_grid() {
        let bridge = GpuSchematicBridge::new();
        let point = bridge.world_to_grid(12.3, 17.9, 10);
        assert_eq!(point.x, 10);
        assert_eq!(point.y, 20);
    }

    #[test]
    fn test_generate_component_labels() {
        let bridge = GpuSchematicBridge::new();
        let mut sch = make_test_schematic();
        sch.components[0].name = "R1".to_string();

        let labels = bridge.generate_component_labels(&sch);
        assert!(!labels.is_empty());
        assert_eq!(labels[0].text, "R1");
    }
}
