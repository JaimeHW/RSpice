//! Buses.
//!
//! Drawing and editing bus segments, hit-testing them, and the taps that
//! break individual signals out of a bus. A bus carries a ripped range of
//! nets, so adding or removing a tap changes connectivity, not just drawing.

use super::super::super::{
    BusDeclaration, BusParseError, BusPropertyImpact, BusSlice, BusTapOrientation, PendingBusTap,
};
use super::super::*;
use rspice_design::schematic::bus_edit::{self, BusTapGeometry};

impl SchematicState {
    /// Add a validated bus as one read-only-safe undo transaction.
    pub fn add_bus(
        &mut self,
        points: Vec<Point>,
        declaration: Option<BusDeclaration>,
    ) -> Result<u64, BusParseError> {
        if self.session.read_only {
            return Err(BusParseError::ReadOnly);
        }
        let edit = self.design.add_bus(points, declaration)?;
        self.session.editor.selection.clear();
        self.session.editor.selection.select_bus(edit.value);
        self.finish_document_edit(edit.committed);
        Ok(edit.value)
    }

    /// Begin an interactive bus route.
    pub fn start_bus(
        &mut self,
        position: Point,
        declaration: Option<BusDeclaration>,
    ) -> Result<(), BusParseError> {
        if self.session.read_only {
            return Err(BusParseError::ReadOnly);
        }
        if let Some(declaration) = &declaration {
            declaration.validate()?;
        }
        self.session.editor.bus_drawing.start(position, declaration);
        Ok(())
    }

    pub fn extend_bus(&mut self, position: Point) {
        if !self.session.read_only {
            self.session.editor.bus_drawing.add_point(position);
        }
    }

    /// Finish the active route and commit the complete polyline atomically.
    pub fn finish_bus(&mut self) -> Result<Option<u64>, BusParseError> {
        if self.session.read_only {
            return Err(BusParseError::ReadOnly);
        }
        if !self.session.editor.bus_drawing.active {
            return Ok(None);
        }
        let points = std::mem::take(&mut self.session.editor.bus_drawing.points);
        let declaration = self.session.editor.bus_drawing.declaration.take();
        self.session.editor.bus_drawing.cancel();
        let points = bus_edit::simplify_polyline(points);
        if points.len() < 2 {
            return Ok(None);
        }
        self.add_bus(points, declaration).map(Some)
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
        if self.session.read_only {
            return Err(BusParseError::ReadOnly);
        }
        let edit = self.design.place_bus_tap(
            BusTapGeometry {
                bus_id,
                bus_point,
                connection_point,
                orientation,
            },
            slice,
        )?;
        self.session.editor.selection.clear();
        self.session.editor.selection.select_bus_tap(edit.value);
        self.finish_document_edit(edit.committed);
        Ok(edit.value)
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
        if self.session.read_only {
            return Err(BusParseError::ReadOnly);
        }
        let edit = self.design.place_configured_bus_tap(
            BusTapGeometry {
                bus_id,
                bus_point,
                connection_point,
                orientation: pending.orientation,
            },
            &pending.bus_declaration,
            &pending.slice,
        )?;
        self.session.editor.selection.clear();
        self.session.editor.selection.select_bus_tap(edit.value);
        self.finish_document_edit(edit.committed);
        Ok(edit.value)
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
        if self.session.read_only {
            return Err(BusParseError::ReadOnly);
        }
        let Some(edit) = self.design.edit_bus_properties(expected, declaration)? else {
            return Ok(false);
        };
        self.finish_document_edit(edit.committed);
        Ok(edit.committed)
    }

    /// Validate and resolve the exact bus-network refactor without cloning
    /// editor caches, clipboard state, or undo history.
    pub fn validate_bus_properties(
        &self,
        expected: &Bus,
        declaration: Option<&BusDeclaration>,
    ) -> Result<BusPropertyImpact, BusParseError> {
        if self.session.read_only {
            return Err(BusParseError::ReadOnly);
        }
        bus_edit::validate_bus_properties(self.design.document(), expected, declaration)
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
        if self.session.read_only {
            return Err(BusParseError::ReadOnly);
        }
        let Some(edit) = self.design.edit_bus_tap_properties(
            expected,
            BusTapGeometry {
                bus_id,
                bus_point,
                connection_point,
                orientation,
            },
            slice,
        )?
        else {
            return Ok(false);
        };
        self.finish_document_edit(edit.committed);
        Ok(edit.committed)
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
        if self.session.read_only {
            return Err(BusParseError::ReadOnly);
        }
        bus_edit::validate_bus_tap_properties(
            self.design.document(),
            expected,
            BusTapGeometry {
                bus_id,
                bus_point,
                connection_point,
                orientation,
            },
            slice,
        )
    }
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
        assert_eq!(state.design.document().buses.len(), 1);
        assert_eq!(state.design.document().buses[0].id, id);
        assert!(state.undo());
        assert!(state.design.document().buses.is_empty());
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
        assert_eq!(state.design.document().bus_taps[0].id, tap_id);
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
        assert_eq!(state.design.document().bus_taps.len(), 1);
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
        assert_eq!(
            state.design.document().buses[0].declaration,
            Some(pending.bus_declaration)
        );
        assert_eq!(state.design.document().bus_taps.len(), 1);
        assert!(state.undo());
        assert_eq!(state.design.document().buses[0].declaration, None);
        assert!(state.design.document().bus_taps.is_empty());
    }

    #[test]
    fn read_only_bus_transactions_never_mutate_or_create_undo() {
        let mut state = SchematicState::default();
        state.session.read_only = true;
        assert_eq!(
            state.add_bus(vec![Point::new(0, 0), Point::new(5, 0)], None),
            Err(BusParseError::ReadOnly)
        );
        assert!(state.design.document().buses.is_empty());
        assert!(!state.can_undo());
    }

    #[test]
    fn bus_property_edit_retypes_attached_taps_in_one_undo() {
        let mut state = SchematicState::default();
        let bus_id = declared_bus(&mut state);
        state
            .design
            .document_mut_for_test()
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
        let expected = state.design.document().buses[0].clone();

        assert!(
            state
                .edit_bus_properties(
                    &expected,
                    Some(BusDeclaration::parse("DATA[15:0]").unwrap()),
                )
                .unwrap()
        );
        assert_eq!(state.design.document().buses[0].points, expected.points);
        assert_eq!(
            state.design.document().bus_taps[0].bus_point,
            Point::new(5, 0)
        );
        assert_eq!(
            state.design.document().bus_taps[0].connection_point,
            Point::new(5, 5)
        );
        assert_eq!(
            state.design.document().buses[0].declaration,
            Some(BusDeclaration::parse("DATA[15:0]").unwrap())
        );
        assert_eq!(state.history().undo_count(), 1);
        assert!(state.undo());
        assert_eq!(state.design.document().buses[0], expected);
        assert_eq!(
            state.design.document().bus_taps[0].bus_point,
            Point::new(5, 0)
        );
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
        let expected = state.design.document().buses[0].clone();

        assert_eq!(
            state
                .edit_bus_properties(&expected, Some(BusDeclaration::parse("DATA[3:0]").unwrap()),),
            Err(BusParseError::SelectorOutOfRange)
        );
        assert_eq!(state.design.document().buses[0], expected);
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
        let expected = state.design.document().buses[0].clone();

        assert!(
            state
                .edit_bus_properties(
                    &expected,
                    Some(BusDeclaration::parse("ADDR<0:15>").unwrap()),
                )
                .unwrap()
        );
        assert_eq!(state.design.document().buses[0].points, expected.points);
        assert_eq!(
            state.design.document().bus_taps[0].slice,
            BusSlice::parse("ADDR<4:6>").unwrap()
        );
        assert_eq!(
            state.design.document().buses[1].declaration,
            Some(BusDeclaration::parse("ADDR<4:6>").unwrap())
        );
        assert!(state.undo());
        assert_eq!(
            state.design.document().bus_taps[0].slice,
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
            .design
            .document()
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
                .design
                .document()
                .buses
                .iter()
                .find(|bus| bus.id == connected)
                .unwrap()
                .declaration,
            Some(BusDeclaration::parse("ADDR[0:3]").unwrap())
        );
        assert_eq!(
            state.design.document().bus_taps[0].slice,
            BusSlice::parse("ADDR[0:3]").unwrap()
        );
        assert_eq!(state.history().undo_count(), 1);
        assert!(state.undo());
        assert_eq!(
            state.design.document().buses,
            before.design.document().buses
        );
        assert_eq!(
            state.design.document().bus_taps,
            before.design.document().bus_taps
        );
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
            .design
            .document()
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
        assert_eq!(
            state.design.document().bus_taps[0].slice,
            BusSlice::parse("DATA[4]").unwrap()
        );
        assert_eq!(
            state.design.document().bus_taps[0].orientation,
            BusTapOrientation::Left
        );
        assert_eq!(state.history().undo_count(), 1);
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
            .design
            .document()
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
        let edited = state.design.document().bus_taps[0].clone();
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
        assert_eq!(state.design.document().bus_taps[0], edited);
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
        let before_taps = state.design.document().bus_taps.clone();
        let expected = state.design.document().buses[0].clone();

        assert!(
            state
                .edit_bus_properties(&expected, Some(BusDeclaration::parse("ADDR<0:7>").unwrap()),)
                .unwrap()
        );
        assert_eq!(
            state.design.document().bus_taps[0].slice,
            BusSlice::parse("ADDR<3>").unwrap()
        );
        assert_eq!(
            state.design.document().bus_taps[1].slice,
            BusSlice::parse("ADDR<4:6>").unwrap()
        );
        assert_eq!(state.history().undo_count(), 1);
        assert!(state.undo());
        assert_eq!(state.design.document().bus_taps, before_taps);
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
            .design
            .document_mut_for_test()
            .wires
            .push(Wire::segment(90, Point::new(0, 5), Point::new(20, 5)));
        state
            .design
            .document_mut_for_test()
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
            .design
            .document()
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
        assert_eq!(state.design.document().bus_taps[0].bus_id, second);
        assert_eq!(
            state.design.document().bus_taps[0].slice,
            BusSlice::parse("ADDR[2]").unwrap()
        );
        assert!(state.undo());
        assert_eq!(state.design.document().bus_taps[0], expected);

        state.design.document_mut_for_test().bus_taps[0].orientation = BusTapOrientation::Left;
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
            .design
            .document_mut_for_test()
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
            .design
            .document()
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
        assert_eq!(state.design.document().bus_taps[0], expected);
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
            .design
            .document()
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
        assert_eq!(state.design.document().bus_taps[0], expected);
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
            .design
            .document()
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
            .design
            .document_mut_for_test()
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
            .design
            .document()
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
            .design
            .document_mut_for_test()
            .buses
            .retain(|bus| bus.id == vector_source || bus.id == first_target);
        vector_state
            .design
            .document_mut_for_test()
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
        let before_buses = state.design.document().buses.clone();
        let before_taps = state.design.document().bus_taps.clone();
        let expected = state
            .design
            .document()
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
                .design
                .document()
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
                .design
                .document()
                .buses
                .iter()
                .find(|bus| bus.id == source)
                .unwrap()
                .declaration,
            Some(BusDeclaration::parse("RENAMED[3:0]").unwrap())
        );
        assert_eq!(
            state.design.document().bus_taps[0].slice,
            BusSlice::parse("RENAMED[3:0]").unwrap()
        );
        assert_eq!(state.history().undo_count(), 1);
        let after_buses = state.design.document().buses.clone();
        let after_taps = state.design.document().bus_taps.clone();
        assert!(state.undo());
        assert_eq!(state.design.document().buses, before_buses);
        assert_eq!(state.design.document().bus_taps, before_taps);
        assert_eq!(
            state
                .design
                .document()
                .buses
                .iter()
                .find(|bus| bus.id == destination)
                .unwrap(),
            &expected
        );
        assert!(state.redo());
        assert_eq!(state.design.document().buses, after_buses);
        assert_eq!(state.design.document().bus_taps, after_taps);
        assert_eq!(
            state.design.document().bus_taps[0].slice,
            BusSlice::parse("RENAMED[3:0]").unwrap()
        );
    }

    #[test]
    fn identical_bus_property_commit_is_a_clean_noop() {
        let mut state = SchematicState::default();
        declared_bus(&mut state);
        state.clear_undo_history();
        state.session.is_dirty = false;
        let version = state.topology_version();
        let expected = state.design.document().buses[0].clone();
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
        assert!(!state.session.is_dirty);
        assert_eq!(state.history().undo_count(), 0);
    }
}
