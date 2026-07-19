use std::collections::{BTreeMap, HashMap, HashSet};

use super::super::super::replacement::{
    format_replacement_parameters, parse_replacement_parameters_strict,
    valid_replacement_parameter_name,
};
use super::super::super::{
    Component, ComponentType, Point, PortDirection, Rotation, SchematicReplacementAuthority,
    SchematicReplacementCompatibility, SchematicReplacementError, SchematicReplacementImpact,
    SchematicReplacementMappingStatus, SchematicReplacementParameter,
    SchematicReplacementParameterMapping, SchematicReplacementPreview,
    SchematicReplacementSemanticStatus, SchematicReplacementSourceSpec,
    SchematicReplacementTargetSpec, SchematicReplacementTerminal,
    SchematicReplacementTerminalMapping, SchematicReplacementValuePolicy,
    SchematicReplacementWireEdit,
};
use super::super::*;

#[derive(Debug)]
struct TerminalPlacement<'a> {
    terminal: &'a SchematicReplacementTerminal,
    world: Point,
}

#[derive(Debug)]
struct BuiltReplacement {
    preview: SchematicReplacementPreview,
}

impl SchematicState {
    /// Capture immutable authority for the exact selected component using its
    /// built-in or generated symbol contract.
    pub fn replacement_authority(
        &self,
    ) -> Result<SchematicReplacementAuthority, SchematicReplacementError> {
        let component_id = self
            .selection
            .single_component()
            .ok_or(SchematicReplacementError::SelectExactlyOneInstance)?;
        let component = self
            .components
            .iter()
            .find(|component| component.id == component_id)
            .ok_or(SchematicReplacementError::SourceInstanceMissing { component_id })?;
        self.replacement_authority_with_spec(SchematicReplacementSourceSpec::from_component(
            component,
        )?)
    }

    /// Capture immutable authority with caller-resolved authored terminals and
    /// source parameter schema. No library-manager reference escapes into the
    /// transaction.
    pub fn replacement_authority_with_spec(
        &self,
        source_spec: SchematicReplacementSourceSpec,
    ) -> Result<SchematicReplacementAuthority, SchematicReplacementError> {
        if self.read_only {
            return Err(SchematicReplacementError::ReadOnly);
        }
        let component_id = self
            .selection
            .single_component()
            .ok_or(SchematicReplacementError::SelectExactlyOneInstance)?;
        let source_component = self
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
            topology_version: self.topology_version(),
            source_component,
            source_spec,
        })
    }

    /// Analyze and materialize the exact non-mutating replacement candidate.
    pub fn preview_instance_replacement(
        &self,
        authority: &SchematicReplacementAuthority,
        target: &SchematicReplacementTargetSpec,
    ) -> Result<SchematicReplacementPreview, SchematicReplacementError> {
        Ok(self.build_instance_replacement(authority, target)?.preview)
    }

    /// Replace the selected instance atomically. All compatibility, stale
    /// authority, wire endpoint, and geometry checks finish before the single
    /// undo transaction begins.
    pub fn replace_selected_instance(
        &mut self,
        authority: &SchematicReplacementAuthority,
        target: &SchematicReplacementTargetSpec,
    ) -> Result<SchematicReplacementImpact, SchematicReplacementError> {
        if self.read_only {
            return Err(SchematicReplacementError::ReadOnly);
        }
        let built = self.build_instance_replacement(authority, target)?;
        let preview = built.preview;
        let impact = preview.impact;
        if !impact.topology_changed {
            return Err(SchematicReplacementError::NoChanges);
        }
        let component_id = impact.component_id;
        let replacement = preview.component;
        let replacement_counter = replacement
            .name
            .strip_prefix(replacement.kind.spice_prefix())
            .and_then(|suffix| suffix.parse::<u32>().ok())
            .map(|number| (replacement.kind.spice_prefix(), number));
        let wire_edits = preview.wire_edits;
        let affected_connections = preview.connections;

        let committed = self.with_undo("replace instance", move |state| {
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
                    if let Some(replacement_index) = edit.point_indices.get(connection.point_index)
                    {
                        connection.point_index = *replacement_index;
                    }
                }
            }
            state.selection.select_only_component(component_id);
            if let Some((prefix, number)) = replacement_counter {
                let counter = state.component_counters.entry(prefix).or_insert(0);
                *counter = (*counter).max(number);
            }
            state.net_mapping.clear();
            state.is_dirty = true;
            state.bump_topology_version();
        });
        if !committed {
            return Err(SchematicReplacementError::CommitFailed);
        }
        Ok(impact)
    }

    fn build_instance_replacement(
        &self,
        authority: &SchematicReplacementAuthority,
        target: &SchematicReplacementTargetSpec,
    ) -> Result<BuiltReplacement, SchematicReplacementError> {
        if self.read_only {
            return Err(SchematicReplacementError::ReadOnly);
        }
        validate_authority(self, authority)?;
        validate_target_spec(target)?;

        let source = &authority.source_component;
        let source_terminals = terminal_placements(source, authority.source_spec.terminals())?;

        let mut replacement = source.clone();
        replacement.kind = target.kind();
        replacement.name = replacement_reference_name(self, source, target.kind())?;
        replacement.value = match target.value_policy() {
            SchematicReplacementValuePolicy::UseTarget => target.value().to_owned(),
            SchematicReplacementValuePolicy::PreserveSource => source.value.clone(),
        };
        replacement.symbol_variant = target.symbol_variant().map(str::to_owned);
        replacement.library_cell = target.library_binding().cloned();

        let target_terminals = terminal_placements(&replacement, target.terminals())?;
        let target_lookup = terminal_lookup(target.terminals())?;
        let connected_source_names = connected_source_terminals(self, source, &source_terminals)?;

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
            self,
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

        let wire_result = build_wire_edits(self, source, &source_terminals, &terminal_map)?;
        validate_replacement_geometry(self, source, &replacement)?;

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
            self.connections.iter().find(|connection| {
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
        Ok(BuiltReplacement {
            preview: SchematicReplacementPreview {
                component: replacement,
                connections: wire_result.connections,
                wire_edits: wire_result.edits,
                compatibility,
                impact,
            },
        })
    }
}

fn replacement_reference_name(
    state: &SchematicState,
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

struct WireBuildResult {
    edits: Vec<SchematicReplacementWireEdit>,
    connections: Vec<WireConnection>,
    relocated_points: usize,
}

fn build_wire_edits(
    state: &SchematicState,
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
            super::movement_ops::orthogonal_route_for_corresponding_points(
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
        let replacement_wire = super::super::super::Wire::new(wire_id, replacement_points.clone());
        for tap in state.bus_taps.iter().filter(|tap| {
            tap.target_kind() == super::super::super::BusTargetKind::Wire
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
    state: &SchematicState,
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
                            super::super::super::WireSegment::new(mapped_source.world, *adjacent)
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

fn connected_source_terminals(
    state: &SchematicState,
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
    state: &SchematicState,
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
    state: &SchematicState,
    authority: &SchematicReplacementAuthority,
) -> Result<(), SchematicReplacementError> {
    if state.selection.single_component() != Some(authority.component_id)
        || state.topology_version() != authority.topology_version
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

fn terminal_lookup(
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

fn terminal_placements<'a>(
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

fn validate_replacement_geometry(
    state: &SchematicState,
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

fn normalized(name: &str) -> String {
    name.trim().to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        LibraryCellInstance, PortSpec, SchematicReplacementParameter, SchematicSnapshot, Wire,
    };

    fn five_pin_binding(cell: &str) -> LibraryCellInstance {
        let mut binding = LibraryCellInstance::new("analog", cell, "symbol");
        binding.terminal_order = ["IN+", "IN-", "V+", "V-", "OUT"]
            .into_iter()
            .map(str::to_owned)
            .collect();
        binding.terminal_dirs = vec![
            PortDirection::In,
            PortDirection::In,
            PortDirection::Supply,
            PortDirection::Supply,
            PortDirection::Out,
        ];
        binding.module_name = Some(cell.to_owned());
        binding
    }

    fn selected_opamp() -> SchematicState {
        let binding = five_pin_binding("OPA189");
        let mut state = SchematicState::default();
        let id = state.add_library_cell_component(Point::new(100, 100), binding);
        let component = state
            .components
            .iter_mut()
            .find(|item| item.id == id)
            .unwrap();
        component.name = "U1".to_owned();
        component.value = "OPA189".to_owned();
        component.params = "gain=100 ibias=2n vos=1u slew=20Meg en=5n temp=27".to_owned();
        state.selection.select_only_component(id);
        state.recalculate_runtime_state();
        state.clear_undo_history();
        state
    }

    fn opa188_target() -> SchematicReplacementTargetSpec {
        SchematicReplacementTargetSpec::library_cell(five_pin_binding("OPA188"))
            .with_value("OPA188")
            .with_parameters(vec![
                SchematicReplacementParameter::new("gain"),
                SchematicReplacementParameter::new("ibias"),
                SchematicReplacementParameter::new("vos"),
                SchematicReplacementParameter::new("slew"),
                SchematicReplacementParameter::new("en"),
                SchematicReplacementParameter::new("temperature").with_aliases(["temp"]),
                SchematicReplacementParameter::new("gbw").with_default("18Meg"),
                SchematicReplacementParameter::new("vnoise").with_default("5.2n"),
            ])
    }

    #[test]
    fn mockup_opamp_contract_reports_five_pins_and_six_of_eight_parameters() {
        let state = selected_opamp();
        let source = state.components[0].clone();
        let source_spec = SchematicReplacementSourceSpec::from_component(&source)
            .unwrap()
            .with_parameter_keys(["gain", "ibias", "vos", "slew", "en", "temp"]);
        let authority = state.replacement_authority_with_spec(source_spec).unwrap();
        let preview = state
            .preview_instance_replacement(&authority, &opa188_target())
            .unwrap();

        assert_eq!(preview.compatibility.mapped_terminal_count, 5);
        assert_eq!(preview.compatibility.target_terminal_count, 5);
        assert_eq!(preview.compatibility.mapped_parameter_count, 6);
        assert_eq!(preview.compatibility.target_parameter_count, 8);
        assert_eq!(preview.component.id, source.id);
        assert_eq!(preview.component.name, "U1");
        assert_eq!(preview.component.pos, source.pos);
        assert_eq!(preview.component.rotation, source.rotation);
        assert_eq!(preview.component.mirror_h, source.mirror_h);
        assert_eq!(preview.component.mirror_v, source.mirror_v);
        assert!(preview.component.params.contains("temperature=27"));
        assert!(preview.component.params.contains("gbw=18Meg"));
    }

    #[test]
    fn commit_is_one_undo_step_and_preserves_clipboard_identity_and_placement() {
        let mut state = selected_opamp();
        let authority = state.replacement_authority().unwrap();
        state.clipboard.components.push(state.components[0].clone());
        let clipboard = serde_json::to_value(&state.clipboard).unwrap();
        let before = state.components[0].clone();
        let topology = state.topology_version();

        let impact = state
            .replace_selected_instance(&authority, &opa188_target())
            .unwrap();

        let after = &state.components[0];
        assert_eq!(after.id, before.id);
        assert_eq!(after.name, before.name);
        assert_eq!(after.pos, before.pos);
        assert_eq!(serde_json::to_value(&state.clipboard).unwrap(), clipboard);
        assert_eq!(impact.component_id, before.id);
        assert_eq!(state.topology_version(), topology.wrapping_add(1));
        assert_eq!(state.undo_description(), Some("replace instance"));
        assert!(state.undo());
        assert_eq!(state.components[0], before);
        assert!(
            !state.can_undo(),
            "replacement must create exactly one undo record"
        );
    }

    #[test]
    fn cross_prefix_replacement_preserves_id_and_assigns_a_unique_reference() {
        let mut state = SchematicState::default();
        let resistor_id = state.add_component(ComponentType::Resistor, Point::origin());
        state.add_component(ComponentType::Capacitor, Point::new(100, 0));
        state.selection.select_only_component(resistor_id);
        state.clear_undo_history();
        let authority = state.replacement_authority().unwrap();

        let preview = state
            .preview_instance_replacement(
                &authority,
                &SchematicReplacementTargetSpec::primitive(ComponentType::Capacitor),
            )
            .unwrap();
        assert_eq!(preview.component.id, resistor_id);
        assert_eq!(preview.component.name, "C2");
        state
            .replace_selected_instance(
                &authority,
                &SchematicReplacementTargetSpec::primitive(ComponentType::Capacitor),
            )
            .unwrap();
        assert_eq!(state.components[0].id, resistor_id);
        assert_eq!(state.components[0].name, "C2");
        let next = state.add_component(ComponentType::Capacitor, Point::new(200, 0));
        assert_eq!(
            state
                .components
                .iter()
                .find(|component| component.id == next)
                .unwrap()
                .name,
            "C3"
        );
        assert!(state.undo());
        assert_eq!(state.components[0].name, "R1");
    }

    #[test]
    fn connected_terminal_alias_retargets_connection_and_wire_endpoint() {
        let mut state = SchematicState::default();
        let id = state.add_component(ComponentType::Resistor, Point::new(100, 100));
        state.selection.select_only_component(id);
        let terminal = state.components[0]
            .terminal_positions()
            .into_iter()
            .find(|(name, _)| *name == "+")
            .unwrap()
            .1;
        let wire_id = state.next_id();
        state
            .wires
            .push(Wire::segment(wire_id, terminal, Point::new(20, 100)));
        state
            .connections
            .push(WireConnection::new(wire_id, 0, id, "+"));
        state.recalculate_runtime_state();
        state.clear_undo_history();
        let authority = state.replacement_authority().unwrap();
        let mut binding = LibraryCellInstance::new("analog", "R_ALIAS", "symbol");
        binding.terminal_order = vec!["P".to_owned(), "N".to_owned()];
        binding.terminal_dirs = vec![PortDirection::InOut, PortDirection::InOut];
        let target = SchematicReplacementTargetSpec::library_cell(binding)
            .with_terminals(vec![
                SchematicReplacementTerminal::new("P", Point::new(-30, 0)).with_aliases(["+"]),
                SchematicReplacementTerminal::new("N", Point::new(30, 0)).with_aliases(["-"]),
            ])
            .with_value("R_ALIAS");

        let preview = state
            .preview_instance_replacement(&authority, &target)
            .unwrap();
        assert_eq!(
            preview.wire_edits[0].original_points,
            [Point::new(80, 100), Point::new(20, 100)]
        );
        assert_eq!(
            preview.wire_edits[0].replacement_points,
            [Point::new(70, 100), Point::new(20, 100)]
        );
        assert_eq!(preview.connections[0].terminal_name, "P");
        state
            .replace_selected_instance(&authority, &target)
            .unwrap();
        assert_eq!(state.wires[0].points[0], Point::new(70, 100));
        assert_eq!(state.connections[0].terminal_name, "P");
    }

    #[test]
    fn diagonal_pin_displacement_inserts_an_orthogonal_bend_and_remaps_connections() {
        let mut state = SchematicState::default();
        let id = state.add_component(ComponentType::Resistor, Point::new(100, 100));
        state.selection.select_only_component(id);
        let terminal = state.components[0].terminal_positions()[0].1;
        let wire_id = state.next_id();
        state
            .wires
            .push(Wire::segment(wire_id, terminal, Point::new(20, 100)));
        state
            .connections
            .push(WireConnection::new(wire_id, 0, id, "+"));
        state.recalculate_runtime_state();
        state.clear_undo_history();
        let authority = state.replacement_authority().unwrap();
        let mut binding = LibraryCellInstance::new("analog", "R_OFFSET", "symbol");
        binding.terminal_order = vec!["P".to_owned(), "N".to_owned()];
        binding.terminal_dirs = vec![PortDirection::InOut, PortDirection::InOut];
        let target = SchematicReplacementTargetSpec::library_cell(binding)
            .with_terminals(vec![
                SchematicReplacementTerminal::new("P", Point::new(-30, -10)).with_aliases(["+"]),
                SchematicReplacementTerminal::new("N", Point::new(30, 0)).with_aliases(["-"]),
            ])
            .with_value("R_OFFSET");

        let preview = state
            .preview_instance_replacement(&authority, &target)
            .unwrap();
        assert_eq!(preview.impact.relocated_wire_points, 1);
        assert!(preview.wire_edits[0].replacement_points.len() >= 3);
        state
            .replace_selected_instance(&authority, &target)
            .unwrap();
        assert!(state.wires[0].is_orthogonal());
        assert_eq!(
            state.wires[0].points[state.connections[0].point_index],
            Point::new(70, 90)
        );
        assert!(state.undo());
        assert_eq!(state.wires[0].points, [terminal, Point::new(20, 100)]);
        assert_eq!(state.connections[0].point_index, 0);
    }

    #[test]
    fn stale_authority_and_unmapped_connected_pin_are_non_mutating() {
        let mut state = SchematicState::default();
        let id = state.add_component(ComponentType::Resistor, Point::new(100, 100));
        state.selection.select_only_component(id);
        let authority = state.replacement_authority().unwrap();
        let before = SchematicSnapshot::capture(&state);
        state.components[0].value = "2k".to_owned();
        assert_eq!(
            state.preview_instance_replacement(
                &authority,
                &SchematicReplacementTargetSpec::primitive(ComponentType::Capacitor)
            ),
            Err(SchematicReplacementError::StaleAuthority)
        );
        state.components[0] = authority.source_component().clone();
        assert!(before.is_equal_state(&state));

        let terminal = state.components[0].terminal_positions()[0].1;
        let wire_id = state.next_id();
        state
            .wires
            .push(Wire::segment(wire_id, terminal, Point::new(0, 100)));
        state
            .connections
            .push(WireConnection::new(wire_id, 0, id, "+"));
        state.bump_topology_version();
        let authority = state.replacement_authority().unwrap();
        let mut binding = LibraryCellInstance::new("work", "one_pin", "schematic");
        binding.bind_interface(&[PortSpec {
            name: "only".to_owned(),
            direction: PortDirection::InOut,
        }]);
        let target = SchematicReplacementTargetSpec::library_cell(binding).with_terminals(vec![
            SchematicReplacementTerminal::new("only", Point::new(20, 0)),
        ]);
        let snapshot = SchematicSnapshot::capture(&state);
        assert_eq!(
            state.preview_instance_replacement(&authority, &target),
            Err(SchematicReplacementError::UnmappedConnectedTerminal {
                terminal: "+".to_owned()
            })
        );
        assert!(snapshot.is_equal_state(&state));
        assert!(!state.can_undo());
    }

    #[test]
    fn invalid_parameter_and_coordinate_contracts_fail_without_panicking() {
        let mut state = SchematicState::default();
        let id = state.add_component(ComponentType::Resistor, Point::new(i32::MAX, 0));
        state.selection.select_only_component(id);
        assert_eq!(
            state.replacement_authority(),
            Err(SchematicReplacementError::CoordinateOverflow)
        );

        state.components[0].pos = Point::origin();
        state.components[0].params = "gain".to_owned();
        assert!(matches!(
            state.replacement_authority(),
            Err(SchematicReplacementError::MalformedParameterString { .. })
        ));
    }

    #[test]
    fn required_target_parameters_need_a_mapping_or_lossless_default() {
        let state = selected_opamp();
        let authority = state.replacement_authority().unwrap();
        let missing =
            SchematicReplacementTargetSpec::library_cell(five_pin_binding("OPA_REQUIRED"))
                .with_value("OPA_REQUIRED")
                .with_parameters(vec![
                    SchematicReplacementParameter::new("corner").required(),
                ]);
        assert_eq!(
            state.preview_instance_replacement(&authority, &missing),
            Err(SchematicReplacementError::MissingRequiredParameter {
                parameter: "corner".to_owned()
            })
        );

        let defaulted =
            SchematicReplacementTargetSpec::library_cell(five_pin_binding("OPA_DEFAULT"))
                .with_value("OPA_DEFAULT")
                .with_parameters(vec![
                    SchematicReplacementParameter::new("corner")
                        .required()
                        .with_default(r#""tt slow""#),
                ]);
        let preview = state
            .preview_instance_replacement(&authority, &defaulted)
            .unwrap();
        assert!(preview.component.params.contains(r#"corner="tt slow""#));
    }
}
