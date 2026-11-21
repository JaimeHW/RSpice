//! The Y axes a strip reads beside its first. Every axis shares the strip's
//! one plot area, as ADS rules its Y1 and Y2 and ViVA and EZwave stack
//! more: the second axis stands on the plot's right edge, a third outside
//! the first axis' labels on the left, a fourth outside the second on the
//! right. Each is drawn as the first is: a hairline in the frame's ink, its
//! ticks and labels in the plot's tick font, its column exactly as wide as
//! its labels. An axis whose curves share one colour sets its unit and
//! labels in that colour; one carrying several colours sets them in text
//! ink with a swatch of each curve beside its unit. The grid stays the
//! first axis'. A strip with no room for an axis leaves it out, and its
//! trace list names it. Scrolled over, an axis zooms about the pointer;
//! dragged, it pans; double-clicked, it fits its curves again. Its range is
//! its own: the strip's other axes stay as they are.

use egui::{Color32, Rect, Sense, Ui, pos2};
use rspice_results::visualization_document::MAX_STRIP_AXES;
use rspice_ui_kit::plot;
use rspice_ui_kit::theme;
use rspice_ui_kit::tokens::Tokens;

use super::StripModel;
use super::navigation::WAVE_SHARED_RIGHT_MARGIN;
use super::pane::SecondaryPlot;
use super::stack::Gutter;

/// How many axes a strip reads beside its first.
pub(crate) const ADDED_AXES: usize = MAX_STRIP_AXES - 1;
/// How far one wheel notch zooms an axis.
const ZOOM_PER_POINT: f64 = 0.003;
/// How far an arrow key pans a focused axis, as a share of its span.
const KEY_PAN_SHARE: f64 = 0.1;
/// The length of an added axis' tick marks, outward from its hairline.
const TICK_LENGTH: f32 = 4.0;
/// What a column keeps past its widest label, so the next column's labels
/// never touch its own.
const COLUMN_PAD: f32 = 4.0;
/// A swatch beside an axis' unit, and the gap after it.
const SWATCH_WIDTH: f32 = 9.0;
const SWATCH_GAP: f32 = 3.0;
/// The least width the plot keeps beside its axes, as points and as a
/// share of the strip: an axis that would squeeze it further waits in the
/// strip's trace list instead.
const PLOT_MIN_WIDTH: f32 = 160.0;
const PLOT_MIN_SHARE: f32 = 0.5;

/// The side of the plot an added axis stands on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Side {
    Left,
    Right,
}

/// The side the strip's `index`th added axis stands on: the second axis on
/// the right edge, the third outside the first on the left, the fourth
/// outside the second on the right.
pub(crate) const fn column_side(index: usize) -> Side {
    if index % 2 == 0 {
        Side::Right
    } else {
        Side::Left
    }
}

/// What one strip's axes need beside its plot, measured as drawn.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct AxisRoom {
    /// The width the first axis' labels and unit take left of the plot.
    pub primary: f32,
    /// The width of each added axis' column, by its place in the strip's
    /// list after the first; 0 for none.
    pub columns: [f32; ADDED_AXES],
}

impl AxisRoom {
    /// The room that holds both: what every strip of an analysis needs,
    /// from what each of them does.
    pub(crate) fn widest(self, other: Self) -> Self {
        let mut columns = self.columns;
        for (column, theirs) in columns.iter_mut().zip(other.columns) {
            *column = column.max(theirs);
        }
        Self {
            primary: self.primary.max(other.primary),
            columns,
        }
    }

    /// Whether `other` needs more room than this anywhere.
    pub(crate) fn outgrown_by(self, other: Self) -> bool {
        const SLACK: f32 = 0.5;
        other.primary > self.primary + SLACK
            || self
                .columns
                .iter()
                .zip(other.columns)
                .any(|(mine, theirs)| theirs > mine + SLACK)
    }
}

/// What the axes of an analysis' strips needed when the stack was last
/// drawn, which the gutter reserves, and what they need this frame so far.
#[derive(Debug, Clone, Copy, Default)]
struct RoomMemory {
    shown: AxisRoom,
    pending: AxisRoom,
    pass: u64,
}

fn room_id(analysis: impl std::hash::Hash + std::fmt::Debug) -> egui::Id {
    egui::Id::new(("rspice.results.axis-room", analysis))
}

/// The memory of `id`, the last frame's measures made the ones reserved
/// once a new frame starts.
fn room_memory(ctx: &egui::Context, id: egui::Id) -> RoomMemory {
    let pass = ctx.cumulative_pass_nr();
    let held = ctx.data(|data| data.get_temp::<RoomMemory>(id));
    match held {
        Some(memory) if memory.pass == pass => memory,
        held => {
            let memory = RoomMemory {
                shown: held.map(|memory| memory.pending).unwrap_or_default(),
                pending: AxisRoom::default(),
                pass,
            };
            ctx.data_mut(|data| data.insert_temp(id, memory));
            memory
        }
    }
}

/// The room the axes of `analysis`' strips took when last drawn: what
/// every strip's gutter reserves, so all of them start and end their plots
/// on one rule.
pub(crate) fn remembered_room(
    ctx: &egui::Context,
    analysis: impl std::hash::Hash + std::fmt::Debug,
) -> AxisRoom {
    room_memory(ctx, room_id(analysis)).shown
}

/// Keep what one strip of `analysis` needs for its axes this frame. A strip
/// that needs more than was reserved draws again at once with the room.
pub(crate) fn note_room(
    ctx: &egui::Context,
    analysis: impl std::hash::Hash + std::fmt::Debug,
    room: AxisRoom,
) {
    let id = room_id(analysis);
    let mut memory = room_memory(ctx, id);
    memory.pending = memory.pending.widest(room);
    if memory.shown.outgrown_by(memory.pending) {
        ctx.request_repaint();
    }
    ctx.data_mut(|data| data.insert_temp(id, memory));
}

/// How the added axes of an analysis' strips fit beside their plots.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct AxisFit {
    /// How many of each strip's added axes have room, in the order they
    /// take it.
    pub kept: usize,
    /// How many of them have been measured: those past `kept` wait in the
    /// strip's trace list for room.
    pub measured: usize,
    /// What the kept left axis takes beyond the first axis' gutter.
    pub left: f32,
    /// What the kept right axes take right of the plot.
    pub right: f32,
}

/// Fit the added axes `room` measures beside plots in strips
/// `strip_width` wide whose left gutter is `name` + `value`: each in turn
/// while the plot keeps its least width. An axis not yet measured has no
/// column, and neither has any after it.
pub(crate) fn fit_axes(strip_width: f32, (name, value): (f32, f32), room: AxisRoom) -> AxisFit {
    let floor = PLOT_MIN_WIDTH.max(strip_width * PLOT_MIN_SHARE);
    let measured = room
        .columns
        .iter()
        .take_while(|width| **width > 0.0)
        .count();
    let mut fit = AxisFit {
        measured,
        ..AxisFit::default()
    };
    for (index, width) in room.columns.into_iter().enumerate().take(measured) {
        let (left, right) = match column_side(index) {
            Side::Left => (
                (plot::Y_LABEL_GAP + room.primary + COLUMN_PAD + width - value).max(0.0),
                fit.right,
            ),
            Side::Right => (fit.left, fit.right + width),
        };
        let plot = strip_width - name - value - left - WAVE_SHARED_RIGHT_MARGIN.max(right);
        if plot < floor {
            break;
        }
        fit = AxisFit {
            kept: index + 1,
            measured,
            left,
            right,
        };
    }
    fit
}

/// Each added axis' column beside `plot_rect`, by its place: the right
/// ones from the plot's right edge outward, the left one outside the first
/// axis' labels (`primary` wide). An axis with no width has no column.
pub(crate) fn column_rects(
    plot_rect: Rect,
    primary: f32,
    widths: [f32; ADDED_AXES],
) -> [Option<Rect>; ADDED_AXES] {
    let mut rects = [None; ADDED_AXES];
    let mut left_edge = plot_rect.left() - plot::Y_LABEL_GAP - primary - COLUMN_PAD;
    let mut right_edge = plot_rect.right();
    for (index, width) in widths.into_iter().enumerate() {
        if width <= 0.0 {
            continue;
        }
        let (left, right) = match column_side(index) {
            Side::Left => {
                left_edge -= width;
                (left_edge, left_edge + width)
            }
            Side::Right => {
                right_edge += width;
                (right_edge - width, right_edge)
            }
        };
        rects[index] = Some(Rect::from_min_max(
            pos2(left, plot_rect.top()),
            pos2(right, plot_rect.bottom()),
        ));
    }
    rects
}

/// The distinct colours of the curves an axis carries, in their order.
pub(crate) fn axis_colours(colours: impl IntoIterator<Item = Color32>) -> Vec<Color32> {
    let mut distinct: Vec<Color32> = Vec::new();
    for colour in colours {
        if !distinct.contains(&colour) {
            distinct.push(colour);
        }
    }
    distinct
}

/// The width a row of swatches for `count` colours takes beside a unit.
pub(crate) fn swatches_width(count: usize) -> f32 {
    count as f32 * (SWATCH_WIDTH + SWATCH_GAP)
}

/// Paint a swatch per colour in a row ending at `right` (or starting at
/// `left`), centred on `y`.
pub(crate) fn paint_swatches(
    painter: &egui::Painter,
    colours: &[Color32],
    (from, rightward): (f32, bool),
    y: f32,
) {
    for (index, colour) in colours.iter().enumerate() {
        let offset = SWATCH_GAP + index as f32 * (SWATCH_WIDTH + SWATCH_GAP);
        let left = if rightward {
            from + offset
        } else {
            from - offset - SWATCH_WIDTH
        };
        painter.hline(
            left..=left + SWATCH_WIDTH,
            y,
            egui::Stroke::new(2.0, *colour),
        );
    }
}

/// What the reader did with a strip's added axes this frame, and the room
/// they need.
#[derive(Debug, Default)]
pub(crate) struct AxesResponse {
    /// The range each axis the reader zoomed, panned or fitted now shows,
    /// by its slot: `None` to fit its curves again.
    pub ranges: Vec<(usize, Option<(f64, f64)>)>,
    /// What the strip's axes need beside its plot.
    pub room: AxisRoom,
}

/// One added axis as its column draws it.
struct Column {
    ladder: Vec<(f64, String)>,
    ink: Color32,
    swatches: Vec<Color32>,
    unit: String,
    width: f32,
}

/// Measure every axis of `axes` beside the plot spanning `plot_rect`, draw
/// those `gutter` has room for, and report what the reader did with them.
/// The first axis' labels take `primary` left of the plot; its unit is
/// centred on `caption_y`.
pub(crate) fn secondary_axes(
    ui: &mut Ui,
    model: &StripModel,
    axes: &[SecondaryPlot<'_>],
    (plot_rect, gutter, primary, caption_y): (Rect, Gutter, f32, f32),
    t: &Tokens,
) -> AxesResponse {
    let mut out = AxesResponse::default();
    out.room.primary = primary;
    if plot_rect.height() <= 1.0 || axes.is_empty() {
        return out;
    }
    let font = plot::y_tick_font();
    let measure = |text: &str| {
        ui.painter()
            .layout_no_wrap(text.to_owned(), font.clone(), Color32::PLACEHOLDER)
            .size()
            .x
    };
    let pitch = plot::y_tick_pitch(ui);
    let columns: Vec<Option<Column>> = axes
        .iter()
        .take(ADDED_AXES)
        .map(|axis| {
            let (low, high) = axis.range;
            if !(low.is_finite() && high.is_finite()) || high <= low {
                return None;
            }
            let colours = axis_colours(
                axis.traces
                    .iter()
                    .filter_map(|index| model.traces.get(*index).map(|trace| trace.color))
                    .chain(axis.expressions.iter().map(|expr| expr.color)),
            );
            let ladder = super::pane::secondary_ticks(plot_rect.height(), pitch, (low, high));
            // Ticks stated as offsets from an anchor would need the anchor
            // beside them, which the column has no room for: such a ladder
            // is labelled in full.
            let offset = ladder.offset_anchor().is_some();
            let ladder: Vec<(f64, String)> = ladder
                .ticks
                .into_iter()
                .map(|(value, label)| {
                    let label = if offset {
                        plot::fmt_si_significant(value, "", 4)
                    } else {
                        label
                    };
                    (value, label)
                })
                .collect();
            let unit = if axis.unit.is_empty() {
                "1".to_owned()
            } else {
                axis.unit.to_owned()
            };
            let (ink, swatches) = match colours.as_slice() {
                [only] => (*only, Vec::new()),
                _ => (t.color.canvas_text_dim, colours),
            };
            let caption = measure(&unit) + swatches_width(swatches.len());
            let labels = ladder
                .iter()
                .map(|(_, label)| measure(label))
                .fold(0.0, f32::max);
            Some(Column {
                ladder,
                ink,
                swatches,
                unit,
                width: plot::Y_LABEL_GAP + labels.max(caption) + COLUMN_PAD,
            })
        })
        .collect();
    for (index, column) in columns.iter().enumerate() {
        out.room.columns[index] = column.as_ref().map_or(0.0, |column| column.width);
    }
    // Only the axes the gutter made room for are drawn: an axis is never
    // squeezed into a column narrower than its labels.
    let mut widths = [0.0; ADDED_AXES];
    for (index, width) in widths.iter_mut().enumerate().take(gutter.kept) {
        *width = out.room.columns[index];
    }
    let rects = column_rects(plot_rect, primary, widths);
    let painter = ui.painter().clone();
    let pixels_per_point = painter.pixels_per_point();
    let hair = egui::Stroke::new(plot::hairline(1.0, pixels_per_point), t.color.canvas_frame);
    for (index, ((axis, column), rect)) in axes.iter().zip(&columns).zip(rects).enumerate() {
        let (Some(column), Some(rect)) = (column, rect) else {
            continue;
        };
        let (low, high) = axis.range;
        let side = column_side(index);
        // The hairline stands on the column's plot-side edge; ticks point
        // away from the plot and labels stand past them.
        let (spine_x, outward, align) = match side {
            Side::Right => (rect.left(), 1.0, egui::Align2::LEFT_CENTER),
            Side::Left => (rect.right(), -1.0, egui::Align2::RIGHT_CENTER),
        };
        painter.vline(spine_x, rect.y_range(), hair);
        let to_y =
            |value: f64| rect.bottom() - ((value - low) / (high - low)) as f32 * rect.height();
        let label_x = spine_x + outward * plot::Y_LABEL_GAP;
        for (value, label) in &column.ladder {
            let y = to_y(*value);
            if !(rect.top() - 0.5..=rect.bottom() + 0.5).contains(&y) {
                continue;
            }
            let tick_end = spine_x + outward * TICK_LENGTH;
            painter.hline(spine_x.min(tick_end)..=spine_x.max(tick_end), y, hair);
            painter.text(pos2(label_x, y), align, label, font.clone(), column.ink);
        }
        // The unit over the column on the first axis' caption row, and a
        // swatch of each curve beside it when the curves differ in colour.
        let caption = painter.text(
            pos2(label_x, caption_y),
            align,
            &column.unit,
            font.clone(),
            column.ink,
        );
        match side {
            Side::Right => paint_swatches(
                &painter,
                &column.swatches,
                (caption.right(), true),
                caption_y,
            ),
            Side::Left => paint_swatches(
                &painter,
                &column.swatches,
                (caption.left(), false),
                caption_y,
            ),
        }

        let id = egui::Id::new(("rspice.results.strip-axis", model.analysis_key, axis.slot));
        let response = ui
            .interact(rect, id, Sense::click_and_drag())
            .on_hover_cursor(egui::CursorIcon::ResizeVertical);
        let unit = if axis.unit.is_empty() {
            "plain-number".to_owned()
        } else {
            axis.unit.to_owned()
        };
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Slider, true, format!("{unit} axis"))
        });
        theme::paint_focus_ring(ui, &response, rect);
        let response = response.on_hover_text(format!(
            "The {unit} axis: scroll to zoom, drag to pan, double-click to fit its curves"
        ));
        let span = high - low;
        let per_point = span / f64::from(rect.height());
        if response.double_clicked() {
            out.ranges.push((axis.slot, None));
            continue;
        }
        if response.dragged() {
            let shift = f64::from(response.drag_delta().y) * per_point;
            if shift != 0.0 {
                out.ranges
                    .push((axis.slot, Some((low + shift, high + shift))));
                continue;
            }
        }
        if response.hovered() {
            let scroll = ui.input(|input| input.smooth_scroll_delta.y);
            if scroll != 0.0
                && let Some(pointer) = response.hover_pos()
            {
                // The axis takes the wheel: the strips under it do not
                // scroll as well.
                ui.input_mut(|input| input.smooth_scroll_delta.y = 0.0);
                let factor = (-f64::from(scroll) * ZOOM_PER_POINT).exp();
                let pivot = low + f64::from(rect.bottom() - pointer.y) * per_point;
                out.ranges.push((
                    axis.slot,
                    Some((
                        pivot - (pivot - low) * factor,
                        pivot + (high - pivot) * factor,
                    )),
                ));
                continue;
            }
        }
        if response.has_focus() {
            let (up, down) = ui.input(|input| {
                (
                    input.key_pressed(egui::Key::ArrowUp),
                    input.key_pressed(egui::Key::ArrowDown),
                )
            });
            let shift = match (up, down) {
                (true, false) => span * KEY_PAN_SHARE,
                (false, true) => -span * KEY_PAN_SHARE,
                _ => 0.0,
            };
            if shift != 0.0 {
                out.ranges
                    .push((axis.slot, Some((low + shift, high + shift))));
            }
        }
    }
    out
}
