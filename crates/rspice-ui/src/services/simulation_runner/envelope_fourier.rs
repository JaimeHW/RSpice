//! Envelope-following analysis.
//!
//! Integrates the slowly-varying envelope of a modulated carrier instead of
//! every RF cycle, and reports the Fourier content of each envelope sample.
//! The adaptive path skips periods whose envelope is not changing, which is
//! the whole point of the analysis.

use super::error::{ensure_not_aborted, poll_periodically};
use super::{
    ServiceRunError, ServiceRunResult, TransientData, build_engine_config,
    parse_runner_netlist_with_abort, run_transient_analysis_with_source_path_and_abort,
};
use crate::simulation::multi_run::{
    EnvelopeAdaptiveMode, EnvelopeExtractionPath, EnvelopeInitialPeriodicSolve,
};
use num_complex::Complex64;
use rspice_core::Value;
use rspice_core::abort_signal::{AbortSignal, NoAbort};
use rspice_core::analysis::{HbConfig, advanced::pss::PssConfig};
use rspice_core::engine::Engine;
use std::path::Path;

/// Conservative upper bound for dense least-squares work performed by one
/// preview extraction. The estimate deliberately over-counts elimination so
/// extreme tone lists fail before transient execution or allocation can
/// monopolize a desktop or browser process.
const MAX_ENVELOPE_PROJECTION_WORK: usize = 100_000_000;

/// Configuration for envelope analysis.
#[derive(Debug, Clone)]
pub struct EnvelopeRunConfig {
    pub fundamental_freq: Value,
    pub additional_carrier_tones: Vec<Value>,
    pub stop_time: Value,
    pub num_harmonics: usize,
    pub envelope_step: Option<Value>,
    pub modulation_sources: Vec<String>,
    pub initial_periodic_solve: EnvelopeInitialPeriodicSolve,
    pub adaptive_mode: EnvelopeAdaptiveMode,
    pub extraction_path: EnvelopeExtractionPath,
}

impl EnvelopeRunConfig {
    fn validate(&self) -> Result<(), String> {
        let mut seen_tones = std::collections::HashSet::new();
        for frequency in
            std::iter::once(&self.fundamental_freq).chain(self.additional_carrier_tones.iter())
        {
            if !frequency.is_finite() || *frequency <= 0.0 {
                return Err("Envelope carrier tones must be finite and positive".to_string());
            }
            if !seen_tones.insert(frequency.to_bits()) {
                return Err("Envelope carrier tones must be unique".to_string());
            }
        }
        if !self.stop_time.is_finite() || self.stop_time <= 0.0 {
            return Err("Envelope stop_time must be positive".to_string());
        }
        if self.num_harmonics == 0 {
            return Err("Envelope num_harmonics must be > 0".to_string());
        }
        if let Some(step) = self.envelope_step
            && (!step.is_finite() || step <= 0.0)
        {
            return Err("Envelope step must be positive when provided".to_string());
        }
        if self.envelope_step.is_some_and(|step| step > self.stop_time) {
            return Err("Envelope step cannot exceed stop_time".to_string());
        }
        let mut seen_sources = std::collections::HashSet::new();
        for source in &self.modulation_sources {
            let trimmed = source.trim();
            if trimmed.is_empty() || trimmed != source {
                return Err(
                    "Envelope modulation source names must be nonempty and trimmed".to_string(),
                );
            }
            if !seen_sources.insert(trimmed.to_ascii_lowercase()) {
                return Err("Envelope modulation source names must be unique".to_string());
            }
        }
        if self.modulation_sources.is_empty()
            && (self.initial_periodic_solve
                != EnvelopeInitialPeriodicSolve::TransientSpectralEstimate
                || self.adaptive_mode != EnvelopeAdaptiveMode::FixedEnvelopeStep)
        {
            return Err(
                "Envelope modulation sources are required for periodic initialization or adaptive/event-aligned execution"
                    .to_string(),
            );
        }
        Ok(())
    }

    fn carrier_tones(&self) -> impl Iterator<Item = Value> + '_ {
        std::iter::once(self.fundamental_freq).chain(self.additional_carrier_tones.iter().copied())
    }
}

/// Envelope analysis output.
#[derive(Debug, Clone)]
pub struct EnvelopeData {
    pub time: Vec<Value>,
    /// Complex carrier envelopes using
    /// `v(t) = Re{envelope(t) * exp(j*omega*t)}`.
    pub waveforms: Vec<(String, Vec<Complex64>)>,
}

/// Run the configured envelope analysis.
pub fn run_envelope_analysis(
    netlist_text: &str,
    config: &EnvelopeRunConfig,
) -> Result<EnvelopeData, String> {
    run_envelope_analysis_with_abort(netlist_text, config, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run envelope analysis with cooperative cancellation.
pub fn run_envelope_analysis_with_abort(
    netlist_text: &str,
    config: &EnvelopeRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<EnvelopeData> {
    run_envelope_analysis_with_source_path_and_abort(netlist_text, config, None, abort)
}

/// Run envelope analysis with a source path used to resolve relative includes
/// and model file references.
pub fn run_envelope_analysis_with_source_path(
    netlist_text: &str,
    config: &EnvelopeRunConfig,
    source_path: Option<&Path>,
) -> Result<EnvelopeData, String> {
    run_envelope_analysis_with_source_path_and_abort(netlist_text, config, source_path, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run envelope analysis with source-path resolution and cooperative
/// cancellation through initialization, transient solving, and preview
/// extraction.
pub fn run_envelope_analysis_with_source_path_and_abort(
    netlist_text: &str,
    config: &EnvelopeRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<EnvelopeData> {
    ensure_not_aborted(abort)?;
    let validation = config.validate();
    ensure_not_aborted(abort)?;
    validation.map_err(ServiceRunError::Failure)?;

    let parsed = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    let samples_per_cycle = (config.num_harmonics.max(1) as f64 * 16.0).max(32.0);
    let highest_carrier = config.carrier_tones().fold(0.0, Value::max);
    let carrier_step = 1.0 / (highest_carrier * samples_per_cycle);
    let coarse_step = config.stop_time / 1200.0;
    let requested_step = config.envelope_step.unwrap_or(coarse_step);
    let step_time = requested_step
        .min(carrier_step)
        .min(coarse_step)
        .min(config.stop_time);
    if !step_time.is_finite() || step_time <= 0.0 {
        return Err(ServiceRunError::Failure(
            "Envelope sampling step could not be represented".to_string(),
        ));
    }
    ensure_not_aborted(abort)?;
    let carrier_tones = config.carrier_tones().collect::<Vec<_>>();
    let estimated_transient_points = estimated_schedule_points(config.stop_time, step_time)?;
    let estimated_output_points = match config.adaptive_mode {
        EnvelopeAdaptiveMode::Enabled => estimated_transient_points,
        EnvelopeAdaptiveMode::FixedEnvelopeStep => {
            estimated_schedule_points(config.stop_time, config.envelope_step.unwrap_or(step_time))?
        }
        // Exact event enumeration is resource-bounded separately. A single
        // point still exercises the dense matrix/cubic tone-count guard here.
        EnvelopeAdaptiveMode::EventAlignedOnly => 1,
    };
    validate_projection_workload(
        estimated_transient_points,
        estimated_output_points,
        carrier_tones.len(),
        1,
    )?;
    Engine::new(build_engine_config(&parsed, None))
        .validate_transient_source_names_with_abort(&parsed, &config.modulation_sources, abort)
        .map_err(|error| {
            ServiceRunError::from_core("Envelope modulation-source validation error", error)
        })?;

    let transient = match config.initial_periodic_solve {
        EnvelopeInitialPeriodicSolve::TransientSpectralEstimate => {
            run_transient_analysis_with_source_path_and_abort(
                netlist_text,
                config.stop_time,
                step_time,
                source_path,
                abort,
            )?
        }
        EnvelopeInitialPeriodicSolve::PeriodicSteadyState => {
            run_pss_initialized_envelope_transient(&parsed, config, step_time, abort)?
        }
        EnvelopeInitialPeriodicSolve::HarmonicBalance => {
            run_hb_initialized_envelope_transient(&parsed, config, step_time, abort)?
        }
    };
    if transient.time.is_empty() {
        return Err(ServiceRunError::Failure(
            "Envelope analysis produced no transient samples".to_string(),
        ));
    }
    if transient.voltages.is_empty() {
        return Err(ServiceRunError::Failure(
            "Envelope analysis found no non-ground node waveforms".to_string(),
        ));
    }

    let requested_output_time =
        envelope_output_times(&parsed, &transient.time, config, step_time, abort)?;
    validate_projection_workload(
        transient.time.len(),
        requested_output_time.len(),
        carrier_tones.len(),
        transient.voltages.len(),
    )?;
    let output_time = centered_projection_output_times(
        &requested_output_time,
        transient.time[0],
        *transient.time.last().expect("nonempty transient checked"),
        &carrier_tones,
        abort,
    )?;
    let waveform_count = carrier_tones
        .len()
        .checked_mul(transient.voltages.len())
        .ok_or_else(|| {
            ServiceRunError::Failure("Envelope result waveform count overflowed".to_string())
        })?;
    let result_value_count = envelope_retained_value_count(output_time.len(), waveform_count)?;
    let result_limit = rspice_core::ResourceLimits::default().max_result_values;
    if result_value_count > result_limit {
        return Err(ServiceRunError::Failure(format!(
            "Envelope extraction requires {result_value_count} result values, exceeding the configured result-value limit {result_limit}"
        )));
    }
    let mut waveforms =
        Vec::with_capacity(transient.voltages.len().saturating_mul(carrier_tones.len()));
    for (name, values) in transient.voltages {
        ensure_not_aborted(abort)?;
        if values.is_empty() {
            continue;
        }
        let envelopes = match config.extraction_path {
            EnvelopeExtractionPath::Preview => compute_carrier_envelopes_with_abort(
                &transient.time,
                &values,
                &output_time,
                &carrier_tones,
                abort,
            )?,
        };
        for (tone_index, (carrier, env)) in carrier_tones.iter().copied().zip(envelopes).enumerate()
        {
            let label = if tone_index == 0 && carrier_tones.len() == 1 {
                format!("ENV({name})")
            } else {
                format!("ENV({name}@{carrier:.12e}Hz)")
            };
            waveforms.push((label, env));
        }
    }
    if waveforms.is_empty() {
        return Err(ServiceRunError::Failure(
            "Envelope analysis produced no envelope traces".to_string(),
        ));
    }

    ensure_not_aborted(abort)?;
    Ok(EnvelopeData {
        time: output_time,
        waveforms,
    })
}

fn envelope_retained_value_count(
    sample_count: usize,
    waveform_count: usize,
) -> ServiceRunResult<usize> {
    // The UI result retains one top-level time vector. Each complex waveform
    // then owns another time vector plus real and imaginary vectors. Count the
    // final representation rather than only the service-layer Complex64
    // payload so the browser and native paths enforce the same memory limit.
    waveform_count
        .checked_mul(3)
        .and_then(|per_sample| per_sample.checked_add(1))
        .and_then(|per_sample| sample_count.checked_mul(per_sample))
        .ok_or_else(|| {
            ServiceRunError::Failure("Envelope result value count overflowed".to_string())
        })
}

fn estimated_schedule_points(stop_time: Value, step: Value) -> ServiceRunResult<usize> {
    let intervals = (stop_time / step).ceil();
    if !intervals.is_finite() || intervals < 0.0 || intervals >= usize::MAX as Value {
        return Err(ServiceRunError::Failure(
            "Envelope schedule point estimate cannot be represented".to_string(),
        ));
    }
    (intervals as usize).checked_add(1).ok_or_else(|| {
        ServiceRunError::Failure("Envelope schedule point estimate overflowed".to_string())
    })
}

fn envelope_output_times(
    netlist: &rspice_core::Netlist,
    adaptive_time: &[Value],
    config: &EnvelopeRunConfig,
    solver_step: Value,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Value>> {
    match config.adaptive_mode {
        EnvelopeAdaptiveMode::Enabled => {
            let mut output = Vec::with_capacity(adaptive_time.len());
            for (index, time) in adaptive_time.iter().copied().enumerate() {
                poll_periodically(abort, index)?;
                output.push(time);
            }
            Ok(output)
        }
        EnvelopeAdaptiveMode::FixedEnvelopeStep => {
            let step = config.envelope_step.unwrap_or(solver_step);
            uniform_envelope_times(config.stop_time, step, abort)
        }
        EnvelopeAdaptiveMode::EventAlignedOnly => {
            let engine = Engine::new(build_engine_config(netlist, None));
            let mut times = engine
                .transient_source_event_times_with_abort(
                    netlist,
                    config.stop_time,
                    solver_step,
                    &config.modulation_sources,
                    abort,
                )
                .map_err(|error| {
                    ServiceRunError::from_core("Envelope source-event schedule error", error)
                })?;
            times.push(0.0);
            times.push(config.stop_time);
            times.sort_by(Value::total_cmp);
            times.dedup_by(|left, right| {
                let scale = left
                    .abs()
                    .max(right.abs())
                    .max(config.stop_time.abs())
                    .max(Value::MIN_POSITIVE);
                (*left - *right).abs() <= 64.0 * Value::EPSILON * scale
            });
            ensure_not_aborted(abort)?;
            Ok(times)
        }
    }
}

fn uniform_envelope_times(
    stop_time: Value,
    step: Value,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Value>> {
    let intervals = (stop_time / step).floor();
    if !intervals.is_finite() || intervals < 0.0 {
        return Err(ServiceRunError::Failure(
            "Envelope fixed output schedule cannot be represented".to_string(),
        ));
    }
    let interval_count = intervals as usize;
    let point_count = interval_count.checked_add(2).ok_or_else(|| {
        ServiceRunError::Failure("Envelope output schedule overflowed".to_string())
    })?;
    let limit = rspice_core::ResourceLimits::default().max_analysis_points;
    if point_count > limit {
        return Err(ServiceRunError::Failure(format!(
            "Envelope fixed output schedule requires {point_count} points, exceeding the configured analysis-point limit {limit}"
        )));
    }
    let mut times = Vec::with_capacity(point_count);
    for index in 0..=interval_count {
        poll_periodically(abort, index)?;
        let time = (index as Value * step).min(stop_time);
        times.push(time);
    }
    if times.last().is_none_or(|last| {
        stop_time - *last
            > 64.0 * Value::EPSILON * stop_time.abs().max(step.abs()).max(Value::MIN_POSITIVE)
    }) {
        times.push(stop_time);
    } else if let Some(last) = times.last_mut() {
        *last = stop_time;
    }
    Ok(times)
}

fn run_pss_initialized_envelope_transient(
    netlist: &rspice_core::Netlist,
    config: &EnvelopeRunConfig,
    step_time: Value,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<TransientData> {
    validate_commensurate_carriers(config, "PSS")?;

    let points_per_period = config.num_harmonics.saturating_mul(16).max(256);
    let pss_config = PssConfig::new(config.fundamental_freq)
        .with_harmonics(config.num_harmonics)
        .with_tstab(20.0 / config.fundamental_freq);
    let mut pss_config = pss_config;
    pss_config.points_per_period = points_per_period;

    let engine = Engine::new(build_engine_config(netlist, None));
    let (_, state) = engine
        .run_pss_with_frozen_source_continuation_state_abort(
            netlist,
            pss_config,
            &config.modulation_sources,
            abort,
        )
        .map_err(|error| ServiceRunError::from_core("Envelope PSS initialization error", error))?;
    let (result, _) = engine
        .run_tran_from_pss_state_with_abort(netlist, &state, config.stop_time, step_time, abort)
        .map_err(|error| ServiceRunError::from_core("Envelope PSS continuation error", error))?;
    let node_names = result.node_names.clone();
    TransientData::from_result_with_abort(result, &node_names, abort)
}

fn run_hb_initialized_envelope_transient(
    netlist: &rspice_core::Netlist,
    config: &EnvelopeRunConfig,
    step_time: Value,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<TransientData> {
    validate_commensurate_carriers(config, "HB")?;
    let hb_config = HbConfig::new(config.fundamental_freq).with_harmonics(config.num_harmonics);
    let engine = Engine::new(build_engine_config(netlist, None));
    let (_, state) = engine
        .run_hb_envelope_continuation_state_with_abort(
            netlist,
            hb_config.clone(),
            &config.modulation_sources,
            abort,
        )
        .map_err(|error| ServiceRunError::from_core("Envelope HB initialization error", error))?;
    let (result, _) = engine
        .run_tran_from_hb_envelope_state_with_abort(
            netlist,
            &hb_config,
            &config.modulation_sources,
            &state,
            config.stop_time,
            step_time,
            abort,
        )
        .map_err(|error| ServiceRunError::from_core("Envelope HB continuation error", error))?;
    let node_names = result.node_names.clone();
    TransientData::from_result_with_abort(result, &node_names, abort)
}

fn validate_commensurate_carriers(
    config: &EnvelopeRunConfig,
    initializer: &str,
) -> ServiceRunResult<()> {
    for tone in &config.additional_carrier_tones {
        let ratio = *tone / config.fundamental_freq;
        let nearest_harmonic = ratio.round();
        let tolerance = 128.0 * Value::EPSILON * ratio.abs().max(1.0);
        if nearest_harmonic < 1.0
            || (ratio - nearest_harmonic).abs() > tolerance
            || nearest_harmonic > config.num_harmonics as Value
        {
            return Err(ServiceRunError::Failure(format!(
                "Envelope {initializer} initialization requires every additional carrier to be an integer harmonic within order {}; tone {tone:.12e}Hz is not compatible with fundamental {:.12e}Hz",
                config.num_harmonics, config.fundamental_freq
            )));
        }
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum EnvelopeProjectionBasis {
    Dc,
    Cos(usize),
    Sin(usize),
}

fn compute_carrier_envelopes_with_abort(
    time: &[Value],
    values: &[Value],
    output_time: &[Value],
    carrier_frequencies: &[Value],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Vec<Complex64>>> {
    ensure_not_aborted(abort)?;
    if time.len() != values.len() || time.len() < 2 {
        return Err(ServiceRunError::Failure(
            "Envelope transient time/value vectors are inconsistent".to_string(),
        ));
    }
    if time.windows(2).any(|pair| pair[1] <= pair[0]) {
        return Err(ServiceRunError::Failure(
            "Envelope transient time axis is not strictly increasing".to_string(),
        ));
    }
    if time.iter().any(|sample| !sample.is_finite())
        || values.iter().any(|sample| !sample.is_finite())
    {
        return Err(ServiceRunError::Failure(
            "Envelope transient trajectory contains a non-finite sample".to_string(),
        ));
    }
    if carrier_frequencies.is_empty()
        || carrier_frequencies
            .iter()
            .any(|frequency| !frequency.is_finite() || *frequency <= 0.0)
    {
        return Err(ServiceRunError::Failure(
            "Envelope carrier projection requires finite positive carrier tones".to_string(),
        ));
    }

    let first_time = time[0];
    let last_time = *time.last().expect("length checked");
    let trajectory_span = last_time - first_time;
    validate_projection_workload(time.len(), output_time.len(), carrier_frequencies.len(), 1)?;
    let projection_window = carrier_projection_window(carrier_frequencies, abort)?;
    let span_tolerance = 128.0
        * Value::EPSILON
        * trajectory_span
            .abs()
            .max(projection_window.abs())
            .max(Value::MIN_POSITIVE);
    if trajectory_span + span_tolerance < projection_window {
        return Err(ServiceRunError::Failure(format!(
            "Envelope trajectory spans {trajectory_span:.12e}s, shorter than the {projection_window:.12e}s window required to resolve the configured carrier tones"
        )));
    }

    let basis = carrier_projection_basis(carrier_frequencies.len());
    let prefix_values =
        build_projection_prefixes(time, values, carrier_frequencies, &basis, abort)?;
    let projected_value_count = output_time
        .len()
        .checked_mul(carrier_frequencies.len())
        .and_then(|count| count.checked_mul(2))
        .ok_or_else(|| {
            ServiceRunError::Failure("Envelope result value count overflowed".to_string())
        })?;
    let result_limit = rspice_core::ResourceLimits::default().max_result_values;
    if projected_value_count > result_limit {
        return Err(ServiceRunError::Failure(format!(
            "Envelope extraction requires {projected_value_count} result values, exceeding the configured result-value limit {result_limit}"
        )));
    }

    let mut envelopes = (0..carrier_frequencies.len())
        .map(|_| Vec::with_capacity(output_time.len()))
        .collect::<Vec<_>>();
    for (index, center) in output_time.iter().copied().enumerate() {
        poll_periodically(abort, index)?;
        let half_window = 0.5 * projection_window;
        let center_tolerance =
            128.0 * Value::EPSILON * center.abs().max(last_time.abs()).max(Value::MIN_POSITIVE);
        if !center.is_finite()
            || center + center_tolerance < first_time + half_window
            || center - center_tolerance > last_time - half_window
        {
            return Err(ServiceRunError::Failure(format!(
                "Envelope output time {center:.12e}s cannot center the required {projection_window:.12e}s carrier-projection window"
            )));
        }
        let window_start = center - half_window;
        let window_stop = window_start + projection_window;
        let mut normal = vec![vec![0.0; basis.len()]; basis.len()];
        let mut rhs = vec![0.0; basis.len()];
        for row in 0..basis.len() {
            poll_periodically(abort, row)?;
            rhs[row] = (projection_prefix_at(
                time,
                values,
                &prefix_values[row],
                window_stop,
                carrier_frequencies,
                basis[row],
            )? - projection_prefix_at(
                time,
                values,
                &prefix_values[row],
                window_start,
                carrier_frequencies,
                basis[row],
            )?) / projection_window;
            for column in 0..=row {
                poll_periodically(abort, column)?;
                let value = projection_basis_product_integral(
                    basis[row],
                    basis[column],
                    carrier_frequencies,
                    window_start,
                    window_stop,
                ) / projection_window;
                normal[row][column] = value;
                normal[column][row] = value;
            }
        }
        let coefficients = solve_projection_system(normal, rhs, abort)?;
        for (tone_index, envelope) in envelopes.iter_mut().enumerate() {
            let cosine = coefficients[1 + 2 * tone_index];
            let sine = coefficients[2 + 2 * tone_index];
            envelope.push(Complex64::new(cosine, -sine));
        }
    }
    ensure_not_aborted(abort)?;
    Ok(envelopes)
}

fn carrier_projection_window(
    carrier_frequencies: &[Value],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Value> {
    let lowest_frequency = carrier_frequencies
        .iter()
        .copied()
        .fold(Value::INFINITY, Value::min);
    let mut window = 1.0 / lowest_frequency;
    for left in 0..carrier_frequencies.len() {
        poll_periodically(abort, left)?;
        for right in 0..left {
            poll_periodically(abort, right)?;
            let separation = (carrier_frequencies[left] - carrier_frequencies[right]).abs();
            window = window.max(2.0 / separation);
        }
    }
    Ok(window)
}

fn centered_projection_output_times(
    requested: &[Value],
    first_time: Value,
    last_time: Value,
    carrier_frequencies: &[Value],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Value>> {
    let window = carrier_projection_window(carrier_frequencies, abort)?;
    let span = last_time - first_time;
    let tolerance = 128.0 * Value::EPSILON * span.abs().max(window.abs()).max(Value::MIN_POSITIVE);
    if !span.is_finite() || span + tolerance < window {
        return Err(ServiceRunError::Failure(format!(
            "Envelope trajectory spans {span:.12e}s, shorter than the {window:.12e}s window required to resolve the configured carrier tones"
        )));
    }
    let midpoint = first_time + 0.5 * span;
    let (minimum_center, maximum_center) = if span >= window {
        (first_time + 0.5 * window, last_time - 0.5 * window)
    } else {
        (midpoint, midpoint)
    };
    let mut centered = Vec::with_capacity(requested.len());
    for (index, time) in requested.iter().copied().enumerate() {
        poll_periodically(abort, index)?;
        if !time.is_finite() || time < first_time || time > last_time {
            return Err(ServiceRunError::Failure(format!(
                "Envelope output time {time:.12e}s lies outside the transient trajectory"
            )));
        }
        let effective = time.clamp(minimum_center, maximum_center);
        let duplicate = centered.last().is_some_and(|previous: &Value| {
            (effective - *previous).abs()
                <= 64.0
                    * Value::EPSILON
                    * effective.abs().max(previous.abs()).max(Value::MIN_POSITIVE)
        });
        if !duplicate {
            centered.push(effective);
        }
    }
    if centered.is_empty() {
        return Err(ServiceRunError::Failure(
            "Envelope output schedule contains no centerable carrier-projection point".to_string(),
        ));
    }
    Ok(centered)
}

fn validate_projection_workload(
    sample_count: usize,
    output_count: usize,
    carrier_count: usize,
    signal_count: usize,
) -> ServiceRunResult<()> {
    let basis_count = carrier_count
        .checked_mul(2)
        .and_then(|count| count.checked_add(1))
        .ok_or_else(|| {
            ServiceRunError::Failure("Envelope projection basis size overflowed".to_string())
        })?;
    let matrix_values = basis_count.checked_mul(basis_count).ok_or_else(|| {
        ServiceRunError::Failure("Envelope projection matrix size overflowed".to_string())
    })?;
    let result_limit = rspice_core::ResourceLimits::default().max_result_values;
    if matrix_values > result_limit {
        return Err(ServiceRunError::Failure(format!(
            "Envelope projection matrix requires {matrix_values} values, exceeding the configured result-value limit {result_limit}"
        )));
    }
    let prefix_values = sample_count.checked_mul(basis_count).ok_or_else(|| {
        ServiceRunError::Failure("Envelope projection workspace size overflowed".to_string())
    })?;
    if prefix_values > result_limit {
        return Err(ServiceRunError::Failure(format!(
            "Envelope projection workspace requires {prefix_values} values, exceeding the configured result-value limit {result_limit}"
        )));
    }
    let prefix_work = prefix_values.checked_mul(signal_count).ok_or_else(|| {
        ServiceRunError::Failure("Envelope projection work estimate overflowed".to_string())
    })?;
    let solve_work = matrix_values
        .checked_mul(basis_count)
        .and_then(|work| work.checked_mul(output_count.max(1)))
        .and_then(|work| work.checked_mul(signal_count))
        .ok_or_else(|| {
            ServiceRunError::Failure("Envelope projection work estimate overflowed".to_string())
        })?;
    let estimated_work = prefix_work.checked_add(solve_work).ok_or_else(|| {
        ServiceRunError::Failure("Envelope projection work estimate overflowed".to_string())
    })?;
    if estimated_work > MAX_ENVELOPE_PROJECTION_WORK {
        return Err(ServiceRunError::Failure(format!(
            "Envelope projection requires an estimated {estimated_work} dense operations, exceeding the preview limit {MAX_ENVELOPE_PROJECTION_WORK}; reduce carrier tones or retained output points"
        )));
    }
    Ok(())
}

fn carrier_projection_basis(tone_count: usize) -> Vec<EnvelopeProjectionBasis> {
    let mut basis = Vec::with_capacity(1 + 2 * tone_count);
    basis.push(EnvelopeProjectionBasis::Dc);
    for tone in 0..tone_count {
        basis.push(EnvelopeProjectionBasis::Cos(tone));
        basis.push(EnvelopeProjectionBasis::Sin(tone));
    }
    basis
}

fn projection_basis_value(
    basis: EnvelopeProjectionBasis,
    carrier_frequencies: &[Value],
    time: Value,
) -> Value {
    match basis {
        EnvelopeProjectionBasis::Dc => 1.0,
        EnvelopeProjectionBasis::Cos(index) => {
            (2.0 * std::f64::consts::PI * carrier_frequencies[index] * time).cos()
        }
        EnvelopeProjectionBasis::Sin(index) => {
            (2.0 * std::f64::consts::PI * carrier_frequencies[index] * time).sin()
        }
    }
}

fn build_projection_prefixes(
    time: &[Value],
    values: &[Value],
    carrier_frequencies: &[Value],
    basis: &[EnvelopeProjectionBasis],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Vec<Value>>> {
    let prefix_value_count = time.len().checked_mul(basis.len()).ok_or_else(|| {
        ServiceRunError::Failure("Envelope projection workspace size overflowed".to_string())
    })?;
    let result_limit = rspice_core::ResourceLimits::default().max_result_values;
    if prefix_value_count > result_limit {
        return Err(ServiceRunError::Failure(format!(
            "Envelope projection workspace requires {prefix_value_count} values, exceeding the configured result-value limit {result_limit}"
        )));
    }
    let mut prefixes = basis
        .iter()
        .map(|_| {
            let mut prefix = Vec::with_capacity(time.len());
            prefix.push(0.0);
            prefix
        })
        .collect::<Vec<_>>();
    for sample in 1..time.len() {
        poll_periodically(abort, sample)?;
        let dt = time[sample] - time[sample - 1];
        for (basis_index, basis) in basis.iter().copied().enumerate() {
            let previous = values[sample - 1]
                * projection_basis_value(basis, carrier_frequencies, time[sample - 1]);
            let current =
                values[sample] * projection_basis_value(basis, carrier_frequencies, time[sample]);
            let next = prefixes[basis_index][sample - 1] + 0.5 * (previous + current) * dt;
            prefixes[basis_index].push(next);
        }
    }
    Ok(prefixes)
}

fn projection_prefix_at(
    time: &[Value],
    values: &[Value],
    prefix: &[Value],
    target: Value,
    carrier_frequencies: &[Value],
    basis: EnvelopeProjectionBasis,
) -> ServiceRunResult<Value> {
    match time.binary_search_by(|sample| sample.total_cmp(&target)) {
        Ok(index) => Ok(prefix[index]),
        Err(upper) if upper > 0 && upper < time.len() => {
            let lower = upper - 1;
            let span = time[upper] - time[lower];
            let fraction = (target - time[lower]) / span;
            let target_value = values[lower] + fraction * (values[upper] - values[lower]);
            let lower_mixed =
                values[lower] * projection_basis_value(basis, carrier_frequencies, time[lower]);
            let target_mixed =
                target_value * projection_basis_value(basis, carrier_frequencies, target);
            Ok(prefix[lower] + 0.5 * (lower_mixed + target_mixed) * (target - time[lower]))
        }
        _ => Err(ServiceRunError::Failure(format!(
            "Envelope window boundary {target:.12e}s cannot be interpolated from the transient trajectory"
        ))),
    }
}

fn projection_basis_product_integral(
    left: EnvelopeProjectionBasis,
    right: EnvelopeProjectionBasis,
    carrier_frequencies: &[Value],
    start: Value,
    stop: Value,
) -> Value {
    let omega = |index: usize| 2.0 * std::f64::consts::PI * carrier_frequencies[index];
    match (left, right) {
        (EnvelopeProjectionBasis::Dc, EnvelopeProjectionBasis::Dc) => stop - start,
        (EnvelopeProjectionBasis::Dc, EnvelopeProjectionBasis::Cos(index))
        | (EnvelopeProjectionBasis::Cos(index), EnvelopeProjectionBasis::Dc) => {
            integrate_cosine(omega(index), start, stop)
        }
        (EnvelopeProjectionBasis::Dc, EnvelopeProjectionBasis::Sin(index))
        | (EnvelopeProjectionBasis::Sin(index), EnvelopeProjectionBasis::Dc) => {
            integrate_sine(omega(index), start, stop)
        }
        (EnvelopeProjectionBasis::Cos(left), EnvelopeProjectionBasis::Cos(right)) => {
            0.5 * (integrate_cosine(omega(left) - omega(right), start, stop)
                + integrate_cosine(omega(left) + omega(right), start, stop))
        }
        (EnvelopeProjectionBasis::Sin(left), EnvelopeProjectionBasis::Sin(right)) => {
            0.5 * (integrate_cosine(omega(left) - omega(right), start, stop)
                - integrate_cosine(omega(left) + omega(right), start, stop))
        }
        (EnvelopeProjectionBasis::Cos(left), EnvelopeProjectionBasis::Sin(right)) => {
            0.5 * (integrate_sine(omega(right) + omega(left), start, stop)
                + integrate_sine(omega(right) - omega(left), start, stop))
        }
        (EnvelopeProjectionBasis::Sin(left), EnvelopeProjectionBasis::Cos(right)) => {
            0.5 * (integrate_sine(omega(left) + omega(right), start, stop)
                + integrate_sine(omega(left) - omega(right), start, stop))
        }
    }
}

fn integrate_cosine(omega: Value, start: Value, stop: Value) -> Value {
    let half_span = 0.5 * (stop - start);
    let midpoint = 0.5 * (start + stop);
    2.0 * half_span * sinc(omega * half_span) * (omega * midpoint).cos()
}

fn integrate_sine(omega: Value, start: Value, stop: Value) -> Value {
    let half_span = 0.5 * (stop - start);
    let midpoint = 0.5 * (start + stop);
    2.0 * half_span * sinc(omega * half_span) * (omega * midpoint).sin()
}

fn sinc(value: Value) -> Value {
    if value.abs() < 1.0e-5 {
        let square = value * value;
        1.0 - square / 6.0 + square * square / 120.0
    } else {
        value.sin() / value
    }
}

fn solve_projection_system(
    mut matrix: Vec<Vec<Value>>,
    mut rhs: Vec<Value>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Value>> {
    let dimension = rhs.len();
    let matrix_scale = matrix
        .iter()
        .flatten()
        .copied()
        .map(Value::abs)
        .fold(0.0, Value::max)
        .max(Value::MIN_POSITIVE);
    let pivot_floor = 4096.0 * Value::EPSILON * matrix_scale * dimension.max(1) as Value;
    for column in 0..dimension {
        poll_periodically(abort, column)?;
        let pivot_row = (column..dimension)
            .max_by(|left, right| {
                matrix[*left][column]
                    .abs()
                    .total_cmp(&matrix[*right][column].abs())
            })
            .expect("nonempty pivot range");
        let pivot = matrix[pivot_row][column].abs();
        if !pivot.is_finite() || pivot <= pivot_floor {
            return Err(ServiceRunError::Failure(
                "Envelope carrier tones cannot be separated on the available time window; increase Envelope stop or remove nearly coincident tones"
                    .to_string(),
            ));
        }
        matrix.swap(column, pivot_row);
        rhs.swap(column, pivot_row);
        for row in (column + 1)..dimension {
            poll_periodically(abort, row)?;
            let factor = matrix[row][column] / matrix[column][column];
            matrix[row][column] = 0.0;
            let (completed_rows, remaining_rows) = matrix.split_at_mut(row);
            let pivot_row = &completed_rows[column];
            let elimination_row = &mut remaining_rows[0];
            for (entry, pivot_entry) in elimination_row.iter_mut().zip(pivot_row).skip(column + 1) {
                *entry -= factor * *pivot_entry;
            }
            rhs[row] -= factor * rhs[column];
        }
    }
    let mut solution = vec![0.0; dimension];
    for row in (0..dimension).rev() {
        poll_periodically(abort, row)?;
        let tail = ((row + 1)..dimension)
            .map(|column| matrix[row][column] * solution[column])
            .sum::<Value>();
        solution[row] = (rhs[row] - tail) / matrix[row][row];
        if !solution[row].is_finite() {
            return Err(ServiceRunError::Failure(
                "Envelope carrier projection produced a non-finite coefficient".to_string(),
            ));
        }
    }
    Ok(solution)
}

/// Configuration for Fourier analysis.
#[derive(Debug, Clone)]
pub struct FourierRunConfig {
    pub fundamental_freq: Value,
    pub num_harmonics: usize,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub start_time: Value,
    pub stop_time: Value,
    pub compute_thd: bool,
    pub normalize: bool,
}

impl FourierRunConfig {
    pub(crate) fn validate(&self) -> Result<(), String> {
        if !self.fundamental_freq.is_finite() || self.fundamental_freq <= 0.0 {
            return Err("Fourier fundamental frequency must be positive".to_string());
        }
        if self.num_harmonics == 0 {
            return Err("Fourier num_harmonics must be greater than zero".to_string());
        }
        if self.output_node.trim().is_empty() {
            return Err("Fourier output node must be specified".to_string());
        }
        validate_fourier_output_accessor(&self.output_node, self.output_ref.as_deref())?;
        if !self.start_time.is_finite() || self.start_time < 0.0 {
            return Err("Fourier start_time must be >= 0".to_string());
        }
        if !self.stop_time.is_finite() || self.stop_time <= self.start_time {
            return Err("Fourier stop_time must be greater than start_time".to_string());
        }
        Ok(())
    }
}

pub(crate) fn fourier_output_is_current(output_node: &str) -> bool {
    let output = output_node.trim();
    output
        .get(..2)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("I("))
        && output.ends_with(')')
}

pub(crate) fn validate_fourier_output_accessor(
    output_node: &str,
    output_ref: Option<&str>,
) -> Result<(), String> {
    let output = output_node.trim();
    if output.is_empty() {
        return Err("Fourier output node must be specified".to_owned());
    }
    if fourier_output_is_current(output) {
        let device = output[2..output.len() - 1].trim();
        if device.is_empty() || device.contains(',') || device.contains('(') || device.contains(')')
        {
            return Err("Fourier current output must identify exactly one device".to_owned());
        }
        if output_ref.is_some_and(|reference| !reference.trim().is_empty()) {
            return Err("Fourier current output must not specify a voltage reference".to_owned());
        }
        return Ok(());
    }

    let node = if output
        .get(..2)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("V("))
        && output.ends_with(')')
    {
        output[2..output.len() - 1].trim()
    } else {
        if output.contains('(') || output.contains(')') {
            return Err(
                "Fourier output must be a node, V(node), or qualified I(device)".to_owned(),
            );
        }
        output
    };
    if node.is_empty() || node.contains(',') || node.contains('(') || node.contains(')') {
        return Err("Fourier voltage output must identify exactly one positive node".to_owned());
    }
    if let Some(reference) = output_ref {
        let reference = reference.trim();
        if reference.contains(',') || reference.contains('(') || reference.contains(')') {
            return Err("Fourier voltage reference must identify exactly one node".to_owned());
        }
    }
    Ok(())
}

/// Fourier analysis output.
#[derive(Debug, Clone)]
pub struct FourierData {
    pub frequencies: Vec<Value>,
    pub response: Vec<Complex64>,
    pub thd_percent: Option<Value>,
    pub dc_component: Value,
    pub output_label: String,
}

/// Run Fourier analysis by executing transient and computing harmonic decomposition.
pub fn run_fourier_analysis(
    netlist_text: &str,
    config: &FourierRunConfig,
) -> Result<FourierData, String> {
    run_fourier_analysis_with_abort(netlist_text, config, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run Fourier analysis with cooperative cancellation.
pub fn run_fourier_analysis_with_abort(
    netlist_text: &str,
    config: &FourierRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<FourierData> {
    run_fourier_analysis_with_source_path_and_abort(netlist_text, config, None, abort)
}

/// Run Fourier analysis with a source path used to resolve relative includes
/// and model file references.
pub fn run_fourier_analysis_with_source_path(
    netlist_text: &str,
    config: &FourierRunConfig,
    source_path: Option<&Path>,
) -> Result<FourierData, String> {
    run_fourier_analysis_with_source_path_and_abort(netlist_text, config, source_path, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run Fourier analysis with source-path resolution and cooperative
/// cancellation through transient solving and harmonic integration.
pub fn run_fourier_analysis_with_source_path_and_abort(
    netlist_text: &str,
    config: &FourierRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<FourierData> {
    ensure_not_aborted(abort)?;
    let validation = config.validate();
    ensure_not_aborted(abort)?;
    validation.map_err(ServiceRunError::Failure)?;

    let window = config.stop_time - config.start_time;
    let max_harmonic_freq = config.fundamental_freq * (config.num_harmonics as f64 + 1.0);
    let nyquist_oversample = 8.0;
    let fine_step = 1.0 / (max_harmonic_freq * nyquist_oversample);
    let coarse_step = window / 1500.0;
    let floor_step = window / 200_000.0;
    let step_time = fine_step
        .min(coarse_step)
        .max(floor_step)
        .min(config.stop_time);
    if !step_time.is_finite() || step_time <= 0.0 {
        return Err(ServiceRunError::Failure(
            "Fourier sampling step could not be represented".to_string(),
        ));
    }
    ensure_not_aborted(abort)?;

    let transient = run_transient_analysis_with_source_path_and_abort(
        netlist_text,
        config.stop_time,
        step_time,
        source_path,
        abort,
    )?;
    if transient.time.len() < 3 {
        return Err(ServiceRunError::Failure(
            "Fourier analysis requires at least 3 transient samples".to_string(),
        ));
    }

    let signal = extract_transient_signal_with_abort(
        &transient,
        &config.output_node,
        config.output_ref.as_deref(),
        abort,
    )?;
    run_fourier_from_signal_with_abort(&transient.time, &signal, config, abort)
}

/// Compute a Fourier result from one already-authenticated transient signal.
///
/// Prepared dependency consumers use this path so they share the standalone
/// Fourier implementation's validation, windowing, resource checks, and
/// cooperative cancellation without launching a replacement transient solve.
pub(crate) fn run_fourier_from_signal_with_abort(
    time: &[Value],
    signal: &[Value],
    config: &FourierRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<FourierData> {
    ensure_not_aborted(abort)?;
    let validation = config.validate();
    ensure_not_aborted(abort)?;
    validation.map_err(ServiceRunError::Failure)?;
    if time.len() != signal.len() {
        return Err(ServiceRunError::Failure(format!(
            "Fourier trajectory contains {} time samples and {} signal samples",
            time.len(),
            signal.len()
        )));
    }

    let (window_time, window_values) =
        exact_fourier_window(time, signal, config.start_time, config.stop_time, abort)?;
    if window_time.len() < 3 {
        return Err(ServiceRunError::Failure(
            "Fourier analysis window has insufficient samples".to_string(),
        ));
    }

    let observed_duration = window_time[window_time.len() - 1] - window_time[0];
    let fundamental_period = 1.0 / config.fundamental_freq;
    let duration_tolerance =
        16.0 * f64::EPSILON * observed_duration.abs().max(fundamental_period).max(1.0);
    if observed_duration + duration_tolerance < fundamental_period {
        return Err(ServiceRunError::Failure(format!(
            "Fourier artifact window spans {observed_duration:.12e}s, shorter than one fundamental period {fundamental_period:.12e}s"
        )));
    }
    let mut observed_max_interval = 0.0_f64;
    for (index, samples) in window_time.windows(2).enumerate() {
        poll_periodically(abort, index)?;
        observed_max_interval = observed_max_interval.max(samples[1] - samples[0]);
    }
    let maximum_frequency = config.fundamental_freq * (config.num_harmonics as f64 + 1.0);
    let required_interval = 1.0 / (maximum_frequency * 8.0);
    if observed_max_interval > required_interval * (1.0 + 16.0 * f64::EPSILON) {
        return Err(ServiceRunError::Failure(format!(
            "Fourier artifact sampling interval {observed_max_interval:.12e}s is too coarse for the requested harmonic basis; use {required_interval:.12e}s or less"
        )));
    }

    let mut decomposition = analyze_fourier_with_abort(
        &window_time,
        &window_values,
        config.fundamental_freq,
        config.num_harmonics,
        abort,
    )?;
    if config.normalize {
        normalize_fourier_decomposition(&mut decomposition)?;
    }
    let output_label = fourier_output_label(config);

    ensure_not_aborted(abort)?;
    Ok(FourierData {
        frequencies: decomposition.frequencies,
        response: decomposition.response,
        thd_percent: config.compute_thd.then_some(decomposition.thd_percent),
        dc_component: decomposition.dc_component,
        output_label,
    })
}

fn normalize_fourier_decomposition(
    decomposition: &mut FourierDecomposition,
) -> ServiceRunResult<()> {
    let fundamental_magnitude = decomposition
        .response
        .get(1)
        .map_or(0.0, |component| component.norm());
    if !fundamental_magnitude.is_finite() || fundamental_magnitude <= 1.0e-15 {
        return Err(ServiceRunError::Failure(
            "Fourier fundamental component is too small to normalize".to_owned(),
        ));
    }
    for component in &mut decomposition.response {
        *component /= fundamental_magnitude;
    }
    decomposition.dc_component /= fundamental_magnitude;
    Ok(())
}

fn exact_fourier_window(
    time: &[Value],
    signal: &[Value],
    start_time: Value,
    stop_time: Value,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<(Vec<Value>, Vec<Value>)> {
    ensure_not_aborted(abort)?;
    let Some((&first_time, &last_time)) = time.first().zip(time.last()) else {
        return Err(ServiceRunError::Failure(
            "Fourier trajectory is empty".to_owned(),
        ));
    };
    if start_time < first_time || stop_time > last_time {
        return Err(ServiceRunError::Failure(format!(
            "Fourier window [{start_time:.12e}, {stop_time:.12e}]s is outside the transient artifact range [{first_time:.12e}, {last_time:.12e}]s"
        )));
    }
    if time.windows(2).any(|pair| pair[1] <= pair[0]) {
        return Err(ServiceRunError::Failure(
            "Fourier trajectory time axis is not strictly increasing".to_owned(),
        ));
    }
    if time.iter().any(|value| !value.is_finite()) || signal.iter().any(|value| !value.is_finite())
    {
        return Err(ServiceRunError::Failure(
            "Fourier trajectory contains a non-finite sample".to_owned(),
        ));
    }

    let start_value = interpolated_signal_value(time, signal, start_time)?;
    let stop_value = interpolated_signal_value(time, signal, stop_time)?;
    let interior_count = time
        .iter()
        .filter(|sample| **sample > start_time && **sample < stop_time)
        .count();
    let mut window_time = Vec::with_capacity(interior_count.saturating_add(2));
    let mut window_values = Vec::with_capacity(interior_count.saturating_add(2));
    window_time.push(start_time);
    window_values.push(start_value);
    for (sample_idx, (&sample_time, &value)) in time.iter().zip(signal.iter()).enumerate() {
        poll_periodically(abort, sample_idx)?;
        if sample_time > start_time && sample_time < stop_time {
            window_time.push(sample_time);
            window_values.push(value);
        }
    }
    window_time.push(stop_time);
    window_values.push(stop_value);
    ensure_not_aborted(abort)?;
    Ok((window_time, window_values))
}

fn interpolated_signal_value(
    time: &[Value],
    signal: &[Value],
    target: Value,
) -> ServiceRunResult<Value> {
    match time.binary_search_by(|sample| sample.total_cmp(&target)) {
        Ok(index) => Ok(signal[index]),
        Err(upper) if upper > 0 && upper < time.len() => {
            let lower = upper - 1;
            let span = time[upper] - time[lower];
            let fraction = (target - time[lower]) / span;
            Ok(signal[lower] + fraction * (signal[upper] - signal[lower]))
        }
        _ => Err(ServiceRunError::Failure(format!(
            "Fourier window boundary {target:.12e}s cannot be interpolated from the transient artifact"
        ))),
    }
}

fn fourier_output_label(config: &FourierRunConfig) -> String {
    let output = config.output_node.trim();
    if fourier_output_is_current(output) {
        return format!("I({})", output[2..output.len() - 1].trim());
    }
    let voltage_node = if output
        .get(..2)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("V("))
        && output.ends_with(')')
    {
        output[2..output.len() - 1].trim()
    } else {
        output
    };
    if let Some(ref_node) = config.output_ref.as_deref() {
        if ref_node.trim().is_empty() || ref_node.trim().eq_ignore_ascii_case("0") {
            format!("V({voltage_node})")
        } else {
            format!("V({voltage_node}, {})", ref_node.trim())
        }
    } else {
        format!("V({voltage_node})")
    }
}

struct FourierDecomposition {
    frequencies: Vec<Value>,
    response: Vec<Complex64>,
    thd_percent: Value,
    dc_component: Value,
}

fn analyze_fourier_with_abort(
    time: &[Value],
    values: &[Value],
    fundamental_freq: Value,
    num_harmonics: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<FourierDecomposition> {
    use std::f64::consts::PI;

    ensure_not_aborted(abort)?;
    if time.len() != values.len() || time.len() < 2 {
        return Err(ServiceRunError::Failure(
            "Fourier time/value vectors are inconsistent".to_string(),
        ));
    }

    let t_end = *time.last().expect("length checked");
    let t_start = (t_end - 1.0 / fundamental_freq).max(time[0]);
    let mut start_idx = 0;
    for (idx, sample_time) in time.iter().enumerate() {
        poll_periodically(abort, idx)?;
        if *sample_time >= t_start {
            start_idx = idx;
            break;
        }
    }
    let end_idx = time.len() - 1;
    if end_idx <= start_idx.saturating_add(1) {
        return Err(ServiceRunError::Failure(
            "Fourier analysis period has insufficient samples".to_string(),
        ));
    }
    let time = &time[start_idx..=end_idx];
    let values = &values[start_idx..=end_idx];
    let origin = time[0];
    let duration = time[time.len() - 1] - origin;
    if !duration.is_finite() || duration <= 0.0 {
        return Err(ServiceRunError::Failure(
            "Fourier analysis period has invalid duration".to_string(),
        ));
    }

    let component_count = num_harmonics.checked_add(1).ok_or_else(|| {
        ServiceRunError::Failure("Fourier harmonic count exceeds this platform".to_string())
    })?;
    let mut frequencies = Vec::new();
    frequencies
        .try_reserve_exact(component_count)
        .map_err(|error| {
            ServiceRunError::Failure(format!(
                "Fourier frequency allocation for {component_count} components failed: {error}"
            ))
        })?;
    let mut response = Vec::new();
    response
        .try_reserve_exact(component_count)
        .map_err(|error| {
            ServiceRunError::Failure(format!(
                "Fourier response allocation for {component_count} components failed: {error}"
            ))
        })?;
    let mut harmonic_sum_sq = 0.0;
    let mut fundamental_magnitude = 0.0;
    let mut dc_component = 0.0;

    for harmonic in 0..=num_harmonics {
        ensure_not_aborted(abort)?;
        let frequency = harmonic as Value * fundamental_freq;
        let (real, imaginary) = if harmonic == 0 {
            let mut integral = 0.0;
            for idx in 1..time.len() {
                poll_periodically(abort, idx)?;
                let dt = time[idx] - time[idx - 1];
                integral += 0.5 * (values[idx] + values[idx - 1]) * dt;
            }
            (integral / duration, 0.0)
        } else {
            let omega = 2.0 * PI * frequency;
            let mut cosine_integral = 0.0;
            let mut sine_integral = 0.0;
            for idx in 1..time.len() {
                poll_periodically(abort, idx)?;
                let dt = time[idx] - time[idx - 1];
                let phase0 = omega * (time[idx - 1] - origin);
                let phase1 = omega * (time[idx] - origin);
                cosine_integral +=
                    0.5 * (values[idx - 1] * phase0.cos() + values[idx] * phase1.cos()) * dt;
                sine_integral +=
                    0.5 * (values[idx - 1] * phase0.sin() + values[idx] * phase1.sin()) * dt;
            }
            (
                2.0 * cosine_integral / duration,
                -2.0 * sine_integral / duration,
            )
        };
        let value = Complex64::new(real, imaginary);
        let magnitude = value.norm();
        if harmonic == 0 {
            dc_component = real;
        } else if harmonic == 1 {
            fundamental_magnitude = magnitude;
        } else {
            harmonic_sum_sq += magnitude * magnitude;
        }
        frequencies.push(frequency);
        response.push(value);
    }

    let thd_percent = if fundamental_magnitude > 1e-15 {
        harmonic_sum_sq.sqrt() / fundamental_magnitude * 100.0
    } else {
        0.0
    };
    ensure_not_aborted(abort)?;
    Ok(FourierDecomposition {
        frequencies,
        response,
        thd_percent,
        dc_component,
    })
}

fn extract_transient_signal_with_abort(
    transient: &TransientData,
    output_node: &str,
    output_ref: Option<&str>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Value>> {
    ensure_not_aborted(abort)?;
    let node = output_node.trim();
    if node.is_empty() {
        return Err(ServiceRunError::Failure(
            "Fourier output node is empty".to_string(),
        ));
    }

    let node_waveform =
        find_transient_waveform_with_abort(transient, node, abort)?.ok_or_else(|| {
            ServiceRunError::Failure(format!(
                "Fourier output node '{}' not found in transient data",
                node
            ))
        })?;

    if let Some(ref_name) = output_ref {
        let ref_name = ref_name.trim();
        if ref_name.is_empty() || ref_name.eq_ignore_ascii_case("0") {
            return copy_values_with_abort(node_waveform, abort);
        }
        let ref_waveform = find_transient_waveform_with_abort(transient, ref_name, abort)?
            .ok_or_else(|| {
                ServiceRunError::Failure(format!(
                    "Fourier output reference node '{}' not found in transient data",
                    ref_name
                ))
            })?;
        if ref_waveform.len() != node_waveform.len() {
            return Err(ServiceRunError::Failure(
                "Fourier node/reference waveform length mismatch".to_string(),
            ));
        }
        let mut differential = Vec::with_capacity(node_waveform.len());
        for (sample_idx, (value, reference)) in node_waveform.iter().zip(ref_waveform).enumerate() {
            poll_periodically(abort, sample_idx)?;
            differential.push(value - reference);
        }
        ensure_not_aborted(abort)?;
        return Ok(differential);
    }

    copy_values_with_abort(node_waveform, abort)
}

fn copy_values_with_abort(
    values: &[Value],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Value>> {
    ensure_not_aborted(abort)?;
    let mut copied = Vec::with_capacity(values.len());
    for (sample_idx, value) in values.iter().enumerate() {
        poll_periodically(abort, sample_idx)?;
        copied.push(*value);
    }
    ensure_not_aborted(abort)?;
    Ok(copied)
}

fn find_transient_waveform_with_abort<'a>(
    transient: &'a TransientData,
    node_name: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Option<&'a [Value]>> {
    ensure_not_aborted(abort)?;
    let target = normalize_waveform_node_name(node_name);
    for (name, values) in &transient.voltages {
        ensure_not_aborted(abort)?;
        let wf_node = normalize_waveform_node_name(name);
        if wf_node == target {
            return Ok(Some(values));
        }
    }
    Ok(None)
}

fn normalize_waveform_node_name(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.len() >= 3
        && (trimmed.starts_with("V(") || trimmed.starts_with("v("))
        && trimmed.ends_with(')')
    {
        return trimmed[2..trimmed.len() - 1].trim().to_ascii_uppercase();
    }
    trimmed.to_ascii_uppercase()
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    struct AbortOnPoll {
        abort_on: usize,
        polls: AtomicUsize,
    }

    impl AbortSignal for AbortOnPoll {
        fn is_aborted(&self) -> bool {
            self.polls.fetch_add(1, Ordering::Relaxed) + 1 >= self.abort_on
        }
    }

    #[test]
    fn carrier_envelope_polls_inside_large_sample_loops() {
        let abort = AbortOnPoll {
            abort_on: 3,
            polls: AtomicUsize::new(0),
        };
        let time = (0..=256)
            .map(|index| index as Value / 256.0)
            .collect::<Vec<_>>();
        let values = time
            .iter()
            .map(|time| (2.0 * PI * time).sin())
            .collect::<Vec<_>>();

        let result = compute_carrier_envelopes_with_abort(&time, &values, &[0.5], &[1.0], &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn carrier_envelope_recovers_sinusoid_amplitude_from_nonuniform_samples() {
        let carrier = 2.0e3;
        let amplitude = 3.25;
        let duration = 8.0 / carrier;
        let mut time = Vec::new();
        let mut current = 0.0;
        let mut index = 0usize;
        while current < duration {
            time.push(current);
            let jitter = 0.72 + 0.24 * ((index * 17 % 31) as Value / 30.0);
            current += jitter / (carrier * 96.0);
            index += 1;
        }
        time.push(duration);
        let values = time
            .iter()
            .map(|time| 1.75 + amplitude * (2.0 * PI * carrier * time + 0.37).sin())
            .collect::<Vec<_>>();
        let output = [
            0.5 / carrier,
            1.5 / carrier,
            4.0 / carrier,
            duration - 0.5 / carrier,
        ];

        let envelopes =
            compute_carrier_envelopes_with_abort(&time, &values, &output, &[carrier], &NoAbort)
                .expect("nonuniform carrier projection should succeed");
        let envelope = &envelopes[0];

        assert_eq!(envelope.len(), output.len());
        for measured in envelope {
            assert!(
                (measured.norm() - amplitude).abs() < 2.0e-3,
                "measured {measured}, expected {amplitude}"
            );
        }
    }

    #[test]
    fn joint_carrier_projection_separates_nonharmonic_tones() {
        let carriers = [1.0e3, 1.35e3];
        let duration = 12.0e-3;
        let time = (0..=4096)
            .map(|index| {
                let uniform = index as Value / 4096.0;
                duration * uniform.powf(1.003)
            })
            .collect::<Vec<_>>();
        let values = time
            .iter()
            .map(|time| {
                0.8 + 2.4 * (2.0 * PI * carriers[0] * time + 0.2).cos()
                    + 0.65 * (2.0 * PI * carriers[1] * time - 0.7).sin()
            })
            .collect::<Vec<_>>();
        let output = [6.0e-3];

        let envelopes =
            compute_carrier_envelopes_with_abort(&time, &values, &output, &carriers, &NoAbort)
                .expect("joint carrier projection should resolve both tones");

        assert!((envelopes[0][0].norm() - 2.4).abs() < 2.0e-4);
        assert!((envelopes[1][0].norm() - 0.65).abs() < 2.0e-4);
    }

    #[test]
    fn carrier_envelope_preserves_standard_complex_phase() {
        let carrier = 4.0e3;
        let amplitude = 1.8;
        let phase = 0.63;
        let duration = 10.0 / carrier;
        let time = (0..=2048)
            .map(|index| duration * index as Value / 2048.0)
            .collect::<Vec<_>>();
        let values = time
            .iter()
            .map(|time| amplitude * (2.0 * PI * carrier * time + phase).cos())
            .collect::<Vec<_>>();

        let envelopes = compute_carrier_envelopes_with_abort(
            &time,
            &values,
            &[duration * 0.5],
            &[carrier],
            &NoAbort,
        )
        .expect("complex carrier projection should succeed");
        let measured = envelopes[0][0];
        assert!((measured.norm() - amplitude).abs() < 1.0e-5);
        assert!((measured.arg() - phase).abs() < 1.0e-5);
    }

    #[test]
    fn projection_workload_rejects_excessive_dense_work_before_allocation() {
        let error = validate_projection_workload(2, 10_000, 64, 1)
            .expect_err("excessive cubic work must fail closed");
        assert!(error.to_string().contains("dense operations"));
    }

    #[test]
    fn projection_workload_accounts_for_every_extracted_signal() {
        validate_projection_workload(2, 100_000, 2, 1)
            .expect("one small-basis signal remains within the preview budget");
        let error = validate_projection_workload(2, 100_000, 2, 9)
            .expect_err("the same projection across nine signals must include all work");
        assert!(error.to_string().contains("dense operations"));
    }

    #[test]
    fn envelope_result_limit_counts_all_retained_time_and_complex_vectors() {
        assert_eq!(
            envelope_retained_value_count(11, 4).expect("small result count"),
            11 * (1 + 3 * 4)
        );
        assert!(envelope_retained_value_count(usize::MAX, 1).is_err());
        assert!(envelope_retained_value_count(1, usize::MAX).is_err());
    }

    #[test]
    fn boundary_output_times_are_relabelled_to_exact_window_centers() {
        let centered = centered_projection_output_times(
            &[0.0, 0.25, 0.5, 0.75, 1.0],
            0.0,
            1.0,
            &[2.0],
            &NoAbort,
        )
        .expect("output centers should be representable");

        assert_eq!(centered, vec![0.25, 0.5, 0.75]);
    }

    #[test]
    fn joint_carrier_projection_rejects_an_unresolvable_short_window() {
        let carriers = [1.0e3, 1.01e3];
        let time = (0..=256)
            .map(|index| index as Value * 10.0e-6)
            .collect::<Vec<_>>();
        let values = vec![0.0; time.len()];

        let error =
            compute_carrier_envelopes_with_abort(&time, &values, &[1.0e-3], &carriers, &NoAbort)
                .expect_err("short trajectory cannot discriminate nearly coincident tones");

        assert!(error.to_string().contains("required to resolve"));
    }

    #[test]
    fn fixed_envelope_schedule_preserves_requested_spacing_and_stop() {
        let times = uniform_envelope_times(1.0, 0.3, &NoAbort)
            .expect("fixed envelope schedule should be representable");

        assert_eq!(times.len(), 5);
        for (actual, expected) in times.iter().zip([0.0, 0.3, 0.6, 0.9, 1.0]) {
            assert!((actual - expected).abs() <= 4.0 * Value::EPSILON);
        }
    }

    #[test]
    fn fourier_integration_polls_inside_harmonic_sample_loops() {
        let abort = AbortOnPoll {
            abort_on: 6,
            polls: AtomicUsize::new(0),
        };
        let time: Vec<Value> = (0..256).map(|idx| idx as Value * 1.0e-9).collect();
        let values: Vec<Value> = time
            .iter()
            .map(|time| (2.0 * std::f64::consts::PI * 10.0e6 * time).sin())
            .collect();

        let result = analyze_fourier_with_abort(&time, &values, 10.0e6, 10, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn cancellable_fourier_matches_core_harmonic_decomposition() {
        use rspice_core::analysis::{FourierAnalysis, FourierConfig};

        let fundamental = 10.0e6;
        let time: Vec<Value> = (0..512).map(|idx| idx as Value * 1.0e-9).collect();
        let values: Vec<Value> = time
            .iter()
            .map(|time| {
                0.25 + (2.0 * std::f64::consts::PI * fundamental * time).sin()
                    + 0.1 * (4.0 * std::f64::consts::PI * fundamental * time).cos()
            })
            .collect();
        let expected = FourierAnalysis::new(FourierConfig::new(fundamental).with_harmonics(6))
            .analyze(&time, &values);

        let actual = analyze_fourier_with_abort(&time, &values, fundamental, 6, &NoAbort)
            .expect("cancellable decomposition should succeed");

        assert_eq!(actual.response.len(), expected.harmonics.len());
        assert!((actual.dc_component - expected.dc_component).abs() < 1.0e-12);
        assert!((actual.thd_percent - expected.thd).abs() < 1.0e-9);
        for (value, harmonic) in actual.response.iter().zip(&expected.harmonics) {
            let expected_value =
                Complex64::from_polar(harmonic.magnitude, harmonic.phase.to_radians());
            assert!((value.re - expected_value.re).abs() < 1.0e-10);
            assert!((value.im - expected_value.im).abs() < 1.0e-10);
        }
    }

    #[test]
    fn cancellation_precedes_invalid_fourier_configuration() {
        let config = FourierRunConfig {
            fundamental_freq: -1.0,
            num_harmonics: 0,
            output_node: String::new(),
            output_ref: None,
            start_time: -1.0,
            stop_time: -1.0,
            compute_thd: true,
            normalize: false,
        };
        let abort = AbortOnPoll {
            abort_on: 2,
            polls: AtomicUsize::new(0),
        };

        let result = run_fourier_analysis_with_abort("invalid", &config, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn current_fourier_preserves_the_qualified_output_label() {
        let time = (0..=100)
            .map(|index| index as Value / 100.0)
            .collect::<Vec<_>>();
        let signal = time
            .iter()
            .map(|time| (2.0 * PI * time).sin())
            .collect::<Vec<_>>();
        let config = FourierRunConfig {
            fundamental_freq: 1.0,
            num_harmonics: 1,
            output_node: "I(V1)".to_owned(),
            output_ref: None,
            start_time: 0.0,
            stop_time: 1.0,
            compute_thd: true,
            normalize: false,
        };

        let result = run_fourier_from_signal_with_abort(&time, &signal, &config, &NoAbort)
            .expect("current Fourier decomposition should succeed");

        assert_eq!(result.output_label, "I(V1)");
    }

    #[test]
    fn thd_retention_and_fundamental_normalization_are_independent_controls() {
        let time = (0..=800)
            .map(|index| index as Value / 800.0)
            .collect::<Vec<_>>();
        let signal = time
            .iter()
            .map(|time| 0.25 + 2.0 * (2.0 * PI * time).sin() + 0.5 * (4.0 * PI * time).sin())
            .collect::<Vec<_>>();
        let config = |compute_thd, normalize| FourierRunConfig {
            fundamental_freq: 1.0,
            num_harmonics: 2,
            output_node: "V(out)".to_owned(),
            output_ref: None,
            start_time: 0.0,
            stop_time: 1.0,
            compute_thd,
            normalize,
        };

        let raw =
            run_fourier_from_signal_with_abort(&time, &signal, &config(true, false), &NoAbort)
                .expect("raw Fourier decomposition");
        let normalized =
            run_fourier_from_signal_with_abort(&time, &signal, &config(false, true), &NoAbort)
                .expect("normalized Fourier decomposition");

        assert!(raw.thd_percent.is_some());
        assert_eq!(normalized.thd_percent, None);
        assert!((raw.response[1].norm() - 2.0).abs() < 1.0e-9);
        assert!((normalized.response[1].norm() - 1.0).abs() < 1.0e-9);
        assert!((normalized.response[2].norm() - 0.25).abs() < 1.0e-9);
        assert!((normalized.dc_component - 0.125).abs() < 1.0e-9);
    }

    #[test]
    fn unaligned_exact_period_window_interpolates_both_boundaries() {
        let time = (0..=101)
            .map(|index| index as Value / 100.0)
            .collect::<Vec<_>>();
        let signal = time
            .iter()
            .map(|time| (2.0 * PI * time).sin())
            .collect::<Vec<_>>();
        let config = FourierRunConfig {
            fundamental_freq: 1.0,
            num_harmonics: 1,
            output_node: "V(out)".to_owned(),
            output_ref: None,
            start_time: 0.005,
            stop_time: 1.005,
            compute_thd: true,
            normalize: false,
        };

        let result = run_fourier_from_signal_with_abort(&time, &signal, &config, &NoAbort)
            .expect("unaligned one-period window should be interpolated exactly");

        assert_eq!(result.output_label, "V(out)");
        assert_eq!(result.frequencies.len(), 2);
    }

    #[test]
    fn current_fourier_rejects_a_voltage_reference() {
        let config = FourierRunConfig {
            fundamental_freq: 1.0,
            num_harmonics: 1,
            output_node: "I(V1)".to_owned(),
            output_ref: Some("0".to_owned()),
            start_time: 0.0,
            stop_time: 1.0,
            compute_thd: true,
            normalize: false,
        };

        assert_eq!(
            config
                .validate()
                .expect_err("current references are invalid"),
            "Fourier current output must not specify a voltage reference"
        );
    }
}
