//! Selecting a generated device's operating-point outputs.
//!
//! A module-scope variable the source describes with `desc` or `units` is an
//! output every instance reports, the way Spectre and ADS list a Verilog-A
//! instance's operating point and Xyce reads `N(m1:gm)`. The stamp computes
//! them after the load, from the values the load computed, and hands them to
//! the host only when the host asks: a report reads the evaluation the solver
//! last accepted, as a SPICE device reports its operating point, and a load
//! nobody reads outputs from pays nothing for them.

use std::collections::{HashMap, HashSet};

use super::emit::emitted_values;
use crate::canonical_ir::cfg::{CfgFunction, CfgTerminator, CfgValueKind, CfgValueType};
use crate::canonical_ir::cfg_opt::prune_with_tracking;
use crate::canonical_ir::schedule::{
    InvalidationClass, natural_loops, schedule_with_parameter_scopes,
};
use crate::canonical_ir::{
    BlockId, CanonicalIrArtifact, CanonicalValueType, HirOutputScaling, ValueId,
};
use crate::semantic::ParameterScope;

/// One reported variable, as its declaration describes it.
pub(super) struct OutputVariable {
    pub(super) name: String,
    pub(super) units: String,
    pub(super) description: String,
    pub(super) scaling: HirOutputScaling,
}

/// The variables `artifact` reports, each with its index in
/// `artifact.hir.variables`: in declaration order, numeric, one per name
/// compared as a deck compares them. A name the compiler made up has nothing
/// a deck could ask for it by.
pub(super) fn select_outputs(artifact: &CanonicalIrArtifact) -> Vec<(usize, OutputVariable)> {
    let mut selected: Vec<(usize, OutputVariable)> = Vec::new();
    for (index, variable) in artifact.hir.variables.iter().enumerate() {
        let Some(output) = &variable.output else {
            continue;
        };
        if variable.name.starts_with("__")
            || variable.name.contains('@')
            || !matches!(
                variable.value_type,
                CanonicalValueType::Real | CanonicalValueType::Integer
            )
            || selected
                .iter()
                .any(|(_, known)| known.name.eq_ignore_ascii_case(&variable.name))
        {
            continue;
        }
        selected.push((
            index,
            OutputVariable {
                name: variable.name.to_string(),
                units: output.units.to_string(),
                description: output.description.to_string(),
                scaling: output.scaling,
            },
        ));
    }
    selected
}

/// The `OUTPUTS` table: one descriptor per reported variable.
pub(super) fn descriptor_table(variables: &[OutputVariable], runtime_path: &str) -> String {
    let mut out = format!(
        "    pub const OUTPUTS: [{runtime_path}::GeneratedVerilogAOutputDescriptor; {}] = [\n",
        variables.len()
    );
    for variable in variables {
        let scaling = match variable.scaling {
            HirOutputScaling::Unscaled => "Unscaled",
            HirOutputScaling::Multiply => "Multiply",
            HirOutputScaling::Divide => "Divide",
        };
        out.push_str(&format!(
            "        {runtime_path}::GeneratedVerilogAOutputDescriptor {{ name: {:?}, units: {:?}, description: {:?}, scaling: {runtime_path}::GeneratedVerilogAOutputScaling::{scaling} }},\n",
            variable.name, variable.units, variable.description
        ));
    }
    out.push_str("    ];\n");
    out
}

/// The operating-point outputs as a slice of their own, run after the load.
pub(super) struct OutputTail {
    /// Computes the outputs, reading what the load computed as
    /// [`CfgValueKind::Outer`] leaves.
    pub(super) function: CfgFunction,
    /// One per output variable, in declaration order.
    pub(super) outputs: Vec<ValueId>,
    /// The load's value each `Outer(index)` reads, in the load's ids.
    pub(super) shared: Vec<ValueId>,
    /// Where each of `shared` sits in the load's output list, filled in when
    /// that list is laid out.
    pub(super) positions: Vec<usize>,
}

/// A load and its outputs, cut from one function optimized for both.
pub(super) struct SplitOutputs {
    pub(super) stamp: CfgFunction,
    /// The load's roots in `stamp`'s ids.
    pub(super) stamp_roots: Vec<ValueId>,
    /// Each value of the joint function in `stamp`'s ids, where it kept one.
    pub(super) stamp_ids: Vec<Option<ValueId>>,
    pub(super) tail: OutputTail,
}

/// Cut `function`, which computes the load's `roots[..output_start]` and the
/// outputs' `roots[output_start..]` together, into the load alone and a tail
/// that computes the outputs from what the load computed.
///
/// A value the load computes is the tail's input rather than its work. Both
/// slices keep the joint function's blocks and branch on the same values, so
/// wherever the tail reaches a use of such a value the load passed through its
/// definition on the same evaluation, and the value the load ends with is the
/// one the tail would compute. A packed derivative qualifies only where the
/// load's Newton pass computes it, since a coarser stage caches scalars; an
/// integer, never.
///
/// What a loop computes, the load ends with from its last iteration. That is
/// the value every use after the loop sees, and inside the loop it is right
/// only when the tail reads all of the loop's work that way: the tail then
/// retraces the last iteration, branch for branch, and leaves where the load
/// left. So what a loop works out for the outputs alone, the load works out
/// too, and the tail never runs a loop again. A loop that also needs what the
/// load cannot hand over, an integer, keeps all of its values in the tail.
///
/// `None` when the tail would still have to run something with state or an
/// effect: an integrator, a limiter, an event, a task. Only the load may run
/// those, so the caller computes the outputs inside the load instead.
pub(super) fn split_output_tail(
    function: &CfgFunction,
    roots: &[ValueId],
    output_start: usize,
    parameter_scopes: &[ParameterScope],
) -> Option<SplitOutputs> {
    let every: Vec<ValueId> = (0..function.values.len()).map(ValueId::from).collect();
    let loops = natural_loops(function);
    let mut defined_in: Vec<Option<BlockId>> = vec![None; function.values.len()];
    for block in &function.blocks {
        for value in block.params.iter().copied().chain(
            block
                .instructions
                .iter()
                .map(|instruction| instruction.result),
        ) {
            defined_in[usize::from(value)] = Some(block.id);
        }
    }
    let output_roots = &roots[output_start..];

    // Loop values the load computes only so that the tail can read them.
    let mut looped: Vec<ValueId> = Vec::new();
    let (mut tail, outputs, shared) = loop {
        let load_roots: Vec<ValueId> = roots[..output_start]
            .iter()
            .chain(&looped)
            .copied()
            .collect();
        let (stamp, stamp_roots, stamp_ids) = prune_with_tracking(function, &load_roots, &every);
        // What the load's emitted body evaluates. A condition the pruned
        // function still names but whose arms the load no longer needs is not
        // among them: handing that to the tail would make the load compute it
        // again.
        let computed = emitted_values(&stamp, &stamp_roots);
        let candidates = load_values(
            function,
            &defined_in,
            &stamp,
            &stamp_ids,
            &computed,
            parameter_scopes,
        );

        let mut running = vec![false; loops.len()];
        let (tail, outputs, shared, kept) = loop {
            let shared: Vec<ValueId> = candidates
                .iter()
                .filter(|(_, block)| {
                    !loops
                        .iter()
                        .zip(&running)
                        .any(|(blocks, running)| *running && blocks.contains(block))
                })
                .map(|(value, _)| *value)
                .collect();
            let (tail, outputs, kept) = read_from_load(function, &shared, output_roots)?;
            let mut more = false;
            for value in tail_work(&defined_in, &shared, &kept) {
                let block = defined_in[usize::from(value)];
                for (blocks, running) in loops.iter().zip(running.iter_mut()) {
                    if !*running && block.is_some_and(|block| blocks.contains(&block)) {
                        *running = true;
                        more = true;
                    }
                }
            }
            if !more {
                break (tail, outputs, shared, kept);
            }
        };
        let computable: HashSet<ValueId> = candidates.iter().map(|(value, _)| *value).collect();
        let more: Vec<ValueId> = tail_work(&defined_in, &shared, &kept)
            .filter(|value| {
                let block = defined_in[usize::from(*value)];
                block.is_some_and(|block| loops.iter().any(|blocks| blocks.contains(&block)))
                    && !computable.contains(value)
                    && !looped.contains(value)
                    && matches!(
                        function.value(*value).value_type,
                        CfgValueType::Real | CfgValueType::Boolean | CfgValueType::Lanes(_)
                    )
            })
            .collect();
        if more.is_empty() {
            break (tail, outputs, shared);
        }
        looped.extend(more);
    };
    let (mut tail, outputs) = if skip_retraced_loops(&mut tail) {
        let (tail, outputs, _) = prune_with_tracking(&tail, &outputs, &[]);
        (tail, outputs)
    } else {
        (tail, outputs)
    };

    // Number what the pruned tail still reads, in the order it reads it.
    let mut reads = Vec::new();
    for value in &mut tail.values {
        if let CfgValueKind::Outer(index) = &mut value.kind {
            let next = u32::try_from(reads.len()).ok()?;
            reads.push(shared[*index as usize]);
            *index = next;
        }
    }
    if !tail.values.iter().all(|value| runs_in_tail(&value.kind)) {
        return None;
    }
    // The load computes what it always did and what the tail reads, and
    // nothing it only would have read on the way there.
    let load_roots: Vec<ValueId> = roots[..output_start]
        .iter()
        .chain(&reads)
        .copied()
        .collect();
    let (stamp, mut stamp_roots, stamp_ids) = prune_with_tracking(function, &load_roots, &every);
    let shared = stamp_roots.split_off(output_start);
    Some(SplitOutputs {
        stamp,
        stamp_roots,
        stamp_ids,
        tail: OutputTail {
            function: tail,
            outputs,
            shared,
            positions: Vec::new(),
        },
    })
}

/// The values of `function` the load computes that the tail may read in its
/// place, with the block each is defined in. A leaf costs nothing to read
/// again, so none is among them.
fn load_values(
    function: &CfgFunction,
    defined_in: &[Option<BlockId>],
    stamp: &CfgFunction,
    stamp_ids: &[Option<ValueId>],
    computed: &[bool],
    parameter_scopes: &[ParameterScope],
) -> Vec<(ValueId, BlockId)> {
    let mut stamp_schedule = None;
    let mut values = Vec::new();
    for value in &function.values {
        let Some(block) = defined_in[usize::from(value.id)] else {
            continue;
        };
        let Some(stamp_value) = stamp_ids[usize::from(value.id)] else {
            continue;
        };
        if !computed[usize::from(stamp_value)]
            || !matches!(
                value.value_type,
                CfgValueType::Real | CfgValueType::Boolean | CfgValueType::Lanes(_)
            )
            || function.lanes_of(value.id) != stamp.lanes_of(stamp_value)
        {
            continue;
        }
        if function
            .lanes_of(value.id)
            .is_some_and(|lanes| lanes.len() > 1)
        {
            let schedule = stamp_schedule
                .get_or_insert_with(|| schedule_with_parameter_scopes(stamp, parameter_scopes));
            if schedule.class(stamp_value) != InvalidationClass::Newton {
                continue;
            }
        }
        values.push((value.id, block));
    }
    values
}

/// What the tail computes itself, in `function`'s ids: each value defined in a
/// block that the pruned tail kept rather than read from the load.
fn tail_work<'a>(
    defined_in: &'a [Option<BlockId>],
    shared: &'a [ValueId],
    kept: &'a [Option<ValueId>],
) -> impl Iterator<Item = ValueId> + 'a {
    let read: HashSet<ValueId> = shared.iter().copied().collect();
    (0..defined_in.len())
        .filter(move |value| defined_in[*value].is_some() && kept[*value].is_some())
        .map(ValueId::from)
        .filter(move |value| !read.contains(value))
}

/// Step over each loop that computes nothing and leaves to one place. All it
/// still does is branch on what the load's last iteration read, which retraces
/// that iteration to the exit the load took; with one exit, that is no work.
fn skip_retraced_loops(function: &mut CfgFunction) -> bool {
    let mut skipped = false;
    for blocks in natural_loops(function) {
        let mut exit = None;
        let mut single = true;
        for block in &blocks {
            let block = function.block(*block);
            if !block.params.is_empty() || !block.instructions.is_empty() {
                single = false;
                break;
            }
            let edges: Vec<(BlockId, bool)> = match &block.terminator {
                CfgTerminator::Jump { target, args } => vec![(*target, args.is_empty())],
                CfgTerminator::Branch {
                    then_target,
                    then_args,
                    else_target,
                    else_args,
                    ..
                } => vec![
                    (*then_target, then_args.is_empty()),
                    (*else_target, else_args.is_empty()),
                ],
                CfgTerminator::Return | CfgTerminator::Wait { .. } | CfgTerminator::Unset => {
                    single = false;
                    break;
                }
            };
            for (target, bare) in edges {
                if blocks.contains(&target) {
                    continue;
                }
                if !bare || exit.is_some_and(|exit| exit != target) {
                    single = false;
                }
                exit = Some(target);
            }
        }
        let (true, Some(exit)) = (single, exit) else {
            continue;
        };
        let enter = |target: &mut BlockId| {
            if blocks.contains(target) {
                *target = exit;
            }
        };
        for block in &mut function.blocks {
            if blocks.contains(&block.id) {
                continue;
            }
            match &mut block.terminator {
                CfgTerminator::Jump { target, .. } => enter(target),
                CfgTerminator::Branch {
                    then_target,
                    else_target,
                    ..
                } => {
                    enter(then_target);
                    enter(else_target);
                }
                CfgTerminator::Return | CfgTerminator::Wait { .. } | CfgTerminator::Unset => {}
            }
        }
        if blocks.contains(&function.entry) {
            function.entry = exit;
        }
        skipped = true;
    }
    skipped
}

/// `function` with each of `shared`'s values read from the load, pruned to
/// `outputs`: the pruned function, its outputs, and where each value of
/// `function` went in it.
fn read_from_load(
    function: &CfgFunction,
    shared: &[ValueId],
    outputs: &[ValueId],
) -> Option<(CfgFunction, Vec<ValueId>, Vec<Option<ValueId>>)> {
    let replacements: HashMap<ValueId, u32> = shared
        .iter()
        .enumerate()
        .map(|(index, value)| Some((*value, u32::try_from(index).ok()?)))
        .collect::<Option<_>>()?;
    let mut tail = function.clone();
    let removed: Vec<Vec<bool>> = tail
        .blocks
        .iter()
        .map(|block| {
            block
                .params
                .iter()
                .map(|parameter| replacements.contains_key(parameter))
                .collect()
        })
        .collect();
    let keep = |target: BlockId, args: &mut Vec<ValueId>| {
        let removed = &removed[usize::from(target)];
        let mut position = 0usize;
        args.retain(|_| {
            let keep = !removed.get(position).copied().unwrap_or(false);
            position += 1;
            keep
        });
    };
    for block in &mut tail.blocks {
        block
            .params
            .retain(|parameter| !replacements.contains_key(parameter));
        block
            .instructions
            .retain(|instruction| !replacements.contains_key(&instruction.result));
        match &mut block.terminator {
            CfgTerminator::Jump { target, args } => keep(*target, args),
            CfgTerminator::Branch {
                then_target,
                then_args,
                else_target,
                else_args,
                ..
            } => {
                keep(*then_target, then_args);
                keep(*else_target, else_args);
            }
            CfgTerminator::Return | CfgTerminator::Wait { .. } | CfgTerminator::Unset => {}
        }
    }
    for value in &mut tail.values {
        if let Some(index) = replacements.get(&value.id) {
            value.kind = CfgValueKind::Outer(*index);
        }
    }
    tail.validate().ok()?;
    let every: Vec<ValueId> = (0..function.values.len()).map(ValueId::from).collect();
    Some(prune_with_tracking(&tail, outputs, &every))
}

/// Whether the tail may evaluate `kind` a second time: a read of the
/// instance, the circuit or the analysis, or arithmetic on what it read.
fn runs_in_tail(kind: &CfgValueKind) -> bool {
    matches!(
        kind,
        CfgValueKind::RealConstant(_)
            | CfgValueKind::BooleanConstant(_)
            | CfgValueKind::IntegerConstant(_)
            | CfgValueKind::BlockParameter
            | CfgValueKind::Parameter(_)
            | CfgValueKind::ParameterGiven(_)
            | CfgValueKind::PortConnected(_)
            | CfgValueKind::EventState(_)
            | CfgValueKind::EvaluationInput(_)
            | CfgValueKind::EvaluationInputIndexed { .. }
            | CfgValueKind::Temperature
            | CfgValueKind::ThermalVoltage
            | CfgValueKind::Multiplicity
            | CfgValueKind::Time
            | CfgValueKind::Analysis(_)
            | CfgValueKind::SimParamValue(_)
            | CfgValueKind::SimParamPresent(_)
            | CfgValueKind::NodePotential(_)
            | CfgValueKind::BranchUnknownFlow(_)
            | CfgValueKind::Unary { .. }
            | CfgValueKind::Binary { .. }
            | CfgValueKind::SumProductsDiv { .. }
            | CfgValueKind::Select { .. }
            | CfgValueKind::ArrayIndex { .. }
            | CfgValueKind::IntegerArithmetic { .. }
            | CfgValueKind::IntegerBitwise { .. }
            | CfgValueKind::IntegerBitwiseNot { .. }
            | CfgValueKind::LaneSplat(_)
            | CfgValueKind::LaneWiden { .. }
            | CfgValueKind::LaneBinary { .. }
            | CfgValueKind::LaneScalar { .. }
            | CfgValueKind::LaneSumProductsDiv { .. }
            | CfgValueKind::LaneExtract { .. }
            | CfgValueKind::Outer(_)
    )
}
