//! Keeping what an import adds to a drawing clear of what the drawing drew.

use rspice_design_model::Point;

use super::{Builder, GRID};
use crate::foreign_schematic::{
    ForeignPoint, ForeignShape, ForeignSymbol, ForeignSymbolDefinition,
};
use crate::schematic::component::Component;
use crate::schematic::component_display::SchematicParameterLabelVisibility;
use crate::schematic::component_type::ComponentType;
use crate::schematic::design_note::{DESIGN_NOTE_TEXT_OFFSET_WORLD, DesignNote, DesignNoteKind};
use crate::schematic::drawing_metrics::SYMBOL_WIDTH_WORLD;
use crate::schematic::label_layout::{
    INSTANCE_NAME_WORLD, INSTANCE_VALUE_WORLD, LayoutPoint, LayoutRect, PlacedText, TextEdge,
    instance_label_layout, net_label_layout, supply_glyph_rect, text_box,
};
use crate::schematic::net_label::NetLabel;
use crate::schematic::value_label::{component_value_label, value_pieces};
use crate::symbol::{SymbolDocument, SymbolShape};
#[cfg(feature = "symbol-artwork")]
use crate::symbol_artwork::{PlacedRun, SymbolLibrary};

/// The air an instance's name or value keeps from a body RSpice draws for
/// it, as the shared layout keeps it.
const LABEL_CLEARANCE: f32 = 4.0;

/// How far a symbol's stroke inks either side of its line.
const STROKE_HALF_WIDTH: f32 = SYMBOL_WIDTH_WORLD / 2.0;

impl Builder<'_> {
    /// The boxes a cell's name and value take where `symbol`'s windows, else
    /// its definition's, letter them, in the symbol's own frame: what an
    /// unconnected terminal's mark keeps off.
    pub(super) fn symbol_lettering(
        &self,
        symbol: &ForeignSymbol,
        definition: &ForeignSymbolDefinition,
    ) -> Vec<LayoutRect> {
        let window = |attribute: u32| {
            symbol
                .windows
                .iter()
                .chain(&definition.windows)
                .find(|window| window.attribute == attribute)
                .filter(|window| !window.hidden)
        };
        let value = symbol
            .attribute("Value")
            .or_else(|| definition.attribute("Value"));
        let edge = |step: i32| match step.signum() {
            1 => TextEdge::Start,
            -1 => TextEdge::End,
            _ => TextEdge::Center,
        };
        [
            (window(0), symbol.attribute("InstName"), INSTANCE_NAME_WORLD),
            (window(3), value, INSTANCE_VALUE_WORLD),
        ]
        .into_iter()
        .filter_map(|(window, text, size)| {
            let (window, text) = (window?, text?);
            let anchor = self.scale(window.offset);
            let (horizontal, vertical) = if window.vertical {
                (edge(-window.reach.y), edge(window.reach.x))
            } else {
                (edge(window.reach.x), edge(window.reach.y))
            };
            let placed = PlacedText {
                anchor: LayoutPoint::new(anchor.x as f32, anchor.y as f32),
                horizontal,
                vertical,
                upward: window.vertical,
            };
            Some(text_box(placed, text, size))
        })
        .collect()
    }

    /// A note where a refused component stood, so the gap reads as one: it
    /// hangs from the top left of the body, its label under the words, so it
    /// stays on the body's own ground rather than across its neighbours.
    /// It marks no point, so it draws no anchor mark: a mark at the corner
    /// would touch a flag standing on the refused part's pin.
    pub(super) fn place_marker(
        &mut self,
        symbol: &ForeignSymbol,
        definition: Option<&ForeignSymbolDefinition>,
        label: &str,
    ) {
        let id = self.id();
        let (low, high) = definition
            .into_iter()
            .flat_map(|definition| {
                let shapes = definition.shapes.iter().flat_map(|shape| match shape {
                    ForeignShape::Line(a, b)
                    | ForeignShape::Rectangle(a, b)
                    | ForeignShape::Ellipse(a, b) => vec![*a, *b],
                    // An arc's box is its whole ellipse's, often wider than
                    // the body.
                    ForeignShape::Arc(_) => Vec::new(),
                });
                shapes.chain(definition.pins.iter().map(|pin| pin.position))
            })
            .map(|point| symbol.orientation.apply(point))
            .map(|point| (point, point))
            .reduce(|(low, high), (point, _)| {
                (
                    ForeignPoint::new(low.x.min(point.x), low.y.min(point.y)),
                    ForeignPoint::new(high.x.max(point.x), high.y.max(point.y)),
                )
            })
            .unwrap_or_default();
        let text = format!("Not imported:\n{label}");
        if let Ok(note) = DesignNote::new(
            id,
            self.scale(symbol.position.offset(low)),
            DesignNoteKind::PlainText,
            text,
        ) {
            self.markers.push((
                self.document.design_notes.len(),
                self.scale(symbol.position.offset(high)),
            ));
            self.document
                .design_notes
                .push(note.with_anchor_mark_hidden(true));
        }
    }

    /// A refused part's marker that would land on a body, a name or value,
    /// a net label, a wire or another note hangs instead from the first
    /// other place along its part's gap where it lands on none: from the
    /// middle or the right of the gap's top, else standing on the left,
    /// middle or right of its bottom. With no such place it stays where it
    /// was.
    pub(super) fn clear_markers(&mut self) {
        if self.markers.is_empty() {
            return;
        }
        let at = |point: Point| LayoutPoint::new(point.x as f32, point.y as f32);
        let obstacles = self
            .drawn_bodies()
            .into_iter()
            .flatten()
            .chain(
                self.document
                    .net_labels
                    .iter()
                    .flat_map(net_label_boxes)
                    .map(Ink::Area),
            )
            .chain(
                self.document
                    .components
                    .iter()
                    .flat_map(|part| lettering_boxes(part).into_iter().map(Ink::Area)),
            )
            .chain(self.document.wires.iter().flat_map(|wire| {
                wire.points
                    .windows(2)
                    .map(|piece| Ink::Stroke(at(piece[0]), at(piece[1])))
                    .collect::<Vec<_>>()
            }))
            .collect::<Vec<_>>();
        let mut notes = self
            .document
            .design_notes
            .iter()
            .map(|note| note_box(note, note.pos))
            .collect::<Vec<_>>();
        for &(index, high) in &self.markers {
            let note = &self.document.design_notes[index];
            let lands = |rect: LayoutRect| {
                obstacles.iter().any(|ink| ink.meets(rect))
                    || notes
                        .iter()
                        .enumerate()
                        .any(|(other, note)| other != index && overlaps(rect, *note))
            };
            if !lands(notes[index]) {
                continue;
            }
            let (width, height) = note.text_extent_world(&note.text);
            let (offset_x, offset_y) = DESIGN_NOTE_TEXT_OFFSET_WORLD;
            let right = high.x - (width + offset_x).ceil() as i32;
            let centre = (note.pos.x + right) / 2;
            let bottom = high.y - (height + offset_y).ceil() as i32;
            let place = [note.pos.y, bottom]
                .into_iter()
                .flat_map(|y| [note.pos.x, centre, right].map(|x| Point::new(x, y)))
                .skip(1)
                .find(|pos| !lands(note_box(note, *pos)));
            if let Some(pos) = place {
                notes[index] = note_box(note, pos);
                self.document.design_notes[index].pos = pos;
            }
        }
    }

    /// The ink each part's body draws, by part: a native part's artwork as
    /// the canvas places it, a cell's own drawing.
    fn drawn_bodies(&self) -> Vec<Vec<Ink>> {
        self.document
            .components
            .iter()
            .map(|component| match &component.library_cell {
                _ if component.source_symbol().is_some() => component
                    .source_symbol()
                    .map(|symbol| body_ink(symbol.document(), component))
                    .unwrap_or_default(),
                _ if component.kind != ComponentType::CellInstance => native_ink(component),
                Some(binding) => self
                    .cells
                    .iter()
                    .find(|cell| cell.cell == binding.cell)
                    .map(|cell| body_ink(&cell.symbol, component))
                    .unwrap_or_default(),
                None => Vec::new(),
            })
            .collect()
    }
}

/// The box a part RSpice draws natively stands in: its symbol's extent,
/// turned with it, about its position.
fn native_body(component: &Component) -> LayoutRect {
    let (width, height) = component.symbol_dimensions();
    let (width, height) = if component.rotation.is_vertical() {
        (height, width)
    } else {
        (width, height)
    };
    let (x, y) = (component.pos.x as f32, component.pos.y as f32);
    let (half_width, half_height) = (width as f32 * 0.5, height as f32 * 0.5);
    LayoutRect {
        min: LayoutPoint::new(x - half_width, y - half_height),
        max: LayoutPoint::new(x + half_width, y + half_height),
    }
}

/// The ink of a part RSpice draws with its own artwork, in world units,
/// as the canvas draws it; its symbol's box when the artwork is not built
/// in or has no symbol for it.
#[cfg(feature = "symbol-artwork")]
fn native_ink(component: &Component) -> Vec<Ink> {
    static ARTWORK: std::sync::LazyLock<Option<SymbolLibrary>> =
        std::sync::LazyLock::new(|| SymbolLibrary::load_embedded().ok());
    match ARTWORK
        .as_ref()
        .and_then(|library| library.placed_artwork(component))
    {
        Some(runs) => artwork_ink(component, &runs),
        None => vec![Ink::Area(native_body(component))],
    }
}

/// The ink of a part RSpice draws with its own artwork: its symbol's box,
/// when the artwork is not built in.
#[cfg(not(feature = "symbol-artwork"))]
fn native_ink(component: &Component) -> Vec<Ink> {
    vec![Ink::Area(native_body(component))]
}

/// The ink of artwork placed for `component`: each straight piece of its
/// strokes and the box of each shape it fills.
#[cfg(feature = "symbol-artwork")]
fn artwork_ink(component: &Component, runs: &[PlacedRun]) -> Vec<Ink> {
    let (x, y) = (component.pos.x as f32, component.pos.y as f32);
    let mut ink = Vec::new();
    for run in runs {
        let points = run
            .points
            .iter()
            .map(|&(across, down)| LayoutPoint::new(x + across, y + down))
            .collect::<Vec<_>>();
        if run.filled {
            let (mut min, mut max) = (points[0], points[0]);
            for point in &points {
                min = LayoutPoint::new(min.x.min(point.x), min.y.min(point.y));
                max = LayoutPoint::new(max.x.max(point.x), max.y.max(point.y));
            }
            ink.push(Ink::Area(LayoutRect { min, max }));
        } else {
            ink.extend(points.windows(2).map(|run| Ink::Stroke(run[0], run[1])));
        }
    }
    ink
}

/// The boxes a part's shown name and value take where they are placed.
fn lettering_boxes(component: &Component) -> Vec<LayoutRect> {
    let origin = LayoutPoint::new(component.pos.x as f32, component.pos.y as f32);
    let layout = instance_label_layout(origin, 1.0, component);
    let shown = SchematicParameterLabelVisibility::NamesAndValues;
    let mut boxes = Vec::new();
    if component.kind == ComponentType::Ground {
        return boxes;
    }
    if component.display_mode.show_name(shown) && !component.name.trim().is_empty() {
        boxes.push(text_box(
            layout.name,
            &component.name,
            component.name_label_pos.size_world(INSTANCE_NAME_WORLD),
        ));
    }
    if !component.display_mode.show_value(shown) {
        return boxes;
    }
    match value_pieces(component) {
        Some(pieces) => boxes.extend(pieces.iter().filter_map(|piece| {
            let placed = piece.position.placed_text(origin, 1.0)?;
            Some(text_box(placed, &piece.text, piece.size_world()))
        })),
        None => {
            let value = component_value_label(component);
            if !value.trim().is_empty() {
                boxes.push(text_box(
                    layout.value,
                    &value,
                    component.value_label_pos.size_world(INSTANCE_VALUE_WORLD),
                ));
            }
        }
    }
    boxes
}

/// The boxes a net label's name, connector tab and supply symbol take.
fn net_label_boxes(label: &NetLabel) -> Vec<LayoutRect> {
    let layout = net_label_layout(label);
    let name = text_box(
        PlacedText {
            anchor: layout.corner,
            horizontal: TextEdge::Start,
            vertical: TextEdge::End,
            upward: false,
        },
        layout.name,
        layout.font_size,
    );
    [
        Some(name),
        layout.tab.map(|(tab, _)| tab),
        supply_glyph_rect(label),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn overlaps(a: LayoutRect, b: LayoutRect) -> bool {
    a.min.x < b.max.x && b.min.x < a.max.x && a.min.y < b.max.y && b.min.y < a.max.y
}

/// What a body draws, for a text to keep off: a box it fills, or a
/// stroke from end to end.
#[derive(Debug, Clone, Copy)]
enum Ink {
    Area(LayoutRect),
    Stroke(LayoutPoint, LayoutPoint),
}

impl Ink {
    /// Whether this ink reaches into `text`'s box; a stroke inks its drawn
    /// width.
    fn meets(self, text: LayoutRect) -> bool {
        match self {
            Self::Area(area) => overlaps(text, area),
            Self::Stroke(start, end) => stroke_meets(start, end, widened(text)),
        }
    }
}

/// The ink of a placed part's drawn body, in world units: each straight run
/// of its lines, and each circle or arc as the box around it.
fn body_ink(symbol: &SymbolDocument, component: &Component) -> Vec<Ink> {
    let at = |point: Point| {
        let point = component.pos + component.transform_point(point);
        LayoutPoint::new(point.x as f32, point.y as f32)
    };
    let mut ink = Vec::new();
    for shape in &symbol.body {
        match shape {
            SymbolShape::Polyline { points, closed } => {
                let ends = points.iter().copied().map(at).collect::<Vec<_>>();
                ink.extend(ends.windows(2).map(|run| Ink::Stroke(run[0], run[1])));
                if let (true, [first, .., last]) = (*closed, ends.as_slice()) {
                    ink.push(Ink::Stroke(*last, *first));
                }
            }
            SymbolShape::Circle { center, radius }
            | SymbolShape::Dot { center, radius }
            | SymbolShape::Arc { center, radius, .. } => {
                let (center, radius) = (at(*center), *radius as f32);
                ink.push(Ink::Area(LayoutRect {
                    min: LayoutPoint::new(center.x - radius, center.y - radius),
                    max: LayoutPoint::new(center.x + radius, center.y + radius),
                }));
            }
            SymbolShape::Curve(curve) => {
                let ends = curve
                    .placed(
                        component.pos + component.transform_point(Point::origin()),
                        component.transform_point(Point::new(1, 0))
                            - component.transform_point(Point::origin()),
                        component.transform_point(Point::new(0, 1))
                            - component.transform_point(Point::origin()),
                    )
                    .tessellated(1.0)
                    .into_iter()
                    .map(|[x, y]| LayoutPoint::new(x as f32, y as f32))
                    .collect::<Vec<_>>();
                ink.extend(ends.windows(2).map(|run| Ink::Stroke(run[0], run[1])));
            }
            SymbolShape::Arrow { .. } | SymbolShape::Text { .. } => {}
        }
    }
    ink
}

/// `text`'s box grown by a stroke's half width on every side: a stroke's
/// line within it inks the text.
fn widened(text: LayoutRect) -> LayoutRect {
    LayoutRect {
        min: LayoutPoint::new(
            text.min.x - STROKE_HALF_WIDTH,
            text.min.y - STROKE_HALF_WIDTH,
        ),
        max: LayoutPoint::new(
            text.max.x + STROKE_HALF_WIDTH,
            text.max.y + STROKE_HALF_WIDTH,
        ),
    }
}

/// Whether the stroke from `start` to `end` passes through `text`'s box:
/// the stroke clipped to the box, edge by edge, leaves something.
fn stroke_meets(start: LayoutPoint, end: LayoutPoint, text: LayoutRect) -> bool {
    let (across, down) = (end.x - start.x, end.y - start.y);
    let (mut from, mut to) = (0.0_f32, 1.0_f32);
    for (step, room) in [
        (-across, start.x - text.min.x),
        (across, text.max.x - start.x),
        (-down, start.y - text.min.y),
        (down, text.max.y - start.y),
    ] {
        if step == 0.0 {
            if room <= 0.0 {
                return false;
            }
            continue;
        }
        let at = room / step;
        if step < 0.0 {
            from = from.max(at);
        } else {
            to = to.min(at);
        }
        if from >= to {
            return false;
        }
    }
    true
}

/// The box a note's words take with its anchor at `pos`.
fn note_box(note: &DesignNote, pos: Point) -> LayoutRect {
    let (width, height) = note.text_extent_world(&note.text);
    let (offset_x, offset_y) = DESIGN_NOTE_TEXT_OFFSET_WORLD;
    let (x, y) = (pos.x as f32 + offset_x, pos.y as f32 + offset_y);
    LayoutRect {
        min: LayoutPoint::new(x, y),
        max: LayoutPoint::new(x + width, y + height),
    }
}

/// Places for pins that must touch nothing, near the middle of the drawn
/// pins: each half a grid step off the grid on both axes, where no wire of a
/// drawing on its tool's grid can run, off every drawn pin, and
/// [`LABEL_CLEARANCE`] clear of the symbol's own `lettering`, where the
/// terminal's mark would stand on the text.
pub(super) fn unconnected_pin_points<'a>(
    drawn: &'a [Point],
    lettering: &'a [LayoutRect],
) -> impl Iterator<Item = Point> + 'a {
    let middle = |values: Vec<i32>| {
        let (low, high) = (
            values.iter().copied().min().unwrap_or_default(),
            values.iter().copied().max().unwrap_or_default(),
        );
        ((low + high) / 2).div_euclid(GRID) * GRID + GRID / 2
    };
    let x = middle(drawn.iter().map(|point| point.x).collect());
    let y = middle(drawn.iter().map(|point| point.y).collect());
    let lettered = |point: &Point| {
        let (x, y) = (point.x as f32, point.y as f32);
        lettering.iter().any(|text| {
            x > text.min.x - LABEL_CLEARANCE
                && x < text.max.x + LABEL_CLEARANCE
                && y > text.min.y - LABEL_CLEARANCE
                && y < text.max.y + LABEL_CLEARANCE
        })
    };
    (0..)
        .map(move |step| Point::new(x + GRID * step, y))
        .filter(move |point| !drawn.contains(point) && !lettered(point))
}
