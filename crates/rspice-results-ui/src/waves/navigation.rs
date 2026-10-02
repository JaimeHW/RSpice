//! Shared waveform X-axis drawing, overview navigation and local gesture state.

use super::{StripModel, trace_key};
use crate::derived::DerivedSeries;
use egui::Ui;
use rspice_app_types::quantity::QuantityPresentationPolicy;
use rspice_ui_kit::plot::{Axis, CursorPair, XScale};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};

pub const WAVE_SHARED_X_HEIGHT: f32 = 57.0;

/// The shared-X strip's three bands, offset from the strip's top edge.
///
/// They are ordered by what each one describes. Tick labels and the A/B
/// flags both read the panes' zoomed viewport, so they sit directly against
/// the panes; the full-range overview lane is the outermost band, and
/// nothing viewport-scaled is ever drawn across it.
const SHARED_X_LABEL_TOP: f32 = 3.0;
const SHARED_X_FLAG_TOP: f32 = 17.0;
const SHARED_X_FLAG_HEIGHT: f32 = 13.0;
const SHARED_X_LANE_TOP: f32 = 35.0;
const SHARED_X_LANE_HEIGHT: f32 = 14.0;

/// The narrowest viewport an overview-handle drag can produce, as a fraction
/// of the full retained sweep. It is the reciprocal of the zoom ceiling, so
/// dragging a handle cannot reach a magnification the zoom controls refuse.
const SHARED_X_MIN_WINDOW: f64 = 1.0 / 200.0;
/// Points the overview lane's mini-trace is drawn from, whatever the run
/// retained. The lane is fourteen pixels tall and a plot wide, so a denser
/// curve would not reach the reader as anything but the same grey band.
const SHARED_X_OVERVIEW_POINTS: usize = 160;
pub const WAVE_SHARED_RIGHT_MARGIN: f32 = 14.0;

pub fn model_x_axis(
    model: &StripModel,
    x0: f64,
    x1: f64,
    quantity_policy: QuantityPresentationPolicy,
) -> Axis {
    let axis = match model.x_scale {
        XScale::Log10 => Axis::log_decades(x0, x1, &model.x_unit),
        XScale::Linear => Axis::linear(x0, x1, &model.x_unit),
    }
    .with_label(&model.x_label);
    if model.x_unit == "Hz" {
        let (scale, offset, unit) = quantity_policy.frequency_axis_transform();
        axis.with_display_transform(scale, offset, unit)
    } else {
        axis
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SharedXDrag {
    Viewport,
    ResizeStart,
    ResizeEnd,
    CursorA,
    CursorB,
}

fn shared_x_drag_id(model: &StripModel) -> egui::Id {
    egui::Id::new(("rspice.results.shared-x-drag", model.analysis_key))
}

/// The host supplies an already qualified source, viewport and cursor domain.
pub struct SharedXAxis {
    pub full_domain: (f64, f64),
    pub current: (f64, f64),
    pub height: f32,
    pub left_margin: f32,
    pub quantity_policy: QuantityPresentationPolicy,
    pub cursors: Option<CursorPair>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SharedXViewChange {
    Fit,
    Range((f64, f64)),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CursorMove {
    A(f64),
    B(f64),
}

/// The host applies these actions to its shared panes and source-bound cursors.
#[derive(Debug, Default)]
pub struct SharedXAxisResponse {
    pub viewport: Option<SharedXViewChange>,
    pub cursor: Option<CursorMove>,
    /// Retained samples visited by this draw, including cache misses.
    pub overview_samples_read: usize,
    pub extrema_samples_read: usize,
}

/// Draw the shared axis and overview, resolving its pointer and keyboard gestures.
pub fn show_shared_x_axis(
    ui: &mut Ui,
    derived: &mut DerivedSeries,
    model: &StripModel,
    input: SharedXAxis,
) -> SharedXAxisResponse {
    let mut output = SharedXAxisResponse::default();
    let SharedXAxis {
        full_domain,
        current,
        height,
        cursors: cursor_values,
        ..
    } = input;
    if height <= 1.0 {
        return output;
    }
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let axis = model_x_axis(model, current.0, current.1, input.quantity_policy);
    let cursor_summary = cursor_values.map_or_else(
        || "No A/B cursors on this axis".to_owned(),
        |cursors| {
            let a = cursors.a.map_or_else(
                || "not placed".to_owned(),
                |value| axis.format_display_value(value),
            );
            let b = cursors.b.map_or_else(
                || "not placed".to_owned(),
                |value| axis.format_display_value(value),
            );
            format!("Cursor A {a}; cursor B {b}")
        },
    );
    let accessibility_label = format!(
        "Shared {} axis. Current range {} to {}. Full retained range {} to {}. {}. Drag the viewport window to pan or either of its edge handles to resize it, click the overview to recenter, use the wheel to zoom at the pointer, drag A or B to move a cursor, and press F to fit.",
        model.x_label(),
        axis.format_display_value(current.0),
        axis.format_display_value(current.1),
        axis.format_display_value(full_domain.0),
        axis.format_display_value(full_domain.1),
        cursor_summary,
    );
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::click_and_drag(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Image, true, accessibility_label.clone())
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::GraphicsDocument);
        node.set_label(accessibility_label.clone());
    });
    let painter = ui.painter().with_clip_rect(rect);
    painter.rect_filled(rect, 0.0, c.bg_panel);
    painter.hline(rect.x_range(), rect.top(), egui::Stroke::new(1.0, c.border));

    let plot_left = (rect.left() + input.left_margin).min(rect.right());
    let plot_right = (rect.right() - WAVE_SHARED_RIGHT_MARGIN).max(plot_left);
    let label_top = rect.top() + SHARED_X_LABEL_TOP;
    let flag_top = rect.top() + SHARED_X_FLAG_TOP;
    let flag_bottom = flag_top + SHARED_X_FLAG_HEIGHT;
    // The lane spans exactly the panes' plot area, so the overview sits
    // under the traces it mirrors and its viewport window maps 1:1 to what
    // the panes show. Both rows name themselves into the panes' own left
    // gutter, which states the thing two stacked X scales otherwise hide:
    // the ticks are the zoomed viewport, the bar below is the full sweep.
    let track = egui::Rect::from_min_max(
        egui::pos2(plot_left, rect.top() + SHARED_X_LANE_TOP),
        egui::pos2(
            plot_right,
            (rect.top() + SHARED_X_LANE_TOP + SHARED_X_LANE_HEIGHT).min(rect.bottom()),
        ),
    );
    let mut viewport = egui::Rect::NOTHING;
    if track.width() > 1.0 {
        painter.rect_filled(track, 0.0, c.canvas_bg);
        painter.rect_stroke(
            track,
            0.0,
            egui::Stroke::new(1.0, c.border),
            egui::StrokeKind::Inside,
        );
        if let Some(trace) = model
            .traces
            .iter()
            .find(|trace| trace.visible && !trace.overlay)
        {
            // The lane's vertical fit is the trace's own finite extremes,
            // which the pane axis above it already holds under this very key.
            // Folding them again here was a second full pass over the
            // retained samples on every frame, for a curve fourteen pixels
            // tall.
            let (minimum, maximum) = derived
                .range_or(trace_key(model, trace), || {
                    output.extrema_samples_read = trace.y.len();
                    rspice_results::measurements::finite_extremes(&trace.y)
                })
                .unwrap_or((0.0, 1.0));
            let span = (maximum - minimum).max(f64::EPSILON);
            // Index the stride instead of stepping an iterator over it:
            // `step_by` has only `Iterator::nth` to skip with, and a `Zip`
            // has no `nth` of its own, so reaching every 1 562nd sample of a
            // quarter-million-sample sweep visited all 250 000 of them.
            let paired = trace.x.len().min(trace.y.len());
            let stride = (trace.x.len() / SHARED_X_OVERVIEW_POINTS).max(1);
            output.overview_samples_read = paired.div_ceil(stride);
            let points = (0..paired)
                .step_by(stride)
                .filter_map(|index| {
                    let (x, y) = (trace.x[index], trace.y[index]);
                    let fraction = model.x_scale.normalize(x, full_domain.0, full_domain.1);
                    (fraction.is_finite() && y.is_finite()).then(|| {
                        egui::pos2(
                            track.left() + track.width() * fraction as f32,
                            track.bottom()
                                - 2.0
                                - ((y - minimum) / span) as f32 * (track.height() - 4.0),
                        )
                    })
                })
                .collect::<Vec<_>>();
            if points.len() >= 2 {
                painter.add(egui::Shape::line(
                    points,
                    egui::Stroke::new(1.0, trace.color.gamma_multiply(0.75)),
                ));
            }
        }

        let (start, end) = shared_axis_viewport_fraction(model.x_scale, full_domain, current);
        let window_left = track.left() + track.width() * start as f32;
        viewport = egui::Rect::from_min_max(
            egui::pos2(window_left, track.top()),
            egui::pos2(
                (track.left() + track.width() * end as f32).max(window_left + 6.0),
                track.bottom(),
            ),
        );
        painter.rect_filled(viewport, 0.0, c.accent.gamma_multiply(0.14));
        painter.rect_stroke(
            viewport,
            0.0,
            egui::Stroke::new(1.0, c.accent),
            egui::StrokeKind::Inside,
        );
        // Grab handles at both edges are the always-visible affordance for
        // resizing the visible span in place: drag an edge in from the full
        // range to zoom, and the opposite edge holds.
        for edge_x in [viewport.left(), viewport.right()] {
            painter.rect_filled(
                egui::Rect::from_min_max(
                    egui::pos2(edge_x - 3.0, track.top() - 1.0),
                    egui::pos2(edge_x + 3.0, track.bottom() + 1.0),
                ),
                2.0,
                c.accent,
            );
            painter.vline(
                edge_x,
                egui::Rangef::new(track.top() + 3.0, track.bottom() - 3.0),
                egui::Stroke::new(1.0, c.canvas_bg),
            );
        }
    }

    // Tick values for the zoomed viewport sit at the top of the strip, right
    // against the panes they describe, and their stubs point up toward the
    // plot.
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let mut last_right = f32::NEG_INFINITY;
    // Past a certain zoom the tick labels stop being values and become
    // differences from one. This strip draws its own tick row instead of
    // letting a plot draw one, so it also owes the reader that value: a row
    // reading "−40n … 0 … +40n" with nothing to subtract them from states a
    // window somewhere near zero, which is not where the reader is. Stated
    // once, at the head of the row and brighter than the ticks, exactly as
    // the plot's own axis states it.
    if let Some(anchor) = axis.offset_anchor() {
        let galley = painter.layout_no_wrap(anchor.to_owned(), font.clone(), c.text);
        last_right = plot_left + galley.size().x;
        painter.galley(egui::pos2(plot_left, label_top), galley, c.text);
    }
    for (value, label) in &axis.ticks {
        let fraction = model.x_scale.normalize(*value, axis.min, axis.max);
        if !fraction.is_finite() || !(0.0..=1.0).contains(&fraction) {
            continue;
        }
        let x = plot_left + (plot_right - plot_left) * fraction as f32;
        painter.vline(
            x,
            egui::Rangef::new(rect.top(), rect.top() + 3.0),
            egui::Stroke::new(1.0, c.border_strong),
        );
        let galley = painter.layout_no_wrap(label.clone(), font.clone(), c.text_dim);
        // The stub marks the true position; the label itself is held inside
        // the plot area so an edge value never reaches into the gutter
        // column and collides with the row's own name.
        let half = galley.size().x * 0.5 + 2.0;
        let centre = x.clamp(plot_left + half, (plot_right - half).max(plot_left + half));
        let left = centre - galley.size().x * 0.5;
        if left >= last_right + 6.0 {
            last_right = left + galley.size().x;
            painter.galley(egui::pos2(left, label_top), galley, c.text_dim);
        }
    }
    let gutter_font = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
    painter.text(
        egui::pos2(plot_left - 8.0, label_top + font.size * 0.5),
        egui::Align2::RIGHT_CENTER,
        shared_x_gutter_label(&model.x_unit),
        gutter_font.clone(),
        c.text_faint,
    );
    painter.text(
        egui::pos2(plot_left - 8.0, track.center().y),
        egui::Align2::RIGHT_CENTER,
        "FULL",
        gutter_font,
        c.text_faint,
    );

    let flag_centre = |value: f64| {
        // Flags track the viewport like the pane cursor lines above them,
        // so a cursor zoomed past simply leaves the strip.
        let fraction = model.x_scale.normalize(value, current.0, current.1);
        (fraction.is_finite() && (-0.001..=1.001).contains(&fraction))
            .then(|| plot_left + (plot_right - plot_left) * fraction.clamp(0.0, 1.0) as f32)
    };
    if let Some(cursors) = cursor_values {
        for (label, value, color) in [("A", cursors.a, c.traces[1]), ("B", cursors.b, c.accent)] {
            let Some(value) = value else { continue };
            if let Some(px) = flag_centre(value) {
                let flag = egui::Rect::from_min_max(
                    egui::pos2(px - 8.0, flag_top),
                    egui::pos2(px + 8.0, flag_bottom),
                );
                painter.vline(
                    px,
                    egui::Rangef::new(rect.top(), flag_top),
                    egui::Stroke::new(1.0, color),
                );
                painter.rect_filled(flag, 2.0, color);
                painter.text(
                    flag.center(),
                    egui::Align2::CENTER_CENTER,
                    label,
                    theme::mono(tokens::FS_0, FontWeight::SemiBold),
                    c.canvas_bg,
                );
            }
            // The overview keeps its own notch at the cursor's absolute
            // source position, so it still locates a cursor the viewport has
            // left behind.
            if track.width() > 1.0 {
                let fraction = model.x_scale.normalize(value, full_domain.0, full_domain.1);
                if fraction.is_finite() {
                    let notch = track.left() + track.width() * fraction.clamp(0.0, 1.0) as f32;
                    painter.vline(
                        notch,
                        egui::Rangef::new(track.top() + 1.0, track.top() + 5.0),
                        egui::Stroke::new(2.0, color),
                    );
                }
            }
        }
    }

    if response.clicked() {
        response.request_focus();
    }
    // The lane band owns the overview gestures and everything above it
    // belongs to the flags, so a flag grab never fights a resize handle.
    let lane_band = egui::Rangef::new(track.top() - 4.0, track.bottom() + 4.0);
    let handle_at = |pointer: egui::Pos2| {
        if track.width() <= 1.0 || !lane_band.contains(pointer.y) {
            return None;
        }
        if (pointer.x - viewport.left()).abs() <= 5.0 {
            Some(SharedXDrag::ResizeStart)
        } else if (pointer.x - viewport.right()).abs() <= 5.0 {
            Some(SharedXDrag::ResizeEnd)
        } else {
            None
        }
    };
    let flag_at = |pointer: egui::Pos2| {
        if pointer.y >= track.top() - 4.0 {
            return None;
        }
        let cursors = cursor_values?;
        [
            (SharedXDrag::CursorA, cursors.a),
            (SharedXDrag::CursorB, cursors.b),
        ]
        .into_iter()
        .find_map(|(drag, value)| {
            let px = flag_centre(value?)?;
            ((pointer.x - px).abs() <= 9.0).then_some(drag)
        })
    };
    let drag_id = shared_x_drag_id(model);
    if response.drag_started()
        && let Some(pointer) = response.interact_pointer_pos()
    {
        let drag = handle_at(pointer)
            .or_else(|| flag_at(pointer))
            .unwrap_or(SharedXDrag::Viewport);
        ui.memory_mut(|memory| memory.data.insert_temp(drag_id, drag));
    }
    let drag = ui.memory(|memory| memory.data.get_temp::<SharedXDrag>(drag_id));
    if response.dragged()
        && let Some(pointer) = response.interact_pointer_pos()
        && track.width() > 1.0
        && let Some(drag) = drag
    {
        let fraction = f64::from(((pointer.x - track.left()) / track.width()).clamp(0.0, 1.0));
        match drag {
            SharedXDrag::Viewport => {
                let delta = ui.ctx().input(|input| input.pointer.delta().x);
                if let Some(range) = panned_shared_x_view(
                    model.x_scale,
                    full_domain,
                    current,
                    f64::from(delta / track.width()),
                ) {
                    output.viewport = Some(SharedXViewChange::Range(range));
                }
            }
            SharedXDrag::ResizeStart | SharedXDrag::ResizeEnd => {
                if let Some(range) = resized_shared_x_view(
                    model.x_scale,
                    full_domain,
                    current,
                    drag == SharedXDrag::ResizeStart,
                    fraction,
                ) {
                    output.viewport = Some(SharedXViewChange::Range(range));
                }
            }
            SharedXDrag::CursorA | SharedXDrag::CursorB => {
                // A flag is anchored to what the panes show, so its drag
                // converts through the viewport rather than the full sweep.
                let view_fraction = f64::from(
                    ((pointer.x - plot_left) / (plot_right - plot_left).max(1.0)).clamp(0.0, 1.0),
                );
                let x = model
                    .x_scale
                    .denormalize(view_fraction, current.0, current.1);
                output.cursor = match drag {
                    SharedXDrag::CursorA => Some(CursorMove::A(x)),
                    SharedXDrag::CursorB => Some(CursorMove::B(x)),
                    SharedXDrag::Viewport | SharedXDrag::ResizeStart | SharedXDrag::ResizeEnd => {
                        None
                    }
                };
            }
        }
    }
    if response.drag_stopped() {
        ui.memory_mut(|memory| memory.data.remove::<SharedXDrag>(drag_id));
    }

    if response.hovered()
        && let Some(pointer) = response.hover_pos()
    {
        ui.ctx().set_cursor_icon(
            if handle_at(pointer).is_some() || flag_at(pointer).is_some() {
                egui::CursorIcon::ResizeHorizontal
            } else if lane_band.contains(pointer.y) {
                egui::CursorIcon::Grab
            } else {
                egui::CursorIcon::Default
            },
        );
    }

    if response.clicked()
        && let Some(pointer) = response.interact_pointer_pos()
        && track.contains(pointer)
        && !viewport.contains(pointer)
        && let Some(range) = recentered_shared_x_view(
            model.x_scale,
            full_domain,
            current,
            f64::from((pointer.x - track.left()) / track.width()),
        )
    {
        output.viewport = Some(SharedXViewChange::Range(range));
    }

    if response.hovered() && track.width() > 1.0 {
        let scroll = ui.input(|input| input.smooth_scroll_delta.y);
        if scroll != 0.0
            && let Some(pointer) = response.hover_pos()
            && let Some(range) = zoomed_shared_x_view(
                model.x_scale,
                full_domain,
                current,
                f64::from((pointer.x - track.left()) / track.width()),
                (f64::from(-scroll) * 0.002).exp(),
            )
        {
            ui.input_mut(|input| input.smooth_scroll_delta = egui::Vec2::ZERO);
            output.viewport = Some(SharedXViewChange::Range(range));
        }
    }

    if response.has_focus() {
        let fit_key =
            ui.input(|input| input.key_pressed(egui::Key::F) || input.key_pressed(egui::Key::Home));
        if fit_key {
            output.viewport = Some(SharedXViewChange::Fit);
        }
        let zoom_in = ui.input(|input| input.key_pressed(egui::Key::Plus));
        let zoom_out = ui.input(|input| input.key_pressed(egui::Key::Minus));
        if (zoom_in || zoom_out)
            && let Some(range) = zoomed_shared_x_view(
                model.x_scale,
                full_domain,
                current,
                0.5,
                if zoom_in { 0.8 } else { 1.25 },
            )
        {
            output.viewport = Some(SharedXViewChange::Range(range));
        }
        let direction = ui.input(|input| {
            if input.key_pressed(egui::Key::ArrowLeft) {
                -1.0
            } else if input.key_pressed(egui::Key::ArrowRight) {
                1.0
            } else {
                0.0
            }
        });
        if direction != 0.0
            && let Some(range) =
                panned_shared_x_view(model.x_scale, full_domain, current, direction * 0.05)
        {
            output.viewport = Some(SharedXViewChange::Range(range));
        }
    }

    if response.double_clicked() {
        output.viewport = Some(SharedXViewChange::Fit);
    }
    theme::paint_focus_ring(ui, &response, rect);
    response.on_hover_text(
        "Shared X overview — drag the window or its edges, click to recenter, wheel to zoom, drag A/B, F to fit",
    );
    output
}

pub fn shared_axis_viewport_fraction(
    scale: XScale,
    full: (f64, f64),
    view: (f64, f64),
) -> (f64, f64) {
    let start = scale.normalize(view.0, full.0, full.1).clamp(0.0, 1.0);
    let end = scale.normalize(view.1, full.0, full.1).clamp(0.0, 1.0);
    (start.min(end), start.max(end))
}

pub fn panned_shared_x_view(
    scale: XScale,
    full: (f64, f64),
    view: (f64, f64),
    fraction_delta: f64,
) -> Option<(f64, f64)> {
    if !fraction_delta.is_finite() {
        return None;
    }
    let (start, end) = shared_axis_viewport_fraction(scale, full, view);
    let width = end - start;
    if !(width > 0.0 && width < 1.0) {
        return None;
    }
    let next_start = (start + fraction_delta).clamp(0.0, 1.0 - width);
    let next_end = next_start + width;
    Some((
        scale.denormalize(next_start, full.0, full.1),
        scale.denormalize(next_end, full.0, full.1),
    ))
}

pub fn zoomed_shared_x_view(
    scale: XScale,
    full: (f64, f64),
    view: (f64, f64),
    anchor_fraction: f64,
    factor: f64,
) -> Option<(f64, f64)> {
    if !anchor_fraction.is_finite() || !factor.is_finite() || factor <= 0.0 {
        return None;
    }
    let (start, end) = shared_axis_viewport_fraction(scale, full, view);
    let width = end - start;
    if width <= 0.0 {
        return None;
    }
    let anchor = anchor_fraction.clamp(0.0, 1.0);
    let relative = ((anchor - start) / width).clamp(0.0, 1.0);
    let next_width = (width * factor).clamp(1.0e-6, 1.0);
    let next_start = (anchor - relative * next_width).clamp(0.0, 1.0 - next_width);
    let next_end = next_start + next_width;
    Some((
        scale.denormalize(next_start, full.0, full.1),
        scale.denormalize(next_end, full.0, full.1),
    ))
}

/// Resize the shared viewport by dragging one edge of the overview window.
///
/// The dragged edge follows the pointer and the opposite edge stays fixed,
/// so the gesture zooms and pans in one motion the way pulling a scrollbar
/// handle does.
pub fn resized_shared_x_view(
    scale: XScale,
    full: (f64, f64),
    view: (f64, f64),
    move_start: bool,
    edge_fraction: f64,
) -> Option<(f64, f64)> {
    if !edge_fraction.is_finite() {
        return None;
    }
    let (start, end) = shared_axis_viewport_fraction(scale, full, view);
    let edge = edge_fraction.clamp(0.0, 1.0);
    let (next_start, next_end) = if move_start {
        (edge.min(end - SHARED_X_MIN_WINDOW), end)
    } else {
        (start, edge.max(start + SHARED_X_MIN_WINDOW))
    };
    let next_start = next_start.clamp(0.0, 1.0 - SHARED_X_MIN_WINDOW);
    let next_end = next_end.clamp(next_start + SHARED_X_MIN_WINDOW, 1.0);
    Some((
        scale.denormalize(next_start, full.0, full.1),
        scale.denormalize(next_end, full.0, full.1),
    ))
}

/// The name the strip prints into the panes' left gutter beside the tick
/// values, following the mockup's axis vocabulary.
///
/// The tick values themselves are SI-prefixed bare numbers, so the gutter is
/// where the axis states its unit — once, for the whole row.
pub fn shared_x_gutter_label(unit: &str) -> String {
    let name = match unit {
        "s" => "TIME",
        "Hz" => "FREQ",
        _ => "X",
    };
    if unit.is_empty() {
        name.to_owned()
    } else {
        format!("{name} · {unit}")
    }
}

pub fn recentered_shared_x_view(
    scale: XScale,
    full: (f64, f64),
    view: (f64, f64),
    centre_fraction: f64,
) -> Option<(f64, f64)> {
    let (start, end) = shared_axis_viewport_fraction(scale, full, view);
    let width = end - start;
    if !(width > 0.0 && width < 1.0 && centre_fraction.is_finite()) {
        return None;
    }
    let next_start = (centre_fraction.clamp(0.0, 1.0) - width * 0.5).clamp(0.0, 1.0 - width);
    Some((
        scale.denormalize(next_start, full.0, full.1),
        scale.denormalize(next_start + width, full.0, full.1),
    ))
}

#[cfg(test)]
mod tests;
