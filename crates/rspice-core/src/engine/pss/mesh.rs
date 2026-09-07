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

    pub(super) fn refinement_midpoint(left: Value, right: Value) -> Option<Value> {
        let midpoint = left + 0.5 * (right - left);
        (midpoint > left && midpoint < right).then_some(midpoint)
    }

    pub(super) fn refinement_steps(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<usize, SimulationError> {
        let mut count = self.steps();
        for (index, pair) in self.times.windows(2).enumerate() {
            if index & 0xff == 0 && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if Self::refinement_midpoint(pair[0], pair[1]).is_some() {
                count = count.checked_add(1).ok_or_else(|| {
                    PssError::InvalidConfig("integration mesh size overflowed".to_owned())
                })?;
            } else if index == 0 {
                // The startup interval must use BE without derivative history.
                // It cannot be qualified by the alternate-method floor probe.
                return Err(PssError::InvalidConfig(
                    "first integration interval has no representable refinement point".to_owned(),
                )
                .into());
            }
        }
        Ok(count)
    }

    /// Preserve every authored time, including adjacent representable clocks.
    /// The caller separately qualifies those intervals with a solved orbit
    /// using the alternate integration method before accepting the mesh.
    pub(super) fn refined(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<(Self, Vec<usize>), SimulationError> {
        let count = self
            .refinement_steps(abort)?
            .checked_add(1)
            .ok_or_else(|| {
                PssError::InvalidConfig("integration mesh size overflowed".to_owned())
            })?;
        let mut times = Vec::new();
        times.try_reserve_exact(count).map_err(|_| {
            PssError::InvalidConfig("integration mesh allocation failed".to_owned())
        })?;
        let mut retained = Vec::new();
        retained.try_reserve_exact(self.times.len()).map_err(|_| {
            PssError::InvalidConfig("integration mesh index allocation failed".to_owned())
        })?;
        for (index, pair) in self.times.windows(2).enumerate() {
            if index & 0xff == 0 && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            retained.push(times.len());
            times.push(pair[0]);
            if let Some(midpoint) = Self::refinement_midpoint(pair[0], pair[1]) {
                times.push(midpoint);
            }
        }
        retained.push(times.len());
        times.push(self.period);
        Ok((Self::from_times(self.period, times)?, retained))
    }
}

impl Engine {
    /// Add resolved source corners to a bounded uniform base. Behavioral
    /// coordinates without an exact event schedule retain interval bounds.
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
        circuit.behavioral_sources.collect_transient_breakpoints(
            period,
            &mut events,
            abort,
            self.config.resource_limits.max_analysis_points,
            true,
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
            let (refined, retained) = mesh.refined(&NoAbort).unwrap();
            for (index, &time) in times.iter().enumerate() {
                assert_eq!(retained[index], 2 * index);
                assert_eq!(refined.time(2 * index, period).to_bits(), time.to_bits());
                assert_eq!(mesh.time(index, period).to_bits(), time.to_bits());
            }
            assert!(refined.times.windows(2).all(|pair| pair[0] < pair[1]));
            assert_eq!(mesh.time(mesh.steps(), period * 1.01), period * 1.01);
        }
    }

    #[test]
    fn mesh_refinement_preserves_adjacent_clocks_and_obeys_cancellation() {
        let mesh =
            PssIntegrationMesh::from_times(1.0, vec![0.0, 0.5, 0.5_f64.next_up(), 1.0]).unwrap();
        let (refined, retained) = mesh.refined(&NoAbort).unwrap();
        assert_eq!(retained, [0, 2, 3, 5]);
        assert_eq!(refined.steps(), mesh.refinement_steps(&NoAbort).unwrap());
        for (index, &time) in mesh.times.iter().enumerate() {
            assert_eq!(refined.times[retained[index]].to_bits(), time.to_bits());
        }
        assert!(matches!(
            mesh.refined(&crate::abort_signal::CountingAbort::new(0)),
            Err(SimulationError::Aborted)
        ));
        let startup =
            PssIntegrationMesh::from_times(1.0, vec![0.0, Value::from_bits(1), 1.0]).unwrap();
        assert!(
            startup
                .refined(&NoAbort)
                .unwrap_err()
                .to_string()
                .contains("first integration interval has no representable refinement point")
        );
    }
}
