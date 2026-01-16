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

use crate::{circuit::NodeId, Value};
use std::collections::VecDeque;

//=============================================================================
// History Buffer for Delay
//=============================================================================

/// Circular buffer for storing time history
#[derive(Debug, Clone)]
struct DelayBuffer {
    /// Time-value pairs: (time, value)
    data: VecDeque<(Value, Value)>,
    /// Maximum storage time
    max_time: Value,
}

impl DelayBuffer {
    fn new(max_time: Value) -> Self {
        Self {
            data: VecDeque::new(),
            max_time,
        }
    }

    /// Add a new sample
    fn push(&mut self, time: Value, value: Value) {
        self.data.push_back((time, value));
        
        // Remove old samples
        while let Some(&(t, _)) = self.data.front() {
            if time - t > self.max_time * 1.5 {
                self.data.pop_front();
            } else {
                break;
            }
        }
    }

    /// Get interpolated value at time (time - delay)
    fn get_delayed(&self, current_time: Value, delay: Value) -> Value {
        let target_time = current_time - delay;
        
        if self.data.is_empty() {
            return 0.0;
        }
        
        // Find surrounding samples for interpolation
        let mut prev: Option<(Value, Value)> = None;
        
        for &(t, v) in self.data.iter() {
            if t >= target_time {
                if let Some((t0, v0)) = prev {
                    // Linear interpolation
                    if (t - t0).abs() > 1e-18 {
                        let alpha = (target_time - t0) / (t - t0);
                        return v0 + alpha * (v - v0);
                    }
                }
                return v;
            }
            prev = Some((t, v));
        }
        
        // Target time is beyond buffer, return last value
        self.data.back().map(|&(_, v)| v).unwrap_or(0.0)
    }

    /// Clear the buffer
    fn clear(&mut self) {
        self.data.clear();
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
        self.history_forward.get_delayed(self.current_time, self.td)
    }

    /// Get delayed backward wave (arrives at port 1)
    pub fn delayed_backward(&self) -> Value {
        self.history_backward.get_delayed(self.current_time, self.td)
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
pub fn open_stub(name: String, node_pos: NodeId, node_neg: NodeId, z0: Value, td: Value) -> TransmissionLine {
    // Open stub has infinite impedance at the far end
    // Use the same node for both ends of port 2 effectively makes it open
    TransmissionLine::new(name, node_pos, node_neg, 0, 0, z0, td)
}

/// Create a short-circuited stub  
pub fn short_stub(name: String, node_pos: NodeId, node_neg: NodeId, z0: Value, td: Value) -> TransmissionLine {
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
            1, 0, 2, 0,
            50.0,   // 50Ω
            1e-9,   // 1ns delay
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
        
        buf.push(0.0, 0.0);
        buf.push(1.0e-9, 1.0);
        
        // Get value at 0.5ns
        let v = buf.get_delayed(1.0e-9, 0.5e-9);
        assert!((v - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_history_update() {
        let mut tl = TransmissionLine::new(
            "T1".to_string(),
            1, 0, 2, 0,
            50.0,
            1e-9,
        );
        
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
            1, 0, 2, 0,
            50.0,
            1e-9,
            1.0,    // 1Ω series resistance
            0.001,  // 1mS shunt conductance
        );
        
        // Should have some attenuation
        assert!(tl.attenuation() < 1.0);
        assert!(tl.attenuation() > 0.0);
    }

    #[test]
    fn test_from_frequency() {
        let tl = TransmissionLine::from_frequency(
            "T1".to_string(),
            1, 0, 2, 0,
            50.0,
            1e9,    // 1GHz
            0.25,   // Quarter wavelength
        );
        
        // TD = NL/freq = 0.25 / 1e9 = 0.25ns
        assert!((tl.td - 0.25e-9).abs() < 1e-12);
    }

    #[test]
    fn test_tline_reset() {
        let mut tl = TransmissionLine::new(
            "T1".to_string(),
            1, 0, 2, 0,
            50.0,
            1e-9,
        );
        
        tl.update_history(0.0, 1.0, 0.01, 0.0, 0.0);
        tl.reset();
        
        // After reset, delayed values should be 0
        let delayed = tl.delayed_forward();
        assert_eq!(delayed, 0.0);
    }
}
