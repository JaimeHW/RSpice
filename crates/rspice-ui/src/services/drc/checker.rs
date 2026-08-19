//! The design rule checker.
//!
//! Runs the configured rules over an extracted design and collects the
//! violations. Which rules run and at what severity is configuration, so a
//! project can promote a warning to an error.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::input::ComponentInfo;
use super::net::{NetInfo, fold_nets};
use super::netlist_gen::extraction::{
    ConnectivityAnchor, ConnectivityDiagnosticKind, ExtractedConnectivity,
};
use super::types::{DrcLocation, DrcResult, DrcSeverity, DrcViolation, DrcViolationType};

/// Where one connectivity diagnostic points, in the vocabulary a finding uses.
pub(super) fn diagnostic_location(anchor: &ConnectivityAnchor) -> DrcLocation {
    match anchor {
        ConnectivityAnchor::Point(point) => DrcLocation::Point {
            x: f64::from(point.x),
            y: f64::from(point.y),
        },
        ConnectivityAnchor::Bus(id) => DrcLocation::Bus { id: *id },
        ConnectivityAnchor::BusTap(id) => DrcLocation::BusTap { id: *id },
        ConnectivityAnchor::NetLabel(name) => DrcLocation::NetLabel { name: name.clone() },
        ConnectivityAnchor::Net(name) => DrcLocation::Node {
            net_name: name.clone(),
        },
    }
}

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
    /// The project's connectivity policy, as the check must apply it.
    ///
    /// A checker that defaulted this would be inventing project policy, so the
    /// caller passes the contract the project actually persisted; the default
    /// here is the contract's own default, which blocks a mismatched vector
    /// connection.
    ///
    /// It serializes into the check's input digest — changing the policy has to
    /// invalidate a cached report — but never deserializes back: the project
    /// contract is the only authority that may set it.
    #[serde(skip_deserializing)]
    pub connectivity: crate::state::ConnectivityPolicy,
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
            connectivity: crate::state::ConnectivityPolicy::default(),
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

    /// Run every enabled rule over one extracted design.
    ///
    /// Both arguments come from the same [`ExtractedConnectivity`]: the
    /// components carry the node names and attachment the extraction decided,
    /// and the connectivity carries the conductors, the ground binding and the
    /// diagnostics. Nothing here looks at the drawing again.
    pub fn check(
        &mut self,
        components: &[ComponentInfo],
        connectivity: &ExtractedConnectivity,
    ) -> DrcResult {
        let start = crate::time_compat::Instant::now();
        self.next_id = 0;
        let mut result = DrcResult::new();

        let net_map = fold_nets(components, connectivity);

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
            self.check_unconnected_pins(components, &mut result);
            self.check_orphan_net_labels(connectivity, &mut result);
            self.check_dangling_wires(&net_map, &mut result);
        }

        // Check for missing ground
        if self.config.check_missing_ground {
            self.check_missing_ground(components, &net_map, &mut result);
        }

        // Check for floating nodes
        if self.config.check_floating_nodes {
            self.check_floating_nodes(&net_map, &mut result);
        }

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

    /// Report every terminal the extraction found nothing else meeting.
    fn check_unconnected_pins(&mut self, components: &[ComponentInfo], result: &mut DrcResult) {
        let mut ordered_components = components.iter().collect::<Vec<_>>();
        ordered_components.sort_by_key(|component| component.id);
        for component in ordered_components {
            let mut pins = component.pins.iter().collect::<Vec<_>>();
            pins.sort_by(|left, right| {
                left.name
                    .cmp(&right.name)
                    .then_with(|| left.point.x.cmp(&right.point.x))
                    .then_with(|| left.point.y.cmp(&right.point.y))
            });
            for pin in pins {
                if pin.attached {
                    continue;
                }
                let id = self.next_id();
                let component_name = component_identity(component);
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

    /// Report every net label the extraction could not place on a conductor.
    ///
    /// The judgement is the extraction's — a label names the net beneath it, so
    /// whether one exists is the same question netlisting already answered.
    fn check_orphan_net_labels(
        &mut self,
        connectivity: &ExtractedConnectivity,
        result: &mut DrcResult,
    ) {
        for diagnostic in &connectivity.diagnostics {
            if diagnostic.kind != ConnectivityDiagnosticKind::OrphanNetLabel {
                continue;
            }
            let id = self.next_id();
            self.add_violation(
                result,
                DrcViolation::new(
                    id,
                    DrcViolationType::OrphanNetLabel,
                    diagnostic.message.clone(),
                    diagnostic_location(&diagnostic.anchor),
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
            if net.connection_count != 0 || net.tapped {
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

    /// Check that the circuit actually reaches a reference node.
    ///
    /// A drawn ground symbol is not itself the reference: node 0 has to bind at
    /// least one other terminal before the design has one. A symbol standing on
    /// its own is the commonest way to draw a circuit that will not solve, so
    /// the finding points at that symbol's terminal rather than at the design.
    fn check_missing_ground(
        &mut self,
        components: &[ComponentInfo],
        net_map: &HashMap<String, NetInfo>,
        result: &mut DrcResult,
    ) {
        let grounded = net_map
            .values()
            .any(|net| net.is_ground && net.connection_count > net.ground_symbol_count);
        if grounded {
            return;
        }

        let anchor = components
            .iter()
            .filter(|component| component.is_ground_symbol)
            .flat_map(|component| component.pins.iter())
            .map(|pin| pin.point)
            .next();
        let (message, location) = match anchor {
            Some(point) => (
                "Ground symbol reaches no circuit; node 0 has no other connection".to_owned(),
                DrcLocation::Point {
                    x: f64::from(point.x),
                    y: f64::from(point.y),
                },
            ),
            None => (
                "Circuit has no ground reference (node 0 or GND)".to_owned(),
                DrcLocation::Global,
            ),
        };
        let id = self.next_id();
        self.add_violation(
            result,
            DrcViolation::new(id, DrcViolationType::MissingGround, message, location),
        );
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
            if !net.bus_member || net.output_drivers.len() <= 1 {
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
                        net.display_name,
                        drivers.len(),
                        drivers.join(", ")
                    ),
                    DrcLocation::Node {
                        net_name: net.display_name.clone(),
                    },
                )
                .with_related(drivers),
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
    // Two connectors are the same declaration when the netlister would join
    // them, and the netlister folds ASCII case whatever the naming policy says.
    let key = |name: &str| name.trim().to_ascii_lowercase();

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

/// Report every vector connection whose two ends declare different widths.
///
/// This is where [`crate::state::BundleWidthMismatchPolicy`] decides something.
/// Under `BlockConnection` a mismatched join is an error: the deck would have
/// to invent or drop conductors to emit it. Under `ExplicitSliceOrExtend` the
/// project has said it will slice or extend deliberately, so a mismatch the
/// designer can resolve with an explicit selector is stated as a warning — but
/// only when the bus is at least as wide as the connection asks for. A bus with
/// too few conductors cannot be sliced into a wider connection under any
/// policy, so that one stays an error.
pub(super) fn append_vector_width_violations(
    connectivity: &ExtractedConnectivity,
    policy: &crate::state::ConnectivityPolicy,
    result: &mut DrcResult,
    severity_overrides: &HashMap<DrcViolationType, DrcSeverity>,
) {
    let mut next_id = result.total_count();
    for mismatch in &connectivity.vector_nets.mismatches {
        let sliceable = policy.width_mismatch
            == crate::state::BundleWidthMismatchPolicy::ExplicitSliceOrExtend
            && mismatch.found_width >= mismatch.declared_width;
        let mut violation = DrcViolation::new(
            next_id,
            DrcViolationType::VectorWidthMismatch,
            format!("{}.", mismatch.message()),
            DrcLocation::Point {
                x: f64::from(mismatch.point.x),
                y: f64::from(mismatch.point.y),
            },
        );
        if sliceable {
            violation.severity = DrcSeverity::Warning;
        }
        if let Some(severity) = severity_overrides.get(&DrcViolationType::VectorWidthMismatch) {
            violation.severity = *severity;
        }
        result.add_violation(violation);
        next_id += 1;
    }
}

/// Report every pair of authored net names that differ only by ASCII case.
///
/// The deck is case-insensitive, so `Out` and `out` are one node in the
/// simulation while the drawing still shows two named nets. That gap is the
/// finding: it is not the naming policy's business, because the policy governs
/// which characters a name may contain and the engine folds case regardless.
/// The authored names are the ones a designer typed — net labels and interface
/// ports — and the finding is located on the second of the two, which is the
/// one that arrived after the name was already taken.
pub(super) fn append_case_collision_violations(
    schematic: &crate::state::SchematicState,
    result: &mut DrcResult,
    severity_overrides: &HashMap<DrcViolationType, DrcSeverity>,
) {
    let mut labels: Vec<&crate::state::NetLabel> = schematic.net_labels.iter().collect();
    labels.sort_by_key(|label| label.id);
    let authored = labels
        .into_iter()
        .map(|label| {
            (
                label.name.trim().to_owned(),
                DrcLocation::NetLabel {
                    name: label.name.clone(),
                },
            )
        })
        .chain(schematic.components.iter().filter_map(|component| {
            let spec = component.port_spec()?;
            Some((
                spec.name.clone(),
                DrcLocation::Component {
                    id: component.id,
                    name: spec.name,
                },
            ))
        }));

    let mut first_spelling: HashMap<String, String> = HashMap::new();
    let mut next_id = result.total_count();
    for (name, location) in authored {
        if name.is_empty() {
            continue;
        }
        match first_spelling.entry(name.to_ascii_lowercase()) {
            std::collections::hash_map::Entry::Vacant(slot) => {
                slot.insert(name);
            }
            std::collections::hash_map::Entry::Occupied(slot) if *slot.get() != name => {
                let mut violation = DrcViolation::new(
                    next_id,
                    DrcViolationType::CaseCollidingNetNames,
                    format!(
                        "Nets `{}` and `{name}` differ only by case; the netlist joins them \
                         into one node.",
                        slot.get()
                    ),
                    location,
                )
                .with_related(vec![slot.get().clone(), name]);
                if let Some(severity) =
                    severity_overrides.get(&DrcViolationType::CaseCollidingNetNames)
                {
                    violation.severity = *severity;
                }
                result.add_violation(violation);
                next_id += 1;
            }
            std::collections::hash_map::Entry::Occupied(_) => {}
        }
    }
}

/// What a finding calls this component: the authored reference where there is
/// one, and its identity otherwise, so a message never names an empty string.
pub(super) fn component_identity(component: &ComponentInfo) -> String {
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
mod tests;
