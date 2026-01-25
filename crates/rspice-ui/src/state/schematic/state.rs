//! Schematic State
//!
//! Main state container for the schematic editor.

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::clipboard::ClipboardData;
use super::component::Component;
use super::component_type::ComponentType;
use super::net_label::{Junction, NetLabel};
use super::point::Point;
use super::rotation::Rotation;
use super::selection::Selection;
use super::tool::Tool;
use super::wire::{Wire, WireConnection, WireDrawing};

// =============================================================================
// Constants
// =============================================================================

/// Default zoom level for serde deserialization (prevents black screen on file load)
fn default_zoom() -> f64 {
    1.0
}

/// Snap distance in grid units for terminal connections
const SNAP_DISTANCE: i32 = 1;

// =============================================================================
// SchematicState
// =============================================================================

/// Main schematic state
///
/// Contains all components, wires, selection state, and interaction state
/// for a single schematic document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchematicState {
    /// All placed components
    pub components: Vec<Component>,

    /// All wires
    pub wires: Vec<Wire>,

    /// Current selection
    pub selection: Selection,

    /// Current tool
    pub tool: Tool,

    /// Wire drawing state
    pub wire_drawing: WireDrawing,

    /// Grid size in pixels
    pub grid_size: i32,

    /// Zoom level (1.0 = 100%) - not part of undo history or saved files
    /// Uses default of 1.0 when deserializing to prevent black screen
    #[serde(skip, default = "default_zoom")]
    pub zoom: f64,

    /// Pan offset in pixels - not part of undo history or saved files
    #[serde(skip, default)]
    pub pan: (f64, f64),

    /// Current schematic file path (for save without dialog)
    #[serde(skip)]
    pub current_file: Option<PathBuf>,

    /// Next component ID (runtime state, not persisted)
    #[serde(skip)]
    next_id: u64,

    /// Component counters for auto-naming (runtime state, not persisted)
    #[serde(skip)]
    component_counters: HashMap<&'static str, u32>,

    /// Clipboard for copy/paste operations
    pub clipboard: ClipboardData,

    /// Net labels for naming nodes
    pub net_labels: Vec<NetLabel>,

    /// Explicit wire junctions for connecting crossing wires
    /// Only wires sharing an endpoint OR joined by an explicit junction are connected
    pub junctions: Vec<Junction>,

    /// Preview rotation for component placement
    pub preview_rotation: Rotation,

    /// Wire-to-terminal connections (for rubber-banding)
    pub connections: Vec<WireConnection>,

    /// Cached point-to-net mapping from last netlist generation (for probe lookup)
    /// Updated after each simulation run
    #[serde(skip)]
    pub net_mapping: HashMap<Point, String>,

    /// Flag indicating unsaved changes (runtime state, not persisted)
    #[serde(skip)]
    pub is_dirty: bool,

    /// Flag indicating zoom_to_fit should be called after next render with actual viewport dimensions.
    /// Set to true when loading a file, cleared after the fit is performed.
    #[serde(skip)]
    pub needs_fit: bool,

    /// Flag indicating the undo history should be reset (e.g., after loading a file).
    /// Set to true when a file is loaded, cleared after history is reset.
    #[serde(skip)]
    pub needs_history_reset: bool,

    /// Topology version counter for cache invalidation (runtime state, not persisted)
    /// Incremented on any structural change (add/remove/move component/wire/junction)
    /// Used by LabelPositionCache and JunctionCache to detect stale data
    #[serde(skip)]
    topology_version: u64,
}

impl Default for SchematicState {
    fn default() -> Self {
        Self {
            components: Vec::new(),
            wires: Vec::new(),
            selection: Selection::default(),
            tool: Tool::default(),
            wire_drawing: WireDrawing::default(),
            grid_size: 10,
            zoom: 1.0,
            pan: (0.0, 0.0),
            current_file: None,
            next_id: 1,
            component_counters: HashMap::new(),
            clipboard: ClipboardData::default(),
            net_labels: Vec::new(),
            junctions: Vec::new(),
            preview_rotation: Rotation::default(),
            connections: Vec::new(),
            net_mapping: HashMap::new(),
            is_dirty: false,
            needs_fit: false,
            needs_history_reset: false,
            topology_version: 0,
        }
    }
}

impl SchematicState {
    // =========================================================================
    // ID and Name Generation
    // =========================================================================

    /// Generate a unique ID
    pub fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Get the current topology version
    ///
    /// This version is incremented whenever the schematic topology changes
    /// (components, wires, junctions added/removed/moved). Used by caches
    /// to detect when they need to be rebuilt.
    pub fn topology_version(&self) -> u64 {
        self.topology_version
    }

    /// Increment the topology version
    ///
    /// Call this after any structural change to invalidate caches.
    /// This is automatically called by mutation methods like add_component,
    /// add_wire, move_component_with_wires, etc.
    pub fn bump_topology_version(&mut self) {
        self.topology_version = self.topology_version.wrapping_add(1);
    }

    /// Recalculate runtime state after loading from file
    /// This MUST be called after deserialization to prevent ID collisions
    pub fn recalculate_runtime_state(&mut self) {
        // Find the maximum ID currently in use (components, wires, and junctions)
        let max_component_id = self.components.iter().map(|c| c.id).max().unwrap_or(0);
        let max_wire_id = self.wires.iter().map(|w| w.id).max().unwrap_or(0);
        let max_junction_id = self.junctions.iter().map(|j| j.id).max().unwrap_or(0);
        self.next_id = max_component_id.max(max_wire_id).max(max_junction_id) + 1;

        // Rebuild component counters from existing component names
        self.component_counters.clear();
        for comp in &self.components {
            let prefix = comp.kind.spice_prefix();
            if !prefix.is_empty() {
                // Extract number from name like "R1", "C5", etc.
                if let Some(num_str) = comp.name.strip_prefix(prefix) {
                    if let Ok(num) = num_str.parse::<u32>() {
                        let counter = self.component_counters.entry(prefix).or_insert(0);
                        *counter = (*counter).max(num);
                    }
                }
            }
        }
    }

    /// Generate a unique component name
    pub fn generate_name(&mut self, kind: ComponentType) -> String {
        let prefix = kind.spice_prefix();
        if prefix.is_empty() {
            return String::new();
        }
        let counter = self.component_counters.entry(prefix).or_insert(0);
        *counter += 1;
        format!("{}{}", prefix, counter)
    }

    // =========================================================================
    // Viewport Management
    // =========================================================================

    /// Zoom to fit all schematic content in the viewport.
    ///
    /// Sets zoom and pan so all components and wires are visible with comfortable margins.
    ///
    /// Parameters:
    /// - `viewport_width`: Width of the schematic canvas in pixels
    /// - `viewport_height`: Height of the schematic canvas in pixels
    pub fn zoom_to_fit(&mut self, viewport_width: f64, viewport_height: f64) {
        // Calculate bounding box of all content (in grid units)
        let bounds = self.content_bounds();

        if bounds.is_none() {
            // No content - reset to default view
            self.zoom = 1.0;
            self.pan = (0.0, 0.0);
            return;
        }

        let (min_x, min_y, max_x, max_y) = bounds.unwrap();
        let gs = self.grid_size as f64;

        // Convert grid units to pixels for proper pan/zoom calculation
        let min_px = min_x as f64 * gs;
        let min_py = min_y as f64 * gs;
        let max_px = max_x as f64 * gs;
        let max_py = max_y as f64 * gs;

        // Add margin (5% of content size, minimum 20 pixels) for a comfortable fit
        let content_width = max_px - min_px;
        let content_height = max_py - min_py;
        let margin = (content_width.max(content_height) * 0.05).max(20.0);

        let total_width = content_width + margin * 2.0;
        let total_height = content_height + margin * 2.0;

        // Calculate zoom to fit (use the smaller scale to ensure everything fits)
        let zoom_x = viewport_width / total_width;
        let zoom_y = viewport_height / total_height;
        let fit_zoom = zoom_x.min(zoom_y);

        // Clamp zoom to reasonable bounds (0.1x to 5x)
        self.zoom = fit_zoom.clamp(0.1, 5.0);

        // Calculate pan to center the content
        let center_px = (min_px + max_px) / 2.0;
        let center_py = (min_py + max_py) / 2.0;

        self.pan = (
            viewport_width / 2.0 - center_px * self.zoom,
            viewport_height / 2.0 - center_py * self.zoom - 15.0,
        );
    }

    /// Calculate the bounding box of all schematic content.
    /// Returns (min_x, min_y, max_x, max_y) in grid coordinates, or None if empty.
    pub fn content_bounds(&self) -> Option<(i32, i32, i32, i32)> {
        if self.components.is_empty() && self.wires.is_empty() && self.junctions.is_empty() {
            return None;
        }

        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        // Include component bounds (with approximate size for the symbol)
        for comp in &self.components {
            let half_w = 30;
            let half_h = 20;
            min_x = min_x.min(comp.pos.x - half_w);
            min_y = min_y.min(comp.pos.y - half_h);
            max_x = max_x.max(comp.pos.x + half_w);
            max_y = max_y.max(comp.pos.y + half_h);
        }

        // Include wire endpoints
        for wire in &self.wires {
            for point in &wire.points {
                min_x = min_x.min(point.x);
                min_y = min_y.min(point.y);
                max_x = max_x.max(point.x);
                max_y = max_y.max(point.y);
            }
        }

        // Include junctions
        for junction in &self.junctions {
            min_x = min_x.min(junction.pos.x);
            min_y = min_y.min(junction.pos.y);
            max_x = max_x.max(junction.pos.x);
            max_y = max_y.max(junction.pos.y);
        }

        Some((min_x, min_y, max_x, max_y))
    }

    // =========================================================================
    // Component Management
    // =========================================================================

    /// Add a component at the given position
    pub fn add_component(&mut self, kind: ComponentType, pos: Point) -> u64 {
        let id = self.next_id();
        let name = self.generate_name(kind);
        let mut component = Component::new(id, kind, pos);
        component.name = name;
        component.rotation = self.preview_rotation;

        // Set default values
        component.value = kind.default_value().to_string();

        self.components.push(component);
        self.is_dirty = true;
        self.bump_topology_version();
        id
    }

    /// Find component at grid position
    pub fn component_at(&self, pos: Point) -> Option<u64> {
        // Check terminals and component bounds
        for comp in &self.components {
            let terminals = comp.terminal_positions();
            for (_, term_pos) in terminals {
                if term_pos == pos {
                    return Some(comp.id);
                }
            }
            // Check if within component bounding box (simplified)
            let dx = (pos.x - comp.pos.x).abs();
            let dy = (pos.y - comp.pos.y).abs();
            if dx <= 2 && dy <= 2 {
                return Some(comp.id);
            }
        }
        None
    }

    /// Rotate selected components
    pub fn rotate_selection(&mut self) {
        for id in &self.selection.components {
            if let Some(c) = self.components.iter_mut().find(|c| c.id == *id) {
                c.rotation = c.rotation.rotate_cw();
            }
        }
        self.is_dirty = true;
        self.bump_topology_version();
    }

    /// Remove selected components and wires
    pub fn delete_selection(&mut self) {
        self.components
            .retain(|c| !self.selection.has_component(c.id));
        self.wires.retain(|w| !self.selection.has_wire(w.id));
        self.selection.clear();
        self.is_dirty = true;
        self.bump_topology_version();
    }

    // =========================================================================
    // Wire Management
    // =========================================================================

    /// Add a wire
    pub fn add_wire(&mut self, points: Vec<Point>) -> Option<u64> {
        if points.len() < 2 {
            return None;
        }
        let id = self.next_id();
        self.wires.push(Wire::new(id, points));
        self.is_dirty = true;
        self.bump_topology_version();
        Some(id)
    }

    /// Find wire at grid position
    pub fn wire_at(&self, pos: Point) -> Option<u64> {
        for wire in &self.wires {
            if wire.contains_point(pos) {
                return Some(wire.id);
            }
        }
        None
    }

    /// Find all wire points at a grid position
    /// Returns (wire_id, point_index) pairs for junction detection
    pub fn wire_points_at(&self, pos: Point) -> Vec<(u64, usize)> {
        let mut result = Vec::new();
        for wire in &self.wires {
            for (idx, point) in wire.points.iter().enumerate() {
                if *point == pos {
                    result.push((wire.id, idx));
                }
            }
        }
        result
    }

    /// Find all wire ENDPOINTS at a grid position
    /// Unlike wire_points_at, this only returns first/last points of wires
    pub fn wire_endpoints_at(&self, pos: Point) -> Vec<(u64, usize)> {
        let mut result = Vec::new();
        for wire in &self.wires {
            if let Some(first) = wire.points.first() {
                if *first == pos {
                    result.push((wire.id, 0));
                }
            }
            if wire.points.len() > 1 {
                if let Some(last) = wire.points.last() {
                    if *last == pos {
                        result.push((wire.id, wire.points.len() - 1));
                    }
                }
            }
        }
        result
    }

    /// Start drawing a wire at position
    pub fn start_wire(&mut self, pos: Point) {
        log::info!("[Wire] start_wire at {:?}", pos);
        self.wire_drawing.clear();
        self.wire_drawing.points.push(pos);
        self.wire_drawing.active = true;
    }

    /// Update the wire preview position (called on mouse move)
    pub fn update_wire_preview(&mut self, pos: Point) {
        if self.wire_drawing.active {
            self.wire_drawing.preview_pos = Some(pos);
        }
    }

    /// Toggle wire routing mode (horizontal-first vs vertical-first)
    pub fn toggle_wire_routing(&mut self) {
        self.wire_drawing.routing_mode = self.wire_drawing.routing_mode.toggle();
    }

    /// Add a point to the current wire using orthogonal routing
    pub fn extend_wire(&mut self, pos: Point) {
        if !self.wire_drawing.active {
            return;
        }

        if let Some(last) = self.wire_drawing.points.last().copied() {
            if last == pos {
                return; // Same point, skip
            }

            // Add corner point for orthogonal routing if needed
            if let Some(corner) = self.wire_drawing.get_route_corner(pos) {
                if corner != last && corner != pos {
                    self.wire_drawing.points.push(corner);
                }
            }

            self.wire_drawing.points.push(pos);
        }
    }

    /// Simplify wire path by removing intermediate points on straight segments
    pub(crate) fn simplify_wire_path(points: Vec<Point>) -> Vec<Point> {
        if points.len() <= 2 {
            return points;
        }

        let mut result = Vec::with_capacity(points.len());
        result.push(points[0]);

        for i in 1..points.len() - 1 {
            let prev = &points[i - 1];
            let curr = &points[i];
            let next = &points[i + 1];

            let all_same_x = prev.x == curr.x && curr.x == next.x;
            let all_same_y = prev.y == curr.y && curr.y == next.y;

            if !all_same_x && !all_same_y {
                result.push(*curr);
            }
        }

        result.push(*points.last().unwrap());
        result
    }

    /// Finish drawing the current wire
    pub fn finish_wire(&mut self) -> Option<u64> {
        if !self.wire_drawing.active {
            return None;
        }

        self.wire_drawing.active = false;
        self.wire_drawing.preview_pos = None;

        let points = std::mem::take(&mut self.wire_drawing.points);
        let simplified = Self::simplify_wire_path(points);

        if simplified.len() < 2 {
            return None;
        }

        // Split the path into individual 2-point wire segments
        let mut last_wire_id = None;
        let mut endpoints_to_check = Vec::new();

        for i in 0..simplified.len() - 1 {
            let segment = vec![simplified[i], simplified[i + 1]];
            endpoints_to_check.push(simplified[i]);
            if i == simplified.len() - 2 {
                endpoints_to_check.push(simplified[i + 1]);
            }
            if let Some(wire_id) = self.add_wire(segment) {
                last_wire_id = Some(wire_id);
            }
        }

        // Auto-create junctions where wire endpoints land on existing wires
        for pt in endpoints_to_check {
            let wires_at_point: Vec<u64> = self
                .wires
                .iter()
                .filter(|w| {
                    let is_just_created =
                        w.points.first() == Some(&pt) || w.points.last() == Some(&pt);
                    w.contains_point(pt) && !is_just_created
                })
                .map(|w| w.id)
                .collect();

            if !wires_at_point.is_empty() && !self.has_junction(pt) {
                self.add_junction(pt);
            }
        }

        last_wire_id
    }

    /// Cancel wire drawing
    pub fn cancel_wire(&mut self) {
        self.wire_drawing.clear();
    }

    // =========================================================================
    // Junction Management
    // =========================================================================

    /// Add an explicit junction at a position
    pub fn add_junction(&mut self, pos: Point) -> u64 {
        // Check if junction already exists at this position
        if let Some(existing) = self.junctions.iter().find(|j| j.pos == pos) {
            return existing.id;
        }

        let id = self.next_id();
        self.junctions.push(Junction::new(id, pos));
        self.is_dirty = true;
        self.bump_topology_version();
        id
    }

    /// Remove a junction by ID
    pub fn remove_junction(&mut self, id: u64) -> bool {
        let len_before = self.junctions.len();
        self.junctions.retain(|j| j.id != id);
        let removed = self.junctions.len() < len_before;
        if removed {
            self.is_dirty = true;
            self.bump_topology_version();
        }
        removed
    }

    /// Find junction at a position
    pub fn junction_at(&self, pos: Point) -> Option<u64> {
        self.junctions.iter().find(|j| j.pos == pos).map(|j| j.id)
    }

    /// Check if a junction exists at a position
    pub fn has_junction(&self, pos: Point) -> bool {
        self.junctions.iter().any(|j| j.pos == pos)
    }

    /// Add a net label at the given position
    pub fn add_net_label(&mut self, pos: Point, name: String) -> u64 {
        let id = self.next_id();
        self.net_labels.push(NetLabel::new(id, pos, name));
        self.is_dirty = true;
        id
    }

    // =========================================================================
    // Clipboard Operations
    // =========================================================================

    /// Copy selected components and wires to clipboard
    pub fn copy_selection(&mut self) {
        if self.selection.is_empty() {
            return;
        }

        let selected_comps: Vec<Component> = self
            .components
            .iter()
            .filter(|c| self.selection.has_component(c.id))
            .cloned()
            .collect();

        let selected_wires: Vec<Wire> = self
            .wires
            .iter()
            .filter(|w| self.selection.has_wire(w.id))
            .cloned()
            .collect();

        self.clipboard = ClipboardData::from_selection(selected_comps, selected_wires);
    }

    /// Check if clipboard has content
    pub fn can_paste(&self) -> bool {
        self.clipboard.has_content()
    }

    /// Paste clipboard contents at the given position
    pub fn paste_at(&mut self, pos: Point) {
        if !self.can_paste() {
            return;
        }

        let clipboard_components = self.clipboard.components.clone();
        let clipboard_wires = self.clipboard.wires.clone();
        let origin = self.clipboard.origin;

        let offset_x = pos.x - origin.x;
        let offset_y = pos.y - origin.y;

        self.selection.clear();

        // Paste components with new IDs
        for comp in clipboard_components {
            let new_id = self.next_id();
            let mut new_comp = comp;
            new_comp.id = new_id;
            new_comp.pos.x += offset_x;
            new_comp.pos.y += offset_y;
            new_comp.name = self.generate_name(new_comp.kind);
            self.components.push(new_comp);
            self.selection.select_component(new_id);
        }

        // Paste wires with new IDs
        for wire in clipboard_wires {
            let new_id = self.next_id();
            let new_points: Vec<Point> = wire
                .points
                .iter()
                .map(|p| Point::new(p.x + offset_x, p.y + offset_y))
                .collect();
            self.wires.push(Wire::new(new_id, new_points));
            self.selection.select_wire(new_id);
        }

        self.is_dirty = true;
        self.bump_topology_version();
    }

    // =========================================================================
    // Wire Connection Management (for rubber-banding)
    // =========================================================================

    /// Find a component terminal at or near a grid position
    pub fn find_terminal_at(&self, pos: Point) -> Option<(u64, String, Point)> {
        for comp in &self.components {
            for (term_name, term_pos) in comp.terminal_positions() {
                let dx = (pos.x - term_pos.x).abs();
                let dy = (pos.y - term_pos.y).abs();
                if dx <= SNAP_DISTANCE && dy <= SNAP_DISTANCE {
                    return Some((comp.id, term_name.to_string(), term_pos));
                }
            }
        }
        None
    }

    /// Rebuild all wire connections based on current positions
    pub fn rebuild_connections(&mut self) {
        self.connections.clear();

        let wire_endpoints: Vec<(u64, Point, usize)> = self
            .wires
            .iter()
            .filter(|w| !w.points.is_empty())
            .flat_map(|w| {
                let mut endpoints = vec![(w.id, w.points[0], 0usize)];
                let end_idx = w.points.len() - 1;
                if end_idx > 0 {
                    endpoints.push((w.id, w.points[end_idx], end_idx));
                }
                endpoints
            })
            .collect();

        for (wire_id, pos, point_index) in wire_endpoints {
            if let Some((comp_id, term_name, _)) = self.find_terminal_at(pos) {
                self.connections.push(WireConnection::new(
                    wire_id,
                    point_index,
                    comp_id,
                    term_name,
                ));
            }
        }
    }

    /// Find all connections for a specific component
    pub fn connections_for_component(&self, component_id: u64) -> Vec<&WireConnection> {
        self.connections
            .iter()
            .filter(|c| c.component_id == component_id)
            .collect()
    }

    /// Move a component and update all attached wire endpoints (rubber-banding)
    pub fn move_component_with_wires(&mut self, component_id: u64, delta: Point) {
        // Get the component's terminal positions BEFORE moving
        let terminals: Vec<Point> = {
            if let Some(comp) = self.components.iter().find(|c| c.id == component_id) {
                comp.terminal_positions()
                    .into_iter()
                    .map(|(_, pos)| pos)
                    .collect()
            } else {
                return;
            }
        };

        // Find ALL wire points that are at ANY terminal position
        let mut wire_updates: Vec<(u64, usize, Point)> = Vec::new();

        for wire in &self.wires {
            for (point_idx, point) in wire.points.iter().enumerate() {
                for term_pos in &terminals {
                    if *point == *term_pos {
                        let new_pos = Point::new(term_pos.x + delta.x, term_pos.y + delta.y);
                        wire_updates.push((wire.id, point_idx, new_pos));
                        break;
                    }
                }
            }
        }

        // Move the component
        if let Some(comp) = self.components.iter_mut().find(|c| c.id == component_id) {
            comp.pos.x += delta.x;
            comp.pos.y += delta.y;
        }

        // Apply wire updates
        for (wire_id, point_idx, new_pos) in wire_updates {
            if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
                if point_idx < wire.points.len() {
                    wire.points[point_idx] = new_pos;
                }
            }
        }

        self.is_dirty = true;
        self.bump_topology_version();
    }

    /// Move all points of a wire by a delta
    pub fn move_wire(&mut self, wire_id: u64, delta: Point) {
        let old_endpoints: Vec<Point> = self
            .wires
            .iter()
            .find(|w| w.id == wire_id)
            .map(|w| {
                let mut eps = Vec::new();
                if let Some(first) = w.points.first() {
                    eps.push(*first);
                }
                if let Some(last) = w.points.last() {
                    eps.push(*last);
                }
                eps
            })
            .unwrap_or_default();

        if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
            for point in &mut wire.points {
                point.x += delta.x;
                point.y += delta.y;
            }
        }

        for old_pt in old_endpoints {
            if let Some(junction) = self.junctions.iter_mut().find(|j| j.pos == old_pt) {
                junction.pos.x += delta.x;
                junction.pos.y += delta.y;
            }
        }

        self.is_dirty = true;
        self.bump_topology_version();
    }

    /// Move all selected components and wires by a delta
    pub fn move_selection(&mut self, delta: Point) {
        let selection = self.selection.clone();

        let selected_wire_ids: std::collections::HashSet<u64> =
            selection.wires.iter().copied().collect();

        // Move all selected components with rubber-banding
        for comp_id in &selection.components {
            let terminals: Vec<Point> = {
                if let Some(comp) = self.components.iter().find(|c| c.id == *comp_id) {
                    comp.terminal_positions()
                        .into_iter()
                        .map(|(_, pos)| pos)
                        .collect()
                } else {
                    continue;
                }
            };

            let mut wire_updates: Vec<(u64, usize, Point)> = Vec::new();
            for wire in &self.wires {
                if selected_wire_ids.contains(&wire.id) {
                    continue;
                }
                for (point_idx, point) in wire.points.iter().enumerate() {
                    for term_pos in &terminals {
                        if *point == *term_pos {
                            let new_pos = Point::new(term_pos.x + delta.x, term_pos.y + delta.y);
                            wire_updates.push((wire.id, point_idx, new_pos));
                            break;
                        }
                    }
                }
            }

            if let Some(comp) = self.components.iter_mut().find(|c| c.id == *comp_id) {
                comp.pos.x += delta.x;
                comp.pos.y += delta.y;
            }

            for (wire_id, point_idx, new_pos) in wire_updates {
                if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
                    if point_idx < wire.points.len() {
                        wire.points[point_idx] = new_pos;
                    }
                }
            }
        }

        // Move all selected wires entirely
        let mut wire_endpoints: Vec<Point> = Vec::new();
        for wire_id in &selection.wires {
            if let Some(wire) = self.wires.iter().find(|w| w.id == *wire_id) {
                if let Some(first) = wire.points.first() {
                    wire_endpoints.push(*first);
                }
                if let Some(last) = wire.points.last() {
                    wire_endpoints.push(*last);
                }
            }
            if let Some(wire) = self.wires.iter_mut().find(|w| w.id == *wire_id) {
                for point in &mut wire.points {
                    point.x += delta.x;
                    point.y += delta.y;
                }
            }
        }

        // Move junctions at selected wire endpoints
        for old_pt in wire_endpoints {
            if let Some(junction) = self.junctions.iter_mut().find(|j| j.pos == old_pt) {
                junction.pos.x += delta.x;
                junction.pos.y += delta.y;
            }
        }

        self.is_dirty = true;
        self.bump_topology_version();
    }

    /// Move all wire points at a junction to a new position
    pub fn move_junction(&mut self, old_pos: Point, new_pos: Point) {
        for wire in &mut self.wires {
            for point in &mut wire.points {
                if *point == old_pos {
                    *point = new_pos;
                }
            }
        }

        if let Some(junction) = self.junctions.iter_mut().find(|j| j.pos == old_pos) {
            junction.pos = new_pos;
        }

        self.is_dirty = true;
    }

    /// Get wire points adjusted for a component drag preview
    pub fn get_wire_preview_points(
        &self,
        wire: &Wire,
        dragging_component_id: Option<u64>,
        delta: Point,
    ) -> Vec<Point> {
        let mut points = wire.points.clone();

        let comp_id = match dragging_component_id {
            Some(id) => id,
            None => return points,
        };

        let terminals: Vec<Point> =
            if let Some(comp) = self.components.iter().find(|c| c.id == comp_id) {
                comp.terminal_positions()
                    .into_iter()
                    .map(|(_, pos)| pos)
                    .collect()
            } else {
                return points;
            };

        for point in points.iter_mut() {
            for term_pos in &terminals {
                if *point == *term_pos {
                    *point = Point::new(term_pos.x + delta.x, term_pos.y + delta.y);
                    break;
                }
            }
        }

        points
    }

    /// Snap wire endpoints to nearby terminals and rebuild connections
    pub fn snap_wire_to_terminals(&mut self, wire_id: u64) {
        let (start_pos, end_pos, end_idx) = {
            if let Some(wire) = self.wires.iter().find(|w| w.id == wire_id) {
                if wire.points.is_empty() {
                    return;
                }
                let end_idx = wire.points.len().saturating_sub(1);
                (
                    Some(wire.points[0]),
                    if end_idx > 0 {
                        Some(wire.points[end_idx])
                    } else {
                        None
                    },
                    end_idx,
                )
            } else {
                return;
            }
        };

        let snap_start = start_pos.and_then(|p| self.find_terminal_at(p).map(|(_, _, pos)| pos));
        let snap_end = end_pos.and_then(|p| self.find_terminal_at(p).map(|(_, _, pos)| pos));

        if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
            if let Some(term_pos) = snap_start {
                wire.points[0] = term_pos;
            }
            if let Some(term_pos) = snap_end {
                if end_idx > 0 {
                    wire.points[end_idx] = term_pos;
                }
            }
        }

        self.rebuild_connections();
    }
}
