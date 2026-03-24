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
}

impl DynamicDevice for Inductor {
    fn stamp_transient(
        &self,
        _voltages: &[Value],
        dt: Value,
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let branch = self
            .branch_index
            .expect("Branch index must be set for inductor");
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

    #[test]
    fn test_inductor_creation() {
        let ind = Inductor::new("L1".to_string(), 1, 0, 1e-3);
        assert_eq!(ind.name, "L1");
        assert_eq!(ind.node_pos, 1);
        assert_eq!(ind.node_neg, 0);
        assert_eq!(ind.inductance, 1e-3);
    }

    #[test]
    fn test_inductor_req() {
        let ind = Inductor::new("L1".to_string(), 1, 0, 1e-3);

        // Req = 2L/dt
        let dt = 1e-6;
        let req = ind.req(dt);
        assert!((req - 2000.0).abs() < 0.01); // 2 * 1e-3 / 1e-6 = 2000
    }

    #[test]
    fn test_initial_current() {
        let mut ind = Inductor::new("L1".to_string(), 1, 0, 1e-3);
        ind.set_initial_current(0.1);
        // Internal state is private, but we can verify by stepping
        assert!(ind.current_prev == 0.1);
    }

    #[test]
    fn test_branch_index() {
        let mut ind = Inductor::new("L1".to_string(), 1, 0, 1e-3);
        assert!(ind.branch_index.is_none());

        ind.set_branch_index(5);
        assert_eq!(ind.branch_index, Some(5));
    }

    #[test]
    fn test_inductor_step() {
        let mut ind = Inductor::new("L1".to_string(), 1, 0, 1e-3);
        ind.set_branch_index(2);

        // Apply 1V for 1µs
        let dt = 1e-6;
        let voltages = vec![1.0, 0.0]; // V(1) = 1V, V(2) = 0V (branch)

        ind.step(&voltages, dt);

        // di = (dt / 2L) * (v + v_prev) = (1e-6 / 2e-3) * (1 + 0) = 0.5e-3 = 0.5mA
        // But we started at 0, so current should increase
        assert!(ind.current_prev > 0.0);
    }
}
