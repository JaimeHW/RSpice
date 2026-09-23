//! Finite response realization shared by the native BSIM3/BSIM4 AC NQS laws.

use crate::device::MatrixStamper;
use crate::{NodeId, Value};

// Constant equation scaling preserves dy/dt=(input-y)/tau at changing bias.
// Scaling by tau itself would introduce an erroneous d(tau)/dt contribution.
const TIME_UNIT: Value = 1e-9;
const CHARGE_UNIT: Value = 1e-9;

pub(crate) struct AcNqsResponse<'a> {
    pub model: &'static str,
    pub name: &'a str,
    /// Intrinsic terminals in drain, gate, source, bulk order.
    pub terminals: [NodeId; 4],
    pub multiplier: Value,
    pub mode: i32,
    /// gm, gmbs, gds before mode assembly and instance multiplication.
    pub current: [Value; 3],
    /// Intrinsic gate, bulk, drain charge rows; columns are gate, drain, source.
    pub charge: [[Value; 3]; 3],
    pub relaxation_time: Value,
}

impl AcNqsResponse<'_> {
    fn invalid(&self, reason: &str) -> String {
        format!("{} '{}': {reason}", self.model, self.name)
    }

    /// Add the relaxation delta to an already stamped quasi-static G/Q.
    /// Auxiliary states are channel current, drain charge/1n, source charge/1n.
    pub(crate) fn stamp(
        &self,
        auxiliary: [NodeId; 3],
        f: &mut impl MatrixStamper,
        q: &mut impl MatrixStamper,
    ) -> Result<(), String> {
        let tau = self.relaxation_time;
        if !tau.is_finite() || tau < 0.0 {
            return Err(self.invalid("invalid AC NQS relaxation time"));
        }
        let rate = if tau == 0.0 { 0.0 } else { TIME_UNIT / tau };
        if !rate.is_finite() || (tau > 0.0 && rate == 0.0) {
            return Err(self.invalid("AC NQS relaxation rate is not representable"));
        }
        if rate == 0.0 {
            for node in auxiliary {
                f.stamp(node, node, 1.0);
            }
            return Ok(());
        }

        let [gm, gmb, gds] = self.current;
        let (gm, gmb, forward, reverse) = if self.mode >= 0 {
            (gm, gmb, gm + gmb, 0.0)
        } else {
            (-gm, -gmb, 0.0, gm + gmb)
        };
        let channel = [gds + reverse, gm, -(gds + forward), gmb];
        let [gate, bulk, drain] = self.charge;
        let source = std::array::from_fn(|i| -(drain[i] + gate[i] + bulk[i]));
        let terminal_row = |row: [Value; 3]| [row[1], row[0], row[2], -row.iter().sum::<Value>()];
        let (drain, source) = (terminal_row(drain), terminal_row(source));
        let (drain, source) = if self.mode >= 0 {
            (drain, source)
        } else {
            let swap = |row: [Value; 4]| [row[2], row[1], row[0], row[3]];
            (swap(source), swap(drain))
        };
        let inputs = [channel, drain, source];
        let scales = [1.0, CHARGE_UNIT, CHARGE_UNIT];
        for (index, node) in auxiliary.into_iter().enumerate() {
            f.stamp(node, node, rate);
            q.stamp(node, node, TIME_UNIT);
            for (terminal, derivative) in self.terminals.into_iter().zip(inputs[index]) {
                let value = self.multiplier * derivative * rate / scales[index];
                if !value.is_finite() {
                    return Err(self.invalid("AC NQS input derivative is not representable"));
                }
                f.stamp(node, terminal, -value);
            }
        }
        let [drain_node, gate_node, source_node, _] = self.terminals;
        // Bulk charge, overlap/junction storage and other current mechanisms
        // retain the model-specific QS stamps supplied by the caller.
        for (node, sign) in [(drain_node, 1.0), (source_node, -1.0)] {
            f.stamp(node, auxiliary[0], sign);
            for (terminal, derivative) in self.terminals.into_iter().zip(channel) {
                f.stamp(node, terminal, -sign * self.multiplier * derivative);
            }
        }
        for (index, terminal, input) in [(1, drain_node, drain), (2, source_node, source)] {
            for (node, sign) in [(terminal, 1.0), (gate_node, -1.0)] {
                q.stamp(node, auxiliary[index], sign * CHARGE_UNIT);
                for (column, derivative) in self.terminals.into_iter().zip(input) {
                    q.stamp(node, column, -sign * self.multiplier * derivative);
                }
            }
        }
        Ok(())
    }
}
