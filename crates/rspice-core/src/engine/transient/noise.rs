//! Transient-noise expansion: TRNOISE sources become deterministic,
//! seeded PWL sample trains before circuit construction.
//!
//! Matching ngspice's trnoise engine:
//! - White noise: independent Gaussian samples of RMS amplitude `NA` every
//!   `NT` seconds, linearly interpolated between samples.
//! - 1/f^alpha noise: a white Gaussian sequence shaped by the Kasdin
//!   fractional-integration filter `h[0]=1, h[k]=h[k-1]*(k-1+alpha/2)/k`,
//!   scaled by `NAMP` — evaluated by FFT convolution (rustfft), the same
//!   construction ngspice's `f_alpha`/`fft` path uses.
//!
//! Expansion (instead of a stateful runtime source) buys three properties
//! for free: PWL evaluation and breakpoint scheduling already exist and are
//! battle-tested; the sample train is born deterministic (seeded by the
//! netlist's `.options seed` plus a stable per-source hash), so reruns are
//! bit-identical; and every analysis that consumes the netlist sees one
//! consistent waveform.

#![allow(clippy::needless_range_loop)]

use crate::Value;
use crate::netlist::{Element, ElementKind, Netlist, SourceSpec};

/// Hard cap on generated samples per source. 4M samples is ~64 MB of PWL
/// points — beyond that the deck should raise NT rather than the simulator
/// silently degrading the spectrum.
const MAX_NOISE_SAMPLES: usize = 1 << 22;

/// Replace every TRNOISE source spec with a generated PWL sample train
/// covering `[0, tstop]`. Returns `None` when the netlist contains no
/// TRNOISE source (the common case — zero cost, no clone).
pub(crate) fn expand_transient_noise(
    netlist: &Netlist,
    tstop: Value,
) -> Result<Option<Netlist>, String> {
    let has_noise = netlist
        .elements
        .iter()
        .any(|element| element_trnoise(element).is_some());
    if !has_noise {
        return Ok(None);
    }

    let base_seed = netlist.options.seed.unwrap_or(0x5EED_0001);
    let mut expanded = netlist.clone();
    for element in &mut expanded.elements {
        let Some(_) = element_trnoise(element) else {
            continue;
        };
        // Per-source stream: stable name hash mixed into the netlist seed so
        // every noise source draws an independent, reproducible sequence.
        let seed = base_seed ^ fnv1a(&element.name.to_ascii_uppercase());
        let name = element.name.clone();
        match &mut element.kind {
            ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                replace_trnoise(spec, tstop, seed, &name)?;
            }
            _ => {}
        }
    }
    Ok(Some(expanded))
}

fn element_trnoise(element: &Element) -> Option<()> {
    match &element.kind {
        ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
            spec_contains_trnoise(spec).then_some(())
        }
        _ => None,
    }
}

fn spec_contains_trnoise(spec: &SourceSpec) -> bool {
    match spec {
        SourceSpec::TrNoise { .. } => true,
        SourceSpec::DcTransient { transient, .. } | SourceSpec::DcAcTransient { transient, .. } => {
            spec_contains_trnoise(transient)
        }
        _ => false,
    }
}

fn replace_trnoise(
    spec: &mut SourceSpec,
    tstop: Value,
    seed: u64,
    name: &str,
) -> Result<(), String> {
    match spec {
        SourceSpec::TrNoise {
            na,
            nt,
            nalpha,
            namp,
        } => {
            let points = generate_noise_points(*na, *nt, *nalpha, *namp, tstop, seed, name)?;
            *spec = SourceSpec::Pwl { points };
            Ok(())
        }
        SourceSpec::DcTransient { transient, .. } | SourceSpec::DcAcTransient { transient, .. } => {
            replace_trnoise(transient, tstop, seed, name)
        }
        _ => Ok(()),
    }
}

/// Generate the noise sample train as PWL points on the `k*NT` grid.
fn generate_noise_points(
    na: Value,
    nt: Value,
    nalpha: Value,
    namp: Value,
    tstop: Value,
    seed: u64,
    name: &str,
) -> Result<Vec<(Value, Value)>, String> {
    if na == 0.0 && namp == 0.0 {
        return Ok(vec![(0.0, 0.0), (tstop.max(1e-12), 0.0)]);
    }
    if !(nt.is_finite() && nt > 0.0) {
        return Err(format!(
            "TRNOISE source '{}' requires a positive sample interval NT",
            name
        ));
    }

    // One sample past tstop so interpolation never extrapolates.
    let n = (tstop / nt).ceil() as usize + 2;
    if n > MAX_NOISE_SAMPLES {
        return Err(format!(
            "TRNOISE source '{}' would need {} samples (tstop/NT); the limit is {}. \
             Raise NT or shorten the transient.",
            name, n, MAX_NOISE_SAMPLES
        ));
    }

    let mut rng = SplitMix64::new(seed);
    let mut samples = vec![0.0f64; n];

    if na != 0.0 {
        for sample in samples.iter_mut() {
            *sample += na * rng.gaussian();
        }
    }

    if namp != 0.0 {
        let flicker = kasdin_one_over_f(n, nalpha, namp, &mut rng);
        for (sample, f) in samples.iter_mut().zip(flicker) {
            *sample += f;
        }
    }

    Ok(samples
        .into_iter()
        .enumerate()
        .map(|(k, v)| (k as Value * nt, v))
        .collect())
}

/// Kasdin 1/f^alpha sequence: white Gaussian sequence convolved with the
/// fractional-integration impulse response, via FFT (O(n log n)).
fn kasdin_one_over_f(n: usize, alpha: Value, amplitude: Value, rng: &mut SplitMix64) -> Vec<Value> {
    use rustfft::{FftPlanner, num_complex::Complex};

    let m = (2 * n).next_power_of_two();
    let mut h = vec![Complex::new(0.0f64, 0.0); m];
    let mut w = vec![Complex::new(0.0f64, 0.0); m];

    // Impulse response of the fractional integrator.
    let mut hk = 1.0f64;
    h[0].re = 1.0;
    for k in 1..n {
        hk *= (k as f64 - 1.0 + alpha / 2.0) / k as f64;
        h[k].re = hk;
    }
    for item in w.iter_mut().take(n) {
        item.re = amplitude * rng.gaussian();
    }

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(m);
    let ifft = planner.plan_fft_inverse(m);
    fft.process(&mut h);
    fft.process(&mut w);
    for (a, b) in h.iter_mut().zip(w.iter()) {
        *a *= *b;
    }
    ifft.process(&mut h);

    let scale = 1.0 / m as f64;
    h.into_iter().take(n).map(|c| c.re * scale).collect()
}

/// SplitMix64 — tiny, fast, platform-stable generator for the noise stream.
struct SplitMix64 {
    state: u64,
    spare: Option<f64>,
}

impl SplitMix64 {
    fn new(seed: u64) -> Self {
        Self {
            state: seed,
            spare: None,
        }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn uniform(&mut self) -> f64 {
        // 53-bit mantissa in (0, 1].
        (((self.next_u64() >> 11) as f64) + 1.0) / 9_007_199_254_740_992.0
    }

    /// Standard normal via Box-Muller (cached pair).
    fn gaussian(&mut self) -> f64 {
        if let Some(z) = self.spare.take() {
            return z;
        }
        let u = self.uniform();
        let v = self.uniform();
        let r = (-2.0 * u.ln()).sqrt();
        let theta = 2.0 * std::f64::consts::PI * v;
        self.spare = Some(r * theta.sin());
        r * theta.cos()
    }
}

/// FNV-1a — stable, dependency-free name hash for per-source seeding.
fn fnv1a(input: &str) -> u64 {
    let mut hash: u64 = 0xCBF2_9CE4_8422_2325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white_noise_matches_requested_variance() {
        let mut rng = SplitMix64::new(42);
        let n = 100_000;
        let na = 1e-3;
        let mut sum = 0.0;
        let mut sum_sq = 0.0;
        for _ in 0..n {
            let v = na * rng.gaussian();
            sum += v;
            sum_sq += v * v;
        }
        let mean = sum / n as f64;
        let var = sum_sq / n as f64 - mean * mean;
        assert!(mean.abs() < 5e-5, "white noise mean ~0, got {mean}");
        assert!(
            (var - na * na).abs() / (na * na) < 0.02,
            "white noise variance within 2%: got {var}, want {}",
            na * na
        );
    }

    #[test]
    fn one_over_f_spectrum_has_the_requested_slope() {
        // Average power in a low band vs a band one decade up must fall by
        // ~10^alpha for 1/f^alpha noise. Coarse but discriminating.
        use rustfft::{FftPlanner, num_complex::Complex};
        let n = 32_768;
        let alpha = 1.0;
        let mut rng = SplitMix64::new(7);
        let series = kasdin_one_over_f(n, alpha, 1.0, &mut rng);

        let mut buf: Vec<Complex<f64>> = series.iter().map(|v| Complex::new(*v, 0.0)).collect();
        FftPlanner::new().plan_fft_forward(n).process(&mut buf);

        let band_power = |lo: usize, hi: usize| -> f64 {
            buf[lo..hi].iter().map(|c| c.norm_sqr()).sum::<f64>() / (hi - lo) as f64
        };
        let low = band_power(8, 32);
        let high = band_power(80, 320);
        let measured_alpha = (low / high).log10() / (80.0f64 / 8.0).log10();
        assert!(
            (measured_alpha - alpha).abs() < 0.3,
            "spectral slope ~{alpha}, measured {measured_alpha}"
        );
    }

    #[test]
    fn expansion_is_deterministic_and_seed_sensitive() {
        let a = generate_noise_points(1e-3, 1e-9, 0.0, 0.0, 1e-6, 99, "v1").unwrap();
        let b = generate_noise_points(1e-3, 1e-9, 0.0, 0.0, 1e-6, 99, "v1").unwrap();
        let c = generate_noise_points(1e-3, 1e-9, 0.0, 0.0, 1e-6, 100, "v1").unwrap();
        assert_eq!(a.len(), b.len());
        assert!(
            a.iter().zip(&b).all(|(x, y)| x == y),
            "same seed => identical train"
        );
        assert!(
            a.iter().zip(&c).any(|(x, y)| x.1 != y.1),
            "different seed => different train"
        );
    }

    #[test]
    fn sample_cap_is_enforced_with_a_clear_error() {
        let err = generate_noise_points(1e-3, 1e-12, 0.0, 0.0, 1.0, 1, "vbig").unwrap_err();
        assert!(
            err.contains("Raise NT"),
            "diagnostic explains the fix: {err}"
        );
    }
}
