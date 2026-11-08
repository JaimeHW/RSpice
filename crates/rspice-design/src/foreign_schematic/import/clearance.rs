//! Keeping the lettering an import places clear of the bodies RSpice draws.

use rspice_design_model::Point;

use super::Builder;
use crate::schematic::component::Component;
use crate::schematic::component_display::SchematicParameterLabelVisibility;
use crate::schematic::component_type::ComponentType;
use crate::schematic::drawing_metrics::SYMBOL_WIDTH_WORLD;
use crate::schematic::label_layout::{
    INSTANCE_NAME_WORLD, INSTANCE_VALUE_WORLD, LayoutPoint, LayoutRect, NET_LABEL_FONT_WORLD,
    PlacedText, TextEdge, instance_label_layout, net_label_layout, supply_glyph_rect, text_box,
};
use crate::schematic::label_position::{LabelPosition, ValuePiece};
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
    /// A part RSpice draws with its own artwork has a body of another size
    /// than the source symbol's, so a name or value the drawing stood
    /// beside the source body can land on RSpice's body or on a
    /// neighbour's. Such a text keeps its side and its line and moves out
    /// along the way it reads to the nearest place where it lands on no
    /// drawn body, no other part's text and no net label. A body is judged
    /// by the ink it draws, so a text standing between a part's leads
    /// stays there. A text the drawing itself set on another text stays
    /// where it was set.
    pub(super) fn clear_lettering(&mut self) {
        let bodies = self.drawn_bodies();
        let net_labels = self
            .document
            .net_labels
            .iter()
            .flat_map(net_label_boxes)
            .map(Ink::Area)
            .collect::<Vec<_>>();
        for &index in &self.lettered {
            // A cell letters its name and value at its symbol's places. Only
            // texts its drawing lettered for it give way, to a part RSpice
            // draws where the drawing drew none: the ground on a pin QSPICE
            // ties to ground itself.
            let cell = self.document.components[index].kind == ComponentType::CellInstance;
            if cell && value_pieces(&self.document.components[index]).is_none() {
                continue;
            }
            for label in [Lettering::Name, Lettering::Value] {
                if cell && label == Lettering::Name {
                    continue;
                }
                let pieces = match label {
                    Lettering::Name => 1,
                    Lettering::Value => value_pieces(&self.document.components[index])
                        .map_or(1, <[ValuePiece]>::len),
                };
                for piece in 0..pieces {
                    let components = &self.document.components;
                    let component = &components[index];
                    let Some((position, text, size)) = label.text(component, piece) else {
                        continue;
                    };
                    let origin = component.pos;
                    let Some(placed) = position
                        .placed_text(LayoutPoint::new(origin.x as f32, origin.y as f32), 1.0)
                    else {
                        continue;
                    };
                    let own = text_box(placed, &text, size);
                    // Only a body RSpice draws in the source's stead moves a
                    // text; a cell is drawn as its source drew it.
                    let lands = components.iter().zip(&bodies).any(|(part, ink)| {
                        part.kind != ComponentType::CellInstance
                            && ink.iter().any(|ink| ink.meets(own))
                    });
                    if !lands {
                        continue;
                    }
                    // A cell's texts stand on its own drawing where the
                    // drawing set them.
                    let obstacles = bodies
                        .iter()
                        .enumerate()
                        .filter(|(other, _)| !cell || *other != index)
                        .flat_map(|(_, ink)| ink.iter().copied())
                        .chain(net_labels.iter().copied())
                        .chain(components.iter().enumerate().flat_map(|(other, part)| {
                            lettering_boxes(part)
                                .into_iter()
                                .filter(move |(which, _)| {
                                    other != index || *which != (label, piece)
                                })
                                .map(|(_, rect)| Ink::Area(rect))
                        }))
                        .collect::<Vec<_>>();
                    // A text may cross its own part's body, and no further:
                    // a text with no clear place beside its part stays by
                    // it rather than wander off past other parts.
                    let body = native_body(component);
                    let reach = (
                        body.max.x - body.min.x + own.max.x - own.min.x + 2.0 * LABEL_CLEARANCE,
                        body.max.y - body.min.y + own.max.y - own.min.y + 2.0 * LABEL_CLEARANCE,
                    );
                    let cleared = clear_of(position, origin, &text, size, &obstacles, reach);
                    label.set(&mut self.document.components[index], piece, cleared);
                }
            }
        }
    }

    /// Names each net whose label may stand on another side on the first
    /// side where its name lands on no part's body. RSpice draws its own
    /// bodies, and grounds, which can reach into a name the drawing centred
    /// beside them; and a flag on the middle of a wire says nothing of the
    /// side its name reads to.
    pub(super) fn clear_net_names(&mut self) {
        // Every body is judged by the ink it draws, so a name beside a
        // slanted edge or between two leads stands clear of them.
        let bodies = self
            .drawn_bodies()
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        let lands = |label: &NetLabel| {
            net_label_boxes(label)
                .first()
                .is_some_and(|name| bodies.iter().any(|body| body.meets(*name)))
        };
        for (index, sides) in std::mem::take(&mut self.free_names) {
            let Some(label) = self.document.net_labels.get(index) else {
                continue;
            };
            if !lands(label) {
                continue;
            }
            let clear = sides.into_iter().find(|side| {
                let mut moved = label.clone();
                moved.side = *side;
                !lands(&moved)
            });
            if let Some(side) = clear {
                self.document.net_labels[index].side = side;
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

/// Which of a part's texts the importer is placing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Lettering {
    Name,
    Value,
}

impl Lettering {
    /// The text's placement, its words and its size; for a value lettered
    /// in pieces, the piece at `piece`. None for a text the part does not
    /// show or a drawing did not place.
    fn text(self, component: &Component, piece: usize) -> Option<(LabelPosition, String, f32)> {
        let shown = SchematicParameterLabelVisibility::NamesAndValues;
        let (position, text, size) = match self {
            Self::Name if component.display_mode.show_name(shown) => (
                component.name_label_pos,
                component.name.clone(),
                INSTANCE_NAME_WORLD,
            ),
            Self::Value if component.display_mode.show_value(shown) => {
                match value_pieces(component) {
                    Some(pieces) => {
                        let piece = pieces.get(piece)?;
                        (piece.position, piece.text.clone(), piece.size_world())
                    }
                    None => (
                        component.value_label_pos,
                        component_value_label(component).into_owned(),
                        INSTANCE_VALUE_WORLD,
                    ),
                }
            }
            _ => return None,
        };
        (matches!(position, LabelPosition::Placed { .. }) && !text.trim().is_empty())
            .then_some((position, text, size))
    }

    fn set(self, component: &mut Component, piece: usize, position: LabelPosition) {
        match self {
            Self::Name => component.name_label_pos = position,
            Self::Value if value_pieces(component).is_some() => {
                if let Some(piece) = component.value_pieces.get_mut(piece) {
                    piece.position = position;
                }
            }
            Self::Value => component.value_label_pos = position,
        }
    }
}

/// The boxes a part's shown name and value take where they are placed,
/// each with which text it is.
fn lettering_boxes(component: &Component) -> Vec<((Lettering, usize), LayoutRect)> {
    let origin = LayoutPoint::new(component.pos.x as f32, component.pos.y as f32);
    let layout = instance_label_layout(origin, 1.0, component);
    let shown = SchematicParameterLabelVisibility::NamesAndValues;
    let mut boxes = Vec::new();
    if component.kind == ComponentType::Ground {
        return boxes;
    }
    if component.display_mode.show_name(shown) && !component.name.trim().is_empty() {
        boxes.push((
            (Lettering::Name, 0),
            text_box(layout.name, &component.name, INSTANCE_NAME_WORLD),
        ));
    }
    if !component.display_mode.show_value(shown) {
        return boxes;
    }
    match value_pieces(component) {
        Some(pieces) => boxes.extend(pieces.iter().enumerate().filter_map(|(index, piece)| {
            let placed = piece.position.placed_text(origin, 1.0)?;
            Some((
                (Lettering::Value, index),
                text_box(placed, &piece.text, piece.size_world()),
            ))
        })),
        None => {
            let value = component_value_label(component);
            if !value.trim().is_empty() {
                boxes.push((
                    (Lettering::Value, 0),
                    text_box(layout.value, &value, INSTANCE_VALUE_WORLD),
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
        NET_LABEL_FONT_WORLD,
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

    /// How far this ink runs along the axis `direction` moves on, within
    /// the band `text` sweeps across it: the least and greatest coordinate
    /// of the ink a text moving that way can meet.
    fn span_along(self, direction: Point, text: LayoutRect) -> (f32, f32) {
        let across = direction.x != 0;
        let text = match self {
            Self::Area(_) => text,
            Self::Stroke(..) => widened(text),
        };
        let along_band = |point: LayoutPoint| {
            if across {
                (point.x, point.y)
            } else {
                (point.y, point.x)
            }
        };
        let (band_min, band_max) = if across {
            (text.min.y, text.max.y)
        } else {
            (text.min.x, text.max.x)
        };
        match self {
            Self::Area(area) => {
                let (low, _) = along_band(area.min);
                let (high, _) = along_band(area.max);
                (low, high)
            }
            Self::Stroke(start, end) => {
                let ((start_along, start_band), (end_along, end_band)) =
                    (along_band(start), along_band(end));
                let rise = end_band - start_band;
                let (mut from, mut to) = (0.0_f32, 1.0_f32);
                if rise != 0.0 {
                    let (enter, leave) = (
                        (band_min - start_band) / rise,
                        (band_max - start_band) / rise,
                    );
                    from = from.max(enter.min(leave));
                    to = to.min(enter.max(leave));
                }
                if from > to {
                    (from, to) = (0.0, 1.0);
                }
                let at = |t: f32| start_along + (end_along - start_along) * t;
                (
                    at(from).min(at(to)) - STROKE_HALF_WIDTH,
                    at(from).max(at(to)) + STROKE_HALF_WIDTH,
                )
            }
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

/// A placed text moved along an axis its box reaches out on to the
/// nearest place where it stands [`LABEL_CLEARANCE`] clear of every
/// obstacle it would land on: the shortest such move, or the shortest of
/// all four for a text centred on its anchor. A text with no such place
/// along its line moves either way across the axis it centres on instead.
/// A text with no such place within `reach`, across and down, stays where
/// it was.
fn clear_of(
    position: LabelPosition,
    origin: Point,
    text: &str,
    size: f32,
    obstacles: &[Ink],
    reach: (f32, f32),
) -> LabelPosition {
    let LabelPosition::Placed {
        offset,
        horizontal: run_horizontal,
        vertical: run_vertical,
        upward,
    } = position
    else {
        return position;
    };
    // The edges the box reaches out on across the page and down it: a run
    // read upward reaches up from its start and right from its top.
    let (horizontal, vertical) = if upward {
        let up = match run_horizontal {
            TextEdge::Start => TextEdge::End,
            TextEdge::Center => TextEdge::Center,
            TextEdge::End => TextEdge::Start,
        };
        (run_vertical, up)
    } else {
        (run_horizontal, run_vertical)
    };
    let Some(placed) =
        position.placed_text(LayoutPoint::new(origin.x as f32, origin.y as f32), 1.0)
    else {
        return position;
    };
    let text = text_box(placed, text, size);
    let across = match horizontal {
        TextEdge::Start => vec![Point::new(1, 0)],
        TextEdge::End => vec![Point::new(-1, 0)],
        TextEdge::Center => Vec::new(),
    };
    let along = match vertical {
        TextEdge::Start => vec![Point::new(0, 1)],
        TextEdge::End => vec![Point::new(0, -1)],
        TextEdge::Center => Vec::new(),
    };
    let both_ways = |axis: Point| [axis, Point::new(-axis.x, -axis.y)];
    // Off its line: either way across an axis the text centres on.
    let off_line = (across.is_empty() != along.is_empty()).then(|| {
        [(horizontal, Point::new(1, 0)), (vertical, Point::new(0, 1))]
            .into_iter()
            .filter(|(edge, _)| *edge == TextEdge::Center)
            .flat_map(|(_, axis)| both_ways(axis))
            .collect::<Vec<_>>()
    });
    let directions = if across.is_empty() && along.is_empty() {
        both_ways(Point::new(1, 0))
            .into_iter()
            .chain(both_ways(Point::new(0, 1)))
            .collect()
    } else {
        across.into_iter().chain(along).collect::<Vec<_>>()
    };
    // How far the text must move along `direction` to stand clear of
    // `obstacle` beyond it.
    let past = |direction: Point, obstacle: &Ink| {
        let (low, high) = obstacle.span_along(direction, text);
        match (direction.x, direction.y) {
            (1, _) => (high + LABEL_CLEARANCE - text.min.x).ceil(),
            (-1, _) => (text.max.x - low + LABEL_CLEARANCE).ceil(),
            (_, 1) => (high + LABEL_CLEARANCE - text.min.y).ceil(),
            _ => (text.max.y - low + LABEL_CLEARANCE).ceil(),
        }
    };
    let moved = |direction: Point, distance: f32| LayoutRect {
        min: LayoutPoint::new(
            text.min.x + direction.x as f32 * distance,
            text.min.y + direction.y as f32 * distance,
        ),
        max: LayoutPoint::new(
            text.max.x + direction.x as f32 * distance,
            text.max.y + direction.y as f32 * distance,
        ),
    };
    let limit = |direction: Point| if direction.x == 0 { reach.1 } else { reach.0 };
    let clear_along = |direction: Point| {
        let mut distance = 0.0_f32;
        for _ in 0..=obstacles.len() {
            if distance > limit(direction) {
                return None;
            }
            let landed = moved(direction, distance);
            match obstacles.iter().find(|obstacle| obstacle.meets(landed)) {
                None => return Some(distance),
                Some(obstacle) => distance = distance.max(past(direction, obstacle)),
            }
        }
        None
    };
    let nearest = |directions: Vec<Point>| {
        directions
            .into_iter()
            .filter_map(|direction| clear_along(direction).map(|distance| (direction, distance)))
            .min_by(|a, b| a.1.total_cmp(&b.1))
    };
    // A text that finds no room along its line steps off it, keeping the
    // side it reads from.
    let Some((direction, distance)) = nearest(directions).or_else(|| off_line.and_then(nearest))
    else {
        return position;
    };
    LabelPosition::Placed {
        offset: offset + Point::new(direction.x * distance as i32, direction.y * distance as i32),
        horizontal: run_horizontal,
        vertical: run_vertical,
        upward,
    }
}
