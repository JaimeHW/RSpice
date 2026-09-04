//! Where the CFG route's machine code goes: a size census of the plan it
//! builds, against the postfix plan for the same module.
//!
//! The CFG route compiled bsimcmg_va to a 417 MB image where the postfix route
//! compiles it to 4.8 MB, and asmhemt did not finish at all. This census
//! measured *why*, at the level the difference is decided — the plan, not the
//! encoder — so the repair could be costed before it was written. With the
//! prelude in the plan builder those two are 5.6 MB and 17.3 MB against a
//! 4.8 MB and 16.7 MB postfix, and this census is what says so.
//!
//! # The two shapes being compared
//!
//! The postfix route runs an **assignment pass** once per evaluation: every
//! model variable is computed into a slot, and each of its value entries is a
//! small program that reads slots ([`NativeOp::LoadVariable`]). Every entry
//! therefore shares all of the body's intermediate arithmetic, and the plan's
//! total size is `assignment pass + Σ (small entries)`.
//!
//! The CFG route had no such pass. Each entry was
//! [`prune_cfg_to_outputs`](crate::canonical_ir::prune_cfg_to_outputs) applied
//! to the scalarized differentiated body for that one output, and then lowered
//! whole: the entry *contained* its entire dependence cone. Nothing was shared
//! between entries beyond what the value-entry cache could prove structurally
//! identical after the fact, and the plan's total size was
//! `Σ (cone of each entry)`.
//!
//! # What the plan builder does now, and why the cones are still measured
//!
//! It builds one [`CfgPrelude`](crate::jit::cfg_prelude::CfgPrelude): the union
//! below, pruned and lowered once, publishing each entry output into a slot.
//! Every entry it covers is *one instruction* — a `LoadPreludeSlot` — so the
//! `route=cfg` per-entry figures no longer describe the plan the builder
//! produces. They still describe the thing the fix is measured against: the
//! `cone_over_union` factor is the duplication that was removed, and the
//! `prelude` line beside the images is what replaced it. `image route=cfg`
//! compiles the real plan and is the figure to read for size.
//!
//! So the figure that decides the whole question is the **duplication factor**:
//! `Σ cone instructions` over the shared work counted once. The body is the
//! obvious denominator but it overstates that work — it carries scalarized
//! derivative lanes no shipped entry reads — so the census also prunes the body
//! to every entry output at once (the `union` below) and reports the factor
//! against both. If it is near one, the entries are disjoint and the explosion
//! is somewhere else — the encoder, the hoists, region duplication. If it is
//! near the entry count, every entry is carrying the same shared work and the
//! route is O(entries x body) by construction.
//!
//! # What each line reports
//!
//! Per model:
//!
//! * `body` — the scalarized differentiated function every entry is sliced out
//!   of: its values, its CFG instructions and its blocks. This is the
//!   denominator.
//! * `union` — one prune of that body to *every* entry output at once. This is
//!   the size an assignment prelude would have to be: the shared work, counted
//!   once. It is the numerator of the fix.
//! * `rows` — the per-equation slices the plan builder already takes, summed.
//!   It separates "each entry re-inlines the body" from "each equation does".
//! * one line per entry kind, on each route.
//!
//! # Lowering is optional, because one model cannot afford it
//!
//! Cone sizes are CFG-level and cost one linear pass each. Lowering an entry
//! onto the block model is what actually builds the instructions a backend
//! encodes, and on asmhemt that is unbounded — over eleven minutes and past
//! eighteen gigabytes without completing. `RSPICE_SIZE_CENSUS_LOWER=0` measures
//! cones only, and [`LOWERING_INSTRUCTION_BUDGET`] stops lowering a model that
//! runs past a ceiling rather than taking the host down with it. A model that
//! stopped early says `lowering=capped` and its block figures are a lower
//! bound; its cone figures are complete either way.
//!
//! `RSPICE_SIZE_CENSUS_IMAGE=1` additionally compiles both plans to x64 images
//! and reports the bytes, which is what calibrates instructions to bytes. It is
//! off by default because compiling the CFG plan is the expensive half and the
//! ratio does not need every model to establish it.
//!
//! `#[ignore]`d: this is measurement work, not a gate. Run it with
//! `--release --features native -- --ignored --nocapture`, narrowed with
//! `RSPICE_CFG_CENSUS_FILTER`.

use std::collections::HashMap;
use std::hash::{DefaultHasher, Hasher};
use std::time::Instant;

use super::census_models::shipped_census_models_matching;
use crate::canonical_ir::cfg_lower::CfgModel;
use crate::canonical_ir::{
    CfgFunction, CfgStateAllocation, ValueId, differentiate, prune_cfg_to_outputs,
};
use crate::jit::assignment::operation_count;
use crate::jit::cfg_lanes::scalarize_lanes;
use crate::jit::cfg_plan_builder::{
    CfgPlanEntry, CfgPlanRefusal, ShippedColumnLanes, build_model_plan_from_canonical_cfg,
    derivative_seeds,
};
use crate::jit::cfg_program::{CfgRuntimeBindings, lower_cfg_function};
use crate::jit::plan_builder::{
    build_model_plan_with_canonical_ir, canonical_branch_unknown_runtime_map,
};
use crate::jit::plan_program::PlanProgram;
use crate::rust_backend::canonical::stored_charges;

/// Block instructions after which a model stops being lowered.
///
/// Not a judgement about what is acceptable — it is a survival limit. The
/// block model holds one `Instruction` per lowered operation, and a model whose
/// entries sum past this has already proved the point the census is measuring;
/// continuing only risks the host. asmhemt is the model this exists for.
const LOWERING_INSTRUCTION_BUDGET: usize = 60_000_000;

/// Which plan field an entry belongs to.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    StampValue,
    Jacobian,
    ReactiveJacobian,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Self::StampValue => "stamp_values",
            Self::Jacobian => "jacobians",
            Self::ReactiveJacobian => "reactive_jacobians",
        }
    }
}

/// One entry kind's figures on the CFG route.
#[derive(Default)]
struct CfgKindTally {
    entries: usize,
    /// Entries the CFG route lowers to the one-instruction constant zero
    /// because its liveness found no value there. Counted apart: they are
    /// entries the shipped route has and this route does not pay for.
    structural_zeros: usize,
    cone_instructions: usize,
    cone_values: usize,
    block_instructions: usize,
    largest: Option<(CfgPlanEntry, usize)>,
    lowered: usize,
}

impl CfgKindTally {
    fn observe_cone(&mut self, instructions: usize, values: usize) {
        self.entries += 1;
        self.cone_instructions += instructions;
        self.cone_values += values;
    }

    fn observe_block(&mut self, entry: CfgPlanEntry, instructions: usize) {
        self.lowered += 1;
        self.block_instructions += instructions;
        if self
            .largest
            .is_none_or(|(_, largest)| instructions > largest)
        {
            self.largest = Some((entry, instructions));
        }
    }
}

/// One entry kind's figures on the postfix route.
#[derive(Default)]
struct PostfixKindTally {
    entries: usize,
    operations: usize,
    largest: usize,
}

impl PostfixKindTally {
    fn observe(&mut self, operations: usize) {
        self.entries += 1;
        self.operations += operations;
        self.largest = self.largest.max(operations);
    }
}

/// The value-entry cache's dedup, counted rather than performed.
///
/// [`ValueEntryCache`](crate::jit::value_cache::ValueEntryCache) publishes one
/// body per structurally identical block program, keyed by
/// [`Program::codegen_identity_hash`](crate::jit::ssa::Program::codegen_identity_hash)
/// and decided by an exact structural comparison. This keeps the hash and drops
/// the program, so the count is the cache's dedup up to a 64-bit collision —
/// under a part in ten trillion at every entry count this corpus reaches, and
/// the alternative is holding every distinct program of a 417 MB plan live.
#[derive(Default)]
struct DedupTally {
    /// First-seen instruction count per distinct program.
    distinct: HashMap<u64, usize>,
    total_instructions: usize,
    total_entries: usize,
}

impl DedupTally {
    fn observe(&mut self, program: &crate::jit::ssa::Program) {
        let instructions = program.instructions().len();
        let mut hasher = DefaultHasher::new();
        program.codegen_identity_hash(&mut hasher);
        self.distinct.entry(hasher.finish()).or_insert(instructions);
        self.total_instructions += instructions;
        self.total_entries += 1;
    }

    /// Instructions the cache removes: everything but one copy of each distinct
    /// program.
    fn deduplicated_instructions(&self) -> usize {
        self.total_instructions - self.distinct.values().sum::<usize>()
    }
}

fn cfg_instructions(function: &CfgFunction) -> usize {
    function
        .blocks
        .iter()
        .map(|block| block.instructions.len())
        .sum()
}

fn plan_program_size(program: &PlanProgram) -> usize {
    match program {
        PlanProgram::Postfix(program) => program.ops().len(),
        PlanProgram::Blocks(program) => program.ssa().instructions().len(),
    }
}

/// The size of every plan field the postfix route builds, printed and returned.
///
/// The assignment passes are the whole point of the comparison: they are the
/// route's shared work, and the CFG route has no counterpart to them.
fn census_postfix_plan(
    module: &str,
    model: &crate::codegen::CompiledModel,
    artifact: &crate::canonical_ir::CanonicalIrArtifact,
) -> Option<(usize, [PostfixKindTally; 3])> {
    let plan = match build_model_plan_with_canonical_ir(model, artifact) {
        Ok(plan) => plan,
        Err(error) => {
            println!("cfg-size model={module} postfix=refused detail={error}");
            return None;
        }
    };
    let assignment_operations: usize = plan
        .assignments
        .iter()
        .chain(&plan.post_assignments)
        .map(operation_count)
        .sum();
    let mut kinds: [PostfixKindTally; 3] = Default::default();
    for entry in &plan.stamp_values {
        kinds[0].observe(plan_program_size(entry));
    }
    for row in &plan.jacobians {
        for entry in row {
            kinds[1].observe(plan_program_size(entry));
        }
    }
    for row in &plan.reactive_jacobians {
        for entry in row {
            kinds[2].observe(plan_program_size(entry));
        }
    }
    let noise: usize = plan
        .noise_psd
        .iter()
        .chain(plan.noise_exponents.iter().flatten())
        .map(plan_program_size)
        .sum();
    println!(
        "cfg-size model={module} route=postfix assignments={} assignment_operations={} \
         noise_operations={noise}",
        plan.assignments.len() + plan.post_assignments.len(),
        assignment_operations,
    );
    for (kind, tally) in [Kind::StampValue, Kind::Jacobian, Kind::ReactiveJacobian]
        .into_iter()
        .zip(&kinds)
    {
        println!(
            "cfg-size model={module} route=postfix kind={} entries={} operations={} largest={}",
            kind.name(),
            tally.entries,
            tally.operations,
            tally.largest,
        );
    }
    Some((assignment_operations, kinds))
}

/// Where every byte of the CFG route's plan comes from, for one module.
///
/// This walks the same steps [`build_model_plan_from_canonical_cfg`] walks, in
/// the same order, and measures each entry as it goes. It is a second copy of
/// that traversal rather than a hook inside it because the production builder
/// must stay a builder: a measurement that changed it would be measuring
/// itself.
fn census_cfg_plan(module: &str, shipped: &super::census_models::CensusModel, lower: bool) {
    let model = &shipped.model;
    let artifact = &shipped.canonical_ir;

    let started = Instant::now();
    let Ok(mut cfg) = CfgModel::from_hir_for_executable_backend(&artifact.hir, &artifact.mir)
    else {
        println!("cfg-size model={module} cfg=refused detail=cfg-lowering");
        return;
    };
    if model.stamp_programs.len() != cfg.residuals.len() {
        println!("cfg-size model={module} cfg=refused detail=equations-unpaired");
        return;
    }
    let charges = stored_charges(&mut cfg.function, &cfg.residuals);
    let Ok(state) = CfgStateAllocation::build(&artifact.hir, &cfg.function) else {
        println!("cfg-size model={module} cfg=refused detail=state-allocation");
        return;
    };
    let primal_instructions = cfg_instructions(&cfg.function);
    let (seeds, correction_lane) = derivative_seeds(&cfg, &artifact.mir);
    let mut differentiated = match differentiate(&cfg.function, &seeds) {
        Ok(differentiated) => differentiated,
        Err(error) => {
            println!("cfg-size model={module} cfg=refused detail=differentiate:{error:?}");
            return;
        }
    };
    // Every read-out before anything else touches the function: taking one
    // appends an instruction.
    let jacobian_rows: Vec<Vec<Option<ValueId>>> = cfg
        .residuals
        .iter()
        .map(|residual| differentiated.derivative_row(*residual))
        .collect();
    let reactive_rows: Vec<Vec<Option<ValueId>>> = charges
        .iter()
        .map(|charge| match charge {
            Some(charge) => differentiated.derivative_row(*charge),
            None => Vec::new(),
        })
        .collect();
    let differentiated_instructions = cfg_instructions(&differentiated.function);
    let Ok(scalarized) = scalarize_lanes(&differentiated.function) else {
        println!("cfg-size model={module} cfg=refused detail=scalarize");
        return;
    };
    let body_instructions = cfg_instructions(&scalarized.function);
    let body_seconds = started.elapsed().as_secs_f64();
    println!(
        "cfg-size model={module} body primal_instructions={primal_instructions} \
         differentiated_instructions={differentiated_instructions} \
         instructions={body_instructions} values={} blocks={} seeds={} seconds={body_seconds:.1}",
        scalarized.function.values.len(),
        scalarized.function.blocks.len(),
        seeds.len(),
    );

    let bindings = {
        let Ok(branch_unknowns) = canonical_branch_unknown_runtime_map(model, &artifact.mir) else {
            println!("cfg-size model={module} cfg=refused detail=branch-unknown-map");
            return;
        };
        let event_state_variables: Vec<Option<usize>> = artifact
            .hir
            .variables
            .iter()
            .filter(|variable| variable.is_state)
            .map(|variable| {
                model
                    .variable_names
                    .iter()
                    .position(|name| *name == variable.name)
            })
            .collect();
        CfgRuntimeBindings::from_mir(
            module,
            &artifact.mir,
            branch_unknowns,
            event_state_variables,
        )
    };
    let Ok(column_lanes) = ShippedColumnLanes::build(model, &artifact.mir) else {
        println!("cfg-size model={module} cfg=refused detail=lane-unmapped");
        return;
    };

    // ---- the outputs every entry asks for, gathered once -----------------
    //
    // Gathered before anything is pruned so that the union below is one pass
    // over the body rather than one per row.
    let mut rows: Vec<Vec<(Kind, CfgPlanEntry, ValueId)>> =
        Vec::with_capacity(model.stamp_programs.len());
    let mut structural_zeros = [0_usize; 3];
    for (stamp_index, stamp) in model.stamp_programs.iter().enumerate() {
        let mut row = Vec::new();
        let stamp_entry = CfgPlanEntry::StampValue(stamp_index);
        match scalarized.scalar(cfg.residuals[stamp_index]) {
            Some(scalar) => row.push((Kind::StampValue, stamp_entry, scalar)),
            None => structural_zeros[0] += 1,
        }
        let derivatives =
            stamp
                .jacobian_programs
                .iter()
                .enumerate()
                .map(|(entry_index, jacobian)| {
                    (
                        Kind::Jacobian,
                        CfgPlanEntry::Jacobian(stamp_index, entry_index),
                        &jacobian.col_axis,
                        &jacobian_rows[stamp_index],
                    )
                })
                .chain(stamp.reactive_jacobians.iter().enumerate().map(
                    |(entry_index, reactive)| {
                        (
                            Kind::ReactiveJacobian,
                            CfgPlanEntry::ReactiveJacobian(stamp_index, entry_index),
                            &reactive.col_axis,
                            &reactive_rows[stamp_index],
                        )
                    },
                ));
        for (kind, entry, axis, source) in derivatives {
            let value = column_lanes
                .lane(axis)
                .filter(|lane| Some(*lane) != correction_lane)
                .and_then(|lane| source.get(lane).copied().flatten())
                .and_then(|value| scalarized.scalar(value));
            match value {
                Some(scalar) => row.push((kind, entry, scalar)),
                None => structural_zeros[kind as usize] += 1,
            }
        }
        rows.push(row);
    }

    // ---- the union: what a shared prelude would have to compute ----------
    let union_started = Instant::now();
    let all_outputs: Vec<ValueId> = rows
        .iter()
        .flat_map(|row| row.iter().map(|(_, _, value)| *value))
        .collect();
    let (union_function, _) = prune_cfg_to_outputs(&scalarized.function, &all_outputs);
    let union_instructions = cfg_instructions(&union_function);
    println!(
        "cfg-size model={module} union outputs={} instructions={union_instructions} values={} \
         blocks={} seconds={:.1}",
        all_outputs.len(),
        union_function.values.len(),
        union_function.blocks.len(),
        union_started.elapsed().as_secs_f64(),
    );
    drop(union_function);

    // ---- per-row and per-entry cones -------------------------------------
    let cones_started = Instant::now();
    let mut kinds: [CfgKindTally; 3] = Default::default();
    for (index, tally) in kinds.iter_mut().enumerate() {
        tally.structural_zeros = structural_zeros[index];
    }
    let mut row_cone_instructions = 0_usize;
    let mut dedup = DedupTally::default();
    let mut lowering_capped = false;
    for row in &rows {
        let row_outputs: Vec<ValueId> = row.iter().map(|(_, _, value)| *value).collect();
        let (row_function, row_mapped) = prune_cfg_to_outputs(&scalarized.function, &row_outputs);
        row_cone_instructions += cfg_instructions(&row_function);
        for ((kind, entry, _), output) in row.iter().zip(row_mapped) {
            let (cone, cone_outputs) = prune_cfg_to_outputs(&row_function, &[output]);
            let tally = &mut kinds[*kind as usize];
            tally.observe_cone(cfg_instructions(&cone), cone.values.len());
            if !lower || lowering_capped {
                continue;
            }
            match lower_cfg_function(&cone, cone_outputs[0], &state, &bindings) {
                Ok(program) => {
                    tally.observe_block(*entry, program.instructions().len());
                    dedup.observe(&program);
                    if dedup.total_instructions > LOWERING_INSTRUCTION_BUDGET {
                        lowering_capped = true;
                    }
                }
                Err(error) => {
                    println!("cfg-size model={module} entry={entry} lowering=refused {error}");
                }
            }
        }
    }
    let cones_seconds = cones_started.elapsed().as_secs_f64();

    let cone_instructions: usize = kinds.iter().map(|tally| tally.cone_instructions).sum();
    let block_instructions: usize = kinds.iter().map(|tally| tally.block_instructions).sum();
    let entries: usize = kinds.iter().map(|tally| tally.entries).sum();
    println!(
        "cfg-size model={module} rows={} row_cone_instructions={row_cone_instructions} \
         row_over_body={:.2} seconds={cones_seconds:.1}{}",
        rows.len(),
        row_cone_instructions as f64 / body_instructions.max(1) as f64,
        if lowering_capped {
            " lowering=capped"
        } else {
            ""
        },
    );
    for (kind, tally) in [Kind::StampValue, Kind::Jacobian, Kind::ReactiveJacobian]
        .into_iter()
        .zip(&kinds)
    {
        let (largest_entry, largest) = tally
            .largest
            .map_or_else(|| ("none".to_string(), 0), |(e, n)| (e.to_string(), n));
        println!(
            "cfg-size model={module} route=cfg kind={} entries={} structural_zeros={} \
             cone_instructions={} cone_values={} lowered={} block_instructions={} \
             largest={largest_entry}:{largest}",
            kind.name(),
            tally.entries,
            tally.structural_zeros,
            tally.cone_instructions,
            tally.cone_values,
            tally.lowered,
            tally.block_instructions,
        );
    }
    println!(
        "cfg-size model={module} factor entries={entries} cone_instructions={cone_instructions} \
         body_instructions={body_instructions} union_instructions={union_instructions} \
         cone_over_body={:.2} cone_over_union={:.2} union_over_body={:.2} \
         block_over_cone={:.2} block_instructions={block_instructions}",
        cone_instructions as f64 / body_instructions.max(1) as f64,
        cone_instructions as f64 / union_instructions.max(1) as f64,
        union_instructions as f64 / body_instructions.max(1) as f64,
        block_instructions as f64 / cone_instructions.max(1) as f64,
    );
    println!(
        "cfg-size model={module} dedup lowered={} distinct={} deduplicated_instructions={} \
         share={:.3}",
        dedup.total_entries,
        dedup.distinct.len(),
        dedup.deduplicated_instructions(),
        dedup.deduplicated_instructions() as f64 / dedup.total_instructions.max(1) as f64,
    );
}

/// Both routes' images for one module, in bytes.
///
/// What calibrates an instruction count to an image size, and the only figure
/// here that costs a whole codegen. Opt-in for that reason.
fn census_images(module: &str, shipped: &super::census_models::CensusModel) {
    let model = &shipped.model;
    let artifact = &shipped.canonical_ir;
    let started = Instant::now();
    match build_model_plan_with_canonical_ir(model, artifact)
        .map_err(|error| error.to_string())
        .and_then(|plan| {
            crate::native::x64::compile_model_plan(model, &plan).map_err(|error| error.to_string())
        }) {
        Ok(native) => println!(
            "cfg-size model={module} image route=postfix bytes={} seconds={:.1}",
            native.code_size_bytes(),
            started.elapsed().as_secs_f64(),
        ),
        Err(error) => println!("cfg-size model={module} image route=postfix refused={error}"),
    }
    let started = Instant::now();
    match build_model_plan_from_canonical_cfg(model, artifact)
        .map_err(|refused| refused.to_string())
        .and_then(|built| {
            // The prelude's figures are the ones that replaced Σ cone: every
            // entry it publishes for is one instruction, so what the image
            // holds once is this, and what it used to hold per entry was the
            // cone the block above still measures.
            println!(
                "cfg-size model={module} prelude slots={} instructions={} \
                 live_current_entries={} live_current_control_flow={} refused={}",
                built.report.prelude_slots,
                built.report.prelude_instructions,
                built.report.live_current_entries,
                built.report.live_current_control_flow,
                built
                    .report
                    .prelude_refused
                    .map_or("none", CfgPlanRefusal::name),
            );
            crate::native::x64::compile_model_plan(model, &built.plan)
                .map_err(|error| error.to_string())
        }) {
        Ok(native) => println!(
            "cfg-size model={module} image route=cfg bytes={} seconds={:.1}",
            native.code_size_bytes(),
            started.elapsed().as_secs_f64(),
        ),
        Err(error) => println!("cfg-size model={module} image route=cfg refused={error}"),
    }
}

/// Measure, per shipped module, where the CFG route's plan size comes from.
///
/// Asserts nothing. The question it answers — is the CFG route O(entries x
/// body) by construction, and how big is the shared part it is duplicating —
/// has no pass/fail form; what it produces is the numerator and denominator a
/// repair is costed from.
#[test]
#[ignore = "measurement; run with --release --features native -- --ignored --nocapture"]
fn the_cfg_route_plan_size_census() {
    let filter = std::env::var("RSPICE_CFG_CENSUS_FILTER").ok();
    let lower = !std::env::var("RSPICE_SIZE_CENSUS_LOWER").is_ok_and(|value| value == "0");
    let images = std::env::var("RSPICE_SIZE_CENSUS_IMAGE").is_ok_and(|value| value == "1");
    let mut models = 0_usize;
    for shipped in shipped_census_models_matching(filter.as_deref()) {
        let module = shipped.name.clone();
        models += 1;
        println!(
            "cfg-size model={module} compile_seconds={:.1} cached={}",
            shipped.compile_seconds, shipped.from_cache,
        );
        let started = Instant::now();
        if census_postfix_plan(&module, &shipped.model, &shipped.canonical_ir).is_some() {
            census_cfg_plan(&module, &shipped, lower);
            if images {
                census_images(&module, &shipped);
            }
        }
        println!(
            "cfg-size model={module} done seconds={:.1}",
            started.elapsed().as_secs_f64()
        );
    }
    println!("cfg-size models={models} lowering={lower} images={images}");
    assert!(models > 0, "the filter matched no shipped module");
}
