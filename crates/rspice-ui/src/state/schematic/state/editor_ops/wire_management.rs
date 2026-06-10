use super::super::*;

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
            if let Some(first) = wire.points.first()
                && *first == pos
            {
                result.push((wire.id, 0));
            }
            if wire.points.len() > 1
                && let Some(last) = wire.points.last()
                && *last == pos
            {
                result.push((wire.id, wire.points.len() - 1));
            }
        }
        result
    }

    /// Find wire vertex at a grid position for dragging
    ///
    /// Returns (wire_id, vertex_index) if there's a wire vertex at this position.
    /// This is used for wire corner dragging - a professional EDA feature.
    /// O(1) via the canvas cache when it's current; linear scan otherwise.
    pub fn wire_vertex_at(&self, pos: Point) -> Option<(u64, usize)> {
        if let Some(cache) = self.canvas_cache() {
            return cache.wire_vertices.get(&pos).copied();
        }
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
    ///
    /// This runs on every pointer-move frame, so it must not scan the
    /// schematic; the canvas cache answers in O(1) when current.
    pub fn is_draggable_wire_point(&self, pos: Point) -> bool {
        if let Some(cache) = self.canvas_cache() {
            return cache.wire_vertices.contains_key(&pos) || cache.junctions.contains(&pos);
        }
        self.wire_vertex_at(pos).is_some() || self.junctions.iter().any(|j| j.pos == pos)
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
            if let Some(corner) = self.wire_drawing.get_route_corner(pos)
                && corner != last
                && corner != pos
            {
                self.wire_drawing.points.push(corner);
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
}
