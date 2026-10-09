//! Reconcile Bode projections, retained Nyquist samples and observed crossings.

use super::{invalid, *};
use crate::analysis::stb::{BodePoint, StbAnalyzer};

fn close(actual: f64, expected: f64, scale: f64) -> bool {
    actual.is_finite()
        && expected.is_finite()
        && (actual - expected).abs()
            <= 64.0 * f64::EPSILON * actual.abs().max(expected.abs()).max(scale)
}

fn real_series<'a>(
    document: &'a AnalysisResultDocument,
    name: &str,
    unit: SignalUnit,
) -> Result<Option<&'a [Option<f64>]>, ResultDocumentError> {
    let Some(signal) = document
        .signals
        .iter()
        .find(|signal| signal.descriptor.canonical_name() == name)
    else {
        return Ok(None);
    };
    let SeriesValues::Real { samples } = &signal.values else {
        return Err(invalid("Bode ordinates must be real"));
    };
    if signal.descriptor.unit() != &unit {
        return Err(invalid("Bode ordinate has inconsistent units"));
    }
    Ok(Some(samples))
}

fn retained_samples<'a>(
    frequencies: &'a [f64],
    raw: Option<&'a [Option<ComplexSample>]>,
    nyquist: &'a [NyquistSample],
) -> impl Iterator<Item = (usize, f64, Option<ComplexSample>, Option<&'a NyquistSample>)> {
    let mut nyquist = nyquist.iter().peekable();
    frequencies
        .iter()
        .enumerate()
        .map(move |(index, &frequency)| {
            let sample = nyquist.next_if(|point| point.frequency == frequency);
            (
                index,
                frequency,
                raw.and_then(|samples| samples.get(index))
                    .copied()
                    .flatten(),
                sample,
            )
        })
}

pub(super) fn validate(
    document: &AnalysisResultDocument,
    payload: &StabilityPayload,
    abort: &dyn AbortSignal,
) -> Result<(), ResultDocumentError> {
    if document.point_count == 0 && document.axes.is_empty() && payload.nyquist.is_empty() {
        return Ok(());
    }
    let [axis] = document.axes.as_slice() else {
        return Err(invalid("STB requires one frequency axis"));
    };
    if axis.kind != ResultAxisKind::Frequency || axis.unit != SignalUnit::Hertz {
        return Err(invalid("STB coordinates must be frequencies in hertz"));
    }
    let AxisValues::Real {
        values: frequencies,
    } = &axis.values
    else {
        return Err(invalid("STB frequencies must use real coordinates"));
    };
    let mut previous_frequency = 0.0;
    let mut nyquist_coordinates = payload.nyquist.iter().peekable();
    for (index, &frequency) in frequencies.iter().enumerate() {
        if index % ABORT_POLL_STRIDE == 0 {
            check_abort(abort)?;
        }
        if !frequency.is_finite() || frequency <= previous_frequency {
            return Err(invalid(
                "STB frequencies must be positive and strictly increasing",
            ));
        }
        previous_frequency = frequency;
        if nyquist_coordinates
            .peek()
            .is_some_and(|point| point.frequency < frequency)
        {
            return Err(invalid(
                "Nyquist coordinates must be an ordered subset of the Bode frequencies",
            ));
        }
        nyquist_coordinates.next_if(|point| point.frequency == frequency);
    }
    if nyquist_coordinates.peek().is_some() {
        return Err(invalid(
            "Nyquist coordinates must be an ordered subset of the Bode frequencies",
        ));
    }
    let mut raw = None;
    for signal in &document.signals {
        check_abort(abort)?;
        if signal.qualifier.is_some() {
            return Err(invalid("STB series must be unqualified"));
        }
        if signal.descriptor.canonical_name() == "loop_gain" {
            let SeriesValues::Complex { samples } = &signal.values else {
                return Err(invalid("loop gain must retain complex samples"));
            };
            if signal.descriptor.unit() != &SignalUnit::Dimensionless {
                return Err(invalid("loop gain must be dimensionless"));
            }
            raw = Some(samples.as_slice());
        }
    }
    let magnitude = real_series(document, "loop_gain_magnitude", SignalUnit::Dimensionless)?;
    let db = real_series(document, "loop_gain_db", SignalUnit::Custom("dB".into()))?;
    let phase = real_series(document, "loop_gain_phase", SignalUnit::Degree)?;
    let mut complete = !frequencies.is_empty();
    let mut previous_phase = None;
    let mut anchored_phase = true;
    for (index, frequency, raw, nyquist) in retained_samples(frequencies, raw, &payload.nyquist) {
        if index % ABORT_POLL_STRIDE == 0 {
            check_abort(abort)?;
        }
        let nyquist = nyquist.map(|point| ComplexSample::new(point.real, point.imaginary));
        if let (Some(raw), Some(nyquist)) = (raw, nyquist)
            && raw != nyquist
        {
            return Err(invalid(
                "Nyquist samples disagree with the complex loop gain",
            ));
        }
        let Some(gain) = raw.or(nyquist) else {
            complete = false;
            previous_phase = None;
            anchored_phase = false;
            continue;
        };
        let mut expected = BodePoint::from_loop_gain(frequency, gain.into());
        expected.unwrap_after(previous_phase);
        previous_phase = expected.phase_deg;
        for (series, wanted, scale) in [
            (magnitude, expected.magnitude, 0.0),
            (db, expected.magnitude_db, 1.0),
        ] {
            if let Some(actual) = series
                .and_then(|samples| samples.get(index))
                .copied()
                .flatten()
                && !wanted.is_some_and(|wanted| close(actual, wanted, scale))
            {
                return Err(invalid(
                    "Bode magnitude disagrees with the complex loop gain",
                ));
            }
        }
        if let Some(actual) = phase
            .and_then(|samples| samples.get(index))
            .copied()
            .flatten()
        {
            let Some(wanted) = expected.phase_deg else {
                return Err(invalid("zero loop gain has no defined phase"));
            };
            let agrees = if anchored_phase && document.schema_version >= 15 {
                close(actual, wanted, 1.0)
            } else {
                // Missing source samples leave the unwrap offset unknown.
                // The retained phase must still have the measured direction.
                let wrapped = actual % 360.0;
                let mut direction = BodePoint::from_loop_gain(frequency, gain.into());
                direction.unwrap_after(Some(wrapped));
                let maximum_unwrapped_phase = 180.0 * (index as f64 + 1.0);
                direction
                    .phase_deg
                    .is_some_and(|value| close(wrapped, value, 1.0))
                    && (actual.abs() <= maximum_unwrapped_phase
                        || close(actual.abs(), maximum_unwrapped_phase, 0.0))
            };
            if !agrees {
                return Err(invalid("Bode phase disagrees with the complex loop gain"));
            }
        }
        if expected.phase_deg.is_none() {
            anchored_phase = true;
        }
    }
    if complete && document.schema_version >= 15 {
        let mut previous_phase = None;
        let points = retained_samples(frequencies, raw, &payload.nyquist).filter_map(
            |(_, frequency, raw, nyquist)| {
                let gain = raw.or_else(|| {
                    nyquist.map(|point| ComplexSample::new(point.real, point.imaginary))
                })?;
                let mut point = BodePoint::from_loop_gain(frequency, gain.into());
                point.unwrap_after(previous_phase);
                previous_phase = point.phase_deg;
                Some(point)
            },
        );
        let expected =
            StbAnalyzer::extract_margins(points, abort).map_err(|error| match error {
                crate::analysis::stb::StbAnalysisError::Aborted => ResultDocumentError::Aborted,
                _ => invalid("cannot reconstruct the retained STB crossings"),
            })?;
        for (name, expected, scale) in [
            (
                "gain_margin_db",
                expected.gain_margin.map(|margin| margin.value),
                1.0,
            ),
            (
                "gain_margin_frequency",
                expected.gain_margin.map(|margin| margin.frequency),
                0.0,
            ),
            (
                "phase_margin_degrees",
                expected.phase_margin.map(|margin| margin.value),
                1.0,
            ),
            (
                "phase_margin_frequency",
                expected.phase_margin.map(|margin| margin.frequency),
                0.0,
            ),
        ] {
            let scalar = document
                .scalars
                .iter()
                .find(|scalar| scalar.name == name)
                .ok_or_else(|| invalid("missing STB margin evidence"))?;
            let matches = match (&scalar.value, expected) {
                (
                    ScalarValue::Real {
                        value: Some(actual),
                    },
                    Some(expected),
                ) => close(*actual, expected, scale),
                (
                    ScalarValue::Unavailable {
                        reason: ScalarUnavailability::NoCrossover,
                    },
                    None,
                ) => true,
                _ => false,
            };
            if !matches {
                return Err(invalid(
                    "STB margins disagree with the retained loop-gain crossings",
                ));
            }
        }
        if !document.scalars.iter().any(|scalar| {
            scalar.name == "unity_gain_crossovers"
                && scalar.value
                    == (ScalarValue::Count {
                        value: expected.num_crossovers as u64,
                    })
        }) {
            return Err(invalid(
                "STB crossover count disagrees with the retained loop gain",
            ));
        }
    }
    check_abort(abort)
}
