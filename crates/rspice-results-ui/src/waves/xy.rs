//! A unit pane drawn against a signal or an expression instead of its
//! strip's sweep: a transfer curve, a hysteresis loop, a Lissajous figure.
//!
//! Each curve pairs its own values with the abscissa's on one grid, read
//! through the strip's window on the sweep: zooming the time axis below the
//! stack chooses which stretch of the loop is drawn. Sparse arrowheads say
//! which way the sweep travels, and the strip's cursors, which stand on the
//! sweep, are drawn where the curves are at that instant.

use super::pane::ResolvedExpr;
use super::stack::{StackHost, StripFrame, shared_x_view};
use super::{StripModel, StripTrace, TraceKind, UnitPane, stable_hash, trace_key};
use crate::presentation::well_hint;
use egui::Ui;
use rspice_results::result_presentation::{ResultViewer, WavePanePresentationKey};
use rspice_results::visualization_document::Abscissa;
use rspice_results::waveform::SharedWaveformValues;
use rspice_ui_kit::plot::sample::sample_at_with;
use rspice_ui_kit::plot::{self, Axis, PlotSpec, SampleInterpolation, Trace, XScale};
use rspice_ui_kit::tokens::Tokens;
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

/// What a pane drawn against an abscissa needs from the stack.
pub(crate) struct XyInput<'a> {
    pub pane: &'a UnitPane<'a>,
    pub ordinal: usize,
    pub exprs: &'a [ResolvedExpr],
    pub frame: StripFrame<'a>,
    pub abscissa: &'a Abscissa,
    pub log_y: bool,
}

/// Draw one unit pane against its abscissa.
pub(crate) fn show(ui: &mut Ui, host: &mut impl StackHost, input: XyInput<'_>) {
    let XyInput {
        pane,
        ordinal,
        exprs,
        frame,
        abscissa,
        log_y,
    } = input;
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
    let window_range = shared_x_view(host.session(), model.analysis_key, frame.pane_count)
        .unwrap_or(frame.x_domain);
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

    let pane_key = WavePanePresentationKey {
        analysis: model.analysis_key,
        unit: pane.unit.to_owned(),
    };
    let x_view = host
        .session()
        .unit_abscissas
        .get(&pane_key)
        .and_then(|choice| choice.x_view);
    let y_view = host
        .session()
        .analysis_plot_view_pane(ResultViewer::Waves, model.analysis_key, ordinal)
        .y
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
        abscissa.text(),
        pairing_note(model.x_label(), interpolated, interpolation)
    );
    let x_axis = Axis::linear(x0, x1, &resolved.unit).with_label(&x_label);
    let y_axis = if log_y {
        Axis::log_decades(y0, y1, pane.axis_unit())
    } else {
        Axis::linear(y0, y1, pane.axis_unit())
    };
    let y_scale = if log_y { XScale::Log10 } else { XScale::Linear };
    let detail = format!(
        "{} drawn against {}; arrows mark the way {} advances{}",
        curves
            .iter()
            .map(|curve| curve.name.as_str())
            .collect::<Vec<_>>()
            .join(", "),
        abscissa.text(),
        if model.x_label().is_empty() {
            "the sweep"
        } else {
            model.x_label()
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
    // A family too large to stroke member by member is drawn as the band it
    // occupies, with the member the reader points at stroked over it, as the
    // panes drawn against the sweep draw it.
    let family_members: Vec<(usize, &StripTrace)> = curves
        .iter()
        .enumerate()
        .filter_map(|(index, curve)| Some((index, model.traces.get(curve.trace?)?)))
        .collect();
    let families = super::pane::family_bands(&family_members);
    let mut banded = std::collections::HashSet::new();
    if !families.is_empty() {
        let view = plot::TraceView {
            x0,
            x1,
            y0,
            y1,
            x_scale: XScale::Linear,
            y_scale,
            columns: (plot_rect.width().ceil().max(0.0) as usize).next_multiple_of(64),
            rows: 0,
        };
        let mut bands = Vec::with_capacity(families.len());
        for family in &families {
            let identity = stable_hash(
                &family
                    .members
                    .iter()
                    .map(|(index, _)| {
                        let curve = &curves[*index];
                        (curve.cache_key, curve.range.start, curve.range.end)
                    })
                    .collect::<Vec<_>>(),
            );
            let members: Vec<plot::BandMember<'_>> = family
                .members
                .iter()
                .map(|(index, _)| plot::BandMember {
                    x: curves[*index].x(),
                    y: curves[*index].y(),
                    parametric: true,
                    shape: None,
                })
                .collect();
            let color = family.members[0].1.signal_color;
            bands.push((
                host.session_mut().cache.band(identity, &members, view),
                if family.overlay {
                    color.gamma_multiply(0.40)
                } else {
                    color
                },
            ));
            banded.extend(family.members.iter().map(|(index, _)| *index));
        }
        let band_range = (y0, y1);
        spec.underlay = Some(Box::new(move |painter, mapper| {
            super::pane::paint_family_bands(painter, mapper.rect, &bands, y_scale, band_range);
        }));
    }
    let highlighted = nearest
        .map(|(index, _)| index)
        .filter(|index| !curves[*index].overlay);
    let stroked = |index: usize| !banded.contains(&index) || highlighted == Some(index);
    let order: Vec<usize> = (0..curves.len())
        .filter(|index| curves[*index].overlay)
        .chain((0..curves.len()).filter(|index| !curves[*index].overlay))
        .filter(|index| stroked(*index))
        .collect();
    for curve in order.iter().map(|index| &curves[*index]) {
        let color = if curve.overlay {
            curve.color.gamma_multiply(0.40)
        } else {
            curve.color
        };
        let window_key = stable_hash(&(curve.cache_key, curve.range.start, curve.range.end));
        let mut drawn = super::pane::apply_family_trace_style(
            Trace::new(curve.x(), curve.y(), color)
                .width(trace_width * curve.width_ratio)
                .cache_key(window_key)
                .parametric(),
            curve.family_style,
        );
        if let Some(slot) = curve.palette_slot {
            drawn = drawn.palette_slot(slot);
        }
        spec.traces.push(drawn);
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
        for curve in curves
            .iter()
            .enumerate()
            .filter(|(index, _)| stroked(*index))
            .map(|(_, curve)| curve)
        {
            let Some((x, y)) = point_at(curve, at, interpolation) else {
                continue;
            };
            let tag = if tagged {
                label.to_owned()
            } else {
                tagged = true;
                format!(
                    "{label} \u{b7} {sweep_name} {} \u{b7} {} {} \u{b7} {}",
                    model.format_x(at, significant_digits, quantity),
                    abscissa.text(),
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
                (abscissa.text().to_owned(), format_x(curve.x()[sample])),
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

    // The X axis names what the pane is drawn against, and clicking that
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
                "Choose what the {} pane is drawn against",
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

    // Sparse arrowheads along each curve, and its start filled and its end
    // hollow, as the polar locus marks its sweep.
    let painter = ui.painter().with_clip_rect(response.plot_rect);
    for curve in order.iter().map(|index| &curves[*index]) {
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

    let pane_view_key = WavePanePresentationKey {
        analysis: model.analysis_key,
        unit: pane.unit.to_owned(),
    };
    if response.response.hovered() || response.response.dragged() {
        host.session_mut().active_wave_pane = Some(pane_view_key.clone());
    }
    // A click places the next cursor at the instant of the nearest point.
    if response.clicked_x.is_some()
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
    if let Some(choice) = chosen {
        super::stack::choose_abscissa(session, &pane_view_key, ordinal, choice);
        return;
    }
    if response.view.reset {
        if let Some(choice) = session.unit_abscissas.get_mut(&pane_view_key) {
            choice.x_view = None;
        }
        session
            .analysis_plot_view_pane_mut(ResultViewer::Waves, model.analysis_key, ordinal)
            .y = None;
    } else if response.view.any() {
        if let Some(x) = response.view.x
            && let Some(choice) = session.unit_abscissas.get_mut(&pane_view_key)
        {
            choice.x_view = Some(x);
        }
        if let Some(y) = response.view.y {
            session
                .analysis_plot_view_pane_mut(ResultViewer::Waves, model.analysis_key, ordinal)
                .y = Some(y);
        }
    }
}

#[cfg(test)]
mod tests;
