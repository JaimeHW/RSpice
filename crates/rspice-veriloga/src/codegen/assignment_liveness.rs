//! Variable dependencies shared by native planning and small-signal replay.

use super::{AssignmentStep, BytecodeProgram, Instruction};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum AssignmentEffects {
    IncludeTasks,
    SkipTasks,
}

/// Close the root set over assignment, dynamic-index, and loop dependencies.
/// This conservative fixed point retains all reaching definitions and loop
/// iterations without copying the assignment programs.
pub(crate) fn propagate_live_assignment_slots(
    steps: &[AssignmentStep],
    live: &mut [bool],
    effects: AssignmentEffects,
) {
    loop {
        let mut changed = false;
        propagate_assignment_liveness(steps, live, &mut changed, effects);
        if !changed {
            break;
        }
    }
}

fn propagate_assignment_liveness(
    steps: &[AssignmentStep],
    live: &mut [bool],
    changed: &mut bool,
    effects: AssignmentEffects,
) {
    for step in steps.iter().rev() {
        match step {
            AssignmentStep::Initialization { .. } => {}
            AssignmentStep::Task(task) => {
                if effects == AssignmentEffects::IncludeTasks {
                    for program in task.expressions() {
                        mark_program_variable_reads_changed(program, live, changed);
                    }
                }
            }
            AssignmentStep::Assign(assignment) => {
                if live.get(assignment.var_index) == Some(&true) {
                    mark_program_variable_reads_changed(&assignment.program, live, changed);
                }
            }
            AssignmentStep::AssignIndexed {
                base,
                len,
                index,
                value,
                ..
            } => {
                if assignment_range_live(*base, *len, live) {
                    mark_program_variable_reads_changed(index, live, changed);
                    mark_program_variable_reads_changed(value, live, changed);
                }
            }
            AssignmentStep::Loop { condition, body } => {
                propagate_assignment_liveness(body, live, changed, effects);
                if body
                    .iter()
                    .any(|step| assignment_step_is_live(step, live, effects))
                {
                    mark_program_variable_reads_changed(condition, live, changed);
                }
            }
        }
    }
}

pub(crate) fn assignment_step_is_live(
    step: &AssignmentStep,
    live: &[bool],
    effects: AssignmentEffects,
) -> bool {
    match step {
        AssignmentStep::Initialization { .. } => false,
        AssignmentStep::Task(_) => effects == AssignmentEffects::IncludeTasks,
        AssignmentStep::Assign(assignment) => live.get(assignment.var_index) == Some(&true),
        AssignmentStep::AssignIndexed { base, len, .. } => assignment_range_live(*base, *len, live),
        AssignmentStep::Loop { body, .. } => body
            .iter()
            .any(|step| assignment_step_is_live(step, live, effects)),
    }
}

pub(crate) fn assignment_range_live(base: usize, len: usize, live: &[bool]) -> bool {
    base.checked_add(len)
        .and_then(|end| live.get(base..end))
        .is_some_and(|range| range.iter().any(|slot| *slot))
}

pub(crate) fn mark_program_variable_reads(program: &BytecodeProgram, live: &mut [bool]) {
    let mut changed = false;
    mark_program_variable_reads_changed(program, live, &mut changed);
}

fn mark_program_variable_reads_changed(
    program: &BytecodeProgram,
    live: &mut [bool],
    changed: &mut bool,
) {
    for instruction in &program.instructions {
        match *instruction {
            Instruction::PushVariable(index) => mark_variable_live(index, live, changed),
            Instruction::PushVariableDyn { base, len, .. } => {
                if let Some(end) = base.checked_add(len) {
                    for index in base..end.min(live.len()) {
                        mark_variable_live(index, live, changed);
                    }
                }
            }
            _ => {}
        }
    }
}

fn mark_variable_live(index: usize, live: &mut [bool], changed: &mut bool) {
    if let Some(slot) = live.get_mut(index)
        && !*slot
    {
        *slot = true;
        *changed = true;
    }
}
