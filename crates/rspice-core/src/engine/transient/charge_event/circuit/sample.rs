use super::*;
use crate::device::MatrixStamper;

fn branch(stamp: &mut EventStamp, state: &[Value], p: usize, n: usize, coefficient: Value) {
    let current = sum([
        (voltage(state, p), coefficient),
        (voltage(state, n), -coefficient),
    ]
    .into_iter())
    .unwrap_or(Value::NAN);
    for (row, sign) in [(p, 1.0), (n, -1.0)] {
        stamp.stamp_rhs(row, -sign * current);
        stamp.stamp(row, p, sign * coefficient);
        stamp.stamp(row, n, -sign * coefficient);
    }
}

fn current_port(stamp: &mut EventStamp, state: &[Value], p: usize, n: usize, column: usize) {
    for (row, sign) in [(p, 1.0), (n, -1.0)] {
        stamp.stamp_rhs(row, -sign * state[column]);
        stamp.stamp(row, column + 1, sign);
    }
}

impl PreparedEventCircuit<'_> {
    pub(in crate::engine::transient) fn sample(
        &mut self,
        time: Value,
        source_side: SourceTimeSide,
        state: &[Value],
        phase: &[Option<EventPhase<'_>>],
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<EventSample> {
        let retained = self.retained_structure_values();
        with_retained_values(options, retained, |bounded| {
            self.sample_inner(time, source_side, state, phase, bounded, abort)
        })
    }

    fn sample_inner(
        &mut self,
        time: Value,
        source_side: SourceTimeSide,
        state: &[Value],
        phase: &[Option<EventPhase<'_>>],
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<EventSample> {
        check_abort(abort)?;
        options.validate()?;
        ResourceLimitError::ensure(
            ResourceKind::MatrixUnknowns,
            state.len(),
            options.limits.max_matrix_unknowns,
        )?;
        let history_side = side(source_side)?;
        self.validate_state(state)?;
        if !time.is_finite() || time < 0.0 || phase.len() != self.models.len() {
            return Err(error("invalid event time or GP history population"));
        }
        let mut sample = EventSample::new(state.len(), options)?;
        let nodes = self.circuit.num_nodes();
        // Match ordinary transient F exactly: authored RSHUNT and the
        // selected dialect's conditioning floor are distinct retained owners.
        for index in 0..nodes {
            if index.is_multiple_of(64) {
                check_abort(abort)?;
            }
            if !self.circuit.is_non_electrical_state_matrix_index(index) {
                let physical = if self.circuit.has_global_shunt_at(index) {
                    self.circuit.global_shunt_conductance()
                } else {
                    0.0
                };
                let shunt = sum([(physical, 1.0), (options.nodal_gmin, 1.0)].into_iter())?;
                if shunt != 0.0 {
                    branch(&mut sample.f, state, index + 1, 0, shunt);
                }
            }
        }
        for (index, stamp) in self.circuit.resistors.stamps.iter().enumerate() {
            if index % 64 == 0 {
                check_abort(abort)?;
            }
            branch(
                &mut sample.f,
                state,
                stamp.pp.row,
                stamp.nn.row,
                self.circuit.resistors.conductances[index],
            );
        }
        for index in 0..self.circuit.vccs.len() {
            if index.is_multiple_of(64) {
                check_abort(abort)?;
            }
            let source = &self.circuit.vccs;
            source
                .stamp_physical_current(
                    index,
                    voltage(state, source.ctrl_pos[index]),
                    voltage(state, source.ctrl_neg[index]),
                    &mut sample.f,
                )
                .map_err(|failure| error(format!("VCCS '{}': {failure:?}", source.names[index])))?;
        }
        for index in 0..self.circuit.cccs.len() {
            check_abort(abort)?;
            let source = &self.circuit.cccs;
            let column = nodes + source.ctrl_branch[index] - 1;
            // Ideal-source currents have independent impulse and finite
            // coefficients in the prepared incidence. Other controlling MNA
            // currents remain ordinary constitutive coordinates here.
            if !self
                .current_structure
                .as_ref()
                .is_some_and(|basis| basis.source_columns[column])
            {
                source
                    .stamp_physical_current(index, state[column], nodes, &mut sample.f)
                    .map_err(|failure| {
                        error(format!("CCCS '{}': {failure:?}", source.names[index]))
                    })?;
            }
        }
        let line_side = match source_side {
            SourceTimeSide::LeftLimit => crate::device::TransmissionLineTimeSide::Incoming,
            SourceTimeSide::RightLimit => crate::device::TransmissionLineTimeSide::Outgoing,
            SourceTimeSide::Published => unreachable!("validated physical event side"),
        };
        for line in &self.circuit.tlines {
            check_abort(abort)?;
            if line.history_storage_values() == 0
                || time - line.delay() > line.accepted_history_time()
            {
                return Err(error(format!(
                    "transmission line '{}': physical event exceeds its accepted delay history",
                    line.name
                )));
            }
            for (p, n, forward) in [
                (line.node1_pos, line.node1_neg, false),
                (line.node2_pos, line.node2_neg, true),
            ] {
                let conductance = line.conductance();
                branch(&mut sample.f, state, p, n, conductance);
                let forcing = conductance * line.lossless_wave_on_side(time, forward, line_side);
                let forcing_rate = conductance
                    * line
                        .lossless_wave_slope_on_side(time, forward, line_side)
                        .map_err(error)?;
                for (row, sign) in [(p, 1.0), (n, -1.0)] {
                    sample.f.stamp_rhs(row, sign * forcing);
                    if row != 0 {
                        sample.f_time[row - 1] = sum([
                            (sample.f_time[row - 1], 1.0),
                            (forcing_rate, -sign),
                        ]
                        .into_iter())?;
                    }
                }
            }
        }
        for (index, stamp) in self.circuit.capacitors.stamps.iter().enumerate() {
            if index % 64 == 0 {
                check_abort(abort)?;
            }
            let p = stamp.pp.row;
            let n = stamp.nn.row;
            let c = self.circuit.capacitors.capacitances[index];
            if let Some(ordinal) = self.circuit.capacitors.ic_branch_indices[index] {
                let row = nodes + ordinal;
                if self.equations[ordinal - 1].is_none() {
                    // The linear descriptor retains the complete F matrix.
                    // The nonlinear event topology instead owns this current
                    // as an independent impulse/finite-current coordinate.
                    current_port(&mut sample.f, state, p, n, row - 1);
                    sample.f.stamp(row, row, 1.0);
                    sample.f.stamp_rhs(row, -state[row - 1]);
                }
                sample.q.stamp(row, p, -c);
                sample.q.stamp(row, n, c);
                sample.q.stamp_rhs(
                    row,
                    sum([(voltage(state, p), c), (voltage(state, n), -c)].into_iter())?,
                );
            } else {
                branch(&mut sample.q, state, p, n, c);
            }
        }
        let rb = &self.circuit.resistor_branches;
        let l = &self.circuit.inductors;
        for (magnetic, count) in [(false, rb.len()), (true, l.len())] {
            for index in 0..count {
                if index % 64 == 0 {
                    check_abort(abort)?;
                }
                let (p, n, column, value) = if magnetic {
                    (
                        l.node_pos[index],
                        l.node_neg[index],
                        nodes + l.branch_indices[index] - 1,
                        l.inductances[index],
                    )
                } else {
                    (
                        rb.node_pos[index],
                        rb.node_neg[index],
                        nodes + rb.branch_indices[index] - 1,
                        rb.resistances[index],
                    )
                };
                if value == 0.0 {
                    continue;
                } // Owned as an ideal zero-voltage branch.
                current_port(&mut sample.f, state, p, n, column);
                sample.f.stamp(column + 1, p, 1.0);
                sample.f.stamp(column + 1, n, -1.0);
                sample.f.stamp_rhs(
                    column + 1,
                    -sum([(voltage(state, p), 1.0), (voltage(state, n), -1.0)].into_iter())?,
                );
                let target = if magnetic {
                    &mut sample.q
                } else {
                    &mut sample.f
                };
                target.stamp(column + 1, column + 1, -value);
                target.stamp_rhs(column + 1, value * state[column]);
            }
        }
        for pair in &self.circuit.coupled_inductor_pairs {
            check_abort(abort)?;
            let a = nodes + pair.branch1_ordinal;
            let b = nodes + pair.branch2_ordinal;
            sample.q.stamp(a, b, -pair.device.m);
            sample.q.stamp(b, a, -pair.device.m);
            sample.q.stamp_rhs(a, pair.device.m * state[b - 1]);
            sample.q.stamp_rhs(b, pair.device.m * state[a - 1]);
        }
        let currents = &self.circuit.current_sources;
        for index in 0..currents.len() {
            if index % 64 == 0 {
                check_abort(abort)?;
            }
            let value = currents.value_at_time_on_side(index, time, source_side);
            let slope = currents
                .time_derivative_at_on_side(index, time, 1, source_side)
                .ok_or_else(|| {
                    error(format!(
                        "source '{}' has no regular current slope",
                        currents.names[index]
                    ))
                })?;
            for (row, sign) in [
                (currents.node_pos[index], 1.0),
                (currents.node_neg[index], -1.0),
            ] {
                sample.f.stamp_rhs(row, -sign * value);
                if row != 0 {
                    sample.f_time[row - 1] =
                        sum([(sample.f_time[row - 1], 1.0), (slope, sign)].into_iter())?;
                }
            }
        }
        for source in &self.circuit.behavioral_sources.current_sources {
            let (value, slope, partials) = if source.physical_time_program().is_some() {
                let [value, slope] = self.behavioral_time_values(
                    &source.name,
                    source.physical_time_program(),
                    time,
                    options,
                    abort,
                )?;
                (value, slope, Vec::new())
            } else {
                let sample = self.behavioral_sample(&source.name, time, options, |available| {
                    source.physical_sample(state, time, available, abort)
                })?;
                (sample.value, sample.time_partial, sample.partials)
            };
            for (row, sign) in [(source.node_pos, 1.0), (source.node_neg, -1.0)] {
                sample.f.stamp_rhs(row, -sign * value);
                for &(column, partial) in &partials {
                    sample.f.stamp(row, column + 1, sign * partial);
                }
                if row != 0 {
                    // Preserve numeric-domain failures for Newton backtracking.
                    // Stamp/index/quota faults still remain independently visible.
                    sample.f_time[row - 1] =
                        sum([(sample.f_time[row - 1], 1.0), (slope, sign)].into_iter())
                            .unwrap_or(Value::NAN);
                }
            }
        }
        for source in &self.circuit.behavioral_sources.voltage_sources {
            if source.physical_time_program().is_some() {
                continue; // Prescribed voltage equations belong to the topology.
            }
            let physical = self.behavioral_sample(&source.name, time, options, |available| {
                source.physical_sample(state, time, available, abort)
            })?;
            let row = nodes + source.branch_ordinal;
            // Keep each term's scale for the nonlinear voltage residual audit.
            sample.f.stamp_rhs(row, -voltage(state, source.node_pos));
            sample.f.stamp_rhs(row, voltage(state, source.node_neg));
            sample.f.stamp_rhs(row, physical.value);
            sample.f.stamp(row, source.node_pos, 1.0);
            sample.f.stamp(row, source.node_neg, -1.0);
            for (column, partial) in physical.partials {
                sample.f.stamp(row, column + 1, -partial);
            }
            sample.f_time[row - 1] = -physical.time_partial;
        }
        for diode in &self.circuit.diodes.devices {
            check_abort(abort)?;
            let p = diode.node_anode;
            let n = diode.node_cathode;
            let vd = voltage(state, p) - voltage(state, n);
            let (current, conductance) = diode.stamped_current_and_conductance(vd);
            let (charge, capacitance) = diode.junction_charge_and_capacitance(vd);
            for (stamp, value, slope) in [
                (&mut sample.f, current, conductance),
                (&mut sample.q, charge, capacitance),
            ] {
                for (row, sign) in [(p, 1.0), (n, -1.0)] {
                    stamp.stamp_rhs(row, -sign * value);
                    stamp.stamp(row, p, sign * slope);
                    stamp.stamp(row, n, -sign * slope);
                }
            }
        }
        for (index, (model, history)) in self.models.iter_mut().zip(phase).enumerate() {
            check_abort(abort)?;
            model.stamp_periodic_fq_with_forward_limit(
                state,
                &mut sample.f,
                &mut sample.q,
                source_side == SourceTimeSide::RightLimit && self.forward_charge_limits[index],
            );
            if let Some(column) = model.mna_rbi_branch_matrix_node(nodes) {
                current_port(
                    &mut sample.f,
                    state,
                    model.node_bx,
                    model.node_bi,
                    column - 1,
                );
            }
            let delay = model.legacy_excess_phase_delay();
            match (delay == 0.0, history) {
                (true, None) => {}
                (false, Some(history)) => {
                    stamp_phase(model, state, time, *history, history_side, &mut sample)?
                }
                _ => {
                    return Err(error(format!(
                        "BJT '{}' GP history does not match its phase model",
                        model.name
                    )));
                }
            }
        }
        // Preserve numeric-domain failure for the event Newton backtracker.
        // Structural and resource faults remain immediate errors.
        sample.nonfinite(state.len())?;
        check_abort(abort)?;
        Ok(sample)
    }
}

fn stamp_phase(
    model: &Bjt,
    state: &[Value],
    time: Value,
    phase: EventPhase<'_>,
    side: DelayTimeSide,
    sample: &mut EventSample,
) -> Result<()> {
    let failure = |detail: String| error(format!("BJT '{}': {detail}", model.name));
    let delay = model.legacy_excess_phase_delay();
    if phase.history.accepted_sample_count() == 0 || !phase.endpoint.is_finite() {
        return Err(failure(
            "event history requires an accepted anchor and a finite incoming input".into(),
        ));
    }
    let retained = phase
        .history
        .small_signal_delay(time, phase.endpoint, delay, None)
        .map_err(&failure)?;
    if retained.to_bits() != delay.to_bits() {
        return Err(failure(
            "event history belongs to another nominal delay".into(),
        ));
    }
    let mut branch = model
        .legacy_forward_transport_branch(&model.mna_internal_state_at_solution(state))
        .ok_or_else(|| failure("missing GP forward-current equation".into()))?;
    let evaluation = phase
        .history
        .difference_at_discontinuity_on_side(
            time,
            phase.endpoint,
            branch.current,
            delay,
            None,
            side,
        )
        .map_err(&failure)?;
    branch.current = evaluation.output;
    for derivative in branch.d_internal.iter_mut().chain(&mut branch.d_external) {
        *derivative = evaluation
            .apply_input_derivative(*derivative)
            .map_err(&failure)?;
    }
    let rate = phase
        .history
        .fixed_trajectory_slope(time, phase.endpoint, delay, side)
        .map_err(&failure)?;
    let terminals = [
        model.node_collector,
        model.node_base,
        model.node_emitter,
        model.node_substrate,
    ];
    let (positive, negative) = model
        .legacy_forward_transport_nodes()
        .ok_or_else(|| failure("missing prepared GP current port".into()))?;
    for (row, sign) in [(positive, 1.0), (negative, -1.0)] {
        sample.f.stamp_rhs(row, -sign * branch.current);
        for (index, &derivative) in branch.d_internal.iter().enumerate() {
            sample
                .f
                .stamp(row, model.mna_internal_node(index), sign * derivative);
        }
        for (index, &derivative) in branch.d_external.iter().enumerate() {
            sample.f.stamp(row, terminals[index], sign * derivative);
        }
        if row != 0 {
            sample.f_time[row - 1] =
                sum([(sample.f_time[row - 1], 1.0), (rate, sign)].into_iter())?;
        }
    }
    Ok(())
}
