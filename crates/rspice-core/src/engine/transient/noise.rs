//! Transient-random expansion: TRNOISE/TRRANDOM sources become deterministic,
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

use crate::Value;
use crate::netlist::{Element, ElementKind, Netlist, SourceSpec};

/// Hard cap on generated samples per source. 4M samples is ~64 MB of PWL
/// points — beyond that the deck should raise NT rather than the simulator
/// silently degrading the spectrum.
const MAX_NOISE_SAMPLES: usize = 1 << 22;

/// Convert a floating sample-grid quotient only after proving that the final
/// count, including the generator's required tail points, fits the hard cap.
/// Rust's float-to-integer cast saturates, and adding the tail afterward can
/// then overflow or wrap; neither behavior is an acceptable resource check.
fn checked_noise_sample_count(
    source_kind: &str,
    name: &str,
    quotient: Value,
    required_tail: usize,
    ratio_name: &str,
    remedy: &str,
) -> Result<usize, String> {
    let maximum_grid_count = MAX_NOISE_SAMPLES.saturating_sub(required_tail);
    if !quotient.is_finite() {
        return Err(format!(
            "{source_kind} source '{name}' has a non-finite sample count from {ratio_name}; \
             the limit is {MAX_NOISE_SAMPLES}. {remedy}"
        ));
    }

    // A transient stop before the source's first sample retains the historic
    // one/two-tail-point behavior rather than manufacturing a negative count.
    let grid_count = quotient.ceil().max(0.0);
    if grid_count > maximum_grid_count as Value {
        return Err(format!(
            "{source_kind} source '{name}' sample count from {ratio_name} exceeds the limit of \
             {MAX_NOISE_SAMPLES} (including {required_tail} required tail samples). {remedy}"
        ));
    }

    // Both the cast and addition are safe because the floating value was
    // bounded against MAX_NOISE_SAMPLES - required_tail above.
    Ok(grid_count as usize + required_tail)
}

/// Replace every TRNOISE/TRRANDOM source with a generated PWL sample train
/// covering `[0, tstop]`. Returns `None` when the netlist contains neither
/// source type (the common case — zero cost, no clone).
pub(crate) fn expand_transient_noise(
    netlist: &Netlist,
    tstop: Value,
) -> Result<Option<Netlist>, String> {
    let has_noise = netlist.elements.iter().any(element_has_transient_random);
    if !has_noise {
        return Ok(None);
    }

    let base_seed = netlist.options.seed.unwrap_or(0x5EED_0001);
    let mut expanded = netlist.clone();
    for element in &mut expanded.elements {
        if !element_has_transient_random(element) {
            continue;
        }
        // Per-source stream: stable name hash mixed into the netlist seed so
        // every noise source draws an independent, reproducible sequence.
        let seed = base_seed ^ fnv1a(&element.name.to_ascii_uppercase());
        let name = element.name.clone();
        match &mut element.kind {
            ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                replace_transient_random(spec, tstop, seed, &name)?;
            }
            _ => {}
        }
    }
    Ok(Some(expanded))
}

fn element_has_transient_random(element: &Element) -> bool {
    match &element.kind {
        ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
            spec_contains_transient_random(spec)
        }
        _ => false,
    }
}

fn spec_contains_transient_random(spec: &SourceSpec) -> bool {
    match spec {
        SourceSpec::RfPort { inner, .. } => spec_contains_transient_random(inner),
        SourceSpec::TrNoise { .. } | SourceSpec::TrRandom { .. } => true,
        SourceSpec::DcTransient { transient, .. } | SourceSpec::DcAcTransient { transient, .. } => {
            spec_contains_transient_random(transient)
        }
        _ => false,
    }
}

fn replace_transient_random(
    spec: &mut SourceSpec,
    tstop: Value,
    seed: u64,
    name: &str,
) -> Result<(), String> {
    match spec {
        SourceSpec::RfPort { inner, .. } => replace_transient_random(inner, tstop, seed, name),
        SourceSpec::TrNoise {
            na,
            nt,
            nalpha,
            namp,
            rts_amplitude,
            rts_capture,
            rts_emit,
        } => {
            let points = generate_noise_points(
                TrNoiseSpectrum {
                    na: *na,
                    nt: *nt,
                    nalpha: *nalpha,
                    namp: *namp,
                },
                TrNoiseRts {
                    rts_amplitude: *rts_amplitude,
                    rts_capture: *rts_capture,
                    rts_emit: *rts_emit,
                },
                tstop,
                seed,
                name,
            )?;
            *spec = SourceSpec::Pwl {
                points,
                delay: 0.0,
                repeat_from: None,
            };
            Ok(())
        }
        SourceSpec::TrRandom {
            distribution,
            sample_interval,
            delay,
            parameter1,
            parameter2,
        } => {
            let points = generate_trrandom_points(
                TrRandomSpec {
                    distribution: *distribution,
                    sample_interval: *sample_interval,
                    delay: *delay,
                    parameter1: *parameter1,
                    parameter2: *parameter2,
                },
                tstop,
                seed,
                name,
            )?;
            *spec = SourceSpec::Pwl {
                points,
                delay: 0.0,
                repeat_from: None,
            };
            Ok(())
        }
        SourceSpec::DcTransient { transient, .. } | SourceSpec::DcAcTransient { transient, .. } => {
            replace_transient_random(transient, tstop, seed, name)
        }
        _ => Ok(()),
    }
}

/// Generate the noise sample train as PWL points on the `k*NT` grid.
fn generate_noise_points(
    spectrum: TrNoiseSpectrum,
    rts: TrNoiseRts,
    tstop: Value,
    seed: u64,
    name: &str,
) -> Result<Vec<(Value, Value)>, String> {
    let TrNoiseSpectrum {
        na,
        nt,
        nalpha,
        namp,
    } = spectrum;
    let TrNoiseRts {
        rts_amplitude,
        rts_capture,
        rts_emit,
    } = rts;
    if na == 0.0 && namp == 0.0 && rts_amplitude == 0.0 {
        return Ok(vec![(0.0, 0.0), (tstop.max(1e-12), 0.0)]);
    }
    if (na != 0.0 || namp != 0.0) && !(nt.is_finite() && nt > 0.0) {
        return Err(format!(
            "TRNOISE source '{}' requires a positive sample interval NT",
            name
        ));
    }

    // One sample past tstop so interpolation never extrapolates.
    let effective_nt = if nt > 0.0 { nt } else { tstop.max(1e-12) };
    let n = checked_noise_sample_count(
        "TRNOISE",
        name,
        tstop / effective_nt,
        2,
        "tstop/NT",
        "Raise NT or shorten the transient.",
    )?;

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

    let points = samples
        .into_iter()
        .enumerate()
        .map(|(k, v)| (k as Value * effective_nt, v))
        .collect::<Vec<_>>();
    add_rts_points(
        points,
        rts_amplitude,
        rts_capture,
        rts_emit,
        tstop,
        &mut rng,
    )
}

fn add_rts_points(
    base: Vec<(Value, Value)>,
    amplitude: Value,
    capture_mean: Value,
    emit_mean: Value,
    tstop: Value,
    rng: &mut SplitMix64,
) -> Result<Vec<(Value, Value)>, String> {
    if amplitude == 0.0 || (capture_mean == 0.0 && emit_mean == 0.0) {
        return Ok(base);
    }
    if !(capture_mean > 0.0 && emit_mean > 0.0) {
        return Err("TRNOISE RTS requires positive capture and emission mean times".to_string());
    }
    let base_value_at = |time: Value| {
        let upper = base.partition_point(|(sample_time, _)| *sample_time <= time);
        if upper == 0 {
            return base[0].1;
        }
        if upper >= base.len() {
            return base.last().map_or(0.0, |(_, value)| *value);
        }
        let (t0, v0) = base[upper - 1];
        let (t1, v1) = base[upper];
        if t1 <= t0 {
            v1
        } else {
            v0 + (v1 - v0) * (time - t0) / (t1 - t0)
        }
    };

    let mut events = Vec::<(Value, Value, Value)>::new();
    let mut time = 0.0;
    let mut state = 0.0;
    while time <= tstop {
        let mean = if state == 0.0 {
            capture_mean
        } else {
            emit_mean
        };
        time += -mean * rng.uniform().ln();
        if time > tstop {
            break;
        }
        let next = if state == 0.0 { amplitude } else { 0.0 };
        events.push((time, state, next));
        state = next;
        if events.len() > MAX_NOISE_SAMPLES {
            return Err("TRNOISE RTS generated too many transitions".to_string());
        }
    }

    let mut output = Vec::with_capacity(base.len() + events.len() * 2);
    let mut event_index = 0usize;
    let mut state = 0.0;
    for &(time, value) in &base {
        while let Some(&(event_time, before, after)) = events.get(event_index)
            && event_time <= time
        {
            let noise = base_value_at(event_time);
            output.push((event_time, noise + before));
            output.push((event_time, noise + after));
            state = after;
            event_index += 1;
        }
        output.push((time, value + state));
    }
    Ok(output)
}

fn generate_trrandom_points(
    spec: TrRandomSpec,
    tstop: Value,
    seed: u64,
    name: &str,
) -> Result<Vec<(Value, Value)>, String> {
    let TrRandomSpec {
        distribution,
        sample_interval,
        delay,
        parameter1,
        parameter2,
    } = spec;
    if !(sample_interval.is_finite() && sample_interval > 0.0) {
        return Err(format!(
            "TRRANDOM source '{}' requires a positive sample interval TS",
            name
        ));
    }
    let available_duration = tstop - delay;
    let quotient = if available_duration.is_finite() {
        available_duration.max(0.0) / sample_interval
    } else {
        available_duration
    };
    let count = checked_noise_sample_count(
        "TRRANDOM",
        name,
        quotient,
        1,
        "(tstop-delay)/TS",
        "Raise TS, delay the source, or shorten the transient.",
    )?;
    let mut rng = SplitMix64::new(seed);
    let mut points = vec![(0.0, parameter2)];
    if delay > 0.0 {
        points.push((delay, parameter2));
    }
    let mut previous = parameter2;
    for index in 0..count {
        let time = delay + index as Value * sample_interval;
        let value = match distribution {
            1 => parameter2 + parameter1 * (2.0 * rng.uniform() - 1.0),
            2 => parameter2 + parameter1 * rng.gaussian(),
            3 => parameter2 - parameter1 * rng.uniform().ln(),
            4 => parameter2 + rng.poisson(parameter1) as Value,
            _ => return Err(format!("TRRANDOM source '{}' has invalid TYPE", name)),
        };
        points.push((time, previous));
        points.push((time, value));
        previous = value;
    }
    points.push((tstop.max(delay), previous));
    Ok(points)
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
    for (k, entry) in h.iter_mut().enumerate().take(n).skip(1) {
        hk *= (k as f64 - 1.0 + alpha / 2.0) / k as f64;
        entry.re = hk;
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
/// The flicker-noise terms of a `TRNOISE` source: the white amplitude, the
/// sample interval, the 1/f exponent and the 1/f amplitude.
#[derive(Clone, Copy)]
struct TrNoiseSpectrum {
    na: Value,
    nt: Value,
    nalpha: Value,
    namp: Value,
}

/// The random-telegraph terms of the same source: amplitude, mean capture
/// time and mean emission time.
#[derive(Clone, Copy)]
struct TrNoiseRts {
    rts_amplitude: Value,
    rts_capture: Value,
    rts_emit: Value,
}

/// A `TRRANDOM` source's shape: which distribution, how often it is resampled,
/// when it starts, and the two distribution parameters.
#[derive(Clone, Copy)]
struct TrRandomSpec {
    distribution: u8,
    sample_interval: Value,
    delay: Value,
    parameter1: Value,
    parameter2: Value,
}

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

    fn poisson(&mut self, lambda: f64) -> u64 {
        if lambda <= 0.0 {
            return 0;
        }
        if lambda >= 64.0 {
            return (lambda + lambda.sqrt() * self.gaussian()).round().max(0.0) as u64;
        }
        let limit = (-lambda).exp();
        let mut product = 1.0;
        let mut count = 0u64;
        loop {
            product *= self.uniform();
            if product <= limit {
                return count;
            }
            count += 1;
        }
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
        let a = generate_noise_points(
            TrNoiseSpectrum {
                na: 1e-3,
                nt: 1e-9,
                nalpha: 0.0,
                namp: 0.0,
            },
            TrNoiseRts {
                rts_amplitude: 0.0,
                rts_capture: 0.0,
                rts_emit: 0.0,
            },
            1e-6,
            99,
            "v1",
        )
        .unwrap();
        let b = generate_noise_points(
            TrNoiseSpectrum {
                na: 1e-3,
                nt: 1e-9,
                nalpha: 0.0,
                namp: 0.0,
            },
            TrNoiseRts {
                rts_amplitude: 0.0,
                rts_capture: 0.0,
                rts_emit: 0.0,
            },
            1e-6,
            99,
            "v1",
        )
        .unwrap();
        let c = generate_noise_points(
            TrNoiseSpectrum {
                na: 1e-3,
                nt: 1e-9,
                nalpha: 0.0,
                namp: 0.0,
            },
            TrNoiseRts {
                rts_amplitude: 0.0,
                rts_capture: 0.0,
                rts_emit: 0.0,
            },
            1e-6,
            100,
            "v1",
        )
        .unwrap();
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
        let err = generate_noise_points(
            TrNoiseSpectrum {
                na: 1e-3,
                nt: 1e-12,
                nalpha: 0.0,
                namp: 0.0,
            },
            TrNoiseRts {
                rts_amplitude: 0.0,
                rts_capture: 0.0,
                rts_emit: 0.0,
            },
            1.0,
            1,
            "vbig",
        )
        .unwrap_err();
        assert!(
            err.contains("Raise NT"),
            "diagnostic explains the fix: {err}"
        );
    }

    #[test]
    fn sample_count_boundaries_include_required_tail_points() {
        assert_eq!(
            checked_noise_sample_count(
                "TRNOISE",
                "vn",
                (MAX_NOISE_SAMPLES - 2) as Value,
                2,
                "tstop/NT",
                "Raise NT.",
            )
            .unwrap(),
            MAX_NOISE_SAMPLES
        );
        assert!(
            checked_noise_sample_count(
                "TRNOISE",
                "vn",
                (MAX_NOISE_SAMPLES - 1) as Value,
                2,
                "tstop/NT",
                "Raise NT.",
            )
            .is_err()
        );
        assert_eq!(
            checked_noise_sample_count(
                "TRRANDOM",
                "vr",
                (MAX_NOISE_SAMPLES - 1) as Value,
                1,
                "(tstop-delay)/TS",
                "Raise TS.",
            )
            .unwrap(),
            MAX_NOISE_SAMPLES
        );
        assert!(
            checked_noise_sample_count(
                "TRRANDOM",
                "vr",
                MAX_NOISE_SAMPLES as Value,
                1,
                "(tstop-delay)/TS",
                "Raise TS.",
            )
            .is_err()
        );
    }

    #[test]
    fn trnoise_extreme_ratio_is_rejected_before_integer_conversion() {
        let error = generate_noise_points(
            TrNoiseSpectrum {
                na: 1.0,
                nt: f64::MIN_POSITIVE,
                nalpha: 0.0,
                namp: 0.0,
            },
            TrNoiseRts {
                rts_amplitude: 0.0,
                rts_capture: 0.0,
                rts_emit: 0.0,
            },
            f64::MAX,
            1,
            "vextreme",
        )
        .expect_err("an infinite tstop/NT quotient must be rejected");
        assert!(error.contains("TRNOISE source 'vextreme'"), "{error}");
        assert!(error.contains("non-finite sample count"), "{error}");
    }

    #[test]
    fn trrandom_extreme_ratio_is_rejected_before_integer_conversion() {
        let error = generate_trrandom_points(
            TrRandomSpec {
                distribution: 1,
                sample_interval: f64::MIN_POSITIVE,
                delay: -f64::MAX,
                parameter1: 1.0,
                parameter2: 0.0,
            },
            f64::MAX,
            1,
            "vrextreme",
        )
        .expect_err("an overflowed (tstop-delay)/TS quotient must be rejected");
        assert!(error.contains("TRRANDOM source 'vrextreme'"), "{error}");
        assert!(error.contains("non-finite sample count"), "{error}");
    }

    #[test]
    fn trrandom_gaussian_is_piecewise_constant_and_seeded() {
        let points = generate_trrandom_points(
            TrRandomSpec {
                distribution: 2,
                sample_interval: 1e-3,
                delay: 0.0,
                parameter1: 1.0,
                parameter2: 0.0,
            },
            3e-3,
            42,
            "vr",
        )
        .expect("TRRANDOM train");
        assert_eq!(points[0], (0.0, 0.0));
        assert_eq!(points[1].0, points[2].0);
        assert_ne!(points[1].1, points[2].1);
        assert!(points.windows(2).all(|window| window[0].0 <= window[1].0));
    }

    #[test]
    fn rts_noise_adds_duplicate_time_step_edges() {
        let points = generate_noise_points(
            TrNoiseSpectrum {
                na: 0.0,
                nt: 0.0,
                nalpha: 0.0,
                namp: 0.0,
            },
            TrNoiseRts {
                rts_amplitude: 1.0,
                rts_capture: 1e-6,
                rts_emit: 1e-6,
            },
            20e-6,
            9,
            "vrts",
        )
        .expect("RTS train");
        assert!(
            points
                .windows(2)
                .any(|window| { window[0].0 == window[1].0 && window[0].1 != window[1].1 })
        );
    }

    #[test]
    fn incomplete_rts_group_is_ignored_like_ngspice() {
        let points = generate_noise_points(
            TrNoiseSpectrum {
                na: 0.05,
                nt: 8e-12,
                nalpha: 0.0,
                namp: 1.0,
            },
            TrNoiseRts {
                rts_amplitude: 0.001,
                rts_capture: 0.0,
                rts_emit: 0.0,
            },
            1e-9,
            9,
            "vnoise",
        )
        .expect("incomplete RTS group is disabled");

        assert!(!points.is_empty());
        assert!(
            points
                .iter()
                .all(|(time, value)| time.is_finite() && value.is_finite())
        );
    }
}
