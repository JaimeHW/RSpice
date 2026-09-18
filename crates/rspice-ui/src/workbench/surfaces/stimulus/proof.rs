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
//! A window holding more cycles than it has columns is drawn as a band between
//! each column's measured extremes rather than as a polyline through samples
//! that would land wherever the grid's own phase put them. Which of the two it
//! is comes from the realization, is stated on the strip and in the plot's own
//! announcement, and changes what a hover can honestly report: a band covers a
//! range, and naming one value out of it would be a number the run never
//! produced.
//!
//! The strip above the plot carries the three spans, what the trace is, the
//! derived readouts the engine resolved, the transient those substitutions were
//! made against, and the hover readout. Nothing in it is computed here.

use egui::{Align2, Color32, Pos2, Rect, Sense, Stroke, Ui, Vec2};

use crate::simulation::stimulus_realize::{EnvelopeColumn, Guide, TraceReading, WaveformTrace};
use crate::state::format_engineering_display;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{ViewOption, view_switch};
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
/// Air between a readout's label and its value.
const READOUT_GAP: f32 = 4.0;

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
    hover: Option<TraceReading>,
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
        Pos2::new(
            (rect.right() - trailing_width).max(rect.left()),
            rect.bottom(),
        ),
    );
    let trailing = Rect::from_min_max(Pos2::new(leading.right(), rect.top()), rect.max);

    let mut strip = ui.new_child(egui::UiBuilder::new().max_rect(leading));
    strip.set_clip_rect(ui.clip_rect().intersect(leading));
    strip.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
        ui.add_space(8.0);
        // Period stays in the switch for a family with no fundamental, faint,
        // saying why: the other two do not move when the family changes.
        let no_fundamental = messages.text(MessageId::StimulusNoFundamental);
        let options = PreviewSpan::ALL.map(|span| ViewOption {
            label: span.label(),
            unavailable: (span == PreviewSpan::Period && stage.realization.fundamental.is_none())
                .then_some(no_fundamental.as_str()),
        });
        let selected = PreviewSpan::ALL
            .iter()
            .position(|span| *span == stage.span)
            .unwrap_or(0);
        if let Some(index) = view_switch(ui, "workbench.stimulus.proof.span", &options, selected) {
            actions.push(StageAction::SetSpan(PreviewSpan::ALL[index]));
        }
        ui.add_space(10.0);
        // What the reader is looking at comes before what it measures: a band
        // read as a curve is a waveform with a period nobody authored.
        if let Some(caption) = stage.realization.trace.caption() {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(caption)
                        .font(theme::mono(tokens::FS_MICRO, FontWeight::Regular))
                        .color(tokens.color.warn),
                )
                .truncate(),
            );
            ui.add_space(10.0);
        }
        for (label, value) in &stage.realization.derived {
            // A readout is a label and its value, and it is painted whole or
            // not at all: half a number beside a full one reads as a different
            // number, and a label with nothing beside it reads as a fault.
            let label_galley = ui.painter().layout_no_wrap(
                label.clone(),
                theme::sans(tokens::FS_MICRO, FontWeight::Regular),
                tokens.color.text_faint,
            );
            let value_galley = ui.painter().layout_no_wrap(
                value.clone(),
                theme::mono(tokens::FS_0, FontWeight::Medium),
                tokens.color.text,
            );
            let size = Vec2::new(
                label_galley.size().x + READOUT_GAP + value_galley.size().x,
                label_galley.size().y.max(value_galley.size().y),
            );
            if ui.next_widget_position().x + size.x > leading.right() - 4.0 {
                break;
            }
            let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
            // Both runs sit on one baseline, which a small label beside a
            // larger value does not do when each is centred on its own height.
            ui.painter().galley(
                Pos2::new(rect.left(), rect.bottom() - label_galley.size().y - 0.5),
                label_galley,
                tokens.color.text_faint,
            );
            ui.painter().galley(
                Pos2::new(
                    rect.right() - value_galley.size().x,
                    rect.bottom() - value_galley.size().y,
                ),
                value_galley,
                tokens.color.text,
            );
            response.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::Label,
                    ui.is_enabled(),
                    format!("{label} {value}"),
                )
            });
            ui.add_space(10.0);
        }
    });

    // A band has no single value at a time, and picking one out of the range it
    // covers would be a number the run never produced.
    let readout = match hover {
        Some(TraceReading::Sample { time, value }) => format!(
            "t = {}s \u{b7} {} = {}{}",
            format_engineering_display(time),
            stage.working.kind().letter().to_lowercase(),
            format_engineering_display(value),
            unit(stage)
        ),
        Some(TraceReading::Band {
            time,
            minimum,
            maximum,
        }) => format!(
            "t = {}s \u{b7} {} = {} to {}{}",
            format_engineering_display(time),
            stage.working.kind().letter().to_lowercase(),
            format_engineering_display(minimum),
            format_engineering_display(maximum),
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
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &chip));
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
    let painter = ui
        .painter()
        .with_clip_rect(region.intersect(ui.clip_rect()));
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
) -> Option<TraceReading> {
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

    let Some(readouts) = stage
        .realization
        .readouts
        .filter(|_| stage.realization.defect.is_none())
    else {
        let plot = plot_rect(rect, MINIMUM_VALUE_GUTTER);
        if plot.width() > 1.0 && plot.height() > 1.0 {
            statement(ui, plot, state, stage);
        }
        return None;
    };

    let (minimum, maximum) = padded_range(readouts.minimum, readouts.maximum);
    // The plot's height does not depend on its gutter, so the ladder can be
    // chosen before the rectangle it is drawn in.
    let axis = value_ticks(
        ui,
        minimum,
        maximum,
        plot_rect(rect, MINIMUM_VALUE_GUTTER).height(),
        unit(stage),
    );
    // The gutter is measured from the labels it has to hold. A fixed one is a
    // guess, and the frame that disproves the guess paints a tick label at a
    // negative x — outside the band, outside the stage, and unreadable.
    let gutter = axis
        .ticks
        .iter()
        .map(|(_, _, width)| *width)
        .fold(0.0_f32, f32::max)
        + TICK_LABEL_INSET;
    let plot = plot_rect(
        rect,
        gutter.clamp(
            MINIMUM_VALUE_GUTTER,
            (rect.width() * 0.4).max(MINIMUM_VALUE_GUTTER),
        ),
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

    paint_grid(ui, &projector, colors.border, &axis);
    if minimum < 0.0 && maximum > 0.0 {
        ui.painter().hline(
            plot.x_range(),
            projector.y(0.0),
            Stroke::new(1.0, colors.text_faint),
        );
    }
    paint_guides(ui, &projector, &stage.realization.guides, colors.warn);

    match &stage.realization.trace {
        WaveformTrace::Curve(samples) => {
            let points = samples
                .iter()
                .map(|(time, value)| Pos2::new(projector.x(*time), projector.y(*value)))
                .collect::<Vec<_>>();
            ui.painter()
                .add(egui::Shape::line(points, Stroke::new(1.4, colors.accent)));
        }
        WaveformTrace::Envelope { columns, .. } => {
            paint_envelope(ui, &projector, columns, colors.accent);
        }
    }

    if paint_markers(ui, stage, &projector, &response, actions) {
        return None;
    }
    response.hover_pos().and_then(|pointer| {
        let fraction = ((pointer.x - plot.left()) / plot.width()).clamp(0.0, 1.0) as f64;
        stage
            .realization
            .trace
            .reading_at(fraction * projector.span)
    })
}

/// The band between each column's extremes, projected onto the plot.
fn paint_envelope(ui: &Ui, projector: &Projector, columns: &[EnvelopeColumn], color: Color32) {
    let projected = columns
        .iter()
        .map(|column| {
            let x = projector.x(column.time);
            (
                Pos2::new(x, projector.y(column.maximum)),
                Pos2::new(x, projector.y(column.minimum)),
            )
        })
        .collect::<Vec<_>>();
    crate::ui::plot::paint_min_max_band(ui.painter(), &projected, color, 1.2);
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
///
/// A band says so here as well as on the strip: the one thing a reader who
/// cannot see this surface most needs from it is whether the shape it states is
/// one curve or the extremes of five hundred of them.
fn accessible_label(state: &AppState, stage: &Stage) -> String {
    let messages = state.ui.messages();
    match stage.realization.defect.as_deref() {
        Some(defect) => defect.to_owned(),
        None => {
            let waveform = messages.format(
                MessageId::StimulusWaveformOf,
                &[
                    ("name", stage.working.name()),
                    (
                        "span",
                        &format!("{}s", format_engineering_display(stage.realization.span)),
                    ),
                ],
            );
            match stage.realization.trace.caption() {
                Some(caption) => format!("{waveform} \u{b7} {caption}"),
                None => waveform,
            }
        }
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

/// The value ticks an axis states, with the width each label needs.
///
/// The plot kit's own ladder — the one the Results axes are drawn from — over
/// the padded range, so every label is a round number and its grid line is
/// drawn at exactly that number. The first version of this band labelled the
/// padded minimum, midpoint and maximum and let the formatter round them: a
/// sine of 2 mV read `2m` at a line 2.48 mV up, above its own crest, and a
/// ramp to 5 V read `-1`, `2`, `6` for -0.6, 2.5 and 5.6. A proof surface may
/// be coarse; it may not state a number that is not where it says it is.
///
/// Zero is on the ladder whenever it is in range, and the ladder's zero is
/// exact, so a waveform centred on it states `0` rather than the `-6.9e-16`
/// two float steps leave behind.
fn value_ticks(ui: &Ui, minimum: f64, maximum: f64, height: f32, unit: &str) -> ValueAxis {
    let font = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
    let target = ((height / VALUE_TICK_PITCH).floor() as usize).clamp(2, 6);
    let series = crate::ui::plot::linear_ticks(minimum, maximum, target);
    let anchor = crate::ui::plot::anchor_label(&series, unit);
    let ticks = series
        .ticks
        .into_iter()
        .map(|(value, label)| {
            // An offset axis states its unit once, on the anchor; an absolute
            // one states it on every tick but zero, which has none.
            let text = if anchor.is_some() || value == 0.0 {
                label
            } else {
                format!("{label}{unit}")
            };
            let width = ui
                .painter()
                .layout_no_wrap(text.clone(), font.clone(), Color32::PLACEHOLDER)
                .size()
                .x;
            (value, text, width)
        })
        .collect();
    ValueAxis { ticks, anchor }
}

/// One value axis: its ticks, and the anchor its labels are offsets from when
/// the range is too narrow against its own level to label absolutely.
struct ValueAxis {
    ticks: Vec<(f64, String, f32)>,
    anchor: Option<String>,
}

/// Roughly how far apart two value ticks are drawn.
const VALUE_TICK_PITCH: f32 = 44.0;
/// Roughly how far apart two time ticks are drawn: a label is about sixty
/// points wide and wants air either side.
const TIME_TICK_PITCH: f32 = 110.0;

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

/// The grid and both axes' labels: one line per tick, at the tick.
///
/// The time axis comes off the same ladder as the value axis, so a grid line
/// and the label under it are the same number. Labels are laid left to right
/// and one that would run into the label before it, or off the plot, is not
/// painted; its grid line still is.
fn paint_grid(ui: &Ui, projector: &Projector, color: Color32, axis: &ValueAxis) {
    let plot = projector.plot;
    let tokens = Tokens::get(ui.ctx());
    let font = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
    let stroke = Stroke::new(0.5, color);
    let target = ((plot.width() / TIME_TICK_PITCH).floor() as usize).clamp(2, 8);
    let mut painted_to = f32::NEG_INFINITY;
    for (time, label) in crate::ui::plot::linear_ticks(0.0, projector.span, target).ticks {
        let x = projector.x(time);
        ui.painter().vline(x, plot.y_range(), stroke);
        let text = if time == 0.0 {
            label
        } else {
            format!("{label}s")
        };
        let galley = ui
            .painter()
            .layout_no_wrap(text, font.clone(), tokens.color.text_faint);
        let left = (x - galley.size().x * 0.5).clamp(
            plot.left(),
            (plot.right() - galley.size().x).max(plot.left()),
        );
        if left < painted_to + 8.0 {
            continue;
        }
        painted_to = left + galley.size().x;
        ui.painter().galley(
            Pos2::new(left, plot.bottom() + 2.0),
            galley,
            tokens.color.text_faint,
        );
    }
    for (value, text, _) in &axis.ticks {
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
    if let Some(anchor) = &axis.anchor {
        ui.painter().text(
            Pos2::new(plot.left() + 4.0, plot.top() + 2.0),
            Align2::LEFT_TOP,
            anchor,
            font,
            tokens.color.text_faint,
        );
    }
}

/// The breakpoint rules and their labels.
///
/// Every rule is drawn where it is. Labels are another matter: a pulse with
/// microsecond edges in a millisecond window puts `TD` and `TR` a pixel apart,
/// and two labels a pixel apart are one smear. Rules whose labels would touch
/// share one label, in time order (`TD·TR`), hung on the first of them.
fn paint_guides(ui: &Ui, projector: &Projector, guides: &[Guide], color: Color32) {
    let plot = projector.plot;
    let tokens = Tokens::get(ui.ctx());
    let font = theme::mono(tokens::FS_MICRO, FontWeight::Regular);
    let width = |text: &str| {
        ui.painter()
            .layout_no_wrap(text.to_owned(), font.clone(), tokens.color.text_faint)
            .size()
            .x
    };
    let mut groups: Vec<(f32, String)> = Vec::new();
    for guide in guides {
        let x = projector.x(guide.time);
        ui.painter().vline(
            x,
            plot.y_range(),
            Stroke::new(1.0, color.gamma_multiply(0.45)),
        );
        let touches = groups
            .last()
            .is_some_and(|(start, label)| x < start + 2.0 + width(label) + GUIDE_LABEL_GAP);
        match groups.last_mut().filter(|_| touches) {
            Some((_, label)) => {
                label.push('\u{b7}');
                label.push_str(guide.label);
            }
            None => groups.push((x, guide.label.to_owned())),
        }
    }
    for (x, label) in groups {
        let galley = ui
            .painter()
            .layout_no_wrap(label, font.clone(), tokens.color.text_faint);
        // A rule near the right edge carries its tag on its left, so the tag
        // stays on the plot it names rather than running into the gutter.
        let left = if x + 2.0 + galley.size().x <= plot.right() {
            x + 2.0
        } else {
            (x - 2.0 - galley.size().x).max(plot.left())
        };
        ui.painter()
            .galley(Pos2::new(left, plot.top()), galley, tokens.color.text_faint);
    }
}

/// Air two guide labels keep between them before they are read as one.
const GUIDE_LABEL_GAP: f32 = 6.0;

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
            if selected {
                colors.accent
            } else {
                colors.bg_app
            },
            Stroke::new(1.2, colors.accent),
        );
        let hit = Rect::from_center_size(center, Vec2::splat(MARKER_GRAB * 2.0));
        if response
            .hover_pos()
            .is_some_and(|pointer| hit.contains(pointer))
        {
            over_marker = true;
            if response.clicked() {
                actions.push(StageAction::SelectPoint(Some(marker.index)));
            }
        }
    }
    over_marker
}

/// The unit the hover readout states its value in.
fn unit(stage: &Stage) -> &'static str {
    match stage.working.kind() {
        crate::state::stimulus_library::definition::StimulusKind::Voltage => "V",
        crate::state::stimulus_library::definition::StimulusKind::Current => "A",
    }
}
