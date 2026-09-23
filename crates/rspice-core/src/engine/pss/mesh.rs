//! Immutable integration times shared by shooting and derivative workers.

use super::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(in crate::engine) struct PssIntegrationMesh {
    period: Value,
    times: Arc<[Value]>,
    delay_corners: Arc<[Value]>,
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
            delay_corners: Arc::from([]),
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

    pub(super) fn times(&self) -> &[Value] {
        &self.times
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
        let mut refined = Self::from_times(self.period, times)?;
        refined.delay_corners = self.delay_corners.clone();
        Ok((refined, retained))
    }
}

impl Engine {
    /// Resolve another generation of line arrivals only between solved meshes.
    /// All shooting/Jacobian workers then replay the same immutable clocks.
    pub(super) fn pss_propagated_source_mesh(
        &self,
        mesh: &PssIntegrationMesh,
        circuit: &CircuitData,
        abort: &dyn AbortSignal,
    ) -> Result<(PssIntegrationMesh, Vec<usize>, Vec<Value>), SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let mut refined = mesh.clone();
        let retained = (0..mesh.times.len()).collect();
        if mesh.delay_corners.is_empty() {
            return Ok((refined, retained, Vec::new()));
        }
        let period = mesh.period;
        let mut corners = mesh.delay_corners.to_vec();
        let mut added = Vec::new();
        // Preserve explicitly adjacent authored clocks. Otherwise merge only
        // roundoff-sized duplicates from different orders of line traversal.
        let merge_tolerance = if corners
            .windows(2)
            .any(|pair| pair[1] - pair[0] <= 32.0 * Value::EPSILON * period)
        {
            0.0
        } else {
            32.0 * Value::EPSILON * period
        };
        for line in circuit
            .tlines
            .iter()
            .filter(|line| !line.is_memoryless_two_port())
        {
            let phase_delay = line.delay().rem_euclid(period);
            for (index, &corner) in mesh.delay_corners.iter().enumerate() {
                if index & 0xff == 0 && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let arrival = if corner >= period - phase_delay {
                    corner - (period - phase_delay)
                } else {
                    corner + phase_delay
                };
                let position = corners.partition_point(|time| *time < arrival);
                if corners
                    .get(position)
                    .is_some_and(|time| (*time - arrival).abs() <= merge_tolerance)
                    || position
                        .checked_sub(1)
                        .and_then(|index| corners.get(index))
                        .is_some_and(|time| (*time - arrival).abs() <= merge_tolerance)
                {
                    continue;
                }
                self.ensure_analysis_points(corners.len().saturating_add(1))?;
                self.ensure_result_values(
                    refined
                        .times
                        .len()
                        .saturating_add(corners.len())
                        .saturating_add(added.len())
                        .saturating_mul(4),
                )?;
                corners.insert(position, arrival);
                added.push(arrival);
            }
        }
        if added.is_empty() {
            return Ok((refined, retained, added));
        }
        let fraction = (0.01 * self.voltage_reltol()).min(1e-4);
        let count = added.len();
        for index in 0..count {
            let corner = added[index];
            let next_index = corners.partition_point(|time| *time <= corner);
            let next = corners.get(next_index).copied().unwrap_or(period);
            let restart = (corner
                + fraction * (next - corner).min(period / refined.steps() as Value))
            .max(corner.next_up());
            if restart < next {
                added.push(restart);
            }
        }
        self.ensure_analysis_points(refined.steps().saturating_add(added.len()))?;
        self.ensure_result_values(
            refined
                .times
                .len()
                .saturating_add(corners.len())
                .saturating_add(added.len())
                .saturating_mul(4),
        )?;
        let mut times = refined.times.to_vec();
        times.extend_from_slice(&added);
        times.sort_by(Value::total_cmp);
        times.dedup();
        let retained = mesh
            .times
            .iter()
            .map(|time| {
                times
                    .binary_search_by(|value| value.total_cmp(time))
                    .expect("refinement retains original clocks")
            })
            .collect();
        refined = PssIntegrationMesh::from_times(period, times)?;
        refined.delay_corners = corners.into();
        Ok((refined, retained, added))
    }

    /// Resolve reflection/cascade geometry before doubling the smooth mesh.
    /// Advancing one line hop must not double every delay coordinate: several
    /// reflections can need new corners while the intervening waveform is
    /// already represented accurately by the current integration intervals.
    pub(super) fn pss_resolve_delay_corners(
        &self,
        circuit: &mut PssCircuit,
        matrix: &mut StaticMatrix,
        config: &PssConfig,
        mut coarse: PssGridSolution,
        iterations: &mut usize,
        abort: &dyn AbortSignal,
    ) -> Result<PssGridSolution, SimulationError> {
        if config.is_autonomous() {
            return Ok(coarse);
        }
        loop {
            let Some(mesh) = circuit.integration_mesh.clone() else {
                return Ok(coarse);
            };
            if mesh.delay_corners.is_empty() {
                return Ok(coarse);
            }
            let (enriched, retained, events) =
                self.pss_propagated_source_mesh(&mesh, circuit, abort)?;
            if events.is_empty() {
                return Ok(coarse);
            }
            let steps = circuit.integration_steps;
            let basis = circuit.delay_basis.clone();
            self.pss_set_reactive_state(circuit, &coarse.state.x0)?;
            circuit.delay_basis = basis.with_events(
                coarse.state.period,
                &events,
                self.config.resource_limits,
                abort,
            )?;
            let initial = circuit.extract_state();
            self.ensure_pss_refinement_capacity(circuit, mesh.steps(), enriched.steps(), false)?;
            circuit.integration_steps = enriched.steps();
            circuit.integration_mesh = Some(enriched);
            if config.verbose {
                log::debug!(
                    "PSS delayed corners: {} -> {} steps, {} state coordinates",
                    mesh.steps(),
                    circuit.grid_steps(config),
                    circuit.state_dimension()
                );
            }
            let fine = self.pss_solve_grid(
                circuit,
                matrix,
                config,
                ShootingState::new(initial, coarse.state.period),
                abort,
            )?;
            *iterations += fine.iterations;
            let error = self.pss_grid_refinement_error(
                circuit,
                &coarse,
                &fine,
                PssSampleMap::Enriched(&retained),
                abort,
            )?;
            if config.verbose {
                log::debug!(
                    "PSS delayed-corner qualification: normalized waveform error {error:.6e}"
                );
            }
            if error <= 1.0 {
                circuit.integration_steps = steps;
                circuit.integration_mesh = Some(mesh);
                circuit.delay_basis = basis;
                return Ok(coarse);
            }
            coarse = fine;
        }
    }

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
        let mut breakpoints = BreakpointManager::new_with_tolerance(Value::from_bits(1));
        Self::collect_independent_source_breakpoints(
            circuit,
            BreakpointWindow {
                tstop: period,
                tstep_hint: period / config.points_per_period as Value,
                dialect: self.config.spice_dialect,
            },
            Some(&selected),
            &mut breakpoints,
            abort,
            self.config.resource_limits.max_analysis_points,
            crate::engine::transient::SourceBreakpointGeometry::PhysicalCorners,
        )?;
        circuit.behavioral_sources.collect_transient_breakpoints(
            period,
            &mut breakpoints,
            abort,
            self.config.resource_limits.max_analysis_points,
            true,
        )?;
        circuit.capacitors.collect_transient_breakpoints(
            period,
            &mut breakpoints,
            abort,
            self.config.resource_limits.max_analysis_points,
            true,
        )?;
        // Seed the first periodic arrival of each prescribed corner. A delay
        // may span several carrier cycles; only its phase affects this clock,
        // whereas the shooting history still covers the entire physical TD.
        // Subsequent solved meshes extend these through reflections/cascades.
        self.ensure_result_values(breakpoints.times().len().saturating_mul(2))?;
        let source_events = breakpoints.times().to_vec();
        for line in circuit
            .tlines
            .iter()
            .filter(|line| !line.is_memoryless_two_port())
        {
            let phase_delay = line.delay().rem_euclid(period);
            for (index, &event) in source_events.iter().enumerate() {
                if index & 0xff == 0 && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                // Subtract first to avoid overflow near the largest period.
                let arrival = if event >= period - phase_delay {
                    event - (period - phase_delay)
                } else {
                    event + phase_delay
                };
                breakpoints.add(arrival);
                self.ensure_analysis_points(breakpoints.times().len())?;
            }
        }
        drop(source_events);
        self.ensure_result_values(breakpoints.times().len().saturating_mul(3))?;
        let delay_corners = breakpoints.times().to_vec();
        if circuit
            .tlines
            .iter()
            .any(|line| !line.is_memoryless_two_port())
        {
            // Native quadratic wave interpolation uses two predecessors. A
            // short restart interval supplies two samples on the new side of
            // a slope corner, just as TRAN's breakpoint restart does. Keep it
            // in the immutable mesh and its history projection; workers must
            // not choose it from their independently perturbed wave values.
            self.ensure_result_values(breakpoints.times().len().saturating_mul(3))?;
            let corners = &delay_corners;
            let fraction = (0.01 * self.voltage_reltol()).min(1e-4);
            for (index, &corner) in corners.iter().enumerate() {
                if index & 0xff == 0 && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let next = corners.get(index + 1).copied().unwrap_or(period);
                let restart = (corner + fraction * (next - corner).min(period / steps as Value))
                    .max(corner.next_up());
                if restart < next {
                    breakpoints.add(restart);
                    self.ensure_analysis_points(breakpoints.times().len())?;
                }
            }
        }
        let events = breakpoints.times();
        if events.is_empty()
            && !circuit.behavioral_sources.needs_time_resolution(period)
            && !circuit
                .capacitors
                .value_expressions
                .iter()
                .flatten()
                .any(|expression| expression.needs_time_resolution(period))
        {
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
        // Resolution accounts for the base mesh, subdivision scratch and
        // returned mesh together. Release the already-merged event storage.
        drop(breakpoints);
        if let Some(refined) = circuit.behavioral_sources.refine_time_mesh(
            &times,
            crate::device::NonlinearConvergenceCriteria::new(
                self.voltage_abstol(),
                self.current_abstol(),
                self.voltage_reltol(),
            ),
            &self.config.resource_limits,
            abort,
        )? {
            times = refined;
            changed = true;
        }
        if changed {
            let mut mesh = PssIntegrationMesh::from_times(period, times)?;
            if circuit
                .tlines
                .iter()
                .any(|line| !line.is_memoryless_two_port())
            {
                mesh.delay_corners = delay_corners.into();
            }
            Ok(Some(mesh))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delayed_mesh_enrichment_preserves_clocks_and_detects_between_sample_echoes() {
        let engine = Engine::default();
        let deck = Netlist::parse(
            "Echo geometry\nV1 near 0 DC 0\nT1 near 0 far 0 Z0=50 TD=0.375\nR1 far 0 50\n.end\n",
        )
        .unwrap();
        let circuit = PssCircuit::new(engine.build_circuit(&deck).unwrap()).unwrap();
        let mut mesh = PssIntegrationMesh::from_times(1.0, vec![0.0, 0.125, 0.5, 1.0]).unwrap();
        mesh.delay_corners = vec![0.125, 0.5].into();
        let (enriched, retained, events) = engine
            .pss_propagated_source_mesh(&mesh, &circuit, &NoAbort)
            .unwrap();
        assert!(events.contains(&0.875));
        for (index, &time) in mesh.times.iter().enumerate() {
            assert_eq!(enriched.times[retained[index]].to_bits(), time.to_bits());
        }
        assert_eq!(&*mesh.delay_corners, &[0.125, 0.5]);
        assert_eq!(
            &*enriched.refined(&NoAbort).unwrap().0.delay_corners,
            &*enriched.delay_corners
        );
        let solution = |time: Vec<Value>, values: Vec<Value>| PssGridSolution {
            state: ShootingState::new(vec![], 1.0),
            iterations: 0,
            jacobian: None,
            waveform: TransientResult {
                time,
                step_sizes: vec![],
                voltages: vec![values],
                branch_currents: vec![],
                num_nodes: 1,
                node_names: vec!["far".into()],
                branch_names: vec![],
                current_impulses: None,
                digital_traces: vec![],
                digital_buses: vec![],
                real_traces: vec![],
                device_op_traces: vec![],
                store_traces: vec![],
                fft_results: vec![],
            },
        };
        let coarse = solution(mesh.times.to_vec(), vec![0.0; mesh.times.len()]);
        let mut values = vec![0.0; enriched.times.len()];
        values[enriched
            .times
            .binary_search_by(|time| time.total_cmp(&0.875))
            .unwrap()] = 1.0;
        let fine = solution(enriched.times.to_vec(), values);
        assert_eq!(
            engine
                .pss_grid_refinement_error(
                    &circuit,
                    &coarse,
                    &fine,
                    PssSampleMap::Retained(&retained),
                    &NoAbort
                )
                .unwrap(),
            0.0
        );
        assert!(
            engine
                .pss_grid_refinement_error(
                    &circuit,
                    &coarse,
                    &fine,
                    PssSampleMap::Enriched(&retained),
                    &NoAbort
                )
                .unwrap()
                > 1.0
        );
        assert!(matches!(
            engine.pss_propagated_source_mesh(
                &mesh,
                &circuit,
                &crate::abort_signal::CountingAbort::new(0)
            ),
            Err(SimulationError::Aborted)
        ));
        let mut limited = Engine::default();
        limited.config.resource_limits.max_analysis_points = mesh.steps();
        assert!(
            limited
                .pss_propagated_source_mesh(&mesh, &circuit, &NoAbort)
                .is_err()
        );
    }

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
