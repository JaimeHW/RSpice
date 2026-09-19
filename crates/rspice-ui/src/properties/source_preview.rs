//! The one painter of an independent source's engine-evaluated waveform.
//!
//! Three surfaces draw this curve — the component editor's evidence pane, the
//! stimulus link dialog's stacked before/after strips, and the 44 point minis
//! in that dialog's definition list — and they have to agree down to the tick
//! labels. Three implementations of the source families used to exist, which is
//! the defect `simulation::stimulus_realize` closed for the values; this closes
//! the same class for the drawing.
//!
//! It owns no semantics. The component becomes a card, the card becomes a
//! `SourceSpec` through the engine's own parser, and the curve is the transient
//! evaluator stepped over the window; everything here is geometry and type.
//!
//! Ticks go through the product's engineering formatter
//! ([`crate::ui::plot::si_tick_label`]), the same trimming and the same
//! typographic minus every other axis in the instrument uses. They used to be
//! `{:.2e}`, so a 3 mV sine was labelled `3.00e-3` on a surface whose whole job
//! is to say what a source does.

use egui::{Align2, Rect, Sense, Stroke, Ui, Vec2, pos2, vec2};

use crate::simulation::stimulus_realize::{self, PreviewTiming, WaveformReadouts, WaveformTrace};
use crate::simulation::table_route::TableSources;
use crate::state::Component;
use crate::state::stimulus_library::definition::{StimulusDefinition, StimulusFamily};
use crate::ui::plot::si_tick_label;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

/// How many points a preview asks the engine for.
///
/// Finer than the stroke at the card's widest, and the density the card has
/// always drawn at.
const PREVIEW_SAMPLES: usize = 96;

/// How many columns a list mini resolves: more than one per point of its
/// width, so a shape repeating faster than the slot can show is drawn as the
/// band it covers rather than as a scribble.
pub(crate) const MINI_SAMPLES: usize = 64;

/// The inset well a list mini is drawn in.
pub(crate) const MINI_SIZE: Vec2 = Vec2::new(44.0, 20.0);

/// How a stimulus surface spells a number beside its unit.
///
/// The display form every axis label, tick and readout on these surfaces uses,
/// with a space before its unit: the deck's own `2us` belongs in a netlist
/// column, not two lines under a plot whose axis says `2 µs`. Two significant
/// figures is what a list row can carry and what separates two definitions of a
/// family.
///
/// Here rather than in one of the three surfaces that spell numbers this way,
/// because the link dialog's row and the library browser's row state the same
/// figure about the same definition and a reader moving between them must not
/// meet two spellings of it.
pub(crate) fn display_spelling(value: f64, unit: &str) -> String {
    si_tick_label(value, unit, 2)
}

/// One definition's evaluated mini, keyed by name and revision.
///
/// Every list that draws a library beside its names holds one of these: the
/// link dialog's pick list and the Stimulus Library's browser. Evaluating per
/// frame is one engine parse and [`MINI_SAMPLES`] steps per definition per
/// frame — for a twenty-four row library, 1,536 sample steps for every pointer
/// move, and nothing about the drawing would show it.
///
/// The revision is part of the key so a definition published in another surface
/// while the list is up redraws rather than showing the picture it had.
pub(crate) type MiniCache = std::collections::HashMap<(String, u32), Result<WaveformTrace, String>>;

/// Evaluate every definition's mini that is not already held, and report how
/// many were evaluated.
///
/// The count is what the tests read: a claim about per-frame cost is only worth
/// making if something can hold the surface to it.
pub(crate) fn ensure_minis(
    cache: &mut MiniCache,
    library: &crate::state::StimulusLibrary,
    timing: PreviewTiming,
) -> usize {
    let mut evaluated = 0;
    for definition in library.definitions() {
        let key = (definition.name().to_owned(), definition.revision());
        if cache.contains_key(&key) {
            continue;
        }
        // Over the shape's own fit span, which is the window the instrument
        // opens a definition in: a list row says what a definition is, and a
        // forty millisecond brownout drawn over a one millisecond transient is
        // a flat line that says nothing.
        cache.insert(
            key,
            stimulus_realize::shape_trace(&definition.preview_component(), MINI_SAMPLES, timing),
        );
        evaluated += 1;
    }
    evaluated
}

/// What a row shows before its mini has been evaluated, which is only ever the
/// frame a definition is added on.
pub(crate) static NO_MINI: std::sync::LazyLock<Result<WaveformTrace, String>> =
    std::sync::LazyLock::new(|| Ok(WaveformTrace::Curve(Vec::new())));

/// This source's waveform over `timing`, or why the engine cannot draw one.
///
/// The generator's and the parser's refusals already name the component and
/// the field, so they are carried through unrewritten: a preview that reworded
/// them would be a second voice saying the same thing differently.
///
/// `tables` is what a file-backed source's table can be found with here, so
/// the preview reads the file the run would.
pub(crate) fn source_curve(
    component: &Component,
    timing: PreviewTiming,
    tables: TableSources<'_>,
) -> Result<WaveformTrace, String> {
    let component = stimulus_realize::reading_reachable_table(component, tables);
    let spec = stimulus_realize::source_spec(&component)?;
    match stimulus_realize::preview_defect(&spec) {
        Some(defect) => Err(defect),
        None => Ok(stimulus_realize::sample_trace(
            &spec,
            timing.window(PREVIEW_SAMPLES),
            timing,
        )),
    }
}

/// The window the Save strip draws a placed source over, and the caption that
/// says what that window is.
///
/// The strip shows the card the dialog is about to publish as the instance
/// drives it today, under the project's own transient, and the caption states
/// exactly that window. The list minis do not come through here: a stored
/// definition is drawn over its own fit span
/// ([`stimulus_realize::shape_trace`]), because a row says what a shape is
/// rather than what one run makes of it.
pub(crate) fn shape_window(timing: PreviewTiming) -> (PreviewTiming, String) {
    let caption = timing.caption();
    (timing, caption)
}

/// One row of a stacked waveform instrument.
///
/// A row carries no geometry of its own. Two waveforms a reader is comparing
/// have to be read across as well as along — is this edge above that one, does
/// this level sit where that one did — and a row that chose its own height or
/// its own left edge is a row that cannot be read across. The stack owns every
/// dimension; a row owns only what it draws.
pub(crate) struct Strip<'a> {
    /// The engine's samples, or its refusal.
    pub curve: &'a Result<WaveformTrace, String>,
    /// The quantity the value labels carry: `V` for a voltage source, `A` for
    /// a current source.
    pub unit: &'a str,
    /// Accent for the card a transaction would leave behind, neutral for the
    /// card that is there now.
    pub accent: bool,
    /// What this row shows, in its own caption row above its plot.
    pub caption: &'a str,
    /// What a screen reader is told this row is.
    pub label: &'a str,
}

/// A stack of waveforms drawn as one instrument.
pub(crate) struct Instrument<'a> {
    /// The rows, top to bottom.
    pub strips: &'a [Strip<'a>],
    /// How tall each plotted area is. One number for every row: a taller
    /// "after" over a shorter "now" reads as two unrelated pictures.
    pub row_height: f32,
    /// The last sample time, which the shared time axis is spread across.
    pub stop: f64,
    /// The window every row was drawn over, stated once and right-aligned in
    /// the first caption row, because it is a fact about the instrument rather
    /// than about any one row of it.
    pub window: Option<&'a str>,
}

/// One row's caption row and plotted area.
pub(crate) struct StripRow {
    /// The caption's own row, above the plot and never overlapping it.
    pub caption: Rect,
    /// The plotted area. Every row's is the same size and the same x range,
    /// and nothing but the grid and the trace is ever drawn inside it.
    pub plot: Rect,
}

/// The height one plotted row claims.
pub(crate) const STRIP_ROW: f32 = 64.0;
/// The row a caption sits in, inside the frame and above its plot.
const CAPTION_ROW: f32 = 14.0;
/// Air between a caption and the plot it names.
const CAPTION_GAP: f32 = 2.0;
/// Air either side of the divider between two rows.
///
/// A divider laid straight onto the plot above it and the caption below it
/// reads as no gap at all: the eye takes the whole frame for one crowded
/// picture instead of two rows it can compare.
const DIVIDER_PAD: f32 = 8.0;
/// Air inside a plot, above its top tick and below its bottom tick, so a trace
/// at either extreme never touches the divider or a caption row.
const PLOT_PAD: f32 = 6.0;
/// Room under the frame for the shared time axis.
const X_AXIS_HEIGHT: f32 = 18.0;
/// Air between the frame and its time axis labels.
const X_AXIS_GAP: f32 = 4.0;
/// Air between a value label and the plot it labels.
const GUTTER_GAP: f32 = 6.0;

/// The vertical track a stack claims, its time axis included.
///
/// A surface has to reserve this before it paints, and a second arithmetic for
/// it elsewhere is how a reserved height and a painted height come to differ.
pub(crate) fn instrument_height(rows: usize, row_height: f32) -> f32 {
    let dividers = rows.saturating_sub(1) as f32;
    2.0 + rows as f32 * (CAPTION_ROW + CAPTION_GAP + row_height)
        + dividers * (2.0 * DIVIDER_PAD + 1.0)
        + X_AXIS_HEIGHT
}

/// Where each row sits inside one frame.
///
/// Stated as its own function because it is the claim the stack is for: every
/// plot has the same width, the same left and right edges and the same height,
/// so a time on one row is directly above the same time on the next; and every
/// caption sits in a row of its own, clear of the data, because a caption drawn
/// over a plot is a line the curve passes through.
pub(crate) fn strip_rows(frame: Rect, row_height: f32, rows: usize) -> Vec<StripRow> {
    let inside = frame.shrink(1.0);
    let block = CAPTION_ROW + CAPTION_GAP + row_height + 2.0 * DIVIDER_PAD + 1.0;
    (0..rows)
        .map(|index| {
            let top = inside.top() + index as f32 * block;
            let plot_top = top + CAPTION_ROW + CAPTION_GAP;
            StripRow {
                caption: Rect::from_min_max(
                    pos2(inside.left(), top),
                    pos2(inside.right(), top + CAPTION_ROW),
                ),
                plot: Rect::from_min_max(
                    pos2(inside.left(), plot_top),
                    pos2(inside.right(), plot_top + row_height),
                ),
            }
        })
        .collect()
}

/// Paint a stack of waveforms as one instrument: one frame, one value gutter,
/// one time axis, and every plot the same size.
///
/// The gutter is measured from the widest label *any* row will print and is
/// then used by all of them, so the labels are right-aligned against a plot
/// edge the rows share and the widest of them starts exactly at the left edge
/// the rest of the pane uses. A per-row gutter puts the two time axes out of
/// register; a fixed one leaves a band of nothing between the pane's edge and
/// the numbers.
pub(crate) fn paint_instrument(ui: &mut Ui, instrument: &Instrument<'_>) {
    let t = Tokens::get(ui.ctx());
    let strips = instrument.strips;
    let plans: Vec<RowPlan> = strips.iter().map(RowPlan::of).collect();
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let caption_font = theme::sans(tokens::FS_MICRO, FontWeight::Regular);
    let gutter = plans
        .iter()
        .flat_map(|plan| plan.labels.iter().flatten())
        .map(|label| measured(ui, label, font.clone()))
        .fold(0.0_f32, f32::max)
        + GUTTER_GAP;

    let height = instrument_height(strips.len(), instrument.row_height);
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
    let announcement = strips
        .iter()
        .zip(&plans)
        .map(|(strip, plan)| plan.announcement(strip))
        .collect::<Vec<_>>()
        .join(". ");
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Image, true, &announcement));

    let frame = Rect::from_min_max(
        pos2(rect.left() + gutter, rect.top()),
        pos2(rect.right(), rect.bottom() - X_AXIS_HEIGHT),
    );
    ui.painter().rect_filled(frame, t.radius, t.color.canvas_bg);
    for (index, (row, strip)) in strip_rows(frame, instrument.row_height, strips.len())
        .into_iter()
        .zip(strips)
        .enumerate()
    {
        if index > 0 {
            ui.painter().hline(
                frame.x_range(),
                row.caption.top() - DIVIDER_PAD - 0.5,
                Stroke::new(1.0, t.color.border),
            );
        }
        // A caption row stands on the canvas ground, not on the pane's, and the
        // light palette's canvas is a mid grey that swallows both quiet text
        // registers. These captions therefore take the full-strength colour and
        // stay quiet by being micro-sized; the row caption and the window are
        // told apart by their faces and their alignment, never by one of them
        // being harder to read.
        ui.painter().text(
            pos2(row.caption.left(), row.caption.center().y),
            Align2::LEFT_CENTER,
            row_caption(strip),
            caption_font.clone(),
            t.color.text,
        );
        if index == 0
            && let Some(window) = instrument.window
        {
            ui.painter().text(
                pos2(row.caption.right() - 6.0, row.caption.center().y),
                Align2::RIGHT_CENTER,
                window,
                theme::mono(tokens::FS_MICRO, FontWeight::Regular),
                t.color.text,
            );
        }
        let window = (0.0, instrument.stop);
        paint_row(ui, &t, row.plot, window, strip, &plans[index], font.clone());
    }
    // The border last, so no gridline or trace lies on top of the frame.
    ui.painter().rect_stroke(
        frame,
        t.radius,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    paint_time_axis(ui, frame, instrument.stop);
}

/// What one row will print, worked out before anything is painted so the stack
/// can size its gutter from every row at once.
struct RowPlan {
    readouts: Option<WaveformReadouts>,
    flat: bool,
    /// The three value labels, top to bottom. A flat row prints only its
    /// middle one, because a band of nothing has no top and no bottom.
    labels: [Option<String>; 3],
}

impl RowPlan {
    fn of(strip: &Strip<'_>) -> Self {
        let readouts = strip.curve.as_ref().ok().and_then(WaveformTrace::readouts);
        let Some(readouts) = readouts else {
            return Self {
                readouts: None,
                flat: true,
                labels: [None, None, None],
            };
        };
        let flat = readouts.span() <= f64::EPSILON * readouts.maximum.abs().max(1.0);
        let digits = tick_digits(if flat {
            readouts.maximum.abs()
        } else {
            readouts.span()
        });
        let label = |value: f64| Some(axis_label(value, strip.unit, digits));
        Self {
            readouts: Some(readouts),
            flat,
            labels: if flat {
                [None, label(readouts.maximum), None]
            } else {
                [
                    label(readouts.maximum),
                    label(readouts.midpoint),
                    label(readouts.minimum),
                ]
            },
        }
    }

    fn announcement(&self, strip: &Strip<'_>) -> String {
        match (strip.curve, self.readouts) {
            (Ok(_), Some(_)) => format!("{}: {}", row_caption(strip), strip.label),
            (Ok(_), None) => format!("{}: no curve", strip.caption),
            (Err(reason), _) => format!("{}: {reason}", strip.caption),
        }
    }
}

/// One row's gridlines, value labels and trace.
///
/// Nothing else is ever drawn between these edges: a caption, a note or a
/// legend inside a plot is a run of text the trace passes through, and the
/// reader cannot tell which of the two they are looking at.
fn paint_row(
    ui: &mut Ui,
    t: &Tokens,
    plot: Rect,
    window: (f64, f64),
    strip: &Strip<'_>,
    plan: &RowPlan,
    font: egui::FontId,
) {
    let Some(readouts) = plan.readouts else {
        let message = match strip.curve {
            Err(reason) => reason.as_str(),
            Ok(_) => "No curve over this window",
        };
        ui.painter().text(
            plot.center(),
            Align2::CENTER_CENTER,
            message,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
        return;
    };
    // The value range maps to the padded band, never to the whole plot: a
    // trace at its own maximum otherwise runs along the edge of the row above
    // it and the reader cannot tell the waveform from the frame.
    let band = Rect::from_min_max(
        pos2(plot.left(), plot.top() + PLOT_PAD),
        pos2(plot.right(), plot.bottom() - PLOT_PAD),
    );
    for (index, fraction) in [1.0_f32, 0.5, 0.0].into_iter().enumerate() {
        let Some(label) = plan.labels[index].as_deref() else {
            continue;
        };
        let y = egui::lerp(band.bottom()..=band.top(), fraction);
        ui.painter()
            .hline(band.x_range(), y, Stroke::new(0.5, t.color.canvas_grid));
        ui.painter().text(
            pos2(band.left() - GUTTER_GAP, y),
            Align2::RIGHT_CENTER,
            label,
            font.clone(),
            t.color.text_faint,
        );
    }
    let Ok(trace) = strip.curve else { return };
    let ink = if strip.accent {
        t.color.accent
    } else {
        t.color.text_dim
    };
    let placement = Placement {
        band,
        window,
        readouts,
        flat: plan.flat,
    };
    paint_trace(ui.painter(), trace, &placement, ink, 1.5);
}

/// A row's caption, with what its trace calls itself where that is not simply
/// a curve.
///
/// A band says it is one in the row that draws it: read as a curve, it is a
/// waveform with a period nobody authored.
fn row_caption(strip: &Strip<'_>) -> String {
    match strip.curve.as_ref().ok().and_then(WaveformTrace::caption) {
        Some(kind) => format!("{} \u{b7} {kind}", strip.caption),
        None => strip.caption.to_owned(),
    }
}

/// Where a trace's times and values land on screen.
struct Placement {
    /// The band the value range maps to.
    band: Rect,
    /// The times the band's left and right edges stand for.
    window: (f64, f64),
    readouts: WaveformReadouts,
    /// A flat trace runs along the middle: a band of nothing has no top and no
    /// bottom to be drawn against.
    flat: bool,
}

impl Placement {
    /// Placed by time, not by index: a trace carries the waveform's own
    /// breakpoints between its grid samples, so its points are not evenly
    /// spaced.
    fn point(&self, time: f64, value: f64) -> egui::Pos2 {
        let (start, stop) = self.window;
        let along = ((time - start) / (stop - start).max(f64::MIN_POSITIVE)).clamp(0.0, 1.0);
        let up = if self.flat {
            0.5
        } else {
            ((value - self.readouts.minimum) / self.readouts.span().max(1e-30)) as f32
        };
        pos2(
            egui::lerp(self.band.left()..=self.band.right(), along as f32),
            egui::lerp(self.band.bottom()..=self.band.top(), up),
        )
    }
}

/// A trace in whichever mode the engine measured it: a polyline through its
/// samples, or the band its columns cover.
fn paint_trace(
    painter: &egui::Painter,
    trace: &WaveformTrace,
    placement: &Placement,
    ink: egui::Color32,
    width: f32,
) {
    match trace {
        WaveformTrace::Curve(samples) => {
            let points = samples
                .iter()
                .map(|(time, value)| placement.point(*time, *value))
                .collect::<Vec<_>>();
            painter.add(egui::Shape::line(points, Stroke::new(width, ink)));
        }
        WaveformTrace::Envelope { columns, .. } => {
            let band = columns
                .iter()
                .map(|column| {
                    (
                        placement.point(column.time, column.minimum),
                        placement.point(column.time, column.maximum),
                    )
                })
                .collect::<Vec<_>>();
            crate::ui::plot::paint_min_max_band(painter, &band, ink, width);
        }
    }
}

/// The first and last time a trace holds, which is the window a mini with no
/// axis of its own is spread across.
fn extent(trace: &WaveformTrace) -> (f64, f64) {
    let ends = match trace {
        WaveformTrace::Curve(samples) => {
            samples.first().zip(samples.last()).map(|(a, b)| (a.0, b.0))
        }
        WaveformTrace::Envelope { columns, .. } => columns
            .first()
            .zip(columns.last())
            .map(|(a, b)| (a.time, b.time)),
    };
    ends.unwrap_or((0.0, 1.0))
}

/// The shared time axis, labelled once under the whole frame.
fn paint_time_axis(ui: &Ui, frame: Rect, stop: f64) {
    let t = Tokens::get(ui.ctx());
    let digits = tick_digits(stop);
    for (fraction, value) in [(0.0_f32, 0.0), (0.5, stop * 0.5), (1.0, stop)] {
        let x = egui::lerp(frame.left()..=frame.right(), fraction);
        let align = match fraction {
            f if f <= 0.0 => Align2::LEFT_TOP,
            f if f >= 1.0 => Align2::RIGHT_TOP,
            _ => Align2::CENTER_TOP,
        };
        ui.painter().text(
            pos2(x, frame.bottom() + X_AXIS_GAP),
            align,
            axis_label(value, "s", digits),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
}

/// How wide one run lays out.
fn measured(ui: &mut Ui, text: &str, font: egui::FontId) -> f32 {
    ui.fonts_mut(|fonts| {
        fonts
            .layout_no_wrap(text.to_owned(), font, egui::Color32::WHITE)
            .size()
            .x
    })
}

/// The two levels a curve runs between, for a row that has no room for a plot
/// axis: `0 to 3.3 V`, or the one level a flat curve holds.
///
/// Read off the evaluated samples through [`WaveformReadouts`], never off the
/// parameter string: a row that parsed `V2=` itself would disagree with the
/// mini beside it the moment the engine substituted a field.
pub(crate) fn level_span_label(readouts: WaveformReadouts, unit: &str) -> String {
    let digits = tick_digits(if readouts.span() > 0.0 {
        readouts.span()
    } else {
        readouts.maximum.abs()
    });
    if readouts.span() <= f64::EPSILON * readouts.maximum.abs().max(1.0) {
        return axis_label(readouts.maximum, unit, digits);
    }
    format!(
        "{} to {}",
        axis_label(readouts.minimum, unit, digits),
        axis_label(readouts.maximum, unit, digits)
    )
}

/// One axis tick, through the product's engineering formatter.
///
/// Zero is `0` with no prefix and no unit: `0 V` and `0 s` are noise on an axis
/// whose other labels already carry the quantity, and the formatter's own
/// zero-handling would print exactly that.
fn axis_label(value: f64, unit: &str, digits: usize) -> String {
    if value == 0.0 {
        return "0".to_owned();
    }
    si_tick_label(value, unit, digits)
}

/// How many decimals an axis asks for to separate its ticks.
///
/// Derived from the span the ticks divide rather than fixed, and read after the
/// formatter has trimmed the zeros it did not need: two places resolve a 3 mV
/// sine into `3 mV` and `1.5 mV`, and the same two places print a 1 ms window's
/// ticks as `1 ms` and `500 µs` rather than as three labels agreeing on
/// everything but their leading digit.
fn tick_digits(span: f64) -> usize {
    if !span.is_finite() || span <= 0.0 {
        return 0;
    }
    // `si_tick_label` scales into the 1..1000 decade, so the mantissa of the
    // span is what decides: 500 needs none, 1.5 needs one.
    let mantissa = span / 10f64.powi(((span.abs().log10() / 3.0).floor() * 3.0) as i32);
    if mantissa >= 100.0 {
        0
    } else if mantissa >= 10.0 {
        1
    } else {
        2
    }
}

/// A 44 x 20 point mini of one stored waveform, in an inset well.
///
/// Painted rather than added as a widget so a clickable list row can place it
/// inside its own rect: a label or an image widget over a click rect eats the
/// press that was meant for the row.
pub(crate) fn paint_mini(
    painter: &egui::Painter,
    t: &Tokens,
    rect: Rect,
    curve: &Result<WaveformTrace, String>,
    gap: MiniGap,
    enabled: bool,
) {
    painter.rect(
        rect,
        3.0,
        t.color.bg_inset,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    let ink = if enabled {
        t.color.accent
    } else {
        t.color.text_faint
    };
    let plot = rect.shrink(3.0);
    let readouts = curve.as_ref().ok().and_then(WaveformTrace::readouts);
    match (curve, readouts) {
        (Ok(trace), Some(readouts)) => {
            let placement = Placement {
                band: plot,
                window: extent(trace),
                readouts,
                flat: readouts.span() <= f64::EPSILON * readouts.maximum.abs().max(1.0),
            };
            paint_trace(painter, trace, &placement, ink, 1.0);
        }
        // A card the engine refused, for a definition that should have drawn,
        // is the one case a mark must not read as data: the flat line this
        // used to draw is a level the definition does not hold.
        (Err(_), _) if !gap.ordinary => crate::ui::widgets::paint_status_mark(
            painter,
            Rect::from_center_size(plot.center(), Vec2::splat(plot.height())),
            crate::ui::widgets::StatusMark::Warning,
            t.color.warn,
        ),
        _ => paint_boundary_mark(painter, plot, gap.family, t.color.text_faint),
    }
}

/// What a mini draws in the slot a curve would have filled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MiniGap {
    /// The family whose mark stands in for the curve.
    family: StimulusFamily,
    /// Whether having no curve is ordinary for this definition, so the mark is
    /// the family's own rather than a warning.
    ordinary: bool,
}

impl MiniGap {
    /// What a definition with no curve shows.
    ///
    /// Two cases are ordinary. A noise train, a random train and a `PWL FILE`
    /// table have no waveform until a run builds them. And a definition that
    /// names a design variable (`{VSUP}` for a supply level is how most
    /// libraries are written) resolves when a deck knows its own `.param`s,
    /// which a preview parsing against an empty scope never does. A warning on
    /// either would teach a reader to ignore the mark.
    pub(crate) fn of(definition: &StimulusDefinition) -> Self {
        let family = definition.family();
        Self {
            family,
            ordinary: boundary_family(family)
                || stimulus_realize::names_design_variable(definition),
        }
    }
}

/// Whether this family has no waveform at this boundary at all, as opposed to
/// one the engine declined to evaluate.
///
/// The three are `preview_defect`'s own: a noise train and a random train are
/// expanded when a transient starts, and a `PWL FILE` table is read from a path
/// this process may not be able to reach.
const fn boundary_family(family: StimulusFamily) -> bool {
    matches!(
        family,
        StimulusFamily::Trnoise | StimulusFamily::Trrandom | StimulusFamily::PwlFile
    )
}

/// The family's own mark, drawn as geometry.
///
/// Vectors rather than glyphs: the bundled text faces carry no sine wave, pulse
/// train or noise band, and a missing glyph is a tofu box in the one place a
/// reader looks first to know what they are looking at. Three surfaces show it
/// — the instrument's identity band, the library browser's group headings, and
/// any list that names a family without room for a curve — so it is painted
/// once, at whatever size the caller's rectangle states.
pub(crate) fn paint_family_mark(
    painter: &egui::Painter,
    rect: Rect,
    family: StimulusFamily,
    color: egui::Color32,
) {
    let stroke = Stroke::new(1.3, color);
    let box_rect = rect.shrink(2.0);
    let x = |fraction: f32| egui::lerp(box_rect.left()..=box_rect.right(), fraction);
    let y = |fraction: f32| egui::lerp(box_rect.bottom()..=box_rect.top(), fraction);
    let line = |points: Vec<egui::Pos2>| painter.add(egui::Shape::line(points, stroke));
    match family {
        StimulusFamily::Dc => {
            line(vec![pos2(x(0.0), y(0.5)), pos2(x(1.0), y(0.5))]);
        }
        StimulusFamily::Ac => {
            line(vec![pos2(x(0.0), y(0.5)), pos2(x(1.0), y(0.5))]);
            line(family_sine(&x, &y, 1.0, 0.18));
        }
        StimulusFamily::Sin => {
            line(family_sine(&x, &y, 1.0, 0.42));
        }
        StimulusFamily::Pulse => {
            line(vec![
                pos2(x(0.0), y(0.15)),
                pos2(x(0.2), y(0.15)),
                pos2(x(0.2), y(0.85)),
                pos2(x(0.6), y(0.85)),
                pos2(x(0.6), y(0.15)),
                pos2(x(1.0), y(0.15)),
            ]);
        }
        StimulusFamily::Pwl | StimulusFamily::PwlFile => {
            line(vec![
                pos2(x(0.0), y(0.2)),
                pos2(x(0.3), y(0.85)),
                pos2(x(0.6), y(0.4)),
                pos2(x(1.0), y(0.6)),
            ]);
        }
        StimulusFamily::Exp => {
            line(
                (0..=12_u8)
                    .map(|step| {
                        let fraction = f32::from(step) / 12.0;
                        pos2(x(fraction), y(0.15 + 0.7 * (1.0 - (-3.0 * fraction).exp())))
                    })
                    .collect(),
            );
        }
        StimulusFamily::Sffm => {
            line(family_sine(&x, &y, 3.0, 0.35));
        }
        StimulusFamily::Am => {
            line(family_sine(&x, &y, 4.0, 0.42));
            line(vec![
                pos2(x(0.0), y(0.6)),
                pos2(x(0.5), y(0.95)),
                pos2(x(1.0), y(0.6)),
            ]);
        }
        StimulusFamily::Pat => {
            line(vec![
                pos2(x(0.0), y(0.15)),
                pos2(x(0.25), y(0.15)),
                pos2(x(0.25), y(0.85)),
                pos2(x(0.5), y(0.85)),
                pos2(x(0.5), y(0.15)),
                pos2(x(0.75), y(0.15)),
                pos2(x(0.75), y(0.85)),
                pos2(x(1.0), y(0.85)),
            ]);
        }
        StimulusFamily::Trnoise => {
            line(
                (0..=16_u8)
                    .map(|step| {
                        let fraction = f32::from(step) / 16.0;
                        let jitter =
                            [0.5, 0.8, 0.3, 0.65, 0.2, 0.75, 0.45, 0.6][usize::from(step) % 8];
                        pos2(x(fraction), y(jitter))
                    })
                    .collect(),
            );
        }
        StimulusFamily::Trrandom => {
            line(vec![
                pos2(x(0.0), y(0.4)),
                pos2(x(0.25), y(0.4)),
                pos2(x(0.25), y(0.8)),
                pos2(x(0.5), y(0.8)),
                pos2(x(0.5), y(0.25)),
                pos2(x(0.75), y(0.25)),
                pos2(x(0.75), y(0.6)),
                pos2(x(1.0), y(0.6)),
            ]);
        }
    }
}

/// `cycles` periods of a sine across a family mark, at `amplitude` of its
/// height.
fn family_sine(
    x: &impl Fn(f32) -> f32,
    y: &impl Fn(f32) -> f32,
    cycles: f32,
    amplitude: f32,
) -> Vec<egui::Pos2> {
    (0..=24_u8)
        .map(|step| {
            let fraction = f32::from(step) / 24.0;
            pos2(
                x(fraction),
                y(0.5 + amplitude * (fraction * cycles * std::f32::consts::TAU).sin()),
            )
        })
        .collect()
}

/// The mark a family with no waveform at this boundary shows instead of a
/// curve.
///
/// Drawn as vectors: the bundled Plex faces carry no glyph for any of these, so
/// a character here would rasterize as a tofu box, and the three families that
/// reach this arm are exactly the three the engine cannot step through until a
/// run builds them.
///
/// Deliberately not [`paint_family_mark`]: this arm is drawn because there is
/// no waveform, and the identity mark is a picture of one.
fn paint_boundary_mark(
    painter: &egui::Painter,
    plot: Rect,
    family: StimulusFamily,
    ink: egui::Color32,
) {
    let stroke = Stroke::new(1.0, ink);
    let middle = plot.center().y;
    match family {
        // A noise train: many small excursions about the level it holds.
        StimulusFamily::Trnoise => {
            let amplitudes = [
                0.0_f32, 0.7, -0.4, 0.9, -0.8, 0.3, -0.6, 0.5, -0.9, 0.4, 0.0,
            ];
            let last = (amplitudes.len() - 1) as f32;
            let points = amplitudes
                .iter()
                .enumerate()
                .map(|(index, amplitude)| {
                    pos2(
                        egui::lerp(plot.left()..=plot.right(), index as f32 / last),
                        middle + amplitude * plot.height() * 0.4,
                    )
                })
                .collect::<Vec<_>>();
            painter.add(egui::Shape::line(points, stroke));
        }
        // A random sample train: held levels with abrupt transitions.
        StimulusFamily::Trrandom => {
            let levels = [0.5_f32, -0.6, 0.2, -0.3];
            let mut points = Vec::with_capacity(levels.len() * 2);
            for (index, level) in levels.iter().enumerate() {
                let left = egui::lerp(
                    plot.left()..=plot.right(),
                    index as f32 / levels.len() as f32,
                );
                let right = egui::lerp(
                    plot.left()..=plot.right(),
                    (index + 1) as f32 / levels.len() as f32,
                );
                let y = middle + level * plot.height() * 0.4;
                points.push(pos2(left, y));
                points.push(pos2(right, y));
            }
            painter.add(egui::Shape::line(points, stroke));
        }
        // Everything else that reaches this arm is a table the engine could not
        // read: the level it does hold, drawn as the flat line it is.
        _ => {
            painter.hline(plot.x_range(), middle, stroke);
        }
    }
}

/// The component editor's preview card: one waveform row in its own frame.
///
/// Kept beside the stack painter because the card is a one-row stack, and the
/// link dialog composes the same painter into a two-row one.
pub(crate) fn paint_preview_card(
    ui: &mut Ui,
    curve: &Result<WaveformTrace, String>,
    timing: PreviewTiming,
    unit: &str,
) {
    let caption = timing.caption();
    paint_instrument(
        ui,
        &Instrument {
            strips: &[Strip {
                curve,
                unit,
                accent: true,
                caption: "Engine-evaluated waveform",
                label: "the waveform this source produces",
            }],
            row_height: 104.0,
            stop: timing.tstop,
            window: Some(&caption),
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every tick this surface publishes goes through the engineering
    /// formatter. `3.00e-3` on a card whose whole job is to say what a source
    /// does was the defect that closed this class.
    #[test]
    fn axis_labels_are_engineering_and_never_exponent_notation() {
        for (value, unit, digits, expected) in [
            (0.0, "V", 2, "0"),
            (3.3, "V", 2, "3.3 V"),
            (3e-3, "V", 2, "3 mV"),
            (-3e-3, "V", 2, "−3 mV"),
            (1e-6, "A", 2, "1 µA"),
            (5e-4, "s", 2, "500 µs"),
            (1e-3, "s", 2, "1 ms"),
        ] {
            assert_eq!(axis_label(value, unit, digits), expected);
        }
        for span in [1e-12, 3e-3, 1.0, 5e5, 1e9] {
            for fraction in [0.0, 0.5, 1.0] {
                let label = axis_label(span * fraction, "V", tick_digits(span));
                assert!(
                    !label.contains('e') && !label.contains('E'),
                    "{label} is exponent notation"
                );
            }
        }
    }

    /// The decimal count follows the span, so three ticks over one span read
    /// as three different numbers.
    #[test]
    fn tick_decimals_separate_the_ticks_they_label() {
        for span in [1e-3, 2e-6, 3.3, 5e5] {
            let digits = tick_digits(span);
            let labels = [0.5, 0.75, 1.0]
                .map(|fraction| axis_label(span * fraction, "V", digits))
                .to_vec();
            let unique: std::collections::BTreeSet<&String> = labels.iter().collect();
            assert_eq!(unique.len(), labels.len(), "{span}: {labels:?}");
        }
    }

    /// A row with no plot axis states the pair of levels the curve runs
    /// between, read off the samples rather than off the card.
    #[test]
    fn a_level_span_states_both_rails_in_engineering_units() {
        let pulse = WaveformReadouts::of(&[(0.0, 0.0), (1.0, 3.3), (2.0, 0.0)]).expect("readouts");
        assert_eq!(level_span_label(pulse, "V"), "0 to 3.3 V");
        let bias = WaveformReadouts::of(&[(0.0, 1.8), (1.0, 1.8)]).expect("readouts");
        assert_eq!(level_span_label(bias, "V"), "1.8 V");
        let sine =
            WaveformReadouts::of(&[(0.0, -3e-3), (1.0, 3e-3), (2.0, -3e-3)]).expect("readouts");
        assert_eq!(level_span_label(sine, "V"), "−3 mV to 3 mV");
    }
}
