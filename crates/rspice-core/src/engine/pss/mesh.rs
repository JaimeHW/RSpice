//! Immutable integration times shared by shooting and derivative workers.

use super::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(in crate::engine) struct PssIntegrationMesh {
    period: Value,
    times: Arc<[Value]>,
    delay_corners: Arc<[Value]>,
    /// Provenance of paired samples at a declared ideal source/arrival edge.
    /// Nearby ordinary samples alone never create this ownership.
    sampled_edges: Arc<[SampledEdge]>,
    boundary_event: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct SampledEdge {
    pub time: Value,
    pub incoming: Value,
    pub outgoing: Value,
}

impl SampledEdge {
    fn periodic_boundary(self, enabled: bool) -> Self {
        if enabled && self.time == 0.0 {
            // One physical phase-zero event owns both limits. Positive-time
            // traversal resolves it exactly; negative history keeps two
            // independent coordinates for its incoming/outgoing waves.
            Self {
                time: 0.0,
                incoming: 0.0_f64.next_down(),
                outgoing: 0.0,
            }
        } else {
            self
        }
    }
    fn key(self) -> [u64; 3] {
        [self.time, self.incoming, self.outgoing]
            .map(|time| if time == 0.0 { 0 } else { time.to_bits() })
    }

    fn shifted(self, offsets: &[Value]) -> Self {
        Self {
            time: PssIntegrationMesh::shift_directed(self.time, offsets, 1),
            incoming: PssIntegrationMesh::shift_directed(self.incoming, offsets, -1),
            outgoing: PssIntegrationMesh::shift_directed(self.outgoing, offsets, 1),
        }
    }
}

impl PssIntegrationMesh {
    pub(super) fn has_boundary_event(&self) -> bool {
        self.boundary_event
    }

    pub(super) fn is_period_end(&self, time: Value) -> bool {
        time == self.period
    }

    pub(super) fn source_side(&self, time: Value) -> crate::circuit::SourceTimeSide {
        use crate::circuit::SourceTimeSide;
        if !self.boundary_event {
            return SourceTimeSide::Published;
        }
        if time == 0.0 {
            SourceTimeSide::RightLimit
        } else if time == self.period
            || self
                .sampled_edges
                .iter()
                .any(|edge| edge.time == time && edge.incoming == time && edge.outgoing > time)
        {
            SourceTimeSide::LeftLimit
        } else {
            SourceTimeSide::Published
        }
    }
    pub(super) fn pending_sampled_edges(
        &self,
        delay: Value,
        endpoint: Value,
        limits: crate::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<[Value; 3]>, SimulationError> {
        if self.sampled_edges.is_empty() {
            return Ok(Vec::new());
        }
        if !endpoint.is_finite()
            || endpoint < 0.0
            || endpoint > self.period
            || !delay.is_finite()
            || delay <= 0.0
        {
            return Err(SimulationError::Circuit(
                "invalid periodic delay event window".into(),
            ));
        }
        let cycles = (delay / self.period).ceil();
        if !cycles.is_finite() || cycles >= usize::MAX as Value {
            return Err(SimulationError::Circuit(
                "periodic delay event window overflows".into(),
            ));
        }
        let capacity = (cycles as usize)
            .saturating_add(1)
            .saturating_mul(self.sampled_edges.len());
        crate::ResourceLimitError::ensure(
            crate::ResourceKind::AnalysisPoints,
            capacity,
            limits.max_analysis_points,
        )?;
        crate::ResourceLimitError::ensure(
            crate::ResourceKind::ResultValues,
            capacity.saturating_mul(3),
            limits.max_result_values,
        )?;
        let mut events = Vec::new();
        events.try_reserve_exact(capacity).map_err(|_| {
            SimulationError::Circuit("periodic delay event allocation failed".into())
        })?;
        for cycle in 0..=cycles as usize {
            for (index, &edge) in self.sampled_edges.iter().enumerate() {
                if index & 0xff == 0 && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let edge = if cycle == 0 {
                    edge
                } else {
                    edge.shifted(&[-self.period, -((cycle - 1) as Value * self.period)])
                };
                // Only events that have yet to arrive after the continuation
                // origin need ownership. Older history keeps its sampled form.
                if edge.time > endpoint - delay && edge.time <= endpoint {
                    events.push([edge.time, edge.incoming, edge.outgoing]);
                }
            }
        }
        events.sort_by(|a, b| {
            a[0].total_cmp(&b[0])
                .then(a[1].total_cmp(&b[1]))
                .then(a[2].total_cmp(&b[2]))
        });
        events.dedup();
        Ok(events)
    }

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
            sampled_edges: Arc::from([]),
            boundary_event: false,
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

    fn shift_directed(mut time: Value, offsets: &[Value], direction: i8) -> Value {
        for &offset in offsets {
            let sum = time + offset;
            let recovered = sum - time;
            let error = (time - (sum - recovered)) + (offset - recovered);
            time = if direction < 0 && error < 0.0 {
                sum.next_down()
            } else if direction > 0 && error > 0.0 {
                sum.next_up()
            } else {
                sum
            };
        }
        time
    }

    /// Preserve the two sides of a represented edge when shifting it onto a
    /// coarser floating-point clock. Ordinary corners keep nearest rounding;
    /// adjacent lower/upper samples use outward rounding of the exact sum.
    pub(super) fn shifted_clock(times: &[Value], index: usize, offsets: &[Value]) -> Value {
        let original = times[index];
        let lower = times
            .get(index + 1)
            .is_some_and(|next| original.next_up() == *next);
        let upper = index
            .checked_sub(1)
            .is_some_and(|previous| times[previous].next_up() == original);
        let mut time = original;
        for &offset in offsets {
            let sum = time + offset;
            let recovered = sum - time;
            let error = (time - (sum - recovered)) + (offset - recovered);
            time = if lower && error < 0.0 {
                sum.next_down()
            } else if upper && error > 0.0 {
                sum.next_up()
            } else {
                sum
            };
        }
        time
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
        refined.sampled_edges = self.sampled_edges.clone();
        refined.boundary_event = self.boundary_event;
        Ok((refined, retained))
    }
}

// PWL permits multiple values at one authored clock. Other native waveform
// corners retain their established schedule; rounded ramp endpoints must not
// be reclassified as ideal jumps by comparing independently evaluated limits.
fn is_pwl_source(spec: &crate::netlist::SourceSpec) -> bool {
    use crate::netlist::SourceSpec;
    match spec {
        SourceSpec::Pwl { .. } | SourceSpec::PwlFile { .. } => true,
        SourceSpec::Distortion { inner, .. }
        | SourceSpec::RfPort { inner, .. }
        | SourceSpec::DcTransient {
            transient: inner, ..
        }
        | SourceSpec::AcTransient {
            transient: inner, ..
        }
        | SourceSpec::DcAcTransient {
            transient: inner, ..
        } => is_pwl_source(inner),
        _ => false,
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
        self.ensure_result_values(
            mesh.times
                .len()
                .saturating_add(mesh.delay_corners.len())
                .saturating_add(mesh.sampled_edges.len().saturating_mul(3))
                .saturating_mul(4),
        )?;
        let period = mesh.period;
        let mut corners = mesh.delay_corners.to_vec();
        let mut edges = mesh.sampled_edges.to_vec();
        let mut seen_edges = edges
            .iter()
            .map(|edge| edge.key())
            .collect::<std::collections::HashSet<_>>();
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
            for (index, edge) in mesh.sampled_edges.iter().enumerate() {
                if index & 0xff == 0 && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let offset = if edge.time >= period - phase_delay {
                    -(period - phase_delay)
                } else {
                    phase_delay
                };
                let edge = edge
                    .shifted(&[offset])
                    .periodic_boundary(mesh.boundary_event);
                if !seen_edges.contains(&edge.key()) {
                    self.ensure_analysis_points(edges.len().saturating_add(1))?;
                    self.ensure_result_values(
                        refined
                            .times
                            .len()
                            .saturating_add(corners.len())
                            .saturating_add(edges.len().saturating_add(1).saturating_mul(3))
                            .saturating_mul(4),
                    )?;
                    seen_edges.insert(edge.key());
                    edges.push(edge);
                }
            }
            for (index, &corner) in mesh.delay_corners.iter().enumerate() {
                if index & 0xff == 0 && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let arrival = if corner >= period - phase_delay {
                    PssIntegrationMesh::shifted_clock(
                        &mesh.delay_corners,
                        index,
                        &[-(period - phase_delay)],
                    )
                } else {
                    PssIntegrationMesh::shifted_clock(&mesh.delay_corners, index, &[phase_delay])
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
            refined.sampled_edges = edges.into();
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
        refined.boundary_event = mesh.boundary_event;
        refined.delay_corners = corners.into();
        refined.sampled_edges = edges.into();
        added.sort_by(Value::total_cmp);
        added.dedup();
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
            if events.is_empty() && enriched.sampled_edges == mesh.sampled_edges {
                return Ok(coarse);
            }
            let steps = circuit.integration_steps;
            let basis = circuit.delay_basis.clone();
            self.pss_set_reactive_state(circuit, &coarse.state.x0)?;
            circuit.delay_basis = basis.with_events(
                coarse.state.period,
                &events,
                &enriched,
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
        // A PWL jump has two values at one authored time. Keep the published
        // sample and the incoming/outgoing values at neighboring clocks, as
        // behavioral source event schedules already do for ideal steps.
        self.ensure_result_values(breakpoints.times().len().saturating_mul(8))?;
        let authored = breakpoints.times().to_vec();
        let mut source_edges = Vec::new();
        for (event, &time) in authored.iter().enumerate() {
            if event & 0xff == 0 && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            use crate::circuit::SourceTimeSide::{LeftLimit, Published, RightLimit};
            let mut before = false;
            let mut after = false;
            for index in 0..circuit.voltage_sources.names.len() {
                if index & 0xff == 0 && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                if !circuit.voltage_sources.source_specs[index]
                    .as_ref()
                    .is_some_and(is_pwl_source)
                {
                    continue;
                }
                let incoming = circuit
                    .voltage_sources
                    .transient_value_at_on_side(index, time, LeftLimit);
                let outgoing = circuit
                    .voltage_sources
                    .transient_value_at_on_side(index, time, RightLimit);
                if incoming != outgoing {
                    let published = circuit
                        .voltage_sources
                        .transient_value_at_on_side(index, time, Published);
                    before |= incoming != published;
                    after |= outgoing != published;
                }
            }
            for index in 0..circuit.current_sources.names.len() {
                if index & 0xff == 0 && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                if !circuit.current_sources.source_specs[index]
                    .as_ref()
                    .is_some_and(is_pwl_source)
                {
                    continue;
                }
                let incoming = circuit
                    .current_sources
                    .value_at_time_on_side(index, time, LeftLimit);
                let outgoing = circuit
                    .current_sources
                    .value_at_time_on_side(index, time, RightLimit);
                if incoming != outgoing {
                    let published = circuit
                        .current_sources
                        .value_at_time_on_side(index, time, Published);
                    before |= incoming != published;
                    after |= outgoing != published;
                }
            }
            if before && time > 0.0 && time < period {
                breakpoints.add(time.next_down());
            }
            if after && time > 0.0 && time < period {
                breakpoints.add(time.next_up());
            }
            if before || after {
                source_edges.push(SampledEdge {
                    time,
                    incoming: if before { time.next_down() } else { time },
                    outgoing: if after { time.next_up() } else { time },
                });
            }
            self.ensure_analysis_points(breakpoints.times().len())?;
        }
        drop(authored);
        // The source's pre-startup limit need not equal its periodic incoming
        // limit. Certify the repeated boundary at T, not its first DC-to-time
        // transition (a constant PWL must not create an orbit event).
        let boundary_event = source_edges.iter().any(|edge| edge.time == period);
        source_edges.retain(|edge| edge.time > 0.0 && edge.time < period);
        if boundary_event {
            source_edges.push(SampledEdge {
                time: 0.0,
                incoming: 0.0_f64.next_down(),
                outgoing: 0.0,
            });
        }
        // Seed the first periodic arrival of each prescribed corner. A delay
        // may span several carrier cycles; only its phase affects this clock,
        // whereas the shooting history still covers the entire physical TD.
        // Subsequent solved meshes extend these through reflections/cascades.
        self.ensure_result_values(
            breakpoints
                .times()
                .len()
                .saturating_mul(2)
                .saturating_add(source_edges.len().saturating_mul(9)),
        )?;
        let source_events = breakpoints.times().to_vec();
        let mut sampled_edges = source_edges.clone();
        let mut seen_edges = source_edges
            .iter()
            .map(|edge| edge.key())
            .collect::<std::collections::HashSet<_>>();
        for line in circuit
            .tlines
            .iter()
            .filter(|line| !line.is_memoryless_two_port())
        {
            let phase_delay = line.delay().rem_euclid(period);
            for (index, &edge) in source_edges.iter().enumerate() {
                if index & 0xff == 0 && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let offset = if edge.time >= period - phase_delay {
                    -(period - phase_delay)
                } else {
                    phase_delay
                };
                let edge = edge.shifted(&[offset]).periodic_boundary(boundary_event);
                for clock in [edge.incoming, edge.outgoing] {
                    if clock > 0.0 && clock < period {
                        breakpoints.add(clock);
                    }
                }
                if !seen_edges.contains(&edge.key()) {
                    self.ensure_analysis_points(sampled_edges.len().saturating_add(1))?;
                    self.ensure_result_values(
                        breakpoints.times().len().saturating_mul(3).saturating_add(
                            source_edges
                                .len()
                                .saturating_add(sampled_edges.len())
                                .saturating_add(1)
                                .saturating_mul(6),
                        ),
                    )?;
                    seen_edges.insert(edge.key());
                    sampled_edges.push(edge);
                }
            }
            for (index, &event) in source_events.iter().enumerate() {
                if index & 0xff == 0 && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                // Subtract first to avoid overflow near the largest period.
                let arrival = if event >= period - phase_delay {
                    PssIntegrationMesh::shifted_clock(
                        &source_events,
                        index,
                        &[-(period - phase_delay)],
                    )
                } else {
                    PssIntegrationMesh::shifted_clock(&source_events, index, &[phase_delay])
                };
                breakpoints.add(arrival);
                self.ensure_analysis_points(breakpoints.times().len())?;
            }
        }
        drop(source_events);
        drop(seen_edges);
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
        self.ensure_result_values(
            capacity
                .saturating_mul(2)
                .saturating_add(events.len())
                .saturating_add(
                    sampled_edges
                        .len()
                        .saturating_add(source_edges.len())
                        .saturating_mul(3),
                ),
        )?;
        let mut times = Vec::new();
        times.try_reserve_exact(capacity).map_err(|_| {
            PssError::InvalidConfig("source integration mesh allocation failed".to_owned())
        })?;
        let dt = period / steps as Value;
        let mut grid = 0;
        let mut event = 0;
        let mut changed = boundary_event;
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
            mesh.boundary_event = boundary_event;
            if circuit
                .tlines
                .iter()
                .any(|line| !line.is_memoryless_two_port())
            {
                mesh.delay_corners = delay_corners.into();
                mesh.sampled_edges = sampled_edges.into();
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
    fn pss_boundary_provenance_distinguishes_periodic_jumps_from_startup() {
        let engine = Engine::default();
        for (points, boundary) in [
            ("0 1 1 1", false),
            ("0 0 0 1 1 1", false),
            ("0 0 0 1 .5 1 .5 0 1 0", true),
        ] {
            let deck = Netlist::parse(&format!("boundary ownership\nV1 a 0 PWL({points}) R=0\nT1 a 0 b 0 Z0=50 TD=.1875\nR1 b 0 50\n.end\n")).unwrap();
            let circuit = engine.build_circuit(&deck).unwrap();
            let mesh = engine
                .pss_source_mesh(&circuit, &PssConfig::new(1.0), 16, &NoAbort)
                .unwrap();
            assert_eq!(
                mesh.as_ref()
                    .is_some_and(PssIntegrationMesh::has_boundary_event),
                boundary
            );
            if !boundary && let Some(mesh) = mesh {
                assert!(
                    mesh.pending_sampled_edges(0.1875, 0.0, Default::default(), &NoAbort)
                        .unwrap()
                        .is_empty()
                );
            }
        }
    }

    #[test]
    fn sampled_delay_incoming_coordinates_survive_projection_refinement_and_restore() {
        let engine = Engine::default();
        let deck = Netlist::parse(
            "Incoming coordinate\nR1 a 0 50\nR2 b 0 50\nT1 a 0 b 0 Z0=50 TD=1.1875\n.end\n",
        )
        .unwrap();
        let mut mesh =
            PssIntegrationMesh::from_times(1.0, vec![0.0, 0.5, 0.5_f64.next_up(), 1.0]).unwrap();
        let original = mesh.clone();
        mesh.sampled_edges = vec![SampledEdge {
            time: 0.5,
            incoming: 0.5,
            outgoing: 0.5_f64.next_up(),
        }]
        .into();
        let edges = mesh
            .pending_sampled_edges(1.1875, 0.0, Default::default(), &NoAbort)
            .unwrap();
        assert_eq!(edges.len(), 1);
        for enrich in [false, true] {
            let mut circuit = PssCircuit::new(engine.build_circuit(&deck).unwrap()).unwrap();
            circuit.integration_mesh = Some(if enrich {
                original.clone()
            } else {
                mesh.clone()
            });
            circuit
                .configure_delay_basis(1.0, 3, Default::default(), &NoAbort)
                .unwrap();
            if enrich {
                circuit.delay_basis = circuit
                    .delay_basis
                    .with_events(1.0, &[], &mesh, Default::default(), &NoAbort)
                    .unwrap();
            }
            circuit
                .set_state(&vec![0.0; circuit.state_dimension()])
                .unwrap();
            let values: Vec<_> = circuit.tlines[0]
                .checkpoint_state()
                .unwrap()
                .state_history
                .iter()
                .flat_map(|sample| {
                    let wave = if sample[0] <= edges[0][0] { 1.0 } else { 3.0 };
                    [wave, -wave]
                })
                .collect();
            circuit.set_state(&values).unwrap();
            let names = circuit.state_basis_names();
            let incoming = names
                .iter()
                .position(|name| name.ends_with(":in]"))
                .unwrap();
            assert_eq!(&values[incoming..incoming + 2], &[1.0, -1.0]);
            circuit.tlines[0]
                .promote_sampled_history_events(&edges)
                .unwrap();
            assert_eq!(circuit.extract_state(), values);
            let mut worker = circuit.clone();
            worker
                .restore_delay_basis(&names, Default::default())
                .unwrap();
            assert_eq!(worker.extract_state(), values);
            assert_eq!(worker.state_basis_names(), names);
            let mut malformed = names;
            malformed[incoming + 1] = malformed[incoming + 1].replace(":in]", "]");
            assert!(
                worker
                    .restore_delay_basis(&malformed, Default::default())
                    .is_err()
            );
            worker.delay_basis = worker
                .delay_basis
                .refined(Default::default(), &NoAbort)
                .unwrap();
            let refined = worker.extract_state();
            let names = worker.state_basis_names();
            let incoming = names
                .iter()
                .position(|name| name.ends_with(":in]"))
                .unwrap();
            assert_eq!(&refined[incoming..incoming + 2], &[1.0, -1.0]);
            worker.set_state(&refined).unwrap();
            assert_eq!(worker.extract_state(), refined);
            assert_eq!(circuit.extract_state(), values);
            worker.tlines[0]
                .accept_history_event(crate::device::TransmissionLineHistoryEvent {
                    time: 0.5,
                    incoming: [7.0, 0.0, -7.0, 0.0],
                    outgoing: [11.0, 0.0, -11.0, 0.0],
                    incoming_wave_slopes: [8.0, -8.0],
                    outgoing_wave_slopes: [0.0; 2],
                })
                .unwrap();
            worker.tlines[0].update_history(1.0, 11.0, 0.0, -11.0, 0.0);
            let advanced = worker.extract_state();
            assert_eq!(&advanced[incoming..incoming + 4], &[7.0, -7.0, 11.0, -11.0]);
        }
    }

    #[test]
    fn sampled_delay_boundary_anchors_survive_basis_creation_and_enrichment() {
        let engine = Engine::default();
        let deck = Netlist::parse(
            "Wrapped history\nR1 a 0 50\nR2 b 0 50\nT1 a 0 b 0 Z0=50 TD=1.1875\n.end\n",
        )
        .unwrap();
        let mut mesh = PssIntegrationMesh::from_times(1.0, vec![0.0, 0.5, 1.0]).unwrap();
        let original = mesh.clone();
        mesh.sampled_edges = vec![SampledEdge {
            time: 0.0,
            incoming: -Value::EPSILON / 2.0,
            outgoing: 0.0,
        }]
        .into();
        let edges = mesh
            .pending_sampled_edges(1.1875, 0.0, Default::default(), &NoAbort)
            .unwrap();
        assert_eq!(edges[0][1], (-1.0_f64).next_down());
        for enrich in [false, true] {
            let mut circuit = PssCircuit::new(engine.build_circuit(&deck).unwrap()).unwrap();
            circuit.integration_mesh = Some(if enrich {
                original.clone()
            } else {
                mesh.clone()
            });
            circuit
                .configure_delay_basis(1.0, 2, Default::default(), &NoAbort)
                .unwrap();
            if enrich {
                // Provenance can change even when all positive clocks exist.
                circuit.delay_basis = circuit
                    .delay_basis
                    .with_events(1.0, &[], &mesh, Default::default(), &NoAbort)
                    .unwrap();
            }
            circuit
                .set_state(&vec![0.0; circuit.state_dimension()])
                .unwrap();
            let history = circuit.tlines[0].checkpoint_state().unwrap();
            for &clock in edges.iter().flatten() {
                assert!(
                    history
                        .state_history
                        .iter()
                        .any(|sample| sample[0] == clock)
                );
            }
            circuit.tlines[0]
                .promote_sampled_history_events_with_endpoint_rates(&edges, Some([0.0; 2]))
                .unwrap();
            assert_eq!(
                circuit.tlines[0].checkpoint_state().unwrap().events.len(),
                2
            );
        }
    }

    #[test]
    fn sampled_delay_event_provenance_survives_refinement_and_full_history_windows() {
        let time = 0.375_f64;
        let mut mesh =
            PssIntegrationMesh::from_times(1.0, vec![0.0, time.next_down(), time, 0.75, 1.0])
                .unwrap();
        // An arbitrary adjacent pair has no physical event provenance.
        assert!(
            mesh.pending_sampled_edges(2.25, 1.0, Default::default(), &NoAbort)
                .unwrap()
                .is_empty()
        );
        mesh.sampled_edges = vec![SampledEdge {
            time,
            incoming: time.next_down(),
            outgoing: time,
        }]
        .into();
        mesh.delay_corners = vec![time.next_down(), time].into();
        let refined = mesh.refined(&NoAbort).unwrap().0;
        assert_eq!(&*refined.sampled_edges, &*mesh.sampled_edges);
        let events = refined
            .pending_sampled_edges(2.25, 1.0, Default::default(), &NoAbort)
            .unwrap();
        assert_eq!(
            events.iter().map(|e| e[0]).collect::<Vec<_>>(),
            vec![-0.625, 0.375]
        );
        assert_eq!(events[0][1], (-0.625_f64).next_down());
        let engine = Engine::default();
        let deck = Netlist::parse(
            "edge propagation\nR1 a 0 50\nR2 b 0 50\nT1 a 0 b 0 Z0=50 TD=.1875\n.end\n",
        )
        .unwrap();
        let circuit = engine.build_circuit(&deck).unwrap();
        let (propagated, _, _) = engine
            .pss_propagated_source_mesh(&mesh, &circuit, &NoAbort)
            .unwrap();
        assert!(
            propagated
                .sampled_edges
                .iter()
                .any(|edge| edge.time == 0.5625 && edge.incoming < edge.outgoing)
        );
        assert!(matches!(
            refined.pending_sampled_edges(
                2.25,
                1.0,
                Default::default(),
                &crate::abort_signal::CountingAbort::new(0)
            ),
            Err(SimulationError::Aborted)
        ));
        let mut limits = crate::ResourceLimits::default();
        limits.max_analysis_points = 1;
        assert!(matches!(
            refined.pending_sampled_edges(2.25, 1.0, limits, &NoAbort),
            Err(SimulationError::ResourceLimit(_))
        ));
    }

    #[test]
    fn shifted_delay_clocks_preserve_both_sides_of_ideal_edges() {
        for (clocks, offsets, expected) in [
            (
                [0.375_f64.next_down(), 0.375],
                vec![-1.0],
                [(-0.625_f64).next_down(), -0.625],
            ),
            (
                [-0.375, (-0.375_f64).next_up()],
                vec![1.0],
                [0.625, 0.625_f64.next_up()],
            ),
            (
                [0.375_f64.next_down(), 0.375],
                vec![-1.0, -1.0],
                [(-1.625_f64).next_down(), -1.625],
            ),
        ] {
            for index in 0..2 {
                assert_eq!(
                    PssIntegrationMesh::shifted_clock(&clocks, index, &offsets),
                    expected[index]
                );
            }
        }
        // Wrapped arrivals subtract the period first, avoiding overflow.
        let period = 1e308;
        assert_eq!(
            PssIntegrationMesh::shifted_clock(&[0.9 * period], 0, &[-period, 0.9 * period]),
            (0.9 * period - period) + 0.9 * period,
        );
    }

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
