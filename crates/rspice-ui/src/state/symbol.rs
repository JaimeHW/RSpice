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
//!
//! # One-way migration from editor schema 1
//!
//! Schema 1 kept free text beside the geometry, in the editor sidecar, where
//! only the editor ever drew it. [`SymbolDocument::load_from_view`] adopts
//! whatever a legacy view still carries into the body, and nothing writes the
//! sidecar field again. Two consequences follow, both deliberate:
//!
//! - A document this build writes carries a `Text` variant that a build older
//!   than editor schema 2 cannot deserialize; it will report the symbol
//!   metadata as invalid rather than silently drop the text.
//! - Schema 1's per-text visibility flag does not survive. Body geometry is
//!   always drawn, so a hidden legacy text becomes a drawn one; hiding it
//!   again means deleting it.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::{Point, PortDirection, PortSpec, SymbolElectricalType, SymbolPinSide, View};

/// Metadata key used by `View::metadata` for the symbol document JSON.
pub const SYMBOL_DOCUMENT_METADATA_KEY: &str = "rspice.symbol.document.v1";
/// Metadata key for symbol-editor-only text, attributes and revision history.
///
/// These records are kept beside the geometric document so older renderers
/// can continue to consume `rspice.symbol.document.v1` without silently
/// discarding the richer authoring contract.
pub const SYMBOL_EDITOR_METADATA_KEY: &str = "rspice.symbol.editor.v1";
pub const SYMBOL_EDITOR_METADATA_SCHEMA_VERSION: u16 = 2;

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
pub const MAX_SYMBOL_REVISION_NOTE_BYTES: usize = 4 * 1024;
pub const MAX_SYMBOL_REVISION_RECORDS: usize = 4_096;

/// Terminal-grid spacing in schematic coordinate units.
///
/// The design spec shows this as a 40 px editor lattice; in the Rust model
/// one schematic unit renders as four px at the symbol editor's 100% scale.
pub const SYMBOL_TERMINAL_GRID: i32 = 10;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolPin {
    pub name: String,
    pub direction: PortDirection,
    pub position: Option<Point>,
    /// Rich electrical semantics retained by model-bound symbols.
    ///
    /// Legacy documents omit this field and derive it from `direction`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub electrical_type: Option<SymbolElectricalType>,
    /// Authored body side. Legacy documents infer it from terminal geometry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<SymbolPinSide>,
    /// Distance along the authored body side in symbol coordinates.
    #[serde(default)]
    pub offset: i32,
}

impl SymbolPin {
    pub fn new(name: impl Into<String>, direction: PortDirection, position: Option<Point>) -> Self {
        Self {
            name: name.into(),
            direction,
            position,
            electrical_type: Some(default_electrical_type(direction)),
            side: Some(default_pin_side(direction, position)),
            offset: pin_offset(default_pin_side(direction, position), position),
        }
    }

    pub fn with_contract(
        mut self,
        electrical_type: SymbolElectricalType,
        side: SymbolPinSide,
        offset: i32,
    ) -> Self {
        self.electrical_type = Some(electrical_type);
        self.side = Some(side);
        self.offset = offset;
        self
    }

    pub fn electrical_type(&self) -> SymbolElectricalType {
        self.electrical_type
            .unwrap_or_else(|| default_electrical_type(self.direction))
    }

    pub fn side(&self) -> SymbolPinSide {
        self.side
            .unwrap_or_else(|| default_pin_side(self.direction, self.position))
    }

    pub fn offset(&self) -> i32 {
        if self.side.is_some() {
            self.offset
        } else {
            pin_offset(self.side(), self.position)
        }
    }

    pub fn set_electrical_contract(
        &mut self,
        electrical_type: SymbolElectricalType,
        direction: PortDirection,
    ) {
        self.electrical_type = Some(electrical_type);
        self.direction = direction;
    }

    pub fn set_side_and_offset(
        &mut self,
        side: SymbolPinSide,
        offset: i32,
        body_bounds: (Point, Point),
    ) {
        self.side = Some(side);
        self.offset = offset;
        self.position = Some(pin_position_for_geometry(side, offset, body_bounds));
    }

    pub fn terminal_on_grid(&self) -> bool {
        self.position.is_some_and(|point| {
            point.x % SYMBOL_TERMINAL_GRID == 0 && point.y % SYMBOL_TERMINAL_GRID == 0
        })
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

fn pin_offset(side: SymbolPinSide, position: Option<Point>) -> i32 {
    let Some(position) = position else {
        return 0;
    };
    match side {
        SymbolPinSide::Left | SymbolPinSide::Right => position.y,
        SymbolPinSide::Top | SymbolPinSide::Bottom => position.x,
    }
}

fn pin_position_for_geometry(
    side: SymbolPinSide,
    offset: i32,
    (min, max): (Point, Point),
) -> Point {
    let stub = SYMBOL_TERMINAL_GRID * 2;
    match side {
        SymbolPinSide::Left => Point::new(min.x.saturating_sub(stub), offset),
        SymbolPinSide::Right => Point::new(max.x.saturating_add(stub), offset),
        SymbolPinSide::Top => Point::new(offset, min.y.saturating_sub(stub)),
        SymbolPinSide::Bottom => Point::new(offset, max.y.saturating_add(stub)),
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

/// As much of editor schema 1's free-text record as the body needs.
///
/// Read only by the migration in [`SymbolDocument::load_from_view`], which
/// moves each one into the body as [`SymbolShape::Text`]; nothing writes it.
/// The record's `id` and `shown` are deliberately not modelled — identity was
/// a sidecar concern, and body geometry is always drawn — so unlike every
/// other record here this one tolerates the fields it does not read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct SymbolTextObject {
    text: String,
    position: Point,
}

/// Drawn height of a symbol text run, in symbol coordinate units.
///
/// The sizes are the ones the rest of a symbol already draws at, so authored
/// text sits in the same type hierarchy as the artwork around it: a pin name,
/// an instance name, and a heading above both.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolTextSize {
    Small,
    #[default]
    Normal,
    Large,
}

impl SymbolTextSize {
    pub const ALL: [Self; 3] = [Self::Small, Self::Normal, Self::Large];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Normal => "normal",
            Self::Large => "large",
        }
    }

    /// Cap height in symbol units. Renderers scale it to their viewport, so
    /// a text run keeps its proportion against the body at every zoom.
    pub const fn height(self) -> i32 {
        match self {
            Self::Small => 5,
            Self::Normal => 9,
            Self::Large => 14,
        }
    }
}

/// Which side of its anchor a symbol text run hangs off.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolTextAlign {
    #[default]
    Left,
    Center,
    Right,
}

impl SymbolTextAlign {
    pub const ALL: [Self; 3] = [Self::Left, Self::Center, Self::Right];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
        }
    }

    /// The alignment a horizontal mirror produces. Glyphs stay upright under
    /// every transform, so a mirror can only move the run to the other side
    /// of its anchor.
    pub const fn mirrored(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Center => Self::Center,
            Self::Right => Self::Left,
        }
    }

    /// Where the run sits once `orient` — the placed instance's own
    /// transform — has carried its advance direction into world space.
    ///
    /// The single owner of the rule, because every surface that draws
    /// authored text spells the answer differently and none of them may
    /// disagree about it.
    pub fn placement(self, orient: impl Fn(Point) -> Point) -> SymbolTextPlacement {
        let Some(step) = self.run_step() else {
            return SymbolTextPlacement::On;
        };
        let run = orient(step);
        if run.x.abs() >= run.y.abs() {
            if run.x >= 0 {
                SymbolTextPlacement::After
            } else {
                SymbolTextPlacement::Before
            }
        } else if run.y >= 0 {
            SymbolTextPlacement::Below
        } else {
            SymbolTextPlacement::Above
        }
    }

    /// The direction the run advances away from its anchor, or `None` when
    /// it straddles the anchor and no orientation can move it off.
    const fn run_step(self) -> Option<Point> {
        match self {
            Self::Left => Some(Point::new(1, 0)),
            Self::Center => None,
            Self::Right => Some(Point::new(-1, 0)),
        }
    }
}

/// Which side of its anchor an oriented text run ends up on.
///
/// Glyphs stay upright under every transform, so an orientation can only move
/// the run around its anchor; this names where it landed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolTextPlacement {
    /// Straddling the anchor, wherever the instance is turned.
    On,
    After,
    Before,
    Below,
    Above,
}

/// The box a text run occupies in symbol units, vertically centred on its
/// anchor.
///
/// The single owner of symbol type metrics. IBM Plex Mono advances 600/1000
/// em, so a glyph of cap height `h` is `3h/5` wide; every renderer sets that
/// same face and size, so what this measures is what they draw.
pub fn symbol_text_bounds(
    anchor: Point,
    text: &str,
    size: SymbolTextSize,
    align: SymbolTextAlign,
) -> (Point, Point) {
    let glyphs = i32::try_from(text.chars().count()).unwrap_or(i32::MAX);
    let span = glyphs.saturating_mul(size.height() * 3 / 5);
    let (before, after) = match align {
        SymbolTextAlign::Left => (0, span),
        SymbolTextAlign::Center => (span / 2, span / 2),
        SymbolTextAlign::Right => (span, 0),
    };
    let half_height = size.height() / 2;
    (
        Point::new(
            anchor.x.saturating_sub(before),
            anchor.y.saturating_sub(half_height),
        ),
        Point::new(
            anchor.x.saturating_add(after),
            anchor.y.saturating_add(half_height),
        ),
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SymbolEditorMetadata {
    pub schema_version: u16,
    pub attributes: Vec<SymbolAttribute>,
    pub revision: u64,
    pub revision_note: String,
    pub revisions: Vec<SymbolRevisionRecord>,
    /// Schema 1's free text, accepted so a legacy sidecar still parses.
    /// [`SymbolDocument::load_from_view`] is what adopts it into the body;
    /// [`Self::normalize`] then drops this copy and [`Self::validate`]
    /// refuses to encode one that still carries it.
    #[serde(default, rename = "texts", skip_serializing)]
    legacy_texts: Vec<SymbolTextObject>,
    #[serde(default, rename = "next_text_id", skip_serializing)]
    legacy_next_text_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolRevisionRecord {
    pub revision: u64,
    pub note: String,
    pub pin_count: usize,
    /// Every body shape, text included.
    pub shape_count: usize,
    /// How many of `shape_count` were text.
    pub text_count: usize,
}

impl Default for SymbolEditorMetadata {
    fn default() -> Self {
        Self {
            schema_version: SYMBOL_EDITOR_METADATA_SCHEMA_VERSION,
            attributes: Vec::new(),
            revision: 0,
            revision_note: String::new(),
            revisions: Vec::new(),
            legacy_texts: Vec::new(),
            legacy_next_text_id: 0,
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
        // The body owns authored text now; the document adopted these on
        // load, so the sidecar's copy is spent.
        self.legacy_texts.clear();
        self.legacy_next_text_id = 0;
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
        if !self.legacy_texts.is_empty() || self.legacy_next_text_id != 0 {
            return Err(
                "Invalid symbol editor metadata: authored text belongs to the symbol body"
                    .to_owned(),
            );
        }
        if self.revision_note.len() > MAX_SYMBOL_REVISION_NOTE_BYTES
            || self.revision_note.chars().any(char::is_control)
        {
            return Err("Invalid symbol revision note".to_owned());
        }
        if self.revisions.len() > MAX_SYMBOL_REVISION_RECORDS {
            return Err(format!(
                "Invalid symbol editor metadata: {} revision records exceed the limit of {MAX_SYMBOL_REVISION_RECORDS}",
                self.revisions.len()
            ));
        }
        let mut previous_revision = 0;
        for record in &self.revisions {
            validate_symbol_text(&record.note, "revision note")?;
            if record.note.len() > MAX_SYMBOL_REVISION_NOTE_BYTES {
                return Err(format!(
                    "Invalid symbol editor metadata: revision note exceeds the limit of {MAX_SYMBOL_REVISION_NOTE_BYTES} bytes"
                ));
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

    pub fn publish_revision(
        &mut self,
        document: &SymbolDocument,
        revision_note: &str,
    ) -> Result<u64, String> {
        let note = revision_note.trim();
        if note.is_empty() {
            return Err("A revision note is required.".to_owned());
        }
        if note.len() > MAX_SYMBOL_REVISION_NOTE_BYTES {
            return Err(format!(
                "The revision note exceeds the limit of {MAX_SYMBOL_REVISION_NOTE_BYTES} bytes."
            ));
        }
        validate_symbol_text(note, "revision note")?;
        if self.revisions.len() >= MAX_SYMBOL_REVISION_RECORDS {
            return Err(format!(
                "The symbol revision history reached its limit of {MAX_SYMBOL_REVISION_RECORDS} records."
            ));
        }
        let revision = self
            .revision
            .checked_add(1)
            .ok_or_else(|| "The symbol revision counter is exhausted.".to_owned())?;
        self.revision = revision;
        self.revision_note = note.to_owned();
        self.revisions.push(SymbolRevisionRecord {
            revision,
            note: note.to_owned(),
            pin_count: document.pins.len(),
            shape_count: document.body.len(),
            text_count: document.text_runs().count(),
        });
        Ok(revision)
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum SymbolShape {
    Polyline {
        points: Vec<Point>,
        closed: bool,
    },
    Circle {
        center: Point,
        radius: i32,
    },
    Arc {
        center: Point,
        radius: i32,
        start_degrees: i32,
        sweep_degrees: i32,
    },
    Arrow {
        tip: Point,
        rotation_quarters: i32,
    },
    Dot {
        center: Point,
        radius: i32,
    },
    /// A run of authored glyphs. The anchor is geometry and moves with every
    /// transform; the glyphs themselves stay upright and read left to right
    /// however the shape or the instance is turned, which is what makes a
    /// mirrored or rotated symbol still legible.
    Text {
        anchor: Point,
        text: String,
        size: SymbolTextSize,
        align: SymbolTextAlign,
    },
}

impl SymbolShape {
    pub fn translate(&mut self, delta: Point) {
        self.map_points(|point| point + delta);
    }

    pub fn rotate_cw(&mut self) {
        self.map_points(|point| Point::new(-point.y, point.x));
        match self {
            SymbolShape::Arc { start_degrees, .. } => {
                *start_degrees = (*start_degrees as i64 + 90).rem_euclid(360) as i32;
            }
            SymbolShape::Arrow {
                rotation_quarters, ..
            } => {
                *rotation_quarters = (*rotation_quarters as i64 + 1).rem_euclid(4) as i32;
            }
            _ => {}
        }
    }

    pub fn mirror_h(&mut self) {
        self.map_points(|point| Point::new(-point.x, point.y));
        match self {
            SymbolShape::Arc {
                start_degrees,
                sweep_degrees,
                ..
            } => {
                *start_degrees = (180_i64 - *start_degrees as i64 - *sweep_degrees as i64)
                    .rem_euclid(360) as i32;
            }
            SymbolShape::Arrow {
                rotation_quarters, ..
            } => {
                *rotation_quarters = (2 - *rotation_quarters).rem_euclid(4);
            }
            // The anchor has already moved; the run has to change which side
            // of it it hangs off, or a mirrored label lands across the body
            // it was set beside.
            SymbolShape::Text { align, .. } => *align = align.mirrored(),
            _ => {}
        }
    }

    pub fn mirror_v(&mut self) {
        self.map_points(|point| Point::new(point.x, -point.y));
        match self {
            SymbolShape::Arc {
                start_degrees,
                sweep_degrees,
                ..
            } => {
                *start_degrees =
                    (-(*start_degrees as i64) - *sweep_degrees as i64).rem_euclid(360) as i32;
            }
            SymbolShape::Arrow {
                rotation_quarters, ..
            } => {
                *rotation_quarters = (-*rotation_quarters).rem_euclid(4);
            }
            _ => {}
        }
    }

    fn map_points(&mut self, transform: impl Fn(Point) -> Point) {
        match self {
            SymbolShape::Polyline { points, .. } => {
                for point in points {
                    *point = transform(*point);
                }
            }
            SymbolShape::Circle { center, .. }
            | SymbolShape::Arc { center, .. }
            | SymbolShape::Dot { center, .. } => {
                *center = transform(*center);
            }
            SymbolShape::Arrow { tip, .. } => {
                *tip = transform(*tip);
            }
            SymbolShape::Text { anchor, .. } => {
                *anchor = transform(*anchor);
            }
        }
    }
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinSummary {
    Match,
    Unplaced(usize),
    Orphaned(usize),
    NoSchematic,
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
        }
    }
}

impl SymbolDocument {
    pub fn generated_from_ports(ports: &[PortSpec]) -> Self {
        let generated = super::generate_symbol(ports);
        let body_half_width = generated.body_half_width();
        let body_half_height = generated.body_half_height();
        // The generator already knows which edge every pin belongs to; record
        // it, so nothing downstream has to re-derive the edge from a terminal
        // coordinate that cannot express it.
        let pins = generated
            .pins
            .into_iter()
            .map(|pin| {
                let offset = pin.offset;
                let side = pin.side;
                SymbolPin::new(pin.name, pin.direction, Some(offset)).with_contract(
                    default_electrical_type(pin.direction),
                    side,
                    match side {
                        SymbolPinSide::Left | SymbolPinSide::Right => offset.y,
                        SymbolPinSide::Top | SymbolPinSide::Bottom => offset.x,
                    },
                )
            })
            .collect();
        Self {
            pins,
            body: vec![SymbolShape::Polyline {
                points: vec![
                    Point::new(-body_half_width, -body_half_height),
                    Point::new(body_half_width, -body_half_height),
                    Point::new(body_half_width, body_half_height),
                    Point::new(-body_half_width, body_half_height),
                ],
                closed: true,
            }],
            origin: Point::origin(),
            name_anchor: Point::new(
                -body_half_width,
                -body_half_height - SYMBOL_TERMINAL_GRID * 2,
            ),
            value_anchor: Point::new(
                -body_half_width,
                body_half_height + SYMBOL_TERMINAL_GRID * 2,
            ),
        }
    }

    pub fn load_from_view(view: &View) -> Result<Self, String> {
        let mut document = match view.metadata.get(SYMBOL_DOCUMENT_METADATA_KEY) {
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
        // Text authored before editor schema 2 lived in the sidecar and was
        // drawn only by the editor. Adopting it here is what puts it on every
        // placed instance and every export, whether or not the symbol is ever
        // saved again.
        for text in legacy_editor_texts(view) {
            document.body.push(SymbolShape::Text {
                anchor: text.position,
                text: text.text,
                size: SymbolTextSize::Normal,
                align: SymbolTextAlign::Left,
            });
        }
        document.validate()?;
        Ok(document)
    }

    /// Every authored text run in the body, with the point it hangs off, in
    /// body order.
    pub fn text_runs(&self) -> impl Iterator<Item = (&str, Point)> {
        self.body.iter().filter_map(|shape| match shape {
            SymbolShape::Text { anchor, text, .. } => Some((text.as_str(), *anchor)),
            _ => None,
        })
    }

    pub fn store_in_view(&self, view: &mut View) -> Result<(), String> {
        self.validate()?;
        let raw = serde_json::to_string(self)
            .map_err(|err| format!("Could not serialize symbol metadata: {err}"))?;
        if raw.len() > MAX_SYMBOL_DOCUMENT_BYTES {
            return Err(format!(
                "Could not serialize symbol metadata: document is {} bytes; the limit is {MAX_SYMBOL_DOCUMENT_BYTES}",
                raw.len()
            ));
        }
        view.metadata
            .insert(SYMBOL_DOCUMENT_METADATA_KEY.to_owned(), raw);
        // The body owns whatever schema-1 text this view carried, so the
        // sidecar's copy has to go with the same write: left behind, the next
        // load would adopt it a second time and double every label.
        retire_legacy_editor_texts(view);
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
            if let Some(position) = pin.position {
                validate_symbol_point(position, &format!("pin '{name}' terminal"))?;
            }
            if pin.offset().unsigned_abs() > MAX_SYMBOL_COORDINATE_ABS as u32 {
                return Err(format!(
                    "Invalid symbol metadata: pin '{name}' offset is outside the supported range"
                ));
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
                SymbolShape::Circle { center, radius } | SymbolShape::Dot { center, radius } => {
                    validate_symbol_point(*center, &format!("round shape {ordinal} center"))?;
                    validate_symbol_radius(*center, *radius, ordinal)?;
                }
                SymbolShape::Arc {
                    center,
                    radius,
                    start_degrees,
                    sweep_degrees,
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

    pub fn reconcile_ports(&mut self, ports: &[PortSpec]) {
        let mut existing = HashSet::new();
        for port in ports {
            if let Some(pin) = self.pin_mut(&port.name) {
                pin.name = port.name.clone();
                pin.direction = port.direction;
            } else {
                self.pins
                    .push(SymbolPin::new(&port.name, port.direction, None));
            }
            existing.insert(port.name.to_ascii_lowercase());
        }

        self.pins.sort_by_key(|pin| {
            ports
                .iter()
                .position(|port| port.name.eq_ignore_ascii_case(&pin.name))
                .unwrap_or(ports.len())
        });
    }

    pub fn pin_summary(&self, ports: &[PortSpec]) -> PinSummary {
        if ports.is_empty() {
            return PinSummary::NoSchematic;
        }
        let port_names = port_name_set(ports);
        let unplaced = ports
            .iter()
            .filter(|port| {
                self.pin(&port.name)
                    .is_none_or(|pin| pin.position.is_none())
            })
            .count();
        if unplaced > 0 {
            return PinSummary::Unplaced(unplaced);
        }
        let orphaned = self
            .pins
            .iter()
            .filter(|pin| !port_names.contains(&pin.name.to_ascii_lowercase()))
            .count();
        if orphaned > 0 {
            return PinSummary::Orphaned(orphaned);
        }
        PinSummary::Match
    }

    pub fn pin_findings(&self, ports: &[PortSpec]) -> Vec<PinFinding> {
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
            if !port_names.contains(&pin.name.to_ascii_lowercase()) && !ports.is_empty() {
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

    /// Which body edge a pin is drawn against.
    ///
    /// The authored side wins wherever the artwork agrees it could be that
    /// edge. Geometry only steps in for a terminal that stands off a
    /// different edge entirely — a document written before the side was
    /// recorded, or one derived from a terminal list — so that no lead is
    /// ever drawn away from the body it is supposed to reach.
    pub fn pin_side(&self, pin: &SymbolPin) -> SymbolPinSide {
        let (Some(position), Some(bounds)) = (pin.position, self.drawn_body_bounds()) else {
            return pin.side();
        };
        if let Some(declared) = pin.side
            && edge_standoff(position, bounds, declared) >= 0
        {
            return declared;
        }
        pin_side_against_body(position, pin.direction, Some(bounds))
    }

    /// Body-only bounds used for side/offset pin placement.
    ///
    /// Pins, attributes and the editing origin are intentionally excluded:
    /// moving one terminal must never recursively expand the body it is
    /// anchored to.
    pub fn body_bounds(&self) -> (Point, Point) {
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        for shape in &self.body {
            match shape {
                SymbolShape::Polyline { points, .. } => {
                    for point in points {
                        xs.push(point.x);
                        ys.push(point.y);
                    }
                }
                SymbolShape::Circle { center, radius } | SymbolShape::Dot { center, radius } => {
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
                } => {
                    let (min, max) = symbol_text_bounds(*anchor, text, *size, *align);
                    xs.extend([min.x, max.x]);
                    ys.extend([min.y, max.y]);
                }
            }
        }
        if xs.is_empty() {
            return (
                Point::new(-SYMBOL_TERMINAL_GRID * 4, -SYMBOL_TERMINAL_GRID * 4),
                Point::new(SYMBOL_TERMINAL_GRID * 4, SYMBOL_TERMINAL_GRID * 4),
            );
        }
        (
            Point::new(
                xs.iter().min().copied().unwrap_or(-40),
                ys.iter().min().copied().unwrap_or(-40),
            ),
            Point::new(
                xs.iter().max().copied().unwrap_or(40),
                ys.iter().max().copied().unwrap_or(40),
            ),
        )
    }
}

/// Free text an editor-schema-1 sidecar still carries.
///
/// A sidecar this build writes never names the field, so the substring test
/// keeps the ordinary load — which happens once per resolved instance — free
/// of a second parse.
fn legacy_editor_texts(view: &View) -> Vec<SymbolTextObject> {
    let Some(encoded) = view.metadata.get(SYMBOL_EDITOR_METADATA_KEY) else {
        return Vec::new();
    };
    if encoded.len() > MAX_SYMBOL_DOCUMENT_BYTES || !encoded.contains("\"texts\"") {
        return Vec::new();
    }
    serde_json::from_str::<SymbolEditorMetadata>(encoded)
        .map(|sidecar| sidecar.legacy_texts)
        .unwrap_or_default()
}

/// Drop the schema-1 free text from a view's sidecar, leaving the rest of it
/// exactly as written so a field this build does not model survives the edit.
fn retire_legacy_editor_texts(view: &mut View) {
    let Some(encoded) = view.metadata.get(SYMBOL_EDITOR_METADATA_KEY) else {
        return;
    };
    if !encoded.contains("\"texts\"") {
        return;
    }
    let Ok(serde_json::Value::Object(mut sidecar)) = serde_json::from_str(encoded) else {
        return;
    };
    sidecar.remove("texts");
    sidecar.remove("next_text_id");
    if let Ok(rewritten) = serde_json::to_string(&sidecar) {
        view.metadata
            .insert(SYMBOL_EDITOR_METADATA_KEY.to_owned(), rewritten);
    }
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
    use crate::state::ViewType;

    #[test]
    fn loading_rejects_arc_sweeps_that_would_expand_unbounded_tessellation() {
        let mut view = View::new("symbol", ViewType::Symbol);
        let document = SymbolDocument {
            body: vec![SymbolShape::Arc {
                center: Point::origin(),
                radius: 10,
                start_degrees: 0,
                sweep_degrees: i32::MAX,
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
            }],
            ..SymbolDocument::default()
        };

        let error = document
            .validate()
            .expect_err("overflowing round-shape bounds must be rejected");

        assert!(error.contains("bounds exceed the coordinate representation"));
    }

    #[test]
    fn repeated_shape_transforms_keep_angles_in_canonical_ranges() {
        let mut arc = SymbolShape::Arc {
            center: Point::origin(),
            radius: 10,
            start_degrees: 0,
            sweep_degrees: 180,
        };
        let mut arrow = SymbolShape::Arrow {
            tip: Point::origin(),
            rotation_quarters: 0,
        };

        for _ in 0..10_000 {
            arc.rotate_cw();
            arrow.rotate_cw();
        }

        let SymbolShape::Arc { start_degrees, .. } = arc else {
            unreachable!();
        };
        let SymbolShape::Arrow {
            rotation_quarters, ..
        } = arrow
        else {
            unreachable!();
        };
        assert!((0..360).contains(&start_degrees));
        assert!((0..4).contains(&rotation_quarters));
    }

    #[test]
    fn pin_side_and_offset_produce_canonical_terminal_geometry() {
        let document = SymbolDocument {
            body: vec![SymbolShape::Polyline {
                points: vec![
                    Point::new(-20, -20),
                    Point::new(20, -20),
                    Point::new(20, 20),
                    Point::new(-20, 20),
                ],
                closed: true,
            }],
            ..SymbolDocument::default()
        };
        let mut pin = SymbolPin::new("OUT", PortDirection::Out, None);

        pin.set_side_and_offset(SymbolPinSide::Right, 10, document.body_bounds());

        assert_eq!(pin.side(), SymbolPinSide::Right);
        assert_eq!(pin.offset(), 10);
        assert_eq!(pin.position, Some(Point::new(40, 10)));
        assert!(pin.terminal_on_grid());
    }

    #[test]
    fn editor_metadata_round_trips_attributes_and_revision_history() {
        let document = SymbolDocument {
            body: vec![SymbolShape::Text {
                anchor: Point::new(10, -20),
                text: "gain stage".to_owned(),
                size: SymbolTextSize::Normal,
                align: SymbolTextAlign::Left,
            }],
            ..SymbolDocument::default()
        };
        let mut metadata = SymbolEditorMetadata::for_document(&document);
        let revision = metadata
            .publish_revision(&document, "Initial reviewed symbol")
            .expect("revision publishes");
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
        assert_eq!(restored.revisions[0].shape_count, 1);
        assert_eq!(restored.revisions[0].text_count, 1);
        assert_eq!(
            restored
                .attribute(SymbolAttributeKind::Reference)
                .expect("reference attribute retained")
                .default_value,
            "U?"
        );
    }

    /// T13: four quarter turns and two horizontal mirrors are the identity on
    /// a text run, alignment included — otherwise a symbol drifts every time
    /// it is turned back to where it started.
    #[test]
    fn text_shapes_round_trip_four_rotations_and_two_mirrors() {
        let original = SymbolShape::Text {
            anchor: Point::new(30, -10),
            text: "AMP".to_owned(),
            size: SymbolTextSize::Large,
            align: SymbolTextAlign::Right,
        };

        let mut rotated = original.clone();
        for _ in 0..4 {
            rotated.rotate_cw();
        }
        let mut mirrored = original.clone();
        mirrored.mirror_h();
        let once = mirrored.clone();
        mirrored.mirror_h();
        let mut flipped = original.clone();
        flipped.mirror_v();
        flipped.mirror_v();

        assert_eq!(rotated, original);
        assert_eq!(mirrored, original);
        assert_eq!(flipped, original);
        assert_eq!(
            once,
            SymbolShape::Text {
                anchor: Point::new(-30, -10),
                text: "AMP".to_owned(),
                size: SymbolTextSize::Large,
                align: SymbolTextAlign::Left,
            },
            "one mirror moves the anchor and the side the run hangs off"
        );
    }

    /// A quarter turn moves the anchor and nothing else: the glyphs stay
    /// upright, so neither the alignment nor the size may follow it round.
    #[test]
    fn rotating_a_text_shape_moves_only_its_anchor() {
        let mut shape = SymbolShape::Text {
            anchor: Point::new(20, 5),
            text: "OTA".to_owned(),
            size: SymbolTextSize::Small,
            align: SymbolTextAlign::Left,
        };

        shape.rotate_cw();

        assert_eq!(
            shape,
            SymbolShape::Text {
                anchor: Point::new(-5, 20),
                text: "OTA".to_owned(),
                size: SymbolTextSize::Small,
                align: SymbolTextAlign::Left,
            }
        );
    }

    #[test]
    fn text_runs_are_measured_from_the_side_they_hang_off() {
        let anchor = Point::new(0, 0);

        let left = symbol_text_bounds(anchor, "AMP", SymbolTextSize::Normal, SymbolTextAlign::Left);
        let right = symbol_text_bounds(
            anchor,
            "AMP",
            SymbolTextSize::Normal,
            SymbolTextAlign::Right,
        );
        let centered = symbol_text_bounds(
            anchor,
            "AMP",
            SymbolTextSize::Normal,
            SymbolTextAlign::Center,
        );

        // Plex Mono at a 9-unit cap height advances 5 units per glyph.
        assert_eq!(left, (Point::new(0, -4), Point::new(15, 4)));
        assert_eq!(right, (Point::new(-15, -4), Point::new(0, 4)));
        assert_eq!(centered, (Point::new(-7, -4), Point::new(7, 4)));
    }

    #[test]
    fn body_bounds_cover_the_whole_text_run() {
        let document = SymbolDocument {
            body: vec![SymbolShape::Text {
                anchor: Point::origin(),
                text: "AMP".to_owned(),
                size: SymbolTextSize::Normal,
                align: SymbolTextAlign::Left,
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
            }],
            ..SymbolDocument::default()
        };

        let error = document
            .validate()
            .expect_err("a control character in body text must be rejected");

        assert!(error.contains("text 1 is invalid or too long"), "{error}");
    }

    /// The one-way migration: a view written by editor schema 1 loads with
    /// its free text in the body, and the sidecar it re-encodes no longer
    /// names the field.
    #[test]
    fn legacy_sidecar_text_is_adopted_into_the_body_and_never_written_back() {
        let mut view = View::new("symbol", ViewType::Symbol);
        SymbolDocument::default()
            .store_in_view(&mut view)
            .expect("the empty document stores");
        view.metadata.insert(
            SYMBOL_EDITOR_METADATA_KEY.to_owned(),
            r#"{"schema_version":1,"attributes":[{"kind":"reference","default_value":"U?","shown":true,"position":{"x":-20,"y":-40}},{"kind":"value","default_value":"VALUE","shown":true,"position":{"x":-20,"y":40}},{"kind":"model","default_value":"","shown":false,"position":{"x":-20,"y":60}}],"texts":[{"id":1,"text":"AMP","position":{"x":10,"y":-20},"shown":true}],"next_text_id":2,"revision":0,"revision_note":"","revisions":[]}"#
                .to_owned(),
        );

        let document = SymbolDocument::load_from_view(&view).expect("the legacy document loads");
        let metadata = SymbolEditorMetadata::load_from_view(&view, &document)
            .expect("the legacy sidecar loads");
        let encoded = metadata.encode().expect("the migrated sidecar encodes");

        assert_eq!(
            document.body,
            vec![SymbolShape::Text {
                anchor: Point::new(10, -20),
                text: "AMP".to_owned(),
                size: SymbolTextSize::Normal,
                align: SymbolTextAlign::Left,
            }]
        );
        assert!(
            !encoded.contains("\"texts\""),
            "the migrated sidecar must not carry free text: {encoded}"
        );
        assert!(
            !encoded.contains("next_text_id"),
            "the migrated sidecar must not carry a text id counter: {encoded}"
        );
        assert_eq!(
            metadata.schema_version,
            SYMBOL_EDITOR_METADATA_SCHEMA_VERSION
        );
    }

    /// Storing the migrated document retires the sidecar field it came from,
    /// so a reload adopts nothing and the label is not drawn twice.
    #[test]
    fn storing_a_migrated_document_retires_the_sidecar_it_adopted_from() {
        let mut view = View::new("symbol", ViewType::Symbol);
        view.metadata.insert(
            SYMBOL_EDITOR_METADATA_KEY.to_owned(),
            r#"{"schema_version":1,"attributes":[],"texts":[{"id":1,"text":"AMP","position":{"x":0,"y":0},"shown":true}],"next_text_id":2,"revision":0,"revision_note":"","revisions":[]}"#
                .to_owned(),
        );

        let migrated = SymbolDocument::load_from_view(&view).expect("the legacy document loads");
        migrated
            .store_in_view(&mut view)
            .expect("the migrated document stores");
        let reloaded = SymbolDocument::load_from_view(&view).expect("the stored document reloads");

        assert_eq!(migrated.text_runs().count(), 1);
        assert_eq!(reloaded.body, migrated.body);
    }
}
