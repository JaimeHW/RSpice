use crate::state::{Bus, BusDeclaration, BusTapOrientation, BusTargetKind, Point, SchematicState};

use super::drawing::nearest_bus_hit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BusTapCandidate {
    pub bus_id: u64,
    pub bus_point: Point,
    pub connection_point: Point,
    pub orientation: BusTapOrientation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum BusTapCandidateError {
    MissingConfiguration,
    NoSourceBus,
    SourceDeclarationConflict(String),
    NoScalarWire,
    NoCompatibleBus,
    MixedScalarBusTarget,
    TargetDeclarationConflict(String),
}

impl BusTapCandidateError {
    pub(super) fn message(&self) -> String {
        match self {
            Self::MissingConfiguration => {
                "Open Place bus tap and validate the bus declaration first.".to_owned()
            }
            Self::NoSourceBus => "Move the pointer onto the source bus.".to_owned(),
            Self::SourceDeclarationConflict(found) => {
                format!("The selected source is declared {found}; it does not match the armed bus.")
            }
            Self::NoScalarWire => {
                "A scalar tap must terminate on an ordinary wire in the selected direction."
                    .to_owned()
            }
            Self::NoCompatibleBus => {
                "A multi-bit tap must terminate on a bus with the exact selected declaration."
                    .to_owned()
            }
            Self::MixedScalarBusTarget => {
                "Scalar wires and buses cannot share an untyped tap endpoint.".to_owned()
            }
            Self::TargetDeclarationConflict(found) => format!(
                "The destination bus is declared {found}; its type does not match the selected slice."
            ),
        }
    }
}

pub(super) fn resolve_bus_tap_candidate(
    schematic: &SchematicState,
    requested: Point,
    source_hit_radius: i32,
) -> Result<BusTapCandidate, BusTapCandidateError> {
    let pending = schematic
        .pending_bus_tap
        .as_ref()
        .ok_or(BusTapCandidateError::MissingConfiguration)?;
    let hit = nearest_bus_hit(&schematic.buses, requested, source_hit_radius.max(0))
        .ok_or(BusTapCandidateError::NoSourceBus)?;
    let source = schematic
        .buses
        .iter()
        .find(|bus| bus.id == hit.bus_id)
        .ok_or(BusTapCandidateError::NoSourceBus)?;

    if let Some(declaration) = &source.declaration
        && declaration != &pending.bus_declaration
    {
        return Err(BusTapCandidateError::SourceDeclarationConflict(
            declaration.to_string(),
        ));
    }
    pending
        .bus_declaration
        .validate_slice(&pending.slice)
        .map_err(|_| {
            BusTapCandidateError::SourceDeclarationConflict(pending.bus_declaration.to_string())
        })?;

    let directions = candidate_directions(
        pending.orientation,
        requested,
        hit.point,
        hit.segment_start,
        hit.segment_end,
    );
    let target = match pending.slice.target_kind() {
        BusTargetKind::Wire => {
            resolve_scalar_target(schematic, hit.bus_id, hit.point, &directions)?
        }
        BusTargetKind::Bus => resolve_bus_target(
            schematic,
            hit.bus_id,
            hit.point,
            &directions,
            &pending.slice,
        )?,
    };

    Ok(BusTapCandidate {
        bus_id: hit.bus_id,
        bus_point: hit.point,
        connection_point: target.0,
        orientation: target.1,
    })
}

fn resolve_scalar_target(
    schematic: &SchematicState,
    source_bus_id: u64,
    source: Point,
    directions: &[BusTapOrientation],
) -> Result<(Point, BusTapOrientation), BusTapCandidateError> {
    for &orientation in directions {
        let mut candidates: Vec<(i64, Point)> = schematic
            .wires
            .iter()
            .flat_map(|wire| wire.points.windows(2))
            .filter_map(|segment| {
                ray_segment_intersection(source, orientation, segment[0], segment[1])
            })
            .collect();
        candidates.sort_by_key(|candidate| (candidate.0, candidate.1.x, candidate.1.y));
        if let Some((_, point)) = candidates.into_iter().next() {
            if schematic.buses.iter().any(|bus| bus.contains_point(point)) {
                return Err(BusTapCandidateError::MixedScalarBusTarget);
            }
            return Ok((point, orientation));
        }
    }

    if ray_hits_other_bus(schematic, source_bus_id, source, directions) {
        Err(BusTapCandidateError::MixedScalarBusTarget)
    } else {
        Err(BusTapCandidateError::NoScalarWire)
    }
}

fn resolve_bus_target(
    schematic: &SchematicState,
    source_bus_id: u64,
    source: Point,
    directions: &[BusTapOrientation],
    slice: &crate::state::BusSlice,
) -> Result<(Point, BusTapOrientation), BusTapCandidateError> {
    let expected = BusDeclaration::new(slice.name.clone(), slice.msb, slice.lsb, slice.notation)
        .map_err(|_| BusTapCandidateError::NoCompatibleBus)?;
    for &orientation in directions {
        let mut candidates: Vec<(i64, Point, &Bus)> = schematic
            .buses
            .iter()
            .filter(|bus| bus.id != source_bus_id)
            .flat_map(|bus| {
                bus.points.windows(2).filter_map(move |segment| {
                    ray_segment_intersection(source, orientation, segment[0], segment[1])
                        .map(|(distance, point)| (distance, point, bus))
                })
            })
            .collect();
        candidates.sort_by_key(|candidate| (candidate.0, candidate.2.id));
        if let Some((_, point, bus)) = candidates.into_iter().next() {
            let Some(declaration) = &bus.declaration else {
                return Err(BusTapCandidateError::TargetDeclarationConflict(
                    "unnamed".to_owned(),
                ));
            };
            if declaration != &expected {
                return Err(BusTapCandidateError::TargetDeclarationConflict(
                    declaration.to_string(),
                ));
            }
            if schematic
                .wires
                .iter()
                .any(|wire| wire.contains_point(point))
            {
                return Err(BusTapCandidateError::MixedScalarBusTarget);
            }
            return Ok((point, orientation));
        }
    }

    if directions.iter().any(|&orientation| {
        schematic.wires.iter().any(|wire| {
            wire.points.windows(2).any(|segment| {
                ray_segment_intersection(source, orientation, segment[0], segment[1]).is_some()
            })
        })
    }) {
        Err(BusTapCandidateError::MixedScalarBusTarget)
    } else {
        Err(BusTapCandidateError::NoCompatibleBus)
    }
}

fn ray_hits_other_bus(
    schematic: &SchematicState,
    source_bus_id: u64,
    source: Point,
    directions: &[BusTapOrientation],
) -> bool {
    directions.iter().any(|&orientation| {
        schematic.buses.iter().any(|bus| {
            bus.id != source_bus_id
                && bus.points.windows(2).any(|segment| {
                    ray_segment_intersection(source, orientation, segment[0], segment[1]).is_some()
                })
        })
    })
}

fn candidate_directions(
    configured: BusTapOrientation,
    cursor: Point,
    source: Point,
    segment_start: Point,
    segment_end: Point,
) -> Vec<BusTapOrientation> {
    if configured != BusTapOrientation::Automatic {
        return vec![configured];
    }

    let mut directions = if segment_start.y == segment_end.y {
        if cursor.y < source.y {
            vec![BusTapOrientation::Up, BusTapOrientation::Down]
        } else {
            vec![BusTapOrientation::Down, BusTapOrientation::Up]
        }
    } else if segment_start.x == segment_end.x {
        if cursor.x < source.x {
            vec![BusTapOrientation::Left, BusTapOrientation::Right]
        } else {
            vec![BusTapOrientation::Right, BusTapOrientation::Left]
        }
    } else {
        vec![
            BusTapOrientation::Right,
            BusTapOrientation::Down,
            BusTapOrientation::Left,
            BusTapOrientation::Up,
        ]
    };

    // If the cursor is exactly on the source, search all remaining directions
    // after the segment-normal pair. This keeps Automatic useful at a snapped
    // coordinate without changing an explicitly requested orientation.
    if cursor == source {
        for orientation in [
            BusTapOrientation::Left,
            BusTapOrientation::Right,
            BusTapOrientation::Up,
            BusTapOrientation::Down,
        ] {
            if !directions.contains(&orientation) {
                directions.push(orientation);
            }
        }
    }
    directions
}

fn ray_segment_intersection(
    source: Point,
    orientation: BusTapOrientation,
    start: Point,
    end: Point,
) -> Option<(i64, Point)> {
    let candidate = match orientation {
        BusTapOrientation::Left | BusTapOrientation::Right => {
            if start.y == end.y && start.y == source.y {
                let x = match orientation {
                    BusTapOrientation::Left => start.x.max(end.x).min(source.x.saturating_sub(1)),
                    BusTapOrientation::Right => start.x.min(end.x).max(source.x.saturating_add(1)),
                    _ => unreachable!(),
                };
                let low = start.x.min(end.x);
                let high = start.x.max(end.x);
                (low..=high).contains(&x).then_some(Point::new(x, source.y))
            } else {
                exact_horizontal_intersection(source.y, start, end)
            }
        }
        BusTapOrientation::Up | BusTapOrientation::Down => {
            if start.x == end.x && start.x == source.x {
                let y = match orientation {
                    BusTapOrientation::Up => start.y.max(end.y).min(source.y.saturating_sub(1)),
                    BusTapOrientation::Down => start.y.min(end.y).max(source.y.saturating_add(1)),
                    _ => unreachable!(),
                };
                let low = start.y.min(end.y);
                let high = start.y.max(end.y);
                (low..=high).contains(&y).then_some(Point::new(source.x, y))
            } else {
                exact_vertical_intersection(source.x, start, end)
            }
        }
        BusTapOrientation::Automatic => None,
    }?;

    let distance = (i64::from(candidate.x) - i64::from(source.x)).abs()
        + (i64::from(candidate.y) - i64::from(source.y)).abs();
    (distance > 0 && direction_accepts(source, candidate, orientation))
        .then_some((distance, candidate))
}

fn exact_horizontal_intersection(y: i32, start: Point, end: Point) -> Option<Point> {
    let dy = i128::from(end.y) - i128::from(start.y);
    if dy == 0 {
        return None;
    }
    let offset = i128::from(y) - i128::from(start.y);
    if !ratio_is_on_segment(offset, dy) {
        return None;
    }
    let dx = i128::from(end.x) - i128::from(start.x);
    let x_numerator = i128::from(start.x) * dy + offset * dx;
    if x_numerator % dy != 0 {
        return None;
    }
    Some(Point::new(i32::try_from(x_numerator / dy).ok()?, y))
}

fn exact_vertical_intersection(x: i32, start: Point, end: Point) -> Option<Point> {
    let dx = i128::from(end.x) - i128::from(start.x);
    if dx == 0 {
        return None;
    }
    let offset = i128::from(x) - i128::from(start.x);
    if !ratio_is_on_segment(offset, dx) {
        return None;
    }
    let dy = i128::from(end.y) - i128::from(start.y);
    let y_numerator = i128::from(start.y) * dx + offset * dy;
    if y_numerator % dx != 0 {
        return None;
    }
    Some(Point::new(x, i32::try_from(y_numerator / dx).ok()?))
}

fn ratio_is_on_segment(numerator: i128, denominator: i128) -> bool {
    if denominator > 0 {
        (0..=denominator).contains(&numerator)
    } else {
        (denominator..=0).contains(&numerator)
    }
}

fn direction_accepts(source: Point, candidate: Point, orientation: BusTapOrientation) -> bool {
    match orientation {
        BusTapOrientation::Left => candidate.x < source.x && candidate.y == source.y,
        BusTapOrientation::Right => candidate.x > source.x && candidate.y == source.y,
        BusTapOrientation::Up => candidate.y < source.y && candidate.x == source.x,
        BusTapOrientation::Down => candidate.y > source.y && candidate.x == source.x,
        BusTapOrientation::Automatic => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{BusSlice, Wire};

    fn armed_state(slice: &str) -> SchematicState {
        let declaration = BusDeclaration::parse("DATA[7:0]").unwrap();
        let mut state = SchematicState::default();
        state.buses.push(
            Bus::segment(
                1,
                Point::new(0, 0),
                Point::new(40, 0),
                Some(declaration.clone()),
            )
            .unwrap(),
        );
        state.pending_bus_tap = Some(
            crate::state::PendingBusTap::new(
                declaration,
                BusSlice::parse(slice).unwrap(),
                BusTapOrientation::Automatic,
            )
            .unwrap(),
        );
        state
    }

    #[test]
    fn scalar_tap_resolves_only_to_scalar_wire() {
        let mut state = armed_state("DATA[3]");
        state
            .wires
            .push(Wire::segment(2, Point::new(20, 10), Point::new(30, 10)));

        let candidate = resolve_bus_tap_candidate(&state, Point::new(20, 1), 2).unwrap();

        assert_eq!(candidate.bus_point, Point::new(20, 0));
        assert_eq!(candidate.connection_point, Point::new(20, 10));
        assert_eq!(candidate.orientation, BusTapOrientation::Down);
    }

    #[test]
    fn multi_bit_tap_rejects_scalar_target_instead_of_collapsing() {
        let mut state = armed_state("DATA[3:0]");
        state
            .wires
            .push(Wire::segment(2, Point::new(20, 10), Point::new(30, 10)));

        assert_eq!(
            resolve_bus_tap_candidate(&state, Point::new(20, 1), 2),
            Err(BusTapCandidateError::MixedScalarBusTarget)
        );
    }

    #[test]
    fn multi_bit_tap_requires_exact_destination_declaration() {
        let mut state = armed_state("DATA[3:0]");
        state.buses.push(
            Bus::segment(
                2,
                Point::new(0, 10),
                Point::new(40, 10),
                Some(BusDeclaration::parse("ADDR[3:0]").unwrap()),
            )
            .unwrap(),
        );

        assert!(matches!(
            resolve_bus_tap_candidate(&state, Point::new(20, 1), 2),
            Err(BusTapCandidateError::TargetDeclarationConflict(_))
        ));
    }

    #[test]
    fn explicit_orientation_is_never_auto_flipped() {
        let mut state = armed_state("DATA[3]");
        state.pending_bus_tap.as_mut().unwrap().orientation = BusTapOrientation::Up;
        state
            .wires
            .push(Wire::segment(2, Point::new(20, 10), Point::new(30, 10)));

        assert_eq!(
            resolve_bus_tap_candidate(&state, Point::new(20, 1), 2),
            Err(BusTapCandidateError::NoScalarWire)
        );
    }

    #[test]
    fn source_hit_radius_does_not_capture_a_nearby_bus() {
        let mut state = armed_state("DATA[3]");
        state.buses.push(
            Bus::segment(
                3,
                Point::new(0, 6),
                Point::new(40, 6),
                Some(BusDeclaration::parse("ADDR[7:0]").unwrap()),
            )
            .unwrap(),
        );
        state
            .wires
            .push(Wire::segment(2, Point::new(20, 10), Point::new(30, 10)));

        assert_eq!(
            resolve_bus_tap_candidate(&state, Point::new(20, 4), 1),
            Err(BusTapCandidateError::NoSourceBus)
        );
    }

    #[test]
    fn ray_intersection_is_safe_at_extreme_coordinates() {
        assert_eq!(
            ray_segment_intersection(
                Point::new(i32::MIN, 0),
                BusTapOrientation::Left,
                Point::new(i32::MIN, 0),
                Point::new(i32::MAX, 0),
            ),
            None
        );
        assert_eq!(
            ray_segment_intersection(
                Point::new(i32::MIN, 0),
                BusTapOrientation::Right,
                Point::new(i32::MAX, i32::MIN),
                Point::new(i32::MAX, i32::MAX),
            ),
            Some((i64::from(u32::MAX), Point::new(i32::MAX, 0)))
        );
    }

    #[test]
    fn non_lattice_source_projection_preview_always_commits_exactly() {
        let declaration = BusDeclaration::parse("DATA[7:0]").unwrap();
        let mut state = SchematicState::default();
        state.buses.push(
            Bus::segment(
                1,
                Point::new(0, 0),
                Point::new(10, 3),
                Some(declaration.clone()),
            )
            .unwrap(),
        );
        state
            .wires
            .push(Wire::segment(2, Point::new(20, 3), Point::new(20, 12)));
        let pending = crate::state::PendingBusTap::new(
            declaration,
            BusSlice::parse("DATA[3]").unwrap(),
            BusTapOrientation::Automatic,
        )
        .unwrap();
        state.pending_bus_tap = Some(pending.clone());

        let candidate = resolve_bus_tap_candidate(&state, Point::new(5, 2), 6).unwrap();
        assert_eq!(candidate.bus_point, Point::new(10, 3));
        assert!(state.buses[0].contains_point(candidate.bus_point));
        assert!(
            state
                .place_configured_bus_tap(
                    candidate.bus_id,
                    candidate.bus_point,
                    candidate.connection_point,
                    &pending,
                )
                .is_ok()
        );
    }

    #[test]
    fn scalar_tap_resolves_an_exact_lattice_crossing_on_a_diagonal_wire() {
        let mut state = armed_state("DATA[3]");
        state
            .wires
            .push(Wire::segment(2, Point::new(0, 10), Point::new(20, 20)));

        let candidate = resolve_bus_tap_candidate(&state, Point::new(10, 1), 2).unwrap();

        assert_eq!(candidate.bus_point, Point::new(10, 0));
        assert_eq!(candidate.connection_point, Point::new(10, 15));
        assert_eq!(candidate.orientation, BusTapOrientation::Down);
    }

    #[test]
    fn diagonal_target_without_an_integer_crossing_is_rejected() {
        let mut state = armed_state("DATA[3]");
        state
            .wires
            .push(Wire::segment(2, Point::new(0, 10), Point::new(3, 20)));

        assert_eq!(
            resolve_bus_tap_candidate(&state, Point::new(1, 1), 2),
            Err(BusTapCandidateError::NoScalarWire)
        );
    }
}
