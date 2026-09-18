//! The engine's own evaluation of the draft, drawn.
//!
//! This is the band the instrument is for. Every other band describes the
//! definition; this one shows what a run will do with it, sampled by the same
//! evaluator the transient steps with, over a window whose length is stated
//! rather than assumed. When a family has no waveform at this boundary — a
//! noise train, a random draw, a table in a file the host cannot read — the
//! plot states that instead of drawing the flat line the spec would return,
//! because a flat line looks like an answer.
//!
//! The strip above the plot carries the three spans, the derived readouts the
//! engine resolved, the transient those substitutions were made against, and
//! the hover readout. Nothing in it is computed here.

use egui::{Align2, Color32, Pos2, Rect, Sense, Stroke, Ui, Vec2};

use crate::simulation::stimulus_realize::Guide;
use crate::state::format_engineering;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::state::PreviewSpan;
use crate::workbench::{AppState, MessageId};

use super::{Stage, StageAction};

/// Height of the strip above the plot.
const STRIP_HEIGHT: f32 = 24.0;
/// Room for the value axis on the left and the time axis underneath.
/// The value gutter the band keeps even when its labels are shorter, so the
/// plot does not jump sideways as a waveform's range changes decade. The real
/// gutter is measured from the labels; this is its floor.
const MINIMUM_VALUE_GUTTER: f32 = 34.0;
/// Space between the longest value label and the plot's left edge.
const TICK_LABEL_GAP: f32 = 5.0;
/// What a measured gutter adds to its widest label: the gap to the plot, and
/// the band's own left inset.
const TICK_LABEL_INSET: f32 = TICK_LABEL_GAP + 6.0;
const TIME_GUTTER: f32 = 16.0;
/// How close a pointer has to be to a PWL marker to select it.
const MARKER_GRAB: f32 = 7.0;

pub(super) fn show(ui: &mut Ui, state: &AppState, stage: &Stage, actions: &mut Vec<StageAction>) {
    let band = ui.available_rect_before_wrap();
    let strip = Rect::from_min_max(
        band.min,
        Pos2::new(band.right(), band.top() + STRIP_HEIGHT.min(band.height())),
    );
    let plot = Rect::from_min_max(Pos2::new(band.left(), strip.bottom()), band.max);
    let hover = plot_body(ui, state, stage, plot, actions);
    strip_row(ui, state, stage, strip, hover, actions);
}

/// The control strip: spans, readouts, evaluator chip, hover value.
fn strip_row(
    ui: &mut Ui,
    state: &AppState,
    stage: &Stage,
    rect: Rect,
    hover: Option<(f64, f64)>,
    actions: &mut Vec<StageAction>,
) {
    let messages = state.ui.messages();
    let tokens = Tokens::get(ui.ctx());
    // The strip reads from both ends: spans and derived readouts from the
    // left, the transient the substitutions were made under and the hover
    // value from the right. The right-hand pair is painted into measured
    // positions rather than laid out, because a row has one cursor and a
    // right-aligned group appended to it takes whatever the left half left
    // over — which at this band's width is nothing.
    let trailing_width = (rect.width() * 0.42).clamp(0.0, 320.0);
    let leading = Rect::from_min_max(
        rect.min,
        Pos2::new((rect.right() - trailing_width).max(rect.left()), rect.bottom()),
    );
    let trailing = Rect::from_min_max(Pos2::new(leading.right(), rect.top()), rect.max);

    let mut strip = ui.new_child(egui::UiBuilder::new().max_rect(leading));
    strip.set_clip_rect(ui.clip_rect().intersect(leading));
    strip.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
        ui.add_space(8.0);
        for span in PreviewSpan::ALL {
            let available = span != PreviewSpan::Period || stage.realization.fundamental.is_some();
            let response = ui
                .add_enabled_ui(available, |ui| {
                    ui.selectable_label(stage.span == span, span.label())
                })
                .inner;
            if response.clicked() && stage.span != span {
                actions.push(StageAction::SetSpan(span));
            }
            if !available {
                response.on_disabled_hover_text(messages.text(MessageId::StimulusNoFundamental));
            }
        }
        ui.add_space(10.0);
        for (label, value) in &stage.realization.derived {
            // A readout that would not fit is dropped rather than clipped:
            // half a number beside a full one reads as a different number.
            if ui.next_widget_position().x > leading.right() - 56.0 {
                break;
            }
            ui.add(egui::Label::new(
                egui::RichText::new(label)
                    .font(theme::sans(tokens::FS_MICRO, FontWeight::Regular))
                    .color(tokens.color.text_faint),
            ));
            ui.add_space(3.0);
            ui.add(
                egui::Label::new(
                    egui::RichText::new(value)
                        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                        .color(tokens.color.text),
                )
                .truncate(),
            );
            ui.add_space(8.0);
        }
    });

    let readout = match hover {
        Some((time, value)) => format!(
            "t = {}s \u{b7} {} = {}{}",
            format_engineering(time),
            stage.working.kind().letter().to_lowercase(),
            format_engineering(value),
            unit(stage)
        ),
        None => messages.text(MessageId::StimulusHoverUnset),
    };
    let chip = messages.text(MessageId::StimulusEngineEvaluator);
    let right = paint_from_right(
        ui,
        trailing,
        rect.right() - 8.0,
        &readout,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        tokens.color.text_dim,
    );
    let chip_rect = paint_chip_from_right(
        ui,
        trailing,
        right - 8.0,
        &chip,
        theme::mono(tokens::FS_MICRO, FontWeight::Regular),
        tokens.color.text_faint,
    );
    // The chip is the one thing here a reader can ask a question of: which
    // transient every substitution above was resolved against.
    let response = ui.interact(
        chip_rect,
        ui.id().with("workbench.stimulus.proof.evaluator"),
        Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &chip)
    });
    response.on_hover_text(stage.timing.caption());
}

/// Paint one run of text so that it ends at `right`, elided into the room it
/// has, and report where it starts.
fn paint_from_right(
    ui: &Ui,
    region: Rect,
    right: f32,
    text: &str,
    font: egui::FontId,
    color: Color32,
) -> f32 {
    let room = (right - region.left()).max(0.0);
    let text = crate::workbench::design_system::elide_text(ui, text, &font, room);
    let painter = ui.painter().with_clip_rect(region.intersect(ui.clip_rect()));
    let galley = painter.layout_no_wrap(text, font, color);
    let left = right - galley.size().x;
    painter.galley(
        Pos2::new(left, region.center().y - galley.size().y * 0.5),
        galley,
        color,
    );
    left
}

/// The same, reporting the rectangle the run occupies so it can be hovered.
fn paint_chip_from_right(
    ui: &Ui,
    region: Rect,
    right: f32,
    text: &str,
    font: egui::FontId,
    color: Color32,
) -> Rect {
    let left = paint_from_right(ui, region, right, text, font, color);
    Rect::from_min_max(
        Pos2::new(left, region.top()),
        Pos2::new(right.max(left), region.bottom()),
    )
}

/// The plot itself, and the hovered sample it reports back.
fn plot_body(
    ui: &mut Ui,
    state: &AppState,
    stage: &Stage,
    rect: Rect,
    actions: &mut Vec<StageAction>,
) -> Option<(f64, f64)> {
    let tokens = Tokens::get(ui.ctx());
    let colors = tokens.color;
    let response = ui.interact(
        rect,
        ui.id().with("workbench.stimulus.proof"),
        Sense::click(),
    );
    ui.painter().rect_filled(rect, 0.0, colors.bg_app);

    let label = accessible_label(state, stage);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Image, ui.is_enabled(), &label)
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Image);
        node.set_label(label.as_str());
    });

    let Some(readouts) = stage.realization.readouts.filter(|_| stage.realization.defect.is_none())
    else {
        let plot = plot_rect(rect, MINIMUM_VALUE_GUTTER);
        if plot.width() > 1.0 && plot.height() > 1.0 {
            statement(ui, plot, state, stage);
        }
        return None;
    };

    let (minimum, maximum) = padded_range(readouts.minimum, readouts.maximum);
    let value_ticks = value_ticks(ui, minimum, maximum);
    // The gutter is measured from the labels it has to hold. A fixed one is a
    // guess, and the frame that disproves the guess paints a tick label at a
    // negative x — outside the band, outside the stage, and unreadable.
    let gutter = value_ticks
        .iter()
        .map(|(_, _, width)| *width)
        .fold(0.0_f32, f32::max)
        + TICK_LABEL_INSET;
    let plot = plot_rect(
        rect,
        gutter.clamp(MINIMUM_VALUE_GUTTER, (rect.width() * 0.4).max(MINIMUM_VALUE_GUTTER)),
    );
    if plot.width() <= 1.0 || plot.height() <= 1.0 {
        return None;
    }
    let projector = Projector {
        plot,
        span: stage.realization.span.max(f64::MIN_POSITIVE),
        minimum,
        maximum,
    };

    paint_grid(ui, &projector, colors.border, &value_ticks);
    if minimum < 0.0 && maximum > 0.0 {
        ui.painter().hline(
            plot.x_range(),
            projector.y(0.0),
            Stroke::new(1.0, colors.text_faint),
        );
    }
    paint_guides(ui, &projector, &stage.realization.guides, colors.warn);

    let points = stage
        .realization
        .samples
        .iter()
        .map(|(time, value)| Pos2::new(projector.x(*time), projector.y(*value)))
        .collect::<Vec<_>>();
    ui.painter()
        .add(egui::Shape::line(points, Stroke::new(1.4, colors.accent)));

    if paint_markers(ui, stage, &projector, &response, actions) {
        return None;
    }
    response.hover_pos().and_then(|pointer| {
        let fraction = ((pointer.x - plot.left()) / plot.width()).clamp(0.0, 1.0) as f64;
        nearest_sample(&stage.realization.samples, fraction * projector.span)
    })
}

/// Where a `(time, value)` lands inside the plot.
///
/// One projection shared by the grid, the curve, the guides and the markers:
/// two of them computed separately would put a breakpoint rule beside the edge
/// it marks rather than on it.
struct Projector {
    plot: Rect,
    span: f64,
    minimum: f64,
    maximum: f64,
}

impl Projector {
    fn x(&self, time: f64) -> f32 {
        egui::lerp(
            self.plot.left()..=self.plot.right(),
            (time / self.span).clamp(0.0, 1.0) as f32,
        )
    }

    fn y(&self, value: f64) -> f32 {
        egui::lerp(
            self.plot.bottom()..=self.plot.top(),
            ((value - self.minimum) / (self.maximum - self.minimum)).clamp(0.0, 1.0) as f32,
        )
    }
}

/// What a reader who cannot see the plot is told about it.
fn accessible_label(state: &AppState, stage: &Stage) -> String {
    let messages = state.ui.messages();
    match stage.realization.defect.as_deref() {
        Some(defect) => defect.to_owned(),
        None => messages.format(
            MessageId::StimulusWaveformOf,
            &[
                ("name", stage.working.name()),
                ("span", &format!("{}s", format_engineering(stage.realization.span))),
            ],
        ),
    }
}

/// Why there is no curve, stated where the curve would have been.
fn statement(ui: &mut Ui, plot: Rect, state: &AppState, stage: &Stage) {
    let messages = state.ui.messages();
    let tokens = Tokens::get(ui.ctx());
    let text = stage
        .realization
        .defect
        .clone()
        .unwrap_or_else(|| messages.text(MessageId::StimulusNoCurve));
    let galley = ui.painter().layout(
        text,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        tokens.color.text_dim,
        (plot.width() - 32.0).max(80.0),
    );
    ui.painter().galley(
        Pos2::new(
            plot.center().x - galley.size().x * 0.5,
            plot.center().y - galley.size().y * 0.5,
        ),
        galley,
        tokens.color.text_dim,
    );
}

/// The three value ticks an axis states, with the width each label needs.
///
/// The labels come from the plot's own tick formatter — the one the Results
/// axes read through — so a value reads the same on this band as it does on
/// the waveform viewer that will show the run. Its zero is exact, which is why
/// the tick values are snapped first: the midpoint of a symmetric range is
/// zero only up to the rounding of two float steps, and `-6.9e-16` is not a
/// number anyone can read off an axis.
fn value_ticks(ui: &Ui, minimum: f64, maximum: f64) -> [(f64, String, f32); 3] {
    let font = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
    let range = maximum - minimum;
    let step = range / 2.0;
    [0.0_f64, 0.5, 1.0].map(|fraction| {
        let value = snapped_to_zero(minimum + range * fraction, step);
        let text = crate::ui::plot::tick_label_with_step(value, step);
        let width = ui
            .painter()
            .layout_no_wrap(text.clone(), font.clone(), Color32::PLACEHOLDER)
            .size()
            .x;
        (value, text, width)
    })
}

/// A tick value that is zero to within the axis's own resolution, as exactly
/// zero.
fn snapped_to_zero(value: f64, step: f64) -> f64 {
    if value.abs() <= step.abs() * ZERO_TICK_TOLERANCE {
        0.0
    } else {
        value
    }
}

/// How close to zero a tick has to be before it is one. A billionth of the
/// tick spacing is far below anything an axis could resolve and far above the
/// rounding two float steps accumulate.
const ZERO_TICK_TOLERANCE: f64 = 1e-9;

/// The plotting area inside the band, given the gutter its value labels need.
fn plot_rect(band: Rect, value_gutter: f32) -> Rect {
    Rect::from_min_max(
        Pos2::new(band.left() + value_gutter, band.top() + 6.0),
        Pos2::new(band.right() - 12.0, band.bottom() - TIME_GUTTER),
    )
}

/// A range that never collapses, so a flat curve still has a plot to sit in.
fn padded_range(minimum: f64, maximum: f64) -> (f64, f64) {
    let span = maximum - minimum;
    let pad = if span > 0.0 {
        span * 0.12
    } else {
        maximum.abs().max(1.0) * 0.12
    };
    (minimum - pad, maximum + pad)
}

fn paint_grid(ui: &Ui, projector: &Projector, color: Color32, value_ticks: &[(f64, String, f32)]) {
    let plot = projector.plot;
    let tokens = Tokens::get(ui.ctx());
    let font = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
    let stroke = Stroke::new(0.5, color);
    let time_step = projector.span / 4.0;
    for fraction in [0.0_f64, 0.25, 0.5, 0.75, 1.0] {
        let time = snapped_to_zero(projector.span * fraction, time_step);
        let x = projector.x(time);
        ui.painter().vline(x, plot.y_range(), stroke);
        if fraction == 0.0 || fraction == 0.5 || fraction == 1.0 {
            ui.painter().text(
                Pos2::new(x, plot.bottom() + 2.0),
                if fraction == 1.0 {
                    Align2::RIGHT_TOP
                } else if fraction == 0.0 {
                    Align2::LEFT_TOP
                } else {
                    Align2::CENTER_TOP
                },
                format!("{}s", crate::ui::plot::tick_label_with_step(time, time_step)),
                font.clone(),
                tokens.color.text_faint,
            );
        }
    }
    for (value, text, _) in value_ticks {
        let y = projector.y(*value);
        ui.painter().hline(plot.x_range(), y, stroke);
        ui.painter().text(
            Pos2::new(plot.left() - TICK_LABEL_GAP, y),
            Align2::RIGHT_CENTER,
            text,
            font.clone(),
            tokens.color.text_faint,
        );
    }
}

/// The breakpoint rules and their labels.
fn paint_guides(ui: &Ui, projector: &Projector, guides: &[Guide], color: Color32) {
    let plot = projector.plot;
    let tokens = Tokens::get(ui.ctx());
    let font = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
    for guide in guides {
        let x = projector.x(guide.time);
        ui.painter().vline(
            x,
            plot.y_range(),
            Stroke::new(1.0, color.gamma_multiply(0.45)),
        );
        // A rule near the right edge carries its tag on its left, so the tag
        // stays on the plot it names rather than running into the gutter.
        let galley = ui.painter().layout_no_wrap(
            guide.label.to_owned(),
            font.clone(),
            tokens.color.text_faint,
        );
        let left = if x + 2.0 + galley.size().x <= plot.right() {
            x + 2.0
        } else {
            (x - 2.0 - galley.size().x).max(plot.left())
        };
        ui.painter()
            .galley(Pos2::new(left, plot.top()), galley, tokens.color.text_faint);
    }
}

/// The authored PWL points, and the click that selects one.
///
/// Reports whether the pointer is over a marker, so the hover readout does not
/// also fire on a press that was aimed at a point.
fn paint_markers(
    ui: &Ui,
    stage: &Stage,
    projector: &Projector,
    response: &egui::Response,
    actions: &mut Vec<StageAction>,
) -> bool {
    let colors = Tokens::get(ui.ctx()).color;
    let mut over_marker = false;
    for marker in &stage.realization.markers {
        let center = Pos2::new(projector.x(marker.time), projector.y(marker.value));
        let selected = stage.selected_point == Some(marker.index);
        ui.painter().circle(
            center,
            if selected { 4.5 } else { 3.0 },
            if selected { colors.accent } else { colors.bg_app },
            Stroke::new(1.2, colors.accent),
        );
        let hit = Rect::from_center_size(center, Vec2::splat(MARKER_GRAB * 2.0));
        if response.hover_pos().is_some_and(|pointer| hit.contains(pointer)) {
            over_marker = true;
            if response.clicked() {
                actions.push(StageAction::SelectPoint(Some(marker.index)));
            }
        }
    }
    over_marker
}

/// The sample nearest a time, which is what the hover readout states.
///
/// Nearest rather than interpolated: the curve is the engine's own samples,
/// and a value between two of them is a number the run never produced.
fn nearest_sample(samples: &[(f64, f64)], time: f64) -> Option<(f64, f64)> {
    samples
        .iter()
        .copied()
        .min_by(|left, right| {
            (left.0 - time)
                .abs()
                .total_cmp(&(right.0 - time).abs())
        })
}

/// The unit the hover readout states its value in.
fn unit(stage: &Stage) -> &'static str {
    match stage.working.kind() {
        crate::state::stimulus_library::definition::StimulusKind::Voltage => "V",
        crate::state::stimulus_library::definition::StimulusKind::Current => "A",
    }
}
