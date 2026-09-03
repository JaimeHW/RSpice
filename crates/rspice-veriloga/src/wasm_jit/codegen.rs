//! SSA-to-standard-WebAssembly value-entry encoding.

use std::borrow::Cow;

use wasm_encoder::{
    BlockType, CodeSection, CustomSection, ExportKind, ExportSection, Function, FunctionSection,
    ImportSection, Instruction as WasmInstruction, MemArg, MemoryType, Module, TypeSection,
    ValType,
};
use wasmparser::{Encoding, Parser, Payload, Validator};

use super::abi::*;
use super::{
    WASM_JIT_ABI_VERSION, WASM_JIT_CONTRACT_SECTION, WASM_JIT_EMITTER_VERSION,
    WASM_JIT_IMPORT_MODULE, WASM_JIT_MEMORY_IMPORT, WasmJitError, WasmJitResult,
};
#[cfg(test)]
use crate::jit::expr::NativeProgram;
use crate::jit::expr::{
    BinaryMathOp, CompareOp, ExtremumOp, IntegerBinaryOp, LogicalOp, NativeOp, UnaryMathOp,
    VoltageNode,
};
use crate::jit::plan_program::PlanProgramRef;
use crate::jit::ssa::{Instruction, Program, Terminator};

pub(crate) const WASM_JIT_EVAL_HELPER_IMPORT: &str = "eval_op_v1";
/// Frame-relative, bounded variable-arity helper capability.
pub(crate) const WASM_JIT_SLICE_HELPER_IMPORT: &str = "eval_op_slice_v1";
/// Frame-free unary transcendental capability.
///
/// `exp` and `ln` dominate every semiconductor model's inner loop, so they do
/// not go through the general `eval_op_v1` descriptor: that path pushes ten
/// arguments, revalidates the evaluation frame, and forces an error-status
/// reload and branch at every call site. These two imports take only what the
/// operation needs and cannot fail, so the call site is a push, a call, and
/// nothing else.
pub(crate) const WASM_JIT_MATH1_IMPORT: &str = "math1_v1";
/// Frame-free binary transcendental capability. See [`WASM_JIT_MATH1_IMPORT`].
pub(crate) const WASM_JIT_MATH2_IMPORT: &str = "math2_v1";
pub(crate) const WASM_JIT_VALUE_EXPORT: &str = "rspice_wasm_jit_value";
pub(crate) const WASM_JIT_ASSIGNMENT_EXPORT: &str = "rspice_wasm_jit_assign";
pub(crate) const WASM_JIT_POST_ASSIGNMENT_EXPORT: &str = "rspice_wasm_jit_post_assign";
/// Driver evaluating every stamp value in one call.
pub(crate) const WASM_JIT_EVALUATION_KERNEL_EXPORT: &str = "rspice_wasm_jit_eval_kernel";
/// Driver evaluating every stamp value and Jacobian entry in one call.
pub(crate) const WASM_JIT_STAMP_KERNEL_EXPORT: &str = "rspice_wasm_jit_stamp_kernel";
const MAX_RUNTIME_LOOP_ITERATIONS: i32 = 100_000;

const ENTRY_TYPE_INDEX: u32 = 0;
const HELPER_TYPE_INDEX: u32 = 1;
const MATH1_TYPE_INDEX: u32 = 2;
const MATH2_TYPE_INDEX: u32 = 3;
const SLICE_HELPER_TYPE_INDEX: u32 = 4;
const HELPER_FUNCTION_INDEX: u32 = 0;
const MATH1_FUNCTION_INDEX: u32 = 1;
const MATH2_FUNCTION_INDEX: u32 = 2;
const SLICE_HELPER_FUNCTION_INDEX: u32 = 3;
/// Imported functions occupy the low indices, so generated entries start after
/// the whole capability surface.
const ENTRY_FUNCTION_INDEX: u32 = 4;
/// Entry signature plus the four capability signatures.
const CAPABILITY_TYPE_COUNT: u32 = 5;
/// Linear memory plus the four imported capability functions.
const CAPABILITY_IMPORT_COUNT: u32 = 5;
const FRAME_LOCAL: u32 = 0;

/// Declare the type section every generated module shares.
fn encode_capability_types(module: &mut Module) {
    let mut types = TypeSection::new();
    types.ty().function([ValType::I32], [ValType::I32]);
    types.ty().function(
        [
            ValType::I32,
            ValType::I32,
            ValType::I32,
            ValType::I32,
            ValType::I64,
            ValType::F64,
            ValType::F64,
            ValType::F64,
            ValType::F64,
            ValType::F64,
        ],
        [ValType::F64],
    );
    types
        .ty()
        .function([ValType::I32, ValType::F64], [ValType::F64]);
    types
        .ty()
        .function([ValType::I32, ValType::F64, ValType::F64], [ValType::F64]);
    types.ty().function(
        [
            ValType::I32,
            ValType::I32,
            ValType::I32,
            ValType::I32,
            ValType::I64,
            ValType::I32,
        ],
        [ValType::F64],
    );
    module.section(&types);
}

/// Declare the import section every generated module shares.
fn encode_capability_imports(module: &mut Module) {
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
    imports.import(
        WASM_JIT_IMPORT_MODULE,
        WASM_JIT_EVAL_HELPER_IMPORT,
        wasm_encoder::EntityType::Function(HELPER_TYPE_INDEX),
    );
    imports.import(
        WASM_JIT_IMPORT_MODULE,
        WASM_JIT_MATH1_IMPORT,
        wasm_encoder::EntityType::Function(MATH1_TYPE_INDEX),
    );
    imports.import(
        WASM_JIT_IMPORT_MODULE,
        WASM_JIT_MATH2_IMPORT,
        wasm_encoder::EntityType::Function(MATH2_TYPE_INDEX),
    );
    imports.import(
        WASM_JIT_IMPORT_MODULE,
        WASM_JIT_SLICE_HELPER_IMPORT,
        wasm_encoder::EntityType::Function(SLICE_HELPER_TYPE_INDEX),
    );
    module.section(&imports);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum WasmAssignment {
    Direct {
        variable_index: u32,
        value_entry: u32,
    },
    Indexed {
        base: u32,
        len: u32,
        lower: i64,
        index_entry: u32,
        value_entry: u32,
    },
    Loop {
        condition_entry: u32,
        body: Vec<WasmAssignment>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WasmAssignmentKernel {
    pub(crate) export_name: &'static str,
    pub(crate) assignments: Vec<WasmAssignment>,
}

/// One stamp's work inside a fused kernel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WasmKernelStamp {
    /// Scalar entry producing the stamp's contribution value.
    pub(crate) value_entry: u32,
    /// Terminal-pair matrix slots the value publishes into, forward then
    /// reverse. The reverse slot receives the negated value.
    pub(crate) current_pair: Option<(u32, u32)>,
    /// Scalar entries producing this stamp's Jacobian row, in output order.
    pub(crate) jacobian_entries: Vec<u32>,
    /// Offset of this stamp's first Jacobian output in the flattened array.
    pub(crate) jacobian_output_base: u32,
}

/// A driver that evaluates a whole model in one call.
///
/// Without this, the browser pays a JavaScript round trip per scalar: one for
/// every stamp value and every Jacobian entry, every Newton iteration, for
/// every instance. The native backends have always fused the same work behind
/// a single entry point, and this is the direct port of that driver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WasmFusedKernel {
    pub(crate) export_name: &'static str,
    /// Index of the assignment kernel this driver runs first.
    pub(crate) assignment_kernel: u32,
    /// Scalar entry of the CFG route's assignment pass, run once after the
    /// assignment kernel and before the first stamp. `None` for a postfix plan,
    /// which is why a shipped driver's body is unchanged.
    pub(crate) prelude_entry: Option<u32>,
    pub(crate) stamps: Vec<WasmKernelStamp>,
    /// Whether the driver also evaluates and publishes Jacobian entries.
    pub(crate) with_jacobians: bool,
}

#[cfg(test)]
pub(crate) fn encode_value_program(program: &NativeProgram) -> WasmJitResult<Vec<u8>> {
    encode_value_module(encode_value_body(PlanProgramRef::Postfix(program))?)
}

/// Wrap one encoded value body in a single-entry module.
///
/// Single-entry modules exist for the encoder tests; production publishes the
/// whole model as one module.
#[cfg(test)]
fn encode_value_module(body: Function) -> WasmJitResult<Vec<u8>> {
    let mut module = Module::new();

    encode_capability_types(&mut module);
    encode_capability_imports(&mut module);

    let mut functions = FunctionSection::new();
    functions.function(ENTRY_TYPE_INDEX);
    module.section(&functions);

    let mut exports = ExportSection::new();
    exports.export(
        WASM_JIT_VALUE_EXPORT,
        ExportKind::Func,
        ENTRY_FUNCTION_INDEX,
    );
    module.section(&exports);

    let mut contract = Vec::with_capacity(12);
    contract.extend_from_slice(&WASM_JIT_ABI_VERSION.to_le_bytes());
    contract.extend_from_slice(&WASM_JIT_EMITTER_VERSION.to_le_bytes());
    contract.extend_from_slice(&WASM_JIT_EVAL_FRAME_BYTES.to_le_bytes());
    module.section(&CustomSection {
        name: Cow::Borrowed(WASM_JIT_CONTRACT_SECTION),
        data: Cow::Owned(contract),
    });

    let mut code = CodeSection::new();
    code.function(&body);
    module.section(&code);
    Ok(module.finish())
}

pub(crate) fn emit_verified_value_program_set(
    programs: &[PlanProgramRef<'_>],
) -> WasmJitResult<Vec<u8>> {
    let bytes = encode_model_program_set(programs, &[], &[])?;
    verify_value_program_set(&bytes, programs)?;
    Ok(bytes)
}

pub(crate) fn emit_verified_model_module(
    programs: &[PlanProgramRef<'_>],
    kernels: &[WasmAssignmentKernel],
    fused: &[WasmFusedKernel],
) -> WasmJitResult<Vec<u8>> {
    let bytes = encode_model_program_set(programs, kernels, fused)?;
    verify_model_module(&bytes, programs, kernels, fused)?;
    Ok(bytes)
}

pub(crate) fn verify_value_program_set(
    bytes: &[u8],
    expected_programs: &[PlanProgramRef<'_>],
) -> WasmJitResult<()> {
    if bytes.len() > super::SHIPPED_MODEL_WASM_CODE_SIZE_BUDGET_BYTES {
        return Err(WasmJitError::ArtifactTooLarge {
            actual: bytes.len(),
            limit: super::SHIPPED_MODEL_WASM_CODE_SIZE_BUDGET_BYTES,
        });
    }
    Validator::new()
        .validate_all(bytes)
        .map_err(|error| WasmJitError::BinaryValidation(error.to_string()))?;
    if bytes != encode_model_program_set(expected_programs, &[], &[])? {
        return Err(WasmJitError::Contract(
            "model value-entry module does not match deterministic translation of its authenticated SSA"
                .into(),
        ));
    }

    let expected_count = u32::try_from(expected_programs.len())
        .map_err(|_| WasmJitError::Encoding("model entry count exceeds u32".into()))?;
    verify_value_module_shape(bytes, expected_count, &[], &[], false)
}

fn verify_model_module(
    bytes: &[u8],
    expected_programs: &[PlanProgramRef<'_>],
    kernels: &[WasmAssignmentKernel],
    fused: &[WasmFusedKernel],
) -> WasmJitResult<()> {
    if bytes.len() > super::SHIPPED_MODEL_WASM_CODE_SIZE_BUDGET_BYTES {
        return Err(WasmJitError::ArtifactTooLarge {
            actual: bytes.len(),
            limit: super::SHIPPED_MODEL_WASM_CODE_SIZE_BUDGET_BYTES,
        });
    }
    Validator::new()
        .validate_all(bytes)
        .map_err(|error| WasmJitError::BinaryValidation(error.to_string()))?;
    if bytes != encode_model_program_set(expected_programs, kernels, fused)? {
        return Err(WasmJitError::Contract(
            "model module does not match deterministic translation of its authenticated plan"
                .into(),
        ));
    }
    let scalar_count = u32::try_from(expected_programs.len())
        .map_err(|_| WasmJitError::Encoding("model entry count exceeds u32".into()))?;
    let assignment_exports = kernels
        .iter()
        .map(|kernel| kernel.export_name)
        .collect::<Vec<_>>();
    let fused_exports = fused
        .iter()
        .map(|kernel| kernel.export_name)
        .collect::<Vec<_>>();
    verify_value_module_shape(
        bytes,
        scalar_count,
        &assignment_exports,
        &fused_exports,
        false,
    )
}

fn encode_model_program_set(
    programs: &[PlanProgramRef<'_>],
    kernels: &[WasmAssignmentKernel],
    fused: &[WasmFusedKernel],
) -> WasmJitResult<Vec<u8>> {
    let value_bodies = programs
        .iter()
        .map(|program| encode_value_body(*program))
        .collect::<WasmJitResult<Vec<_>>>()?;
    let function_count = u32::try_from(value_bodies.len())
        .map_err(|_| WasmJitError::Encoding("model entry count exceeds u32".into()))?;
    let kernel_count = u32::try_from(kernels.len())
        .map_err(|_| WasmJitError::Encoding("assignment kernel count exceeds u32".into()))?;
    let fused_count = u32::try_from(fused.len())
        .map_err(|_| WasmJitError::Encoding("fused kernel count exceeds u32".into()))?;
    let mut module = Module::new();

    encode_capability_types(&mut module);
    encode_capability_imports(&mut module);

    let mut functions = FunctionSection::new();
    for _ in 0..function_count + kernel_count + fused_count {
        functions.function(ENTRY_TYPE_INDEX);
    }
    module.section(&functions);

    let mut exports = ExportSection::new();
    for index in 0..function_count {
        exports.export(
            &format!("rspice_wasm_jit_value_{index:08x}"),
            ExportKind::Func,
            ENTRY_FUNCTION_INDEX + index,
        );
    }
    for (index, kernel) in kernels.iter().enumerate() {
        let index = u32::try_from(index)
            .map_err(|_| WasmJitError::Encoding("assignment kernel index exceeds u32".into()))?;
        exports.export(
            kernel.export_name,
            ExportKind::Func,
            ENTRY_FUNCTION_INDEX + function_count + index,
        );
    }
    for (index, kernel) in fused.iter().enumerate() {
        let index = u32::try_from(index)
            .map_err(|_| WasmJitError::Encoding("fused kernel index exceeds u32".into()))?;
        exports.export(
            kernel.export_name,
            ExportKind::Func,
            ENTRY_FUNCTION_INDEX + function_count + kernel_count + index,
        );
    }
    module.section(&exports);

    let mut contract = Vec::with_capacity(24);
    contract.extend_from_slice(&WASM_JIT_ABI_VERSION.to_le_bytes());
    contract.extend_from_slice(&WASM_JIT_EMITTER_VERSION.to_le_bytes());
    contract.extend_from_slice(&WASM_JIT_EVAL_FRAME_BYTES.to_le_bytes());
    contract.extend_from_slice(&function_count.to_le_bytes());
    contract.extend_from_slice(&kernel_count.to_le_bytes());
    contract.extend_from_slice(&fused_count.to_le_bytes());
    module.section(&CustomSection {
        name: Cow::Borrowed(WASM_JIT_CONTRACT_SECTION),
        data: Cow::Owned(contract),
    });

    let mut code = CodeSection::new();
    for body in &value_bodies {
        code.function(body);
    }
    for kernel in kernels {
        code.function(&encode_assignment_kernel(kernel, function_count)?);
    }
    for kernel in fused {
        code.function(&encode_fused_kernel(kernel, function_count, kernel_count)?);
    }
    module.section(&code);
    Ok(module.finish())
}

/// Encode one plan entry.
///
/// Both forms reach the same encoder, the same helper imports and the same
/// module validator: a postfix entry is lifted into SSA the way it always was,
/// and a block entry is already in that form.
fn encode_value_body(program: PlanProgramRef<'_>) -> WasmJitResult<Function> {
    match program {
        PlanProgramRef::Postfix(program) => {
            let ssa = Program::lower(program)
                .map_err(|error| WasmJitError::Encoding(error.to_string()))?;
            encode_value_body_from_ssa(&ssa)
        }
        PlanProgramRef::Blocks(program) => encode_value_body_from_ssa(program.ssa()),
    }
}

/// Encode one already-lowered value entry.
///
/// Taking the SSA directly is what lets the branch form of a conditional reach
/// this encoder through the same locals, helpers and module validator as the
/// shipped select form.
fn encode_value_body_from_ssa(ssa: &Program) -> WasmJitResult<Function> {
    let verifier = |error: crate::jit::JitError| WasmJitError::Encoding(error.to_string());
    let loops = ssa.loop_ranges().map_err(verifier)?;
    // One scratch local per block parameter of the widest merge, used only to
    // stage a back edge's arguments. See `emit_edge_arguments`.
    let scratch = if loops.is_empty() {
        0
    } else {
        ssa.blocks()
            .iter()
            .map(|block| ssa.parameters(block).len())
            .max()
            .unwrap_or(0)
    };
    let local_count = u32::try_from(ssa.value_count() + scratch)
        .map_err(|_| WasmJitError::Encoding("SSA local count exceeds u32".into()))?;
    let locals = (local_count != 0)
        .then_some((local_count, ValType::F64))
        .into_iter();
    let mut body = Function::new(locals);
    emit_frame_guard(&mut body);
    emit_clear_error_status(&mut body);
    let mut labels = Vec::new();
    emit_block_region(
        &mut body,
        ssa,
        ssa.entry(),
        None,
        &loops,
        &mut labels,
        ssa.value_count(),
    )?;
    body.instruction(&WasmInstruction::I32Const(WASM_JIT_STATUS_OK));
    body.instruction(&WasmInstruction::End);
    Ok(body)
}

/// One entry of the WebAssembly label stack, innermost last.
///
/// `br` counts labels outwards from zero, so a jump back to a loop header has
/// to know how many `if` and `loop` labels have opened since that loop's own.
/// Tracking them explicitly is what keeps a nested `if` inside a loop body
/// from silently shifting the branch depth.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WasmLabel {
    Loop(crate::jit::ssa::BlockId),
    Structured,
}

/// Emit the straight-line chain of blocks starting at `start`, stopping when
/// it reaches `stop`.
///
/// WebAssembly has no `goto`, so control flow is realized structurally: a
/// branch becomes `if`/`else`/`end` around the two arm regions, and execution
/// continues at the block where they reconverge. The SSA verifier already
/// requires every branch's arms to be single-entry regions that reconverge at
/// exactly one block, which is what makes this total without a relooper.
///
/// A loop is the one shape that is not a reconverging diamond, and it gets the
/// only other structure WebAssembly has. A natural loop headed by `H` and
/// exited on one of `H`'s two edges becomes
///
/// ```text
/// loop
///   <H's instructions>
///   <condition>
///   if
///     <body region, ending in `br` back to this loop>
///   end          ;; falling out of the `if` leaves the loop
/// end
/// <exit edge's arguments>
/// ```
///
/// which needs no `block` wrapper and no `br_if`, because a `loop` label
/// repeats only when something branches to it. The verifier has already proved
/// the loop occupies one contiguous layout range with a single entry, so the
/// body region is exactly the blocks between the header and the latch.
fn emit_block_region(
    body: &mut Function,
    ssa: &Program,
    start: crate::jit::ssa::BlockId,
    stop: Option<crate::jit::ssa::BlockId>,
    loops: &[crate::jit::ssa::LoopRange],
    labels: &mut Vec<WasmLabel>,
    scratch_base: usize,
) -> WasmJitResult<()> {
    let verifier = |error: crate::jit::JitError| WasmJitError::Encoding(error.to_string());
    let mut current = start;
    loop {
        if Some(current) == stop {
            return Ok(());
        }
        // A jump to a header whose `loop` is still open is the back edge.
        if let Some(depth) = branch_depth(labels, current) {
            body.instruction(&WasmInstruction::Br(depth));
            return Ok(());
        }
        let block = ssa.block(current).map_err(verifier)?;
        let header_of = loops
            .iter()
            .find(|range| range.header() == current)
            .copied();
        if let Some(range) = header_of {
            emit_loop(body, ssa, block, range, loops, labels, scratch_base)?;
            let Terminator::Branch {
                then_edge,
                else_edge,
                ..
            } = block.terminator()
            else {
                return Err(WasmJitError::Encoding(format!(
                    "SSA loop header {} does not end in a two-way branch",
                    current.index()
                )));
            };
            let exit = if range.contains_layout(then_edge.target()) {
                else_edge
            } else {
                then_edge
            };
            emit_edge_arguments(body, ssa, exit, scratch_base)?;
            current = exit.target();
            continue;
        }
        for instruction in &ssa.instructions()[block.instruction_start()..block.instruction_end()] {
            emit_instruction(body, instruction)?;
            body.instruction(&WasmInstruction::LocalSet(value_local(
                instruction.result().index(),
            )?));
        }
        match block.terminator() {
            Terminator::Return(value) => {
                body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
                body.instruction(&WasmInstruction::LocalGet(value_local(value.index())?));
                body.instruction(&WasmInstruction::F64Store(f64_mem(FRAME_RESULT_OFFSET)));
                return Ok(());
            }
            Terminator::Jump(edge) => {
                emit_edge_arguments(body, ssa, edge, scratch_base)?;
                current = edge.target();
            }
            Terminator::Branch {
                condition,
                then_edge,
                else_edge,
            } => {
                let join = ssa.branch_join(block).map_err(verifier)?;
                // Verilog-A truthiness: anything but exact zero, NaN included.
                // `f64.ne` reports an unordered compare as true, which is the
                // same predicate the select form feeds to `select`.
                body.instruction(&WasmInstruction::LocalGet(value_local(condition.index())?));
                body.instruction(&WasmInstruction::F64Const(0.0.into()));
                body.instruction(&WasmInstruction::F64Ne);
                body.instruction(&WasmInstruction::If(BlockType::Empty));
                labels.push(WasmLabel::Structured);
                emit_edge_arguments(body, ssa, then_edge, scratch_base)?;
                emit_block_region(
                    body,
                    ssa,
                    then_edge.target(),
                    Some(join),
                    loops,
                    labels,
                    scratch_base,
                )?;
                body.instruction(&WasmInstruction::Else);
                emit_edge_arguments(body, ssa, else_edge, scratch_base)?;
                emit_block_region(
                    body,
                    ssa,
                    else_edge.target(),
                    Some(join),
                    loops,
                    labels,
                    scratch_base,
                )?;
                body.instruction(&WasmInstruction::End);
                labels.pop();
                current = join;
            }
        }
    }
}

/// Emit one natural loop's `loop ... end`, leaving the exit edge to the caller.
fn emit_loop(
    body: &mut Function,
    ssa: &Program,
    header: &crate::jit::ssa::BasicBlock,
    range: crate::jit::ssa::LoopRange,
    loops: &[crate::jit::ssa::LoopRange],
    labels: &mut Vec<WasmLabel>,
    scratch_base: usize,
) -> WasmJitResult<()> {
    let Terminator::Branch {
        condition,
        then_edge,
        else_edge,
    } = header.terminator()
    else {
        return Err(WasmJitError::Encoding(format!(
            "SSA loop header {} does not end in a two-way branch",
            header.id().index()
        )));
    };
    let (into_body, continues_when_true) = if range.contains_layout(then_edge.target()) {
        (then_edge, true)
    } else if range.contains_layout(else_edge.target()) {
        (else_edge, false)
    } else {
        return Err(WasmJitError::Encoding(format!(
            "SSA loop headed by block {} has no edge into its own body",
            header.id().index()
        )));
    };

    body.instruction(&WasmInstruction::Loop(BlockType::Empty));
    labels.push(WasmLabel::Loop(header.id()));
    for instruction in &ssa.instructions()[header.instruction_start()..header.instruction_end()] {
        emit_instruction(body, instruction)?;
        body.instruction(&WasmInstruction::LocalSet(value_local(
            instruction.result().index(),
        )?));
    }
    body.instruction(&WasmInstruction::LocalGet(value_local(condition.index())?));
    body.instruction(&WasmInstruction::F64Const(0.0.into()));
    if continues_when_true {
        body.instruction(&WasmInstruction::F64Ne);
    } else {
        body.instruction(&WasmInstruction::F64Eq);
    }
    body.instruction(&WasmInstruction::If(BlockType::Empty));
    labels.push(WasmLabel::Structured);
    emit_edge_arguments(body, ssa, into_body, scratch_base)?;
    emit_block_region(
        body,
        ssa,
        into_body.target(),
        None,
        loops,
        labels,
        scratch_base,
    )?;
    body.instruction(&WasmInstruction::End);
    labels.pop();
    body.instruction(&WasmInstruction::End);
    labels.pop();
    Ok(())
}

/// How many labels out the still-open `loop` for `header` sits, or `None` when
/// no such loop is open.
fn branch_depth(labels: &[WasmLabel], header: crate::jit::ssa::BlockId) -> Option<u32> {
    labels
        .iter()
        .rev()
        .position(|label| *label == WasmLabel::Loop(header))
        .and_then(|depth| u32::try_from(depth).ok())
}

/// Bind one edge's arguments to its target's block parameters.
///
/// Every SSA value owns a distinct local, so on a forward edge no parameter
/// can already hold an argument and the assignments need no sequencing. A loop
/// back edge is where that stops being true: the header's parameters are
/// exactly what the latch passes back, so `x, y = y, x` around one is a
/// permutation of the locals and a straight sequence of `local.set` would
/// lose a value. The machine backends break such a cycle with one reserved
/// spill slot; here the equivalent is a reserved band of locals, and staging
/// every argument through it costs one extra copy per parameter on the edges
/// that need it and nothing anywhere else.
fn emit_edge_arguments(
    body: &mut Function,
    ssa: &Program,
    edge: &crate::jit::ssa::Edge,
    scratch_base: usize,
) -> WasmJitResult<()> {
    let target = ssa
        .block(edge.target())
        .map_err(|error| WasmJitError::Encoding(error.to_string()))?;
    let parameters = ssa.parameters(target);
    let aliases = parameters.iter().any(|parameter| {
        edge.arguments()
            .iter()
            .any(|argument| *argument == parameter.value())
    });
    if aliases {
        for (slot, argument) in edge.arguments().iter().enumerate() {
            body.instruction(&WasmInstruction::LocalGet(value_local(argument.index())?));
            body.instruction(&WasmInstruction::LocalSet(value_local(
                scratch_base + slot,
            )?));
        }
        for (slot, parameter) in parameters.iter().enumerate() {
            body.instruction(&WasmInstruction::LocalGet(value_local(
                scratch_base + slot,
            )?));
            body.instruction(&WasmInstruction::LocalSet(value_local(
                parameter.value().index(),
            )?));
        }
        return Ok(());
    }
    for (argument, parameter) in edge.arguments().iter().zip(parameters) {
        body.instruction(&WasmInstruction::LocalGet(value_local(argument.index())?));
        body.instruction(&WasmInstruction::LocalSet(value_local(
            parameter.value().index(),
        )?));
    }
    Ok(())
}

fn encode_assignment_kernel(
    kernel: &WasmAssignmentKernel,
    scalar_count: u32,
) -> WasmJitResult<Function> {
    let loop_depth = assignment_loop_depth(&kernel.assignments)?;
    let loop_depth_u32 = u32::try_from(loop_depth)
        .map_err(|_| WasmJitError::Encoding("assignment loop depth exceeds wasm32".into()))?;
    let i32_locals = loop_depth_u32
        .checked_add(2)
        .ok_or_else(|| WasmJitError::Encoding("assignment local count overflow".into()))?;
    let scratch_f64_local = 1_u32
        .checked_add(i32_locals)
        .ok_or_else(|| WasmJitError::Encoding("assignment local index overflow".into()))?;
    let mut body = Function::new([(i32_locals, ValType::I32), (1, ValType::F64)]);
    emit_frame_guard(&mut body);
    emit_clear_error_status(&mut body);
    emit_assignments(
        &mut body,
        &kernel.assignments,
        scalar_count,
        0,
        scratch_f64_local,
    )?;
    body.instruction(&WasmInstruction::I32Const(WASM_JIT_STATUS_OK));
    body.instruction(&WasmInstruction::End);
    Ok(body)
}

/// Locals used by a fused kernel body. Local 0 is the frame-offset parameter,
/// so declared locals begin at 1 and the f64 follows the i32s. The status slot
/// deliberately aliases [`ASSIGNMENT_STATUS_LOCAL`]: both hold a callee's
/// return status only until the branch that inspects it.
const KERNEL_STATUS_LOCAL: u32 = 1;
const KERNEL_VALUE_LOCAL: u32 = 2;
const KERNEL_I32_LOCAL_COUNT: u32 = 1;

/// Encode the driver that evaluates a whole model in one call.
///
/// The shape mirrors the native fused kernels exactly: run the assignment
/// pass, then for each stamp skip it when the instance deactivated it,
/// evaluate its value, reject a non-finite result, publish the value into the
/// sequential contribution array and the terminal-pair matrix, and -- when the
/// driver carries Jacobians -- evaluate and publish each Jacobian entry.
fn encode_fused_kernel(
    kernel: &WasmFusedKernel,
    scalar_count: u32,
    kernel_count: u32,
) -> WasmJitResult<Function> {
    if kernel.assignment_kernel >= kernel_count {
        return Err(WasmJitError::Encoding(format!(
            "fused kernel references assignment kernel {} outside count {kernel_count}",
            kernel.assignment_kernel
        )));
    }
    let mut body = Function::new([(KERNEL_I32_LOCAL_COUNT, ValType::I32), (1, ValType::F64)]);
    debug_assert_eq!(KERNEL_VALUE_LOCAL, 1 + KERNEL_I32_LOCAL_COUNT);
    emit_frame_guard(&mut body);
    emit_clear_error_status(&mut body);

    // The assignment kernels follow every scalar entry in the function index
    // space, so the driver calls its own sibling rather than duplicating the
    // assignment lowering.
    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
    body.instruction(&WasmInstruction::Call(
        ENTRY_FUNCTION_INDEX + scalar_count + kernel.assignment_kernel,
    ));
    body.instruction(&WasmInstruction::LocalTee(KERNEL_STATUS_LOCAL));
    body.instruction(&WasmInstruction::If(BlockType::Empty));
    body.instruction(&WasmInstruction::LocalGet(KERNEL_STATUS_LOCAL));
    body.instruction(&WasmInstruction::Return);
    body.instruction(&WasmInstruction::End);

    // The CFG route's assignment pass, once, before the first stamp: every
    // value entry below is a read of a slot it publishes.
    if let Some(prelude) = kernel.prelude_entry {
        emit_value_entry_call(&mut body, prelude, scalar_count)?;
    }

    for (stamp_index, stamp) in kernel.stamps.iter().enumerate() {
        let stamp_index = u32::try_from(stamp_index)
            .map_err(|_| WasmJitError::Encoding("fused kernel stamp index exceeds u32".into()))?;

        // `block ... br_if 0` is the structured equivalent of the native
        // backends' forward jump over an inactive stamp.
        body.instruction(&WasmInstruction::Block(BlockType::Empty));
        emit_program_active_load(&mut body, stamp_index);
        body.instruction(&WasmInstruction::I32Eqz);
        body.instruction(&WasmInstruction::BrIf(0));

        emit_value_entry_call(&mut body, stamp.value_entry, scalar_count)?;
        body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
        body.instruction(&WasmInstruction::F64Load(f64_mem(FRAME_RESULT_OFFSET)));
        body.instruction(&WasmInstruction::LocalTee(KERNEL_VALUE_LOCAL));
        emit_non_finite_guard(&mut body);

        // The sequential contribution array the model reads back through
        // `I(...)` probes, then the terminal-pair matrix.
        const SEQUENTIAL_CURRENTS: (u64, u64) = (
            FRAME_PRIOR_CURRENTS_PTR_OFFSET,
            FRAME_PRIOR_CURRENTS_LEN_OFFSET,
        );
        const PAIR_CURRENTS: (u64, u64) = (FRAME_CURRENTS_PTR_OFFSET, FRAME_CURRENTS_LEN_OFFSET);
        const JACOBIANS: (u64, u64) = (FRAME_JACOBIANS_PTR_OFFSET, FRAME_JACOBIANS_LEN_OFFSET);

        emit_f64_array_store(&mut body, SEQUENTIAL_CURRENTS, stamp_index, &|body| {
            body.instruction(&WasmInstruction::LocalGet(KERNEL_VALUE_LOCAL));
        });
        if let Some((forward, reverse)) = stamp.current_pair {
            emit_f64_array_store(&mut body, PAIR_CURRENTS, forward, &|body| {
                body.instruction(&WasmInstruction::LocalGet(KERNEL_VALUE_LOCAL));
            });
            if forward != reverse {
                emit_f64_array_store(&mut body, PAIR_CURRENTS, reverse, &|body| {
                    body.instruction(&WasmInstruction::LocalGet(KERNEL_VALUE_LOCAL));
                    body.instruction(&WasmInstruction::F64Neg);
                });
            }
        }

        if kernel.with_jacobians {
            for (output, entry) in stamp.jacobian_entries.iter().copied().enumerate() {
                let output = u32::try_from(output)
                    .ok()
                    .and_then(|output| stamp.jacobian_output_base.checked_add(output))
                    .ok_or_else(|| {
                        WasmJitError::Encoding("fused kernel Jacobian output index overflow".into())
                    })?;
                emit_value_entry_call(&mut body, entry, scalar_count)?;
                emit_f64_array_store(&mut body, JACOBIANS, output, &|body| {
                    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
                    body.instruction(&WasmInstruction::F64Load(f64_mem(FRAME_RESULT_OFFSET)));
                });
            }
        }

        body.instruction(&WasmInstruction::End);
    }

    body.instruction(&WasmInstruction::I32Const(WASM_JIT_STATUS_OK));
    body.instruction(&WasmInstruction::End);
    Ok(body)
}

/// Push `program_active[index]` as an i32, or zero when the array is too short.
///
/// A short array deactivates the stamp rather than reading out of bounds: the
/// device always sizes it to the stamp count, and failing closed keeps a
/// malformed frame from publishing an unevaluated contribution.
fn emit_program_active_load(body: &mut Function, index: u32) {
    // A branch rather than a `select`, because `select` evaluates both arms:
    // the byte load has to stay inside the range check or a malformed frame
    // traps the whole module instead of deactivating one stamp.
    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
    body.instruction(&WasmInstruction::I32Load(i32_mem(
        FRAME_PROGRAM_ACTIVE_LEN_OFFSET,
    )));
    body.instruction(&WasmInstruction::I32Const(index as i32));
    body.instruction(&WasmInstruction::I32GtU);
    body.instruction(&WasmInstruction::If(BlockType::Result(ValType::I32)));
    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
    body.instruction(&WasmInstruction::I32Load(i32_mem(
        FRAME_PROGRAM_ACTIVE_PTR_OFFSET,
    )));
    body.instruction(&WasmInstruction::I32Const(index as i32));
    body.instruction(&WasmInstruction::I32Add);
    body.instruction(&WasmInstruction::I32Load8U(MemArg {
        offset: 0,
        align: 0,
        memory_index: 0,
    }));
    body.instruction(&WasmInstruction::Else);
    body.instruction(&WasmInstruction::I32Const(0));
    body.instruction(&WasmInstruction::End);
}

/// Reject a non-finite contribution, matching the native kernels' guard.
///
/// Consumes the value from the stack. `!(|v| < inf)` is true for both NaN and
/// either infinity, which is exactly the exponent-mask test the x64 driver
/// emits.
fn emit_non_finite_guard(body: &mut Function) {
    body.instruction(&WasmInstruction::F64Abs);
    body.instruction(&WasmInstruction::F64Const(f64::INFINITY.into()));
    body.instruction(&WasmInstruction::F64Lt);
    body.instruction(&WasmInstruction::I32Eqz);
    body.instruction(&WasmInstruction::If(BlockType::Empty));
    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
    body.instruction(&WasmInstruction::I32Const(WASM_JIT_STATUS_RUNTIME_ERROR));
    body.instruction(&WasmInstruction::I32Store(i32_mem(
        FRAME_ERROR_STATUS_OFFSET,
    )));
    body.instruction(&WasmInstruction::I32Const(WASM_JIT_STATUS_RUNTIME_ERROR));
    body.instruction(&WasmInstruction::Return);
    body.instruction(&WasmInstruction::End);
}

/// Store an f64 into a frame-addressed array, bounds-checked against the
/// array's declared length.
///
/// An out-of-range index fails the dispatch instead of writing, and instead of
/// being dropped: the device sizes every one of these arrays from the compiled
/// model before dispatching, so a short array means the frame disagrees with
/// the module about the model's shape. Dropping the write would leave the
/// caller reading a stale zero and stamping it as a real contribution or
/// derivative, which is a wrong answer rather than a reported failure.
fn emit_f64_array_store(
    body: &mut Function,
    array: (u64, u64),
    index: u32,
    value: &dyn Fn(&mut Function),
) {
    let (pointer_offset, length_offset) = array;
    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
    body.instruction(&WasmInstruction::I32Load(i32_mem(length_offset)));
    body.instruction(&WasmInstruction::I32Const(index as i32));
    body.instruction(&WasmInstruction::I32LeU);
    body.instruction(&WasmInstruction::If(BlockType::Empty));
    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
    body.instruction(&WasmInstruction::I32Const(WASM_JIT_STATUS_RUNTIME_ERROR));
    body.instruction(&WasmInstruction::I32Store(i32_mem(
        FRAME_ERROR_STATUS_OFFSET,
    )));
    body.instruction(&WasmInstruction::I32Const(WASM_JIT_STATUS_RUNTIME_ERROR));
    body.instruction(&WasmInstruction::Return);
    body.instruction(&WasmInstruction::End);

    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
    body.instruction(&WasmInstruction::I32Load(i32_mem(pointer_offset)));
    body.instruction(&WasmInstruction::I32Const((index as i32).wrapping_mul(8)));
    body.instruction(&WasmInstruction::I32Add);
    value(body);
    body.instruction(&WasmInstruction::F64Store(f64_mem(0)));
}

fn assignment_loop_depth(assignments: &[WasmAssignment]) -> WasmJitResult<usize> {
    assignments.iter().try_fold(0_usize, |maximum, assignment| {
        let depth = match assignment {
            WasmAssignment::Direct { .. } | WasmAssignment::Indexed { .. } => 0,
            WasmAssignment::Loop { body, .. } => assignment_loop_depth(body)?
                .checked_add(1)
                .ok_or_else(|| WasmJitError::Encoding("assignment loop depth overflow".into()))?,
        };
        Ok(maximum.max(depth))
    })
}

fn emit_assignments(
    body: &mut Function,
    assignments: &[WasmAssignment],
    scalar_count: u32,
    loop_depth: u32,
    scratch_f64_local: u32,
) -> WasmJitResult<()> {
    for assignment in assignments {
        match assignment {
            WasmAssignment::Direct {
                variable_index,
                value_entry,
            } => {
                emit_variable_range_guard(body, *variable_index, 1)?;
                emit_value_entry_call(body, *value_entry, scalar_count)?;
                body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
                body.instruction(&WasmInstruction::I32Load(i32_mem(
                    FRAME_VARIABLES_PTR_OFFSET,
                )));
                body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
                body.instruction(&WasmInstruction::F64Load(f64_mem(FRAME_RESULT_OFFSET)));
                body.instruction(&WasmInstruction::F64Store(f64_mem(
                    u64::from(*variable_index) * 8,
                )));
            }
            WasmAssignment::Indexed {
                base,
                len,
                lower,
                index_entry,
                value_entry,
            } => {
                emit_variable_range_guard(body, *base, *len)?;
                emit_value_entry_call(body, *index_entry, scalar_count)?;
                body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
                body.instruction(&WasmInstruction::I32Const(2));
                body.instruction(&WasmInstruction::I32Const(*base as i32));
                body.instruction(&WasmInstruction::I32Const(*len as i32));
                body.instruction(&WasmInstruction::I64Const(*lower));
                body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
                body.instruction(&WasmInstruction::F64Load(f64_mem(FRAME_RESULT_OFFSET)));
                for _ in 0..4 {
                    body.instruction(&WasmInstruction::F64Const(0.0.into()));
                }
                body.instruction(&WasmInstruction::Call(HELPER_FUNCTION_INDEX));
                body.instruction(&WasmInstruction::LocalSet(scratch_f64_local));
                emit_return_existing_error(body);
                body.instruction(&WasmInstruction::LocalGet(scratch_f64_local));
                body.instruction(&WasmInstruction::I32TruncSatF64U);
                body.instruction(&WasmInstruction::LocalSet(ASSIGNMENT_SLOT_LOCAL));

                emit_value_entry_call(body, *value_entry, scalar_count)?;
                body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
                body.instruction(&WasmInstruction::I32Load(i32_mem(
                    FRAME_VARIABLES_PTR_OFFSET,
                )));
                body.instruction(&WasmInstruction::LocalGet(ASSIGNMENT_SLOT_LOCAL));
                body.instruction(&WasmInstruction::I32Const(8));
                body.instruction(&WasmInstruction::I32Mul);
                body.instruction(&WasmInstruction::I32Add);
                body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
                body.instruction(&WasmInstruction::F64Load(f64_mem(FRAME_RESULT_OFFSET)));
                body.instruction(&WasmInstruction::F64Store(f64_mem(0)));
            }
            WasmAssignment::Loop {
                condition_entry,
                body: loop_body,
            } => {
                let counter_local = ASSIGNMENT_LOOP_COUNTER_BASE_LOCAL
                    .checked_add(loop_depth)
                    .ok_or_else(|| {
                        WasmJitError::Encoding("assignment loop local overflow".into())
                    })?;
                body.instruction(&WasmInstruction::I32Const(0));
                body.instruction(&WasmInstruction::LocalSet(counter_local));
                body.instruction(&WasmInstruction::Block(BlockType::Empty));
                body.instruction(&WasmInstruction::Loop(BlockType::Empty));
                emit_value_entry_call(body, *condition_entry, scalar_count)?;
                body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
                body.instruction(&WasmInstruction::F64Load(f64_mem(FRAME_RESULT_OFFSET)));
                body.instruction(&WasmInstruction::F64Const(0.0.into()));
                body.instruction(&WasmInstruction::F64Eq);
                body.instruction(&WasmInstruction::BrIf(1));
                emit_assignments(
                    body,
                    loop_body,
                    scalar_count,
                    loop_depth.checked_add(1).ok_or_else(|| {
                        WasmJitError::Encoding("assignment loop depth overflow".into())
                    })?,
                    scratch_f64_local,
                )?;
                body.instruction(&WasmInstruction::LocalGet(counter_local));
                body.instruction(&WasmInstruction::I32Const(1));
                body.instruction(&WasmInstruction::I32Add);
                body.instruction(&WasmInstruction::LocalTee(counter_local));
                body.instruction(&WasmInstruction::I32Const(MAX_RUNTIME_LOOP_ITERATIONS));
                body.instruction(&WasmInstruction::I32GeU);
                body.instruction(&WasmInstruction::If(BlockType::Empty));
                emit_status_return(body, WASM_JIT_STATUS_RUNTIME_ERROR);
                body.instruction(&WasmInstruction::End);
                body.instruction(&WasmInstruction::Br(0));
                body.instruction(&WasmInstruction::End);
                body.instruction(&WasmInstruction::End);
            }
        }
    }
    Ok(())
}

const ASSIGNMENT_STATUS_LOCAL: u32 = 1;
const ASSIGNMENT_SLOT_LOCAL: u32 = 2;
const ASSIGNMENT_LOOP_COUNTER_BASE_LOCAL: u32 = 3;

fn emit_value_entry_call(body: &mut Function, entry: u32, scalar_count: u32) -> WasmJitResult<()> {
    if entry >= scalar_count {
        return Err(WasmJitError::Encoding(format!(
            "assignment references scalar entry {entry} outside count {scalar_count}"
        )));
    }
    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
    body.instruction(&WasmInstruction::Call(ENTRY_FUNCTION_INDEX + entry));
    body.instruction(&WasmInstruction::LocalTee(ASSIGNMENT_STATUS_LOCAL));
    body.instruction(&WasmInstruction::If(BlockType::Empty));
    body.instruction(&WasmInstruction::LocalGet(ASSIGNMENT_STATUS_LOCAL));
    body.instruction(&WasmInstruction::Return);
    body.instruction(&WasmInstruction::End);
    Ok(())
}

fn emit_variable_range_guard(body: &mut Function, base: u32, len: u32) -> WasmJitResult<()> {
    if len == 0 {
        return Err(WasmJitError::Encoding(
            "indexed assignment has zero-length storage".into(),
        ));
    }
    let required = base
        .checked_add(len)
        .ok_or_else(|| WasmJitError::Encoding("assignment variable range overflow".into()))?;
    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
    body.instruction(&WasmInstruction::I32Load(i32_mem(
        FRAME_VARIABLES_LEN_OFFSET,
    )));
    body.instruction(&WasmInstruction::I32Const(required as i32));
    body.instruction(&WasmInstruction::I32LtU);
    body.instruction(&WasmInstruction::If(BlockType::Empty));
    emit_status_return(body, WASM_JIT_STATUS_RUNTIME_ERROR);
    body.instruction(&WasmInstruction::End);
    Ok(())
}

fn emit_return_existing_error(body: &mut Function) {
    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
    body.instruction(&WasmInstruction::I32Load(i32_mem(
        FRAME_ERROR_STATUS_OFFSET,
    )));
    body.instruction(&WasmInstruction::If(BlockType::Empty));
    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
    body.instruction(&WasmInstruction::I32Load(i32_mem(
        FRAME_ERROR_STATUS_OFFSET,
    )));
    body.instruction(&WasmInstruction::Return);
    body.instruction(&WasmInstruction::End);
}

#[cfg(test)]
pub(crate) fn emit_verified_value_program(program: &NativeProgram) -> WasmJitResult<Vec<u8>> {
    let bytes = encode_value_program(program)?;
    verify_value_program(&bytes, program)?;
    Ok(bytes)
}

#[cfg(test)]
pub(crate) fn verify_value_program(
    bytes: &[u8],
    expected_program: &NativeProgram,
) -> WasmJitResult<()> {
    if bytes.len() > super::SHIPPED_MODEL_WASM_CODE_SIZE_BUDGET_BYTES {
        return Err(WasmJitError::ArtifactTooLarge {
            actual: bytes.len(),
            limit: super::SHIPPED_MODEL_WASM_CODE_SIZE_BUDGET_BYTES,
        });
    }
    Validator::new()
        .validate_all(bytes)
        .map_err(|error| WasmJitError::BinaryValidation(error.to_string()))?;

    let expected = encode_value_program(expected_program)?;
    if bytes != expected {
        return Err(WasmJitError::Contract(
            "value module does not match deterministic translation of its authenticated SSA".into(),
        ));
    }

    verify_value_module_shape(bytes, 1, &[], &[], true)
}

fn verify_value_module_shape(
    bytes: &[u8],
    expected_scalar_count: u32,
    assignment_exports: &[&str],
    fused_exports: &[&str],
    single_export: bool,
) -> WasmJitResult<()> {
    let expected_kernel_count = u32::try_from(assignment_exports.len())
        .map_err(|_| WasmJitError::Contract("assignment kernel count exceeds u32".into()))?;
    let expected_fused_count = u32::try_from(fused_exports.len())
        .map_err(|_| WasmJitError::Contract("fused kernel count exceeds u32".into()))?;
    let kernel_exports = assignment_exports
        .iter()
        .chain(fused_exports)
        .copied()
        .collect::<Vec<_>>();
    let expected_count = expected_scalar_count
        .checked_add(expected_kernel_count)
        .and_then(|count| count.checked_add(expected_fused_count))
        .ok_or_else(|| WasmJitError::Contract("model function count overflow".into()))?;
    let mut version = 0_u32;
    let mut types = 0_u32;
    let mut imports = 0_u32;
    let mut functions = 0_u32;
    let mut exports = 0_u32;
    let mut contracts = 0_u32;
    let mut code_starts = 0_u32;
    let mut code_bodies = 0_u32;
    for payload in Parser::new(0).parse_all(bytes) {
        match payload.map_err(|error| WasmJitError::BinaryValidation(error.to_string()))? {
            Payload::Version { encoding, .. } => {
                version += 1;
                if encoding != Encoding::Module {
                    return Err(WasmJitError::Contract(
                        "component encoding is forbidden".into(),
                    ));
                }
            }
            Payload::TypeSection(reader) => {
                types += reader.count();
                let entries = reader
                    .into_iter_err_on_gc_types()
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|error| WasmJitError::Contract(error.to_string()))?;
                const I32: wasmparser::ValType = wasmparser::ValType::I32;
                const I64: wasmparser::ValType = wasmparser::ValType::I64;
                const F64: wasmparser::ValType = wasmparser::ValType::F64;
                let expected: [(&[wasmparser::ValType], &[wasmparser::ValType]); 5] = [
                    (&[I32], &[I32]),
                    (&[I32, I32, I32, I32, I64, F64, F64, F64, F64, F64], &[F64]),
                    (&[I32, F64], &[F64]),
                    (&[I32, F64, F64], &[F64]),
                    (&[I32, I32, I32, I32, I64, I32], &[F64]),
                ];
                if entries.len() != expected.len()
                    || entries
                        .iter()
                        .zip(expected)
                        .any(|(entry, (params, results))| {
                            entry.params() != params || entry.results() != results
                        })
                {
                    return Err(WasmJitError::Contract(format!(
                        "value-module function signatures do not match ABI v{WASM_JIT_ABI_VERSION}"
                    )));
                }
            }
            Payload::ImportSection(reader) => {
                imports += reader.count();
                let mut flattened = Vec::new();
                for group in reader {
                    let group = group.map_err(|error| WasmJitError::Contract(error.to_string()))?;
                    let wasmparser::Imports::Single(_, import) = group else {
                        return Err(WasmJitError::Contract(
                            "compact imports are forbidden".into(),
                        ));
                    };
                    flattened.push(import);
                }
                if flattened.len() != 5 {
                    return Err(WasmJitError::Contract(
                        "value module must import exactly memory, eval_op_v1, math1_v1, math2_v1, and eval_op_slice_v1"
                            .into(),
                    ));
                }
                let memory = &flattened[0];
                if memory.module != WASM_JIT_IMPORT_MODULE || memory.name != WASM_JIT_MEMORY_IMPORT
                {
                    return Err(WasmJitError::Contract(
                        "unexpected value-module memory capability".into(),
                    ));
                }
                let wasmparser::TypeRef::Memory(memory_type) = memory.ty else {
                    return Err(WasmJitError::Contract(
                        "first value-module import must be memory".into(),
                    ));
                };
                if memory_type.memory64
                    || memory_type.shared
                    || memory_type.initial != 0
                    || memory_type.maximum.is_some()
                    || memory_type.page_size_log2.is_some()
                {
                    return Err(WasmJitError::Contract(
                        "value-module memory import has forbidden limits or flags".into(),
                    ));
                }
                // Order is part of the contract: the browser worker binds by
                // position-independent name, but the emitted call sites address
                // imported functions by index.
                for (import, name, type_index) in [
                    (
                        &flattened[1],
                        WASM_JIT_EVAL_HELPER_IMPORT,
                        HELPER_TYPE_INDEX,
                    ),
                    (&flattened[2], WASM_JIT_MATH1_IMPORT, MATH1_TYPE_INDEX),
                    (&flattened[3], WASM_JIT_MATH2_IMPORT, MATH2_TYPE_INDEX),
                    (
                        &flattened[4],
                        WASM_JIT_SLICE_HELPER_IMPORT,
                        SLICE_HELPER_TYPE_INDEX,
                    ),
                ] {
                    if import.module != WASM_JIT_IMPORT_MODULE
                        || import.name != name
                        || import.ty != wasmparser::TypeRef::Func(type_index)
                    {
                        return Err(WasmJitError::Contract(format!(
                            "unexpected value-module capability at the {name} import slot"
                        )));
                    }
                }
            }
            Payload::FunctionSection(reader) => {
                functions += reader.count();
                for type_index in reader {
                    if type_index.map_err(|error| WasmJitError::Contract(error.to_string()))?
                        != ENTRY_TYPE_INDEX
                    {
                        return Err(WasmJitError::Contract(
                            "value entry must use ABI entry type zero".into(),
                        ));
                    }
                }
            }
            Payload::ExportSection(reader) => {
                exports += reader.count();
                for (index, export) in reader.into_iter().enumerate() {
                    let export =
                        export.map_err(|error| WasmJitError::Contract(error.to_string()))?;
                    let index = u32::try_from(index)
                        .map_err(|_| WasmJitError::Contract("export index overflow".into()))?;
                    let expected_name = if single_export {
                        WASM_JIT_VALUE_EXPORT.to_owned()
                    } else if index < expected_scalar_count {
                        format!("rspice_wasm_jit_value_{index:08x}")
                    } else {
                        let kernel_index =
                            usize::try_from(index - expected_scalar_count).map_err(|_| {
                                WasmJitError::Contract("kernel export index overflow".into())
                            })?;
                        kernel_exports
                            .get(kernel_index)
                            .ok_or_else(|| {
                                WasmJitError::Contract("unexpected extra model export".into())
                            })?
                            .to_string()
                    };
                    if export.name != expected_name
                        || export.kind != wasmparser::ExternalKind::Func
                        || export.index != ENTRY_FUNCTION_INDEX + index
                    {
                        return Err(WasmJitError::Contract(format!(
                            "value-module export {index} does not match its deterministic ABI entry"
                        )));
                    }
                }
            }
            Payload::CustomSection(section) => {
                if section.name() != super::WASM_JIT_CONTRACT_SECTION {
                    return Err(WasmJitError::Contract(
                        "unknown value-module custom section".into(),
                    ));
                }
                contracts += 1;
                let mut expected = Vec::with_capacity(if single_export { 12 } else { 24 });
                expected.extend_from_slice(&WASM_JIT_ABI_VERSION.to_le_bytes());
                expected.extend_from_slice(&WASM_JIT_EMITTER_VERSION.to_le_bytes());
                expected.extend_from_slice(&WASM_JIT_EVAL_FRAME_BYTES.to_le_bytes());
                if !single_export {
                    expected.extend_from_slice(&expected_scalar_count.to_le_bytes());
                    expected.extend_from_slice(&expected_kernel_count.to_le_bytes());
                    expected.extend_from_slice(&expected_fused_count.to_le_bytes());
                }
                if section.data() != expected {
                    return Err(WasmJitError::Contract(
                        "value-module contract payload mismatch".into(),
                    ));
                }
            }
            Payload::CodeSectionStart { count, .. } => {
                code_starts += 1;
                if count != expected_count {
                    return Err(WasmJitError::Contract(
                        "value-module code count does not match its manifest".into(),
                    ));
                }
            }
            Payload::CodeSectionEntry(_) => code_bodies += 1,
            Payload::End(_) => {}
            _ => {
                return Err(WasmJitError::Contract(
                    "value module contains a forbidden section".into(),
                ));
            }
        }
    }
    if (
        version,
        types,
        imports,
        functions,
        exports,
        contracts,
        code_starts,
        code_bodies,
    ) != (
        1,
        CAPABILITY_TYPE_COUNT,
        CAPABILITY_IMPORT_COUNT,
        expected_count,
        expected_count,
        1,
        1,
        expected_count,
    ) {
        return Err(WasmJitError::Contract(format!(
            "value-module section shape mismatch: version={version} types={types} imports={imports} functions={functions} exports={exports} contracts={contracts} code_starts={code_starts} code_bodies={code_bodies}"
        )));
    }
    Ok(())
}

fn emit_frame_guard(body: &mut Function) {
    for (offset, expected, comparison) in [
        (
            FRAME_MAGIC_OFFSET,
            WASM_JIT_FRAME_MAGIC,
            GuardComparison::Equal,
        ),
        (
            FRAME_ABI_VERSION_OFFSET,
            WASM_JIT_ABI_VERSION,
            GuardComparison::Equal,
        ),
        (
            FRAME_BYTE_LEN_OFFSET,
            WASM_JIT_EVAL_FRAME_BYTES,
            GuardComparison::AtLeast,
        ),
    ] {
        body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
        body.instruction(&WasmInstruction::I32Load(i32_mem(offset)));
        body.instruction(&WasmInstruction::I32Const(expected as i32));
        body.instruction(&match comparison {
            GuardComparison::Equal => WasmInstruction::I32Ne,
            GuardComparison::AtLeast => WasmInstruction::I32LtU,
        });
        body.instruction(&WasmInstruction::If(BlockType::Empty));
        emit_status_return(body, WASM_JIT_STATUS_ABI_MISMATCH);
        body.instruction(&WasmInstruction::End);
    }
}

#[derive(Clone, Copy)]
enum GuardComparison {
    Equal,
    AtLeast,
}

fn emit_instruction(body: &mut Function, instruction: &Instruction) -> WasmJitResult<()> {
    let op = instruction.op();
    let operands = instruction.operands();
    match op {
        NativeOp::Const(value) => {
            body.instruction(&WasmInstruction::F64Const(value.into()));
        }
        NativeOp::LoadParam(index) => emit_f64_array_load(
            body,
            FRAME_PARAMETERS_PTR_OFFSET,
            FRAME_PARAMETERS_LEN_OFFSET,
            index,
        )?,
        NativeOp::LoadParamGiven(index) => emit_u8_array_load(
            body,
            FRAME_PARAMETER_GIVEN_PTR_OFFSET,
            FRAME_PARAMETER_GIVEN_LEN_OFFSET,
            index,
        )?,
        NativeOp::LoadPortConnected(index) => emit_u8_array_load(
            body,
            FRAME_PORT_CONNECTED_PTR_OFFSET,
            FRAME_PORT_CONNECTED_LEN_OFFSET,
            index,
        )?,
        NativeOp::LoadVoltage { pos, neg } => {
            emit_voltage_node(body, pos)?;
            emit_voltage_node(body, neg)?;
            body.instruction(&WasmInstruction::F64Sub);
        }
        NativeOp::LoadCurrent(index) => emit_f64_array_load(
            body,
            FRAME_CURRENTS_PTR_OFFSET,
            FRAME_CURRENTS_LEN_OFFSET,
            index,
        )?,
        NativeOp::LoadPriorCurrent(index) => emit_f64_array_load(
            body,
            FRAME_PRIOR_CURRENTS_PTR_OFFSET,
            FRAME_PRIOR_CURRENTS_LEN_OFFSET,
            index,
        )?,
        NativeOp::LoadInternalVoltage(index) => emit_f64_array_load(
            body,
            FRAME_INTERNAL_VOLTAGES_PTR_OFFSET,
            FRAME_INTERNAL_VOLTAGES_LEN_OFFSET,
            index,
        )?,
        NativeOp::LoadVariable(index) => emit_f64_array_load(
            body,
            FRAME_VARIABLES_PTR_OFFSET,
            FRAME_VARIABLES_LEN_OFFSET,
            index,
        )?,
        NativeOp::LoadBranchUnknown(index) => emit_f64_array_load(
            body,
            FRAME_BRANCH_UNKNOWNS_PTR_OFFSET,
            FRAME_BRANCH_UNKNOWNS_LEN_OFFSET,
            index,
        )?,
        NativeOp::LoadTemperature => emit_frame_f64_load(body, FRAME_TEMPERATURE_OFFSET),
        NativeOp::LoadThermalVoltage => emit_frame_f64_load(body, FRAME_THERMAL_VOLTAGE_OFFSET),
        NativeOp::LoadTime => emit_frame_f64_load(body, FRAME_TIME_OFFSET),
        NativeOp::LoadMfactor => emit_frame_f64_load(body, FRAME_M_FACTOR_OFFSET),
        NativeOp::LoadPreludeSlot(index) => emit_f64_array_load(
            body,
            FRAME_PRELUDE_SLOTS_PTR_OFFSET,
            FRAME_PRELUDE_SLOTS_LEN_OFFSET,
            index,
        )?,
        // An identity on its operand, so the result it leaves on the stack is
        // a second read of the operand's own local rather than anything new.
        // Every SSA value already owns a local here, which is what lets the
        // store take its address and its value in the order WebAssembly wants
        // without a scratch local of its own.
        NativeOp::StorePreludeSlot(index) => {
            let operand = operands.first().copied().ok_or_else(|| {
                WasmJitError::Encoding("prelude slot store has no operand".into())
            })?;
            emit_bounds_guard(body, FRAME_PRELUDE_SLOTS_LEN_OFFSET, index)?;
            body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
            body.instruction(&WasmInstruction::I32Load(i32_mem(
                FRAME_PRELUDE_SLOTS_PTR_OFFSET,
            )));
            body.instruction(&WasmInstruction::LocalGet(value_local(operand.index())?));
            body.instruction(&WasmInstruction::F64Store(f64_mem(element_offset(
                index, 8,
            )?)));
            body.instruction(&WasmInstruction::LocalGet(value_local(operand.index())?));
        }
        NativeOp::Analysis(analysis_id) => {
            let mask = 1_u32.checked_shl(u32::from(analysis_id)).ok_or_else(|| {
                WasmJitError::Encoding(format!(
                    "analysis id {analysis_id} exceeds the evaluation-frame mask"
                ))
            })?;
            body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
            body.instruction(&WasmInstruction::I32Load(i32_mem(
                FRAME_ANALYSIS_MASK_OFFSET,
            )));
            body.instruction(&WasmInstruction::I32Const(mask as i32));
            body.instruction(&WasmInstruction::I32And);
            body.instruction(&WasmInstruction::I32Const(0));
            body.instruction(&WasmInstruction::I32Ne);
            body.instruction(&WasmInstruction::F64ConvertI32U);
        }
        NativeOp::Add => emit_binary(body, operands, WasmInstruction::F64Add)?,
        NativeOp::Sub => emit_binary(body, operands, WasmInstruction::F64Sub)?,
        NativeOp::Mul => emit_binary(body, operands, WasmInstruction::F64Mul)?,
        NativeOp::Div => emit_binary(body, operands, WasmInstruction::F64Div)?,
        NativeOp::AddConst(value) => {
            emit_rhs_const(body, operands, value, WasmInstruction::F64Add)?
        }
        NativeOp::SubConst(value) => {
            emit_rhs_const(body, operands, value, WasmInstruction::F64Sub)?
        }
        NativeOp::MulConst(value) => {
            emit_rhs_const(body, operands, value, WasmInstruction::F64Mul)?
        }
        NativeOp::DivConst(value) => {
            emit_rhs_const(body, operands, value, WasmInstruction::F64Div)?
        }
        NativeOp::SubFromConst(value) => {
            emit_lhs_const(body, operands, value, WasmInstruction::F64Sub)?
        }
        NativeOp::DivFromConst(value) => {
            emit_lhs_const(body, operands, value, WasmInstruction::F64Div)?
        }
        NativeOp::Neg => {
            emit_operand(body, operands, 0)?;
            body.instruction(&WasmInstruction::F64Neg);
        }
        NativeOp::Abs => {
            emit_operand(body, operands, 0)?;
            body.instruction(&WasmInstruction::F64Abs);
        }
        NativeOp::Square => {
            emit_operand(body, operands, 0)?;
            emit_operand(body, operands, 0)?;
            body.instruction(&WasmInstruction::F64Mul);
        }
        NativeOp::Sqrt => {
            emit_operand(body, operands, 0)?;
            body.instruction(&WasmInstruction::F64Sqrt);
        }
        NativeOp::Compare(compare) => emit_compare(body, operands, compare, None)?,
        NativeOp::CompareConst(compare, value) => {
            emit_compare(body, operands, compare, Some(value))?
        }
        NativeOp::Logical(logical) => emit_logical(body, operands, logical)?,
        NativeOp::LogicalConst(logical, rhs) => emit_logical_const(body, operands, logical, rhs)?,
        NativeOp::IfElse => {
            emit_operand(body, operands, 1)?;
            emit_operand(body, operands, 2)?;
            emit_truthy(body, operands, 0)?;
            body.instruction(&WasmInstruction::Select);
        }
        NativeOp::UnaryMath(UnaryMathOp::Floor) => {
            emit_operand(body, operands, 0)?;
            body.instruction(&WasmInstruction::F64Floor);
        }
        NativeOp::UnaryMath(UnaryMathOp::Ceil) => {
            emit_operand(body, operands, 0)?;
            body.instruction(&WasmInstruction::F64Ceil);
        }
        NativeOp::Extremum(kind) => emit_extremum(
            body,
            kind,
            &|body| emit_operand(body, operands, 0),
            &|body| emit_operand(body, operands, 1),
        )?,
        NativeOp::ExtremumConst(kind, value) => emit_extremum(
            body,
            kind,
            &|body| emit_operand(body, operands, 0),
            &|body| {
                body.instruction(&WasmInstruction::F64Const(value.into()));
                Ok(())
            },
        )?,
        NativeOp::ExtremumConstLhs(kind, value) => emit_extremum(
            body,
            kind,
            &|body| {
                body.instruction(&WasmInstruction::F64Const(value.into()));
                Ok(())
            },
            &|body| emit_operand(body, operands, 0),
        )?,
        _ => emit_helper_call(body, op, operands, instruction.result())?,
    };
    Ok(())
}

/// Lower `min`/`max` to match the reference `constant_extremum`.
///
/// This cannot be `f64.min`/`f64.max` alone. WebAssembly returns a NaN when
/// either operand is NaN, and returns the negatively-signed zero when the
/// operands are zeros of opposite sign. Rust's `f64::min`/`f64::max` -- which
/// the bytecode VM and both native backends evaluate -- return the non-NaN
/// operand instead, and the x64 backend's NaN/zero fixup settles the
/// equal-magnitude zero case by returning the left operand. Compact models
/// guard divisions with `max(x, 1e-30)` constantly, so this has to agree
/// exactly, not merely on ordinary values.
///
/// Emitted as two `select`s rather than branches: both arms are values that
/// are already available, and the operands are re-read from their locals
/// rather than spilled to scratch.
fn emit_extremum(
    body: &mut Function,
    kind: ExtremumOp,
    left: &dyn Fn(&mut Function) -> WasmJitResult<()>,
    right: &dyn Fn(&mut Function) -> WasmJitResult<()>,
) -> WasmJitResult<()> {
    // Outer select's "condition true" arm: the left operand is NaN, so the
    // result is whatever the right operand is (NaN included).
    right(body)?;

    // Inner select's "condition true" arm: return the left operand.
    left(body)?;

    // Inner select's "condition false" arm: both operands are ordinary, where
    // WebAssembly and the reference agree.
    left(body)?;
    right(body)?;
    body.instruction(&match kind {
        ExtremumOp::Min => WasmInstruction::F64Min,
        ExtremumOp::Max => WasmInstruction::F64Max,
    });

    // Inner condition: the right operand is NaN, or both operands are zero.
    right(body)?;
    right(body)?;
    body.instruction(&WasmInstruction::F64Ne);
    emit_is_zero_magnitude(body, left)?;
    emit_is_zero_magnitude(body, right)?;
    body.instruction(&WasmInstruction::I32And);
    body.instruction(&WasmInstruction::I32Or);
    body.instruction(&WasmInstruction::Select);

    // Outer condition: the left operand is NaN.
    left(body)?;
    left(body)?;
    body.instruction(&WasmInstruction::F64Ne);
    body.instruction(&WasmInstruction::Select);
    Ok(())
}

fn emit_is_zero_magnitude(
    body: &mut Function,
    operand: &dyn Fn(&mut Function) -> WasmJitResult<()>,
) -> WasmJitResult<()> {
    operand(body)?;
    body.instruction(&WasmInstruction::F64Abs);
    body.instruction(&WasmInstruction::F64Const(0.0.into()));
    body.instruction(&WasmInstruction::F64Eq);
    Ok(())
}

fn emit_binary(
    body: &mut Function,
    operands: &[crate::jit::ssa::ValueId],
    op: WasmInstruction<'static>,
) -> WasmJitResult<()> {
    emit_operand(body, operands, 0)?;
    emit_operand(body, operands, 1)?;
    body.instruction(&op);
    Ok(())
}

fn emit_rhs_const(
    body: &mut Function,
    operands: &[crate::jit::ssa::ValueId],
    value: f64,
    op: WasmInstruction<'static>,
) -> WasmJitResult<()> {
    emit_operand(body, operands, 0)?;
    body.instruction(&WasmInstruction::F64Const(value.into()));
    body.instruction(&op);
    Ok(())
}

fn emit_lhs_const(
    body: &mut Function,
    operands: &[crate::jit::ssa::ValueId],
    value: f64,
    op: WasmInstruction<'static>,
) -> WasmJitResult<()> {
    body.instruction(&WasmInstruction::F64Const(value.into()));
    emit_operand(body, operands, 0)?;
    body.instruction(&op);
    Ok(())
}

fn emit_compare(
    body: &mut Function,
    operands: &[crate::jit::ssa::ValueId],
    compare: CompareOp,
    rhs: Option<f64>,
) -> WasmJitResult<()> {
    emit_operand(body, operands, 0)?;
    if let Some(rhs) = rhs {
        body.instruction(&WasmInstruction::F64Const(rhs.into()));
    } else {
        emit_operand(body, operands, 1)?;
    }
    body.instruction(&match compare {
        CompareOp::Gt => WasmInstruction::F64Gt,
        CompareOp::Lt => WasmInstruction::F64Lt,
        CompareOp::Ge => WasmInstruction::F64Ge,
        CompareOp::Le => WasmInstruction::F64Le,
        CompareOp::Eq => WasmInstruction::F64Eq,
        CompareOp::Ne => WasmInstruction::F64Ne,
    });
    body.instruction(&WasmInstruction::F64ConvertI32U);
    Ok(())
}

fn emit_logical(
    body: &mut Function,
    operands: &[crate::jit::ssa::ValueId],
    logical: LogicalOp,
) -> WasmJitResult<()> {
    emit_truthy(body, operands, 0)?;
    match logical {
        LogicalOp::Not => body.instruction(&WasmInstruction::I32Eqz),
        LogicalOp::And | LogicalOp::Or => {
            emit_truthy(body, operands, 1)?;
            body.instruction(&match logical {
                LogicalOp::And => WasmInstruction::I32And,
                LogicalOp::Or => WasmInstruction::I32Or,
                LogicalOp::Not => unreachable!(),
            })
        }
    };
    body.instruction(&WasmInstruction::F64ConvertI32U);
    Ok(())
}

fn emit_logical_const(
    body: &mut Function,
    operands: &[crate::jit::ssa::ValueId],
    logical: LogicalOp,
    rhs: bool,
) -> WasmJitResult<()> {
    emit_truthy(body, operands, 0)?;
    body.instruction(&WasmInstruction::I32Const(i32::from(rhs)));
    body.instruction(&match logical {
        LogicalOp::And => WasmInstruction::I32And,
        LogicalOp::Or => WasmInstruction::I32Or,
        LogicalOp::Not => {
            return Err(WasmJitError::Encoding(
                "logical-constant NOT is not canonical".into(),
            ));
        }
    });
    body.instruction(&WasmInstruction::F64ConvertI32U);
    Ok(())
}

fn emit_truthy(
    body: &mut Function,
    operands: &[crate::jit::ssa::ValueId],
    operand: usize,
) -> WasmJitResult<()> {
    emit_operand(body, operands, operand)?;
    body.instruction(&WasmInstruction::F64Const(0.0.into()));
    body.instruction(&WasmInstruction::F64Ne);
    Ok(())
}

/// Emit a transcendental through the frame-free capability, or report that the
/// operation needs the general descriptor path.
///
/// These operations read no simulator state and cannot publish a runtime
/// error: `constant_unary_math` and `constant_binary_math` are total over the
/// doubles, returning NaN where the function is undefined exactly as the
/// bytecode and native backends do. That is what lets the call site skip the
/// error-status reload and branch that every stateful helper needs.
fn emit_pure_math_call(
    body: &mut Function,
    op: NativeOp,
    operands: &[crate::jit::ssa::ValueId],
) -> WasmJitResult<bool> {
    let (function_index, opcode, arity) = match op {
        NativeOp::UnaryMath(kind) => (MATH1_FUNCTION_INDEX, 100 + unary_math_code(kind), 1),
        NativeOp::BinaryMath(kind) => (MATH2_FUNCTION_INDEX, 200 + binary_math_code(kind), 2),
        _ => return Ok(false),
    };
    if operands.len() < arity {
        return Err(WasmJitError::Encoding(format!(
            "pure math operation expects {arity} operand(s), lowering supplied {}",
            operands.len()
        )));
    }
    body.instruction(&WasmInstruction::I32Const(opcode));
    for index in 0..arity {
        emit_operand(body, operands, index)?;
    }
    body.instruction(&WasmInstruction::Call(function_index));
    Ok(true)
}

fn emit_helper_call(
    body: &mut Function,
    op: NativeOp,
    operands: &[crate::jit::ssa::ValueId],
    result: crate::jit::ssa::ValueId,
) -> WasmJitResult<()> {
    // The pure-math path has no early-return branch to navigate, so the result
    // stays on the stack for the caller's own `local.set` rather than making a
    // round trip through a local.
    if emit_pure_math_call(body, op, operands)? {
        return Ok(());
    }

    let descriptor = helper_descriptor(op)?;
    if let NativeOp::ZiState(layout) | NativeOp::ZiStateDerivative(layout) = op {
        let operand_count = layout.validate_operand_budget().map_err(|error| {
            WasmJitError::Encoding(format!("Zi runtime layout rejected: {error}"))
        })?;
        if operands.len() != operand_count {
            return Err(WasmJitError::Encoding(format!(
                "Zi lowering supplied {} operands for a layout requiring {}",
                operands.len(),
                operand_count
            )));
        }
        return emit_slice_helper_call(body, descriptor, operands, result);
    }
    if matches!(op, NativeOp::SlewStateDerivative(_)) {
        if operands.len() != 6 {
            return Err(WasmJitError::Encoding(format!(
                "slew derivative lowering supplied {} operands; exactly 6 are required",
                operands.len()
            )));
        }
        return emit_slice_helper_call(body, descriptor, operands, result);
    }
    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
    body.instruction(&WasmInstruction::I32Const(descriptor.opcode));
    body.instruction(&WasmInstruction::I32Const(descriptor.aux0));
    body.instruction(&WasmInstruction::I32Const(descriptor.aux1));
    body.instruction(&WasmInstruction::I64Const(descriptor.aux2));
    for index in 0..5 {
        if index < operands.len() {
            emit_operand(body, operands, index)?;
        } else {
            body.instruction(&WasmInstruction::F64Const(0.0.into()));
        }
    }
    body.instruction(&WasmInstruction::Call(HELPER_FUNCTION_INDEX));
    emit_checked_helper_result(body, result)
}

fn emit_slice_helper_call(
    body: &mut Function,
    descriptor: HelperDescriptor,
    operands: &[crate::jit::ssa::ValueId],
    result: crate::jit::ssa::ValueId,
) -> WasmJitResult<()> {
    if operands.len() > WASM_JIT_MAX_SLICE_OPERANDS {
        return Err(WasmJitError::Encoding(format!(
            "variable-arity browser helper requires {} operands, exceeding the bounded maximum {}",
            operands.len(),
            WASM_JIT_MAX_SLICE_OPERANDS
        )));
    }
    let operand_bytes = operands
        .len()
        .checked_mul(std::mem::size_of::<f64>())
        .ok_or_else(|| WasmJitError::Encoding("slice-helper frame size overflow".into()))?;
    let required_frame_bytes = usize::try_from(WASM_JIT_SLICE_OPERANDS_OFFSET)
        .ok()
        .and_then(|offset| offset.checked_add(operand_bytes))
        .and_then(|bytes| i32::try_from(bytes).ok())
        .ok_or_else(|| WasmJitError::Encoding("slice-helper frame size exceeds wasm32".into()))?;

    // A short frame must fail before the first store, avoiding a WebAssembly
    // trap and preserving the normal status-based error path.
    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
    body.instruction(&WasmInstruction::I32Load(i32_mem(FRAME_BYTE_LEN_OFFSET)));
    body.instruction(&WasmInstruction::I32Const(required_frame_bytes));
    body.instruction(&WasmInstruction::I32LtU);
    body.instruction(&WasmInstruction::If(BlockType::Empty));
    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
    body.instruction(&WasmInstruction::I32Const(WASM_JIT_STATUS_ABI_MISMATCH));
    body.instruction(&WasmInstruction::I32Store(i32_mem(
        FRAME_ERROR_STATUS_OFFSET,
    )));
    body.instruction(&WasmInstruction::I32Const(WASM_JIT_STATUS_ABI_MISMATCH));
    body.instruction(&WasmInstruction::Return);
    body.instruction(&WasmInstruction::End);

    for (index, _) in operands.iter().enumerate() {
        let offset = usize::try_from(WASM_JIT_SLICE_OPERANDS_OFFSET)
            .ok()
            .and_then(|base| {
                index
                    .checked_mul(std::mem::size_of::<f64>())
                    .and_then(|delta| base.checked_add(delta))
            })
            .and_then(|offset| u64::try_from(offset).ok())
            .ok_or_else(|| WasmJitError::Encoding("slice-helper operand offset overflow".into()))?;
        body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
        emit_operand(body, operands, index)?;
        body.instruction(&WasmInstruction::F64Store(f64_mem(offset)));
    }

    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
    body.instruction(&WasmInstruction::I32Const(descriptor.opcode));
    body.instruction(&WasmInstruction::I32Const(descriptor.aux0));
    body.instruction(&WasmInstruction::I32Const(descriptor.aux1));
    body.instruction(&WasmInstruction::I64Const(descriptor.aux2));
    body.instruction(&WasmInstruction::I32Const(
        i32::try_from(operands.len())
            .map_err(|_| WasmJitError::Encoding("slice-helper operand count exceeds i32".into()))?,
    ));
    body.instruction(&WasmInstruction::Call(SLICE_HELPER_FUNCTION_INDEX));
    emit_checked_helper_result(body, result)
}

fn emit_checked_helper_result(
    body: &mut Function,
    result: crate::jit::ssa::ValueId,
) -> WasmJitResult<()> {
    let result_local = value_local(result.index())?;
    body.instruction(&WasmInstruction::LocalSet(result_local));
    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
    body.instruction(&WasmInstruction::I32Load(i32_mem(
        FRAME_ERROR_STATUS_OFFSET,
    )));
    body.instruction(&WasmInstruction::If(BlockType::Empty));
    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
    body.instruction(&WasmInstruction::I32Load(i32_mem(
        FRAME_ERROR_STATUS_OFFSET,
    )));
    body.instruction(&WasmInstruction::Return);
    body.instruction(&WasmInstruction::End);
    body.instruction(&WasmInstruction::LocalGet(result_local));
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct HelperDescriptor {
    opcode: i32,
    aux0: i32,
    aux1: i32,
    aux2: i64,
}

fn helper_descriptor(op: NativeOp) -> WasmJitResult<HelperDescriptor> {
    let mut descriptor = HelperDescriptor {
        opcode: 0,
        aux0: 0,
        aux1: 0,
        aux2: 0,
    };
    match op {
        NativeOp::LoadVariableDyn { base, len, lower } => {
            descriptor.opcode = 1;
            descriptor.aux0 = index_i32(base)?;
            descriptor.aux1 = index_i32(len)?;
            descriptor.aux2 = lower;
        }
        NativeOp::Extremum(kind) => descriptor.opcode = 10 + extremum_code(kind),
        NativeOp::ExtremumConst(kind, value) => {
            descriptor.opcode = 12 + extremum_code(kind);
            descriptor.aux2 = value.to_bits() as i64;
        }
        NativeOp::ExtremumConstLhs(kind, value) => {
            descriptor.opcode = 14 + extremum_code(kind);
            descriptor.aux2 = value.to_bits() as i64;
        }
        NativeOp::UnaryMath(kind) => descriptor.opcode = 100 + unary_math_code(kind),
        NativeOp::BinaryMath(kind) => descriptor.opcode = 200 + binary_math_code(kind),
        NativeOp::IntegerCast => descriptor.opcode = 300,
        NativeOp::IntegerBinary(kind) => descriptor.opcode = 301 + integer_code(kind),
        NativeOp::IntegerShiftConst(kind, shift) => {
            descriptor.opcode = 310 + integer_code(kind);
            descriptor.aux0 = i32::from(shift);
        }
        NativeOp::IntegerBinaryConst(kind, value) => {
            descriptor.opcode = 320 + integer_code(kind);
            descriptor.aux2 = value;
        }
        NativeOp::TableLookup(index) => set_index(&mut descriptor, 400, index)?,
        NativeOp::TableDerivative(index) => set_index(&mut descriptor, 401, index)?,
        NativeOp::LimitState(index) => set_index(&mut descriptor, 410, index)?,
        NativeOp::LimiterPrevious(index) => set_index(&mut descriptor, 411, index)?,
        NativeOp::LimiterStore(index) => set_index(&mut descriptor, 412, index)?,
        NativeOp::LaplaceState(index) => set_index(&mut descriptor, 420, index)?,
        NativeOp::LaplaceStateDerivative(index) => set_index(&mut descriptor, 432, index)?,
        NativeOp::ZiState(layout) | NativeOp::ZiStateDerivative(layout) => {
            let operand_count = layout.validate_operand_budget().map_err(|error| {
                WasmJitError::Encoding(format!("Zi runtime layout rejected: {error}"))
            })?;
            debug_assert!(operand_count <= WASM_JIT_MAX_SLICE_OPERANDS);
            descriptor.opcode = if matches!(op, NativeOp::ZiStateDerivative(_)) {
                429
            } else {
                421
            };
            descriptor.aux0 = i32::try_from(layout.filter_id).map_err(|_| {
                WasmJitError::Encoding("Zi filter slot exceeds the signed browser ABI".into())
            })?;
            descriptor.aux1 = encode_zi_layout_descriptor(layout).ok_or_else(|| {
                WasmJitError::Encoding("Zi runtime layout exceeds browser descriptor limits".into())
            })?;
            descriptor.aux2 = 0;
        }
        NativeOp::TimerState(index) => set_index(&mut descriptor, 422, index)?,
        NativeOp::TransitionState(index) => set_index(&mut descriptor, 423, index)?,
        NativeOp::TransitionStateDerivative(index) => set_index(&mut descriptor, 446, index)?,
        NativeOp::SlewState(index) => set_index(&mut descriptor, 424, index)?,
        NativeOp::SlewStateDerivative(index) => set_index(&mut descriptor, 445, index)?,
        NativeOp::AbsDelayState(index) => set_index(&mut descriptor, 425, index)?,
        NativeOp::AbsDelayStateMax(index) => set_index(&mut descriptor, 447, index)?,
        NativeOp::AbsDelayStateDerivative(index) => set_index(&mut descriptor, 448, index)?,
        NativeOp::AbsDelayStateDerivativeMax(index) => set_index(&mut descriptor, 449, index)?,
        NativeOp::CrossState(index) => set_index(&mut descriptor, 426, index)?,
        NativeOp::AboveState(index) => set_index(&mut descriptor, 427, index)?,
        NativeOp::LastCrossingState(index) => set_index(&mut descriptor, 428, index)?,
        NativeOp::WhiteNoise => descriptor.opcode = 430,
        NativeOp::FlickerNoise => descriptor.opcode = 431,
        NativeOp::DdtState(index) => set_index(&mut descriptor, 440, index)?,
        NativeOp::DdtJacobian => descriptor.opcode = 441,
        NativeOp::IdtState(index) => set_index(&mut descriptor, 442, index)?,
        NativeOp::IdtJacobian => descriptor.opcode = 443,
        NativeOp::IdtModState(index) => set_index(&mut descriptor, 444, index)?,
        _ => {
            return Err(WasmJitError::Encoding(format!(
                "native op {op:?} reached helper fallback without an ABI opcode"
            )));
        }
    }
    Ok(descriptor)
}

fn set_index(descriptor: &mut HelperDescriptor, opcode: i32, index: usize) -> WasmJitResult<()> {
    descriptor.opcode = opcode;
    descriptor.aux0 = index_i32(index)?;
    Ok(())
}

fn unary_math_code(op: UnaryMathOp) -> i32 {
    match op {
        UnaryMathOp::Exp => 0,
        UnaryMathOp::Log => 1,
        UnaryMathOp::Log10 => 2,
        UnaryMathOp::Sin => 3,
        UnaryMathOp::Cos => 4,
        UnaryMathOp::Tan => 5,
        UnaryMathOp::Sinh => 6,
        UnaryMathOp::Cosh => 7,
        UnaryMathOp::Tanh => 8,
        UnaryMathOp::Asinh => 9,
        UnaryMathOp::Acosh => 10,
        UnaryMathOp::Atanh => 11,
        UnaryMathOp::Limexp => 12,
        UnaryMathOp::LimitedExp => 13,
        UnaryMathOp::Asin => 14,
        UnaryMathOp::Acos => 15,
        UnaryMathOp::Atan => 16,
        UnaryMathOp::Floor => 17,
        UnaryMathOp::Ceil => 18,
    }
}

fn binary_math_code(op: BinaryMathOp) -> i32 {
    match op {
        BinaryMathOp::Pow => 0,
        BinaryMathOp::Atan2 => 1,
        BinaryMathOp::Hypot => 2,
        BinaryMathOp::Mod => 3,
    }
}

fn integer_code(op: IntegerBinaryOp) -> i32 {
    match op {
        IntegerBinaryOp::Shl => 0,
        IntegerBinaryOp::Shr => 1,
        IntegerBinaryOp::BitAnd => 2,
        IntegerBinaryOp::BitOr => 3,
        IntegerBinaryOp::BitXor => 4,
    }
}

fn extremum_code(op: ExtremumOp) -> i32 {
    match op {
        ExtremumOp::Min => 0,
        ExtremumOp::Max => 1,
    }
}

fn emit_voltage_node(body: &mut Function, node: VoltageNode) -> WasmJitResult<()> {
    match node {
        VoltageNode::Terminal(index) => emit_f64_array_load(
            body,
            FRAME_TERMINAL_VOLTAGES_PTR_OFFSET,
            FRAME_TERMINAL_VOLTAGES_LEN_OFFSET,
            index,
        ),
        VoltageNode::Internal(index) => emit_f64_array_load(
            body,
            FRAME_INTERNAL_VOLTAGES_PTR_OFFSET,
            FRAME_INTERNAL_VOLTAGES_LEN_OFFSET,
            index,
        ),
        VoltageNode::Ground => {
            body.instruction(&WasmInstruction::F64Const(0.0.into()));
            Ok(())
        }
    }
}

fn emit_frame_f64_load(body: &mut Function, offset: u64) {
    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
    body.instruction(&WasmInstruction::F64Load(f64_mem(offset)));
}

fn emit_f64_array_load(
    body: &mut Function,
    pointer_offset: u64,
    length_offset: u64,
    index: usize,
) -> WasmJitResult<()> {
    emit_bounds_guard(body, length_offset, index)?;
    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
    body.instruction(&WasmInstruction::I32Load(i32_mem(pointer_offset)));
    body.instruction(&WasmInstruction::F64Load(f64_mem(element_offset(
        index, 8,
    )?)));
    Ok(())
}

fn emit_u8_array_load(
    body: &mut Function,
    pointer_offset: u64,
    length_offset: u64,
    index: usize,
) -> WasmJitResult<()> {
    emit_bounds_guard(body, length_offset, index)?;
    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
    body.instruction(&WasmInstruction::I32Load(i32_mem(pointer_offset)));
    body.instruction(&WasmInstruction::I32Load8U(MemArg {
        offset: u64::try_from(index)
            .map_err(|_| WasmJitError::Encoding("byte index exceeds u64".into()))?,
        align: 0,
        memory_index: 0,
    }));
    body.instruction(&WasmInstruction::F64ConvertI32U);
    Ok(())
}

fn emit_bounds_guard(body: &mut Function, length_offset: u64, index: usize) -> WasmJitResult<()> {
    let required = index
        .checked_add(1)
        .ok_or_else(|| WasmJitError::Encoding("array bound overflow".into()))?;
    let required = u32::try_from(required)
        .map_err(|_| WasmJitError::Encoding("array bound exceeds wasm32".into()))?;
    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
    body.instruction(&WasmInstruction::I32Load(i32_mem(length_offset)));
    body.instruction(&WasmInstruction::I32Const(required as i32));
    body.instruction(&WasmInstruction::I32LtU);
    body.instruction(&WasmInstruction::If(BlockType::Empty));
    emit_status_return(body, WASM_JIT_STATUS_RUNTIME_ERROR);
    body.instruction(&WasmInstruction::End);
    Ok(())
}

fn emit_status_return(body: &mut Function, status: i32) {
    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
    body.instruction(&WasmInstruction::I32Const(status));
    body.instruction(&WasmInstruction::I32Store(i32_mem(
        FRAME_ERROR_STATUS_OFFSET,
    )));
    body.instruction(&WasmInstruction::I32Const(status));
    body.instruction(&WasmInstruction::Return);
}

fn emit_clear_error_status(body: &mut Function) {
    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
    body.instruction(&WasmInstruction::I32Const(WASM_JIT_STATUS_OK));
    body.instruction(&WasmInstruction::I32Store(i32_mem(
        FRAME_ERROR_STATUS_OFFSET,
    )));
}

fn emit_operand(
    body: &mut Function,
    operands: &[crate::jit::ssa::ValueId],
    index: usize,
) -> WasmJitResult<()> {
    let value = operands.get(index).ok_or_else(|| {
        WasmJitError::Encoding(format!("SSA instruction is missing operand {index}"))
    })?;
    body.instruction(&WasmInstruction::LocalGet(value_local(value.index())?));
    Ok(())
}

fn value_local(value_index: usize) -> WasmJitResult<u32> {
    u32::try_from(value_index)
        .ok()
        .and_then(|index| index.checked_add(1))
        .ok_or_else(|| WasmJitError::Encoding("SSA value local exceeds u32".into()))
}

fn element_offset(index: usize, width: usize) -> WasmJitResult<u64> {
    index
        .checked_mul(width)
        .and_then(|offset| u64::try_from(offset).ok())
        .ok_or_else(|| WasmJitError::Encoding("array byte offset overflow".into()))
}

fn index_i32(index: usize) -> WasmJitResult<i32> {
    u32::try_from(index)
        .map(|value| value as i32)
        .map_err(|_| WasmJitError::Encoding("model index exceeds wasm32".into()))
}

fn i32_mem(offset: u64) -> MemArg {
    MemArg {
        offset,
        align: 2,
        memory_index: 0,
    }
}

fn f64_mem(offset: u64) -> MemArg {
    MemArg {
        offset,
        align: 3,
        memory_index: 0,
    }
}

/// Bind the shared slice and frame-free transcendental capabilities for an
/// execution test.
///
/// Every module the emitter produces imports these, so each of the crate's
/// independent-engine harnesses needs them; the bodies are the same production
/// entry points the browser binds, so a test can never disagree with the
/// browser about what `exp` means.
#[cfg(test)]
pub(super) fn define_test_math_imports<T>(linker: &mut wasmi::Linker<T>) {
    linker
        .func_wrap(
            WASM_JIT_IMPORT_MODULE,
            WASM_JIT_SLICE_HELPER_IMPORT,
            |_: i32, _: i32, _: i32, _: i32, _: i64, _: i32| -> f64 { 0.0 },
        )
        .expect("define slice helper import");
    linker
        .func_wrap(
            WASM_JIT_IMPORT_MODULE,
            WASM_JIT_MATH1_IMPORT,
            |opcode: i32, value: f64| -> f64 { super::runtime::math1_v1(opcode, value) },
        )
        .expect("define unary math import");
    linker
        .func_wrap(
            WASM_JIT_IMPORT_MODULE,
            WASM_JIT_MATH2_IMPORT,
            |opcode: i32, left: f64, right: f64| -> f64 {
                super::runtime::math2_v1(opcode, left, right)
            },
        )
        .expect("define binary math import");
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::*;
    use crate::jit::expr::{
        BinaryMathOp, CompareOp, ExtremumOp, IntegerBinaryOp, LogicalOp, NativeOp, NativeProgram,
        UnaryMathOp, VoltageNode, native_op_stack_effect,
    };
    use wasmi::{Caller, Engine, Linker, Memory, MemoryType, Module, Store};
    use wasmparser::Validator;

    fn program(ops: Vec<NativeOp>, depth: usize) -> NativeProgram {
        NativeProgram::from_ops_for_test(ops, depth, Vec::new(), Vec::new())
    }

    #[test]
    fn scalar_ssa_module_is_deterministic_and_standard_wasm() {
        let program = program(
            vec![
                NativeOp::LoadParam(0),
                NativeOp::LoadVariable(1),
                NativeOp::Mul,
                NativeOp::AddConst(2.0),
            ],
            2,
        );
        let first = emit_verified_value_program(&program).expect("encode scalar SSA module");
        let second = emit_verified_value_program(&program).expect("re-encode scalar SSA module");
        assert_eq!(first, second);
        Validator::new()
            .validate_all(&first)
            .expect("validate emitted scalar module");
        assert!(first.len() < 1024);
    }

    #[test]
    fn helper_abi_covers_math_and_stateful_operators() {
        for (ops, depth) in [
            (
                vec![NativeOp::Const(2.0), NativeOp::UnaryMath(UnaryMathOp::Exp)],
                1,
            ),
            (
                vec![NativeOp::Const(1.0), NativeOp::LaplaceStateDerivative(2)],
                1,
            ),
            (
                vec![
                    NativeOp::Const(1.0),
                    NativeOp::Const(0.1),
                    NativeOp::IdtState(3),
                ],
                2,
            ),
            (
                vec![
                    NativeOp::Const(1.0),
                    NativeOp::Const(0.0),
                    NativeOp::Const(1.0),
                    NativeOp::Const(0.0),
                    NativeOp::Const(0.0),
                    NativeOp::CrossState(2),
                ],
                5,
            ),
        ] {
            let bytes =
                emit_verified_value_program(&program(ops, depth)).expect("encode helper op");
            Validator::new()
                .validate_all(&bytes)
                .expect("validate helper module");
        }
    }

    #[test]
    fn laplace_derivative_descriptor_is_pinned_to_the_read_only_helper() {
        let descriptor = helper_descriptor(NativeOp::LaplaceStateDerivative(7))
            .expect("encode Laplace derivative helper descriptor");
        assert_eq!(descriptor.opcode, 432);
        assert_eq!(descriptor.aux0, 7);
        assert_eq!(descriptor.aux1, 0);
        assert_eq!(descriptor.aux2, 0);
    }

    #[test]
    fn transition_derivative_descriptor_is_pinned_to_scalar_opcode_446() {
        let descriptor = helper_descriptor(NativeOp::TransitionStateDerivative(7))
            .expect("encode transition derivative helper descriptor");
        assert_eq!(descriptor.opcode, 446);
        assert_eq!(descriptor.aux0, 7);
        assert_eq!(descriptor.aux1, 0);
        assert_eq!(descriptor.aux2, 0);
    }

    #[test]
    fn absdelay_extended_descriptors_are_pinned_to_scalar_opcodes() {
        for (op, opcode) in [
            (NativeOp::AbsDelayStateMax(7), 447),
            (NativeOp::AbsDelayStateDerivative(7), 448),
            (NativeOp::AbsDelayStateDerivativeMax(7), 449),
        ] {
            let descriptor = helper_descriptor(op).expect("encode absdelay helper descriptor");
            assert_eq!(descriptor.opcode, opcode);
            assert_eq!(descriptor.aux0, 7);
            assert_eq!(descriptor.aux1, 0);
            assert_eq!(descriptor.aux2, 0);
        }
    }

    #[test]
    fn generated_module_routes_transition_derivative_through_opcode_446() {
        const OPERANDS: [f64; 5] = [1.5, -2.0, 0.25, 0.5, 0.75];
        let mut ops = OPERANDS
            .into_iter()
            .map(NativeOp::Const)
            .collect::<Vec<_>>();
        ops.push(NativeOp::TransitionStateDerivative(9));
        let bytes = emit_verified_value_program(&program(ops, 5))
            .expect("encode transition derivative module");

        let engine = Engine::default();
        let module = Module::new(&engine, bytes.as_slice())
            .expect("compile transition derivative module in wasmi");
        let mut store = Store::new(&engine, TestHostState::default());
        let memory = Memory::new(&mut store, MemoryType::new(1, None))
            .expect("allocate imported primary memory");
        store.data_mut().memory = Some(memory);
        let mut linker = Linker::new(&engine);
        linker
            .define(WASM_JIT_IMPORT_MODULE, WASM_JIT_MEMORY_IMPORT, memory)
            .expect("define primary memory import");
        linker
            .func_wrap(
                WASM_JIT_IMPORT_MODULE,
                WASM_JIT_EVAL_HELPER_IMPORT,
                |frame_offset: i32,
                 opcode: i32,
                 aux0: i32,
                 aux1: i32,
                 aux2: i64,
                 op0: f64,
                 op1: f64,
                 op2: f64,
                 op3: f64,
                 op4: f64|
                 -> f64 {
                    assert_eq!(frame_offset, 0);
                    assert_eq!(opcode, 446);
                    assert_eq!(aux0, 9);
                    assert_eq!(aux1, 0);
                    assert_eq!(aux2, 0);
                    let actual = [op0, op1, op2, op3, op4];
                    for (index, (actual, expected)) in actual
                        .iter()
                        .copied()
                        .zip(OPERANDS.iter().copied())
                        .enumerate()
                    {
                        assert_eq!(
                            actual.to_bits(),
                            expected.to_bits(),
                            "scalar operand {index} changed sign or value"
                        );
                    }
                    op1
                },
            )
            .expect("define transition derivative scalar helper import");
        linker
            .func_wrap(
                WASM_JIT_IMPORT_MODULE,
                WASM_JIT_SLICE_HELPER_IMPORT,
                |_: i32, _: i32, _: i32, _: i32, _: i64, _: i32| -> f64 {
                    panic!("five-operand transition derivative must use the scalar helper")
                },
            )
            .expect("define slice helper import");
        linker
            .func_wrap(
                WASM_JIT_IMPORT_MODULE,
                WASM_JIT_MATH1_IMPORT,
                |opcode: i32, value: f64| -> f64 { super::super::runtime::math1_v1(opcode, value) },
            )
            .expect("define unary math import");
        linker
            .func_wrap(
                WASM_JIT_IMPORT_MODULE,
                WASM_JIT_MATH2_IMPORT,
                |opcode: i32, left: f64, right: f64| -> f64 {
                    super::super::runtime::math2_v1(opcode, left, right)
                },
            )
            .expect("define binary math import");
        let instance = linker
            .instantiate_and_start(&mut store, &module)
            .expect("instantiate transition derivative module");

        let frame_len = WASM_JIT_EVAL_FRAME_BYTES as usize;
        let mut frame = vec![0_u8; frame_len];
        frame[FRAME_MAGIC_OFFSET as usize..FRAME_MAGIC_OFFSET as usize + 4]
            .copy_from_slice(&WASM_JIT_FRAME_MAGIC.to_le_bytes());
        frame[FRAME_ABI_VERSION_OFFSET as usize..FRAME_ABI_VERSION_OFFSET as usize + 4]
            .copy_from_slice(&WASM_JIT_ABI_VERSION.to_le_bytes());
        frame[FRAME_BYTE_LEN_OFFSET as usize..FRAME_BYTE_LEN_OFFSET as usize + 4].copy_from_slice(
            &u32::try_from(frame_len)
                .expect("test frame length fits wasm32")
                .to_le_bytes(),
        );
        memory
            .write(&mut store, 0, &frame)
            .expect("write transition derivative frame");

        let entry = instance
            .get_typed_func::<i32, i32>(&store, WASM_JIT_VALUE_EXPORT)
            .expect("resolve transition derivative export");
        assert_eq!(
            entry
                .call(&mut store, 0)
                .expect("execute transition derivative"),
            WASM_JIT_STATUS_OK
        );
        let result = f64::from_le_bytes(
            memory.data(&store)
                [FRAME_RESULT_OFFSET as usize..FRAME_RESULT_OFFSET as usize + size_of::<f64>()]
                .try_into()
                .expect("complete transition derivative result"),
        );
        assert_eq!(result.to_bits(), (-2.0_f64).to_bits());
    }

    #[test]
    fn generated_module_routes_signed_slew_derivative_through_opcode_445() {
        const OPERANDS: [f64; 6] = [10.0, 0.0, 2.0, -0.25, -2.0, 0.5];
        let mut ops = OPERANDS
            .into_iter()
            .map(NativeOp::Const)
            .collect::<Vec<_>>();
        ops.push(NativeOp::SlewStateDerivative(0));
        let bytes = emit_verified_value_program(&program(ops, 6))
            .expect("encode signed slew derivative module");

        let engine = Engine::default();
        let module = Module::new(&engine, bytes.as_slice())
            .expect("compile signed slew derivative module in wasmi");
        let mut store = Store::new(&engine, TestHostState::default());
        let memory = Memory::new(&mut store, MemoryType::new(1, None))
            .expect("allocate imported primary memory");
        store.data_mut().memory = Some(memory);
        let mut linker = Linker::new(&engine);
        linker
            .define(WASM_JIT_IMPORT_MODULE, WASM_JIT_MEMORY_IMPORT, memory)
            .expect("define primary memory import");
        linker
            .func_wrap(
                WASM_JIT_IMPORT_MODULE,
                WASM_JIT_EVAL_HELPER_IMPORT,
                |_: i32,
                 _: i32,
                 _: i32,
                 _: i32,
                 _: i64,
                 _: f64,
                 _: f64,
                 _: f64,
                 _: f64,
                 _: f64|
                 -> f64 {
                    panic!("six-operand slew derivative must use the slice helper")
                },
            )
            .expect("define scalar helper import");
        linker
            .func_wrap(
                WASM_JIT_IMPORT_MODULE,
                WASM_JIT_SLICE_HELPER_IMPORT,
                |caller: Caller<'_, TestHostState>,
                 frame_offset: i32,
                 opcode: i32,
                 aux0: i32,
                 aux1: i32,
                 aux2: i64,
                 operand_count: i32|
                 -> f64 {
                    assert_eq!(frame_offset, 0);
                    assert_eq!(opcode, 445);
                    assert_eq!(aux0, 0);
                    assert_eq!(aux1, 0);
                    assert_eq!(aux2, 0);
                    assert_eq!(operand_count, 6);
                    let memory = caller.data().memory.expect("installed test memory");
                    let bytes = memory.data(&caller);
                    let base = usize::try_from(WASM_JIT_SLICE_OPERANDS_OFFSET)
                        .expect("slice operand offset fits host usize");
                    let actual = std::array::from_fn::<_, 6, _>(|index| {
                        let offset = base + index * size_of::<f64>();
                        f64::from_le_bytes(
                            bytes[offset..offset + size_of::<f64>()]
                                .try_into()
                                .expect("complete slice operand"),
                        )
                    });
                    for (index, (actual, expected)) in actual
                        .iter()
                        .copied()
                        .zip(OPERANDS.iter().copied())
                        .enumerate()
                    {
                        assert_eq!(
                            actual.to_bits(),
                            expected.to_bits(),
                            "slice operand {index} changed sign or value"
                        );
                    }
                    actual[3]
                },
            )
            .expect("define signed slew derivative slice helper import");
        linker
            .func_wrap(
                WASM_JIT_IMPORT_MODULE,
                WASM_JIT_MATH1_IMPORT,
                |opcode: i32, value: f64| -> f64 { super::super::runtime::math1_v1(opcode, value) },
            )
            .expect("define unary math import");
        linker
            .func_wrap(
                WASM_JIT_IMPORT_MODULE,
                WASM_JIT_MATH2_IMPORT,
                |opcode: i32, left: f64, right: f64| -> f64 {
                    super::super::runtime::math2_v1(opcode, left, right)
                },
            )
            .expect("define binary math import");
        let instance = linker
            .instantiate_and_start(&mut store, &module)
            .expect("instantiate signed slew derivative module");

        let frame_len = usize::try_from(WASM_JIT_SLICE_OPERANDS_OFFSET)
            .expect("slice operand offset fits host usize")
            + OPERANDS.len() * size_of::<f64>();
        let mut frame = vec![0_u8; frame_len];
        frame[FRAME_MAGIC_OFFSET as usize..FRAME_MAGIC_OFFSET as usize + 4]
            .copy_from_slice(&WASM_JIT_FRAME_MAGIC.to_le_bytes());
        frame[FRAME_ABI_VERSION_OFFSET as usize..FRAME_ABI_VERSION_OFFSET as usize + 4]
            .copy_from_slice(&WASM_JIT_ABI_VERSION.to_le_bytes());
        frame[FRAME_BYTE_LEN_OFFSET as usize..FRAME_BYTE_LEN_OFFSET as usize + 4].copy_from_slice(
            &u32::try_from(frame_len)
                .expect("test frame length fits wasm32")
                .to_le_bytes(),
        );
        memory
            .write(&mut store, 0, &frame)
            .expect("write signed slew derivative frame");

        let entry = instance
            .get_typed_func::<i32, i32>(&store, WASM_JIT_VALUE_EXPORT)
            .expect("resolve signed slew derivative export");
        assert_eq!(
            entry
                .call(&mut store, 0)
                .expect("execute signed slew derivative"),
            WASM_JIT_STATUS_OK
        );
        let result = f64::from_le_bytes(
            memory.data(&store)
                [FRAME_RESULT_OFFSET as usize..FRAME_RESULT_OFFSET as usize + size_of::<f64>()]
                .try_into()
                .expect("complete signed derivative result"),
        );
        assert_eq!(result.to_bits(), (-0.25_f64).to_bits());
    }

    #[test]
    fn model_module_emits_direct_indexed_and_bounded_loop_assignment_kernels() {
        let value = program(vec![NativeOp::Const(7.0)], 1);
        let index = program(vec![NativeOp::Const(1.0)], 1);
        let condition = program(vec![NativeOp::Const(0.0)], 1);
        let programs = [&value, &index, &condition].map(PlanProgramRef::Postfix);
        let kernels = [WasmAssignmentKernel {
            export_name: WASM_JIT_ASSIGNMENT_EXPORT,
            assignments: vec![
                WasmAssignment::Direct {
                    variable_index: 0,
                    value_entry: 0,
                },
                WasmAssignment::Indexed {
                    base: 1,
                    len: 3,
                    lower: -1,
                    index_entry: 1,
                    value_entry: 0,
                },
                WasmAssignment::Loop {
                    condition_entry: 2,
                    body: vec![WasmAssignment::Direct {
                        variable_index: 4,
                        value_entry: 0,
                    }],
                },
            ],
        }];
        let first = emit_verified_model_module(&programs, &kernels, &[])
            .expect("encode complete assignment kernel");
        let second = emit_verified_model_module(&programs, &kernels, &[])
            .expect("re-encode complete assignment kernel");
        assert_eq!(first, second);
        Validator::new()
            .validate_all(&first)
            .expect("validate assignment model module");
        assert!(first.len() < 4096);
    }

    #[derive(Default)]
    struct TestHostState {
        memory: Option<Memory>,
    }

    fn read_u32(bytes: &[u8], offset: usize) -> Option<u32> {
        Some(u32::from_le_bytes(
            bytes.get(offset..offset.checked_add(4)?)?.try_into().ok()?,
        ))
    }

    #[test]
    fn independent_wasm_engine_executes_direct_indexed_and_loop_assignments() {
        let seven = program(vec![NativeOp::Const(7.0)], 1);
        let index = program(vec![NativeOp::Const(1.0)], 1);
        let condition = program(vec![NativeOp::LoadVariable(4)], 1);
        let zero = program(vec![NativeOp::Const(0.0)], 1);
        let programs = [&seven, &index, &condition, &zero].map(PlanProgramRef::Postfix);
        let kernels = [WasmAssignmentKernel {
            export_name: WASM_JIT_ASSIGNMENT_EXPORT,
            assignments: vec![
                WasmAssignment::Direct {
                    variable_index: 0,
                    value_entry: 0,
                },
                WasmAssignment::Indexed {
                    base: 1,
                    len: 3,
                    lower: -1,
                    index_entry: 1,
                    value_entry: 0,
                },
                WasmAssignment::Loop {
                    condition_entry: 2,
                    body: vec![WasmAssignment::Direct {
                        variable_index: 4,
                        value_entry: 3,
                    }],
                },
            ],
        }];
        let bytes = emit_verified_model_module(&programs, &kernels, &[])
            .expect("encode executable assignment module");

        let engine = Engine::default();
        let module = Module::new(&engine, bytes.as_slice()).expect("compile module in wasmi");
        let mut store = Store::new(&engine, TestHostState::default());
        let memory = Memory::new(&mut store, MemoryType::new(1, None))
            .expect("allocate imported primary memory");
        store.data_mut().memory = Some(memory);
        let mut linker = Linker::new(&engine);
        linker
            .define(WASM_JIT_IMPORT_MODULE, WASM_JIT_MEMORY_IMPORT, memory)
            .expect("define primary memory import");
        linker
            .func_wrap(
                WASM_JIT_IMPORT_MODULE,
                WASM_JIT_EVAL_HELPER_IMPORT,
                |mut caller: Caller<'_, TestHostState>,
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
                    let Some(frame_offset) = usize::try_from(frame_offset).ok() else {
                        return 0.0;
                    };
                    let memory = caller.data().memory.expect("installed test memory");
                    let variables = {
                        let bytes = memory.data(&caller);
                        let Some(variables_ptr) =
                            read_u32(bytes, frame_offset + FRAME_VARIABLES_PTR_OFFSET as usize)
                        else {
                            return 0.0;
                        };
                        let Some(variables_len) =
                            read_u32(bytes, frame_offset + FRAME_VARIABLES_LEN_OFFSET as usize)
                        else {
                            return 0.0;
                        };
                        let Some(variables_ptr) = usize::try_from(variables_ptr).ok() else {
                            return 0.0;
                        };
                        let Some(variables_len) = usize::try_from(variables_len).ok() else {
                            return 0.0;
                        };
                        let mut variables = Vec::with_capacity(variables_len);
                        for index in 0..variables_len {
                            let Some(offset) = index
                                .checked_mul(size_of::<f64>())
                                .and_then(|offset| variables_ptr.checked_add(offset))
                            else {
                                return 0.0;
                            };
                            let Some(raw) = bytes.get(offset..offset + size_of::<f64>()) else {
                                return 0.0;
                            };
                            variables.push(f64::from_le_bytes(raw.try_into().unwrap()));
                        }
                        variables
                    };
                    match super::super::runtime::evaluate_helper(
                        opcode,
                        aux0,
                        aux1,
                        aux2,
                        [operand0, operand1, operand2, operand3, operand4],
                        &variables,
                    ) {
                        Ok(value) => value,
                        Err(_) => {
                            let offset = frame_offset + FRAME_ERROR_STATUS_OFFSET as usize;
                            memory
                                .write(
                                    &mut caller,
                                    offset,
                                    &WASM_JIT_STATUS_RUNTIME_ERROR.to_le_bytes(),
                                )
                                .expect("write helper failure status");
                            0.0
                        }
                    }
                },
            )
            .expect("define scalar helper import");
        define_test_math_imports(&mut linker);
        let instance = linker
            .instantiate_and_start(&mut store, &module)
            .expect("instantiate executable assignment module");

        const FRAME_OFFSET: usize = 0;
        const VARIABLES_OFFSET: usize = 256;
        let mut frame = vec![0_u8; WASM_JIT_EVAL_FRAME_BYTES as usize];
        frame[FRAME_MAGIC_OFFSET as usize..FRAME_MAGIC_OFFSET as usize + 4]
            .copy_from_slice(&WASM_JIT_FRAME_MAGIC.to_le_bytes());
        frame[FRAME_ABI_VERSION_OFFSET as usize..FRAME_ABI_VERSION_OFFSET as usize + 4]
            .copy_from_slice(&WASM_JIT_ABI_VERSION.to_le_bytes());
        frame[FRAME_BYTE_LEN_OFFSET as usize..FRAME_BYTE_LEN_OFFSET as usize + 4]
            .copy_from_slice(&WASM_JIT_EVAL_FRAME_BYTES.to_le_bytes());
        frame[FRAME_VARIABLES_PTR_OFFSET as usize..FRAME_VARIABLES_PTR_OFFSET as usize + 4]
            .copy_from_slice(&(VARIABLES_OFFSET as u32).to_le_bytes());
        frame[FRAME_VARIABLES_LEN_OFFSET as usize..FRAME_VARIABLES_LEN_OFFSET as usize + 4]
            .copy_from_slice(&5_u32.to_le_bytes());
        memory
            .write(&mut store, FRAME_OFFSET, &frame)
            .expect("write evaluation frame");
        for (index, value) in [0.0_f64, 0.0, 0.0, 0.0, 1.0].into_iter().enumerate() {
            memory
                .write(
                    &mut store,
                    VARIABLES_OFFSET + index * size_of::<f64>(),
                    &value.to_le_bytes(),
                )
                .expect("write initial variable");
        }

        let assign = instance
            .get_typed_func::<i32, i32>(&store, WASM_JIT_ASSIGNMENT_EXPORT)
            .expect("resolve assignment export");
        let status = assign
            .call(&mut store, FRAME_OFFSET as i32)
            .expect("execute assignment export");
        assert_eq!(status, WASM_JIT_STATUS_OK);

        let memory_bytes = memory.data(&store);
        let variables = (0..5)
            .map(|index| {
                let offset = VARIABLES_OFFSET + index * size_of::<f64>();
                f64::from_le_bytes(
                    memory_bytes[offset..offset + size_of::<f64>()]
                        .try_into()
                        .unwrap(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(variables, vec![7.0, 0.0, 0.0, 7.0, 0.0]);
    }

    #[test]
    fn independent_wasm_engine_executes_analysis_masks_by_id() {
        let programs = (0_u8..=8)
            .map(|analysis_id| program(vec![NativeOp::Analysis(analysis_id)], 1))
            .collect::<Vec<_>>();
        let program_refs = programs
            .iter()
            .map(PlanProgramRef::Postfix)
            .collect::<Vec<_>>();
        let bytes = emit_verified_value_program_set(&program_refs)
            .expect("encode analysis-mask model module");

        let engine = Engine::default();
        let module = Module::new(&engine, bytes.as_slice()).expect("compile module in wasmi");
        let mut store = Store::new(&engine, ());
        let memory = Memory::new(&mut store, MemoryType::new(1, None))
            .expect("allocate imported primary memory");
        let mut linker = Linker::new(&engine);
        linker
            .define(WASM_JIT_IMPORT_MODULE, WASM_JIT_MEMORY_IMPORT, memory)
            .expect("define primary memory import");
        linker
            .func_wrap(
                WASM_JIT_IMPORT_MODULE,
                WASM_JIT_EVAL_HELPER_IMPORT,
                |_: i32,
                 _: i32,
                 _: i32,
                 _: i32,
                 _: i64,
                 _: f64,
                 _: f64,
                 _: f64,
                 _: f64,
                 _: f64|
                 -> f64 { 0.0 },
            )
            .expect("define unused scalar helper import");
        define_test_math_imports(&mut linker);
        let instance = linker
            .instantiate_and_start(&mut store, &module)
            .expect("instantiate analysis-mask module");

        let mut frame = vec![0_u8; WASM_JIT_EVAL_FRAME_BYTES as usize];
        frame[FRAME_MAGIC_OFFSET as usize..FRAME_MAGIC_OFFSET as usize + 4]
            .copy_from_slice(&WASM_JIT_FRAME_MAGIC.to_le_bytes());
        frame[FRAME_ABI_VERSION_OFFSET as usize..FRAME_ABI_VERSION_OFFSET as usize + 4]
            .copy_from_slice(&WASM_JIT_ABI_VERSION.to_le_bytes());
        frame[FRAME_BYTE_LEN_OFFSET as usize..FRAME_BYTE_LEN_OFFSET as usize + 4]
            .copy_from_slice(&WASM_JIT_EVAL_FRAME_BYTES.to_le_bytes());

        for (mask, expected) in [
            ((1 << 0) | (1 << 5) | (1 << 7), [0_u8, 5, 7].as_slice()),
            ((1 << 2) | (1 << 8), [2_u8, 8].as_slice()),
            ((1 << 1) | (1 << 6), [1_u8, 6].as_slice()),
        ] {
            frame[FRAME_ANALYSIS_MASK_OFFSET as usize..FRAME_ANALYSIS_MASK_OFFSET as usize + 4]
                .copy_from_slice(&(mask as u32).to_le_bytes());
            memory
                .write(&mut store, 0, &frame)
                .expect("write evaluation frame");

            for analysis_id in 0_u8..=8 {
                let export = format!("rspice_wasm_jit_value_{analysis_id:08x}");
                let entry = instance
                    .get_typed_func::<i32, i32>(&store, &export)
                    .expect("resolve analysis export");
                assert_eq!(
                    entry.call(&mut store, 0).expect("execute analysis export"),
                    WASM_JIT_STATUS_OK
                );
                let raw = memory
                    .data(&store)
                    .get(FRAME_RESULT_OFFSET as usize..FRAME_RESULT_OFFSET as usize + 8)
                    .expect("read result bytes");
                let result = f64::from_le_bytes(raw.try_into().unwrap());
                assert_eq!(
                    result,
                    if expected.contains(&analysis_id) {
                        1.0
                    } else {
                        0.0
                    },
                    "analysis id {analysis_id} under mask {mask:#x}"
                );
            }
        }
    }

    /// Transcendentals take the frame-free capability, and the frame-carrying
    /// descriptor helper is never reached for them.
    ///
    /// `exp` and `ln` dominate every semiconductor model's inner loop. Routing
    /// them through `eval_op_v1` costs ten pushed arguments, a frame
    /// revalidation, and an error-status reload and branch at each call site,
    /// so the trap here is a silent regression back onto that path.
    #[test]
    fn transcendentals_execute_through_the_frame_free_math_capability() {
        // exp(ln(param0)) ** 1.0 -- one unary pair and one binary op, so a
        // regression on either capability fails this.
        let program = program(
            vec![
                NativeOp::LoadParam(0),
                NativeOp::UnaryMath(UnaryMathOp::Log),
                NativeOp::UnaryMath(UnaryMathOp::Exp),
                NativeOp::Const(1.0),
                NativeOp::BinaryMath(BinaryMathOp::Pow),
            ],
            2,
        );
        let bytes = emit_verified_value_program(&program).expect("encode transcendental module");

        let engine = Engine::default();
        let module = Module::new(&engine, bytes.as_slice()).expect("compile module in wasmi");
        let mut store = Store::new(&engine, ());
        let memory = Memory::new(&mut store, MemoryType::new(1, None))
            .expect("allocate imported primary memory");
        let mut linker = Linker::new(&engine);
        linker
            .define(WASM_JIT_IMPORT_MODULE, WASM_JIT_MEMORY_IMPORT, memory)
            .expect("define primary memory import");
        linker
            .func_wrap(
                WASM_JIT_IMPORT_MODULE,
                WASM_JIT_EVAL_HELPER_IMPORT,
                |_: i32,
                 _: i32,
                 _: i32,
                 _: i32,
                 _: i64,
                 _: f64,
                 _: f64,
                 _: f64,
                 _: f64,
                 _: f64|
                 -> f64 {
                    panic!("pure transcendentals must not reach the frame-carrying helper")
                },
            )
            .expect("define trap helper import");
        define_test_math_imports(&mut linker);
        let instance = linker
            .instantiate_and_start(&mut store, &module)
            .expect("instantiate transcendental module");

        const PARAMETERS_OFFSET: u32 = 256;
        let mut frame = vec![0_u8; WASM_JIT_EVAL_FRAME_BYTES as usize];
        let mut write_u32 = |offset: u64, value: u32| {
            let offset = offset as usize;
            frame[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        };
        write_u32(FRAME_MAGIC_OFFSET, WASM_JIT_FRAME_MAGIC);
        write_u32(FRAME_ABI_VERSION_OFFSET, WASM_JIT_ABI_VERSION);
        write_u32(FRAME_BYTE_LEN_OFFSET, WASM_JIT_EVAL_FRAME_BYTES);
        write_u32(FRAME_PARAMETERS_PTR_OFFSET, PARAMETERS_OFFSET);
        write_u32(FRAME_PARAMETERS_LEN_OFFSET, 1);
        memory
            .write(&mut store, 0, &frame)
            .expect("write evaluation frame");

        let input = 7.5_f64;
        memory
            .write(&mut store, PARAMETERS_OFFSET as usize, &input.to_le_bytes())
            .expect("write parameter");

        let entry = instance
            .get_typed_func::<i32, i32>(&store, WASM_JIT_VALUE_EXPORT)
            .expect("resolve transcendental export");
        assert_eq!(
            entry.call(&mut store, 0).expect("execute transcendental"),
            WASM_JIT_STATUS_OK
        );
        let raw = memory
            .data(&store)
            .get(FRAME_RESULT_OFFSET as usize..FRAME_RESULT_OFFSET as usize + 8)
            .expect("read result bytes");
        let result = f64::from_le_bytes(raw.try_into().unwrap());
        assert_eq!(
            result,
            input.ln().exp().powf(1.0),
            "the capability must be bit-identical to the shared constant-math semantics"
        );
    }

    /// The inlined operations agree with the reference bit for bit, including
    /// the corners where WebAssembly's own instructions disagree with it.
    ///
    /// `f64.min`/`f64.max` return a NaN when either operand is NaN and prefer
    /// the negative zero for equal-magnitude zeros; the reference returns the
    /// non-NaN operand and settles zeros on the left operand. Comparing raw
    /// bits rather than values is what makes the zero-sign case meaningful.
    #[test]
    fn inlined_extremum_and_integer_ops_match_the_reference_bitwise() {
        const CORNERS: [f64; 9] = [
            f64::NAN,
            0.0,
            -0.0,
            1.0,
            -1.0,
            f64::INFINITY,
            f64::NEG_INFINITY,
            1.0e-30,
            -3.5,
        ];

        let engine = Engine::default();
        for kind in [ExtremumOp::Min, ExtremumOp::Max] {
            let program = program(
                vec![
                    NativeOp::LoadParam(0),
                    NativeOp::LoadParam(1),
                    NativeOp::Extremum(kind),
                ],
                2,
            );
            let bytes = emit_verified_value_program(&program).expect("encode extremum module");
            let (mut store, memory, instance) = instantiate_value_module(&engine, &bytes);

            for left in CORNERS {
                for right in CORNERS {
                    let actual = call_value_entry(&mut store, &memory, &instance, &[left, right]);
                    let expected = crate::jit::expr::constant_extremum(kind, left, right);
                    assert_eq!(
                        actual.to_bits(),
                        expected.to_bits(),
                        "{kind:?}({left}, {right}) produced {actual}, reference is {expected}"
                    );
                }
            }
        }

        for op in [
            IntegerBinaryOp::BitAnd,
            IntegerBinaryOp::BitOr,
            IntegerBinaryOp::BitXor,
        ] {
            let program = program(
                vec![
                    NativeOp::LoadParam(0),
                    NativeOp::LoadParam(1),
                    NativeOp::IntegerBinary(op),
                ],
                2,
            );
            let bytes = emit_verified_value_program(&program).expect("encode bitwise module");
            let (mut store, memory, instance) = instantiate_value_module(&engine, &bytes);

            for left in [0.0, 6.0, -6.0, 255.0, f64::from(i32::MIN)] {
                for right in [0.0, 3.0, -3.0, 15.0, f64::from(i32::MAX)] {
                    let actual = call_value_entry(&mut store, &memory, &instance, &[left, right]);
                    let expected = crate::jit::expr::constant_integer_binary(op, left, right)
                        .expect("valid signed-32-bit bitwise operation");
                    assert_eq!(
                        actual.to_bits(),
                        expected.to_bits(),
                        "{op:?}({left}, {right}) produced {actual}, reference is {expected}"
                    );
                }
            }
        }

        let program = program(vec![NativeOp::LoadParam(0), NativeOp::IntegerCast], 1);
        let bytes = emit_verified_value_program(&program).expect("encode integer-cast module");
        let (mut store, memory, instance) = instantiate_value_module(&engine, &bytes);
        for value in [
            0.0,
            -0.0,
            2.5,
            -2.5,
            f64::from(i32::MAX),
            f64::from(i32::MIN),
        ] {
            let actual = call_value_entry(&mut store, &memory, &instance, &[value]);
            let expected = f64::from(
                crate::integer_runtime::real_to_integer(value)
                    .expect("valid signed-32-bit conversion"),
            );
            assert_eq!(
                actual.to_bits(),
                expected.to_bits(),
                "integer cast of {value} produced {actual}, reference is {expected}"
            );
        }
    }

    /// Instantiate a single-entry value module with every capability bound.
    /// The frame-carrying helper evaluates the same shared pure contract used
    /// by the browser host runtime.
    fn instantiate_value_module(
        engine: &Engine,
        bytes: &[u8],
    ) -> (Store<()>, Memory, wasmi::Instance) {
        let module = Module::new(engine, bytes).expect("compile module in wasmi");
        let mut store = Store::new(engine, ());
        let memory = Memory::new(&mut store, MemoryType::new(1, None))
            .expect("allocate imported primary memory");
        let mut linker = Linker::new(engine);
        linker
            .define(WASM_JIT_IMPORT_MODULE, WASM_JIT_MEMORY_IMPORT, memory)
            .expect("define primary memory import");
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
                    crate::wasm_jit::runtime::evaluate_helper(
                        opcode,
                        aux0,
                        aux1,
                        aux2,
                        [operand0, operand1, operand2, operand3, operand4],
                        &[],
                    )
                    .expect("pure value helper")
                },
            )
            .expect("define trap helper import");
        define_test_math_imports(&mut linker);
        let instance = linker
            .instantiate_and_start(&mut store, &module)
            .expect("instantiate value module");
        (store, memory, instance)
    }

    /// Write `parameters` into a fresh frame and run the module's sole entry.
    fn call_value_entry(
        store: &mut Store<()>,
        memory: &Memory,
        instance: &wasmi::Instance,
        parameters: &[f64],
    ) -> f64 {
        const PARAMETERS_OFFSET: u32 = 256;
        let mut frame = vec![0_u8; WASM_JIT_EVAL_FRAME_BYTES as usize];
        let mut write_u32 = |offset: u64, value: u32| {
            let offset = offset as usize;
            frame[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        };
        write_u32(FRAME_MAGIC_OFFSET, WASM_JIT_FRAME_MAGIC);
        write_u32(FRAME_ABI_VERSION_OFFSET, WASM_JIT_ABI_VERSION);
        write_u32(FRAME_BYTE_LEN_OFFSET, WASM_JIT_EVAL_FRAME_BYTES);
        write_u32(FRAME_PARAMETERS_PTR_OFFSET, PARAMETERS_OFFSET);
        write_u32(
            FRAME_PARAMETERS_LEN_OFFSET,
            u32::try_from(parameters.len()).expect("parameter count fits"),
        );
        memory.write(&mut *store, 0, &frame).expect("write frame");
        for (index, value) in parameters.iter().enumerate() {
            memory
                .write(
                    &mut *store,
                    PARAMETERS_OFFSET as usize + index * size_of::<f64>(),
                    &value.to_le_bytes(),
                )
                .expect("write parameter");
        }

        let entry = instance
            .get_typed_func::<i32, i32>(&*store, WASM_JIT_VALUE_EXPORT)
            .expect("resolve value export");
        assert_eq!(
            entry.call(&mut *store, 0).expect("execute value entry"),
            WASM_JIT_STATUS_OK
        );
        let raw = memory
            .data(&*store)
            .get(FRAME_RESULT_OFFSET as usize..FRAME_RESULT_OFFSET as usize + 8)
            .expect("read result bytes");
        f64::from_le_bytes(raw.try_into().unwrap())
    }

    #[test]
    fn verifier_rejects_valid_wasm_tampering_against_authenticated_ssa() {
        let expected = program(vec![NativeOp::Const(2.0)], 1);
        let bytes = emit_verified_value_program(&expected).expect("encode constant module");
        let alternate = program(vec![NativeOp::Const(3.0)], 1);
        assert!(verify_value_program(&bytes, &alternate).is_err());
    }

    #[test]
    fn zi_slice_helper_enforces_its_exact_browser_resource_bound() {
        let at_limit = crate::codegen::ZiRuntimeLayout {
            filter_id: 0,
            numerator: crate::codegen::ZiPolynomialLayout::Coefficients { len: 1019 },
            denominator: crate::codegen::ZiPolynomialLayout::Coefficients { len: 1 },
            direct_assignment: false,
        };
        assert_eq!(at_limit.operand_count(), WASM_JIT_MAX_SLICE_OPERANDS);
        helper_descriptor(NativeOp::ZiState(at_limit)).expect("1,024 operands are supported");

        let over_limit = crate::codegen::ZiRuntimeLayout {
            numerator: crate::codegen::ZiPolynomialLayout::Coefficients { len: 1020 },
            ..at_limit
        };
        let error = helper_descriptor(NativeOp::ZiState(over_limit))
            .expect_err("1,025 operands must fail closed");
        assert!(error.to_string().contains("platform-uniform maximum 1024"));
    }

    #[test]
    fn every_canonical_native_op_family_has_a_wasm_translation() {
        let mut ops = vec![
            NativeOp::Const(1.0),
            NativeOp::LoadParam(0),
            NativeOp::LoadParamGiven(0),
            NativeOp::LoadPortConnected(0),
            NativeOp::LoadVoltage {
                pos: VoltageNode::Terminal(0),
                neg: VoltageNode::Ground,
            },
            NativeOp::LoadCurrent(0),
            NativeOp::LoadPriorCurrent(0),
            NativeOp::LoadInternalVoltage(0),
            NativeOp::LoadVariable(0),
            NativeOp::LoadVariableDyn {
                base: 0,
                len: 1,
                lower: -1,
            },
            NativeOp::LoadBranchUnknown(0),
            NativeOp::LoadTemperature,
            NativeOp::LoadThermalVoltage,
            NativeOp::LoadTime,
            NativeOp::Analysis(1),
            NativeOp::LoadMfactor,
            NativeOp::Add,
            NativeOp::Sub,
            NativeOp::Mul,
            NativeOp::Div,
            NativeOp::AddConst(1.0),
            NativeOp::SubConst(1.0),
            NativeOp::MulConst(1.0),
            NativeOp::DivConst(1.0),
            NativeOp::SubFromConst(1.0),
            NativeOp::DivFromConst(1.0),
            NativeOp::Neg,
            NativeOp::Abs,
            NativeOp::Square,
            NativeOp::Sqrt,
            NativeOp::IfElse,
            NativeOp::IntegerCast,
            NativeOp::TableLookup(0),
            NativeOp::TableDerivative(0),
            NativeOp::LimitState(0),
            NativeOp::LimiterPrevious(0),
            NativeOp::LimiterStore(0),
            NativeOp::LaplaceState(0),
            NativeOp::LaplaceStateDerivative(0),
            NativeOp::ZiState(crate::codegen::ZiRuntimeLayout::unit_coefficients(0)),
            NativeOp::ZiStateDerivative(crate::codegen::ZiRuntimeLayout::unit_coefficients(0)),
            NativeOp::TimerState(0),
            NativeOp::TransitionState(0),
            NativeOp::TransitionStateDerivative(0),
            NativeOp::SlewState(0),
            NativeOp::SlewStateDerivative(0),
            NativeOp::AbsDelayState(0),
            NativeOp::AbsDelayStateMax(0),
            NativeOp::AbsDelayStateDerivative(0),
            NativeOp::AbsDelayStateDerivativeMax(0),
            NativeOp::CrossState(0),
            NativeOp::AboveState(0),
            NativeOp::LastCrossingState(0),
            NativeOp::WhiteNoise,
            NativeOp::FlickerNoise,
            NativeOp::DdtState(0),
            NativeOp::DdtJacobian,
            NativeOp::IdtState(0),
            NativeOp::IdtJacobian,
            NativeOp::IdtModState(0),
        ];
        for compare in [
            CompareOp::Gt,
            CompareOp::Lt,
            CompareOp::Ge,
            CompareOp::Le,
            CompareOp::Eq,
            CompareOp::Ne,
        ] {
            ops.push(NativeOp::Compare(compare));
            ops.push(NativeOp::CompareConst(compare, 1.0));
        }
        for logical in [LogicalOp::And, LogicalOp::Or, LogicalOp::Not] {
            ops.push(NativeOp::Logical(logical));
            if logical != LogicalOp::Not {
                ops.push(NativeOp::LogicalConst(logical, true));
            }
        }
        for extremum in [ExtremumOp::Min, ExtremumOp::Max] {
            ops.push(NativeOp::Extremum(extremum));
            ops.push(NativeOp::ExtremumConst(extremum, 1.0));
            ops.push(NativeOp::ExtremumConstLhs(extremum, 1.0));
        }
        for unary in [
            UnaryMathOp::Exp,
            UnaryMathOp::Log,
            UnaryMathOp::Log10,
            UnaryMathOp::Sin,
            UnaryMathOp::Cos,
            UnaryMathOp::Tan,
            UnaryMathOp::Sinh,
            UnaryMathOp::Cosh,
            UnaryMathOp::Tanh,
            UnaryMathOp::Asinh,
            UnaryMathOp::Acosh,
            UnaryMathOp::Atanh,
            UnaryMathOp::Limexp,
            UnaryMathOp::LimitedExp,
            UnaryMathOp::Asin,
            UnaryMathOp::Acos,
            UnaryMathOp::Atan,
            UnaryMathOp::Floor,
            UnaryMathOp::Ceil,
        ] {
            ops.push(NativeOp::UnaryMath(unary));
        }
        for binary in [
            BinaryMathOp::Pow,
            BinaryMathOp::Atan2,
            BinaryMathOp::Hypot,
            BinaryMathOp::Mod,
        ] {
            ops.push(NativeOp::BinaryMath(binary));
        }
        for integer in [
            IntegerBinaryOp::Shl,
            IntegerBinaryOp::Shr,
            IntegerBinaryOp::BitAnd,
            IntegerBinaryOp::BitOr,
            IntegerBinaryOp::BitXor,
        ] {
            ops.push(NativeOp::IntegerBinary(integer));
            ops.push(NativeOp::IntegerShiftConst(integer, 1));
            ops.push(NativeOp::IntegerBinaryConst(integer, 1));
        }

        for op in ops {
            let (pops, pushes) = native_op_stack_effect(&op);
            assert_eq!(pushes, 1, "{op:?}");
            let mut program_ops = vec![NativeOp::Const(1.0); pops];
            program_ops.push(op);
            let current_dependencies = match op {
                NativeOp::LoadCurrent(index) => vec![index],
                _ => Vec::new(),
            };
            let prior_current_dependencies = match op {
                NativeOp::LoadPriorCurrent(index) => vec![index],
                _ => Vec::new(),
            };
            let program = NativeProgram::from_ops_for_test(
                program_ops,
                pops.max(1),
                current_dependencies,
                prior_current_dependencies,
            );
            let module = emit_verified_value_program(&program)
                .unwrap_or_else(|error| panic!("missing WASM translation for {op:?}: {error}"));
            assert!(module.len() <= super::super::SHIPPED_MODEL_WASM_CODE_SIZE_BUDGET_BYTES);
        }
    }

    /// Encode the branch form of one value entry into a single-entry module.
    fn branching_value_module(program: &NativeProgram) -> Vec<u8> {
        let ssa = Program::lower(program)
            .expect("lower the postfix program")
            .with_branching_conditionals()
            .expect("re-express conditionals as branches");
        let body = encode_value_body_from_ssa(&ssa).expect("encode the branch-form body");
        let bytes = encode_value_module(body).expect("assemble the module");
        Validator::new()
            .validate_all(&bytes)
            .expect("branch-form module is valid WebAssembly");
        bytes
    }

    #[test]
    fn structured_branches_agree_with_the_select_form_bit_for_bit() {
        // Nested conditionals whose arms own real work, including a helper
        // call the untaken arm must not make.
        let source = program(
            vec![
                NativeOp::LoadParam(0),
                NativeOp::LoadParam(1),
                NativeOp::LoadParam(2),
                NativeOp::LoadParam(3),
                NativeOp::Mul,
                NativeOp::LoadParam(4),
                NativeOp::UnaryMath(UnaryMathOp::Exp),
                NativeOp::IfElse,
                NativeOp::LoadParam(5),
                NativeOp::Sqrt,
                NativeOp::IfElse,
                NativeOp::LoadParam(6),
                NativeOp::Add,
            ],
            4,
        );

        let engine = Engine::default();
        let select_bytes = emit_verified_value_program(&source).expect("encode the select form");
        let branch_bytes = branching_value_module(&source);
        let (mut select_store, select_memory, select_instance) =
            instantiate_value_module(&engine, &select_bytes);
        let (mut branch_store, branch_memory, branch_instance) =
            instantiate_value_module(&engine, &branch_bytes);

        let truthiness = [
            0.0_f64,
            -0.0,
            f64::NAN,
            1.0,
            -1.0,
            f64::INFINITY,
            f64::NEG_INFINITY,
        ];
        for outer in truthiness {
            for inner in truthiness {
                let parameters = [outer, inner, 3.0_f64, 0.5, 2.0, 9.0, -4.5];
                let expected = call_value_entry(
                    &mut select_store,
                    &select_memory,
                    &select_instance,
                    &parameters,
                );
                let actual = call_value_entry(
                    &mut branch_store,
                    &branch_memory,
                    &branch_instance,
                    &parameters,
                );
                assert_eq!(
                    actual.to_bits(),
                    expected.to_bits(),
                    "structured branch and select disagree at outer={outer} inner={inner}"
                );
            }
        }
    }

    /// A loop, structurized into `loop`/`if`/`end` and executed.
    ///
    /// WebAssembly has no branch to an arbitrary label, so this is the one
    /// backend where a back edge changes the shape of the emitted code rather
    /// than the direction of a jump. Running the same fixture the machine
    /// backends run is what proves the structurizer put the `br` at the right
    /// depth and staged the swapping edge through its scratch locals: both
    /// mistakes produce a number, and only the wrong one.
    #[test]
    fn a_loop_is_structurized_and_computes_what_it_says() {
        let engine = Engine::default();
        for (limit, scale, first, second) in [
            (20.0_f64, 3.0_f64, 1.0_f64, 2.0_f64),
            (100.0, 0.5, -4.0, 7.5),
            (1.0, 1.0, 0.25, 0.75),
            (-1.0, 3.0, 6.0, 9.0),
        ] {
            let ssa = Program::loop_fixture_for_test(limit, scale).expect("build the loop program");
            let body = encode_value_body_from_ssa(&ssa).expect("encode the loop body");
            let bytes = encode_value_module(body).expect("assemble the module");
            Validator::new()
                .validate_all(&bytes)
                .expect("the loop module is valid WebAssembly");
            let (mut store, memory, instance) = instantiate_value_module(&engine, &bytes);
            let expected = Program::loop_fixture_expectation(limit, scale, first, second);
            assert_eq!(
                call_value_entry(&mut store, &memory, &instance, &[first, second]).to_bits(),
                expected.to_bits(),
                "loop with limit={limit} scale={scale} first={first} second={second}"
            );
        }
    }

    /// The empty-block loop, structurized and executed.
    ///
    /// The defect this fixture was written for is the native allocator's — a
    /// block parameter bound just before an empty preheader read as if it were
    /// defined inside the loop, so its register was handed to its own reader.
    /// WebAssembly gives every value a local and allocates nothing, so it
    /// cannot have that defect; running the fixture here is what says so
    /// rather than assuming it, and it pins the structurizer against a loop
    /// whose carried value enters through a block with no instructions.
    #[test]
    fn a_loop_carrying_a_value_through_an_empty_block_computes_what_it_says() {
        let engine = Engine::default();
        for (trips, value) in [(3.0_f64, 5.0_f64), (1.0, -2.5), (0.0, 7.0), (6.0, 0.25)] {
            let ssa = Program::empty_block_loop_fixture_for_test(trips)
                .expect("build the empty-block loop");
            let body = encode_value_body_from_ssa(&ssa).expect("encode the loop body");
            let bytes = encode_value_module(body).expect("assemble the module");
            Validator::new()
                .validate_all(&bytes)
                .expect("the loop module is valid WebAssembly");
            let (mut store, memory, instance) = instantiate_value_module(&engine, &bytes);
            let expected = Program::empty_block_loop_fixture_expectation(trips, value);
            assert_eq!(
                call_value_entry(&mut store, &memory, &instance, &[value]).to_bits(),
                expected.to_bits(),
                "empty-block loop with trips={trips} value={value}"
            );
        }
    }

    #[test]
    fn a_structured_branch_binds_its_join_parameter_from_both_arms() {
        let source = program(
            vec![
                NativeOp::LoadParam(0),
                NativeOp::LoadParam(1),
                NativeOp::Sqrt,
                NativeOp::LoadParam(2),
                NativeOp::UnaryMath(UnaryMathOp::Exp),
                NativeOp::IfElse,
            ],
            3,
        );
        let ssa = Program::lower(&source)
            .expect("lower")
            .with_branching_conditionals()
            .expect("split");
        assert_eq!(ssa.blocks().len(), 4);
        assert_eq!(ssa.block_parameter_count(), 1);
        assert_eq!(
            ssa.branch_join(&ssa.blocks()[0])
                .expect("join block")
                .index(),
            3
        );

        let engine = Engine::default();
        let bytes = branching_value_module(&source);
        let (mut store, memory, instance) = instantiate_value_module(&engine, &bytes);
        assert_eq!(
            call_value_entry(&mut store, &memory, &instance, &[1.0, 9.0, 2.0]).to_bits(),
            3.0_f64.to_bits(),
            "the taken arm's square root reaches the join parameter"
        );
        assert_eq!(
            call_value_entry(&mut store, &memory, &instance, &[0.0, 9.0, 0.0]).to_bits(),
            1.0_f64.to_bits(),
            "the untaken arm's exponential reaches the join parameter"
        );
    }
}
