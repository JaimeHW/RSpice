//! The OTA: a four-quadrant multiplying transconductance amplifier.
//!
//! Terminals: 1 inverting input, 2 non-inverting input, 3 and 4 the
//! multiplying input, 7 the output and 8 the common. The output current is
//! `f(G·m·(V2 − V1 + ref))`, with `m = V3 − V4` (1 when both are unused) and
//! `f` a hyperbolic tangent bounded by `Iout` and `Isink`. The output voltage
//! is held between `Vlow` and `Vhigh` above the common by a clamp of
//! impedance `Rclamp`, switched in gradually over `epsilon`.

use super::{InstanceParams, SpecialFunctionError, require};
use crate::Value;

/// Input-referred noise densities, each white with a 1/f corner:
/// `density² = white² · (1 + corner / f)`.
#[derive(Debug, Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct OtaNoise {
    /// Input voltage noise (V/√Hz) and its corner (Hz).
    pub en: Value,
    pub enk: Value,
    /// Normal-mode input current noise between the inputs (A/√Hz).
    pub in_: Value,
    pub ink: Value,
    /// Common-mode input current noise (A/√Hz): one current injected into
    /// both inputs from the common.
    pub incm: Value,
    pub incmk: Value,
}

#[derive(Debug, Clone, PartialEq)]
#[doc(hidden)]
pub struct Ota {
    pub g: Value,
    pub iout: Value,
    pub isink: Value,
    pub asym: bool,
    pub linear: bool,
    pub offset: Value,
    pub vhigh: Value,
    pub vlow: Value,
    pub rclamp: Value,
    pub epsilon: Value,
    /// Noiseless resistance loading the output, if any.
    pub rout: Option<Value>,
    /// Capacitance loading the output.
    pub cout: Value,
    pub noise: OtaNoise,
}

/// The drive current out of the output terminal and its partial derivatives
/// with respect to the differential and multiplying inputs.
#[derive(Debug, Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct OtaDrive {
    pub current: Value,
    pub d_input: Value,
    pub d_multiplier: Value,
}

impl Ota {
    #[doc(hidden)]
    pub fn from_params(params: &[(String, Value)]) -> Result<Self, SpecialFunctionError> {
        let mut p = InstanceParams::new("OTA", params);
        let g = p.or("G", 1.0)?;
        let iout = p.or("IOUT", 10e-6)?;
        let isink = p.or("ISINK", -iout)?;
        let asym = p.flag("ASYM")?;
        let linear = p.flag("LINEAR")?;
        let offset = p.or("REF", 0.0)?;
        let vhigh = p.or("VHIGH", 2.0)?;
        let vlow = p.or("VLOW", 0.0)?;
        let rclamp = p.or("RCLAMP", 1.0)?;
        let epsilon = p.or("EPSILON", 0.0)?;
        let rout = p.get("ROUT")?;
        let cout = p.or("COUT", 0.0)?;
        let noise = OtaNoise {
            en: p.or("EN", 0.0)?,
            enk: p.or("ENK", 0.0)?,
            in_: p.or("IN", 0.0)?,
            ink: p.or("INK", 0.0)?,
            incm: p.or("INCM", 0.0)?,
            incmk: p.or("INCMK", 0.0)?,
        };
        p.finish()?;
        if !linear {
            require(iout > isink, || {
                format!("OTA needs Iout above Isink (Iout={iout}, Isink={isink})")
            })?;
            if asym {
                require(iout > 0.0 && isink < 0.0, || {
                    format!(
                        "OTA asym needs a positive Iout and a negative Isink \
                         (Iout={iout}, Isink={isink})"
                    )
                })?;
            }
        }
        require(vhigh > vlow, || {
            format!("OTA needs Vhigh above Vlow (Vhigh={vhigh}, Vlow={vlow})")
        })?;
        require(rclamp > 0.0, || {
            format!("OTA Rclamp must be positive ({rclamp})")
        })?;
        require(epsilon >= 0.0, || {
            format!("OTA epsilon must not be negative ({epsilon})")
        })?;
        if let Some(rout) = rout {
            require(rout > 0.0, || format!("OTA Rout must be positive ({rout})"))?;
        }
        require(cout >= 0.0, || {
            format!("OTA Cout must not be negative ({cout})")
        })?;
        for (name, value) in [
            ("en", noise.en),
            ("enk", noise.enk),
            ("in", noise.in_),
            ("ink", noise.ink),
            ("incm", noise.incm),
            ("incmk", noise.incmk),
        ] {
            require(value >= 0.0, || {
                format!("OTA noise parameter {name} must not be negative ({value})")
            })?;
        }
        Ok(Self {
            g,
            iout,
            isink,
            asym,
            linear,
            offset,
            vhigh,
            vlow,
            rclamp,
            epsilon,
            rout,
            cout,
            noise,
        })
    }

    /// The current driven out of the output at differential input
    /// `V2 − V1` and multiplier `m`.
    #[doc(hidden)]
    pub fn drive(&self, input: Value, multiplier: Value) -> OtaDrive {
        let effective = input + self.offset;
        let x = self.g * multiplier * effective;
        let (current, slope) = if self.linear {
            (x, 1.0)
        } else if self.asym {
            let limit = if x >= 0.0 { self.iout } else { -self.isink };
            bounded_tanh(x, limit)
        } else {
            let center = 0.5 * (self.iout + self.isink);
            let half = 0.5 * (self.iout - self.isink);
            let (current, slope) = bounded_tanh(x, half);
            (center + current, slope)
        };
        OtaDrive {
            current,
            d_input: slope * self.g * multiplier,
            d_multiplier: slope * self.g * effective,
        }
    }

    /// The current the compliance clamp sinks from the output at output
    /// voltage `v` above the common, and its conductance.
    #[doc(hidden)]
    pub fn clamp(&self, v: Value) -> (Value, Value) {
        let excess = |beyond: Value| -> (Value, Value) {
            if beyond <= 0.0 {
                (0.0, 0.0)
            } else if beyond >= self.epsilon {
                (
                    (beyond - 0.5 * self.epsilon) / self.rclamp,
                    1.0 / self.rclamp,
                )
            } else {
                let scale = self.epsilon * self.rclamp;
                (0.5 * beyond * beyond / scale, beyond / scale)
            }
        };
        let (above, g_above) = excess(v - self.vhigh);
        let (below, g_below) = excess(self.vlow - v);
        (above - below, g_above + g_below)
    }
}

/// `limit · tanh(x / limit)` and its slope.
fn bounded_tanh(x: Value, limit: Value) -> (Value, Value) {
    let t = (x / limit).tanh();
    (limit * t, 1.0 - t * t)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ota(params: &[(&str, Value)]) -> Ota {
        let params: Vec<(String, Value)> = params
            .iter()
            .map(|(name, value)| (name.to_string(), *value))
            .collect();
        Ota::from_params(&params).unwrap()
    }

    #[test]
    fn symmetric_tanh_saturates_at_iout() {
        let device = ota(&[("G", 1e-3), ("IOUT", 10e-6)]);
        // LTspice 26.1.1 batch: 10 mV into G=1m, Iout=10u gives 7.6159 uA.
        let drive = device.drive(10e-3, 1.0);
        assert!((drive.current - 7.615_941_6e-6).abs() < 1e-12);
        assert!((device.drive(1.0, 1.0).current - 10e-6).abs() < 1e-15);
        assert!((device.drive(0.0, 1.0).d_input - 1e-3).abs() < 1e-15);
    }

    #[test]
    fn isink_shifts_the_center_unless_asym() {
        let shifted = ota(&[("G", 1e-3), ("IOUT", 10e-6), ("ISINK", -2e-6)]);
        // LTspice 26.1.1 batch: 4 uA at zero input, 9.5867 uA at 10 mV.
        assert!((shifted.drive(0.0, 1.0).current - 4e-6).abs() < 1e-15);
        assert!((shifted.drive(10e-3, 1.0).current - 9.586_657_6e-6).abs() < 1e-12);
        let asym = ota(&[
            ("G", 1e-3),
            ("IOUT", 10e-6),
            ("ISINK", -2e-6),
            ("ASYM", 1.0),
        ]);
        assert_eq!(asym.drive(0.0, 1.0).current, 0.0);
        // LTspice 26.1.1 batch: -1.99982 uA at -10 mV.
        assert!((asym.drive(-10e-3, 1.0).current + 1.999_818_4e-6).abs() < 1e-12);
    }

    #[test]
    fn multiplier_scales_the_transconductance_not_the_limit() {
        let device = ota(&[("G", 1e-3), ("IOUT", 10e-6)]);
        let drive = device.drive(10e-3, 2.0);
        assert!((drive.current - 10e-6 * 2.0_f64.tanh()).abs() < 1e-15);
        let step = 1e-6;
        let numeric = (device.drive(10e-3, 2.0 + step).current
            - device.drive(10e-3, 2.0 - step).current)
            / (2.0 * step);
        assert!((drive.d_multiplier - numeric).abs() < 1e-12);
    }

    #[test]
    fn clamp_switches_in_quadratically_over_epsilon() {
        let device = ota(&[
            ("VHIGH", 2.0),
            ("VLOW", -1.0),
            ("RCLAMP", 1e3),
            ("EPSILON", 0.5),
        ]);
        // LTspice 26.1.1 batch: 15.625 uA at 2.125 V, 375 uA at 2.625 V.
        assert!((device.clamp(2.125).0 - 15.625e-6).abs() < 1e-15);
        assert!((device.clamp(2.625).0 - 375e-6).abs() < 1e-15);
        assert!((device.clamp(-1.5).0 + 250e-6).abs() < 1e-15);
        assert_eq!(device.clamp(0.0), (0.0, 0.0));
    }

    #[test]
    fn unknown_parameters_are_refused() {
        let params = vec![("GAIN".to_string(), 1.0)];
        let error = Ota::from_params(&params).unwrap_err();
        assert!(error.to_string().contains("GAIN"));
    }
}
