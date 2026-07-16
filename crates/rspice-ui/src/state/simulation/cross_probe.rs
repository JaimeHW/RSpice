use super::*;

/// Bidirectional mapping between schematic grid points and SPICE net names.
///
/// This enables professional-grade cross-probing between schematic and waveform viewer:
/// - Click on schematic wire → find corresponding waveform
/// - Click on waveform → highlight corresponding wire on schematic
///
/// The mapping is populated during netlist generation and persists until the next
/// simulation run or schematic modification.
#[derive(Debug, Clone, Default)]
pub struct CrossProbeMapping {
    /// Node point to net name lookup (e.g., Point(280, 200) → "NET3").
    /// Holds net node points (vertices, terminals, junctions, labels);
    /// probes between nodes resolve through `net_segments`.
    pub point_to_net: HashMap<Point, String>,

    /// Net name to node points lookup (e.g., "NET3" → [Point(280, 200), ...])
    /// Enables: select waveform → highlight all connected wire segments
    pub net_to_points: HashMap<String, Vec<Point>>,

    /// Wire segments per net, for resolving probes between nodes.
    pub net_segments: HashMap<String, Vec<(Point, Point)>>,

    /// Version counter - incremented when mapping is updated
    /// Used to detect when probe cache needs refresh
    pub version: u64,

    /// Exact schematic topology revision from which this map was generated.
    /// Consumers must reject the map after any structural edit.
    pub source_topology_version: Option<u64>,
}

/// Does `p` lie on the orthogonal segment `(a, b)` (inclusive)?
fn on_segment(p: Point, a: Point, b: Point) -> bool {
    if a.y == b.y && p.y == a.y {
        let (x0, x1) = (a.x.min(b.x), a.x.max(b.x));
        return p.x >= x0 && p.x <= x1;
    }
    if a.x == b.x && p.x == a.x {
        let (y0, y1) = (a.y.min(b.y), a.y.max(b.y));
        return p.y >= y0 && p.y <= y1;
    }
    false
}

impl CrossProbeMapping {
    /// Create a new empty mapping
    pub fn new() -> Self {
        Self::default()
    }

    /// Update mapping from netlist generation result
    pub fn update(
        &mut self,
        point_to_net: HashMap<Point, String>,
        net_to_points: HashMap<String, Vec<Point>>,
        net_segments: HashMap<String, Vec<(Point, Point)>>,
        source_topology_version: u64,
    ) {
        self.point_to_net = point_to_net;
        self.net_to_points = net_to_points;
        self.net_segments = net_segments;
        self.source_topology_version = Some(source_topology_version);
        self.version += 1;
    }

    /// Clear the mapping
    pub fn clear(&mut self) {
        self.point_to_net.clear();
        self.net_to_points.clear();
        self.net_segments.clear();
        self.source_topology_version = None;
        self.version += 1;
    }

    /// Look up net name for a grid point
    ///
    /// Node points answer from the hash; anything else scans the net
    /// segment lists (probe clicks are rare — this is not a frame path).
    /// Returns None if the point is not on a net (e.g., empty space).
    pub fn net_at(&self, point: Point) -> Option<&String> {
        if let Some(name) = self.point_to_net.get(&point) {
            return Some(name);
        }
        self.net_segments.iter().find_map(|(name, segments)| {
            segments
                .iter()
                .any(|&(a, b)| on_segment(point, a, b))
                .then_some(name)
        })
    }

    /// Look up all grid points for a net name
    ///
    /// Returns empty slice if net not found
    pub fn points_for_net(&self, net_name: &str) -> &[Point] {
        self.net_to_points
            .get(net_name)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Check if mapping is populated
    pub fn is_populated(&self) -> bool {
        !self.point_to_net.is_empty()
    }
}
