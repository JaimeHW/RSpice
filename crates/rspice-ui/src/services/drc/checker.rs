use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::input::{ComponentInfo, NetLabelInfo, WireInfo};
use super::net::{
    DisjointSet, NetAccumulator, NetInfo, PointKey, ensure_point_id, is_auto_generated_net_name,
    merge_net_accumulator, point_on_segment, segment_intersection_point,
};
use super::types::{DrcLocation, DrcResult, DrcSeverity, DrcViolation, DrcViolationType};

/// Design Rule Checker engine.
///
/// Performs comprehensive connectivity and electrical rule checks
/// on a schematic design.
pub struct DrcChecker {
    /// Counter for violation IDs
    next_id: usize,
    /// Configuration options
    config: DrcConfig,
}

/// DRC configuration options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrcConfig {
    /// Check for floating nodes
    pub check_floating_nodes: bool,
    /// Check for unconnected pins
    pub check_unconnected_pins: bool,
    /// Check for missing ground
    pub check_missing_ground: bool,
    /// Check for duplicate names
    pub check_duplicate_names: bool,
    /// Check for shorted outputs
    pub check_shorted_outputs: bool,
    /// Minimum connection count for a node to not be floating
    pub min_connections: usize,
    /// Severity overrides by violation type
    pub severity_overrides: HashMap<DrcViolationType, DrcSeverity>,
}

impl Default for DrcConfig {
    fn default() -> Self {
        Self {
            check_floating_nodes: true,
            check_unconnected_pins: true,
            check_missing_ground: true,
            check_duplicate_names: true,
            check_shorted_outputs: true,
            min_connections: 2,
            severity_overrides: HashMap::new(),
        }
    }
}

impl DrcChecker {
    /// Create a new DRC checker with default configuration
    pub fn new() -> Self {
        Self {
            next_id: 0,
            config: DrcConfig::default(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: DrcConfig) -> Self {
        Self { next_id: 0, config }
    }

    /// Get the next violation ID
    fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Run all DRC checks on the schematic.
    pub fn check_connectivity(
        &mut self,
        components: &[ComponentInfo],
        wires: &[WireInfo],
        net_labels: &[NetLabelInfo],
    ) -> DrcResult {
        let start = std::time::Instant::now();
        let mut result = DrcResult::new();

        // Build net connectivity map
        let net_map = self.build_net_map(components, wires, net_labels);

        // Check for duplicate component names
        if self.config.check_duplicate_names {
            self.check_duplicate_names(components, &mut result);
        }

        // Check for missing ground
        if self.config.check_missing_ground {
            self.check_missing_ground(&net_map, &mut result);
        }

        // Check for floating nodes
        if self.config.check_floating_nodes {
            self.check_floating_nodes(&net_map, &mut result);
        }

        // Check for shorted outputs (multiple voltage sources on same net)
        if self.config.check_shorted_outputs {
            self.check_shorted_outputs(components, &net_map, &mut result);
        }

        result.completed = true;
        result.duration_ms = start.elapsed().as_millis() as u64;
        result
    }

    /// Build a connectivity-derived map of net names and electrical attributes.
    ///
    /// This combines explicit net names with geometric connectivity from wire
    /// segments, component pin coordinates, and net label coordinates.
    fn build_net_map(
        &self,
        components: &[ComponentInfo],
        wires: &[WireInfo],
        net_labels: &[NetLabelInfo],
    ) -> HashMap<String, NetInfo> {
        let mut net_map: HashMap<String, NetInfo> = HashMap::new();
        let mut point_ids: HashMap<PointKey, usize> = HashMap::new();
        let mut points_by_id: Vec<PointKey> = Vec::new();
        let mut dsu = DisjointSet::default();
        let mut segments: Vec<(PointKey, PointKey)> = Vec::with_capacity(wires.len());

        for wire in wires {
            let start = PointKey::from_f64(wire.start_x, wire.start_y);
            let end = PointKey::from_f64(wire.end_x, wire.end_y);
            let start_id = ensure_point_id(start, &mut point_ids, &mut points_by_id, &mut dsu);
            let end_id = ensure_point_id(end, &mut point_ids, &mut points_by_id, &mut dsu);
            dsu.union(start_id, end_id);
            segments.push((start, end));
        }

        // Coarse spatial hash over segment bounding boxes: intersection and
        // point-on-segment sweeps consider only segments sharing a cell,
        // replacing all-pairs scans that froze the UI on large designs.
        const CELL: i64 = 256;
        let mut cells: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
        for (index, &(start, end)) in segments.iter().enumerate() {
            let (x0, x1) = (start.x().min(end.x()), start.x().max(end.x()));
            let (y0, y1) = (start.y().min(end.y()), start.y().max(end.y()));
            for cx in x0.div_euclid(CELL)..=x1.div_euclid(CELL) {
                for cy in y0.div_euclid(CELL)..=y1.div_euclid(CELL) {
                    cells.entry((cx, cy)).or_default().push(index);
                }
            }
        }

        // Merge touching/crossing segments so T-junctions and wire intersections
        // become a single electrical net.
        let mut tested: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
        for bucket in cells.values() {
            for (slot, &i) in bucket.iter().enumerate() {
                for &j in &bucket[slot + 1..] {
                    let pair = (i.min(j), i.max(j));
                    if !tested.insert(pair) {
                        continue;
                    }
                    if let Some(intersection) =
                        segment_intersection_point(segments[pair.0], segments[pair.1])
                    {
                        let p_id = ensure_point_id(
                            intersection,
                            &mut point_ids,
                            &mut points_by_id,
                            &mut dsu,
                        );
                        let (a0, a1) = segments[pair.0];
                        let (b0, b1) = segments[pair.1];
                        let a0_id = point_ids[&a0];
                        let a1_id = point_ids[&a1];
                        let b0_id = point_ids[&b0];
                        let b1_id = point_ids[&b1];
                        dsu.union(a0_id, p_id);
                        dsu.union(a1_id, p_id);
                        dsu.union(b0_id, p_id);
                        dsu.union(b1_id, p_id);
                    }
                }
            }
        }

        // A point on a segment is inside that segment's bounding box, so
        // the point's cell already lists every candidate.
        let segments_near = |point: PointKey| -> &[usize] {
            cells
                .get(&(point.x().div_euclid(CELL), point.y().div_euclid(CELL)))
                .map(|bucket| bucket.as_slice())
                .unwrap_or(&[])
        };

        // Attach component pins to any segment they lie on.
        for comp in components {
            for pin in &comp.pins {
                let (Some(x), Some(y)) = (pin.x, pin.y) else {
                    continue;
                };
                let point = PointKey::from_f64(x, y);
                let point_id = ensure_point_id(point, &mut point_ids, &mut points_by_id, &mut dsu);
                for &seg_index in segments_near(point) {
                    let (start, end) = segments[seg_index];
                    if point_on_segment(point, start, end) {
                        dsu.union(point_id, point_ids[&start]);
                        dsu.union(point_id, point_ids[&end]);
                    }
                }
            }
        }

        // Attach labels to any segment they touch.
        for label in net_labels {
            let point = PointKey::from_f64(label.x, label.y);
            let point_id = ensure_point_id(point, &mut point_ids, &mut points_by_id, &mut dsu);
            for &seg_index in segments_near(point) {
                let (start, end) = segments[seg_index];
                if point_on_segment(point, start, end) {
                    dsu.union(point_id, point_ids[&start]);
                    dsu.union(point_id, point_ids[&end]);
                }
            }
        }

        let mut cluster_accumulators: HashMap<usize, NetAccumulator> = HashMap::new();
        let mut name_only_accumulators: HashMap<String, NetAccumulator> = HashMap::new();

        // Count component pin connections, preferring geometry-aware clusters
        // when coordinates are available and falling back to net name grouping.
        for comp in components {
            for pin in &comp.pins {
                let update_acc = |acc: &mut NetAccumulator| {
                    acc.connection_count += 1;
                    acc.connected_components.insert(comp.name.clone());
                    if !pin.net_name.trim().is_empty() {
                        acc.names.insert(pin.net_name.clone());
                    }
                    if comp.is_voltage_source {
                        acc.has_voltage_source = true;
                    }
                    if comp.is_current_source {
                        acc.has_current_source = true;
                    }
                };

                if let (Some(x), Some(y)) = (pin.x, pin.y) {
                    let point = PointKey::from_f64(x, y);
                    let point_id =
                        ensure_point_id(point, &mut point_ids, &mut points_by_id, &mut dsu);
                    let root = dsu.find(point_id);
                    update_acc(cluster_accumulators.entry(root).or_default());
                } else {
                    update_acc(
                        name_only_accumulators
                            .entry(pin.net_name.clone())
                            .or_default(),
                    );
                }
            }
        }

        // Fold label names into geometry clusters.
        for label in net_labels {
            let point = PointKey::from_f64(label.x, label.y);
            let point_id = ensure_point_id(point, &mut point_ids, &mut points_by_id, &mut dsu);
            let root = dsu.find(point_id);
            let acc = cluster_accumulators.entry(root).or_default();
            if !label.name.trim().is_empty() {
                acc.names.insert(label.name.clone());
            }
        }

        let mut representative_by_root: HashMap<usize, PointKey> = HashMap::new();
        for point in points_by_id.iter().copied() {
            if let Some(&id) = point_ids.get(&point) {
                let root = dsu.find(id);
                representative_by_root.entry(root).or_insert(point);
            }
        }

        for (root, acc) in cluster_accumulators {
            let fallback = representative_by_root.get(&root).copied();
            merge_net_accumulator(&mut net_map, acc, fallback);
        }

        for (_, acc) in name_only_accumulators {
            merge_net_accumulator(&mut net_map, acc, None);
        }

        for net in net_map.values_mut() {
            net.connected_components.sort();
            net.connected_components.dedup();
        }

        net_map
    }

    /// Check for duplicate component names
    fn check_duplicate_names(&mut self, components: &[ComponentInfo], result: &mut DrcResult) {
        let mut names: HashMap<String, Vec<usize>> = HashMap::new();

        for (idx, comp) in components.iter().enumerate() {
            names.entry(comp.name.clone()).or_default().push(idx);
        }

        for (name, indices) in names {
            if indices.len() > 1 {
                let id = self.next_id();
                result.add_violation(
                    DrcViolation::new(
                        id,
                        DrcViolationType::DuplicateName,
                        format!(
                            "Duplicate component name '{}' ({} instances)",
                            name,
                            indices.len()
                        ),
                        DrcLocation::Component {
                            id: indices[0],
                            name: name.clone(),
                        },
                    )
                    .with_related(vec![name]),
                );
            }
        }
    }

    /// Check for missing ground reference
    fn check_missing_ground(&mut self, net_map: &HashMap<String, NetInfo>, result: &mut DrcResult) {
        let has_ground = net_map.values().any(|n| n.is_ground);
        let has_zero_net = net_map.contains_key("0");

        if !has_ground && !has_zero_net {
            let id = self.next_id();
            result.add_violation(DrcViolation::new(
                id,
                DrcViolationType::MissingGround,
                "Circuit has no ground reference (node 0 or GND)",
                DrcLocation::Global,
            ));
        }
    }

    /// Check for floating nodes
    fn check_floating_nodes(&mut self, net_map: &HashMap<String, NetInfo>, result: &mut DrcResult) {
        for (name, net) in net_map {
            if net.connection_count == 0 {
                continue;
            }
            if net.connection_count < self.config.min_connections && !net.is_ground {
                // Skip auto-generated net names from wire coordinates
                if is_auto_generated_net_name(name) {
                    continue;
                }

                let id = self.next_id();
                result.add_violation(
                    DrcViolation::new(
                        id,
                        DrcViolationType::FloatingNode,
                        format!(
                            "Node '{}' has only {} connection(s)",
                            name, net.connection_count
                        ),
                        DrcLocation::Node {
                            net_name: name.clone(),
                        },
                    )
                    .with_related(net.connected_components.clone()),
                );
            }
        }
    }

    /// Check for multiple voltage sources on same net
    fn check_shorted_outputs(
        &mut self,
        components: &[ComponentInfo],
        net_map: &HashMap<String, NetInfo>,
        result: &mut DrcResult,
    ) {
        // Count voltage sources per net
        let mut voltage_source_nets: HashMap<String, Vec<String>> = HashMap::new();

        for comp in components {
            if comp.is_voltage_source {
                for pin in &comp.pins {
                    // Only check output pins (positive terminal for voltage source)
                    if pin.is_output || pin.name == "+" {
                        voltage_source_nets
                            .entry(pin.net_name.clone())
                            .or_default()
                            .push(comp.name.clone());
                    }
                }
            }
        }

        for (net_name, sources) in voltage_source_nets {
            if sources.len() > 1 {
                let id = self.next_id();
                result.add_violation(
                    DrcViolation::new(
                        id,
                        DrcViolationType::ShortedOutputs,
                        format!(
                            "Net '{}' has {} voltage sources connected: {}",
                            net_name,
                            sources.len(),
                            sources.join(", ")
                        ),
                        DrcLocation::Node { net_name },
                    )
                    .with_related(sources),
                );
            }
        }

        // Suppress unused warning
        let _ = net_map;
    }
}

impl Default for DrcChecker {
    fn default() -> Self {
        Self::new()
    }
}
