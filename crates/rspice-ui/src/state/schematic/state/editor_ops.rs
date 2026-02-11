use super::*;

impl SchematicState {
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

    /// Find wire vertex at a grid position for dragging
    ///
    /// Returns (wire_id, vertex_index) if there's a wire vertex at this position.
    /// This is used for wire corner dragging - a professional EDA feature.
    pub fn wire_vertex_at(&self, pos: Point) -> Option<(u64, usize)> {
        for wire in &self.wires {
            for (idx, point) in wire.points.iter().enumerate() {
                if *point == pos {
                    return Some((wire.id, idx));
                }
            }
        }
        None
    }

    /// Check if a position is a draggable wire point
    ///
    /// Returns true if there's either:
    /// - A wire vertex at this position
    /// - A junction marker at this position
    /// - A wire segment that passes through this position
    ///
    /// This is used for detecting draggable points to enable T-junction dragging.
    pub fn is_draggable_wire_point(&self, pos: Point) -> bool {
        // Check for wire vertices
        if self.wire_vertex_at(pos).is_some() {
            return true;
        }

        // Check for junction markers
        if self.junctions.iter().any(|j| j.pos == pos) {
            return true;
        }

        false
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
    ///
    /// Implements professional EDA behavior:
    /// - When a wire endpoint lands on another wire mid-segment, the other wire
    ///   is automatically split at that point (creating a proper vertex)
    /// - This ensures correct rubber-banding: all wires at a T-junction share
    ///   a common endpoint vertex, so moving any wire keeps the junction intact
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

        // Professional EDA behavior: split existing wires at T-junction points
        // This ensures all wires at a junction share a common endpoint vertex
        for pt in &endpoints_to_check {
            self.split_wires_at_t_junction(*pt);
        }

        // Add junction markers where 3+ wire endpoints meet
        self.update_wire_junctions();

        last_wire_id
    }

    /// Split all wires that pass through a point mid-segment (T-junction creation)
    ///
    /// When a wire endpoint lands on another wire's mid-segment, we split the
    /// through wire at that point. This implements professional EDA behavior
    /// where T-junctions are formed by splitting wires, not just by visual overlap.
    ///
    /// This ensures correct rubber-banding: since all wires at the junction
    /// share the same endpoint vertex, moving any attached wire keeps the
    /// junction topology intact.
    pub fn split_wires_at_t_junction(&mut self, point: Point) {
        // Find wires that pass through this point but don't have it as a vertex
        let wires_to_split: Vec<u64> = self
            .wires
            .iter()
            .filter(|w| {
                // Wire passes through point mid-segment (not at a vertex)
                w.contains_point(point) && !w.points.contains(&point)
            })
            .map(|w| w.id)
            .collect();

        // Split each wire at the junction point
        for wire_id in wires_to_split {
            let _ = self.split_wire(wire_id, point);
        }
    }

    /// Cancel wire drawing
    pub fn cancel_wire(&mut self) {
        self.wire_drawing.clear();
    }

    // =========================================================================
    // Wire Operations (Commercial-Grade)
    // =========================================================================

    /// Split a wire into two wires at the given point
    ///
    /// If the point is exactly on the wire (either at a vertex or on a segment),
    /// this will create two new wires: one from the original start to the split point,
    /// and one from the split point to the original end.
    ///
    /// Returns `Some((wire_before_id, wire_after_id))` if successful, `None` otherwise.
    ///
    /// # Arguments
    /// * `wire_id` - The ID of the wire to split
    /// * `at_point` - The point at which to split (must be on the wire)
    pub fn split_wire(&mut self, wire_id: u64, at_point: Point) -> Option<(u64, u64)> {
        // Find the wire
        let wire_idx = self.wires.iter().position(|w| w.id == wire_id)?;
        let wire = &self.wires[wire_idx];

        // Validate that the point is on the wire
        if !wire.contains_point(at_point) {
            return None;
        }

        // Don't split at endpoints - nothing to split
        if wire.start() == Some(at_point) || wire.end() == Some(at_point) {
            return None;
        }

        // Find where to split
        let points = wire.points.clone();

        // Check if split point is at an existing vertex
        let vertex_idx = points.iter().position(|p| *p == at_point);

        let (before_points, after_points) = if let Some(v_idx) = vertex_idx {
            // Split at vertex - both wires share this point
            let before: Vec<Point> = points[..=v_idx].to_vec();
            let after: Vec<Point> = points[v_idx..].to_vec();
            (before, after)
        } else {
            // Point is on a segment, need to find which one and insert it
            let mut before_points = Vec::new();
            let mut after_points = Vec::new();
            let mut found_segment = false;

            for i in 0..points.len() - 1 {
                let seg = WireSegment::new(points[i], points[i + 1]);
                if !found_segment {
                    before_points.push(points[i]);
                    if seg.contains_point(at_point) && points[i] != at_point {
                        before_points.push(at_point);
                        after_points.push(at_point);
                        found_segment = true;
                    }
                }
                if found_segment {
                    after_points.push(points[i + 1]);
                }
            }

            if !found_segment {
                return None;
            }

            (before_points, after_points)
        };

        // Validate both parts are valid wires
        if before_points.len() < 2 || after_points.len() < 2 {
            return None;
        }

        // Remove original wire
        self.wires.remove(wire_idx);

        // Create two new wires
        let id1 = self.next_id();
        let id2 = self.next_id();

        self.wires.push(Wire::new(id1, before_points));
        self.wires.push(Wire::new(id2, after_points));

        self.is_dirty = true;
        self.bump_topology_version();

        Some((id1, id2))
    }

    /// Split a wire at a specific segment, inserting a corner point at the midpoint
    ///
    /// This is useful for creating corners in straight wire runs.
    ///
    /// # Arguments
    /// * `wire_id` - The wire to modify
    /// * `segment_index` - Which segment to split (0 = first segment)
    ///
    /// Returns the modified wire ID if successful
    pub fn split_wire_at_segment(&mut self, wire_id: u64, segment_index: usize) -> Option<u64> {
        let wire = self.wires.iter_mut().find(|w| w.id == wire_id)?;

        if segment_index >= wire.segment_count() {
            return None;
        }

        let segment = wire.segment_at(segment_index)?;
        let midpoint = segment.midpoint();

        // Don't insert if midpoint equals an endpoint (zero-length segment)
        if midpoint == segment.start || midpoint == segment.end {
            return None;
        }

        // Insert the midpoint as a new vertex
        wire.points.insert(segment_index + 1, midpoint);

        self.is_dirty = true;
        self.bump_topology_version();

        Some(wire_id)
    }

    /// Merge two wires that share an endpoint
    ///
    /// The wires must be connected at exactly one endpoint. After merging,
    /// the first wire is removed and the second wire is modified to include
    /// all points from both.
    ///
    /// # Arguments
    /// * `wire_a` - First wire ID
    /// * `wire_b` - Second wire ID
    ///
    /// Returns the ID of the merged wire if successful
    pub fn merge_wires(&mut self, wire_a: u64, wire_b: u64) -> Option<u64> {
        if wire_a == wire_b {
            return None;
        }

        // Find both wires
        let idx_a = self.wires.iter().position(|w| w.id == wire_a)?;
        let idx_b = self.wires.iter().position(|w| w.id == wire_b)?;

        // Check if they share an endpoint
        let (a_start, a_end) = (self.wires[idx_a].start()?, self.wires[idx_a].end()?);
        let (b_start, b_end) = (self.wires[idx_b].start()?, self.wires[idx_b].end()?);

        // Determine connection type and build merged points
        let merged_points: Vec<Point> = if a_end == b_start {
            // A's end connects to B's start: A.start ? A.end/B.start ? B.end
            let mut pts = self.wires[idx_a].points.clone();
            pts.extend(self.wires[idx_b].points.iter().skip(1));
            pts
        } else if a_end == b_end {
            // A's end connects to B's end: A.start ? A.end/B.end ? B.start
            let mut pts = self.wires[idx_a].points.clone();
            pts.extend(self.wires[idx_b].points.iter().rev().skip(1));
            pts
        } else if a_start == b_end {
            // B's end connects to A's start: B.start ? B.end/A.start ? A.end
            let mut pts = self.wires[idx_b].points.clone();
            pts.extend(self.wires[idx_a].points.iter().skip(1));
            pts
        } else if a_start == b_start {
            // A's start connects to B's start: A.end ? A.start/B.start ? B.end
            let mut pts: Vec<Point> = self.wires[idx_a].points.iter().rev().cloned().collect();
            pts.extend(self.wires[idx_b].points.iter().skip(1));
            pts
        } else {
            // Wires don't share an endpoint
            return None;
        };

        // Remove both wires (higher index first to avoid shifting)
        let (remove_first, remove_second) = if idx_a > idx_b {
            (idx_a, idx_b)
        } else {
            (idx_b, idx_a)
        };
        self.wires.remove(remove_first);
        self.wires.remove(remove_second);

        // Create merged wire
        let merged_id = self.next_id();
        self.wires.push(Wire::new(merged_id, merged_points));

        self.is_dirty = true;
        self.bump_topology_version();

        Some(merged_id)
    }

    /// Remove unnecessary intermediate vertices from a wire
    ///
    /// This removes collinear points (points that lie on a straight line
    /// between their neighbors) to simplify the wire path.
    ///
    /// # Arguments
    /// * `wire_id` - The wire to straighten
    pub fn straighten_wire(&mut self, wire_id: u64) {
        if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
            let simplified = Self::simplify_wire_path(wire.points.clone());
            if simplified != wire.points {
                wire.points = simplified;
                self.is_dirty = true;
                self.bump_topology_version();
            }
        }
    }

    /// Optimize all wires by removing collinear intermediate points
    pub fn optimize_all_wires(&mut self) {
        let mut changed = false;
        for wire in &mut self.wires {
            let simplified = Self::simplify_wire_path(wire.points.clone());
            if simplified != wire.points {
                wire.points = simplified;
                changed = true;
            }
        }
        if changed {
            self.is_dirty = true;
            self.bump_topology_version();
        }
    }

    /// Remove degenerate segments from all wires
    ///
    /// This is a cleanup operation that removes:
    /// 1. Zero-length segments (consecutive identical points)
    /// 2. Wires that become invalid after cleanup (< 2 points)
    ///
    /// This is called automatically after wire editing operations to ensure
    /// the schematic maintains valid topology. Matches Cadence Virtuoso behavior.
    ///
    /// # Returns
    /// A tuple of (wires_modified, wires_removed) counts
    pub fn remove_degenerate_segments(&mut self) -> (usize, usize) {
        let mut wires_modified = 0;
        let initial_wire_count = self.wires.len();

        // Phase 1: Remove zero-length segments from each wire
        for wire in &mut self.wires {
            let original_len = wire.points.len();

            // Remove consecutive duplicate points (zero-length segments)
            let mut cleaned = Vec::with_capacity(wire.points.len());
            for point in &wire.points {
                if cleaned.last() != Some(point) {
                    cleaned.push(*point);
                }
            }

            if cleaned.len() != original_len {
                wire.points = cleaned;
                wires_modified += 1;
            }
        }

        // Phase 2: Remove wires that are now invalid (< 2 points)
        let wires_to_remove: Vec<u64> = self
            .wires
            .iter()
            .filter(|w| w.points.len() < 2)
            .map(|w| w.id)
            .collect();

        for wire_id in &wires_to_remove {
            log::info!("Removing zero-length wire id={}", wire_id);
        }

        self.wires.retain(|w| w.points.len() >= 2);

        let wires_removed = initial_wire_count - self.wires.len();

        if wires_modified > 0 || wires_removed > 0 {
            self.is_dirty = true;
            self.bump_topology_version();
        }

        (wires_modified, wires_removed)
    }

    /// Remove degenerate segments from a specific wire
    ///
    /// Returns true if the wire was modified, false if unchanged or not found.
    /// If the wire becomes invalid (< 2 points), it is removed entirely.
    pub fn remove_degenerate_segments_for_wire(&mut self, wire_id: u64) -> bool {
        let wire_idx = match self.wires.iter().position(|w| w.id == wire_id) {
            Some(idx) => idx,
            None => return false,
        };

        let wire = &mut self.wires[wire_idx];
        let original_len = wire.points.len();

        // Remove consecutive duplicate points
        let mut cleaned = Vec::with_capacity(wire.points.len());
        for point in &wire.points {
            if cleaned.last() != Some(point) {
                cleaned.push(*point);
            }
        }

        let was_modified = cleaned.len() != original_len;

        if cleaned.len() < 2 {
            // Wire is now invalid, remove it
            self.wires.remove(wire_idx);
            self.is_dirty = true;
            self.bump_topology_version();
            return true;
        }

        if was_modified {
            self.wires[wire_idx].points = cleaned;
            self.is_dirty = true;
            self.bump_topology_version();
        }

        was_modified
    }

    /// Clean up wire topology after editing operations
    ///
    /// This comprehensive cleanup method should be called after bulk editing:
    /// 1. Removes degenerate (zero-length) segments
    /// 2. Optimizes wire paths (removes collinear points)
    /// 3. Updates junction markers
    ///
    /// This matches commercial EDA tool behavior for maintaining clean topology.
    pub fn cleanup_wire_topology(&mut self) {
        self.remove_degenerate_segments();
        self.optimize_all_wires();
        self.update_wire_junctions();
    }

    /// Delete a wire by ID
    ///
    /// Returns true if a wire was deleted
    pub fn delete_wire(&mut self, wire_id: u64) -> bool {
        let len_before = self.wires.len();
        self.wires.retain(|w| w.id != wire_id);
        let deleted = self.wires.len() < len_before;
        if deleted {
            self.is_dirty = true;
            self.bump_topology_version();
        }
        deleted
    }

    /// Insert a corner vertex into an existing wire at a specific location
    ///
    /// # Arguments
    /// * `wire_id` - The wire to modify
    /// * `at_point` - The point on the wire where to insert the corner
    /// * `corner_offset` - The offset to move the new corner point
    ///
    /// Returns true if successful
    pub fn insert_wire_corner(
        &mut self,
        wire_id: u64,
        at_point: Point,
        corner_offset: Point,
    ) -> bool {
        let wire = match self.wires.iter_mut().find(|w| w.id == wire_id) {
            Some(w) => w,
            None => return false,
        };

        // Find the segment containing the point
        if let Some((seg_idx, _)) = wire.segment_containing_point(at_point) {
            // Insert two new vertices: the original point and the offset corner
            let new_corner = Point::new(at_point.x + corner_offset.x, at_point.y + corner_offset.y);

            // Insert after the segment start
            wire.points.insert(seg_idx + 1, at_point);
            wire.points.insert(seg_idx + 2, new_corner);

            self.is_dirty = true;
            self.bump_topology_version();
            return true;
        }

        false
    }

    /// Move a specific vertex of a wire
    ///
    /// # Arguments
    /// * `wire_id` - The wire to modify
    /// * `vertex_index` - Which vertex to move
    /// * `new_pos` - The new position
    ///
    /// Returns true if successful
    pub fn move_wire_vertex(&mut self, wire_id: u64, vertex_index: usize, new_pos: Point) -> bool {
        if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
            if wire.move_vertex(vertex_index, new_pos) {
                self.is_dirty = true;
                self.bump_topology_version();
                return true;
            }
        }
        false
    }

    /// Move ALL wire vertices at a given position to a new position
    ///
    /// This is the professional EDA behavior for junction/corner dragging:
    /// when you drag a point where multiple wires meet, all of them move together.
    ///
    /// For T-junctions where a wire passes through without a vertex, we first
    /// split that wire at the junction point so it can move with the others.
    ///
    /// # Arguments
    /// * `old_pos` - The current position of the vertices to move
    /// * `new_pos` - The new position
    ///
    /// Returns true if any vertices were moved
    pub fn move_all_vertices_at(&mut self, old_pos: Point, new_pos: Point) -> bool {
        if old_pos == new_pos {
            return false;
        }

        // First, check if this is a junction point where wires might pass through
        // without having a vertex. If so, split those wires first.
        let is_junction = self.junctions.iter().any(|j| j.pos == old_pos);
        if is_junction {
            // Split any wires that pass through this junction point but don't have a vertex there
            self.split_wires_at_t_junction(old_pos);
        }

        let mut moved = false;

        // Move all wire vertices at this position
        for wire in &mut self.wires {
            for point in &mut wire.points {
                if *point == old_pos {
                    *point = new_pos;
                    moved = true;
                }
            }
        }

        // Also move any junction at this position
        for junction in &mut self.junctions {
            if junction.pos == old_pos {
                junction.pos = new_pos;
            }
        }

        if moved {
            self.is_dirty = true;
            self.bump_topology_version();
        }

        moved
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
    ///
    /// In addition to explicitly selected wires, automatically includes
    /// any wires that have both endpoints connected to selected components.
    /// This preserves circuit connectivity when copying/pasting.
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

        // Get all terminal positions for selected components
        let selected_terminals: Vec<Point> = selected_comps
            .iter()
            .flat_map(|c| c.terminal_positions().into_iter().map(|(_, pos)| pos))
            .collect();

        // Find wires that have both endpoints at selected component terminals
        let mut wires_to_copy: Vec<Wire> = Vec::new();

        for wire in &self.wires {
            // Check if explicitly selected
            if self.selection.has_wire(wire.id) {
                wires_to_copy.push(wire.clone());
                continue;
            }

            // Check if both endpoints connect to selected components
            if wire.points.len() >= 2 {
                let start = wire.points[0];
                let end = *wire.points.last().unwrap();

                let start_connected = selected_terminals.contains(&start);
                let end_connected = selected_terminals.contains(&end);

                if start_connected && end_connected {
                    wires_to_copy.push(wire.clone());
                }
            }
        }

        self.clipboard = ClipboardData::from_selection(selected_comps, wires_to_copy);
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

    /// Move all selected components and rubber-band connected wires
    ///
    /// This is the multi-component version of move_component_with_wires.
    /// Wires connected to selected components are stretched to maintain
    /// the connection. Wires that connect two selected components are
    /// moved entirely (not stretched).
    pub fn move_selection_with_rubber_band(&mut self, delta: Point) {
        let selected_components: Vec<u64> = self.selection.components.iter().copied().collect();
        if selected_components.is_empty() && self.selection.wires.is_empty() {
            return;
        }

        // Collect all terminal positions for selected components BEFORE moving
        let mut all_terminals: Vec<(u64, Point)> = Vec::new();
        for comp_id in &selected_components {
            if let Some(comp) = self.components.iter().find(|c| c.id == *comp_id) {
                for (_, pos) in comp.terminal_positions() {
                    all_terminals.push((*comp_id, pos));
                }
            }
        }

        // Find wires that should be stretched (one end connected to selection)
        // vs moved entirely (both ends connected to selection)
        let mut wire_updates: Vec<(u64, usize, Point)> = Vec::new();
        let mut wires_to_move: Vec<u64> = Vec::new();

        for wire in &self.wires {
            let start = wire.points.first().copied();
            let end = wire.points.last().copied();

            // Check if endpoints connect to selected components
            let start_connected = start.map_or(false, |p| {
                all_terminals.iter().any(|(_, term_pos)| *term_pos == p)
            });
            let end_connected = end.map_or(false, |p| {
                all_terminals.iter().any(|(_, term_pos)| *term_pos == p)
            });

            if start_connected && end_connected {
                // Both ends connected to selection - move entire wire
                wires_to_move.push(wire.id);
            } else {
                // Stretch endpoints that are connected
                for (point_idx, point) in wire.points.iter().enumerate() {
                    for (_, term_pos) in &all_terminals {
                        if *point == *term_pos {
                            let new_pos = Point::new(point.x + delta.x, point.y + delta.y);
                            wire_updates.push((wire.id, point_idx, new_pos));
                            break;
                        }
                    }
                }
            }
        }

        // Move selected components
        for comp_id in &selected_components {
            if let Some(comp) = self.components.iter_mut().find(|c| c.id == *comp_id) {
                comp.pos.x += delta.x;
                comp.pos.y += delta.y;
            }
        }

        // Move selected wires (from selection, not from rubber-banding)
        for wire_id in self.selection.wires.iter() {
            if let Some(wire) = self.wires.iter_mut().find(|w| w.id == *wire_id) {
                for point in &mut wire.points {
                    point.x += delta.x;
                    point.y += delta.y;
                }
            }
        }

        // Move wires that have both ends connected to selection
        for wire_id in wires_to_move {
            // Skip if already in selection (already moved above)
            if self.selection.wires.contains(&wire_id) {
                continue;
            }
            if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
                for point in &mut wire.points {
                    point.x += delta.x;
                    point.y += delta.y;
                }
            }
        }

        // Apply stretch updates for partially connected wires
        for (wire_id, point_idx, new_pos) in wire_updates {
            // Skip if wire was already moved entirely
            if self.selection.wires.contains(&wire_id) {
                continue;
            }
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
