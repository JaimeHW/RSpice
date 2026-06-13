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
        let start = crate::common::time_compat::Instant::now();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::drc::input::PinInfo;

    fn pin(name: &str, net: &str) -> PinInfo {
        PinInfo {
            name: name.to_string(),
            net_name: net.to_string(),
            is_output: false,
            x: None,
            y: None,
        }
    }

    fn pin_at(name: &str, net: &str, x: f64, y: f64) -> PinInfo {
        PinInfo {
            name: name.to_string(),
            net_name: net.to_string(),
            is_output: false,
            x: Some(x),
            y: Some(y),
        }
    }

    fn resistor(id: usize, name: &str, pins: Vec<PinInfo>) -> ComponentInfo {
        ComponentInfo {
            id,
            name: name.to_string(),
            component_type: "resistor".to_string(),
            pins,
            is_voltage_source: false,
            is_current_source: false,
        }
    }

    fn vsource(id: usize, name: &str, plus_net: &str, minus_net: &str) -> ComponentInfo {
        ComponentInfo {
            id,
            name: name.to_string(),
            component_type: "voltage_source".to_string(),
            pins: vec![pin("+", plus_net), pin("-", minus_net)],
            is_voltage_source: true,
            is_current_source: false,
        }
    }

    fn wire(id: usize, x0: f64, y0: f64, x1: f64, y1: f64) -> WireInfo {
        WireInfo {
            id,
            start_x: x0,
            start_y: y0,
            end_x: x1,
            end_y: y1,
        }
    }

    fn label(name: &str, x: f64, y: f64) -> NetLabelInfo {
        NetLabelInfo {
            name: name.to_string(),
            x,
            y,
        }
    }

    fn of_type<'a>(result: &'a DrcResult, vt: DrcViolationType) -> Vec<&'a DrcViolation> {
        result
            .violations()
            .iter()
            .filter(|v| v.violation_type == vt)
            .collect()
    }

    // V1 across vin/0 with R1 in parallel: every net has two connections and
    // ground is present, so a default check is clean.
    fn minimal_circuit() -> Vec<ComponentInfo> {
        vec![
            vsource(0, "V1", "vin", "0"),
            resistor(1, "R1", vec![pin("1", "vin"), pin("2", "0")]),
        ]
    }

    #[test]
    fn empty_schematic_reports_only_missing_ground() {
        let mut checker = DrcChecker::new();
        let result = checker.check_connectivity(&[], &[], &[]);
        assert!(result.completed);
        assert_eq!(result.total_count(), 1);
        assert_eq!(
            result.violations()[0].violation_type,
            DrcViolationType::MissingGround
        );
    }

    #[test]
    fn empty_schematic_clean_when_ground_check_disabled() {
        let config = DrcConfig {
            check_missing_ground: false,
            ..DrcConfig::default()
        };
        let mut checker = DrcChecker::with_config(config);
        let result = checker.check_connectivity(&[], &[], &[]);
        assert!(result.completed);
        assert_eq!(result.total_count(), 0);
        assert!(result.passed());
    }

    #[test]
    fn minimal_connected_circuit_has_no_violations() {
        let mut checker = DrcChecker::new();
        let result = checker.check_connectivity(&minimal_circuit(), &[], &[]);
        assert_eq!(result.total_count(), 0, "{:?}", result.violations());
        assert!(result.passed());
    }

    #[test]
    fn duplicate_component_names_are_flagged() {
        let mut components = minimal_circuit();
        components.push(resistor(2, "R1", vec![pin("1", "vin"), pin("2", "0")]));

        let mut checker = DrcChecker::new();
        let result = checker.check_connectivity(&components, &[], &[]);
        let dups = of_type(&result, DrcViolationType::DuplicateName);
        assert_eq!(dups.len(), 1);
        assert_eq!(dups[0].severity, DrcSeverity::Critical);
        match &dups[0].location {
            DrcLocation::Component { name, .. } => assert_eq!(name, "R1"),
            other => panic!("expected component location, got {:?}", other),
        }
    }

    #[test]
    fn triplicate_name_reported_once_not_per_pair() {
        let components = vec![
            vsource(0, "V1", "vin", "0"),
            resistor(1, "R1", vec![pin("1", "vin"), pin("2", "0")]),
            resistor(2, "R1", vec![pin("1", "vin"), pin("2", "0")]),
            resistor(3, "R1", vec![pin("1", "vin"), pin("2", "0")]),
        ];
        let mut checker = DrcChecker::new();
        let result = checker.check_connectivity(&components, &[], &[]);
        let dups = of_type(&result, DrcViolationType::DuplicateName);
        assert_eq!(dups.len(), 1);
        assert!(dups[0].message.contains("3 instances"));
    }

    #[test]
    fn duplicate_name_check_can_be_disabled() {
        let mut components = minimal_circuit();
        components.push(resistor(2, "R1", vec![pin("1", "vin"), pin("2", "0")]));

        let config = DrcConfig {
            check_duplicate_names: false,
            ..DrcConfig::default()
        };
        let mut checker = DrcChecker::with_config(config);
        let result = checker.check_connectivity(&components, &[], &[]);
        assert!(of_type(&result, DrcViolationType::DuplicateName).is_empty());
    }

    #[test]
    fn missing_ground_flagged_with_global_location() {
        let components = vec![
            vsource(0, "V1", "vin", "vee"),
            resistor(1, "R1", vec![pin("1", "vin"), pin("2", "vee")]),
        ];
        let mut checker = DrcChecker::new();
        let result = checker.check_connectivity(&components, &[], &[]);
        let missing = of_type(&result, DrcViolationType::MissingGround);
        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0].location, DrcLocation::Global);
        assert_eq!(missing[0].severity, DrcSeverity::Critical);
    }

    #[test]
    fn gnd_name_satisfies_ground_check_case_insensitive() {
        let components = vec![
            vsource(0, "V1", "vin", "GND"),
            resistor(1, "R1", vec![pin("1", "vin"), pin("2", "GND")]),
        ];
        let mut checker = DrcChecker::new();
        let result = checker.check_connectivity(&components, &[], &[]);
        assert!(of_type(&result, DrcViolationType::MissingGround).is_empty());
    }

    #[test]
    fn single_connection_net_is_floating() {
        let mut components = minimal_circuit();
        components.push(resistor(2, "R2", vec![pin("1", "vin"), pin("2", "out")]));

        let mut checker = DrcChecker::new();
        let result = checker.check_connectivity(&components, &[], &[]);
        let floating = of_type(&result, DrcViolationType::FloatingNode);
        assert_eq!(floating.len(), 1);
        assert_eq!(floating[0].severity, DrcSeverity::Error);
        assert_eq!(
            floating[0].location,
            DrcLocation::Node {
                net_name: "out".to_string()
            }
        );
        // Related items identify the component so the UI can highlight it.
        assert_eq!(floating[0].related_items, vec!["R2".to_string()]);
    }

    #[test]
    fn ground_net_is_never_floating() {
        let components = vec![resistor(0, "R1", vec![pin("1", "0"), pin("2", "a")])];
        let mut checker = DrcChecker::new();
        let result = checker.check_connectivity(&components, &[], &[]);
        let floating = of_type(&result, DrcViolationType::FloatingNode);
        assert_eq!(floating.len(), 1);
        assert_eq!(
            floating[0].location,
            DrcLocation::Node {
                net_name: "a".to_string()
            }
        );
    }

    #[test]
    fn auto_named_nets_are_not_reported_floating() {
        // A pin with coordinates but no net name lands in a geometry cluster
        // whose canonical name is auto-generated; the floating check skips it.
        let components = vec![
            vsource(0, "V1", "vin", "0"),
            resistor(1, "R1", vec![pin("1", "vin"), pin("2", "0")]),
            resistor(
                2,
                "R2",
                vec![pin("1", "vin"), pin_at("2", "", 300.0, 300.0)],
            ),
        ];
        let mut checker = DrcChecker::new();
        let result = checker.check_connectivity(&components, &[], &[]);
        assert!(of_type(&result, DrcViolationType::FloatingNode).is_empty());
    }

    #[test]
    fn min_connections_config_is_honored() {
        let config = DrcConfig {
            min_connections: 3,
            ..DrcConfig::default()
        };
        let mut checker = DrcChecker::with_config(config);
        let result = checker.check_connectivity(&minimal_circuit(), &[], &[]);
        let floating = of_type(&result, DrcViolationType::FloatingNode);
        // "vin" has two connections, below the threshold; "0" is exempt as ground.
        assert_eq!(floating.len(), 1);
        assert_eq!(
            floating[0].location,
            DrcLocation::Node {
                net_name: "vin".to_string()
            }
        );
    }

    #[test]
    fn two_voltage_sources_on_one_net_are_shorted_outputs() {
        let components = vec![
            vsource(0, "V1", "vin", "0"),
            vsource(1, "V2", "vin", "0"),
            resistor(2, "R1", vec![pin("1", "vin"), pin("2", "0")]),
        ];
        let mut checker = DrcChecker::new();
        let result = checker.check_connectivity(&components, &[], &[]);
        let shorted = of_type(&result, DrcViolationType::ShortedOutputs);
        assert_eq!(shorted.len(), 1);
        assert_eq!(
            shorted[0].location,
            DrcLocation::Node {
                net_name: "vin".to_string()
            }
        );
        let mut related = shorted[0].related_items.clone();
        related.sort();
        assert_eq!(related, vec!["V1".to_string(), "V2".to_string()]);
    }

    #[test]
    fn sources_on_distinct_nets_are_not_shorted() {
        let components = vec![
            vsource(0, "V1", "vin", "0"),
            vsource(1, "V2", "vcc", "0"),
            resistor(2, "R1", vec![pin("1", "vin"), pin("2", "0")]),
            resistor(3, "R2", vec![pin("1", "vcc"), pin("2", "0")]),
        ];
        let mut checker = DrcChecker::new();
        let result = checker.check_connectivity(&components, &[], &[]);
        assert!(of_type(&result, DrcViolationType::ShortedOutputs).is_empty());
        assert!(result.passed());
    }

    #[test]
    fn pins_on_same_wire_form_one_net() {
        // Both pins lie on a single horizontal wire; a label on the wire
        // names the net so the floating check would see it if it misfired.
        let components = vec![
            resistor(0, "R1", vec![pin_at("1", "", 0.0, 0.0), pin("2", "0")]),
            resistor(1, "R2", vec![pin_at("1", "", 50.0, 0.0), pin("2", "0")]),
        ];
        let wires = vec![wire(0, 0.0, 0.0, 100.0, 0.0)];
        let labels = vec![label("mid", 80.0, 0.0)];
        let mut checker = DrcChecker::new();
        let result = checker.check_connectivity(&components, &wires, &labels);
        assert!(
            of_type(&result, DrcViolationType::FloatingNode).is_empty(),
            "{:?}",
            result.violations()
        );
    }

    #[test]
    fn crossing_wires_merge_into_one_net() {
        // Vertical and horizontal wires crossing at the origin carry one pin
        // each; the intersection unifies them into a two-connection net.
        let components = vec![
            resistor(0, "R1", vec![pin_at("1", "", 0.0, 100.0), pin("2", "0")]),
            resistor(1, "R2", vec![pin_at("1", "", 100.0, 0.0), pin("2", "0")]),
        ];
        let wires = vec![
            wire(0, 0.0, -100.0, 0.0, 100.0),
            wire(1, -100.0, 0.0, 100.0, 0.0),
        ];
        let labels = vec![label("x", 0.0, -100.0)];
        let mut checker = DrcChecker::new();
        let result = checker.check_connectivity(&components, &wires, &labels);
        assert!(
            of_type(&result, DrcViolationType::FloatingNode).is_empty(),
            "{:?}",
            result.violations()
        );
    }

    #[test]
    fn labeled_pin_with_no_wire_is_floating_under_label_name() {
        // The label shares the pin coordinate, so the cluster takes the
        // label's name and the single connection is reported against it.
        let components = vec![
            vsource(0, "V1", "vin", "0"),
            resistor(1, "R1", vec![pin("1", "vin"), pin("2", "0")]),
            resistor(
                2,
                "R2",
                vec![pin("1", "vin"), pin_at("2", "", 0.0, 200.0)],
            ),
        ];
        let labels = vec![label("dangling", 0.0, 200.0)];
        let mut checker = DrcChecker::new();
        let result = checker.check_connectivity(&components, &[], &labels);
        let floating = of_type(&result, DrcViolationType::FloatingNode);
        assert_eq!(floating.len(), 1);
        assert_eq!(
            floating[0].location,
            DrcLocation::Node {
                net_name: "dangling".to_string()
            }
        );
        assert_eq!(floating[0].related_items, vec!["R2".to_string()]);
    }

    #[test]
    fn fixed_violation_disappears_on_recheck() {
        let mut components = minimal_circuit();
        components.push(resistor(2, "R1", vec![pin("1", "vin"), pin("2", "0")]));

        let mut checker = DrcChecker::new();
        let result = checker.check_connectivity(&components, &[], &[]);
        assert_eq!(of_type(&result, DrcViolationType::DuplicateName).len(), 1);

        components[2].name = "R2".to_string();
        let result = checker.check_connectivity(&components, &[], &[]);
        assert!(of_type(&result, DrcViolationType::DuplicateName).is_empty());
        assert_eq!(result.total_count(), 0);
    }

    #[test]
    fn violation_ids_are_unique_within_a_result() {
        let components = vec![
            vsource(0, "V1", "vin", "x"),
            vsource(1, "V1", "vin", "y"),
            resistor(2, "R1", vec![pin("1", "vin"), pin("2", "z")]),
        ];
        let mut checker = DrcChecker::new();
        let result = checker.check_connectivity(&components, &[], &[]);
        assert!(result.total_count() >= 3);
        let mut ids: Vec<usize> = result.violations().iter().map(|v| v.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), result.total_count());
    }

    #[test]
    fn summary_counts_match_violations() {
        let components = vec![
            // No ground (critical), floating "out" (error), duplicate name (critical).
            resistor(0, "R1", vec![pin("1", "a"), pin("2", "out")]),
            resistor(1, "R1", vec![pin("1", "a"), pin("2", "b")]),
            resistor(2, "R2", vec![pin("1", "a"), pin("2", "b")]),
        ];
        let mut checker = DrcChecker::new();
        let result = checker.check_connectivity(&components, &[], &[]);
        let summary = result.summary();
        assert_eq!(summary.total, result.total_count());
        assert_eq!(summary.critical, 2);
        assert_eq!(summary.errors, 1);
        assert!(!summary.passed);
        assert!(result.has_errors());
    }
}
