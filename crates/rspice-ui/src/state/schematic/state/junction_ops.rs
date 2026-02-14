use super::*;

impl SchematicState {
    // =========================================================================
    // Automatic Junction Detection System (Commercial-Grade)
    // =========================================================================

    /// Find all intersection points between wires
    ///
    /// This detects where wires cross or connect, which is essential for
    /// automatic junction placement. Like Cadence, we check both:
    /// 1. Wire-to-wire segment intersections
    /// 2. Wire endpoints touching other wires
    pub fn find_wire_intersections(&self) -> Vec<(Point, Vec<u64>)> {
        use std::collections::HashMap;

        let mut intersection_map: HashMap<Point, Vec<u64>> = HashMap::new();

        // Phase 1: Find all wire endpoint connections
        for wire in &self.wires {
            if let Some(start) = wire.start() {
                intersection_map.entry(start).or_default().push(wire.id);
            }
            if let Some(end) = wire.end() {
                if wire.points.len() > 1 {
                    intersection_map.entry(end).or_default().push(wire.id);
                }
            }
        }

        // Phase 2: Find wire-to-wire segment intersections
        let wires: Vec<_> = self.wires.iter().collect();
        for i in 0..wires.len() {
            for j in (i + 1)..wires.len() {
                let wire_a = wires[i];
                let wire_b = wires[j];

                // Get all intersection points between these two wires
                let intersections = wire_a.intersections_with_wire(wire_b);
                for point in intersections {
                    let entry = intersection_map.entry(point).or_default();
                    if !entry.contains(&wire_a.id) {
                        entry.push(wire_a.id);
                    }
                    if !entry.contains(&wire_b.id) {
                        entry.push(wire_b.id);
                    }
                }
            }
        }

        // Phase 3: Check if any wire passes through another wire's vertex
        for wire in &self.wires {
            for other_wire in &self.wires {
                if wire.id == other_wire.id {
                    continue;
                }
                // Check if other_wire passes through any vertex of wire
                for vertex in &wire.points {
                    if other_wire.contains_point(*vertex) {
                        let entry = intersection_map.entry(*vertex).or_default();
                        if !entry.contains(&wire.id) {
                            entry.push(wire.id);
                        }
                        if !entry.contains(&other_wire.id) {
                            entry.push(other_wire.id);
                        }
                    }
                }
            }
        }

        // Convert to sorted vector - only include points with 2+ wires (actual connections)
        let mut result: Vec<_> = intersection_map
            .into_iter()
            .filter(|(_, wire_ids)| wire_ids.len() >= 2)
            .collect();
        result.sort_by(|a, b| a.0.x.cmp(&b.0.x).then_with(|| a.0.y.cmp(&b.0.y)));
        result
    }

    /// Detect junction points that need visual markers
    ///
    /// A junction needs a marker (dot) when:
    /// - 3+ wire segments meet at a point (T-junction or cross)
    /// - A wire endpoint meets another wire mid-segment (T-junction)
    ///
    /// This counts SEGMENTS meeting at each point, not wire IDs:
    /// - A wire endpoint contributes 1 segment
    /// - A wire passing through mid-segment contributes 2 segments
    pub fn detect_junction_points(&self) -> Vec<Point> {
        use std::collections::HashMap;

        let mut segment_counts: HashMap<Point, usize> = HashMap::new();

        for wire in &self.wires {
            // Check each point on the wire
            for (i, point) in wire.points.iter().enumerate() {
                let is_endpoint = i == 0 || i == wire.points.len() - 1;
                let count = if is_endpoint { 1 } else { 2 }; // Mid-point = 2 segments
                *segment_counts.entry(*point).or_insert(0) += count;
            }

            // Also check if wire passes through any point on another wire
            for other_wire in &self.wires {
                if wire.id == other_wire.id {
                    continue;
                }
                // Check if other_wire vertices lie on this wire's segments
                for vertex in &other_wire.points {
                    if wire.contains_point(*vertex) {
                        // Check if this is already counted as a wire vertex
                        let is_vertex_of_wire = wire.points.contains(vertex);
                        if !is_vertex_of_wire {
                            // Wire passes through this point mid-segment = 2 segments
                            *segment_counts.entry(*vertex).or_insert(0) += 2;
                        }
                    }
                }
            }
        }

        // Return points where 3+ segments meet (T-junction or more)
        segment_counts
            .into_iter()
            .filter(|(_, count)| *count >= 3)
            .map(|(point, _)| point)
            .collect()
    }

    /// Classify the type of junction at a given point
    pub fn classify_junction_type(&self, pos: Point) -> super::super::wire::JunctionType {
        let wire_count = self.wires.iter().filter(|w| w.contains_point(pos)).count();
        super::super::wire::JunctionType::from_wire_count(wire_count)
    }

    /// Automatically place junctions at all detected intersection points
    ///
    /// This is the main entry point for automatic junction management.
    /// Call this after wire operations to maintain junction consistency.
    pub fn auto_place_junctions(&mut self) {
        let junction_points = self.detect_junction_points();
        let mut changes = false;

        // Add junctions at detected points that don't have one
        for point in &junction_points {
            let has_junction = self.junctions.iter().any(|j| j.pos == *point);
            if !has_junction {
                self.add_junction(*point);
                changes = true;
            }
        }

        // Remove junctions that are no longer at intersection points
        let len_before = self.junctions.len();
        self.junctions.retain(|j| junction_points.contains(&j.pos));
        if self.junctions.len() != len_before {
            changes = true;
        }

        if changes {
            self.is_dirty = true;
            self.bump_topology_version();
        }
    }

    /// Remove orphaned junctions that no longer have wire connections
    pub fn remove_orphan_junctions(&mut self) -> usize {
        let initial_count = self.junctions.len();

        self.junctions.retain(|junction| {
            // Keep junction if any wire passes through it
            self.wires.iter().any(|w| w.contains_point(junction.pos))
        });

        let removed = initial_count - self.junctions.len();
        if removed > 0 {
            self.is_dirty = true;
            self.bump_topology_version();
        }
        removed
    }

    /// Update junction markers based on current wire topology
    ///
    /// This is a more comprehensive update that:
    /// 1. Removes orphan junctions
    /// 2. Places new junctions where needed
    /// 3. Updates junction types
    pub fn update_wire_junctions(&mut self) {
        self.remove_orphan_junctions();
        self.auto_place_junctions();
    }

    /// Find all points where wires could potentially be split
    /// (where they cross other wires without connecting)
    pub fn find_potential_splits(&self) -> Vec<(u64, Point)> {
        let mut splits = Vec::new();

        for wire in &self.wires {
            for other_wire in &self.wires {
                if wire.id == other_wire.id {
                    continue;
                }

                let intersections = wire.intersections_with_wire(other_wire);
                for point in intersections {
                    // Check if this intersection point is already a vertex on wire
                    let is_vertex = wire.points.contains(&point);
                    if !is_vertex {
                        splits.push((wire.id, point));
                    }
                }
            }
        }

        // Deduplicate
        splits.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.x.cmp(&b.1.x)));
        splits.dedup();
        splits
    }

    /// Create T-junctions by splitting wires at intersection points
    ///
    /// When a wire endpoint touches another wire mid-segment,
    /// this will split the second wire and create proper junction.
    pub fn create_t_junctions_from_endpoints(&mut self) {
        // Find all points where one wire ends on another wire's segment
        let mut splits_needed: Vec<(u64, Point)> = Vec::new();

        for wire in &self.wires {
            let endpoints = [wire.start(), wire.end()];
            for endpoint in endpoints.into_iter().flatten() {
                for other_wire in &self.wires {
                    if other_wire.id == wire.id {
                        continue;
                    }

                    // Check if endpoint is on other_wire but not at a vertex
                    if other_wire.contains_point(endpoint) {
                        let is_at_vertex = other_wire.points.contains(&endpoint);
                        if !is_at_vertex {
                            splits_needed.push((other_wire.id, endpoint));
                        }
                    }
                }
            }
        }

        // Perform splits and add junctions
        let has_splits = !splits_needed.is_empty();
        for (wire_id, point) in splits_needed {
            // Insert vertex at the intersection point
            if let Some(wire) = self.wires.iter_mut().find(|w| w.id == wire_id) {
                if let Some((segment_idx, _segment)) = wire.segment_containing_point(point) {
                    wire.insert_vertex(segment_idx + 1, point);
                }
            }

            // Ensure junction exists at this point
            let has_junction = self.junctions.iter().any(|j| j.pos == point);
            if !has_junction {
                self.add_junction(point);
            }
        }

        if has_splits {
            self.is_dirty = true;
            self.bump_topology_version();
        }
    }

    /// Count how many wire segments connect at a given point
    pub fn count_connections_at(&self, pos: Point) -> usize {
        let mut count = 0;
        for wire in &self.wires {
            // Count endpoints
            if wire.start() == Some(pos) {
                count += 1;
            }
            if wire.end() == Some(pos) && wire.points.len() > 1 {
                count += 1;
            }

            // Count mid-wire vertices (each vertex connects 2 segments)
            for (i, point) in wire.points.iter().enumerate() {
                if *point == pos && i > 0 && i < wire.points.len() - 1 {
                    count += 2; // This vertex connects two segments
                }
            }
        }
        count
    }
}
