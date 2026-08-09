//! Architecture-neutral assignment-pass plan.
//!
//! Lowering resolves direct and indexed targets before machine codegen. Both
//! native backends consume this exact recursive plan, preserving source order
//! and the bounded runtime-loop contract.

#![cfg_attr(not(feature = "native"), allow(dead_code))]

use super::expr::{NativeOp, NativeProgram};
use std::collections::HashSet;
pub(crate) const MAX_ASSIGNMENT_CHUNK_OPERATIONS: usize = 12 * 1024;

#[derive(Debug)]
pub(crate) enum NativeAssignment {
    Direct {
        var_index: usize,
        program: NativeProgram,
    },
    Indexed {
        base: usize,
        len: usize,
        lower: i64,
        index: NativeProgram,
        value: NativeProgram,
    },
    Loop {
        condition: NativeProgram,
        body: Vec<NativeAssignment>,
    },
}

pub(crate) fn operation_count(assignment: &NativeAssignment) -> usize {
    match assignment {
        NativeAssignment::Direct { program, .. } => program.ops().len(),
        NativeAssignment::Indexed { index, value, .. } => {
            index.ops().len().saturating_add(value.ops().len())
        }
        NativeAssignment::Loop { condition, body } => condition.ops().len().saturating_add(
            body.iter()
                .map(operation_count)
                .fold(0_usize, usize::saturating_add),
        ),
    }
}

pub(crate) fn chunk_ranges(assignments: &[NativeAssignment]) -> Vec<std::ops::Range<usize>> {
    let mut ranges = Vec::new();
    let mut start = 0_usize;
    let mut operations = 0_usize;
    for (index, assignment) in assignments.iter().enumerate() {
        let cost = operation_count(assignment);
        if index > start && operations.saturating_add(cost) > MAX_ASSIGNMENT_CHUNK_OPERATIONS {
            ranges.push(start..index);
            start = index;
            operations = 0;
        }
        operations = operations.saturating_add(cost);
    }
    if start < assignments.len() {
        ranges.push(start..assignments.len());
    }
    ranges
}

/// Partition one already-bounded assignment slice into maximal batches that
/// can share pure SSA values without observing a stale variable load.
/// Indexed assignments and loops remain singleton barriers. A later direct
/// assignment starts a new batch whenever it reads or rewrites a variable
/// published earlier in the current batch.
pub(crate) fn shareable_batch_ranges(
    assignments: &[NativeAssignment],
) -> Vec<std::ops::Range<usize>> {
    let mut ranges = Vec::new();
    let mut start = 0_usize;
    while start < assignments.len() {
        let NativeAssignment::Direct {
            var_index: first_target,
            ..
        } = &assignments[start]
        else {
            ranges.push(start..start + 1);
            start += 1;
            continue;
        };
        let mut published = HashSet::new();
        published.insert(*first_target);
        let mut end = start + 1;
        while end < assignments.len() {
            let NativeAssignment::Direct { var_index, program } = &assignments[end] else {
                break;
            };
            if published.contains(var_index) || program_reads_any_variable(program, &published) {
                break;
            }
            published.insert(*var_index);
            end += 1;
        }
        ranges.push(start..end);
        start = end;
    }
    ranges
}

fn program_reads_any_variable(program: &NativeProgram, variables: &HashSet<usize>) -> bool {
    program.ops().iter().any(|op| match *op {
        NativeOp::LoadVariable(index) => variables.contains(&index),
        NativeOp::LoadVariableDyn { base, len, .. } => {
            let end = base.checked_add(len);
            variables
                .iter()
                .any(|index| *index >= base && end.map_or(true, |exclusive| *index < exclusive))
        }
        _ => false,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        MAX_ASSIGNMENT_CHUNK_OPERATIONS, NativeAssignment, chunk_ranges, operation_count,
        shareable_batch_ranges,
    };
    use crate::jit::expr::{NativeOp, NativeProgram};

    fn constant_assignment(var_index: usize) -> NativeAssignment {
        NativeAssignment::Direct {
            var_index,
            program: NativeProgram::from_ops_for_test(
                vec![NativeOp::Const(var_index as f64)],
                1,
                Vec::new(),
                Vec::new(),
            ),
        }
    }

    #[test]
    fn chunks_bound_operation_count_without_reordering() {
        let assignments = (0..=MAX_ASSIGNMENT_CHUNK_OPERATIONS)
            .map(constant_assignment)
            .collect::<Vec<_>>();
        assert_eq!(
            chunk_ranges(&assignments),
            vec![
                0..MAX_ASSIGNMENT_CHUNK_OPERATIONS,
                MAX_ASSIGNMENT_CHUNK_OPERATIONS..MAX_ASSIGNMENT_CHUNK_OPERATIONS + 1,
            ]
        );
    }

    #[test]
    fn recursive_loop_cost_includes_condition_and_body() {
        let assignment = NativeAssignment::Loop {
            condition: NativeProgram::from_ops_for_test(
                vec![NativeOp::Const(1.0)],
                1,
                Vec::new(),
                Vec::new(),
            ),
            body: vec![constant_assignment(0), constant_assignment(1)],
        };
        assert_eq!(operation_count(&assignment), 3);
    }

    #[test]
    fn shareable_batches_stop_at_variable_dependencies_and_control_flow() {
        let assignments = vec![
            constant_assignment(0),
            constant_assignment(1),
            NativeAssignment::Direct {
                var_index: 2,
                program: NativeProgram::from_ops_for_test(
                    vec![NativeOp::LoadVariable(0)],
                    1,
                    Vec::new(),
                    Vec::new(),
                ),
            },
            constant_assignment(3),
            NativeAssignment::Loop {
                condition: NativeProgram::from_ops_for_test(
                    vec![NativeOp::Const(0.0)],
                    1,
                    Vec::new(),
                    Vec::new(),
                ),
                body: Vec::new(),
            },
            constant_assignment(4),
        ];
        assert_eq!(
            shareable_batch_ranges(&assignments),
            vec![0..2, 2..4, 4..5, 5..6]
        );
    }

    #[test]
    fn dynamic_variable_reads_conservatively_split_overlapping_batches() {
        let assignments = vec![
            constant_assignment(5),
            NativeAssignment::Direct {
                var_index: 9,
                program: NativeProgram::from_ops_for_test(
                    vec![NativeOp::LoadVariableDyn {
                        base: 4,
                        len: 3,
                        lower: 0,
                    }],
                    1,
                    Vec::new(),
                    Vec::new(),
                ),
            },
        ];
        assert_eq!(shareable_batch_ranges(&assignments), vec![0..1, 1..2]);
    }
}
