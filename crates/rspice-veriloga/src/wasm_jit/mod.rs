//! Standard-WebAssembly code generation for the browser Verilog-A JIT.
//!
//! A browser does not expose native executable-memory publication. RSpice
//! therefore emits a secondary, ordinary WebAssembly module and asks the
//! browser engine to validate and compile it inside the simulation worker.
//! This module owns deterministic encoding and an independent structural and
//! semantic verifier. Browser-specific compilation and instance registries
//! deliberately live outside this crate.

mod abi;
mod codegen;
#[cfg(target_arch = "wasm32")]
mod dispatch;
#[cfg(any(target_arch = "wasm32", test))]
mod executable;
#[cfg(any(target_arch = "wasm32", test))]
mod runtime;

#[cfg(target_arch = "wasm32")]
pub(crate) use abi::WasmJitEvalFrame;
pub use abi::{
    WASM_JIT_EVAL_FRAME_BYTES, WASM_JIT_FRAME_MAGIC, WASM_JIT_MAX_EVAL_FRAME_BYTES,
    WASM_JIT_MAX_SLICE_OPERANDS, WASM_JIT_SLICE_OPERANDS_OFFSET, WASM_JIT_STATUS_ABI_MISMATCH,
    WASM_JIT_STATUS_OK, WASM_JIT_STATUS_RUNTIME_ERROR,
};
#[cfg(target_arch = "wasm32")]
pub(crate) use dispatch::dispatch_model_entry;
#[cfg(target_arch = "wasm32")]
pub use dispatch::install_browser_dispatcher;
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) use executable::{WasmJitExecutable, WasmJitExecutableEntry};
#[cfg(target_arch = "wasm32")]
pub use runtime::{
    WasmJitRuntimeSession, eval_op_slice_v1, eval_op_v1, math1_v1, math2_v1, with_runtime_session,
};

use std::borrow::Cow;

use crate::canonical_ir::CanonicalIrArtifact;
use crate::codegen::CompiledModel;
use crate::jit::assignment::NativeAssignment;
use crate::jit::model_plan::NativeModelPlan;
use crate::jit::plan_program::PlanProgramRef;
use thiserror::Error;
use wasm_encoder::{
    CodeSection, CustomSection, ExportKind, ExportSection, Function, FunctionSection,
    ImportSection, Instruction, MemArg, MemoryType, Module, TypeSection, ValType,
};
use wasmparser::{Encoding, ExternalKind, Imports, Operator, Parser, Payload, TypeRef, Validator};

/// Version of the linear-memory and helper-function contract understood by
/// emitted modules and the browser worker.
pub const WASM_JIT_ABI_VERSION: u32 = 7;

/// Version of the deterministic encoder. It participates in cache identity
/// independently of the ABI because code layout may change without changing
/// runtime frames.
pub const WASM_JIT_EMITTER_VERSION: u32 = 7;

/// Hard ceiling for one qualified shipped model's generated module.
pub const SHIPPED_MODEL_WASM_CODE_SIZE_BUDGET_BYTES: usize = 32 * 1024 * 1024;

/// Maximum allowed size of the architecture probe. A larger probe indicates
/// accidental dependency/code inclusion and fails before reaching a browser.
pub const WASM_JIT_PROBE_SIZE_BUDGET_BYTES: usize = 4 * 1024;

/// Import namespace reserved for the secondary module's capability surface.
pub const WASM_JIT_IMPORT_MODULE: &str = "rspice_jit";

/// The only memory capability accepted by the architecture probe.
pub const WASM_JIT_MEMORY_IMPORT: &str = "memory";

/// Probe entrypoint installed in the browser worker.
pub const WASM_JIT_PROBE_EXPORT: &str = "rspice_wasm_jit_probe";

pub(crate) const WASM_JIT_CONTRACT_SECTION: &str = "rspice.wasm-jit.contract";
pub const WASM_JIT_PROBE_FRAME_BYTES: u64 = 16;
const WASM_JIT_PROBE_OUTPUT_OFFSET: u64 = 8;

/// A verified deterministic module ready for browser compilation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WasmJitArtifact {
    bytes: Vec<u8>,
    digest: String,
    abi_version: u32,
    emitter_version: u32,
}

/// Deterministic evidence that a model passed the same canonical planning
/// pipeline consumed by the native JITs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WasmJitPlanSummary {
    cache_key: String,
    entry_programs: usize,
    assignment_programs: usize,
    operations: usize,
    maximum_stack_depth: usize,
    emitted_value_module_bytes: usize,
}

/// Assignment phase containing a generated scalar entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmJitAssignmentPhase {
    Assignment,
    PostAssignment,
}

/// Meaning of a scalar program within one recursive assignment node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmJitAssignmentExpression {
    DirectValue,
    IndexedIndex,
    IndexedValue,
    LoopCondition,
}

/// Stable semantic role of one exported scalar entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WasmJitValueRole {
    Assignment {
        phase: WasmJitAssignmentPhase,
        /// Recursive source-order assignment indexes from the phase root.
        path: Vec<u32>,
        expression: WasmJitAssignmentExpression,
    },
    ParameterDefault {
        parameter_index: u32,
    },
    StaticCondition {
        stamp_index: u32,
    },
    StampValue {
        stamp_index: u32,
    },
    Jacobian {
        stamp_index: u32,
        entry_index: u32,
    },
    ReactiveJacobian {
        stamp_index: u32,
        entry_index: u32,
    },
    NoisePowerSpectralDensity {
        noise_index: u32,
    },
    NoiseExponent {
        noise_index: u32,
    },
}

/// One deterministic export in a model-wide generated module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WasmJitValueEntry {
    export_name: String,
    role: WasmJitValueRole,
}

impl WasmJitValueEntry {
    pub fn export_name(&self) -> &str {
        &self.export_name
    }

    pub fn role(&self) -> &WasmJitValueRole {
        &self.role
    }
}

/// Verified model-wide scalar module and the exact semantic export manifest
/// needed by a browser worker to install its entry table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WasmJitModelArtifact {
    module: WasmJitArtifact,
    cache_key: String,
    entries: Vec<WasmJitValueEntry>,
    assignment_export: String,
    post_assignment_export: Option<String>,
    /// Whole-model drivers, present when the shared contribution-ordering rule
    /// allows fusing. Absent means the worker must use the per-entry exports.
    evaluation_kernel_export: Option<String>,
    stamp_kernel_export: Option<String>,
}

impl WasmJitModelArtifact {
    pub fn module(&self) -> &WasmJitArtifact {
        &self.module
    }

    pub fn cache_key(&self) -> &str {
        &self.cache_key
    }

    pub fn entries(&self) -> &[WasmJitValueEntry] {
        &self.entries
    }

    pub fn assignment_export(&self) -> &str {
        &self.assignment_export
    }

    pub fn post_assignment_export(&self) -> Option<&str> {
        self.post_assignment_export.as_deref()
    }

    pub fn evaluation_kernel_export(&self) -> Option<&str> {
        self.evaluation_kernel_export.as_deref()
    }

    pub fn stamp_kernel_export(&self) -> Option<&str> {
        self.stamp_kernel_export.as_deref()
    }

    pub fn into_module(self) -> WasmJitArtifact {
        self.module
    }
}

impl WasmJitPlanSummary {
    pub fn cache_key(&self) -> &str {
        &self.cache_key
    }

    pub fn entry_programs(&self) -> usize {
        self.entry_programs
    }

    pub fn assignment_programs(&self) -> usize {
        self.assignment_programs
    }

    pub fn operations(&self) -> usize {
        self.operations
    }

    pub fn maximum_stack_depth(&self) -> usize {
        self.maximum_stack_depth
    }

    pub fn emitted_value_module_bytes(&self) -> usize {
        self.emitted_value_module_bytes
    }
}

impl WasmJitArtifact {
    /// Encoded standard-WebAssembly bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// BLAKE3 identity of the complete verified module.
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn abi_version(&self) -> u32 {
        self.abi_version
    }

    pub fn emitter_version(&self) -> u32 {
        self.emitter_version
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

/// Failure at the untrusted generated-module boundary.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum WasmJitError {
    #[error("WASM JIT artifact is {actual} bytes, exceeding its {limit}-byte budget")]
    ArtifactTooLarge { actual: usize, limit: usize },
    #[error("WASM JIT binary validation failed: {0}")]
    BinaryValidation(String),
    #[error("WASM JIT contract verification failed: {0}")]
    Contract(String),
    #[error("WASM JIT encoding failed: {0}")]
    Encoding(String),
    #[error("WASM JIT canonical model planning failed: {0}")]
    Planning(String),
}

pub type WasmJitResult<T> = Result<T, WasmJitError>;

/// Build and validate the complete architecture-neutral model plan.
///
/// This is a real backend qualification boundary: all assignments, parameter
/// defaults, conditions, values, Jacobians, reactive Jacobians, and noise
/// programs are lowered through the same implementation used by x64 and
/// AArch64. Module emission consumes this plan directly.
pub fn qualify_model_plan(
    model: &CompiledModel,
    artifact: &CanonicalIrArtifact,
) -> WasmJitResult<WasmJitPlanSummary> {
    let plan = crate::jit::cfg_plan_builder::build_default_model_plan(model, artifact)
        .map_err(|error| WasmJitError::Planning(error.to_string()))?;
    summarize_model_plan(artifact, &plan)
}

/// Compile every scalar expression in a model through the shared canonical
/// plan and return one verified standard-WebAssembly module plus its semantic
/// entry manifest.
pub fn compile_model_value_module(
    model: &CompiledModel,
    artifact: &CanonicalIrArtifact,
) -> WasmJitResult<WasmJitModelArtifact> {
    let plan = crate::jit::cfg_plan_builder::build_default_model_plan(model, artifact)
        .map_err(|error| WasmJitError::Planning(error.to_string()))?;
    emit_model_value_module(artifact, &plan)
}

fn emit_model_value_module(
    canonical_ir: &CanonicalIrArtifact,
    plan: &NativeModelPlan,
) -> WasmJitResult<WasmJitModelArtifact> {
    struct PlannedValue<'a> {
        program: PlanProgramRef<'a>,
        role: WasmJitValueRole,
    }

    fn u32_index(index: usize, what: &str) -> WasmJitResult<u32> {
        u32::try_from(index)
            .map_err(|_| WasmJitError::Encoding(format!("{what} index exceeds wasm32")))
    }

    fn collect_assignments<'a>(
        assignments: &'a [NativeAssignment],
        phase: WasmJitAssignmentPhase,
        path: &mut Vec<u32>,
        entries: &mut Vec<PlannedValue<'a>>,
        kernel: &mut Vec<codegen::WasmAssignment>,
    ) -> WasmJitResult<()> {
        for (index, assignment) in assignments.iter().enumerate() {
            path.push(u32_index(index, "assignment")?);
            match assignment {
                NativeAssignment::Direct { var_index, program } => {
                    let value_entry = u32_index(entries.len(), "scalar entry")?;
                    entries.push(PlannedValue {
                        program: PlanProgramRef::Postfix(program),
                        role: WasmJitValueRole::Assignment {
                            phase,
                            path: path.clone(),
                            expression: WasmJitAssignmentExpression::DirectValue,
                        },
                    });
                    kernel.push(codegen::WasmAssignment::Direct {
                        variable_index: u32_index(*var_index, "variable")?,
                        value_entry,
                    });
                }
                NativeAssignment::Indexed {
                    base,
                    len,
                    lower,
                    index,
                    value,
                } => {
                    let index_entry = u32_index(entries.len(), "scalar entry")?;
                    entries.push(PlannedValue {
                        program: PlanProgramRef::Postfix(index),
                        role: WasmJitValueRole::Assignment {
                            phase,
                            path: path.clone(),
                            expression: WasmJitAssignmentExpression::IndexedIndex,
                        },
                    });
                    let value_entry = u32_index(entries.len(), "scalar entry")?;
                    entries.push(PlannedValue {
                        program: PlanProgramRef::Postfix(value),
                        role: WasmJitValueRole::Assignment {
                            phase,
                            path: path.clone(),
                            expression: WasmJitAssignmentExpression::IndexedValue,
                        },
                    });
                    kernel.push(codegen::WasmAssignment::Indexed {
                        base: u32_index(*base, "variable base")?,
                        len: u32_index(*len, "variable length")?,
                        lower: *lower,
                        index_entry,
                        value_entry,
                    });
                }
                NativeAssignment::Loop { condition, body } => {
                    let condition_entry = u32_index(entries.len(), "scalar entry")?;
                    entries.push(PlannedValue {
                        program: PlanProgramRef::Postfix(condition),
                        role: WasmJitValueRole::Assignment {
                            phase,
                            path: path.clone(),
                            expression: WasmJitAssignmentExpression::LoopCondition,
                        },
                    });
                    let mut loop_body = Vec::new();
                    collect_assignments(body, phase, path, entries, &mut loop_body)?;
                    kernel.push(codegen::WasmAssignment::Loop {
                        condition_entry,
                        body: loop_body,
                    });
                }
            }
            path.pop();
        }
        Ok(())
    }

    let mut planned = Vec::new();
    let mut assignment_kernel = Vec::new();
    collect_assignments(
        &plan.assignments,
        WasmJitAssignmentPhase::Assignment,
        &mut Vec::new(),
        &mut planned,
        &mut assignment_kernel,
    )?;
    let mut post_assignment_kernel = Vec::new();
    collect_assignments(
        &plan.post_assignments,
        WasmJitAssignmentPhase::PostAssignment,
        &mut Vec::new(),
        &mut planned,
        &mut post_assignment_kernel,
    )?;
    for (parameter_index, program) in plan.parameter_defaults.iter().enumerate() {
        if let Some(program) = program {
            planned.push(PlannedValue {
                program: program.borrow(),
                role: WasmJitValueRole::ParameterDefault {
                    parameter_index: u32_index(parameter_index, "parameter")?,
                },
            });
        }
    }
    for (stamp_index, program) in plan.static_conditions.iter().enumerate() {
        if let Some(program) = program {
            planned.push(PlannedValue {
                program: program.borrow(),
                role: WasmJitValueRole::StaticCondition {
                    stamp_index: u32_index(stamp_index, "stamp")?,
                },
            });
        }
    }
    // Scalar-entry indices of the stamp work, recorded while planning so the
    // fused drivers can call the same functions the per-entry path exports.
    let mut stamp_value_entries = Vec::with_capacity(plan.stamp_values.len());
    let mut stamp_jacobian_entries = Vec::with_capacity(plan.jacobians.len());
    for (stamp_index, program) in plan.stamp_values.iter().enumerate() {
        stamp_value_entries.push(u32_index(planned.len(), "scalar entry")?);
        planned.push(PlannedValue {
            program: program.borrow(),
            role: WasmJitValueRole::StampValue {
                stamp_index: u32_index(stamp_index, "stamp")?,
            },
        });
    }
    for (stamp_index, programs) in plan.jacobians.iter().enumerate() {
        let mut entries = Vec::with_capacity(programs.len());
        for (entry_index, program) in programs.iter().enumerate() {
            entries.push(u32_index(planned.len(), "scalar entry")?);
            planned.push(PlannedValue {
                program: program.borrow(),
                role: WasmJitValueRole::Jacobian {
                    stamp_index: u32_index(stamp_index, "stamp")?,
                    entry_index: u32_index(entry_index, "Jacobian")?,
                },
            });
        }
        stamp_jacobian_entries.push(entries);
    }
    for (stamp_index, programs) in plan.reactive_jacobians.iter().enumerate() {
        for (entry_index, program) in programs.iter().enumerate() {
            planned.push(PlannedValue {
                program: program.borrow(),
                role: WasmJitValueRole::ReactiveJacobian {
                    stamp_index: u32_index(stamp_index, "stamp")?,
                    entry_index: u32_index(entry_index, "reactive Jacobian")?,
                },
            });
        }
    }
    for (noise_index, program) in plan.noise_psd.iter().enumerate() {
        planned.push(PlannedValue {
            program: program.borrow(),
            role: WasmJitValueRole::NoisePowerSpectralDensity {
                noise_index: u32_index(noise_index, "noise")?,
            },
        });
    }
    for (noise_index, program) in plan.noise_exponents.iter().enumerate() {
        if let Some(program) = program {
            planned.push(PlannedValue {
                program: program.borrow(),
                role: WasmJitValueRole::NoiseExponent {
                    noise_index: u32_index(noise_index, "noise")?,
                },
            });
        }
    }

    let programs = planned
        .iter()
        .map(|entry| entry.program)
        .collect::<Vec<_>>();
    let mut kernels = vec![codegen::WasmAssignmentKernel {
        export_name: codegen::WASM_JIT_ASSIGNMENT_EXPORT,
        assignments: assignment_kernel,
    }];
    if !post_assignment_kernel.is_empty() {
        kernels.push(codegen::WasmAssignmentKernel {
            export_name: codegen::WASM_JIT_POST_ASSIGNMENT_EXPORT,
            assignments: post_assignment_kernel,
        });
    }
    // A driver that evaluates the whole model in one call, replacing one
    // JavaScript round trip per stamp value and per Jacobian entry. Eligibility
    // is the shared contribution-ordering rule the machine backends use, so the
    // browser never fuses a model they would not.
    let mut fused = Vec::new();
    let mut evaluation_kernel_export = None;
    let mut stamp_kernel_export = None;
    if plan.current_dependencies.evaluation_kernel_order_safe() {
        let mut stamps = Vec::with_capacity(stamp_value_entries.len());
        let mut jacobian_output_base = 0_u32;
        for (stamp_index, value_entry) in stamp_value_entries.iter().copied().enumerate() {
            let jacobian_entries = stamp_jacobian_entries
                .get(stamp_index)
                .cloned()
                .unwrap_or_default();
            let current_pair = plan
                .published_current_pairs
                .get(stamp_index)
                .copied()
                .flatten()
                .map(|(forward, reverse)| {
                    Ok::<_, WasmJitError>((
                        u32_index(forward, "current pair")?,
                        u32_index(reverse, "current pair")?,
                    ))
                })
                .transpose()?;
            let entry_count = u32_index(jacobian_entries.len(), "Jacobian")?;
            stamps.push(codegen::WasmKernelStamp {
                value_entry,
                current_pair,
                jacobian_entries,
                jacobian_output_base,
            });
            jacobian_output_base = jacobian_output_base
                .checked_add(entry_count)
                .ok_or_else(|| WasmJitError::Encoding("Jacobian output base overflow".into()))?;
        }

        evaluation_kernel_export = Some(codegen::WASM_JIT_EVALUATION_KERNEL_EXPORT.to_owned());
        fused.push(codegen::WasmFusedKernel {
            export_name: codegen::WASM_JIT_EVALUATION_KERNEL_EXPORT,
            assignment_kernel: 0,
            stamps: stamps.clone(),
            with_jacobians: false,
        });
        if plan.current_dependencies.stamp_kernel_order_safe() {
            stamp_kernel_export = Some(codegen::WASM_JIT_STAMP_KERNEL_EXPORT.to_owned());
            fused.push(codegen::WasmFusedKernel {
                export_name: codegen::WASM_JIT_STAMP_KERNEL_EXPORT,
                assignment_kernel: 0,
                stamps,
                with_jacobians: true,
            });
        }
    }

    let bytes = codegen::emit_verified_model_module(&programs, &kernels, &fused)?;
    let entries = planned
        .into_iter()
        .enumerate()
        .map(|(index, entry)| WasmJitValueEntry {
            export_name: format!("rspice_wasm_jit_value_{index:08x}"),
            role: entry.role,
        })
        .collect();
    let cache_key = model_cache_key(canonical_ir);
    Ok(WasmJitModelArtifact {
        module: WasmJitArtifact {
            digest: blake3::hash(&bytes).to_hex().to_string(),
            bytes,
            abi_version: WASM_JIT_ABI_VERSION,
            emitter_version: WASM_JIT_EMITTER_VERSION,
        },
        cache_key,
        entries,
        assignment_export: codegen::WASM_JIT_ASSIGNMENT_EXPORT.to_owned(),
        post_assignment_export: (kernels.len() == 2)
            .then(|| codegen::WASM_JIT_POST_ASSIGNMENT_EXPORT.to_owned()),
        evaluation_kernel_export,
        stamp_kernel_export,
    })
}

fn model_cache_key(artifact: &CanonicalIrArtifact) -> String {
    let mut identity = blake3::Hasher::new();
    identity.update(b"rspice-wasm-jit-model\0");
    identity.update(&WASM_JIT_ABI_VERSION.to_le_bytes());
    identity.update(&WASM_JIT_EMITTER_VERSION.to_le_bytes());
    identity.update(artifact.metadata.source_digest.as_bytes());
    identity.update(artifact.hir_digest.as_bytes());
    identity.update(artifact.mir_digest.as_bytes());
    identity.finalize().to_hex().to_string()
}

fn summarize_model_plan(
    artifact: &CanonicalIrArtifact,
    plan: &NativeModelPlan,
) -> WasmJitResult<WasmJitPlanSummary> {
    let mut entry_programs = 0_usize;
    let mut assignment_programs = 0_usize;
    let mut operations = 0_usize;
    let mut maximum_stack_depth = 0_usize;
    let mut emitted_programs = Vec::new();

    fn include_program<'a>(
        program: PlanProgramRef<'a>,
        programs: &mut usize,
        operations: &mut usize,
        maximum_stack_depth: &mut usize,
        emitted_programs: &mut Vec<PlanProgramRef<'a>>,
    ) -> WasmJitResult<()> {
        *programs = programs
            .checked_add(1)
            .ok_or_else(|| contract_error("model program count overflow"))?;
        *operations = operations
            .checked_add(program.operation_count())
            .ok_or_else(|| contract_error("model operation count overflow"))?;
        *maximum_stack_depth = (*maximum_stack_depth).max(program.max_stack_depth());
        emitted_programs.push(program);
        Ok(())
    }

    fn include_assignment<'a>(
        assignment: &'a NativeAssignment,
        programs: &mut usize,
        operations: &mut usize,
        maximum_stack_depth: &mut usize,
        emitted_programs: &mut Vec<PlanProgramRef<'a>>,
    ) -> WasmJitResult<()> {
        match assignment {
            NativeAssignment::Direct { program, .. } => include_program(
                PlanProgramRef::Postfix(program),
                programs,
                operations,
                maximum_stack_depth,
                emitted_programs,
            ),
            NativeAssignment::Indexed { index, value, .. } => {
                include_program(
                    PlanProgramRef::Postfix(index),
                    programs,
                    operations,
                    maximum_stack_depth,
                    emitted_programs,
                )?;
                include_program(
                    PlanProgramRef::Postfix(value),
                    programs,
                    operations,
                    maximum_stack_depth,
                    emitted_programs,
                )
            }
            NativeAssignment::Loop { condition, body } => {
                include_program(
                    PlanProgramRef::Postfix(condition),
                    programs,
                    operations,
                    maximum_stack_depth,
                    emitted_programs,
                )?;
                for assignment in body {
                    include_assignment(
                        assignment,
                        programs,
                        operations,
                        maximum_stack_depth,
                        emitted_programs,
                    )?;
                }
                Ok(())
            }
        }
    }

    for assignment in plan.assignments.iter().chain(&plan.post_assignments) {
        include_assignment(
            assignment,
            &mut assignment_programs,
            &mut operations,
            &mut maximum_stack_depth,
            &mut emitted_programs,
        )?;
    }
    for program in plan.parameter_defaults.iter().flatten() {
        include_program(
            program.borrow(),
            &mut entry_programs,
            &mut operations,
            &mut maximum_stack_depth,
            &mut emitted_programs,
        )?;
    }
    for program in plan.static_conditions.iter().flatten() {
        include_program(
            program.borrow(),
            &mut entry_programs,
            &mut operations,
            &mut maximum_stack_depth,
            &mut emitted_programs,
        )?;
    }
    for program in &plan.stamp_values {
        include_program(
            program.borrow(),
            &mut entry_programs,
            &mut operations,
            &mut maximum_stack_depth,
            &mut emitted_programs,
        )?;
    }
    for program in plan.jacobians.iter().flatten() {
        include_program(
            program.borrow(),
            &mut entry_programs,
            &mut operations,
            &mut maximum_stack_depth,
            &mut emitted_programs,
        )?;
    }
    for program in plan.reactive_jacobians.iter().flatten() {
        include_program(
            program.borrow(),
            &mut entry_programs,
            &mut operations,
            &mut maximum_stack_depth,
            &mut emitted_programs,
        )?;
    }
    for program in &plan.noise_psd {
        include_program(
            program.borrow(),
            &mut entry_programs,
            &mut operations,
            &mut maximum_stack_depth,
            &mut emitted_programs,
        )?;
    }
    for program in plan.noise_exponents.iter().flatten() {
        include_program(
            program.borrow(),
            &mut entry_programs,
            &mut operations,
            &mut maximum_stack_depth,
            &mut emitted_programs,
        )?;
    }

    let emitted_value_module_bytes =
        codegen::emit_verified_value_program_set(&emitted_programs)?.len();

    Ok(WasmJitPlanSummary {
        cache_key: model_cache_key(artifact),
        entry_programs,
        assignment_programs,
        operations,
        maximum_stack_depth,
        emitted_value_module_bytes,
    })
}

/// Emit and independently verify the secondary-module architecture probe.
///
/// The probe imports the already-instantiated RSpice linear memory. Given the
/// offset of a two-f64 frame, it loads the first value, doubles it, stores the
/// result in the second value, and returns status zero. This establishes the
/// exact shared-memory and async browser-compilation path used by later model
/// kernels without pretending that a JavaScript-only mock is a JIT backend.
pub fn emit_architecture_probe() -> WasmJitResult<WasmJitArtifact> {
    let bytes = encode_architecture_probe();
    verify_architecture_probe(&bytes)?;
    Ok(WasmJitArtifact {
        digest: blake3::hash(&bytes).to_hex().to_string(),
        bytes,
        abi_version: WASM_JIT_ABI_VERSION,
        emitter_version: WASM_JIT_EMITTER_VERSION,
    })
}

fn encode_architecture_probe() -> Vec<u8> {
    let mut module = Module::new();

    let mut types = TypeSection::new();
    types.ty().function([ValType::I32], [ValType::I32]);
    module.section(&types);

    let mut imports = ImportSection::new();
    imports.import(
        WASM_JIT_IMPORT_MODULE,
        WASM_JIT_MEMORY_IMPORT,
        MemoryType {
            minimum: 0,
            maximum: None,
            memory64: false,
            shared: false,
            page_size_log2: None,
        },
    );
    module.section(&imports);

    let mut functions = FunctionSection::new();
    functions.function(0);
    module.section(&functions);

    let mut exports = ExportSection::new();
    exports.export(WASM_JIT_PROBE_EXPORT, ExportKind::Func, 0);
    module.section(&exports);

    let mut contract = Vec::with_capacity(8);
    contract.extend_from_slice(&WASM_JIT_ABI_VERSION.to_le_bytes());
    contract.extend_from_slice(&WASM_JIT_EMITTER_VERSION.to_le_bytes());
    module.section(&CustomSection {
        name: Cow::Borrowed(WASM_JIT_CONTRACT_SECTION),
        data: Cow::Owned(contract),
    });

    let aligned_f64 = |offset| MemArg {
        offset,
        align: 3,
        memory_index: 0,
    };
    let mut probe = Function::new([]);
    probe.instruction(&Instruction::LocalGet(0));
    probe.instruction(&Instruction::LocalGet(0));
    probe.instruction(&Instruction::F64Load(aligned_f64(0)));
    probe.instruction(&Instruction::F64Const(2.0.into()));
    probe.instruction(&Instruction::F64Mul);
    probe.instruction(&Instruction::F64Store(aligned_f64(
        WASM_JIT_PROBE_OUTPUT_OFFSET,
    )));
    probe.instruction(&Instruction::I32Const(WASM_JIT_STATUS_OK));
    probe.instruction(&Instruction::End);

    let mut code = CodeSection::new();
    code.function(&probe);
    module.section(&code);
    module.finish()
}

/// Independently validate and translation-check an architecture probe.
pub fn verify_architecture_probe(bytes: &[u8]) -> WasmJitResult<()> {
    if bytes.len() > WASM_JIT_PROBE_SIZE_BUDGET_BYTES {
        return Err(WasmJitError::ArtifactTooLarge {
            actual: bytes.len(),
            limit: WASM_JIT_PROBE_SIZE_BUDGET_BYTES,
        });
    }
    Validator::new()
        .validate_all(bytes)
        .map_err(|error| WasmJitError::BinaryValidation(error.to_string()))?;

    let mut saw_version = false;
    let mut saw_type = false;
    let mut saw_import = false;
    let mut saw_function = false;
    let mut saw_export = false;
    let mut saw_contract = false;
    let mut saw_code_start = false;
    let mut code_bodies = 0_u32;

    for payload in Parser::new(0).parse_all(bytes) {
        let payload = payload.map_err(|error| WasmJitError::BinaryValidation(error.to_string()))?;
        match payload {
            Payload::Version { encoding, .. } => {
                require(!saw_version, "duplicate module header")?;
                require(
                    encoding == Encoding::Module,
                    "component encoding is forbidden",
                )?;
                saw_version = true;
            }
            Payload::TypeSection(reader) => {
                require(!saw_type, "duplicate type section")?;
                require(reader.count() == 1, "probe must define exactly one type")?;
                let types = reader
                    .into_iter_err_on_gc_types()
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|error| WasmJitError::Contract(error.to_string()))?;
                let ty = &types[0];
                require(
                    ty.params() == [wasmparser::ValType::I32],
                    "probe parameter must be i32",
                )?;
                require(
                    ty.results() == [wasmparser::ValType::I32],
                    "probe result must be i32",
                )?;
                saw_type = true;
            }
            Payload::ImportSection(reader) => {
                require(!saw_import, "duplicate import section")?;
                require(
                    reader.count() == 1,
                    "probe must import exactly one capability",
                )?;
                let group = reader
                    .into_iter()
                    .next()
                    .transpose()
                    .map_err(|error| WasmJitError::Contract(error.to_string()))?
                    .ok_or_else(|| contract_error("probe memory import is missing"))?;
                let Imports::Single(_, import) = group else {
                    return Err(contract_error("compact imports are forbidden"));
                };
                require(
                    import.module == WASM_JIT_IMPORT_MODULE,
                    "unexpected import namespace",
                )?;
                require(
                    import.name == WASM_JIT_MEMORY_IMPORT,
                    "unexpected imported capability",
                )?;
                let TypeRef::Memory(memory) = import.ty else {
                    return Err(contract_error("probe capability must be a memory"));
                };
                require(!memory.memory64, "memory64 is forbidden")?;
                require(
                    !memory.shared,
                    "shared memory is not part of the scalar probe",
                )?;
                require(
                    memory.initial == 0,
                    "probe import minimum must be zero pages",
                )?;
                require(
                    memory.maximum.is_none(),
                    "probe import maximum must be unconstrained",
                )?;
                require(
                    memory.page_size_log2.is_none(),
                    "custom page sizes are forbidden",
                )?;
                saw_import = true;
            }
            Payload::FunctionSection(reader) => {
                require(!saw_function, "duplicate function section")?;
                require(
                    reader.count() == 1,
                    "probe must define exactly one function",
                )?;
                let type_index = reader
                    .into_iter()
                    .next()
                    .transpose()
                    .map_err(|error| WasmJitError::Contract(error.to_string()))?
                    .ok_or_else(|| contract_error("probe function is missing"))?;
                require(type_index == 0, "probe function must use type zero")?;
                saw_function = true;
            }
            Payload::ExportSection(reader) => {
                require(!saw_export, "duplicate export section")?;
                require(
                    reader.count() == 1,
                    "probe must export exactly one entrypoint",
                )?;
                let export = reader
                    .into_iter()
                    .next()
                    .transpose()
                    .map_err(|error| WasmJitError::Contract(error.to_string()))?
                    .ok_or_else(|| contract_error("probe export is missing"))?;
                require(
                    export.name == WASM_JIT_PROBE_EXPORT,
                    "unexpected probe export name",
                )?;
                require(
                    export.kind == ExternalKind::Func,
                    "probe export must be a function",
                )?;
                require(export.index == 0, "probe must export function zero")?;
                saw_export = true;
            }
            Payload::CustomSection(section) => {
                require(!saw_contract, "duplicate contract section")?;
                require(
                    section.name() == WASM_JIT_CONTRACT_SECTION,
                    "unknown custom section",
                )?;
                let mut expected = Vec::with_capacity(8);
                expected.extend_from_slice(&WASM_JIT_ABI_VERSION.to_le_bytes());
                expected.extend_from_slice(&WASM_JIT_EMITTER_VERSION.to_le_bytes());
                require(
                    section.data() == expected,
                    "contract version payload mismatch",
                )?;
                saw_contract = true;
            }
            Payload::CodeSectionStart { count, size, .. } => {
                require(!saw_code_start, "duplicate code section")?;
                require(count == 1, "probe code section must contain one body")?;
                require(
                    usize::try_from(size).unwrap_or(usize::MAX) <= WASM_JIT_PROBE_SIZE_BUDGET_BYTES,
                    "probe code section exceeds its budget",
                )?;
                saw_code_start = true;
            }
            Payload::CodeSectionEntry(body) => {
                code_bodies = code_bodies
                    .checked_add(1)
                    .ok_or_else(|| contract_error("probe code-body count overflow"))?;
                require(code_bodies == 1, "probe contains extra code bodies")?;
                require(
                    body.get_locals_reader().map_err(binary_error)?.get_count() == 0,
                    "probe locals are forbidden",
                )?;
                verify_probe_body(body.get_operators_reader().map_err(binary_error)?)?;
            }
            Payload::End(_) => {}
            _ => return Err(contract_error("probe contains a forbidden section")),
        }
    }

    for (present, name) in [
        (saw_version, "module header"),
        (saw_type, "type section"),
        (saw_import, "import section"),
        (saw_function, "function section"),
        (saw_export, "export section"),
        (saw_contract, "contract section"),
        (saw_code_start && code_bodies == 1, "code section"),
    ] {
        require(present, &format!("probe is missing its {name}"))?;
    }
    Ok(())
}

fn verify_probe_body(mut operators: wasmparser::OperatorsReader<'_>) -> WasmJitResult<()> {
    match operators.read().map_err(binary_error)? {
        Operator::LocalGet { local_index: 0 } => {}
        _ => return Err(contract_error("probe output address is not frame base")),
    }
    match operators.read().map_err(binary_error)? {
        Operator::LocalGet { local_index: 0 } => {}
        _ => return Err(contract_error("probe input address is not frame base")),
    }
    match operators.read().map_err(binary_error)? {
        Operator::F64Load { memarg }
            if memarg.memory == 0 && memarg.offset == 0 && memarg.align == 3 => {}
        _ => {
            return Err(contract_error(
                "probe input load is not the approved aligned access",
            ));
        }
    }
    match operators.read().map_err(binary_error)? {
        Operator::F64Const { value } if value.bits() == 2.0_f64.to_bits() => {}
        _ => {
            return Err(contract_error(
                "probe multiplier is not the exact f64 constant 2.0",
            ));
        }
    }
    match operators.read().map_err(binary_error)? {
        Operator::F64Mul => {}
        _ => return Err(contract_error("probe arithmetic is not f64 multiplication")),
    }
    match operators.read().map_err(binary_error)? {
        Operator::F64Store { memarg }
            if memarg.memory == 0
                && memarg.offset == WASM_JIT_PROBE_OUTPUT_OFFSET
                && memarg.align == 3 => {}
        _ => {
            return Err(contract_error(
                "probe output store is not the approved aligned access",
            ));
        }
    }
    match operators.read().map_err(binary_error)? {
        Operator::I32Const {
            value: WASM_JIT_STATUS_OK,
        } => {}
        _ => return Err(contract_error("probe does not return the success status")),
    }
    match operators.read().map_err(binary_error)? {
        Operator::End => {}
        _ => {
            return Err(contract_error(
                "probe body does not end after the status result",
            ));
        }
    }
    require(operators.eof(), "probe body has trailing instructions")
}

fn require(condition: bool, detail: &str) -> WasmJitResult<()> {
    if condition {
        Ok(())
    } else {
        Err(contract_error(detail))
    }
}

fn contract_error(detail: impl Into<String>) -> WasmJitError {
    WasmJitError::Contract(detail.into())
}

fn binary_error(error: wasmparser::BinaryReaderError) -> WasmJitError {
    WasmJitError::BinaryValidation(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::{
        WASM_JIT_ABI_VERSION, WASM_JIT_EMITTER_VERSION, WASM_JIT_PROBE_FRAME_BYTES,
        WASM_JIT_PROBE_SIZE_BUDGET_BYTES, WasmJitExecutable, WasmJitExecutableEntry,
        WasmJitValueRole, compile_model_value_module, emit_architecture_probe, qualify_model_plan,
        verify_architecture_probe,
    };
    use crate::{CompilerOptions, VerilogACompiler};

    #[test]
    fn architecture_probe_is_small_deterministic_and_verified() {
        let first = emit_architecture_probe().expect("emit verified WASM JIT probe");
        let second = emit_architecture_probe().expect("re-emit verified WASM JIT probe");
        assert_eq!(first, second);
        assert_eq!(first.abi_version(), WASM_JIT_ABI_VERSION);
        assert_eq!(first.emitter_version(), WASM_JIT_EMITTER_VERSION);
        assert_eq!(first.digest().len(), 64);
        assert!(first.bytes().len() <= WASM_JIT_PROBE_SIZE_BUDGET_BYTES);
        assert_eq!(WASM_JIT_PROBE_FRAME_BYTES, 16);
    }

    #[test]
    fn verifier_rejects_tampering() {
        let artifact = emit_architecture_probe().expect("emit verified WASM JIT probe");
        let mut tampered = artifact.bytes().to_vec();
        let multiplier = 2.0_f64.to_le_bytes();
        let at = tampered
            .windows(multiplier.len())
            .position(|bytes| bytes == multiplier)
            .expect("encoded multiplier");
        tampered[at] ^= 1;
        assert!(verify_architecture_probe(&tampered).is_err());
    }

    #[test]
    fn verifier_rejects_oversized_artifacts_before_parsing() {
        let bytes = vec![0_u8; WASM_JIT_PROBE_SIZE_BUDGET_BYTES + 1];
        assert!(verify_architecture_probe(&bytes).is_err());
    }

    #[test]
    fn complete_resistor_model_uses_shared_canonical_plan() {
        let source = r#"
`include "disciplines.vams"
module wasm_resistor(p, n);
  inout p, n;
  electrical p, n;
  parameter real resistance = 2.0 from (0:inf);
  real voltage;
  analog begin
    voltage = V(p, n);
    I(p, n) <+ voltage / resistance;
  end
endmodule
"#;
        let report = VerilogACompiler::new(CompilerOptions::default())
            .compile_runtime(source, Some("wasm_resistor"))
            .expect("compile coherent WASM JIT inputs");
        let first = qualify_model_plan(&report.model, &report.canonical_ir)
            .expect("qualify complete model plan");
        let second = qualify_model_plan(&report.model, &report.canonical_ir)
            .expect("re-qualify deterministic model plan");

        assert_eq!(first, second);
        assert_eq!(first.cache_key().len(), 64);
        assert!(first.entry_programs() >= 2);
        assert!(first.assignment_programs() >= 1);
        assert!(first.operations() >= first.entry_programs());
        assert!(first.maximum_stack_depth() >= 1);
        assert!(first.emitted_value_module_bytes() > 0);

        let module = compile_model_value_module(&report.model, &report.canonical_ir)
            .expect("compile model-wide scalar module");
        assert_eq!(module.cache_key(), first.cache_key());
        assert!(module.module().bytes().len() > first.emitted_value_module_bytes());
        assert_eq!(module.module().digest().len(), 64);
        assert_eq!(module.assignment_export(), "rspice_wasm_jit_assign");
        assert!(module.entries().iter().enumerate().all(|(index, entry)| {
            entry.export_name() == format!("rspice_wasm_jit_value_{index:08x}")
        }));
        assert!(
            module
                .entries()
                .iter()
                .any(|entry| matches!(entry.role(), WasmJitValueRole::Assignment { .. }))
        );
        assert!(
            module
                .entries()
                .iter()
                .any(|entry| matches!(entry.role(), WasmJitValueRole::StampValue { .. }))
        );

        let executable = WasmJitExecutable::from_artifact(&report.model, &module)
            .expect("authenticate dense solver entry table");
        assert_eq!(
            executable
                .export(WasmJitExecutableEntry::ParameterDefault(0))
                .is_some(),
            report.model.parameters[0].default_program.is_some()
        );
        assert!(
            executable
                .export(WasmJitExecutableEntry::StampValue(0))
                .is_some()
        );
        assert!(
            executable
                .export(WasmJitExecutableEntry::Jacobian { stamp: 0, entry: 0 })
                .is_some()
        );
        assert_eq!(
            executable.export(WasmJitExecutableEntry::StaticCondition(0)),
            None
        );
        assert_eq!(
            executable.export(WasmJitExecutableEntry::StampValue(1)),
            None
        );
        assert_eq!(
            executable.export(WasmJitExecutableEntry::ReactiveJacobian { stamp: 0, entry: 0 }),
            None
        );
        assert_eq!(executable.export(WasmJitExecutableEntry::NoisePsd(0)), None);
        assert_eq!(
            executable.export(WasmJitExecutableEntry::NoiseExponent(0)),
            None
        );
    }

    /// A model whose contributions have visibly different shapes: one
    /// transcendental, one clamped, and one bilinear in two voltages. A driver
    /// that permuted stamps or Jacobian output slots would then publish
    /// obviously wrong numbers rather than plausible ones.
    const FUSED_KERNEL_SOURCE: &str = r#"
`include "disciplines.vams"
module wasm_kernel_pair(p, n, c);
  inout p, n, c;
  electrical p, n, c;
  parameter real conductance = 3.0;
  real shaped;
  analog begin
    shaped = exp(V(p, n)) + conductance;
    I(p, n) <+ shaped * conductance;
    I(c, n) <+ max(V(c, n), 1.0e-30) * conductance;
    I(p, c) <+ V(p, n) * V(c, n) * conductance;
  end
endmodule
"#;

    /// A model whose second contribution overflows to infinity at the
    /// harness's operating point: `V(p, n)` is ln 4, so the exponential's
    /// argument runs far past the largest one `exp` can represent.
    const FUSED_KERNEL_OVERFLOW_SOURCE: &str = r#"
`include "disciplines.vams"
module wasm_kernel_overflow(p, n, c);
  inout p, n, c;
  electrical p, n, c;
  analog begin
    I(p, n) <+ V(p, n);
    I(c, n) <+ exp(V(p, n) * 1000.0);
  end
endmodule
"#;

    /// A three-terminal model instantiated in an independent wasm engine, with
    /// a frame and its arrays laid out in linear memory.
    ///
    /// Every fused-driver test runs against the same frame the per-entry
    /// exports do, so the layout is written once here rather than per test.
    struct FusedKernelHarness {
        artifact: super::WasmJitModelArtifact,
        executable: WasmJitExecutable,
        store: wasmi::Store<()>,
        memory: wasmi::Memory,
        instance: wasmi::Instance,
        frame: Vec<u8>,
        /// Jacobian entry count per stamp, in model order.
        stamp_jacobians: Vec<usize>,
        parameters: usize,
        /// Compiled variable names, so a test can name the slot it reads back.
        variable_names: Vec<smol_str::SmolStr>,
    }

    impl FusedKernelHarness {
        const PARAMETERS: u32 = 512;
        const VOLTAGES: u32 = 640;
        const VARIABLES: u32 = 768;
        const PROGRAM_ACTIVE: u32 = 1024;
        const SEQUENTIAL_CURRENTS: u32 = 1152;
        const PAIR_CURRENTS: u32 = 1280;
        const JACOBIANS: u32 = 1536;

        fn new() -> Self {
            Self::for_source(FUSED_KERNEL_SOURCE, "wasm_kernel_pair")
        }

        fn for_source(source: &str, module_name: &str) -> Self {
            use wasmi::{Engine, Linker, Memory, MemoryType, Module, Store};

            use super::abi::{
                FRAME_ABI_VERSION_OFFSET, FRAME_BYTE_LEN_OFFSET, FRAME_CURRENTS_LEN_OFFSET,
                FRAME_CURRENTS_PTR_OFFSET, FRAME_JACOBIANS_LEN_OFFSET, FRAME_JACOBIANS_PTR_OFFSET,
                FRAME_MAGIC_OFFSET, FRAME_PARAMETERS_LEN_OFFSET, FRAME_PARAMETERS_PTR_OFFSET,
                FRAME_PRIOR_CURRENTS_LEN_OFFSET, FRAME_PRIOR_CURRENTS_PTR_OFFSET,
                FRAME_PROGRAM_ACTIVE_LEN_OFFSET, FRAME_PROGRAM_ACTIVE_PTR_OFFSET,
                FRAME_TERMINAL_VOLTAGES_LEN_OFFSET, FRAME_TERMINAL_VOLTAGES_PTR_OFFSET,
                FRAME_VARIABLES_LEN_OFFSET, FRAME_VARIABLES_PTR_OFFSET,
            };
            use super::{
                WASM_JIT_ABI_VERSION, WASM_JIT_EVAL_FRAME_BYTES, WASM_JIT_FRAME_MAGIC,
                WASM_JIT_IMPORT_MODULE, WASM_JIT_MEMORY_IMPORT,
            };

            let report = VerilogACompiler::new(CompilerOptions::default())
                .compile_runtime(source, Some(module_name))
                .expect("compile fused-kernel model");
            let artifact = compile_model_value_module(&report.model, &report.canonical_ir)
                .expect("compile fused-kernel module");
            let executable = WasmJitExecutable::from_artifact(&report.model, &artifact)
                .expect("authenticate fused-kernel entry table");
            let stamp_jacobians = report
                .model
                .stamp_programs
                .iter()
                .map(|stamp| stamp.jacobian_programs.len())
                .collect::<Vec<_>>();
            let stamp_count = stamp_jacobians.len();
            let jacobian_count = stamp_jacobians.iter().sum::<usize>();
            let parameters = report.model.parameters.len();
            let variable_names = report.model.variable_names.clone();

            let engine = Engine::default();
            let module = Module::new(&engine, artifact.module().bytes())
                .expect("compile fused-kernel module in independent engine");
            let mut store = Store::new(&engine, ());
            let memory = Memory::new(&mut store, MemoryType::new(1, None))
                .expect("allocate imported primary memory");
            let mut linker = Linker::new(&engine);
            linker
                .define(WASM_JIT_IMPORT_MODULE, WASM_JIT_MEMORY_IMPORT, memory)
                .expect("define memory import");
            linker
                .func_wrap(
                    WASM_JIT_IMPORT_MODULE,
                    super::codegen::WASM_JIT_EVAL_HELPER_IMPORT,
                    |_: i32,
                     opcode: i32,
                     aux0: i32,
                     aux1: i32,
                     aux2: i64,
                     operand0: f64,
                     operand1: f64,
                     operand2: f64,
                     operand3: f64,
                     operand4: f64|
                     -> f64 {
                        super::runtime::evaluate_helper(
                            opcode,
                            aux0,
                            aux1,
                            aux2,
                            [operand0, operand1, operand2, operand3, operand4],
                            &[],
                        )
                        .expect("fused-kernel helper operation")
                    },
                )
                .expect("define helper import");
            super::codegen::define_test_math_imports(&mut linker);
            let instance = linker
                .instantiate_and_start(&mut store, &module)
                .expect("instantiate fused-kernel module");

            let pair_len = (report.model.num_terminals + 1) * (report.model.num_terminals + 1);
            let mut frame = vec![0_u8; WASM_JIT_EVAL_FRAME_BYTES as usize];
            {
                let mut write = |offset: u64, value: u32| {
                    let offset = offset as usize;
                    frame[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
                };
                write(FRAME_MAGIC_OFFSET, WASM_JIT_FRAME_MAGIC);
                write(FRAME_ABI_VERSION_OFFSET, WASM_JIT_ABI_VERSION);
                write(FRAME_BYTE_LEN_OFFSET, WASM_JIT_EVAL_FRAME_BYTES);
                write(FRAME_PARAMETERS_PTR_OFFSET, Self::PARAMETERS);
                write(FRAME_PARAMETERS_LEN_OFFSET, parameters as u32);
                write(FRAME_TERMINAL_VOLTAGES_PTR_OFFSET, Self::VOLTAGES);
                write(
                    FRAME_TERMINAL_VOLTAGES_LEN_OFFSET,
                    report.model.num_terminals as u32,
                );
                write(FRAME_VARIABLES_PTR_OFFSET, Self::VARIABLES);
                write(
                    FRAME_VARIABLES_LEN_OFFSET,
                    report.model.num_variables as u32,
                );
                write(FRAME_PROGRAM_ACTIVE_PTR_OFFSET, Self::PROGRAM_ACTIVE);
                write(FRAME_PROGRAM_ACTIVE_LEN_OFFSET, stamp_count as u32);
                write(FRAME_PRIOR_CURRENTS_PTR_OFFSET, Self::SEQUENTIAL_CURRENTS);
                write(FRAME_PRIOR_CURRENTS_LEN_OFFSET, stamp_count as u32);
                write(FRAME_CURRENTS_PTR_OFFSET, Self::PAIR_CURRENTS);
                write(FRAME_CURRENTS_LEN_OFFSET, pair_len as u32);
                write(FRAME_JACOBIANS_PTR_OFFSET, Self::JACOBIANS);
                write(FRAME_JACOBIANS_LEN_OFFSET, jacobian_count as u32);
            }

            Self {
                artifact,
                executable,
                store,
                memory,
                instance,
                frame,
                stamp_jacobians,
                parameters,
                variable_names,
            }
        }

        fn stamp_count(&self) -> usize {
            self.stamp_jacobians.len()
        }

        fn jacobian_count(&self) -> usize {
            self.stamp_jacobians.iter().sum()
        }

        /// First flat Jacobian slot belonging to `stamp`, the same running base
        /// the device uses to read the array back.
        fn jacobian_base(&self, stamp: usize) -> usize {
            self.stamp_jacobians[..stamp].iter().sum()
        }

        /// Restore the frame, the inputs, and every output array.
        fn reset(&mut self) {
            use std::mem::size_of;

            let stamp_count = self.stamp_count();
            let jacobian_count = self.jacobian_count();
            self.memory
                .write(&mut self.store, 0, &self.frame)
                .expect("write frame");
            for index in 0..self.parameters {
                self.write_f64(Self::PARAMETERS as usize + index * size_of::<f64>(), 3.0);
            }
            for (index, value) in [4.0_f64.ln(), 0.0, 0.75].into_iter().enumerate() {
                self.write_f64(Self::VOLTAGES as usize + index * size_of::<f64>(), value);
            }
            self.memory
                .write(
                    &mut self.store,
                    Self::PROGRAM_ACTIVE as usize,
                    &vec![1_u8; stamp_count],
                )
                .expect("write activation");
            self.memory
                .write(
                    &mut self.store,
                    Self::SEQUENTIAL_CURRENTS as usize,
                    &vec![0_u8; stamp_count * size_of::<f64>()],
                )
                .expect("clear contributions");
            self.memory
                .write(
                    &mut self.store,
                    Self::JACOBIANS as usize,
                    &vec![0_u8; jacobian_count * size_of::<f64>()],
                )
                .expect("clear Jacobians");
        }

        fn deactivate_every_stamp(&mut self) {
            let stamp_count = self.stamp_count();
            self.memory
                .write(
                    &mut self.store,
                    Self::PROGRAM_ACTIVE as usize,
                    &vec![0_u8; stamp_count],
                )
                .expect("deactivate every stamp");
        }

        /// Overwrite one frame field in linear memory, after [`Self::reset`]
        /// has written the frame there.
        fn poke_frame_u32(&mut self, offset: u64, value: u32) {
            self.memory
                .write(&mut self.store, offset as usize, &value.to_le_bytes())
                .expect("write frame field");
        }

        fn write_f64(&mut self, offset: usize, value: f64) {
            self.memory
                .write(&mut self.store, offset, &value.to_le_bytes())
                .expect("write f64");
        }

        fn read_i32(&self, offset: usize) -> i32 {
            let raw = self
                .memory
                .data(&self.store)
                .get(offset..offset + std::mem::size_of::<i32>())
                .expect("read i32");
            i32::from_le_bytes(raw.try_into().unwrap())
        }

        fn read_f64(&self, offset: usize) -> f64 {
            let raw = self
                .memory
                .data(&self.store)
                .get(offset..offset + std::mem::size_of::<f64>())
                .expect("read f64");
            f64::from_le_bytes(raw.try_into().unwrap())
        }

        /// Call an export with frame offset zero, returning its status.
        fn call(&mut self, export: &str) -> i32 {
            let entry = self
                .instance
                .get_typed_func::<i32, i32>(&self.store, export)
                .expect("resolve export");
            entry.call(&mut self.store, 0).expect("call export")
        }

        fn stamp_value_export(&self, stamp: usize) -> String {
            self.executable
                .export(WasmJitExecutableEntry::StampValue(stamp))
                .expect("stamp value export")
                .to_owned()
        }

        fn jacobian_export(&self, stamp: usize, entry: usize) -> String {
            self.executable
                .export(WasmJitExecutableEntry::Jacobian { stamp, entry })
                .expect("Jacobian export")
                .to_owned()
        }
    }

    /// The shape `ekv3_302.00` has: a scratch variable a contribution reads
    /// and a later statement reassigns.
    const REUSED_AFTER_READ: &str = r#"
`include "disciplines.vams"
module wasm_reuse(p, n);
  inout p, n;
  electrical p, n;
  real tmp;
  real reported;
  analog begin
    tmp = 2.5e-3;
    I(p, n) <+ V(p, n) * tmp;
    tmp = 1.25;
    reported = tmp;
  end
endmodule
"#;

    /// The browser route reads the definition reaching the contribution.
    ///
    /// `tests/reaching_definitions.rs` holds this on the VM and the machine
    /// backends; this is the same arithmetic executed as WebAssembly, because
    /// the module is compiled from the same plan and nothing else pins that the
    /// spliced copy survives into the emitted module. The expected value is the
    /// LRM's — `V * 2.5e-3` at the reading contribution's own program point —
    /// so it cannot agree with a defect two routes share, and the later write is
    /// read back to prove it happened.
    #[test]
    fn a_wasm_contribution_reads_the_definition_reaching_it() {
        use std::mem::size_of;

        use super::abi::FRAME_RESULT_OFFSET;

        const BIAS: f64 = 4.0;
        const REACHING: f64 = 2.5e-3;
        const OVERWRITTEN: f64 = 1.25;

        let mut harness = FusedKernelHarness::for_source(REUSED_AFTER_READ, "wasm_reuse");
        let reported = harness
            .variable_names
            .iter()
            .position(|name| name == "reported")
            .expect("the module declares the reporting variable");
        let assignment_export = harness.artifact.assignment_export().to_owned();
        let value_export = harness.stamp_value_export(0);

        harness.reset();
        harness.write_f64(FusedKernelHarness::VOLTAGES as usize, BIAS);
        harness.write_f64(
            FusedKernelHarness::VOLTAGES as usize + size_of::<f64>(),
            0.0,
        );
        assert_eq!(harness.call(&assignment_export), 0);
        assert_eq!(harness.call(&value_export), 0);

        assert_eq!(
            harness.read_f64(FRAME_RESULT_OFFSET as usize),
            BIAS * REACHING,
            "the contribution reads tmp at its reaching definition"
        );
        assert_eq!(
            harness.read_f64(FusedKernelHarness::VARIABLES as usize + reported * size_of::<f64>()),
            OVERWRITTEN,
            "the later write has to have happened, or this pin proves nothing"
        );
    }

    /// The fused driver publishes exactly what the per-entry path produces.
    ///
    /// Fusing is the whole point of the browser backend's hot path -- one call
    /// instead of one JavaScript round trip per stamp value -- so the risk it
    /// carries is that the driver and the individual exports disagree. This
    /// runs both against the same frame and compares the published
    /// contribution array.
    #[test]
    fn fused_evaluation_kernel_publishes_the_per_entry_results() {
        use std::mem::size_of;

        use super::abi::FRAME_RESULT_OFFSET;

        let mut harness = FusedKernelHarness::new();
        let kernel_export = harness
            .artifact
            .evaluation_kernel_export()
            .expect("a model with no prior-current reads must fuse")
            .to_owned();
        let assignment_export = harness.artifact.assignment_export().to_owned();
        let stamp_count = harness.stamp_count();
        assert!(stamp_count >= 3, "the model must exercise several stamps");

        // Per-entry path: assignment kernel, then each stamp value export.
        harness.reset();
        assert_eq!(harness.call(&assignment_export), 0);
        let mut per_entry = Vec::with_capacity(stamp_count);
        for stamp in 0..stamp_count {
            let export = harness.stamp_value_export(stamp);
            assert_eq!(harness.call(&export), 0);
            per_entry.push(harness.read_f64(FRAME_RESULT_OFFSET as usize));
        }

        // Fused path: one call publishing every stamp.
        harness.reset();
        assert_eq!(harness.call(&kernel_export), 0);
        for (stamp, expected) in per_entry.iter().copied().enumerate() {
            let published = harness.read_f64(
                FusedKernelHarness::SEQUENTIAL_CURRENTS as usize + stamp * size_of::<f64>(),
            );
            assert_eq!(
                published.to_bits(),
                expected.to_bits(),
                "stamp {stamp}: fused kernel published {published}, per-entry produced {expected}"
            );
        }
        assert!(
            per_entry.iter().any(|value| *value != 0.0),
            "the comparison must exercise non-trivial contributions"
        );

        // A deactivated stamp must be skipped, exactly as the native drivers do.
        harness.reset();
        harness.deactivate_every_stamp();
        assert_eq!(harness.call(&kernel_export), 0);
        for stamp in 0..stamp_count {
            assert_eq!(
                harness.read_f64(
                    FusedKernelHarness::SEQUENTIAL_CURRENTS as usize + stamp * size_of::<f64>()
                ),
                0.0,
                "stamp {stamp} published a contribution while inactive"
            );
        }
    }

    /// The fused stamp driver writes each derivative to the slot the device
    /// reads it back from.
    ///
    /// The device consumes one flat, model-order Jacobian array indexed by a
    /// running per-stamp base. An error in the emitted output index would not
    /// fail anything: it would attribute one contribution's derivative to
    /// another and converge on a wrong answer. So the entries are compared
    /// slot by slot against the per-entry exports, and the model is chosen so
    /// the values differ.
    #[test]
    fn fused_stamp_kernel_publishes_the_per_entry_jacobians() {
        use std::mem::size_of;

        use super::abi::{FRAME_JACOBIANS_LEN_OFFSET, FRAME_RESULT_OFFSET};

        let mut harness = FusedKernelHarness::new();
        let kernel_export = harness
            .artifact
            .stamp_kernel_export()
            .expect("a model whose Jacobians read no later contribution must fuse")
            .to_owned();
        let assignment_export = harness.artifact.assignment_export().to_owned();
        let stamp_count = harness.stamp_count();
        assert!(
            harness.stamp_jacobians.iter().any(|entries| *entries >= 2),
            "one stamp must carry several Jacobian entries, or the per-stamp \
             output base is never exercised"
        );

        // Per-entry path, interleaved exactly as the driver runs it: a stamp's
        // value publishes before its own derivatives are evaluated.
        harness.reset();
        assert_eq!(harness.call(&assignment_export), 0);
        let mut per_entry = Vec::with_capacity(harness.jacobian_count());
        for stamp in 0..stamp_count {
            let export = harness.stamp_value_export(stamp);
            assert_eq!(harness.call(&export), 0);
            let value = harness.read_f64(FRAME_RESULT_OFFSET as usize);
            harness.write_f64(
                FusedKernelHarness::SEQUENTIAL_CURRENTS as usize + stamp * size_of::<f64>(),
                value,
            );
            for entry in 0..harness.stamp_jacobians[stamp] {
                let export = harness.jacobian_export(stamp, entry);
                assert_eq!(harness.call(&export), 0);
                per_entry.push(harness.read_f64(FRAME_RESULT_OFFSET as usize));
            }
        }

        // Fused path: one call publishing every contribution and derivative.
        harness.reset();
        assert_eq!(harness.call(&kernel_export), 0);
        for stamp in 0..stamp_count {
            let base = harness.jacobian_base(stamp);
            for entry in 0..harness.stamp_jacobians[stamp] {
                let slot = base + entry;
                let published = harness
                    .read_f64(FusedKernelHarness::JACOBIANS as usize + slot * size_of::<f64>());
                let expected = per_entry[slot];
                assert_eq!(
                    published.to_bits(),
                    expected.to_bits(),
                    "stamp {stamp} entry {entry}: fused driver published {published}, \
                     per-entry produced {expected}"
                );
            }
        }
        assert!(
            per_entry.iter().filter(|value| **value != 0.0).count() >= 2,
            "the comparison must exercise several non-trivial derivatives"
        );
        assert!(
            per_entry.windows(2).any(|pair| pair[0] != pair[1]),
            "the derivatives must differ, or a permuted output slot would go \
             unnoticed"
        );

        // A deactivated stamp evaluates no derivative at all.
        harness.reset();
        harness.deactivate_every_stamp();
        assert_eq!(harness.call(&kernel_export), 0);
        for slot in 0..harness.jacobian_count() {
            assert_eq!(
                harness.read_f64(FusedKernelHarness::JACOBIANS as usize + slot * size_of::<f64>()),
                0.0,
                "Jacobian slot {slot} was written while its stamp was inactive"
            );
        }

        // A frame that disagrees with the module about the model's shape fails
        // the dispatch rather than leaving a stale zero to be stamped as a real
        // derivative.
        harness.reset();
        harness.poke_frame_u32(FRAME_JACOBIANS_LEN_OFFSET, 0);
        assert_eq!(
            harness.call(&kernel_export),
            super::WASM_JIT_STATUS_RUNTIME_ERROR
        );
    }

    /// A non-finite contribution stops the driver before it is published.
    ///
    /// On the per-entry path the device audits each value as it comes back, so
    /// an infinity never reaches the solver. A fused driver publishes into the
    /// context itself, so the audit has to be inside the generated code -- and
    /// it has to run before the store, not after, or the contribution the
    /// device reads back is already wrong.
    #[test]
    fn a_non_finite_contribution_fails_the_fused_driver_before_publishing_it() {
        use std::mem::size_of;

        use super::abi::FRAME_ERROR_STATUS_OFFSET;

        let mut harness =
            FusedKernelHarness::for_source(FUSED_KERNEL_OVERFLOW_SOURCE, "wasm_kernel_overflow");
        let kernel_export = harness
            .artifact
            .evaluation_kernel_export()
            .expect("a model with no prior-current reads must fuse")
            .to_owned();
        assert_eq!(harness.stamp_count(), 2);

        harness.reset();
        assert_eq!(
            harness.call(&kernel_export),
            super::WASM_JIT_STATUS_RUNTIME_ERROR,
            "an overflowing contribution must fail the dispatch"
        );
        assert_eq!(
            harness.read_i32(FRAME_ERROR_STATUS_OFFSET as usize),
            super::WASM_JIT_STATUS_RUNTIME_ERROR,
            "the frame must record why, not only that the status was non-zero"
        );

        let published = harness.read_f64(FusedKernelHarness::SEQUENTIAL_CURRENTS as usize);
        assert_eq!(
            published,
            4.0_f64.ln(),
            "the driver must keep the contributions it evaluated before the bad one"
        );
        assert_eq!(
            harness.read_f64(FusedKernelHarness::SEQUENTIAL_CURRENTS as usize + size_of::<f64>()),
            0.0,
            "the infinity must never be stored where the device reads contributions"
        );
    }

    #[cfg(all(
        feature = "native",
        any(target_arch = "aarch64", target_arch = "x86_64")
    ))]
    #[test]
    fn browser_wasm_and_host_native_jits_match_for_real_pure_model() {
        use std::mem::size_of;

        use wasmi::{Engine, Linker, Memory, MemoryType, Module, Store};

        use super::{
            WASM_JIT_IMPORT_MODULE, WASM_JIT_MEMORY_IMPORT, WasmJitValueRole,
            codegen::WASM_JIT_EVAL_HELPER_IMPORT,
        };
        use crate::native::{EvalContext, compile_native_with_canonical_ir};
        use crate::wasm_jit::abi::{
            FRAME_ABI_VERSION_OFFSET, FRAME_BYTE_LEN_OFFSET, FRAME_MAGIC_OFFSET,
            FRAME_PARAMETERS_LEN_OFFSET, FRAME_PARAMETERS_PTR_OFFSET, FRAME_RESULT_OFFSET,
            FRAME_TERMINAL_VOLTAGES_LEN_OFFSET, FRAME_TERMINAL_VOLTAGES_PTR_OFFSET,
            FRAME_VARIABLES_LEN_OFFSET, FRAME_VARIABLES_PTR_OFFSET,
        };

        let source = r#"
`include "disciplines.vams"
module jit_differential(p, n);
  inout p, n;
  electrical p, n;
  parameter real gain = 2.0;
  real scaled;
  analog begin
    scaled = exp(V(p, n)) + gain;
    I(p, n) <+ scaled / gain + flicker_noise(3.0 * gain, 1.5, "differential");
  end
endmodule
"#;
        let report = VerilogACompiler::new(CompilerOptions::default())
            .compile_runtime(source, Some("jit_differential"))
            .expect("compile differential model");
        let native = compile_native_with_canonical_ir(&report.model, &report.canonical_ir)
            .expect("compile host-native differential model");
        let artifact = compile_model_value_module(&report.model, &report.canonical_ir)
            .expect("compile browser-WASM differential model");

        let params = [2.0_f64];
        let voltages = [4.0_f64.ln(), 0.0];
        let mut native_variables = vec![0.0_f64; report.model.num_variables];
        let mut native_context = EvalContext::empty_for_test();
        native_context.params = params.as_ptr();
        native_context.voltages = voltages.as_ptr();
        native_context.num_terminals = report.model.num_terminals;
        native.run_assignments(&native_context, native_variables.as_mut_ptr());
        assert!(native_context.take_runtime_error().is_none());

        let engine = Engine::default();
        let module = Module::new(&engine, artifact.module().bytes())
            .expect("compile generated module in independent WebAssembly engine");
        let mut store = Store::new(&engine, ());
        let memory = Memory::new(&mut store, MemoryType::new(1, None))
            .expect("allocate imported primary memory");
        let mut linker = Linker::new(&engine);
        linker
            .define(WASM_JIT_IMPORT_MODULE, WASM_JIT_MEMORY_IMPORT, memory)
            .expect("define memory import");
        linker
            .func_wrap(
                WASM_JIT_IMPORT_MODULE,
                WASM_JIT_EVAL_HELPER_IMPORT,
                |_: i32,
                 opcode: i32,
                 aux0: i32,
                 aux1: i32,
                 aux2: i64,
                 operand0: f64,
                 operand1: f64,
                 operand2: f64,
                 operand3: f64,
                 operand4: f64|
                 -> f64 {
                    super::runtime::evaluate_helper(
                        opcode,
                        aux0,
                        aux1,
                        aux2,
                        [operand0, operand1, operand2, operand3, operand4],
                        &[],
                    )
                    .expect("pure differential helper operation")
                },
            )
            .expect("define helper import");
        super::codegen::define_test_math_imports(&mut linker);
        let instance = linker
            .instantiate_and_start(&mut store, &module)
            .expect("instantiate browser-WASM differential model");

        const FRAME_OFFSET: usize = 0;
        const PARAMS_OFFSET: usize = 256;
        const VOLTAGES_OFFSET: usize = 512;
        const VARIABLES_OFFSET: usize = 768;
        let mut frame = vec![0_u8; super::WASM_JIT_EVAL_FRAME_BYTES as usize];
        let mut write_frame_u32 = |offset: u64, value: u32| {
            frame[offset as usize..offset as usize + size_of::<u32>()]
                .copy_from_slice(&value.to_le_bytes());
        };
        write_frame_u32(FRAME_MAGIC_OFFSET, super::WASM_JIT_FRAME_MAGIC);
        write_frame_u32(FRAME_ABI_VERSION_OFFSET, super::WASM_JIT_ABI_VERSION);
        write_frame_u32(FRAME_BYTE_LEN_OFFSET, super::WASM_JIT_EVAL_FRAME_BYTES);
        write_frame_u32(FRAME_PARAMETERS_PTR_OFFSET, PARAMS_OFFSET as u32);
        write_frame_u32(FRAME_PARAMETERS_LEN_OFFSET, params.len() as u32);
        write_frame_u32(FRAME_TERMINAL_VOLTAGES_PTR_OFFSET, VOLTAGES_OFFSET as u32);
        write_frame_u32(FRAME_TERMINAL_VOLTAGES_LEN_OFFSET, voltages.len() as u32);
        write_frame_u32(FRAME_VARIABLES_PTR_OFFSET, VARIABLES_OFFSET as u32);
        write_frame_u32(FRAME_VARIABLES_LEN_OFFSET, native_variables.len() as u32);
        memory
            .write(&mut store, FRAME_OFFSET, &frame)
            .expect("write WASM evaluation frame");
        for (index, value) in params.into_iter().enumerate() {
            memory
                .write(
                    &mut store,
                    PARAMS_OFFSET + index * size_of::<f64>(),
                    &value.to_le_bytes(),
                )
                .expect("write WASM parameter");
        }
        for (index, value) in voltages.into_iter().enumerate() {
            memory
                .write(
                    &mut store,
                    VOLTAGES_OFFSET + index * size_of::<f64>(),
                    &value.to_le_bytes(),
                )
                .expect("write WASM terminal voltage");
        }
        memory
            .write(
                &mut store,
                VARIABLES_OFFSET,
                &vec![0_u8; native_variables.len() * size_of::<f64>()],
            )
            .expect("clear WASM variables");

        let assignment = instance
            .get_typed_func::<i32, i32>(&store, artifact.assignment_export())
            .expect("resolve WASM assignment kernel");
        assert_eq!(
            assignment
                .call(&mut store, FRAME_OFFSET as i32)
                .expect("run WASM assignment kernel"),
            super::WASM_JIT_STATUS_OK
        );
        let wasm_variables = (0..native_variables.len())
            .map(|index| {
                let offset = VARIABLES_OFFSET + index * size_of::<f64>();
                f64::from_le_bytes(
                    memory.data(&store)[offset..offset + size_of::<f64>()]
                        .try_into()
                        .unwrap(),
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(wasm_variables, native_variables);
        let mut compared_stamp = false;
        let mut compared_jacobian = false;
        let mut compared_noise_psd = false;
        let mut compared_noise_exponent = false;
        for entry in artifact.entries() {
            let expected = match entry.role() {
                WasmJitValueRole::Assignment { .. } => continue,
                WasmJitValueRole::ParameterDefault { parameter_index } => native
                    .run_parameter_default(
                        *parameter_index as usize,
                        &native_context,
                        native_variables.as_ptr(),
                    ),
                WasmJitValueRole::StaticCondition { stamp_index } => native.run_static_condition(
                    *stamp_index as usize,
                    &native_context,
                    native_variables.as_ptr(),
                ),
                WasmJitValueRole::StampValue { stamp_index } => {
                    compared_stamp = true;
                    native.run_stamp_value(
                        *stamp_index as usize,
                        &native_context,
                        native_variables.as_ptr(),
                    )
                }
                WasmJitValueRole::Jacobian {
                    stamp_index,
                    entry_index,
                } => {
                    compared_jacobian = true;
                    native.run_jacobian(
                        *stamp_index as usize,
                        *entry_index as usize,
                        &native_context,
                        native_variables.as_ptr(),
                    )
                }
                WasmJitValueRole::ReactiveJacobian {
                    stamp_index,
                    entry_index,
                } => native.run_reactive_jacobian(
                    *stamp_index as usize,
                    *entry_index as usize,
                    &native_context,
                    native_variables.as_ptr(),
                ),
                WasmJitValueRole::NoisePowerSpectralDensity { noise_index } => {
                    compared_noise_psd = true;
                    native.run_noise_psd(
                        *noise_index as usize,
                        &native_context,
                        native_variables.as_ptr(),
                    )
                }
                WasmJitValueRole::NoiseExponent { noise_index } => {
                    compared_noise_exponent = true;
                    native.run_noise_exponent(
                        *noise_index as usize,
                        &native_context,
                        native_variables.as_ptr(),
                    )
                }
            }
            .expect("native manifest entry");
            let function = instance
                .get_typed_func::<i32, i32>(&store, entry.export_name())
                .expect("resolve WASM manifest entry");
            assert_eq!(
                function
                    .call(&mut store, FRAME_OFFSET as i32)
                    .expect("run WASM manifest entry"),
                super::WASM_JIT_STATUS_OK,
                "{}",
                entry.export_name()
            );
            let actual = f64::from_le_bytes(
                memory.data(&store)
                    [FRAME_RESULT_OFFSET as usize..FRAME_RESULT_OFFSET as usize + size_of::<f64>()]
                    .try_into()
                    .unwrap(),
            );
            assert_eq!(
                actual.to_bits(),
                expected.to_bits(),
                "{} ({:?})",
                entry.export_name(),
                entry.role()
            );
        }
        assert!(compared_stamp);
        assert!(compared_jacobian);
        assert!(compared_noise_psd);
        assert!(compared_noise_exponent);
    }
}
