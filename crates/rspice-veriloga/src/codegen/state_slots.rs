//! The bytecode half of the state vocabulary: reading and rewriting the slot a
//! state instruction names, and correlating a program's slots with the
//! canonical sites that own them.
//!
//! [`crate::canonical_ir::state`] defines what makes an expression own a state
//! record and how the executed sites of a module are numbered. Only the
//! direction that *mentions bytecode* lives here — from an [`Instruction`] to
//! the slot it addresses, and back.
//!
//! This used to sit in the JIT's expression lowering, where it was reachable
//! only under `feature = "native"` or `feature = "wasm-jit"`. The per-site
//! renumbering in [`super::state_renumbering`] runs on every compiled model
//! regardless of which runtime will execute it — the VM route has to see the
//! same slot numbers the JIT route does, or a model interpreted and a model
//! compiled would integrate different histories — so the vocabulary had to
//! become unconditional. The JIT keeps thin wrappers that add the model name to
//! the error.

use crate::canonical_ir::state::{self, CanonicalStateOperator, MissingExpression};
use crate::canonical_ir::{ExprId, MirModel};
use crate::codegen::{BytecodeProgram, Instruction};

/// Read the slot a state instruction addresses, if it is one of this
/// operator's.
///
/// The `Zi` families read through [`super::ZiRuntimeLayout::filter_id`], which
/// is the layout's identity field rather than a bare index; every other family
/// carries its slot directly.
impl CanonicalStateOperator {
    pub(crate) fn bytecode_slot(self, instruction: &Instruction) -> Option<usize> {
        match (self, instruction) {
            (Self::Ddt, Instruction::DdtState(slot)) | (Self::Idt, Instruction::IdtState(slot)) => {
                Some(*slot)
            }
            (Self::IdtMod, Instruction::IdtModState(slot)) => Some(*slot),
            (
                Self::Transition,
                Instruction::TransitionState(slot) | Instruction::TransitionStateDerivative(slot),
            ) => Some(*slot),
            (Self::Slew, Instruction::SlewState(slot) | Instruction::SlewStateDerivative(slot)) => {
                Some(*slot)
            }
            (
                Self::Absdelay,
                Instruction::AbsDelayState(slot)
                | Instruction::AbsDelayStateMax(slot)
                | Instruction::AbsDelayStateDerivative(slot)
                | Instruction::AbsDelayStateDerivativeMax(slot),
            ) => Some(*slot),
            (
                Self::Laplace,
                Instruction::LaplaceState(slot) | Instruction::LaplaceStateDerivative(slot),
            ) => Some(*slot),
            (Self::Zi, Instruction::ZiState(layout) | Instruction::ZiStateDerivative(layout)) => {
                Some(layout.filter_id)
            }
            (Self::Cross, Instruction::CrossState(slot))
            | (Self::Cross, Instruction::LastCrossingState(slot)) => Some(*slot),
            (Self::Above, Instruction::AboveState(slot)) => Some(*slot),
            (Self::Timer, Instruction::TimerState(slot)) => Some(*slot),
            (Self::Limit, Instruction::LimitState(slot))
            | (Self::Limit, Instruction::CanonicalLimitState(slot)) => Some(*slot),
            (Self::TableLookup, Instruction::TableLookup(slot)) => Some(*slot),
            _ => None,
        }
    }

    /// Write a new slot into a state instruction of this operator's kind.
    ///
    /// The exact mirror of [`Self::bytecode_slot`], and deliberately written as
    /// the same match rather than as a generic field poke: the two directions
    /// have to agree about which instruction belongs to which family, and a
    /// divergence between them would renumber an instruction the reader never
    /// counted.
    ///
    /// Returns whether the instruction was one of this operator's.
    pub(crate) fn rewrite_bytecode_slot(self, instruction: &mut Instruction, slot: usize) -> bool {
        match (self, instruction) {
            (Self::Ddt, Instruction::DdtState(held))
            | (Self::Idt, Instruction::IdtState(held))
            | (Self::IdtMod, Instruction::IdtModState(held))
            | (
                Self::Transition,
                Instruction::TransitionState(held) | Instruction::TransitionStateDerivative(held),
            )
            | (Self::Slew, Instruction::SlewState(held) | Instruction::SlewStateDerivative(held))
            | (
                Self::Absdelay,
                Instruction::AbsDelayState(held)
                | Instruction::AbsDelayStateMax(held)
                | Instruction::AbsDelayStateDerivative(held)
                | Instruction::AbsDelayStateDerivativeMax(held),
            )
            | (
                Self::Laplace,
                Instruction::LaplaceState(held) | Instruction::LaplaceStateDerivative(held),
            )
            | (Self::Cross, Instruction::CrossState(held) | Instruction::LastCrossingState(held))
            | (Self::Above, Instruction::AboveState(held))
            | (Self::Timer, Instruction::TimerState(held))
            | (
                Self::Limit,
                Instruction::LimitState(held) | Instruction::CanonicalLimitState(held),
            )
            | (Self::TableLookup, Instruction::TableLookup(held)) => {
                *held = slot;
                true
            }
            (Self::Zi, Instruction::ZiState(layout) | Instruction::ZiStateDerivative(layout)) => {
                layout.filter_id = slot;
                true
            }
            _ => false,
        }
    }
}

/// Whether any instruction of the program addresses a state record.
pub(crate) fn carries_state(program: &BytecodeProgram) -> bool {
    program.instructions.iter().any(|instruction| {
        CanonicalStateOperator::ALL
            .iter()
            .any(|operator| operator.bytecode_slot(instruction).is_some())
    })
}

/// Every state-bearing site under one canonical expression, split by the record
/// family it owns and kept in traversal order.
///
/// One walk answers all thirteen families. A canonical expression owns at most
/// one record — [`state::classify`] is what decides which — so a single
/// post-order pass fills every list, where this used to be thirteen passes over
/// the same tree asking a different question each time.
#[derive(Debug, Default)]
pub(crate) struct CanonicalStateSiteScan {
    by_operator: [Vec<ExprId>; CanonicalStateOperator::ALL.len()],
}

impl CanonicalStateSiteScan {
    pub(crate) fn for_expression(
        mir: &MirModel,
        expr_id: ExprId,
    ) -> Result<Self, MissingExpression> {
        let mut scan = Self::default();
        state::visit_state_sites(&mir.expressions, expr_id, &mut |operator, kind| {
            scan.push(kind, operator);
        })?;
        Ok(scan)
    }

    /// The sites of one family, in traversal order.
    pub(crate) fn sites(&self, operator: CanonicalStateOperator) -> &[ExprId] {
        &self.by_operator[operator.index()]
    }

    fn push(&mut self, operator: CanonicalStateOperator, expr_id: ExprId) {
        self.by_operator[operator.index()].push(expr_id);
    }
}

/// Why a canonical expression and the program compiled from it could not be
/// correlated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StatePairingError {
    /// The canonical traversal reached an expression the arena does not hold.
    MissingExpression(ExprId),
    /// The two lists describe a different number of records of one family.
    CountMismatch {
        expression: ExprId,
        operator: CanonicalStateOperator,
        canonical: Vec<ExprId>,
        bytecode: Vec<usize>,
    },
}

impl From<MissingExpression> for StatePairingError {
    fn from(missing: MissingExpression) -> Self {
        Self::MissingExpression(missing.0)
    }
}

impl std::fmt::Display for StatePairingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingExpression(id) => write!(f, "{}", MissingExpression(*id)),
            Self::CountMismatch {
                expression,
                operator,
                canonical,
                bytecode,
            } => write!(
                f,
                "canonical expression {expression} has {} {} operators {canonical:?} but bytecode \
                 program has {} {}State slots {bytecode:?}",
                canonical.len(),
                operator.name(),
                bytecode.len(),
                operator.name(),
            ),
        }
    }
}

/// Give each canonical site the runtime slot the program it is lowered from
/// allocated for it.
///
/// The identity and the order of the sites come from the canonical level; only
/// the *number* comes from the bytecode. Historically the two numbering spaces
/// were not the same size — a module with noise in an assignment is emitted
/// twice, once as its assignment steps and again as the noise-shadowed replay,
/// and the generator allocates a fresh scalar-state slot at each emission, so
/// one canonical `ddt` site could own two bytecode slots.
/// [`super::state_renumbering`] is what collapses that, using exactly this
/// correlation to decide which emitted slot belongs to which site.
///
/// The length disagreement below is therefore a real error rather than a
/// tolerance: within one program the two lists describe the same operators in
/// the same order, and a program whose bytecode names more or fewer records
/// than the canonical expression owns is a correlation that cannot be made.
pub(crate) fn pair_canonical_state_slots(
    expr_id: ExprId,
    scan: &CanonicalStateSiteScan,
    bytecode_program: &BytecodeProgram,
    operator: CanonicalStateOperator,
) -> Result<Vec<(ExprId, usize)>, StatePairingError> {
    let canonical_exprs = scan.sites(operator);

    let bytecode_slots = bytecode_program
        .instructions
        .iter()
        .filter_map(|instruction| operator.bytecode_slot(instruction))
        .collect::<Vec<_>>();

    if canonical_exprs.len() != bytecode_slots.len() {
        return Err(StatePairingError::CountMismatch {
            expression: expr_id,
            operator,
            canonical: canonical_exprs.to_vec(),
            bytecode: bytecode_slots,
        });
    }

    Ok(canonical_exprs
        .iter()
        .copied()
        .zip(bytecode_slots)
        .collect())
}
