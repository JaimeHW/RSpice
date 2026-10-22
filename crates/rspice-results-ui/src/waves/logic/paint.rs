//! Painting logic and bus lanes from their plans.
//!
//! High and low are two levels inside the lane's row with vertical edges at
//! the exact instants. An unknown (`x`) stretch is a band filled between the
//! levels in the kit's warning token, and high impedance (`z`) a dashed line
//! at mid level. A bus word is a hexagonal segment labelled in the lane's
//! radix, and a word with any unknown or high-impedance bit takes the
//! unknown band's fill. Activity blocks are filled between the levels in the
//! lane's own colour. No colour is introduced here: the lane colour is the
//! host's trace colour, and everything else is a kit token.

use super::model::{BusLane, LogicLane, LogicLevel};
use super::plan::{LaneStroke, TimeMapping, plan_lane};
use egui::{Color32, FontId, Painter, Pos2, Rect, Shape, Stroke, pos2};
use rspice_results::events::projection::BusRadix;
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};

/// Space between a lane's levels and its row edges, in points.
const LEVEL_INSET: f32 = 3.0;
/// The run of a bus segment's pointed ends, in points.
const BUS_SLANT: f32 = 3.0;
/// Clearance a bus label keeps from its segment's pointed ends.
const LABEL_CLEARANCE: f32 = 2.0;
/// Narrower segments are not offered a label at all, so a dense bus lays
/// out no text it would only elide.
const MIN_LABELLED_SEGMENT: f32 = 12.0;
/// How strongly the unknown band and activity blocks fill.
const UNKNOWN_FILL: f32 = 0.4;
const ACTIVITY_FILL: f32 = 0.55;
const Z_DASH: f32 = 3.0;
const Z_GAP: f32 = 2.0;

/// The inks one lane is painted in.
#[derive(Debug, Clone)]
pub struct LaneStyle {
    /// The lane's trace colour.
    pub color: Color32,
    /// Unknown (`x`) stretches and unresolved bus words: the warning token.
    pub unknown: Color32,
    /// Bus word labels.
    pub text: Color32,
    pub font: FontId,
    pub width: f32,
}

impl LaneStyle {
    pub fn new(tokens: &Tokens, color: Color32) -> Self {
        Self {
            color,
            unknown: tokens.color.warn,
            text: tokens.color.text,
            font: theme::mono(tokens::FS_MICRO, FontWeight::Regular),
            width: 1.0,
        }
    }
}

/// Where a lane's levels sit inside its row.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LaneLevels {
    pub high: f32,
    pub low: f32,
}

impl LaneLevels {
    pub fn of(rect: Rect) -> Self {
        let inset = LEVEL_INSET.min(rect.height() * 0.25);
        Self {
            high: rect.top() + inset,
            low: rect.bottom() - inset,
        }
    }

    pub fn mid(self) -> f32 {
        (self.high + self.low) * 0.5
    }
}

fn activity(painter: &Painter, x0: f32, x1: f32, levels: LaneLevels, style: &LaneStyle) {
    painter.rect_filled(
        Rect::from_x_y_ranges(x0..=x1.max(x0 + 1.0), levels.high..=levels.low),
        0.0,
        style.color.gamma_multiply(ACTIVITY_FILL),
    );
}

fn level_run(
    painter: &Painter,
    x0: f32,
    x1: f32,
    level: LogicLevel,
    levels: LaneLevels,
    style: &LaneStyle,
) {
    let stroke = Stroke::new(style.width, style.color);
    match level {
        LogicLevel::Low => {
            painter.hline(x0..=x1, levels.low, stroke);
        }
        LogicLevel::High => {
            painter.hline(x0..=x1, levels.high, stroke);
        }
        LogicLevel::Unknown => {
            painter.rect_filled(
                Rect::from_x_y_ranges(x0..=x1, levels.high..=levels.low),
                0.0,
                style.unknown.gamma_multiply(UNKNOWN_FILL),
            );
            let edge = Stroke::new(style.width, style.unknown);
            painter.hline(x0..=x1, levels.high, edge);
            painter.hline(x0..=x1, levels.low, edge);
        }
        LogicLevel::HighImpedance => {
            painter.extend(Shape::dashed_line(
                &[pos2(x0, levels.mid()), pos2(x1, levels.mid())],
                stroke,
                Z_DASH,
                Z_GAP,
            ));
        }
    }
}

/// Paint a logic lane into `rect`, whose horizontal extent `map` spans.
pub fn paint_logic_lane(
    painter: &Painter,
    rect: Rect,
    lane: &LogicLane,
    map: &TimeMapping,
    style: &LaneStyle,
) {
    let levels = LaneLevels::of(rect);
    for stroke in plan_lane(lane.times(), map) {
        match stroke {
            LaneStroke::Hold { x0, x1, index } => {
                level_run(painter, x0, x1, lane.levels()[index], levels, style);
            }
            // The first instant is when the net was first observed, not a
            // change from anything, so it draws no edge.
            LaneStroke::Edge { x, index } if index > 0 => {
                painter.vline(
                    x,
                    levels.high..=levels.low,
                    Stroke::new(style.width, style.color),
                );
            }
            LaneStroke::Edge { .. } => {}
            LaneStroke::Activity { x0, x1 } => activity(painter, x0, x1, levels, style),
        }
    }
}

/// The outline of one bus segment across `[x0, x1]`: pointed where a word
/// changes, square where the view clips it.
fn segment_outline(x0: f32, x1: f32, levels: LaneLevels, map: &TimeMapping) -> Vec<Pos2> {
    let slant = BUS_SLANT.min((x1 - x0) * 0.5);
    let left_slant = if x0 > map.left { slant } else { 0.0 };
    let right_slant = if x1 < map.right { slant } else { 0.0 };
    let mid = levels.mid();
    vec![
        pos2(x0, mid),
        pos2(x0 + left_slant, levels.high),
        pos2(x1 - right_slant, levels.high),
        pos2(x1, mid),
        pos2(x1 - right_slant, levels.low),
        pos2(x0 + left_slant, levels.low),
    ]
}

/// Paint a bus lane into `rect`, its words spelled in `radix`.
///
/// A label is drawn only where it fits whole between its segment's pointed
/// ends; a narrower segment shows its outline alone.
pub fn paint_bus_lane(
    painter: &Painter,
    rect: Rect,
    lane: &BusLane,
    radix: BusRadix,
    map: &TimeMapping,
    style: &LaneStyle,
) {
    let levels = LaneLevels::of(rect);
    if let Some(refusal) = lane.refusal() {
        painter.text(
            pos2(map.left + LABEL_CLEARANCE, levels.mid()),
            egui::Align2::LEFT_CENTER,
            refusal,
            style.font.clone(),
            style.unknown,
        );
        return;
    }
    let stroke = Stroke::new(style.width, style.color);
    for planned in plan_lane(lane.times(), map) {
        match planned {
            LaneStroke::Hold { x0, x1, index } => {
                let unresolved = lane.word_is_unresolved(index);
                let fill = if unresolved {
                    style.unknown.gamma_multiply(UNKNOWN_FILL)
                } else {
                    Color32::TRANSPARENT
                };
                let outline = segment_outline(x0, x1, levels, map);
                painter.add(Shape::convex_polygon(outline, fill, stroke));
                let width = x1 - x0 - 2.0 * (BUS_SLANT + LABEL_CLEARANCE);
                if x1 - x0 >= MIN_LABELLED_SEGMENT
                    && let Some(word) = lane.spell(index, radix)
                {
                    let galley = painter.layout_no_wrap(
                        word.text().to_owned(),
                        style.font.clone(),
                        style.text,
                    );
                    if galley.size().x <= width {
                        let centre = pos2((x0 + x1) * 0.5, levels.mid());
                        painter.galley(centre - galley.size() * 0.5, galley, style.text);
                    }
                }
            }
            LaneStroke::Edge { .. } => {}
            LaneStroke::Activity { x0, x1 } => activity(painter, x0, x1, levels, style),
        }
    }
}
