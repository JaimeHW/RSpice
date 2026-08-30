//! Primary-module implementation of the versioned helper capabilities.

use crate::codegen::Instruction;
use crate::jit::expr::{
    BinaryMathOp, ExtremumOp, IntegerBinaryOp, UnaryMathOp, constant_binary_math,
    constant_dynamic_variable_slot, constant_extremum, constant_integer_binary,
    constant_unary_math,
};
use crate::vm::{Vm, VmContext, execute_zi_state, execute_zi_state_derivative};

#[cfg(target_arch = "wasm32")]
use std::cell::{Cell, RefCell};

#[cfg(target_arch = "wasm32")]
use super::abi::{
    WASM_JIT_FRAME_MAGIC, WASM_JIT_MAX_EVAL_FRAME_BYTES, WASM_JIT_SLICE_OPERANDS_OFFSET,
    WASM_JIT_STATUS_ABI_MISMATCH, WASM_JIT_STATUS_RUNTIME_ERROR, WasmJitEvalFrame,
};
use super::abi::{WASM_JIT_MAX_SLICE_OPERANDS, decode_zi_layout_descriptor};
#[cfg(target_arch = "wasm32")]
use super::{WASM_JIT_ABI_VERSION, WASM_JIT_EVAL_FRAME_BYTES};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum HelperError {
    InvalidOpcode,
    InvalidDynamicIndex,
    InvalidIntegerShift,
    StatefulRuntimeUnavailable,
    StatefulRuntimeFailed,
}

/// Owned runtime state used while a generated secondary module is active.
///
/// Ownership is moved into the dispatch registry before JavaScript invokes a
/// secondary module. Consequently a re-entrant helper call never aliases an
/// outstanding Rust borrow of the simulator state.
#[derive(Debug)]
pub struct WasmJitRuntimeSession {
    context: VmContext,
    scratch_stack: Vec<f64>,
    last_error: Option<String>,
}

impl WasmJitRuntimeSession {
    pub fn new(context: VmContext) -> Self {
        Self {
            context,
            scratch_stack: Vec::with_capacity(8),
            last_error: None,
        }
    }

    pub fn context(&self) -> &VmContext {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut VmContext {
        &mut self.context
    }

    pub fn into_context(self) -> VmContext {
        self.context
    }

    pub fn take_error(&mut self) -> Option<String> {
        self.last_error.take()
    }

    fn fail(&mut self, detail: impl Into<String>) -> HelperError {
        if self.last_error.is_none() {
            self.last_error = Some(detail.into());
        }
        HelperError::StatefulRuntimeFailed
    }

    fn execute_instruction(
        &mut self,
        instruction: Instruction,
        operands: &[f64],
    ) -> Result<f64, HelperError> {
        self.scratch_stack.clear();
        self.scratch_stack.extend_from_slice(operands);
        let mut vm = Vm {
            context: &mut self.context,
            stack: std::mem::take(&mut self.scratch_stack),
        };
        let result = vm
            .execute_instruction(&instruction)
            .map_err(|error| error.to_string());
        let value = match result {
            Ok(()) if vm.stack.len() == 1 => Ok(vm.stack[0]),
            Ok(()) => Err(format!(
                "WASM JIT helper {:?} produced {} stack values; expected one",
                instruction,
                vm.stack.len()
            )),
            Err(error) => Err(format!("WASM JIT helper {instruction:?} failed: {error}")),
        };
        self.scratch_stack = vm.stack;
        value.map_err(|detail| self.fail(detail))
    }
}

pub(super) fn evaluate_helper(
    opcode: i32,
    aux0: i32,
    aux1: i32,
    aux2: i64,
    operands: [f64; 5],
    variables: &[f64],
) -> Result<f64, HelperError> {
    evaluate_helper_with_session(opcode, aux0, aux1, aux2, operands, variables, None)
}

fn evaluate_helper_with_session(
    opcode: i32,
    aux0: i32,
    aux1: i32,
    aux2: i64,
    operands: [f64; 5],
    variables: &[f64],
    session: Option<&mut WasmJitRuntimeSession>,
) -> Result<f64, HelperError> {
    match opcode {
        1 => {
            let slot = dynamic_slot(aux0, aux1, aux2, operands[0], variables.len())?;
            variables
                .get(slot)
                .copied()
                .ok_or(HelperError::InvalidDynamicIndex)
        }
        // Assignment kernels request an absolute variable-array index. The
        // generated caller performs the store only after this helper succeeds.
        2 => dynamic_slot(aux0, aux1, aux2, operands[0], variables.len()).map(|slot| slot as f64),
        10..=11 => Ok(constant_extremum(
            extremum(opcode - 10)?,
            operands[0],
            operands[1],
        )),
        12..=13 => Ok(constant_extremum(
            extremum(opcode - 12)?,
            operands[0],
            f64::from_bits(aux2 as u64),
        )),
        14..=15 => Ok(constant_extremum(
            extremum(opcode - 14)?,
            f64::from_bits(aux2 as u64),
            operands[0],
        )),
        100..=118 => Ok(constant_unary_math(unary(opcode - 100)?, operands[0])),
        200..=203 => Ok(constant_binary_math(
            binary(opcode - 200)?,
            operands[0],
            operands[1],
        )),
        300 => Ok((operands[0] as i64) as f64),
        301..=305 => integer(integer_op(opcode - 301)?, operands[0], operands[1]),
        310..=314 => integer(integer_op(opcode - 310)?, operands[0], f64::from(aux0)),
        320..=324 => integer(integer_op(opcode - 320)?, operands[0], aux2 as f64),
        // Noise source expressions contribute only to the separately emitted
        // noise PSD/exponent entries. Their large-signal value is exactly
        // zero in the bytecode, x64, and AArch64 runtimes.
        430..=431 => Ok(0.0),
        400..=65_535 => evaluate_stateful_helper(
            opcode,
            aux0,
            aux1,
            aux2,
            operands,
            session.ok_or(HelperError::StatefulRuntimeUnavailable)?,
        ),
        _ => Err(HelperError::InvalidOpcode),
    }
}

fn evaluate_stateful_helper(
    opcode: i32,
    aux0: i32,
    _aux1: i32,
    _aux2: i64,
    operands: [f64; 5],
    session: &mut WasmJitRuntimeSession,
) -> Result<f64, HelperError> {
    if matches!(opcode, 421 | 429) {
        return Err(session.fail(
            "WASM JIT Zi operation reached the five-operand scalar helper; the slice helper is required",
        ));
    }
    let index = u32::try_from(aux0)
        .ok()
        .and_then(|value| usize::try_from(value).ok())
        .ok_or_else(|| session.fail("WASM JIT stateful helper has a negative slot index"))?;
    let instruction = match opcode {
        400 => {
            require_slot(
                session,
                index,
                session.context.lookup_tables.len(),
                "lookup table",
            )?;
            (Instruction::TableLookup(index), 1)
        }
        401 => {
            require_slot(
                session,
                index,
                session.context.lookup_tables.len(),
                "lookup table derivative",
            )?;
            (Instruction::TableDerivative(index), 1)
        }
        410 => {
            require_limit_state(session, index)?;
            (Instruction::LimitState(index), 2)
        }
        411 => return limiter_previous(session, index, operands[0]),
        412 => return limiter_store(session, index, operands[0], operands[1]),
        420 => {
            require_slot(
                session,
                index,
                session.context.laplace_filters.len(),
                "Laplace filter",
            )?;
            (Instruction::LaplaceState(index), 1)
        }
        422 => (Instruction::TimerState(index), 4),
        423 => {
            require_slot(
                session,
                index,
                session.context.transition_filters.len(),
                "transition filter",
            )?;
            (Instruction::TransitionState(index), 4)
        }
        424 => {
            require_slot(
                session,
                index,
                session.context.slew_filters.len(),
                "slew filter",
            )?;
            (Instruction::SlewState(index), 3)
        }
        425 => {
            require_slot(
                session,
                index,
                session.context.delay_buffers.len(),
                "delay buffer",
            )?;
            (Instruction::AbsDelayState(index), 2)
        }
        426 => {
            require_slot(
                session,
                index,
                session.context.cross_detectors.len(),
                "cross detector",
            )?;
            (Instruction::CrossState(index), 5)
        }
        427 => {
            require_slot(
                session,
                index,
                session.context.cross_detectors.len(),
                "above detector",
            )?;
            (Instruction::AboveState(index), 4)
        }
        428 => {
            require_slot(
                session,
                index,
                session.context.cross_detectors.len(),
                "last-crossing detector",
            )?;
            (Instruction::LastCrossingState(index), 2)
        }
        440 => {
            require_integration_state(session, index)?;
            (Instruction::DdtState(index), 1)
        }
        441 => (Instruction::DdtJacobian, 1),
        442 => {
            require_integration_state(session, index)?;
            (Instruction::IdtState(index), 2)
        }
        443 => (Instruction::IdtJacobian, 1),
        444 => {
            require_integration_state(session, index)?;
            (Instruction::IdtModState(index), 4)
        }
        _ => return Err(HelperError::InvalidOpcode),
    };
    let operands = operands.get(..instruction.1).ok_or_else(|| {
        session.fail(format!(
            "WASM JIT scalar helper opcode {opcode} requires {} operands, exceeding its five-lane ABI",
            instruction.1
        ))
    })?;
    session.execute_instruction(instruction.0, operands)
}

fn evaluate_slice_helper_with_session(
    opcode: i32,
    aux0: i32,
    aux1: i32,
    aux2: i64,
    operands: &[f64],
    session: &mut WasmJitRuntimeSession,
) -> Result<f64, HelperError> {
    if operands.len() > WASM_JIT_MAX_SLICE_OPERANDS {
        return Err(session.fail(format!(
            "WASM JIT slice helper received {} operands, exceeding the bounded maximum {}",
            operands.len(),
            WASM_JIT_MAX_SLICE_OPERANDS
        )));
    }
    let storage = match opcode {
        421 => "ZI filter",
        429 => "ZI derivative filter",
        _ => {
            return Err(session.fail(format!(
                "WASM JIT slice helper opcode {opcode} is not allowlisted"
            )));
        }
    };
    if aux2 != 0 {
        return Err(session.fail("WASM JIT Zi slice helper received nonzero reserved metadata"));
    }
    let filter_id = u32::try_from(aux0)
        .ok()
        .and_then(|value| usize::try_from(value).ok())
        .ok_or_else(|| session.fail("WASM JIT Zi slice helper has a negative filter slot"))?;
    let layout = decode_zi_layout_descriptor(filter_id, aux1)
        .ok_or_else(|| session.fail("WASM JIT Zi slice helper received an invalid layout"))?;
    let operand_count = layout
        .validate_operand_budget()
        .map_err(|error| session.fail(format!("WASM JIT Zi layout rejected: {error}")))?;
    if operands.len() != operand_count {
        return Err(session.fail(format!(
            "WASM JIT Zi slice helper layout requires {} operands, received {}",
            operand_count,
            operands.len()
        )));
    }
    require_slot(
        session,
        filter_id,
        session.context.zi_filters.len(),
        storage,
    )?;
    let result = match opcode {
        421 => execute_zi_state(&mut session.context, layout, operands),
        429 => execute_zi_state_derivative(&mut session.context, layout, operands),
        _ => unreachable!(),
    };
    result
        .map_err(|error| format!("WASM JIT helper Zi operation failed: {error}"))
        .map_err(|detail| session.fail(detail))
}

fn require_slot(
    session: &mut WasmJitRuntimeSession,
    index: usize,
    len: usize,
    storage: &str,
) -> Result<(), HelperError> {
    if index < len {
        Ok(())
    } else {
        Err(session.fail(format!(
            "WASM JIT {storage} slot {index} is outside preallocated length {len}"
        )))
    }
}

fn require_limit_state(
    session: &mut WasmJitRuntimeSession,
    index: usize,
) -> Result<(), HelperError> {
    let values_len = session.context.state_values.len();
    let initialized_len = session.context.state_initialized.len();
    if index < values_len && index < initialized_len {
        Ok(())
    } else {
        Err(session.fail(format!(
            "WASM JIT limiter state {index} exceeds value/init lengths {values_len}/{initialized_len}"
        )))
    }
}

fn require_integration_state(
    session: &mut WasmJitRuntimeSession,
    index: usize,
) -> Result<(), HelperError> {
    let context = &session.context;
    let valid = [
        context.state_values.len(),
        context.state_values_prev.len(),
        context.state_values_older.len(),
        context.state_derivatives.len(),
        context.state_derivatives_prev.len(),
        context.state_initialized.len(),
    ]
    .into_iter()
    .all(|len| index < len);
    if valid {
        Ok(())
    } else {
        Err(session.fail(format!(
            "WASM JIT integration state {index} is not fully preallocated"
        )))
    }
}

fn limiter_previous(
    session: &mut WasmJitRuntimeSession,
    index: usize,
    proposed: f64,
) -> Result<f64, HelperError> {
    if !session.context.evaluation_mode.limiting_enabled() {
        return Ok(proposed);
    }
    require_limit_state(session, index)?;
    Ok(if session.context.state_initialized[index] {
        session.context.state_values[index]
    } else {
        proposed
    })
}

fn limiter_store(
    session: &mut WasmJitRuntimeSession,
    index: usize,
    proposed: f64,
    candidate: f64,
) -> Result<f64, HelperError> {
    if !session.context.evaluation_mode.limiting_enabled() {
        return Ok(proposed);
    }
    require_limit_state(session, index)?;
    session.context.limiter_active |= u8::from(candidate != proposed);
    session.context.state_values[index] = candidate;
    session.context.state_initialized[index] = true;
    Ok(candidate)
}

fn dynamic_slot(
    base: i32,
    len: i32,
    lower: i64,
    raw_index: f64,
    variables_len: usize,
) -> Result<usize, HelperError> {
    let base = u32::try_from(base)
        .ok()
        .and_then(|value| usize::try_from(value).ok())
        .ok_or(HelperError::InvalidDynamicIndex)?;
    let len = u32::try_from(len)
        .ok()
        .and_then(|value| usize::try_from(value).ok())
        .ok_or(HelperError::InvalidDynamicIndex)?;
    let slot = constant_dynamic_variable_slot(raw_index, base, len, lower)
        .ok_or(HelperError::InvalidDynamicIndex)?;
    (slot < variables_len)
        .then_some(slot)
        .ok_or(HelperError::InvalidDynamicIndex)
}

fn integer(op: IntegerBinaryOp, lhs: f64, rhs: f64) -> Result<f64, HelperError> {
    constant_integer_binary(op, lhs, rhs).ok_or(HelperError::InvalidIntegerShift)
}

fn extremum(code: i32) -> Result<ExtremumOp, HelperError> {
    match code {
        0 => Ok(ExtremumOp::Min),
        1 => Ok(ExtremumOp::Max),
        _ => Err(HelperError::InvalidOpcode),
    }
}

fn binary(code: i32) -> Result<BinaryMathOp, HelperError> {
    match code {
        0 => Ok(BinaryMathOp::Pow),
        1 => Ok(BinaryMathOp::Atan2),
        2 => Ok(BinaryMathOp::Hypot),
        3 => Ok(BinaryMathOp::Mod),
        _ => Err(HelperError::InvalidOpcode),
    }
}

fn integer_op(code: i32) -> Result<IntegerBinaryOp, HelperError> {
    match code {
        0 => Ok(IntegerBinaryOp::Shl),
        1 => Ok(IntegerBinaryOp::Shr),
        2 => Ok(IntegerBinaryOp::BitAnd),
        3 => Ok(IntegerBinaryOp::BitOr),
        4 => Ok(IntegerBinaryOp::BitXor),
        _ => Err(HelperError::InvalidOpcode),
    }
}

fn unary(code: i32) -> Result<UnaryMathOp, HelperError> {
    match code {
        0 => Ok(UnaryMathOp::Exp),
        1 => Ok(UnaryMathOp::Log),
        2 => Ok(UnaryMathOp::Log10),
        3 => Ok(UnaryMathOp::Sin),
        4 => Ok(UnaryMathOp::Cos),
        5 => Ok(UnaryMathOp::Tan),
        6 => Ok(UnaryMathOp::Sinh),
        7 => Ok(UnaryMathOp::Cosh),
        8 => Ok(UnaryMathOp::Tanh),
        9 => Ok(UnaryMathOp::Asinh),
        10 => Ok(UnaryMathOp::Acosh),
        11 => Ok(UnaryMathOp::Atanh),
        12 => Ok(UnaryMathOp::Limexp),
        13 => Ok(UnaryMathOp::LimitedExp),
        14 => Ok(UnaryMathOp::Asin),
        15 => Ok(UnaryMathOp::Acos),
        16 => Ok(UnaryMathOp::Atan),
        17 => Ok(UnaryMathOp::Floor),
        18 => Ok(UnaryMathOp::Ceil),
        _ => Err(HelperError::InvalidOpcode),
    }
}

#[cfg(target_arch = "wasm32")]
struct ActiveRuntimeSession {
    frame_offset: u32,
    token: u32,
    generation: u32,
    session: WasmJitRuntimeSession,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static ACTIVE_RUNTIME_SESSION: RefCell<Option<ActiveRuntimeSession>> = const { RefCell::new(None) };
    static NEXT_RUNTIME_GENERATION: Cell<u32> = const { Cell::new(0) };
}

/// Invoke one secondary-module dispatch with exclusive ownership of its
/// stateful runtime context installed in the primary module.
///
/// The returned tuple always gives ownership of the session back to the
/// caller, including validation failures. `invoke` must synchronously call a
/// generated module; no session capability survives this function.
#[cfg(target_arch = "wasm32")]
pub fn with_runtime_session<R>(
    frame_offset: u32,
    session: WasmJitRuntimeSession,
    invoke: impl FnOnce() -> R,
) -> (Result<R, String>, WasmJitRuntimeSession) {
    if ACTIVE_RUNTIME_SESSION.with(|active| active.borrow().is_some()) {
        return (
            Err("WASM JIT runtime session dispatch cannot be nested".to_owned()),
            session,
        );
    }
    let Some(frame) = (unsafe { validated_frame(frame_offset) }) else {
        return (
            Err("WASM JIT runtime session received an invalid evaluation frame".to_owned()),
            session,
        );
    };
    let generation = NEXT_RUNTIME_GENERATION.with(|next| {
        let generation = next.get().wrapping_add(1).max(1);
        next.set(generation);
        generation
    });
    let token = generation.rotate_left(13) ^ WASM_JIT_FRAME_MAGIC;
    frame.session_token = token;
    frame.session_generation = generation;
    ACTIVE_RUNTIME_SESSION.with(|active| {
        *active.borrow_mut() = Some(ActiveRuntimeSession {
            frame_offset,
            token,
            generation,
            session,
        });
    });

    let result = invoke();
    let active = ACTIVE_RUNTIME_SESSION
        .with(|active| active.borrow_mut().take())
        .expect("active WASM JIT runtime session is owned until synchronous dispatch returns");
    if let Some(frame) = unsafe { validated_frame(frame_offset) }
        && frame.session_token == token
        && frame.session_generation == generation
    {
        frame.session_token = 0;
        frame.session_generation = 0;
    }
    (Ok(result), active.session)
}

/// Host capability imported by generated secondary modules.
///
/// The JavaScript worker forwards the call to this primary-module export. A
/// generated module has no access to arbitrary JavaScript or DOM capabilities.
#[cfg(target_arch = "wasm32")]
#[allow(clippy::too_many_arguments)]
pub fn eval_op_v1(
    frame_offset: u32,
    opcode: i32,
    aux0: i32,
    aux1: i32,
    aux2: i64,
    operand0: f64,
    operand1: f64,
    operand2: f64,
    operand3: f64,
    operand4: f64,
) -> f64 {
    let Some((frame, variables)) = (unsafe { validated_frame_and_variables(frame_offset) }) else {
        return 0.0;
    };
    let operands = [operand0, operand1, operand2, operand3, operand4];
    let result = if is_stateful_opcode(opcode) {
        let token = frame.session_token;
        let generation = frame.session_generation;
        ACTIVE_RUNTIME_SESSION.with(|active| {
            let mut active = active.borrow_mut();
            let active = active
                .as_mut()
                .ok_or(HelperError::StatefulRuntimeUnavailable)?;
            if token == 0
                || generation == 0
                || active.frame_offset != frame_offset
                || active.token != token
                || active.generation != generation
            {
                return Err(HelperError::StatefulRuntimeUnavailable);
            }
            evaluate_helper_with_session(
                opcode,
                aux0,
                aux1,
                aux2,
                operands,
                variables,
                Some(&mut active.session),
            )
        })
    } else {
        evaluate_helper(opcode, aux0, aux1, aux2, operands, variables)
    };
    match result {
        Ok(value) => value,
        Err(_) => {
            frame.error_status = WASM_JIT_STATUS_RUNTIME_ERROR;
            0.0
        }
    }
}

/// Variable-arity host capability for bounded stateful operations.
///
/// Operands reside in the authenticated trailing region of `frame_offset`;
/// callers supply only the count, never a linear-memory pointer. ABI v5
/// allowlists Zi value and derivative operations on this capability.
#[cfg(target_arch = "wasm32")]
pub fn eval_op_slice_v1(
    frame_offset: u32,
    opcode: i32,
    aux0: i32,
    aux1: i32,
    aux2: i64,
    operand_count: i32,
) -> f64 {
    let operand_count = match u32::try_from(operand_count)
        .ok()
        .and_then(|count| usize::try_from(count).ok())
        .filter(|count| *count <= WASM_JIT_MAX_SLICE_OPERANDS)
    {
        Some(count) => count,
        None => {
            unsafe { record_frame_status(frame_offset, WASM_JIT_STATUS_ABI_MISMATCH) };
            return 0.0;
        }
    };
    let Some((frame, operands)) =
        (unsafe { validated_frame_and_slice_operands(frame_offset, operand_count) })
    else {
        unsafe { record_frame_status(frame_offset, WASM_JIT_STATUS_ABI_MISMATCH) };
        return 0.0;
    };
    let token = frame.session_token;
    let generation = frame.session_generation;
    let result = ACTIVE_RUNTIME_SESSION.with(|active| {
        let mut active = active.borrow_mut();
        let active = active
            .as_mut()
            .ok_or(HelperError::StatefulRuntimeUnavailable)?;
        if token == 0
            || generation == 0
            || active.frame_offset != frame_offset
            || active.token != token
            || active.generation != generation
        {
            return Err(HelperError::StatefulRuntimeUnavailable);
        }
        evaluate_slice_helper_with_session(opcode, aux0, aux1, aux2, operands, &mut active.session)
    });
    match result {
        Ok(value) => value,
        Err(_) => {
            frame.error_status = WASM_JIT_STATUS_RUNTIME_ERROR;
            0.0
        }
    }
}

/// Frame-free unary transcendental capability imported by generated modules.
///
/// Deliberately not `wasm32`-gated and deliberately not taking a frame: this
/// is the hot path, it reads no simulator state, and it is host-testable
/// against the same `constant_unary_math` the bytecode and native backends
/// call. An opcode outside the emitted range cannot be produced by the
/// emitter; returning NaN for one keeps the failure loud, because every
/// consumer of a value entry rejects a non-finite result with a typed error.
pub fn math1_v1(opcode: i32, value: f64) -> f64 {
    match unary(opcode - 100) {
        Ok(op) => constant_unary_math(op, value),
        Err(_) => f64::NAN,
    }
}

/// Frame-free binary transcendental capability. See [`math1_v1`].
pub fn math2_v1(opcode: i32, left: f64, right: f64) -> f64 {
    match binary(opcode - 200) {
        Ok(op) => constant_binary_math(op, left, right),
        Err(_) => f64::NAN,
    }
}

#[cfg(target_arch = "wasm32")]
fn is_stateful_opcode(opcode: i32) -> bool {
    matches!(opcode, 400..=429 | 440..=444)
}

#[cfg(target_arch = "wasm32")]
unsafe fn validated_frame(frame_offset: u32) -> Option<&'static mut WasmJitEvalFrame> {
    let memory_bytes = core::arch::wasm32::memory_size(0).checked_mul(65_536)?;
    let frame = unsafe { validated_frame_header(frame_offset, memory_bytes) }?;
    if frame.byte_len < WASM_JIT_EVAL_FRAME_BYTES || frame.byte_len > WASM_JIT_MAX_EVAL_FRAME_BYTES
    {
        return None;
    }
    let frame_start = usize::try_from(frame_offset).ok()?;
    let frame_end = frame_start.checked_add(usize::try_from(frame.byte_len).ok()?)?;
    if frame_end > memory_bytes {
        return None;
    }
    Some(frame)
}

#[cfg(target_arch = "wasm32")]
unsafe fn validated_frame_header(
    frame_offset: u32,
    memory_bytes: usize,
) -> Option<&'static mut WasmJitEvalFrame> {
    let frame_start = usize::try_from(frame_offset).ok()?;
    let header_end = frame_start.checked_add(WASM_JIT_EVAL_FRAME_BYTES as usize)?;
    if frame_start % align_of::<WasmJitEvalFrame>() != 0 || header_end > memory_bytes {
        return None;
    }
    let frame = unsafe { &mut *(frame_start as *mut WasmJitEvalFrame) };
    if frame.magic != WASM_JIT_FRAME_MAGIC || frame.abi_version != WASM_JIT_ABI_VERSION {
        return None;
    }
    Some(frame)
}

#[cfg(target_arch = "wasm32")]
unsafe fn record_frame_status(frame_offset: u32, status: i32) {
    let Some(memory_bytes) = core::arch::wasm32::memory_size(0).checked_mul(65_536) else {
        return;
    };
    if let Some(frame) = unsafe { validated_frame_header(frame_offset, memory_bytes) } {
        frame.error_status = status;
    }
}

#[cfg(target_arch = "wasm32")]
unsafe fn validated_frame_and_slice_operands(
    frame_offset: u32,
    operand_count: usize,
) -> Option<(&'static mut WasmJitEvalFrame, &'static [f64])> {
    if operand_count > WASM_JIT_MAX_SLICE_OPERANDS {
        return None;
    }
    let frame = unsafe { validated_frame(frame_offset) }?;
    let operand_bytes = operand_count.checked_mul(size_of::<f64>())?;
    let required_len = usize::try_from(WASM_JIT_SLICE_OPERANDS_OFFSET)
        .ok()?
        .checked_add(operand_bytes)?;
    if usize::try_from(frame.byte_len).ok()? < required_len {
        return None;
    }
    let operands_start = usize::try_from(frame_offset)
        .ok()?
        .checked_add(usize::try_from(WASM_JIT_SLICE_OPERANDS_OFFSET).ok()?)?;
    let operands = if operand_count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(operands_start as *const f64, operand_count) }
    };
    Some((frame, operands))
}

/// Last frame that passed full validation, and the memory size it was checked
/// against.
///
/// Every helper call used to re-derive the frame and variable slices from raw
/// offsets: a `memory.size` probe, a magic and ABI check, and an overlap test,
/// per `exp()`. One evaluation drives thousands of those against the same
/// frame. Caching the validated result collapses that to once per frame, and
/// the guard is exact rather than heuristic — a changed offset, a grown
/// memory, or a rewritten frame header all miss and revalidate.
#[cfg(target_arch = "wasm32")]
struct ValidatedFrameCache {
    frame_offset: u32,
    memory_bytes: usize,
    byte_len: u32,
    variables_ptr: u32,
    variables_len: u32,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static VALIDATED_FRAME: Cell<Option<ValidatedFrameCache>> = const { Cell::new(None) };
}

#[cfg(target_arch = "wasm32")]
unsafe fn validated_frame_and_variables(
    frame_offset: u32,
) -> Option<(&'static mut WasmJitEvalFrame, &'static [f64])> {
    let memory_bytes = core::arch::wasm32::memory_size(0).checked_mul(65_536)?;
    let cached = VALIDATED_FRAME.with(|slot| {
        let cached = slot.take();
        let hit = cached.as_ref().is_some_and(|entry| {
            entry.frame_offset == frame_offset && entry.memory_bytes == memory_bytes
        });
        slot.set(cached);
        hit
    });
    if cached {
        // Safety: the offsets in the cache passed the full validation below
        // against this exact memory size, and linear memory never shrinks.
        // The header is re-read because only the offsets were authenticated.
        let frame = unsafe { &mut *(frame_offset as usize as *mut WasmJitEvalFrame) };
        if frame.magic == WASM_JIT_FRAME_MAGIC
            && frame.abi_version == WASM_JIT_ABI_VERSION
            && frame.byte_len >= WASM_JIT_EVAL_FRAME_BYTES
        {
            let variables_ptr = frame.variables_ptr;
            let variables_len = frame.variables_len;
            let unchanged = VALIDATED_FRAME.with(|slot| {
                let entry = slot.take();
                let unchanged = entry.as_ref().is_some_and(|entry| {
                    entry.byte_len == frame.byte_len
                        && entry.variables_ptr == variables_ptr
                        && entry.variables_len == variables_len
                });
                slot.set(entry);
                unchanged
            });
            if unchanged {
                let variables = if variables_len == 0 {
                    &[][..]
                } else {
                    unsafe {
                        std::slice::from_raw_parts(
                            variables_ptr as usize as *const f64,
                            variables_len as usize,
                        )
                    }
                };
                return Some((frame, variables));
            }
        }
        VALIDATED_FRAME.with(|slot| slot.set(None));
    }
    let validated = unsafe { validate_frame_and_variables_uncached(frame_offset, memory_bytes) }?;
    VALIDATED_FRAME.with(|slot| {
        slot.set(Some(ValidatedFrameCache {
            frame_offset,
            memory_bytes,
            byte_len: validated.0.byte_len,
            variables_ptr: validated.0.variables_ptr,
            variables_len: validated.0.variables_len,
        }));
    });
    Some(validated)
}

#[cfg(target_arch = "wasm32")]
unsafe fn validate_frame_and_variables_uncached(
    frame_offset: u32,
    memory_bytes: usize,
) -> Option<(&'static mut WasmJitEvalFrame, &'static [f64])> {
    let frame_start = usize::try_from(frame_offset).ok()?;
    let frame = unsafe { validated_frame(frame_offset) }?;
    let frame_end = frame_start.checked_add(usize::try_from(frame.byte_len).ok()?)?;
    let variables_start = usize::try_from(frame.variables_ptr).ok()?;
    let variables_len = usize::try_from(frame.variables_len).ok()?;
    let variable_bytes = variables_len.checked_mul(size_of::<f64>())?;
    let variables_end = variables_start.checked_add(variable_bytes)?;
    if variables_start % align_of::<f64>() != 0
        || variables_end > memory_bytes
        || ranges_overlap(frame_start, frame_end, variables_start, variables_end)
    {
        return None;
    }
    let variables = if variables_len == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(variables_start as *const f64, variables_len) }
    };
    Some((frame, variables))
}

#[cfg(target_arch = "wasm32")]
fn ranges_overlap(a_start: usize, a_end: usize, b_start: usize, b_end: usize) -> bool {
    a_start < b_end && b_start < a_end
}

#[cfg(target_arch = "wasm32")]
use std::mem::{align_of, size_of};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::IntegrationCoefficients;

    #[test]
    fn pure_helper_matches_shared_constant_semantics() {
        let variables = [3.0, 5.0, 7.0];
        assert_eq!(
            evaluate_helper(1, 0, 3, 0, [1.2, 0.0, 0.0, 0.0, 0.0], &variables),
            Ok(5.0)
        );
        assert_eq!(
            evaluate_helper(2, 0, 3, 0, [1.2, 0.0, 0.0, 0.0, 0.0], &variables),
            Ok(1.0)
        );
        assert_eq!(
            evaluate_helper(100, 0, 0, 0, [2.0, 0.0, 0.0, 0.0, 0.0], &variables),
            Ok(2.0_f64.exp())
        );
        assert_eq!(
            evaluate_helper(200, 0, 0, 0, [2.0, 3.0, 0.0, 0.0, 0.0], &variables),
            Ok(8.0)
        );
        assert_eq!(
            evaluate_helper(303, 0, 0, 0, [6.0, 3.0, 0.0, 0.0, 0.0], &variables),
            Ok(2.0)
        );
        assert_eq!(evaluate_helper(430, 0, 0, 0, [9.0; 5], &variables), Ok(0.0));
        assert_eq!(evaluate_helper(431, 0, 0, 0, [9.0; 5], &variables), Ok(0.0));
    }

    #[test]
    fn helper_fails_closed_for_bounds_shifts_and_uninstalled_state() {
        let variables = [1.0];
        assert_eq!(
            evaluate_helper(1, 0, 1, 0, [2.0, 0.0, 0.0, 0.0, 0.0], &variables),
            Err(HelperError::InvalidDynamicIndex)
        );
        assert_eq!(
            evaluate_helper(301, 0, 0, 0, [1.0, 64.0, 0.0, 0.0, 0.0], &variables),
            Err(HelperError::InvalidIntegerShift)
        );
        assert_eq!(
            evaluate_helper(440, 0, 0, 0, [0.0; 5], &variables),
            Err(HelperError::StatefulRuntimeUnavailable)
        );
    }

    #[test]
    fn stateful_helpers_share_reference_vm_candidate_semantics() {
        let mut context = VmContext::with_states(0, 1);
        context.state_values_prev[0] = 2.0;
        context.state_values_older[0] = 1.5;
        context.state_initialized[0] = true;
        context.integration = IntegrationCoefficients::backward_euler(0.1);
        let mut session = WasmJitRuntimeSession::new(context);

        let derivative = evaluate_helper_with_session(
            440,
            0,
            0,
            0,
            [3.0, 0.0, 0.0, 0.0, 0.0],
            &[],
            Some(&mut session),
        )
        .expect("evaluate ddt state through reference VM semantics");
        assert_eq!(derivative, 10.0);
        assert_eq!(session.context().state_values[0], 3.0);
        assert_eq!(session.context().state_derivatives[0], 10.0);

        let jacobian = evaluate_helper_with_session(
            441,
            0,
            0,
            0,
            [2.5, 0.0, 0.0, 0.0, 0.0],
            &[],
            Some(&mut session),
        )
        .expect("evaluate ddt Jacobian through reference VM semantics");
        assert_eq!(jacobian, 25.0);
        session.context_mut().time = 1.0;
        assert!(session.take_error().is_none());
        assert_eq!(session.into_context().time, 1.0);
    }

    #[test]
    fn stateful_helper_rejects_unallocated_storage_without_growing_it() {
        let mut session = WasmJitRuntimeSession::new(VmContext::default());
        assert_eq!(
            evaluate_helper_with_session(
                440,
                0,
                0,
                0,
                [1.0, 0.0, 0.0, 0.0, 0.0],
                &[],
                Some(&mut session),
            ),
            Err(HelperError::StatefulRuntimeFailed)
        );
        assert!(session.context().state_values.is_empty());
        assert!(
            session
                .take_error()
                .is_some_and(|error| error.contains("not fully preallocated"))
        );
    }

    #[test]
    fn zi_slice_helper_accepts_six_operands_and_scalar_helper_rejects_it() {
        let layout = crate::codegen::ZiRuntimeLayout::unit_coefficients(0);
        let descriptor = super::super::abi::encode_zi_layout_descriptor(layout)
            .expect("unit Zi browser descriptor");
        let mut context = VmContext::default();
        context.zi_filters.push(
            crate::zfilter::ZiFilter::new(vec![1.0], vec![1.0], 1.0)
                .expect("valid placeholder Zi filter"),
        );
        let mut session = WasmJitRuntimeSession::new(context);
        let operands = [1.0, 1.0, 1.0, 0.0, 2.5, 0.0];
        assert_eq!(
            evaluate_slice_helper_with_session(421, 0, descriptor, 0, &operands, &mut session,),
            Ok(2.5)
        );
        let derivative_operands = [1.0, 1.0, 1.0, 0.0, 3.0, 0.0];
        assert_eq!(
            evaluate_slice_helper_with_session(
                429,
                0,
                descriptor,
                0,
                &derivative_operands,
                &mut session,
            ),
            Ok(3.0)
        );
        assert_eq!(session.scratch_stack.capacity(), 8);

        assert_eq!(
            evaluate_helper_with_session(421, 0, descriptor, 0, [0.0; 5], &[], Some(&mut session),),
            Err(HelperError::StatefulRuntimeFailed)
        );
        assert!(
            session
                .take_error()
                .is_some_and(|error| error.contains("slice helper is required"))
        );
    }

    #[test]
    fn zi_slice_helper_fails_closed_on_descriptor_and_count_mismatch() {
        let layout = crate::codegen::ZiRuntimeLayout::unit_coefficients(0);
        let descriptor = super::super::abi::encode_zi_layout_descriptor(layout)
            .expect("unit Zi browser descriptor");
        let mut context = VmContext::default();
        context.zi_filters.push(
            crate::zfilter::ZiFilter::new(vec![1.0], vec![1.0], 1.0)
                .expect("valid placeholder Zi filter"),
        );
        let mut session = WasmJitRuntimeSession::new(context);
        assert_eq!(
            evaluate_slice_helper_with_session(421, 0, descriptor, 0, &[1.0; 5], &mut session,),
            Err(HelperError::StatefulRuntimeFailed)
        );
        assert!(
            session
                .take_error()
                .is_some_and(|error| error.contains("requires 6 operands, received 5"))
        );
        assert_eq!(
            evaluate_slice_helper_with_session(421, 0, descriptor, 1, &[1.0; 6], &mut session,),
            Err(HelperError::StatefulRuntimeFailed)
        );
        assert!(
            session
                .take_error()
                .is_some_and(|error| error.contains("nonzero reserved metadata"))
        );
    }
}
