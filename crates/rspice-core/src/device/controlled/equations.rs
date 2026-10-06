//! Shared linear source coefficients and finite current observation.
use super::*;
use rspice_veriloga_runtime::arithmetic::{ArithmeticError, sum_products};

impl Vccs {
    /// One-based MNA nodes; zero terminals are omitted by each matrix adapter.
    #[inline]
    pub(crate) fn conductance_entries(&self, index: usize) -> [(NodeId, NodeId, Value); 4] {
        let (p, n) = (self.node_pos[index], self.node_neg[index]);
        let (cp, cn) = (self.ctrl_pos[index], self.ctrl_neg[index]);
        let gm = self.transconductances[index];
        [(p, cp, gm), (p, cn, -gm), (n, cp, -gm), (n, cn, gm)]
    }

    pub(crate) fn current_at_control_voltage(
        &self,
        index: usize,
        positive: Value,
        negative: Value,
    ) -> Result<Value, ArithmeticError> {
        let gm = self.transconductances[index];
        sum_products([(positive, gm), (negative, -gm)].into_iter())
    }

    pub(crate) fn stamp_physical_current(
        &self,
        index: usize,
        positive: Value,
        negative: Value,
        stamp: &mut impl crate::device::MatrixStamper,
    ) -> Result<(), ArithmeticError> {
        let current = self.current_at_control_voltage(index, positive, negative)?;
        stamp.stamp_rhs(self.node_pos[index], -current);
        stamp.stamp_rhs(self.node_neg[index], current);
        for (row, column, value) in self.conductance_entries(index) {
            stamp.stamp(row, column, value);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vccs_observation_preserves_a_finite_current_across_large_common_mode() {
        let mut sources = Vccs::new();
        sources.add(
            "g".into(),
            VoltageControlledNodes {
                node_pos: 1,
                node_neg: 0,
                ctrl_pos: 2,
                ctrl_neg: 3,
            },
            1e-308,
        );
        let current = sources
            .current_at_control_voltage(0, 1e308, -1e308)
            .unwrap();
        assert!((current - 2.0).abs() <= 2.0 * f64::EPSILON);
        assert_eq!(sources.current_at_control_voltage(0, 1e308, 1e308), Ok(0.0));
        assert!(
            sources
                .current_at_control_voltage(0, Value::INFINITY, 0.0)
                .is_err()
        );
    }
}
