//! Analog input thresholds that require a physical mixed-signal timepoint.

use crate::Value;
use super::DigitalState;

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
