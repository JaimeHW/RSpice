//! Role-aware generated symbols for hierarchical cell instances.
//!
//! The schematic-from-ports analog of Virtuoso's "Create Cellview From
//! Cellview": a cell's interface ports produce a symbol whose pins land by
//! electrical role — inputs on the left edge, outputs and bidirectional pins
//! on the right, positive supplies on top, negative supplies and grounds
//! below, a thermal node on the bottom edge right of the rails. An amplifier
//! pin set draws the amplifier triangle. Every pin sits on the routing grid,
//! and the pin sequence stays in interface order so terminal index N is
//! always `.SUBCKT` port N.
//!
//! The block is sized to its own contents: the body is wide enough to print
//! every pin name inside it at the one pin-text size without the two name
//! columns meeting, and tall enough to keep the busier edge on the pin
//! pitch. Everything a renderer needs to draw one — body outline, which edge
//! a pin belongs to, where its lead ends, where and whether its name goes —
//! is published here, so the canvas, the SVG export, and the printed sheet
//! cannot drift apart.

use rspice_design_model::symbol_pin::SymbolPinSide;

use rspice_design_model::Point;
use rspice_design_model::port::{PinRole, PortDirection, PortSpec};

use crate::pin_roles::interface_pin;

/// Full width of the block catalog artwork is drawn against (terminals at
/// ±width/2).
pub const GENERATED_WIDTH: i32 = 60;
/// Lead length between a terminal and the body outline: one grid.
pub const GENERATED_STUB: i32 = 10;
/// Pin-name type size in symbol units: 8 pt at 100% zoom, which every
/// display draws above the legibility floor.
pub const GENERATED_PIN_LABEL_SIZE: f32 = 8.0;
/// Clear space between the body outline and a pin name.
pub const GENERATED_LABEL_GUTTER: i32 = 4;
/// Half the letter height of an instance's name and value, which are set
/// centred on their anchors.
const INSTANCE_LABEL_HALF_HEIGHT: i32 = 4;
/// Longest pin name printed in full. Imported interfaces may carry names far
/// past anything readable; the block stops growing for them and the renderer
/// elides instead, so one pathological port cannot produce a symbol nobody
/// can place.
pub const GENERATED_PIN_LABEL_MAX_CHARS: usize = 24;
/// Narrowest body a generated block draws.
const MIN_BODY_WIDTH: i32 = 60;
/// Vertical pitch between side pins, one routing-grid pair.
const PIN_PITCH: i32 = 20;
/// Horizontal pitch between pins sharing the top or bottom edge.
const RAIL_PITCH: i32 = 20;
/// Height and depth of a clock input's dynamic-input wedge.
const CLOCK_WEDGE_HALF: i32 = 3;
const CLOCK_WEDGE_DEPTH: i32 = 4;
/// Half-width of a thermal tap's bar and its distance from the body.
const THERMAL_BAR_HALF: i32 = 3;
const THERMAL_BAR_INSET: i32 = 4;

/// How wide a column of pin names is, in symbol units.
#[derive(Debug, Clone, Copy)]
struct NameMetrics {
    /// Monospace advance per character. IBM Plex Mono advances 600/1000 em.
    advance: f32,
    gutter: i32,
    column_gap: i32,
}

/// Names at [`GENERATED_PIN_LABEL_SIZE`].
const PRINTED_NAMES: NameMetrics = NameMetrics {
    advance: GENERATED_PIN_LABEL_SIZE * 0.6,
    gutter: GENERATED_LABEL_GUTTER,
    column_gap: 8,
};

/// The 5-unit names the block catalog artwork was drawn against. Catalog
/// artwork prints no names, but its terminals sit where this block put them.
const ARTWORK_NAMES: NameMetrics = NameMetrics {
    advance: 3.0,
    gutter: 3,
    column_gap: 4,
};

/// One pin of a generated symbol, in interface order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedPin {
    /// Port name (drawn inside the body, beside its lead).
    pub name: String,
    /// Declared direction.
    pub direction: PortDirection,
    /// Body edge this pin belongs to.
    pub side: SymbolPinSide,
    /// Terminal offset from the symbol origin, unrotated grid units.
    pub offset: Point,
}

/// What a generated symbol draws at a pin besides its lead.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PinMark {
    #[default]
    Plain,
    /// The dynamic-input wedge inside the body at the lead.
    Clock,
    /// A thermal tap: the bar across the lead of a temperature node.
    Thermal,
}

/// What a generated symbol knows about one port besides its name and
/// direction: its role and its mark.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterfacePin {
    pub role: PinRole,
    pub mark: PinMark,
}

impl InterfacePin {
    pub const fn new(role: PinRole) -> Self {
        Self {
            role,
            mark: PinMark::Plain,
        }
    }

    /// The edge the generation rules give this pin on a block.
    pub const fn inferred_side(self) -> SymbolPinSide {
        if matches!(self.mark, PinMark::Thermal) {
            return SymbolPinSide::Bottom;
        }
        match self.role {
            PinRole::NonInvertingInput | PinRole::InvertingInput | PinRole::Input => {
                SymbolPinSide::Left
            }
            PinRole::Output | PinRole::Bidirectional => SymbolPinSide::Right,
            PinRole::PositiveSupply => SymbolPinSide::Top,
            PinRole::NegativeSupply | PinRole::Ground => SymbolPinSide::Bottom,
        }
    }
}

/// How a generated symbol letters one pin's name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinLabel {
    /// Inside the body, the standard gutter in from the outline.
    Inside,
    /// Inside the body, starting this far in from the terminal: past a mark
    /// drawn at the lead.
    InsideFrom(i32),
    /// Not lettered; the body's marks name the pin.
    Hidden,
    /// Outside the body at `offset` from the terminal, its box reaching from
    /// there by `reach` (each step 1 or -1, 0 to centre on that axis).
    Beside { offset: Point, reach: Point },
}

/// The body a generated symbol draws.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedBody {
    /// A rectangle sized to the pin names.
    Block,
    /// The amplifier triangle: non-inverting input upper left, inverting
    /// lower left, output at the apex, positive supply on top and negative
    /// supply or ground below.
    Amplifier,
}

/// A complete generated symbol: pins plus body extents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedSymbol {
    /// Pins in interface order (terminal index == port index).
    pub pins: Vec<GeneratedPin>,
    /// Full symbol width (terminal to terminal).
    pub width: i32,
    /// Full symbol height, centred on the origin, holding every terminal.
    pub height: i32,
    pub body: GeneratedBody,
    /// Corner-to-corner extents of the drawn body.
    pub body_extent: (Point, Point),
    /// How each pin's name is lettered, parallel to `pins`.
    pub labels: Vec<PinLabel>,
    /// What each pin draws besides its lead, parallel to `pins`.
    pub marks: Vec<PinMark>,
}

impl GeneratedSymbol {
    /// A block whose pins a catalog places itself: the body one stub inside
    /// the side terminals, every name lettered inside it.
    pub fn fixed_block(pins: Vec<GeneratedPin>, width: i32, height: i32) -> Self {
        let (half_width, half_height) = (body_half_width(width), body_half_height(height));
        let count = pins.len();
        Self {
            pins,
            width,
            height,
            body: GeneratedBody::Block,
            body_extent: (
                Point::new(-half_width, -half_height),
                Point::new(half_width, half_height),
            ),
            labels: vec![PinLabel::Inside; count],
            marks: vec![PinMark::Plain; count],
        }
    }

    /// Symbol extents as `(width, height)` grid units.
    pub fn dimensions(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    /// Half-width of the drawn body.
    pub fn body_half_width(&self) -> i32 {
        let (min, max) = self.body_extent;
        (max.x - min.x) / 2
    }

    /// Corner-to-corner extents of the drawn body.
    pub fn body_bounds(&self) -> (Point, Point) {
        self.body_extent
    }

    /// The body as polylines, each with whether it closes. Every renderer
    /// draws exactly these, so the canvas, the SVG export and the printed
    /// sheet draw the same outline.
    pub fn body_outline(&self) -> Vec<(Vec<Point>, bool)> {
        let (min, max) = self.body_extent;
        let mut outline = match self.body {
            GeneratedBody::Block => vec![(
                vec![min, Point::new(max.x, min.y), max, Point::new(min.x, max.y)],
                true,
            )],
            GeneratedBody::Amplifier => amplifier_outline(&self.pins, min, max),
        };
        for (pin, mark) in self.pins.iter().zip(&self.marks) {
            let at = lead_inner(pin.offset, pin.side, Some(self.body_extent));
            let step = inward_step(pin.side);
            let across = Point::new(step.y, step.x);
            let point = |along: i32, side: i32| {
                Point::new(
                    at.x + step.x * along + across.x * side,
                    at.y + step.y * along + across.y * side,
                )
            };
            match mark {
                PinMark::Plain => {}
                PinMark::Clock => outline.push((
                    vec![
                        point(0, CLOCK_WEDGE_HALF),
                        point(CLOCK_WEDGE_DEPTH, 0),
                        point(0, -CLOCK_WEDGE_HALF),
                    ],
                    false,
                )),
                PinMark::Thermal => outline.push((
                    vec![
                        point(-THERMAL_BAR_INSET, THERMAL_BAR_HALF),
                        point(-THERMAL_BAR_INSET, -THERMAL_BAR_HALF),
                    ],
                    false,
                )),
            }
        }
        outline
    }

    /// Where the instance name goes: on the body's left edge, the line it is
    /// centred on far enough up that its letters end a gutter above the
    /// topmost thing the symbol draws.
    pub fn name_anchor(&self) -> Point {
        let (min, _) = self.body_extent;
        let top = self
            .pins
            .iter()
            .map(|pin| pin.offset.y)
            .chain([min.y])
            .min()
            .unwrap_or(min.y);
        Point::new(
            min.x,
            top - GENERATED_LABEL_GUTTER - INSTANCE_LABEL_HALF_HEIGHT,
        )
    }

    /// Where the cell name goes: on the body's left edge, its letters
    /// starting a gutter below the lowest terminal or body edge.
    pub fn value_anchor(&self) -> Point {
        let (min, max) = self.body_extent;
        let bottom = self
            .pins
            .iter()
            .map(|pin| pin.offset.y)
            .chain([max.y])
            .max()
            .unwrap_or(max.y);
        Point::new(
            min.x,
            bottom + GENERATED_LABEL_GUTTER + INSTANCE_LABEL_HALF_HEIGHT,
        )
    }
}

/// Full width and height of the amplifier template: terminals at ±40.
const AMPLIFIER_SIZE: i32 = 80;
/// Inputs sit one pin pitch apart, centred on the output.
const AMPLIFIER_INPUT_Y: i32 = PIN_PITCH;
/// The `+` and `-` marks: 8 units long, 4 in from the left edge.
const AMPLIFIER_MARK_INSET: i32 = 4;
const AMPLIFIER_MARK_LENGTH: i32 = 8;

fn amplifier_outline(pins: &[GeneratedPin], min: Point, max: Point) -> Vec<(Vec<Point>, bool)> {
    // Rail terminals sit on the body box; their leads continue inside it to
    // the sloped edges, which cross x = 0 halfway.
    let edge = max.y / 2;
    let left = min.x + AMPLIFIER_MARK_INSET;
    let right = left + AMPLIFIER_MARK_LENGTH;
    let centre = (left + right) / 2;
    let half = AMPLIFIER_MARK_LENGTH / 2;
    let mut outline = vec![(
        vec![min, Point::new(max.x, 0), Point::new(min.x, max.y)],
        true,
    )];
    for pin in pins {
        match pin.side {
            SymbolPinSide::Top => outline.push((vec![pin.offset, Point::new(0, -edge)], false)),
            SymbolPinSide::Bottom => outline.push((vec![pin.offset, Point::new(0, edge)], false)),
            SymbolPinSide::Left | SymbolPinSide::Right => {}
        }
    }
    outline.extend([
        (
            vec![
                Point::new(left, -AMPLIFIER_INPUT_Y),
                Point::new(right, -AMPLIFIER_INPUT_Y),
            ],
            false,
        ),
        (
            vec![
                Point::new(centre, -AMPLIFIER_INPUT_Y - half),
                Point::new(centre, -AMPLIFIER_INPUT_Y + half),
            ],
            false,
        ),
        (
            vec![
                Point::new(left, AMPLIFIER_INPUT_Y),
                Point::new(right, AMPLIFIER_INPUT_Y),
            ],
            false,
        ),
    ]);
    outline
}

/// Interface indices of an amplifier pin set: one input of each polarity,
/// one output, at most one positive supply and at most one negative supply
/// or ground, and nothing else. Comparators share the set.
struct AmplifierPins {
    non_inverting: usize,
    inverting: usize,
    output: usize,
    positive: Option<usize>,
    negative: Option<usize>,
}

fn amplifier_pins(roles: &[PinRole]) -> Option<AmplifierPins> {
    let at_most_one = |wanted: &[PinRole]| {
        let mut found = roles
            .iter()
            .enumerate()
            .filter(|(_, role)| wanted.contains(role));
        match (found.next(), found.next()) {
            (first, None) => Some(first.map(|(index, _)| index)),
            _ => None,
        }
    };
    let pins = AmplifierPins {
        non_inverting: at_most_one(&[PinRole::NonInvertingInput])??,
        inverting: at_most_one(&[PinRole::InvertingInput])??,
        output: at_most_one(&[PinRole::Output])??,
        positive: at_most_one(&[PinRole::PositiveSupply])?,
        negative: at_most_one(&[PinRole::NegativeSupply, PinRole::Ground])?,
    };
    let counted = 3 + usize::from(pins.positive.is_some()) + usize::from(pins.negative.is_some());
    (counted == roles.len()).then_some(pins)
}

/// Whether `roles` are an amplifier's pin set, the set a generated symbol
/// draws as the amplifier triangle.
pub fn is_amplifier_pin_set(roles: &[PinRole]) -> bool {
    amplifier_pins(roles).is_some()
}

fn generate_amplifier(ports: &[PortSpec], pins: &AmplifierPins) -> GeneratedSymbol {
    let half = AMPLIFIER_SIZE / 2;
    let rail_name = |below: i32| PinLabel::Beside {
        offset: Point::new(GENERATED_LABEL_GUTTER, below * PIN_PITCH / 2),
        reach: Point::new(1, 0),
    };
    let mut placement = vec![
        (
            SymbolPinSide::Bottom,
            Point::new(0, half),
            PinRole::Bidirectional,
            PinLabel::Hidden
        );
        ports.len()
    ];
    placement[pins.non_inverting] = (
        SymbolPinSide::Left,
        Point::new(-half, -AMPLIFIER_INPUT_Y),
        PinRole::NonInvertingInput,
        PinLabel::Hidden,
    );
    placement[pins.inverting] = (
        SymbolPinSide::Left,
        Point::new(-half, AMPLIFIER_INPUT_Y),
        PinRole::InvertingInput,
        PinLabel::Hidden,
    );
    placement[pins.output] = (
        SymbolPinSide::Right,
        Point::new(half, 0),
        PinRole::Output,
        PinLabel::Hidden,
    );
    if let Some(positive) = pins.positive {
        placement[positive] = (
            SymbolPinSide::Top,
            Point::new(0, -half),
            PinRole::PositiveSupply,
            rail_name(1),
        );
    }
    if let Some(negative) = pins.negative {
        placement[negative] = (
            SymbolPinSide::Bottom,
            Point::new(0, half),
            PinRole::NegativeSupply,
            rail_name(-1),
        );
    }
    let half_body = half - GENERATED_STUB;
    GeneratedSymbol {
        pins: ports
            .iter()
            .zip(&placement)
            .map(|(port, (side, offset, role, _))| GeneratedPin {
                name: port.name.clone(),
                direction: pin_direction(port.direction, *role),
                side: *side,
                offset: *offset,
            })
            .collect(),
        width: AMPLIFIER_SIZE,
        height: AMPLIFIER_SIZE,
        body: GeneratedBody::Amplifier,
        body_extent: (Point::new(-half_body, -half), Point::new(half_body, half)),
        labels: placement.iter().map(|(_, _, _, label)| *label).collect(),
        marks: vec![PinMark::Plain; ports.len()],
    }
}

/// Half-width of the body drawn inside a block of the given full width.
pub fn body_half_width(width: i32) -> i32 {
    (width / 2 - GENERATED_STUB).max(GENERATED_STUB)
}

/// Half-height of the body drawn inside a block of the given full height.
pub fn body_half_height(height: i32) -> i32 {
    (height / 2).max(GENERATED_STUB * 2)
}

/// Unit step from a terminal toward the body interior.
///
/// Renderers transform this through the instance orientation to decide how
/// a pin name is aligned, so a rotated block still reads outward-in.
pub fn inward_step(side: SymbolPinSide) -> Point {
    match side {
        SymbolPinSide::Left => Point::new(1, 0),
        SymbolPinSide::Right => Point::new(-1, 0),
        SymbolPinSide::Top => Point::new(0, 1),
        SymbolPinSide::Bottom => Point::new(0, -1),
    }
}

/// Inner end of a pin's lead: the point on the body outline that the pin's
/// own edge puts it against.
///
/// The edge decides the axis — never the terminal's own coordinates, which
/// say the wrong thing the moment a body is taller than it is wide, turning
/// an outer side pin's lead through ninety degrees so it never arrives. The
/// body decides the length, so a lead always lands on the outline instead of
/// stopping a fixed distance short of it. Without a body to reach, the lead
/// falls back to one stub length.
pub fn lead_inner(terminal: Point, side: SymbolPinSide, body: Option<(Point, Point)>) -> Point {
    let step = inward_step(side);
    let Some((min, max)) = body else {
        return Point::new(
            terminal.x + step.x * GENERATED_STUB,
            terminal.y + step.y * GENERATED_STUB,
        );
    };
    match side {
        SymbolPinSide::Left => Point::new(min.x.max(terminal.x), terminal.y),
        SymbolPinSide::Right => Point::new(max.x.min(terminal.x), terminal.y),
        SymbolPinSide::Top => Point::new(terminal.x, min.y.max(terminal.y)),
        SymbolPinSide::Bottom => Point::new(terminal.x, max.y.min(terminal.y)),
    }
}

/// Where a pin's name is drawn: just inside the body outline, on the pin's
/// own lead axis. The renderer aligns the text away from this point, so the
/// name reads into the body and never crosses the outline it belongs to.
pub fn pin_label_anchor(
    terminal: Point,
    side: SymbolPinSide,
    body: Option<(Point, Point)>,
) -> Point {
    let inner = lead_inner(terminal, side, body);
    let step = inward_step(side);
    Point::new(
        inner.x + step.x * GENERATED_LABEL_GUTTER,
        inner.y + step.y * GENERATED_LABEL_GUTTER,
    )
}

/// Evenly spaced side-pin Y coordinates: `n` pins at `PIN_PITCH`, centred
/// on the origin, always on the 10-unit grid.
fn side_axis(n: usize) -> Vec<i32> {
    (0..n as i32)
        .map(|idx| idx * PIN_PITCH - (n as i32 - 1) * (PIN_PITCH / 2))
        .collect()
}

/// X coordinate of the `k`-th pin sharing the top or bottom edge: 0, then
/// fanning out by `RAIL_PITCH` alternating right/left of centre.
fn rail_x(k: usize) -> i32 {
    let step = k.div_ceil(2) as i32 * RAIL_PITCH;
    if k % 2 == 1 { step } else { -step }
}

/// The pin name as it is printed: the name itself, or an elided form that
/// fits the width the block reserved for it.
pub fn fit_pin_name(name: &str) -> String {
    if name.chars().count() <= GENERATED_PIN_LABEL_MAX_CHARS {
        return name.to_owned();
    }
    let mut fitted: String = name
        .chars()
        .take(GENERATED_PIN_LABEL_MAX_CHARS - 1)
        .collect();
    fitted.push('…');
    fitted
}

/// Width of a printed pin name in symbol units, rounded up.
fn name_span(name: &str, metrics: NameMetrics) -> i32 {
    let chars = name.chars().count().min(GENERATED_PIN_LABEL_MAX_CHARS) as f32;
    (chars * metrics.advance).ceil() as i32
}

/// Round `width` up so half of it — where the terminals land — stays on the
/// routing grid.
fn onto_grid(width: i32) -> i32 {
    let step = 2 * GENERATED_STUB;
    width.saturating_add(step - 1) / step * step
}

/// The block catalog artwork is drawn against: pins by declared direction
/// alone, inputs left, outputs and bidirectional pins right, supply rails
/// alternating top and bottom on the body edge, the width fitted to 5-unit
/// names. Catalog artwork matches its terminals here, so this geometry is
/// frozen.
pub fn generate_artwork_block(ports: &[PortSpec]) -> GeneratedSymbol {
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut rails = Vec::new();
    for (index, port) in ports.iter().enumerate() {
        match port.direction {
            PortDirection::In => left.push(index),
            PortDirection::Out | PortDirection::InOut => right.push(index),
            PortDirection::Supply => rails.push(index),
        }
    }
    let rows = left.len().max(right.len()).max(1);
    let height = (rows as i32 * PIN_PITCH).max(40);
    let rail_reach = (0..rails.len())
        .map(|slot| rail_x(slot / 2).abs())
        .max()
        .unwrap_or(0);
    let span = |slots: &[usize]| {
        slots
            .iter()
            .map(|&index| name_span(&ports[index].name, ARTWORK_NAMES))
            .max()
            .unwrap_or(0)
    };
    let (left_span, right_span) = (span(&left), span(&right));
    let gap = if left_span > 0 && right_span > 0 {
        ARTWORK_NAMES.column_gap
    } else {
        0
    };
    let names = 2 * ARTWORK_NAMES.gutter + left_span + right_span + gap;
    let width = onto_grid(
        names
            .max(2 * (rail_reach + RAIL_PITCH / 2))
            .saturating_add(2 * GENERATED_STUB)
            .max(GENERATED_WIDTH),
    );
    let (hw, hh) = (width / 2, body_half_height(height));
    let mut placement = vec![(SymbolPinSide::Left, Point::origin()); ports.len()];
    for (slot, &index) in left.iter().enumerate() {
        placement[index] = (
            SymbolPinSide::Left,
            Point::new(-hw, side_axis(left.len())[slot]),
        );
    }
    for (slot, &index) in right.iter().enumerate() {
        placement[index] = (
            SymbolPinSide::Right,
            Point::new(hw, side_axis(right.len())[slot]),
        );
    }
    for (slot, &index) in rails.iter().enumerate() {
        let (side, edge) = if slot % 2 == 0 {
            (SymbolPinSide::Top, -hh)
        } else {
            (SymbolPinSide::Bottom, hh)
        };
        placement[index] = (side, Point::new(rail_x(slot / 2), edge));
    }
    let pins = ports
        .iter()
        .zip(placement)
        .map(|(port, (side, offset))| GeneratedPin {
            name: port.name.clone(),
            direction: port.direction,
            side,
            offset,
        })
        .collect();
    GeneratedSymbol::fixed_block(pins, width, height)
}

/// Generate the symbol for an interface by the generation rules: each
/// port's role read from its name and declared direction.
///
/// Empty interfaces produce an empty pin list with the minimum body —
/// callers treat that as "nothing to place yet".
pub fn generate_symbol(ports: &[PortSpec]) -> GeneratedSymbol {
    generate_symbol_with_roles(ports, &[])
}

/// Generate the symbol for an interface whose pin roles are known, parallel
/// to `ports`. Roles that do not cover the interface are read from the
/// ports' names and directions instead.
pub fn generate_symbol_with_roles(ports: &[PortSpec], roles: &[PinRole]) -> GeneratedSymbol {
    let pins = if roles.len() == ports.len() {
        ports
            .iter()
            .zip(roles)
            .map(|(port, &role)| InterfacePin {
                mark: interface_pin(&port.name, port.direction, false).mark,
                role,
            })
            .collect::<Vec<_>>()
    } else {
        ports
            .iter()
            .map(|port| interface_pin(&port.name, port.direction, false))
            .collect()
    };
    generate_interface_symbol(ports, &pins)
}

/// Generate the symbol for an interface whose pins are described, parallel
/// to `ports`. An amplifier pin set draws the triangle; any other draws a
/// block with each pin on the edge its role takes.
pub fn generate_interface_symbol(ports: &[PortSpec], pins: &[InterfacePin]) -> GeneratedSymbol {
    let described = pins.len() == ports.len();
    let pin = |index: usize| {
        if described {
            pins[index]
        } else {
            interface_pin(&ports[index].name, ports[index].direction, false)
        }
    };
    let roles = (0..ports.len())
        .map(|index| pin(index).role)
        .collect::<Vec<_>>();
    if let Some(amplifier) = amplifier_pins(&roles) {
        return generate_amplifier(ports, &amplifier);
    }

    // Bucket by edge, in interface order within each edge; on the right,
    // outputs come before bidirectional pins, and on the bottom, rails
    // before thermal nodes.
    let mut edges: [Vec<usize>; 4] = Default::default();
    for index in 0..ports.len() {
        edges[side_slot(pin(index).inferred_side())].push(index);
    }
    edges[side_slot(SymbolPinSide::Right)]
        .sort_by_key(|&index| (pin(index).role == PinRole::Bidirectional, index));
    edges[side_slot(SymbolPinSide::Bottom)]
        .sort_by_key(|&index| (pin(index).mark == PinMark::Thermal, index));
    let [left, right, top, bottom] = &edges;

    // Top and bottom pins fan out from the centre of their edge; thermal
    // nodes continue right of the last rail.
    let mut x_of = vec![0; ports.len()];
    for edge in [top, bottom] {
        let mut fanned = 0;
        let mut rightmost: Option<i32> = None;
        for &index in edge {
            let x = if pin(index).mark == PinMark::Thermal && edge == bottom {
                rightmost.map_or(RAIL_PITCH, |x| x + RAIL_PITCH)
            } else {
                let x = rail_x(fanned);
                fanned += 1;
                x
            };
            rightmost = Some(rightmost.map_or(x, |most| most.max(x)));
            x_of[index] = x;
        }
    }

    let rows = left.len().max(right.len()).max(2) as i32;
    let half_side = rows * PIN_PITCH / 2;
    let top_edge = -half_side - if top.is_empty() { 0 } else { GENERATED_STUB };
    let bottom_edge = half_side + if bottom.is_empty() { 0 } else { GENERATED_STUB };
    let column = |slots: &[usize]| {
        slots
            .iter()
            .map(|&index| {
                let mark = if pin(index).mark == PinMark::Clock {
                    CLOCK_WEDGE_DEPTH
                } else {
                    0
                };
                name_span(&ports[index].name, PRINTED_NAMES) + mark
            })
            .max()
            .unwrap_or(0)
    };
    let (left_span, right_span) = (column(left), column(right));
    let gap = if left_span > 0 && right_span > 0 {
        PRINTED_NAMES.column_gap
    } else {
        0
    };
    let names = 2 * PRINTED_NAMES.gutter + left_span + right_span + gap;
    let rail_reach = top
        .iter()
        .chain(bottom)
        .map(|&index| x_of[index].abs() + GENERATED_STUB)
        .max()
        .unwrap_or(0);
    let body_width = onto_grid(names.max(2 * rail_reach).max(MIN_BODY_WIDTH));
    let half_body = body_width / 2;
    let terminal_x = half_body + GENERATED_STUB;

    let mut placement = vec![(SymbolPinSide::Left, Point::origin()); ports.len()];
    for (axis, &index) in side_axis(left.len()).into_iter().zip(left) {
        placement[index] = (SymbolPinSide::Left, Point::new(-terminal_x, axis));
    }
    for (axis, &index) in side_axis(right.len()).into_iter().zip(right) {
        placement[index] = (SymbolPinSide::Right, Point::new(terminal_x, axis));
    }
    for &index in top {
        placement[index] = (
            SymbolPinSide::Top,
            Point::new(x_of[index], top_edge - GENERATED_STUB),
        );
    }
    for &index in bottom {
        placement[index] = (
            SymbolPinSide::Bottom,
            Point::new(x_of[index], bottom_edge + GENERATED_STUB),
        );
    }
    let reach = placement
        .iter()
        .map(|(_, offset)| offset.y.abs())
        .chain([top_edge.abs(), bottom_edge.abs()])
        .max()
        .unwrap_or(half_side);
    let marks = (0..ports.len())
        .map(|index| match (pin(index).mark, placement[index].0) {
            // A wedge marks a clock where it enters the body from the side.
            (PinMark::Clock, SymbolPinSide::Left | SymbolPinSide::Right) => PinMark::Clock,
            (PinMark::Thermal, _) => PinMark::Thermal,
            _ => PinMark::Plain,
        })
        .collect::<Vec<_>>();
    let labels = marks
        .iter()
        .map(|mark| match mark {
            PinMark::Clock => {
                PinLabel::InsideFrom(GENERATED_STUB + CLOCK_WEDGE_DEPTH + GENERATED_LABEL_GUTTER)
            }
            PinMark::Plain | PinMark::Thermal => PinLabel::Inside,
        })
        .collect();
    GeneratedSymbol {
        pins: ports
            .iter()
            .zip(placement)
            .enumerate()
            .map(|(index, (port, (side, offset)))| GeneratedPin {
                name: port.name.clone(),
                direction: pin_direction(port.direction, pin(index).role),
                side,
                offset,
            })
            .collect(),
        width: 2 * terminal_x,
        height: 2 * reach,
        body: GeneratedBody::Block,
        body_extent: (
            Point::new(-half_body, top_edge),
            Point::new(half_body, bottom_edge),
        ),
        labels,
        marks,
    }
}

/// The direction a generated pin states: the declared one, except that a
/// bidirectional port playing a supply's role is drawn as the rail it is.
const fn pin_direction(declared: PortDirection, role: PinRole) -> PortDirection {
    match (declared, role) {
        (
            PortDirection::InOut,
            PinRole::PositiveSupply | PinRole::NegativeSupply | PinRole::Ground,
        ) => PortDirection::Supply,
        _ => declared,
    }
}

const fn side_slot(side: SymbolPinSide) -> usize {
    match side {
        SymbolPinSide::Left => 0,
        SymbolPinSide::Right => 1,
        SymbolPinSide::Top => 2,
        SymbolPinSide::Bottom => 3,
    }
}

#[cfg(test)]
mod tests;
