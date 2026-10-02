//! Unit-scoped waveform plot construction, retained readout and curve hit testing.

use super::navigation::{WAVE_SHARED_RIGHT_MARGIN, model_x_axis};
use super::readout::{MAX_READOUT_BRANCHES, branch_tag};
use super::{
    ReadoutPolicy, StripModel, StripTrace, UnitPane, anchor_key, cursor_interpolation, trace_key,
};
use egui::Ui;
use rspice_app_types::quantity::QuantityPresentationPolicy;
use rspice_results::family_projection::FamilyTraceStyle;
use rspice_results::result_presentation::{MarkerKind, WaveformPresentationKey};
use rspice_results::waveform::SharedWaveformValues;
use rspice_ui_kit::plot::sample::{
    BranchSample, SweepShape, sample_at_with_shape, sample_branches_into,
};
use rspice_ui_kit::plot::{
    self, Axis, CursorPair, DisplayDecimation, MAX_AXIS_TICKS, PlotSpec, SampleInterpolation,
    Trace, XScale, fmt_si_significant,
};
use std::sync::Arc;

/// One expression trace resolved for plotting.
pub struct ResolvedExpr {
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
}

#[derive(Debug)]
pub struct FamilyEnvelopeSeries {
    pub x: Vec<f64>,
    pub minimum: Vec<f64>,
    pub maximum: Vec<f64>,
    pub color: egui::Color32,
    pub minimum_cache_key: u64,
    pub maximum_cache_key: u64,
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

/// The Y interval a pane fits itself to when the reader has not pinned one.
///
/// Everything the pane draws widens it: the traces, whatever expressions the
/// strip carries, and the specification limits — a bound drawn off the top of
/// the axis is a bound the reader cannot check against.
pub fn pane_auto_y(
    pane_range: Option<(f64, f64)>,
    exprs: &[ResolvedExpr],
    limits: &[plot::LimitLine],
) -> Option<(f64, f64)> {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    if let Some((a, b)) = pane_range {
        lo = a;
        hi = b;
    }
    for expr in exprs {
        if let Some((a, b)) = expr.y_extremes {
            lo = lo.min(a);
            hi = hi.max(b);
        }
    }
    for limit in limits {
        lo = lo.min(limit.y);
        hi = hi.max(limit.y);
    }
    if !lo.is_finite() || !hi.is_finite() {
        None
    } else if lo == hi && lo > 0.0 {
        Some((lo / 1.1, hi * 1.1))
    } else if lo == hi {
        Some((lo - 1.0, hi + 1.0))
    } else {
        Some((lo, hi))
    }
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

/// Display inputs resolved by the host for one unit-scoped pane.
pub struct PanePlot<'a> {
    pub x_range: (f64, f64),
    pub auto_y: (f64, f64),
    pub y_view: Option<(f64, f64)>,
    pub log_y: bool,
    pub left_margin: f32,
    pub readout: ReadoutPolicy,
    pub quantity: QuantityPresentationPolicy,
    pub display_decimation: DisplayDecimation,
    pub minor_grid: bool,
    pub horizontal_cursor: Option<f64>,
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
    let y_axis = if log_y {
        Axis::log_decades(y0, y1, pane.unit)
    } else if pane.unit == "°" && input.y_view.is_none() {
        // An unzoomed degree pane keeps the 45° lattice a Bode phase
        // reading expects; arbitrary zoom depths fall back to plain linear
        // ticks, which stay legible where the lattice would crowd.
        y0 = (y0 / 45.0).floor() * 45.0;
        y1 = (y1 / 45.0).ceil() * 45.0;
        // Continuous phase does not stay inside one turn: an unwrapped loop
        // response walks thousands of degrees, and 45° steps across it are
        // thousands of labels stacked into an unreadable band — and thousands
        // of galleys laid out every frame. The lattice thins by whole 45°
        // multiples so what is left still falls on the values a phase reading
        // is taken at.
        let steps = ((y1 - y0) / 45.0).round().max(0.0) as usize;
        let stride = (steps + 1).div_ceil(MAX_AXIS_TICKS).max(1);
        let ticks: Vec<f64> = (0..=steps)
            .step_by(stride)
            .map(|index| 45.0f64.mul_add(index as f64, y0))
            .collect();
        Axis::with_ticks(y0, y1, "°", &ticks)
    } else {
        Axis::linear(y0, y1, pane.unit)
    };
    let y_axis = if pane.unit == "rad" {
        match quantity_policy.angle_display {
            rspice_app_types::quantity::AngleDisplay::Degrees => {
                y_axis.with_display_transform(180.0 / std::f64::consts::PI, 0.0, "°")
            }
            rspice_app_types::quantity::AngleDisplay::Radians => y_axis,
        }
    } else if pane.unit == "°" {
        let (scale, offset, unit) = quantity_policy.degree_axis_transform();
        y_axis.with_display_transform(scale, offset, unit)
    } else {
        y_axis
    };
    // The scale the pane is actually drawn on. Every hit test below maps
    // through it rather than assuming a linear ordinate, because "nearest"
    // has to mean nearest on screen — and on a decade pane a linear guess is
    // wrong by most of the window.
    let y_scale = if log_y { XScale::Log10 } else { XScale::Linear };
    // The plot's own description is where the caution has to live as words.
    // The kind tag carries it as colour and a glyph, and neither of those
    // reaches a reader who cannot see the strip.
    let mut spec = PlotSpec::new(x_axis, model.x_scale, y_axis)
        .accessible_name("Waveform plot")
        .without_x_axis_chrome()
        .with_right_margin(WAVE_SHARED_RIGHT_MARGIN);
    if let Some(reason) = model.incomplete {
        spec = spec.accessible_detail(reason);
    }
    spec.left_margin = input.left_margin;
    if log_y {
        spec = spec.with_log_y();
    }
    spec.display_decimation = input.display_decimation;
    spec.limit_lines = input.specification_limits;
    spec.minor_grid = input.minor_grid;
    spec.horizontal_cursor = input.horizontal_cursor;
    spec.horizontal_cursor_interactive = input.horizontal_cursor_interactive;
    spec.markers = input.markers;

    // 0 dB reference on a log-magnitude pane.
    if pane.unit == "dB" && y0 < 0.0 && y1 > 0.0 {
        spec.ref_lines.push(plot::RefLine { y: 0.0 });
    }

    // Family envelopes are derived only from exact shared X coordinates.
    // They draw behind source curves and never interpolate missing family
    // samples into evidence that was not retained.
    for envelope in input.family_envelopes {
        let mut minimum = Trace::new(&envelope.x, &envelope.minimum, envelope.color)
            .thin()
            .dashed()
            .cache_key(envelope.minimum_cache_key);
        let mut maximum = Trace::new(&envelope.x, &envelope.maximum, envelope.color)
            .thin()
            .dashed()
            .cache_key(envelope.maximum_cache_key);
        if envelope.x.len() == 1 {
            minimum = minimum.show_single_point();
            maximum = maximum.show_single_point();
        }
        spec.traces.push(minimum);
        spec.traces.push(maximum);
    }

    // Run owns weight: overlay traces keep the signal hue at reduced alpha
    // and stroke, painted first so the active run draws at full strength
    // on top.
    let pane_traces: Vec<(usize, &StripTrace)> = pane
        .traces
        .iter()
        .copied()
        .filter_map(|index| model.traces.get(index).map(|trace| (index, trace)))
        .collect();
    let draw_order = pane_traces
        .iter()
        .filter(|(_, trace)| trace.overlay)
        .chain(pane_traces.iter().filter(|(_, trace)| !trace.overlay));
    for (_, trace) in draw_order {
        let color = if trace.overlay {
            trace.color.gamma_multiply(0.40)
        } else {
            trace.color
        };
        // The reduction has to be told what the abscissa is. Without it a
        // reverse sweep vanished the moment it was zoomed — every window it
        // was asked for came back empty — and a hysteresis loop lost whichever
        // branch fell outside one contiguous index window.
        let mut plot_trace = apply_family_trace_style(
            Trace::new(&trace.x, &trace.y, color)
                .cache_key(trace_key(model, trace))
                .shape(&trace.shape),
            trace.family_style,
        );
        if trace.overlay {
            plot_trace = plot_trace.thin();
        }
        spec.traces.push(plot_trace);
    }
    for expr in exprs {
        spec.traces.push(apply_family_trace_style(
            Trace::new(&expr.x, &expr.y, expr.color)
                .thin()
                .cache_key(expr.cache_key)
                .shape(&expr.shape),
            expr.family_style,
        ));
    }

    let readout = |x: f64| -> Vec<(String, String)> {
        let mut rows = vec![(
            model.x_label().to_owned(),
            model.format_x(x, significant_digits, quantity_policy),
        )];
        let mut samples: Vec<BranchSample> = Vec::new();
        for (_, trace) in pane_traces.iter().take(6) {
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
                fmt_si_significant(value, "", significant_digits),
            ));
        }
        rows
    };

    let response = plot::show(ui, &spec, cache, input.cursors.as_ref(), Some(&readout));
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
