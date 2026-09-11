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
use crate::jit::model_plan::{NativeAssignmentCoverage, NativeModelPlan};
use crate::jit::plan_program::PlanProgramRef;
use thiserror::Error;
use wasm_encoder::{
    CodeSection, CustomSection, ExportKind, ExportSection, Function, FunctionSection,
    ImportSection, Instruction, MemArg, MemoryType, Module, TypeSection, ValType,
};
use wasmparser::{Encoding, ExternalKind, Imports, Operator, Parser, Payload, TypeRef, Validator};

/// Version of the linear-memory and helper-function contract understood by
/// emitted modules and the browser worker.
/// Version 13 adds simulation-parameter helper opcodes 470 and 471.
/// Version 14 adds bounded range-protected sum-products quotient helpers.
pub const WASM_JIT_ABI_VERSION: u32 = 14;

/// Version of the deterministic encoder. It participates in cache identity
/// independently of the ABI because code layout may change without changing
/// runtime frames.
///
/// 7 to 8 at the CFG plan flip. It had held at 7 through the whole block-model
/// lane because a postfix plan emitted a byte-identical module either way, so
/// nothing a browser had cached was stale. A CFG plan does not: the same module
/// now emits from block programs and one assignment prelude, so a worker
/// holding a 7 would answer with code this crate no longer produces.
///
/// 8 to 9 when the noise magnitudes joined that prelude. An 8 emitted every
/// `noise_psd` and `noise_exponents` entry as the shipped postfix program; a 9
/// emits each as a block program, and for all but a guarded site value that
/// block program is one load of a prelude slot an 8's module never published.
/// The functions have the same names and signatures, so nothing but the version
/// tells the two apart.
///
/// 9 to 10 when the CFG plan's assignment pass stopped being rooted on what the
/// *postfix* plan's entries read. A 9 emitted an assignment kernel keeping every
/// slot those entries used to load; a 10 emits one keeping what this plan reads
/// — the static conditions, the event-state leaves and the observable set — and
/// on `hisimhv_n5_va` that is eleven megabytes of machine code fewer on the
/// native side. Same exports, same frame, fewer assignments inside the kernel,
/// so again nothing but the version tells a stale worker's module from a
/// current one.
/// 10 to 11 when a block program's back edge started counting its trips. A 10
/// emits a `loop` a Verilog-A `while` can spin in forever; an 11 emits the same
/// `loop` with a counter, and returns [`WASM_JIT_STATUS_RUNTIME_ERROR`] on the
/// hundred-thousandth trip rather than hanging the worker. Same exports, same
/// frame, one more local per loop, so again nothing but the version tells a
/// stale worker's module from a current one.
///
/// 11 to 12 when the assignment kernel stopped publishing the externally
/// observable set. An 11 emits a kernel that computes every declared variable
/// as well as what the plan reads; a 12 emits one holding what the plan reads
/// and the two simulator-control tasks, which for most modules is nothing at
/// all. An 11 already knew how to omit `rspice_wasm_jit_assign` — the mechanism
/// landed there and no module reached it — so this is the version where the
/// export actually starts going missing, and a worker holding an 11 would both
/// compute variables a 12 does not and call an export a 12 no longer emits.
/// 12 to 13 separates physical-analysis global-event filters from
/// phase-sensitive analysis queries. Old modules must be rebuilt even though
/// their frame layout and helper signatures remain compatible.
/// 13 to 14 supplies zero-initialized analog locals on loop/conditional entry
/// edges before SSA merges, including paths with no explicit assignment.
/// 14 to 15 holds coefficients outside ddt during reactive differentiation;
/// old modules contain the spurious q * dk/dx term and must be rebuilt.
/// 15 to 16 preserves higher-order ddx and descending shadow update order.
/// 18 to 19 inserts integer parameter default conversions before their use in
/// dependent defaults and generated expressions.
/// 19 to 20 preserves signed integer arithmetic and its checked helper calls.
/// 20 to 21 preserves ddx primal validation through symbolic differentiation.
/// 21 to 22 preserves signed zero in primal arithmetic and derivative factors.
/// 22 to 23 requires sign proofs before specializing fractional powers.
/// 23 to 24 selects extrema tangents without inactive singular arithmetic.
/// 24 to 25 implements scalar hypot/atan2 AD and scales their derivative rules.
/// 25 to 26 preserves Hypot bytecode and stabilizes legacy math derivatives.
/// 26 to 27 avoids raw squares/cubes in legacy quotient derivatives.
/// 27 to 28 preserves finite legacy hypot curvature at extreme input gains.
/// 28 to 29 evaluates simulator queries and their selected fallbacks at runtime.
/// 29 to 30 protects quotient numerators against intermediate range loss in scalar and packed AD.
/// 30 to 31 publishes limiter affine residual correction entries.
/// 31 to 32 resolves flow probes through simultaneous current equations.
/// 32 to 33 preserves instance branch ownership and nested port-flow equations.
/// 33 to 34 retains distinct potential branches and consistent source directions.
/// Version 35 resolves declared grounds before allocating solver nodes.
/// Version 36 uses one canonical unknown per physical potential branch.
/// Version 37 preserves ordered source retention on switch branches.
pub const WASM_JIT_EMITTER_VERSION: u32 = 37;

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
    TaskGuard,
    TaskArgument,
}

/// Stable semantic role of one exported scalar entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WasmJitValueRole {
    /// The CFG route's assignment pass: one entry that publishes every value
    /// entry's output into a prelude slot, run once between the assignment
    /// kernel and the first stamp. At most one per module, and none at all for
    /// the postfix plans production compiles.
    Prelude,
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
    LimiterCorrection {
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
    assignment_coverage: NativeAssignmentCoverage,
    /// The module assignment pass, when it has steps to run.
    assignment_export: Option<String>,
    /// The CFG route's assignment pass, when the plan carried one.
    prelude_export: Option<String>,
    /// How many prelude slots one evaluation of that pass publishes.
    prelude_slots: usize,
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

    /// The CFG route's assignment pass, when this artifact carries one.
    pub fn prelude_export(&self) -> Option<&str> {
        self.prelude_export.as_deref()
    }

    /// How many prelude slots one evaluation publishes.
    pub fn prelude_slots(&self) -> usize {
        self.prelude_slots
    }

    /// The module assignment pass, when this artifact carries one.
    pub fn assignment_export(&self) -> Option<&str> {
        self.assignment_export.as_deref()
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
                NativeAssignment::Task(task) => {
                    let program = task
                        .finish_operand()
                        .ok_or_else(|| WasmJitError::Encoding("invalid analog task plan".into()))?;
                    let guard_entry = if let Some(guard) = &task.guard {
                        let entry = u32_index(entries.len(), "scalar entry")?;
                        entries.push(PlannedValue {
                            program: PlanProgramRef::Postfix(guard),
                            role: WasmJitValueRole::Assignment {
                                phase,
                                path: path.clone(),
                                expression: WasmJitAssignmentExpression::TaskGuard,
                            },
                        });
                        Some(entry)
                    } else {
                        None
                    };
                    let argument_entry = u32_index(entries.len(), "scalar entry")?;
                    entries.push(PlannedValue {
                        program: PlanProgramRef::Postfix(program),
                        role: WasmJitValueRole::Assignment {
                            phase,
                            path: path.clone(),
                            expression: WasmJitAssignmentExpression::TaskArgument,
                        },
                    });
                    kernel.push(codegen::WasmAssignment::Finish {
                        site: task.site,
                        guard_entry,
                        argument_entry,
                    });
                }
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
    // The CFG route's assignment pass, planned right after the assignment
    // kernels it reads the variables of and before every entry that reads one
    // of its slots. `None` for a postfix plan, and then nothing below changes.
    let prelude_entry = plan
        .prelude
        .as_ref()
        .map(|prelude| {
            let index = u32_index(planned.len(), "scalar entry")?;
            planned.push(PlannedValue {
                program: prelude.program.borrow(),
                role: WasmJitValueRole::Prelude,
            });
            Ok::<_, WasmJitError>(index)
        })
        .transpose()?;
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
    for (stamp_index, program) in plan.limiter_corrections.iter().enumerate() {
        if let Some(program) = program {
            planned.push(PlannedValue {
                program: program.borrow(),
                role: WasmJitValueRole::LimiterCorrection {
                    stamp_index: u32_index(stamp_index, "stamp")?,
                },
            });
        }
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
    // A module whose assignment pass has no steps emits no kernel for it, so
    // nothing exports one and no driver calls one.
    let mut kernels = Vec::with_capacity(2);
    let assignment_kernel_index = if assignment_kernel.is_empty() {
        None
    } else {
        kernels.push(codegen::WasmAssignmentKernel {
            export_name: codegen::WASM_JIT_ASSIGNMENT_EXPORT,
            assignments: assignment_kernel,
        });
        Some(u32_index(kernels.len() - 1, "assignment kernel")?)
    };
    let has_post_assignment_kernel = !post_assignment_kernel.is_empty();
    if has_post_assignment_kernel {
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
            assignment_kernel: assignment_kernel_index,
            prelude_entry,
            stamps: stamps.clone(),
            with_jacobians: false,
        });
        if plan.current_dependencies.stamp_kernel_order_safe()
            && plan.limiter_corrections.iter().all(Option::is_none)
        {
            stamp_kernel_export = Some(codegen::WASM_JIT_STAMP_KERNEL_EXPORT.to_owned());
            fused.push(codegen::WasmFusedKernel {
                export_name: codegen::WASM_JIT_STAMP_KERNEL_EXPORT,
                assignment_kernel: assignment_kernel_index,
                prelude_entry,
                stamps,
                with_jacobians: true,
            });
        }
    }

    let bytes = codegen::emit_verified_model_module(&programs, &kernels, &fused)?;
    let entries: Vec<WasmJitValueEntry> = planned
        .into_iter()
        .enumerate()
        .map(|(index, entry)| WasmJitValueEntry {
            export_name: format!("rspice_wasm_jit_value_{index:08x}"),
            role: entry.role,
        })
        .collect();
    let prelude_export_name = prelude_entry
        .map(|entry| {
            usize::try_from(entry)
                .ok()
                .and_then(|entry| entries.get(entry))
                .map(|entry: &WasmJitValueEntry| entry.export_name().to_owned())
                .ok_or_else(|| {
                    WasmJitError::Encoding("prelude entry has no published export".into())
                })
        })
        .transpose()?;
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
        assignment_coverage: plan.assignment_coverage,
        assignment_export: assignment_kernel_index
            .map(|_| codegen::WASM_JIT_ASSIGNMENT_EXPORT.to_owned()),
        prelude_export: prelude_export_name,
        prelude_slots: plan.prelude_slot_count(),
        post_assignment_export: has_post_assignment_kernel
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
            NativeAssignment::Task(task) => {
                for program in task.expressions() {
                    include_program(
                        PlanProgramRef::Postfix(program),
                        programs,
                        operations,
                        maximum_stack_depth,
                        emitted_programs,
                    )?;
                }
                Ok(())
            }
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
    for program in plan.limiter_corrections.iter().flatten() {
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

    /// A module whose assignment pass has no steps emits no kernel for it, and
    /// no driver calls one.
    ///
    /// The fixture is deliberately the smallest thing that reaches the
    /// backend: no procedural variable to assign, so no static condition to
    /// evaluate, no event state to commit and no simulator-control task
    /// variable to publish. The exports say the shape directly — an absent
    /// `assignment_export` is a module with no kernel to call — and the
    /// module still validates, which is what says the drivers agree.
    #[test]
    fn a_module_with_no_assignments_emits_no_assignment_kernel() {
        let source = r#"
`include "disciplines.vams"
module wasm_no_assignments(p, n);
  inout p, n;
  electrical p, n;
  parameter real resistance = 2.0 from (0:inf);
  analog I(p, n) <+ V(p, n) / resistance;
endmodule
"#;
        let report = VerilogACompiler::new(CompilerOptions::default())
            .compile_runtime(source, Some("wasm_no_assignments"))
            .expect("compile assignment-free WASM JIT inputs");
        assert!(
            report.model.assignment_steps.is_empty(),
            "the fixture must have nothing to assign"
        );

        let module = compile_model_value_module(&report.model, &report.canonical_ir)
            .expect("compile assignment-free model module");
        assert_eq!(
            module.assignment_export(),
            None,
            "an assignment pass with no steps must not be exported"
        );
        assert_eq!(module.post_assignment_export(), None);
        assert!(
            module.evaluation_kernel_export().is_some(),
            "the driver this pin is about has to be present to be checked"
        );
        assert!(
            !module
                .entries()
                .iter()
                .any(|entry| matches!(entry.role(), WasmJitValueRole::Assignment { .. })),
            "no assignment entry can exist for a pass with no steps"
        );

        // The control: one thing to assign and the kernel is back, so this is a
        // statement about the pass and not about the emitter giving up. It has
        // to be `$bound_step` rather than a procedural variable, because an
        // entry that read a procedural variable would read a prelude slot and
        // leave the pass empty here too.
        let with_assignment = r#"
`include "disciplines.vams"
module wasm_one_assignment(p, n);
  inout p, n;
  electrical p, n;
  parameter real resistance = 2.0 from (0:inf);
  analog begin
    $bound_step(1.0e-6);
    I(p, n) <+ V(p, n) / resistance;
  end
endmodule
"#;
        let report = VerilogACompiler::new(CompilerOptions::default())
            .compile_runtime(with_assignment, Some("wasm_one_assignment"))
            .expect("compile single-assignment WASM JIT inputs");
        let module = compile_model_value_module(&report.model, &report.canonical_ir)
            .expect("compile single-assignment model module");
        assert_eq!(module.assignment_export(), Some("rspice_wasm_jit_assign"));
    }

    #[test]
    fn complete_resistor_model_uses_shared_canonical_plan() {
        // `$bound_step` is what gives this module an assignment pass to check.
        // The stepper reads it back out of the variable array and no entry ever
        // reads it, so it is one of the two names a plan roots its pass on; a
        // procedural variable would be published through a prelude slot instead
        // and leave the pass with nothing in it.
        let source = r#"
`include "disciplines.vams"
module wasm_resistor(p, n);
  inout p, n;
  electrical p, n;
  parameter real resistance = 2.0 from (0:inf);
  analog begin
    $bound_step(1.0e-6);
    I(p, n) <+ V(p, n) / resistance;
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
        assert_eq!(module.assignment_export(), Some("rspice_wasm_jit_assign"));
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
        store: wasmi::Store<super::runtime::WasmJitRuntimeSession>,
        memory: wasmi::Memory,
        instance: wasmi::Instance,
        frame: Vec<u8>,
        /// Jacobian entry count per stamp, in model order.
        stamp_jacobians: Vec<usize>,
        parameters: usize,
        event_state_variables: Vec<usize>,
    }

    impl FusedKernelHarness {
        const PARAMETERS: u32 = 8704;
        const VOLTAGES: u32 = 8832;
        const VARIABLES: u32 = 8960;
        const PROGRAM_ACTIVE: u32 = 9216;
        const SEQUENTIAL_CURRENTS: u32 = 9344;
        const PAIR_CURRENTS: u32 = 9472;
        const JACOBIANS: u32 = 9728;
        /// Where the assignment prelude publishes.
        ///
        /// Past every other region, because it is the only one whose length is
        /// a property of the *plan* rather than of the model: a browser worker
        /// reads `WasmJitModelArtifact::prelude_slots` and allocates from it,
        /// and this harness has to do the same or the module stores through the
        /// frame at offset zero.
        const PRELUDE_SLOTS: u32 = 12288;

        fn new() -> Self {
            Self::for_source(FUSED_KERNEL_SOURCE, "wasm_kernel_pair")
        }

        fn for_source(source: &str, module_name: &str) -> Self {
            Self::for_source_with_plan(source, module_name, false)
        }

        fn for_source_with_plan(source: &str, module_name: &str, postfix: bool) -> Self {
            use std::mem::size_of;

            use wasmi::{Engine, Linker, Memory, MemoryType, Module, Store};

            use super::abi::{
                FRAME_ABI_VERSION_OFFSET, FRAME_BYTE_LEN_OFFSET, FRAME_CURRENTS_LEN_OFFSET,
                FRAME_CURRENTS_PTR_OFFSET, FRAME_JACOBIANS_LEN_OFFSET, FRAME_JACOBIANS_PTR_OFFSET,
                FRAME_MAGIC_OFFSET, FRAME_PARAMETERS_LEN_OFFSET, FRAME_PARAMETERS_PTR_OFFSET,
                FRAME_PRELUDE_SLOTS_LEN_OFFSET, FRAME_PRELUDE_SLOTS_PTR_OFFSET,
                FRAME_PRIOR_CURRENTS_LEN_OFFSET, FRAME_PRIOR_CURRENTS_PTR_OFFSET,
                FRAME_PROGRAM_ACTIVE_LEN_OFFSET, FRAME_PROGRAM_ACTIVE_PTR_OFFSET,
                FRAME_TERMINAL_VOLTAGES_LEN_OFFSET, FRAME_TERMINAL_VOLTAGES_PTR_OFFSET,
                FRAME_VARIABLES_LEN_OFFSET, FRAME_VARIABLES_PTR_OFFSET,
                WASM_JIT_MAX_EVAL_FRAME_BYTES,
            };
            use super::{
                WASM_JIT_ABI_VERSION, WASM_JIT_FRAME_MAGIC, WASM_JIT_IMPORT_MODULE,
                WASM_JIT_MEMORY_IMPORT,
            };

            let report = VerilogACompiler::new(CompilerOptions::default())
                .compile_runtime(source, Some(module_name))
                .expect("compile fused-kernel model");
            let artifact = if postfix {
                let plan = crate::jit::plan_builder::build_model_plan_with_canonical_ir(
                    &report.model,
                    &report.canonical_ir,
                )
                .expect("build postfix plan");
                super::emit_model_value_module(&report.canonical_ir, &plan)
            } else {
                compile_model_value_module(&report.model, &report.canonical_ir)
            }
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

            let engine = Engine::default();
            let module = Module::new(&engine, artifact.module().bytes())
                .expect("compile fused-kernel module in independent engine");
            let state_layout = crate::canonical_ir::state::CanonicalStateLayout::from_hir(
                &report.canonical_ir.hir,
            );
            let mut context = crate::vm::VmContext::with_states(
                report.model.num_terminals,
                state_layout
                    .family_len(crate::canonical_ir::state::CanonicalStateFamily::Integration),
            );
            context.cross_detectors.resize_with(
                state_layout
                    .family_len(crate::canonical_ir::state::CanonicalStateFamily::CrossDetector),
                Default::default,
            );
            context.variables.resize(report.model.num_variables, 0.0);
            context
                .configure_event_state_variables(&report.model.event_state_variables)
                .unwrap();
            let mut store =
                Store::new(&engine, super::runtime::WasmJitRuntimeSession::new(context));
            let memory = Memory::new(&mut store, MemoryType::new(1, None))
                .expect("allocate imported primary memory");
            let mut linker = Linker::new(&engine);
            linker
                .define(WASM_JIT_IMPORT_MODULE, WASM_JIT_MEMORY_IMPORT, memory)
                .expect("define memory import");
            let variable_count = report.model.num_variables;
            assert!(
                Self::VARIABLES as usize + variable_count * 8 <= Self::PROGRAM_ACTIVE as usize,
                "fixture variable storage overlaps the activation region"
            );
            linker
                .func_wrap(
                    WASM_JIT_IMPORT_MODULE,
                    super::codegen::WASM_JIT_EVAL_HELPER_IMPORT,
                    move |mut caller: wasmi::Caller<'_, super::runtime::WasmJitRuntimeSession>,
                          frame_offset: i32,
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
                        let variables = if matches!(opcode, 1 | 2) {
                            memory.data(&caller)[Self::VARIABLES as usize
                                ..Self::VARIABLES as usize + variable_count * 8]
                                .chunks_exact(8)
                                .map(|bytes| f64::from_le_bytes(bytes.try_into().unwrap()))
                                .collect::<Vec<_>>()
                        } else {
                            Vec::new()
                        };
                        super::runtime::evaluate_helper_with_session(
                            opcode,
                            aux0,
                            aux1,
                            aux2,
                            [operand0, operand1, operand2, operand3, operand4],
                            &variables,
                            Some(caller.data_mut()),
                        )
                        .unwrap_or_else(|_| {
                            // Match the primary Wasm runtime's helper failure contract.
                            memory
                                .write(
                                    &mut caller,
                                    frame_offset as usize
                                        + super::abi::FRAME_ERROR_STATUS_OFFSET as usize,
                                    &super::WASM_JIT_STATUS_RUNTIME_ERROR.to_le_bytes(),
                                )
                                .expect("write helper failure status");
                            0.0
                        })
                    },
                )
                .expect("define helper import");
            super::codegen::define_test_math_imports(&mut linker, memory);
            let instance = linker
                .instantiate_and_start(&mut store, &module)
                .expect("instantiate fused-kernel module");

            let pair_len = (report.model.num_terminals + 1) * (report.model.num_terminals + 1);
            let mut frame = vec![0_u8; WASM_JIT_MAX_EVAL_FRAME_BYTES as usize];
            {
                let mut write = |offset: u64, value: u32| {
                    let offset = offset as usize;
                    frame[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
                };
                write(FRAME_MAGIC_OFFSET, WASM_JIT_FRAME_MAGIC);
                write(FRAME_ABI_VERSION_OFFSET, WASM_JIT_ABI_VERSION);
                write(FRAME_BYTE_LEN_OFFSET, WASM_JIT_MAX_EVAL_FRAME_BYTES);
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
                let prelude_slots = artifact.prelude_slots();
                assert!(
                    Self::PRELUDE_SLOTS as usize + prelude_slots * size_of::<f64>() <= 64 * 1024,
                    "this model publishes {prelude_slots} prelude slots, past the harness's \
                     one page of linear memory"
                );
                write(FRAME_PRELUDE_SLOTS_PTR_OFFSET, Self::PRELUDE_SLOTS);
                write(FRAME_PRELUDE_SLOTS_LEN_OFFSET, prelude_slots as u32);
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
                event_state_variables: report.model.event_state_variables.clone(),
            }
        }

        /// Publish the prelude.
        ///
        /// The browser worker runs this once between the assignment pass and
        /// the first value export, and a fused kernel runs it for itself — so a
        /// test that calls a value export directly has to, or the export reads
        /// the zero its slot was seeded with. A plan without a prelude exports
        /// none and this does nothing.
        fn call_prelude(&mut self) {
            let Some(export) = self.artifact.prelude_export().map(str::to_owned) else {
                return;
            };
            assert_eq!(
                self.call(&export),
                0,
                "the prelude publishes without trapping"
            );
        }

        /// The assignment pass if the module has one, exactly as the driver
        /// calls it. A CFG plan roots the pass on what its entries read, and
        /// entries that read prelude slots root nothing, so a module can have
        /// no pass at all and still be whole.
        fn call_assignments(&mut self) {
            let Some(export) = self.artifact.assignment_export().map(str::to_owned) else {
                return;
            };
            assert_eq!(
                self.call(&export),
                0,
                "the assignment pass publishes without trapping"
            );
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

    #[test]
    fn wasm_switch_branches_preserve_ordered_retention_and_jacobians() {
        use super::abi::{
            FRAME_INTERNAL_VOLTAGES_LEN_OFFSET, FRAME_INTERNAL_VOLTAGES_PTR_OFFSET,
            FRAME_RESULT_OFFSET,
        };
        let source = "module switched(p); inout p; electrical p; analog begin
            V(p)<+2*I(p); I(p)<+5*V(p); V(p)<+3*I(p); V(p)<+4*I(p); end endmodule";
        let report = VerilogACompiler::default()
            .compile_runtime(source, None)
            .unwrap();
        let mut harness = FusedKernelHarness::for_source(source, "switched");
        harness.reset();
        harness.poke_frame_u32(
            FRAME_INTERNAL_VOLTAGES_PTR_OFFSET,
            FusedKernelHarness::VOLTAGES + 8,
        );
        harness.poke_frame_u32(FRAME_INTERNAL_VOLTAGES_LEN_OFFSET, 1);
        harness.write_f64(FusedKernelHarness::VOLTAGES as usize, 7.0);
        harness.write_f64(FusedKernelHarness::VOLTAGES as usize + 8, -1.0);
        harness.call_assignments();
        harness.call_prelude();
        let stamp = report.model.stamp_programs.len() - 1;
        for (entry, jacobian) in report.model.stamp_programs[stamp]
            .jacobian_programs
            .iter()
            .enumerate()
        {
            let expected = match jacobian.col_axis {
                crate::codegen::ColumnAxis::Node(0) => -1.0,
                crate::codegen::ColumnAxis::Node(1) => -7.0,
                other => panic!("unexpected switch constraint column {other:?}"),
            };
            let export = harness.jacobian_export(stamp, entry);
            assert_eq!(harness.call(&export), 0);
            assert_eq!(harness.read_f64(FRAME_RESULT_OFFSET as usize), expected);
        }
        let export = harness.stamp_value_export(stamp);
        assert_eq!(harness.call(&export), 0);
        assert_eq!(harness.read_f64(FRAME_RESULT_OFFSET as usize), 0.0);
    }

    #[test]
    fn wasm_limit_function_identifiers_preserve_previous_iteration_state() {
        use super::abi::FRAME_RESULT_OFFSET;
        let source = r#"
module callback(p,n,c);
 inout p,n,c; electrical p,n,c;
 analog function real pnjlim;
  input real proposed,previous;
  input integer increment;
  pnjlim=min(proposed,previous+0.25*increment);
 endfunction
 analog I(p,n)<+$limit(V(p,n),pnjlim,1.6);
endmodule
"#;
        for typed in [false, true] {
            let source = if typed {
                source
                    .replace("pnjlim", "clip")
                    .replace("clip,1.6", "\"clip\",\"typed\",-1.0,1.6")
            } else {
                source.into()
            };
            let mut harness = FusedKernelHarness::for_source(&source, "callback");
            harness.reset();
            let export = harness.stamp_value_export(0);
            for (proposed, expected) in [(0.0, 0.0), (2.0, 0.5), (2.0, 1.0), (2.0, 1.5), (2.0, 2.0)]
            {
                harness
                    .store
                    .data_mut()
                    .context_mut()
                    .begin_stateful_evaluation();
                harness.write_f64(
                    FusedKernelHarness::VOLTAGES as usize,
                    if typed { -proposed } else { proposed },
                );
                harness.call_assignments();
                harness.call_prelude();
                assert_eq!(harness.call(&export), 0);
                assert_eq!(harness.read_f64(FRAME_RESULT_OFFSET as usize), expected);
            }
        }
    }

    #[test]
    fn static_dae_wasm_event_bodies_retain_the_settled_candidate() {
        use super::abi::{FRAME_ANALYSIS_MASK_OFFSET, FRAME_RESULT_OFFSET};
        use crate::vm::VerilogAEvaluationMode as Mode;
        let source = include_str!("../../tests/fixtures/static_dae_events.va");
        for postfix in [false, true] {
            let mut harness =
                FusedKernelHarness::for_source_with_plan(source, "static_dae_events", postfix);
            harness.reset();
            let value = harness.stamp_value_export(0);
            for (time, voltage, initial, final_step, expected) in [
                (0.0, -1.0, true, false, -1.0),
                (0.5, 1.0, false, true, 11113.0),
            ] {
                let context = harness.store.data_mut().context_mut();
                context.analysis_type = 2;
                context.time = time;
                context.set_timestep(0.5);
                context.analysis_initial_step = initial;
                context.analysis_final_step = final_step;
                context.evaluation_mode = Mode::NewtonLimited;
                context.begin_stateful_evaluation();
                let mask = context.analysis_query_mask();
                let inputs = context.variables.clone();
                harness
                    .memory
                    .write(
                        &mut harness.store,
                        FRAME_ANALYSIS_MASK_OFFSET as usize,
                        &mask.to_le_bytes(),
                    )
                    .unwrap();
                for (index, value) in inputs.iter().copied().enumerate() {
                    harness.write_f64(FusedKernelHarness::VARIABLES as usize + index * 8, value);
                }
                harness.write_f64(FusedKernelHarness::VOLTAGES as usize, voltage);
                harness.call_assignments();
                harness.call_prelude();
                assert_eq!(harness.call(&value), 0);
                let variables = (0..inputs.len())
                    .map(|index| {
                        harness.read_f64(FusedKernelHarness::VARIABLES as usize + index * 8)
                    })
                    .collect::<Vec<_>>();
                let context = harness.store.data_mut().context_mut();
                context.variables.clone_from(&variables);
                context.evaluation_mode = Mode::StaticDaeProbe;
                context.begin_stateful_evaluation();
                let mask = context.analysis_query_mask();
                harness
                    .memory
                    .write(
                        &mut harness.store,
                        FRAME_ANALYSIS_MASK_OFFSET as usize,
                        &mask.to_le_bytes(),
                    )
                    .unwrap();
                harness.call_assignments();
                harness.call_prelude();
                assert_eq!(harness.call(&value), 0);
                assert_eq!(
                    harness.read_f64(FRAME_RESULT_OFFSET as usize),
                    expected,
                    "postfix={postfix}, time={time}, candidate={variables:?}"
                );
                for &index in &harness.event_state_variables {
                    assert_eq!(
                        harness.read_f64(FusedKernelHarness::VARIABLES as usize + index * 8),
                        variables[index]
                    );
                }
                harness
                    .store
                    .data_mut()
                    .context_mut()
                    .advance_state()
                    .unwrap();
            }
        }
    }

    #[test]
    fn static_dae_wasm_value_and_jacobian_retain_the_primal_candidate() {
        use super::abi::FRAME_RESULT_OFFSET;
        use crate::vm::VerilogAEvaluationMode as Mode;
        let source = "module static_history(p,n,c); inout p,n,c; electrical p,n,c;
            analog I(p,n)<+2.0*V(p,n)+ddt(3.0*V(p,n))+idt(V(p,n),0.0); endmodule";
        for postfix in [false, true] {
            let mut harness =
                FusedKernelHarness::for_source_with_plan(source, "static_history", postfix);
            harness.reset();
            let value = harness.stamp_value_export(0);
            let jacobian = harness.jacobian_export(0, 0);
            let context = harness.store.data_mut().context_mut();
            context.analysis_type = 2;
            context.set_timestep(0.5);
            context.begin_stateful_evaluation();
            harness.write_f64(FusedKernelHarness::VOLTAGES as usize, 2.0);
            harness.call_assignments();
            harness.call_prelude();
            assert_eq!(harness.call(&value), 0);
            assert_eq!(harness.read_f64(FRAME_RESULT_OFFSET as usize), 5.0);
            let context = harness.store.data_mut().context_mut();
            let states = context.state_values.clone();
            let valid = context.state_candidate_valid.clone();
            context.evaluation_mode = Mode::StaticDaeProbe;
            context.begin_stateful_evaluation();
            harness.write_f64(FusedKernelHarness::VOLTAGES as usize, 4.0);
            harness.call_assignments();
            harness.call_prelude();
            assert_eq!(harness.call(&value), 0);
            assert_eq!(harness.read_f64(FRAME_RESULT_OFFSET as usize), 9.0);
            assert_eq!(harness.call(&jacobian), 0);
            assert_eq!(harness.read_f64(FRAME_RESULT_OFFSET as usize), 2.0);
            let context = harness.store.data_mut().context_mut();
            assert_eq!(context.state_values, states);
            assert_eq!(context.state_candidate_valid, valid);
        }
    }

    #[test]
    fn wasm_default_limit_preserves_probe_history_in_automatic_and_postfix_plans() {
        use super::abi::FRAME_RESULT_OFFSET;
        use crate::vm::VerilogAEvaluationMode as Mode;
        let source = "module bounded(p,n,c); inout p,n,c; electrical p,n,c;
            analog I(p,n)<+$limit(V(p,n),0.25); endmodule";
        for postfix in [false, true] {
            let mut harness = FusedKernelHarness::for_source_with_plan(source, "bounded", postfix);
            harness.reset();
            let export = harness.stamp_value_export(0);
            for (mode, proposed, expected, active) in [
                (Mode::NewtonLimited, 0.0, 0.0, 0),
                (Mode::NewtonLimited, 1.0, 0.25, 1),
                (Mode::StaticProbe, 2.0, 2.0, 1),
                (Mode::SmallSignal, 2.0, 2.0, 1),
                (Mode::NewtonLimited, 1.0, 0.5, 1),
                (Mode::NewtonLimited, 1.0, 0.75, 1),
                (Mode::NewtonLimited, 1.0, 1.0, 0),
            ] {
                let context = harness.store.data_mut().context_mut();
                context.evaluation_mode = mode;
                context.begin_stateful_evaluation();
                if mode.limiting_enabled() {
                    context.limiter_active = 0;
                }
                harness.write_f64(FusedKernelHarness::VOLTAGES as usize, proposed);
                harness.call_assignments();
                harness.call_prelude();
                assert_eq!(harness.call(&export), 0);
                assert_eq!(
                    harness.read_f64(FRAME_RESULT_OFFSET as usize),
                    expected,
                    "postfix={postfix}; mode={mode:?}"
                );
                assert_eq!(
                    harness.store.data_mut().context_mut().limiter_active,
                    active
                );
            }
        }
    }

    #[test]
    fn wasm_observation_coverage_matches_published_variables() {
        for (expression, has_limiter) in [
            ("V(p,n)", false),
            ("$limit(V(p,n),0.25)", true),
            ("$limit(V(p,n),clip)", true),
            ("$limit(V(n,p),\"clip\",\"typed\",-1.0)", true),
        ] {
            let source = format!(
                "module observed(p,n,c); inout p,n,c; electrical p,n,c;
                 real limited, observed;
                 analog function real clip; input real proposed,previous;
                   clip=min(proposed,previous+0.25); endfunction
                 analog begin limited={expression}; observed=3*limited;
                   I(p,n)<+limited*limited; end endmodule"
            );
            let report = VerilogACompiler::default()
                .compile_runtime(&source, Some("observed"))
                .unwrap();
            let slot = report
                .model
                .variable_names
                .iter()
                .position(|name| name == "observed")
                .expect("declared observable variable");
            let offset = FusedKernelHarness::VARIABLES as usize + slot * size_of::<f64>();
            for postfix in [false, true] {
                let mut harness =
                    FusedKernelHarness::for_source_with_plan(&source, "observed", postfix);
                let published = harness.executable.publishes_observable_variables();
                assert_eq!(published, postfix || has_limiter, "{expression}");
                harness.reset();
                for (proposal, expected) in [(0.0, 0.0), (1.0, 0.75)] {
                    harness.write_f64(offset, f64::NAN);
                    harness.write_f64(FusedKernelHarness::VOLTAGES as usize, proposal);
                    harness
                        .store
                        .data_mut()
                        .context_mut()
                        .begin_stateful_evaluation();
                    harness.call_assignments();
                    harness.call_prelude();
                    let value = harness.stamp_value_export(0);
                    assert_eq!(harness.call(&value), 0);
                    let observed = harness.read_f64(offset);
                    if published {
                        assert_eq!(
                            observed,
                            if has_limiter {
                                expected
                            } else {
                                3.0 * proposal
                            }
                        );
                    } else {
                        assert!(observed.is_nan(), "CFG readback still requires observation");
                    }
                }
            }
        }
    }

    #[test]
    fn wasm_nonlinear_limit_reuses_previous_newton_history_for_derivatives() {
        use super::abi::FRAME_RESULT_OFFSET;
        for body in [
            "limited=$limit(V(p,n),0.25); I(p,n)<+limited*limited;",
            "limited=pow($limit(V(p,n),0.25),2); I(p,n)<+limited;",
            "I(p,n)<+pow($limit(V(p,n),0.25),2);",
            "I(p,n)<+pow($limit(V(p,n),clip),2);",
            "I(p,n)<+pow($limit(V(n,p),\"clip\",\"typed\",-1.0),2);",
        ] {
            let source = format!(
                "module bounded(p,n,c); inout p,n,c; electrical p,n,c; real limited; analog function real clip; input real proposed,previous; clip=min(proposed,previous+0.25); endfunction analog begin {body} end endmodule"
            );
            for postfix in [false, true] {
                let mut harness =
                    FusedKernelHarness::for_source_with_plan(&source, "bounded", postfix);
                harness.reset();
                let value = harness.stamp_value_export(0);
                let derivative = harness.jacobian_export(0, 0);
                let correction = harness
                    .executable
                    .export(WasmJitExecutableEntry::LimiterCorrection(0))
                    .expect("limiter correction export")
                    .to_owned();
                for (proposed, limited) in
                    [(0.0, 0.0), (1.0, 0.25), (1.0, 0.5), (1.0, 0.75), (1.0, 1.0)]
                {
                    harness
                        .store
                        .data_mut()
                        .context_mut()
                        .begin_stateful_evaluation();
                    harness.write_f64(FusedKernelHarness::VOLTAGES as usize, proposed);
                    harness.call_assignments();
                    harness.call_prelude();
                    for _ in 0..2 {
                        assert_eq!(harness.call(&value), 0);
                        assert_eq!(
                            harness.read_f64(FRAME_RESULT_OFFSET as usize),
                            limited * limited,
                            "{body}; postfix={postfix}"
                        );
                        assert_eq!(harness.call(&correction), 0);
                        let correction = harness.read_f64(FRAME_RESULT_OFFSET as usize);
                        assert_eq!(
                            correction,
                            2.0 * limited * (limited - proposed),
                            "{body}; postfix={postfix}"
                        );
                        assert_eq!(
                            2.0 * limited * proposed - limited * limited + correction,
                            limited * limited
                        );
                        assert_eq!(harness.call(&derivative), 0);
                        assert_eq!(
                            harness.read_f64(FRAME_RESULT_OFFSET as usize),
                            2.0 * limited,
                            "{body}; postfix={postfix}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn wasm_reactive_stamping_holds_external_derivative_coefficients_at_the_bias_point() {
        use super::abi::FRAME_RESULT_OFFSET;
        for (expression, capacitances) in [
            ("V(p,n)*ddt(V(p,n))", [3.0, -2.0]),
            ("V(c,n)*ddt(V(p,n))", [2.0, -4.0]),
            ("ddt(V(p,n))/V(c,n)", [0.5, -0.25]),
            ("((V(c,n)>0)?2.0:4.0)*ddt(V(p,n))", [2.0, 4.0]),
            ("(2.0+V(c,n))*ddt(V(p,n)*V(p,n))", [24.0, 8.0]),
        ] {
            let source = format!(
                "module weighted_derivative(p,n,c); inout p,n,c; electrical p,n,c; analog I(p,n)<+{expression}; endmodule"
            );
            let report = VerilogACompiler::default()
                .compile_runtime(&source, Some("weighted_derivative"))
                .unwrap();
            // Both production CFG lowering and the canonical MIR fallback must
            // preserve the held tangent. Execute both through an independent VM.
            for postfix in [false, true] {
                let mut harness = FusedKernelHarness::for_source_with_plan(
                    &source,
                    "weighted_derivative",
                    postfix,
                );
                harness.reset();
                for (bias, capacitance) in [[3.0, 0.0, 2.0], [-2.0, 0.0, -4.0]]
                    .into_iter()
                    .zip(capacitances)
                {
                    for (node, value) in bias.into_iter().enumerate() {
                        harness.write_f64(
                            FusedKernelHarness::VOLTAGES as usize + node * size_of::<f64>(),
                            value,
                        );
                    }
                    harness.call_assignments();
                    harness.call_prelude();
                    let entries = &report.model.stamp_programs[0].reactive_jacobians;
                    assert!(!entries.is_empty());
                    for (entry, derivative) in entries.iter().enumerate() {
                        let expected = match derivative.col_axis {
                            crate::codegen::ColumnAxis::Node(0) => capacitance,
                            crate::codegen::ColumnAxis::Node(1) => -capacitance,
                            _ => 0.0,
                        };
                        let export = harness
                            .executable
                            .export(WasmJitExecutableEntry::ReactiveJacobian { stamp: 0, entry })
                            .unwrap()
                            .to_owned();
                        assert_eq!(harness.call(&export), 0);
                        assert_eq!(
                            harness.read_f64(FRAME_RESULT_OFFSET as usize),
                            expected,
                            "{expression}; postfix={postfix}; entry={entry}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn wasm_discontinuity_preserves_degree_flags_and_resets_in_both_plans() {
        let source = "module hints(p,n); inout p,n; electrical p,n; parameter real degree=0; analog begin if(V(p,n)>0) $discontinuity(degree); if(V(p,n)>1) $discontinuity(-1); I(p,n)<+V(p,n); end endmodule";
        let report = VerilogACompiler::default()
            .compile_runtime(source, Some("hints"))
            .unwrap();
        let slot = report
            .model
            .variable_names
            .iter()
            .position(|name| name == "$discontinuity")
            .unwrap();
        for postfix in [false, true] {
            let mut harness = FusedKernelHarness::for_source_with_plan(source, "hints", postfix);
            harness.reset();
            for (degree, voltage, expected) in [
                (-1.0, 1.0, 2.0),
                (0.0, 1.0, 1.0),
                (2.0, 2.0, 3.0),
                (0.5, 1.0, 4.0),
                (f64::INFINITY, 1.0, 4.0),
                (-2.0, 0.0, 0.0),
                (3.0, 1.0, 1.0),
            ] {
                harness.write_f64(FusedKernelHarness::PARAMETERS as usize, degree);
                harness.write_f64(FusedKernelHarness::VOLTAGES as usize, voltage);
                harness.write_f64(FusedKernelHarness::VOLTAGES as usize + 8, 0.0);
                harness.call_assignments();
                assert_eq!(
                    harness.read_f64(FusedKernelHarness::VARIABLES as usize + slot * 8),
                    expected,
                    "postfix={postfix}, degree={degree}, voltage={voltage}"
                );
            }
        }
    }

    #[test]
    fn wasm_indirect_sources_export_analysis_constant_activation_in_both_plans() {
        use super::abi::FRAME_RESULT_OFFSET;

        let source = "module gated(p,n); inout p,n; electrical p,n;
            parameter integer en=1;
            analog if(en) V(p,n): V(p,n)==3;
            endmodule";
        for postfix in [false, true] {
            let mut harness = FusedKernelHarness::for_source_with_plan(source, "gated", postfix);
            harness.reset();
            let condition = harness
                .executable
                .export(WasmJitExecutableEntry::StaticCondition(0))
                .expect("indirect source exposes its structural activation condition")
                .to_owned();
            harness.write_f64(FusedKernelHarness::VOLTAGES as usize, 0.25);
            harness.write_f64(FusedKernelHarness::VOLTAGES as usize + 8, 0.0);
            for enabled in [1.0, 0.0, 1.0] {
                harness.write_f64(FusedKernelHarness::PARAMETERS as usize, enabled);
                harness.call_assignments();
                harness.call_prelude();
                assert_eq!(harness.call(&condition), 0);
                assert_eq!(
                    harness.read_f64(FRAME_RESULT_OFFSET as usize),
                    enabled,
                    "postfix={postfix}, enabled={enabled}"
                );
                if enabled != 0.0 {
                    let residual = harness.stamp_value_export(0);
                    assert_eq!(harness.call(&residual), 0);
                    assert_eq!(harness.read_f64(FRAME_RESULT_OFFSET as usize), -2.75);
                }
            }
        }
    }

    #[test]
    fn wasm_potential_sources_preserve_branch_identity_in_both_plans() {
        use super::abi::{
            FRAME_BRANCH_UNKNOWNS_LEN_OFFSET, FRAME_BRANCH_UNKNOWNS_PTR_OFFSET, FRAME_RESULT_OFFSET,
        };
        use crate::codegen::ColumnAxis;
        for (declarations, body, branches, derivatives) in [
            (
                "branch(p) a,b;",
                "V(a)<+2*I(a); V(b)<+3*I(b);",
                2,
                [[2.0, 0.0], [0.0, 3.0]],
            ),
            (
                "branch(p) a;",
                "V(a)<+2*I(a); V(p)<+3*I(p);",
                2,
                [[2.0, 0.0], [0.0, 3.0]],
            ),
            (
                "branch(p) a,b;",
                "V(a)<+2*I(a)+I(b); V(b)<+I(a)+3*I(b);",
                2,
                [[2.0, 1.0], [1.0, 3.0]],
            ),
            (
                "branch(p) a;",
                "V(a)<+2*I(a); V(a)<+3*I(a);",
                1,
                [[2.0, 0.0], [3.0, 0.0]],
            ),
            (
                "electrical g; ground g;",
                "V(p)<+2*I(p); V(g,p)<+3*I(g,p);",
                1,
                [[2.0, 0.0], [3.0, 0.0]],
            ),
            (
                "branch(p) a,b;",
                "V(a)<+I(a)+ddx(I(a)*I(b),I(b)); V(b)<+3*I(b)+ddx(I(a)*I(a),I(b));",
                2,
                [[2.0, 0.0], [0.0, 3.0]],
            ),
        ] {
            let source = format!(
                "module parallel(p); inout p; electrical p; {declarations} analog begin {body} end endmodule"
            );
            let report = VerilogACompiler::default()
                .compile_runtime(&source, Some("parallel"))
                .unwrap();
            assert_eq!(report.model.branch_sources.len(), branches);
            for postfix in [false, true] {
                let mut harness =
                    FusedKernelHarness::for_source_with_plan(&source, "parallel", postfix);
                harness.reset();
                let currents = FusedKernelHarness::VOLTAGES + 64;
                harness.poke_frame_u32(FRAME_BRANCH_UNKNOWNS_PTR_OFFSET, currents);
                harness.poke_frame_u32(FRAME_BRANCH_UNKNOWNS_LEN_OFFSET, branches as u32);
                harness.write_f64(currents as usize, 1.0);
                harness.write_f64(currents as usize + 8, 2.0);
                harness.call_assignments();
                harness.call_prelude();
                for (stamp, program) in report.model.stamp_programs.iter().enumerate() {
                    let export = harness.stamp_value_export(stamp);
                    assert_eq!(harness.call(&export), 0);
                    let value = harness.read_f64(FRAME_RESULT_OFFSET as usize);
                    let expected = derivatives[stamp][0] + 2.0 * derivatives[stamp][1];
                    assert!(
                        (value - expected).abs() < 1e-12,
                        "{body}; postfix={postfix}, stamp={stamp}: {value} != {expected}"
                    );
                    let mut actual = [0.0; 2];
                    for (entry, jacobian) in program.jacobian_programs.iter().enumerate() {
                        let export = harness.jacobian_export(stamp, entry);
                        assert_eq!(harness.call(&export), 0);
                        let value = harness.read_f64(FRAME_RESULT_OFFSET as usize);
                        match jacobian.col_axis {
                            ColumnAxis::Branch(column) => actual[column] = value,
                            ColumnAxis::Node(_) => assert_eq!(value, 0.0),
                        }
                    }
                    assert_eq!(actual, derivatives[stamp], "{body}; postfix={postfix}");
                }
            }
        }
    }

    #[test]
    fn wasm_hierarchy_port_currents_keep_instance_jacobians_in_both_plans() {
        use super::abi::{
            FRAME_INTERNAL_VOLTAGES_LEN_OFFSET, FRAME_INTERNAL_VOLTAGES_PTR_OFFSET,
            FRAME_RESULT_OFFSET,
        };
        use crate::codegen::{ColumnAxis, StampIndex};
        let source = "module child(p,q); inout p,q; electrical p,q; parameter real gain=1;
            analog begin if(gain>0) I(p)<+gain*V(p); I(q)<+3*I(<p>); end endmodule
            module top(p,q); inout p,q; electrical p,q;
            child #(.gain(1)) a(p,q); child #(.gain(2)) b(p,q); endmodule";
        let report = VerilogACompiler::default()
            .compile_runtime(source, Some("top"))
            .unwrap();
        assert_eq!(report.model.internal_state_nodes.len(), 2);
        for postfix in [false, true] {
            let mut harness = FusedKernelHarness::for_source_with_plan(source, "top", postfix);
            harness.reset();
            let internal = FusedKernelHarness::VOLTAGES + 64;
            harness.poke_frame_u32(FRAME_INTERNAL_VOLTAGES_PTR_OFFSET, internal);
            harness.poke_frame_u32(FRAME_INTERNAL_VOLTAGES_LEN_OFFSET, 2);
            harness.write_f64(FusedKernelHarness::VOLTAGES as usize, 1.0);
            harness.write_f64(FusedKernelHarness::VOLTAGES as usize + 8, 0.0);
            harness.write_f64(internal as usize, 0.0);
            harness.write_f64(internal as usize + 8, 0.0);
            // The host supplies resolved instance overrides, just as the
            // browser worker does after its parameter-default pass.
            assert_eq!(report.model.parameters.len(), 2);
            for (index, value) in [1.0, 2.0].into_iter().enumerate() {
                harness.write_f64(FusedKernelHarness::PARAMETERS as usize + 8 * index, value);
            }
            harness.call_assignments();
            harness.call_prelude();
            let mut matrix = [[0.0; 4]; 4];
            let mut residual = [0.0; 4];
            for (stamp, program) in report.model.stamp_programs.iter().enumerate() {
                let export = harness.stamp_value_export(stamp);
                assert_eq!(harness.call(&export), 0);
                let value = harness.read_f64(FRAME_RESULT_OFFSET as usize);
                let rows = program
                    .stamp_locations
                    .iter()
                    .filter_map(|location| {
                        let row = match location.row {
                            StampIndex::Terminal(row) => row,
                            StampIndex::Internal(row) => row + 2,
                            StampIndex::Ground => return None,
                            StampIndex::Branch(_) => panic!("unexpected potential source"),
                        };
                        Some((row, -location.sign))
                    })
                    .collect::<Vec<_>>();
                for &(row, sign) in &rows {
                    residual[row] += sign * value;
                }
                for (entry, derivative) in program.jacobian_programs.iter().enumerate() {
                    let ColumnAxis::Node(col) = derivative.col_axis else {
                        panic!("unexpected branch axis")
                    };
                    let export = harness.jacobian_export(stamp, entry);
                    assert_eq!(harness.call(&export), 0);
                    let value = harness.read_f64(FRAME_RESULT_OFFSET as usize);
                    let row = match derivative.row {
                        StampIndex::Terminal(row) => row,
                        StampIndex::Internal(row) => row + 2,
                        StampIndex::Ground => continue,
                        StampIndex::Branch(_) => panic!("unexpected potential source"),
                    };
                    matrix[row][col] += derivative.sign * value;
                }
            }
            for row in 0..4 {
                assert!((matrix[row][0] - residual[row]).abs() < 1e-12);
            }
            for pivot in (2..4).rev() {
                assert!((matrix[pivot][pivot] - 1.0).abs() < 1e-12, "{matrix:?}");
                for row in 0..pivot {
                    for col in 0..pivot {
                        matrix[row][col] -=
                            matrix[row][pivot] * matrix[pivot][col] / matrix[pivot][pivot];
                    }
                }
            }
            assert!((matrix[0][0] - 3.0).abs() < 1e-12, "{matrix:?}");
            assert!((matrix[1][0] - 9.0).abs() < 1e-12, "{matrix:?}");
        }
    }

    #[test]
    fn wasm_flow_probes_use_simultaneous_current_unknowns_in_both_plans() {
        use super::abi::{
            FRAME_INTERNAL_VOLTAGES_LEN_OFFSET, FRAME_INTERNAL_VOLTAGES_PTR_OFFSET,
            FRAME_RESULT_OFFSET,
        };
        use crate::codegen::ColumnAxis;
        let source = "module flow(p,n,q); inout p,n,q; electrical p,n,q; analog begin I(q,n)<+3*I(p,n); I(p,n)<+2*V(p,n)+0.1*I(p,n); end endmodule";
        let report = VerilogACompiler::default()
            .compile_runtime(source, None)
            .unwrap();
        assert_eq!(report.model.internal_state_nodes.len(), 1);
        for postfix in [false, true] {
            let mut harness = FusedKernelHarness::for_source_with_plan(source, "flow", postfix);
            harness.reset();
            let internal = FusedKernelHarness::VOLTAGES + 64;
            harness.poke_frame_u32(FRAME_INTERNAL_VOLTAGES_PTR_OFFSET, internal);
            harness.poke_frame_u32(FRAME_INTERNAL_VOLTAGES_LEN_OFFSET, 1);
            harness.write_f64(FusedKernelHarness::VOLTAGES as usize, 1.0);
            harness.write_f64(internal as usize, -2.0 / 0.9);
            harness.call_assignments();
            harness.call_prelude();
            for (stamp, expected) in [6.0 / 0.9, 2.0 / 0.9, -2.0 / 0.9, -2.0 / 0.9]
                .into_iter()
                .enumerate()
            {
                let export = harness.stamp_value_export(stamp);
                assert_eq!(harness.call(&export), 0);
                assert!((harness.read_f64(FRAME_RESULT_OFFSET as usize) - expected).abs() < 1e-12);
                for (entry, derivative) in report.model.stamp_programs[stamp]
                    .jacobian_programs
                    .iter()
                    .enumerate()
                {
                    let expected = match (stamp, derivative.col_axis) {
                        (0, ColumnAxis::Node(3)) => -3.0,
                        (1, ColumnAxis::Node(0)) => 2.0,
                        (1, ColumnAxis::Node(1)) => -2.0,
                        (1, ColumnAxis::Node(3)) => -0.1,
                        (2 | 3, ColumnAxis::Node(3)) => 1.0,
                        _ => panic!("unexpected current-probe derivative"),
                    };
                    let export = harness.jacobian_export(stamp, entry);
                    assert_eq!(harness.call(&export), 0);
                    assert_eq!(harness.read_f64(FRAME_RESULT_OFFSET as usize), expected);
                }
            }
        }
    }

    #[test]
    fn wasm_implicit_integrator_uses_its_solver_unknown_in_both_plans() {
        use super::abi::{
            FRAME_INTERNAL_VOLTAGES_LEN_OFFSET, FRAME_INTERNAL_VOLTAGES_PTR_OFFSET,
            FRAME_RESULT_OFFSET,
        };
        use crate::codegen::ColumnAxis;
        let source = "module implicit_integrator(p,n); inout p,n; electrical p,n; parameter integer enabled=1; analog I(p)<+(enabled ? idt(V(p,n)) : 0.25); endmodule";
        let report = VerilogACompiler::default()
            .compile_runtime(source, Some("implicit_integrator"))
            .unwrap();
        for postfix in [false, true] {
            let mut harness =
                FusedKernelHarness::for_source_with_plan(source, "implicit_integrator", postfix);
            for enabled in [1.0, 0.0, 1.0] {
                harness.reset();
                let internal = FusedKernelHarness::VOLTAGES + 64;
                harness.poke_frame_u32(FRAME_INTERNAL_VOLTAGES_PTR_OFFSET, internal);
                harness.poke_frame_u32(FRAME_INTERNAL_VOLTAGES_LEN_OFFSET, 1);
                harness.write_f64(FusedKernelHarness::PARAMETERS as usize, enabled);
                harness.write_f64(FusedKernelHarness::VOLTAGES as usize, 2.0);
                harness.write_f64(FusedKernelHarness::VOLTAGES as usize + 8, 1.5);
                harness.write_f64(internal as usize, 9.0);
                harness.call_assignments();
                harness.call_prelude();
                let on = enabled > 0.0;
                for (stamp, expected) in [if on { 9.0 } else { 0.25 }, if on { -0.5 } else { 9.0 }]
                    .into_iter()
                    .enumerate()
                {
                    let export = harness.stamp_value_export(stamp);
                    assert_eq!(harness.call(&export), 0);
                    assert_eq!(
                        harness.read_f64(FRAME_RESULT_OFFSET as usize),
                        expected,
                        "postfix={postfix}, enabled={enabled}, stamp={stamp}"
                    );
                    for (entry, derivative) in report.model.stamp_programs[stamp]
                        .jacobian_programs
                        .iter()
                        .enumerate()
                    {
                        let expected = match (stamp, derivative.col_axis, on) {
                            (0, ColumnAxis::Node(2), true)
                            | (1, ColumnAxis::Node(1), true)
                            | (1, ColumnAxis::Node(2), false) => 1.0,
                            (1, ColumnAxis::Node(0), true) => -1.0,
                            _ => 0.0,
                        };
                        let export = harness.jacobian_export(stamp, entry);
                        assert_eq!(harness.call(&export), 0);
                        assert_eq!(
                            harness.read_f64(FRAME_RESULT_OFFSET as usize),
                            expected,
                            "postfix={postfix}, enabled={enabled}, stamp={stamp}, entry={entry}"
                        );
                    }
                }
                for (entry, derivative) in report.model.stamp_programs[1]
                    .reactive_jacobians
                    .iter()
                    .enumerate()
                {
                    let expected = if on && matches!(derivative.col_axis, ColumnAxis::Node(2)) {
                        1.0
                    } else {
                        0.0
                    };
                    let export = harness
                        .executable
                        .export(WasmJitExecutableEntry::ReactiveJacobian { stamp: 1, entry })
                        .unwrap()
                        .to_owned();
                    assert_eq!(harness.call(&export), 0);
                    assert_eq!(harness.read_f64(FRAME_RESULT_OFFSET as usize), expected);
                }
            }
        }
    }

    #[test]
    fn wasm_homogeneous_math_preserves_extreme_scales() {
        use super::abi::FRAME_RESULT_OFFSET;
        for op in ["hypot", "atan2"] {
            for derivative in 0..3 {
                let expression = format!("{op}(V(p),V(q))");
                let expression = match derivative {
                    1 => format!("ddx({expression},V(p))"),
                    2 => format!("ddx({expression},V(q))"),
                    _ => expression,
                };
                let source = format!(
                    "module planar(p,q); inout p,q; electrical p,q; analog I(p)<+{expression}; endmodule"
                );
                for postfix in [false, true] {
                    let mut harness =
                        FusedKernelHarness::for_source_with_plan(&source, "planar", postfix);
                    let mut entries = vec![(harness.stamp_value_export(0), 0)];
                    {
                        let report = VerilogACompiler::default()
                            .compile_runtime(&source, Some("planar"))
                            .unwrap();
                        for (index, entry) in report.model.stamp_programs[0]
                            .jacobian_programs
                            .iter()
                            .enumerate()
                        {
                            let expected_index = match entry.col_axis {
                                crate::codegen::ColumnAxis::Node(0) => 1,
                                crate::codegen::ColumnAxis::Node(1) => 2,
                                _ => panic!("unexpected planar column"),
                            };
                            entries.push((harness.jacobian_export(0, index), expected_index));
                        }
                    }
                    for scale in [1e-200, 1e-100, 1.0, 1e100, 1e200, 8e307] {
                        for (a, b) in [(-1.0_f64, 2.0_f64), (0.0, -2.0), (1.0, -1.0), (1.0, 1.0)] {
                            let (p, q) = (a * scale, b * scale);
                            let expected = if op == "hypot" {
                                let r = a.hypot(b);
                                [p.hypot(q), a / r, b / r]
                            } else {
                                let d = a * a + b * b;
                                [p.atan2(q), (b / d) / scale, (-a / d) / scale]
                            };
                            let d = a * a + b * b;
                            let hessian = if op == "hypot" {
                                let r = a.hypot(b);
                                let factor = (1.0 / (r * r * r)) / scale;
                                [b * b * factor, -a * b * factor, a * a * factor]
                            } else {
                                let factor = ((1.0 / (d * d)) / scale) / scale;
                                [
                                    -2.0 * a * b * factor,
                                    (a * a - b * b) * factor,
                                    2.0 * a * b * factor,
                                ]
                            };
                            let outputs = match derivative {
                                1 => [expected[1], hessian[0], hessian[1]],
                                2 => [expected[2], hessian[1], hessian[2]],
                                _ => expected,
                            };
                            harness.reset();
                            harness.write_f64(FusedKernelHarness::VOLTAGES as usize, p);
                            harness.write_f64(FusedKernelHarness::VOLTAGES as usize + 8, q);
                            harness.call_assignments();
                            harness.call_prelude();
                            for (entry, expected_index) in &entries {
                                if derivative != 0
                                    && *expected_index != 0
                                    && !(1e-100..=1e100).contains(&scale)
                                {
                                    continue;
                                }
                                let expected = outputs[*expected_index];
                                assert_eq!(harness.call(entry), 0, "{expression} at {p},{q}");
                                let actual = harness.read_f64(FRAME_RESULT_OFFSET as usize);
                                if expected == 0.0 {
                                    assert_eq!(actual, expected);
                                } else {
                                    assert!(
                                        (actual / expected - 1.0).abs() < 1e-12,
                                        "{expression}, postfix={postfix}, entry={entry} at {p:e},{q:e}: expected {expected:e}, got {actual:e}"
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn wasm_quotient_range_large_gain() {
        assert_large_gain_derivative("ddx(1e308/(1e308*V(p)),V(p))", 0.01, 1.0, -1e4, 2e6);
    }

    #[test]
    fn wasm_quotient_range_large_canceling_terms() {
        assert_large_gain_derivative(
            "ddx(1.6e308*V(p)/(3*V(p)-1),V(p))",
            1.0,
            1.0,
            -4e307,
            1.2e308,
        );
    }

    #[test]
    fn wasm_quotient_range_tiny_proportional_cancellation() {
        assert_large_gain_derivative("ddx(V(p)/V(p),V(p))", 1e-309, 1.0, 0.0, 0.0);
        for expression in ["ddx((2*V(p))/(3*V(p)),V(p))", "ddx((5*V(p))/(7*V(p)),V(p))"] {
            for bias in [-1e-309, 1e-309, 1e-100, 1.0] {
                assert_large_gain_derivative(expression, bias, 1.0, 0.0, 0.0);
            }
        }
    }

    #[test]
    fn wasm_quotient_range_rescued_tiny_tangent() {
        assert_large_gain_derivative(
            "ddx(1e308/(1e200+1e-200*V(p)),V(p))",
            0.0,
            1.0,
            -1e-292,
            0.0,
        );
    }

    #[test]
    fn wasm_large_gain_hypot_keeps_finite_curvature() {
        for gain in [1e-200_f64, 1.0, 1e200, -1e200] {
            let slope = gain.abs() / std::f64::consts::SQRT_2;
            assert_large_gain_derivative(
                &format!("ddx(hypot({gain:e}*V(p),{gain:e}*V(q)),V(p))"),
                1.0,
                1.0,
                slope,
                slope / 2.0,
            );
        }
        for (a, b) in [(1e150_f64, 1e-150_f64), (1e-150, 1e150)] {
            let base = format!("hypot({a:e}*V(p),{b:e}*V(q))");
            for (axis, value, curvature) in [
                (
                    "p",
                    a / std::f64::consts::SQRT_2,
                    (a * a) / (2.0 * std::f64::consts::SQRT_2),
                ),
                (
                    "q",
                    b / std::f64::consts::SQRT_2,
                    -(a * b) / (2.0 * std::f64::consts::SQRT_2),
                ),
            ] {
                assert_large_gain_derivative(
                    &format!("ddx({base},V({axis}))"),
                    1.0 / a,
                    1.0 / b,
                    value,
                    curvature,
                );
            }
        }
    }

    fn assert_large_gain_derivative(expression: &str, p: f64, q: f64, value: f64, slope: f64) {
        use super::abi::FRAME_RESULT_OFFSET;
        let source = format!(
            "module gain(p,q); inout p,q; electrical p,q; analog I(p)<+{expression}; endmodule"
        );
        let report = VerilogACompiler::default()
            .compile_runtime(&source, None)
            .unwrap();
        let column = report.model.stamp_programs[0]
            .jacobian_programs
            .iter()
            .position(|entry| matches!(entry.col_axis, crate::codegen::ColumnAxis::Node(0)))
            .unwrap();
        for postfix in [false, true] {
            let mut harness = FusedKernelHarness::for_source_with_plan(&source, "gain", postfix);
            harness.reset();
            harness.write_f64(FusedKernelHarness::VOLTAGES as usize, p);
            harness.write_f64(FusedKernelHarness::VOLTAGES as usize + 8, q);
            harness.call_assignments();
            harness.call_prelude();
            for (entry, expected) in [
                (harness.stamp_value_export(0), value),
                (harness.jacobian_export(0, column), slope),
            ] {
                assert_eq!(
                    harness.call(&entry),
                    0,
                    "{expression}, postfix={postfix}, {entry}"
                );
                let actual = harness.read_f64(FRAME_RESULT_OFFSET as usize);
                if expected == 0.0 {
                    assert_eq!(actual, expected, "{expression}, postfix={postfix}, {entry}");
                } else {
                    assert!(
                        (actual / expected - 1.0).abs() < 1e-12,
                        "{expression}, postfix={postfix}, {entry}: expected {expected:e}, got {actual:e}"
                    );
                }
            }
        }
    }

    #[test]
    fn wasm_quotient_derivatives_preserve_representable_results() {
        use super::abi::FRAME_RESULT_OFFSET;
        for derivative in [false, true] {
            let expression = if derivative {
                "ddx(1e308/V(p),V(p))"
            } else {
                "1e308/V(p)"
            };
            let source = format!(
                "module quotient(p); inout p; electrical p; analog I(p)<+{expression}; endmodule"
            );
            for postfix in [false, true] {
                let mut harness =
                    FusedKernelHarness::for_source_with_plan(&source, "quotient", postfix);
                for p in [-1e200_f64, -1e100, 1e100, 1e200] {
                    let q = 1e308 / p;
                    let slope = -q / p;
                    let curvature = (-2.0 * slope) / p;
                    let outputs = if derivative {
                        [slope, curvature]
                    } else {
                        [q, slope]
                    };
                    harness.reset();
                    harness.write_f64(FusedKernelHarness::VOLTAGES as usize, p);
                    harness.call_assignments();
                    harness.call_prelude();
                    for (entry, expected) in [
                        (harness.stamp_value_export(0), outputs[0]),
                        (harness.jacobian_export(0, 0), outputs[1]),
                    ] {
                        assert_eq!(harness.call(&entry), 0);
                        let actual = harness.read_f64(FRAME_RESULT_OFFSET as usize);
                        assert!(
                            (actual / expected - 1.0).abs() < 1e-12,
                            "{expression}, postfix={postfix}, {entry}, p={p:e}: expected {expected:e}, got {actual:e}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn wasm_quotient_mixed_partials_preserve_representable_results() {
        use super::abi::FRAME_RESULT_OFFSET;
        for derivative in 0..3 {
            let expression = "1e200*V(p)*V(p)/(V(q)*V(q))";
            let expression = match derivative {
                1 => format!("ddx({expression},V(p))"),
                2 => format!("ddx({expression},V(q))"),
                _ => expression.into(),
            };
            let source = format!(
                "module quotient(p,q); inout p,q; electrical p,q; analog I(p)<+{expression}; endmodule"
            );
            let report = VerilogACompiler::default()
                .compile_runtime(&source, None)
                .unwrap();
            for postfix in [false, true] {
                let mut harness =
                    FusedKernelHarness::for_source_with_plan(&source, "quotient", postfix);
                let mut entries = vec![(harness.stamp_value_export(0), 0)];
                for (index, entry) in report.model.stamp_programs[0]
                    .jacobian_programs
                    .iter()
                    .enumerate()
                {
                    let expected_index = match entry.col_axis {
                        crate::codegen::ColumnAxis::Node(0) => 1,
                        crate::codegen::ColumnAxis::Node(1) => 2,
                        _ => panic!("unexpected quotient column"),
                    };
                    entries.push((harness.jacobian_export(0, index), expected_index));
                }
                for p in [-2.0, 0.0, 0.75] {
                    for q in [-1e150, -1e100, 1e50, 1e100, 1e150] {
                        let scale = (1e200 / q) / q;
                        let value = scale * p * p;
                        let dp = 2.0 * scale * p;
                        let dq = (-2.0 * value) / q;
                        let outputs = match derivative {
                            1 => [dp, 2.0 * scale, (-2.0 * dp) / q],
                            2 => [dq, (-2.0 * dp) / q, (-3.0 * dq) / q],
                            _ => [value, dp, dq],
                        };
                        harness.reset();
                        harness.write_f64(FusedKernelHarness::VOLTAGES as usize, p);
                        harness.write_f64(FusedKernelHarness::VOLTAGES as usize + 8, q);
                        harness.call_assignments();
                        harness.call_prelude();
                        for (entry, output) in &entries {
                            assert_eq!(harness.call(entry), 0);
                            let actual = harness.read_f64(FRAME_RESULT_OFFSET as usize);
                            let expected = outputs[*output];
                            if expected == 0.0 {
                                assert_eq!(actual, expected);
                            } else {
                                assert!(
                                    (actual / expected - 1.0).abs() < 1e-12,
                                    "{expression}, postfix={postfix}, {entry}, p={p:e}, q={q:e}: expected {expected:e}, got {actual:e}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn wasm_hypot_shared_operand_preserves_range() {
        use super::abi::FRAME_RESULT_OFFSET;
        let source =
            "module shared(p); inout p; electrical p; analog I(p)<+hypot(V(p),V(p)); endmodule";
        for postfix in [false, true] {
            let mut harness = FusedKernelHarness::for_source_with_plan(source, "shared", postfix);
            for p in [-1e308_f64, 1e308, -1e-200, 1e-200] {
                harness.reset();
                harness.write_f64(FusedKernelHarness::VOLTAGES as usize, p);
                harness.call_assignments();
                harness.call_prelude();
                for (entry, expected) in [
                    (harness.stamp_value_export(0), p.hypot(p)),
                    (
                        harness.jacobian_export(0, 0),
                        p.signum() * std::f64::consts::SQRT_2,
                    ),
                ] {
                    assert_eq!(harness.call(&entry), 0);
                    let actual = harness.read_f64(FRAME_RESULT_OFFSET as usize);
                    assert!(
                        (actual / expected - 1.0).abs() < 1e-12,
                        "postfix={postfix}, {entry}, p={p:e}: expected {expected:e}, got {actual:e}"
                    );
                }
            }
        }
    }

    #[test]
    fn wasm_extrema_select_numeric_values_and_derivatives() {
        use super::abi::FRAME_RESULT_OFFSET;
        for (expression, p) in [
            ("max(V(p),sqrt(V(q)))", 1.0),
            ("max(sqrt(V(q)),V(p))", 1.0),
            ("min(V(p),sqrt(V(q)))", -1.0),
            ("min(sqrt(V(q)),V(p))", -1.0),
        ] {
            for (expression, expected) in [
                (expression.to_string(), p),
                (format!("ddx({expression},V(p))"), 1.0),
                (format!("ddx({expression},V(q))"), 0.0),
            ] {
                let source = format!(
                    "module extrema(p,q); inout p,q; electrical p,q; analog I(p)<+{expression}; endmodule"
                );
                let report = VerilogACompiler::default()
                    .compile_runtime(&source, Some("extrema"))
                    .unwrap();
                for postfix in [false, true] {
                    let mut harness =
                        FusedKernelHarness::for_source_with_plan(&source, "extrema", postfix);
                    let value = harness.stamp_value_export(0);
                    let mut entries = vec![(value, expected)];
                    if !expression.starts_with("ddx") {
                        for (index, entry) in report.model.stamp_programs[0]
                            .jacobian_programs
                            .iter()
                            .enumerate()
                        {
                            let expected = match entry.col_axis {
                                crate::codegen::ColumnAxis::Node(0) => 1.0,
                                crate::codegen::ColumnAxis::Node(1) => 0.0,
                                _ => panic!("unexpected extrema column"),
                            };
                            entries.push((harness.jacobian_export(0, index), expected));
                        }
                    }
                    for q in [-1.0, 0.0] {
                        harness.reset();
                        harness.write_f64(FusedKernelHarness::VOLTAGES as usize, p);
                        harness.write_f64(FusedKernelHarness::VOLTAGES as usize + 8, q);
                        harness.call_assignments();
                        harness.call_prelude();
                        for (entry, expected) in &entries {
                            assert_eq!(
                                harness.call(entry),
                                0,
                                "{expression}, q={q}, postfix={postfix}, entry={entry}"
                            );
                            assert_eq!(
                                harness.read_f64(FRAME_RESULT_OFFSET as usize),
                                *expected,
                                "{expression}, q={q}, postfix={postfix}, entry={entry}"
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn wasm_signed_zero_factors_preserve_values_and_jacobians() {
        use super::abi::FRAME_RESULT_OFFSET;
        for (zero, values, apply) in [
            (
                "0.0*V(q)",
                [-2.0, 2.0],
                (|x: f64| 0.0 * x) as fn(f64) -> f64,
            ),
            ("0.0/V(q)", [-2.0, 2.0], |x| 0.0 / x),
            ("0.0+V(q)", [-0.0, 0.0], |x| 0.0 + x),
            ("V(q)-(-0.0)", [-0.0, 0.0], |x| x - (-0.0)),
            ("pow(0.0*V(q),0.5)", [-2.0, 2.0], |x| (0.0 * x).powf(0.5)),
            ("pow(sqrt(0.0*V(q)),0.5)", [-2.0, 2.0], |x| {
                (0.0 * x).sqrt().powf(0.5)
            }),
        ] {
            let source = format!(
                "module signed_zero(p,q); inout p,q; electrical p,q; analog I(p)<+V(p)*atan2({zero},-1.0); endmodule"
            );
            for postfix in [false, true] {
                let mut harness =
                    FusedKernelHarness::for_source_with_plan(&source, "signed_zero", postfix);
                let value = harness.stamp_value_export(0);
                let jacobian = harness.jacobian_export(0, 0);
                for q in values {
                    harness.reset();
                    harness.write_f64(FusedKernelHarness::VOLTAGES as usize, 1.0);
                    harness.write_f64(FusedKernelHarness::VOLTAGES as usize + 8, q);
                    harness.call_assignments();
                    harness.call_prelude();
                    for export in [&value, &jacobian] {
                        assert_eq!(harness.call(export), 0);
                        assert_eq!(
                            harness.read_f64(FRAME_RESULT_OFFSET as usize),
                            apply(q).atan2(-1.0),
                            "{zero} at {q:?}, postfix={postfix}, {export}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn wasm_ddx_preserves_domain_failures_through_predicates_and_nested_derivatives() {
        use super::abi::FRAME_RESULT_OFFSET;
        for (expression, valid) in [
            ("ddx(V(p)%V(q),V(p))", 1.0),
            ("ddx(ddx(V(p)%V(q),V(p)),V(p))", 0.0),
            ("(ddx(V(p)%V(q),V(p))>0 ? 1 : 0)", 1.0),
            ("ddx(a/b,V(p))", 0.0),
            ("min(ddx(0.0/V(q),V(p))+V(p),V(p))", 5.0),
            ("max(ddx(0.0/V(q),V(p))+V(p),V(p))", 5.0),
        ] {
            let source = format!(
                "module derivative_domain(p,q,n); inout p,q,n; electrical p,q,n;
                integer a,b; analog begin a=V(p); b=V(q);
                I(p,n)<+(V(q)<0 ? 3 : {expression}); end endmodule"
            );
            for postfix in [false, true] {
                let mut harness =
                    FusedKernelHarness::for_source_with_plan(&source, "derivative_domain", postfix);
                let value = harness.stamp_value_export(0);
                for (denominator, expected) in [(-1.0, Some(3.0)), (0.0, None), (2.0, Some(valid))]
                {
                    harness.reset();
                    harness.write_f64(FusedKernelHarness::VOLTAGES as usize, 5.0);
                    harness.write_f64(FusedKernelHarness::VOLTAGES as usize + 8, denominator);
                    harness.write_f64(FusedKernelHarness::VOLTAGES as usize + 16, 0.0);
                    let exports = [
                        harness.artifact.assignment_export().map(str::to_owned),
                        harness.artifact.prelude_export().map(str::to_owned),
                        Some(value.clone()),
                    ];
                    let mut status = 0;
                    for export in exports.into_iter().flatten() {
                        status = harness.call(&export);
                        if status != 0 {
                            break;
                        }
                    }
                    if let Some(expected) = expected {
                        assert_eq!(status, 0, "{expression}, postfix={postfix}");
                        assert_eq!(harness.read_f64(FRAME_RESULT_OFFSET as usize), expected);
                    } else {
                        assert_ne!(
                            status, 0,
                            "{expression} hid division by zero, postfix={postfix}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn integer_wasm_arithmetic_preserves_values_and_zero_tangents() {
        use super::abi::FRAME_RESULT_OFFSET;
        for (operator, left, right, expected) in [
            ("/", 5.0, 2.0, 2.0),
            ("/", -5.0, 2.0, -2.0),
            ("+", 2147483647.0, 1.0, -2147483648.0),
            ("-", -2147483648.0, 1.0, 2147483647.0),
            ("*", 2147483647.0, 2.0, -2.0),
            ("**", 2.0, 31.0, -2147483648.0),
            ("**", 2.0, -1.0, 0.0),
            ("**", -1.0, -3.0, -1.0),
            ("%", -5.0, 2.0, -1.0),
        ] {
            let source = format!(
                "module typed_wasm(p,q,n); inout p,q,n; electrical p,q,n; integer a,b; analog begin a=V(p,n); b=V(q,n); I(p,n)<+(a {operator} b)+0.25*V(p,n); end endmodule"
            );
            for postfix in [false, true] {
                let mut harness =
                    FusedKernelHarness::for_source_with_plan(&source, "typed_wasm", postfix);
                let value = harness.stamp_value_export(0);
                let jacobian = harness.jacobian_export(0, 0);
                harness.reset();
                harness.write_f64(FusedKernelHarness::VOLTAGES as usize, left);
                harness.write_f64(FusedKernelHarness::VOLTAGES as usize + 8, right);
                harness.write_f64(FusedKernelHarness::VOLTAGES as usize + 16, 0.0);
                harness.call_assignments();
                harness.call_prelude();
                for (export, expected) in [(&value, expected + 0.25 * left), (&jacobian, 0.25)] {
                    assert_eq!(harness.call(export), 0);
                    assert_eq!(
                        harness.read_f64(FRAME_RESULT_OFFSET as usize),
                        expected,
                        "{left} {operator} {right}"
                    );
                }
            }
        }
    }

    #[test]
    fn integer_wasm_assignments_round_before_differentiation() {
        use super::abi::FRAME_RESULT_OFFSET;
        let source = "module integer_wasm(p,n); inout p,n; electrical p,n; integer q; analog begin q=V(p,n); I(p,n)<+q+0.25*V(p,n); end endmodule";
        let mut harness = FusedKernelHarness::for_source(source, "integer_wasm");
        let value = harness.stamp_value_export(0);
        let jacobian = harness.jacobian_export(0, 0);
        harness.reset();
        for v in [-2.5_f64, -0.5, 0.49, 0.5, 1.25, 2.5] {
            harness.write_f64(FusedKernelHarness::VOLTAGES as usize, v);
            harness.call_assignments();
            harness.call_prelude();
            for (export, expected) in [(&value, v.round() + 0.25 * v), (&jacobian, 0.25)] {
                assert_eq!(harness.call(export), 0);
                assert_eq!(
                    harness.read_f64(FRAME_RESULT_OFFSET as usize),
                    expected,
                    "at {v}"
                );
            }
        }
    }

    #[test]
    fn conditional_integer_wasm_assignments_skip_invalid_untaken_values() {
        use super::abi::FRAME_RESULT_OFFSET;
        for declarations_and_body in [
            "integer q; analog begin q=0; if(V(p,n)>0.0) begin if(V(p,n)<10.0) q=V(p,n); else q=3; end I(p,n)<+q; end",
            "integer q[0:0],idx; analog begin idx=0; q[idx]=0; if(V(p,n)>0.0) begin if(V(p,n)<10.0) q[idx]=V(p,n); else q[idx]=3; end I(p,n)<+q[idx]; end",
        ] {
            let source = format!(
                "module guarded_wasm(p,n); inout p,n; electrical p,n; {declarations_and_body} endmodule"
            );
            let mut harness = FusedKernelHarness::for_source(&source, "guarded_wasm");
            let value = harness.stamp_value_export(0);
            harness.reset();
            for (v, expected) in [(1.5, 2.0), (3e9, 3.0), (-3e9, 0.0), (2.5, 3.0)] {
                harness.write_f64(FusedKernelHarness::VOLTAGES as usize, v);
                harness.call_assignments();
                harness.call_prelude();
                assert_eq!(harness.call(&value), 0, "at {v}");
                assert_eq!(
                    harness.read_f64(FRAME_RESULT_OFFSET as usize),
                    expected,
                    "at {v}"
                );
            }
        }
    }

    #[test]
    fn real_modulo_wasm_kernels_preserve_values_and_derivatives() {
        use super::abi::FRAME_RESULT_OFFSET;
        for (body, order) in [
            (
                "I(p,n)<+(10.0+V(p,n)*V(p,n)*V(p,n))%(2.0+V(p,n)*V(p,n)*V(p,n));",
                0,
            ),
            (
                "r=(10.0+V(p,n)*V(p,n)*V(p,n))%(2.0+V(p,n)*V(p,n)*V(p,n)); I(p,n)<+ddx(r,V(p,n));",
                1,
            ),
            (
                "I(p,n)<+ddx(ddx((10.0+V(p,n)*V(p,n)*V(p,n))%(2.0+V(p,n)*V(p,n)*V(p,n)),V(p,n)),V(p,n));",
                2,
            ),
        ] {
            let source = format!(
                "module remainder_wasm(p,n); inout p,n; electrical p,n; real r; analog begin {body} end endmodule"
            );
            let mut harness = FusedKernelHarness::for_source(&source, "remainder_wasm");
            let value_export = harness.stamp_value_export(0);
            let jacobian_export = harness.jacobian_export(0, 0);
            harness.reset();
            for v in [-3.0_f64, -0.75, 0.5, 1.25] {
                let a = 10.0 + v * v * v;
                let b = 2.0 + v * v * v;
                let scale = 1.0 - (a / b).trunc();
                let derivatives = [a % b, scale * 3.0 * v * v, scale * 6.0 * v, scale * 6.0];
                harness.write_f64(FusedKernelHarness::VOLTAGES as usize, v);
                harness.call_assignments();
                harness.call_prelude();
                for (export, expected) in [
                    (&value_export, derivatives[order]),
                    (&jacobian_export, derivatives[order + 1]),
                ] {
                    assert_eq!(harness.call(export), 0);
                    let actual = harness.read_f64(FRAME_RESULT_OFFSET as usize);
                    assert!(
                        (actual - expected).abs() < 1e-10,
                        "{body} at {v}: {actual} != {expected}"
                    );
                }
            }
        }
    }

    #[test]
    fn nested_ddx_wasm_kernels_preserve_higher_order_jacobians() {
        use super::abi::FRAME_RESULT_OFFSET;
        for body in [
            "analog I(p,n)<+ddx(ddx(V(p,n)*V(p,n)*V(p,n),V(p,n)),V(p,n));",
            "real x,y; analog begin x=V(p,n)*V(p,n)*V(p,n); y=ddx(x,V(p,n)); I(p,n)<+ddx(y,V(p,n)); end",
            "real q[2:2]; integer idx; analog begin idx=2; q[idx]=V(p,n)*V(p,n)*V(p,n); I(p,n)<+ddx(ddx(q[idx],V(p,n)),V(p,n)); end",
        ] {
            let source =
                format!("module nested_wasm(p,n); inout p,n; electrical p,n; {body} endmodule");
            let mut harness = FusedKernelHarness::for_source(&source, "nested_wasm");
            let value_export = harness.stamp_value_export(0);
            let jacobian_export = harness.jacobian_export(0, 0);
            harness.reset();
            for v in [-0.75, 0.0, 1.25] {
                harness.write_f64(FusedKernelHarness::VOLTAGES as usize, v);
                harness.call_assignments();
                harness.call_prelude();
                assert_eq!(harness.call(&value_export), 0);
                let value = harness.read_f64(FRAME_RESULT_OFFSET as usize);
                assert!((value - 6.0 * v).abs() < 1e-10, "{body}: {value}");
                assert_eq!(harness.call(&jacobian_export), 0);
                let value = harness.read_f64(FRAME_RESULT_OFFSET as usize);
                assert!((value - 6.0).abs() < 1e-10, "{body}: {value}");
            }
        }
    }

    /// The shape `ekv3_302.00` has: a scratch variable a contribution reads
    /// and a later statement reassigns.
    const REUSED_AFTER_READ: &str = r#"
`include "disciplines.vams"
module wasm_reuse(p, n);
  inout p, n;
  electrical p, n, m;
  real tmp;
  real reported;
  analog begin
    tmp = 2.5e-3;
    I(p, n) <+ V(p, n) * tmp;
    tmp = 1.25;
    reported = tmp;
    I(m, n) <+ reported;
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
    /// so it cannot agree with a defect two routes share.
    ///
    /// The second contribution is the guard: it reads `tmp` through `reported`
    /// after the overwrite, so a module that had simply dropped the later write
    /// would fail it, and the first assertion would otherwise be satisfied by a
    /// module that never wrote `tmp` twice at all. Both readings come out of
    /// emitted code rather than out of the variable array, which is what lets
    /// the pin stand on a plan that publishes no variable.
    #[test]
    fn a_wasm_contribution_reads_the_definition_reaching_it() {
        use std::mem::size_of;

        use super::abi::FRAME_RESULT_OFFSET;

        const BIAS: f64 = 4.0;
        const REACHING: f64 = 2.5e-3;
        const OVERWRITTEN: f64 = 1.25;

        let mut harness = FusedKernelHarness::for_source(REUSED_AFTER_READ, "wasm_reuse");
        let reaching_export = harness.stamp_value_export(0);
        let overwritten_export = harness.stamp_value_export(1);

        harness.reset();
        harness.write_f64(FusedKernelHarness::VOLTAGES as usize, BIAS);
        harness.write_f64(
            FusedKernelHarness::VOLTAGES as usize + size_of::<f64>(),
            0.0,
        );
        harness.call_assignments();
        harness.call_prelude();
        assert_eq!(harness.call(&reaching_export), 0);

        assert_eq!(
            harness.read_f64(FRAME_RESULT_OFFSET as usize),
            BIAS * REACHING,
            "the contribution reads tmp at its reaching definition"
        );

        assert_eq!(harness.call(&overwritten_export), 0);
        assert_eq!(
            harness.read_f64(FRAME_RESULT_OFFSET as usize),
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
        let stamp_count = harness.stamp_count();
        assert!(stamp_count >= 3, "the model must exercise several stamps");

        // Per-entry path: assignment kernel, then each stamp value export.
        harness.reset();
        harness.call_assignments();
        harness.call_prelude();
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

    /// The power rule's exponent term at a zero base, under wasmi.
    ///
    /// The WebAssembly route lowers the same canonical plan as x64 and AArch64
    /// (`compile_model_value_module` → `build_default_model_plan`), so the
    /// derivative pass's unguarded `a^b · ln(a)` at `a = 0` reached it too.
    /// The fixture is the x64 one
    /// (`native::x64::tests::power_rule_exponent_term_is_finite_at_a_zero_base`),
    /// resistive form: the exponent reaches `V(t)` only through a merge whose
    /// taken arm does not depend on it, so the derivative is analytically 0.
    #[test]
    fn power_rule_exponent_term_is_finite_at_a_zero_base_under_wasm() {
        use std::mem::size_of;

        use super::abi::FRAME_RESULT_OFFSET;

        const SOURCE: &str = r#"
module wasm_pow_exponent_zero_base(p, n, t);
  inout p, n, t;
  electrical p, n, t;
  parameter real vsat = 0.04;
  parameter real m = 4.0;
  parameter real tm = 0.0;
  parameter integer sh = 0;
  real temp, mt;
  analog begin
    if (sh != 0)
      temp = $temperature + V(t);
    else
      temp = $temperature;
    mt = m * (1.0 + tm * (temp - 300.15));
    I(p, n) <+ 1.0e-3 * pow(V(p, n) / vsat, mt);
  end
endmodule
"#;
        let report = VerilogACompiler::new(CompilerOptions::default())
            .compile_runtime(SOURCE, Some("wasm_pow_exponent_zero_base"))
            .expect("compile fixture");
        let entry = report.model.stamp_programs[0]
            .jacobian_programs
            .iter()
            .position(|entry| matches!(entry.col_axis, crate::codegen::ColumnAxis::Node(2)))
            .expect("a Jacobian column for V(t)");

        let mut harness = FusedKernelHarness::for_source(SOURCE, "wasm_pow_exponent_zero_base");
        harness.reset();
        // The harness's default point is not this fixture's: the parameters
        // keep their declared defaults and every terminal sits at 0 V, which
        // puts the power's base at exactly zero.
        for (index, parameter) in report.model.parameters.iter().enumerate() {
            harness.write_f64(
                FusedKernelHarness::PARAMETERS as usize + index * size_of::<f64>(),
                parameter.default,
            );
        }
        for index in 0..3 {
            harness.write_f64(
                FusedKernelHarness::VOLTAGES as usize + index * size_of::<f64>(),
                0.0,
            );
        }
        harness.call_assignments();
        harness.call_prelude();
        let export = harness.jacobian_export(0, entry);
        assert_eq!(harness.call(&export), 0);
        let value = harness.read_f64(FRAME_RESULT_OFFSET as usize);
        assert!(
            value.is_finite(),
            "d I(p, n) / d V(t) at V(p, n) = 0 under wasm is {value}; the exponent term is 0 · ln(0) · 0 and must be 0"
        );
        assert_eq!(value, 0.0);
    }

    /// The power rule's base term at a zero base, under wasmi.
    ///
    /// Sibling of [`power_rule_exponent_term_is_finite_at_a_zero_base_under_wasm`]
    /// and of the x64 fixture
    /// (`native::x64::tests::power_rule_base_term_is_finite_at_a_zero_base`):
    /// `b · a^(b−1) · da` is `∞ · 0` at `a = 0` for `b < 1`, and the merge
    /// keeps `da/dV(t)` a live lane whose value is exactly 0.
    #[test]
    fn power_rule_base_term_is_finite_at_a_zero_base_under_wasm() {
        use std::mem::size_of;

        use super::abi::FRAME_RESULT_OFFSET;

        const SOURCE: &str = r#"
module wasm_pow_base_zero_base(p, n, t);
  inout p, n, t;
  electrical p, n, t;
  parameter integer sh = 0;
  real a;
  analog begin
    if (sh != 0)
      a = V(p, n) + V(t);
    else
      a = V(p, n);
    I(p, n) <+ 1.0e-3 * pow(a, 0.5);
  end
endmodule
"#;
        let report = VerilogACompiler::new(CompilerOptions::default())
            .compile_runtime(SOURCE, Some("wasm_pow_base_zero_base"))
            .expect("compile fixture");
        let entry = report.model.stamp_programs[0]
            .jacobian_programs
            .iter()
            .position(|entry| matches!(entry.col_axis, crate::codegen::ColumnAxis::Node(2)))
            .expect("a Jacobian column for V(t)");

        let mut harness = FusedKernelHarness::for_source(SOURCE, "wasm_pow_base_zero_base");
        harness.reset();
        for (index, parameter) in report.model.parameters.iter().enumerate() {
            harness.write_f64(
                FusedKernelHarness::PARAMETERS as usize + index * size_of::<f64>(),
                parameter.default,
            );
        }
        for index in 0..3 {
            harness.write_f64(
                FusedKernelHarness::VOLTAGES as usize + index * size_of::<f64>(),
                0.0,
            );
        }
        harness.call_assignments();
        harness.call_prelude();
        let export = harness.jacobian_export(0, entry);
        assert_eq!(harness.call(&export), 0);
        let value = harness.read_f64(FRAME_RESULT_OFFSET as usize);
        assert!(
            value.is_finite(),
            "d I(p, n) / d V(t) at V(p, n) = 0 under wasm is {value}; the base term is 0.5 · 0^-0.5 · 0 and must be 0"
        );
        assert_eq!(value, 0.0);
    }

    /// The wasm half of
    /// `native::x64::tests::condition_snapshot_of_an_untaken_block_stays_out_of_the_physics`:
    /// the plan is shared, so a hoisted inner-`if` snapshot that reads a slot
    /// the untaken block did not write must not select the block's assignment
    /// on a second pass either. `I(p, n) = g · V(p, n)` with `g = 1.0` below
    /// `vth`; a leaking snapshot would make it `2.0 · V(p, n)`.
    #[test]
    fn a_condition_snapshot_of_an_untaken_block_stays_out_of_the_wasm_physics() {
        use std::mem::size_of;

        use super::abi::FRAME_RESULT_OFFSET;

        const SOURCE: &str = r#"
module wasm_condition_snapshot_untaken(p, n);
  inout p, n;
  electrical p, n;
  parameter real vth = 0.5;
  real g, t;
  analog begin
    g = 1.0;
    if (V(p, n) > vth) begin
      t = V(p, n) - vth;
      if (t > 0.1) g = 2.0;
    end
    t = 3.0;
    I(p, n) <+ g * V(p, n);
  end
endmodule
"#;
        const BIAS: f64 = 0.25;

        let report = VerilogACompiler::new(CompilerOptions::default())
            .compile_runtime(SOURCE, Some("wasm_condition_snapshot_untaken"))
            .expect("compile fixture");
        let mut harness = FusedKernelHarness::for_source(SOURCE, "wasm_condition_snapshot_untaken");
        let export = harness.stamp_value_export(0);
        harness.reset();
        for (index, parameter) in report.model.parameters.iter().enumerate() {
            harness.write_f64(
                FusedKernelHarness::PARAMETERS as usize + index * size_of::<f64>(),
                parameter.default,
            );
        }
        harness.write_f64(FusedKernelHarness::VOLTAGES as usize, BIAS);
        harness.write_f64(
            FusedKernelHarness::VOLTAGES as usize + size_of::<f64>(),
            0.0,
        );
        for pass in 0..2 {
            harness.call_assignments();
            harness.call_prelude();
            assert_eq!(harness.call(&export), 0);
            let value = harness.read_f64(FRAME_RESULT_OFFSET as usize);
            assert_eq!(
                value, BIAS,
                "pass {pass}: I(p, n) = g · V(p, n) with g = 1.0 below vth; the snapshot of the untaken block's inner condition must not select g = 2.0"
            );
        }
    }

    /// The wasm half of
    /// `native::x64::tests::assignment_pass_power_rule_base_term_is_finite_at_a_zero_base`:
    /// the assignment pass is lowered once by `native::expr` and emitted for
    /// both machines, so its power rule's base term is read back here off the
    /// variable array. `seen` is event state, which roots the pass on `c` and
    /// through it on `q`'s shadow along `V(t)` — without a root the pass would
    /// not compute `c` at all and the read would be a slot nobody wrote.
    #[test]
    fn the_wasm_assignment_pass_power_rule_base_term_is_finite_at_a_zero_base() {
        use std::mem::size_of;

        const SOURCE: &str = r#"
module wasm_pow_shadow_zero_base(p, n, t);
  inout p, n, t;
  electrical p, n, t;
  parameter integer sh = 0;
  real a, q, c, seen;
  analog begin
    if (sh != 0)
      a = V(p, n) + V(t);
    else
      a = V(p, n);
    q = 1.0e-12 * pow(a, 0.5);
    c = ddx(q, V(t));
    @(initial_step) seen = c;
    I(p, n) <+ ddt(q);
    I(p, n) <+ 1.0e-3 * V(p, n);
  end
endmodule
"#;

        let report = VerilogACompiler::new(CompilerOptions::default())
            .compile_runtime(SOURCE, Some("wasm_pow_shadow_zero_base"))
            .expect("compile fixture");
        assert!(
            report.model.num_variables <= 32,
            "the harness's variable region holds 32 slots"
        );
        let slot = |name: &str| {
            report
                .model
                .variable_names
                .iter()
                .position(|variable| variable == name)
                .unwrap_or_else(|| panic!("variable {name}"))
        };
        let (a, c) = (slot("a"), slot("c"));
        let mut harness = FusedKernelHarness::for_source(SOURCE, "wasm_pow_shadow_zero_base");
        harness.reset();
        for (index, parameter) in report.model.parameters.iter().enumerate() {
            harness.write_f64(
                FusedKernelHarness::PARAMETERS as usize + index * size_of::<f64>(),
                parameter.default,
            );
        }
        let variable =
            |index: usize| FusedKernelHarness::VARIABLES as usize + index * size_of::<f64>();

        // Away from zero first, to prove the pass writes the slots this test
        // reads: `a` is `V(p, n)` and `c` is 0 because `a` does not carry V(t).
        for (index, value) in [0.09, 0.0, 0.0].into_iter().enumerate() {
            harness.write_f64(
                FusedKernelHarness::VOLTAGES as usize + index * size_of::<f64>(),
                value,
            );
        }
        harness.call_assignments();
        assert_eq!(harness.read_f64(variable(a)), 0.09, "the pass publishes a");
        assert_eq!(harness.read_f64(variable(c)), 0.0, "dq/dV(t) at a = 0.09");

        // At the zero base the same term is `0.5 · 0^-0.5 · 0`.
        harness.write_f64(FusedKernelHarness::VOLTAGES as usize, 0.0);
        harness.call_assignments();
        let value = harness.read_f64(variable(c));
        assert!(
            value.is_finite(),
            "c = ddx(q, V(t)) at V(p, n) = 0 under wasm is {value}; the base term is 0.5 · 0^-0.5 · 0 and must be 0"
        );
        assert_eq!(value, 0.0);
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
        let stamp_count = harness.stamp_count();
        assert!(
            harness.stamp_jacobians.iter().any(|entries| *entries >= 2),
            "one stamp must carry several Jacobian entries, or the per-stamp \
             output base is never exercised"
        );

        // Per-entry path, interleaved exactly as the driver runs it: a stamp's
        // value publishes before its own derivatives are evaluated.
        harness.reset();
        harness.call_assignments();
        harness.call_prelude();
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
            FRAME_PARAMETERS_LEN_OFFSET, FRAME_PARAMETERS_PTR_OFFSET,
            FRAME_PRELUDE_SLOTS_LEN_OFFSET, FRAME_PRELUDE_SLOTS_PTR_OFFSET, FRAME_RESULT_OFFSET,
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
        let mut native_prelude_slots =
            vec![0.0_f64; native.required_storage().prelude_slots.max(1)];
        let mut native_context = EvalContext::empty_for_test();
        native_context.params = params.as_ptr();
        native_context.voltages = voltages.as_ptr();
        native_context.num_terminals = report.model.num_terminals;
        native_context.prelude_slots = native_prelude_slots.as_mut_ptr();
        native_context.prelude_slots_len = native_prelude_slots.len();
        native.run_assignments(&native_context, native_variables.as_mut_ptr());
        // Both backends publish their prelude between the assignment pass and
        // the first entry, and this test's whole claim is that the two agree
        // entry by entry — so both have to have run it, or it compares one
        // route's values against the other route's seeded zeros.
        native.run_prelude(&native_context, native_variables.as_ptr());
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
        super::codegen::define_test_math_imports(&mut linker, memory);
        let instance = linker
            .instantiate_and_start(&mut store, &module)
            .expect("instantiate browser-WASM differential model");

        const FRAME_OFFSET: usize = 0;
        const PARAMS_OFFSET: usize = 256;
        const VOLTAGES_OFFSET: usize = 512;
        const VARIABLES_OFFSET: usize = 768;
        const PRELUDE_SLOTS_OFFSET: usize = 4096;
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
        write_frame_u32(FRAME_PRELUDE_SLOTS_PTR_OFFSET, PRELUDE_SLOTS_OFFSET as u32);
        write_frame_u32(
            FRAME_PRELUDE_SLOTS_LEN_OFFSET,
            artifact.prelude_slots() as u32,
        );
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

        if let Some(export) = artifact.assignment_export() {
            let assignment = instance
                .get_typed_func::<i32, i32>(&store, export)
                .expect("resolve WASM assignment kernel");
            assert_eq!(
                assignment
                    .call(&mut store, FRAME_OFFSET as i32)
                    .expect("run WASM assignment kernel"),
                super::WASM_JIT_STATUS_OK
            );
        }
        if let Some(export) = artifact.prelude_export() {
            let prelude = instance
                .get_typed_func::<i32, i32>(&store, export)
                .expect("resolve WASM prelude");
            assert_eq!(
                prelude
                    .call(&mut store, FRAME_OFFSET as i32)
                    .expect("run WASM prelude"),
                super::WASM_JIT_STATUS_OK
            );
        }
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
                WasmJitValueRole::Assignment { .. } | WasmJitValueRole::Prelude => continue,
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
                WasmJitValueRole::LimiterCorrection { stamp_index } => native
                    .run_limiter_correction(
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
