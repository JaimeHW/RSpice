//! Shared waveform X-axis drawing, overview navigation and local gesture state.

use super::{StripModel, StripTrace, trace_key};
use crate::derived::DerivedSeries;
use egui::Ui;
use rspice_results::waveform::Fetched;
use rspice_ui_kit::plot::{self, Axis, CursorPair, XScale, floored_span};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;

mod jump;
pub use jump::{
    CursorJump, JumpSearch, crossing_level, jump_cursor, jump_trace, land_jump,
    place_stepping_cursor, prepare_jump, stepping_from,
};

/// The ruler ends under its overview lane by the room the lane keeps above
/// it, so the lane stands centred in the band below the tick row rather than
/// against the strip's bottom edge.
pub const WAVE_SHARED_X_HEIGHT: f32 = SHARED_X_LANE_TOP + SHARED_X_LANE_HEIGHT + SHARED_X_LANE_GAP;

/// The shared-X strip's two bands, offset from the strip's top edge.
///
/// They are ordered by what each one describes. The tick row reads the
/// panes' zoomed viewport, so it sits directly against the panes, and the
/// A/B boxes stand in it, in the place of the tick values they cover, as a
/// marker's value does on an instrument's axis. A row of their own sat empty
/// whenever no cursor was placed. The full-range overview lane is the
/// outermost band, with equal room above and below it, and nothing
/// viewport-scaled is ever drawn across it.
const SHARED_X_FLAG_HEIGHT: f32 = 13.0;
/// A box hangs from its cursor's line, a lead under the ruler's top edge.
const SHARED_X_FLAG_TOP: f32 = RULER_LEAD;
/// The room above and below the overview lane.
const SHARED_X_LANE_GAP: f32 = 5.0;
const SHARED_X_LANE_TOP: f32 = plot::X_AXIS_ROW_HEIGHT + SHARED_X_LANE_GAP;
const SHARED_X_LANE_HEIGHT: f32 = 14.0;

/// Points the overview lane's mini-trace is drawn from, whatever the run
/// retained. The lane is fourteen pixels tall and a plot wide, so a denser
/// curve would not reach the reader as anything but the same grey band.
const SHARED_X_OVERVIEW_POINTS: usize = 160;
/// The height of a ruler with no overview track: its row of ticks, which
/// holds the cursor boxes too, as tall as every plot's abscissa row.
pub const WAVE_RULER_HEIGHT_WITHOUT_OVERVIEW: f32 = plot::X_AXIS_ROW_HEIGHT;

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

/// How far a cursor's line runs on from the pane above it, across the
/// ruler's top edge, to reach its box.
const RULER_LEAD: f32 = 2.0;

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
/// each where [`ruler_flag_left`] stands it clear of the row's `values`, and
/// two that would touch pushed apart by halves, an edge holding the one
/// against it while the other takes the rest.
fn ruler_flag_lefts(flags: &[(f32, f32)], span: egui::Rangef, values: &[egui::Rangef]) -> Vec<f32> {
    let mut lefts: Vec<f32> = flags
        .iter()
        .map(|&(px, width)| ruler_flag_left(px, width, span, values))
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

/// Where a box `width` wide under a cursor at `px` stands in `span`: centred
/// under it, or slid along the row the least that keeps it a pad off every
/// tick value in `values` while its cursor still meets its top edge clear of
/// a corner, so the line still runs into the box. A cursor standing on a
/// value no slide clears keeps its box centred, and that value gives way.
fn ruler_flag_left(px: f32, width: f32, span: egui::Rangef, values: &[egui::Rangef]) -> f32 {
    let last = (span.max - width).max(span.min);
    let centred = (px - width * 0.5).clamp(span.min, last);
    let reach = egui::Rangef::new(
        (px + RULER_FLAG_PAD - width).max(span.min),
        (px - RULER_FLAG_PAD).min(last),
    );
    if reach.min > reach.max {
        return centred;
    }
    // The lefts at which the box would come within a pad of a value.
    let blocked: Vec<(f32, f32)> = values
        .iter()
        .map(|value| {
            (
                value.min - RULER_FLAG_PAD - width,
                value.max + RULER_FLAG_PAD,
            )
        })
        .collect();
    let clear = |left: f32| blocked.iter().all(|&(from, to)| left <= from || left >= to);
    std::iter::once(centred.clamp(reach.min, reach.max))
        .chain(blocked.iter().flat_map(|&(from, to)| [from, to]))
        .filter(|left| reach.contains(*left) && clear(*left))
        .min_by(|a, b| (a - centred).abs().total_cmp(&(b - centred).abs()))
        .unwrap_or(centred)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SharedXDrag {
    Viewport,
    ResizeStart,
    ResizeEnd,
    CursorA,
    CursorB,
}

/// The host supplies an already qualified source, viewport and cursor domain.
pub struct SharedXAxis {
    pub full_domain: (f64, f64),
    pub current: (f64, f64),
    pub height: f32,
    pub left_margin: f32,
    /// What the panes keep right of their plots, which the ruler keeps too
    /// so its ticks stand under theirs.
    pub right_margin: f32,
    pub quantity_policy: QuantityPresentationPolicy,
    /// The reader's significant digits, which a cursor's value box reads in.
    pub significant_digits: usize,
    pub cursors: Option<CursorPair>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SharedXViewChange {
    Fit,
    Range((f64, f64)),
    /// Keep a running transient's newest rows in view as they arrive.
    Follow,
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
    /// Where the gutter caption stands, for a host that offers a choice on
    /// it.
    pub caption_rect: Option<egui::Rect>,
}

/// The record a ruler's overview track mirrors: the strip, whose first
/// visible curve the track draws in miniature, and the cache that miniature
/// is read from.
pub(crate) struct Overview<'a> {
    pub derived: &'a mut DerivedSeries,
    pub model: &'a StripModel,
}

/// What a ruler reads and how it names itself. A strip drawn against time
/// and a strip drawn against a signal stand on the same ruler: one band, one
/// row of ticks, one caption in the gutter.
pub(crate) struct Ruler<'a> {
    /// Where the ruler keeps the state of its gestures.
    pub id: egui::Id,
    /// The ticks of the range the plots above it show.
    pub axis: Axis,
    pub scale: XScale,
    /// The axis' name, as its accessible label reads it.
    pub name: &'a str,
    /// The words in the gutter beside the tick values: the quantity, and its
    /// unit.
    pub caption: String,
    /// A cursor's X as its box on the ruler reads it.
    pub cursor_text: &'a dyn Fn(f64) -> String,
    /// The record its overview track mirrors. A ruler with none has no
    /// record to scroll: it draws no track and takes no gesture that would
    /// move along one.
    pub overview: Option<Overview<'a>>,
    /// What the ruler's tooltip says in place of its overview's gestures.
    pub hover: Option<String>,
}

/// Draw the shared axis and overview, resolving its pointer and keyboard gestures.
pub fn show_shared_x_axis(
    ui: &mut Ui,
    derived: &mut DerivedSeries,
    model: &StripModel,
    input: SharedXAxis,
) -> SharedXAxisResponse {
    let (significant_digits, quantity_policy) = (input.significant_digits, input.quantity_policy);
    let cursor_text = |value: f64| model.format_x(value, significant_digits, quantity_policy);
    show_ruler(
        ui,
        Ruler {
            id: egui::Id::new(("rspice.results.shared-x", model.analysis_key)),
            axis: model_x_axis(model, input.current.0, input.current.1, quantity_policy),
            scale: model.x_scale,
            name: model.x_label(),
            caption: plot::x_axis_caption(model.x_label(), &model.x_unit),
            cursor_text: &cursor_text,
            overview: Some(Overview { derived, model }),
            hover: None,
        },
        input,
    )
}

/// Draw a ruler: the ticks of the range shown, the cursors' boxes, the
/// caption in the gutter and, when it has a record to scroll, the overview
/// track under them; the strip header carries its scale toggle. Resolves its
/// pointer and keyboard
/// gestures.
pub(crate) fn show_ruler(ui: &mut Ui, ruler: Ruler<'_>, input: SharedXAxis) -> SharedXAxisResponse {
    let mut output = SharedXAxisResponse::default();
    let Ruler {
        id,
        axis,
        scale,
        name,
        caption,
        cursor_text,
        overview,
        hover,
    } = ruler;
    let SharedXAxis {
        full_domain,
        current,
        height,
        right_margin,
        cursors: cursor_values,
        ..
    } = input;
    if height <= 1.0 {
        return output;
    }
    let has_overview = overview.is_some();
    let t = Tokens::get(ui.ctx());
    let c = t.color;
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
    let accessibility_label = if has_overview {
        format!(
            "Shared {} axis. Current range {} to {}. Full range {} to {}. {}. Drag the viewport window to pan or either of its edge handles to resize it, click the overview to recenter, use the wheel to zoom at the pointer, drag A or B to move a cursor, and press F to fit.",
            name,
            axis.format_display_value(current.0),
            axis.format_display_value(current.1),
            axis.format_display_value(full_domain.0),
            axis.format_display_value(full_domain.1),
            cursor_summary,
        )
    } else {
        format!(
            "{} axis. Current range {} to {}. {}. Press plus or minus to zoom, the arrow keys to pan, and F to fit.",
            name,
            axis.format_display_value(current.0),
            axis.format_display_value(current.1),
            cursor_summary,
        )
    };
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
    plot::paint_x_axis_band(&painter, rect, &t);

    let plot_left = (rect.left() + input.left_margin).min(rect.right());
    let plot_right = (rect.right() - right_margin).max(plot_left);
    let flag_top = rect.top() + SHARED_X_FLAG_TOP;
    let flag_bottom = flag_top + SHARED_X_FLAG_HEIGHT;
    // The lane spans exactly the panes' plot area, so the overview sits
    // under the traces it mirrors and its viewport window maps 1:1 to what
    // the panes show: its thumb says what the ticks above it cover. The
    // gutter beside it is the panes' own left gutter, left empty as theirs is
    // under the Y labels.
    //
    // A ruler with no record to scroll has an empty track, which every
    // overview gesture and the track's drawing pass over.
    let track = if has_overview {
        egui::Rect::from_min_max(
            egui::pos2(plot_left, rect.top() + SHARED_X_LANE_TOP),
            egui::pos2(
                plot_right,
                (rect.top() + SHARED_X_LANE_TOP + SHARED_X_LANE_HEIGHT).min(rect.bottom()),
            ),
        )
    } else {
        egui::Rect::from_min_max(
            egui::pos2(plot_left, rect.bottom()),
            egui::pos2(plot_left, rect.bottom()),
        )
    };
    // The lane band owns the overview gestures and everything above it
    // belongs to the flags, so a flag grab never fights a resize handle.
    let lane_band = egui::Rangef::new(track.top() - 4.0, track.bottom() + 4.0);
    // Whether the panes show less than the whole record: the thumb takes its
    // wash and the cursors get their notches only then.
    let (shown_start, shown_end) = shared_axis_viewport_fraction(scale, full_domain, current);
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
        let window_left = track.left() + track.width() * shown_start as f32;
        viewport = egui::Rect::from_min_max(
            egui::pos2(window_left, track.top()),
            egui::pos2(
                (track.left() + track.width() * shown_end as f32).max(window_left + 6.0),
                track.bottom(),
            ),
        );
        // The thumb is a window with two ends to pull at full view too, as
        // a range slider at full range is. It stands inside the plot pane,
        // so it is drawn in the neutral inks and never in the accent: a
        // frame in the strong border, the pressed wash while zoomed in, and
        // the hover fill while the pointer is over the track at full view,
        // where a wash across the whole track would say nothing and no
        // thumb at all hid that its ends pull in to zoom. Both washes go
        // down before the miniature, which they would otherwise cover.
        let lane_hovered = response
            .hover_pos()
            .is_some_and(|pointer| lane_band.contains(pointer.y));
        if zoomed {
            painter.rect_filled(viewport, 0.0, c.bg_active);
        } else if lane_hovered {
            painter.rect_filled(viewport, 0.0, c.bg_hover);
        }
        if let Some(Overview { derived, model }) = overview
            && let Some(trace) = model
                .traces
                .iter()
                .find(|trace| trace.visible && !trace.overlay)
        {
            // The lane's vertical fit is the trace's own finite extremes,
            // which the pane axis above it already holds under this very key.
            // Folding them again here was a second full pass over the
            // retained samples on every frame, for a curve fourteen pixels
            // tall.
            // The miniature, fit and segments both, is read once per data
            // version and scale: the lane is drawn on every frame.
            let key = super::stable_hash(&(trace_key(model, trace), scale == XScale::Log10));
            // A stored trace's miniature is drawn once the reader service
            // has read its tile.
            let miniature = match derived.overview(key) {
                Some(held) => Some(held),
                None => overview_miniature(derived, model, trace, scale, &mut output)
                    .map(|miniature| derived.keep_overview(key, miniature)),
            };
            let miniature = miniature.unwrap_or_else(|| {
                std::sync::Arc::new(crate::derived::OverviewMiniature {
                    range: None,
                    segments: Vec::new(),
                })
            });
            let (minimum, maximum) = miniature.range.unwrap_or((0.0, 1.0));
            let span = (maximum - minimum).max(f64::EPSILON);
            for segment in &miniature.segments {
                let points = segment
                    .iter()
                    .copied()
                    // t = 0 has no place on a logarithmic axis: the miniature
                    // starts at the first positive sample, as the panes do.
                    .filter(|(x, _)| scale == XScale::Linear || *x > 0.0)
                    .filter_map(|(x, y)| {
                        let fraction = scale.normalize(x, full_domain.0, full_domain.1);
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

        // The handles take the full ink while the thumb is in use, zoomed
        // in or under the pointer, and the secondary ink at rest.
        let handle_ink = if zoomed || lane_hovered {
            c.text
        } else {
            c.text_dim
        };
        painter.rect_stroke(
            viewport,
            0.0,
            egui::Stroke::new(1.0, c.border_strong),
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
        let fraction = scale.normalize(value, current.0, current.1);
        (fraction.is_finite() && (-0.001..=1.001).contains(&fraction))
            .then(|| plot_left + (plot_right - plot_left) * fraction.clamp(0.0, 1.0) as f32)
    };
    // Each cursor's foot on the ruler is its letter and X value in its own
    // colour, as EZwave's cursor bar and Verdi's ruler read the cursor time.
    // The box is the one place a stack names its cursors: the panes above
    // draw plain lines. A box shares the tick row, so it slides off the
    // values there, and a value it cannot clear gives way.
    let values = plot::x_axis_value_spans(
        &painter,
        &axis,
        scale,
        egui::Rangef::new(plot_left, plot_right),
        &t,
    );
    let flags = cursor_values.map_or_else(Vec::new, |cursors| {
        let placed: Vec<(SharedXDrag, &str, f32, String, egui::Color32)> = [
            (SharedXDrag::CursorA, "A", cursors.a, c.canvas_cursor_a),
            (SharedXDrag::CursorB, "B", cursors.b, c.canvas_cursor_b),
        ]
        .into_iter()
        .filter_map(|(drag, letter, value, color)| {
            let px = flag_centre(value?)?;
            let text = cursor_text(value?);
            Some((drag, letter, px, text, color))
        })
        .collect();
        let flag_font = theme::mono(tokens::FS_0, FontWeight::SemiBold);
        // The letter always rides along: two boxes pushed apart by cursors
        // close together are told apart by it, each still taking its line.
        let galleys: Vec<_> = placed
            .iter()
            .map(|(_, letter, _, text, _)| {
                painter.layout_no_wrap(format!("{letter} {text}"), flag_font.clone(), c.canvas_bg)
            })
            .collect();
        let spans: Vec<(f32, f32)> = placed
            .iter()
            .zip(&galleys)
            .map(|((_, _, px, _, _), galley)| (*px, galley.size().x + 2.0 * RULER_FLAG_PAD))
            .collect();
        let lefts = ruler_flag_lefts(&spans, egui::Rangef::new(plot_left, plot_right), &values);
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
    // against the panes they describe, in the row every plot reads its
    // abscissa from; a value in a cursor box's column gives way to the box,
    // as the two would read as one number.
    let keep_clear: Vec<egui::Rangef> = flags
        .iter()
        .map(|flag| flag.rect.x_range().expand(RULER_FLAG_PAD))
        .collect();
    output.caption_rect = Some(plot::paint_x_axis_row(
        &painter,
        rect,
        plot::XAxisRow {
            axis: &axis,
            scale,
            plot: egui::Rangef::new(plot_left, plot_right),
            caption: &caption,
            keep_clear: &keep_clear,
        },
        &t,
    ));

    // The same fills and ink as the plot's own cursor flags, so one cursor
    // is one colour everywhere and its value reads off the fill whichever
    // interface theme frames the canvas. The box hangs from the cursor's own
    // line: the pane directly above draws it down to the ruler, and the
    // ruler carries it on, at the cursor's exact X, into the box's top edge,
    // so the value reads as that line's. A box slid along the row, or pushed
    // aside by its neighbour, still takes the line where the cursor stands.
    // The lead starts a pixel above the ruler, over the rule on its top edge
    // wherever that rule's pixel row lands, so the line never breaks there.
    let lead = ui
        .painter()
        .with_clip_rect(rect.with_min_y(rect.top() - 1.0).intersect(ui.clip_rect()));
    for flag in &flags {
        lead.vline(
            flag.px,
            egui::Rangef::new(rect.top() - 1.0, flag.rect.top()),
            egui::Stroke::new(1.0, flag.color),
        );
        painter.rect_filled(flag.rect, 2.0, flag.color);
        painter.galley(
            flag.rect.center() - flag.galley.size() * 0.5,
            flag.galley.clone(),
            c.canvas_bg,
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
            let fraction = scale.normalize(value, full_domain.0, full_domain.1);
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
    let drag_id = id.with("drag");
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
                    scale,
                    full_domain,
                    current,
                    f64::from(delta / track.width()),
                ) {
                    output.viewport = Some(SharedXViewChange::Range(range));
                }
            }
            SharedXDrag::ResizeStart | SharedXDrag::ResizeEnd => {
                if let Some(range) = resized_shared_x_view(
                    scale,
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
                let x = scale.denormalize(view_fraction, current.0, current.1);
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
            } else if has_overview && lane_band.contains(pointer.y) {
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
            scale,
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
                scale,
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
        // Left and Right pan the ruler; egui must not also move the focus on
        // them.
        ui.memory_mut(|memory| {
            memory.set_focus_lock_filter(
                response.id,
                egui::EventFilter {
                    horizontal_arrows: true,
                    ..Default::default()
                },
            );
        });
        let fit_key =
            ui.input(|input| input.key_pressed(egui::Key::F) || input.key_pressed(egui::Key::Home));
        if fit_key {
            output.viewport = Some(SharedXViewChange::Fit);
        }
        if ui.input(|input| input.key_pressed(egui::Key::End)) {
            output.viewport = Some(SharedXViewChange::Follow);
        }
        let zoom_in = ui.input(|input| input.key_pressed(egui::Key::Plus));
        let zoom_out = ui.input(|input| input.key_pressed(egui::Key::Minus));
        if (zoom_in || zoom_out)
            && let Some(range) = zoomed_shared_x_view(
                scale,
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
            && let Some(range) = panned_shared_x_view(scale, full_domain, current, direction * 0.05)
        {
            output.viewport = Some(SharedXViewChange::Range(range));
        }
    }

    if response.double_clicked() {
        output.viewport = Some(SharedXViewChange::Fit);
    }
    // A click takes the keyboard so the keys above work, and the ring that
    // shows where the keyboard is stays for focus Tab moved here.
    if theme::focus_is_visible(ui.ctx(), &response) {
        theme::paint_focus_ring(ui, &response, rect);
    }
    response.on_hover_text(hover.unwrap_or_else(|| {
        "Drag an end to zoom in \u{00b7} drag the window to pan \u{00b7} click to recenter \u{00b7} \
         wheel to zoom \u{00b7} double-click or F for the full view \u{00b7} End follows a \
         running transient's newest results \u{00b7} drag a cursor's box to move it"
            .to_owned()
    }));
    output
}

/// A stored trace's range, and its miniature as unbroken runs of points.
type StoredOverview = (Option<(f64, f64)>, Vec<Vec<(f64, f64)>>);

/// Read the overview miniature of `trace` on a `scale` axis; `None` while the
/// tile of a stored trace is being read.
fn overview_miniature(
    derived: &mut DerivedSeries,
    model: &StripModel,
    trace: &StripTrace,
    scale: XScale,
    output: &mut SharedXAxisResponse,
) -> Option<crate::derived::OverviewMiniature> {
    // A stored trace's miniature is its tile across the whole axis, by
    // decade on a logarithmic one, read from the store's summaries rather
    // than its column, off the frame.
    let stored = match stored_overview(trace, scale) {
        Fetched::Ready(overview) => Some(overview),
        Fetched::Pending => return None,
        Fetched::Refused(_) => None,
    };
    // The lane's vertical fit is the trace's own finite extremes, which the
    // pane axis above it already holds under this very key.
    let range = derived.range_or(trace_key(model, trace), || match &stored {
        Some((range, _)) => *range,
        None => {
            output.extrema_samples_read = trace.y.len();
            trace
                .y
                .in_memory()
                .and_then(rspice_results::measurements::finite_extremes)
        }
    });
    let segments = match (stored, trace.x.in_memory(), trace.y.in_memory()) {
        (Some((_, segments)), ..) => segments,
        (None, Some(x), Some(y)) => {
            let samples = x.len().min(y.len());
            let (x, y) = (&x[..samples], &y[..samples]);
            let indices = derived.overview_indices_or(trace_key(model, trace), || {
                output.overview_samples_read = samples.saturating_mul(2);
                rspice_results::waveform::display_sample_indices(y, SHARED_X_OVERVIEW_POINTS)
            });
            output.overview_samples_read += indices.len();
            overview_segments(x, y, &indices)
        }
        // A stored trace that cannot tile, on an axis that does not ascend or
        // a decade axis that reaches zero, has no miniature: drawing one would
        // read its whole column.
        (None, ..) => Vec::new(),
    };
    Some(crate::derived::OverviewMiniature { range, segments })
}

/// The range and miniature of a trace whose samples a results store keeps: its
/// tile across its whole axis on a `scale` axis, each pixel's least and
/// greatest value at the pixel's middle, broken where a pixel holds no
/// available value. `Refused` for a trace held in memory or one that cannot
/// tile.
fn stored_overview(trace: &StripTrace, scale: XScale) -> Fetched<StoredOverview> {
    use rspice_results::waveform::ColumnTile;
    if trace.y.in_memory().is_some() {
        return Fetched::Refused("held in memory".to_owned());
    }
    let tile =
        match trace
            .y
            .fetch_axis_tile(&trace.x, scale == XScale::Log10, SHARED_X_OVERVIEW_POINTS)
        {
            Fetched::Ready(tile) => tile,
            Fetched::Pending => return Fetched::Pending,
            Fetched::Refused(reason) => return Fetched::Refused(reason),
        };
    let mut points: Vec<(f64, f64)> = Vec::new();
    match tile.tile.as_ref() {
        ColumnTile::Samples { x, y, .. } => {
            points.extend(x.iter().copied().zip(y.iter().copied()));
        }
        ColumnTile::Envelope { min, max, touched } => {
            for pixel in (0..min.len()).filter(|&pixel| touched[pixel]) {
                let x = tile.middle(pixel, min.len());
                points.push((x, min[pixel]));
                if max[pixel] != min[pixel] {
                    points.push((x, max[pixel]));
                }
            }
        }
    }
    let range = points
        .iter()
        .map(|&(_, y)| y)
        .filter(|y| y.is_finite())
        .fold(None, |range: Option<(f64, f64)>, y| {
            Some(range.map_or((y, y), |(lo, hi)| (lo.min(y), hi.max(y))))
        });
    let segments = points
        .split(|(x, y)| !(x.is_finite() && y.is_finite()))
        .filter(|segment| !segment.is_empty())
        .map(<[(f64, f64)]>::to_vec)
        .collect();
    Fetched::Ready((range, segments))
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
    let next_width = (width * factor).clamp(narrowest_window(scale, full, anchor), 1.0);
    let next_start = (anchor - relative * next_width).clamp(0.0, 1.0 - next_width);
    let next_end = next_start + next_width;
    Some((
        scale.denormalize(next_start, full.0, full.1),
        scale.denormalize(next_end, full.0, full.1),
    ))
}

/// The narrowest window at `at` (a fraction of the full sweep), as a fraction
/// of the sweep: the plots' own floor, so a ruler zooms no further than the
/// panes it stands under and no less far.
fn narrowest_window(scale: XScale, full: (f64, f64), at: f64) -> f64 {
    let value = scale.denormalize(at, full.0, full.1);
    floored_span((value, value), scale)
        .map(|(low, high)| {
            (scale.normalize(high, full.0, full.1) - scale.normalize(low, full.0, full.1)).abs()
        })
        .filter(|width| width.is_finite() && *width > 0.0)
        .unwrap_or(f64::EPSILON)
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
    // The edge that holds is where the window cannot narrow past.
    let least = narrowest_window(scale, full, if move_start { end } else { start });
    let (next_start, next_end) = if move_start {
        (edge.min(end - least), end)
    } else {
        (start, edge.max(start + least))
    };
    let next_start = next_start.clamp(0.0, 1.0 - least);
    let next_end = next_end.clamp(next_start + least, 1.0);
    Some((
        scale.denormalize(next_start, full.0, full.1),
        scale.denormalize(next_end, full.0, full.1),
    ))
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
