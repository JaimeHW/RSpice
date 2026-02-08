//! Voltage and current source models

use super::traits::{LinearDevice, MatrixStamper};
use crate::{Value, circuit::NodeId};

/// Independent voltage source
#[derive(Debug, Clone)]
pub struct VoltageSource {
    pub name: String,
    pub node_pos: NodeId,
    pub node_neg: NodeId,
    /// Branch index in MNA matrix
    pub branch_index: Option<NodeId>,
    /// DC voltage value
    pub dc_value: Value,
    /// AC magnitude
    pub ac_magnitude: Value,
    /// AC phase (radians)
    pub ac_phase: Value,
    /// Time-dependent function (if any)
    pub transient_fn: Option<TransientSource>,
}

/// Time-dependent source functions
#[derive(Debug, Clone)]
pub enum TransientSource {
    Pulse {
        v1: Value,
        v2: Value,
        delay: Value,
        rise: Value,
        fall: Value,
        width: Value,
        period: Value,
    },
    Sin {
        offset: Value,
        amplitude: Value,
        frequency: Value,
        delay: Value,
        damping: Value,
        phase: Value,
    },
    Pwl {
        points: Vec<(Value, Value)>,
    },
    Exp {
        v1: Value,
        v2: Value,
        td1: Value,
        tau1: Value,
        td2: Value,
        tau2: Value,
    },
}

impl VoltageSource {
    pub fn new_dc(name: String, node_pos: NodeId, node_neg: NodeId, dc_value: Value) -> Self {
        Self {
            name,
            node_pos,
            node_neg,
            branch_index: None,
            dc_value,
            ac_magnitude: 0.0,
            ac_phase: 0.0,
            transient_fn: None,
        }
    }

    pub fn set_branch_index(&mut self, index: NodeId) {
        self.branch_index = Some(index);
    }

    /// Get voltage at given time for transient analysis
    pub fn voltage_at(&self, time: Value) -> Value {
        match &self.transient_fn {
            None => self.dc_value,
            Some(TransientSource::Sin {
                offset,
                amplitude,
                frequency,
                delay,
                damping,
                phase,
            }) => {
                if time < *delay {
                    *offset
                } else {
                    let t = time - delay;
                    offset
                        + amplitude
                            * (-damping * t).exp()
                            * (2.0 * std::f64::consts::PI * frequency * t + phase).sin()
                }
            }
            Some(TransientSource::Pulse {
                v1,
                v2,
                delay,
                rise,
                fall,
                width,
                period,
            }) => {
                if time < *delay {
                    return *v1;
                }
                let t = (time - delay) % period;
                if t < *rise {
                    v1 + (v2 - v1) * t / rise
                } else if t < rise + width {
                    *v2
                } else if t < rise + width + fall {
                    v2 + (v1 - v2) * (t - rise - width) / fall
                } else {
                    *v1
                }
            }
            Some(TransientSource::Pwl { points }) => {
                if points.is_empty() {
                    return self.dc_value;
                }
                if time <= points[0].0 {
                    return points[0].1;
                }
                if time >= points[points.len() - 1].0 {
                    return points[points.len() - 1].1;
                }
                // Linear interpolation
                for i in 0..points.len() - 1 {
                    if time >= points[i].0 && time < points[i + 1].0 {
                        let (t1, v1) = points[i];
                        let (t2, v2) = points[i + 1];
                        return v1 + (v2 - v1) * (time - t1) / (t2 - t1);
                    }
                }
                self.dc_value
            }
            Some(TransientSource::Exp {
                v1,
                v2,
                td1,
                tau1,
                td2,
                tau2,
            }) => {
                if time < *td1 {
                    *v1
                } else if time < *td2 {
                    v1 + (v2 - v1) * (1.0 - (-(time - td1) / tau1).exp())
                } else {
                    v1 + (v2 - v1) * (1.0 - (-(time - td1) / tau1).exp())
                        - (v2 - v1) * (1.0 - (-(time - td2) / tau2).exp())
                }
            }
        }
    }
}

impl LinearDevice for VoltageSource {
    fn stamp_linear(&self, matrix: &mut impl MatrixStamper, _rhs: &mut [Value]) {
        let branch = self
            .branch_index
            .expect("Branch index must be set for voltage source");

        // MNA stamp for ideal voltage source
        // Adds extra equation: V(n+) - V(n-) = Vs
        // Adds branch current variable

        matrix.stamp(branch, self.node_pos, 1.0);
        matrix.stamp(branch, self.node_neg, -1.0);
        matrix.stamp(self.node_pos, branch, 1.0);
        matrix.stamp(self.node_neg, branch, -1.0);

        matrix.stamp_rhs(branch, self.dc_value);
    }
}

/// Independent current source
#[derive(Debug, Clone)]
pub struct CurrentSource {
    pub name: String,
    pub node_pos: NodeId,
    pub node_neg: NodeId,
    /// DC current value
    pub dc_value: Value,
    /// AC magnitude
    pub ac_magnitude: Value,
    /// AC phase (radians)
    pub ac_phase: Value,
    /// Time-dependent function
    pub transient_fn: Option<TransientSource>,
}

impl CurrentSource {
    pub fn new_dc(name: String, node_pos: NodeId, node_neg: NodeId, dc_value: Value) -> Self {
        Self {
            name,
            node_pos,
            node_neg,
            dc_value,
            ac_magnitude: 0.0,
            ac_phase: 0.0,
            transient_fn: None,
        }
    }

    /// Get current at given time for transient analysis
    pub fn current_at(&self, time: Value) -> Value {
        // Similar implementation to VoltageSource::voltage_at
        match &self.transient_fn {
            None => self.dc_value,
            Some(TransientSource::Sin {
                offset,
                amplitude,
                frequency,
                delay,
                damping,
                phase,
            }) => {
                if time < *delay {
                    *offset
                } else {
                    let t = time - delay;
                    offset
                        + amplitude
                            * (-damping * t).exp()
                            * (2.0 * std::f64::consts::PI * frequency * t + phase).sin()
                }
            }
            // Other transient sources follow same pattern as voltage source
            _ => self.dc_value,
        }
    }
}

impl LinearDevice for CurrentSource {
    fn stamp_linear(&self, matrix: &mut impl MatrixStamper, _rhs: &mut [Value]) {
        // Current source stamps directly into RHS
        // Current flows from node_pos to node_neg
        matrix.stamp_rhs(self.node_pos, -self.dc_value);
        matrix.stamp_rhs(self.node_neg, self.dc_value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dc_voltage_source() {
        let vs = VoltageSource::new_dc("V1".to_string(), 1, 0, 5.0);
        assert_eq!(vs.dc_value, 5.0);
        assert_eq!(vs.voltage_at(0.0), 5.0);
        assert_eq!(vs.voltage_at(1.0), 5.0);
    }

    #[test]
    fn test_sin_source() {
        let mut vs = VoltageSource::new_dc("V1".to_string(), 1, 0, 0.0);
        vs.transient_fn = Some(TransientSource::Sin {
            offset: 0.0,
            amplitude: 1.0,
            frequency: 1000.0, // 1kHz
            delay: 0.0,
            damping: 0.0,
            phase: 0.0,
        });

        // At t=0, sin(0) = 0
        assert!(vs.voltage_at(0.0).abs() < 0.01);

        // At t=0.25ms (quarter period), sin(π/2) = 1
        let v = vs.voltage_at(0.00025);
        assert!((v - 1.0).abs() < 0.01);

        // At t=0.5ms (half period), sin(π) = 0
        let v = vs.voltage_at(0.0005);
        assert!(v.abs() < 0.01);
    }

    #[test]
    fn test_sin_with_delay() {
        let mut vs = VoltageSource::new_dc("V1".to_string(), 1, 0, 0.0);
        vs.transient_fn = Some(TransientSource::Sin {
            offset: 2.5,
            amplitude: 1.0,
            frequency: 1000.0,
            delay: 1e-3, // 1ms delay
            damping: 0.0,
            phase: 0.0,
        });

        // Before delay, should be offset
        assert_eq!(vs.voltage_at(0.0), 2.5);
        assert_eq!(vs.voltage_at(0.5e-3), 2.5);

        // After delay, should oscillate around offset
        let v = vs.voltage_at(1.25e-3); // 0.25ms after delay
        assert!((v - 3.5).abs() < 0.1); // offset + amplitude
    }

    #[test]
    fn test_pulse_source() {
        let mut vs = VoltageSource::new_dc("V1".to_string(), 1, 0, 0.0);
        vs.transient_fn = Some(TransientSource::Pulse {
            v1: 0.0,
            v2: 5.0,
            delay: 0.0,
            rise: 1e-9,    // 1ns rise
            fall: 1e-9,    // 1ns fall
            width: 10e-6,  // 10µs width
            period: 20e-6, // 20µs period (50% duty)
        });

        // At start, should be V1
        assert!(vs.voltage_at(0.0).abs() < 0.01);

        // During pulse width, should be V2
        assert!((vs.voltage_at(5e-6) - 5.0).abs() < 0.01);

        // After fall, should be V1
        assert!(vs.voltage_at(15e-6).abs() < 0.01);
    }

    #[test]
    fn test_pulse_with_delay() {
        let mut vs = VoltageSource::new_dc("V1".to_string(), 1, 0, 0.0);
        vs.transient_fn = Some(TransientSource::Pulse {
            v1: 0.0,
            v2: 3.3,
            delay: 1e-6, // 1µs delay
            rise: 10e-9,
            fall: 10e-9,
            width: 5e-6,
            period: 10e-6,
        });

        // Before delay
        assert_eq!(vs.voltage_at(0.0), 0.0);
        assert_eq!(vs.voltage_at(0.5e-6), 0.0);

        // After delay
        assert!((vs.voltage_at(3e-6) - 3.3).abs() < 0.01);
    }

    #[test]
    fn test_pwl_source() {
        let mut vs = VoltageSource::new_dc("V1".to_string(), 1, 0, 0.0);
        vs.transient_fn = Some(TransientSource::Pwl {
            points: vec![(0.0, 0.0), (1e-3, 1.0), (2e-3, 1.0), (3e-3, 0.0)],
        });

        // At defined points
        assert!(vs.voltage_at(0.0).abs() < 0.01);
        assert!((vs.voltage_at(1e-3) - 1.0).abs() < 0.01);
        assert!((vs.voltage_at(2e-3) - 1.0).abs() < 0.01);

        // Interpolated
        let v = vs.voltage_at(0.5e-3);
        assert!((v - 0.5).abs() < 0.01);

        // After last point
        assert!(vs.voltage_at(5e-3).abs() < 0.01);
    }

    #[test]
    fn test_exp_source() {
        let mut vs = VoltageSource::new_dc("V1".to_string(), 1, 0, 0.0);
        vs.transient_fn = Some(TransientSource::Exp {
            v1: 0.0,
            v2: 5.0,
            td1: 0.0,
            tau1: 1e-3, // 1ms rise time constant
            td2: 10e-3, // 10ms before decay starts
            tau2: 2e-3, // 2ms decay time constant
        });

        // At t=0, should be v1
        assert_eq!(vs.voltage_at(0.0), 0.0);

        // At ~5*tau1 = 5ms, should be close to v2
        let v = vs.voltage_at(5e-3);
        assert!((v - 5.0).abs() < 0.1);
    }

    #[test]
    fn test_current_source_dc() {
        let cs = CurrentSource::new_dc("I1".to_string(), 1, 0, 1e-3);
        assert_eq!(cs.dc_value, 1e-3);
        assert_eq!(cs.current_at(0.0), 1e-3);
    }

    #[test]
    fn test_current_source_sin() {
        let mut cs = CurrentSource::new_dc("I1".to_string(), 1, 0, 0.0);
        cs.transient_fn = Some(TransientSource::Sin {
            offset: 1e-3,
            amplitude: 0.5e-3,
            frequency: 1000.0,
            delay: 0.0,
            damping: 0.0,
            phase: 0.0,
        });

        // At t=0, should be offset (sin(0) = 0)
        assert!((cs.current_at(0.0) - 1e-3).abs() < 1e-6);
    }
}
