//! Render Pass Module
//!
//! Per-frame render logic for the GPU schematic canvas.
//! Handles layer ordering, dirty flag checking, and buffer coordination.
//!
//! # Architecture
//!
//! Professional EDA tools render in strict layer order:
//! 1. Background/Grid
//! 2. Wires (normal → selected → highlighted)
//! 3. Components (normal → selected)
//! 4. Junctions
//! 5. Labels/Text
//! 6. Selection overlay (box selection, drag preview)
//! 7. UI overlays (cursors, snap indicators)
//!
//! This module provides the render pass coordination without
//! depending on actual GPU resources - enabling full testing.

use crate::gpu::gpu_cache::{DirtyFlags, GpuRenderCache};
use crate::gpu::integration::GpuSchematicBridge;
use crate::gpu::renderer::{ComponentData, JunctionData, WireData};
use crate::state::render_context::RenderContext;
use crate::state::{Point, SchematicState, Tool};

// =============================================================================
// Render Layers
// =============================================================================

/// Enumeration of render layers in draw order
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum RenderLayer {
    /// Background grid (always first)
    Grid = 0,
    /// Wire segments (normal state)
    WiresNormal = 1,
    /// Wire segments (selected state)
    WiresSelected = 2,
    /// Wire segments (highlighted - probe/cross-probe)
    WiresHighlighted = 3,
    /// Component symbols (normal state)
    ComponentsNormal = 4,
    /// Component symbols (selected state)
    ComponentsSelected = 5,
    /// Component symbols (highlighted)
    ComponentsHighlighted = 6,
    /// Junction dots
    Junctions = 7,
    /// Component labels
    Labels = 8,
    /// Net labels
    NetLabels = 9,
    /// Wire preview (during wire drawing)
    WirePreview = 10,
    /// Component preview (during placement)
    ComponentPreview = 11,
    /// Box selection rectangle
    BoxSelection = 12,
    /// Drag preview overlay
    DragPreview = 13,
    /// Cursor/snap indicators
    Cursor = 14,
}

impl RenderLayer {
    /// Get all layers in draw order
    pub fn all_in_order() -> &'static [RenderLayer] {
        &[
            RenderLayer::Grid,
            RenderLayer::WiresNormal,
            RenderLayer::WiresSelected,
            RenderLayer::WiresHighlighted,
            RenderLayer::ComponentsNormal,
            RenderLayer::ComponentsSelected,
            RenderLayer::ComponentsHighlighted,
            RenderLayer::Junctions,
            RenderLayer::Labels,
            RenderLayer::NetLabels,
            RenderLayer::WirePreview,
            RenderLayer::ComponentPreview,
            RenderLayer::BoxSelection,
            RenderLayer::DragPreview,
            RenderLayer::Cursor,
        ]
    }

    /// Check if this layer needs geometry update
    pub fn needs_geometry(&self) -> bool {
        match self {
            RenderLayer::Grid => false,         // Grid is procedural
            RenderLayer::BoxSelection => false, // Computed per-frame
            RenderLayer::Cursor => false,       // Computed per-frame
            RenderLayer::DragPreview => false,  // Computed per-frame
            _ => true,
        }
    }

    /// Check if this layer is visible based on tool and state
    pub fn is_visible(&self, tool: &Tool, wire_drawing: bool, has_selection: bool) -> bool {
        match self {
            RenderLayer::WirePreview => wire_drawing,
            RenderLayer::ComponentPreview => matches!(tool, Tool::Place(_)),
            RenderLayer::BoxSelection => false, // Controlled separately
            RenderLayer::DragPreview => false,  // Controlled separately
            RenderLayer::WiresSelected | RenderLayer::ComponentsSelected => has_selection,
            _ => true,
        }
    }
}

// =============================================================================
// Render State
// =============================================================================

/// Complete render state for a frame
#[derive(Debug, Clone, Default)]
pub struct RenderState {
    /// Current camera/viewport state
    pub viewport: ViewportState,
    /// Grid rendering state
    pub grid: GridState,
    /// Wire rendering state
    pub wires: WireRenderState,
    /// Component rendering state
    pub components: ComponentRenderState,
    /// Junction rendering state
    pub junctions: JunctionRenderState,
    /// Overlay state (selection box, previews)
    pub overlay: OverlayState,
    /// Which layers are dirty (need re-render)
    pub dirty_layers: LayerDirtyFlags,
}

/// Viewport/camera state
#[derive(Debug, Clone)]
pub struct ViewportState {
    /// Pan offset in screen pixels
    pub pan: (f64, f64),
    /// Zoom level (1.0 = 100%)
    pub zoom: f64,
    /// Canvas width in pixels
    pub width: f32,
    /// Canvas height in pixels
    pub height: f32,
    /// Grid size in schematic units
    pub grid_size: i32,
}

impl Default for ViewportState {
    fn default() -> Self {
        Self {
            pan: (0.0, 0.0),
            zoom: 1.0,
            width: 800.0,
            height: 600.0,
            grid_size: 10,
        }
    }
}

impl ViewportState {
    /// Get the visible world bounds
    pub fn visible_bounds(&self) -> (f32, f32, f32, f32) {
        let min_x = (-self.pan.0 / self.zoom) as f32;
        let min_y = (-self.pan.1 / self.zoom) as f32;
        let max_x = ((self.width as f64 - self.pan.0) / self.zoom) as f32;
        let max_y = ((self.height as f64 - self.pan.1) / self.zoom) as f32;
        (min_x, min_y, max_x, max_y)
    }

    /// Check if a point is visible
    pub fn is_visible(&self, x: f32, y: f32) -> bool {
        let (min_x, min_y, max_x, max_y) = self.visible_bounds();
        x >= min_x && x <= max_x && y >= min_y && y <= max_y
    }
}

/// Grid rendering state
#[derive(Debug, Clone, Default)]
pub struct GridState {
    /// Whether to show grid
    pub visible: bool,
    /// Minor grid spacing (in schematic units)
    pub minor_spacing: f32,
    /// Major grid spacing (every N minor lines)
    pub major_interval: u32,
    /// Minor grid color [r, g, b, a]
    pub minor_color: [f32; 4],
    /// Major grid color
    pub major_color: [f32; 4],
}

impl GridState {
    /// Create default grid state
    /// Uses fixed 20px minor / 100px major grid to match SVG renderer
    pub fn new(_grid_size: i32) -> Self {
        Self {
            visible: true,
            // SVG uses hardcoded 20px minor grid, not schematic.grid_size
            minor_spacing: 20.0,
            // Major grid every 5 minor lines = 100px (matching SVG)
            major_interval: 5,
            // Colors matching SVG's rgba(128, 128, 128, 0.08) for minor
            minor_color: [0.5, 0.5, 0.5, 0.08],
            // Colors matching SVG's rgba(128, 128, 128, 0.2) for major
            major_color: [0.5, 0.5, 0.5, 0.2],
        }
    }
}

/// Wire rendering state
#[derive(Debug, Clone, Default)]
pub struct WireRenderState {
    /// Normal wire count
    pub normal_count: usize,
    /// Selected wire count
    pub selected_count: usize,
    /// Highlighted wire count
    pub highlighted_count: usize,
}

/// Component rendering state
#[derive(Debug, Clone, Default)]
pub struct ComponentRenderState {
    /// Normal component count
    pub normal_count: usize,
    /// Selected component count
    pub selected_count: usize,
    /// Highlighted component count
    pub highlighted_count: usize,
}

/// Junction rendering state
#[derive(Debug, Clone, Default)]
pub struct JunctionRenderState {
    /// Total junction count
    pub count: usize,
}

/// Overlay state for selection box, previews, etc.
#[derive(Debug, Clone, Default)]
pub struct OverlayState {
    /// Box selection active
    pub box_selection_active: bool,
    /// Box selection start (screen coords)
    pub box_start: (f64, f64),
    /// Box selection end (screen coords)
    pub box_end: (f64, f64),
    /// Wire preview points
    pub wire_preview: Vec<Point>,
    /// Component preview position
    pub component_preview: Option<(Point, i32)>, // (position, symbol_type)
    /// Drag offset (for visual feedback)
    pub drag_offset: (i32, i32),
    /// Whether drag is active
    pub drag_active: bool,
}

/// Per-layer dirty flags
#[derive(Debug, Clone, Copy, Default)]
pub struct LayerDirtyFlags {
    pub grid: bool,
    pub wires: bool,
    pub components: bool,
    pub junctions: bool,
    pub labels: bool,
    pub overlays: bool,
}

impl LayerDirtyFlags {
    /// Create with all flags set
    pub fn all() -> Self {
        Self {
            grid: true,
            wires: true,
            components: true,
            junctions: true,
            labels: true,
            overlays: true,
        }
    }

    /// Check if any layer is dirty
    pub fn any(&self) -> bool {
        self.grid || self.wires || self.components || self.junctions || self.labels || self.overlays
    }

    /// Clear all flags
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// Create from GPU cache dirty flags
    pub fn from_cache_flags(flags: &DirtyFlags) -> Self {
        Self {
            grid: flags.grid,
            wires: flags.wires,
            components: flags.components,
            junctions: flags.junctions,
            labels: false,   // Labels have separate tracking
            overlays: false, // Overlays always update
        }
    }
}

// =============================================================================
// Render Pass
// =============================================================================

/// Render pass coordinator
///
/// Orchestrates the per-frame update of render state from schematic state.
/// This is CPU-side preparation - actual GPU uploads happen in the renderer.
pub struct RenderPass {
    /// Current render state
    pub state: RenderState,
    /// Last processed topology version
    last_topology_version: u64,
}

impl Default for RenderPass {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderPass {
    /// Create a new render pass
    pub fn new() -> Self {
        Self {
            state: RenderState::default(),
            last_topology_version: u64::MAX,
        }
    }

    /// Prepare render state for a frame
    ///
    /// This updates the render state based on the current schematic state.
    /// Returns true if any layers need re-rendering.
    pub fn prepare(
        &mut self,
        schematic: &SchematicState,
        bridge: &GpuSchematicBridge,
        viewport: ViewportState,
    ) -> bool {
        // Update viewport
        self.state.viewport = viewport;
        self.state.grid = GridState::new(schematic.grid_size);

        // Check for topology change
        let topology_changed = schematic.topology_version() != self.last_topology_version;
        if topology_changed {
            self.last_topology_version = schematic.topology_version();
            self.state.dirty_layers = LayerDirtyFlags::all();
        } else {
            // Check cache dirty flags
            self.state.dirty_layers = LayerDirtyFlags::from_cache_flags(&bridge.dirty_flags());
        }

        // Update wire counts
        self.update_wire_state(bridge);

        // Update component counts
        self.update_component_state(bridge);

        // Update junction count
        self.state.junctions.count = bridge.junction_data().len();

        // Overlays always update (they depend on interaction state)
        self.state.dirty_layers.overlays = true;

        self.state.dirty_layers.any()
    }

    /// Update wire render state from bridge
    fn update_wire_state(&mut self, bridge: &GpuSchematicBridge) {
        let wires = bridge.wire_data();
        let mut normal = 0;
        let mut selected = 0;

        for wire in wires {
            if wire.selected {
                selected += 1;
            } else {
                normal += 1;
            }
        }

        self.state.wires = WireRenderState {
            normal_count: normal,
            selected_count: selected,
            highlighted_count: 0, // TODO: cross-probe highlighting
        };
    }

    /// Update component render state from bridge
    fn update_component_state(&mut self, bridge: &GpuSchematicBridge) {
        let components = bridge.component_data();
        let mut normal = 0;
        let mut selected = 0;

        for comp in components {
            if comp.selected {
                selected += 1;
            } else {
                normal += 1;
            }
        }

        self.state.components = ComponentRenderState {
            normal_count: normal,
            selected_count: selected,
            highlighted_count: 0,
        };
    }

    /// Set overlay state
    pub fn set_overlay(&mut self, overlay: OverlayState) {
        self.state.overlay = overlay;
    }

    /// Get visible layers in draw order
    pub fn visible_layers(&self, tool: &Tool, wire_drawing: bool) -> Vec<RenderLayer> {
        let has_selection =
            self.state.wires.selected_count > 0 || self.state.components.selected_count > 0;

        RenderLayer::all_in_order()
            .iter()
            .filter(|layer| layer.is_visible(tool, wire_drawing, has_selection))
            .copied()
            .collect()
    }

    /// Check if a specific layer needs update
    pub fn layer_needs_update(&self, layer: RenderLayer) -> bool {
        match layer {
            RenderLayer::Grid => self.state.dirty_layers.grid,
            RenderLayer::WiresNormal
            | RenderLayer::WiresSelected
            | RenderLayer::WiresHighlighted => self.state.dirty_layers.wires,
            RenderLayer::ComponentsNormal
            | RenderLayer::ComponentsSelected
            | RenderLayer::ComponentsHighlighted => self.state.dirty_layers.components,
            RenderLayer::Junctions => self.state.dirty_layers.junctions,
            RenderLayer::Labels | RenderLayer::NetLabels => self.state.dirty_layers.labels,
            _ => self.state.dirty_layers.overlays,
        }
    }
}

// =============================================================================
// Render Statistics
// =============================================================================

/// Per-frame render statistics
#[derive(Debug, Clone, Copy, Default)]
pub struct RenderStats {
    /// Total draw calls this frame
    pub draw_calls: u32,
    /// Total vertices rendered
    pub vertex_count: u32,
    /// Total instances rendered
    pub instance_count: u32,
    /// Time spent preparing (microseconds)
    pub prepare_time_us: u64,
    /// Time spent rendering (microseconds)
    pub render_time_us: u64,
    /// Frame number
    pub frame_number: u64,
}

impl RenderStats {
    /// Calculate FPS from prepare + render time
    pub fn estimated_fps(&self) -> f32 {
        let total_us = self.prepare_time_us + self.render_time_us;
        if total_us > 0 {
            1_000_000.0 / total_us as f32
        } else {
            0.0
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Component, ComponentType, Wire};

    // =========================================================================
    // RenderLayer Tests
    // =========================================================================

    #[test]
    fn test_render_layer_order() {
        let layers = RenderLayer::all_in_order();
        assert_eq!(layers[0], RenderLayer::Grid);
        assert_eq!(layers[layers.len() - 1], RenderLayer::Cursor);
    }

    #[test]
    fn test_render_layer_count() {
        let layers = RenderLayer::all_in_order();
        assert_eq!(layers.len(), 15);
    }

    #[test]
    fn test_render_layer_needs_geometry() {
        assert!(!RenderLayer::Grid.needs_geometry());
        assert!(RenderLayer::WiresNormal.needs_geometry());
        assert!(RenderLayer::ComponentsNormal.needs_geometry());
        assert!(!RenderLayer::BoxSelection.needs_geometry());
    }

    #[test]
    fn test_render_layer_visibility_wire_preview() {
        assert!(!RenderLayer::WirePreview.is_visible(&Tool::Select, false, false));
        assert!(RenderLayer::WirePreview.is_visible(&Tool::Select, true, false));
    }

    #[test]
    fn test_render_layer_visibility_component_preview() {
        assert!(!RenderLayer::ComponentPreview.is_visible(&Tool::Select, false, false));
        assert!(RenderLayer::ComponentPreview.is_visible(
            &Tool::Place(ComponentType::Resistor),
            false,
            false
        ));
    }

    #[test]
    fn test_render_layer_visibility_selection() {
        assert!(!RenderLayer::WiresSelected.is_visible(&Tool::Select, false, false));
        assert!(RenderLayer::WiresSelected.is_visible(&Tool::Select, false, true));
    }

    // =========================================================================
    // ViewportState Tests
    // =========================================================================

    #[test]
    fn test_viewport_default() {
        let vp = ViewportState::default();
        assert_eq!(vp.zoom, 1.0);
        assert_eq!(vp.width, 800.0);
    }

    #[test]
    fn test_viewport_visible_bounds_no_pan() {
        let vp = ViewportState {
            pan: (0.0, 0.0),
            zoom: 1.0,
            width: 800.0,
            height: 600.0,
            grid_size: 10,
        };
        let (min_x, min_y, max_x, max_y) = vp.visible_bounds();
        assert_eq!(min_x, 0.0);
        assert_eq!(min_y, 0.0);
        assert_eq!(max_x, 800.0);
        assert_eq!(max_y, 600.0);
    }

    #[test]
    fn test_viewport_visible_bounds_with_pan() {
        let vp = ViewportState {
            pan: (100.0, 50.0),
            zoom: 1.0,
            width: 800.0,
            height: 600.0,
            grid_size: 10,
        };
        let (min_x, min_y, max_x, max_y) = vp.visible_bounds();
        assert_eq!(min_x, -100.0);
        assert_eq!(min_y, -50.0);
    }

    #[test]
    fn test_viewport_visible_bounds_with_zoom() {
        let vp = ViewportState {
            pan: (0.0, 0.0),
            zoom: 2.0,
            width: 800.0,
            height: 600.0,
            grid_size: 10,
        };
        let (min_x, min_y, max_x, max_y) = vp.visible_bounds();
        assert_eq!(max_x, 400.0); // Half visible at 2x zoom
        assert_eq!(max_y, 300.0);
    }

    #[test]
    fn test_viewport_is_visible() {
        let vp = ViewportState::default();
        assert!(vp.is_visible(400.0, 300.0)); // Center
        assert!(vp.is_visible(0.0, 0.0)); // Top-left
        assert!(!vp.is_visible(-10.0, 0.0)); // Outside left
        assert!(!vp.is_visible(810.0, 0.0)); // Outside right
    }

    // =========================================================================
    // GridState Tests
    // =========================================================================

    #[test]
    fn test_grid_state_new() {
        let grid = GridState::new(10);
        assert!(grid.visible);
        // GridState uses fixed 20px minor grid matching SVG (ignores grid_size)
        assert_eq!(grid.minor_spacing, 20.0);
        assert_eq!(grid.major_interval, 5);
    }

    // =========================================================================
    // LayerDirtyFlags Tests
    // =========================================================================

    #[test]
    fn test_layer_dirty_flags_default() {
        let flags = LayerDirtyFlags::default();
        assert!(!flags.any());
    }

    #[test]
    fn test_layer_dirty_flags_all() {
        let flags = LayerDirtyFlags::all();
        assert!(flags.grid);
        assert!(flags.wires);
        assert!(flags.components);
        assert!(flags.any());
    }

    #[test]
    fn test_layer_dirty_flags_clear() {
        let mut flags = LayerDirtyFlags::all();
        flags.clear();
        assert!(!flags.any());
    }

    #[test]
    fn test_layer_dirty_flags_from_cache() {
        let cache_flags = DirtyFlags {
            wires: true,
            components: false,
            junctions: true,
            selection: false,
            grid: false,
            camera: false,
        };
        let layer_flags = LayerDirtyFlags::from_cache_flags(&cache_flags);
        assert!(layer_flags.wires);
        assert!(!layer_flags.components);
        assert!(layer_flags.junctions);
    }

    // =========================================================================
    // RenderPass Tests
    // =========================================================================

    #[test]
    fn test_render_pass_new() {
        let pass = RenderPass::new();
        assert_eq!(pass.last_topology_version, u64::MAX);
    }

    #[test]
    fn test_render_pass_visible_layers_select_tool() {
        let mut pass = RenderPass::new();
        pass.state.wires.selected_count = 1;

        let layers = pass.visible_layers(&Tool::Select, false);

        assert!(layers.contains(&RenderLayer::Grid));
        assert!(layers.contains(&RenderLayer::WiresNormal));
        assert!(layers.contains(&RenderLayer::WiresSelected));
        assert!(!layers.contains(&RenderLayer::WirePreview)); // No wire drawing
    }

    #[test]
    fn test_render_pass_visible_layers_wire_drawing() {
        let pass = RenderPass::new();
        let layers = pass.visible_layers(&Tool::Wire, true);

        assert!(layers.contains(&RenderLayer::WirePreview));
    }

    #[test]
    fn test_render_pass_visible_layers_place_tool() {
        let pass = RenderPass::new();
        let layers = pass.visible_layers(&Tool::Place(ComponentType::Resistor), false);

        assert!(layers.contains(&RenderLayer::ComponentPreview));
    }

    #[test]
    fn test_render_pass_layer_needs_update() {
        let mut pass = RenderPass::new();
        pass.state.dirty_layers.wires = true;
        pass.state.dirty_layers.components = false;

        assert!(pass.layer_needs_update(RenderLayer::WiresNormal));
        assert!(pass.layer_needs_update(RenderLayer::WiresSelected));
        assert!(!pass.layer_needs_update(RenderLayer::ComponentsNormal));
    }

    // =========================================================================
    // RenderStats Tests
    // =========================================================================

    #[test]
    fn test_render_stats_default() {
        let stats = RenderStats::default();
        assert_eq!(stats.draw_calls, 0);
        assert_eq!(stats.frame_number, 0);
    }

    #[test]
    fn test_render_stats_estimated_fps() {
        let stats = RenderStats {
            prepare_time_us: 8000,
            render_time_us: 8000, // 16ms total = ~60fps
            ..Default::default()
        };
        let fps = stats.estimated_fps();
        assert!(fps > 50.0 && fps < 70.0);
    }

    #[test]
    fn test_render_stats_zero_time() {
        let stats = RenderStats::default();
        assert_eq!(stats.estimated_fps(), 0.0);
    }

    // =========================================================================
    // OverlayState Tests
    // =========================================================================

    #[test]
    fn test_overlay_state_default() {
        let overlay = OverlayState::default();
        assert!(!overlay.box_selection_active);
        assert!(!overlay.drag_active);
        assert!(overlay.wire_preview.is_empty());
    }

    // =========================================================================
    // Integration Tests
    // =========================================================================

    #[test]
    fn test_render_pass_full_cycle() {
        let pass = RenderPass::new();

        // Verify initial state
        assert_eq!(pass.state.wires.normal_count, 0);
        assert_eq!(pass.state.components.normal_count, 0);

        // Would need actual schematic and bridge for full test
    }

    #[test]
    fn test_layer_ordering_is_correct() {
        let layers = RenderLayer::all_in_order();

        // Grid must be first
        assert_eq!(layers[0], RenderLayer::Grid);

        // Wires before components
        let wire_idx = layers
            .iter()
            .position(|l| *l == RenderLayer::WiresNormal)
            .unwrap();
        let comp_idx = layers
            .iter()
            .position(|l| *l == RenderLayer::ComponentsNormal)
            .unwrap();
        assert!(wire_idx < comp_idx);

        // Junctions after components
        let junction_idx = layers
            .iter()
            .position(|l| *l == RenderLayer::Junctions)
            .unwrap();
        assert!(junction_idx > comp_idx);

        // Overlays last
        let box_idx = layers
            .iter()
            .position(|l| *l == RenderLayer::BoxSelection)
            .unwrap();
        assert!(box_idx > junction_idx);
    }
}
