//! Flip-flops and the two-phase counter.
//!
//! Inputs read as logic levels against `Ref`, as a gate's do, and the outputs
//! are a gate's output stage. DFLOP (D 1, CLK 3, PRE 4, CLR 5) takes D on each
//! rising clock edge; while PRE or CLR is high it holds Q high or low, CLR
//! first. An unused D reads high. SRFLOP (S 1, R 2) is a latch: R holds Q low,
//! else S holds it high. COUNTER counts rising edges of CLK (1) modulo
//! `cycles`, and holds the count at zero while RST (2) is high. Its `Phi1`
//! output (terminal 7) is high for the first `duty·cycles` counts of each
//! period and `Phi2` (terminal 6) for as many counts half a period later.
//! Terminals without a function are ignored.
//!
//! A flip-flop starts low, or high given `IC` above 0.5, and a counter at
//! zero, and neither changes state in the operating point: the first time
//! step applies the inputs. The counter switches its phases on the clock edge
//! whatever `Td` says, and takes no `IC`.

use super::logic::OutputStage;
use super::{InstanceParams, SpecialFunctionError, require};
use crate::Value;

/// Which sequential device, with the counter's period and phase width in
/// counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub enum SequentialFunction {
    DFlop,
    SrFlop,
    Counter { cycles: u32, high: u32 },
}

/// A DFLOP, SRFLOP or COUNTER.
#[derive(Debug, Clone, PartialEq)]
#[doc(hidden)]
pub struct Sequential {
    pub function: SequentialFunction,
    /// Input threshold above the common.
    pub reference: Value,
    pub stage: OutputStage,
}

impl Sequential {
    /// The device LTspice names `device` (`DFLOP`, `SRFLOP` or `COUNTER`).
    #[doc(hidden)]
    pub fn from_params(
        device: &'static str,
        params: &[(String, Value)],
    ) -> Result<Self, SpecialFunctionError> {
        let mut p = InstanceParams::new(device, params);
        let mut stage = OutputStage::read(&mut p)?;
        let reference = p.or("REF", 0.5 * (stage.vhigh + stage.vlow))?;
        let function = match device {
            "DFLOP" => SequentialFunction::DFlop,
            "SRFLOP" => SequentialFunction::SrFlop,
            "COUNTER" => {
                let cycles = p
                    .get("CYCLES")?
                    .ok_or_else(|| SpecialFunctionError::new("COUNTER needs a value of cycles"))?;
                require(cycles >= 0.0, || {
                    format!("COUNTER cycles must not be negative ({cycles})")
                })?;
                require(cycles < Value::from(u32::MAX), || {
                    format!("COUNTER cycles is too large ({cycles})")
                })?;
                // A count runs to the nearest whole number of cycles, and a
                // phase lasts the nearest whole number of counts to
                // `duty·cycles`, or half the period rounded down.
                let cycles = (cycles + 0.5).floor() as u32;
                let high = match p.get("DUTY")? {
                    Some(duty) => (duty * Value::from(cycles) + 0.5)
                        .floor()
                        .clamp(0.0, cycles.into()) as u32,
                    None => cycles / 2,
                };
                require(stage.initial.is_none(), || {
                    "COUNTER does not define parameter IC".to_string()
                })?;
                stage.delay = 0.0;
                SequentialFunction::Counter { cycles, high }
            }
            other => {
                return Err(SpecialFunctionError::new(format!(
                    "{other} is not a sequential special function"
                )));
            }
        };
        p.finish()?;
        Ok(Self {
            function,
            reference,
            stage,
        })
    }

    /// The state after the inputs on terminals 1 to 5 move from `before` to
    /// `now`, each a logic level or `None` for an unused terminal. A
    /// flip-flop's state is Q (0 or 1), a counter's its count.
    #[doc(hidden)]
    pub fn next(&self, state: u32, before: [Option<bool>; 5], now: [Option<bool>; 5]) -> u32 {
        let level = |levels: [Option<bool>; 5], pin: usize| {
            let unused_high = self.function == SequentialFunction::DFlop && pin == 1;
            levels[pin - 1].unwrap_or(unused_high)
        };
        let rising = |pin: usize| !level(before, pin) && level(now, pin);
        match self.function {
            SequentialFunction::DFlop => {
                if level(now, 5) {
                    0
                } else if level(now, 4) {
                    1
                } else if rising(3) {
                    u32::from(level(now, 1))
                } else {
                    state
                }
            }
            SequentialFunction::SrFlop => {
                if level(now, 2) {
                    0
                } else if level(now, 1) {
                    1
                } else {
                    state
                }
            }
            SequentialFunction::Counter { cycles, .. } => {
                if level(now, 2) || cycles == 0 {
                    0
                } else if rising(1) {
                    (state + 1) % cycles
                } else {
                    state
                }
            }
        }
    }

    /// The levels of `Q` (terminal 7) and `_Q` (terminal 6) in `state`; a
    /// counter's `Phi1` and `Phi2`.
    #[doc(hidden)]
    pub fn outputs(&self, state: u32) -> (bool, bool) {
        match self.function {
            SequentialFunction::DFlop | SequentialFunction::SrFlop => (state == 1, state != 1),
            SequentialFunction::Counter { cycles, high } => {
                let shift = cycles / 2;
                let phi2 = cycles > 0 && (state + cycles - shift) % cycles < high;
                (state < high, phi2)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn counter(cycles: Value, duty: Option<Value>) -> Sequential {
        let mut params = vec![("CYCLES".to_string(), cycles)];
        params.extend(duty.map(|duty| ("DUTY".to_string(), duty)));
        Sequential::from_params("COUNTER", &params).unwrap()
    }

    /// Each output's level over one period of counts, as `1`s and `0`s.
    fn phases(counter: &Sequential) -> (String, String) {
        let SequentialFunction::Counter { cycles, .. } = counter.function else {
            unreachable!()
        };
        let (mut phi1, mut phi2) = (String::new(), String::new());
        for count in 0..cycles.max(1) {
            let (a, b) = counter.outputs(count);
            phi1.push(if a { '1' } else { '0' });
            phi2.push(if b { '1' } else { '0' });
        }
        (phi1, phi2)
    }

    #[test]
    fn counter_phases_follow_ltspice() {
        // LTspice 26.1.1 batch, sampled after each rising clock edge.
        let cases = [
            (3.0, None, "100", "010"),
            (4.0, None, "1100", "0011"),
            (7.0, None, "1110000", "0001110"),
            (7.0, Some(0.5), "1111000", "0001111"),
            (4.0, Some(0.25), "1000", "0010"),
            (4.0, Some(0.75), "1110", "1011"),
            (5.0, Some(0.1), "10000", "00100"),
            (6.0, Some(0.25), "110000", "000110"),
            (8.0, Some(0.6), "11111000", "10001111"),
            (2.5, None, "100", "010"),
            (2.4, Some(0.5), "10", "01"),
            (1.0, None, "0", "0"),
            (1.0, Some(0.5), "1", "1"),
            (0.0, None, "0", "0"),
        ];
        for (cycles, duty, phi1, phi2) in cases {
            let phases = phases(&counter(cycles, duty));
            assert_eq!(
                phases,
                (phi1.into(), phi2.into()),
                "cycles={cycles} duty={duty:?}"
            );
        }
    }

    #[test]
    fn counter_counts_rising_edges_until_reset() {
        let counter = counter(4.0, None);
        let low = [Some(false), Some(false), None, None, None];
        let high = [Some(true), Some(false), None, None, None];
        assert_eq!(counter.next(3, low, high), 0);
        assert_eq!(counter.next(1, high, low), 1);
        let reset = [Some(true), Some(true), None, None, None];
        assert_eq!(counter.next(2, low, reset), 0);
    }

    #[test]
    fn flip_flops_give_clear_and_reset_priority() {
        let dflop = Sequential::from_params("DFLOP", &[]).unwrap();
        let levels = |d, clk, pre, clr| [Some(d), None, Some(clk), Some(pre), Some(clr)];
        // A rising clock takes D; a falling one does not.
        assert_eq!(
            dflop.next(
                0,
                levels(true, false, false, false),
                levels(true, true, false, false)
            ),
            1
        );
        assert_eq!(
            dflop.next(
                1,
                levels(false, true, false, false),
                levels(false, false, false, false)
            ),
            1
        );
        // CLR wins over PRE and the clock.
        assert_eq!(
            dflop.next(
                1,
                levels(true, false, true, false),
                levels(true, true, true, true)
            ),
            0
        );
        // An unused D reads high.
        let unused_d = |clk| [None, None, Some(clk), None, None];
        assert_eq!(dflop.next(0, unused_d(false), unused_d(true)), 1);

        let srflop = Sequential::from_params("SRFLOP", &[]).unwrap();
        let sr = |s, r| [Some(s), Some(r), None, None, None];
        assert_eq!(srflop.next(1, sr(false, false), sr(true, true)), 0);
        assert_eq!(srflop.next(0, sr(true, true), sr(true, false)), 1);
        assert_eq!(srflop.outputs(1), (true, false));
    }

    #[test]
    fn counter_requires_cycles() {
        assert!(Sequential::from_params("COUNTER", &[]).is_err());
        assert!(Sequential::from_params("COUNTER", &[("CYCLES".to_string(), -1.0)]).is_err());
    }
}
