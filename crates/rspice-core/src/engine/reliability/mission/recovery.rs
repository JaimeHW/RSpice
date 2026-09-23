//! Preserve noncommuting capture/recovery intervals through both repetition
//! levels: representative transient windows inside ordered mission phases.

use super::*;
use crate::analysis::reliability::AgingModel;

pub(super) fn integrate<'a>(
    model: &'a AgingModel,
    request: &ReliabilityRunRequest,
    phases: &[ReliabilityPhaseStress],
    device: usize,
    seconds: f64,
    abort: &dyn AbortSignal,
) -> Result<AgingClock<'a>, SimulationError> {
    let mut result = AgingClock::new(model).map_err(aging_error)?;
    let cycle_duration: f64 = request.study.mission.iter().map(|p| p.duration_s).sum();
    let cycles = if request.study.repeat_mission {
        (seconds / cycle_duration).floor()
    } else {
        0.0
    };
    let mut remaining = if request.study.repeat_mission {
        seconds % cycle_duration
    } else {
        seconds
    };
    if cycles > 0.0 {
        let mut cycle = AgingClock::new(model).map_err(aging_error)?;
        for (phase, data) in request.study.mission.iter().zip(phases) {
            append_window(
                &mut cycle,
                model,
                data,
                device,
                phase.duration_s,
                request.min_stress_voltage,
                abort,
            )?;
        }
        result
            .append_repeated_history(&cycle, cycles, abort)
            .map_err(aging_error)?;
    }
    for (phase, data) in request.study.mission.iter().zip(phases) {
        let duration = remaining.min(phase.duration_s).max(0.0);
        if duration > 0.0 {
            append_window(
                &mut result,
                model,
                data,
                device,
                duration,
                request.min_stress_voltage,
                abort,
            )?;
        }
        remaining = (remaining - duration).max(0.0);
    }
    Ok(result)
}

fn append_window<'a>(
    result: &mut AgingClock<'a>,
    model: &'a AgingModel,
    data: &ReliabilityPhaseStress,
    device: usize,
    duration: f64,
    minimum: f64,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    let times = &data.time_s;
    let samples = &data.devices[device].samples;
    let span = *times.last().ok_or_else(|| invalid("Empty stress window"))?;
    if !span.is_finite() || span <= 0.0 || times.len() != samples.len() || times[0] != 0.0 {
        return Err(invalid("Malformed stress window"));
    }
    let whole = (duration / span).floor();
    if !whole.is_finite() {
        return Err(invalid("Stress window repetition count overflow"));
    }
    if whole > 0.0 {
        let mut window = AgingClock::new(model).map_err(aging_error)?;
        append_samples(&mut window, times, samples, span, minimum, abort)?;
        result
            .append_repeated_history(&window, whole, abort)
            .map_err(aging_error)?;
    }
    let remainder = duration % span;
    if remainder > 0.0 {
        append_samples(result, times, samples, remainder, minimum, abort)?;
    }
    Ok(())
}

fn append_samples(
    clock: &mut AgingClock<'_>,
    times: &[f64],
    samples: &[AgingStress],
    duration: f64,
    minimum: f64,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    for i in 1..times.len() {
        check_abort(abort)?;
        let start = times[i - 1];
        let stop = times[i];
        if !start.is_finite() || !stop.is_finite() || stop <= start {
            return Err(invalid("Stress time axis is not strictly increasing"));
        }
        if start >= duration {
            break;
        }
        let endpoint = duration.min(stop);
        let b = blend(
            samples[i - 1],
            samples[i],
            (endpoint - start) / (stop - start),
        );
        // Exact at constant stress; for varying stress, chronological endpoint
        // half steps approximate the time-ordered kinetic equation to order 2.
        trapezoid(
            clock,
            AgingMechanism::Nbti,
            samples[i - 1],
            b,
            endpoint - start,
            minimum,
            abort,
        )?;
    }
    Ok(())
}
