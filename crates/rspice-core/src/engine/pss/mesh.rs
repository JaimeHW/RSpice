//! Immutable integration times shared by shooting and derivative workers.

use super::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(in crate::engine) struct PssIntegrationMesh {
    period: Value,
    times: Arc<[Value]>,
}

impl PssIntegrationMesh {
    pub(in crate::engine) fn from_times(
        period: Value,
        times: Vec<Value>,
    ) -> Result<Self, SimulationError> {
        if !period.is_finite()
            || period <= 0.0
            || times.len() < 2
            || times.first() != Some(&0.0)
            || times.last() != Some(&period)
            || times.iter().any(|time| !time.is_finite())
            || times.windows(2).any(|pair| pair[1] <= pair[0])
        {
            return Err(PssError::InvalidConfig(
                "integration mesh must increase from zero to the exact finite period".to_owned(),
            )
            .into());
        }
        Ok(Self {
            period,
            times: times.into(),
        })
    }

    pub(in crate::engine) fn steps(&self) -> usize {
        self.times.len() - 1
    }

    pub(super) fn time(&self, index: usize, period: Value) -> Value {
        if index == self.steps() {
            period
        } else if period == self.period {
            // Replay source corners bit for bit. Normalizing a retained
            // time and multiplying it back can move an ideal edge by an ULP.
            self.times[index]
        } else {
            // An autonomous period probe scales the same phase mesh.
            period * (self.times[index] / self.period)
        }
    }

    pub(super) fn refined(&self, abort: &dyn AbortSignal) -> Result<Self, SimulationError> {
        let count = self
            .steps()
            .checked_mul(2)
            .and_then(|steps| steps.checked_add(1))
            .ok_or_else(|| {
                PssError::InvalidConfig("integration mesh size overflowed".to_owned())
            })?;
        let mut times = Vec::new();
        times.try_reserve_exact(count).map_err(|_| {
            PssError::InvalidConfig("integration mesh allocation failed".to_owned())
        })?;
        for (index, pair) in self.times.windows(2).enumerate() {
            if index & 0xff == 0 && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let midpoint = pair[0] + 0.5 * (pair[1] - pair[0]);
            if midpoint <= pair[0] || midpoint >= pair[1] {
                return Err(PssError::InvalidConfig(format!(
                    "integration interval [{:.17e}, {:.17e}] has no representable refinement point",
                    pair[0], pair[1],
                ))
                .into());
            }
            times.push(pair[0]);
            times.push(midpoint);
        }
        times.push(self.period);
        Self::from_times(self.period, times)
    }
}

impl Engine {
    /// Add authored independent-source corners to a bounded uniform base.
    /// Behavioral interval bounds remain active until their event schedule
    /// can provide the same complete timing contract.
    pub(in crate::engine) fn pss_source_mesh(
        &self,
        circuit: &CircuitData,
        config: &PssConfig,
        steps: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Option<PssIntegrationMesh>, SimulationError> {
        self.ensure_analysis_points(steps)?;
        if config.is_autonomous() {
            return Ok(None);
        }
        let period = config.period();
        let selected = circuit
            .independent_source_pss_properties(period, false)
            .filter(|(_, _, _, interval)| interval.is_some())
            .map(|(name, _, _, _)| name.to_ascii_lowercase())
            .collect::<std::collections::HashSet<_>>();
        if selected.is_empty() {
            return Ok(None);
        }
        let mut events = BreakpointManager::new_with_tolerance(Value::from_bits(1));
        Self::collect_independent_source_breakpoints(
            circuit,
            BreakpointWindow {
                tstop: period,
                tstep_hint: period / config.points_per_period as Value,
                dialect: self.config.spice_dialect,
            },
            Some(&selected),
            &mut events,
            abort,
            self.config.resource_limits.max_analysis_points,
            crate::engine::transient::SourceBreakpointGeometry::PhysicalCorners,
        )?;
        let events = events.times();
        if events.is_empty() {
            return Ok(None);
        }
        let capacity = steps
            .checked_add(events.len())
            .and_then(|count| count.checked_add(1))
            .ok_or_else(|| {
                PssError::InvalidConfig("source integration mesh size overflowed".to_owned())
            })?;
        // The temporary merge and immutable mesh can coexist during conversion.
        self.ensure_result_values(capacity.saturating_mul(2).saturating_add(events.len()))?;
        let mut times = Vec::new();
        times.try_reserve_exact(capacity).map_err(|_| {
            PssError::InvalidConfig("source integration mesh allocation failed".to_owned())
        })?;
        let dt = period / steps as Value;
        let mut grid = 0;
        let mut event = 0;
        let mut changed = false;
        while grid <= steps || event < events.len() {
            if (grid + event) & 0xff == 0 && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let regular = if grid <= steps {
                Some(if grid == steps {
                    period
                } else {
                    grid as Value * dt
                })
            } else {
                None
            };
            let authored = events.get(event).copied();
            let next = match (regular, authored) {
                (Some(regular), Some(authored))
                    if regular == authored
                        || (grid > 0
                            && grid < steps
                            && (regular - authored).abs()
                                <= 8.0 * Value::EPSILON * regular.abs().max(authored.abs())) =>
                {
                    changed |= regular != authored;
                    grid += 1;
                    event += 1;
                    authored
                }
                (Some(regular), Some(authored)) if regular < authored => {
                    grid += 1;
                    regular
                }
                (Some(_), Some(authored)) | (None, Some(authored)) => {
                    event += 1;
                    changed = true;
                    authored
                }
                (Some(regular), None) => {
                    grid += 1;
                    regular
                }
                (None, None) => break,
            };
            if times.last() != Some(&next) {
                times.push(next);
                self.ensure_analysis_points(times.len().saturating_sub(1))?;
            }
        }
        if changed {
            PssIntegrationMesh::from_times(period, times).map(Some)
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mesh_refinement_preserves_every_original_time_at_extreme_scales() {
        for period in [1e-300, 1e-6, 1e300] {
            let times = vec![0.0, period * 0.013, period * 0.25, period * 0.731, period];
            let mesh = PssIntegrationMesh::from_times(period, times.clone()).unwrap();
            let refined = mesh.refined(&NoAbort).unwrap();
            for (index, &time) in times.iter().enumerate() {
                assert_eq!(refined.time(2 * index, period).to_bits(), time.to_bits());
                assert_eq!(mesh.time(index, period).to_bits(), time.to_bits());
            }
            assert!(refined.times.windows(2).all(|pair| pair[0] < pair[1]));
            assert_eq!(mesh.time(mesh.steps(), period * 1.01), period * 1.01);
        }
    }

    #[test]
    fn mesh_refinement_refuses_unrepresentable_intervals_and_obeys_cancellation() {
        let mesh =
            PssIntegrationMesh::from_times(1.0, vec![0.0, 0.5, 0.5_f64.next_up(), 1.0]).unwrap();
        assert!(
            mesh.refined(&NoAbort)
                .unwrap_err()
                .to_string()
                .contains("no representable refinement point")
        );
        assert!(matches!(
            mesh.refined(&crate::abort_signal::CountingAbort::new(0)),
            Err(SimulationError::Aborted)
        ));
    }
}
