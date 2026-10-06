//! One window law for finite samples and analytic distributional observations.
//!
//! Express cosine sums as nonnegative polynomials in sin(pi*x). This avoids
//! subtracting nearly equal constants at a taper's zero endpoints.
use super::*;

pub(super) struct Polynomial {
    pub sine: [Value; 7],
    pub triangle: Value,
}

pub(super) fn polynomial(window: FftWindow) -> Option<Polynomial> {
    let mut sine = [0.0; 7];
    let mut triangle = 0.0;
    match window {
        FftWindow::Rectangular => sine[0] = 1.0,
        FftWindow::Bartlett => triangle = 1.0,
        FftWindow::BartlettHann => {
            triangle = 0.24;
            sine[2] = 0.76;
        }
        FftWindow::Hamming => {
            sine[0] = 0.08;
            sine[2] = 0.92;
        }
        FftWindow::Hann | FftWindow::Cosine2 => sine[2] = 1.0,
        FftWindow::Blackman67Db => {
            sine[0] = 0.0049;
            sine[2] = 0.36134;
            sine[4] = 0.63376;
        }
        FftWindow::Blackman => {
            sine[2] = 0.36;
            sine[4] = 0.64;
        }
        FftWindow::BlackmanHarris => {
            sine[0] = 0.00006;
            sine[2] = 0.05658;
            sine[4] = 0.5696;
            sine[6] = 0.37376;
        }
        FftWindow::Nuttall => {
            sine[0] = 0.0003628;
            sine[2] = 0.0770988;
            sine[4] = 0.5820232;
            sine[6] = 0.3405152;
        }
        FftWindow::HalfCycleSine => sine[1] = 1.0,
        FftWindow::HalfCycleSine3 => sine[3] = 1.0,
        FftWindow::HalfCycleSine6 => sine[6] = 1.0,
        FftWindow::Cosine4 => sine[4] = 1.0,
        FftWindow::Gaussian | FftWindow::Kaiser => return None,
    }
    Some(Polynomial { sine, triangle })
}

/// x is on the normalized window support [0,1]. Reflection preserves small
/// distances from either endpoint; the exact symmetry axes stay exact.
pub(super) fn sine_cosine(x: Value) -> (Value, Value) {
    if x == 0.0 {
        return (0.0, 1.0);
    }
    if x == 0.5 {
        return (1.0, 0.0);
    }
    if x == 1.0 {
        return (0.0, -1.0);
    }
    let (sine, cosine) = (PI * x.min(1.0 - x)).sin_cos();
    (sine, if x > 0.5 { -cosine } else { cosine })
}

pub(super) fn coefficient(window: FftWindow, x: Value, alpha: Value) -> Value {
    if let Some(poly) = polynomial(window) {
        let sine = sine_cosine(x).0;
        return poly
            .sine
            .iter()
            .rev()
            .fold(0.0, |sum, coefficient| sum * sine + coefficient)
            + poly.triangle * 2.0 * x.min(1.0 - x);
    }
    match window {
        FftWindow::Gaussian => (-0.5 * (alpha * (2.0 * x - 1.0)).powi(2)).exp(),
        FftWindow::Kaiser => {
            let radial = (4.0 * x * (1.0 - x)).max(0.0).sqrt();
            modified_bessel_i0(alpha * radial) / modified_bessel_i0(alpha)
        }
        _ => unreachable!("polynomial windows returned above"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_endpoint_tapers_retain_small_nonzero_weights() {
        for x in [1e-10_f64, 1.0 - 1e-10] {
            let distance: Value = x.min(1.0 - x);
            for (window, power, leading) in [
                (FftWindow::Hann, 2, 1.0),
                (FftWindow::Blackman, 2, 0.36),
                (FftWindow::Cosine4, 4, 1.0),
                (FftWindow::HalfCycleSine6, 6, 1.0),
            ] {
                let expected = leading * (PI * distance).powi(power);
                let actual = coefficient(window, x, 3.0);
                assert!(
                    (actual / expected - 1.0).abs() < 2e-14,
                    "{window:?}: {actual} vs {expected}"
                );
                assert_eq!(coefficient(window, 0.0, 3.0), 0.0);
                assert_eq!(coefficient(window, 1.0, 3.0), 0.0);
            }
        }
    }
}
