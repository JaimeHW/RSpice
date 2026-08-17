//! Source and transmission-line breakpoint scheduling helpers.

use super::*;

impl Engine {
    #[inline]
    pub(super) fn max_expected_source_delta(
        circuit: &crate::circuit::CircuitData,
        t0: Value,
        t1: Value,
    ) -> Value {
        circuit
            .voltage_sources
            .max_expected_delta(t0, t1)
            .max(circuit.current_sources.max_expected_delta(t0, t1))
    }

    #[inline]
    pub(super) fn startup_source_activity_delta_for_retry_floor(
        circuit: &crate::circuit::CircuitData,
        time: Value,
        attempted_dt: Value,
        tstop: Value,
        initial_timestep: Value,
        preferred_min_timestep: Value,
    ) -> Value {
        let retry_floor = initial_timestep.min(preferred_min_timestep);
        let activity_horizon = if retry_floor.is_finite() && retry_floor > 0.0 {
            attempted_dt.max(retry_floor)
        } else {
            attempted_dt
        };
        let remaining = (tstop - time).max(0.0);
        let activity_horizon = activity_horizon.min(remaining);

        if activity_horizon.is_finite() && activity_horizon > 0.0 {
            Self::max_expected_source_delta(circuit, time, time + activity_horizon)
        } else {
            Self::max_expected_source_delta(circuit, time, time + attempted_dt)
        }
    }

    #[inline]
    pub(super) fn add_breakpoint_if_in_range(
        breakpoints: &mut BreakpointManager,
        time: Value,
        tstop: Value,
    ) {
        if time.is_finite() && time >= 0.0 && time <= tstop {
            breakpoints.add(time);
        }
    }

    pub(super) fn collect_xspice_runtime_breakpoints(
        circuit: &mut crate::circuit::CircuitData,
        breakpoints: &mut BreakpointManager,
        tstop: Value,
    ) {
        let mut runtime_breakpoints = Vec::new();
        if let Some(event_time) = circuit.next_xspice_event_time() {
            if event_time.is_finite() && event_time >= 0.0 && event_time <= tstop {
                runtime_breakpoints.push(event_time);
            }
        }
        circuit.drain_xspice_requested_breakpoints(|time| {
            if time.is_finite() && time >= 0.0 && time <= tstop {
                runtime_breakpoints.push(time);
            }
        });
        breakpoints.replace_runtime_breakpoints(runtime_breakpoints);
    }

    #[cfg(test)]
    pub(super) fn add_source_spec_breakpoints(
        breakpoints: &mut BreakpointManager,
        spec: &crate::netlist::SourceSpec,
        tstop: Value,
        tstep_hint: Value,
        dialect: crate::engine::SpiceDialect,
    ) {
        Self::add_source_spec_breakpoints_with_pwl(
            breakpoints,
            spec,
            None,
            tstop,
            tstep_hint,
            dialect,
            &crate::abort_signal::NoAbort,
            usize::MAX,
        )
        .expect("unbounded non-cancellable breakpoint collection cannot fail");
    }

    fn add_source_spec_breakpoints_with_pwl(
        breakpoints: &mut BreakpointManager,
        spec: &crate::netlist::SourceSpec,
        pwl_waveform: Option<&crate::device::pwl_file::PwlWaveform>,
        tstop: Value,
        tstep_hint: Value,
        dialect: crate::engine::SpiceDialect,
        abort: &dyn crate::abort_signal::AbortSignal,
        max_points: usize,
    ) -> Result<(), crate::engine::SimulationError> {
        use crate::netlist::SourceSpec;

        match spec {
            SourceSpec::Distortion { inner, .. } => {
                Self::add_source_spec_breakpoints_with_pwl(
                    breakpoints,
                    inner,
                    pwl_waveform,
                    tstop,
                    tstep_hint,
                    dialect,
                    abort,
                    max_points,
                )?;
            }
            SourceSpec::RfPort { inner, .. } => {
                Self::add_source_spec_breakpoints_with_pwl(
                    breakpoints,
                    inner,
                    pwl_waveform,
                    tstop,
                    tstep_hint,
                    dialect,
                    abort,
                    max_points,
                )?;
            }
            // TRNOISE breakpoints come from its expanded PWL sample train;
            // the unexpanded spec itself schedules none.
            SourceSpec::Dc(_)
            | SourceSpec::Ac { .. }
            | SourceSpec::DcAc { .. }
            | SourceSpec::TrNoise { .. }
            | SourceSpec::TrRandom { .. } => {}
            SourceSpec::DcTransient { transient, .. }
            | SourceSpec::DcAcTransient { transient, .. } => {
                Self::add_source_spec_breakpoints_with_pwl(
                    breakpoints,
                    transient,
                    pwl_waveform,
                    tstop,
                    tstep_hint,
                    dialect,
                    abort,
                    max_points,
                )?;
            }
            SourceSpec::Pulse {
                delay,
                rise,
                fall,
                width,
                period,
                pulse_count,
                width_defaults_to_zero,
                ..
            } => {
                // Same resolution as the waveform runtime, so breakpoints
                // land exactly on the edges the source actually produces.
                let (td, tr, tf, pw, per) =
                    crate::circuit::VoltageSources::resolve_pulse_timing_with_defaults(
                        *delay,
                        *rise,
                        *fall,
                        *width,
                        *period,
                        *width_defaults_to_zero,
                        tstep_hint.max(1e-18),
                        tstop.max(1e-18),
                        dialect,
                    );

                let per_valid = per.is_finite() && per > 0.0;
                // Xyce applies its modulo path for every nonzero period.  A
                // negative period maps every nonnegative simulation time to a
                // nonpositive local time, so PulseData remains at V1 and has
                // no physical source edges to schedule.
                if dialect == crate::engine::SpiceDialect::Xyce && per.is_finite() && per < 0.0 {
                    return Self::check_source_breakpoint_collection(
                        breakpoints,
                        abort,
                        max_points,
                    );
                }
                let edge_offsets = [0.0, tr, tr + pw, tr + pw + tf];
                let minimum_offset = edge_offsets
                    .iter()
                    .copied()
                    .fold(Value::INFINITY, Value::min);
                let maximum_offset = edge_offsets
                    .iter()
                    .copied()
                    .fold(Value::NEG_INFINITY, Value::max);
                let base_cycle_start = td;
                let first_cycle_start = if per_valid {
                    let earliest_relevant_start = -maximum_offset;
                    if base_cycle_start >= earliest_relevant_start {
                        base_cycle_start
                    } else {
                        let phase = (base_cycle_start - earliest_relevant_start).rem_euclid(per);
                        earliest_relevant_start + phase
                    }
                } else {
                    base_cycle_start
                };
                // A bounded train produces no edges past its final period, so
                // scheduling them would force timesteps at times where the
                // waveform is flat.
                let train_end = if per_valid && *pulse_count > 0.0 {
                    td + pulse_count * per
                } else {
                    Value::INFINITY
                };
                let last_relevant_start = (tstop - minimum_offset).min(train_end);
                let mut previous_cycle_start = None;

                // Test each computed cycle time directly instead of deriving
                // a floating count. Decimal inputs can make
                // floor((last-first)/PER) one too small even when the final
                // fused cycle start is representably inside the interval.
                for cycle in 0..=1_000_000_usize {
                    if cycle % 1024 == 0 {
                        Self::check_source_breakpoint_collection(breakpoints, abort, max_points)?;
                    }
                    let cycle_start = if per_valid {
                        per.mul_add(cycle as Value, first_cycle_start)
                    } else {
                        first_cycle_start
                    };
                    if !cycle_start.is_finite() {
                        return Err(crate::engine::SimulationError::Circuit(format!(
                            "transient PULSE source-event cycle {cycle} is not finite"
                        )));
                    }
                    if !last_relevant_start.is_finite() || cycle_start > last_relevant_start {
                        break;
                    }
                    if cycle == 1_000_000 {
                        return Err(crate::engine::SimulationError::Circuit(
                            "transient source-event schedule exceeds the exact enumeration limit of 1000000 PULSE cycles"
                                .to_string(),
                        ));
                    }
                    if let Some(previous) = previous_cycle_start {
                        if cycle_start <= previous {
                            return Err(crate::engine::SimulationError::Circuit(format!(
                                "transient PULSE period {per:.17e} cannot advance an exactly representable source-event schedule near {previous:.17e}"
                            )));
                        }
                    }
                    previous_cycle_start = Some(cycle_start);
                    for offset in edge_offsets {
                        Self::add_breakpoint_if_in_range(breakpoints, cycle_start + offset, tstop);
                    }
                    if !per_valid {
                        break;
                    }
                }
            }
            SourceSpec::Sin { delay, .. } => {
                // Xyce 7.10 SinData inherits SourceData's no-op breakpoint
                // implementation.  Its delay remains part of the waveform,
                // but it is not registered with StepErrorControl.
                if dialect != crate::engine::SpiceDialect::Xyce {
                    Self::add_breakpoint_if_in_range(breakpoints, *delay, tstop);
                }
            }
            SourceSpec::Pwl {
                points,
                delay,
                repeat_from,
            } => {
                let times = points.iter().map(|(time, _)| *time + *delay);
                Self::add_repeating_pwl_breakpoints(
                    breakpoints,
                    times,
                    *repeat_from,
                    *delay,
                    tstop,
                    abort,
                    max_points,
                )?;
            }
            SourceSpec::PwlFile {
                path,
                time_scale,
                value_scale,
                time_offset,
                value_offset,
                delay,
                repeat_from,
            } => match pwl_waveform {
                Some(wf) => {
                    Self::add_repeating_pwl_breakpoints(
                        breakpoints,
                        wf.scaled_knot_times().map(|time| time + *delay),
                        repeat_from.map(|value| value * *time_scale),
                        *delay + *time_offset,
                        tstop,
                        abort,
                        max_points,
                    )?;
                }
                None => match crate::device::pwl_file::load_pwl_file(path) {
                    Ok(wf) => {
                        let wf =
                            wf.with_scaling(*time_scale, *value_scale, *time_offset, *value_offset);
                        Self::add_repeating_pwl_breakpoints(
                            breakpoints,
                            wf.scaled_knot_times().map(|time| time + *delay),
                            repeat_from.map(|value| value * *time_scale),
                            *delay + *time_offset,
                            tstop,
                            abort,
                            max_points,
                        )?;
                    }
                    Err(err) => {
                        log::warn!(
                            "Failed to load PWL file '{}' for breakpoint extraction: {}",
                            path,
                            err
                        );
                    }
                },
            },
            SourceSpec::Pat {
                vhi,
                vlo,
                delay,
                rise,
                fall,
                sample,
                data,
                repeat_count,
            } => {
                if ![*vhi, *vlo, *delay, *rise, *fall, *sample]
                    .into_iter()
                    .all(Value::is_finite)
                    || *rise <= 0.0
                    || *fall <= 0.0
                    || *sample <= 0.0
                {
                    return Ok(());
                }

                let mut source_times = Vec::new();
                crate::circuit::VoltageSources::visit_pat_points(
                    *vhi,
                    *vlo,
                    *rise,
                    *fall,
                    *sample,
                    data,
                    |source_time, _| source_times.push(source_time),
                );
                if source_times.is_empty() {
                    return Ok(());
                }

                for &source_time in &source_times {
                    Self::add_breakpoint_if_in_range(breakpoints, *delay + source_time, tstop);
                }

                if *repeat_count == 0 {
                    return Ok(());
                }
                let Some(pattern_duration) =
                    crate::circuit::VoltageSources::pat_pattern_duration(data, *sample)
                else {
                    return Ok(());
                };
                if !pattern_duration.is_finite() || pattern_duration <= 0.0 {
                    return Ok(());
                }

                let requested_cycles = if *repeat_count < 0 {
                    (((tstop - *delay).max(0.0) / pattern_duration).ceil() as usize)
                        .saturating_add(1)
                } else {
                    *repeat_count as usize
                };
                if max_points != usize::MAX && requested_cycles > 1_000_000 {
                    return Err(crate::engine::SimulationError::Circuit(format!(
                        "transient source-event schedule requires {requested_cycles} PAT cycles, exceeding the exact enumeration limit 1000000"
                    )));
                }
                let max_cycles = requested_cycles.min(1_000_000);

                for cycle in 1..=max_cycles {
                    if cycle % 1024 == 0 {
                        Self::check_source_breakpoint_collection(breakpoints, abort, max_points)?;
                    }
                    let offset = cycle as Value * pattern_duration;
                    if *delay + offset > tstop && source_times[0] >= 0.0 {
                        break;
                    }
                    for &source_time in &source_times {
                        Self::add_breakpoint_if_in_range(
                            breakpoints,
                            *delay + offset + source_time,
                            tstop,
                        );
                    }
                }
            }
            SourceSpec::Exp { td1, td2, .. } => {
                // Xyce 7.10 ExpData inherits SourceData's no-op breakpoint
                // implementation.  Preserve ngspice/native edge scheduling,
                // but do not introduce timestep boundaries that Xyce itself
                // does not request when running in its compatibility dialect.
                if dialect == crate::engine::SpiceDialect::Xyce {
                    return Ok(());
                }
                // Match the waveform runtime: omitted or zero delays
                // resolve to tstep-based defaults (ngspice vsrcload.c).
                let step_default = tstep_hint.max(1e-18);
                let td1 = if td1.is_finite() && *td1 != 0.0 {
                    *td1
                } else {
                    step_default
                };
                let td2 = if td2.is_finite() && *td2 != 0.0 {
                    *td2
                } else {
                    td1 + step_default
                };
                Self::add_breakpoint_if_in_range(breakpoints, td1, tstop);
                Self::add_breakpoint_if_in_range(breakpoints, td2, tstop);
            }
            // SFFM is exactly 0 until TD and generally discontinuous there
            // (ngspice vsrcload.c), so native/ngspice operation schedules the
            // switch-on instant. Xyce 7.10 SFFMData inherits SourceData's
            // no-op breakpoint implementation.
            SourceSpec::Sffm { delay, .. } => {
                if dialect != crate::engine::SpiceDialect::Xyce {
                    Self::add_breakpoint_if_in_range(breakpoints, *delay, tstop);
                }
            }
            // AM is an ngspice/RSpice extension rather than a Xyce 7.10
            // SourceData family, so retain its established edge scheduling.
            SourceSpec::Am { delay, .. } => {
                Self::add_breakpoint_if_in_range(breakpoints, *delay, tstop);
            }
        }
        Self::check_source_breakpoint_collection(breakpoints, abort, max_points)
    }

    fn add_repeating_pwl_breakpoints<I>(
        breakpoints: &mut BreakpointManager,
        times: I,
        repeat_from: Option<Value>,
        time_offset: Value,
        tstop: Value,
        abort: &dyn crate::abort_signal::AbortSignal,
        max_points: usize,
    ) -> Result<(), crate::engine::SimulationError>
    where
        I: IntoIterator<Item = Value>,
    {
        let times = times
            .into_iter()
            .filter(|time| time.is_finite())
            .collect::<Vec<_>>();
        if times.is_empty() {
            return Ok(());
        }
        for &time in &times {
            Self::add_breakpoint_if_in_range(breakpoints, time, tstop);
        }

        let Some(repeat_from) = repeat_from else {
            return Self::check_source_breakpoint_collection(breakpoints, abort, max_points);
        };
        let Some(&last) = times.last() else {
            return Self::check_source_breakpoint_collection(breakpoints, abort, max_points);
        };
        let first = times[0];
        let repeat_start = (time_offset + repeat_from).max(first);
        if !repeat_start.is_finite() || repeat_start >= last {
            return Self::check_source_breakpoint_collection(breakpoints, abort, max_points);
        }
        let period = last - repeat_start;
        if !period.is_finite() || period <= Value::EPSILON {
            return Self::check_source_breakpoint_collection(breakpoints, abort, max_points);
        }

        let repeating_knots = times
            .iter()
            .copied()
            .filter(|time| *time >= repeat_start)
            .collect::<Vec<_>>();
        if repeating_knots.is_empty() {
            return Self::check_source_breakpoint_collection(breakpoints, abort, max_points);
        }
        let mut cycle = 1.0;
        loop {
            if cycle as usize % 1024 == 0 {
                Self::check_source_breakpoint_collection(breakpoints, abort, max_points)?;
            }
            let cycle_offset = period * cycle;
            let mut added = false;
            for &time in &repeating_knots {
                let repeated = time + cycle_offset;
                if repeated > tstop {
                    continue;
                }
                Self::add_breakpoint_if_in_range(breakpoints, repeated, tstop);
                added = true;
            }
            if !added || repeat_start + cycle_offset > tstop {
                break;
            }
            cycle += 1.0;
            if cycle > 1.0e6 {
                if max_points != usize::MAX && repeat_start + period * cycle <= tstop {
                    return Err(crate::engine::SimulationError::Circuit(
                        "transient source-event PWL repetition exceeds the exact enumeration limit 1000000"
                            .to_string(),
                    ));
                }
                break;
            }
        }
        Self::check_source_breakpoint_collection(breakpoints, abort, max_points)
    }

    fn check_source_breakpoint_collection(
        breakpoints: &BreakpointManager,
        abort: &dyn crate::abort_signal::AbortSignal,
        max_points: usize,
    ) -> Result<(), crate::engine::SimulationError> {
        if abort.is_aborted() {
            return Err(crate::engine::SimulationError::Aborted);
        }
        let count = breakpoints.times().len();
        if count > max_points {
            return Err(crate::engine::SimulationError::Circuit(format!(
                "transient source-event schedule requires more than the configured {max_points} analysis points"
            )));
        }
        Ok(())
    }

    pub(super) fn add_user_transient_breakpoints(
        breakpoints: &mut BreakpointManager,
        netlist: &crate::netlist::Netlist,
        tstop: Value,
        abort: &dyn crate::abort_signal::AbortSignal,
        max_points: usize,
    ) -> Result<(), crate::engine::SimulationError> {
        if !netlist.options.output_time_points.is_empty() {
            let transient_analyses = netlist
                .analyses
                .iter()
                .filter_map(|analysis| match analysis {
                    crate::netlist::AnalysisCommand::Tran { stop, start, .. } => {
                        Some((*stop, *start))
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();
            if !matches!(transient_analyses.as_slice(), [(stop, None)] if stop.to_bits() == tstop.to_bits())
            {
                return Err(crate::engine::SimulationError::Circuit(
                    "OUTPUTTIMEPOINTS currently requires one selected .TRAN analysis with omitted TSTART"
                        .to_string(),
                ));
            }
            if let Some(time) = netlist
                .options
                .output_time_points
                .iter()
                .find(|time| **time > tstop)
            {
                return Err(crate::engine::SimulationError::Circuit(format!(
                    "OUTPUTTIMEPOINTS time {time} exceeds the selected transient stop {tstop}"
                )));
            }
            if netlist.options.output_time_points.windows(2).any(|times| {
                times[0].to_bits() != times[1].to_bits()
                    && (times[1] - times[0]).abs()
                        <= crate::numerics::integration::XYCE_BREAKPOINT_TOLERANCE
            }) {
                return Err(crate::engine::SimulationError::Circuit(
                    "OUTPUTTIMEPOINTS contains distinct times inside the Xyce breakpoint equality tolerance"
                        .to_string(),
                ));
            }
            let conflicts_with = |time: Value, sorted: &[Value]| {
                let position = sorted.partition_point(|candidate| *candidate < time);
                sorted
                    .get(position.saturating_sub(1)..position.saturating_add(1).min(sorted.len()))
                    .into_iter()
                    .flatten()
                    .any(|candidate| {
                        candidate.to_bits() != time.to_bits()
                            && (*candidate - time).abs()
                                <= crate::numerics::integration::XYCE_BREAKPOINT_TOLERANCE
                    })
            };
            if netlist.options.output_time_points.iter().any(|time| {
                conflicts_with(*time, breakpoints.times())
                    || conflicts_with(*time, &netlist.options.timeint_breakpoints)
            }) {
                return Err(crate::engine::SimulationError::Circuit(
                    "OUTPUTTIMEPOINTS is not exactly representable beside an existing Xyce breakpoint"
                        .to_string(),
                ));
            }
        }
        let requested = netlist
            .options
            .output_time_points
            .len()
            .saturating_add(netlist.options.timeint_breakpoints.len());
        crate::resource::ResourceLimitError::ensure(
            crate::resource::ResourceKind::AnalysisPoints,
            requested,
            max_points,
        )?;
        let mut retained = Vec::with_capacity(requested);
        for (index, time) in netlist
            .options
            .output_time_points
            .iter()
            .chain(&netlist.options.timeint_breakpoints)
            .copied()
            .enumerate()
        {
            if index % 256 == 0 && abort.is_aborted() {
                return Err(crate::engine::SimulationError::Aborted);
            }
            if !time.is_finite() || time < 0.0 {
                return Err(crate::engine::SimulationError::Circuit(format!(
                    "transient user-breakpoint schedule contains invalid time {time}"
                )));
            }
            if time <= tstop {
                retained.push(if time == 0.0 { 0.0 } else { time });
            }
        }
        retained.sort_by(Value::total_cmp);
        retained.dedup_by(|left, right| {
            (*left - *right).abs() <= crate::numerics::integration::XYCE_BREAKPOINT_TOLERANCE
        });
        let existing = breakpoints.times();
        let mut existing_index = 0usize;
        let mut retained_index = 0usize;
        let mut union_count = 0usize;
        let mut last: Option<Value> = None;
        while existing_index < existing.len() || retained_index < retained.len() {
            let value = match (
                existing.get(existing_index).copied(),
                retained.get(retained_index).copied(),
            ) {
                (Some(established), Some(candidate))
                    if (established - candidate).abs()
                        <= crate::numerics::integration::XYCE_BREAKPOINT_TOLERANCE =>
                {
                    existing_index += 1;
                    retained_index += 1;
                    established
                }
                (Some(established), Some(candidate))
                    if established.total_cmp(&candidate).is_le() =>
                {
                    existing_index += 1;
                    established
                }
                (Some(_), Some(candidate)) => {
                    retained_index += 1;
                    candidate
                }
                (Some(established), None) => {
                    existing_index += 1;
                    established
                }
                (None, Some(candidate)) => {
                    retained_index += 1;
                    candidate
                }
                (None, None) => break,
            };
            if last.is_none_or(|last| {
                (value - last).abs() > crate::numerics::integration::XYCE_BREAKPOINT_TOLERANCE
            }) {
                union_count = union_count.saturating_add(1);
                last = Some(value);
            }
        }
        crate::resource::ResourceLimitError::ensure(
            crate::resource::ResourceKind::AnalysisPoints,
            union_count,
            max_points,
        )?;
        breakpoints.extend(retained);
        Self::check_source_breakpoint_collection(breakpoints, abort, max_points)
    }

    pub(in crate::engine) fn collect_transient_source_breakpoints(
        circuit: &crate::circuit::CircuitData,
        tstop: Value,
        tstep_hint: Value,
        dialect: crate::engine::SpiceDialect,
        breakpoints: &mut BreakpointManager,
        abort: &dyn crate::abort_signal::AbortSignal,
        max_points: usize,
    ) -> Result<(), crate::engine::SimulationError> {
        Self::collect_independent_source_breakpoints(
            circuit,
            tstop,
            tstep_hint,
            dialect,
            None,
            breakpoints,
            abort,
            max_points,
        )?;

        for switch in &circuit.generic_switches {
            for &time in switch.time_breakpoints() {
                Self::add_breakpoint_if_in_range(breakpoints, time, tstop);
            }
        }

        for time in circuit
            .behavioral_sources
            .transient_breakpoints(tstop, tstep_hint)
        {
            Self::add_breakpoint_if_in_range(breakpoints, time, tstop);
        }

        for instance in &circuit.xspice_instances {
            match instance.transient_breakpoints() {
                Ok(times) => {
                    for time in times {
                        Self::add_breakpoint_if_in_range(breakpoints, time, tstop);
                    }
                }
                Err(err) => {
                    log::warn!(
                        "Failed to collect XSPICE '{}' transient breakpoints: {:?}",
                        instance.model_name(),
                        err
                    );
                }
            }
        }

        Self::check_source_breakpoint_collection(breakpoints, abort, max_points)
    }

    /// Collect authored independent-source events, optionally restricted to a
    /// case-folded source-name set. This is deliberately narrower than the
    /// transient integrator's complete breakpoint schedule: envelope
    /// event-aligned sampling must follow the selected modulation sources,
    /// not internal switch, transmission-line, or code-model events.
    pub(in crate::engine) fn collect_independent_source_breakpoints(
        circuit: &crate::circuit::CircuitData,
        tstop: Value,
        tstep_hint: Value,
        dialect: crate::engine::SpiceDialect,
        selected_sources: Option<&std::collections::HashSet<String>>,
        breakpoints: &mut BreakpointManager,
        abort: &dyn crate::abort_signal::AbortSignal,
        max_points: usize,
    ) -> Result<(), crate::engine::SimulationError> {
        for (index, (name, spec, pwl_waveform)) in circuit
            .voltage_sources
            .transient_specs_named_with_pwl()
            .chain(circuit.current_sources.transient_specs_named_with_pwl())
            .enumerate()
        {
            if index % 256 == 0 {
                Self::check_source_breakpoint_collection(breakpoints, abort, max_points)?;
            }
            if selected_sources
                .is_some_and(|selected| !selected.contains(name.to_ascii_lowercase().as_str()))
            {
                continue;
            }
            Self::add_source_spec_breakpoints_with_pwl(
                breakpoints,
                spec,
                pwl_waveform,
                tstop,
                tstep_hint,
                dialect,
                abort,
                max_points,
            )?;
        }
        Self::check_source_breakpoint_collection(breakpoints, abort, max_points)
    }

    pub(super) fn transmission_line_delays(circuit: &crate::circuit::CircuitData) -> Vec<Value> {
        let mut delays: Vec<Value> = circuit
            .tlines
            .iter()
            // Ngspice TXL and LTRA advance their native histories from accepted
            // points and schedule arrival breakpoints dynamically from history
            // derivative changes, not from statically propagated source edges.
            .filter(|tl| {
                !tl.is_zero_length_pass_through()
                    && !tl.has_txl_runtime()
                    && !tl.has_distributed_rlgc()
            })
            .map(crate::device::TransmissionLine::delay)
            .chain(
                circuit
                    .coupled_tlines
                    .iter()
                    // Native (ngspice-faithful) CPL lines do NOT use statically
                    // propagated mode-arrival breakpoints: ngspice controls the
                    // CPL step purely via tstep/tmax (capped at 0.9*min(taul)).
                    // Flooding the schedule with per-mode arrival breakpoints
                    // forces sub-picosecond steps whose over-refined trapezoidal
                    // convolution diverges from the coarser-step reference. Only
                    // modal-fallback CPL lines contribute arrival breakpoints.
                    .filter(|tl| !tl.uses_native_runtime())
                    .flat_map(crate::device::CoupledTransmissionLine::propagation_delays),
            )
            .filter(|delay| delay.is_finite() && *delay > 0.0)
            .collect();
        delays.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        delays.dedup_by(|a, b| {
            let scale = a.abs().max(b.abs()).max(1.0);
            (*a - *b).abs() <= scale * 1e-12
        });
        delays
    }

    pub(super) fn collect_transient_tline_breakpoints(
        circuit: &crate::circuit::CircuitData,
        source_breakpoints: &[Value],
        tstop: Value,
        breakpoints: &mut BreakpointManager,
    ) {
        if source_breakpoints.is_empty() {
            return;
        }

        let delays = Self::transmission_line_delays(circuit);
        if delays.is_empty() {
            return;
        }

        let mut generated = 0_usize;
        'origins: for &origin in source_breakpoints {
            for &delay in &delays {
                let mut arrival = origin + delay;
                while arrival.is_finite() && arrival <= tstop {
                    if breakpoints.add(arrival) {
                        generated += 1;
                        if generated >= MAX_PROPAGATED_TLINE_BREAKPOINTS {
                            log::warn!(
                                "Capped propagated transmission-line breakpoints at {} entries (tstop={:.3e}s)",
                                MAX_PROPAGATED_TLINE_BREAKPOINTS,
                                tstop
                            );
                            break 'origins;
                        }
                    }
                    arrival += delay;
                }
            }
        }
    }

    #[inline]
    pub(super) fn wave_event_exceeds_tolerance(
        previous: Value,
        current: Value,
        reltol: Value,
        abstol: Value,
    ) -> bool {
        if !previous.is_finite() || !current.is_finite() {
            return false;
        }
        let scale = previous.abs().max(current.abs());
        let threshold = abstol.max(scale * reltol);
        (current - previous).abs() > threshold
    }

    #[inline]
    pub(super) fn maybe_schedule_tline_arrival_breakpoint(
        breakpoints: &mut BreakpointManager,
        event_time: Value,
        delay: Value,
        tstop: Value,
        previous_wave: Value,
        current_wave: Value,
        reltol: Value,
        abstol: Value,
        dynamic_breakpoints_added: &mut usize,
        warned_dynamic_breakpoint_cap: &mut bool,
    ) {
        if !Self::wave_event_exceeds_tolerance(previous_wave, current_wave, reltol, abstol) {
            return;
        }
        if !(event_time.is_finite() && delay.is_finite() && delay > 0.0) {
            return;
        }

        let arrival = event_time + delay;
        if !(arrival.is_finite() && arrival > event_time && arrival <= tstop) {
            return;
        }

        if *dynamic_breakpoints_added >= MAX_DYNAMIC_TLINE_BREAKPOINTS {
            if !*warned_dynamic_breakpoint_cap {
                log::warn!(
                    "Capped dynamic transmission-line breakpoints at {} entries (tstop={:.3e}s)",
                    MAX_DYNAMIC_TLINE_BREAKPOINTS,
                    tstop
                );
                *warned_dynamic_breakpoint_cap = true;
            }
            return;
        }

        if breakpoints.add(arrival) {
            *dynamic_breakpoints_added += 1;
            if *dynamic_breakpoints_added >= MAX_DYNAMIC_TLINE_BREAKPOINTS
                && !*warned_dynamic_breakpoint_cap
            {
                log::warn!(
                    "Capped dynamic transmission-line breakpoints at {} entries (tstop={:.3e}s)",
                    MAX_DYNAMIC_TLINE_BREAKPOINTS,
                    tstop
                );
                *warned_dynamic_breakpoint_cap = true;
            }
        }
    }

    #[inline]
    pub(super) fn schedule_dynamic_tline_breakpoint(
        breakpoints: &mut BreakpointManager,
        arrival: Value,
        tstop: Value,
        dynamic_breakpoints_added: &mut usize,
        warned_dynamic_breakpoint_cap: &mut bool,
    ) {
        if !(arrival.is_finite() && arrival >= 0.0 && arrival <= tstop) {
            return;
        }

        if *dynamic_breakpoints_added >= MAX_DYNAMIC_TLINE_BREAKPOINTS {
            if !*warned_dynamic_breakpoint_cap {
                log::warn!(
                    "Capped dynamic transmission-line breakpoints at {} entries (tstop={:.3e}s)",
                    MAX_DYNAMIC_TLINE_BREAKPOINTS,
                    tstop
                );
                *warned_dynamic_breakpoint_cap = true;
            }
            return;
        }

        if breakpoints.add(arrival) {
            *dynamic_breakpoints_added += 1;
            if *dynamic_breakpoints_added >= MAX_DYNAMIC_TLINE_BREAKPOINTS
                && !*warned_dynamic_breakpoint_cap
            {
                log::warn!(
                    "Capped dynamic transmission-line breakpoints at {} entries (tstop={:.3e}s)",
                    MAX_DYNAMIC_TLINE_BREAKPOINTS,
                    tstop
                );
                *warned_dynamic_breakpoint_cap = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_delays_close(actual: &[Value], expected: &[Value]) {
        assert_eq!(
            actual.len(),
            expected.len(),
            "actual={actual:?} expected={expected:?}"
        );
        for (&actual, &expected) in actual.iter().zip(expected) {
            let tolerance = expected.abs().max(1.0) * 1.0e-12;
            assert!(
                (actual - expected).abs() <= tolerance,
                "actual={actual:.17e} expected={expected:.17e} tolerance={tolerance:.17e}"
            );
        }
    }

    #[test]
    fn authored_schedule_union_honors_the_engine_resource_limit_before_allocation() {
        let netlist = crate::netlist::Netlist::parse(
            "bounded schedule\n\
             .TRAN 1 3\n\
             .OPTIONS OUTPUT OUTPUTTIMEPOINTS=1,2\n\
             .END\n",
        )
        .expect("bounded schedule parses");
        let mut breakpoints = BreakpointManager::new();
        breakpoints.add(0.0);
        breakpoints.add(3.0);

        let error = Engine::add_user_transient_breakpoints(
            &mut breakpoints,
            &netlist,
            3.0,
            &crate::abort_signal::NoAbort,
            3,
        )
        .expect_err("source and authored breakpoint union must share one engine limit");
        assert!(matches!(
            error,
            crate::engine::SimulationError::ResourceLimit(crate::resource::ResourceLimitError {
                resource: crate::resource::ResourceKind::AnalysisPoints,
                requested: 4,
                limit: 3,
            })
        ));
        assert_eq!(
            breakpoints.times(),
            &[0.0, 3.0],
            "failed admission must not mutate the established schedule"
        );
    }

    #[test]
    fn smooth_delayed_source_breakpoints_follow_dialect_policy() {
        let specs = [
            crate::netlist::SourceSpec::Sin {
                offset: 0.0,
                amplitude: 1.0,
                frequency: 1.0e6,
                delay: 10.0e-9,
                damping: 0.0,
                phase: 0.0,
            },
            crate::netlist::SourceSpec::Sffm {
                offset: 0.0,
                amplitude: 1.0,
                carrier_freq: 1.0e6,
                modulation_index: 0.5,
                signal_freq: 100.0e3,
                delay: 10.0e-9,
                phase_modulation: 0.0,
                phase_carrier: 0.0,
            },
        ];

        for spec in &specs {
            for dialect in [
                crate::engine::SpiceDialect::BestAvailable,
                crate::engine::SpiceDialect::Ngspice,
            ] {
                let mut breakpoints = BreakpointManager::new();
                Engine::add_source_spec_breakpoints(
                    &mut breakpoints,
                    spec,
                    100.0e-9,
                    1.0e-9,
                    dialect,
                );
                assert_delays_close(breakpoints.times(), &[10.0e-9]);
            }

            let mut xyce_breakpoints = BreakpointManager::new();
            Engine::add_source_spec_breakpoints(
                &mut xyce_breakpoints,
                spec,
                100.0e-9,
                1.0e-9,
                crate::engine::SpiceDialect::Xyce,
            );
            assert!(xyce_breakpoints.times().is_empty());
        }
    }

    #[test]
    fn external_pwl_breakpoints_use_the_circuit_owned_snapshot() {
        let spec = crate::netlist::SourceSpec::PwlFile {
            path: "this-file-must-not-be-opened-for-breakpoints.pwl".to_string(),
            time_scale: 2.0,
            value_scale: 1.0,
            time_offset: 1.0,
            value_offset: 0.0,
            delay: 3.0,
            repeat_from: None,
        };
        let waveform = crate::device::pwl_file::PwlWaveform::new(vec![(0.0, 0.0), (2.0, 1.0)])
            .expect("valid waveform")
            .with_scaling(2.0, 1.0, 1.0, 0.0);
        let mut breakpoints = BreakpointManager::new_with_tolerance(1.0e-12);

        Engine::add_source_spec_breakpoints_with_pwl(
            &mut breakpoints,
            &spec,
            Some(&waveform),
            10.0,
            0.1,
            crate::engine::SpiceDialect::Xyce,
            &crate::abort_signal::NoAbort,
            usize::MAX,
        )
        .expect("unbounded breakpoint extraction");

        assert_delays_close(breakpoints.times(), &[4.0, 8.0]);
    }

    #[test]
    fn exponential_source_breakpoints_follow_dialect_policy() {
        let spec = crate::netlist::SourceSpec::Exp {
            v1: 0.0,
            v2: 10.0,
            td1: 3.0e-3,
            tau1: 10.0e-3,
            td2: 200.0e-3,
            tau2: 10.0e-3,
        };

        let mut xyce_breakpoints = BreakpointManager::new();
        Engine::add_source_spec_breakpoints(
            &mut xyce_breakpoints,
            &spec,
            300.0e-3,
            100.0e-6,
            crate::engine::SpiceDialect::Xyce,
        );
        assert!(xyce_breakpoints.times().is_empty());

        for dialect in [
            crate::engine::SpiceDialect::BestAvailable,
            crate::engine::SpiceDialect::Ngspice,
        ] {
            let mut breakpoints = BreakpointManager::new();
            Engine::add_source_spec_breakpoints(
                &mut breakpoints,
                &spec,
                300.0e-3,
                100.0e-6,
                dialect,
            );
            assert_delays_close(breakpoints.times(), &[3.0e-3, 200.0e-3]);
        }
    }

    /// PW omitted defaults the width to zero, and with PER omitted too the
    /// only period the defaults could derive is the two edges. A one-shot has
    /// three breakpoints — the delay and the two edge ends — and nothing after
    /// them, because the waveform is flat at V1 for the rest of the run.
    /// Scheduling a repeat there would force timesteps at times where nothing
    /// happens. See `pulse_with_no_width_and_no_period_fires_once` for the
    /// waveform side and the reference deck that pins it.
    #[test]
    fn pulse_with_only_rise_and_fall_schedules_one_shot_breakpoints() {
        let mut breakpoints = BreakpointManager::new_with_tolerance(1.0e-21);
        let spec = crate::netlist::SourceSpec::Pulse {
            v1: 0.0,
            v2: 1.0,
            delay: 1.0e-9,
            rise: 2.0e-9,
            fall: 3.0e-9,
            width: Value::NAN,
            period: Value::NAN,
            pulse_count: 0.0,
            width_defaults_to_zero: true,
        };

        Engine::add_source_spec_breakpoints(
            &mut breakpoints,
            &spec,
            20.0e-9,
            0.5e-9,
            crate::engine::SpiceDialect::BestAvailable,
        );

        assert_delays_close(breakpoints.times(), &[1.0e-9, 3.0e-9, 6.0e-9]);
    }

    /// An authored PW still supplies a period worth repeating on, so the
    /// train keeps scheduling edges for the whole run. This is the case
    /// `tests/ngspice/general/rtlinv.cir` measures, and it is what keeps the
    /// one-shot rule above narrow.
    #[test]
    fn pulse_with_authored_width_and_omitted_period_still_repeats() {
        let mut breakpoints = BreakpointManager::new_with_tolerance(1.0e-21);
        let spec = crate::netlist::SourceSpec::Pulse {
            v1: 0.0,
            v2: 1.0,
            delay: 1.0e-9,
            rise: 2.0e-9,
            fall: 3.0e-9,
            width: 4.0e-9,
            period: Value::NAN,
            pulse_count: 0.0,
            width_defaults_to_zero: false,
        };

        Engine::add_source_spec_breakpoints(
            &mut breakpoints,
            &spec,
            20.0e-9,
            0.5e-9,
            crate::engine::SpiceDialect::BestAvailable,
        );

        // Period is TR + PW + TF = 9 ns: edges at TD, +TR, +TR+PW, +TR+PW+TF,
        // then the same four one period later, and the start of the third.
        assert_delays_close(
            breakpoints.times(),
            &[1.0e-9, 3.0e-9, 7.0e-9, 10.0e-9, 12.0e-9, 16.0e-9, 19.0e-9],
        );
    }

    #[test]
    fn xyce_pulse_omitted_period_breakpoints_use_transient_stop_default() {
        let mut breakpoints = BreakpointManager::new_with_tolerance(1.0e-15);
        let spec = crate::netlist::SourceSpec::Pulse {
            v1: 0.0,
            v2: 1.0,
            delay: 10.0e-6,
            rise: 1.0e-6,
            fall: 1.0e-6,
            width: 100.0e-3,
            period: Value::NAN,
            pulse_count: 0.0,
            width_defaults_to_zero: false,
        };

        Engine::add_source_spec_breakpoints(
            &mut breakpoints,
            &spec,
            400.0e-3,
            0.5e-6,
            crate::engine::SpiceDialect::Xyce,
        );

        assert_delays_close(
            breakpoints.times(),
            &[10.0e-6, 11.0e-6, 100.011e-3, 100.012e-3],
        );
    }

    #[test]
    fn xyce_explicit_zero_pulse_edges_do_not_gain_tstep_ramps() {
        let mut breakpoints = BreakpointManager::new_with_tolerance(1.0e-15);
        let spec = crate::netlist::SourceSpec::Pulse {
            v1: 0.0,
            v2: 20.0,
            delay: 0.0,
            rise: 0.0,
            fall: 0.0,
            width: 0.2,
            period: 0.4,
            pulse_count: 0.0,
            width_defaults_to_zero: false,
        };

        Engine::add_source_spec_breakpoints(
            &mut breakpoints,
            &spec,
            0.8,
            0.02,
            crate::engine::SpiceDialect::Xyce,
        );

        assert_delays_close(breakpoints.times(), &[0.0, 0.2, 0.4, 0.6, 0.8]);
    }

    #[test]
    fn xyce_far_negative_pulse_delay_skips_historical_cycles() {
        let mut breakpoints = BreakpointManager::new_with_tolerance(1.0e-15);
        let spec = crate::netlist::SourceSpec::Pulse {
            v1: 0.0,
            v2: 1.0,
            delay: -2_000_000.0,
            rise: 0.0,
            fall: 0.0,
            width: 0.25,
            period: 1.0,
            pulse_count: 0.0,
            width_defaults_to_zero: false,
        };

        Engine::add_source_spec_breakpoints_with_pwl(
            &mut breakpoints,
            &spec,
            None,
            2.0,
            0.1,
            crate::engine::SpiceDialect::Xyce,
            &crate::abort_signal::NoAbort,
            64,
        )
        .expect("only in-range cycles should count toward the exact schedule");

        assert_delays_close(breakpoints.times(), &[0.0, 0.25, 1.0, 1.25, 2.0]);
    }

    #[test]
    fn xyce_fractional_period_includes_representable_final_cycle() {
        let mut breakpoints = BreakpointManager::new_with_tolerance(1.0e-15);
        let spec = crate::netlist::SourceSpec::Pulse {
            v1: 0.0,
            v2: 1.0,
            delay: 0.001,
            rise: 0.0,
            fall: 0.0,
            width: 0.0,
            period: 0.001,
            pulse_count: 0.0,
            width_defaults_to_zero: false,
        };

        Engine::add_source_spec_breakpoints(
            &mut breakpoints,
            &spec,
            0.011,
            0.0001,
            crate::engine::SpiceDialect::Xyce,
        );

        assert_delays_close(
            breakpoints.times(),
            &[
                0.001, 0.002, 0.003, 0.004, 0.005, 0.006, 0.007, 0.008, 0.009, 0.010, 0.011,
            ],
        );
    }

    #[test]
    fn xyce_negative_pulse_period_has_no_nonnegative_edges() {
        let mut breakpoints = BreakpointManager::new_with_tolerance(1.0e-15);
        let spec = crate::netlist::SourceSpec::Pulse {
            v1: 0.0,
            v2: 1.0,
            delay: 0.0,
            rise: 0.0,
            fall: 0.0,
            width: 0.2,
            period: -0.4,
            pulse_count: 0.0,
            width_defaults_to_zero: false,
        };

        Engine::add_source_spec_breakpoints(
            &mut breakpoints,
            &spec,
            0.8,
            0.02,
            crate::engine::SpiceDialect::Xyce,
        );

        assert!(breakpoints.times().is_empty());
    }

    #[test]
    fn ngspice_pulse_omitted_period_breakpoints_repeat_after_waveform_duration() {
        let mut breakpoints = BreakpointManager::new_with_tolerance(1.0e-15);
        let spec = crate::netlist::SourceSpec::Pulse {
            v1: 0.0,
            v2: 1.0,
            delay: 10.0e-6,
            rise: 1.0e-6,
            fall: 1.0e-6,
            width: 100.0e-3,
            period: Value::NAN,
            pulse_count: 0.0,
            width_defaults_to_zero: false,
        };

        Engine::add_source_spec_breakpoints(
            &mut breakpoints,
            &spec,
            400.0e-3,
            0.5e-6,
            crate::engine::SpiceDialect::Ngspice,
        );

        assert_delays_close(
            breakpoints.times(),
            &[
                10.0e-6, 11.0e-6, 100.011e-3, 100.012e-3, 100.013e-3, 200.013e-3, 200.014e-3,
                200.015e-3, 300.015e-3, 300.016e-3, 300.017e-3,
            ],
        );
    }

    /// A bounded train schedules the edges of the periods it runs for and
    /// nothing after: `NP = 2` over a 10 ms window is two cycles of edges,
    /// then a flat hold that needs no timestep placed in it.
    #[test]
    fn bounded_pulse_train_stops_scheduling_edges_after_its_last_period() {
        let mut breakpoints = BreakpointManager::new_with_tolerance(1.0e-15);
        let spec = crate::netlist::SourceSpec::Pulse {
            v1: -1.0,
            v2: 1.0,
            delay: 0.0,
            rise: 1.0e-5,
            fall: 1.0e-5,
            width: 5.0e-4,
            period: 1.0e-3,
            pulse_count: 2.0,
            width_defaults_to_zero: false,
        };

        Engine::add_source_spec_breakpoints(
            &mut breakpoints,
            &spec,
            1.0e-2,
            2.0e-5,
            crate::engine::SpiceDialect::BestAvailable,
        );

        assert_delays_close(
            breakpoints.times(),
            &[
                0.0, 1.0e-5, 5.1e-4, 5.2e-4, 1.0e-3, 1.01e-3, 1.51e-3, 1.52e-3, 2.0e-3, 2.01e-3,
                2.51e-3, 2.52e-3,
            ],
        );
    }

    #[test]
    fn transmission_line_delays_skip_native_txl_and_ltra_scalar_lines() {
        let mut circuit = crate::circuit::CircuitData::new();

        circuit.tlines.push(crate::device::TransmissionLine::new(
            "TLOSSLESS".to_string(),
            1,
            0,
            2,
            0,
            50.0,
            1.0e-9,
        ));

        let mut txl_line =
            crate::device::TransmissionLine::new("TTXL".to_string(), 3, 0, 4, 0, 50.0, 4.0e-9);
        assert!(txl_line.enable_txl_runtime(12.45, 8.972e-9, 0.0, 0.468e-12, 16.0));
        circuit.tlines.push(txl_line);

        let mut ltra_line =
            crate::device::TransmissionLine::new("TLTRA".to_string(), 5, 0, 6, 0, 75.0, 5.0e-9);
        ltra_line.set_distributed_rlgc(0.25, 4.0, 0.0, 1.0, 1.0);
        circuit.tlines.push(ltra_line);

        let coupled = crate::device::CoupledTransmissionLine::new(
            "PCOUPLED".to_string(),
            vec![7, 8],
            0,
            vec![9, 10],
            0,
            &[vec![0.0, 0.0], vec![0.0, 0.0]],
            &[vec![4.0, 0.0], vec![0.0, 9.0]],
            &[vec![1.0, 0.0], vec![0.0, 1.0]],
            &[vec![0.0, 0.0], vec![0.0, 0.0]],
            1.0e-9,
        )
        .expect("valid coupled modal line");
        circuit.coupled_tlines.push(coupled);

        let delays = Engine::transmission_line_delays(&circuit);

        assert_delays_close(&delays, &[1.0e-9, 2.0e-9, 3.0e-9]);
    }

    #[test]
    fn pwlts_models_schedule_smoothed_table_time_breakpoints() {
        use std::sync::Arc;

        let instance = crate::xspice::XspiceInstance::new(
            "ATABLE",
            Arc::new(crate::xspice::models::PiecewiseLinearTimeSeries),
            vec![crate::xspice::PortConnection::Analog(1)],
            &[],
            &[],
            &[
                ("x_array".to_string(), vec![0.0, 1.0e-9, 2.0e-9, 3.0e-9]),
                ("y_array".to_string(), vec![0.0, 10.0, 20.0, 30.0]),
            ],
            &[],
        )
        .expect("pwlts instance constructs");
        let mut circuit = crate::circuit::CircuitData::new();
        circuit.add_xspice_instance(instance);

        let mut breakpoints = BreakpointManager::new_with_tolerance(1.0e-21);
        Engine::collect_transient_source_breakpoints(
            &circuit,
            2.5e-9,
            1.0e-9,
            crate::engine::SpiceDialect::BestAvailable,
            &mut breakpoints,
            &crate::abort_signal::NoAbort,
            usize::MAX,
        )
        .expect("bounded source breakpoint collection");

        assert_delays_close(
            breakpoints.times(),
            &[0.99e-9, 1.0e-9, 1.01e-9, 1.99e-9, 2.0e-9, 2.01e-9],
        );
    }

    #[test]
    fn propagated_tline_breakpoints_skip_ltra_but_keep_lossless_scalar() {
        let mut circuit = crate::circuit::CircuitData::new();

        circuit.tlines.push(crate::device::TransmissionLine::new(
            "TLOSSLESS".to_string(),
            1,
            0,
            2,
            0,
            50.0,
            1.0e-9,
        ));

        let mut ltra_line =
            crate::device::TransmissionLine::new("TLTRA".to_string(), 3, 0, 4, 0, 75.0, 5.0e-9);
        ltra_line.set_distributed_rlgc(0.25, 4.0, 0.0, 1.0, 1.0);
        circuit.tlines.push(ltra_line);

        let mut breakpoints = BreakpointManager::new();
        Engine::collect_transient_tline_breakpoints(&circuit, &[1.0e-9], 7.0e-9, &mut breakpoints);

        assert_delays_close(
            breakpoints.times(),
            &[2.0e-9, 3.0e-9, 4.0e-9, 5.0e-9, 6.0e-9, 7.0e-9],
        );
    }
}
