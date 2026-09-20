//! Configurable warning and critical bands around the actual SOA limit.
use super::SoARuleVerdict;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SoaThresholds {
    /// Fraction of the actual limit above which a near-limit warning is emitted.
    /// None disables warnings; actual limit exceedances remain violations.
    pub warning_fraction: Option<f64>,
    /// Fraction above which a violation is critical. None keeps it a violation.
    pub critical_fraction: Option<f64>,
}

impl Default for SoaThresholds {
    fn default() -> Self {
        Self {
            warning_fraction: Some(0.9),
            critical_fraction: Some(1.2),
        }
    }
}

impl SoaThresholds {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }

    pub fn validate(self) -> Result<(), String> {
        if self
            .warning_fraction
            .is_some_and(|value| !value.is_finite() || !(0.0..=1.0).contains(&value))
        {
            return Err(
                "SOA warning threshold must be between 0% and 100% of the limit, or disabled"
                    .into(),
            );
        }
        if self
            .critical_fraction
            .is_some_and(|value| !(value * 100.0).is_finite() || value < 1.0)
        {
            return Err(
                "SOA critical threshold must be finite and at least 100% of the limit, or disabled"
                    .into(),
            );
        }
        Ok(())
    }

    /// Strict comparisons preserve the historical boundary behavior. Multiplying
    /// a finite limit by a large finite ratio may overflow to infinity: no
    /// finite stress can exceed that mathematical threshold, as intended.
    pub fn verdict(self, actual: f64, limit: f64) -> SoARuleVerdict {
        if self
            .critical_fraction
            .is_some_and(|fraction| actual > limit * fraction)
        {
            SoARuleVerdict::Critical
        } else if actual > limit {
            SoARuleVerdict::Violation
        } else if self
            .warning_fraction
            .is_some_and(|fraction| actual > limit * fraction)
        {
            SoARuleVerdict::Warning
        } else {
            SoARuleVerdict::Pass
        }
    }
}

#[cfg(test)]
#[test]
fn soa_thresholds_respect_boundaries_disabled_bands_and_zero_limits() {
    use SoARuleVerdict::*;
    let thresholds = SoaThresholds {
        warning_fraction: Some(0.5),
        critical_fraction: Some(2.0),
    };
    thresholds.validate().unwrap();
    for (actual, expected) in [
        (0.5, Pass),
        (0.75, Warning),
        (1.0, Warning),
        (1.5, Violation),
        (2.0, Violation),
        (2.5, Critical),
    ] {
        assert_eq!(thresholds.verdict(actual, 1.0), expected);
    }
    let disabled = SoaThresholds {
        warning_fraction: None,
        critical_fraction: None,
    };
    assert_eq!(disabled.verdict(0.99, 1.0), Pass);
    assert_eq!(disabled.verdict(1000.0, 1.0), Violation);
    assert_eq!(disabled.verdict(0.0, 0.0), Pass);
    assert_eq!(disabled.verdict(0.1, 0.0), Violation);
    assert_eq!(thresholds.verdict(0.1, 0.0), Critical);
    for value in [-0.1, 1.1, f64::NAN, f64::INFINITY] {
        assert!(
            SoaThresholds {
                warning_fraction: Some(value),
                ..thresholds
            }
            .validate()
            .is_err()
        );
    }
    for value in [0.9, f64::NAN, f64::INFINITY, f64::MAX] {
        assert!(
            SoaThresholds {
                critical_fraction: Some(value),
                ..thresholds
            }
            .validate()
            .is_err()
        );
    }
    assert_eq!(
        serde_json::from_str::<SoaThresholds>("{}").unwrap(),
        SoaThresholds::default()
    );
}
