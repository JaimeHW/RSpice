//! Window Functions for FFT
//!
//! Commercial-grade windowing functions used in spectral analysis.
//! Each window trades off between frequency resolution and spectral leakage.

use std::{collections::TryReserveError, f64::consts::PI};

// =============================================================================
// Window Function Types
// =============================================================================

/// Window function type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum WindowFunction {
    /// Rectangular window (no windowing)
    Rectangular,
    /// Hanning (Hann) window - good general purpose
    #[default]
    Hanning,
    /// Hamming window - minimizes first sidelobe
    Hamming,
    /// Blackman window - low sidelobes
    Blackman,
    /// Blackman-Harris window - very low sidelobes
    BlackmanHarris,
    /// Flat-top window - accurate amplitude measurement
    FlatTop,
    /// Kaiser window with configurable beta
    Kaiser,
    /// Gaussian window
    Gaussian,
}

impl WindowFunction {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Rectangular => "Rectangular",
            Self::Hanning => "Hanning",
            Self::Hamming => "Hamming",
            Self::Blackman => "Blackman",
            Self::BlackmanHarris => "Blackman-Harris",
            Self::FlatTop => "Flat-Top",
            Self::Kaiser => "Kaiser",
            Self::Gaussian => "Gaussian",
        }
    }

    /// All window functions
    pub fn all() -> &'static [WindowFunction] {
        &[
            Self::Rectangular,
            Self::Hanning,
            Self::Hamming,
            Self::Blackman,
            Self::BlackmanHarris,
            Self::FlatTop,
            Self::Kaiser,
            Self::Gaussian,
        ]
    }
}

// =============================================================================
// Window Generation
// =============================================================================

/// Generate window coefficients after reserving their complete storage.
pub(super) fn try_generate_window(
    window_type: WindowFunction,
    length: usize,
) -> Result<Vec<f64>, TryReserveError> {
    match window_type {
        WindowFunction::Rectangular => rectangular_window(length),
        WindowFunction::Hanning => hanning_window(length),
        WindowFunction::Hamming => hamming_window(length),
        WindowFunction::Blackman => blackman_window(length),
        WindowFunction::BlackmanHarris => blackman_harris_window(length),
        WindowFunction::FlatTop => flat_top_window(length),
        WindowFunction::Kaiser => kaiser_window(length, 5.0),
        WindowFunction::Gaussian => gaussian_window(length, 2.5),
    }
}

#[cfg(test)]
fn generate_window(window_type: WindowFunction, length: usize) -> Vec<f64> {
    try_generate_window(window_type, length).expect("window test fixture allocation")
}

// =============================================================================
// Window Implementations
// =============================================================================

fn coefficient_vector(length: usize) -> Result<Vec<f64>, TryReserveError> {
    let mut coefficients = Vec::new();
    coefficients.try_reserve_exact(length)?;
    Ok(coefficients)
}

fn rectangular_window(length: usize) -> Result<Vec<f64>, TryReserveError> {
    let mut coefficients = coefficient_vector(length)?;
    coefficients.resize(length, 1.0);
    Ok(coefficients)
}

fn symmetric_window(
    length: usize,
    mut coefficient_at: impl FnMut(f64) -> f64,
) -> Result<Vec<f64>, TryReserveError> {
    let mut coefficients = coefficient_vector(length)?;
    if length == 0 {
        return Ok(coefficients);
    }
    if length == 1 {
        coefficients.push(1.0);
        return Ok(coefficients);
    }

    let denominator = (length - 1) as f64;
    for index in 0..length {
        coefficients.push(coefficient_at(index as f64 / denominator));
    }
    Ok(coefficients)
}

fn hanning_window(length: usize) -> Result<Vec<f64>, TryReserveError> {
    symmetric_window(length, |position| 0.5 * (1.0 - (2.0 * PI * position).cos()))
}

fn hamming_window(length: usize) -> Result<Vec<f64>, TryReserveError> {
    symmetric_window(length, |position| 0.54 - 0.46 * (2.0 * PI * position).cos())
}

fn blackman_window(length: usize) -> Result<Vec<f64>, TryReserveError> {
    let a0 = 0.42;
    let a1 = 0.5;
    let a2 = 0.08;

    symmetric_window(length, |position| {
        a0 - a1 * (2.0 * PI * position).cos() + a2 * (4.0 * PI * position).cos()
    })
}

fn blackman_harris_window(length: usize) -> Result<Vec<f64>, TryReserveError> {
    let a0 = 0.35875;
    let a1 = 0.48829;
    let a2 = 0.14128;
    let a3 = 0.01168;

    symmetric_window(length, |position| {
        a0 - a1 * (2.0 * PI * position).cos() + a2 * (4.0 * PI * position).cos()
            - a3 * (6.0 * PI * position).cos()
    })
}

fn flat_top_window(length: usize) -> Result<Vec<f64>, TryReserveError> {
    let a0 = 0.21557895;
    let a1 = 0.41663158;
    let a2 = 0.277263158;
    let a3 = 0.083578947;
    let a4 = 0.006947368;

    symmetric_window(length, |position| {
        a0 - a1 * (2.0 * PI * position).cos() + a2 * (4.0 * PI * position).cos()
            - a3 * (6.0 * PI * position).cos()
            + a4 * (8.0 * PI * position).cos()
    })
}

fn kaiser_window(length: usize, beta: f64) -> Result<Vec<f64>, TryReserveError> {
    let denom = bessel_i0(beta);

    symmetric_window(length, |position| {
        let centered = 2.0 * position - 1.0;
        let argument = beta * (1.0 - centered * centered).sqrt();
        bessel_i0(argument) / denom
    })
}

fn gaussian_window(length: usize, sigma: f64) -> Result<Vec<f64>, TryReserveError> {
    symmetric_window(length, |position| {
        let normalized = (2.0 * position - 1.0) / sigma;
        (-0.5 * normalized * normalized).exp()
    })
}

/// Modified Bessel function of first kind, order 0
fn bessel_i0(x: f64) -> f64 {
    let ax = x.abs();
    if ax < 3.75 {
        let y = (x / 3.75).powi(2);
        1.0 + y
            * (3.5156229
                + y * (3.0899424
                    + y * (1.2067492 + y * (0.2659732 + y * (0.0360768 + y * 0.0045813)))))
    } else {
        let y = 3.75 / ax;
        (ax.exp() / ax.sqrt())
            * (0.39894228
                + y * (0.01328592
                    + y * (0.00225319
                        + y * (-0.00157565
                            + y * (0.00916281
                                + y * (-0.02057706
                                    + y * (0.02635537 + y * (-0.01647633 + y * 0.00392377))))))))
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------
    // Analytic oracles
    //
    // A window is only ever used through two scalars: the coherent gain a
    // tone's amplitude has to be divided by, and the equivalent noise
    // bandwidth a noise density has to be divided by. Both have closed forms
    // in the window's own cosine-sum coefficients, and both are derived here
    // from those coefficients rather than from a table — so retuning a
    // window's coefficients moves the expected values with it, and mistyping
    // one does not.
    //
    // For `w[i] = a0 − a1·cos(2πx) + a2·cos(4πx) − …`, over a whole number of
    // periods: the mean is `a0`, and by Parseval the mean square is
    // `a0² + ½·Σ_{k≥1} aₖ²`. So
    //
    //     CG   = a0
    //     ENBW = (a0² + ½·Σ aₖ²) / a0²   bins.
    //
    // These windows are the *symmetric* form — the denominator is `N − 1`, so
    // one endpoint is repeated and the sums sit `O(1/N)` off the periodic
    // limit. Every comparison below is therefore against a long window, with
    // a tolerance the offset fits inside rather than an exact equality that
    // would be asserting the sampling artefact.
    // -------------------------------------------------------------------

    /// Long enough that the symmetric form's `O(1/N)` offset from the periodic
    /// limit — a few parts in 10⁵ at this length — sits an order of magnitude
    /// inside the tolerances used here, while those tolerances stay two orders
    /// of magnitude tighter than the gap between any two of these windows.
    const LONG: usize = 1 << 16;

    /// Cosine-sum coefficients, in the order the closed forms want them.
    fn cosine_sum_coefficients(window: WindowFunction) -> Option<&'static [f64]> {
        Some(match window {
            WindowFunction::Rectangular => &[1.0],
            WindowFunction::Hanning => &[0.5, 0.5],
            WindowFunction::Hamming => &[0.54, 0.46],
            WindowFunction::Blackman => &[0.42, 0.5, 0.08],
            WindowFunction::BlackmanHarris => &[0.35875, 0.48829, 0.14128, 0.01168],
            WindowFunction::FlatTop => &[
                0.21557895,
                0.41663158,
                0.277263158,
                0.083578947,
                0.006947368,
            ],
            // Not cosine sums: their closed forms are elsewhere in this module.
            WindowFunction::Kaiser | WindowFunction::Gaussian => return None,
        })
    }

    /// Coherent gain: the mean of the window, which is what an amplitude
    /// reading is divided by.
    fn coherent_gain(coefficients: &[f64]) -> f64 {
        coefficients.iter().sum::<f64>() / coefficients.len() as f64
    }

    /// Equivalent noise bandwidth in bins, which is what a noise density is
    /// divided by.
    fn noise_bandwidth_bins(coefficients: &[f64]) -> f64 {
        let sum = coefficients.iter().sum::<f64>();
        let squares = coefficients.iter().map(|c| c * c).sum::<f64>();
        coefficients.len() as f64 * squares / (sum * sum)
    }

    #[track_caller]
    fn assert_relative(actual: f64, expected: f64, tolerance: f64, what: &str) {
        let error = (actual - expected).abs() / expected.abs();
        assert!(
            error <= tolerance,
            "{what}: {actual} vs {expected}, relative error {error:e}"
        );
    }

    /// Coherent gain is `a0`, for every cosine-sum window at once.
    ///
    /// This is the scalar an amplitude reading is divided by, so an error here
    /// is a spectrum whose peaks are the wrong height — and one that looks
    /// perfectly plausible.
    #[test]
    fn cosine_sum_coherent_gain_is_the_constant_term() {
        for &window in WindowFunction::all() {
            let Some(a) = cosine_sum_coefficients(window) else {
                continue;
            };
            assert_relative(
                coherent_gain(&generate_window(window, LONG)),
                a[0],
                5.0e-4,
                &format!("{} coherent gain", window.display_name()),
            );
        }
    }

    /// ENBW is `(a0² + ½·Σ_{k≥1} aₖ²) / a0²`, derived from the coefficients.
    ///
    /// The values this produces are the published ones — Hann 1.5 bins,
    /// Hamming 1.3628, Blackman 1.7268, Blackman-Harris 2.0044, flat-top
    /// 3.7702 — but none of them is written down here. The only assertion the
    /// viewer previously carried was that Blackman-Harris lands somewhere in
    /// `(1, 2.1)` bins, which every one of these windows but the flat-top
    /// satisfies.
    #[test]
    fn cosine_sum_noise_bandwidth_follows_from_the_coefficients() {
        for &window in WindowFunction::all() {
            let Some(a) = cosine_sum_coefficients(window) else {
                continue;
            };
            let mean_square = a[0] * a[0] + 0.5 * a[1..].iter().map(|c| c * c).sum::<f64>();
            assert_relative(
                noise_bandwidth_bins(&generate_window(window, LONG)),
                mean_square / (a[0] * a[0]),
                5.0e-4,
                &format!("{} ENBW", window.display_name()),
            );
        }
    }

    /// The two scalars are not interchangeable, and a window that returned one
    /// for the other would still pass a "positive and finite" check. The
    /// rectangular window is the only one they agree on, which is what makes
    /// it the reference: it is unity for both by definition.
    #[test]
    fn the_two_scalars_agree_only_for_the_rectangular_window() {
        for &window in WindowFunction::all() {
            let coefficients = generate_window(window, LONG);
            let (gain, bandwidth) = (
                coherent_gain(&coefficients),
                noise_bandwidth_bins(&coefficients),
            );
            if window == WindowFunction::Rectangular {
                assert_relative(gain, 1.0, 1.0e-12, "rectangular coherent gain");
                assert_relative(bandwidth, 1.0, 1.0e-12, "rectangular ENBW");
            } else {
                assert!(
                    gain < 1.0 && bandwidth > 1.0,
                    "{} tapers, so it must lose coherent gain ({gain}) and widen its \
                     noise bandwidth ({bandwidth})",
                    window.display_name()
                );
            }
        }
    }

    /// Every window is symmetric about its centre.
    ///
    /// Asymmetry is a phase error the magnitude spectrum hides completely, so
    /// nothing downstream would report it.
    #[test]
    fn every_window_is_symmetric_about_its_centre() {
        for &window in WindowFunction::all() {
            for length in [2usize, 3, 16, 17, 1024] {
                let w = generate_window(window, length);
                assert_eq!(w.len(), length, "{}", window.display_name());
                for index in 0..length {
                    assert!(
                        (w[index] - w[length - 1 - index]).abs() < 1.0e-12,
                        "{} of length {length} is asymmetric at {index}",
                        window.display_name()
                    );
                }
            }
        }
    }

    /// Degenerate lengths produce a window, not a panic or a silent zero.
    ///
    /// A zero-gain window divides a spectrum by nothing.
    #[test]
    fn degenerate_lengths_produce_a_usable_window() {
        for &window in WindowFunction::all() {
            assert!(generate_window(window, 0).is_empty());
            assert_eq!(
                generate_window(window, 1),
                vec![1.0],
                "{} of length 1",
                window.display_name()
            );
        }
    }

    /// The Kaiser window's `I₀` is a polynomial approximation. Its oracle is
    /// the defining series `I₀(x) = Σ (x²/4)^k / (k!)²`, summed here to
    /// convergence, which shares no coefficient with the approximation.
    ///
    /// The approximation is Abramowitz & Stegun 9.8.1/9.8.2, quoted to about
    /// 2e-7 relative; that is what it is held to, so a mistyped coefficient
    /// fails while the approximation itself passes.
    #[test]
    fn the_kaiser_bessel_approximation_matches_the_defining_series() {
        let series = |x: f64| {
            let mut term = 1.0_f64;
            let mut total = 1.0_f64;
            for k in 1..200 {
                term *= (x * x / 4.0) / (f64::from(k) * f64::from(k));
                total += term;
                if term <= total * 1.0e-18 {
                    break;
                }
            }
            total
        };
        for step in 0..=120 {
            let x = f64::from(step) * 0.1;
            assert_relative(
                bessel_i0(x),
                series(x),
                2.0e-7,
                &format!("I0({x}) against its series"),
            );
        }
    }

    /// The Kaiser window is `I₀(β√(1−x²)) / I₀(β)` on `x ∈ [−1, 1]`, so it is
    /// 1 at the centre and `1/I₀(β)` at the ends. `generate_window` fixes
    /// β = 5.
    #[test]
    fn the_kaiser_window_is_its_bessel_ratio_at_the_centre_and_the_ends() {
        const BETA: f64 = 5.0;
        let w = generate_window(WindowFunction::Kaiser, 1_025);
        assert_relative(w[512], 1.0, 1.0e-12, "Kaiser centre");
        assert_relative(w[0], 1.0 / bessel_i0(BETA), 1.0e-12, "Kaiser end");
    }

    /// The Gaussian window is `exp(−x²/2)` over `x = (i − c)/(σ·c)`, so with
    /// `generate_window`'s σ = 2.5 it falls to `exp(−1/(2σ²))` at the ends.
    #[test]
    fn the_gaussian_window_falls_to_its_closed_form_at_the_ends() {
        const SIGMA: f64 = 2.5;
        let w = generate_window(WindowFunction::Gaussian, 1_025);
        assert_relative(w[512], 1.0, 1.0e-12, "Gaussian centre");
        assert_relative(
            w[0],
            (-0.5 / (SIGMA * SIGMA)).exp(),
            1.0e-12,
            "Gaussian end",
        );
    }
}
