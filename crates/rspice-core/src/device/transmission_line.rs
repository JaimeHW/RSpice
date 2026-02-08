//! Transmission Line Models
//!
//! Implements lossless and lossy transmission lines for high-frequency simulation.
//!
//! # SPICE Syntax
//! ```text
//! T<name> n1+ n1- n2+ n2- Z0=<impedance> TD=<delay>
//! T1 1 0 2 0 Z0=50 TD=1ns
//! ```
//!
//! # Theory
//! A lossless transmission line is characterized by:
//! - Z0: Characteristic impedance (Ω)
//! - TD: Propagation delay (s)
//!
//! The telegrapher's equations relate voltage and current at both ends:
//! ```text
//! V1(t) + Z0*I1(t) = V2(t-TD) + Z0*I2(t-TD)
//! V2(t) + Z0*I2(t) = V1(t-TD) + Z0*I1(t-TD)
//! ```
//!
//! # Implementation
//! Uses delay buffers to store past values and interpolates for accurate delays.
//! The transmission line is modeled as dependent sources with delay.

use crate::{Value, circuit::NodeId};
use std::collections::VecDeque;

//=============================================================================
// History Buffer for Delay with Cubic Hermite Interpolation
//=============================================================================

/// Sample point with derivative for cubic interpolation
#[derive(Debug, Clone, Copy)]
struct Sample {
    time: Value,
    value: Value,
    slope: Value, // dv/dt at this point
}

/// Circular buffer for storing time history with cubic Hermite interpolation
///
/// Uses cubic Hermite splines for smooth C1-continuous interpolation,
/// which preserves high-frequency content better than linear interpolation.
/// This is critical for transmission line simulation where linear interpolation
/// introduces artificial numerical damping.
#[derive(Debug, Clone)]
struct DelayBuffer {
    /// Samples with time, value, and slope
    data: VecDeque<Sample>,
    /// Maximum storage time
    max_time: Value,
    /// Previous value for slope estimation
    prev_value: Value,
    /// Previous time for slope estimation
    prev_time: Value,
}

impl DelayBuffer {
    fn new(max_time: Value) -> Self {
        Self {
            data: VecDeque::new(),
            max_time,
            prev_value: 0.0,
            prev_time: -1e30, // Very negative so first slope is ~0
        }
    }

    /// Add a new sample with automatic slope estimation
    fn push(&mut self, time: Value, value: Value) {
        // Estimate slope using backward difference
        let dt = time - self.prev_time;
        let slope = if dt > 1e-18 {
            (value - self.prev_value) / dt
        } else {
            0.0
        };

        self.data.push_back(Sample { time, value, slope });

        self.prev_value = value;
        self.prev_time = time;

        // Remove old samples
        while let Some(s) = self.data.front() {
            if time - s.time > self.max_time * 1.5 {
                self.data.pop_front();
            } else {
                break;
            }
        }
    }

    /// Get interpolated value at time (time - delay) using cubic Hermite spline
    ///
    /// Cubic Hermite provides C1 continuity and better preserves:
    /// - High frequency signal content
    /// - Sharp transitions in digital signals
    /// - Phase accuracy at RF frequencies
    fn get_delayed(&self, current_time: Value, delay: Value) -> Value {
        let target_time = current_time - delay;

        if self.data.is_empty() {
            return 0.0;
        }

        // Binary search would be faster for large buffers, but linear is fine
        // for typical transmission line delays (< 100 samples)
        let mut prev: Option<&Sample> = None;

        for s in self.data.iter() {
            if s.time >= target_time {
                if let Some(p) = prev {
                    return Self::cubic_hermite(p, s, target_time);
                }
                return s.value;
            }
            prev = Some(s);
        }

        // Target time is beyond buffer, return last value
        self.data.back().map(|s| s.value).unwrap_or(0.0)
    }

    /// Cubic Hermite spline interpolation between two samples
    ///
    /// Given points p0 and p1 with values v0, v1 and slopes m0, m1,
    /// interpolates smoothly with continuous first derivative.
    ///
    /// H(t) = (2t³ - 3t² + 1)v0 + (t³ - 2t² + t)Δt·m0
    ///      + (-2t³ + 3t²)v1 + (t³ - t²)Δt·m1
    #[inline]
    fn cubic_hermite(p0: &Sample, p1: &Sample, t: Value) -> Value {
        let dt = p1.time - p0.time;
        if dt.abs() < 1e-18 {
            return p1.value;
        }

        // Normalized parameter s ∈ [0, 1]
        let s = (t - p0.time) / dt;
        let s2 = s * s;
        let s3 = s2 * s;

        // Hermite basis functions
        let h00 = 2.0 * s3 - 3.0 * s2 + 1.0; // Position at p0
        let h10 = s3 - 2.0 * s2 + s; // Tangent at p0
        let h01 = -2.0 * s3 + 3.0 * s2; // Position at p1
        let h11 = s3 - s2; // Tangent at p1

        // Interpolated value
        h00 * p0.value + h10 * dt * p0.slope + h01 * p1.value + h11 * dt * p1.slope
    }

    /// Clear the buffer
    fn clear(&mut self) {
        self.data.clear();
        self.prev_value = 0.0;
        self.prev_time = -1e30;
    }
}

//=============================================================================
// Lossless Transmission Line
//=============================================================================

/// Lossless transmission line
#[derive(Debug, Clone)]
pub struct TransmissionLine {
    /// Instance name
    pub name: String,

    // Port 1 nodes
    pub node1_pos: NodeId,
    pub node1_neg: NodeId,

    // Port 2 nodes
    pub node2_pos: NodeId,
    pub node2_neg: NodeId,

    // Parameters
    /// Characteristic impedance (Ω)
    pub z0: Value,
    /// Propagation delay (s)
    pub td: Value,
    /// Frequency for loss calculation (optional)
    pub freq: Option<Value>,
    /// Normalized length (optional)
    pub nl: Option<Value>,
    /// One-way attenuation factor (0 < a <= 1)
    attenuation: Value,

    // Internal state
    /// Branch indices for current variables
    branch1: Option<NodeId>,
    branch2: Option<NodeId>,

    // History buffers for delayed values
    /// V1 + Z0*I1 history
    history_forward: DelayBuffer,
    /// V2 + Z0*I2 history  
    history_backward: DelayBuffer,

    /// Current simulation time
    current_time: Value,
}

impl TransmissionLine {
    /// Create a new lossless transmission line
    pub fn new(
        name: String,
        node1_pos: NodeId,
        node1_neg: NodeId,
        node2_pos: NodeId,
        node2_neg: NodeId,
        z0: Value,
        td: Value,
    ) -> Self {
        Self {
            name,
            node1_pos,
            node1_neg,
            node2_pos,
            node2_neg,
            z0,
            td,
            freq: None,
            nl: None,
            attenuation: 1.0,
            branch1: None,
            branch2: None,
            history_forward: DelayBuffer::new(td),
            history_backward: DelayBuffer::new(td),
            current_time: 0.0,
        }
    }

    /// Create from frequency and normalized length
    pub fn from_frequency(
        name: String,
        node1_pos: NodeId,
        node1_neg: NodeId,
        node2_pos: NodeId,
        node2_neg: NodeId,
        z0: Value,
        freq: Value,
        nl: Value,
    ) -> Self {
        // TD = NL / freq (number of wavelengths at frequency)
        let td = nl / freq;

        let mut tl = Self::new(name, node1_pos, node1_neg, node2_pos, node2_neg, z0, td);
        tl.freq = Some(freq);
        tl.nl = Some(nl);
        tl
    }

    /// Set branch indices for MNA
    pub fn set_branches(&mut self, branch1: NodeId, branch2: NodeId) {
        self.branch1 = Some(branch1);
        self.branch2 = Some(branch2);
    }

    /// Get characteristic impedance
    #[inline]
    pub fn impedance(&self) -> Value {
        self.z0
    }

    /// Get propagation delay
    #[inline]
    pub fn delay(&self) -> Value {
        self.td
    }

    /// Set one-way attenuation factor.
    ///
    /// Values are clamped to the physically meaningful range `(0, 1]`.
    pub fn set_attenuation(&mut self, attenuation: Value) {
        self.attenuation = attenuation.clamp(1e-6, 1.0);
    }

    /// Get one-way attenuation factor.
    #[inline]
    pub fn attenuation(&self) -> Value {
        self.attenuation
    }

    /// Get propagation velocity (if freq and nl are set)
    pub fn velocity(&self) -> Option<Value> {
        match (self.freq, self.nl) {
            (Some(f), Some(nl)) => {
                // v = wavelength * freq = (length/nl) * freq
                // But we don't have physical length, just normalized
                Some(f / nl * self.td)
            }
            _ => None,
        }
    }

    /// Update history buffers with current state
    pub fn update_history(&mut self, time: Value, v1: Value, i1: Value, v2: Value, i2: Value) {
        self.current_time = time;

        // Forward wave: V1 + Z0*I1 propagates to port 2
        self.history_forward.push(time, v1 + self.z0 * i1);

        // Backward wave: V2 + Z0*I2 propagates to port 1
        self.history_backward.push(time, v2 + self.z0 * i2);
    }

    /// Get delayed forward wave (arrives at port 2)
    pub fn delayed_forward(&self) -> Value {
        self.delayed_forward_at(self.current_time)
    }

    /// Get delayed backward wave (arrives at port 1)
    pub fn delayed_backward(&self) -> Value {
        self.delayed_backward_at(self.current_time)
    }

    /// Get delayed forward wave at an explicit simulation time.
    pub fn delayed_forward_at(&self, time: Value) -> Value {
        self.history_forward.get_delayed(time, self.td) * self.attenuation
    }

    /// Get delayed backward wave at an explicit simulation time.
    pub fn delayed_backward_at(&self, time: Value) -> Value {
        self.history_backward.get_delayed(time, self.td) * self.attenuation
    }

    /// Reset for new simulation
    pub fn reset(&mut self) {
        self.history_forward.clear();
        self.history_backward.clear();
        self.current_time = 0.0;
    }

    /// Get equivalent conductance (G = 1/Z0)
    #[inline]
    pub fn conductance(&self) -> Value {
        1.0 / self.z0
    }
}

//=============================================================================
// Lossy Transmission Line (Simplified)
//=============================================================================

/// Lossy transmission line with series resistance and shunt conductance
#[derive(Debug, Clone)]
pub struct LossyTransmissionLine {
    /// Base lossless line
    pub base: TransmissionLine,

    // Loss parameters (per unit length, normalized)
    /// DC resistance (Ω)
    pub r: Value,
    /// Shunt conductance (S)
    pub g: Value,
    /// Skin effect resistance (Ω/√Hz)
    pub rs: Value,

    /// Attenuation factor
    attenuation: Value,
}

impl LossyTransmissionLine {
    /// Create a new lossy transmission line
    pub fn new(
        name: String,
        node1_pos: NodeId,
        node1_neg: NodeId,
        node2_pos: NodeId,
        node2_neg: NodeId,
        z0: Value,
        td: Value,
        r: Value,
        g: Value,
    ) -> Self {
        let base = TransmissionLine::new(name, node1_pos, node1_neg, node2_pos, node2_neg, z0, td);

        // Calculate attenuation: exp(-(R/2Z0 + G*Z0/2) * length)
        // For normalized line, use TD as proxy for length
        let alpha = r / (2.0 * z0) + g * z0 / 2.0;
        let attenuation = (-alpha * td / 1e-9).exp().max(0.001).min(1.0);

        Self {
            base,
            r,
            g,
            rs: 0.0,
            attenuation,
        }
    }

    /// Get attenuation factor (0-1)
    pub fn attenuation(&self) -> Value {
        self.attenuation
    }

    /// Get delayed and attenuated forward wave
    pub fn delayed_forward(&self) -> Value {
        self.base.delayed_forward() * self.attenuation
    }

    /// Get delayed and attenuated backward wave
    pub fn delayed_backward(&self) -> Value {
        self.base.delayed_backward() * self.attenuation
    }
}

//=============================================================================
// Stub Helpers
//=============================================================================

/// Create an open-circuited stub
#[allow(dead_code)]
pub fn open_stub(
    name: String,
    node_pos: NodeId,
    node_neg: NodeId,
    z0: Value,
    td: Value,
) -> TransmissionLine {
    // Open stub has infinite impedance at the far end
    // Use the same node for both ends of port 2 effectively makes it open
    TransmissionLine::new(name, node_pos, node_neg, 0, 0, z0, td)
}

/// Create a short-circuited stub  
#[allow(dead_code)]
pub fn short_stub(
    name: String,
    node_pos: NodeId,
    node_neg: NodeId,
    z0: Value,
    td: Value,
) -> TransmissionLine {
    // For a shorted stub, we'd need to add a short at the far end
    // This is a simplified version - in practice, add a very low resistance
    TransmissionLine::new(name, node_pos, node_neg, 0, 0, z0, td)
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tline_creation() {
        let tl = TransmissionLine::new(
            "T1".to_string(),
            1,
            0,
            2,
            0,
            50.0, // 50Ω
            1e-9, // 1ns delay
        );

        assert_eq!(tl.z0, 50.0);
        assert_eq!(tl.td, 1e-9);
        assert_eq!(tl.conductance(), 0.02);
    }

    #[test]
    fn test_delay_buffer() {
        let mut buf = DelayBuffer::new(1e-9);

        // Add samples
        buf.push(0.0, 0.0);
        buf.push(0.5e-9, 0.5);
        buf.push(1.0e-9, 1.0);
        buf.push(1.5e-9, 1.5);
        buf.push(2.0e-9, 2.0);

        // Get delayed value (1ns delay from current time 2ns -> target 1ns)
        let v = buf.get_delayed(2.0e-9, 1.0e-9);
        assert!((v - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_delay_interpolation() {
        let mut buf = DelayBuffer::new(1e-9);

        // Need more points for cubic Hermite to work properly
        // Cubic interpolation uses slopes, so we need consistent data
        buf.push(0.0, 0.0);
        buf.push(0.25e-9, 0.25);
        buf.push(0.5e-9, 0.5);
        buf.push(0.75e-9, 0.75);
        buf.push(1.0e-9, 1.0);

        // Get value at 0.5ns - with linear ramp, cubic should match closely
        let v = buf.get_delayed(1.0e-9, 0.5e-9);
        // With linear data, cubic Hermite interpolation should give close to 0.5
        assert!((v - 0.5).abs() < 0.1, "Expected ~0.5, got {}", v);
    }

    #[test]
    fn test_history_update() {
        let mut tl = TransmissionLine::new("T1".to_string(), 1, 0, 2, 0, 50.0, 1e-9);

        // V + Z0*I at port 1 = 1 + 50*0.01 = 1.5
        tl.update_history(0.0, 1.0, 0.01, 0.0, 0.0);
        tl.update_history(0.5e-9, 1.0, 0.01, 0.0, 0.0);
        tl.update_history(1.0e-9, 1.0, 0.01, 0.0, 0.0);
        tl.update_history(1.5e-9, 1.0, 0.01, 0.0, 0.0);

        // After 1ns delay, the forward wave should arrive at port 2
        let delayed = tl.delayed_forward();
        assert!((delayed - 1.5).abs() < 0.1);
    }

    #[test]
    fn test_lossy_tline() {
        let tl = LossyTransmissionLine::new(
            "T1".to_string(),
            1,
            0,
            2,
            0,
            50.0,
            1e-9,
            1.0,   // 1Ω series resistance
            0.001, // 1mS shunt conductance
        );

        // Should have some attenuation
        assert!(tl.attenuation() < 1.0);
        assert!(tl.attenuation() > 0.0);
    }

    #[test]
    fn test_from_frequency() {
        let tl = TransmissionLine::from_frequency(
            "T1".to_string(),
            1,
            0,
            2,
            0,
            50.0,
            1e9,  // 1GHz
            0.25, // Quarter wavelength
        );

        // TD = NL/freq = 0.25 / 1e9 = 0.25ns
        assert!((tl.td - 0.25e-9).abs() < 1e-12);
    }

    #[test]
    fn test_tline_attenuation_affects_delayed_wave() {
        let mut tl = TransmissionLine::new("T1".to_string(), 1, 0, 2, 0, 50.0, 1e-9);
        tl.set_attenuation(0.5);

        tl.update_history(0.0, 1.0, 0.0, 0.0, 0.0);
        tl.update_history(0.5e-9, 1.0, 0.0, 0.0, 0.0);
        tl.update_history(1.0e-9, 1.0, 0.0, 0.0, 0.0);
        tl.update_history(1.5e-9, 1.0, 0.0, 0.0, 0.0);

        let delayed = tl.delayed_forward();
        assert!(
            (delayed - 0.5).abs() < 0.1,
            "expected attenuated delayed wave near 0.5, got {}",
            delayed
        );
    }

    #[test]
    fn test_tline_delayed_at_uses_explicit_time() {
        let mut tl = TransmissionLine::new("T1".to_string(), 1, 0, 2, 0, 50.0, 1e-9);
        tl.update_history(0.0, 0.0, 0.0, 0.0, 0.0);
        tl.update_history(1.0e-9, 1.0, 0.0, 0.0, 0.0);
        tl.update_history(2.0e-9, 2.0, 0.0, 0.0, 0.0);

        let delayed = tl.delayed_forward_at(2.0e-9);
        assert!(
            (delayed - 1.0).abs() < 0.15,
            "expected delayed value near 1.0 at explicit time, got {}",
            delayed
        );
    }

    #[test]
    fn test_tline_reset() {
        let mut tl = TransmissionLine::new("T1".to_string(), 1, 0, 2, 0, 50.0, 1e-9);

        tl.update_history(0.0, 1.0, 0.01, 0.0, 0.0);
        tl.reset();

        // After reset, delayed values should be 0
        let delayed = tl.delayed_forward();
        assert_eq!(delayed, 0.0);
    }
}
