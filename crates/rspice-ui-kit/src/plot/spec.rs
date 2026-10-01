//! Plot specification — the declarative description a viewer hands to
//! [`super::render::show`].

use egui::Color32;

use super::decimate::DisplayDecimation;
use super::format::{offset_anchor_label, tick_label, tick_label_with_step, tick_offset_label};
use super::sample::SweepShape;
use super::scale::{TickSeries, XScale, anchor_label, decade_ticks, linear_ticks};

/// One axis: range, tick positions/labels, and the unit shown at its end.
#[derive(Debug, Clone)]
pub struct Axis {
    /// Lower bound (data space).
    pub min: f64,
    /// Upper bound (data space).
    pub max: f64,
    /// Tick positions with preformatted labels.
    pub ticks: Vec<(f64, String)>,
    /// Unit label drawn once at the axis end ("V", "dB", "Hz", "UI").
    pub unit: String,
    /// Semantic quantity label (for example a swept device parameter name, or
    /// `Temperature`).
    /// Ordinary viewers may omit it; family projections retain the exact
    /// manifest label independently from the engineering unit.
    pub label: Option<String>,
    /// Affine presentation transform applied only to labels/accessibility.
    /// Geometry and stored samples remain in the axis' canonical data space.
    pub display_scale: f64,
    pub display_offset: f64,
    /// Even tick spacing in canonical data space; zero when the ticks are
    /// explicit or logarithmic. Kept so a presentation transform can relabel
    /// at the precision the spacing calls for.
    pub(super) tick_step: f64,
    /// Canonical value the tick labels are stated as offsets from.
    pub(super) anchor: Option<f64>,
    /// The anchor as chrome: rendered once beside the axis, so the ticks can
    /// carry only what differs between them.
    pub(super) offset_anchor: Option<String>,
}

impl Axis {
    fn from_series(min: f64, max: f64, unit: impl Into<String>, series: TickSeries) -> Self {
        let unit = unit.into();
        let offset_anchor = anchor_label(&series, &unit);
        Self {
            min,
            max,
            ticks: series.ticks,
            unit,
            label: None,
            display_scale: 1.0,
            display_offset: 0.0,
            tick_step: series.step,
            anchor: series.anchor,
            offset_anchor,
        }
    }

    /// Linear axis with nice 1–2–5 ticks.
    pub fn linear(min: f64, max: f64, unit: impl Into<String>) -> Self {
        Self::from_series(min, max, unit, linear_ticks(min, max, 6))
    }

    /// Linear axis with a target tick count.
    pub fn linear_with(min: f64, max: f64, unit: impl Into<String>, target: usize) -> Self {
        Self::from_series(min, max, unit, linear_ticks(min, max, target))
    }

    /// Log-frequency axis with decade ticks.
    pub fn log_decades(min: f64, max: f64, unit: impl Into<String>) -> Self {
        Self::from_series(min, max, unit, decade_ticks(min, max))
    }

    /// Axis with explicit tick positions (labels generated).
    pub fn with_ticks(min: f64, max: f64, unit: impl Into<String>, ticks: &[f64]) -> Self {
        Self::from_series(
            min,
            max,
            unit,
            TickSeries {
                ticks: ticks.iter().map(|&v| (v, tick_label(v))).collect(),
                anchor: None,
                step: 0.0,
            },
        )
    }

    /// Present canonical data through an affine conversion without changing
    /// the plot geometry. This keeps zoom/cursor coordinates and stored data
    /// exact while allowing Hz→rad/s, °C→K/°F, and degree→radian labels.
    #[must_use]
    pub fn with_display_transform(
        mut self,
        scale: f64,
        offset: f64,
        unit: impl Into<String>,
    ) -> Self {
        debug_assert!(scale.is_finite() && scale != 0.0);
        debug_assert!(offset.is_finite());
        self.display_scale = scale;
        self.display_offset = offset;
        self.unit = unit.into();
        let display = |value: f64| value.mul_add(scale, offset);
        let display_step = self.tick_step * scale.abs();
        match self.anchor {
            // Offsets are differences, so the transform's offset cancels out
            // of every tick and appears only in the anchor.
            Some(anchor) => {
                for (value, label) in &mut self.ticks {
                    *label = tick_offset_label((*value - anchor) * scale, display_step);
                }
                self.offset_anchor = Some(offset_anchor_label(
                    display(anchor),
                    &self.unit,
                    display_step,
                ));
            }
            None => {
                for (value, label) in &mut self.ticks {
                    *label = tick_label_with_step(display(*value), display_step);
                }
            }
        }
        self
    }

    /// Attach the exact semantic quantity label rendered with the unit and
    /// included in plot accessibility text.
    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        let label = label.into();
        self.label = (!label.trim().is_empty()).then_some(label);
        self
    }

    #[must_use]
    pub fn end_label(&self) -> String {
        match (self.label.as_deref(), self.unit.as_str()) {
            (Some(label), "") => label.to_owned(),
            (Some(label), unit) => format!("{label} · {unit}"),
            (None, unit) => unit.to_owned(),
        }
    }

    /// The value this axis' tick labels are stated as offsets from, already
    /// formatted with its unit. `None` — the ordinary case — means the labels
    /// are absolute.
    ///
    /// A surface that renders its own tick row instead of letting the plot
    /// draw one (a shared axis strip beneath a stack of panes) has to draw
    /// this beside that row: without it a row of offsets reads as a row of
    /// absolute values, which is a different number.
    #[must_use]
    pub fn offset_anchor(&self) -> Option<&str> {
        self.offset_anchor.as_deref()
    }

    #[must_use]
    pub fn display_value(&self, canonical: f64) -> f64 {
        canonical.mul_add(self.display_scale, self.display_offset)
    }

    #[must_use]
    pub fn format_display_value(&self, canonical: f64) -> String {
        tick_label(self.display_value(canonical))
    }

    #[must_use]
    pub fn format_display_delta(&self, canonical_delta: f64) -> String {
        tick_label(canonical_delta * self.display_scale)
    }
}

/// One trace. Data is borrowed; the engine never copies it except into the
/// cached decimation envelope.
pub struct Trace<'a> {
    /// X samples, monotonically non-decreasing.
    pub x: &'a [f64],
    /// Y samples, same length as `x`.
    pub y: &'a [f64],
    /// Stroke color.
    pub color: Color32,
    /// Dashed stroke (secondary quantities — fit overlays).
    pub dashed: bool,
    /// Explicit categorical dash cue. Unlike the global color-safe display
    /// preference, this is part of a persisted family presentation policy.
    pub dash_style: Option<usize>,
    /// Explicit categorical marker cue from a family presentation policy.
    pub marker_style: Option<usize>,
    /// Paint a neutral point primitive when this trace contains one sample.
    /// This preserves exact isolated family points without assigning a
    /// categorical marker meaning that the policy did not request.
    pub show_single_point: bool,
    /// Stroke width in points (design: 1.8 primary, 1.4 thin).
    pub width: f32,
    /// Stable identity for the decimation cache. Must be globally unique
    /// per trace — the renderer mixes nothing in, so two traces sharing a
    /// key would serve each other's envelopes. `None` disables caching.
    pub cache_key: Option<u64>,
    /// This curve is a locus: X is a coordinate, not an ordering.
    ///
    /// Smith and Nyquist sweep a path that revisits X values, so the
    /// X-ordered reductions cannot describe them and silently drop whichever
    /// branch falls outside their contiguous index window. Set this and the
    /// renderer reduces in source order instead. It is a fact about the
    /// data, never about the user's display preference.
    pub parametric: bool,
    /// What this trace's X column actually is, when the owner has already
    /// classified it.
    ///
    /// A reverse sweep and a hysteresis loop are both ordinary results and
    /// neither is a locus, but both break an ascending reduction. Declaring
    /// the shape routes the trace to a reduction that can describe it. `None`
    /// reads as ascending, which is what every caller meant before shapes
    /// existed. Classifying is O(n), so it belongs in the owner's cache, not
    /// in the per-frame render path.
    pub shape: Option<&'a SweepShape>,
}

impl<'a> Trace<'a> {
    /// A solid 1.8 pt trace.
    pub fn new(x: &'a [f64], y: &'a [f64], color: Color32) -> Self {
        Self {
            x,
            y,
            color,
            dashed: false,
            dash_style: None,
            marker_style: None,
            show_single_point: false,
            width: 1.8,
            cache_key: None,
            parametric: false,
            shape: None,
        }
    }

    /// Dashed stroke.
    pub fn dashed(mut self) -> Self {
        self.dashed = true;
        self
    }

    /// Apply one deterministic categorical dash pattern.
    pub fn dash_style(mut self, ordinal: usize) -> Self {
        self.dash_style = Some(ordinal);
        self
    }

    /// Apply one deterministic categorical point-marker shape.
    pub fn marker_style(mut self, ordinal: usize) -> Self {
        self.marker_style = Some(ordinal);
        self
    }

    /// Keep a one-sample trace visible using a neutral circle marker.
    pub fn show_single_point(mut self) -> Self {
        self.show_single_point = true;
        self
    }

    /// Set an explicit policy-controlled stroke width.
    pub fn width(mut self, points: f32) -> Self {
        self.width = points;
        self
    }

    /// Thin (1.4 pt) stroke.
    pub fn thin(mut self) -> Self {
        self.width = 1.4;
        self
    }

    /// Set the decimation-cache identity (globally unique per trace).
    pub fn cache_key(mut self, key: u64) -> Self {
        self.cache_key = Some(key);
        self
    }

    /// Declare this curve a locus, so it reduces in source order.
    pub fn parametric(mut self) -> Self {
        self.parametric = true;
        self
    }

    /// Declare the monotone structure of this trace's X column, so the
    /// renderer picks a reduction that can describe it.
    #[must_use]
    pub fn shape(mut self, shape: &'a SweepShape) -> Self {
        self.shape = Some(shape);
        self
    }
}

/// Compose a per-trace decimation cache key from a viewer-scoped base and a
/// trace ordinal.
///
/// The ordinal moves into bits the base cannot occupy. Folding it in with a
/// bitwise OR — the obvious spelling — silently aliases as soon as the ordinal
/// reaches a bit the base already sets, and two traces sharing a key serve
/// each other's envelopes: one curve drawn in another's place, with nothing to
/// show that anything went wrong.
#[must_use]
pub fn trace_cache_key(base: u64, index: usize) -> u64 {
    (base & 0xFFFF_FFFF) | ((index as u64) << 32)
}

/// How a [`Marker`] draws.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarkerShape {
    /// A hollow dot sitting on the curve at (`x`, `y`), tag beside it.
    #[default]
    Point,
    /// A dashed limit line spanning the plot height at `x`, tag at the top.
    /// `y` is ignored — the callout is about the X position alone.
    LimitLine,
}

/// An analysis marker: a hollow dot on the curve plus a bordered tag, with
/// an optional drop line to the X axis (UGF, HDn, µ, LSL …).
#[derive(Debug, Clone)]
pub struct Marker {
    /// Position (data space).
    pub x: f64,
    /// Position (data space).
    pub y: f64,
    /// Dot ring and tag text color.
    pub color: Color32,
    /// Tag text ("UGF 10.4 MHz").
    pub label: String,
    /// Draw a dashed drop line from the dot to the X axis.
    pub drop_line: bool,
    /// Vertical tag offset in points (to stagger near-colliding tags).
    pub label_dy: f32,
    /// Geometry. Defaults to [`MarkerShape::Point`].
    pub shape: MarkerShape,
}

impl Marker {
    /// A dot-and-tag marker.
    pub fn point(x: f64, y: f64, color: Color32, label: impl Into<String>) -> Self {
        Self {
            x,
            y,
            color,
            label: label.into(),
            drop_line: false,
            label_dy: 0.0,
            shape: MarkerShape::Point,
        }
    }

    /// A dashed vertical limit line at `x`, tagged at the top of the plot.
    pub fn limit_line(x: f64, color: Color32, label: impl Into<String>) -> Self {
        Self {
            shape: MarkerShape::LimitLine,
            ..Self::point(x, 0.0, color, label)
        }
    }
}

/// A horizontal dashed reference line (0 dB, thresholds).
#[derive(Debug, Clone, Copy)]
pub struct RefLine {
    /// Y position on the left axis.
    pub y: f64,
}

/// A labeled horizontal project limit on the left Y axis.
#[derive(Debug, Clone)]
pub struct LimitLine {
    pub y: f64,
    pub color: Color32,
    pub label: String,
}

/// A shaded vertical band (±σ ranges).
#[derive(Debug, Clone, Copy)]
pub struct Band {
    /// Band start (data space).
    pub x0: f64,
    /// Band end (data space).
    pub x1: f64,
}

/// Data→screen mapping handed to [`PlotSpec::underlay`] hooks.
pub struct PlotMapper {
    /// The inner plot rectangle.
    pub rect: egui::Rect,
    pub(super) x0: f64,
    pub(super) x1: f64,
    pub(super) x_scale: XScale,
    pub(super) y0: f64,
    pub(super) y1: f64,
}

impl PlotMapper {
    /// Data X → screen X.
    pub fn x(&self, x: f64) -> f32 {
        self.rect.left() + (self.x_scale.normalize(x, self.x0, self.x1) as f32) * self.rect.width()
    }

    /// Data Y (left axis) → screen Y.
    pub fn y(&self, y: f64) -> f32 {
        self.rect.bottom() - (((y - self.y0) / (self.y1 - self.y0)) as f32) * self.rect.height()
    }
}

/// Custom drawing under the traces (histogram bars, eye acquisitions).
/// Receives a painter clipped to the plot area and the data→screen mapper.
pub type Underlay<'a> = Box<dyn Fn(&egui::Painter, &PlotMapper) + 'a>;

/// The full declarative plot description for one frame.
pub struct PlotSpec<'a> {
    /// Concise screen-reader name for this engineering graphic.
    pub accessible_name: &'a str,
    /// Optional viewer-owned description for data painted by a custom
    /// underlay rather than by [`Trace`] entries (for example eye-density
    /// acquisitions and a compliance mask).
    pub accessible_detail: Option<&'a str>,
    /// X axis.
    pub x: Axis,
    /// X scale (linear or log decades).
    pub x_scale: XScale,
    /// Y axis.
    pub y: Axis,
    /// Y scale. Linear by default; unit-pane controls may opt into
    /// logarithmic decades only for strictly positive source data.
    pub y_scale: XScale,
    /// Traces, drawn in order (first = bottom).
    pub traces: Vec<Trace<'a>>,
    /// Viewer-only trace sampling policy.
    pub display_decimation: DisplayDecimation,
    /// Analysis markers, drawn above traces.
    pub markers: Vec<Marker>,
    /// Horizontal reference lines.
    pub ref_lines: Vec<RefLine>,
    /// Exact labeled limits from an owning project contract.
    pub limit_lines: Vec<LimitLine>,
    /// Draw unlabeled subdivisions between major ticks.
    pub minor_grid: bool,
    /// Horizontal measurement cursor on the left Y axis.
    pub horizontal_cursor: Option<f64>,
    /// Whether primary click/drag updates `horizontal_cursor`.
    pub horizontal_cursor_interactive: bool,
    /// Shaded vertical bands, drawn under everything.
    pub bands: Vec<Band>,
    /// Custom content drawn after the grid, under the traces.
    pub underlay: Option<Underlay<'a>>,
    /// Left margin in points (56 default; widen for long tick labels).
    pub left_margin: f32,
    /// Right margin in points. `None` selects the renderer default. A stack
    /// uses one explicit margin so every pane's X grid stays aligned.
    pub right_margin: Option<f32>,
    /// Whether this plot owns labeled X-axis chrome. Unit-scoped waveform
    /// panes disable it and render one shared X strip beneath the stack.
    pub x_axis_chrome: bool,
}

impl<'a> PlotSpec<'a> {
    /// A plot with the given axes and defaults for everything else.
    pub fn new(x: Axis, x_scale: XScale, y: Axis) -> Self {
        Self {
            accessible_name: "Simulation results plot",
            accessible_detail: None,
            x,
            x_scale,
            y,
            y_scale: XScale::Linear,
            traces: Vec::new(),
            display_decimation: DisplayDecimation::default(),
            markers: Vec::new(),
            ref_lines: Vec::new(),
            limit_lines: Vec::new(),
            minor_grid: false,
            horizontal_cursor: None,
            horizontal_cursor_interactive: false,
            bands: Vec::new(),
            underlay: None,
            left_margin: 56.0,
            right_margin: None,
            x_axis_chrome: true,
        }
    }

    /// Set the screen-reader name for this engineering graphic.
    pub fn accessible_name(mut self, name: &'a str) -> Self {
        self.accessible_name = name;
        self
    }

    /// Describe meaningful engineering data rendered by a custom underlay.
    #[must_use]
    pub fn accessible_detail(mut self, detail: &'a str) -> Self {
        self.accessible_detail = (!detail.trim().is_empty()).then_some(detail);
        self
    }

    /// Keep X geometry, grids, cursors, and interactions while suppressing
    /// the repeated labels owned by a shared axis strip.
    #[must_use]
    pub fn without_x_axis_chrome(mut self) -> Self {
        self.x_axis_chrome = false;
        self
    }

    /// Reserve an explicit right margin for alignment with sibling plots.
    #[must_use]
    pub fn with_right_margin(mut self, points: f32) -> Self {
        self.right_margin = points.is_finite().then_some(points.max(0.0));
        self
    }

    /// Render and navigate the left Y axis in logarithmic decades.
    #[must_use]
    pub fn with_log_y(mut self) -> Self {
        self.y_scale = XScale::Log10;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A viewer composing a per-trace cache key from a viewer-scoped base and
    /// a trace ordinal must not fold the ordinal into bits the base already
    /// occupies: two traces sharing a key serve each other's envelopes, which
    /// draws one curve in another's place. Smith's base ends in 0xF0, so the
    /// bitwise OR it used aliased from the seventeenth trace onwards — well
    /// inside a four-port sheet.
    #[test]
    fn trace_cache_keys_stay_distinct_past_sixteen_traces() {
        let keys = (0..64usize)
            .map(|index| trace_cache_key(0x501_00F0, index))
            .collect::<std::collections::HashSet<_>>();

        assert_eq!(keys.len(), 64, "trace cache keys aliased");
    }

    /// Two viewers must not collide either, at any ordinal.
    #[test]
    fn trace_cache_keys_separate_their_viewers() {
        let smith = (0..32usize).map(|index| trace_cache_key(0x501_00F0, index));
        let nyquist = (0..32usize)
            .map(|index| trace_cache_key(0x419_0000, index))
            .collect::<std::collections::HashSet<_>>();

        assert!(smith.into_iter().all(|key| !nyquist.contains(&key)));
    }
}
