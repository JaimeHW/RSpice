//! The design rule checker.
//!
//! Runs the configured rules over an extracted design and collects the
//! violations. Which rules run and at what severity is configuration, so a
//! project can promote a warning to an error.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::input::{ComponentInfo, JunctionInfo, NetLabelInfo, WireInfo};
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
    net_naming_policy: crate::state::NetNamingPolicy,
}

/// DRC configuration options.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DrcConfig {
    /// Check for floating nodes
    pub check_floating_nodes: bool,
    /// Check exact terminal attachment, orphan labels, and conductor chains
    /// that have no real electrical endpoint.
    pub check_unconnected_pins: bool,
    /// Check for missing ground
    pub check_missing_ground: bool,
    /// Check for duplicate names
    pub check_duplicate_names: bool,
    /// Check required parameters and explicit numeric schema ranges.
    pub check_component_parameters: bool,
    /// Check component types whose binding authority is available.
    pub check_unknown_components: bool,
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
            check_component_parameters: true,
            check_unknown_components: true,
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
            net_naming_policy: crate::state::NetNamingPolicy::SpiceCompatibleRelaxed,
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: DrcConfig) -> Self {
        Self {
            next_id: 0,
            config,
            net_naming_policy: crate::state::NetNamingPolicy::SpiceCompatibleRelaxed,
        }
    }

    /// Bind name comparison to the owning schematic document policy.
    pub(crate) fn set_net_naming_policy(&mut self, policy: crate::state::NetNamingPolicy) {
        self.net_naming_policy = policy;
    }

    /// Get the next violation ID
    fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Publish one finding through the configured severity policy. Keeping
    /// this at the checker boundary guarantees every enabled check, including
    /// typed-bus checks, honors the same project override contract.
    fn add_violation(&self, result: &mut DrcResult, mut violation: DrcViolation) {
        if let Some(severity) = self
            .config
            .severity_overrides
            .get(&violation.violation_type)
        {
            violation.severity = *severity;
        }
        result.add_violation(violation);
    }

    /// Run legacy geometry-only DRC checks without explicit junction input.
    ///
    /// Pure interior/interior crossings are disconnected in this compatibility
    /// API. Schematic callers must use [`Self::check_connectivity_with_junctions`]
    /// with the junction-aware extraction bridge.
    #[cfg(test)]
    pub fn check_connectivity(
        &mut self,
        components: &[ComponentInfo],
        wires: &[WireInfo],
        net_labels: &[NetLabelInfo],
    ) -> DrcResult {
        self.check_connectivity_with_junctions(components, wires, net_labels, &[])
    }

    /// Run all DRC checks with the schematic's explicit junction positions.
    ///
    /// A crossing between the interiors of two segments is electrically
    /// connected only when its position appears in `junctions`. Shared
    /// endpoints and endpoint-to-segment (T) contacts remain connected
    /// without an explicit junction, matching standard schematic semantics.
    pub fn check_connectivity_with_junctions(
        &mut self,
        components: &[ComponentInfo],
        wires: &[WireInfo],
        net_labels: &[NetLabelInfo],
        junctions: &[JunctionInfo],
    ) -> DrcResult {
        let start = crate::time_compat::Instant::now();
        self.next_id = 0;
        let mut result = DrcResult::new();

        // Build net connectivity map
        let net_map = self.normalize_net_map_for_policy(
            self.build_net_map(components, wires, net_labels, junctions),
        );

        // Check for duplicate component names
        if self.config.check_duplicate_names {
            self.check_component_references(components, &mut result);
            self.check_duplicate_names(components, &mut result);
        }

        if self.config.check_component_parameters {
            self.check_component_parameters(components, &mut result);
        }

        if self.config.check_unknown_components {
            self.check_unknown_components(components, &mut result);
        }

        if self.config.check_unconnected_pins {
            self.check_unconnected_pins(components, wires, net_labels, &mut result);
            self.check_orphan_net_labels(components, wires, net_labels, &mut result);
            self.check_dangling_wires(&net_map, &mut result);
        }

        // Check for missing ground
        if self.config.check_missing_ground {
            self.check_missing_ground(&net_map, &mut result);
        }

        // Check for floating nodes
        if self.config.check_floating_nodes {
            self.check_floating_nodes(&net_map, &mut result);
        }

        self.check_bus_member_name_conflicts(&net_map, &mut result);

        // Check mutually exclusive driver contracts on the resolved net map.
        if self.config.check_shorted_outputs {
            self.check_source_to_source(&net_map, &mut result);
            self.check_shorted_outputs(&net_map, &mut result);
            self.check_duplicate_bus_member_drivers(&net_map, &mut result);
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
        junctions: &[JunctionInfo],
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
        const MAX_CELLS_PER_SEGMENT: i128 = 4_096;
        let mut cells: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
        let mut long_segments = Vec::new();
        for (index, &(start, end)) in segments.iter().enumerate() {
            let (x0, x1) = (start.x().min(end.x()), start.x().max(end.x()));
            let (y0, y1) = (start.y().min(end.y()), start.y().max(end.y()));
            let (cx0, cx1) = (x0.div_euclid(CELL), x1.div_euclid(CELL));
            let (cy0, cy1) = (y0.div_euclid(CELL), y1.div_euclid(CELL));
            let cell_count =
                (i128::from(cx1) - i128::from(cx0) + 1) * (i128::from(cy1) - i128::from(cy0) + 1);
            if cell_count > MAX_CELLS_PER_SEGMENT {
                // Imported and generated geometry can legally span a very
                // large coordinate range. Indexing every cell in its bounding
                // box would make one diagonal an allocation-time denial of
                // service. A small exact fallback list keeps memory bounded.
                long_segments.push(index);
                continue;
            }
            for cx in cx0..=cx1 {
                for cy in cy0..=cy1 {
                    cells.entry((cx, cy)).or_default().push(index);
                }
            }
        }

        // Merge shared endpoints and endpoint-to-segment contacts. A pure
        // interior crossing is deliberately left disconnected; the explicit
        // junction pass below handles marked crossings.
        let mut candidate_pairs: std::collections::HashSet<(usize, usize)> =
            std::collections::HashSet::new();
        for bucket in cells.values() {
            for (slot, &i) in bucket.iter().enumerate() {
                for &j in &bucket[slot + 1..] {
                    candidate_pairs.insert((i.min(j), i.max(j)));
                }
            }
        }
        for &i in &long_segments {
            for j in 0..segments.len() {
                if i != j {
                    candidate_pairs.insert((i.min(j), i.max(j)));
                }
            }
        }
        let mut candidate_pairs = candidate_pairs.into_iter().collect::<Vec<_>>();
        candidate_pairs.sort_unstable();
        for pair in candidate_pairs {
            if let Some(intersection) =
                segment_intersection_point(segments[pair.0], segments[pair.1])
            {
                let (a0, a1) = segments[pair.0];
                let (b0, b1) = segments[pair.1];
                if intersection != a0
                    && intersection != a1
                    && intersection != b0
                    && intersection != b1
                {
                    continue;
                }
                let p_id =
                    ensure_point_id(intersection, &mut point_ids, &mut points_by_id, &mut dsu);
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

        // Ordinary segments come from the point's spatial cell; exceptionally
        // large segments are appended from the bounded exact fallback.
        let segments_near = |point: PointKey| -> Vec<usize> {
            let mut candidates = cells
                .get(&(point.x().div_euclid(CELL), point.y().div_euclid(CELL)))
                .cloned()
                .unwrap_or_default();
            candidates.extend(long_segments.iter().copied());
            candidates.sort_unstable();
            candidates.dedup();
            candidates
        };

        // A persisted junction connects every segment passing through its
        // position, including two (or more) segment interiors. Repeated
        // junctions are harmless because the disjoint-set union is idempotent.
        for junction in junctions {
            let point = PointKey::from_f64(junction.x, junction.y);
            let point_id = ensure_point_id(point, &mut point_ids, &mut points_by_id, &mut dsu);
            for seg_index in segments_near(point) {
                let (start, end) = segments[seg_index];
                if point_on_segment(point, start, end) {
                    dsu.union(point_id, point_ids[&start]);
                    dsu.union(point_id, point_ids[&end]);
                }
            }
        }

        // Attach component pins to any segment they lie on.
        for comp in components {
            for pin in &comp.pins {
                let (Some(x), Some(y)) = (pin.x, pin.y) else {
                    continue;
                };
                let point = PointKey::from_f64(x, y);
                let point_id = ensure_point_id(point, &mut point_ids, &mut points_by_id, &mut dsu);
                for seg_index in segments_near(point) {
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
            for seg_index in segments_near(point) {
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
            let component_identity = if comp.name.trim().is_empty() {
                format!("component #{}", comp.id)
            } else {
                comp.name.clone()
            };
            for pin in &comp.pins {
                let update_acc = |acc: &mut NetAccumulator| {
                    acc.connection_count += 1;
                    acc.connected_components.insert(component_identity.clone());
                    if !pin.net_name.trim().is_empty() {
                        acc.names.insert(pin.net_name.clone());
                    }
                    if comp.is_voltage_source {
                        acc.has_voltage_source = true;
                    }
                    if comp.is_current_source {
                        acc.has_current_source = true;
                    }
                    if pin.is_output {
                        acc.output_drivers.insert(component_identity.clone());
                        if comp.is_voltage_source {
                            acc.voltage_source_drivers
                                .insert(component_identity.clone());
                        } else {
                            acc.declared_output_drivers
                                .insert(component_identity.clone());
                        }
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
            if label.electrical_anchor {
                acc.electrical_anchor_count = acc.electrical_anchor_count.saturating_add(1);
            }
        }

        // Retain the stable conductor identities in their final electrical
        // clusters. This lets dangling-wire checks follow an entire connected
        // wire chain instead of judging each segment in isolation.
        for (wire, (start, _)) in wires.iter().zip(segments.iter().copied()) {
            let root = dsu.find(point_ids[&start]);
            cluster_accumulators
                .entry(root)
                .or_default()
                .wire_ids
                .insert(wire.id);
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

    fn normalize_net_map_for_policy(
        &self,
        net_map: HashMap<String, NetInfo>,
    ) -> HashMap<String, NetInfo> {
        if self.net_naming_policy == crate::state::NetNamingPolicy::StrictCaseSensitive {
            return net_map;
        }

        let mut entries: Vec<_> = net_map.into_iter().collect();
        entries.sort_by(|(left, _), (right, _)| {
            left.to_ascii_lowercase()
                .cmp(&right.to_ascii_lowercase())
                .then_with(|| left.cmp(right))
        });
        let mut merged_by_normalized: HashMap<String, (String, NetInfo)> = HashMap::new();
        for (name, net) in entries {
            let normalized = name.to_ascii_lowercase();
            let (_, merged) = merged_by_normalized
                .entry(normalized)
                .or_insert_with(|| (name, NetInfo::default()));
            merged.names.extend(net.names);
            merged.connection_count = merged.connection_count.saturating_add(net.connection_count);
            merged.has_voltage_source |= net.has_voltage_source;
            merged.has_current_source |= net.has_current_source;
            merged.is_ground |= net.is_ground;
            merged.connected_components.extend(net.connected_components);
            merged.output_drivers.extend(net.output_drivers);
            merged
                .voltage_source_drivers
                .extend(net.voltage_source_drivers);
            merged
                .declared_output_drivers
                .extend(net.declared_output_drivers);
            merged.wire_ids.extend(net.wire_ids);
            merged.electrical_anchor_count = merged
                .electrical_anchor_count
                .saturating_add(net.electrical_anchor_count);
        }

        merged_by_normalized
            .into_values()
            .map(|(name, mut net)| {
                net.connected_components.sort();
                net.connected_components.dedup();
                (name, net)
            })
            .collect()
    }

    fn check_component_references(&mut self, components: &[ComponentInfo], result: &mut DrcResult) {
        let mut components = components.iter().collect::<Vec<_>>();
        components.sort_by_key(|component| component.id);
        for component in components {
            if !component.reference_required {
                continue;
            }
            if component.name.trim().is_empty() {
                let id = self.next_id();
                self.add_violation(
                    result,
                    DrcViolation::new(
                        id,
                        DrcViolationType::EmptyName,
                        format!(
                            "{} component #{} has no reference designator",
                            component.component_type, component.id
                        ),
                        DrcLocation::Component {
                            id: component.id,
                            name: String::new(),
                        },
                    ),
                );
            } else if let Some(error) = component.reference_error.as_deref() {
                let id = self.next_id();
                self.add_violation(
                    result,
                    DrcViolation::new(
                        id,
                        DrcViolationType::InvalidName,
                        format!("Invalid reference '{}': {error}", component.name),
                        DrcLocation::Component {
                            id: component.id,
                            name: component.name.clone(),
                        },
                    ),
                );
            }
        }
    }

    fn check_component_parameters(&mut self, components: &[ComponentInfo], result: &mut DrcResult) {
        let mut components = components.iter().collect::<Vec<_>>();
        components.sort_by_key(|component| component.id);
        for component in components {
            let component_name = component_identity(component);
            let mut missing_parameters = component.missing_parameters.iter().collect::<Vec<_>>();
            missing_parameters.sort();
            for parameter in missing_parameters {
                let id = self.next_id();
                self.add_violation(
                    result,
                    DrcViolation::new(
                        id,
                        DrcViolationType::MissingParameter,
                        format!(
                            "{} is missing required parameter '{}'",
                            component_name, parameter
                        ),
                        DrcLocation::Component {
                            id: component.id,
                            name: component.name.clone(),
                        },
                    )
                    .with_related(vec![component_name.clone(), (*parameter).clone()]),
                );
            }
            let mut range_issues = component.out_of_range_parameters.iter().collect::<Vec<_>>();
            range_issues.sort_by(|left, right| left.name.cmp(&right.name));
            for issue in range_issues {
                let expected = match (issue.min, issue.max) {
                    (Some(min), Some(max)) => format!("between {min} and {max}"),
                    (Some(min), None) => format!("at least {min}"),
                    (None, Some(max)) => format!("at most {max}"),
                    (None, None) => continue,
                };
                let id = self.next_id();
                self.add_violation(
                    result,
                    DrcViolation::new(
                        id,
                        DrcViolationType::ValueOutOfRange,
                        format!(
                            "{} parameter '{}' is {} but must be {}",
                            component_name, issue.display_name, issue.value, expected
                        ),
                        DrcLocation::Component {
                            id: component.id,
                            name: component.name.clone(),
                        },
                    )
                    .with_related(vec![component_name.clone(), issue.name.clone()]),
                );
            }
        }
    }

    fn check_unknown_components(&mut self, components: &[ComponentInfo], result: &mut DrcResult) {
        let mut components = components
            .iter()
            .filter(|component| component.component_known == Some(false))
            .collect::<Vec<_>>();
        components.sort_by_key(|component| component.id);
        for component in components {
            let component_name = component_identity(component);
            let id = self.next_id();
            self.add_violation(
                result,
                DrcViolation::new(
                    id,
                    DrcViolationType::UnknownComponent,
                    format!(
                        "{} has no resolvable source, model template, or hierarchy master",
                        component_name
                    ),
                    DrcLocation::Component {
                        id: component.id,
                        name: component.name.clone(),
                    },
                )
                .with_related(vec![component_name]),
            );
        }
    }

    fn check_unconnected_pins(
        &mut self,
        components: &[ComponentInfo],
        wires: &[WireInfo],
        net_labels: &[NetLabelInfo],
        result: &mut DrcResult,
    ) {
        let pin_points = components
            .iter()
            .flat_map(|component| {
                component
                    .pins
                    .iter()
                    .filter_map(|pin| Some(PointKey::from_f64(pin.x?, pin.y?)))
            })
            .fold(HashMap::<PointKey, usize>::new(), |mut counts, point| {
                *counts.entry(point).or_default() += 1;
                counts
            });
        let mut ordered_components = components.iter().collect::<Vec<_>>();
        ordered_components.sort_by_key(|component| component.id);
        for component in ordered_components {
            let mut pins = component.pins.iter().collect::<Vec<_>>();
            pins.sort_by(|left, right| {
                left.name
                    .cmp(&right.name)
                    .then_with(|| {
                        left.x
                            .unwrap_or(f64::NEG_INFINITY)
                            .total_cmp(&right.x.unwrap_or(f64::NEG_INFINITY))
                    })
                    .then_with(|| {
                        left.y
                            .unwrap_or(f64::NEG_INFINITY)
                            .total_cmp(&right.y.unwrap_or(f64::NEG_INFINITY))
                    })
            });
            for pin in pins {
                let (Some(x), Some(y)) = (pin.x, pin.y) else {
                    continue;
                };
                let point = PointKey::from_f64(x, y);
                let on_wire = wires.iter().any(|wire| {
                    point_on_segment(
                        point,
                        PointKey::from_f64(wire.start_x, wire.start_y),
                        PointKey::from_f64(wire.end_x, wire.end_y),
                    )
                });
                let direct_pin_contact = pin_points.get(&point).copied().unwrap_or_default() > 1;
                let anchored = net_labels
                    .iter()
                    .any(|label| PointKey::from_f64(label.x, label.y) == point);
                if on_wire || direct_pin_contact || anchored {
                    continue;
                }
                let id = self.next_id();
                let component_name = if component.name.trim().is_empty() {
                    format!("component #{}", component.id)
                } else {
                    component.name.clone()
                };
                self.add_violation(
                    result,
                    DrcViolation::new(
                        id,
                        DrcViolationType::UnconnectedPin,
                        format!("Pin {}.{} is not connected", component_name, pin.name),
                        DrcLocation::Component {
                            id: component.id,
                            name: component.name.clone(),
                        },
                    )
                    .with_related(vec![component_name, pin.name.clone()]),
                );
            }
        }
    }

    fn check_orphan_net_labels(
        &mut self,
        components: &[ComponentInfo],
        wires: &[WireInfo],
        net_labels: &[NetLabelInfo],
        result: &mut DrcResult,
    ) {
        let pin_points = components
            .iter()
            .flat_map(|component| component.pins.iter())
            .filter_map(|pin| Some(PointKey::from_f64(pin.x?, pin.y?)))
            .collect::<std::collections::HashSet<_>>();
        let mut labels = net_labels
            .iter()
            .filter(|label| !label.synthetic)
            .collect::<Vec<_>>();
        labels.sort_by(|left, right| {
            left.name
                .cmp(&right.name)
                .then_with(|| left.x.total_cmp(&right.x))
                .then_with(|| left.y.total_cmp(&right.y))
        });
        for label in labels {
            let point = PointKey::from_f64(label.x, label.y);
            let on_wire = wires.iter().any(|wire| {
                point_on_segment(
                    point,
                    PointKey::from_f64(wire.start_x, wire.start_y),
                    PointKey::from_f64(wire.end_x, wire.end_y),
                )
            });
            if on_wire || pin_points.contains(&point) {
                continue;
            }
            let id = self.next_id();
            self.add_violation(
                result,
                DrcViolation::new(
                    id,
                    DrcViolationType::OrphanNetLabel,
                    format!(
                        "Net label '{}' is not attached to a conductor or pin",
                        label.name
                    ),
                    DrcLocation::NetLabel {
                        name: label.name.clone(),
                    },
                ),
            );
        }
    }

    fn check_dangling_wires(&mut self, net_map: &HashMap<String, NetInfo>, result: &mut DrcResult) {
        let mut dangling = std::collections::BTreeMap::<u64, String>::new();
        let mut net_names = net_map.keys().collect::<Vec<_>>();
        net_names.sort();
        for net_name in net_names {
            let net = &net_map[net_name];
            if net.connection_count != 0 || net.electrical_anchor_count != 0 {
                continue;
            }
            for wire_id in &net.wire_ids {
                dangling.entry(*wire_id).or_insert_with(|| net_name.clone());
            }
        }
        for (wire_id, net_name) in dangling {
            let id = self.next_id();
            self.add_violation(
                result,
                DrcViolation::new(
                    id,
                    DrcViolationType::DanglingWire,
                    format!("Wire #{wire_id} has no component or typed-bus endpoint"),
                    DrcLocation::Wire { id: wire_id },
                )
                .with_related(vec![net_name]),
            );
        }
    }

    /// Check for duplicate component names
    fn check_duplicate_names(&mut self, components: &[ComponentInfo], result: &mut DrcResult) {
        let mut names: std::collections::BTreeMap<String, Vec<usize>> =
            std::collections::BTreeMap::new();

        for (idx, comp) in components.iter().enumerate() {
            if !comp.name.trim().is_empty() {
                names.entry(comp.name.clone()).or_default().push(idx);
            }
        }

        for (name, mut indices) in names {
            if indices.len() > 1 {
                indices.sort_by_key(|index| components[*index].id);
                let id = self.next_id();
                self.add_violation(
                    result,
                    DrcViolation::new(
                        id,
                        DrcViolationType::DuplicateName,
                        format!(
                            "Duplicate component name '{}' ({} instances)",
                            name,
                            indices.len()
                        ),
                        DrcLocation::Component {
                            id: components[indices[0]].id,
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
            self.add_violation(
                result,
                DrcViolation::new(
                    id,
                    DrcViolationType::MissingGround,
                    "Circuit has no ground reference (node 0 or GND)",
                    DrcLocation::Global,
                ),
            );
        }
    }

    /// Check for floating nodes
    fn check_floating_nodes(&mut self, net_map: &HashMap<String, NetInfo>, result: &mut DrcResult) {
        let mut names = net_map.keys().collect::<Vec<_>>();
        names.sort();
        for name in names {
            let net = &net_map[name];
            if net.connection_count == 0 {
                continue;
            }
            if net.connection_count < self.config.min_connections && !net.is_ground {
                let id = self.next_id();
                self.add_violation(
                    result,
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

    /// Check for directly connected independent voltage-source outputs.
    fn check_source_to_source(
        &mut self,
        net_map: &HashMap<String, NetInfo>,
        result: &mut DrcResult,
    ) {
        let mut names = net_map.keys().collect::<Vec<_>>();
        names.sort();
        for net_name in names {
            let net = &net_map[net_name];
            let mut sources = net
                .voltage_source_drivers
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            sources.sort();
            if sources.len() > 1 {
                let id = self.next_id();
                self.add_violation(
                    result,
                    DrcViolation::new(
                        id,
                        DrcViolationType::SourceToSource,
                        format!(
                            "Net '{}' directly connects {} voltage-source outputs: {}",
                            net_name,
                            sources.len(),
                            sources.join(", ")
                        ),
                        DrcLocation::Node {
                            net_name: net_name.clone(),
                        },
                    )
                    .with_related(sources),
                );
            }
        }
    }

    /// Check for multiple hierarchy-declared output pins on one scalar net.
    fn check_shorted_outputs(
        &mut self,
        net_map: &HashMap<String, NetInfo>,
        result: &mut DrcResult,
    ) {
        let mut names = net_map.keys().collect::<Vec<_>>();
        names.sort();
        for net_name in names {
            let net = &net_map[net_name];
            let mut drivers = net
                .declared_output_drivers
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            drivers.sort();
            if drivers.len() <= 1 {
                continue;
            }
            let id = self.next_id();
            self.add_violation(
                result,
                DrcViolation::new(
                    id,
                    DrcViolationType::ShortedOutputs,
                    format!(
                        "Net '{}' has {} declared output drivers: {}",
                        net_name,
                        drivers.len(),
                        drivers.join(", ")
                    ),
                    DrcLocation::Node {
                        net_name: net_name.clone(),
                    },
                )
                .with_related(drivers),
            );
        }
    }

    fn check_duplicate_bus_member_drivers(
        &mut self,
        net_map: &HashMap<String, NetInfo>,
        result: &mut DrcResult,
    ) {
        let mut net_names = net_map.keys().collect::<Vec<_>>();
        net_names.sort();
        for net_name in net_names {
            let net = &net_map[net_name];
            let is_scalar_bus_member =
                crate::state::BusSlice::parse(net_name).is_ok_and(|slice| slice.is_scalar());
            if !is_scalar_bus_member || net.output_drivers.len() <= 1 {
                continue;
            }
            let mut drivers: Vec<_> = net.output_drivers.iter().cloned().collect();
            drivers.sort();
            let id = self.next_id();
            self.add_violation(
                result,
                DrcViolation::new(
                    id,
                    DrcViolationType::DuplicateBusMemberDriver,
                    format!(
                        "Typed bus member '{}' has {} output drivers: {}",
                        net_name,
                        drivers.len(),
                        drivers.join(", ")
                    ),
                    DrcLocation::Node {
                        net_name: net_name.clone(),
                    },
                )
                .with_related(drivers),
            );
        }
    }

    fn check_bus_member_name_conflicts(
        &mut self,
        net_map: &HashMap<String, NetInfo>,
        result: &mut DrcResult,
    ) {
        let mut net_names = net_map.keys().collect::<Vec<_>>();
        net_names.sort();
        for net_name in net_names {
            let net = &net_map[net_name];
            let mut typed_names: Vec<String> = net
                .names
                .iter()
                .filter(|name| {
                    crate::state::BusSlice::parse(name).is_ok_and(|slice| slice.is_scalar())
                })
                .cloned()
                .collect();
            typed_names.sort();
            typed_names.dedup();
            let Some(typed_name) = typed_names.first() else {
                continue;
            };

            let mut conflicts: Vec<String> = net
                .names
                .iter()
                .filter(|name| {
                    !is_auto_generated_net_name(name)
                        && match self.net_naming_policy {
                            crate::state::NetNamingPolicy::StrictCaseSensitive => {
                                *name != typed_name
                            }
                            crate::state::NetNamingPolicy::SpiceCompatibleRelaxed => {
                                !name.eq_ignore_ascii_case(typed_name)
                            }
                        }
                })
                .cloned()
                .collect();
            conflicts.sort();
            conflicts.dedup();
            if conflicts.is_empty() {
                continue;
            }

            let mut related = vec![typed_name.clone()];
            related.extend(conflicts.iter().cloned());
            let id = self.next_id();
            self.add_violation(
                result,
                DrcViolation::new(
                    id,
                    DrcViolationType::BusRangeConflict,
                    format!(
                        "Typed bus member '{}' conflicts with net name{} {}",
                        typed_name,
                        if conflicts.len() == 1 { "" } else { "s" },
                        conflicts
                            .iter()
                            .map(|name| format!("'{name}'"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    DrcLocation::Node {
                        net_name: typed_name.clone(),
                    },
                )
                .with_related(related),
            );
        }
    }
}

/// Report every off-sheet connector whose declared name no second connector
/// repeats.
///
/// A connector is a claim that its name continues elsewhere in the cellview.
/// One standing alone is either a crossing that was never completed or a label
/// that should not have claimed one; netlisting is unaffected either way, which
/// is why this is stated rather than failed. The rule counts declarations
/// rather than sheets because the name, not the page, is what joins nets.
pub(super) fn append_off_sheet_connector_violations(
    schematic: &crate::state::SchematicState,
    result: &mut DrcResult,
    severity_overrides: &HashMap<DrcViolationType, DrcSeverity>,
) {
    let policy = schematic.document_policy.net_naming;
    let key = |name: &str| match policy {
        crate::state::NetNamingPolicy::StrictCaseSensitive => name.to_owned(),
        crate::state::NetNamingPolicy::SpiceCompatibleRelaxed => name.to_ascii_lowercase(),
    };

    let mut declarations: HashMap<String, usize> = HashMap::new();
    for label in &schematic.net_labels {
        if label.kind.off_sheet_direction().is_some() {
            *declarations.entry(key(&label.name)).or_default() += 1;
        }
    }

    let mut next_id = result.total_count();
    for label in &schematic.net_labels {
        if label.kind.off_sheet_direction().is_none()
            || declarations.get(&key(&label.name)).copied() != Some(1)
        {
            continue;
        }
        let mut violation = DrcViolation::new(
            next_id,
            DrcViolationType::OffSheetConnectorWithoutPartner,
            format!(
                "Off-sheet connector `{}` has no partner on another sheet.",
                label.name
            ),
            DrcLocation::NetLabel {
                name: label.name.clone(),
            },
        );
        if let Some(severity) =
            severity_overrides.get(&DrcViolationType::OffSheetConnectorWithoutPartner)
        {
            violation.severity = *severity;
        }
        result.add_violation(violation);
        next_id += 1;
    }
}

fn component_identity(component: &ComponentInfo) -> String {
    if component.name.trim().is_empty() {
        format!("component #{}", component.id)
    } else {
        component.name.clone()
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
    use crate::services::drc::input::{ParameterRangeIssue, PinInfo};

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

    fn resistor(id: u64, name: &str, pins: Vec<PinInfo>) -> ComponentInfo {
        ComponentInfo {
            id,
            name: name.to_string(),
            component_type: "resistor".to_string(),
            pins,
            is_voltage_source: false,
            is_current_source: false,
            reference_required: true,
            reference_error: None,
            component_known: Some(true),
            missing_parameters: Vec::new(),
            out_of_range_parameters: Vec::new(),
        }
    }

    fn vsource(id: u64, name: &str, plus_net: &str, minus_net: &str) -> ComponentInfo {
        let mut plus = pin("+", plus_net);
        plus.is_output = true;
        ComponentInfo {
            id,
            name: name.to_string(),
            component_type: "voltage_source".to_string(),
            pins: vec![plus, pin("-", minus_net)],
            is_voltage_source: true,
            is_current_source: false,
            reference_required: true,
            reference_error: None,
            component_known: Some(true),
            missing_parameters: Vec::new(),
            out_of_range_parameters: Vec::new(),
        }
    }

    fn wire(id: u64, x0: f64, y0: f64, x1: f64, y1: f64) -> WireInfo {
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
            synthetic: false,
            electrical_anchor: false,
        }
    }

    fn of_type(result: &DrcResult, vt: DrcViolationType) -> Vec<&DrcViolation> {
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
    fn auto_named_single_connection_nets_are_reported_floating() {
        // A pin with coordinates but no net name lands in a geometry cluster
        // whose canonical name is auto-generated; it is still a real dangling
        // schematic connection and should be reported.
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
        let floating = of_type(&result, DrcViolationType::FloatingNode);
        assert_eq!(floating.len(), 1);
        assert_eq!(
            floating[0].location,
            DrcLocation::Node {
                net_name: "net_300_300".to_string()
            }
        );
        assert_eq!(floating[0].related_items, vec!["R2".to_string()]);
    }

    #[test]
    fn explicit_net_label_starting_with_net_prefix_is_reported_floating() {
        // User labels may intentionally start with "net_"; string shape alone
        // must not cause DRC to treat them as generated and suppress them.
        let components = vec![
            vsource(0, "V1", "vin", "0"),
            resistor(1, "R1", vec![pin("1", "vin"), pin("2", "0")]),
            resistor(2, "R2", vec![pin("1", "vin"), pin_at("2", "", 0.0, 200.0)]),
        ];
        let labels = vec![label("net_out", 0.0, 200.0)];
        let mut checker = DrcChecker::new();
        let result = checker.check_connectivity(&components, &[], &labels);
        let floating = of_type(&result, DrcViolationType::FloatingNode);
        assert_eq!(floating.len(), 1);
        assert_eq!(
            floating[0].location,
            DrcLocation::Node {
                net_name: "net_out".to_string()
            }
        );
        assert_eq!(floating[0].related_items, vec!["R2".to_string()]);
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
    fn two_voltage_sources_on_one_net_are_source_to_source() {
        let components = vec![
            vsource(0, "V1", "vin", "0"),
            vsource(1, "V2", "vin", "0"),
            resistor(2, "R1", vec![pin("1", "vin"), pin("2", "0")]),
        ];
        let mut checker = DrcChecker::new();
        let result = checker.check_connectivity(&components, &[], &[]);
        let shorted = of_type(&result, DrcViolationType::SourceToSource);
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
        assert!(of_type(&result, DrcViolationType::SourceToSource).is_empty());
        assert!(result.passed());
    }

    #[test]
    fn hierarchy_declared_outputs_share_final_connectivity_before_short_check() {
        let mut first = resistor(1, "X1", vec![pin_at("OUT", "", 0.0, 0.0)]);
        first.pins[0].is_output = true;
        let mut second = resistor(2, "X2", vec![pin_at("OUT", "", 100.0, 0.0)]);
        second.pins[0].is_output = true;
        let config = DrcConfig {
            check_missing_ground: false,
            check_floating_nodes: false,
            ..DrcConfig::default()
        };
        let mut checker = DrcChecker::with_config(config);
        let result =
            checker.check_connectivity(&[second, first], &[wire(7, 0.0, 0.0, 100.0, 0.0)], &[]);
        let shorted = of_type(&result, DrcViolationType::ShortedOutputs);
        assert_eq!(shorted.len(), 1);
        assert_eq!(
            shorted[0].related_items,
            vec!["X1".to_owned(), "X2".to_owned()]
        );
    }

    #[test]
    fn unconnected_pin_check_uses_exact_geometry_and_honors_its_gate() {
        let components = vec![resistor(7, "R7", vec![pin_at("1", "", 10.0, 20.0)])];
        let base = DrcConfig {
            check_missing_ground: false,
            check_floating_nodes: false,
            ..DrcConfig::default()
        };
        let mut checker = DrcChecker::with_config(base.clone());
        let result = checker.check_connectivity(&components, &[], &[]);
        let unconnected = of_type(&result, DrcViolationType::UnconnectedPin);
        assert_eq!(unconnected.len(), 1);
        assert_eq!(
            unconnected[0].location,
            DrcLocation::Component {
                id: 7,
                name: "R7".to_owned()
            }
        );

        let mut disabled = base;
        disabled.check_unconnected_pins = false;
        let mut checker = DrcChecker::with_config(disabled);
        let result = checker.check_connectivity(&components, &[], &[]);
        assert!(of_type(&result, DrcViolationType::UnconnectedPin).is_empty());
    }

    #[test]
    fn wire_or_direct_pin_contact_satisfies_terminal_attachment() {
        let on_wire = vec![resistor(1, "R1", vec![pin_at("1", "", 10.0, 0.0)])];
        let touching = vec![
            resistor(1, "R1", vec![pin_at("1", "", 5.0, 5.0)]),
            resistor(2, "R2", vec![pin_at("1", "", 5.0, 5.0)]),
        ];
        let config = DrcConfig {
            check_missing_ground: false,
            check_floating_nodes: false,
            ..DrcConfig::default()
        };

        let mut checker = DrcChecker::with_config(config.clone());
        let wired = checker.check_connectivity(&on_wire, &[wire(9, 0.0, 0.0, 20.0, 0.0)], &[]);
        assert!(of_type(&wired, DrcViolationType::UnconnectedPin).is_empty());

        let mut checker = DrcChecker::with_config(config);
        let direct = checker.check_connectivity(&touching, &[], &[]);
        assert!(of_type(&direct, DrcViolationType::UnconnectedPin).is_empty());
    }

    #[test]
    fn orphan_labels_and_dangling_wire_chains_are_exact_and_deterministic() {
        let config = DrcConfig {
            check_missing_ground: false,
            check_floating_nodes: false,
            ..DrcConfig::default()
        };
        let wires = vec![
            wire(20, 0.0, 0.0, 20.0, 0.0),
            wire(10, 20.0, 0.0, 40.0, 0.0),
        ];
        let labels = vec![label("ON_WIRE", 10.0, 0.0), label("ORPHAN", 100.0, 100.0)];
        let mut checker = DrcChecker::with_config(config.clone());
        let result = checker.check_connectivity(&[], &wires, &labels);

        let orphan = of_type(&result, DrcViolationType::OrphanNetLabel);
        assert_eq!(orphan.len(), 1);
        assert_eq!(
            orphan[0].location,
            DrcLocation::NetLabel {
                name: "ORPHAN".to_owned()
            }
        );
        let dangling = of_type(&result, DrcViolationType::DanglingWire);
        assert_eq!(
            dangling
                .iter()
                .map(|finding| match &finding.location {
                    DrcLocation::Wire { id } => *id,
                    _ => unreachable!("wire finding"),
                })
                .collect::<Vec<_>>(),
            vec![10, 20]
        );

        let mut disabled = config;
        disabled.check_unconnected_pins = false;
        let mut checker = DrcChecker::with_config(disabled);
        let result = checker.check_connectivity(&[], &wires, &labels);
        assert!(of_type(&result, DrcViolationType::OrphanNetLabel).is_empty());
        assert!(of_type(&result, DrcViolationType::DanglingWire).is_empty());
    }

    #[test]
    fn exact_reference_contract_emits_empty_and_invalid_names_only_when_required() {
        let mut empty = resistor(3, "", vec![]);
        let mut invalid = resistor(4, "C4", vec![]);
        invalid.reference_error = Some("Resistor designators must begin with `R`.".to_owned());
        let mut structural = resistor(5, "", vec![]);
        structural.reference_required = false;
        structural.component_type = "port".to_owned();
        empty.reference_error = None;
        let config = DrcConfig {
            check_missing_ground: false,
            check_floating_nodes: false,
            check_unconnected_pins: false,
            ..DrcConfig::default()
        };
        let mut checker = DrcChecker::with_config(config);
        let result = checker.check_connectivity(&[empty, invalid, structural], &[], &[]);
        assert_eq!(of_type(&result, DrcViolationType::EmptyName).len(), 1);
        assert_eq!(of_type(&result, DrcViolationType::InvalidName).len(), 1);
        assert_eq!(of_type(&result, DrcViolationType::DuplicateName).len(), 0);
    }

    #[test]
    fn schema_parameter_findings_are_structured_gated_and_overrideable() {
        let mut component = resistor(8, "R8", vec![]);
        component.missing_parameters = vec!["Resistance".to_owned()];
        component.out_of_range_parameters = vec![ParameterRangeIssue {
            name: "m".to_owned(),
            display_name: "Multiplier".to_owned(),
            value: 0.0,
            min: Some(1.0),
            max: Some(10_000.0),
        }];
        let mut config = DrcConfig {
            check_missing_ground: false,
            check_floating_nodes: false,
            check_unconnected_pins: false,
            ..DrcConfig::default()
        };
        config
            .severity_overrides
            .insert(DrcViolationType::ValueOutOfRange, DrcSeverity::Critical);
        let mut checker = DrcChecker::with_config(config.clone());
        let result = checker.check_connectivity(&[component.clone()], &[], &[]);

        let missing = of_type(&result, DrcViolationType::MissingParameter);
        assert_eq!(missing.len(), 1);
        assert!(missing[0].message.contains("Resistance"));
        let range = of_type(&result, DrcViolationType::ValueOutOfRange);
        assert_eq!(range.len(), 1);
        assert_eq!(range[0].severity, DrcSeverity::Critical);
        assert!(range[0].message.contains("between 1 and 10000"));

        config.check_component_parameters = false;
        let mut checker = DrcChecker::with_config(config);
        let disabled = checker.check_connectivity(&[component], &[], &[]);
        assert!(of_type(&disabled, DrcViolationType::MissingParameter).is_empty());
        assert!(of_type(&disabled, DrcViolationType::ValueOutOfRange).is_empty());
    }

    #[test]
    fn unknown_component_requires_authoritative_resolution_and_honors_gate() {
        let mut unresolved = resistor(9, "X9", vec![]);
        unresolved.component_known = Some(false);
        let mut unknown_authority = resistor(10, "X10", vec![]);
        unknown_authority.component_known = None;
        let config = DrcConfig {
            check_missing_ground: false,
            check_floating_nodes: false,
            check_unconnected_pins: false,
            ..DrcConfig::default()
        };
        let mut checker = DrcChecker::with_config(config.clone());
        let result = checker.check_connectivity(&[unknown_authority, unresolved.clone()], &[], &[]);
        let unknown = of_type(&result, DrcViolationType::UnknownComponent);
        assert_eq!(unknown.len(), 1);
        assert_eq!(
            unknown[0].location,
            DrcLocation::Component {
                id: 9,
                name: "X9".to_owned()
            }
        );

        let mut disabled = config;
        disabled.check_unknown_components = false;
        let mut checker = DrcChecker::with_config(disabled);
        let result = checker.check_connectivity(&[unresolved], &[], &[]);
        assert!(of_type(&result, DrcViolationType::UnknownComponent).is_empty());
    }

    #[test]
    fn new_connectivity_findings_honor_severity_overrides() {
        let mut config = DrcConfig {
            check_missing_ground: false,
            check_floating_nodes: false,
            ..DrcConfig::default()
        };
        config
            .severity_overrides
            .insert(DrcViolationType::UnconnectedPin, DrcSeverity::Critical);
        let mut checker = DrcChecker::with_config(config);
        let result = checker.check_connectivity(
            &[resistor(1, "R1", vec![pin_at("1", "", 0.0, 0.0)])],
            &[],
            &[],
        );
        assert_eq!(
            of_type(&result, DrcViolationType::UnconnectedPin)[0].severity,
            DrcSeverity::Critical
        );
    }

    #[test]
    fn finding_order_and_ids_are_repeatable() {
        let components = vec![
            resistor(2, "R2", vec![pin("1", "z")]),
            resistor(1, "R1", vec![pin("1", "a")]),
            resistor(3, "R1", vec![pin("1", "b")]),
        ];
        let capture = |components: &[ComponentInfo]| {
            let mut checker = DrcChecker::new();
            checker
                .check_connectivity(components, &[], &[])
                .violations()
                .iter()
                .map(|finding| {
                    (
                        finding.id,
                        finding.violation_type,
                        finding.location.display(),
                        finding.message.clone(),
                    )
                })
                .collect::<Vec<_>>()
        };
        let reversed = components.iter().cloned().rev().collect::<Vec<_>>();
        assert_eq!(capture(&components), capture(&reversed));
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
    fn unmarked_mid_segment_crossing_stays_disconnected() {
        // The segments cross in both interiors. Without a persisted junction,
        // each side remains a one-pin floating net.
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
        assert_eq!(of_type(&result, DrcViolationType::FloatingNode).len(), 2);
    }

    #[test]
    fn explicit_junction_connects_mid_segment_crossing() {
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
        let result = checker.check_connectivity_with_junctions(
            &components,
            &wires,
            &labels,
            &[JunctionInfo::new(0.0, 0.0)],
        );
        assert!(
            of_type(&result, DrcViolationType::FloatingNode).is_empty(),
            "{:?}",
            result.violations()
        );
    }

    #[test]
    fn endpoint_to_segment_contact_connects_without_junction() {
        let components = vec![
            resistor(0, "R1", vec![pin_at("1", "", -100.0, 0.0), pin("2", "0")]),
            resistor(1, "R2", vec![pin_at("1", "", 0.0, 100.0), pin("2", "0")]),
        ];
        let wires = vec![
            wire(0, -100.0, 0.0, 100.0, 0.0),
            wire(1, 0.0, 0.0, 0.0, 100.0),
        ];
        let mut checker = DrcChecker::new();
        let result = checker.check_connectivity(&components, &wires, &[]);
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
            resistor(2, "R2", vec![pin("1", "vin"), pin_at("2", "", 0.0, 200.0)]),
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

    #[test]
    fn configured_severity_override_is_applied_to_checker_findings() {
        let mut config = DrcConfig::default();
        config
            .severity_overrides
            .insert(DrcViolationType::MissingGround, DrcSeverity::Info);
        let mut checker = DrcChecker::with_config(config);

        let result = checker.check_connectivity(&[], &[], &[]);

        assert_eq!(result.total_count(), 1);
        assert_eq!(result.violations()[0].severity, DrcSeverity::Info);
        assert!(result.passed());
    }

    #[test]
    fn relaxed_policy_merges_case_variant_typed_members_before_driver_checks() {
        let components = vec![
            vsource(1, "V1", "DATA[3]", "0"),
            vsource(2, "V2", "data[3]", "0"),
        ];
        let config = DrcConfig {
            check_missing_ground: false,
            check_floating_nodes: false,
            ..DrcConfig::default()
        };

        let mut relaxed = DrcChecker::with_config(config.clone());
        relaxed.set_net_naming_policy(crate::state::NetNamingPolicy::SpiceCompatibleRelaxed);
        let relaxed_result = relaxed.check_connectivity(&components, &[], &[]);
        assert!(relaxed_result.violations().iter().any(|violation| {
            violation.violation_type == DrcViolationType::DuplicateBusMemberDriver
        }));

        let mut strict = DrcChecker::with_config(config);
        strict.set_net_naming_policy(crate::state::NetNamingPolicy::StrictCaseSensitive);
        let strict_result = strict.check_connectivity(&components, &[], &[]);
        assert!(!strict_result.violations().iter().any(|violation| {
            violation.violation_type == DrcViolationType::DuplicateBusMemberDriver
        }));
    }

    #[test]
    fn extreme_diagonal_uses_bounded_index_and_retains_exact_attachments() {
        let components = vec![resistor(1, "R1", vec![pin_at("1", "", 0.0, 0.0)])];
        let wires = vec![WireInfo {
            id: 1,
            start_x: f64::from(i32::MIN),
            start_y: f64::from(i32::MIN),
            end_x: f64::from(i32::MAX),
            end_y: f64::from(i32::MAX),
        }];
        let labels = vec![NetLabelInfo {
            name: "LONG_DIAGONAL".to_owned(),
            x: 100.0,
            y: 100.0,
            synthetic: false,
            electrical_anchor: false,
        }];

        let checker = DrcChecker::new();
        let nets = checker.build_net_map(&components, &wires, &labels, &[]);
        let net = nets
            .get("LONG_DIAGONAL")
            .expect("label and pin attach through the exact long-segment fallback");
        assert_eq!(net.connection_count, 1);
        assert!(net.connected_components.contains(&"R1".to_owned()));
    }

    fn off_sheet_findings(schematic: &crate::state::SchematicState) -> Vec<DrcViolation> {
        let mut result = DrcResult::new();
        append_off_sheet_connector_violations(schematic, &mut result, &HashMap::new());
        result.violations().to_vec()
    }

    #[test]
    fn a_lone_off_sheet_connector_is_advised_and_a_paired_one_is_not() {
        use crate::state::{CrossSheetPortDirection, NetLabel, Point};

        let mut schematic = crate::state::SchematicState::default();
        schematic.net_labels.push(NetLabel::off_sheet(
            1,
            Point::origin(),
            "BIAS",
            CrossSheetPortDirection::Output,
        ));
        schematic.net_labels.push(NetLabel::off_sheet(
            2,
            Point::new(1_000_000, 0),
            "SENSE",
            CrossSheetPortDirection::Input,
        ));
        schematic.net_labels.push(NetLabel::off_sheet(
            3,
            Point::new(2_000_000, 0),
            "SENSE",
            CrossSheetPortDirection::Output,
        ));
        // A local label of the same name is not a partner: it makes no
        // crossing claim of its own.
        schematic
            .net_labels
            .push(NetLabel::new(4, Point::new(40, 0), "BIAS"));

        let findings = off_sheet_findings(&schematic);
        assert_eq!(findings.len(), 1);
        assert_eq!(
            findings[0].violation_type,
            DrcViolationType::OffSheetConnectorWithoutPartner
        );
        assert_eq!(findings[0].severity, DrcSeverity::Info);
        assert_eq!(
            findings[0].message,
            "Off-sheet connector `BIAS` has no partner on another sheet."
        );
        assert_eq!(
            findings[0].location,
            DrcLocation::NetLabel {
                name: "BIAS".to_owned()
            }
        );
    }

    #[test]
    fn partner_matching_follows_the_document_naming_policy_and_severity_overrides() {
        use crate::state::{CrossSheetPortDirection, NetLabel, NetNamingPolicy, Point};

        let mut schematic = crate::state::SchematicState::default();
        schematic.net_labels.push(NetLabel::off_sheet(
            1,
            Point::origin(),
            "bias",
            CrossSheetPortDirection::Output,
        ));
        schematic.net_labels.push(NetLabel::off_sheet(
            2,
            Point::new(1_000_000, 0),
            "BIAS",
            CrossSheetPortDirection::Input,
        ));

        schematic.document_policy.net_naming = NetNamingPolicy::StrictCaseSensitive;
        assert_eq!(
            off_sheet_findings(&schematic).len(),
            2,
            "case-sensitive naming makes these two unrelated declarations"
        );

        schematic.document_policy.net_naming = NetNamingPolicy::SpiceCompatibleRelaxed;
        assert!(
            off_sheet_findings(&schematic).is_empty(),
            "relaxed naming pairs them exactly as the netlister would"
        );

        schematic.document_policy.net_naming = NetNamingPolicy::StrictCaseSensitive;
        let mut result = DrcResult::new();
        let overrides = HashMap::from([(
            DrcViolationType::OffSheetConnectorWithoutPartner,
            DrcSeverity::Error,
        )]);
        append_off_sheet_connector_violations(&schematic, &mut result, &overrides);
        assert!(result.has_errors());
    }

    #[test]
    fn the_advisory_continues_the_result_id_sequence_it_is_appended_to() {
        use crate::state::{CrossSheetPortDirection, NetLabel, Point};

        let mut schematic = crate::state::SchematicState::default();
        schematic.net_labels.push(NetLabel::off_sheet(
            1,
            Point::origin(),
            "BIAS",
            CrossSheetPortDirection::Supply,
        ));

        let mut result = DrcResult::new();
        result.add_violation(DrcViolation::new(
            0,
            DrcViolationType::MissingGround,
            "existing finding",
            DrcLocation::Global,
        ));
        append_off_sheet_connector_violations(&schematic, &mut result, &HashMap::new());

        let ids: Vec<usize> = result
            .violations()
            .iter()
            .map(|violation| violation.id)
            .collect();
        assert_eq!(ids, vec![0, 1]);
    }
}
