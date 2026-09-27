//! Validated component replacement and shared terminal routing.

use super::document::SchematicDocument;
use super::replacement::*;
use super::replacement::{
    format_replacement_parameters, parse_replacement_parameters_strict,
    valid_replacement_parameter_name,
};
use super::{
    component::Component, component_type::ComponentType, rotation::Rotation, wire::WireConnection,
};
use rspice_design_model::{Point, port::PortDirection};
use std::collections::{BTreeMap, HashMap, HashSet};

/// Current editor selection and topology counter used to check retained review authority.
#[derive(Debug, Clone, Copy)]
pub struct ReplacementContext {
    pub selected_component: Option<u64>,
    pub topology_version: u64,
}

#[derive(Debug)]
pub(super) struct TerminalPlacement<'a> {
    pub(super) terminal: &'a SchematicReplacementTerminal,
    pub(super) world: Point,
}

/// Capture immutable authority with caller-resolved authored terminals and
/// source parameter schema. No library-manager reference escapes into the
/// transaction.
pub fn replacement_authority_with_spec(
    document: &SchematicDocument,
    context: ReplacementContext,
    source_spec: SchematicReplacementSourceSpec,
) -> Result<SchematicReplacementAuthority, SchematicReplacementError> {
    let component_id = context
        .selected_component
        .ok_or(SchematicReplacementError::SelectExactlyOneInstance)?;
    let source_component = document
        .components
        .iter()
        .find(|component| component.id == component_id)
        .cloned()
        .ok_or(SchematicReplacementError::SourceInstanceMissing { component_id })?;
    validate_source_spec(&source_spec)?;
    if source_component.kind != ComponentType::CellInstance {
        validate_primitive_terminals(source_component.kind, source_spec.terminals(), true)?;
    }
    terminal_placements(&source_component, source_spec.terminals())?;
    Ok(SchematicReplacementAuthority {
        component_id,
        topology_version: context.topology_version,
        source_component,
        source_spec,
    })
}

/// Rebuild compatibility evidence against the current document and editor context.
pub fn preview_instance_replacement(
    document: &SchematicDocument,
    context: ReplacementContext,
    authority: &SchematicReplacementAuthority,
    target: &SchematicReplacementTargetSpec,
) -> Result<SchematicReplacementPreview, SchematicReplacementError> {
    build_instance_replacement(document, context, authority, target)
}

/// A validated replacement bound to the exclusively borrowed target document.
#[derive(Debug)]
pub struct InstanceReplacement<'a> {
    document: &'a mut SchematicDocument,
    impact: SchematicReplacementImpact,
    replacement: Component,
    replacement_counter: Option<(&'static str, u32)>,
    wire_edits: Vec<SchematicReplacementWireEdit>,
    affected_connections: Vec<WireConnection>,
}

impl<'a> InstanceReplacement<'a> {
    pub fn prepare(
        document: &'a mut SchematicDocument,
        context: ReplacementContext,
        authority: &SchematicReplacementAuthority,
        target: &SchematicReplacementTargetSpec,
    ) -> Result<Self, SchematicReplacementError> {
        let preview = build_instance_replacement(document, context, authority, target)?;
        let impact = preview.impact;
        if !impact.topology_changed {
            return Err(SchematicReplacementError::NoChanges);
        }
        let replacement = preview.component;
        let replacement_counter = replacement
            .name
            .strip_prefix(replacement.kind.spice_prefix())
            .and_then(|suffix| suffix.parse::<u32>().ok())
            .map(|number| (replacement.kind.spice_prefix(), number));
        let wire_edits = preview.wire_edits;
        let affected_connections = preview.connections;

        Ok(Self {
            document,
            impact,
            replacement,
            replacement_counter,
            wire_edits,
            affected_connections,
        })
    }

    pub fn document(&self) -> &SchematicDocument {
        self.document
    }
    pub fn impact(&self) -> SchematicReplacementImpact {
        self.impact
    }
    pub fn reference_counter(&self) -> Option<(&'static str, u32)> {
        self.replacement_counter
    }

    pub fn commit(self) {
        let Self {
            document: state,
            impact,
            replacement,
            wire_edits,
            affected_connections,
            ..
        } = self;
        let component_id = impact.component_id;
        if let Some(component) = state
            .components
            .iter_mut()
            .find(|component| component.id == component_id)
        {
            *component = replacement;
        }
        for replacement_connection in affected_connections {
            if let Some(connection) = state.connections.iter_mut().find(|connection| {
                connection.component_id == component_id
                    && connection.wire_id == replacement_connection.wire_id
                    && connection.point_index == replacement_connection.point_index
            }) {
                connection.terminal_name = replacement_connection.terminal_name;
            }
        }
        for edit in wire_edits {
            if let Some(wire) = state.wires.iter_mut().find(|wire| wire.id == edit.wire_id) {
                wire.points = edit.replacement_points;
            }
            for connection in state
                .connections
                .iter_mut()
                .filter(|connection| connection.wire_id == edit.wire_id)
            {
                if let Some(replacement_index) = edit.point_indices.get(connection.point_index) {
                    connection.point_index = *replacement_index;
                }
            }
        }
    }
}

fn build_instance_replacement(
    document: &SchematicDocument,
    context: ReplacementContext,
    authority: &SchematicReplacementAuthority,
    target: &SchematicReplacementTargetSpec,
) -> Result<SchematicReplacementPreview, SchematicReplacementError> {
    validate_authority(document, context, authority)?;
    validate_target_spec(target)?;

    let source = &authority.source_component;
    let source_terminals = terminal_placements(source, authority.source_spec.terminals())?;

    let mut replacement = source.clone();
    replacement.kind = target.kind();
    replacement.name = replacement_reference_name(document, source, target.kind())?;
    replacement.value = match target.value_policy() {
        SchematicReplacementValuePolicy::UseTarget => target.value().to_owned(),
        SchematicReplacementValuePolicy::PreserveSource => source.value.clone(),
    };
    replacement.symbol_variant = target.symbol_variant().map(str::to_owned);
    replacement.library_cell = target.library_binding().cloned();

    let target_terminals = terminal_placements(&replacement, target.terminals())?;
    let target_lookup = terminal_lookup(target.terminals())?;
    let connected_source_names = connected_source_terminals(document, source, &source_terminals)?;

    let mut terminal_mappings = Vec::with_capacity(source_terminals.len());
    let mut terminal_map = HashMap::<String, &TerminalPlacement<'_>>::new();
    let mut used_targets = HashSet::new();
    for source_terminal in &source_terminals {
        let source_key = normalized(source_terminal.terminal.name());
        let connected = connected_source_names.contains(&source_key);
        let mapped = target_lookup.get(&source_key).copied();
        let (target_terminal, status, direction_compatible) = match mapped {
            Some((target_index, status)) => {
                if !used_targets.insert(target_index) {
                    let name = target
                        .terminals()
                        .get(target_index)
                        .map_or("target terminal", SchematicReplacementTerminal::name)
                        .to_owned();
                    return Err(SchematicReplacementError::AmbiguousTerminalAlias { name });
                }
                let target_terminal = target_terminals.get(target_index).ok_or_else(|| {
                    SchematicReplacementError::InvalidTargetContract {
                        reason: "terminal placement index is inconsistent".to_owned(),
                    }
                })?;
                let direction_compatible = directions_compatible(
                    source_terminal.terminal.direction(),
                    target_terminal.terminal.direction(),
                );
                terminal_map.insert(source_key, target_terminal);
                (
                    Some(target_terminal.terminal.name().to_owned()),
                    status,
                    direction_compatible,
                )
            }
            None if connected => {
                return Err(SchematicReplacementError::UnmappedConnectedTerminal {
                    terminal: source_terminal.terminal.name().to_owned(),
                });
            }
            None => (None, SchematicReplacementMappingStatus::Unmapped, true),
        };
        terminal_mappings.push(SchematicReplacementTerminalMapping {
            source: source_terminal.terminal.name().to_owned(),
            target: target_terminal,
            status,
            connected,
            direction_compatible,
        });
    }
    validate_target_terminal_contacts(
        document,
        source,
        &source_terminals,
        &target_terminals,
        &terminal_map,
    )?;

    let source_params = parse_replacement_parameters_strict(&source.params)?;
    let target_defaults = parse_replacement_parameters_strict(target.default_params())?;
    let parameter_result = map_parameters(
        &authority.source_spec,
        &source_params,
        target.parameters(),
        target_defaults,
    )?;
    replacement.params = format_replacement_parameters(&parameter_result.values);

    let wire_result = build_wire_edits(document, source, &source_terminals, &terminal_map)?;
    validate_replacement_geometry(document, source, &replacement)?;

    let model_status = model_status(source, &replacement);
    let netlist_status = netlist_status(
        source,
        &replacement,
        authority.source_spec.terminals(),
        target,
    );
    let mapped_terminal_count = terminal_mappings
        .iter()
        .filter(|mapping| mapping.target.is_some())
        .count();
    let compatibility = SchematicReplacementCompatibility {
        component_id: source.id,
        source_terminal_count: source_terminals.len(),
        target_terminal_count: target_terminals.len(),
        mapped_terminal_count,
        connected_terminal_count: terminal_mappings
            .iter()
            .filter(|mapping| mapping.connected)
            .count(),
        source_parameter_count: parameter_result.source_count,
        target_parameter_count: target.parameters().len(),
        mapped_parameter_count: parameter_result.mapped_count,
        terminal_mappings,
        parameter_mappings: parameter_result.mappings,
        model_status,
        netlist_status,
    };
    let connections_changed = wire_result.connections.iter().any(|candidate| {
        document.connections.iter().find(|connection| {
            connection.component_id == source.id
                && connection.wire_id == candidate.wire_id
                && connection.point_index == candidate.point_index
        }) != Some(candidate)
    });
    let impact = SchematicReplacementImpact {
        component_id: source.id,
        preserved_connections: wire_result.connections.len(),
        relocated_wire_points: wire_result.relocated_points,
        preserved_parameters: parameter_result.preserved_value_count,
        dropped_parameters: parameter_result.dropped_value_count,
        topology_changed: replacement != *source
            || !wire_result.edits.is_empty()
            || connections_changed,
    };
    Ok(SchematicReplacementPreview {
        component: replacement,
        connections: wire_result.connections,
        wire_edits: wire_result.edits,
        compatibility,
        impact,
    })
}

fn replacement_reference_name(
    state: &SchematicDocument,
    source: &Component,
    target_kind: ComponentType,
) -> Result<String, SchematicReplacementError> {
    let source_prefix = source.kind.spice_prefix();
    let target_prefix = target_kind.spice_prefix();
    if target_prefix.is_empty() {
        return Err(SchematicReplacementError::InvalidTargetContract {
            reason: "the target does not own a SPICE reference-designator prefix".to_owned(),
        });
    }
    if source_prefix.eq_ignore_ascii_case(target_prefix) {
        return Ok(source.name.clone());
    }
    (1..=u32::MAX)
        .map(|index| format!("{target_prefix}{index}"))
        .find(|candidate| {
            state.components.iter().all(|component| {
                component.id == source.id || !component.name.eq_ignore_ascii_case(candidate)
            })
        })
        .ok_or_else(|| SchematicReplacementError::InvalidTargetContract {
            reason: format!("no unique `{target_prefix}` reference designator remains"),
        })
}

struct ParameterMapResult {
    values: HashMap<String, String>,
    mappings: Vec<SchematicReplacementParameterMapping>,
    source_count: usize,
    mapped_count: usize,
    preserved_value_count: usize,
    dropped_value_count: usize,
}

fn map_parameters(
    source_spec: &SchematicReplacementSourceSpec,
    source_values: &HashMap<String, String>,
    target_parameters: &[SchematicReplacementParameter],
    target_values: HashMap<String, String>,
) -> Result<ParameterMapResult, SchematicReplacementError> {
    let target_lookup = parameter_lookup(target_parameters)?;
    let mut canonical_target_values = HashMap::with_capacity(target_values.len());
    for (key, value) in target_values {
        let Some((target_index, _)) = target_lookup.get(&normalized(&key)).copied() else {
            return Err(SchematicReplacementError::InvalidTargetContract {
                reason: format!("default parameter '{key}' is not declared by the target"),
            });
        };
        let target_parameter = target_parameters.get(target_index).ok_or_else(|| {
            SchematicReplacementError::InvalidTargetContract {
                reason: "parameter lookup index is inconsistent".to_owned(),
            }
        })?;
        let canonical = normalized(target_parameter.name());
        if canonical_target_values.insert(canonical, value).is_some() {
            return Err(SchematicReplacementError::InvalidTargetContract {
                reason: format!(
                    "more than one default resolves to parameter '{}'",
                    target_parameter.name()
                ),
            });
        }
    }
    let mut target_values = canonical_target_values;
    for parameter in target_parameters {
        if let Some(default) = parameter.default_value() {
            target_values
                .entry(normalized(parameter.name()))
                .or_insert_with(|| default.to_owned());
        }
    }

    let mut source_keys: HashSet<String> = source_spec
        .parameter_keys()
        .iter()
        .map(|key| normalized(key))
        .collect();
    source_keys.extend(source_values.keys().map(|key| normalized(key)));
    let mut source_keys: Vec<_> = source_keys.into_iter().collect();
    source_keys.sort();

    let mut mappings = Vec::with_capacity(source_keys.len());
    let mut used_targets = HashMap::<usize, bool>::new();
    let mut mapped_count = 0;
    let mut preserved_value_count = 0;
    let mut dropped_value_count = 0;
    for source_key in &source_keys {
        let authored_value = source_values.get(source_key);
        match target_lookup.get(source_key).copied() {
            Some((target_index, status)) => {
                let target_parameter = target_parameters.get(target_index).ok_or_else(|| {
                    SchematicReplacementError::InvalidTargetContract {
                        reason: "parameter mapping index is inconsistent".to_owned(),
                    }
                })?;
                let already_has_authored_value = used_targets.get(&target_index).copied();
                if already_has_authored_value == Some(true) && authored_value.is_some() {
                    return Err(SchematicReplacementError::AmbiguousParameterAlias {
                        name: target_parameter.name().to_owned(),
                    });
                }
                if already_has_authored_value.is_none() {
                    mapped_count += 1;
                }
                used_targets
                    .entry(target_index)
                    .and_modify(|has_value| *has_value |= authored_value.is_some())
                    .or_insert_with(|| authored_value.is_some());
                let target_name = normalized(target_parameter.name());
                if let Some(value) = authored_value {
                    target_values.insert(target_name.clone(), value.clone());
                    preserved_value_count += 1;
                }
                mappings.push(SchematicReplacementParameterMapping {
                    source: source_key.clone(),
                    target: Some(target_parameter.name().to_owned()),
                    status,
                    has_authored_value: authored_value.is_some(),
                });
            }
            None => {
                dropped_value_count += usize::from(authored_value.is_some());
                mappings.push(SchematicReplacementParameterMapping {
                    source: source_key.clone(),
                    target: None,
                    status: SchematicReplacementMappingStatus::Unmapped,
                    has_authored_value: authored_value.is_some(),
                });
            }
        }
    }

    for parameter in target_parameters {
        let key = normalized(parameter.name());
        if parameter.is_required() && target_values.get(&key).is_none_or(|value| value.is_empty()) {
            return Err(SchematicReplacementError::MissingRequiredParameter {
                parameter: parameter.name().to_owned(),
            });
        }
    }
    Ok(ParameterMapResult {
        values: target_values,
        mappings,
        source_count: source_keys.len(),
        mapped_count,
        preserved_value_count,
        dropped_value_count,
    })
}

pub(super) struct WireBuildResult {
    pub(super) edits: Vec<SchematicReplacementWireEdit>,
    pub(super) connections: Vec<WireConnection>,
    relocated_points: usize,
}

pub(super) fn build_wire_edits(
    state: &SchematicDocument,
    source: &Component,
    source_terminals: &[TerminalPlacement<'_>],
    terminal_map: &HashMap<String, &TerminalPlacement<'_>>,
) -> Result<WireBuildResult, SchematicReplacementError> {
    let source_positions: HashMap<_, _> = source_terminals
        .iter()
        .map(|terminal| (normalized(terminal.terminal.name()), terminal.world))
        .collect();
    let mut endpoint_moves = BTreeMap::<u64, BTreeMap<usize, Point>>::new();
    let mut connections = Vec::new();
    for source_terminal in source_terminals {
        let source_key = normalized(source_terminal.terminal.name());
        let Some(target) = terminal_map.get(&source_key) else {
            continue;
        };
        if source_terminal.world == target.world {
            continue;
        }
        for wire in state
            .wires
            .iter()
            .filter(|wire| wire.contains_point(source_terminal.world))
        {
            let explicitly_owned = state.connections.iter().any(|connection| {
                connection.component_id == source.id
                    && connection.wire_id == wire.id
                    && normalized(&connection.terminal_name) == source_key
                    && wire
                        .points
                        .get(connection.point_index)
                        .is_some_and(|point| *point == source_terminal.world)
            });
            if !explicitly_owned {
                return Err(SchematicReplacementError::FixedElectricalAnchor {
                    point: source_terminal.world,
                });
            }
        }
    }
    for connection in state
        .connections
        .iter()
        .filter(|connection| connection.component_id == source.id)
    {
        let source_key = normalized(&connection.terminal_name);
        let old = source_positions.get(&source_key).copied().ok_or_else(|| {
            SchematicReplacementError::InvalidSourceContract {
                reason: format!(
                    "connection names undeclared terminal '{}'",
                    connection.terminal_name
                ),
            }
        })?;
        let target = terminal_map.get(&source_key).ok_or_else(|| {
            SchematicReplacementError::UnmappedConnectedTerminal {
                terminal: connection.terminal_name.clone(),
            }
        })?;
        let wire = state
            .wires
            .iter()
            .find(|wire| wire.id == connection.wire_id)
            .ok_or(SchematicReplacementError::StaleConnection {
                wire_id: connection.wire_id,
                point_index: connection.point_index,
            })?;
        let current = wire.points.get(connection.point_index).copied().ok_or(
            SchematicReplacementError::StaleConnection {
                wire_id: connection.wire_id,
                point_index: connection.point_index,
            },
        )?;
        if current != old {
            return Err(SchematicReplacementError::StaleConnection {
                wire_id: connection.wire_id,
                point_index: connection.point_index,
            });
        }
        if old != target.world {
            if !connection.is_endpoint(wire.points.len()) {
                return Err(SchematicReplacementError::UnsupportedInteriorConnection {
                    wire_id: connection.wire_id,
                    point_index: connection.point_index,
                });
            }
            if state.connections.iter().any(|other| {
                other.wire_id == connection.wire_id
                    && other.point_index == connection.point_index
                    && other.component_id != source.id
            }) {
                return Err(SchematicReplacementError::SharedWireAnchor {
                    wire_id: connection.wire_id,
                    point_index: connection.point_index,
                });
            }
            validate_movable_terminal_anchor(state, source.id, wire.id, old)?;
            let adjacent_index = if connection.point_index == 0 {
                1
            } else {
                wire.points.len().saturating_sub(2)
            };
            if wire
                .points
                .get(adjacent_index)
                .is_some_and(|point| *point == target.world)
            {
                return Err(SchematicReplacementError::DegenerateWire { wire_id: wire.id });
            }
            if endpoint_moves
                .entry(wire.id)
                .or_default()
                .insert(connection.point_index, target.world)
                .is_some_and(|existing| existing != target.world)
            {
                return Err(SchematicReplacementError::SharedWireAnchor {
                    wire_id: wire.id,
                    point_index: connection.point_index,
                });
            }
        }
        let mut updated = connection.clone();
        updated.terminal_name = target.terminal.name().to_owned();
        connections.push(updated);
    }
    let relocated_points = endpoint_moves.values().map(BTreeMap::len).sum();
    let mut edits = Vec::with_capacity(endpoint_moves.len());
    for (wire_id, moves) in endpoint_moves {
        let wire = state.wires.iter().find(|wire| wire.id == wire_id).ok_or(
            SchematicReplacementError::StaleConnection {
                wire_id,
                point_index: 0,
            },
        )?;
        let mut moved_points = wire.points.clone();
        for (point_index, target) in moves {
            let point = moved_points.get_mut(point_index).ok_or(
                SchematicReplacementError::StaleConnection {
                    wire_id,
                    point_index,
                },
            )?;
            *point = target;
        }
        let (replacement_points, point_indices) =
            super::movement::orthogonal_route_for_corresponding_points(
                wire_id,
                &wire.points,
                &moved_points,
            )
            .map_err(|_| SchematicReplacementError::OrthogonalRouteUnavailable { wire_id })?;
        if replacement_points.len() < 2
            || replacement_points
                .windows(2)
                .any(|points| points[0] == points[1])
        {
            return Err(SchematicReplacementError::DegenerateWire { wire_id });
        }
        let replacement_wire = super::wire::Wire::new(wire_id, replacement_points.clone());
        for tap in state.bus_taps.iter().filter(|tap| {
            tap.target_kind() == super::bus::BusTargetKind::Wire
                && wire.contains_point(tap.connection_point)
        }) {
            if !replacement_wire.contains_point(tap.connection_point) {
                return Err(SchematicReplacementError::FixedElectricalAnchor {
                    point: tap.connection_point,
                });
            }
        }
        edits.push(SchematicReplacementWireEdit {
            wire_id,
            original_points: wire.points.clone(),
            replacement_points,
            point_indices,
        });
    }
    connections.sort_by_key(|connection| (connection.wire_id, connection.point_index));
    Ok(WireBuildResult {
        edits,
        connections,
        relocated_points,
    })
}

fn validate_target_terminal_contacts(
    state: &SchematicDocument,
    source: &Component,
    source_terminals: &[TerminalPlacement<'_>],
    target_terminals: &[TerminalPlacement<'_>],
    terminal_map: &HashMap<String, &TerminalPlacement<'_>>,
) -> Result<(), SchematicReplacementError> {
    let retained_contacts: HashSet<Point> = source_terminals
        .iter()
        .filter_map(|source_terminal| {
            terminal_map
                .get(&normalized(source_terminal.terminal.name()))
                .filter(|target| target.world == source_terminal.world)
                .map(|target| target.world)
        })
        .collect();
    for target in target_terminals {
        if retained_contacts.contains(&target.world) {
            continue;
        }
        let mapped_source = source_terminals.iter().find(|source_terminal| {
            terminal_map
                .get(&normalized(source_terminal.terminal.name()))
                .is_some_and(|mapped| mapped.terminal.name() == target.terminal.name())
        });
        let unsafe_wire_contact = state
            .wires
            .iter()
            .filter(|wire| wire.contains_point(target.world))
            .any(|wire| {
                !mapped_source.is_some_and(|mapped_source| {
                    state.connections.iter().any(|connection| {
                        if connection.component_id != source.id
                            || connection.wire_id != wire.id
                            || !connection
                                .terminal_name
                                .eq_ignore_ascii_case(mapped_source.terminal.name())
                            || !connection.is_endpoint(wire.points.len())
                            || wire.points.get(connection.point_index) != Some(&mapped_source.world)
                        {
                            return false;
                        }
                        let adjacent_index = if connection.point_index == 0 {
                            1
                        } else {
                            wire.points.len().saturating_sub(2)
                        };
                        wire.points.get(adjacent_index).is_some_and(|adjacent| {
                            super::wire::WireSegment::new(mapped_source.world, *adjacent)
                                .contains_point(target.world)
                        })
                    })
                })
            });
        if unsafe_wire_contact
            || state
                .junctions
                .iter()
                .any(|junction| junction.pos == target.world)
            || state
                .net_labels
                .iter()
                .any(|label| label.pos == target.world)
            || state
                .bus_taps
                .iter()
                .any(|tap| tap.connection_point == target.world || tap.bus_point == target.world)
        {
            return Err(SchematicReplacementError::FixedElectricalAnchor {
                point: target.world,
            });
        }
        for other in state
            .components
            .iter()
            .filter(|component| component.id != source.id)
        {
            if other
                .terminal_positions()
                .into_iter()
                .any(|(_, point)| point == target.world)
            {
                return Err(SchematicReplacementError::GeometryCollision {
                    other_component_id: other.id,
                });
            }
        }
    }
    Ok(())
}

pub(super) fn connected_source_terminals(
    state: &SchematicDocument,
    source: &Component,
    terminals: &[TerminalPlacement<'_>],
) -> Result<HashSet<String>, SchematicReplacementError> {
    let by_name: HashMap<_, _> = terminals
        .iter()
        .map(|terminal| (normalized(terminal.terminal.name()), terminal.world))
        .collect();
    let mut connected = HashSet::new();
    for connection in state
        .connections
        .iter()
        .filter(|connection| connection.component_id == source.id)
    {
        let key = normalized(&connection.terminal_name);
        if !by_name.contains_key(&key) {
            return Err(SchematicReplacementError::InvalidSourceContract {
                reason: format!(
                    "connection names undeclared terminal '{}'",
                    connection.terminal_name
                ),
            });
        }
        connected.insert(key);
    }
    for terminal in terminals {
        if state
            .wires
            .iter()
            .any(|wire| wire.contains_point(terminal.world))
        {
            connected.insert(normalized(terminal.terminal.name()));
        }
    }
    Ok(connected)
}

fn validate_movable_terminal_anchor(
    state: &SchematicDocument,
    source_component_id: u64,
    owning_wire_id: u64,
    point: Point,
) -> Result<(), SchematicReplacementError> {
    if state.junctions.iter().any(|junction| junction.pos == point)
        || state.net_labels.iter().any(|label| label.pos == point)
        || state
            .bus_taps
            .iter()
            .any(|tap| tap.connection_point == point)
    {
        return Err(SchematicReplacementError::FixedElectricalAnchor { point });
    }
    for wire in state.wires.iter().filter(|wire| wire.id != owning_wire_id) {
        if !wire.contains_point(point) {
            continue;
        }
        let explicitly_owned = state.connections.iter().any(|connection| {
            connection.component_id == source_component_id
                && connection.wire_id == wire.id
                && wire
                    .points
                    .get(connection.point_index)
                    .is_some_and(|p| *p == point)
        });
        if !explicitly_owned {
            return Err(SchematicReplacementError::FixedElectricalAnchor { point });
        }
    }
    Ok(())
}

fn validate_authority(
    state: &SchematicDocument,
    context: ReplacementContext,
    authority: &SchematicReplacementAuthority,
) -> Result<(), SchematicReplacementError> {
    if context.selected_component != Some(authority.component_id)
        || context.topology_version != authority.topology_version
    {
        return Err(SchematicReplacementError::StaleAuthority);
    }
    let source = state
        .components
        .iter()
        .find(|component| component.id == authority.component_id)
        .ok_or(SchematicReplacementError::SourceInstanceMissing {
            component_id: authority.component_id,
        })?;
    if source != &authority.source_component {
        return Err(SchematicReplacementError::StaleAuthority);
    }
    Ok(())
}

fn validate_source_spec(
    source: &SchematicReplacementSourceSpec,
) -> Result<(), SchematicReplacementError> {
    validate_terminal_contract(source.terminals(), true)?;
    validate_parameter_keys(source.parameter_keys(), true)
}

fn validate_target_spec(
    target: &SchematicReplacementTargetSpec,
) -> Result<(), SchematicReplacementError> {
    match (target.kind(), target.library_binding()) {
        (ComponentType::CellInstance, None) => {
            return Err(SchematicReplacementError::InvalidTargetContract {
                reason: "a cell instance requires library/cell/view binding metadata".to_owned(),
            });
        }
        (ComponentType::CellInstance, Some(binding)) => {
            if binding.library.trim().is_empty()
                || binding.cell.trim().is_empty()
                || binding.view.trim().is_empty()
            {
                return Err(SchematicReplacementError::InvalidTargetContract {
                    reason: "library, cell, and view names must be non-empty".to_owned(),
                });
            }
            if target.terminals().is_empty() {
                return Err(SchematicReplacementError::InvalidTargetContract {
                    reason: "a cell instance must declare at least one terminal".to_owned(),
                });
            }
            if !binding.terminal_order.is_empty()
                && (binding.terminal_order.len() != target.terminals().len()
                    || binding
                        .terminal_order
                        .iter()
                        .zip(target.terminals())
                        .any(|(bound, terminal)| !bound.eq_ignore_ascii_case(terminal.name())))
            {
                return Err(SchematicReplacementError::InvalidTargetContract {
                    reason: "terminal contract does not match the binding's netlist order"
                        .to_owned(),
                });
            }
        }
        (_, Some(_)) => {
            return Err(SchematicReplacementError::InvalidTargetContract {
                reason: "primitive targets cannot carry library-cell binding metadata".to_owned(),
            });
        }
        (kind, None) => validate_primitive_terminals(kind, target.terminals(), false)?,
    }
    validate_terminal_contract(target.terminals(), false)?;
    validate_target_parameters(target.parameters())?;
    parse_replacement_parameters_strict(target.default_params())?;
    Ok(())
}

fn validate_primitive_terminals(
    kind: ComponentType,
    terminals: &[SchematicReplacementTerminal],
    source: bool,
) -> Result<(), SchematicReplacementError> {
    let expected = kind.terminal_offsets();
    let valid = expected.len() == terminals.len()
        && expected
            .iter()
            .zip(terminals)
            .all(|((name, offset), terminal)| {
                name.eq_ignore_ascii_case(terminal.name()) && *offset == terminal.offset()
            });
    if valid {
        return Ok(());
    }
    Err(if source {
        SchematicReplacementError::InvalidSourceContract {
            reason: format!(
                "{} terminals do not match its built-in netlist symbol",
                kind.display_name()
            ),
        }
    } else {
        SchematicReplacementError::InvalidTargetContract {
            reason: format!(
                "{} terminals do not match its built-in netlist symbol",
                kind.display_name()
            ),
        }
    })
}

fn validate_terminal_contract(
    terminals: &[SchematicReplacementTerminal],
    source: bool,
) -> Result<(), SchematicReplacementError> {
    let mut names = HashSet::new();
    for terminal in terminals {
        if terminal.name().trim().is_empty() {
            return Err(if source {
                SchematicReplacementError::InvalidSourceContract {
                    reason: "terminal names must be non-empty".to_owned(),
                }
            } else {
                SchematicReplacementError::InvalidTargetContract {
                    reason: "terminal names must be non-empty".to_owned(),
                }
            });
        }
        let key = normalized(terminal.name());
        if !names.insert(key.clone()) {
            return Err(SchematicReplacementError::DuplicateTerminalName { name: key });
        }
        for alias in terminal.aliases() {
            if alias.trim().is_empty() {
                return Err(SchematicReplacementError::InvalidTargetContract {
                    reason: "terminal aliases must be non-empty".to_owned(),
                });
            }
        }
    }
    Ok(())
}

fn validate_parameter_keys(keys: &[String], source: bool) -> Result<(), SchematicReplacementError> {
    let mut names = HashSet::new();
    for name in keys {
        if !valid_replacement_parameter_name(name) {
            return Err(if source {
                SchematicReplacementError::InvalidSourceContract {
                    reason: format!("'{name}' is not a valid parameter name"),
                }
            } else {
                SchematicReplacementError::InvalidTargetContract {
                    reason: format!("'{name}' is not a valid parameter name"),
                }
            });
        }
        let key = normalized(name);
        if !names.insert(key.clone()) {
            return Err(SchematicReplacementError::DuplicateParameterName { name: key });
        }
    }
    Ok(())
}

fn validate_target_parameters(
    parameters: &[SchematicReplacementParameter],
) -> Result<(), SchematicReplacementError> {
    let keys: Vec<_> = parameters
        .iter()
        .map(|parameter| parameter.name().to_owned())
        .collect();
    validate_parameter_keys(&keys, false)?;
    for parameter in parameters {
        if let Some(default) = parameter.default_value() {
            let authored = format!("{}={default}", parameter.name());
            let parsed = parse_replacement_parameters_strict(&authored).map_err(|error| {
                SchematicReplacementError::InvalidTargetContract {
                    reason: format!(
                        "default for parameter '{}' is not a lossless SPICE value: {error}",
                        parameter.name()
                    ),
                }
            })?;
            if parsed
                .get(&normalized(parameter.name()))
                .map(String::as_str)
                != Some(default)
            {
                return Err(SchematicReplacementError::InvalidTargetContract {
                    reason: format!(
                        "default for parameter '{}' is not represented losslessly",
                        parameter.name()
                    ),
                });
            }
        }
    }
    parameter_lookup(parameters).map(|_| ())
}

pub(super) fn terminal_lookup(
    terminals: &[SchematicReplacementTerminal],
) -> Result<HashMap<String, (usize, SchematicReplacementMappingStatus)>, SchematicReplacementError>
{
    let mut lookup = HashMap::new();
    for (index, terminal) in terminals.iter().enumerate() {
        insert_terminal_lookup(
            &mut lookup,
            terminal.name(),
            index,
            SchematicReplacementMappingStatus::Exact,
        )?;
        for alias in terminal.aliases() {
            insert_terminal_lookup(
                &mut lookup,
                alias,
                index,
                SchematicReplacementMappingStatus::Alias,
            )?;
        }
    }
    Ok(lookup)
}

fn insert_terminal_lookup(
    lookup: &mut HashMap<String, (usize, SchematicReplacementMappingStatus)>,
    name: &str,
    index: usize,
    status: SchematicReplacementMappingStatus,
) -> Result<(), SchematicReplacementError> {
    let key = normalized(name);
    if lookup.insert(key.clone(), (index, status)).is_some() {
        return Err(SchematicReplacementError::AmbiguousTerminalAlias { name: key });
    }
    Ok(())
}

fn parameter_lookup(
    parameters: &[SchematicReplacementParameter],
) -> Result<HashMap<String, (usize, SchematicReplacementMappingStatus)>, SchematicReplacementError>
{
    let mut lookup = HashMap::new();
    for (index, parameter) in parameters.iter().enumerate() {
        insert_parameter_lookup(
            &mut lookup,
            parameter.name(),
            index,
            SchematicReplacementMappingStatus::Exact,
        )?;
        for alias in parameter.aliases() {
            if !valid_replacement_parameter_name(alias) {
                return Err(SchematicReplacementError::InvalidTargetContract {
                    reason: format!("'{alias}' is not a valid parameter alias"),
                });
            }
            insert_parameter_lookup(
                &mut lookup,
                alias,
                index,
                SchematicReplacementMappingStatus::Alias,
            )?;
        }
    }
    Ok(lookup)
}

fn insert_parameter_lookup(
    lookup: &mut HashMap<String, (usize, SchematicReplacementMappingStatus)>,
    name: &str,
    index: usize,
    status: SchematicReplacementMappingStatus,
) -> Result<(), SchematicReplacementError> {
    let key = normalized(name);
    if lookup.insert(key.clone(), (index, status)).is_some() {
        return Err(SchematicReplacementError::AmbiguousParameterAlias { name: key });
    }
    Ok(())
}

pub(super) fn terminal_placements<'a>(
    component: &Component,
    terminals: &'a [SchematicReplacementTerminal],
) -> Result<Vec<TerminalPlacement<'a>>, SchematicReplacementError> {
    terminals
        .iter()
        .map(|terminal| {
            let offset = checked_transform(component, terminal.offset())?;
            let x = component
                .pos
                .x
                .checked_add(offset.x)
                .ok_or(SchematicReplacementError::CoordinateOverflow)?;
            let y = component
                .pos
                .y
                .checked_add(offset.y)
                .ok_or(SchematicReplacementError::CoordinateOverflow)?;
            Ok(TerminalPlacement {
                terminal,
                world: Point::new(x, y),
            })
        })
        .collect()
}

fn checked_transform(
    component: &Component,
    point: Point,
) -> Result<Point, SchematicReplacementError> {
    let x = if component.mirror_h {
        point
            .x
            .checked_neg()
            .ok_or(SchematicReplacementError::CoordinateOverflow)?
    } else {
        point.x
    };
    let y = if component.mirror_v {
        point
            .y
            .checked_neg()
            .ok_or(SchematicReplacementError::CoordinateOverflow)?
    } else {
        point.y
    };
    match component.rotation {
        Rotation::R0 => Ok(Point::new(x, y)),
        Rotation::R90 => Ok(Point::new(
            y.checked_neg()
                .ok_or(SchematicReplacementError::CoordinateOverflow)?,
            x,
        )),
        Rotation::R180 => Ok(Point::new(
            x.checked_neg()
                .ok_or(SchematicReplacementError::CoordinateOverflow)?,
            y.checked_neg()
                .ok_or(SchematicReplacementError::CoordinateOverflow)?,
        )),
        Rotation::R270 => Ok(Point::new(
            y,
            x.checked_neg()
                .ok_or(SchematicReplacementError::CoordinateOverflow)?,
        )),
    }
}

pub(super) fn validate_replacement_geometry(
    state: &SchematicDocument,
    source: &Component,
    replacement: &Component,
) -> Result<(), SchematicReplacementError> {
    let source_bounds = checked_component_bounds(source)?;
    let replacement_bounds = checked_component_bounds(replacement)?;
    for other in state
        .components
        .iter()
        .filter(|component| component.id != source.id)
    {
        let other_bounds = checked_component_bounds(other)?;
        if bounds_overlap(replacement_bounds, other_bounds)
            && !bounds_overlap(source_bounds, other_bounds)
        {
            return Err(SchematicReplacementError::GeometryCollision {
                other_component_id: other.id,
            });
        }
    }
    Ok(())
}

fn checked_component_bounds(
    component: &Component,
) -> Result<(i32, i32, i32, i32), SchematicReplacementError> {
    let (width, height) = component.symbol_dimensions();
    let (half_width, half_height) = if component.rotation.is_vertical() {
        (height / 2, width / 2)
    } else {
        (width / 2, height / 2)
    };
    Ok((
        component
            .pos
            .x
            .checked_sub(half_width)
            .ok_or(SchematicReplacementError::CoordinateOverflow)?,
        component
            .pos
            .y
            .checked_sub(half_height)
            .ok_or(SchematicReplacementError::CoordinateOverflow)?,
        component
            .pos
            .x
            .checked_add(half_width)
            .ok_or(SchematicReplacementError::CoordinateOverflow)?,
        component
            .pos
            .y
            .checked_add(half_height)
            .ok_or(SchematicReplacementError::CoordinateOverflow)?,
    ))
}

fn bounds_overlap(a: (i32, i32, i32, i32), b: (i32, i32, i32, i32)) -> bool {
    a.0 < b.2 && a.2 > b.0 && a.1 < b.3 && a.3 > b.1
}

fn directions_compatible(source: Option<PortDirection>, target: Option<PortDirection>) -> bool {
    match (source, target) {
        (Some(source), Some(target)) => {
            source == target || source == PortDirection::InOut || target == PortDirection::InOut
        }
        _ => true,
    }
}

fn model_status(source: &Component, target: &Component) -> SchematicReplacementSemanticStatus {
    let same_binding = match (&source.library_cell, &target.library_cell) {
        (Some(source), Some(target)) => {
            source.library.eq_ignore_ascii_case(&target.library)
                && source.cell.eq_ignore_ascii_case(&target.cell)
                && source.view.eq_ignore_ascii_case(&target.view)
                && source.module_name == target.module_name
        }
        (None, None) => source.kind == target.kind && source.value == target.value,
        _ => false,
    };
    if same_binding {
        SchematicReplacementSemanticStatus::Preserved
    } else {
        SchematicReplacementSemanticStatus::CompatibleChange
    }
}

fn netlist_status(
    source: &Component,
    target: &Component,
    source_terminals: &[SchematicReplacementTerminal],
    target_spec: &SchematicReplacementTargetSpec,
) -> SchematicReplacementSemanticStatus {
    let source_names: Vec<_> = source_terminals
        .iter()
        .map(|terminal| normalized(terminal.name()))
        .collect();
    let target_names: Vec<_> = target_spec
        .terminals()
        .iter()
        .map(|terminal| normalized(terminal.name()))
        .collect();
    if source.kind == target.kind && source_names == target_names {
        SchematicReplacementSemanticStatus::Preserved
    } else {
        SchematicReplacementSemanticStatus::CompatibleChange
    }
}

pub(super) fn normalized(name: &str) -> String {
    name.trim().to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::super::component::LibraryCellInstance;
    use super::super::history::SchematicSnapshot;
    use super::super::interface_repair::InstanceInterfaceRepair;
    use super::*;
    use rspice_design_model::port::PortSpec;

    #[test]
    fn discarded_replacement_and_repair_preserve_the_document() {
        let mut document = SchematicDocument::default();
        let mut component = Component::new(1, ComponentType::Resistor, Point::origin());
        component.name = "R1".to_owned();
        document.components.push(component);
        let context = ReplacementContext {
            selected_component: Some(1),
            topology_version: 7,
        };
        let source =
            SchematicReplacementSourceSpec::from_component(&document.components[0]).unwrap();
        let authority = replacement_authority_with_spec(&document, context, source).unwrap();
        let target = SchematicReplacementTargetSpec::primitive(ComponentType::Capacitor);
        let before = SchematicSnapshot::capture(&document);
        let replacement =
            InstanceReplacement::prepare(&mut document, context, &authority, &target).unwrap();
        assert!(replacement.impact().topology_changed);
        drop(replacement);
        assert!(before.is_equal_document(&document));

        let old_ports = [PortSpec {
            name: "IN".to_owned(),
            direction: PortDirection::In,
        }];
        assert_eq!(
            InstanceInterfaceRepair::prepare(&mut document, 1, &old_ports, |_| panic!(
                "ineligible source must not resolve symbols"
            ))
            .unwrap_err(),
            SchematicReplacementError::SelectExactlyOneInstance,
        );
        assert!(before.is_equal_document(&document));
        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&old_ports);
        document.components[0] = Component::new(1, ComponentType::CellInstance, Point::origin())
            .with_library_cell(binding);
        let new_ports = [PortSpec {
            name: "OUT".to_owned(),
            direction: PortDirection::Out,
        }];
        let before = SchematicSnapshot::capture(&document);
        assert_eq!(
            InstanceInterfaceRepair::prepare(&mut document, 1, &[], |_| panic!(
                "empty interface must not resolve symbols"
            ))
            .unwrap_err(),
            SchematicReplacementError::SelectExactlyOneInstance,
        );
        assert!(before.is_equal_document(&document));

        drop(InstanceInterfaceRepair::prepare(&mut document, 1, &new_ports, |_| None).unwrap());
        assert!(before.is_equal_document(&document));
    }
}
