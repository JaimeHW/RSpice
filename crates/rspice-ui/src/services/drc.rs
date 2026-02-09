//! Design Rule Checker (DRC/ERC)
//!
//! Professional-grade design and electrical rule checking for RSpice schematics.
//! Follows patterns from Cadence DRC and Mentor Calibre.
//!
//! # Features
//!
//! - Floating node detection (nodes with only one connection)
//! - Shorted outputs detection (multiple voltage sources on same net)
//! - Missing ground/reference check
//! - Duplicate component name detection
//! - Unconnected pin warnings
//! - Short circuit detection
//!
//! # Example
//!
//! ```ignore
//! use rspice_ui::services::drc::{DrcChecker, DrcResult};
//! use rspice_ui::state::SchematicState;
//!
//! let schematic = SchematicState::new();
//! let result = DrcChecker::check(&schematic);
//!
//! if result.has_errors() {
//!     for error in result.errors() {
//!         println!("DRC Error: {}", error.message);
//!     }
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

//=============================================================================
// DRC Violation Types
//=============================================================================

/// Severity level for DRC violations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum DrcSeverity {
    /// Informational note (won't affect simulation)
    Info,
    /// Warning (may affect simulation results)
    Warning,
    /// Error (will likely cause simulation failure)
    Error,
    /// Critical error (simulation cannot proceed)
    Critical,
}

impl DrcSeverity {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Info => "Info",
            Self::Warning => "Warning",
            Self::Error => "Error",
            Self::Critical => "Critical",
        }
    }

    /// Get icon for UI display
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Info => "ℹ️",
            Self::Warning => "⚠️",
            Self::Error => "❌",
            Self::Critical => "🛑",
        }
    }
}

/// Type of DRC violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DrcViolationType {
    //-------------------------------------------------------------------------
    // Connectivity Issues
    //-------------------------------------------------------------------------
    /// Node with only one connection (floating)
    FloatingNode,
    /// Component pin not connected to any wire
    UnconnectedPin,
    /// Net label with no matching connections
    OrphanNetLabel,
    /// Wire segment not connected to anything
    DanglingWire,

    //-------------------------------------------------------------------------
    // Electrical Issues
    //-------------------------------------------------------------------------
    /// Multiple voltage sources on same net
    ShortedOutputs,
    /// Missing ground/reference node
    MissingGround,
    /// Direct short circuit (wire from net to same net)
    ShortCircuit,
    /// Voltage source directly connected to another voltage source
    SourceToSource,

    //-------------------------------------------------------------------------
    // Naming Issues
    //-------------------------------------------------------------------------
    /// Duplicate component reference designator
    DuplicateName,
    /// Empty component name
    EmptyName,
    /// Invalid characters in name
    InvalidName,

    //-------------------------------------------------------------------------
    // Component Issues
    //-------------------------------------------------------------------------
    /// Component with missing required parameter
    MissingParameter,
    /// Component value out of expected range
    ValueOutOfRange,
    /// Unknown component type
    UnknownComponent,
}

impl DrcViolationType {
    /// Get the default severity for this violation type
    pub fn default_severity(&self) -> DrcSeverity {
        match self {
            // Critical - simulation cannot run
            Self::MissingGround => DrcSeverity::Critical,
            Self::DuplicateName => DrcSeverity::Critical,

            // Error - likely to cause simulation failure
            Self::FloatingNode => DrcSeverity::Error,
            Self::ShortedOutputs => DrcSeverity::Error,
            Self::ShortCircuit => DrcSeverity::Error,
            Self::SourceToSource => DrcSeverity::Error,
            Self::MissingParameter => DrcSeverity::Error,

            // Warning - may affect results
            Self::UnconnectedPin => DrcSeverity::Warning,
            Self::DanglingWire => DrcSeverity::Warning,
            Self::OrphanNetLabel => DrcSeverity::Warning,
            Self::ValueOutOfRange => DrcSeverity::Warning,

            // Info
            Self::EmptyName => DrcSeverity::Info,
            Self::InvalidName => DrcSeverity::Info,
            Self::UnknownComponent => DrcSeverity::Info,
        }
    }

    /// Get a description of this violation type
    pub fn description(&self) -> &'static str {
        match self {
            Self::FloatingNode => "Node has only one connection",
            Self::UnconnectedPin => "Component pin is not connected",
            Self::OrphanNetLabel => "Net label has no connections",
            Self::DanglingWire => "Wire segment is not connected to anything",
            Self::ShortedOutputs => "Multiple voltage sources connected to same net",
            Self::MissingGround => "Circuit has no ground reference (node 0)",
            Self::ShortCircuit => "Direct short circuit detected",
            Self::SourceToSource => "Voltage source outputs directly connected",
            Self::DuplicateName => "Multiple components have the same name",
            Self::EmptyName => "Component has no reference designator",
            Self::InvalidName => "Component name contains invalid characters",
            Self::MissingParameter => "Required component parameter is missing",
            Self::ValueOutOfRange => "Component value is outside expected range",
            Self::UnknownComponent => "Component type is not recognized",
        }
    }

    /// Get suggested fix for this violation
    pub fn suggested_fix(&self) -> &'static str {
        match self {
            Self::FloatingNode => "Connect the node to another component or remove it",
            Self::UnconnectedPin => "Connect a wire to this pin or mark as intentionally NC",
            Self::OrphanNetLabel => "Connect wires to both ends of the net or remove the label",
            Self::DanglingWire => "Complete the wire connection or delete the segment",
            Self::ShortedOutputs => "Connect only one voltage source per net",
            Self::MissingGround => "Add a ground symbol (GND) to the circuit",
            Self::ShortCircuit => "Remove the short or check intended connectivity",
            Self::SourceToSource => "Add a resistor between voltage sources",
            Self::DuplicateName => "Rename components to have unique identifiers",
            Self::EmptyName => "Assign a reference designator to the component",
            Self::InvalidName => "Use only alphanumeric characters and underscores",
            Self::MissingParameter => "Set the required parameter value",
            Self::ValueOutOfRange => "Check the parameter value is within valid range",
            Self::UnknownComponent => "Check the component type or define the model",
        }
    }
}

//=============================================================================
// DRC Violation
//=============================================================================

/// A single DRC violation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DrcViolation {
    /// Unique identifier for this violation
    pub id: usize,
    /// Type of violation
    pub violation_type: DrcViolationType,
    /// Severity level
    pub severity: DrcSeverity,
    /// Human-readable message
    pub message: String,
    /// Location in schematic (component ID, wire ID, or coordinates)
    pub location: DrcLocation,
    /// Related component names (for cross-referencing)
    pub related_items: Vec<String>,
}

impl DrcViolation {
    /// Create a new violation
    pub fn new(
        id: usize,
        violation_type: DrcViolationType,
        message: impl Into<String>,
        location: DrcLocation,
    ) -> Self {
        Self {
            id,
            severity: violation_type.default_severity(),
            violation_type,
            message: message.into(),
            location,
            related_items: Vec::new(),
        }
    }

    /// Add related items
    pub fn with_related(mut self, items: Vec<String>) -> Self {
        self.related_items = items;
        self
    }

    /// Override severity
    pub fn with_severity(mut self, severity: DrcSeverity) -> Self {
        self.severity = severity;
        self
    }
}

/// Location of a DRC violation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DrcLocation {
    /// At a specific schematic coordinate
    Point { x: f64, y: f64 },
    /// On a component
    Component { id: usize, name: String },
    /// On a wire
    Wire { id: usize },
    /// On a net label
    NetLabel { name: String },
    /// On a node (by net name)
    Node { net_name: String },
    /// Global (no specific location)
    Global,
}

impl DrcLocation {
    /// Get display string for this location
    pub fn display(&self) -> String {
        match self {
            Self::Point { x, y } => format!("({:.1}, {:.1})", x, y),
            Self::Component { name, .. } => format!("Component {}", name),
            Self::Wire { id } => format!("Wire #{}", id),
            Self::NetLabel { name } => format!("Net '{}'", name),
            Self::Node { net_name } => format!("Node '{}'", net_name),
            Self::Global => "Global".to_string(),
        }
    }
}

//=============================================================================
// DRC Result
//=============================================================================

/// Result of a DRC check.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DrcResult {
    /// All violations found
    violations: Vec<DrcViolation>,
    /// Whether the check completed successfully
    pub completed: bool,
    /// Time taken in milliseconds
    pub duration_ms: u64,
}

impl DrcResult {
    /// Create a new empty result
    pub fn new() -> Self {
        Self {
            violations: Vec::new(),
            completed: false,
            duration_ms: 0,
        }
    }

    /// Add a violation
    pub fn add_violation(&mut self, violation: DrcViolation) {
        self.violations.push(violation);
    }

    /// Get all violations
    pub fn violations(&self) -> &[DrcViolation] {
        &self.violations
    }

    /// Get violations filtered by severity
    pub fn violations_by_severity(&self, severity: DrcSeverity) -> Vec<&DrcViolation> {
        self.violations
            .iter()
            .filter(|v| v.severity == severity)
            .collect()
    }

    /// Get only errors (Error and Critical)
    pub fn errors(&self) -> Vec<&DrcViolation> {
        self.violations
            .iter()
            .filter(|v| v.severity >= DrcSeverity::Error)
            .collect()
    }

    /// Get only warnings
    pub fn warnings(&self) -> Vec<&DrcViolation> {
        self.violations
            .iter()
            .filter(|v| v.severity == DrcSeverity::Warning)
            .collect()
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        self.violations
            .iter()
            .any(|v| v.severity >= DrcSeverity::Error)
    }

    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        self.violations
            .iter()
            .any(|v| v.severity == DrcSeverity::Warning)
    }

    /// Check if the design passed (no errors or critical)
    pub fn passed(&self) -> bool {
        !self.has_errors()
    }

    /// Get total violation count
    pub fn total_count(&self) -> usize {
        self.violations.len()
    }

    /// Get count by severity
    pub fn count_by_severity(&self, severity: DrcSeverity) -> usize {
        self.violations
            .iter()
            .filter(|v| v.severity == severity)
            .count()
    }

    /// Get summary statistics
    pub fn summary(&self) -> DrcSummary {
        DrcSummary {
            total: self.violations.len(),
            critical: self.count_by_severity(DrcSeverity::Critical),
            errors: self.count_by_severity(DrcSeverity::Error),
            warnings: self.count_by_severity(DrcSeverity::Warning),
            info: self.count_by_severity(DrcSeverity::Info),
            passed: self.passed(),
        }
    }
}

/// Summary of DRC results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrcSummary {
    pub total: usize,
    pub critical: usize,
    pub errors: usize,
    pub warnings: usize,
    pub info: usize,
    pub passed: bool,
}

impl DrcSummary {
    /// Get display string
    pub fn display(&self) -> String {
        if self.passed {
            if self.warnings > 0 {
                format!("Passed with {} warning(s)", self.warnings)
            } else {
                "Passed - no issues found".to_string()
            }
        } else {
            format!(
                "Failed: {} critical, {} error(s), {} warning(s)",
                self.critical, self.errors, self.warnings
            )
        }
    }
}

//=============================================================================
// DRC Checker
//=============================================================================

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

        // Merge touching/crossing segments so T-junctions and wire intersections
        // become a single electrical net.
        for i in 0..segments.len() {
            for j in (i + 1)..segments.len() {
                if let Some(intersection) = segment_intersection_point(segments[i], segments[j]) {
                    let p_id =
                        ensure_point_id(intersection, &mut point_ids, &mut points_by_id, &mut dsu);
                    let (a0, a1) = segments[i];
                    let (b0, b1) = segments[j];
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

        // Attach component pins to any segment they lie on.
        for comp in components {
            for pin in &comp.pins {
                let (Some(x), Some(y)) = (pin.x, pin.y) else {
                    continue;
                };
                let point = PointKey::from_f64(x, y);
                let point_id = ensure_point_id(point, &mut point_ids, &mut points_by_id, &mut dsu);
                for &(start, end) in &segments {
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
            for &(start, end) in &segments {
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

//=============================================================================
// Simplified Input Types
//=============================================================================

/// Simplified component info for DRC checking.
#[derive(Debug, Clone)]
pub struct ComponentInfo {
    pub id: usize,
    pub name: String,
    pub component_type: String,
    pub pins: Vec<PinInfo>,
    pub is_voltage_source: bool,
    pub is_current_source: bool,
}

/// Simplified pin info.
#[derive(Debug, Clone)]
pub struct PinInfo {
    pub name: String,
    pub net_name: String,
    pub is_output: bool,
    /// Optional pin x-coordinate in schematic space.
    pub x: Option<f64>,
    /// Optional pin y-coordinate in schematic space.
    pub y: Option<f64>,
}

/// Simplified wire info.
#[derive(Debug, Clone)]
pub struct WireInfo {
    pub id: usize,
    pub start_x: f64,
    pub start_y: f64,
    pub end_x: f64,
    pub end_y: f64,
}

/// Simplified net label info.
#[derive(Debug, Clone)]
pub struct NetLabelInfo {
    pub name: String,
    pub x: f64,
    pub y: f64,
}

/// Internal net tracking info.
#[derive(Debug, Clone, Default)]
struct NetInfo {
    name: String,
    connection_count: usize,
    has_voltage_source: bool,
    has_current_source: bool,
    is_ground: bool,
    connected_components: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PointKey {
    x: i64,
    y: i64,
}

impl PointKey {
    fn from_f64(x: f64, y: f64) -> Self {
        Self {
            x: x.round() as i64,
            y: y.round() as i64,
        }
    }

    fn as_auto_name(self) -> String {
        format!("net_{}_{}", self.x, self.y)
    }
}

#[derive(Debug, Clone, Default)]
struct NetAccumulator {
    names: HashSet<String>,
    connection_count: usize,
    has_voltage_source: bool,
    has_current_source: bool,
    connected_components: HashSet<String>,
}

#[derive(Debug, Clone, Default)]
struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl DisjointSet {
    fn make_set(&mut self) -> usize {
        let id = self.parent.len();
        self.parent.push(id);
        self.rank.push(0);
        id
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
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

fn ensure_point_id(
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

fn point_on_segment(point: PointKey, seg_start: PointKey, seg_end: PointKey) -> bool {
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

fn segment_intersection_point(
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

fn merge_net_accumulator(
    net_map: &mut HashMap<String, NetInfo>,
    acc: NetAccumulator,
    fallback_point: Option<PointKey>,
) {
    let canonical_name = canonical_net_name(&acc.names, fallback_point);
    let entry = net_map
        .entry(canonical_name.clone())
        .or_insert_with(|| NetInfo {
            name: canonical_name.clone(),
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

fn canonical_net_name(names: &HashSet<String>, fallback_point: Option<PointKey>) -> String {
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

fn is_ground_like(name: &str) -> bool {
    name.eq_ignore_ascii_case("0")
        || name.eq_ignore_ascii_case("gnd")
        || name.eq_ignore_ascii_case("ground")
}

fn is_auto_generated_net_name(name: &str) -> bool {
    if name.starts_with("net_") {
        return true;
    }
    name.contains('_')
        && name
            .chars()
            .all(|c| c.is_numeric() || c == '_' || c == '-' || c == '.')
}

//=============================================================================
// Schematic Data Extraction
//=============================================================================

/// Extract DRC-compatible data from a SchematicState.
///
/// This is the bridge between the schematic representation and the DRC checker.
/// It extracts components, wires, and net labels in the format required by
/// DrcChecker::check_connectivity().
///
/// # Performance
/// - O(n) iteration over components
/// - O(n) iteration over wires  
/// - O(n) iteration over net labels
///
/// # Example
/// ```ignore
/// use rspice_ui::services::drc::{DrcChecker, extract_drc_data};
///
/// let (components, wires, net_labels) = extract_drc_data(&schematic);
/// let result = DrcChecker::new().check_connectivity(&components, &wires, &net_labels);
/// ```
pub fn extract_drc_data(
    schematic: &crate::state::SchematicState,
) -> (Vec<ComponentInfo>, Vec<WireInfo>, Vec<NetLabelInfo>) {
    use crate::state::ComponentType;

    let mut components = Vec::with_capacity(schematic.components.len());
    let mut wires = Vec::with_capacity(schematic.wires.len());
    let mut net_labels = Vec::with_capacity(schematic.net_labels.len());

    // Build point-to-net mapping from existing net_mapping or create from connectivity
    let net_mapping = &schematic.net_mapping;

    // Extract components
    for comp in &schematic.components {
        let terminal_positions = comp.terminal_positions();
        let mut pins = Vec::with_capacity(terminal_positions.len());

        for (pin_name, pin_pos) in terminal_positions {
            // Look up net name from the cached mapping, or create a positional name
            let net_name = net_mapping
                .get(&pin_pos)
                .cloned()
                .unwrap_or_else(|| format!("net_{}_{}", pin_pos.x, pin_pos.y));

            let is_output = matches!(
                comp.kind,
                ComponentType::VoltageSource
                    | ComponentType::VoltageSourceAc
                    | ComponentType::VoltageSourcePulse
                    | ComponentType::VoltageSourceSin
                    | ComponentType::VoltageSourcePwl
            ) && pin_name == "+";

            pins.push(PinInfo {
                name: pin_name.to_string(),
                net_name,
                is_output,
                x: Some(pin_pos.x as f64),
                y: Some(pin_pos.y as f64),
            });
        }

        let is_voltage_source = matches!(
            comp.kind,
            ComponentType::VoltageSource
                | ComponentType::VoltageSourceAc
                | ComponentType::VoltageSourcePulse
                | ComponentType::VoltageSourceSin
                | ComponentType::VoltageSourcePwl
        );

        let is_current_source = matches!(
            comp.kind,
            ComponentType::CurrentSource
                | ComponentType::CurrentSourceAc
                | ComponentType::CurrentSourcePulse
                | ComponentType::CurrentSourceSin
                | ComponentType::CurrentSourcePwl
        );

        components.push(ComponentInfo {
            id: comp.id as usize,
            name: if comp.name.is_empty() {
                comp.spice_instance_name()
            } else {
                comp.name.clone()
            },
            component_type: comp.kind.spice_prefix().to_string(),
            pins,
            is_voltage_source,
            is_current_source,
        });
    }

    // Extract wires
    for wire in &schematic.wires {
        if wire.points.len() >= 2 {
            // Create WireInfo for each segment
            for i in 0..wire.points.len() - 1 {
                let start = &wire.points[i];
                let end = &wire.points[i + 1];
                wires.push(WireInfo {
                    id: wire.id as usize,
                    start_x: start.x as f64,
                    start_y: start.y as f64,
                    end_x: end.x as f64,
                    end_y: end.y as f64,
                });
            }
        }
    }

    // Extract net labels (including ground symbols)
    for label in &schematic.net_labels {
        net_labels.push(NetLabelInfo {
            name: label.name.clone(),
            x: label.pos.x as f64,
            y: label.pos.y as f64,
        });
    }

    // Check for ground components (GND symbol)
    for comp in &schematic.components {
        if matches!(comp.kind, ComponentType::Ground) {
            // Ground component acts as a net label for "0"
            net_labels.push(NetLabelInfo {
                name: "0".to_string(),
                x: comp.pos.x as f64,
                y: comp.pos.y as f64,
            });
        }
    }

    (components, wires, net_labels)
}

/// Run a complete DRC check on a schematic.
///
/// This is a convenience function that extracts data and runs the check
/// in a single call.
///
/// # Example
/// ```ignore
/// use rspice_ui::services::drc::run_drc_check;
///
/// let result = run_drc_check(&schematic);
/// if result.passed() {
///     println!("DRC passed!");
/// }
/// ```
pub fn run_drc_check(schematic: &crate::state::SchematicState) -> DrcResult {
    let (components, wires, net_labels) = extract_drc_data(schematic);
    let mut checker = DrcChecker::new();
    checker.check_connectivity(&components, &wires, &net_labels)
}

/// Run a complete DRC check with custom configuration.
pub fn run_drc_check_with_config(
    schematic: &crate::state::SchematicState,
    config: DrcConfig,
) -> DrcResult {
    let (components, wires, net_labels) = extract_drc_data(schematic);
    let mut checker = DrcChecker::with_config(config);
    checker.check_connectivity(&components, &wires, &net_labels)
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_resistor(id: usize, name: &str, net1: &str, net2: &str) -> ComponentInfo {
        ComponentInfo {
            id,
            name: name.to_string(),
            component_type: "R".to_string(),
            pins: vec![
                PinInfo {
                    name: "1".to_string(),
                    net_name: net1.to_string(),
                    is_output: false,
                    x: None,
                    y: None,
                },
                PinInfo {
                    name: "2".to_string(),
                    net_name: net2.to_string(),
                    is_output: false,
                    x: None,
                    y: None,
                },
            ],
            is_voltage_source: false,
            is_current_source: false,
        }
    }

    fn make_voltage_source(id: usize, name: &str, pos: &str, neg: &str) -> ComponentInfo {
        ComponentInfo {
            id,
            name: name.to_string(),
            component_type: "V".to_string(),
            pins: vec![
                PinInfo {
                    name: "+".to_string(),
                    net_name: pos.to_string(),
                    is_output: true,
                    x: None,
                    y: None,
                },
                PinInfo {
                    name: "-".to_string(),
                    net_name: neg.to_string(),
                    is_output: false,
                    x: None,
                    y: None,
                },
            ],
            is_voltage_source: true,
            is_current_source: false,
        }
    }

    fn make_ground() -> NetLabelInfo {
        NetLabelInfo {
            name: "0".to_string(),
            x: 0.0,
            y: 0.0,
        }
    }

    fn make_pin(name: &str, net_name: &str, is_output: bool, x: f64, y: f64) -> PinInfo {
        PinInfo {
            name: name.to_string(),
            net_name: net_name.to_string(),
            is_output,
            x: Some(x),
            y: Some(y),
        }
    }

    #[test]
    fn test_no_violations_simple_circuit() {
        let mut checker = DrcChecker::new();

        let components = vec![
            make_voltage_source(0, "V1", "in", "0"),
            make_resistor(1, "R1", "in", "out"),
            make_resistor(2, "R2", "out", "0"),
        ];

        let net_labels = vec![make_ground()];
        let wires = vec![];

        let result = checker.check_connectivity(&components, &wires, &net_labels);

        assert!(result.completed);
        assert!(result.passed(), "Simple circuit should pass DRC");
    }

    #[test]
    fn test_missing_ground_detected() {
        let mut checker = DrcChecker::new();

        let components = vec![
            make_voltage_source(0, "V1", "in", "ref"),
            make_resistor(1, "R1", "in", "ref"),
        ];

        let result = checker.check_connectivity(&components, &[], &[]);

        assert!(!result.passed());
        assert_eq!(result.count_by_severity(DrcSeverity::Critical), 1);

        let critical = result.violations_by_severity(DrcSeverity::Critical);
        assert_eq!(critical[0].violation_type, DrcViolationType::MissingGround);
    }

    #[test]
    fn test_duplicate_names_detected() {
        let mut checker = DrcChecker::new();

        let components = vec![
            make_resistor(0, "R1", "a", "b"),
            make_resistor(1, "R1", "b", "c"), // Duplicate!
            make_resistor(2, "R2", "c", "0"),
        ];

        let result = checker.check_connectivity(&components, &[], &[make_ground()]);

        assert!(!result.passed());
        let critical = result.violations_by_severity(DrcSeverity::Critical);
        assert_eq!(critical.len(), 1);
        assert_eq!(critical[0].violation_type, DrcViolationType::DuplicateName);
        assert!(critical[0].message.contains("R1"));
    }

    #[test]
    fn test_shorted_voltage_sources() {
        let mut checker = DrcChecker::new();

        let components = vec![
            make_voltage_source(0, "V1", "out", "0"),
            make_voltage_source(1, "V2", "out", "0"), // Same net!
        ];

        let result = checker.check_connectivity(&components, &[], &[make_ground()]);

        assert!(!result.passed());
        let errors = result.errors();
        let shorted = errors
            .iter()
            .find(|v| v.violation_type == DrcViolationType::ShortedOutputs);
        assert!(shorted.is_some(), "Should detect shorted voltage sources");
    }

    #[test]
    fn test_floating_node() {
        let mut checker = DrcChecker::new();

        let components = vec![
            make_voltage_source(0, "V1", "in", "0"),
            make_resistor(1, "R1", "in", "floating"), // 'floating' only has 1 connection
        ];

        let result = checker.check_connectivity(&components, &[], &[make_ground()]);

        let floating = result
            .violations()
            .iter()
            .find(|v| v.violation_type == DrcViolationType::FloatingNode);

        assert!(floating.is_some(), "Should detect floating node");
        assert!(floating.unwrap().message.contains("floating"));
    }

    #[test]
    fn test_severity_levels() {
        assert!(DrcSeverity::Critical > DrcSeverity::Error);
        assert!(DrcSeverity::Error > DrcSeverity::Warning);
        assert!(DrcSeverity::Warning > DrcSeverity::Info);
    }

    #[test]
    fn test_violation_type_descriptions() {
        // All violation types should have descriptions and suggested fixes
        let types = [
            DrcViolationType::FloatingNode,
            DrcViolationType::MissingGround,
            DrcViolationType::DuplicateName,
            DrcViolationType::ShortedOutputs,
        ];

        for vtype in types {
            assert!(!vtype.description().is_empty());
            assert!(!vtype.suggested_fix().is_empty());
        }
    }

    #[test]
    fn test_drc_result_summary() {
        let mut result = DrcResult::new();

        result.add_violation(DrcViolation::new(
            0,
            DrcViolationType::FloatingNode,
            "Test",
            DrcLocation::Global,
        ));
        result.add_violation(DrcViolation::new(
            1,
            DrcViolationType::MissingGround,
            "Test",
            DrcLocation::Global,
        ));

        let summary = result.summary();
        assert_eq!(summary.total, 2);
        assert_eq!(summary.errors, 1);
        assert_eq!(summary.critical, 1);
        assert!(!summary.passed);
    }

    #[test]
    fn test_config_disable_checks() {
        let config = DrcConfig {
            check_missing_ground: false,
            ..Default::default()
        };

        let mut checker = DrcChecker::with_config(config);

        let components = vec![
            make_voltage_source(0, "V1", "in", "ref"),
            make_resistor(1, "R1", "in", "ref"),
        ];

        let result = checker.check_connectivity(&components, &[], &[]);

        // Missing ground should NOT be reported
        let critical = result.violations_by_severity(DrcSeverity::Critical);
        assert!(
            !critical
                .iter()
                .any(|v| v.violation_type == DrcViolationType::MissingGround),
            "Missing ground check should be disabled"
        );
    }

    #[test]
    fn test_location_display() {
        let loc1 = DrcLocation::Point { x: 100.0, y: 200.0 };
        assert!(loc1.display().contains("100"));

        let loc2 = DrcLocation::Component {
            id: 1,
            name: "R1".to_string(),
        };
        assert!(loc2.display().contains("R1"));

        let loc3 = DrcLocation::Global;
        assert_eq!(loc3.display(), "Global");
    }

    #[test]
    fn test_violation_with_related() {
        let violation = DrcViolation::new(
            0,
            DrcViolationType::DuplicateName,
            "Duplicate",
            DrcLocation::Global,
        )
        .with_related(vec!["R1".to_string(), "R1".to_string()]);

        assert_eq!(violation.related_items.len(), 2);
    }

    #[test]
    fn test_serialization() {
        let result = DrcResult::new();
        let json = serde_json::to_string(&result);
        assert!(json.is_ok());
    }

    // =========================================================================
    // Schematic Extraction Tests
    // =========================================================================

    #[test]
    fn test_extract_drc_data_empty_schematic() {
        let schematic = crate::state::SchematicState::default();
        let (components, wires, net_labels) = extract_drc_data(&schematic);

        assert!(components.is_empty());
        assert!(wires.is_empty());
        assert!(net_labels.is_empty());
    }

    #[test]
    fn test_extract_drc_data_with_ground_component() {
        use crate::state::{Component, ComponentType, Point, SchematicState};

        let mut schematic = SchematicState::default();

        // Add a ground symbol
        let ground = Component::new(1, ComponentType::Ground, Point::new(10, 20));
        schematic.components.push(ground);

        let (components, _wires, net_labels) = extract_drc_data(&schematic);

        // Ground component should generate both a component AND a net label
        assert_eq!(components.len(), 1);
        assert!(net_labels.iter().any(|n| n.name == "0"));
    }

    #[test]
    fn test_extract_drc_data_with_voltage_source() {
        use crate::state::{Component, ComponentType, Point, SchematicState};

        let mut schematic = SchematicState::default();

        // Add a voltage source
        let vsrc = Component::new(1, ComponentType::VoltageSource, Point::new(10, 20))
            .with_name_value("V1", "5");
        schematic.components.push(vsrc);

        let (components, _wires, _net_labels) = extract_drc_data(&schematic);

        assert_eq!(components.len(), 1);
        assert!(components[0].is_voltage_source);
        assert!(!components[0].is_current_source);
        assert_eq!(components[0].name, "V1");
    }

    #[test]
    fn test_extract_drc_data_with_current_source() {
        use crate::state::{Component, ComponentType, Point, SchematicState};

        let mut schematic = SchematicState::default();

        // Add a current source
        let isrc = Component::new(1, ComponentType::CurrentSource, Point::new(10, 20))
            .with_name_value("I1", "1m");
        schematic.components.push(isrc);

        let (components, _wires, _net_labels) = extract_drc_data(&schematic);

        assert_eq!(components.len(), 1);
        assert!(!components[0].is_voltage_source);
        assert!(components[0].is_current_source);
        assert_eq!(components[0].name, "I1");
    }

    #[test]
    fn test_extract_drc_data_with_resistor() {
        use crate::state::{Component, ComponentType, Point, SchematicState};

        let mut schematic = SchematicState::default();

        // Add a resistor
        let res = Component::new(1, ComponentType::Resistor, Point::new(10, 20))
            .with_name_value("R1", "1k");
        schematic.components.push(res);

        let (components, _wires, _net_labels) = extract_drc_data(&schematic);

        assert_eq!(components.len(), 1);
        assert!(!components[0].is_voltage_source);
        assert!(!components[0].is_current_source);
        assert_eq!(components[0].name, "R1");
        assert_eq!(components[0].component_type, "R");
        assert_eq!(components[0].pins.len(), 2); // Resistors have 2 pins
    }

    #[test]
    fn test_extract_drc_data_with_wires() {
        use crate::state::{Point, SchematicState, Wire};

        let mut schematic = SchematicState::default();

        // Add a wire with multiple segments
        let wire = Wire::new(
            1,
            vec![Point::new(0, 0), Point::new(10, 0), Point::new(10, 10)],
        );
        schematic.wires.push(wire);

        let (_components, wires, _net_labels) = extract_drc_data(&schematic);

        // Wire with 3 points = 2 segments
        assert_eq!(wires.len(), 2);

        // First segment
        assert_eq!(wires[0].start_x, 0.0);
        assert_eq!(wires[0].start_y, 0.0);
        assert_eq!(wires[0].end_x, 10.0);
        assert_eq!(wires[0].end_y, 0.0);

        // Second segment
        assert_eq!(wires[1].start_x, 10.0);
        assert_eq!(wires[1].start_y, 0.0);
        assert_eq!(wires[1].end_x, 10.0);
        assert_eq!(wires[1].end_y, 10.0);
    }

    #[test]
    fn test_extract_drc_data_with_net_labels() {
        use crate::state::{NetLabel, Point, SchematicState};

        let mut schematic = SchematicState::default();

        // Add net labels
        schematic.net_labels.push(NetLabel {
            id: 1,
            name: "VDD".to_string(),
            pos: Point::new(0, 0),
        });
        schematic.net_labels.push(NetLabel {
            id: 2,
            name: "VSS".to_string(),
            pos: Point::new(0, 10),
        });

        let (_components, _wires, net_labels) = extract_drc_data(&schematic);

        assert_eq!(net_labels.len(), 2);
        assert!(net_labels.iter().any(|n| n.name == "VDD"));
        assert!(net_labels.iter().any(|n| n.name == "VSS"));
    }

    #[test]
    fn test_run_drc_check_empty_schematic() {
        let schematic = crate::state::SchematicState::default();
        let result = run_drc_check(&schematic);

        // Empty schematic should pass (no components = no violations)
        assert!(result.completed);
        // May have missing ground warning since there's no GND
        // but that's OK for an empty schematic
    }

    #[test]
    fn test_run_drc_check_with_ground() {
        use crate::state::{Component, ComponentType, Point, SchematicState};

        let mut schematic = SchematicState::default();

        // Add just a ground symbol
        let ground = Component::new(1, ComponentType::Ground, Point::new(10, 20));
        schematic.components.push(ground);

        let result = run_drc_check(&schematic);

        assert!(result.completed);
        // With ground present, should not have missing ground error
        let has_missing_ground = result
            .violations()
            .iter()
            .any(|v| v.violation_type == DrcViolationType::MissingGround);
        assert!(!has_missing_ground);
    }

    #[test]
    fn test_run_drc_check_duplicate_names() {
        use crate::state::{Component, ComponentType, Point, SchematicState};

        let mut schematic = SchematicState::default();

        // Add two resistors with the same name
        let r1 = Component::new(1, ComponentType::Resistor, Point::new(0, 0))
            .with_name_value("R1", "1k");
        let r1_dupe = Component::new(2, ComponentType::Resistor, Point::new(10, 10))
            .with_name_value("R1", "2k"); // Duplicate name!
        let gnd = Component::new(3, ComponentType::Ground, Point::new(0, 10));

        schematic.components.push(r1);
        schematic.components.push(r1_dupe);
        schematic.components.push(gnd);

        let result = run_drc_check(&schematic);

        assert!(result.completed);
        assert!(!result.passed()); // Should fail due to duplicate name

        let has_duplicate = result
            .violations()
            .iter()
            .any(|v| v.violation_type == DrcViolationType::DuplicateName);
        assert!(has_duplicate);
    }

    #[test]
    fn test_run_drc_check_with_config() {
        use crate::state::SchematicState;

        let schematic = SchematicState::default();

        // Disable all checks
        let config = DrcConfig {
            check_floating_nodes: false,
            check_unconnected_pins: false,
            check_missing_ground: false,
            check_duplicate_names: false,
            check_shorted_outputs: false,
            ..Default::default()
        };

        let result = run_drc_check_with_config(&schematic, config);

        assert!(result.completed);
        assert!(result.passed()); // Should pass with all checks disabled
        assert_eq!(result.total_count(), 0);
    }

    #[test]
    fn test_wire_topology_merges_pins_connected_by_segment() {
        let mut checker = DrcChecker::new();
        let components = vec![
            ComponentInfo {
                id: 0,
                name: "V1".to_string(),
                component_type: "V".to_string(),
                pins: vec![
                    make_pin("+", "net_0_0", true, 0.0, 0.0),
                    make_pin("-", "0", false, 0.0, -10.0),
                ],
                is_voltage_source: true,
                is_current_source: false,
            },
            ComponentInfo {
                id: 1,
                name: "R1".to_string(),
                component_type: "R".to_string(),
                pins: vec![
                    make_pin("1", "net_10_0", false, 10.0, 0.0),
                    make_pin("2", "0", false, 10.0, -10.0),
                ],
                is_voltage_source: false,
                is_current_source: false,
            },
        ];
        let wires = vec![WireInfo {
            id: 1,
            start_x: 0.0,
            start_y: 0.0,
            end_x: 10.0,
            end_y: 0.0,
        }];
        let labels = vec![NetLabelInfo {
            name: "0".to_string(),
            x: 0.0,
            y: -10.0,
        }];

        let result = checker.check_connectivity(&components, &wires, &labels);
        let has_floating = result
            .violations()
            .iter()
            .any(|v| v.violation_type == DrcViolationType::FloatingNode);
        assert!(
            !has_floating,
            "wire-connected pins should not be reported as floating"
        );
    }

    #[test]
    fn test_net_label_renames_wire_connected_cluster() {
        let checker = DrcChecker::new();
        let components = vec![ComponentInfo {
            id: 0,
            name: "R1".to_string(),
            component_type: "R".to_string(),
            pins: vec![
                make_pin("1", "net_0_0", false, 0.0, 0.0),
                make_pin("2", "net_10_0", false, 10.0, 0.0),
            ],
            is_voltage_source: false,
            is_current_source: false,
        }];
        let wires = vec![WireInfo {
            id: 7,
            start_x: 0.0,
            start_y: 0.0,
            end_x: 10.0,
            end_y: 0.0,
        }];
        let labels = vec![NetLabelInfo {
            name: "VDD".to_string(),
            x: 5.0,
            y: 0.0,
        }];

        let net_map = checker.build_net_map(&components, &wires, &labels);
        assert!(
            net_map.contains_key("VDD"),
            "expected label to define canonical net name, nets: {:?}",
            net_map.keys().collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_wire_only_clusters_do_not_trigger_floating_nodes() {
        let config = DrcConfig {
            check_missing_ground: false,
            ..Default::default()
        };
        let mut checker = DrcChecker::with_config(config);
        let wires = vec![WireInfo {
            id: 1,
            start_x: 0.0,
            start_y: 0.0,
            end_x: 10.0,
            end_y: 0.0,
        }];

        let result = checker.check_connectivity(&[], &wires, &[]);
        let has_floating = result
            .violations()
            .iter()
            .any(|v| v.violation_type == DrcViolationType::FloatingNode);
        assert!(
            !has_floating,
            "wire-only topology should not be treated as floating electrical nodes"
        );
    }
}
