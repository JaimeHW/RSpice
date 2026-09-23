//! Authored current/voltage SOA boundaries at specified pulse durations.
//! Curves must already represent the stated case temperature and duty conditions.
//! The checked window is conservatively treated as one complete pulse; this is
//! not a thermal-impedance or repetitive-pulse recovery model.
use super::SoAParameter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SoaVoltageInterpolation {
    Linear,
    Logarithmic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SoaPulseInterpolation {
    LongerPulse,
    Logarithmic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaPulseCurve {
    pub duration_s: f64,
    pub currents_a: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaCurrentEnvelope {
    pub source: String,
    pub conditions: String,
    /// Shared increasing voltage magnitudes, starting at zero.
    pub voltages_v: Vec<f64>,
    pub dc_currents_a: Option<Vec<f64>>,
    pub pulses: Vec<SoaPulseCurve>,
    /// None selects the continuous boundary. A pulse must cover the full window.
    pub pulse_width_s: Option<f64>,
    pub voltage_interpolation: SoaVoltageInterpolation,
    pub pulse_interpolation: SoaPulseInterpolation,
}

impl SoaCurrentEnvelope {
    #[cfg(test)]
    pub(crate) fn test_fixture() -> Self {
        Self {
            source: "Synthetic SOA fixture".into(),
            conditions: "Synthetic fixed case temperature; single window".into(),
            voltages_v: vec![0.0, 1.0, 10.0],
            dc_currents_a: Some(vec![0.0, 0.01, 0.001]),
            pulses: vec![
                SoaPulseCurve {
                    duration_s: 1e-9,
                    currents_a: vec![0.0, 0.04, 0.004],
                },
                SoaPulseCurve {
                    duration_s: 100e-9,
                    currents_a: vec![0.0, 0.02, 0.002],
                },
            ],
            pulse_width_s: Some(10e-9),
            voltage_interpolation: SoaVoltageInterpolation::Logarithmic,
            pulse_interpolation: SoaPulseInterpolation::Logarithmic,
        }
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.source.trim().is_empty()
            || self.conditions.trim().is_empty()
            || self.source.len() > 4096
            || self.conditions.len() > 4096
        {
            return Err(
                "SOA curves require a source and operating conditions (up to 4096 bytes each)"
                    .into(),
            );
        }
        if !(2..=1024).contains(&self.voltages_v.len())
            || self.pulses.len() > 128
            || self.voltages_v[0] != 0.0
            || self.voltages_v.iter().any(|v| !v.is_finite() || *v < 0.0)
            || self.voltages_v.windows(2).any(|w| w[1] <= w[0])
        {
            return Err("SOA curves need 2–1024 increasing finite voltage magnitudes starting at zero and at most 128 pulse rows".into());
        }
        let values = |currents: &[f64]| {
            currents.len() == self.voltages_v.len()
                && currents.iter().all(|v| v.is_finite() && *v >= 0.0)
                && currents.iter().any(|v| *v > 0.0)
        };
        if self.dc_currents_a.as_ref().is_some_and(|v| !values(v))
            || self
                .pulses
                .iter()
                .any(|p| !p.duration_s.is_finite() || p.duration_s <= 0.0 || !values(&p.currents_a))
            || self.pulses.windows(2).any(|p| {
                p[1].duration_s <= p[0].duration_s
                    || p[1]
                        .currents_a
                        .iter()
                        .zip(&p[0].currents_a)
                        .any(|(long, short)| long > short)
            })
            || self
                .dc_currents_a
                .as_ref()
                .zip(self.pulses.last())
                .is_some_and(|(dc, pulse)| {
                    dc.iter()
                        .zip(&pulse.currents_a)
                        .any(|(dc, pulse)| dc > pulse)
                })
        {
            return Err("SOA current rows must match the voltage columns, be finite and nonnegative, and never increase with pulse duration; DC must be no greater than the longest pulse".into());
        }
        match self.pulse_width_s {
            Some(width) if !width.is_finite() || width <= 0.0 => Err("SOA pulse width must be finite and positive".into()),
            Some(_) if self.pulses.is_empty() => Err("SOA pulse selection needs at least one pulse curve".into()),
            Some(width) if width > self.pulses.last().unwrap().duration_s && self.dc_currents_a.is_none() => Err("SOA pulse width exceeds the longest supplied curve; supply a longer or DC curve".into()),
            None if self.dc_currents_a.is_none() => Err("SOA continuous operation requires a DC curve".into()),
            _ => Ok(()),
        }
    }

    pub fn validate_window(&self, start_s: f64, stop_s: f64) -> Result<(), String> {
        self.validate()?;
        if !start_s.is_finite() || !stop_s.is_finite() || start_s < 0.0 || stop_s <= start_s {
            return Err("SOA curve window must have finite increasing times".into());
        }
        if self
            .pulse_width_s
            .is_some_and(|width| width < stop_s - start_s)
        {
            return Err("SOA selected pulse width must cover the entire checked window; shorten the window or choose a longer/DC curve".into());
        }
        Ok(())
    }

    /// Called after validation. Beyond the final voltage, the allowed current is
    /// zero. No voltage or duration extrapolation creates an unprovided rating.
    pub fn limit(&self, voltage_v: f64) -> Result<f64, String> {
        if !voltage_v.is_finite() || voltage_v < 0.0 {
            return Err("SOA curve voltage must be a finite magnitude".into());
        }
        if voltage_v > *self.voltages_v.last().ok_or("Missing SOA voltage axis")? {
            return Ok(0.0);
        }
        let upper = self.voltages_v.partition_point(|v| *v < voltage_v);
        // Use a common mode on each voltage interval so longer pulse/DC rows
        // remain nested even when only one of them reaches zero.
        let positive_ends =
            |values: &[f64]| upper > 0 && values[upper - 1] > 0.0 && values[upper] > 0.0;
        let logarithmic = self.voltage_interpolation == SoaVoltageInterpolation::Logarithmic
            && self.dc_currents_a.as_ref().is_none_or(|v| positive_ends(v))
            && self.pulses.iter().all(|p| positive_ends(&p.currents_a));
        let row = |currents: &[f64]| -> f64 {
            if upper == 0 || self.voltages_v[upper] == voltage_v {
                currents[upper]
            } else {
                interpolate(
                    self.voltages_v[upper - 1],
                    self.voltages_v[upper],
                    voltage_v,
                    currents[upper - 1],
                    currents[upper],
                    logarithmic,
                )
            }
        };
        let value = match self.pulse_width_s {
            None => row(self.dc_currents_a.as_ref().ok_or("Missing SOA DC curve")?),
            Some(width) => {
                let upper = self.pulses.partition_point(|p| p.duration_s < width);
                if upper == self.pulses.len() {
                    row(self
                        .dc_currents_a
                        .as_ref()
                        .ok_or("Missing longer SOA curve")?)
                } else if upper == 0
                    || self.pulses[upper].duration_s == width
                    || self.pulse_interpolation == SoaPulseInterpolation::LongerPulse
                {
                    row(&self.pulses[upper].currents_a)
                } else {
                    let a = &self.pulses[upper - 1];
                    let b = &self.pulses[upper];
                    interpolate(
                        a.duration_s,
                        b.duration_s,
                        width,
                        row(&a.currents_a),
                        row(&b.currents_a),
                        true,
                    )
                }
            }
        };
        if !value.is_finite() || value < 0.0 {
            return Err("SOA interpolated current is not representable".into());
        }
        Ok(value)
    }

    pub fn voltage_parameter(parameter: SoAParameter) -> Option<SoAParameter> {
        match parameter.base_parameter() {
            SoAParameter::Id => Some(SoAParameter::Vds),
            SoAParameter::Ic => Some(SoAParameter::Vce),
            SoAParameter::Ia => Some(SoAParameter::Vak),
            _ => None,
        }
    }
}

fn interpolate(a: f64, b: f64, x: f64, ya: f64, yb: f64, logarithmic: bool) -> f64 {
    // Segments touching zero have a linear continuation, even on log plots.
    let log = logarithmic && a > 0.0 && ya > 0.0 && yb > 0.0;
    let fraction = if log {
        let ratio_log = |high: f64, low: f64| {
            let relative = (high - low) / low;
            if relative.is_finite() {
                relative.ln_1p()
            } else {
                high.ln() - low.ln()
            }
        };
        ratio_log(x, a) / ratio_log(b, a)
    } else {
        (x - a) / (b - a)
    };
    let value = if log {
        ((1.0 - fraction) * ya.ln() + fraction * yb.ln()).exp()
    } else {
        (1.0 - fraction) * ya + fraction * yb
    };
    value.clamp(ya.min(yb), ya.max(yb))
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SoaCurrentEnvelopeEvidence {
    pub maximum_current_a: f64,
    pub curve: SoaCurrentEnvelope,
}

#[derive(Debug, Clone, Default)]
pub struct SoaEnvelopeSamples {
    pub voltages_v: Vec<f64>,
    pub limits_a: Vec<f64>,
}

pub fn soa_envelope_voltage_waveform_name(device: &str, parameter: SoAParameter) -> String {
    format!("SOA_{}_CURVE_VOLTAGE({device})", parameter.stress_code())
}
pub fn soa_envelope_limit_waveform_name(device: &str, parameter: SoAParameter) -> String {
    format!("SOA_{}_CURVE_LIMIT({device})", parameter.stress_code())
}

#[test]
fn soa_current_envelope_interpolates_physical_boundaries_and_checks_domains() {
    let mut curve = SoaCurrentEnvelope::test_fixture();
    curve.validate_window(0.0, 1e-9).unwrap();
    let close = |a: f64, b: f64| assert!((a - b).abs() < 1e-13 * b.abs().max(1e-300), "{a} != {b}");
    let peak = (0.04_f64 * 0.02).sqrt();
    for v in [0.0, 0.25, 1.0, 2.0, 7.0, 10.0, 11.0] {
        close(
            curve.limit(v).unwrap(),
            if v > 10.0 {
                0.0
            } else if v <= 1.0 {
                peak * v
            } else {
                peak / v
            },
        );
    }
    assert!(curve.validate_window(0.0, 20e-9).is_err());
    assert!(curve.limit(f64::NAN).is_err());
    curve.pulse_interpolation = SoaPulseInterpolation::LongerPulse;
    close(curve.limit(2.0).unwrap(), 0.01);
    curve.pulse_width_s = Some(0.1e-9);
    close(curve.limit(2.0).unwrap(), 0.02);
    curve.pulse_width_s = Some(1e-6);
    close(curve.limit(2.0).unwrap(), 0.005);
    curve.pulse_width_s = None;
    close(curve.limit(2.0).unwrap(), 0.005);
    curve.dc_currents_a = None;
    assert!(curve.validate().is_err());
    curve = SoaCurrentEnvelope::test_fixture();
    curve.pulses[1].currents_a[1] = 0.05;
    assert!(curve.validate().is_err());
    curve = SoaCurrentEnvelope::test_fixture();
    curve.voltages_v = vec![0.0, 1e-300, 1e300];
    curve.dc_currents_a = Some(vec![0.0, 1e300, 1e-300]);
    curve.pulses.clear();
    curve.pulse_width_s = None;
    curve.validate().unwrap();
    close(curve.limit(1.0).unwrap(), 1.0);
    curve = SoaCurrentEnvelope::test_fixture();
    curve.voltages_v = vec![0.0, 1.0, 100.0];
    curve.dc_currents_a = None;
    curve.pulses[0].currents_a = vec![0.0, 100.0, 1.0];
    curve.pulses[1].currents_a = vec![0.0, 50.0, 0.0];
    curve.validate().unwrap();
    curve.pulse_width_s = Some(100e-9);
    close(curve.limit(10.0).unwrap(), 50.0 * (1.0 - 9.0 / 99.0));
    let long = curve.limit(10.0).unwrap();
    curve.pulse_width_s = Some(1e-9);
    assert!(long <= curve.limit(10.0).unwrap());
}
