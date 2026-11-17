//! `$random`, `$dist_*` (IEEE 1364-2005 section 17.9) and the real
//! `$rdist_*` of Verilog-AMS (LRM 2.4 section 9.13), shared by the analog
//! runtimes and the digital kernel.
//!
//! The `$random` and `$dist_*` generators are section 17.9.3's reference
//! algorithms, implemented from the standard's text: the 32-bit linear
//! congruential seed update `seed = 69069 * seed + 1`, a uniform deviate built
//! from the seed's top 23 bits as an IEEE single, and the distribution
//! functions on top of it. Every simulator that follows the text — Icarus
//! does — produces the same sequence for the same seed, so random stimulus is
//! reproducible across tools as well as across runs.
//!
//! The standard's C uses a 32-bit `long`; every seed operation here is done in
//! `i32` with wrapping arithmetic to match it, and every `(long)` cast of a
//! double is Rust's saturating `as i32`, which is what the x86 conversion the
//! reference relies on produces for the only out-of-range case the algorithms
//! can reach.
//!
//! LRM 2.4 table 9-26 maps each `$rdist_*` to the same C function. The C
//! takes `long` parameters; the real functions pass their real parameters
//! through the same arithmetic unrounded and return the real result the C
//! computes before `rtl_dist_*` rounds it. A degree of freedom or a stage
//! count steers a loop, so it is rounded to an integer first, as a real
//! assigned to a Verilog integer is.

/// Section 17.9.3's `uniform`: one deviate in `[start, end)`, advancing the
/// seed. A zero seed is first replaced by the standard's fixed nonzero value.
fn uniform(seed: &mut i32, start: i32, end: i32) -> f64 {
    uniform_real(seed, f64::from(start), f64::from(end))
}

/// [`uniform`] over real bounds: the generalization `$rdist_uniform` uses.
fn uniform_real(seed: &mut i32, start: f64, end: f64) -> f64 {
    const D: f64 = 0.000_000_119_209_289_550_781_25;
    if *seed == 0 {
        *seed = 259_341_593;
    }
    let (a, b) = if start >= end {
        (0.0, 2_147_483_647.0)
    } else {
        (start, end)
    };
    *seed = seed.wrapping_mul(69_069).wrapping_add(1);
    let bits = ((*seed as u32) >> 9) | 0x3f80_0000;
    let mut c = f64::from(f32::from_bits(bits));
    c += c * D;
    (b - a) * (c - 1.0) + a
}

/// The standard's `(long) r` for a non-negative `r` and `(long)(r - 1)` for a
/// negative one.
fn floor_cast(r: f64) -> i32 {
    if r >= 0.0 { r as i32 } else { (r - 1.0) as i32 }
}

/// Round half away from zero, the way every `rtl_dist_*` returns its value.
fn round_cast(r: f64) -> i32 {
    if r >= 0.0 {
        (r + 0.5) as i32
    } else {
        -((-r + 0.5) as i32)
    }
}

/// `rtl_dist_uniform`: an integer uniformly distributed over `[start, end]`.
pub fn dist_uniform(seed: &mut i32, start: i32, end: i32) -> i32 {
    if start >= end {
        return start;
    }
    if end != i32::MAX {
        let end = end + 1;
        let i = floor_cast(uniform(seed, start, end));
        i.max(start).min(end - 1)
    } else if start != i32::MIN {
        let start = start - 1;
        let i = floor_cast(uniform(seed, start, end) + 1.0);
        if i <= start { start + 1 } else { i.min(end) }
    } else {
        let r = (uniform(seed, start, end) + 2_147_483_648.0) / 4_294_967_295.0;
        floor_cast(r * 4_294_967_296.0 - 2_147_483_648.0)
    }
}

/// `$random`: the full signed 32-bit range.
pub fn random(seed: &mut i32) -> i32 {
    dist_uniform(seed, i32::MIN, i32::MAX)
}

/// The polar Box-Muller method, as section 17.9.3 writes it.
fn normal(seed: &mut i32, mean: f64, deviation: f64) -> f64 {
    let mut v1 = 0.0;
    let mut s = 1.0;
    while s >= 1.0 || s == 0.0 {
        v1 = uniform(seed, -1, 1);
        let v2 = uniform(seed, -1, 1);
        s = v1 * v1 + v2 * v2;
    }
    let s = v1 * (-2.0 * s.ln() / s).sqrt();
    s * deviation + mean
}

fn exponential(seed: &mut i32, mean: f64) -> f64 {
    let n = uniform(seed, 0, 1);
    if n != 0.0 { -n.ln() * mean } else { n }
}

fn poisson(seed: &mut i32, mean: f64) -> i32 {
    let mut n = 0;
    let p = (-mean).exp();
    let mut q = uniform(seed, 0, 1);
    while p < q {
        n += 1;
        q *= uniform(seed, 0, 1);
    }
    n
}

fn chi_square(seed: &mut i32, degrees: i32) -> f64 {
    let mut x = if degrees % 2 != 0 {
        let x = normal(seed, 0.0, 1.0);
        x * x
    } else {
        0.0
    };
    let mut k = 2;
    while k <= degrees {
        x += 2.0 * exponential(seed, 1.0);
        k += 2;
    }
    x
}

fn student_t(seed: &mut i32, degrees: i32) -> f64 {
    let chi2 = chi_square(seed, degrees);
    let root = (chi2 / f64::from(degrees)).sqrt();
    normal(seed, 0.0, 1.0) / root
}

fn erlangian(seed: &mut i32, k: i32, mean: f64) -> f64 {
    let mut x = 1.0;
    for _ in 1..=k {
        x *= uniform(seed, 0, 1);
    }
    -mean * x.ln() / f64::from(k)
}

/// The outcome of a `$dist_*` call: a value, or the standard's warning and 0
/// for a parameter outside the distribution's domain (the seed is unchanged).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Distribution {
    Value(i32),
    Invalid(&'static str),
}

/// The outcome of a `$rdist_*` call: a value, or the error LRM 2.4 section
/// 9.13.2 requires for a parameter outside the distribution's domain.
pub type RealDistribution = Result<f64, &'static str>;

pub fn dist_normal(seed: &mut i32, mean: i32, deviation: i32) -> Distribution {
    Distribution::Value(round_cast(normal(
        seed,
        f64::from(mean),
        f64::from(deviation),
    )))
}

pub fn dist_exponential(seed: &mut i32, mean: i32) -> Distribution {
    if mean <= 0 {
        return Distribution::Invalid("Exponential distribution must have a positive mean");
    }
    Distribution::Value(round_cast(exponential(seed, f64::from(mean))))
}

pub fn dist_poisson(seed: &mut i32, mean: i32) -> Distribution {
    if mean <= 0 {
        return Distribution::Invalid("Poisson distribution must have a positive mean");
    }
    Distribution::Value(poisson(seed, f64::from(mean)))
}

pub fn dist_chi_square(seed: &mut i32, degrees: i32) -> Distribution {
    if degrees <= 0 {
        return Distribution::Invalid(
            "Chi_square distribution must have positive degree of freedom",
        );
    }
    Distribution::Value(round_cast(chi_square(seed, degrees)))
}

pub fn dist_t(seed: &mut i32, degrees: i32) -> Distribution {
    if degrees <= 0 {
        return Distribution::Invalid("t distribution must have positive degree of freedom");
    }
    Distribution::Value(round_cast(student_t(seed, degrees)))
}

pub fn dist_erlang(seed: &mut i32, k: i32, mean: i32) -> Distribution {
    if k <= 0 {
        return Distribution::Invalid("k-stage erlangian distribution must have positive k");
    }
    Distribution::Value(round_cast(erlangian(seed, k, f64::from(mean))))
}

/// The Verilog real-to-integer rounding of a loop-count parameter.
fn count(value: f64) -> i32 {
    if value.is_finite() {
        round_cast(value)
    } else {
        0
    }
}

/// `$rdist_uniform`: a real uniformly distributed over `[start, end)`.
pub fn rdist_uniform(seed: &mut i32, start: f64, end: f64) -> f64 {
    uniform_real(seed, start, end)
}

/// `$rdist_normal`.
pub fn rdist_normal(seed: &mut i32, mean: f64, deviation: f64) -> f64 {
    normal(seed, mean, deviation)
}

/// `$rdist_exponential`.
pub fn rdist_exponential(seed: &mut i32, mean: f64) -> RealDistribution {
    if mean.is_nan() || mean <= 0.0 {
        return Err("$rdist_exponential's mean must be greater than zero");
    }
    Ok(exponential(seed, mean))
}

/// `$rdist_poisson`: the count, as a real.
pub fn rdist_poisson(seed: &mut i32, mean: f64) -> RealDistribution {
    if mean.is_nan() || mean <= 0.0 {
        return Err("$rdist_poisson's mean must be greater than zero");
    }
    Ok(f64::from(poisson(seed, mean)))
}

/// `$rdist_chi_square`.
pub fn rdist_chi_square(seed: &mut i32, degrees: f64) -> RealDistribution {
    let degrees = count(degrees);
    if degrees <= 0 {
        return Err("$rdist_chi_square's degree of freedom must be greater than zero");
    }
    Ok(chi_square(seed, degrees))
}

/// `$rdist_t`.
pub fn rdist_t(seed: &mut i32, degrees: f64) -> RealDistribution {
    let degrees = count(degrees);
    if degrees <= 0 {
        return Err("$rdist_t's degree of freedom must be greater than zero");
    }
    Ok(student_t(seed, degrees))
}

/// `$rdist_erlang`.
pub fn rdist_erlang(seed: &mut i32, k: f64, mean: f64) -> RealDistribution {
    let k = count(k);
    if k <= 0 {
        return Err("$rdist_erlang's k-stage must be greater than zero");
    }
    if mean.is_nan() || mean <= 0.0 {
        return Err("$rdist_erlang's mean must be greater than zero");
    }
    Ok(erlangian(seed, k, mean))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// With whole parameters a real distribution draws the deviate its
    /// integer twin rounds, from the same seed stream (LRM 2.4 table 9-26).
    #[test]
    fn real_distributions_are_the_integer_ones_before_rounding() {
        for start in [-7, 0, 5, 123_456] {
            let (mut real_seed, mut integer_seed) = (start, start);
            for _ in 0..64 {
                let real = rdist_normal(&mut real_seed, 3.0, 40.0);
                assert_eq!(
                    dist_normal(&mut integer_seed, 3, 40),
                    Distribution::Value(round_cast(real))
                );
                assert_eq!(real_seed, integer_seed);
                let real = rdist_exponential(&mut real_seed, 9.0).unwrap();
                assert_eq!(
                    dist_exponential(&mut integer_seed, 9),
                    Distribution::Value(round_cast(real))
                );
                let real = rdist_poisson(&mut real_seed, 4.0).unwrap();
                assert_eq!(
                    dist_poisson(&mut integer_seed, 4),
                    Distribution::Value(real as i32)
                );
                let real = rdist_chi_square(&mut real_seed, 3.0).unwrap();
                assert_eq!(
                    dist_chi_square(&mut integer_seed, 3),
                    Distribution::Value(round_cast(real))
                );
                let real = rdist_t(&mut real_seed, 5.0).unwrap();
                assert_eq!(
                    dist_t(&mut integer_seed, 5),
                    Distribution::Value(round_cast(real))
                );
                let real = rdist_erlang(&mut real_seed, 3.0, 6.0).unwrap();
                assert_eq!(
                    dist_erlang(&mut integer_seed, 3, 6),
                    Distribution::Value(round_cast(real))
                );
                assert_eq!(real_seed, integer_seed);
            }
        }
    }

    /// The real distributions' sample moments over many draws lie within
    /// five standard errors of the distributions' own: uniform on `[a, b)`
    /// has mean `(a+b)/2` and variance `(b-a)^2/12`, normal `m` and `s^2`,
    /// exponential `m` and `m^2`, Poisson `m` and `m`.
    #[test]
    fn real_distributions_have_their_moments() {
        const DRAWS: usize = 200_000;
        let check = |name: &str, draw: &mut dyn FnMut() -> f64, mean: f64, variance: f64| {
            let samples: Vec<f64> = (0..DRAWS).map(|_| draw()).collect();
            let n = DRAWS as f64;
            let sample_mean = samples.iter().sum::<f64>() / n;
            let sample_variance = samples
                .iter()
                .map(|x| (x - sample_mean).powi(2))
                .sum::<f64>()
                / (n - 1.0);
            let mean_error = (variance / n).sqrt();
            assert!(
                (sample_mean - mean).abs() < 5.0 * mean_error,
                "{name}: mean {sample_mean} vs {mean}"
            );
            // The variance's standard error, bounded for these
            // light-tailed distributions by twice the variance over sqrt(n).
            assert!(
                (sample_variance - variance).abs() < 5.0 * 2.0 * variance / n.sqrt(),
                "{name}: variance {sample_variance} vs {variance}"
            );
        };
        let mut seed = 11;
        check(
            "uniform",
            &mut || rdist_uniform(&mut seed, -2.0, 6.0),
            2.0,
            64.0 / 12.0,
        );
        check(
            "normal",
            &mut || rdist_normal(&mut seed, 1.5, 0.25),
            1.5,
            0.0625,
        );
        check(
            "exponential",
            &mut || rdist_exponential(&mut seed, 2.0).unwrap(),
            2.0,
            4.0,
        );
        check(
            "poisson",
            &mut || rdist_poisson(&mut seed, 3.5).unwrap(),
            3.5,
            3.5,
        );
        let replay: Vec<f64> = {
            let mut seed = 42;
            (0..8).map(|_| rdist_normal(&mut seed, 0.0, 1.0)).collect()
        };
        let again: Vec<f64> = {
            let mut seed = 42;
            (0..8).map(|_| rdist_normal(&mut seed, 0.0, 1.0)).collect()
        };
        assert_eq!(replay, again, "a seed fixes the stream");
    }

    #[test]
    fn real_distribution_domains_are_errors() {
        let mut seed = 3;
        assert!(rdist_exponential(&mut seed, 0.0).is_err());
        assert!(rdist_poisson(&mut seed, -1.0).is_err());
        assert!(rdist_chi_square(&mut seed, 0.4).is_err());
        assert!(rdist_t(&mut seed, f64::NAN).is_err());
        assert!(rdist_erlang(&mut seed, 2.0, 0.0).is_err());
        assert_eq!(seed, 3, "a refused draw leaves the seed");
    }
}
