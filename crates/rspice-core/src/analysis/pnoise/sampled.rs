//! Requests and retained sampling geometry for periodic noise.

use crate::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PeriodicNoiseEdgeDirection {
    Rising,
    Falling,
    Either,
}

/// One crossing in a half-open carrier period, starting at phase zero.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PeriodicNoiseEdge {
    pub threshold_volts: Value,
    pub direction: PeriodicNoiseEdgeDirection,
    /// One-based occurrence among crossings in the selected direction.
    pub occurrence: usize,
    pub phase_tolerance_degrees: Value,
    /// Reject timing measurements at crossings below this absolute slew.
    pub minimum_slew_volts_per_second: Value,
}

impl Default for PeriodicNoiseEdge {
    fn default() -> Self {
        Self {
            threshold_volts: 0.0,
            direction: PeriodicNoiseEdgeDirection::Rising,
            occurrence: 1,
            phase_tolerance_degrees: 1e-8,
            minimum_slew_volts_per_second: 0.0,
        }
    }
}

impl PeriodicNoiseEdge {
    pub fn validate(&self) -> Result<(), String> {
        if !self.threshold_volts.is_finite() || self.occurrence == 0 {
            return Err(
                "sampled PNOISE needs a finite threshold and a positive crossing occurrence".into(),
            );
        }
        if !self.phase_tolerance_degrees.is_finite()
            || !(1e-12..=1.0).contains(&self.phase_tolerance_degrees)
            || !self.minimum_slew_volts_per_second.is_finite()
            || self.minimum_slew_volts_per_second < 0.0
        {
            return Err("sampled PNOISE phase tolerance must be 1e-12..1 degree and minimum slew must be finite and nonnegative".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum PeriodicNoiseSampling {
    Phase {
        phase_degrees: Value,
    },
    Edge {
        edge: PeriodicNoiseEdge,
    },
    Delay {
        edge: PeriodicNoiseEdge,
        reference_node: String,
        reference_ref: Option<String>,
        reference_edge: PeriodicNoiseEdge,
        /// Carrier periods added to the output crossing, relative to the reference.
        periods: u32,
    },
}

impl PeriodicNoiseSampling {
    pub fn validate(&self) -> Result<(), String> {
        match self {
            Self::Phase { phase_degrees } if !phase_degrees.is_finite() => {
                Err("sampled PNOISE phase must be finite".into())
            }
            Self::Phase { .. } => Ok(()),
            Self::Edge { edge } => edge.validate(),
            Self::Delay {
                edge,
                reference_node,
                reference_ref,
                reference_edge,
                ..
            } => {
                edge.validate()?;
                reference_edge.validate()?;
                if reference_node.trim().is_empty()
                    || reference_node.chars().any(char::is_control)
                    || reference_ref.as_ref().is_some_and(|name| {
                        name.trim().is_empty() || name.chars().any(char::is_control)
                    })
                {
                    return Err("sampled PNOISE delay needs a valid reference voltage probe".into());
                }
                Ok(())
            }
        }
    }

    pub const fn is_timing(&self) -> bool {
        !matches!(self, Self::Phase { .. })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PeriodicNoiseSamplePoint {
    pub node: String,
    pub reference: Option<String>,
    pub phase_degrees: Value,
    pub voltage: Value,
    pub slew_volts_per_second: Value,
}

/// The exact sampling request and the crossings resolved on the retained orbit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PeriodicNoiseSamplingEvidence {
    pub request: PeriodicNoiseSampling,
    pub output: PeriodicNoiseSamplePoint,
    pub reference: Option<PeriodicNoiseSamplePoint>,
    pub nominal_delay_seconds: Option<Value>,
}
