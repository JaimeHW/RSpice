//! Eye compliance mask, stored in absolute units.
//!
//! Mask geometry is stored as (seconds, volts) together with the data rate
//! and signal levels it was authored against, so a saved mask no longer
//! distorts silently when the data rate or swing of the measured eye
//! changes. Legacy masks — normalized to the eye window and swing — are
//! migrated on deserialization (see the `Deserialize` impl).

use serde::{Deserialize, Serialize};

/// Nominal authoring context shared by the built-in default mask and the
/// legacy migration: matches `EyeData::default()` (1 Gb/s, 0.8 V swing
/// centered on 0 V) and the legacy 2-UI eye window every normalized mask
/// was defined against.
const NOMINAL_DATA_RATE: f64 = 1e9;
const NOMINAL_SWING: f64 = 0.8;
const NOMINAL_V_CROSS: f64 = 0.0;
const LEGACY_WINDOW_UI: f64 = 2.0;

/// Eye mask for compliance testing.
///
/// Geometry is absolute: polygon points are (time in seconds, voltage in
/// volts). Time is measured from the start of the eye window at
/// `reference_data_rate` — one unit interval is `1 / reference_data_rate`
/// seconds — and voltages are plain volts on the eye's vertical axis.
#[derive(Debug, Clone, Serialize)]
pub struct EyeMask {
    /// Mask is enabled
    pub enabled: bool,
    /// Mask name (e.g., "100GBASE-KR4")
    pub name: String,
    /// Inner polygon (forbidden region), absolute (seconds, volts)
    pub inner: MaskPolygon,
    /// Outer polygon (boundary), absolute (seconds, volts)
    pub outer: Option<MaskPolygon>,
    /// Data rate (b/s) the mask geometry was authored at; defines the unit
    /// interval the stored times are measured in
    pub reference_data_rate: f64,
    /// Crossing level (V) the mask was authored around
    pub reference_v_cross: f64,
    /// Signal swing (V, peak-to-peak) at authoring time
    pub reference_swing: f64,
    /// Mask violation count
    pub violation_count: usize,
    /// Total samples tested
    pub total_samples: usize,
    /// Geometric margin from the last test — how far the mask can be grown
    /// about its centre before the first sample touches it, as a fraction of
    /// its authored size. Negative when the eye already violates. `None`
    /// until a test has run against acquisitions. Never serialized: it is a
    /// property of the data on screen, not of the mask.
    #[serde(skip)]
    pub margin: Option<f64>,
}

impl Default for EyeMask {
    fn default() -> Self {
        Self {
            enabled: false,
            name: "Generic".to_string(),
            inner: MaskPolygon::default_inner(),
            outer: None,
            reference_data_rate: NOMINAL_DATA_RATE,
            reference_v_cross: NOMINAL_V_CROSS,
            reference_swing: NOMINAL_SWING,
            violation_count: 0,
            total_samples: 0,
            margin: None,
        }
    }
}

impl EyeMask {
    /// One unit interval in seconds at the authoring rate (guarded against
    /// degenerate stored rates).
    fn reference_ui_seconds(&self) -> f64 {
        if self.reference_data_rate.is_finite() && self.reference_data_rate > 0.0 {
            1.0 / self.reference_data_rate
        } else {
            1.0 / NOMINAL_DATA_RATE
        }
    }

    /// The inner polygon mapped to display coordinates: x in unit
    /// intervals, y in volts.
    ///
    /// Time mapping: stored times are seconds at the reference data rate.
    /// When the current data rate differs from the reference, we scale the
    /// mask times by the UI ratio (current bit period / reference bit
    /// period) rather than keeping absolute seconds — a compliance mask is
    /// a statement about eye geometry *per unit interval*, and absolute
    /// seconds would smear it across many UIs (or collapse it to a sliver)
    /// at other rates. Dividing the scaled time by the current bit period
    /// to reach UI coordinates cancels the current rate, so the display
    /// coordinate is simply `t_seconds × reference_data_rate`, independent
    /// of the data on screen.
    ///
    /// Voltages are absolute and map straight onto the volt axis — that is
    /// the fidelity fix: the forbidden region no longer dilates with the
    /// measured swing.
    pub fn inner_in_ui_volts(&self) -> MaskPolygon {
        let ui = self.reference_ui_seconds();
        MaskPolygon {
            points: self
                .inner
                .points
                .iter()
                .map(|&(t_s, volts)| (t_s / ui, volts))
                .collect(),
        }
    }

    /// Check if a display-space point — time in unit intervals, voltage in
    /// volts — violates (falls inside) the inner mask. Batch tests should
    /// map the polygon once via [`EyeMask::inner_in_ui_volts`] instead of
    /// calling this per sample.
    #[cfg(test)]
    pub fn check_violation(&self, t_ui: f64, volts: f64) -> bool {
        self.inner_in_ui_volts().contains(t_ui, volts)
    }

    /// Fraction of tested samples that stayed out of the mask.
    ///
    /// This is a pass rate, not a margin, and it is labelled as one. The
    /// distinction matters: an eye that grazes the mask with one sample in a
    /// hundred thousand has a pass rate of 99.999 % and a margin of zero.
    ///
    /// `None` when nothing has been tested — a mask with no samples has not
    /// passed, and reporting 100 % for an empty test is how a mask verdict
    /// comes to mean nothing.
    pub fn pass_rate(&self) -> Option<f64> {
        (self.total_samples > 0)
            .then(|| 1.0 - (self.violation_count as f64 / self.total_samples as f64))
    }
}

/// How far the mask can be scaled about its centre before the first sample
/// falls inside it, less one.
///
/// This is the margin an eye instrument reports: a positive value is
/// headroom — the eye would still pass a mask that much larger — and a
/// negative one says how far the mask must shrink to pass. It is a geometric
/// property of the acquisitions against the polygon, which is what the reader
/// is asking when they ask how close this eye is to failing.
///
/// `None` when there is nothing to test against.
pub(super) fn geometric_margin(
    inner: &MaskPolygon,
    traces: &[rspice_core::analysis::signal_integrity::EyeTrace],
) -> Option<f64> {
    /// Largest scaling searched. Beyond this the mask has left the eye
    /// entirely and the exact number stops being informative.
    const MAX_SCALE: f64 = 4.0;
    /// Bisection steps; the bracket is `MAX_SCALE` wide.
    const STEPS: usize = 22;

    if inner.points.len() < 3 {
        return None;
    }
    let center = inner.centroid()?;
    let has_samples = traces
        .iter()
        .any(|trace| trace.time.len().min(trace.amplitude.len()) > 0);
    if !has_samples {
        return None;
    }

    let violates = |scale: f64| {
        let scaled = inner.scaled_about(center, scale);
        traces.iter().any(|trace| {
            let n = trace.time.len().min(trace.amplitude.len());
            (0..n).any(|i| scaled.contains(trace.time[i], trace.amplitude[i]))
        })
    };

    if !violates(MAX_SCALE) {
        return Some(MAX_SCALE - 1.0);
    }
    // A degenerate polygon contains nothing, so the search is always
    // bracketed and an already-violating mask returns a negative margin.
    let mut clear = 0.0f64;
    let mut hit = MAX_SCALE;
    for _ in 0..STEPS {
        let middle = 0.5 * (clear + hit);
        if violates(middle) {
            hit = middle;
        } else {
            clear = middle;
        }
    }
    Some(0.5 * (clear + hit) - 1.0)
}

/// Migration shim: legacy masks stored polygons normalized to the eye
/// window / swing and carried no reference fields. The presence of
/// `reference_data_rate` marks the absolute format; without it, every
/// polygon is reinterpreted from the legacy normalized form under the
/// nominal authoring context.
impl<'de> Deserialize<'de> for EyeMask {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct EyeMaskDe {
            #[serde(default)]
            enabled: bool,
            #[serde(default = "default_mask_name")]
            name: String,
            #[serde(default)]
            inner: MaskPolygon,
            #[serde(default)]
            outer: Option<MaskPolygon>,
            /// Present only in the absolute format.
            #[serde(default)]
            reference_data_rate: Option<f64>,
            #[serde(default)]
            reference_v_cross: Option<f64>,
            #[serde(default)]
            reference_swing: Option<f64>,
            #[serde(default)]
            violation_count: usize,
            #[serde(default)]
            total_samples: usize,
        }

        let de = EyeMaskDe::deserialize(deserializer)?;
        let (inner, outer, rate, v_cross, swing) = match de.reference_data_rate {
            // Absolute format: points are already (seconds, volts).
            Some(rate) => (
                de.inner,
                de.outer,
                rate,
                de.reference_v_cross.unwrap_or(NOMINAL_V_CROSS),
                de.reference_swing.unwrap_or(NOMINAL_SWING),
            ),
            // Legacy normalized format: (t, v) in 0..1 of the 2-UI window /
            // fraction of swing about the crossing, authored against the
            // nominal context.
            None => (
                MaskPolygon::from_legacy_normalized(&de.inner.points),
                de.outer
                    .as_ref()
                    .map(|p| MaskPolygon::from_legacy_normalized(&p.points)),
                NOMINAL_DATA_RATE,
                NOMINAL_V_CROSS,
                NOMINAL_SWING,
            ),
        };
        Ok(Self {
            enabled: de.enabled,
            name: de.name,
            inner,
            outer,
            reference_data_rate: rate,
            reference_v_cross: v_cross,
            reference_swing: swing,
            violation_count: de.violation_count,
            total_samples: de.total_samples,
            // A margin is a property of the data on screen, not of the mask,
            // so a restored mask has none until it is tested.
            margin: None,
        })
    }
}

fn default_mask_name() -> String {
    "Generic".to_string()
}

/// Polygon for mask definition
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MaskPolygon {
    /// Vertices. In an [`EyeMask`] these are absolute (seconds, volts);
    /// [`EyeMask::inner_in_ui_volts`] maps them to display coordinates,
    /// where `contains` then operates.
    pub points: Vec<(f64, f64)>,
}

impl MaskPolygon {
    /// Default inner mask: hexagonal keep-out spanning the eye opening at
    /// the centre of the nominal 1 Gb/s two-UI window, ±0.2 V about a 0 V
    /// crossing level (25 % of the nominal 0.8 V swing) — the same geometry
    /// the old normalized mask produced on the default eye.
    ///
    /// It sits on the *opening*, at 1 UI, not on a crossing: a mask
    /// straddling a crossing would be violated by every waveform that has
    /// edges, which is all of them. The fold anchors the opening at 1 UI for
    /// exactly this reason.
    pub fn default_inner() -> Self {
        Self {
            points: vec![
                (0.70e-9, 0.0),
                (0.80e-9, 0.20),
                (1.20e-9, 0.20),
                (1.30e-9, 0.0),
                (1.20e-9, -0.20),
                (0.80e-9, -0.20),
            ],
        }
    }

    /// Convert a legacy normalized polygon — t in 0..1 of the 2-UI eye
    /// window, v as a fraction of the swing about the crossing — into
    /// absolute (seconds, volts) under the nominal authoring context.
    fn from_legacy_normalized(points: &[(f64, f64)]) -> Self {
        let window_seconds = LEGACY_WINDOW_UI / NOMINAL_DATA_RATE;
        Self {
            points: points
                .iter()
                .map(|&(tn, vn)| (tn * window_seconds, NOMINAL_V_CROSS + vn * NOMINAL_SWING))
                .collect(),
        }
    }

    /// Area centroid, which is the point a compliance mask grows and shrinks
    /// about. Falls back to the vertex mean for a degenerate (zero-area)
    /// polygon, where the area centroid is undefined.
    fn centroid(&self) -> Option<(f64, f64)> {
        let n = self.points.len();
        if n < 3 {
            return None;
        }
        let mut twice_area = 0.0;
        let mut cx = 0.0;
        let mut cy = 0.0;
        for i in 0..n {
            let (x0, y0) = self.points[i];
            let (x1, y1) = self.points[(i + 1) % n];
            let cross = x0 * y1 - x1 * y0;
            twice_area += cross;
            cx += (x0 + x1) * cross;
            cy += (y0 + y1) * cross;
        }
        if twice_area.abs() > f64::EPSILON {
            return Some((cx / (3.0 * twice_area), cy / (3.0 * twice_area)));
        }
        let mean_x = self.points.iter().map(|p| p.0).sum::<f64>() / n as f64;
        let mean_y = self.points.iter().map(|p| p.1).sum::<f64>() / n as f64;
        (mean_x.is_finite() && mean_y.is_finite()).then_some((mean_x, mean_y))
    }

    /// The polygon scaled about a point, keeping its shape.
    fn scaled_about(&self, center: (f64, f64), scale: f64) -> Self {
        Self {
            points: self
                .points
                .iter()
                .map(|&(x, y)| {
                    (
                        center.0 + (x - center.0) * scale,
                        center.1 + (y - center.1) * scale,
                    )
                })
                .collect(),
        }
    }

    /// Check if point is inside polygon (ray casting)
    pub fn contains(&self, x: f64, y: f64) -> bool {
        if self.points.len() < 3 {
            return false;
        }

        let mut inside = false;
        let n = self.points.len();

        for i in 0..n {
            let j = (i + 1) % n;
            let (xi, yi) = self.points[i];
            let (xj, yj) = self.points[j];

            if ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi) {
                inside = !inside;
            }
        }

        inside
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Old sessions stored polygons normalized to the eye window and swing,
    /// with no reference fields — exactly this payload shape. Loading one
    /// must yield the absolute equivalent under the nominal context.
    #[test]
    fn legacy_normalized_mask_migrates_to_absolute_units() {
        let legacy = r#"{
            "enabled": true,
            "name": "100GBASE-KR4",
            "inner": { "points": [[0.35, 0.0], [0.5, 0.25], [0.65, 0.0], [0.5, -0.25]] },
            "outer": null,
            "violation_count": 3,
            "total_samples": 100
        }"#;
        let mask: EyeMask = serde_json::from_str(legacy).expect("legacy mask loads");
        assert!(mask.enabled);
        assert_eq!(mask.name, "100GBASE-KR4");
        assert_eq!(mask.violation_count, 3);
        // 0.35 of the 2-UI / 1 Gb/s window = 0.70 ns; 0.25 of 0.8 V = 0.2 V.
        assert!((mask.inner.points[0].0 - 0.70e-9).abs() < 1e-18);
        assert!((mask.inner.points[1].1 - 0.20).abs() < 1e-12);
        assert_eq!(mask.reference_data_rate, NOMINAL_DATA_RATE);
        // Display geometry is unchanged by the migration: 0.35 × 2 UI = 0.7 UI.
        let ui = mask.inner_in_ui_volts();
        assert!((ui.points[0].0 - 0.70).abs() < 1e-12);
    }

    #[test]
    fn absolute_mask_round_trips() {
        let mask = EyeMask::default();
        let json = serde_json::to_string(&mask).expect("mask serializes");
        let back: EyeMask = serde_json::from_str(&json).expect("absolute mask loads");
        assert_eq!(back.inner.points, mask.inner.points);
        assert_eq!(back.reference_data_rate, mask.reference_data_rate);
        assert_eq!(back.reference_swing, mask.reference_swing);
    }

    /// The mask is pinned to unit intervals, not absolute screen seconds:
    /// the display mapping depends only on the reference rate.
    #[test]
    fn display_mapping_pins_mask_to_unit_intervals() {
        let mask = EyeMask::default();
        let ui = mask.inner_in_ui_volts();
        assert!((ui.points[0].0 - 0.70).abs() < 1e-12);
        assert!((ui.points[3].0 - 1.30).abs() < 1e-12);
        assert!(ui.contains(1.0, 0.0));
        assert!(!ui.contains(0.2, 0.0));
        assert!(mask.check_violation(1.0, 0.1));
        assert!(!mask.check_violation(1.0, 0.5));
    }
}
