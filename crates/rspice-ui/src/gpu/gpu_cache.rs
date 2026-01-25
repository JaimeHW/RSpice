//! GPU Render Cache
//!
//! Commercial-grade GPU data management following the RenderContext pattern.
//! This module provides efficient synchronization between SchematicState and
//! GPU buffers with incremental updates and dirty tracking.
//!
//! # Architecture
//!
//! Professional EDA tools (Cadence, Altium) maintain separate CPU and GPU
//! representations with explicit synchronization:
//!
//! 1. **Dirty Tracking**: Track which data types have changed (wires, components, etc.)
//! 2. **Incremental Updates**: Only rebuild changed buffers, not entire GPU state
//! 3. **Double Buffering**: Allow GPU to render while CPU prepares next frame
//! 4. **Version Coherence**: Match topology_version with RenderContext for consistency
//!
//! # Usage
//!
//! ```ignore
//! // Per-frame update
//! let render_ctx = RenderContext::prepare(&schematic.read(), grid_size);
//! gpu_cache.synchronize(&schematic.read(), &render_ctx);
//!
//! // Render using synchronized GPU data
//! renderer.render(&gpu_cache);
//! ```

use std::collections::HashSet;

use crate::gpu::renderer::{ComponentData, JunctionData, WireData};
use crate::state::render_context::RenderContext;
use crate::state::{ComponentType, Point, Rotation, SchematicState};

// =============================================================================
// Dirty Flags
// =============================================================================

/// Bitflags for tracking which data needs GPU update
#[derive(Debug, Clone, Copy, Default)]
pub struct DirtyFlags {
    /// Wire geometry changed
    pub wires: bool,
    /// Component geometry changed
    pub components: bool,
    /// Junction dots changed
    pub junctions: bool,
    /// Selection state changed
    pub selection: bool,
    /// Grid/background changed
    pub grid: bool,
    /// Camera/view changed (requires uniform update)
    pub camera: bool,
}

impl DirtyFlags {
    /// Create with all flags set (full rebuild needed)
    pub fn all() -> Self {
        Self {
            wires: true,
            components: true,
            junctions: true,
            selection: true,
            grid: true,
            camera: true,
        }
    }

    /// Check if any geometry has changed (requires buffer update)
    pub fn any_geometry(&self) -> bool {
        self.wires || self.components || self.junctions
    }

    /// Check if any flag is set
    pub fn any(&self) -> bool {
        self.wires || self.components || self.junctions || self.selection || self.grid || self.camera
    }

    /// Clear all flags
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

// =============================================================================
// GPU Render Cache
// =============================================================================

/// Cache of GPU-compatible data converted from SchematicState.
///
/// This struct maintains the CPU-side staging area for GPU buffers.
/// When topology changes, we rebuild the affected data and mark it
/// for upload to GPU buffers.
///
/// # Design Rationale
///
/// Professional simulators separate:
/// 1. **Schematic State** - Authoritative data model (SchematicState)
/// 2. **Render Cache** - CPU-side cache for rendering (RenderContext)
/// 3. **GPU Cache** - GPU-compatible representation (this struct)
/// 4. **GPU Buffers** - Actual GPU memory (in SchematicRenderer)
///
/// This separation allows:
/// - Independent optimization of each layer
/// - Clear data flow and ownership
/// - Testability without GPU
/// - Graceful degradation to CPU rendering
#[derive(Debug, Clone)]
pub struct GpuRenderCache {
    /// Converted wire data ready for GPU
    pub wires: Vec<WireData>,

    /// Converted component data ready for GPU
    pub components: Vec<ComponentData>,

    /// Junction dot data ready for GPU
    pub junctions: Vec<JunctionData>,

    /// Currently selected wire IDs (for highlight pass)
    pub selected_wires: HashSet<u64>,

    /// Currently selected component IDs (for highlight pass)
    pub selected_components: HashSet<u64>,

    /// Grid spacing in world units
    pub grid_size: f32,

    /// Last synchronized topology version
    topology_version: u64,

    /// Last synchronized selection version (for selection-only updates)
    selection_version: u64,

    /// Dirty flags indicating what needs GPU upload
    pub dirty: DirtyFlags,

    /// Statistics for performance monitoring
    pub stats: CacheStats,
}

/// Performance statistics for the GPU cache
#[derive(Debug, Clone, Copy, Default)]
pub struct CacheStats {
    /// Number of wire segments cached
    pub wire_segment_count: usize,
    /// Number of components cached
    pub component_count: usize,
    /// Number of junctions cached
    pub junction_count: usize,
    /// Last rebuild time in microseconds
    pub last_rebuild_us: u64,
}

impl Default for GpuRenderCache {
    fn default() -> Self {
        Self::new()
    }
}

impl GpuRenderCache {
    /// Create empty GPU render cache
    pub fn new() -> Self {
        Self {
            wires: Vec::new(),
            components: Vec::new(),
            junctions: Vec::new(),
            selected_wires: HashSet::new(),
            selected_components: HashSet::new(),
            grid_size: 10.0,
            topology_version: u64::MAX, // Force initial rebuild
            selection_version: 0,
            dirty: DirtyFlags::all(),
            stats: CacheStats::default(),
        }
    }

    /// Check if cache needs full rebuild for the given topology version
    pub fn needs_rebuild(&self, current_version: u64) -> bool {
        self.topology_version != current_version
    }

    /// Get the topology version this cache was built for
    pub fn topology_version(&self) -> u64 {
        self.topology_version
    }

    /// Synchronize GPU cache with schematic state.
    ///
    /// This is the main entry point for frame updates. It:
    /// 1. Checks if topology has changed
    /// 2. Rebuilds changed data if needed
    /// 3. Updates selection state
    /// 4. Sets dirty flags for GPU buffer uploads
    ///
    /// # Performance
    /// - No-op if nothing changed: O(1)
    /// - Selection-only change: O(selection size)
    /// - Topology change: O(n) but only once per change
    pub fn synchronize(
        &mut self,
        schematic: &SchematicState,
        render_ctx: &RenderContext,
    ) {
        let start = std::time::Instant::now();
        let current_version = schematic.topology_version();

        // Check for topology change (structural change)
        if self.topology_version != current_version {
            self.rebuild_geometry(schematic, render_ctx);
            self.topology_version = current_version;
            self.dirty.wires = true;
            self.dirty.components = true;
            self.dirty.junctions = true;
        }

        // Always sync selection (cheap and allows per-frame selection updates)
        self.sync_selection(schematic);

        // Update stats
        self.stats.last_rebuild_us = start.elapsed().as_micros() as u64;
    }

    /// Rebuild all geometry from schematic state
    fn rebuild_geometry(
        &mut self,
        schematic: &SchematicState,
        render_ctx: &RenderContext,
    ) {
        self.rebuild_wires(schematic);
        self.rebuild_components(schematic);
        self.rebuild_junctions(schematic, render_ctx);
        self.grid_size = schematic.grid_size as f32;
    }

    /// Rebuild wire data
    fn rebuild_wires(&mut self, schematic: &SchematicState) {
        self.wires.clear();
        self.wires.reserve(schematic.wires.len());

        for wire in &schematic.wires {
            let points: Vec<[f32; 2]> = wire
                .points
                .iter()
                .map(|p| [p.x as f32, p.y as f32])
                .collect();

            let selected = schematic.selection.has_wire(wire.id);

            self.wires.push(WireData { points, selected });
        }

        // Update stats
        self.stats.wire_segment_count = self.wires.iter()
            .map(|w| w.points.len().saturating_sub(1))
            .sum();
    }

    /// Rebuild component data
    fn rebuild_components(&mut self, schematic: &SchematicState) {
        self.components.clear();
        self.components.reserve(schematic.components.len());

        for comp in &schematic.components {
            let component_data = ComponentData {
                x: comp.pos.x as f32,
                y: comp.pos.y as f32,
                rotation: rotation_to_radians(&comp.rotation),
                symbol_type: component_type_to_symbol_id(&comp.kind),
                selected: schematic.selection.has_component(comp.id),
            };

            self.components.push(component_data);
        }

        self.stats.component_count = self.components.len();
    }

    /// Rebuild junction data from render context
    ///
    /// Uses RenderContext's junction_points which already handles
    /// the complex logic of 3+ wire endpoints and terminal exclusion.
    fn rebuild_junctions(
        &mut self,
        schematic: &SchematicState,
        render_ctx: &RenderContext,
    ) {
        self.junctions.clear();

        // Get auto-detected junctions from render context
        for point in render_ctx.junction_points() {
            self.junctions.push(JunctionData {
                x: point.x as f32,
                y: point.y as f32,
                selected: false, // Auto-junctions aren't individually selectable
            });
        }

        // Add explicit user-placed junctions (junctions don't have selection in Selection struct)
        for junction in &schematic.junctions {
            self.junctions.push(JunctionData {
                x: junction.pos.x as f32,
                y: junction.pos.y as f32,
                selected: false, // Junctions aren't in Selection
            });
        }

        self.stats.junction_count = self.junctions.len();
    }

    /// Sync selection state without full geometry rebuild
    fn sync_selection(&mut self, schematic: &SchematicState) {
        // Clear previous selection tracking
        self.selected_wires.clear();
        self.selected_components.clear();

        // Collect current selection from Vec fields
        for &id in &schematic.selection.wires {
            self.selected_wires.insert(id);
        }
        for &id in &schematic.selection.components {
            self.selected_components.insert(id);
        }

        // Update selection flags in cached data
        // This is O(n) but selection changes are rare compared to renders
        for (i, wire) in self.wires.iter_mut().enumerate() {
            if let Some(orig_wire) = schematic.wires.get(i) {
                let is_selected = schematic.selection.has_wire(orig_wire.id);
                if wire.selected != is_selected {
                    wire.selected = is_selected;
                    self.dirty.selection = true;
                }
            }
        }

        for (i, comp) in self.components.iter_mut().enumerate() {
            if let Some(orig_comp) = schematic.components.get(i) {
                let is_selected = schematic.selection.has_component(orig_comp.id);
                if comp.selected != is_selected {
                    comp.selected = is_selected;
                    self.dirty.selection = true;
                }
            }
        }
    }

    /// Mark camera as dirty (requires uniform upload)
    pub fn mark_camera_dirty(&mut self) {
        self.dirty.camera = true;
    }

    /// Mark grid as dirty (when grid size changes)
    pub fn mark_grid_dirty(&mut self) {
        self.dirty.grid = true;
    }

    /// Clear dirty flags after GPU upload
    pub fn clear_dirty(&mut self) {
        self.dirty.clear();
    }

    /// Check if the cache has any data
    pub fn is_empty(&self) -> bool {
        self.wires.is_empty() && self.components.is_empty() && self.junctions.is_empty()
    }

    /// Get total vertex count estimate for GPU buffer sizing
    pub fn estimated_vertex_count(&self) -> usize {
        // Wires: 6 vertices per segment (2 triangles)
        let wire_verts = self.stats.wire_segment_count * 6;
        // Components: ~20 vertices per component average
        let comp_verts = self.stats.component_count * 20;
        // Junctions: ~12 vertices per circle (hexagon approximation)
        let junction_verts = self.stats.junction_count * 12;

        wire_verts + comp_verts + junction_verts
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Convert rotation enum to radians
fn rotation_to_radians(rotation: &Rotation) -> f32 {
    match rotation {
        Rotation::R0 => 0.0,
        Rotation::R90 => std::f32::consts::FRAC_PI_2,
        Rotation::R180 => std::f32::consts::PI,
        Rotation::R270 => 3.0 * std::f32::consts::FRAC_PI_2,
    }
}

/// Map component type to GPU symbol ID for instanced rendering
///
/// Each component type gets a unique ID that maps to a symbol atlas entry.
/// IDs are grouped by category for cache-friendly access patterns.
fn component_type_to_symbol_id(component_type: &ComponentType) -> u32 {
    match component_type {
        // Passives (0-9)
        ComponentType::Resistor => 0,
        ComponentType::Capacitor => 1,
        ComponentType::Inductor => 2,
        ComponentType::CoupledInductor => 3,

        // Sources (10-29)
        ComponentType::VoltageSource => 10,
        ComponentType::CurrentSource => 11,
        ComponentType::VoltageSourceAc => 12,
        ComponentType::VoltageSourcePulse => 13,
        ComponentType::VoltageSourceSin => 14,
        ComponentType::VoltageSourcePwl => 15,
        ComponentType::VoltageSourceExp => 16,
        ComponentType::VoltageSourceSffm => 17,
        ComponentType::CurrentSourceAc => 18,
        ComponentType::CurrentSourcePulse => 19,
        ComponentType::CurrentSourceSin => 20,
        ComponentType::CurrentSourcePwl => 21,
        ComponentType::CurrentSourceExp => 22,
        ComponentType::CurrentSourceNoise => 23,

        // Controlled sources (30-39)
        ComponentType::Vcvs => 30,
        ComponentType::Vccs => 31,
        ComponentType::Ccvs => 32,
        ComponentType::Cccs => 33,

        // Semiconductors (40-59)
        ComponentType::Diode => 40,
        ComponentType::NpnBjt => 41,
        ComponentType::PnpBjt => 42,
        ComponentType::Nmos => 43,
        ComponentType::Pmos => 44,
        ComponentType::Njfet => 45,
        ComponentType::Pjfet => 46,
        ComponentType::NVdmos => 47,
        ComponentType::PVdmos => 48,
        ComponentType::SaturableInductor => 49,

        // Special (60-69)
        ComponentType::Ground => 60,

        // XSPICE Analog (70-79)
        ComponentType::XspiceGain => 70,
        ComponentType::XspiceSummer => 71,
        ComponentType::XspiceMultiplier => 72,
        ComponentType::XspiceDivider => 73,
        ComponentType::XspiceLimiter => 74,
        ComponentType::XspiceIntegrator => 75,
        ComponentType::XspiceDifferentiator => 76,

        // XSPICE Digital (80-99)
        ComponentType::XspiceInverter => 80,
        ComponentType::XspiceBuffer => 81,
        ComponentType::XspiceAndGate => 82,
        ComponentType::XspiceOrGate => 83,
        ComponentType::XspiceNandGate => 84,
        ComponentType::XspiceNorGate => 85,
        ComponentType::XspiceXorGate => 86,
        ComponentType::XspiceTristate => 87,
        ComponentType::XspiceDFlipFlop => 88,
        ComponentType::XspiceJkFlipFlop => 89,
        ComponentType::XspiceSrLatch => 90,
        ComponentType::XspiceAdcBridge => 91,
        ComponentType::XspiceDacBridge => 92,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Component, Wire};

    fn make_schematic() -> SchematicState {
        let mut schematic = SchematicState::default();
        schematic.components.push(Component::new(
            1,
            ComponentType::Resistor,
            Point::new(0, 0),
        ));
        schematic.wires.push(Wire::new(
            1,
            vec![Point::new(2, 0), Point::new(10, 0)],
        ));
        schematic.bump_topology_version();
        schematic
    }

    fn make_complex_schematic() -> SchematicState {
        let mut schematic = SchematicState::default();

        // Add various component types
        schematic.components.push(Component::new(1, ComponentType::Resistor, Point::new(0, 0)));
        schematic.components.push(Component::new(2, ComponentType::Capacitor, Point::new(10, 0)));
        schematic.components.push(Component::new(3, ComponentType::VoltageSource, Point::new(20, 0)));
        schematic.components.push(Component::new(4, ComponentType::Ground, Point::new(30, 0)));
        schematic.components.push(Component::new(5, ComponentType::Diode, Point::new(40, 0)));
        schematic.components.push(Component::new(6, ComponentType::Nmos, Point::new(50, 0)));

        // Add multiple wires
        schematic.wires.push(Wire::new(1, vec![Point::new(2, 0), Point::new(10, 0)]));
        schematic.wires.push(Wire::new(2, vec![Point::new(12, 0), Point::new(20, 0)]));
        schematic.wires.push(Wire::new(3, vec![Point::new(22, 0), Point::new(30, 0), Point::new(30, 10)]));

        schematic.bump_topology_version();
        schematic
    }

    // =========================================================================
    // Basic Functionality Tests
    // =========================================================================

    #[test]
    fn test_new_cache_needs_rebuild() {
        let cache = GpuRenderCache::new();
        assert!(cache.needs_rebuild(1));
        assert!(cache.dirty.any());
    }

    #[test]
    fn test_new_cache_is_empty() {
        let cache = GpuRenderCache::new();
        assert!(cache.is_empty());
        assert_eq!(cache.wires.len(), 0);
        assert_eq!(cache.components.len(), 0);
        assert_eq!(cache.junctions.len(), 0);
    }

    #[test]
    fn test_synchronize_updates_version() {
        let mut cache = GpuRenderCache::new();
        let schematic = make_schematic();
        let render_ctx = RenderContext::new();

        cache.synchronize(&schematic, &render_ctx);

        assert_eq!(cache.topology_version(), schematic.topology_version());
        assert!(!cache.needs_rebuild(schematic.topology_version()));
    }

    // =========================================================================
    // Wire Conversion Tests
    // =========================================================================

    #[test]
    fn test_wires_converted() {
        let mut cache = GpuRenderCache::new();
        let schematic = make_schematic();
        let render_ctx = RenderContext::new();

        cache.synchronize(&schematic, &render_ctx);

        assert_eq!(cache.wires.len(), 1);
        assert_eq!(cache.wires[0].points.len(), 2);
        assert_eq!(cache.wires[0].points[0], [2.0, 0.0]);
        assert_eq!(cache.wires[0].points[1], [10.0, 0.0]);
    }

    #[test]
    fn test_multi_segment_wire() {
        let mut cache = GpuRenderCache::new();
        let schematic = make_complex_schematic();
        let render_ctx = RenderContext::new();

        cache.synchronize(&schematic, &render_ctx);

        // Wire 3 has 3 points (2 segments)
        assert_eq!(cache.wires[2].points.len(), 3);
        assert_eq!(cache.stats.wire_segment_count, 4); // 1 + 1 + 2 segments
    }

    #[test]
    fn test_wire_selection_state() {
        let mut cache = GpuRenderCache::new();
        let mut schematic = make_schematic();
        let render_ctx = RenderContext::new();

        // Initially not selected
        cache.synchronize(&schematic, &render_ctx);
        assert!(!cache.wires[0].selected);

        // Select wire
        schematic.selection.toggle_wire(1);
        cache.synchronize(&schematic, &render_ctx);
        assert!(cache.wires[0].selected);

        // Deselect wire
        schematic.selection.toggle_wire(1);
        cache.synchronize(&schematic, &render_ctx);
        assert!(!cache.wires[0].selected);
    }

    // =========================================================================
    // Component Conversion Tests
    // =========================================================================

    #[test]
    fn test_components_converted() {
        let mut cache = GpuRenderCache::new();
        let schematic = make_schematic();
        let render_ctx = RenderContext::new();

        cache.synchronize(&schematic, &render_ctx);

        assert_eq!(cache.components.len(), 1);
        assert_eq!(cache.components[0].x, 0.0);
        assert_eq!(cache.components[0].y, 0.0);
        assert_eq!(cache.components[0].symbol_type, 0); // Resistor
    }

    #[test]
    fn test_multiple_component_types() {
        let mut cache = GpuRenderCache::new();
        let schematic = make_complex_schematic();
        let render_ctx = RenderContext::new();

        cache.synchronize(&schematic, &render_ctx);

        assert_eq!(cache.components.len(), 6);
        assert_eq!(cache.components[0].symbol_type, 0);  // Resistor
        assert_eq!(cache.components[1].symbol_type, 1);  // Capacitor
        assert_eq!(cache.components[2].symbol_type, 10); // VoltageSource
        assert_eq!(cache.components[3].symbol_type, 60); // Ground
        assert_eq!(cache.components[4].symbol_type, 40); // Diode
        assert_eq!(cache.components[5].symbol_type, 43); // Nmos
    }

    #[test]
    fn test_component_selection_state() {
        let mut cache = GpuRenderCache::new();
        let mut schematic = make_schematic();
        let render_ctx = RenderContext::new();

        // Initial sync - not selected
        cache.synchronize(&schematic, &render_ctx);
        assert!(!cache.components[0].selected);

        // Select component
        schematic.selection.toggle_component(1);
        cache.synchronize(&schematic, &render_ctx);
        assert!(cache.components[0].selected);
    }

    #[test]
    fn test_component_rotation() {
        let mut cache = GpuRenderCache::new();
        let mut schematic = SchematicState::default();

        // Add rotated components
        let mut comp1 = Component::new(1, ComponentType::Resistor, Point::new(0, 0));
        comp1.rotation = Rotation::R0;
        schematic.components.push(comp1);

        let mut comp2 = Component::new(2, ComponentType::Resistor, Point::new(10, 0));
        comp2.rotation = Rotation::R90;
        schematic.components.push(comp2);

        let mut comp3 = Component::new(3, ComponentType::Resistor, Point::new(20, 0));
        comp3.rotation = Rotation::R180;
        schematic.components.push(comp3);

        let mut comp4 = Component::new(4, ComponentType::Resistor, Point::new(30, 0));
        comp4.rotation = Rotation::R270;
        schematic.components.push(comp4);

        schematic.bump_topology_version();
        let render_ctx = RenderContext::new();
        cache.synchronize(&schematic, &render_ctx);

        assert!((cache.components[0].rotation - 0.0).abs() < 0.001);
        assert!((cache.components[1].rotation - std::f32::consts::FRAC_PI_2).abs() < 0.001);
        assert!((cache.components[2].rotation - std::f32::consts::PI).abs() < 0.001);
        assert!((cache.components[3].rotation - 3.0 * std::f32::consts::FRAC_PI_2).abs() < 0.001);
    }

    // =========================================================================
    // Selection Sync Tests
    // =========================================================================

    #[test]
    fn test_selection_sync_multiple_items() {
        let mut cache = GpuRenderCache::new();
        let mut schematic = make_complex_schematic();
        let render_ctx = RenderContext::new();

        // Select multiple items
        schematic.selection.select_component(1);
        schematic.selection.select_component(3);
        schematic.selection.select_wire(2);

        cache.synchronize(&schematic, &render_ctx);

        assert!(cache.components[0].selected); // id=1
        assert!(!cache.components[1].selected); // id=2
        assert!(cache.components[2].selected); // id=3
        assert!(!cache.wires[0].selected); // id=1
        assert!(cache.wires[1].selected); // id=2

        assert!(cache.selected_components.contains(&1));
        assert!(cache.selected_components.contains(&3));
        assert!(cache.selected_wires.contains(&2));
    }

    #[test]
    fn test_selection_dirty_flag() {
        let mut cache = GpuRenderCache::new();
        let mut schematic = make_schematic();
        let mut render_ctx = RenderContext::new();
        render_ctx.rebuild(&schematic.components, &schematic.wires, 10, schematic.topology_version());

        // First sync
        cache.synchronize(&schematic, &render_ctx);
        cache.clear_dirty();

        // Change selection
        schematic.selection.select_component(1);
        cache.synchronize(&schematic, &render_ctx);

        assert!(cache.dirty.selection);
    }

    // =========================================================================
    // Dirty Flags Tests
    // =========================================================================

    #[test]
    fn test_no_rebuild_if_unchanged() {
        let mut cache = GpuRenderCache::new();
        let schematic = make_schematic();
        let mut render_ctx = RenderContext::new();
        render_ctx.rebuild(&schematic.components, &schematic.wires, 10, schematic.topology_version());

        // First sync
        cache.synchronize(&schematic, &render_ctx);
        cache.clear_dirty();

        // Second sync - should not be dirty
        cache.synchronize(&schematic, &render_ctx);
        assert!(!cache.dirty.wires);
        assert!(!cache.dirty.components);
        assert!(!cache.dirty.junctions);
    }

    #[test]
    fn test_dirty_flags_clear() {
        let mut cache = GpuRenderCache::new();
        assert!(cache.dirty.any());

        cache.clear_dirty();
        assert!(!cache.dirty.any());
    }

    #[test]
    fn test_dirty_flags_all() {
        let flags = DirtyFlags::all();
        assert!(flags.wires);
        assert!(flags.components);
        assert!(flags.junctions);
        assert!(flags.selection);
        assert!(flags.grid);
        assert!(flags.camera);
        assert!(flags.any());
        assert!(flags.any_geometry());
    }

    #[test]
    fn test_dirty_flags_any_geometry() {
        let mut flags = DirtyFlags::default();
        assert!(!flags.any_geometry());

        flags.wires = true;
        assert!(flags.any_geometry());

        flags.wires = false;
        flags.components = true;
        assert!(flags.any_geometry());

        flags.components = false;
        flags.junctions = true;
        assert!(flags.any_geometry());
    }

    #[test]
    fn test_mark_camera_dirty() {
        let mut cache = GpuRenderCache::new();
        cache.clear_dirty();
        assert!(!cache.dirty.camera);

        cache.mark_camera_dirty();
        assert!(cache.dirty.camera);
    }

    #[test]
    fn test_mark_grid_dirty() {
        let mut cache = GpuRenderCache::new();
        cache.clear_dirty();
        assert!(!cache.dirty.grid);

        cache.mark_grid_dirty();
        assert!(cache.dirty.grid);
    }

    // =========================================================================
    // Statistics Tests
    // =========================================================================

    #[test]
    fn test_stats_updated() {
        let mut cache = GpuRenderCache::new();
        let schematic = make_schematic();
        let render_ctx = RenderContext::new();

        cache.synchronize(&schematic, &render_ctx);

        assert_eq!(cache.stats.component_count, 1);
        assert_eq!(cache.stats.wire_segment_count, 1);
    }

    #[test]
    fn test_stats_complex_schematic() {
        let mut cache = GpuRenderCache::new();
        let schematic = make_complex_schematic();
        let render_ctx = RenderContext::new();

        cache.synchronize(&schematic, &render_ctx);

        assert_eq!(cache.stats.component_count, 6);
        assert_eq!(cache.stats.wire_segment_count, 4); // 1 + 1 + 2 segments
    }

    #[test]
    fn test_estimated_vertex_count() {
        let mut cache = GpuRenderCache::new();
        let schematic = make_complex_schematic();
        let render_ctx = RenderContext::new();

        cache.synchronize(&schematic, &render_ctx);

        let estimate = cache.estimated_vertex_count();
        // 4 wire segments * 6 = 24
        // 6 components * 20 = 120
        // junctions * 12
        assert!(estimate >= 24 + 120);
    }

    // =========================================================================
    // Component Type Mapping Tests
    // =========================================================================

    #[test]
    fn test_component_type_mapping_passives() {
        assert_eq!(component_type_to_symbol_id(&ComponentType::Resistor), 0);
        assert_eq!(component_type_to_symbol_id(&ComponentType::Capacitor), 1);
        assert_eq!(component_type_to_symbol_id(&ComponentType::Inductor), 2);
        assert_eq!(component_type_to_symbol_id(&ComponentType::CoupledInductor), 3);
    }

    #[test]
    fn test_component_type_mapping_sources() {
        assert_eq!(component_type_to_symbol_id(&ComponentType::VoltageSource), 10);
        assert_eq!(component_type_to_symbol_id(&ComponentType::CurrentSource), 11);
        assert_eq!(component_type_to_symbol_id(&ComponentType::VoltageSourceAc), 12);
    }

    #[test]
    fn test_component_type_mapping_semiconductors() {
        assert_eq!(component_type_to_symbol_id(&ComponentType::Diode), 40);
        assert_eq!(component_type_to_symbol_id(&ComponentType::NpnBjt), 41);
        assert_eq!(component_type_to_symbol_id(&ComponentType::PnpBjt), 42);
        assert_eq!(component_type_to_symbol_id(&ComponentType::Nmos), 43);
        assert_eq!(component_type_to_symbol_id(&ComponentType::Pmos), 44);
    }

    #[test]
    fn test_component_type_mapping_special() {
        assert_eq!(component_type_to_symbol_id(&ComponentType::Ground), 60);
    }

    #[test]
    fn test_component_type_mapping_xspice() {
        assert_eq!(component_type_to_symbol_id(&ComponentType::XspiceGain), 70);
        assert_eq!(component_type_to_symbol_id(&ComponentType::XspiceAndGate), 82);
        assert_eq!(component_type_to_symbol_id(&ComponentType::XspiceDFlipFlop), 88);
    }

    // =========================================================================
    // Rotation Conversion Tests
    // =========================================================================

    #[test]
    fn test_rotation_to_radians() {
        assert!((rotation_to_radians(&Rotation::R0) - 0.0).abs() < 0.001);
        assert!((rotation_to_radians(&Rotation::R90) - std::f32::consts::FRAC_PI_2).abs() < 0.001);
        assert!((rotation_to_radians(&Rotation::R180) - std::f32::consts::PI).abs() < 0.001);
        assert!((rotation_to_radians(&Rotation::R270) - 3.0 * std::f32::consts::FRAC_PI_2).abs() < 0.001);
    }

    // =========================================================================
    // Edge Case Tests
    // =========================================================================

    #[test]
    fn test_empty_schematic() {
        let mut cache = GpuRenderCache::new();
        let schematic = SchematicState::default();
        let render_ctx = RenderContext::new();

        cache.synchronize(&schematic, &render_ctx);

        assert!(cache.wires.is_empty());
        assert!(cache.components.is_empty());
        assert_eq!(cache.stats.component_count, 0);
        assert_eq!(cache.stats.wire_segment_count, 0);
    }

    #[test]
    fn test_single_point_wire() {
        let mut cache = GpuRenderCache::new();
        let mut schematic = SchematicState::default();
        // Wire with just one point (degenerate case)
        schematic.wires.push(Wire::new(1, vec![Point::new(0, 0)]));
        schematic.bump_topology_version();
        let render_ctx = RenderContext::new();

        cache.synchronize(&schematic, &render_ctx);

        assert_eq!(cache.wires.len(), 1);
        assert_eq!(cache.stats.wire_segment_count, 0); // saturating_sub handles this
    }

    #[test]
    fn test_topology_version_change_triggers_rebuild() {
        let mut cache = GpuRenderCache::new();
        let mut schematic = make_schematic();
        let render_ctx = RenderContext::new();

        // First sync
        cache.synchronize(&schematic, &render_ctx);
        cache.clear_dirty();
        let first_count = cache.components.len();

        // Add another component and bump version
        schematic.components.push(Component::new(
            2,
            ComponentType::Capacitor,
            Point::new(20, 0),
        ));
        schematic.bump_topology_version();

        // Should rebuild
        cache.synchronize(&schematic, &render_ctx);
        assert!(cache.dirty.components);
        assert_eq!(cache.components.len(), first_count + 1);
    }

    #[test]
    fn test_grid_size_updated() {
        let mut cache = GpuRenderCache::new();
        let mut schematic = make_schematic();
        schematic.grid_size = 20;
        let render_ctx = RenderContext::new();

        cache.synchronize(&schematic, &render_ctx);

        assert_eq!(cache.grid_size, 20.0);
    }
}
