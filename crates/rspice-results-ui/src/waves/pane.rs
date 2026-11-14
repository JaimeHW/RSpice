//! Unit-scoped waveform plot construction, retained readout and curve hit testing.

use super::navigation::{WAVE_SHARED_RIGHT_MARGIN, model_x_axis};
use super::readout::{MAX_READOUT_BRANCHES, branch_tag};
use super::{
    ReadoutPolicy, StripModel, StripTrace, UnitPane, anchor_key, cursor_interpolation, stable_hash,
    trace_key,
};
use egui::Ui;
use rspice_results::family_projection::FamilyTraceStyle;
use rspice_results::result_presentation::{MarkerKind, WaveformPresentationKey};
use rspice_results::waveform::SharedWaveformValues;
use rspice_ui_kit::palette::TRACE_HUES;
use rspice_ui_kit::plot::sample::{
    BranchSample, SweepShape, sample_at_with_shape, sample_branches_into,
};
use rspice_ui_kit::plot::{
    self, Axis, CursorPair, MAX_AXIS_TICKS, PlotSpec, SampleInterpolation, Trace, TraceLook, XScale,
};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;
use std::sync::Arc;

/// One expression trace resolved for plotting.
pub struct ResolvedExpr {
    /// The expression's place in the strip's expression list, which is how
    /// it finds the pane its unit routed it to.
    pub slot: usize,
    pub x: SharedWaveformValues,
    pub y: SharedWaveformValues,
    /// What the expression's abscissa is, on the same terms as a waveform
    /// trace's: an expression over a reverse sweep is still a reverse sweep.
    pub shape: Arc<SweepShape>,
    pub color: egui::Color32,
    pub cache_key: u64,
    pub label: String,
    pub y_extremes: Option<(f64, f64)>,
    pub family_style: Option<FamilyTraceStyle>,
    /// The family group it was evaluated on, as a trace's
    /// [`StripTrace::presentation_key`]; 0 outside a family.
    pub presentation_key: u64,
}

/// The envelope of one family of curves in a pane: its members, folded per
/// screen column when the pane draws, into the band a large family draws as.
///
/// A family's members need not share a single abscissa: two Monte Carlo
/// trials of an adaptive transient step at different instants, so an
/// envelope read only where their samples coincide is a scatter of points
/// rather than a boundary. The fold reads each member's own stroke in every
/// column instead; see [`plot::DecimationCache::band`].
#[derive(Debug)]
pub struct FamilyEnvelopeSeries {
    /// The family's members, as indices into the strip model's traces.
    pub members: Vec<usize>,
}

pub fn apply_family_trace_style<'a>(
    mut trace: Trace<'a>,
    style: Option<FamilyTraceStyle>,
) -> Trace<'a> {
    let Some(style) = style else {
        return trace;
    };
    trace = trace.show_single_point();
    if let Some(ordinal) = style.dash_ordinal {
        trace = trace.dash_style(ordinal);
    }
    if let Some(ordinal) = style.marker_ordinal {
        trace = trace.marker_style(ordinal);
    }
    if let Some(width) = style.width_points {
        trace = trace.width(width);
    }
    trace
}

/// What a swatch needs to draw a strip trace the way its pane draws it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SwatchStyle {
    pub palette_slot: Option<usize>,
    pub family_style: Option<FamilyTraceStyle>,
    pub ordinal: usize,
}

impl SwatchStyle {
    pub fn of(trace: &StripTrace) -> Self {
        Self {
            palette_slot: trace.palette_slot,
            family_style: trace.family_style,
            ordinal: trace.waveform_index,
        }
    }

    /// The curve's dash and marker, exactly as its pane resolves them, so a
    /// legend or readout swatch is the curve.
    pub fn look(&self, color_safe: bool) -> TraceLook {
        strip_trace_cues(
            Trace::new(&[], &[], egui::Color32::PLACEHOLDER),
            self.palette_slot,
            self.family_style,
            self.ordinal,
        )
        .look(self.ordinal, color_safe)
    }
}

/// A strip trace's plot curve with the cues its swatch reads: its family
/// style and a palette slot.
///
/// A curve whose colour came from its family's own policy has no slot in the
/// palette, and its colour-safe marker would otherwise come from its place in
/// the pane's draw order, which its legend chip cannot know: the curve drew
/// one symbol and its chip another. It takes its waveform's place in the hue
/// cycle instead, inside the first pass, so it draws solid when color-safe
/// traces are off.
fn strip_trace_cues(
    plot: Trace<'_>,
    palette_slot: Option<usize>,
    family_style: Option<FamilyTraceStyle>,
    waveform_index: usize,
) -> Trace<'_> {
    apply_family_trace_style(plot, family_style)
        .palette_slot(palette_slot.unwrap_or(waveform_index % TRACE_HUES))
}

/// How far apart, relative to their size, a fit's extremes may be and still
/// be one level: a node held steady differs from itself only in the rounding
/// of its solution, far below any tolerance a solver converges to.
const FLAT_FIT_RELATIVE_SPAN: f64 = 1.0e-9;

/// The Y interval a pane fits itself to when the reader has not pinned one.
///
/// Everything the pane draws widens it: the traces, whatever expressions the
/// strip carries, and the specification limits — a bound drawn off the top of
/// the axis is a bound the reader cannot check against.
pub fn pane_auto_y(
    pane_range: Option<(f64, f64)>,
    expressions: Option<(f64, f64)>,
    limits: &[plot::LimitLine],
) -> Option<(f64, f64)> {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for (a, b) in pane_range.into_iter().chain(expressions) {
        lo = lo.min(a);
        hi = hi.max(b);
    }
    for limit in limits {
        lo = lo.min(limit.y);
        hi = hi.max(limit.y);
    }
    if !lo.is_finite() || !hi.is_finite() {
        None
    } else if hi - lo > FLAT_FIT_RELATIVE_SPAN * lo.abs().max(hi.abs()) {
        Some((lo, hi))
    } else if lo > 0.0 {
        Some((lo / 1.1, hi * 1.1))
    } else {
        Some((lo - 1.0, hi + 1.0))
    }
}

/// The Y interval a logarithmic pane fits itself to: the positive values its
/// curves draw, and its positive specification limits, padded by the same
/// share of the axis a linear fit is, in decades. A value at or below zero
/// has no place on the axis and leaves the fit alone.
pub fn pane_auto_log_y(
    positive: Option<(f64, f64)>,
    limits: &[plot::LimitLine],
) -> Option<(f64, f64)> {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for value in positive
        .into_iter()
        .flat_map(|(a, b)| [a, b])
        .chain(limits.iter().map(|limit| limit.y))
        .filter(|value| value.is_finite() && *value > 0.0)
    {
        lo = lo.min(value);
        hi = hi.max(value);
    }
    if lo > hi {
        return None;
    }
    let pad = (hi / lo).log10() * 0.08;
    if !(pad > 0.0 && pad.is_finite()) || hi - lo <= FLAT_FIT_RELATIVE_SPAN * hi {
        return Some((lo / 1.1, hi * 1.1));
    }
    let factor = 10f64.powf(pad);
    Some((lo / factor, hi * factor))
}

/// The active-run trace whose drawn curve passes closest to the pointer.
///
/// Two things make this the trace the reader is pointing at rather than an
/// approximation of it. The value is mapped to the screen through the pane's
/// own scale, which is the mapping the painter used — a linear guess on a
/// decade pane picks a curve the pointer is nowhere near. And a loop is
/// measured on every branch that reaches this abscissa, so clicking the return
/// leg of a hysteresis curve anchors to that curve rather than to whichever
/// neighbour happened to sit near its forward leg.
pub fn nearest_drawn_trace<'a>(
    pane_traces: &[(usize, &'a StripTrace)],
    x: f64,
    pointer_y: Option<f32>,
    plot_rect: egui::Rect,
    y_scale: XScale,
    (y0, y1): (f64, f64),
    interpolation: SampleInterpolation,
) -> Option<&'a StripTrace> {
    let screen_y = |value: f64| -> Option<f32> {
        let fraction = y_scale.normalize(value, y0, y1);
        (value.is_finite() && fraction.is_finite())
            .then(|| plot_rect.bottom() - fraction as f32 * plot_rect.height())
    };
    let mut samples: Vec<BranchSample> = Vec::new();
    let mut best: Option<(&StripTrace, f32)> = None;
    for (_, trace) in pane_traces.iter().filter(|(_, trace)| !trace.overlay) {
        // The ordinary sweep has one answer here and is spared the branch
        // walk; anything else is measured on every leg that reaches this
        // abscissa, so clicking the return leg of a loop finds that curve.
        let mut values: Vec<f64> = if trace.shape.is_single_ascending() {
            Vec::new()
        } else {
            sample_branches_into(
                &trace.x,
                &trace.y,
                &trace.shape,
                x,
                interpolation,
                &mut samples,
            );
            samples.iter().map(|sample| sample.value).collect()
        };
        if values.is_empty() {
            values.push(sample_at_with_shape(
                &trace.x,
                &trace.y,
                &trace.shape,
                x,
                interpolation,
            ));
        }
        for value in values {
            let Some(y) = screen_y(value) else { continue };
            let distance = pointer_y.map_or(0.0, |pointer| (pointer - y).abs());
            if best.is_none_or(|(_, closest)| distance < closest) {
                best = Some((trace, distance));
            }
        }
    }
    best.map(|(trace, _)| trace)
}

/// A marker already selected by the host from its quick or retained store.
pub struct MarkerPresentation<'a> {
    pub anchor: &'a WaveformPresentationKey,
    pub x: f64,
    pub kind: MarkerKind,
    pub color: egui::Color32,
    pub label: String,
}

/// Project source-anchored marker positions onto this pane's exact sweep branches.
pub fn plot_markers<'a>(
    model: &StripModel,
    pane: &UnitPane,
    interpolation: SampleInterpolation,
    markers: impl IntoIterator<Item = MarkerPresentation<'a>>,
) -> Vec<plot::Marker> {
    let mut projected = Vec::new();
    // Markers ride their anchored trace: Y is resampled here rather than
    // stored, so zoom, pan, and a re-run all leave the tag on the curve.
    for marker in markers {
        let color = marker.color;
        let label = marker.label;
        if marker.kind == MarkerKind::Spec {
            // A spec constrains the X position, which every pane of the
            // strip shares — so it draws on all of them.
            projected.push(plot::Marker::limit_line(marker.x, color, label));
            continue;
        }
        // A marker belongs to the pane that owns its trace's unit; the
        // other panes are a different scale and would misplace it.
        let anchored = pane
            .traces
            .iter()
            .filter_map(|&index| model.traces.get(index).map(|trace| (index, trace)))
            .find(|(_, trace)| !trace.overlay && anchor_key(model, trace) == *marker.anchor);
        let Some((_, trace)) = anchored else {
            continue;
        };
        // A loop has a value on each branch that reaches this X, and a tag on
        // only one of them points at half the evidence.
        let mut samples = Vec::new();
        sample_branches_into(
            &trace.x,
            &trace.y,
            &trace.shape,
            marker.x,
            interpolation,
            &mut samples,
        );
        if trace.shape.branch_count() <= 1 || samples.is_empty() {
            let y = sample_at_with_shape(&trace.x, &trace.y, &trace.shape, marker.x, interpolation);
            if y.is_finite() {
                projected.push(plot::Marker::point(marker.x, y, color, label));
            }
            continue;
        }
        for sample in &samples {
            if !sample.value.is_finite() {
                continue;
            }
            projected.push(plot::Marker::point(
                marker.x,
                sample.value,
                color,
                format!("{label} {}", branch_tag(&trace.shape, sample.run)),
            ));
        }
    }

    projected
}

/// The stroke a run trace is drawn with unless the reader chose another.
pub const DEFAULT_TRACE_WIDTH: f32 = 1.8;

/// Overlays and expressions keep the shipped 1.4 : 1.8 proportion to the
/// active run's traces whatever width those are drawn at.
const SECONDARY_TRACE_RATIO: f32 = 1.4 / 1.8;

/// A family with more members than this in one pane draws them as the band
/// they occupy, with the member under the pointer stroked over it.
///
/// Past this many, overlapping members of one quantity stop reading one by
/// one, while each still costs a stroke of two points per pixel column on
/// every frame: five hundred Monte Carlo trials were 6.6 million vertices and
/// 80 ms a frame to tessellate before a single one of them could be told
/// apart. Sixty-four curves still stroke inside one 60 Hz frame on a laptop.
pub(crate) const FAMILY_BAND_MEMBERS: usize = 64;

/// The members of one family in a pane, when there are too many to stroke one
/// by one.
pub(crate) struct FamilyBand<'a> {
    pub(crate) members: Vec<(usize, &'a StripTrace)>,
    pub(crate) overlay: bool,
}

/// Whether two curves are members of one family: the same quantity of the
/// same run across the family's members (its Monte Carlo trials, `.step`
/// points or corners), drawn at the same weight.
fn same_family(member: &StripTrace, trace: &StripTrace) -> bool {
    member.run_id == trace.run_id
        && member.overlay == trace.overlay
        && member.kind == trace.kind
        && member.source_waveform_name == trace.source_waveform_name
}

/// Group a pane's family members by their family and keep the families too
/// large to stroke one by one. The active run and each overlay are judged on
/// their own, because each is drawn at its own weight.
///
/// Only a family folds into a band. Curves of different quantities are never
/// one band however many share a pane: a band says its members are one
/// quantity spread by what varied between them, which is false of unrelated
/// node voltages, so they are always stroked one by one.
pub(crate) fn family_bands<'a>(pane_traces: &[(usize, &'a StripTrace)]) -> Vec<FamilyBand<'a>> {
    let mut families: Vec<FamilyBand<'a>> = Vec::new();
    for &(index, trace) in pane_traces {
        if trace.family_group_ordinal.is_none() {
            continue;
        }
        match families
            .iter_mut()
            .find(|family| same_family(family.members[0].1, trace))
        {
            Some(family) => family.members.push((index, trace)),
            None => families.push(FamilyBand {
                members: vec![(index, trace)],
                overlay: trace.overlay,
            }),
        }
    }
    families.retain(|family| family.members.len() > FAMILY_BAND_MEMBERS);
    families
}

/// Paint each family's band under the curves, through the same projection the
/// renderer strokes curves with.
pub(crate) fn paint_family_bands(
    painter: &egui::Painter,
    rect: egui::Rect,
    bands: &[(Arc<[[f64; 2]]>, egui::Color32)],
    y_scale: XScale,
    (y0, y1): (f64, f64),
) {
    let screen_y =
        |value: f64| rect.bottom() - (y_scale.normalize(value, y0, y1) as f32) * rect.height();
    for (band, color) in bands {
        let columns = band.len() as f32;
        let mut run: Vec<(egui::Pos2, egui::Pos2)> = Vec::new();
        for (column, [minimum, maximum]) in band.iter().enumerate() {
            let x = rect.left() + ((column as f32 + 0.5) / columns) * rect.width();
            let (top, bottom) = (screen_y(*maximum), screen_y(*minimum));
            if top.is_finite() && bottom.is_finite() {
                run.push((egui::pos2(x, top), egui::pos2(x, bottom)));
                continue;
            }
            if run.len() >= 2 {
                plot::paint_min_max_band(painter, &run, *color, 1.0);
            }
            run.clear();
        }
        if run.len() >= 2 {
            plot::paint_min_max_band(painter, &run, *color, 1.0);
        }
    }
}

/// How near the pointer, in points, a family member's stroke must pass for
/// the member to be highlighted.
const HIGHLIGHT_REACH: f32 = 8.0;

/// What the Y axis of a pane whose unit cannot be stated says in its place.
const NO_UNIT: &str = "no unit";

/// The row above a pane's tick column, where its unit is stated: an
/// expression whose unit cannot be stated says so there, while a plain
/// number's axis goes unmarked.
fn y_caption_row(pane: &UnitPane) -> plot::YCaptionRow {
    plot::YCaptionRow {
        without_unit: pane.unlabeled.then(|| NO_UNIT.to_owned()),
    }
}

/// The steps a tall pane's linear Y ladder aims for.
const DENSEST_Y_LADDER: usize = 6;

/// The densest linear Y ladder, [`DENSEST_Y_LADDER`] steps at most, whose
/// every label the pane has room to draw `pitch` apart. A pane too short even
/// for the sparsest keeps those of its ticks that can be labelled, so it
/// never draws a grid line it cannot say the value of.
fn labelled_y_axis(plot_height: f32, pitch: f32, axis_with: impl Fn(usize) -> Axis) -> Axis {
    const SPARSEST: usize = 2;
    let apart = |axis: &Axis, lower: f64, upper: f64| {
        ((upper - lower).abs() / (axis.max - axis.min)) as f32 * plot_height
    };
    let fits = |axis: &Axis| {
        axis.ticks
            .windows(2)
            .all(|pair| apart(axis, pair[0].0, pair[1].0) >= pitch)
    };
    if let Some(axis) = (SPARSEST..=DENSEST_Y_LADDER)
        .rev()
        .map(&axis_with)
        .find(fits)
    {
        return axis;
    }
    let mut axis = axis_with(SPARSEST);
    let mut kept: Option<f64> = None;
    let ticks = std::mem::take(&mut axis.ticks)
        .into_iter()
        .filter(|(value, _)| {
            let keep = kept.is_none_or(|last| apart(&axis, last, *value) >= pitch);
            if keep {
                kept = Some(*value);
            }
            keep
        })
        .collect();
    axis.ticks = ticks;
    axis
}

/// Display inputs resolved by the host for one unit-scoped pane.
pub struct PanePlot<'a> {
    pub x_range: (f64, f64),
    pub auto_y: (f64, f64),
    pub y_view: Option<(f64, f64)>,
    pub log_y: bool,
    pub left_margin: f32,
    pub readout: ReadoutPolicy,
    pub quantity: QuantityPresentationPolicy,
    pub minor_grid: bool,
    /// Stroke of the active run's traces, in points. Overlays and
    /// expressions are drawn proportionally thinner.
    pub trace_width: f32,
    pub horizontal_cursors: [Option<f64>; 2],
    pub horizontal_cursor_interactive: bool,
    pub cursors: Option<CursorPair>,
    pub specification_limits: Vec<plot::LimitLine>,
    pub markers: Vec<plot::Marker>,
    pub expressions: &'a [ResolvedExpr],
    pub family_envelopes: &'a [FamilyEnvelopeSeries],
    /// Hit testing is only needed for marker placement or a newly anchored cursor A.
    pub find_nearest: bool,
}

pub struct PanePlotResponse<'a> {
    pub response: plot::PlotResponse,
    pub nearest_trace: Option<&'a StripTrace>,
}

/// Draw source curves, envelopes, expressions and markers with the exact retained readout.
pub fn show_plot<'a>(
    ui: &mut Ui,
    cache: &mut plot::DecimationCache,
    model: &'a StripModel,
    pane: &UnitPane,
    input: PanePlot<'_>,
) -> PanePlotResponse<'a> {
    let (x0, x1) = input.x_range;
    let (auto_y0, auto_y1) = input.auto_y;
    let log_y = input.log_y;
    let presentation = input.readout;
    let quantity_policy = input.quantity;
    let significant_digits = usize::from(presentation.displayed_significant_digits().get());
    let interpolation = cursor_interpolation(presentation.cursor_interpolation());
    let exprs = input.expressions;
    let (mut y0, mut y1) = input
        .y_view
        .filter(|(minimum, maximum)| !log_y || (*minimum > 0.0 && *maximum > 0.0))
        .unwrap_or((auto_y0, auto_y1));

    let x_axis = model_x_axis(model, x0, x1, quantity_policy);
    let lattice = !log_y && pane.axis_unit() == "°" && input.y_view.is_none();
    if lattice {
        // An unzoomed degree pane keeps the 45° lattice a Bode phase
        // reading expects; arbitrary zoom depths fall back to plain linear
        // ticks, which stay legible where the lattice would crowd.
        y0 = (y0 / 45.0).floor() * 45.0;
        y1 = (y1 / 45.0).ceil() * 45.0;
    }
    let y_axis_with = |density: usize| {
        let y_axis = if log_y {
            Axis::log_decades(y0, y1, pane.axis_unit())
        } else if lattice {
            // Continuous phase does not stay inside one turn: an unwrapped
            // loop response walks thousands of degrees, and 45° steps across
            // it are thousands of labels stacked into an unreadable band —
            // and thousands of galleys laid out every frame. The lattice
            // thins by whole 45° multiples so what is left still falls on the
            // values a phase reading is taken at.
            // A shorter pane halves the lattice again for each step of
            // density it gives up.
            let steps = ((y1 - y0) / 45.0).round().max(0.0) as usize;
            let stride = (steps + 1).div_ceil(MAX_AXIS_TICKS).max(1)
                << DENSEST_Y_LADDER.saturating_sub(density);
            let ticks: Vec<f64> = (0..=steps)
                .step_by(stride)
                .map(|index| 45.0f64.mul_add(index as f64, y0))
                .collect();
            Axis::with_ticks(y0, y1, "°", &ticks)
        } else {
            Axis::linear_with(y0, y1, pane.axis_unit(), density)
        };
        if pane.axis_unit() == "rad" {
            match quantity_policy.angle_display {
                rspice_vocabulary::quantity::AngleDisplay::Degrees => {
                    y_axis.with_display_transform(180.0 / std::f64::consts::PI, 0.0, "°")
                }
                rspice_vocabulary::quantity::AngleDisplay::Radians => y_axis,
            }
        } else if pane.axis_unit() == "°" {
            let (scale, offset, unit) = quantity_policy.degree_axis_transform();
            y_axis.with_display_transform(scale, offset, unit)
        } else {
            y_axis
        }
    };
    // The Y ladder is as dense as the pane has room to label: a short pane
    // carries fewer ticks, each labelled, rather than the tall pane's six
    // with every other label dropped where they would collide. A decade
    // ladder has one density.
    let densest = y_axis_with(DENSEST_Y_LADDER);
    let y_axis = if log_y {
        densest
    } else {
        let plot_height = plot::plot_rect(
            ui,
            &PlotSpec::new(Axis::linear(0.0, 1.0, ""), model.x_scale, densest)
                .without_x_axis_chrome()
                .with_y_caption_row(y_caption_row(pane)),
        )
        .height();
        labelled_y_axis(plot_height, plot::y_tick_pitch(ui), y_axis_with)
    };
    // The scale the pane is actually drawn on. Every hit test below maps
    // through it rather than assuming a linear ordinate, because "nearest"
    // has to mean nearest on screen — and on a decade pane a linear guess is
    // wrong by most of the window.
    let y_scale = if log_y { XScale::Log10 } else { XScale::Linear };
    let pane_traces: Vec<(usize, &StripTrace)> = pane
        .traces
        .iter()
        .copied()
        .filter_map(|index| model.traces.get(index).map(|trace| (index, trace)))
        .collect();
    let families = family_bands(&pane_traces);
    // The plot's own description is where the caution has to live as words.
    // The kind tag carries it as colour and a glyph, and neither of those
    // reaches a reader who cannot see the strip. A band says which family it
    // stands for, and how many members, for the same reason.
    let detail = families
        .iter()
        .map(|family| {
            format!(
                "{} members of {} drawn as the band they occupy",
                family.members.len(),
                family.members[0].1.base_name
            )
        })
        .chain(model.incomplete.map(str::to_owned))
        .collect::<Vec<_>>()
        .join(". ");
    let mut spec = PlotSpec::new(x_axis, model.x_scale, y_axis)
        .accessible_name("Waveform plot")
        .without_x_axis_chrome()
        .with_y_caption_row(y_caption_row(pane))
        .with_right_margin(WAVE_SHARED_RIGHT_MARGIN);
    if !detail.is_empty() {
        spec = spec.accessible_detail(&detail);
    }
    spec.left_margin = input.left_margin;
    if log_y {
        spec = spec.with_log_y();
    }
    spec.limit_lines = input.specification_limits;
    spec.minor_grid = input.minor_grid;
    spec.horizontal_cursors = input.horizontal_cursors;
    spec.horizontal_cursor_interactive = input.horizontal_cursor_interactive;
    spec.markers = input.markers;

    // 0 dB reference on a log-magnitude pane.
    if pane.axis_unit() == "dB" && y0 < 0.0 && y1 > 0.0 {
        spec.ref_lines.push(plot::RefLine { y: 0.0 });
    }

    // A large family is drawn as the band it occupies, with the member the
    // reader points at stroked over it; see `FAMILY_BAND_MEMBERS`. A family's
    // envelope is that same band, drawn under its members.
    let mut banded = std::collections::HashSet::new();
    let mut highlighted: Vec<usize> = Vec::new();
    let enveloped: Vec<Vec<(usize, &StripTrace)>> = input
        .family_envelopes
        .iter()
        .map(|envelope| {
            envelope
                .members
                .iter()
                .filter_map(|index| model.traces.get(*index).map(|trace| (*index, trace)))
                .collect::<Vec<_>>()
        })
        .filter(|members| members.len() >= 2)
        .collect();
    let family_in_pane = pane_traces
        .iter()
        .any(|(_, trace)| trace.family_group_ordinal.is_some());
    if !families.is_empty() || !enveloped.is_empty() || family_in_pane {
        let plot_rect = plot::plot_rect(ui, &spec);
        // The renderer's own column bucketing, so a band is quantized as its
        // members' envelopes would have been.
        let columns = (plot_rect.width().ceil().max(0.0) as usize).next_multiple_of(64);
        let view = plot::TraceView {
            x0: spec.x.min,
            x1: spec.x.max,
            y0: spec.y.min,
            y1: spec.y.max,
            x_scale: spec.x_scale,
            y_scale: spec.y_scale,
            columns,
            rows: 0,
        };
        let pointer = ui
            .input(|input| input.pointer.hover_pos())
            .filter(|position| {
                plot_rect.contains(*position) && ui.rect_contains_pointer(plot_rect)
            });
        let mut band = |members: &[(usize, &StripTrace)]| {
            let identity = stable_hash(
                &members
                    .iter()
                    .map(|(_, trace)| trace_key(model, trace))
                    .collect::<Vec<_>>(),
            );
            let members: Vec<plot::BandMember<'_>> = members
                .iter()
                .map(|(_, trace)| plot::BandMember {
                    x: trace.x.as_slice(),
                    y: trace.y.as_slice(),
                    parametric: false,
                    shape: Some(trace.shape.as_ref()),
                })
                .collect();
            cache.band(identity, &members, view)
        };
        let mut bands = Vec::with_capacity(families.len() + enveloped.len());
        for family in &families {
            // The quantity's own hue: a band stands for every member, not for
            // the one a family encoding happened to colour first.
            let color = family.members[0].1.signal_color;
            bands.push((
                band(&family.members),
                if family.overlay {
                    color.gamma_multiply(0.40)
                } else {
                    color
                },
            ));
            banded.extend(family.members.iter().map(|(index, _)| *index));
            if let Some(position) = pointer.filter(|_| !family.overlay) {
                let x = spec.x_scale.denormalize(
                    f64::from((position.x - plot_rect.left()) / plot_rect.width()),
                    spec.x.min,
                    spec.x.max,
                );
                let nearest = nearest_drawn_trace(
                    &family.members,
                    x,
                    Some(position.y),
                    plot_rect,
                    y_scale,
                    (spec.y.min, spec.y.max),
                    interpolation,
                );
                highlighted.extend(
                    family
                        .members
                        .iter()
                        .filter(|(_, trace)| nearest.is_some_and(|near| std::ptr::eq(near, *trace)))
                        .map(|(index, _)| *index),
                );
            }
        }
        // An envelope over a family already drawn as its band would be the
        // same band twice.
        for members in &enveloped {
            if members.iter().all(|(index, _)| banded.contains(index)) {
                continue;
            }
            bands.push((band(members), members[0].1.signal_color));
        }
        // The member of a family the pointer is on is the one the reader is
        // reading: it strokes over its siblings at the active run's width and
        // the readout names it first.
        if let Some(position) = pointer.filter(|_| highlighted.is_empty()) {
            let members: Vec<(usize, &StripTrace)> = pane_traces
                .iter()
                .copied()
                .filter(|(index, trace)| {
                    trace.visible
                        && !trace.overlay
                        && trace.family_group_ordinal.is_some()
                        && !banded.contains(index)
                })
                .collect();
            let x = spec.x_scale.denormalize(
                f64::from((position.x - plot_rect.left()) / plot_rect.width()),
                spec.x.min,
                spec.x.max,
            );
            let nearest = nearest_drawn_trace(
                &members,
                x,
                Some(position.y),
                plot_rect,
                y_scale,
                (spec.y.min, spec.y.max),
                interpolation,
            )
            .filter(|nearest| {
                // Only a member the pointer is on: a pointer anywhere else in
                // the plot is placing a cursor or reading the axis.
                let value =
                    sample_at_with_shape(&nearest.x, &nearest.y, &nearest.shape, x, interpolation);
                let y = plot_rect.bottom()
                    - (y_scale.normalize(value, spec.y.min, spec.y.max) as f32)
                        * plot_rect.height();
                (y - position.y).abs() <= HIGHLIGHT_REACH
            });
            highlighted.extend(
                members
                    .iter()
                    .filter(|(_, trace)| nearest.is_some_and(|near| std::ptr::eq(near, *trace)))
                    .map(|(index, _)| *index),
            );
        }
        let (band_scale, band_range) = (spec.y_scale, (spec.y.min, spec.y.max));
        spec.underlay = Some(Box::new(move |painter, mapper| {
            paint_family_bands(painter, mapper.rect, &bands, band_scale, band_range);
        }));
    }

    // Run owns weight: overlay traces keep the signal hue at reduced alpha
    // and stroke, painted first so the active run draws at full strength
    // on top.
    // A highlighted member is stroked last, over everything it was among.
    let draw_order = pane_traces
        .iter()
        .filter(|(_, trace)| trace.overlay)
        .chain(
            pane_traces
                .iter()
                .filter(|(index, trace)| !trace.overlay && !highlighted.contains(index)),
        )
        .chain(
            pane_traces
                .iter()
                .filter(|(index, trace)| !trace.overlay && highlighted.contains(index)),
        )
        .filter(|(index, _)| !banded.contains(index) || highlighted.contains(index));
    let highlighted_members: Vec<&StripTrace> = highlighted
        .iter()
        .filter(|index| !banded.contains(*index))
        .filter_map(|index| model.traces.get(*index))
        .collect();
    // A member of the family a highlighted member belongs to.
    let sibling_of_highlight = |trace: &StripTrace| {
        trace.family_group_ordinal.is_some()
            && highlighted_members
                .iter()
                .any(|member| same_family(member, trace))
    };
    for (index, trace) in draw_order {
        // While one member of a family that is stroked member by member is
        // highlighted, its siblings step back to an overlay's weight: members
        // of one quantity share its hue, and a wider stroke alone is lost
        // among them.
        let lifted = highlighted.contains(index) && !banded.contains(index);
        let sibling = !lifted && sibling_of_highlight(trace);
        let color = if trace.overlay || sibling {
            trace.color.gamma_multiply(0.40)
        } else {
            trace.color
        };
        // The reduction has to be told what the abscissa is. Without it a
        // reverse sweep vanished the moment it was zoomed — every window it
        // was asked for came back empty — and a hysteresis loop lost whichever
        // branch fell outside one contiguous index window.
        let mut plot_trace = strip_trace_cues(
            Trace::new(&trace.x, &trace.y, color)
                .width(input.trace_width)
                .cache_key(trace_key(model, trace))
                .shape(&trace.shape),
            trace.palette_slot,
            trace.family_style,
            trace.waveform_index,
        );
        if trace.overlay {
            plot_trace = plot_trace.width(input.trace_width * SECONDARY_TRACE_RATIO);
        }
        if lifted {
            plot_trace = plot_trace.width(input.trace_width);
        }
        spec.traces.push(plot_trace);
    }
    for expr in exprs {
        let mut trace = apply_family_trace_style(
            Trace::new(&expr.x, &expr.y, expr.color)
                .width(input.trace_width * SECONDARY_TRACE_RATIO)
                .cache_key(expr.cache_key)
                .shape(&expr.shape),
            expr.family_style,
        );
        // An expression's dash and marker follow its hue, as a signal's do,
        // so its legend swatch is its curve wherever the pane lists it. A
        // real lane's step trace is no expression.
        if expr.slot != usize::MAX {
            trace = trace.palette_slot(super::stack::expression_hue_slot(model, expr.slot));
        }
        spec.traces.push(trace);
    }

    // A family reads out the member the reader points at, not the first six
    // of its members.
    let readout_traces: Vec<&StripTrace> = highlighted
        .iter()
        .filter_map(|index| model.traces.get(*index))
        .chain(
            pane_traces
                .iter()
                .filter(|(index, trace)| {
                    !banded.contains(index)
                        && !highlighted.contains(index)
                        && !sibling_of_highlight(trace)
                })
                .map(|(_, trace)| *trace),
        )
        .take(6)
        .collect();
    let readout = |x: f64| -> Vec<(String, String)> {
        let mut rows = vec![(
            model.x_label().to_owned(),
            model.format_x(x, significant_digits, quantity_policy),
        )];
        let mut samples: Vec<BranchSample> = Vec::new();
        for trace in &readout_traces {
            sample_branches_into(
                &trace.x,
                &trace.y,
                &trace.shape,
                x,
                interpolation,
                &mut samples,
            );
            // A sweep with one answer at this X keeps its single unlabelled
            // row. A loop reports each branch that reaches here, because one
            // of the two numbers is not the reading.
            if trace.shape.branch_count() <= 1
                || samples.is_empty()
                || samples.len() > MAX_READOUT_BRANCHES
            {
                let value =
                    sample_at_with_shape(&trace.x, &trace.y, &trace.shape, x, interpolation);
                rows.push((
                    trace.name.clone(),
                    model.format_trace_value(trace, value, significant_digits, quantity_policy),
                ));
                continue;
            }
            for sample in &samples {
                rows.push((
                    format!("{} {}", trace.name, branch_tag(&trace.shape, sample.run)),
                    model.format_trace_value(
                        trace,
                        sample.value,
                        significant_digits,
                        quantity_policy,
                    ),
                ));
            }
        }
        for expr in exprs.iter().take(3) {
            let value = sample_at_with_shape(&expr.x, &expr.y, &expr.shape, x, interpolation);
            rows.push((
                expr.label.clone(),
                // The hover readout is a readout: the expression's value in
                // the unit of the pane that draws it, not the reader's digit
                // preference spelled out in zeros, nor a prefix with no unit.
                super::units::fmt_in_unit_readout(value, pane.axis_unit(), significant_digits),
            ));
        }
        rows
    };

    let response = plot::show(ui, &spec, cache, input.cursors.as_ref(), Some(&readout));
    if pane.unlabeled
        && let Some(caption) = response.y_caption
    {
        // The axis says it has no unit; why is the expression's to say.
        let reason = model
            .expression_panes
            .iter()
            .filter(|expression| pane.expressions.contains(&expression.slot))
            .find_map(|expression| expression.unknown.as_deref())
            .unwrap_or("its unit cannot be stated");
        let explained = ui.interact(
            caption,
            response.response.id.with("no-unit-caption"),
            egui::Sense::hover(),
        );
        explained.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, NO_UNIT));
        explained.on_hover_text(format!(
            "No unit: {reason}, so this expression has an axis of its own"
        ));
    }
    let nearest_trace = response
        .clicked_x
        .filter(|_| input.find_nearest)
        .and_then(|x| {
            nearest_drawn_trace(
                &pane_traces,
                x,
                response.response.interact_pointer_pos().map(|pos| pos.y),
                response.plot_rect,
                y_scale,
                (y0, y1),
                interpolation,
            )
        });
    PanePlotResponse {
        response,
        nearest_trace,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A curve coloured by its family's policy has no palette slot. With
    /// color-safe traces on, its pane draws it with the symbol its legend
    /// chip shows wherever it falls in the pane's draw order, and with them
    /// off it draws solid.
    #[test]
    fn a_curve_without_a_palette_slot_draws_the_symbol_its_chip_shows() {
        let style = FamilyTraceStyle {
            color: None,
            dash_ordinal: None,
            marker_ordinal: None,
            width_points: Some(1.2),
        };
        for waveform_index in 0..12 {
            let chip = SwatchStyle {
                palette_slot: None,
                family_style: Some(style),
                ordinal: waveform_index,
            };
            for draw_index in 0..12 {
                let curve = strip_trace_cues(
                    Trace::new(&[], &[], egui::Color32::RED),
                    None,
                    Some(style),
                    waveform_index,
                );
                assert_eq!(
                    curve.look(draw_index, true),
                    chip.look(true),
                    "waveform {waveform_index} drawn {draw_index}th"
                );
                let solid = curve.look(draw_index, false);
                assert_eq!((solid.dash, solid.marker), (None, None));
            }
        }
    }
}
