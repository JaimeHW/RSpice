//! The phase detector, sample-and-hold, modulator and varistor.
//!
//! PHASEDET (A 1, B 2) is a phase-frequency detector: a rising edge on A
//! steps its state up and one on B steps it down, within −1 to 1, and
//! terminal 7 sources `Iout` times the state into its load. Its inputs read
//! high at or above `Ref`, 0.5 V whatever the output limits, and an input
//! already high in the operating point steps the state at the first time
//! step. `Rout` loads the output, which `Rclamp` holds within `Vlow` to
//! `Vhigh` when they are given.
//!
//! SAMPLEHOLD reads `V1 − V2`. With CLK (3) it takes the input on each rising
//! clock edge; with S/H (4) it follows the input while S/H is high and holds
//! it from the moment S/H falls. Exactly one of the two is connected. The
//! output, terminal 7, is the held value behind `Rout`, held within `Vlow`
//! to `Vhigh` through `Rout/1000` when they are given. It holds zero in the
//! operating point, tracking or not.
//!
//! MODULATOR is a quadrature oscillator at `space + (mark − space)·V(FM)`
//! hertz with amplitude `V(AM)`, either input reading 1 V when unused:
//! terminal 7 is `A·sin φ` and terminal 6 `A·cos φ`, each behind `Rout`.
//!
//! VARISTOR conducts from terminal 7 to the common only beyond the breakdown
//! voltage `|V1 − V2|`, through `Rclamp`. Given `table(x1 y1 x2 y2 ...)`,
//! the breakdown is that table read at `V2 − V1` instead, linear between
//! entries and held beyond the ends, as LTspice reads it.
//!
//! Each reads `Td` as LTspice does, without delaying anything.

use super::{InstanceParams, SpecialFunctionError, require};
use crate::Value;

/// The current a limit to `low`..`high` draws from a terminal at `v`
/// through `resistance`, and its conductance.
fn limit(v: Value, low: Option<Value>, high: Option<Value>, resistance: Value) -> (Value, Value) {
    match (low, high) {
        (_, Some(high)) if v > high => ((v - high) / resistance, 1.0 / resistance),
        (Some(low), _) if v < low => ((v - low) / resistance, 1.0 / resistance),
        _ => (0.0, 0.0),
    }
}

/// Optional output limits, which must be ordered when both are given.
fn limits(
    p: &mut InstanceParams<'_>,
) -> Result<(Option<Value>, Option<Value>), SpecialFunctionError> {
    let device = p.device;
    let high = p.get("VHIGH")?;
    let low = p.get("VLOW")?;
    if let (Some(high), Some(low)) = (high, low) {
        require(high > low, || {
            format!("{device} needs Vhigh above Vlow (Vhigh={high}, Vlow={low})")
        })?;
    }
    Ok((low, high))
}

fn positive(device: &str, name: &str, value: Value) -> Result<Value, SpecialFunctionError> {
    require(value > 0.0, || {
        format!("{device} {name} must be positive ({value})")
    })?;
    Ok(value)
}

/// A PHASEDET.
#[derive(Debug, Clone, PartialEq)]
#[doc(hidden)]
pub struct PhaseDetector {
    pub iout: Value,
    /// Input threshold above the common.
    pub reference: Value,
    pub vlow: Option<Value>,
    pub vhigh: Option<Value>,
    pub rclamp: Value,
    pub rout: Value,
}

impl PhaseDetector {
    #[doc(hidden)]
    pub fn from_params(params: &[(String, Value)]) -> Result<Self, SpecialFunctionError> {
        let mut p = InstanceParams::new("PHASEDET", params);
        let iout = p.or("IOUT", 100e-6)?;
        let reference = p.or("REF", 0.5)?;
        let (vlow, vhigh) = limits(&mut p)?;
        let rclamp = positive("PHASEDET", "Rclamp", p.or("RCLAMP", 10.0)?)?;
        let rout = positive("PHASEDET", "Rout", p.or("ROUT", 1e10)?)?;
        p.get("TD")?;
        p.finish()?;
        Ok(Self {
            iout,
            reference,
            vlow,
            vhigh,
            rclamp,
            rout,
        })
    }

    /// The state after A and B move from `before` to `now`: an A edge steps
    /// up before a B edge in the same step steps down.
    #[doc(hidden)]
    pub fn next(state: Value, before: [bool; 2], now: [bool; 2]) -> Value {
        let rising = |input: usize| !before[input] && now[input];
        let mut state = state;
        if rising(0) {
            state = (state + 1.0).min(1.0);
        }
        if rising(1) {
            state = (state - 1.0).max(-1.0);
        }
        state
    }

    /// The current from terminal 7 through the device to the common at
    /// output voltage `v` in `state`, and its conductance.
    #[doc(hidden)]
    pub fn current(&self, state: Value, v: Value) -> (Value, Value) {
        let (clamp, conductance) = limit(v, self.vlow, self.vhigh, self.rclamp);
        (
            v / self.rout + clamp - self.iout * state,
            1.0 / self.rout + conductance,
        )
    }
}

/// A SAMPLEHOLD.
#[derive(Debug, Clone, PartialEq)]
#[doc(hidden)]
pub struct SampleHold {
    /// CLK and S/H threshold above the common.
    pub reference: Value,
    pub rout: Value,
    pub vlow: Option<Value>,
    pub vhigh: Option<Value>,
}

impl SampleHold {
    #[doc(hidden)]
    pub fn from_params(params: &[(String, Value)]) -> Result<Self, SpecialFunctionError> {
        let mut p = InstanceParams::new("SAMPLEHOLD", params);
        let reference = p.or("REF", 0.5)?;
        let rout = positive("SAMPLEHOLD", "Rout", p.or("ROUT", 1e3)?)?;
        let (vlow, vhigh) = limits(&mut p)?;
        p.get("TD")?;
        p.finish()?;
        Ok(Self {
            reference,
            rout,
            vlow,
            vhigh,
        })
    }

    /// The current from terminal 7 through the device to the common at
    /// output voltage `v` holding `held`, and its conductance.
    #[doc(hidden)]
    pub fn current(&self, held: Value, v: Value) -> (Value, Value) {
        let (clamp, conductance) = limit(v, self.vlow, self.vhigh, self.rout / 1e3);
        (
            (v - held) / self.rout + clamp,
            1.0 / self.rout + conductance,
        )
    }
}

/// A MODULATOR.
#[derive(Debug, Clone, PartialEq)]
#[doc(hidden)]
pub struct Modulator {
    /// Frequency at `V(FM)` = 1 V.
    pub mark: Value,
    /// Frequency at `V(FM)` = 0 V.
    pub space: Value,
    pub rout: Value,
}

impl Modulator {
    #[doc(hidden)]
    pub fn from_params(params: &[(String, Value)]) -> Result<Self, SpecialFunctionError> {
        let mut p = InstanceParams::new("MODULATOR", params);
        let mark = p
            .get("MARK")?
            .ok_or_else(|| SpecialFunctionError::new("MODULATOR needs a mark frequency"))?;
        let space = p
            .get("SPACE")?
            .ok_or_else(|| SpecialFunctionError::new("MODULATOR needs a space frequency"))?;
        let rout = positive("MODULATOR", "Rout", p.or("ROUT", 1.0)?)?;
        p.get("TD")?;
        p.finish()?;
        Ok(Self { mark, space, rout })
    }
}

/// A VARISTOR.
#[derive(Debug, Clone, PartialEq)]
#[doc(hidden)]
pub struct Varistor {
    pub rclamp: Value,
    /// `(x, breakdown)` entries, `x` increasing, read at `V2 − V1`.
    pub table: Option<Vec<(Value, Value)>>,
}

impl Varistor {
    #[doc(hidden)]
    pub fn from_params(params: &[(String, Value)]) -> Result<Self, SpecialFunctionError> {
        let mut p = InstanceParams::new("VARISTOR", params);
        let rclamp = positive("VARISTOR", "Rclamp", p.or("RCLAMP", 1.0)?)?;
        let mut values = Vec::new();
        while let Some(value) = p.get(&crate::netlist::SpecialFunction::table_entry(
            values.len() + 1,
        ))? {
            values.push(value);
        }
        let table = if values.is_empty() {
            None
        } else {
            require(values.len() % 2 == 0, || {
                format!(
                    "VARISTOR table needs x and breakdown pairs, not {} values",
                    values.len()
                )
            })?;
            let entries: Vec<(Value, Value)> =
                values.chunks(2).map(|pair| (pair[0], pair[1])).collect();
            require(entries.windows(2).all(|pair| pair[0].0 < pair[1].0), || {
                "VARISTOR table x values must increase".to_string()
            })?;
            Some(entries)
        };
        p.get("TD")?;
        p.finish()?;
        Ok(Self { rclamp, table })
    }

    /// The breakdown voltage and its partial with respect to `control`.
    fn breakdown(&self, control: Value) -> (Value, Value) {
        let Some(table) = &self.table else {
            return (control.abs(), control.signum());
        };
        let x = -control;
        let upper = table.partition_point(|&(at, _)| at <= x);
        if upper == 0 {
            return (table[0].1, 0.0);
        }
        if upper == table.len() {
            return (table[upper - 1].1, 0.0);
        }
        let ((x0, y0), (x1, y1)) = (table[upper - 1], table[upper]);
        let slope = (y1 - y0) / (x1 - x0);
        (y0 + slope * (x - x0), -slope)
    }

    /// The current from terminal 7 through the device to the common at
    /// `v`, and its partials with respect to `v` and `control`, `V1 − V2`.
    /// Above the breakdown `b` the device conducts `(v − b)/Rclamp`, and
    /// below `−b` it conducts `(v + b)/Rclamp`, in that order, so a negative
    /// breakdown from a table conducts everywhere, as in LTspice.
    #[doc(hidden)]
    pub fn current(&self, v: Value, control: Value) -> (Value, Value, Value) {
        let (b, db) = self.breakdown(control);
        let g = 1.0 / self.rclamp;
        if v > b {
            ((v - b) * g, g, -g * db)
        } else if v < -b {
            ((v + b) * g, g, g * db)
        } else {
            (0.0, 0.0, 0.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params(pairs: &[(&str, Value)]) -> Vec<(String, Value)> {
        pairs
            .iter()
            .map(|(name, value)| (name.to_string(), *value))
            .collect()
    }

    #[test]
    fn phase_detector_saturates_and_cancels_coincident_edges() {
        let (low, high) = ([false, false], [true, false]);
        assert_eq!(PhaseDetector::next(1.0, low, high), 1.0);
        assert_eq!(PhaseDetector::next(-1.0, low, high), 0.0);
        // LTspice 26.1.1 batch: from +1, A and B together leave 0.
        assert_eq!(PhaseDetector::next(1.0, [false; 2], [true; 2]), 0.0);
        assert_eq!(PhaseDetector::next(0.0, [false; 2], [false, true]), -1.0);
    }

    #[test]
    fn limits_follow_ltspice() {
        // LTspice 26.1.1 batch: 100 uA into 1 Meg sits at 2.50097 V above
        // Vhigh = 2.5 through the 10 ohm default clamp.
        let pfd = PhaseDetector::from_params(&params(&[("VHIGH", 2.5), ("VLOW", -0.5)])).unwrap();
        let v: Value = 2.500_975;
        let (current, _) = pfd.current(1.0, v);
        assert!((current + v / 1e6).abs() < 1e-8, "{current}");
        // A held 1.3 V behind the 1k default sits 0.8 V / 1001 above
        // Vhigh = 0.5.
        let sh = SampleHold::from_params(&params(&[("VHIGH", 0.5), ("VLOW", -0.2)])).unwrap();
        let (current, _) = sh.current(1.3, 0.5 + 0.8 / 1001.0);
        assert!(current.abs() < 1e-12, "{current}");
    }

    #[test]
    fn varistor_conducts_beyond_its_breakdown_either_way() {
        let varistor = Varistor::from_params(&params(&[("RCLAMP", 10.0)])).unwrap();
        assert_eq!(varistor.current(1.5, -2.0), (0.0, 0.0, 0.0));
        let (current, g, d_control) = varistor.current(-2.5, 2.0);
        assert!((current + 0.05).abs() < 1e-15);
        assert_eq!((g, d_control), (0.1, 0.1));
    }

    #[test]
    fn a_varistor_table_sets_the_breakdown_from_v2_minus_v1() {
        let table = |values: &[Value]| {
            let pairs: Vec<(String, Value)> = values
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    (
                        crate::netlist::SpecialFunction::table_entry(index + 1),
                        *value,
                    )
                })
                .collect();
            Varistor::from_params(&pairs)
        };
        // LTspice 26.1.1: table(.5 0 10.5 10) with V2 - V1 = 1 breaks down
        // at 0.5, with V2 - V1 = -5 at 0 and with V2 - V1 = 20 at 10.
        let varistor = table(&[0.5, 0.0, 10.5, 10.0]).unwrap();
        assert_eq!(varistor.current(5.0, -1.0).0, 4.5);
        assert_eq!(varistor.current(5.0, 5.0).0, 5.0);
        assert_eq!(varistor.current(-20.0, -20.0).0, -10.0);
        // table(0 0 1 -1 2 -2 3 -3 4 -4) with V2 - V1 = 3: breakdown -3.
        let varistor = table(&[0.0, 0.0, 1.0, -1.0, 2.0, -2.0, 3.0, -3.0, 4.0, -4.0]).unwrap();
        assert_eq!(varistor.current(-2.0, -3.0).0, 1.0);
        assert_eq!(varistor.current(-5.0, -3.0).0, -8.0);
        assert!(table(&[0.5, 0.0, 10.5]).is_err());
        assert!(table(&[1.0, 0.0, 0.5, 1.0]).is_err());
    }

    #[test]
    fn modulator_needs_mark_and_space() {
        assert!(Modulator::from_params(&params(&[("MARK", 1e3)])).is_err());
        assert!(Modulator::from_params(&params(&[("MARK", 1e3), ("SPACE", 0.0)])).is_ok());
    }
}
