//! Typed bus connectivity projection shared by DRC and netlisting.
//!
//! Scalar taps become exact member-name aliases on ordinary scalar wires.
//! Multi-bit taps remain vector-only contracts and are never projected into
//! the scalar node graph.

use std::collections::HashMap;

use crate::state::{Bus, BusDeclaration, BusTargetKind, Point, SchematicState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BusDiagnosticKind {
    MalformedBus,
    UnnamedBus,
    RangeConflict,
    DanglingTap,
    MixedTap,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BusDiagnostic {
    pub kind: BusDiagnosticKind,
    pub message: String,
    pub point: Point,
    pub bus_id: Option<u64>,
    pub tap_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ScalarTapBinding {
    pub tap_id: u64,
    pub member_name: String,
    pub point: Point,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct BusConnectivityAnalysis {
    pub scalar_taps: Vec<ScalarTapBinding>,
    pub diagnostics: Vec<BusDiagnostic>,
}

pub(crate) fn analyze_bus_connectivity(schematic: &SchematicState) -> BusConnectivityAnalysis {
    let mut analysis = BusConnectivityAnalysis::default();
    let buses_by_id: HashMap<u64, &Bus> = schematic.buses.iter().map(|bus| (bus.id, bus)).collect();

    for bus in &schematic.buses {
        if let Err(error) = bus.validate() {
            analysis.diagnostics.push(BusDiagnostic {
                kind: BusDiagnosticKind::MalformedBus,
                message: format!("Bus #{} is malformed: {error}.", bus.id),
                point: bus.points.first().copied().unwrap_or(Point::origin()),
                bus_id: Some(bus.id),
                tap_id: None,
            });
        } else if bus.declaration.is_none() {
            analysis.diagnostics.push(BusDiagnostic {
                kind: BusDiagnosticKind::UnnamedBus,
                message: format!(
                    "Bus #{} has no typed declaration; assign a name and range before sign-off.",
                    bus.id
                ),
                point: bus.points.first().copied().unwrap_or(Point::origin()),
                bus_id: Some(bus.id),
                tap_id: None,
            });
        }
    }

    // Touching bus geometry is one vector connection and therefore requires
    // exactly identical declarations.
    for (index, left) in schematic.buses.iter().enumerate() {
        for right in &schematic.buses[index + 1..] {
            let Some(point) = bus_intersection(left, right) else {
                continue;
            };
            if left.declaration != right.declaration {
                analysis.diagnostics.push(BusDiagnostic {
                    kind: BusDiagnosticKind::RangeConflict,
                    message: format!(
                        "Touching buses #{} and #{} have incompatible declarations ({} versus {}).",
                        left.id,
                        right.id,
                        declaration_label(left.declaration.as_ref()),
                        declaration_label(right.declaration.as_ref())
                    ),
                    point,
                    bus_id: Some(right.id),
                    tap_id: None,
                });
            }
        }
    }

    for tap in &schematic.bus_taps {
        let Some(source) = buses_by_id.get(&tap.bus_id).copied() else {
            analysis.diagnostics.push(BusDiagnostic {
                kind: BusDiagnosticKind::DanglingTap,
                message: format!("Bus tap #{} references a missing source bus.", tap.id),
                point: tap.bus_point,
                bus_id: None,
                tap_id: Some(tap.id),
            });
            continue;
        };
        if let Err(error) = tap.validate_against_bus(source) {
            analysis.diagnostics.push(BusDiagnostic {
                kind: BusDiagnosticKind::RangeConflict,
                message: format!("Bus tap #{} is invalid: {error}.", tap.id),
                point: tap.bus_point,
                bus_id: Some(source.id),
                tap_id: Some(tap.id),
            });
            continue;
        }

        let touches_wire = schematic
            .wires
            .iter()
            .any(|wire| wire.contains_point(tap.connection_point));
        let destination_buses: Vec<&Bus> = schematic
            .buses
            .iter()
            .filter(|bus| bus.id != source.id && bus.contains_point(tap.connection_point))
            .collect();

        match tap.target_kind() {
            BusTargetKind::Wire => {
                if !destination_buses.is_empty() {
                    analysis.diagnostics.push(BusDiagnostic {
                        kind: BusDiagnosticKind::MixedTap,
                        message: format!(
                            "Scalar bus tap {} terminates on bus geometry; connect it to one ordinary wire.",
                            tap.slice
                        ),
                        point: tap.connection_point,
                        bus_id: Some(source.id),
                        tap_id: Some(tap.id),
                    });
                } else if !touches_wire {
                    analysis.diagnostics.push(BusDiagnostic {
                        kind: BusDiagnosticKind::DanglingTap,
                        message: format!(
                            "Scalar bus tap {} does not terminate on an ordinary wire.",
                            tap.slice
                        ),
                        point: tap.connection_point,
                        bus_id: Some(source.id),
                        tap_id: Some(tap.id),
                    });
                } else {
                    analysis.scalar_taps.push(ScalarTapBinding {
                        tap_id: tap.id,
                        member_name: tap.slice.to_string(),
                        point: tap.connection_point,
                    });
                }
            }
            BusTargetKind::Bus => {
                if touches_wire {
                    analysis.diagnostics.push(BusDiagnostic {
                        kind: BusDiagnosticKind::MixedTap,
                        message: format!(
                            "Multi-bit bus tap {} terminates on scalar wire geometry.",
                            tap.slice
                        ),
                        point: tap.connection_point,
                        bus_id: Some(source.id),
                        tap_id: Some(tap.id),
                    });
                    continue;
                }
                if destination_buses.is_empty() {
                    analysis.diagnostics.push(BusDiagnostic {
                        kind: BusDiagnosticKind::DanglingTap,
                        message: format!(
                            "Multi-bit bus tap {} does not terminate on another bus.",
                            tap.slice
                        ),
                        point: tap.connection_point,
                        bus_id: Some(source.id),
                        tap_id: Some(tap.id),
                    });
                    continue;
                }
                let expected = BusDeclaration::new(
                    tap.slice.name.clone(),
                    tap.slice.msb,
                    tap.slice.lsb,
                    tap.slice.notation,
                )
                .expect("multi-bit validated slice is a valid declaration");
                if !destination_buses
                    .iter()
                    .any(|bus| bus.declaration.as_ref() == Some(&expected))
                {
                    analysis.diagnostics.push(BusDiagnostic {
                        kind: BusDiagnosticKind::RangeConflict,
                        message: format!(
                            "Bus tap {} requires destination {}; found {}.",
                            tap.slice,
                            expected,
                            destination_buses
                                .iter()
                                .map(|bus| declaration_label(bus.declaration.as_ref()))
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                        point: tap.connection_point,
                        bus_id: Some(source.id),
                        tap_id: Some(tap.id),
                    });
                }
            }
        }
    }

    analysis
}

fn declaration_label(declaration: Option<&BusDeclaration>) -> String {
    declaration.map_or_else(|| "unnamed".to_owned(), ToString::to_string)
}

fn bus_intersection(left: &Bus, right: &Bus) -> Option<Point> {
    left.points.windows(2).find_map(|a| {
        right.points.windows(2).find_map(|b| {
            let point = segment_intersection(a[0], a[1], b[0], b[1])?;
            // Match scalar schematic topology: a shared endpoint or a
            // T-contact is connected, while a pure interior/interior X
            // crossing is only a visual crossing. Vector connectivity must
            // never be invented merely because two strokes overlap on screen.
            let endpoint_contact = ([a[0], a[1]].contains(&point)
                && point_on_segment(point, b[0], b[1]))
                || ([b[0], b[1]].contains(&point) && point_on_segment(point, a[0], a[1]));
            endpoint_contact.then_some(point)
        })
    })
}

fn segment_intersection(a0: Point, a1: Point, b0: Point, b1: Point) -> Option<Point> {
    let ax = i128::from(a0.x);
    let ay = i128::from(a0.y);
    let rx = i128::from(a1.x) - ax;
    let ry = i128::from(a1.y) - ay;
    let qx = i128::from(b0.x);
    let qy = i128::from(b0.y);
    let sx = i128::from(b1.x) - qx;
    let sy = i128::from(b1.y) - qy;
    let denominator = cross(rx, ry, sx, sy);
    let qpx = qx - ax;
    let qpy = qy - ay;

    if denominator == 0 {
        if cross(qpx, qpy, rx, ry) != 0 {
            return None;
        }
        return [a0, a1, b0, b1]
            .into_iter()
            .find(|point| point_on_segment(*point, a0, a1) && point_on_segment(*point, b0, b1));
    }

    let t_numerator = cross(qpx, qpy, sx, sy);
    let u_numerator = cross(qpx, qpy, rx, ry);
    if !ratio_in_unit_interval(t_numerator, denominator)
        || !ratio_in_unit_interval(u_numerator, denominator)
    {
        return None;
    }

    Some(Point::new(
        rational_coordinate(ax, rx, t_numerator, denominator),
        rational_coordinate(ay, ry, t_numerator, denominator),
    ))
}

fn cross(ax: i128, ay: i128, bx: i128, by: i128) -> i128 {
    ax * by - ay * bx
}

fn ratio_in_unit_interval(numerator: i128, denominator: i128) -> bool {
    if denominator > 0 {
        (0..=denominator).contains(&numerator)
    } else {
        (denominator..=0).contains(&numerator)
    }
}

fn rational_coordinate(origin: i128, delta: i128, numerator: i128, denominator: i128) -> i32 {
    let value = (origin * denominator + delta * numerator) / denominator;
    value.clamp(i128::from(i32::MIN), i128::from(i32::MAX)) as i32
}

fn point_on_segment(point: Point, start: Point, end: Point) -> bool {
    let px = i128::from(point.x);
    let py = i128::from(point.y);
    let sx = i128::from(start.x);
    let sy = i128::from(start.y);
    let ex = i128::from(end.x);
    let ey = i128::from(end.y);
    let cross = (px - sx) * (ey - sy) - (py - sy) * (ex - sx);
    cross == 0 && px >= sx.min(ex) && px <= sx.max(ex) && py >= sy.min(ey) && py <= sy.max(ey)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{BusSlice, BusTap, BusTapOrientation, Wire};

    fn declared_bus(id: u64, name: &str, y: i32) -> Bus {
        Bus::segment(
            id,
            Point::new(0, y),
            Point::new(40, y),
            Some(BusDeclaration::parse(name).unwrap()),
        )
        .unwrap()
    }

    #[test]
    fn scalar_tap_projects_exact_member_name_only_when_wire_terminates_it() {
        let bus = declared_bus(1, "DATA[7:0]", 0);
        let tap = BusTap::new(
            2,
            &bus,
            Point::new(20, 0),
            Point::new(20, 10),
            BusSlice::parse("DATA[3]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        let mut schematic = SchematicState::default();
        schematic.buses.push(bus);
        schematic.bus_taps.push(tap);

        let dangling = analyze_bus_connectivity(&schematic);
        assert!(dangling.scalar_taps.is_empty());
        assert!(
            dangling
                .diagnostics
                .iter()
                .any(|item| item.kind == BusDiagnosticKind::DanglingTap)
        );

        schematic
            .wires
            .push(Wire::segment(3, Point::new(20, 10), Point::new(40, 10)));
        let connected = analyze_bus_connectivity(&schematic);
        assert_eq!(connected.scalar_taps.len(), 1);
        assert_eq!(connected.scalar_taps[0].member_name, "DATA[3]");
    }

    #[test]
    fn multi_bit_tap_never_projects_to_scalar_connectivity() {
        let source = declared_bus(1, "DATA[7:0]", 0);
        let destination = declared_bus(2, "DATA[3:0]", 10);
        let tap = BusTap::new(
            3,
            &source,
            Point::new(20, 0),
            Point::new(20, 10),
            BusSlice::parse("DATA[3:0]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        let mut schematic = SchematicState::default();
        schematic.buses = vec![source, destination];
        schematic.bus_taps.push(tap);

        let analysis = analyze_bus_connectivity(&schematic);

        assert!(analysis.scalar_taps.is_empty());
        assert!(analysis.diagnostics.is_empty());
    }

    #[test]
    fn touching_same_name_with_conflicting_range_is_reported() {
        let mut schematic = SchematicState::default();
        schematic.buses = vec![
            declared_bus(1, "DATA[7:0]", 0),
            declared_bus(2, "DATA[15:0]", 0),
        ];

        assert!(
            analyze_bus_connectivity(&schematic)
                .diagnostics
                .iter()
                .any(|item| item.kind == BusDiagnosticKind::RangeConflict)
        );
    }

    #[test]
    fn pure_interior_bus_crossings_do_not_invent_connectivity() {
        let left = Bus::segment(
            1,
            Point::new(0, 0),
            Point::new(20, 20),
            Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
        )
        .unwrap();
        let right = Bus::segment(
            2,
            Point::new(0, 20),
            Point::new(20, 0),
            Some(BusDeclaration::parse("DATA[15:0]").unwrap()),
        )
        .unwrap();
        let mut schematic = SchematicState::default();
        schematic.buses = vec![left, right];

        let analysis = analyze_bus_connectivity(&schematic);

        assert!(analysis.diagnostics.is_empty());
    }

    #[test]
    fn endpoint_to_bus_contact_is_connected_and_range_checked() {
        let left = Bus::segment(
            1,
            Point::new(0, 0),
            Point::new(20, 20),
            Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
        )
        .unwrap();
        let right = Bus::segment(
            2,
            Point::new(10, 10),
            Point::new(20, 0),
            Some(BusDeclaration::parse("DATA[15:0]").unwrap()),
        )
        .unwrap();
        let mut schematic = SchematicState::default();
        schematic.buses = vec![left, right];

        let analysis = analyze_bus_connectivity(&schematic);

        assert!(analysis.diagnostics.iter().any(|item| {
            item.kind == BusDiagnosticKind::RangeConflict && item.point == Point::new(10, 10)
        }));
    }

    #[test]
    fn extreme_coordinate_intersections_do_not_overflow() {
        let left = Bus::segment(
            1,
            Point::new(i32::MIN, i32::MIN),
            Point::new(i32::MAX, i32::MAX),
            Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
        )
        .unwrap();
        let right = Bus::segment(
            2,
            Point::new(i32::MIN, i32::MAX),
            Point::new(i32::MAX, i32::MIN),
            Some(BusDeclaration::parse("DATA[15:0]").unwrap()),
        )
        .unwrap();
        let mut schematic = SchematicState::default();
        schematic.buses = vec![left, right];

        assert!(analyze_bus_connectivity(&schematic).diagnostics.is_empty());
    }
}
