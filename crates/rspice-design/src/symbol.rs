//! Persistent symbol-view document state.
//!
//! A symbol view is the public contract of a cell: body artwork, label
//! anchors, and the pins that a parent schematic can wire to. The paired
//! schematic supplies the canonical port list when it exists; this document
//! stores the user's symbol-specific placement and artwork.
//!
//! Authored text is body artwork, not authoring metadata: a `+`, a `−` or an
//! `OTA` is what makes the body readable, so it lives in
//! [`SymbolDocument::body`] as [`SymbolShape::Text`] and reaches every
//! surface the rest of the body reaches — canvas, preview, SVG export and
//! extents alike.

pub mod edit;
pub mod generated;
pub mod publication;

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use sha2::{Digest as ShaDigest, Sha256};

use rspice_vocabulary::product::ContentDigest;

pub use rspice_design_model::symbol_geometry::{
    ARROW_LENGTH, SymbolCurve, SymbolDash, SymbolFill, SymbolShape, SymbolStroke, SymbolStyle,
    SymbolTextAlign, SymbolTextPlacement, SymbolTextRotation, SymbolTextSize, arrow_corners,
    rectangle_corners, symbol_label_sample, symbol_label_text, symbol_text_bounds,
};

use crate::library::View;
use crate::schematic::label_layout::TextEdge;
use crate::symbol_generation::{GeneratedSymbol, PinLabel, PinMark, generate_symbol_with_roles};
use rspice_design_model::{
    Point,
    bus::{BusDeclaration, declared_vector, declared_width},
    port::{PinRole, PortDirection, PortSpec},
    symbol_pin::{SymbolElectricalType, SymbolPinSide, SymbolPinStyle},
};

#[cfg(any(test, feature = "symbol-test-observation"))]
thread_local! {
    /// How many symbol cellviews have been deserialized on this thread.
    ///
    /// A symbol cellview keeps its typed model binding and its artwork as two
    /// JSON documents in the view's metadata, and reading either one costs a
    /// deserialization plus a validation pass. That is the expensive half of
    /// deriving anything about a symbol, it depends only on the view, and a
    /// surface that derives its rows from the whole registry on every frame
    /// pays it per symbol in the corpus rather than per row on screen — which
    /// is what the Symbols page did. Counting is the only way to state that as
    /// a test: the work is invisible in the result, and a timing assertion on a
    /// fixture small enough to build is noise.
    pub static SYMBOL_VIEW_PARSES: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };

}

/// Metadata key used by `View::metadata` for the symbol document JSON.
pub const SYMBOL_DOCUMENT_METADATA_KEY: &str = "rspice.symbol.document.v1";
/// Metadata key for symbol-editor-only text, attributes and revision history.
///
/// These records are kept beside the geometric document so older renderers
/// can continue to consume `rspice.symbol.document.v1` without silently
/// discarding the richer authoring contract.
pub const SYMBOL_EDITOR_METADATA_KEY: &str = "rspice.symbol.editor.v1";
/// Schema 4 records each revision as the project save that wrote it: when it
/// was saved, the digest of the artwork and the ordered pin names, with no
/// note.
pub const SYMBOL_EDITOR_METADATA_SCHEMA_VERSION: u16 = 4;

/// Resource and geometry limits for authored symbol metadata.
///
/// Symbol documents are embedded in project metadata and may originate from
/// imported libraries. These limits are deliberately generous for practical
/// symbols while keeping parsing, tessellation, and bounds calculations
/// deterministic for malformed or hostile input.
pub const MAX_SYMBOL_DOCUMENT_BYTES: usize = 4 * 1024 * 1024;
pub const MAX_SYMBOL_PINS: usize = 16_384;
pub const MAX_SYMBOL_SHAPES: usize = 100_000;
pub const MAX_SYMBOL_POINTS: usize = 1_000_000;
pub const MAX_SYMBOL_PIN_NAME_BYTES: usize = 1_024;
pub const MAX_SYMBOL_COORDINATE_ABS: i32 = 1_000_000_000;
pub const MAX_SYMBOL_RADIUS: i32 = 1_000_000_000;
pub const MAX_SYMBOL_TEXT_OBJECTS: usize = 16_384;
pub const MAX_SYMBOL_TEXT_BYTES: usize = 16 * 1024;
/// The revision records a symbol keeps; a save past this many lets the
/// oldest go.
pub const MAX_SYMBOL_REVISION_RECORDS: usize = 4_096;

/// Terminal-grid spacing in schematic coordinate units.
///
/// The design spec shows this as a 40 px editor lattice; in the Rust model
/// one schematic unit renders as four px at the symbol editor's 100% scale.
pub const SYMBOL_TERMINAL_GRID: i32 = 10;

/// The lead a new pin is given: two terminal pitches.
pub const SYMBOL_PIN_LENGTH: i32 = SYMBOL_TERMINAL_GRID * 2;

/// A pin is its tip, the side of the symbol it sits on, the length of the
/// lead that runs from the tip toward the body, and the mark its lead ends
/// in. Every one of them is stored: nothing about a pin is worked out from
/// the artwork once it is placed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolPin {
    pub name: String,
    pub direction: PortDirection,
    /// The tip: the terminal a wire attaches to. `None` while unplaced.
    pub position: Option<Point>,
    /// Rich electrical semantics retained by model-bound symbols; absent,
    /// the direction implies them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub electrical_type: Option<SymbolElectricalType>,
    /// The side of the symbol the pin sits on. Its lead runs from the tip
    /// toward the body, away from this side.
    pub side: SymbolPinSide,
    /// How far the lead runs from the tip toward the body, in symbol units.
    /// Zero makes the pin a bare connection point: no lead and no mark.
    pub length: i32,
    /// The mark drawn where the lead meets the body.
    #[serde(default, skip_serializing_if = "SymbolPinStyle::is_plain")]
    pub style: SymbolPinStyle,
    /// The drawing letters no name at this pin: its body's artwork marks it
    /// instead, or the drawing it came from showed none. The name still
    /// names the terminal everywhere else.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub name_hidden: bool,
    /// How far in from the terminal the drawing this pin came from starts
    /// its name. The name never starts closer than the standard gutter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_offset: Option<i32>,
    /// Where the drawing this pin came from letters its name, when it sets
    /// the place itself rather than a distance in from the terminal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_at: Option<PinNamePlacement>,
}

/// A pin name's place as the drawing the pin came from sets it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PinNamePlacement {
    /// The name's anchor from the pin's terminal, in symbol coordinates.
    pub offset: Point,
    /// Which way the name's box reaches from its anchor, across and down:
    /// each step 1 or -1, or 0 where the box is centred on that axis.
    pub reach: Point,
    /// The name reads upward, turned a quarter counter-clockwise about its
    /// anchor, rather than upright.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub upward: bool,
}

impl SymbolPin {
    /// A pin with the standard lead, on the side its tip suggests: the axis
    /// the tip stands farther out along, or, unplaced, the side its
    /// direction takes.
    pub fn new(name: impl Into<String>, direction: PortDirection, position: Option<Point>) -> Self {
        Self {
            name: name.into(),
            direction,
            position,
            electrical_type: Some(default_electrical_type(direction)),
            side: default_pin_side(direction, position),
            length: SYMBOL_PIN_LENGTH,
            style: SymbolPinStyle::Plain,
            name_hidden: false,
            name_offset: None,
            name_at: None,
        }
    }

    /// The same pin on `side`.
    pub fn with_side(mut self, side: SymbolPinSide) -> Self {
        self.side = side;
        self
    }

    /// The same pin with a lead `length` long.
    pub fn with_length(mut self, length: i32) -> Self {
        self.length = length;
        self
    }

    /// The same pin drawing `style` where its lead meets the body.
    pub fn with_style(mut self, style: SymbolPinStyle) -> Self {
        self.style = style;
        self
    }

    /// Where the lead meets the body: the tip stepped toward the body by the
    /// lead's length. The tip itself for a bare connection point.
    pub fn lead_inner(&self) -> Option<Point> {
        self.position
            .map(|tip| lead_inner_end(tip, self.side, self.length))
    }

    /// Whether this pin states the direction of a port declaring `declared`.
    /// A bidirectional terminal may be drawn as the rail it is.
    pub fn states_direction(&self, declared: PortDirection) -> bool {
        self.direction == declared
            || (declared == PortDirection::InOut && self.direction == PortDirection::Supply)
    }

    /// The same pin with its name left off the drawing, or lettered.
    pub fn with_name_hidden(mut self, hidden: bool) -> Self {
        self.name_hidden = hidden;
        self
    }

    /// The same pin with its name starting this far in from the terminal,
    /// or at the standard gutter.
    pub fn with_name_offset(mut self, offset: Option<i32>) -> Self {
        self.name_offset = offset;
        self
    }

    /// The same pin with its name lettered where the drawing it came from
    /// sets it, or where the symbol's own layout puts it.
    pub fn with_name_at(mut self, at: Option<PinNamePlacement>) -> Self {
        self.name_at = at;
        self
    }

    /// The same pin with `electrical_type`, on `side`.
    pub fn with_contract(
        mut self,
        electrical_type: SymbolElectricalType,
        side: SymbolPinSide,
    ) -> Self {
        self.electrical_type = Some(electrical_type);
        self.side = side;
        self
    }

    pub fn electrical_type(&self) -> SymbolElectricalType {
        self.electrical_type
            .unwrap_or_else(|| default_electrical_type(self.direction))
    }

    pub fn set_electrical_contract(
        &mut self,
        electrical_type: SymbolElectricalType,
        direction: PortDirection,
    ) {
        self.electrical_type = Some(electrical_type);
        self.direction = direction;
    }

    pub fn terminal_on_grid(&self) -> bool {
        self.position.is_some_and(|point| {
            point.x % SYMBOL_TERMINAL_GRID == 0 && point.y % SYMBOL_TERMINAL_GRID == 0
        })
    }

    /// The vector this pin declares, when its name declares one.
    ///
    /// A symbol pin reads its width from the same authority as the interface
    /// port it stands for: the name. One terminal drawn `DATA[7:0]` is one
    /// pin of eight conductors, so the symbol and the schematic cannot come to
    /// different conclusions about how many nodes an instance carries.
    pub fn vector(&self) -> Option<BusDeclaration> {
        declared_vector(&self.name)
    }

    /// Conductors this pin carries: the declared width, or one.
    pub fn width(&self) -> usize {
        declared_width(&self.name)
    }
}

fn default_electrical_type(direction: PortDirection) -> SymbolElectricalType {
    match direction {
        PortDirection::Supply => SymbolElectricalType::Power,
        PortDirection::In | PortDirection::Out | PortDirection::InOut => {
            SymbolElectricalType::Analog
        }
    }
}

fn default_pin_side(direction: PortDirection, position: Option<Point>) -> SymbolPinSide {
    if let Some(position) = position {
        if position.x.abs() >= position.y.abs() {
            if position.x < 0 {
                return SymbolPinSide::Left;
            }
            return SymbolPinSide::Right;
        }
        if position.y < 0 {
            return SymbolPinSide::Top;
        }
        return SymbolPinSide::Bottom;
    }
    match direction {
        PortDirection::In => SymbolPinSide::Left,
        PortDirection::Out | PortDirection::InOut => SymbolPinSide::Right,
        PortDirection::Supply => SymbolPinSide::Top,
    }
}

/// How a run of symbol text reading `rotation`'s way hangs off its anchor to
/// land at `placement`, as the label layout's horizontal and vertical edges.
/// A run read upward is hung upright and then turned a quarter
/// counter-clockwise about its anchor, so its upright hang is the one that
/// turn carries to `placement`. The canvas and print both hang a symbol's
/// lettering by this.
pub fn symbol_text_hang(
    placement: SymbolTextPlacement,
    rotation: SymbolTextRotation,
) -> (TextEdge, TextEdge) {
    use SymbolTextPlacement::{Above, After, Before, Below, On};
    use SymbolTextRotation::{R0, R90};
    use TextEdge::{Center, End, Start};
    match (rotation, placement) {
        (_, On) => (Center, Center),
        (R0, After) | (R90, Above) => (Start, Center),
        (R0, Before) | (R90, Below) => (End, Center),
        (R0, Below) | (R90, After) => (Center, Start),
        (R0, Above) | (R90, Before) => (Center, End),
    }
}

/// Which body edge a terminal belongs to, judged against the artwork it is
/// drawn beside.
///
/// A terminal outside the body belongs to the edge it stands off from; one
/// on or inside the outline belongs to the edge it is nearest. Comparing the
/// terminal's own `|x|` against its `|y|` cannot answer this — on any body
/// taller than it is wide, that reads an outer side pin as a rail and turns
/// its lead through ninety degrees, away from the body it should reach.
pub fn pin_side_against_body(
    position: Point,
    direction: PortDirection,
    body_bounds: Option<(Point, Point)>,
) -> SymbolPinSide {
    let Some(bounds) = body_bounds else {
        return default_pin_side(direction, Some(position));
    };
    let (min, max) = bounds;
    let outside = SymbolPinSide::ALL.map(|side| (side, edge_standoff(position, bounds, side)));
    if let Some((side, _)) = outside
        .iter()
        .filter(|(_, standoff)| *standoff > 0)
        .max_by_key(|(_, standoff)| *standoff)
    {
        return *side;
    }
    let inside = [
        (SymbolPinSide::Left, position.x.saturating_sub(min.x)),
        (SymbolPinSide::Right, max.x.saturating_sub(position.x)),
        (SymbolPinSide::Top, position.y.saturating_sub(min.y)),
        (SymbolPinSide::Bottom, max.y.saturating_sub(position.y)),
    ];
    inside
        .iter()
        .min_by_key(|(_, reach)| *reach)
        .map(|(side, _)| *side)
        .unwrap_or_else(|| default_pin_side(direction, Some(position)))
}

/// How far a terminal stands off one body edge: positive outside the body,
/// zero on the edge itself, negative inside.
fn edge_standoff(position: Point, (min, max): (Point, Point), side: SymbolPinSide) -> i32 {
    match side {
        SymbolPinSide::Left => min.x.saturating_sub(position.x),
        SymbolPinSide::Right => position.x.saturating_sub(max.x),
        SymbolPinSide::Top => min.y.saturating_sub(position.y),
        SymbolPinSide::Bottom => position.y.saturating_sub(max.y),
    }
}

/// The body end of a lead `length` long from `tip`, on a pin on `side`.
pub fn lead_inner_end(tip: Point, side: SymbolPinSide, length: i32) -> Point {
    let step = crate::symbol_generation::inward_step(side);
    Point::new(
        tip.x.saturating_add(step.x.saturating_mul(length)),
        tip.y.saturating_add(step.y.saturating_mul(length)),
    )
}

/// How long a lead from `tip`, on a pin on `side`, must be to reach the edge
/// of `body`: zero where the tip is on or inside that edge.
///
/// A suggestion for a pin being placed, never a reading of a placed one.
pub fn lead_length_to_body(tip: Point, side: SymbolPinSide, body: (Point, Point)) -> i32 {
    edge_standoff(tip, body, side).max(0)
}

/// Radius of an inverted pin's bubble, in symbol units.
pub const PIN_BUBBLE_RADIUS: i32 = 3;
/// Half the height of a clock pin's wedge, across its lead.
pub const PIN_WEDGE_HALF: i32 = 3;
/// How far a clock pin's wedge reaches into the body.
pub const PIN_WEDGE_DEPTH: i32 = 4;

/// What a placed pin draws: its lead, and the marks its style puts where the
/// lead meets the body. A bare connection point draws none of them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PinDrawing {
    /// The lead, tip first. It stops at the bubble when there is one.
    pub lead: Option<(Point, Point)>,
    /// The bubble's centre and radius, just outside the body.
    pub bubble: Option<(Point, i32)>,
    /// The wedge, its two ends on the body edge and its point inside.
    pub wedge: Option<[Point; 3]>,
}

impl PinDrawing {
    /// The same drawing moved by `delta`.
    #[must_use]
    pub fn shifted(self, delta: Point) -> Self {
        Self {
            lead: self.lead.map(|(from, to)| (from + delta, to + delta)),
            bubble: self.bubble.map(|(centre, radius)| (centre + delta, radius)),
            wedge: self
                .wedge
                .map(|corners| corners.map(|corner| corner + delta)),
        }
    }
}

/// The lead and marks of a pin at `tip` on `side`, `length` long, in
/// `style`.
pub fn pin_drawing(
    tip: Point,
    side: SymbolPinSide,
    length: i32,
    style: SymbolPinStyle,
) -> PinDrawing {
    if length <= 0 {
        return PinDrawing {
            lead: None,
            bubble: None,
            wedge: None,
        };
    }
    let inner = lead_inner_end(tip, side, length);
    let step = crate::symbol_generation::inward_step(side);
    let across = Point::new(step.y, step.x);
    let at = |along: i32, aside: i32| {
        Point::new(
            inner.x + step.x * along + across.x * aside,
            inner.y + step.y * along + across.y * aside,
        )
    };
    let bubble_span = if style.bubble() {
        (2 * PIN_BUBBLE_RADIUS).min(length)
    } else {
        0
    };
    let lead_end = at(-bubble_span, 0);
    PinDrawing {
        lead: (lead_end != tip).then_some((tip, lead_end)),
        bubble: style
            .bubble()
            .then(|| (at(-bubble_span / 2, 0), bubble_span / 2)),
        wedge: style.wedge().then(|| {
            [
                at(0, PIN_WEDGE_HALF),
                at(PIN_WEDGE_DEPTH, 0),
                at(0, -PIN_WEDGE_HALF),
            ]
        }),
    }
}

/// Where a pin's name starts by default: a gutter past the body end of its
/// lead, and past the wedge of a clock.
pub fn pin_name_anchor(
    tip: Point,
    side: SymbolPinSide,
    length: i32,
    style: SymbolPinStyle,
) -> Point {
    let inner = lead_inner_end(tip, side, length);
    let wedge = if length > 0 && style.wedge() {
        PIN_WEDGE_DEPTH
    } else {
        0
    };
    lead_inner_end(
        inner,
        side,
        wedge + crate::symbol_generation::GENERATED_LABEL_GUTTER,
    )
}

/// The coordinate a tip on `side` is placed by along that side.
fn along_side(side: SymbolPinSide, tip: Point) -> i32 {
    match side {
        SymbolPinSide::Left | SymbolPinSide::Right => tip.y,
        SymbolPinSide::Top | SymbolPinSide::Bottom => tip.x,
    }
}

/// The tip of a pin on `side` standing `length` off that edge of `body`,
/// `along` the edge.
fn tip_off_body(side: SymbolPinSide, along: i32, length: i32, (min, max): (Point, Point)) -> Point {
    match side {
        SymbolPinSide::Left => Point::new(min.x.saturating_sub(length), along),
        SymbolPinSide::Right => Point::new(max.x.saturating_add(length), along),
        SymbolPinSide::Top => Point::new(along, min.y.saturating_sub(length)),
        SymbolPinSide::Bottom => Point::new(along, max.y.saturating_add(length)),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolPinElectricalKind {
    AnalogInput,
    AnalogOutput,
    AnalogBidirectional,
    Power,
    Ground,
    DigitalInput,
    DigitalOutput,
}

impl SymbolPinElectricalKind {
    pub const ALL: [Self; 7] = [
        Self::AnalogInput,
        Self::AnalogOutput,
        Self::AnalogBidirectional,
        Self::Power,
        Self::Ground,
        Self::DigitalInput,
        Self::DigitalOutput,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::AnalogInput => "analog input",
            Self::AnalogOutput => "analog output",
            Self::AnalogBidirectional => "analog bidirectional",
            Self::Power => "power",
            Self::Ground => "ground",
            Self::DigitalInput => "digital input",
            Self::DigitalOutput => "digital output",
        }
    }

    pub const fn contract(self) -> (SymbolElectricalType, PortDirection) {
        match self {
            Self::AnalogInput => (SymbolElectricalType::Analog, PortDirection::In),
            Self::AnalogOutput => (SymbolElectricalType::Analog, PortDirection::Out),
            Self::AnalogBidirectional => (SymbolElectricalType::Analog, PortDirection::InOut),
            Self::Power => (SymbolElectricalType::Power, PortDirection::Supply),
            Self::Ground => (SymbolElectricalType::Ground, PortDirection::Supply),
            Self::DigitalInput => (SymbolElectricalType::Logic, PortDirection::In),
            Self::DigitalOutput => (SymbolElectricalType::Logic, PortDirection::Out),
        }
    }

    pub fn from_pin(pin: &SymbolPin) -> Self {
        match (pin.electrical_type(), pin.direction) {
            (SymbolElectricalType::Logic, PortDirection::Out) => Self::DigitalOutput,
            (SymbolElectricalType::Logic, _) => Self::DigitalInput,
            (SymbolElectricalType::Power, _) => Self::Power,
            (SymbolElectricalType::Ground, _) => Self::Ground,
            (_, PortDirection::In) => Self::AnalogInput,
            (_, PortDirection::Out) => Self::AnalogOutput,
            (_, PortDirection::InOut | PortDirection::Supply) => Self::AnalogBidirectional,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolAttributeKind {
    Reference,
    Value,
    Model,
}

impl SymbolAttributeKind {
    pub const ALL: [Self; 3] = [Self::Reference, Self::Value, Self::Model];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Reference => "refdes",
            Self::Value => "value",
            Self::Model => "model",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolAttribute {
    pub kind: SymbolAttributeKind,
    pub default_value: String,
    pub shown: bool,
    pub position: Point,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SymbolEditorMetadata {
    pub schema_version: u16,
    pub attributes: Vec<SymbolAttribute>,
    pub revision: u64,
    pub revisions: Vec<SymbolRevisionRecord>,
}

/// One saved symbol revision: a project save that wrote changed artwork.
///
/// The record identifies what was saved, not how much of it there was.
/// Counts cannot answer the question a revision history exists to answer —
/// two documents with three pins and four shapes are routinely different
/// documents, and a rename changes neither number. The digest settles
/// whether two revisions are the same artwork, and the ordered pin names are
/// the netlist contract that downstream instances were wired against.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolRevisionRecord {
    pub revision: u64,
    /// When the save that wrote this revision ran, in milliseconds since the
    /// Unix epoch.
    pub saved_unix_ms: u64,
    /// Content digest of the saved document.
    pub document_digest: String,
    /// Pin names in netlist order when saved.
    pub pin_names: Vec<String>,
}

impl Default for SymbolEditorMetadata {
    fn default() -> Self {
        Self {
            schema_version: SYMBOL_EDITOR_METADATA_SCHEMA_VERSION,
            attributes: Vec::new(),
            revision: 0,
            revisions: Vec::new(),
        }
    }
}

impl SymbolEditorMetadata {
    pub fn for_document(document: &SymbolDocument) -> Self {
        Self {
            attributes: vec![
                SymbolAttribute {
                    kind: SymbolAttributeKind::Reference,
                    default_value: "U?".to_owned(),
                    shown: true,
                    position: document.name_anchor,
                },
                SymbolAttribute {
                    kind: SymbolAttributeKind::Value,
                    default_value: "VALUE".to_owned(),
                    shown: true,
                    position: document.value_anchor,
                },
                SymbolAttribute {
                    kind: SymbolAttributeKind::Model,
                    default_value: String::new(),
                    shown: false,
                    position: Point::new(
                        document.value_anchor.x,
                        document
                            .value_anchor
                            .y
                            .saturating_add(SYMBOL_TERMINAL_GRID * 2),
                    ),
                },
            ],
            ..Self::default()
        }
    }

    pub fn normalize(&mut self, document: &SymbolDocument) {
        self.schema_version = SYMBOL_EDITOR_METADATA_SCHEMA_VERSION;
        for kind in SymbolAttributeKind::ALL {
            if self
                .attributes
                .iter()
                .all(|attribute| attribute.kind != kind)
            {
                let fallback = Self::for_document(document);
                if let Some(attribute) = fallback
                    .attributes
                    .into_iter()
                    .find(|attribute| attribute.kind == kind)
                {
                    self.attributes.push(attribute);
                }
            }
        }
        self.attributes.sort_by_key(|attribute| attribute.kind);
        self.attributes.dedup_by_key(|attribute| attribute.kind);
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != SYMBOL_EDITOR_METADATA_SCHEMA_VERSION {
            return Err(format!(
                "Unsupported symbol editor metadata schema {}",
                self.schema_version
            ));
        }
        if self.revisions.len() > MAX_SYMBOL_REVISION_RECORDS {
            return Err(format!(
                "Invalid symbol editor metadata: {} revision records exceed the limit of {MAX_SYMBOL_REVISION_RECORDS}",
                self.revisions.len()
            ));
        }
        let mut previous_revision = 0;
        for record in &self.revisions {
            for name in &record.pin_names {
                validate_symbol_text(name, "revision pin name")?;
            }
            if record.revision == 0 || record.revision <= previous_revision {
                return Err(
                    "Invalid symbol editor metadata: revision history is not strictly ordered"
                        .to_owned(),
                );
            }
            previous_revision = record.revision;
        }
        if previous_revision > self.revision {
            return Err(
                "Invalid symbol editor metadata: revision history exceeds the active revision"
                    .to_owned(),
            );
        }
        let mut kinds = HashSet::new();
        for attribute in &self.attributes {
            if !kinds.insert(attribute.kind) {
                return Err(format!(
                    "Invalid symbol editor metadata: duplicate '{}' attribute",
                    attribute.kind.key()
                ));
            }
            validate_symbol_text(&attribute.default_value, "attribute value")?;
            validate_symbol_point(attribute.position, "attribute position")?;
        }
        if kinds.len() != SymbolAttributeKind::ALL.len() {
            return Err("Invalid symbol editor metadata: the refdes, value and model attributes are required".to_owned());
        }
        Ok(())
    }

    pub fn load_from_view(view: &View, document: &SymbolDocument) -> Result<Self, String> {
        let Some(encoded) = view.metadata.get(SYMBOL_EDITOR_METADATA_KEY) else {
            return Ok(Self::for_document(document));
        };
        if encoded.len() > MAX_SYMBOL_DOCUMENT_BYTES {
            return Err(format!(
                "Invalid symbol editor metadata: document is {} bytes; the limit is {MAX_SYMBOL_DOCUMENT_BYTES}",
                encoded.len()
            ));
        }
        let mut metadata: Self = serde_json::from_str(encoded)
            .map_err(|error| format!("Invalid symbol editor metadata: {error}"))?;
        metadata.normalize(document);
        metadata.validate()?;
        Ok(metadata)
    }

    pub fn encode(&self) -> Result<String, String> {
        self.validate()?;
        let encoded = serde_json::to_string(self)
            .map_err(|error| format!("Could not serialize symbol editor metadata: {error}"))?;
        if encoded.len() > MAX_SYMBOL_DOCUMENT_BYTES {
            return Err(format!(
                "Could not serialize symbol editor metadata: document is {} bytes; the limit is {MAX_SYMBOL_DOCUMENT_BYTES}",
                encoded.len()
            ));
        }
        Ok(encoded)
    }

    pub fn attribute(&self, kind: SymbolAttributeKind) -> Option<&SymbolAttribute> {
        self.attributes
            .iter()
            .find(|attribute| attribute.kind == kind)
    }

    pub fn attribute_mut(&mut self, kind: SymbolAttributeKind) -> Option<&mut SymbolAttribute> {
        self.attributes
            .iter_mut()
            .find(|attribute| attribute.kind == kind)
    }

    /// Record that a project save at `saved_unix_ms` writes `document`, and
    /// answer the new revision number, or `None` when the latest revision is
    /// already this artwork: a save that changes nothing about the symbol
    /// records nothing.
    pub fn record_saved_revision(
        &mut self,
        document: &SymbolDocument,
        saved_unix_ms: u64,
    ) -> Option<u64> {
        let digest = document.content_digest();
        if self
            .revisions
            .last()
            .is_some_and(|latest| latest.document_digest == digest)
        {
            return None;
        }
        let revision = self.revision.saturating_add(1);
        self.revision = revision;
        if self.revisions.len() >= MAX_SYMBOL_REVISION_RECORDS {
            self.revisions.remove(0);
        }
        self.revisions.push(SymbolRevisionRecord {
            revision,
            saved_unix_ms,
            document_digest: digest,
            pin_names: document.pins.iter().map(|pin| pin.name.clone()).collect(),
        });
        Some(revision)
    }

    /// This metadata with the revision history `from` holds in place of its
    /// own. The history records project saves, which an undo step does not
    /// take back: restoring the attributes an earlier edit left must not
    /// rewind the revision counter past a revision a save already wrote.
    pub fn with_revision_history_of(mut self, from: &Self) -> Self {
        self.revision = from.revision;
        self.revisions.clone_from(&from.revisions);
        self
    }
}

fn validate_symbol_text(value: &str, label: &str) -> Result<(), String> {
    if value.len() > MAX_SYMBOL_TEXT_BYTES || value.chars().any(char::is_control) {
        return Err(format!(
            "Invalid symbol metadata: {label} is invalid or too long"
        ));
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolLabelAnchors {
    pub name: Point,
    pub value: Point,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolDocument {
    pub pins: Vec<SymbolPin>,
    pub body: Vec<SymbolShape>,
    pub origin: Point,
    pub name_anchor: Point,
    pub value_anchor: Point,
    /// The type size, in world units, of the pin names of a symbol that came
    /// from another tool's drawing, which letters them in the ordinary label
    /// face at the drawing's size. None for RSpice's own symbols.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pin_name_size: Option<u16>,
    /// The pin-contract findings this symbol's team has accepted as they
    /// stand, stored with the symbol so they travel with its library.
    ///
    /// Not written while empty: a build that predates waivers refuses an
    /// unknown field here, so a symbol nobody waived anything in must stay
    /// one it can open.
    #[serde(default, skip_serializing_if = "crate::drc::DrcWaivers::is_empty")]
    pub waivers: crate::drc::DrcWaivers,
    /// A placed part letters no name: the symbol's reference is not shown
    /// on its instances. Not written while it is.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub name_hidden: bool,
    /// A placed part letters no value: the symbol's value is not shown on
    /// its instances. Not written while it is.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub value_hidden: bool,
}

/// What a symbol's pins answer to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum SymbolInterface {
    /// Nothing else in the cell declares its pins, so the symbol's own pins
    /// are the cell's interface: a symbol drawn first is complete as it is.
    SymbolDefined,
    /// The cell's schematic, compiled module or bound model declares these
    /// pins, in netlist order. A schematic may declare none.
    Declared(Vec<PortSpec>),
}

impl SymbolInterface {
    /// The declared pins; none where the symbol defines the interface.
    pub fn ports(&self) -> &[PortSpec] {
        match self {
            Self::SymbolDefined => &[],
            Self::Declared(ports) => ports,
        }
    }
}

/// How a symbol's pins stand against its interface, counted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PinSummary {
    /// The interface is declared and holds no pins, while the symbol has
    /// some.
    pub interface_empty: bool,
    /// Declared pins the symbol has no pin for.
    pub missing: usize,
    /// Declared pins whose symbol pin has no terminal on the symbol.
    pub unplaced: usize,
    /// Symbol pins the interface does not declare.
    pub orphaned: usize,
}

impl PinSummary {
    /// Whether the pins and the interface agree.
    pub fn matches(&self) -> bool {
        *self == Self::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinFindingKind {
    UnplacedPin,
    OrphanedPin,
    PinOffGrid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinFinding {
    pub kind: PinFindingKind,
    pub pin_name: String,
}

impl Default for SymbolDocument {
    fn default() -> Self {
        Self {
            pins: Vec::new(),
            body: Vec::new(),
            origin: Point::origin(),
            name_anchor: Point::new(-20, -40),
            value_anchor: Point::new(-20, 40),
            pin_name_size: None,
            waivers: crate::drc::DrcWaivers::default(),
            name_hidden: false,
            value_hidden: false,
        }
    }
}

impl SymbolDocument {
    /// Shows or hides the attribute `kind` names on every placed part: the
    /// reference as the part's name, the value as its value. A placed part
    /// letters no model, so the model's flag changes nothing here.
    pub fn show_on_instances(&mut self, kind: SymbolAttributeKind, shown: bool) {
        match kind {
            SymbolAttributeKind::Reference => self.name_hidden = !shown,
            SymbolAttributeKind::Value => self.value_hidden = !shown,
            SymbolAttributeKind::Model => {}
        }
    }

    /// The type size of this symbol's pin names, in world units.
    #[must_use]
    pub fn pin_name_world(&self) -> f32 {
        self.pin_name_size.map_or(
            crate::symbol_generation::GENERATED_PIN_LABEL_SIZE,
            f32::from,
        )
    }

    pub fn generated_from_ports(ports: &[PortSpec]) -> Self {
        Self::generated_from_interface(ports, &[])
    }

    /// The generated symbol for an interface whose pin roles are known,
    /// parallel to `ports`: an amplifier pin set draws the triangle.
    pub fn generated_from_interface(ports: &[PortSpec], roles: &[PinRole]) -> Self {
        Self::from_generated(generate_symbol_with_roles(ports, roles))
    }

    /// The document of a generated symbol, pins on the edges it assigns and
    /// lettered as it says.
    pub fn from_generated(generated: GeneratedSymbol) -> Self {
        let body = generated.body_shapes();
        let name_anchor = generated.name_anchor();
        let value_anchor = generated.value_anchor();
        let body_box = generated.body_bounds();
        // The generator knows which edge every pin belongs to and how far its
        // terminal stands off the body; both are stored, so nothing
        // downstream re-derives either from the artwork. A clock's wedge is
        // its pin's style rather than artwork.
        let pins = generated
            .pins
            .into_iter()
            .zip(generated.labels)
            .zip(generated.marks)
            .map(|((pin, label), mark)| {
                let tip = pin.offset;
                let side = pin.side;
                let style = if mark == PinMark::Clock {
                    SymbolPinStyle::Clock
                } else {
                    SymbolPinStyle::Plain
                };
                let pin = SymbolPin::new(pin.name, pin.direction, Some(tip))
                    .with_contract(default_electrical_type(pin.direction), side)
                    .with_length(lead_length_to_body(tip, side, body_box))
                    .with_style(style);
                match label {
                    PinLabel::Inside => pin,
                    PinLabel::InsideFrom(reach) => pin.with_name_offset(Some(reach)),
                    PinLabel::Hidden => pin.with_name_hidden(true),
                    PinLabel::Beside { offset, reach } => SymbolPin {
                        name_at: Some(PinNamePlacement {
                            offset,
                            reach,
                            upward: false,
                        }),
                        ..pin
                    },
                }
            })
            .collect();
        Self {
            pins,
            body,
            origin: Point::origin(),
            name_anchor,
            value_anchor,
            pin_name_size: None,
            waivers: crate::drc::DrcWaivers::default(),
            name_hidden: false,
            value_hidden: false,
        }
    }

    pub fn load_from_view(view: &View) -> Result<Self, String> {
        // Counted unconditionally: a view carrying no document still builds the
        // default artwork and validates the result, which is the same
        // derivation as the parse it replaces.
        #[cfg(any(test, feature = "symbol-test-observation"))]
        SYMBOL_VIEW_PARSES.with(|count| count.set(count.get() + 1));

        let document = match view.metadata.get(SYMBOL_DOCUMENT_METADATA_KEY) {
            Some(raw) if raw.len() > MAX_SYMBOL_DOCUMENT_BYTES => {
                return Err(format!(
                    "Invalid symbol metadata: document is {} bytes; the limit is {MAX_SYMBOL_DOCUMENT_BYTES}",
                    raw.len()
                ));
            }
            Some(raw) => serde_json::from_str::<Self>(raw)
                .map_err(|err| format!("Invalid symbol metadata: {err}"))?,
            None => Self::default(),
        };
        document.validate()?;
        Ok(document)
    }

    /// Content digest of the whole document — pins, body, origin, anchors.
    ///
    /// Two documents share a digest exactly when they would serialize
    /// identically, so a revision history can say whether a published
    /// revision is the artwork on screen without storing a second copy of it.
    pub fn content_digest(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(b"rspice-symbol-document/v1");
        hasher.update(serde_json::to_vec(self).unwrap_or_default());
        ContentDigest::from_bytes(hasher.finalize().into()).to_string()
    }

    /// Every authored text run in the body, with the point it hangs off, in
    /// body order.
    #[cfg(any(test, feature = "test-support"))]
    pub fn text_runs(&self) -> impl Iterator<Item = (&str, Point)> {
        self.body.iter().filter_map(|shape| match shape {
            SymbolShape::Text { anchor, text, .. } => Some((text.as_str(), *anchor)),
            _ => None,
        })
    }

    fn encode(&self) -> Result<String, String> {
        self.validate()?;
        let raw = serde_json::to_string(self)
            .map_err(|err| format!("Could not serialize symbol metadata: {err}"))?;
        if raw.len() > MAX_SYMBOL_DOCUMENT_BYTES {
            return Err(format!(
                "Could not serialize symbol metadata: document is {} bytes; the limit is {MAX_SYMBOL_DOCUMENT_BYTES}",
                raw.len()
            ));
        }
        Ok(raw)
    }

    pub fn store_in_view(&self, view: &mut View) -> Result<(), String> {
        let raw = self.encode()?;
        view.metadata
            .insert(SYMBOL_DOCUMENT_METADATA_KEY.to_owned(), raw);
        view.modified = true;
        Ok(())
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.pins.len() > MAX_SYMBOL_PINS {
            return Err(format!(
                "Invalid symbol metadata: {} pins exceed the limit of {MAX_SYMBOL_PINS}",
                self.pins.len()
            ));
        }
        if self.body.len() > MAX_SYMBOL_SHAPES {
            return Err(format!(
                "Invalid symbol metadata: {} shapes exceed the limit of {MAX_SYMBOL_SHAPES}",
                self.body.len()
            ));
        }

        validate_symbol_point(self.origin, "origin")?;
        validate_symbol_point(self.name_anchor, "name label anchor")?;
        validate_symbol_point(self.value_anchor, "value label anchor")?;

        let mut pin_names = HashSet::with_capacity(self.pins.len());
        for (index, pin) in self.pins.iter().enumerate() {
            let name = pin.name.trim();
            if name.is_empty() {
                return Err(format!(
                    "Invalid symbol metadata: pin {} has an empty name",
                    index + 1
                ));
            }
            if pin.name.len() > MAX_SYMBOL_PIN_NAME_BYTES {
                return Err(format!(
                    "Invalid symbol metadata: pin '{}' has a {}-byte name; the limit is {MAX_SYMBOL_PIN_NAME_BYTES}",
                    name,
                    pin.name.len()
                ));
            }
            if name != pin.name {
                return Err(format!(
                    "Invalid symbol metadata: pin '{}' contains leading or trailing whitespace",
                    pin.name
                ));
            }
            if name.chars().any(char::is_control) {
                return Err(format!(
                    "Invalid symbol metadata: pin '{name}' contains a control character"
                ));
            }
            if !pin_names.insert(name.to_ascii_lowercase()) {
                return Err(format!(
                    "Invalid symbol metadata: duplicate pin name '{name}'"
                ));
            }
            if pin.length < 0 {
                return Err(format!(
                    "Invalid symbol metadata: pin '{name}' has a negative lead length {}",
                    pin.length
                ));
            }
            if let Some(position) = pin.position {
                validate_symbol_point(position, &format!("pin '{name}' terminal"))?;
            }
            if let Some(inner) = pin.lead_inner() {
                validate_symbol_point(inner, &format!("pin '{name}' lead"))?;
            }
        }

        let mut total_points = 0_usize;
        let mut total_texts = 0_usize;
        for (index, shape) in self.body.iter().enumerate() {
            let ordinal = index + 1;
            match shape {
                SymbolShape::Polyline { points, .. } => {
                    if points.len() < 2 {
                        return Err(format!(
                            "Invalid symbol metadata: polyline {ordinal} must contain at least two points"
                        ));
                    }
                    total_points = total_points.checked_add(points.len()).ok_or_else(|| {
                        "Invalid symbol metadata: symbol point count overflowed".to_owned()
                    })?;
                    if total_points > MAX_SYMBOL_POINTS {
                        return Err(format!(
                            "Invalid symbol metadata: {total_points} polyline points exceed the limit of {MAX_SYMBOL_POINTS}"
                        ));
                    }
                    for (point_index, point) in points.iter().copied().enumerate() {
                        validate_symbol_point(
                            point,
                            &format!("polyline {ordinal} point {}", point_index + 1),
                        )?;
                    }
                }
                SymbolShape::Rectangle { min, max, .. } => {
                    validate_symbol_point(*min, &format!("rectangle {ordinal} corner"))?;
                    validate_symbol_point(*max, &format!("rectangle {ordinal} corner"))?;
                    if min.x > max.x || min.y > max.y {
                        return Err(format!(
                            "Invalid symbol metadata: rectangle {ordinal} names its corners out of order"
                        ));
                    }
                }
                SymbolShape::Curve { curve, .. } => validate_symbol_curve(curve, ordinal)?,
                SymbolShape::Circle { center, radius, .. }
                | SymbolShape::Dot { center, radius } => {
                    validate_symbol_point(*center, &format!("round shape {ordinal} center"))?;
                    validate_symbol_radius(*center, *radius, ordinal)?;
                }
                SymbolShape::Arc {
                    center,
                    radius,
                    start_degrees,
                    sweep_degrees,
                    ..
                } => {
                    validate_symbol_point(*center, &format!("arc {ordinal} center"))?;
                    validate_symbol_radius(*center, *radius, ordinal)?;
                    if !(-360..=360).contains(sweep_degrees) || *sweep_degrees == 0 {
                        return Err(format!(
                            "Invalid symbol metadata: arc {ordinal} sweep must be between -360 and 360 degrees and nonzero"
                        ));
                    }
                    if start_degrees.unsigned_abs() > 360_000 {
                        return Err(format!(
                            "Invalid symbol metadata: arc {ordinal} start angle is outside the supported range"
                        ));
                    }
                }
                SymbolShape::Arrow { tip, .. } => {
                    validate_symbol_point(*tip, &format!("arrow {ordinal} tip"))?;
                }
                SymbolShape::Text { anchor, text, .. } => {
                    validate_symbol_point(*anchor, &format!("text {ordinal} anchor"))?;
                    validate_symbol_text(text, &format!("text {ordinal}"))?;
                    total_texts += 1;
                    if total_texts > MAX_SYMBOL_TEXT_OBJECTS {
                        return Err(format!(
                            "Invalid symbol metadata: {total_texts} text objects exceed the limit of {MAX_SYMBOL_TEXT_OBJECTS}"
                        ));
                    }
                }
                SymbolShape::Label {
                    anchor,
                    parameter,
                    default,
                    ..
                } => {
                    validate_symbol_point(*anchor, &format!("label {ordinal} anchor"))?;
                    validate_symbol_text(default, &format!("label {ordinal} default"))?;
                    // The name an instance's parameter text sets it by.
                    if parameter.is_empty()
                        || parameter.len() > MAX_SYMBOL_TEXT_BYTES
                        || parameter
                            .chars()
                            .any(|ch| ch.is_whitespace() || ch.is_control() || ch == '=')
                    {
                        return Err(format!(
                            "Invalid symbol metadata: label {ordinal} does not name a parameter"
                        ));
                    }
                    total_texts += 1;
                    if total_texts > MAX_SYMBOL_TEXT_OBJECTS {
                        return Err(format!(
                            "Invalid symbol metadata: {total_texts} text objects exceed the limit of {MAX_SYMBOL_TEXT_OBJECTS}"
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn pin(&self, name: &str) -> Option<&SymbolPin> {
        self.pins
            .iter()
            .find(|pin| pin.name.eq_ignore_ascii_case(name))
    }

    pub fn pin_mut(&mut self, name: &str) -> Option<&mut SymbolPin> {
        self.pins
            .iter_mut()
            .find(|pin| pin.name.eq_ignore_ascii_case(name))
    }

    /// Bring the pin list back into agreement with the interface `ports`.
    ///
    /// Body artwork is never read or written here: reconciliation is a
    /// contract operation, and regenerating a body would discard drawing the
    /// author owns.
    ///
    /// - A declared port with no pin gains one, placed against the body on
    ///   the side generation gives it (inputs left, outputs right, positive
    ///   rails on top, ground and negative rails below) and clear of the
    ///   terminals already there.
    /// - A pin that already exists keeps its authored geometry and takes the
    ///   port's direction. It keeps its electrical type while its direction
    ///   stands; a changed direction re-derives the type from the new one.
    ///   Its side is re-derived only while it is unplaced, because moving a
    ///   drawn terminal is an artwork change.
    /// - A pin no port declares is **kept**. It is the orphan the symbol
    ///   check reports; deleting it here would silently disconnect every
    ///   instance already wired to it.
    /// - Order is rewritten only when the two name sets agree exactly.
    ///   Sorting a mismatched list would interleave orphans into netlist
    ///   order, which is the one thing the order means.
    #[cfg(any(test, feature = "test-support"))]
    pub fn reconcile_ports(&mut self, ports: &[PortSpec]) {
        self.reconcile_typed_ports(ports, |_| None);
    }

    /// [`Self::reconcile_ports`], with the electrical type `electrical` states
    /// for a port winning over both the pin's own and its direction's: a
    /// discrete Verilog-A port is `Logic` whichever way it points.
    pub fn reconcile_typed_ports(
        &mut self,
        ports: &[PortSpec],
        electrical: impl Fn(&PortSpec) -> Option<SymbolElectricalType>,
    ) {
        self.reconcile_contract_ports(ports, electrical, |_| None);
    }

    /// [`Self::reconcile_typed_ports`], with the side `stated_side` gives a
    /// port's new pin winning over the side its direction implies: a model's
    /// contract says where each of its terminals is drawn.
    pub fn reconcile_contract_ports(
        &mut self,
        ports: &[PortSpec],
        electrical: impl Fn(&PortSpec) -> Option<SymbolElectricalType>,
        stated_side: impl Fn(&PortSpec) -> Option<SymbolPinSide>,
    ) {
        let bounds = self.body_bounds();
        for port in ports {
            let stated = electrical(port);
            let known = match self.pin_mut(&port.name) {
                Some(pin) => {
                    let agrees = pin.states_direction(port.direction);
                    let kept = pin.electrical_type.filter(|_| agrees);
                    let electrical = stated
                        .or(kept)
                        .unwrap_or_else(|| default_electrical_type(port.direction));
                    let direction = if agrees && stated.is_none() {
                        pin.direction
                    } else {
                        port.direction
                    };
                    pin.name = port.name.clone();
                    pin.set_electrical_contract(electrical, direction);
                    if pin.position.is_some() {
                        continue;
                    }
                    true
                }
                None => false,
            };
            // The side the model states, else the edge generation gives the
            // port: a ground or negative rail goes to the bottom, as on a
            // symbol made from the schematic.
            let side = stated_side(port).unwrap_or_else(|| {
                crate::pin_roles::interface_pin(&port.name, port.direction, false).inferred_side()
            });
            let (tip, length) = self.free_place(side, bounds);
            if known {
                if let Some(pin) = self.pin_mut(&port.name) {
                    pin.side = side;
                    pin.position = Some(tip);
                    pin.length = length;
                }
                continue;
            }
            let electrical = stated.unwrap_or_else(|| default_electrical_type(port.direction));
            let pin = SymbolPin::new(&port.name, port.direction, Some(tip))
                .with_contract(electrical, side)
                .with_length(length);
            self.pins.push(pin);
        }

        let declared = port_name_set(ports);
        let placed: HashSet<String> = self
            .pins
            .iter()
            .map(|pin| pin.name.to_ascii_lowercase())
            .collect();
        if declared != placed {
            return;
        }
        self.pins.sort_by_key(|pin| {
            ports
                .iter()
                .position(|port| port.name.eq_ignore_ascii_case(&pin.name))
                .unwrap_or(ports.len())
        });
    }

    /// Place the unplaced pin `name` on `side`, where [`Self::free_place`]
    /// puts a new pin there. A placed pin is left where it is.
    pub fn place_pin_on_side(&mut self, name: &str, side: SymbolPinSide) {
        let (tip, length) = self.free_place(side, self.body_bounds());
        if let Some(pin) = self.pin_mut(name).filter(|pin| pin.position.is_none()) {
            pin.side = side;
            pin.position = Some(tip);
            pin.length = length;
        }
    }

    /// Where a pin newly placed on `side` stands, and the lead that carries it
    /// to `body`: on the next free terminal-grid place along that side, in
    /// line with the terminals already there, or the standard lead off the
    /// body when there are none.
    fn free_place(&self, side: SymbolPinSide, body: (Point, Point)) -> (Point, i32) {
        let along = self.next_free_along(side);
        let mut tip = tip_off_body(side, along, SYMBOL_PIN_LENGTH, body);
        if let Some(column) = self.terminal_column(side) {
            tip = match side {
                SymbolPinSide::Left | SymbolPinSide::Right => Point::new(column, tip.y),
                SymbolPinSide::Top | SymbolPinSide::Bottom => Point::new(tip.x, column),
            };
        }
        (tip, lead_length_to_body(tip, side, body))
    }

    /// The line the outermost terminals on `side` stand on, when any do.
    fn terminal_column(&self, side: SymbolPinSide) -> Option<i32> {
        let reach = self
            .pins
            .iter()
            .filter(|pin| pin.side == side)
            .filter_map(|pin| pin.position)
            .map(|position| match side {
                SymbolPinSide::Left | SymbolPinSide::Right => position.x,
                SymbolPinSide::Top | SymbolPinSide::Bottom => position.y,
            });
        match side {
            SymbolPinSide::Left | SymbolPinSide::Top => reach.min(),
            SymbolPinSide::Right | SymbolPinSide::Bottom => reach.max(),
        }
    }

    /// The next terminal-grid place along `side` that no placed pin occupies.
    fn next_free_along(&self, side: SymbolPinSide) -> i32 {
        let used: HashSet<i32> = self
            .pins
            .iter()
            .filter(|pin| pin.side == side)
            .filter_map(|pin| pin.position)
            .map(|tip| along_side(side, tip))
            .collect();
        (0..=MAX_SYMBOL_PINS as i32)
            .map(|step| step * SYMBOL_TERMINAL_GRID)
            .find(|along| !used.contains(along))
            .unwrap_or(0)
    }

    /// How the pins stand against `interface`, counted.
    pub fn pin_summary(&self, interface: &SymbolInterface) -> PinSummary {
        let SymbolInterface::Declared(ports) = interface else {
            return PinSummary::default();
        };
        if ports.is_empty() {
            return PinSummary {
                interface_empty: !self.pins.is_empty(),
                ..PinSummary::default()
            };
        }
        let port_names = port_name_set(ports);
        let mut summary = PinSummary::default();
        for port in ports {
            match self.pin(&port.name) {
                None => summary.missing += 1,
                Some(pin) if pin.position.is_none() => summary.unplaced += 1,
                Some(_) => {}
            }
        }
        summary.orphaned = self
            .pins
            .iter()
            .filter(|pin| !port_names.contains(&pin.name.to_ascii_lowercase()))
            .count();
        summary
    }

    /// Every pin finding against `interface`. A symbol that defines the
    /// interface itself is checked only for terminals off the grid; one whose
    /// interface declares no pins has every pin it carries reported as not in
    /// that interface.
    pub fn pin_findings(&self, interface: &SymbolInterface) -> Vec<PinFinding> {
        let ports = interface.ports();
        let declared = matches!(interface, SymbolInterface::Declared(_));
        let port_names = port_name_set(ports);
        let mut findings = Vec::new();
        for port in ports {
            if self
                .pin(&port.name)
                .is_none_or(|pin| pin.position.is_none())
            {
                findings.push(PinFinding {
                    kind: PinFindingKind::UnplacedPin,
                    pin_name: port.name.clone(),
                });
            }
        }
        for pin in &self.pins {
            if declared && !port_names.contains(&pin.name.to_ascii_lowercase()) {
                findings.push(PinFinding {
                    kind: PinFindingKind::OrphanedPin,
                    pin_name: pin.name.clone(),
                });
            }
            if pin.position.is_some() && !pin.terminal_on_grid() {
                findings.push(PinFinding {
                    kind: PinFindingKind::PinOffGrid,
                    pin_name: pin.name.clone(),
                });
            }
        }
        findings
    }

    pub fn labels(&self) -> SymbolLabelAnchors {
        SymbolLabelAnchors {
            name: self.name_anchor,
            value: self.value_anchor,
        }
    }

    /// Body bounds when there is drawn artwork to judge a pin against.
    ///
    /// [`Self::body_bounds`] answers with a nominal box for an empty body so
    /// that pin placement always has somewhere to anchor; callers that are
    /// reasoning about real geometry need to know the difference.
    pub fn drawn_body_bounds(&self) -> Option<(Point, Point)> {
        (!self.body.is_empty()).then(|| self.body_bounds())
    }

    /// The side suggested for a pin first placed with its tip at `point`:
    /// judged against the drawn shape nearest the point, so a stray line
    /// elsewhere in the artwork cannot turn a pin placed beside the body.
    pub fn suggested_pin_side(&self, point: Point) -> SymbolPinSide {
        let nearest = self
            .body
            .iter()
            .filter_map(|shape| shapes_bounds([shape]))
            .min_by_key(|bounds| box_distance(point, *bounds));
        pin_side_against_body(
            point,
            PortDirection::InOut,
            Some(nearest.unwrap_or_else(|| self.body_bounds())),
        )
    }

    /// Settle the pins of a symbol another tool drew: each takes the side
    /// its drawing puts it on and no lead of its own, because the drawing's
    /// lead, where it has one, is artwork.
    ///
    /// The side a pin was given stands wherever the artwork agrees it could
    /// be that edge; a terminal that stands off a different edge entirely
    /// takes the edge it stands off. This reads the artwork once, as the
    /// symbol arrives, and never again.
    pub fn settle_drawn_pins(&mut self) {
        let bounds = self.drawn_body_bounds();
        for pin in &mut self.pins {
            pin.length = 0;
            let (Some(position), Some(bounds)) = (pin.position, bounds) else {
                continue;
            };
            if edge_standoff(position, bounds, pin.side) < 0 {
                pin.side = pin_side_against_body(position, pin.direction, Some(bounds));
            }
        }
    }

    /// Body-only bounds, which suggest the side and lead of a pin as it is
    /// first placed.
    ///
    /// Pins, attributes and the editing origin are intentionally excluded:
    /// placing one terminal must never expand the body the next one is
    /// placed against.
    pub fn body_bounds(&self) -> (Point, Point) {
        shapes_bounds(&self.body).unwrap_or((
            Point::new(-SYMBOL_TERMINAL_GRID * 4, -SYMBOL_TERMINAL_GRID * 4),
            Point::new(SYMBOL_TERMINAL_GRID * 4, SYMBOL_TERMINAL_GRID * 4),
        ))
    }
}

/// The box around `shapes`; `None` when they draw nothing.
fn shapes_bounds<'a>(shapes: impl IntoIterator<Item = &'a SymbolShape>) -> Option<(Point, Point)> {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for shape in shapes {
        match shape {
            SymbolShape::Polyline { points, .. } => {
                for point in points {
                    xs.push(point.x);
                    ys.push(point.y);
                }
            }
            SymbolShape::Rectangle { min, max, .. } => {
                xs.extend([min.x, max.x]);
                ys.extend([min.y, max.y]);
            }
            SymbolShape::Circle { center, radius, .. } | SymbolShape::Dot { center, radius } => {
                xs.extend([
                    center.x.saturating_sub(*radius),
                    center.x.saturating_add(*radius),
                ]);
                ys.extend([
                    center.y.saturating_sub(*radius),
                    center.y.saturating_add(*radius),
                ]);
            }
            SymbolShape::Arc { center, radius, .. } => {
                xs.extend([
                    center.x.saturating_sub(*radius),
                    center.x.saturating_add(*radius),
                ]);
                ys.extend([
                    center.y.saturating_sub(*radius),
                    center.y.saturating_add(*radius),
                ]);
            }
            SymbolShape::Curve { curve, .. } => {
                let (least, greatest) = curve.whole_unit_bounds();
                xs.extend([least.x, greatest.x]);
                ys.extend([least.y, greatest.y]);
            }
            SymbolShape::Arrow { tip, .. } => {
                xs.extend([
                    tip.x.saturating_sub(SYMBOL_TERMINAL_GRID),
                    tip.x.saturating_add(SYMBOL_TERMINAL_GRID),
                ]);
                ys.extend([
                    tip.y.saturating_sub(SYMBOL_TERMINAL_GRID),
                    tip.y.saturating_add(SYMBOL_TERMINAL_GRID),
                ]);
            }
            SymbolShape::Text {
                anchor,
                text,
                size,
                align,
                rotation,
            } => {
                let (min, max) = symbol_text_bounds(*anchor, text, *size, *align, *rotation);
                xs.extend([min.x, max.x]);
                ys.extend([min.y, max.y]);
            }
            // What an instance letters, measured by the editor's sample;
            // a hidden label is drawn on no instance, so it takes no room.
            SymbolShape::Label {
                anchor,
                parameter,
                default,
                size,
                align,
                rotation,
                shown,
            } => {
                if *shown {
                    let sample = symbol_label_sample(parameter, default);
                    let (min, max) = symbol_text_bounds(*anchor, &sample, *size, *align, *rotation);
                    xs.extend([min.x, max.x]);
                    ys.extend([min.y, max.y]);
                }
            }
        }
    }
    Some((
        Point::new(xs.iter().min().copied()?, ys.iter().min().copied()?),
        Point::new(xs.iter().max().copied()?, ys.iter().max().copied()?),
    ))
}

/// How far `point` lies outside `bounds`, in whole units along each axis
/// summed; zero inside.
fn box_distance(point: Point, (min, max): (Point, Point)) -> i64 {
    let outside = |value: i32, low: i32, high: i32| {
        i64::from(low.saturating_sub(value).max(0)) + i64::from(value.saturating_sub(high).max(0))
    };
    outside(point.x, min.x, max.x) + outside(point.y, min.y, max.y)
}

fn validate_symbol_point(point: Point, label: &str) -> Result<(), String> {
    if point.x.unsigned_abs() > MAX_SYMBOL_COORDINATE_ABS as u32
        || point.y.unsigned_abs() > MAX_SYMBOL_COORDINATE_ABS as u32
    {
        return Err(format!(
            "Invalid symbol metadata: {label} ({}, {}) exceeds the supported coordinate range",
            point.x, point.y
        ));
    }
    Ok(())
}

fn validate_symbol_curve(curve: &SymbolCurve, ordinal: usize) -> Result<(), String> {
    let numbers = [
        curve.center[0],
        curve.center[1],
        curve.radii[0],
        curve.radii[1],
        curve.start_degrees,
        curve.sweep_degrees,
    ];
    if !numbers.iter().all(|number| number.is_finite())
        || curve.radii[0] <= 0.0
        || curve.radii[1] <= 0.0
        || curve.sweep_degrees <= 0.0
        || curve.sweep_degrees > 360.0
    {
        return Err(format!(
            "Invalid symbol metadata: curve {ordinal} needs finite positive radii and a sweep between 0 and 360 degrees"
        ));
    }
    let limit = f64::from(MAX_SYMBOL_COORDINATE_ABS);
    let (least, greatest) = curve.bounds();
    if least
        .into_iter()
        .chain(greatest)
        .any(|coordinate| coordinate.abs() > limit)
    {
        return Err(format!(
            "Invalid symbol metadata: curve {ordinal} bounds exceed the coordinate representation"
        ));
    }
    Ok(())
}

fn validate_symbol_radius(center: Point, radius: i32, ordinal: usize) -> Result<(), String> {
    if !(1..=MAX_SYMBOL_RADIUS).contains(&radius) {
        return Err(format!(
            "Invalid symbol metadata: round shape {ordinal} radius must be between 1 and {MAX_SYMBOL_RADIUS}"
        ));
    }
    let limit = i64::from(MAX_SYMBOL_COORDINATE_ABS);
    let radius = i64::from(radius);
    let x = i64::from(center.x);
    let y = i64::from(center.y);
    if [x - radius, x + radius, y - radius, y + radius]
        .into_iter()
        .any(|coordinate| coordinate.unsigned_abs() > limit as u64)
    {
        return Err(format!(
            "Invalid symbol metadata: round shape {ordinal} bounds exceed the coordinate representation"
        ));
    }
    Ok(())
}

fn port_name_set(ports: &[PortSpec]) -> HashSet<String> {
    ports
        .iter()
        .map(|port| port.name.to_ascii_lowercase())
        .collect()
}

#[cfg(test)]
mod validation_tests {
    use super::*;
    use crate::library::ViewType;

    #[test]
    fn a_pins_name_declares_how_many_conductors_it_carries() {
        let vector = SymbolPin::new("DATA[3:0]", PortDirection::InOut, Some(Point::new(-40, 0)));
        assert_eq!(vector.vector().map(|range| range.width()), Some(4));
        assert_eq!(vector.width(), 4);

        let scalar = SymbolPin::new("EN", PortDirection::In, Some(Point::new(-40, 10)));
        assert!(scalar.vector().is_none());
        assert_eq!(scalar.width(), 1);
    }

    #[test]
    fn a_stored_pin_states_its_side_and_lead_and_reads_its_width_from_its_name() {
        let stored = serde_json::json!({
            "name": "DATA[3:0]",
            "direction": "InOut",
            "position": { "x": -40, "y": 0 },
            "side": "left",
            "length": 0
        });
        let restored: SymbolPin = serde_json::from_value(stored).expect("a stored pin loads");
        assert!(restored.electrical_type.is_none());
        assert_eq!(restored.side, SymbolPinSide::Left);
        assert_eq!(restored.length, 0);
        assert_eq!(restored.style, SymbolPinStyle::Plain);
        assert_eq!(restored.width(), 4);

        // Nothing about a pin is worked out from the drawing: a pin that does
        // not say which side it is on is not a pin.
        let sideless = serde_json::json!({
            "name": "EN",
            "direction": "In",
            "position": { "x": -40, "y": 0 },
            "length": 20
        });
        assert!(serde_json::from_value::<SymbolPin>(sideless).is_err());
    }

    #[test]
    fn loading_rejects_arc_sweeps_that_would_expand_unbounded_tessellation() {
        let mut view = View::new("symbol", ViewType::Symbol);
        let document = SymbolDocument {
            body: vec![SymbolShape::Arc {
                center: Point::origin(),
                radius: 10,
                start_degrees: 0,
                sweep_degrees: i32::MAX,
                style: Default::default(),
            }],
            ..SymbolDocument::default()
        };
        view.metadata.insert(
            SYMBOL_DOCUMENT_METADATA_KEY.to_owned(),
            serde_json::to_string(&document).expect("test document serializes"),
        );

        let error = SymbolDocument::load_from_view(&view)
            .expect_err("an unbounded imported arc sweep must fail closed");

        assert!(error.contains("sweep must be between -360 and 360"));
    }

    #[test]
    fn storing_invalid_geometry_is_atomic() {
        let mut view = View::new("symbol", ViewType::Symbol);
        view.metadata.insert(
            SYMBOL_DOCUMENT_METADATA_KEY.to_owned(),
            "retained".to_owned(),
        );
        let document = SymbolDocument {
            body: vec![SymbolShape::Polyline {
                points: vec![Point::origin()],
                closed: false,
                style: Default::default(),
            }],
            ..SymbolDocument::default()
        };

        let error = document
            .store_in_view(&mut view)
            .expect_err("an incomplete stored polyline must fail validation");

        assert!(error.contains("at least two points"));
        assert_eq!(
            view.metadata
                .get(SYMBOL_DOCUMENT_METADATA_KEY)
                .map(String::as_str),
            Some("retained")
        );
        assert!(!view.modified);
    }

    #[test]
    fn a_rectangle_loads_only_with_its_corners_in_order() {
        let document = |min: Point, max: Point| SymbolDocument {
            body: vec![SymbolShape::Rectangle {
                min,
                max,
                style: Default::default(),
            }],
            ..SymbolDocument::default()
        };

        assert!(
            document(Point::new(-20, -10), Point::new(20, 10))
                .validate()
                .is_ok()
        );
        let error = document(Point::new(20, -10), Point::new(-20, 10))
            .validate()
            .expect_err("a rectangle's corners are stored least first");
        assert!(error.contains("rectangle 1"), "{error}");
        assert_eq!(
            document(Point::new(-20, -10), Point::new(20, 10)).body_bounds(),
            (Point::new(-20, -10), Point::new(20, 10))
        );
    }

    #[test]
    fn pin_identity_is_unique_case_insensitively() {
        let document = SymbolDocument {
            pins: vec![
                SymbolPin::new("OUT", PortDirection::Out, Some(Point::origin())),
                SymbolPin::new("out", PortDirection::Out, Some(Point::new(10, 0))),
            ],
            ..SymbolDocument::default()
        };

        let error = document
            .validate()
            .expect_err("ambiguous symbol pin identity must be rejected");

        assert!(error.contains("duplicate pin name 'out'"));
    }

    #[test]
    fn round_shape_bounds_must_fit_the_coordinate_representation() {
        let document = SymbolDocument {
            body: vec![SymbolShape::Circle {
                center: Point::new(MAX_SYMBOL_COORDINATE_ABS, 0),
                radius: MAX_SYMBOL_RADIUS,
                style: Default::default(),
            }],
            ..SymbolDocument::default()
        };

        let error = document
            .validate()
            .expect_err("overflowing round-shape bounds must be rejected");

        assert!(error.contains("bounds exceed the coordinate representation"));
    }

    /// The lead runs from the tip toward the body by its own length on every
    /// side, whatever artwork the body holds: a stray line far from the body
    /// moves no lead.
    #[test]
    fn a_pins_lead_reaches_from_its_tip_by_its_stored_length() {
        let mut document = SymbolDocument {
            body: vec![SymbolShape::Polyline {
                points: vec![
                    Point::new(-20, -20),
                    Point::new(20, -20),
                    Point::new(20, 20),
                    Point::new(-20, 20),
                ],
                closed: true,
                style: Default::default(),
            }],
            pins: vec![
                SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, 0)))
                    .with_side(SymbolPinSide::Left),
                SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(40, 10)))
                    .with_side(SymbolPinSide::Right),
                SymbolPin::new("VDD", PortDirection::Supply, Some(Point::new(0, -40)))
                    .with_side(SymbolPinSide::Top),
                SymbolPin::new("VSS", PortDirection::Supply, Some(Point::new(0, 40)))
                    .with_side(SymbolPinSide::Bottom)
                    .with_length(0),
            ],
            ..SymbolDocument::default()
        };
        let leads = |document: &SymbolDocument| {
            document
                .pins
                .iter()
                .map(|pin| pin.lead_inner())
                .collect::<Vec<_>>()
        };
        let expected = [
            Some(Point::new(-20, 0)),
            Some(Point::new(20, 10)),
            Some(Point::new(0, -20)),
            Some(Point::new(0, 40)),
        ];
        assert_eq!(leads(&document), expected);

        document.body.push(SymbolShape::Polyline {
            points: vec![Point::new(-200, 150), Point::new(-190, 150)],
            closed: false,
            style: Default::default(),
        });
        assert_eq!(leads(&document), expected);
        document.validate().expect("the document stays valid");
    }

    #[test]
    fn a_negative_lead_length_is_refused() {
        let document = SymbolDocument {
            pins: vec![
                SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, 0))).with_length(-10),
            ],
            ..SymbolDocument::default()
        };
        let error = document
            .validate()
            .expect_err("a lead cannot run away from the body");
        assert!(error.contains("negative lead length"), "{error}");
    }

    /// Each style marks the body end of the lead: a bubble the lead stops
    /// at, a wedge inside the body, or both. A bare connection point draws
    /// nothing, whatever its style.
    #[test]
    fn each_pin_style_marks_the_body_end_of_its_lead() {
        let tip = Point::new(-40, 0);
        let drawing = |length, style| pin_drawing(tip, SymbolPinSide::Left, length, style);
        let wedge = [Point::new(-20, 3), Point::new(-16, 0), Point::new(-20, -3)];

        let plain = drawing(20, SymbolPinStyle::Plain);
        assert_eq!(plain.lead, Some((tip, Point::new(-20, 0))));
        assert_eq!((plain.bubble, plain.wedge), (None, None));

        let inverted = drawing(20, SymbolPinStyle::Inverted);
        assert_eq!(inverted.lead, Some((tip, Point::new(-26, 0))));
        assert_eq!(inverted.bubble, Some((Point::new(-23, 0), 3)));
        assert_eq!(inverted.wedge, None);

        let clock = drawing(20, SymbolPinStyle::Clock);
        assert_eq!(clock.lead, plain.lead);
        assert_eq!((clock.bubble, clock.wedge), (None, Some(wedge)));

        let both = drawing(20, SymbolPinStyle::InvertedClock);
        assert_eq!(both.lead, inverted.lead);
        assert_eq!((both.bubble, both.wedge), (inverted.bubble, Some(wedge)));

        for style in SymbolPinStyle::ALL {
            let bare = drawing(0, style);
            assert_eq!((bare.lead, bare.bubble, bare.wedge), (None, None, None));
        }
    }

    /// A generated clock input stores its wedge as its pin's style, so the
    /// document's artwork no longer carries it and the pin draws it exactly
    /// where the generator drew it.
    #[test]
    fn a_generated_clock_pin_draws_its_wedge_as_its_style() {
        let ports = [
            ("in", PortDirection::In),
            ("clk", PortDirection::In),
            ("en", PortDirection::In),
            ("out", PortDirection::Out),
            ("vdd", PortDirection::InOut),
            ("vss", PortDirection::InOut),
        ]
        .map(|(name, direction)| PortSpec {
            name: name.to_owned(),
            direction,
        });
        let generated = crate::symbol_generation::generate_symbol(&ports);
        let wedge = vec![Point::new(-30, 3), Point::new(-26, 0), Point::new(-30, -3)];
        assert!(generated.body_outline().contains(&(wedge.clone(), false)));

        let document = SymbolDocument::from_generated(generated);
        assert!(
            !document.body.iter().any(
                |shape| matches!(shape, SymbolShape::Polyline { points, .. } if *points == wedge)
            ),
            "the wedge is the pin's, not artwork"
        );
        let clk = document.pin("clk").expect("the clock pin");
        assert_eq!(clk.style, SymbolPinStyle::Clock);
        let tip = clk.position.expect("a generated pin is placed");
        let drawn = pin_drawing(tip, clk.side, clk.length, clk.style);
        assert_eq!(drawn.wedge.map(Vec::from), Some(wedge));
        assert!(
            document
                .pins
                .iter()
                .filter(|pin| pin.name != "clk")
                .all(|pin| pin.style.is_plain())
        );
    }

    /// A port the symbol has no pin for is placed in line with the pins
    /// already on its side, its lead reaching the body.
    #[test]
    fn reconciling_places_a_new_pin_in_line_with_its_side() {
        let port = |name: &str, direction| PortSpec {
            name: name.to_owned(),
            direction,
        };
        let mut document = SymbolDocument::generated_from_ports(&[
            port("a", PortDirection::In),
            port("y", PortDirection::Out),
        ]);
        let column = document.pin("a").and_then(|pin| pin.position).expect("a").x;
        document.reconcile_ports(&[
            port("a", PortDirection::In),
            port("y", PortDirection::Out),
            port("b", PortDirection::In),
        ]);
        let added = document.pin("b").expect("b is added");
        let tip = added.position.expect("b is placed");
        assert_eq!(added.side, SymbolPinSide::Left);
        assert_eq!(tip.x, column);
        let (min, _) = document.body_bounds();
        assert_eq!(added.lead_inner().map(|inner| inner.x), Some(min.x));
    }

    /// A rail added to the schematic joins the symbol on the edge a symbol
    /// made from that schematic gives it: a positive supply on top, ground
    /// and a negative supply below, whichever way the port is declared.
    #[test]
    fn reconciling_puts_a_new_rail_on_the_edge_generation_gives_it() {
        let port = |name: &str, direction| PortSpec {
            name: name.to_owned(),
            direction,
        };
        let mut ports = vec![port("a", PortDirection::In), port("y", PortDirection::Out)];
        let mut document = SymbolDocument::generated_from_ports(&ports);
        ports.extend([
            port("vdd", PortDirection::Supply),
            port("vss", PortDirection::Supply),
            port("gnd", PortDirection::InOut),
            port("vcc", PortDirection::InOut),
        ]);
        document.reconcile_ports(&ports);
        let generated = SymbolDocument::generated_from_ports(&ports);
        for name in ["vdd", "vss", "gnd", "vcc"] {
            let side = |document: &SymbolDocument| document.pin(name).map(|pin| pin.side);
            assert_eq!(side(&document), side(&generated), "{name}");
        }
        assert_eq!(
            document.pin("vss").map(|pin| pin.side),
            Some(SymbolPinSide::Bottom)
        );
    }

    #[test]
    fn editor_metadata_round_trips_attributes_and_revision_history() {
        let document = SymbolDocument {
            body: vec![SymbolShape::Text {
                anchor: Point::new(10, -20),
                text: "gain stage".to_owned(),
                size: SymbolTextSize::Normal,
                align: SymbolTextAlign::Left,
                rotation: Default::default(),
            }],
            ..SymbolDocument::default()
        };
        let mut metadata = SymbolEditorMetadata::for_document(&document);
        let revision = metadata
            .record_saved_revision(&document, 1_763_000_000_000)
            .expect("changed artwork records a revision");
        let mut view = View::new("symbol", ViewType::Symbol);
        view.metadata.insert(
            SYMBOL_EDITOR_METADATA_KEY.to_owned(),
            metadata.encode().expect("editor metadata encodes"),
        );

        let restored =
            SymbolEditorMetadata::load_from_view(&view, &document).expect("sidecar round trips");

        assert_eq!(revision, 1);
        assert_eq!(restored.revision, 1);
        assert_eq!(restored.revisions.len(), 1);
        assert_eq!(
            restored.revisions[0].document_digest,
            document.content_digest(),
            "a revision identifies the artwork it published, not how many \
             objects it happened to contain"
        );
        assert!(restored.revisions[0].pin_names.is_empty());
        assert_eq!(restored.revisions[0].saved_unix_ms, 1_763_000_000_000);
        assert_eq!(
            restored
                .attribute(SymbolAttributeKind::Reference)
                .expect("reference attribute retained")
                .default_value,
            "U?"
        );
    }

    #[test]
    fn a_save_records_a_revision_only_when_the_artwork_changed() {
        let mut document = SymbolDocument::default();
        let mut metadata = SymbolEditorMetadata::for_document(&document);

        assert_eq!(metadata.record_saved_revision(&document, 10), Some(1));
        assert_eq!(
            metadata.record_saved_revision(&document, 20),
            None,
            "saving the same artwork again is not a new revision"
        );
        document.pins.push(SymbolPin::new(
            "A",
            PortDirection::In,
            Some(Point::new(-20, 0)),
        ));
        assert_eq!(metadata.record_saved_revision(&document, 30), Some(2));

        assert_eq!(metadata.revision, 2);
        let saved: Vec<(u64, u64)> = metadata
            .revisions
            .iter()
            .map(|record| (record.revision, record.saved_unix_ms))
            .collect();
        assert_eq!(saved, [(1, 10), (2, 30)]);
        assert_eq!(metadata.revisions[1].pin_names, ["A"]);
        metadata.validate().expect("the history stays valid");
    }

    #[test]
    fn a_full_revision_history_lets_the_oldest_record_go() {
        let mut document = SymbolDocument::default();
        let mut metadata = SymbolEditorMetadata::for_document(&document);
        for step in 0..=MAX_SYMBOL_REVISION_RECORDS {
            document.origin = Point::new(i32::try_from(step).expect("small step"), 0);
            assert!(
                metadata.record_saved_revision(&document, 1).is_some(),
                "a save is never refused for the length of the history"
            );
        }

        assert_eq!(metadata.revisions.len(), MAX_SYMBOL_REVISION_RECORDS);
        assert_eq!(metadata.revisions[0].revision, 2);
        assert_eq!(
            metadata.revision,
            u64::try_from(MAX_SYMBOL_REVISION_RECORDS + 1).expect("fits")
        );
        metadata
            .validate()
            .expect("the trimmed history stays valid");
    }

    #[test]
    fn body_bounds_cover_the_whole_text_run() {
        let document = SymbolDocument {
            body: vec![SymbolShape::Text {
                anchor: Point::origin(),
                text: "AMP".to_owned(),
                size: SymbolTextSize::Normal,
                align: SymbolTextAlign::Left,
                rotation: Default::default(),
            }],
            ..SymbolDocument::default()
        };

        assert_eq!(
            document.body_bounds(),
            (Point::new(0, -4), Point::new(15, 4))
        );
    }

    #[test]
    fn text_shapes_with_control_characters_are_rejected() {
        let document = SymbolDocument {
            body: vec![SymbolShape::Text {
                anchor: Point::origin(),
                text: "A\u{7}MP".to_owned(),
                size: SymbolTextSize::Normal,
                align: SymbolTextAlign::Left,
                rotation: Default::default(),
            }],
            ..SymbolDocument::default()
        };

        let error = document
            .validate()
            .expect_err("a control character in body text must be rejected");

        assert!(error.contains("text 1 is invalid or too long"), "{error}");
    }

    /// A label must name one parameter an instance can set, and takes room
    /// in the symbol's extents only while instances letter it.
    #[test]
    fn a_label_names_a_parameter_and_takes_room_only_when_shown() {
        let label = |parameter: &str, shown| SymbolDocument {
            body: vec![SymbolShape::Label {
                anchor: Point::origin(),
                parameter: parameter.to_owned(),
                default: "1u".to_owned(),
                size: SymbolTextSize::Normal,
                align: SymbolTextAlign::Left,
                rotation: SymbolTextRotation::R0,
                shown,
            }],
            ..SymbolDocument::default()
        };

        assert_eq!(label("W", true).validate(), Ok(()));
        for unnamed in ["", "W L", "W=1u"] {
            let error = label(unnamed, true)
                .validate()
                .expect_err("a label must name one parameter");
            assert!(
                error.contains("label 1 does not name a parameter"),
                "{error}"
            );
        }
        // "W=1u" is four glyphs, five units each at the normal size.
        assert_eq!(
            label("W", true).body_bounds(),
            (Point::new(0, -4), Point::new(20, 4))
        );
        assert_eq!(
            label("W", false).body_bounds(),
            SymbolDocument::default().body_bounds()
        );
    }

    /// Reconciling used to reset every pin to its direction's default type,
    /// so a `Logic` pin came back `Analog` from an unrelated port change.
    #[test]
    fn reconciling_ports_keeps_a_pins_type_while_its_direction_stands() {
        let port = |name: &str, direction| PortSpec {
            name: name.to_owned(),
            direction,
        };
        let mut document = SymbolDocument::generated_from_ports(&[
            port("clk", PortDirection::In),
            port("q", PortDirection::Out),
        ]);
        for name in ["clk", "q"] {
            document.pin_mut(name).expect("the pin").electrical_type =
                Some(SymbolElectricalType::Logic);
        }
        let electrical = |document: &SymbolDocument, name: &str| {
            document.pin(name).and_then(|pin| pin.electrical_type)
        };

        // `q` turns round, and `en` is new.
        let ports = [
            port("clk", PortDirection::In),
            port("q", PortDirection::InOut),
            port("en", PortDirection::In),
        ];
        let mut untyped = document.clone();
        untyped.reconcile_ports(&ports);

        assert_eq!(
            electrical(&untyped, "clk"),
            Some(SymbolElectricalType::Logic)
        );
        assert_eq!(
            electrical(&untyped, "q"),
            Some(SymbolElectricalType::Analog)
        );
        assert_eq!(
            electrical(&untyped, "en"),
            Some(SymbolElectricalType::Analog)
        );

        // A type the interface states wins over the pin's and its direction's.
        let mut typed = document;
        typed.reconcile_typed_ports(&ports, |_| Some(SymbolElectricalType::Logic));
        for name in ["clk", "q", "en"] {
            assert_eq!(
                electrical(&typed, name),
                Some(SymbolElectricalType::Logic),
                "{name}"
            );
        }
    }

    /// A side the contract states for a port puts its new pin on that side,
    /// not on the one its direction implies; a pin already drawn stays put.
    #[test]
    fn a_contract_side_places_a_new_pin_where_the_contract_draws_it() {
        let port = |name: &str, direction| PortSpec {
            name: name.to_owned(),
            direction,
        };
        let drawn = SymbolPin::new("G", PortDirection::In, Some(Point::new(-60, 0)))
            .with_contract(SymbolElectricalType::Analog, SymbolPinSide::Left);
        let mut document = SymbolDocument {
            pins: vec![drawn.clone()],
            ..SymbolDocument::default()
        };
        let ports = [
            port("G", PortDirection::In),
            port("B", PortDirection::InOut),
        ];

        document.reconcile_contract_ports(
            &ports,
            |port| (port.name == "B").then_some(SymbolElectricalType::Power),
            |port| match port.name.as_str() {
                "B" => Some(SymbolPinSide::Bottom),
                _ => Some(SymbolPinSide::Top),
            },
        );

        let bulk = document.pin("B").expect("the missing terminal is added");
        assert_eq!(bulk.side, SymbolPinSide::Bottom);
        assert_eq!(bulk.electrical_type(), SymbolElectricalType::Power);
        let tip = bulk.position.expect("and placed");
        assert!(tip.y > 0, "below the body: {tip:?}");
        assert_eq!(document.pin("G"), Some(&drawn), "a drawn pin stays put");
    }
}
