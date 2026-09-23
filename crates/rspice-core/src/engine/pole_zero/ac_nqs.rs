//! Explicit AC-only BSIM3/BSIM4 relaxation states for pole-zero extraction.

use super::*;
use crate::device::MatrixStamper;

struct DescriptorStamper<'a>(&'a mut Matrix);

impl MatrixStamper for DescriptorStamper<'_> {
    fn stamp(&mut self, row: usize, col: usize, value: Value) {
        if row != 0 && col != 0 {
            self.0.add(row - 1, col - 1, value);
        }
    }

    fn stamp_rhs(&mut self, _: usize, _: Value) {}
}

impl Engine {
    pub(super) fn pz_ac_nqs_state_count(circuit: &CircuitData) -> usize {
        let bsim3 = circuit
            .bsim3v3
            .devices
            .iter()
            .filter(|device| device.uses_ac_nqs())
            .count();
        let bsim4 = circuit
            .bsim4v8
            .devices
            .iter()
            .filter(|device| device.uses_ac_nqs())
            .count();
        bsim3.saturating_add(bsim4).saturating_mul(3)
    }

    pub(super) fn stamp_ac_nqs_pz_descriptor_states(
        circuit: &CircuitData,
        op_voltages: &[Value],
        g: &mut Matrix,
        c: &mut Matrix,
    ) -> Result<(), SimulationError> {
        let count = Self::pz_ac_nqs_state_count(circuit);
        if count == 0 {
            return Ok(());
        }
        let mut first = Self::descriptor_expand_square(g, c, count) + 1;
        for device in circuit
            .bsim3v3
            .devices
            .iter()
            .filter(|device| device.uses_ac_nqs())
        {
            // The base PZ assembly omits these devices entirely. Stamp the
            // complete native response, including its physical terminal G/C
            // and the anchor of any overridden transient deficit coordinate.
            device
                .stamp_ac_nqs_response(
                    op_voltages,
                    [first, first + 1, first + 2],
                    &mut DescriptorStamper(g),
                    &mut DescriptorStamper(c),
                )
                .map_err(SimulationError::Circuit)?;
            first += 3;
        }
        for device in circuit
            .bsim4v8
            .devices
            .iter()
            .filter(|device| device.uses_ac_nqs())
        {
            device
                .stamp_ac_nqs_response(
                    op_voltages,
                    [first, first + 1, first + 2],
                    &mut DescriptorStamper(g),
                    &mut DescriptorStamper(c),
                )
                .map_err(SimulationError::Circuit)?;
            first += 3;
        }
        Ok(())
    }
}
