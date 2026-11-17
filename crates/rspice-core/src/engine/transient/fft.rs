//! Typed `.FFT` post-processing over accepted transient histories.

use super::super::{
    Engine, SimulationError, TransientFftBin, TransientFftHarmonic, TransientFftMetrics,
    TransientFftResult, TransientFftStatus, TransientResult,
};
use crate::abort_signal::AbortSignal;
use crate::netlist::{FftAnalysis, FftFormat, FftWindow, Netlist, XyceFftMode};
use crate::numerics::rustfft_qualification::qualify_rustfft_forward_length;
use crate::{ResourceKind, ResourceLimitError, ResourceLimits, Value};
use rustfft::{FftPlanner, num_complex::Complex};
use std::f64::consts::PI;

#[cfg(test)]
mod current_impulse_tests;
mod current_impulses;
mod window;
mod window_derivatives;

pub(super) const FFT_RETAINED_VALUES_PER_BIN: usize = 5;
pub(super) const FFT_RETAINED_VALUES_PER_HARMONIC: usize = 4;
pub(super) const FFT_RETAINED_METRIC_VALUES: usize = 8;
pub(super) const FFT_MAX_RANKED_HARMONICS: usize = 30;
const FFT_DB_NOISE_FLOOR: Value = 1.0e-10;

/// Xyce enables exact FFT sample-time stops unless explicitly disabled. Its
/// output interval schedule is interpolation-only and makes the modes
/// mutually exclusive.
pub(super) fn uses_accurate_sampling(netlist: &Netlist) -> bool {
    netlist.options.fft_accurate.unwrap_or(true)
        && netlist.options.output_interval_schedule.is_none()
}

pub(super) fn sample_time(analysis: &FftAnalysis, transient_stop: Value, sample: usize) -> Value {
    uniform_time(
        analysis.start.unwrap_or(0.0),
        analysis.stop.unwrap_or(transient_stop),
        analysis.points,
        sample,
    )
}

fn uniform_time(start: Value, stop: Value, points: usize, sample: usize) -> Value {
    let interval = (stop - start) / points as Value;
    start + sample as Value * interval
}

pub(super) fn preflight(
    engine: &Engine,
    netlist: &Netlist,
    transient_stop: Value,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    let mut retained_values = 0usize;
    for (index, analysis) in netlist.fft_analyses.iter().enumerate() {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        validate_request(index, analysis, transient_stop)?;
        qualify_rustfft_forward_length(analysis.points).map_err(|error| {
            SimulationError::Circuit(format!(
                ".FFT request {} cannot create a qualified transform plan: {error}",
                index + 1
            ))
        })?;
        let mut analysis_values =
            (analysis.points / 2 + 1).saturating_mul(FFT_RETAINED_VALUES_PER_BIN);
        if netlist.options.fft_output_metrics.unwrap_or(false) {
            analysis_values = analysis_values
                .saturating_add(FFT_RETAINED_METRIC_VALUES)
                .saturating_add(
                    (analysis.points / 2)
                        .min(FFT_MAX_RANKED_HARMONICS)
                        .saturating_mul(FFT_RETAINED_VALUES_PER_HARMONIC),
                );
        }
        retained_values = retained_values.saturating_add(analysis_values);
    }
    ResourceLimitError::ensure(
        ResourceKind::ResultValues,
        retained_values,
        engine.config.resource_limits.max_result_values,
    )?;
    Ok(())
}

pub(super) fn evaluate(
    engine: &Engine,
    netlist: &Netlist,
    result: &TransientResult,
    transient_stop: Value,
    abort: &dyn AbortSignal,
) -> Result<Vec<TransientFftResult>, SimulationError> {
    if netlist.fft_analyses.is_empty() {
        return Ok(Vec::new());
    }
    validate_history(result)?;
    let mut planner = FftPlanner::<Value>::new();
    let mode = netlist.options.fft_mode.unwrap_or_default();
    let accurate_sampling = uses_accurate_sampling(netlist);
    let emit_metrics = netlist.options.fft_output_metrics.unwrap_or(false);
    let mut spectra = Vec::new();
    spectra
        .try_reserve_exact(netlist.fft_analyses.len())
        .map_err(|_| allocation_error("result list"))?;
    for (index, analysis) in netlist.fft_analyses.iter().enumerate() {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let (output_name, physical_type, values) =
            crate::analysis::measure_signals::evaluate_tran_fft_output_with_abort(
                netlist,
                result,
                index,
                engine.config.resource_limits,
                abort,
            )?;
        spectra.push(evaluate_one(
            index,
            analysis,
            mode,
            accurate_sampling,
            emit_metrics,
            output_name,
            physical_type,
            &result.time,
            &values,
            Some((netlist, result)),
            transient_stop,
            &mut planner,
            engine.config.resource_limits.max_result_values,
            abort,
        )?);
    }
    Ok(spectra)
}

impl Engine {
    /// Evaluate authored `.FFT` requests against a retained transient result.
    /// Uses the same checked window, physical-current, normalization and
    /// resource contracts as FFT post-processing during a transient run.
    pub fn evaluate_transient_fft_results(
        &self,
        netlist: &Netlist,
        result: &TransientResult,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<TransientFftResult>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if netlist.fft_analyses.is_empty() {
            return Ok(Vec::new());
        }
        let stop = result.time.last().copied().ok_or_else(|| {
            SimulationError::Circuit(".FFT requires a nonempty transient history".into())
        })?;
        preflight(self, netlist, stop, abort)?;
        evaluate(self, netlist, result, stop, abort)
    }
}

/// A transient's `.FFT` spectra taken from its rows a span at a time, in one pass in time order:
/// each card's operand is evaluated over a span's rows and sampled onto the card's uniform
/// record, so that only the records are held. The spectra equal the ones the rows read back
/// whole give ([`Engine::finish_transient_from_rows`]).
pub struct TransientFftStream {
    limits: ResourceLimits,
    transient_stop: Value,
    mode: XyceFftMode,
    accurate_sampling: bool,
    emit_metrics: bool,
    /// The last row's time, which the next span's rows follow.
    last_time: Option<Value>,
    cards: Vec<FftCard>,
}

/// One card of a [`TransientFftStream`].
struct FftCard {
    status: TransientFftStatus,
    /// The operand's column name and physical type, once a span has given them.
    output: Option<(String, &'static str)>,
    record: UniformRecord,
}

impl Engine {
    /// A pass that takes `netlist`'s `.FFT` spectra from the rows of a transient run to
    /// `transient_stop`, whose first and last rows lie at `history`; `shell` is the run's result.
    /// `None` when the spectra need the rows in memory: the run has impulses, which a spectrum
    /// adds, or an operand reads rows other than its own (`sdt`).
    pub fn transient_fft_stream(
        &self,
        netlist: &Netlist,
        shell: &TransientResult,
        transient_stop: Value,
        history: (Value, Value),
    ) -> Result<Option<TransientFftStream>, SimulationError> {
        if shell.current_impulses.is_some()
            || shell.voltage_impulses.is_some()
            || (0..netlist.fft_analyses.len()).any(|index| {
                crate::analysis::measure_signals::tran_fft_operand_reads_other_rows(netlist, index)
            })
        {
            return Ok(None);
        }
        let engine = self.resolved_for_netlist(netlist);
        let mode = netlist.options.fft_mode.unwrap_or_default();
        let mut cards = Vec::new();
        cards
            .try_reserve_exact(netlist.fft_analyses.len())
            .map_err(|_| allocation_error("result list"))?;
        for (index, analysis) in netlist.fft_analyses.iter().enumerate() {
            validate_request(index, analysis, transient_stop)?;
            let status = record_status(analysis, transient_stop, history);
            let record = UniformRecord::new(index, analysis, mode, transient_stop, &status)?;
            cards.push(FftCard {
                status,
                output: None,
                record,
            });
        }
        Ok(Some(TransientFftStream {
            limits: engine.config.resource_limits,
            transient_stop,
            mode,
            accurate_sampling: uses_accurate_sampling(netlist),
            emit_metrics: netlist.options.fft_output_metrics.unwrap_or(false),
            last_time: None,
            cards,
        }))
    }
}

impl TransientFftStream {
    /// Sample the rows `span` holds, which follow the ones given before, onto each card's record.
    pub fn rows(
        &mut self,
        netlist: &Netlist,
        span: &TransientResult,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        for &time in &span.time {
            if !time.is_finite() || self.last_time.is_some_and(|last| time <= last) {
                return Err(SimulationError::Circuit(
                    ".FFT requires finite, strictly increasing transient times".to_string(),
                ));
            }
            self.last_time = Some(time);
        }
        for (index, card) in self.cards.iter_mut().enumerate() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let (name, physical_type, values) =
                crate::analysis::measure_signals::evaluate_tran_fft_output_with_abort(
                    netlist,
                    span,
                    index,
                    self.limits,
                    abort,
                )?;
            if values.len() != span.time.len() {
                return Err(request_error(
                    index,
                    format!(
                        "resolved output has {} values for {} transient samples",
                        values.len(),
                        span.time.len()
                    ),
                ));
            }
            match &card.output {
                None => card.output = Some((name, physical_type)),
                Some(output) if *output == (name, physical_type) => {}
                Some(_) => {
                    return Err(request_error(
                        index,
                        "resolved output names another column over later rows".to_owned(),
                    ));
                }
            }
            card.record.push(&span.time, &values, abort)?;
        }
        Ok(())
    }

    /// Each card's spectrum, once every row has been given.
    pub fn finish(
        self,
        netlist: &Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<TransientFftResult>, SimulationError> {
        if self.last_time.is_none() {
            return Err(SimulationError::Circuit(
                ".FFT requires at least one retained transient sample".to_string(),
            ));
        }
        let mut planner = FftPlanner::<Value>::new();
        let mut spectra = Vec::new();
        spectra
            .try_reserve_exact(self.cards.len())
            .map_err(|_| allocation_error("result list"))?;
        for (index, (analysis, card)) in netlist.fft_analyses.iter().zip(self.cards).enumerate() {
            let output = card
                .output
                .ok_or_else(|| request_error(index, "resolved output has no values".to_owned()))?;
            spectra.push(transform(
                index,
                analysis,
                self.mode,
                self.accurate_sampling,
                self.emit_metrics,
                output,
                card.status,
                card.record,
                &[],
                self.transient_stop,
                &mut planner,
                self.limits.max_result_values,
                abort,
            )?);
        }
        Ok(spectra)
    }
}

fn validate_request(
    index: usize,
    analysis: &FftAnalysis,
    transient_stop: Value,
) -> Result<(), SimulationError> {
    if analysis.points < 4 || !analysis.points.is_power_of_two() {
        return Err(request_error(
            index,
            format!(
                "NP must be a power of two no smaller than 4, found {}",
                analysis.points
            ),
        ));
    }
    let start = analysis.start.unwrap_or(0.0);
    let stop = analysis.stop.unwrap_or(transient_stop);
    if !start.is_finite() || start < 0.0 {
        return Err(request_error(
            index,
            format!("START must be finite and non-negative, found {start}"),
        ));
    }
    if !stop.is_finite() || stop <= start {
        return Err(request_error(
            index,
            format!("STOP must be finite and greater than START ({start}), found {stop}"),
        ));
    }
    if stop > transient_stop {
        return Err(request_error(
            index,
            format!("STOP {stop} exceeds transient stop time {transient_stop}"),
        ));
    }
    for (name, value, allow_zero) in [
        ("FREQ", analysis.fundamental_frequency, false),
        ("FMIN", analysis.minimum_frequency, true),
        ("FMAX", analysis.maximum_frequency, false),
    ] {
        if let Some(value) = value
            && (!value.is_finite() || value < 0.0 || (!allow_zero && value == 0.0))
        {
            return Err(request_error(
                index,
                format!(
                    "{name} must be a {}finite frequency, found {value}",
                    if allow_zero {
                        "non-negative "
                    } else {
                        "positive "
                    }
                ),
            ));
        }
    }
    if let (Some(minimum), Some(maximum)) = (analysis.minimum_frequency, analysis.maximum_frequency)
        && minimum > maximum
    {
        return Err(request_error(
            index,
            format!("FMIN {minimum} exceeds FMAX {maximum}"),
        ));
    }
    if !analysis.alpha.is_finite() || !(1.0..=20.0).contains(&analysis.alpha) {
        return Err(request_error(
            index,
            format!(
                "ALFA must be finite and between 1 and 20, found {}",
                analysis.alpha
            ),
        ));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn evaluate_one(
    index: usize,
    analysis: &FftAnalysis,
    mode: XyceFftMode,
    accurate_sampling: bool,
    emit_metrics: bool,
    output_name: String,
    physical_type: &'static str,
    time: &[Value],
    values: &[Value],
    current_record: Option<(&Netlist, &TransientResult)>,
    transient_stop: Value,
    planner: &mut FftPlanner<Value>,
    max_values: usize,
    abort: &dyn AbortSignal,
) -> Result<TransientFftResult, SimulationError> {
    validate_request(index, analysis, transient_stop)?;
    if values.len() != time.len() {
        return Err(request_error(
            index,
            format!(
                "resolved output has {} values for {} transient samples",
                values.len(),
                time.len()
            ),
        ));
    }
    let start = analysis.start.unwrap_or(0.0);
    let stop = analysis.stop.unwrap_or(transient_stop);
    let history_stop = *time.last().expect("validated non-empty history");
    let status = record_status(analysis, transient_stop, (time[0], history_stop));
    let impulses = if status.is_complete()
        && let Some((netlist, result)) = current_record
    {
        use crate::analysis::measure_signals::impulse_observation;
        let spec = match &analysis.output {
            crate::netlist::FftOutput::Probe(spec) => spec.clone(),
            crate::netlist::FftOutput::Expression(expression) => format!("{{{expression}}}"),
        };
        let observation = impulse_observation::resolve_with_coverage(
            Some(netlist),
            result,
            &spec,
            (start, stop),
            abort,
        )
        .map_err(|error| impulse_observation_error(index, error))?;
        // Finite DFT samples stop one spacing before STOP. Complete current
        // coverage additionally needs every accepted event through STOP.
        if observation.observes_signal && stop > history_stop {
            return Err(request_error(index,
                "current impulse history does not cover the entire requested (START, STOP] interval".into()));
        }
        observation.terms
    } else {
        Vec::new()
    };

    let mut record = UniformRecord::new(index, analysis, mode, transient_stop, &status)?;
    record.push(time, values, abort)?;
    transform(
        index,
        analysis,
        mode,
        accurate_sampling,
        emit_metrics,
        (output_name, physical_type),
        status,
        record,
        &impulses,
        transient_stop,
        planner,
        max_values,
        abort,
    )
}

/// Whether a card's uniform record lies inside the run's rows, which span `history`.
fn record_status(
    analysis: &FftAnalysis,
    transient_stop: Value,
    history: (Value, Value),
) -> TransientFftStatus {
    let (history_start, history_stop) = history;
    let first_target = analysis.start.unwrap_or(0.0);
    let last_target = sample_time(analysis, transient_stop, analysis.points - 1);
    if first_target < history_start || last_target > history_stop {
        TransientFftStatus::IncompleteHistory {
            available_start: history_start,
            available_stop: history_stop,
        }
    } else {
        TransientFftStatus::Complete
    }
}

/// One card's uniform record, sampled from a run's rows as they are fed in time order: a sample
/// time takes the row there, or interpolates linearly between the two rows around it. Rows fed
/// a span at a time give the record the whole run gives.
struct UniformRecord {
    index: usize,
    start: Value,
    stop: Value,
    points: usize,
    window: FftWindow,
    alpha: Value,
    denominator: Value,
    /// The record lies inside the rows; one that does not takes no samples.
    complete: bool,
    /// The next sample to take.
    sample: usize,
    /// The last row fed, as (time, value).
    previous: Option<(Value, Value)>,
    /// Each sample taken, weighted by its window coefficient.
    input: Vec<Complex<Value>>,
}

impl UniformRecord {
    fn new(
        index: usize,
        analysis: &FftAnalysis,
        mode: XyceFftMode,
        transient_stop: Value,
        status: &TransientFftStatus,
    ) -> Result<Self, SimulationError> {
        let complete = status.is_complete();
        let points = analysis.points;
        let mut input = Vec::new();
        input
            .try_reserve_exact(if complete { points } else { 0 })
            .map_err(|_| allocation_error("uniform input record"))?;
        Ok(Self {
            index,
            start: analysis.start.unwrap_or(0.0),
            stop: analysis.stop.unwrap_or(transient_stop),
            points,
            window: analysis.window,
            alpha: analysis.alpha,
            denominator: if mode.uses_periodic_windows() {
                points as Value
            } else {
                (points - 1) as Value
            },
            complete,
            sample: 0,
            previous: None,
            input,
        })
    }

    fn time(&self, sample: usize) -> Value {
        uniform_time(self.start, self.stop, self.points, sample)
    }

    fn coefficient(&self, sample: usize) -> Value {
        transient_fft_window_coefficient_with_alpha(
            self.window,
            sample,
            self.denominator,
            self.alpha,
        )
    }

    /// Take every sample at or before the last of these rows, which follow the ones fed before.
    fn push(
        &mut self,
        time: &[Value],
        values: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        if !self.complete {
            return Ok(());
        }
        for (&row_time, &row_value) in time.iter().zip(values) {
            while self.sample < self.points {
                let target = self.time(self.sample);
                if target > row_time {
                    break;
                }
                if self.sample.is_multiple_of(64) && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                // The rows before this one lie before the target; with none, the target lies
                // at or before the first row and is taken only there.
                let value = if target == row_time {
                    Some(row_value)
                } else {
                    self.previous.map(|(left_time, left_value)| {
                        let fraction = (target - left_time) / (row_time - left_time);
                        left_value + fraction * (row_value - left_value)
                    })
                };
                let value = value.ok_or_else(|| self.uninterpolated(target))?;
                if !value.is_finite() {
                    return Err(request_error(
                        self.index,
                        format!(
                            "resolved output is non-finite at uniform sample {} (time {target})",
                            self.sample
                        ),
                    ));
                }
                self.input
                    .push(Complex::new(value * self.coefficient(self.sample), 0.0));
                self.sample += 1;
            }
            self.previous = Some((row_time, row_value));
        }
        Ok(())
    }

    fn uninterpolated(&self, target: Value) -> SimulationError {
        request_error(
            self.index,
            format!("cannot interpolate the resolved output at time {target}"),
        )
    }

    /// The samples taken and the window's coherent gain, once every row is fed.
    fn finish(
        self,
        abort: &dyn AbortSignal,
    ) -> Result<(Vec<Complex<Value>>, Value), SimulationError> {
        if self.complete && self.sample < self.points {
            return Err(self.uninterpolated(self.time(self.sample)));
        }
        let mut window_sum = 0.0;
        for sample in 0..self.points {
            if sample.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            window_sum += self.coefficient(sample);
        }
        Ok((self.input, window_sum / self.points as Value))
    }
}

/// A card's spectrum from its uniform `record`.
#[allow(clippy::too_many_arguments)]
fn transform(
    index: usize,
    analysis: &FftAnalysis,
    mode: XyceFftMode,
    accurate_sampling: bool,
    emit_metrics: bool,
    (output_name, physical_type): (String, &'static str),
    status: TransientFftStatus,
    record: UniformRecord,
    impulses: &[crate::analysis::measure_signals::impulse_observation::ImpulseContribution<'_>],
    transient_stop: Value,
    planner: &mut FftPlanner<Value>,
    max_values: usize,
    abort: &dyn AbortSignal,
) -> Result<TransientFftResult, SimulationError> {
    let (start, stop) = (record.start, record.stop);
    let point_count = record.points;
    let sample_interval = (stop - start) / point_count as Value;
    let frequency_resolution = 1.0 / (stop - start);
    let (mut input, coherent_gain) = record.finish(abort)?;
    if !coherent_gain.is_finite() || coherent_gain <= 0.0 {
        return Err(request_error(
            index,
            format!("window coherent gain is invalid ({coherent_gain})"),
        ));
    }
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    let bin_count = point_count / 2 + 1;
    let format = analysis.format.unwrap_or_else(|| mode.default_format());
    if status.is_complete() {
        qualify_rustfft_forward_length(point_count).map_err(|error| {
            request_error(
                index,
                format!("transform planning is not qualified: {error}"),
            )
        })?;
        let fft = planner.plan_fft_forward(point_count);
        let scratch_len = fft.get_inplace_scratch_len();
        let mut scratch = Vec::new();
        scratch
            .try_reserve_exact(scratch_len)
            .map_err(|_| allocation_error("transform scratch"))?;
        scratch.resize(scratch_len, Complex::new(0.0, 0.0));
        fft.process_with_scratch(&mut input, &mut scratch);
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }

        input.truncate(bin_count);
        let base_scale = 1.0 / (point_count as Value * coherent_gain);
        for (bin, coefficient) in input.iter_mut().enumerate() {
            let one_sided_scale = if bin == 0 || bin == point_count / 2 {
                base_scale
            } else {
                2.0 * base_scale
            };
            *coefficient *= one_sided_scale;
        }
        current_impulses::add_to_bins(
            &mut input,
            impulses,
            analysis,
            mode,
            transient_stop,
            coherent_gain,
            abort,
        )
        .map_err(|error| impulse_observation_error(index, error))?;
        window_derivatives::add_to_bins(
            &mut input,
            impulses,
            analysis,
            mode,
            transient_stop,
            coherent_gain,
            max_values,
            abort,
        )?;
        if format == FftFormat::Normalized {
            let largest = input
                .iter()
                .map(|coefficient| coefficient.norm())
                .fold(0.0, Value::max);
            if largest > 0.0 {
                for coefficient in &mut input {
                    *coefficient /= largest;
                }
            }
        }
    }

    let nyquist_bin = point_count / 2;
    let fundamental_bin = rounded_frequency_bin(
        index,
        "FREQ",
        analysis.fundamental_frequency,
        1,
        frequency_resolution,
        nyquist_bin,
    )?;
    let maximum_metric_bin = rounded_frequency_bin(
        index,
        "FMAX",
        analysis.maximum_frequency,
        nyquist_bin,
        frequency_resolution,
        nyquist_bin,
    )?;
    let minimum_metric_bin = rounded_frequency_bin(
        index,
        "FMIN",
        analysis.minimum_frequency,
        1,
        frequency_resolution,
        nyquist_bin,
    )?;
    if maximum_metric_bin < minimum_metric_bin
        || (fundamental_bin == 1 && maximum_metric_bin < 2)
        || (fundamental_bin > 1 && maximum_metric_bin < 1)
    {
        return Err(request_error(
            index,
            format!(
                "rounded FREQ/FMIN/FMAX bins are inconsistent ({fundamental_bin}/{minimum_metric_bin}/{maximum_metric_bin})"
            ),
        ));
    }

    let mut bins = Vec::new();
    bins.try_reserve_exact(if status.is_complete() { bin_count } else { 0 })
        .map_err(|_| allocation_error("one-sided spectrum"))?;
    for (bin, coefficient) in input.into_iter().enumerate() {
        if bin.is_multiple_of(256) && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let magnitude = coefficient.norm();
        let phase_degrees = coefficient.im.atan2(coefficient.re).to_degrees();
        if !coefficient.re.is_finite()
            || !coefficient.im.is_finite()
            || !magnitude.is_finite()
            || !phase_degrees.is_finite()
        {
            return Err(request_error(
                index,
                format!("transform produced a non-finite coefficient at bin {bin}"),
            ));
        }
        bins.push(TransientFftBin {
            index: bin,
            frequency: bin as Value * frequency_resolution,
            real: coefficient.re,
            imaginary: coefficient.im,
            magnitude,
            phase_degrees,
        });
    }
    let metrics = (emit_metrics && status.is_complete())
        .then(|| {
            calculate_metrics(
                index,
                &bins,
                fundamental_bin,
                minimum_metric_bin,
                maximum_metric_bin,
                analysis.minimum_frequency.is_some(),
                abort,
            )
        })
        .transpose()?;
    Ok(TransientFftResult {
        status,
        output: analysis.output.clone(),
        output_name,
        physical_type,
        start_time: start,
        stop_time: stop,
        sample_interval,
        point_count,
        accurate_sampling,
        format,
        mode,
        window: analysis.window,
        window_name: analysis.window_name.clone(),
        alpha: analysis.alpha,
        coherent_gain,
        frequency_resolution,
        fundamental_bin,
        minimum_metric_bin,
        maximum_metric_bin,
        bins,
        metrics,
    })
}

fn calculate_metrics(
    request_index: usize,
    bins: &[TransientFftBin],
    fundamental_bin: usize,
    minimum_metric_bin: usize,
    maximum_metric_bin: usize,
    minimum_was_authored: bool,
    abort: &dyn AbortSignal,
) -> Result<TransientFftMetrics, SimulationError> {
    let fundamental = bins.get(fundamental_bin).ok_or_else(|| {
        SimulationError::Circuit(".FFT fundamental bin is unavailable".to_string())
    })?;
    let fundamental_magnitude = fundamental.magnitude;
    if !fundamental_magnitude.is_finite() || fundamental_magnitude <= FFT_DB_NOISE_FLOOR {
        return Err(request_error(
            request_index,
            format!(
                "FFTOUT metrics require a finite first-harmonic magnitude above {FFT_DB_NOISE_FLOOR:e}; bin {fundamental_bin} has {fundamental_magnitude}"
            ),
        ));
    }

    let mut distortion_power = 0.0;
    for bin in
        (fundamental_bin.saturating_mul(2)..=maximum_metric_bin).step_by(fundamental_bin.max(1))
    {
        if bin.is_multiple_of(256) && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        distortion_power += bins[bin].magnitude.powi(2);
    }
    let thd_ratio = distortion_power.sqrt() / fundamental_magnitude;
    let thd_db = 20.0 * thd_ratio.max(FFT_DB_NOISE_FLOOR).log10();

    let mut noise_and_distortion_power = 0.0;
    for (bin, value) in bins.iter().enumerate().skip(1) {
        if bin.is_multiple_of(256) && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if bin != fundamental_bin {
            noise_and_distortion_power += value.magnitude.powi(2);
        }
    }
    let sndr_denominator = noise_and_distortion_power.sqrt().max(FFT_DB_NOISE_FLOOR);
    let sndr_db = 20.0 * (fundamental_magnitude / sndr_denominator).log10();
    let enob_bits = (sndr_db - 1.76) / 6.02;

    let signal_frequency_limit = maximum_metric_bin.max(fundamental_bin);
    let mut noise_power = 0.0;
    for (bin, value) in bins.iter().enumerate().skip(1) {
        if bin.is_multiple_of(256) && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if bin % fundamental_bin != 0 || bin > signal_frequency_limit {
            noise_power += value.magnitude.powi(2);
        }
    }
    let snr_ratio = fundamental_magnitude / noise_power.sqrt().max(FFT_DB_NOISE_FLOOR);
    let snr_db = 20.0 * snr_ratio.log10();

    let sfdr_lower = if !minimum_was_authored && maximum_metric_bin >= fundamental_bin {
        fundamental_bin
    } else {
        minimum_metric_bin
    };
    let mut largest_spur: Option<&TransientFftBin> = None;
    for (bin, candidate) in bins
        .iter()
        .enumerate()
        .take(maximum_metric_bin + 1)
        .skip(sfdr_lower)
    {
        if bin != fundamental_bin
            && candidate.magnitude > largest_spur.map_or(0.0, |spur| spur.magnitude)
        {
            largest_spur = Some(candidate);
        }
    }
    let sfdr_spur_magnitude = largest_spur.map_or(0.0, |spur| spur.magnitude);
    let sfdr_db =
        20.0 * (fundamental_magnitude / sfdr_spur_magnitude.max(FFT_DB_NOISE_FLOOR)).log10();

    let ranked_len = bins.len().saturating_sub(1).min(FFT_MAX_RANKED_HARMONICS);
    let mut largest_harmonics: Vec<TransientFftHarmonic> = Vec::new();
    largest_harmonics
        .try_reserve_exact(ranked_len)
        .map_err(|_| allocation_error("ranked harmonic list"))?;
    for bin in bins.iter().skip(1) {
        if bin.index.is_multiple_of(256) && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let harmonic = TransientFftHarmonic {
            rank: 0,
            bin: bin.index,
            frequency: bin.frequency,
            magnitude: bin.magnitude,
            magnitude_db: 20.0 * bin.magnitude.max(FFT_DB_NOISE_FLOOR).log10(),
            phase_degrees: bin.phase_degrees,
        };
        let position = largest_harmonics
            .iter()
            .position(|retained| {
                harmonic.magnitude > retained.magnitude
                    || (harmonic.magnitude == retained.magnitude && harmonic.bin < retained.bin)
            })
            .unwrap_or(largest_harmonics.len());
        if largest_harmonics.len() < ranked_len {
            largest_harmonics.insert(position, harmonic);
        } else if position < ranked_len {
            largest_harmonics.pop();
            largest_harmonics.insert(position, harmonic);
        }
    }
    for (rank, harmonic) in largest_harmonics.iter_mut().enumerate() {
        harmonic.rank = rank + 1;
    }

    for (name, value) in [
        ("fundamental magnitude", fundamental_magnitude),
        ("THD ratio", thd_ratio),
        ("THD dB", thd_db),
        ("SNDR", sndr_db),
        ("ENOB", enob_bits),
        ("SNR", snr_db),
        ("SFDR", sfdr_db),
    ] {
        if !value.is_finite() {
            return Err(request_error(
                request_index,
                format!("FFTOUT {name} is non-finite for the selected spectrum"),
            ));
        }
    }

    Ok(TransientFftMetrics {
        fundamental_magnitude,
        thd_ratio,
        thd_db,
        sndr_db,
        enob_bits,
        snr_db,
        sfdr_db,
        sfdr_spur_bin: largest_spur.map(|spur| spur.index),
        sfdr_spur_frequency: largest_spur.map(|spur| spur.frequency),
        largest_harmonics,
    })
}

fn validate_history(result: &TransientResult) -> Result<(), SimulationError> {
    if result.time.is_empty() {
        return Err(SimulationError::Circuit(
            ".FFT requires at least one retained transient sample".to_string(),
        ));
    }
    if !result.time[0].is_finite()
        || result
            .time
            .windows(2)
            .any(|pair| !pair[1].is_finite() || pair[1] <= pair[0])
    {
        return Err(SimulationError::Circuit(
            ".FFT requires finite, strictly increasing transient times".to_string(),
        ));
    }
    Ok(())
}

/// Return the exact coefficient applied by the transient FFT engine.
/// Result publishers use this to validate calibration, including
/// ALFA-dependent windows.
pub fn transient_fft_window_coefficient_with_alpha(
    window: FftWindow,
    index: usize,
    denominator: Value,
    alpha: Value,
) -> Value {
    window_coefficient_at_position_with_alpha(window, index as Value, denominator, alpha)
}

fn window_coefficient_at_position_with_alpha(
    window: FftWindow,
    index: Value,
    denominator: Value,
    alpha: Value,
) -> Value {
    if window == FftWindow::Rectangular {
        return 1.0;
    }
    window::coefficient(window, index / denominator, alpha)
}

/// Modified Bessel function I0 evaluated with its rapidly convergent power
/// series. `.FFT ALFA` is bounded to 1..=20, so this avoids overflow while
/// retaining substantially more precision than the transform needs.
fn modified_bessel_i0(value: Value) -> Value {
    let half_squared = (value * 0.5).powi(2);
    let mut term = 1.0;
    let mut sum = 1.0;
    for order in 1..=64 {
        let order_value = order as Value;
        term *= half_squared / (order_value * order_value);
        sum += term;
        if term <= sum * 1.0e-16 {
            break;
        }
    }
    sum
}

/// Compute the coherent gain of the exact FFT window convention used by the
/// transient engine. Public result adapters use this to validate that
/// serialized window metadata describes the coefficients the engine applied.
#[cfg(test)]
pub fn transient_fft_window_coherent_gain(
    window: FftWindow,
    mode: XyceFftMode,
    points: usize,
    abort: &dyn AbortSignal,
) -> Result<Value, SimulationError> {
    transient_fft_window_coherent_gain_with_alpha(
        window,
        FftAnalysis::DEFAULT_ALPHA,
        mode,
        points,
        abort,
    )
}

/// Compute coherent gain using an explicit `.FFT ALFA` value.
pub fn transient_fft_window_coherent_gain_with_alpha(
    window: FftWindow,
    alpha: Value,
    mode: XyceFftMode,
    points: usize,
    abort: &dyn AbortSignal,
) -> Result<Value, SimulationError> {
    if points < 4 || !points.is_power_of_two() {
        return Err(SimulationError::Circuit(
            "FFT coherent gain requires a power-of-two point count of at least four".to_owned(),
        ));
    }
    if !alpha.is_finite() || !(1.0..=20.0).contains(&alpha) {
        return Err(SimulationError::Circuit(format!(
            "FFT window ALFA must be finite and between 1 and 20, found {alpha}"
        )));
    }
    let denominator = if mode.uses_periodic_windows() {
        points as Value
    } else {
        (points - 1) as Value
    };
    let mut window_sum = 0.0;
    for index in 0..points {
        if index.is_multiple_of(64) && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        window_sum +=
            transient_fft_window_coefficient_with_alpha(window, index, denominator, alpha);
    }
    let coherent_gain = window_sum / points as Value;
    if !coherent_gain.is_finite() || coherent_gain <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "FFT window coherent gain is invalid ({coherent_gain})"
        )));
    }
    Ok(coherent_gain)
}

fn rounded_frequency_bin(
    request_index: usize,
    name: &str,
    requested: Option<Value>,
    default: usize,
    resolution: Value,
    nyquist_bin: usize,
) -> Result<usize, SimulationError> {
    let Some(requested) = requested else {
        return Ok(default);
    };
    let rounded = (requested / resolution).round();
    if !rounded.is_finite() || rounded < 0.0 || rounded > usize::MAX as Value {
        return Err(request_error(
            request_index,
            format!("{name} cannot be represented as a transform bin"),
        ));
    }
    let bin = rounded as usize;
    if name == "FREQ" && bin == 0 {
        return Err(request_error(
            request_index,
            format!("FREQ {requested} Hz rounds below the first transform bin"),
        ));
    }
    if bin > nyquist_bin {
        return Err(request_error(
            request_index,
            format!("{name} {requested} Hz rounds to bin {bin}, above Nyquist bin {nyquist_bin}"),
        ));
    }
    Ok(bin)
}

fn request_error(index: usize, detail: String) -> SimulationError {
    SimulationError::Circuit(format!(".FFT request {}: {detail}", index + 1))
}

fn impulse_observation_error(
    index: usize,
    error: crate::analysis::measure_signals::impulse_observation::ImpulseObservationError,
) -> SimulationError {
    match error {
        crate::analysis::measure_signals::impulse_observation::ImpulseObservationError::Aborted => {
            SimulationError::Aborted
        }
        error => request_error(index, error.to_string()),
    }
}

fn allocation_error(allocation: &str) -> SimulationError {
    SimulationError::Circuit(format!(
        ".FFT could not reserve memory for its {allocation}"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::NoAbort;
    use crate::netlist::FftOutput;

    #[test]
    fn all_symmetric_windows_are_finite_and_have_positive_gain() {
        let windows = [
            FftWindow::Rectangular,
            FftWindow::Bartlett,
            FftWindow::BartlettHann,
            FftWindow::Hamming,
            FftWindow::Hann,
            FftWindow::Blackman67Db,
            FftWindow::Blackman,
            FftWindow::BlackmanHarris,
            FftWindow::Nuttall,
            FftWindow::HalfCycleSine,
            FftWindow::HalfCycleSine3,
            FftWindow::HalfCycleSine6,
            FftWindow::Cosine2,
            FftWindow::Cosine4,
            FftWindow::Gaussian,
            FftWindow::Kaiser,
        ];
        for window in windows {
            let coefficients = (0..64)
                .map(|index| transient_fft_window_coefficient_with_alpha(window, index, 63.0, 3.0))
                .collect::<Vec<_>>();
            assert!(
                coefficients
                    .iter()
                    .all(|coefficient| coefficient.is_finite())
            );
            assert!(coefficients.iter().sum::<Value>() > 0.0);
            for pair in coefficients.iter().zip(coefficients.iter().rev()) {
                assert!((pair.0 - pair.1).abs() < 1.0e-14, "{window:?}");
            }
        }
    }

    #[test]
    fn gaussian_and_kaiser_alpha_change_the_window_shape() {
        let gaussian_default =
            transient_fft_window_coefficient_with_alpha(FftWindow::Gaussian, 0, 63.0, 3.0);
        let gaussian_narrow =
            transient_fft_window_coefficient_with_alpha(FftWindow::Gaussian, 0, 63.0, 6.0);
        assert!(gaussian_narrow < gaussian_default);

        let kaiser_default =
            transient_fft_window_coefficient_with_alpha(FftWindow::Kaiser, 0, 63.0, 3.0);
        let kaiser_narrow =
            transient_fft_window_coefficient_with_alpha(FftWindow::Kaiser, 0, 63.0, 12.0);
        assert!(kaiser_narrow < kaiser_default);
        for alpha in [1.0, 3.0, 20.0] {
            let gain = transient_fft_window_coherent_gain_with_alpha(
                FftWindow::Kaiser,
                alpha,
                XyceFftMode::HspiceCompatible,
                64,
                &NoAbort,
            )
            .expect("qualified Kaiser gain");
            assert!(gain.is_finite() && gain > 0.0);
        }
    }

    #[test]
    fn public_window_gain_preserves_periodic_and_symmetric_conventions() {
        let symmetric = transient_fft_window_coherent_gain(
            FftWindow::Hann,
            XyceFftMode::HspiceCompatible,
            8,
            &NoAbort,
        )
        .expect("symmetric Hann gain");
        let periodic = transient_fft_window_coherent_gain(
            FftWindow::Hann,
            XyceFftMode::SpectreCompatible,
            8,
            &NoAbort,
        )
        .expect("periodic Hann gain");
        assert!((symmetric - 7.0 / 16.0).abs() < 1.0e-14);
        assert!((periodic - 0.5).abs() < 1.0e-14);
        assert!(
            transient_fft_window_coherent_gain(
                FftWindow::Rectangular,
                XyceFftMode::HspiceCompatible,
                6,
                &NoAbort,
            )
            .is_err()
        );
    }

    #[test]
    fn a_uniform_record_fed_a_span_at_a_time_equals_the_whole_rows() {
        let netlist = Netlist::parse(
            "fft uniform record\nV1 out 0 0\nR1 out 0 1k\n.tran 1u 1\n.fft v(out) np=8 window=rect\n.end\n",
        )
        .unwrap();
        let analysis = &netlist.fft_analyses[0];
        let time = [0.0, 0.1, 0.25, 0.4, 0.5, 0.77, 1.0];
        let values = [0.0, 0.2, 0.5, 0.8, 1.0, 1.54, 2.0];
        let record = |spans: &[usize]| {
            let mut record = UniformRecord::new(
                0,
                analysis,
                XyceFftMode::default(),
                1.0,
                &TransientFftStatus::Complete,
            )
            .unwrap();
            let mut first = 0;
            for &end in spans {
                record
                    .push(&time[first..end], &values[first..end], &NoAbort)
                    .unwrap();
                first = end;
            }
            record.finish(&NoAbort).unwrap()
        };
        let (whole, gain) = record(&[time.len()]);
        assert_eq!(gain, 1.0);
        // A sample at a row takes it; one between rows interpolates the two around it.
        let samples = whole.iter().map(|sample| sample.re).collect::<Vec<_>>();
        assert_eq!(samples[0], 0.0);
        assert_eq!(samples[2], 0.5);
        assert_eq!(samples[4], 1.0);
        assert!((samples[1] - 0.25).abs() < 1e-15, "{samples:?}");
        assert!((samples[6] - 1.5).abs() < 1e-15, "{samples:?}");
        for spans in [
            &[1, 7][..],
            &[3, 7],
            &[2, 4, 5, 6, 7],
            &[1, 2, 3, 4, 5, 6, 7],
        ] {
            assert_eq!(record(spans), (whole.clone(), gain), "{spans:?}");
        }
    }

    #[test]
    fn output_interval_schedule_selects_interpolation_even_when_accurate_is_authored() {
        let netlist = Netlist::parse(
            "fft output schedule compatibility\nV1 out 0 0\nR1 out 0 1k\n.options output initial_interval=100u\n.options fft fft_accurate=1\n.tran 1u 1m\n.fft v(out) np=8\n.end\n",
        )
        .expect("compatible interpolation schedule parses");

        assert!(!uses_accurate_sampling(&netlist));
    }

    #[test]
    fn one_sided_calibration_does_not_double_the_nyquist_bin() {
        let analysis = FftAnalysis {
            output: FftOutput::Probe("V(OUT)".to_string()),
            start: Some(0.0),
            stop: Some(1.0),
            points: 8,
            format: Some(FftFormat::Unnormalized),
            window: FftWindow::Rectangular,
            window_name: "RECT".to_string(),
            alpha: FftAnalysis::DEFAULT_ALPHA,
            fundamental_frequency: None,
            minimum_frequency: None,
            maximum_frequency: None,
        };
        let time = (0..=8)
            .map(|index| index as Value / 8.0)
            .collect::<Vec<_>>();
        let values = (0..=8)
            .map(|index| if index % 2 == 0 { 1.0 } else { -1.0 })
            .collect::<Vec<_>>();
        let mut planner = FftPlanner::new();
        let result = evaluate_one(
            0,
            &analysis,
            XyceFftMode::HspiceCompatible,
            true,
            false,
            "V(OUT)".to_string(),
            "voltage",
            &time,
            &values,
            None,
            1.0,
            &mut planner,
            crate::ResourceLimits::default().max_result_values,
            &NoAbort,
        )
        .expect("exact Nyquist record transforms");

        assert_eq!(result.bins.len(), 5);
        assert!((result.bins[4].magnitude - 1.0).abs() < 1.0e-14);
        assert!(result.bins[..4].iter().all(|bin| bin.magnitude < 1.0e-14));
    }

    #[test]
    fn no_spur_metrics_are_finite_and_use_the_reporting_floor() {
        let bins = (0..=4)
            .map(|index| TransientFftBin {
                index,
                frequency: index as Value,
                real: if index == 1 { 1.0 } else { 0.0 },
                imaginary: 0.0,
                magnitude: if index == 1 { 1.0 } else { 0.0 },
                phase_degrees: 0.0,
            })
            .collect::<Vec<_>>();
        let metrics = calculate_metrics(0, &bins, 1, 1, 4, false, &NoAbort)
            .expect("a resolved pure fundamental has finite floor-limited metrics");

        assert_eq!(metrics.thd_ratio, 0.0);
        assert_eq!(metrics.thd_db, -200.0);
        assert_eq!(metrics.sndr_db, 200.0);
        assert_eq!(metrics.snr_db, 200.0);
        assert_eq!(metrics.sfdr_db, 200.0);
        assert_eq!(metrics.sfdr_spur_bin, None);
        assert_eq!(metrics.sfdr_spur_frequency, None);
        assert!(
            [
                metrics.enob_bits,
                metrics.thd_db,
                metrics.sndr_db,
                metrics.snr_db,
                metrics.sfdr_db,
            ]
            .into_iter()
            .all(Value::is_finite)
        );
    }
}
