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
        }
    }
}

impl EyeMask {
    /// Create new mask
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

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
    pub fn check_violation(&self, t_ui: f64, volts: f64) -> bool {
        self.inner_in_ui_volts().contains(t_ui, volts)
    }

    /// Get mask margin (minimum distance to mask)
    pub fn get_margin(&self) -> f64 {
        if self.total_samples == 0 {
            return 1.0;
        }
        1.0 - (self.violation_count as f64 / self.total_samples as f64)
    }

    /// Is mask passing (no violations)?
    pub fn is_passing(&self) -> bool {
        self.violation_count == 0
    }
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
    /// Default inner mask: hexagonal eye opening centered on the crossing
    /// at 1 UI of the nominal 1 Gb/s eye, ±0.2 V about a 0 V crossing
    /// (25 % of the nominal 0.8 V swing) — the same geometry the old
    /// normalized mask produced on the default 2-UI, 1 Gb/s eye.
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
