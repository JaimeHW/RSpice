//! Public time-domain result types.

use crate::analysis::fourier::FourierResult;
use crate::analysis::measure::MeasureResult;
use crate::engine::waveform::{
    TransientChannelAvailability, TransientChannelRole, TransientChannelSample,
    TransientResultCompressed, TransientSampleAbsence,
};
use crate::netlist::{FftFormat, FftOutput, FftWindow, XyceFftMode, XyceOutputIntervalSchedule};
use crate::xspice::{DigitalState, DigitalStrength, DigitalValue};
use crate::{NodeId, Value};
use std::collections::HashMap;

/// Stable wire spelling of one XSPICE digital logic state.
///
/// Persistence layers need a stable text spelling of a committed digital
/// sample. The engine owns the one it publishes so a pickle or a wire format
/// never has to invent a second naming of the same state.
pub const fn digital_state_tag(state: DigitalState) -> &'static str {
    match state {
        DigitalState::Zero => "zero",
        DigitalState::One => "one",
        DigitalState::Unknown => "unknown",
        DigitalState::ZeroR => "zero-resistive",
        DigitalState::OneR => "one-resistive",
        DigitalState::UnknownR => "unknown-resistive",
        DigitalState::ZeroZ => "zero-high-z",
        DigitalState::OneZ => "one-high-z",
        DigitalState::UnknownZ => "unknown-high-z",
        DigitalState::HighZ => "high-z",
    }
}

/// Parse a spelling produced by [`digital_state_tag`].
pub fn digital_state_from_tag(tag: &str) -> Option<DigitalState> {
    Some(match tag {
        "zero" => DigitalState::Zero,
        "one" => DigitalState::One,
        "unknown" => DigitalState::Unknown,
        "zero-resistive" => DigitalState::ZeroR,
        "one-resistive" => DigitalState::OneR,
        "unknown-resistive" => DigitalState::UnknownR,
        "zero-high-z" => DigitalState::ZeroZ,
        "one-high-z" => DigitalState::OneZ,
        "unknown-high-z" => DigitalState::UnknownZ,
        "high-z" => DigitalState::HighZ,
        _ => return None,
    })
}

/// Stable wire spelling of one XSPICE digital drive strength.
pub const fn digital_strength_tag(strength: DigitalStrength) -> &'static str {
    match strength {
        DigitalStrength::Undetermined => "undetermined",
        DigitalStrength::HighZ => "high-z",
        DigitalStrength::Resistive => "resistive",
        DigitalStrength::Strong => "strong",
    }
}

/// Parse a spelling produced by [`digital_strength_tag`].
pub fn digital_strength_from_tag(tag: &str) -> Option<DigitalStrength> {
    Some(match tag {
        "undetermined" => DigitalStrength::Undetermined,
        "high-z" => DigitalStrength::HighZ,
        "resistive" => DigitalStrength::Resistive,
        "strong" => DigitalStrength::Strong,
        _ => None?,
    })
}

/// One accepted or linearly interpolated transient output sample.
#[derive(Debug, Clone, Copy, PartialEq)]
enum TransientOutputCoordinate {
    Accepted(usize),
    Linear {
        previous: usize,
        current: usize,
        from_current: Value,
    },
}

/// Sample selection and interpolation used by transient output writers.
///
/// The solver result always retains its complete accepted history. This view
/// selects output rows without changing integration history, measurements,
/// Fourier analysis, checkpoint continuation, or compression inputs.
#[derive(Debug, Clone, PartialEq)]
pub struct TransientOutputProjection {
    source_len: usize,
    times: Vec<Value>,
    coordinates: Vec<TransientOutputCoordinate>,
}

impl TransientOutputProjection {
    /// Exact output times represented by this projection.
    pub fn times(&self) -> &[Value] {
        &self.times
    }

    /// Project one accepted-history series onto the output grid.
    ///
    /// Interval schedules use the same current-side linear formula as Xyce
    /// 7.10's transient integration methods. Exact accepted samples are copied
    /// without arithmetic so their IEEE-754 values remain unchanged.
    pub fn project(&self, values: &[Value]) -> Result<Vec<Value>, String> {
        if values.len() != self.source_len {
            return Err(format!(
                "transient output series has {} samples, expected {}",
                values.len(),
                self.source_len
            ));
        }
        Ok(self
            .coordinates
            .iter()
            .map(|coordinate| match *coordinate {
                TransientOutputCoordinate::Accepted(index) => values[index],
                TransientOutputCoordinate::Linear {
                    previous,
                    current,
                    from_current,
                } => {
                    let current_value = values[current];
                    current_value + from_current * (current_value - values[previous])
                }
            })
            .collect())
    }
}

/// One accepted digital event sample for a named XSPICE digital node.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DigitalTracePoint {
    /// Event time in seconds.
    pub time: Value,
    /// Digital value at this event time.
    pub value: DigitalValue,
}

/// Accepted digital event history for one XSPICE digital node.
#[derive(Debug, Clone, PartialEq)]
pub struct DigitalTrace {
    /// Original netlist node name.
    pub node_name: String,
    /// Committed event samples for this node.
    pub points: Vec<DigitalTracePoint>,
}

/// One accepted real-valued event sample for a named XSPICE real node.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RealTracePoint {
    /// Event time in seconds.
    pub time: Value,
    /// Real event-node value at this event time.
    pub value: Value,
}

/// Accepted real-valued event history for one XSPICE real node.
#[derive(Debug, Clone, PartialEq)]
pub struct RealTrace {
    /// Original netlist node name.
    pub node_name: String,
    /// Committed event samples for this node.
    pub points: Vec<RealTracePoint>,
}

/// Accepted operating-point parameter history for one device.
#[derive(Debug, Clone, PartialEq)]
pub struct TransientDeviceOpTrace {
    /// Original netlist device name.
    pub device_name: String,
    /// Device operating-point parameter name, such as `vbs` or `gm`.
    pub parameter: String,
    /// One value per accepted transient sample.
    pub values: Vec<Value>,
}

/// One typed non-solution device store waveform, such as a compact-model
/// internal resistance. Store traces are deliberately separate from voltage
/// nodes so units, matrix topology, compression, and UI labeling remain sound.
#[derive(Debug, Clone, PartialEq)]
pub struct TransientStoreTrace {
    /// Canonical Xyce store name, for example `YMEMRISTOR!MR1:R`.
    pub name: String,
    /// One value per accepted transient sample.
    pub values: Vec<Value>,
}

/// One calibrated bin in a one-sided transient `.FFT` spectrum.
#[derive(Debug, Clone, PartialEq)]
pub struct TransientFftBin {
    /// Zero-based DFT bin index. The final bin is the Nyquist bin.
    pub index: usize,
    /// Bin-center frequency in hertz.
    pub frequency: Value,
    /// Real component after one-sided amplitude calibration and `FORMAT`.
    pub real: Value,
    /// Imaginary component after one-sided amplitude calibration and `FORMAT`.
    pub imaginary: Value,
    /// Linear magnitude of `(real, imaginary)`.
    pub magnitude: Value,
    /// Phase in degrees in the range accepted by `atan2`.
    pub phase_degrees: Value,
}

/// One entry in the magnitude-ranked harmonic list requested by `FFTOUT=1`.
#[derive(Debug, Clone, PartialEq)]
pub struct TransientFftHarmonic {
    /// One-based position in descending-magnitude order.
    pub rank: usize,
    /// DFT bin (harmonic) index.
    pub bin: usize,
    /// Bin-center frequency in hertz.
    pub frequency: Value,
    /// Magnitude in the spectrum's effective `FORMAT`.
    pub magnitude: Value,
    /// Magnitude in decibels, with Xyce's `1e-10` reporting floor.
    pub magnitude_db: Value,
    /// Phase in degrees.
    pub phase_degrees: Value,
}

/// Xyce-compatible figures requested by `.OPTIONS FFT FFTOUT=1`.
#[derive(Debug, Clone, PartialEq)]
pub struct TransientFftMetrics {
    /// Magnitude of the `FREQ`-selected first harmonic.
    pub fundamental_magnitude: Value,
    /// Total harmonic distortion as a linear amplitude ratio.
    pub thd_ratio: Value,
    /// Total harmonic distortion in decibels.
    pub thd_db: Value,
    /// Signal-to-noise-and-distortion ratio in decibels.
    pub sndr_db: Value,
    /// Effective number of bits, `(SNDR - 1.76) / 6.02`.
    pub enob_bits: Value,
    /// Signal-to-noise ratio in decibels.
    pub snr_db: Value,
    /// Spurious-free dynamic range in decibels.
    pub sfdr_db: Value,
    /// Bin containing the largest spur, or `None` when none exists.
    pub sfdr_spur_bin: Option<usize>,
    /// Frequency of the largest spur, or `None` when none exists.
    pub sfdr_spur_frequency: Option<Value>,
    /// Up to 30 non-DC bins sorted by descending magnitude, then source bin.
    pub largest_harmonics: Vec<TransientFftHarmonic>,
}

/// Typed result of one source-authored transient `.FFT` directive.
#[derive(Debug, Clone, PartialEq)]
pub struct TransientFftResult {
    /// Authored probe or braced expression.
    pub output: FftOutput,
    /// Display spelling of the resolved scalar column.
    pub output_name: String,
    /// Physical quantity class: `voltage`, `current`, or `parameter`.
    pub physical_type: &'static str,
    /// Inclusive beginning of the sampled record.
    pub start_time: Value,
    /// Exclusive end of the sampled record.
    pub stop_time: Value,
    /// Uniform sample spacing `(stop_time - start_time) / point_count`.
    pub sample_interval: Value,
    /// Number of uniformly resampled real input points.
    pub point_count: usize,
    /// Whether the transient solver was forced to land on every sample time.
    /// This is normally true and becomes false for `FFT_ACCURATE=0` or an
    /// incompatible `.OPTIONS OUTPUT INITIAL_INTERVAL` schedule.
    pub accurate_sampling: bool,
    /// Effective coefficient format. `Normalized` divides every complex bin
    /// by the largest calibrated one-sided magnitude.
    pub format: FftFormat,
    /// Effective Xyce FFT compatibility mode.
    pub mode: XyceFftMode,
    /// Effective window selection.
    pub window: FftWindow,
    /// Canonical source spelling retained by the parser.
    pub window_name: String,
    /// HSPICE-compatible `ALFA` value (retained; unsupported Gaussian/Kaiser
    /// windows do not currently consume it).
    pub alpha: Value,
    /// Mean window coefficient used for coherent-gain compensation.
    pub coherent_gain: Value,
    /// DFT bin width in hertz, exactly `1 / (stop_time - start_time)`.
    pub frequency_resolution: Value,
    /// Rounded first-harmonic bin selected by `FREQ` (default 1).
    pub fundamental_bin: usize,
    /// Rounded lower metric bin selected by `FMIN`.
    pub minimum_metric_bin: usize,
    /// Rounded upper metric bin selected by `FMAX`.
    pub maximum_metric_bin: usize,
    /// DC through Nyquist, inclusive. `FMIN`/`FMAX` select metric bounds and
    /// intentionally do not truncate this source spectrum.
    pub bins: Vec<TransientFftBin>,
    /// Additional Xyce-compatible figures and ranked bins requested by
    /// `.OPTIONS FFT FFTOUT=1`.
    pub metrics: Option<TransientFftMetrics>,
}

/// Typed result of one source-authored `.FOUR` operand.
///
/// `.FOUR` resolves its operands through the ordered transient output
/// resolver, so the authored spelling and the physical-quantity class are
/// retained beside the spectrum rather than being re-derived by a frontend.
#[derive(Debug, Clone, PartialEq)]
pub struct TransientFourierResult {
    /// Zero-based ordinal of the authored `.FOUR` card.
    pub card_index: usize,
    /// Authored output spelling, such as `V(out)`.
    pub output: String,
    /// Physical quantity class: `voltage`, `current`, or `parameter`.
    pub physical_type: &'static str,
    /// Authored fundamental frequency in hertz.
    pub fundamental: Value,
    /// Authored harmonic count.
    pub harmonic_count: usize,
    /// The spectrum itself.
    pub spectrum: FourierResult,
}

/// Typed transient post-processing products.
///
/// These are evaluated on the exact accepted trajectory, before any output
/// projection or waveform decimation, so a compressed result publishes the
/// same numbers as an uncompressed one and no frontend has to recompute them
/// from a decimated expansion.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TransientPostResults {
    /// Source-authored `.FFT` spectra in netlist order.
    pub fft: Vec<TransientFftResult>,
    /// Source-authored `.FOUR` spectra, one entry per resolved operand, in
    /// card then operand order.
    pub fourier: Vec<TransientFourierResult>,
    /// Source-authored transient `.MEASURE` results in netlist order.
    pub measurements: Vec<MeasureResult>,
}

impl TransientPostResults {
    /// Whether no post-process product was requested or produced.
    pub fn is_empty(&self) -> bool {
        self.fft.is_empty() && self.fourier.is_empty() && self.measurements.is_empty()
    }
}

/// Result of transient analysis - time-domain waveforms
#[derive(Debug, Clone)]
pub struct TransientResult {
    /// Time points
    pub time: Vec<Value>,
    /// Accepted integration interval for each time point. The initial sample
    /// at `time[0]` has a zero interval; subsequent entries are the exact
    /// timestep used to produce the corresponding sample. Keeping this
    /// alongside the rounded absolute times makes deterministic grid replay
    /// preserve the producing run's integration coefficients.
    pub step_sizes: Vec<Value>,
    /// Voltage waveforms, indexed `[node_index][time_index]`. A waveform is
    /// empty when output projection deliberately did not retain that node;
    /// every non-empty waveform is aligned with `time`.
    pub voltages: Vec<Vec<Value>>,
    /// Branch current waveforms, indexed `[branch_index][time_index]`. A
    /// waveform is empty when output projection deliberately did not retain
    /// that branch; every non-empty waveform is aligned with `time`.
    pub branch_currents: Vec<Vec<Value>>,
    /// Number of nodes
    pub num_nodes: usize,
    /// Node names from the netlist (maps index to original name like "N001", "out", etc.)
    /// Index 0 corresponds to `voltages[0]`, which is node 1 (not ground)
    pub node_names: Vec<String>,
    /// Branch names aligned with `branch_currents`
    pub branch_names: Vec<String>,
    /// XSPICE digital node histories captured at accepted transient points.
    pub digital_traces: Vec<DigitalTrace>,
    /// XSPICE real-valued event node histories captured at accepted transient points.
    pub real_traces: Vec<RealTrace>,
    /// Device operating-point values captured at accepted transient points.
    pub device_op_traces: Vec<TransientDeviceOpTrace>,
    /// Typed device store outputs captured at accepted transient points.
    pub store_traces: Vec<TransientStoreTrace>,
    /// Source-authored `.FFT` post-processing results in netlist order.
    pub fft_results: Vec<TransientFftResult>,
}

impl TransientResult {
    /// Borrow the columns an abort signal's sample hook is allowed to see.
    ///
    /// The hook sits at the bottom of the crate and must not name a driver
    /// result type, so the driver narrows itself on the way out: waveform
    /// columns, their names, the accepted times, and the committed event state
    /// of the point being reported, and nothing else.
    ///
    /// The committed event state of the same accepted point is not stored
    /// column-major and so cannot be read back off the result — the traces
    /// keep only the changes. The caller therefore hands over the snapshot
    /// buffers it just recorded from, which is also what keeps the hook
    /// allocation-free.
    pub(crate) fn observable_sample<'a>(
        &'a self,
        digital_values: &'a [(NodeId, DigitalValue)],
        real_values: &'a [(NodeId, Value)],
    ) -> crate::abort_signal::TransientSample<'a> {
        crate::abort_signal::TransientSample {
            time: &self.time,
            node_names: &self.node_names,
            node_voltages: &self.voltages,
            branch_names: &self.branch_names,
            branch_currents: &self.branch_currents,
            digital_values,
            real_values,
        }
    }

    /// Build the Xyce transient output view for a requested schedule.
    ///
    /// With no output schedule, every accepted sample at or after TSTART is
    /// selected. `OUTPUTTIMEPOINTS` requires exact accepted solver breakpoints.
    /// `INITIAL_INTERVAL` instead builds Xyce's run-relative output lattice and
    /// linearly interpolates it from the complete accepted history.
    pub fn output_projection(
        &self,
        output_time_points: &[Value],
        output_interval_schedule: Option<&XyceOutputIntervalSchedule>,
        start_time: Value,
        stop_time: Value,
        max_points: usize,
    ) -> Result<TransientOutputProjection, String> {
        if !start_time.is_finite()
            || !stop_time.is_finite()
            || start_time < 0.0
            || stop_time < start_time
        {
            return Err(format!(
                "invalid transient output window [{start_time}, {stop_time}]"
            ));
        }
        if self.time.is_empty() {
            return Err("transient result has no accepted samples".to_string());
        }
        if !output_time_points.is_empty() && output_interval_schedule.is_some() {
            return Err(
                "transient output cannot combine OUTPUTTIMEPOINTS and INITIAL_INTERVAL".to_string(),
            );
        }
        if let Some(invalid) = output_time_points
            .iter()
            .find(|time| !time.is_finite() || **time < 0.0)
        {
            return Err(format!(
                "transient output schedule contains invalid time {invalid}"
            ));
        }
        for (index, &time) in self.time.iter().enumerate() {
            if !time.is_finite() {
                return Err(format!("transient result time[{index}] is not finite"));
            }
            if index > 0 && time <= self.time[index - 1] {
                return Err(format!(
                    "transient result time grid is not strictly increasing at index {index}"
                ));
            }
        }

        let (times, coordinates) = if let Some(schedule) = output_interval_schedule {
            let first = self.time.partition_point(|time| *time < start_time);
            let output_start_time = self.time.get(first).copied().ok_or_else(|| {
                format!("transient result has no accepted sample at or after TSTART={start_time}")
            })?;
            if output_start_time > stop_time {
                return Err("transient output projection selected no samples".to_string());
            }
            let events = schedule.output_events(&self.time, start_time, stop_time, max_points)?;
            let mut times = Vec::with_capacity(events.len());
            let mut coordinates = Vec::with_capacity(events.len());
            for event in events {
                times.push(event.output_time);
                let Some(target) = event.interpolation_time else {
                    coordinates.push(TransientOutputCoordinate::Accepted(event.accepted_index));
                    continue;
                };
                let current = event.accepted_index;
                if current == 0 || current >= self.time.len() {
                    return Err(format!(
                        "transient output time {target} has no accepted interpolation bracket"
                    ));
                }
                let previous = current - 1;
                let width = self.time[current] - self.time[previous];
                let from_current = (target - self.time[current]) / width;
                if !from_current.is_finite() || !(-1.0..=0.0).contains(&from_current) {
                    return Err(format!(
                        "transient output time {target} has an invalid interpolation bracket"
                    ));
                }
                coordinates.push(TransientOutputCoordinate::Linear {
                    previous,
                    current,
                    from_current,
                });
            }
            (times, coordinates)
        } else if output_time_points.is_empty() {
            let first = self.time.partition_point(|time| *time < start_time);
            let last = self.time.partition_point(|time| *time <= stop_time);
            let indices = (first..last).collect::<Vec<_>>();
            crate::resource::ResourceLimitError::ensure(
                crate::resource::ResourceKind::AnalysisPoints,
                indices.len(),
                max_points,
            )
            .map_err(|error| error.to_string())?;
            (
                indices.iter().map(|index| self.time[*index]).collect(),
                indices
                    .into_iter()
                    .map(TransientOutputCoordinate::Accepted)
                    .collect(),
            )
        } else {
            let mut requested = output_time_points
                .iter()
                .copied()
                .filter(|time| *time >= start_time && *time <= stop_time)
                .collect::<Vec<_>>();
            requested.push(stop_time);
            requested.sort_by(Value::total_cmp);
            requested.dedup_by(|left, right| left.to_bits() == right.to_bits());
            crate::resource::ResourceLimitError::ensure(
                crate::resource::ResourceKind::AnalysisPoints,
                requested.len(),
                max_points,
            )
            .map_err(|error| error.to_string())?;

            let mut coordinates = Vec::with_capacity(requested.len());
            for &requested_time in &requested {
                let index = self
                    .time
                    .binary_search_by(|time| time.total_cmp(&requested_time))
                    .map_err(|_| {
                        format!(
                            "transient result did not land on requested output time {requested_time}"
                        )
                    })?;
                coordinates.push(TransientOutputCoordinate::Accepted(index));
            }
            (requested, coordinates)
        };

        if coordinates.is_empty() {
            return Err("transient output projection selected no samples".to_string());
        }
        Ok(TransientOutputProjection {
            source_len: self.time.len(),
            times,
            coordinates,
        })
    }

    /// Append one accepted device operating-point snapshot. Missing parameters
    /// are padded with NaN so every stored trace remains aligned with `time`.
    pub(crate) fn record_device_op_sample(
        &mut self,
        report: crate::circuit::DeviceOpReport,
    ) -> usize {
        if self.time.is_empty() {
            return 0;
        }

        let sample_index = self.time.len() - 1;
        let mut seen = vec![false; self.device_op_traces.len()];
        let mut added_values = 0usize;

        for entry in report.entries {
            for (parameter, value) in entry.params {
                let trace_index = self
                    .device_op_traces
                    .iter()
                    .position(|trace| {
                        trace.device_name.eq_ignore_ascii_case(&entry.name)
                            && trace.parameter.eq_ignore_ascii_case(parameter)
                    })
                    .unwrap_or_else(|| {
                        let trace_index = self.device_op_traces.len();
                        self.device_op_traces.push(TransientDeviceOpTrace {
                            device_name: entry.name.clone(),
                            parameter: parameter.to_string(),
                            values: vec![Value::NAN; sample_index],
                        });
                        added_values = added_values.saturating_add(sample_index);
                        seen.push(false);
                        trace_index
                    });

                let trace = &mut self.device_op_traces[trace_index];
                if trace.values.len() < sample_index {
                    added_values = added_values.saturating_add(sample_index - trace.values.len());
                    trace.values.resize(sample_index, Value::NAN);
                }
                if trace.values.len() == sample_index {
                    trace.values.push(value);
                    added_values = added_values.saturating_add(1);
                } else if let Some(slot) = trace.values.get_mut(sample_index) {
                    *slot = value;
                }
                seen[trace_index] = true;
            }
        }

        for (trace_index, trace) in self.device_op_traces.iter_mut().enumerate() {
            if seen.get(trace_index).copied().unwrap_or(false) {
                continue;
            }
            if trace.values.len() < sample_index {
                added_values = added_values.saturating_add(sample_index - trace.values.len());
                trace.values.resize(sample_index, Value::NAN);
            }
            if trace.values.len() == sample_index {
                trace.values.push(Value::NAN);
                added_values = added_values.saturating_add(1);
            }
        }
        added_values
    }

    /// Append committed XSPICE digital values at an accepted transient time.
    pub(crate) fn record_digital_snapshot(
        &mut self,
        time: Value,
        snapshot: &[(NodeId, DigitalValue)],
        trace_indices: &mut HashMap<NodeId, usize>,
        retained_nodes: &[bool],
    ) -> usize {
        let mut added_values = 0usize;
        for &(node_id, value) in snapshot {
            let Some(node_index) = node_id.checked_sub(1) else {
                continue;
            };
            if !retained_nodes.get(node_index).copied().unwrap_or(false) {
                continue;
            }
            let Some(node_name) = self.node_names.get(node_index) else {
                continue;
            };
            let trace_idx = match trace_indices.get(&node_id).copied() {
                Some(index) => index,
                None => {
                    let index = self.digital_traces.len();
                    self.digital_traces.push(DigitalTrace {
                        node_name: node_name.clone(),
                        points: Vec::new(),
                    });
                    trace_indices.insert(node_id, index);
                    index
                }
            };

            let trace = &mut self.digital_traces[trace_idx];
            if trace
                .points
                .last()
                .is_some_and(|point| point.time == time && point.value == value)
            {
                continue;
            }
            if trace
                .points
                .last()
                .is_some_and(|point| point.value == value)
            {
                continue;
            }
            trace.points.push(DigitalTracePoint { time, value });
            added_values = added_values.saturating_add(2);
        }
        added_values
    }

    /// Get the complete digital event trace for a named XSPICE digital node.
    pub fn digital_trace_named(&self, name: &str) -> Option<&[DigitalTracePoint]> {
        self.digital_traces
            .iter()
            .find(|trace| trace.node_name.eq_ignore_ascii_case(name))
            .map(|trace| trace.points.as_slice())
    }

    /// Append committed XSPICE real event values at an accepted transient time.
    pub(crate) fn record_real_snapshot(
        &mut self,
        time: Value,
        snapshot: &[(NodeId, Value)],
        trace_indices: &mut HashMap<NodeId, usize>,
        retained_nodes: &[bool],
    ) -> usize {
        let mut added_values = 0usize;
        for &(node_id, value) in snapshot {
            let Some(node_index) = node_id.checked_sub(1) else {
                continue;
            };
            if !retained_nodes.get(node_index).copied().unwrap_or(false) {
                continue;
            }
            let Some(node_name) = self.node_names.get(node_index) else {
                continue;
            };
            let trace_idx = match trace_indices.get(&node_id).copied() {
                Some(index) => index,
                None => {
                    let index = self.real_traces.len();
                    self.real_traces.push(RealTrace {
                        node_name: node_name.clone(),
                        points: Vec::new(),
                    });
                    trace_indices.insert(node_id, index);
                    index
                }
            };

            let trace = &mut self.real_traces[trace_idx];
            if trace
                .points
                .last()
                .is_some_and(|point| point.time == time && point.value == value)
            {
                continue;
            }
            if trace
                .points
                .last()
                .is_some_and(|point| point.value == value)
            {
                continue;
            }
            trace.points.push(RealTracePoint { time, value });
            added_values = added_values.saturating_add(2);
        }
        added_values
    }

    /// Get the complete real-valued event trace for a named XSPICE real node.
    pub fn real_trace_named(&self, name: &str) -> Option<&[RealTracePoint]> {
        self.real_traces
            .iter()
            .find(|trace| trace.node_name.eq_ignore_ascii_case(name))
            .map(|trace| trace.points.as_slice())
    }

    /// Get voltage at a node at a specific time index.
    ///
    /// Panics for invalid non-ground node IDs or time indices. Use
    /// [`Self::try_voltage_at`] for checked access.
    #[track_caller]
    pub fn voltage_at(&self, node: usize, time_index: usize) -> Value {
        self.try_voltage_at(node, time_index).unwrap_or_else(|| {
            panic!(
                "node {} / time index {} out of range for TransientResult with {} nodes and {} samples",
                node,
                time_index,
                self.num_nodes,
                self.time.len()
            )
        })
    }

    /// Get voltage at a node at a specific time index, returning `None` when
    /// the node or index is invalid. Ground (node 0) is `Some(0.0)` at every
    /// stored time point.
    pub fn try_voltage_at(&self, node: usize, time_index: usize) -> Option<Value> {
        self.time.get(time_index)?;
        if node == 0 {
            return Some(0.0);
        }
        if node > self.num_nodes {
            return None;
        }
        self.voltages
            .get(node - 1)
            .and_then(|v| v.get(time_index))
            .copied()
    }

    /// Resolve a node name to a 1-based node index, treating common ground
    /// aliases as node 0.
    pub fn node_index_named(&self, name: &str) -> Option<usize> {
        if Self::is_ground_name(name) {
            return Some(0);
        }

        self.node_names
            .iter()
            .position(|candidate| candidate.eq_ignore_ascii_case(name))
            .map(|idx| idx + 1)
    }

    /// Get the complete voltage waveform for a node.
    ///
    /// Panics when `node` is invalid. Use [`Self::try_voltage_waveform`] for
    /// checked access.
    #[track_caller]
    pub fn voltage_waveform(&self, node: usize) -> &[Value] {
        self.try_voltage_waveform(node).unwrap_or_else(|| {
            panic!(
                "node {} out of range for TransientResult with {} nodes",
                node, self.num_nodes
            )
        })
    }

    /// Get the complete voltage waveform for a node, returning `None` when the
    /// node is invalid.
    pub fn try_voltage_waveform(&self, node: usize) -> Option<&[Value]> {
        if node == 0 || node > self.num_nodes {
            return None;
        }
        self.voltages.get(node - 1).map(|v| v.as_slice())
    }

    /// Get the complete voltage waveform for a named node, returning `None`
    /// when the name does not resolve to a non-ground node.
    pub fn try_voltage_waveform_named(&self, name: &str) -> Option<&[Value]> {
        let node = self.node_index_named(name)?;
        if node == 0 {
            return None;
        }
        self.try_voltage_waveform(node)
    }

    /// Get the complete current waveform for a named branch, returning `None`
    /// when the branch name does not resolve.
    pub fn try_branch_current_waveform_named(&self, name: &str) -> Option<&[Value]> {
        let branch_idx = self
            .branch_names
            .iter()
            .position(|candidate| candidate.eq_ignore_ascii_case(name))?;
        self.branch_currents
            .get(branch_idx)
            .map(|waveform| waveform.as_slice())
    }

    /// Get the complete operating-point parameter waveform for a named device.
    pub fn try_device_op_waveform_named(
        &self,
        device_name: &str,
        parameter: &str,
    ) -> Option<&[Value]> {
        self.device_op_traces
            .iter()
            .find(|trace| {
                trace.device_name.eq_ignore_ascii_case(device_name)
                    && trace.parameter.eq_ignore_ascii_case(parameter)
            })
            .map(|trace| trace.values.as_slice())
    }

    /// Get a typed device-store waveform by its canonical store name.
    pub fn try_store_waveform_named(&self, name: &str) -> Option<&[Value]> {
        self.store_traces
            .iter()
            .find(|trace| trace.name.eq_ignore_ascii_case(name))
            .map(|trace| trace.values.as_slice())
    }

    /// Get voltage at a named node and time index, returning `None` when the
    /// name or time index is invalid.
    pub fn try_voltage_at_named(&self, name: &str, time_index: usize) -> Option<Value> {
        let node = self.node_index_named(name)?;
        if node == 0 {
            return self.time.get(time_index).map(|_| 0.0);
        }
        self.try_voltage_at(node, time_index)
    }

    /// Get number of time points
    pub fn num_points(&self) -> usize {
        self.time.len()
    }

    fn is_ground_name(name: &str) -> bool {
        crate::naming::is_spice_ground_name(name)
    }
}

impl TransientResultCompressed {
    /// Expand the retained inventory into a regular transient result.
    ///
    /// This preserves the retained grid; it does not reconstruct discarded
    /// samples. Event-driven digital and real traces were never decimated and
    /// are carried through unchanged.
    ///
    /// [`TransientResult`] stores every analog sample as a bare `Value`, so it
    /// cannot represent a sample the producing run computed as non-finite.
    /// Expansion therefore fails closed on such a channel instead of inventing
    /// a number; read those samples from
    /// [`TransientResultCompressed::channels`], which keeps them as typed
    /// absences. A device operating-point parameter that a device did not
    /// report is expanded back to the `NaN` padding that
    /// [`TransientResult::record_device_op_sample`] writes, which is the exact
    /// representation it was compressed from.
    pub fn try_into_transient(self) -> Result<TransientResult, String> {
        self.validate()?;
        let point_count = self.time.len();
        let mut num_nodes = 0usize;
        let mut node_names = Vec::new();
        let mut voltages = Vec::new();
        let mut branch_names = Vec::new();
        let mut branch_currents = Vec::new();
        let mut device_op_traces = Vec::new();
        let mut store_traces = Vec::new();

        for channel in &self.channels {
            let projected = channel.availability == TransientChannelAvailability::Available;
            let canonical = channel.descriptor.canonical_name();
            let expand = |padding: Option<Value>| -> Result<Vec<Value>, String> {
                if !projected {
                    return Ok(Vec::new());
                }
                let mut values = Vec::with_capacity(point_count);
                for (index, sample) in channel.samples.iter().enumerate() {
                    match *sample {
                        TransientChannelSample::Value(value) => values.push(value),
                        TransientChannelSample::Absent(TransientSampleAbsence::NotRecorded) => {
                            match padding {
                                Some(padding) => values.push(padding),
                                None => {
                                    return Err(format!(
                                        "compressed transient channel '{canonical}' has no recorded value at retained sample {index}, which an expanded transient result cannot represent"
                                    ));
                                }
                            }
                        }
                        TransientChannelSample::Absent(TransientSampleAbsence::NonFinite) => {
                            return Err(format!(
                                "compressed transient channel '{canonical}' is absent at retained sample {index} because the producing run computed a non-finite value; read it from the compressed channels, which keep the absence typed"
                            ));
                        }
                    }
                }
                Ok(values)
            };

            match channel.descriptor.role() {
                TransientChannelRole::NodeVoltage { node, .. } => {
                    num_nodes += 1;
                    node_names.push(node.clone());
                    voltages.push(expand(None)?);
                }
                TransientChannelRole::BranchCurrent { branch } => {
                    branch_names.push(branch.clone());
                    branch_currents.push(expand(None)?);
                }
                TransientChannelRole::DeviceObservable { device, parameter } => {
                    device_op_traces.push(TransientDeviceOpTrace {
                        device_name: device.clone(),
                        parameter: parameter.clone(),
                        values: expand(Some(Value::NAN))?,
                    });
                }
                TransientChannelRole::DeviceStore { store } => {
                    store_traces.push(TransientStoreTrace {
                        name: store.clone(),
                        values: expand(None)?,
                    });
                }
            }
        }

        Ok(TransientResult {
            time: self.time,
            step_sizes: self.step_sizes,
            voltages,
            branch_currents,
            num_nodes,
            node_names,
            branch_names,
            digital_traces: self.digital_traces,
            real_traces: self.real_traces,
            device_op_traces,
            store_traces,
            fft_results: self.post_results.fft.clone(),
        })
    }
}

impl TryFrom<TransientResultCompressed> for TransientResult {
    type Error = String;

    fn try_from(compressed: TransientResultCompressed) -> Result<Self, Self::Error> {
        compressed.try_into_transient()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn result_on_grid(time: Vec<Value>) -> TransientResult {
        let values = time.iter().map(|time| 2.0 * time).collect::<Vec<_>>();
        TransientResult {
            step_sizes: vec![0.0; time.len()],
            time,
            voltages: vec![values],
            branch_currents: Vec::new(),
            num_nodes: 1,
            node_names: vec!["out".to_string()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        }
    }

    #[test]
    fn voltage_access_checks_time_before_synthesizing_ground() {
        let result = result_on_grid(vec![0.0, 1.0]);

        assert_eq!(result.try_voltage_at(0, 0), Some(0.0));
        assert_eq!(result.try_voltage_at(0, 1), Some(0.0));
        assert_eq!(result.try_voltage_at_named("0", 1), Some(0.0));
        assert_eq!(result.voltage_at(0, 1), 0.0);

        assert_eq!(result.try_voltage_at(0, 2), None);
        assert_eq!(result.try_voltage_at_named("0", 2), None);
        let panic = std::panic::catch_unwind(|| result.voltage_at(0, 2));
        assert!(
            panic.is_err(),
            "unchecked ground access must reject invalid time indices"
        );
    }

    #[test]
    fn voltage_access_rejects_samples_outside_the_time_grid() {
        let mut result = result_on_grid(vec![0.0, 1.0]);
        result.voltages[0].push(4.0);

        assert_eq!(result.try_voltage_at(1, 1), Some(2.0));
        assert_eq!(result.try_voltage_at(1, 2), None);
        assert_eq!(result.try_voltage_at(2, 0), None);
    }

    #[test]
    fn transient_output_projection_preserves_full_history_and_selects_schedule() {
        let result = result_on_grid(vec![0.0, 0.5, 1.0, 1.5, 2.0]);

        let all = result
            .output_projection(&[], None, 0.5, 1.5, 100)
            .expect("ordinary output projection succeeds");
        assert_eq!(all.times(), &[0.5, 1.0, 1.5]);

        let scheduled = result
            .output_projection(&[0.0, 1.0], None, 0.5, 2.0, 100)
            .expect("scheduled output projection succeeds");
        assert_eq!(scheduled.times(), &[1.0, 2.0]);
        assert_eq!(
            scheduled
                .project(&result.voltages[0])
                .expect("aligned waveform projects"),
            vec![2.0, 4.0]
        );
        assert_eq!(result.time.len(), 5, "projection must not mutate history");
    }

    #[test]
    fn transient_output_projection_requires_exact_accepted_schedule_points() {
        let result = result_on_grid(vec![0.0, 1.0, 2.0]);
        let error = result
            .output_projection(&[1.5], None, 0.0, 2.0, 100)
            .expect_err("output points are solver stops, not interpolation requests");
        assert!(error.contains("did not land"));

        let projection = result
            .output_projection(&[0.0, 2.0], None, 0.0, 2.0, 100)
            .expect("explicit zero and deduplicated stop are valid");
        assert_eq!(projection.times(), &[0.0, 2.0]);
        assert!(projection.project(&[1.0]).is_err());
    }

    #[test]
    fn transient_output_projection_rejects_invalid_result_grids() {
        for time in [vec![], vec![0.0, 0.0], vec![0.0, Value::NAN]] {
            let result = result_on_grid(time);
            assert!(result.output_projection(&[], None, 0.0, 1.0, 100).is_err());
        }

        let result = result_on_grid(vec![0.0, 1.0]);
        for schedule in [vec![-1.0], vec![Value::NAN], vec![Value::INFINITY]] {
            assert!(
                result
                    .output_projection(&schedule, None, 0.0, 1.0, 100)
                    .is_err()
            );
        }
    }

    #[test]
    fn interval_output_projection_anchors_at_resume_and_interpolates_off_grid() {
        let initial = 2.0e-4;
        let stop = initial + 6.0e-8;
        let result = result_on_grid(vec![initial, initial + 3.0e-8, stop]);
        let schedule = XyceOutputIntervalSchedule {
            initial_interval: 2.0e-8,
            intervals: Vec::new(),
        };
        let projection = result
            .output_projection(&[], Some(&schedule), 0.0, stop, 10)
            .expect("resumed interval output projects");
        let second = initial + 2.0e-8;
        let third = second + 2.0e-8;
        assert_eq!(
            projection
                .times()
                .iter()
                .map(|time| time.to_bits())
                .collect::<Vec<_>>(),
            [initial, second, third, stop].map(Value::to_bits)
        );
        let projected = projection
            .project(&result.voltages[0])
            .expect("affine waveform projects");
        for (&time, &value) in projection.times().iter().zip(&projected) {
            assert!((value - 2.0 * time).abs() <= 4.0 * Value::EPSILON * time.abs().max(1.0));
        }
    }

    #[test]
    fn interval_output_projection_resets_at_transitions_and_bounds_rows() {
        let result = result_on_grid(vec![0.0, 0.45, 0.8, 1.0]);
        let schedule = XyceOutputIntervalSchedule {
            initial_interval: 0.3,
            intervals: vec![crate::netlist::XyceOutputInterval {
                time: 0.5,
                interval: 0.2,
            }],
        };
        let projection = result
            .output_projection(&[], Some(&schedule), 0.0, 1.0, 6)
            .expect("transition schedule projects");
        let after_transition = 0.5 + 0.2;
        assert_eq!(projection.times(), &[0.0, 0.3, 0.5, after_transition, 0.9]);
        assert!(
            result
                .output_projection(&[], Some(&schedule), 0.0, 1.0, 4)
                .is_err()
        );
    }

    #[test]
    fn interval_output_projection_preserves_run_lattice_after_off_lattice_tstart() {
        let result = result_on_grid(vec![0.0, 0.95, 1.02, 1.2]);
        let schedule = XyceOutputIntervalSchedule {
            initial_interval: 0.1,
            intervals: Vec::new(),
        };
        let projection = result
            .output_projection(&[], Some(&schedule), 0.95, 1.2, 20)
            .expect("off-lattice TSTART projects");
        let xyce_next = 0.999_999_999_999_999_9_f64;
        let xyce_third = xyce_next + 0.1;
        assert_eq!(xyce_next.to_bits(), 0x3fefffffffffffff);
        assert_eq!(
            projection
                .times()
                .iter()
                .map(|time| time.to_bits())
                .collect::<Vec<_>>(),
            [0.95, xyce_next, xyce_third, 1.2].map(Value::to_bits)
        );
    }

    #[test]
    fn interval_output_projection_replays_transition_rounding_from_accepted_grid() {
        let schedule = XyceOutputIntervalSchedule {
            initial_interval: 0.5,
            intervals: vec![crate::netlist::XyceOutputInterval {
                time: 0.5,
                interval: 0.1,
            }],
        };
        let separate_steps = result_on_grid(vec![0.0, 0.55, 0.65, 0.75, 0.85, 1.0])
            .output_projection(&[], Some(&schedule), 0.0, 1.0, 20)
            .expect("separate accepted steps project");
        let leap = result_on_grid(vec![0.0, 0.55, 0.65, 0.85, 1.0])
            .output_projection(&[], Some(&schedule), 0.0, 1.0, 20)
            .expect("accepted-step leap projects");

        let separate_eight_tenths = separate_steps
            .times()
            .iter()
            .copied()
            .min_by(|left, right| (left - 0.8).abs().total_cmp(&(right - 0.8).abs()))
            .expect("separate grid has output rows");
        let leap_eight_tenths = leap
            .times()
            .iter()
            .copied()
            .min_by(|left, right| (left - 0.8).abs().total_cmp(&(right - 0.8).abs()))
            .expect("leap grid has output rows");
        assert_eq!(separate_eight_tenths.to_bits(), 0x3fe9_9999_9999_999a);
        assert_eq!(leap_eight_tenths.to_bits(), 0x3fe9_9999_9999_9999);
    }

    #[test]
    fn interval_output_projection_preserves_duplicate_final_events_and_states() {
        let result = result_on_grid(vec![0.0, 0.95, 1.0]);
        let schedule = XyceOutputIntervalSchedule {
            initial_interval: 0.3,
            intervals: Vec::new(),
        };
        let projection = result
            .output_projection(&[], Some(&schedule), 0.0, 1.0, 10)
            .expect("Xyce duplicate-final schedule projects");
        assert_eq!(projection.times().len(), 6);
        assert_eq!(
            projection
                .times()
                .iter()
                .map(|time| time.to_bits())
                .collect::<Vec<_>>(),
            [0.0, 0.3, 0.6, 0.899_999_999_999_999_9, 1.0, 1.0].map(Value::to_bits)
        );

        let projected = projection
            .project(&result.voltages[0])
            .expect("duplicate output events retain their source states");
        assert_eq!(
            projected[projected.len() - 2].to_bits(),
            (2.0f64 * 0.95).to_bits()
        );
        assert_eq!(projected[projected.len() - 1].to_bits(), 2.0f64.to_bits());
    }
}
