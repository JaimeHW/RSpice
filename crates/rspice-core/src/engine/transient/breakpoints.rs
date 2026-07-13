//! Source and transmission-line breakpoint scheduling helpers.

use super::*;

impl Engine {
    #[inline]
    pub(super) fn max_expected_source_delta(
        circuit: &crate::circuit::Circuit,
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
        circuit: &crate::circuit::Circuit,
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
        circuit: &mut crate::circuit::Circuit,
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

    pub(super) fn add_source_spec_breakpoints(
        breakpoints: &mut BreakpointManager,
        spec: &crate::netlist::SourceSpec,
        tstop: Value,
        tstep_hint: Value,
        dialect: crate::engine::SpiceDialect,
    ) {
        use crate::netlist::SourceSpec;

        match spec {
            SourceSpec::Distortion { inner, .. } => {
                Self::add_source_spec_breakpoints(breakpoints, inner, tstop, tstep_hint, dialect);
            }
            SourceSpec::RfPort { inner, .. } => {
                Self::add_source_spec_breakpoints(breakpoints, inner, tstop, tstep_hint, dialect);
            }
            // TRNOISE breakpoints come from its expanded PWL sample train;
            // the unexpanded spec itself schedules none.
            SourceSpec::Dc(_)
            | SourceSpec::Ac { .. }
            | SourceSpec::DcAc { .. }
            | SourceSpec::TrNoise { .. } => {}
            SourceSpec::DcTransient { transient, .. }
            | SourceSpec::DcAcTransient { transient, .. } => {
                Self::add_source_spec_breakpoints(
                    breakpoints,
                    transient,
                    tstop,
                    tstep_hint,
                    dialect,
                );
            }
            SourceSpec::Pulse {
                delay,
                rise,
                fall,
                width,
                period,
                phase,
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
                let phase_time = if per_valid {
                    let phase_cycles = (phase / 360.0).rem_euclid(1.0);
                    if phase_cycles > 0.0 {
                        phase_cycles * per - per
                    } else {
                        0.0
                    }
                } else {
                    0.0
                };
                let max_cycles = if per_valid {
                    (((tstop - td - phase_time).max(0.0) / per).ceil() as usize).saturating_add(1)
                } else {
                    1
                };
                let max_cycles = max_cycles.min(1_000_000);

                for cycle in 0..max_cycles {
                    let cycle_start = if per_valid {
                        td - phase_time + per * cycle as Value
                    } else {
                        td
                    };
                    if cycle_start > tstop {
                        break;
                    }
                    Self::add_breakpoint_if_in_range(breakpoints, cycle_start, tstop);
                    Self::add_breakpoint_if_in_range(breakpoints, cycle_start + tr, tstop);
                    Self::add_breakpoint_if_in_range(breakpoints, cycle_start + tr + pw, tstop);
                    Self::add_breakpoint_if_in_range(
                        breakpoints,
                        cycle_start + tr + pw + tf,
                        tstop,
                    );
                    if !per_valid {
                        break;
                    }
                }
            }
            SourceSpec::Sin { delay, .. } => {
                Self::add_breakpoint_if_in_range(breakpoints, *delay, tstop);
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
                );
            }
            SourceSpec::PwlFile {
                path,
                time_scale,
                value_scale,
                time_offset,
                value_offset,
                delay,
                repeat_from,
            } => match crate::device::pwl_file::load_pwl_file(path) {
                Ok(wf) => {
                    let wf =
                        wf.with_scaling(*time_scale, *value_scale, *time_offset, *value_offset);
                    Self::add_repeating_pwl_breakpoints(
                        breakpoints,
                        wf.scaled_knot_times().map(|time| time + *delay),
                        repeat_from.map(|value| value * *time_scale),
                        *delay + *time_offset,
                        tstop,
                    );
                }
                Err(err) => {
                    log::warn!(
                        "Failed to load PWL file '{}' for breakpoint extraction: {}",
                        path,
                        err
                    );
                }
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
                    return;
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
                    return;
                }

                for &source_time in &source_times {
                    Self::add_breakpoint_if_in_range(breakpoints, *delay + source_time, tstop);
                }

                if *repeat_count == 0 {
                    return;
                }
                let Some(pattern_duration) =
                    crate::circuit::VoltageSources::pat_pattern_duration(data, *sample)
                else {
                    return;
                };
                if !pattern_duration.is_finite() || pattern_duration <= 0.0 {
                    return;
                }

                let max_cycles = if *repeat_count < 0 {
                    (((tstop - *delay).max(0.0) / pattern_duration).ceil() as usize)
                        .saturating_add(1)
                } else {
                    *repeat_count as usize
                }
                .min(1_000_000);

                for cycle in 1..=max_cycles {
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
            // SFFM/AM are exactly 0 until TD and generally discontinuous
            // there (ngspice vsrcload.c), so the switch-on instant must be
            // a timestep boundary.
            SourceSpec::Sffm { delay, .. } | SourceSpec::Am { delay, .. } => {
                Self::add_breakpoint_if_in_range(breakpoints, *delay, tstop);
            }
        }
    }

    fn add_repeating_pwl_breakpoints<I>(
        breakpoints: &mut BreakpointManager,
        times: I,
        repeat_from: Option<Value>,
        time_offset: Value,
        tstop: Value,
    ) where
        I: IntoIterator<Item = Value>,
    {
        let times = times
            .into_iter()
            .filter(|time| time.is_finite())
            .collect::<Vec<_>>();
        if times.is_empty() {
            return;
        }
        for &time in &times {
            Self::add_breakpoint_if_in_range(breakpoints, time, tstop);
        }

        let Some(repeat_from) = repeat_from else {
            return;
        };
        let Some(&last) = times.last() else {
            return;
        };
        let first = times[0];
        let repeat_start = (time_offset + repeat_from).max(first);
        if !repeat_start.is_finite() || repeat_start >= last {
            return;
        }
        let period = last - repeat_start;
        if !period.is_finite() || period <= Value::EPSILON {
            return;
        }

        let repeating_knots = times
            .iter()
            .copied()
            .filter(|time| *time >= repeat_start)
            .collect::<Vec<_>>();
        if repeating_knots.is_empty() {
            return;
        }
        let mut cycle = 1.0;
        loop {
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
                break;
            }
        }
    }

    pub(in crate::engine) fn collect_transient_source_breakpoints(
        circuit: &crate::circuit::Circuit,
        tstop: Value,
        tstep_hint: Value,
        dialect: crate::engine::SpiceDialect,
        breakpoints: &mut BreakpointManager,
    ) {
        for spec in circuit
            .voltage_sources
            .source_specs
            .iter()
            .chain(circuit.current_sources.source_specs.iter())
            .filter_map(|spec| spec.as_ref())
        {
            Self::add_source_spec_breakpoints(breakpoints, spec, tstop, tstep_hint, dialect);
        }

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
    }

    pub(super) fn transmission_line_delays(circuit: &crate::circuit::Circuit) -> Vec<Value> {
        let mut delays: Vec<Value> = circuit
            .tlines
            .iter()
            // Ngspice TXL and LTRA advance their native histories from accepted
            // points and schedule arrival breakpoints dynamically from history
            // derivative changes, not from statically propagated source edges.
            .filter(|tl| !tl.has_txl_runtime() && !tl.has_distributed_rlgc())
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
        circuit: &crate::circuit::Circuit,
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
    fn sin_sources_schedule_delay_breakpoints() {
        let mut breakpoints = BreakpointManager::new();
        let spec = crate::netlist::SourceSpec::Sin {
            offset: 0.0,
            amplitude: 1.0,
            frequency: 1.0e6,
            delay: 10.0e-9,
            damping: 0.0,
            phase: 0.0,
        };

        Engine::add_source_spec_breakpoints(
            &mut breakpoints,
            &spec,
            100.0e-9,
            1.0e-9,
            crate::engine::SpiceDialect::BestAvailable,
        );

        assert_delays_close(breakpoints.times(), &[10.0e-9]);
    }

    #[test]
    fn pulse_with_only_rise_and_fall_defaults_width_to_zero_for_breakpoints() {
        let mut breakpoints = BreakpointManager::new_with_tolerance(1.0e-21);
        let spec = crate::netlist::SourceSpec::Pulse {
            v1: 0.0,
            v2: 1.0,
            delay: 1.0e-9,
            rise: 2.0e-9,
            fall: 3.0e-9,
            width: Value::NAN,
            period: Value::NAN,
            phase: 0.0,
            width_defaults_to_zero: true,
        };

        Engine::add_source_spec_breakpoints(
            &mut breakpoints,
            &spec,
            20.0e-9,
            0.5e-9,
            crate::engine::SpiceDialect::BestAvailable,
        );

        assert_delays_close(
            breakpoints.times(),
            &[
                1.0e-9, 3.0e-9, 6.0e-9, 8.0e-9, 11.0e-9, 13.0e-9, 16.0e-9, 18.0e-9,
            ],
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
            phase: 0.0,
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
    fn ngspice_pulse_omitted_period_breakpoints_use_transient_stop_default() {
        let mut breakpoints = BreakpointManager::new_with_tolerance(1.0e-15);
        let spec = crate::netlist::SourceSpec::Pulse {
            v1: 0.0,
            v2: 1.0,
            delay: 10.0e-6,
            rise: 1.0e-6,
            fall: 1.0e-6,
            width: 100.0e-3,
            period: Value::NAN,
            phase: 0.0,
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
            &[10.0e-6, 11.0e-6, 100.011e-3, 100.012e-3],
        );
    }

    #[test]
    fn pulse_phase_shifts_breakpoints_like_ngspice_xspice_mode() {
        let mut breakpoints = BreakpointManager::new_with_tolerance(1.0e-15);
        let spec = crate::netlist::SourceSpec::Pulse {
            v1: -1.0,
            v2: 1.0,
            delay: 0.0,
            rise: 1.0e-5,
            fall: 1.0e-5,
            width: 5.0e-4,
            period: 1.0e-3,
            phase: 45.0,
            width_defaults_to_zero: false,
        };

        Engine::add_source_spec_breakpoints(
            &mut breakpoints,
            &spec,
            1.0e-3,
            2.0e-5,
            crate::engine::SpiceDialect::BestAvailable,
        );

        assert_delays_close(breakpoints.times(), &[8.75e-4, 8.85e-4]);
    }

    #[test]
    fn transmission_line_delays_skip_native_txl_and_ltra_scalar_lines() {
        let mut circuit = crate::circuit::Circuit::new();

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
        let mut circuit = crate::circuit::Circuit::new();
        circuit.add_xspice_instance(instance);

        let mut breakpoints = BreakpointManager::new_with_tolerance(1.0e-21);
        Engine::collect_transient_source_breakpoints(
            &circuit,
            2.5e-9,
            1.0e-9,
            crate::engine::SpiceDialect::BestAvailable,
            &mut breakpoints,
        );

        assert_delays_close(
            breakpoints.times(),
            &[0.99e-9, 1.0e-9, 1.01e-9, 1.99e-9, 2.0e-9, 2.01e-9],
        );
    }

    #[test]
    fn propagated_tline_breakpoints_skip_ltra_but_keep_lossless_scalar() {
        let mut circuit = crate::circuit::Circuit::new();

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
