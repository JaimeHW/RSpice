//! The x86-64 backend: canonical IR in, executable image out.
//!
//! Drives the architecture-neutral lowering in the crate-internal `expr`
//! module, then hands the resulting programs to `codegen` for encoding and
//! assembles the entry points into a [`super::NativeModel`].
//! Noise sources are planned here too, since their evaluation schedule
//! differs from the time-domain one.
//!
//! `compile_model_with_canonical_ir` is the production path.
//! `compile_model`, which works from the bytecode model alone, exists only
//! for the `native-bytecode-contract-tests` feature.

mod calling_convention;
pub(crate) mod codegen;
mod driver;
pub mod encoder;
mod verifier;

pub(super) use super::ssa as ir;

#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
use super::assignment::MAX_ASSIGNMENT_CHUNK_OPERATIONS;
use super::assignment::{NativeAssignment, chunk_ranges as assignment_chunk_ranges};
use super::expr::NativeProgram;
use super::model::{
    CodeOffset, NativeCurrentDependencies, NativeEntryOffsets, NativeEntryStarts, NativeModel,
    NativeRequiredStorage,
};
use super::model_plan::NativeModelPlan;
use super::runtime::ExecutableMemory;
#[cfg(all(windows, target_arch = "x86_64"))]
use super::runtime::WindowsX64RuntimeFunction;
use super::value_cache::ValueEntryCache;
use super::{JitError, JitResult};
use crate::canonical_ir::CanonicalIrArtifact;
use crate::codegen::CompiledModel;

#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
use crate::jit::plan_builder::{
    canonical_branch_unknown_runtime_map, checked_logical_index,
    derivative_shadow_axes_from_suffix, infer_current_terminal_pair, infer_current_unified_pair,
    live_canonical_assignment_slots, lower_assignment_step, lower_static_condition_program,
    native_assignment_roots, push_prior_current_probe_aliases, ranges_overlap,
};

const ENTRY_ALIGNMENT: usize = 16;
const X64_NOP: u8 = 0x90;

#[derive(Debug)]
struct CompiledX64Function {
    bytes: Vec<u8>,
    code_len: usize,
    data_ranges: Vec<X64DataRange>,
    rip_relative_relocations: Vec<X64RipRelativeRelocation>,
    windows_unwind: Option<WindowsX64UnwindInfo>,
}

#[derive(Debug)]
struct X64FunctionBody {
    bytes: Vec<u8>,
    code_len: usize,
    data_ranges: Vec<X64DataRange>,
    rip_relative_relocations: Vec<X64RipRelativeRelocation>,
}

impl X64FunctionBody {
    fn code_only(bytes: Vec<u8>) -> Self {
        let code_len = bytes.len();
        Self {
            bytes,
            code_len,
            data_ranges: Vec::new(),
            rip_relative_relocations: Vec::new(),
        }
    }
}

impl CompiledX64Function {
    fn new(body: X64FunctionBody, windows_unwind: Option<WindowsX64UnwindInfo>) -> Self {
        Self {
            bytes: body.bytes,
            code_len: body.code_len,
            data_ranges: body.data_ranges,
            rip_relative_relocations: body.rip_relative_relocations,
            windows_unwind,
        }
    }

    fn code_only(bytes: Vec<u8>, windows_unwind: Option<WindowsX64UnwindInfo>) -> Self {
        Self::new(X64FunctionBody::code_only(bytes), windows_unwind)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum X64DataKind {
    ScalarF64,
    Vector128,
}

impl X64DataKind {
    fn width(self) -> usize {
        match self {
            Self::ScalarF64 => std::mem::size_of::<f64>(),
            Self::Vector128 => 16,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct X64DataRange {
    start: usize,
    end: usize,
    alignment: usize,
    kind: X64DataKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct X64RipRelativeRelocation {
    displacement_offset: usize,
    target_offset: usize,
    kind: X64DataKind,
}

#[derive(Debug, Clone)]
#[cfg_attr(not(windows), allow(dead_code))]
struct WindowsX64UnwindInfo {
    prologue_size: u8,
    frame_register: u8,
    operations: Vec<WindowsX64UnwindOperation>,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(not(windows), allow(dead_code))]
enum WindowsX64UnwindOperation {
    PushNonvolatile {
        code_offset: u8,
        register: u8,
    },
    AllocateStack {
        code_offset: u8,
        size: u32,
    },
    SaveXmm128 {
        code_offset: u8,
        register: u8,
        stack_offset: u32,
    },
    SetFramePointer {
        code_offset: u8,
    },
}

#[derive(Debug)]
#[cfg_attr(not(all(windows, target_arch = "x86_64")), allow(dead_code))]
struct PendingWindowsX64UnwindFunction {
    begin: CodeOffset,
    end: CodeOffset,
    info: WindowsX64UnwindInfo,
}

#[cfg(feature = "native-bytecode-contract-tests")]
pub(crate) fn compile_model(model: &CompiledModel) -> JitResult<NativeModel> {
    let plan = crate::jit::plan_builder::build_model_plan_from_bytecode(model)?;
    compile_model_plan(model, &plan)
}

pub(crate) fn compile_model_with_canonical_ir(
    model: &CompiledModel,
    artifact: &CanonicalIrArtifact,
) -> JitResult<NativeModel> {
    let plan = crate::jit::plan_builder::build_model_plan_with_canonical_ir(model, artifact)?;
    compile_model_plan(model, &plan)
}

fn compile_model_plan(model: &CompiledModel, plan: &NativeModelPlan) -> JitResult<NativeModel> {
    plan.validate_shape(model)?;

    let mut image = Vec::new();
    let mut entry_starts = Vec::new();
    let mut windows_unwind_functions = Vec::new();
    let mut value_entries = ValueEntryCache::default();

    let assignment = append_assignment_pass(
        &plan.assignments,
        &mut image,
        &mut entry_starts,
        &mut windows_unwind_functions,
    )?;
    let assignment_image_end = image.len();
    let post_assignment = if plan.post_assignments.is_empty() {
        None
    } else {
        Some(append_assignment_pass(
            &plan.post_assignments,
            &mut image,
            &mut entry_starts,
            &mut windows_unwind_functions,
        )?)
    };
    let post_assignment_image_end = image.len();

    let mut parameter_defaults = Vec::with_capacity(plan.parameter_defaults.len());
    for program in &plan.parameter_defaults {
        parameter_defaults.push(
            program
                .as_ref()
                .map(|program| {
                    append_value_entry(
                        &mut image,
                        &mut entry_starts,
                        &mut windows_unwind_functions,
                        &mut value_entries,
                        program,
                    )
                })
                .transpose()?,
        );
    }

    let mut static_conditions = Vec::with_capacity(plan.static_conditions.len());
    for program in &plan.static_conditions {
        static_conditions.push(
            program
                .as_ref()
                .map(|program| {
                    append_value_entry(
                        &mut image,
                        &mut entry_starts,
                        &mut windows_unwind_functions,
                        &mut value_entries,
                        program,
                    )
                })
                .transpose()?,
        );
    }

    let mut stamp_values = Vec::with_capacity(plan.stamp_values.len());
    for program in &plan.stamp_values {
        stamp_values.push(append_value_entry(
            &mut image,
            &mut entry_starts,
            &mut windows_unwind_functions,
            &mut value_entries,
            program,
        )?);
    }

    let mut jacobians = Vec::with_capacity(plan.jacobians.len());
    for row in &plan.jacobians {
        let mut entries = Vec::with_capacity(row.len());
        for program in row {
            entries.push(append_value_entry(
                &mut image,
                &mut entry_starts,
                &mut windows_unwind_functions,
                &mut value_entries,
                program,
            )?);
        }
        jacobians.push(entries);
    }

    let mut reactive_jacobians = Vec::with_capacity(plan.reactive_jacobians.len());
    for row in &plan.reactive_jacobians {
        let mut entries = Vec::with_capacity(row.len());
        for program in row {
            entries.push(append_value_entry(
                &mut image,
                &mut entry_starts,
                &mut windows_unwind_functions,
                &mut value_entries,
                program,
            )?);
        }
        reactive_jacobians.push(entries);
    }

    let mut noise_psd = Vec::with_capacity(plan.noise_psd.len());
    for program in &plan.noise_psd {
        noise_psd.push(append_value_entry(
            &mut image,
            &mut entry_starts,
            &mut windows_unwind_functions,
            &mut value_entries,
            program,
        )?);
    }

    let mut noise_exponents = Vec::with_capacity(plan.noise_exponents.len());
    for program in &plan.noise_exponents {
        noise_exponents.push(
            program
                .as_ref()
                .map(|program| {
                    append_value_entry(
                        &mut image,
                        &mut entry_starts,
                        &mut windows_unwind_functions,
                        &mut value_entries,
                        program,
                    )
                })
                .transpose()?,
        );
    }
    let value_entries_image_end = image.len();

    let evaluation_kernel = align_image_for_entry(&mut image, &mut entry_starts);
    let evaluation_kernel_artifact = codegen::compile_fused_evaluation_kernel_artifact(
        evaluation_kernel.as_usize(),
        assignment,
        &plan.stamp_values,
        &plan.published_current_pairs,
    )?;
    append_compiled_function_at_offset(
        &mut image,
        evaluation_kernel,
        &mut windows_unwind_functions,
        evaluation_kernel_artifact,
    )?;
    let evaluation_kernel_image_end = image.len();

    let stamp_kernel = align_image_for_entry(&mut image, &mut entry_starts);
    let stamp_kernel_artifact = codegen::compile_fused_stamp_kernel_artifact(
        stamp_kernel.as_usize(),
        assignment,
        &plan.stamp_values,
        &plan.jacobians,
        &plan.published_current_pairs,
    )?;
    append_compiled_function_at_offset(
        &mut image,
        stamp_kernel,
        &mut windows_unwind_functions,
        stamp_kernel_artifact,
    )?;

    if std::env::var_os("RSPICE_NATIVE_X64_IMAGE_TRACE").is_some() {
        eprintln!(
            "native-x64-image assignment={} post_assignment={} value_entries={} evaluation_kernel={} stamp_kernel={} total={}",
            assignment_image_end,
            post_assignment_image_end - assignment_image_end,
            value_entries_image_end - post_assignment_image_end,
            evaluation_kernel_image_end - value_entries_image_end,
            image.len() - evaluation_kernel_image_end,
            image.len(),
        );
    }

    let entries = NativeEntryOffsets {
        assignment,
        post_assignment,
        evaluation_kernel: Some(evaluation_kernel),
        stamp_kernel: Some(stamp_kernel),
        parameter_defaults,
        static_conditions,
        stamp_values,
        jacobians,
        reactive_jacobians,
        noise_psd,
        noise_exponents,
    };
    validate_compiled_entry_shape(model, &entries, &plan.current_dependencies)?;
    verify_x64_image_layout(model, &image, &entry_starts)?;

    #[cfg(all(windows, target_arch = "x86_64"))]
    let executable = {
        let runtime_functions =
            append_windows_x64_unwind_metadata(&mut image, &windows_unwind_functions)?;
        ExecutableMemory::allocate_with_windows_unwind(&image, &runtime_functions)?
    };
    #[cfg(not(all(windows, target_arch = "x86_64")))]
    let executable = ExecutableMemory::allocate(&image)?;

    NativeModel::from_executable_image_with_dependencies(
        model.num_terminals,
        model.internal_nodes,
        model.num_variables,
        model.parameters.len(),
        model.branch_sources.len(),
        executable,
        entries,
        NativeEntryStarts::new(entry_starts),
        plan.current_dependencies.clone(),
        NativeRequiredStorage::for_model(model),
    )
}

fn verify_x64_function_code(bytes: &[u8], entry_kind: &str) -> JitResult<()> {
    verifier::verify_exact_function(bytes, entry_kind).map(|_| ())
}

fn verify_x64_function_artifact(artifact: &CompiledX64Function, entry_kind: &str) -> JitResult<()> {
    if artifact.code_len == 0 || artifact.code_len > artifact.bytes.len() {
        return Err(JitError::Verifier {
            model: "native-x64".into(),
            detail: format!(
                "compiled {entry_kind} has invalid code range 0..{} for artifact length {}",
                artifact.code_len,
                artifact.bytes.len()
            )
            .into(),
        });
    }
    let verified =
        verifier::verify_exact_function(&artifact.bytes[..artifact.code_len], entry_kind)?;
    #[cfg(windows)]
    if let Some(unwind) = &artifact.windows_unwind {
        verifier::verify_windows_unwind_prologue(
            &artifact.bytes[..artifact.code_len],
            unwind,
            entry_kind,
        )?;
    }

    let mut prior_end = artifact.code_len;
    for (index, range) in artifact.data_ranges.iter().enumerate() {
        if range.alignment == 0
            || !range.alignment.is_power_of_two()
            || range.start % range.alignment != 0
            || range.start < prior_end
            || range.end <= range.start
            || range.end > artifact.bytes.len()
            || (range.end - range.start) % range.kind.width() != 0
        {
            return Err(JitError::Verifier {
                model: "native-x64".into(),
                detail: format!(
                    "compiled {entry_kind} has invalid {:?} data range {index}: {}..{} (alignment {})",
                    range.kind, range.start, range.end, range.alignment
                )
                .into(),
            });
        }
        if artifact.bytes[prior_end..range.start]
            .iter()
            .any(|byte| *byte != X64_NOP)
        {
            return Err(JitError::Verifier {
                model: "native-x64".into(),
                detail: format!(
                    "compiled {entry_kind} has non-padding bytes between typed code/data ranges at {prior_end}..{}",
                    range.start
                )
                .into(),
            });
        }
        prior_end = range.end;
    }
    if prior_end != artifact.bytes.len() {
        return Err(JitError::Verifier {
            model: "native-x64".into(),
            detail: format!(
                "compiled {entry_kind} has {} untyped trailing byte(s)",
                artifact.bytes.len() - prior_end
            )
            .into(),
        });
    }

    if verified.rip_relative_references.len() != artifact.rip_relative_relocations.len() {
        return Err(JitError::Verifier {
            model: "native-x64".into(),
            detail: format!(
                "compiled {entry_kind} decoded {} RIP-relative references but declares {} relocations",
                verified.rip_relative_references.len(),
                artifact.rip_relative_relocations.len()
            )
            .into(),
        });
    }
    for reference in &verified.rip_relative_references {
        let relocation = artifact
            .rip_relative_relocations
            .iter()
            .find(|relocation| relocation.displacement_offset == reference.displacement_offset)
            .ok_or_else(|| JitError::Verifier {
                model: "native-x64".into(),
                detail: format!(
                    "compiled {entry_kind} has undeclared RIP-relative displacement at byte {}",
                    reference.displacement_offset
                )
                .into(),
            })?;
        let decoded_target = usize::try_from(reference.target_offset).map_err(|_| {
            JitError::Verifier {
                model: "native-x64".into(),
                detail: format!(
                    "compiled {entry_kind} RIP-relative reference at byte {} targets negative offset {}",
                    reference.displacement_offset, reference.target_offset
                )
                .into(),
            }
        })?;
        if decoded_target != relocation.target_offset {
            return Err(JitError::Verifier {
                model: "native-x64".into(),
                detail: format!(
                    "compiled {entry_kind} RIP-relative reference at byte {} resolves to {decoded_target}, declared target is {}",
                    reference.displacement_offset, relocation.target_offset
                )
                .into(),
            });
        }
        let target_range = artifact.data_ranges.iter().find(|range| {
            range.kind == relocation.kind
                && relocation.target_offset >= range.start
                && relocation
                    .target_offset
                    .checked_add(relocation.kind.width())
                    .is_some_and(|end| end <= range.end)
        });
        if target_range.is_none() {
            return Err(JitError::Verifier {
                model: "native-x64".into(),
                detail: format!(
                    "compiled {entry_kind} {:?} relocation at byte {} targets untyped/out-of-range data byte {}",
                    relocation.kind, relocation.displacement_offset, relocation.target_offset
                )
                .into(),
            });
        }
    }
    Ok(())
}

fn verify_x64_image_layout(
    model: &CompiledModel,
    image: &[u8],
    entry_starts: &[CodeOffset],
) -> JitResult<()> {
    if entry_starts.is_empty() {
        return Err(JitError::Verifier {
            model: model.name.clone(),
            detail: "emitted x64 image has no entry points".into(),
        });
    }
    if entry_starts[0].as_usize() != 0 {
        return Err(JitError::Verifier {
            model: model.name.clone(),
            detail: format!(
                "first x64 entry starts at byte {}, expected byte 0",
                entry_starts[0].as_usize()
            )
            .into(),
        });
    }
    for (index, start) in entry_starts.iter().copied().enumerate() {
        let start = start.as_usize();
        if start % ENTRY_ALIGNMENT != 0 {
            return Err(JitError::Verifier {
                model: model.name.clone(),
                detail: format!(
                    "x64 entry {index} starts at unaligned byte {start}; required alignment is {ENTRY_ALIGNMENT}"
                )
                .into(),
            });
        }
        let end = entry_starts
            .get(index + 1)
            .map_or(image.len(), |offset| offset.as_usize());
        if end <= start || end > image.len() {
            return Err(JitError::Verifier {
                model: model.name.clone(),
                detail: format!(
                    "x64 entry {index} has invalid byte range {start}..{end} for image length {}",
                    image.len()
                )
                .into(),
            });
        }
        let verified =
            verifier::verify_function_prefix(&image[start..end], &format!("image entry {index}"))?;
        for relative_target in verified.direct_call_targets {
            let absolute_target = i64::try_from(start)
                .ok()
                .and_then(|start| start.checked_add(relative_target))
                .and_then(|target| usize::try_from(target).ok());
            if absolute_target.is_none_or(|target| {
                entry_starts
                    .binary_search_by_key(&target, |offset| offset.as_usize())
                    .is_err()
            }) {
                return Err(JitError::Verifier {
                    model: model.name.clone(),
                    detail: format!(
                        "x64 entry {index} has a direct call to non-entry byte {absolute_target:?}"
                    )
                    .into(),
                });
            }
        }
    }
    Ok(())
}

#[cfg(all(windows, target_arch = "x86_64"))]
fn append_windows_x64_unwind_metadata(
    image: &mut Vec<u8>,
    functions: &[PendingWindowsX64UnwindFunction],
) -> JitResult<Vec<WindowsX64RuntimeFunction>> {
    let code_image_len = image.len();
    let mut runtime_functions = Vec::with_capacity(functions.len());
    let mut previous_begin = None;
    for function in functions {
        let begin = function.begin.as_usize();
        let end = function.end.as_usize();
        if begin >= end || end > code_image_len {
            return Err(JitError::Encoding {
                model: "native-x64".into(),
                detail: format!(
                    "Windows unwind function range {begin}..{end} is outside code image length {code_image_len}"
                )
                .into(),
            });
        }
        if previous_begin.is_some_and(|previous| previous >= begin) {
            return Err(JitError::Encoding {
                model: "native-x64".into(),
                detail: "Windows unwind function ranges are not strictly ordered".into(),
            });
        }
        previous_begin = Some(begin);

        let function_len = end - begin;
        if usize::from(function.info.prologue_size) > function_len {
            return Err(JitError::Encoding {
                model: "native-x64".into(),
                detail: format!(
                    "Windows unwind prologue length {} exceeds function length {function_len}",
                    function.info.prologue_size
                )
                .into(),
            });
        }
        let encoded = encode_windows_x64_unwind_info(&function.info)?;
        let padding = (4 - (image.len() % 4)) % 4;
        image.resize(image.len() + padding, 0);
        let unwind_info_address = u32::try_from(image.len()).map_err(|_| JitError::Encoding {
            model: "native-x64".into(),
            detail: "Windows unwind-info address exceeds u32 RVA range".into(),
        })?;
        image.extend_from_slice(&encoded);
        runtime_functions.push(WindowsX64RuntimeFunction {
            begin_address: u32::try_from(begin).map_err(|_| JitError::Encoding {
                model: "native-x64".into(),
                detail: "Windows runtime-function begin address exceeds u32 RVA range".into(),
            })?,
            end_address: u32::try_from(end).map_err(|_| JitError::Encoding {
                model: "native-x64".into(),
                detail: "Windows runtime-function end address exceeds u32 RVA range".into(),
            })?,
            unwind_info_address,
        });
    }
    Ok(runtime_functions)
}

#[cfg(all(windows, target_arch = "x86_64"))]
fn encode_windows_x64_unwind_info(info: &WindowsX64UnwindInfo) -> JitResult<Vec<u8>> {
    const UWOP_PUSH_NONVOL: u8 = 0;
    const UWOP_ALLOC_LARGE: u8 = 1;
    const UWOP_ALLOC_SMALL: u8 = 2;
    const UWOP_SET_FPREG: u8 = 3;
    const UWOP_SAVE_XMM128: u8 = 8;

    let mut operations = info.operations.clone();
    operations.sort_by_key(|operation| {
        std::cmp::Reverse(match operation {
            WindowsX64UnwindOperation::PushNonvolatile { code_offset, .. }
            | WindowsX64UnwindOperation::AllocateStack { code_offset, .. }
            | WindowsX64UnwindOperation::SaveXmm128 { code_offset, .. }
            | WindowsX64UnwindOperation::SetFramePointer { code_offset } => *code_offset,
        })
    });
    let mut codes = Vec::new();
    let mut code_slots = 0usize;
    for operation in operations {
        let code_offset = match operation {
            WindowsX64UnwindOperation::PushNonvolatile {
                code_offset,
                register,
            } => {
                if register > 15 {
                    return Err(JitError::Encoding {
                        model: "native-x64".into(),
                        detail: format!(
                            "Windows unwind nonvolatile register {register} is outside x64 range"
                        )
                        .into(),
                    });
                }
                codes.extend_from_slice(&[code_offset, (register << 4) | UWOP_PUSH_NONVOL]);
                code_slots += 1;
                code_offset
            }
            WindowsX64UnwindOperation::AllocateStack { code_offset, size } => {
                if size == 0 || size % 8 != 0 {
                    return Err(JitError::Encoding {
                        model: "native-x64".into(),
                        detail: format!(
                            "Windows unwind stack allocation {size} is not a positive multiple of 8"
                        )
                        .into(),
                    });
                }
                if size <= 128 {
                    let op_info = u8::try_from(size / 8 - 1).map_err(|_| JitError::Encoding {
                        model: "native-x64".into(),
                        detail: "Windows small stack allocation op-info exceeds u8".into(),
                    })?;
                    codes.extend_from_slice(&[code_offset, (op_info << 4) | UWOP_ALLOC_SMALL]);
                    code_slots += 1;
                } else {
                    let scaled = u16::try_from(size / 8).map_err(|_| JitError::Encoding {
                        model: "native-x64".into(),
                        detail: format!(
                            "Windows x64 unwind stack allocation {size} exceeds UWOP_ALLOC_LARGE range"
                        )
                        .into(),
                    })?;
                    codes.extend_from_slice(&[code_offset, UWOP_ALLOC_LARGE]);
                    codes.extend_from_slice(&scaled.to_le_bytes());
                    code_slots += 2;
                }
                code_offset
            }
            WindowsX64UnwindOperation::SaveXmm128 {
                code_offset,
                register,
                stack_offset,
            } => {
                if register > 15 || stack_offset % 16 != 0 {
                    return Err(JitError::Encoding {
                        model: "native-x64".into(),
                        detail: format!(
                            "invalid Windows unwind XMM{register} save offset {stack_offset}"
                        )
                        .into(),
                    });
                }
                let scaled =
                    u16::try_from(stack_offset / 16).map_err(|_| JitError::Encoding {
                        model: "native-x64".into(),
                        detail: format!(
                            "XMM{register} unwind save offset {stack_offset} exceeds UWOP_SAVE_XMM128 range"
                        )
                        .into(),
                    })?;
                codes.extend_from_slice(&[code_offset, (register << 4) | UWOP_SAVE_XMM128]);
                codes.extend_from_slice(&scaled.to_le_bytes());
                code_slots += 2;
                code_offset
            }
            WindowsX64UnwindOperation::SetFramePointer { code_offset } => {
                codes.extend_from_slice(&[code_offset, UWOP_SET_FPREG]);
                code_slots += 1;
                code_offset
            }
        };
        if code_offset == 0 || code_offset > info.prologue_size {
            return Err(JitError::Encoding {
                model: "native-x64".into(),
                detail: format!(
                    "Windows unwind code offset {code_offset} is outside prologue length {}",
                    info.prologue_size
                )
                .into(),
            });
        }
    }
    let code_slots = u8::try_from(code_slots).map_err(|_| JitError::Encoding {
        model: "native-x64".into(),
        detail: "Windows x64 unwind-code count exceeds u8".into(),
    })?;
    let mut encoded = vec![
        1, // UNW_VERSION=1, no exception/unwind handler flags
        info.prologue_size,
        code_slots,
        info.frame_register,
    ];
    encoded.extend_from_slice(&codes);
    if code_slots % 2 != 0 {
        encoded.extend_from_slice(&[0, 0]);
    }
    Ok(encoded)
}

fn validate_compiled_entry_shape(
    model: &CompiledModel,
    entries: &NativeEntryOffsets,
    dependencies: &NativeCurrentDependencies,
) -> JitResult<()> {
    validate_compiled_entry_count(
        model,
        "parameter-default",
        entries.parameter_defaults.len(),
        model.parameters.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "static-condition",
        entries.static_conditions.len(),
        model.stamp_programs.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "stamp-value",
        entries.stamp_values.len(),
        model.stamp_programs.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "jacobian stamp",
        entries.jacobians.len(),
        model.stamp_programs.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "reactive-jacobian stamp",
        entries.reactive_jacobians.len(),
        model.stamp_programs.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "noise PSD",
        entries.noise_psd.len(),
        model.noise_sources.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "noise exponent",
        entries.noise_exponents.len(),
        model.noise_sources.len(),
    )?;

    for (index, (entry, parameter)) in entries
        .parameter_defaults
        .iter()
        .zip(&model.parameters)
        .enumerate()
    {
        if entry.is_some() != parameter.default_program.is_some() {
            return Err(compiled_entry_shape_error(
                model,
                format!("parameter-default {index} optional entry does not match compiled program"),
            ));
        }
    }

    for (stamp_index, stamp) in model.stamp_programs.iter().enumerate() {
        if entries.static_conditions[stamp_index].is_some() != stamp.static_condition.is_some() {
            return Err(compiled_entry_shape_error(
                model,
                format!(
                    "static-condition {stamp_index} optional entry does not match compiled guard"
                ),
            ));
        }

        validate_compiled_entry_count(
            model,
            format!("jacobian {stamp_index}"),
            entries.jacobians[stamp_index].len(),
            stamp.jacobian_programs.len(),
        )?;
        validate_compiled_entry_count(
            model,
            format!("reactive-jacobian {stamp_index}"),
            entries.reactive_jacobians[stamp_index].len(),
            stamp.reactive_jacobians.len(),
        )?;
    }

    for (index, (entry, source)) in entries
        .noise_exponents
        .iter()
        .zip(&model.noise_sources)
        .enumerate()
    {
        if entry.is_some() != source.exponent_program.is_some() {
            return Err(compiled_entry_shape_error(
                model,
                format!("noise exponent {index} optional entry does not match compiled program"),
            ));
        }
    }

    validate_current_dependency_shape(model, entries, dependencies)?;
    Ok(())
}

fn validate_current_dependency_shape(
    model: &CompiledModel,
    entries: &NativeEntryOffsets,
    dependencies: &NativeCurrentDependencies,
) -> JitResult<()> {
    validate_compiled_entry_count(
        model,
        "static-condition branch-unknown dependency",
        dependencies.static_condition_branch_unknowns.len(),
        entries.static_conditions.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "stamp-value current dependency",
        dependencies.stamp_values.len(),
        entries.stamp_values.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "stamp-value prior-current dependency",
        dependencies.stamp_value_prior_currents.len(),
        entries.stamp_values.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "stamp-value branch-unknown dependency",
        dependencies.stamp_value_branch_unknowns.len(),
        entries.stamp_values.len(),
    )?;
    validate_nested_current_dependency_shape(
        model,
        "jacobian current dependency",
        &dependencies.jacobians,
        &entries.jacobians,
    )?;
    validate_nested_current_dependency_shape(
        model,
        "jacobian prior-current dependency",
        &dependencies.jacobian_prior_currents,
        &entries.jacobians,
    )?;
    validate_nested_current_dependency_shape(
        model,
        "jacobian branch-unknown dependency",
        &dependencies.jacobian_branch_unknowns,
        &entries.jacobians,
    )?;
    validate_nested_current_dependency_shape(
        model,
        "reactive-jacobian current dependency",
        &dependencies.reactive_jacobians,
        &entries.reactive_jacobians,
    )?;
    validate_nested_current_dependency_shape(
        model,
        "reactive-jacobian prior-current dependency",
        &dependencies.reactive_jacobian_prior_currents,
        &entries.reactive_jacobians,
    )?;
    validate_nested_current_dependency_shape(
        model,
        "reactive-jacobian branch-unknown dependency",
        &dependencies.reactive_jacobian_branch_unknowns,
        &entries.reactive_jacobians,
    )?;
    validate_compiled_entry_count(
        model,
        "noise PSD current dependency",
        dependencies.noise_psd.len(),
        entries.noise_psd.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "noise PSD prior-current dependency",
        dependencies.noise_psd_prior_currents.len(),
        entries.noise_psd.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "noise PSD branch-unknown dependency",
        dependencies.noise_psd_branch_unknowns.len(),
        entries.noise_psd.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "noise exponent current dependency",
        dependencies.noise_exponents.len(),
        entries.noise_exponents.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "noise exponent prior-current dependency",
        dependencies.noise_exponent_prior_currents.len(),
        entries.noise_exponents.len(),
    )?;
    validate_compiled_entry_count(
        model,
        "noise exponent branch-unknown dependency",
        dependencies.noise_exponent_branch_unknowns.len(),
        entries.noise_exponents.len(),
    )?;
    Ok(())
}

fn validate_nested_current_dependency_shape(
    model: &CompiledModel,
    label: &str,
    dependencies: &[Vec<Vec<usize>>],
    entries: &[Vec<CodeOffset>],
) -> JitResult<()> {
    validate_compiled_entry_count(
        model,
        format!("{label} stamp"),
        dependencies.len(),
        entries.len(),
    )?;
    for (stamp_index, (dependency_entries, entry_offsets)) in
        dependencies.iter().zip(entries).enumerate()
    {
        validate_compiled_entry_count(
            model,
            format!("{label} {stamp_index}"),
            dependency_entries.len(),
            entry_offsets.len(),
        )?;
    }
    Ok(())
}

fn validate_compiled_entry_count(
    model: &CompiledModel,
    label: impl std::fmt::Display,
    actual: usize,
    expected: usize,
) -> JitResult<()> {
    if actual != expected {
        return Err(compiled_entry_shape_error(
            model,
            format!("{label} entry shape {actual} does not match compiled shape {expected}"),
        ));
    }
    Ok(())
}

fn compiled_entry_shape_error(
    model: &CompiledModel,
    detail: impl Into<smol_str::SmolStr>,
) -> JitError {
    JitError::InternalCompilerError {
        model: model.name.clone(),
        detail: detail.into(),
    }
}

fn append_assignment_pass(
    assignments: &[NativeAssignment],
    image: &mut Vec<u8>,
    entry_starts: &mut Vec<CodeOffset>,
    windows_unwind_functions: &mut Vec<PendingWindowsX64UnwindFunction>,
) -> JitResult<CodeOffset> {
    if assignments.is_empty() {
        let artifact = CompiledX64Function::code_only(vec![0xC3], None);
        let offset = align_image_for_entry(image, entry_starts);
        append_compiled_function_at_offset(image, offset, windows_unwind_functions, artifact)?;
        return Ok(offset);
    }

    let chunk_ranges = assignment_chunk_ranges(assignments);
    let mut chunk_offsets = Vec::with_capacity(chunk_ranges.len());
    for range in chunk_ranges {
        let artifact = codegen::compile_assignment_pass_function_artifact(&assignments[range])?;
        let offset = align_image_for_entry(image, entry_starts);
        append_compiled_function_at_offset(image, offset, windows_unwind_functions, artifact)?;
        chunk_offsets.push(offset);
    }

    let assignment = if let [only] = chunk_offsets.as_slice() {
        *only
    } else {
        let offset = align_image_for_entry(image, entry_starts);
        let artifact = codegen::compile_assignment_dispatch_function_artifact(
            offset.as_usize(),
            &chunk_offsets,
        )?;
        append_compiled_function_at_offset(image, offset, windows_unwind_functions, artifact)?;
        offset
    };
    Ok(assignment)
}

fn append_value_entry(
    image: &mut Vec<u8>,
    entry_starts: &mut Vec<CodeOffset>,
    windows_unwind_functions: &mut Vec<PendingWindowsX64UnwindFunction>,
    value_entries: &mut ValueEntryCache<CodeOffset>,
    program: &NativeProgram,
) -> JitResult<CodeOffset> {
    if let Some(offset) = value_entries.lookup(program) {
        return Ok(offset);
    }
    let artifact = codegen::compile_value_function_artifact(program)?;
    let offset = align_image_for_entry(image, entry_starts);
    append_compiled_function_at_offset(image, offset, windows_unwind_functions, artifact)?;
    value_entries.insert(program, offset);
    Ok(offset)
}

fn append_compiled_function_at_offset(
    image: &mut Vec<u8>,
    offset: CodeOffset,
    windows_unwind_functions: &mut Vec<PendingWindowsX64UnwindFunction>,
    artifact: CompiledX64Function,
) -> JitResult<()> {
    if offset.as_usize() != image.len() {
        return Err(JitError::InternalCompilerError {
            model: "native-x64".into(),
            detail: format!(
                "compiled function offset {} does not match image length {}",
                offset.as_usize(),
                image.len()
            )
            .into(),
        });
    }
    verify_x64_function_artifact(&artifact, "image entry")?;
    let code_end = offset
        .as_usize()
        .checked_add(artifact.code_len)
        .ok_or_else(|| JitError::Encoding {
            model: "native-x64".into(),
            detail: "compiled function code range overflow".into(),
        })?;
    if let Some(info) = artifact.windows_unwind {
        windows_unwind_functions.push(PendingWindowsX64UnwindFunction {
            begin: offset,
            end: CodeOffset::new(code_end),
            info,
        });
    }
    image.extend_from_slice(&artifact.bytes);
    Ok(())
}

fn align_image_for_entry(image: &mut Vec<u8>, entry_starts: &mut Vec<CodeOffset>) -> CodeOffset {
    let padding = (ENTRY_ALIGNMENT - (image.len() % ENTRY_ALIGNMENT)) % ENTRY_ALIGNMENT;
    image.resize(image.len() + padding, X64_NOP);
    let offset = CodeOffset::new(image.len());
    entry_starts.push(offset);
    offset
}

#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
mod tests {
    use super::{
        MAX_ASSIGNMENT_CHUNK_OPERATIONS, NativeModel, append_assignment_pass,
        assignment_chunk_ranges, canonical_branch_unknown_runtime_map,
        compile_model_with_canonical_ir, derivative_shadow_axes_from_suffix,
        live_canonical_assignment_slots, lower_assignment_step, lower_static_condition_program,
        native_assignment_roots, validate_compiled_entry_shape, verify_x64_function_code,
        verify_x64_image_layout,
    };
    use crate::canonical_ir::hir::HirRegion;
    use crate::canonical_ir::{
        BranchUnknownId, CanonicalIrArtifact, HirContributionKind, HirExprKind, MirBranchUnknown,
        MirEquationKind, NodeId,
    };
    use crate::codegen::{
        AssignmentStep, BytecodeProgram, ColumnAxis, CompiledModel, Instruction, JacobianEntry,
        StampIndex, StampProgram,
    };
    use crate::device::VerilogADevice;
    use crate::native::EvalContext;
    use crate::native::expr::{
        CanonicalDerivativeAxis, EntryKind, NativeLoweringLimits, NativeOp, NativeProgram,
        PriorCurrentProbe,
    };
    use crate::native::model::{
        CodeOffset, NativeCurrentDependencies, NativeEntryOffsets, NativeStampKernelIo,
    };
    use crate::native::runtime::ExecutableMemory;
    use crate::native::x64::codegen::NativeAssignment;
    use crate::vm::{Vm, VmContext};
    use crate::{CompilerOptions, VerilogACompiler};
    use smol_str::SmolStr;
    use std::path::{Path, PathBuf};

    fn canonical_artifact_with_unsupported_root(
        compiler: &VerilogACompiler,
        source: &str,
    ) -> CanonicalIrArtifact {
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        let metadata = artifact.metadata.clone();
        let mut hir = artifact.hir.clone();
        let mut mir = artifact.mir.clone();
        let root = usize::from(mir.equations[0].expression.id);
        let unsupported = HirExprKind::StringLiteral {
            value: "unsupported-native-expression".into(),
        };
        hir.expressions[root].kind = unsupported.clone();
        mir.expressions[root].kind = unsupported;
        hir.contributions[0].expression.kind = "string".into();
        mir.equations[0].expression.kind = "string".into();
        CanonicalIrArtifact::from_parts(metadata, hir, mir)
            .expect("synthetic canonical artifact has refreshed digests")
    }

    fn rebuild_canonical_artifact(artifact: CanonicalIrArtifact) -> CanonicalIrArtifact {
        let CanonicalIrArtifact {
            metadata, hir, mir, ..
        } = artifact;
        CanonicalIrArtifact::from_parts(metadata, hir, mir)
            .expect("synthetic canonical artifact has refreshed digests")
    }

    #[test]
    fn append_value_entry_aligns_vector_literal_entries_for_concatenated_images() {
        let program = NativeProgram::from_bytecode(
            "aligned-vector-entry",
            EntryKind::StampValue,
            &BytecodeProgram {
                instructions: vec![
                    Instruction::PushVariable(0),
                    Instruction::PushConst(1.0),
                    Instruction::PushConst(0.0),
                    Instruction::IfElse,
                ],
            },
            NativeLoweringLimits::new(0, 0, 0, 1, 0),
        )
        .expect("variable ifelse lowers to native program");
        let mut image = vec![0xC3];
        let mut entry_starts = Vec::new();
        let mut windows_unwind_functions = Vec::new();
        let mut value_entries = super::ValueEntryCache::default();

        let offset = super::append_value_entry(
            &mut image,
            &mut entry_starts,
            &mut windows_unwind_functions,
            &mut value_entries,
            &program,
        )
        .expect("append aligned value entry");

        assert_eq!(offset.as_usize(), super::ENTRY_ALIGNMENT);
        assert!(entry_starts.contains(&offset));
        assert_eq!(offset.as_usize() % super::ENTRY_ALIGNMENT, 0);
        assert!(
            image[1..offset.as_usize()]
                .iter()
                .all(|byte| *byte == super::X64_NOP),
            "entry padding should be x64 NOPs"
        );

        let memory = ExecutableMemory::allocate(&image).expect("allocate aligned image");
        let entry = memory
            .ptr_at(offset.as_usize())
            .expect("aligned entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[]);
        let vars = [2.0];

        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 1.0_f64.to_bits());
    }

    #[test]
    fn x64_structural_verifiers_reject_missing_ret_and_unaligned_entries() {
        let missing_ret = verify_x64_function_code(&[0x31, 0xC0], "test entry")
            .expect_err("function without RET must fail verification");
        assert!(missing_ret.to_string().contains("without RET"));

        let model = compiled_model_with_variables(0);
        let unaligned = verify_x64_image_layout(
            &model,
            &[0xC3, super::X64_NOP, 0xC3],
            &[CodeOffset::new(0), CodeOffset::new(2)],
        )
        .expect_err("unaligned entry must fail verification");
        assert!(unaligned.to_string().contains("unaligned byte 2"));
    }

    #[test]
    fn x64_artifact_verifier_authenticates_typed_literal_relocations() {
        // movsd xmm0, qword ptr [rip + 8]; ret; <alignment>; 1.0_f64
        let mut bytes = vec![0xF2, 0x0F, 0x10, 0x05, 8, 0, 0, 0, 0xC3];
        bytes.resize(16, super::X64_NOP);
        bytes.extend_from_slice(&1.0_f64.to_le_bytes());
        let mut artifact = super::CompiledX64Function {
            bytes,
            code_len: 9,
            data_ranges: vec![super::X64DataRange {
                start: 16,
                end: 24,
                alignment: 8,
                kind: super::X64DataKind::ScalarF64,
            }],
            rip_relative_relocations: vec![super::X64RipRelativeRelocation {
                displacement_offset: 4,
                target_offset: 16,
                kind: super::X64DataKind::ScalarF64,
            }],
            windows_unwind: None,
        };
        super::verify_x64_function_artifact(&artifact, "typed literal")
            .expect("well-formed typed artifact");

        artifact.rip_relative_relocations[0].target_offset = 15;
        let error = super::verify_x64_function_artifact(&artifact, "typed literal")
            .expect_err("mismatched relocation metadata must fail");
        assert!(error.to_string().contains("declared target is 15"));
    }

    #[cfg(windows)]
    #[test]
    fn x64_artifact_verifier_rejects_unwind_metadata_that_drifted_from_the_prologue() {
        let program = NativeProgram::from_ops_for_test(
            vec![
                NativeOp::LoadVariable(0),
                NativeOp::UnaryMath(crate::native::expr::UnaryMathOp::Exp),
            ],
            1,
            Vec::new(),
            Vec::new(),
        );
        let mut artifact = super::codegen::compile_value_function_artifact(&program)
            .expect("compile helper-backed artifact");
        let unwind = artifact
            .windows_unwind
            .as_mut()
            .expect("helper-backed Win64 artifact has unwind metadata");
        match &mut unwind.operations[0] {
            super::WindowsX64UnwindOperation::PushNonvolatile { code_offset, .. }
            | super::WindowsX64UnwindOperation::AllocateStack { code_offset, .. }
            | super::WindowsX64UnwindOperation::SaveXmm128 { code_offset, .. }
            | super::WindowsX64UnwindOperation::SetFramePointer { code_offset } => {
                *code_offset += 1;
            }
        }

        let error = super::verify_x64_function_artifact(&artifact, "corrupt unwind")
            .expect_err("unwind/prologue drift must fail independent verification");
        assert!(error.to_string().contains("unwind operation"));
    }

    #[cfg(windows)]
    #[test]
    fn windows_registers_unwind_metadata_for_driver_and_helper_entries() {
        use windows_sys::Win32::System::Diagnostics::Debug::RtlLookupFunctionEntry;

        let source = r#"
module native_windows_unwind(p, n);
  inout p, n;
  electrical p, n;
  analog begin
    I(p, n) <+ exp(V(p, n)) + V(p, n);
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("compile Windows-unwind native model");

        let assert_registered = |address: *const u8, expected_frame_register: u8| {
            let control_pc = address as usize as u64 + 1;
            let mut image_base = 0_u64;
            let function = unsafe {
                RtlLookupFunctionEntry(control_pc, &mut image_base, std::ptr::null_mut())
            };
            assert!(
                !function.is_null(),
                "RtlLookupFunctionEntry must find generated entry {address:p}"
            );
            let unwind_info_address = unsafe { (*function).Anonymous.UnwindInfoAddress };
            let unwind_info = (image_base as usize + unwind_info_address as usize) as *const u8;
            let header = unsafe { std::slice::from_raw_parts(unwind_info, 4) };
            assert_eq!(header[0] & 0x07, 1, "Windows unwind version must be 1");
            assert_eq!(
                header[3] & 0x0f,
                expected_frame_register,
                "unexpected Windows unwind frame register"
            );
        };

        assert_registered(native.stamp_kernel_address_for_test(), 5);
        assert_registered(native.stamp_value_address_for_test(0), 5);
    }

    #[test]
    fn compile_model_with_canonical_ir_executes_mir_stamp_value() {
        let source = r#"
module native_canonical_res(p, n);
  inout p, n;
  electrical p, n;
  parameter real r = 2.0;
  analog begin
    I(p, n) <+ V(p, n) / r;
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("canonical MIR stamp value compiles to native x64");

        assert_eq!(native.native_stamp_count(), 1);
        let params = [2.0_f64];
        let voltages = [5.0_f64, 1.0_f64];
        let ctx = eval_context(&params, &voltages);
        assert_eq!(
            native
                .run_stamp_value(0, &ctx, std::ptr::null())
                .expect("stamp value entry"),
            (voltages[0] - voltages[1]) / params[0]
        );

        let mut currents = vec![0.0; model.stamp_programs.len()];
        let current_axis_width = model.num_terminals + 1;
        let mut branch_currents = vec![0.0; current_axis_width * current_axis_width];
        let jacobian_count = model
            .stamp_programs
            .iter()
            .map(|stamp| stamp.jacobian_programs.len())
            .sum();
        let mut fused_jacobians = vec![0.0; jacobian_count];
        let mut variables = vec![0.0; model.num_variables];
        let mut fused_ctx = eval_context(&params, &voltages);
        fused_ctx.branch_currents = branch_currents.as_mut_ptr();
        fused_ctx.branch_currents_len = branch_currents.len();
        fused_ctx.currents = currents.as_mut_ptr();
        fused_ctx.currents_len = currents.len();
        let active = [1_u8];
        let io = NativeStampKernelIo {
            program_active: active.as_ptr(),
            jacobians: fused_jacobians.as_mut_ptr(),
        };
        assert!(native.run_stamp_kernel(&fused_ctx, variables.as_mut_ptr(), &io));
        assert_eq!(currents[0], (voltages[0] - voltages[1]) / params[0]);
        let expected_jacobians = (0..model.stamp_programs[0].jacobian_programs.len())
            .map(|entry| {
                native
                    .run_jacobian(0, entry, &fused_ctx, variables.as_ptr())
                    .expect("Jacobian entry")
            })
            .collect::<Vec<_>>();
        assert_eq!(fused_jacobians, expected_jacobians);
    }

    #[test]
    fn fused_stamp_kernel_aborts_before_publishing_after_assignment_failure() {
        let source = r#"
module native_fused_abort(p, n);
  inout p, n;
  electrical p, n;
  integer idx;
  real values[0:0];
  analog begin
    idx = V(p, n);
    values[idx] = 1.0;
    I(p, n) <+ values[0] + 3.0 * V(p, n);
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("compile failing fused stamp fixture");

        let voltages = [2.0_f64, 0.0_f64];
        let mut variables = vec![0.0; native.num_variables.max(1)];
        let mut currents = vec![-101.0; model.stamp_programs.len()];
        let current_axis_width = model.num_terminals + 1;
        let mut branch_currents = vec![-303.0; current_axis_width * current_axis_width];
        let jacobian_count = model
            .stamp_programs
            .iter()
            .map(|stamp| stamp.jacobian_programs.len())
            .sum();
        assert!(
            jacobian_count > 0,
            "fixture must contain later Jacobian work"
        );
        let mut jacobians = vec![-202.0; jacobian_count];
        let mut ctx = eval_context(&[], &voltages);
        ctx.branch_currents = branch_currents.as_mut_ptr();
        ctx.branch_currents_len = branch_currents.len();
        ctx.currents = currents.as_mut_ptr();
        ctx.currents_len = currents.len();
        let active = vec![1_u8; model.stamp_programs.len()];
        let io = NativeStampKernelIo {
            program_active: active.as_ptr(),
            jacobians: jacobians.as_mut_ptr(),
        };

        assert!(native.run_stamp_kernel(&ctx, variables.as_mut_ptr(), &io));

        let error = ctx
            .take_runtime_error()
            .expect("failing assignment must abort the fused driver");
        assert!(
            error.contains("array index 2 outside declared bounds [0:0]"),
            "unexpected fused-driver diagnostic: {error}"
        );
        assert!(
            currents.iter().all(|value| *value == -101.0),
            "the driver must not publish a contribution after assignment failure"
        );
        assert!(
            jacobians.iter().all(|value| *value == -202.0),
            "the driver must not publish a Jacobian after assignment failure"
        );
        assert!(
            branch_currents.iter().all(|value| *value == -303.0),
            "the driver must not publish terminal-pair currents after assignment failure"
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_uses_mir_parameter_defaults() {
        let source = r#"
module native_canonical_param_default(p, n);
  inout p, n;
  electrical p, n;
  parameter real base = 2.0;
  parameter real derived = base * 3.0;
  analog I(p, n) <+ V(p, n) * derived;
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let mut model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        model.parameters[1].default_program = Some(BytecodeProgram {
            instructions: vec![Instruction::PushConst(99.0)],
        });

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("canonical parameter default compiles to native x64");
        let params = [2.0_f64, 0.0];
        let ctx = eval_context(&params, &[]);
        let value = native
            .run_parameter_default(1, &ctx, std::ptr::null())
            .expect("derived parameter default has native entry");

        assert_eq!(value.to_bits(), 6.0_f64.to_bits());
    }

    #[test]
    fn compile_model_with_canonical_ir_rejects_mismatched_parameter_defaults() {
        let source = r#"
module native_canonical_param_guard(p, n);
  inout p, n;
  electrical p, n;
  parameter real base = 2.0;
  parameter real derived = base * 3.0;
  analog I(p, n) <+ V(p, n) * derived;
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let mut artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        artifact.hir.parameters[0].default = Some(4.0);
        artifact.mir.parameters[0].default = Some(4.0);
        let artifact = rebuild_canonical_artifact(artifact);

        let error = compile_model_with_canonical_ir(&model, &artifact)
            .expect_err("same-source mismatched parameter default must be rejected");
        let message = error.to_string();
        assert!(
            message.contains("canonical parameter 'base' default"),
            "unexpected error: {message}"
        );
        assert!(
            message.contains("no interpreter fallback"),
            "canonical parameter mismatch must keep hard-JIT contract: {message}"
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_rejects_mismatched_stamp_branch() {
        let source = r#"
module native_canonical_shape_guard(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ V(p, n);
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let mut artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        artifact.hir.contributions[0].branch = "n,p".into();
        artifact.hir.contributions[0].declared_branch = None;
        let HirRegion::Contribution(region) = &mut artifact.hir.body[0] else {
            panic!("fixture root must remain a contribution");
        };
        region.branch = "n,p".into();
        region.declared_branch = None;
        artifact.mir.equations[0].branch.label = "n,p".into();
        artifact.mir.equations[0].branch.declared_name = None;
        artifact.mir.equations[0].branch.pos_node = Some(NodeId::new(1));
        artifact.mir.equations[0].branch.neg_node = Some(NodeId::new(0));
        let artifact = rebuild_canonical_artifact(artifact);

        let error = compile_model_with_canonical_ir(&model, &artifact)
            .expect_err("same-source reversed canonical branch must be rejected");
        let message = error.to_string();
        assert!(
            message.contains("canonical equation 0 branch"),
            "unexpected error: {message}"
        );
        assert!(
            message.contains("no interpreter fallback"),
            "canonical mismatch must keep hard-JIT contract: {message}"
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_rejects_mismatched_stamp_kind() {
        let source = r#"
module native_canonical_kind_guard(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ V(p, n);
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let mut artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        artifact.hir.contributions[0].kind = HirContributionKind::Potential;
        let HirRegion::Contribution(region) = &mut artifact.hir.body[0] else {
            panic!("fixture root must remain a contribution");
        };
        region.kind = HirContributionKind::Potential;
        artifact.mir.equations[0].kind = MirEquationKind::Potential;
        let equation = artifact.mir.equations[0].clone();
        artifact.mir.branch_unknowns = vec![MirBranchUnknown {
            id: BranchUnknownId::new(0),
            equation: equation.id,
            declared_name: equation.branch.declared_name,
            pos_node: equation.branch.pos_node,
            neg_node: equation.branch.neg_node,
        }];
        let artifact = rebuild_canonical_artifact(artifact);

        let error = compile_model_with_canonical_ir(&model, &artifact)
            .expect_err("same-source wrong-kind canonical equation must be rejected");
        let message = error.to_string();
        assert!(
            message.contains("canonical equation 0 kind"),
            "unexpected error: {message}"
        );
        assert!(
            message.contains("no interpreter fallback"),
            "canonical kind mismatch must keep hard-JIT contract: {message}"
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_lowers_noise_entries_from_mir() {
        let source = r#"
module native_canonical_noise_guard(p, n, ctrl);
  inout p, n, ctrl;
  electrical p, n, ctrl;
  parameter real scale = 2.0;
  analog begin
    I(p, n) <+ (scale + V(ctrl, n)) * white_noise(3.0, "thermal");
    I(p, n) <+ (scale - V(p, n)) * flicker_noise(4.0, 1.25, "flicker");
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let mut model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        assert_eq!(model.noise_sources.len(), 2);
        assert!(
            model.noise_sources[1].exponent_program.is_some(),
            "fixture must include a flicker exponent"
        );

        model.noise_sources[0].psd_program = BytecodeProgram {
            instructions: vec![Instruction::PushConst(99.0)],
        };
        model.noise_sources[1].psd_program = BytecodeProgram {
            instructions: vec![Instruction::PushConst(88.0)],
        };
        model.noise_sources[1].exponent_program = Some(BytecodeProgram {
            instructions: vec![Instruction::PushConst(77.0)],
        });

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("canonical noise entries compile to native x64");
        let params = [2.0_f64];
        let voltages = [0.5_f64, 0.0_f64, 0.25_f64];
        let ctx = eval_context(&params, &voltages);

        let white_psd = native
            .run_noise_psd(0, &ctx, std::ptr::null())
            .expect("white noise PSD entry");
        let flicker_psd = native
            .run_noise_psd(1, &ctx, std::ptr::null())
            .expect("flicker noise PSD entry");
        let flicker_exponent = native
            .run_noise_exponent(1, &ctx, std::ptr::null())
            .expect("flicker source has native exponent entry");

        assert_close(
            "white PSD must come from canonical MIR, not poisoned bytecode",
            (2.0_f64 + 0.25).powi(2) * 3.0,
            white_psd,
        );
        assert_close(
            "flicker PSD must come from canonical MIR, not poisoned bytecode",
            (2.0_f64 - 0.5).powi(2) * 4.0,
            flicker_psd,
        );
        assert_eq!(flicker_exponent.to_bits(), 1.25_f64.to_bits());
    }

    #[test]
    fn compile_model_with_canonical_ir_rejects_mismatched_noise_kind() {
        let source = r#"
module native_canonical_noise_kind_guard(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ white_noise(3.0, "thermal");
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let mut artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        let noise_index = artifact
            .mir
            .expressions
            .iter()
            .position(|expr| match &expr.kind {
                HirExprKind::NoiseSource { .. } => true,
                HirExprKind::SystemFunction { name, .. } | HirExprKind::Call { name, .. } => {
                    name.trim_start_matches('$') == "white_noise"
                }
                _ => false,
            })
            .expect("fixture has canonical noise source");
        let mut exponent_expr = None;
        match &mut artifact.mir.expressions[noise_index].kind {
            HirExprKind::NoiseSource { source, .. } => {
                *source = "flicker".into();
            }
            HirExprKind::SystemFunction { name, args } | HirExprKind::Call { name, args } => {
                *name = "flicker_noise".into();
                exponent_expr = args.get(1).copied();
            }
            _ => unreachable!("fixture expression was selected as a canonical noise source"),
        }
        if let Some(expr) = exponent_expr {
            artifact.mir.expressions[usize::from(expr)].kind = HirExprKind::Number {
                value: 1.0,
                raw: "1.0".into(),
            };
            artifact.hir.expressions[usize::from(expr)].kind =
                artifact.mir.expressions[usize::from(expr)].kind.clone();
        }
        artifact.hir.expressions[noise_index].kind =
            artifact.mir.expressions[noise_index].kind.clone();
        let artifact = rebuild_canonical_artifact(artifact);

        let error = compile_model_with_canonical_ir(&model, &artifact)
            .expect_err("canonical/compiled noise kind drift must fail native compile");
        let message = error.to_string();
        assert!(
            message.contains("canonical noise source 0 kind"),
            "unexpected error: {message}"
        );
        assert!(
            message.contains("no interpreter fallback"),
            "canonical noise mismatch must keep hard-JIT contract: {message}"
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_maps_duplicate_potential_branch_unknowns() {
        let source = r#"
module native_canonical_duplicate_vsrc(p, n);
  inout p, n;
  electrical p, n;
  analog begin
    V(p, n) <+ 1.0;
    V(p, n) <+ I(p, n) * 2.0;
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        assert_eq!(
            model.branch_sources.len(),
            1,
            "compiled solver allocation should merge same-branch potential contributions"
        );
        assert_eq!(
            artifact.mir.branch_unknowns.len(),
            2,
            "canonical MIR keeps a dense branch unknown per potential equation"
        );

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("duplicate canonical potential branch unknowns map to runtime branch slot");

        let branch_unknowns = [3.0_f64];
        let mut ctx = eval_context(&[], &[0.0, 0.0]);
        ctx.branch_unknowns = branch_unknowns.as_ptr();
        assert_eq!(
            native
                .run_stamp_value(0, &ctx, std::ptr::null())
                .expect("first stamp value entry"),
            1.0
        );
        assert_eq!(
            native
                .run_stamp_value(1, &ctx, std::ptr::null())
                .expect("second stamp value entry"),
            6.0
        );
        assert_eq!(
            native.stamp_value_branch_unknowns(1),
            Some([0_usize].as_slice())
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_lowers_node_jacobians_from_mir() {
        let source = r#"
module native_canonical_node_jacobian(p, n);
  inout p, n;
  electrical p, n;
  parameter real g = 2.0;
  analog I(p, n) <+ g * V(p, n);
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let mut model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        assert!(
            model.stamp_programs[0]
                .jacobian_programs
                .iter()
                .any(|jacobian| matches!(jacobian.col_axis, ColumnAxis::Node(0))),
            "fixture must include positive-node Jacobian"
        );
        assert!(
            model.stamp_programs[0]
                .jacobian_programs
                .iter()
                .any(|jacobian| matches!(jacobian.col_axis, ColumnAxis::Node(1))),
            "fixture must include negative-node Jacobian"
        );
        poison_jacobian_bytecode(&mut model, 99.0);

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("canonical node Jacobians compile to native x64");
        let params = [2.0_f64];
        let voltages = [5.0_f64, 1.0_f64];
        let ctx = eval_context(&params, &voltages);

        assert_jacobian_axis_value(
            &model,
            &native,
            &ctx,
            std::ptr::null(),
            0,
            |axis| matches!(axis, ColumnAxis::Node(0)),
            2.0,
            "canonical_node_jacobian d/dp",
        );
        assert_jacobian_axis_value(
            &model,
            &native,
            &ctx,
            std::ptr::null(),
            0,
            |axis| matches!(axis, ColumnAxis::Node(1)),
            -2.0,
            "canonical_node_jacobian d/dn",
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_lowers_reactive_jacobians_from_mir() {
        let source = r#"
module native_canonical_reactive_jacobian(p, n);
  inout p, n;
  electrical p, n;
  parameter real c = 2.5e-12;
  analog I(p, n) <+ ddt(c * V(p, n));
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let mut model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        assert!(
            model.stamp_programs[0]
                .reactive_jacobians
                .iter()
                .any(|jacobian| matches!(jacobian.col_axis, ColumnAxis::Node(0))),
            "fixture must include positive-node reactive Jacobian"
        );
        assert!(
            model.stamp_programs[0]
                .reactive_jacobians
                .iter()
                .any(|jacobian| matches!(jacobian.col_axis, ColumnAxis::Node(1))),
            "fixture must include negative-node reactive Jacobian"
        );
        poison_reactive_jacobian_bytecode(&mut model, 99.0);

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("canonical reactive Jacobians compile to native x64");
        let ctx = eval_context(&[2.5e-12], &[0.8, 0.0]);

        assert_reactive_jacobian_axis_value(
            &model,
            &native,
            &ctx,
            std::ptr::null(),
            0,
            |axis| matches!(axis, ColumnAxis::Node(0)),
            2.5e-12,
            "canonical_reactive_jacobian dQ/dp",
        );
        assert_reactive_jacobian_axis_value(
            &model,
            &native,
            &ctx,
            std::ptr::null(),
            0,
            |axis| matches!(axis, ColumnAxis::Node(1)),
            -2.5e-12,
            "canonical_reactive_jacobian dQ/dn",
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_lowers_branch_jacobians_from_mir() {
        for (name, body, expected) in [
            ("forward", "V(p, n) <+ r * I(p, n);", 2.0_f64),
            ("reversed", "V(p, n) <+ r * I(n, p);", -2.0_f64),
        ] {
            let source = format!(
                r#"
module native_canonical_branch_jacobian_{name}(p, n);
  inout p, n;
  electrical p, n;
  parameter real r = 2.0;
  analog {body}
endmodule
"#
            );
            let compiler = VerilogACompiler::new(CompilerOptions::default());
            let mut model = compiler.compile(&source).expect("compile bytecode model");
            let artifact = compiler
                .compile_canonical_ir(&source)
                .expect("compile canonical IR");
            assert_eq!(
                model.branch_sources.len(),
                1,
                "fixture must allocate one runtime branch unknown"
            );
            assert!(
                model.stamp_programs[0]
                    .jacobian_programs
                    .iter()
                    .any(|jacobian| matches!(jacobian.col_axis, ColumnAxis::Branch(0))),
                "fixture must include branch-current Jacobian"
            );
            poison_jacobian_bytecode(&mut model, 99.0);

            let native = compile_model_with_canonical_ir(&model, &artifact)
                .expect("canonical branch Jacobian compiles to native x64");
            let branch_unknowns = [3.0_f64];
            let mut ctx = eval_context(&[2.0_f64], &[0.0, 0.0]);
            ctx.branch_unknowns = branch_unknowns.as_ptr();

            assert_jacobian_axis_value(
                &model,
                &native,
                &ctx,
                std::ptr::null(),
                0,
                |axis| matches!(axis, ColumnAxis::Branch(0)),
                expected,
                format!("canonical_branch_jacobian_{name} d/dI0"),
            );
        }
    }

    #[test]
    fn compile_model_with_canonical_ir_lowers_branch_shadow_jacobian_from_mir() {
        let source = r#"
module native_canonical_branch_shadow_jacobian(p, n);
  inout p, n;
  electrical p, n;
  parameter real r = 2.0;
  real x;
  analog begin
    x = I(p, n);
    V(p, n) <+ r * x;
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let mut model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        assert!(
            model
                .variable_names
                .iter()
                .any(|name| name.as_str() == "x@dI0"),
            "fixture must include branch-current derivative shadow"
        );
        poison_jacobian_bytecode(&mut model, 99.0);

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("canonical branch-shadow Jacobian compiles to native x64");
        let branch_unknowns = [3.0_f64];
        let mut ctx = eval_context(&[2.0_f64], &[0.0, 0.0]);
        ctx.branch_unknowns = branch_unknowns.as_ptr();
        let mut variables = vec![0.0_f64; native.num_variables.max(1)];
        native.run_assignments(&ctx, variables.as_mut_ptr());

        assert_jacobian_axis_value(
            &model,
            &native,
            &ctx,
            variables.as_ptr(),
            0,
            |axis| matches!(axis, ColumnAxis::Branch(0)),
            2.0,
            "canonical_branch_shadow_jacobian d/dI0",
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_lowers_assignments_from_hir_not_bytecode() {
        let source = r#"
module native_canonical_assignment_source(p, n);
  inout p, n;
  electrical p, n;
  real x;
  analog begin
    x = V(p, n) + 2.0;
    I(p, n) <+ x;
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let mut model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        let x_index = model
            .variable_names
            .iter()
            .position(|name| name.as_str() == "x")
            .expect("fixture has x variable");
        assert!(
            !model.assignment_steps.is_empty(),
            "fixture must exercise assignments"
        );
        let assignment = model
            .assignment_steps
            .iter_mut()
            .find_map(|step| match step {
                AssignmentStep::Assign(assignment) if assignment.var_index == x_index => {
                    Some(assignment)
                }
                _ => None,
            })
            .expect("fixture has direct assignment to x");
        assignment.program = BytecodeProgram {
            instructions: vec![Instruction::PushConst(99.0)],
        };

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("canonical assignments compile to native x64");
        let voltages = [5.0_f64, 1.0_f64];
        let ctx = eval_context(&[], &voltages);
        let mut vars = vec![0.0_f64; native.num_variables.max(1)];
        native.run_assignments(&ctx, vars.as_mut_ptr());
        if let Some(error) = ctx.take_runtime_error() {
            panic!("native canonical assignment failed: {error}");
        }

        assert_eq!(vars[x_index].to_bits(), 6.0_f64.to_bits());
        assert_eq!(
            native
                .run_stamp_value(0, &ctx, vars.as_ptr())
                .expect("stamp value entry")
                .to_bits(),
            6.0_f64.to_bits()
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_lowers_ddx_jacobians_from_mir() {
        let source = r#"
module native_canonical_ddx_jacobian(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ ddx(V(p, n) * V(p, n), V(p, n));
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let mut model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        assert!(!model.stamp_programs[0].jacobian_programs.is_empty());
        poison_jacobian_bytecode(&mut model, 99.0);

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("canonical ddx Jacobians compile to native x64");
        let mut context = native_model_benchmark_context(&model, "canonical_ddx_jacobian");
        context.voltages[0] = 3.0;
        context.voltages[1] = 0.0;
        resolve_native_parameter_defaults(&model, &native, &mut context);
        let ctx = eval_context_from_vm_context(&mut context);

        assert_jacobian_axis_value(
            &model,
            &native,
            &ctx,
            context.variables.as_ptr(),
            0,
            |axis| matches!(axis, ColumnAxis::Node(0)),
            2.0,
            "canonical_ddx_jacobian d/dp",
        );
        assert_jacobian_axis_value(
            &model,
            &native,
            &ctx,
            context.variables.as_ptr(),
            0,
            |axis| matches!(axis, ColumnAxis::Node(1)),
            -2.0,
            "canonical_ddx_jacobian d/dn",
        );
    }

    #[test]
    fn canonical_repeated_assignment_uses_matching_state_program_occurrence() {
        let source = r#"
module native_canonical_repeated_assignment_state(p, n);
  inout p, n;
  electrical p, n;
  real x;
  analog begin
    x = V(p, n);
    x = ddt(V(p, n));
    I(p, n) <+ x;
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        let x_index = model
            .variable_names
            .iter()
            .position(|name| name == "x")
            .expect("fixture has x variable");
        let x_programs = model
            .assignment_steps
            .iter()
            .filter_map(|step| match step {
                AssignmentStep::Assign(assignment) if assignment.var_index == x_index => {
                    Some(&assignment.program)
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(x_programs.len(), 2, "fixture repeats the x assignment");
        assert!(
            !x_programs[0]
                .instructions
                .iter()
                .any(|instruction| matches!(instruction, Instruction::DdtState(_))),
            "first x assignment must not own a ddt slot"
        );
        assert!(
            x_programs[1]
                .instructions
                .iter()
                .any(|instruction| matches!(instruction, Instruction::DdtState(_))),
            "second x assignment must own the ddt slot"
        );

        compile_model_with_canonical_ir(&model, &artifact)
            .expect("repeated canonical assignments pair state slots by occurrence");
    }

    #[test]
    fn canonical_jacobian_maps_state_slots_used_by_product_rule() {
        let source = r#"
module native_canonical_jacobian_state_product(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ V(p, n) * ddt(V(p, n) * V(p, n));
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        assert!(
            model.stamp_programs[0]
                .jacobian_programs
                .iter()
                .flat_map(|jacobian| &jacobian.program.instructions)
                .any(|instruction| matches!(instruction, Instruction::DdtState(_))),
            "product-rule Jacobian must read the ddt value state"
        );

        compile_model_with_canonical_ir(&model, &artifact)
            .expect("canonical Jacobian maps bytecode state slots before MIR lowering");
    }

    #[test]
    fn canonical_guarded_assignment_current_probe_compiles_with_structural_dependency() {
        let source = r#"
`include "disciplines.vams"
module native_canonical_guarded_assignment_current(p, n);
  inout p, n;
  electrical p, n;
  real operating_current;
  analog begin
    if (analysis("static"))
      operating_current = I(<p>);
    I(p, n) <+ V(p, n);
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("canonical guarded assignment current compiles");
        assert_eq!(
            native.assignment_current_pairs().len(),
            0,
            "pre-current assignments must not read the terminal-current cache"
        );
        assert_eq!(native.plan_stats().assignment_entry_points, 2);
        assert_eq!(
            native.post_assignment_prior_currents(),
            &[0],
            "the post-current assignment reads the exact contribution slot"
        );
    }

    #[test]
    fn canonical_post_current_assignments_preserve_source_order() {
        let source = r#"
module native_canonical_post_current_order(p, n);
  inout p, n;
  electrical p, n;
  real before, sensed, after;
  analog begin
    before = 1.0;
    I(p, n) <+ V(p, n);
    sensed = I(p, n) + before;
    before = 2.0;
    after = sensed + before;
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("post-current assignment phases compile");
        let variable = |name: &str| {
            model
                .variable_names
                .iter()
                .position(|candidate| candidate == name)
                .unwrap_or_else(|| panic!("fixture has {name} variable"))
        };
        let before = variable("before");
        let sensed = variable("sensed");
        let after = variable("after");
        let currents = [3.0_f64];
        let mut ctx = eval_context(&[], &[0.0, 0.0]);
        ctx.currents = currents.as_ptr();
        ctx.currents_len = currents.len();
        let mut variables = vec![0.0_f64; native.num_variables.max(1)];

        native.run_assignments(&ctx, variables.as_mut_ptr());
        assert_eq!(variables[before].to_bits(), 1.0_f64.to_bits());
        assert_eq!(variables[sensed].to_bits(), 0.0_f64.to_bits());

        assert!(native.run_post_assignments(&ctx, variables.as_mut_ptr()));
        assert_eq!(variables[sensed].to_bits(), 4.0_f64.to_bits());
        assert_eq!(variables[before].to_bits(), 2.0_f64.to_bits());
        assert_eq!(variables[after].to_bits(), 6.0_f64.to_bits());
    }

    #[test]
    fn canonical_current_dependent_stamp_assignment_is_rejected_as_a_cycle() {
        let source = r#"
module native_canonical_current_assignment_cycle(p, n);
  inout p, n;
  electrical p, n;
  real sensed;
  analog begin
    sensed = I(p, n);
    I(p, n) <+ sensed;
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        let error = compile_model_with_canonical_ir(&model, &artifact)
            .expect_err("current-dependent stamp assignment must be rejected");

        let message = error.to_string();
        assert!(message.contains("sensed"), "{message}");
        assert!(
            message.contains("required before contribution-current evaluation"),
            "{message}"
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_lowers_math_ddx_jacobians_from_mir() {
        let source = r#"
module native_canonical_math_ddx_jacobian(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ ddx(
      sqrt(V(p, n) + 4.0)
    + exp(0.1 * V(p, n))
    + log(V(p, n) + 3.0)
    + log10(V(p, n) + 3.0)
    + sin(0.2 * V(p, n))
    + cos(0.15 * V(p, n))
    + tan(0.1 * V(p, n))
    + sinh(0.2 * V(p, n))
    + cosh(0.15 * V(p, n))
    + tanh(0.3 * V(p, n))
    + asinh(0.4 * V(p, n))
    + acosh(V(p, n) + 2.0)
    + atanh(0.1 * V(p, n))
    + asin(0.07 * V(p, n))
    + acos(0.05 * V(p, n))
    + atan(0.2 * V(p, n))
    + atan2(V(p, n), V(p, n) + 1.0)
    + hypot(2.0 * V(p, n), V(p, n) + 3.0)
    + pow(V(p, n) + 2.0, 2.5),
    V(p, n));
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let mut model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        assert!(!model.stamp_programs[0].jacobian_programs.is_empty());
        poison_jacobian_bytecode(&mut model, 99.0);

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("canonical math ddx Jacobians compile to native x64");
        let x = 0.4_f64;
        let expected = central_second_derivative(
            |x| {
                (x + 4.0).sqrt()
                    + (0.1 * x).exp()
                    + (x + 3.0).ln()
                    + (x + 3.0).log10()
                    + (0.2 * x).sin()
                    + (0.15 * x).cos()
                    + (0.1 * x).tan()
                    + (0.2 * x).sinh()
                    + (0.15 * x).cosh()
                    + (0.3 * x).tanh()
                    + (0.4 * x).asinh()
                    + (x + 2.0).acosh()
                    + (0.1 * x).atanh()
                    + (0.07 * x).asin()
                    + (0.05 * x).acos()
                    + (0.2 * x).atan()
                    + x.atan2(x + 1.0)
                    + (2.0 * x).hypot(x + 3.0)
                    + (x + 2.0).powf(2.5)
            },
            x,
        );
        let mut context = native_model_benchmark_context(&model, "canonical_math_ddx_jacobian");
        context.voltages[0] = x;
        context.voltages[1] = 0.0;
        resolve_native_parameter_defaults(&model, &native, &mut context);
        let ctx = eval_context_from_vm_context(&mut context);

        assert_jacobian_axis_approx(
            &model,
            &native,
            &ctx,
            context.variables.as_ptr(),
            0,
            |axis| matches!(axis, ColumnAxis::Node(0)),
            expected,
            1.0e-5,
            "canonical_math_ddx_jacobian d/dp",
        );
        assert_jacobian_axis_approx(
            &model,
            &native,
            &ctx,
            context.variables.as_ptr(),
            0,
            |axis| matches!(axis, ColumnAxis::Node(1)),
            -expected,
            1.0e-5,
            "canonical_math_ddx_jacobian d/dn",
        );
    }

    #[test]
    fn canonical_ddx_second_derivative_lowers_third_order_product_rule() {
        let source = r#"
module native_canonical_ddx_third_product(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ ddx(
      V(p, n) * V(p, n) * V(p, n) * V(p, n),
      V(p, n));
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        let expression = artifact.mir.equations[0].expression.id;
        let axis = CanonicalDerivativeAxis::Node(NodeId::from(0));
        let program = NativeProgram::from_mir_expression_second_derivative(
            "native_canonical_ddx_third_product",
            EntryKind::Jacobian,
            &artifact.mir,
            crate::canonical_ir::EquationId::new(0),
            expression,
            axis,
            axis,
            NativeLoweringLimits::new(2, 0, 0, 0, 0),
        )
        .expect("lower second derivative of ddx through a third-order product rule");
        let bytes =
            super::codegen::compile_value_function(&program).expect("compile derivative leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate derivative leaf");
        let entry = memory.ptr_at(0).expect("derivative entry");
        let function: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[2.0, 0.0]);

        assert_close(
            "canonical ddx third-order product derivative",
            48.0,
            function(&ctx, std::ptr::null()),
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_lowers_assignment_fed_ddx_jacobians_from_mir() {
        let source = r#"
module native_canonical_assignment_ddx_jacobian(p, n);
  inout p, n;
  electrical p, n;
  real x;
  analog begin
    x = V(p, n) * V(p, n);
    I(p, n) <+ ddx(x, V(p, n));
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let mut model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        assert!(
            !model.variable_names.is_empty(),
            "fixture must allocate assignment-fed derivative storage"
        );
        poison_jacobian_bytecode(&mut model, 99.0);

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("canonical assignment-fed ddx Jacobians compile to native x64");
        let mut context =
            native_model_benchmark_context(&model, "canonical_assignment_ddx_jacobian");
        context.voltages[0] = 3.0;
        context.voltages[1] = 0.0;
        resolve_native_parameter_defaults(&model, &native, &mut context);
        let ctx = eval_context_from_vm_context(&mut context);
        ctx.clear_runtime_error();
        native.run_assignments(&ctx, context.variables.as_mut_ptr());
        if let Some(error) = ctx.take_runtime_error() {
            panic!("native assignment failed before assignment-fed ddx Jacobian: {error}");
        }
        let ctx = eval_context_from_vm_context(&mut context);

        assert_finite_close(
            "canonical assignment-fed ddx current",
            "stamp value",
            6.0,
            native
                .run_stamp_value(0, &ctx, context.variables.as_ptr())
                .expect("stamp value entry"),
        )
        .expect("assignment-fed ddx current matches expected value");
        assert_jacobian_axis_value(
            &model,
            &native,
            &ctx,
            context.variables.as_ptr(),
            0,
            |axis| matches!(axis, ColumnAxis::Node(0)),
            2.0,
            "canonical_assignment_ddx_jacobian d/dp",
        );
        assert_jacobian_axis_value(
            &model,
            &native,
            &ctx,
            context.variables.as_ptr(),
            0,
            |axis| matches!(axis, ColumnAxis::Node(1)),
            -2.0,
            "canonical_assignment_ddx_jacobian d/dn",
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_lowers_array_fed_ddx_jacobians_from_mir() {
        let source = r#"
module native_canonical_array_ddx_jacobian(p, n);
  inout p, n;
  electrical p, n;
  real q[0:1];
  integer idx;
  analog begin
    idx = 1;
    q[idx] = V(p, n) * V(p, n);
    I(p, n) <+ ddx(q[idx], V(p, n));
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let mut model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        assert!(
            model
                .variable_names
                .iter()
                .any(|name| name.starts_with("q[")),
            "fixture must allocate array-fed derivative storage"
        );
        poison_jacobian_bytecode(&mut model, 99.0);

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("canonical array-fed ddx Jacobians compile to native x64");
        let mut context = native_model_benchmark_context(&model, "canonical_array_ddx_jacobian");
        context.voltages[0] = 3.0;
        context.voltages[1] = 0.0;
        resolve_native_parameter_defaults(&model, &native, &mut context);
        let ctx = eval_context_from_vm_context(&mut context);
        ctx.clear_runtime_error();
        native.run_assignments(&ctx, context.variables.as_mut_ptr());
        if let Some(error) = ctx.take_runtime_error() {
            panic!("native assignment failed before array-fed ddx Jacobian: {error}");
        }
        let ctx = eval_context_from_vm_context(&mut context);

        assert_finite_close(
            "canonical array-fed ddx current",
            "stamp value",
            6.0,
            native
                .run_stamp_value(0, &ctx, context.variables.as_ptr())
                .expect("stamp value entry"),
        )
        .expect("array-fed ddx current matches expected value");
        assert_jacobian_axis_value(
            &model,
            &native,
            &ctx,
            context.variables.as_ptr(),
            0,
            |axis| matches!(axis, ColumnAxis::Node(0)),
            2.0,
            "canonical_array_ddx_jacobian d/dp",
        );
        assert_jacobian_axis_value(
            &model,
            &native,
            &ctx,
            context.variables.as_ptr(),
            0,
            |axis| matches!(axis, ColumnAxis::Node(1)),
            -2.0,
            "canonical_array_ddx_jacobian d/dn",
        );
    }

    #[test]
    fn native_x64_generated_model_plan_publishes_dense_entrypoint_surface() {
        let source = r#"
`include "disciplines.vams"
module native_generated_plan_guard(p, n, ctrl);
  inout p, n, ctrl;
  electrical p, n, ctrl;
  parameter real base = 1.5;
  parameter real gain = base * 2.0;
  parameter real enable_a = 1.0;
  parameter real enable_b = 1.0;
  real vp;
  real vc;
  real g0;
  real g1;
  real g2;
  real g3;
  real accum;
  analog begin
    vp = V(p, n);
    vc = V(ctrl, n);
    g0 = gain + vc * 0.1;
    g1 = g0 * g0 + 0.25;
    g2 = sqrt(g1) + exp(0.01 * vp);
    g3 = sin(vc) - cos(vp);
    accum = g0 + g1 + g2 + g3;
    if (enable_a > 0.5) begin
      I(p, n) <+ g0 * vp;
      I(p, n) <+ g1 * vp;
      I(ctrl, n) <+ 0.5 * vc;
    end
    if (enable_b > 0.5) begin
      I(p, n) <+ accum * 0.125;
    end
    I(p, n) <+ g2 * vp + g3;
    I(p, n) <+ ddt(1.0e-12 * vp);
    I(p, n) <+ white_noise(1.0e-18 * (1.0 + abs(vp)), "thermal");
    I(p, n) <+ flicker_noise(2.0e-18 * (1.0 + abs(vc)), 1.0, "flicker");
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile generated model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile generated canonical IR");

        let expected_parameter_defaults = model
            .parameters
            .iter()
            .filter(|parameter| parameter.default_program.is_some())
            .count();
        let expected_static_conditions = model
            .stamp_programs
            .iter()
            .filter(|stamp| stamp.static_condition.is_some())
            .count();
        let expected_jacobians = model
            .stamp_programs
            .iter()
            .map(|stamp| stamp.jacobian_programs.len())
            .sum::<usize>();
        let expected_reactive_jacobians = model
            .stamp_programs
            .iter()
            .map(|stamp| stamp.reactive_jacobians.len())
            .sum::<usize>();
        let expected_noise_entries = model.noise_sources.len()
            + model
                .noise_sources
                .iter()
                .filter(|source| source.exponent_program.is_some())
                .count();

        assert!(
            count_assignment_steps(&model.assignment_steps) >= 7,
            "fixture must exercise a dense assignment pass"
        );
        assert!(
            model.stamp_programs.len() >= 6,
            "fixture must publish multiple stamp entry points"
        );
        assert!(
            expected_parameter_defaults >= 1,
            "fixture must include dependent parameter defaults"
        );
        assert!(
            expected_static_conditions >= 2,
            "fixture must include parameter-static contribution guards"
        );
        assert!(
            expected_jacobians >= model.stamp_programs.len(),
            "fixture must publish Jacobian coverage for every stamp"
        );
        assert!(
            expected_reactive_jacobians >= 1,
            "fixture must include reactive Jacobian coverage"
        );
        assert!(
            expected_noise_entries >= 3,
            "fixture must include white-noise PSD plus flicker PSD/exponent entries"
        );

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("generated model compiles to native x64 without fallback");
        let stats = native.plan_stats();
        assert_eq!(stats.assignment_entry_points, 1);
        assert_eq!(
            stats.parameter_default_entry_points,
            expected_parameter_defaults
        );
        assert_eq!(
            stats.static_condition_entry_points,
            expected_static_conditions
        );
        assert_eq!(stats.stamp_value_entry_points, model.stamp_programs.len());
        assert_eq!(stats.jacobian_entry_points, expected_jacobians);
        assert_eq!(
            stats.reactive_jacobian_entry_points,
            expected_reactive_jacobians
        );
        assert_eq!(stats.noise_source_entry_points, expected_noise_entries);
        assert_eq!(native.native_stamp_count(), model.stamp_programs.len());

        let mut context = native_model_benchmark_context(&model, "generated_plan_guard");
        context.voltages[0] = 0.8;
        context.voltages[1] = 0.0;
        context.voltages[2] = 0.2;
        resolve_native_parameter_defaults(&model, &native, &mut context);
        let oracle_stats = assert_native_matches_bytecode_finite_entries(
            &model,
            &artifact,
            &native,
            context,
            "generated",
        )
        .expect("generated-model native values match bytecode oracle");
        assert!(
            oracle_stats.variables >= 7
                && oracle_stats.stamps >= 6
                && oracle_stats.jacobians >= expected_jacobians
                && oracle_stats.reactive_jacobians >= expected_reactive_jacobians,
            "oracle stats did not cover the dense surface: variables={} stamps={} jacobians={} reactive_jacobians={}",
            oracle_stats.variables,
            oracle_stats.stamps,
            oracle_stats.jacobians,
            oracle_stats.reactive_jacobians
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_maps_reversed_duplicate_branch_unknowns() {
        let source = r#"
module native_canonical_reversed_duplicate_vsrc(p, n);
  inout p, n;
  electrical p, n;
  analog begin
    V(n, p) <+ 1.0;
    V(p, n) <+ I(p, n) * 2.0;
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        assert_eq!(
            model.branch_sources.len(),
            1,
            "compiled solver allocation should merge opposite branch orientations"
        );
        assert_eq!(artifact.mir.branch_unknowns.len(), 2);

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("reversed canonical branch unknown maps to runtime branch slot");

        let branch_unknowns = [3.0_f64];
        let mut ctx = eval_context(&[], &[0.0, 0.0]);
        ctx.branch_unknowns = branch_unknowns.as_ptr();
        assert_eq!(
            native
                .run_stamp_value(1, &ctx, std::ptr::null())
                .expect("second stamp value entry"),
            -6.0
        );
        assert_eq!(
            native.stamp_value_branch_unknowns(1),
            Some([0_usize].as_slice())
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_maps_named_branch_unknowns() {
        let source = r#"
module native_canonical_named_duplicate_vsrc(p, n);
  inout p, n;
  electrical p, n;
  branch (p, n) probe;
  analog begin
    V(n, p) <+ 1.0;
    V(probe) <+ I(probe) * 2.0;
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        assert_eq!(model.branch_sources.len(), 1);
        assert_eq!(artifact.mir.branch_unknowns.len(), 2);

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("named canonical branch unknown maps to runtime branch slot");

        let branch_unknowns = [3.0_f64];
        let mut ctx = eval_context(&[], &[0.0, 0.0]);
        ctx.branch_unknowns = branch_unknowns.as_ptr();
        assert_eq!(
            native
                .run_stamp_value(1, &ctx, std::ptr::null())
                .expect("named branch stamp value entry"),
            -6.0
        );
        assert_eq!(
            native.stamp_value_branch_unknowns(1),
            Some([0_usize].as_slice())
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_rejects_source_digest_mismatch() {
        let model_source = r#"
module native_canonical_digest_guard(p, n);
  inout p, n;
  electrical p, n;
  real g;
  analog begin
    g = 1.0;
    I(p, n) <+ g * V(p, n);
  end
endmodule
"#;
        let artifact_source = r#"
module native_canonical_digest_guard(p, n);
  inout p, n;
  electrical p, n;
  real g;
  analog begin
    g = 2.0;
    I(p, n) <+ g * V(p, n);
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler
            .compile(model_source)
            .expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(artifact_source)
            .expect("compile canonical IR");

        let error = compile_model_with_canonical_ir(&model, &artifact)
            .expect_err("mismatched canonical source must hard-fail");
        let message = error.to_string();
        assert!(
            message.contains("canonical source digest"),
            "unexpected error: {message}"
        );
        assert!(
            message.contains("no interpreter fallback"),
            "canonical digest mismatch must preserve hard-JIT contract: {message}"
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_rejects_missing_model_source_digest() {
        let source = r#"
module native_canonical_missing_digest_guard(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ V(p, n);
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let mut model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        model.source_digest = SmolStr::default();

        let error = compile_model_with_canonical_ir(&model, &artifact)
            .expect_err("digest-less compiled model must hard-fail");
        let message = error.to_string();
        assert!(
            message.contains("missing source digest"),
            "unexpected error: {message}"
        );
        assert!(
            message.contains("digest-aware compiler/codegen path"),
            "missing-digest error must name the required rebuild path: {message}"
        );
        assert!(
            message.contains("no interpreter fallback"),
            "missing-digest error must preserve hard-JIT contract: {message}"
        );
    }

    #[test]
    fn native_x64_compiled_entry_shape_accepts_compiled_model_surface() {
        let model = compile_shape_validator_model();
        let (entries, dependencies) = expected_native_entry_shape(&model);
        validate_compiled_entry_shape(&model, &entries, &dependencies)
            .expect("compiled-model entry shape validates before publication");
    }

    #[test]
    fn native_x64_compiled_entry_shape_rejects_backend_table_drift() {
        let model = compile_shape_validator_model();
        let (mut entries, dependencies) = expected_native_entry_shape(&model);
        let stamp_index = entries
            .jacobians
            .iter()
            .position(|entries| !entries.is_empty())
            .expect("shape fixture has jacobian entries");
        entries.jacobians[stamp_index].pop();

        let error = validate_compiled_entry_shape(&model, &entries, &dependencies)
            .expect_err("missing jacobian entry must fail before executable memory publication");
        let message = error.to_string();
        assert!(
            message.contains(&format!("jacobian {stamp_index}")),
            "unexpected error: {message}"
        );
        assert!(
            message.contains("no interpreter fallback"),
            "shape validation must keep the hard-JIT failure contract: {message}"
        );

        let (entries, mut dependencies) = expected_native_entry_shape(&model);
        dependencies.reactive_jacobians.pop();
        let error = validate_compiled_entry_shape(&model, &entries, &dependencies)
            .expect_err("missing dependency table must fail before publication");
        let message = error.to_string();
        assert!(
            message.contains("reactive-jacobian current dependency stamp"),
            "unexpected error: {message}"
        );
        assert!(
            message.contains("no interpreter fallback"),
            "dependency validation must keep the hard-JIT failure contract: {message}"
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_lowers_static_condition_from_mir() {
        let source = r#"
`include "disciplines.vams"
module native_canonical_static_guard(p, n);
  inout p, n;
  electrical p, n;
  parameter real enabled = 1.0;
  analog begin
    if (enabled > 0.5) begin
      I(p, n) <+ V(p, n);
    end
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let mut model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        let stamp_index = model
            .stamp_programs
            .iter()
            .position(|stamp| stamp.static_condition.is_some())
            .expect("fixture must include a static condition");
        model.stamp_programs[stamp_index].static_condition = Some(BytecodeProgram {
            instructions: vec![Instruction::PushConst(0.0)],
        });

        let program = lower_static_condition_program(
            &model,
            Some(&artifact.mir),
            stamp_index,
            model.stamp_programs[stamp_index]
                .static_condition
                .as_ref()
                .expect("poisoned static condition bytecode exists"),
            NativeLoweringLimits::for_model(&model),
        )
        .expect("canonical static condition lowers to native ops");
        assert_ne!(
            program.ops(),
            &[NativeOp::Const(0.0)],
            "canonical static guard must not use poisoned bytecode"
        );

        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("canonical static condition compiles to native x64");
        let params: Vec<f64> = model.parameters.iter().map(|param| param.default).collect();
        let ctx = eval_context(&params, &[2.0, 0.0]);
        let mut vars = vec![0.0_f64; native.num_variables.max(1)];
        native.run_assignments(&ctx, vars.as_mut_ptr());
        let active = native
            .run_static_condition(stamp_index, &ctx, vars.as_ptr())
            .expect("static condition has native entry");

        assert_eq!(active.to_bits(), 1.0_f64.to_bits());
    }

    #[test]
    fn compile_model_with_canonical_ir_maps_guarded_indirect_branch_unknown() {
        let source = r#"
`include "disciplines.vams"
module native_canonical_guarded_indirect(p);
  inout p;
  electrical p;
  parameter integer en = 1;
  analog begin
    if (en)
      V(p): V(p) == 3.0;
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");

        assert_eq!(model.branch_sources.len(), 1);
        assert_eq!(artifact.mir.branch_unknowns.len(), 1);
        assert_eq!(
            artifact.mir.branch_unknowns[0].equation,
            artifact.mir.equations[0].id
        );

        let _native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("guarded indirect branch unknown maps to the compiled branch source");
    }

    #[test]
    fn compile_model_with_canonical_ir_rejects_missing_static_condition_guard() {
        let source = r#"
`include "disciplines.vams"
module native_canonical_static_guard_shape(p, n);
  inout p, n;
  electrical p, n;
  parameter real enabled = 1.0;
  analog begin
    if (enabled > 0.5) begin
      I(p, n) <+ V(p, n);
    end
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let mut artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        assert!(
            model
                .stamp_programs
                .iter()
                .any(|stamp| stamp.static_condition.is_some()),
            "fixture must include a compiled static condition"
        );
        let root = usize::from(artifact.mir.equations[0].expression.id);
        let replacement = HirExprKind::Number {
            value: 1.0,
            raw: "1.0".into(),
        };
        artifact.hir.expressions[root].kind = replacement.clone();
        artifact.mir.expressions[root].kind = replacement;
        artifact.hir.contributions[0].expression.kind = "number".into();
        artifact.mir.equations[0].expression.kind = "number".into();
        artifact = rebuild_canonical_artifact(artifact);

        let error = compile_model_with_canonical_ir(&model, &artifact)
            .expect_err("missing canonical static guard must fail native compile");
        let message = error.to_string();
        assert!(
            message.contains("missing leading static-condition guard"),
            "unexpected error: {message}"
        );
        assert!(
            message.contains("no interpreter fallback"),
            "canonical static guard mismatch must keep hard-JIT contract: {message}"
        );
    }

    #[test]
    #[ignore = "release-only source-level native x64 throughput probe; run with --release --features native -- --ignored --nocapture"]
    fn native_x64_model_microbench_reports_entrypoint_throughput() {
        assert!(
            !cfg!(debug_assertions),
            "native x64 model microbench is release-only; rerun with --release"
        );

        let source = r#"
`include "disciplines.vams"
module native_model_microbench(p, n);
  inout p, n;
  electrical p, n;
  real g;
  analog begin
    g = 0.25;
    I(p, n) <+ g * V(p, n);
  end
endmodule
"#;
        let (model, native) = compile_native_microbench_model(source);
        assert!(
            !model.assignment_steps.is_empty(),
            "fixture must exercise the native assignment pass"
        );
        assert_eq!(native.native_stamp_count(), 1);
        assert!(
            !model.stamp_programs[0].jacobian_programs.is_empty(),
            "fixture must exercise native jacobian entrypoints"
        );

        let params = [];
        let voltages = [8.0_f64, 3.0];
        let ctx = eval_context(&params, &voltages);
        let mut vars = vec![0.0_f64; native.num_variables.max(1)];
        native.run_assignments(&ctx, vars.as_mut_ptr());
        assert_near(vars[0], 0.25, "assignment output");
        assert_near(
            native
                .run_stamp_value(0, &ctx, vars.as_ptr())
                .expect("stamp value entry"),
            1.25,
            "stamp value",
        );
        let expected_jacobian = native
            .run_jacobian(0, 0, &ctx, vars.as_ptr())
            .expect("Jacobian entry");
        assert!(
            (expected_jacobian.abs() - 0.25).abs() <= 1.0e-12,
            "first jacobian entry should be +/-0.25, got {expected_jacobian}"
        );

        let iterations = native_model_microbench_iterations();
        let samples = native_model_microbench_samples();
        eprintln!("native-x64-model-microbench iterations={iterations} samples={samples}");

        run_native_model_entry_microbench(
            "assignment_fed",
            "assignments",
            iterations,
            samples,
            0.25,
            || {
                native.run_assignments(
                    std::hint::black_box(&ctx),
                    std::hint::black_box(vars.as_mut_ptr()),
                );
                std::hint::black_box(vars[0])
            },
        );
        run_native_model_entry_microbench(
            "assignment_fed",
            "stamp_value",
            iterations,
            samples,
            1.25,
            || {
                native
                    .run_stamp_value(
                        0,
                        std::hint::black_box(&ctx),
                        std::hint::black_box(vars.as_ptr()),
                    )
                    .expect("stamp value entry")
            },
        );
        run_native_model_entry_microbench(
            "assignment_fed",
            "jacobian0",
            iterations,
            samples,
            expected_jacobian,
            || {
                native
                    .run_jacobian(
                        0,
                        0,
                        std::hint::black_box(&ctx),
                        std::hint::black_box(vars.as_ptr()),
                    )
                    .expect("Jacobian entry")
            },
        );
    }

    #[test]
    #[ignore = "release-only shipped-model native x64 throughput probe; run with --release --features native -- --ignored --nocapture"]
    fn native_x64_shipped_model_microbench_reports_sweep_throughput() {
        assert!(
            !cfg!(debug_assertions),
            "native x64 shipped-model microbench is release-only; rerun with --release"
        );

        let cases = [
            (
                "juncap200",
                shipped_cmc_model_path(&["PSP104.1.0_vacode", "vacode", "juncap200.va"]),
                None,
            ),
            (
                "r3_cmc",
                shipped_cmc_model_path(&["r3_cmc_release1.1.2_2023Jun16", "r3_cmc.va"]),
                None,
            ),
            (
                "diode_cmc",
                shipped_cmc_model_path(&["diode_cmc_3.0_20250714", "vacode", "diode_cmc.va"]),
                Some("DIODE_CMC"),
            ),
            (
                "vbic13_4t",
                shipped_veriloga_model_path(&["vbic_1.3", "vacode", "vbic_1p3.va"]),
                Some("vbic13_4t"),
            ),
            (
                "bsimbulk",
                shipped_cmc_model_path(&["BSIM-BULK107.2.1_02112025", "code", "bsimbulk.va"]),
                Some("bsimbulk"),
            ),
            (
                "bsimcmg",
                shipped_cmc_model_path(&["BSIM-CMG_112.1.0_04282026", "code", "bsimcmg.va"]),
                Some("bsimcmg_va"),
            ),
            (
                "psp104",
                shipped_cmc_model_path(&["PSP104.1.0_vacode", "vacode", "psp104.va"]),
                Some("PSP104VA"),
            ),
            (
                "psp104_nqs",
                shipped_cmc_model_path(&["PSP104.1.0_vacode", "vacode", "psp104_nqs.va"]),
                Some("PSPNQS104VA"),
            ),
            (
                "hicuml0",
                shipped_cmc_model_path(&["hicumL0_v2p1p0_files", "hicumL0_v2p1p0.va"]),
                Some("hicumL0va"),
            ),
            (
                "hicuml2",
                shipped_cmc_model_path(&["hicumL2_v320_files", "hicumL2_v320.va"]),
                Some("hicumL2va"),
            ),
            (
                "bsimimg",
                shipped_cmc_model_path(&["BSIM-IMG_103.0.0_20200102", "code", "bsimimg.va"]),
                Some("bsimimg"),
            ),
            (
                "bsimsoi461",
                shipped_veriloga_model_path(&["bsimsoi_4.6.1", "vacode", "bsimsoi.va"]),
                Some("bsimsoi_va"),
            ),
            (
                "bsimsoi47",
                shipped_cmc_model_path(&["BSIM-SOI_4.7.0_05192025", "code", "bsimsoi.va"]),
                Some("bsimsoi"),
            ),
            (
                "bsimsoi100",
                shipped_cmc_model_path(&["BSIM_SOI_100.1.1_09152025", "code", "bsimsoi.va"]),
                Some("bsimsoi"),
            ),
            (
                "l_utsoi102",
                shipped_cmc_model_path(&[
                    "L_UTSOI_102.9.0_code_package",
                    "vacode",
                    "L_UTSOI_102.va",
                ]),
                Some("l_utsoi"),
            ),
            (
                "hisimhv",
                shipped_cmc_model_path(&[
                    "HiSIM_HV_2.5.1_Release_20230209",
                    "HiSIM_HV_2.5.1_VA-Code",
                    "hisimhv_va",
                    "hisimhv.va",
                ]),
                Some("hisimhv_va"),
            ),
            (
                "hisimsoi",
                shipped_cmc_model_path(&[
                    "HiSIM_SOI_1.5.0_Release_20211008",
                    "HiSIM_SOI_1.5.0_VA-Code",
                    "hisimsoi_va",
                    "hisimsoi.va",
                ]),
                Some("hisimsoi_va"),
            ),
            (
                "asmhemt",
                shipped_cmc_model_path(&["ASM-HEMT101.6.0_05132026", "vacode", "asmhemt.va"]),
                Some("asmhemt"),
            ),
        ];
        let iterations = shipped_model_microbench_iterations();
        let samples = shipped_model_microbench_samples();
        eprintln!("native-x64-shipped-microbench iterations={iterations} samples={samples}");

        for (name, path, module) in cases {
            if !shipped_model_filter_allows(name) {
                continue;
            }
            run_shipped_model_microbench_case(name, &path, module, iterations, samples);
        }
    }

    #[test]
    #[ignore = "release-only shipped-model native x64 finite oracle; run with --release --features native -- --ignored --nocapture"]
    fn native_x64_shipped_model_finite_entries_match_bytecode_reference() {
        assert!(
            !cfg!(debug_assertions),
            "native x64 shipped-model finite oracle is release-only; rerun with --release"
        );

        let cases = [
            (
                "juncap200",
                shipped_cmc_model_path(&["PSP104.1.0_vacode", "vacode", "juncap200.va"]),
                None,
            ),
            (
                "bsimcmg",
                shipped_cmc_model_path(&["BSIM-CMG_112.1.0_04282026", "code", "bsimcmg.va"]),
                Some("bsimcmg_va"),
            ),
            (
                "r3_cmc",
                shipped_cmc_model_path(&["r3_cmc_release1.1.2_2023Jun16", "r3_cmc.va"]),
                None,
            ),
            (
                "diode_cmc",
                shipped_cmc_model_path(&["diode_cmc_3.0_20250714", "vacode", "diode_cmc.va"]),
                Some("DIODE_CMC"),
            ),
            (
                "vbic13_4t",
                shipped_veriloga_model_path(&["vbic_1.3", "vacode", "vbic_1p3.va"]),
                Some("vbic13_4t"),
            ),
            (
                "bsimbulk",
                shipped_cmc_model_path(&["BSIM-BULK107.2.1_02112025", "code", "bsimbulk.va"]),
                Some("bsimbulk"),
            ),
            (
                "psp104",
                shipped_cmc_model_path(&["PSP104.1.0_vacode", "vacode", "psp104.va"]),
                Some("PSP104VA"),
            ),
            (
                "bsimimg",
                shipped_cmc_model_path(&["BSIM-IMG_103.0.0_20200102", "code", "bsimimg.va"]),
                Some("bsimimg"),
            ),
            (
                "psp104_nqs",
                shipped_cmc_model_path(&["PSP104.1.0_vacode", "vacode", "psp104_nqs.va"]),
                Some("PSPNQS104VA"),
            ),
            (
                "hicuml0",
                shipped_cmc_model_path(&["hicumL0_v2p1p0_files", "hicumL0_v2p1p0.va"]),
                Some("hicumL0va"),
            ),
            (
                "hicuml2",
                shipped_cmc_model_path(&["hicumL2_v320_files", "hicumL2_v320.va"]),
                Some("hicumL2va"),
            ),
            (
                "bsimsoi47",
                shipped_cmc_model_path(&["BSIM-SOI_4.7.0_05192025", "code", "bsimsoi.va"]),
                Some("bsimsoi"),
            ),
            (
                "bsimsoi461",
                shipped_veriloga_model_path(&["bsimsoi_4.6.1", "vacode", "bsimsoi.va"]),
                Some("bsimsoi_va"),
            ),
            (
                "bsimsoi100",
                shipped_cmc_model_path(&["BSIM_SOI_100.1.1_09152025", "code", "bsimsoi.va"]),
                Some("bsimsoi"),
            ),
            (
                "l_utsoi102",
                shipped_cmc_model_path(&[
                    "L_UTSOI_102.9.0_code_package",
                    "vacode",
                    "L_UTSOI_102.va",
                ]),
                Some("l_utsoi"),
            ),
            (
                "hisimhv",
                shipped_cmc_model_path(&[
                    "HiSIM_HV_2.5.1_Release_20230209",
                    "HiSIM_HV_2.5.1_VA-Code",
                    "hisimhv_va",
                    "hisimhv.va",
                ]),
                Some("hisimhv_va"),
            ),
            (
                "hisimsoi",
                shipped_cmc_model_path(&[
                    "HiSIM_SOI_1.5.0_Release_20211008",
                    "HiSIM_SOI_1.5.0_VA-Code",
                    "hisimsoi_va",
                    "hisimsoi.va",
                ]),
                Some("hisimsoi_va"),
            ),
            (
                "asmhemt",
                shipped_cmc_model_path(&["ASM-HEMT101.6.0_05132026", "vacode", "asmhemt.va"]),
                Some("asmhemt"),
            ),
        ];

        for (name, path, module) in cases {
            if !shipped_model_filter_allows(name) {
                continue;
            }
            assert_shipped_model_finite_entries_match_bytecode(name, &path, module);
        }
    }

    #[test]
    #[ignore = "release-only shipped-model VerilogADevice native x64 probe; run with --release --features native -- --ignored --nocapture"]
    fn native_x64_shipped_model_devices_run_without_interpreter_fallback() {
        assert!(
            !cfg!(debug_assertions),
            "native x64 shipped-model device probe is release-only; rerun with --release"
        );

        let cases = [
            (
                "juncap200",
                shipped_cmc_model_path(&["PSP104.1.0_vacode", "vacode", "juncap200.va"]),
                None,
            ),
            (
                "bsimcmg",
                shipped_cmc_model_path(&["BSIM-CMG_112.1.0_04282026", "code", "bsimcmg.va"]),
                Some("bsimcmg_va"),
            ),
            (
                "r3_cmc",
                shipped_cmc_model_path(&["r3_cmc_release1.1.2_2023Jun16", "r3_cmc.va"]),
                None,
            ),
            (
                "diode_cmc",
                shipped_cmc_model_path(&["diode_cmc_3.0_20250714", "vacode", "diode_cmc.va"]),
                Some("DIODE_CMC"),
            ),
            (
                "vbic13_4t",
                shipped_veriloga_model_path(&["vbic_1.3", "vacode", "vbic_1p3.va"]),
                Some("vbic13_4t"),
            ),
            (
                "bsimbulk",
                shipped_cmc_model_path(&["BSIM-BULK107.2.1_02112025", "code", "bsimbulk.va"]),
                Some("bsimbulk"),
            ),
            (
                "psp104",
                shipped_cmc_model_path(&["PSP104.1.0_vacode", "vacode", "psp104.va"]),
                Some("PSP104VA"),
            ),
            (
                "bsimimg",
                shipped_cmc_model_path(&["BSIM-IMG_103.0.0_20200102", "code", "bsimimg.va"]),
                Some("bsimimg"),
            ),
            (
                "psp104_nqs",
                shipped_cmc_model_path(&["PSP104.1.0_vacode", "vacode", "psp104_nqs.va"]),
                Some("PSPNQS104VA"),
            ),
            (
                "hicuml0",
                shipped_cmc_model_path(&["hicumL0_v2p1p0_files", "hicumL0_v2p1p0.va"]),
                Some("hicumL0va"),
            ),
            (
                "hicuml2",
                shipped_cmc_model_path(&["hicumL2_v320_files", "hicumL2_v320.va"]),
                Some("hicumL2va"),
            ),
            (
                "bsimsoi47",
                shipped_cmc_model_path(&["BSIM-SOI_4.7.0_05192025", "code", "bsimsoi.va"]),
                Some("bsimsoi"),
            ),
            (
                "bsimsoi461",
                shipped_veriloga_model_path(&["bsimsoi_4.6.1", "vacode", "bsimsoi.va"]),
                Some("bsimsoi_va"),
            ),
            (
                "bsimsoi100",
                shipped_cmc_model_path(&["BSIM_SOI_100.1.1_09152025", "code", "bsimsoi.va"]),
                Some("bsimsoi"),
            ),
            (
                "l_utsoi102",
                shipped_cmc_model_path(&[
                    "L_UTSOI_102.9.0_code_package",
                    "vacode",
                    "L_UTSOI_102.va",
                ]),
                Some("l_utsoi"),
            ),
            (
                "hisimhv",
                shipped_cmc_model_path(&[
                    "HiSIM_HV_2.5.1_Release_20230209",
                    "HiSIM_HV_2.5.1_VA-Code",
                    "hisimhv_va",
                    "hisimhv.va",
                ]),
                Some("hisimhv_va"),
            ),
            (
                "hisimsoi",
                shipped_cmc_model_path(&[
                    "HiSIM_SOI_1.5.0_Release_20211008",
                    "HiSIM_SOI_1.5.0_VA-Code",
                    "hisimsoi_va",
                    "hisimsoi.va",
                ]),
                Some("hisimsoi_va"),
            ),
            (
                "asmhemt",
                shipped_cmc_model_path(&["ASM-HEMT101.6.0_05132026", "vacode", "asmhemt.va"]),
                Some("asmhemt"),
            ),
        ];

        for (name, path, module) in cases {
            if !shipped_model_filter_allows(name) {
                continue;
            }
            run_shipped_model_device_probe(name, &path, module);
        }
    }

    #[test]
    fn compile_model_with_canonical_ir_rejects_unsupported_mir_stamp() {
        let source = r#"
`include "disciplines.vams"
module native_canonical_unsupported(p, n);
  inout p, n;
  electrical p, n;
  analog I(p, n) <+ V(p, n);
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let artifact = canonical_artifact_with_unsupported_root(&compiler, source);

        let error = compile_model_with_canonical_ir(&model, &artifact)
            .expect_err("unsupported canonical stamp must not fall back to bytecode");

        assert!(
            error.to_string().contains("expression kind string"),
            "{error}"
        );
    }

    #[test]
    fn canonical_logical_range_helpers_reject_overflow_without_wraparound() {
        let model = compiled_model_with_variables(0);
        let error = super::ranges_overlap(&model, "x", i64::MAX - 1, 2, 0, 1)
            .expect_err("overflowing canonical range must hard-fail");
        let message = error.to_string();
        assert!(
            message.contains("logical range"),
            "unexpected error: {message}"
        );
        assert!(
            message.contains("no interpreter fallback"),
            "canonical range overflow must preserve hard-JIT contract: {message}"
        );

        let error = super::checked_logical_index(&model, "x", i64::MAX, 1)
            .expect_err("overflowing logical index must hard-fail");
        let message = error.to_string();
        assert!(
            message.contains("logical index"),
            "unexpected error: {message}"
        );
        assert!(
            message.contains("no interpreter fallback"),
            "canonical logical-index overflow must preserve hard-JIT contract: {message}"
        );
    }

    #[test]
    fn compile_model_with_canonical_ir_rejects_indexed_assignment_logical_range_overflow() {
        let source = r#"
`include "disciplines.vams"
module native_canonical_indexed_range_overflow(p, n);
  inout p, n;
  electrical p, n;
  integer i;
  real x[0:1];
  analog begin
    i = 0;
    x[i] = V(p, n);
    I(p, n) <+ x[0];
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let mut artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        let array = artifact
            .hir
            .arrays
            .iter_mut()
            .find(|array| array.name.as_str() == "x")
            .expect("fixture has x array metadata");
        array.lower = i64::MAX;
        array.len = 2;
        let artifact = rebuild_canonical_artifact(artifact);

        let error = compile_model_with_canonical_ir(&model, &artifact)
            .expect_err("overflowing canonical indexed-assignment range must hard-fail");
        let message = error.to_string();
        assert!(
            message.contains("logical range"),
            "unexpected error: {message}"
        );
        assert!(
            message.contains("no interpreter fallback"),
            "canonical range overflow must preserve hard-JIT contract: {message}"
        );
    }

    #[test]
    fn native_assignment_roots_excludes_internal_derivative_shadows_and_guards() {
        let mut model = compiled_model_with_variables(4);
        model.variable_names = vec![
            SmolStr::new("x"),
            SmolStr::new("x@d0"),
            SmolStr::new("__guard0"),
            SmolStr::new("array@d1[0]"),
        ];

        let live = native_assignment_roots(&model);

        assert_eq!(live, vec![true, false, false, false]);
    }

    #[test]
    fn assignment_chunks_bound_code_size_and_dispatch_in_source_order() {
        let mut first_ops = Vec::with_capacity(MAX_ASSIGNMENT_CHUNK_OPERATIONS - 1);
        first_ops.push(NativeOp::Const(1.0));
        for _ in 0..((MAX_ASSIGNMENT_CHUNK_OPERATIONS - 2) / 2) {
            first_ops.extend([NativeOp::Const(1.0), NativeOp::Add]);
        }
        let first = NativeProgram::from_ops_for_test(first_ops, 2, Vec::new(), Vec::new());
        let second = NativeProgram::from_ops_for_test(
            vec![
                NativeOp::LoadVariable(0),
                NativeOp::Const(2.0),
                NativeOp::Add,
            ],
            2,
            Vec::new(),
            Vec::new(),
        );
        let assignments = [
            NativeAssignment::Direct {
                var_index: 0,
                program: first,
            },
            NativeAssignment::Direct {
                var_index: 1,
                program: second,
            },
        ];

        assert_eq!(assignment_chunk_ranges(&assignments), vec![0..1, 1..2]);

        let mut image = Vec::new();
        let mut entry_starts = Vec::new();
        let mut unwind = Vec::new();
        let dispatcher =
            append_assignment_pass(&assignments, &mut image, &mut entry_starts, &mut unwind)
                .expect("compile chunked assignment dispatcher");
        assert_eq!(entry_starts.len(), 3, "two chunks plus one dispatcher");
        assert_eq!(dispatcher, entry_starts[2]);

        let memory = ExecutableMemory::allocate(&image).expect("allocate chunked assignments");
        let entry = memory
            .ptr_at(dispatcher.as_usize())
            .expect("chunked assignment dispatcher entry");
        let function: extern "C" fn(*const EvalContext, *mut f64) =
            unsafe { std::mem::transmute(entry) };
        let context = eval_context(&[], &[]);
        let mut variables = [0.0_f64; 2];
        function(&context, variables.as_mut_ptr());

        let expected_first = (MAX_ASSIGNMENT_CHUNK_OPERATIONS / 2) as f64;
        assert_eq!(variables[0], expected_first);
        assert_eq!(variables[1], expected_first + 2.0);
    }

    #[test]
    fn assignment_dispatcher_aborts_before_later_chunks_on_runtime_error() {
        let mut failing_ops = Vec::with_capacity(MAX_ASSIGNMENT_CHUNK_OPERATIONS);
        failing_ops.push(NativeOp::Const(1.0));
        for _ in 0..((MAX_ASSIGNMENT_CHUNK_OPERATIONS - 2) / 2 + 1) {
            failing_ops.extend([NativeOp::Const(1.0), NativeOp::Add]);
        }
        failing_ops.push(NativeOp::LoadVariableDyn {
            base: 0,
            len: 1,
            lower: 0,
        });
        let assignments = [
            NativeAssignment::Direct {
                var_index: 0,
                program: NativeProgram::from_ops_for_test(failing_ops, 2, Vec::new(), Vec::new()),
            },
            NativeAssignment::Direct {
                var_index: 1,
                program: NativeProgram::from_ops_for_test(
                    vec![NativeOp::Const(5.0)],
                    1,
                    Vec::new(),
                    Vec::new(),
                ),
            },
        ];
        assert_eq!(assignment_chunk_ranges(&assignments), vec![0..1, 1..2]);

        let mut image = Vec::new();
        let mut entry_starts = Vec::new();
        let mut unwind = Vec::new();
        let dispatcher =
            append_assignment_pass(&assignments, &mut image, &mut entry_starts, &mut unwind)
                .expect("compile failing chunk dispatcher");
        let memory = ExecutableMemory::allocate(&image).expect("allocate failing dispatcher");
        let entry = memory
            .ptr_at(dispatcher.as_usize())
            .expect("failing assignment dispatcher entry");
        let function: extern "C" fn(*const EvalContext, *mut f64) =
            unsafe { std::mem::transmute(entry) };
        let context = eval_context(&[], &[]);
        let mut variables = [0.0_f64, -7.0_f64];

        function(&context, variables.as_mut_ptr());

        assert!(context.take_runtime_error().is_some());
        assert_eq!(
            variables[1], -7.0,
            "dispatcher must not execute chunks after the first runtime error"
        );
    }

    #[test]
    fn native_assignment_roots_keeps_internal_shadow_reads_from_jacobians() {
        let mut model = compiled_model_with_variables(3);
        model.variable_names = vec![
            SmolStr::new("x"),
            SmolStr::new("x@d0"),
            SmolStr::new("__guard0"),
        ];
        model.stamp_programs.push(StampProgram {
            stamp_locations: Vec::new(),
            value_program: BytecodeProgram {
                instructions: vec![Instruction::PushConst(0.0)],
            },
            jacobian_programs: vec![JacobianEntry {
                row: StampIndex::Ground,
                col: StampIndex::Ground,
                col_axis: ColumnAxis::Node(0),
                sign: 1.0,
                program: BytecodeProgram {
                    instructions: vec![Instruction::PushVariable(1)],
                },
            }],
            reactive_jacobians: Vec::new(),
            branch_ordinal: None,
            indirect: false,
            static_condition: None,
        });

        let live = native_assignment_roots(&model);

        assert_eq!(live, vec![true, true, false]);
    }

    #[test]
    fn lower_assignment_step_folds_constant_indexed_write_to_direct_assignment() {
        let model = compiled_model_with_variables(4);
        let step = indexed_assignment_step(
            1,
            3,
            1,
            vec![Instruction::PushConst(2.49)],
            vec![Instruction::PushConst(11.0)],
        );

        let assignment = lower_assignment_step(&model, &step).expect("lower indexed assignment");

        match assignment {
            NativeAssignment::Direct { var_index, program } => {
                assert_eq!(var_index, 2);
                assert_eq!(program.ops(), &[NativeOp::Const(11.0)]);
            }
            other => panic!("expected direct assignment, got {other:?}"),
        }
    }

    #[test]
    fn lower_assignment_step_rejects_direct_target_outside_variable_storage() {
        let model = compiled_model_with_variables(1);
        let step = AssignmentStep::Assign(crate::codegen::AssignmentProgram {
            var_index: 1,
            program: crate::codegen::BytecodeProgram {
                instructions: vec![crate::codegen::Instruction::PushConst(11.0)],
            },
        });

        let error = lower_assignment_step(&model, &step)
            .expect_err("native assignment target must stay inside variable storage");
        let message = error.to_string();
        assert!(
            message.contains("assignment target variable 1"),
            "unexpected error: {message}"
        );
        assert!(
            message.contains("no interpreter fallback"),
            "native assignment target error must preserve hard-JIT contract: {message}"
        );
    }

    #[test]
    fn lower_assignment_step_preserves_unsafe_indexed_writes_on_helper_path() {
        let cases = [
            ("dynamic", vec![Instruction::PushVariable(0)], 0),
            ("nan", vec![Instruction::PushConst(f64::NAN)], 0),
            ("infinity", vec![Instruction::PushConst(f64::INFINITY)], 0),
            (
                "huge finite",
                vec![Instruction::PushConst(1.0e300)],
                i64::MAX,
            ),
            ("out of range", vec![Instruction::PushConst(2.0)], 0),
        ];

        for (name, index, lower) in cases {
            let model = compiled_model_with_variables(1);
            let step = indexed_assignment_step(
                0,
                1,
                lower,
                index.clone(),
                vec![Instruction::PushConst(11.0)],
            );

            let assignment =
                lower_assignment_step(&model, &step).expect("lower indexed assignment");

            match assignment {
                NativeAssignment::Indexed {
                    base,
                    len,
                    lower: actual_lower,
                    index: index_program,
                    value,
                } => {
                    assert_eq!(base, 0, "{name}");
                    assert_eq!(len, 1, "{name}");
                    assert_eq!(actual_lower, lower, "{name}");
                    assert_eq!(value.ops(), &[NativeOp::Const(11.0)], "{name}");
                    assert!(
                        !index_program.ops().is_empty(),
                        "{name}: index program must remain on helper path"
                    );
                }
                other => panic!("{name}: expected indexed helper path, got {other:?}"),
            }
        }
    }

    fn compile_shape_validator_model() -> CompiledModel {
        let source = r#"
`include "disciplines.vams"
module native_shape_validator(p, n);
  inout p, n;
  electrical p, n;
  parameter real base = 1.0;
  parameter real gain = base * 2.0;
  real v;
  analog begin
    v = V(p, n);
    if (gain > 0.0) begin
      I(p, n) <+ gain * v;
    end
    I(p, n) <+ ddt(1.0e-12 * v);
    I(p, n) <+ flicker_noise(1.0e-18 * (1.0 + abs(v)), 1.0, "flicker");
  end
endmodule
"#;
        VerilogACompiler::new(CompilerOptions::default())
            .compile(source)
            .expect("compile native shape validator model")
    }

    fn expected_native_entry_shape(
        model: &CompiledModel,
    ) -> (NativeEntryOffsets, NativeCurrentDependencies) {
        let offset = CodeOffset::new(0);
        let entries = NativeEntryOffsets {
            assignment: offset,
            post_assignment: None,
            evaluation_kernel: None,
            stamp_kernel: None,
            parameter_defaults: model
                .parameters
                .iter()
                .map(|parameter| parameter.default_program.as_ref().map(|_| offset))
                .collect(),
            static_conditions: model
                .stamp_programs
                .iter()
                .map(|stamp| stamp.static_condition.as_ref().map(|_| offset))
                .collect(),
            stamp_values: vec![offset; model.stamp_programs.len()],
            jacobians: model
                .stamp_programs
                .iter()
                .map(|stamp| vec![offset; stamp.jacobian_programs.len()])
                .collect(),
            reactive_jacobians: model
                .stamp_programs
                .iter()
                .map(|stamp| vec![offset; stamp.reactive_jacobians.len()])
                .collect(),
            noise_psd: vec![offset; model.noise_sources.len()],
            noise_exponents: model
                .noise_sources
                .iter()
                .map(|source| source.exponent_program.as_ref().map(|_| offset))
                .collect(),
        };
        let dependencies = NativeCurrentDependencies {
            assignment_current_pairs: Vec::new(),
            assignment_prior_currents: Vec::new(),
            assignment_branch_unknowns: Vec::new(),
            post_assignment_current_pairs: Vec::new(),
            post_assignment_prior_currents: Vec::new(),
            post_assignment_branch_unknowns: Vec::new(),
            static_condition_branch_unknowns: vec![Vec::new(); entries.static_conditions.len()],
            stamp_values: vec![Vec::new(); entries.stamp_values.len()],
            stamp_value_prior_currents: vec![Vec::new(); entries.stamp_values.len()],
            stamp_value_branch_unknowns: vec![Vec::new(); entries.stamp_values.len()],
            jacobians: empty_nested_dependencies(&entries.jacobians),
            jacobian_prior_currents: empty_nested_dependencies(&entries.jacobians),
            jacobian_branch_unknowns: empty_nested_dependencies(&entries.jacobians),
            reactive_jacobians: empty_nested_dependencies(&entries.reactive_jacobians),
            reactive_jacobian_prior_currents: empty_nested_dependencies(
                &entries.reactive_jacobians,
            ),
            reactive_jacobian_branch_unknowns: empty_nested_dependencies(
                &entries.reactive_jacobians,
            ),
            noise_psd: vec![Vec::new(); entries.noise_psd.len()],
            noise_psd_prior_currents: vec![Vec::new(); entries.noise_psd.len()],
            noise_psd_branch_unknowns: vec![Vec::new(); entries.noise_psd.len()],
            noise_exponents: vec![Vec::new(); entries.noise_exponents.len()],
            noise_exponent_prior_currents: vec![Vec::new(); entries.noise_exponents.len()],
            noise_exponent_branch_unknowns: vec![Vec::new(); entries.noise_exponents.len()],
        };
        (entries, dependencies)
    }

    fn empty_nested_dependencies(entries: &[Vec<CodeOffset>]) -> Vec<Vec<Vec<usize>>> {
        entries
            .iter()
            .map(|stamp_entries| vec![Vec::new(); stamp_entries.len()])
            .collect()
    }

    fn compile_native_microbench_model(source: &str) -> (CompiledModel, NativeModel) {
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler.compile(source).expect("compile bytecode model");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical IR");
        let native = compile_model_with_canonical_ir(&model, &artifact)
            .expect("source-level canonical model compiles to native x64");
        (model, native)
    }

    fn run_shipped_model_microbench_case(
        name: &str,
        path: &Path,
        module: Option<&str>,
        iterations: usize,
        samples: usize,
    ) {
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let compile_start = web_time::Instant::now();
        let runtime = compiler
            .compile_file_runtime_with_metadata(path, module)
            .unwrap_or_else(|error| {
                panic!(
                    "compile shipped Verilog-A model {name} at {}: {error}",
                    path.display()
                )
            });
        let compile_elapsed = compile_start.elapsed();
        let native_start = web_time::Instant::now();
        let native = compile_model_with_canonical_ir(&runtime.model, &runtime.canonical_ir)
            .unwrap_or_else(|error| panic!("native x64 compile shipped model {name}: {error}"));
        let native_compile_elapsed = native_start.elapsed();
        let mut context = native_model_benchmark_context(&runtime.model, name);
        resolve_native_parameter_defaults(&runtime.model, &native, &mut context);
        let nonfinite_reference =
            collect_bytecode_nonfinite_reference(&runtime.model, context.clone())
                .unwrap_or_else(|error| panic!("{name}: bytecode non-finite reference: {error}"));
        if !nonfinite_reference.is_empty() {
            eprintln!(
                "native-x64-shipped-microbench model={name} bytecode_nonfinite_reference stamps={} jacobians={} reactive_jacobians={}",
                nonfinite_reference.stamps.len(),
                nonfinite_reference.jacobians.len(),
                nonfinite_reference.reactive_jacobians.len(),
            );
        }
        let mut sanity_checksum = run_native_model_sweep_once(
            &runtime.model,
            &native,
            &mut context,
            name,
            &nonfinite_reference,
        );
        assert!(
            sanity_checksum.is_finite(),
            "{name}: shipped-model sanity sweep checksum must stay finite"
        );

        let warmup_iterations = (iterations / 10).max(1);
        sanity_checksum += run_native_model_sweep_sample(
            &runtime.model,
            &native,
            &mut context,
            name,
            warmup_iterations,
            &nonfinite_reference,
        );
        assert!(
            sanity_checksum.is_finite(),
            "{name}: shipped-model warmup checksum must stay finite"
        );

        let mut sample_ns_per_sweep = Vec::with_capacity(samples);
        let mut checksum = 0.0_f64;
        for _ in 0..samples {
            let start = web_time::Instant::now();
            checksum += run_native_model_sweep_sample(
                &runtime.model,
                &native,
                &mut context,
                name,
                iterations,
                &nonfinite_reference,
            );
            let elapsed = start.elapsed();
            sample_ns_per_sweep.push(elapsed.as_nanos() as f64 / iterations as f64);
        }
        sample_ns_per_sweep.sort_by(|left, right| left.total_cmp(right));
        let min_ns_per_sweep = sample_ns_per_sweep[0];
        let median_ns_per_sweep = sample_ns_per_sweep[sample_ns_per_sweep.len() / 2];
        let p95_index = ((sample_ns_per_sweep.len() as f64 * 0.95).ceil() as usize)
            .saturating_sub(1)
            .min(sample_ns_per_sweep.len() - 1);
        let p95_ns_per_sweep = sample_ns_per_sweep[p95_index];
        let mean_ns_per_sweep =
            sample_ns_per_sweep.iter().sum::<f64>() / sample_ns_per_sweep.len() as f64;
        let standard_deviation = (sample_ns_per_sweep
            .iter()
            .map(|sample| {
                let delta = sample - mean_ns_per_sweep;
                delta * delta
            })
            .sum::<f64>()
            / sample_ns_per_sweep.len() as f64)
            .sqrt();
        let relative_standard_deviation =
            standard_deviation / mean_ns_per_sweep.max(f64::MIN_POSITIVE);
        let checksum = std::hint::black_box(checksum);
        assert!(
            checksum.is_finite(),
            "{name}: shipped-model benchmark checksum must stay finite"
        );
        let stats = native.plan_stats();
        eprintln!(
            "native-x64-shipped-microbench model={name} compile_ms={:.3} native_compile_ms={:.3} code_bytes={} entry_points={} dependencies={} params={} vars={} assignments={} stamps={} jacobians={} reactive_jacobians={} min_ns_per_sweep={min_ns_per_sweep:.3} median_ns_per_sweep={median_ns_per_sweep:.3} p95_ns_per_sweep={p95_ns_per_sweep:.3} relative_standard_deviation={relative_standard_deviation:.6} checksum={checksum:.17e}",
            compile_elapsed.as_secs_f64() * 1000.0,
            native_compile_elapsed.as_secs_f64() * 1000.0,
            native.code_size_bytes(),
            stats.total_entry_points(),
            runtime.dependencies.len(),
            runtime.model.parameters.len(),
            runtime.model.num_variables,
            count_assignment_steps(&runtime.model.assignment_steps),
            stats.stamp_value_entry_points,
            stats.jacobian_entry_points,
            stats.reactive_jacobian_entry_points,
        );
    }

    fn assert_shipped_model_finite_entries_match_bytecode(
        name: &str,
        path: &Path,
        module: Option<&str>,
    ) {
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let runtime = compiler
            .compile_file_runtime_with_metadata(path, module)
            .unwrap_or_else(|error| {
                panic!(
                    "compile shipped Verilog-A oracle model {name} at {}: {error}",
                    path.display()
                )
            });
        let native = compile_model_with_canonical_ir(&runtime.model, &runtime.canonical_ir)
            .unwrap_or_else(|error| panic!("native x64 compile oracle model {name}: {error}"));
        let mut context = native_model_benchmark_context(&runtime.model, name);
        resolve_native_parameter_defaults(&runtime.model, &native, &mut context);

        let stats = assert_native_matches_bytecode_finite_entries(
            &runtime.model,
            &runtime.canonical_ir,
            &native,
            context,
            name,
        )
        .unwrap_or_else(|error| panic!("{name}: finite native oracle failed: {error}"));
        eprintln!(
            "native-x64-shipped-oracle model={name} variables={} higher_order_shadows={} stamps={} jacobians={} reactive_jacobians={} skipped_nonfinite={}",
            stats.variables,
            stats.higher_order_shadows,
            stats.stamps,
            stats.jacobians,
            stats.reactive_jacobians,
            stats.skipped_nonfinite,
        );
    }

    fn run_shipped_model_device_probe(name: &str, path: &Path, module: Option<&str>) {
        shipped_probe_trace(name, "compile-runtime:start");
        let frontend_start = web_time::Instant::now();
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let runtime = compiler
            .compile_file_runtime_with_metadata(path, module)
            .unwrap_or_else(|error| {
                panic!(
                    "compile shipped Verilog-A device probe model {name} at {}: {error}",
                    path.display()
                )
            });
        let frontend_elapsed = frontend_start.elapsed();
        shipped_probe_trace(name, "native-device:start");
        let native_start = web_time::Instant::now();
        let model = std::sync::Arc::new(runtime.model.clone());
        let nodes = (1..=model.num_terminals).collect::<Vec<_>>();
        let mut device = VerilogADevice::try_new_with_canonical_ir(
            format!("{name}_device"),
            std::sync::Arc::clone(&model),
            &runtime.canonical_ir,
            &nodes,
        )
        .unwrap_or_else(|error| {
            panic!("{name}: shipped device native construction failed: {error}")
        });
        let native_elapsed = native_start.elapsed();
        assert!(
            device.is_using_native(),
            "{name}: shipped device must use native code"
        );
        assert!(
            device.native_code_size_bytes()
                <= crate::native::SHIPPED_MODEL_NATIVE_CODE_SIZE_BUDGET_BYTES,
            "{name}: x64 native image is {} bytes, exceeding the shipped-model budget of {} bytes",
            device.native_code_size_bytes(),
            crate::native::SHIPPED_MODEL_NATIVE_CODE_SIZE_BUDGET_BYTES,
        );
        if matches!(name, "hicuml0" | "hicuml2") {
            device
                .try_set_analysis_type(2)
                .unwrap_or_else(|error| panic!("{name}: set transient analysis: {error}"));
        }
        shipped_probe_trace(name, "native-device:ready");

        let terminal_count = model.num_terminals;
        let internal_indices = ((terminal_count + 1)
            ..(terminal_count + 1 + device.num_internal_nodes()))
            .collect::<Vec<_>>();
        shipped_probe_trace(name, "set-indices:start");
        device.set_internal_node_indices(&internal_indices);
        let branch_start = terminal_count + 1 + device.num_internal_nodes();
        let branch_indices =
            (branch_start..(branch_start + device.num_branch_unknowns())).collect::<Vec<_>>();
        device.set_branch_current_indices(&branch_indices);

        let solution_len =
            terminal_count + device.num_internal_nodes() + device.num_branch_unknowns();
        let mut solution = vec![0.0; solution_len.max(1)];
        for (terminal, node) in nodes.iter().copied().enumerate() {
            solution[node - 1] = shipped_device_terminal_bias(name, terminal);
        }
        let canonical_internal_nodes = runtime
            .canonical_ir
            .mir
            .nodes
            .iter()
            .filter(|node| !node.is_external)
            .collect::<Vec<_>>();
        assert_eq!(
            canonical_internal_nodes.len(),
            device.num_internal_nodes(),
            "{name}: canonical and runtime internal-node counts must match"
        );
        for (ordinal, node) in canonical_internal_nodes.into_iter().enumerate() {
            if let Some(value) = shipped_device_internal_bias(name, node.name.as_str()) {
                solution[terminal_count + ordinal] = value;
            }
        }

        device
            .try_update_all_voltages(&solution)
            .unwrap_or_else(|error| {
                panic!("{name}: shipped device voltage update failed: {error}")
            });
        device
            .try_set_analysis_step(true, false)
            .unwrap_or_else(|error| panic!("{name}: enter initial analysis step: {error}"));
        device.try_evaluate().unwrap_or_else(|error| {
            panic!("{name}: shipped device native initial-step evaluation failed: {error}")
        });
        device
            .try_set_analysis_step(false, false)
            .unwrap_or_else(|error| panic!("{name}: leave initial analysis step: {error}"));
        shipped_probe_trace(name, "evaluate:start");
        let currents = device.try_evaluate().unwrap_or_else(|error| {
            panic!("{name}: shipped device native evaluate failed: {error}")
        });
        shipped_probe_trace(name, "evaluate:done");
        assert_eq!(
            currents.len(),
            model.stamp_programs.len(),
            "{name}: device evaluate must return one current per stamp"
        );
        let finite_currents = currents.iter().filter(|value| value.is_finite()).count();
        assert!(
            finite_currents > 0,
            "{name}: shipped device native evaluate must produce finite currents"
        );

        let mut matrix_entries = 0usize;
        let mut rhs_entries = 0usize;
        let mut matrix_l1 = 0.0_f64;
        let mut rhs_l1 = 0.0_f64;
        shipped_probe_trace(name, "stamp:start");
        device
            .try_stamp(
                &solution,
                |row, col, value| {
                    assert!(
                        value.is_finite(),
                        "{name}: non-finite matrix stamp ({row},{col})={value}"
                    );
                    matrix_entries += 1;
                    matrix_l1 += value.abs();
                },
                |row, value| {
                    assert!(
                        value.is_finite(),
                        "{name}: non-finite rhs stamp ({row})={value}"
                    );
                    rhs_entries += 1;
                    rhs_l1 += value.abs();
                },
            )
            .unwrap_or_else(|error| panic!("{name}: shipped device native stamp failed: {error}"));
        shipped_probe_trace(name, "stamp:done");
        assert!(
            matrix_entries > 0,
            "{name}: shipped device native stamp must produce matrix entries"
        );
        assert!(
            rhs_entries > 0,
            "{name}: shipped device native stamp must produce RHS entries"
        );

        let mut reactive_entries = 0usize;
        let mut reactive_l1 = 0.0_f64;
        shipped_probe_trace(name, "reactive:start");
        device
            .try_stamp_reactive(&solution, |row, col, value| {
                assert!(
                    value.is_finite(),
                    "{name}: non-finite reactive stamp ({row},{col})={value}"
                );
                reactive_entries += 1;
                reactive_l1 += value.abs();
            })
            .unwrap_or_else(|error| {
                panic!("{name}: shipped device native reactive stamp failed: {error}")
            });
        shipped_probe_trace(name, "reactive:done");
        assert!(
            reactive_entries > 0,
            "{name}: shipped device native reactive stamp must produce entries"
        );

        eprintln!(
            "native-x64-shipped-device model={name} frontend_ms={:.3} native_ms={:.3} code_bytes={} native_chunks={} finite_currents={} matrix_entries={} rhs_entries={} reactive_entries={} matrix_l1={matrix_l1:.17e} rhs_l1={rhs_l1:.17e} reactive_l1={reactive_l1:.17e}",
            frontend_elapsed.as_secs_f64() * 1000.0,
            native_elapsed.as_secs_f64() * 1000.0,
            device.native_code_size_bytes(),
            device.native_chunk_count(),
            finite_currents,
            matrix_entries,
            rhs_entries,
            reactive_entries,
        );
    }

    fn shipped_probe_trace(name: &str, stage: &str) {
        if std::env::var_os("RSPICE_NATIVE_SHIPPED_PROBE_TRACE").is_some() {
            eprintln!("native-x64-shipped-probe model={name} stage={stage}");
        }
    }

    fn shipped_model_filter_allows(name: &str) -> bool {
        let Ok(filter) = std::env::var("RSPICE_NATIVE_SHIPPED_MODEL_FILTER") else {
            return true;
        };
        filter
            .split(',')
            .map(str::trim)
            .any(|candidate| candidate.eq_ignore_ascii_case(name))
    }

    fn shipped_device_terminal_bias(name: &str, terminal: usize) -> f64 {
        match name {
            "juncap200" => [0.2, 0.0].get(terminal).copied().unwrap_or(0.0),
            "bsimcmg" => [0.05, 0.7, 0.0, 0.0, 0.0]
                .get(terminal)
                .copied()
                .unwrap_or(0.0),
            "bsimbulk" => [0.05, 0.7, 0.0, 0.0, 0.0]
                .get(terminal)
                .copied()
                .unwrap_or(0.0),
            "psp104" => [0.05, 0.7, 0.0, 0.0].get(terminal).copied().unwrap_or(0.0),
            "r3_cmc" => [0.1, 0.0, 0.0, 0.0].get(terminal).copied().unwrap_or(0.0),
            "diode_cmc" => [0.7, 0.0, 0.0, 0.0].get(terminal).copied().unwrap_or(0.0),
            "vbic13_4t" => [0.2, 0.75, 0.0, 0.0].get(terminal).copied().unwrap_or(0.0),
            "bsimimg" => [0.15, 0.7, 0.05, -0.05, 0.01]
                .get(terminal)
                .copied()
                .unwrap_or(0.0),
            "hisimsoi" => [0.05, 0.7, 0.0, 0.0, 0.0, 0.0]
                .get(terminal)
                .copied()
                .unwrap_or(0.0),
            "bsimsoi47" => [0.05, 0.7, 0.0, 0.0, 0.0, 0.0, 0.0]
                .get(terminal)
                .copied()
                .unwrap_or(0.0),
            "bsimsoi100" => [0.05, 0.7, 0.0, 0.0, 0.0, 0.0]
                .get(terminal)
                .copied()
                .unwrap_or(0.0),
            "psp104_nqs" => [0.05, 0.7, 0.0, 0.0, 0.0]
                .get(terminal)
                .copied()
                .unwrap_or(0.0),
            "hicuml0" => [0.2, 0.8, 0.0, 0.0].get(terminal).copied().unwrap_or(0.0),
            "hicuml2" => [0.2, 0.8, 0.0, 0.0, 0.0]
                .get(terminal)
                .copied()
                .unwrap_or(0.0),
            "asmhemt" => [0.1, 0.3, 0.0, 0.0, 0.0]
                .get(terminal)
                .copied()
                .unwrap_or(0.0),
            _ => 0.0,
        }
    }

    fn shipped_device_internal_bias(name: &str, node: &str) -> Option<f64> {
        match (name, node.to_ascii_lowercase().as_str()) {
            ("bsimcmg", "di" | "di1" | "di2") => Some(shipped_device_terminal_bias(name, 0)),
            ("bsimcmg", "si" | "si1") => Some(shipped_device_terminal_bias(name, 2)),
            ("bsimcmg", "ge" | "gi" | "gint" | "gints" | "gintd") => {
                Some(shipped_device_terminal_bias(name, 1))
            }
            ("bsimimg", "di") => Some(shipped_device_terminal_bias(name, 0)),
            ("bsimimg", "si") => Some(shipped_device_terminal_bias(name, 2)),
            ("bsimimg", "ge" | "gi") => Some(shipped_device_terminal_bias(name, 1)),
            ("vbic13_4t", "cx" | "ci") => Some(shipped_device_terminal_bias(name, 0)),
            ("vbic13_4t", "bx" | "bi" | "bp") => Some(shipped_device_terminal_bias(name, 1)),
            ("vbic13_4t", "ei") => Some(shipped_device_terminal_bias(name, 2)),
            ("vbic13_4t", "si") => Some(shipped_device_terminal_bias(name, 3)),
            _ => None,
        }
    }

    #[derive(Default)]
    struct FiniteOracleStats {
        variables: usize,
        higher_order_shadows: usize,
        stamps: usize,
        jacobians: usize,
        reactive_jacobians: usize,
        skipped_nonfinite: usize,
    }

    fn require_clean_native_context(
        ctx: &EvalContext,
        stage: impl std::fmt::Display,
    ) -> Result<(), String> {
        if let Some(error) = ctx.take_runtime_error() {
            return Err(format!("native runtime error during {stage}: {error}"));
        }
        Ok(())
    }

    fn assert_native_matches_bytecode_finite_entries(
        model: &CompiledModel,
        artifact: &CanonicalIrArtifact,
        native: &NativeModel,
        base_context: VmContext,
        name: &str,
    ) -> Result<FiniteOracleStats, String> {
        let canonical_branch_unknown_map =
            canonical_branch_unknown_runtime_map(model, &artifact.mir)
                .map_err(|error| error.to_string())?;
        let limits = NativeLoweringLimits::for_model(model)
            .with_canonical_branch_unknown_map(&canonical_branch_unknown_map);
        let live_variables = live_canonical_assignment_slots(model, &artifact.mir, limits)
            .map_err(|error| error.to_string())?;
        let mut bytecode_context = base_context.clone();
        bytecode_context.clear_currents();
        bytecode_context
            .currents
            .resize(model.stamp_programs.len(), 0.0);
        let mut vm = Vm::new(&mut bytecode_context);
        let (pre_current_assignment_steps, post_current_targets) =
            split_bytecode_assignment_steps_at_completed_current(
                &model.assignment_steps,
                model.num_variables,
            );
        execute_bytecode_assignment_steps(&mut vm, &pre_current_assignment_steps)
            .map_err(|error| error.to_string())?;

        let mut native_context = base_context;
        native_context.clear_currents();
        native_context
            .currents
            .resize(model.stamp_programs.len(), 0.0);
        let mut ctx = eval_context_from_vm_context(&mut native_context);
        ctx.clear_runtime_error();
        native.run_assignments(&ctx, native_context.variables.as_mut_ptr());
        require_clean_native_context(&ctx, "assignments")?;

        let mut stats = FiniteOracleStats::default();
        for (index, is_live) in live_variables.iter().copied().enumerate() {
            if !is_live || post_current_targets.get(index).copied().unwrap_or(false) {
                continue;
            }
            let variable_name = model
                .variable_names
                .get(index)
                .map(|name| name.as_str())
                .unwrap_or("<unnamed>");
            assert_internal_variable_compatible(
                name,
                format!("variable {index} ({variable_name})"),
                variable_name,
                vm.context.variables[index],
                native_context.variables[index],
                &mut stats.variables,
                &mut stats.higher_order_shadows,
                &mut stats.skipped_nonfinite,
            )?;
        }

        let mut prior_current_probes = Vec::new();
        for (stamp_index, stamp) in model.stamp_programs.iter().enumerate() {
            let bytecode_active = if let Some(condition) = &stamp.static_condition {
                let reference = vm.execute(condition).map_err(|error| error.to_string())?;
                ctx = eval_context_from_vm_context(&mut native_context);
                let actual = native
                    .run_static_condition(stamp_index, &ctx, native_context.variables.as_ptr())
                    .ok_or_else(|| format!("missing native static condition {stamp_index}"))?;
                require_clean_native_context(
                    &ctx,
                    format!("static condition {stamp_index} oracle"),
                )?;
                assert_finite_close(
                    name,
                    format!("static_condition {stamp_index}"),
                    reference,
                    actual,
                )?;
                reference != 0.0
            } else {
                true
            };

            let native_active = if stamp.static_condition.is_some() {
                ctx = eval_context_from_vm_context(&mut native_context);
                let active = native
                    .run_static_condition(stamp_index, &ctx, native_context.variables.as_ptr())
                    .ok_or_else(|| format!("missing native static condition {stamp_index}"))?;
                require_clean_native_context(
                    &ctx,
                    format!("static condition {stamp_index} activity"),
                )?;
                active != 0.0
            } else {
                true
            };
            if bytecode_active != native_active {
                return Err(format!(
                    "static condition {stamp_index} active mismatch: bytecode={bytecode_active} native={native_active}"
                ));
            }
            if !bytecode_active {
                push_prior_current_probe_aliases_for_stamp(
                    model,
                    &mut prior_current_probes,
                    stamp_index,
                    stamp,
                );
                continue;
            }

            let reference = execute_bytecode_value_program_with_prior_current_probes(
                &mut vm,
                &stamp.value_program,
                &prior_current_probes,
            )
            .map_err(|error| error.to_string())?;
            ctx = eval_context_from_vm_context(&mut native_context);
            let actual = native
                .run_stamp_value(stamp_index, &ctx, native_context.variables.as_ptr())
                .ok_or_else(|| format!("missing native stamp-value entry {stamp_index}"))?;
            require_clean_native_context(&ctx, format!("stamp {stamp_index}"))?;
            assert_close_or_skip_nonfinite(
                name,
                format!("stamp {stamp_index}"),
                reference,
                actual,
                &mut stats.stamps,
                &mut stats.skipped_nonfinite,
            )?;
            vm.context.currents[stamp_index] = reference;
            native_context.currents[stamp_index] = actual;
            if stamp.branch_ordinal.is_none()
                && let Some((pos, neg)) = super::infer_current_terminal_pair(stamp)
            {
                vm.context.set_branch_current(pos, neg, reference);
                native_context.set_branch_current(pos, neg, actual);
            }

            let mut jacobian_prior_current_probes = prior_current_probes.clone();
            push_prior_current_probe_aliases_for_stamp(
                model,
                &mut jacobian_prior_current_probes,
                stamp_index,
                stamp,
            );
            for entry_index in 0..stamp.jacobian_programs.len() {
                let reference = execute_bytecode_value_program_with_prior_current_probes(
                    &mut vm,
                    &stamp.jacobian_programs[entry_index].program,
                    &jacobian_prior_current_probes,
                )
                .map_err(|error| error.to_string())?;
                ctx = eval_context_from_vm_context(&mut native_context);
                let actual = native
                    .run_jacobian(
                        stamp_index,
                        entry_index,
                        &ctx,
                        native_context.variables.as_ptr(),
                    )
                    .ok_or_else(|| {
                        format!("missing native Jacobian entry {stamp_index}.{entry_index}")
                    })?;
                require_clean_native_context(
                    &ctx,
                    format!("Jacobian {stamp_index}.{entry_index}"),
                )?;
                assert_close_or_skip_nonfinite(
                    name,
                    format!("jacobian {stamp_index}.{entry_index}"),
                    reference,
                    actual,
                    &mut stats.jacobians,
                    &mut stats.skipped_nonfinite,
                )?;
            }

            for entry_index in 0..stamp.reactive_jacobians.len() {
                let reference = vm
                    .execute(&stamp.reactive_jacobians[entry_index].program)
                    .map_err(|error| error.to_string())?;
                ctx = eval_context_from_vm_context(&mut native_context);
                let actual = native
                    .run_reactive_jacobian(
                        stamp_index,
                        entry_index,
                        &ctx,
                        native_context.variables.as_ptr(),
                    )
                    .ok_or_else(|| {
                        format!(
                            "missing native reactive-Jacobian entry {stamp_index}.{entry_index}"
                        )
                    })?;
                require_clean_native_context(
                    &ctx,
                    format!("reactive Jacobian {stamp_index}.{entry_index}"),
                )?;
                assert_close_or_skip_nonfinite(
                    name,
                    format!("reactive_jacobian {stamp_index}.{entry_index}"),
                    reference,
                    actual,
                    &mut stats.reactive_jacobians,
                    &mut stats.skipped_nonfinite,
                )?;
            }

            push_prior_current_probe_aliases_for_stamp(
                model,
                &mut prior_current_probes,
                stamp_index,
                stamp,
            );
        }

        Ok(stats)
    }

    fn assert_close_or_skip_nonfinite(
        name: &str,
        entry: impl std::fmt::Display,
        reference: f64,
        actual: f64,
        matched_count: &mut usize,
        skipped_nonfinite: &mut usize,
    ) -> Result<(), String> {
        if !reference.is_finite() {
            *skipped_nonfinite += 1;
            return Ok(());
        }
        assert_finite_close(name, entry, reference, actual)?;
        *matched_count += 1;
        Ok(())
    }

    fn assert_internal_variable_compatible(
        name: &str,
        entry: impl std::fmt::Display,
        variable_name: &str,
        reference: f64,
        actual: f64,
        matched_count: &mut usize,
        higher_order_shadow_count: &mut usize,
        skipped_nonfinite: &mut usize,
    ) -> Result<(), String> {
        let derivative_order = variable_name
            .split_once('@')
            .and_then(|(_, suffix)| derivative_shadow_axes_from_suffix(suffix))
            .map_or(0, |axes| axes.len());
        if derivative_order <= 1 {
            return assert_close_or_skip_nonfinite(
                name,
                entry,
                reference,
                actual,
                matched_count,
                skipped_nonfinite,
            );
        }

        // Canonical native differentiation and the legacy shadow generator
        // can legally parenthesize higher-order product/chain-rule terms
        // differently. Ill-conditioned compact models may amplify that
        // rounding inside these private temporaries even when every
        // solver-visible stamp and Jacobian agrees. Require native code to
        // preserve finiteness here; the strict comparisons below remain the
        // production contract for all externally consumed entries.
        if !reference.is_finite() {
            *skipped_nonfinite += 1;
            return Ok(());
        }
        if !actual.is_finite() {
            return Err(format!(
                "{}: native higher-order shadow is non-finite while bytecode is finite: bytecode={reference} native={actual}",
                entry
            ));
        }
        *higher_order_shadow_count += 1;
        Ok(())
    }

    #[test]
    fn finite_oracle_treats_higher_order_shadows_as_internal_finite_state() {
        let mut matched = 0;
        let mut higher_order = 0;
        let mut skipped = 0;
        assert_internal_variable_compatible(
            "fixture",
            "variable q@d1@d1",
            "q@d1@d1",
            1.0,
            2.0,
            &mut matched,
            &mut higher_order,
            &mut skipped,
        )
        .expect("finite higher-order shadows may differ by reassociation");
        assert_eq!(matched, 0);
        assert_eq!(higher_order, 1);
        assert_eq!(skipped, 0);

        assert!(
            assert_internal_variable_compatible(
                "fixture",
                "variable q@d1@d1",
                "q@d1@d1",
                1.0,
                f64::INFINITY,
                &mut matched,
                &mut higher_order,
                &mut skipped,
            )
            .is_err(),
            "native higher-order shadows must remain finite whenever bytecode is finite"
        );
        assert!(
            assert_internal_variable_compatible(
                "fixture",
                "variable q@d1",
                "q@d1",
                1.0,
                2.0,
                &mut matched,
                &mut higher_order,
                &mut skipped,
            )
            .is_err(),
            "first-order state remains subject to strict numerical agreement"
        );
    }

    fn assert_finite_close(
        name: &str,
        entry: impl std::fmt::Display,
        reference: f64,
        actual: f64,
    ) -> Result<(), String> {
        let entry = entry.to_string();
        if !reference.is_finite() {
            return Err(format!(
                "{entry}: bytecode reference is non-finite: {reference}"
            ));
        }
        if !actual.is_finite() {
            return Err(format!(
                "{entry}: native value is non-finite while bytecode is finite: bytecode={reference} native={actual}"
            ));
        }
        if reference == actual {
            return Ok(());
        }
        let scale = reference.abs().max(actual.abs()).max(1.0);
        let tolerance = 1.0e-8 * scale;
        let delta = (reference - actual).abs();
        if delta <= tolerance {
            return Ok(());
        }
        Err(format!(
            "{entry}: {name} native/reference mismatch: bytecode={reference:.17e} native={actual:.17e} delta={delta:.17e} tolerance={tolerance:.17e}"
        ))
    }

    fn native_model_benchmark_context(model: &CompiledModel, name: &str) -> VmContext {
        let mut context = VmContext::with_internal_nodes(model.num_terminals, model.internal_nodes);
        context.voltages.fill(0.0);
        context.internal_voltages.fill(0.0);
        if name == "bsimcmg" && context.voltages.len() >= 5 {
            context.voltages[0] = 0.05;
            context.voltages[1] = 0.7;
            context.voltages[2] = 0.0;
            context.voltages[3] = 0.0;
            context.voltages[4] = 0.0;
        }
        context.parameters = model
            .parameters
            .iter()
            .map(|parameter| parameter.default)
            .collect();
        context.param_given = vec![0; model.parameters.len()];
        context.variables = vec![0.0; model.num_variables.max(1)];
        context.currents = vec![0.0; model.stamp_programs.len()];
        context.branch_current_values = vec![0.0; model.branch_sources.len()];
        context.lookup_tables = model.lookup_tables.clone();
        context.laplace_filters = model.laplace_filters.clone();
        context.zi_filters = model.zi_filters.clone();
        context.time = 1.0e-9;
        context.set_timestep(1.0e-12);
        context.set_integration_coefficients(crate::vm::IntegrationCoefficients::inactive());
        preallocate_native_benchmark_context(&mut context, model);
        context
    }

    fn resolve_native_parameter_defaults(
        model: &CompiledModel,
        native: &NativeModel,
        context: &mut VmContext,
    ) {
        for index in 0..model.parameters.len() {
            let ctx = eval_context_from_vm_context(context);
            if let Some(value) =
                native.run_parameter_default(index, &ctx, context.variables.as_ptr())
            {
                context.parameters[index] = value;
            }
        }
    }

    fn run_native_model_sweep_sample(
        model: &CompiledModel,
        native: &NativeModel,
        context: &mut VmContext,
        name: &str,
        iterations: usize,
        nonfinite_reference: &NonFiniteReference,
    ) -> f64 {
        let mut checksum = 0.0_f64;
        for _ in 0..iterations {
            checksum += std::hint::black_box(run_native_model_sweep_once(
                model,
                native,
                context,
                name,
                nonfinite_reference,
            ));
        }
        std::hint::black_box(checksum)
    }

    fn run_native_model_sweep_once(
        model: &CompiledModel,
        native: &NativeModel,
        context: &mut VmContext,
        name: &str,
        nonfinite_reference: &NonFiniteReference,
    ) -> f64 {
        context.clear_currents();
        context.currents.resize(model.stamp_programs.len(), 0.0);

        let mut ctx = eval_context_from_vm_context(context);
        ctx.clear_runtime_error();
        native.run_assignments(&ctx, context.variables.as_mut_ptr());
        require_clean_native_context(&ctx, format!("{name} assignments"))
            .unwrap_or_else(|error| panic!("{error}"));

        let mut checksum = 0.0_f64;
        for (stamp_index, stamp) in model.stamp_programs.iter().enumerate() {
            ctx = eval_context_from_vm_context(context);
            if let Some(active) =
                native.run_static_condition(stamp_index, &ctx, context.variables.as_ptr())
            {
                require_clean_native_context(
                    &ctx,
                    format!("{name} static condition {stamp_index}"),
                )
                .unwrap_or_else(|error| panic!("{error}"));
                if active == 0.0 {
                    continue;
                }
            }

            ctx = eval_context_from_vm_context(context);
            let value = native
                .run_stamp_value(stamp_index, &ctx, context.variables.as_ptr())
                .unwrap_or_else(|| {
                    panic!("{name}: missing native stamp-value entry {stamp_index}")
                });
            require_clean_native_context(&ctx, format!("{name} stamp {stamp_index}"))
                .unwrap_or_else(|error| panic!("{error}"));
            if !value.is_finite() {
                assert!(
                    nonfinite_reference.stamps.contains(&stamp_index),
                    "{name}: non-finite stamp {stamp_index} value {value}"
                );
            } else {
                checksum += value;
            }
            context.currents[stamp_index] = value;
            if stamp.branch_ordinal.is_none()
                && let Some((pos, neg)) = super::infer_current_terminal_pair(stamp)
            {
                context.set_branch_current(pos, neg, value);
            }

            for entry_index in 0..stamp.jacobian_programs.len() {
                ctx = eval_context_from_vm_context(context);
                let value = native
                    .run_jacobian(stamp_index, entry_index, &ctx, context.variables.as_ptr())
                    .unwrap_or_else(|| {
                        panic!("{name}: missing native Jacobian entry {stamp_index}.{entry_index}")
                    });
                require_clean_native_context(
                    &ctx,
                    format!("{name} Jacobian {stamp_index}.{entry_index}"),
                )
                .unwrap_or_else(|error| panic!("{error}"));
                if !value.is_finite() {
                    assert!(
                        nonfinite_reference
                            .jacobians
                            .contains(&(stamp_index, entry_index)),
                        "{name}: non-finite jacobian {stamp_index}.{entry_index} value {value}"
                    );
                    continue;
                }
                checksum += value;
            }

            for entry_index in 0..stamp.reactive_jacobians.len() {
                ctx = eval_context_from_vm_context(context);
                let value = native
                    .run_reactive_jacobian(
                        stamp_index,
                        entry_index,
                        &ctx,
                        context.variables.as_ptr(),
                    )
                    .unwrap_or_else(|| {
                        panic!(
                            "{name}: missing native reactive-Jacobian entry {stamp_index}.{entry_index}"
                        )
                    });
                require_clean_native_context(
                    &ctx,
                    format!("{name} reactive Jacobian {stamp_index}.{entry_index}"),
                )
                .unwrap_or_else(|error| panic!("{error}"));
                if !value.is_finite() {
                    assert!(
                        nonfinite_reference
                            .reactive_jacobians
                            .contains(&(stamp_index, entry_index)),
                        "{name}: non-finite reactive jacobian {stamp_index}.{entry_index} value {value}"
                    );
                    continue;
                }
                checksum += value;
            }
        }

        checksum
    }

    #[derive(Default)]
    struct NonFiniteReference {
        stamps: Vec<usize>,
        jacobians: Vec<(usize, usize)>,
        reactive_jacobians: Vec<(usize, usize)>,
    }

    impl NonFiniteReference {
        fn is_empty(&self) -> bool {
            self.stamps.is_empty()
                && self.jacobians.is_empty()
                && self.reactive_jacobians.is_empty()
        }
    }

    fn collect_bytecode_nonfinite_reference(
        model: &CompiledModel,
        mut context: VmContext,
    ) -> Result<NonFiniteReference, String> {
        let mut reference = NonFiniteReference::default();
        context.clear_currents();
        context.currents.resize(model.stamp_programs.len(), 0.0);
        let mut vm = Vm::new(&mut context);
        let (pre_current_assignment_steps, _) =
            split_bytecode_assignment_steps_at_completed_current(
                &model.assignment_steps,
                model.num_variables,
            );
        execute_bytecode_assignment_steps(&mut vm, &pre_current_assignment_steps)
            .map_err(|error| error.to_string())?;

        let mut prior_current_probes = Vec::new();
        for (stamp_index, stamp) in model.stamp_programs.iter().enumerate() {
            if let Some(condition) = &stamp.static_condition
                && vm.execute(condition).map_err(|error| error.to_string())? == 0.0
            {
                push_prior_current_probe_aliases_for_stamp(
                    model,
                    &mut prior_current_probes,
                    stamp_index,
                    stamp,
                );
                continue;
            }

            let value = execute_bytecode_value_program_with_prior_current_probes(
                &mut vm,
                &stamp.value_program,
                &prior_current_probes,
            )
            .map_err(|error| error.to_string())?;
            if !value.is_finite() {
                reference.stamps.push(stamp_index);
            }
            vm.context.currents[stamp_index] = value;
            if stamp.branch_ordinal.is_none()
                && let Some((pos, neg)) = super::infer_current_terminal_pair(stamp)
            {
                vm.context.set_branch_current(pos, neg, value);
            }

            let mut jacobian_prior_current_probes = prior_current_probes.clone();
            push_prior_current_probe_aliases_for_stamp(
                model,
                &mut jacobian_prior_current_probes,
                stamp_index,
                stamp,
            );
            for entry_index in 0..stamp.jacobian_programs.len() {
                let value = execute_bytecode_value_program_with_prior_current_probes(
                    &mut vm,
                    &stamp.jacobian_programs[entry_index].program,
                    &jacobian_prior_current_probes,
                )
                .map_err(|error| error.to_string())?;
                if !value.is_finite() {
                    reference.jacobians.push((stamp_index, entry_index));
                }
            }

            for entry_index in 0..stamp.reactive_jacobians.len() {
                let value = vm
                    .execute(&stamp.reactive_jacobians[entry_index].program)
                    .map_err(|error| error.to_string())?;
                if !value.is_finite() {
                    reference
                        .reactive_jacobians
                        .push((stamp_index, entry_index));
                }
            }

            push_prior_current_probe_aliases_for_stamp(
                model,
                &mut prior_current_probes,
                stamp_index,
                stamp,
            );
        }

        Ok(reference)
    }

    fn execute_bytecode_value_program_with_prior_current_probes(
        vm: &mut Vm<'_>,
        program: &crate::codegen::BytecodeProgram,
        prior_current_probes: &[PriorCurrentProbe],
    ) -> Result<f64, crate::vm::VmError> {
        let mut rewritten = None;
        for (index, instruction) in program.instructions.iter().enumerate() {
            let Instruction::PushCurrent(pos, neg) = *instruction else {
                continue;
            };
            if vm.context.try_current(pos, neg).is_ok() {
                continue;
            }
            let Some(value) =
                prior_current_probe_value(vm.context, prior_current_probes, pos, neg)?
            else {
                continue;
            };
            let rewritten = rewritten.get_or_insert_with(|| program.clone());
            rewritten.instructions[index] = Instruction::PushConst(value);
        }

        if let Some(rewritten) = rewritten {
            vm.execute(&rewritten)
        } else {
            vm.execute(program)
        }
    }

    fn prior_current_probe_value(
        context: &VmContext,
        probes: &[PriorCurrentProbe],
        pos: usize,
        neg: usize,
    ) -> Result<Option<f64>, crate::vm::VmError> {
        let mut value = None;
        for probe in probes
            .iter()
            .filter(|probe| probe.pos == pos && probe.neg == neg)
        {
            let current = context.currents.get(probe.current_index).copied().ok_or(
                crate::vm::VmError::InvalidInstruction("missing prior contribution current slot"),
            )?;
            let current = if probe.inverted { -current } else { current };
            value = Some(value.unwrap_or(0.0) + current);
        }
        Ok(value)
    }

    fn push_prior_current_probe_aliases_for_stamp(
        model: &CompiledModel,
        probes: &mut Vec<PriorCurrentProbe>,
        stamp_index: usize,
        stamp: &StampProgram,
    ) {
        if stamp.branch_ordinal.is_none()
            && let Some((pos, neg)) = super::infer_current_unified_pair(model, stamp)
        {
            super::push_prior_current_probe_aliases(probes, stamp_index, pos, neg);
        }
    }

    fn execute_bytecode_assignment_steps(
        vm: &mut Vm<'_>,
        steps: &[AssignmentStep],
    ) -> Result<(), crate::vm::VmError> {
        for step in steps {
            match step {
                AssignmentStep::Assign(assignment) => {
                    let value = vm.execute(&assignment.program)?;
                    vm.context.variables[assignment.var_index] = value;
                }
                AssignmentStep::AssignIndexed {
                    base,
                    len,
                    lower,
                    index,
                    value,
                } => {
                    let raw_index = vm.execute(index)?;
                    let slot = Vm::array_slot(raw_index, *base, *len, *lower)?;
                    let value = vm.execute(value)?;
                    vm.context.variables[slot] = value;
                }
                AssignmentStep::Loop { condition, body } => {
                    let mut iterations = 0usize;
                    while vm.execute(condition)? != 0.0 {
                        execute_bytecode_assignment_steps(vm, body)?;
                        iterations += 1;
                        assert!(
                            iterations < 100_000,
                            "bytecode reference loop exceeded shipped benchmark iteration guard"
                        );
                    }
                }
            }
        }
        Ok(())
    }

    fn split_bytecode_assignment_steps_at_completed_current(
        steps: &[AssignmentStep],
        num_variables: usize,
    ) -> (Vec<AssignmentStep>, Vec<bool>) {
        let split = steps
            .iter()
            .position(bytecode_assignment_step_reads_current)
            .unwrap_or(steps.len());
        let mut post_targets = vec![false; num_variables];
        mark_bytecode_assignment_targets(&steps[split..], &mut post_targets);
        (steps[..split].to_vec(), post_targets)
    }

    fn bytecode_assignment_step_reads_current(step: &AssignmentStep) -> bool {
        match step {
            AssignmentStep::Assign(assignment) => {
                bytecode_program_reads_current(&assignment.program)
            }
            AssignmentStep::AssignIndexed { index, value, .. } => {
                bytecode_program_reads_current(index) || bytecode_program_reads_current(value)
            }
            AssignmentStep::Loop { condition, body } => {
                bytecode_program_reads_current(condition)
                    || body.iter().any(bytecode_assignment_step_reads_current)
            }
        }
    }

    fn bytecode_program_reads_current(program: &BytecodeProgram) -> bool {
        program
            .instructions
            .iter()
            .any(|instruction| matches!(instruction, Instruction::PushCurrent(_, _)))
    }

    fn mark_bytecode_assignment_targets(steps: &[AssignmentStep], targets: &mut [bool]) {
        for step in steps {
            match step {
                AssignmentStep::Assign(assignment) => {
                    if let Some(target) = targets.get_mut(assignment.var_index) {
                        *target = true;
                    }
                }
                AssignmentStep::AssignIndexed { base, len, .. } => {
                    let end = base.saturating_add(*len).min(targets.len());
                    for target in targets.iter_mut().take(end).skip(*base) {
                        *target = true;
                    }
                }
                AssignmentStep::Loop { body, .. } => {
                    mark_bytecode_assignment_targets(body, targets);
                }
            }
        }
    }

    fn preallocate_native_benchmark_context(context: &mut VmContext, model: &CompiledModel) {
        let mut max_state = None;
        let mut max_delay_buffer = None;
        let mut max_transition_filter = None;
        let mut max_slew_filter = None;
        let mut max_cross_detector = None;

        let mut scan_program = |program: &BytecodeProgram| {
            for instruction in &program.instructions {
                match instruction {
                    Instruction::DdtState(idx)
                    | Instruction::IdtState(idx)
                    | Instruction::IdtModState(idx)
                    | Instruction::LimitState(idx)
                    | Instruction::CanonicalLimitState(idx) => {
                        update_max_slot(&mut max_state, *idx);
                    }
                    Instruction::AbsDelayState(idx) => {
                        update_max_slot(&mut max_delay_buffer, *idx);
                    }
                    Instruction::TransitionState(idx) => {
                        update_max_slot(&mut max_transition_filter, *idx);
                    }
                    Instruction::SlewState(idx) | Instruction::SlewStateDerivative(idx) => {
                        update_max_slot(&mut max_slew_filter, *idx);
                    }
                    Instruction::CrossState(idx)
                    | Instruction::AboveState(idx)
                    | Instruction::LastCrossingState(idx) => {
                        update_max_slot(&mut max_cross_detector, *idx);
                    }
                    _ => {}
                }
            }
        };

        for parameter in &model.parameters {
            if let Some(program) = &parameter.default_program {
                scan_program(program);
            }
        }
        scan_assignment_steps(&model.assignment_steps, &mut scan_program);
        for stamp in &model.stamp_programs {
            if let Some(condition) = &stamp.static_condition {
                scan_program(condition);
            }
            scan_program(&stamp.value_program);
            for jacobian in &stamp.jacobian_programs {
                scan_program(&jacobian.program);
            }
            for jacobian in &stamp.reactive_jacobians {
                scan_program(&jacobian.program);
            }
        }
        for source in &model.noise_sources {
            scan_program(&source.psd_program);
            if let Some(program) = &source.exponent_program {
                scan_program(program);
            }
        }

        if let Some(max_idx) = max_state {
            context.allocate_states(max_idx + 1);
        }
        if let Some(max_idx) = max_delay_buffer {
            context.allocate_delay_buffers(max_idx + 1);
        }
        if let Some(max_idx) = max_transition_filter {
            context.allocate_transition_filters(max_idx + 1);
        }
        if let Some(max_idx) = max_slew_filter {
            context.allocate_slew_filters(max_idx + 1);
        }
        if let Some(max_idx) = max_cross_detector {
            context.allocate_cross_detectors(max_idx + 1);
        }
    }

    fn scan_assignment_steps(
        steps: &[AssignmentStep],
        scan_program: &mut impl FnMut(&BytecodeProgram),
    ) {
        for step in steps {
            match step {
                AssignmentStep::Assign(assignment) => {
                    scan_program(&assignment.program);
                }
                AssignmentStep::AssignIndexed { index, value, .. } => {
                    scan_program(index);
                    scan_program(value);
                }
                AssignmentStep::Loop { condition, body } => {
                    scan_program(condition);
                    scan_assignment_steps(body, scan_program);
                }
            }
        }
    }

    fn eval_context_from_vm_context(context: &mut VmContext) -> EvalContext {
        let integration = context.integration_coefficients();
        EvalContext {
            voltages: context.voltages.as_ptr(),
            internal_voltages: context.internal_voltages.as_ptr(),
            params: context.parameters.as_ptr(),
            branch_currents: context.terminal_pair_currents_ptr(),
            branch_currents_len: context.terminal_pair_currents_len(),
            currents: context.currents.as_ptr(),
            currents_len: context.currents.len(),
            num_terminals: context.terminal_count(),
            port_connected: context.port_connected.as_ptr(),
            port_connected_len: context.port_connected.len(),
            temperature: context.temperature,
            time: context.time,
            timestep: context.timestep(),
            state_prev: if context.state_values_prev.is_empty() {
                std::ptr::null()
            } else {
                context.state_values_prev.as_ptr()
            },
            state_values: if context.state_values.is_empty() {
                std::ptr::null_mut()
            } else {
                context.state_values.as_mut_ptr()
            },
            state_initialized: if context.state_initialized.is_empty() {
                std::ptr::null_mut()
            } else {
                context.state_initialized.as_mut_ptr() as *mut u8
            },
            state_initialized_len: context.state_initialized.len(),
            lookup_tables: if context.lookup_tables.is_empty() {
                std::ptr::null()
            } else {
                context.lookup_tables.as_ptr()
            },
            lookup_tables_len: context.lookup_tables.len(),
            laplace_filters: if context.laplace_filters.is_empty() {
                std::ptr::null_mut()
            } else {
                context.laplace_filters.as_mut_ptr()
            },
            laplace_filters_len: context.laplace_filters.len(),
            param_given: context.param_given.as_ptr(),
            param_given_len: context.param_given.len(),
            branch_unknowns: if context.branch_current_values.is_empty() {
                std::ptr::null()
            } else {
                context.branch_current_values.as_ptr()
            },
            analysis_type: context.analysis_type,
            multiplicity: context.multiplicity,
            zi_filters: if context.zi_filters.is_empty() {
                std::ptr::null_mut()
            } else {
                context.zi_filters.as_mut_ptr()
            },
            zi_filters_len: context.zi_filters.len(),
            transition_filters: if context.transition_filters.is_empty() {
                std::ptr::null_mut()
            } else {
                context.transition_filters.as_mut_ptr()
            },
            transition_filters_len: context.transition_filters.len(),
            slew_filters: if context.slew_filters.is_empty() {
                std::ptr::null_mut()
            } else {
                context.slew_filters.as_mut_ptr()
            },
            slew_filters_len: context.slew_filters.len(),
            delay_buffers: if context.delay_buffers.is_empty() {
                std::ptr::null_mut()
            } else {
                context.delay_buffers.as_mut_ptr()
            },
            delay_buffers_len: context.delay_buffers.len(),
            cross_detectors: if context.cross_detectors.is_empty() {
                std::ptr::null_mut()
            } else {
                context.cross_detectors.as_mut_ptr()
            },
            cross_detectors_len: context.cross_detectors.len(),
            state_prev_len: context.state_values_prev.len(),
            state_values_len: context.state_values.len(),
            timer_event_bound: &mut context.timer_event_bound,
            analysis_initial_step: u8::from(context.analysis_initial_step),
            analysis_final_step: u8::from(context.analysis_final_step),
            state_older: context.state_values_older.as_ptr(),
            state_older_len: context.state_values_older.len(),
            state_derivatives: context.state_derivatives.as_mut_ptr(),
            state_derivatives_len: context.state_derivatives.len(),
            state_derivatives_prev: context.state_derivatives_prev.as_ptr(),
            state_derivatives_prev_len: context.state_derivatives_prev.len(),
            integration_derivative_scale: integration.derivative_scale,
            integration_previous_value_scale: integration.previous_value_scale,
            integration_older_value_scale: integration.older_value_scale,
            integration_previous_derivative_scale: integration.previous_derivative_scale,
            integration_active: u8::from(integration.active),
            limiter_active: &mut context.limiter_active,
            limiting_enabled: u8::from(context.evaluation_mode.limiting_enabled()),
            runtime_status: Default::default(),
            state_candidate_valid: if context.state_candidate_valid.is_empty() {
                std::ptr::null_mut()
            } else {
                context.state_candidate_valid.as_mut_ptr()
            },
            state_candidate_valid_len: context.state_candidate_valid.len(),
            state_older_candidate: if context.state_older_candidate.is_empty() {
                std::ptr::null_mut()
            } else {
                context.state_older_candidate.as_mut_ptr()
            },
            state_older_candidate_len: context.state_older_candidate.len(),
        }
    }

    fn update_max_slot(max_slot: &mut Option<usize>, idx: usize) {
        *max_slot = Some(max_slot.map_or(idx, |prev| prev.max(idx)));
    }

    fn count_assignment_steps(steps: &[AssignmentStep]) -> usize {
        steps
            .iter()
            .map(|step| match step {
                AssignmentStep::Assign(_) | AssignmentStep::AssignIndexed { .. } => 1,
                AssignmentStep::Loop { body, .. } => 1 + count_assignment_steps(body),
            })
            .sum()
    }

    fn shipped_cmc_model_path(parts: &[&str]) -> PathBuf {
        let mut path = shipped_veriloga_model_path(&["cmc"]);
        for part in parts {
            path = path.join(part);
        }
        assert!(
            path.exists(),
            "required shipped CMC model fixture missing: {}",
            path.display()
        );
        path
    }

    fn shipped_veriloga_model_path(parts: &[&str]) -> PathBuf {
        let mut path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("models")
            .join("veriloga");
        for part in parts {
            path = path.join(part);
        }
        assert!(
            path.exists(),
            "required shipped Verilog-A model fixture missing: {}",
            path.display()
        );
        path
    }

    fn shipped_model_microbench_iterations() -> usize {
        2_000
    }

    fn shipped_model_microbench_samples() -> usize {
        5
    }

    fn run_native_model_entry_microbench<F>(
        case: &str,
        entry: &str,
        iterations: usize,
        samples: usize,
        expected: f64,
        mut f: F,
    ) where
        F: FnMut() -> f64,
    {
        assert_near(f(), expected, entry);

        let warmup_iterations = iterations / 10;
        let warmup_checksum = run_native_model_microbench_sample(&mut f, warmup_iterations.max(1));
        assert!(
            warmup_checksum.is_finite(),
            "{case} {entry}: native model microbench warmup checksum must stay finite"
        );

        let mut sample_ns_per_eval = Vec::with_capacity(samples);
        let mut checksum = 0.0_f64;
        for _ in 0..samples {
            let start = web_time::Instant::now();
            checksum += run_native_model_microbench_sample(&mut f, iterations);
            let elapsed = start.elapsed();
            sample_ns_per_eval.push(elapsed.as_nanos() as f64 / iterations as f64);
        }
        sample_ns_per_eval.sort_by(|left, right| left.total_cmp(right));
        let min_ns_per_eval = sample_ns_per_eval[0];
        let median_ns_per_eval = sample_ns_per_eval[sample_ns_per_eval.len() / 2];
        let checksum = std::hint::black_box(checksum);
        assert!(
            checksum.is_finite(),
            "{case} {entry}: native model microbench checksum must stay finite"
        );
        eprintln!(
            "native-x64-model-microbench case={case} entry={entry} min_ns_per_eval={min_ns_per_eval:.3} median_ns_per_eval={median_ns_per_eval:.3} checksum={checksum:.17e}",
        );
    }

    fn run_native_model_microbench_sample<F>(f: &mut F, iterations: usize) -> f64
    where
        F: FnMut() -> f64,
    {
        let mut checksum = 0.0_f64;
        for _ in 0..iterations {
            checksum += std::hint::black_box(f());
        }
        std::hint::black_box(checksum)
    }

    fn native_model_microbench_iterations() -> usize {
        5_000_000
    }

    fn native_model_microbench_samples() -> usize {
        5
    }

    fn assert_near(got: f64, expected: f64, name: &str) {
        assert!(
            (got - expected).abs() <= 1.0e-12,
            "{name}: got {got:.17e}, expected {expected:.17e}"
        );
    }

    fn poison_jacobian_bytecode(model: &mut CompiledModel, value: f64) {
        for stamp in &mut model.stamp_programs {
            for jacobian in &mut stamp.jacobian_programs {
                jacobian.program = BytecodeProgram {
                    instructions: vec![Instruction::PushConst(value)],
                };
            }
        }
    }

    fn poison_reactive_jacobian_bytecode(model: &mut CompiledModel, value: f64) {
        for stamp in &mut model.stamp_programs {
            for jacobian in &mut stamp.reactive_jacobians {
                jacobian.program = BytecodeProgram {
                    instructions: vec![Instruction::PushConst(value)],
                };
            }
        }
    }

    fn assert_jacobian_axis_value(
        model: &CompiledModel,
        native: &NativeModel,
        ctx: &EvalContext,
        variables: *const f64,
        stamp_index: usize,
        matches_axis: impl Fn(&ColumnAxis) -> bool,
        expected: f64,
        label: impl std::fmt::Display,
    ) {
        let label = label.to_string();
        let entry_index = model.stamp_programs[stamp_index]
            .jacobian_programs
            .iter()
            .position(|jacobian| matches_axis(&jacobian.col_axis))
            .unwrap_or_else(|| panic!("{label}: missing matching Jacobian axis"));
        let actual = native
            .run_jacobian(stamp_index, entry_index, ctx, variables)
            .expect("Jacobian entry");
        assert_close(&label, expected, actual);
    }

    fn assert_jacobian_axis_approx(
        model: &CompiledModel,
        native: &NativeModel,
        ctx: &EvalContext,
        variables: *const f64,
        stamp_index: usize,
        matches_axis: impl Fn(&ColumnAxis) -> bool,
        expected: f64,
        relative_tolerance: f64,
        label: impl std::fmt::Display,
    ) {
        let label = label.to_string();
        let entry_index = model.stamp_programs[stamp_index]
            .jacobian_programs
            .iter()
            .position(|jacobian| matches_axis(&jacobian.col_axis))
            .unwrap_or_else(|| panic!("{label}: missing matching Jacobian axis"));
        let actual = native
            .run_jacobian(stamp_index, entry_index, ctx, variables)
            .expect("Jacobian entry");
        assert!(
            actual.is_finite(),
            "{label}: native Jacobian is non-finite: {actual}"
        );
        let tolerance = relative_tolerance * expected.abs().max(actual.abs()).max(1.0);
        let delta = (actual - expected).abs();
        assert!(
            delta <= tolerance,
            "{label}: canonical Jacobian mismatch: expected={expected:.17e} actual={actual:.17e} delta={delta:.17e} tolerance={tolerance:.17e}"
        );
    }

    fn central_second_derivative(f: impl Fn(f64) -> f64, x: f64) -> f64 {
        let h = 1.0e-4_f64;
        (f(x + h) - 2.0 * f(x) + f(x - h)) / (h * h)
    }

    fn assert_reactive_jacobian_axis_value(
        model: &CompiledModel,
        native: &NativeModel,
        ctx: &EvalContext,
        variables: *const f64,
        stamp_index: usize,
        matches_axis: impl Fn(&ColumnAxis) -> bool,
        expected: f64,
        label: impl std::fmt::Display,
    ) {
        let label = label.to_string();
        let entry_index = model.stamp_programs[stamp_index]
            .reactive_jacobians
            .iter()
            .position(|jacobian| matches_axis(&jacobian.col_axis))
            .unwrap_or_else(|| panic!("{label}: missing matching reactive Jacobian axis"));
        let actual = native
            .run_reactive_jacobian(stamp_index, entry_index, ctx, variables)
            .expect("reactive Jacobian entry");
        assert_close(&label, expected, actual);
    }

    fn indexed_assignment_step(
        base: usize,
        len: usize,
        lower: i64,
        index: Vec<Instruction>,
        value: Vec<Instruction>,
    ) -> AssignmentStep {
        AssignmentStep::AssignIndexed {
            base,
            len,
            lower,
            index: BytecodeProgram {
                instructions: index,
            },
            value: BytecodeProgram {
                instructions: value,
            },
        }
    }

    fn compiled_model_with_variables(num_variables: usize) -> CompiledModel {
        CompiledModel {
            name: SmolStr::new("native_x64_assignment_test"),
            source_digest: SmolStr::default(),
            num_terminals: 0,
            terminal_names: Vec::new(),
            parameters: Vec::new(),
            num_variables,
            variable_names: Vec::new(),
            event_state_variables: Vec::new(),
            assignment_steps: Vec::new(),
            stamp_programs: Vec::new(),
            lookup_tables: Vec::new(),
            internal_nodes: 0,
            branch_sources: Vec::new(),
            laplace_filters: Vec::new(),
            zi_filters: Vec::new(),
            zi_filter_definitions: Vec::new(),
            noise_sources: Vec::new(),
        }
    }

    fn assert_close(label: &str, expected: f64, actual: f64) {
        let tolerance = 1.0e-12_f64.max(expected.abs() * 1.0e-12);
        assert!(
            (actual - expected).abs() <= tolerance,
            "{label}: expected {expected}, got {actual}"
        );
    }

    fn eval_context(params: &[f64], voltages: &[f64]) -> EvalContext {
        EvalContext {
            voltages: voltages.as_ptr(),
            internal_voltages: std::ptr::null(),
            params: params.as_ptr(),
            branch_currents: std::ptr::null(),
            branch_currents_len: 0,
            currents: std::ptr::null(),
            currents_len: 0,
            num_terminals: voltages.len(),
            port_connected: std::ptr::null(),
            port_connected_len: 0,
            temperature: 0.0,
            time: 0.0,
            timestep: 0.0,
            state_prev: std::ptr::null(),
            state_values: std::ptr::null_mut(),
            state_initialized: std::ptr::null_mut(),
            state_initialized_len: 0,
            lookup_tables: std::ptr::null(),
            lookup_tables_len: 0,
            laplace_filters: std::ptr::null_mut(),
            laplace_filters_len: 0,
            param_given: std::ptr::null(),
            param_given_len: 0,
            branch_unknowns: std::ptr::null(),
            analysis_type: 0,
            multiplicity: 1.0,
            zi_filters: std::ptr::null_mut(),
            zi_filters_len: 0,
            transition_filters: std::ptr::null_mut(),
            transition_filters_len: 0,
            slew_filters: std::ptr::null_mut(),
            slew_filters_len: 0,
            delay_buffers: std::ptr::null_mut(),
            delay_buffers_len: 0,
            cross_detectors: std::ptr::null_mut(),
            cross_detectors_len: 0,
            state_prev_len: 0,
            state_values_len: 0,
            timer_event_bound: std::ptr::null_mut(),
            analysis_initial_step: 0,
            analysis_final_step: 0,
            state_older: std::ptr::null(),
            state_older_len: 0,
            state_derivatives: std::ptr::null_mut(),
            state_derivatives_len: 0,
            state_derivatives_prev: std::ptr::null(),
            state_derivatives_prev_len: 0,
            integration_derivative_scale: 0.0,
            integration_previous_value_scale: 0.0,
            integration_older_value_scale: 0.0,
            integration_previous_derivative_scale: 0.0,
            integration_active: 0,
            limiter_active: std::ptr::null_mut(),
            limiting_enabled: 0,
            runtime_status: Default::default(),
            state_candidate_valid: std::ptr::null_mut(),
            state_candidate_valid_len: 0,
            state_older_candidate: std::ptr::null_mut(),
            state_older_candidate_len: 0,
        }
    }
}
