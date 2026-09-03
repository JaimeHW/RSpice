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
//! # What a prelude may not publish
//!
//! A value that reads a *contribution current*. The runtime publishes those
//! during the evaluation — `VmContext::currents` grows one slot as each
//! residual is evaluated, and the fused kernels store each one as they go — so
//! a value computed ahead of the first publication would read a slot nothing
//! has written. [`LiveCurrentTaint`] is what keeps them out, and the entries it
//! names keep the per-entry cone they have always had.
//!
//! # Where this is used
//!
//! [`build_model_plan_from_canonical_cfg`](crate::jit::cfg_plan_builder::build_model_plan_from_canonical_cfg)
//! builds one per module and turns every publishable entry into
//! [`CfgPrelude::entry_program`]. `DEFAULT_PLAN_ROUTE` still selects the
//! postfix route, so nothing production compiles carries one yet; the fixture
//! and the frame census beside this module are what qualified the design.

use std::collections::HashMap;

use crate::canonical_ir::{
    CfgFunction, CfgStateAllocation, CfgTerminator, CfgValueKind, ValueId, prune_cfg_to_outputs,
};
use crate::codegen::state_renumbering::StateSlotMapping;
use crate::jit::JitResult;
use crate::jit::cfg_plan_builder::{CfgPlanEntry, CfgPlanRefusal, CfgPlanRefused};
use crate::jit::cfg_program::{CfgRuntimeBindings, lower_cfg_function_to_prelude_slots};
use crate::jit::expr::NativeOp;
use crate::jit::plan_program::{BlockProgram, PlanProgramRef};
use crate::jit::ssa::{BlockId, BuilderTerminator, Program, ProgramBuilder, ValueType};

/// Which of a function's values a prelude must not publish, because computing
/// them early would read a current the evaluation has not published yet.
///
/// # Why this is a taint and not a refusal
///
/// [`CfgValueKind::ContributedCurrent`] is the CFG's name for "the running sum
/// the runtime already holds for this branch". The executable lowering reads it
/// out of the runtime's own storage — `NativeOp::LoadCurrent` for a terminal
/// pair, `NativeOp::LoadPriorCurrent` for a contribution slot — and that
/// storage is filled *as the evaluation proceeds*. An entry that reads one is
/// therefore ordered against the residuals before it, which is the whole point
/// of `JitCurrentDependencies::evaluation_kernel_order_safe`; a prelude runs
/// before residual zero, so it cannot compute such a value at all.
///
/// Refusing the module over one probe would be the wrong trade: the probe class
/// is closed and pinned (`a_contribution_current_probe_reads_the_shipped_routes_storage`),
/// and every other entry of the same module still wants a slot. So the entries
/// that read one keep the cone they have always had and the rest take slots.
///
/// # Control dependence, and why it is not decided here
///
/// A cone contains every *surviving* block's branch condition —
/// `prune_to_outputs` marks them live whether or not anything downstream reads
/// them — so a condition derived from a contributed current would put the read
/// into the union whatever this analysis says about the outputs. Which blocks
/// survive is a property of the union prune, not of the whole function, so
/// deciding it here would refuse a prelude over a branch the union never
/// reaches. [`Self::taints_control_flow`] therefore only *reports* that the
/// function has such a branch; what decides it is the guard in
/// [`CfgPrelude::build`], read off the operations the prelude actually emitted,
/// and a prelude that trips it is dropped rather than taking the module down.
#[derive(Debug, Clone)]
pub(crate) struct LiveCurrentTaint {
    tainted: Vec<bool>,
    control: bool,
}

impl LiveCurrentTaint {
    /// Every value that transitively reads a contribution current.
    ///
    /// One forward walk over the use graph, paid only by a module that has a
    /// probe at all: the root scan returns immediately on the rest, which is
    /// every compact model the size census measured.
    pub(crate) fn build(function: &CfgFunction) -> Self {
        let mut tainted = vec![false; function.values.len()];
        let mut worklist: Vec<usize> = Vec::new();
        for (index, value) in function.values.iter().enumerate() {
            if matches!(value.kind, CfgValueKind::ContributedCurrent { .. }) {
                tainted[index] = true;
                worklist.push(index);
            }
        }
        if worklist.is_empty() {
            return Self {
                tainted,
                control: false,
            };
        }

        // Uses, both kinds: an operand of an instruction's value, and an
        // argument on an edge into a block parameter. The second is what makes
        // a merge of a probed current tainted, and leaving it out would publish
        // a slot whose value depends on one.
        let mut users: Vec<Vec<u32>> = vec![Vec::new(); function.values.len()];
        for (index, value) in function.values.iter().enumerate() {
            let user = u32::try_from(index).expect("CFG value index fits u32");
            for operand in value.kind.operands() {
                users[usize::from(operand)].push(user);
            }
        }
        fn edge(args: &[ValueId], params: &[ValueId], users: &mut [Vec<u32>]) {
            for (argument, param) in args.iter().zip(params) {
                let param = u32::try_from(usize::from(*param)).expect("CFG value index fits u32");
                users[usize::from(*argument)].push(param);
            }
        }
        for block in &function.blocks {
            match &block.terminator {
                CfgTerminator::Jump { target, args } => {
                    edge(args, &function.block(*target).params, &mut users);
                }
                CfgTerminator::Branch {
                    then_target,
                    then_args,
                    else_target,
                    else_args,
                    ..
                } => {
                    edge(then_args, &function.block(*then_target).params, &mut users);
                    edge(else_args, &function.block(*else_target).params, &mut users);
                }
                CfgTerminator::Wait {
                    resume,
                    resume_args,
                    ..
                } => {
                    edge(resume_args, &function.block(*resume).params, &mut users);
                }
                CfgTerminator::Return | CfgTerminator::Unset => {}
            }
        }

        while let Some(value) = worklist.pop() {
            for user in &users[value] {
                let user = *user as usize;
                if !tainted[user] {
                    tainted[user] = true;
                    worklist.push(user);
                }
            }
        }

        let control = function.blocks.iter().any(|block| match &block.terminator {
            CfgTerminator::Branch { condition, .. } => tainted[usize::from(*condition)],
            _ => false,
        });
        Self { tainted, control }
    }

    /// Whether a prelude may publish `value`.
    pub(crate) fn publishable(&self, value: ValueId) -> bool {
        !self.tainted[usize::from(value)]
    }

    /// Whether any branch of this function depends on a contributed current.
    ///
    /// Reported, not acted on: see the type documentation.
    pub(crate) fn taints_control_flow(&self) -> bool {
        self.control
    }
}

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

        // The ordering guard, checked on the emitted operations rather than
        // trusted from the analysis that was supposed to prevent it. A prelude
        // runs before the first contribution is published, so a load of one
        // here would read a slot nothing has written — silently, and only in a
        // model that has a probe. `LiveCurrentTaint` keeps those entries out;
        // this refuses the plan if it ever fails to.
        let reads = PlanProgramRef::Blocks(&program);
        if !reads.current_pair_dependencies().is_empty()
            || !reads.prior_current_dependencies().is_empty()
        {
            return Err(refuse(
                CfgPlanRefusal::PreludeLiveCurrent,
                format!(
                    "the prelude reads contribution currents {:?}/{:?}, which are published after \
                     it runs",
                    reads.current_pair_dependencies(),
                    reads.prior_current_dependencies()
                ),
            ));
        }

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

    /// The shared program, taken for a plan to carry.
    pub(crate) fn into_program(self) -> BlockProgram {
        self.program
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
