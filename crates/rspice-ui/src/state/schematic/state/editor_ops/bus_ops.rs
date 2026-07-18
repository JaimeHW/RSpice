use super::super::super::{
    BusDeclaration, BusParseError, BusPropertyImpact, BusSlice, BusTapOrientation, BusTargetKind,
    PendingBusTap,
};
use super::super::*;

impl SchematicState {
    /// Add a validated bus as one read-only-safe undo transaction.
    pub fn add_bus(
        &mut self,
        points: Vec<Point>,
        declaration: Option<BusDeclaration>,
    ) -> Result<u64, BusParseError> {
        if self.read_only {
            return Err(BusParseError::ReadOnly);
        }
        let candidate = Bus::new(0, points, declaration)?;
        let mut created_id = None;
        self.with_undo("draw bus", |schematic| {
            let id = schematic.next_id();
            schematic.buses.push(Bus { id, ..candidate });
            schematic.selection.clear();
            schematic.selection.select_bus(id);
            schematic.is_dirty = true;
            schematic.bump_topology_version();
            created_id = Some(id);
        });
        created_id.ok_or(BusParseError::ReadOnly)
    }

    /// Find the first bus whose polyline contains an exact grid position.
    pub fn bus_at(&self, position: Point) -> Option<u64> {
        self.buses
            .iter()
            .find(|bus| bus.contains_point(position))
            .map(|bus| bus.id)
    }

    /// Find the nearest source point on any bus within a grid-unit radius.
    pub fn nearest_bus_at(&self, position: Point, radius: i32) -> Option<(u64, Point)> {
        let max_distance = i128::from(radius.max(0)).pow(2);
        self.buses
            .iter()
            .filter_map(|bus| {
                let (point, distance) = bus.nearest_point(position)?;
                (distance <= max_distance).then_some((bus.id, point, distance))
            })
            .min_by_key(|(_, _, distance)| *distance)
            .map(|(id, point, _)| (id, point))
    }

    /// Begin an interactive bus route.
    pub fn start_bus(
        &mut self,
        position: Point,
        declaration: Option<BusDeclaration>,
    ) -> Result<(), BusParseError> {
        if self.read_only {
            return Err(BusParseError::ReadOnly);
        }
        if let Some(declaration) = &declaration {
            declaration.validate()?;
        }
        self.bus_drawing.start(position, declaration);
        Ok(())
    }

    pub fn update_bus_preview(&mut self, position: Point) {
        self.bus_drawing.update_preview(position);
    }

    pub fn extend_bus(&mut self, position: Point) {
        if !self.read_only {
            self.bus_drawing.add_point(position);
        }
    }

    pub fn toggle_bus_routing(&mut self) {
        self.bus_drawing.routing_mode = self.bus_drawing.routing_mode.toggle();
    }

    /// Finish the active route and commit the complete polyline atomically.
    pub fn finish_bus(&mut self) -> Result<Option<u64>, BusParseError> {
        if self.read_only {
            return Err(BusParseError::ReadOnly);
        }
        if !self.bus_drawing.active {
            return Ok(None);
        }
        let points = std::mem::take(&mut self.bus_drawing.points);
        let declaration = self.bus_drawing.declaration.take();
        self.bus_drawing.cancel();
        let points = simplify_polyline(points);
        if points.len() < 2 {
            return Ok(None);
        }
        self.add_bus(points, declaration).map(Some)
    }

    pub fn cancel_bus(&mut self) {
        self.bus_drawing.cancel();
    }

    /// Cancel every unfinished conductor-routing gesture. Escape and tool
    /// switches use this to guarantee an invisible route can never commit.
    pub fn cancel_routing_gestures(&mut self) {
        self.wire_drawing.cancel();
        self.bus_drawing.cancel();
    }

    /// Place a validated tap as one atomic, undoable topology mutation.
    pub fn place_bus_tap(
        &mut self,
        bus_id: u64,
        bus_point: Point,
        connection_point: Point,
        slice: BusSlice,
        orientation: BusTapOrientation,
    ) -> Result<u64, BusParseError> {
        if self.read_only {
            return Err(BusParseError::ReadOnly);
        }
        let bus = self
            .buses
            .iter()
            .find(|bus| bus.id == bus_id)
            .ok_or(BusParseError::InvalidBusReference)?;
        let candidate = BusTap::new(0, bus, bus_point, connection_point, slice, orientation)?;
        let mut created_id = None;
        self.with_undo("place bus tap", |schematic| {
            let id = schematic.next_id();
            schematic.bus_taps.push(BusTap { id, ..candidate });
            schematic.selection.clear();
            schematic.selection.select_bus_tap(id);
            schematic.is_dirty = true;
            schematic.bump_topology_version();
            created_id = Some(id);
        });
        created_id.ok_or(BusParseError::ReadOnly)
    }

    /// Place a pending tap configuration, assigning its declaration to an
    /// unnamed source bus in the same atomic transaction. Existing declared
    /// buses must match the pending type exactly.
    pub fn place_configured_bus_tap(
        &mut self,
        bus_id: u64,
        bus_point: Point,
        connection_point: Point,
        pending: &PendingBusTap,
    ) -> Result<u64, BusParseError> {
        if self.read_only {
            return Err(BusParseError::ReadOnly);
        }
        let source = self
            .buses
            .iter()
            .find(|bus| bus.id == bus_id)
            .ok_or(BusParseError::InvalidBusReference)?;
        if let Some(existing) = &source.declaration
            && existing != &pending.bus_declaration
        {
            return Err(BusParseError::DeclarationMismatch);
        }
        let mut typed_source = source.clone();
        typed_source.declaration = Some(pending.bus_declaration.clone());
        let candidate = BusTap::new(
            0,
            &typed_source,
            bus_point,
            connection_point,
            pending.slice.clone(),
            pending.orientation,
        )?;

        let mut created_id = None;
        self.with_undo("place bus tap", |schematic| {
            if let Some(source) = schematic.buses.iter_mut().find(|bus| bus.id == bus_id) {
                source
                    .declaration
                    .get_or_insert_with(|| pending.bus_declaration.clone());
            }
            let id = schematic.next_id();
            schematic.bus_taps.push(BusTap { id, ..candidate });
            schematic.selection.clear();
            schematic.selection.select_bus_tap(id);
            schematic.is_dirty = true;
            schematic.bump_topology_version();
            created_id = Some(id);
        });
        created_id.ok_or(BusParseError::ReadOnly)
    }

    /// Move a bus while preserving tap attachment at both ends.
    pub fn move_bus(&mut self, bus_id: u64, delta: Point) -> bool {
        if self.read_only || delta == Point::origin() || !self.buses.iter().any(|b| b.id == bus_id)
        {
            return false;
        }
        let target_taps: std::collections::HashSet<u64> = self
            .buses
            .iter()
            .find(|bus| bus.id == bus_id)
            .map(|bus| {
                self.bus_taps
                    .iter()
                    .filter(|tap| {
                        tap.target_kind() == BusTargetKind::Bus
                            && bus.contains_point(tap.connection_point)
                    })
                    .map(|tap| tap.id)
                    .collect()
            })
            .unwrap_or_default();
        self.with_undo("move bus", |schematic| {
            if let Some(bus) = schematic.buses.iter_mut().find(|bus| bus.id == bus_id) {
                bus.translate(delta);
            }
            for tap in &mut schematic.bus_taps {
                if tap.bus_id == bus_id {
                    tap.bus_point = offset_point(tap.bus_point, delta);
                }
                if target_taps.contains(&tap.id) {
                    tap.connection_point = offset_point(tap.connection_point, delta);
                }
            }
            schematic.is_dirty = true;
            schematic.bump_topology_version();
        })
    }

    /// Move a tap's geometry while preserving its source relationship.
    pub fn move_bus_tap(&mut self, tap_id: u64, delta: Point) -> bool {
        if self.read_only
            || delta == Point::origin()
            || !self.bus_taps.iter().any(|tap| tap.id == tap_id)
        {
            return false;
        }
        let Some(tap) = self.bus_taps.iter().find(|tap| tap.id == tap_id) else {
            return false;
        };
        let moved_source = offset_point(tap.bus_point, delta);
        let Some(bus) = self.buses.iter().find(|bus| bus.id == tap.bus_id) else {
            return false;
        };
        if !bus.contains_point(moved_source) {
            return false;
        }
        self.with_undo("move bus tap", |schematic| {
            if let Some(tap) = schematic.bus_taps.iter_mut().find(|tap| tap.id == tap_id) {
                tap.translate(delta);
            }
            schematic.is_dirty = true;
            schematic.bump_topology_version();
        })
    }

    /// Apply the complete editable bus-property contract as one guarded undo
    /// transaction. Geometry remains bit-exact. Attached selectors follow an
    /// intentional base-name or notation rename and reverse their endpoints
    /// when declaration direction reverses; the transaction is rejected if
    /// any selected member would fall outside the new range.
    pub fn edit_bus_properties(
        &mut self,
        expected: &Bus,
        declaration: Option<BusDeclaration>,
    ) -> Result<bool, BusParseError> {
        let (candidate_buses, candidate_taps, impact) =
            build_bus_property_candidates(self, expected, declaration.as_ref())?;
        if !impact.has_changes() {
            return Ok(false);
        }

        let changed = self.with_undo("edit bus properties", move |schematic| {
            schematic.buses = candidate_buses;
            schematic.bus_taps = candidate_taps;
            schematic.is_dirty = true;
            schematic.bump_topology_version();
        });
        Ok(changed)
    }

    /// Validate and resolve the exact bus-network refactor without cloning
    /// editor caches, clipboard state, or undo history.
    pub fn validate_bus_properties(
        &self,
        expected: &Bus,
        declaration: Option<&BusDeclaration>,
    ) -> Result<BusPropertyImpact, BusParseError> {
        let (_, _, impact) = build_bus_property_candidates(self, expected, declaration)?;
        Ok(impact)
    }

    /// Apply a complete bus-tap property contract atomically. The guarded
    /// baseline prevents a stale dialog from overwriting a newer edit, and
    /// `BusTap::new` validates source membership, selector type, geometry and
    /// orientation before the undo transaction begins.
    pub fn edit_bus_tap_properties(
        &mut self,
        expected: &BusTap,
        bus_id: u64,
        bus_point: Point,
        connection_point: Point,
        slice: BusSlice,
        orientation: BusTapOrientation,
    ) -> Result<bool, BusParseError> {
        let candidate = build_bus_tap_property_candidate(
            self,
            expected,
            bus_id,
            bus_point,
            connection_point,
            slice,
            orientation,
        )?;
        if &candidate == expected {
            return Ok(false);
        }

        let tap_id = candidate.id;
        let changed = self.with_undo("edit bus tap properties", move |schematic| {
            if let Some(tap) = schematic.bus_taps.iter_mut().find(|tap| tap.id == tap_id) {
                *tap = candidate;
            }
            schematic.is_dirty = true;
            schematic.bump_topology_version();
        });
        Ok(changed)
    }

    pub fn validate_bus_tap_properties(
        &self,
        expected: &BusTap,
        bus_id: u64,
        bus_point: Point,
        connection_point: Point,
        slice: BusSlice,
        orientation: BusTapOrientation,
    ) -> Result<bool, BusParseError> {
        let candidate = build_bus_tap_property_candidate(
            self,
            expected,
            bus_id,
            bus_point,
            connection_point,
            slice,
            orientation,
        )?;
        Ok(&candidate != expected)
    }
}

fn build_bus_property_candidates(
    schematic: &SchematicState,
    expected: &Bus,
    declaration: Option<&BusDeclaration>,
) -> Result<(Vec<Bus>, Vec<BusTap>, BusPropertyImpact), BusParseError> {
    if schematic.read_only {
        return Err(BusParseError::ReadOnly);
    }
    let current = schematic
        .buses
        .iter()
        .find(|bus| bus.id == expected.id)
        .ok_or(BusParseError::InvalidBusReference)?;
    if current != expected {
        return Err(BusParseError::StaleObject);
    }
    let selected_direction_reversed = current
        .declaration
        .as_ref()
        .zip(declaration)
        .is_some_and(|(before, after)| before.direction() != after.direction());
    let mut candidate_buses = schematic.buses.clone();
    let candidate = candidate_buses
        .iter_mut()
        .find(|bus| bus.id == expected.id)
        .expect("validated bus identity remains present");
    candidate.declaration = declaration.cloned();
    candidate.validate()?;

    let connectivity = bus_connectivity(schematic, expected.id);
    let connected = &connectivity.connected;
    let mut candidate_taps = schematic.bus_taps.clone();
    let Some(selected_declaration) = declaration else {
        let has_dependency = schematic.bus_taps.iter().any(|tap| {
            tap.bus_id == expected.id
                || (tap.target_kind() == BusTargetKind::Bus
                    && connectivity
                        .targets_by_tap
                        .get(&tap.id)
                        .is_some_and(|targets| targets.contains(&expected.id)))
        });
        if has_dependency {
            return Err(BusParseError::UndeclaredBus);
        }
        let impact = property_impact(
            schematic,
            &candidate_buses,
            &candidate_taps,
            connected.len(),
        );
        return Ok((candidate_buses, candidate_taps, impact));
    };

    for bus in &mut candidate_buses {
        if bus.id == expected.id || !connected.contains(&bus.id) {
            continue;
        }
        let declaration = bus
            .declaration
            .as_mut()
            .ok_or(BusParseError::UndeclaredBus)?;
        declaration.name.clone_from(&selected_declaration.name);
        declaration.notation = selected_declaration.notation;
        // Apply the selected bus's direction *delta* to the whole connected
        // vector network. A pure rename/notation edit must preserve each
        // connected declaration's own orientation; an intentional reversal
        // reverses every connected declaration exactly once.
        if selected_direction_reversed {
            std::mem::swap(&mut declaration.msb, &mut declaration.lsb);
        }
        declaration.validate()?;
    }

    let bus_indices: std::collections::HashMap<u64, usize> = candidate_buses
        .iter()
        .enumerate()
        .map(|(index, bus)| (bus.id, index))
        .collect();
    for tap in &mut candidate_taps {
        if !connected.contains(&tap.bus_id) {
            continue;
        }
        let source = bus_indices
            .get(&tap.bus_id)
            .map(|index| &candidate_buses[*index])
            .ok_or(BusParseError::InvalidBusReference)?;
        let source_before = schematic
            .buses
            .iter()
            .find(|bus| bus.id == tap.bus_id)
            .ok_or(BusParseError::InvalidBusReference)?;
        let source_declaration = source
            .declaration
            .as_ref()
            .ok_or(BusParseError::UndeclaredBus)?;
        if tap.target_kind() == BusTargetKind::Bus {
            let targets = connectivity
                .targets_by_tap
                .get(&tap.id)
                .map(Vec::as_slice)
                .unwrap_or_default();
            match targets {
                [] => {
                    let source_direction_reversed = source_before
                        .declaration
                        .as_ref()
                        .is_some_and(|before| before.direction() != source_declaration.direction());
                    tap.slice.name.clone_from(&source_declaration.name);
                    tap.slice.notation = source_declaration.notation;
                    if source_direction_reversed {
                        std::mem::swap(&mut tap.slice.msb, &mut tap.slice.lsb);
                    }
                }
                [target_id] => {
                    let target_declaration = bus_indices
                        .get(target_id)
                        .map(|index| &candidate_buses[*index])
                        .ok_or(BusParseError::InvalidDestination)?
                        .declaration
                        .as_ref()
                        .ok_or(BusParseError::InvalidDestination)?;
                    tap.slice.name.clone_from(&target_declaration.name);
                    tap.slice.msb = target_declaration.msb;
                    tap.slice.lsb = target_declaration.lsb;
                    tap.slice.notation = target_declaration.notation;
                }
                _ => return Err(BusParseError::InvalidDestination),
            }
        } else {
            tap.slice.name.clone_from(&source_declaration.name);
            tap.slice.notation = source_declaration.notation;
        }
        tap.validate_against_bus(source)?;
        validate_tap_destination(&schematic.wires, &candidate_buses, tap)?;
    }
    let impact = property_impact(
        schematic,
        &candidate_buses,
        &candidate_taps,
        connected.len(),
    );
    Ok((candidate_buses, candidate_taps, impact))
}

#[allow(clippy::too_many_arguments)]
fn build_bus_tap_property_candidate(
    schematic: &SchematicState,
    expected: &BusTap,
    bus_id: u64,
    bus_point: Point,
    connection_point: Point,
    slice: BusSlice,
    orientation: BusTapOrientation,
) -> Result<BusTap, BusParseError> {
    if schematic.read_only {
        return Err(BusParseError::ReadOnly);
    }
    let current = schematic
        .bus_taps
        .iter()
        .find(|tap| tap.id == expected.id)
        .ok_or(BusParseError::InvalidBusReference)?;
    if current != expected {
        return Err(BusParseError::StaleObject);
    }
    let bus = schematic
        .buses
        .iter()
        .find(|bus| bus.id == bus_id)
        .ok_or(BusParseError::InvalidBusReference)?;
    let candidate = BusTap::new(
        expected.id,
        bus,
        bus_point,
        connection_point,
        slice,
        orientation,
    )?;
    validate_tap_destination(&schematic.wires, &schematic.buses, &candidate)?;
    Ok(candidate)
}

fn simplify_polyline(points: Vec<Point>) -> Vec<Point> {
    let mut result = Vec::with_capacity(points.len());
    for point in points {
        if result.last() == Some(&point) {
            continue;
        }
        while result.len() >= 2 {
            let a: Point = result[result.len() - 2];
            let b: Point = result[result.len() - 1];
            let collinear = (i128::from(b.x) - i128::from(a.x))
                * (i128::from(point.y) - i128::from(b.y))
                == (i128::from(b.y) - i128::from(a.y)) * (i128::from(point.x) - i128::from(b.x));
            if !collinear {
                break;
            }
            result.pop();
        }
        result.push(point);
    }
    result
}

fn offset_point(point: Point, delta: Point) -> Point {
    Point::new(
        point.x.saturating_add(delta.x),
        point.y.saturating_add(delta.y),
    )
}

#[derive(Debug, Default)]
struct BusConnectivity {
    connected: std::collections::HashSet<u64>,
    targets_by_tap: std::collections::HashMap<u64, Vec<u64>>,
}

fn bus_connectivity(schematic: &SchematicState, seed: u64) -> BusConnectivity {
    let mut adjacency: std::collections::HashMap<u64, Vec<u64>> = schematic
        .buses
        .iter()
        .map(|bus| (bus.id, Vec::new()))
        .collect();
    let mut targets_by_tap = std::collections::HashMap::new();
    for tap in &schematic.bus_taps {
        if tap.target_kind() != BusTargetKind::Bus {
            continue;
        }
        let targets: Vec<u64> = schematic
            .buses
            .iter()
            .filter(|bus| bus.id != tap.bus_id && bus.contains_point(tap.connection_point))
            .map(|bus| bus.id)
            .collect();
        for target in &targets {
            adjacency.entry(tap.bus_id).or_default().push(*target);
            adjacency.entry(*target).or_default().push(tap.bus_id);
        }
        targets_by_tap.insert(tap.id, targets);
    }

    let mut connected = std::collections::HashSet::new();
    let mut pending = std::collections::VecDeque::from([seed]);
    while let Some(bus_id) = pending.pop_front() {
        if !connected.insert(bus_id) {
            continue;
        }
        if let Some(neighbors) = adjacency.get(&bus_id) {
            pending.extend(neighbors.iter().copied());
        }
    }
    BusConnectivity {
        connected,
        targets_by_tap,
    }
}

fn property_impact(
    schematic: &SchematicState,
    candidate_buses: &[Bus],
    candidate_taps: &[BusTap],
    connected_buses: usize,
) -> BusPropertyImpact {
    BusPropertyImpact {
        connected_buses,
        buses_changed: schematic
            .buses
            .iter()
            .zip(candidate_buses)
            .filter(|(stored, candidate)| stored != candidate)
            .count(),
        taps_changed: schematic
            .bus_taps
            .iter()
            .zip(candidate_taps)
            .filter(|(stored, candidate)| stored != candidate)
            .count(),
    }
}

fn validate_tap_destination(
    wires: &[Wire],
    buses: &[Bus],
    candidate: &BusTap,
) -> Result<(), BusParseError> {
    let touches_wire = wires
        .iter()
        .any(|wire| wire.contains_point(candidate.connection_point));
    let buses_at_destination: Vec<&Bus> = buses
        .iter()
        .filter(|bus| bus.contains_point(candidate.connection_point))
        .collect();
    let source_collision = buses_at_destination
        .iter()
        .any(|bus| bus.id == candidate.bus_id);
    let destination_buses: Vec<&Bus> = buses_at_destination
        .into_iter()
        .filter(|bus| bus.id != candidate.bus_id)
        .collect();

    if source_collision {
        return Err(BusParseError::InvalidDestination);
    }

    match candidate.target_kind() {
        BusTargetKind::Wire if !touches_wire && destination_buses.is_empty() => Ok(()),
        BusTargetKind::Wire if touches_wire && destination_buses.is_empty() => Ok(()),
        BusTargetKind::Bus if !touches_wire && destination_buses.is_empty() => Ok(()),
        BusTargetKind::Bus
            if !touches_wire
                && destination_buses.len() == 1
                && destination_matches_slice(&destination_buses, &candidate.slice) =>
        {
            Ok(())
        }
        BusTargetKind::Wire | BusTargetKind::Bus => Err(BusParseError::InvalidDestination),
    }
}

fn destination_matches_slice(destination_buses: &[&Bus], slice: &BusSlice) -> bool {
    let Ok(expected) =
        BusDeclaration::new(slice.name.clone(), slice.msb, slice.lsb, slice.notation)
    else {
        return false;
    };
    destination_buses
        .iter()
        .any(|bus| bus.declaration.as_ref() == Some(&expected))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn declared_bus(state: &mut SchematicState) -> u64 {
        state
            .add_bus(
                vec![Point::new(0, 0), Point::new(20, 0)],
                Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
            )
            .unwrap()
    }

    #[test]
    fn route_finish_is_one_undoable_polyline_operation() {
        let mut state = SchematicState::default();
        state.start_bus(Point::new(0, 0), None).unwrap();
        state.extend_bus(Point::new(10, 10));
        let id = state.finish_bus().unwrap().unwrap();
        assert_eq!(state.buses.len(), 1);
        assert_eq!(state.buses[0].id, id);
        assert!(state.undo());
        assert!(state.buses.is_empty());
    }

    #[test]
    fn tap_placement_allows_fanout_and_is_undoable() {
        let mut state = SchematicState::default();
        let bus_id = declared_bus(&mut state);
        state.clear_undo_history();
        let tap_id = state
            .place_bus_tap(
                bus_id,
                Point::new(10, 0),
                Point::new(10, 5),
                BusSlice::parse("DATA[3]").unwrap(),
                BusTapOrientation::Down,
            )
            .unwrap();
        assert_eq!(state.bus_taps[0].id, tap_id);
        assert!(
            state
                .place_bus_tap(
                    bus_id,
                    Point::new(15, 0),
                    Point::new(15, 5),
                    BusSlice::parse("DATA[3]").unwrap(),
                    BusTapOrientation::Down,
                )
                .is_ok()
        );
        assert!(state.undo());
        assert_eq!(state.bus_taps.len(), 1);
    }

    #[test]
    fn configured_tap_types_unnamed_bus_in_same_undo_step() {
        let mut state = SchematicState::default();
        let bus_id = state
            .add_bus(vec![Point::new(0, 0), Point::new(20, 0)], None)
            .unwrap();
        state.clear_undo_history();
        let pending = PendingBusTap::new(
            BusDeclaration::parse("DATA[7:0]").unwrap(),
            BusSlice::parse("DATA[4]").unwrap(),
            BusTapOrientation::Automatic,
        )
        .unwrap();
        state
            .place_configured_bus_tap(bus_id, Point::new(5, 0), Point::new(5, 5), &pending)
            .unwrap();
        assert_eq!(state.buses[0].declaration, Some(pending.bus_declaration));
        assert_eq!(state.bus_taps.len(), 1);
        assert!(state.undo());
        assert_eq!(state.buses[0].declaration, None);
        assert!(state.bus_taps.is_empty());
    }

    #[test]
    fn read_only_bus_transactions_never_mutate_or_create_undo() {
        let mut state = SchematicState {
            read_only: true,
            ..SchematicState::default()
        };
        assert_eq!(
            state.add_bus(vec![Point::new(0, 0), Point::new(5, 0)], None),
            Err(BusParseError::ReadOnly)
        );
        assert!(state.buses.is_empty());
        assert!(!state.can_undo());
    }

    #[test]
    fn nearest_bus_returns_projected_source_point() {
        let mut state = SchematicState::default();
        let bus_id = declared_bus(&mut state);
        assert_eq!(
            state.nearest_bus_at(Point::new(7, 2), 2),
            Some((bus_id, Point::new(7, 0)))
        );
        assert_eq!(state.nearest_bus_at(Point::new(7, 3), 2), None);
    }

    #[test]
    fn bus_property_edit_retypes_attached_taps_in_one_undo() {
        let mut state = SchematicState::default();
        let bus_id = declared_bus(&mut state);
        state
            .wires
            .push(Wire::segment(90, Point::new(0, 5), Point::new(20, 5)));
        state
            .place_bus_tap(
                bus_id,
                Point::new(5, 0),
                Point::new(5, 5),
                BusSlice::parse("DATA[3]").unwrap(),
                BusTapOrientation::Down,
            )
            .unwrap();
        state.clear_undo_history();
        let expected = state.buses[0].clone();

        assert!(
            state
                .edit_bus_properties(
                    &expected,
                    Some(BusDeclaration::parse("DATA[15:0]").unwrap()),
                )
                .unwrap()
        );
        assert_eq!(state.buses[0].points, expected.points);
        assert_eq!(state.bus_taps[0].bus_point, Point::new(5, 0));
        assert_eq!(state.bus_taps[0].connection_point, Point::new(5, 5));
        assert_eq!(
            state.buses[0].declaration,
            Some(BusDeclaration::parse("DATA[15:0]").unwrap())
        );
        assert_eq!(state.undo_history.undo_count(), 1);
        assert!(state.undo());
        assert_eq!(state.buses[0], expected);
        assert_eq!(state.bus_taps[0].bus_point, Point::new(5, 0));
    }

    #[test]
    fn bus_property_edit_rejects_a_declaration_that_invalidates_a_tap() {
        let mut state = SchematicState::default();
        let bus_id = declared_bus(&mut state);
        state
            .place_bus_tap(
                bus_id,
                Point::new(5, 0),
                Point::new(5, 5),
                BusSlice::parse("DATA[7]").unwrap(),
                BusTapOrientation::Down,
            )
            .unwrap();
        state.clear_undo_history();
        let expected = state.buses[0].clone();

        assert_eq!(
            state
                .edit_bus_properties(&expected, Some(BusDeclaration::parse("DATA[3:0]").unwrap()),),
            Err(BusParseError::SelectorOutOfRange)
        );
        assert_eq!(state.buses[0], expected);
        assert!(!state.can_undo());
    }

    #[test]
    fn bus_rename_notation_and_direction_rebase_dependent_selectors() {
        let mut state = SchematicState::default();
        let bus_id = declared_bus(&mut state);
        state
            .add_bus(
                vec![Point::new(0, 5), Point::new(20, 5)],
                Some(BusDeclaration::parse("DATA[6:4]").unwrap()),
            )
            .unwrap();
        state
            .place_bus_tap(
                bus_id,
                Point::new(5, 0),
                Point::new(5, 5),
                BusSlice::parse("DATA[6:4]").unwrap(),
                BusTapOrientation::Down,
            )
            .unwrap();
        state.clear_undo_history();
        let expected = state.buses[0].clone();

        assert!(
            state
                .edit_bus_properties(
                    &expected,
                    Some(BusDeclaration::parse("ADDR<0:15>").unwrap()),
                )
                .unwrap()
        );
        assert_eq!(state.buses[0].points, expected.points);
        assert_eq!(
            state.bus_taps[0].slice,
            BusSlice::parse("ADDR<4:6>").unwrap()
        );
        assert_eq!(
            state.buses[1].declaration,
            Some(BusDeclaration::parse("ADDR<4:6>").unwrap())
        );
        assert!(state.undo());
        assert_eq!(
            state.bus_taps[0].slice,
            BusSlice::parse("DATA[6:4]").unwrap()
        );
    }

    #[test]
    fn rename_without_selected_reversal_preserves_connected_bus_orientation() {
        let mut state = SchematicState::default();
        let selected = state
            .add_bus(
                vec![Point::new(0, 0), Point::new(20, 0)],
                Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
            )
            .unwrap();
        let connected = state
            .add_bus(
                vec![Point::new(0, 5), Point::new(20, 5)],
                Some(BusDeclaration::parse("DATA[0:3]").unwrap()),
            )
            .unwrap();
        state
            .place_bus_tap(
                selected,
                Point::new(5, 0),
                Point::new(5, 5),
                BusSlice::parse("DATA[0:3]").unwrap(),
                BusTapOrientation::Down,
            )
            .unwrap();
        state.clear_undo_history();
        let before = state.clone();
        let expected = state
            .buses
            .iter()
            .find(|bus| bus.id == selected)
            .unwrap()
            .clone();

        assert!(
            state
                .edit_bus_properties(&expected, Some(BusDeclaration::parse("ADDR[7:0]").unwrap()),)
                .unwrap()
        );
        assert_eq!(
            state
                .buses
                .iter()
                .find(|bus| bus.id == connected)
                .unwrap()
                .declaration,
            Some(BusDeclaration::parse("ADDR[0:3]").unwrap())
        );
        assert_eq!(
            state.bus_taps[0].slice,
            BusSlice::parse("ADDR[0:3]").unwrap()
        );
        assert_eq!(state.undo_history.undo_count(), 1);
        assert!(state.undo());
        assert_eq!(state.buses, before.buses);
        assert_eq!(state.bus_taps, before.bus_taps);
    }

    #[test]
    fn dangling_scalar_tap_properties_allow_selector_and_orientation_edits() {
        let mut state = SchematicState::default();
        let source = declared_bus(&mut state);
        let tap_id = state
            .place_bus_tap(
                source,
                Point::new(5, 0),
                Point::new(5, 5),
                BusSlice::parse("DATA[3]").unwrap(),
                BusTapOrientation::Down,
            )
            .unwrap();
        state.clear_undo_history();
        let expected = state
            .bus_taps
            .iter()
            .find(|tap| tap.id == tap_id)
            .unwrap()
            .clone();

        assert!(
            state
                .edit_bus_tap_properties(
                    &expected,
                    source,
                    expected.bus_point,
                    expected.connection_point,
                    BusSlice::parse("DATA[4]").unwrap(),
                    BusTapOrientation::Left,
                )
                .unwrap()
        );
        assert_eq!(state.bus_taps[0].slice, BusSlice::parse("DATA[4]").unwrap());
        assert_eq!(state.bus_taps[0].orientation, BusTapOrientation::Left);
        assert_eq!(state.undo_history.undo_count(), 1);
    }

    #[test]
    fn dangling_vector_tap_accepts_opposite_direction_within_source_only() {
        let mut state = SchematicState::default();
        let source = declared_bus(&mut state);
        let tap_id = state
            .place_bus_tap(
                source,
                Point::new(5, 0),
                Point::new(5, 5),
                BusSlice::parse("DATA[3:0]").unwrap(),
                BusTapOrientation::Down,
            )
            .unwrap();
        state.clear_undo_history();
        let expected = state
            .bus_taps
            .iter()
            .find(|tap| tap.id == tap_id)
            .unwrap()
            .clone();

        assert!(
            state
                .edit_bus_tap_properties(
                    &expected,
                    source,
                    expected.bus_point,
                    expected.connection_point,
                    BusSlice::parse("DATA[0:3]").unwrap(),
                    BusTapOrientation::Up,
                )
                .unwrap()
        );
        let edited = state.bus_taps[0].clone();
        assert_eq!(edited.slice, BusSlice::parse("DATA[0:3]").unwrap());
        assert_eq!(edited.orientation, BusTapOrientation::Up);
        assert_eq!(
            state.edit_bus_tap_properties(
                &edited,
                source,
                edited.bus_point,
                edited.connection_point,
                BusSlice::parse("DATA[8:9]").unwrap(),
                edited.orientation,
            ),
            Err(BusParseError::SelectorOutOfRange)
        );
        assert_eq!(state.bus_taps[0], edited);
    }

    #[test]
    fn bus_rename_rebases_dangling_scalar_and_vector_taps_atomically() {
        let mut state = SchematicState::default();
        let source = declared_bus(&mut state);
        state
            .place_bus_tap(
                source,
                Point::new(5, 0),
                Point::new(5, 5),
                BusSlice::parse("DATA[3]").unwrap(),
                BusTapOrientation::Down,
            )
            .unwrap();
        state
            .place_bus_tap(
                source,
                Point::new(10, 0),
                Point::new(10, 5),
                BusSlice::parse("DATA[6:4]").unwrap(),
                BusTapOrientation::Down,
            )
            .unwrap();
        state.clear_undo_history();
        let before_taps = state.bus_taps.clone();
        let expected = state.buses[0].clone();

        assert!(
            state
                .edit_bus_properties(&expected, Some(BusDeclaration::parse("ADDR<0:7>").unwrap()),)
                .unwrap()
        );
        assert_eq!(state.bus_taps[0].slice, BusSlice::parse("ADDR<3>").unwrap());
        assert_eq!(
            state.bus_taps[1].slice,
            BusSlice::parse("ADDR<4:6>").unwrap()
        );
        assert_eq!(state.undo_history.undo_count(), 1);
        assert!(state.undo());
        assert_eq!(state.bus_taps, before_taps);
    }

    #[test]
    fn tap_property_edit_can_rebind_source_and_is_stale_safe() {
        let mut state = SchematicState::default();
        let first = declared_bus(&mut state);
        let second = state
            .add_bus(
                vec![Point::new(0, 10), Point::new(20, 10)],
                Some(BusDeclaration::parse("ADDR[7:0]").unwrap()),
            )
            .unwrap();
        state
            .wires
            .push(Wire::segment(90, Point::new(0, 5), Point::new(20, 5)));
        state
            .wires
            .push(Wire::segment(91, Point::new(0, 15), Point::new(20, 15)));
        let tap_id = state
            .place_bus_tap(
                first,
                Point::new(5, 0),
                Point::new(5, 5),
                BusSlice::parse("DATA[3]").unwrap(),
                BusTapOrientation::Down,
            )
            .unwrap();
        state.clear_undo_history();
        let expected = state
            .bus_taps
            .iter()
            .find(|tap| tap.id == tap_id)
            .unwrap()
            .clone();

        assert!(
            state
                .edit_bus_tap_properties(
                    &expected,
                    second,
                    Point::new(5, 10),
                    Point::new(5, 15),
                    BusSlice::parse("ADDR[2]").unwrap(),
                    BusTapOrientation::Up,
                )
                .unwrap()
        );
        assert_eq!(state.bus_taps[0].bus_id, second);
        assert_eq!(state.bus_taps[0].slice, BusSlice::parse("ADDR[2]").unwrap());
        assert!(state.undo());
        assert_eq!(state.bus_taps[0], expected);

        state.bus_taps[0].orientation = BusTapOrientation::Left;
        assert_eq!(
            state.edit_bus_tap_properties(
                &expected,
                first,
                expected.bus_point,
                expected.connection_point,
                expected.slice.clone(),
                expected.orientation,
            ),
            Err(BusParseError::StaleObject)
        );
    }

    #[test]
    fn tap_property_edit_rejects_scalar_bus_destination_type_mismatches() {
        let mut state = SchematicState::default();
        let source = declared_bus(&mut state);
        state
            .wires
            .push(Wire::segment(90, Point::new(0, 5), Point::new(20, 5)));
        let tap_id = state
            .place_bus_tap(
                source,
                Point::new(5, 0),
                Point::new(5, 5),
                BusSlice::parse("DATA[3]").unwrap(),
                BusTapOrientation::Down,
            )
            .unwrap();
        state.clear_undo_history();
        let expected = state
            .bus_taps
            .iter()
            .find(|tap| tap.id == tap_id)
            .unwrap()
            .clone();

        assert_eq!(
            state.edit_bus_tap_properties(
                &expected,
                source,
                expected.bus_point,
                expected.connection_point,
                BusSlice::parse("DATA[3:2]").unwrap(),
                expected.orientation,
            ),
            Err(BusParseError::InvalidDestination)
        );
        assert_eq!(state.bus_taps[0], expected);
        assert!(!state.can_undo());
    }

    #[test]
    fn tap_property_edit_rejects_a_range_incompatible_with_destination_bus() {
        let mut state = SchematicState::default();
        let source = declared_bus(&mut state);
        state
            .add_bus(
                vec![Point::new(0, 5), Point::new(20, 5)],
                Some(BusDeclaration::parse("DATA[3:2]").unwrap()),
            )
            .unwrap();
        let tap_id = state
            .place_bus_tap(
                source,
                Point::new(5, 0),
                Point::new(5, 5),
                BusSlice::parse("DATA[3:2]").unwrap(),
                BusTapOrientation::Down,
            )
            .unwrap();
        state.clear_undo_history();
        let expected = state
            .bus_taps
            .iter()
            .find(|tap| tap.id == tap_id)
            .unwrap()
            .clone();

        assert_eq!(
            state.edit_bus_tap_properties(
                &expected,
                source,
                expected.bus_point,
                expected.connection_point,
                BusSlice::parse("DATA[5:4]").unwrap(),
                expected.orientation,
            ),
            Err(BusParseError::InvalidDestination)
        );
        assert_eq!(state.bus_taps[0], expected);
        assert!(!state.can_undo());
    }

    #[test]
    fn tap_destination_validation_rejects_source_loops_mixed_and_ambiguous_targets() {
        let mut scalar_state = SchematicState::default();
        let scalar_source = declared_bus(&mut scalar_state);
        let scalar_tap = scalar_state
            .place_bus_tap(
                scalar_source,
                Point::new(5, 0),
                Point::new(5, 5),
                BusSlice::parse("DATA[3]").unwrap(),
                BusTapOrientation::Down,
            )
            .unwrap();
        let scalar_expected = scalar_state
            .bus_taps
            .iter()
            .find(|tap| tap.id == scalar_tap)
            .unwrap()
            .clone();

        assert_eq!(
            scalar_state.edit_bus_tap_properties(
                &scalar_expected,
                scalar_source,
                scalar_expected.bus_point,
                Point::new(10, 0),
                scalar_expected.slice.clone(),
                scalar_expected.orientation,
            ),
            Err(BusParseError::InvalidDestination)
        );

        scalar_state
            .add_bus(
                vec![Point::new(0, 5), Point::new(20, 5)],
                Some(BusDeclaration::parse("DATA[3:0]").unwrap()),
            )
            .unwrap();
        scalar_state
            .wires
            .push(Wire::segment(90, Point::new(0, 5), Point::new(20, 5)));
        assert_eq!(
            scalar_state.edit_bus_tap_properties(
                &scalar_expected,
                scalar_source,
                scalar_expected.bus_point,
                scalar_expected.connection_point,
                scalar_expected.slice.clone(),
                scalar_expected.orientation,
            ),
            Err(BusParseError::InvalidDestination)
        );

        let mut vector_state = SchematicState::default();
        let vector_source = declared_bus(&mut vector_state);
        let first_target = vector_state
            .add_bus(
                vec![Point::new(0, 5), Point::new(20, 5)],
                Some(BusDeclaration::parse("DATA[3:0]").unwrap()),
            )
            .unwrap();
        vector_state
            .add_bus(
                vec![Point::new(0, 5), Point::new(20, 5)],
                Some(BusDeclaration::parse("DATA[3:0]").unwrap()),
            )
            .unwrap();
        let vector_tap = vector_state
            .place_bus_tap(
                vector_source,
                Point::new(5, 0),
                Point::new(5, 10),
                BusSlice::parse("DATA[3:0]").unwrap(),
                BusTapOrientation::Down,
            )
            .unwrap();
        let vector_expected = vector_state
            .bus_taps
            .iter()
            .find(|tap| tap.id == vector_tap)
            .unwrap()
            .clone();

        assert_eq!(
            vector_state.edit_bus_tap_properties(
                &vector_expected,
                vector_source,
                vector_expected.bus_point,
                Point::new(5, 5),
                vector_expected.slice.clone(),
                vector_expected.orientation,
            ),
            Err(BusParseError::InvalidDestination)
        );

        vector_state
            .buses
            .retain(|bus| bus.id == vector_source || bus.id == first_target);
        vector_state
            .wires
            .push(Wire::segment(91, Point::new(0, 5), Point::new(20, 5)));
        assert_eq!(
            vector_state.edit_bus_tap_properties(
                &vector_expected,
                vector_source,
                vector_expected.bus_point,
                Point::new(5, 5),
                vector_expected.slice.clone(),
                vector_expected.orientation,
            ),
            Err(BusParseError::InvalidDestination)
        );
    }

    #[test]
    fn bus_property_edit_refactors_an_incoming_typed_connection_atomically() {
        let mut state = SchematicState::default();
        let destination = state
            .add_bus(
                vec![Point::new(0, 5), Point::new(20, 5)],
                Some(BusDeclaration::parse("LINK[3:0]").unwrap()),
            )
            .unwrap();
        let source = state
            .add_bus(
                vec![Point::new(0, 0), Point::new(20, 0)],
                Some(BusDeclaration::parse("LINK[3:0]").unwrap()),
            )
            .unwrap();
        state
            .place_bus_tap(
                source,
                Point::new(5, 0),
                Point::new(5, 5),
                BusSlice::parse("LINK[3:0]").unwrap(),
                BusTapOrientation::Down,
            )
            .unwrap();
        state.clear_undo_history();
        let before_buses = state.buses.clone();
        let before_taps = state.bus_taps.clone();
        let expected = state
            .buses
            .iter()
            .find(|bus| bus.id == destination)
            .unwrap()
            .clone();

        assert!(
            state
                .edit_bus_properties(
                    &expected,
                    Some(BusDeclaration::parse("RENAMED[3:0]").unwrap()),
                )
                .unwrap()
        );
        assert_eq!(
            state
                .buses
                .iter()
                .find(|bus| bus.id == destination)
                .unwrap(),
            &Bus::segment(
                destination,
                Point::new(0, 5),
                Point::new(20, 5),
                Some(BusDeclaration::parse("RENAMED[3:0]").unwrap()),
            )
            .unwrap()
        );
        assert_eq!(
            state
                .buses
                .iter()
                .find(|bus| bus.id == source)
                .unwrap()
                .declaration,
            Some(BusDeclaration::parse("RENAMED[3:0]").unwrap())
        );
        assert_eq!(
            state.bus_taps[0].slice,
            BusSlice::parse("RENAMED[3:0]").unwrap()
        );
        assert_eq!(state.undo_history.undo_count(), 1);
        let after_buses = state.buses.clone();
        let after_taps = state.bus_taps.clone();
        assert!(state.undo());
        assert_eq!(state.buses, before_buses);
        assert_eq!(state.bus_taps, before_taps);
        assert_eq!(
            state
                .buses
                .iter()
                .find(|bus| bus.id == destination)
                .unwrap(),
            &expected
        );
        assert!(state.redo());
        assert_eq!(state.buses, after_buses);
        assert_eq!(state.bus_taps, after_taps);
        assert_eq!(
            state.bus_taps[0].slice,
            BusSlice::parse("RENAMED[3:0]").unwrap()
        );
    }

    #[test]
    fn identical_bus_property_commit_is_a_clean_noop() {
        let mut state = SchematicState::default();
        declared_bus(&mut state);
        state.clear_undo_history();
        state.is_dirty = false;
        let version = state.topology_version();
        let expected = state.buses[0].clone();
        let declaration = expected.declaration.clone().expect("declared bus");

        let impact = state
            .validate_bus_properties(&expected, Some(&declaration))
            .unwrap();
        assert_eq!(impact.connected_buses, 1);
        assert!(!impact.has_changes());
        assert!(
            !state
                .edit_bus_properties(&expected, Some(declaration))
                .unwrap()
        );
        assert_eq!(state.topology_version(), version);
        assert!(!state.is_dirty);
        assert_eq!(state.undo_history.undo_count(), 0);
    }
}
