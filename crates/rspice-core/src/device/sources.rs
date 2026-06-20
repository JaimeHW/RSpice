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

    /// Stamp this source after verifying its MNA branch has been assigned.
    ///
    /// Public direct-device callers should prefer this checked API. The
    /// legacy [`LinearDevice`] implementation logs and returns on this error
    /// because that trait cannot report failures.
    pub fn try_stamp_linear(
        &self,
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) -> Result<(), String> {
        let Some(branch) = self.branch_index else {
            return Err(format!(
                "Voltage source '{}' cannot be stamped before its MNA branch index is assigned",
                self.name
            ));
        };

        // MNA stamp for ideal voltage source
        // Adds extra equation: V(n+) - V(n-) = Vs
        // Adds branch current variable

        matrix.stamp(branch, self.node_pos, 1.0);
        matrix.stamp(branch, self.node_neg, -1.0);
        matrix.stamp(self.node_pos, branch, 1.0);
        matrix.stamp(self.node_neg, branch, -1.0);

        matrix.stamp_rhs(branch, self.dc_value);
        Ok(())
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
    fn stamp_linear(&self, matrix: &mut impl MatrixStamper, rhs: &mut [Value]) {
        if let Err(message) = self.try_stamp_linear(matrix, rhs) {
            log::error!("{message}");
        }
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
    use std::panic::{AssertUnwindSafe, catch_unwind};

    #[derive(Default)]
    struct RecordingStamper {
        stamps: usize,
        rhs_stamps: usize,
    }

    impl MatrixStamper for RecordingStamper {
        fn stamp(&mut self, _row: NodeId, _col: NodeId, _value: Value) {
            self.stamps += 1;
        }

        fn stamp_rhs(&mut self, _index: NodeId, _value: Value) {
            self.rhs_stamps += 1;
        }
    }

    #[test]
    fn voltage_source_without_branch_index_reports_checked_error() {
        let source = VoltageSource::new_dc("V1".to_string(), 1, 0, 5.0);
        let mut matrix = RecordingStamper::default();
        let mut rhs = [];

        let message = source
            .try_stamp_linear(&mut matrix, &mut rhs)
            .expect_err("missing branch index must be a checked error");

        assert!(
            message.contains("branch index is assigned"),
            "unexpected error: {message}"
        );
        assert_eq!(matrix.stamps, 0);
        assert_eq!(matrix.rhs_stamps, 0);
    }

    #[test]
    fn voltage_source_legacy_stamp_without_branch_index_does_not_panic() {
        let source = VoltageSource::new_dc("V1".to_string(), 1, 0, 5.0);
        let mut matrix = RecordingStamper::default();
        let mut rhs = [];

        let result = catch_unwind(AssertUnwindSafe(|| {
            source.stamp_linear(&mut matrix, &mut rhs);
        }));

        assert!(result.is_ok(), "legacy trait wrapper should not panic");
    }
}
