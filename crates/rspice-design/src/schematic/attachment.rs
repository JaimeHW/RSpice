//! What sits on the geometry an edit moves.
//!
//! A net label, a probe, either end of a bus tap and a junction dot each sit
//! on a pin or a conductor, and that is what they mean: the label names the
//! net there, the tap joins its bit to the wire. An edit that moves the pin
//! or the conductor takes whatever sits on it along, selected or not, so a
//! drag, a Move, a turn, a flip, an Align or a Stretch never leaves a net
//! without its name or a bit off its bus. Each edit says what it moves and
//! where that goes; [`attached`] says what follows.

use super::{
    bus::{Bus, BusTap, BusTargetKind},
    document::SchematicDocument,
    net_label::{Junction, NetLabel},
    probe::SchematicProbe,
    wire::{Wire, WireSegment},
};
use rspice_design_model::Point;
use std::collections::{HashMap, HashSet};

/// A wire or a bus, by id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Conductor {
    Wire(u64),
    Bus(u64),
}

/// What an edit moves, as it stood before the edit.
#[derive(Debug, Clone, Copy)]
pub struct MovingGeometry<'a> {
    /// Points that move: the pins of the parts the edit carries, and any
    /// conductor vertex it carries.
    pub points: &'a HashSet<Point>,
    /// Conductors that move whole.
    pub conductors: &'a HashSet<Conductor>,
    /// Stretches of a conductor that move whole while the rest of it stays,
    /// as Stretch moves one segment.
    pub runs: &'a [(Conductor, WireSegment)],
    /// Whether a wire vertex on one of `points` moves with it. Every edit
    /// carries those but a Move that breaks connections.
    pub carries_vertices: bool,
    /// Pins that stay where they are. What sits on one stays with it.
    pub fixed_points: &'a HashSet<Point>,
}

/// What an attached object follows: a moving point, or a conductor (or the
/// run of it) that moves whole.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    Point(Point),
    Conductor(Conductor),
}

/// One object that can sit on moving geometry, as an edit's selection names
/// it. A selected object moves with the selection, not with its anchor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Attachable {
    NetLabel(u64),
    Probe(u64),
    Junction(Point),
    BusTap(u64),
}

/// The drawing [`attached`] reads: everything an anchor can be and
/// everything that can sit on one.
#[derive(Debug, Clone, Default)]
pub struct Scene<'a> {
    pub wires: Vec<&'a Wire>,
    pub buses: Vec<&'a Bus>,
    pub net_labels: Vec<&'a NetLabel>,
    pub probes: Vec<&'a SchematicProbe>,
    pub junctions: Vec<&'a Junction>,
    pub bus_taps: Vec<&'a BusTap>,
}

impl<'a> Scene<'a> {
    pub fn of(document: &'a SchematicDocument) -> Self {
        Self {
            wires: document.wires.iter().collect(),
            buses: document.buses.iter().collect(),
            net_labels: document.net_labels.iter().collect(),
            probes: document.probes.iter().collect(),
            junctions: document.junctions.iter().collect(),
            bus_taps: document.bus_taps.iter().collect(),
        }
    }
}

/// The objects that follow an edit, each with what it follows, by id.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Attached {
    pub net_labels: HashMap<u64, Anchor>,
    pub probes: HashMap<u64, Anchor>,
    pub junctions: HashMap<u64, Anchor>,
    /// Bus taps whose wire or bus end sits on moving geometry.
    pub tap_connections: HashMap<u64, Anchor>,
    /// Bus taps whose own bus moves under their bus end.
    pub tap_buses: HashMap<u64, Anchor>,
}

impl Attached {
    pub fn is_empty(&self) -> bool {
        self.net_labels.is_empty()
            && self.probes.is_empty()
            && self.junctions.is_empty()
            && self.tap_connections.is_empty()
            && self.tap_buses.is_empty()
    }

    /// Move every attached object of `document` to where `to` takes the
    /// point it sits on along its anchor.
    pub fn carry(
        &self,
        document: &mut SchematicDocument,
        mut to: impl FnMut(Anchor, Point) -> Point,
    ) {
        for label in &mut document.net_labels {
            if let Some(&anchor) = self.net_labels.get(&label.id) {
                label.pos = to(anchor, label.pos);
            }
        }
        for probe in &mut document.probes {
            if let Some(&anchor) = self.probes.get(&probe.id) {
                probe.position = to(anchor, probe.position);
            }
        }
        for junction in &mut document.junctions {
            if let Some(&anchor) = self.junctions.get(&junction.id) {
                junction.pos = to(anchor, junction.pos);
            }
        }
        for tap in &mut document.bus_taps {
            if let Some(&anchor) = self.tap_connections.get(&tap.id) {
                tap.connection_point = to(anchor, tap.connection_point);
            }
            if let Some(&anchor) = self.tap_buses.get(&tap.id) {
                tap.bus_point = to(anchor, tap.bus_point);
            }
        }
    }
}

/// Which conductors an object can sit on.
#[derive(Debug, Clone, Copy)]
enum Seat {
    /// A label, a probe or a dot: any pin, wire or bus.
    Any,
    /// A tap's wire end.
    Wires,
    /// A tap's bus end on another bus.
    BusesBut(u64),
    /// A tap's end on its own bus.
    OwnBus(u64),
}

impl Seat {
    fn admits(self, conductor: Conductor) -> bool {
        match (self, conductor) {
            (Self::Any, _) | (Self::Wires, Conductor::Wire(_)) => true,
            (Self::BusesBut(own), Conductor::Bus(id)) => id != own,
            (Self::OwnBus(own), Conductor::Bus(id)) => id == own,
            _ => false,
        }
    }

    const fn admits_pins(self) -> bool {
        matches!(self, Self::Any | Self::Wires)
    }
}

/// Everything in `scene` that sits on what `moving` moves, with what it
/// follows; what `selected` names is left to the selection.
///
/// An object follows a moving point it sits on, or else a conductor or run
/// that moves whole under it. A pin or a conductor that stays where it is
/// under the object keeps it: a dot where a moved wire ends on one that
/// stays, or a label on a pin a Move that breaks connections pulls out of a
/// wire, stays with what did not move.
pub fn attached(
    scene: &Scene<'_>,
    moving: MovingGeometry<'_>,
    selected: impl Fn(Attachable) -> bool,
) -> Attached {
    let mut attached = Attached::default();
    if moving.points.is_empty() && moving.conductors.is_empty() && moving.runs.is_empty() {
        return attached;
    }
    let finder = AnchorFinder::new(scene, moving);
    for label in &scene.net_labels {
        if !selected(Attachable::NetLabel(label.id))
            && let Some(anchor) = finder.anchor(label.pos, Seat::Any)
        {
            attached.net_labels.insert(label.id, anchor);
        }
    }
    for probe in &scene.probes {
        if !selected(Attachable::Probe(probe.id))
            && let Some(anchor) = finder.anchor(probe.position, Seat::Any)
        {
            attached.probes.insert(probe.id, anchor);
        }
    }
    for junction in &scene.junctions {
        if !selected(Attachable::Junction(junction.pos))
            && let Some(anchor) = finder.anchor(junction.pos, Seat::Any)
        {
            attached.junctions.insert(junction.id, anchor);
        }
    }
    for tap in &scene.bus_taps {
        if selected(Attachable::BusTap(tap.id)) {
            continue;
        }
        let seat = match tap.target_kind() {
            BusTargetKind::Wire => Seat::Wires,
            BusTargetKind::Bus => Seat::BusesBut(tap.bus_id),
        };
        if let Some(anchor) = finder.anchor(tap.connection_point, seat) {
            attached.tap_connections.insert(tap.id, anchor);
        }
        if let Some(anchor) = finder.anchor(tap.bus_point, Seat::OwnBus(tap.bus_id)) {
            attached.tap_buses.insert(tap.id, anchor);
        }
    }
    attached
}

struct AnchorFinder<'s, 'm> {
    scene: &'s Scene<'s>,
    moving: MovingGeometry<'m>,
    /// The conductors that move whole, looked up once.
    whole: Vec<(Conductor, &'s [Point])>,
}

impl<'s, 'm> AnchorFinder<'s, 'm> {
    fn new(scene: &'s Scene<'s>, moving: MovingGeometry<'m>) -> Self {
        let whole = if moving.conductors.is_empty() {
            Vec::new()
        } else {
            conductors(scene)
                .filter(|(conductor, _)| moving.conductors.contains(conductor))
                .collect()
        };
        Self {
            scene,
            moving,
            whole,
        }
    }

    fn anchor(&self, point: Point, seat: Seat) -> Option<Anchor> {
        let anchor =
            if seat.admits_pins() && self.moving.points.contains(&point) {
                Anchor::Point(point)
            } else if let Some(&(conductor, _)) = self.whole.iter().find(|(conductor, points)| {
                seat.admits(*conductor) && polyline_contains(points, point)
            }) {
                Anchor::Conductor(conductor)
            } else {
                let &(conductor, _) = self.moving.runs.iter().find(|(conductor, run)| {
                    seat.admits(*conductor) && run.contains_point(point)
                })?;
                Anchor::Conductor(conductor)
            };
        (!self.held(point, seat)).then_some(anchor)
    }

    /// Whether a pin or a conductor that stays where it is sits under
    /// `point`.
    fn held(&self, point: Point, seat: Seat) -> bool {
        if matches!(seat, Seat::OwnBus(_)) {
            // A tap's bus end belongs to its bus alone.
            return false;
        }
        if seat.admits_pins() && self.moving.fixed_points.contains(&point) {
            return true;
        }
        let carried_point = self.moving.carries_vertices && self.moving.points.contains(&point);
        conductors(self.scene).any(|(conductor, points)| {
            seat.admits(conductor)
                && !self.moving.conductors.contains(&conductor)
                && polyline_contains(points, point)
                && !(carried_point
                    && matches!(conductor, Conductor::Wire(_))
                    && points.contains(&point))
                && !self
                    .moving
                    .runs
                    .iter()
                    .any(|(moving, run)| *moving == conductor && run.contains_point(point))
        })
    }
}

fn conductors<'s>(scene: &'s Scene<'s>) -> impl Iterator<Item = (Conductor, &'s [Point])> + 's {
    scene
        .wires
        .iter()
        .map(|wire| (Conductor::Wire(wire.id), wire.points.as_slice()))
        .chain(
            scene
                .buses
                .iter()
                .map(|bus| (Conductor::Bus(bus.id), bus.points.as_slice())),
        )
}

fn polyline_contains(points: &[Point], point: Point) -> bool {
    points.contains(&point)
        || points
            .windows(2)
            .any(|pair| WireSegment::new(pair[0], pair[1]).contains_point(point))
}

/// Keep on its wire each label and probe that sat on a wire an edit laid
/// again, where the new route no longer passes it.
///
/// A wire whose end moved along its own run, or that was squared afresh,
/// can leave a label it carried mid-run off the new route, and the net
/// would lose its name. Such a label moves to the nearest point of the new
/// route. One still on the route, on another conductor, or that `leave`
/// names stays where it is. `relaid` holds each laid wire's id with its
/// vertices before the edit.
pub fn settle_on_relaid_wires(
    document: &mut SchematicDocument,
    relaid: &[(u64, &[Point])],
    leave: impl Fn(Attachable) -> bool,
) {
    let mut labels = Vec::new();
    let mut probes = Vec::new();
    for &(wire_id, before) in relaid {
        let Some(route) = document
            .wires
            .iter()
            .find(|wire| wire.id == wire_id)
            .map(|wire| wire.points.as_slice())
        else {
            continue;
        };
        if route == before || route.is_empty() {
            continue;
        }
        let stranded = |point: Point| {
            polyline_contains(before, point)
                && !polyline_contains(route, point)
                && !document
                    .wires
                    .iter()
                    .filter(|wire| wire.id != wire_id)
                    .map(|wire| wire.points.as_slice())
                    .chain(document.buses.iter().map(|bus| bus.points.as_slice()))
                    .any(|points| polyline_contains(points, point))
        };
        for label in &document.net_labels {
            if !leave(Attachable::NetLabel(label.id)) && stranded(label.pos) {
                labels.push((label.id, nearest_on_route(route, label.pos)));
            }
        }
        for probe in &document.probes {
            if !leave(Attachable::Probe(probe.id)) && stranded(probe.position) {
                probes.push((probe.id, nearest_on_route(route, probe.position)));
            }
        }
    }
    for (id, pos) in labels {
        if let Some(label) = document.net_labels.iter_mut().find(|label| label.id == id) {
            label.pos = pos;
        }
    }
    for (id, position) in probes {
        if let Some(probe) = document.probes.iter_mut().find(|probe| probe.id == id) {
            probe.position = position;
        }
    }
}

/// The point of `route` nearest `point`, by the distance along the grid;
/// the first such point when two are as near. A slanted run offers only its
/// ends.
fn nearest_on_route(route: &[Point], point: Point) -> Point {
    let distance = |candidate: Point| {
        (i64::from(candidate.x) - i64::from(point.x)).abs()
            + (i64::from(candidate.y) - i64::from(point.y)).abs()
    };
    let mut best = route[0];
    for pair in route.windows(2) {
        let (start, end) = (pair[0], pair[1]);
        let nearest = if start.y == end.y {
            Point::new(
                point.x.clamp(start.x.min(end.x), start.x.max(end.x)),
                start.y,
            )
        } else if start.x == end.x {
            Point::new(
                start.x,
                point.y.clamp(start.y.min(end.y), start.y.max(end.y)),
            )
        } else if distance(end) < distance(start) {
            end
        } else {
            start
        };
        if distance(nearest) < distance(best) {
            best = nearest;
        }
    }
    best
}
