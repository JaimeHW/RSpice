//! The spectral scalars of a periodic signal — `thd`, `snr`, `sinad`, `sfdr`
//! and `enob` — read off the FFT sheet's own engine, so a calculator reading
//! and the sheet's readout of the same record are one number.
//!
//! A record is sampled as ViVA's `dft` samples it: `n` points over the window
//! from its start inclusive to its stop exclusive, the window taken as one
//! period, each point read off the trace as drawn. The transform is
//! rectangular, ViVA's default for its spectrum measurements, so a record of
//! whole periods is coherent and leaks nothing. The fundamental is the
//! largest tone, as ViVA's `thd` takes it; a call that states the
//! fundamental is refused unless the largest tone is in that frequency's
//! bin, rather than read at a tone it did not name. The harmonics counted
//! are the first ten unless the call says otherwise, as the FFT sheet counts
//! them.

use super::evaluator::EvaluationError;
use super::measures::{Argument, Reduction, number};
use super::series::Series;
use super::value::RealValue;
use crate::fft::data::{FftData, HarmonicCoverage, SpectrumAnalysis, SpectrumNormalization};
use crate::fft::pipeline::{MAX_REFERENCE_RESAMPLE_POINTS, MIN_FFT_SAMPLES};
use crate::fft::window::WindowFunction;

/// The harmonics counted when a call does not say: the FFT sheet's default.
const DEFAULT_HARMONICS: usize = 10;

/// The reduction for `name`, already lower-cased.
pub(super) fn lookup(name: &str) -> Option<Reduction> {
    Some(match name {
        "thd" => thd,
        "snr" => snr,
        "sinad" => sinad,
        "sfdr" => sfdr,
        "enob" => enob,
        _ => return None,
    })
}

/// The window and sample count a call reads, its defaults found in one pass:
/// the trace's first and last x, and the samples from the start up to the
/// stop.
struct Record {
    from: f64,
    to: f64,
    samples: usize,
}

fn record(name: &str, series: &Series, rest: &[Argument]) -> Result<Record, EvaluationError> {
    let from = number(name, "window start", rest, 0)?;
    let to = number(name, "window stop", rest, 1)?;
    let count = number(name, "sample count", rest, 2)?;
    let (mut first, mut last) = (None, None);
    let mut previous = f64::NEG_INFINITY;
    for (x, _) in series.pairs() {
        if !x.is_finite() || x < previous {
            return Err(EvaluationError::MathError(format!(
                "{name} needs a time axis that runs forward"
            )));
        }
        previous = x;
        first.get_or_insert(x);
        last = Some(x);
    }
    let (Some(first), Some(last)) = (first, last) else {
        return Err(EvaluationError::MathError(format!(
            "{name} needs a waveform with samples"
        )));
    };
    let (from, to) = (from.unwrap_or(first), to.unwrap_or(last));
    if from >= to {
        return Err(EvaluationError::MathError(format!(
            "{name} reads a window from its start to a later stop, not from {from} to {to}"
        )));
    }
    if from < first || to > last {
        return Err(EvaluationError::MathError(format!(
            "{name}'s window from {from} to {to} reaches past the trace, which runs from {first} to {last}"
        )));
    }
    let samples = match count {
        Some(count) => count,
        None => series.pairs().filter(|&(x, _)| x >= from && x < to).count() as f64,
    };
    let (least, most) = (MIN_FFT_SAMPLES, MAX_REFERENCE_RESAMPLE_POINTS);
    if samples.fract() != 0.0 || samples < least as f64 || samples > most as f64 {
        return Err(EvaluationError::MathError(format!(
            "{name} takes a whole number of samples from {least} to {most}, not {samples}"
        )));
    }
    Ok(Record {
        from,
        to,
        samples: samples as usize,
    })
}

/// The record's samples: the trace read at `from + k·(to − from)/n` for k
/// from 0 to n − 1, interpolated linearly between its samples.
fn sampled(name: &str, series: &Series, record: &Record) -> Result<Vec<f64>, EvaluationError> {
    let step = (record.to - record.from) / record.samples as f64;
    let mut out = Vec::with_capacity(record.samples);
    let mut pairs = series.pairs();
    let Some(mut before) = pairs.next() else {
        return Err(EvaluationError::MathError(format!(
            "{name} needs a waveform with samples"
        )));
    };
    let mut after = before;
    for k in 0..record.samples {
        let at = record.from + k as f64 * step;
        while after.0 < at {
            before = after;
            match pairs.next() {
                Some(next) => after = next,
                None => break,
            }
        }
        let value = if after.0 == at || after.0 == before.0 {
            after.1
        } else {
            before.1 + (after.1 - before.1) * (at - before.0) / (after.0 - before.0)
        };
        if !value.is_finite() {
            return Err(EvaluationError::MathError(format!(
                "{name} reads a trace with undefined samples in its window"
            )));
        }
        out.push(value);
    }
    Ok(out)
}

/// The FFT sheet's analysis of the record the call names.
fn analysis(
    name: &str,
    series: &Series,
    rest: &[Argument],
) -> Result<SpectrumAnalysis, EvaluationError> {
    let record = record(name, series, rest)?;
    let fundamental = number(name, "fundamental", rest, 3)?;
    if let Some(stated) = fundamental
        && stated <= 0.0
    {
        return Err(EvaluationError::MathError(format!(
            "{name} fundamental must be a positive frequency, not {stated}"
        )));
    }
    let harmonics = match number(name, "harmonic count", rest, 4)? {
        None => DEFAULT_HARMONICS,
        Some(count) if count >= 1.0 && count.fract() == 0.0 && count <= u32::MAX as f64 => {
            count as usize
        }
        Some(count) => {
            return Err(EvaluationError::MathError(format!(
                "{name} harmonic count must be a whole number of 1 or more, not {count}"
            )));
        }
    };
    let samples = sampled(name, series, &record)?;
    let rate = record.samples as f64 / (record.to - record.from);
    let fft = FftData::from_time_domain_with_normalization(
        name,
        &samples,
        rate,
        WindowFunction::Rectangular,
        SpectrumNormalization::Peak,
    )
    .map_err(|error| EvaluationError::MathError(format!("{name}: {error}")))?;
    let analysis = SpectrumAnalysis::analyze(&fft, harmonics)
        .map_err(|error| EvaluationError::MathError(format!("{name}: {error}")))?;
    let Some(bin) = analysis.fundamental_bin else {
        return Err(EvaluationError::MathError(format!(
            "{name} found no fundamental: the record holds nothing but its mean"
        )));
    };
    let width = fft.frequency_resolution();
    let largest = bin as f64 * width;
    if let Some(stated) = fundamental
        && (stated - largest).abs() > width / 2.0
    {
        return Err(EvaluationError::MathError(format!(
            "{name}: the record's largest tone is at {largest} Hz, not the stated \
             fundamental {stated} Hz"
        )));
    }
    Ok(analysis)
}

/// Why a reading that needs every counted harmonic has none.
fn coverage_refusal(name: &str, coverage: HarmonicCoverage) -> EvaluationError {
    EvaluationError::MathError(match coverage {
        HarmonicCoverage::InsufficientBandwidth {
            requested_order,
            analyzed_through,
            ..
        } => format!(
            "{name} counts {requested_order} harmonics, but this record's samples reach only \
             the {analyzed_through}th below half the sample rate; take more samples or count \
             fewer harmonics"
        ),
        HarmonicCoverage::InsufficientResolution { failed_order, .. } => format!(
            "{name} cannot tell harmonic {failed_order} from the ones below it at this \
             record's resolution; read more periods"
        ),
        _ => format!("{name} found no fundamental"),
    })
}

/// `thd(x, from, to, n, harmonics)` — the total harmonic distortion in
/// percent.
fn thd(name: &str, series: &Series, rest: &[Argument]) -> Result<RealValue, EvaluationError> {
    let analysis = analysis(name, series, rest)?;
    analysis
        .thd_percent
        .map(RealValue::Scalar)
        .ok_or_else(|| coverage_refusal(name, analysis.harmonic_coverage))
}

/// `snr(...)` — the fundamental over the noise, harmonics excluded, in dB.
fn snr(name: &str, series: &Series, rest: &[Argument]) -> Result<RealValue, EvaluationError> {
    let analysis = analysis(name, series, rest)?;
    analysis
        .snr_db
        .map(RealValue::Scalar)
        .ok_or_else(|| coverage_refusal(name, analysis.harmonic_coverage))
}

fn sinad_db(name: &str, series: &Series, rest: &[Argument]) -> Result<f64, EvaluationError> {
    let analysis = analysis(name, series, rest)?;
    analysis
        .sinad_db
        .ok_or_else(|| coverage_refusal(name, analysis.harmonic_coverage))
}

/// `sinad(...)` — the fundamental over the noise and distortion, in dB.
fn sinad(name: &str, series: &Series, rest: &[Argument]) -> Result<RealValue, EvaluationError> {
    sinad_db(name, series, rest).map(RealValue::Scalar)
}

/// `sfdr(...)` — the fundamental over the largest other bin, harmonics
/// included, in dB.
fn sfdr(name: &str, series: &Series, rest: &[Argument]) -> Result<RealValue, EvaluationError> {
    let analysis = analysis(name, series, rest)?;
    analysis
        .sfdr_db
        .map(RealValue::Scalar)
        .ok_or_else(|| coverage_refusal(name, analysis.harmonic_coverage))
}

/// `enob(...)` — the effective number of bits, (SINAD − 1.76 dB) / 6.02 dB.
fn enob(name: &str, series: &Series, rest: &[Argument]) -> Result<RealValue, EvaluationError> {
    sinad_db(name, series, rest).map(|sinad| RealValue::Scalar((sinad - 1.76) / 6.02))
}

#[cfg(test)]
mod tests;
