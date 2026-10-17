//! Analog input thresholds that require a physical mixed-signal timepoint.

use super::DigitalState;
use crate::Value;

/// The decision law associated with an analog input's two thresholds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalogThresholdBehavior {
    /// Retain the previous decision between the thresholds.
    Hysteresis,
    /// Publish X between the thresholds, as the XSPICE ADC does.
    UnknownBand,
}

/// A model's effective, elaboration-time analog event threshold.
/// Values use the connected input quantity's units, including current inputs.
#[derive(Debug, Clone)]
pub struct AnalogInputThreshold {
    /// Input port name.
    pub port: String,
    /// Vector element, or zero for a scalar port.
    pub element: usize,
    /// Lower decision threshold.
    pub low: Value,
    /// Upper decision threshold.
    pub high: Value,
    /// The model's decision law.
    pub behavior: AnalogThresholdBehavior,
}

/// The A/D decision law of every built-in converter: hysteresis between the
/// thresholds, and X once the input has stayed between them for `timeout`.
///
/// The two [`AnalogThresholdBehavior`]s are its limits. A zero timeout is the
/// XSPICE unknown band, which publishes X on entry; an infinite one is pure
/// hysteresis, which never does.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct AnalogBandLaw {
    /// How long an input may stay strictly between the thresholds before the
    /// output reads X, in seconds. Zero or positive; may be infinite.
    pub(crate) timeout: Value,
}

impl AnalogBandLaw {
    /// The XSPICE `adc_bridge` law.
    pub(crate) const UNKNOWN_BAND: Self = Self { timeout: 0.0 };
    /// The HDL host's built-in converter law.
    #[cfg(any(test, feature = "veriloga"))]
    pub(crate) const HYSTERESIS: Self = Self {
        timeout: Value::INFINITY,
    };

    /// The decision for `input` at `time`, and the first threshold traversed
    /// to reach it; `None` holds the previous decision.
    ///
    /// `expired` says the input has been in the band since a time `timeout`
    /// or more ago. An input still in the band then
    /// reads X, with the lower threshold reported as the one traversed.
    pub(crate) fn decision(
        self,
        input: Value,
        previous: Option<Value>,
        held: Option<DigitalState>,
        low: Value,
        high: Value,
        expired: bool,
    ) -> Option<(DigitalState, Value)> {
        if self.timeout == 0.0 {
            return AnalogThresholdBehavior::UnknownBand.decision(input, previous, held, low, high);
        }
        let decision =
            AnalogThresholdBehavior::Hysteresis.decision(input, previous, held, low, high);
        if decision.is_some() || !self.timeout.is_finite() {
            return decision;
        }
        expired.then_some((DigitalState::Unknown, low))
    }

    /// When an input that entered the band at `entry` reads X: the instant
    /// a converter asks the stepper to land on, and the one `expired` is
    /// measured against.
    pub(crate) fn deadline(self, entry: Value) -> Value {
        entry + self.timeout
    }
}

impl AnalogThresholdBehavior {
    /// Return the decision and first threshold traversed to reach it. Detecting
    /// the first boundary prevents a coarse step from skipping an X interval.
    pub(crate) fn decision(
        self,
        input: Value,
        previous: Option<Value>,
        held: Option<DigitalState>,
        low: Value,
        high: Value,
    ) -> Option<(DigitalState, Value)> {
        match self {
            Self::Hysteresis => {
                if low == high && input == low {
                    let high = if previous.is_some_and(|previous| previous < input) {
                        true
                    } else if previous.is_some_and(|previous| previous > input) {
                        false
                    } else {
                        held == Some(DigitalState::One)
                    };
                    Some((
                        if high {
                            DigitalState::One
                        } else {
                            DigitalState::Zero
                        },
                        low,
                    ))
                } else if input <= low {
                    Some((DigitalState::Zero, low))
                } else if input >= high {
                    Some((DigitalState::One, high))
                } else {
                    None
                }
            }
            Self::UnknownBand => {
                // The low-first model law also collapses reversed thresholds
                // to a single boundary at low.
                let high = high.max(low);
                let state = if input <= low {
                    DigitalState::Zero
                } else if input >= high {
                    DigitalState::One
                } else {
                    DigitalState::Unknown
                };
                let threshold = match held {
                    Some(DigitalState::Zero) => low,
                    Some(DigitalState::One) => high,
                    _ if state == DigitalState::One => high,
                    _ => low,
                };
                Some((state, threshold))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An input, the previous input, the held decision, and the thresholds.
    type Case = (Value, Option<Value>, Option<DigitalState>, Value, Value);

    /// Inputs below, on, inside, on and above a band, with each history.
    fn cases() -> Vec<Case> {
        let mut cases = Vec::new();
        for (low, high) in [(1.0, 2.0), (1.5, 1.5), (2.0, 1.0)] {
            for input in [0.5, 1.0, 1.25, 1.5, 1.75, 2.0, 2.5] {
                for previous in [None, Some(0.0), Some(1.5), Some(3.0)] {
                    for held in [None, Some(DigitalState::Zero), Some(DigitalState::One)] {
                        cases.push((input, previous, held, low, high));
                    }
                }
            }
        }
        cases
    }

    #[test]
    fn the_band_law_is_the_unknown_band_at_a_zero_timeout_and_hysteresis_at_an_infinite_one() {
        for (input, previous, held, low, high) in cases() {
            for expired in [false, true] {
                assert_eq!(
                    AnalogBandLaw::UNKNOWN_BAND.decision(input, previous, held, low, high, expired),
                    AnalogThresholdBehavior::UnknownBand.decision(input, previous, held, low, high)
                );
                assert_eq!(
                    AnalogBandLaw::HYSTERESIS.decision(input, previous, held, low, high, expired),
                    AnalogThresholdBehavior::Hysteresis.decision(input, previous, held, low, high)
                );
            }
        }
    }

    #[test]
    fn a_finite_timeout_holds_inside_the_band_until_its_deadline_and_then_reads_x() {
        let law = AnalogBandLaw { timeout: 100.0e-9 };
        let held = Some(DigitalState::One);
        assert_eq!(law.decision(1.5, Some(2.5), held, 1.0, 2.0, false), None);
        assert_eq!(
            law.decision(1.5, Some(1.5), held, 1.0, 2.0, true),
            Some((DigitalState::Unknown, 1.0))
        );
        // Outside the band an expired deadline is stale and changes nothing.
        assert_eq!(
            law.decision(2.5, Some(1.5), held, 1.0, 2.0, true),
            Some((DigitalState::One, 2.0))
        );
        assert_eq!(
            law.decision(0.5, Some(1.5), held, 1.0, 2.0, true),
            Some((DigitalState::Zero, 1.0))
        );
    }
}
