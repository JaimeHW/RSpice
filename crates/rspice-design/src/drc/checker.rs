//! The design rule checker.
//!
//! Runs the configured rules over an extracted design and collects the
//! violations. Which rules run and at what severity is configuration, so a
//! project can promote a warning to an error.

use serde::{Deserialize, Serialize};
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, HashMap};

use super::input::ComponentInfo;
use super::net::{NetInfo, fold_nets};
use super::source_loops;
use super::types::{DrcLocation, DrcResult, DrcSeverity, DrcViolation, DrcViolationType};
use crate::connectivity::{ConnectivityAnchor, ConnectivityDiagnosticKind, ExtractedConnectivity};
use crate::schematic::component::{CardRole, WrittenCard};
use crate::schematic::port::PortDirection;

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
        ConnectivityAnchor::Component { id, name } => DrcLocation::Component {
            id: *id,
            name: name.clone(),
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
    pub connectivity: crate::connectivity_contract::ConnectivityPolicy,
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
            connectivity: crate::connectivity_contract::ConnectivityPolicy::default(),
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
        let start = web_time::Instant::now();
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
            self.check_instance_references(components, &mut result);
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
            self.check_source_to_source(components, &mut result);
            self.check_shorted_outputs(&net_map, &mut result);
            self.check_duplicate_bus_member_drivers(&net_map, &mut result);
        }

        result.completed = true;
        result.duration_ms = start.elapsed().as_millis() as u64;
        result
    }

    /// Report every emitted instance whose reference the schema refuses.
    ///
    /// How bad that is depends on what the deck does with it. The netlister
    /// names an unnamed primitive `<prefix><id>` and prefixes one that starts
    /// with the wrong letter, so the run starts and the results carry the
    /// name it chose — which is why the finding states that name. Only an
    /// instance the deck cannot write at all is an error: a cell instance
    /// whose reference is not a path segment, or a name the card cannot hold.
    fn check_component_references(&mut self, components: &[ComponentInfo], result: &mut DrcResult) {
        let mut components = components.iter().collect::<Vec<_>>();
        components.sort_by_key(|component| component.id);
        for component in components {
            if !component.reference_required {
                continue;
            }
            let written = component.written_cards.is_some();
            let consequence = match component.written_cards.as_deref() {
                Some([]) => "; the netlist writes no card for it".to_owned(),
                Some(cards) => format!("; the netlist writes it as {}", written_as(cards)),
                None => "; the netlist cannot write this instance".to_owned(),
            };
            if component.name.trim().is_empty() {
                let id = self.next_id();
                self.add_violation(
                    result,
                    DrcViolation::new(
                        id,
                        DrcViolationType::EmptyName,
                        format!(
                            "{} component #{} has no reference designator{consequence}",
                            component.component_type, component.id
                        ),
                        DrcLocation::Component {
                            id: component.id,
                            name: String::new(),
                        },
                    )
                    .with_instance_severity(written),
                );
            } else if let Some(error) = component.reference_error.as_deref() {
                let id = self.next_id();
                self.add_violation(
                    result,
                    DrcViolation::new(
                        id,
                        DrcViolationType::InvalidName,
                        format!(
                            "Invalid reference '{}': {}{consequence}",
                            component.name,
                            error.trim_end_matches('.')
                        ),
                        DrcLocation::Component {
                            id: component.id,
                            name: component.name.clone(),
                        },
                    )
                    .with_instance_severity(written),
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

    /// Report every emitted instance whose type resolves to nothing the deck
    /// can write.
    ///
    /// Only an object that emits a SPICE instance is asked: a structural
    /// object writes none. Extraction only ever answers `Some(false)` for a
    /// cell instance whose master it could not resolve, and such an instance
    /// cannot be written under any name, so the finding is an error.
    /// `component_known == None` means the extraction boundary held no
    /// hierarchy authority, and stays silent either way.
    fn check_unknown_components(&mut self, components: &[ComponentInfo], result: &mut DrcResult) {
        let mut components = components
            .iter()
            .filter(|component| {
                component.reference_required && component.component_known == Some(false)
            })
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
                .with_instance_severity(false)
                .with_related(vec![component_name]),
            );
        }
    }

    /// Report every reference to another instance that names no single one.
    ///
    /// The deck writes such a reference as the element it names, under that
    /// element's emitted name; a reference that names none, or several, leaves
    /// it an element the engine does not have, and the run fails at parse.
    fn check_instance_references(&mut self, components: &[ComponentInfo], result: &mut DrcResult) {
        let mut components = components
            .iter()
            .filter(|component| !component.broken_references.is_empty())
            .collect::<Vec<_>>();
        components.sort_by_key(|component| component.id);
        for component in components {
            let identity = component_identity(component);
            for broken in &component.broken_references {
                let names = if broken.ambiguous {
                    "more than one instance"
                } else {
                    "no instance"
                };
                let id = self.next_id();
                self.add_violation(
                    result,
                    DrcViolation::new(
                        id,
                        DrcViolationType::UnresolvedReference,
                        format!(
                            "{identity} {}='{}' names {names} of this cellview",
                            broken.parameter, broken.reference
                        ),
                        DrcLocation::Component {
                            id: component.id,
                            name: component.name.clone(),
                        },
                    )
                    .with_related(vec![identity.clone(), broken.reference.clone()]),
                );
            }
        }
    }

    /// Report every terminal the extraction found nothing else meeting.
    ///
    /// Three findings, because an unattached terminal is three different
    /// claims. A declared input nothing reaches is a pin the design said it
    /// would feed and then did not, so it is an error — and so is a declared
    /// supply pin, which states that the parent feeds this rail: left open,
    /// the rail is fed by nothing. A declared output nothing reaches still
    /// solves and merely loses the signal that was drawn, so it is a warning.
    /// And a terminal no interface classified — every passive leg and every
    /// primitive device terminal, which is most of a real schematic — says
    /// nothing about direction at all, so it keeps the unclassified finding:
    /// `InOut` is a declaration that deliberately declines to say, and is read
    /// the same way as no declaration.
    ///
    /// Two terminals are not open pins at all. An input the netlister writes
    /// `null` for when nothing meets it may legally stay open. And an
    /// interface port's own terminal is where the node leaves for the parent,
    /// so nothing inside the cellview has to meet it.
    fn check_unconnected_pins(&mut self, components: &[ComponentInfo], result: &mut DrcResult) {
        let mut ordered_components = components
            .iter()
            .filter(|component| !component.is_interface_port)
            .collect::<Vec<_>>();
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
                if pin.attached || pin.optional {
                    continue;
                }
                let id = self.next_id();
                let component_name = component_identity(component);
                let (violation_type, message) = match pin.direction {
                    Some(PortDirection::In) => (
                        DrcViolationType::UndrivenInput,
                        format!("Input pin {}.{} has no driver", component_name, pin.name),
                    ),
                    Some(PortDirection::Supply) => (
                        DrcViolationType::UndrivenInput,
                        format!(
                            "Supply pin {}.{} is not connected; nothing feeds its rail",
                            component_name, pin.name
                        ),
                    ),
                    Some(PortDirection::Out) => (
                        DrcViolationType::UnloadedOutput,
                        format!("Output pin {}.{} drives nothing", component_name, pin.name),
                    ),
                    None | Some(PortDirection::InOut) => (
                        DrcViolationType::UnconnectedPin,
                        format!("Pin {}.{} is not connected", component_name, pin.name),
                    ),
                };
                self.add_violation(
                    result,
                    DrcViolation::new(
                        id,
                        violation_type,
                        message,
                        DrcLocation::Terminal {
                            component_id: component.id,
                            component_name: component.name.clone(),
                            pin_name: pin.name.clone(),
                            point: pin.point,
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

    /// Report every set of instances the deck writes cards of one name for.
    ///
    /// The engine refuses two elements whose names differ only in case within
    /// one scope, and each cellview is a scope of its own: the top level, or
    /// its own subcircuit. So it is the written names that are compared, and
    /// compared folded, not the drawn ones: a resistor drawn `C4` is written
    /// `RC4` beside a drawn `RC4`, an unnamed resistor #4 is written `R4`
    /// beside a drawn `R4`, and `R1` beside `r1` is one name to the engine.
    /// Every card the deck writes is compared, not only an instance's own
    /// element: a source drawn `VSENSE_H1` is a second card of the sense
    /// source written beside `H1`, and a transformer drawn `T1` writes no `T1`
    /// for a transmission line drawn `T1` to collide with. Instances that
    /// collide on several cards, as two sources drawn `H1` do on the source and
    /// on its sense source, are one finding, stated on the card that matters
    /// most. An instance the deck cannot write at all is the reference rule's
    /// error, and collides with nothing.
    fn check_duplicate_names(&mut self, components: &[ComponentInfo], result: &mut DrcResult) {
        type Sharing<'a> = Vec<(&'a ComponentInfo, &'a WrittenCard)>;
        let mut written = BTreeMap::<String, Sharing<'_>>::new();
        for component in components {
            for card in component.written_cards.iter().flatten() {
                written
                    .entry(card.name.to_ascii_uppercase())
                    .or_default()
                    .push((component, card));
            }
        }

        let mut collisions = BTreeMap::<Vec<u64>, Sharing<'_>>::new();
        let significance = |sharing: &Sharing<'_>| sharing.iter().map(|(_, card)| card.role).min();
        for mut sharing in written.into_values() {
            if sharing.len() < 2 {
                continue;
            }
            sharing.sort_by_key(|(component, card)| (component.id, card.role));
            let between = sharing
                .iter()
                .map(|(component, _)| component.id)
                .collect::<Vec<_>>();
            match collisions.entry(between) {
                Entry::Vacant(entry) => {
                    entry.insert(sharing);
                }
                Entry::Occupied(mut entry) => {
                    if significance(&sharing) < significance(entry.get()) {
                        entry.insert(sharing);
                    }
                }
            }
        }
        let mut collisions = collisions.into_values().collect::<Vec<_>>();
        collisions.sort_by_key(|sharing| sharing[0].1.name.to_ascii_uppercase());

        for sharing in collisions {
            let drawn = sharing
                .iter()
                .map(|(component, _)| component_identity(component))
                .collect::<Vec<_>>();
            let cards = sharing
                .iter()
                .map(|(component, card)| card_label(component, card))
                .collect::<Vec<_>>();
            let mut spellings = sharing
                .iter()
                .map(|(_, card)| card.name.clone())
                .collect::<Vec<_>>();
            spellings.sort();
            spellings.dedup();
            let message = match spellings.as_slice() {
                [spelling] if cards.iter().all(|card| *card == cards[0]) => {
                    let written = if drawn[0] == *spelling {
                        String::new()
                    } else {
                        format!(", written {spelling}")
                    };
                    format!(
                        "Duplicate component name '{}' ({} instances){written}",
                        drawn[0],
                        drawn.len()
                    )
                }
                [spelling] => {
                    let quantity = if cards.len() == 2 { "both" } else { "all" };
                    format!("{} are {quantity} written {spelling}", listed(&cards))
                }
                _ => format!(
                    "{} are written {}, which the simulator reads as one name",
                    listed(&cards),
                    listed(&spellings)
                ),
            };
            let (first, _) = sharing[0];
            let id = self.next_id();
            self.add_violation(
                result,
                DrcViolation::new(
                    id,
                    DrcViolationType::DuplicateName,
                    message,
                    DrcLocation::Component {
                        id: first.id,
                        name: first.name.clone(),
                    },
                )
                .with_related(drawn),
            );
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

    /// Report every node with fewer connections than the configured minimum.
    ///
    /// A node that continues outside the cellview — through an interface port,
    /// or as a global the deck declares — is connected to something no
    /// terminal here represents, so that counts as one connection: an
    /// interface port a cell does not use inside is the parent's net in the
    /// deck, not a dangling node. Whether anything else in the design does
    /// touch a global is a question about the whole design, which a
    /// hierarchy check answers once over every cellview it walks.
    ///
    /// A node made only of terminals nothing meets is an open pin, and the
    /// open-pin rule names the terminal itself; saying it a second time about
    /// the node would report one slip twice. The node is still reported when
    /// that rule is off, because then nothing else would say it — except for
    /// a terminal that may legally stay open, whose node the deck never writes.
    fn check_floating_nodes(&mut self, net_map: &HashMap<String, NetInfo>, result: &mut DrcResult) {
        let mut names = net_map.keys().collect::<Vec<_>>();
        names.sort();
        for name in names {
            let net = &net_map[name];
            if net.connection_count == 0 {
                continue;
            }
            let only_open_terminals = net.unattached_count == net.connection_count;
            if only_open_terminals
                && (self.config.check_unconnected_pins
                    || net.optional_open_count == net.connection_count)
            {
                continue;
            }
            let connections = net.connection_count + usize::from(net.continues_outside);
            if connections < self.config.min_connections && !net.is_ground {
                let id = self.next_id();
                self.add_violation(
                    result,
                    DrcViolation::new(
                        id,
                        DrcViolationType::FloatingNode,
                        format!("Node '{}' has only {} connection(s)", name, connections),
                        DrcLocation::Node {
                            net_name: name.clone(),
                        },
                    )
                    .with_related(net.connected_components.clone()),
                );
            }
        }
    }

    /// Report each loop the independent voltage sources close.
    ///
    /// Sources that share a node are an ordinary circuit. Only a loop made of
    /// sources alone leaves its current undetermined, and that is exactly
    /// what the engine refuses (see [`source_loops`]). Each source that
    /// closes a loop is one finding, as it is one refusal there: it names the
    /// sources the loop runs through and stands at the node the closing source
    /// holds against the rest.
    fn check_source_to_source(&mut self, components: &[ComponentInfo], result: &mut DrcResult) {
        let mut sources = components
            .iter()
            .filter(|component| component.is_voltage_source)
            .filter_map(|component| {
                let terminal = |name: &str| {
                    component
                        .pins
                        .iter()
                        .find(|pin| pin.name == name)
                        .map(|pin| pin.net_name.as_str())
                };
                Some((component, [terminal("+")?, terminal("-")?]))
            })
            .collect::<Vec<_>>();
        sources.sort_by_key(|(component, _)| component.id);
        let branches = sources
            .iter()
            .map(|(_, nodes)| nodes.map(source_loops::node_key))
            .collect::<Vec<_>>();

        for members in source_loops::closed_loops(&branches) {
            let closing = *members
                .last()
                .expect("a loop has the source that closes it");
            let (_, [plus, minus]) = sources[closing];
            let node = if source_loops::node_key(plus) == source_loops::GROUND_KEY {
                minus
            } else {
                plus
            };
            let names = members
                .iter()
                .map(|&member| component_identity(sources[member].0))
                .collect::<Vec<_>>();
            let message = if names.len() == 1 {
                format!(
                    "Voltage source {} has both terminals on node '{node}'",
                    names[0]
                )
            } else {
                format!(
                    "Voltage sources {} form a closed loop through node '{node}'",
                    names.join(", ")
                )
            };
            let id = self.next_id();
            self.add_violation(
                result,
                DrcViolation::new(
                    id,
                    DrcViolationType::SourceToSource,
                    message,
                    DrcLocation::Node {
                        net_name: node.to_owned(),
                    },
                )
                .with_related(names),
            );
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
    schematic: &crate::schematic::document::SchematicDocument,
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
/// This is where [`crate::connectivity_contract::BundleWidthMismatchPolicy`] decides something.
/// Under `BlockConnection` a mismatched join is an error: the deck would have
/// to invent or drop conductors to emit it. Under `ExplicitSliceOrExtend` the
/// project has said it will slice or extend deliberately, so a mismatch the
/// designer can resolve with an explicit selector is stated as a warning — but
/// only when the bus is at least as wide as the connection asks for. A bus with
/// too few conductors cannot be sliced into a wider connection under any
/// policy, so that one stays an error.
pub(super) fn append_vector_width_violations(
    connectivity: &ExtractedConnectivity,
    policy: &crate::connectivity_contract::ConnectivityPolicy,
    result: &mut DrcResult,
    severity_overrides: &HashMap<DrcViolationType, DrcSeverity>,
) {
    let mut next_id = result.total_count();
    for mismatch in &connectivity.vector_nets.mismatches {
        let sliceable = policy.width_mismatch
            == crate::connectivity_contract::BundleWidthMismatchPolicy::ExplicitSliceOrExtend
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
    schematic: &crate::schematic::document::SchematicDocument,
    result: &mut DrcResult,
    severity_overrides: &HashMap<DrcViolationType, DrcSeverity>,
) {
    let mut labels: Vec<&crate::schematic::net_label::NetLabel> =
        schematic.net_labels.iter().collect();
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

/// What a duplicate finding calls one written card: the instance, for its own
/// element, and otherwise the card's part in it, as in `H1's sense source`.
fn card_label(component: &ComponentInfo, card: &WrittenCard) -> String {
    let identity = component_identity(component);
    let part = match card.role {
        CardRole::Element => return identity,
        CardRole::SenseSource => "sense source",
        CardRole::PrimaryWinding => "primary winding",
        CardRole::SecondaryWinding => "secondary winding",
        CardRole::Coupling => "coupling",
    };
    format!("{identity}'s {part}")
}

/// The names a reference finding says the deck writes an instance under: its
/// own element's, and where it has none, every card written in its place.
fn written_as(cards: &[WrittenCard]) -> String {
    match cards.iter().find(|card| card.role == CardRole::Element) {
        Some(element) => element.name.clone(),
        None => listed(
            &cards
                .iter()
                .map(|card| card.name.clone())
                .collect::<Vec<_>>(),
        ),
    }
}

/// Two or more names as a sentence lists them: `a and b`, `a, b and c`.
fn listed(names: &[String]) -> String {
    match names.split_last() {
        Some((last, leading)) if !leading.is_empty() => {
            format!("{} and {last}", leading.join(", "))
        }
        _ => names.join(""),
    }
}

impl Default for DrcChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;
