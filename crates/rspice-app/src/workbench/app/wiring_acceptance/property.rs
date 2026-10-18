//! Random edit sequences on a cell view whose two sheets are drawn alike,
//! checked after every edit against four invariants:
//!
//! - **Sheet isolation.** An edit on one sheet leaves the other sheet's
//!   objects of every kind -- parts, wires, dots, buses, taps, labels,
//!   probes, notes and shapes -- and its nets exactly as they were, although
//!   both sheets draw on the very same coordinates.
//! - **Dots are junctions.** The dots painted are the points the junction
//!   rule marks on the design a run reads, and each is where the netlister
//!   joins three conductor ends or the crossing the dot itself joins.
//! - **Undo is exact.** Undo puts back the objects, the active sheet and the
//!   connectivity the edit found, to the bit; redo puts back what it left.
//! - **No wire doubles back.** No wire reverses onto itself at a vertex or
//!   holds a zero-length segment.
//!
//! The default run checks [`DEFAULT_SEQUENCES`] seeded sequences. The full
//! ten thousand are ignored by default and run with
//! `cargo test -p rspice-app --lib -- --ignored ten_thousand_random_edit_sequences`.
//! A failure is shrunk to the fewest steps that still fail the same way and
//! reported with its seed and those steps, written as the Rust a regression
//! test replays from the annotated drawing. Each different way of failing is
//! reported once.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::panic::{AssertUnwindSafe, catch_unwind};

use super::Bench;
use crate::schematic::view::selection_transform::SelectionTransform;
use crate::state::{Component, ComponentType, Junction, MoveSelectionMode, Point, Rotation, Wire};
use crate::workbench::app_state::AppState;
use rspice_design::schematic::bus::{Bus, BusTap};
use rspice_design::schematic::design_note::DesignNote;
use rspice_design::schematic::document::SchematicDocument;
use rspice_design::schematic::documentation_shape::DocumentationShape;
use rspice_design::schematic::junction_edit::JunctionRule;
use rspice_design::schematic::net_label::NetLabel;
use rspice_design::schematic::probe::SchematicProbe;

/// Sequences the default test run checks, about 24 s alone in the dev
/// profile. The full ten thousand take about nine minutes.
const DEFAULT_SEQUENCES: u64 = 400;
/// Edits in one sequence.
const STEPS: usize = 12;
/// Every coordinate an edit names is a multiple of this.
const PITCH: i32 = 20;

pub(super) fn p(x: i32, y: i32) -> Point {
    Point::new(x, y)
}

/// One object an edit selects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Pick {
    Part(u64),
    Wire(u64),
    Bus(u64),
    Tap(u64),
    Label(u64),
}

impl Pick {
    const fn id(self) -> u64 {
        match self {
            Self::Part(id) | Self::Wire(id) | Self::Bus(id) | Self::Tap(id) | Self::Label(id) => id,
        }
    }
}

/// One way to turn or flip a selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Turn {
    Clockwise,
    CounterClockwise,
    MirrorHorizontal,
    MirrorVertical,
}

impl Turn {
    fn transform(self) -> SelectionTransform {
        match self {
            Self::Clockwise => SelectionTransform::RotateClockwise,
            Self::CounterClockwise => SelectionTransform::RotateCounterClockwise,
            Self::MirrorHorizontal => SelectionTransform::MirrorHorizontal,
            Self::MirrorVertical => SelectionTransform::MirrorVertical,
        }
    }
}

/// One edit, named the way a user would make it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Step {
    /// Wire-tool clicks; two points off one line route an L.
    Draw(Vec<Point>),
    /// Select one object and delete it.
    Delete(Pick),
    /// Move the selection by a delta, breaking its connections or not.
    Move(Vec<Pick>, Point, bool),
    /// Drag a part through offsets from where it stands.
    Drag(u64, Vec<Point>),
    /// Stretch one wire segment by a delta.
    Stretch(u64, usize, Point),
    /// Turn or flip the selection.
    Transform(Vec<Pick>, Turn),
    /// Drop a resistor whose `+` pin lands on the point, turned so.
    Place(Point, Rotation),
    /// A junction-tool click.
    Junction(Point),
    /// Copy the selection.
    Copy(Vec<Pick>),
    /// Paste the clipboard with its middle at the point.
    Paste(Point),
    /// Open the other sheet.
    Sheet,
    Undo,
    Redo,
    /// A net-label click at the point; the number tells the label's name
    /// apart from the others the sheet holds.
    Label(Point, u32),
    /// Bus-tool clicks, the bus declared four wide under a name its number
    /// makes.
    Bus(Vec<Point>, u32),
    /// A tap from the bus at the first point out to the second, for the
    /// bus member numbered.
    Tap(u64, Point, Point, u32),
}

impl Step {
    /// The step as Rust, for a regression test to replay.
    fn rust(&self) -> String {
        let point = |point: &Point| format!("p({}, {})", point.x, point.y);
        let points = |points: &[Point]| {
            let listed = points.iter().map(point).collect::<Vec<_>>();
            format!("vec![{}]", listed.join(", "))
        };
        let picks = |picks: &[Pick]| {
            let listed = picks
                .iter()
                .map(|pick| format!("Pick::{pick:?}"))
                .collect::<Vec<_>>();
            format!("vec![{}]", listed.join(", "))
        };
        match self {
            Self::Draw(clicks) => format!("Draw({})", points(clicks)),
            Self::Delete(pick) => format!("Delete(Pick::{pick:?})"),
            Self::Move(selection, delta, broken) => {
                format!("Move({}, {}, {broken})", picks(selection), point(delta))
            }
            Self::Drag(part, offsets) => format!("Drag({part}, {})", points(offsets)),
            Self::Stretch(wire, segment, delta) => {
                format!("Stretch({wire}, {segment}, {})", point(delta))
            }
            Self::Transform(selection, turn) => {
                format!("Transform({}, Turn::{turn:?})", picks(selection))
            }
            Self::Place(pin, rotation) => format!("Place({}, Rotation::{rotation:?})", point(pin)),
            Self::Junction(at) => format!("Junction({})", point(at)),
            Self::Copy(selection) => format!("Copy({})", picks(selection)),
            Self::Paste(at) => format!("Paste({})", point(at)),
            Self::Sheet => "Sheet".to_owned(),
            Self::Undo => "Undo".to_owned(),
            Self::Redo => "Redo".to_owned(),
            Self::Label(at, name) => format!("Label({}, {name})", point(at)),
            Self::Bus(clicks, name) => format!("Bus({}, {name})", points(clicks)),
            Self::Tap(bus, from, to, member) => {
                format!("Tap({bus}, {}, {}, {member})", point(from), point(to))
            }
        }
    }

    /// Whether the step edits the active sheet, as opposed to navigating,
    /// copying or stepping through history.
    fn edits_a_sheet(&self) -> bool {
        !matches!(self, Self::Undo | Self::Redo | Self::Sheet | Self::Copy(_))
    }
}

/// A small xorshift generator: the sequences are a function of the seed.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(
            seed.wrapping_mul(0x9E37_79B9_7F4A_7C15)
                .wrapping_add(0xD1B5_4A32_D192_ED03)
                | 1,
        )
    }

    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, bound: usize) -> usize {
        usize::try_from(self.next() % u64::try_from(bound.max(1)).unwrap_or(1)).unwrap_or(0)
    }

    fn chance(&mut self, percent: usize) -> bool {
        self.below(100) < percent
    }

    fn pick<T: Clone>(&mut self, items: &[T]) -> Option<T> {
        (!items.is_empty()).then(|| items[self.below(items.len())].clone())
    }

    /// A multiple of the pitch in `-reach..=reach` pitches, never zero.
    fn step(&mut self, reach: usize) -> i32 {
        let magnitude = i32::try_from(self.below(reach) + 1).unwrap_or(1) * PITCH;
        if self.chance(50) {
            magnitude
        } else {
            -magnitude
        }
    }

    /// A pitch point in the window both fixture sheets draw in.
    fn point(&mut self) -> Point {
        let x = i32::try_from(self.below(21)).unwrap_or(0) * PITCH;
        let y = i32::try_from(self.below(16)).unwrap_or(0) * PITCH;
        p(x, y)
    }
}

/// The active sheet's objects, as a generator reads them.
struct ActiveSheet {
    parts: Vec<Component>,
    wires: Vec<Wire>,
    junctions: Vec<Point>,
    buses: Vec<Bus>,
    taps: Vec<u64>,
    labels: Vec<u64>,
}

impl ActiveSheet {
    fn read(bench: &Bench) -> Self {
        let active = bench.active_sheet();
        let document = bench.state().schematic.document();
        let on_sheet = |id: u64| bench.sheet_of(id) == active;
        Self {
            parts: document
                .components
                .iter()
                .filter(|part| on_sheet(part.id))
                .cloned()
                .collect(),
            wires: document
                .wires
                .iter()
                .filter(|wire| on_sheet(wire.id))
                .cloned()
                .collect(),
            junctions: document
                .junctions
                .iter()
                .filter(|junction| on_sheet(junction.id))
                .map(|junction| junction.pos)
                .collect(),
            buses: document
                .buses
                .iter()
                .filter(|bus| on_sheet(bus.id))
                .cloned()
                .collect(),
            taps: document
                .bus_taps
                .iter()
                .map(|tap| tap.id)
                .filter(|id| on_sheet(*id))
                .collect(),
            labels: document
                .net_labels
                .iter()
                .map(|label| label.id)
                .filter(|id| on_sheet(*id))
                .collect(),
        }
    }

    /// Up to three objects, mostly parts and wires.
    fn picks(&self, rng: &mut Rng) -> Vec<Pick> {
        let mut picks = Vec::new();
        for _ in 0..=rng.below(3) {
            let pick = match rng.below(100) {
                0..=54 => None,
                55..=89 => rng.pick(&self.wires).map(|wire| Pick::Wire(wire.id)),
                90..=93 => rng.pick(&self.buses).map(|bus| Pick::Bus(bus.id)),
                94..=96 => rng.pick(&self.taps).map(Pick::Tap),
                _ => rng.pick(&self.labels).map(Pick::Label),
            }
            .or_else(|| rng.pick(&self.parts).map(|part| Pick::Part(part.id)));
            if let Some(pick) = pick.filter(|pick| !picks.contains(pick)) {
                picks.push(pick);
            }
        }
        picks
    }

    /// A pitch point on one of the active buses.
    fn on_bus(&self, rng: &mut Rng) -> Option<(u64, Point)> {
        let bus = rng.pick(&self.buses)?;
        let segment = rng.below(bus.points.len().saturating_sub(1));
        let (a, b) = (*bus.points.get(segment)?, *bus.points.get(segment + 1)?);
        let span = ((b.x - a.x).abs() + (b.y - a.y).abs()) / PITCH;
        let along =
            i32::try_from(rng.below(usize::try_from(span + 1).unwrap_or(1))).unwrap_or(0) * PITCH;
        Some((
            bus.id,
            p(
                a.x + (b.x - a.x).signum() * along,
                a.y + (b.y - a.y).signum() * along,
            ),
        ))
    }

    fn pins(&self) -> Vec<Point> {
        self.parts
            .iter()
            .flat_map(|part| part.terminal_positions())
            .map(|(_, point)| point)
            .collect()
    }

    fn vertices(&self) -> Vec<Point> {
        self.wires
            .iter()
            .flat_map(|wire| wire.points.iter().copied())
            .collect()
    }

    /// A pitch point strictly inside one of the active wires' segments.
    fn interior(&self, rng: &mut Rng) -> Option<Point> {
        let segments = self
            .wires
            .iter()
            .flat_map(|wire| wire.points.windows(2))
            .filter(|segment| {
                let (a, b) = (segment[0], segment[1]);
                (a.x == b.x && (a.y - b.y).abs() >= 2 * PITCH)
                    || (a.y == b.y && (a.x - b.x).abs() >= 2 * PITCH)
            })
            .collect::<Vec<_>>();
        let segment = rng.pick(&segments)?;
        let (a, b) = (segment[0], segment[1]);
        let span = ((b.x - a.x).abs() + (b.y - a.y).abs()) / PITCH;
        let along = i32::try_from(rng.below(usize::try_from(span - 1).unwrap_or(1)) + 1)
            .unwrap_or(1)
            * PITCH;
        Some(p(
            a.x + (b.x - a.x).signum() * along,
            a.y + (b.y - a.y).signum() * along,
        ))
    }

    /// Where an orthogonal segment of one active wire crosses another's.
    fn crossings(&self) -> Vec<Point> {
        let segments = self
            .wires
            .iter()
            .flat_map(|wire| wire.points.windows(2).map(|pair| (pair[0], pair[1])))
            .collect::<Vec<_>>();
        let mut found = Vec::new();
        for &(a, b) in &segments {
            for &(c, d) in &segments {
                if a.y == b.y && c.x == d.x {
                    let x = c.x;
                    let y = a.y;
                    if a.x.min(b.x) < x && x < a.x.max(b.x) && c.y.min(d.y) < y && y < c.y.max(d.y)
                    {
                        found.push(p(x, y));
                    }
                }
            }
        }
        found
    }

    /// A point a click would aim for: a pin, a vertex, a wire's inside or
    /// open canvas.
    fn target(&self, rng: &mut Rng) -> Point {
        match rng.below(4) {
            0 => rng.pick(&self.pins()),
            1 => rng.pick(&self.vertices()),
            2 => self.interior(rng),
            _ => None,
        }
        .unwrap_or_else(|| rng.point())
    }
}

/// The next edit for the bench as it stands.
fn generate(bench: &Bench, rng: &mut Rng) -> Step {
    let sheet = ActiveSheet::read(bench);
    let roll = rng.below(112);
    let step = match roll {
        0..=19 => {
            let start = sheet.target(rng);
            let end = if rng.chance(60) {
                let reach = rng.step(6);
                if rng.chance(50) {
                    p(start.x + reach, start.y)
                } else {
                    p(start.x, start.y + reach)
                }
            } else {
                sheet.target(rng)
            };
            (start != end).then(|| Step::Draw(vec![start, end]))
        }
        20..=25 => sheet.picks(rng).first().copied().map(Step::Delete),
        26..=33 => {
            let picks = sheet.picks(rng);
            let delta = p(rng.step(3), if rng.chance(50) { 0 } else { rng.step(3) });
            (!picks.is_empty()).then(|| Step::Move(picks, delta, rng.chance(25)))
        }
        34..=41 => rng.pick(&sheet.parts).map(|part| {
            let offsets = (0..=rng.below(3))
                .map(|_| p(rng.step(3), rng.step(3)))
                .scan(Point::origin(), |at, step| {
                    *at = p(at.x + step.x, at.y + step.y);
                    Some(*at)
                })
                .collect();
            Step::Drag(part.id, offsets)
        }),
        42..=47 => rng.pick(&sheet.wires).and_then(|wire| {
            let segment = rng.below(wire.points.len().saturating_sub(1));
            let (a, b) = (*wire.points.get(segment)?, *wire.points.get(segment + 1)?);
            let delta = if a.y == b.y {
                p(0, rng.step(3))
            } else if a.x == b.x {
                p(rng.step(3), 0)
            } else {
                return None;
            };
            Some(Step::Stretch(wire.id, segment, delta))
        }),
        48..=53 => {
            let picks = sheet.picks(rng);
            let turn = [
                Turn::Clockwise,
                Turn::CounterClockwise,
                Turn::MirrorHorizontal,
                Turn::MirrorVertical,
            ][rng.below(4)];
            (!picks.is_empty()).then_some(Step::Transform(picks, turn))
        }
        54..=61 => {
            let at = sheet.interior(rng).unwrap_or_else(|| rng.point());
            let rotation =
                [Rotation::R0, Rotation::R90, Rotation::R180, Rotation::R270][rng.below(4)];
            Some(Step::Place(at, rotation))
        }
        62..=67 => {
            let mut candidates = sheet.crossings();
            candidates.extend(sheet.junctions.iter().copied());
            candidates.extend(sheet.vertices());
            rng.pick(&candidates).map(Step::Junction)
        }
        68..=71 => {
            let picks = sheet.picks(rng);
            (!picks.is_empty()).then_some(Step::Copy(picks))
        }
        72..=76 => Some(Step::Paste(rng.point())),
        77..=81 => Some(Step::Sheet),
        82..=92 => Some(Step::Undo),
        93..=99 => Some(Step::Redo),
        100..=103 => {
            let name = u32::try_from(rng.below(4)).unwrap_or(0);
            Some(Step::Label(sheet.target(rng), name))
        }
        104..=107 => {
            let start = sheet.target(rng);
            let reach = rng.step(6);
            let end = if rng.chance(50) {
                p(start.x + reach, start.y)
            } else {
                p(start.x, start.y + reach)
            };
            Some(Step::Bus(
                vec![start, end],
                u32::try_from(rng.below(2)).unwrap_or(0),
            ))
        }
        // The tap tool lands a tap's end on a wire.
        _ => sheet.on_bus(rng).and_then(|(bus, from)| {
            let to = if rng.chance(50) {
                rng.pick(&sheet.vertices())
            } else {
                sheet.interior(rng)
            }?;
            let member = u32::try_from(rng.below(4)).unwrap_or(0);
            (to != from).then_some(Step::Tap(bus, from, to, member))
        }),
    };
    step.unwrap_or(Step::Undo)
}

/// Whether the object `id` is on the active sheet now.
fn on_active_sheet(bench: &Bench, id: u64) -> bool {
    let document = bench.state().schematic.document();
    let exists = document.components.iter().any(|part| part.id == id)
        || document.wires.iter().any(|wire| wire.id == id)
        || document.buses.iter().any(|bus| bus.id == id)
        || document.bus_taps.iter().any(|tap| tap.id == id)
        || document.net_labels.iter().any(|label| label.id == id);
    exists && bench.sheet_of(id) == bench.active_sheet()
}

/// Select `picks` that are on the active sheet; whether any was.
fn select(bench: &mut Bench, picks: &[Pick]) -> bool {
    let live = picks
        .iter()
        .copied()
        .filter(|pick| on_active_sheet(bench, pick.id()))
        .collect::<Vec<_>>();
    bench.select(|selection| {
        for pick in &live {
            match pick {
                Pick::Part(id) => selection.select_component(*id),
                Pick::Wire(id) => selection.select_wire(*id),
                Pick::Bus(id) => selection.select_bus(*id),
                Pick::Tap(id) => selection.select_bus_tap(*id),
                Pick::Label(id) => selection.select_net_label(*id),
            }
        }
    });
    !live.is_empty()
}

/// The name a `Label` or `Bus` step numbered `name` gives on `sheet`: no
/// step names a net on both sheets, though a copy pasted across carries
/// its names with it.
fn sheet_name(sheet: usize, kind: &str, name: u32) -> String {
    format!("S{}{kind}{name}", sheet + 1)
}

/// Where a resistor turned to `rotation` stands for its `+` pin to be at
/// `pin`.
fn resistor_for_pin(pin: Point, rotation: Rotation) -> Point {
    let mut probe = Component::new(0, ComponentType::Resistor, Point::origin());
    probe.rotation = rotation;
    let offset = probe.terminal_positions()[0].1;
    p(pin.x - offset.x, pin.y - offset.y)
}

/// Make the edit. A step that names an object no longer on the active sheet
/// does nothing, which is what lets a shrunk sequence drop the step that
/// made it.
fn apply(bench: &mut Bench, step: &Step) {
    match step {
        Step::Draw(clicks) => bench.draw_wire(clicks),
        Step::Delete(pick) => {
            if select(bench, &[*pick]) {
                bench.delete_selection();
            }
        }
        Step::Move(picks, delta, broken) => {
            if select(bench, picks) {
                let mode = if *broken {
                    MoveSelectionMode::BreakConnections
                } else {
                    MoveSelectionMode::Connected
                };
                bench.move_selection(*delta, mode);
            }
        }
        Step::Drag(part, offsets) => {
            if select(bench, &[Pick::Part(*part)]) {
                bench.drag_part(*part, offsets);
            }
        }
        Step::Stretch(wire, segment, delta) => {
            let present = on_active_sheet(bench, *wire)
                && bench
                    .state()
                    .schematic
                    .document()
                    .wires
                    .iter()
                    .any(|candidate| candidate.id == *wire && segment + 1 < candidate.points.len());
            if present {
                bench.select(|selection| selection.select_only_wire_segment(*wire, *segment));
                bench.stretch_selection(*delta);
            }
        }
        Step::Transform(picks, turn) => {
            if select(bench, picks) {
                bench.turn_selection(turn.transform());
            }
        }
        Step::Place(pin, rotation) => {
            bench.place(
                ComponentType::Resistor,
                resistor_for_pin(*pin, *rotation),
                *rotation,
            );
        }
        Step::Junction(at) => bench.click_junction(*at),
        Step::Copy(picks) => {
            if select(bench, picks) {
                bench.copy_selection();
            }
        }
        Step::Paste(at) => bench.paste_at(*at),
        Step::Sheet => bench.activate_sheet(1 - bench.active_sheet()),
        Step::Undo => bench.undo(),
        Step::Redo => bench.redo(),
        Step::Label(at, name) => {
            let name = sheet_name(bench.active_sheet(), "N", *name);
            bench.place_label(*at, &name);
        }
        Step::Bus(clicks, name) => {
            let name = sheet_name(bench.active_sheet(), "B", *name);
            bench.draw_bus(clicks, &format!("{name}[3:0]"));
        }
        Step::Tap(bus, from, to, member) => {
            if !on_active_sheet(bench, *bus) {
                return;
            }
            let base = bench
                .state()
                .schematic
                .document()
                .buses
                .iter()
                .find(|candidate| candidate.id == *bus)
                .and_then(|candidate| candidate.declaration.as_ref())
                .map(|declaration| declaration.name.clone());
            if let Some(base) = base {
                bench.place_tap(*bus, *from, *to, &format!("{base}[{member}]"));
            }
        }
    }
}

/// One member of a net: a wire, or a pin of a part.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Member {
    Wire(u64),
    Pin(u64, String),
}

/// What one sheet holds: its objects of every kind by id, and its nets.
#[derive(Debug, Clone, Default, PartialEq)]
struct Sheet {
    parts: Vec<Component>,
    wires: Vec<Wire>,
    junctions: Vec<Junction>,
    buses: Vec<Bus>,
    taps: Vec<BusTap>,
    labels: Vec<NetLabel>,
    probes: Vec<SchematicProbe>,
    notes: Vec<DesignNote>,
    shapes: Vec<DocumentationShape>,
    nets: BTreeSet<BTreeSet<Member>>,
}

impl Sheet {
    /// The objects of `document` that `on_sheet` admits, and `nets`.
    fn read(
        document: &SchematicDocument,
        on_sheet: impl Fn(u64) -> bool,
        nets: BTreeSet<BTreeSet<Member>>,
    ) -> Self {
        fn own<T: Clone>(
            objects: &[T],
            id: fn(&T) -> u64,
            on_sheet: &dyn Fn(u64) -> bool,
        ) -> Vec<T> {
            let mut own = objects
                .iter()
                .filter(|object| on_sheet(id(object)))
                .cloned()
                .collect::<Vec<_>>();
            own.sort_by_key(id);
            own
        }
        Self {
            parts: own(&document.components, |object| object.id, &on_sheet),
            wires: own(&document.wires, |object| object.id, &on_sheet),
            junctions: own(&document.junctions, |object| object.id, &on_sheet),
            buses: own(&document.buses, |object| object.id, &on_sheet),
            taps: own(&document.bus_taps, |object| object.id, &on_sheet),
            labels: own(&document.net_labels, |object| object.id, &on_sheet),
            probes: own(&document.probes, |object| object.id, &on_sheet),
            notes: own(&document.design_notes, |object| object.id, &on_sheet),
            shapes: own(
                &document.documentation_shapes,
                |object| object.id,
                &on_sheet,
            ),
            nets,
        }
    }
}

/// `projected` with only the objects `on_sheet` admits.
fn sheet_alone(projected: &SchematicDocument, on_sheet: impl Fn(u64) -> bool) -> SchematicDocument {
    let mut alone = projected.clone();
    alone.components.retain(|object| on_sheet(object.id));
    alone.wires.retain(|object| on_sheet(object.id));
    alone.junctions.retain(|object| on_sheet(object.id));
    alone.buses.retain(|object| on_sheet(object.id));
    alone.bus_taps.retain(|object| on_sheet(object.id));
    alone.net_labels.retain(|object| on_sheet(object.id));
    alone
        .connections
        .retain(|record| on_sheet(record.wire_id) && on_sheet(record.component_id));
    alone
}

/// Each sheet's nets, as its own drawing makes them.
///
/// A net name joins every sheet that carries it, as the deck reads them:
/// copying a named bus from one sheet to the other joins its nets there. So
/// a sheet's nets are read from its own drawing alone, and an edit on the
/// other sheet can change them only by changing this sheet's objects. While
/// no net reaches both sheets, the deck's own nets are each sheet's already.
fn sheet_nets(bench: &Bench, projected: &SchematicDocument) -> [BTreeSet<BTreeSet<Member>>; 2] {
    let sheet_of = |member: &Member| match member {
        Member::Wire(id) | Member::Pin(id, _) => bench.sheet_of(*id),
    };
    let nets = nets_by_member(projected);
    let shared = nets.iter().any(|net| {
        let mut sheets = net.iter().map(sheet_of);
        let first = sheets.next();
        sheets.any(|sheet| Some(sheet) != first)
    });
    if shared {
        return [0, 1].map(|sheet| {
            nets_by_member(&sheet_alone(projected, |id| bench.sheet_of(id) == sheet))
        });
    }
    let mut by_sheet = [BTreeSet::new(), BTreeSet::new()];
    for net in nets {
        if let Some(member) = net.first() {
            by_sheet[sheet_of(member)].insert(net);
        }
    }
    by_sheet
}

/// The nets `document` makes, each as the wires and pins it joins.
fn nets_by_member(document: &SchematicDocument) -> BTreeSet<BTreeSet<Member>> {
    let extracted = rspice_design::connectivity::extract(document, |_| None, |_| None);
    let mut nets = std::collections::BTreeMap::<String, BTreeSet<Member>>::new();
    for wire in &document.wires {
        let name = extracted
            .net_of_wire(wire.id)
            .map_or_else(|| format!("wire {}", wire.id), |net| net.spice_name());
        nets.entry(name).or_default().insert(Member::Wire(wire.id));
    }
    for terminal in &extracted.terminals {
        nets.entry(terminal.net_name.clone())
            .or_default()
            .insert(Member::Pin(terminal.component_id, terminal.pin.clone()));
    }
    nets.into_values().collect()
}

/// Everything an undo has to put back.
#[derive(Debug, Clone, PartialEq)]
struct World {
    active: usize,
    sheets: [Sheet; 2],
}

impl World {
    fn read(bench: &Bench, projected: &SchematicDocument) -> Self {
        let document = bench.state().schematic.document();
        let [first, second] = sheet_nets(bench, projected);
        let sheet =
            |index: usize, nets| Sheet::read(document, |id| bench.sheet_of(id) == index, nets);
        Self {
            active: bench.active_sheet(),
            sheets: [sheet(0, first), sheet(1, second)],
        }
    }
}

/// The invariants that hold of any one drawing, whatever led to it.
fn check_drawing(bench: &Bench, projected: &SchematicDocument) -> Result<(), String> {
    for wire in &bench.state().schematic.document().wires {
        if wire.points.len() < 2 {
            return Err(format!(
                "doubled-back: wire {} has {:?}",
                wire.id, wire.points
            ));
        }
        for pair in wire.points.windows(2) {
            if pair[0] == pair[1] {
                return Err(format!(
                    "doubled-back: wire {} has a zero-length segment at {:?}: {:?}",
                    wire.id, pair[0], wire.points
                ));
            }
        }
        for corner in wire.points.windows(3) {
            let (a, b, c) = (corner[0], corner[1], corner[2]);
            let (ux, uy) = (i64::from(b.x - a.x), i64::from(b.y - a.y));
            let (vx, vy) = (i64::from(c.x - b.x), i64::from(c.y - b.y));
            if ux * vy == uy * vx && ux * vx + uy * vy < 0 {
                return Err(format!(
                    "doubled-back: wire {} reverses at {b:?}: {:?}",
                    wire.id, wire.points
                ));
            }
        }
    }
    check_dots(projected)
}

/// The conductor ends at one point: wire ends with a vertex there, wires
/// running through it, and pins on it.
struct Ends {
    vertex_ends: usize,
    interiors: usize,
    on_pins: usize,
    /// Wire interiors cross there: runs through it in two directions or
    /// more, one wire's or several, with no vertex on it.
    crossing: bool,
}

impl Ends {
    fn count(&self) -> usize {
        self.vertex_ends + 2 * self.interiors + self.on_pins
    }
}

/// The conductor ends at `point`. A crossing is judged as the junction rule
/// judges one, by the directions the runs through it take, so a wire that
/// crosses itself crosses.
fn ends_at(document: &SchematicDocument, pins: &[Point], point: Point) -> Ends {
    let mut vertex_ends = 0;
    let mut interiors = 0;
    let mut directions = BTreeSet::new();
    for wire in &document.wires {
        let last = wire.points.len().saturating_sub(1);
        let at_vertex = wire
            .points
            .iter()
            .enumerate()
            .filter(|(_, vertex)| **vertex == point)
            .map(|(index, _)| if index == 0 || index == last { 1 } else { 2 })
            .sum::<usize>();
        if at_vertex > 0 {
            vertex_ends += at_vertex;
            continue;
        }
        let mut through = false;
        for segment in wire
            .segments()
            .filter(|segment| segment.contains_point(point))
        {
            through = true;
            let (dx, dy) = (
                i64::from(segment.end.x) - i64::from(segment.start.x),
                i64::from(segment.end.y) - i64::from(segment.start.y),
            );
            let mut divisor = (dx.abs(), dy.abs());
            while divisor.1 != 0 {
                divisor = (divisor.1, divisor.0 % divisor.1);
            }
            let sign = if dx < 0 || (dx == 0 && dy < 0) { -1 } else { 1 };
            directions.insert((sign * dx / divisor.0, sign * dy / divisor.0));
        }
        interiors += usize::from(through);
    }
    Ends {
        vertex_ends,
        interiors,
        on_pins: pins.iter().filter(|pin| **pin == point).count(),
        crossing: vertex_ends == 0 && directions.len() >= 2,
    }
}

/// The painted dots against the junction rule and the netlister, on the
/// design a run reads.
fn check_dots(projected: &SchematicDocument) -> Result<(), String> {
    let key = |point: Point| (point.x, point.y);
    let painted = projected
        .junctions
        .iter()
        .map(|junction| key(junction.pos))
        .collect::<BTreeSet<_>>();
    if painted.len() != projected.junctions.len() {
        return Err(format!(
            "dots: two dots on one point among {:?}",
            projected.junctions
        ));
    }
    let rule = JunctionRule::for_document(projected)
        .points(projected)
        .into_iter()
        .map(key)
        .collect::<BTreeSet<_>>();
    if rule != painted {
        return Err(format!(
            "dots: painted {:?} but the junction rule marks {:?}",
            painted.difference(&rule).collect::<Vec<_>>(),
            rule.difference(&painted).collect::<Vec<_>>()
        ));
    }
    let pins = rspice_design::schematic::junction_edit::placed_pins(projected);
    let extracted = rspice_design::connectivity::extract(projected, |_| None, |_| None);
    for &(x, y) in &painted {
        let dot = p(x, y);
        if ends_at(projected, &pins, dot).count() < 3 {
            return Err(format!(
                "dots: the dot at {dot:?} marks fewer than three conductor ends"
            ));
        }
        let Some(net) = extracted.net_at(dot) else {
            return Err(format!("dots: the dot at {dot:?} is on no net"));
        };
        for wire in projected
            .wires
            .iter()
            .filter(|wire| wire.contains_point(dot))
        {
            if extracted.net_of_wire(wire.id).map(|net| net.id) != Some(net.id) {
                return Err(format!(
                    "dots: the dot at {dot:?} does not join wire {} in the netlist",
                    wire.id
                ));
            }
        }
    }
    let candidates = projected
        .wires
        .iter()
        .flat_map(|wire| wire.points.iter().copied())
        .chain(pins.iter().copied())
        .map(key)
        .collect::<BTreeSet<_>>();
    for (x, y) in candidates {
        let point = p(x, y);
        let ends = ends_at(projected, &pins, point);
        if !ends.crossing && ends.count() >= 3 && !painted.contains(&(x, y)) {
            return Err(format!(
                "dots: the netlister joins {} conductor ends at {point:?} and no dot is painted",
                ends.count()
            ));
        }
    }
    Ok(())
}

/// Why a sequence failed: the step and what broke.
#[derive(Debug, Clone)]
struct Failure {
    step: usize,
    message: String,
}

impl Failure {
    fn kind(&self) -> &str {
        self.message.split(':').next().unwrap_or_default()
    }
}

/// The undo model: what each undo and redo must put back.
#[derive(Default)]
struct History {
    undo: Vec<World>,
    redo: Vec<World>,
}

/// Make `step` and check every invariant against the world it found.
fn step_and_check(
    bench: &mut Bench,
    history: &mut History,
    before: &World,
    step: &Step,
) -> Result<World, String> {
    let made = catch_unwind(AssertUnwindSafe(|| apply(bench, step)));
    if let Err(panic) = made {
        let message = panic
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| panic.downcast_ref::<&str>().map(|text| (*text).to_owned()))
            .unwrap_or_default();
        return Err(format!("panic: {message}"));
    }
    let projected = bench.projected();
    let after = World::read(bench, &projected);
    check_drawing(bench, &projected)?;
    if step.edits_a_sheet() {
        let other = 1 - before.active;
        if after.sheets[other] != before.sheets[other] {
            return Err(format!(
                "isolation: an edit on sheet {} changed sheet {other}\nbefore {:?}\nafter  {:?}",
                before.active, before.sheets[other], after.sheets[other]
            ));
        }
    }
    match step {
        Step::Undo => match history.undo.pop() {
            Some(expected) if after != expected => {
                return Err(format!(
                    "undo: the world after undo differs from before the edit\nexpected {expected:?}\nfound    {after:?}"
                ));
            }
            Some(_) => history.redo.push(before.clone()),
            None if after != *before => {
                return Err(format!(
                    "undo: with nothing to undo, undo changed the world\nbefore {before:?}\nafter  {after:?}"
                ));
            }
            None => {}
        },
        Step::Redo => match history.redo.pop() {
            Some(expected) if after != expected => {
                return Err(format!(
                    "undo: the world after redo differs from after the edit\nexpected {expected:?}\nfound    {after:?}"
                ));
            }
            Some(_) => history.undo.push(before.clone()),
            None if after != *before => {
                return Err(format!(
                    "undo: with nothing to redo, redo changed the world\nbefore {before:?}\nafter  {after:?}"
                ));
            }
            None => {}
        },
        _ if after != *before => {
            history.undo.push(before.clone());
            history.redo.clear();
        }
        _ => {}
    }
    Ok(after)
}

/// Which drawing a sequence starts from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Drawing {
    /// Parts and wires alone, the drawing the first regressions were found
    /// on: their object numbers count from it.
    Wiring,
    /// The wiring, and on it a bus tapped onto two wires, two net labels
    /// and a probe flag, which the property test starts from.
    Annotated,
}

/// The fixture: each sheet draws the same objects at the same coordinates —
/// a T, an unmarked crossing and an L, and annotated, a bus above them
/// tapped onto the T's top wire and onto the L's corner, labels on the
/// crossing wire and at the T's foot, and a probe at the crossing wire's
/// end — and neither history holds a step. Each sheet names its own nets.
fn fixture(drawing: Drawing) -> (Bench, AppState) {
    let mut bench = Bench::new(&[]);
    for sheet in 0..2 {
        bench.activate_sheet(sheet);
        bench.place(ComponentType::Resistor, p(100, 100), Rotation::R0);
        bench.place(ComponentType::Resistor, p(300, 100), Rotation::R0);
        bench.place(ComponentType::Resistor, p(200, 240), Rotation::R90);
        bench.place(ComponentType::Resistor, p(100, 180), Rotation::R0);
        bench.draw_wire(&[p(120, 100), p(280, 100)]);
        bench.draw_wire(&[p(200, 100), p(200, 220)]);
        bench.draw_wire(&[p(120, 180), p(280, 180)]);
        bench.draw_wire(&[p(320, 100), p(360, 160)]);
    }
    if drawing == Drawing::Annotated {
        for sheet in 0..2 {
            bench.activate_sheet(sheet);
            let name = sheet_name(sheet, "D", 0);
            let bus = bench.draw_bus(&[p(120, 40), p(380, 40)], &format!("{name}[3:0]"));
            let taps = [
                bench.place_tap(bus, p(160, 40), p(160, 100), &format!("{name}[1]")),
                bench.place_tap(bus, p(360, 40), p(360, 100), &format!("{name}[2]")),
            ];
            let labels = [
                bench.place_label(p(240, 180), &sheet_name(sheet, "X", 0)),
                bench.place_label(p(200, 220), &sheet_name(sheet, "Y", 0)),
            ];
            let probe = bench.place_probe(p(280, 180));
            assert!(
                bus != 0 && !taps.contains(&0) && !labels.contains(&0) && probe != 0,
                "the annotated fixture is incomplete\n{}",
                bench.describe()
            );
        }
    }
    bench.activate_sheet(0);
    bench.forget_history();
    let state = bench.state().clone();
    (bench, state)
}

/// Replay `steps` from the wiring fixture, as a regression test pins a
/// sequence the property test reported; the first failure, if any.
pub(super) fn replay(steps: &[Step]) -> Result<(), String> {
    replay_on(Drawing::Wiring, steps)
}

/// Replay `steps` from `drawing`; the first failure, if any.
pub(super) fn replay_on(drawing: Drawing, steps: &[Step]) -> Result<(), String> {
    let (mut bench, fixture) = fixture(drawing);
    run(&mut bench, &fixture, steps).map_err(|failure| {
        format!(
            "step {} `{}`: {}",
            failure.step,
            steps[failure.step].rust(),
            failure.message
        )
    })
}

fn run(bench: &mut Bench, fixture: &AppState, steps: &[Step]) -> Result<(), Failure> {
    *bench.state_mut() = fixture.clone();
    let mut history = History::default();
    let mut world = World::read(bench, &bench.projected());
    for (index, step) in steps.iter().enumerate() {
        world = step_and_check(bench, &mut history, &world, step).map_err(|message| Failure {
            step: index,
            message,
        })?;
    }
    Ok(())
}

/// Generate and check one seeded sequence; on failure, the steps up to the
/// failing one and the failure.
fn explore(bench: &mut Bench, fixture: &AppState, seed: u64) -> Result<(), (Vec<Step>, Failure)> {
    *bench.state_mut() = fixture.clone();
    let mut rng = Rng::new(seed);
    let mut history = History::default();
    let mut world = World::read(bench, &bench.projected());
    let mut steps = Vec::new();
    for index in 0..STEPS {
        let step = generate(bench, &mut rng);
        steps.push(step.clone());
        match step_and_check(bench, &mut history, &world, &step) {
            Ok(next) => world = next,
            Err(message) => {
                return Err((
                    steps,
                    Failure {
                        step: index,
                        message,
                    },
                ));
            }
        }
    }
    Ok(())
}

/// The fewest of `steps` that still fail the way `failure` did.
fn shrink(
    bench: &mut Bench,
    fixture: &AppState,
    mut steps: Vec<Step>,
    mut failure: Failure,
) -> (Vec<Step>, Failure) {
    loop {
        let mut shrunk = false;
        for index in (0..steps.len()).rev() {
            let mut candidate = steps.clone();
            candidate.remove(index);
            if let Err(found) = run(bench, fixture, &candidate)
                && found.kind() == failure.kind()
            {
                candidate.truncate(found.step + 1);
                steps = candidate;
                failure = found;
                shrunk = true;
                break;
            }
        }
        if !shrunk {
            return (steps, failure);
        }
    }
}

/// Check every seed; report each different way of failing once, shrunk.
fn check_sequences(seeds: std::ops::Range<u64>) {
    let (mut bench, fixture) = fixture(Drawing::Annotated);
    let mut seen = BTreeSet::new();
    let mut reports = Vec::new();
    for seed in seeds {
        let Err((steps, failure)) = explore(&mut bench, &fixture, seed) else {
            continue;
        };
        // The invariant and the words of its finding before any coordinate.
        let signature = failure
            .message
            .split(|character: char| character.is_ascii_digit() || character == '\n')
            .next()
            .unwrap_or_default()
            .to_owned();
        if !seen.insert(signature) {
            continue;
        }
        let (steps, failure) = shrink(&mut bench, &fixture, steps, failure);
        let mut report = format!(
            "seed {seed}: {} at step {} of the shrunk sequence\nsteps:\n",
            failure.kind(),
            failure.step
        );
        for step in &steps {
            let _ = writeln!(report, "    {},", step.rust());
        }
        let _ = write!(report, "{}", failure.message);
        reports.push(report);
    }
    assert!(reports.is_empty(), "{}", reports.join("\n\n"));
}

#[test]
fn random_edit_sequences_keep_sheets_dots_undo_and_wires_sound() {
    check_sequences(0..DEFAULT_SEQUENCES);
}

#[test]
#[ignore = "ten thousand sequences take minutes; run on demand"]
fn ten_thousand_random_edit_sequences() {
    check_sequences(0..10_000);
}
