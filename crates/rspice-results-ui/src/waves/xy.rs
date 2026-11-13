//! A unit pane drawn against a signal or an expression instead of its
//! strip's sweep: a transfer curve, a hysteresis loop, a Lissajous figure.
//!
//! Each curve pairs its own values with the abscissa's on one grid, read
//! through the strip's window on the sweep: zooming the time axis below the
//! stack chooses which stretch of the loop is drawn. Where a curve's X turns
//! back, sparse arrowheads say which way the sweep travels; and the strip's
//! cursors, which stand on the sweep, are drawn where the curves are at that
//! instant.

use super::pane::ResolvedExpr;
use super::stack::{StackHost, StripFrame};
use super::{StripModel, StripTrace, TraceKind, UnitPane, stable_hash, trace_key};
use crate::presentation::well_hint;
use egui::Ui;
use rspice_results::visualization_document::Abscissa;
use rspice_results::waveform::SharedWaveformValues;
use rspice_ui_kit::plot::sample::sample_at_with;
use rspice_ui_kit::plot::{self, Axis, PlotSpec, SampleInterpolation, Trace, XScale};
use rspice_ui_kit::tokens::Tokens;
use rspice_ui_kit::widgets::chip;
use std::ops::Range;
use std::sync::Arc;

/// Which member of a strip a series belongs to: its run, whether that run
/// is an overlay, and its family group (0 outside a family).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MemberKey {
    pub run_id: u64,
    pub overlay: bool,
    pub presentation_key: u64,
}

impl MemberKey {
    pub fn of(trace: &StripTrace) -> Self {
        Self {
            run_id: trace.run_id,
            overlay: trace.overlay,
            presentation_key: trace.presentation_key,
        }
    }
}

/// The abscissa's values for one member, on that member's own grid.
pub struct AbscissaSeries {
    pub member: MemberKey,
    pub grid: SharedWaveformValues,
    pub values: SharedWaveformValues,
    /// Identity of these values for the resampling memo; it changes when
    /// they are evaluated again.
    pub cache_key: u64,
}

/// An abscissa resolved against one strip.
pub struct ResolvedAbscissa {
    /// The unit its values are in; empty when it has none.
    pub unit: String,
    pub series: Vec<AbscissaSeries>,
}

/// Why two operands have no common grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XyRefusal {
    /// They were sampled on different grids of a sweep that turns back.
    TurnsBack,
    /// Their sweeps do not overlap.
    Disjoint,
}

/// Both operands of one curve, on one grid.
#[derive(Debug, Clone, PartialEq)]
pub struct XyCurve {
    /// The sweep value of each point: a time, a frequency, a swept source.
    pub sweep: Vec<f64>,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    /// Whether the sweep only ever increases, so a window on it is one
    /// contiguous run of points.
    pub ascending: bool,
    /// Whether some point was read between an operand's samples.
    pub interpolated: bool,
}

fn ascending(grid: &[f64]) -> bool {
    grid.iter().all(|value| value.is_finite()) && grid.windows(2).all(|pair| pair[0] <= pair[1])
}

/// Put `y` and `x`, each sampled on its own grid, onto one grid.
///
/// Operands sampled on the same grid are paired sample by sample, exactly.
/// Otherwise the grid is every sample of either operand where both are
/// defined, and each operand is read there by `interpolation`; a sweep that
/// turns back has no such merged grid.
pub fn resample(
    (x_grid, x_values): (&[f64], &[f64]),
    (y_grid, y_values): (&[f64], &[f64]),
    interpolation: SampleInterpolation,
) -> Result<XyCurve, XyRefusal> {
    let x_len = x_grid.len().min(x_values.len());
    let y_len = y_grid.len().min(y_values.len());
    let (x_grid, x_values) = (&x_grid[..x_len], &x_values[..x_len]);
    let (y_grid, y_values) = (&y_grid[..y_len], &y_values[..y_len]);
    let same_grid = x_len == y_len
        && x_grid
            .iter()
            .zip(y_grid)
            .all(|(a, b)| a.to_bits() == b.to_bits());
    if same_grid {
        return Ok(XyCurve {
            sweep: x_grid.to_vec(),
            x: x_values.to_vec(),
            y: y_values.to_vec(),
            ascending: ascending(x_grid),
            interpolated: false,
        });
    }
    if !ascending(x_grid) || !ascending(y_grid) {
        return Err(XyRefusal::TurnsBack);
    }
    let (Some(&x_first), Some(&x_last), Some(&y_first), Some(&y_last)) =
        (x_grid.first(), x_grid.last(), y_grid.first(), y_grid.last())
    else {
        return Err(XyRefusal::Disjoint);
    };
    let (low, high) = (x_first.max(y_first), x_last.min(y_last));
    if low > high {
        return Err(XyRefusal::Disjoint);
    }
    let mut sweep: Vec<f64> = Vec::with_capacity(x_len + y_len);
    let (mut i, mut j) = (0, 0);
    while i < x_len || j < y_len {
        let next = match (x_grid.get(i), y_grid.get(j)) {
            (Some(&a), Some(&b)) if a <= b => {
                i += 1;
                a
            }
            (Some(_), Some(&b)) | (None, Some(&b)) => {
                j += 1;
                b
            }
            (Some(&a), None) => {
                i += 1;
                a
            }
            (None, None) => break,
        };
        if (low..=high).contains(&next) && sweep.last() != Some(&next) {
            sweep.push(next);
        }
    }
    let x = sweep
        .iter()
        .map(|&t| sample_at_with(x_grid, x_values, t, interpolation))
        .collect();
    let y = sweep
        .iter()
        .map(|&t| sample_at_with(y_grid, y_values, t, interpolation))
        .collect();
    Ok(XyCurve {
        sweep,
        x,
        y,
        ascending: true,
        interpolated: true,
    })
}

/// The points of `curve` whose sweep lies in `window`; every point when the
/// sweep turns back, since a window on it is no single run of points.
pub fn window(curve: &XyCurve, (start, end): (f64, f64)) -> Range<usize> {
    if !curve.ascending {
        return 0..curve.sweep.len();
    }
    let first = curve.sweep.partition_point(|&t| t < start);
    let last = curve.sweep.partition_point(|&t| t <= end);
    first..last.max(first)
}

/// The part of the plot's X span a curve must travel back before its X counts
/// as turning back: the axis' own resolution, so solver noise on a ramp is no
/// loop.
const TURN_BACK_SHARE: f64 = 1.0e-3;

/// Whether `x` turns back by more than `tolerance` anywhere: a loop, a
/// hysteresis, a Lissajous figure. Only such a curve needs arrowheads to say
/// which way it travels; on a monotonic sweep its axis says so already.
pub(crate) fn turns_back(x: &[f64], tolerance: f64) -> bool {
    let mut finite = x.iter().copied().filter(|value| value.is_finite());
    let Some(anchor) = finite.next() else {
        return false;
    };
    // The running extreme in the way X has been travelling, once it has
    // travelled past the tolerance at all.
    let mut heading: Option<(bool, f64)> = None;
    for value in finite {
        heading = match heading {
            None if (value - anchor).abs() > tolerance => Some((value > anchor, value)),
            None => None,
            Some((rising, extreme)) => {
                let back = if rising {
                    extreme - value
                } else {
                    value - extreme
                };
                if back > tolerance {
                    return true;
                }
                let further = if rising {
                    value.max(extreme)
                } else {
                    value.min(extreme)
                };
                Some((rising, further))
            }
        };
    }
    false
}

/// Screen distance between arrowheads along a curve, in points.
const ARROW_SPACING: f32 = 150.0;
/// At most this many arrowheads on one curve.
const MAX_ARROWS: usize = 24;

/// Where the arrowheads of a curve drawn through `points` go, with the way
/// each points: one every [`ARROW_SPACING`] of its length, the first half a
/// spacing in, and one at its middle when it is shorter than that. A point
/// that is not finite breaks the curve.
pub fn direction_arrows(points: &[egui::Pos2]) -> Vec<(egui::Pos2, egui::Vec2)> {
    let segments = || {
        points
            .windows(2)
            .filter(|pair| pair[0].is_finite() && pair[1].is_finite())
            .map(|pair| (pair[0], pair[1] - pair[0]))
            .filter(|(_, step)| step.length() > 1.0e-3)
    };
    let mut arrows = Vec::new();
    let mut travelled = 0.0;
    let mut next = ARROW_SPACING * 0.5;
    for (from, step) in segments() {
        let length = step.length();
        while travelled + length >= next && arrows.len() < MAX_ARROWS {
            arrows.push((from + step * ((next - travelled) / length), step / length));
            next += ARROW_SPACING;
        }
        travelled += length;
    }
    if arrows.is_empty() && travelled > 12.0 {
        let middle = travelled * 0.5;
        let mut walked = 0.0;
        for (from, step) in segments() {
            let length = step.length();
            if walked + length >= middle {
                arrows.push((from + step * ((middle - walked) / length), step / length));
                break;
            }
            walked += length;
        }
    }
    arrows
}

fn paint_arrow(painter: &egui::Painter, at: egui::Pos2, forward: egui::Vec2, color: egui::Color32) {
    let side = egui::vec2(-forward.y, forward.x);
    painter.add(egui::Shape::convex_polygon(
        vec![
            at + forward * 5.0,
            at - forward * 3.0 + side * 3.5,
            at - forward * 3.0 - side * 3.5,
        ],
        color,
        egui::Stroke::NONE,
    ));
}

/// How the pane says its curves were paired, after the abscissa's name.
fn pairing_note(sweep: &str, interpolated: bool, interpolation: SampleInterpolation) -> String {
    let sweep = if sweep.is_empty() { "sweep" } else { sweep };
    if !interpolated {
        return format!("at each {sweep} sample");
    }
    let between = match interpolation {
        SampleInterpolation::Linear => "linear",
        SampleInterpolation::MonotoneCubic => "monotone cubic",
        SampleInterpolation::Nearest => "nearest sample",
    };
    format!("at each {sweep} sample of either, {between} between")
}

/// Why a choice reads nothing on this strip, in the reader's words.
fn refusal_text(abscissa: &Abscissa, curve: &str, refusal: XyRefusal) -> String {
    match refusal {
        XyRefusal::TurnsBack => format!(
            "{curve} and {} are sampled on different grids of a sweep that turns back, \
             so they share no grid",
            abscissa.text()
        ),
        XyRefusal::Disjoint => format!(
            "{curve} and {} do not overlap in the sweep",
            abscissa.text()
        ),
    }
}

/// The point a nested sweep's column names after its quantity, as in
/// `I(VD) [VG=2; point 2]`, with its leading space.
fn member_suffix(name: &str) -> Option<&str> {
    name.ends_with(']')
        .then(|| name.rfind(" ["))
        .flatten()
        .map(|start| &name[start..])
}

/// A column's quantity without the point a nested sweep names after it.
fn member_base(name: &str) -> &str {
    member_suffix(name).map_or(name, |suffix| &name[..name.len() - suffix.len()])
}

/// The column of `abscissa` a nested sweep's member `curve` reads against:
/// the one its own point names, so `I(VD) [VG=2; point 2]` drawn against
/// `V(D) [VG=1.5; point 1]` reads `V(D) [VG=2; point 2]`.
fn member_abscissa_name(curve: &str, abscissa: &str) -> Option<String> {
    let suffix = member_suffix(curve)?;
    Some(format!("{}{suffix}", member_base(abscissa)))
}

/// The abscissa as the strip names it. A nested sweep's member column names
/// its quantity alone, `V(D)` for `V(D) [VG=1; point 1]`: every member is
/// read against the column of its own point, not the one chosen.
pub(crate) fn abscissa_label(abscissa: &Abscissa) -> &str {
    match abscissa {
        Abscissa::Signal { name } => member_base(name),
        Abscissa::Expression { text } => text,
    }
}

/// Whether the curve `curve` would be read against itself when its pane is
/// drawn against the signal `abscissa`: that is the diagonal, not a curve,
/// so the pane neither draws nor lists it.
pub(crate) fn is_own_abscissa(curve: &str, abscissa: &str) -> bool {
    curve == abscissa || member_abscissa_name(curve, abscissa).is_some_and(|name| name == curve)
}

/// The abscissa of a signal: that signal's series in each member of the
/// strip that plots it.
pub(crate) fn signal_abscissa(model: &StripModel, name: &str) -> Option<ResolvedAbscissa> {
    let traces: Vec<&StripTrace> = model
        .traces
        .iter()
        .filter(|trace| {
            trace.source_waveform_name == name
                && matches!(
                    trace.kind,
                    TraceKind::Value | TraceKind::Real | TraceKind::Imaginary
                )
        })
        .collect();
    let first = traces.first()?;
    Some(ResolvedAbscissa {
        unit: model.trace_unit(first).to_owned(),
        series: traces
            .iter()
            .map(|trace| AbscissaSeries {
                member: MemberKey::of(trace),
                grid: Arc::clone(&trace.x),
                values: Arc::clone(&trace.y),
                cache_key: trace_key(model, trace),
            })
            .collect(),
    })
}

/// Overlays and expressions keep the waveform panes' 1.4 : 1.8 stroke.
const SECONDARY_RATIO: f32 = 1.4 / 1.8;

/// One curve as the pane draws it.
struct Curve {
    /// The strip trace the curve draws, which places it in its family; an
    /// expression has none.
    trace: Option<usize>,
    name: String,
    color: egui::Color32,
    palette_slot: Option<usize>,
    family_style: Option<rspice_results::family_projection::FamilyTraceStyle>,
    overlay: bool,
    width_ratio: f32,
    data: Arc<XyCurve>,
    range: Range<usize>,
    cache_key: u64,
}

impl Curve {
    fn sweep(&self) -> &[f64] {
        &self.data.sweep[self.range.clone()]
    }
    fn x(&self) -> &[f64] {
        &self.data.x[self.range.clone()]
    }
    fn y(&self) -> &[f64] {
        &self.data.y[self.range.clone()]
    }
}

fn finite_bounds<'a>(values: impl Iterator<Item = &'a f64>, positive: bool) -> Option<(f64, f64)> {
    let (mut low, mut high) = (f64::INFINITY, f64::NEG_INFINITY);
    for &value in values {
        if value.is_finite() && (!positive || value > 0.0) {
            low = low.min(value);
            high = high.max(value);
        }
    }
    if !high.is_finite() {
        return None;
    }
    if low == high {
        let pad = if low == 0.0 { 1.0 } else { low.abs() * 0.05 };
        return Some((low - pad, high + pad));
    }
    let pad = (high - low) * 0.04;
    Some((low - pad, high + pad))
}

/// The point of `curve` at sweep value `t`, read by `interpolation`.
fn point_at(curve: &Curve, t: f64, interpolation: SampleInterpolation) -> Option<(f64, f64)> {
    let sweep = curve.sweep();
    let (first, last) = (*sweep.first()?, *sweep.last()?);
    if curve.data.ascending {
        if t < first || t > last {
            return None;
        }
        return Some((
            sample_at_with(sweep, curve.x(), t, interpolation),
            sample_at_with(sweep, curve.y(), t, interpolation),
        ));
    }
    // A sweep that turns back reads at the sample nearest the cursor.
    let index =
        (0..sweep.len()).min_by(|a, b| (sweep[*a] - t).abs().total_cmp(&(sweep[*b] - t).abs()))?;
    Some((curve.x()[index], curve.y()[index]))
}

/// Significant digits of a coordinate on a logarithmic axis, whose
/// resolution is relative to the value.
const LOG_AXIS_DIGITS: usize = 4;

/// `value` read on an axis spanning `span`, stated to that axis' resolution:
/// a thousandth of its span, rounded down to a decade, is the last digit
/// shown, and a value finer than that reads as zero in the axis unit. Half a
/// period of a sine on a 2 V axis is `0 V`, not the `122 aV` its last bit
/// holds. A logarithmic axis resolves a share of the value instead.
fn at_resolution(value: f64, (low, high): (f64, f64), log: bool, unit: &str) -> String {
    let span = (high - low).abs();
    if log || !value.is_finite() || !(span.is_finite() && span > 0.0) {
        return super::fmt_in_unit_readout(value, unit, LOG_AXIS_DIGITS);
    }
    let resolution = 10f64.powf(span.log10().floor() - 3.0);
    let snapped = (value / resolution).round() * resolution;
    if snapped == 0.0 {
        return super::fmt_in_unit_readout(0.0, unit, 1);
    }
    let digits = snapped.abs().log10().floor() - resolution.log10().round() + 1.0;
    super::fmt_in_unit_readout(snapped, unit, digits.max(1.0) as usize)
}

/// The stroke of each member of a crowded family: the thinnest that draws.
const HAIRLINE_WIDTH: f32 = 1.0;

/// The alpha of each hairline of a crowded family of `members`. It falls
/// inversely with the count, so where a share `s` of the members cross the
/// ink is `1 - (1 - a)^(s n)`, about `1 - e^(-32 s)` whatever the family's
/// size: density reads the same for a hundred trials as for a thousand, as
/// a band would. Half opaque just past the band threshold, and never below
/// the faintest step an 8-bit display draws a few times over.
fn crowded_member_alpha(members: usize) -> f32 {
    let threshold = super::pane::FAMILY_BAND_MEMBERS as f32;
    (0.5 * threshold / members.max(1) as f32).clamp(1.0 / 64.0, 0.5)
}

/// The side of the square a cursor point can be taken hold of in.
const CURSOR_GRIP_SIDE: f32 = 14.0;

/// Width of each end's text box in the sweep span field: nine characters
/// at the field size, `1.2346 ms`, inside the box's margins.
const SWEEP_RANGE_END_WIDTH: f32 = 80.0;

/// Space between the parts of the sweep span field.
const SWEEP_RANGE_GAP: f32 = 4.0;

/// The toggle after the span field that has it follow the analysis' strip.
const FOLLOW: &str = "follow";

/// The sweep the span field names: its axis' label, else "sweep".
fn sweep_name(model: &StripModel) -> &str {
    if model.x_label().is_empty() {
        "sweep"
    } else {
        model.x_label()
    }
}

/// The width a strip's header leaves for its "t <from> – <to>" field and the
/// "follow" toggle after it: the sweep's name and the dash at the field size,
/// both ends' boxes, and the toggle as the kit's chip sizes it.
pub(crate) fn sweep_range_field_width(ui: &Ui, model: &StripModel) -> f32 {
    let measure = |text: &str, font: egui::FontId| {
        ui.fonts_mut(|fonts| {
            fonts
                .layout_no_wrap(text.to_owned(), font, egui::Color32::WHITE)
                .size()
                .x
        })
    };
    let field = rspice_ui_kit::theme::field_mono();
    let words = measure(sweep_name(model), field.clone()) + measure("\u{2013}", field);
    let label = measure(
        FOLLOW,
        rspice_ui_kit::theme::mono(
            rspice_ui_kit::tokens::FS_0,
            rspice_ui_kit::theme::FontWeight::Regular,
        ),
    );
    let tokens = Tokens::get(ui.ctx());
    let toggle = (label + 18.0).max(if tokens.metrics.is_touch() {
        tokens.metrics.touch_target()
    } else {
        0.0
    });
    words + 2.0 * SWEEP_RANGE_END_WIDTH + toggle + 4.0 * SWEEP_RANGE_GAP
}

/// `text` read as a sweep value: a number with an optional SPICE scale and
/// unit, as a deck writes it (`250u`, `1.5ms`), spaces and a micro sign
/// allowed. Empty text is no value.
fn parse_sweep_value(text: &str) -> Option<Result<f64, String>> {
    let compact: String = text
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| {
            if matches!(c, '\u{b5}' | '\u{3bc}') {
                'u'
            } else {
                c
            }
        })
        .collect();
    (!compact.is_empty()).then(|| rspice_results::spice_value::parse_spice_value_checked(&compact))
}

/// The span `(from, to)` typed into the field, held within the record
/// `domain`: an empty end is the record's end, and a span that is the whole
/// record is none. Refused, with the reason, when an end does not read as a
/// number or the span is empty or backwards.
pub(crate) fn typed_sweep_range(
    (from, to): (&str, &str),
    domain: (f64, f64),
) -> Result<Option<(f64, f64)>, String> {
    let read = |text: &str, end: f64| {
        parse_sweep_value(text).map_or(Ok(end), |value| {
            value.map(|value| value.clamp(domain.0, domain.1))
        })
    };
    let (start, stop) = (read(from, domain.0)?, read(to, domain.1)?);
    if start >= stop {
        return Err("the span must start before it ends".to_owned());
    }
    Ok((start > domain.0 || stop < domain.1).then_some((start, stop)))
}

/// The span of the sweep `strip` draws, inside the record `domain`: while it
/// follows, the window its analysis' strip shows; otherwise the span the
/// reader typed; the whole record when neither narrows it.
pub(crate) fn drawn_span(
    results: &crate::session::ResultViewerState,
    strip: &crate::session::XyStrip,
    domain: (f64, f64),
    pane_count: usize,
) -> (f64, f64) {
    let span = if strip.follow_view {
        super::stack::shared_x_view(results, strip.analysis, pane_count)
    } else {
        strip.sweep_range
    };
    span.map(|(start, stop)| (start.max(domain.0), stop.min(domain.1)))
        .filter(|(start, stop)| start < stop)
        .unwrap_or(domain)
}

/// What the reader did in a strip header's span field.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum SpanEdit {
    /// Typed a span (`None` for the whole record), which stops following.
    Typed(Option<(f64, f64)>),
    /// Turned following the analysis' strip on or off.
    Follow(bool),
}

/// The strip header's "t <from> – <to>" field and its "follow" toggle: the
/// span of the sweep the strip draws, the whole record by default, or while
/// following, the window its analysis' strip shows. An end applies when the
/// reader presses Enter or leaves it; one that does not read reverts.
pub(crate) fn sweep_range_field(
    ui: &mut Ui,
    rect: egui::Rect,
    model: &StripModel,
    (strip, span): (&crate::session::XyStrip, (f64, f64)),
    (identity, significant_digits, quantity): (
        u64,
        usize,
        rspice_vocabulary::quantity::QuantityPresentationPolicy,
    ),
) -> Option<SpanEdit> {
    let domain = model.x_range?;
    let (start, stop) = span;
    let sweep = sweep_name(model);
    let t = Tokens::get(ui.ctx());
    // The ends and the words beside them read at the field size, as every
    // input does.
    let font = rspice_ui_kit::theme::field_mono();
    let mut changed = None;
    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
        |ui| {
            ui.set_clip_rect(rect);
            ui.spacing_mut().item_spacing.x = SWEEP_RANGE_GAP;
            ui.label(
                egui::RichText::new(sweep)
                    .font(font.clone())
                    .color(t.color.text_dim),
            );
            let mut ends = [String::new(), String::new()];
            let mut typed = false;
            let mut committed = false;
            for (end, value) in [(0usize, start), (1, stop)] {
                if end == 1 {
                    ui.label(
                        egui::RichText::new("\u{2013}")
                            .font(font.clone())
                            .color(t.color.text_dim),
                    );
                }
                let id = egui::Id::new(("xy-sweep-range", identity, end));
                let shown = model.format_x(value, significant_digits, quantity);
                let mut text = ui
                    .data_mut(|data| data.get_temp::<String>(id))
                    .unwrap_or_else(|| shown.clone());
                let response = ui
                    .add(
                        egui::TextEdit::singleline(&mut text)
                            .id(id.with("edit"))
                            .font(rspice_ui_kit::theme::field_mono())
                            .desired_width(SWEEP_RANGE_END_WIDTH),
                    )
                    .on_hover_text(format!(
                        "The span of {sweep} the curves are drawn over. Enter applies; an \
                         empty end is the record's end. Typing a span stops following."
                    ));
                if response.lost_focus() {
                    committed = true;
                    ui.data_mut(|data| data.remove::<String>(id));
                } else if response.has_focus() {
                    ui.data_mut(|data| data.insert_temp(id, text.clone()));
                } else {
                    ui.data_mut(|data| data.remove::<String>(id));
                    text = shown.clone();
                }
                typed |= text != shown;
                ends[end] = text;
            }
            if committed
                && typed
                && let Ok(range) = typed_sweep_range((&ends[0], &ends[1]), domain)
                && (strip.follow_view || range != strip.sweep_range)
            {
                changed = Some(SpanEdit::Typed(range));
            }
            if chip(ui, FOLLOW, strip.follow_view)
                .on_hover_text(format!(
                    "Follow the {sweep} window shown above: zooming that strip chooses the span \
                     drawn here."
                ))
                .clicked()
            {
                changed = Some(SpanEdit::Follow(!strip.follow_view));
            }
        },
    );
    changed
}

/// What a strip drawn against an abscissa needs from the stack.
pub(crate) struct XyInput<'a> {
    /// The unit pane whose curves the strip plots.
    pub pane: &'a UnitPane<'a>,
    pub exprs: &'a [ResolvedExpr],
    pub frame: StripFrame<'a>,
    /// The strip, by its place among the session's strips.
    pub strip: usize,
    pub log_y: bool,
}

/// Draw one strip's curves against its abscissa.
pub(crate) fn show(ui: &mut Ui, host: &mut impl StackHost, input: XyInput<'_>) {
    let XyInput {
        pane,
        exprs,
        frame,
        strip: strip_index,
        log_y,
    } = input;
    let Some(strip) = host.session().xy_strips.get(strip_index).cloned() else {
        return;
    };
    let abscissa = &strip.abscissa;
    let model = frame.model;
    let t = Tokens::get(ui.ctx());
    let interpolation = super::cursor_interpolation(frame.options.readout.cursor_interpolation());
    let significant_digits =
        usize::from(frame.options.readout.displayed_significant_digits().get());
    let resolved = match abscissa {
        Abscissa::Signal { name } => signal_abscissa(model, name)
            .ok_or_else(|| format!("{name} is not a signal of this analysis")),
        Abscissa::Expression { text } => host.resolve_abscissa_expression(model, text),
    };
    let resolved = match resolved {
        Ok(resolved) => resolved,
        Err(reason) => {
            well_hint(ui, &format!("Cannot draw against it: {reason}"));
            return;
        }
    };
    let operand = |member: MemberKey| {
        let find = |matches: &dyn Fn(MemberKey) -> bool| {
            resolved.series.iter().find(|series| matches(series.member))
        };
        find(&|held| held == member)
            .or_else(|| {
                // An abscissa outside any family pairs with every member of
                // its run.
                find(&|held| {
                    held.presentation_key == 0
                        && held.run_id == member.run_id
                        && held.overlay == member.overlay
                })
            })
            .or_else(|| {
                // An expression is evaluated on the active run alone.
                find(&|held| {
                    !member.overlay
                        && !held.overlay
                        && held.presentation_key == member.presentation_key
                })
            })
    };
    // The strip draws the span of the sweep its header names: the whole
    // record unless the reader narrowed it, or while it follows, the window
    // its analysis' strip shows.
    let window_range = drawn_span(host.session(), &strip, frame.x_domain, frame.pane_count);
    let mut refusals: Vec<String> = Vec::new();
    let mut curves: Vec<Curve> = Vec::new();
    let trace_width = frame.options.trace_width;
    let active_run = model
        .traces
        .iter()
        .find(|trace| !trace.overlay)
        .map_or(0, |trace| trace.run_id);
    let derived = &mut host.session_mut().derived;
    let mut pair = |name: &str,
                    key: u64,
                    series: Option<&AbscissaSeries>,
                    (grid, values): (&SharedWaveformValues, &SharedWaveformValues)|
     -> Option<(Arc<XyCurve>, u64)> {
        let Some(series) = series else {
            refusals.push(format!("{name} has no {} to pair with", abscissa.text()));
            return None;
        };
        let memo = stable_hash(&("xy", key, series.cache_key, interpolation as u8));
        match derived.xy_curve_or(memo, || {
            resample(
                (series.grid.as_slice(), series.values.as_slice()),
                (grid.as_slice(), values.as_slice()),
                interpolation,
            )
        }) {
            Ok(curve) => Some((curve, memo)),
            Err(refusal) => {
                refusals.push(refusal_text(abscissa, name, refusal));
                None
            }
        }
    };
    for &index in &pane.traces {
        let Some(trace) = model.traces.get(index) else {
            continue;
        };
        if let Abscissa::Signal { name } = abscissa
            && is_own_abscissa(&trace.source_waveform_name, name)
        {
            continue;
        }
        // A nested sweep names each member's columns with its point,
        // `I(VD) [VG=2; point 2]`: that curve reads against the abscissa's
        // column of the same point.
        let sibling = match abscissa {
            Abscissa::Signal { name } => member_abscissa_name(&trace.source_waveform_name, name)
                .and_then(|name| signal_abscissa(model, &name)),
            Abscissa::Expression { .. } => None,
        };
        let Some((data, cache_key)) = pair(
            &trace.name,
            trace_key(model, trace),
            sibling
                .as_ref()
                .and_then(|sibling| sibling.series.first())
                .or_else(|| operand(MemberKey::of(trace))),
            (&trace.x, &trace.y),
        ) else {
            continue;
        };
        let range = window(&data, window_range);
        curves.push(Curve {
            trace: Some(index),
            name: trace.name.clone(),
            color: trace.color,
            palette_slot: trace
                .palette_slot
                .filter(|_| trace.family_style.is_none_or(|style| style.color.is_none())),
            family_style: trace.family_style,
            overlay: trace.overlay,
            width_ratio: if trace.overlay { SECONDARY_RATIO } else { 1.0 },
            cache_key,
            data,
            range,
        });
    }
    for expr in exprs {
        let member = MemberKey {
            run_id: active_run,
            overlay: false,
            presentation_key: expr.presentation_key,
        };
        let Some((data, cache_key)) = pair(
            &expr.label,
            expr.cache_key,
            operand(member),
            (&expr.x, &expr.y),
        ) else {
            continue;
        };
        let range = window(&data, window_range);
        curves.push(Curve {
            trace: None,
            name: expr.label.clone(),
            color: expr.color,
            palette_slot: None,
            family_style: expr.family_style,
            overlay: false,
            width_ratio: SECONDARY_RATIO,
            cache_key,
            data,
            range,
        });
    }
    if curves.is_empty() {
        let reason = refusals
            .first()
            .cloned()
            .unwrap_or_else(|| "No visible traces — enable one in the legend".to_owned());
        well_hint(ui, &reason);
        return;
    }

    let x_view = strip.x_view;
    let y_view = strip
        .y_view
        .filter(|(low, high)| !log_y || (*low > 0.0 && *high > 0.0));
    let auto_x = finite_bounds(curves.iter().flat_map(|curve| curve.x()), false);
    let auto_y = finite_bounds(curves.iter().flat_map(|curve| curve.y()), log_y);
    let (Some((x0, x1)), Some((y0, y1))) = (x_view.or(auto_x), y_view.or(auto_y)) else {
        well_hint(ui, "No point of these curves lies in the window");
        return;
    };
    let interpolated = curves.iter().any(|curve| curve.data.interpolated);
    let x_label = format!(
        "{} \u{b7} {}",
        abscissa_label(abscissa),
        pairing_note(model.x_label(), interpolated, interpolation)
    );
    let x_axis = Axis::linear(x0, x1, &resolved.unit).with_label(&x_label);
    let y_axis = if log_y {
        Axis::log_decades(y0, y1, pane.axis_unit())
    } else {
        Axis::linear(y0, y1, pane.axis_unit())
    };
    let y_scale = if log_y { XScale::Log10 } else { XScale::Linear };
    // Arrowheads and the start and end marks go only on a curve whose X turns
    // back; a monotonic sweep reads its way off the axis.
    let turn_back_tolerance =
        auto_x.map_or(f64::INFINITY, |(low, high)| (high - low) * TURN_BACK_SHARE);
    let turning: Vec<bool> = curves
        .iter()
        .map(|curve| turns_back(curve.x(), turn_back_tolerance))
        .collect();
    let detail = format!(
        "{} drawn against {}{}{}",
        curves
            .iter()
            .map(|curve| curve.name.as_str())
            .collect::<Vec<_>>()
            .join(", "),
        abscissa_label(abscissa),
        if !turning.contains(&true) {
            String::new()
        } else if model.x_label().is_empty() {
            "; arrows mark the way the sweep advances".to_owned()
        } else {
            format!("; arrows mark the way {} advances", model.x_label())
        },
        refusals
            .iter()
            .map(|refusal| format!(". {refusal}"))
            .collect::<String>()
    );
    let mut spec = PlotSpec::new(x_axis, XScale::Linear, y_axis)
        .accessible_name("Signal versus signal plot")
        .accessible_detail(&detail)
        .with_right_margin(super::navigation::WAVE_SHARED_RIGHT_MARGIN);
    spec.left_margin = frame.gutter.width();
    if log_y {
        spec = spec.with_log_y();
    }
    spec.minor_grid = host.session().show_minor_grid;
    // The hover readout reads the point nearest the pointer, on screen.
    let plot_rect = plot::plot_rect(ui, &spec);
    let to_screen = |x: f64, y: f64| {
        egui::pos2(
            plot_rect.left() + (XScale::Linear.normalize(x, x0, x1) as f32) * plot_rect.width(),
            plot_rect.bottom() - (y_scale.normalize(y, y0, y1) as f32) * plot_rect.height(),
        )
    };
    let pointer = ui
        .input(|input| input.pointer.hover_pos())
        .filter(|position| plot_rect.contains(*position) && ui.rect_contains_pointer(plot_rect));
    let nearest = pointer.and_then(|position| {
        let mut best: Option<(usize, usize, f32)> = None;
        for (index, curve) in curves.iter().enumerate() {
            let stride = (curve.range.len() / 20_000).max(1);
            for sample in (0..curve.range.len()).step_by(stride) {
                let distance = to_screen(curve.x()[sample], curve.y()[sample]).distance(position);
                if distance.is_finite() && best.is_none_or(|(.., held)| distance < held) {
                    best = Some((index, sample, distance));
                }
            }
        }
        best.filter(|(.., distance)| *distance <= 24.0)
            .map(|(curve, sample, _)| (curve, sample))
    });
    // A family too large to read member by member is not folded into a band
    // here: a band filled per column of X fills a loop's interior, and X is
    // no function axis on this plot. Each member is a hairline instead, its
    // alpha falling as the family grows so that where members crowd the ink
    // builds up as a band would; the member nearest the pointer is drawn
    // over them at full weight, with its arrows and cursor points.
    let family_members: Vec<(usize, &StripTrace)> = curves
        .iter()
        .enumerate()
        .filter_map(|(index, curve)| Some((index, model.traces.get(curve.trace?)?)))
        .collect();
    let mut crowded: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    for family in super::pane::family_bands(&family_members) {
        let size = family.members.len();
        crowded.extend(family.members.iter().map(|(index, _)| (*index, size)));
    }
    let highlighted = nearest
        .map(|(index, _)| index)
        .filter(|index| !curves[*index].overlay);
    let faint = |index: usize| crowded.contains_key(&index) && highlighted != Some(index);
    let stroked = |index: usize| !faint(index);
    let order: Vec<usize> = (0..curves.len())
        .filter(|index| faint(*index))
        .chain((0..curves.len()).filter(|index| !faint(*index) && curves[*index].overlay))
        .chain((0..curves.len()).filter(|index| !faint(*index) && !curves[*index].overlay))
        .collect();
    let shade = |index: usize| {
        let curve = &curves[index];
        if curve.overlay {
            curve.color.gamma_multiply(0.40)
        } else {
            curve.color
        }
    };
    // The crowded members are painted under the curves as plain hairlines:
    // no dash, no symbol and no style weight of their own, which would only
    // add ink. Points closer than half a pixel to the last one drawn add
    // nothing either, so a long member costs what the screen can show.
    let hairlines: Vec<(&[f64], &[f64], egui::Color32)> = order
        .iter()
        .copied()
        .filter(|index| faint(*index))
        .filter_map(|index| {
            let members = *crowded.get(&index)?;
            let curve = &curves[index];
            Some((
                curve.x(),
                curve.y(),
                shade(index).gamma_multiply(crowded_member_alpha(members)),
            ))
        })
        .collect();
    // The member the reader points at in a crowd is drawn over a margin of
    // the canvas, so its line stands out of a crowd of its own hue.
    let halo = highlighted
        .filter(|index| crowded.contains_key(index))
        .map(|index| (curves[index].x(), curves[index].y()));
    let halo_stroke = egui::Stroke::new(trace_width + 3.0, t.color.canvas_bg);
    if !hairlines.is_empty() {
        spec.underlay = Some(Box::new(move |painter, mapper| {
            let rect = mapper.rect;
            let at = |x: f64, y: f64| {
                egui::pos2(
                    rect.left() + (XScale::Linear.normalize(x, x0, x1) as f32) * rect.width(),
                    rect.bottom() - (y_scale.normalize(y, y0, y1) as f32) * rect.height(),
                )
            };
            for &(xs, ys, color) in &hairlines {
                let stroke = egui::Stroke::new(HAIRLINE_WIDTH, color);
                let mut run: Vec<egui::Pos2> = Vec::new();
                for (&x, &y) in xs.iter().zip(ys) {
                    let point = at(x, y);
                    if !point.is_finite() {
                        if run.len() >= 2 {
                            painter.add(egui::Shape::line(std::mem::take(&mut run), stroke));
                        }
                        run.clear();
                        continue;
                    }
                    if run
                        .last()
                        .is_none_or(|last| last.distance_sq(point) >= 0.25)
                    {
                        run.push(point);
                    }
                }
                if run.len() >= 2 {
                    painter.add(egui::Shape::line(run, stroke));
                }
            }
            if let Some((xs, ys)) = halo {
                let points: Vec<egui::Pos2> = xs
                    .iter()
                    .zip(ys)
                    .map(|(&x, &y)| at(x, y))
                    .filter(|point| point.is_finite())
                    .collect();
                painter.add(egui::Shape::line(points, halo_stroke));
            }
        }));
    }
    for index in order.iter().copied().filter(|index| !faint(*index)) {
        let curve = &curves[index];
        let window_key = stable_hash(&(curve.cache_key, curve.range.start, curve.range.end));
        let styled = super::pane::apply_family_trace_style(
            Trace::new(curve.x(), curve.y(), shade(index))
                .width(trace_width * curve.width_ratio)
                .cache_key(window_key)
                .parametric(),
            curve.family_style,
        );
        spec.traces.push(match curve.palette_slot {
            Some(slot) => styled.palette_slot(slot),
            None => styled,
        });
    }

    // The strip's cursors stand on the sweep; here each is the point every
    // curve has reached at that instant, the first curve's tagged with the
    // instant and both coordinates.
    let cursors = (host.session().cursor_strip == Some(model.analysis_index)
        || (host.session().linked_cursors
            && frame.linked_cursor_domain == Some(&model.cursor_domain())))
    .then_some(host.session().cursors);
    let quantity = frame.options.quantity;
    // Both coordinates, in every tag and in the hover readout, are stated to
    // the resolution of the axis they are read on.
    let format_x = |value: f64| at_resolution(value, (x0, x1), false, &resolved.unit);
    let format_y = |value: f64| at_resolution(value, (y0, y1), log_y, pane.axis_unit());
    let sweep_name = if model.x_label().is_empty() {
        "sweep"
    } else {
        model.x_label()
    };
    // Where each cursor point stands on screen, by cursor and curve, so the
    // reader can take hold of it.
    let mut grips: Vec<(bool, usize, egui::Pos2)> = Vec::new();
    for (label, position, color) in [
        (
            "A",
            cursors.and_then(|pair| pair.a),
            t.color.canvas_cursor_a,
        ),
        (
            "B",
            cursors.and_then(|pair| pair.b),
            t.color.canvas_cursor_b,
        ),
    ] {
        let Some(at) = position else { continue };
        let mut tagged = false;
        for (index, curve) in curves
            .iter()
            .enumerate()
            .filter(|(index, _)| stroked(*index))
        {
            let Some((x, y)) = point_at(curve, at, interpolation) else {
                continue;
            };
            grips.push((label == "A", index, to_screen(x, y)));
            let tag = if tagged {
                label.to_owned()
            } else {
                tagged = true;
                format!(
                    "{label} \u{b7} {sweep_name} {} \u{b7} {} {} \u{b7} {}",
                    model.format_x(at, significant_digits, quantity),
                    abscissa_label(abscissa),
                    format_x(x),
                    format_y(y)
                )
            };
            spec.markers.push(plot::Marker::point(x, y, color, tag));
        }
    }

    let readout_rows: Vec<(String, String)> = nearest
        .map(|(index, sample)| {
            let curve = &curves[index];
            vec![
                (
                    sweep_name.to_owned(),
                    model.format_x(curve.sweep()[sample], significant_digits, quantity),
                ),
                (
                    abscissa_label(abscissa).to_owned(),
                    format_x(curve.x()[sample]),
                ),
                (curve.name.clone(), format_y(curve.y()[sample])),
            ]
        })
        .unwrap_or_default();
    let readout = |_: f64| readout_rows.clone();
    let response = plot::show(
        ui,
        &spec,
        &mut host.session_mut().cache,
        None,
        Some(&readout),
    );

    // The X axis names what the strip is drawn against, and clicking that
    // name chooses again, as ViVA's and EZwave's axis menus do.
    let mut chosen = None;
    if let Some(label_rect) = response.x_label_rect {
        let label = ui
            .interact(
                label_rect.expand(2.0),
                ui.id().with(("xy-x-label", model.analysis_key, pane.unit)),
                egui::Sense::click(),
            )
            .on_hover_cursor(egui::CursorIcon::PointingHand)
            .on_hover_text(format!(
                "Choose what the {} curves are plotted against",
                pane.unit
            ));
        if label.hovered()
            || egui::Popup::is_id_open(ui.ctx(), egui::Popup::default_response_id(&label))
        {
            ui.painter().hline(
                label_rect.x_range(),
                label_rect.bottom(),
                egui::Stroke::new(1.0, t.color.canvas_text_dim),
            );
        }
        let menu = super::stack::abscissa_menu(host.session(), model, pane, Some(abscissa.clone()));
        let draft_id = label.id.with("draft");
        chosen = egui::Popup::menu(&label)
            .show(|ui| {
                super::header::abscissa_choices(ui, pane.unit, &menu, &str::to_owned, draft_id)
            })
            .and_then(|inner| inner.inner);
    }

    // A cursor point is taken hold of and dragged along its curve: the
    // cursor moves to the instant of the sample nearest the pointer, so the
    // strips drawn against the sweep follow it, as moving the cursor there
    // moves the point here.
    let mut dragged: Option<(bool, f64)> = None;
    for &(is_a, index, at) in &grips {
        if !response.plot_rect.contains(at) {
            continue;
        }
        let grip = ui
            .interact(
                egui::Rect::from_center_size(at, egui::Vec2::splat(CURSOR_GRIP_SIDE)),
                ui.id().with(("xy-cursor-point", is_a, index)),
                egui::Sense::click_and_drag(),
            )
            .on_hover_cursor(egui::CursorIcon::Grab);
        if grip.dragged()
            && let Some(pointer) = grip.interact_pointer_pos()
        {
            let curve = &curves[index];
            let nearest = (0..curve.range.len())
                .map(|sample| (sample, to_screen(curve.x()[sample], curve.y()[sample])))
                .filter(|(_, point)| point.is_finite())
                .min_by(|(_, a), (_, b)| a.distance(pointer).total_cmp(&b.distance(pointer)));
            if let Some((sample, _)) = nearest {
                dragged = Some((is_a, curve.sweep()[sample]));
            }
        }
    }

    // Sparse arrowheads along each curve that turns back, and its start
    // filled and its end hollow, as the polar locus marks its sweep.
    let painter = ui.painter().with_clip_rect(response.plot_rect);
    for curve in order
        .iter()
        .filter(|index| stroked(**index) && turning[**index])
        .map(|index| &curves[*index])
    {
        let color = if curve.overlay {
            curve.color.gamma_multiply(0.40)
        } else {
            curve.color
        };
        let stride = (curve.range.len() / 4_096).max(1);
        let points: Vec<egui::Pos2> = (0..curve.range.len())
            .step_by(stride)
            .chain((curve.range.len() > 1).then(|| curve.range.len() - 1))
            .map(|sample| to_screen(curve.x()[sample], curve.y()[sample]))
            .collect();
        for (at, forward) in direction_arrows(&points) {
            paint_arrow(&painter, at, forward, color);
        }
        if let Some(start) = points.iter().find(|point| point.is_finite()) {
            painter.circle_filled(*start, 3.0, color);
        }
        if let Some(stop) = points.iter().rev().find(|point| point.is_finite()) {
            painter.circle_stroke(*stop, 3.5, egui::Stroke::new(1.4, color));
        }
    }

    // A click places the next cursor at the instant of the nearest point.
    if dragged.is_none()
        && response.clicked_x.is_some()
        && host.session().cursor_placement_enabled()
        && !host.session().marker_tool.is_armed()
        && !host.session().trace_pick.is_armed()
        && let Some((index, sample)) = nearest
    {
        let at = curves[index].sweep()[sample];
        let results = host.session_mut();
        if results.cursor_strip != Some(model.analysis_index) {
            results.cursors = plot::CursorPair::default();
        }
        results.cursor_strip = Some(model.analysis_index);
        results.cursors.place(at);
    }
    let session = host.session_mut();
    match dragged {
        Some((true, at)) => {
            session.cursors.a = Some(at);
            session.cursor_a_anchor = None;
        }
        Some((false, at)) => session.cursors.b = Some(at),
        None => {}
    }
    if let Some(choice) = chosen {
        super::stack::repoint_xy_strip(session, strip_index, choice);
        return;
    }
    let Some(held) = session.xy_strips.get_mut(strip_index) else {
        return;
    };
    if response.view.reset {
        held.x_view = None;
        held.y_view = None;
    } else if response.view.any() {
        if let Some(x) = response.view.x {
            held.x_view = Some(x);
        }
        if let Some(y) = response.view.y {
            held.y_view = Some(y);
        }
    }
}

#[cfg(test)]
mod tests;
