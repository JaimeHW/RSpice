//! Logic and bus lanes in the waveform stack, drawn from exact event traces.
//!
//! The stack's panes plot interpolated values. Digital nets are not that: a
//! net holds a level from one accepted event instant to the next. These lanes
//! draw that, on the panes' own time axis, from the committed event history
//! rather than from the `D()` columns resampled onto the analog grid.
//!
//! The widget is stateless. Which rows exist, which are selected, each bus's
//! radix and whether it is expanded are the host's to hold (they persist in
//! the visualization document); the widget reports what the reader asked for
//! in [`LaneStackResponse`] and the host applies it.

pub mod cursor;
pub mod markers;
pub mod model;
pub mod paint;
pub mod plan;
pub mod real;

pub use cursor::{EdgeStep, EventLane, LaneReadout};
pub use markers::{
    AssertionSpan, HdlMarker, MarkerSeverity, MarkerStripInput, MarkerStripResponse, SourceLine,
    marker_strip_height, show_marker_strip,
};
pub use model::{BusLane, BusOrigin, EventLanes, LogicLane, LogicLevel, RealLane};
pub use paint::{LaneLevels, LaneStyle, paint_bus_lane, paint_logic_lane};
pub use plan::{LaneStroke, TimeMapping, plan_lane};
pub use real::real_step_trace;
pub use rspice_results::events::projection::BusRadix;

use super::header::elide;
use egui::{Color32, Rect, Sense, Ui, pos2, vec2};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};

/// Height of a logic lane's row, in points.
pub const LOGIC_ROW_HEIGHT: f32 = 18.0;
/// Height of a bus lane's row: a word label needs a little more than a level.
pub const BUS_ROW_HEIGHT: f32 = 20.0;
/// Width of a bus row's expand control in the gutter.
const CHEVRON_WIDTH: f32 = 12.0;
/// Indent of an expanded bus's member rows in the gutter.
const MEMBER_INDENT: f32 = 12.0;
const GUTTER_PAD: f32 = 4.0;

/// The radixes a bus can be read in, in menu order, with their short names.
pub const BUS_RADIXES: [(BusRadix, &str); 4] = [
    (BusRadix::Binary, "BIN"),
    (BusRadix::Hex, "HEX"),
    (BusRadix::Unsigned, "DEC"),
    (BusRadix::Signed, "±DEC"),
];

/// One row of the lane stack.
pub enum LaneRow<'a> {
    Logic {
        lane: &'a LogicLane,
        color: Color32,
    },
    Bus {
        lane: &'a BusLane,
        radix: BusRadix,
        expanded: bool,
        /// The member lanes an expanded bus shows under it, MSB first.
        members: Vec<&'a LogicLane>,
        color: Color32,
    },
}

impl<'a> LaneRow<'a> {
    /// The lane the cursor reads and steps along on this row.
    pub fn event_lane(&self) -> EventLane<'a> {
        match self {
            Self::Logic { lane, .. } => EventLane::Logic(lane),
            Self::Bus { lane, radix, .. } => EventLane::Bus(lane, *radix),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Logic { lane, .. } => lane.name(),
            Self::Bus { lane, .. } => lane.name(),
        }
    }

    /// Height of this row with its member rows.
    pub fn height(&self) -> f32 {
        match self {
            Self::Logic { .. } => LOGIC_ROW_HEIGHT,
            Self::Bus {
                expanded, members, ..
            } => {
                let shown = if *expanded { members.len() } else { 0 };
                (shown as f32).mul_add(LOGIC_ROW_HEIGHT, BUS_ROW_HEIGHT)
            }
        }
    }
}

/// Total height of a lane stack.
pub fn stack_height(rows: &[LaneRow<'_>]) -> f32 {
    rows.iter().map(LaneRow::height).sum()
}

/// Everything one frame of the lane stack draws from.
pub struct LaneStackInput<'a> {
    pub rows: &'a [LaneRow<'a>],
    /// The shared time axis' visible range.
    pub x_range: (f64, f64),
    /// The panes' left gutter; lane names and readouts sit in it.
    pub left_margin: f32,
    /// The panes' right margin, so lane and pane X agree to the pixel.
    pub right_margin: f32,
    /// Row indices the reader selected. The first is the lane the cursor
    /// steps along.
    pub selected: &'a [usize],
    /// The strip's time cursor, shared with its panes.
    pub cursor: Option<f64>,
    /// Significant digits a real value is read out to.
    pub significant_digits: usize,
    /// Assertion violations, each shaded on its operand's lane.
    pub violations: &'a [AssertionSpan],
}

/// What the reader asked of the lane stack this frame.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LaneStackResponse {
    /// Select a row; `true` adds it to the selection instead.
    pub select: Option<(usize, bool)>,
    /// Move the time cursor to this exact event instant: the selected lane's
    /// previous or next edge (Left and Right while the lanes have focus).
    pub cursor_to: Option<f64>,
    /// Expand or collapse a bus row.
    pub toggle_expanded: Option<usize>,
    /// Read a bus row in another radix.
    pub set_radix: Option<(usize, BusRadix)>,
    /// Group these logic rows, MSB first, into a user bus.
    pub group: Option<Vec<usize>>,
    /// Ungroup a user bus row.
    pub ungroup: Option<usize>,
    /// A cross-probe for the host to route.
    pub probe: Option<LaneProbe>,
}

/// The time mapping of a lane stack occupying `rect`.
pub fn lane_mapping(rect: Rect, input: &LaneStackInput<'_>) -> TimeMapping {
    TimeMapping {
        t0: input.x_range.0,
        t1: input.x_range.1,
        left: rect.left() + input.left_margin,
        right: rect.right() - input.right_margin,
    }
}

/// The row under screen Y `y`, and which member of an expanded bus, if the
/// pointer is on one of its member rows.
fn row_at(rows: &[LaneRow<'_>], top: f32, y: f32) -> Option<(usize, Option<usize>)> {
    let mut row_top = top;
    for (index, row) in rows.iter().enumerate() {
        let height = row.height();
        if y >= row_top && y < row_top + height {
            let main = match row {
                LaneRow::Logic { .. } => LOGIC_ROW_HEIGHT,
                LaneRow::Bus { .. } => BUS_ROW_HEIGHT,
            };
            let member =
                (y >= row_top + main).then(|| ((y - row_top - main) / LOGIC_ROW_HEIGHT) as usize);
            return Some((index, member));
        }
        row_top += height;
    }
    None
}

/// A cross-probe the reader asked for from a lane. The host routes it: to
/// the HDL source, to the driving processes, or to the schematic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LaneProbe {
    /// Open the HDL declaration of the lane's net.
    GoToDeclaration { signal: String },
    /// List the processes that drive the lane's net, each with its
    /// `file:line`. Per-event attribution is not recorded, so at a transition
    /// this is the same static driver set.
    GoToDrivers { signal: String },
    /// Select and highlight the lane's net on the schematic.
    SelectNet { signal: String },
}

/// The digital quantity a lane draws, `D(net)`, as the result names it: a
/// bus by its name without its range, a member row by its member net.
pub fn lane_signal(row: &LaneRow<'_>, member: Option<usize>) -> Option<String> {
    let net = match (row, member) {
        (LaneRow::Logic { lane, .. }, _) => lane.name(),
        (LaneRow::Bus { members, .. }, Some(member)) => members.get(member)?.name(),
        (LaneRow::Bus { lane, .. }, None) => {
            rspice_results::engine_evidence::split_bus_notation(lane.name()).0
        }
    };
    Some(format!("D({net})"))
}

/// The expand control of a bus row: a small triangle painted as a vector,
/// pointing right when collapsed and down when expanded.
fn paint_chevron(ui: &Ui, centre: egui::Pos2, expanded: bool, color: Color32) {
    let r = 3.0;
    let points = if expanded {
        vec![
            pos2(centre.x - r, centre.y - r * 0.6),
            pos2(centre.x + r, centre.y - r * 0.6),
            pos2(centre.x, centre.y + r * 0.8),
        ]
    } else {
        vec![
            pos2(centre.x - r * 0.6, centre.y - r),
            pos2(centre.x + r * 0.8, centre.y),
            pos2(centre.x - r * 0.6, centre.y + r),
        ]
    };
    ui.painter().add(egui::Shape::convex_polygon(
        points,
        color,
        egui::Stroke::NONE,
    ));
}

/// Paint one row's gutter: its value at the cursor right-aligned, and its
/// name elided to the space the value leaves.
fn paint_gutter(
    ui: &Ui,
    rect: Rect,
    left: f32,
    name: &str,
    readout: Option<&str>,
    tokens: &Tokens,
) {
    let font = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
    let painter = ui.painter();
    let mut right = rect.right() - GUTTER_PAD;
    if let Some(readout) = readout {
        let value = painter.layout_no_wrap(readout.to_owned(), font.clone(), tokens.color.text);
        let origin = pos2(
            (right - value.size().x).max(left),
            rect.center().y - value.size().y * 0.5,
        );
        right = origin.x - GUTTER_PAD;
        painter.galley(origin, value, tokens.color.text);
    }
    let width = (right - left).max(0.0);
    let mut galley = painter.layout_no_wrap(name.to_owned(), font.clone(), tokens.color.text_dim);
    let chars = name.chars().count().max(1);
    if galley.size().x > width {
        // The gutter font is monospaced, so one glyph's advance is the
        // whole name's over its length.
        let advance = (galley.size().x / chars as f32).max(1.0);
        let fit = (width / advance).floor().max(1.0) as usize;
        galley = painter.layout_no_wrap(elide(name, fit), font, tokens.color.text_dim);
    }
    let top = rect.center().y - galley.size().y * 0.5;
    painter.galley(pos2(left, top), galley, tokens.color.text_dim);
}

/// Draw the lane stack into the next `stack_height(rows)` points of `ui`.
pub fn show_lanes(ui: &mut Ui, input: &LaneStackInput<'_>) -> LaneStackResponse {
    let tokens = Tokens::get(ui.ctx());
    let height = stack_height(input.rows);
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::click());
    let map = lane_mapping(rect, input);
    let mut output = LaneStackResponse::default();
    if !ui.is_rect_visible(rect) {
        return output;
    }
    let gutter_width = input.left_margin - GUTTER_PAD;
    let lane_clip = Rect::from_x_y_ranges(map.left..=map.right, rect.y_range());
    let lane_painter = ui.painter().with_clip_rect(lane_clip);
    let readout = |lane: EventLane<'_>| {
        input
            .cursor
            .and_then(|t| lane.readout_at(t, input.significant_digits))
            .map(|readout| readout.text)
    };

    let mut top = rect.top();
    for (index, row) in input.rows.iter().enumerate() {
        let row_rect =
            Rect::from_min_size(pos2(rect.left(), top), vec2(rect.width(), row.height()));
        if input.selected.contains(&index) {
            ui.painter()
                .rect_filled(row_rect, 0.0, tokens.color.accent_dim);
        }
        let signal = lane_signal(row, None);
        for span in input.violations.iter().filter(|span| {
            span.operand == row.name() || signal.as_deref() == Some(span.operand.as_str())
        }) {
            markers::paint_violation_span(&lane_painter, row_rect, span, &map, &tokens);
        }
        match row {
            LaneRow::Logic { lane, color } => {
                let lane_rect = Rect::from_x_y_ranges(map.left..=map.right, row_rect.y_range());
                let gutter =
                    Rect::from_min_size(row_rect.min, vec2(gutter_width, LOGIC_ROW_HEIGHT));
                paint_gutter(
                    ui,
                    gutter,
                    gutter.left() + GUTTER_PAD,
                    lane.name(),
                    readout(EventLane::Logic(lane)).as_deref(),
                    &tokens,
                );
                paint_logic_lane(
                    &lane_painter,
                    lane_rect,
                    lane,
                    &map,
                    &LaneStyle::new(&tokens, *color),
                );
            }
            LaneRow::Bus {
                lane,
                radix,
                expanded,
                members,
                color,
            } => {
                let main =
                    Rect::from_min_size(row_rect.min, vec2(row_rect.width(), BUS_ROW_HEIGHT));
                let gutter = Rect::from_min_size(main.min, vec2(gutter_width, BUS_ROW_HEIGHT));
                if !members.is_empty() {
                    paint_chevron(
                        ui,
                        pos2(gutter.left() + GUTTER_PAD + 3.0, gutter.center().y),
                        *expanded,
                        tokens.color.text_dim,
                    );
                }
                paint_gutter(
                    ui,
                    gutter,
                    // Only a bus with member lanes to show has the control.
                    gutter.left()
                        + GUTTER_PAD
                        + if members.is_empty() {
                            0.0
                        } else {
                            CHEVRON_WIDTH
                        },
                    lane.name(),
                    readout(EventLane::Bus(lane, *radix)).as_deref(),
                    &tokens,
                );
                let style = LaneStyle::new(&tokens, *color);
                let lane_rect = Rect::from_x_y_ranges(map.left..=map.right, main.y_range());
                paint_bus_lane(&lane_painter, lane_rect, lane, *radix, &map, &style);
                if *expanded {
                    let mut member_top = main.bottom();
                    for member in members {
                        let member_rect = Rect::from_x_y_ranges(
                            rect.x_range(),
                            member_top..=member_top + LOGIC_ROW_HEIGHT,
                        );
                        let member_gutter = Rect::from_min_size(
                            member_rect.min,
                            vec2(gutter_width, LOGIC_ROW_HEIGHT),
                        );
                        paint_gutter(
                            ui,
                            member_gutter,
                            member_gutter.left() + GUTTER_PAD + MEMBER_INDENT,
                            member.name(),
                            readout(EventLane::Logic(member)).as_deref(),
                            &tokens,
                        );
                        let member_lane =
                            Rect::from_x_y_ranges(map.left..=map.right, member_rect.y_range());
                        paint_logic_lane(&lane_painter, member_lane, member, &map, &style);
                        member_top += LOGIC_ROW_HEIGHT;
                    }
                }
            }
        }
        top += row.height();
    }
    if let Some(t) = input.cursor
        && (input.x_range.0..=input.x_range.1).contains(&t)
    {
        lane_painter.vline(
            map.x(t),
            rect.y_range(),
            egui::Stroke::new(1.0, tokens.color.accent),
        );
    }

    let pointer_row = response
        .interact_pointer_pos()
        .or_else(|| response.hover_pos())
        .and_then(|pointer| row_at(input.rows, rect.top(), pointer.y).map(|row| (pointer, row)));
    if response.clicked()
        && let Some((pointer, (index, member))) = pointer_row
    {
        let on_chevron = pointer.x < rect.left() + GUTTER_PAD + CHEVRON_WIDTH;
        if member.is_none()
            && on_chevron
            && matches!(&input.rows[index], LaneRow::Bus { members, .. } if !members.is_empty())
        {
            output.toggle_expanded = Some(index);
        } else {
            let extend = ui.input(|i| i.modifiers.command || i.modifiers.shift);
            output.select = Some((index, extend));
            // Selecting a lane highlights its net, as selecting an analog
            // trace does.
            if !extend {
                output.probe = lane_signal(&input.rows[index], member)
                    .map(|signal| LaneProbe::SelectNet { signal });
            }
        }
        // Focus is what routes Left and Right here; a pointer press takes it
        // without showing a ring.
        response.request_focus();
    }
    // A double click on a lane, at a transition or between them, asks who
    // drives it.
    if response.double_clicked()
        && let Some((pointer, (index, member))) = pointer_row
        && pointer.x >= map.left
    {
        output.probe =
            lane_signal(&input.rows[index], member).map(|signal| LaneProbe::GoToDrivers { signal });
    }
    if response.has_focus() {
        ui.memory_mut(|memory| {
            memory.set_focus_lock_filter(
                response.id,
                egui::EventFilter {
                    horizontal_arrows: true,
                    ..Default::default()
                },
            );
        });
        let step = ui.input(|i| {
            if i.key_pressed(egui::Key::ArrowLeft) {
                Some(EdgeStep::Previous)
            } else if i.key_pressed(egui::Key::ArrowRight) {
                Some(EdgeStep::Next)
            } else {
                None
            }
        });
        if let Some(step) = step
            && let Some(lane) = input
                .selected
                .first()
                .and_then(|index| input.rows.get(*index))
                .map(LaneRow::event_lane)
        {
            let from = input.cursor.unwrap_or(match step {
                EdgeStep::Previous => input.x_range.1,
                EdgeStep::Next => input.x_range.0,
            });
            output.cursor_to = lane.step_from(from, step);
        }
    }
    if let (Some(t), Some((_, (index, None)))) = (input.cursor, pointer_row)
        && let Some(fallback) = input
            .rows
            .get(index)
            .and_then(|row| row.event_lane().readout_at(t, input.significant_digits))
            .and_then(|readout| readout.fallback)
    {
        response.clone().on_hover_text(fallback);
    }
    let menu_row = response.id.with("menu-row");
    if response.secondary_clicked()
        && let Some((_, row)) = pointer_row
    {
        ui.ctx().data_mut(|data| data.insert_temp(menu_row, row));
    }
    response.context_menu(|ui| {
        let Some((index, member)) = ui
            .ctx()
            .data(|data| data.get_temp::<(usize, Option<usize>)>(menu_row))
        else {
            ui.close();
            return;
        };
        lane_menu(ui, input, index, member, &mut output);
    });
    output
}

/// The row menu: the lane's cross-probes, a bus's radix and grouping, and
/// grouping a logic selection.
fn lane_menu(
    ui: &mut Ui,
    input: &LaneStackInput<'_>,
    index: usize,
    member: Option<usize>,
    output: &mut LaneStackResponse,
) {
    let Some(row) = input.rows.get(index) else {
        ui.close();
        return;
    };
    if let Some(signal) = lane_signal(row, member) {
        let probes = [
            (
                "Go to declaration",
                LaneProbe::GoToDeclaration {
                    signal: signal.clone(),
                },
            ),
            (
                "Go to drivers",
                LaneProbe::GoToDrivers {
                    signal: signal.clone(),
                },
            ),
            ("Select net", LaneProbe::SelectNet { signal }),
        ];
        for (label, probe) in probes {
            if ui.button(label).clicked() {
                output.probe = Some(probe);
                ui.close();
            }
        }
        if member.is_none() {
            ui.separator();
        }
    }
    if member.is_some() {
        return;
    }
    if let LaneRow::Bus { lane, radix, .. } = row {
        for (candidate, label) in BUS_RADIXES {
            if ui.radio(*radix == candidate, label).clicked() {
                output.set_radix = Some((index, candidate));
                ui.close();
            }
        }
        if lane.origin() == BusOrigin::User {
            ui.separator();
            if ui.button("Ungroup").clicked() {
                output.ungroup = Some(index);
                ui.close();
            }
        }
    }
    let logic: Vec<usize> = input
        .selected
        .iter()
        .copied()
        .filter(|selected| matches!(input.rows.get(*selected), Some(LaneRow::Logic { .. })))
        .collect();
    if logic.len() >= 2 && logic.contains(&index) {
        let mut ordered = logic;
        ordered.sort_unstable();
        if ui.button("Group as bus").clicked() {
            output.group = Some(ordered);
            ui.close();
        }
    }
}

#[cfg(test)]
mod render_tests;
#[cfg(test)]
mod tests;
