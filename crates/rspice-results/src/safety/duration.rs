//! Retained SOA duration policy and evidence, independent of execution.
use super::{SoARuleVerdict, SoaThresholds};
use serde::{Deserialize, Serialize};

/// Retrospective duration screening; recovery applies only between excursions.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum SoaDurationMode {
    #[default]
    PerExcursion,
    Cumulative {
        recovery_time_s: Option<f64>,
    },
}
impl SoaDurationMode {
    pub fn is_default(&self) -> bool {
        *self == Self::PerExcursion
    }

    pub fn validate(self, minimum_duration_s: Option<f64>) -> Result<(), String> {
        if minimum_duration_s.is_some_and(|value| !value.is_finite() || value <= 0.0) {
            return Err("SOA duration threshold must be finite and positive".into());
        }
        if let Self::Cumulative { recovery_time_s } = self {
            if minimum_duration_s.is_none() {
                return Err("Cumulative SOA exposure requires a duration threshold".into());
            }
            if recovery_time_s.is_some_and(|value| !value.is_finite() || value <= 0.0) {
                return Err("SOA recovery time must be finite and positive".into());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaCumulativeDurationEvidence {
    pub recovery_time_s: Option<f64>,
    pub peak_exposure_s: f64,
    pub final_exposure_s: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaDurationEvidence {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cumulative: Option<SoaCumulativeDurationEvidence>,
    pub minimum_duration_s: f64,
    pub total_exceedance_s: f64,
    pub longest_excursion_s: f64,
    pub qualified_excursions: u64,
    pub rejected_excursions: u64,
    pub clipped_excursions: u64,
}
impl SoaDurationEvidence {
    pub fn mode(self) -> SoaDurationMode {
        self.cumulative
            .map_or(SoaDurationMode::PerExcursion, |value| {
                SoaDurationMode::Cumulative {
                    recovery_time_s: value.recovery_time_s,
                }
            })
    }

    pub fn validate(self) -> Result<(), String> {
        self.mode().validate(Some(self.minimum_duration_s))?;
        if let Some(cumulative) = self.cumulative
            && (!cumulative.peak_exposure_s.is_finite()
                || cumulative.peak_exposure_s < 0.0
                || cumulative.peak_exposure_s > self.total_exceedance_s
                || !cumulative.final_exposure_s.is_finite()
                || cumulative.final_exposure_s < 0.0
                || cumulative.final_exposure_s > cumulative.peak_exposure_s)
        {
            return Err("SOA cumulative duration evidence is invalid".into());
        }
        if !self.minimum_duration_s.is_finite()
            || self.minimum_duration_s <= 0.0
            || !self.total_exceedance_s.is_finite()
            || self.total_exceedance_s < 0.0
            || !self.longest_excursion_s.is_finite()
            || self.longest_excursion_s < 0.0
            || self.longest_excursion_s > self.total_exceedance_s
            || self
                .qualified_excursions
                .checked_add(self.rejected_excursions)
                .is_none_or(|count| self.clipped_excursions > count)
        {
            return Err("SOA duration evidence is invalid".into());
        }
        Ok(())
    }
}

pub fn soa_duration_verdict(
    thresholds: SoaThresholds,
    actual: f64,
    limit: f64,
    qualified: bool,
) -> SoARuleVerdict {
    let verdict = thresholds.verdict(actual, limit);
    if !qualified
        && matches!(
            verdict,
            SoARuleVerdict::Violation | SoARuleVerdict::Critical
        )
    {
        if thresholds.warning_fraction.is_some() {
            SoARuleVerdict::Warning
        } else {
            SoARuleVerdict::Pass
        }
    } else {
        verdict
    }
}
