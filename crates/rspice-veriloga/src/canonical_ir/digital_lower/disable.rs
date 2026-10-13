//! `disable` (IEEE 1364-2005 section 10.3, design P2 §4.1).
//!
//! A named block some `disable` names gets an exit block. Inside it, every
//! resume block, which is every place a thread can wait, first tests whether
//! the thread resumed to leave the block and branches to the exit if so. The
//! kernel unwinds a thread by resuming it where it waits with that test
//! armed, so the way out of a block is ordinary control flow and every value
//! live after the block arrives the way it would at the block's end.
//!
//! A thread inside a disabled task resumes at the enable's resume block
//! instead, which copies nothing back; a task that disables itself returns,
//! copying its outputs as its end would.
//!
//! The statement hands its target to the kernel and goes on without yielding.
//! A block of a process or function body with no `fork` in it is executed
//! only by the thread that is in it, so a `disable` inside it jumps straight
//! to the exit.

use super::super::cfg::DigitalDisableTarget;
use super::super::digital::{
    CfgDigitalProcess, DigitalRoutine, DigitalRoutineId, DigitalRoutineKind, DigitalScope,
    DigitalScopeId, DigitalScopeKind,
};
use super::*;
use crate::ast::DigitalDisable;
use std::cell::RefCell;
use std::sync::Arc;

/// The `disable` state of one body's lowering.
#[derive(Default)]
pub(super) struct ThreadTables {
    /// Every name a `disable` of the design targets.
    disabled: Arc<HashSet<SmolStr>>,
    /// The disabled named blocks around the statement being lowered,
    /// innermost last.
    unwind: Vec<UnwindScope>,
    /// The routine being lowered.
    pub routine: Option<RoutineExit>,
    /// `disable`s of a block outside their own body.
    pub pending: Vec<PendingDisable>,
}

impl ThreadTables {
    pub fn new(disabled: Arc<HashSet<SmolStr>>) -> Self {
        Self {
            disabled,
            ..Self::default()
        }
    }
}

/// A disabled named block around the statement being lowered.
pub(super) struct UnwindScope {
    scope: DigitalScopeId,
    name: SmolStr,
    exit: BlockId,
    /// Only the thread inside the block can execute a `disable` of it there.
    direct: bool,
}

/// The routine being lowered, and the block a task that disables itself
/// returns from once one does.
pub(super) struct RoutineExit {
    pub routine: DigitalRoutineId,
    pub function: bool,
    pub exit: Option<BlockId>,
}

/// The body a pending `disable` is in.
#[derive(Clone, Copy)]
pub(super) enum DisableOwner {
    Process(DigitalProcessId),
    Routine(DigitalRoutineId),
}

/// A `disable` of a block outside its own body: the block whose wait names
/// it, and the scope paths it may denote, nearest first.
pub(super) struct PendingDisable {
    pub owner: Option<DisableOwner>,
    block: BlockId,
    candidates: Vec<SmolStr>,
    name: SmolStr,
    span: Span,
}

/// Every name a `disable` in these bodies targets, by its last component.
pub(super) fn disabled_names<'a>(
    bodies: impl IntoIterator<Item = &'a DigitalStatement>,
) -> HashSet<SmolStr> {
    let names = RefCell::new(HashSet::new());
    for body in bodies {
        super::local_storage::any_statement(body, &|statement| {
            if let DigitalStatement::Disable(disable) = statement {
                names.borrow_mut().insert(last_component(&disable.target));
            }
            false
        });
    }
    names.into_inner()
}

fn last_component(name: &SmolStr) -> SmolStr {
    name.rsplit('.')
        .next()
        .map_or_else(|| name.clone(), SmolStr::from)
}

/// Point every pending `disable` at the block it names, now that every
/// scope of the design exists.
pub(super) fn resolve_disables(
    processes: &mut [CfgDigitalProcess],
    routines: &mut [DigitalRoutine],
    scopes: &[DigitalScope],
    pending: Vec<PendingDisable>,
    diagnostics: &mut Vec<DigitalLoweringDiagnostic>,
) {
    if pending.is_empty() {
        return;
    }
    let blocks: HashMap<&str, DigitalScopeId> = scopes
        .iter()
        .filter(|scope| scope.kind == DigitalScopeKind::Block)
        .map(|scope| (scope.path.as_str(), scope.id))
        .collect();
    for disable in pending {
        let Some(scope) = disable
            .candidates
            .iter()
            .find_map(|path| blocks.get(path.as_str()).copied())
        else {
            diagnostics.push(DigitalLoweringDiagnostic::refusal(
                format!(
                    "`disable {}` names no named block or task in scope",
                    disable.name
                ),
                SourceSpanRef::from(disable.span),
            ));
            continue;
        };
        let function = match disable.owner {
            Some(DisableOwner::Process(id)) => processes
                .iter_mut()
                .find(|process| process.id == id)
                .map(|process| &mut process.function),
            Some(DisableOwner::Routine(id)) => routines
                .iter_mut()
                .find(|routine| routine.id == id)
                .map(|routine| &mut routine.function),
            None => None,
        };
        let wait =
            function.and_then(|function| function.blocks.get_mut(usize::from(disable.block)));
        match wait.map(|block| &mut block.terminator) {
            Some(CfgTerminator::Wait {
                wait: DigitalWait::Disable { target },
                ..
            }) => *target = DigitalDisableTarget::Block(scope),
            _ => diagnostics.push(DigitalLoweringDiagnostic::invariant(
                "a `disable` awaiting its block has no wait",
                SourceSpanRef::from(disable.span),
            )),
        }
    }
}

impl ProcessLowerer<'_> {
    /// Open a named block's way out when some `disable` names it; `forks`
    /// says whether its statements contain a `fork`. Returns whether it did,
    /// in which case [`Self::close_unwind`] closes it.
    pub(super) fn open_unwind(
        &mut self,
        scope: Option<DigitalScopeId>,
        name: &SmolStr,
        forks: bool,
    ) -> bool {
        let Some(scope) = scope.filter(|_| self.calls.threads.disabled.contains(name)) else {
            return false;
        };
        let in_task = self
            .calls
            .threads
            .routine
            .as_ref()
            .is_some_and(|routine| !routine.function);
        let exit = self.builder.create_block();
        self.calls.threads.unwind.push(UnwindScope {
            scope,
            name: name.clone(),
            exit,
            direct: !in_task && !forks,
        });
        true
    }

    /// Set aside the disabled blocks around a `fork`'s branches.
    pub(super) fn hide_unwinding(&mut self) -> Vec<UnwindScope> {
        std::mem::take(&mut self.calls.threads.unwind)
    }

    pub(super) fn restore_unwinding(&mut self, around: Vec<UnwindScope>) {
        self.calls.threads.unwind = around;
    }

    /// Close the innermost way out: the block's end and every unwinding of
    /// it meet at its exit, where execution goes on.
    pub(super) fn close_unwind(&mut self, end: BlockId) -> BlockId {
        let Some(scope) = self.calls.threads.unwind.pop() else {
            return end;
        };
        self.builder.set_terminator(
            end,
            CfgTerminator::Jump {
                target: scope.exit,
                args: Vec::new(),
            },
        );
        self.builder.seal_block(scope.exit);
        scope.exit
    }

    /// Start a resume block with the unwinding test of every disabled block
    /// around it, all computed in the resume block itself (that is what
    /// tells the kernel the thread is inside them) and taken innermost
    /// first. Returns the block execution goes on in.
    pub(super) fn unwind_tests(&mut self, resume: BlockId) -> BlockId {
        let scopes: Vec<_> = self
            .calls
            .threads
            .unwind
            .iter()
            .rev()
            .map(|scope| (scope.scope, scope.exit))
            .collect();
        let tests: Vec<_> = scopes
            .iter()
            .map(|(scope, _)| {
                self.builder.push(
                    resume,
                    CfgValueType::FourState { width: 1 },
                    CfgValueKind::DigitalUnwinding { scope: *scope },
                )
            })
            .collect();
        let mut current = resume;
        for ((_, exit), test) in scopes.into_iter().zip(tests) {
            let next = self.builder.create_block();
            self.builder.set_terminator(
                current,
                CfgTerminator::Branch {
                    condition: test,
                    then_target: exit,
                    then_args: Vec::new(),
                    else_target: next,
                    else_args: Vec::new(),
                },
            );
            self.builder.seal_block(next);
            current = next;
        }
        current
    }

    /// `disable name` (section 10.3): a named block around the statement, a
    /// task, or a named block elsewhere, in that order of lookup.
    pub(super) fn disable_statement(
        &mut self,
        block: BlockId,
        disable: &DigitalDisable,
    ) -> BlockId {
        let in_function = self
            .calls
            .threads
            .routine
            .as_ref()
            .is_some_and(|routine| routine.function);
        if let Some(scope) = self
            .calls
            .threads
            .unwind
            .iter()
            .rev()
            .find(|scope| scope.name == disable.target)
        {
            let (scope, exit, direct) = (scope.scope, scope.exit, scope.direct);
            if direct || in_function {
                return self.leave_to(block, exit);
            }
            return self.disable_wait(block, DigitalDisableTarget::Block(scope));
        }
        if let Some(signature) = self.user_routine(&disable.target).cloned() {
            if signature.kind != DigitalRoutineKind::Task {
                self.error(
                    format!(
                        "`{}` is a function, which cannot be disabled",
                        disable.target
                    ),
                    disable.span,
                );
                return block;
            }
            if in_function {
                self.error("a function cannot disable a task", disable.span);
                return block;
            }
            let resume = self.disable_wait(block, DigitalDisableTarget::Task(signature.id));
            // A task that disables itself returns (design P2 §4.1).
            let Some(routine) = self
                .calls
                .threads
                .routine
                .as_ref()
                .filter(|routine| routine.routine == signature.id)
            else {
                return resume;
            };
            let exit = match routine.exit {
                Some(exit) => exit,
                None => {
                    let exit = self.builder.create_block();
                    if let Some(routine) = self.calls.threads.routine.as_mut() {
                        routine.exit = Some(exit);
                    }
                    exit
                }
            };
            return self.leave_to(resume, exit);
        }
        if in_function {
            self.error(
                "a function can disable only a named block inside it",
                disable.span,
            );
            return block;
        }
        // A block of another body: its scope is numbered when that body is
        // lowered, so the wait names it once every body has been.
        let mut candidates: Vec<SmolStr> = self
            .calls
            .stack
            .iter()
            .rev()
            .map(|frame| format!("{}.{}", frame.path, disable.target).into())
            .collect();
        candidates.push(disable.target.clone());
        let at = self.builder.tail(block);
        let resume =
            self.disable_wait(block, DigitalDisableTarget::Block(DigitalScopeId(u32::MAX)));
        self.calls.threads.pending.push(PendingDisable {
            owner: None,
            block: at,
            candidates,
            name: disable.target.clone(),
            span: disable.span,
        });
        resume
    }

    fn disable_wait(&mut self, block: BlockId, target: DigitalDisableTarget) -> BlockId {
        self.suspend(block, DigitalWait::Disable { target }, &mut [])
    }

    /// Leave for `exit` from `block`. The branch is always taken; the block
    /// after it keeps the statements that follow, which never run.
    fn leave_to(&mut self, block: BlockId, exit: BlockId) -> BlockId {
        let taken = self.builder.push_leaf(
            CfgValueType::FourState { width: 1 },
            CfgValueKind::FourStateConstant(FourStateValue::from_u64(1, 1)),
        );
        let after = self.builder.create_block();
        self.builder.set_terminator(
            block,
            CfgTerminator::Branch {
                condition: taken,
                then_target: exit,
                then_args: Vec::new(),
                else_target: after,
                else_args: Vec::new(),
            },
        );
        self.builder.seal_block(after);
        after
    }
}
