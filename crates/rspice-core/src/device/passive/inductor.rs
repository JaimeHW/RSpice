//! Inductor device model

use crate::device::traits::{DynamicDevice, MatrixStamper};
use crate::{Value, circuit::NodeId};

/// Inductor with companion model for transient analysis
#[derive(Debug, Clone)]
pub struct Inductor {
    pub name: String,
    pub node_pos: NodeId,
    pub node_neg: NodeId,
    pub inductance: Value,
    /// Current branch variable index in MNA
    pub branch_index: Option<NodeId>,
    /// Current through inductor at previous time step
    current_prev: Value,
    /// Previous voltage across inductor
    voltage_prev: Value,
}

impl Inductor {
    pub fn new(name: String, node_pos: NodeId, node_neg: NodeId, inductance: Value) -> Self {
        Self {
            name,
            node_pos,
            node_neg,
            inductance,
            branch_index: None,
            current_prev: 0.0,
            voltage_prev: 0.0,
        }
    }

    /// Set initial current
    pub fn set_initial_current(&mut self, current: Value) {
        self.current_prev = current;
    }

    /// Set the branch index for MNA
    pub fn set_branch_index(&mut self, index: NodeId) {
        self.branch_index = Some(index);
    }

    /// Get equivalent resistance for trapezoidal integration
    pub fn req(&self, dt: Value) -> Value {
        2.0 * self.inductance / dt
    }

    /// Stamp this inductor after verifying its MNA branch has been assigned.
    ///
    /// Public direct-device callers should prefer this checked API. The
    /// legacy [`DynamicDevice`] implementation logs and returns on this error
    /// because that trait cannot report failures.
    pub fn try_stamp_transient(
        &self,
        _voltages: &[Value],
        dt: Value,
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) -> Result<(), String> {
        let Some(branch) = self.branch_index else {
            return Err(format!(
                "Inductor '{}' cannot be stamped before its MNA branch index is assigned",
                self.name
            ));
        };
        let req = self.req(dt);

        // Trapezoidal companion model for inductor
        // v = req * i + veq
        // where req = 2L/dt

        // MNA stamp for inductor (treated as voltage source with series resistance)
        // Row for branch current equation: v+ - v- - req*i = veq
        matrix.stamp(branch, self.node_pos, 1.0);
        matrix.stamp(branch, self.node_neg, -1.0);
        matrix.stamp(branch, branch, -req);

        // Node equations: current contribution
        matrix.stamp(self.node_pos, branch, 1.0);
        matrix.stamp(self.node_neg, branch, -1.0);

        // Equivalent voltage source
        let veq = req * self.current_prev + self.voltage_prev;
        matrix.stamp_rhs(branch, veq);
        Ok(())
    }
}

impl DynamicDevice for Inductor {
    fn stamp_transient(
        &self,
        _voltages: &[Value],
        dt: Value,
        matrix: &mut impl MatrixStamper,
        rhs: &mut [Value],
    ) {
        if let Err(message) = self.try_stamp_transient(_voltages, dt, matrix, rhs) {
            log::error!("{message}");
        }
    }

    fn step(&mut self, voltages: &[Value], dt: Value) {
        let v_pos = if self.node_pos == 0 {
            0.0
        } else {
            voltages[self.node_pos - 1]
        };
        let v_neg = if self.node_neg == 0 {
            0.0
        } else {
            voltages[self.node_neg - 1]
        };
        let v = v_pos - v_neg;

        // Update current for next step using trapezoidal rule
        // i = i_prev + (dt / 2L) * (v + v_prev)
        self.current_prev += (dt / (2.0 * self.inductance)) * (v + self.voltage_prev);
        self.voltage_prev = v;
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
    fn inductor_without_branch_index_reports_checked_error() {
        let inductor = Inductor::new("L1".to_string(), 1, 0, 1e-3);
        let mut matrix = RecordingStamper::default();
        let mut rhs = [];

        let message = inductor
            .try_stamp_transient(&[], 1e-6, &mut matrix, &mut rhs)
            .expect_err("missing branch index must be a checked error");

        assert!(
            message.contains("branch index is assigned"),
            "unexpected error: {message}"
        );
        assert_eq!(matrix.stamps, 0);
        assert_eq!(matrix.rhs_stamps, 0);
    }

    #[test]
    fn inductor_legacy_stamp_without_branch_index_does_not_panic() {
        let inductor = Inductor::new("L1".to_string(), 1, 0, 1e-3);
        let mut matrix = RecordingStamper::default();
        let mut rhs = [];

        let result = catch_unwind(AssertUnwindSafe(|| {
            inductor.stamp_transient(&[], 1e-6, &mut matrix, &mut rhs);
        }));

        assert!(result.is_ok(), "legacy trait wrapper should not panic");
    }
}
