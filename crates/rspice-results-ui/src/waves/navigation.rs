//! Shared waveform X-axis drawing, overview navigation and local gesture state.

use super::{StripModel, trace_key};
use crate::derived::DerivedSeries;
use egui::Ui;
use rspice_ui_kit::plot::{Axis, CursorPair, XScale};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;

mod jump;
pub use jump::{CursorJump, crossing_level, jump_cursor, jump_trace};

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

/// Space either side of the value inside a cursor's ruler box, and the least
/// gap between two boxes.
const RULER_FLAG_PAD: f32 = 4.0;

/// How far a cursor's one-pixel caret hangs under its ruler box, short of
/// the overview track below.
const RULER_CARET: f32 = 2.0;

/// The height of a cursor's notch on the overview track's top edge.
const RULER_NOTCH: f32 = 3.0;

/// Half the width of a grab handle at either end of the overview's thumb.
const THUMB_HANDLE_HALF: f32 = 3.0;

/// A cursor's value box on the shared ruler, which is also its drag handle.
struct RulerFlag {
    drag: SharedXDrag,
    /// Where the cursor itself stands.
    px: f32,
    rect: egui::Rect,
    galley: std::sync::Arc<egui::Galley>,
    color: egui::Color32,
}

/// The left edges of the cursors' ruler boxes, each `(cursor x, width)`:
/// centred under its cursor inside `span`, and two that would touch pushed
/// apart by halves, an edge holding the one against it while the other takes
/// the rest.
fn ruler_flag_lefts(flags: &[(f32, f32)], span: egui::Rangef) -> Vec<f32> {
    let mut lefts: Vec<f32> = flags
        .iter()
        .map(|&(px, width)| (px - width * 0.5).clamp(span.min, (span.max - width).max(span.min)))
        .collect();
    if let [(first_px, _), (second_px, _)] = flags {
        let (first, second) = if first_px <= second_px {
            (0, 1)
        } else {
            (1, 0)
        };
        let overlap = lefts[first] + flags[first].1 + RULER_FLAG_PAD - lefts[second];
        if overlap > 0.0 {
            lefts[first] -= overlap * 0.5;
            lefts[second] += overlap * 0.5;
            let under = span.min - lefts[first];
            if under > 0.0 {
                lefts[first] += under;
                lefts[second] += under;
            }
            let over = lefts[second] + flags[second].1 - span.max;
            if over > 0.0 {
                lefts[first] -= over;
                lefts[second] -= over;
            }
        }
    }
    lefts
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
    /// The reader's significant digits, which a cursor's value box reads in.
    pub significant_digits: usize,
    pub cursors: Option<CursorPair>,
    /// Why the axis cannot be logarithmic, when it cannot.
    pub log_refusal: Option<String>,
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
    /// Retained sample visits charged to this draw, including cache construction.
    pub overview_samples_read: usize,
    pub extrema_samples_read: usize,
    /// The reader switched the axis between linear and logarithmic.
    pub scale_toggled: bool,
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
    // The lane band owns the overview gestures and everything above it
    // belongs to the flags, so a flag grab never fights a resize handle.
    let lane_band = egui::Rangef::new(track.top() - 4.0, track.bottom() + 4.0);
    // Whether the panes show less than the whole record: the thumb takes the
    // accent and the cursors get their notches only then.
    let (shown_start, shown_end) =
        shared_axis_viewport_fraction(model.x_scale, full_domain, current);
    let zoomed = shown_start > 1.0e-9 || shown_end < 1.0 - 1.0e-9;
    let mut viewport = egui::Rect::NOTHING;
    let mut handle_centres = [f32::NAN; 2];
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
            // Cache bounded source knots with explicit gap separators. Both
            // extrema and gap discovery happen only when the source changes.
            let indices = derived.overview_indices_or(trace_key(model, trace), || {
                output.overview_samples_read = trace.y.len().saturating_mul(2);
                rspice_results::waveform::RetainedWaveform::new(
                    "overview",
                    trace.x.clone(),
                    trace.y.clone(),
                )
                .display_sample_indices(SHARED_X_OVERVIEW_POINTS)
            });
            output.overview_samples_read += indices.len();
            for segment in overview_segments(&trace.x, &trace.y, &indices) {
                let points = segment
                    .into_iter()
                    // t = 0 has no place on a logarithmic axis: the miniature
                    // starts at the first positive sample, as the panes do.
                    .filter(|(x, _)| model.x_scale == XScale::Linear || *x > 0.0)
                    .filter_map(|(x, y)| {
                        let fraction = model.x_scale.normalize(x, full_domain.0, full_domain.1);
                        let point = egui::pos2(
                            track.left() + track.width() * fraction as f32,
                            track.bottom()
                                - 2.0
                                - ((y - minimum) / span) as f32 * (track.height() - 4.0),
                        );
                        point.is_finite().then_some(point)
                    })
                    .collect::<Vec<_>>();
                if points.len() >= 2 {
                    painter.add(egui::Shape::line(
                        points,
                        egui::Stroke::new(1.0, trace.color.gamma_multiply(0.75)),
                    ));
                }
            }
        }

        let window_left = track.left() + track.width() * shown_start as f32;
        viewport = egui::Rect::from_min_max(
            egui::pos2(window_left, track.top()),
            egui::pos2(
                (track.left() + track.width() * shown_end as f32).max(window_left + 6.0),
                track.bottom(),
            ),
        );
        // The thumb is a window with two ends to pull at full view too, as
        // a range slider at full range is. Zoomed in, it takes the accent.
        // At full view it is a 1 px frame in the secondary text ink, whose
        // handles take the accent while the pointer is over the track: an
        // amber thumb across the whole track would say nothing, and no
        // thumb at all hid that its ends pull in to zoom.
        let lane_hovered = response
            .hover_pos()
            .is_some_and(|pointer| lane_band.contains(pointer.y));
        let (frame_ink, handle_ink) = if zoomed {
            (c.accent, c.accent)
        } else if lane_hovered {
            (c.text_dim, c.accent)
        } else {
            (c.text_dim, c.text_dim)
        };
        if zoomed {
            painter.rect_filled(viewport, 0.0, c.accent.gamma_multiply(0.14));
        }
        painter.rect_stroke(
            viewport,
            0.0,
            egui::Stroke::new(1.0, frame_ink),
            egui::StrokeKind::Inside,
        );
        // A grab handle at each end resizes the visible span in place: drag
        // one and the opposite end holds. Each stays inside the track, so a
        // thumb against an end never hangs its handle into the gutter.
        handle_centres = [viewport.left(), viewport.right()].map(|edge| {
            edge.clamp(
                track.left() + THUMB_HANDLE_HALF,
                (track.right() - THUMB_HANDLE_HALF).max(track.left() + THUMB_HANDLE_HALF),
            )
        });
        for centre in handle_centres {
            painter.rect_filled(
                egui::Rect::from_min_max(
                    egui::pos2(centre - THUMB_HANDLE_HALF, track.top() - 1.0),
                    egui::pos2(centre + THUMB_HANDLE_HALF, track.bottom() + 1.0),
                ),
                2.0,
                handle_ink,
            );
            painter.vline(
                centre,
                egui::Rangef::new(track.top() + 3.0, track.bottom() - 3.0),
                egui::Stroke::new(1.0, c.canvas_bg),
            );
        }
    }

    let flag_centre = |value: f64| {
        // Flags track the viewport like the pane cursor lines above them,
        // so a cursor zoomed past simply leaves the strip.
        let fraction = model.x_scale.normalize(value, current.0, current.1);
        (fraction.is_finite() && (-0.001..=1.001).contains(&fraction))
            .then(|| plot_left + (plot_right - plot_left) * fraction.clamp(0.0, 1.0) as f32)
    };
    // Each cursor's foot on the ruler is its X value in its own colour, as
    // EZwave's cursor bar and Verdi's ruler read the cursor time: the pane's
    // top flag already names the letter. Laid out before the tick row, so a
    // tick value over a box gives way to it.
    let flags = cursor_values.map_or_else(Vec::new, |cursors| {
        let placed: Vec<(SharedXDrag, &str, f32, String, egui::Color32)> = [
            (SharedXDrag::CursorA, "A", cursors.a, c.canvas_cursor_a),
            (SharedXDrag::CursorB, "B", cursors.b, c.canvas_cursor_b),
        ]
        .into_iter()
        .filter_map(|(drag, letter, value, color)| {
            let px = flag_centre(value?)?;
            let text = model.format_x(value?, input.significant_digits, input.quantity_policy);
            Some((drag, letter, px, text, color))
        })
        .collect();
        let flag_font = theme::mono(tokens::FS_0, FontWeight::SemiBold);
        let lay_out = |tagged: bool| {
            let galleys: Vec<_> = placed
                .iter()
                .map(|(_, letter, _, text, _)| {
                    let text = if tagged {
                        format!("{letter} {text}")
                    } else {
                        text.clone()
                    };
                    painter.layout_no_wrap(text, flag_font.clone(), c.canvas_bg)
                })
                .collect();
            let spans: Vec<(f32, f32)> = placed
                .iter()
                .zip(&galleys)
                .map(|((_, _, px, _, _), galley)| (*px, galley.size().x + 2.0 * RULER_FLAG_PAD))
                .collect();
            let lefts = ruler_flag_lefts(&spans, egui::Rangef::new(plot_left, plot_right));
            let under_its_cursor = spans
                .iter()
                .zip(&lefts)
                .all(|((px, width), left)| (*left..=*left + *width).contains(px));
            (galleys, spans, lefts, under_its_cursor)
        };
        // The letter rides along only while each box still stands under its
        // own cursor; two cursors close together drop it to make room.
        let tagged = lay_out(true);
        let (galleys, spans, lefts, _) = if tagged.3 { tagged } else { lay_out(false) };
        placed
            .iter()
            .zip(galleys)
            .zip(spans.iter().zip(&lefts))
            .map(
                |(((drag, _, px, _, color), galley), ((_, width), left))| RulerFlag {
                    drag: *drag,
                    px: *px,
                    rect: egui::Rect::from_min_max(
                        egui::pos2(*left, flag_top),
                        egui::pos2(*left + *width, flag_bottom),
                    ),
                    galley,
                    color: *color,
                },
            )
            .collect::<Vec<_>>()
    });

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
        // A tick value in a cursor box's column gives way to the box: the
        // two would read as one number.
        let label = egui::Rangef::new(left, left + galley.size().x);
        let under_a_flag = flags
            .iter()
            .any(|flag| flag.rect.x_range().expand(RULER_FLAG_PAD).intersects(label));
        if left >= last_right + 6.0 && !under_a_flag {
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
        gutter_font.clone(),
        c.text_faint,
    );
    // The scale toggle sits in the gutter between the two rows' names. A
    // domain a logarithmic axis cannot show refuses it with the reason.
    let (scale_text, other) = match model.x_scale {
        XScale::Log10 => ("LOG", "linear"),
        XScale::Linear => ("LIN", "logarithmic"),
    };
    let refused = model.x_scale == XScale::Linear && input.log_refusal.is_some();
    let scale_galley = painter.layout_no_wrap(scale_text.to_owned(), gutter_font, c.text_dim);
    let scale_rect = egui::Rect::from_min_size(
        egui::pos2(
            plot_left - 8.0 - scale_galley.size().x,
            flag_top + (SHARED_X_FLAG_HEIGHT - scale_galley.size().y) * 0.5,
        ),
        scale_galley.size(),
    )
    .expand(2.0);
    if scale_rect.left() > rect.left() {
        let scale_response = ui
            .interact(
                scale_rect,
                egui::Id::new(("rspice.results.x-scale", model.analysis_key)),
                egui::Sense::click(),
            )
            .on_hover_text(match &input.log_refusal {
                Some(reason) if refused => {
                    format!("A logarithmic X axis is not available: {reason}")
                }
                _ => format!("X axis {}; click for {other}", scale_text.to_lowercase()),
            });
        scale_response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Button,
                !refused,
                format!("X scale {scale_text}"),
            )
        });
        let colour = if refused {
            c.text_faint
        } else if scale_response.hovered() || model.x_scale == XScale::Log10 {
            c.accent
        } else {
            c.text_dim
        };
        if scale_response.hovered() && !refused {
            painter.rect_stroke(
                scale_rect,
                2.0,
                egui::Stroke::new(1.0, c.border),
                egui::StrokeKind::Inside,
            );
        }
        painter.galley(scale_rect.min + egui::vec2(2.0, 2.0), scale_galley, colour);
        output.scale_toggled = scale_response.clicked() && !refused;
    }

    // The same fills and ink as the plot's own cursor flags, so one cursor
    // is one colour everywhere and its value reads off the fill whichever
    // interface theme frames the canvas. The box is all the ruler draws of
    // a cursor: a line rising from it would read as a cursor on whatever
    // axis the pane above it draws.
    for flag in &flags {
        painter.rect_filled(flag.rect, 2.0, flag.color);
        painter.galley(
            flag.rect.center() - flag.galley.size() * 0.5,
            flag.galley.clone(),
            c.canvas_bg,
        );
        // A one-pixel caret under the box stands at the cursor's exact X, so
        // a box its neighbour pushed aside still points at its cursor.
        painter.vline(
            flag.px,
            egui::Rangef::new(flag.rect.bottom(), flag.rect.bottom() + RULER_CARET),
            egui::Stroke::new(1.0, flag.color),
        );
    }
    // The overview's notches say where a cursor stands in the whole record,
    // which the boxes above already say while the viewport shows all of it:
    // only a zoomed viewport gets them, each a short tick on the track's top
    // edge, so a cursor the viewport has left behind is still found.
    if let Some(cursors) = cursor_values
        && zoomed
        && track.width() > 1.0
    {
        for (value, color) in [
            (cursors.a, c.canvas_cursor_a),
            (cursors.b, c.canvas_cursor_b),
        ] {
            let Some(value) = value else { continue };
            let fraction = model.x_scale.normalize(value, full_domain.0, full_domain.1);
            if fraction.is_finite() {
                let notch = track.left() + track.width() * fraction.clamp(0.0, 1.0) as f32;
                painter.vline(
                    notch,
                    egui::Rangef::new(track.top(), track.top() + RULER_NOTCH),
                    egui::Stroke::new(1.0, color),
                );
            }
        }
    }

    if response.clicked() {
        response.request_focus();
    }
    let handle_at = |pointer: egui::Pos2| {
        if track.width() <= 1.0 || !lane_band.contains(pointer.y) {
            return None;
        }
        if (pointer.x - handle_centres[0]).abs() <= 5.0 {
            Some(SharedXDrag::ResizeStart)
        } else if (pointer.x - handle_centres[1]).abs() <= 5.0 {
            Some(SharedXDrag::ResizeEnd)
        } else {
            None
        }
    };
    let flag_at = |pointer: egui::Pos2| {
        if pointer.y >= track.top() - 4.0 {
            return None;
        }
        flags
            .iter()
            .find(|flag| {
                flag.rect.x_range().expand(2.0).contains(pointer.x)
                    || (pointer.x - flag.px).abs() <= 4.0
            })
            .map(|flag| flag.drag)
    };
    let drag_id = shared_x_drag_id(model);
    // What a drag takes hold of is decided where the button went down: egui
    // starts a drag only once the pointer has moved past the click distance,
    // by when it has left a handle's few pixels.
    if response.drag_started()
        && let Some(pointer) = ui
            .input(|input| input.pointer.press_origin())
            .or_else(|| response.interact_pointer_pos())
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
        "Drag an end to zoom in \u{00b7} drag the window to pan \u{00b7} click to recenter \u{00b7} \
         wheel to zoom \u{00b7} double-click or F for the full view \u{00b7} drag a cursor's box \
         to move it",
    );
    output
}

fn overview_segments(x: &[f64], y: &[f64], indices: &[usize]) -> Vec<Vec<(f64, f64)>> {
    let mut segments = Vec::new();
    let mut segment = Vec::new();
    for &index in indices {
        if x[index].is_finite() && y[index].is_finite() {
            segment.push((x[index], y[index]));
        } else if !segment.is_empty() {
            segments.push(std::mem::take(&mut segment));
        }
    }
    if !segment.is_empty() {
        segments.push(segment);
    }
    segments
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
