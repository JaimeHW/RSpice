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

#[derive(Clone, Copy)]
pub enum SoaLimitTrace<'a> {
    Constant(f64),
    Samples(&'a [f64]),
}
impl SoaLimitTrace<'_> {
    pub fn at(self, index: usize) -> f64 {
        match self {
            Self::Constant(value) => value,
            Self::Samples(values) => values[index],
        }
    }
}

#[derive(Debug, Clone)]
pub struct SoaExcursion {
    pub start_s: f64,
    pub end_s: f64,
    pub first_sample: usize,
    pub last_sample: usize,
    pub qualified: bool,
}

pub struct SoaDurationScan {
    pub evidence: SoaDurationEvidence,
    pub qualified_samples: Vec<bool>,
    pub excursions: Vec<SoaExcursion>,
}

/// An invalid trace, invalid duration threshold, or interrupted scan.
#[derive(Debug)]
pub enum SoaDurationScanError {
    InvalidMinimumDuration(f64),
    InvalidInput(String),
    Aborted,
}

/// Accumulated exposure starts at zero at the observation-window boundary.
/// It increases with above-limit time, optionally decays exponentially in gaps,
/// and qualifies each whole excursion using its exposure at that excursion's end.
pub fn scan_soa_duration_with_mode(
    time: &[f64],
    stress: &[f64],
    limits: SoaLimitTrace<'_>,
    minimum_duration_s: f64,
    mode: SoaDurationMode,
    mut is_aborted: impl FnMut() -> bool,
) -> Result<SoaDurationScan, SoaDurationScanError> {
    let invalid = |message: &str| SoaDurationScanError::InvalidInput(message.into());
    if !minimum_duration_s.is_finite() || minimum_duration_s <= 0.0 {
        return Err(SoaDurationScanError::InvalidMinimumDuration(
            minimum_duration_s,
        ));
    }
    mode.validate(Some(minimum_duration_s))
        .map_err(SoaDurationScanError::InvalidInput)?;
    if time.is_empty()
        || time.len() != stress.len()
        || matches!(limits, SoaLimitTrace::Samples(values) if values.len() != time.len())
    {
        return Err(invalid(
            "SOA duration traces have incomplete sample coverage",
        ));
    }
    for i in 0..time.len() {
        if i % 256 == 0 && is_aborted() {
            return Err(SoaDurationScanError::Aborted);
        }
        if !time[i].is_finite()
            || time[i] < 0.0
            || (i > 0 && time[i] <= time[i - 1])
            || !stress[i].is_finite()
            || stress[i] < 0.0
            || !limits.at(i).is_finite()
            || limits.at(i) < 0.0
        {
            return Err(invalid(
                "SOA duration requires increasing finite times and nonnegative finite stress/limits",
            ));
        }
    }
    // Stable root fraction for opposite-signed margins, even near f64::MAX.
    let crossing = |i: usize| {
        let a = (stress[i - 1] - limits.at(i - 1)).abs();
        let b = (stress[i] - limits.at(i)).abs();
        let scale = a.max(b);
        let fraction = (a / scale) / (a / scale + b / scale);
        time[i - 1] + (time[i] - time[i - 1]) * fraction
    };
    let mut excursions = Vec::new();
    let mut open = (stress[0] > limits.at(0)).then_some((time[0], 0usize));
    for i in 1..time.len() {
        if i % 256 == 0 && is_aborted() {
            return Err(SoaDurationScanError::Aborted);
        }
        let before = stress[i - 1] > limits.at(i - 1);
        let after = stress[i] > limits.at(i);
        if !before && after {
            open = Some((crossing(i), i));
        }
        if before && !after {
            let (start_s, first_sample) = open.take().expect("open positive excursion");
            let end_s = crossing(i);
            excursions.push(SoaExcursion {
                start_s,
                end_s,
                first_sample,
                last_sample: i - 1,
                qualified: end_s - start_s >= minimum_duration_s,
            });
        }
    }
    if let Some((start_s, first_sample)) = open {
        let end_s = time[time.len() - 1];
        excursions.push(SoaExcursion {
            start_s,
            end_s,
            first_sample,
            last_sample: time.len() - 1,
            qualified: end_s - start_s >= minimum_duration_s,
        });
    }
    let mut evidence = SoaDurationEvidence {
        cumulative: match mode {
            SoaDurationMode::PerExcursion => None,
            SoaDurationMode::Cumulative { recovery_time_s } => {
                Some(SoaCumulativeDurationEvidence {
                    recovery_time_s,
                    peak_exposure_s: 0.0,
                    final_exposure_s: 0.0,
                })
            }
        },
        minimum_duration_s,
        total_exceedance_s: 0.0,
        longest_excursion_s: 0.0,
        qualified_excursions: 0,
        rejected_excursions: 0,
        clipped_excursions: 0,
    };
    let mut qualified_samples = vec![false; time.len()];
    let mut correction = 0.0;
    let mut previous_end = time[0];
    for (index, excursion) in excursions.iter_mut().enumerate() {
        if index % 256 == 0 && is_aborted() {
            return Err(SoaDurationScanError::Aborted);
        }
        let duration = excursion.end_s - excursion.start_s;
        let increment = duration - correction;
        let total = evidence.total_exceedance_s + increment;
        correction = (total - evidence.total_exceedance_s) - increment;
        evidence.total_exceedance_s = total;
        evidence.longest_excursion_s = evidence.longest_excursion_s.max(duration);
        if let Some(cumulative) = &mut evidence.cumulative {
            cumulative.final_exposure_s = if let Some(tau) = cumulative.recovery_time_s {
                let gap = excursion.start_s - previous_end;
                // Rounding cannot let recovered exposure exceed total exposure.
                (cumulative.final_exposure_s * (-gap / tau).exp() + duration).min(total)
            } else {
                total
            };
            cumulative.peak_exposure_s =
                cumulative.peak_exposure_s.max(cumulative.final_exposure_s);
            excursion.qualified = cumulative.final_exposure_s >= minimum_duration_s;
        }
        previous_end = excursion.end_s;
        if excursion.qualified {
            evidence.qualified_excursions += 1;
        } else {
            evidence.rejected_excursions += 1;
        }
        if excursion.first_sample == 0 || excursion.last_sample == time.len() - 1 {
            evidence.clipped_excursions += 1;
        }
        if excursion.qualified {
            for chunk in
                qualified_samples[excursion.first_sample..=excursion.last_sample].chunks_mut(256)
            {
                if is_aborted() {
                    return Err(SoaDurationScanError::Aborted);
                }
                chunk.fill(true);
            }
        }
    }
    if let Some(cumulative) = &mut evidence.cumulative
        && let Some(tau) = cumulative.recovery_time_s
    {
        cumulative.final_exposure_s *= (-(time[time.len() - 1] - previous_end) / tau).exp();
    }
    evidence
        .validate()
        .map_err(SoaDurationScanError::InvalidInput)?;
    Ok(SoaDurationScan {
        evidence,
        qualified_samples,
        excursions,
    })
}
