//! Operating-point badges (design D1 section 8): plain text in `info` over a
//! knockout of the canvas, placed in world space beside what each one
//! annotates and decluttered greedily by priority, so a badge covers nothing
//! the canvas draws, stands where it can only be read as its own subject's,
//! and the kept set does not change as the view pans.

use super::super::{
    design_notes,
    design_view::{DesignView, design_note_visible},
    instance_text, net_labels,
    symbol_context::SchematicSymbolContext,
    viewport::Viewport,
};
use egui::{Painter, Pos2, Rect, Vec2};
use rspice_design::schematic::{
    bus_route::BUS_STRAND_OFFSET_WORLD,
    component::Component,
    design_note::{DesignNote, DesignNoteKind},
    drawing_metrics::{BUS_STRAND_WIDTH_WORLD, JUNCTION_RADIUS_WORLD, WIRE_WIDTH_WORLD},
    label_layout::{self, LayoutPoint, LayoutRect, ValueClass},
    label_position::InstanceText,
    net_label::NetLabel,
    value_label::value_pieces,
    wire::Wire,
};
use rspice_design_model::Point;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::sync::{Arc, Mutex, PoisonError};

/// What a badge annotates.
#[derive(Debug, Clone, PartialEq)]
pub enum BadgeSubject {
    /// A net at its representative point, with the net label on the active
    /// sheet that names it, if any, and the wires it owns there. A named
    /// net's value sits by its own name; anywhere else a badge names its
    /// net, and it may stand along any of the net's wires.
    Net {
        anchor: Point,
        label: Option<u64>,
        wires: Vec<u64>,
    },
    /// A device, by instance: its quantities form one block.
    Device { component: u64 },
}

/// One badge the host resolved from a run.
#[derive(Debug, Clone, PartialEq)]
pub struct OperatingPointCanvasAnnotation {
    pub subject: BadgeSubject,
    /// One line per quantity, in the host's order; at most four are drawn.
    pub lines: Vec<String>,
    /// Declutter priority: 0 for the selection or the hovered net or device
    /// (never dropped), 1 for nets with an authored name, 2 for device
    /// blocks, 3 for unnamed nets.
    pub class: u8,
    /// The stable order within a class: the net name, then the refdes.
    pub key: String,
    /// The selected device's current, lettered in Mono Medium.
    pub selected_current: bool,
}

/// Badge text in world units, under a 13-pt ceiling.
const BADGE_WORLD: f32 = 9.0;
const BADGE_CEILING_PT: f32 = 13.0;
const MAX_LINES: usize = 4;
/// The knockout stands this far outside the text, in points.
const KNOCKOUT_OUTSET: Vec2 = Vec2::new(1.5, 0.5);
const KNOCKOUT_ALPHA: f32 = 0.80;
/// A candidate is rejected when it comes this close to anything, in points.
const CLEARANCE_PT: f32 = 2.0;
/// Gaps between a badge and its subject, in world units.
const LABEL_GAP_WU: f32 = 3.0;
const CORNER_OFFSET_WU: f32 = 4.0;
const BODY_GAP_WU: f32 = 4.0;
/// How far a wire and a bus reach either side of their centrelines, in
/// world units: a bus draws a strand either side of its centreline.
const WIRE_REACH_WU: f32 = WIRE_WIDTH_WORLD * 0.5;
const BUS_REACH_WU: f32 = BUS_STRAND_OFFSET_WORLD + BUS_STRAND_WIDTH_WORLD * 0.5;
/// A named net's first candidates stand by its label (after it, before it,
/// above it) and letter its value alone; the rest name the net.
const BESIDE_LABEL_SPOTS: usize = 3;

/// Whose geometry an obstacle is. A badge may stand against its own
/// subject's, closer than the clearance, but never on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObstacleOwner {
    Label(u64),
    Instance(u64),
    Wire(u64),
}

/// What an obstacle covers, in world units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ObstacleShape {
    /// A box: a body, a text.
    Box(Rect),
    /// A stroke from one point to another, reaching `half_width` either side.
    Stroke {
        from: Pos2,
        to: Pos2,
        half_width: f32,
    },
}

impl ObstacleShape {
    /// The box the shape stands in.
    pub fn bounds(&self) -> Rect {
        match *self {
            Self::Box(rect) => rect,
            Self::Stroke {
                from,
                to,
                half_width,
            } => Rect::from_two_pos(from, to).expand(half_width),
        }
    }

    /// Whether the shape meets `rect` at any point, or, when `strict`, over
    /// some area: a badge may abut its own subject's box. A stroke's reach
    /// is no edge to abut.
    fn meets(&self, rect: Rect, strict: bool) -> bool {
        match *self {
            Self::Box(own) if strict => {
                own.min.x < rect.max.x
                    && rect.min.x < own.max.x
                    && own.min.y < rect.max.y
                    && rect.min.y < own.max.y
            }
            Self::Box(own) => own.intersects(rect),
            Self::Stroke {
                from,
                to,
                half_width,
            } => segment_meets(from, to, rect.expand(half_width)),
        }
    }

    /// How much of `rect` the shape covers, to rank spots that are all
    /// blocked.
    fn covered(&self, rect: Rect) -> f32 {
        let shared = self.bounds().intersect(rect);
        if shared.is_positive() && self.meets(rect, false) {
            shared.area()
        } else {
            0.0
        }
    }
}

/// Whether the segment from `from` to `to` meets `rect`: the segment
/// clipped to the box (Liang and Barsky) is not empty.
fn segment_meets(from: Pos2, to: Pos2, rect: Rect) -> bool {
    let delta = to - from;
    let (mut enter, mut leave) = (0.0_f32, 1.0_f32);
    for (toward, room) in [
        (-delta.x, from.x - rect.min.x),
        (delta.x, rect.max.x - from.x),
        (-delta.y, from.y - rect.min.y),
        (delta.y, rect.max.y - from.y),
    ] {
        if toward == 0.0 {
            if room < 0.0 {
                return false;
            }
        } else if toward < 0.0 {
            enter = enter.max(room / toward);
        } else {
            leave = leave.min(room / toward);
        }
    }
    enter <= leave
}

/// Something a badge must not cover, in world units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Obstacle {
    pub owner: Option<ObstacleOwner>,
    pub shape: ObstacleShape,
}

/// One badge to place: its priority, stable key, candidate rects in order
/// of preference, and its subject's own geometry, which it may stand
/// against but not on.
#[derive(Debug, Clone, PartialEq)]
pub struct BadgeLayoutInput {
    pub class: u8,
    pub key: String,
    pub candidates: Vec<Rect>,
    pub own: Vec<ObstacleOwner>,
}

/// Where a badge landed: the index of its input, which of its candidates,
/// and its rect.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlacedBadge {
    pub index: usize,
    pub candidate: usize,
    pub rect: Rect,
}

/// Rects bucketed by a uniform grid, so a candidate is tested against what
/// stands near it rather than against everything on the sheet. A rect is
/// listed in every cell it reaches, so a query may meet it more than once.
struct Grid {
    cell: f32,
    buckets: HashMap<(i32, i32), Vec<usize>>,
}

impl Grid {
    fn new(cell: f32) -> Self {
        Self {
            cell,
            buckets: HashMap::new(),
        }
    }

    fn span(&self, rect: Rect) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
        let at = |value: f32| (value / self.cell).floor() as i32;
        (
            at(rect.min.x)..=at(rect.max.x),
            at(rect.min.y)..=at(rect.max.y),
        )
    }

    /// List `index` under `rect`; a rect that is not finite stands nowhere.
    fn insert(&mut self, rect: Rect, index: usize) {
        if !rect.is_finite() {
            return;
        }
        let (columns, rows) = self.span(rect);
        for row in rows {
            for column in columns.clone() {
                self.buckets.entry((column, row)).or_default().push(index);
            }
        }
    }

    /// Everything listed in the cells `rect` reaches (a finite rect).
    fn near(&self, rect: Rect) -> impl Iterator<Item = usize> + '_ {
        let (columns, rows) = self.span(rect);
        rows.flat_map(move |row| columns.clone().map(move |column| (column, row)))
            .filter_map(|cell| self.buckets.get(&cell))
            .flatten()
            .copied()
    }
}

/// A grid cell as wide as the middle obstacle is long, so most obstacles
/// stand in a cell or two and a spot meets a handful of them.
fn grid_cell(obstacles: &[Obstacle]) -> f32 {
    let mut extents: Vec<f32> = obstacles
        .iter()
        .map(|obstacle| {
            let bounds = obstacle.shape.bounds();
            bounds.width().max(bounds.height())
        })
        .filter(|extent| extent.is_finite())
        .collect();
    if extents.is_empty() {
        return 1.0;
    }
    let middle = extents.len() / 2;
    let (_, median, _) = extents.select_nth_unstable_by(middle, f32::total_cmp);
    // One world unit is the drawing's own resolution: no cell is finer.
    median.max(1.0)
}

/// Place badges greedily: by class, then key, each takes its first
/// candidate that, grown by `clearance`, meets no obstacle and no badge
/// already placed, and that does not lie on its own subject's geometry. A
/// badge of class 1 to 3 with no clear candidate is dropped; a class-0
/// badge never is: it takes the candidate in `view` that overlaps least, and
/// draws over what it covers.
pub fn layout_badges(
    badges: &[BadgeLayoutInput],
    obstacles: &[Obstacle],
    clearance: f32,
    view: Rect,
) -> Vec<PlacedBadge> {
    layout_badges_reading_view(badges, obstacles, clearance, view).0
}

/// [`layout_badges`], and whether its outcome read `view` at all: only a
/// class-0 badge with no clear candidate consults it, and whether a
/// candidate is clear does not depend on the view.
fn layout_badges_reading_view(
    badges: &[BadgeLayoutInput],
    obstacles: &[Obstacle],
    clearance: f32,
    view: Rect,
) -> (Vec<PlacedBadge>, bool) {
    let mut read_view = false;
    let mut order: Vec<usize> = (0..badges.len()).collect();
    order.sort_by(|&left, &right| {
        (badges[left].class, &badges[left].key, left).cmp(&(
            badges[right].class,
            &badges[right].key,
            right,
        ))
    });
    let cell = grid_cell(obstacles);
    let mut fixed = Grid::new(cell);
    for (index, obstacle) in obstacles.iter().enumerate() {
        fixed.insert(obstacle.shape.bounds(), index);
    }
    let mut taken = Grid::new(cell);
    let mut placed: Vec<PlacedBadge> = Vec::new();
    for index in order {
        let badge = &badges[index];
        let mut own = badge.own.clone();
        own.sort_unstable();
        let is_own = |owner: Option<ObstacleOwner>| {
            owner.is_some_and(|owner| own.binary_search(&owner).is_ok())
        };
        // Its own subject blocks a spot only by lying under it; anything
        // else blocks it by coming within the clearance.
        let blocks = |obstacle: &Obstacle, rect: Rect, grown: Rect| {
            if is_own(obstacle.owner) {
                obstacle.shape.meets(rect, true)
            } else {
                obstacle.shape.meets(grown, false)
            }
        };
        let clear = |rect: Rect| {
            let grown = rect.expand(clearance);
            grown.is_finite()
                && !fixed
                    .near(grown)
                    .any(|at| blocks(&obstacles[at], rect, grown))
                && !taken
                    .near(grown)
                    .any(|at| placed[at].rect.intersects(grown))
        };
        let chosen = badge
            .candidates
            .iter()
            .copied()
            .enumerate()
            .find(|&(_, rect)| clear(rect));
        let chosen = chosen.or_else(|| {
            (badge.class == 0)
                .then(|| {
                    read_view = true;
                    let overlap = |rect: Rect| -> f32 {
                        let grown = rect.expand(clearance);
                        let fixed: f32 = obstacles
                            .iter()
                            .map(|obstacle| {
                                if is_own(obstacle.owner) {
                                    obstacle.shape.covered(rect)
                                } else {
                                    obstacle.shape.covered(grown)
                                }
                            })
                            .sum();
                        let taken: f32 = placed
                            .iter()
                            .map(|placed| ObstacleShape::Box(placed.rect).covered(grown))
                            .sum();
                        fixed + taken
                    };
                    badge
                        .candidates
                        .iter()
                        .copied()
                        .enumerate()
                        .filter(|(_, rect)| rect.intersects(view))
                        .min_by(|(_, left), (_, right)| overlap(*left).total_cmp(&overlap(*right)))
                        .or_else(|| badge.candidates.first().map(|rect| (0, *rect)))
                })
                .flatten()
        });
        if let Some((candidate, rect)) = chosen {
            taken.insert(rect, placed.len());
            placed.push(PlacedBadge {
                index,
                candidate,
                rect,
            });
        }
    }
    (placed, read_view)
}

/// Everything [`layout_badges`] reads, held by content: a badge list, an
/// obstacle or a clearance that differs in any way is a different layout.
/// The view is held only when the layout read it.
#[derive(Debug, Clone, PartialEq)]
struct BadgeLayoutKey {
    badges: Vec<BadgeLayoutInput>,
    obstacles: Vec<Obstacle>,
    clearance: f32,
    view: Option<Rect>,
}

/// The last badge layout and what it was placed from, so a steady frame
/// reuses it. Placing tests every candidate against what stands near it;
/// recognising the same inputs is linear.
///
/// Behind a lock because the canvas paints through shared borrows. A clone
/// holds nothing, so a copied session never paints another's layout.
#[derive(Debug, Default)]
pub struct BadgeLayoutMemo {
    held: Mutex<Option<(BadgeLayoutKey, Arc<Vec<PlacedBadge>>)>>,
}

impl Clone for BadgeLayoutMemo {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl BadgeLayoutMemo {
    /// The layout of these inputs: the held one if they are the inputs it
    /// was placed from, else placed afresh and held.
    pub fn layout(
        &self,
        badges: Vec<BadgeLayoutInput>,
        obstacles: Vec<Obstacle>,
        clearance: f32,
        view: Rect,
    ) -> Arc<Vec<PlacedBadge>> {
        let mut held = self.held.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some((key, placed)) = held.as_ref()
            && key.clearance == clearance
            && key.view.is_none_or(|held_view| held_view == view)
            && key.badges == badges
            && key.obstacles == obstacles
        {
            return Arc::clone(placed);
        }
        let (placed, read_view) = layout_badges_reading_view(&badges, &obstacles, clearance, view);
        let placed = Arc::new(placed);
        let key = BadgeLayoutKey {
            badges,
            obstacles,
            clearance,
            view: read_view.then_some(view),
        };
        *held = Some((key, Arc::clone(&placed)));
        placed
    }
}

fn world_pos(point: Point) -> Pos2 {
    Pos2::new(point.x as f32, point.y as f32)
}

fn world_rect((min, max): (Point, Point)) -> Rect {
    Rect::from_min_max(world_pos(min), world_pos(max))
}

fn layout_rect(rect: LayoutRect) -> Rect {
    Rect::from_min_max(
        Pos2::new(rect.min.x, rect.min.y),
        Pos2::new(rect.max.x, rect.max.y),
    )
}

/// The world boxes an instance's name and value take as the canvas letters
/// them: a box a text, or a box a piece of a value an imported drawing
/// lettered in several.
fn lettering_boxes(
    component: &Component,
    view: &DesignView<'_>,
    symbols: &SchematicSymbolContext,
) -> Vec<Rect> {
    let labels = view.parameter_labels;
    let mut boxes: Vec<Rect> = [InstanceText::Name, InstanceText::Value]
        .into_iter()
        .filter_map(|text| instance_text::lettered_box(component, symbols, labels, text))
        .map(|(rect, _)| layout_rect(rect))
        .collect();
    if let Some(pieces) = value_pieces(component) {
        let shown = component.display_mode.show_value(labels)
            && match instance_text::lettering_symbol(component, symbols) {
                Some(symbol) => !symbol.document().value_hidden,
                None => label_layout::instance_value_class(component) == ValueClass::Value,
            };
        if shown {
            let origin = LayoutPoint::new(component.pos.x as f32, component.pos.y as f32);
            boxes.extend(
                pieces
                    .iter()
                    .filter(|piece| !piece.text.trim().is_empty())
                    .filter_map(|piece| {
                        let placed = piece.position.placed_text(origin, 1.0)?;
                        Some(layout_rect(label_layout::text_box(
                            placed,
                            &piece.text,
                            piece.size_world(),
                        )))
                    }),
            );
        }
    }
    boxes
}

/// The box a note's text takes. A property display's fields are read only
/// when it is drawn, so its box is its source text's, not the widest its
/// fields could letter.
fn note_bounds(note: &DesignNote) -> Rect {
    world_rect(match note.kind {
        DesignNoteKind::PropertyDisplay => design_notes::world_bounds(note, &note.text),
        _ => design_notes::conservative_world_bounds(note),
    })
}

/// A badge's knockout in world units: as the host lettered it, and, for a
/// net with a label, with the label's name before its value, for the spots
/// away from the label where the value alone would not say which net it is.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BadgeSize {
    pub lettered: Vec2,
    pub named: Option<Vec2>,
}

/// The drawing a placement reads, indexed once per placement.
struct Drawing<'a> {
    view: &'a DesignView<'a>,
    symbols: &'a SchematicSymbolContext,
    labels: HashMap<u64, &'a NetLabel>,
    wires: HashMap<u64, &'a Wire>,
    components: HashMap<u64, &'a Component>,
}

impl<'a> Drawing<'a> {
    fn new(view: &'a DesignView<'a>, symbols: &'a SchematicSymbolContext) -> Self {
        let document = view.document;
        Self {
            view,
            symbols,
            labels: document
                .net_labels
                .iter()
                .map(|label| (label.id, label))
                .collect(),
            wires: document.wires.iter().map(|wire| (wire.id, wire)).collect(),
            components: document
                .components
                .iter()
                .map(|component| (component.id, component))
                .collect(),
        }
    }
}

/// Spots along a wire segment, the label gap clear of it: above and below a
/// run, right and left of a rise, each at its start, middle and end, or at
/// its middle only where the segment is shorter than the badge.
fn along_segment(from: Point, to: Point, size: Vec2) -> Vec<Rect> {
    let (from, to) = (world_pos(from), world_pos(to));
    let at = |x: f32, y: f32| Rect::from_min_size(Pos2::new(x, y), size);
    // Where a badge `extent` long stands along a span from `low` to `high`.
    let stations = |low: f32, high: f32, extent: f32| {
        let middle = (low + high - extent) * 0.5;
        if high - low < extent + 2.0 * CORNER_OFFSET_WU {
            vec![middle]
        } else {
            vec![
                low + CORNER_OFFSET_WU,
                middle,
                high - CORNER_OFFSET_WU - extent,
            ]
        }
    };
    if from.y == to.y {
        let xs = stations(from.x.min(to.x), from.x.max(to.x), size.x);
        [from.y - LABEL_GAP_WU - size.y, from.y + LABEL_GAP_WU]
            .into_iter()
            .flat_map(|y| xs.iter().map(move |&x| at(x, y)))
            .collect()
    } else if from.x == to.x {
        let ys = stations(from.y.min(to.y), from.y.max(to.y), size.y);
        [from.x + LABEL_GAP_WU, from.x - LABEL_GAP_WU - size.x]
            .into_iter()
            .flat_map(|x| ys.iter().map(move |&y| at(x, y)))
            .collect()
    } else {
        Vec::new()
    }
}

/// The candidates for a badge of world `size` on `subject`, in order.
fn candidates(subject: &BadgeSubject, size: BadgeSize, drawing: &Drawing<'_>) -> Vec<Rect> {
    let at = |min: Pos2, size: Vec2| Rect::from_min_size(min, size);
    match subject {
        BadgeSubject::Net {
            anchor,
            label,
            wires,
        } => {
            let anchor = world_pos(*anchor);
            let mut spots = Vec::new();
            let label = label.and_then(|id| drawing.labels.get(&id));
            let away = match (label, size.named) {
                (Some(label), Some(named)) => {
                    // The label's bounds reach past a connector's tab and
                    // down to the wire it hangs from, so its text's middle
                    // is read off the text, not the bounds.
                    let bounds = world_rect(net_labels::world_bounds(label));
                    let text = label_layout::net_label_layout(label);
                    let middle = text.corner.y - text.font_size * label_layout::LINE_BOX_EM * 0.5;
                    let beside = size.lettered;
                    spots.extend([
                        at(
                            Pos2::new(bounds.max.x + LABEL_GAP_WU, middle - beside.y * 0.5),
                            beside,
                        ),
                        at(
                            Pos2::new(
                                bounds.min.x - LABEL_GAP_WU - beside.x,
                                middle - beside.y * 0.5,
                            ),
                            beside,
                        ),
                        at(
                            Pos2::new(bounds.min.x, bounds.min.y - LABEL_GAP_WU - beside.y),
                            beside,
                        ),
                    ]);
                    named
                }
                _ => size.lettered,
            };
            spots.extend([
                at(
                    Pos2::new(
                        anchor.x + CORNER_OFFSET_WU,
                        anchor.y - CORNER_OFFSET_WU - away.y,
                    ),
                    away,
                ),
                at(
                    Pos2::new(anchor.x + CORNER_OFFSET_WU, anchor.y + CORNER_OFFSET_WU),
                    away,
                ),
                at(
                    Pos2::new(
                        anchor.x - CORNER_OFFSET_WU - away.x,
                        anchor.y - CORNER_OFFSET_WU - away.y,
                    ),
                    away,
                ),
                at(
                    Pos2::new(
                        anchor.x - CORNER_OFFSET_WU - away.x,
                        anchor.y + CORNER_OFFSET_WU,
                    ),
                    away,
                ),
            ]);
            // Then along the net's own wires, nearest the anchor first: a
            // badge beside a wire reads as that wire's.
            let mut along: Vec<Rect> = wires
                .iter()
                .filter_map(|id| drawing.wires.get(id))
                .flat_map(|wire| wire.points.windows(2))
                .flat_map(|pair| along_segment(pair[0], pair[1], away))
                .collect();
            along.sort_by(|left, right| {
                left.center()
                    .distance_sq(anchor)
                    .total_cmp(&right.center().distance_sq(anchor))
            });
            spots.extend(along);
            spots
        }
        BadgeSubject::Device { component } => {
            let Some(component) = drawing.components.get(component) else {
                return Vec::new();
            };
            let size = size.lettered;
            let body = world_rect(drawing.symbols.component_bounds(component));
            let mut spots = Vec::with_capacity(5);
            // Under the instance's own name and value when they stand beside
            // the body, aligned with them.
            let lettering = lettering_boxes(component, drawing.view, drawing.symbols)
                .into_iter()
                .reduce(|left, right| left.union(right));
            if let Some(lettering) = lettering {
                if lettering.min.x >= body.max.x {
                    spots.push(at(Pos2::new(lettering.min.x, lettering.max.y), size));
                } else if lettering.max.x <= body.min.x {
                    spots.push(at(
                        Pos2::new(lettering.max.x - size.x, lettering.max.y),
                        size,
                    ));
                }
            }
            spots.extend([
                at(Pos2::new(body.max.x + BODY_GAP_WU, body.min.y), size),
                at(
                    Pos2::new(body.min.x - BODY_GAP_WU - size.x, body.min.y),
                    size,
                ),
                at(Pos2::new(body.min.x, body.max.y + BODY_GAP_WU), size),
                at(
                    Pos2::new(body.min.x, body.min.y - BODY_GAP_WU - size.y),
                    size,
                ),
            ]);
            spots
        }
    }
}

/// What a badge must not cover on the active sheet, in world units: every
/// net label, instance body, name and value, wire, junction dot, bus, note
/// and probe flag the canvas draws there.
fn obstacles(view: &DesignView<'_>, symbols: &SchematicSymbolContext) -> Vec<Obstacle> {
    let document = view.document;
    let mut obstacles = Vec::new();
    let mut add = |owner: Option<ObstacleOwner>, shape: ObstacleShape| {
        obstacles.push(Obstacle { owner, shape });
    };
    let stroke = |pair: &[Point], half_width: f32| ObstacleShape::Stroke {
        from: world_pos(pair[0]),
        to: world_pos(pair[1]),
        half_width,
    };
    for label in document
        .net_labels
        .iter()
        .filter(|label| view.object_is_visible(label.id))
    {
        add(
            Some(ObstacleOwner::Label(label.id)),
            ObstacleShape::Box(world_rect(net_labels::world_bounds(label))),
        );
    }
    for component in document
        .components
        .iter()
        .filter(|component| view.object_is_visible(component.id))
    {
        let owner = Some(ObstacleOwner::Instance(component.id));
        add(
            owner,
            ObstacleShape::Box(world_rect(symbols.component_bounds(component))),
        );
        for rect in lettering_boxes(component, view, symbols) {
            add(owner, ObstacleShape::Box(rect));
        }
    }
    // A junction dot belongs to the wires that meet at it, which are all on
    // one net.
    let mut wire_at = HashMap::new();
    for wire in document
        .wires
        .iter()
        .filter(|wire| view.object_is_visible(wire.id))
    {
        let owner = Some(ObstacleOwner::Wire(wire.id));
        for &point in &wire.points {
            wire_at.entry(point).or_insert(wire.id);
        }
        for pair in wire.points.windows(2) {
            add(owner, stroke(pair, WIRE_REACH_WU));
        }
    }
    for junction in document
        .junctions
        .iter()
        .filter(|junction| view.object_is_visible(junction.id))
    {
        add(
            wire_at.get(&junction.pos).copied().map(ObstacleOwner::Wire),
            ObstacleShape::Box(Rect::from_center_size(
                world_pos(junction.pos),
                Vec2::splat(2.0 * JUNCTION_RADIUS_WORLD),
            )),
        );
    }
    for bus in document
        .buses
        .iter()
        .filter(|bus| view.object_is_visible(bus.id))
    {
        for pair in bus.points.windows(2) {
            add(None, stroke(pair, BUS_REACH_WU));
        }
    }
    for note in document.design_notes.iter().filter(|note| {
        view.object_is_visible(note.id) && design_note_visible(note, view.review_markers)
    }) {
        add(None, ObstacleShape::Box(note_bounds(note)));
    }
    for probe in document
        .probes
        .iter()
        .filter(|probe| view.object_is_visible(probe.id))
    {
        add(None, ObstacleShape::Box(world_rect(probe.world_bounds())));
    }
    obstacles
}

/// The subject's own geometry, which its badge may stand against.
fn own(subject: &BadgeSubject) -> Vec<ObstacleOwner> {
    match subject {
        BadgeSubject::Net { label, wires, .. } => label
            .map(ObstacleOwner::Label)
            .into_iter()
            .chain(wires.iter().copied().map(ObstacleOwner::Wire))
            .collect(),
        BadgeSubject::Device { component } => vec![ObstacleOwner::Instance(*component)],
    }
}

fn world_to_screen(viewport: &Viewport, world: Pos2) -> Pos2 {
    viewport.bounds.min + viewport.offset.to_vec2() + world.to_vec2() * viewport.zoom
}

fn screen_to_world(viewport: &Viewport, screen: Pos2) -> Pos2 {
    ((screen - viewport.bounds.min - viewport.offset.to_vec2()) / viewport.zoom.max(f32::EPSILON))
        .to_pos2()
}

/// What placing a set of badges reads besides the memo.
pub struct BadgePlacement<'a> {
    pub view: &'a DesignView<'a>,
    pub symbols: &'a SchematicSymbolContext,
    pub annotations: &'a [OperatingPointCanvasAnnotation],
    /// Each annotation's knockout sizes, in world units.
    pub sizes: &'a [BadgeSize],
    pub zoom: f32,
    /// The visible world rect.
    pub world_view: Rect,
}

/// Where each badge lands, in world units: read from `layout` when it holds
/// the layout of exactly these inputs, else placed afresh (and held there).
/// The candidates and obstacles are gathered every call, which is linear;
/// only the placement is reused.
pub fn place_badges(
    placement: &BadgePlacement<'_>,
    layout: Option<&BadgeLayoutMemo>,
) -> Arc<Vec<PlacedBadge>> {
    let BadgePlacement {
        view,
        symbols,
        annotations,
        sizes,
        zoom,
        world_view,
    } = *placement;
    let drawing = Drawing::new(view, symbols);
    let inputs: Vec<BadgeLayoutInput> = annotations
        .iter()
        .zip(sizes)
        .map(|(annotation, size)| BadgeLayoutInput {
            class: annotation.class,
            key: annotation.key.clone(),
            candidates: candidates(&annotation.subject, *size, &drawing),
            own: own(&annotation.subject),
        })
        .collect();
    let obstacles = obstacles(view, symbols);
    let clearance = CLEARANCE_PT / zoom;
    match layout {
        Some(memo) => memo.layout(inputs, obstacles, clearance, world_view),
        None => Arc::new(layout_badges(&inputs, &obstacles, clearance, world_view)),
    }
}

pub(super) fn draw_operating_point_annotations(
    painter: &Painter,
    available: Rect,
    viewport: &Viewport,
    view: &DesignView<'_>,
    symbols: &SchematicSymbolContext,
    annotations: &[OperatingPointCanvasAnnotation],
    layout: Option<&BadgeLayoutMemo>,
) {
    use rspice_ui_kit::theme::{self, FontWeight};

    if annotations.is_empty() {
        return;
    }
    let ppp = painter.pixels_per_point();
    let zoom = viewport.zoom.max(f32::EPSILON);
    // Below the annotation floor no badge and no greeking.
    let Some(size_pt) = super::super::lod::annotation_size(BADGE_WORLD, zoom, ppp) else {
        return;
    };
    let size_pt = size_pt.min(BADGE_CEILING_PT);
    let palette = rspice_ui_kit::tokens::Tokens::get(painter.ctx()).color;
    let named_by_label = annotations
        .iter()
        .any(|annotation| matches!(annotation.subject, BadgeSubject::Net { label: Some(_), .. }));
    let labels: HashMap<u64, &NetLabel> = if named_by_label {
        view.document
            .net_labels
            .iter()
            .map(|label| (label.id, label))
            .collect()
    } else {
        HashMap::new()
    };
    // Each badge as the host lettered it, and a named net's with its name
    // before its value, for a spot away from its label.
    let galleys: Vec<_> = annotations
        .iter()
        .map(|annotation| {
            let weight = if annotation.selected_current {
                FontWeight::Medium
            } else {
                FontWeight::Regular
            };
            let mut shown = annotation
                .lines
                .iter()
                .take(MAX_LINES)
                .map(|line| line.replace('\u{03bc}', "\u{00b5}"))
                .collect::<Vec<_>>();
            // A badge cut short says so rather than passing for the whole.
            if annotation.lines.len() > MAX_LINES
                && let Some(last) = shown.last_mut()
            {
                last.push_str(" \u{2026}");
            }
            let text = shown.join("\n");
            let letter = |text: &str| {
                painter.fonts_mut(|fonts| {
                    rspice_ui_kit::text::layout_no_wrap(
                        fonts,
                        text,
                        theme::mono(size_pt, weight),
                        palette.info,
                    )
                })
            };
            let named = match &annotation.subject {
                BadgeSubject::Net {
                    label: Some(id), ..
                } => labels.get(id).map(|label| {
                    letter(&format!(
                        "{} {text}",
                        label_layout::net_label_display_name(label)
                    ))
                }),
                _ => None,
            };
            (letter(&text), named)
        })
        .collect();
    let knockout = |size: Vec2| (size + 2.0 * KNOCKOUT_OUTSET) / zoom;
    let sizes: Vec<BadgeSize> = galleys
        .iter()
        .map(|(lettered, named)| BadgeSize {
            lettered: knockout(lettered.size()),
            named: named.as_ref().map(|named| knockout(named.size())),
        })
        .collect();
    let world_view = Rect::from_min_max(
        screen_to_world(viewport, available.min),
        screen_to_world(viewport, available.max),
    );
    let placed = place_badges(
        &BadgePlacement {
            view,
            symbols,
            annotations,
            sizes: &sizes,
            zoom,
            world_view,
        },
        layout,
    );
    for placed in placed.iter() {
        let min = world_to_screen(viewport, placed.rect.min);
        let galley = match &galleys[placed.index] {
            (_, Some(named)) if placed.candidate >= BESIDE_LABEL_SPOTS => named,
            (lettered, _) => lettered,
        };
        let knockout = Rect::from_min_size(min, galley.size() + 2.0 * KNOCKOUT_OUTSET);
        if !knockout.intersects(available) {
            continue;
        }
        painter.rect_filled(
            knockout,
            0.0,
            palette.canvas_bg.gamma_multiply(KNOCKOUT_ALPHA),
        );
        painter.galley(min + KNOCKOUT_OUTSET, galley.clone(), palette.info);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn badge(class: u8, key: &str, candidates: &[Rect]) -> BadgeLayoutInput {
        BadgeLayoutInput {
            class,
            key: key.to_owned(),
            candidates: candidates.to_vec(),
            own: Vec::new(),
        }
    }

    fn obstacle(rect: Rect) -> Obstacle {
        Obstacle {
            owner: None,
            shape: ObstacleShape::Box(rect),
        }
    }

    fn rect(x: f32, y: f32) -> Rect {
        Rect::from_min_size(Pos2::new(x, y), Vec2::new(20.0, 6.0))
    }

    const VIEW: Rect = Rect {
        min: Pos2::new(-1000.0, -1000.0),
        max: Pos2::new(1000.0, 1000.0),
    };

    /// Two badges that want the same spot: the higher priority keeps it and
    /// the other takes its next candidate; nothing overlaps.
    #[test]
    fn competing_badges_take_their_next_candidate() {
        let placed = layout_badges(
            &[
                badge(3, "n2", &[rect(0.0, 0.0), rect(0.0, 20.0)]),
                badge(1, "OUT", &[rect(0.0, 0.0), rect(0.0, 40.0)]),
            ],
            &[],
            1.0,
            VIEW,
        );
        assert_eq!(placed.len(), 2);
        let named = placed.iter().find(|badge| badge.index == 1).expect("named");
        let unnamed = placed
            .iter()
            .find(|badge| badge.index == 0)
            .expect("unnamed");
        assert_eq!(named.rect, rect(0.0, 0.0), "class 1 places first");
        assert_eq!(unnamed.rect, rect(0.0, 20.0));
        assert!(!named.rect.expand(1.0).intersects(unnamed.rect));
    }

    /// An obstacle (a label, a name, a body) rejects a candidate; a badge of
    /// class 1 to 3 with every candidate blocked is dropped, a class-0 badge
    /// is kept on its least-covered candidate.
    #[test]
    fn blocked_badges_drop_unless_they_annotate_the_selection() {
        let obstacles = [
            obstacle(Rect::from_min_size(
                Pos2::new(-5.0, -5.0),
                Vec2::new(30.0, 10.0),
            )),
            obstacle(Rect::from_min_size(
                Pos2::new(-5.0, 35.0),
                Vec2::new(30.0, 4.0),
            )),
        ];
        let candidates = [rect(0.0, 0.0), rect(0.0, 36.0)];
        assert!(layout_badges(&[badge(2, "R1", &candidates)], &obstacles, 1.0, VIEW).is_empty());
        let kept = layout_badges(&[badge(0, "R1", &candidates)], &obstacles, 1.0, VIEW);
        assert_eq!(kept.len(), 1);
        assert_eq!(kept[0].rect, rect(0.0, 36.0), "the least covered candidate");
    }

    /// A badge stands against its own subject (the gap to its own label or
    /// body is less than the clearance at low zoom) but not against anyone
    /// else's, and never on it.
    #[test]
    fn a_badge_is_not_blocked_by_its_own_subject() {
        let body = Rect::from_min_size(Pos2::new(-20.0, -10.0), Vec2::new(18.0, 20.0));
        let own = Obstacle {
            owner: Some(ObstacleOwner::Instance(7)),
            shape: ObstacleShape::Box(body),
        };
        let candidates = [rect(0.0, -10.0)];
        let mine = BadgeLayoutInput {
            own: vec![ObstacleOwner::Instance(7)],
            ..badge(2, "R7", &candidates)
        };
        assert_eq!(layout_badges(&[mine.clone()], &[own], 4.0, VIEW).len(), 1);
        let on_it = BadgeLayoutInput {
            candidates: vec![rect(-10.0, -10.0)],
            ..mine
        };
        assert!(layout_badges(&[on_it], &[own], 4.0, VIEW).is_empty());
        let theirs = BadgeLayoutInput {
            own: vec![ObstacleOwner::Instance(8)],
            ..badge(2, "R8", &candidates)
        };
        assert!(layout_badges(&[theirs], &[own], 4.0, VIEW).is_empty());
    }

    /// The outcome depends on the badges, not on the order the host lists
    /// them in, so the kept set does not flicker.
    #[test]
    fn layout_is_independent_of_input_order() {
        let badges = vec![
            badge(1, "A", &[rect(0.0, 0.0), rect(30.0, 0.0)]),
            badge(1, "B", &[rect(0.0, 0.0), rect(60.0, 0.0)]),
            badge(3, "c", &[rect(0.0, 0.0), rect(30.0, 0.0), rect(90.0, 0.0)]),
        ];
        let forward = layout_badges(&badges, &[], 1.0, VIEW);
        let mut reversed_input = badges.clone();
        reversed_input.reverse();
        let reversed = layout_badges(&reversed_input, &[], 1.0, VIEW);
        let rects = |placed: &[PlacedBadge], input: &[BadgeLayoutInput]| {
            let mut rects: Vec<(String, [f32; 2])> = placed
                .iter()
                .map(|badge| {
                    (
                        input[badge.index].key.clone(),
                        [badge.rect.min.x, badge.rect.min.y],
                    )
                })
                .collect();
            rects.sort_by(|left, right| left.0.cmp(&right.0));
            rects
        };
        assert_eq!(rects(&forward, &badges), rects(&reversed, &reversed_input));
        assert_eq!(forward.len(), 3);
    }

    /// The held layout is reused only for the inputs it was placed from:
    /// each input changed on its own places afresh, and the fresh layout is
    /// the one a direct placement gives. The view is an input only while a
    /// class-0 badge fell back on it.
    #[test]
    fn a_held_layout_follows_every_input() {
        type Inputs = (Vec<BadgeLayoutInput>, Vec<Obstacle>, f32, Rect);
        let base: Inputs = (
            vec![
                badge(1, "OUT", &[rect(0.0, 0.0), rect(0.0, 40.0)]),
                badge(3, "n2", &[rect(0.0, 0.0), rect(0.0, 20.0)]),
            ],
            vec![Obstacle {
                owner: Some(ObstacleOwner::Label(4)),
                shape: ObstacleShape::Box(Rect::from_min_size(
                    Pos2::new(100.0, 0.0),
                    Vec2::new(30.0, 10.0),
                )),
            }],
            1.0,
            VIEW,
        );
        type Change = (&'static str, fn(&mut Inputs));
        let changes: [Change; 9] = [
            ("class", |inputs| inputs.0[1].class = 2),
            ("key", |inputs| inputs.0[0].key = "OUTP".to_owned()),
            ("candidate", |inputs| {
                inputs.0[1].candidates[1] = rect(0.0, 24.0)
            }),
            ("owner", |inputs| {
                inputs.0[0].own = vec![ObstacleOwner::Label(4)];
            }),
            ("badge added", |inputs| {
                inputs.0.push(badge(2, "R1", &[rect(50.0, 50.0)]));
            }),
            ("obstacle rect", |inputs| {
                inputs.1[0].shape = ObstacleShape::Box(Rect::from_min_size(
                    Pos2::new(-5.0, -5.0),
                    Vec2::new(30.0, 10.0),
                ));
            }),
            ("obstacle owner", |inputs| {
                inputs.1[0].owner = Some(ObstacleOwner::Instance(4));
            }),
            ("obstacle added", |inputs| {
                inputs.1.push(obstacle(rect(0.0, 40.0)))
            }),
            ("clearance", |inputs| inputs.2 = 3.0),
        ];
        for (input, change) in changes {
            let memo = BadgeLayoutMemo::default();
            let place = |(badges, obstacles, clearance, view): &Inputs| {
                memo.layout(badges.clone(), obstacles.clone(), *clearance, *view)
            };
            let held = place(&base);
            assert!(Arc::ptr_eq(&held, &place(&base)), "{input}: steady");
            let mut changed = base.clone();
            change(&mut changed);
            let fresh = place(&changed);
            assert!(!Arc::ptr_eq(&held, &fresh), "{input}: placed afresh");
            assert_eq!(
                *fresh,
                layout_badges(&changed.0, &changed.1, changed.2, changed.3),
                "{input}"
            );
        }

        // No badge fell back on the view, so panning reuses the layout.
        let memo = BadgeLayoutMemo::default();
        let held = memo.layout(base.0.clone(), base.1.clone(), base.2, base.3);
        let panned = base.3.translate(Vec2::new(500.0, 0.0));
        assert!(Arc::ptr_eq(
            &held,
            &memo.layout(base.0.clone(), base.1.clone(), base.2, panned)
        ));

        // A blocked class-0 badge takes its least-covered candidate in view,
        // so then the view is an input like any other.
        let blocked = vec![badge(0, "R1", &[rect(0.0, 0.0), rect(400.0, 0.0)])];
        let walls = vec![obstacle(rect(0.0, 0.0)), obstacle(rect(400.0, 0.0))];
        let near = Rect::from_min_size(Pos2::new(-50.0, -50.0), Vec2::new(100.0, 100.0));
        let far = near.translate(Vec2::new(400.0, 0.0));
        let held = memo.layout(blocked.clone(), walls.clone(), 1.0, near);
        let moved = memo.layout(blocked.clone(), walls.clone(), 1.0, far);
        assert!(!Arc::ptr_eq(&held, &moved));
        assert_eq!(held[0].rect, rect(0.0, 0.0));
        assert_eq!(moved[0].rect, rect(400.0, 0.0), "the candidate in view");
    }
}
