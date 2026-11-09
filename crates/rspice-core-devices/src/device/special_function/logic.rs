//! Behavioral gates and Schmitt triggers, and the output stage they share.
//!
//! Terminals 1 to 5 are inputs, 6 the complementary output `_Q`, 7 the
//! output `Q` and 8 the common. A gate reads each used input as true at or
//! above `Ref`; BUF and OR are true when any used input is, AND when every
//! one is, and XOR when exactly one is. A Schmitt trigger reads `V1 − V2`
//! (terminal 2 defaults to the common) and switches high above `Vt + Vh`
//! and low below `Vt − Vh`.
//!
//! Each output is a source at `Vlow` or `Vhigh` above the common behind
//! `Rlow` or `Rhigh`, loaded by `Cout`. A change of the logic function
//! reaches the outputs `Td` later provided it still holds then, so input
//! pulses shorter than `Td` are ignored. With `Trise`/`Tfall` the source
//! slews: `dx/dt = clamp((L − x)/τ, −Sfall, Srise)` with
//! `S = (Vhigh − Vlow)/T` and `τ = 0.1·Trise·Tfall/(Trise + Tfall)`, a
//! linear ramp that settles exponentially over its last `S·τ`.
//!
//! `Vhigh` may sit below `Vlow`, as LTspice allows: a true output then drives
//! the lower level. Such an output switches without slewing.
//!
//! `IC` above 0.5 starts `Q` high and below it low. A gate or Schmitt
//! trigger holds that level through the operating point only: from the first
//! time step its function decides, and a change reaches the outputs `Td`
//! after time zero.

use super::{InstanceParams, SpecialFunctionError, require};
use crate::Value;

/// The logic function of a gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub enum GateFunction {
    /// BUF and OR: any used input true.
    Or,
    And,
    /// True when exactly one used input is true.
    Xor,
}

impl GateFunction {
    #[doc(hidden)]
    pub fn evaluate(self, inputs: impl IntoIterator<Item = bool>) -> bool {
        let mut any = false;
        let mut all = true;
        let mut count = 0usize;
        for input in inputs {
            any |= input;
            all &= input;
            count += usize::from(input);
        }
        match self {
            Self::Or => any,
            Self::And => all,
            Self::Xor => count == 1,
        }
    }
}

/// The source, impedance and timing of a logic output.
#[derive(Debug, Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct OutputStage {
    pub vhigh: Value,
    pub vlow: Value,
    pub rhigh: Value,
    pub rlow: Value,
    pub cout: Value,
    /// Propagation delay and input hold time.
    pub delay: Value,
    pub trise: Value,
    pub tfall: Value,
    /// Largest step the simulator takes after a state change.
    pub tripdt: Option<Value>,
    /// Whether `Q` starts high, from `IC`.
    pub initial: Option<bool>,
    /// The current driven into the output while high, in place of `Vhigh`.
    pub isrc: Option<Value>,
    /// The current driven into the output while low, in place of `Vlow`.
    pub isink: Option<Value>,
}

impl OutputStage {
    pub(super) fn read(p: &mut InstanceParams<'_>) -> Result<Self, SpecialFunctionError> {
        let device = p.device;
        let vhigh = p.or("VHIGH", 1.0)?;
        let vlow = p.or("VLOW", 0.0)?;
        let rout = p.or("ROUT", 1.0)?;
        let rhigh = p.or("RHIGH", rout)?;
        let rlow = p.or("RLOW", rout)?;
        let tau = p.get("TAU")?;
        let cout = p.get("COUT")?;
        let delay = p.or("TD", 0.0)?;
        let trise = p.or("TRISE", 0.0)?;
        let tfall = p.or("TFALL", trise)?;
        let tripdt = p.get("TRIPDT")?;
        let initial = p.get("IC")?.map(|ic| ic > 0.5);
        let isrc = p.get("ISRC")?;
        let isink = p.get("ISINK")?;
        require(
            (isrc.is_none() && isink.is_none()) || (trise == 0.0 && tfall == 0.0),
            || format!("{device} takes a current Isrc or Isink or a slewing Trise/Tfall, not both"),
        )?;
        require(vhigh != vlow, || {
            format!("{device} needs Vhigh apart from Vlow (Vhigh={vhigh}, Vlow={vlow})")
        })?;
        require(vhigh > vlow || (trise == 0.0 && tfall == 0.0), || {
            format!(
                "{device} slews with Trise or Tfall only with Vhigh above Vlow \
                 (Vhigh={vhigh}, Vlow={vlow})"
            )
        })?;
        for (name, value) in [("Rout", rout), ("Rhigh", rhigh), ("Rlow", rlow)] {
            require(value > 0.0, || {
                format!("{device} {name} must be positive ({value})")
            })?;
        }
        for (name, value) in [("Td", delay), ("Trise", trise), ("Tfall", tfall)] {
            require(value >= 0.0, || {
                format!("{device} {name} must not be negative ({value})")
            })?;
        }
        if let Some(tripdt) = tripdt {
            require(tripdt > 0.0, || {
                format!("{device} tripdt must be positive ({tripdt})")
            })?;
        }
        let cout = match (tau, cout) {
            (Some(_), Some(_)) => {
                return Err(SpecialFunctionError::new(format!(
                    "{device} takes an output RC time constant Tau or a capacitance Cout, not both"
                )));
            }
            (Some(tau), None) => {
                require(tau >= 0.0, || {
                    format!("{device} Tau must not be negative ({tau})")
                })?;
                require(trise == 0.0 && tfall == 0.0, || {
                    format!("{device} takes a slewing Trise/Tfall or a time constant Tau, not both")
                })?;
                tau / rout
            }
            (None, cout) => cout.unwrap_or(0.0),
        };
        require(cout >= 0.0, || {
            format!("{device} Cout must not be negative ({cout})")
        })?;
        Ok(Self {
            vhigh,
            vlow,
            rhigh,
            rlow,
            cout,
            delay,
            trise,
            tfall,
            tripdt,
            initial,
            isrc,
            isink,
        })
    }

    /// The current an output `voltage` above the common draws from its node
    /// while driven `high` from `source`, and that current's slope.
    ///
    /// `Isrc` and `Isink` drive the state's current into the node beside its
    /// resistance to the common, and either one clamps the output through
    /// 1 ohm once it is past the state's own level (LTspice 26.1.1).
    #[doc(hidden)]
    pub fn output_current(&self, high: bool, source: Value, voltage: Value) -> (Value, Value) {
        let conductance = 1.0 / self.resistance(high);
        let (drive, level, past) = if high {
            (self.isrc, self.vhigh, voltage > self.vhigh)
        } else {
            (self.isink, self.vlow, voltage < self.vlow)
        };
        let current = match drive {
            Some(drive) => voltage * conductance - drive,
            None => (voltage - source) * conductance,
        };
        if past && (self.isrc.is_some() || self.isink.is_some()) {
            (current + voltage - level, conductance + 1.0)
        } else {
            (current, conductance)
        }
    }

    #[doc(hidden)]
    pub fn level(&self, high: bool) -> Value {
        if high { self.vhigh } else { self.vlow }
    }

    #[doc(hidden)]
    pub fn resistance(&self, high: bool) -> Value {
        if high { self.rhigh } else { self.rlow }
    }

    fn slope(&self, rising: bool) -> Value {
        let time = if rising { self.trise } else { self.tfall };
        if time > 0.0 {
            (self.vhigh - self.vlow) / time
        } else {
            Value::INFINITY
        }
    }

    fn settling(&self) -> Value {
        if self.trise > 0.0 && self.tfall > 0.0 {
            0.1 * self.trise * self.tfall / (self.trise + self.tfall)
        } else {
            0.0
        }
    }

    /// The source position `dt` after `x`, moving toward the `high` level.
    #[doc(hidden)]
    pub fn slew(&self, x: Value, high: bool, dt: Value) -> Value {
        let target = self.level(high);
        let distance = (target - x).abs();
        if distance == 0.0 || dt <= 0.0 {
            return if dt <= 0.0 { x } else { target };
        }
        let slope = self.slope(target > x);
        if slope.is_infinite() {
            return target;
        }
        let tau = self.settling();
        let tail = slope * tau;
        let ramp = ((distance - tail) / slope).max(0.0);
        let remaining = if dt <= ramp {
            distance - slope * dt
        } else if tau > 0.0 {
            distance.min(tail) * (-(dt - ramp) / tau).exp()
        } else {
            0.0
        };
        target - remaining.copysign(target - x)
    }

    /// How long after `x` a move toward `high` stays on its linear ramp, when
    /// it has one; the stepper lands on its end.
    #[doc(hidden)]
    pub fn ramp_duration(&self, x: Value, high: bool) -> Option<Value> {
        let target = self.level(high);
        let distance = (target - x).abs();
        let slope = self.slope(target > x);
        if distance == 0.0 || slope.is_infinite() {
            return None;
        }
        Some(((distance - slope * self.settling()) / slope).max(0.0))
    }
}

/// A gate: BUF, AND, OR or XOR.
#[derive(Debug, Clone, PartialEq)]
#[doc(hidden)]
pub struct Gate {
    pub function: GateFunction,
    /// Input threshold above the common.
    pub reference: Value,
    pub stage: OutputStage,
}

impl Gate {
    #[doc(hidden)]
    pub fn from_params(
        device: &'static str,
        function: GateFunction,
        params: &[(String, Value)],
    ) -> Result<Self, SpecialFunctionError> {
        let mut p = InstanceParams::new(device, params);
        let stage = OutputStage::read(&mut p)?;
        let reference = p.or("REF", 0.5 * (stage.vhigh + stage.vlow))?;
        p.schmitt_trip_points()?;
        p.finish()?;
        Ok(Self {
            function,
            reference,
            stage,
        })
    }
}

/// A Schmitt trigger, single-ended or differential.
#[derive(Debug, Clone, PartialEq)]
#[doc(hidden)]
pub struct Schmitt {
    pub threshold: Value,
    pub hysteresis: Value,
    pub stage: OutputStage,
}

impl Schmitt {
    #[doc(hidden)]
    pub fn from_params(params: &[(String, Value)]) -> Result<Self, SpecialFunctionError> {
        let mut p = InstanceParams::new("SCHMITT", params);
        let stage = OutputStage::read(&mut p)?;
        // `Ref` is the threshold too, as on a gate; `Vt` wins over it.
        let reference = p.get("REF")?;
        let threshold = p
            .get("VT")?
            .or(reference)
            .unwrap_or(0.5 * (stage.vhigh + stage.vlow));
        let hysteresis = p.or("VH", 0.05 * threshold.abs())?;
        p.finish()?;
        require(hysteresis >= 0.0, || {
            format!("SCHMITT Vh must not be negative ({hysteresis})")
        })?;
        Ok(Self {
            threshold,
            hysteresis,
            stage,
        })
    }

    /// The trigger state after reading `input` from state `high`.
    #[doc(hidden)]
    pub fn next(&self, high: bool, input: Value) -> bool {
        if high {
            input >= self.threshold - self.hysteresis
        } else {
            input > self.threshold + self.hysteresis
        }
    }

    /// The level the input crosses to leave state `high`.
    #[doc(hidden)]
    pub fn trip(&self, high: bool) -> Value {
        if high {
            self.threshold - self.hysteresis
        } else {
            self.threshold + self.hysteresis
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stage(params: &[(&str, Value)]) -> OutputStage {
        let params: Vec<(String, Value)> = params
            .iter()
            .map(|(name, value)| (name.to_string(), *value))
            .collect();
        Gate::from_params("BUF", GateFunction::Or, &params)
            .unwrap()
            .stage
    }

    #[test]
    fn slew_is_a_ramp_with_an_exponential_tail() {
        let stage = stage(&[("TRISE", 20e-9)]);
        // LTspice 26.1.1 batch, Trise=20n: linear to 95 % at 19 ns, then
        // tau = 1 ns.
        assert!((stage.slew(0.0, true, 10e-9) - 0.5).abs() < 1e-12);
        assert!((stage.slew(0.0, true, 19e-9) - 0.95).abs() < 1e-12);
        let tail = stage.slew(0.0, true, 21e-9);
        assert!((tail - (1.0 - 0.05 * (-2.0_f64).exp())).abs() < 1e-12);
        let ramp = stage.ramp_duration(0.0, true).unwrap();
        assert!((ramp - 19e-9).abs() < 1e-21);
        // Two partial steps compose to the single step.
        let split = stage.slew(stage.slew(0.0, true, 18.5e-9), true, 2.5e-9);
        assert!((split - tail).abs() < 1e-12);
    }

    #[test]
    fn unequal_edges_share_one_settling_constant() {
        let stage = stage(&[("TRISE", 10e-9), ("TFALL", 4e-9)]);
        // LTspice 26.1.1 batch: tau = 0.1·10n·4n/14n = 0.2857 ns on both edges.
        let tau: Value = 0.1 * 10e-9 * 4e-9 / 14e-9;
        let fall = stage.slew(1.0, false, 4e-9);
        let ramp = (1.0 - 0.25e9 * tau) / 0.25e9;
        let expected = 0.25e9 * tau * (-(4e-9 - ramp) / tau).exp();
        assert!((fall - expected).abs() < 1e-12);
    }

    #[test]
    fn tau_is_an_output_capacitance_behind_rout() {
        assert!((stage(&[("TAU", 10e-9), ("ROUT", 1e3)]).cout - 10e-12).abs() < 1e-24);
    }

    #[test]
    fn schmitt_defaults_follow_ltspice() {
        // LTspice 26.1.1 batch: Vhigh=3 trips at 1.575 V and 1.425 V.
        let params = vec![("VHIGH".to_string(), 3.0)];
        let schmitt = Schmitt::from_params(&params).unwrap();
        assert!((schmitt.trip(false) - 1.575).abs() < 1e-12);
        assert!((schmitt.trip(true) - 1.425).abs() < 1e-12);
    }

    #[test]
    fn xor_is_true_for_exactly_one_input() {
        assert!(GateFunction::Xor.evaluate([true, false, false]));
        assert!(!GateFunction::Xor.evaluate([true, true, true]));
        assert!(GateFunction::Or.evaluate([false, true]));
        assert!(!GateFunction::And.evaluate([false, true]));
    }
}
