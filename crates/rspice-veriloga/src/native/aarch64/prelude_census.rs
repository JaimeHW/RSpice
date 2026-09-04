//! Does the CFG prelude encode on AArch64?
//!
//! [`the_cfg_prelude_frame_census`](crate::native::cfg_prelude_census) settled
//! that a prelude fits an AArch64 spill frame. This settles the other half:
//! whether the emitter can produce the function at all, and whether the image
//! builder will take it.
//!
//! # What used to stop it
//!
//! Two limits, both at a megabyte, and neither of them the one the plan
//! assumed. AArch64 conditional branches do reach only ±1 MiB, but the encoder
//! has never emitted a short one: `b_cond_placeholder` and the compare-and-
//! branch forms always reserve the inverse condition over an unconditional
//! `B`, whose imm26 reaches 128 MiB. What actually bound was
//!
//! 1. `LDR (literal)`, whose signed 19-bit word displacement reaches
//!    1,048,572 bytes forward — and every pooled constant used to sit in one
//!    pool after the function's `RET`; and
//! 2. the Windows ARM64 `.xdata` Function Length field, 18 bits of instruction
//!    words, which is exactly 1,048,572 bytes and is what
//!    `A64_SEGMENT_THRESHOLD_BYTES` takes its value from.
//!
//! Constants now go into inline islands as the function grows, which removes
//! the first. The second was never a limit on the code, only on how much of it
//! one `.xdata` record can name: the Windows unwind publisher describes a
//! longer function as several fragments, one `.pdata`/`.xdata` pair each, so
//! it no longer refuses either. Every other platform takes the function
//! unchanged.
//!
//! # What each line reports
//!
//! Per module: the A64 function bytes of the prelude, how many constant
//! islands the emitter had to place, whether the image builder accepted the
//! function, and how many Windows ARM64 unwind fragments it takes.
//!
//! The same run re-encodes every value entry of the *shipped postfix plan* and
//! asserts that none of them gained an island. That is the byte-identity
//! evidence: islands are the only new bytes the emitter can produce, so a plan
//! that places none encodes exactly as it did before.
//!
//! `#[ignore]`d: this is measurement work. Run it with
//! `--release --features native -- --ignored --nocapture`, narrowed with
//! `RSPICE_CFG_CENSUS_FILTER`.

use std::time::Instant;

use super::codegen::{compile_value_function, compile_value_function_from_ssa};
use super::image::{A64ImageBuilder, A64_SEGMENT_THRESHOLD_BYTES};
use super::unwind::{analyze_function, append_windows_unwind_data};
use super::verifier::verify_exact_function;
use crate::jit::cfg_prelude::CfgPrelude;
use crate::jit::plan_builder::build_model_plan_with_canonical_ir;
use crate::native::JitResult;
use crate::native::census_models::shipped_census_models_matching;
use crate::native::cfg_prelude_census::prelude_inputs;
use crate::native::model::CodeOffset;
use crate::native::model_plan::NativeModelPlan;
use crate::native::plan_program::{PlanProgram, PlanProgramRef};

/// Count the inline constant islands in one emitted function.
///
/// The walk mirrors the verifier's: an announced island is skipped whole, so
/// no constant is ever mistaken for an instruction.
fn constant_islands(bytes: &[u8]) -> usize {
    let mut islands = 0;
    let mut offset = 0;
    while offset + 4 <= bytes.len() {
        let instruction = u32::from_le_bytes(
            bytes[offset..offset + 4]
                .try_into()
                .expect("four-byte A64 word"),
        );
        if instruction & 0xFFE0_001F == 0xD420_0000 {
            islands += 1;
            offset += 4 + (((instruction >> 5) & 0xFFFF) as usize) * 8;
            continue;
        }
        if instruction & 0xFFFF_FC1F == 0xD65F_0000 {
            break;
        }
        offset += 4;
    }
    islands
}

fn compile_entry(program: PlanProgramRef<'_>) -> JitResult<Vec<u8>> {
    match program {
        PlanProgramRef::Postfix(program) => compile_value_function(program),
        PlanProgramRef::Blocks(program) => compile_value_function_from_ssa(program.ssa()),
    }
}

fn plan_value_entries(plan: &NativeModelPlan) -> Vec<&PlanProgram> {
    let mut entries: Vec<&PlanProgram> = Vec::new();
    entries.extend(plan.parameter_defaults.iter().flatten());
    entries.extend(plan.static_conditions.iter().flatten());
    entries.extend(plan.stamp_values.iter());
    entries.extend(plan.jacobians.iter().flatten());
    entries.extend(plan.reactive_jacobians.iter().flatten());
    entries.extend(plan.noise_psd.iter());
    entries.extend(plan.noise_exponents.iter().flatten());
    entries
}

/// How many `.pdata`/`.xdata` pairs Windows ARM64 needs for a function of this
/// shape.
///
/// One per fragment: a function within what the `.xdata` header's 18-bit
/// Function Length field can name is one fragment, and a longer one is
/// described by several.
fn windows_unwind_fragments(bytes: &[u8]) -> Result<usize, String> {
    let verified =
        verify_exact_function(bytes, "prelude").map_err(|error| format!("unverified({error})"))?;
    let analyzed = analyze_function(CodeOffset::new(0), &bytes[..verified.code_bytes], "prelude")
        .map_err(|error| format!("unanalyzed({error})"))?;
    let mut image = bytes[..verified.code_bytes].to_vec();
    append_windows_unwind_data(&mut image, &[analyzed])
        .map(|table| table.len())
        .map_err(|error| format!("refused({error})"))
}

#[test]
#[ignore = "measurement; run with --release --features native -- --ignored --nocapture"]
fn the_aarch64_prelude_encoding_census() {
    let filter = std::env::var("RSPICE_CFG_CENSUS_FILTER").ok();
    let mut models = 0_usize;
    let mut encoded = 0_usize;
    let mut refused = Vec::new();
    let mut postfix_islands = Vec::new();

    for shipped in shipped_census_models_matching(filter.as_deref()) {
        let module = shipped.name.clone();
        models += 1;

        // Byte identity for what ships today: the postfix route's own entries
        // must not gain a single island.
        match build_model_plan_with_canonical_ir(&shipped.model, &shipped.canonical_ir) {
            Ok(plan) => {
                let mut entries = 0_usize;
                let mut bytes = 0_usize;
                let mut islands = 0_usize;
                let mut largest = 0_usize;
                for entry in plan_value_entries(&plan) {
                    match compile_entry(entry.borrow()) {
                        Ok(encoded) => {
                            entries += 1;
                            bytes += encoded.len();
                            largest = largest.max(encoded.len());
                            islands += constant_islands(&encoded);
                        }
                        Err(error) => {
                            println!("a64-postfix model={module} entry refused={error}");
                        }
                    }
                }
                println!(
                    "a64-postfix model={module} entries={entries} bytes={bytes} \
                     largest_entry_bytes={largest} islands={islands}"
                );
                if islands > 0 {
                    postfix_islands.push((module.clone(), islands));
                }
            }
            Err(error) => println!("a64-postfix model={module} plan refused={error}"),
        }

        let Some(inputs) = prelude_inputs(&module, &shipped.model, &shipped.canonical_ir) else {
            continue;
        };
        let prelude = match CfgPrelude::build(
            &module,
            &inputs.function,
            &inputs.entries,
            &inputs.state,
            &inputs.bindings,
            &inputs.slots,
        ) {
            Ok(prelude) => prelude,
            Err(refusal) => {
                println!("a64-prelude model={module} refused={refusal}");
                continue;
            }
        };

        let started = Instant::now();
        let bytes = match compile_value_function_from_ssa(prelude.program().ssa()) {
            Ok(bytes) => bytes,
            Err(error) => {
                println!("a64-prelude model={module} encode refused={error}");
                refused.push((module.clone(), format!("{error}")));
                continue;
            }
        };
        let encode_seconds = started.elapsed().as_secs_f64();

        // The image builder is where the megabyte refusal used to live.
        let mut image = A64ImageBuilder::new();
        let published = match image.append_function(bytes.clone(), "prelude") {
            Ok(_) => "ok".to_string(),
            Err(error) => {
                refused.push((module.clone(), format!("{error}")));
                format!("refused({error})")
            }
        };

        // The Windows unwind publisher, which is what refused a prelude past a
        // megabyte before it fragmented them.
        let unwind = windows_unwind_fragments(&bytes);
        if let Err(detail) = &unwind {
            refused.push((module.clone(), detail.clone()));
        }
        let windows_unwind = match &unwind {
            Ok(fragments) => format!("fragments={fragments}"),
            Err(detail) => detail.clone(),
        };

        encoded += 1;
        println!(
            "a64-prelude model={module} slots={} block_instructions={} function_bytes={} \
             over_xdata_limit={} islands={} image={published} windows_unwind={windows_unwind} \
             seconds={encode_seconds:.1}",
            prelude.slot_count(),
            prelude.program().ssa().instructions().len(),
            bytes.len(),
            bytes.len() > A64_SEGMENT_THRESHOLD_BYTES,
            constant_islands(&bytes),
        );
    }

    println!(
        "a64-prelude models={models} encoded={encoded} refused={}",
        refused.len()
    );
    assert!(models > 0, "the filter matched no shipped module");
    assert!(
        postfix_islands.is_empty(),
        "these shipped postfix plans placed constant islands, so their A64 bytes moved: \
         {postfix_islands:?}"
    );
    assert!(
        refused.is_empty(),
        "these modules' preludes do not encode on AArch64: {refused:?}"
    );
}
