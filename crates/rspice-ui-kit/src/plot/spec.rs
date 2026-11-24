//! Plot specification — the declarative description a viewer hands to
//! [`super::render::show`].

use egui::Color32;

use super::format::{axis_tick_labels, offset_anchor_label, offset_tick_labels, tick_label};
use super::sample::SweepShape;
use super::scale::{
    LabelColumn, TickSeries, XScale, anchor_label, decade_ticks, fitted_linear_ticks, linear_ticks,
    linear_ticks_at,
};

/// One axis: range, tick positions/labels, and the unit shown at its end.
#[derive(Debug, Clone)]
pub struct Axis {
    /// Lower bound (data space).
    pub min: f64,
    /// Upper bound (data space).
    pub max: f64,
    /// Tick positions with preformatted labels.
    pub ticks: Vec<(f64, String)>,
    /// The axis' unit ("V", "dB", "Hz", "UI"), stated once: in the X row's
    /// gutter caption, or on the Y axis' top tick label.
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
                let deltas = self
                    .ticks
                    .iter()
                    .map(|(value, _)| (*value - anchor) * scale)
                    .collect::<Vec<_>>();
                let labels = offset_tick_labels(&deltas, display_step);
                for ((_, label), text) in self.ticks.iter_mut().zip(labels) {
                    *label = text;
                }
                self.offset_anchor = Some(offset_anchor_label(
                    display(anchor),
                    &self.unit,
                    display_step,
                ));
            }
            None => {
                let values = self
                    .ticks
                    .iter()
                    .map(|(value, _)| display(*value))
                    .collect::<Vec<_>>();
                let labels = axis_tick_labels(&values, display_step);
                for ((_, label), text) in self.ticks.iter_mut().zip(labels) {
                    *label = text;
                }
            }
        }
        self
    }

    /// This axis as a column of labels `column` long reads it: an even
    /// linear ladder too dense for every label to stand clear coarsens along
    /// 1–2–5 until each does, so a short plot keeps even steps and names
    /// every tick rather than dropping a label from the middle of its scale.
    /// `None` when the axis already fits, or when its ticks are not a ladder
    /// it made itself (decades, an explicit list, labels written by hand).
    #[must_use]
    pub fn fitted(&self, column: LabelColumn) -> Option<Self> {
        if !(self.tick_step > 0.0 && self.max > self.min) {
            return None;
        }
        let fraction = |value: f64| (value - self.min) / (self.max - self.min);
        if column.holds(self.ticks.iter().map(|(value, _)| fraction(*value))) {
            return None;
        }
        let rebuilt = |series: TickSeries| {
            let mut axis = Self::from_series(self.min, self.max, self.unit.clone(), series);
            if self.display_scale != 1.0 || self.display_offset != 0.0 {
                axis = axis.with_display_transform(
                    self.display_scale,
                    self.display_offset,
                    self.unit.clone(),
                );
            }
            axis.label.clone_from(&self.label);
            axis
        };
        if rebuilt(linear_ticks_at(self.min, self.max, self.tick_step)).ticks != self.ticks {
            return None;
        }
        let fitted = rebuilt(fitted_linear_ticks(
            self.min,
            self.max,
            self.tick_step,
            column,
        ));
        (fitted.ticks != self.ticks).then_some(fitted)
    }

    /// Attach the exact semantic quantity label rendered with the unit and
    /// included in plot accessibility text.
    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        let label = label.into();
        self.label = (!label.trim().is_empty()).then_some(label);
        self
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
    /// The slot of the canvas palette this trace's colour came from. A
    /// trace past the palette's last hue repeats a colour, and its slot is
    /// what gives it a dash of its own; see [`Trace::look`].
    pub palette_slot: Option<usize>,
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
    /// The samples, when a results store keeps them: `x` and `y` are then empty, and the trace
    /// is drawn from the store's tiles at the view's width without reading either column.
    pub stored: Option<StoredSamples<'a>>,
    /// The shared columns `x` and `y` are borrowed from, when they are: what
    /// lets the plot reduce this curve off the frame when drawing it costs
    /// more than a frame. A curve without them is reduced on the frame.
    pub shared: Option<StoredSamples<'a>>,
    /// The Y axis the trace reads against when it is not the plot's own: a
    /// second axis of the strip, with a range and scale of its own. `None`
    /// reads against [`PlotSpec::y`].
    pub y_axis: Option<TraceAxis>,
    /// How the samples are drawn: joined, as points, as steps or as stems.
    pub draw: DrawMode,
    /// The dash the reader chose, `Some(None)` for a solid line: every cue
    /// gives way to it.
    pub chosen_dash: Option<Option<(f32, f32)>>,
    /// The symbol the reader chose, `Some(None)` for none: every cue gives
    /// way to it.
    pub chosen_marker: Option<Option<super::render::TraceMarkerShape>>,
}

/// A Y axis a trace reads against beside the plot's own: its range and
/// scale. The plot draws no ticks for it; its owner draws its gutter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TraceAxis {
    pub min: f64,
    pub max: f64,
    pub scale: XScale,
}

/// How a trace's samples are drawn.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum DrawMode {
    /// Each sample joined to the next.
    #[default]
    Line,
    /// Each sample a dot, unjoined.
    Points,
    /// Each sample held until the next: a sample-and-hold staircase.
    Steps,
    /// Each sample a stem from zero, or the axis' bottom when zero is out of
    /// view, to its value.
    Stems,
}

/// A trace's samples as windows of the columns of a results store: its run's axis and the
/// signal's values over the same rows.
#[derive(Debug, Clone, Copy)]
pub struct StoredSamples<'a> {
    pub x: &'a rspice_results::waveform::WaveformValues,
    pub y: &'a rspice_results::waveform::WaveformValues,
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
            palette_slot: None,
            show_single_point: false,
            width: 1.8,
            cache_key: None,
            parametric: false,
            shape: None,
            stored: None,
            shared: None,
            y_axis: None,
            draw: DrawMode::Line,
            chosen_dash: None,
            chosen_marker: None,
        }
    }

    /// A solid 1.8 pt trace of samples a results store keeps, drawn from its tiles. Samples the
    /// store cannot tile (held in memory, or on a scale its tiles do not follow) are drawn as
    /// [`Self::new`] draws them.
    pub fn stored(
        x: &'a rspice_results::waveform::WaveformValues,
        y: &'a rspice_results::waveform::WaveformValues,
        color: Color32,
    ) -> Self {
        Self {
            stored: Some(StoredSamples { x, y }),
            ..Self::new(&[], &[], color)
        }
    }

    /// Whether the trace has no samples to draw.
    pub fn is_empty(&self) -> bool {
        match self.stored {
            Some(stored) => stored.x.is_empty() || stored.y.is_empty(),
            None => self.x.is_empty() || self.y.is_empty(),
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

    /// Place this trace in the canvas palette's cycle at `slot`, the slot its
    /// colour was taken from.
    pub fn palette_slot(mut self, slot: usize) -> Self {
        self.palette_slot = Some(slot);
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

    /// Read against `axis` rather than the plot's own Y axis.
    pub fn on_axis(mut self, axis: TraceAxis) -> Self {
        self.y_axis = Some(axis);
        self
    }

    /// Draw the samples as `mode` draws them.
    pub fn draw(mut self, mode: DrawMode) -> Self {
        self.draw = mode;
        self
    }

    /// Draw in the dash and symbol the reader chose, where they chose one,
    /// whatever the trace's cues would give it.
    pub fn chosen_look(
        mut self,
        dash: Option<Option<(f32, f32)>>,
        marker: Option<Option<super::render::TraceMarkerShape>>,
    ) -> Self {
        self.chosen_dash = dash;
        self.chosen_marker = marker;
        self
    }

    /// Set the decimation-cache identity (globally unique per trace).
    pub fn cache_key(mut self, key: u64) -> Self {
        self.cache_key = Some(key);
        self
    }

    /// Name the shared columns `x` and `y` are borrowed from, so the plot
    /// can reduce this curve off the frame.
    pub fn shared(
        mut self,
        x: &'a rspice_results::waveform::WaveformValues,
        y: &'a rspice_results::waveform::WaveformValues,
    ) -> Self {
        self.shared = Some(StoredSamples { x, y });
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
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum MarkerShape {
    /// A hollow dot sitting on the curve at (`x`, `y`), tag beside it.
    #[default]
    Point,
    /// A dashed limit line spanning the plot height at `x`, tag at the top.
    /// `y` is ignored — the callout is about the X position alone.
    LimitLine,
    /// A distance read at `x`: a capped vertical bar from `y` to `to`, tag
    /// beside its middle (a loop's gain or phase margin, from the curve to
    /// the level it is measured against).
    Span { to: f64 },
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

    /// A capped vertical span at `x` from `from` to `to`, tagged beside it.
    pub fn span(x: f64, from: f64, to: f64, color: Color32, label: impl Into<String>) -> Self {
        Self {
            shape: MarkerShape::Span { to },
            ..Self::point(x, from, color, label)
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

/// The left gutter every plot keeps at least: room for the Y tick labels
/// and, under them, the X row's caption.
pub const PLOT_GUTTER: f32 = 64.0;

/// The share of its span a fit leaves free on each side: enough that no
/// curve runs along the frame, little enough that the curves fill the plot,
/// as ViVA's and EZwave's fits leave them.
pub const FIT_HEADROOM: f64 = 0.04;

/// A linear fit of the values spanning `min..max`, padded by
/// [`FIT_HEADROOM`] each side.
///
/// A span that cannot survive its own padding is flat as far as any axis
/// can draw it: exactly constant, or constant to within a few ulps. Both
/// widen by one either side, because a range finer than the tick ladder can
/// subdivide leaves the plot with no labels at all.
#[must_use]
pub fn padded_fit((min, max): (f64, f64)) -> (f64, f64) {
    let pad = (max - min) * FIT_HEADROOM;
    if pad <= 0.0 {
        return (min - 1.0, max + 1.0);
    }
    (min - pad, max + pad)
}

/// Custom drawing under the traces (histogram bars, eye acquisitions).
/// Receives a painter clipped to the plot area and the data→screen mapper.
pub type Underlay<'a> = Box<dyn Fn(&egui::Painter, &PlotMapper) + 'a>;

/// Curves a plot makes only when it draws a new picture of them. A pane of a
/// hundred thousand curves names them all by one key, and a frame that shows
/// the picture it holds of them never lists them.
pub struct Curves<'a> {
    /// Changes whenever a curve, its samples or how it is drawn would:
    /// what each curve's identity and look say for curves listed one by one.
    pub key: u64,
    /// How many curves draw something.
    pub count: usize,
    /// The curves, drawn after [`PlotSpec::traces`], in order.
    pub make: Box<dyn Fn() -> Vec<Trace<'a>> + 'a>,
}

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
    /// Curves drawn after `traces` and made only when the plot draws them
    /// afresh. Each must carry its cache key.
    pub curves: Option<Curves<'a>>,
    /// Analysis markers, drawn above traces.
    pub markers: Vec<Marker>,
    /// Horizontal reference lines.
    pub ref_lines: Vec<RefLine>,
    /// Vertical reference lines, at X positions, drawn as the horizontal
    /// ones are: a complex plane's imaginary axis reads as its real axis
    /// does, not as one more grid line.
    pub x_ref_lines: Vec<f64>,
    /// Exact labeled limits from an owning project contract.
    pub limit_lines: Vec<LimitLine>,
    /// Draw unlabeled subdivisions between major ticks.
    pub minor_grid: bool,
    /// The ink the Y tick labels and unit are set in instead of the
    /// canvas' dim text: the colour of the only curves the axis carries,
    /// when the plot reads several axes and each says whose it is.
    pub y_ink: Option<Color32>,
    /// Horizontal measurement cursors H1 and H2 on the left Y axis.
    pub horizontal_cursors: [Option<f64>; 2],
    /// Whether primary click/drag places or moves a horizontal cursor.
    pub horizontal_cursor_interactive: bool,
    /// Shaded vertical bands, drawn under everything.
    pub bands: Vec<Band>,
    /// Custom content drawn after the grid, under the traces.
    pub underlay: Option<Underlay<'a>>,
    /// The least left gutter, in points: the waveform stack's, so a plot
    /// standing on its own keeps the stack's left edge. The renderer widens
    /// it to the widest Y label, its unit included, so no label is clipped.
    pub left_margin: f32,
    /// Right margin in points. `None` selects the renderer default. A stack
    /// uses one explicit margin so every pane's X grid stays aligned.
    pub right_margin: Option<f32>,
    /// Whether this plot draws its own X axis row under it (see
    /// [`super::paint_x_axis_row`]). The strips of a waveform stack leave it
    /// to the one ruler under the stack, which paints the same row.
    pub x_axis_chrome: bool,
    /// Whether the left and bottom axis lines are drawn. A chart read on its
    /// own grid (a Smith chart, a polar disc) has no Cartesian axes for them
    /// to stand for.
    pub frame: bool,
    /// What the Y axis says at the head of its column when it has no unit
    /// to put on its top tick label ("unit unknown"). Left out, such an axis
    /// goes unmarked, as a plain number's axis does in ViVA and ADS.
    pub y_without_unit: Option<String>,
    /// Whether a trackpad pinch zooms both axes about the pointer, as on a
    /// chart read on its own grid (Smith, Nyquist, pole-zero, scatter),
    /// rather than X alone, as on a waveform read against time.
    pub pinch_zooms_both_axes: bool,
    /// The significant digits a value the plot prints itself (a horizontal
    /// cursor's level, the difference between two) is stated in.
    pub significant_digits: usize,
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
            curves: None,
            markers: Vec::new(),
            ref_lines: Vec::new(),
            x_ref_lines: Vec::new(),
            limit_lines: Vec::new(),
            minor_grid: false,
            y_ink: None,
            horizontal_cursors: [None; 2],
            horizontal_cursor_interactive: false,
            bands: Vec::new(),
            underlay: None,
            left_margin: PLOT_GUTTER,
            right_margin: None,
            x_axis_chrome: true,
            frame: true,
            y_without_unit: None,
            pinch_zooms_both_axes: false,
            significant_digits: 4,
        }
    }

    /// Draw no left or bottom axis line: the chart carries its own grid.
    #[must_use]
    pub fn without_frame(mut self) -> Self {
        self.frame = false;
        self
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

    /// What the Y axis says at the head of its column without a unit: see
    /// [`Self::y_without_unit`].
    #[must_use]
    pub fn with_y_without_unit(mut self, text: impl Into<String>) -> Self {
        self.y_without_unit = Some(text.into());
        self
    }

    /// Let a trackpad pinch zoom both axes about the pointer.
    #[must_use]
    pub fn pinch_zooms_both_axes(mut self) -> Self {
        self.pinch_zooms_both_axes = true;
        self
    }

    /// State the values the plot prints itself in `digits` significant
    /// digits.
    #[must_use]
    pub fn with_significant_digits(mut self, digits: usize) -> Self {
        self.significant_digits = digits.max(1);
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
