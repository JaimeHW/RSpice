//! Closed-form oracles for the statistics, through the parser and evaluator,
//! on deliberately non-uniform grids; and the same reductions over a stored
//! trace, which must read it a block at a time.

use std::f64::consts::PI;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::calculator::evaluator::{EvaluationContext, evaluate};
use crate::calculator::parser::try_parse;
use crate::calculator::retained::WaveformsContext;
use crate::calculator::value::{CalcValue, RealValue};
use crate::waveform::{
    BLOCK_SAMPLES, ColumnSource, RetainedWaveform, StoredWindow, WaveformValues, WindowLead,
};

fn plain_number(text: &str) -> Result<f64, String> {
    text.parse().map_err(|error| format!("{error}"))
}

fn measure(context: &impl EvaluationContext, expression: &str) -> Result<f64, String> {
    let parsed = try_parse(expression, plain_number).map_err(|error| error.to_string())?;
    match evaluate(&parsed, context).map_err(|error| error.to_string())? {
        CalcValue::Real(RealValue::Scalar(value)) => Ok(value),
        other => Err(format!("{expression} is not a scalar: {other:?}")),
    }
}

#[track_caller]
fn close(context: &impl EvaluationContext, expression: &str, expected: f64, tolerance: f64) {
    let actual =
        measure(context, expression).unwrap_or_else(|error| panic!("{expression} failed: {error}"));
    assert!(
        (actual - expected).abs() <= tolerance,
        "{expression}: expected {expected}, got {actual} (tolerance {tolerance})"
    );
}

/// A grid from 0 to `end` whose steps grow and shrink, as a transient's do.
fn uneven_grid(points: usize, end: f64) -> Vec<f64> {
    (0..points)
        .map(|i| {
            let u = i as f64 / (points - 1) as f64;
            end * (u + 0.3 * (2.0 * PI * u).sin() / (2.0 * PI))
        })
        .collect()
}

fn traces(signals: Vec<(&str, Vec<f64>, Vec<f64>)>) -> Vec<RetainedWaveform> {
    signals
        .into_iter()
        .map(|(name, x, y)| RetainedWaveform::new(name, x, y))
        .collect()
}

const AMPLITUDE: f64 = 1.5;
const OFFSET: f64 = 0.25;
const HZ: f64 = 1e3;

fn sine(t: f64) -> f64 {
    OFFSET + AMPLITUDE * (2.0 * PI * HZ * t).sin()
}

/// Over whole periods a sine's deviation is A/√2, its skewness 0, its
/// kurtosis 3/2, its median the offset, and the level it is below for a
/// share p of the time is offset + A·sin(π(p − ½)).
#[test]
fn a_sines_statistics_are_its_closed_forms() {
    let x = uneven_grid(60_001, 3.0 / HZ);
    let y = x.iter().map(|&t| sine(t)).collect();
    let waveforms = traces(vec![("V(s)", x, y)]);
    let context = WaveformsContext::new(&waveforms);
    let deviation = AMPLITUDE / 2f64.sqrt();
    close(&context, "stddev(V(s))", deviation, 1e-6 * deviation);
    close(
        &context,
        "stddev(V(s), \"population\")",
        deviation,
        1e-6 * deviation,
    );
    close(
        &context,
        "variance(V(s))",
        deviation * deviation,
        2e-6 * deviation,
    );
    close(&context, "skewness(V(s))", 0.0, 1e-6);
    close(&context, "kurtosis(V(s))", 1.5, 1e-6);
    close(&context, "median(V(s))", OFFSET, 1e-6);
    for percent in [10.0, 25.0, 90.0] {
        let expected = OFFSET + AMPLITUDE * (PI * (percent / 100.0 - 0.5)).sin();
        close(
            &context,
            &format!("percentile(V(s), {percent})"),
            expected,
            1e-6,
        );
    }
    close(&context, "percentile(V(s), 0)", OFFSET - AMPLITUDE, 1e-6);
    close(&context, "percentile(V(s), 100)", OFFSET + AMPLITUDE, 1e-6);
}

/// A ramp's values are uniform, and its trace is straight, so its
/// statistics are exact on any grid: deviation 1/√12, skewness 0, kurtosis
/// 9/5, and its percentiles the ramp itself.
#[test]
fn a_ramps_statistics_are_exact_on_a_random_grid() {
    let mut state = 0x9e37_79b9_7f4a_7c15_u64;
    let mut steps: Vec<f64> = (0..500)
        .map(|_| {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1);
            0.05 + (state >> 11) as f64 / (1u64 << 53) as f64
        })
        .collect();
    let total: f64 = steps.iter().sum();
    steps.iter_mut().for_each(|step| *step /= total);
    let mut x = vec![0.0];
    for step in steps {
        x.push(x.last().unwrap() + step);
    }
    *x.last_mut().unwrap() = 1.0;
    let waveforms = traces(vec![("V(r)", x.clone(), x)]);
    let context = WaveformsContext::new(&waveforms);
    close(&context, "stddev(V(r))", 1.0 / 12f64.sqrt(), 1e-14);
    close(&context, "skewness(V(r))", 0.0, 1e-12);
    close(&context, "kurtosis(V(r))", 1.8, 1e-12);
    close(&context, "median(V(r))", 0.5, 1e-14);
    close(&context, "percentile(V(r), 90)", 0.9, 1e-14);
}

/// A pulse train high for a quarter of its span is a two-level
/// distribution: skewness (1 − 2p)/√(p(1 − p)) and kurtosis
/// (1 − 3p(1 − p))/(p(1 − p)) with p = ¼, median the low level, and the
/// high level from the 75th percentile up. Its vertical edges hold no span.
#[test]
fn a_pulse_trains_statistics_follow_its_duty_cycle() {
    let x = vec![0.0, 0.3, 0.3, 0.4, 0.4, 0.75, 0.75, 0.9, 0.9, 1.0];
    let y = vec![0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0];
    let waveforms = traces(vec![("V(p)", x, y)]);
    let context = WaveformsContext::new(&waveforms);
    let p: f64 = 0.25;
    let spread = p * (1.0 - p);
    close(&context, "stddev(V(p))", spread.sqrt(), 1e-14);
    close(
        &context,
        "skewness(V(p))",
        (1.0 - 2.0 * p) / spread.sqrt(),
        1e-12,
    );
    close(
        &context,
        "kurtosis(V(p))",
        (1.0 - 3.0 * spread) / spread,
        1e-12,
    );
    close(&context, "median(V(p))", 0.0, 0.0);
    close(&context, "percentile(V(p), 80)", 1.0, 0.0);
}

/// "sample" reads the samples alone, unweighted, divided by N − 1, whatever
/// the grid; a constant trace has no skewness; a percentage is 0 to 100.
#[test]
fn a_sample_is_read_unweighted_and_nonsense_is_refused() {
    let x = vec![0.0, 0.1, 0.5, 0.6, 2.0, 2.2, 3.0, 9.0];
    let y = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let waveforms = traces(vec![("V(d)", x.clone(), y), ("V(c)", x, vec![1.0; 8])]);
    let context = WaveformsContext::new(&waveforms);
    close(&context, "variance(V(d), \"sample\")", 32.0 / 7.0, 1e-14);
    close(
        &context,
        "stddev(V(d), \"sample\")",
        (32.0_f64 / 7.0).sqrt(),
        1e-14,
    );
    for (expression, reason) in [
        ("skewness(V(c))", "has no spread"),
        ("percentile(V(d), 120)", "from 0 to 100"),
        ("stddev(V(d), \"rising\")", "\"population\", \"sample\""),
        ("stddev(1)", "measures a waveform, not a scalar"),
    ] {
        let error = measure(&context, expression).unwrap_err();
        assert!(error.contains(reason), "{expression}: {error}");
    }
}

/// A stored column that records the longest read it is asked for.
struct Column {
    values: Vec<f64>,
    longest: AtomicUsize,
}

impl ColumnSource for Column {
    fn name(&self) -> &str {
        "v(s)"
    }

    fn store(&self) -> String {
        "run.rsr".to_owned()
    }

    fn rows(&self) -> u64 {
        self.values.len() as u64
    }

    fn read(&self, first: u64, out: &mut [f64]) -> Result<(), String> {
        self.longest.fetch_max(out.len(), Ordering::SeqCst);
        let first = first as usize;
        out.copy_from_slice(&self.values[first..first + out.len()]);
        Ok(())
    }
}

fn stored(values: Vec<f64>) -> (Arc<Column>, WaveformValues) {
    let rows = values.len() as u64;
    let column = Arc::new(Column {
        values,
        longest: AtomicUsize::new(0),
    });
    let window = StoredWindow {
        first: 0,
        rows,
        lead: WindowLead::None,
    };
    let values = WaveformValues::stored(column.clone(), window).unwrap();
    (column, values)
}

/// A reduction over a stored trace, or over clip of one, reads it a block
/// at a time and never whole, and gives the answer the same samples give in
/// memory, bit for bit.
#[test]
fn a_reduction_over_a_stored_trace_reads_it_a_block_at_a_time() {
    let points = 3 * BLOCK_SAMPLES + 123;
    let x = uneven_grid(points, 3.0 / HZ);
    let y: Vec<f64> = x.iter().map(|&t| sine(t)).collect();
    let (x_column, x_stored) = stored(x.clone());
    let (y_column, y_stored) = stored(y.clone());
    let on_disk = vec![RetainedWaveform::new("V(s)", x_stored, y_stored)];
    let in_memory = traces(vec![("V(s)", x, y)]);
    let (disk, memory) = (
        WaveformsContext::new(&on_disk),
        WaveformsContext::new(&in_memory),
    );
    let deviation = AMPLITUDE / 2f64.sqrt();
    for (expression, expected, tolerance) in [
        ("stddev(V(s))", deviation, 1e-6),
        ("kurtosis(V(s))", 1.5, 1e-6),
        ("percentile(V(s), 25)", OFFSET - deviation, 1e-6),
        // One whole period inside the run, its ends interpolated.
        ("stddev(clip(V(s), 0.0011, 0.0021))", deviation, 1e-6),
        ("variance(V(s), \"sample\")", f64::NAN, 0.0),
    ] {
        let from_disk = measure(&disk, expression).unwrap();
        let from_memory = measure(&memory, expression).unwrap();
        assert_eq!(
            from_disk.to_bits(),
            from_memory.to_bits(),
            "{expression}: stored {from_disk}, in memory {from_memory}"
        );
        if expected.is_finite() {
            assert!(
                (from_disk - expected).abs() <= tolerance,
                "{expression}: expected {expected}, got {from_disk}"
            );
        }
    }
    for column in [&x_column, &y_column] {
        let longest = column.longest.load(Ordering::SeqCst);
        assert!(
            longest <= BLOCK_SAMPLES,
            "a read of {longest} samples took more than a block"
        );
    }
}

/// The stored path refuses what the in-memory one refuses.
#[test]
fn a_stored_trace_is_refused_where_its_samples_are_undefined() {
    let (_, x) = stored(vec![0.0, 1.0, 2.0, 3.0]);
    let (_, y) = stored(vec![0.0, f64::NAN, 2.0, 3.0]);
    let on_disk = vec![RetainedWaveform::new("V(h)", x, y)];
    let context = WaveformsContext::new(&on_disk);
    let error = measure(&context, "stddev(V(h))").unwrap_err();
    assert!(error.contains("undefined samples"), "{error}");
    let error = measure(&context, "stddev(clip(V(h), 5, 6))").unwrap_err();
    assert!(error.contains("lies outside the trace"), "{error}");
}
