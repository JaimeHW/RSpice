use super::super::super::{
    BusDeclaration, BusParseError, BusSlice, BusTapOrientation, BusTargetKind, PendingBusTap,
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
}
