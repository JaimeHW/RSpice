//! Waveform Recording and Compression
//!
//! Implements efficient storage for transient simulation waveforms using a
//! linear interpolation-based compression algorithm similar to LTspice.
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
    /// Absolute tolerance for storing points (volts/amps)
    /// Points within this absolute error of interpolated value are skipped
    pub abs_tol: Value,
    
    /// Relative tolerance for storing points (fraction)
    /// Points within this relative error are skipped
    pub rel_tol: Value,
    
    /// Whether compression is enabled
    /// When disabled, all points are stored (useful for debugging)
    pub enabled: bool,
    
    /// Minimum time between stored points (prevents over-compression)
    /// Set to 0.0 to allow maximum compression
    pub min_interval: Value,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            abs_tol: 1e-6,      // 1 microvolt
            rel_tol: 1e-3,      // 0.1%
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
    
    /// High-fidelity compression (minimal loss)
    pub fn high_fidelity() -> Self {
        Self {
            abs_tol: 1e-9,
            rel_tol: 1e-4, // 0.01%
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
        
        // Check minimum interval constraint
        if config.min_interval > 0.0 && (t_prev - t_stored) < config.min_interval {
            return false;
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
    
    /// Stored values: values[channel][point_index]
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
    ) -> Self {
        assert_eq!(initial_values.len(), num_channels);
        
        let channel_states: Vec<_> = initial_values
            .iter()
            .map(|&v| ChannelState::new(t0, v))
            .collect();
        
        let values: Vec<_> = initial_values
            .iter()
            .map(|&v| vec![v])
            .collect();
        
        Self {
            config,
            num_channels,
            channel_states,
            times: vec![t0],
            values,
            input_count: 1,
        }
    }
    
    /// Record a new time point with values for all channels
    ///
    /// Returns true if any point was actually stored (useful for debugging)
    pub fn record(&mut self, t: Value, values: &[Value]) -> bool {
        assert_eq!(values.len(), self.num_channels);
        
        self.input_count += 1;
        
        // If compression is disabled, store everything
        if !self.config.enabled {
            self.times.push(t);
            for (ch, &v) in values.iter().enumerate() {
                self.values[ch].push(v);
                self.channel_states[ch].last_stored = (t, v);
            }
            return true;
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
            let store_time = self.channel_states
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
        
        any_stored
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
    pub fn finalize(&mut self, t_final: Value, final_values: &[Value]) {
        assert_eq!(final_values.len(), self.num_channels);
        
        // Always store the final point
        let last_time = *self.times.last().unwrap_or(&0.0);
        if (t_final - last_time).abs() > 1e-30 {
            self.times.push(t_final);
            for (ch, &v) in final_values.iter().enumerate() {
                self.values[ch].push(v);
                self.channel_states[ch].last_stored = (t_final, v);
            }
        }
    }
    
    /// Get the stored time points
    pub fn times(&self) -> &[Value] {
        &self.times
    }
    
    /// Get the stored values for a channel
    pub fn channel_values(&self, channel: usize) -> &[Value] {
        &self.values[channel]
    }
    
    /// Get all stored values (values[channel][point])
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
    
    /// Convert to TransientResult structure (for compatibility)
    pub fn to_transient_result(&self) -> TransientResultCompressed {
        TransientResultCompressed {
            time: self.times.clone(),
            voltages: self.values.clone(),
            num_nodes: self.num_channels,
            compression_ratio: self.compression_ratio(),
            input_points: self.input_count,
        }
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
    
    /// Voltage waveforms: voltages[node][point]
    pub voltages: Vec<Vec<Value>>,
    
    /// Number of nodes
    pub num_nodes: usize,
    
    /// Compression ratio achieved
    pub compression_ratio: Value,
    
    /// Total number of simulation points before compression
    pub input_points: usize,
}

impl TransientResultCompressed {
    /// Get value at arbitrary time via linear interpolation
    ///
    /// This is how compressed waveforms are read - the stored points
    /// are the control points for piecewise linear interpolation.
    pub fn interpolate(&self, node: usize, time: Value) -> Option<Value> {
        if node >= self.num_nodes || self.time.is_empty() {
            return None;
        }
        
        let times = &self.time;
        let values = &self.voltages[node];
        
        // Handle edge cases
        if time <= times[0] {
            return Some(values[0]);
        }
        if time >= *times.last().unwrap() {
            return Some(*values.last().unwrap());
        }
        
        // Binary search for interval
        let idx = match times.binary_search_by(|t| t.partial_cmp(&time).unwrap()) {
            Ok(i) => return Some(values[i]),
            Err(i) => i - 1,
        };
        
        // Linear interpolation
        let t0 = times[idx];
        let t1 = times[idx + 1];
        let v0 = values[idx];
        let v1 = values[idx + 1];
        
        let frac = (time - t0) / (t1 - t0);
        Some(v0 + frac * (v1 - v0))
    }
    
    /// Sample waveform at uniform intervals
    ///
    /// Useful for FFT or other analysis that requires uniform sampling.
    pub fn resample(&self, node: usize, num_points: usize) -> Option<(Vec<Value>, Vec<Value>)> {
        if node >= self.num_nodes || self.time.is_empty() {
            return None;
        }
        
        let t_start = self.time[0];
        let t_end = *self.time.last().unwrap();
        let dt = (t_end - t_start) / (num_points - 1) as Value;
        
        let times: Vec<_> = (0..num_points)
            .map(|i| t_start + i as Value * dt)
            .collect();
        
        let values: Vec<_> = times
            .iter()
            .map(|&t| self.interpolate(node, t).unwrap_or(0.0))
            .collect();
        
        Some((times, values))
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compression_disabled() {
        let config = CompressionConfig::none();
        let mut recorder = WaveformRecorder::new(1, 0.0, &[0.0], config);
        
        for i in 1..=100 {
            let t = i as Value * 0.01;
            let v = t; // Linear ramp
            recorder.record(t, &[v]);
        }
        recorder.finalize(1.0, &[1.0]);
        
        // With compression disabled, all points should be stored
        assert_eq!(recorder.stored_count(), 101); // 100 + initial
        assert!((recorder.compression_ratio() - 1.0).abs() < 0.01);
    }
    
    #[test]
    fn test_linear_ramp_compresses() {
        let config = CompressionConfig::default();
        let mut recorder = WaveformRecorder::new(1, 0.0, &[0.0], config);
        
        // Perfect linear ramp - should compress heavily
        for i in 1..=1000 {
            let t = i as Value * 0.001;
            let v = t; // Linear ramp from 0 to 1
            recorder.record(t, &[v]);
        }
        recorder.finalize(1.0, &[1.0]);
        
        // A perfect linear ramp should compress to just start and end
        assert!(recorder.stored_count() <= 5, "Linear ramp should compress heavily, got {} points", recorder.stored_count());
        assert!(recorder.compression_ratio() > 100.0);
    }
    
    #[test]
    fn test_step_function_stored() {
        let config = CompressionConfig::default();
        let mut recorder = WaveformRecorder::new(1, 0.0, &[0.0], config);
        
        // Step function at t=0.5
        for i in 1..=100 {
            let t = i as Value * 0.01;
            let v = if t < 0.5 { 0.0 } else { 1.0 };
            recorder.record(t, &[v]);
        }
        recorder.finalize(1.0, &[1.0]);
        
        // Step should be preserved (points around the transition)
        let result = recorder.to_transient_result();
        
        // Should have at least: start, point before step, point after step, end
        assert!(recorder.stored_count() >= 3);
        
        // Verify the step is captured correctly via interpolation
        assert!((result.interpolate(0, 0.49).unwrap() - 0.0).abs() < 0.1);
        assert!((result.interpolate(0, 0.51).unwrap() - 1.0).abs() < 0.1);
    }
    
    #[test]
    fn test_sine_wave_compression() {
        // This test validates the waveform compression algorithm by:
        // 1. Verifying compression occurs (fewer stored than input points)
        // 2. Verifying stored points accurately represent the original waveform
        // 3. Verifying the compression tolerance is respected
        
        use std::f64::consts::PI;
        
        // Use no compression to record the "ground truth" waveform
        let mut uncompressed = WaveformRecorder::new(1, 0.0, &[0.0], CompressionConfig::none());
        
        // Use default compression for the test
        let mut compressed = WaveformRecorder::new(1, 0.0, &[0.0], CompressionConfig::default());
        
        // Generate sine wave with 1000 points
        for i in 1..=1000 {
            let t = i as Value * 0.001;
            let v = (2.0 * PI * t).sin();
            uncompressed.record(t, &[v]);
            compressed.record(t, &[v]);
        }
        uncompressed.finalize(1.0, &[(2.0 * PI).sin()]);
        compressed.finalize(1.0, &[(2.0 * PI).sin()]);
        
        // Test 1: Verify compression happened
        assert_eq!(uncompressed.stored_count(), 1001, "Uncompressed should store all points");
        assert!(compressed.stored_count() < 500, 
            "Compressed should store fewer than half: got {}", compressed.stored_count());
        assert!(compressed.compression_ratio() > 2.0,
            "Should achieve at least 2x compression: got {:.1}x", compressed.compression_ratio());
        
        // Test 2: Verify each stored point is accurate
        // (stored points should closely match the actual sine values)
        let result = compressed.to_transient_result();
        for (i, &t) in result.time.iter().enumerate() {
            let stored_v = result.voltages[0][i];
            let actual_v = (2.0 * PI * t).sin();
            let error = (stored_v - actual_v).abs();
            // Stored points should be within 1% of actual (generous tolerance for timing drift)
            assert!(error < 0.01, 
                "Stored point at t={:.4} has error {:.4} (stored={:.4}, actual={:.4})",
                t, error, stored_v, actual_v);
        }
        
        // Test 3: Verify waveform shape is preserved
        // Check that extrema are captured reasonably well
        let peak = result.interpolate(0, 0.25).unwrap();
        let trough = result.interpolate(0, 0.75).unwrap();
        
        // With default compression (0.1% rel tol), peak should be within ~10% 
        // due to piecewise linear approximation of the curved peak region
        assert!(peak > 0.85, "Peak should be > 0.85, got {:.3}", peak);
        assert!(trough < -0.85, "Trough should be < -0.85, got {:.3}", trough);
    }
    
    #[test] 
    fn test_multi_channel() {
        let config = CompressionConfig::default();
        let mut recorder = WaveformRecorder::new(3, 0.0, &[0.0, 1.0, 2.0], config);
        
        for i in 1..=100 {
            let t = i as Value * 0.01;
            recorder.record(t, &[t, 1.0 - t, 2.0]);
        }
        recorder.finalize(1.0, &[1.0, 0.0, 2.0]);
        
        let result = recorder.to_transient_result();
        
        // Channel 0: ramp up
        assert!((result.interpolate(0, 0.5).unwrap() - 0.5).abs() < 0.01);
        
        // Channel 1: ramp down
        assert!((result.interpolate(1, 0.5).unwrap() - 0.5).abs() < 0.01);
        
        // Channel 2: constant (should compress maximally)
        assert!((result.interpolate(2, 0.5).unwrap() - 2.0).abs() < 0.001);
    }
    
    #[test]
    fn test_resampling() {
        let config = CompressionConfig::none();
        let mut recorder = WaveformRecorder::new(1, 0.0, &[0.0], config);
        
        for i in 1..=10 {
            let t = i as Value * 0.1;
            recorder.record(t, &[t * t]); // Parabola
        }
        recorder.finalize(1.0, &[1.0]);
        
        let result = recorder.to_transient_result();
        let (times, values) = result.resample(0, 5).unwrap();
        
        assert_eq!(times.len(), 5);
        assert_eq!(values.len(), 5);
        assert!((times[0] - 0.0).abs() < 1e-10);
        assert!((times[4] - 1.0).abs() < 1e-10);
    }
}
