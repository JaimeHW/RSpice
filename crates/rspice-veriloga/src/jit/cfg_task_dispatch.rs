//! Keep task-only inlined computations at their source position, with the same
//! dispatch permission as the calls. Shared numerical work remains unconditional.

use crate::canonical_ir::cfg::{
    CfgBlock, CfgFunction, CfgInstruction, CfgTerminator, CfgValue, CfgValueKind, CfgValueType,
    is_leaf_kind,
};
use crate::canonical_ir::cfg_opt::prune_to_outputs_with_tracking;
use crate::canonical_ir::{BlockId, ValueId};
use std::collections::{HashMap, HashSet};

pub(super) fn guard_task_only_computations(function: &mut CfgFunction, outputs: &[ValueId]) {
    let mut required = outputs.to_vec();
    // Analog-history updates retain their established dispatch/placement contract.
    required.extend(
        function
            .values
            .iter()
            .filter_map(|value| value.kind.state_site().map(|_| value.id)),
    );
    let bypasses = guard_unobserved_loops(function, &required);
    let mut observation = function.clone();
    for (header, terminator) in bypasses {
        observation.blocks[usize::from(header)].terminator = terminator;
    }
    let tracked: Vec<_> = function.values.iter().map(|value| value.id).collect();
    let (_, _, live) = prune_to_outputs_with_tracking(&observation, &required, &tracked);
    let reachable = observation_reachable(function);
    let guarded: Vec<_> = function
        .values
        .iter()
        .map(|value| {
            live[usize::from(value.id)].is_none()
                && matches!(value.value_type, CfgValueType::Real | CfgValueType::Boolean)
                && !is_leaf_kind(&value.kind)
        })
        .collect();
    if !function.blocks.iter().any(|block| {
        reachable[usize::from(block.id)]
            && block
                .instructions
                .iter()
                .any(|instruction| guarded[usize::from(instruction.result)])
    }) {
        return;
    }
    let enabled = new_value(
        function,
        CfgValueType::Boolean,
        CfgValueKind::AnalogTasksEnabled,
    );
    let zero = new_value(
        function,
        CfgValueType::Real,
        CfgValueKind::RealConstant(0.0),
    );
    let false_value = new_value(
        function,
        CfgValueType::Boolean,
        CfgValueKind::BooleanConstant(false),
    );
    for (block_index, &is_reachable) in reachable.iter().enumerate() {
        if !is_reachable {
            continue;
        }
        let instructions = std::mem::take(&mut function.blocks[block_index].instructions);
        let terminator = std::mem::replace(
            &mut function.blocks[block_index].terminator,
            CfgTerminator::Unset,
        );
        let mut current = BlockId::from(block_index);
        let mut position = 0;
        while position < instructions.len() {
            if !guarded[usize::from(instructions[position].result)] {
                function.blocks[usize::from(current)]
                    .instructions
                    .push(instructions[position].clone());
                position += 1;
                continue;
            }
            let start = position;
            while position < instructions.len()
                && guarded[usize::from(instructions[position].result)]
            {
                position += 1;
            }
            let group = &instructions[start..position];
            let body = BlockId::from(function.blocks.len());
            let join = BlockId::from(function.blocks.len() + 1);
            let mut mapping = HashMap::with_capacity(group.len());
            for instruction in group {
                let original = instruction.result;
                let mut value = function.value(original).clone();
                value.id = ValueId::from(function.values.len());
                mapping.insert(original, value.id);
                function.values.push(value);
            }
            let mut body_instructions = Vec::with_capacity(group.len());
            let mut values = Vec::with_capacity(group.len());
            let mut zeros = Vec::with_capacity(group.len());
            let params: Vec<_> = group.iter().map(|instruction| instruction.result).collect();
            for &original in &params {
                let result = mapping[&original];
                function.values[usize::from(result)]
                    .kind
                    .map_operands(|operand| mapping.get(&operand).copied().unwrap_or(operand));
                let value = &mut function.values[usize::from(original)];
                value.kind = CfgValueKind::BlockParameter;
                zeros.push(if value.value_type == CfgValueType::Boolean {
                    false_value
                } else {
                    zero
                });
                values.push(result);
                body_instructions.push(CfgInstruction { result });
            }
            function.blocks[usize::from(current)].terminator = CfgTerminator::Branch {
                condition: enabled,
                then_target: body,
                then_args: Vec::new(),
                else_target: join,
                else_args: zeros,
            };
            function.blocks.push(CfgBlock {
                id: body,
                params: Vec::new(),
                instructions: body_instructions,
                terminator: CfgTerminator::Jump {
                    target: join,
                    args: values,
                },
            });
            function.blocks.push(CfgBlock {
                id: join,
                params,
                instructions: Vec::new(),
                terminator: CfgTerminator::Unset,
            });
            current = join;
        }
        function.blocks[usize::from(current)].terminator = terminator;
    }
}

/// Bypass only loops with no required data/history and one identical exit edge.
/// The original body and counter still execute unchanged on numerical dispatch.
fn guard_unobserved_loops(
    function: &mut CfgFunction,
    required: &[ValueId],
) -> Vec<(BlockId, CfgTerminator)> {
    let mut loops = crate::canonical_ir::schedule::natural_loops(function);
    loops.sort_by_key(|body| std::cmp::Reverse(body.len()));
    let live = data_dependencies(function, required);
    let mut predecessors = vec![Vec::new(); function.blocks.len()];
    for block in &function.blocks {
        for successor in block.successors() {
            predecessors[usize::from(successor)].push(block.id);
        }
    }
    let mut covered = HashSet::new();
    let mut candidates = Vec::new();
    for body in loops {
        if !body.is_disjoint(&covered)
            || body.iter().any(|&id| {
                let block = function.block(id);
                block
                    .params
                    .iter()
                    .chain(block.instructions.iter().map(|i| &i.result))
                    .any(|value| live[usize::from(*value)])
            })
        {
            continue;
        }
        let headers: Vec<_> = function
            .blocks
            .iter()
            .filter(|block| {
                body.contains(&block.id)
                    && (block.id == function.entry
                        || predecessors[usize::from(block.id)]
                            .iter()
                            .any(|id| !body.contains(id)))
            })
            .map(|block| block.id)
            .collect();
        let [header] = headers.as_slice() else {
            continue;
        };
        let exits: Vec<_> = function
            .blocks
            .iter()
            .filter(|block| body.contains(&block.id))
            .flat_map(|block| edges(&block.terminator))
            .filter(|(target, _)| !body.contains(target))
            .map(|(target, args)| (target, args.to_vec()))
            .collect();
        let Some((target, args)) = exits.first() else {
            continue;
        };
        if exits
            .iter()
            .any(|(candidate, values)| candidate != target || values != args)
        {
            continue;
        }
        if args.iter().any(|&arg| {
            !matches!(
                function.value(arg).value_type,
                CfgValueType::Real | CfgValueType::Boolean
            )
        }) {
            continue;
        }
        covered.extend(body.iter().copied());
        candidates.push((*header, *target, args.clone(), body));
    }
    if candidates.is_empty() {
        return Vec::new();
    }
    let enabled = new_value(
        function,
        CfgValueType::Boolean,
        CfgValueKind::AnalogTasksEnabled,
    );
    let zero = new_value(
        function,
        CfgValueType::Real,
        CfgValueKind::RealConstant(0.0),
    );
    let false_value = new_value(
        function,
        CfgValueType::Boolean,
        CfgValueKind::BooleanConstant(false),
    );
    let mut bypasses = Vec::new();
    for (header, target, mut args, loop_body) in candidates {
        for arg in &mut args {
            if !live[usize::from(*arg)] {
                *arg = if function.value(*arg).value_type == CfgValueType::Boolean {
                    false_value
                } else {
                    zero
                };
            }
        }
        // Enter through a preheader, retaining the original loop's single
        // back-edge target and its structured exit. An exit guard inside the
        // header would introduce a second loop-exit decision.
        let dispatch = BlockId::from(function.blocks.len());
        let mut params = Vec::new();
        for original in function.block(header).params.clone() {
            params.push(new_value(
                function,
                function.value(original).value_type,
                CfgValueKind::BlockParameter,
            ));
        }
        for block in &mut function.blocks {
            if loop_body.contains(&block.id) {
                continue;
            }
            match &mut block.terminator {
                CfgTerminator::Jump { target, .. } => {
                    if *target == header {
                        *target = dispatch;
                    }
                }
                CfgTerminator::Branch {
                    then_target,
                    else_target,
                    ..
                } => {
                    if *then_target == header {
                        *then_target = dispatch;
                    }
                    if *else_target == header {
                        *else_target = dispatch;
                    }
                }
                _ => {}
            }
        }
        if function.entry == header {
            function.entry = dispatch;
        }
        function.blocks.push(CfgBlock {
            id: dispatch,
            params: params.clone(),
            instructions: Vec::new(),
            terminator: CfgTerminator::Branch {
                condition: enabled,
                then_target: header,
                then_args: params,
                else_target: target,
                else_args: args.clone(),
            },
        });
        bypasses.push((dispatch, CfgTerminator::Jump { target, args }));
    }
    bypasses
}

fn edges(terminator: &CfgTerminator) -> Vec<(BlockId, &[ValueId])> {
    match terminator {
        CfgTerminator::Jump { target, args } => vec![(*target, args)],
        CfgTerminator::Branch {
            then_target,
            then_args,
            else_target,
            else_args,
            ..
        } => vec![(*then_target, then_args), (*else_target, else_args)],
        _ => Vec::new(),
    }
}

/// Value dependence through loop merges, without making loop conditions roots.
/// Bounds checks retain the ordinary CFG's mandatory-check policy.
fn data_dependencies(function: &CfgFunction, roots: &[ValueId]) -> Vec<bool> {
    let mut incoming = vec![Vec::new(); function.values.len()];
    for block in &function.blocks {
        for (target, args) in edges(&block.terminator) {
            for (&param, &arg) in function.block(target).params.iter().zip(args) {
                incoming[usize::from(param)].push(arg);
            }
        }
    }
    let mut pending = roots.to_vec();
    pending.extend(function.values.iter().filter_map(|value| {
        matches!(value.kind, CfgValueKind::ArrayIndex { .. }).then_some(value.id)
    }));
    let mut live = vec![false; function.values.len()];
    while let Some(value) = pending.pop() {
        if std::mem::replace(&mut live[usize::from(value)], true) {
            continue;
        }
        pending.extend(function.value(value).kind.operands());
        pending.extend_from_slice(&incoming[usize::from(value)]);
    }
    live
}

fn new_value(function: &mut CfgFunction, value_type: CfgValueType, kind: CfgValueKind) -> ValueId {
    let id = ValueId::from(function.values.len());
    function.values.push(CfgValue {
        id,
        value_type,
        kind,
    });
    id
}

/// Existing task bodies are already unreachable when the journal is absent.
/// Treat every other predicate as unknown; this analysis must be conservative.
fn observation_reachable(function: &CfgFunction) -> Vec<bool> {
    let mut reachable = vec![false; function.blocks.len()];
    let mut pending = vec![function.entry];
    while let Some(block) = pending.pop() {
        if std::mem::replace(&mut reachable[usize::from(block)], true) {
            continue;
        }
        let block = function.block(block);
        match &block.terminator {
            CfgTerminator::Branch {
                condition,
                else_target,
                ..
            } if matches!(
                function.value(*condition).kind,
                CfgValueKind::AnalogTasksEnabled
            ) =>
            {
                pending.push(*else_target);
            }
            _ => pending.extend(block.successors()),
        }
    }
    reachable
}
