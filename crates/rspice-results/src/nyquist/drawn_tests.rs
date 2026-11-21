//! The drawn locus: a coarse sweep draws the curve a fine one does, and the
//! margins and the nearest approach are read on that curve.

use super::tests::{swept_loop, three_pole};
use super::*;
use num_complex::Complex64;

/// How long one unit of the plane is drawn on the Nyquist sheet at 1600x900,
/// framed on the critical point of the loop below: measured on the rendered
/// sheet, where −2 and 1 on the real axis stand 571 px apart.
const PIXELS_PER_UNIT: f64 = 190.0;

/// The loop the sheet's own pictures draw: L = 4/(1 + jf/1 kHz)³, a gain
/// margin of 6.02 dB at √3 kHz and a phase margin of 27.14° at 1.23 kHz.
const GAIN: f64 = 4.0;
const POLE: f64 = 1.0e3;

/// The part of the plane the critical-point frame shows for that loop.
fn in_critical_frame(point: &NyquistPoint) -> bool {
    (-2.2..=1.2).contains(&point.real) && (-1.7..=1.7).contains(&point.imag)
}

/// Distance from `point` to the segment `a`–`b`.
fn distance_to_segment(point: (f64, f64), a: &NyquistPoint, b: &NyquistPoint) -> f64 {
    let (dx, dy) = (b.real - a.real, b.imag - a.imag);
    let length_squared = dx * dx + dy * dy;
    let t = if length_squared > 0.0 {
        (((point.0 - a.real) * dx + (point.1 - a.imag) * dy) / length_squared).clamp(0.0, 1.0)
    } else {
        0.0
    };
    (point.0 - a.real - t * dx).hypot(point.1 - a.imag - t * dy)
}

/// Distance from `point` to the polyline through `vertices`.
fn distance_to_polyline(point: (f64, f64), vertices: &[NyquistPoint]) -> f64 {
    vertices
        .windows(2)
        .map(|pair| distance_to_segment(point, &pair[0], &pair[1]))
        .fold(f64::INFINITY, f64::min)
}

/// The farthest any vertex of `drawn` inside the frame sits from `reference`,
/// in pixels of the 1600x900 sheet.
fn farthest_in_pixels(drawn: &[NyquistPoint], reference: &[NyquistPoint]) -> f64 {
    drawn
        .iter()
        .filter(|point| in_critical_frame(point))
        .map(|point| distance_to_polyline((point.real, point.imag), reference))
        .fold(0.0, f64::max)
        * PIXELS_PER_UNIT
}

/// The same loop sampled ten and a hundred times a decade draws one curve:
/// within a pixel of each other at 1600x900 (it measures 0.015 px), where
/// chords between the coarse samples cut the corners by 8.2 px.
#[test]
fn a_coarse_sweep_draws_the_curve_a_fine_one_does() {
    let coarse = swept_loop(1.0, 1.0e6, 10, three_pole(GAIN, POLE));
    let fine = swept_loop(1.0, 1.0e6, 100, three_pole(GAIN, POLE));
    let (coarse_drawn, fine_drawn) = (coarse.drawn_locus(), fine.drawn_locus());

    let apart = farthest_in_pixels(&coarse_drawn.points, &fine_drawn.points)
        .max(farthest_in_pixels(&fine_drawn.points, &coarse_drawn.points));
    assert!(apart < 1.0, "the two sweeps draw {apart} px apart");

    let chords = farthest_in_pixels(&fine_drawn.points, &coarse.points);
    assert!(
        chords > 4.0,
        "chords between the samples miss by {chords} px"
    );

    // Every sample is a vertex of the drawn locus, in order.
    let mut vertices = coarse_drawn.points.iter();
    for sample in &coarse.points {
        assert!(
            vertices.any(|vertex| vertex == sample),
            "{sample:?} is drawn"
        );
    }
}

/// At ten points a decade the margins read on the drawn curve are the closed
/// form's, wherever the samples fall: measured within 0.0011° and 0.00013 dB
/// over sixteen offsets of the sweep. And each lies on the drawn locus, where
/// its marker is put.
#[test]
fn the_margins_lie_on_the_drawn_locus_and_read_the_closed_form() {
    let unity = (GAIN.powf(2.0 / 3.0) - 1.0).sqrt();
    let phase_margin = 180.0 - 3.0 * unity.atan().to_degrees();
    let gain_margin = 20.0 * (8.0 / GAIN).log10();
    for offset in 0..16 {
        let start = 10f64.powf(f64::from(offset) / 160.0);
        let locus = swept_loop(start, 1.0e6 * start, 10, three_pole(GAIN, POLE));
        let drawn = locus.drawn_locus();

        let phase = locus.phase_margin().expect("the loop crosses unity");
        assert!(
            (phase.value - phase_margin).abs() < 0.002,
            "phase margin {} against {phase_margin}",
            phase.value
        );
        let (sin, cos) = phase.phase_deg.to_radians().sin_cos();
        let off = distance_to_polyline((cos, sin), &drawn.points) * PIXELS_PER_UNIT;
        assert!(off < 0.05, "the phase margin sits {off} px off the locus");

        let gain = locus.gain_margin().expect("the loop crosses -180°");
        let decibels = 20.0 * gain.value.log10();
        assert!(
            (decibels - gain_margin).abs() < 0.0002,
            "gain margin {decibels} against {gain_margin}"
        );
        let off = distance_to_polyline((-1.0 / gain.value, 0.0), &drawn.points) * PIXELS_PER_UNIT;
        assert!(off < 0.05, "the gain margin sits {off} px off the locus");
    }
}

/// The nearest approach to −1 is read on the drawn curve, between samples:
/// at ten points a decade it is the closed form's to a part in 10^4 (it
/// measures 3e-5), where the nearest retained sample is 21% farther.
#[test]
fn the_nearest_approach_to_the_critical_point_is_read_between_samples() {
    let loop_gain = three_pole(GAIN, POLE);
    let locus = swept_loop(1.0, 1.0e6, 10, &loop_gain);
    let closed_form = (0..=200_000)
        .map(|index| {
            let f = POLE * 10f64.powf(-1.0 + 2.0 * f64::from(index) / 200_000.0);
            (Complex64::new(1.0, 0.0) + loop_gain(Complex64::new(0.0, TAU * f))).norm()
        })
        .fold(f64::INFINITY, f64::min);
    let sampled = locus
        .points
        .iter()
        .map(NyquistPoint::distance_from_critical)
        .fold(f64::INFINITY, f64::min);

    let nearest = locus
        .min_distance_from_critical()
        .expect("the locus has samples");
    assert!(
        (nearest / closed_form - 1.0).abs() < 1.0e-4,
        "nearest approach {nearest} against {closed_form}"
    );
    assert!(nearest < sampled, "{nearest} is no nearer than {sampled}");
}
