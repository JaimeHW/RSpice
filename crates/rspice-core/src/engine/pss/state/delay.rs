//! Method-of-steps shooting coordinates on each line's full physical delay.
use super::*;

#[derive(Debug, Clone, Default)]
pub(in crate::engine::pss) struct PssDelayBasis {
    lines: Vec<LineCoordinates>,
    dimension: usize,
}

#[derive(Debug, Clone)]
struct LineCoordinates {
    index: usize,
    intervals: usize,
    delay: Value,
    /// Explicit physical clocks retain corners that a uniform projection loses.
    /// None preserves the compact legacy uniform-grid identity.
    knots: Option<std::sync::Arc<[Value]>>,
    /// A published incoming limit can share the event's exact clock. Its
    /// coordinate must remain incoming after sampled history becomes owned.
    incoming_knots: std::sync::Arc<[Value]>,
}

impl LineCoordinates {
    fn offset(&self, knot: usize) -> Value {
        if let Some(knots) = &self.knots {
            knots[knot]
        } else if knot == self.intervals {
            0.0
        } else {
            -self.delay * ((self.intervals - knot) as Value / self.intervals as Value)
        }
    }

    fn name(&self, line: &str, knot: usize, port: usize) -> String {
        match &self.knots {
            Some(knots) => format!(
                "W{port}:{line}[{knot}/{}@{:016x}{}]",
                self.intervals,
                knots[knot].to_bits(),
                if self.is_incoming(knots[knot]) {
                    ":in"
                } else {
                    ""
                },
            ),
            None => format!("W{port}:{line}[{knot}/{}]", self.intervals),
        }
    }

    fn is_incoming(&self, clock: Value) -> bool {
        self.incoming_knots
            .binary_search_by(|value| value.total_cmp(&clock))
            .is_ok()
    }

    fn with_incoming_edges(&mut self, edges: &[[Value; 3]]) {
        let mut incoming = self.incoming_knots.to_vec();
        incoming.extend(edges.iter().filter_map(|edge| {
            (edge[0] == edge[1] && edge[0] >= -self.delay && edge[0] <= 0.0)
                .then_some(if edge[0] == 0.0 { 0.0 } else { edge[0] })
        }));
        incoming.sort_by(Value::total_cmp);
        incoming.dedup();
        self.incoming_knots = incoming.into();
    }
}

impl PssDelayBasis {
    fn push(
        &mut self,
        line: LineCoordinates,
        limits: crate::ResourceLimits,
    ) -> Result<(), SimulationError> {
        self.dimension = self
            .dimension
            .saturating_add(2usize.saturating_mul(line.intervals.saturating_add(1)));
        crate::ResourceLimitError::ensure(
            crate::ResourceKind::AnalysisPoints,
            self.dimension,
            limits.max_analysis_points,
        )?;
        crate::ResourceLimitError::ensure(
            crate::ResourceKind::ResultValues,
            self.dimension
                .saturating_mul(self.dimension)
                .saturating_mul(4),
            limits.max_result_values,
        )?;
        if line.offset(0) != -line.delay || line.offset(line.intervals).to_bits() != 0 {
            return Err(SimulationError::Circuit(
                "PSS delay-state knots must span exactly [-TD, 0]".into(),
            ));
        }
        let mut previous = line.offset(0);
        for knot in 1..=line.intervals {
            let time = line.offset(knot);
            if !time.is_finite() || time <= previous {
                return Err(SimulationError::Circuit(
                    "PSS delay-state knot times cannot be represented distinctly".into(),
                ));
            }
            previous = time;
        }
        self.lines.push(line);
        Ok(())
    }

    pub(super) fn new(
        circuit: &CircuitData,
        period: Value,
        steps: usize,
        mesh: Option<&PssIntegrationMesh>,
        limits: crate::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        if !period.is_finite() || period <= 0.0 || steps == 0 {
            return Err(SimulationError::Circuit(
                "PSS delay-state grid requires a finite positive period and a nonzero step count"
                    .into(),
            ));
        }
        let mut basis = Self::default();
        for (index, line) in circuit.tlines.iter().enumerate() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if line.is_memoryless_two_port() {
                continue;
            }
            let count = (line.delay() / period * steps as Value).ceil().max(8.0);
            if !count.is_finite() || count >= (usize::MAX / 2 - 1) as Value {
                return Err(SimulationError::Circuit(format!(
                    "PSS line '{}' requires an unrepresentable delay-state grid",
                    line.name
                )));
            }
            let intervals = count as usize;
            let mut coordinates = LineCoordinates {
                index,
                intervals,
                delay: line.delay(),
                knots: None,
                incoming_knots: std::sync::Arc::from([]),
            };
            if let Some(mesh) = mesh {
                let edges = mesh.pending_sampled_edges(line.delay(), 0.0, limits, abort)?;
                let cycles = (line.delay() / period).ceil() as usize;
                let capacity = cycles
                    .saturating_mul(mesh.steps())
                    .saturating_add(intervals)
                    .saturating_add(2)
                    .saturating_add(edges.len().saturating_mul(3));
                let dimension = basis
                    .dimension
                    .saturating_add(2usize.saturating_mul(capacity));
                crate::ResourceLimitError::ensure(
                    crate::ResourceKind::AnalysisPoints,
                    dimension,
                    limits.max_analysis_points,
                )?;
                crate::ResourceLimitError::ensure(
                    crate::ResourceKind::ResultValues,
                    dimension.saturating_mul(dimension).saturating_mul(4),
                    limits.max_result_values,
                )?;
                let mut knots = Vec::new();
                knots.try_reserve_exact(capacity).map_err(|_| {
                    SimulationError::Circuit("PSS delay-state allocation failed".into())
                })?;
                knots.extend((0..=intervals).map(|knot| coordinates.offset(knot)));
                // A wrapped edge can own an incoming anchor just before zero,
                // outside the positive integration mesh. Retain its declared
                // clocks explicitly instead of reconstructing them from phases.
                knots.extend(
                    edges
                        .iter()
                        .flatten()
                        .copied()
                        .filter(|time| *time > -line.delay() && *time < 0.0),
                );
                for cycle in 1..=cycles {
                    if abort.is_aborted() {
                        return Err(SimulationError::Aborted);
                    }
                    let previous_cycles = (cycle - 1) as Value * period;
                    for index in 0..mesh.steps() {
                        if index & 0xff == 0 && abort.is_aborted() {
                            return Err(SimulationError::Aborted);
                        }
                        // A full cycle*period can overflow even though this
                        // partial last cycle still intersects the delay window.
                        let time = PssIntegrationMesh::shifted_clock(
                            mesh.times(),
                            index,
                            &[-period, -previous_cycles],
                        );
                        if time > -line.delay() && time < 0.0 {
                            knots.push(time);
                        }
                    }
                }
                knots.sort_by(Value::total_cmp);
                knots.dedup();
                coordinates.intervals = knots.len() - 1;
                coordinates.knots = Some(knots.into());
                coordinates.with_incoming_edges(&edges);
            }
            basis.push(coordinates, limits)?;
        }
        Ok(basis)
    }

    pub(in crate::engine::pss) fn refined(
        &self,
        limits: crate::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        let mut basis = Self::default();
        for coordinates in &self.lines {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let intervals = coordinates.intervals.checked_mul(2).ok_or_else(|| {
                SimulationError::Circuit("PSS delay-state refinement overflowed".into())
            })?;
            let mut refined = LineCoordinates {
                intervals,
                ..coordinates.clone()
            };
            if let Some(knots) = &coordinates.knots {
                let dimension = basis
                    .dimension
                    .saturating_add(2usize.saturating_mul(intervals.saturating_add(1)));
                crate::ResourceLimitError::ensure(
                    crate::ResourceKind::AnalysisPoints,
                    dimension,
                    limits.max_analysis_points,
                )?;
                crate::ResourceLimitError::ensure(
                    crate::ResourceKind::ResultValues,
                    dimension.saturating_mul(dimension).saturating_mul(4),
                    limits.max_result_values,
                )?;
                let mut times = Vec::new();
                times.try_reserve_exact(intervals + 1).map_err(|_| {
                    SimulationError::Circuit("PSS delay-state refinement allocation failed".into())
                })?;
                for (index, pair) in knots.windows(2).enumerate() {
                    if index & 0xff == 0 && abort.is_aborted() {
                        return Err(SimulationError::Aborted);
                    }
                    times.push(pair[0]);
                    if let Some(midpoint) =
                        PssIntegrationMesh::refinement_midpoint(pair[0], pair[1])
                    {
                        times.push(midpoint);
                    }
                }
                times.push(0.0);
                refined.intervals = times.len() - 1;
                refined.knots = Some(times.into());
            }
            basis.push(refined, limits)?;
        }
        Ok(basis)
    }

    /// Keep independently resolved history while adding newly discovered
    /// periodic event clocks throughout every physical delay window.
    pub(in crate::engine::pss) fn with_events(
        &self,
        period: Value,
        events: &[Value],
        mesh: &PssIntegrationMesh,
        limits: crate::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        let mut basis = Self::default();
        for coordinates in &self.lines {
            let edges = mesh.pending_sampled_edges(coordinates.delay, 0.0, limits, abort)?;
            let cycles = (coordinates.delay / period).ceil() as usize;
            let capacity = coordinates
                .intervals
                .saturating_add(1)
                .saturating_add(cycles.saturating_mul(events.len()))
                .saturating_add(edges.len().saturating_mul(3));
            let dimension = basis
                .dimension
                .saturating_add(2usize.saturating_mul(capacity));
            crate::ResourceLimitError::ensure(
                crate::ResourceKind::AnalysisPoints,
                dimension,
                limits.max_analysis_points,
            )?;
            crate::ResourceLimitError::ensure(
                crate::ResourceKind::ResultValues,
                dimension.saturating_mul(dimension).saturating_mul(4),
                limits.max_result_values,
            )?;
            let mut knots = Vec::new();
            knots.try_reserve_exact(capacity).map_err(|_| {
                SimulationError::Circuit("PSS delay event allocation failed".into())
            })?;
            knots.extend((0..=coordinates.intervals).map(|knot| coordinates.offset(knot)));
            knots.extend(
                edges
                    .iter()
                    .flatten()
                    .copied()
                    .filter(|time| *time > -coordinates.delay && *time < 0.0),
            );
            for cycle in 1..=cycles {
                for index in 0..events.len() {
                    if index & 0xff == 0 && abort.is_aborted() {
                        return Err(SimulationError::Aborted);
                    }
                    let time = PssIntegrationMesh::shifted_clock(
                        events,
                        index,
                        &[-period, -((cycle - 1) as Value * period)],
                    );
                    if time > -coordinates.delay && time < 0.0 {
                        knots.push(time);
                    }
                }
            }
            knots.sort_by(Value::total_cmp);
            knots.dedup();
            let mut enriched = LineCoordinates {
                intervals: knots.len() - 1,
                knots: Some(knots.into()),
                ..coordinates.clone()
            };
            enriched.with_incoming_edges(&edges);
            basis.push(enriched, limits)?;
        }
        Ok(basis)
    }

    /// The authenticated retained basis names carry the selected history grid.
    /// It can be finer than the initial time mesh, so reconstruct its actual
    /// knot count rather than guessing it from the configured carrier period.
    fn from_names(
        circuit: &CircuitData,
        names: &[String],
        limits: crate::ResourceLimits,
    ) -> Result<Self, SimulationError> {
        let mut basis = Self::default();
        for (index, line) in circuit.tlines.iter().enumerate() {
            if line.is_memoryless_two_port() {
                continue;
            }
            let prefix = format!("W1:{}[0/", line.name);
            let intervals = names
                .get(basis.dimension)
                .and_then(|name| name.strip_prefix(&prefix))
                .and_then(|name| name.strip_suffix(']'))
                .and_then(|count| count.split('@').next()?.parse::<usize>().ok())
                .filter(|count| *count >= 8)
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "retained PSS delay-state grid does not match line '{}'",
                        line.name
                    ))
                })?;
            let mut coordinates = LineCoordinates {
                index,
                intervals,
                delay: line.delay(),
                knots: None,
                incoming_knots: std::sync::Arc::from([]),
            };
            let dimension = basis
                .dimension
                .saturating_add(2usize.saturating_mul(intervals.saturating_add(1)));
            crate::ResourceLimitError::ensure(
                crate::ResourceKind::AnalysisPoints,
                dimension,
                limits.max_analysis_points,
            )?;
            crate::ResourceLimitError::ensure(
                crate::ResourceKind::ResultValues,
                dimension.saturating_mul(dimension).saturating_mul(4),
                limits.max_result_values,
            )?;
            if names[basis.dimension]
                .strip_prefix(&prefix)
                .is_some_and(|field| field.contains('@'))
            {
                let count = intervals
                    .checked_add(1)
                    .and_then(|n| n.checked_mul(2))
                    .ok_or_else(|| {
                        SimulationError::Circuit("retained PSS delay-state size overflowed".into())
                    })?;
                let entries = names
                    .get(basis.dimension..)
                    .and_then(|tail| tail.get(..count))
                    .ok_or_else(|| {
                        SimulationError::Circuit(
                            "retained PSS delay-state grid is truncated".into(),
                        )
                    })?;
                let mut knots = Vec::new();
                let mut incoming = Vec::new();
                knots.try_reserve_exact(intervals + 1).map_err(|_| {
                    SimulationError::Circuit("retained PSS delay-state allocation failed".into())
                })?;
                for pair in entries.chunks_exact(2) {
                    let field = pair[0]
                        .rsplit_once('@')
                        .and_then(|(_, bits)| bits.strip_suffix(']'))
                        .ok_or_else(|| {
                            SimulationError::Circuit(
                                "retained PSS delay-state clock is invalid".into(),
                            )
                        })?;
                    let bits = u64::from_str_radix(field.strip_suffix(":in").unwrap_or(field), 16)
                        .map_err(|_| {
                            SimulationError::Circuit(
                                "retained PSS delay-state clock is invalid".into(),
                            )
                        })?;
                    let clock = Value::from_bits(bits);
                    knots.push(clock);
                    if field.ends_with(":in") {
                        incoming.push(clock);
                    }
                }
                coordinates.knots = Some(knots.into());
                coordinates.incoming_knots = incoming.into();
            }
            basis.push(coordinates, limits)?;
        }
        if basis.names(circuit) != names {
            return Err(SimulationError::Circuit(
                "retained PSS delay-state coordinates are inconsistent".into(),
            ));
        }
        Ok(basis)
    }

    pub(super) fn dimension(&self) -> usize {
        self.dimension
    }

    pub(super) fn names(&self, circuit: &CircuitData) -> Vec<String> {
        self.lines
            .iter()
            .flat_map(|coordinates| {
                (0..=coordinates.intervals).flat_map(move |knot| {
                    [1, 2].map(|port| {
                        coordinates.name(&circuit.tlines[coordinates.index].name, knot, port)
                    })
                })
            })
            .collect()
    }

    pub(super) fn extract(&self, circuit: &CircuitData) -> Vec<Value> {
        self.lines
            .iter()
            .flat_map(|coordinates| {
                let line = &circuit.tlines[coordinates.index];
                (0..=coordinates.intervals).flat_map(move |knot| {
                    let time = if let Some(knots) = &coordinates.knots {
                        PssIntegrationMesh::shifted_clock(
                            knots,
                            knot,
                            &[line.accepted_history_time()],
                        )
                    } else {
                        line.accepted_history_time() + coordinates.offset(knot)
                    };
                    let side = if coordinates.is_incoming(coordinates.offset(knot)) {
                        crate::device::TransmissionLineTimeSide::Incoming
                    } else {
                        crate::device::TransmissionLineTimeSide::Outgoing
                    };
                    [true, false].map(|forward| line.lossless_wave_at_on_side(time, forward, side))
                })
            })
            .collect()
    }

    pub(super) fn set(
        &self,
        circuit: &mut CircuitData,
        values: &[Value],
    ) -> Result<(), SimulationError> {
        if values.len() != self.dimension || values.iter().any(|value| !value.is_finite()) {
            return Err(SimulationError::Circuit(
                "PSS delay state has an invalid shape or value".into(),
            ));
        }
        let mut cursor = 0;
        for coordinates in &self.lines {
            let line = &mut circuit.tlines[coordinates.index];
            line.reset();
            for knot in 0..=coordinates.intervals {
                // Only the outgoing waves are independent DDE coordinates.
                // This canonical V=wave/I=0 embedding supplies the native
                // interpolator; accepted future samples carry actual V and I.
                line.update_history(
                    coordinates.offset(knot),
                    values[cursor],
                    0.0,
                    values[cursor + 1],
                    0.0,
                );
                cursor += 2;
            }
        }
        Ok(())
    }
}

impl PssCircuit {
    pub(in crate::engine) fn restore_delay_basis(
        &mut self,
        names: &[String],
        limits: crate::ResourceLimits,
    ) -> Result<(), SimulationError> {
        let prefix = self.physical_state_dimension()
            + self.behavioral_sources.integral_count()
            + self.capacitors.integral_count();
        let names = names.get(prefix..).ok_or_else(|| {
            SimulationError::Circuit(
                "retained PSS basis omitted physical/integral coordinates".into(),
            )
        })?;
        self.delay_basis = PssDelayBasis::from_names(&self.circuit, names, limits)?;
        Ok(())
    }

    pub(in crate::engine) fn configure_delay_basis(
        &mut self,
        period: Value,
        steps: usize,
        limits: crate::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        self.delay_basis = PssDelayBasis::new(
            &self.circuit,
            period,
            steps,
            self.integration_mesh.as_ref(),
            limits,
            abort,
        )?;
        Ok(())
    }

    pub(in crate::engine::pss) fn accept_delay_history(&mut self, solution: &[Value], time: Value) {
        for line in &mut self.circuit.tlines {
            if line.is_memoryless_two_port() {
                continue;
            }
            let node = |node: usize| if node == 0 { 0.0 } else { solution[node - 1] };
            let voltage = |positive, negative| node(positive) - node(negative);
            let v1 = voltage(line.node1_pos, line.node1_neg);
            let v2 = voltage(line.node2_pos, line.node2_neg);
            let (i1, i2) = if let Some((one, two)) = line.ltra_branch_matrix_indices() {
                (solution[one - 1], solution[two - 1])
            } else {
                line.transient_port_response(time).port_currents(v1, v2)
            };
            line.update_history(time, v1, i1, v2, i2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NoAbort;

    #[test]
    fn pss_delay_state_retains_corner_clocks_and_portable_basis() {
        let engine = Engine::new(Default::default());
        let deck = Netlist::parse("Corner state\nVIN in 0 SIN(0 1 1)\nRS in near 50\nT1 near 0 far 0 Z0=50 TD=1.137\nRL far 0 50\n.end\n").unwrap();
        let mut circuit = PssCircuit::new(engine.build_circuit(&deck).unwrap()).unwrap();
        let times = vec![0.0, 0.017, 0.71, 0.76, 0.83, 0.88, 1.0];
        circuit.integration_mesh =
            Some(PssIntegrationMesh::from_times(1.0, times.clone()).unwrap());
        circuit
            .configure_delay_basis(1.0, 6, Default::default(), &NoAbort)
            .unwrap();
        let original = circuit.delay_basis.clone();
        let knots = original.lines[0].knots.as_ref().unwrap();
        for &time in &times[..times.len() - 1] {
            assert!(knots.contains(&(time - 1.0)));
        }
        assert!(knots.contains(&((0.88 - 1.0) - 1.0)));
        let values: Vec<_> = (0..circuit.state_dimension())
            .map(|k| (k as Value * 0.21).sin())
            .collect();
        circuit.set_state(&values).unwrap();
        let names = circuit.state_basis_names();
        let mut worker = circuit.clone();
        worker
            .restore_delay_basis(&names, Default::default())
            .unwrap();
        assert_eq!(worker.state_basis_names(), names);
        assert_eq!(worker.extract_state(), values);
        worker.delay_basis = worker
            .delay_basis
            .refined(Default::default(), &NoAbort)
            .unwrap();
        let fine_knots = worker.delay_basis.lines[0].knots.as_ref().unwrap();
        for &knot in knots.iter() {
            assert!(
                fine_knots
                    .iter()
                    .any(|value| value.to_bits() == knot.to_bits())
            );
        }
        let refined = worker.extract_state();
        worker.set_state(&refined).unwrap();
        assert_eq!(worker.extract_state(), refined);
        let mut malformed = names.clone();
        malformed[0] = malformed[0].replace(
            &format!("{:016x}", (-1.137_f64).to_bits()),
            &format!("{:016x}", (-1.138_f64).to_bits()),
        );
        assert!(
            worker
                .restore_delay_basis(&malformed, Default::default())
                .is_err()
        );
        let mut limits = crate::ResourceLimits::default();
        limits.max_result_values = 100;
        assert!(worker.restore_delay_basis(&names, limits).is_err());
        assert!(matches!(
            original.refined(
                Default::default(),
                &crate::abort_signal::CountingAbort::new(0)
            ),
            Err(SimulationError::Aborted)
        ));
    }

    #[test]
    fn pss_delay_state_preserves_nonperiodic_memory_and_worker_isolation() {
        let engine = Engine::new(Default::default());
        let netlist = Netlist::parse(
            "Delay state\nVIN in 0 SIN(0.3 0.8 1)\nRS in near 50\nT1 near 0 far 0 Z0=50 TD=2\nRL far 0 50\n.end\n"
        ).unwrap();
        let data = engine.build_circuit(&netlist).unwrap();
        let mut circuit = PssCircuit::new(data).unwrap();
        circuit
            .configure_delay_basis(1.0, 8, Default::default(), &NoAbort)
            .unwrap();
        assert_eq!(circuit.state_dimension(), 34);
        let values: Vec<_> = (0..17)
            .flat_map(|k| [1.0 + 0.01 * (k * k) as Value, -0.4 + 0.07 * k as Value])
            .collect();
        circuit.set_state(&values).unwrap();
        assert_eq!(circuit.extract_state(), values);
        let mut worker = circuit.clone();
        worker.set_state(&vec![0.0; 34]).unwrap();
        assert_eq!(circuit.extract_state(), values);
        assert_eq!(circuit.perturbation_scale(33, 0.0, 1e-6, 1e-8), 1.0);

        let mut matrix = engine.build_matrix(&circuit).unwrap();
        circuit.link_indices(&matrix);
        let seed = engine
            .pss_initial_node_solution(&mut circuit, &NoAbort)
            .unwrap();
        engine
            .pss_run_tran_internal(
                &mut circuit,
                &mut matrix,
                seed,
                PssTraversal {
                    tstop: 1.0,
                    max_step: 0.125,
                    fixed_grid: true,
                    integration_method: None,
                    retain_waveform: false,
                },
                None,
                &NoAbort,
            )
            .unwrap();
        let final_state = circuit.extract_state();
        // The first half of a two-period delay is still the independently
        // supplied history. Wrapping one carrier cycle would fail this check.
        for k in 0..=8 {
            assert_eq!(final_state[2 * k], values[2 * (k + 8)]);
            assert_eq!(final_state[2 * k + 1], values[2 * (k + 8) + 1]);
        }
        for k in 9..=16 {
            let time = (k - 8) as Value / 8.0;
            let expected = 0.3 + 0.8 * (std::f64::consts::TAU * time).sin();
            assert!((final_state[2 * k] - expected).abs() < 1e-8);
            assert!(final_state[2 * k + 1].abs() < 1e-8);
        }
        let reference = circuit.clone();
        circuit.tlines[0].rebase_lossless_history(0.0).unwrap();
        assert_eq!(circuit.extract_state(), final_state);
        for time in [0.125, 0.375, 0.875] {
            let original = reference.tlines[0].transient_port_response(time + 1.0);
            let rebased = circuit.tlines[0].transient_port_response(time);
            assert_eq!(original.i_eq_port1(), rebased.i_eq_port1());
            assert_eq!(original.i_eq_port2(), rebased.i_eq_port2());
        }
        circuit.set_state(&values).unwrap();
        circuit.delay_basis = circuit
            .delay_basis
            .refined(Default::default(), &NoAbort)
            .unwrap();
        let refined = circuit.extract_state();
        for knot in 0..17 {
            assert_eq!(
                &refined[4 * knot..4 * knot + 2],
                &values[2 * knot..2 * knot + 2]
            );
        }
        let names = circuit.state_basis_names();
        worker
            .restore_delay_basis(&names, Default::default())
            .unwrap();
        assert_eq!(worker.state_basis_names(), names);
        worker.set_state(&refined).unwrap();
        assert_eq!(worker.extract_state(), refined);
        let mut malformed = names;
        *malformed.last_mut().unwrap() = "W2:T1[wrong]".into();
        assert!(
            worker
                .restore_delay_basis(&malformed, Default::default())
                .is_err()
        );
        let mut limits = crate::ResourceLimits::default();
        limits.max_result_values = 100;
        assert!(circuit.delay_basis.refined(limits, &NoAbort).is_err());
        assert!(
            circuit
                .configure_delay_basis(0.0, 8, Default::default(), &NoAbort)
                .is_err()
        );
    }
}
