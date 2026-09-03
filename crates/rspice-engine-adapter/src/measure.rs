//! Deterministic measurement extraction: canonical decimal formatting,
//! series identity hashing, and wire-grammar measurement names.
//!
//! Measurements expose a stable result shape — `name`,
//! `unit`, a canonical scalar decimal, `sample_count`, and `series_sha256` —
//! so evidence evaluation is a name-keyed comparison rather than a format
//! translation. Every representation choice here is part of the qualified
//! engine behavior: two runs of the same request must serialize identically
//! byte for byte.

use rspice_core::abort_signal::{AbortSignal, NoAbort};
use serde_json::Value;
use sha2::{Digest, Sha256};
use thiserror::Error;

/// One extracted measurement in manifest order.
pub struct Measurement {
    pub name: String,
    pub unit: &'static str,
    pub value_decimal: String,
    pub sample_count: usize,
    pub series_sha256: Option<String>,
}

impl Measurement {
    /// A scalar observation: one sample, no series identity.
    pub fn scalar(name: String, unit: &'static str, value: f64) -> Option<Self> {
        Some(Self {
            name,
            unit,
            value_decimal: canonical_decimal(value)?,
            sample_count: 1,
            series_sha256: None,
        })
    }

    /// A waveform observation. The scalar is the series' final sample — the
    /// settled value for sweeps and transients, the last frequency point for
    /// spectra — and the hash commits to every sample in order.
    pub fn series(name: String, unit: &'static str, samples: &[f64]) -> Option<Self> {
        Self::series_with_abort(name, unit, samples, &NoAbort).ok()?
    }

    pub fn series_with_abort(
        name: String,
        unit: &'static str,
        samples: &[f64],
        abort: &dyn AbortSignal,
    ) -> Result<Option<Self>, MeasurementError> {
        let Some(&last) = samples.last() else {
            return Ok(None);
        };
        Ok(Some(Self {
            name,
            unit,
            value_decimal: canonical_decimal(last).ok_or(MeasurementError::NonFinite)?,
            sample_count: samples.len(),
            series_sha256: Some(
                series_sha256_with_abort(samples, abort)?.ok_or(MeasurementError::NonFinite)?,
            ),
        }))
    }

    pub fn to_manifest_value(&self) -> Value {
        serde_json::json!({
            "name": self.name,
            "unit": self.unit,
            "value_decimal": self.value_decimal,
            "sample_count": self.sample_count,
            "series_sha256": self.series_sha256,
        })
    }
}

/// Formats a finite value in the canonical wire decimal grammar
/// `^-?(0|[1-9][0-9]*)(\.[0-9]+)?([eE][+-]?[0-9]{1,3})?$`.
///
/// Rust's `{:e}` is the shortest round-trip representation with a normalized
/// single-digit mantissa and a bare exponent, which satisfies the grammar for
/// every finite double; negative zero is folded to `0e0` so algebraically
/// equal results cannot differ by IEEE sign bookkeeping. `None` for
/// non-finite values — those are refusals upstream, never measurements.
pub fn canonical_decimal(value: f64) -> Option<String> {
    if !value.is_finite() {
        return None;
    }
    if value == 0.0 {
        return Some("0e0".to_owned());
    }
    Some(format!("{value:e}"))
}

/// Series identity: the SHA-256 of a versioned, newline-delimited canonical
/// decimal rendering of every sample. Reproducibility conformance compares
/// exactly this value across repetitions.
pub fn series_sha256(samples: &[f64]) -> Option<String> {
    series_sha256_with_abort(samples, &NoAbort).ok()?
}

pub fn series_sha256_with_abort(
    samples: &[f64],
    abort: &dyn AbortSignal,
) -> Result<Option<String>, MeasurementError> {
    let mut hasher = Sha256::new();
    hasher.update(b"rspice-series-v1\n");
    for sample in samples {
        if abort.is_aborted() {
            return Err(MeasurementError::Aborted);
        }
        let decimal = canonical_decimal(*sample).ok_or(MeasurementError::NonFinite)?;
        hasher.update(decimal.as_bytes());
        hasher.update(b"\n");
    }
    let digest: [u8; 32] = hasher.finalize().into();
    Ok(Some(crate::wire::digest_hex(&digest)))
}

#[derive(Debug, Clone, Copy, Error, PartialEq, Eq)]
pub enum MeasurementError {
    #[error("measurement construction was cancelled")]
    Aborted,
    #[error("measurement contains a non-finite value")]
    NonFinite,
}

/// Maps an engine-side signal label into the measurement-name grammar
/// `^[a-z][a-z0-9_().:+/\[\]-]{0,119}$`: lowercased, disallowed bytes folded
/// to `-`, truncated to fit. Callers prefix with a probe form (`v(...)`),
/// so the leading character is always a letter.
pub fn measurement_name(prefix: &str, label: &str) -> String {
    let mut name = String::with_capacity(prefix.len() + label.len() + 1);
    name.push_str(prefix);
    name.push('(');
    for byte in label.bytes() {
        let lower = byte.to_ascii_lowercase();
        name.push(match lower {
            b'a'..=b'z' | b'0'..=b'9' | b'_' | b'.' | b':' | b'+' | b'/' | b'[' | b']' | b'-' => {
                lower as char
            }
            _ => '-',
        });
        if name.len() >= 119 {
            break;
        }
    }
    name.push(')');
    name
}

/// Sorts measurements by name and drops nothing silently: a duplicate name
/// after sanitization is disambiguated with a deterministic ordinal suffix.
pub fn finalize_measurements(mut measurements: Vec<Measurement>) -> Vec<Measurement> {
    measurements.sort_by(|a, b| a.name.cmp(&b.name));
    let mut previous: Option<(String, usize)> = None;
    for measurement in &mut measurements {
        match &mut previous {
            Some((name, ordinal)) if *name == measurement.name => {
                *ordinal += 1;
                measurement.name = format!("{}-{ordinal}", measurement.name);
            }
            _ => previous = Some((measurement.name.clone(), 1)),
        }
    }
    measurements
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grammar(value: &str) -> bool {
        // The canonical wire decimal grammar, transliterated.
        let rest = value.strip_prefix('-').unwrap_or(value);
        let (mantissa, exponent) = match rest.split_once(['e', 'E']) {
            Some((mantissa, exponent)) => (mantissa, Some(exponent)),
            None => (rest, None),
        };
        let (integer, fraction) = match mantissa.split_once('.') {
            Some((integer, fraction)) => (integer, Some(fraction)),
            None => (mantissa, None),
        };
        let integer_ok = integer == "0"
            || (integer.len() <= 31
                && integer.as_bytes()[0] != b'0'
                && integer.bytes().all(|b| b.is_ascii_digit()));
        let fraction_ok = fraction.is_none_or(|fraction| {
            (1..=30).contains(&fraction.len()) && fraction.bytes().all(|b| b.is_ascii_digit())
        });
        let exponent_ok = exponent.is_none_or(|exponent| {
            let digits = exponent.strip_prefix(['+', '-']).unwrap_or(exponent);
            !digits.is_empty()
                && digits.len() <= 3
                && (digits == "0" || digits.as_bytes()[0] != b'0')
                && digits.bytes().all(|b| b.is_ascii_digit())
        });
        !integer.is_empty() && integer_ok && fraction_ok && exponent_ok
    }

    #[test]
    fn canonical_decimals_satisfy_the_wire_grammar_and_round_trip() {
        for value in [
            0.0,
            -0.0,
            1.0,
            -1.5,
            2.5e-3,
            1.0 / 3.0,
            6.626e-34,
            1.7976931348623157e308,
            5e-324,
            -273.15,
            1e3,
        ] {
            let encoded = canonical_decimal(value).expect("finite value must encode");
            assert!(grammar(&encoded), "{encoded} violates the decimal grammar");
            let decoded: f64 = encoded.parse().expect("canonical decimal must parse");
            if value == 0.0 {
                assert_eq!(decoded, 0.0);
            } else {
                assert_eq!(decoded, value, "{encoded} must round-trip exactly");
            }
        }
        assert_eq!(canonical_decimal(f64::NAN), None);
        assert_eq!(canonical_decimal(f64::INFINITY), None);
        assert_eq!(canonical_decimal(-0.0).as_deref(), Some("0e0"));
    }

    #[test]
    fn series_hashes_commit_to_order_and_every_sample() {
        let forward = series_sha256(&[1.0, 2.0, 3.0]).expect("series hash");
        let reversed = series_sha256(&[3.0, 2.0, 1.0]).expect("series hash");
        let truncated = series_sha256(&[1.0, 2.0]).expect("series hash");
        assert_ne!(forward, reversed);
        assert_ne!(forward, truncated);
        assert_eq!(
            forward,
            series_sha256(&[1.0, 2.0, 3.0]).expect("series hash")
        );
        assert_eq!(series_sha256(&[1.0, f64::NAN]), None);
    }

    #[test]
    fn measurement_names_fit_the_wire_grammar() {
        assert_eq!(measurement_name("v", "OUT"), "v(out)");
        assert_eq!(measurement_name("i", "V1#branch"), "i(v1-branch)");
        let long = measurement_name("v", &"n".repeat(400));
        assert!(long.len() <= 120, "grammar caps names at 120 characters");
        assert!(long.ends_with(')'));
    }

    #[test]
    fn duplicate_names_disambiguate_deterministically() {
        let measurements = finalize_measurements(vec![
            Measurement::scalar("v(out)".into(), "V", 1.0).expect("measurement"),
            Measurement::scalar("v(out)".into(), "V", 2.0).expect("measurement"),
            Measurement::scalar("i(v1)".into(), "A", 3.0).expect("measurement"),
        ]);
        let names: Vec<_> = measurements.iter().map(|m| m.name.as_str()).collect();
        assert_eq!(names, ["i(v1)", "v(out)", "v(out)-2"]);
    }
}
