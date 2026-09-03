//! The two forms one planned value entry can take.
//!
//! Canonical lowering has two routes to the same expression. The MIR route
//! emits a flat postfix [`NativeProgram`]: an operation stream with an implicit
//! operand stack, which every backend lifts into SSA before it emits anything.
//! The CFG route ([`crate::jit::cfg_program::lower_cfg_function`]) emits a
//! block program directly — [`ssa::Program`], with basic blocks, typed block
//! parameters and terminators — because a conditional it lowered was a branch
//! in the source and stays one all the way down.
//!
//! Those are different types, so the plan cannot hold "the program" as one of
//! them. [`PlanProgram`] is the sum: a plan entry carries whichever form its
//! route produced, and each backend dispatches on it. Since W-F3c the default
//! plan takes the block variant for every module's `stamp_values`, `jacobians`
//! and `reactive_jacobians`; `parameter_defaults`, `static_conditions`, noise
//! and the assignment passes stay postfix, as does every field of a module the
//! CFG route refuses.
//!
//! # What a block program has to carry that SSA does not
//!
//! A [`NativeProgram`] records three runtime read-sets alongside its
//! operations: the contribution currents it reads from this pass, the ones it
//! reads from an earlier pass, and the branch unknowns it loads. The runtime
//! validates a dispatch against those sets and the fused kernels order their
//! work by them. [`ssa::Program`] carries none of it.
//!
//! [`BlockProgram`] therefore *recomputes* them from its own instructions. It
//! must not borrow the MIR program's sets: the two routes do not reach the same
//! unknowns (the CFG census measured 54 reactive entries reachable only through
//! the CFG route), so a borrowed set would describe a program that was never
//! compiled — the class of defect that is silently wrong rather than loudly
//! broken.

#![cfg_attr(not(feature = "native"), allow(dead_code))]

use super::JitResult;
use super::expr::{NativeOp, NativeProgram, collect_op_dependencies};
use super::ssa;
use crate::canonical_ir::state::{CanonicalStateFamily, CanonicalStateOperator};
use crate::codegen::StateRenumberingError;
use crate::codegen::state_renumbering::StateSlotMapping;

/// One value entry lowered through the CFG route, plus the runtime read-sets
/// its instructions imply.
#[derive(Debug, Clone)]
pub(crate) struct BlockProgram {
    program: ssa::Program,
    current_pair_dependencies: Vec<usize>,
    prior_current_dependencies: Vec<usize>,
    branch_unknown_dependencies: Vec<usize>,
}

impl BlockProgram {
    /// Adopt one lowered block program into a plan.
    ///
    /// Fallible, and the reason is the state vocabulary. A state instruction
    /// names a slot, and which canonical site owns which slot is decided in
    /// exactly one place — [`StateSlotMapping`], built from the module's
    /// bytecode. This constructor *consults* that decision rather than
    /// repeating it: every slot the block program addresses must be one the
    /// mapping allocated for that family, or the two routes have numbered the
    /// same module differently and the entry would read a record nothing
    /// writes. That is [`StateRenumberingError::SlotUnclaimed`], which is where
    /// the whole program keeps its numbering refusals.
    ///
    /// The check is deliberately a subset test over the operations whose family
    /// is named by the bytecode vocabulary (see [`state_slot`]). Operations it
    /// does not classify are passed rather than guessed, so this can refuse a
    /// real disagreement and can never invent one.
    pub(crate) fn adopt(
        module: &str,
        program: ssa::Program,
        slots: &StateSlotMapping,
    ) -> Result<Self, StateRenumberingError> {
        for instruction in program.instructions() {
            let Some((family, slot)) = state_slot(instruction.op()) else {
                continue;
            };
            if !slots.allocated.contains(&(family, slot)) {
                return Err(StateRenumberingError::SlotUnclaimed {
                    module: module.to_owned(),
                    family,
                    slot,
                });
            }
        }
        Ok(Self::from_lowered(program))
    }

    /// Adopt a block program whose module has no state vocabulary to consult.
    ///
    /// The qualification route, and `cfg(test)` says so rather than a comment:
    /// a fixture built by a test owns no compiled module, so there is no
    /// [`StateSlotMapping`] to check it against. Every shipped adoption —
    /// [`crate::jit::cfg_plan_builder`]'s included — goes through
    /// [`Self::adopt`].
    #[cfg(test)]
    pub(crate) fn adopt_unrooted(program: ssa::Program) -> Self {
        Self::from_lowered(program)
    }

    fn from_lowered(program: ssa::Program) -> Self {
        let dependencies = collect_op_dependencies(
            program
                .instructions()
                .iter()
                .map(|instruction| instruction.op()),
        );
        Self {
            program,
            current_pair_dependencies: dependencies.current_pairs,
            prior_current_dependencies: dependencies.prior_currents,
            branch_unknown_dependencies: dependencies.branch_unknowns,
        }
    }

    pub(crate) fn ssa(&self) -> &ssa::Program {
        &self.program
    }
}

/// The `(family, slot)` a native operation addresses, where the bytecode
/// vocabulary names one.
///
/// The families come from [`CanonicalStateOperator::family`] — the same
/// function [`CanonicalStateOperator::bytecode_slot`] classifies bytecode
/// instructions through — so a block program is checked against the mapping in
/// the mapping's own terms.
///
/// Only operations with a named bytecode counterpart are classified.
/// `TableDerivative`, `LimiterPrevious` and `LimiterStore` address arrays the
/// bytecode side reaches under a different instruction, so classifying them
/// here would risk refusing a module over a slot the mapping simply never
/// counted. They are left unclassified on purpose.
fn state_slot(op: NativeOp) -> Option<(CanonicalStateFamily, usize)> {
    let (operator, slot) = match op {
        NativeOp::DdtState(slot) => (CanonicalStateOperator::Ddt, slot),
        NativeOp::IdtState(slot) => (CanonicalStateOperator::Idt, slot),
        NativeOp::IdtModState(slot) => (CanonicalStateOperator::IdtMod, slot),
        NativeOp::TransitionState(slot) | NativeOp::TransitionStateDerivative(slot) => {
            (CanonicalStateOperator::Transition, slot)
        }
        NativeOp::SlewState(slot) | NativeOp::SlewStateDerivative(slot) => {
            (CanonicalStateOperator::Slew, slot)
        }
        NativeOp::AbsDelayState(slot)
        | NativeOp::AbsDelayStateMax(slot)
        | NativeOp::AbsDelayStateDerivative(slot)
        | NativeOp::AbsDelayStateDerivativeMax(slot) => (CanonicalStateOperator::Absdelay, slot),
        NativeOp::LaplaceState(slot) | NativeOp::LaplaceStateDerivative(slot) => {
            (CanonicalStateOperator::Laplace, slot)
        }
        NativeOp::ZiState(layout) | NativeOp::ZiStateDerivative(layout) => {
            (CanonicalStateOperator::Zi, layout.filter_id)
        }
        NativeOp::CrossState(slot) | NativeOp::LastCrossingState(slot) => {
            (CanonicalStateOperator::Cross, slot)
        }
        NativeOp::AboveState(slot) => (CanonicalStateOperator::Above, slot),
        NativeOp::TimerState(slot) => (CanonicalStateOperator::Timer, slot),
        NativeOp::LimitState(slot) => (CanonicalStateOperator::Limit, slot),
        NativeOp::TableLookup(slot) => (CanonicalStateOperator::TableLookup, slot),
        _ => return None,
    };
    Some((operator.family(), slot))
}

/// One plan entry's program, in whichever form its route produced.
///
/// Both arms have a constructor:
/// [`build_model_plan_with_canonical_ir`](crate::jit::plan_builder::build_model_plan_with_canonical_ir)
/// builds every entry as `Postfix`, and
/// [`build_model_plan_from_canonical_cfg`](crate::jit::cfg_plan_builder::build_model_plan_from_canonical_cfg)
/// builds the value fields as `Blocks`. What production compiles is
/// [`build_default_model_plan`](crate::jit::cfg_plan_builder::build_default_model_plan),
/// which composes the two.
#[derive(Debug, Clone)]
pub(crate) enum PlanProgram {
    /// The MIR route's flat operation stream. Parameter defaults, static
    /// conditions and noise on every module; every field of a module the CFG
    /// route refuses.
    Postfix(NativeProgram),
    /// The CFG route's blocks and terminators. Residual, Jacobian and
    /// reactive-Jacobian entries, on every module the CFG route builds.
    Blocks(BlockProgram),
}

impl PlanProgram {
    pub(crate) fn borrow(&self) -> PlanProgramRef<'_> {
        match self {
            Self::Postfix(program) => PlanProgramRef::Postfix(program),
            Self::Blocks(program) => PlanProgramRef::Blocks(program),
        }
    }

    pub(crate) fn current_pair_dependencies(&self) -> &[usize] {
        self.borrow().current_pair_dependencies()
    }

    pub(crate) fn prior_current_dependencies(&self) -> &[usize] {
        self.borrow().prior_current_dependencies()
    }

    pub(crate) fn branch_unknown_dependencies(&self) -> &[usize] {
        self.borrow().branch_unknown_dependencies()
    }
}

/// A borrowed [`PlanProgram`], and the form every backend consumes.
///
/// Borrowed because the emitters walk a whole plan at once and mix in the
/// programs an assignment carries, which are postfix by construction; a
/// borrowed sum lets both reach the same encoder without copying a program.
#[derive(Debug, Clone, Copy)]
pub(crate) enum PlanProgramRef<'a> {
    Postfix(&'a NativeProgram),
    Blocks(&'a BlockProgram),
}

impl<'a> From<&'a NativeProgram> for PlanProgramRef<'a> {
    fn from(program: &'a NativeProgram) -> Self {
        Self::Postfix(program)
    }
}

impl<'a> PlanProgramRef<'a> {
    pub(crate) fn current_pair_dependencies(self) -> &'a [usize] {
        match self {
            Self::Postfix(program) => program.current_pair_dependencies(),
            Self::Blocks(program) => &program.current_pair_dependencies,
        }
    }

    pub(crate) fn prior_current_dependencies(self) -> &'a [usize] {
        match self {
            Self::Postfix(program) => program.prior_current_dependencies(),
            Self::Blocks(program) => &program.prior_current_dependencies,
        }
    }

    pub(crate) fn branch_unknown_dependencies(self) -> &'a [usize] {
        match self {
            Self::Postfix(program) => program.branch_unknown_dependencies(),
            Self::Blocks(program) => &program.branch_unknown_dependencies,
        }
    }

    /// The deepest operand stack the entry needs, for the wasm plan summary
    /// that sizes a frame before it emits.
    ///
    /// Only that summary asks a whole entry: the native backends take the
    /// depth off the lowered SSA instead, because they check it against their
    /// own architectural ceiling after the lift rather than before it.
    #[cfg(feature = "wasm-jit")]
    pub(crate) fn max_stack_depth(self) -> usize {
        match self {
            Self::Postfix(program) => program.max_stack_depth(),
            Self::Blocks(program) => program.program.maximum_stack_depth(),
        }
    }

    /// How many operations the entry carries, for the wasm plan summary that
    /// reports emitted size.
    #[cfg(feature = "wasm-jit")]
    pub(crate) fn operation_count(self) -> usize {
        match self {
            Self::Postfix(program) => program.ops().len(),
            Self::Blocks(program) => program.program.instructions().len(),
        }
    }

    /// The block form of this entry, lifting the postfix stream when that is
    /// what it holds.
    ///
    /// The postfix arm is the shipped route's exact lowering, unchanged: the
    /// sum type decides which program a backend compiles, never how.
    pub(crate) fn lower_to_ssa(self) -> JitResult<ssa::Program> {
        match self {
            Self::Postfix(program) => ssa::Program::lower(program),
            Self::Blocks(program) => Ok(program.program.clone()),
        }
    }

    /// Which lowering this entry carries. Reported by the plan censuses; no
    /// backend refuses an entry for its form any more, so nothing in the
    /// shipping path reads it.
    #[cfg(test)]
    pub(crate) fn form_name(self) -> &'static str {
        match self {
            Self::Postfix(_) => "postfix",
            Self::Blocks(_) => "block",
        }
    }
}

#[cfg(all(test, feature = "native"))]
mod tests {
    use super::{BlockProgram, PlanProgramRef};
    use crate::canonical_ir::state::CanonicalStateFamily;
    use crate::codegen::StateRenumberingError;
    use crate::codegen::state_renumbering::StateSlotMapping;
    use crate::jit::expr::{NativeOp, NativeProgram};
    use crate::jit::ssa;

    /// A postfix fixture whose recorded read-sets are checked against its own
    /// operation stream, so an equality pin against it means something.
    fn postfix(
        ops: Vec<NativeOp>,
        depth: usize,
        current_pairs: Vec<usize>,
        prior_currents: Vec<usize>,
    ) -> NativeProgram {
        let program = NativeProgram::from_ops_for_test(ops, depth, current_pairs, prior_currents);
        program
            .validate_dependency_metadata()
            .expect("the fixture records the read-sets its operations imply");
        program
    }

    /// One source expression, two program forms, one read-set.
    ///
    /// The block form recomputes what the postfix form recorded. If the two
    /// ever disagreed, the runtime would validate a dispatch against one set
    /// and the backend would execute the other.
    #[test]
    fn the_block_form_recomputes_the_postfix_form_read_sets() {
        let source = postfix(
            vec![
                NativeOp::LoadCurrent(2),
                NativeOp::LoadBranchUnknown(1),
                NativeOp::Add,
                NativeOp::LoadPriorCurrent(4),
                NativeOp::Add,
                NativeOp::LoadCurrent(2),
                NativeOp::Add,
                NativeOp::LoadBranchUnknown(0),
                NativeOp::Add,
            ],
            2,
            vec![2],
            vec![4],
        );
        let blocks =
            BlockProgram::adopt_unrooted(ssa::Program::lower(&source).expect("lift the postfix"));

        let postfix_entry = PlanProgramRef::Postfix(&source);
        let block_entry = PlanProgramRef::Blocks(&blocks);
        assert_eq!(
            block_entry.current_pair_dependencies(),
            postfix_entry.current_pair_dependencies(),
        );
        assert_eq!(
            block_entry.prior_current_dependencies(),
            postfix_entry.prior_current_dependencies(),
        );
        assert_eq!(
            block_entry.branch_unknown_dependencies(),
            postfix_entry.branch_unknown_dependencies(),
        );
        // Not vacuous: the sets are the ones the operations name, de-duplicated
        // and in first-appearance order.
        assert_eq!(block_entry.current_pair_dependencies(), [2]);
        assert_eq!(block_entry.prior_current_dependencies(), [4]);
        assert_eq!(block_entry.branch_unknown_dependencies(), [1, 0]);
    }

    /// The reason the sets are recomputed rather than borrowed: a block program
    /// can read an unknown the postfix program for the same entry never
    /// reaches, and its read-set has to say so.
    #[test]
    fn a_block_only_unknown_reaches_the_block_form_read_set() {
        let mir_route = postfix(
            vec![NativeOp::LoadBranchUnknown(0)],
            1,
            Vec::new(),
            Vec::new(),
        );
        let cfg_route = postfix(
            vec![
                NativeOp::LoadBranchUnknown(0),
                NativeOp::LoadBranchUnknown(7),
                NativeOp::Add,
            ],
            2,
            Vec::new(),
            Vec::new(),
        );
        let blocks = BlockProgram::adopt_unrooted(
            ssa::Program::lower(&cfg_route).expect("lift the postfix"),
        );

        assert_eq!(
            PlanProgramRef::Postfix(&mir_route).branch_unknown_dependencies(),
            [0]
        );
        assert_eq!(
            PlanProgramRef::Blocks(&blocks).branch_unknown_dependencies(),
            [0, 7],
            "an unknown only the block program reads still has to be in its read set"
        );
    }

    /// A read inside one arm of a branch is a read of the entry, wherever the
    /// block that holds it sits in the layout.
    #[test]
    fn reads_inside_branch_arms_reach_the_read_set() {
        let conditional = postfix(
            vec![
                NativeOp::LoadBranchUnknown(3),
                NativeOp::LoadCurrent(1),
                NativeOp::LoadPriorCurrent(5),
                NativeOp::IfElse,
            ],
            3,
            vec![1],
            vec![5],
        );
        let select = ssa::Program::lower(&conditional).expect("lift the postfix");
        let branched = select
            .with_branching_conditionals()
            .expect("split the conditional");
        assert!(
            !branched.is_single_block(),
            "the fixture has to actually branch or it proves nothing"
        );

        let blocks = BlockProgram::adopt_unrooted(branched);
        let entry = PlanProgramRef::Blocks(&blocks);
        assert_eq!(entry.current_pair_dependencies(), [1]);
        assert_eq!(entry.prior_current_dependencies(), [5]);
        assert_eq!(entry.branch_unknown_dependencies(), [3]);
    }

    /// Adoption consults the module's slot mapping rather than trusting the
    /// number a block program carries.
    ///
    /// A `ddt` site is numbered once, by [`StateSlotMapping`]. A block program
    /// naming a slot that mapping never allocated has been numbered by some
    /// other rule, and the refusal says exactly which family and slot.
    #[test]
    fn adoption_refuses_a_state_slot_the_module_never_allocated() {
        let source = postfix(
            vec![NativeOp::Const(1.0), NativeOp::DdtState(3)],
            1,
            Vec::new(),
            Vec::new(),
        );
        let lowered = ssa::Program::lower(&source).expect("lift the postfix");

        let unallocated = StateSlotMapping::default();
        match BlockProgram::adopt("bsim4", lowered.clone(), &unallocated) {
            Err(StateRenumberingError::SlotUnclaimed {
                module,
                family,
                slot,
            }) => {
                assert_eq!(module, "bsim4");
                assert_eq!(family, CanonicalStateFamily::Integration);
                assert_eq!(slot, 3);
            }
            other => panic!("expected an unclaimed-slot refusal, got {other:?}"),
        }

        let mut allocated = StateSlotMapping::default();
        allocated
            .allocated
            .insert((CanonicalStateFamily::Integration, 3));
        let adopted = BlockProgram::adopt("bsim4", lowered, &allocated)
            .expect("a slot the module allocated is adoptable");
        assert_eq!(adopted.ssa().instructions().len(), 2);
    }
}
