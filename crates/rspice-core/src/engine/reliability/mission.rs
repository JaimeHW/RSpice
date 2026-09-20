//! Analytic repetition counts with retained-sample acceleration quadrature.
//!
//! All supported clocks accumulate irreversible effective age. This permits
//! combining identical complete mission cycles without simulating every cycle.
//! Each phase's representative waveform restarts at that phase's boundary.

use super::*;

pub(super) fn integrate(
    request: &ReliabilityRunRequest,
    phases: &[ReliabilityPhaseStress],
    limits: crate::resource::ResourceLimits,
    mut retained: usize,
    abort: &dyn AbortSignal,
) -> Result<Vec<ReliabilityAgingCheckpoint>, SimulationError> {
    crate::resource::ResourceLimitError::ensure(
        crate::resource::ResourceKind::AnalysisPoints,
        request.target_years.len(),
        limits.max_analysis_points,
    )?;
    let cycle: f64 = request
        .study
        .mission
        .iter()
        .map(|phase| phase.duration_s)
        .sum();
    let mut checkpoints = Vec::new();
    for &years in &request.target_years {
        check_abort(abort)?;
        let seconds = years * SECONDS_PER_AGING_YEAR;
        let cycles = if request.study.repeat_mission {
            (seconds / cycle).floor()
        } else {
            0.0
        };
        if !cycles.is_finite() {
            return Err(invalid("Mission repetition count overflow"));
        }
        let mut remaining = if request.study.repeat_mission {
            seconds % cycle
        } else {
            seconds
        };
        let phase_exposures: Vec<_> = request
            .study
            .mission
            .iter()
            .map(|phase| {
                let partial = remaining.min(phase.duration_s).max(0.0);
                remaining = (remaining - partial).max(0.0);
                (cycles, partial)
            })
            .collect();
        let mut devices = Vec::new();
        for (device_index, binding) in request.study.bindings.iter().enumerate() {
            check_abort(abort)?;
            let mut contributions = Vec::new();
            for model_id in &binding.aging_models {
                let model = request
                    .study
                    .model_pack
                    .models
                    .iter()
                    .find(|model| &model.id == model_id)
                    .ok_or_else(|| invalid("Bound model disappeared during mission integration"))?;
                let mut clock = AgingClock::new(model).map_err(aging_error)?;
                for ((phase, data), &(cycles, partial)) in request
                    .study
                    .mission
                    .iter()
                    .zip(phases)
                    .zip(&phase_exposures)
                {
                    check_abort(abort)?;
                    let samples = &data.devices[device_index].samples;
                    if cycles > 0.0 {
                        integrate_window(
                            &mut clock,
                            model.mechanism,
                            &data.time_s,
                            samples,
                            phase.duration_s,
                            cycles,
                            request.min_stress_voltage,
                            abort,
                        )?;
                    }
                    if partial > 0.0 {
                        integrate_window(
                            &mut clock,
                            model.mechanism,
                            &data.time_s,
                            samples,
                            partial,
                            1.0,
                            request.min_stress_voltage,
                            abort,
                        )?;
                    }
                }
                let mut evaluated = clock.evaluate().map_err(aging_error)?;
                // Summing quadrature intervals introduces rounding in the clock's
                // diagnostic elapsed time; the requested checkpoint owns this axis.
                evaluated.elapsed_seconds = seconds;
                retained = retained.saturating_add(3 + evaluated.parameters.len());
                crate::resource::ResourceLimitError::ensure(
                    crate::resource::ResourceKind::ResultValues,
                    retained,
                    limits.max_result_values,
                )?;
                contributions.push(evaluated);
            }
            devices.push(ReliabilityDeviceAging {
                device: binding.device.clone(),
                contributions,
            });
        }
        checkpoints.push(ReliabilityAgingCheckpoint { years, devices });
    }
    Ok(checkpoints)
}

#[allow(clippy::too_many_arguments)]
fn integrate_window(
    clock: &mut AgingClock<'_>,
    mechanism: AgingMechanism,
    times: &[f64],
    samples: &[AgingStress],
    exposure: f64,
    repetitions: f64,
    minimum_gate: f64,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    let span = *times.last().ok_or_else(|| invalid("Empty stress window"))?;
    if !span.is_finite() || span <= 0.0 || times.len() != samples.len() || times[0] != 0.0 {
        return Err(invalid("Malformed stress window"));
    }
    let whole = (exposure / span).floor();
    let remainder = exposure % span;
    let complete_weight = whole * repetitions;
    if !complete_weight.is_finite() {
        return Err(invalid("Stress window repetition count overflow"));
    }
    for index in 1..times.len() {
        check_abort(abort)?;
        let start = times[index - 1];
        let stop = times[index];
        if !start.is_finite() || !stop.is_finite() || stop <= start {
            return Err(invalid("Stress time axis is not strictly increasing"));
        }
        if complete_weight > 0.0 {
            trapezoid(
                clock,
                mechanism,
                samples[index - 1],
                samples[index],
                (stop - start) * complete_weight,
                minimum_gate,
                abort,
            )?;
        }
        if remainder > start {
            let endpoint = remainder.min(stop);
            let weight = (endpoint - start) / (stop - start);
            let interpolated = blend(samples[index - 1], samples[index], weight);
            trapezoid(
                clock,
                mechanism,
                samples[index - 1],
                interpolated,
                (endpoint - start) * repetitions,
                minimum_gate,
                abort,
            )?;
        }
    }
    Ok(())
}

fn blend(a: AgingStress, b: AgingStress, weight: f64) -> AgingStress {
    let blend = |a, b| (1.0 - weight) * a + weight * b;
    AgingStress {
        gate_source_v: blend(a.gate_source_v, b.gate_source_v),
        drain_source_v: blend(a.drain_source_v, b.drain_source_v),
        temperature_k: blend(a.temperature_k, b.temperature_k),
        current_density_a_per_m2: blend(a.current_density_a_per_m2, b.current_density_a_per_m2),
    }
}

fn trapezoid(
    clock: &mut AgingClock<'_>,
    mechanism: AgingMechanism,
    a: AgingStress,
    b: AgingStress,
    duration: f64,
    minimum_gate: f64,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    if !duration.is_finite() || duration <= 0.0 {
        return Err(invalid("Stress quadrature duration is not representable"));
    }
    if a == b {
        return clock
            .advance_with_activity(
                duration,
                a,
                mechanism == AgingMechanism::Electromigration
                    || a.gate_source_v.abs() >= minimum_gate,
                abort,
            )
            .map_err(aging_error);
    }
    for stress in [a, b] {
        clock
            .advance_with_activity(
                duration * 0.5,
                stress,
                mechanism == AgingMechanism::Electromigration
                    || stress.gate_source_v.abs() >= minimum_gate,
                abort,
            )
            .map_err(aging_error)?;
    }
    Ok(())
}

fn aging_error(error: AgingError) -> SimulationError {
    match error {
        AgingError::Aborted => SimulationError::Aborted,
        other => invalid(other.to_string()),
    }
}
