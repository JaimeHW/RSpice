//! Array replication.
//!
//! Replicating a selection across a row/column grid, and keeping the result
//! electrically sound: replicas are collision-checked against existing
//! geometry before they are committed, and pin-to-pin abutment between
//! adjacent replicas becomes a real connection rather than a coincidence.

use std::collections::{HashMap, HashSet};

use crate::state::DocumentationShapeGeometry;

use super::super::super::{
    BusNotation, BusSlice, MAX_BUS_MEMBER_INDEX, SchematicArrayCount, SchematicArrayError,
    SchematicArrayImpact, SchematicArrayKind, SchematicArrayNameAtom, SchematicArrayNaming,
    SchematicArrayPlacement, SchematicArrayPlan, SchematicArrayPreview, geometry_from_points,
};
use super::super::*;

const MAX_ARRAY_GENERATED_OBJECTS: usize = 65_536;
const MAX_ARRAY_GENERATED_SEGMENTS: usize = 131_072;

#[derive(Default)]
struct ArrayAdditions {
    components: Vec<Component>,
    wires: Vec<Wire>,
    junctions: Vec<Junction>,
    buses: Vec<Bus>,
    bus_taps: Vec<BusTap>,
    net_labels: Vec<NetLabel>,
    design_notes: Vec<DesignNote>,
    documentation_shapes: Vec<DocumentationShape>,
    connections: Vec<WireConnection>,
    selection: Selection,
}

struct BuiltArray {
    preview: SchematicArrayPreview,
    next_id: u64,
}

struct CapturedArraySelection {
    objects: ClipboardData,
    connections: Vec<WireConnection>,
}

struct FreshIdAllocator {
    next: u64,
    used: HashSet<u64>,
}

impl FreshIdAllocator {
    fn from_state(state: &SchematicState) -> Self {
        Self {
            next: state.next_id.max(1),
            used: live_ids(state).collect(),
        }
    }

    fn allocate(&mut self) -> Result<u64, SchematicArrayError> {
        let start = self.next.max(1);
        loop {
            let id = self.next.max(1);
            self.next = id.checked_add(1).unwrap_or(1);
            if self.used.insert(id) {
                return Ok(id);
            }
            if self.next == start {
                return Err(SchematicArrayError::IdentifierExhausted);
            }
        }
    }
}

#[derive(Default)]
struct ReplicaGeometry {
    component_bounds: Vec<(u64, (i32, i32, i32, i32))>,
    terminals: Vec<(u64, Point)>,
    conductors: Vec<(u64, Point, Point)>,
    anchors: Vec<(u64, Point)>,
}

#[derive(Clone, Copy)]
enum CollisionPrimitive {
    Body {
        id: u64,
        bounds: (i32, i32, i32, i32),
    },
    Terminal {
        id: u64,
        point: Point,
    },
    Conductor {
        id: u64,
        start: Point,
        end: Point,
    },
    Anchor {
        id: u64,
        point: Point,
    },
}

#[derive(Default)]
struct GeometryBroadPhase {
    primitives: Vec<CollisionPrimitive>,
    buckets: HashMap<(i32, i32), Vec<usize>>,
    global: Vec<usize>,
}

impl GeometryBroadPhase {
    const CELL_SIZE: i32 = 64;
    const MAX_BUCKETS_PER_PRIMITIVE: usize = 1_024;

    fn from_geometry(geometry: &ReplicaGeometry) -> Self {
        let mut index = Self::default();
        index.extend(geometry);
        index
    }

    fn extend(&mut self, geometry: &ReplicaGeometry) {
        for &(id, bounds) in &geometry.component_bounds {
            self.insert(CollisionPrimitive::Body { id, bounds });
        }
        for &(id, point) in &geometry.terminals {
            self.insert(CollisionPrimitive::Terminal { id, point });
        }
        for &(id, start, end) in &geometry.conductors {
            self.insert(CollisionPrimitive::Conductor { id, start, end });
        }
        for &(id, point) in &geometry.anchors {
            self.insert(CollisionPrimitive::Anchor { id, point });
        }
    }

    fn insert(&mut self, primitive: CollisionPrimitive) {
        let index = self.primitives.len();
        let bounds = primitive_bounds(primitive);
        self.primitives.push(primitive);
        let Some(cells) = covered_cells(bounds, Self::MAX_BUCKETS_PER_PRIMITIVE) else {
            self.global.push(index);
            return;
        };
        for cell in cells {
            self.buckets.entry(cell).or_default().push(index);
        }
    }

    fn query(&self, bounds: (i32, i32, i32, i32)) -> Vec<CollisionPrimitive> {
        let mut indices: HashSet<usize> = self.global.iter().copied().collect();
        if let Some(cells) = covered_cells(bounds, Self::MAX_BUCKETS_PER_PRIMITIVE) {
            for cell in cells {
                if let Some(bucket) = self.buckets.get(&cell) {
                    indices.extend(bucket.iter().copied());
                }
            }
        } else {
            indices.extend(0..self.primitives.len());
        }
        indices
            .into_iter()
            .filter_map(|index| self.primitives.get(index).copied())
            .collect()
    }
}

impl SchematicState {
    /// Whether at least one complete, live schematic object can be repeated.
    /// Sub-object-only wire selections intentionally do not enable the command.
    pub fn has_live_array_selection(&self) -> bool {
        if !self.selection.wire_segments.is_empty() || !self.selection.wire_vertices.is_empty() {
            return false;
        }
        self.components
            .iter()
            .any(|item| self.selection.has_component(item.id))
            || self
                .wires
                .iter()
                .any(|item| self.selection.has_wire(item.id) && item.points.len() >= 2)
            || self
                .buses
                .iter()
                .any(|item| self.selection.has_bus(item.id) && item.validate().is_ok())
            || self
                .bus_taps
                .iter()
                .any(|item| self.selection.has_bus_tap(item.id))
            || self
                .net_labels
                .iter()
                .any(|item| self.selection.has_net_label(item.id))
            || self
                .design_notes
                .iter()
                .any(|item| self.selection.has_design_note(item.id) && item.validate().is_ok())
            || self.documentation_shapes.iter().any(|item| {
                self.selection.has_documentation_shape(item.id) && item.validate().is_ok()
            })
    }

    /// Validate the complete source set that Create Array would capture,
    /// without requiring a placement plan and without mutating the document.
    ///
    /// Unlike [`Self::has_live_array_selection`], this rejects mixed
    /// selections containing any stale, partial, malformed, or unsupported
    /// object even when another selected object is otherwise valid. UI command
    /// enablement should use this strict contract.
    pub fn validate_array_source_selection(&self) -> Result<(), SchematicArrayError> {
        validate_live_array_selection(self)?;
        validate_array_source_selection_direct(self)
    }

    /// Produce the deterministic naming contract shown by the array dialog.
    /// Every range starts at the retained source identity and contains exactly
    /// `count.member_count()` members.  Existing reference collisions are
    /// rejected here, before the dialog can present a misleading valid state.
    pub fn default_array_naming(
        &self,
        count: SchematicArrayCount,
    ) -> Result<SchematicArrayNaming, SchematicArrayError> {
        let member_count = count.member_count();
        let mut clauses = Vec::new();
        let mut selected_components: Vec<_> = self
            .components
            .iter()
            .filter(|item| self.selection.has_component(item.id))
            .collect();
        selected_components.sort_by_key(|item| item.id);
        let mut reference_groups =
            HashMap::<(String, String, Option<usize>), Vec<SchematicArrayNameAtom>>::new();
        for component in &selected_components {
            if component.name.is_empty() {
                continue;
            }
            let atom = SchematicArrayNameAtom::parse(&component.name)?;
            if let SchematicArrayNameAtom::Reference {
                prefix,
                suffix,
                minimum_width,
                ..
            } = &atom
            {
                reference_groups
                    .entry((prefix.clone(), suffix.clone(), *minimum_width))
                    .or_default()
                    .push(atom);
            } else {
                return Err(SchematicArrayError::InvalidSourceName {
                    name: component.name.clone(),
                });
            }
        }
        let selected_ids = &self.selection.components;
        let occupied_names: HashSet<String> = self
            .components
            .iter()
            .filter(|component| !selected_ids.contains(&component.id))
            .map(|component| component.name.to_ascii_lowercase())
            .collect();
        let mut reference_strides = HashMap::new();
        for (key, atoms) in &reference_groups {
            reference_strides.insert(
                key.clone(),
                find_collision_free_reference_stride(atoms, member_count, &occupied_names)?,
            );
        }
        for component in selected_components {
            if component.name.is_empty() {
                continue;
            }
            let atom = SchematicArrayNameAtom::parse(&component.name)?;
            let key = match &atom {
                SchematicArrayNameAtom::Reference {
                    prefix,
                    suffix,
                    minimum_width,
                    ..
                } => (prefix.clone(), suffix.clone(), *minimum_width),
                _ => unreachable!("component references were validated above"),
            };
            let stride = reference_strides.get(&key).copied().unwrap_or(1);
            let end = atom
                .numeric_value()
                .checked_add(
                    u64::try_from(member_count.saturating_sub(1))
                        .map_err(|_| SchematicArrayError::CountOverflow)?
                        .checked_mul(stride)
                        .ok_or(SchematicArrayError::CountOverflow)?,
                )
                .ok_or(SchematicArrayError::CountOverflow)?;
            clauses.push(format!("{}…{}", atom, atom.with_numeric_value(end)));
        }

        let mut indexed_sources = Vec::<(String, u32, BusNotation)>::new();
        let mut selected_labels: Vec<_> = self
            .net_labels
            .iter()
            .filter(|item| self.selection.has_net_label(item.id))
            .collect();
        selected_labels.sort_by_key(|item| item.id);
        for label in selected_labels {
            if let Some((name, start, notation)) = scalar_bus_name(&label.name) {
                indexed_sources.push((name, start, notation));
            }
        }

        let mut selected_taps: Vec<_> = self
            .bus_taps
            .iter()
            .filter(|item| self.selection.has_bus_tap(item.id) && item.slice.is_scalar())
            .collect();
        selected_taps.sort_by_key(|item| item.id);
        for tap in selected_taps {
            indexed_sources.push((tap.slice.name.clone(), tap.slice.msb, tap.slice.notation));
        }
        let mut seen_indexed_sources = HashSet::new();
        indexed_sources.retain(|(name, start, notation)| {
            seen_indexed_sources.insert((name.to_ascii_lowercase(), *start, *notation))
        });
        let mut indexed_groups = HashMap::<(String, BusNotation), Vec<u64>>::new();
        for (name, start, notation) in &indexed_sources {
            indexed_groups
                .entry((name.to_ascii_lowercase(), *notation))
                .or_default()
                .push(u64::from(*start));
        }
        let mut indexed_strides = HashMap::new();
        for (key, starts) in &indexed_groups {
            indexed_strides.insert(
                key.clone(),
                find_collision_free_index_stride(starts, member_count)?,
            );
        }
        for (name, start, notation) in indexed_sources {
            let stride = indexed_strides
                .get(&(name.to_ascii_lowercase(), notation))
                .copied()
                .unwrap_or(1);
            let end = checked_bus_sequence_end(start, member_count, stride)?;
            clauses.push(format_bus_range(&name, start, end, notation));
        }

        let naming = SchematicArrayNaming::parse(&clauses.join(" · "))?
            .normalized_for_members(member_count)?;
        self.validate_array_naming(&naming, member_count)?;
        Ok(naming)
    }

    pub fn preview_array_selection(
        &self,
        plan: &SchematicArrayPlan,
    ) -> Result<SchematicArrayPreview, SchematicArrayError> {
        self.preview_array_selection_resolved(
            plan,
            |component| {
                component
                    .terminal_positions()
                    .into_iter()
                    .map(|(name, point)| (name.to_owned(), point))
                    .collect()
            },
            Component::bounding_box,
        )
    }

    /// Build the exact immutable additions that commit will apply.  The live
    /// document, clipboard, dirty flag, topology version, and undo history are
    /// untouched by preview.
    pub fn preview_array_selection_resolved(
        &self,
        plan: &SchematicArrayPlan,
        mut terminal_points_for: impl FnMut(&Component) -> Vec<(String, Point)>,
        mut component_bounds_for: impl FnMut(&Component) -> (i32, i32, i32, i32),
    ) -> Result<SchematicArrayPreview, SchematicArrayError> {
        Ok(self
            .build_array(plan, &mut terminal_points_for, &mut component_bounds_for)?
            .preview)
    }

    pub fn array_selection(
        &mut self,
        plan: &SchematicArrayPlan,
    ) -> Result<SchematicArrayImpact, SchematicArrayError> {
        self.array_selection_resolved(
            plan,
            |component| {
                component
                    .terminal_positions()
                    .into_iter()
                    .map(|(name, point)| (name.to_owned(), point))
                    .collect()
            },
            Component::bounding_box,
        )
    }

    /// Atomically commit a previously previewable array using resolved symbol
    /// geometry.  Candidate validation completes before the undo transaction
    /// begins; a rejection is therefore a true no-op.
    pub fn array_selection_resolved(
        &mut self,
        plan: &SchematicArrayPlan,
        mut terminal_points_for: impl FnMut(&Component) -> Vec<(String, Point)>,
        mut component_bounds_for: impl FnMut(&Component) -> (i32, i32, i32, i32),
    ) -> Result<SchematicArrayImpact, SchematicArrayError> {
        if self.read_only {
            return Err(SchematicArrayError::ReadOnly);
        }
        let built = self.build_array(plan, &mut terminal_points_for, &mut component_bounds_for)?;
        let impact = built.preview.impact;
        let electrical = impact.electrical;
        let next_id = built.next_id;
        let preview = built.preview;
        let committed = self.with_undo("create array", move |state| {
            state.components.extend(preview.components);
            state.wires.extend(preview.wires);
            state.junctions.extend(preview.junctions);
            state.buses.extend(preview.buses);
            state.bus_taps.extend(preview.bus_taps);
            state.net_labels.extend(preview.net_labels);
            state.design_notes.extend(preview.design_notes);
            state
                .documentation_shapes
                .extend(preview.documentation_shapes);
            state.connections.extend(preview.connections);
            state.selection = preview.selection;
            state.next_id = next_id;
            state.rebuild_component_counters_after_array();
            state.is_dirty = true;
            if electrical {
                state.bump_topology_version();
            }
        });
        if !committed {
            return Err(SchematicArrayError::CommitFailed);
        }
        Ok(impact)
    }

    fn build_array(
        &self,
        plan: &SchematicArrayPlan,
        terminal_points_for: &mut impl FnMut(&Component) -> Vec<(String, Point)>,
        component_bounds_for: &mut impl FnMut(&Component) -> (i32, i32, i32, i32),
    ) -> Result<BuiltArray, SchematicArrayError> {
        if self.read_only {
            return Err(SchematicArrayError::ReadOnly);
        }
        validate_live_array_selection(self)?;
        validate_array_source_selection_direct(self)?;
        let normalized_plan =
            SchematicArrayPlan::new(plan.kind, plan.count, plan.naming.clone(), plan.placement)?;
        let plan = &normalized_plan;
        let capture = capture_array_selection(self, |component| {
            terminal_points_for(component)
                .into_iter()
                .map(|(_, point)| point)
                .collect()
        });
        if !capture.objects.has_content() {
            return Err(SchematicArrayError::EmptySelection);
        }
        let electrical = capture_is_electrical(&capture.objects);
        if plan.kind == SchematicArrayKind::RadialDocumentation && electrical {
            return Err(SchematicArrayError::RadialDocumentationOnly {
                object_id: first_electrical_object_id(&capture.objects),
            });
        }
        if plan.kind == SchematicArrayKind::RadialDocumentation
            && (capture.objects.design_notes.is_empty()
                && capture.objects.documentation_shapes.is_empty())
        {
            return Err(SchematicArrayError::UnsupportedSelection);
        }
        validate_captured_array_selection(&capture)?;
        let members = plan.count.member_count();
        validate_generation_budget(&capture, members)?;
        self.validate_array_naming(&plan.naming, members)?;
        validate_naming_coverage(&capture.objects, &plan.naming, &self.selection)?;

        let mut ids = FreshIdAllocator::from_state(self);
        let mut additions = ArrayAdditions {
            selection: closed_source_selection(&capture.objects),
            ..Default::default()
        };
        let outside_geometry = geometry_for_state(self, terminal_points_for, component_bounds_for);
        let mut collision_index = GeometryBroadPhase::from_geometry(&outside_geometry);
        let radial_documentation = plan.kind == SchematicArrayKind::RadialDocumentation;
        let mut radial_note_positions: Vec<(u64, Point)> = if radial_documentation {
            self.design_notes
                .iter()
                .map(|note| (note.id, note.pos))
                .collect()
        } else {
            Vec::new()
        };
        let mut radial_shape_geometries: Vec<(u64, DocumentationShapeGeometry)> =
            if radial_documentation {
                self.documentation_shapes
                    .iter()
                    .map(|shape| (shape.id, shape.geometry.clone()))
                    .collect()
            } else {
                Vec::new()
            };

        for member_index in 1..members {
            let transform = member_transform(plan, member_index)?;
            let mut component_map = HashMap::new();
            let mut wire_map = HashMap::new();
            let mut bus_map = HashMap::new();
            let mut replica = ArrayAdditions::default();

            for source in &capture.objects.components {
                let mut item = source.clone();
                let old_id = item.id;
                item.id = ids.allocate()?;
                item.pos = transform_point(item.pos, transform)?;
                if !item.name.is_empty() {
                    item.name = plan
                        .naming
                        .value_for_source(&item.name, member_index)
                        .ok_or_else(|| SchematicArrayError::MissingNamingRange {
                            source: item.name.clone(),
                        })?;
                }
                component_map.insert(old_id, item.id);
                replica.selection.select_component(item.id);
                replica.components.push(item);
            }

            for source in &capture.objects.wires {
                let mut item = source.clone();
                let old_id = item.id;
                item.id = ids.allocate()?;
                item.points = item
                    .points
                    .iter()
                    .copied()
                    .map(|point| transform_point(point, transform))
                    .collect::<Result<_, _>>()?;
                wire_map.insert(old_id, item.id);
                replica.selection.select_wire(item.id);
                replica.wires.push(item);
            }

            for source in &capture.objects.buses {
                let mut item = source.clone();
                let old_id = item.id;
                item.id = ids.allocate()?;
                item.points = item
                    .points
                    .iter()
                    .copied()
                    .map(|point| transform_point(point, transform))
                    .collect::<Result<_, _>>()?;
                if item.validate().is_err() {
                    return Err(SchematicArrayError::InvalidGeometry { object_id: old_id });
                }
                bus_map.insert(old_id, item.id);
                replica.selection.select_bus(item.id);
                replica.buses.push(item);
            }

            for source in &capture.objects.bus_taps {
                let mut item = source.clone();
                let old_id = item.id;
                item.id = ids.allocate()?;
                item.bus_id = *bus_map
                    .get(&source.bus_id)
                    .ok_or(SchematicArrayError::InvalidBusTap { tap_id: old_id })?;
                item.bus_point = transform_point(item.bus_point, transform)?;
                item.connection_point = transform_point(item.connection_point, transform)?;
                if item.slice.is_scalar() {
                    let source_name = item.slice.to_string();
                    if let Some(value) = plan.naming.value_for_source(&source_name, member_index) {
                        item.slice = BusSlice::parse(&value)
                            .map_err(|_| SchematicArrayError::InvalidSourceName { name: value })?;
                    }
                }
                let bus = replica
                    .buses
                    .iter()
                    .find(|bus| bus.id == item.bus_id)
                    .ok_or(SchematicArrayError::InvalidBusTap { tap_id: old_id })?;
                item.validate_against_bus(bus)
                    .map_err(|_| SchematicArrayError::InvalidBusTap { tap_id: old_id })?;
                replica.selection.select_bus_tap(item.id);
                replica.bus_taps.push(item);
            }

            for source in &capture.objects.net_labels {
                let mut item = source.clone();
                item.id = ids.allocate()?;
                item.pos = transform_point(item.pos, transform)?;
                if scalar_bus_name(&item.name).is_some()
                    && let Some(value) = plan.naming.value_for_source(&item.name, member_index)
                {
                    item.name = value;
                }
                NetLabel::validate_name(&item.name, self.document_policy.net_naming).map_err(
                    |_| SchematicArrayError::InvalidSourceName {
                        name: item.name.clone(),
                    },
                )?;
                replica.selection.select_net_label(item.id);
                replica.net_labels.push(item);
            }

            for source in &capture.objects.junctions {
                let pos = transform_point(*source, transform)?;
                let item = Junction::new(ids.allocate()?, pos);
                replica.selection.select_junction(pos);
                replica.junctions.push(item);
            }

            for source in &capture.objects.design_notes {
                let id = ids.allocate()?;
                let pos = transform_point(source.pos, transform)?;
                let mut item =
                    DesignNote::new(id, pos, source.kind, source.text.clone()).map_err(|_| {
                        SchematicArrayError::InvalidGeometry {
                            object_id: source.id,
                        }
                    })?;
                item.layer = source.layer;
                item.validate()
                    .map_err(|_| SchematicArrayError::InvalidGeometry {
                        object_id: source.id,
                    })?;
                if let Some((other_id, _)) = radial_note_positions
                    .iter()
                    .find(|(_, existing)| *existing == pos)
                {
                    return Err(SchematicArrayError::GeometryCollision {
                        object_id: id,
                        other_id: *other_id,
                    });
                }
                if radial_documentation {
                    radial_note_positions.push((id, pos));
                }
                replica.selection.select_design_note(id);
                replica.design_notes.push(item);
            }

            for source in &capture.objects.documentation_shapes {
                let id = ids.allocate()?;
                let geometry =
                    transform_documentation_geometry(&source.geometry, transform, source.id)?;
                let mut item = DocumentationShape::new(id, geometry).map_err(|_| {
                    SchematicArrayError::InvalidGeometry {
                        object_id: source.id,
                    }
                })?;
                item.layer = source.layer;
                if let Some((other_id, _)) = radial_shape_geometries.iter().find(|(_, existing)| {
                    documentation_geometries_equivalent(existing, &item.geometry)
                }) {
                    return Err(SchematicArrayError::GeometryCollision {
                        object_id: id,
                        other_id: *other_id,
                    });
                }
                if radial_documentation {
                    radial_shape_geometries.push((id, item.geometry.clone()));
                }
                replica.selection.select_documentation_shape(id);
                replica.documentation_shapes.push(item);
            }

            replica.connections =
                remapped_captured_connections(&capture.connections, &component_map, &wire_map)?;
            let durable_connection_points = replica
                .connections
                .iter()
                .map(|connection| (connection.wire_id, connection.point_index))
                .collect::<HashSet<_>>();
            replica.connections.extend(
                resolved_replica_connections(
                    &replica.components,
                    &replica.wires,
                    terminal_points_for,
                )?
                .into_iter()
                .filter(|connection| {
                    !durable_connection_points
                        .contains(&(connection.wire_id, connection.point_index))
                }),
            );
            canonicalize_connections(&mut replica.connections);

            let replica_geometry =
                geometry_for_additions(&replica, terminal_points_for, component_bounds_for);
            validate_geometry_separation(&replica_geometry, &collision_index)?;
            collision_index.extend(&replica_geometry);
            additions.append(replica);
        }

        validate_candidate(self, &additions)?;
        let impact = additions.impact(members, electrical);
        let preview = additions.into_preview(impact);
        Ok(BuiltArray {
            preview,
            next_id: ids.next,
        })
    }

    fn validate_array_naming(
        &self,
        naming: &SchematicArrayNaming,
        members: usize,
    ) -> Result<(), SchematicArrayError> {
        let selected_ids = &self.selection.components;
        let mut occupied: HashSet<String> = self
            .components
            .iter()
            .filter(|component| !selected_ids.contains(&component.id))
            .filter(|component| !component.name.is_empty())
            .map(|component| component.name.to_ascii_lowercase())
            .collect();
        let mut selected: Vec<_> = self
            .components
            .iter()
            .filter(|component| selected_ids.contains(&component.id))
            .collect();
        selected.sort_by_key(|component| component.id);
        for component in selected {
            if component.name.is_empty() {
                continue;
            }
            for member in 0..members {
                let generated = naming
                    .value_for_source(&component.name, member)
                    .ok_or_else(|| SchematicArrayError::MissingNamingRange {
                        source: component.name.clone(),
                    })?;
                if member == 0 && generated != component.name {
                    return Err(SchematicArrayError::InvalidSourceName { name: generated });
                }
                let folded = generated.to_ascii_lowercase();
                if !occupied.insert(folded) {
                    return Err(SchematicArrayError::NameCollision { name: generated });
                }
            }
        }
        Ok(())
    }

    fn rebuild_component_counters_after_array(&mut self) {
        for component in &self.components {
            let prefix = component.kind.spice_prefix();
            if let Some(number) = component
                .name
                .strip_prefix(prefix)
                .and_then(|suffix| suffix.parse::<u32>().ok())
            {
                let counter = self.component_counters.entry(prefix).or_insert(0);
                *counter = (*counter).max(number);
            }
        }
    }
}

impl ArrayAdditions {
    fn append(&mut self, mut other: Self) {
        self.components.append(&mut other.components);
        self.wires.append(&mut other.wires);
        self.junctions.append(&mut other.junctions);
        self.buses.append(&mut other.buses);
        self.bus_taps.append(&mut other.bus_taps);
        self.net_labels.append(&mut other.net_labels);
        self.design_notes.append(&mut other.design_notes);
        self.documentation_shapes
            .append(&mut other.documentation_shapes);
        self.connections.append(&mut other.connections);
        self.selection.components.extend(other.selection.components);
        self.selection.wires.extend(other.selection.wires);
        self.selection.junctions.extend(other.selection.junctions);
        self.selection.buses.extend(other.selection.buses);
        self.selection.bus_taps.extend(other.selection.bus_taps);
        self.selection.net_labels.extend(other.selection.net_labels);
        self.selection
            .design_notes
            .extend(other.selection.design_notes);
        self.selection
            .documentation_shapes
            .extend(other.selection.documentation_shapes);
    }

    fn impact(&self, members: usize, electrical: bool) -> SchematicArrayImpact {
        SchematicArrayImpact {
            members,
            replicas: members.saturating_sub(1),
            components: self.components.len(),
            wires: self.wires.len(),
            junctions: self.junctions.len(),
            buses: self.buses.len(),
            bus_taps: self.bus_taps.len(),
            net_labels: self.net_labels.len(),
            design_notes: self.design_notes.len(),
            documentation_shapes: self.documentation_shapes.len(),
            connections: self.connections.len(),
            electrical,
        }
    }

    fn into_preview(self, impact: SchematicArrayImpact) -> SchematicArrayPreview {
        SchematicArrayPreview {
            impact,
            components: self.components,
            wires: self.wires,
            junctions: self.junctions,
            buses: self.buses,
            bus_taps: self.bus_taps,
            net_labels: self.net_labels,
            design_notes: self.design_notes,
            documentation_shapes: self.documentation_shapes,
            connections: self.connections,
            selection: self.selection,
        }
    }
}

#[derive(Clone, Copy)]
enum MemberTransform {
    Translate(Point),
    Rotate {
        center: Point,
        member_index: usize,
        member_count: usize,
    },
}

fn member_transform(
    plan: &SchematicArrayPlan,
    member_index: usize,
) -> Result<MemberTransform, SchematicArrayError> {
    match (plan.kind, plan.placement) {
        (SchematicArrayKind::Linear, SchematicArrayPlacement::Pitch(pitch)) => {
            let factor =
                i64::try_from(member_index).map_err(|_| SchematicArrayError::CoordinateOverflow)?;
            Ok(MemberTransform::Translate(scale_point(pitch, factor)?))
        }
        (SchematicArrayKind::Rectangular, SchematicArrayPlacement::Pitch(pitch)) => {
            let column = member_index % plan.count.columns();
            let row = member_index / plan.count.columns();
            Ok(MemberTransform::Translate(Point::new(
                scale_coordinate(pitch.x, column)?,
                scale_coordinate(pitch.y, row)?,
            )))
        }
        (SchematicArrayKind::RadialDocumentation, SchematicArrayPlacement::Center(center)) => {
            Ok(MemberTransform::Rotate {
                center,
                member_index,
                member_count: plan.count.member_count(),
            })
        }
        _ => Err(SchematicArrayError::PlacementKindMismatch),
    }
}

fn transform_point(point: Point, transform: MemberTransform) -> Result<Point, SchematicArrayError> {
    match transform {
        MemberTransform::Translate(delta) => checked_add_point(point, delta),
        MemberTransform::Rotate {
            center,
            member_index,
            member_count,
        } => rotate_point(point, center, member_index, member_count),
    }
}

fn checked_add_point(point: Point, delta: Point) -> Result<Point, SchematicArrayError> {
    Ok(Point::new(
        point
            .x
            .checked_add(delta.x)
            .ok_or(SchematicArrayError::CoordinateOverflow)?,
        point
            .y
            .checked_add(delta.y)
            .ok_or(SchematicArrayError::CoordinateOverflow)?,
    ))
}

fn scale_coordinate(value: i32, factor: usize) -> Result<i32, SchematicArrayError> {
    let factor = i64::try_from(factor).map_err(|_| SchematicArrayError::CoordinateOverflow)?;
    i32::try_from(i64::from(value) * factor).map_err(|_| SchematicArrayError::CoordinateOverflow)
}

fn scale_point(point: Point, factor: i64) -> Result<Point, SchematicArrayError> {
    Ok(Point::new(
        i32::try_from(i64::from(point.x) * factor)
            .map_err(|_| SchematicArrayError::CoordinateOverflow)?,
        i32::try_from(i64::from(point.y) * factor)
            .map_err(|_| SchematicArrayError::CoordinateOverflow)?,
    ))
}

fn rotate_point(
    point: Point,
    center: Point,
    member_index: usize,
    member_count: usize,
) -> Result<Point, SchematicArrayError> {
    if let Some(turns) = exact_quarter_turn(member_index, member_count) {
        return rotate_point_quarter_turn(point, center, turns);
    }
    let radians = std::f64::consts::TAU * member_index as f64 / member_count as f64;
    let x = f64::from(point.x) - f64::from(center.x);
    let y = f64::from(point.y) - f64::from(center.y);
    let rotated_x = f64::from(center.x) + x * radians.cos() - y * radians.sin();
    let rotated_y = f64::from(center.y) + x * radians.sin() + y * radians.cos();
    if !rotated_x.is_finite()
        || !rotated_y.is_finite()
        || rotated_x.round() < f64::from(i32::MIN)
        || rotated_x.round() > f64::from(i32::MAX)
        || rotated_y.round() < f64::from(i32::MIN)
        || rotated_y.round() > f64::from(i32::MAX)
    {
        return Err(SchematicArrayError::CoordinateOverflow);
    }
    Ok(Point::new(
        rotated_x.round() as i32,
        rotated_y.round() as i32,
    ))
}

fn exact_quarter_turn(member_index: usize, member_count: usize) -> Option<u8> {
    if member_count == 0 {
        return None;
    }
    let scaled = (member_index as u128) * 4;
    let members = member_count as u128;
    scaled
        .is_multiple_of(members)
        .then_some(((scaled / members) % 4) as u8)
}

fn rotate_point_quarter_turn(
    point: Point,
    center: Point,
    turns: u8,
) -> Result<Point, SchematicArrayError> {
    let x = i64::from(point.x) - i64::from(center.x);
    let y = i64::from(point.y) - i64::from(center.y);
    let (x, y) = match turns % 4 {
        0 => (x, y),
        1 => (-y, x),
        2 => (-x, -y),
        _ => (y, -x),
    };
    Ok(Point::new(
        i32::try_from(i64::from(center.x) + x)
            .map_err(|_| SchematicArrayError::CoordinateOverflow)?,
        i32::try_from(i64::from(center.y) + y)
            .map_err(|_| SchematicArrayError::CoordinateOverflow)?,
    ))
}

/// Transform one durable documentation primitive without silently changing
/// its represented geometry. Rectangles and callouts encode axis-aligned
/// boxes, so only exact quarter-turn rotations are representable. Other
/// angles fail closed instead of rebuilding a different axis-aligned object.
fn transform_documentation_geometry(
    geometry: &DocumentationShapeGeometry,
    transform: MemberTransform,
    object_id: u64,
) -> Result<DocumentationShapeGeometry, SchematicArrayError> {
    if matches!(
        geometry,
        DocumentationShapeGeometry::Rectangle { .. } | DocumentationShapeGeometry::Callout { .. }
    ) && matches!(
        transform,
        MemberTransform::Rotate {
            member_index,
            member_count,
            ..
        } if exact_quarter_turn(member_index, member_count).is_none()
    ) {
        return Err(SchematicArrayError::InvalidGeometry { object_id });
    }

    let points = geometry
        .points()
        .into_iter()
        .map(|point| transform_point(point, transform))
        .collect::<Result<Vec<_>, _>>()?;
    geometry_from_points(geometry.kind(), &points)
        .map_err(|_| SchematicArrayError::InvalidGeometry { object_id })
}

fn documentation_geometries_equivalent(
    left: &DocumentationShapeGeometry,
    right: &DocumentationShapeGeometry,
) -> bool {
    match (left, right) {
        (
            DocumentationShapeGeometry::Rectangle {
                first: left_first,
                opposite: left_opposite,
            },
            DocumentationShapeGeometry::Rectangle {
                first: right_first,
                opposite: right_opposite,
            },
        ) => {
            normalized_rectangle(*left_first, *left_opposite)
                == normalized_rectangle(*right_first, *right_opposite)
        }
        (
            DocumentationShapeGeometry::Line {
                start: left_start,
                end: left_end,
            },
            DocumentationShapeGeometry::Line {
                start: right_start,
                end: right_end,
            },
        ) => {
            (left_start == right_start && left_end == right_end)
                || (left_start == right_end && left_end == right_start)
        }
        (
            DocumentationShapeGeometry::Polygon { points: left },
            DocumentationShapeGeometry::Polygon { points: right },
        ) => polygons_equivalent(left, right),
        (
            DocumentationShapeGeometry::Arc {
                start: left_start,
                through: left_through,
                end: left_end,
            },
            DocumentationShapeGeometry::Arc {
                start: right_start,
                through: right_through,
                end: right_end,
            },
        ) => {
            (left_start == right_start && left_through == right_through && left_end == right_end)
                || (left_start == right_end
                    && left_through == right_through
                    && left_end == right_start)
        }
        (
            DocumentationShapeGeometry::Callout {
                tip: left_tip,
                elbow: left_elbow,
                box_corner: left_corner,
            },
            DocumentationShapeGeometry::Callout {
                tip: right_tip,
                elbow: right_elbow,
                box_corner: right_corner,
            },
        ) => left_tip == right_tip && left_elbow == right_elbow && left_corner == right_corner,
        _ => false,
    }
}

fn normalized_rectangle(first: Point, opposite: Point) -> (i32, i32, i32, i32) {
    (
        first.x.min(opposite.x),
        first.y.min(opposite.y),
        first.x.max(opposite.x),
        first.y.max(opposite.y),
    )
}

fn polygons_equivalent(left: &[Point], right: &[Point]) -> bool {
    if left.len() != right.len() || left.is_empty() {
        return false;
    }
    (0..right.len()).any(|offset| {
        left.iter()
            .enumerate()
            .all(|(index, point)| *point == right[(index + offset) % right.len()])
            || left
                .iter()
                .enumerate()
                .all(|(index, point)| *point == right[(offset + right.len() - index) % right.len()])
    })
}

fn validate_live_array_selection(state: &SchematicState) -> Result<(), SchematicArrayError> {
    if !state.selection.wire_segments.is_empty() || !state.selection.wire_vertices.is_empty() {
        return Err(SchematicArrayError::PartialSelection);
    }
    let stale = state
        .selection
        .components
        .iter()
        .any(|id| !state.components.iter().any(|item| item.id == *id))
        || state
            .selection
            .wires
            .iter()
            .any(|id| !state.wires.iter().any(|item| item.id == *id))
        || state
            .selection
            .buses
            .iter()
            .any(|id| !state.buses.iter().any(|item| item.id == *id))
        || state
            .selection
            .bus_taps
            .iter()
            .any(|id| !state.bus_taps.iter().any(|item| item.id == *id))
        || state
            .selection
            .net_labels
            .iter()
            .any(|id| !state.net_labels.iter().any(|item| item.id == *id))
        || state
            .selection
            .design_notes
            .iter()
            .any(|id| !state.design_notes.iter().any(|item| item.id == *id))
        || state
            .selection
            .documentation_shapes
            .iter()
            .any(|id| !state.documentation_shapes.iter().any(|item| item.id == *id))
        || state
            .selection
            .junctions
            .iter()
            .any(|selection| !state.junctions.iter().any(|item| item.pos == selection.pos));
    if stale {
        return Err(SchematicArrayError::StaleSelection {
            object_id: first_stale_selection_id(state),
        });
    }
    if !state.has_live_array_selection() {
        return Err(SchematicArrayError::EmptySelection);
    }
    Ok(())
}

/// Validate the exact default complete-object capture as a borrowed view.
/// This is intentionally kept allocation-light because command availability
/// and the open dialog call it every frame. Only compact identity/point sets
/// are materialized; schematic objects and strings are never cloned.
fn validate_array_source_selection_direct(
    state: &SchematicState,
) -> Result<(), SchematicArrayError> {
    let selected_terminals: HashSet<Point> = state
        .components
        .iter()
        .filter(|component| state.selection.has_component(component.id))
        .flat_map(|component| {
            component
                .terminal_positions()
                .into_iter()
                .map(|(_, point)| point)
        })
        .collect();
    let selected_owned_wire_points: HashSet<(u64, usize)> = state
        .connections
        .iter()
        .filter(|connection| {
            state
                .selection
                .components
                .contains(&connection.component_id)
        })
        .map(|connection| (connection.wire_id, connection.point_index))
        .collect();

    let mut captured_wire_ids = HashSet::new();
    for wire in &state.wires {
        let explicitly_selected = state.selection.has_wire(wire.id);
        let implicitly_selected = if wire.points.len() < 2 {
            false
        } else {
            let last = wire.points.len() - 1;
            let geometrically_owned = selected_terminals.contains(&wire.points[0])
                && selected_terminals.contains(&wire.points[last]);
            let durably_owned = selected_owned_wire_points.contains(&(wire.id, 0))
                && selected_owned_wire_points.contains(&(wire.id, last));
            geometrically_owned || durably_owned
        };
        if !explicitly_selected && !implicitly_selected {
            continue;
        }
        if wire.points.len() < 2 || wire.points.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(SchematicArrayError::InvalidGeometry { object_id: wire.id });
        }
        captured_wire_ids.insert(wire.id);
    }

    let selected_tap_owner_ids: HashSet<u64> = state
        .bus_taps
        .iter()
        .filter(|tap| state.selection.has_bus_tap(tap.id))
        .map(|tap| tap.bus_id)
        .collect();
    for bus in &state.buses {
        if state.selection.has_bus(bus.id) || selected_tap_owner_ids.contains(&bus.id) {
            bus.validate()
                .map_err(|_| SchematicArrayError::InvalidGeometry { object_id: bus.id })?;
        }
    }
    for tap in &state.bus_taps {
        if !state.selection.has_bus_tap(tap.id) && !state.selection.has_bus(tap.bus_id) {
            continue;
        }
        let bus = state
            .buses
            .iter()
            .find(|bus| bus.id == tap.bus_id)
            .ok_or(SchematicArrayError::InvalidBusTap { tap_id: tap.id })?;
        tap.validate_against_bus(bus)
            .map_err(|_| SchematicArrayError::InvalidBusTap { tap_id: tap.id })?;
    }
    for note in &state.design_notes {
        if state.selection.has_design_note(note.id) {
            note.validate()
                .map_err(|_| SchematicArrayError::InvalidGeometry { object_id: note.id })?;
        }
    }
    for shape in &state.documentation_shapes {
        if state.selection.has_documentation_shape(shape.id) {
            shape
                .validate()
                .map_err(|_| SchematicArrayError::InvalidGeometry {
                    object_id: shape.id,
                })?;
        }
    }

    let mut owned_points = HashSet::new();
    for connection in &state.connections {
        if !captured_wire_ids.contains(&connection.wire_id)
            || !state
                .selection
                .components
                .contains(&connection.component_id)
        {
            continue;
        }
        let Some(wire) = state
            .wires
            .iter()
            .find(|wire| wire.id == connection.wire_id)
        else {
            return Err(SchematicArrayError::InvalidConnection(connection.wire_id));
        };
        if connection.point_index >= wire.points.len()
            || !owned_points.insert((connection.wire_id, connection.point_index))
        {
            return Err(SchematicArrayError::InvalidConnection(connection.wire_id));
        }
    }
    Ok(())
}

fn validate_capture(capture: &ClipboardData) -> Result<(), SchematicArrayError> {
    for wire in &capture.wires {
        if wire.points.len() < 2 || wire.points.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(SchematicArrayError::InvalidGeometry { object_id: wire.id });
        }
    }
    for bus in &capture.buses {
        bus.validate()
            .map_err(|_| SchematicArrayError::InvalidGeometry { object_id: bus.id })?;
    }
    for tap in &capture.bus_taps {
        let bus = capture
            .buses
            .iter()
            .find(|bus| bus.id == tap.bus_id)
            .ok_or(SchematicArrayError::InvalidBusTap { tap_id: tap.id })?;
        tap.validate_against_bus(bus)
            .map_err(|_| SchematicArrayError::InvalidBusTap { tap_id: tap.id })?;
    }
    for note in &capture.design_notes {
        note.validate()
            .map_err(|_| SchematicArrayError::InvalidGeometry { object_id: note.id })?;
    }
    for shape in &capture.documentation_shapes {
        shape
            .validate()
            .map_err(|_| SchematicArrayError::InvalidGeometry {
                object_id: shape.id,
            })?;
    }
    if capture.components.is_empty()
        && capture.wires.is_empty()
        && capture.buses.is_empty()
        && capture.bus_taps.is_empty()
        && capture.net_labels.is_empty()
        && capture.design_notes.is_empty()
        && capture.documentation_shapes.is_empty()
    {
        return Err(SchematicArrayError::UnsupportedSelection);
    }
    Ok(())
}

fn validate_captured_array_selection(
    capture: &CapturedArraySelection,
) -> Result<(), SchematicArrayError> {
    validate_capture(&capture.objects)?;
    let mut owned_points = HashSet::new();
    for connection in &capture.connections {
        let Some(wire) = capture
            .objects
            .wires
            .iter()
            .find(|wire| wire.id == connection.wire_id)
        else {
            return Err(SchematicArrayError::InvalidConnection(connection.wire_id));
        };
        if connection.point_index >= wire.points.len()
            || !capture
                .objects
                .components
                .iter()
                .any(|component| component.id == connection.component_id)
            || !owned_points.insert((connection.wire_id, connection.point_index))
        {
            return Err(SchematicArrayError::InvalidConnection(connection.wire_id));
        }
    }
    Ok(())
}

fn validate_naming_coverage(
    capture: &ClipboardData,
    naming: &SchematicArrayNaming,
    selection: &Selection,
) -> Result<(), SchematicArrayError> {
    let mut required = HashSet::new();
    for component in &capture.components {
        if !component.name.is_empty() {
            required.insert(component.name.clone());
        }
    }
    for label in &capture.net_labels {
        if scalar_bus_name(&label.name).is_some() {
            required.insert(label.name.clone());
        }
    }
    for tap in &capture.bus_taps {
        if selection.has_bus_tap(tap.id) && tap.slice.is_scalar() {
            required.insert(tap.slice.to_string());
        }
    }
    for source in &required {
        if !naming.contains_source(source) {
            return Err(SchematicArrayError::MissingNamingRange {
                source: source.clone(),
            });
        }
    }
    if let Some(source) = naming.sources().find(|source| !required.contains(*source)) {
        return Err(SchematicArrayError::UnmatchedNamingRange {
            source: source.to_owned(),
        });
    }
    Ok(())
}

fn capture_array_selection(
    state: &SchematicState,
    terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
) -> CapturedArraySelection {
    let mut objects = state.capture_complete_selection_resolved(terminal_points_for);
    let selected_components = &state.selection.components;
    let captured_wire_ids: HashSet<u64> = objects.wires.iter().map(|wire| wire.id).collect();
    for wire in &state.wires {
        if captured_wire_ids.contains(&wire.id) || wire.points.len() < 2 {
            continue;
        }
        let last = wire.points.len() - 1;
        let owns_start = state.connections.iter().any(|connection| {
            connection.wire_id == wire.id
                && connection.point_index == 0
                && selected_components.contains(&connection.component_id)
        });
        let owns_end = state.connections.iter().any(|connection| {
            connection.wire_id == wire.id
                && connection.point_index == last
                && selected_components.contains(&connection.component_id)
        });
        if owns_start && owns_end {
            objects.wires.push(wire.clone());
        }
    }

    let captured_wire_ids: HashSet<u64> = objects.wires.iter().map(|wire| wire.id).collect();
    let captured_component_ids: HashSet<u64> = objects
        .components
        .iter()
        .map(|component| component.id)
        .collect();
    for junction in &state.junctions {
        if objects.junctions.contains(&junction.pos)
            || !objects
                .wires
                .iter()
                .any(|wire| wire.contains_point(junction.pos))
        {
            continue;
        }
        objects.junctions.push(junction.pos);
    }
    objects.junctions.sort_by_key(|point| (point.x, point.y));
    objects.junctions.dedup();

    let mut connections: Vec<_> = state
        .connections
        .iter()
        .filter(|connection| {
            captured_wire_ids.contains(&connection.wire_id)
                && captured_component_ids.contains(&connection.component_id)
        })
        .cloned()
        .collect();
    canonicalize_connections(&mut connections);
    CapturedArraySelection {
        objects,
        connections,
    }
}

fn validate_generation_budget(
    capture: &CapturedArraySelection,
    members: usize,
) -> Result<(), SchematicArrayError> {
    let replicas = members.saturating_sub(1);
    let source_objects = capture
        .objects
        .count()
        .checked_add(capture.connections.len())
        .ok_or(SchematicArrayError::CountOverflow)?;
    let generated_objects = source_objects
        .checked_mul(replicas)
        .ok_or(SchematicArrayError::CountOverflow)?;
    if generated_objects > MAX_ARRAY_GENERATED_OBJECTS {
        return Err(SchematicArrayError::GeneratedObjectBudgetExceeded {
            requested: generated_objects,
            maximum: MAX_ARRAY_GENERATED_OBJECTS,
        });
    }

    let conductor_segments = capture
        .objects
        .wires
        .iter()
        .map(|wire| wire.points.len().saturating_sub(1))
        .chain(
            capture
                .objects
                .buses
                .iter()
                .map(|bus| bus.points.len().saturating_sub(1)),
        )
        .chain(capture.objects.bus_taps.iter().map(|_| 1usize))
        .try_fold(0usize, usize::checked_add)
        .ok_or(SchematicArrayError::CountOverflow)?;
    let documentation_segments = capture
        .objects
        .documentation_shapes
        .iter()
        .map(|shape| match &shape.geometry {
            DocumentationShapeGeometry::Rectangle { .. } => 4,
            DocumentationShapeGeometry::Line { .. } => 1,
            DocumentationShapeGeometry::Polygon { points } => points.len(),
            DocumentationShapeGeometry::Arc { .. } => 3,
            DocumentationShapeGeometry::Callout { .. } => 3,
        })
        .try_fold(0usize, usize::checked_add)
        .ok_or(SchematicArrayError::CountOverflow)?;
    let generated_segments = conductor_segments
        .checked_add(documentation_segments)
        .and_then(|segments| segments.checked_mul(replicas))
        .ok_or(SchematicArrayError::CountOverflow)?;
    if generated_segments > MAX_ARRAY_GENERATED_SEGMENTS {
        return Err(SchematicArrayError::GeneratedSegmentBudgetExceeded {
            requested: generated_segments,
            maximum: MAX_ARRAY_GENERATED_SEGMENTS,
        });
    }
    Ok(())
}

fn closed_source_selection(capture: &ClipboardData) -> Selection {
    let mut selection = Selection::default();
    for component in &capture.components {
        selection.select_component(component.id);
    }
    for wire in &capture.wires {
        selection.select_wire(wire.id);
    }
    for point in &capture.junctions {
        selection.select_junction(*point);
    }
    for bus in &capture.buses {
        selection.select_bus(bus.id);
    }
    for tap in &capture.bus_taps {
        selection.select_bus_tap(tap.id);
    }
    for label in &capture.net_labels {
        selection.select_net_label(label.id);
    }
    for note in &capture.design_notes {
        selection.select_design_note(note.id);
    }
    for shape in &capture.documentation_shapes {
        selection.select_documentation_shape(shape.id);
    }
    selection
}

fn remapped_captured_connections(
    source: &[WireConnection],
    component_map: &HashMap<u64, u64>,
    wire_map: &HashMap<u64, u64>,
) -> Result<Vec<WireConnection>, SchematicArrayError> {
    source
        .iter()
        .map(|connection| {
            Ok(WireConnection::new(
                *wire_map
                    .get(&connection.wire_id)
                    .ok_or(SchematicArrayError::InvalidConnection(connection.wire_id))?,
                connection.point_index,
                *component_map
                    .get(&connection.component_id)
                    .ok_or(SchematicArrayError::InvalidConnection(connection.wire_id))?,
                connection.terminal_name.clone(),
            ))
        })
        .collect()
}

fn canonicalize_connections(connections: &mut Vec<WireConnection>) {
    connections.sort_by(|left, right| {
        (
            left.wire_id,
            left.point_index,
            left.component_id,
            &left.terminal_name,
        )
            .cmp(&(
                right.wire_id,
                right.point_index,
                right.component_id,
                &right.terminal_name,
            ))
    });
    connections.dedup();
}

fn capture_is_electrical(capture: &ClipboardData) -> bool {
    !capture.components.is_empty()
        || !capture.wires.is_empty()
        || !capture.junctions.is_empty()
        || !capture.buses.is_empty()
        || !capture.bus_taps.is_empty()
        || !capture.net_labels.is_empty()
}

fn first_electrical_object_id(capture: &ClipboardData) -> u64 {
    capture
        .components
        .first()
        .map(|item| item.id)
        .or_else(|| capture.wires.first().map(|item| item.id))
        .or_else(|| capture.buses.first().map(|item| item.id))
        .or_else(|| capture.bus_taps.first().map(|item| item.id))
        .or_else(|| capture.net_labels.first().map(|item| item.id))
        .unwrap_or(0)
}

fn first_stale_selection_id(state: &SchematicState) -> u64 {
    state
        .selection
        .components
        .iter()
        .copied()
        .find(|id| !state.components.iter().any(|item| item.id == *id))
        .or_else(|| {
            state
                .selection
                .wires
                .iter()
                .copied()
                .find(|id| !state.wires.iter().any(|item| item.id == *id))
        })
        .or_else(|| {
            state
                .selection
                .buses
                .iter()
                .copied()
                .find(|id| !state.buses.iter().any(|item| item.id == *id))
        })
        .or_else(|| {
            state
                .selection
                .bus_taps
                .iter()
                .copied()
                .find(|id| !state.bus_taps.iter().any(|item| item.id == *id))
        })
        .or_else(|| {
            state
                .selection
                .net_labels
                .iter()
                .copied()
                .find(|id| !state.net_labels.iter().any(|item| item.id == *id))
        })
        .or_else(|| {
            state
                .selection
                .design_notes
                .iter()
                .copied()
                .find(|id| !state.design_notes.iter().any(|item| item.id == *id))
        })
        .or_else(|| {
            state
                .selection
                .documentation_shapes
                .iter()
                .copied()
                .find(|id| !state.documentation_shapes.iter().any(|item| item.id == *id))
        })
        .or_else(|| {
            state
                .selection
                .junctions
                .iter()
                .find(|selection| !state.junctions.iter().any(|item| item.pos == selection.pos))
                .map(|_| 0)
        })
        .unwrap_or(0)
}

fn resolved_replica_connections(
    components: &[Component],
    wires: &[Wire],
    terminal_points_for: &mut impl FnMut(&Component) -> Vec<(String, Point)>,
) -> Result<Vec<WireConnection>, SchematicArrayError> {
    let mut terminals: HashMap<Point, Vec<(u64, String)>> = HashMap::new();
    for component in components {
        for (name, point) in terminal_points_for(component) {
            terminals
                .entry(point)
                .or_default()
                .push((component.id, name));
        }
    }
    let mut connections = Vec::new();
    for wire in wires {
        for point_index in [0, wire.points.len().saturating_sub(1)] {
            let Some(point) = wire.points.get(point_index).copied() else {
                continue;
            };
            let Some(matches) = terminals.get(&point) else {
                continue;
            };
            if matches.len() != 1 {
                return Err(SchematicArrayError::AmbiguousTerminalContact(point));
            }
            let (component_id, terminal_name) = &matches[0];
            connections.push(WireConnection::new(
                wire.id,
                point_index,
                *component_id,
                terminal_name.clone(),
            ));
        }
    }
    connections.sort_by(|left, right| {
        (
            left.wire_id,
            left.point_index,
            left.component_id,
            &left.terminal_name,
        )
            .cmp(&(
                right.wire_id,
                right.point_index,
                right.component_id,
                &right.terminal_name,
            ))
    });
    connections.dedup();
    Ok(connections)
}

fn geometry_for_state(
    state: &SchematicState,
    terminal_points_for: &mut impl FnMut(&Component) -> Vec<(String, Point)>,
    component_bounds_for: &mut impl FnMut(&Component) -> (i32, i32, i32, i32),
) -> ReplicaGeometry {
    geometry_from_parts(
        &state.components,
        &state.wires,
        &state.buses,
        &state.bus_taps,
        &state.net_labels,
        &state.junctions,
        terminal_points_for,
        component_bounds_for,
    )
}

fn geometry_for_additions(
    additions: &ArrayAdditions,
    terminal_points_for: &mut impl FnMut(&Component) -> Vec<(String, Point)>,
    component_bounds_for: &mut impl FnMut(&Component) -> (i32, i32, i32, i32),
) -> ReplicaGeometry {
    geometry_from_parts(
        &additions.components,
        &additions.wires,
        &additions.buses,
        &additions.bus_taps,
        &additions.net_labels,
        &additions.junctions,
        terminal_points_for,
        component_bounds_for,
    )
}

#[allow(clippy::too_many_arguments)]
fn geometry_from_parts(
    components: &[Component],
    wires: &[Wire],
    buses: &[Bus],
    taps: &[BusTap],
    labels: &[NetLabel],
    junctions: &[Junction],
    terminal_points_for: &mut impl FnMut(&Component) -> Vec<(String, Point)>,
    component_bounds_for: &mut impl FnMut(&Component) -> (i32, i32, i32, i32),
) -> ReplicaGeometry {
    let mut result = ReplicaGeometry::default();
    for component in components {
        let (x1, y1, x2, y2) = component_bounds_for(component);
        result.component_bounds.push((
            component.id,
            (x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2)),
        ));
        result.terminals.extend(
            terminal_points_for(component)
                .into_iter()
                .map(|(_, point)| (component.id, point)),
        );
    }
    for wire in wires {
        result.conductors.extend(
            wire.points
                .windows(2)
                .map(|pair| (wire.id, pair[0], pair[1])),
        );
    }
    for bus in buses {
        result
            .conductors
            .extend(bus.points.windows(2).map(|pair| (bus.id, pair[0], pair[1])));
    }
    result.conductors.extend(
        taps.iter()
            .map(|tap| (tap.id, tap.bus_point, tap.connection_point)),
    );
    result
        .anchors
        .extend(labels.iter().map(|label| (label.id, label.pos)));
    result
        .anchors
        .extend(junctions.iter().map(|junction| (junction.id, junction.pos)));
    result
}

fn validate_geometry_separation(
    incoming: &ReplicaGeometry,
    outside: &GeometryBroadPhase,
) -> Result<(), SchematicArrayError> {
    for &(id, bounds) in &incoming.component_bounds {
        for primitive in outside.query(bounds) {
            match primitive {
                CollisionPrimitive::Body {
                    id: other_id,
                    bounds: other_bounds,
                } if rectangles_overlap(bounds, other_bounds) => {
                    return Err(SchematicArrayError::GeometryCollision {
                        object_id: id,
                        other_id,
                    });
                }
                CollisionPrimitive::Conductor {
                    id: other_id,
                    start,
                    end,
                } if segment_enters_open_rect(start, end, bounds) => {
                    return Err(SchematicArrayError::GeometryCollision {
                        object_id: id,
                        other_id,
                    });
                }
                CollisionPrimitive::Terminal {
                    id: other_id,
                    point,
                }
                | CollisionPrimitive::Anchor {
                    id: other_id,
                    point,
                } if point_in_open_rect(point, bounds) => {
                    return Err(SchematicArrayError::GeometryCollision {
                        object_id: id,
                        other_id,
                    });
                }
                _ => {}
            }
        }
    }
    for &(id, start, end) in &incoming.conductors {
        let bounds = segment_bounds(start, end);
        for primitive in outside.query(bounds) {
            match primitive {
                CollisionPrimitive::Conductor {
                    id: other_id,
                    start: other_start,
                    end: other_end,
                } if segments_intersect(start, end, other_start, other_end) => {
                    return Err(SchematicArrayError::UnintendedContact {
                        object_id: id,
                        other_id,
                        point: segment_contact_point(start, end, other_start, other_end)
                            .unwrap_or(start),
                    });
                }
                CollisionPrimitive::Terminal {
                    id: other_id,
                    point,
                }
                | CollisionPrimitive::Anchor {
                    id: other_id,
                    point,
                } if point_on_segment(point, start, end) => {
                    return Err(SchematicArrayError::UnintendedContact {
                        object_id: id,
                        other_id,
                        point,
                    });
                }
                CollisionPrimitive::Body {
                    id: other_id,
                    bounds,
                } if segment_enters_open_rect(start, end, bounds) => {
                    return Err(SchematicArrayError::GeometryCollision {
                        object_id: id,
                        other_id,
                    });
                }
                _ => {}
            }
        }
    }
    for &(component_id, terminal) in &incoming.terminals {
        for primitive in outside.query(point_bounds(terminal)) {
            match primitive {
                CollisionPrimitive::Conductor {
                    id: other_id,
                    start,
                    end,
                } if point_on_segment(terminal, start, end) => {
                    return Err(SchematicArrayError::UnintendedContact {
                        object_id: component_id,
                        other_id,
                        point: terminal,
                    });
                }
                CollisionPrimitive::Terminal {
                    id: other_id,
                    point,
                }
                | CollisionPrimitive::Anchor {
                    id: other_id,
                    point,
                } if terminal == point => {
                    return Err(SchematicArrayError::UnintendedContact {
                        object_id: component_id,
                        other_id,
                        point: terminal,
                    });
                }
                CollisionPrimitive::Body {
                    id: other_id,
                    bounds,
                } if point_in_open_rect(terminal, bounds) => {
                    return Err(SchematicArrayError::GeometryCollision {
                        object_id: component_id,
                        other_id,
                    });
                }
                _ => {}
            }
        }
    }
    for &(id, anchor) in &incoming.anchors {
        for primitive in outside.query(point_bounds(anchor)) {
            match primitive {
                CollisionPrimitive::Conductor {
                    id: other_id,
                    start,
                    end,
                } if point_on_segment(anchor, start, end) => {
                    return Err(SchematicArrayError::UnintendedContact {
                        object_id: id,
                        other_id,
                        point: anchor,
                    });
                }
                CollisionPrimitive::Terminal {
                    id: other_id,
                    point,
                }
                | CollisionPrimitive::Anchor {
                    id: other_id,
                    point,
                } if anchor == point => {
                    return Err(SchematicArrayError::UnintendedContact {
                        object_id: id,
                        other_id,
                        point: anchor,
                    });
                }
                CollisionPrimitive::Body {
                    id: other_id,
                    bounds,
                } if point_in_open_rect(anchor, bounds) => {
                    return Err(SchematicArrayError::GeometryCollision {
                        object_id: id,
                        other_id,
                    });
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn live_ids(state: &SchematicState) -> impl Iterator<Item = u64> + '_ {
    state
        .components
        .iter()
        .map(|item| item.id)
        .chain(state.wires.iter().map(|item| item.id))
        .chain(state.junctions.iter().map(|item| item.id))
        .chain(state.buses.iter().map(|item| item.id))
        .chain(state.bus_taps.iter().map(|item| item.id))
        .chain(state.net_labels.iter().map(|item| item.id))
        .chain(state.design_notes.iter().map(|item| item.id))
        .chain(state.documentation_shapes.iter().map(|item| item.id))
}

fn validate_candidate(
    state: &SchematicState,
    additions: &ArrayAdditions,
) -> Result<(), SchematicArrayError> {
    let mut ids: HashSet<u64> = live_ids(state).collect();
    for id in additions
        .components
        .iter()
        .map(|item| item.id)
        .chain(additions.wires.iter().map(|item| item.id))
        .chain(additions.junctions.iter().map(|item| item.id))
        .chain(additions.buses.iter().map(|item| item.id))
        .chain(additions.bus_taps.iter().map(|item| item.id))
        .chain(additions.net_labels.iter().map(|item| item.id))
        .chain(additions.design_notes.iter().map(|item| item.id))
        .chain(additions.documentation_shapes.iter().map(|item| item.id))
    {
        if id == 0 || !ids.insert(id) {
            return Err(SchematicArrayError::DuplicateIdentity(id));
        }
    }
    let mut names: HashSet<String> = state
        .components
        .iter()
        .filter(|component| !component.name.is_empty())
        .map(|component| component.name.to_ascii_lowercase())
        .collect();
    for component in &additions.components {
        if !component.name.is_empty() && !names.insert(component.name.to_ascii_lowercase()) {
            return Err(SchematicArrayError::NameCollision {
                name: component.name.clone(),
            });
        }
    }
    for connection in &additions.connections {
        let Some(wire) = additions
            .wires
            .iter()
            .find(|wire| wire.id == connection.wire_id)
            .or_else(|| {
                state
                    .wires
                    .iter()
                    .find(|wire| wire.id == connection.wire_id)
            })
        else {
            return Err(SchematicArrayError::InvalidConnection(connection.wire_id));
        };
        if connection.point_index >= wire.points.len()
            || !additions
                .components
                .iter()
                .chain(state.components.iter())
                .any(|component| component.id == connection.component_id)
        {
            return Err(SchematicArrayError::InvalidConnection(connection.wire_id));
        }
    }
    Ok(())
}

fn rectangles_overlap(
    (left_min_x, left_min_y, left_max_x, left_max_y): (i32, i32, i32, i32),
    (right_min_x, right_min_y, right_max_x, right_max_y): (i32, i32, i32, i32),
) -> bool {
    left_min_x < right_max_x
        && left_max_x > right_min_x
        && left_min_y < right_max_y
        && left_max_y > right_min_y
}

fn primitive_bounds(primitive: CollisionPrimitive) -> (i32, i32, i32, i32) {
    match primitive {
        CollisionPrimitive::Body { bounds, .. } => bounds,
        CollisionPrimitive::Terminal { point, .. } | CollisionPrimitive::Anchor { point, .. } => {
            point_bounds(point)
        }
        CollisionPrimitive::Conductor { start, end, .. } => segment_bounds(start, end),
    }
}

fn point_bounds(point: Point) -> (i32, i32, i32, i32) {
    (point.x, point.y, point.x, point.y)
}

fn segment_bounds(start: Point, end: Point) -> (i32, i32, i32, i32) {
    (
        start.x.min(end.x),
        start.y.min(end.y),
        start.x.max(end.x),
        start.y.max(end.y),
    )
}

fn covered_cells(
    (min_x, min_y, max_x, max_y): (i32, i32, i32, i32),
    maximum: usize,
) -> Option<Vec<(i32, i32)>> {
    let min_cell_x = min_x.div_euclid(GeometryBroadPhase::CELL_SIZE);
    let max_cell_x = max_x.div_euclid(GeometryBroadPhase::CELL_SIZE);
    let min_cell_y = min_y.div_euclid(GeometryBroadPhase::CELL_SIZE);
    let max_cell_y = max_y.div_euclid(GeometryBroadPhase::CELL_SIZE);
    let width = i64::from(max_cell_x) - i64::from(min_cell_x) + 1;
    let height = i64::from(max_cell_y) - i64::from(min_cell_y) + 1;
    let count = usize::try_from(width.checked_mul(height)?).ok()?;
    if count > maximum {
        return None;
    }
    let mut cells = Vec::with_capacity(count);
    for x in min_cell_x..=max_cell_x {
        for y in min_cell_y..=max_cell_y {
            cells.push((x, y));
        }
    }
    Some(cells)
}

fn point_in_open_rect(point: Point, (min_x, min_y, max_x, max_y): (i32, i32, i32, i32)) -> bool {
    point.x > min_x && point.x < max_x && point.y > min_y && point.y < max_y
}

fn segment_enters_open_rect(
    start: Point,
    end: Point,
    (min_x, min_y, max_x, max_y): (i32, i32, i32, i32),
) -> bool {
    if min_x >= max_x || min_y >= max_y {
        return false;
    }
    let start_x = f64::from(start.x);
    let start_y = f64::from(start.y);
    let dx = f64::from(end.x) - start_x;
    let dy = f64::from(end.y) - start_y;
    let mut enter: f64 = 0.0;
    let mut exit: f64 = 1.0;
    for (origin, direction, lower, upper) in [
        (start_x, dx, f64::from(min_x), f64::from(max_x)),
        (start_y, dy, f64::from(min_y), f64::from(max_y)),
    ] {
        if direction == 0.0 {
            if origin <= lower || origin >= upper {
                return false;
            }
            continue;
        }
        let first = (lower - origin) / direction;
        let second = (upper - origin) / direction;
        enter = enter.max(first.min(second));
        exit = exit.min(first.max(second));
        if enter > exit {
            return false;
        }
    }
    let sample = ((enter.max(0.0) + exit.min(1.0)) / 2.0).clamp(0.0, 1.0);
    let x = start_x + sample * dx;
    let y = start_y + sample * dy;
    x > f64::from(min_x) && x < f64::from(max_x) && y > f64::from(min_y) && y < f64::from(max_y)
}

fn point_on_segment(point: Point, start: Point, end: Point) -> bool {
    let cross = (i128::from(point.x) - i128::from(start.x))
        * (i128::from(end.y) - i128::from(start.y))
        - (i128::from(point.y) - i128::from(start.y)) * (i128::from(end.x) - i128::from(start.x));
    cross == 0
        && point.x >= start.x.min(end.x)
        && point.x <= start.x.max(end.x)
        && point.y >= start.y.min(end.y)
        && point.y <= start.y.max(end.y)
}

fn segments_intersect(
    left_start: Point,
    left_end: Point,
    right_start: Point,
    right_end: Point,
) -> bool {
    let orientation = |a: Point, b: Point, c: Point| {
        (i128::from(b.x) - i128::from(a.x)) * (i128::from(c.y) - i128::from(a.y))
            - (i128::from(b.y) - i128::from(a.y)) * (i128::from(c.x) - i128::from(a.x))
    };
    let o1 = orientation(left_start, left_end, right_start);
    let o2 = orientation(left_start, left_end, right_end);
    let o3 = orientation(right_start, right_end, left_start);
    let o4 = orientation(right_start, right_end, left_end);
    (o1 == 0 && point_on_segment(right_start, left_start, left_end))
        || (o2 == 0 && point_on_segment(right_end, left_start, left_end))
        || (o3 == 0 && point_on_segment(left_start, right_start, right_end))
        || (o4 == 0 && point_on_segment(left_end, right_start, right_end))
        || ((o1 > 0) != (o2 > 0) && (o3 > 0) != (o4 > 0))
}

fn segment_contact_point(
    left_start: Point,
    left_end: Point,
    right_start: Point,
    right_end: Point,
) -> Option<Point> {
    [left_start, left_end]
        .into_iter()
        .find(|point| point_on_segment(*point, right_start, right_end))
        .or_else(|| {
            [right_start, right_end]
                .into_iter()
                .find(|point| point_on_segment(*point, left_start, left_end))
        })
        .or_else(|| {
            let x1 = f64::from(left_start.x);
            let y1 = f64::from(left_start.y);
            let x2 = f64::from(left_end.x);
            let y2 = f64::from(left_end.y);
            let x3 = f64::from(right_start.x);
            let y3 = f64::from(right_start.y);
            let x4 = f64::from(right_end.x);
            let y4 = f64::from(right_end.y);
            let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
            if denominator == 0.0 {
                return None;
            }
            let left_cross = x1 * y2 - y1 * x2;
            let right_cross = x3 * y4 - y3 * x4;
            let x = (left_cross * (x3 - x4) - (x1 - x2) * right_cross) / denominator;
            let y = (left_cross * (y3 - y4) - (y1 - y2) * right_cross) / denominator;
            Some(Point::new(x.round() as i32, y.round() as i32))
        })
}

fn find_collision_free_reference_stride(
    atoms: &[SchematicArrayNameAtom],
    members: usize,
    occupied: &HashSet<String>,
) -> Result<u64, SchematicArrayError> {
    let search_limit = occupied
        .len()
        .saturating_add(atoms.len())
        .saturating_add(1)
        .saturating_mul(members.max(2))
        .max(1);
    for stride in 1..=u64::try_from(search_limit).unwrap_or(u64::MAX) {
        let mut generated = HashSet::with_capacity(atoms.len().saturating_mul(members));
        let mut valid = true;
        'sources: for atom in atoms {
            for member in 0..members {
                let Some(offset) = u64::try_from(member)
                    .ok()
                    .and_then(|member| member.checked_mul(stride))
                else {
                    valid = false;
                    break 'sources;
                };
                let Some(number) = atom.numeric_value().checked_add(offset) else {
                    valid = false;
                    break 'sources;
                };
                let name = atom
                    .with_numeric_value(number)
                    .to_string()
                    .to_ascii_lowercase();
                if occupied.contains(&name) || !generated.insert(name) {
                    valid = false;
                    break 'sources;
                }
            }
        }
        if valid {
            return Ok(stride);
        }
    }
    Err(SchematicArrayError::InvalidSourceName {
        name: atoms.first().map_or_else(String::new, ToString::to_string),
    })
}

fn checked_bus_sequence_end(
    start: u32,
    members: usize,
    stride: u64,
) -> Result<u32, SchematicArrayError> {
    let intervals =
        u64::try_from(members.saturating_sub(1)).map_err(|_| SchematicArrayError::CountOverflow)?;
    let end = u64::from(start)
        .checked_add(
            intervals
                .checked_mul(stride)
                .ok_or(SchematicArrayError::CountOverflow)?,
        )
        .ok_or(SchematicArrayError::CountOverflow)?;
    if end > u64::from(MAX_BUS_MEMBER_INDEX) {
        return Err(SchematicArrayError::CountOverflow);
    }
    u32::try_from(end).map_err(|_| SchematicArrayError::CountOverflow)
}

fn find_collision_free_index_stride(
    starts: &[u64],
    members: usize,
) -> Result<u64, SchematicArrayError> {
    let search_limit = starts
        .len()
        .saturating_add(1)
        .saturating_mul(members.max(2));
    for stride in 1..=u64::try_from(search_limit).unwrap_or(u64::MAX) {
        let mut generated = HashSet::new();
        let valid = starts.iter().all(|start| {
            (0..members).all(|member| {
                u64::try_from(member)
                    .ok()
                    .and_then(|member| member.checked_mul(stride))
                    .and_then(|offset| start.checked_add(offset))
                    .filter(|index| *index <= u64::from(MAX_BUS_MEMBER_INDEX))
                    .is_some_and(|index| generated.insert(index))
            })
        });
        if valid {
            return Ok(stride);
        }
    }
    Err(SchematicArrayError::CountOverflow)
}

fn scalar_bus_name(value: &str) -> Option<(String, u32, BusNotation)> {
    let slice = BusSlice::parse(value).ok()?;
    slice
        .is_scalar()
        .then_some((slice.name, slice.msb, slice.notation))
}

fn format_bus_range(name: &str, start: u32, end: u32, notation: BusNotation) -> String {
    match notation {
        BusNotation::Square => format!("{name}[{start}]…{name}[{end}]"),
        BusNotation::Angle => format!("{name}<{start}>…{name}<{end}>"),
    }
}


#[cfg(test)]
mod tests;
