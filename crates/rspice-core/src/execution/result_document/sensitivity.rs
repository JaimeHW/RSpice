//! Reconcile retained derivatives with their nominal output and parameter.

use super::*;
use crate::analysis::sensitivity::{AcSensitivityDerived, SensitivityValue};
use std::collections::HashSet;

fn invalid(vector: &str, quantity: &str, row: usize) -> ResultDocumentError {
    ResultDocumentError::Malformed {
        location: "sensitivity projections",
        detail: format!(
            "'{vector}' {quantity} at sample {row} disagrees with its nominal output and absolute derivative"
        ),
    }
}

// Permit platform rounding in derived projections without a dimensionful
// absolute floor that could hide a missing or fabricated tiny derivative.
fn close(actual: f64, expected: f64) -> bool {
    if actual == expected {
        return true;
    }
    if !actual.is_finite() || !expected.is_finite() || actual == 0.0 || expected == 0.0 {
        return false;
    }
    let scale = actual.abs().max(expected.abs());
    (actual / scale - expected / scale).abs() <= 64.0 * f64::EPSILON
}

fn agrees<T>(
    actual: SensitivityValue<T>,
    expected: SensitivityValue<T>,
    close: impl Fn(T, T) -> bool,
) -> bool {
    match (actual, expected) {
        (SensitivityValue::Available(actual), SensitivityValue::Available(expected)) => {
            close(actual, expected)
        }
        (
            SensitivityValue::Unavailable {
                unavailable: actual,
            },
            SensitivityValue::Unavailable {
                unavailable: expected,
            },
        ) => actual == expected,
        _ => false,
    }
}

pub(super) fn validate(
    document: &AnalysisResultDocument,
    payload: &SensitivityPayload,
    abort: &dyn AbortSignal,
) -> Result<(), ResultDocumentError> {
    let mut identities = HashSet::new();
    for vector in payload
        .entries
        .iter()
        .map(|entry| &entry.vector_name)
        .chain(payload.ac_entries.iter().map(|entry| &entry.vector_name))
    {
        check_abort(abort)?;
        if !identities.insert(vector.to_ascii_lowercase()) {
            return Err(ResultDocumentError::Malformed {
                location: "sensitivity identity",
                detail: format!("duplicate sensitivity vector '{vector}'"),
            });
        }
    }
    if !payload.entries.is_empty() {
        let output = document
            .scalars
            .iter()
            .find_map(|scalar| {
                if scalar.name == "output_value"
                    && let ScalarValue::Real { value } = scalar.value
                {
                    value
                } else {
                    None
                }
            })
            .ok_or_else(|| invalid(&payload.output, "nominal output", 0))?;
        for entry in &payload.entries {
            check_abort(abort)?;
            let expected =
                SensitivityValue::normalized(entry.nominal_value, entry.absolute, output);
            if !agrees(entry.normalized, expected, close) {
                return Err(invalid(&entry.vector_name, "normalized derivative", 0));
            }
        }
    }
    if !payload.ac_entries.is_empty() {
        let outputs = document
            .signals
            .iter()
            .find_map(|signal| {
                if signal.descriptor.canonical_name() == "output"
                    && let SeriesValues::Complex { samples } = &signal.values
                {
                    Some(samples)
                } else {
                    None
                }
            })
            .ok_or_else(|| invalid(&payload.output, "nominal output", 0))?;
        for entry in &payload.ac_entries {
            check_abort(abort)?;
            if entry.absolute.len() != document.point_count || outputs.len() != document.point_count
            {
                return Err(invalid(&entry.vector_name, "trace length", 0));
            }
            // Payload validation already requires all four derivative traces
            // to have the same length; series validation checks the output.
            for (row, ((((output, derivative), normalized), magnitude), phase)) in outputs
                .iter()
                .zip(&entry.absolute)
                .zip(&entry.normalized)
                .zip(&entry.magnitude)
                .zip(&entry.phase)
                .enumerate()
            {
                check_abort(abort)?;
                let output =
                    output.ok_or_else(|| invalid(&payload.output, "nominal output", row))?;
                let expected = AcSensitivityDerived::new(
                    entry.nominal_value,
                    output.into(),
                    (*derivative).into(),
                )
                .map_err(|_| invalid(&entry.vector_name, "derived arithmetic", row))?;
                if !agrees(
                    normalized.map(Into::<crate::Complex64>::into),
                    expected.normalized,
                    |actual, expected| {
                        close(actual.re, expected.re) && close(actual.im, expected.im)
                    },
                ) {
                    return Err(invalid(&entry.vector_name, "normalized derivative", row));
                }
                if !agrees(*magnitude, expected.magnitude, close) {
                    return Err(invalid(&entry.vector_name, "magnitude derivative", row));
                }
                if !agrees(*phase, expected.phase, close) {
                    return Err(invalid(&entry.vector_name, "phase derivative", row));
                }
            }
        }
    }
    Ok(())
}
