//! Requests and retained sampling geometry for periodic noise.

use crate::Value;
use serde::{Deserialize, Serialize};

pub use crate::netlist::{PeriodicNoiseEdge, PeriodicNoiseEdgeDirection, PeriodicNoiseSampling};

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
    pub carrier_frequency_hz: Value,
    pub request: PeriodicNoiseSampling,
    pub output: PeriodicNoiseSamplePoint,
    pub reference: Option<PeriodicNoiseSamplePoint>,
    pub nominal_delay_seconds: Option<Value>,
}

impl PeriodicNoiseSamplingEvidence {
    /// Check the retained geometry before accepting a serialized observation.
    pub fn validate(&self) -> Result<(), String> {
        self.request.validate()?;
        if !self.carrier_frequency_hz.is_finite() || self.carrier_frequency_hz <= 0.0 {
            return Err("sampled PNOISE evidence needs a positive finite carrier".into());
        }
        let point = |sample: &PeriodicNoiseSamplePoint, edge: Option<&PeriodicNoiseEdge>| {
            if sample.node.trim().is_empty()
                || sample.node.chars().any(char::is_control)
                || sample.reference.as_ref().is_some_and(|node| {
                    node.trim().is_empty() || node.chars().any(char::is_control)
                })
                || !sample.phase_degrees.is_finite()
                || !(0.0..360.0).contains(&sample.phase_degrees)
                || !sample.voltage.is_finite()
                || !sample.slew_volts_per_second.is_finite()
            {
                return Err(
                    "sampled PNOISE evidence contains an invalid voltage sample".to_string()
                );
            }
            if let Some(edge) = edge {
                let slew = sample.slew_volts_per_second;
                if slew == 0.0
                    || slew.abs() < edge.minimum_slew_volts_per_second
                    || (edge.direction == PeriodicNoiseEdgeDirection::Rising && slew < 0.0)
                    || (edge.direction == PeriodicNoiseEdgeDirection::Falling && slew > 0.0)
                {
                    return Err("sampled PNOISE evidence has an inconsistent crossing slew".into());
                }
            }
            Ok(())
        };
        let same_node = |first: &str, second: &str| {
            let is_ground = |s: &str| s == "0" || s.eq_ignore_ascii_case("gnd");
            first.trim().eq_ignore_ascii_case(second.trim())
                || (is_ground(first) && is_ground(second))
        };
        match &self.request {
            PeriodicNoiseSampling::Phase { phase_degrees } => {
                point(&self.output, None)?;
                let delta = (self.output.phase_degrees - phase_degrees.rem_euclid(360.0)).abs();
                if delta.min((360.0 - delta).abs()) > 1e-10 {
                    return Err("sampled PNOISE evidence has a different sampling phase".into());
                }
            }
            PeriodicNoiseSampling::Edge { edge } => point(&self.output, Some(edge))?,
            PeriodicNoiseSampling::Delay {
                edge,
                reference_node,
                reference_ref,
                reference_edge,
                periods,
            } => {
                point(&self.output, Some(edge))?;
                let reference = self
                    .reference
                    .as_ref()
                    .ok_or("sampled PNOISE delay evidence needs a reference crossing")?;
                point(reference, Some(reference_edge))?;
                if !same_node(reference_node, &reference.node)
                    || !same_node(
                        reference_ref.as_deref().unwrap_or("0"),
                        reference.reference.as_deref().unwrap_or("0"),
                    )
                {
                    return Err(
                        "sampled PNOISE delay evidence has a different reference probe".into(),
                    );
                }
                let delay = self
                    .nominal_delay_seconds
                    .ok_or("sampled PNOISE delay evidence needs its nominal delay")?;
                let expected = (f64::from(*periods)
                    + (self.output.phase_degrees - reference.phase_degrees) / 360.0)
                    / self.carrier_frequency_hz;
                let tolerance =
                    32.0 * f64::EPSILON * (expected.abs() + 1.0 / self.carrier_frequency_hz);
                if !delay.is_finite()
                    || delay < 0.0
                    || !expected.is_finite()
                    || (delay - expected).abs() > tolerance
                {
                    return Err(
                        "sampled PNOISE delay evidence has an inconsistent nominal delay".into(),
                    );
                }
            }
        }
        if !matches!(self.request, PeriodicNoiseSampling::Delay { .. })
            && (self.reference.is_some() || self.nominal_delay_seconds.is_some())
        {
            return Err("sampled PNOISE evidence has an unrequested reference crossing".into());
        }
        Ok(())
    }
}
