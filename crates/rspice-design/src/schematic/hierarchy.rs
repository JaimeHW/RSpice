//! Transactional extraction of selected schematic instances into a child cell.
//!
//! The planner is deliberately independent of the library/workspace owner. It
//! consumes one immutable connectivity receipt and produces complete parent and
//! child candidates. The application validates both candidates against the
//! project hierarchy before publishing either document.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

use super::{
    component::{Component, LibraryCellInstance},
    component_edit::{self, ComponentPlacement},
    component_type::ComponentType,
    document::SchematicDocument,
    identity::SchematicIdentity,
    junction_edit,
    net_label::NetLabel,
    port::{PortContract, PortDirection, PortDiscipline, PortSignalType, PortSpec},
    rotation::Rotation,
    terminal_connection, wire_edit,
};
use rspice_design_model::Point;

/// Immutable source document, allocation state and selection facts for planning.
pub struct HierarchySource<'a> {
    pub document: &'a SchematicDocument,
    pub identity: &'a SchematicIdentity,
    pub topology_version: u64,
    pub selected_components: &'a HashSet<u64>,
    pub selected_count: usize,
}

/// A candidate document with its allocator and exact topology invalidation count.
#[derive(Debug)]
pub struct HierarchyDocument {
    pub document: SchematicDocument,
    pub identity: SchematicIdentity,
    pub topology_changes: u64,
}

impl HierarchyDocument {
    fn new(document: SchematicDocument, identity: SchematicIdentity) -> Self {
        Self {
            document,
            identity,
            topology_changes: 0,
        }
    }
    fn repair_document(&mut self) {
        let repaired = self.identity.repair_document(&mut self.document);
        self.topology_changes = self
            .topology_changes
            .wrapping_add(repaired.topology_changes);
    }
    fn add_component(&mut self, kind: ComponentType, position: Point) -> u64 {
        let id = component_edit::add_component(
            &mut self.document,
            &mut self.identity,
            kind,
            ComponentPlacement {
                position,
                rotation: Rotation::default(),
                mirror_h: false,
            },
            None,
        );
        self.topology_changes = self.topology_changes.wrapping_add(1);
        id
    }
    fn add_library_cell_component(
        &mut self,
        position: Point,
        binding: LibraryCellInstance,
        rotation: Rotation,
        mirror_h: bool,
    ) -> u64 {
        let id = component_edit::add_library_cell_component(
            &mut self.document,
            &mut self.identity,
            ComponentPlacement {
                position,
                rotation,
                mirror_h,
            },
            binding,
        );
        self.topology_changes = self.topology_changes.wrapping_add(1);
        id
    }
    fn add_wire(&mut self, points: Vec<Point>) {
        if wire_edit::add_wire(&mut self.document, &mut self.identity, points).is_some() {
            self.topology_changes = self.topology_changes.wrapping_add(1);
        }
    }
    fn add_junction(&mut self, pos: Point) {
        if junction_edit::add_junction(&mut self.document, &mut self.identity, pos).1 {
            self.topology_changes = self.topology_changes.wrapping_add(1);
        }
    }
    fn add_net_label(&mut self, pos: Point, name: String) {
        junction_edit::add_net_label(&mut self.document, &mut self.identity, pos, name);
        self.topology_changes = self.topology_changes.wrapping_add(1);
    }
}

type ComponentObstacle = (u64, (i32, i32, i32, i32));

/// One resolved terminal in the exact source schematic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HierarchyExtractionTerminal {
    pub component_id: u64,
    pub terminal_name: String,
    pub point: Point,
    pub direction: Option<PortDirection>,
    pub discipline: PortDiscipline,
}

/// Immutable connectivity facts produced by the canonical net extractor.
#[derive(Debug, Clone, PartialEq)]
pub struct HierarchyNetConnectivity {
    pub point_to_net: HashMap<Point, String>,
    pub net_segments: HashMap<String, Vec<(Point, Point)>>,
}

/// One inferred child interface port.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HierarchyExtractionPort {
    pub name: String,
    pub direction: PortDirection,
    pub discipline: PortDiscipline,
    pub source_net: String,
    pub parent_anchor: Point,
}

/// A reviewed extraction plan. It contains no borrowed state and can therefore
/// be retained by a modal as an exact commit authority.
#[derive(Debug, Clone, PartialEq)]
pub struct HierarchyExtractionPlan {
    pub topology_version: u64,
    pub source_component_ids: Vec<u64>,
    pub source_instance_names: Vec<String>,
    pub source_net_count: usize,
    pub origin: Point,
    pub ports: Vec<HierarchyExtractionPort>,
    point_to_net: HashMap<Point, String>,
    source_segments_by_net: HashMap<String, Vec<(Point, Point)>>,
    source_bus_segments: Vec<(Point, Point)>,
    source_terminals: Vec<HierarchyExtractionTerminal>,
    source_component_bounds: HashMap<u64, (i32, i32, i32, i32)>,
    nets: Vec<PlannedNet>,
}

/// One exact scalar boundary that must remain electrically identical when a
/// group of instances moves to another sheet in the same cell view.
///
/// The stationary side is anchored to retained wire geometry while the moved
/// side names the exact component terminal.  Keeping both identities avoids
/// the ambiguous "object origin" anchors that previously made multi-terminal
/// instances impossible to materialize safely.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SheetMoveBoundary {
    pub net_name: String,
    pub direction: PortDirection,
    pub discipline: PortDiscipline,
    pub stationary_wire_id: u64,
    pub stationary_point: Point,
    pub moved_component_id: u64,
    pub moved_terminal_name: String,
}

/// Complete, topology-bound object and boundary plan for a same-cell sheet
/// move. Internal scalar conductors move with the selected instances; shared
/// nets remain on the source sheet and receive explicit typed contracts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SheetMoveConnectivityPlan {
    pub topology_version: u64,
    pub source_component_ids: Vec<u64>,
    pub moved_object_ids: Vec<u64>,
    pub boundaries: Vec<SheetMoveBoundary>,
}

impl HierarchyExtractionPlan {
    pub fn internal_source_net_names(&self) -> impl Iterator<Item = &str> {
        self.nets
            .iter()
            .filter(|net| !net.boundary && !net.global_ground)
            .map(|net| net.source_name.as_str())
    }

    /// Convert the canonical hierarchy connectivity receipt into a same-cell
    /// sheet-move plan without guessing at net or terminal identity.
    pub fn sheet_move_connectivity(
        &self,
        document: &SchematicDocument,
        topology_version: u64,
    ) -> Result<SheetMoveConnectivityPlan, HierarchyExtractionError> {
        if self.topology_version != topology_version {
            return Err(HierarchyExtractionError::InvalidConnectivity(
                "the schematic topology changed while the sheet move was being reviewed".to_owned(),
            ));
        }

        let selected = self
            .source_component_ids
            .iter()
            .copied()
            .collect::<HashSet<_>>();
        let mut moved_object_ids = self
            .source_component_ids
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        let mut boundaries = Vec::new();

        for net in &self.nets {
            let selected_terminals = self
                .source_terminals
                .iter()
                .filter(|terminal| {
                    selected.contains(&terminal.component_id)
                        && self.point_to_net.get(&terminal.point) == Some(&net.source_name)
                })
                .collect::<Vec<_>>();
            if selected_terminals.is_empty() {
                continue;
            }

            if net.boundary || net.global_ground {
                let direction = inferred_direction(&net.source_name, &selected_terminals);
                let discipline = inferred_discipline(&net.source_name, &selected_terminals)?;
                for terminal in selected_terminals {
                    let connection = document.connections
                        .iter()
                        .find(|connection| {
                            connection.component_id == terminal.component_id
                                && connection
                                    .terminal_name
                                    .eq_ignore_ascii_case(&terminal.terminal_name)
                        })
                        .ok_or_else(|| {
                            HierarchyExtractionError::InvalidConnectivity(format!(
                                "terminal {}:{} has no canonical wire connection for a cross-sheet boundary",
                                terminal.component_id, terminal.terminal_name
                            ))
                        })?;
                    let wire = document
                        .wires
                        .iter()
                        .find(|wire| wire.id == connection.wire_id)
                        .ok_or_else(|| {
                            HierarchyExtractionError::InvalidConnectivity(format!(
                                "terminal {}:{} references missing wire {}",
                                terminal.component_id, terminal.terminal_name, connection.wire_id
                            ))
                        })?;
                    let stationary_point = wire
                        .points
                        .get(connection.point_index)
                        .copied()
                        .ok_or_else(|| {
                            HierarchyExtractionError::InvalidConnectivity(format!(
                                "terminal {}:{} references invalid point {} on wire {}",
                                terminal.component_id,
                                terminal.terminal_name,
                                connection.point_index,
                                connection.wire_id
                            ))
                        })?;
                    if stationary_point != terminal.point
                        || self.point_to_net.get(&stationary_point) != Some(&net.source_name)
                    {
                        return Err(HierarchyExtractionError::InvalidConnectivity(format!(
                            "terminal {}:{} no longer resolves to retained net '{}'",
                            terminal.component_id, terminal.terminal_name, net.source_name
                        )));
                    }
                    boundaries.push(SheetMoveBoundary {
                        net_name: net.source_name.clone(),
                        direction,
                        discipline,
                        stationary_wire_id: wire.id,
                        stationary_point,
                        moved_component_id: terminal.component_id,
                        moved_terminal_name: terminal.terminal_name.clone(),
                    });
                }
                continue;
            }

            for wire in &document.wires {
                let wire_net = wire
                    .points
                    .iter()
                    .find_map(|point| self.point_to_net.get(point));
                if wire_net == Some(&net.source_name) {
                    if wire
                        .points
                        .iter()
                        .filter_map(|point| self.point_to_net.get(point))
                        .any(|resolved| resolved != &net.source_name)
                    {
                        return Err(HierarchyExtractionError::InvalidConnectivity(format!(
                            "wire {} spans more than one canonical net",
                            wire.id
                        )));
                    }
                    moved_object_ids.insert(wire.id);
                }
            }
            for junction in &document.junctions {
                if self.point_to_net.get(&junction.pos) == Some(&net.source_name) {
                    moved_object_ids.insert(junction.id);
                }
            }
            for label in &document.net_labels {
                if self.point_to_net.get(&label.pos) == Some(&net.source_name) {
                    moved_object_ids.insert(label.id);
                }
            }
        }

        boundaries.sort_by(|left, right| {
            (
                left.net_name.to_ascii_lowercase(),
                left.moved_component_id,
                left.moved_terminal_name.to_ascii_lowercase(),
                left.stationary_wire_id,
                left.stationary_point.x,
                left.stationary_point.y,
            )
                .cmp(&(
                    right.net_name.to_ascii_lowercase(),
                    right.moved_component_id,
                    right.moved_terminal_name.to_ascii_lowercase(),
                    right.stationary_wire_id,
                    right.stationary_point.x,
                    right.stationary_point.y,
                ))
        });

        Ok(SheetMoveConnectivityPlan {
            topology_version: self.topology_version,
            source_component_ids: self.source_component_ids.clone(),
            moved_object_ids: moved_object_ids.into_iter().collect(),
            boundaries,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct PlannedNet {
    source_name: String,
    selected_points: Vec<Point>,
    /// Exact authored conductor geometry that belongs in the extracted cell.
    /// Each path is an original (possibly split) source segment; it is never a
    /// synthesized shortest path between terminals.
    paths: Vec<Vec<Point>>,
    child_junctions: Vec<Point>,
    child_labels: Vec<(Point, String)>,
    parent_segments: Vec<(Point, Point)>,
    boundary: bool,
    global_ground: bool,
}

/// Complete non-mutating result ready for project-level validation/publication.
#[derive(Debug)]
pub struct HierarchyExtractionCandidate {
    pub parent: HierarchyDocument,
    pub child: HierarchyDocument,
    pub instance_id: u64,
    pub instance_name: String,
    pub binding: LibraryCellInstance,
}

pub fn validate_candidate_connectivity(
    parent: &SchematicDocument,
    child: &SchematicDocument,
    instance_id: u64,
    plan: &HierarchyExtractionPlan,
    parent_point_to_net: &HashMap<Point, String>,
    child_point_to_net: &HashMap<Point, String>,
) -> Result<(), HierarchyExtractionError> {
    let instance = parent
        .components
        .iter()
        .find(|component| component.id == instance_id)
        .ok_or_else(|| {
            HierarchyExtractionError::CandidateConnectivity(
                "the parent candidate lost the generated hierarchy instance".to_owned(),
            )
        })?;
    let instance_terminals = instance
        .terminal_positions()
        .into_iter()
        .map(|(_, point)| point)
        .collect::<Vec<_>>();
    let mut parent_nets = HashSet::new();
    for (port, terminal) in plan.ports.iter().zip(instance_terminals) {
        let pin_net = parent_point_to_net.get(&terminal).ok_or_else(|| {
            HierarchyExtractionError::CandidateConnectivity(format!(
                "parent pin '{}' is disconnected",
                port.name
            ))
        })?;
        let anchor_net = parent_point_to_net
            .get(&port.parent_anchor)
            .ok_or_else(|| {
                HierarchyExtractionError::CandidateConnectivity(format!(
                    "retained parent net '{}' lost its boundary anchor",
                    port.source_net
                ))
            })?;
        if pin_net != anchor_net {
            return Err(HierarchyExtractionError::CandidateConnectivity(format!(
                "parent pin '{}' does not reach its retained boundary net",
                port.name
            )));
        }
        if !parent_nets.insert(pin_net.clone()) {
            return Err(HierarchyExtractionError::CandidateConnectivity(
                "two distinct boundary nets would be merged in the parent".to_owned(),
            ));
        }
    }

    let retained_sources = plan
        .nets
        .iter()
        .filter(|net| net.boundary || net.global_ground)
        .map(|net| net.source_name.as_str())
        .collect::<HashSet<_>>();
    let mut source_by_candidate = HashMap::<&str, &str>::new();
    let mut candidate_by_source = HashMap::<&str, &str>::new();
    for (point, source) in &plan.point_to_net {
        if !retained_sources.contains(source.as_str()) {
            continue;
        }
        let Some(candidate) = parent_point_to_net.get(point) else {
            continue;
        };
        if let Some(previous) = source_by_candidate.insert(candidate, source)
            && previous != source
        {
            return Err(HierarchyExtractionError::CandidateConnectivity(format!(
                "automatic hierarchy routing would merge source nets '{previous}' and '{source}'"
            )));
        }
        if let Some(previous) = candidate_by_source.insert(source, candidate)
            && previous != candidate
        {
            return Err(HierarchyExtractionError::CandidateConnectivity(format!(
                "retained source net '{source}' would be split in the parent"
            )));
        }
    }

    let mut child_net_by_source = HashMap::<&str, String>::new();
    let mut distinct_child_nets = HashSet::new();
    for net in &plan.nets {
        let mut resolved = None::<String>;
        for source_point in &net.selected_points {
            let child_point = checked_sub(*source_point, plan.origin)?;
            let actual = child_point_to_net.get(&child_point).ok_or_else(|| {
                HierarchyExtractionError::CandidateConnectivity(format!(
                    "a moved terminal on '{}' is disconnected in the child",
                    net.source_name
                ))
            })?;
            match &resolved {
                Some(expected) if expected != actual => {
                    return Err(HierarchyExtractionError::CandidateConnectivity(format!(
                        "source net '{}' was split in the child",
                        net.source_name
                    )));
                }
                None => resolved = Some(actual.clone()),
                _ => {}
            }
        }
        let actual = resolved.ok_or_else(|| {
            HierarchyExtractionError::CandidateConnectivity(format!(
                "source net '{}' has no moved terminal",
                net.source_name
            ))
        })?;
        if net.global_ground && actual != "0" {
            return Err(HierarchyExtractionError::CandidateConnectivity(
                "global node 0 was not preserved in the extracted cell".to_owned(),
            ));
        }
        if !net.boundary
            && !net.global_ground
            && !auto_net_name(&net.source_name)
            && !actual.eq_ignore_ascii_case(&net.source_name)
        {
            return Err(HierarchyExtractionError::CandidateConnectivity(format!(
                "internal net '{}' lost its canonical name in the child",
                net.source_name
            )));
        }
        if !distinct_child_nets.insert(actual.clone()) {
            return Err(HierarchyExtractionError::CandidateConnectivity(
                "two distinct source nets would be merged in the child".to_owned(),
            ));
        }
        child_net_by_source.insert(net.source_name.as_str(), actual);
    }
    for port in &plan.ports {
        let component = child
            .components
            .iter()
            .find(|component| {
                component.kind == ComponentType::Port
                    && component.value.eq_ignore_ascii_case(&port.name)
            })
            .ok_or_else(|| {
                HierarchyExtractionError::CandidateConnectivity(format!(
                    "child port '{}' is missing",
                    port.name
                ))
            })?;
        let terminal = component
            .terminal_positions()
            .first()
            .map(|(_, point)| *point)
            .ok_or_else(|| {
                HierarchyExtractionError::CandidateConnectivity(format!(
                    "child port '{}' has no terminal",
                    port.name
                ))
            })?;
        let port_net = child_point_to_net.get(&terminal).ok_or_else(|| {
            HierarchyExtractionError::CandidateConnectivity(format!(
                "child port '{}' is disconnected",
                port.name
            ))
        })?;
        if child_net_by_source.get(port.source_net.as_str()) != Some(port_net) {
            return Err(HierarchyExtractionError::CandidateConnectivity(format!(
                "child port '{}' does not reach source net '{}'",
                port.name, port.source_net
            )));
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HierarchyExtractionError {
    ReadOnly,
    SelectInstances,
    MissingInstance(u64),
    InterfacePortSelected(String),
    MissingTerminalNet { instance: String, terminal: String },
    InvalidConnectivity(String),
    InvalidPortName(String),
    CoordinateOverflow,
    NoPlacement,
    CandidateConnectivity(String),
}

impl std::fmt::Display for HierarchyExtractionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadOnly => formatter.write_str("the active schematic is read-only"),
            Self::SelectInstances => formatter.write_str(
                "select one or more complete instances and no wires, labels, buses, or graphics",
            ),
            Self::MissingInstance(id) => {
                write!(formatter, "selected instance {id} no longer exists")
            }
            Self::InterfacePortSelected(name) => write!(
                formatter,
                "interface port '{name}' belongs to the parent cell contract and cannot move into a child"
            ),
            Self::MissingTerminalNet { instance, terminal } => write!(
                formatter,
                "terminal {instance}.{terminal} has no canonical connectivity identity"
            ),
            Self::InvalidConnectivity(detail) => formatter.write_str(detail),
            Self::InvalidPortName(name) => write!(
                formatter,
                "the inferred boundary name '{name}' is not a valid interface identifier"
            ),
            Self::CoordinateOverflow => {
                formatter.write_str("the extracted geometry exceeds the schematic coordinate range")
            }
            Self::NoPlacement => formatter.write_str(
                "no collision-free parent placement is available near the selected instances",
            ),
            Self::CandidateConnectivity(detail) => formatter.write_str(detail),
        }
    }
}

impl std::error::Error for HierarchyExtractionError {}

impl HierarchySource<'_> {
    /// Plan the exact selected-instance extraction without mutating the document.
    pub fn plan_hierarchy_extraction(
        &self,
        terminals: &[HierarchyExtractionTerminal],
        connectivity: &HierarchyNetConnectivity,
        component_bounds: &HashMap<u64, (i32, i32, i32, i32)>,
    ) -> Result<HierarchyExtractionPlan, HierarchyExtractionError> {
        if self.selected_components.is_empty()
            || self.selected_count != self.selected_components.len()
        {
            return Err(HierarchyExtractionError::SelectInstances);
        }

        let mut source_component_ids = self.selected_components.iter().copied().collect::<Vec<_>>();
        source_component_ids.sort_unstable();
        let selected = source_component_ids.iter().copied().collect::<HashSet<_>>();
        let mut source_components = Vec::with_capacity(source_component_ids.len());
        for id in &source_component_ids {
            let component = self
                .document
                .components
                .iter()
                .find(|component| component.id == *id)
                .ok_or(HierarchyExtractionError::MissingInstance(*id))?;
            if component.kind == ComponentType::Port {
                return Err(HierarchyExtractionError::InterfacePortSelected(
                    component.value.trim().to_owned(),
                ));
            }
            source_components.push(component);
        }

        let origin = snapped_selection_center(&source_components, self.document.grid_size)?;
        let mut terminals_by_net = BTreeMap::<String, Vec<&HierarchyExtractionTerminal>>::new();
        for terminal in terminals
            .iter()
            .filter(|terminal| selected.contains(&terminal.component_id))
        {
            let net = connectivity
                .point_to_net
                .get(&terminal.point)
                .ok_or_else(|| {
                    let instance = source_components
                        .iter()
                        .find(|component| component.id == terminal.component_id)
                        .map_or_else(
                            || terminal.component_id.to_string(),
                            |component| component.name.clone(),
                        );
                    HierarchyExtractionError::MissingTerminalNet {
                        instance,
                        terminal: terminal.terminal_name.clone(),
                    }
                })?;
            terminals_by_net
                .entry(net.clone())
                .or_default()
                .push(terminal);
        }
        if terminals_by_net.is_empty() {
            return Err(HierarchyExtractionError::InvalidConnectivity(
                "the selected instances expose no electrical terminals".to_owned(),
            ));
        }

        let bus_bound_nets = self
            .document
            .bus_taps
            .iter()
            .filter_map(|tap| {
                connectivity
                    .point_to_net
                    .get(&tap.connection_point)
                    .cloned()
            })
            .collect::<HashSet<_>>();
        let mut reserved_names = HashSet::<String>::new();
        let mut ports = Vec::new();
        let mut nets = Vec::with_capacity(terminals_by_net.len());
        for (source_name, mut selected_terminals) in terminals_by_net {
            selected_terminals.sort_by_key(|terminal| {
                (
                    terminal.component_id,
                    terminal.terminal_name.to_ascii_lowercase(),
                )
            });
            let mut selected_points = selected_terminals
                .iter()
                .map(|terminal| terminal.point)
                .collect::<Vec<_>>();
            selected_points.sort_by_key(|point| (point.x, point.y));
            selected_points.dedup();
            let global_ground = source_name == "0";
            let boundary = !global_ground
                && (terminals.iter().any(|terminal| {
                    !selected.contains(&terminal.component_id)
                        && connectivity.point_to_net.get(&terminal.point) == Some(&source_name)
                }) || bus_bound_nets.contains(&source_name));
            let mut external_terminal_points = terminals
                .iter()
                .filter(|terminal| {
                    !selected.contains(&terminal.component_id)
                        && connectivity.point_to_net.get(&terminal.point) == Some(&source_name)
                        && !selected_points.contains(&terminal.point)
                })
                .map(|terminal| terminal.point)
                .collect::<HashSet<_>>();
            external_terminal_points.extend(
                self.document
                    .bus_taps
                    .iter()
                    .filter(|tap| {
                        connectivity.point_to_net.get(&tap.connection_point) == Some(&source_name)
                            && !selected_points.contains(&tap.connection_point)
                    })
                    .map(|tap| tap.connection_point),
            );
            let paths = retained_net_geometry(
                &selected_points,
                connectivity
                    .net_segments
                    .get(&source_name)
                    .map(Vec::as_slice)
                    .unwrap_or(&[]),
                !boundary && !global_ground,
                &external_terminal_points,
            );
            let parent_segments = if boundary || global_ground {
                connectivity
                    .net_segments
                    .get(&source_name)
                    .cloned()
                    .unwrap_or_default()
            } else {
                Vec::new()
            };
            let point_is_in_child = |point: Point| {
                (!boundary && !global_ground)
                    || selected_points.contains(&point)
                    || paths.iter().any(|path| {
                        path.windows(2)
                            .any(|pair| point_on_segment(point, pair[0], pair[1]))
                    })
            };
            let child_junctions = self
                .document
                .junctions
                .iter()
                .filter(|junction| {
                    connectivity.point_to_net.get(&junction.pos) == Some(&source_name)
                        && point_is_in_child(junction.pos)
                })
                .map(|junction| junction.pos)
                .collect::<Vec<_>>();
            let child_labels = self
                .document
                .net_labels
                .iter()
                .filter(|label| {
                    connectivity.point_to_net.get(&label.pos) == Some(&source_name)
                        && point_is_in_child(label.pos)
                })
                .map(|label| (label.pos, label.name.clone()))
                .collect::<Vec<_>>();

            if boundary {
                let direction = inferred_direction(&source_name, &selected_terminals);
                let discipline = inferred_discipline(&source_name, &selected_terminals)?;
                let base = inferred_port_name(
                    &source_name,
                    source_components.as_slice(),
                    selected_terminals[0],
                    ports.len() + 1,
                );
                let name = unique_port_name(base, &mut reserved_names);
                NetLabel::validate_name(&name, self.document.document_policy.net_naming)
                    .map_err(|_| HierarchyExtractionError::InvalidPortName(name.clone()))?;
                ports.push(HierarchyExtractionPort {
                    name,
                    direction,
                    discipline,
                    source_net: source_name.clone(),
                    parent_anchor: selected_points[0],
                });
            }
            nets.push(PlannedNet {
                source_name,
                selected_points,
                paths,
                child_junctions,
                child_labels,
                parent_segments,
                boundary,
                global_ground,
            });
        }

        Ok(HierarchyExtractionPlan {
            topology_version: self.topology_version,
            source_component_ids,
            source_instance_names: source_components
                .iter()
                .map(|component| component.name.clone())
                .collect(),
            source_net_count: nets.len(),
            origin,
            ports,
            point_to_net: connectivity.point_to_net.clone(),
            source_segments_by_net: connectivity.net_segments.clone(),
            source_bus_segments: self
                .document
                .buses
                .iter()
                .flat_map(|bus| bus.points.windows(2).map(|pair| (pair[0], pair[1])))
                .collect(),
            source_terminals: terminals.to_vec(),
            source_component_bounds: component_bounds.clone(),
            nets,
        })
    }

    /// Materialize complete parent/child candidates from a retained plan.
    pub fn materialize_hierarchy_extraction(
        &self,
        plan: &HierarchyExtractionPlan,
        library: &str,
        cell: &str,
        view: &str,
        parent_rotation: Rotation,
        parent_mirror_h: bool,
    ) -> Result<HierarchyExtractionCandidate, HierarchyExtractionError> {
        if self.topology_version != plan.topology_version {
            return Err(HierarchyExtractionError::InvalidConnectivity(
                "the schematic changed after the hierarchy preview was created".to_owned(),
            ));
        }
        let selected = plan
            .source_component_ids
            .iter()
            .copied()
            .collect::<HashSet<_>>();
        if *self.selected_components != selected || self.selected_count != selected.len() {
            return Err(HierarchyExtractionError::SelectInstances);
        }

        let mut child = HierarchyDocument::new(
            SchematicDocument::default(),
            SchematicIdentity::with_cursor(1),
        );
        child.document.grid_size = self.document.grid_size;
        child.document.document_policy = self.document.document_policy;
        for component in self
            .document
            .components
            .iter()
            .filter(|component| selected.contains(&component.id))
        {
            let mut moved = component.clone();
            moved.pos = checked_sub(moved.pos, plan.origin)?;
            child.document.components.push(moved);
        }
        child.repair_document();

        for net in &plan.nets {
            for path in &net.paths {
                let translated = path
                    .iter()
                    .map(|point| checked_sub(*point, plan.origin))
                    .collect::<Result<Vec<_>, _>>()?;
                if translated.len() >= 2 {
                    child.add_wire(translated);
                }
            }
            for junction in &net.child_junctions {
                child.add_junction(checked_sub(*junction, plan.origin)?);
            }
            for (position, name) in &net.child_labels {
                child.add_net_label(checked_sub(*position, plan.origin)?, name.clone());
            }
        }
        for net in plan
            .nets
            .iter()
            .filter(|net| !net.boundary && !net.global_ground)
            .filter(|net| !auto_net_name(&net.source_name))
            .filter(|net| {
                !net.child_labels
                    .iter()
                    .any(|(_, name)| name.eq_ignore_ascii_case(&net.source_name))
            })
        {
            let position = checked_sub(net.selected_points[0], plan.origin)?;
            child.add_net_label(position, net.source_name.clone());
        }

        for net in plan.nets.iter().filter(|net| net.global_ground) {
            let selected_ground_is_moved = self.document.components.iter().any(|component| {
                selected.contains(&component.id)
                    && component.kind == ComponentType::Ground
                    && component
                        .terminal_positions()
                        .iter()
                        .any(|(_, point)| plan.point_to_net.get(point) == Some(&net.source_name))
            });
            if !selected_ground_is_moved {
                let terminal = checked_sub(net.selected_points[0], plan.origin)?;
                let component_y = terminal
                    .y
                    .checked_add(10)
                    .ok_or(HierarchyExtractionError::CoordinateOverflow)?;
                child.add_component(ComponentType::Ground, Point::new(terminal.x, component_y));
            }
        }

        place_child_ports(&mut child, plan)?;
        child.repair_document();
        let mut child_terminals = plan
            .source_terminals
            .iter()
            .filter(|terminal| selected.contains(&terminal.component_id))
            .map(|terminal| {
                Ok((
                    terminal.component_id,
                    terminal.terminal_name.clone(),
                    checked_sub(terminal.point, plan.origin)?,
                ))
            })
            .collect::<Result<Vec<_>, HierarchyExtractionError>>()?;
        child_terminals.extend(
            child
                .document
                .components
                .iter()
                .filter(|component| !selected.contains(&component.id))
                .flat_map(|component| {
                    component
                        .terminal_positions()
                        .into_iter()
                        .map(move |(name, point)| (component.id, name.to_owned(), point))
                }),
        );
        terminal_connection::rebuild_connections_from_terminals(
            &mut child.document,
            &child_terminals,
        );

        let child_only_nets = plan
            .nets
            .iter()
            .filter(|net| !net.boundary && !net.global_ground)
            .map(|net| net.source_name.as_str())
            .collect::<HashSet<_>>();
        let mut parent = HierarchyDocument::new(self.document.clone(), self.identity.clone());
        parent
            .document
            .components
            .retain(|component| !selected.contains(&component.id));
        parent
            .document
            .connections
            .retain(|connection| !selected.contains(&connection.component_id));
        parent.document.wires.retain(|wire| {
            !wire.points.iter().any(|point| {
                self.net_name_at_point(*point, plan)
                    .is_some_and(|net| child_only_nets.contains(net))
            })
        });
        parent.document.junctions.retain(|junction| {
            self.net_name_at_point(junction.pos, plan)
                .is_none_or(|net| !child_only_nets.contains(net))
        });
        parent.document.net_labels.retain(|label| {
            self.net_name_at_point(label.pos, plan)
                .is_none_or(|net| !child_only_nets.contains(net))
        });

        let ports = plan
            .ports
            .iter()
            .map(|port| PortSpec {
                name: port.name.clone(),
                direction: port.direction,
            })
            .collect::<Vec<_>>();
        let mut binding = LibraryCellInstance::new(library, cell, view);
        binding.bind_interface(&ports);
        let (instance_pos, parent_routes) = hierarchy_instance_position(&parent, plan, &binding)?;
        let instance_id = parent.add_library_cell_component(
            instance_pos,
            binding.clone(),
            parent_rotation,
            parent_mirror_h,
        );
        let instance = parent
            .document
            .components
            .iter_mut()
            .find(|component| component.id == instance_id)
            .expect("new hierarchy instance exists");
        instance.rotation = Rotation::R0;
        let instance_name = instance.name.clone();
        let instance_terminals = instance
            .terminal_positions()
            .into_iter()
            .map(|(name, point)| (name.to_owned(), point))
            .collect::<Vec<_>>();
        if instance_terminals.len() != plan.ports.len() {
            return Err(HierarchyExtractionError::CandidateConnectivity(
                "the generated hierarchy symbol does not match the inferred interface".to_owned(),
            ));
        }
        for route in parent_routes {
            if route.len() >= 2 {
                parent.add_wire(route);
            }
        }
        parent.topology_changes = parent.topology_changes.wrapping_add(1);
        parent.repair_document();
        let mut parent_terminals = plan
            .source_terminals
            .iter()
            .filter(|terminal| !selected.contains(&terminal.component_id))
            .map(|terminal| {
                (
                    terminal.component_id,
                    terminal.terminal_name.clone(),
                    terminal.point,
                )
            })
            .collect::<Vec<_>>();
        parent_terminals.extend(
            instance_terminals
                .into_iter()
                .map(|(name, point)| (instance_id, name, point)),
        );
        terminal_connection::rebuild_connections_from_terminals(
            &mut parent.document,
            &parent_terminals,
        );

        Ok(HierarchyExtractionCandidate {
            parent,
            child,
            instance_id,
            instance_name,
            binding,
        })
    }

    fn net_name_at_point<'a>(
        &self,
        point: Point,
        plan: &'a HierarchyExtractionPlan,
    ) -> Option<&'a str> {
        plan.point_to_net.get(&point).map(String::as_str)
    }
}

/// Best available terminal direction. Bound LCV contracts are authoritative;
/// built-in active devices provide conservative electrical intent and passive
/// or ambiguous terminals remain bidirectional.
pub fn hierarchy_terminal_direction(
    component: &Component,
    terminal_name: &str,
) -> Option<PortDirection> {
    if let Some(binding) = component.library_cell.as_ref()
        && binding.terminal_dirs.len() == binding.terminal_order.len()
        && let Some(index) = binding
            .terminal_order
            .iter()
            .position(|name| name.eq_ignore_ascii_case(terminal_name))
    {
        return binding.terminal_dirs.get(index).copied();
    }
    let terminal = terminal_name.to_ascii_lowercase();
    match component.kind {
        ComponentType::OpAmp => match terminal.as_str() {
            "out" => Some(PortDirection::Out),
            "in+" | "in-" => Some(PortDirection::In),
            _ => None,
        },
        ComponentType::Vcvs | ComponentType::Vccs | ComponentType::Ccvs | ComponentType::Cccs => {
            if terminal.starts_with('c') {
                Some(PortDirection::In)
            } else if terminal.starts_with('o') {
                Some(PortDirection::Out)
            } else {
                None
            }
        }
        ComponentType::XspiceGain
        | ComponentType::XspiceLimiter
        | ComponentType::XspiceIntegrator
        | ComponentType::XspiceDifferentiator
        | ComponentType::XspiceInverter
        | ComponentType::XspiceBuffer
        | ComponentType::XspiceAndGate
        | ComponentType::XspiceOrGate
        | ComponentType::XspiceNandGate
        | ComponentType::XspiceNorGate
        | ComponentType::XspiceXorGate
        | ComponentType::XspiceAdcBridge
        | ComponentType::XspiceDacBridge => {
            if terminal == "out" || terminal == "q" || terminal == "qn" {
                Some(PortDirection::Out)
            } else {
                Some(PortDirection::In)
            }
        }
        _ => None,
    }
}

fn inferred_direction(
    source_name: &str,
    terminals: &[&HierarchyExtractionTerminal],
) -> PortDirection {
    let lower = source_name.to_ascii_lowercase();
    if source_name == "0"
        || [
            "vdd", "vss", "vcc", "vee", "gnd", "ground", "avdd", "avss", "dvdd", "dvss",
        ]
        .iter()
        .any(|rail| lower == *rail)
    {
        return PortDirection::Supply;
    }
    let directions = terminals
        .iter()
        .filter_map(|terminal| terminal.direction)
        .collect::<HashSet<_>>();
    if directions.contains(&PortDirection::Supply) {
        PortDirection::Supply
    } else if directions.len() == 1 {
        *directions.iter().next().expect("one direction")
    } else {
        PortDirection::InOut
    }
}

fn inferred_discipline(
    source_name: &str,
    terminals: &[&HierarchyExtractionTerminal],
) -> Result<PortDiscipline, HierarchyExtractionError> {
    let disciplines = terminals
        .iter()
        .map(|terminal| terminal.discipline)
        .collect::<HashSet<_>>();
    if disciplines.len() == 1 {
        return Ok(*disciplines.iter().next().expect("one discipline"));
    }
    let detail = disciplines
        .iter()
        .map(|discipline| discipline.keyword())
        .collect::<Vec<_>>()
        .join(", ");
    Err(HierarchyExtractionError::InvalidConnectivity(format!(
        "boundary net '{source_name}' mixes incompatible disciplines ({detail})"
    )))
}

fn inferred_port_name(
    source_name: &str,
    components: &[&Component],
    first: &HierarchyExtractionTerminal,
    ordinal: usize,
) -> String {
    if source_name == "0" {
        return "VSS".to_owned();
    }
    if !auto_net_name(source_name) {
        return sanitize_identifier(source_name);
    }
    let instance = components
        .iter()
        .find(|component| component.id == first.component_id)
        .map_or("PORT", |component| component.name.as_str());
    let inferred = sanitize_identifier(&format!("{instance}_{}", first.terminal_name));
    if inferred.is_empty() {
        format!("PORT{ordinal}")
    } else {
        inferred
    }
}

fn auto_net_name(source_name: &str) -> bool {
    source_name.strip_prefix("net").is_some_and(|suffix| {
        !suffix.is_empty() && suffix.chars().all(|character| character.is_ascii_digit())
    })
}

fn sanitize_identifier(raw: &str) -> String {
    let mut value = raw
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '_' || character == '$' {
                character
            } else {
                '_'
            }
        })
        .collect::<String>();
    if value
        .chars()
        .next()
        .is_some_and(|character| character.is_ascii_digit())
    {
        value.insert_str(0, "N_");
    }
    value.truncate(128);
    value
}

fn unique_port_name(base: String, reserved: &mut HashSet<String>) -> String {
    let base = if base.is_empty() {
        "PORT".to_owned()
    } else {
        base
    };
    if reserved.insert(base.to_ascii_lowercase()) {
        return base;
    }
    (2..)
        .map(|index| {
            let suffix = format!("_{index}");
            let budget = 128usize.saturating_sub(suffix.len());
            format!(
                "{}{}",
                base.chars().take(budget).collect::<String>(),
                suffix
            )
        })
        .find(|candidate| reserved.insert(candidate.to_ascii_lowercase()))
        .expect("the interface identifier namespace is unbounded")
}

fn snapped_selection_center(
    components: &[&Component],
    grid: i32,
) -> Result<Point, HierarchyExtractionError> {
    let min_x = components
        .iter()
        .map(|component| component.pos.x)
        .min()
        .unwrap_or(0);
    let max_x = components
        .iter()
        .map(|component| component.pos.x)
        .max()
        .unwrap_or(0);
    let min_y = components
        .iter()
        .map(|component| component.pos.y)
        .min()
        .unwrap_or(0);
    let max_y = components
        .iter()
        .map(|component| component.pos.y)
        .max()
        .unwrap_or(0);
    let x = min_x
        .checked_add(max_x)
        .ok_or(HierarchyExtractionError::CoordinateOverflow)?
        / 2;
    let y = min_y
        .checked_add(max_y)
        .ok_or(HierarchyExtractionError::CoordinateOverflow)?
        / 2;
    let pitch = grid.max(1);
    Ok(Point::new((x / pitch) * pitch, (y / pitch) * pitch))
}

fn retained_net_geometry(
    terminals: &[Point],
    segments: &[(Point, Point)],
    include_all: bool,
    external_terminal_points: &HashSet<Point>,
) -> Vec<Vec<Point>> {
    if segments.is_empty() {
        return Vec::new();
    }
    if include_all {
        return segments
            .iter()
            .filter_map(|(start, end)| (*start != *end).then_some(vec![*start, *end]))
            .collect();
    }

    // Preserve every authored segment in each selected physical island. A
    // named net can contain disconnected islands, so boundary/global nets do
    // not pull unrelated parent-only geometry into the child.
    let graph = split_segment_graph(segments, terminals);
    let mut queue = VecDeque::new();
    let mut retained_points = HashSet::new();
    for terminal in terminals {
        if graph.contains_key(terminal) && retained_points.insert(*terminal) {
            queue.push_back(*terminal);
        }
    }
    while let Some(point) = queue.pop_front() {
        for neighbor in graph.get(&point).into_iter().flatten() {
            // A boundary wire remains in the parent. Do not copy the edge
            // entering an unselected terminal into the child; the new
            // explicit port connects at the selected-side anchor instead.
            if external_terminal_points.contains(neighbor) {
                continue;
            }
            if retained_points.insert(*neighbor) {
                queue.push_back(*neighbor);
            }
        }
    }

    let graph_points = graph.keys().copied().collect::<Vec<_>>();
    let mut retained = Vec::new();
    for &(start, end) in segments {
        let mut split_points = graph_points
            .iter()
            .copied()
            .filter(|point| point_on_segment(*point, start, end))
            .collect::<Vec<_>>();
        if start.x == end.x {
            split_points.sort_by_key(|point| point.y);
        } else {
            split_points.sort_by_key(|point| point.x);
        }
        let kept = split_points
            .windows(2)
            .filter(|pair| retained_points.contains(&pair[0]) && retained_points.contains(&pair[1]))
            .map(|pair| (pair[0], pair[1]))
            .collect::<Vec<_>>();
        if kept.len() == split_points.len().saturating_sub(1) {
            retained.push(vec![start, end]);
        } else {
            retained.extend(
                kept.into_iter()
                    .filter(|(a, b)| a != b)
                    .map(|(a, b)| vec![a, b]),
            );
        }
    }
    retained
}

fn split_segment_graph(
    segments: &[(Point, Point)],
    terminals: &[Point],
) -> HashMap<Point, Vec<Point>> {
    let mut graph = HashMap::<Point, Vec<Point>>::new();
    let mut candidates = terminals.to_vec();
    candidates.extend(segments.iter().flat_map(|(a, b)| [*a, *b]));
    candidates.sort_by_key(|point| (point.x, point.y));
    candidates.dedup();
    for &(a, b) in segments {
        let mut points = candidates
            .iter()
            .copied()
            .filter(|point| point_on_segment(*point, a, b))
            .collect::<Vec<_>>();
        if a.x == b.x {
            points.sort_by_key(|point| point.y);
        } else {
            points.sort_by_key(|point| point.x);
        }
        for pair in points.windows(2) {
            if pair[0] != pair[1] {
                graph.entry(pair[0]).or_default().push(pair[1]);
                graph.entry(pair[1]).or_default().push(pair[0]);
            }
        }
    }
    graph
}

fn place_child_ports(
    child: &mut HierarchyDocument,
    plan: &HierarchyExtractionPlan,
) -> Result<(), HierarchyExtractionError> {
    if plan.ports.is_empty() {
        return Ok(());
    }
    let obstacles = child
        .document
        .components
        .iter()
        .map(|component| {
            let source = plan
                .source_component_bounds
                .get(&component.id)
                .copied()
                .unwrap_or_else(|| {
                    let translated = component.bounding_box();
                    (
                        translated.0.saturating_add(plan.origin.x),
                        translated.1.saturating_add(plan.origin.y),
                        translated.2.saturating_add(plan.origin.x),
                        translated.3.saturating_add(plan.origin.y),
                    )
                });
            Ok((
                component.id,
                (
                    source
                        .0
                        .checked_sub(plan.origin.x)
                        .ok_or(HierarchyExtractionError::CoordinateOverflow)?,
                    source
                        .1
                        .checked_sub(plan.origin.y)
                        .ok_or(HierarchyExtractionError::CoordinateOverflow)?,
                    source
                        .2
                        .checked_sub(plan.origin.x)
                        .ok_or(HierarchyExtractionError::CoordinateOverflow)?,
                    source
                        .3
                        .checked_sub(plan.origin.y)
                        .ok_or(HierarchyExtractionError::CoordinateOverflow)?,
                ),
            ))
        })
        .collect::<Result<Vec<_>, HierarchyExtractionError>>()?;
    let min_x = obstacles
        .iter()
        .map(|(_, bounds)| bounds.0)
        .min()
        .unwrap_or(-20);
    let max_x = obstacles
        .iter()
        .map(|(_, bounds)| bounds.2)
        .max()
        .unwrap_or(20);
    let min_y = obstacles
        .iter()
        .map(|(_, bounds)| bounds.1)
        .min()
        .unwrap_or(-20);
    let max_y = obstacles
        .iter()
        .map(|(_, bounds)| bounds.3)
        .max()
        .unwrap_or(20);
    let pitch = child.document.grid_size.max(10).saturating_mul(2);
    let mut port_routes = Vec::<Vec<Point>>::new();
    let mut left = 0i32;
    let mut right = 0i32;
    let mut rail = 0i32;
    for (index, port) in plan.ports.iter().enumerate() {
        let anchor = checked_sub(port.parent_anchor, plan.origin)?;
        let (terminal, component_pos, rotation) = match port.direction {
            PortDirection::In => {
                let y = exterior_slot(min_y, max_y, left, pitch);
                left = left.saturating_add(1);
                let terminal = Point::new(min_x.saturating_sub(40), y);
                (
                    terminal,
                    Point::new(terminal.x.saturating_add(10), terminal.y),
                    Rotation::R0,
                )
            }
            PortDirection::Out | PortDirection::InOut => {
                let y = exterior_slot(min_y, max_y, right, pitch);
                right = right.saturating_add(1);
                let terminal = Point::new(max_x.saturating_add(40), y);
                (
                    terminal,
                    Point::new(terminal.x.saturating_sub(10), terminal.y),
                    Rotation::R180,
                )
            }
            PortDirection::Supply => {
                let top = rail % 2 == 0;
                let distance = rail / 2 + 1;
                let offset = distance.saturating_mul(pitch);
                let x = if top {
                    min_x.saturating_sub(offset)
                } else {
                    max_x.saturating_add(offset)
                };
                rail = rail.saturating_add(1);
                if top {
                    let terminal = Point::new(x, min_y.saturating_sub(40));
                    (
                        terminal,
                        Point::new(terminal.x, terminal.y.saturating_add(10)),
                        Rotation::R90,
                    )
                } else {
                    let terminal = Point::new(x, max_y.saturating_add(40));
                    (
                        terminal,
                        Point::new(terminal.x, terminal.y.saturating_sub(10)),
                        Rotation::R270,
                    )
                }
            }
        };
        let id = child.add_component(ComponentType::Port, component_pos);
        let component = child
            .document
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("new child port exists");
        component.rotation = rotation;
        component.value.clone_from(&port.name);
        component.params = PortContract {
            direction: port.direction,
            signal_type: if port.direction == PortDirection::Supply {
                PortSignalType::Power
            } else if port.discipline == PortDiscipline::Logic {
                PortSignalType::Logic
            } else {
                PortSignalType::Analog
            },
            discipline: port.discipline,
            netlist_order: Some(index + 1),
            documentation: format!(
                "{} {} {} extracted hierarchy interface port",
                port.name,
                port.direction.keyword(),
                port.discipline.keyword()
            ),
        }
        .encoded_params();
        let anchor_component_ids = plan
            .source_terminals
            .iter()
            .filter(|source| {
                source.point == port.parent_anchor
                    && plan.source_component_ids.contains(&source.component_id)
            })
            .map(|source| source.component_id)
            .collect::<HashSet<_>>();
        let route = (0..32).find_map(|attempt| {
            let lane = index.saturating_add(attempt * plan.ports.len().max(1));
            let route = orthogonal_path(terminal, anchor, lane, child.document.grid_size.max(1));
            child_route_is_safe(
                &route,
                anchor,
                &port.source_net,
                plan,
                &obstacles,
                &anchor_component_ids,
                &port_routes,
            )
            .then_some(route)
        });
        let Some(route) = route else {
            return Err(HierarchyExtractionError::NoPlacement);
        };
        child.add_wire(route.clone());
        port_routes.push(route);
    }
    Ok(())
}

fn exterior_slot(minimum: i32, maximum: i32, index: i32, pitch: i32) -> i32 {
    let distance = (index / 2 + 1).saturating_mul(pitch);
    if index % 2 == 0 {
        minimum.saturating_sub(distance)
    } else {
        maximum.saturating_add(distance)
    }
}

fn child_route_is_safe(
    route: &[Point],
    anchor: Point,
    source_net: &str,
    plan: &HierarchyExtractionPlan,
    obstacles: &[ComponentObstacle],
    anchor_component_ids: &HashSet<u64>,
    other_routes: &[Vec<Point>],
) -> bool {
    for segment in route.windows(2).map(|pair| (pair[0], pair[1])) {
        if obstacles.iter().any(|(component_id, rect)| {
            let leaves_anchor_component = anchor_component_ids.contains(component_id)
                && (segment.0 == anchor || segment.1 == anchor);
            segment_intersects_rect(segment, *rect)
                && !leaves_anchor_component
                && !segment_touches_rect_only_at_anchor(segment, anchor, *rect)
        }) {
            return false;
        }
        for net in plan.nets.iter().filter(|net| net.source_name != source_net) {
            let intersects_foreign = net.paths.iter().any(|path| {
                path.windows(2).any(|pair| {
                    let Ok(start) = checked_sub(pair[0], plan.origin) else {
                        return true;
                    };
                    let Ok(end) = checked_sub(pair[1], plan.origin) else {
                        return true;
                    };
                    segments_intersect(segment, (start, end))
                })
            });
            if intersects_foreign {
                return false;
            }
        }
        if other_routes.iter().any(|other| {
            other
                .windows(2)
                .map(|pair| (pair[0], pair[1]))
                .any(|existing| segments_intersect(segment, existing))
        }) {
            return false;
        }
    }
    true
}

fn segment_touches_rect_only_at_anchor(
    segment: (Point, Point),
    anchor: Point,
    rect: (i32, i32, i32, i32),
) -> bool {
    if segment.0 != anchor && segment.1 != anchor {
        return false;
    }
    let other = if segment.0 == anchor {
        segment.1
    } else {
        segment.0
    };
    (anchor.x == rect.0 && other.x <= rect.0 && other.y == anchor.y)
        || (anchor.x == rect.2 && other.x >= rect.2 && other.y == anchor.y)
        || (anchor.y == rect.1 && other.y <= rect.1 && other.x == anchor.x)
        || (anchor.y == rect.3 && other.y >= rect.3 && other.x == anchor.x)
}

fn hierarchy_instance_position(
    parent: &HierarchyDocument,
    plan: &HierarchyExtractionPlan,
    binding: &LibraryCellInstance,
) -> Result<(Point, Vec<Vec<Point>>), HierarchyExtractionError> {
    let step = parent.document.grid_size.max(10).saturating_mul(4);
    let candidates = std::iter::once(Point::origin()).chain((1i32..=8).flat_map(|ring| {
        let radius = ring.saturating_mul(step);
        [
            Point::new(radius, 0),
            Point::new(-radius, 0),
            Point::new(0, radius),
            Point::new(0, -radius),
            Point::new(radius, radius),
            Point::new(-radius, radius),
            Point::new(radius, -radius),
            Point::new(-radius, -radius),
        ]
    }));
    let obstacles = parent
        .document
        .components
        .iter()
        .map(|component| {
            plan.source_component_bounds
                .get(&component.id)
                .copied()
                .unwrap_or_else(|| component.bounding_box())
        })
        .collect::<Vec<_>>();
    for delta in candidates {
        let Some(position) = checked_add(plan.origin, delta) else {
            continue;
        };
        let candidate = Component::new(u64::MAX, ComponentType::CellInstance, position)
            .with_library_cell(binding.clone());
        let bounds = candidate.bounding_box();
        if obstacles
            .iter()
            .any(|obstacle| rectangles_overlap(bounds, *obstacle))
        {
            continue;
        }
        let crosses_retained_wire = plan.source_segments_by_net.iter().any(|(net, source)| {
            if !net_retained_in_parent(plan, net) {
                return false;
            }
            let segments = plan
                .nets
                .iter()
                .find(|planned| planned.source_name == *net)
                .map_or(source.as_slice(), |planned| {
                    planned.parent_segments.as_slice()
                });
            segments
                .iter()
                .copied()
                .any(|segment| segment_intersects_rect(segment, bounds))
        });
        if crosses_retained_wire
            || plan
                .source_bus_segments
                .iter()
                .copied()
                .any(|segment| segment_intersects_rect(segment, bounds))
        {
            continue;
        }
        let terminals = candidate
            .terminal_positions()
            .into_iter()
            .map(|(_, point)| point)
            .collect::<Vec<_>>();
        if terminals.len() != plan.ports.len() {
            continue;
        }
        let mut routes = Vec::<Vec<Point>>::new();
        let mut all_routed = true;
        for (index, (port, terminal)) in plan.ports.iter().zip(terminals).enumerate() {
            let route = (0..16).find_map(|attempt| {
                let lane = index.saturating_add(attempt * plan.ports.len().max(1));
                let route = orthogonal_path(
                    terminal,
                    port.parent_anchor,
                    lane,
                    parent.document.grid_size.max(1),
                );
                route_is_safe(
                    &route,
                    port.parent_anchor,
                    &port.source_net,
                    plan,
                    &obstacles,
                    &routes,
                )
                .then_some(route)
            });
            let Some(route) = route else {
                all_routed = false;
                break;
            };
            routes.push(route);
        }
        if all_routed {
            return Ok((position, routes));
        }
    }
    Err(HierarchyExtractionError::NoPlacement)
}

fn route_is_safe(
    route: &[Point],
    anchor: Point,
    source_net: &str,
    plan: &HierarchyExtractionPlan,
    obstacles: &[(i32, i32, i32, i32)],
    other_routes: &[Vec<Point>],
) -> bool {
    let route_segments = route.windows(2).map(|pair| (pair[0], pair[1]));
    for segment in route_segments {
        if obstacles.iter().any(|obstacle| {
            segment_intersects_rect(segment, *obstacle)
                && !segment_touches_rect_only_at_anchor(segment, anchor, *obstacle)
        }) {
            return false;
        }
        if plan
            .source_bus_segments
            .iter()
            .copied()
            .any(|bus| segments_intersect(segment, bus))
        {
            return false;
        }
        for (net, source_segments) in &plan.source_segments_by_net {
            if net == source_net || !net_retained_in_parent(plan, net) {
                continue;
            }
            let segments = plan
                .nets
                .iter()
                .find(|planned| planned.source_name == *net)
                .map_or(source_segments.as_slice(), |planned| {
                    planned.parent_segments.as_slice()
                });
            if segments
                .iter()
                .copied()
                .any(|existing| segments_intersect(segment, existing))
            {
                return false;
            }
        }
        if other_routes.iter().any(|other| {
            other
                .windows(2)
                .map(|pair| (pair[0], pair[1]))
                .any(|existing| segments_intersect(segment, existing))
        }) {
            return false;
        }
    }
    true
}

fn net_retained_in_parent(plan: &HierarchyExtractionPlan, source_net: &str) -> bool {
    plan.nets
        .iter()
        .find(|net| net.source_name == source_net)
        .is_none_or(|net| net.boundary || net.global_ground)
}

fn orthogonal_path(start: Point, end: Point, lane: usize, grid: i32) -> Vec<Point> {
    if start.x == end.x || start.y == end.y {
        return vec![start, end];
    }
    let horizontal_first = lane.is_multiple_of(2);
    let corner = if horizontal_first {
        Point::new(end.x, start.y)
    } else {
        Point::new(start.x, end.y)
    };
    let mut route = vec![start, corner, end];
    if lane >= 2 {
        let offset = i32::try_from(lane / 2)
            .unwrap_or(i32::MAX)
            .saturating_mul(grid);
        route = if horizontal_first {
            let x = corner.x.saturating_add(offset);
            vec![start, Point::new(x, start.y), Point::new(x, end.y), end]
        } else {
            let y = corner.y.saturating_add(offset);
            vec![start, Point::new(start.x, y), Point::new(end.x, y), end]
        };
    }
    simplify_path(route)
}

fn simplify_path(points: Vec<Point>) -> Vec<Point> {
    let mut result = Vec::with_capacity(points.len());
    for point in points {
        if result.last() == Some(&point) {
            continue;
        }
        while result.len() >= 2 {
            let a = result[result.len() - 2];
            let b = result[result.len() - 1];
            if (a.x == b.x && b.x == point.x) || (a.y == b.y && b.y == point.y) {
                result.pop();
            } else {
                break;
            }
        }
        result.push(point);
    }
    result
}

fn checked_sub(point: Point, origin: Point) -> Result<Point, HierarchyExtractionError> {
    Ok(Point::new(
        point
            .x
            .checked_sub(origin.x)
            .ok_or(HierarchyExtractionError::CoordinateOverflow)?,
        point
            .y
            .checked_sub(origin.y)
            .ok_or(HierarchyExtractionError::CoordinateOverflow)?,
    ))
}

fn checked_add(point: Point, delta: Point) -> Option<Point> {
    Some(Point::new(
        point.x.checked_add(delta.x)?,
        point.y.checked_add(delta.y)?,
    ))
}

fn point_on_segment(point: Point, a: Point, b: Point) -> bool {
    if a.x == b.x {
        point.x == a.x && point.y >= a.y.min(b.y) && point.y <= a.y.max(b.y)
    } else if a.y == b.y {
        point.y == a.y && point.x >= a.x.min(b.x) && point.x <= a.x.max(b.x)
    } else {
        point == a || point == b
    }
}

fn rectangles_overlap(a: (i32, i32, i32, i32), b: (i32, i32, i32, i32)) -> bool {
    a.0 <= b.2 && a.2 >= b.0 && a.1 <= b.3 && a.3 >= b.1
}

fn segment_intersects_rect(segment: (Point, Point), rect: (i32, i32, i32, i32)) -> bool {
    let inside = |point: Point| {
        point.x >= rect.0 && point.x <= rect.2 && point.y >= rect.1 && point.y <= rect.3
    };
    if inside(segment.0) || inside(segment.1) {
        return true;
    }
    let top_left = Point::new(rect.0, rect.1);
    let top_right = Point::new(rect.2, rect.1);
    let bottom_right = Point::new(rect.2, rect.3);
    let bottom_left = Point::new(rect.0, rect.3);
    [
        (top_left, top_right),
        (top_right, bottom_right),
        (bottom_right, bottom_left),
        (bottom_left, top_left),
    ]
    .into_iter()
    .any(|edge| segments_intersect(segment, edge))
}

fn segments_intersect(a: (Point, Point), b: (Point, Point)) -> bool {
    fn orientation(a: Point, b: Point, c: Point) -> i64 {
        (i64::from(b.y) - i64::from(a.y)) * (i64::from(c.x) - i64::from(b.x))
            - (i64::from(b.x) - i64::from(a.x)) * (i64::from(c.y) - i64::from(b.y))
    }
    fn on_segment(point: Point, segment: (Point, Point)) -> bool {
        point.x >= segment.0.x.min(segment.1.x)
            && point.x <= segment.0.x.max(segment.1.x)
            && point.y >= segment.0.y.min(segment.1.y)
            && point.y <= segment.0.y.max(segment.1.y)
    }

    let o1 = orientation(a.0, a.1, b.0);
    let o2 = orientation(a.0, a.1, b.1);
    let o3 = orientation(b.0, b.1, a.0);
    let o4 = orientation(b.0, b.1, a.1);
    if ((o1 > 0 && o2 < 0) || (o1 < 0 && o2 > 0)) && ((o3 > 0 && o4 < 0) || (o3 < 0 && o4 > 0)) {
        return true;
    }
    (o1 == 0 && on_segment(b.0, a))
        || (o2 == 0 && on_segment(b.1, a))
        || (o3 == 0 && on_segment(a.0, b))
        || (o4 == 0 && on_segment(a.1, b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schematic::history::SchematicSnapshot;

    #[test]
    fn hierarchy_candidates_are_headless_and_leave_source_and_allocator_unchanged() {
        let mut document = SchematicDocument::default();
        let mut component = Component::new(1, ComponentType::Resistor, Point::origin());
        component.name = "R1".to_owned();
        document.components.push(component);
        let mut identity = SchematicIdentity::with_cursor(2);
        identity.recalculate(&mut document);
        let selected = HashSet::from([1]);
        let source = HierarchySource {
            document: &document,
            identity: &identity,
            topology_version: 7,
            selected_components: &selected,
            selected_count: 1,
        };
        let terminals = document.components[0]
            .terminal_positions()
            .into_iter()
            .map(|(name, point)| HierarchyExtractionTerminal {
                component_id: 1,
                terminal_name: name.to_owned(),
                point,
                direction: None,
                discipline: PortDiscipline::Electrical,
            })
            .collect::<Vec<_>>();
        let connectivity = HierarchyNetConnectivity {
            point_to_net: terminals
                .iter()
                .enumerate()
                .map(|(index, terminal)| (terminal.point, format!("internal_{index}")))
                .collect(),
            net_segments: HashMap::new(),
        };
        let before = SchematicSnapshot::capture(&document);
        let cursor = identity.cursor();
        let plan = source
            .plan_hierarchy_extraction(&terminals, &connectivity, &HashMap::new())
            .unwrap();
        let candidate = source
            .materialize_hierarchy_extraction(
                &plan,
                "work",
                "child",
                "schematic",
                Rotation::R90,
                true,
            )
            .unwrap();
        assert!(before.is_equal_document(&document));
        assert_eq!(identity.cursor(), cursor);
        assert_eq!(candidate.child.document.components.len(), 1);
        assert_eq!(candidate.child.document.net_labels.len(), 2);
        assert!(candidate.child.document.wires.is_empty());
        assert_eq!(candidate.child.topology_changes, 2);
        assert_eq!(candidate.parent.topology_changes, 2);
        let instance = &candidate.parent.document.components[0];
        assert_eq!(instance.id, cursor);
        assert_eq!(instance.rotation, Rotation::R0);
        assert!(instance.mirror_h);
        let stale = HierarchySource {
            topology_version: 8,
            ..source
        };
        assert!(matches!(
            stale.materialize_hierarchy_extraction(
                &plan,
                "work",
                "child",
                "schematic",
                Rotation::R0,
                false
            ),
            Err(HierarchyExtractionError::InvalidConnectivity(_))
        ));
        assert!(before.is_equal_document(&document));
        assert_eq!(identity.cursor(), cursor);
    }
}
