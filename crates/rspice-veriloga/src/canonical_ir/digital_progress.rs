//! Whether a thread standing at a block can still make progress.
//!
//! A hang detector's question, answered from the graph rather than by
//! counting. `forever n = n + 1;` never suspends, and `always #0 x = ~x;`
//! suspends only into the same time step; both run until a limit sized for
//! legal programs stops them, which takes seconds. Once a thread stands in a
//! part of its graph from which no path reaches a way out, the limit has
//! nothing left to measure: the graph already says the thread can never let
//! time advance.
//!
//! The answer is conservative. A block counts as a way out when anything in it
//! could end the thread's run or suspend it for longer than a delta cycle: a
//! `Return`, any timing control but a constant `#0`, a task enable, a fork, a
//! `disable`, a system task (it may be `$finish`), a function call (its body may
//! call one) and an analog read (it may wait for a sample). A branch on a
//! constant condition takes only the edge it selects, because that is the only
//! one it can ever take; any other branch keeps both.

use super::cfg::{CfgFunction, CfgTerminator, CfgValueKind, DigitalWait};
use super::digital_value::truth;
use super::ids::{BlockId, ValueId};
use crate::four_state::FourStateBit;

/// What counts as progress for [`blocks_without_progress`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigitalProgress {
    /// Any suspension. A thread that cannot reach one runs forever inside one
    /// activation.
    Suspends,
    /// Any suspension but a constant `#0`, which resumes in the inactive
    /// region of the same time step (IEEE 1364-2005 section 9.7.7). A thread
    /// that cannot reach one keeps its time step open forever.
    AdvancesTime,
}

/// For every block of `function`, indexed by block, whether a thread standing
/// at it can never again reach `progress`.
pub fn blocks_without_progress(function: &CfgFunction, progress: DigitalProgress) -> Vec<bool> {
    let count = function.blocks.len();
    let mut predecessors = vec![Vec::new(); count];
    let mut reaches = vec![false; count];
    let mut pending = Vec::new();
    for (index, block) in function.blocks.iter().enumerate() {
        let escapes = block.instructions.iter().any(|instruction| {
            matches!(
                function.value(instruction.result).kind,
                CfgValueKind::DigitalSystemTask { .. }
                    | CfgValueKind::DigitalCall { .. }
                    | CfgValueKind::DigitalAnalogVariable { .. }
            )
        });
        let successors: [Option<BlockId>; 2] = match &block.terminator {
            _ if escapes => [None, None],
            CfgTerminator::Jump { target, .. } => [Some(*target), None],
            CfgTerminator::Branch {
                condition,
                then_target,
                else_target,
                ..
            } => match constant_truth(function, *condition) {
                Some(FourStateBit::One) => [Some(*then_target), None],
                Some(_) => [Some(*else_target), None],
                None => [Some(*then_target), Some(*else_target)],
            },
            CfgTerminator::Wait {
                wait: DigitalWait::Delay(delay),
                resume,
                ..
            } if progress == DigitalProgress::AdvancesTime
                && is_constant_zero(function, *delay) =>
            {
                [Some(*resume), None]
            }
            CfgTerminator::Wait { .. } | CfgTerminator::Return | CfgTerminator::Unset => {
                reaches[index] = true;
                pending.push(index);
                [None, None]
            }
        };
        if escapes {
            reaches[index] = true;
            pending.push(index);
        }
        for successor in successors.into_iter().flatten() {
            if let Some(list) = predecessors.get_mut(usize::from(successor)) {
                list.push(index);
            }
        }
    }
    while let Some(index) = pending.pop() {
        for &predecessor in &predecessors[index] {
            if !reaches[predecessor] {
                reaches[predecessor] = true;
                pending.push(predecessor);
            }
        }
    }
    reaches.into_iter().map(|reaches| !reaches).collect()
}

/// A condition's truth when it is a constant, which a branch can then only
/// ever resolve one way (IEEE 1364-2005 section 9.4: only a known one is true).
/// A wide condition reaches the branch as `!!x` (`while (1)`), so a logical
/// negation of a constant is one too.
fn constant_truth(function: &CfgFunction, value: ValueId) -> Option<FourStateBit> {
    match &function.value(value).kind {
        CfgValueKind::BooleanConstant(value) => Some(known(*value)),
        CfgValueKind::IntegerConstant(value) => Some(known(*value != 0)),
        CfgValueKind::FourStateConstant(value) => Some(truth(value)),
        CfgValueKind::DigitalLogicalNot { input } => match constant_truth(function, *input)? {
            FourStateBit::One => Some(FourStateBit::Zero),
            FourStateBit::Zero => Some(FourStateBit::One),
            _ => Some(FourStateBit::Unknown),
        },
        _ => None,
    }
}

fn known(value: bool) -> FourStateBit {
    if value {
        FourStateBit::One
    } else {
        FourStateBit::Zero
    }
}

/// Whether a delay operand is the constant zero, before or after its
/// conversion to ticks.
fn is_constant_zero(function: &CfgFunction, value: ValueId) -> bool {
    match &function.value(value).kind {
        CfgValueKind::DigitalDelayTicks { input, .. } => is_constant_zero(function, *input),
        CfgValueKind::IntegerConstant(value) => *value == 0,
        CfgValueKind::RealConstant(value) => *value == 0.0,
        CfgValueKind::FourStateConstant(value) => truth(value) == FourStateBit::Zero,
        _ => false,
    }
}
