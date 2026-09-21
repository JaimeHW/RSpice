//! Phase-sampled voltage and correlated threshold-crossing timing noise.
//!
//! A sample is the coherent sum of output sidebands, not their power sum.
//! Timing uses delta_t = -delta_v / slew at the solved threshold crossing.
//! A delay observation combines both crossings before the noise covariance
//! is evaluated, including their offset-frequency phase separation.

use super::*;
use crate::analysis::harmonic_balance::{PeriodicNoiseOutput, PeriodicNoiseProjection};
use std::f64::consts::TAU;

pub use crate::analysis::pnoise::{
    PeriodicNoiseEdge, PeriodicNoiseEdgeDirection, PeriodicNoiseSamplePoint, PeriodicNoiseSampling,
    PeriodicNoiseSamplingEvidence,
};

struct Sample {
    nodes: (Option<usize>, Option<usize>),
    phase: Value,
    scale: Value,
}

pub(super) struct PreparedSampling {
    pub evidence: PeriodicNoiseSamplingEvidence,
    output: Sample,
    reference: Option<Sample>,
    delay_cycles: Value,
}

fn invalid(message: impl Into<String>) -> SimulationError {
    SimulationError::Circuit(message.into())
}

/// Bounded evaluation of the actual carrier Fourier series and its derivatives.
struct Orbit<'a> {
    coefficients: Vec<Complex64>,
    first_bound: Value,
    second_bound: Value,
    roundoff: Value,
    evaluations: usize,
    limit: usize,
    abort: &'a dyn AbortSignal,
}

impl<'a> Orbit<'a> {
    fn new(
        state: &HbSolverState,
        nodes: (Option<usize>, Option<usize>),
        limit: usize,
        abort: &'a dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        let positive = nodes.0.and_then(|node| state.x.get(node));
        let negative = nodes.1.and_then(|node| state.x.get(node));
        let count = positive
            .map_or(0, Vec::len)
            .max(negative.map_or(0, Vec::len));
        let mut coefficients = Vec::with_capacity(count);
        let (mut first_bound, mut second_bound, mut amplitude) = (0.0, 0.0, 0.0);
        for index in 0..count {
            if index.is_multiple_of(256) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let coefficient = positive
                .and_then(|values| values.get(index))
                .copied()
                .unwrap_or_default()
                - negative
                    .and_then(|values| values.get(index))
                    .copied()
                    .unwrap_or_default();
            let magnitude = coefficient.norm() * if index == 0 { 1.0 } else { 2.0 };
            amplitude += magnitude;
            first_bound += index as Value * magnitude;
            second_bound += (index as Value).powi(2) * magnitude;
            coefficients.push(coefficient);
        }
        let roundoff = amplitude * Value::EPSILON * 32.0 * count.max(1) as Value;
        if !amplitude.is_finite()
            || !first_bound.is_finite()
            || !second_bound.is_finite()
            || !roundoff.is_finite()
        {
            return Err(invalid(
                "sampled PNOISE carrier or derivative bounds are non-finite",
            ));
        }
        Ok(Self {
            coefficients,
            first_bound,
            second_bound,
            roundoff,
            evaluations: 0,
            limit,
            abort,
        })
    }

    fn at(&mut self, phase: Value) -> Result<(Value, Value), SimulationError> {
        if self.abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        self.evaluations = self.evaluations.saturating_add(1);
        crate::ResourceLimitError::ensure(
            crate::ResourceKind::AnalysisPoints,
            self.evaluations,
            self.limit,
        )?;
        let mut value = self.coefficients.first().map_or(0.0, |c| c.re);
        let mut derivative = 0.0;
        let (mut value_error, mut derivative_error) = (0.0, 0.0);
        for (index, coefficient) in self.coefficients.iter().enumerate().skip(1) {
            if index.is_multiple_of(256) && self.abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let c =
                *coefficient * Complex64::from_polar(1.0, (index as Value * phase).rem_euclid(TAU));
            let term = 2.0 * c.re - value_error;
            let sum = value + term;
            value_error = (sum - value) - term;
            value = sum;
            let term = -2.0 * index as Value * c.im - derivative_error;
            let sum = derivative + term;
            derivative_error = (sum - derivative) - term;
            derivative = sum;
        }
        if !value.is_finite() || !derivative.is_finite() {
            return Err(invalid("sampled PNOISE carrier evaluation is non-finite"));
        }
        Ok((value, derivative))
    }

    fn crossing(&mut self, edge: &PeriodicNoiseEdge) -> Result<Value, SimulationError> {
        if self.first_bound == 0.0 {
            return Err(invalid(
                "sampled PNOISE cannot find a crossing on a constant carrier output",
            ));
        }
        let tolerance = edge.phase_tolerance_degrees.to_radians();
        let mut pending = vec![(0.0, TAU)];
        let mut found = Vec::<Value>::new();
        while let Some((left, right)) = pending.pop() {
            let middle = (left + right) * 0.5;
            let radius = (right - left) * 0.5;
            let (value, derivative) = self.at(middle)?;
            let residual = value - edge.threshold_volts;
            // A Lipschitz enclosure excludes roots; derivative bounds certify
            // monotonic intervals. Close crossing pairs are subdivided rather
            // than silently skipped by a fixed sampling grid.
            if residual.abs() > self.first_bound * radius + self.roundoff {
                continue;
            }
            if derivative.abs() <= self.second_bound * radius + self.roundoff {
                if right - left <= tolerance || middle == left || middle == right {
                    return Err(invalid(
                        "sampled PNOISE threshold is tangent or crossings cannot be resolved at the requested phase tolerance",
                    ));
                }
                pending.push((middle, right));
                pending.push((left, middle));
                continue;
            }
            let rising = derivative > 0.0;
            if matches!(edge.direction, PeriodicNoiseEdgeDirection::Rising) && !rising
                || matches!(edge.direction, PeriodicNoiseEdgeDirection::Falling) && rising
            {
                continue;
            }
            let mut a = left;
            let mut b = right;
            let va = self.at(a)?.0 - edge.threshold_volts;
            let vb = self.at(b)?.0 - edge.threshold_volts;
            if rising && (va > self.roundoff || vb < -self.roundoff)
                || !rising && (va < -self.roundoff || vb > self.roundoff)
            {
                continue;
            }
            let phase = if va.abs() <= self.roundoff {
                a
            } else if vb.abs() <= self.roundoff {
                b
            } else {
                while b - a > tolerance {
                    let mid = (a + b) * 0.5;
                    if mid == a || mid == b {
                        break;
                    }
                    let vm = self.at(mid)?.0 - edge.threshold_volts;
                    if (vm < 0.0) == rising {
                        a = mid;
                    } else {
                        b = mid;
                    }
                }
                (a + b) * 0.5
            };
            let phase = if TAU - phase <= tolerance { 0.0 } else { phase };
            if found.iter().all(|old| (old - phase).abs() > tolerance) {
                found.push(phase);
            }
        }
        found.sort_by(Value::total_cmp);
        found.get(edge.occurrence - 1).copied().ok_or_else(|| {
            invalid(format!(
                "sampled PNOISE found {} matching crossings, requested occurrence {}",
                found.len(),
                edge.occurrence
            ))
        })
    }
}

impl PreparedSampling {
    pub(super) fn prepare(
        request: &PeriodicNoiseSampling,
        state: &HbSolverState,
        names: &[String],
        nodes: (usize, Option<usize>),
        frequency: Value,
        limit: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        request.validate().map_err(invalid)?;
        let make_sample = |nodes: (Option<usize>, Option<usize>),
                           edge: Option<&PeriodicNoiseEdge>,
                           phase: Value|
         -> Result<(Sample, PeriodicNoiseSamplePoint), SimulationError> {
            let mut orbit = Orbit::new(state, nodes, limit, abort)?;
            let phase = if let Some(edge) = edge {
                orbit.crossing(edge)?
            } else {
                phase.rem_euclid(360.0).to_radians()
            };
            let (voltage, derivative) = orbit.at(phase)?;
            let slew = derivative * TAU * frequency;
            if !slew.is_finite() {
                return Err(invalid("sampled PNOISE crossing slew is non-finite"));
            }
            let scale = if let Some(edge) = edge {
                if slew == 0.0 || slew.abs() < edge.minimum_slew_volts_per_second {
                    return Err(invalid(
                        "sampled PNOISE crossing slew is zero or below the configured minimum",
                    ));
                }
                let scale = -1.0 / slew;
                if !scale.is_finite() || scale == 0.0 {
                    return Err(invalid("sampled PNOISE timing scale is not representable"));
                }
                scale
            } else {
                1.0
            };
            Ok((
                Sample {
                    nodes,
                    phase,
                    scale,
                },
                PeriodicNoiseSamplePoint {
                    node: nodes
                        .0
                        .map_or_else(|| "0".into(), |index| names[index].clone()),
                    reference: nodes.1.map(|index| names[index].clone()),
                    phase_degrees: phase.to_degrees(),
                    voltage,
                    slew_volts_per_second: slew,
                },
            ))
        };
        let edge = match request {
            PeriodicNoiseSampling::Phase { .. } => None,
            PeriodicNoiseSampling::Edge { edge } | PeriodicNoiseSampling::Delay { edge, .. } => {
                Some(edge)
            }
        };
        let phase = if let PeriodicNoiseSampling::Phase { phase_degrees } = request {
            *phase_degrees
        } else {
            0.0
        };
        let (output, output_evidence) = make_sample((Some(nodes.0), nodes.1), edge, phase)?;
        let (reference, reference_evidence, delay_cycles) = if let PeriodicNoiseSampling::Delay {
            reference_node,
            reference_ref,
            reference_edge,
            periods,
            ..
        } = request
        {
            let resolve = |name: &str| -> Result<Option<usize>, SimulationError> {
                if name == "0" || name.eq_ignore_ascii_case("gnd") {
                    return Ok(None);
                }
                names
                    .iter()
                    .position(|node| node.eq_ignore_ascii_case(name.trim()))
                    .map(Some)
                    .ok_or_else(|| {
                        invalid(format!(
                            "sampled PNOISE reference node {name:?} was not found"
                        ))
                    })
            };
            let reference_nodes = (
                resolve(reference_node)?,
                reference_ref.as_deref().map(resolve).transpose()?.flatten(),
            );
            let (mut reference, evidence) =
                make_sample(reference_nodes, Some(reference_edge), 0.0)?;
            reference.scale = -reference.scale;
            let delay_cycles = f64::from(*periods) + (output.phase - reference.phase) / TAU;
            if delay_cycles < 0.0 || !(delay_cycles / frequency).is_finite() {
                return Err(invalid(
                    "sampled PNOISE output crossing precedes its reference; increase the delay period count",
                ));
            }
            (Some(reference), Some(evidence), delay_cycles)
        } else {
            (None, None, 0.0)
        };
        Ok(Self {
            evidence: PeriodicNoiseSamplingEvidence {
                request: request.clone(),
                output: output_evidence,
                reference: reference_evidence,
                nominal_delay_seconds: reference.as_ref().map(|_| delay_cycles / frequency),
            },
            output,
            reference,
            delay_cycles,
        })
    }

    pub(super) fn projection(
        &self,
        offset: Value,
        fundamental: Value,
        max_sideband: i32,
    ) -> PeriodicNoiseProjection {
        let delay_phase = TAU * ((offset / fundamental) * self.delay_cycles).rem_euclid(1.0);
        let mut terms = Vec::new();
        for (sample, offset_phase) in [
            (Some(&self.output), delay_phase),
            (self.reference.as_ref(), 0.0),
        ] {
            let Some(sample) = sample else {
                continue;
            };
            for sideband in -max_sideband..=max_sideband {
                let phase = (Value::from(sideband) * sample.phase + offset_phase).rem_euclid(TAU);
                terms.push((
                    PeriodicNoiseOutput {
                        node_pos: sample.nodes.0,
                        node_neg: sample.nodes.1,
                        sideband,
                    },
                    Complex64::from_polar(sample.scale, phase),
                ));
            }
        }
        PeriodicNoiseProjection { terms }
    }
}

#[cfg(test)]
mod tests;
