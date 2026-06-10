//! Plot specification — the declarative description a viewer hands to
//! [`super::render::show`].

use egui::Color32;

use super::format::tick_label;
use super::scale::{XScale, decade_ticks, linear_ticks};

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
    pub unit: &'static str,
}

impl Axis {
    /// Linear axis with nice 1–2–5 ticks.
    pub fn linear(min: f64, max: f64, unit: &'static str) -> Self {
        Self {
            min,
            max,
            ticks: linear_ticks(min, max, 6),
            unit,
        }
    }

    /// Linear axis with a target tick count.
    pub fn linear_with(min: f64, max: f64, unit: &'static str, target: usize) -> Self {
        Self {
            min,
            max,
            ticks: linear_ticks(min, max, target),
            unit,
        }
    }

    /// Log-frequency axis with decade ticks.
    pub fn log_decades(min: f64, max: f64, unit: &'static str) -> Self {
        Self {
            min,
            max,
            ticks: decade_ticks(min, max),
            unit,
        }
    }

    /// Axis with explicit tick positions (labels generated).
    pub fn with_ticks(min: f64, max: f64, unit: &'static str, ticks: &[f64]) -> Self {
        Self {
            min,
            max,
            ticks: ticks.iter().map(|&v| (v, tick_label(v))).collect(),
            unit,
        }
    }
}

/// Which Y axis a trace or marker is mapped to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum YSide {
    /// Primary (left) axis.
    #[default]
    Left,
    /// Secondary (right) axis — gain/phase plots.
    Right,
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
    /// Axis mapping.
    pub side: YSide,
    /// Dashed stroke (secondary quantities — phase, fit overlays).
    pub dashed: bool,
    /// Stroke width in points (design: 1.8 primary, 1.4 thin).
    pub width: f32,
    /// Stable identity for the decimation cache. Combine the data version
    /// with a per-trace index; `None` disables caching for this trace.
    pub cache_key: Option<u64>,
}

impl<'a> Trace<'a> {
    /// A solid 1.8 pt trace on the left axis.
    pub fn new(x: &'a [f64], y: &'a [f64], color: Color32) -> Self {
        Self {
            x,
            y,
            color,
            side: YSide::Left,
            dashed: false,
            width: 1.8,
            cache_key: None,
        }
    }

    /// Map to the right axis.
    pub fn right(mut self) -> Self {
        self.side = YSide::Right;
        self
    }

    /// Dashed stroke.
    pub fn dashed(mut self) -> Self {
        self.dashed = true;
        self
    }

    /// Thin (1.4 pt) stroke.
    pub fn thin(mut self) -> Self {
        self.width = 1.4;
        self
    }

    /// Set the decimation-cache identity.
    pub fn cache_key(mut self, key: u64) -> Self {
        self.cache_key = Some(key);
        self
    }
}

/// An analysis marker: a hollow dot on the curve plus a bordered tag, with
/// an optional drop line to the X axis (UGF, HDn, µ, LSL …).
#[derive(Debug, Clone)]
pub struct Marker {
    /// Position (data space).
    pub x: f64,
    /// Position (data space, on `side`'s axis).
    pub y: f64,
    /// Axis mapping for `y`.
    pub side: YSide,
    /// Dot ring and tag text color.
    pub color: Color32,
    /// Tag text ("UGF 10.4 MHz").
    pub label: String,
    /// Draw a dashed drop line from the dot to the X axis.
    pub drop_line: bool,
    /// Vertical tag offset in points (to stagger near-colliding tags).
    pub label_dy: f32,
}

/// A horizontal dashed reference line (0 dB, thresholds).
#[derive(Debug, Clone, Copy)]
pub struct RefLine {
    /// Y position on the left axis.
    pub y: f64,
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
        self.rect.left()
            + (self.x_scale.normalize(x, self.x0, self.x1) as f32) * self.rect.width()
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
    /// X axis.
    pub x: Axis,
    /// X scale (linear or log decades).
    pub x_scale: XScale,
    /// Left Y axis.
    pub y: Axis,
    /// Optional right Y axis with its tick/unit tint.
    pub y_right: Option<(Axis, Color32)>,
    /// Traces, drawn in order (first = bottom).
    pub traces: Vec<Trace<'a>>,
    /// Analysis markers, drawn above traces.
    pub markers: Vec<Marker>,
    /// Horizontal reference lines.
    pub ref_lines: Vec<RefLine>,
    /// Shaded vertical bands, drawn under everything.
    pub bands: Vec<Band>,
    /// Custom content drawn after the grid, under the traces.
    pub underlay: Option<Underlay<'a>>,
    /// Left margin in points (56 default; widen for long tick labels).
    pub left_margin: f32,
}

impl<'a> PlotSpec<'a> {
    /// A plot with the given axes and defaults for everything else.
    pub fn new(x: Axis, x_scale: XScale, y: Axis) -> Self {
        Self {
            x,
            x_scale,
            y,
            y_right: None,
            traces: Vec::new(),
            markers: Vec::new(),
            ref_lines: Vec::new(),
            bands: Vec::new(),
            underlay: None,
            left_margin: 56.0,
        }
    }
}
