use super::super::*;

impl SchematicState {
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
}
