//! Unit-scoped waveform plot construction, retained readout and curve hit testing.

use super::navigation::model_x_axis;
use super::readout::{MAX_READOUT_BRANCHES, branch_tag};
use super::{
    ReadoutPolicy, StripModel, StripTrace, UnitPane, anchor_key, cursor_interpolation, stable_hash,
    trace_key,
};
use egui::Ui;
use rspice_results::family_projection::FamilyTraceStyle;
use rspice_results::result_presentation::{MarkerKind, WaveformPresentationKey};
use rspice_results::visualization_document::TraceStyle;
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
#[derive(Clone)]
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
    /// How the signals it reads from other runs are read onto its axis,
    /// which its legend entry states: `Run 2, linear X`.
    pub reading: Option<String>,
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
    /// The style the view gives the curve.
    pub style: TraceStyle,
}

impl SwatchStyle {
    pub fn of(trace: &StripTrace) -> Self {
        Self {
            palette_slot: trace.palette_slot,
            family_style: trace.family_style,
            ordinal: trace.waveform_index,
            style: trace.style,
        }
    }

    /// The curve's dash and marker, exactly as its pane resolves them, so a
    /// legend or readout swatch is the curve.
    pub fn look(&self, color_safe: bool) -> TraceLook {
        styled(
            strip_trace_cues(
                Trace::new(&[], &[], egui::Color32::PLACEHOLDER),
                self.palette_slot,
                self.family_style,
                self.ordinal,
            ),
            self.style,
        )
        .look(self.ordinal, color_safe)
    }
}

/// `plot` as the reader styled it: their width, dash, symbol and joining
/// where they chose one, its cues elsewhere.
pub(crate) fn styled(plot: Trace<'_>, style: TraceStyle) -> Trace<'_> {
    use rspice_results::visualization_document::{TraceDraw, TraceSymbol};
    use rspice_ui_kit::plot::{DrawMode, TraceMarkerShape};
    let plot = match style.width {
        Some(width) => plot.width(width.points()),
        None => plot,
    };
    let marker = style.symbol.map(|symbol| match symbol {
        TraceSymbol::None => None,
        TraceSymbol::Circle => Some(TraceMarkerShape::Circle),
        TraceSymbol::Square => Some(TraceMarkerShape::Square),
        TraceSymbol::Diamond => Some(TraceMarkerShape::Diamond),
        TraceSymbol::Triangle => Some(TraceMarkerShape::Triangle),
        TraceSymbol::Cross => Some(TraceMarkerShape::Cross),
        TraceSymbol::Plus => Some(TraceMarkerShape::Plus),
    });
    plot.chosen_look(style.dash.map(|dash| dash.pattern()), marker)
        .draw(match style.draw {
            TraceDraw::Line => DrawMode::Line,
            TraceDraw::Points => DrawMode::Points,
            TraceDraw::Steps => DrawMode::Steps,
            TraceDraw::Stems => DrawMode::Stems,
        })
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

/// The samples the curve of `trace`, the strip's `index`th, draws in
/// `color`: a stored trace's from its store's tiles, never its columns; one
/// held in memory from its columns; one in memory over a stored axis from the
/// read of that axis in `held`. `None` while that read is on its way.
fn curve_samples<'t>(
    trace: &'t StripTrace,
    index: usize,
    held: &'t std::collections::HashMap<
        usize,
        (
            rspice_results::waveform::HeldColumn,
            rspice_results::waveform::HeldColumn,
        ),
    >,
    color: egui::Color32,
) -> Option<Trace<'t>> {
    if trace.y.is_stored() {
        return Some(Trace::stored(&trace.x, &trace.y, color));
    }
    let (xs, ys): (&[f64], &[f64]) = match (trace.x.in_memory(), trace.y.in_memory()) {
        (Some(xs), Some(ys)) => (xs, ys),
        _ => {
            let (xs, ys) = held.get(&index)?;
            (xs, ys)
        }
    };
    Some(Trace::new(xs, ys, color).shared(&trace.x, &trace.y))
}

/// The curves of `traces`, `model`'s, as a pane strokes them. Run owns
/// weight: overlay traces keep the signal hue at reduced alpha and stroke,
/// painted first so the active run draws at full strength on top. A member
/// `highlighted` is stroked last, over everything it was among; one of a
/// family `banded` is left to its band unless it is highlighted.
fn pane_curves<'m>(
    model: &'m StripModel,
    traces: &[(usize, &'m StripTrace)],
    (highlighted, banded): (&[usize], &std::collections::HashSet<usize>),
    held: &'m std::collections::HashMap<
        usize,
        (
            rspice_results::waveform::HeldColumn,
            rspice_results::waveform::HeldColumn,
        ),
    >,
    trace_width: f32,
) -> Vec<Trace<'m>> {
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
    let mut curves = Vec::new();
    let draw_order = traces
        .iter()
        .filter(|(_, trace)| trace.overlay)
        .chain(
            traces
                .iter()
                .filter(|(index, trace)| !trace.overlay && !highlighted.contains(index)),
        )
        .chain(
            traces
                .iter()
                .filter(|(index, trace)| !trace.overlay && highlighted.contains(index)),
        )
        .filter(|(index, _)| !banded.contains(index) || highlighted.contains(index));
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
        // A stored trace draws from its store's tiles, never its columns.
        let Some(samples) = curve_samples(trace, *index, held, color) else {
            continue;
        };
        let mut plot_trace = strip_trace_cues(
            samples
                .width(trace_width)
                .cache_key(trace_key(model, trace))
                .shape(&trace.shape),
            trace.palette_slot,
            trace.family_style,
            trace.waveform_index,
        );
        let width = trace
            .style
            .width
            .map_or(trace_width, |width| width.points());
        plot_trace = styled(plot_trace, trace.style);
        if trace.overlay {
            plot_trace = plot_trace.width(width * SECONDARY_TRACE_RATIO);
        }
        if lifted {
            plot_trace = plot_trace.width(width);
        }
        curves.push(plot_trace);
    }
    curves
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
    let pad = (hi / lo).log10() * plot::FIT_HEADROOM;
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
        } else if let Some((xs, ys)) = trace.held() {
            sample_branches_into(&xs, &ys, &trace.shape, x, interpolation, &mut samples);
            samples.iter().map(|sample| sample.value).collect()
        } else {
            Vec::new()
        };
        if values.is_empty() {
            values.push(super::trace_value_at(trace, x, interpolation));
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
        if !marker.kind.rides_a_trace() {
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
        if !trace.shape.is_single_ascending()
            && let Some((xs, ys)) = trace.held()
        {
            sample_branches_into(
                &xs,
                &ys,
                &trace.shape,
                marker.x,
                interpolation,
                &mut samples,
            );
        }
        if trace.shape.branch_count() <= 1 || samples.is_empty() {
            let y = model.trace_value(trace, marker.x, interpolation);
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
pub(crate) const UNKNOWN_UNIT: &str = "unit unknown";

/// How a pane of plain numbers, a ratio say, is named.
pub(crate) const NO_UNIT: &str = "no unit";

/// A pane's plot as its axes read: an expression whose unit cannot be
/// stated says so at the head of its column, while a plain number's axis
/// goes unmarked.
fn pane_spec<'a>(spec: PlotSpec<'a>, pane: &UnitPane) -> PlotSpec<'a> {
    let spec = spec.without_x_axis_chrome();
    if pane.unlabeled {
        spec.with_y_without_unit(UNKNOWN_UNIT)
    } else {
        spec
    }
}

/// The ladder an added axis reading `range` labels along `column`, as dense
/// as the strip's first axis would be.
pub(crate) fn secondary_ticks(column: plot::LabelColumn, range: (f64, f64)) -> Axis {
    labelled_y_axis(column, |target| {
        Axis::linear_with(range.0, range.1, "", target)
    })
}

/// The steps a tall pane's linear Y ladder aims for.
const DENSEST_Y_LADDER: usize = 6;

/// The densest Y ladder, [`DENSEST_Y_LADDER`] steps at most, whose every
/// label stands clear along `column`, the labels of ticks on the frame
/// nudged inside it as they are drawn. A pane too short even for the
/// sparsest coarsens a linear ladder further, to two ticks, and names what
/// still crowds at even strides from the top: a short pane reads even steps,
/// never a scale with a value missing from its middle.
pub(crate) fn labelled_y_axis(
    column: plot::LabelColumn,
    axis_with: impl Fn(usize) -> Axis,
) -> Axis {
    const SPARSEST: usize = 2;
    let fits = |axis: &Axis| {
        column.holds(
            axis.ticks
                .iter()
                .map(|(value, _)| (value - axis.min) / (axis.max - axis.min)),
        )
    };
    (SPARSEST..=DENSEST_Y_LADDER)
        .rev()
        .map(&axis_with)
        .find(fits)
        .unwrap_or_else(|| {
            let sparsest = axis_with(SPARSEST);
            sparsest.fitted(column).unwrap_or(sparsest)
        })
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
    /// The levels a loop's margins are read against on this pane, drawn as
    /// reference lines.
    pub references: Vec<f64>,
    /// What the comparison that last answered marks on the pane.
    pub comparison: ComparisonMarks,
    pub expressions: &'a [ResolvedExpr],
    pub family_envelopes: &'a [FamilyEnvelopeSeries],
    /// Hit testing is only needed for marker placement or a newly anchored cursor A.
    pub find_nearest: bool,
    /// The strip's Y axes after its first, each with the curves read against
    /// it and the range it shows.
    pub secondary: &'a [SecondaryPlot<'a>],
    /// What the plot keeps right of its area: the axes standing there.
    pub right_margin: f32,
    /// The ink of the first axis' labels, when it says whose axis it is by
    /// its curves' one colour.
    pub y_ink: Option<egui::Color32>,
    /// The strips the sheet shows, the pane's among them, which a pane of
    /// many curves draws from where the frame is not.
    pub strips: Option<&'a Arc<Vec<StripModel>>>,
}

/// One Y axis of a strip after its first, as its plot draws it: the curves
/// read against it, over the range it shows.
pub struct SecondaryPlot<'a> {
    pub unit: &'a str,
    pub traces: &'a [usize],
    pub expressions: &'a [ResolvedExpr],
    pub range: (f64, f64),
    /// The key its range is pinned by in the session's viewports.
    pub slot: usize,
}

/// The tolerance a difference is judged against, drawn as the band about
/// zero the difference may stay inside: at each X, the absolute tolerance
/// and the relative one's share of the baseline there.
#[derive(Debug, Clone, PartialEq)]
pub struct ToleranceBand {
    pub x: Vec<f64>,
    /// How far from zero the difference may stand at each X.
    pub half_width: Vec<f64>,
    pub fill: egui::Color32,
    /// The band's two edges.
    pub edge: egui::Color32,
}

impl ToleranceBand {
    /// The Y interval the band spans.
    #[must_use]
    pub fn extremes(&self) -> Option<(f64, f64)> {
        let widest = self
            .half_width
            .iter()
            .copied()
            .filter(|width| width.is_finite())
            .fold(None, |widest: Option<f64>, width| {
                Some(widest.map_or(width, |widest| widest.max(width)))
            })?;
        Some((-widest, widest))
    }
}

/// What a comparison marks on a pane: the places its runs part by more than
/// the tolerance, and the tolerance band of a difference drawn there.
#[derive(Debug, Clone, Default)]
pub struct ComparisonMarks {
    /// The X span of each place, a list per compared signal the pane draws.
    pub places: Vec<Arc<[(f64, f64)]>>,
    pub place_color: egui::Color32,
    pub tolerance: Option<Arc<ToleranceBand>>,
}

impl ComparisonMarks {
    fn is_empty(&self) -> bool {
        self.places.iter().all(|places| places.is_empty()) && self.tolerance.is_none()
    }
}

/// Shade each place across the plot, at least a few points wide so a place
/// of one sample shows at any zoom.
fn paint_places(
    painter: &egui::Painter,
    mapper: &plot::PlotMapper,
    places: &[Arc<[(f64, f64)]>],
    color: egui::Color32,
) {
    const NARROWEST: f32 = 3.0;
    let rect = mapper.rect;
    for &(first, last) in places.iter().flat_map(|places| places.iter()) {
        let (left, right) = (mapper.x(first), mapper.x(last));
        let (left, right) = (left.min(right), left.max(right));
        let grow = ((NARROWEST - (right - left)) * 0.5).max(0.0);
        let (left, right) = (
            (left - grow).max(rect.left()),
            (right + grow).min(rect.right()),
        );
        if left < right {
            painter.rect_filled(
                egui::Rect::from_x_y_ranges(left..=right, rect.y_range()),
                0.0,
                color,
            );
        }
    }
}

/// Fill the band between `-half_width` and `half_width`, a quad per pair of
/// samples a point or more apart on screen, and stroke its two edges over
/// the fill.
fn paint_tolerance(painter: &egui::Painter, mapper: &plot::PlotMapper, band: &ToleranceBand) {
    let mut mesh = egui::Mesh::default();
    // Each unbroken run of the band: its top and bottom edges on screen.
    let mut runs: Vec<(Vec<egui::Pos2>, Vec<egui::Pos2>)> = vec![(Vec::new(), Vec::new())];
    for (&x, &width) in band.x.iter().zip(&band.half_width) {
        let Some((tops, bottoms)) = runs.last_mut() else {
            break;
        };
        if !(x.is_finite() && width.is_finite()) {
            if !tops.is_empty() {
                runs.push((Vec::new(), Vec::new()));
            }
            continue;
        }
        let (sx, top, bottom) = (mapper.x(x), mapper.y(width), mapper.y(-width));
        if let (Some(&previous_top), Some(&previous_bottom)) = (tops.last(), bottoms.last()) {
            if sx - previous_top.x < 1.0 && sx >= previous_top.x {
                continue;
            }
            let base = mesh.vertices.len() as u32;
            for vertex in [
                previous_top,
                egui::pos2(sx, top),
                egui::pos2(sx, bottom),
                previous_bottom,
            ] {
                mesh.colored_vertex(vertex, band.fill);
            }
            mesh.add_triangle(base, base + 1, base + 2);
            mesh.add_triangle(base, base + 2, base + 3);
        }
        tops.push(egui::pos2(sx, top));
        bottoms.push(egui::pos2(sx, bottom));
    }
    if !mesh.is_empty() {
        painter.add(egui::Shape::mesh(mesh));
    }
    for edge in runs.into_iter().flat_map(|(tops, bottoms)| [tops, bottoms]) {
        if edge.len() > 1 {
            painter.add(egui::Shape::line(edge, egui::Stroke::new(1.0, band.edge)));
        }
    }
}

pub struct PanePlotResponse<'a> {
    pub response: plot::PlotResponse,
    /// The width the first axis' labels and unit take left of the plot,
    /// outside which an axis on the left stands.
    pub y_labels_width: f32,
    pub nearest_trace: Option<&'a StripTrace>,
    /// What a right click on the plot found this frame.
    pub context: Option<PlotContext<'a>>,
}

/// Where a right click on the plot landed and what it found there: what the
/// plot's menu acts on.
pub struct PlotContext<'a> {
    pub x: f64,
    /// The value of the pane's first axis at the height clicked.
    pub y: f64,
    /// The curve nearest the click.
    pub trace: Option<&'a StripTrace>,
    /// What the hover readout reads at `x`.
    pub values: Vec<(String, String)>,
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
            &pane_spec(
                PlotSpec::new(Axis::linear(0.0, 1.0, ""), model.x_scale, densest),
                pane,
            ),
        )
        .height();
        labelled_y_axis(plot::y_label_column(ui.painter(), plot_height), y_axis_with)
    };
    // The scale the pane is actually drawn on. Every hit test below maps
    // through it rather than assuming a linear ordinate, because "nearest"
    // has to mean nearest on screen — and on a decade pane a linear guess is
    // wrong by most of the window.
    let y_scale = if log_y { XScale::Log10 } else { XScale::Linear };
    // A strip of no families draws its curves from one picture named by the
    // strip's revision: a frame that shows the picture it holds lists none of
    // them, and a hover or a click lists them when it needs them.
    let lazy = !model.trace_facts().families && input.family_envelopes.is_empty();
    let listed = std::cell::OnceCell::new();
    let pane_traces = || {
        listed
            .get_or_init(|| {
                pane.traces
                    .iter()
                    .copied()
                    .filter_map(|index| model.traces.get(index).map(|trace| (index, trace)))
                    .collect::<Vec<(usize, &StripTrace)>>()
            })
            .as_slice()
    };
    let families = if lazy {
        Vec::new()
    } else {
        family_bands(pane_traces())
    };
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
    // A trace held in memory over an axis a results store keeps (a magnitude in dB, an
    // expression, over a stored run) strokes its axis from a read made off the frame, and is
    // not drawn until that read has arrived. Only such traces are read here.
    let held_traces: std::collections::HashMap<usize, _> = model
        .trace_facts()
        .over_stored_axes
        .iter()
        .filter_map(|index| Some((*index, model.traces.get(*index)?.held()?)))
        .collect();
    let held_exprs: Vec<_> = exprs
        .iter()
        .map(|expr| super::held_pair(&expr.x, &expr.y))
        .collect();
    let held_secondary: Vec<Vec<_>> = input
        .secondary
        .iter()
        .map(|axis| {
            axis.expressions
                .iter()
                .map(|expr| super::held_pair(&expr.x, &expr.y))
                .collect()
        })
        .collect();
    // What the first axis' labels take, its unit riding on the top one, so
    // an axis on the left stands just outside them.
    let y_labels_width = y_axis
        .ticks
        .iter()
        .map(|(_, label)| label.clone())
        .chain(
            y_axis
                .ticks
                .last()
                .map(|(_, label)| plot::tick_with_unit(label, pane.axis_unit())),
        )
        .map(|text| {
            ui.painter()
                .layout_no_wrap(text, plot::y_tick_font(), egui::Color32::PLACEHOLDER)
                .size()
                .x
        })
        .fold(0.0, f32::max);
    // A large family is drawn as the band it occupies, with the member the
    // reader points at stroked over it; see `FAMILY_BAND_MEMBERS`. Declared
    // before the spec, whose curves are made from them.
    let mut banded = std::collections::HashSet::new();
    let mut highlighted: Vec<usize> = Vec::new();
    let mut spec = pane_spec(PlotSpec::new(x_axis, model.x_scale, y_axis), pane)
        .accessible_name("Waveform plot")
        .with_right_margin(input.right_margin)
        .with_significant_digits(significant_digits);
    if !detail.is_empty() {
        spec = spec.accessible_detail(&detail);
    }
    spec.left_margin = input.left_margin;
    if log_y {
        spec = spec.with_log_y();
    }
    spec.limit_lines = input.specification_limits;
    spec.minor_grid = input.minor_grid;
    spec.y_ink = input.y_ink;
    spec.horizontal_cursors = input.horizontal_cursors;
    spec.horizontal_cursor_interactive = input.horizontal_cursor_interactive;
    spec.markers = input.markers;

    // 0 dB reference on a log-magnitude pane.
    if pane.axis_unit() == "dB" && y0 < 0.0 && y1 > 0.0 {
        spec.ref_lines.push(plot::RefLine { y: 0.0 });
    }
    // A two-port's stability factor reads against the level that parts
    // stable from unstable: K = 1 and μ = 1.
    for level in exprs
        .iter()
        .filter_map(|expr| crate::s_parameters::reference_level(&expr.label))
    {
        if y0 < level && level < y1 && !spec.ref_lines.iter().any(|line| line.y == level) {
            spec.ref_lines.push(plot::RefLine { y: level });
        }
    }
    // The levels a loop’s margins are read against.
    for &y in &input.references {
        if y0.min(y1) < y && y < y0.max(y1) && spec.ref_lines.iter().all(|line| line.y != y) {
            spec.ref_lines.push(plot::RefLine { y });
        }
    }

    // A family's envelope is the band a large family is drawn as, drawn
    // under its members.
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
    let family_in_pane = !lazy
        && pane_traces()
            .iter()
            .any(|(_, trace)| trace.family_group_ordinal.is_some());
    if !families.is_empty() || !enveloped.is_empty() || family_in_pane {
        let plot_rect = plot::plot_rect(ui, &spec);
        // The renderer's own column count, so a band is quantized as its
        // members' envelopes would have been.
        let columns = plot::display_columns(plot_rect.width(), ui.ctx().pixels_per_point());
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
            if let Some(held) = cache.held_band(identity, view) {
                return held;
            }
            // A stored member is folded from its tile of the view, never read.
            let members: Vec<plot::BandMember<'_>> = members
                .iter()
                .map(|(_, trace)| {
                    let memory = trace.x.in_memory().zip(trace.y.in_memory());
                    plot::BandMember {
                        x: memory.map_or(&[], |(x, _)| x),
                        y: memory.map_or(&[], |(_, y)| y),
                        parametric: false,
                        shape: Some(trace.shape.as_ref()),
                        stored: memory.is_none().then_some(plot::StoredSamples {
                            x: &trace.x,
                            y: &trace.y,
                        }),
                    }
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
            let members: Vec<(usize, &StripTrace)> = pane_traces()
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
                let value = model.trace_value(nearest, x, interpolation);
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
    // The places the runs part and the tolerance band of a difference lie
    // under every curve; the band only on a linear axis, where zero and the
    // band about it can be drawn.
    if !input.comparison.is_empty() {
        let marks = input.comparison;
        let under = spec.underlay.take();
        spec.underlay = Some(Box::new(move |painter, mapper| {
            paint_places(painter, mapper, &marks.places, marks.place_color);
            if let Some(tolerance) = marks.tolerance.as_deref().filter(|_| !log_y) {
                paint_tolerance(painter, mapper, tolerance);
            }
            if let Some(under) = &under {
                under(painter, mapper);
            }
        }));
    }

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
    let make = || {
        let mut traces = pane_curves(
            model,
            pane_traces(),
            (&highlighted, &banded),
            &held_traces,
            input.trace_width,
        );
        for (expr, held) in exprs.iter().zip(&held_exprs) {
            let Some((expr_x, expr_y)) = held else {
                continue;
            };
            let mut trace = apply_family_trace_style(
                Trace::new(expr_x, expr_y, expr.color)
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
            traces.push(trace);
        }
        // The curves of the strip's other axes, each mapped through its own.
        for (axis, held) in input.secondary.iter().zip(&held_secondary) {
            let on = plot::TraceAxis {
                min: axis.range.0,
                max: axis.range.1,
                scale: XScale::Linear,
            };
            for index in axis.traces {
                let Some(trace) = model.traces.get(*index) else {
                    continue;
                };
                let color = if trace.overlay {
                    trace.color.gamma_multiply(0.40)
                } else {
                    trace.color
                };
                let Some(samples) = curve_samples(trace, *index, &held_traces, color) else {
                    continue;
                };
                let width = trace
                    .style
                    .width
                    .map_or(input.trace_width, |width| width.points());
                let width = if trace.overlay {
                    width * SECONDARY_TRACE_RATIO
                } else {
                    width
                };
                traces.push(
                    styled(
                        strip_trace_cues(
                            samples
                                .cache_key(trace_key(model, trace))
                                .shape(&trace.shape),
                            trace.palette_slot,
                            trace.family_style,
                            trace.waveform_index,
                        ),
                        trace.style,
                    )
                    .width(width)
                    .on_axis(on),
                );
            }
            for (expr, held) in axis.expressions.iter().zip(held) {
                let Some((expr_x, expr_y)) = held else {
                    continue;
                };
                let mut trace = apply_family_trace_style(
                    Trace::new(expr_x, expr_y, expr.color)
                        .width(input.trace_width * SECONDARY_TRACE_RATIO)
                        .cache_key(expr.cache_key)
                        .shape(&expr.shape),
                    expr.family_style,
                );
                if expr.slot != usize::MAX {
                    trace = trace.palette_slot(super::stack::expression_hue_slot(model, expr.slot));
                }
                traces.push(trace.on_axis(on));
            }
        }

        traces
    };
    if lazy {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        (
            model.revision(),
            super::indices_digest(pane.traces.iter().copied()),
            input.trace_width.to_bits(),
            held_traces.len(),
        )
            .hash(&mut hasher);
        let expression = |hasher: &mut std::collections::hash_map::DefaultHasher,
                          expr: &ResolvedExpr,
                          held: bool| {
            (
                expr.cache_key,
                expr.color.to_array(),
                expr.slot,
                held,
                format!("{:?}", expr.family_style),
            )
                .hash(hasher);
        };
        for (expr, held) in exprs.iter().zip(&held_exprs) {
            expression(&mut hasher, expr, held.is_some());
        }
        for (axis, held) in input.secondary.iter().zip(&held_secondary) {
            (
                axis.range.0.to_bits(),
                axis.range.1.to_bits(),
                super::indices_digest(axis.traces.iter().copied()),
            )
                .hash(&mut hasher);
            for (expr, held) in axis.expressions.iter().zip(held) {
                expression(&mut hasher, expr, held.is_some());
            }
        }
        // Every curve listed draws, but one held in memory over a stored axis
        // whose read is on its way.
        let waiting = model
            .trace_facts()
            .over_stored_axes
            .len()
            .saturating_sub(held_traces.len());
        let count = (pane.traces.len()
            + input
                .secondary
                .iter()
                .map(|axis| axis.traces.len())
                .sum::<usize>()
            + held_exprs.iter().flatten().count()
            + held_secondary.iter().flatten().flatten().count())
        .saturating_sub(waiting);
        // A pane of nothing but its traces makes them again where the frame
        // is not, from the strips the sheet shares.
        let off_frame = input
            .strips
            .filter(|_| {
                exprs.is_empty()
                    && input.secondary.is_empty()
                    && model.trace_facts().over_stored_axes.is_empty()
            })
            .and_then(|strips| {
                let strip = strips.iter().position(|other| std::ptr::eq(other, model))?;
                let strips = Arc::clone(strips);
                let indices = pane.traces.clone();
                let (highlighted, banded) = (highlighted.clone(), banded.clone());
                let trace_width = input.trace_width;
                let make: plot::OwnedCurves =
                    Arc::new(move |visit: &mut dyn FnMut(&[Trace<'_>])| {
                        let Some(model) = strips.get(strip) else {
                            return;
                        };
                        let traces: Vec<(usize, &StripTrace)> = indices
                            .iter()
                            .copied()
                            .filter_map(|index| model.traces.get(index).map(|trace| (index, trace)))
                            .collect();
                        let held = std::collections::HashMap::new();
                        visit(&pane_curves(
                            model,
                            &traces,
                            (&highlighted, &banded),
                            &held,
                            trace_width,
                        ));
                    });
                Some(make)
            });
        spec.curves = Some(plot::Curves {
            key: hasher.finish(),
            count,
            make: Box::new(make),
            off_frame,
        });
    } else {
        spec.traces.extend(make());
    }

    // A family reads out the member the reader points at, not the first six
    // of its members.
    let readout_traces = || {
        highlighted
            .iter()
            .filter_map(|index| model.traces.get(*index))
            .chain(
                pane_traces()
                    .iter()
                    .filter(|(index, trace)| {
                        !banded.contains(index)
                            && !highlighted.contains(index)
                            && !sibling_of_highlight(trace)
                    })
                    .map(|(_, trace)| *trace),
            )
            .chain(
                input
                    .secondary
                    .iter()
                    .flat_map(|axis| axis.traces.iter())
                    .filter_map(|index| model.traces.get(*index)),
            )
    };
    let readout = |x: f64| -> Vec<(String, String)> {
        let mut rows = vec![(
            model.x_label().to_owned(),
            model.format_x(x, significant_digits, quantity_policy),
        )];
        let mut samples: Vec<BranchSample> = Vec::new();
        for trace in readout_traces() {
            samples.clear();
            if !trace.shape.is_single_ascending()
                && let Some((xs, ys)) = trace.held()
            {
                sample_branches_into(&xs, &ys, &trace.shape, x, interpolation, &mut samples);
            }
            // A sweep with one answer at this X keeps its single unlabelled
            // row. A loop reports each branch that reaches here, because one
            // of the two numbers is not the reading.
            if trace.shape.branch_count() <= 1
                || samples.is_empty()
                || samples.len() > MAX_READOUT_BRANCHES
            {
                let value = model.trace_value(trace, x, interpolation);
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
        // Every curve the strip draws reads out, however many: a readout
        // that stops at a count leaves the rest unread.
        let expressions = exprs
            .iter()
            .zip(&held_exprs)
            .map(|(expr, held)| (expr, held, pane.axis_unit()))
            .chain(
                input
                    .secondary
                    .iter()
                    .zip(&held_secondary)
                    .flat_map(|(axis, held)| {
                        axis.expressions
                            .iter()
                            .zip(held)
                            .map(|(expr, held)| (expr, held, axis.unit))
                    }),
            );
        for (expr, held, unit) in expressions {
            let value = held.as_ref().map_or(f64::NAN, |(xs, ys)| {
                sample_at_with_shape(xs, ys, &expr.shape, x, interpolation)
            });
            rows.push((
                expr.label.clone(),
                // The hover readout is a readout: the expression's value in
                // the unit of the axis that draws it, not the reader's digit
                // preference spelled out in zeros, nor a prefix with no unit.
                rspice_ui_kit::plot::fmt_si_significant(value, unit, significant_digits),
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
        explained
            .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, UNKNOWN_UNIT));
        explained.on_hover_text(format!(
            "Unit unknown: {reason}, so this expression has an axis of its own"
        ));
    }
    let nearest_trace = response
        .clicked_x
        .filter(|_| input.find_nearest)
        .and_then(|x| {
            nearest_drawn_trace(
                pane_traces(),
                x,
                response.response.interact_pointer_pos().map(|pos| pos.y),
                response.plot_rect,
                y_scale,
                (y0, y1),
                interpolation,
            )
        });
    let context = response.context_at.map(|(x, y)| PlotContext {
        x,
        y,
        trace: nearest_drawn_trace(
            pane_traces(),
            x,
            response.response.interact_pointer_pos().map(|pos| pos.y),
            response.plot_rect,
            y_scale,
            (y0, y1),
            interpolation,
        ),
        values: readout(x),
    });
    PanePlotResponse {
        response,
        y_labels_width,
        nearest_trace,
        context,
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
                style: TraceStyle::default(),
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
