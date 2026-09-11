//! Suspend at an analog-variable read without repeating its digital prefix.
//!
//! Timing controls already carry explicit block arguments. A sample request
//! can occur inside an expression, so it instead captures the SSA values live
//! at that instruction. These immutable lists are shared across activations;
//! the continuation owns only its cursor and captured scalar values.

use std::collections::{BTreeSet, VecDeque};

use super::*;

#[derive(Debug)]
pub(super) struct AnalogReadPlan {
    captures: HashMap<ValueId, Arc<[ValueId]>>,
}

impl AnalogReadPlan {
    fn build(function: &CfgFunction) -> Self {
        let count = function.blocks.len();
        let mut predecessors = vec![Vec::new(); count];
        for block in &function.blocks {
            for successor in activation_successors(block) {
                predecessors[usize::from(successor)].push(block.id);
            }
        }
        let mut live_in = vec![BTreeSet::new(); count];
        let mut pending: VecDeque<_> = function.blocks.iter().rev().map(|b| b.id).collect();
        let mut queued = vec![true; count];
        while let Some(id) = pending.pop_front() {
            let index = usize::from(id);
            queued[index] = false;
            let block = function.block(id);
            let mut live = live_out(block, &live_in);
            for instruction in block.instructions.iter().rev() {
                live.remove(&instruction.result);
                live.extend(function.value(instruction.result).kind.operands());
            }
            for parameter in &block.params {
                live.remove(parameter);
            }
            if live == live_in[index] {
                continue;
            }
            live_in[index] = live;
            for predecessor in &predecessors[index] {
                let p = usize::from(*predecessor);
                if !queued[p] {
                    queued[p] = true;
                    pending.push_back(*predecessor);
                }
            }
        }

        let mut captures = HashMap::new();
        for block in &function.blocks {
            let mut live = live_out(block, &live_in);
            for instruction in block.instructions.iter().rev() {
                let value = function.value(instruction.result);
                live.remove(&instruction.result);
                live.extend(value.kind.operands());
                if matches!(value.kind, CfgValueKind::DigitalAnalogVariable { .. }) {
                    let values: Vec<_> = live
                        .iter()
                        .copied()
                        .filter(|id| {
                            !matches!(
                                function.value(*id).kind,
                                CfgValueKind::FourStateConstant(_)
                                    | CfgValueKind::IntegerConstant(_)
                                    | CfgValueKind::RealConstant(_)
                            )
                        })
                        .collect();
                    captures.insert(value.id, values.into());
                }
            }
        }
        Self { captures }
    }
}

/// A real timing suspension ends this activation. Its operands and resume
/// arguments are uses here; the next activation's private values are not.
fn activation_successors(block: &super::super::cfg::CfgBlock) -> Vec<BlockId> {
    if matches!(block.terminator, CfgTerminator::Wait { .. }) {
        Vec::new()
    } else {
        block.successors()
    }
}

fn live_out(
    block: &super::super::cfg::CfgBlock,
    live_in: &[BTreeSet<ValueId>],
) -> BTreeSet<ValueId> {
    let mut live = BTreeSet::new();
    for successor in activation_successors(block) {
        live.extend(&live_in[usize::from(successor)]);
    }
    match &block.terminator {
        CfgTerminator::Jump { args, .. } => live.extend(args),
        CfgTerminator::Branch {
            condition,
            then_args,
            else_args,
            ..
        } => {
            live.insert(*condition);
            live.extend(then_args);
            live.extend(else_args);
        }
        CfgTerminator::Wait {
            wait, resume_args, ..
        } => {
            live.extend(wait.operands());
            live.extend(resume_args);
        }
        CfgTerminator::Return | CfgTerminator::Unset => {}
    }
    live
}

impl<E: DigitalEnvironment + ?Sized> Interpreter<'_, '_, E> {
    fn analog_captures(
        &mut self,
        block: BlockId,
        instruction: usize,
    ) -> Result<Arc<[ValueId]>, DigitalEvalError> {
        let function = self.function();
        let instruction_value = function
            .block(block)
            .instructions
            .get(instruction)
            .filter(|value| {
                matches!(
                    function.value(value.result).kind,
                    CfgValueKind::DigitalAnalogVariable { .. }
                )
            })
            .ok_or(DigitalEvalError::InvalidAnalogResume { block, instruction })?
            .result;
        let plan = self
            .scratch
            .analog_read_plans
            .entry(self.process.id)
            .or_insert_with(|| Arc::new(AnalogReadPlan::build(function)));
        Ok(Arc::clone(&plan.captures[&instruction_value]))
    }

    pub(super) fn suspend_analog_read(
        &mut self,
        block: BlockId,
        instruction: usize,
        probe: DigitalAnalogProbeId,
    ) -> Result<DigitalProcessOutcome, DigitalEvalError> {
        let captures = self.analog_captures(block, instruction)?;
        let mut arguments = std::mem::take(&mut self.scratch.resume);
        arguments.clear();
        for value in captures.iter() {
            match self.scalar(*value) {
                Ok(value) => arguments.push(value.into_owned()),
                Err(error) => {
                    self.scratch.recycle_resume(arguments);
                    return Err(error);
                }
            }
        }
        Ok(DigitalProcessOutcome::Suspended(DigitalSuspension {
            wait: DigitalWaitRequest::AnalogSample(probe),
            resume: DigitalResumeState {
                plan_identity: self.plan.content_identity,
                process: self.process.id,
                block,
                analog_instruction: Some(instruction),
                arguments,
            },
        }))
    }

    pub(super) fn restore_analog_read(
        &mut self,
        block: BlockId,
        instruction: usize,
    ) -> Result<(), DigitalEvalError> {
        let captures = self.analog_captures(block, instruction)?;
        if captures.len() != self.scratch.arguments.len() {
            return Err(DigitalEvalError::ArgumentArityMismatch {
                target: block,
                expected: captures.len(),
                found: self.scratch.arguments.len(),
            });
        }
        let DigitalEvalScratch {
            table, arguments, ..
        } = &mut *self.scratch;
        for (id, value) in captures.iter().zip(arguments.drain(..)) {
            table.define(*id, value);
        }
        Ok(())
    }
}
