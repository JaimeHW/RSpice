//! When each value has to be recomputed.
//!
//! A compact model is evaluated many times per operating point and many
//! operating points per analysis, but most of what it computes does not change
//! that often. Binning the parameters, folding in the temperature, and building
//! the geometry coefficients depend on the model card and nothing else; a BSIM
//! card's `if (!$param_given(x)) x = ...` prologue runs hundreds of statements
//! that are the same on the ten-thousandth Newton iteration as on the first.
//!
//! This pass says which is which, and the emitter turns that into separate
//! functions with separate caches. That is the *only* reason to split a body —
//! never because it got large. Splitting for compile time was measured and
//! costs 52% runtime (`tools/perf-probes/archive/split`).
//!
//! ## Control dependence, not just data
//!
//! A value's class is at least the class of everything it reads. It is also at
//! least the class of every branch it is control-dependent on, and that second
//! half is the one that is easy to get wrong: an expression built only from
//! parameters, sitting inside `if (V(a,b) > 0)`, is *not* instance-static.
//! Hoisting it into the per-card function would evaluate it on paths the model
//! never takes, which is how a guarded division by a parameter that may be zero
//! becomes an infinity in code that used to be unreachable. The guard is there
//! because the model author put it there.
//!
//! Control dependence is the post-dominance frontier — Ferrante, Ottenstein and
//! Warren's construction — and the two halves are solved together as a fixed
//! point, because a branch's condition is itself a value with a class.

use std::collections::{BTreeMap, HashMap, HashSet};

use crate::semantic::ParameterScope;

use super::cfg::{
    CfgBlock, CfgFunction, CfgInstruction, CfgTerminator, CfgValidationError, CfgValue,
    CfgValueKind,
};
use super::{BlockId, ParamId, ValueId};

/// How often a value has to be recomputed, coarsest first.
///
/// Deliberately fewer classes than the level this replaces distinguishes. These
/// are the ones a *body* can be split on: the analysis-specific classes it also
/// had — AC frequency, noise frequency, operating-point report — are properties
/// of which output is being asked for, not of when a value goes stale, and
/// mixing the two is what made the old schedule hard to reason about.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InvalidationClass {
    /// Constants and model-card parameters. Recomputed when a model card is
    /// created or changed, before any per-device geometry is applied.
    Model,
    /// Per-device parameters and the instance multiplier. Recomputed when an
    /// instance is bound to a model card or its geometry changes.
    Instance,
    /// Adds the temperature and the thermal voltage.
    Temperature,
    /// Adds the time point and the integration coefficient.
    Timestep,
    /// Adds the unknowns. Recomputed every Newton iteration, which is what all
    /// the others exist to keep work out of.
    Newton,
}

impl InvalidationClass {
    pub const ALL: [Self; 5] = [
        Self::Model,
        Self::Instance,
        Self::Temperature,
        Self::Timestep,
        Self::Newton,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Model => "model",
            Self::Instance => "instance",
            Self::Temperature => "temperature",
            Self::Timestep => "timestep",
            Self::Newton => "newton",
        }
    }

    fn join(self, other: Self) -> Self {
        self.max(other)
    }
}

/// The class of every value and every block.
#[derive(Debug, Clone, PartialEq)]
pub struct Schedule {
    /// Indexed by [`ValueId`].
    pub values: Vec<InvalidationClass>,
    /// The class a block's guards impose on everything inside it. Indexed by
    /// [`BlockId`].
    pub blocks: Vec<InvalidationClass>,
}

impl Schedule {
    pub fn class(&self, value: ValueId) -> InvalidationClass {
        self.values[usize::from(value)]
    }

    /// How many values fall in each class, coarsest first.
    pub fn census(&self) -> [usize; 5] {
        let mut counts = [0usize; 5];
        for class in &self.values {
            counts[*class as usize] += 1;
        }
        counts
    }
}

/// One parameter read by a model/instance-static branch condition.
///
/// Value and `$param_given` reads stay distinct. They invalidate together, but
/// preserving the distinction lets a later specializer construct the smallest
/// possible key without guessing which half of the parameter state mattered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParameterDependency {
    pub parameter: ParamId,
    pub scope: ParameterScope,
    pub reads_value: bool,
    pub reads_given: bool,
}

/// Static inputs that decide one model-structure branch.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StaticDependencies {
    /// Sorted by parameter id for deterministic reports and specialization
    /// keys.
    pub parameters: Vec<ParameterDependency>,
    /// Whether the per-instance `m` multiplier participates in the condition.
    pub multiplicity: bool,
}

/// A model/instance-static branch that remains around work evaluated more
/// frequently.
///
/// This is deliberately an analysis result, not an instruction to duplicate
/// code. A backend can rank these by `newton_values`, group repeated
/// conditions, and reject any candidate whose emitted variants exceed its
/// source-size budget.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralGuard {
    pub branch: BlockId,
    pub condition: ValueId,
    pub class: InvalidationClass,
    pub dependencies: StaticDependencies,
    /// Blocks control-dependent on this branch.
    pub controlled_blocks: usize,
    /// Instructions in those blocks, independent of invalidation class.
    pub controlled_values: usize,
    /// Instructions in those blocks that execute in the Newton stage.
    pub newton_values: usize,
}

/// Find parameter-controlled branches that can shape a generated hot path.
///
/// Bias-, time-, and temperature-dependent branches are intentionally absent:
/// their outcome is part of the numerical evaluation, not model structure.
/// Constant branches are absent too; CFG optimization is responsible for
/// deleting those instead of asking runtime specialization to carry a key bit.
pub fn structural_guards(
    function: &CfgFunction,
    schedule: &Schedule,
    parameter_scopes: &[ParameterScope],
) -> Vec<StructuralGuard> {
    let control = transitive_control_dependence(&control_dependence(function));
    let block_of = block_of_value(function);
    let incoming = incoming_values(function);
    let mut guards = Vec::new();

    for block in &function.blocks {
        let CfgTerminator::Branch { condition, .. } = block.terminator else {
            continue;
        };
        let class = schedule.class(condition);
        if class > InvalidationClass::Instance {
            continue;
        }

        let dependencies = static_dependencies(
            function,
            condition,
            parameter_scopes,
            &control,
            &block_of,
            &incoming,
        );
        if dependencies.parameters.is_empty() && !dependencies.multiplicity {
            continue;
        }

        let mut controlled_blocks = 0usize;
        let mut controlled_values = 0usize;
        let mut newton_values = 0usize;
        for (candidate, sources) in function.blocks.iter().zip(&control) {
            if !sources.contains(&block.id) {
                continue;
            }
            controlled_blocks += 1;
            controlled_values = controlled_values.saturating_add(candidate.instructions.len());
            newton_values = newton_values.saturating_add(
                candidate
                    .instructions
                    .iter()
                    .filter(|instruction| {
                        schedule.class(instruction.result) == InvalidationClass::Newton
                    })
                    .count(),
            );
        }
        if newton_values == 0 {
            continue;
        }

        guards.push(StructuralGuard {
            branch: block.id,
            condition,
            class,
            dependencies,
            controlled_blocks,
            controlled_values,
            newton_values,
        });
    }

    guards.sort_unstable_by(|left, right| {
        right
            .newton_values
            .cmp(&left.newton_values)
            .then_with(|| right.controlled_values.cmp(&left.controlled_values))
            .then_with(|| usize::from(left.branch).cmp(&usize::from(right.branch)))
    });
    guards
}

fn transitive_control_dependence(direct: &[Vec<BlockId>]) -> Vec<Vec<BlockId>> {
    direct
        .iter()
        .map(|sources| {
            let mut seen: HashSet<BlockId> = HashSet::new();
            let mut pending = sources.clone();
            while let Some(source) = pending.pop() {
                if !seen.insert(source) {
                    continue;
                }
                pending.extend(direct[usize::from(source)].iter().copied());
            }
            let mut sources: Vec<_> = seen.into_iter().collect();
            sources.sort_unstable_by_key(|source| usize::from(*source));
            sources
        })
        .collect()
}

fn static_dependencies(
    function: &CfgFunction,
    root: ValueId,
    parameter_scopes: &[ParameterScope],
    control: &[Vec<BlockId>],
    block_of: &[Option<BlockId>],
    incoming: &[Vec<(BlockId, ValueId)>],
) -> StaticDependencies {
    let mut reads: BTreeMap<ParamId, (bool, bool)> = BTreeMap::new();
    let mut multiplicity = false;
    let mut seen: HashSet<ValueId> = HashSet::new();
    let mut pending = vec![root];

    while let Some(value) = pending.pop() {
        if !seen.insert(value) {
            continue;
        }
        match &function.value(value).kind {
            CfgValueKind::Parameter(parameter) => {
                reads.entry(*parameter).or_default().0 = true;
            }
            CfgValueKind::ParameterGiven(parameter) => {
                reads.entry(*parameter).or_default().1 = true;
            }
            CfgValueKind::Multiplicity => multiplicity = true,
            CfgValueKind::BlockParameter => {
                pending.extend(
                    incoming[usize::from(value)]
                        .iter()
                        .map(|(_, argument)| *argument),
                );
            }
            kind => pending.extend(kind.operands()),
        }

        // A condition computed under another static condition depends on that
        // outer choice even when its arithmetic operands do not name it.
        if let Some(block) = block_of[usize::from(value)] {
            for source in &control[usize::from(block)] {
                if let CfgTerminator::Branch { condition, .. } = function.block(*source).terminator
                {
                    pending.push(condition);
                }
            }
        }
    }

    StaticDependencies {
        parameters: reads
            .into_iter()
            .map(
                |(parameter, (reads_value, reads_given))| ParameterDependency {
                    parameter,
                    scope: parameter_scopes
                        .get(usize::from(parameter))
                        .copied()
                        .unwrap_or(ParameterScope::Instance),
                    reads_value,
                    reads_given,
                },
            )
            .collect(),
        multiplicity,
    }
}

/// Classify every value in `function`.
///
/// The two halves alternate to a fixed point rather than running once each,
/// and BSIM-CMG is why. Raising a loop lifts the values *inside* it, which
/// leaves anything outside that reads one holding a staler class than the value
/// it depends on — and that invariant, that a value is at least as volatile as
/// everything it reads, is what the whole split rests on. Breaking it puts an
/// instance-class consumer in a stage that dropped the block defining what it
/// reads, and the value ends up in the stage's table with nothing defining it.
///
/// Both halves only ever raise, over a five-element lattice, so this settles.
pub fn schedule(function: &CfgFunction) -> Schedule {
    schedule_with_parameter_scopes(function, &[])
}

/// Classify every value while preserving the model/instance parameter boundary
/// declared by the Verilog-A source.
///
/// Missing entries are conservatively instance-scoped, which keeps hand-built
/// CFG fixtures and older callers behaviorally compatible.
pub fn schedule_with_parameter_scopes(
    function: &CfgFunction,
    parameter_scopes: &[ParameterScope],
) -> Schedule {
    // These are graph properties, not fixed-point state. Computing either in
    // `propagate` repeats a whole-graph analysis every time loop/projection
    // raising asks for another propagation pass. Indexing incoming values is
    // more important still: looking them up by rescanning every CFG edge for
    // every block parameter is quadratic on large compact models.
    let control = control_dependence(function);
    let block_of = block_of_value(function);
    let incoming = incoming_values(function);
    let mut schedule = classify(function, parameter_scopes, &control, &block_of, &incoming);
    loop {
        let before = schedule.clone();
        raise_loops(function, &mut schedule);
        raise_ambiguous_projections(function, &mut schedule);
        if schedule == before {
            return schedule;
        }
        propagate(function, &mut schedule, &control, &block_of, &incoming);
    }
}

/// Everything in a loop shares one class.
///
/// A stage hands a coarser stage's value across in a *slot*, which holds one
/// value. A loop-carried variable holds a different one on each iteration, so a
/// loop that straddled a stage boundary would read whichever value the slot
/// last held. Raising the whole loop to the most volatile thing in it costs the
/// coarser stages some work they could in principle have kept — and compact
/// models have very few run-time loops, so in practice it costs nothing at all.
fn raise_loops(function: &CfgFunction, schedule: &mut Schedule) {
    let block_of = block_of_value(function);
    for body in natural_loops(function) {
        let mut class = InvalidationClass::Model;
        for block in &body {
            class = class.join(schedule.blocks[usize::from(*block)]);
        }
        for (index, block) in block_of.iter().enumerate() {
            if block.is_some_and(|block| body.contains(&block)) {
                class = class.join(schedule.values[index]);
            }
        }
        for block in &body {
            schedule.blocks[usize::from(*block)] = class;
        }
        for (index, block) in block_of.iter().enumerate() {
            if block.is_some_and(|block| body.contains(&block)) {
                schedule.values[index] = class;
            }
        }
    }
}

/// Raise anything a dropped region gets to *choose* between.
///
/// This is the property the slice actually needs, stated directly: if a stage
/// drops a region, every path out of that region has to land in the same place,
/// or the stage is being asked to reproduce a choice whose test it dropped.
///
/// Control dependence is supposed to guarantee that already — a block reached
/// from one arm and not the other is control-dependent on the branch, so it is
/// at least as volatile. But control dependence is the post-dominance frontier,
/// and **post-dominance says nothing useful about a region that may not
/// terminate**. Around a run-time loop it silently under-classifies, and
/// BSIM-CMG is where that showed: a Newton-class block reaching two distinct
/// instance-class blocks, one predecessor each.
///
/// Patching the frontier was the obvious move and the wrong one. The frontier
/// is a proxy; this checks the thing itself, on the real successor edges, and
/// raises whatever it finds until there is nothing left to find. Both halves
/// only ever raise over a four-element lattice, so it terminates.
fn raise_ambiguous_projections(function: &CfgFunction, schedule: &mut Schedule) {
    // A search per edge is the obvious implementation and is far too slow — on
    // BSIM-CMG's two thousand blocks it is a walk of the region for every edge
    // into it, inside two fixed points. This is the same question asked as a
    // dataflow over a three-element lattice, which settles in a few sweeps.
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Frontier {
        Nothing,
        Exactly(BlockId),
        Several,
    }
    let join = |left: Frontier, right: Frontier| match (left, right) {
        (Frontier::Nothing, other) | (other, Frontier::Nothing) => other,
        (Frontier::Exactly(left), Frontier::Exactly(right)) if left == right => {
            Frontier::Exactly(left)
        }
        _ => Frontier::Several,
    };

    let mut predecessors: Vec<Vec<BlockId>> = vec![Vec::new(); function.blocks.len()];
    for block in &function.blocks {
        for successor in block.successors() {
            predecessors[usize::from(successor)].push(block.id);
        }
    }

    // Newton keeps every block, so it can never be ambiguous.
    for class in [
        InvalidationClass::Model,
        InvalidationClass::Instance,
        InvalidationClass::Temperature,
        InvalidationClass::Timestep,
    ] {
        loop {
            let blocks = schedule.blocks.clone();
            let kept = |block: BlockId| blocks[usize::from(block)] <= class;

            // Where each dropped block's paths land, as far as this stage can
            // tell them apart.
            let mut lands = vec![Frontier::Nothing; function.blocks.len()];
            let mut settling = true;
            while settling {
                settling = false;
                for block in function.blocks.iter().rev() {
                    if kept(block.id) {
                        continue;
                    }
                    let mut now = Frontier::Nothing;
                    for successor in block.successors() {
                        now = join(
                            now,
                            if kept(successor) {
                                Frontier::Exactly(successor)
                            } else {
                                lands[usize::from(successor)]
                            },
                        );
                    }
                    if now != lands[usize::from(block.id)] {
                        lands[usize::from(block.id)] = now;
                        settling = true;
                    }
                }
            }

            // The choice is made where the paths diverge, but the blocks that
            // have to be raised are where they land — and those can be reached
            // through sub-paths that are each unambiguous on their own. So the
            // divergence is marked and carried forward across the region rather
            // than read off its immediate successors.
            let mut inside = vec![false; function.blocks.len()];
            for block in &function.blocks {
                if !kept(block.id) && lands[usize::from(block.id)] == Frontier::Several {
                    inside[usize::from(block.id)] = true;
                }
            }
            let mut spreading = true;
            while spreading {
                spreading = false;
                for block in &function.blocks {
                    if !inside[usize::from(block.id)] {
                        continue;
                    }
                    for successor in block.successors() {
                        if !kept(successor) && !inside[usize::from(successor)] {
                            inside[usize::from(successor)] = true;
                            spreading = true;
                        }
                    }
                }
            }

            let mut changed = false;
            for block in &function.blocks {
                if !kept(block.id) {
                    continue;
                }
                for source in &predecessors[usize::from(block.id)] {
                    if !inside[usize::from(*source)] {
                        continue;
                    }
                    let raised =
                        schedule.blocks[usize::from(block.id)].join(blocks[usize::from(*source)]);
                    if raised != schedule.blocks[usize::from(block.id)] {
                        schedule.blocks[usize::from(block.id)] = raised;
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }
    }
}

/// The blocks of each natural loop, keyed by nothing — one set per back edge.
fn natural_loops(function: &CfgFunction) -> Vec<HashSet<BlockId>> {
    let mut predecessors: Vec<Vec<BlockId>> = vec![Vec::new(); function.blocks.len()];
    for block in &function.blocks {
        for successor in block.successors() {
            predecessors[usize::from(successor)].push(block.id);
        }
    }

    let mut loops = Vec::new();
    for (tail, header) in back_edges(function) {
        // Everything that reaches the tail without passing through the header.
        let mut body: HashSet<BlockId> = HashSet::from([header, tail]);
        let mut stack = vec![tail];
        while let Some(block) = stack.pop() {
            for predecessor in &predecessors[usize::from(block)] {
                if body.insert(*predecessor) {
                    stack.push(*predecessor);
                }
            }
        }
        loops.push(body);
    }
    loops
}

/// Edges that jump backwards, as `(tail, header)`.
fn back_edges(function: &CfgFunction) -> Vec<(BlockId, BlockId)> {
    let mut edges = Vec::new();
    let mut state: HashMap<BlockId, u8> = HashMap::new();
    let mut stack = vec![(function.entry, 0usize)];
    state.insert(function.entry, 1);
    while let Some((block, index)) = stack.pop() {
        let successors = function.block(block).successors();
        if index < successors.len() {
            stack.push((block, index + 1));
            let successor = successors[index];
            match state.get(&successor) {
                Some(1) => edges.push((block, successor)),
                Some(_) => {}
                None => {
                    state.insert(successor, 1);
                    stack.push((successor, 0));
                }
            }
        } else {
            state.insert(block, 2);
        }
    }
    edges
}

fn classify(
    function: &CfgFunction,
    parameter_scopes: &[ParameterScope],
    control: &[Vec<BlockId>],
    block_of: &[Option<BlockId>],
    incoming: &[Vec<(BlockId, ValueId)>],
) -> Schedule {
    let values: Vec<InvalidationClass> = function
        .values
        .iter()
        .map(|value| {
            // Every packed derivative is Newton-class, seeded here rather than
            // fixed up afterwards so the fixed point carries it to the scalars
            // that read one. Almost all of them are Newton anyway — a
            // derivative traces back to a seed, and a seed is an unknown. The
            // exceptions are the constant lane arrays, the zero a merge passes
            // from an arm reaching no unknown and the widening of one, and
            // caching those would be caching a constant.
            if value.value_type.shape().is_some() {
                return InvalidationClass::Newton;
            }
            leaf_class(&value.kind, parameter_scopes)
        })
        .collect();
    let blocks = vec![InvalidationClass::Model; function.blocks.len()];
    let mut schedule = Schedule { values, blocks };
    propagate(function, &mut schedule, control, block_of, incoming);
    schedule
}

/// Raise every class until each value is at least as volatile as everything it
/// reads and as the guards it sits under.
///
/// Only ever raises, and starts from whatever `schedule` already holds, so it
/// can be re-run after something else has raised part of it.
fn propagate(
    function: &CfgFunction,
    schedule: &mut Schedule,
    control: &[Vec<BlockId>],
    block_of: &[Option<BlockId>],
    incoming: &[Vec<(BlockId, ValueId)>],
) {
    let Schedule { values, blocks } = schedule;

    // A branch's condition is a value with a class of its own, so the two
    // halves are mutually recursive. Monotone over a five-element lattice, so
    // it settles.
    loop {
        let mut changed = false;

        for (block, sources) in control.iter().enumerate() {
            let mut class = InvalidationClass::Model;
            for source in sources {
                if let CfgTerminator::Branch { condition, .. } = &function.block(*source).terminator
                {
                    class = class.join(values[usize::from(*condition)]);
                }
            }
            // Joined rather than assigned. The two are the same on a first run
            // — `blocks` starts at Instance and `class` only rises with
            // `values` — but assignment would undo a loop raising on a re-run.
            let class = class.join(blocks[block]);
            if blocks[block] != class {
                blocks[block] = class;
                changed = true;
            }
        }

        for value in &function.values {
            let index = usize::from(value.id);
            let mut class = values[index];
            for operand in value.kind.operands() {
                class = class.join(values[usize::from(operand)]);
            }
            if matches!(value.kind, CfgValueKind::BlockParameter) {
                // A merge is as volatile as the most volatile thing merged into
                // it, and as the branch that chose between them.
                for (source, argument) in &incoming[index] {
                    class = class
                        .join(values[usize::from(*argument)])
                        .join(blocks[usize::from(*source)]);
                }
            }
            if let Some(block) = block_of[index] {
                class = class.join(blocks[usize::from(block)]);
            }
            if values[index] != class {
                values[index] = class;
                changed = true;
            }
        }

        if !changed {
            return;
        }
    }
}

/// How much smaller the Newton stage has to be before splitting pays for
/// itself.
///
/// Measured, `tests/cfg_runtime.rs`, as the ratio of whole-body values to
/// Newton-stage values against the runtime actually observed:
///
/// | | value cut | measured speed-up |
/// | :--- | ---: | ---: |
/// | `vbic_4T_et_cf` | 1.02x | 0.98x |
/// | `EPFL_HEMT_10a` | 1.02x | 1.01x |
/// | `r3_cmc` | 1.15x | 1.17x |
/// | `DIODE_CMC` | 3.30x | 1.95x |
///
/// The two that cut nothing came out *slower* than the unsplit body, which is
/// the whole point: a stage that removes no work still adds a staged load for
/// everything it reads, and something has to pay for that. 1.05 sits in the gap
/// with room on both sides rather than on top of either group.
const WORTH_SPLITTING: f64 = 1.05;

/// Whether slicing this body by invalidation class is worth what it costs.
///
/// Splitting is a caching decision and caching is not free: a value a coarser
/// stage computed arrives through a slot rather than a register, and on a model
/// that is nearly all bias-dependent there is no saved work to set against
/// those loads. Deciding from the stages themselves rather than from the class
/// census costs nothing — the stages are already built, and their value counts
/// are already known — and it measures the thing that correlates instead of a
/// proxy for it.
pub fn worth_splitting(function: &CfgFunction, stages: &[Stage]) -> bool {
    if stages.len() < 2 {
        return false;
    }
    let Some(newton) = stages.last() else {
        return false;
    };
    let whole = function.values.len().max(1) as f64;
    let cut = whole / newton.function.values.len().max(1) as f64;
    cut >= WORTH_SPLITTING
}

/// One invalidation class's share of a split body.
#[derive(Debug, Clone, PartialEq)]
pub struct Stage {
    pub class: InvalidationClass,
    pub function: CfgFunction,
    /// Values this stage computes that a later one reads, as
    /// `(slot, value in this stage)`. The caller writes each to its slot.
    pub exports: Vec<(u32, ValueId)>,
    /// Source-function value for each entry in `exports`. Keeping the origin
    /// explicit lets independent slices prove that they are exporting the same
    /// computation without depending on stage-local value numbering.
    pub export_origins: Vec<ValueId>,
    /// Where each requested output landed, `None` if another stage computes it.
    pub outputs: Vec<Option<ValueId>>,
}

impl Stage {
    /// The slot this stage writes `value` to, if anything reads it from the
    /// cache rather than from this stage's own locals.
    ///
    /// An output the last stage computes has no slot and needs none — the
    /// caller reads it where it is produced.
    pub fn slot_of(&self, value: ValueId) -> Option<u32> {
        self.exports
            .iter()
            .find(|(_, exported)| *exported == value)
            .map(|(slot, _)| *slot)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SplitError {
    /// A packed derivative would have had to cross a stage boundary. It cannot
    /// by construction — every derivative depends on a seed, which is
    /// Newton-class — so this means the classification is wrong rather than
    /// that the model is unusual.
    PackedValueCrossesStages(ValueId),
    /// A stage named a value that neither it nor any cache provides. Always a
    /// bug in the slicing rather than in the model: something was demanded at a
    /// class the slot assignment did not anticipate.
    OperandLeftBehind(InvalidationClass, ValueId),
    /// A stage did not survive [`CfgFunction::validate`]. Always the slicing's
    /// fault, never the model's — so the value id alone is not enough to act
    /// on, and `detail` says what the offending value actually is.
    MalformedStage(InvalidationClass, CfgValidationError, String),
    /// An edge into a region this stage dropped could land on more than one
    /// block the stage keeps, so reproducing it would need the very test the
    /// stage dropped.
    AmbiguousProjection(InvalidationClass, BlockId, String),
    /// A stage owns a value a later stage reads, and did not produce it. The
    /// slot would stay zero and the reader would carry on with it.
    ExportedValueLost(InvalidationClass, ValueId, String),
}

impl std::fmt::Display for SplitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PackedValueCrossesStages(value) => {
                write!(f, "{value} is packed and would have to cross a stage")
            }
            Self::OperandLeftBehind(class, value) => {
                write!(
                    f,
                    "the {} stage reads {value}, which nothing in it defines and no slot carries",
                    class.name()
                )
            }
            Self::MalformedStage(class, error, detail) => {
                write!(
                    f,
                    "the {} stage is malformed: {error}{detail}",
                    class.name()
                )
            }
            Self::AmbiguousProjection(class, block, detail) => {
                write!(
                    f,
                    "the {} stage cannot route the edge into {block}: it reaches more than one \
                     block the stage keeps{detail}",
                    class.name()
                )
            }
            Self::ExportedValueLost(class, value, detail) => {
                write!(
                    f,
                    "the {} stage owns {value}, which a later stage reads, but does not \
                     produce it{detail}",
                    class.name()
                )
            }
        }
    }
}

impl std::error::Error for SplitError {}

/// Cut `function` into one function per invalidation class it uses.
///
/// Each stage keeps the control flow it needs and nothing else: a conditional
/// whose test is more volatile than the stage disappears, because everything
/// inside it is too. What a stage reads from a coarser one arrives as
/// [`CfgValueKind::Staged`] — except for leaves, which are simply rebuilt, so a
/// parameter stays `parameters[3]` in every stage rather than costing a slot.
///
/// Stages come back coarsest first, which is the order they run in.
pub fn split(
    function: &CfgFunction,
    schedule: &Schedule,
    outputs: &[ValueId],
) -> Result<Vec<Stage>, SplitError> {
    let block_of = block_of_value(function);

    // A value needs a slot when something more volatile than it reads it, and
    // neither of the cheaper answers applies: a leaf is rebuilt in place, and a
    // packed value is recomputed from its own operands. Rebuilding a packed
    // value is not a concession — the only ones that fall below the Newton
    // class are constant lane arrays, and caching a constant would cost a slot
    // to save nothing.
    let mut slots: Vec<Option<u32>> = vec![None; function.values.len()];
    let mut next_slot = 0u32;
    let demand = |value: ValueId,
                  reader: InvalidationClass,
                  slots: &mut Vec<Option<u32>>,
                  next_slot: &mut u32|
     -> Result<(), SplitError> {
        let held = schedule.values[usize::from(value)];
        if held >= reader || super::cfg::is_leaf_kind(&function.value(value).kind) {
            return Ok(());
        }
        if function.value(value).value_type.shape().is_some() {
            // `raise_packed` puts every one of these in the last stage, so
            // reaching here means the classification changed under this and a
            // Jacobian would silently be a stale one.
            return Err(SplitError::PackedValueCrossesStages(value));
        }
        if slots[usize::from(value)].is_none() {
            slots[usize::from(value)] = Some(*next_slot);
            *next_slot += 1;
        }
        Ok(())
    };

    for value in &function.values {
        let reader = schedule.values[usize::from(value.id)];
        for operand in value.kind.operands() {
            demand(operand, reader, &mut slots, &mut next_slot)?;
        }
    }
    // Control flow is *replicated* into every stage that keeps the block, so a
    // branch's condition is read by all of them and not just by the one that
    // computes it. Demanding it at the deepest class in use is what makes the
    // guard available to the Newton stage even when the test itself is
    // instance-static — which is the common case, since most guards in a
    // compact model are model-card options.
    let deepest = schedule
        .values
        .iter()
        .copied()
        .max()
        .unwrap_or(InvalidationClass::Model);
    for block in &function.blocks {
        match &block.terminator {
            CfgTerminator::Jump { target, args } => {
                for (param, argument) in function.block(*target).params.iter().zip(args) {
                    let reader = schedule.values[usize::from(*param)];
                    demand(*argument, reader, &mut slots, &mut next_slot)?;
                }
            }
            CfgTerminator::Branch {
                condition,
                then_target,
                then_args,
                else_target,
                else_args,
            } => {
                demand(*condition, deepest, &mut slots, &mut next_slot)?;
                for (target, args) in [(then_target, then_args), (else_target, else_args)] {
                    for (param, argument) in function.block(*target).params.iter().zip(args) {
                        let reader = schedule.values[usize::from(*param)];
                        demand(*argument, reader, &mut slots, &mut next_slot)?;
                    }
                }
            }
            CfgTerminator::Return | CfgTerminator::Unset => {}
        }
    }
    // An output is read by whoever called `split`, and that read happens after
    // the last stage has run — so it is demanded at the deepest class in use,
    // exactly as a replicated branch condition is. Without this, an output a
    // coarse stage owns is computed into a local that stage drops on return and
    // the caller has nowhere to read it from: `Stage::outputs` would name it and
    // no slot would carry it. An output the deepest stage computes still costs
    // nothing, because `demand` stops as soon as the holder is no coarser than
    // the reader.
    for output in outputs {
        demand(*output, deepest, &mut slots, &mut next_slot)?;
    }

    let mut stages = Vec::new();
    for class in InvalidationClass::ALL {
        if !schedule.values.contains(&class) {
            continue;
        }
        stages.push(build_stage(
            function, schedule, outputs, &block_of, &slots, class, deepest,
        )?);
    }
    Ok(stages)
}

#[allow(clippy::too_many_arguments)]
fn build_stage(
    function: &CfgFunction,
    schedule: &Schedule,
    outputs: &[ValueId],
    block_of: &[Option<BlockId>],
    slots: &[Option<u32>],
    class: InvalidationClass,
    deepest: InvalidationClass,
) -> Result<Stage, SplitError> {
    let kept_block = |block: BlockId| schedule.blocks[usize::from(block)] <= class;
    let held = |value: ValueId| schedule.values[usize::from(value)];
    // Whether this stage computes the value itself, rather than reading what a
    // coarser one cached.
    let defined_here = |value: ValueId| held(value) == class;
    // Which stage reports a requested output as its own.
    //
    // For anything with an instruction, the stage that computes it. A leaf has
    // no instruction and is rebuilt wherever it is read, so no stage computes it
    // in that sense and every stage could claim it; the last one does, because
    // that is where the caller reads. `I(a,c) <+ bias` is the shape that gets
    // here — a contribution reading no unknown, whose whole residual simplifies
    // to a parameter.
    let owns_output = |output: ValueId| {
        if super::cfg::is_leaf_kind(&function.value(output).kind) {
            class == deepest
        } else {
            held(output) == class
        }
    };

    // Values are renumbered per stage, so a stage is a function in its own
    // right rather than a view onto a bigger one.
    let mut mapped: Vec<Option<ValueId>> = vec![None; function.values.len()];
    let mut values: Vec<CfgValue> = Vec::new();
    let translate = |value: ValueId,
                     mapped: &mut Vec<Option<ValueId>>,
                     values: &mut Vec<CfgValue>|
     -> ValueId {
        if let Some(existing) = mapped[usize::from(value)] {
            return existing;
        }
        let source = function.value(value);
        let kind = if defined_here(value) {
            source.kind.clone()
        } else {
            match slots[usize::from(value)] {
                Some(slot) => CfgValueKind::Staged { slot },
                // A leaf, so it is cheaper to rebuild than to cache.
                None => source.kind.clone(),
            }
        };
        let id = ValueId::from(values.len());
        values.push(CfgValue {
            id,
            value_type: source.value_type,
            kind,
        });
        mapped[usize::from(value)] = Some(id);
        id
    };

    // Every value this stage names, transitively. The walk stops at anything a
    // coarser stage owns, because that arrives as a slot or a rebuilt leaf and
    // its operands do not come along.
    let mut order: Vec<ValueId> = Vec::new();
    let mut roots: Vec<ValueId> = Vec::new();
    for block in &function.blocks {
        if !kept_block(block.id) {
            continue;
        }
        roots.extend(
            block
                .params
                .iter()
                .copied()
                .filter(|param| held(*param) == class),
        );
        roots.extend(
            block
                .instructions
                .iter()
                .map(|entry| entry.result)
                .filter(|result| defined_here(*result)),
        );
        let arguments = |target: BlockId, args: &[ValueId]| -> Vec<ValueId> {
            function
                .block(target)
                .params
                .iter()
                .zip(args)
                .filter(|(param, _)| held(**param) == class)
                .map(|(_, argument)| *argument)
                .collect()
        };
        match &block.terminator {
            CfgTerminator::Jump { target, args } => roots.extend(arguments(*target, args)),
            CfgTerminator::Branch {
                condition,
                then_target,
                then_args,
                else_target,
                else_args,
            } => {
                if held(*condition) <= class {
                    roots.push(*condition);
                    roots.extend(arguments(*then_target, then_args));
                    roots.extend(arguments(*else_target, else_args));
                }
            }
            CfgTerminator::Return | CfgTerminator::Unset => {}
        }
    }
    roots.extend(
        outputs
            .iter()
            .copied()
            .filter(|output| owns_output(*output)),
    );
    let mut seen: HashSet<ValueId> = HashSet::new();
    let mut stack: Vec<(ValueId, usize)> = Vec::new();
    for root in roots {
        if !seen.insert(root) {
            continue;
        }
        stack.push((root, 0));
        while let Some((value, index)) = stack.pop() {
            let operands = if defined_here(value) {
                function.value(value).kind.operands()
            } else {
                Vec::new()
            };
            if index < operands.len() {
                stack.push((value, index + 1));
                let operand = operands[index];
                if seen.insert(operand) {
                    stack.push((operand, 0));
                }
            } else {
                order.push(value);
            }
        }
    }
    for value in &order {
        translate(*value, &mut mapped, &mut values);
    }

    let mut blocks: Vec<CfgBlock> = Vec::with_capacity(function.blocks.len());
    let mut block_map: Vec<Option<BlockId>> = vec![None; function.blocks.len()];
    for block in &function.blocks {
        if kept_block(block.id) {
            block_map[usize::from(block.id)] = Some(BlockId::from(blocks.len()));
            blocks.push(CfgBlock {
                id: BlockId::from(blocks.len()),
                params: Vec::new(),
                instructions: Vec::new(),
                terminator: CfgTerminator::Unset,
            });
        }
    }

    // One explicit exit, created before any terminator is decided, so every
    // routing decision below is total: there is always somewhere to send an
    // edge and no case has to invent a `Return` in the middle of the graph.
    let exit = BlockId::from(blocks.len());
    blocks.push(CfgBlock {
        id: exit,
        params: Vec::new(),
        instructions: Vec::new(),
        terminator: CfgTerminator::Return,
    });

    for block in &function.blocks {
        let Some(id) = block_map[usize::from(block.id)] else {
            continue;
        };
        let params: Vec<ValueId> = block
            .params
            .iter()
            .filter(|param| held(**param) == class)
            .map(|param| translate(*param, &mut mapped, &mut values))
            .collect();
        let instructions: Vec<CfgInstruction> = block
            .instructions
            .iter()
            .filter(|entry| defined_here(entry.result))
            .map(|entry| CfgInstruction {
                result: translate(entry.result, &mut mapped, &mut values),
            })
            .collect();

        // Arguments are filtered the same way the target filtered its
        // parameters, so the two stay in step.
        let arguments = |target: BlockId,
                         args: &[ValueId],
                         values: &mut Vec<CfgValue>,
                         mapped: &mut Vec<Option<ValueId>>|
         -> Vec<ValueId> {
            function
                .block(target)
                .params
                .iter()
                .zip(args)
                .filter(|(param, _)| held(**param) == class)
                .map(|(_, argument)| translate(*argument, mapped, values))
                .collect()
        };

        let terminator = match &block.terminator {
            CfgTerminator::Jump { target, args } => match block_map[usize::from(*target)] {
                Some(mapped_target) => CfgTerminator::Jump {
                    args: arguments(*target, args, &mut values, &mut mapped),
                    target: mapped_target,
                },
                None => CfgTerminator::Jump {
                    target: onward(*target, function, &block_map, exit, class, schedule)?,
                    args: Vec::new(),
                },
            },
            CfgTerminator::Branch {
                condition,
                then_target,
                then_args,
                else_target,
                else_args,
            } => {
                if held(*condition) <= class {
                    // Each arm resolves on its own. Collapsing the whole branch
                    // because one arm was dropped takes the *other* arm down
                    // with it, and a kept block that nothing reaches is a value
                    // this stage silently stops computing — which is how
                    // BSIM-CMG came to own a slot it never wrote.
                    let then_kept = block_map[usize::from(*then_target)];
                    let else_kept = block_map[usize::from(*else_target)];
                    let then_landing = match then_kept {
                        Some(kept) => kept,
                        None => onward(*then_target, function, &block_map, exit, class, schedule)?,
                    };
                    let else_landing = match else_kept {
                        Some(kept) => kept,
                        None => onward(*else_target, function, &block_map, exit, class, schedule)?,
                    };
                    CfgTerminator::Branch {
                        condition: translate(*condition, &mut mapped, &mut values),
                        then_args: match then_kept {
                            Some(_) => arguments(*then_target, then_args, &mut values, &mut mapped),
                            None => Vec::new(),
                        },
                        then_target: then_landing,
                        else_args: match else_kept {
                            Some(_) => arguments(*else_target, else_args, &mut values, &mut mapped),
                            None => Vec::new(),
                        },
                        else_target: else_landing,
                    }
                } else {
                    // The test is more volatile than this stage, so both arms
                    // are too. Skip straight to where they reconverge; the join
                    // cannot carry a parameter this stage keeps, because every
                    // argument to it comes from an arm.
                    let then_landing =
                        onward(*then_target, function, &block_map, exit, class, schedule)?;
                    let else_landing =
                        onward(*else_target, function, &block_map, exit, class, schedule)?;
                    if then_landing != else_landing {
                        return Err(SplitError::AmbiguousProjection(
                            class,
                            block.id,
                            String::new(),
                        ));
                    }
                    CfgTerminator::Jump {
                        target: then_landing,
                        args: Vec::new(),
                    }
                }
            }
            CfgTerminator::Return | CfgTerminator::Unset => CfgTerminator::Jump {
                target: exit,
                args: Vec::new(),
            },
        };

        blocks[usize::from(id)] = CfgBlock {
            id,
            params,
            instructions,
            terminator,
        };
    }

    // Exactly one `Return` — the emitter's post-dominance walk needs that — and
    // it now falls out of the construction rather than needing a pass: the exit
    // block is the only one that returns, and every other terminator was built
    // to reach it.

    // Operands still name the whole function's values, because a value is
    // cloned before the things it reads have necessarily been given stage ids.
    // One pass at the end, now that every value this stage names has one.
    let mut unmapped: Option<ValueId> = None;
    for value in &mut values {
        value
            .kind
            .map_operands(|operand| match mapped[usize::from(operand)] {
                Some(id) => id,
                None => {
                    unmapped = unmapped.or(Some(operand));
                    operand
                }
            });
    }
    if let Some(operand) = unmapped {
        return Err(SplitError::OperandLeftBehind(class, operand));
    }

    let entry = block_map[usize::from(function.entry)].unwrap_or(BlockId::from(0usize));
    let (mut blocks, entry) = prune_blocks(blocks, entry);
    // The outputs are read by the caller, not by a block, so compaction has to
    // be told about them or a leaf output nothing else reads is swept.
    let produced: Vec<ValueId> = outputs
        .iter()
        .filter(|output| owns_output(**output))
        .filter_map(|output| mapped[usize::from(*output)])
        .collect();
    let values = compact_values(&mut blocks, values, &mut mapped, &produced);
    let stage = CfgFunction {
        entry,
        blocks,
        values,
        shapes: function.shapes.clone(),
    };
    stage.validate().map_err(|error| {
        let detail = describe(&stage, &error, &mapped, schedule, block_of);
        SplitError::MalformedStage(class, error, detail)
    })?;

    // A value this stage owns and a later one reads has to survive to here. If
    // it does not, the slot is never written and the stage that reads it gets a
    // zero — a wrong answer rather than a failure, which is the one outcome not
    // worth tolerating. `compact_values` is what made this reachable: before it,
    // such a value would have been "defined" in a block that no longer runs,
    // which is equally wrong and merely quieter.
    let mut exports = Vec::new();
    let mut export_origins = Vec::new();
    for (index, slot) in slots.iter().enumerate() {
        let Some(slot) = *slot else { continue };
        let value = ValueId::from(index);
        if schedule.values[index] != class {
            continue;
        }
        match mapped[index] {
            Some(mapped) => {
                exports.push((slot, mapped));
                export_origins.push(value);
            }
            None => {
                let home = block_of[index];
                // How it is reached matters, because "kept but unreachable" and
                // "dropped" are opposite bugs: the first means an edge into it
                // went missing, the second means the classification put it in
                // the wrong stage.
                let arrivals: Vec<String> = home
                    .into_iter()
                    .flat_map(|home| {
                        function
                            .blocks
                            .iter()
                            .filter(move |candidate| candidate.successors().contains(&home))
                    })
                    .map(|source| {
                        format!(
                            "{} ({:?}, {}, {})",
                            source.id,
                            schedule.blocks[usize::from(source.id)],
                            match block_map[usize::from(source.id)] {
                                Some(_) => "kept",
                                None => "dropped",
                            },
                            match &source.terminator {
                                CfgTerminator::Jump { .. } => "jump".to_string(),
                                CfgTerminator::Branch { condition, .. } => format!(
                                    "branch on a {:?} test",
                                    schedule.values[usize::from(*condition)]
                                ),
                                CfgTerminator::Return => "return".to_string(),
                                CfgTerminator::Unset => "unset".to_string(),
                            },
                        )
                    })
                    .take(6)
                    .collect();
                return Err(SplitError::ExportedValueLost(
                    class,
                    value,
                    format!(
                        " ({:?}; defined in {home:?}, whose class is {:?}; that block was {}; \
                         reached from {})",
                        function.value(value).kind,
                        home.map(|block| schedule.blocks[usize::from(block)]),
                        match home.and_then(|block| block_map[usize::from(block)]) {
                            Some(_) => "kept by this stage",
                            None => "dropped by this stage",
                        },
                        arrivals.join(", "),
                    ),
                ));
            }
        }
    }
    let outputs = outputs
        .iter()
        .map(|output| {
            owns_output(*output)
                .then(|| mapped[usize::from(*output)])
                .flatten()
        })
        .collect();

    let _ = block_of;
    Ok(Stage {
        class,
        function: stage,
        exports,
        export_origins,
        outputs,
    })
}

/// What a validation failure is actually about.
///
/// A value id in a stage's own numbering means nothing on its own: the stage is
/// renumbered, so the id cannot be looked up in the function it came from. What
/// is worth knowing is the value's kind and who reads it — a value nothing reads
/// is a stage that kept something it should have dropped, and a value something
/// reads is a stage that dropped something it should have kept. Those are
/// opposite bugs and the id alone does not distinguish them.
fn describe(
    stage: &CfgFunction,
    error: &CfgValidationError,
    mapped: &[Option<ValueId>],
    schedule: &Schedule,
    block_of: &[Option<BlockId>],
) -> String {
    let CfgValidationError::UndefinedValue(value) = *error else {
        return String::new();
    };
    let readers: Vec<ValueId> = stage
        .values
        .iter()
        .filter(|other| other.kind.operands().contains(&value))
        .map(|other| other.id)
        .take(4)
        .collect();
    let mut carried = Vec::new();
    for block in &stage.blocks {
        let mentioned = match &block.terminator {
            CfgTerminator::Jump { args, .. } => args.contains(&value),
            CfgTerminator::Branch {
                condition,
                then_args,
                else_args,
                ..
            } => *condition == value || then_args.contains(&value) || else_args.contains(&value),
            CfgTerminator::Return | CfgTerminator::Unset => false,
        };
        if mentioned {
            carried.push(block.id);
        }
    }

    // Where it came from matters more than where it landed: the interesting
    // question is whether the value's own class and the class of the block that
    // defines it disagree, which is the one way a stage can keep a value and
    // drop its definition.
    let origin = mapped
        .iter()
        .position(|stage_id| *stage_id == Some(value))
        .map(ValueId::from);
    let provenance = match origin {
        Some(origin) => {
            let block = block_of[usize::from(origin)];
            format!(
                "; was {origin}, class {:?}, defined in {block:?} whose class is {:?}",
                schedule.class(origin),
                block.map(|block| schedule.blocks[usize::from(block)]),
            )
        }
        None => "; no original value maps to it".to_string(),
    };

    format!(
        " ({:?}; read by {readers:?}; on the edges out of {carried:?}{provenance})",
        stage.value(value).kind
    )
}

/// Drop the values whose definitions the block pruning took away, and renumber.
///
/// A stage is built from the whole body's block list and then pruned, and
/// pruning is where the two can fall out of step: a block that ends up
/// unreachable is swept, but the values its instructions defined are still in
/// the stage's value table, now with nothing defining them. That surfaces as
/// `UndefinedValue`, one layer away from its cause, and BSIM-CMG spent three
/// wrong hypotheses proving that chasing *why* a particular block became
/// unreachable is the wrong shape of fix — the value table simply has to follow
/// the blocks, whatever the reason.
///
/// Kept: anything a surviving block defines, plus whatever those transitively
/// read. A leaf reaches this only by being read, so leaves that were live in the
/// whole body but are dead in this stage stop being emitted into its prologue.
fn compact_values(
    blocks: &mut [CfgBlock],
    values: Vec<CfgValue>,
    mapped: &mut [Option<ValueId>],
    produced: &[ValueId],
) -> Vec<CfgValue> {
    let mut keep = vec![false; values.len()];
    let mut stack: Vec<ValueId> = Vec::new();
    let demand = |value: ValueId, keep: &mut Vec<bool>, stack: &mut Vec<ValueId>| {
        if !keep[usize::from(value)] {
            keep[usize::from(value)] = true;
            stack.push(value);
        }
    };
    for value in produced {
        demand(*value, &mut keep, &mut stack);
    }
    for block in blocks.iter() {
        for param in &block.params {
            demand(*param, &mut keep, &mut stack);
        }
        for instruction in &block.instructions {
            demand(instruction.result, &mut keep, &mut stack);
        }
        match &block.terminator {
            CfgTerminator::Jump { args, .. } => {
                for arg in args {
                    demand(*arg, &mut keep, &mut stack);
                }
            }
            CfgTerminator::Branch {
                condition,
                then_args,
                else_args,
                ..
            } => {
                demand(*condition, &mut keep, &mut stack);
                for arg in then_args.iter().chain(else_args) {
                    demand(*arg, &mut keep, &mut stack);
                }
            }
            CfgTerminator::Return | CfgTerminator::Unset => {}
        }
    }
    while let Some(value) = stack.pop() {
        for operand in values[usize::from(value)].kind.operands() {
            demand(operand, &mut keep, &mut stack);
        }
    }

    let mut renumber: Vec<Option<ValueId>> = vec![None; values.len()];
    let mut compacted: Vec<CfgValue> = Vec::with_capacity(values.len());
    for value in &values {
        if !keep[usize::from(value.id)] {
            continue;
        }
        renumber[usize::from(value.id)] = Some(ValueId::from(compacted.len()));
        let mut value = value.clone();
        value.id = ValueId::from(compacted.len());
        compacted.push(value);
    }
    let translate =
        |value: ValueId| renumber[usize::from(value)].expect("everything reachable was kept above");
    for value in &mut compacted {
        value.kind.map_operands(translate);
    }
    for block in blocks.iter_mut() {
        for param in &mut block.params {
            *param = translate(*param);
        }
        for instruction in &mut block.instructions {
            instruction.result = translate(instruction.result);
        }
        match &mut block.terminator {
            CfgTerminator::Jump { args, .. } => {
                for arg in args {
                    *arg = translate(*arg);
                }
            }
            CfgTerminator::Branch {
                condition,
                then_args,
                else_args,
                ..
            } => {
                *condition = translate(*condition);
                for arg in then_args.iter_mut().chain(else_args) {
                    *arg = translate(*arg);
                }
            }
            CfgTerminator::Return | CfgTerminator::Unset => {}
        }
    }
    // Exports and outputs are read out of `mapped` after this, so it has to
    // move with everything else — and a value that did not survive is one this
    // stage no longer provides.
    for entry in mapped.iter_mut() {
        *entry = entry.and_then(|value| renumber[usize::from(value)]);
    }
    compacted
}

/// Where an edge into a block this stage dropped should go instead.
///
/// A stage keeps a block when the block's guard is no more volatile than the
/// stage, so the blocks it keeps are not a contiguous region: a Newton-guarded
/// loop sits between two stretches of instance-static work, and the instance
/// stage keeps both ends and neither middle. Cutting the edge and returning —
/// which is what this did until BSIM-CMG caught it — strands everything only
/// reachable through the dropped region. The block is still emitted, still
/// carries its instructions, and nothing defines their results any more, which
/// surfaces one stage later as a value that is never defined.
///
/// The walk is forward through the real successor edges, stopping at the first
/// kept block on each path. Following immediate post-dominators instead reads
/// well and is wrong around loops: post-dominance is undefined where a region
/// can fail to terminate, so the chain runs out and the edge falls through to
/// the exit, stranding everything after the loop. BSIM-CMG is the only model in
/// the corpus with run-time loops and the only one that showed it.
///
/// Reaching more than one distinct kept block would mean the stage has to
/// reproduce a choice whose test it dropped. Control dependence says that
/// cannot happen — anything downstream of a more volatile test is itself more
/// volatile — so it is reported rather than guessed at.
fn onward(
    target: BlockId,
    function: &CfgFunction,
    block_map: &[Option<BlockId>],
    exit: BlockId,
    class: InvalidationClass,
    schedule: &Schedule,
) -> Result<BlockId, SplitError> {
    let mut seen: HashSet<BlockId> = HashSet::new();
    let mut queue: Vec<BlockId> = vec![target];
    let mut landings: Vec<BlockId> = Vec::new();
    while let Some(block) = queue.pop() {
        if !seen.insert(block) {
            continue;
        }
        if block_map[usize::from(block)].is_some() {
            // Stop here: this is the frontier, and what lies beyond it is
            // reached through it rather than around it.
            if !landings.contains(&block) {
                landings.push(block);
            }
            continue;
        }
        queue.extend(function.block(block).successors());
    }
    match landings.as_slice() {
        [] => Ok(exit),
        [single] => Ok(block_map[usize::from(*single)].expect("kept, as the walk just checked")),
        several => Err(SplitError::AmbiguousProjection(
            class,
            target,
            format!(
                " ({target} is {:?}; landings {})",
                schedule.blocks[usize::from(target)],
                several
                    .iter()
                    .take(6)
                    .map(|block| format!(
                        "{block} ({:?}, {} predecessors)",
                        schedule.blocks[usize::from(*block)],
                        function
                            .blocks
                            .iter()
                            .filter(|source| source.successors().contains(block))
                            .count()
                    ))
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
        )),
    }
}

/// Drop the control flow a stage kept but does not use.
///
/// Slicing leaves a stage with the *shape* of the whole body — every guard the
/// model wrote, whether or not anything of this class sits inside it. Measured
/// on `DIODE_CMC`, whose 2,275 blocks come from 530 source conditionals, that
/// scaffolding was larger than the work: the timestep stage held one value and
/// 62 KB of empty `if`/`else`. So an empty block with one exit is spliced out
/// and a branch whose arms have become the same block is a jump.
///
/// A loop header is never removed. Its emptiness in some stage would be a
/// question about the loop, and `raise_loops` has already put every loop
/// entirely inside one stage.
fn prune_blocks(mut blocks: Vec<CfgBlock>, entry: BlockId) -> (Vec<CfgBlock>, BlockId) {
    // Spliced-out blocks are recorded rather than deleted in place, so the next
    // round does not rediscover them: nothing points at one any more, but it is
    // still in the vector until the unreachable sweep at the end.
    let mut spliced: HashSet<BlockId> = HashSet::new();
    loop {
        let mut redirect: HashMap<BlockId, BlockId> = HashMap::new();
        let headers = loop_headers(&blocks, entry);
        for block in &blocks {
            if block.id == entry || headers.contains(&block.id) || spliced.contains(&block.id) {
                continue;
            }
            if !block.params.is_empty() || !block.instructions.is_empty() {
                continue;
            }
            if let CfgTerminator::Jump { target, args } = &block.terminator
                && args.is_empty()
                && *target != block.id
            {
                redirect.insert(block.id, *target);
            }
        }
        if redirect.is_empty() {
            break;
        }
        spliced.extend(redirect.keys().copied());
        // A chain of empty blocks collapses in one pass rather than one per
        // link, which on a model with thousands of them is the difference
        // between linear and quadratic.
        let resolve = |mut block: BlockId| {
            for _ in 0..redirect.len() + 1 {
                match redirect.get(&block) {
                    Some(next) if *next != block => block = *next,
                    _ => break,
                }
            }
            block
        };
        for block in &mut blocks {
            match &mut block.terminator {
                CfgTerminator::Jump { target, .. } => *target = resolve(*target),
                CfgTerminator::Branch {
                    then_target,
                    else_target,
                    ..
                } => {
                    *then_target = resolve(*then_target);
                    *else_target = resolve(*else_target);
                }
                CfgTerminator::Return | CfgTerminator::Unset => {}
            }
        }
        for block in &mut blocks {
            if let CfgTerminator::Branch {
                then_target,
                then_args,
                else_target,
                else_args,
                ..
            } = &block.terminator
                && then_target == else_target
                && then_args == else_args
            {
                block.terminator = CfgTerminator::Jump {
                    target: *then_target,
                    args: then_args.clone(),
                };
            }
        }
    }

    // Whatever is now unreachable goes, and the rest is renumbered so block ids
    // stay dense.
    let mut reachable: HashSet<BlockId> = HashSet::from([entry]);
    let mut stack = vec![entry];
    while let Some(block) = stack.pop() {
        for successor in blocks[usize::from(block)].successors() {
            if reachable.insert(successor) {
                stack.push(successor);
            }
        }
    }
    let mut renumber: Vec<Option<BlockId>> = vec![None; blocks.len()];
    let mut kept: Vec<CfgBlock> = Vec::with_capacity(reachable.len());
    for block in &blocks {
        if !reachable.contains(&block.id) {
            continue;
        }
        renumber[usize::from(block.id)] = Some(BlockId::from(kept.len()));
        kept.push(block.clone());
    }
    for block in &mut kept {
        block.id = renumber[usize::from(block.id)].expect("a kept block is renumbered");
        match &mut block.terminator {
            CfgTerminator::Jump { target, .. } => {
                *target = renumber[usize::from(*target)].expect("a reachable target is kept");
            }
            CfgTerminator::Branch {
                then_target,
                else_target,
                ..
            } => {
                *then_target =
                    renumber[usize::from(*then_target)].expect("a reachable target is kept");
                *else_target =
                    renumber[usize::from(*else_target)].expect("a reachable target is kept");
            }
            CfgTerminator::Return | CfgTerminator::Unset => {}
        }
    }
    let entry = renumber[usize::from(entry)].expect("the entry is reachable from itself");
    (kept, entry)
}

/// Blocks some edge jumps backwards into.
fn loop_headers(blocks: &[CfgBlock], entry: BlockId) -> HashSet<BlockId> {
    let mut headers = HashSet::new();
    let mut state: HashMap<BlockId, u8> = HashMap::new();
    let mut stack = vec![(entry, 0usize)];
    state.insert(entry, 1);
    while let Some((block, index)) = stack.pop() {
        let successors = blocks[usize::from(block)].successors();
        if index < successors.len() {
            stack.push((block, index + 1));
            let successor = successors[index];
            match state.get(&successor) {
                Some(1) => {
                    headers.insert(successor);
                }
                Some(_) => {}
                None => {
                    state.insert(successor, 1);
                    stack.push((successor, 0));
                }
            }
        } else {
            state.insert(block, 2);
        }
    }
    headers
}

/// What a value depends on before anything it reads is considered.
fn leaf_class(kind: &CfgValueKind, parameter_scopes: &[ParameterScope]) -> InvalidationClass {
    match kind {
        CfgValueKind::NodePotential(_)
        | CfgValueKind::BranchFlow(_)
        | CfgValueKind::BranchUnknownFlow(_)
        | CfgValueKind::EventState(_)
        | CfgValueKind::Ddt { .. }
        // `idt` accumulates into per-instance history on every evaluation, so
        // caching it at a coarser class would integrate a step the solver did
        // not take.
        | CfgValueKind::Idt { .. }
        | CfgValueKind::Cross { .. }
        | CfgValueKind::Above { .. }
        | CfgValueKind::Timer { .. }
        | CfgValueKind::Limit { .. }
        | CfgValueKind::LimitPrevious { .. }
        // Both of these are constant within an analysis, and both are put here
        // rather than lower on purpose: `$simparam("gmin")` moves during gmin
        // stepping, and a cache keyed on anything coarser would hand back a
        // value from the previous step. The cost is small — they appear a
        // handful of times per model — and the failure they avoid is a
        // convergence bug that looks like a model problem.
        | CfgValueKind::Analysis(_)
        | CfgValueKind::SimParam { .. } => InvalidationClass::Newton,

        CfgValueKind::Time | CfgValueKind::DdtScale | CfgValueKind::IdtScale => {
            InvalidationClass::Timestep
        }

        CfgValueKind::Temperature | CfgValueKind::ThermalVoltage => InvalidationClass::Temperature,

        CfgValueKind::Multiplicity => InvalidationClass::Instance,

        CfgValueKind::Parameter(parameter) | CfgValueKind::ParameterGiven(parameter) => {
            match parameter_scopes
                .get(usize::from(*parameter))
                .copied()
                .unwrap_or(ParameterScope::Instance)
            {
                ParameterScope::Model => InvalidationClass::Model,
                ParameterScope::Instance => InvalidationClass::Instance,
            }
        }

        _ => InvalidationClass::Model,
    }
}

fn block_of_value(function: &CfgFunction) -> Vec<Option<BlockId>> {
    let mut block_of = vec![None; function.values.len()];
    for block in &function.blocks {
        for param in &block.params {
            block_of[usize::from(*param)] = Some(block.id);
        }
        for instruction in &block.instructions {
            block_of[usize::from(instruction.result)] = Some(block.id);
        }
    }
    block_of
}

/// Every `(predecessor, argument)` feeding each block parameter, indexed by
/// value id.
fn incoming_values(function: &CfgFunction) -> Vec<Vec<(BlockId, ValueId)>> {
    let mut incoming = vec![Vec::new(); function.values.len()];
    for block in &function.blocks {
        let mut record = |target: BlockId, args: &[ValueId]| {
            for (param, argument) in function.block(target).params.iter().zip(args) {
                incoming[usize::from(*param)].push((block.id, *argument));
            }
        };
        match &block.terminator {
            CfgTerminator::Jump { target, args } => record(*target, args),
            CfgTerminator::Branch {
                then_target,
                then_args,
                else_target,
                else_args,
                ..
            } => {
                record(*then_target, then_args);
                record(*else_target, else_args);
            }
            CfgTerminator::Return | CfgTerminator::Unset => {}
        }
    }
    incoming
}

/// For each block, the branching blocks it is control-dependent on.
///
/// Ferrante-Ottenstein-Warren: for every edge `A -> B` where `B` does not
/// post-dominate `A`, everything from `B` up to (not including) the immediate
/// post-dominator of `A` is control-dependent on `A`.
fn control_dependence(function: &CfgFunction) -> Vec<Vec<BlockId>> {
    let ipdom = immediate_post_dominators(function);
    let mut dependence: Vec<HashSet<BlockId>> = vec![HashSet::new(); function.blocks.len()];

    for block in &function.blocks {
        let successors = block.successors();
        if successors.len() < 2 {
            continue;
        }
        let stop = ipdom[usize::from(block.id)];
        for successor in successors {
            let mut current = Some(successor);
            while let Some(node) = current {
                if Some(node) == stop || node == block.id {
                    break;
                }
                if !dependence[usize::from(node)].insert(block.id) {
                    // Already recorded, so the rest of this walk was too.
                    break;
                }
                let next = ipdom[usize::from(node)];
                current = match next {
                    Some(next) if next != node => Some(next),
                    _ => None,
                };
            }
        }
    }

    dependence
        .into_iter()
        .map(|sources| {
            let mut sources: Vec<BlockId> = sources.into_iter().collect();
            sources.sort_unstable_by_key(|block| usize::from(*block));
            sources
        })
        .collect()
}

/// Immediate post-dominators, by the usual iteration on the reversed graph.
fn immediate_post_dominators(function: &CfgFunction) -> Vec<Option<BlockId>> {
    let Some(exit) = function
        .blocks
        .iter()
        .find(|block| matches!(block.terminator, CfgTerminator::Return))
        .map(|block| block.id)
    else {
        return vec![None; function.blocks.len()];
    };

    let mut reversed: Vec<Vec<BlockId>> = vec![Vec::new(); function.blocks.len()];
    for block in &function.blocks {
        for successor in block.successors() {
            reversed[usize::from(successor)].push(block.id);
        }
    }

    let order = reverse_postorder(&reversed, exit, function.blocks.len());
    let mut position = vec![usize::MAX; function.blocks.len()];
    for (index, block) in order.iter().enumerate() {
        position[usize::from(*block)] = index;
    }

    let mut predecessors: Vec<Vec<BlockId>> = vec![Vec::new(); function.blocks.len()];
    for (block, targets) in reversed.iter().enumerate() {
        for target in targets {
            predecessors[usize::from(*target)].push(BlockId::from(block));
        }
    }

    let mut ipdom: Vec<Option<BlockId>> = vec![None; function.blocks.len()];
    ipdom[usize::from(exit)] = Some(exit);
    loop {
        let mut changed = false;
        for block in &order {
            if *block == exit {
                continue;
            }
            let mut candidate: Option<BlockId> = None;
            for predecessor in &predecessors[usize::from(*block)] {
                if ipdom[usize::from(*predecessor)].is_none() {
                    continue;
                }
                candidate = Some(match candidate {
                    Some(current) => intersect(&ipdom, &position, *predecessor, current, exit),
                    None => *predecessor,
                });
            }
            if candidate.is_some() && ipdom[usize::from(*block)] != candidate {
                ipdom[usize::from(*block)] = candidate;
                changed = true;
            }
        }
        if !changed {
            return ipdom;
        }
    }
}

fn reverse_postorder(successors: &[Vec<BlockId>], entry: BlockId, count: usize) -> Vec<BlockId> {
    let mut visited = vec![false; count];
    let mut postorder = Vec::with_capacity(count);
    let mut stack = vec![(entry, 0usize)];
    visited[usize::from(entry)] = true;
    while let Some((block, index)) = stack.pop() {
        let edges = &successors[usize::from(block)];
        if index < edges.len() {
            stack.push((block, index + 1));
            let next = edges[index];
            if !visited[usize::from(next)] {
                visited[usize::from(next)] = true;
                stack.push((next, 0));
            }
        } else {
            postorder.push(block);
        }
    }
    postorder.reverse();
    postorder
}

fn intersect(
    ipdom: &[Option<BlockId>],
    position: &[usize],
    mut left: BlockId,
    mut right: BlockId,
    exit: BlockId,
) -> BlockId {
    while left != right {
        while position[usize::from(left)] > position[usize::from(right)] {
            match ipdom[usize::from(left)] {
                Some(next) if next != left => left = next,
                _ => return exit,
            }
        }
        while position[usize::from(right)] > position[usize::from(left)] {
            match ipdom[usize::from(right)] {
                Some(next) if next != right => right = next,
                _ => return exit,
            }
        }
    }
    left
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical_ir::cfg::{CfgBinaryOp, CfgValueType, CfgVariable, SsaBuilder};
    use crate::canonical_ir::{NodeId, ParamId, VariableId};

    /// Straight-line arithmetic keeps the class of what it reads.
    #[test]
    fn a_value_is_as_volatile_as_what_it_reads() {
        let mut builder = SsaBuilder::new();
        let entry = builder.create_block();
        builder.seal_block(entry);

        let param = builder.push_leaf(
            CfgValueType::Real,
            CfgValueKind::Parameter(ParamId::from(0usize)),
        );
        let temperature = builder.push_leaf(CfgValueType::Real, CfgValueKind::Temperature);
        let potential = builder.push_leaf(
            CfgValueType::Real,
            CfgValueKind::NodePotential(NodeId::from(0usize)),
        );

        let scaled = builder.push(
            entry,
            CfgValueType::Real,
            CfgValueKind::Binary {
                op: CfgBinaryOp::Mul,
                left: param,
                right: param,
            },
        );
        let warmed = builder.push(
            entry,
            CfgValueType::Real,
            CfgValueKind::Binary {
                op: CfgBinaryOp::Mul,
                left: scaled,
                right: temperature,
            },
        );
        let biased = builder.push(
            entry,
            CfgValueType::Real,
            CfgValueKind::Binary {
                op: CfgBinaryOp::Mul,
                left: warmed,
                right: potential,
            },
        );
        builder.set_terminator(entry, CfgTerminator::Return);

        let (function, outputs) = builder
            .finish_with_outputs(entry, &[scaled, warmed, biased])
            .expect("the fixture is well formed");
        let schedule = schedule(&function);

        assert_eq!(schedule.class(outputs[0]), InvalidationClass::Instance);
        assert_eq!(schedule.class(outputs[1]), InvalidationClass::Temperature);
        assert_eq!(schedule.class(outputs[2]), InvalidationClass::Newton);
    }

    /// The half that is easy to get wrong, and the reason the pass exists in
    /// this form: parameter arithmetic under a bias-dependent guard is not
    /// instance-static, however much it looks like it.
    #[test]
    fn a_guard_makes_what_it_guards_as_volatile_as_itself() {
        let mut builder = SsaBuilder::new();
        let entry = builder.create_block();
        builder.seal_block(entry);
        let taken = builder.create_block();
        let skipped = builder.create_block();
        let join = builder.create_block();

        let param = builder.push_leaf(
            CfgValueType::Real,
            CfgValueKind::Parameter(ParamId::from(0usize)),
        );
        let zero = builder.push_leaf(CfgValueType::Real, CfgValueKind::RealConstant(0.0));
        let potential = builder.push_leaf(
            CfgValueType::Real,
            CfgValueKind::NodePotential(NodeId::from(0usize)),
        );
        let condition = builder.push(
            entry,
            CfgValueType::Boolean,
            CfgValueKind::Binary {
                op: CfgBinaryOp::Gt,
                left: potential,
                right: zero,
            },
        );
        builder.set_terminator(
            entry,
            CfgTerminator::Branch {
                condition,
                then_target: taken,
                then_args: Vec::new(),
                else_target: skipped,
                else_args: Vec::new(),
            },
        );
        builder.seal_block(taken);
        builder.seal_block(skipped);

        // Reads nothing but a parameter, and is still Newton-class because it
        // only runs when the bias says so.
        let guarded = builder.push(
            taken,
            CfgValueType::Real,
            CfgValueKind::Binary {
                op: CfgBinaryOp::Mul,
                left: param,
                right: param,
            },
        );
        builder.write_variable(CfgVariable::Local(VariableId::from(0usize)), taken, guarded);
        builder.set_terminator(
            taken,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );
        builder.write_variable(CfgVariable::Local(VariableId::from(0usize)), skipped, zero);
        builder.set_terminator(
            skipped,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );
        builder.seal_block(join);
        let merged = builder
            .read_variable(CfgVariable::Local(VariableId::from(0usize)), join)
            .expect("both arms define it");
        builder.set_terminator(join, CfgTerminator::Return);

        let (function, outputs) = builder
            .finish_with_outputs(join, &[guarded, merged])
            .expect("the fixture is well formed");
        let schedule = schedule(&function);

        assert_eq!(
            schedule.class(outputs[0]),
            InvalidationClass::Newton,
            "parameter arithmetic under a bias-dependent guard must not be hoisted \
             out of the guard that protects it"
        );
        assert_eq!(schedule.class(outputs[1]), InvalidationClass::Newton);
    }
}
