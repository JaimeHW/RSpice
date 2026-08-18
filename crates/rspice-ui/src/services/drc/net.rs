//! Net extraction.
//!
//! Unions connected geometry into nets with a disjoint-set over point keys,
//! which is what makes extraction linear in the number of wire endpoints
//! rather than quadratic.

use std::collections::{HashMap, HashSet};

/// Internal net tracking info.
#[derive(Debug, Clone, Default)]
pub(super) struct NetInfo {
    /// Every explicit name projected onto this electrical cluster. Retaining
    /// aliases is required to detect typed bus-member/name conflicts instead
    /// of silently discarding all but the canonical display name.
    pub(super) names: HashSet<String>,
    pub(super) connection_count: usize,
    pub(super) has_voltage_source: bool,
    pub(super) has_current_source: bool,
    pub(super) is_ground: bool,
    pub(super) connected_components: Vec<String>,
    pub(super) output_drivers: HashSet<String>,
    pub(super) voltage_source_drivers: HashSet<String>,
    pub(super) declared_output_drivers: HashSet<String>,
    pub(super) wire_ids: HashSet<u64>,
    pub(super) electrical_anchor_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct PointKey {
    x: i64,
    y: i64,
}

impl PointKey {
    pub(super) fn from_f64(x: f64, y: f64) -> Self {
        Self {
            x: x.round() as i64,
            y: y.round() as i64,
        }
    }

    pub(super) fn as_auto_name(self) -> String {
        format!("net_{}_{}", self.x, self.y)
    }

    pub(super) fn x(self) -> i64 {
        self.x
    }

    pub(super) fn y(self) -> i64 {
        self.y
    }
}

#[derive(Debug, Clone, Default)]
pub(super) struct NetAccumulator {
    pub(super) names: HashSet<String>,
    pub(super) connection_count: usize,
    pub(super) has_voltage_source: bool,
    pub(super) has_current_source: bool,
    pub(super) connected_components: HashSet<String>,
    pub(super) output_drivers: HashSet<String>,
    pub(super) voltage_source_drivers: HashSet<String>,
    pub(super) declared_output_drivers: HashSet<String>,
    pub(super) wire_ids: HashSet<u64>,
    pub(super) electrical_anchor_count: usize,
}

#[derive(Debug, Clone, Default)]
pub(super) struct DisjointSet {
    pub(super) parent: Vec<usize>,
    pub(super) rank: Vec<u8>,
}

impl DisjointSet {
    pub(super) fn make_set(&mut self) -> usize {
        let id = self.parent.len();
        self.parent.push(id);
        self.rank.push(0);
        id
    }

    pub(super) fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    pub(super) fn union(&mut self, a: usize, b: usize) {
        let root_a = self.find(a);
        let root_b = self.find(b);
        if root_a == root_b {
            return;
        }

        let rank_a = self.rank[root_a];
        let rank_b = self.rank[root_b];
        if rank_a < rank_b {
            self.parent[root_a] = root_b;
        } else if rank_a > rank_b {
            self.parent[root_b] = root_a;
        } else {
            self.parent[root_b] = root_a;
            self.rank[root_a] = self.rank[root_a].saturating_add(1);
        }
    }
}

pub(super) fn ensure_point_id(
    point: PointKey,
    point_ids: &mut HashMap<PointKey, usize>,
    points_by_id: &mut Vec<PointKey>,
    dsu: &mut DisjointSet,
) -> usize {
    if let Some(&id) = point_ids.get(&point) {
        return id;
    }
    let id = dsu.make_set();
    point_ids.insert(point, id);
    points_by_id.push(point);
    id
}

pub(super) fn point_on_segment(point: PointKey, seg_start: PointKey, seg_end: PointKey) -> bool {
    let min_x = seg_start.x.min(seg_end.x);
    let max_x = seg_start.x.max(seg_end.x);
    let min_y = seg_start.y.min(seg_end.y);
    let max_y = seg_start.y.max(seg_end.y);

    if seg_start.x == seg_end.x {
        point.x == seg_start.x && point.y >= min_y && point.y <= max_y
    } else if seg_start.y == seg_end.y {
        point.y == seg_start.y && point.x >= min_x && point.x <= max_x
    } else {
        // Fallback for non-Manhattan segments.
        // Coordinates originate as i32 schematic units, but the products of
        // two full-range deltas do not fit in i64. Promote before subtracting
        // so malformed/imported extreme geometry remains deterministic in
        // debug and release builds.
        let dx1 = i128::from(point.x) - i128::from(seg_start.x);
        let dy1 = i128::from(point.y) - i128::from(seg_start.y);
        let dx2 = i128::from(seg_end.x) - i128::from(seg_start.x);
        let dy2 = i128::from(seg_end.y) - i128::from(seg_start.y);
        let cross = dx1 * dy2 - dy1 * dx2;
        if cross != 0 {
            return false;
        }
        point.x >= min_x && point.x <= max_x && point.y >= min_y && point.y <= max_y
    }
}

pub(super) fn segment_intersection_point(
    a: (PointKey, PointKey),
    b: (PointKey, PointKey),
) -> Option<PointKey> {
    let (a0, a1) = a;
    let (b0, b1) = b;
    let r = (
        i128::from(a1.x) - i128::from(a0.x),
        i128::from(a1.y) - i128::from(a0.y),
    );
    let s = (
        i128::from(b1.x) - i128::from(b0.x),
        i128::from(b1.y) - i128::from(b0.y),
    );
    let q_minus_p = (
        i128::from(b0.x) - i128::from(a0.x),
        i128::from(b0.y) - i128::from(a0.y),
    );
    let cross = |left: (i128, i128), right: (i128, i128)| left.0 * right.1 - left.1 * right.0;
    let denominator = cross(r, s);

    if denominator == 0 {
        if cross(q_minus_p, r) != 0 {
            return None;
        }
        // Collinear overlap: return the first shared authored endpoint in a
        // deterministic order. The caller only needs one point to union the
        // two complete segments.
        let mut shared = [a0, a1, b0, b1]
            .into_iter()
            .filter(|point| point_on_segment(*point, a0, a1) && point_on_segment(*point, b0, b1))
            .collect::<Vec<_>>();
        shared.sort_unstable_by_key(|point| (point.x, point.y));
        return shared.into_iter().next();
    }

    let t_numerator = cross(q_minus_p, s);
    let u_numerator = cross(q_minus_p, r);
    let within_segment = |numerator: i128| {
        if denominator > 0 {
            (0..=denominator).contains(&numerator)
        } else {
            (denominator..=0).contains(&numerator)
        }
    };
    if !within_segment(t_numerator) || !within_segment(u_numerator) {
        return None;
    }

    // Schematic connectivity lives on the integer design lattice. A visual
    // crossing between lattice points cannot host a terminal or persisted
    // junction, so it is deliberately not promoted to an electrical point.
    let x_numerator = i128::from(a0.x) * denominator + r.0 * t_numerator;
    let y_numerator = i128::from(a0.y) * denominator + r.1 * t_numerator;
    if x_numerator % denominator != 0 || y_numerator % denominator != 0 {
        return None;
    }
    Some(PointKey {
        x: i64::try_from(x_numerator / denominator).ok()?,
        y: i64::try_from(y_numerator / denominator).ok()?,
    })
}

pub(super) fn merge_net_accumulator(
    net_map: &mut HashMap<String, NetInfo>,
    acc: NetAccumulator,
    fallback_point: Option<PointKey>,
) {
    let canonical_name = canonical_net_name(&acc.names, fallback_point);
    let entry = net_map
        .entry(canonical_name.clone())
        .or_insert_with(|| NetInfo {
            ..NetInfo::default()
        });

    entry.connection_count += acc.connection_count;
    entry.has_voltage_source |= acc.has_voltage_source;
    entry.has_current_source |= acc.has_current_source;
    // Ground is decided by the product's one alias authority, so the report
    // and the engine cannot disagree about which cluster is node 0.
    entry.is_ground |= crate::state::is_ground_reference(&canonical_name)
        || acc
            .names
            .iter()
            .any(|name| crate::state::is_ground_reference(name));
    entry.names.extend(acc.names);

    for component in acc.connected_components {
        if !entry
            .connected_components
            .iter()
            .any(|existing| existing == &component)
        {
            entry.connected_components.push(component);
        }
    }
    entry.output_drivers.extend(acc.output_drivers);
    entry
        .voltage_source_drivers
        .extend(acc.voltage_source_drivers);
    entry
        .declared_output_drivers
        .extend(acc.declared_output_drivers);
    entry.wire_ids.extend(acc.wire_ids);
    entry.electrical_anchor_count = entry
        .electrical_anchor_count
        .saturating_add(acc.electrical_anchor_count);
}

pub(super) fn canonical_net_name(
    names: &HashSet<String>,
    fallback_point: Option<PointKey>,
) -> String {
    let mut candidates: Vec<String> = names
        .iter()
        .filter_map(|name| {
            let trimmed = name.trim();
            (!trimmed.is_empty()).then_some(trimmed.to_string())
        })
        .collect();
    candidates.sort_by(|a, b| {
        is_auto_generated_net_name(a)
            .cmp(&is_auto_generated_net_name(b))
            .then_with(|| a.cmp(b))
    });

    if let Some(name) = candidates.into_iter().next() {
        name
    } else if let Some(point) = fallback_point {
        point.as_auto_name()
    } else {
        "net_unassigned".to_string()
    }
}

pub(super) fn is_auto_generated_net_name(name: &str) -> bool {
    if let Some(coordinates) = name.strip_prefix("net_") {
        return coordinates
            .split_once('_')
            .is_some_and(|(x, y)| x.parse::<i64>().is_ok() && y.parse::<i64>().is_ok());
    }
    name.contains('_')
        && name
            .chars()
            .all(|c| c.is_numeric() || c == '_' || c == '-' || c == '.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_coordinate_fallbacks_are_classified_as_generated_net_names() {
        assert!(is_auto_generated_net_name("net_10_-20"));
        assert!(is_auto_generated_net_name("10_-20"));
        assert!(!is_auto_generated_net_name("net_user"));
        assert!(!is_auto_generated_net_name("net_10_20_suffix"));
    }

    #[test]
    fn diagonal_point_test_handles_full_i32_coordinate_range() {
        let start = PointKey::from_f64(f64::from(i32::MIN), f64::from(i32::MIN));
        let end = PointKey::from_f64(f64::from(i32::MAX), f64::from(i32::MAX));
        let origin = PointKey::from_f64(0.0, 0.0);

        assert!(point_on_segment(origin, start, end));
    }

    #[test]
    fn segment_intersection_is_exact_for_any_angle_lattice_geometry() {
        let point = |x, y| PointKey { x, y };

        assert_eq!(
            segment_intersection_point((point(0, 0), point(40, 40)), (point(0, 40), point(40, 0)),),
            Some(point(20, 20))
        );
        assert_eq!(
            segment_intersection_point(
                (point(0, 0), point(30, 20)),
                (point(15, 10), point(15, 40)),
            ),
            Some(point(15, 10)),
            "endpoint-to-diagonal attachment is retained"
        );
        assert_eq!(
            segment_intersection_point((point(0, 0), point(3, 2)), (point(0, 2), point(3, 0)),),
            None,
            "a crossing between lattice points cannot be an authored node"
        );
    }

    #[test]
    fn collinear_any_angle_overlap_returns_a_deterministic_shared_point() {
        let point = |x, y| PointKey { x, y };
        assert_eq!(
            segment_intersection_point(
                (point(0, 0), point(40, 40)),
                (point(20, 20), point(60, 60)),
            ),
            Some(point(20, 20))
        );
        assert_eq!(
            segment_intersection_point((point(0, 0), point(40, 40)), (point(0, 20), point(40, 60)),),
            None
        );
    }
}
