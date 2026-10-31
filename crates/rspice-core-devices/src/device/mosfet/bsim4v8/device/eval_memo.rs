//! The instance's most recent module evaluations.
//!
//! `eval` is a pure function of its bias inputs, the engine gmin and the
//! instance's immutable model data. One transient Newton iterate is evaluated
//! at the same inputs several times: by the update, by the charge companion,
//! by the residual proof's physical probe (whenever limiting left the iterate
//! unchanged), by the charge truncation and by the history commit. ngspice
//! loads each iterate once and every later consumer reads `CKTstate0`; here a
//! repeated evaluation returns the recorded result, which is the same value
//! bit for bit.

use std::cell::RefCell;

use super::super::eval::{Bsim4v8Bias, Bsim4v8Charge, Bsim4v8JunctionBias, Bsim4v8Op};
use crate::Value;

/// The inputs of one evaluation, compared bit for bit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Bsim4v8EvalInputs {
    bias: [u64; 3],
    junction: Option<[u64; 2]>,
    gate_mid_vgs: Option<u64>,
    gmin: u64,
}

impl Bsim4v8EvalInputs {
    pub(super) fn new(
        bias: Bsim4v8Bias,
        junction: Option<Bsim4v8JunctionBias>,
        gate_mid_vgs: Option<Value>,
        gmin: Value,
    ) -> Self {
        Self {
            bias: [bias.vds.to_bits(), bias.vgs.to_bits(), bias.vbs.to_bits()],
            junction: junction.map(|j| [j.vbs.to_bits(), j.vbd.to_bits()]),
            gate_mid_vgs: gate_mid_vgs.map(Value::to_bits),
            gmin: gmin.to_bits(),
        }
    }

    /// Only the charge model reads the gate-mid bias, so a DC request may
    /// reuse an evaluation at any gate-mid bias.
    fn same_dc_inputs(&self, other: &Self) -> bool {
        self.bias == other.bias && self.junction == other.junction && self.gmin == other.gmin
    }
}

#[derive(Debug)]
struct Bsim4v8EvalRecord {
    inputs: Bsim4v8EvalInputs,
    /// The module's result; `charge` is present when the charge model ran.
    op: Bsim4v8Op,
    /// The DC section's `output_vdsat`, which the charge model overwrites.
    dc_output_vdsat: Value,
}

/// The two most recent evaluations, newest first. A clone starts empty: the
/// records are a cache, never state.
#[derive(Debug, Default)]
pub(super) struct Bsim4v8EvalMemo(RefCell<[Option<Box<Bsim4v8EvalRecord>>; 2]>);

impl Clone for Bsim4v8EvalMemo {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl Bsim4v8EvalMemo {
    fn find<T>(&self, mut hit: impl FnMut(&Bsim4v8EvalRecord) -> Option<T>) -> Option<T> {
        let records = self.0.borrow();
        records.iter().flatten().find_map(|record| hit(record))
    }

    /// The DC-only result of a recorded evaluation at `inputs`: exactly what
    /// `eval` returns without the charge model.
    pub(super) fn dc(&self, inputs: &Bsim4v8EvalInputs) -> Option<Bsim4v8Op> {
        self.find(|record| {
            record.inputs.same_dc_inputs(inputs).then(|| {
                let mut op = record.op.clone();
                op.charge = None;
                op.output_vdsat = record.dc_output_vdsat;
                op
            })
        })
    }

    /// The charge state and channel mode of a recorded evaluation at `inputs`
    /// that ran the charge model.
    pub(super) fn charge(&self, inputs: &Bsim4v8EvalInputs) -> Option<(Bsim4v8Charge, i32)> {
        self.find(|record| {
            let charge = record
                .op
                .charge
                .as_ref()
                .filter(|_| record.inputs == *inputs)?;
            Some((charge.clone(), record.op.mode))
        })
    }

    /// Record an evaluation as the newest, evicting the older record.
    pub(super) fn record(&self, inputs: Bsim4v8EvalInputs, op: &Bsim4v8Op, dc_output_vdsat: Value) {
        let mut records = self.0.borrow_mut();
        records.swap(0, 1);
        match &mut records[0] {
            Some(record) => {
                record.inputs = inputs;
                record.op.clone_from(op);
                record.dc_output_vdsat = dc_output_vdsat;
            }
            slot @ None => {
                *slot = Some(Box::new(Bsim4v8EvalRecord {
                    inputs,
                    op: op.clone(),
                    dc_output_vdsat,
                }));
            }
        }
    }
}
