//! Exact ambiguous-crossing candidates shared by editing and canvas queries.

use super::wire::{Wire, WireSegment};
use rspice_design_model::Point;
use std::collections::{HashMap, HashSet};

/// Nearest candidate with deterministic distance and coordinate tie breaking.
pub fn nearest_junction_candidate(candidates: &[Point], pos: Point, radius: i32) -> Option<Point> {
    let radius_sq = i128::from(radius.max(0)).pow(2);
    candidates
        .iter()
        .copied()
        .filter_map(|candidate| {
            let dx = i128::from(candidate.x) - i128::from(pos.x);
            let dy = i128::from(candidate.y) - i128::from(pos.y);
            let distance_sq = dx * dx + dy * dy;
            (distance_sq <= radius_sq).then_some((distance_sq, candidate))
        })
        .min_by_key(|(distance_sq, candidate)| (*distance_sq, candidate.x, candidate.y))
        .map(|(_, candidate)| candidate)
}

pub fn collect_junction_candidates(wires: &[Wire]) -> Vec<Point> {
    const CELL: i32 = 256;
    let segments: Vec<(u64, WireSegment)> = wires
        .iter()
        .flat_map(|wire| wire.segments().map(move |segment| (wire.id, segment)))
        .collect();
    let mut cells: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
    for (index, (_, segment)) in segments.iter().enumerate() {
        for cell_x in segment.start.x.min(segment.end.x).div_euclid(CELL)
            ..=segment.start.x.max(segment.end.x).div_euclid(CELL)
        {
            for cell_y in segment.start.y.min(segment.end.y).div_euclid(CELL)
                ..=segment.start.y.max(segment.end.y).div_euclid(CELL)
            {
                cells.entry((cell_x, cell_y)).or_default().push(index);
            }
        }
    }

    let mut candidates = HashSet::new();
    let mut tested = HashSet::new();
    for bucket in cells.values() {
        for (offset, &left) in bucket.iter().enumerate() {
            for &right in &bucket[offset + 1..] {
                let pair = if left < right {
                    (left, right)
                } else {
                    (right, left)
                };
                if !tested.insert(pair) || segments[left].0 == segments[right].0 {
                    continue;
                }
                // `intersection` rounds a crossing that falls between grid
                // points; a dot there would lie on one wire and join nothing.
                if let Some(point) = segments[left].1.intersection(&segments[right].1)
                    && segments[left].1.contains_point(point)
                    && segments[right].1.contains_point(point)
                    && point != segments[left].1.start
                    && point != segments[left].1.end
                    && point != segments[right].1.start
                    && point != segments[right].1.end
                {
                    candidates.insert(point);
                }
            }
        }
    }
    let mut candidates: Vec<_> = candidates.into_iter().collect();
    candidates.sort_by_key(|point| (point.x, point.y));
    candidates
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A free-angle wire from (0,0) to (70,30) crosses y = 10 at x = 23⅓.
    /// The nearest grid point lies on the horizontal wire only, so a dot
    /// offered there would join nothing.
    #[test]
    fn a_crossing_between_grid_points_is_not_offered() {
        let between = [
            Wire::new(1, vec![Point::new(0, 0), Point::new(70, 30)]),
            Wire::new(2, vec![Point::new(0, 10), Point::new(70, 10)]),
        ];
        assert!(collect_junction_candidates(&between).is_empty());

        let on_grid = [
            Wire::new(1, vec![Point::new(0, 0), Point::new(40, 40)]),
            Wire::new(2, vec![Point::new(0, 20), Point::new(40, 20)]),
        ];
        assert_eq!(
            collect_junction_candidates(&on_grid),
            vec![Point::new(20, 20)]
        );
    }
}
