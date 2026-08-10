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
use crate::jit::expr::{
    BinaryMathOp, CompareOp, ExtremumOp, IntegerBinaryOp, LogicalOp, NativeOp, NativeProgram,
    UnaryMathOp, VoltageNode,
};
use crate::jit::ssa::{Instruction, Program};

pub(crate) const WASM_JIT_EVAL_HELPER_IMPORT: &str = "eval_op_v1";
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
const MAX_RUNTIME_LOOP_ITERATIONS: i32 = 100_000;

const ENTRY_TYPE_INDEX: u32 = 0;
const HELPER_TYPE_INDEX: u32 = 1;
const MATH1_TYPE_INDEX: u32 = 2;
const MATH2_TYPE_INDEX: u32 = 3;
const HELPER_FUNCTION_INDEX: u32 = 0;
const MATH1_FUNCTION_INDEX: u32 = 1;
const MATH2_FUNCTION_INDEX: u32 = 2;
/// Imported functions occupy the low indices, so generated entries start after
/// the whole capability surface.
const ENTRY_FUNCTION_INDEX: u32 = 3;
/// Entry signature plus the three capability signatures.
const CAPABILITY_TYPE_COUNT: u32 = 4;
/// Linear memory plus the three imported capability functions.
const CAPABILITY_IMPORT_COUNT: u32 = 4;
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

#[cfg(test)]
pub(crate) fn encode_value_program(program: &NativeProgram) -> WasmJitResult<Vec<u8>> {
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

    let body = encode_value_body(program)?;
    let mut code = CodeSection::new();
    code.function(&body);
    module.section(&code);
    Ok(module.finish())
}

pub(crate) fn emit_verified_value_program_set(
    programs: &[&NativeProgram],
) -> WasmJitResult<Vec<u8>> {
    let bytes = encode_model_program_set(programs, &[])?;
    verify_value_program_set(&bytes, programs)?;
    Ok(bytes)
}

pub(crate) fn emit_verified_model_module(
    programs: &[&NativeProgram],
    kernels: &[WasmAssignmentKernel],
) -> WasmJitResult<Vec<u8>> {
    let bytes = encode_model_program_set(programs, kernels)?;
    verify_model_module(&bytes, programs, kernels)?;
    Ok(bytes)
}

pub(crate) fn verify_value_program_set(
    bytes: &[u8],
    expected_programs: &[&NativeProgram],
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
    if bytes != encode_model_program_set(expected_programs, &[])? {
        return Err(WasmJitError::Contract(
            "model value-entry module does not match deterministic translation of its authenticated SSA"
                .into(),
        ));
    }

    let expected_count = u32::try_from(expected_programs.len())
        .map_err(|_| WasmJitError::Encoding("model entry count exceeds u32".into()))?;
    verify_value_module_shape(bytes, expected_count, &[], false)
}

fn verify_model_module(
    bytes: &[u8],
    expected_programs: &[&NativeProgram],
    kernels: &[WasmAssignmentKernel],
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
    if bytes != encode_model_program_set(expected_programs, kernels)? {
        return Err(WasmJitError::Contract(
            "model module does not match deterministic translation of its authenticated plan"
                .into(),
        ));
    }
    let scalar_count = u32::try_from(expected_programs.len())
        .map_err(|_| WasmJitError::Encoding("model entry count exceeds u32".into()))?;
    let kernel_exports = kernels
        .iter()
        .map(|kernel| kernel.export_name)
        .collect::<Vec<_>>();
    verify_value_module_shape(bytes, scalar_count, &kernel_exports, false)
}

fn encode_model_program_set(
    programs: &[&NativeProgram],
    kernels: &[WasmAssignmentKernel],
) -> WasmJitResult<Vec<u8>> {
    let function_count = u32::try_from(programs.len())
        .map_err(|_| WasmJitError::Encoding("model entry count exceeds u32".into()))?;
    let kernel_count = u32::try_from(kernels.len())
        .map_err(|_| WasmJitError::Encoding("assignment kernel count exceeds u32".into()))?;
    let mut module = Module::new();

    encode_capability_types(&mut module);
    encode_capability_imports(&mut module);

    let mut functions = FunctionSection::new();
    for _ in 0..function_count {
        functions.function(ENTRY_TYPE_INDEX);
    }
    for _ in 0..kernel_count {
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
    module.section(&exports);

    let mut contract = Vec::with_capacity(20);
    contract.extend_from_slice(&WASM_JIT_ABI_VERSION.to_le_bytes());
    contract.extend_from_slice(&WASM_JIT_EMITTER_VERSION.to_le_bytes());
    contract.extend_from_slice(&WASM_JIT_EVAL_FRAME_BYTES.to_le_bytes());
    contract.extend_from_slice(&function_count.to_le_bytes());
    contract.extend_from_slice(&kernel_count.to_le_bytes());
    module.section(&CustomSection {
        name: Cow::Borrowed(WASM_JIT_CONTRACT_SECTION),
        data: Cow::Owned(contract),
    });

    let mut code = CodeSection::new();
    for program in programs {
        code.function(&encode_value_body(program)?);
    }
    for kernel in kernels {
        code.function(&encode_assignment_kernel(kernel, function_count)?);
    }
    module.section(&code);
    Ok(module.finish())
}

fn encode_value_body(program: &NativeProgram) -> WasmJitResult<Function> {
    let ssa = Program::lower(program).map_err(|error| WasmJitError::Encoding(error.to_string()))?;
    let local_count = u32::try_from(ssa.instructions().len())
        .map_err(|_| WasmJitError::Encoding("SSA local count exceeds u32".into()))?;
    let locals = (local_count != 0)
        .then_some((local_count, ValType::F64))
        .into_iter();
    let mut body = Function::new(locals);
    emit_frame_guard(&mut body);
    emit_clear_error_status(&mut body);
    for instruction in ssa.instructions() {
        emit_instruction(&mut body, instruction)?;
        body.instruction(&WasmInstruction::LocalSet(value_local(
            instruction.result().index(),
        )?));
    }
    body.instruction(&WasmInstruction::LocalGet(FRAME_LOCAL));
    body.instruction(&WasmInstruction::LocalGet(value_local(
        ssa.result().index(),
    )?));
    body.instruction(&WasmInstruction::F64Store(f64_mem(FRAME_RESULT_OFFSET)));
    body.instruction(&WasmInstruction::I32Const(WASM_JIT_STATUS_OK));
    body.instruction(&WasmInstruction::End);
    Ok(body)
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

    verify_value_module_shape(bytes, 1, &[], true)
}

fn verify_value_module_shape(
    bytes: &[u8],
    expected_scalar_count: u32,
    kernel_exports: &[&str],
    single_export: bool,
) -> WasmJitResult<()> {
    let expected_kernel_count = u32::try_from(kernel_exports.len())
        .map_err(|_| WasmJitError::Contract("assignment kernel count exceeds u32".into()))?;
    let expected_count = expected_scalar_count
        .checked_add(expected_kernel_count)
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
                let expected: [(&[wasmparser::ValType], &[wasmparser::ValType]); 4] = [
                    (&[I32], &[I32]),
                    (&[I32, I32, I32, I32, I64, F64, F64, F64, F64, F64], &[F64]),
                    (&[I32, F64], &[F64]),
                    (&[I32, F64, F64], &[F64]),
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
                if flattened.len() != 4 {
                    return Err(WasmJitError::Contract(
                        "value module must import exactly memory, eval_op_v1, math1_v1, and math2_v1"
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
                let mut expected = Vec::with_capacity(if single_export { 12 } else { 20 });
                expected.extend_from_slice(&WASM_JIT_ABI_VERSION.to_le_bytes());
                expected.extend_from_slice(&WASM_JIT_EMITTER_VERSION.to_le_bytes());
                expected.extend_from_slice(&WASM_JIT_EVAL_FRAME_BYTES.to_le_bytes());
                if !single_export {
                    expected.extend_from_slice(&expected_scalar_count.to_le_bytes());
                    expected.extend_from_slice(&expected_kernel_count.to_le_bytes());
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
        _ => emit_helper_call(body, op, operands, instruction.result())?,
    };
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

#[derive(Clone, Copy)]
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
        NativeOp::ZiState(index) => set_index(&mut descriptor, 421, index)?,
        NativeOp::TimerState(index) => set_index(&mut descriptor, 422, index)?,
        NativeOp::TransitionState(index) => set_index(&mut descriptor, 423, index)?,
        NativeOp::SlewState(index) => set_index(&mut descriptor, 424, index)?,
        NativeOp::AbsDelayState(index) => set_index(&mut descriptor, 425, index)?,
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

/// Bind the frame-free transcendental capabilities for an execution test.
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
    fn model_module_emits_direct_indexed_and_bounded_loop_assignment_kernels() {
        let value = program(vec![NativeOp::Const(7.0)], 1);
        let index = program(vec![NativeOp::Const(1.0)], 1);
        let condition = program(vec![NativeOp::Const(0.0)], 1);
        let programs = [&value, &index, &condition];
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
        let first = emit_verified_model_module(&programs, &kernels)
            .expect("encode complete assignment kernel");
        let second = emit_verified_model_module(&programs, &kernels)
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
        let programs = [&seven, &index, &condition, &zero];
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
        let bytes = emit_verified_model_module(&programs, &kernels)
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
        let program_refs = programs.iter().collect::<Vec<_>>();
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

    #[test]
    fn verifier_rejects_valid_wasm_tampering_against_authenticated_ssa() {
        let expected = program(vec![NativeOp::Const(2.0)], 1);
        let bytes = emit_verified_value_program(&expected).expect("encode constant module");
        let alternate = program(vec![NativeOp::Const(3.0)], 1);
        assert!(verify_value_program(&bytes, &alternate).is_err());
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
            NativeOp::ZiState(0),
            NativeOp::TimerState(0),
            NativeOp::TransitionState(0),
            NativeOp::SlewState(0),
            NativeOp::AbsDelayState(0),
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
}
