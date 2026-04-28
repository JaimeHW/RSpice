//! Wire Struct
//!
//! Core Wire type representing electrical connections in the schematic.
//! Each wire is a polyline (sequence of connected points).

use super::super::point::Point;
use super::segment::{WireHitResult, WireSegment};
use serde::{Deserialize, Serialize};

// =============================================================================
// Wire
// =============================================================================

/// A wire segment connecting two or more points
///
/// Wires represent electrical connections in the schematic.
/// Each wire is a polyline (sequence of connected points).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wire {
    /// Unique identifier within the schematic
    pub id: u64,

    /// Wire path (sequence of connected points)
    ///
    /// Points are in grid coordinates. The wire connects
    /// point[0] → point[1] → ... → point[n-1].
    pub points: Vec<Point>,
}

impl Wire {
    /// Create a new wire with the given points
    pub fn new(id: u64, points: Vec<Point>) -> Self {
        Self { id, points }
    }

    /// Create a two-point wire (single segment)
    pub fn segment(id: u64, start: Point, end: Point) -> Self {
        Self {
            id,
            points: vec![start, end],
        }
    }

    /// Check if wire contains a point (on any segment)
    pub fn contains_point(&self, p: Point) -> bool {
        // Check vertices
        if self.points.contains(&p) {
            return true;
        }

        // Check segments
        for segment in self.points.windows(2) {
            if Self::point_on_segment(p, segment[0], segment[1]) {
                return true;
            }
        }
        false
    }

    /// Check if point lies on a horizontal or vertical segment
    fn point_on_segment(p: Point, a: Point, b: Point) -> bool {
        // Horizontal segment
        if a.y == b.y && p.y == a.y {
            let (min_x, max_x) = if a.x < b.x { (a.x, b.x) } else { (b.x, a.x) };
            return p.x >= min_x && p.x <= max_x;
        }
        // Vertical segment
        if a.x == b.x && p.x == a.x {
            let (min_y, max_y) = if a.y < b.y { (a.y, b.y) } else { (b.y, a.y) };
            return p.y >= min_y && p.y <= max_y;
        }
        false
    }

    /// Get the start point (first point)
    pub fn start(&self) -> Option<Point> {
        self.points.first().copied()
    }

    /// Get the end point (last point)
    pub fn end(&self) -> Option<Point> {
        self.points.last().copied()
    }

    /// Get both endpoints
    pub fn endpoints(&self) -> (Option<Point>, Option<Point>) {
        (self.start(), self.end())
    }

    /// Check if this wire connects to another at any endpoint
    pub fn connects_to(&self, other: &Wire) -> bool {
        let self_endpoints = [self.start(), self.end()];
        let other_endpoints = [other.start(), other.end()];

        for se in &self_endpoints {
            for oe in &other_endpoints {
                if let (Some(s), Some(o)) = (se, oe)
                    && s == o
                {
                    return true;
                }
            }
        }
        false
    }

    /// Get the total length of the wire in grid units
    pub fn length(&self) -> i32 {
        let mut total = 0;
        for segment in self.points.windows(2) {
            total += segment[0].manhattan_distance(segment[1]);
        }
        total
    }

    /// Check if the wire is empty (no points or single point)
    pub fn is_empty(&self) -> bool {
        self.points.len() < 2
    }

    /// Get number of segments in the wire
    pub fn segment_count(&self) -> usize {
        if self.points.len() < 2 {
            0
        } else {
            self.points.len() - 1
        }
    }

    // =========================================================================
    // Segment-Level Operations (Robust)
    // =========================================================================

    /// Get an iterator over all segments of the wire
    pub fn segments(&self) -> impl Iterator<Item = WireSegment> + '_ {
        self.points.windows(2).map(|w| WireSegment::new(w[0], w[1]))
    }

    /// Get a specific segment by index
    /// Returns None if index is out of bounds
    pub fn segment_at(&self, index: usize) -> Option<WireSegment> {
        if index < self.segment_count() {
            Some(WireSegment::new(self.points[index], self.points[index + 1]))
        } else {
            None
        }
    }

    /// Find the segment containing a point and its index
    /// Returns (segment_index, segment) if found
    pub fn segment_containing_point(&self, p: Point) -> Option<(usize, WireSegment)> {
        for (idx, segment) in self.segments().enumerate() {
            if segment.contains_point(p) {
                return Some((idx, segment));
            }
        }
        None
    }

    /// Find the segment closest to a point within tolerance
    /// Returns (segment_index, segment, distance) if within tolerance
    pub fn segment_near_point(
        &self,
        p: Point,
        tolerance: f64,
    ) -> Option<(usize, WireSegment, f64)> {
        let mut best: Option<(usize, WireSegment, f64)> = None;

        for (idx, segment) in self.segments().enumerate() {
            let dist = segment.distance_to_point(p);
            if dist <= tolerance {
                match &best {
                    None => best = Some((idx, segment, dist)),
                    Some((_, _, best_dist)) if dist < *best_dist => {
                        best = Some((idx, segment, dist));
                    }
                    _ => {}
                }
            }
        }

        best
    }

    // =========================================================================
    // Vertex Operations (for corner manipulation)
    // =========================================================================

    /// Get a vertex by index
    pub fn vertex_at(&self, index: usize) -> Option<Point> {
        self.points.get(index).copied()
    }

    /// Get the number of vertices in the wire
    #[inline]
    pub fn vertex_count(&self) -> usize {
        self.points.len()
    }

    /// Insert a vertex at the specified index
    /// The index must be in range [0, vertex_count]
    pub fn insert_vertex(&mut self, index: usize, point: Point) {
        if index <= self.points.len() {
            self.points.insert(index, point);
        }
    }

    /// Remove a vertex at the specified index
    /// Returns the removed point, or None if index is invalid
    /// Will not remove if it would make the wire invalid (< 2 points)
    pub fn remove_vertex(&mut self, index: usize) -> Option<Point> {
        if index < self.points.len() && self.points.len() > 2 {
            Some(self.points.remove(index))
        } else {
            None
        }
    }

    /// Move a vertex to a new position
    /// Returns true if successful
    pub fn move_vertex(&mut self, index: usize, new_pos: Point) -> bool {
        if index < self.points.len() {
            self.points[index] = new_pos;
            true
        } else {
            false
        }
    }

    /// Check if a vertex is near a point within tolerance
    /// Returns (vertex_index, vertex_point, distance) if found
    pub fn vertex_near(&self, p: Point, tolerance: i32) -> Option<(usize, Point, f64)> {
        let tolerance_f = tolerance as f64;
        let mut best: Option<(usize, Point, f64)> = None;

        for (idx, vertex) in self.points.iter().enumerate() {
            let dx = (p.x - vertex.x) as f64;
            let dy = (p.y - vertex.y) as f64;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist <= tolerance_f {
                match &best {
                    None => best = Some((idx, *vertex, dist)),
                    Some((_, _, best_dist)) if dist < *best_dist => {
                        best = Some((idx, *vertex, dist));
                    }
                    _ => {}
                }
            }
        }

        best
    }

    // =========================================================================
    // Bounding Box and Geometry
    // =========================================================================

    /// Get the axis-aligned bounding box of the wire
    /// Returns (min_point, max_point) or None if wire is empty
    pub fn bounding_box(&self) -> Option<(Point, Point)> {
        if self.points.is_empty() {
            return None;
        }

        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        for p in &self.points {
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
        }

        Some((Point::new(min_x, min_y), Point::new(max_x, max_y)))
    }

    /// Check if all segments of this wire are orthogonal
    pub fn is_orthogonal(&self) -> bool {
        self.segments().all(|s| s.is_orthogonal())
    }

    /// Get the total Euclidean length of the wire
    pub fn euclidean_length(&self) -> f64 {
        self.segments().map(|s| s.euclidean_length()).sum()
    }

    /// Find the closest point on the wire to a given position
    pub fn closest_point(&self, p: Point) -> Option<(Point, f64)> {
        let mut best: Option<(Point, f64)> = None;

        for segment in self.segments() {
            let closest = segment.closest_point(p);
            let dist = segment.distance_to_point(p);

            match &best {
                None => best = Some((closest, dist)),
                Some((_, best_dist)) if dist < *best_dist => {
                    best = Some((closest, dist));
                }
                _ => {}
            }
        }

        best
    }

    // =========================================================================
    // Advanced Hit Testing
    // =========================================================================

    /// Perform hit testing on the wire with tolerance
    ///
    /// Returns a WireHitResult indicating what was hit:
    /// - Vertex: A corner point was hit (for corner manipulation)
    /// - Segment: A wire segment was hit (for segment selection/manipulation)
    /// - None: Nothing within tolerance
    ///
    /// Vertices have priority over segments at the same location
    pub fn hit_test(&self, pos: Point, tolerance: f64) -> WireHitResult {
        // First check vertices (higher priority for manipulation)
        if let Some((idx, vertex, dist)) = self.vertex_near(pos, tolerance as i32) {
            return WireHitResult::Vertex {
                index: idx,
                point: vertex,
                distance: dist,
            };
        }

        // Then check segments
        if let Some((idx, segment, dist)) = self.segment_near_point(pos, tolerance) {
            return WireHitResult::Segment {
                index: idx,
                closest_point: segment.closest_point(pos),
                distance: dist,
                segment,
            };
        }

        WireHitResult::None
    }

    /// Hit test with separate tolerances for vertices and segments
    /// This allows for larger vertex hit zones for easier corner manipulation
    pub fn hit_test_with_tolerances(
        &self,
        pos: Point,
        vertex_tolerance: f64,
        segment_tolerance: f64,
    ) -> WireHitResult {
        // First check vertices with vertex tolerance
        if let Some((idx, vertex, dist)) = self.vertex_near(pos, vertex_tolerance as i32) {
            return WireHitResult::Vertex {
                index: idx,
                point: vertex,
                distance: dist,
            };
        }

        // Then check segments with segment tolerance
        if let Some((idx, segment, dist)) = self.segment_near_point(pos, segment_tolerance) {
            return WireHitResult::Segment {
                index: idx,
                closest_point: segment.closest_point(pos),
                distance: dist,
                segment,
            };
        }

        WireHitResult::None
    }

    /// Check if this wire intersects with a segment
    /// Returns all intersection points
    pub fn intersections_with_segment(&self, seg: &WireSegment) -> Vec<Point> {
        let mut intersections = Vec::new();
        for wire_seg in self.segments() {
            if let Some(p) = wire_seg.intersection(seg) {
                // Avoid duplicates at vertices
                if !intersections.contains(&p) {
                    intersections.push(p);
                }
            }
        }
        intersections
    }

    /// Check if this wire intersects with another wire
    /// Returns all intersection points
    pub fn intersections_with_wire(&self, other: &Wire) -> Vec<Point> {
        let mut intersections = Vec::new();
        for seg in other.segments() {
            for p in self.intersections_with_segment(&seg) {
                if !intersections.contains(&p) {
                    intersections.push(p);
                }
            }
        }
        intersections
    }

    // =========================================================================
    // Wire Manipulation
    // =========================================================================

    /// Translate the entire wire by a delta
    pub fn translate(&mut self, dx: i32, dy: i32) {
        for point in &mut self.points {
            point.x += dx;
            point.y += dy;
        }
    }

    /// Create a translated copy of this wire
    pub fn translated(&self, dx: i32, dy: i32) -> Self {
        Self {
            id: self.id,
            points: self
                .points
                .iter()
                .map(|p| Point::new(p.x + dx, p.y + dy))
                .collect(),
        }
    }

    /// Split the wire at a point
    ///
    /// Inserts a new vertex at the given point if it lies on the wire.
    /// Returns true if the split was successful.
    pub fn split_at(&mut self, p: Point) -> bool {
        if let Some((idx, _segment)) = self.segment_containing_point(p) {
            // Don't split at existing vertices
            if self.points[idx] != p && self.points[idx + 1] != p {
                self.points.insert(idx + 1, p);
                return true;
            }
        }
        false
    }

    /// Simplify the wire by removing redundant vertices
    ///
    /// Removes vertices that are collinear with their neighbors.
    pub fn simplify(&mut self) {
        if self.points.len() <= 2 {
            return;
        }

        let mut result = Vec::with_capacity(self.points.len());
        result.push(self.points[0]);

        for window in self.points.windows(3) {
            let (p1, p2, p3) = (window[0], window[1], window[2]);

            // Check if p2 is collinear
            let collinear_h = p1.y == p2.y && p2.y == p3.y;
            let collinear_v = p1.x == p2.x && p2.x == p3.x;

            if !collinear_h && !collinear_v {
                result.push(p2);
            }
        }

        result.push(*self.points.last().unwrap());
        self.points = result;
    }
}

// =============================================================================
// Tests
// =============================================================================
