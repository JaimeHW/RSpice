//! The CFG route's assignment pass: one program that computes every entry
//! output once, into a per-evaluation slot.
//!
//! # What this is for
//!
//! [`build_model_plan_from_canonical_cfg`](crate::jit::cfg_plan_builder::build_model_plan_from_canonical_cfg)
//! lowers each of a module's `stamp_values`, `jacobians` and
//! `reactive_jacobians` entries as its own pruned dependence cone of the
//! scalarized differentiated body. The cones overlap almost completely: the
//! size census measured `Σ cone / union` at 24 on asmesd, 75 on hicumL2va,
//! 300 on bsimcmg_va and 383 on asmhemt, which is the whole of the CFG route's
//! code-size explosion. Two Jacobian entries of the same residual share about
//! ninety-five percent of their instructions and are still never *codegen
//! identical*, so the value-entry cache cannot reach it.
//!
//! The postfix route has never had this problem, because it has an assignment
//! pass: every shared value is computed once per evaluation into a slot, and
//! each entry is a short program that reads slots. A [`CfgPrelude`] is that
//! pass for the CFG route — the union of every entry output, pruned once from
//! the body, lowered once, publishing each output into a slot.
//!
//! # What a slot is, and is not
//!
//! One `f64` in `EvalContext::prelude_slots`, live for exactly one evaluation.
//! There is one per *distinct entry output value*, and none for the union's
//! interior: an interior value stays an SSA value and, if the allocator runs
//! out of registers, a spill slot in the program's own frame. Two entries that
//! read the same value share one slot, which is why the count is over distinct
//! values rather than over entries.
//!
//! Slots deliberately do not alias the runtime's variable slots. A scalarized
//! derivative lane has no MIR variable to alias, so aliasing would buy only the
//! eventual retirement of the postfix assignment pass, which is gated on
//! separate work; and sharing one array between two publications that are
//! written by different passes is exactly the kind of coupling that turns a
//! sizing mistake into a silently wrong read.
//!
//! # Where this is not yet used
//!
//! Nothing in production builds a prelude: the CFG plan still inlines each
//! entry's cone, and `DEFAULT_PLAN_ROUTE` still selects the postfix route
//! anyway. This module is reached by its own fixture and by the frame census
//! beside it, which is what qualifies the design before the plan is rewired.

// Reached only by those two tests until W-F10b-2 builds a prelude inside
// `build_model_plan_from_canonical_cfg`, turns each entry into
// `CfgPrelude::entry_program`, and sizes `NativeRequiredStorage::prelude_slots`
// from `CfgPrelude::slot_count`. This attribute comes off with that lane.
#![cfg_attr(not(test), allow(dead_code))]

use std::collections::HashMap;

use crate::canonical_ir::{CfgFunction, CfgStateAllocation, ValueId, prune_cfg_to_outputs};
use crate::codegen::state_renumbering::StateSlotMapping;
use crate::jit::JitResult;
use crate::jit::cfg_plan_builder::{CfgPlanEntry, CfgPlanRefusal, CfgPlanRefused};
use crate::jit::cfg_program::{CfgRuntimeBindings, lower_cfg_function_to_prelude_slots};
use crate::jit::expr::NativeOp;
use crate::jit::plan_program::BlockProgram;
use crate::jit::ssa::{BlockId, BuilderTerminator, Program, ProgramBuilder, ValueType};

/// One module's prelude: the shared program, and which slot each entry reads.
#[derive(Debug, Clone)]
pub(crate) struct CfgPrelude {
    program: BlockProgram,
    slots: HashMap<CfgPlanEntry, usize>,
    slot_count: usize,
}

impl CfgPrelude {
    /// Build the prelude for `entries`, which are `(entry, output value)` pairs
    /// taken from the *same* `function` every entry would otherwise be sliced
    /// out of.
    ///
    /// One prune, one lowering, one hoist decision per analog-operator site.
    /// The union prune is a single linear walk over the body — measured at
    /// 0.0–0.1 s on every shipped module, including asmhemt's 91,461-instruction
    /// body — so building the prelude costs strictly less than building the
    /// first entry cone did.
    pub(crate) fn build(
        module: &str,
        function: &CfgFunction,
        entries: &[(CfgPlanEntry, ValueId)],
        state: &CfgStateAllocation,
        bindings: &CfgRuntimeBindings,
        state_slots: &StateSlotMapping,
    ) -> Result<Self, CfgPlanRefused> {
        let refuse = |class: CfgPlanRefusal, detail: String| CfgPlanRefused {
            module: module.to_owned(),
            class,
            detail,
        };
        if entries.is_empty() {
            return Err(refuse(
                CfgPlanRefusal::Lowering,
                "a prelude with no entry outputs would compute nothing".to_string(),
            ));
        }

        // One slot per distinct output value, assigned in the order the entries
        // are given so that the numbering is a property of the plan rather than
        // of a hash iteration.
        let mut slot_of_value: HashMap<ValueId, usize> = HashMap::new();
        let mut outputs: Vec<ValueId> = Vec::new();
        let mut slots: HashMap<CfgPlanEntry, usize> = HashMap::with_capacity(entries.len());
        for (entry, value) in entries {
            let slot = *slot_of_value.entry(*value).or_insert_with(|| {
                outputs.push(*value);
                outputs.len() - 1
            });
            if slots.insert(*entry, slot).is_some() {
                return Err(refuse(
                    CfgPlanRefusal::Lowering,
                    format!("{entry} was given a prelude slot twice"),
                ));
            }
        }
        let slot_count = outputs.len();

        // Prune to every output at once. This is the union the census names as
        // the numerator of the fix, and taking it here is what makes the whole
        // prelude one slice rather than one slice per entry.
        let (pruned, pruned_outputs) = prune_cfg_to_outputs(function, &outputs);
        let publications: Vec<(ValueId, usize)> = pruned_outputs
            .into_iter()
            .enumerate()
            .map(|(slot, value)| (value, slot))
            .collect();

        let program = lower_cfg_function_to_prelude_slots(&pruned, &publications, state, bindings)
            .map_err(|error| refuse(CfgPlanRefusal::Lowering, format!("prelude: {error}")))?;
        let program = BlockProgram::adopt(module, program, state_slots)
            .map_err(|error| refuse(CfgPlanRefusal::SlotUnclaimed, format!("prelude: {error}")))?;

        Ok(Self {
            program,
            slots,
            slot_count,
        })
    }

    /// The shared program, which publishes every slot once per evaluation.
    pub(crate) fn program(&self) -> &BlockProgram {
        &self.program
    }

    /// How many `f64` slots the evaluation must supply.
    pub(crate) fn slot_count(&self) -> usize {
        self.slot_count
    }

    /// The slot `entry` reads, or `None` for an entry this prelude does not
    /// publish — a structural zero, which the plan builds as a constant.
    pub(crate) fn slot(&self, entry: CfgPlanEntry) -> Option<usize> {
        self.slots.get(&entry).copied()
    }

    /// The program `entry` becomes once the prelude publishes its value: one
    /// instruction that reads one slot.
    ///
    /// This is the whole of the size fix. An entry that was its own copy of the
    /// dependence cone — thousands of instructions on a compact model — is a
    /// single load, and every backend already emits the surrounding value
    /// function for it.
    pub(crate) fn entry_program(&self, entry: CfgPlanEntry) -> Option<JitResult<Program>> {
        let slot = self.slot(entry)?;
        Some(slot_read_program(slot))
    }
}

/// A value program that reads one prelude slot and returns it.
pub(crate) fn slot_read_program(slot: usize) -> JitResult<Program> {
    let entry = BlockId::new(0)?;
    let mut builder = ProgramBuilder::new(&[Vec::new()])?;
    builder.begin_block(entry)?;
    let value = builder.push(NativeOp::LoadPreludeSlot(slot), &[], ValueType::F64)?;
    builder.end_block(BuilderTerminator::Return(value))?;
    builder.finish(entry, entry)
}
