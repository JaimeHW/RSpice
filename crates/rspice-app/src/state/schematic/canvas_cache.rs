//! App integration for the editor's topology-versioned canvas geometry.

use super::junction_candidates::{collect_junction_candidates, nearest_junction_candidate};
use super::point::Point;
use super::state::SchematicState;
use rspice_schematic_editor::session::canvas_cache::CanvasCache;

impl SchematicState {
    /// Rebuild the canvas cache if the topology changed since it was last
    /// built. Call once per frame before painting or hover hit-tests.
    pub fn ensure_canvas_cache(&mut self) {
        let version = self.topology_version();
        self.session
            .editor
            .ensure_canvas_cache(self.design.document(), version);
    }

    /// The canvas cache if it is current for this topology version.
    pub fn canvas_cache(&self) -> Option<&CanvasCache> {
        self.session.editor.canvas_cache(self.topology_version())
    }

    /// Return the nearest valid explicit-junction target within `radius`.
    /// The frame cache serves the hot path; the fallback keeps the first
    /// interactive frame correct before derived geometry has been rebuilt.
    pub fn nearest_junction_candidate(&self, pos: Point, radius: i32) -> Option<Point> {
        let fallback;
        let candidates = if let Some(cache) = self.canvas_cache() {
            cache.junction_candidates.as_slice()
        } else {
            fallback = collect_junction_candidates(&self.design.document().wires);
            fallback.as_slice()
        };
        nearest_junction_candidate(candidates, pos, radius)
    }
}

#[cfg(test)]
mod tests {
    use super::super::state::SchematicState;
    use super::super::wire::Wire;
    use super::Point;

    /// The cached hit-test answers must match the linear-scan fallback,
    /// and topology bumps must invalidate.
    #[test]
    fn cache_matches_linear_scan_and_invalidates() {
        let mut state = SchematicState::default();
        state
            .design
            .document_mut_for_test()
            .wires
            .push(Wire::new(1, vec![Point::new(0, 0), Point::new(40, 0)]));
        state
            .design
            .document_mut_for_test()
            .wires
            .push(Wire::new(2, vec![Point::new(40, 0), Point::new(40, 40)]));
        state.bump_topology_version();

        // Fallback answers (cache not built yet).
        assert!(state.canvas_cache().is_none());
        assert_eq!(state.wire_vertex_at(Point::new(40, 0)), Some((1, 1)));
        assert!(state.is_draggable_wire_point(Point::new(40, 40)));

        // Cached answers are identical.
        state.ensure_canvas_cache();
        assert_eq!(state.wire_vertex_at(Point::new(40, 0)), Some((1, 1)));
        assert!(state.is_draggable_wire_point(Point::new(40, 40)));
        assert!(!state.is_draggable_wire_point(Point::new(99, 99)));
        let cache = state.canvas_cache().expect("cache fresh");
        assert_eq!(
            cache.wire_bounds[1],
            (Point::new(40, 0), Point::new(40, 40))
        );

        // A topology bump invalidates; rebuilding picks up the new bounds.
        state.design.document_mut_for_test().wires[0].points[0] = Point::new(-20, 0);
        state.bump_topology_version();
        assert!(state.canvas_cache().is_none());
        state.ensure_canvas_cache();
        let cache = state.canvas_cache().expect("cache rebuilt");
        assert_eq!(
            cache.wire_bounds[0],
            (Point::new(-20, 0), Point::new(40, 0))
        );
    }

    #[test]
    fn junction_candidates_are_deduplicated_cached_and_nearest() {
        let mut state = SchematicState::default();
        state.design.document_mut_for_test().wires = vec![
            Wire::new(1, vec![Point::new(0, 20), Point::new(40, 20)]),
            Wire::new(2, vec![Point::new(20, 0), Point::new(20, 40)]),
            Wire::new(3, vec![Point::new(0, 0), Point::new(40, 40)]),
        ];
        state.bump_topology_version();

        assert_eq!(
            state.nearest_junction_candidate(Point::new(19, 21), 4),
            Some(Point::new(20, 20))
        );
        state.ensure_canvas_cache();
        let cache = state.canvas_cache().expect("cache fresh");
        assert_eq!(cache.junction_candidates, vec![Point::new(20, 20)]);
        assert_eq!(
            state.nearest_junction_candidate(Point::new(19, 21), 4),
            Some(Point::new(20, 20))
        );
        assert_eq!(
            state.nearest_junction_candidate(Point::new(100, 100), 4),
            None
        );
    }

    #[test]
    fn endpoint_and_t_contacts_are_not_explicit_junction_targets() {
        let mut state = SchematicState::default();
        state.design.document_mut_for_test().wires = vec![
            Wire::new(1, vec![Point::new(0, 20), Point::new(40, 20)]),
            Wire::new(2, vec![Point::new(20, 20), Point::new(20, 40)]),
        ];
        state.bump_topology_version();

        assert_eq!(
            state.nearest_junction_candidate(Point::new(20, 20), 4),
            None
        );
        state.ensure_canvas_cache();
        assert!(
            state
                .canvas_cache()
                .expect("cache fresh")
                .junction_candidates
                .is_empty()
        );
    }

    #[test]
    fn large_schematic_viewport_query_stays_spatial_and_keeps_long_wires() {
        let mut state = SchematicState::default();
        state.design.document_mut_for_test().wires = (0_u64..10_000)
            .map(|index| {
                let x = i32::try_from(index).expect("bounded fixture") * 512;
                Wire::new(index + 1, vec![Point::new(x, 0), Point::new(x, 40)])
            })
            .collect();
        state.design.document_mut_for_test().wires.push(Wire::new(
            20_000,
            vec![Point::new(-2_000_000, 20), Point::new(7_000_000, 20)],
        ));
        state.bump_topology_version();
        state.ensure_canvas_cache();

        let cache = state.canvas_cache().expect("cache fresh");
        let indices = cache.wire_indices_in_world_rect(5_100.0, -20.0, 5_250.0, 80.0);
        assert!(
            indices.contains(&10),
            "the nearby short wire is a candidate"
        );
        assert!(
            indices.contains(&10_000),
            "an oversized conductor remains a candidate without unbounded bin replication"
        );
        assert!(
            indices.len() <= 4,
            "a small viewport must not degrade to a 10,001-wire scan"
        );
    }
}
