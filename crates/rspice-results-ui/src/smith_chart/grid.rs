//! Smith chart grid geometry on the reflection-coefficient plane.
//!
//! The impedance chart is the constant-resistance circles and the
//! constant-reactance arcs of z = (1 + Γ)/(1 − Γ). The admittance chart is
//! the same family for y = (1 − Γ)/(1 + Γ) = z(−Γ): every circle reflected
//! through the chart centre, so constant conductance g passes through Γ = −1
//! and a positive (capacitive) susceptance arc lies in the lower half.

use num_complex::Complex64;

/// Which coordinate grid a Smith pane draws.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SmithGrid {
    /// Constant resistance and reactance.
    #[default]
    Impedance,
    /// Constant conductance and susceptance.
    Admittance,
    /// Both, the admittance grid drawn in its own ink.
    ImpedanceAdmittance,
}

impl SmithGrid {
    /// Segmented-control labels, in [`Self::index`] order.
    pub const OPTIONS: [&'static str; 3] = ["Z", "Y", "Z+Y"];

    pub const fn index(self) -> usize {
        match self {
            Self::Impedance => 0,
            Self::Admittance => 1,
            Self::ImpedanceAdmittance => 2,
        }
    }

    pub const fn from_index(index: usize) -> Self {
        match index {
            1 => Self::Admittance,
            2 => Self::ImpedanceAdmittance,
            _ => Self::Impedance,
        }
    }

    pub const fn draws_impedance(self) -> bool {
        matches!(self, Self::Impedance | Self::ImpedanceAdmittance)
    }

    pub const fn draws_admittance(self) -> bool {
        matches!(self, Self::Admittance | Self::ImpedanceAdmittance)
    }
}

/// The normalized values the grid draws and labels: resistance and
/// conductance circles, and reactance and susceptance arcs of either sign.
pub const GRID_VALUES: [f64; 5] = [0.2, 0.5, 1.0, 2.0, 5.0];

/// A grid circle on the Γ plane.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridCircle {
    pub center: Complex64,
    pub radius: f64,
}

/// Constant normalized resistance `r`: centre r/(r + 1), radius 1/(r + 1).
pub fn resistance_circle(r: f64) -> GridCircle {
    GridCircle {
        center: Complex64::new(r / (r + 1.0), 0.0),
        radius: 1.0 / (r + 1.0),
    }
}

/// Constant normalized reactance `x` (signed): centre 1 + j/x, radius 1/|x|.
pub fn reactance_circle(x: f64) -> GridCircle {
    GridCircle {
        center: Complex64::new(1.0, 1.0 / x),
        radius: 1.0 / x.abs(),
    }
}

/// Constant normalized conductance `g`: the resistance circle reflected
/// through the centre.
pub fn conductance_circle(g: f64) -> GridCircle {
    let circle = resistance_circle(g);
    GridCircle {
        center: -circle.center,
        radius: circle.radius,
    }
}

/// Constant normalized susceptance `b` (signed): the reactance arc reflected
/// through the centre.
pub fn susceptance_circle(b: f64) -> GridCircle {
    let circle = reactance_circle(b);
    GridCircle {
        center: -circle.center,
        radius: circle.radius,
    }
}

/// Where a resistance circle crosses the real axis away from Γ = 1, and
/// where a conductance circle does away from Γ = −1: the label anchors.
pub fn real_axis_crossing(value: f64, admittance: bool) -> Complex64 {
    let gamma = Complex64::new((value - 1.0) / (value + 1.0), 0.0);
    if admittance { -gamma } else { gamma }
}

/// Where a reactance (or susceptance) arc meets the |Γ| = 1 boundary away
/// from Γ = 1 (or Γ = −1): Γ = (jx − 1)/(jx + 1) on the impedance chart.
pub fn rim_crossing(value: f64, admittance: bool) -> Complex64 {
    let z = Complex64::new(0.0, value);
    let gamma = (z - 1.0) / (z + 1.0);
    if admittance { -gamma } else { gamma }
}

/// The arc of a circle that lies inside the unit disc, as a start angle and
/// a sweep in radians about the circle's own centre. `None` when no part of
/// the circle is inside.
///
/// Drawing only this arc keeps a circle the size of a continent — a
/// near-degenerate stability circle — to a few dozen points.
pub fn arc_inside_unit_disc(center: Complex64, radius: f64) -> Option<(f64, f64)> {
    if !(radius.is_finite() && radius > 0.0 && center.re.is_finite() && center.im.is_finite()) {
        return None;
    }
    let distance = center.norm();
    if distance + radius <= 1.0 + 1.0e-9 {
        return Some((0.0, std::f64::consts::TAU));
    }
    if distance >= 1.0 + radius || radius >= distance + 1.0 {
        return None;
    }
    // |c + r·e^{jφ}| ≤ 1 ⇔ cos(φ − φ₀) ≥ (d² + r² − 1)/(2rd), with φ₀ the
    // direction from the centre towards the origin.
    let toward_origin = (-center.im).atan2(-center.re);
    let cosine = ((distance * distance + radius * radius - 1.0) / (2.0 * radius * distance))
        .clamp(-1.0, 1.0);
    let half = cosine.acos();
    (half > 0.0).then_some((toward_origin - half, 2.0 * half))
}

/// Sample points along the in-disc arc of a circle, densely enough that a
/// 300 px chart shows no facets.
pub fn arc_points(center: Complex64, radius: f64) -> Vec<Complex64> {
    let Some((start, sweep)) = arc_inside_unit_disc(center, radius) else {
        return Vec::new();
    };
    let steps = ((sweep / std::f64::consts::TAU) * 256.0).ceil().max(12.0) as usize;
    (0..=steps)
        .map(|step| {
            let angle = start + sweep * step as f64 / steps as f64;
            center + Complex64::from_polar(radius, angle)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn z_to_gamma(z: Complex64) -> Complex64 {
        (z - 1.0) / (z + 1.0)
    }

    fn y_to_gamma(y: Complex64) -> Complex64 {
        (1.0 - y) / (1.0 + y)
    }

    fn on(circle: GridCircle, gamma: Complex64) -> bool {
        ((gamma - circle.center).norm() - circle.radius).abs() < 1.0e-12
    }

    /// The impedance grid's circles pass through the impedances they name.
    #[test]
    fn impedance_circles_pass_through_their_impedances() {
        for value in GRID_VALUES {
            for other in [0.0, 0.3, 1.0, -2.5, 40.0] {
                assert!(on(
                    resistance_circle(value),
                    z_to_gamma(Complex64::new(value, other))
                ));
                for sign in [1.0, -1.0] {
                    assert!(on(
                        reactance_circle(sign * value),
                        z_to_gamma(Complex64::new(other.abs(), sign * value))
                    ));
                }
            }
        }
    }

    /// The admittance grid is the impedance grid reflected through the
    /// centre: its circles pass through the admittances they name, constant
    /// conductance through Γ = −1, and +jb lies in the lower half.
    #[test]
    fn admittance_circles_are_the_impedance_circles_reflected_through_the_centre() {
        for value in GRID_VALUES {
            let g = conductance_circle(value);
            assert!(on(g, Complex64::new(-1.0, 0.0)));
            assert!(on(g, Complex64::new((1.0 - value) / (1.0 + value), 0.0)));
            assert_eq!(g.center, -resistance_circle(value).center);
            for other in [0.0, 0.3, 1.0, -2.5, 40.0] {
                assert!(on(g, y_to_gamma(Complex64::new(value, other))));
                for sign in [1.0, -1.0] {
                    let b = susceptance_circle(sign * value);
                    assert!(on(b, y_to_gamma(Complex64::new(other.abs(), sign * value))));
                    // Each admittance circle is an impedance circle's mirror.
                    let gamma = z_to_gamma(Complex64::new(other.abs(), sign * value));
                    assert!(on(b, -gamma));
                }
            }
            // A capacitor (positive susceptance) sits in the lower half.
            assert!(y_to_gamma(Complex64::new(1.0, value)).im < 0.0);
            assert!(susceptance_circle(value).center.im < 0.0);
            assert!(rim_crossing(value, true).im < 0.0);
            assert!(on(susceptance_circle(value), rim_crossing(value, true)));
            assert!((rim_crossing(value, true).norm() - 1.0).abs() < 1.0e-12);
            assert!(on(g, real_axis_crossing(value, true)));
            assert!(on(
                resistance_circle(value),
                real_axis_crossing(value, false)
            ));
        }
    }

    #[test]
    fn only_the_arc_inside_the_chart_is_drawn() {
        // Wholly inside, wholly outside, enclosing the chart.
        assert_eq!(
            arc_inside_unit_disc(Complex64::new(0.2, 0.0), 0.5),
            Some((0.0, std::f64::consts::TAU))
        );
        assert!(arc_inside_unit_disc(Complex64::new(3.0, 0.0), 1.5).is_none());
        assert!(arc_inside_unit_disc(Complex64::new(0.1, 0.0), 4.0).is_none());
        // A huge, far circle that clips the chart: every sampled point is on
        // the circle and within the chart, and the arc reaches the rim.
        let center = Complex64::from_polar(25.37, 102.76f64.to_radians());
        let points = arc_points(center, 25.17);
        assert!(points.len() > 2);
        for point in &points {
            assert!(((point - center).norm() - 25.17).abs() < 1.0e-9);
            assert!(point.norm() <= 1.0 + 1.0e-9);
        }
        assert!((points[0].norm() - 1.0).abs() < 1.0e-9);
        assert!((points[points.len() - 1].norm() - 1.0).abs() < 1.0e-9);
    }
}
