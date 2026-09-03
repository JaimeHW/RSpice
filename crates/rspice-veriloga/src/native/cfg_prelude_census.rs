//! Does the CFG prelude fit in a stack frame?
//!
//! [`the_cfg_route_plan_size_census`](super::cfg_size_census) measured the
//! problem: every entry the CFG route builds carries its own copy of the shared
//! body, and `Σ cone / union` reaches 383 on asmhemt. The repair is a prelude —
//! the union, lowered once, publishing each entry output into a per-evaluation
//! slot. This measures whether that repair is *representable*.
//!
//! # The one measurement that could refuse the design
//!
//! A prelude computes tens of thousands of values in one program. Every value
//! the register allocator cannot keep in a register goes to a spill slot in the
//! program's own stack frame, and live-range slot reuse has never been
//! exercised at this scale — the largest block program the shipped route has
//! ever built is one entry's cone. If the frame does not fit, the prelude has
//! to be split at dominance boundaries the way the postfix assignment pass is
//! split at `MAX_ASSIGNMENT_CHUNK_OPERATIONS`, and that changes the design.
//!
//! The binding ceiling is AArch64's. Its spill accesses are `LDR`/`STR` with a
//! scaled unsigned 12-bit immediate, so `spill_slot_offset` refuses slot 4096
//! and beyond: 4,096 slots, 32,768 bytes. x64 addresses its frame with a
//! `disp32` and refuses only past `i32::MAX` bytes, which no program reaches.
//! So the census reports both allocations — x64 allocates ten value registers
//! and AArch64 fifteen, so their spill counts differ — and asserts the AArch64
//! ceiling on the AArch64 allocation.
//!
//! # What each line reports
//!
//! One line per model: the union's instruction count (the shared work, counted
//! once), the prelude's block instruction count and their ratio, the two
//! backends' spill-slot counts and the AArch64 frame in bytes against its
//! ceiling, the slot count (one per distinct entry output), and — under
//! `RSPICE_PRELUDE_CENSUS_IMAGE=1` — the x64 image of the prelude alone.
//!
//! `#[ignore]`d: this is measurement work. Run it with
//! `--release --features native -- --ignored --nocapture`, narrowed with
//! `RSPICE_CFG_CENSUS_FILTER`.

use std::time::Instant;

use super::census_models::shipped_census_models_matching;
use crate::canonical_ir::cfg_lower::CfgModel;
use crate::canonical_ir::{
    CfgFunction, CfgStateAllocation, ValueId, differentiate, prune_cfg_to_outputs,
};
use crate::codegen::state_renumbering::StateSlotMapping;
use crate::jit::cfg_lanes::scalarize_lanes;
use crate::jit::cfg_plan_builder::{CfgPlanEntry, ShippedColumnLanes, derivative_seeds};
use crate::jit::cfg_prelude::{CfgPrelude, LiveCurrentTaint};
use crate::jit::cfg_program::CfgRuntimeBindings;
use crate::jit::plan_builder::canonical_branch_unknown_runtime_map;
use crate::jit::ssa::RegisterAllocation;
use crate::rust_backend::canonical::stored_charges;

/// The highest spill slot AArch64 can address, plus one.
///
/// Not a policy: `spill_slot_offset` in the AArch64 backend refuses a slot
/// whose scaled offset leaves the `LDR`/`STR` immediate field, and this is that
/// boundary written where the census can compare against it.
const AARCH64_SPILL_SLOT_CEILING: usize = 4096;

const WORD_BYTES: usize = 8;

fn cfg_instructions(function: &CfgFunction) -> usize {
    function
        .blocks
        .iter()
        .map(|block| block.instructions.len())
        .sum()
}

/// Everything the prelude needs for one module, walked in the plan builder's
/// own order.
pub(super) struct PreludeInputs {
    pub(super) function: CfgFunction,
    pub(super) entries: Vec<(CfgPlanEntry, ValueId)>,
    pub(super) state: CfgStateAllocation,
    pub(super) bindings: CfgRuntimeBindings,
    pub(super) slots: StateSlotMapping,
}

/// Reproduce the CFG plan builder's traversal as far as the entry outputs.
///
/// A second copy of that traversal rather than a hook inside it, for the same
/// reason [`super::cfg_size_census`] keeps one: the production builder must
/// stay a builder. Shared with the round-trip fixture beside it so that the
/// gate and the measurement are looking at the same entry set.
pub(super) fn prelude_inputs(
    module: &str,
    model: &crate::codegen::CompiledModel,
    artifact: &crate::canonical_ir::CanonicalIrArtifact,
) -> Option<PreludeInputs> {
    let Ok(mut cfg) = CfgModel::from_hir_for_executable_backend(&artifact.hir, &artifact.mir)
    else {
        println!("prelude model={module} refused=cfg-lowering");
        return None;
    };
    if model.stamp_programs.len() != cfg.residuals.len() {
        println!("prelude model={module} refused=equations-unpaired");
        return None;
    }
    let charges = stored_charges(&mut cfg.function, &cfg.residuals);
    let Ok(state) = CfgStateAllocation::build(&artifact.hir, &cfg.function) else {
        println!("prelude model={module} refused=state-allocation");
        return None;
    };
    let (seeds, correction_lane) = derivative_seeds(&cfg, &artifact.mir);
    let mut differentiated = match differentiate(&cfg.function, &seeds) {
        Ok(differentiated) => differentiated,
        Err(error) => {
            println!("prelude model={module} refused=differentiate:{error:?}");
            return None;
        }
    };
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
    let Ok(scalarized) = scalarize_lanes(&differentiated.function) else {
        println!("prelude model={module} refused=scalarize");
        return None;
    };
    let Ok(branch_unknowns) = canonical_branch_unknown_runtime_map(model, &artifact.mir) else {
        println!("prelude model={module} refused=branch-unknown-map");
        return None;
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
    let bindings = CfgRuntimeBindings::from_mir(
        module,
        &artifact.mir,
        branch_unknowns,
        event_state_variables,
    );
    let Ok(column_lanes) = ShippedColumnLanes::build(model, &artifact.mir) else {
        println!("prelude model={module} refused=lane-unmapped");
        return None;
    };

    let mut entries = Vec::new();
    for (stamp_index, stamp) in model.stamp_programs.iter().enumerate() {
        if let Some(scalar) = scalarized.scalar(cfg.residuals[stamp_index]) {
            entries.push((CfgPlanEntry::StampValue(stamp_index), scalar));
        }
        let derivatives =
            stamp
                .jacobian_programs
                .iter()
                .enumerate()
                .map(|(entry_index, jacobian)| {
                    (
                        CfgPlanEntry::Jacobian(stamp_index, entry_index),
                        &jacobian.col_axis,
                        &jacobian_rows[stamp_index],
                    )
                })
                .chain(stamp.reactive_jacobians.iter().enumerate().map(
                    |(entry_index, reactive)| {
                        (
                            CfgPlanEntry::ReactiveJacobian(stamp_index, entry_index),
                            &reactive.col_axis,
                            &reactive_rows[stamp_index],
                        )
                    },
                ));
        for (entry, axis, source) in derivatives {
            let value = column_lanes
                .lane(axis)
                .filter(|lane| Some(*lane) != correction_lane)
                .and_then(|lane| source.get(lane).copied().flatten())
                .and_then(|value| scalarized.scalar(value));
            if let Some(scalar) = value {
                entries.push((entry, scalar));
            }
        }
    }

    // The same partition the plan builder makes: an entry whose value reads a
    // contribution current keeps its own cone, because the prelude runs before
    // the first of them is published. Measuring the prelude *with* those
    // entries would measure a program the builder never builds — and, since
    // `CfgPrelude::build` refuses one, would report the module as refused.
    let taint = LiveCurrentTaint::build(&scalarized.function);
    let deferred = entries.len();
    entries.retain(|(_, value)| taint.publishable(*value));
    let deferred = deferred - entries.len();
    if deferred > 0 {
        println!("prelude model={module} live_current_entries={deferred}");
    }

    Some(PreludeInputs {
        function: scalarized.function,
        entries,
        state,
        bindings,
        slots: StateSlotMapping::build(model, &artifact.hir, &artifact.mir),
    })
}

/// Measure, per shipped module, whether one prelude fits one frame.
#[test]
#[ignore = "measurement; run with --release --features native -- --ignored --nocapture"]
fn the_cfg_prelude_frame_census() {
    let filter = std::env::var("RSPICE_CFG_CENSUS_FILTER").ok();
    let images = std::env::var("RSPICE_PRELUDE_CENSUS_IMAGE").is_ok_and(|value| value == "1");
    let mut models = 0_usize;
    let mut over_ceiling = Vec::new();
    for shipped in shipped_census_models_matching(filter.as_deref()) {
        let module = shipped.name.clone();
        models += 1;
        let Some(inputs) = prelude_inputs(&module, &shipped.model, &shipped.canonical_ir) else {
            continue;
        };

        let outputs: Vec<ValueId> = inputs.entries.iter().map(|(_, value)| *value).collect();
        let (union_function, _) = prune_cfg_to_outputs(&inputs.function, &outputs);
        let union_instructions = cfg_instructions(&union_function);
        let union_values = union_function.values.len();
        drop(union_function);

        let started = Instant::now();
        let prelude = match CfgPrelude::build(
            &module,
            &inputs.function,
            &inputs.entries,
            &inputs.state,
            &inputs.bindings,
            &inputs.slots,
        ) {
            Ok(prelude) => prelude,
            Err(refused) => {
                println!("prelude model={module} refused={refused}");
                continue;
            }
        };
        let build_seconds = started.elapsed().as_secs_f64();
        let ssa = prelude.program().ssa();
        let block_instructions = ssa.instructions().len();

        let x64_spills =
            RegisterAllocation::build(ssa, crate::native::x64::codegen::X64_VALUE_BANK)
                .map(|allocation| allocation.spill_slot_count());
        let a64_spills =
            RegisterAllocation::build(ssa, crate::native::aarch64::codegen::A64_VALUE_BANK)
                .map(|allocation| allocation.spill_slot_count());
        let report = |slots: &Result<usize, crate::jit::JitError>| match slots {
            Ok(slots) => slots.to_string(),
            Err(error) => format!("refused({error})"),
        };
        let a64_frame_bytes = a64_spills.as_ref().map(|slots| slots * WORD_BYTES);

        println!(
            "prelude model={module} entries={} slots={} union_instructions={union_instructions} \
             union_values={union_values} block_instructions={block_instructions} \
             block_over_union={:.2} x64_spill_slots={} a64_spill_slots={} a64_frame_bytes={} \
             a64_frame_ceiling_bytes={} blocks={} seconds={build_seconds:.1}",
            inputs.entries.len(),
            prelude.slot_count(),
            block_instructions as f64 / union_instructions.max(1) as f64,
            report(&x64_spills),
            report(&a64_spills),
            a64_frame_bytes
                .as_ref()
                .map_or_else(|_| "refused".to_string(), usize::to_string),
            AARCH64_SPILL_SLOT_CEILING * WORD_BYTES,
            ssa.blocks().len(),
        );

        if let Ok(slots) = &a64_spills
            && *slots >= AARCH64_SPILL_SLOT_CEILING
        {
            over_ceiling.push((module.clone(), *slots));
        }

        if images {
            let started = Instant::now();
            match crate::native::x64::codegen::compile_value_function_artifact_from_ssa(ssa) {
                Ok(artifact) => println!(
                    "prelude model={module} image bytes={} seconds={:.1}",
                    artifact.bytes().len(),
                    started.elapsed().as_secs_f64(),
                ),
                Err(error) => println!("prelude model={module} image refused={error}"),
            }
        }
    }
    println!(
        "prelude models={models} over_ceiling={}",
        over_ceiling.len()
    );
    assert!(models > 0, "the filter matched no shipped module");
    assert!(
        over_ceiling.is_empty(),
        "these modules' preludes do not fit an AArch64 spill frame of \
         {AARCH64_SPILL_SLOT_CEILING} slots and have to be chunked at dominance \
         boundaries: {over_ceiling:?}"
    );
}
