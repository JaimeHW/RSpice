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
    AssertionSpan, HdlMarker, MarkerSeverity, MarkerStripInput, MarkerStripResponse, RunMarkers,
    SourceLine, StopMark, marker_strip_height, markers_height, show_marker_strip,
};
pub use model::{BusLane, BusOrigin, EventLanes, LogicLane, LogicLevel, RealLane};
pub use paint::{LaneLevels, LaneStyle, paint_bus_lane, paint_logic_lane};
pub use plan::{LaneStroke, TimeMapping, plan_lane};
pub use real::real_step_trace;
pub use rspice_results::events::projection::BusRadix;

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
pub const BUS_RADIXES: [(BusRadix, &str); 6] = [
    (BusRadix::Binary, BusRadix::Binary.short_label()),
    (BusRadix::Octal, BusRadix::Octal.short_label()),
    (BusRadix::Hex, BusRadix::Hex.short_label()),
    (BusRadix::Unsigned, BusRadix::Unsigned.short_label()),
    (BusRadix::Signed, BusRadix::Signed.short_label()),
    (BusRadix::Ascii, BusRadix::Ascii.short_label()),
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
    /// The gutter's name column, at its left; the value column is the rest.
    pub name_width: f32,
    /// The panes' right margin, so lane and pane X agree to the pixel.
    pub right_margin: f32,
    /// Row indices the reader selected. The first is the lane the cursor
    /// steps along.
    pub selected: &'a [usize],
    /// The strip's time cursor A, shared with its panes. Each row's gutter
    /// reads its value here.
    pub cursor: Option<f64>,
    /// The strip's cursor B. The cursor placed last is the one an edge step
    /// moves: B once it is placed, A before.
    pub cursor_b: Option<f64>,
    /// Significant digits a real value is read out to.
    pub significant_digits: usize,
    /// Assertion violations, each shaded on its operand's lane.
    pub violations: &'a [AssertionSpan],
    /// Whether the host can open a lane's HDL declaration and drivers. When
    /// it cannot, those probes are not offered at all.
    pub source_probes: bool,
}

/// What the reader asked of the lane stack this frame.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LaneStackResponse {
    /// Select a row; `true` adds it to the selection instead.
    pub select: Option<(usize, bool)>,
    /// Move the stepping cursor (B once placed, A before) to this exact
    /// event instant: the selected lane's previous or next edge (Left and
    /// Right while the lanes have focus).
    pub cursor_to: Option<f64>,
    /// Expand or collapse a bus row.
    pub toggle_expanded: Option<usize>,
    /// Read a bus row in another radix.
    pub set_radix: Option<(usize, BusRadix)>,
    /// Group these logic rows, MSB first, into a user bus.
    pub group: Option<Vec<usize>>,
    /// Ungroup a user bus row.
    pub ungroup: Option<usize>,
    /// Move a row one place down (`true`) or up (`false`).
    pub move_row: Option<(usize, bool)>,
    /// A cross-probe for the host to route.
    pub probe: Option<LaneProbe>,
    /// The name column's new width, while the reader drags its edge.
    pub resize_name: Option<f32>,
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

/// Where a lane row's gutter columns sit: the name column from the row's
/// left edge, the value column after it, up to the time axis.
#[derive(Clone, Copy)]
struct GutterColumns {
    name_right: f32,
    value_left: f32,
    value_right: f32,
}

impl GutterColumns {
    fn of(rect: Rect, input: &LaneStackInput<'_>) -> Self {
        let name_right = rect.left() + input.name_width.min(input.left_margin);
        Self {
            name_right,
            value_left: name_right,
            value_right: rect.left() + input.left_margin,
        }
    }
}

/// Paint one row's gutter: its name in the name column from `name_left`,
/// elided in the middle only when it does not fit, and its value at the
/// cursor right-aligned in the value column. Returns the name's cell when
/// the name was elided, so a hover can state it whole.
fn paint_gutter(
    ui: &Ui,
    row: Rect,
    name_left: f32,
    columns: GutterColumns,
    name: &str,
    readout: Option<&str>,
    tokens: &Tokens,
) -> Option<Rect> {
    let font = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
    let painter = ui.painter();
    if let Some(readout) = readout {
        let width = (columns.value_right - columns.value_left - 2.0 * GUTTER_PAD).max(0.0);
        let value = rspice_ui_kit::panels::elide_middle(ui, readout, &font, width);
        let galley = painter.layout_no_wrap(value, font.clone(), tokens.color.canvas_text);
        let origin = pos2(
            columns.value_right - GUTTER_PAD - galley.size().x,
            row.center().y - galley.size().y * 0.5,
        );
        painter.galley(origin, galley, tokens.color.canvas_text);
    }
    let width = (columns.name_right - GUTTER_PAD - name_left).max(0.0);
    let shown = rspice_ui_kit::panels::elide_middle(ui, name, &font, width);
    let elided = shown != name;
    let galley = painter.layout_no_wrap(shown, font, tokens.color.canvas_text_dim);
    let top = row.center().y - galley.size().y * 0.5;
    painter.galley(pos2(name_left, top), galley, tokens.color.canvas_text_dim);
    elided.then(|| Rect::from_x_y_ranges(name_left..=columns.name_right, row.y_range()))
}

/// The name column's width that shows every row of `specs` whole: each name
/// at the indent its row draws it at, after a bus's chevron, with the pad
/// after it. A bus's members count whether it is open or not, so opening one
/// does not move the time axis.
pub fn name_column_fit(ui: &Ui, specs: &[super::lanes::LaneSpec]) -> f32 {
    use super::lanes::LaneSpec;
    let font = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
    let width = |name: &str| {
        ui.painter()
            .layout_no_wrap(name.to_owned(), font.clone(), Color32::WHITE)
            .size()
            .x
    };
    specs
        .iter()
        .flat_map(|spec| match spec {
            LaneSpec::Logic { net, .. } => vec![width(net)],
            LaneSpec::Bus { label, members, .. } => {
                let chevron = if members.is_empty() {
                    0.0
                } else {
                    CHEVRON_WIDTH
                };
                std::iter::once(chevron + width(label))
                    .chain(
                        members
                            .iter()
                            .map(|member| MEMBER_INDENT + width(&member.net)),
                    )
                    .collect()
            }
        })
        .fold(0.0, f32::max)
        + 2.0 * GUTTER_PAD
}

/// The grip on the name column's edge: a strip a few points wide the
/// reader drags to size the column. Returns the column's new width.
pub fn name_column_grip(ui: &Ui, rect: Rect, name_width: f32, id: egui::Id) -> Option<f32> {
    let edge = rect.left() + name_width;
    let grip = Rect::from_x_y_ranges(edge - 3.0..=edge + 3.0, rect.y_range());
    let response = ui.interact(grip, id, Sense::drag());
    if response.hovered() || response.dragged() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeColumn);
        let tokens = Tokens::get(ui.ctx());
        ui.painter().vline(
            edge,
            rect.y_range(),
            egui::Stroke::new(1.0, tokens.color.canvas_text_faint),
        );
    }
    response
        .dragged()
        .then(|| response.interact_pointer_pos())
        .flatten()
        .map(|pointer| pointer.x - rect.left())
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
    let columns = GutterColumns::of(rect, input);
    let mut elided: Vec<(Rect, &str)> = Vec::new();
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
        // A deck names a net without regard to case, so the check's operand
        // does too.
        for span in input.violations.iter().filter(|span| {
            span.operand.eq_ignore_ascii_case(row.name())
                || signal
                    .as_deref()
                    .is_some_and(|signal| signal.eq_ignore_ascii_case(&span.operand))
        }) {
            markers::paint_violation_span(&lane_painter, row_rect, span, &map, &tokens);
        }
        match row {
            LaneRow::Logic { lane, color } => {
                let lane_rect = Rect::from_x_y_ranges(map.left..=map.right, row_rect.y_range());
                elided.extend(
                    paint_gutter(
                        ui,
                        row_rect,
                        row_rect.left() + GUTTER_PAD,
                        columns,
                        lane.name(),
                        readout(EventLane::Logic(lane)).as_deref(),
                        &tokens,
                    )
                    .map(|cell| (cell, lane.name())),
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
                if !members.is_empty() {
                    paint_chevron(
                        ui,
                        pos2(main.left() + GUTTER_PAD + 3.0, main.center().y),
                        *expanded,
                        tokens.color.canvas_text_dim,
                    );
                }
                elided.extend(
                    paint_gutter(
                        ui,
                        main,
                        // Only a bus with member lanes to show has the control.
                        main.left()
                            + GUTTER_PAD
                            + if members.is_empty() {
                                0.0
                            } else {
                                CHEVRON_WIDTH
                            },
                        columns,
                        lane.name(),
                        readout(EventLane::Bus(lane, *radix)).as_deref(),
                        &tokens,
                    )
                    .map(|cell| (cell, lane.name())),
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
                        elided.extend(
                            paint_gutter(
                                ui,
                                member_rect,
                                member_rect.left() + GUTTER_PAD + MEMBER_INDENT,
                                columns,
                                member.name(),
                                readout(EventLane::Logic(member)).as_deref(),
                                &tokens,
                            )
                            .map(|cell| (cell, member.name())),
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
    // The cursors run through the lanes in the colours the panes draw them.
    for (cursor, color) in [
        (input.cursor, tokens.color.canvas_cursor_a),
        (input.cursor_b, tokens.color.canvas_cursor_b),
    ] {
        if let Some(t) = cursor
            && (input.x_range.0..=input.x_range.1).contains(&t)
        {
            lane_painter.vline(map.x(t), rect.y_range(), egui::Stroke::new(1.0, color));
        }
    }

    output.resize_name = name_column_grip(
        ui,
        rect,
        columns.name_right - rect.left(),
        response.id.with("grip"),
    );
    if let Some(pointer) = response.hover_pos()
        && let Some((_, name)) = elided.iter().find(|(cell, _)| cell.contains(pointer))
    {
        response.clone().on_hover_text_at_pointer(*name);
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
        && input.source_probes
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
            let from = input.cursor_b.or(input.cursor).unwrap_or(match step {
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
        let source = [
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
        ];
        let probes = source
            .into_iter()
            .filter(|_| input.source_probes)
            .chain([("Select net", LaneProbe::SelectNet { signal })]);
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
    let groups = logic.len() >= 2 && logic.contains(&index);
    if groups {
        let mut ordered = logic;
        ordered.sort_unstable();
        if ui.button("Group as bus").clicked() {
            output.group = Some(ordered);
            ui.close();
        }
    }
    // The probes end in a separator; a bus's radixes or a grouping take one
    // of their own before the moves.
    if groups || matches!(row, LaneRow::Bus { .. }) {
        ui.separator();
    }
    let last = input.rows.len().saturating_sub(1);
    for (label, down, enabled) in [
        ("Move up", false, index > 0),
        ("Move down", true, index < last),
    ] {
        if ui.add_enabled(enabled, egui::Button::new(label)).clicked() {
            output.move_row = Some((index, down));
            ui.close();
        }
    }
}

#[cfg(test)]
mod render_tests;
#[cfg(test)]
mod tests;
