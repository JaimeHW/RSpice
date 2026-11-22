//! What the eye folds at, and where that came from.
//!
//! An eye is a picture of one bit period. Which bit period is a question the
//! waveform can usually but not always answer — a pattern whose shortest run
//! is two bits is genuinely indistinguishable from one running at half the
//! rate — so every commercial eye instrument takes the data rate as an input
//! and shows what it is currently folding at. This module carries both
//! halves: the reader's choice, and the provenance of whatever is on screen.

use serde::{Deserialize, Serialize};

use crate::eye_diagram::UiEstimateRejection;
use rspice_vocabulary::product::{AnalysisInstanceId, DatasetId};

/// Where the eye's unit interval comes from.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum EyeTimebase {
    /// Recover it from the waveform.
    #[default]
    Auto,
    /// Fold at the unit interval the reader stated, in seconds.
    Explicit { unit_interval: f64 },
}

/// A clock signal an eye is triggered from: each acquisition starts at one
/// of its rising crossings of `threshold`, in volts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EyeClockSignal {
    pub signal: String,
    pub threshold: f64,
}

/// What folds the eye, as ViVA's eye assistant offers it: a recovered or a
/// stated ideal clock, with an optional offset, or a clock signal with a
/// threshold; and the signal folded.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EyeFold {
    /// The signal folded; `None` folds the signal the spectrum reads.
    pub source: Option<String>,
    /// The ideal clock's period, recovered or stated.
    pub timebase: EyeTimebase,
    /// When the ideal clock's first edge falls, in seconds: the time every
    /// acquisition starts at, a period apart. `None` places the data's
    /// crossings half a unit interval in, so the opening is centred.
    pub offset: Option<f64>,
    /// The clock signal the eye is triggered from instead of an ideal
    /// clock.
    pub clock: Option<EyeClockSignal>,
}

impl EyeFold {
    /// Whether the offset and the clock a loaded eye was folded with are
    /// the ones asked for, ignoring the period, which the provenance
    /// answers.
    pub fn same_trigger(&self, other: &Self) -> bool {
        self.source == other.source
            && self.offset.map(f64::to_bits) == other.offset.map(f64::to_bits)
            && self.clock == other.clock
    }
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
    Explicit {
        unit_interval: f64,
        /// The record's crossings do not agree on a phase at this rate.
        ///
        /// The eye is folded anyway — the reader asked for this rate, and
        /// refusing it would make the control decorative — but every figure
        /// quoted off it is then a property of the fold rather than of the
        /// signal, so the viewer has to say so.
        ///
        /// This is a fact about the data at the requested rate, not a second
        /// rate: it deliberately takes no part in the staleness comparison
        /// that decides whether the eye on screen still answers the rate
        /// being asked for.
        incoherent: bool,
    },
    /// Triggered from a clock signal's rising edges, at their median
    /// spacing.
    Triggered {
        unit_interval: f64,
        edge_count: usize,
    },
    /// Nothing was folded: the waveform does not carry a recoverable bit
    /// period and the reader has not stated one.
    AutoRejected(UiEstimateRejection),
    /// Nothing was folded: the clock signal does not trigger, or is not in
    /// the result; the reason in the reader's words.
    ClockRejected(String),
    /// Nothing was folded: the record could not be read, its store having failed or the record
    /// being larger than the loaded results may hold.
    Unreadable(String),
}

impl EyeTimebaseProvenance {
    /// Why no eye could be folded, in the reader's terms.
    ///
    /// Each says what is missing and what to do about it, because the remedy
    /// is the same in every case and the reader cannot be expected to infer
    /// it from a refusal.
    pub fn rejection_hint(&self, source: &str) -> Option<String> {
        let rejection = match self {
            Self::AutoRejected(rejection) => rejection,
            Self::Unreadable(reason) => {
                return Some(format!("{source} could not be read: {reason}"));
            }
            Self::ClockRejected(reason) => return Some(reason.clone()),
            Self::Auto { .. } | Self::Explicit { .. } | Self::Triggered { .. } => return None,
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

    let value = rspice_vocabulary::quantity::parse_engineering_value(magnitude.trim())?;
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
