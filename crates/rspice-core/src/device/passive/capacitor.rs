//! Capacitor device model

use crate::{circuit::NodeId, Value};
use crate::device::traits::{DynamicDevice, MatrixStamper};

/// Capacitor with companion model for transient analysis
#[derive(Debug, Clone)]
pub struct Capacitor {
    pub name: String,
    pub node_pos: NodeId,
    pub node_neg: NodeId,
    pub capacitance: Value,
    /// Voltage across capacitor at previous time step
    voltage_prev: Value,
    /// Companion model equivalent current source
    ieq: Value,
}

impl Capacitor {
    pub fn new(name: String, node_pos: NodeId, node_neg: NodeId, capacitance: Value) -> Self {
        Self {
            name,
            node_pos,
            node_neg,
            capacitance,
            voltage_prev: 0.0,
            ieq: 0.0,
        }
    }

    /// Set initial voltage
    pub fn set_initial_voltage(&mut self, voltage: Value) {
        self.voltage_prev = voltage;
    }

    /// Get equivalent conductance for trapezoidal integration
    pub fn geq(&self, dt: Value) -> Value {
        2.0 * self.capacitance / dt
    }
}

impl DynamicDevice for Capacitor {
    fn stamp_transient(
        &self,
        _voltages: &[Value],
        dt: Value,
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        // Trapezoidal companion model:
        // i = geq * v + ieq
        // where geq = 2C/dt and ieq = 2C/dt * v_prev + i_prev
        
        let geq = self.geq(dt);
        
        // Stamp conductance
        matrix.stamp(self.node_pos, self.node_pos, geq);
        matrix.stamp(self.node_pos, self.node_neg, -geq);
        matrix.stamp(self.node_neg, self.node_pos, -geq);
        matrix.stamp(self.node_neg, self.node_neg, geq);
        
        // Stamp current source (ieq)
        matrix.stamp_rhs(self.node_pos, -self.ieq);
        matrix.stamp_rhs(self.node_neg, self.ieq);
    }

    fn step(&mut self, voltages: &[Value], dt: Value) {
        let v_pos = if self.node_pos == 0 { 0.0 } else { voltages[self.node_pos - 1] };
        let v_neg = if self.node_neg == 0 { 0.0 } else { voltages[self.node_neg - 1] };
        let v = v_pos - v_neg;
        
        // Update companion model current source for next step
        // For trapezoidal: ieq = 2C/dt * v + i_prev = 2C/dt * v + 2C/dt * v_prev + ieq_prev
        let geq = self.geq(dt);
        self.ieq = geq * v + geq * self.voltage_prev + self.ieq;
        
        self.voltage_prev = v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capacitor_geq() {
        let c = Capacitor::new("C1".to_string(), 1, 0, 1e-6);
        let geq = c.geq(1e-6); // dt = 1us
        assert!((geq - 2.0).abs() < 1e-10); // 2 * 1e-6 / 1e-6 = 2
    }
}
