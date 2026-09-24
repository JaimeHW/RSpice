//! Typed bus connectivity projection shared by DRC and netlisting.
//!
//! Scalar taps become exact member-name aliases on ordinary scalar wires.
//! A multi-bit tap needs no alias of its own: a slice keeps its bus's base
//! name, and a multi-bit tap is only well formed when the bus it lands on
//! declares exactly that slice, so the vector projection names both ends'
//! conductors with the same [`deck_bit_name`] and the two are already one set
//! of nodes. What this pass owes a multi-bit tap is therefore the judgement —
//! `MixedTap` when it terminates on scalar geometry, `DanglingTap` when it
//! terminates on nothing, `RangeConflict` when the destination declares
//! something else — and never a second naming of bits the projection has
//! already named.
//!
//! Notation is a spelling of the drawing, never a separate conductor: the deck
//! names a member without its delimiters, so one stem declared both ways is
//! one net there however far apart the two declarations were drawn. The deck's
//! merge is the truth and is left alone; this pass is what states it on the
//! design, so a bus never silently joins another bus of its own name.
//!
//! A scalar projection carries both spellings of its bit, because they answer
//! different questions. `member_name` is what the user drew — it is compared
//! against authored net labels and shown in diagnostics and DRC. `deck_name` is
//! the node identity headed for the netlist, where the authored `<n>`/`[n]`
//! delimiters do not survive a probe. Collapsing the two would either put `#`
//! in front of the user or put `[` in front of the engine. Every message this
//! module writes quotes the authored spelling, which is the one the drawing
//! shows.

use std::collections::{HashMap, HashSet};

use crate::simulation::netlist_gen::deck_bit_name;
use crate::state::{Bus, BusDeclaration, BusNotation, BusTargetKind, Point, SchematicState};

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
    /// Authored spelling, e.g. `DATA[3]` — matched against net labels and shown
    /// to the user.
    pub member_name: String,
    /// Deck spelling, e.g. `DATA#3` — the node identity the netlist carries.
    pub deck_name: String,
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
    let mut stems_conflicting_where_they_touch: HashSet<&str> = HashSet::new();
    for (index, left) in schematic.buses.iter().enumerate() {
        for right in &schematic.buses[index + 1..] {
            let Some(point) = bus_intersection(left, right) else {
                continue;
            };
            if left.declaration != right.declaration {
                if let (Some(left_declaration), Some(right_declaration)) =
                    (left.declaration.as_ref(), right.declaration.as_ref())
                    && left_declaration.name == right_declaration.name
                    && left_declaration.notation != right_declaration.notation
                {
                    stems_conflicting_where_they_touch.insert(left_declaration.name.as_str());
                }
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

    analysis.diagnostics.extend(two_notation_stem_diagnostics(
        &schematic.buses,
        &stems_conflicting_where_they_touch,
    ));

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
                        deck_name: deck_bit_name(&tap.slice.name, tap.slice.msb),
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

/// One finding per stem the design declares in both notations.
///
/// [`deck_bit_name`] spells a member the same way whichever delimiters the
/// drawing used, so `DATA[3:0]` and `DATA<3:0>` mint the same conductors and
/// the engine solves them as one net — wherever on the design they were drawn,
/// and whether or not any geometry joins them. The merge is the deck's truth
/// and stays; what the drawing owes its reader is a statement of it, which is
/// this finding. Declarations that touch are already judged as one connection
/// by the rule above and are left to it, so a stem is named here exactly once
/// and by exactly one rule.
fn two_notation_stem_diagnostics(
    buses: &[Bus],
    stems_conflicting_where_they_touch: &HashSet<&str>,
) -> Vec<BusDiagnostic> {
    let mut stems: Vec<(&str, Option<&Bus>, Option<&Bus>)> = Vec::new();
    for bus in buses {
        let Some(declaration) = bus.declaration.as_ref() else {
            continue;
        };
        let stem = declaration.name.as_str();
        if !stems.iter().any(|(name, _, _)| *name == stem) {
            stems.push((stem, None, None));
        }
        let entry = stems
            .iter_mut()
            .find(|(name, _, _)| *name == stem)
            .expect("the stem is present");
        let slot = match declaration.notation {
            BusNotation::Square => &mut entry.1,
            BusNotation::Angle => &mut entry.2,
        };
        if slot.is_none() {
            *slot = Some(bus);
        }
    }

    stems
        .into_iter()
        .filter(|(stem, _, _)| !stems_conflicting_where_they_touch.contains(*stem))
        .filter_map(|(stem, square, angle)| {
            let (square, angle) = (square?, angle?);
            let (left, right) = if square.id <= angle.id {
                (square, angle)
            } else {
                (angle, square)
            };
            Some(BusDiagnostic {
                kind: BusDiagnosticKind::RangeConflict,
                message: format!(
                    "Buses #{} and #{} declare {stem} in two notations ({} versus {}); \
                     the deck merges them into one net.",
                    left.id,
                    right.id,
                    declaration_label(left.declaration.as_ref()),
                    declaration_label(right.declaration.as_ref())
                ),
                point: right.points.first().copied().unwrap_or(Point::origin()),
                bus_id: Some(right.id),
                tap_id: None,
            })
        })
        .collect()
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
        assert!(dangling.diagnostics.iter().any(|item| {
            // A diagnostic is read by whoever drew the tap, so it stays in the
            // spelling they drew.
            item.kind == BusDiagnosticKind::DanglingTap && item.message.contains("DATA[3]")
        }));

        schematic
            .wires
            .push(Wire::segment(3, Point::new(20, 10), Point::new(40, 10)));
        let connected = analyze_bus_connectivity(&schematic);
        assert_eq!(connected.scalar_taps.len(), 1);
        // The user's spelling is preserved for labels, diagnostics and DRC; the
        // deck carries the bit under a name the engine can lex and probe.
        assert_eq!(connected.scalar_taps[0].member_name, "DATA[3]");
        assert_eq!(connected.scalar_taps[0].deck_name, "DATA#3");
    }

    #[test]
    fn a_well_formed_multi_bit_tap_is_judged_and_left_to_the_vector_projection() {
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

        assert!(analysis.diagnostics.is_empty());
        // No scalar binding, because there is nothing left to bind: the slice
        // and the destination declaration share a base, so the four conductors
        // this tap selects are already named `DATA#3`..`DATA#0` at both ends by
        // the vector projection. That the two ends really are one set of nodes
        // is proven where it is decided, on the emitted deck.
        assert!(analysis.scalar_taps.is_empty());
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

    /// Touching declarations are judged by the connection they make, and that
    /// judgement is not doubled by the stem rule.
    #[test]
    fn touching_two_notation_buses_keep_exactly_the_connection_finding() {
        let mut schematic = SchematicState::default();
        schematic.buses = vec![
            declared_bus(1, "DATA[3:0]", 0),
            declared_bus(2, "DATA<3:0>", 0),
        ];

        let diagnostics = analyze_bus_connectivity(&schematic).diagnostics;

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].kind, BusDiagnosticKind::RangeConflict);
        assert!(
            diagnostics[0]
                .message
                .starts_with("Touching buses #1 and #2"),
            "{}",
            diagnostics[0].message
        );
    }

    /// The hole the deck opens: nothing joins these two declarations on the
    /// design, and `deck_bit_name` joins them anyway.
    #[test]
    fn a_stem_declared_in_two_notations_is_reported_even_where_nothing_touches() {
        let mut schematic = SchematicState::default();
        schematic.buses = vec![
            declared_bus(1, "DATA[3:0]", 0),
            declared_bus(2, "DATA<3:0>", 40),
        ];

        let diagnostics = analyze_bus_connectivity(&schematic).diagnostics;

        assert_eq!(diagnostics.len(), 1);
        let finding = &diagnostics[0];
        assert_eq!(finding.kind, BusDiagnosticKind::RangeConflict);
        // Both declarations are named, so the reader can find each end, and
        // the finding is located on one of them.
        assert!(
            finding.message.contains("Buses #1 and #2")
                && finding.message.contains("DATA[3:0]")
                && finding.message.contains("DATA<3:0>"),
            "{}",
            finding.message
        );
        assert_eq!(finding.bus_id, Some(2));
        assert_eq!(finding.point, Point::new(0, 40));
    }

    /// Sheets are a membership overlay on one drawing, so two sheets of a cell
    /// view spelling one stem two ways is the same design fact — and the same
    /// single finding — as two disconnected buses on one sheet.
    #[test]
    fn two_notation_stems_are_reported_once_however_far_apart_they_are_drawn() {
        let mut schematic = SchematicState::default();
        schematic.buses = vec![
            declared_bus(1, "DATA[7:0]", 0),
            declared_bus(2, "DATA[7:0]", 40),
            declared_bus(3, "DATA<3:0>", 80),
            declared_bus(4, "DATA<3:0>", 120),
        ];

        let diagnostics = analyze_bus_connectivity(&schematic).diagnostics;

        assert_eq!(diagnostics.len(), 1);
        assert!(
            diagnostics[0].message.contains("Buses #1 and #3"),
            "{}",
            diagnostics[0].message
        );
    }

    /// One notation is one spelling: separate declarations of the same stem are
    /// how a bus is drawn across a design, and say nothing on their own.
    #[test]
    fn one_notation_across_the_design_reports_nothing() {
        let mut schematic = SchematicState::default();
        schematic.buses = vec![
            declared_bus(1, "DATA[7:0]", 0),
            declared_bus(2, "DATA[3:0]", 40),
            declared_bus(3, "ADDR<7:0>", 80),
        ];

        assert!(analyze_bus_connectivity(&schematic).diagnostics.is_empty());
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
