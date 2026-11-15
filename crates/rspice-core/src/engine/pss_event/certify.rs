//! Certification of an event orbit before it is published.
//!
//! - **Periodicity** is the Newton exit itself: every coordinate closes to
//!   its relative or absolute tolerance, and the discrete state comes back.
//! - **Mesh**: the orbit on the bisected mesh agrees with the orbit on the
//!   nominal one at every nominal point and every event, within the engine's
//!   transient tolerances, each node's relative one taken of the node's peak
//!   over the period as native shooting takes it; otherwise the mesh is
//!   refined and compared again.
//! - **Linearization**: `Φ`, assembled from coordinate columns, predicts the
//!   map's response along three seeded directions to 1e-4.
//! - **Event margins**: no two state-dependent events of different sources
//!   lie within the guard of each other, and no crossing grazes: neither a
//!   crossing whose expression moves less than its tolerance across the
//!   guard, nor a detector whose expression turns back within its tolerance
//!   of zero without crossing.
//!
//! A failed check refuses the orbit by name; it is never returned.

use super::Refusal;
use super::boundary::ObservedEvent;
use super::newton::{FD_STEP, Orbit, Traversal, signature};
use crate::Value;
use crate::analysis::pss::RefinementTolerance;
use crate::engine::SimulationError;
use crate::engine::transient::state_capture::CapturedTransientState;

/// How far the linearization may miss the map along a seeded direction.
const DIRECTION_AGREEMENT: Value = 1.0e-4;
/// The relative tolerance of a `cross` expression, against the largest value
/// the expression takes over the orbit.
const EXPRESSION_TOLERANCE: Value = 1.0e-6;

/// The deviation between two orbits, in units of the transient tolerances,
/// and where the worst of it is.
#[derive(Clone, Debug)]
pub(super) struct MeshDeviation {
    pub(super) deviation: Value,
    pub(super) worst: String,
}

impl Orbit<'_> {
    /// Compare an orbit with its refinement at every point of the coarser
    /// mesh and at every event.
    pub(super) fn mesh_deviation(&self, coarse: &Traversal, fine: &Traversal) -> MeshDeviation {
        let (reltol, vntol) = self.mesh_tolerance;
        let mut worst = MeshDeviation {
            deviation: 0.0,
            worst: String::new(),
        };
        let mut note = |deviation: Value, what: &dyn Fn() -> String| {
            if deviation > worst.deviation {
                worst.deviation = deviation;
                worst.worst = what();
            }
        };
        // The node voltages carry the orbit's state. A branch current is the
        // derivative of a charge or the drive of a source, and at a switch's
        // change it is a spike each mesh resolves on its own points, which a
        // pointwise comparison would charge to the mesh. Each node is held
        // to its swing, as native shooting holds its waveforms.
        let tolerances: Vec<RefinementTolerance> = coarse
            .result
            .voltages
            .iter()
            .zip(&fine.result.voltages)
            .map(|(a, b)| RefinementTolerance::of(a, b, reltol, vntol))
            .collect();
        for (coarse_index, &time) in coarse.result.time.iter().enumerate() {
            let Ok(fine_index) = fine
                .result
                .time
                .binary_search_by(|probe| probe.total_cmp(&time))
            else {
                continue;
            };
            for (node, ((a, b), tolerance)) in coarse
                .result
                .voltages
                .iter()
                .zip(&fine.result.voltages)
                .zip(&tolerances)
                .enumerate()
            {
                let (Some(&a), Some(&b)) = (a.get(coarse_index), b.get(fine_index)) else {
                    continue;
                };
                note(tolerance.deviation(a, b), &|| {
                    format!(
                        "V({}) at t={time:.6e} s",
                        coarse
                            .result
                            .node_names
                            .get(node)
                            .map_or("?", String::as_str)
                    )
                });
            }
        }
        let period = traversal_period(coarse);
        let time_tolerance = (reltol * period).max(self.guard);
        for (a, b) in coarse.events.iter().zip(&fine.events) {
            note((a.time - b.time).abs() / time_tolerance, &|| {
                format!(
                    "the event on {} at t={:.6e} s",
                    self.basis.discrete_name(a.quantity),
                    a.time
                )
            });
        }
        worst
    }

    /// Check `Φ` against the map along three seeded directions; return the
    /// worst relative miss.
    pub(super) fn certify_linearization(
        &self,
        image: &CapturedTransientState,
        z: &[Value],
        base: &Traversal,
        mesh: &[Value],
        phi: &[Vec<Value>],
    ) -> Result<Value, SimulationError> {
        let n = z.len();
        if n == 0 {
            return Ok(0.0);
        }
        let wanted = signature(&base.events);
        let mut seed: u64 = 0x9e37_79b9_7f4a_7c15;
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            (seed >> 11) as Value / (1u64 << 53) as Value * 2.0 - 1.0
        };
        let mut worst: Value = 0.0;
        for _ in 0..3 {
            let direction: Vec<Value> = (0..n)
                .map(|index| next() * self.basis.unit(index).scale(z[index]))
                .collect();
            let mut epsilon = FD_STEP;
            let mut response = None;
            for _ in 0..4 {
                let trial: Vec<Value> = z
                    .iter()
                    .zip(&direction)
                    .map(|(value, step)| value + epsilon * step)
                    .collect();
                let traversal = self.traverse(image, &trial, mesh)?;
                if signature(&traversal.events) == wanted {
                    response = Some(traversal.z_end);
                    break;
                }
                epsilon *= 0.1;
            }
            let Some(response) = response else {
                return Err(Refusal::grazing_under_perturbation("a seeded direction"));
            };
            let measured: Vec<Value> = response
                .iter()
                .zip(&base.z_end)
                .map(|(after, before)| after - before)
                .collect();
            let predicted: Vec<Value> = phi
                .iter()
                .map(|row| {
                    epsilon
                        * row
                            .iter()
                            .zip(&direction)
                            .map(|(a, b)| a * b)
                            .sum::<Value>()
                })
                .collect();
            // A response below a millionth of the perturbation is what the
            // traversals' Newton tolerance leaves, not the map's: a map that
            // contracts it away (a clamped capacitor) has nothing to miss.
            let perturbation = epsilon * direction.iter().map(|d| d.abs()).fold(0.0, Value::max);
            let reference = measured
                .iter()
                .chain(&predicted)
                .map(|value| value.abs())
                .fold(1.0e-6 * perturbation, Value::max)
                .max(Value::MIN_POSITIVE);
            let (row, miss) = measured
                .iter()
                .zip(&predicted)
                .map(|(a, b)| (a - b).abs() / reference)
                .enumerate()
                .fold(
                    (0, 0.0),
                    |best, (row, miss)| {
                        if miss > best.1 { (row, miss) } else { best }
                    },
                );
            if miss > DIRECTION_AGREEMENT {
                return Err(Refusal::linearization(self.basis.name(row), miss));
            }
            worst = worst.max(miss);
        }
        Ok(worst)
    }

    /// The smallest gap between state-dependent events of different sources
    /// and the smallest crossing rate on the orbit; refuses an ambiguous
    /// order or a grazing crossing.
    pub(super) fn certify_margins(
        &self,
        traversal: &Traversal,
    ) -> Result<(Option<Value>, Option<Value>), SimulationError> {
        let period = traversal_period(traversal);
        let crossing_offset = self.basis.crossing_offset();
        let dependent: Vec<&ObservedEvent> = traversal
            .events
            .iter()
            .filter(|event| event.state_dependent)
            .collect();
        let mut min_gap: Option<Value> = None;
        for (index, event) in dependent.iter().enumerate() {
            for other in dependent.iter().skip(index + 1) {
                // Two switches changing together are solved at one accepted
                // point; their order has no meaning to compete over.
                if other.quantity == event.quantity
                    || (event.quantity < crossing_offset && other.quantity < crossing_offset)
                {
                    continue;
                }
                let direct = (other.time - event.time).abs();
                let gap = direct.min((period - direct).abs());
                min_gap = Some(min_gap.map_or(gap, |best: Value| best.min(gap)));
                if gap <= self.guard {
                    return Err(Refusal::order_ambiguous(
                        (self.basis.discrete_name(event.quantity), event.time),
                        (self.basis.discrete_name(other.quantity), other.time),
                        self.guard,
                    ));
                }
            }
        }
        let detectors = traversal
            .crossings
            .first()
            .map_or(0, |(_, values)| values.len());
        let mut scale = vec![0.0_f64; detectors];
        for (_, values) in &traversal.crossings {
            for (slot, value) in scale.iter_mut().zip(values) {
                *slot = slot.max(value.abs());
            }
        }
        let tolerance: Vec<Value> = scale
            .iter()
            .map(|scale| (EXPRESSION_TOLERANCE * scale).max(1.0e-12))
            .collect();
        let mut min_slope: Option<Value> = None;
        for event in &dependent {
            let Some(slope) = event.slope else {
                continue;
            };
            let detector = event.quantity.saturating_sub(crossing_offset);
            let floor = tolerance.get(detector).copied().unwrap_or(1.0e-12);
            min_slope = Some(min_slope.map_or(slope.abs(), |best: Value| best.min(slope.abs())));
            if slope.abs() * self.guard < floor {
                return Err(Refusal::grazing(
                    self.basis.discrete_name(event.quantity),
                    event.time,
                    slope,
                ));
            }
        }
        for (detector, &floor) in tolerance.iter().enumerate() {
            let series: Vec<(Value, Value)> = traversal
                .crossings
                .iter()
                .filter_map(|(time, values)| values.get(detector).map(|value| (*time, *value)))
                .collect();
            for window in series.windows(3) {
                let [(t0, a), (t1, b), (t2, c)] = [window[0], window[1], window[2]];
                let turning = (b - a) * (c - b) < 0.0;
                if !turning || a.signum() != b.signum() || b.signum() != c.signum() {
                    continue;
                }
                let extremum = parabolic_extremum((t0, a), (t1, b), (t2, c));
                if extremum.abs() < floor {
                    return Err(Refusal::grazing(
                        self.basis.discrete_name(crossing_offset + detector),
                        t1,
                        0.0,
                    ));
                }
            }
        }
        Ok((min_gap, min_slope))
    }
}

/// The span a traversal covered.
fn traversal_period(traversal: &Traversal) -> Value {
    traversal.end.time - traversal.result.time.first().copied().unwrap_or(0.0)
}

/// The extreme value of the parabola through three points.
fn parabolic_extremum(a: (Value, Value), b: (Value, Value), c: (Value, Value)) -> Value {
    let (x0, y0) = a;
    let (x1, y1) = b;
    let (x2, y2) = c;
    let denominator = (x0 - x1) * (x0 - x2) * (x1 - x2);
    if denominator == 0.0 {
        return y1;
    }
    let p = (x2 * (y1 - y0) + x1 * (y0 - y2) + x0 * (y2 - y1)) / denominator;
    let q = (x2 * x2 * (y0 - y1) + x1 * x1 * (y2 - y0) + x0 * x0 * (y1 - y2)) / denominator;
    let r = (x1 * x2 * (x1 - x2) * y0 + x2 * x0 * (x2 - x0) * y1 + x0 * x1 * (x0 - x1) * y2)
        / denominator;
    if p == 0.0 {
        return y1;
    }
    let vertex = -q / (2.0 * p);
    if vertex < x0.min(x2) || vertex > x0.max(x2) {
        return y1;
    }
    r - q * q / (4.0 * p)
}

#[cfg(test)]
mod tests {
    use super::parabolic_extremum;

    #[test]
    fn the_parabola_through_three_samples_finds_a_touching_peak() {
        // y = 1 - (x - 1)^2 peaks at 1 between its samples at 0.5 and 1.25.
        let at = |x: f64| 1.0 - (x - 1.0) * (x - 1.0);
        let peak = parabolic_extremum((0.5, at(0.5)), (0.75, at(0.75)), (1.25, at(1.25)));
        assert!((peak - 1.0).abs() <= 1.0e-12, "{peak}");
    }
}
