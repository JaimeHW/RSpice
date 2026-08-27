//! What the eye folds at, and where that came from.
//!
//! An eye is a picture of one bit period. Which bit period is a question the
//! waveform can usually but not always answer — a pattern whose shortest run
//! is two bits is genuinely indistinguishable from one running at half the
//! rate — so every commercial eye instrument takes the data rate as an input
//! and shows what it is currently folding at. This module carries both
//! halves: the reader's choice, and the provenance of whatever is on screen.

use serde::{Deserialize, Serialize};

use crate::analysis::eye_diagram::UiEstimateRejection;
use crate::product::{AnalysisInstanceId, DatasetId};

/// Where the eye's unit interval comes from.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum EyeTimebase {
    /// Recover it from the waveform.
    #[default]
    Auto,
    /// Fold at the unit interval the reader stated, in seconds.
    Explicit { unit_interval: f64 },
}

/// Which result a timebase choice belongs to.
///
/// Keyed on the prepared analysis instance so the choice survives a re-run of
/// the same plan card — re-running is exactly when a reader wants to keep the
/// rate they set. Results that predate prepared-task provenance fall back to
/// the dataset plus the run-local analysis id, which is still exact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EyeTimebaseKey {
    Prepared(AnalysisInstanceId),
    Legacy(DatasetId, u64),
}

impl EyeTimebaseKey {
    /// Only prepared keys are worth persisting: a legacy key names a
    /// run-local id that a later session will not mint again.
    pub fn is_persistable(&self) -> bool {
        matches!(self, Self::Prepared(_))
    }
}

/// What the eye on screen was actually folded at.
#[derive(Debug, Clone, PartialEq)]
pub enum EyeTimebaseProvenance {
    /// Recovered from the waveform.
    Auto {
        unit_interval: f64,
        edge_count: usize,
        rms_residual_ui: f64,
        low_confidence: bool,
    },
    /// Stated by the reader.
    Explicit { unit_interval: f64 },
    /// Nothing was folded: the waveform does not carry a recoverable bit
    /// period and the reader has not stated one.
    AutoRejected(UiEstimateRejection),
}

impl EyeTimebaseProvenance {
    /// Why no eye could be folded, in the reader's terms.
    ///
    /// Each says what is missing and what to do about it, because the remedy
    /// is the same in every case and the reader cannot be expected to infer
    /// it from a refusal.
    pub fn rejection_hint(&self, source: &str) -> Option<String> {
        let Self::AutoRejected(rejection) = self else {
            return None;
        };
        Some(match rejection {
            UiEstimateRejection::NoSignalSwing => {
                format!("{source} never changes level — an eye needs transitions to fold")
            }
            UiEstimateRejection::TooFewTransitions { crossings } => format!(
                "Only {crossings} transitions in {source} — set the data rate to fold the eye"
            ),
            UiEstimateRejection::NoConsistentFundamental { .. } => {
                format!("No consistent bit period in {source} — set the data rate to fold the eye")
            }
        })
    }
}

/// Read a data rate or a unit interval from what the reader typed.
///
/// Both spellings are in daily use — a SerDes is quoted in gigabits per
/// second, a DDR strobe in picoseconds — so both are accepted and
/// disambiguated the way an engineer would: an explicit `b/s`, `bps` or
/// `baud` is a rate, and so is any bare magnitude above a kilo, because
/// nobody folds an eye at a thousand seconds.
pub fn parse_eye_timebase(input: &str) -> Result<f64, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("Enter a data rate (2.5G, 1Gb/s) or a unit interval (400p, 1ns)".to_owned());
    }

    let lowered = trimmed.to_ascii_lowercase();
    let mut is_rate = false;
    let mut magnitude = trimmed;
    for suffix in ["bits/s", "bit/s", "baud", "bps", "b/s"] {
        if let Some(stripped) = lowered.strip_suffix(suffix) {
            is_rate = true;
            magnitude = &trimmed[..stripped.len()];
            break;
        }
    }

    let value = crate::quantity::parse_engineering_value(magnitude.trim())?;
    if !(value.is_finite() && value > 0.0) {
        return Err("The unit interval must be a positive time".to_owned());
    }

    let unit_interval = if is_rate || value > 1e3 {
        1.0 / value
    } else {
        value
    };
    if !(unit_interval.is_finite() && unit_interval > 0.0) {
        return Err("The unit interval must be a positive time".to_owned());
    }
    Ok(unit_interval)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rates_and_periods_are_told_apart_by_unit_and_by_magnitude() {
        // Explicit rate units.
        assert!((parse_eye_timebase("1Gb/s").unwrap() - 1e-9).abs() < 1e-18);
        assert!((parse_eye_timebase("2.5 Gbps").unwrap() - 400e-12).abs() < 1e-18);
        assert!((parse_eye_timebase("10Gbaud").unwrap() - 100e-12).abs() < 1e-18);
        // A bare magnitude above a kilo is a rate; nobody folds at 1e9 s.
        assert!((parse_eye_timebase("1G").unwrap() - 1e-9).abs() < 1e-18);
        // Times stay times.
        assert!((parse_eye_timebase("400p").unwrap() - 400e-12).abs() < 1e-18);
        assert!((parse_eye_timebase("1ns").unwrap() - 1e-9).abs() < 1e-18);
        assert!((parse_eye_timebase("1n").unwrap() - 1e-9).abs() < 1e-18);
    }

    #[test]
    fn a_timebase_that_is_not_a_positive_time_is_refused() {
        assert!(parse_eye_timebase("").is_err());
        assert!(parse_eye_timebase("0").is_err());
        assert!(parse_eye_timebase("-1n").is_err());
        assert!(parse_eye_timebase("fast").is_err());
    }
}
