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
use crate::abort_signal::AbortSignal;
use crate::engine::SimulationError;
use crate::netlist::{Element, ElementKind, SourceSpec};

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

/// Expand the elaborated sources in place, after instance parameters and
/// canonical names are resolved. Each instance receives an independent stream.
pub(in crate::engine) fn expand_transient_noise(
    elements: &mut [Element],
    seed: Option<u64>,
    tstop: Value,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    let base_seed = seed.unwrap_or(0x5EED_0001);
    for (index, element) in elements.iter_mut().enumerate() {
        if index.is_multiple_of(64) && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if let ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) =
            &mut element.kind
            && spec_contains_transient_random(spec)
        {
            // A single generated train may be large. Do not defer cancellation
            // until another 64 source instances have allocated their samples.
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let seed = base_seed ^ fnv1a(&element.name.to_ascii_uppercase());
            replace_transient_random(spec, tstop, seed, &element.name, 0.0, abort)?;
        }
    }
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    Ok(())
}

fn spec_contains_transient_random(spec: &SourceSpec) -> bool {
    match spec {
        SourceSpec::Distortion { inner, .. } | SourceSpec::RfPort { inner, .. } => {
            spec_contains_transient_random(inner)
        }
        SourceSpec::TrNoise { .. } | SourceSpec::TrRandom { .. } => true,
        SourceSpec::DcTransient { transient, .. }
        | SourceSpec::AcTransient { transient, .. }
        | SourceSpec::DcAcTransient { transient, .. } => spec_contains_transient_random(transient),
        _ => false,
    }
}

fn replace_transient_random(
    spec: &mut SourceSpec,
    tstop: Value,
    seed: u64,
    name: &str,
    dc_offset: Value,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    let mut points = match spec {
        SourceSpec::Distortion { inner, .. } | SourceSpec::RfPort { inner, .. } => {
            return replace_transient_random(inner, tstop, seed, name, dc_offset, abort);
        }
        SourceSpec::DcTransient {
            dc_value,
            transient,
        }
        | SourceSpec::DcAcTransient {
            dc_value,
            transient,
            ..
        } => {
            return replace_transient_random(transient, tstop, seed, name, *dc_value, abort);
        }
        SourceSpec::AcTransient { transient, .. } => {
            return replace_transient_random(transient, tstop, seed, name, dc_offset, abort);
        }
        SourceSpec::TrNoise {
            na,
            nt,
            nalpha,
            namp,
            rts_amplitude,
            rts_capture,
            rts_emit,
        } => generate_noise_points(
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
            abort,
        )?,
        SourceSpec::TrRandom {
            distribution,
            sample_interval,
            delay,
            parameter1,
            parameter2,
        } => generate_trrandom_points(
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
            abort,
        )?,
        _ => return Ok(()),
    };
    // Ngspice adds explicit DC to TRNOISE/TRRANDOM at every time, unlike
    // ordinary waveforms. Preserve that offset when lowering to generic PWL.
    if dc_offset != 0.0 {
        for (index, (_, value)) in points.iter_mut().enumerate() {
            if index.is_multiple_of(512) {
                check_noise_abort(abort)?;
            }
            *value += dc_offset;
            if !value.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "transient-random source '{name}' DC offset produced a non-finite sample"
                )));
            }
        }
    }
    *spec = SourceSpec::Pwl {
        points,
        delay: 0.0,
        repeat_from: None,
    };
    Ok(())
}

/// Generate the noise sample train as PWL points on the `k*NT` grid.
fn generate_noise_points(
    spectrum: TrNoiseSpectrum,
    rts: TrNoiseRts,
    tstop: Value,
    seed: u64,
    name: &str,
    abort: &dyn AbortSignal,
) -> Result<Vec<(Value, Value)>, SimulationError> {
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
    check_noise_abort(abort)?;
    // NALPHA=0 disables flicker noise, as in ngspice's trnoise_state_gen.
    let flicker_enabled = namp != 0.0 && nalpha > 0.0;
    let rts_enabled = rts_amplitude != 0.0 && !(rts_capture == 0.0 && rts_emit == 0.0);
    if na == 0.0 && !flicker_enabled && !rts_enabled {
        return Ok(vec![(0.0, 0.0), (tstop.max(1e-12), 0.0)]);
    }
    if (na != 0.0 || flicker_enabled) && !(nt.is_finite() && nt > 0.0) {
        return Err(SimulationError::Circuit(format!(
            "TRNOISE source '{name}' requires a positive sample interval NT"
        )));
    }
    // Cover a complete sample interval beyond tstop. RTS generation uses this
    // same horizon so inserting a later event cannot change earlier interpolation.
    let effective_nt = if nt > 0.0 { nt } else { tstop.max(1e-12) };
    let n = checked_noise_sample_count(
        "TRNOISE",
        name,
        tstop / effective_nt,
        2,
        "tstop/NT",
        "Raise NT or shorten the transient.",
    )
    .map_err(SimulationError::Circuit)?;
    let waveform_end = (n - 1) as Value * effective_nt;
    if !waveform_end.is_finite() {
        return Err(SimulationError::Circuit(format!(
            "TRNOISE source '{name}' sample times exceed finite precision"
        )));
    }
    let mut samples = vec![0.0; n];
    // Separate component streams: enabling another component or extending the
    // requested horizon must not change samples already drawn from this one.
    let mut white_rng = SplitMix64::new(seed);
    if na != 0.0 {
        for (index, sample) in samples.iter_mut().enumerate().skip(1) {
            if index.is_multiple_of(512) {
                check_noise_abort(abort)?;
            }
            *sample = na * white_rng.gaussian();
        }
    }
    if flicker_enabled {
        let mut flicker_rng = SplitMix64::new(seed ^ 0x464C_4943_4B45_5221);
        let flicker = kasdin_one_over_f(n, nalpha, namp, &mut flicker_rng, abort)?;
        let origin = flicker[0];
        for (index, (sample, value)) in samples.iter_mut().zip(flicker).enumerate().skip(1) {
            if index.is_multiple_of(512) {
                check_noise_abort(abort)?;
            }
            *sample += value - origin;
        }
    }
    // The noise component starts at zero. Expansion adds any explicit DC
    // offset after generation, including at the origin.
    let mut points = Vec::with_capacity(n);
    for (index, value) in samples.into_iter().enumerate() {
        if index.is_multiple_of(512) {
            check_noise_abort(abort)?;
        }
        if !value.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "TRNOISE source '{name}' generated a non-finite sample"
            )));
        }
        points.push((index as Value * effective_nt, value));
    }
    let mut rts_rng = SplitMix64::new(seed ^ 0x5254_535F_4E4F_4953);
    add_rts_points(
        points,
        rts_amplitude,
        rts_capture,
        rts_emit,
        waveform_end,
        &mut rts_rng,
        abort,
    )
}

fn add_rts_points(
    base: Vec<(Value, Value)>,
    amplitude: Value,
    capture_mean: Value,
    emit_mean: Value,
    tstop: Value,
    rng: &mut SplitMix64,
    abort: &dyn AbortSignal,
) -> Result<Vec<(Value, Value)>, SimulationError> {
    if amplitude == 0.0 || (capture_mean == 0.0 && emit_mean == 0.0) {
        return Ok(base);
    }
    if !(capture_mean.is_finite() && capture_mean > 0.0 && emit_mean.is_finite() && emit_mean > 0.0)
    {
        return Err(SimulationError::Circuit(
            "TRNOISE RTS requires positive capture and emission mean times".to_string(),
        ));
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
        if events.len().is_multiple_of(512) {
            check_noise_abort(abort)?;
        }
        let mean = if state == 0.0 {
            capture_mean
        } else {
            emit_mean
        };
        let next_time = time - mean * rng.uniform().ln();
        if next_time <= time {
            return Err(SimulationError::Circuit(
                "TRNOISE RTS event clock cannot advance at the requested precision".to_string(),
            ));
        }
        time = next_time;
        if time > tstop {
            break;
        }
        let next = if state == 0.0 { amplitude } else { 0.0 };
        events.push((time, state, next));
        state = next;
        if events.len() > MAX_NOISE_SAMPLES {
            return Err(SimulationError::Circuit(
                "TRNOISE RTS generated too many transitions".to_string(),
            ));
        }
    }

    let mut output = Vec::with_capacity(base.len() + events.len() * 2);
    let mut event_index = 0usize;
    let mut state = 0.0;
    for (index, &(time, value)) in base.iter().enumerate() {
        if index.is_multiple_of(512) {
            check_noise_abort(abort)?;
        }
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
    abort: &dyn AbortSignal,
) -> Result<Vec<(Value, Value)>, SimulationError> {
    check_noise_abort(abort)?;
    let TrRandomSpec {
        distribution,
        sample_interval,
        delay,
        parameter1,
        parameter2,
    } = spec;
    if !(sample_interval.is_finite() && sample_interval > 0.0) {
        return Err(SimulationError::Circuit(format!(
            "TRRANDOM source '{}' requires a positive sample interval TS",
            name
        )));
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
    )
    .map_err(SimulationError::Circuit)?;
    let final_sample_time = delay + (count - 1) as Value * sample_interval;
    if !final_sample_time.is_finite() {
        return Err(SimulationError::Circuit(format!(
            "TRRANDOM source '{name}' sample times exceed finite precision"
        )));
    }
    let mut rng = SplitMix64::new(seed);
    let mut points = vec![(0.0, parameter2)];
    if delay > 0.0 {
        points.push((delay, parameter2));
    }
    let mut previous = parameter2;
    for index in 0..count {
        if index.is_multiple_of(512) {
            check_noise_abort(abort)?;
        }
        let time = delay + index as Value * sample_interval;
        let value = match distribution {
            1 => parameter2 + parameter1 * (2.0 * rng.uniform() - 1.0),
            2 => parameter2 + parameter1 * rng.gaussian(),
            3 => parameter2 - parameter1 * rng.uniform().ln(),
            4 => parameter2 + rng.poisson(parameter1) as Value,
            _ => {
                return Err(SimulationError::Circuit(format!(
                    "TRRANDOM source '{}' has invalid TYPE",
                    name
                )));
            }
        };
        if !value.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "TRRANDOM source '{name}' generated a non-finite sample"
            )));
        }
        points.push((time, previous));
        points.push((time, value));
        previous = value;
    }
    points.push((tstop.max(final_sample_time).max(delay), previous));
    Ok(points)
}

fn check_noise_abort(abort: &dyn AbortSignal) -> Result<(), SimulationError> {
    if abort.is_aborted() {
        Err(SimulationError::Aborted)
    } else {
        Ok(())
    }
}

/// Causal Kasdin convolution with horizon-independent operation order.
/// Every source/target pair belongs to one dyadic interval: the source lies
/// in its left half and the target in its right half. At that midpoint the
/// left inputs are complete, so their contributions can be added once. Block
/// sizes depend only on the midpoint, never on the requested output length.
/// This costs O(n log^2 n), retains O(n) storage and preserves exact prefixes.
fn kasdin_one_over_f(
    n: usize,
    alpha: Value,
    amplitude: Value,
    rng: &mut SplitMix64,
    abort: &dyn AbortSignal,
) -> Result<Vec<Value>, SimulationError> {
    use rustfft::{FftPlanner, num_complex::Complex};
    check_noise_abort(abort)?;
    if n == 0 {
        return Ok(Vec::new());
    }
    let size = n.next_power_of_two();
    let mut h = vec![1.0; size];
    for k in 1..size {
        if k.is_multiple_of(512) {
            check_noise_abort(abort)?;
        }
        h[k] = h[k - 1] * (k as Value - 1.0 + alpha / 2.0) / k as Value;
    }
    let mut white = Vec::with_capacity(n);
    for index in 0..n {
        if index.is_multiple_of(512) {
            check_noise_abort(abort)?;
        }
        white.push(amplitude * rng.gaussian());
    }
    let mut result = vec![0.0; n];
    let mut planner = FftPlanner::new();
    let mut kernels: Vec<Option<Vec<Complex<Value>>>> = vec![None; size.ilog2() as usize + 1];
    let mut work = Vec::new();
    let mut scratch = Vec::new();
    for midpoint in 1..n {
        if midpoint.is_multiple_of(512) {
            check_noise_abort(abort)?;
        }
        let level = midpoint.trailing_zeros() as usize;
        let block = 1usize << level;
        let end = (midpoint + block).min(n);
        let left = midpoint - block;
        if block <= 32 {
            for target in midpoint..end {
                let mut sum = 0.0;
                for source in left..midpoint {
                    sum += white[source] * h[target - source];
                }
                result[target] += sum;
            }
            continue;
        }
        check_noise_abort(abort)?;
        let length = block * 2;
        let forward = planner.plan_fft_forward(length);
        let inverse = planner.plan_fft_inverse(length);
        scratch.resize(
            forward
                .get_inplace_scratch_len()
                .max(inverse.get_inplace_scratch_len()),
            Complex::default(),
        );
        if kernels[level].is_none() {
            let mut kernel: Vec<_> = h[..length]
                .iter()
                .map(|value| Complex::new(*value, 0.0))
                .collect();
            forward.process_with_scratch(&mut kernel, &mut scratch);
            kernels[level] = Some(kernel);
            check_noise_abort(abort)?;
        }
        let kernel = kernels[level].as_ref().expect("kernel initialized");
        work.resize(length, Complex::default());
        work.fill(Complex::default());
        for (entry, value) in work.iter_mut().zip(&white[left..midpoint]) {
            entry.re = *value;
        }
        forward.process_with_scratch(&mut work, &mut scratch);
        check_noise_abort(abort)?;
        for (entry, coefficient) in work.iter_mut().zip(kernel) {
            *entry *= coefficient;
        }
        inverse.process_with_scratch(&mut work, &mut scratch);
        check_noise_abort(abort)?;
        // Circular wrap lands only in the first half, which is discarded.
        let scale = 1.0 / length as Value;
        for target in midpoint..end {
            result[target] += work[target - left].re * scale;
        }
    }
    for (index, (value, diagonal)) in result.iter_mut().zip(white).enumerate() {
        if index.is_multiple_of(512) {
            check_noise_abort(abort)?;
        }
        *value += diagonal;
    }
    Ok(result)
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
        // Open endpoints keep logarithmic draws finite and RTS dwell times
        // strictly positive. Rejection avoids biasing the first discrete bin.
        loop {
            let bits = self.next_u64() >> 11;
            if bits != 0 {
                return bits as f64 / 9_007_199_254_740_992.0;
            }
        }
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
    use crate::abort_signal::NoAbort;
    use crate::netlist::Netlist;

    #[test]
    fn finite_stop_time_cannot_overflow_generated_source_clocks() {
        let noise = generate_noise_points(
            TrNoiseSpectrum {
                na: 1.0,
                nt: 1e308,
                nalpha: 0.0,
                namp: 0.0,
            },
            TrNoiseRts {
                rts_amplitude: 0.0,
                rts_capture: 0.0,
                rts_emit: 0.0,
            },
            1.5e308,
            42,
            "Vnoise",
            &NoAbort,
        )
        .unwrap_err();
        let random = generate_trrandom_points(
            TrRandomSpec {
                distribution: 2,
                sample_interval: 1e308,
                delay: 0.0,
                parameter1: 1.0,
                parameter2: 0.0,
            },
            1.5e308,
            42,
            "Vrandom",
            &NoAbort,
        )
        .unwrap_err();
        for error in [noise, random] {
            assert!(
                matches!(error, SimulationError::Circuit(ref message) if message.contains("sample times exceed finite precision"))
            );
        }
    }

    #[test]
    fn causal_flicker_matches_direct_convolution_and_preserves_exact_prefixes() {
        for alpha in [0.25, 1.0, 1.8] {
            let n = 513;
            let mut rng = SplitMix64::new(42);
            let full = kasdin_one_over_f(n, alpha, 1.0, &mut rng, &NoAbort).unwrap();
            let mut reference_rng = SplitMix64::new(42);
            let white: Vec<_> = (0..n).map(|_| reference_rng.gaussian()).collect();
            let mut h = vec![1.0; n];
            for k in 1..n {
                h[k] = h[k - 1] * (k as Value - 1.0 + alpha / 2.0) / k as Value;
            }
            for target in 0..n {
                let direct: Value = (0..=target)
                    .map(|source| white[source] * h[target - source])
                    .sum();
                assert!(
                    (full[target] - direct).abs() < 1e-12,
                    "alpha={alpha}, sample={target}: {} vs {direct}",
                    full[target]
                );
            }
            for length in [1, 2, 3, 31, 32, 33, 63, 64, 65, 127, 128, 129, 257, 512] {
                let mut rng = SplitMix64::new(42);
                let prefix = kasdin_one_over_f(length, alpha, 1.0, &mut rng, &NoAbort).unwrap();
                for (actual, expected) in prefix.iter().zip(&full) {
                    assert_eq!(
                        actual.to_bits(),
                        expected.to_bits(),
                        "alpha={alpha}, length={length}"
                    );
                }
            }
        }
    }

    #[test]
    fn noise_components_start_at_zero_and_keep_the_same_horizon_prefix() {
        for (white, flicker, rts) in [
            (1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            (1.0, 1.0, 0.0),
            (0.0, 0.0, 1.0),
            (1.0, 1.0, 1.0),
        ] {
            let generate = |stop| {
                generate_noise_points(
                    TrNoiseSpectrum {
                        na: white,
                        nt: 1e-9,
                        nalpha: 1.0,
                        namp: flicker,
                    },
                    TrNoiseRts {
                        rts_amplitude: rts,
                        rts_capture: 0.7e-9,
                        rts_emit: 0.9e-9,
                    },
                    stop,
                    42,
                    "Vnoise",
                    &NoAbort,
                )
                .unwrap()
            };
            let full = generate(257.25e-9);
            assert_eq!(full[0], (0.0, 0.0));
            for stop in [
                0.25e-9, 1.25e-9, 31.25e-9, 32.25e-9, 63.25e-9, 64.25e-9, 127.25e-9, 128.25e-9,
            ] {
                let actual: Vec<_> = generate(stop)
                    .into_iter()
                    .filter(|(time, _)| *time <= stop)
                    .collect();
                let expected: Vec<_> = full
                    .iter()
                    .copied()
                    .filter(|(time, _)| *time <= stop)
                    .collect();
                assert_eq!(
                    actual, expected,
                    "components=({white},{flicker},{rts}), tstop={stop}"
                );
            }
        }
    }

    #[test]
    fn white_and_flicker_components_use_independent_random_streams() {
        let generate = |na, namp| {
            generate_noise_points(
                TrNoiseSpectrum {
                    na,
                    nt: 1e-9,
                    nalpha: 1.0,
                    namp,
                },
                TrNoiseRts {
                    rts_amplitude: 0.0,
                    rts_capture: 0.0,
                    rts_emit: 0.0,
                },
                100e-9,
                42,
                "Vnoise",
                &NoAbort,
            )
            .unwrap()
        };
        let white = generate(1.0, 0.0);
        let flicker = generate(0.0, 1.0);
        let both = generate(1.0, 1.0);
        for ((w, f), sum) in white.iter().zip(&flicker).zip(&both) {
            assert_eq!(sum.1.to_bits(), (w.1 + f.1).to_bits());
        }
    }

    #[test]
    fn non_grid_trrandom_horizons_keep_ordered_stable_prefixes() {
        for distribution in 1..=4 {
            let generate = |stop| {
                generate_trrandom_points(
                    TrRandomSpec {
                        distribution,
                        sample_interval: 1e-9,
                        delay: 0.3e-9,
                        parameter1: 2.0,
                        parameter2: 0.25,
                    },
                    stop,
                    42,
                    "Irandom",
                    &NoAbort,
                )
                .unwrap()
            };
            let full = generate(10.75e-9);
            for stop in [0.1e-9, 0.3e-9, 0.75e-9, 3.75e-9, 9.75e-9] {
                let short = generate(stop);
                assert!(short.windows(2).all(|pair| pair[0].0 <= pair[1].0));
                let prefix: Vec<_> = short.into_iter().filter(|(time, _)| *time < stop).collect();
                let expected: Vec<_> = full
                    .iter()
                    .copied()
                    .filter(|(time, _)| *time < stop)
                    .collect();
                assert_eq!(prefix, expected, "TYPE={distribution}, tstop={stop}");
            }
        }
    }

    #[test]
    fn long_flicker_generation_observes_cancellation() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        struct PollBudget(AtomicUsize);
        impl AbortSignal for PollBudget {
            fn is_aborted(&self) -> bool {
                self.0.fetch_add(1, Ordering::Relaxed) >= 4
            }
        }
        let mut rng = SplitMix64::new(42);
        let error = kasdin_one_over_f(
            1 << 20,
            1.0,
            1.0,
            &mut rng,
            &PollBudget(AtomicUsize::new(0)),
        )
        .unwrap_err();
        assert!(matches!(error, SimulationError::Aborted));
    }

    #[test]
    fn expansion_honors_abort_before_later_random_instances() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        struct AbortAfterPollBudget(AtomicUsize);
        impl AbortSignal for AbortAfterPollBudget {
            fn is_aborted(&self) -> bool {
                self.0.fetch_add(1, Ordering::Relaxed) >= 2
            }
        }
        let mut netlist = Netlist::parse(
            "noise cancellation\nV1 a 0 TRNOISE(1 1n 0 0)\nV2 b 0 TRNOISE(1 1n 0 0)\n.end\n",
        )
        .unwrap();
        let error = expand_transient_noise(
            &mut netlist.elements,
            None,
            10e-9,
            &AbortAfterPollBudget(AtomicUsize::new(0)),
        )
        .unwrap_err();
        assert!(matches!(error, SimulationError::Aborted));
        let ElementKind::VoltageSource(second) = &netlist.elements[1].kind else {
            panic!("source retained")
        };
        assert!(
            spec_contains_transient_random(second),
            "later instances must not allocate a waveform after cancellation"
        );
    }

    #[test]
    fn expansion_preserves_distortion_dc_and_ac_annotations() {
        let mut netlist = Netlist::parse(
            "annotated noise\nV1 out 0 DC .25 AC 2 90 TRNOISE(1 1n 0 0) DISTOF1 3 45 DISTOF2 4 90\nR1 out 0 1\n.end\n"
        ).unwrap();
        expand_transient_noise(&mut netlist.elements, netlist.options.seed, 10e-9, &NoAbort)
            .unwrap();
        let ElementKind::VoltageSource(spec) = &netlist.elements[0].kind else {
            panic!("voltage source retained")
        };
        assert_eq!(crate::engine::extract_dc_value(spec), 0.25);
        assert_eq!(
            crate::engine::extract_ac_value(spec),
            (2.0, std::f64::consts::FRAC_PI_2)
        );
        let f1 = spec.distortion_f1().unwrap();
        let f2 = spec.distortion_f2().unwrap();
        assert_eq!((f1.magnitude, f1.phase), (3.0, std::f64::consts::FRAC_PI_4));
        assert_eq!((f2.magnitude, f2.phase), (4.0, std::f64::consts::FRAC_PI_2));
        assert!(
            !spec_contains_transient_random(spec),
            "expansion replaces the random waveform exactly once"
        );
    }

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
        let series = kasdin_one_over_f(n, alpha, 1.0, &mut rng, &NoAbort).unwrap();

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
            &NoAbort,
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
            &NoAbort,
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
            &NoAbort,
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
            &NoAbort,
        )
        .unwrap_err();
        assert!(
            err.to_string().contains("Raise NT"),
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
            &NoAbort,
        )
        .expect_err("an infinite tstop/NT quotient must be rejected");
        assert!(
            error.to_string().contains("TRNOISE source 'vextreme'"),
            "{error}"
        );
        assert!(
            error.to_string().contains("non-finite sample count"),
            "{error}"
        );
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
            &NoAbort,
        )
        .expect_err("an overflowed (tstop-delay)/TS quotient must be rejected");
        assert!(
            error.to_string().contains("TRRANDOM source 'vrextreme'"),
            "{error}"
        );
        assert!(
            error.to_string().contains("non-finite sample count"),
            "{error}"
        );
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
            &NoAbort,
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
            &NoAbort,
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
            &NoAbort,
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
