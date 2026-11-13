use super::*;
use crate::calculator::functions::FunctionRegistry;
use crate::calculator::value::RealValue;
use std::f64::consts::PI;

/// `exp(-t/TAU) sin(2 pi F t)` over five periods.
const F: f64 = 1.0e3;
const TAU: f64 = 2.0e-3;
const SAMPLES: usize = 20_001;
const STOP: f64 = 5.0 / F;

fn damped(t: f64) -> f64 {
    (-t / TAU).exp() * (2.0 * PI * F * t).sin()
}

fn damped_sine() -> (Vec<f64>, Vec<f64>) {
    let x: Vec<f64> = (0..SAMPLES)
        .map(|index| STOP * index as f64 / (SAMPLES - 1) as f64)
        .collect();
    let y = x.iter().map(|t| damped(*t)).collect();
    (x, y)
}

const DT: f64 = STOP / (SAMPLES - 1) as f64;

/// The kth extremum of the damped sine: `tan(wt) = w tau`. Even k are
/// peaks, odd k valleys.
fn extremum(k: usize) -> f64 {
    let omega = 2.0 * PI * F;
    ((omega * TAU).atan() + k as f64 * PI) / omega
}

/// The largest error a sample can make in the value of an extremum it is
/// nearest: the curve is within half a sample of its top, and falls away from
/// it no faster than its curvature allows.
fn extremum_value_tolerance() -> f64 {
    let curvature = (2.0 * PI * F + 1.0 / TAU).powi(2);
    0.5 * curvature * (0.5 * DT).powi(2)
}

#[track_caller]
fn assert_extremum(found: FeaturePoint, k: usize) {
    let t = extremum(k);
    assert!(
        (found.x - t).abs() <= 0.5 * DT * (1.0 + 1e-9),
        "extremum {k}: found at {}, analytic {t}, sample spacing {DT}",
        found.x
    );
    assert!(
        (found.y - damped(t)).abs() <= extremum_value_tolerance(),
        "extremum {k}: found {}, analytic {}",
        found.y,
        damped(t)
    );
}

fn search(x: &[f64], y: &[f64], from: f64, feature: Feature) -> FeaturePoint {
    find(x, y, from, feature, default_excursion(y)).expect("the feature is found")
}

#[test]
fn the_maximum_and_minimum_of_a_damped_sine_are_its_first_peak_and_valley() {
    let (x, y) = damped_sine();
    assert_extremum(search(&x, &y, 0.0, Feature::Maximum), 0);
    assert_extremum(search(&x, &y, 0.0, Feature::Minimum), 1);
}

#[test]
fn stepping_peaks_and_valleys_visits_each_analytic_extremum_in_turn() {
    let (x, y) = damped_sine();
    let mut from = 0.0;
    for k in (0..10).step_by(2) {
        let peak = search(&x, &y, from, Feature::Peak(Direction::Next));
        assert_extremum(peak, k);
        from = peak.x;
    }
    assert_eq!(
        find(
            &x,
            &y,
            from,
            Feature::Peak(Direction::Next),
            default_excursion(&y)
        ),
        Err(SearchRefusal::NotFound),
        "five periods hold five peaks"
    );
    for k in (0..10).step_by(2).rev().skip(1) {
        let peak = search(&x, &y, from, Feature::Peak(Direction::Previous));
        assert_extremum(peak, k);
        from = peak.x;
    }
    let mut from = 0.0;
    for k in (1..10).step_by(2) {
        let valley = search(&x, &y, from, Feature::Valley(Direction::Next));
        assert_extremum(valley, k);
        from = valley.x;
    }
}

/// The analytic crossing of `level` in `(low, high)`, by bisection.
fn analytic_crossing(level: f64, mut low: f64, mut high: f64) -> f64 {
    let rising = damped(high) > damped(low);
    for _ in 0..200 {
        let mid = 0.5 * (low + high);
        if (damped(mid) > level) == rising {
            high = mid;
        } else {
            low = mid;
        }
    }
    0.5 * (low + high)
}

#[test]
fn crossings_of_a_level_land_on_the_analytic_crossings_to_interpolation_accuracy() {
    let (x, y) = damped_sine();
    // Linear interpolation of a smooth curve errs by at most a quarter-sample
    // squared times its curvature over its slope.
    for level in [0.0, 0.3, -0.25] {
        let mut from = DT * 0.5;
        let mut count = 0;
        while let Ok(found) = find(
            &x,
            &y,
            from,
            Feature::Crossing {
                level,
                direction: Direction::Next,
            },
            0.0,
        ) {
            let index = x.partition_point(|t| *t < found.x);
            let analytic = analytic_crossing(level, x[index - 1], x[index]);
            let slope = (y[index] - y[index - 1]).abs() / DT;
            let curvature = (2.0 * PI * F + 1.0 / TAU).powi(2);
            let tolerance = 2.0 * curvature * DT * DT / (8.0 * slope) * (-x[index - 1] / TAU).exp();
            assert!(
                (found.x - analytic).abs() <= tolerance.max(1e-15),
                "level {level}: found {}, analytic {analytic}, tolerance {tolerance}",
                found.x
            );
            assert_eq!(found.y, level);
            from = found.x;
            count += 1;
        }
        assert!(count >= 4, "level {level} is crossed {count} times");
    }
}

/// The calculator's `cross(w, level, n)` for this curve.
fn calculator_cross(x: &[f64], y: &[f64], level: f64, n: usize) -> Option<f64> {
    let result = FunctionRegistry::dispatch(
        "cross",
        vec![
            RealValue::create_waveform(x.to_vec(), y.to_vec()),
            RealValue::Scalar(level),
            RealValue::Scalar(n as f64),
        ],
    );
    match result {
        Ok(RealValue::Scalar(value)) => Some(value),
        _ => None,
    }
}

#[test]
fn the_nth_next_crossing_is_the_calculators_cross_to_the_last_bit() {
    let (x, y) = damped_sine();
    for level in [0.0, 0.3, -0.25, 0.71] {
        let mut from = f64::NEG_INFINITY;
        for n in 1.. {
            let stepped = find(
                &x,
                &y,
                from,
                Feature::Crossing {
                    level,
                    direction: Direction::Next,
                },
                0.0,
            )
            .ok()
            .map(|found| found.x);
            assert_eq!(
                stepped.map(f64::to_bits),
                calculator_cross(&x, &y, level, n).map(f64::to_bits),
                "crossing {n} of {level}"
            );
            let Some(at) = stepped else { break };
            from = at;
        }
    }
}

#[test]
fn a_flat_top_is_one_peak_at_its_first_sample_and_an_at_level_run_one_crossing() {
    let x: Vec<f64> = (0..9).map(f64::from).collect();
    let y = [0.0, 1.0, 2.0, 2.0, 2.0, 1.0, 0.0, 0.0, 1.0];
    let peak = find(&x, &y, -1.0, Feature::Peak(Direction::Next), 0.0).unwrap();
    assert_eq!(peak, FeaturePoint { x: 2.0, y: 2.0 });
    let valley = find(&x, &y, 2.0, Feature::Valley(Direction::Next), 0.0).unwrap();
    assert_eq!(valley, FeaturePoint { x: 6.0, y: 0.0 }, "a flat bottom too");
    let level = [1.0, 1.0, 0.0, 0.0, -1.0];
    assert_eq!(crossings(&x[..5], &level, 0.0), vec![2.0]);
}

#[test]
fn chatter_on_a_settled_curve_is_not_a_peak() {
    let x: Vec<f64> = (0..200).map(f64::from).collect();
    let y: Vec<f64> = x
        .iter()
        .map(|t| {
            let settle = 1.0 - (-t / 10.0).exp();
            settle + 1e-9 * (t * 7.0).sin()
        })
        .collect();
    assert_eq!(
        find(
            &x,
            &y,
            0.0,
            Feature::Peak(Direction::Next),
            default_excursion(&y)
        ),
        Err(SearchRefusal::NotFound)
    );
    assert!(
        find(&x, &y, 0.0, Feature::Peak(Direction::Next), 0.0).is_ok(),
        "with no excursion the chatter is all peaks"
    );
}

#[test]
fn the_first_and_last_samples_are_never_peaks_but_can_be_the_maximum() {
    let x = [0.0, 1.0, 2.0, 3.0];
    let y = [3.0, 2.0, 1.0, 0.0];
    assert_eq!(
        find(&x, &y, -1.0, Feature::Peak(Direction::Next), 0.0),
        Err(SearchRefusal::NotFound)
    );
    assert_eq!(
        find(&x, &y, 2.0, Feature::Maximum, 0.0),
        Ok(FeaturePoint { x: 0.0, y: 3.0 })
    );
}

#[test]
fn samples_step_one_at_a_time_and_skip_holes() {
    let x = [0.0, 1.0, 2.0, 3.0];
    let y = [0.0, f64::NAN, 2.0, 3.0];
    let next = |from| {
        find(&x, &y, from, Feature::Sample(Direction::Next), 0.0)
            .map(|found| found.x)
            .ok()
    };
    assert_eq!(next(-0.5), Some(0.0));
    assert_eq!(next(0.0), Some(2.0), "the hole is not a sample");
    assert_eq!(next(2.4), Some(3.0));
    assert_eq!(next(3.0), None);
    assert_eq!(
        find(&x, &y, 2.0, Feature::Sample(Direction::Previous), 0.0).map(|found| found.x),
        Ok(0.0)
    );
}

#[test]
fn a_sweep_that_turns_back_or_holds_nothing_is_refused() {
    let x = [0.0, 1.0, 0.5];
    let y = [0.0, 1.0, 2.0];
    assert_eq!(
        find(&x, &y, 0.0, Feature::Maximum, 0.0),
        Err(SearchRefusal::Unordered { sample: 2 })
    );
    assert_eq!(
        find(&[0.0], &[f64::NAN], 0.0, Feature::Maximum, 0.0),
        Err(SearchRefusal::NoSamples)
    );
    assert!(
        SearchRefusal::Unordered { sample: 2 }
            .to_string()
            .contains("turns back at sample 2")
    );
}

#[test]
fn a_peak_marker_climbs_to_its_peak_and_finds_it_again_when_the_curve_moves() {
    let (x, y) = damped_sine();
    // Placed by eye a few samples off the third peak.
    let seed = extremum(4) + 7.0 * DT;
    let found = nearest(&x, &y, seed, Anchor::Peak).expect("the marker is on a hill");
    assert_extremum(found, 4);
    let valley = nearest(&x, &y, extremum(3) - 5.0 * DT, Anchor::Valley).unwrap();
    assert_extremum(valley, 3);

    // The result changes: the same circuit rings a little later. The marker
    // keeps its seed and lands on the peak as it now is.
    let delay = 9.0 * DT;
    let later: Vec<f64> = x.iter().map(|t| damped((t - delay).max(0.0))).collect();
    let moved = nearest(&x, &later, seed, Anchor::Peak).unwrap();
    assert!(
        (moved.x - (extremum(4) + delay)).abs() <= 0.5 * DT * (1.0 + 1e-9),
        "the peak moved with the curve: {} against {}",
        moved.x,
        extremum(4) + delay
    );
}

#[test]
fn a_crossing_marker_takes_the_crossing_of_its_level_nearest_it() {
    let (x, y) = damped_sine();
    let zeros: Vec<f64> = crossings(&x, &y, 0.0);
    let found = nearest(&x, &y, zeros[3] + 0.2 / F, Anchor::Crossing { level: 0.0 }).unwrap();
    assert_eq!(found.x, zeros[3]);
    assert_eq!(nearest(&x, &y, 0.0, Anchor::Crossing { level: 2.0 }), None);
}

#[test]
fn a_peak_marker_on_a_curve_that_only_rises_has_no_peak() {
    let x: Vec<f64> = (0..10).map(f64::from).collect();
    let y = x.clone();
    assert_eq!(nearest(&x, &y, 4.2, Anchor::Peak), None);
    assert_eq!(
        nearest(&x, &y, 4.2, Anchor::Valley),
        None,
        "nor a valley: the bottom is its first sample"
    );
}
