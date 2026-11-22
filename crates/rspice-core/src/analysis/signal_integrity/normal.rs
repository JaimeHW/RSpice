//! The standard normal tail and its inverse, which every bit-error figure of
//! the eye is quoted through.
//!
//! A bit-error rate of 1e-12 sits seven standard deviations out, where a
//! one-term asymptote is still a per cent off, so the tail is the
//! complementary error function itself at every Q.

/// Probability that a standard normal variate exceeds `x`: Q(x) = ½·erfc(x/√2).
#[must_use]
pub fn q_function(x: f64) -> f64 {
    0.5 * libm::erfc(x / std::f64::consts::SQRT_2)
}

/// The `x` whose upper tail [`q_function`] is `probability`, for a
/// probability in `(0, 1)`; `None` outside it.
///
/// Acklam's rational approximation, polished by one Halley step on the
/// tail itself, which leaves it within a few parts in 10^15 of the exact
/// quantile from the centre out past Q = 37.
#[must_use]
pub fn inverse_q_function(probability: f64) -> Option<f64> {
    if !(probability > 0.0 && probability < 1.0) {
        return None;
    }
    // Acklam's coefficients for the lower-tail quantile.
    const A: [f64; 6] = [
        -3.969_683_028_665_376e1,
        2.209_460_984_245_205e2,
        -2.759_285_104_469_687e2,
        1.383_577_518_672_69e2,
        -3.066_479_806_614_716e1,
        2.506_628_277_459_239,
    ];
    const B: [f64; 5] = [
        -5.447_609_879_822_406e1,
        1.615_858_368_580_409e2,
        -1.556_989_798_598_866e2,
        6.680_131_188_771_972e1,
        -1.328_068_155_288_572e1,
    ];
    const C: [f64; 6] = [
        -7.784_894_002_430_293e-3,
        -3.223_964_580_411_365e-1,
        -2.400_758_277_161_838,
        -2.549_732_539_343_734,
        4.374_664_141_464_968,
        2.938_163_982_698_783,
    ];
    const D: [f64; 4] = [
        7.784_695_709_041_462e-3,
        3.224_671_290_700_398e-1,
        2.445_134_137_142_996,
        3.754_408_661_907_416,
    ];
    const LOW: f64 = 0.024_25;

    // The lower-tail quantile of `lower`, so that Q(x) = probability means
    // x = −quantile(probability).
    let lower = probability;
    let quantile = if lower < LOW {
        let q = (-2.0 * lower.ln()).sqrt();
        (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    } else if lower <= 1.0 - LOW {
        let q = lower - 0.5;
        let r = q * q;
        (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r + A[5]) * q
            / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0)
    } else {
        let q = (-2.0 * (1.0 - lower).ln()).sqrt();
        -(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    };
    let mut x = -quantile;
    // One Halley step on Q(x) − probability, whose derivative is −φ(x).
    let density = (-0.5 * x * x).exp() / (2.0 * std::f64::consts::PI).sqrt();
    if density > 0.0 {
        let error = q_function(x) - probability;
        let step = error / density;
        x += step / (1.0 + 0.5 * x * step);
    }
    x.is_finite().then_some(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Q at the points the 11-18 audit checked, against scipy's
    /// `norm.sf`: the old asymptote read 5.8222e-16 at Q = 8.01 and
    /// 7.6946e-24 at Q = 10.
    #[test]
    fn the_tail_is_the_complementary_error_function_at_every_q() {
        for (q, expected) in [
            (1.0, 1.586_552_539_314_570_7e-1),
            (3.0, 1.349_898_031_630_093_3e-3),
            (7.0, 1.279_812_543_885_835e-12),
            (8.0, 6.220_960_574_271_74e-16),
            (8.01, 5.735_422_180_258_049e-16),
            (10.0, 7.619_853_024_160_47e-24),
            (12.0, 1.776_482_112_077_653e-33),
        ] {
            let got = q_function(q);
            assert!(
                ((got - expected) / expected).abs() < 1e-12,
                "Q({q}) = {got:e}, not {expected:e}"
            );
        }
    }

    #[test]
    fn the_inverse_returns_the_q_it_was_given() {
        for q in [
            0.0,
            0.3,
            1.0,
            2.5,
            5.0,
            7.034_483_825_301_131,
            9.0,
            14.0,
            -1.7,
        ] {
            let back = inverse_q_function(q_function(q)).expect("inside (0, 1)");
            assert!((back - q).abs() < 1e-9, "Q⁻¹(Q({q})) = {back}");
        }
        // The Q a 1e-12 error rate sits at, as quoted in every jitter budget.
        let at_1e12 = inverse_q_function(1e-12).expect("inside (0, 1)");
        assert!((at_1e12 - 7.034_483_825_301_131).abs() < 1e-9, "{at_1e12}");
        assert_eq!(inverse_q_function(0.0), None);
        assert_eq!(inverse_q_function(1.0), None);
    }
}
