use std::collections::{HashMap, HashSet};

/// Internal net tracking info.
#[derive(Debug, Clone, Default)]
pub(super) struct NetInfo {
    pub(super) connection_count: usize,
    pub(super) has_voltage_source: bool,
    pub(super) has_current_source: bool,
    pub(super) is_ground: bool,
    pub(super) connected_components: Vec<String>,
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
        let dx1 = point.x - seg_start.x;
        let dy1 = point.y - seg_start.y;
        let dx2 = seg_end.x - seg_start.x;
        let dy2 = seg_end.y - seg_start.y;
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
    let a_vertical = a0.x == a1.x;
    let b_vertical = b0.x == b1.x;

    match (a_vertical, b_vertical) {
        (true, false) => {
            let y = b0.y;
            let x = a0.x;
            let point = PointKey { x, y };
            (point_on_segment(point, a0, a1) && point_on_segment(point, b0, b1)).then_some(point)
        }
        (false, true) => {
            let y = a0.y;
            let x = b0.x;
            let point = PointKey { x, y };
            (point_on_segment(point, a0, a1) && point_on_segment(point, b0, b1)).then_some(point)
        }
        (true, true) => {
            if a0.x != b0.x {
                return None;
            }
            let a_min_y = a0.y.min(a1.y);
            let a_max_y = a0.y.max(a1.y);
            let b_min_y = b0.y.min(b1.y);
            let b_max_y = b0.y.max(b1.y);
            let overlap_start = a_min_y.max(b_min_y);
            let overlap_end = a_max_y.min(b_max_y);
            if overlap_start <= overlap_end {
                Some(PointKey {
                    x: a0.x,
                    y: overlap_start,
                })
            } else {
                None
            }
        }
        (false, false) => {
            if a0.y != b0.y {
                return None;
            }
            let a_min_x = a0.x.min(a1.x);
            let a_max_x = a0.x.max(a1.x);
            let b_min_x = b0.x.min(b1.x);
            let b_max_x = b0.x.max(b1.x);
            let overlap_start = a_min_x.max(b_min_x);
            let overlap_end = a_max_x.min(b_max_x);
            if overlap_start <= overlap_end {
                Some(PointKey {
                    x: overlap_start,
                    y: a0.y,
                })
            } else {
                None
            }
        }
    }
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
    entry.is_ground |= canonical_name.eq_ignore_ascii_case("0")
        || canonical_name.eq_ignore_ascii_case("gnd")
        || canonical_name.eq_ignore_ascii_case("ground")
        || acc.names.iter().any(|name| is_ground_like(name));

    for component in acc.connected_components {
        if !entry
            .connected_components
            .iter()
            .any(|existing| existing == &component)
        {
            entry.connected_components.push(component);
        }
    }
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

pub(super) fn is_ground_like(name: &str) -> bool {
    name.eq_ignore_ascii_case("0")
        || name.eq_ignore_ascii_case("gnd")
        || name.eq_ignore_ascii_case("ground")
}

pub(super) fn is_auto_generated_net_name(name: &str) -> bool {
    if name.starts_with("net_") {
        return true;
    }
    name.contains('_')
        && name
            .chars()
            .all(|c| c.is_numeric() || c == '_' || c == '-' || c == '.')
}
