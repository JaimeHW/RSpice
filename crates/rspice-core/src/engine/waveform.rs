//! Waveform Recording and Compression
//!
//! Implements efficient storage for transient simulation waveforms using a
//! linear interpolation-based compression algorithm.
//!
//! The key insight is that storing every simulation timestep is wasteful when
//! signals change linearly between points. Instead, we only store points where
//! linear interpolation from the last stored point would introduce unacceptable
//! error.
//!
//! # Algorithm
//!
//! For each new point (t, v):
//! 1. If it's the first point, always store it
//! 2. Calculate what value linear interpolation would predict at time t
//! 3. If |actual - interpolated| > abs_tol + rel_tol * |actual|, then:
//!    - Store the *previous* point (to preserve the slope before the change)
//!    - Update the reference point
//! 4. Always store the final point
//!
//! This achieves 10-100x compression for typical waveforms while preserving
//! all significant signal changes.

use crate::Value;

//=============================================================================
// Configuration
//=============================================================================

/// Configuration for waveform compression
#[derive(Debug, Clone)]
pub struct CompressionConfig {
    /// Absolute tolerance in each channel's native units.
    /// Points within this absolute error of the interpolated value are skipped.
    pub abs_tol: Value,

    /// Relative tolerance for storing points (fraction)
    /// Points within this relative error are skipped
    pub rel_tol: Value,

    /// Whether compression is enabled
    /// When disabled, all points are stored (useful for debugging)
    pub enabled: bool,

    /// Maximum time between retained points (prevents over-compression).
    /// The historical field name is retained for API compatibility. Set to
    /// 0.0 to impose no time-axis gap limit.
    pub min_interval: Value,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            abs_tol: 1e-6, // 1 microvolt
            rel_tol: 1e-3, // 0.1%
            enabled: true,
            min_interval: 0.0,
        }
    }
}

impl CompressionConfig {
    /// No compression (store all points)
    pub fn none() -> Self {
        Self {
            enabled: false,
            ..Self::default()
        }
    }

    /// Aggressive compression (good for long simulations)
    pub fn aggressive() -> Self {
        Self {
            abs_tol: 1e-5,
            rel_tol: 1e-2, // 1%
            enabled: true,
            min_interval: 0.0,
        }
    }
}

//=============================================================================
// Per-Signal State
//=============================================================================

/// State for one signal channel during recording
#[derive(Debug, Clone)]
struct ChannelState {
    /// Last actually stored point: (time, value)
    last_stored: (Value, Value),

    /// Previous point (may need to be stored on slope change)
    previous: (Value, Value),

    /// Whether we have a pending previous point
    has_previous: bool,
}

impl ChannelState {
    fn new(t0: Value, v0: Value) -> Self {
        Self {
            last_stored: (t0, v0),
            previous: (t0, v0),
            has_previous: false,
        }
    }

    /// Calculate interpolated value at time t from last stored point to current
    fn interpolate(&self, t_current: Value, v_current: Value, t_query: Value) -> Value {
        let (t_stored, v_stored) = self.last_stored;
        let dt = t_current - t_stored;

        if dt.abs() < 1e-30 {
            return v_stored;
        }

        let slope = (v_current - v_stored) / dt;
        v_stored + slope * (t_query - t_stored)
    }

    /// Check if previous point should be stored (interpolation error too high)
    fn should_store_previous(
        &self,
        t_current: Value,
        v_current: Value,
        config: &CompressionConfig,
    ) -> bool {
        if !self.has_previous {
            return false;
        }

        let (t_prev, v_prev) = self.previous;
        let (t_stored, _) = self.last_stored;

        // A positive interval is a hard maximum retained gap whenever the
        // source grid contains an intermediate point that can satisfy it.
        if config.min_interval > 0.0 && t_current - t_stored > config.min_interval {
            return true;
        }

        // Calculate what linear interpolation would predict at t_prev
        let v_interpolated = self.interpolate(t_current, v_current, t_prev);

        // Calculate error
        let error = (v_prev - v_interpolated).abs();
        let threshold = config.abs_tol + config.rel_tol * v_prev.abs();

        error > threshold
    }
}

//=============================================================================
// Waveform Recorder
//=============================================================================

/// Compressed waveform storage for multiple channels
///
/// Uses the linear interpolation algorithm to achieve 10-100x compression
/// while preserving all significant signal transitions.
#[derive(Debug)]
pub struct WaveformRecorder {
    /// Compression configuration
    config: CompressionConfig,

    /// Number of channels (signals)
    num_channels: usize,

    /// Per-channel compression state
    channel_states: Vec<ChannelState>,

    /// Stored time points (shared across all channels for simplicity)
    /// In a more sophisticated implementation, each channel could have
    /// independent time points, but this adds complexity for post-processing
    times: Vec<Value>,

    /// Stored values: values`[channel][point_index]`
    values: Vec<Vec<Value>>,

    /// Total number of input points (for compression ratio stats)
    input_count: usize,
}

impl WaveformRecorder {
    /// Create a new recorder for the given number of channels
    ///
    /// # Arguments
    /// * `num_channels` - Number of signals to record
    /// * `t0` - Initial time
    /// * `initial_values` - Initial values for each channel
    /// * `config` - Compression configuration
    pub fn new(
        num_channels: usize,
        t0: Value,
        initial_values: &[Value],
        config: CompressionConfig,
    ) -> Result<Self, String> {
        validate_channel_count(
            "initial waveform sample",
            initial_values.len(),
            num_channels,
        )?;

        let channel_states: Vec<_> = initial_values
            .iter()
            .map(|&v| ChannelState::new(t0, v))
            .collect();

        let values: Vec<_> = initial_values.iter().map(|&v| vec![v]).collect();

        Ok(Self {
            config,
            num_channels,
            channel_states,
            times: vec![t0],
            values,
            input_count: 1,
        })
    }

    /// Record a new time point with values for all channels
    ///
    /// Returns true if any point was actually stored (useful for debugging)
    pub fn record(&mut self, t: Value, values: &[Value]) -> Result<bool, String> {
        validate_channel_count("waveform sample", values.len(), self.num_channels)?;

        self.input_count += 1;

        // If compression is disabled, store everything
        if !self.config.enabled {
            self.times.push(t);
            for (ch, &v) in values.iter().enumerate() {
                self.values[ch].push(v);
                self.channel_states[ch].last_stored = (t, v);
            }
            return Ok(true);
        }

        // Check each channel for whether we need to store the previous point
        let mut any_stored = false;

        for (ch, &v) in values.iter().enumerate() {
            if self.channel_states[ch].should_store_previous(t, v, &self.config) {
                // We need to store the previous point to preserve slope change
                any_stored = true;
            }
        }

        if any_stored {
            // Find the earliest previous point time that needs storing
            let store_time = self
                .channel_states
                .iter()
                .filter(|s| s.has_previous)
                .map(|s| s.previous.0)
                .fold(Value::MAX, |a, b| a.min(b));

            // Store that time point with interpolated values for all channels
            self.store_point(store_time, values, t);
        }

        // Update previous point for all channels
        for (ch, &v) in values.iter().enumerate() {
            self.channel_states[ch].previous = (t, v);
            self.channel_states[ch].has_previous = true;
        }

        Ok(any_stored)
    }

    /// Store a point at the given time
    fn store_point(&mut self, t_store: Value, current_values: &[Value], t_current: Value) {
        self.times.push(t_store);

        for (ch, &v_current) in current_values.iter().enumerate() {
            let state = &mut self.channel_states[ch];

            // If this channel had a previous point at this time, use it
            // Otherwise interpolate to this time
            let v_store = if state.has_previous && (state.previous.0 - t_store).abs() < 1e-30 {
                state.previous.1
            } else {
                state.interpolate(t_current, v_current, t_store)
            };

            self.values[ch].push(v_store);
            state.last_stored = (t_store, v_store);
            state.has_previous = false;
        }
    }

    /// Finalize recording, ensuring the last point is stored
    ///
    /// This must be called at the end of simulation to ensure the final
    /// values are recorded.
    pub fn finalize(&mut self, t_final: Value, final_values: &[Value]) -> Result<(), String> {
        validate_channel_count(
            "final waveform sample",
            final_values.len(),
            self.num_channels,
        )?;

        // Always store the final point
        let last_time = *self.times.last().unwrap_or(&0.0);
        if (t_final - last_time).abs() > 1e-30 {
            self.times.push(t_final);
            for (ch, &v) in final_values.iter().enumerate() {
                self.values[ch].push(v);
                self.channel_states[ch].last_stored = (t_final, v);
            }
        }
        Ok(())
    }

    /// Get the stored time points
    pub fn times(&self) -> &[Value] {
        &self.times
    }

    /// Get all stored values, indexed `[channel][point]`.
    pub fn all_values(&self) -> &[Vec<Value>] {
        &self.values
    }

    /// Get the number of stored points
    pub fn stored_count(&self) -> usize {
        self.times.len()
    }

    /// Get the total number of input points
    pub fn input_count(&self) -> usize {
        self.input_count
    }

    /// Get the compression ratio (input_count / stored_count)
    pub fn compression_ratio(&self) -> Value {
        self.input_count as Value / self.stored_count() as Value
    }
}

//=============================================================================
// Result Structures
//=============================================================================

/// Compressed transient result with metadata
#[derive(Debug, Clone)]
pub struct TransientResultCompressed {
    /// Time points (non-uniform due to compression)
    pub time: Vec<Value>,

    /// Exact integration interval associated with each retained time point.
    pub step_sizes: Vec<Value>,

    /// Voltage waveforms, indexed `[node][point]`.
    pub voltages: Vec<Vec<Value>>,

    /// Branch-current waveforms, indexed `[branch][point]`.
    pub branch_currents: Vec<Vec<Value>>,

    /// Number of nodes
    pub num_nodes: usize,

    /// Node names aligned with `voltages`
    pub node_names: Vec<String>,

    /// Branch names aligned with `branch_currents`.
    pub branch_names: Vec<String>,

    /// Device operating-point waveforms requested by the authored output
    /// projection. Values are decimated on the same retained time grid.
    pub device_op_traces: Vec<super::TransientDeviceOpTrace>,

    /// Typed non-solution device store waveforms.
    pub store_traces: Vec<super::TransientStoreTrace>,

    /// `.FFT` post-processing is computed before waveform decimation and is
    /// retained losslessly alongside the compressed time-domain channels.
    pub fft_results: Vec<super::TransientFftResult>,

    /// Compression ratio achieved
    pub compression_ratio: Value,

    /// Total number of simulation points before compression
    pub input_points: usize,
}

/// Detailed compression statistics
#[derive(Debug, Clone)]
pub struct CompressionStats {
    /// Total input time points
    pub input_points: usize,
    /// Stored time points after compression
    pub stored_points: usize,
    /// Number of channels (signals)
    pub num_channels: usize,
    /// Compression ratio (input / stored)
    pub compression_ratio: Value,
    /// Estimated input memory (bytes, without compression)
    pub input_bytes: usize,
    /// Actual stored memory (bytes)
    pub stored_bytes: usize,
    /// Memory savings ratio
    pub memory_savings_ratio: Value,
}

impl CompressionStats {
    /// Calculate compression statistics
    pub fn calculate(input_points: usize, stored_points: usize, num_channels: usize) -> Self {
        // f64 is 8 bytes
        let bytes_per_value = std::mem::size_of::<Value>();

        // Input: time vec + N channel vecs, all with input_points entries
        let input_bytes = bytes_per_value * input_points * (1 + num_channels);

        // Stored: time vec + N channel vecs, all with stored_points entries
        let stored_bytes = bytes_per_value * stored_points * (1 + num_channels);

        let compression_ratio = if stored_points > 0 {
            input_points as Value / stored_points as Value
        } else {
            1.0
        };

        let memory_savings_ratio = if stored_bytes > 0 {
            input_bytes as Value / stored_bytes as Value
        } else {
            1.0
        };

        Self {
            input_points,
            stored_points,
            num_channels,
            compression_ratio,
            input_bytes,
            stored_bytes,
            memory_savings_ratio,
        }
    }

    /// Format as human-readable summary
    pub fn summary(&self) -> String {
        format!(
            "{}/{} points ({:.1}x), {:.0} KB → {:.0} KB ({:.1}x memory savings)",
            self.stored_points,
            self.input_points,
            self.compression_ratio,
            self.input_bytes as f64 / 1024.0,
            self.stored_bytes as f64 / 1024.0,
            self.memory_savings_ratio,
        )
    }
}

impl TransientResultCompressed {
    /// Validate the complete analog result inventory and retained-grid
    /// alignment before exposing or expanding this result.
    pub fn validate(&self) -> Result<(), String> {
        let point_count = self.time.len();
        if self.step_sizes.len() != point_count {
            return Err(format!(
                "compressed transient has {} step sizes for {point_count} time points",
                self.step_sizes.len()
            ));
        }
        if self.voltages.len() != self.num_nodes || self.node_names.len() != self.num_nodes {
            return Err(format!(
                "compressed transient declares {} nodes but has {} voltage channels and {} node names",
                self.num_nodes,
                self.voltages.len(),
                self.node_names.len()
            ));
        }
        if self.branch_currents.len() != self.branch_names.len() {
            return Err(format!(
                "compressed transient has {} branch-current channels but {} branch names",
                self.branch_currents.len(),
                self.branch_names.len()
            ));
        }
        if self.input_points < point_count {
            return Err(format!(
                "compressed transient retains {point_count} points from an impossible {}-point input",
                self.input_points
            ));
        }
        if !self.compression_ratio.is_finite() || self.compression_ratio < 1.0 {
            return Err(format!(
                "compressed transient has invalid compression ratio {}",
                self.compression_ratio
            ));
        }
        let expected_ratio = if point_count == 0 {
            1.0
        } else {
            self.input_points as Value / point_count as Value
        };
        let ratio_tolerance = 16.0 * Value::EPSILON * expected_ratio.max(1.0);
        if (self.compression_ratio - expected_ratio).abs() > ratio_tolerance {
            return Err(format!(
                "compressed transient ratio {} is inconsistent with {} retained points from a {}-point input (expected {expected_ratio})",
                self.compression_ratio, point_count, self.input_points
            ));
        }
        if self
            .time
            .windows(2)
            .any(|window| !window[0].is_finite() || window[1] <= window[0])
            || self.time.last().is_some_and(|time| !time.is_finite())
        {
            return Err(
                "compressed transient time points must be finite and strictly increasing"
                    .to_string(),
            );
        }
        if self
            .step_sizes
            .iter()
            .any(|step| !step.is_finite() || *step < 0.0)
        {
            return Err(
                "compressed transient step sizes must be finite and non-negative".to_string(),
            );
        }

        for (kind, name, values, may_be_projected_out) in self
            .voltages
            .iter()
            .enumerate()
            .map(|(index, values)| ("voltage", self.node_names[index].as_str(), values, true))
            .chain(
                self.branch_currents
                    .iter()
                    .enumerate()
                    .map(|(index, values)| {
                        (
                            "branch-current",
                            self.branch_names[index].as_str(),
                            values,
                            true,
                        )
                    }),
            )
            .chain(self.device_op_traces.iter().map(|trace| {
                (
                    "device operating-point",
                    trace.parameter.as_str(),
                    &trace.values,
                    false,
                )
            }))
            .chain(
                self.store_traces
                    .iter()
                    .map(|trace| ("device store", trace.name.as_str(), &trace.values, false)),
            )
        {
            if values.len() != point_count && !(may_be_projected_out && values.is_empty()) {
                return Err(format!(
                    "compressed transient {kind} channel '{name}' has {} values for {point_count} time points",
                    values.len()
                ));
            }
            if values.iter().any(|value| !value.is_finite()) {
                return Err(format!(
                    "compressed transient {kind} channel '{name}' contains a non-finite value"
                ));
            }
        }
        Ok(())
    }

    fn aligned_values<'a>(&self, values: &'a [Value]) -> Option<&'a [Value]> {
        (values.len() == self.time.len()).then_some(values)
    }

    fn aligned_channel_values(&self, node: usize) -> Option<&[Value]> {
        if node >= self.num_nodes {
            return None;
        }

        let values = self.voltages.get(node)?;
        self.aligned_values(values)
    }

    fn interpolate_values(&self, values: &[Value], time: Value) -> Option<Value> {
        if self.time.is_empty() || !time.is_finite() {
            return None;
        }
        let values = self.aligned_values(values)?;
        let times = &self.time;
        if time <= times[0] {
            return Some(values[0]);
        }
        if time >= *times.last()? {
            return values.last().copied();
        }
        let index = match times.binary_search_by(|candidate| candidate.total_cmp(&time)) {
            Ok(index) => return values.get(index).copied(),
            Err(index) => index.checked_sub(1)?,
        };
        let t0 = times[index];
        let t1 = times[index + 1];
        let fraction = (time - t0) / (t1 - t0);
        Some(values[index] + fraction * (values[index + 1] - values[index]))
    }

    /// Get value at arbitrary time via linear interpolation
    ///
    /// This is how compressed waveforms are read - the stored points
    /// are the control points for piecewise linear interpolation.
    pub fn interpolate(&self, node: usize, time: Value) -> Option<Value> {
        let values = self.aligned_channel_values(node)?;
        self.interpolate_values(values, time)
    }

    /// Get the retained branch-current waveform for a canonical branch name.
    pub fn try_branch_current_waveform_named(&self, name: &str) -> Option<&[Value]> {
        let index = self
            .branch_names
            .iter()
            .position(|candidate| candidate.eq_ignore_ascii_case(name))?;
        self.aligned_values(self.branch_currents.get(index)?)
    }

    /// Interpolate a retained branch-current waveform by canonical name.
    pub fn interpolate_branch_current_named(&self, name: &str, time: Value) -> Option<Value> {
        self.interpolate_values(self.try_branch_current_waveform_named(name)?, time)
    }

    /// Get a retained device operating-point waveform by device and parameter.
    pub fn try_device_op_waveform_named(
        &self,
        device_name: &str,
        parameter: &str,
    ) -> Option<&[Value]> {
        let trace = self.device_op_traces.iter().find(|trace| {
            trace.device_name.eq_ignore_ascii_case(device_name)
                && trace.parameter.eq_ignore_ascii_case(parameter)
        })?;
        self.aligned_values(&trace.values)
    }

    /// Interpolate a retained device operating-point waveform.
    pub fn interpolate_device_op_named(
        &self,
        device_name: &str,
        parameter: &str,
        time: Value,
    ) -> Option<Value> {
        self.interpolate_values(
            self.try_device_op_waveform_named(device_name, parameter)?,
            time,
        )
    }

    /// Get a retained typed device-store waveform by canonical name.
    pub fn try_store_waveform_named(&self, name: &str) -> Option<&[Value]> {
        let trace = self
            .store_traces
            .iter()
            .find(|trace| trace.name.eq_ignore_ascii_case(name))?;
        self.aligned_values(&trace.values)
    }

    /// Interpolate a retained typed device-store waveform by canonical name.
    pub fn interpolate_store_named(&self, name: &str, time: Value) -> Option<Value> {
        self.interpolate_values(self.try_store_waveform_named(name)?, time)
    }

    /// Sample waveform at uniform intervals
    ///
    /// Useful for FFT or other analysis that requires uniform sampling.
    pub fn resample(&self, node: usize, num_points: usize) -> Option<(Vec<Value>, Vec<Value>)> {
        if node >= self.num_nodes || self.time.is_empty() || num_points < 2 {
            return None;
        }
        self.aligned_channel_values(node)?;

        let t_start = self.time[0];
        let t_end = *self.time.last()?;
        let dt = (t_end - t_start) / (num_points - 1) as Value;

        let times: Vec<_> = (0..num_points).map(|i| t_start + i as Value * dt).collect();

        let values = times
            .iter()
            .map(|&time| self.interpolate(node, time))
            .collect::<Option<Vec<_>>>()?;

        Some((times, values))
    }
}

fn validate_channel_count(context: &str, actual: usize, expected: usize) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "{context} has {actual} value(s) but recorder expects {expected} channel(s)"
        ))
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn malformed_compressed_result(voltages: Vec<Vec<Value>>) -> TransientResultCompressed {
        TransientResultCompressed {
            time: vec![0.0, 1.0, 2.0],
            step_sizes: vec![0.0, 1.0, 1.0],
            voltages,
            branch_currents: Vec::new(),
            num_nodes: 1,
            node_names: vec!["out".to_string()],
            branch_names: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
            compression_ratio: 1.0,
            input_points: 3,
        }
    }

    #[test]
    fn compressed_result_rejects_missing_or_misaligned_voltage_channels() {
        let missing_channel = malformed_compressed_result(Vec::new());
        assert_eq!(missing_channel.interpolate(0, 0.5), None);
        assert_eq!(missing_channel.resample(0, 3), None);
        assert!(missing_channel.validate().is_err());

        let short_channel = malformed_compressed_result(vec![vec![0.0]]);
        assert_eq!(short_channel.interpolate(0, 0.5), None);
        assert_eq!(short_channel.resample(0, 3), None);
        assert!(short_channel.validate().is_err());
    }

    #[test]
    fn compressed_result_validates_projected_out_solution_channels() {
        let mut projected = malformed_compressed_result(vec![Vec::new()]);
        projected.branch_names = vec!["V1".to_string()];
        projected.branch_currents = vec![Vec::new()];

        projected
            .validate()
            .expect("empty projected-out voltage and current channels are typed missingness");
        assert_eq!(projected.interpolate(0, 0.5), None);
        assert_eq!(projected.try_branch_current_waveform_named("v1"), None);
    }

    #[test]
    fn compressed_result_rejects_inconsistent_ratio_metadata() {
        let mut malformed = malformed_compressed_result(vec![vec![0.0, 1.0, 2.0]]);
        malformed.input_points = 6;
        malformed.compression_ratio = 1.5;

        let error = malformed
            .validate()
            .expect_err("ratio metadata must agree with retained and input point counts");
        assert!(error.contains("inconsistent"), "unexpected error: {error}");
    }

    #[test]
    fn recorder_new_rejects_mismatched_initial_values_without_panicking() {
        let err = WaveformRecorder::new(2, 0.0, &[1.0], CompressionConfig::none())
            .expect_err("initial sample width must be validated");
        assert!(err.contains("initial waveform sample"));
    }

    #[test]
    fn recorder_record_rejects_mismatched_values_without_panicking() {
        let mut recorder = WaveformRecorder::new(2, 0.0, &[1.0, 2.0], CompressionConfig::none())
            .expect("recorder initializes");

        let err = recorder
            .record(1.0, &[3.0])
            .expect_err("record sample width must be validated");
        assert!(err.contains("waveform sample"));
    }

    #[test]
    fn recorder_finalize_rejects_mismatched_values_without_panicking() {
        let mut recorder = WaveformRecorder::new(2, 0.0, &[1.0, 2.0], CompressionConfig::none())
            .expect("recorder initializes");

        let err = recorder
            .finalize(1.0, &[3.0])
            .expect_err("final sample width must be validated");
        assert!(err.contains("final waveform sample"));
    }

    #[test]
    fn recorder_positive_interval_limits_retained_gap() {
        let config = CompressionConfig {
            abs_tol: 1.0,
            rel_tol: 1.0,
            enabled: true,
            min_interval: 1.0,
        };
        let mut recorder =
            WaveformRecorder::new(1, 0.0, &[0.0], config).expect("recorder initializes");
        for index in 1..=8 {
            let time = index as Value * 0.25;
            recorder
                .record(time, &[time])
                .expect("aligned sample records");
        }
        recorder
            .finalize(2.0, &[2.0])
            .expect("aligned final sample records");

        assert!(
            recorder
                .times()
                .windows(2)
                .all(|window| window[1] - window[0] <= 1.0),
            "retained grid exceeded maximum interval: {:?}",
            recorder.times()
        );
    }
}
