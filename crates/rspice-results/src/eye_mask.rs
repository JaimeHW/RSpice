//! The eye's compliance mask, drawn by the reader.
//!
//! A mask is a keep-out shape over the eye's opening — a hexagon, a
//! diamond, a rectangle or a polygon of the reader's own — sized in unit
//! intervals and volts and centred on the eye it is tested against: half
//! way across the folded window and on the eye's threshold, so a mask drawn
//! for a single-ended 0 to 1.2 V eye sits on 0.6 V, not on 0 V. An offset
//! moves it off that centre, a margin grows or shrinks it about its centre,
//! and an automatic fit sizes it to the largest the eye clears.
//!
//! No standard's mask ships: a named template is a compliance claim, and
//! the eye sheet does not model a standard's reference equaliser or clock
//! recovery. A mask is exchanged as a small text file of its vertices.

use serde::{Deserialize, Serialize};

/// The drawn shapes of a mask.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MaskShape {
    /// Pointed at the crossings' sides, flat over the top and bottom.
    #[default]
    Hexagon,
    Diamond,
    Rectangle,
    /// The reader's own vertices.
    Custom,
}

impl MaskShape {
    pub const ALL: [Self; 4] = [Self::Hexagon, Self::Diamond, Self::Rectangle, Self::Custom];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Hexagon => "Hexagon",
            Self::Diamond => "Diamond",
            Self::Rectangle => "Rectangle",
            Self::Custom => "Custom",
        }
    }
}

/// Fewest vertices a custom mask has.
pub const MIN_MASK_VERTICES: usize = 3;
/// Share of the eye's amplitude an unsized mask is tall.
const UNSIZED_HEIGHT_SHARE: f64 = 0.4;
/// How much of the clearance an automatic fit takes, so the fitted mask
/// does not touch the sample that bounds it.
const FIT_SHARE: f64 = 0.98;

/// A mask: its shape, its size, where it sits relative to the eye and the
/// result of its last test.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EyeMask {
    pub shape: MaskShape,
    /// Full width, in unit intervals.
    pub width_ui: f64,
    /// Width of a hexagon's flat top and bottom, in unit intervals.
    pub flat_ui: f64,
    /// Full height, in volts; `None` until sized, when it is a share of the
    /// eye's amplitude.
    pub height: Option<f64>,
    /// A custom mask's vertices about its centre, in unit intervals and
    /// volts.
    pub vertices: Vec<(f64, f64)>,
    /// How far the centre sits right of the window's centre, in unit
    /// intervals, and above the eye's threshold, in volts.
    pub offset_ui: f64,
    pub offset_v: f64,
    /// How much larger the mask is tested than drawn, as a fraction of its
    /// size: negative shrinks it.
    pub margin: f64,
    /// Samples inside the mask at the last test.
    #[serde(skip)]
    pub violation_count: usize,
    /// Samples tested.
    #[serde(skip)]
    pub total_samples: usize,
    /// How far the tested mask could grow about its centre before a sample
    /// touches it, as a fraction of its size; negative when it is already
    /// violated. `None` until tested.
    #[serde(skip)]
    pub clearance: Option<f64>,
}

impl Default for EyeMask {
    fn default() -> Self {
        Self {
            shape: MaskShape::Hexagon,
            width_ui: 0.5,
            flat_ui: 0.2,
            height: None,
            vertices: Vec::new(),
            offset_ui: 0.0,
            offset_v: 0.0,
            margin: 0.0,
            violation_count: 0,
            total_samples: 0,
            clearance: None,
        }
    }
}

/// Where a mask is centred on one eye: the window's centre, in unit
/// intervals, and the eye's threshold, in volts, with the amplitude an
/// unsized mask takes its height from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaskFrame {
    pub centre_ui: f64,
    pub threshold: f64,
    pub amplitude: f64,
}

impl EyeMask {
    /// Whether the drawn size is a mask: a positive width and height, and
    /// enough vertices for a custom one.
    pub fn validate(&self) -> Result<(), String> {
        let positive = |value: f64| value.is_finite() && value > 0.0;
        if !positive(self.width_ui) {
            return Err("The mask's width must be a positive number of unit intervals".into());
        }
        if !(self.flat_ui.is_finite() && (0.0..self.width_ui).contains(&self.flat_ui)) {
            return Err("The hexagon's flat must be narrower than the mask".into());
        }
        if self.height.is_some_and(|height| !positive(height)) {
            return Err("The mask's height must be a positive number of volts".into());
        }
        if self.shape == MaskShape::Custom && self.vertices.len() < MIN_MASK_VERTICES {
            return Err(format!(
                "A custom mask needs at least {MIN_MASK_VERTICES} vertices"
            ));
        }
        if !(self.offset_ui.is_finite() && self.offset_v.is_finite()) {
            return Err("The mask's offset must be a number".into());
        }
        if !(self.margin.is_finite() && self.margin > -1.0) {
            return Err("The mask's margin must be above −100 %".into());
        }
        Ok(())
    }

    /// The height the mask is drawn at on an eye of `amplitude`.
    pub fn drawn_height(&self, amplitude: f64) -> f64 {
        self.height
            .unwrap_or(UNSIZED_HEIGHT_SHARE * amplitude.abs())
    }

    /// The shape's vertices about its centre, before the margin, in unit
    /// intervals and volts.
    pub fn outline(&self, amplitude: f64) -> Vec<(f64, f64)> {
        let half_w = 0.5 * self.width_ui;
        let half_h = 0.5 * self.drawn_height(amplitude);
        let half_flat = 0.5 * self.flat_ui;
        match self.shape {
            MaskShape::Hexagon => vec![
                (-half_w, 0.0),
                (-half_flat, half_h),
                (half_flat, half_h),
                (half_w, 0.0),
                (half_flat, -half_h),
                (-half_flat, -half_h),
            ],
            MaskShape::Diamond => {
                vec![(-half_w, 0.0), (0.0, half_h), (half_w, 0.0), (0.0, -half_h)]
            }
            MaskShape::Rectangle => vec![
                (-half_w, -half_h),
                (-half_w, half_h),
                (half_w, half_h),
                (half_w, -half_h),
            ],
            MaskShape::Custom => self.vertices.clone(),
        }
    }

    /// The mask as tested on the eye `frame` describes: placed and grown by
    /// its margin, in the eye's unit intervals and volts.
    pub fn polygon(&self, frame: MaskFrame) -> MaskPolygon {
        let centre = (
            frame.centre_ui + self.offset_ui,
            frame.threshold + self.offset_v,
        );
        let scale = 1.0 + self.margin;
        MaskPolygon {
            points: self
                .outline(frame.amplitude)
                .into_iter()
                .map(|(x, y)| (centre.0 + x * scale, centre.1 + y * scale))
                .collect(),
        }
    }

    /// Count the samples of `traces` inside the mask on the eye `frame`
    /// describes, and how far it clears them.
    pub fn test_traces<'a>(
        &mut self,
        frame: MaskFrame,
        traces: impl Iterator<Item = (&'a [f64], &'a [f64])> + Clone,
    ) {
        let polygon = self.polygon(frame);
        self.violation_count = 0;
        self.total_samples = 0;
        for (time, amplitude) in traces.clone() {
            let n = time.len().min(amplitude.len());
            for i in 0..n {
                if polygon.contains(time[i], amplitude[i]) {
                    self.violation_count += 1;
                }
                self.total_samples += 1;
            }
        }
        self.clearance = geometric_margin(&polygon, traces);
    }

    /// Forget the last test.
    pub fn clear_test(&mut self) {
        self.violation_count = 0;
        self.total_samples = 0;
        self.clearance = None;
    }

    /// The mask sized to the largest of its shape that `traces` clear about
    /// its centre, with no margin: its width and height scaled together, so
    /// the shape stays the one drawn. `None` when there is nothing to fit
    /// to or no size clears the eye.
    pub fn fitted<'a>(
        &self,
        frame: MaskFrame,
        traces: impl Iterator<Item = (&'a [f64], &'a [f64])> + Clone,
    ) -> Option<Self> {
        let unmargined = Self {
            margin: 0.0,
            ..self.clone()
        };
        let clearance = geometric_margin(&unmargined.polygon(frame), traces)?;
        let scale = (1.0 + clearance) * FIT_SHARE;
        if !(scale.is_finite() && scale > 0.0) {
            return None;
        }
        Some(Self {
            width_ui: self.width_ui * scale,
            flat_ui: self.flat_ui * scale,
            height: Some(self.drawn_height(frame.amplitude) * scale),
            vertices: self
                .vertices
                .iter()
                .map(|&(x, y)| (x * scale, y * scale))
                .collect(),
            margin: 0.0,
            ..unmargined
        })
    }

    /// The mask as a text file: one vertex about its centre per line, in
    /// unit intervals and volts, before its margin and offset.
    pub fn to_text(&self, amplitude: f64) -> String {
        let mut text = String::from(
            "# RSpice eye mask: vertices about the mask's centre\n# unit_intervals volts\n",
        );
        for (x, y) in self.outline(amplitude) {
            text.push_str(&format!("{x} {y}\n"));
        }
        text
    }

    /// A custom mask from a text file of vertices, one `unit_intervals
    /// volts` pair per line; `#` starts a comment.
    pub fn from_text(text: &str) -> Result<Self, String> {
        let mut vertices = Vec::new();
        for (number, line) in text.lines().enumerate() {
            let line = line.split('#').next().unwrap_or("").trim();
            if line.is_empty() {
                continue;
            }
            let mut fields = line
                .split(|c: char| c.is_whitespace() || c == ',')
                .filter(|field| !field.is_empty());
            let parse = |field: Option<&str>| {
                field
                    .and_then(|field| field.parse::<f64>().ok())
                    .filter(|value| value.is_finite())
            };
            match (parse(fields.next()), parse(fields.next()), fields.next()) {
                (Some(x), Some(y), None) => vertices.push((x, y)),
                _ => {
                    return Err(format!(
                        "Line {} is not a vertex: write unit intervals then volts",
                        number + 1
                    ));
                }
            }
        }
        let mask = Self {
            shape: MaskShape::Custom,
            vertices,
            ..Self::default()
        };
        mask.validate()?;
        Ok(mask)
    }
}

/// How far `polygon` can be scaled about its centre before the first
/// sample falls inside it, less one: positive headroom, negative how far it
/// must shrink to pass. `None` when there is nothing to test against.
fn geometric_margin<'a>(
    polygon: &MaskPolygon,
    traces: impl Iterator<Item = (&'a [f64], &'a [f64])> + Clone,
) -> Option<f64> {
    /// Largest scaling searched. Beyond this the mask has left the eye
    /// entirely and the exact number stops being informative.
    const MAX_SCALE: f64 = 4.0;
    /// Bisection steps; the bracket is `MAX_SCALE` wide.
    const STEPS: usize = 22;

    if polygon.points.len() < MIN_MASK_VERTICES {
        return None;
    }
    let center = polygon.centroid()?;
    let has_samples = traces
        .clone()
        .any(|(time, amplitude)| time.len().min(amplitude.len()) > 0);
    if !has_samples {
        return None;
    }

    let violates = |scale: f64| {
        let scaled = polygon.scaled_about(center, scale);
        traces.clone().any(|(time, amplitude)| {
            let n = time.len().min(amplitude.len());
            (0..n).any(|i| scaled.contains(time[i], amplitude[i]))
        })
    };

    if !violates(MAX_SCALE) {
        return Some(MAX_SCALE - 1.0);
    }
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

/// A closed polygon in the eye's unit intervals and volts.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MaskPolygon {
    pub points: Vec<(f64, f64)>,
}

impl MaskPolygon {
    /// Area centroid, the point a mask grows and shrinks about; the vertex
    /// mean for a polygon of no area.
    fn centroid(&self) -> Option<(f64, f64)> {
        let n = self.points.len();
        if n < MIN_MASK_VERTICES {
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
        #[allow(clippy::cast_precision_loss)]
        let count = n as f64;
        let mean_x = self.points.iter().map(|p| p.0).sum::<f64>() / count;
        let mean_y = self.points.iter().map(|p| p.1).sum::<f64>() / count;
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

    /// Whether `(x, y)` is inside, by ray casting.
    pub fn contains(&self, x: f64, y: f64) -> bool {
        if self.points.len() < MIN_MASK_VERTICES {
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

    /// A single-ended 0 to 1.2 V eye folded over two unit intervals.
    const SINGLE_ENDED: MaskFrame = MaskFrame {
        centre_ui: 1.0,
        threshold: 0.6,
        amplitude: 1.2,
    };

    #[test]
    fn a_mask_sits_on_the_eyes_threshold_not_on_zero_volts() {
        let mask = EyeMask {
            height: Some(0.4),
            ..EyeMask::default()
        };
        let polygon = mask.polygon(SINGLE_ENDED);
        assert!(polygon.contains(1.0, 0.6));
        assert!(!polygon.contains(1.0, 0.0));
        let top = polygon.points.iter().map(|p| p.1).fold(f64::MIN, f64::max);
        let bottom = polygon.points.iter().map(|p| p.1).fold(f64::MAX, f64::min);
        assert!((top - 0.8).abs() < 1e-12 && (bottom - 0.4).abs() < 1e-12);
    }

    #[test]
    fn offset_and_margin_move_and_grow_the_mask_about_its_centre() {
        let mask = EyeMask {
            shape: MaskShape::Rectangle,
            width_ui: 0.4,
            height: Some(0.2),
            offset_ui: 0.1,
            offset_v: -0.05,
            margin: 0.5,
            ..EyeMask::default()
        };
        let polygon = mask.polygon(SINGLE_ENDED);
        let xs: Vec<f64> = polygon.points.iter().map(|p| p.0).collect();
        let ys: Vec<f64> = polygon.points.iter().map(|p| p.1).collect();
        let span = |values: &[f64]| {
            values.iter().copied().fold(f64::MIN, f64::max)
                - values.iter().copied().fold(f64::MAX, f64::min)
        };
        assert!((span(&xs) - 0.6).abs() < 1e-12);
        assert!((span(&ys) - 0.3).abs() < 1e-12);
        assert!(polygon.contains(1.1, 0.55));
    }

    #[test]
    fn a_fit_grows_the_mask_to_just_inside_the_samples_around_it() {
        // Samples on the opening's rim: 0.3 UI either side, 0.25 V above
        // and below the threshold.
        let time = [0.7, 1.3, 1.0, 1.0];
        let volts = [0.6, 0.6, 0.85, 0.35];
        let traces = [(&time[..], &volts[..])];
        let mask = EyeMask {
            shape: MaskShape::Diamond,
            width_ui: 0.2,
            height: Some(0.1),
            ..EyeMask::default()
        };
        let fitted = mask
            .fitted(SINGLE_ENDED, traces.iter().copied())
            .expect("samples bound it");
        assert!(
            (fitted.width_ui - 0.6 * FIT_SHARE).abs() < 1e-5,
            "{fitted:?}"
        );
        assert!((fitted.height.unwrap() - 0.3 * FIT_SHARE).abs() < 1e-5);
        let mut tested = fitted;
        tested.test_traces(SINGLE_ENDED, traces.iter().copied());
        assert_eq!(tested.violation_count, 0);
    }

    #[test]
    fn a_mask_round_trips_through_its_text_file() {
        let mask = EyeMask {
            height: Some(0.3),
            ..EyeMask::default()
        };
        let text = mask.to_text(1.2);
        let read = EyeMask::from_text(&text).expect("its own file reads");
        assert_eq!(read.shape, MaskShape::Custom);
        assert_eq!(read.outline(1.2), mask.outline(1.2));
        assert!(EyeMask::from_text("0 0\n1 1\n").is_err());
        let error = EyeMask::from_text("0 0\n1 x\n0.5 1\n").expect_err("a bad line");
        assert!(error.contains("Line 2"), "{error}");
    }

    #[test]
    fn a_mask_drawn_too_small_or_too_flat_is_refused() {
        assert!(EyeMask::default().validate().is_ok());
        let flat = EyeMask {
            flat_ui: 0.6,
            ..EyeMask::default()
        };
        assert!(flat.validate().is_err());
        let low = EyeMask {
            height: Some(0.0),
            ..EyeMask::default()
        };
        assert!(low.validate().is_err());
    }
}
