//! Frame-Coherent Canvas Cache
//!
//! Derived geometry the schematic canvas needs every frame: wire bounding
//! boxes for viewport culling and a vertex/junction index for O(1) hover
//! hit-tests. Rebuilt lazily when `topology_version` advances; never
//! persisted, and deliberately reset (not copied) on clone — a clone
//! rebuilds its own on first use.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, PoisonError};

use rspice_design::schematic::net_label::Junction;
use rspice_design::schematic::wire::Wire;
use rspice_design_model::Point;
use rspice_design_model::design_management::SheetId;

use crate::view::drawing::{ConductorInk, chain_conductors};

const SPATIAL_CELL: i32 = 256;
const MAX_CELLS_PER_WIRE: i64 = 4_096;

/// Which conductors a set of junction candidates was found among: the
/// active sheet and the catalog revision that records which objects it owns,
/// or `None` for a drawing with no sheet catalog.
pub type JunctionCandidateScope = Option<(Option<SheetId>, u64)>;

/// Cached per-frame canvas geometry, valid for one topology version.
#[derive(Debug, Default)]
pub struct CanvasCache {
    /// Topology version this cache was built for; `None` = never built.
    version: Option<u64>,

    /// Wire AABBs as (min, max), parallel to the schematic's wire list.
    pub wire_bounds: Vec<(Point, Point)>,

    /// Spatial bins of wire-list indices used to avoid scanning an entire
    /// large design for viewport culling and point hit-testing.
    wire_cells: HashMap<(i32, i32), Vec<usize>>,

    /// Very long conductors are cheaper to test directly than to replicate
    /// into an unbounded number of spatial bins.
    oversized_wires: Vec<usize>,

    /// First (wire id, vertex index) at each grid point, in wire order —
    /// matching the linear-scan semantics of `wire_vertex_at`.
    pub wire_vertices: HashMap<Point, (u64, usize)>,

    /// Junction marker positions.
    pub junctions: HashSet<Point>,

    /// Where the Junction tool may place a junction: crossings of the active
    /// sheet's conductors, found once per topology and sheet scope rather
    /// than once per hover frame. Behind a lock because the canvas reads the
    /// cache through a shared borrow.
    junction_candidates: Mutex<Option<(JunctionCandidateScope, Arc<Vec<Point>>)>>,

    /// The conductor chains last painted, with what they were chained from.
    conductor_chains: Mutex<Option<HeldConductorChains>>,
}

/// One frame's conductors to paint: per ink, in paint order, the indices of
/// the visible wires drawn in it.
pub type ConductorGroups = Vec<(ConductorInk, Vec<usize>)>;

/// Each ink's chained conductor paths, in paint order.
pub type ConductorChains = Arc<Vec<(ConductorInk, Vec<Vec<Point>>)>>;

/// Chains and everything they were chained from: the grouping, and every
/// grouped wire's points by content, so a wire moved without a topology
/// bump still chains afresh.
#[derive(Debug)]
struct HeldConductorChains {
    groups: ConductorGroups,
    lengths: Vec<usize>,
    points: Vec<Point>,
    chains: ConductorChains,
}

/// The points of the wires `indices` names, in order; a missing wire has
/// none.
fn wire_points<'a>(
    wires: &'a [Wire],
    indices: &'a [usize],
) -> impl Iterator<Item = &'a [Point]> + 'a {
    indices.iter().map(|index| {
        wires
            .get(*index)
            .map_or(&[][..], |wire| wire.points.as_slice())
    })
}

impl HeldConductorChains {
    /// Whether `groups` over `wires` is what these chains were built from.
    fn chained_from(&self, wires: &[Wire], groups: &ConductorGroups) -> bool {
        if self.groups != *groups {
            return false;
        }
        let mut lengths = self.lengths.iter();
        let mut points = self.points.as_slice();
        for wire in groups
            .iter()
            .flat_map(|(_, indices)| wire_points(wires, indices))
        {
            if lengths.next() != Some(&wire.len()) || !points.starts_with(wire) {
                return false;
            }
            points = &points[wire.len()..];
        }
        true
    }
}

impl Clone for CanvasCache {
    /// Cloning a schematic must not copy derived caches.
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl CanvasCache {
    /// The cache, if it matches `version`; `None` means callers must fall
    /// back to scanning live data.
    pub fn fresh(&self, version: u64) -> Option<&Self> {
        (self.version == Some(version)).then_some(self)
    }

    pub(super) fn rebuild(&mut self, wires: &[Wire], junctions: &[Junction], version: u64) {
        self.wire_bounds.clear();
        self.wire_bounds.reserve(wires.len());
        self.wire_vertices.clear();
        self.junctions.clear();
        self.wire_cells.clear();
        self.oversized_wires.clear();
        *self
            .junction_candidates
            .get_mut()
            .unwrap_or_else(PoisonError::into_inner) = None;
        *self
            .conductor_chains
            .get_mut()
            .unwrap_or_else(PoisonError::into_inner) = None;

        for wire in wires {
            let mut min = Point::new(i32::MAX, i32::MAX);
            let mut max = Point::new(i32::MIN, i32::MIN);
            for (index, point) in wire.points.iter().enumerate() {
                min.x = min.x.min(point.x);
                min.y = min.y.min(point.y);
                max.x = max.x.max(point.x);
                max.y = max.y.max(point.y);
                self.wire_vertices.entry(*point).or_insert((wire.id, index));
            }
            self.wire_bounds.push((min, max));
            let wire_index = self.wire_bounds.len() - 1;
            let min_cell_x = min.x.div_euclid(SPATIAL_CELL);
            let max_cell_x = max.x.div_euclid(SPATIAL_CELL);
            let min_cell_y = min.y.div_euclid(SPATIAL_CELL);
            let max_cell_y = max.y.div_euclid(SPATIAL_CELL);
            let cells_x = i64::from(max_cell_x)
                .saturating_sub(i64::from(min_cell_x))
                .saturating_add(1);
            let cells_y = i64::from(max_cell_y)
                .saturating_sub(i64::from(min_cell_y))
                .saturating_add(1);
            let cell_count = cells_x.saturating_mul(cells_y);
            if cell_count > MAX_CELLS_PER_WIRE {
                self.oversized_wires.push(wire_index);
            } else {
                for cell_x in min_cell_x..=max_cell_x {
                    for cell_y in min_cell_y..=max_cell_y {
                        self.wire_cells
                            .entry((cell_x, cell_y))
                            .or_default()
                            .push(wire_index);
                    }
                }
            }
        }

        self.junctions.extend(junctions.iter().map(|j| j.pos));
        self.version = Some(version);
    }

    /// Sorted wire-list indices whose cached bounds may intersect a world
    /// rectangle. Large/zoomed-out queries fall back to the complete ordered
    /// list instead of walking more empty cells than there are wires.
    pub fn wire_indices_in_world_rect(&self, x0: f32, y0: f32, x1: f32, y1: f32) -> Vec<usize> {
        let to_cell = |value: f32| {
            if !value.is_finite() {
                0
            } else {
                (value.floor().clamp(i32::MIN as f32, i32::MAX as f32) as i32)
                    .div_euclid(SPATIAL_CELL)
            }
        };
        let min_cell_x = to_cell(x0.min(x1));
        let max_cell_x = to_cell(x0.max(x1));
        let min_cell_y = to_cell(y0.min(y1));
        let max_cell_y = to_cell(y0.max(y1));
        let cell_count = i64::from(max_cell_x.saturating_sub(min_cell_x).saturating_add(1))
            .saturating_mul(i64::from(
                max_cell_y.saturating_sub(min_cell_y).saturating_add(1),
            ));
        let scan_threshold = i64::try_from(self.wire_bounds.len())
            .unwrap_or(i64::MAX)
            .saturating_mul(4)
            .max(MAX_CELLS_PER_WIRE);
        if cell_count > scan_threshold {
            return (0..self.wire_bounds.len()).collect();
        }

        let mut indices = self.oversized_wires.clone();
        for cell_x in min_cell_x..=max_cell_x {
            for cell_y in min_cell_y..=max_cell_y {
                if let Some(bucket) = self.wire_cells.get(&(cell_x, cell_y)) {
                    indices.extend(bucket.iter().copied());
                }
            }
        }
        indices.sort_unstable();
        indices.dedup();
        indices
    }

    /// The junction candidates found among the conductors of `scope`, found
    /// by `find` only if none are held for exactly that scope at this
    /// topology.
    pub fn junction_candidates(
        &self,
        scope: JunctionCandidateScope,
        find: impl FnOnce() -> Vec<Point>,
    ) -> Arc<Vec<Point>> {
        let mut held = self
            .junction_candidates
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        if let Some((held_scope, candidates)) = held.as_ref()
            && *held_scope == scope
        {
            return Arc::clone(candidates);
        }
        let candidates = Arc::new(find());
        *held = Some((scope, Arc::clone(&candidates)));
        candidates
    }

    /// The chained paths of `groups` over `wires`: the held ones when the
    /// grouping and every grouped wire's points are what they were chained
    /// from, else chained afresh and held. A steady frame then only compares;
    /// chaining copies every wire and indexes every end.
    pub fn conductor_chains(&self, wires: &[Wire], groups: ConductorGroups) -> ConductorChains {
        let mut held = self
            .conductor_chains
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        if let Some(held) = held.as_ref()
            && held.chained_from(wires, &groups)
        {
            return Arc::clone(&held.chains);
        }
        let grouped = || {
            groups
                .iter()
                .flat_map(|(_, indices)| wire_points(wires, indices))
        };
        let chains: ConductorChains = Arc::new(
            groups
                .iter()
                .map(|(ink, indices)| (*ink, chain_conductors(wire_points(wires, indices))))
                .collect(),
        );
        *held = Some(HeldConductorChains {
            lengths: grouped().map(<[Point]>::len).collect(),
            points: grouped().flatten().copied().collect(),
            groups,
            chains: Arc::clone(&chains),
        });
        chains
    }

    /// Sorted wire-list candidates for an exact authored grid point.
    pub fn wire_indices_at_point(&self, point: Point) -> Vec<usize> {
        let mut indices = self.oversized_wires.clone();
        if let Some(bucket) = self.wire_cells.get(&(
            point.x.div_euclid(SPATIAL_CELL),
            point.y.div_euclid(SPATIAL_CELL),
        )) {
            indices.extend(bucket.iter().copied());
        }
        indices.sort_unstable();
        indices.dedup();
        indices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn corner_wires() -> Vec<Wire> {
        vec![
            Wire::new(1, vec![Point::new(0, 0), Point::new(40, 0)]),
            Wire::new(2, vec![Point::new(40, 0), Point::new(40, 40)]),
            Wire::new(3, vec![Point::new(0, 80), Point::new(40, 80)]),
        ]
    }

    fn chained_afresh(
        wires: &[Wire],
        groups: &ConductorGroups,
    ) -> Vec<(ConductorInk, Vec<Vec<Point>>)> {
        groups
            .iter()
            .map(|(ink, indices)| (*ink, chain_conductors(wire_points(wires, indices))))
            .collect()
    }

    /// The held chains are reused for the same grouping over the same
    /// wires, and chained afresh when the grouping changes (an ink, a wire
    /// added, dropped or reordered) or when a grouped wire's points change
    /// with the grouping unchanged, which no topology counter has to report.
    #[test]
    fn held_conductor_chains_follow_their_grouping_and_wires() {
        type Inputs = (Vec<Wire>, ConductorGroups);
        let base: Inputs = (
            corner_wires(),
            vec![
                (ConductorInk::Plain, vec![0, 1]),
                (ConductorInk::Selected, vec![2]),
            ],
        );
        let changes: [(&str, fn(&mut Inputs)); 6] = [
            ("ink", |inputs| inputs.1[1].0 = ConductorInk::Highlighted),
            ("wire added", |inputs| inputs.1[1].1.insert(0, 1)),
            ("wire dropped", |inputs| inputs.1[0].1.truncate(1)),
            ("order", |inputs| inputs.1[0].1.reverse()),
            ("a point moved", |inputs| {
                inputs.0[1].points[1] = Point::new(80, 40)
            }),
            ("a vertex added", |inputs| {
                inputs.0[0].points.insert(1, Point::new(20, 0));
            }),
        ];
        for (input, change) in changes {
            let cache = CanvasCache::default();
            let held = cache.conductor_chains(&base.0, base.1.clone());
            assert!(Arc::ptr_eq(
                &held,
                &cache.conductor_chains(&base.0, base.1.clone())
            ));
            let mut changed = base.clone();
            change(&mut changed);
            let fresh = cache.conductor_chains(&changed.0, changed.1.clone());
            assert!(!Arc::ptr_eq(&held, &fresh), "{input}: chained afresh");
            assert_eq!(*fresh, chained_afresh(&changed.0, &changed.1), "{input}");
        }

        // A rebuild for a new topology drops what was held.
        let mut cache = CanvasCache::default();
        let held = cache.conductor_chains(&base.0, base.1.clone());
        cache.rebuild(&base.0, &[], 2);
        assert!(!Arc::ptr_eq(
            &held,
            &cache.conductor_chains(&base.0, base.1.clone())
        ));
    }
}
