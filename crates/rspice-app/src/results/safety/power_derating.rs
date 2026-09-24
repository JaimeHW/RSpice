//! Linear power derating above a reference device temperature.
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaPowerDerating {
    pub reference_temperature_kelvin: f64,
    pub watts_per_kelvin: f64,
}

impl SoaPowerDerating {
    pub fn validate(self) -> Result<(), String> {
        if !self.reference_temperature_kelvin.is_finite()
            || self.reference_temperature_kelvin <= 0.0
        {
            return Err("SOA derating reference temperature must be above absolute zero".into());
        }
        if !self.watts_per_kelvin.is_finite() || self.watts_per_kelvin <= 0.0 {
            return Err("SOA derating slope must be finite and positive".into());
        }
        Ok(())
    }

    /// Full rated power below the reference; no negative allowed power.
    pub fn limit(self, rated_power_w: f64, temperature_kelvin: f64) -> f64 {
        (rated_power_w
            - (temperature_kelvin - self.reference_temperature_kelvin).max(0.0)
                * self.watts_per_kelvin)
            .max(0.0)
    }
}

/// The configured curve; actual temperatures and limits are retained as ordinary
/// waveform payloads so worker transports can stream their numeric data.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaPowerDeratingEvidence {
    pub rated_power_w: f64,
    pub curve: SoaPowerDerating,
}
impl SoaPowerDeratingEvidence {
    pub fn validate(self) -> Result<(), String> {
        self.curve.validate()?;
        if !self.rated_power_w.is_finite() || self.rated_power_w <= 0.0 {
            return Err("SOA derating rated power must be finite and positive".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct SoaDeratingSamples {
    pub temperatures_kelvin: Vec<f64>,
    pub limits_w: Vec<f64>,
}

pub fn soa_power_limit_waveform_name(device: &str) -> String {
    format!("SOA_PDISS_LIMIT({device})")
}
pub fn soa_derating_temperature_waveform_name(device: &str) -> String {
    format!("SOA_PDISS_TEMPERATURE({device})")
}

/// Highest utilization is the worst point. A positive stress at zero limit is
/// worse than any finite utilization; equal utilizations prefer larger stress.
/// Normalize mantissas and compare exponents so extreme finite ratios neither
/// overflow nor underflow. Inputs have already been validated as nonnegative.
pub fn compare_soa_stress(
    actual: f64,
    limit: f64,
    other_actual: f64,
    other_limit: f64,
) -> Ordering {
    fn components(value: f64) -> (f64, i32) {
        let bits = value.to_bits();
        let exponent = ((bits >> 52) & 0x7ff) as i32;
        if exponent == 0 {
            let (mantissa, exponent) = components(value * 4_503_599_627_370_496.0);
            return (mantissa, exponent - 52);
        }
        (
            f64::from_bits((bits & 0x000f_ffff_ffff_ffff) | (1023u64 << 52)),
            exponent - 1023,
        )
    }
    let ratio = |a: f64, l: f64| {
        if a == 0.0 {
            return (0, 0, 0.0);
        }
        if l == 0.0 {
            return (2, 0, 0.0);
        }
        let (am, ae) = components(a);
        let (lm, le) = components(l);
        let mut mantissa = am / lm;
        let mut exponent = ae - le;
        if mantissa < 1.0 {
            mantissa *= 2.0;
            exponent -= 1;
        }
        (1, exponent, mantissa)
    };
    let left = ratio(actual, limit);
    let right = ratio(other_actual, other_limit);
    left.0
        .cmp(&right.0)
        .then_with(|| left.1.cmp(&right.1))
        .then_with(|| left.2.total_cmp(&right.2))
        .then_with(|| actual.total_cmp(&other_actual))
}
