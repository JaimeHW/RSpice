//! Operating-point badges (design D1 section 8): plain text in `info` over a
//! knockout of the canvas, placed in world space beside what each one
//! annotates and decluttered greedily by priority, so a badge never covers
//! its own subject and the kept set does not change as the view pans.

use super::super::{
    design_view::DesignView, drawing::compute_label_layout, net_labels,
    resolved_symbol_render::instance_name_anchor, symbol_context::SchematicSymbolContext,
    viewport::Viewport,
};
use egui::{Painter, Pos2, Rect, Vec2};
use rspice_design::schematic::component::Component;
use rspice_design::schematic::component_type::ComponentType;
use rspice_design_model::Point;
use std::sync::{Arc, Mutex, PoisonError};

/// What a badge annotates.
#[derive(Debug, Clone, PartialEq)]
pub enum BadgeSubject {
    /// A net at its representative point, with the net label on the active
    /// sheet that names it, if any: a named net's value sits right after its
    /// own name.
    Net { anchor: Point, label: Option<u64> },
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
/// An instance name's estimated advance and line, in world units, for the
/// obstacle it makes.
const NAME_ADVANCE_WU: f32 = 6.0;
const NAME_LINE_WU: f32 = 13.0;

/// Whose geometry an obstacle is. A badge's own subject never blocks it:
/// its candidates already stand clear of it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObstacleOwner {
    Label(u64),
    Instance(u64),
}

/// Something a badge must not cover, in world units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Obstacle {
    pub owner: Option<ObstacleOwner>,
    pub rect: Rect,
}

/// One badge to place: its priority, stable key, candidate rects in order
/// of preference, and the subject whose geometry it may stand against.
#[derive(Debug, Clone, PartialEq)]
pub struct BadgeLayoutInput {
    pub class: u8,
    pub key: String,
    pub candidates: Vec<Rect>,
    pub owner: Option<ObstacleOwner>,
}

/// Where a badge landed: the index of its input and its rect.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlacedBadge {
    pub index: usize,
    pub rect: Rect,
}

/// Place badges greedily: by class, then key, each takes its first
/// candidate that, grown by `clearance`, meets no obstacle (but its own
/// subject's) and no badge already placed. A badge of class 1 to 3 with no
/// clear candidate is dropped; a class-0 badge never is: it takes the
/// candidate in `view` that overlaps least, and draws over what it covers.
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
    let mut placed: Vec<PlacedBadge> = Vec::new();
    for index in order {
        let badge = &badges[index];
        let blockers = || {
            obstacles
                .iter()
                .filter(|obstacle| obstacle.owner.is_none() || obstacle.owner != badge.owner)
                .map(|obstacle| &obstacle.rect)
                .chain(placed.iter().map(|placed| &placed.rect))
        };
        let clear = |rect: &Rect| {
            let grown = rect.expand(clearance);
            !blockers().any(|blocker| blocker.intersects(grown))
        };
        let chosen = badge.candidates.iter().find(|rect| clear(rect)).copied();
        let chosen = chosen.or_else(|| {
            (badge.class == 0)
                .then(|| {
                    read_view = true;
                    let overlap = |rect: &Rect| -> f32 {
                        let grown = rect.expand(clearance);
                        blockers()
                            .map(|blocker| {
                                let shared = blocker.intersect(grown);
                                if shared.is_positive() {
                                    shared.area()
                                } else {
                                    0.0
                                }
                            })
                            .sum()
                    };
                    badge
                        .candidates
                        .iter()
                        .filter(|rect| rect.intersects(view))
                        .min_by(|left, right| overlap(left).total_cmp(&overlap(right)))
                        .or(badge.candidates.first())
                        .copied()
                })
                .flatten()
        });
        if let Some(rect) = chosen {
            placed.push(PlacedBadge { index, rect });
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
/// reuses it. Placing is quadratic (every badge against every obstacle and
/// every badge already placed); recognising the same inputs is linear.
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

/// The world rect an instance's name occupies, estimated from its anchor
/// and justification, as an obstacle a badge must not cover.
fn instance_name_rect(component: &Component, symbols: &SchematicSymbolContext) -> Option<Rect> {
    if component.name.is_empty() || component.kind == ComponentType::Ground {
        return None;
    }
    let (anchor, align) = match symbols.resolved_symbol(component) {
        Some(symbol) => instance_name_anchor(component, symbol),
        None => {
            let origin = Pos2::new(component.pos.x as f32, component.pos.y as f32);
            let layout = compute_label_layout(origin, 1.0, component);
            (layout.name_pos, layout.name_align)
        }
    };
    let size = Vec2::new(
        component.name.chars().count() as f32 * NAME_ADVANCE_WU,
        NAME_LINE_WU,
    );
    Some(align.anchor_size(anchor, size))
}

fn world_rect((min, max): (Point, Point)) -> Rect {
    Rect::from_min_max(
        Pos2::new(min.x as f32, min.y as f32),
        Pos2::new(max.x as f32, max.y as f32),
    )
}

/// The candidates for a badge of world `size` on `subject`, in order.
fn candidates(
    subject: &BadgeSubject,
    size: Vec2,
    view: &DesignView<'_>,
    symbols: &SchematicSymbolContext,
) -> Vec<Rect> {
    let at = |min: Pos2| Rect::from_min_size(min, size);
    match subject {
        BadgeSubject::Net { anchor, label } => {
            let anchor = Pos2::new(anchor.x as f32, anchor.y as f32);
            let label = label.and_then(|id| {
                view.document
                    .net_labels
                    .iter()
                    .find(|label| label.id == id)
                    .map(|label| world_rect(net_labels::world_bounds(label)))
            });
            let first = match label {
                Some(label) => at(Pos2::new(
                    label.max.x + LABEL_GAP_WU,
                    label.center().y - size.y * 0.5,
                )),
                None => at(Pos2::new(
                    anchor.x + CORNER_OFFSET_WU,
                    anchor.y - CORNER_OFFSET_WU - size.y,
                )),
            };
            vec![
                first,
                at(Pos2::new(
                    anchor.x + CORNER_OFFSET_WU,
                    anchor.y + CORNER_OFFSET_WU,
                )),
                at(Pos2::new(
                    anchor.x - CORNER_OFFSET_WU - size.x,
                    anchor.y - CORNER_OFFSET_WU - size.y,
                )),
                at(Pos2::new(
                    anchor.x - CORNER_OFFSET_WU - size.x,
                    anchor.y + CORNER_OFFSET_WU,
                )),
            ]
        }
        BadgeSubject::Device { component } => {
            let Some(component) = view
                .document
                .components
                .iter()
                .find(|item| item.id == *component)
            else {
                return Vec::new();
            };
            let body = world_rect(symbols.component_bounds(component));
            let mut candidates = Vec::with_capacity(5);
            // Under the instance's own name when the name sits beside the
            // body, aligned with it.
            if let Some(name) = instance_name_rect(component, symbols) {
                if name.min.x >= body.max.x {
                    candidates.push(at(Pos2::new(name.min.x, name.max.y)));
                } else if name.max.x <= body.min.x {
                    candidates.push(at(Pos2::new(name.max.x - size.x, name.max.y)));
                }
            }
            candidates.extend([
                at(Pos2::new(body.max.x + BODY_GAP_WU, body.min.y)),
                at(Pos2::new(body.min.x - BODY_GAP_WU - size.x, body.min.y)),
                at(Pos2::new(body.min.x, body.max.y + BODY_GAP_WU)),
                at(Pos2::new(body.min.x, body.min.y - BODY_GAP_WU - size.y)),
            ]);
            candidates
        }
    }
}

/// What a badge must not cover: every net-label text, instance name and
/// instance body on the active sheet, in world units.
fn obstacles(view: &DesignView<'_>, symbols: &SchematicSymbolContext) -> Vec<Obstacle> {
    let document = view.document;
    let mut obstacles = Vec::new();
    for label in document
        .net_labels
        .iter()
        .filter(|label| view.object_is_visible(label.id))
    {
        obstacles.push(Obstacle {
            owner: Some(ObstacleOwner::Label(label.id)),
            rect: world_rect(net_labels::world_bounds(label)),
        });
    }
    for component in document
        .components
        .iter()
        .filter(|component| view.object_is_visible(component.id))
    {
        let owner = Some(ObstacleOwner::Instance(component.id));
        obstacles.push(Obstacle {
            owner,
            rect: world_rect(symbols.component_bounds(component)),
        });
        obstacles
            .extend(instance_name_rect(component, symbols).map(|rect| Obstacle { owner, rect }));
    }
    obstacles
}

/// The subject geometry a badge may stand against.
fn owner(subject: &BadgeSubject) -> Option<ObstacleOwner> {
    match subject {
        BadgeSubject::Net { label, .. } => label.map(ObstacleOwner::Label),
        BadgeSubject::Device { component } => Some(ObstacleOwner::Instance(*component)),
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
    /// Each annotation's knockout size, in world units.
    pub sizes: &'a [Vec2],
    pub zoom: f32,
    /// The visible world rect.
    pub world_view: Rect,
}

/// Where each badge lands, in world units: read from `layout` when it holds
/// the layout of exactly these inputs, else placed afresh (and held there).
/// The candidates and obstacles are gathered every call, which is linear;
/// only the quadratic placement is reused.
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
    let inputs: Vec<BadgeLayoutInput> = annotations
        .iter()
        .zip(sizes)
        .map(|(annotation, size)| BadgeLayoutInput {
            class: annotation.class,
            key: annotation.key.clone(),
            candidates: candidates(&annotation.subject, *size, view, symbols),
            owner: owner(&annotation.subject),
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
    let palette = rspice_ui_kit::tokens::active_palette();
    let galleys: Vec<_> = annotations
        .iter()
        .map(|annotation| {
            let weight = if annotation.selected_current {
                FontWeight::Medium
            } else {
                FontWeight::Regular
            };
            let text = annotation
                .lines
                .iter()
                .take(MAX_LINES)
                .map(|line| line.replace('\u{03bc}', "\u{00b5}"))
                .collect::<Vec<_>>()
                .join("\n");
            painter.layout_no_wrap(text, theme::mono(size_pt, weight), palette.info)
        })
        .collect();
    let sizes: Vec<Vec2> = galleys
        .iter()
        .map(|galley| (galley.size() + 2.0 * KNOCKOUT_OUTSET) / zoom)
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
        let galley = &galleys[placed.index];
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
            owner: None,
        }
    }

    fn obstacle(rect: Rect) -> Obstacle {
        Obstacle { owner: None, rect }
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
    /// else's.
    #[test]
    fn a_badge_is_not_blocked_by_its_own_subject() {
        let body = Rect::from_min_size(Pos2::new(-20.0, -10.0), Vec2::new(18.0, 20.0));
        let own = Obstacle {
            owner: Some(ObstacleOwner::Instance(7)),
            rect: body,
        };
        let candidates = [rect(0.0, -10.0)];
        let mine = BadgeLayoutInput {
            owner: Some(ObstacleOwner::Instance(7)),
            ..badge(2, "R7", &candidates)
        };
        assert_eq!(layout_badges(&[mine], &[own], 4.0, VIEW).len(), 1);
        let theirs = BadgeLayoutInput {
            owner: Some(ObstacleOwner::Instance(8)),
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
                rect: Rect::from_min_size(Pos2::new(100.0, 0.0), Vec2::new(30.0, 10.0)),
            }],
            1.0,
            VIEW,
        );
        let changes: [(&str, fn(&mut Inputs)); 9] = [
            ("class", |inputs| inputs.0[1].class = 2),
            ("key", |inputs| inputs.0[0].key = "OUTP".to_owned()),
            ("candidate", |inputs| {
                inputs.0[1].candidates[1] = rect(0.0, 24.0)
            }),
            ("owner", |inputs| {
                inputs.0[0].owner = Some(ObstacleOwner::Label(4));
            }),
            ("badge added", |inputs| {
                inputs.0.push(badge(2, "R1", &[rect(50.0, 50.0)]));
            }),
            ("obstacle rect", |inputs| {
                inputs.1[0].rect =
                    Rect::from_min_size(Pos2::new(-5.0, -5.0), Vec2::new(30.0, 10.0));
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
