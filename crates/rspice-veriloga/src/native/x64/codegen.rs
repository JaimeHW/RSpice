use super::encoder::{ConditionCode, Gpr, X64Encoder, Xmm};
use crate::native::abi::{
    rspice_absdelay_state_native, rspice_acos, rspice_acosh, rspice_asin, rspice_asinh,
    rspice_atan, rspice_atan2, rspice_atanh, rspice_ceil, rspice_cos, rspice_cosh,
    rspice_cross_state_native, rspice_dynamic_variable_load_native,
    rspice_dynamic_variable_slot_native, rspice_exp, rspice_floor, rspice_hypot,
    rspice_idtmod_wrap, rspice_laplace_step_native, rspice_limexp, rspice_limited_exp, rspice_log,
    rspice_log10, rspice_mod, rspice_native_current_probe_error,
    rspice_native_integer_shift_count_error, rspice_native_limit_state_bounds_error,
    rspice_native_limit_state_initialized_error, rspice_native_limit_state_values_bounds_error,
    rspice_native_limit_state_values_error, rspice_native_loop_limit_error,
    rspice_native_param_given_error, rspice_native_port_connected_error,
    rspice_native_prior_current_error, rspice_native_state_prev_bounds_error,
    rspice_native_state_values_bounds_error, rspice_native_state_values_error, rspice_pow,
    rspice_sin, rspice_sinh, rspice_slew_state_native, rspice_table_derivative_native,
    rspice_table_lookup_native, rspice_tan, rspice_tanh, rspice_timer_state_native,
    rspice_transition_state_native, rspice_zi_step_native,
};
use crate::native::expr::{BinaryMathOp, IntegerBinaryOp, UnaryMathOp};
use crate::native::expr::{
    CompareOp, ExtremumOp, LogicalOp, NativeOp, NativeProgram, VoltageNode, native_op_stack_effect,
};
use crate::native::{JitError, JitResult};

const MODEL: &str = "native-x64";
const VOLTAGES_OFFSET: i32 = 0;
const INTERNAL_VOLTAGES_OFFSET: i32 = 8;
const PARAMS_OFFSET: i32 = 16;
const BRANCH_CURRENTS_OFFSET: i32 = 24;
const BRANCH_CURRENTS_LEN_OFFSET: i32 = 32;
const CURRENTS_OFFSET: i32 = 40;
const CURRENTS_LEN_OFFSET: i32 = 48;
const PORT_CONNECTED_OFFSET: i32 = 64;
const PORT_CONNECTED_LEN_OFFSET: i32 = 72;
const TEMPERATURE_OFFSET: i32 = 80;
const TIME_OFFSET: i32 = 88;
const TIMESTEP_OFFSET: i32 = 96;
const STATE_PREV_OFFSET: i32 = 104;
const STATE_VALUES_OFFSET: i32 = 112;
const STATE_INITIALIZED_OFFSET: i32 = 120;
const STATE_INITIALIZED_LEN_OFFSET: i32 = 128;
const LOOKUP_TABLES_OFFSET: i32 = 136;
const LOOKUP_TABLES_LEN_OFFSET: i32 = 144;
const PARAM_GIVEN_OFFSET: i32 = 168;
const PARAM_GIVEN_LEN_OFFSET: i32 = 176;
const BRANCH_UNKNOWNS_OFFSET: i32 = 184;
const ANALYSIS_TYPE_OFFSET: i32 = 192;
const MFACTOR_OFFSET: i32 = 200;
const STATE_PREV_LEN_OFFSET: i32 = 288;
const STATE_VALUES_LEN_OFFSET: i32 = 296;
const WORD_BYTES: usize = std::mem::size_of::<f64>();
const LITERAL_POOL_ALIGNMENT: usize = WORD_BYTES;
const VECTOR_LITERAL_ALIGNMENT: usize = 16;
const LITERAL_POOL_PADDING_BYTE: u8 = 0x90;
const ABS_VALUE_MASK_LOW: u64 = 0x7fff_ffff_ffff_ffff;
const ABS_VALUE_MASK_HIGH: u64 = 0;
const K_BOLTZMANN: f64 = 1.380649e-23;
const Q_ELECTRON: f64 = 1.602176634e-19;
const THERMAL_VOLTAGE_PER_K: f64 = K_BOLTZMANN / Q_ELECTRON;
const BOOLEAN_EPSILON: f64 = 1.0e-15;
const TIMESTEP_DC_EPSILON: f64 = 1.0e-20;
const F64_EXACT_INTEGER_LIMIT_ABS_BITS: u64 = 0x4330_0000_0000_0000;
const I64_MAX_EXCLUSIVE_AS_F64: f64 = 9_223_372_036_854_775_808.0;
const I64_MIN_AS_F64: f64 = -9_223_372_036_854_775_808.0;
const INLINE_DYNAMIC_LOWER_ABS_LIMIT: i64 = 1_i64 << 51;
const DYNAMIC_READ_FRAME_BYTES: i32 = 16;
const ROUND_TEMP_FRAME_BYTES: i32 = 16;
const STATEFUL_SCRATCH_FRAME_BYTES: i32 = 16;
const CALLEE_SAVED_XMM_BYTES: i32 = 16;
#[cfg(test)]
const CALL_RESULT_SLOT: usize = 6;
#[cfg(windows)]
const CALL_SHADOW_BYTES: i32 = 32;
#[cfg(not(windows))]
const CALL_SHADOW_BYTES: i32 = 0;
const LOCAL_SLOT_BYTES: i32 = 8;
const LOCAL_FRAME_ALIGN_BYTES: i32 = 16;
const INDEXED_ASSIGNMENT_SLOT_PTR_DISP: i32 = 0;
const MAX_RUNTIME_LOOP_ITERATIONS: i32 = 100_000;
#[cfg(windows)]
const CALLER_SAVED_XMM_COUNT: usize = 6;
#[cfg(not(windows))]
const CALLER_SAVED_XMM_COUNT: usize = XMM_STACK.len();
const XMM_STACK: [Xmm; 16] = [
    Xmm::Xmm0,
    Xmm::Xmm1,
    Xmm::Xmm2,
    Xmm::Xmm3,
    Xmm::Xmm4,
    Xmm::Xmm5,
    Xmm::Xmm6,
    Xmm::Xmm7,
    Xmm::Xmm8,
    Xmm::Xmm9,
    Xmm::Xmm10,
    Xmm::Xmm11,
    Xmm::Xmm12,
    Xmm::Xmm13,
    Xmm::Xmm14,
    Xmm::Xmm15,
];
#[allow(dead_code)]
pub(crate) fn compile_value_function(program: &NativeProgram) -> JitResult<Vec<u8>> {
    let needs_stateful_scratch = program_needs_stateful_stack_scratch(program);
    let local_slot_bytes = if needs_stateful_scratch {
        STATEFUL_SCRATCH_FRAME_BYTES
    } else {
        0
    };
    let callee_saved_xmm_count = callee_saved_xmm_count_for_depth(program.max_stack_depth());
    let local_frame_bytes =
        align_local_frame(local_slot_bytes + callee_saved_xmm_frame_bytes(callee_saved_xmm_count));
    let mut compiler = FunctionCompiler::new(
        program_uses_helper_calls(program),
        value_program_needs_saved_entry_args(program),
        local_frame_bytes,
        None,
        needs_stateful_scratch.then_some(0),
        callee_saved_xmm_count,
    );
    compiler.emit_program(program)?;
    compiler.finish_value_function()
}

#[allow(dead_code)]
pub(crate) fn compile_assignment_function(
    var_index: usize,
    program: &NativeProgram,
) -> JitResult<Vec<u8>> {
    let needs_stateful_scratch = program_needs_stateful_stack_scratch(program);
    let local_slot_bytes = if needs_stateful_scratch {
        STATEFUL_SCRATCH_FRAME_BYTES
    } else {
        0
    };
    let callee_saved_xmm_count = callee_saved_xmm_count_for_depth(program.max_stack_depth());
    let local_frame_bytes =
        align_local_frame(local_slot_bytes + callee_saved_xmm_frame_bytes(callee_saved_xmm_count));
    let mut compiler = FunctionCompiler::new(
        program_uses_helper_calls(program),
        program_uses_helper_calls(program),
        local_frame_bytes,
        None,
        needs_stateful_scratch.then_some(0),
        callee_saved_xmm_count,
    );
    compiler.emit_program(program)?;
    compiler.finish_assignment_function(var_index)
}

#[derive(Debug)]
pub(crate) enum NativeAssignment {
    Direct {
        var_index: usize,
        program: NativeProgram,
    },
    Indexed {
        base: usize,
        len: usize,
        lower: i64,
        index: NativeProgram,
        value: NativeProgram,
    },
    Loop {
        condition: NativeProgram,
        body: Vec<NativeAssignment>,
    },
}

pub(crate) fn compile_assignment_pass_function(
    assignments: &[NativeAssignment],
) -> JitResult<Vec<u8>> {
    let has_indexed_assignment = assignments.iter().any(assignment_has_indexed);
    let loop_depth = assignment_loop_depth(assignments);
    let has_stateful_scratch = assignments
        .iter()
        .any(assignment_needs_stateful_stack_scratch);
    let uses_helper_calls = assignments.iter().any(assignment_uses_helper_calls);
    let max_stack_depth = assignment_max_stack_depth(assignments);
    let callee_saved_xmm_count = callee_saved_xmm_count_for_depth(max_stack_depth);
    let indexed_slot_bytes = if has_indexed_assignment {
        LOCAL_SLOT_BYTES
    } else {
        0
    };
    let loop_counter_base_disp = (loop_depth > 0).then_some(indexed_slot_bytes);
    let stateful_scratch_base_disp =
        has_stateful_scratch.then_some(indexed_slot_bytes + loop_depth * LOCAL_SLOT_BYTES);
    let stateful_scratch_bytes = if has_stateful_scratch {
        STATEFUL_SCRATCH_FRAME_BYTES
    } else {
        0
    };
    let local_frame_bytes = align_local_frame(
        indexed_slot_bytes
            + loop_depth * LOCAL_SLOT_BYTES
            + stateful_scratch_bytes
            + callee_saved_xmm_frame_bytes(callee_saved_xmm_count),
    );

    let mut compiler = FunctionCompiler::new(
        uses_helper_calls,
        uses_helper_calls,
        local_frame_bytes,
        loop_counter_base_disp,
        stateful_scratch_base_disp,
        callee_saved_xmm_count,
    );
    for assignment in assignments {
        compiler.emit_assignment_step(assignment, 0)?;
    }
    compiler.finish_assignment_pass_function()
}

#[derive(Debug)]
struct FunctionCompiler {
    encoder: X64Encoder,
    depth: usize,
    literals: Vec<LiteralPatch>,
    vector_literals: Vec<VectorLiteralPatch>,
    uses_helper_calls: bool,
    saves_entry_args: bool,
    local_frame_bytes: i32,
    loop_counter_base_disp: Option<i32>,
    stateful_scratch_base_disp: Option<i32>,
    callee_saved_xmm_count: usize,
    early_return_jumps: Vec<usize>,
}

#[derive(Debug)]
struct LiteralPatch {
    displacement_offset: usize,
    value: f64,
}

#[derive(Debug)]
struct VectorLiteralPatch {
    displacement_offset: usize,
    lanes: [u64; 2],
}

impl FunctionCompiler {
    fn new(
        uses_helper_calls: bool,
        saves_entry_args: bool,
        local_frame_bytes: i32,
        loop_counter_base_disp: Option<i32>,
        stateful_scratch_base_disp: Option<i32>,
        callee_saved_xmm_count: usize,
    ) -> Self {
        debug_assert_eq!(local_frame_bytes % 16, 0);
        if let Some(base_disp) = stateful_scratch_base_disp {
            debug_assert!(base_disp + STATEFUL_SCRATCH_FRAME_BYTES <= local_frame_bytes);
        }
        debug_assert!(callee_saved_xmm_frame_bytes(callee_saved_xmm_count) <= local_frame_bytes);
        let mut compiler = Self {
            encoder: X64Encoder::new(),
            depth: 0,
            literals: Vec::new(),
            vector_literals: Vec::new(),
            uses_helper_calls,
            saves_entry_args,
            local_frame_bytes,
            loop_counter_base_disp,
            stateful_scratch_base_disp,
            callee_saved_xmm_count,
            early_return_jumps: Vec::new(),
        };
        if compiler.has_stack_setup() {
            compiler.emit_prologue();
        }
        compiler
    }

    fn has_stack_setup(&self) -> bool {
        self.saves_entry_args || self.local_frame_bytes > 0
    }

    fn saves_entry_args(&self) -> bool {
        self.saves_entry_args
    }

    fn emit_program(&mut self, program: &NativeProgram) -> JitResult<()> {
        program.validate_dependency_metadata()?;
        if program.max_stack_depth() > XMM_STACK.len() {
            return Err(register_allocation_error(format!(
                "expression stack depth {} exceeds {} XMM registers",
                program.max_stack_depth(),
                XMM_STACK.len()
            )));
        }

        self.depth = 0;
        let mut context_pointer_cache = None;
        for op in program.ops() {
            match *op {
                NativeOp::Const(value) => {
                    let dst = self.push_register()?;
                    self.emit_constant_load(dst, value);
                }
                NativeOp::LoadParam(index) => {
                    let dst = self.push_register()?;
                    self.emit_context_pointer_load_cached(
                        PARAMS_OFFSET,
                        &mut context_pointer_cache,
                    );
                    self.encoder
                        .movsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(index)?);
                }
                NativeOp::LoadParamGiven(index) => {
                    self.emit_param_given_load(index, &mut context_pointer_cache)?;
                }
                NativeOp::LoadPortConnected(index) => {
                    self.emit_port_connected_load(index, &mut context_pointer_cache)?;
                }
                NativeOp::LoadVoltage { pos, neg } => {
                    self.emit_voltage_load(pos, neg, &mut context_pointer_cache)?;
                }
                NativeOp::LoadCurrent(pair_index) => {
                    self.emit_current_load(pair_index, &mut context_pointer_cache)?;
                }
                NativeOp::LoadPriorCurrent(current_index) => {
                    self.emit_prior_current_load(current_index, &mut context_pointer_cache)?;
                }
                NativeOp::LoadInternalVoltage(index) => {
                    let dst = self.push_register()?;
                    self.emit_context_pointer_load_cached(
                        INTERNAL_VOLTAGES_OFFSET,
                        &mut context_pointer_cache,
                    );
                    self.encoder
                        .movsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(index)?);
                }
                NativeOp::LoadVariable(index) => {
                    let dst = self.push_register()?;
                    self.encoder.movsd_xmm_m64_base_disp32(
                        dst,
                        self.vars_arg_reg(),
                        byte_disp(index)?,
                    );
                }
                NativeOp::LoadVariableDyn { base, len, lower } => {
                    self.emit_dynamic_variable_load(base, len, lower)?;
                }
                NativeOp::LoadBranchUnknown(index) => {
                    let dst = self.push_register()?;
                    self.emit_context_pointer_load_cached(
                        BRANCH_UNKNOWNS_OFFSET,
                        &mut context_pointer_cache,
                    );
                    self.encoder
                        .movsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(index)?);
                }
                NativeOp::LoadTemperature => {
                    self.emit_context_f64_load(TEMPERATURE_OFFSET)?;
                }
                NativeOp::LoadThermalVoltage => {
                    self.emit_thermal_voltage_load()?;
                }
                NativeOp::LoadTime => {
                    self.emit_context_f64_load(TIME_OFFSET)?;
                }
                NativeOp::Analysis(analysis_id) => {
                    self.emit_analysis_check(analysis_id)?;
                }
                NativeOp::LoadMfactor => {
                    self.emit_context_f64_load(MFACTOR_OFFSET)?;
                }
                NativeOp::Add => self.emit_binary_op(BinaryOp::Add)?,
                NativeOp::Sub => self.emit_binary_op(BinaryOp::Sub)?,
                NativeOp::Mul => self.emit_binary_op(BinaryOp::Mul)?,
                NativeOp::Div => self.emit_binary_op(BinaryOp::Div)?,
                NativeOp::AddConst(value) => {
                    self.emit_literal_rhs_binary_op(value, BinaryOp::Add)?
                }
                NativeOp::SubConst(value) => {
                    self.emit_literal_rhs_binary_op(value, BinaryOp::Sub)?
                }
                NativeOp::MulConst(value) => {
                    self.emit_literal_rhs_binary_op(value, BinaryOp::Mul)?
                }
                NativeOp::DivConst(value) => {
                    self.emit_literal_rhs_binary_op(value, BinaryOp::Div)?
                }
                NativeOp::SubFromConst(value) => {
                    self.emit_literal_lhs_binary_op(value, BinaryOp::Sub)?
                }
                NativeOp::DivFromConst(value) => {
                    self.emit_literal_lhs_binary_op(value, BinaryOp::Div)?
                }
                NativeOp::Neg => self.emit_neg()?,
                NativeOp::Abs => self.emit_abs()?,
                NativeOp::Square => self.emit_square()?,
                NativeOp::Sqrt => self.emit_sqrt()?,
                NativeOp::Compare(op) => self.emit_compare(op)?,
                NativeOp::CompareConst(op, value) => self.emit_compare_const(op, value)?,
                NativeOp::Logical(op) => self.emit_logical(op)?,
                NativeOp::LogicalConst(op, value) => self.emit_logical_const(op, value)?,
                NativeOp::IfElse => self.emit_ifelse()?,
                NativeOp::Extremum(op) => self.emit_extremum(op)?,
                NativeOp::ExtremumConst(op, value) => self.emit_extremum_const(op, value)?,
                NativeOp::ExtremumConstLhs(op, value) => self.emit_extremum_const_lhs(op, value)?,
                NativeOp::UnaryMath(op) => self.emit_unary_math(op)?,
                NativeOp::BinaryMath(op) => self.emit_binary_math(op)?,
                NativeOp::IntegerCast => self.emit_integer_cast()?,
                NativeOp::IntegerBinary(op) => self.emit_integer_binary(op)?,
                NativeOp::IntegerShiftConst(op, count) => {
                    self.emit_integer_shift_const(op, count)?
                }
                NativeOp::IntegerBinaryConst(op, value) => {
                    self.emit_integer_bitwise_const(op, value)?
                }
                NativeOp::TableLookup(table_id) => {
                    self.emit_table_helper_call(table_id, rspice_table_lookup_native)?
                }
                NativeOp::TableDerivative(table_id) => {
                    self.emit_table_helper_call(table_id, rspice_table_derivative_native)?
                }
                NativeOp::LimitState(index) => self.emit_limit_state(index)?,
                NativeOp::LaplaceState(filter_id) => self.emit_laplace_state(filter_id)?,
                NativeOp::ZiState(filter_id) => self.emit_zi_state(filter_id)?,
                NativeOp::TimerState(timer_id) => self.emit_timer_state(timer_id)?,
                NativeOp::TransitionState(filter_id) => self.emit_transition_state(filter_id)?,
                NativeOp::SlewState(filter_id) => self.emit_slew_state(filter_id)?,
                NativeOp::AbsDelayState(buffer_id) => self.emit_absdelay_state(buffer_id)?,
                NativeOp::CrossState(detector_id) => self.emit_cross_state(detector_id)?,
                NativeOp::WhiteNoise => self.emit_white_noise()?,
                NativeOp::FlickerNoise => self.emit_flicker_noise()?,
                NativeOp::DdtState(index) => self.emit_ddt_state(index)?,
                NativeOp::DdtJacobian => self.emit_ddt_jacobian()?,
                NativeOp::IdtState(index) => self.emit_idt_state(index)?,
                NativeOp::IdtJacobian => self.emit_idt_jacobian()?,
                NativeOp::IdtModState(index) => self.emit_idtmod_state(index)?,
            }
            if !native_op_preserves_context_pointer_cache(*op) {
                context_pointer_cache = None;
            }
        }

        if self.depth != 1 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("final expression stack depth {}, expected 1", self.depth).into(),
            });
        }

        Ok(())
    }

    fn finish_value_function(mut self) -> JitResult<Vec<u8>> {
        self.patch_early_returns_to_current()?;
        self.emit_return();
        self.finish_with_literals()
    }

    fn finish_assignment_function(mut self, var_index: usize) -> JitResult<Vec<u8>> {
        self.emit_assignment_store(var_index)?;
        self.patch_early_returns_to_current()?;
        self.emit_return();
        self.finish_with_literals()
    }

    fn finish_assignment_pass_function(mut self) -> JitResult<Vec<u8>> {
        self.patch_early_returns_to_current()?;
        self.emit_return();
        self.finish_with_literals()
    }

    fn emit_prologue(&mut self) {
        if self.saves_entry_args() {
            self.encoder.push_r64(Gpr::R12);
            self.encoder.push_r64(Gpr::R13);
            self.encoder
                .mov_r64_r64(saved_ctx_arg_reg(), entry_ctx_arg_reg());
            self.encoder
                .mov_r64_r64(saved_vars_arg_reg(), entry_vars_arg_reg());
        }
        if self.local_frame_bytes > 0 {
            self.encoder.sub_rsp_imm32(self.local_frame_bytes);
        }
        self.emit_callee_saved_xmm_stores();
    }

    fn emit_return(&mut self) {
        self.emit_callee_saved_xmm_loads();
        if self.local_frame_bytes > 0 {
            self.encoder.add_rsp_imm32(self.local_frame_bytes);
        }
        if self.saves_entry_args() {
            self.encoder.pop_r64(Gpr::R13);
            self.encoder.pop_r64(Gpr::R12);
        }
        self.encoder.ret();
    }

    fn emit_callee_saved_xmm_stores(&mut self) {
        for index in 0..self.callee_saved_xmm_count {
            let register = callee_saved_xmm_register(index);
            let disp = self.callee_saved_xmm_disp(index);
            self.encoder
                .movdqu_m128_base_disp32_xmm(Gpr::Rsp, disp, register);
        }
    }

    fn emit_callee_saved_xmm_loads(&mut self) {
        for index in (0..self.callee_saved_xmm_count).rev() {
            let register = callee_saved_xmm_register(index);
            let disp = self.callee_saved_xmm_disp(index);
            self.encoder
                .movdqu_xmm_m128_base_disp32(register, Gpr::Rsp, disp);
        }
    }

    fn callee_saved_xmm_disp(&self, index: usize) -> i32 {
        debug_assert!(index < self.callee_saved_xmm_count);
        self.local_frame_bytes - callee_saved_xmm_frame_bytes(self.callee_saved_xmm_count)
            + (index as i32 * CALLEE_SAVED_XMM_BYTES)
    }

    fn ctx_arg_reg(&self) -> Gpr {
        if self.saves_entry_args() {
            saved_ctx_arg_reg()
        } else {
            entry_ctx_arg_reg()
        }
    }

    fn vars_arg_reg(&self) -> Gpr {
        if self.saves_entry_args() {
            saved_vars_arg_reg()
        } else {
            entry_vars_arg_reg()
        }
    }

    fn emit_assignment_store(&mut self, var_index: usize) -> JitResult<()> {
        if self.depth != 1 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!(
                    "assignment expression stack depth {}, expected 1",
                    self.depth
                )
                .into(),
            });
        }
        self.encoder.movsd_m64_base_disp32_xmm(
            self.vars_arg_reg(),
            byte_disp(var_index)?,
            Xmm::Xmm0,
        );
        self.depth = 0;
        Ok(())
    }

    fn emit_assignment_step(
        &mut self,
        assignment: &NativeAssignment,
        loop_depth: i32,
    ) -> JitResult<()> {
        match assignment {
            NativeAssignment::Direct { var_index, program } => {
                self.emit_program(program)?;
                self.emit_assignment_store(*var_index)
            }
            NativeAssignment::Indexed {
                base,
                len,
                lower,
                index,
                value,
            } => self.emit_indexed_assignment(*base, *len, *lower, index, value),
            NativeAssignment::Loop { condition, body } => {
                self.emit_loop_assignment(condition, body, loop_depth)
            }
        }
    }

    fn emit_loop_assignment(
        &mut self,
        condition: &NativeProgram,
        body: &[NativeAssignment],
        loop_depth: i32,
    ) -> JitResult<()> {
        let counter_disp = self.loop_counter_disp(loop_depth)?;
        self.encoder.xor_r64_r64(Gpr::R10, Gpr::R10);
        self.encoder
            .mov_m64_base_disp32_r64(Gpr::Rsp, counter_disp, Gpr::R10);

        let loop_start = self.encoder.position();
        self.emit_program(condition)?;
        let loop_exit = self.emit_loop_exit_if_zero()?;

        for assignment in body {
            self.emit_assignment_step(assignment, loop_depth + 1)?;
        }

        self.encoder
            .mov_r64_m64_base_disp32(Gpr::R10, Gpr::Rsp, counter_disp);
        self.encoder.add_r64_imm32(Gpr::R10, 1);
        self.encoder
            .mov_m64_base_disp32_r64(Gpr::Rsp, counter_disp, Gpr::R10);
        self.encoder
            .cmp_r64_imm32(Gpr::R10, MAX_RUNTIME_LOOP_ITERATIONS);
        let limit_reached = self
            .encoder
            .jcc_rel32_placeholder(ConditionCode::AboveOrEqual);
        self.emit_jmp_to_offset(loop_start)?;
        self.patch_rel32_to_current(limit_reached)?;
        self.emit_runtime_loop_limit_error_call();
        let return_after_error = self.encoder.jmp_rel32_placeholder();
        self.early_return_jumps.push(return_after_error);
        self.patch_rel32_to_current(loop_exit)?;
        Ok(())
    }

    fn loop_counter_disp(&self, loop_depth: i32) -> JitResult<i32> {
        let Some(base_disp) = self.loop_counter_base_disp else {
            return Err(JitError::InternalCompilerError {
                model: MODEL.into(),
                detail: "loop assignment emitted without loop-counter frame slot".into(),
            });
        };
        Ok(base_disp + loop_depth * LOCAL_SLOT_BYTES)
    }

    fn emit_loop_exit_if_zero(&mut self) -> JitResult<usize> {
        if self.depth != 1 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("loop condition stack depth {}, expected 1", self.depth).into(),
            });
        }

        self.encoder.movq_r64_xmm(Gpr::R10, Xmm::Xmm0);
        self.encoder.btr_r64_imm8(Gpr::R10, 63);
        self.encoder.test_r64_r64(Gpr::R10, Gpr::R10);
        let loop_exit = self.encoder.jcc_rel32_placeholder(ConditionCode::Equal);
        self.depth = 0;
        Ok(loop_exit)
    }

    fn emit_indexed_assignment(
        &mut self,
        base: usize,
        len: usize,
        lower: i64,
        index: &NativeProgram,
        value: &NativeProgram,
    ) -> JitResult<()> {
        debug_assert!(self.local_frame_bytes >= LOCAL_SLOT_BYTES);

        self.emit_program(index)?;
        if dynamic_variable_inline_supported(len, lower) {
            self.emit_dynamic_variable_slot_inline(base, len, lower)?;
        } else {
            self.emit_dynamic_variable_slot_call(base, len, lower)?;
            self.encoder.test_r64_r64(Gpr::Rax, Gpr::Rax);
            let null_slot = self.encoder.jcc_rel32_placeholder(ConditionCode::Equal);
            self.early_return_jumps.push(null_slot);
            self.encoder.mov_m64_base_disp32_r64(
                Gpr::Rsp,
                INDEXED_ASSIGNMENT_SLOT_PTR_DISP,
                Gpr::Rax,
            );
        }

        self.emit_program(value)?;
        if self.depth != 1 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!(
                    "indexed assignment value stack depth {}, expected 1",
                    self.depth
                )
                .into(),
            });
        }
        self.encoder
            .mov_r64_m64_base_disp32(Gpr::Rax, Gpr::Rsp, INDEXED_ASSIGNMENT_SLOT_PTR_DISP);
        self.encoder
            .movsd_m64_base_disp32_xmm(Gpr::Rax, 0, Xmm::Xmm0);
        self.depth = 0;
        Ok(())
    }

    fn push_register(&mut self) -> JitResult<Xmm> {
        if self.depth >= XMM_STACK.len() {
            return Err(register_allocation_error(format!(
                "expression stack requires more than {} XMM registers",
                XMM_STACK.len()
            )));
        }

        let register = XMM_STACK[self.depth];
        self.depth += 1;
        Ok(register)
    }

    fn scratch_register(&self) -> JitResult<Xmm> {
        if self.depth >= XMM_STACK.len() {
            return Err(register_allocation_error(
                "operation requires a scratch XMM register but all are live".to_string(),
            ));
        }

        Ok(XMM_STACK[self.depth])
    }

    fn stateful_scratch_disp(&self, slot: usize) -> JitResult<i32> {
        debug_assert!(slot < 2);
        let base_disp = self.stateful_scratch_base_disp.ok_or_else(|| {
            register_allocation_error(
                "stateful operation requires a local scratch frame at full XMM stack depth"
                    .to_string(),
            )
        })?;
        base_disp
            .checked_add((slot * WORD_BYTES) as i32)
            .ok_or_else(|| JitError::Encoding {
                model: MODEL.into(),
                detail: "stateful scratch displacement overflow".into(),
            })
    }

    fn emit_binary_op(&mut self, op: BinaryOp) -> JitResult<()> {
        if self.depth < 2 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("binary op requires stack depth 2, found {}", self.depth).into(),
            });
        }

        let left = XMM_STACK[self.depth - 2];
        let right = XMM_STACK[self.depth - 1];
        match op {
            BinaryOp::Add => self.encoder.addsd_xmm_xmm(left, right),
            BinaryOp::Sub => self.encoder.subsd_xmm_xmm(left, right),
            BinaryOp::Mul => self.encoder.mulsd_xmm_xmm(left, right),
            BinaryOp::Div => self.encoder.divsd_xmm_xmm(left, right),
        }
        self.depth -= 1;
        Ok(())
    }

    fn emit_literal_rhs_binary_op(&mut self, value: f64, op: BinaryOp) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "literal RHS binary op requires stack depth 1, found 0".into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        self.emit_literal_binary_op(target, value, op);
        Ok(())
    }

    fn emit_literal_lhs_binary_op(&mut self, value: f64, op: BinaryOp) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "literal LHS binary op requires stack depth 1, found 0".into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        match op {
            BinaryOp::Sub | BinaryOp::Div => {
                if self.depth < XMM_STACK.len() {
                    let scratch = self.scratch_register()?;
                    self.emit_literal_lhs_scratch_binary_op(target, scratch, value, op);
                } else {
                    self.emit_literal_lhs_stack_binary_op(target, value, op);
                }
            }
            BinaryOp::Add | BinaryOp::Mul => {
                unreachable!("literal LHS binary lowering only accepts sub/div")
            }
        }
        Ok(())
    }

    fn emit_literal_lhs_scratch_binary_op(
        &mut self,
        target: Xmm,
        scratch: Xmm,
        value: f64,
        op: BinaryOp,
    ) {
        self.emit_literal_load(scratch, value);
        match op {
            BinaryOp::Sub => self.encoder.subsd_xmm_xmm(scratch, target),
            BinaryOp::Div => self.encoder.divsd_xmm_xmm(scratch, target),
            BinaryOp::Add | BinaryOp::Mul => {
                unreachable!("literal LHS scratch lowering only accepts sub/div")
            }
        }
        self.encoder.movsd_xmm_xmm(target, scratch);
    }

    fn emit_literal_lhs_stack_binary_op(&mut self, target: Xmm, value: f64, op: BinaryOp) {
        self.encoder.sub_rsp_imm32(ROUND_TEMP_FRAME_BYTES);
        self.encoder.movsd_m64_base_disp32_xmm(Gpr::Rsp, 0, target);
        self.emit_literal_load(target, value);
        match op {
            BinaryOp::Sub => self.encoder.subsd_xmm_m64_base_disp32(target, Gpr::Rsp, 0),
            BinaryOp::Div => self.encoder.divsd_xmm_m64_base_disp32(target, Gpr::Rsp, 0),
            BinaryOp::Add | BinaryOp::Mul => {
                unreachable!("literal LHS stack lowering only accepts sub/div")
            }
        }
        self.encoder.add_rsp_imm32(ROUND_TEMP_FRAME_BYTES);
    }

    fn emit_neg(&mut self) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "neg requires stack depth 1, found 0".into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        self.encoder.movq_r64_xmm(Gpr::R11, target);
        self.encoder.btc_r64_imm8(Gpr::R11, 63);
        self.encoder.movq_xmm_r64(target, Gpr::R11);
        Ok(())
    }

    fn emit_abs(&mut self) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "abs requires stack depth 1, found 0".into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        self.emit_abs_register(target);
        Ok(())
    }

    fn emit_square(&mut self) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "square requires stack depth 1, found 0".into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        self.encoder.mulsd_xmm_xmm(target, target);
        Ok(())
    }

    fn emit_abs_register(&mut self, target: Xmm) {
        let displacement_offset = self.encoder.andpd_xmm_m128_rip_disp32(target, 0);
        self.vector_literals.push(VectorLiteralPatch {
            displacement_offset,
            lanes: [ABS_VALUE_MASK_LOW, ABS_VALUE_MASK_HIGH],
        });
    }

    fn emit_sqrt(&mut self) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "sqrt requires stack depth 1, found 0".into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        self.encoder.sqrtsd_xmm_xmm(target, target);
        Ok(())
    }

    fn emit_unary_math(&mut self, op: UnaryMathOp) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "unary math requires stack depth 1, found 0".into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        match op {
            UnaryMathOp::Floor => self.emit_floor_or_ceil(target, RoundDirection::Floor),
            UnaryMathOp::Ceil => self.emit_floor_or_ceil(target, RoundDirection::Ceil),
            UnaryMathOp::Limexp => self.emit_limexp(target),
            UnaryMathOp::LimitedExp => self.emit_limited_exp(target),
            _ => {
                self.emit_unary_helper_call(target, unary_math_helper(op));
                Ok(())
            }
        }
    }

    fn emit_limexp(&mut self, target: Xmm) -> JitResult<()> {
        self.emit_literal_compare(target, 40.0);
        let high_jump = self.encoder.jcc_rel32_placeholder(ConditionCode::Above);

        self.emit_literal_compare(target, -40.0);
        let exp_jump = self.encoder.jcc_rel32_placeholder(ConditionCode::Parity);
        let low_jump = self.encoder.jcc_rel32_placeholder(ConditionCode::Below);

        self.patch_rel32_to_current(exp_jump)?;
        self.emit_unary_helper_call(target, rspice_exp);
        let exp_done = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(low_jump)?;
        self.emit_literal_load(target, (-40.0_f64).exp());
        let low_done = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(high_jump)?;
        self.emit_literal_binary_op(target, 1.0, BinaryOp::Add);
        self.emit_literal_binary_op(target, 40.0, BinaryOp::Sub);
        self.emit_literal_binary_op(target, 40.0_f64.exp(), BinaryOp::Mul);

        self.patch_rel32_to_current(exp_done)?;
        self.patch_rel32_to_current(low_done)?;
        Ok(())
    }

    fn emit_limited_exp(&mut self, target: Xmm) -> JitResult<()> {
        self.emit_literal_compare(target, 80.0);
        let high_jump = self.encoder.jcc_rel32_placeholder(ConditionCode::Above);

        self.emit_literal_compare(target, -80.0);
        let exp_jump = self.encoder.jcc_rel32_placeholder(ConditionCode::Parity);
        let low_jump = self.encoder.jcc_rel32_placeholder(ConditionCode::Below);

        self.patch_rel32_to_current(exp_jump)?;
        self.emit_unary_helper_call(target, rspice_exp);
        let exp_done = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(low_jump)?;
        self.emit_literal_load(target, 1.804851387e-35);
        let low_done = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(high_jump)?;
        self.emit_literal_binary_op(target, 1.0, BinaryOp::Add);
        self.emit_literal_binary_op(target, 80.0, BinaryOp::Sub);
        self.emit_literal_binary_op(target, 80.0_f64.exp(), BinaryOp::Mul);

        self.patch_rel32_to_current(exp_done)?;
        self.patch_rel32_to_current(low_done)?;
        Ok(())
    }

    fn emit_floor_or_ceil(&mut self, target: Xmm, direction: RoundDirection) -> JitResult<()> {
        self.encoder.movq_r64_xmm(Gpr::R11, target);
        self.encoder.btr_r64_imm8(Gpr::R11, 63);
        self.encoder.cmp_r64_imm32(Gpr::R11, 0);
        let zero = self.encoder.jcc_rel32_placeholder(ConditionCode::Equal);
        self.encoder
            .movabs_r64_imm64(Gpr::R10, F64_EXACT_INTEGER_LIMIT_ABS_BITS);
        self.encoder.cmp_r64_r64(Gpr::R11, Gpr::R10);
        let already_integral_or_unordered = self
            .encoder
            .jcc_rel32_placeholder(ConditionCode::AboveOrEqual);

        if self.depth < XMM_STACK.len() {
            let original = self.scratch_register()?;
            self.encoder.movsd_xmm_xmm(original, target);
            self.emit_floor_or_ceil_adjust(target, direction, |compiler, target| {
                compiler.encoder.ucomisd_xmm_xmm(target, original);
            })?;
        } else {
            self.encoder.sub_rsp_imm32(ROUND_TEMP_FRAME_BYTES);
            self.encoder.movsd_m64_base_disp32_xmm(Gpr::Rsp, 0, target);
            self.emit_floor_or_ceil_adjust(target, direction, |compiler, target| {
                compiler
                    .encoder
                    .ucomisd_xmm_m64_base_disp32(target, Gpr::Rsp, 0);
            })?;
            self.encoder.add_rsp_imm32(ROUND_TEMP_FRAME_BYTES);
        }
        self.patch_rel32_to_current(zero)?;
        self.patch_rel32_to_current(already_integral_or_unordered)?;
        Ok(())
    }

    fn emit_floor_or_ceil_adjust(
        &mut self,
        target: Xmm,
        direction: RoundDirection,
        compare_original: impl FnOnce(&mut Self, Xmm),
    ) -> JitResult<()> {
        self.encoder.cvttsd2si_r64_xmm(Gpr::R10, target);
        self.encoder.cvtsi2sd_xmm_r64(target, Gpr::R10);
        compare_original(self, target);

        let skip_adjust = match direction {
            RoundDirection::Floor => self
                .encoder
                .jcc_rel32_placeholder(ConditionCode::BelowOrEqual),
            RoundDirection::Ceil => self
                .encoder
                .jcc_rel32_placeholder(ConditionCode::AboveOrEqual),
        };
        let adjustment = match direction {
            RoundDirection::Floor => -1,
            RoundDirection::Ceil => 1,
        };
        self.encoder.add_r64_imm32(Gpr::R10, adjustment);
        self.patch_rel32_to_current(skip_adjust)?;
        self.encoder.cvtsi2sd_xmm_r64(target, Gpr::R10);
        Ok(())
    }

    fn emit_unary_helper_call(&mut self, target: Xmm, helper: UnaryHelper) {
        debug_assert!(self.uses_helper_calls);
        let frame_bytes =
            call_frame_bytes_for_slots(call_frame_spill_slot_count(self.depth, |_, register| {
                register != target
            }));
        self.encoder.sub_rsp_imm32(frame_bytes);
        self.emit_call_frame_spills(self.depth, |_, register| register != target);

        if target != Xmm::Xmm0 {
            self.encoder.movsd_xmm_xmm(Xmm::Xmm0, target);
        }
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);

        self.emit_helper_result_to_target_and_restore(target, self.depth, |_, _| true);
        self.encoder.add_rsp_imm32(frame_bytes);
    }

    fn emit_helper_result_to_target_and_restore(
        &mut self,
        target: Xmm,
        restore_depth: usize,
        mut should_restore: impl FnMut(usize, Xmm) -> bool,
    ) {
        if target != Xmm::Xmm0 {
            self.encoder.movsd_xmm_xmm(target, Xmm::Xmm0);
        }
        for (index, register) in XMM_STACK.iter().copied().take(restore_depth).enumerate() {
            if register != target && should_restore(index, register) {
                self.encoder
                    .movsd_xmm_m64_base_disp32(register, Gpr::Rsp, call_spill_disp(index));
            }
        }
    }

    fn emit_call_frame_spills(
        &mut self,
        spill_depth: usize,
        mut should_spill: impl FnMut(usize, Xmm) -> bool,
    ) {
        for (index, register) in XMM_STACK.iter().copied().take(spill_depth).enumerate() {
            if should_spill(index, register) {
                self.encoder
                    .movsd_m64_base_disp32_xmm(Gpr::Rsp, call_spill_disp(index), register);
            }
        }
    }

    fn emit_dynamic_variable_load(&mut self, base: usize, len: usize, lower: i64) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "dynamic variable load requires stack depth 1, found 0".into(),
            });
        }

        if !dynamic_variable_inline_supported(len, lower) {
            let target = XMM_STACK[self.depth - 1];
            return self.emit_dynamic_variable_helper_call(
                target,
                base,
                len,
                lower,
                rspice_dynamic_variable_load_native,
            );
        }

        let target = XMM_STACK[self.depth - 1];
        let base_disp = byte_disp(base)?;

        if self.depth < XMM_STACK.len() {
            let raw_index = self.scratch_register()?;
            self.encoder.movsd_xmm_xmm(raw_index, target);
            let slow_jumps =
                self.emit_dynamic_variable_address_inline(target, base_disp, len, lower)?;
            self.encoder.movsd_xmm_m64_base_disp32(target, Gpr::R11, 0);
            let fast_done = self.encoder.jmp_rel32_placeholder();

            for slow_jump in slow_jumps {
                self.patch_rel32_to_current(slow_jump)?;
            }
            self.emit_dynamic_variable_load_slow_return_from_register(
                raw_index, base_disp, len, lower,
            );
            self.patch_rel32_to_current(fast_done)?;
            return Ok(());
        }

        self.encoder.sub_rsp_imm32(DYNAMIC_READ_FRAME_BYTES);
        self.encoder.movsd_m64_base_disp32_xmm(Gpr::Rsp, 0, target);

        let slow_jumps =
            self.emit_dynamic_variable_address_inline(target, base_disp, len, lower)?;
        self.encoder.movsd_xmm_m64_base_disp32(target, Gpr::R11, 0);
        self.encoder.add_rsp_imm32(DYNAMIC_READ_FRAME_BYTES);
        let fast_done = self.encoder.jmp_rel32_placeholder();

        for slow_jump in slow_jumps {
            self.patch_rel32_to_current(slow_jump)?;
        }
        self.emit_dynamic_variable_load_slow_return(base_disp, len, lower);
        self.patch_rel32_to_current(fast_done)?;
        Ok(())
    }

    fn emit_dynamic_variable_load_slow_return(&mut self, base_disp: i32, len: usize, lower: i64) {
        self.encoder
            .movsd_xmm_m64_base_disp32(Xmm::Xmm0, Gpr::Rsp, 0);
        self.emit_dynamic_variable_load_slow_return_from_xmm0(base_disp, len, lower);
        self.encoder.add_rsp_imm32(DYNAMIC_READ_FRAME_BYTES);
        let return_after_error = self.encoder.jmp_rel32_placeholder();
        self.early_return_jumps.push(return_after_error);
    }

    fn emit_dynamic_variable_load_slow_return_from_register(
        &mut self,
        raw_index: Xmm,
        base_disp: i32,
        len: usize,
        lower: i64,
    ) {
        if raw_index != Xmm::Xmm0 {
            self.encoder.movsd_xmm_xmm(Xmm::Xmm0, raw_index);
        }
        self.emit_dynamic_variable_load_slow_return_from_xmm0(base_disp, len, lower);
        let return_after_error = self.encoder.jmp_rel32_placeholder();
        self.early_return_jumps.push(return_after_error);
    }

    fn emit_dynamic_variable_load_slow_return_from_xmm0(
        &mut self,
        base_disp: i32,
        len: usize,
        lower: i64,
    ) {
        let frame_bytes = call_frame_bytes_for_slots(0);
        self.encoder.sub_rsp_imm32(frame_bytes);
        self.encoder
            .mov_r64_r64(dynamic_variable_base_arg_reg(), self.vars_arg_reg());
        if base_disp != 0 {
            self.encoder
                .add_r64_imm32(dynamic_variable_base_arg_reg(), base_disp);
        }
        self.emit_usize_arg(dynamic_variable_len_arg_reg(), len);
        self.emit_i64_arg(dynamic_variable_lower_arg_reg(), lower);
        let helper: DynamicVariableHelper = rspice_dynamic_variable_load_native;
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);
        self.encoder.add_rsp_imm32(frame_bytes);
    }

    fn emit_dynamic_variable_slot_inline(
        &mut self,
        base: usize,
        len: usize,
        lower: i64,
    ) -> JitResult<()> {
        if self.depth != 1 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!(
                    "dynamic variable slot inline path requires stack depth 1, found {}",
                    self.depth
                )
                .into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        let base_disp = byte_disp(base)?;
        self.encoder
            .movsd_m64_base_disp32_xmm(Gpr::Rsp, INDEXED_ASSIGNMENT_SLOT_PTR_DISP, target);

        let slow_jumps =
            self.emit_dynamic_variable_address_inline(target, base_disp, len, lower)?;
        self.encoder
            .mov_m64_base_disp32_r64(Gpr::Rsp, INDEXED_ASSIGNMENT_SLOT_PTR_DISP, Gpr::R11);
        let fast_done = self.encoder.jmp_rel32_placeholder();

        for slow_jump in slow_jumps {
            self.patch_rel32_to_current(slow_jump)?;
        }
        self.emit_dynamic_variable_slot_slow_return(base_disp, len, lower);
        self.patch_rel32_to_current(fast_done)?;
        self.depth = 0;
        Ok(())
    }

    fn emit_dynamic_variable_slot_slow_return(&mut self, base_disp: i32, len: usize, lower: i64) {
        self.encoder.movsd_xmm_m64_base_disp32(
            Xmm::Xmm0,
            Gpr::Rsp,
            INDEXED_ASSIGNMENT_SLOT_PTR_DISP,
        );
        let frame_bytes = call_frame_bytes_for_slots(0);
        self.encoder.sub_rsp_imm32(frame_bytes);
        self.encoder
            .mov_r64_r64(dynamic_variable_base_arg_reg(), self.vars_arg_reg());
        if base_disp != 0 {
            self.encoder
                .add_r64_imm32(dynamic_variable_base_arg_reg(), base_disp);
        }
        self.emit_usize_arg(dynamic_variable_len_arg_reg(), len);
        self.emit_i64_arg(dynamic_variable_lower_arg_reg(), lower);
        let helper: DynamicVariableSlotHelper = rspice_dynamic_variable_slot_native;
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);
        self.encoder.add_rsp_imm32(frame_bytes);
        let return_after_error = self.encoder.jmp_rel32_placeholder();
        self.early_return_jumps.push(return_after_error);
    }

    fn emit_dynamic_variable_address_inline(
        &mut self,
        index: Xmm,
        base_disp: i32,
        len: usize,
        lower: i64,
    ) -> JitResult<Vec<usize>> {
        let mut slow_jumps = Vec::new();

        self.encoder.movq_r64_xmm(Gpr::R8, index);
        self.encoder.mov_r64_r64(Gpr::R11, Gpr::R8);
        self.encoder.btr_r64_imm8(Gpr::R11, 63);
        self.encoder
            .movabs_r64_imm64(Gpr::R10, F64_EXACT_INTEGER_LIMIT_ABS_BITS);
        self.encoder.cmp_r64_r64(Gpr::R11, Gpr::R10);
        slow_jumps.push(
            self.encoder
                .jcc_rel32_placeholder(ConditionCode::AboveOrEqual),
        );

        self.encoder.test_r64_r64(Gpr::R8, Gpr::R8);
        let non_negative = self
            .encoder
            .jcc_rel32_placeholder(ConditionCode::NotNegative);
        self.emit_literal_binary_op(index, 0.5, BinaryOp::Sub);
        let rounded = self.encoder.jmp_rel32_placeholder();
        self.patch_rel32_to_current(non_negative)?;
        self.emit_literal_binary_op(index, 0.5, BinaryOp::Add);
        self.patch_rel32_to_current(rounded)?;

        self.encoder.cvttsd2si_r64_xmm(Gpr::R10, index);
        self.emit_i64_subtract(Gpr::R10, lower);
        self.encoder.test_r64_r64(Gpr::R10, Gpr::R10);
        slow_jumps.push(self.encoder.jcc_rel32_placeholder(ConditionCode::Negative));
        self.emit_usize_compare(Gpr::R10, len);
        slow_jumps.push(
            self.encoder
                .jcc_rel32_placeholder(ConditionCode::AboveOrEqual),
        );

        self.encoder.lea_r64_base_index_scale8_disp32(
            Gpr::R11,
            self.vars_arg_reg(),
            Gpr::R10,
            base_disp,
        );
        Ok(slow_jumps)
    }

    fn emit_dynamic_variable_helper_call(
        &mut self,
        target: Xmm,
        base: usize,
        len: usize,
        lower: i64,
        helper: DynamicVariableHelper,
    ) -> JitResult<()> {
        debug_assert!(self.uses_helper_calls);
        debug_assert!(xmm_stack_slot(target) < self.depth);
        let base_disp = byte_disp(base)?;

        let frame_bytes =
            call_frame_bytes_for_slots(call_frame_spill_slot_count(self.depth, |_, register| {
                register != target
            }));
        self.encoder.sub_rsp_imm32(frame_bytes);
        self.emit_call_frame_spills(self.depth, |_, register| register != target);

        if target != Xmm::Xmm0 {
            self.encoder.movsd_xmm_xmm(Xmm::Xmm0, target);
        }
        self.encoder
            .mov_r64_r64(dynamic_variable_base_arg_reg(), self.vars_arg_reg());
        if base_disp != 0 {
            self.encoder
                .add_r64_imm32(dynamic_variable_base_arg_reg(), base_disp);
        }
        self.emit_usize_arg(dynamic_variable_len_arg_reg(), len);
        self.emit_i64_arg(dynamic_variable_lower_arg_reg(), lower);
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);

        self.emit_helper_result_to_target_and_restore(target, self.depth, |_, _| true);
        self.encoder.add_rsp_imm32(frame_bytes);
        Ok(())
    }

    fn emit_dynamic_variable_slot_call(
        &mut self,
        base: usize,
        len: usize,
        lower: i64,
    ) -> JitResult<()> {
        if self.depth != 1 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!(
                    "dynamic variable slot helper requires stack depth 1, found {}",
                    self.depth
                )
                .into(),
            });
        }

        debug_assert!(self.uses_helper_calls);
        let base_disp = byte_disp(base)?;

        let frame_bytes = call_frame_bytes_for_slots(0);
        self.encoder.sub_rsp_imm32(frame_bytes);
        self.encoder
            .mov_r64_r64(dynamic_variable_base_arg_reg(), self.vars_arg_reg());
        if base_disp != 0 {
            self.encoder
                .add_r64_imm32(dynamic_variable_base_arg_reg(), base_disp);
        }
        self.emit_usize_arg(dynamic_variable_len_arg_reg(), len);
        self.emit_i64_arg(dynamic_variable_lower_arg_reg(), lower);
        let helper: DynamicVariableSlotHelper = rspice_dynamic_variable_slot_native;
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);
        self.encoder.add_rsp_imm32(frame_bytes);
        self.depth = 0;
        Ok(())
    }

    fn emit_runtime_loop_limit_error_call(&mut self) {
        let frame_bytes = call_frame_bytes_for_slots(0);
        self.encoder.sub_rsp_imm32(frame_bytes);
        let helper: VoidHelper = rspice_native_loop_limit_error;
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);
        self.encoder.add_rsp_imm32(frame_bytes);
    }

    fn emit_binary_math(&mut self, op: BinaryMathOp) -> JitResult<()> {
        if self.depth < 2 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("binary math requires stack depth 2, found {}", self.depth).into(),
            });
        }

        let left = XMM_STACK[self.depth - 2];
        let right = XMM_STACK[self.depth - 1];
        self.emit_binary_helper_call(left, right, binary_math_helper(op));
        self.depth -= 1;
        Ok(())
    }

    fn emit_integer_binary(&mut self, op: IntegerBinaryOp) -> JitResult<()> {
        if self.depth < 2 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!(
                    "integer binary op requires stack depth 2, found {}",
                    self.depth
                )
                .into(),
            });
        }

        let left = XMM_STACK[self.depth - 2];
        let right = XMM_STACK[self.depth - 1];
        match op {
            IntegerBinaryOp::Shl | IntegerBinaryOp::Shr => {
                self.emit_integer_shift_op(left, right, op)?
            }
            IntegerBinaryOp::BitAnd | IntegerBinaryOp::BitOr | IntegerBinaryOp::BitXor => {
                self.emit_integer_bitwise_op(left, right, op)?
            }
        }
        self.depth -= 1;
        Ok(())
    }

    fn emit_integer_cast(&mut self) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "integer cast requires stack depth 1, found 0".into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        self.emit_rust_f64_to_i64(target, Gpr::R10)?;
        self.encoder.cvtsi2sd_xmm_r64(target, Gpr::R10);
        Ok(())
    }

    fn emit_integer_shift_op(
        &mut self,
        left: Xmm,
        right: Xmm,
        op: IntegerBinaryOp,
    ) -> JitResult<()> {
        #[cfg(windows)]
        let restore_entry_ctx = !self.saves_entry_args();
        #[cfg(windows)]
        if restore_entry_ctx {
            self.encoder.mov_r64_r64(Gpr::R10, entry_ctx_arg_reg());
        }

        self.emit_rust_f64_to_i64(left, Gpr::R11)?;
        self.emit_rust_f64_to_i64(right, Gpr::Rcx)?;
        self.encoder.test_r64_r64(Gpr::Rcx, Gpr::Rcx);
        let negative_count = self.encoder.jcc_rel32_placeholder(ConditionCode::Negative);
        self.encoder.cmp_r64_imm32(Gpr::Rcx, 64);
        let too_large_count = self
            .encoder
            .jcc_rel32_placeholder(ConditionCode::AboveOrEqual);
        match op {
            IntegerBinaryOp::Shl => self.encoder.shl_r64_cl(Gpr::R11),
            IntegerBinaryOp::Shr => self.encoder.sar_r64_cl(Gpr::R11),
            IntegerBinaryOp::BitAnd | IntegerBinaryOp::BitOr | IntegerBinaryOp::BitXor => {
                unreachable!("bitwise integer ops use emit_integer_bitwise_op")
            }
        }
        #[cfg(windows)]
        if restore_entry_ctx {
            self.encoder.mov_r64_r64(entry_ctx_arg_reg(), Gpr::R10);
        }
        self.encoder.cvtsi2sd_xmm_r64(left, Gpr::R11);
        let valid_count_done = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(negative_count)?;
        self.patch_rel32_to_current(too_large_count)?;
        self.emit_integer_shift_count_error_return();
        self.patch_rel32_to_current(valid_count_done)?;
        Ok(())
    }

    fn emit_integer_shift_const(&mut self, op: IntegerBinaryOp, count: u8) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "constant integer shift requires stack depth 1, found 0".into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        self.emit_rust_f64_to_i64(target, Gpr::R11)?;
        if count != 0 {
            match op {
                IntegerBinaryOp::Shl => self.encoder.shl_r64_imm8(Gpr::R11, count),
                IntegerBinaryOp::Shr => self.encoder.sar_r64_imm8(Gpr::R11, count),
                IntegerBinaryOp::BitAnd | IntegerBinaryOp::BitOr | IntegerBinaryOp::BitXor => {
                    unreachable!("constant integer shifts only accept shl/shr")
                }
            }
        }
        self.encoder.cvtsi2sd_xmm_r64(target, Gpr::R11);
        Ok(())
    }

    fn emit_integer_shift_count_error_return(&mut self) {
        self.emit_void_error_return(rspice_native_integer_shift_count_error);
    }

    fn emit_integer_bitwise_op(
        &mut self,
        left: Xmm,
        right: Xmm,
        op: IntegerBinaryOp,
    ) -> JitResult<()> {
        self.emit_rust_f64_to_i64(left, Gpr::R10)?;
        self.emit_rust_f64_to_i64(right, Gpr::R11)?;
        match op {
            IntegerBinaryOp::BitAnd => self.encoder.and_r64_r64(Gpr::R10, Gpr::R11),
            IntegerBinaryOp::BitOr => self.encoder.or_r64_r64(Gpr::R10, Gpr::R11),
            IntegerBinaryOp::BitXor => self.encoder.xor_r64_r64(Gpr::R10, Gpr::R11),
            IntegerBinaryOp::Shl | IntegerBinaryOp::Shr => {
                unreachable!("shift integer ops use emit_integer_shift_op")
            }
        }
        self.encoder.cvtsi2sd_xmm_r64(left, Gpr::R10);
        Ok(())
    }

    fn emit_integer_bitwise_const(&mut self, op: IntegerBinaryOp, value: i64) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "constant integer bitwise op requires stack depth 1, found 0".into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        self.emit_rust_f64_to_i64(target, Gpr::R10)?;
        if let Ok(value) = i32::try_from(value) {
            match op {
                IntegerBinaryOp::BitAnd => self.encoder.and_r64_imm32(Gpr::R10, value),
                IntegerBinaryOp::BitOr => self.encoder.or_r64_imm32(Gpr::R10, value),
                IntegerBinaryOp::BitXor => self.encoder.xor_r64_imm32(Gpr::R10, value),
                IntegerBinaryOp::Shl | IntegerBinaryOp::Shr => {
                    unreachable!("constant bitwise lowering only accepts bitwise ops")
                }
            }
        } else {
            self.emit_i64_arg(Gpr::R11, value);
            match op {
                IntegerBinaryOp::BitAnd => self.encoder.and_r64_r64(Gpr::R10, Gpr::R11),
                IntegerBinaryOp::BitOr => self.encoder.or_r64_r64(Gpr::R10, Gpr::R11),
                IntegerBinaryOp::BitXor => self.encoder.xor_r64_r64(Gpr::R10, Gpr::R11),
                IntegerBinaryOp::Shl | IntegerBinaryOp::Shr => {
                    unreachable!("constant bitwise lowering only accepts bitwise ops")
                }
            }
        }
        self.encoder.cvtsi2sd_xmm_r64(target, Gpr::R10);
        Ok(())
    }

    fn emit_current_load(
        &mut self,
        pair_index: usize,
        context_pointer_cache: &mut Option<i32>,
    ) -> JitResult<()> {
        self.emit_guarded_context_f64_slice_load(
            BRANCH_CURRENTS_OFFSET,
            BRANCH_CURRENTS_LEN_OFFSET,
            pair_index,
            rspice_native_current_probe_error,
            context_pointer_cache,
        )
    }

    fn emit_prior_current_load(
        &mut self,
        current_index: usize,
        context_pointer_cache: &mut Option<i32>,
    ) -> JitResult<()> {
        self.emit_guarded_context_f64_slice_load(
            CURRENTS_OFFSET,
            CURRENTS_LEN_OFFSET,
            current_index,
            rspice_native_prior_current_error,
            context_pointer_cache,
        )
    }

    fn emit_guarded_context_f64_slice_load(
        &mut self,
        pointer_field_offset: i32,
        len_field_offset: i32,
        index: usize,
        helper: VoidHelper,
        context_pointer_cache: &mut Option<i32>,
    ) -> JitResult<()> {
        let dst = self.push_register()?;
        let value_disp = byte_disp(index)?;

        self.emit_context_pointer_load_cached(pointer_field_offset, context_pointer_cache);
        self.encoder.test_r64_r64(Gpr::Rax, Gpr::Rax);
        let missing_storage = self.encoder.jcc_rel32_placeholder(ConditionCode::Equal);

        self.encoder
            .mov_r64_m64_base_disp32(Gpr::R10, self.ctx_arg_reg(), len_field_offset);
        self.encoder
            .cmp_r64_imm32(Gpr::R10, slice_index_imm32(index)?);
        let index_out_of_range = self
            .encoder
            .jcc_rel32_placeholder(ConditionCode::BelowOrEqual);

        self.encoder
            .movsd_xmm_m64_base_disp32(dst, Gpr::Rax, value_disp);
        let done = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(missing_storage)?;
        self.patch_rel32_to_current(index_out_of_range)?;
        self.emit_void_error_return(helper);

        self.patch_rel32_to_current(done)
    }

    fn emit_rust_f64_to_i64(&mut self, src: Xmm, dst: Gpr) -> JitResult<()> {
        self.encoder.ucomisd_xmm_xmm(src, src);
        let nan = self.encoder.jcc_rel32_placeholder(ConditionCode::Parity);
        self.emit_literal_compare(src, I64_MAX_EXCLUSIVE_AS_F64);
        let positive_saturation = self
            .encoder
            .jcc_rel32_placeholder(ConditionCode::AboveOrEqual);
        self.emit_literal_compare(src, I64_MIN_AS_F64);
        let negative_saturation = self
            .encoder
            .jcc_rel32_placeholder(ConditionCode::BelowOrEqual);

        self.encoder.cvttsd2si_r64_xmm(dst, src);
        let done_after_convert = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(nan)?;
        self.encoder.xor_r64_r64(dst, dst);
        let done_after_nan = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(positive_saturation)?;
        self.encoder.movabs_r64_imm64(dst, i64::MAX as u64);
        let done_after_positive_saturation = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(negative_saturation)?;
        self.encoder.movabs_r64_imm64(dst, i64::MIN as u64);

        self.patch_rel32_to_current(done_after_convert)?;
        self.patch_rel32_to_current(done_after_nan)?;
        self.patch_rel32_to_current(done_after_positive_saturation)
    }

    fn emit_table_helper_call(&mut self, table_id: usize, helper: TableHelper) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "table model helper requires stack depth 1, found 0".into(),
            });
        }

        debug_assert!(self.uses_helper_calls);
        let target = XMM_STACK[self.depth - 1];
        let frame_bytes =
            call_frame_bytes_for_slots(call_frame_spill_slot_count(self.depth, |_, register| {
                register != target
            }));
        self.encoder.sub_rsp_imm32(frame_bytes);
        self.emit_call_frame_spills(self.depth, |_, register| register != target);

        if target != Xmm::Xmm0 {
            self.encoder.movsd_xmm_xmm(Xmm::Xmm0, target);
        }
        let ctx = self.ctx_arg_reg();
        if table_ptr_arg_reg() == ctx {
            self.encoder.mov_r64_m64_base_disp32(
                table_len_arg_reg(),
                ctx,
                LOOKUP_TABLES_LEN_OFFSET,
            );
            self.encoder
                .mov_r64_m64_base_disp32(table_ptr_arg_reg(), ctx, LOOKUP_TABLES_OFFSET);
        } else {
            self.encoder
                .mov_r64_m64_base_disp32(table_ptr_arg_reg(), ctx, LOOKUP_TABLES_OFFSET);
            self.encoder.mov_r64_m64_base_disp32(
                table_len_arg_reg(),
                ctx,
                LOOKUP_TABLES_LEN_OFFSET,
            );
        }
        self.emit_usize_arg(table_id_arg_reg(), table_id);
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);

        self.emit_helper_result_to_target_and_restore(target, self.depth, |_, _| true);
        self.encoder.add_rsp_imm32(frame_bytes);
        Ok(())
    }

    fn emit_context_filter_helper_call(
        &mut self,
        target: Xmm,
        filter_id: usize,
        helper: ContextFilterHelper,
    ) -> JitResult<()> {
        debug_assert!(self.uses_helper_calls);
        debug_assert!(xmm_stack_slot(target) < self.depth);
        let frame_bytes =
            call_frame_bytes_for_slots(call_frame_spill_slot_count(self.depth, |_, register| {
                register != target
            }));
        self.encoder.sub_rsp_imm32(frame_bytes);
        self.emit_call_frame_spills(self.depth, |_, register| register != target);

        if target != Xmm::Xmm0 {
            self.encoder.movsd_xmm_xmm(Xmm::Xmm0, target);
        }
        self.encoder
            .mov_r64_r64(context_filter_ctx_arg_reg(), self.ctx_arg_reg());
        self.emit_usize_arg(context_filter_id_arg_reg(), filter_id);
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);

        self.emit_helper_result_to_target_and_restore(target, self.depth, |_, _| true);
        self.encoder.add_rsp_imm32(frame_bytes);
        Ok(())
    }

    fn emit_limit_state(&mut self, state_index: usize) -> JitResult<()> {
        if self.depth < 2 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("limit state requires stack depth 2, found {}", self.depth).into(),
            });
        }

        let value = XMM_STACK[self.depth - 2];
        let step = XMM_STACK[self.depth - 1];
        let state_disp = byte_disp(state_index)?;
        let initialized_disp = byte_disp_u8(state_index)?;
        let state_index_i32 = i32::try_from(state_index).map_err(|_| JitError::Encoding {
            model: MODEL.into(),
            detail: format!("state index {state_index} exceeds x64 imm32 range").into(),
        })?;

        self.emit_context_pointer_load(STATE_VALUES_OFFSET);
        self.encoder.test_r64_r64(Gpr::Rax, Gpr::Rax);
        let no_state = self.encoder.jcc_rel32_placeholder(ConditionCode::Equal);

        self.encoder
            .mov_r64_m64_base_disp32(Gpr::R11, self.ctx_arg_reg(), STATE_VALUES_LEN_OFFSET);
        self.encoder.cmp_r64_imm32(Gpr::R11, state_index_i32);
        let state_values_out_of_range = self
            .encoder
            .jcc_rel32_placeholder(ConditionCode::BelowOrEqual);

        self.encoder.mov_r64_m64_base_disp32(
            Gpr::R10,
            self.ctx_arg_reg(),
            STATE_INITIALIZED_OFFSET,
        );
        self.encoder.test_r64_r64(Gpr::R10, Gpr::R10);
        let no_initialized_flags = self.encoder.jcc_rel32_placeholder(ConditionCode::Equal);

        self.encoder.mov_r64_m64_base_disp32(
            Gpr::R11,
            self.ctx_arg_reg(),
            STATE_INITIALIZED_LEN_OFFSET,
        );
        self.encoder.cmp_r64_imm32(Gpr::R11, state_index_i32);
        let initialized_flags_out_of_range = self
            .encoder
            .jcc_rel32_placeholder(ConditionCode::BelowOrEqual);

        self.encoder
            .movzx_r32_m8_base_disp32(Gpr::R11, Gpr::R10, initialized_disp);
        self.encoder.test_r8_r8(Gpr::R11, Gpr::R11);
        let first_evaluation = self.encoder.jcc_rel32_placeholder(ConditionCode::Equal);

        if self.depth < XMM_STACK.len() {
            let positive_step = self.scratch_register()?;
            self.encoder.movsd_xmm_xmm(positive_step, step);
            self.emit_limit_state_clamp_delta(value, step, state_disp, |compiler, value| {
                compiler.encoder.minsd_xmm_xmm(value, positive_step);
            })?;
        } else {
            self.encoder.sub_rsp_imm32(WORD_BYTES as i32);
            self.encoder.movsd_m64_base_disp32_xmm(Gpr::Rsp, 0, step);
            self.emit_limit_state_clamp_delta(value, step, state_disp, |compiler, value| {
                compiler
                    .encoder
                    .minsd_xmm_m64_base_disp32(value, Gpr::Rsp, 0);
            })?;
            self.encoder.add_rsp_imm32(WORD_BYTES as i32);
        }

        self.patch_rel32_to_current(first_evaluation)?;
        self.encoder
            .movsd_m64_base_disp32_xmm(Gpr::Rax, state_disp, value);
        self.encoder
            .mov_m8_base_disp32_imm8(Gpr::R10, initialized_disp, 1);
        let done_after_initialized_store = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(no_initialized_flags)?;
        self.emit_limit_state_error_return(rspice_native_limit_state_initialized_error);

        self.patch_rel32_to_current(initialized_flags_out_of_range)?;
        self.emit_limit_state_error_return(rspice_native_limit_state_bounds_error);

        self.patch_rel32_to_current(state_values_out_of_range)?;
        self.emit_limit_state_error_return(rspice_native_limit_state_values_bounds_error);

        self.patch_rel32_to_current(no_state)?;
        self.emit_limit_state_error_return(rspice_native_limit_state_values_error);

        self.patch_rel32_to_current(done_after_initialized_store)?;
        self.depth -= 1;
        Ok(())
    }

    fn emit_limit_state_clamp_delta(
        &mut self,
        value: Xmm,
        step: Xmm,
        state_disp: i32,
        emit_upper_clamp: impl FnOnce(&mut Self, Xmm),
    ) -> JitResult<()> {
        self.encoder
            .subsd_xmm_m64_base_disp32(value, Gpr::Rax, state_disp);
        self.encoder.ucomisd_xmm_xmm(value, value);
        let unordered_delta = self.encoder.jcc_rel32_placeholder(ConditionCode::Parity);
        self.encoder.movq_r64_xmm(Gpr::R11, step);
        self.encoder.btc_r64_imm8(Gpr::R11, 63);
        self.encoder.movq_xmm_r64(step, Gpr::R11);
        self.encoder.maxsd_xmm_xmm(value, step);
        emit_upper_clamp(self, value);
        self.encoder
            .addsd_xmm_m64_base_disp32(value, Gpr::Rax, state_disp);
        self.patch_rel32_to_current(unordered_delta)?;
        Ok(())
    }

    fn emit_limit_state_error_return(&mut self, helper: VoidHelper) {
        self.emit_void_error_return(helper);
    }

    fn emit_white_noise(&mut self) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "white noise requires stack depth 1, found 0".into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        self.encoder.xorpd_xmm_xmm(target, target);
        Ok(())
    }

    fn emit_flicker_noise(&mut self) -> JitResult<()> {
        if self.depth < 2 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("flicker noise requires stack depth 2, found {}", self.depth)
                    .into(),
            });
        }

        let target = XMM_STACK[self.depth - 2];
        self.encoder.xorpd_xmm_xmm(target, target);
        self.depth -= 1;
        Ok(())
    }

    fn emit_laplace_state(&mut self, filter_id: usize) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "laplace state requires stack depth 1, found 0".into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        self.emit_context_filter_helper_call(target, filter_id, rspice_laplace_step_native)
    }

    fn emit_zi_state(&mut self, filter_id: usize) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "zi state requires stack depth 1, found 0".into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        self.emit_context_filter_helper_call(target, filter_id, rspice_zi_step_native)
    }

    fn emit_timer_state(&mut self, _timer_id: usize) -> JitResult<()> {
        if self.depth < 2 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("timer state requires stack depth 2, found {}", self.depth).into(),
            });
        }

        let start = XMM_STACK[self.depth - 2];
        let period = XMM_STACK[self.depth - 1];
        self.emit_timer_helper_call(start, period, rspice_timer_state_native);
        self.depth -= 1;
        Ok(())
    }

    fn emit_transition_state(&mut self, filter_id: usize) -> JitResult<()> {
        if self.depth < 4 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!(
                    "transition state requires stack depth 4, found {}",
                    self.depth
                )
                .into(),
            });
        }

        let input = XMM_STACK[self.depth - 4];
        self.emit_operand_context_filter_helper_call(
            input,
            4,
            filter_id,
            rspice_transition_state_native,
        );
        self.depth -= 3;
        Ok(())
    }

    fn emit_slew_state(&mut self, filter_id: usize) -> JitResult<()> {
        if self.depth < 3 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("slew state requires stack depth 3, found {}", self.depth).into(),
            });
        }

        let input = XMM_STACK[self.depth - 3];
        self.emit_operand_context_filter_helper_call(input, 3, filter_id, rspice_slew_state_native);
        self.depth -= 2;
        Ok(())
    }

    fn emit_absdelay_state(&mut self, buffer_id: usize) -> JitResult<()> {
        if self.depth < 2 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!(
                    "absdelay state requires stack depth 2, found {}",
                    self.depth
                )
                .into(),
            });
        }

        let input = XMM_STACK[self.depth - 2];
        self.emit_operand_context_filter_helper_call(
            input,
            2,
            buffer_id,
            rspice_absdelay_state_native,
        );
        self.depth -= 1;
        Ok(())
    }

    fn emit_cross_state(&mut self, detector_id: usize) -> JitResult<()> {
        if self.depth < 2 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("cross state requires stack depth 2, found {}", self.depth).into(),
            });
        }

        let input = XMM_STACK[self.depth - 2];
        self.emit_operand_context_filter_helper_call(
            input,
            2,
            detector_id,
            rspice_cross_state_native,
        );
        self.depth -= 1;
        Ok(())
    }

    fn emit_ddt_state(&mut self, state_index: usize) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "ddt state requires stack depth 1, found 0".into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        if self.depth >= XMM_STACK.len() {
            let value_disp = self.stateful_scratch_disp(0)?;
            let prior_disp = self.stateful_scratch_disp(1)?;

            self.encoder
                .movsd_m64_base_disp32_xmm(Gpr::Rsp, value_disp, target);
            self.emit_state_prev_load_if_available(state_index, target)?;
            self.encoder
                .movsd_m64_base_disp32_xmm(Gpr::Rsp, prior_disp, target);
            self.encoder
                .movsd_xmm_m64_base_disp32(target, Gpr::Rsp, value_disp);
            self.emit_state_value_store(state_index, target)?;

            self.encoder
                .subsd_xmm_m64_base_disp32(target, Gpr::Rsp, prior_disp);
            return self.emit_timestep_guarded_scale_from_frame(target, BinaryOp::Div);
        }

        let scratch = self.scratch_register()?;
        self.encoder.movsd_xmm_xmm(scratch, target);
        self.emit_state_prev_load_if_available(state_index, scratch)?;
        self.emit_state_value_store(state_index, target)?;

        self.encoder.subsd_xmm_xmm(target, scratch);
        self.emit_timestep_guarded_scale(target, scratch, BinaryOp::Div)
    }

    fn emit_ddt_jacobian(&mut self) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "ddt jacobian requires stack depth 1, found 0".into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        if self.depth < XMM_STACK.len() {
            let scratch = self.scratch_register()?;
            self.emit_timestep_guarded_scale(target, scratch, BinaryOp::Div)
        } else {
            self.emit_timestep_guarded_scale_from_frame(target, BinaryOp::Div)
        }
    }

    fn emit_idt_state(&mut self, state_index: usize) -> JitResult<()> {
        if self.depth < 2 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("idt state requires stack depth 2, found {}", self.depth).into(),
            });
        }

        let value = XMM_STACK[self.depth - 2];
        let ic = XMM_STACK[self.depth - 1];
        if self.depth >= XMM_STACK.len() {
            let value_disp = self.stateful_scratch_disp(0)?;
            let timestep_disp = self.stateful_scratch_disp(1)?;

            self.encoder
                .movsd_m64_base_disp32_xmm(Gpr::Rsp, value_disp, value);
            self.encoder
                .movsd_xmm_m64_base_disp32(value, self.ctx_arg_reg(), TIMESTEP_OFFSET);
            self.encoder.movq_r64_xmm(Gpr::R11, value);
            self.emit_abs_register(value);
            self.emit_literal_compare(value, TIMESTEP_DC_EPSILON);

            let non_dc_path = self.encoder.jcc_rel32_placeholder(ConditionCode::Above);
            self.encoder.movsd_xmm_xmm(value, ic);
            self.emit_state_value_store(state_index, value)?;
            let done = self.encoder.jmp_rel32_placeholder();

            self.patch_rel32_to_current(non_dc_path)?;
            self.emit_state_prev_load_if_available(state_index, ic)?;

            self.encoder.movq_xmm_r64(value, Gpr::R11);
            self.encoder
                .movsd_m64_base_disp32_xmm(Gpr::Rsp, timestep_disp, value);
            self.encoder
                .movsd_xmm_m64_base_disp32(value, Gpr::Rsp, value_disp);
            self.encoder
                .mulsd_xmm_m64_base_disp32(value, Gpr::Rsp, timestep_disp);
            self.encoder.addsd_xmm_xmm(value, ic);
            self.emit_state_value_store(state_index, value)?;

            self.patch_rel32_to_current(done)?;
            self.depth -= 1;
            return Ok(());
        }

        let scratch = self.scratch_register()?;
        self.encoder
            .movsd_xmm_m64_base_disp32(scratch, self.ctx_arg_reg(), TIMESTEP_OFFSET);
        self.encoder.movq_r64_xmm(Gpr::R11, scratch);
        self.emit_abs_register(scratch);
        self.emit_literal_compare(scratch, TIMESTEP_DC_EPSILON);

        let non_dc_path = self.encoder.jcc_rel32_placeholder(ConditionCode::Above);
        self.encoder.movsd_xmm_xmm(value, ic);
        self.emit_state_value_store(state_index, value)?;
        let done = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(non_dc_path)?;
        self.emit_state_prev_load_if_available(state_index, ic)?;

        self.encoder.movq_xmm_r64(scratch, Gpr::R11);
        self.encoder.mulsd_xmm_xmm(value, scratch);
        self.encoder.addsd_xmm_xmm(value, ic);
        self.emit_state_value_store(state_index, value)?;

        self.patch_rel32_to_current(done)?;
        self.depth -= 1;
        Ok(())
    }

    fn emit_idt_jacobian(&mut self) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "idt jacobian requires stack depth 1, found 0".into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        if self.depth < XMM_STACK.len() {
            let scratch = self.scratch_register()?;
            self.emit_timestep_guarded_scale(target, scratch, BinaryOp::Mul)
        } else {
            self.emit_timestep_guarded_scale_from_frame(target, BinaryOp::Mul)
        }
    }

    fn emit_idtmod_state(&mut self, state_index: usize) -> JitResult<()> {
        if self.depth < 4 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("idtmod state requires stack depth 4, found {}", self.depth).into(),
            });
        }

        let value = XMM_STACK[self.depth - 4];
        let ic = XMM_STACK[self.depth - 3];
        let modulus = XMM_STACK[self.depth - 2];
        let offset = XMM_STACK[self.depth - 1];
        if self.depth >= XMM_STACK.len() {
            let value_disp = self.stateful_scratch_disp(0)?;
            let timestep_disp = self.stateful_scratch_disp(1)?;

            self.encoder
                .movsd_m64_base_disp32_xmm(Gpr::Rsp, value_disp, value);
            self.encoder
                .movsd_xmm_m64_base_disp32(value, self.ctx_arg_reg(), TIMESTEP_OFFSET);
            self.encoder.movq_r64_xmm(Gpr::R11, value);
            self.emit_abs_register(value);
            self.emit_literal_compare(value, TIMESTEP_DC_EPSILON);

            let non_dc_path = self.encoder.jcc_rel32_placeholder(ConditionCode::Above);
            self.encoder.movsd_xmm_xmm(value, ic);
            self.emit_consuming_ternary_helper_call(value, modulus, offset, rspice_idtmod_wrap);
            self.emit_state_value_store(state_index, value)?;
            let done = self.encoder.jmp_rel32_placeholder();

            self.patch_rel32_to_current(non_dc_path)?;
            self.emit_state_prev_load_if_available(state_index, ic)?;

            self.encoder.movq_xmm_r64(value, Gpr::R11);
            self.encoder
                .movsd_m64_base_disp32_xmm(Gpr::Rsp, timestep_disp, value);
            self.encoder
                .movsd_xmm_m64_base_disp32(value, Gpr::Rsp, value_disp);
            self.encoder
                .mulsd_xmm_m64_base_disp32(value, Gpr::Rsp, timestep_disp);
            self.encoder.addsd_xmm_xmm(value, ic);
            self.emit_consuming_ternary_helper_call(value, modulus, offset, rspice_idtmod_wrap);
            self.emit_state_value_store(state_index, value)?;

            self.patch_rel32_to_current(done)?;
            self.depth -= 3;
            return Ok(());
        }

        let scratch = self.scratch_register()?;
        self.encoder
            .movsd_xmm_m64_base_disp32(scratch, self.ctx_arg_reg(), TIMESTEP_OFFSET);
        self.encoder.movq_r64_xmm(Gpr::R11, scratch);
        self.emit_abs_register(scratch);
        self.emit_literal_compare(scratch, TIMESTEP_DC_EPSILON);

        let non_dc_path = self.encoder.jcc_rel32_placeholder(ConditionCode::Above);
        self.encoder.movsd_xmm_xmm(value, ic);
        self.emit_consuming_ternary_helper_call(value, modulus, offset, rspice_idtmod_wrap);
        self.emit_state_value_store(state_index, value)?;
        let done = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(non_dc_path)?;
        self.emit_state_prev_load_if_available(state_index, ic)?;

        self.encoder.movq_xmm_r64(scratch, Gpr::R11);
        self.encoder.mulsd_xmm_xmm(value, scratch);
        self.encoder.addsd_xmm_xmm(value, ic);
        self.emit_consuming_ternary_helper_call(value, modulus, offset, rspice_idtmod_wrap);
        self.emit_state_value_store(state_index, value)?;

        self.patch_rel32_to_current(done)?;
        self.depth -= 3;
        Ok(())
    }

    fn emit_timestep_guarded_scale(
        &mut self,
        target: Xmm,
        scratch: Xmm,
        op: BinaryOp,
    ) -> JitResult<()> {
        self.encoder
            .movsd_xmm_m64_base_disp32(scratch, self.ctx_arg_reg(), TIMESTEP_OFFSET);
        self.encoder.movq_r64_xmm(Gpr::R11, scratch);
        self.emit_abs_register(scratch);
        self.emit_literal_compare(scratch, TIMESTEP_DC_EPSILON);

        let non_dc_path = self.encoder.jcc_rel32_placeholder(ConditionCode::Above);
        self.encoder.xorpd_xmm_xmm(target, target);
        let done = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(non_dc_path)?;
        self.encoder.movq_xmm_r64(scratch, Gpr::R11);
        match op {
            BinaryOp::Mul => self.encoder.mulsd_xmm_xmm(target, scratch),
            BinaryOp::Div => self.encoder.divsd_xmm_xmm(target, scratch),
            BinaryOp::Add | BinaryOp::Sub => unreachable!("timestep scaling supports mul/div"),
        }

        self.patch_rel32_to_current(done)
    }

    fn emit_timestep_guarded_scale_from_frame(
        &mut self,
        target: Xmm,
        op: BinaryOp,
    ) -> JitResult<()> {
        let value_disp = self.stateful_scratch_disp(0)?;
        let timestep_disp = self.stateful_scratch_disp(1)?;

        self.encoder
            .movsd_m64_base_disp32_xmm(Gpr::Rsp, value_disp, target);
        self.encoder
            .movsd_xmm_m64_base_disp32(target, self.ctx_arg_reg(), TIMESTEP_OFFSET);
        self.encoder.movq_r64_xmm(Gpr::R11, target);
        self.emit_abs_register(target);
        self.emit_literal_compare(target, TIMESTEP_DC_EPSILON);

        let non_dc_path = self.encoder.jcc_rel32_placeholder(ConditionCode::Above);
        self.encoder.xorpd_xmm_xmm(target, target);
        let done = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(non_dc_path)?;
        self.encoder.movq_xmm_r64(target, Gpr::R11);
        self.encoder
            .movsd_m64_base_disp32_xmm(Gpr::Rsp, timestep_disp, target);
        self.encoder
            .movsd_xmm_m64_base_disp32(target, Gpr::Rsp, value_disp);
        match op {
            BinaryOp::Mul => {
                self.encoder
                    .mulsd_xmm_m64_base_disp32(target, Gpr::Rsp, timestep_disp);
            }
            BinaryOp::Div => {
                self.encoder
                    .divsd_xmm_m64_base_disp32(target, Gpr::Rsp, timestep_disp);
            }
            BinaryOp::Add | BinaryOp::Sub => unreachable!("timestep scaling supports mul/div"),
        }

        self.patch_rel32_to_current(done)
    }

    fn emit_state_value_store(&mut self, state_index: usize, src: Xmm) -> JitResult<()> {
        let state_disp = byte_disp(state_index)?;
        let state_index_i32 = state_index_imm32(state_index)?;

        self.emit_context_pointer_load(STATE_VALUES_OFFSET);
        self.encoder.test_r64_r64(Gpr::Rax, Gpr::Rax);
        let missing_state = self.encoder.jcc_rel32_placeholder(ConditionCode::Equal);

        self.encoder
            .mov_r64_m64_base_disp32(Gpr::R10, self.ctx_arg_reg(), STATE_VALUES_LEN_OFFSET);
        self.encoder.cmp_r64_imm32(Gpr::R10, state_index_i32);
        let state_values_out_of_range = self
            .encoder
            .jcc_rel32_placeholder(ConditionCode::BelowOrEqual);

        self.encoder
            .movsd_m64_base_disp32_xmm(Gpr::Rax, state_disp, src);
        let done = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(missing_state)?;
        self.emit_state_values_error_return();

        self.patch_rel32_to_current(state_values_out_of_range)?;
        self.emit_state_values_bounds_error_return();

        self.patch_rel32_to_current(done)
    }

    fn emit_state_prev_load_if_available(&mut self, state_index: usize, dst: Xmm) -> JitResult<()> {
        let state_disp = byte_disp(state_index)?;
        let state_index_i32 = state_index_imm32(state_index)?;

        self.emit_context_pointer_load(STATE_PREV_OFFSET);
        self.encoder.test_r64_r64(Gpr::Rax, Gpr::Rax);
        let no_previous_state = self.encoder.jcc_rel32_placeholder(ConditionCode::Equal);

        self.encoder
            .mov_r64_m64_base_disp32(Gpr::R10, self.ctx_arg_reg(), STATE_PREV_LEN_OFFSET);
        self.encoder.cmp_r64_imm32(Gpr::R10, state_index_i32);
        let previous_state_out_of_range = self
            .encoder
            .jcc_rel32_placeholder(ConditionCode::BelowOrEqual);

        self.encoder
            .movsd_xmm_m64_base_disp32(dst, Gpr::Rax, state_disp);
        let done = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(previous_state_out_of_range)?;
        self.emit_state_prev_bounds_error_return();

        self.patch_rel32_to_current(no_previous_state)?;
        self.patch_rel32_to_current(done)
    }

    fn emit_state_values_error_return(&mut self) {
        self.emit_void_error_return(rspice_native_state_values_error);
    }

    fn emit_state_values_bounds_error_return(&mut self) {
        self.emit_void_error_return(rspice_native_state_values_bounds_error);
    }

    fn emit_state_prev_bounds_error_return(&mut self) {
        self.emit_void_error_return(rspice_native_state_prev_bounds_error);
    }

    fn emit_void_error_return(&mut self, helper: VoidHelper) {
        let frame_bytes = call_frame_bytes_for_slots(0);
        self.encoder.sub_rsp_imm32(frame_bytes);
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);
        self.encoder.add_rsp_imm32(frame_bytes);
        self.encoder.xorpd_xmm_xmm(Xmm::Xmm0, Xmm::Xmm0);
        let return_after_error = self.encoder.jmp_rel32_placeholder();
        self.early_return_jumps.push(return_after_error);
    }

    fn emit_binary_helper_call(&mut self, left: Xmm, right: Xmm, helper: BinaryHelper) {
        debug_assert!(self.uses_helper_calls);
        let frame_bytes =
            call_frame_bytes_for_slots(call_frame_spill_slot_count(self.depth, |_, register| {
                register != left && register != right
            }));
        self.encoder.sub_rsp_imm32(frame_bytes);
        self.emit_call_frame_spills(self.depth, |_, register| {
            register != left && register != right
        });

        if left != Xmm::Xmm0 {
            self.encoder.movsd_xmm_xmm(Xmm::Xmm0, left);
        }
        if right != Xmm::Xmm1 {
            self.encoder.movsd_xmm_xmm(Xmm::Xmm1, right);
        }
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);

        self.emit_helper_result_to_target_and_restore(left, self.depth, |_, register| {
            register != right
        });
        self.encoder.add_rsp_imm32(frame_bytes);
    }

    fn emit_consuming_ternary_helper_call(
        &mut self,
        target: Xmm,
        arg1: Xmm,
        arg2: Xmm,
        helper: TernaryHelper,
    ) {
        debug_assert!(self.uses_helper_calls);
        let target_slot = xmm_stack_slot(target);
        let arg1_slot = xmm_stack_slot(arg1);
        let arg2_slot = xmm_stack_slot(arg2);
        debug_assert!(target_slot < self.depth);
        debug_assert!(target_slot < arg1_slot);
        debug_assert!(arg1_slot < arg2_slot);
        debug_assert!(arg2_slot < self.depth);

        let frame_bytes = call_frame_bytes_for_slots(target_slot);
        self.encoder.sub_rsp_imm32(frame_bytes);
        self.emit_call_frame_spills(target_slot, |_, _| true);

        if target != Xmm::Xmm0 {
            self.encoder.movsd_xmm_xmm(Xmm::Xmm0, target);
        }
        if arg1 != Xmm::Xmm1 {
            self.encoder.movsd_xmm_xmm(Xmm::Xmm1, arg1);
        }
        if arg2 != Xmm::Xmm2 {
            self.encoder.movsd_xmm_xmm(Xmm::Xmm2, arg2);
        }
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);

        self.emit_helper_result_to_target_and_restore(target, target_slot, |_, _| true);
        self.encoder.add_rsp_imm32(frame_bytes);
    }

    fn emit_timer_helper_call(&mut self, start: Xmm, period: Xmm, helper: TimerHelper) {
        debug_assert!(self.uses_helper_calls);
        let frame_bytes =
            call_frame_bytes_for_slots(call_frame_spill_slot_count(self.depth, |_, register| {
                register != start && register != period
            }));
        self.encoder.sub_rsp_imm32(frame_bytes);
        self.emit_call_frame_spills(self.depth, |_, register| {
            register != start && register != period
        });

        if start != Xmm::Xmm0 {
            self.encoder.movsd_xmm_xmm(Xmm::Xmm0, start);
        }
        if period != Xmm::Xmm1 {
            self.encoder.movsd_xmm_xmm(Xmm::Xmm1, period);
        }
        self.encoder
            .mov_r64_r64(timer_ctx_arg_reg(), self.ctx_arg_reg());
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);

        self.emit_helper_result_to_target_and_restore(start, self.depth, |_, register| {
            register != period
        });
        self.encoder.add_rsp_imm32(frame_bytes);
    }

    fn emit_operand_context_filter_helper_call(
        &mut self,
        input: Xmm,
        operand_count: usize,
        filter_id: usize,
        helper: OperandContextFilterHelper,
    ) {
        debug_assert!(self.uses_helper_calls);
        let input_slot = xmm_stack_slot(input);
        debug_assert!(operand_count > 0);
        debug_assert!(input_slot + operand_count <= self.depth);

        let frame_bytes = call_frame_bytes_for_slots(self.depth);
        self.encoder.sub_rsp_imm32(frame_bytes);
        self.encoder.mov_r64_r64(Gpr::R11, Gpr::Rsp);
        for (index, register) in XMM_STACK.iter().copied().take(self.depth).enumerate() {
            self.encoder
                .movsd_m64_base_disp32_xmm(Gpr::R11, call_spill_disp(index), register);
        }

        self.encoder
            .mov_r64_r64(operand_filter_ctx_arg_reg(), self.ctx_arg_reg());
        self.encoder
            .mov_r64_r64(operand_filter_operands_arg_reg(), Gpr::R11);
        let operands_disp = call_spill_disp(input_slot);
        if operands_disp != 0 {
            self.encoder
                .add_r64_imm32(operand_filter_operands_arg_reg(), operands_disp);
        }
        self.emit_usize_arg(operand_filter_id_arg_reg(), filter_id);
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);

        self.emit_helper_result_to_target_and_restore(input, input_slot, |_, _| true);
        self.encoder.add_rsp_imm32(frame_bytes);
    }

    fn emit_compare(&mut self, op: CompareOp) -> JitResult<()> {
        if self.depth < 2 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("comparison requires stack depth 2, found {}", self.depth).into(),
            });
        }

        let left = XMM_STACK[self.depth - 2];
        let right = XMM_STACK[self.depth - 1];
        let condition = match op {
            CompareOp::Gt => {
                self.encoder.ucomisd_xmm_xmm(left, right);
                ConditionCode::Above
            }
            CompareOp::Ge => {
                self.encoder.ucomisd_xmm_xmm(left, right);
                ConditionCode::AboveOrEqual
            }
            CompareOp::Lt => {
                self.encoder.ucomisd_xmm_xmm(right, left);
                ConditionCode::Above
            }
            CompareOp::Le => {
                self.encoder.ucomisd_xmm_xmm(right, left);
                ConditionCode::AboveOrEqual
            }
            CompareOp::Eq => {
                self.encoder.subsd_xmm_xmm(left, right);
                self.emit_abs_register(left);
                self.emit_literal_load(right, BOOLEAN_EPSILON);
                self.encoder.ucomisd_xmm_xmm(right, left);
                ConditionCode::Above
            }
            CompareOp::Ne => {
                self.encoder.subsd_xmm_xmm(left, right);
                self.emit_abs_register(left);
                self.emit_literal_load(right, BOOLEAN_EPSILON);
                self.encoder.ucomisd_xmm_xmm(left, right);
                ConditionCode::AboveOrEqual
            }
        };
        self.encoder.setcc_r8(condition, Gpr::R10);
        self.encoder.movzx_r32_r8(Gpr::R10, Gpr::R10);
        self.encoder.cvtsi2sd_xmm_r32(left, Gpr::R10);
        self.depth -= 1;
        Ok(())
    }

    fn emit_compare_const(&mut self, op: CompareOp, value: f64) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "literal RHS comparison requires stack depth 1, found 0".into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        match op {
            CompareOp::Gt => {
                self.emit_literal_compare(target, value);
                self.emit_condition_result(target, ConditionCode::Above);
            }
            CompareOp::Ge => {
                self.emit_literal_compare(target, value);
                self.emit_condition_result(target, ConditionCode::AboveOrEqual);
            }
            CompareOp::Lt => {
                self.emit_literal_compare(target, value);
                self.emit_ordered_condition_result(target, ConditionCode::Below);
            }
            CompareOp::Le => {
                self.emit_literal_compare(target, value);
                self.emit_ordered_condition_result(target, ConditionCode::BelowOrEqual);
            }
            CompareOp::Eq => {
                if value != 0.0 {
                    self.emit_literal_binary_op(target, value, BinaryOp::Sub);
                }
                self.emit_abs_register(target);
                self.emit_literal_compare(target, BOOLEAN_EPSILON);
                self.emit_ordered_condition_result(target, ConditionCode::Below);
            }
            CompareOp::Ne => {
                if value != 0.0 {
                    self.emit_literal_binary_op(target, value, BinaryOp::Sub);
                }
                self.emit_abs_register(target);
                self.emit_literal_compare(target, BOOLEAN_EPSILON);
                self.emit_condition_result(target, ConditionCode::AboveOrEqual);
            }
        }
        Ok(())
    }

    fn emit_condition_result(&mut self, dst: Xmm, condition: ConditionCode) {
        self.encoder.setcc_r8(condition, Gpr::R10);
        self.encoder.movzx_r32_r8(Gpr::R10, Gpr::R10);
        self.encoder.cvtsi2sd_xmm_r32(dst, Gpr::R10);
    }

    fn emit_ordered_condition_result(&mut self, dst: Xmm, condition: ConditionCode) {
        self.encoder.setcc_r8(condition, Gpr::R10);
        self.encoder.setcc_r8(ConditionCode::NotParity, Gpr::R11);
        self.encoder.and_r8_r8(Gpr::R10, Gpr::R11);
        self.encoder.movzx_r32_r8(Gpr::R10, Gpr::R10);
        self.encoder.cvtsi2sd_xmm_r32(dst, Gpr::R10);
    }

    fn emit_logical(&mut self, op: LogicalOp) -> JitResult<()> {
        match op {
            LogicalOp::And | LogicalOp::Or => self.emit_logical_binary(op),
            LogicalOp::Not => self.emit_logical_not(),
        }
    }

    fn emit_logical_binary(&mut self, op: LogicalOp) -> JitResult<()> {
        if self.depth < 2 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("logical op requires stack depth 2, found {}", self.depth).into(),
            });
        }

        let left = XMM_STACK[self.depth - 2];
        let right = XMM_STACK[self.depth - 1];
        self.emit_truthy_to_gpr(right, Gpr::R11);
        self.emit_truthy_to_gpr(left, Gpr::R10);
        match op {
            LogicalOp::And => self.encoder.and_r8_r8(Gpr::R10, Gpr::R11),
            LogicalOp::Or => self.encoder.or_r8_r8(Gpr::R10, Gpr::R11),
            LogicalOp::Not => unreachable!("logical binary lowering only accepts and/or"),
        }
        self.encoder.movzx_r32_r8(Gpr::R10, Gpr::R10);
        self.encoder.cvtsi2sd_xmm_r32(left, Gpr::R10);
        self.depth -= 1;
        Ok(())
    }

    fn emit_logical_const(&mut self, op: LogicalOp, rhs_truthy: bool) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "literal RHS logical op requires stack depth 1, found 0".into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        match (op, rhs_truthy) {
            (LogicalOp::And, true) | (LogicalOp::Or, false) => {
                self.emit_truthy_to_gpr(target, Gpr::R10);
                self.emit_gpr_bool_result(target, Gpr::R10);
            }
            (LogicalOp::And, false) => self.emit_bool_result(target, false),
            (LogicalOp::Or, true) => self.emit_bool_result(target, true),
            (LogicalOp::Not, _) => unreachable!("logical constant RHS only accepts and/or"),
        }
        Ok(())
    }

    fn emit_logical_not(&mut self) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "logical not requires stack depth 1, found 0".into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        self.emit_falsy_to_gpr(target, Gpr::R10, Gpr::R11);
        self.emit_gpr_bool_result(target, Gpr::R10);
        Ok(())
    }

    fn emit_truthy_to_gpr(&mut self, value: Xmm, dst: Gpr) {
        self.emit_abs_register(value);
        self.emit_literal_compare(value, BOOLEAN_EPSILON);
        self.encoder.setcc_r8(ConditionCode::Above, dst);
    }

    fn emit_bool_result(&mut self, dst: Xmm, value: bool) {
        if value {
            self.encoder.mov_r32_imm32(Gpr::R10, 1);
            self.encoder.cvtsi2sd_xmm_r32(dst, Gpr::R10);
        } else {
            self.encoder.xorpd_xmm_xmm(dst, dst);
        }
    }

    fn emit_gpr_bool_result(&mut self, dst: Xmm, src: Gpr) {
        self.encoder.movzx_r32_r8(src, src);
        self.encoder.cvtsi2sd_xmm_r32(dst, src);
    }

    fn emit_falsy_to_gpr(&mut self, value: Xmm, dst: Gpr, scratch: Gpr) {
        self.emit_abs_register(value);
        self.emit_literal_compare(value, BOOLEAN_EPSILON);
        self.encoder.setcc_r8(ConditionCode::Below, dst);
        self.encoder.setcc_r8(ConditionCode::NotParity, scratch);
        self.encoder.and_r8_r8(dst, scratch);
    }

    fn emit_ifelse(&mut self) -> JitResult<()> {
        if self.depth < 3 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("ifelse requires stack depth 3, found {}", self.depth).into(),
            });
        }

        let cond = XMM_STACK[self.depth - 3];
        let then_value = XMM_STACK[self.depth - 2];
        let else_value = XMM_STACK[self.depth - 1];
        self.emit_truthy_to_gpr(cond, Gpr::R10);
        self.encoder.movq_r64_xmm(Gpr::R8, else_value);
        self.encoder.movq_r64_xmm(Gpr::R11, then_value);
        self.encoder.test_r8_r8(Gpr::R10, Gpr::R10);
        self.encoder.cmovne_r64_r64(Gpr::R8, Gpr::R11);
        self.encoder.movq_xmm_r64(cond, Gpr::R8);
        self.depth -= 2;
        Ok(())
    }

    fn emit_extremum(&mut self, op: ExtremumOp) -> JitResult<()> {
        if self.depth < 2 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: format!("min/max requires stack depth 2, found {}", self.depth).into(),
            });
        }

        let left = XMM_STACK[self.depth - 2];
        let right = XMM_STACK[self.depth - 1];
        self.emit_extremum_left_prelude(left);
        self.emit_extremum_register_op(left, right, op);
        self.emit_extremum_select_left_fixup(left, right);
        self.depth -= 1;
        Ok(())
    }

    fn emit_extremum_const(&mut self, op: ExtremumOp, value: f64) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "literal RHS min/max requires stack depth 1, found 0".into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        self.emit_extremum_left_prelude(target);
        self.emit_literal_extremum_op(target, value, op);
        self.emit_extremum_select_left_fixup_from_result(target);
        Ok(())
    }

    fn emit_extremum_const_lhs(&mut self, op: ExtremumOp, value: f64) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "literal LHS min/max requires stack depth 1, found 0".into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        if self.depth < XMM_STACK.len() {
            let left = self.scratch_register()?;
            self.emit_literal_load(left, value);
            self.emit_extremum_left_prelude(left);
            self.emit_extremum_register_op(left, target, op);
            self.emit_extremum_select_left_fixup(left, target);
            self.encoder.movsd_xmm_xmm(target, left);
        } else {
            self.encoder.sub_rsp_imm32(ROUND_TEMP_FRAME_BYTES);
            self.encoder.movsd_m64_base_disp32_xmm(Gpr::Rsp, 0, target);
            self.emit_literal_load(target, value);
            self.emit_extremum_left_prelude(target);
            self.emit_extremum_memory_op(target, Gpr::Rsp, 0, op);
            self.emit_extremum_select_left_fixup_from_result(target);
            self.encoder.add_rsp_imm32(ROUND_TEMP_FRAME_BYTES);
        }

        Ok(())
    }

    fn emit_extremum_left_prelude(&mut self, left: Xmm) {
        self.encoder.movq_r64_xmm(Gpr::R8, left);
        self.encoder.ucomisd_xmm_xmm(left, left);
        self.encoder.setcc_r8(ConditionCode::NotParity, Gpr::R10);
        self.emit_abs_zero_from_bits_to_gpr(Gpr::R8, Gpr::R9);
    }

    fn emit_extremum_register_op(&mut self, left: Xmm, right: Xmm, op: ExtremumOp) {
        match op {
            ExtremumOp::Min => self.encoder.minsd_xmm_xmm(left, right),
            ExtremumOp::Max => self.encoder.maxsd_xmm_xmm(left, right),
        }
    }

    fn emit_extremum_memory_op(&mut self, left: Xmm, base: Gpr, disp: i32, op: ExtremumOp) {
        match op {
            ExtremumOp::Min => self.encoder.minsd_xmm_m64_base_disp32(left, base, disp),
            ExtremumOp::Max => self.encoder.maxsd_xmm_m64_base_disp32(left, base, disp),
        }
    }

    fn emit_extremum_select_left_fixup(&mut self, result: Xmm, right: Xmm) {
        self.encoder.ucomisd_xmm_xmm(right, right);
        self.encoder.setcc_r8(ConditionCode::Parity, Gpr::R11);
        self.emit_extremum_select_left_fixup_after_right_check(result);
    }

    fn emit_extremum_select_left_fixup_from_result(&mut self, result: Xmm) {
        self.encoder.ucomisd_xmm_xmm(result, result);
        self.encoder.setcc_r8(ConditionCode::Parity, Gpr::R11);
        self.emit_extremum_select_left_fixup_after_right_check(result);
    }

    fn emit_extremum_select_left_fixup_after_right_check(&mut self, result: Xmm) {
        self.encoder.and_r8_r8(Gpr::R10, Gpr::R11);
        self.emit_abs_zero_to_gpr(result, Gpr::R11);
        self.encoder.and_r8_r8(Gpr::R9, Gpr::R11);
        self.encoder.or_r8_r8(Gpr::R10, Gpr::R9);
        self.encoder.movq_r64_xmm(Gpr::R11, result);
        self.encoder.test_r8_r8(Gpr::R10, Gpr::R10);
        self.encoder.cmovne_r64_r64(Gpr::R11, Gpr::R8);
        self.encoder.movq_xmm_r64(result, Gpr::R11);
    }

    fn emit_abs_zero_to_gpr(&mut self, value: Xmm, dst: Gpr) {
        self.encoder.movq_r64_xmm(dst, value);
        self.emit_abs_zero_bits_in_place(dst);
    }

    fn emit_abs_zero_from_bits_to_gpr(&mut self, bits: Gpr, dst: Gpr) {
        debug_assert_ne!(bits, dst);
        self.encoder.mov_r64_r64(dst, bits);
        self.emit_abs_zero_bits_in_place(dst);
    }

    fn emit_abs_zero_bits_in_place(&mut self, dst: Gpr) {
        self.encoder.btr_r64_imm8(dst, 63);
        self.encoder.test_r64_r64(dst, dst);
        self.encoder.setcc_r8(ConditionCode::Equal, dst);
    }

    fn emit_voltage_load(
        &mut self,
        pos: VoltageNode,
        neg: VoltageNode,
        context_pointer_cache: &mut Option<i32>,
    ) -> JitResult<()> {
        let dst = self.push_register()?;

        match (pos, neg) {
            (VoltageNode::Ground, VoltageNode::Ground) => {
                self.encoder.xorpd_xmm_xmm(dst, dst);
            }
            (pos, VoltageNode::Ground) => {
                self.emit_node_voltage_load(dst, pos, context_pointer_cache)?;
            }
            (VoltageNode::Ground, neg) => {
                self.encoder.xorpd_xmm_xmm(dst, dst);
                self.emit_node_voltage_subtract(dst, neg, context_pointer_cache)?;
            }
            (VoltageNode::Terminal(pos), VoltageNode::Terminal(neg)) => {
                self.emit_same_storage_voltage_difference(
                    dst,
                    VOLTAGES_OFFSET,
                    pos,
                    neg,
                    context_pointer_cache,
                )?;
            }
            (VoltageNode::Internal(pos), VoltageNode::Internal(neg)) => {
                self.emit_same_storage_voltage_difference(
                    dst,
                    INTERNAL_VOLTAGES_OFFSET,
                    pos,
                    neg,
                    context_pointer_cache,
                )?;
            }
            (pos, neg) => {
                self.emit_node_voltage_load(dst, pos, context_pointer_cache)?;
                self.emit_node_voltage_subtract(dst, neg, context_pointer_cache)?;
            }
        }

        Ok(())
    }

    fn emit_same_storage_voltage_difference(
        &mut self,
        dst: Xmm,
        ctx_field_offset: i32,
        pos_index: usize,
        neg_index: usize,
        context_pointer_cache: &mut Option<i32>,
    ) -> JitResult<()> {
        self.emit_context_pointer_load_cached(ctx_field_offset, context_pointer_cache);
        self.encoder
            .movsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(pos_index)?);
        if pos_index == neg_index {
            self.encoder.subsd_xmm_xmm(dst, dst);
        } else {
            self.encoder
                .subsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(neg_index)?);
        }
        Ok(())
    }

    fn emit_thermal_voltage_load(&mut self) -> JitResult<()> {
        let dst = self.push_register()?;
        self.encoder
            .movsd_xmm_m64_base_disp32(dst, self.ctx_arg_reg(), TEMPERATURE_OFFSET);
        self.emit_literal_binary_op(dst, THERMAL_VOLTAGE_PER_K, BinaryOp::Mul);
        Ok(())
    }

    fn emit_node_voltage_load(
        &mut self,
        dst: Xmm,
        node: VoltageNode,
        context_pointer_cache: &mut Option<i32>,
    ) -> JitResult<()> {
        match node {
            VoltageNode::Terminal(index) => {
                self.emit_terminal_voltage_load(dst, index, context_pointer_cache)
            }
            VoltageNode::Internal(index) => {
                self.emit_internal_voltage_load(dst, index, context_pointer_cache)
            }
            VoltageNode::Ground => {
                self.encoder.xorpd_xmm_xmm(dst, dst);
                Ok(())
            }
        }
    }

    fn emit_node_voltage_subtract(
        &mut self,
        dst: Xmm,
        node: VoltageNode,
        context_pointer_cache: &mut Option<i32>,
    ) -> JitResult<()> {
        match node {
            VoltageNode::Terminal(index) => {
                self.emit_context_pointer_load_cached(VOLTAGES_OFFSET, context_pointer_cache);
                self.encoder
                    .subsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(index)?);
                Ok(())
            }
            VoltageNode::Internal(index) => {
                self.emit_context_pointer_load_cached(
                    INTERNAL_VOLTAGES_OFFSET,
                    context_pointer_cache,
                );
                self.encoder
                    .subsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(index)?);
                Ok(())
            }
            VoltageNode::Ground => Ok(()),
        }
    }

    fn emit_terminal_voltage_load(
        &mut self,
        dst: Xmm,
        index: usize,
        context_pointer_cache: &mut Option<i32>,
    ) -> JitResult<()> {
        self.emit_context_pointer_load_cached(VOLTAGES_OFFSET, context_pointer_cache);
        self.encoder
            .movsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(index)?);
        Ok(())
    }

    fn emit_internal_voltage_load(
        &mut self,
        dst: Xmm,
        index: usize,
        context_pointer_cache: &mut Option<i32>,
    ) -> JitResult<()> {
        self.emit_context_pointer_load_cached(INTERNAL_VOLTAGES_OFFSET, context_pointer_cache);
        self.encoder
            .movsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(index)?);
        Ok(())
    }

    fn emit_context_pointer_load(&mut self, ctx_field_offset: i32) {
        self.encoder
            .mov_r64_m64_base_disp32(Gpr::Rax, self.ctx_arg_reg(), ctx_field_offset);
    }

    fn emit_context_pointer_load_cached(
        &mut self,
        ctx_field_offset: i32,
        context_pointer_cache: &mut Option<i32>,
    ) {
        if *context_pointer_cache != Some(ctx_field_offset) {
            self.emit_context_pointer_load(ctx_field_offset);
            *context_pointer_cache = Some(ctx_field_offset);
        }
    }

    fn emit_context_f64_load(&mut self, ctx_field_offset: i32) -> JitResult<()> {
        let dst = self.push_register()?;
        self.encoder
            .movsd_xmm_m64_base_disp32(dst, self.ctx_arg_reg(), ctx_field_offset);
        Ok(())
    }

    fn emit_param_given_load(
        &mut self,
        index: usize,
        context_pointer_cache: &mut Option<i32>,
    ) -> JitResult<()> {
        self.emit_guarded_context_u8_slice_load(
            PARAM_GIVEN_OFFSET,
            PARAM_GIVEN_LEN_OFFSET,
            index,
            rspice_native_param_given_error,
            context_pointer_cache,
        )
    }

    fn emit_port_connected_load(
        &mut self,
        index: usize,
        context_pointer_cache: &mut Option<i32>,
    ) -> JitResult<()> {
        self.emit_guarded_context_u8_slice_load(
            PORT_CONNECTED_OFFSET,
            PORT_CONNECTED_LEN_OFFSET,
            index,
            rspice_native_port_connected_error,
            context_pointer_cache,
        )
    }

    fn emit_guarded_context_u8_slice_load(
        &mut self,
        pointer_field_offset: i32,
        len_field_offset: i32,
        index: usize,
        helper: VoidHelper,
        context_pointer_cache: &mut Option<i32>,
    ) -> JitResult<()> {
        let dst = self.push_register()?;
        let value_disp = byte_disp_u8(index)?;

        self.emit_context_pointer_load_cached(pointer_field_offset, context_pointer_cache);
        self.encoder.test_r64_r64(Gpr::Rax, Gpr::Rax);
        let missing_storage = self.encoder.jcc_rel32_placeholder(ConditionCode::Equal);

        self.encoder
            .mov_r64_m64_base_disp32(Gpr::R10, self.ctx_arg_reg(), len_field_offset);
        self.encoder
            .cmp_r64_imm32(Gpr::R10, slice_index_imm32(index)?);
        let index_out_of_range = self
            .encoder
            .jcc_rel32_placeholder(ConditionCode::BelowOrEqual);

        self.encoder
            .movzx_r32_m8_base_disp32(Gpr::R10, Gpr::Rax, value_disp);
        self.encoder.cvtsi2sd_xmm_r32(dst, Gpr::R10);
        let done = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(missing_storage)?;
        self.patch_rel32_to_current(index_out_of_range)?;
        self.emit_void_error_return(helper);

        self.patch_rel32_to_current(done)
    }

    fn emit_analysis_check(&mut self, analysis_id: u8) -> JitResult<()> {
        let dst = self.push_register()?;
        if analysis_id > 6 {
            self.encoder.xorpd_xmm_xmm(dst, dst);
            return Ok(());
        }

        self.encoder
            .movzx_r32_m8_base_disp32(Gpr::R10, self.ctx_arg_reg(), ANALYSIS_TYPE_OFFSET);
        match analysis_id {
            5 => {
                self.encoder.cmp_r32_imm8(Gpr::R10, 0);
                self.encoder.setcc_r8(ConditionCode::Equal, Gpr::R11);
                self.encoder.cmp_r32_imm8(Gpr::R10, 4);
                self.encoder.setcc_r8(ConditionCode::Equal, Gpr::R10);
                self.encoder.or_r8_r8(Gpr::R10, Gpr::R11);
            }
            6 => {
                self.encoder.cmp_r32_imm8(Gpr::R10, 1);
                self.encoder.setcc_r8(ConditionCode::Equal, Gpr::R11);
                self.encoder.cmp_r32_imm8(Gpr::R10, 3);
                self.encoder.setcc_r8(ConditionCode::Equal, Gpr::R10);
                self.encoder.or_r8_r8(Gpr::R10, Gpr::R11);
            }
            _ => {
                self.encoder.cmp_r32_imm8(Gpr::R10, analysis_id);
                self.encoder.setcc_r8(ConditionCode::Equal, Gpr::R10);
            }
        }
        self.encoder.movzx_r32_r8(Gpr::R10, Gpr::R10);
        self.encoder.cvtsi2sd_xmm_r32(dst, Gpr::R10);
        Ok(())
    }

    fn emit_literal_load(&mut self, dst: Xmm, value: f64) {
        let displacement_offset = self.encoder.movsd_xmm_m64_rip_disp32(dst, 0);
        self.literals.push(LiteralPatch {
            displacement_offset,
            value,
        });
    }

    fn emit_constant_load(&mut self, dst: Xmm, value: f64) {
        if value.to_bits() == 0 {
            self.encoder.xorpd_xmm_xmm(dst, dst);
        } else {
            self.emit_literal_load(dst, value);
        }
    }

    fn emit_literal_binary_op(&mut self, dst: Xmm, value: f64, op: BinaryOp) {
        if matches!(op, BinaryOp::Mul) && value.to_bits() == 2.0_f64.to_bits() {
            self.encoder.addsd_xmm_xmm(dst, dst);
            return;
        }

        let displacement_offset = match op {
            BinaryOp::Add => self.encoder.addsd_xmm_m64_rip_disp32(dst, 0),
            BinaryOp::Sub => self.encoder.subsd_xmm_m64_rip_disp32(dst, 0),
            BinaryOp::Mul => self.encoder.mulsd_xmm_m64_rip_disp32(dst, 0),
            BinaryOp::Div => self.encoder.divsd_xmm_m64_rip_disp32(dst, 0),
        };
        self.literals.push(LiteralPatch {
            displacement_offset,
            value,
        });
    }

    fn emit_literal_compare(&mut self, left: Xmm, value: f64) {
        let displacement_offset = self.encoder.ucomisd_xmm_m64_rip_disp32(left, 0);
        self.literals.push(LiteralPatch {
            displacement_offset,
            value,
        });
    }

    fn emit_literal_extremum_op(&mut self, dst: Xmm, value: f64, op: ExtremumOp) {
        let displacement_offset = match op {
            ExtremumOp::Min => self.encoder.minsd_xmm_m64_rip_disp32(dst, 0),
            ExtremumOp::Max => self.encoder.maxsd_xmm_m64_rip_disp32(dst, 0),
        };
        self.literals.push(LiteralPatch {
            displacement_offset,
            value,
        });
    }

    fn emit_usize_arg(&mut self, dst: Gpr, value: usize) {
        if value == 0 {
            self.encoder.xor_r64_r64(dst, dst);
        } else if let Ok(value) = u32::try_from(value) {
            self.encoder.mov_r32_imm32(dst, value);
        } else {
            self.encoder.movabs_r64_imm64(dst, value as u64);
        }
    }

    fn emit_i64_arg(&mut self, dst: Gpr, value: i64) {
        if value == 0 {
            self.encoder.xor_r64_r64(dst, dst);
        } else if let Ok(value) = u32::try_from(value) {
            self.encoder.mov_r32_imm32(dst, value);
        } else if let Ok(value) = i32::try_from(value) {
            self.encoder.mov_r64_imm32(dst, value);
        } else {
            self.encoder.movabs_r64_imm64(dst, value as u64);
        }
    }

    fn emit_usize_compare(&mut self, left: Gpr, value: usize) {
        if let Ok(value) = i32::try_from(value) {
            self.encoder.cmp_r64_imm32(left, value);
        } else {
            self.encoder.movabs_r64_imm64(Gpr::R11, value as u64);
            self.encoder.cmp_r64_r64(left, Gpr::R11);
        }
    }

    fn emit_i64_subtract(&mut self, target: Gpr, value: i64) {
        if value == 0 {
            return;
        }
        if let Ok(value) = i32::try_from(value) {
            self.encoder.sub_r64_imm32(target, value);
        } else {
            self.encoder.movabs_r64_imm64(Gpr::R11, value as u64);
            self.encoder.sub_r64_r64(target, Gpr::R11);
        }
    }

    fn finish_with_literals(self) -> JitResult<Vec<u8>> {
        let mut bytes = self.encoder.into_bytes();
        let mut literal_offsets: Vec<(u64, usize)> = Vec::new();
        if !self.literals.is_empty() {
            let padding = (LITERAL_POOL_ALIGNMENT - (bytes.len() % LITERAL_POOL_ALIGNMENT))
                % LITERAL_POOL_ALIGNMENT;
            bytes.resize(bytes.len() + padding, LITERAL_POOL_PADDING_BYTE);
        }
        for literal in &self.literals {
            let bits = literal.value.to_bits();
            let literal_offset = literal_offsets
                .iter()
                .find_map(|(candidate, offset)| (*candidate == bits).then_some(*offset))
                .unwrap_or_else(|| {
                    let offset = bytes.len();
                    bytes.extend_from_slice(&bits.to_le_bytes());
                    literal_offsets.push((bits, offset));
                    offset
                });
            let next_instruction_offset = literal.displacement_offset + std::mem::size_of::<i32>();
            let displacement = i32::try_from(
                literal_offset as isize - next_instruction_offset as isize,
            )
            .map_err(|_| JitError::Relocation {
                model: MODEL.into(),
                detail: "literal pool displacement does not fit in i32".into(),
            })?;

            bytes[literal.displacement_offset..literal.displacement_offset + 4]
                .copy_from_slice(&displacement.to_le_bytes());
        }

        let mut vector_literal_offsets: Vec<([u64; 2], usize)> = Vec::new();
        if !self.vector_literals.is_empty() {
            let padding = (VECTOR_LITERAL_ALIGNMENT - (bytes.len() % VECTOR_LITERAL_ALIGNMENT))
                % VECTOR_LITERAL_ALIGNMENT;
            bytes.resize(bytes.len() + padding, LITERAL_POOL_PADDING_BYTE);
        }
        for literal in &self.vector_literals {
            let literal_offset = vector_literal_offsets
                .iter()
                .find_map(|(candidate, offset)| (*candidate == literal.lanes).then_some(*offset))
                .unwrap_or_else(|| {
                    let offset = bytes.len();
                    bytes.extend_from_slice(&literal.lanes[0].to_le_bytes());
                    bytes.extend_from_slice(&literal.lanes[1].to_le_bytes());
                    vector_literal_offsets.push((literal.lanes, offset));
                    offset
                });
            let next_instruction_offset = literal.displacement_offset + std::mem::size_of::<i32>();
            let displacement = i32::try_from(
                literal_offset as isize - next_instruction_offset as isize,
            )
            .map_err(|_| JitError::Relocation {
                model: MODEL.into(),
                detail: "vector literal pool displacement does not fit in i32".into(),
            })?;

            bytes[literal.displacement_offset..literal.displacement_offset + 4]
                .copy_from_slice(&displacement.to_le_bytes());
        }

        Ok(bytes)
    }

    fn emit_jmp_to_offset(&mut self, target_offset: usize) -> JitResult<()> {
        let displacement_offset = self.encoder.jmp_rel32_placeholder();
        self.patch_rel32_to_offset(displacement_offset, target_offset)
    }

    fn patch_rel32_to_current(&mut self, displacement_offset: usize) -> JitResult<()> {
        self.patch_rel32_to_offset(displacement_offset, self.encoder.position())
    }

    fn patch_rel32_to_offset(
        &mut self,
        displacement_offset: usize,
        target_offset: usize,
    ) -> JitResult<()> {
        let next_instruction_offset = displacement_offset + std::mem::size_of::<i32>();
        let displacement = i32::try_from(target_offset as isize - next_instruction_offset as isize)
            .map_err(|_| JitError::Relocation {
                model: MODEL.into(),
                detail: "branch displacement does not fit in i32".into(),
            })?;
        self.encoder.patch_i32(displacement_offset, displacement);
        Ok(())
    }

    fn patch_early_returns_to_current(&mut self) -> JitResult<()> {
        let jumps = std::mem::take(&mut self.early_return_jumps);
        for displacement_offset in jumps {
            self.patch_rel32_to_current(displacement_offset)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy)]
enum RoundDirection {
    Floor,
    Ceil,
}

type UnaryHelper = extern "C" fn(f64) -> f64;
type BinaryHelper = extern "C" fn(f64, f64) -> f64;
type TernaryHelper = extern "C" fn(f64, f64, f64) -> f64;
type VoidHelper = extern "C" fn();
type TableHelper =
    unsafe extern "C" fn(f64, *const crate::codegen::LookupTable, usize, usize) -> f64;
type ContextFilterHelper =
    unsafe extern "C" fn(f64, *const crate::native::EvalContext, usize) -> f64;
type TimerHelper = unsafe extern "C" fn(f64, f64, *const crate::native::EvalContext) -> f64;
type OperandContextFilterHelper =
    unsafe extern "C" fn(*const f64, *const crate::native::EvalContext, usize) -> f64;
type DynamicVariableHelper = unsafe extern "C" fn(f64, *const f64, usize, i64) -> f64;
type DynamicVariableSlotHelper = unsafe extern "C" fn(f64, *mut f64, usize, i64) -> *mut f64;

fn assignment_uses_helper_calls(assignment: &NativeAssignment) -> bool {
    match assignment {
        NativeAssignment::Direct { program, .. } => program_uses_helper_calls(program),
        NativeAssignment::Indexed {
            len,
            lower,
            index,
            value,
            ..
        } => {
            !dynamic_variable_inline_supported(*len, *lower)
                || program_uses_helper_calls(index)
                || program_uses_helper_calls(value)
        }
        NativeAssignment::Loop { condition, body } => {
            program_uses_helper_calls(condition) || body.iter().any(assignment_uses_helper_calls)
        }
    }
}

fn assignment_needs_stateful_stack_scratch(assignment: &NativeAssignment) -> bool {
    match assignment {
        NativeAssignment::Direct { program, .. } => program_needs_stateful_stack_scratch(program),
        NativeAssignment::Indexed { index, value, .. } => {
            program_needs_stateful_stack_scratch(index)
                || program_needs_stateful_stack_scratch(value)
        }
        NativeAssignment::Loop { condition, body } => {
            program_needs_stateful_stack_scratch(condition)
                || body.iter().any(assignment_needs_stateful_stack_scratch)
        }
    }
}

fn assignment_max_stack_depth(assignments: &[NativeAssignment]) -> usize {
    assignments
        .iter()
        .map(|assignment| match assignment {
            NativeAssignment::Direct { program, .. } => program.max_stack_depth(),
            NativeAssignment::Indexed { index, value, .. } => {
                index.max_stack_depth().max(value.max_stack_depth())
            }
            NativeAssignment::Loop { condition, body } => condition
                .max_stack_depth()
                .max(assignment_max_stack_depth(body)),
        })
        .max()
        .unwrap_or(0)
}

fn assignment_has_indexed(assignment: &NativeAssignment) -> bool {
    match assignment {
        NativeAssignment::Direct { .. } => false,
        NativeAssignment::Indexed { .. } => true,
        NativeAssignment::Loop { body, .. } => body.iter().any(assignment_has_indexed),
    }
}

fn assignment_loop_depth(assignments: &[NativeAssignment]) -> i32 {
    assignments
        .iter()
        .map(|assignment| match assignment {
            NativeAssignment::Direct { .. } | NativeAssignment::Indexed { .. } => 0,
            NativeAssignment::Loop { body, .. } => 1 + assignment_loop_depth(body),
        })
        .max()
        .unwrap_or(0)
}

fn align_local_frame(bytes: i32) -> i32 {
    if bytes == 0 {
        0
    } else {
        ((bytes + LOCAL_FRAME_ALIGN_BYTES - 1) / LOCAL_FRAME_ALIGN_BYTES) * LOCAL_FRAME_ALIGN_BYTES
    }
}

fn callee_saved_xmm_count_for_depth(max_stack_depth: usize) -> usize {
    max_stack_depth
        .min(XMM_STACK.len())
        .saturating_sub(CALLER_SAVED_XMM_COUNT)
}

fn callee_saved_xmm_frame_bytes(count: usize) -> i32 {
    count as i32 * CALLEE_SAVED_XMM_BYTES
}

fn callee_saved_xmm_register(index: usize) -> Xmm {
    XMM_STACK[CALLER_SAVED_XMM_COUNT + index]
}

fn unary_math_helper(op: UnaryMathOp) -> UnaryHelper {
    match op {
        UnaryMathOp::Exp => rspice_exp,
        UnaryMathOp::Log => rspice_log,
        UnaryMathOp::Log10 => rspice_log10,
        UnaryMathOp::Sin => rspice_sin,
        UnaryMathOp::Cos => rspice_cos,
        UnaryMathOp::Tan => rspice_tan,
        UnaryMathOp::Sinh => rspice_sinh,
        UnaryMathOp::Cosh => rspice_cosh,
        UnaryMathOp::Tanh => rspice_tanh,
        UnaryMathOp::Asinh => rspice_asinh,
        UnaryMathOp::Acosh => rspice_acosh,
        UnaryMathOp::Atanh => rspice_atanh,
        UnaryMathOp::Limexp => rspice_limexp,
        UnaryMathOp::LimitedExp => rspice_limited_exp,
        UnaryMathOp::Asin => rspice_asin,
        UnaryMathOp::Acos => rspice_acos,
        UnaryMathOp::Atan => rspice_atan,
        UnaryMathOp::Floor => rspice_floor,
        UnaryMathOp::Ceil => rspice_ceil,
    }
}

fn binary_math_helper(op: BinaryMathOp) -> BinaryHelper {
    match op {
        BinaryMathOp::Pow => rspice_pow,
        BinaryMathOp::Atan2 => rspice_atan2,
        BinaryMathOp::Hypot => rspice_hypot,
        BinaryMathOp::Mod => rspice_mod,
    }
}

fn call_spill_disp(index: usize) -> i32 {
    CALL_SHADOW_BYTES + (index * WORD_BYTES) as i32
}

fn call_frame_bytes_for_slots(slot_count: usize) -> i32 {
    let spill_bytes = (slot_count * WORD_BYTES) as i32;
    let mut frame_bytes = CALL_SHADOW_BYTES + spill_bytes;
    while frame_bytes % LOCAL_FRAME_ALIGN_BYTES != LOCAL_SLOT_BYTES {
        frame_bytes += LOCAL_SLOT_BYTES;
    }
    frame_bytes
}

fn call_frame_spill_slot_count(
    spill_depth: usize,
    mut should_spill: impl FnMut(usize, Xmm) -> bool,
) -> usize {
    XMM_STACK
        .iter()
        .copied()
        .take(spill_depth)
        .enumerate()
        .filter_map(|(index, register)| should_spill(index, register).then_some(index + 1))
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
fn call_result_disp() -> i32 {
    call_spill_disp(CALL_RESULT_SLOT)
}

fn program_uses_helper_calls(program: &NativeProgram) -> bool {
    program.ops().iter().any(native_op_uses_helper_call)
}

fn value_program_needs_saved_entry_args(program: &NativeProgram) -> bool {
    let mut helper_seen = false;
    for op in program.ops() {
        if native_op_needs_saved_entry_args_for_internal_helper_continuation(*op) {
            return true;
        }
        if helper_seen && native_op_reads_entry_args(*op) {
            return true;
        }
        helper_seen |= native_op_uses_helper_call(op);
    }
    false
}

fn native_op_needs_saved_entry_args_for_internal_helper_continuation(op: NativeOp) -> bool {
    matches!(op, NativeOp::IdtModState(_))
}

fn native_op_uses_helper_call(op: &NativeOp) -> bool {
    matches!(
        op,
        NativeOp::BinaryMath(_)
            | NativeOp::TableLookup(_)
            | NativeOp::TableDerivative(_)
            | NativeOp::LaplaceState(_)
            | NativeOp::ZiState(_)
            | NativeOp::TimerState(_)
            | NativeOp::TransitionState(_)
            | NativeOp::SlewState(_)
            | NativeOp::AbsDelayState(_)
            | NativeOp::CrossState(_)
            | NativeOp::IdtModState(_)
    ) || matches!(op, NativeOp::UnaryMath(op) if unary_math_uses_helper(*op))
        || matches!(
            op,
            NativeOp::LoadVariableDyn { len, lower, .. }
                if !dynamic_variable_inline_supported(*len, *lower)
        )
}

fn native_op_reads_entry_args(op: NativeOp) -> bool {
    !matches!(
        op,
        NativeOp::Const(_)
            | NativeOp::Add
            | NativeOp::Sub
            | NativeOp::Mul
            | NativeOp::Div
            | NativeOp::AddConst(_)
            | NativeOp::SubConst(_)
            | NativeOp::MulConst(_)
            | NativeOp::DivConst(_)
            | NativeOp::SubFromConst(_)
            | NativeOp::DivFromConst(_)
            | NativeOp::Neg
            | NativeOp::Abs
            | NativeOp::Square
            | NativeOp::Sqrt
            | NativeOp::Compare(_)
            | NativeOp::CompareConst(_, _)
            | NativeOp::Logical(_)
            | NativeOp::LogicalConst(_, _)
            | NativeOp::IfElse
            | NativeOp::Extremum(_)
            | NativeOp::ExtremumConst(_, _)
            | NativeOp::ExtremumConstLhs(_, _)
            | NativeOp::UnaryMath(_)
            | NativeOp::BinaryMath(_)
            | NativeOp::IntegerCast
            | NativeOp::IntegerBinary(_)
            | NativeOp::IntegerShiftConst(_, _)
            | NativeOp::IntegerBinaryConst(_, _)
            | NativeOp::WhiteNoise
            | NativeOp::FlickerNoise
    )
}

fn native_op_preserves_context_pointer_cache(op: NativeOp) -> bool {
    match op {
        NativeOp::LoadVariableDyn { len, lower, .. } => {
            dynamic_variable_inline_supported(len, lower)
        }
        _ => matches!(
            op,
            NativeOp::Const(_)
                | NativeOp::LoadParam(_)
                | NativeOp::LoadParamGiven(_)
                | NativeOp::LoadPortConnected(_)
                | NativeOp::LoadVoltage { .. }
                | NativeOp::LoadTemperature
                | NativeOp::LoadThermalVoltage
                | NativeOp::LoadTime
                | NativeOp::Analysis(_)
                | NativeOp::LoadMfactor
                | NativeOp::LoadCurrent(_)
                | NativeOp::LoadPriorCurrent(_)
                | NativeOp::LoadInternalVoltage(_)
                | NativeOp::LoadBranchUnknown(_)
                | NativeOp::LoadVariable(_)
                | NativeOp::Add
                | NativeOp::Sub
                | NativeOp::Mul
                | NativeOp::Div
                | NativeOp::AddConst(_)
                | NativeOp::SubConst(_)
                | NativeOp::MulConst(_)
                | NativeOp::DivConst(_)
                | NativeOp::SubFromConst(_)
                | NativeOp::DivFromConst(_)
                | NativeOp::Neg
                | NativeOp::Abs
                | NativeOp::Square
                | NativeOp::Sqrt
                | NativeOp::Compare(_)
                | NativeOp::CompareConst(_, _)
                | NativeOp::Logical(_)
                | NativeOp::LogicalConst(_, _)
                | NativeOp::IfElse
                | NativeOp::Extremum(_)
                | NativeOp::ExtremumConst(_, _)
                | NativeOp::ExtremumConstLhs(_, _)
                | NativeOp::UnaryMath(UnaryMathOp::Floor | UnaryMathOp::Ceil)
                | NativeOp::IntegerCast
                | NativeOp::IntegerBinary(_)
                | NativeOp::IntegerShiftConst(_, _)
                | NativeOp::IntegerBinaryConst(_, _)
                | NativeOp::WhiteNoise
                | NativeOp::FlickerNoise
        ),
    }
}

fn program_needs_stateful_stack_scratch(program: &NativeProgram) -> bool {
    let mut depth = 0_usize;
    for op in program.ops().iter().copied() {
        if native_op_uses_stateful_scratch_at_depth(op, depth) {
            return true;
        }
        depth = native_stack_depth_after(depth, op);
    }
    false
}

fn native_op_uses_stateful_scratch_at_depth(op: NativeOp, depth: usize) -> bool {
    depth >= XMM_STACK.len()
        && matches!(
            op,
            NativeOp::DdtState(_)
                | NativeOp::DdtJacobian
                | NativeOp::IdtState(_)
                | NativeOp::IdtJacobian
                | NativeOp::IdtModState(_)
        )
}

fn native_stack_depth_after(depth: usize, op: NativeOp) -> usize {
    let (pops, pushes) = native_op_stack_effect(&op);
    debug_assert!(
        depth >= pops,
        "native op {op:?} requires stack depth {pops}, found {depth}"
    );
    depth.saturating_sub(pops) + pushes
}

fn unary_math_uses_helper(op: UnaryMathOp) -> bool {
    !matches!(op, UnaryMathOp::Floor | UnaryMathOp::Ceil)
}

fn dynamic_variable_inline_supported(len: usize, lower: i64) -> bool {
    let Ok(len_i64) = i64::try_from(len) else {
        return false;
    };
    let Some(upper) = lower
        .checked_add(len_i64)
        .and_then(|exclusive| exclusive.checked_sub(1))
    else {
        return false;
    };
    let supported = -INLINE_DYNAMIC_LOWER_ABS_LIMIT..=INLINE_DYNAMIC_LOWER_ABS_LIMIT;
    supported.contains(&lower) && supported.contains(&upper)
}

fn xmm_stack_slot(register: Xmm) -> usize {
    XMM_STACK
        .iter()
        .position(|candidate| *candidate == register)
        .expect("register belongs to native XMM stack")
}

fn byte_disp(index: usize) -> JitResult<i32> {
    let byte_offset = index
        .checked_mul(WORD_BYTES)
        .ok_or_else(|| JitError::Encoding {
            model: MODEL.into(),
            detail: format!("index {index} byte offset overflow").into(),
        })?;

    i32::try_from(byte_offset).map_err(|_| JitError::Encoding {
        model: MODEL.into(),
        detail: format!("index {index} byte offset exceeds x64 disp32 range").into(),
    })
}

fn byte_disp_u8(index: usize) -> JitResult<i32> {
    i32::try_from(index).map_err(|_| JitError::Encoding {
        model: MODEL.into(),
        detail: format!("u8 flag index {index} exceeds x64 disp32 range").into(),
    })
}

fn slice_index_imm32(index: usize) -> JitResult<i32> {
    i32::try_from(index).map_err(|_| JitError::Encoding {
        model: MODEL.into(),
        detail: format!("slice index {index} exceeds x64 imm32 range").into(),
    })
}

fn state_index_imm32(index: usize) -> JitResult<i32> {
    i32::try_from(index).map_err(|_| JitError::Encoding {
        model: MODEL.into(),
        detail: format!("state index {index} exceeds x64 imm32 range").into(),
    })
}

fn register_allocation_error(detail: String) -> JitError {
    JitError::RegisterAllocation {
        model: MODEL.into(),
        detail: detail.into(),
    }
}

fn saved_ctx_arg_reg() -> Gpr {
    Gpr::R12
}

fn saved_vars_arg_reg() -> Gpr {
    Gpr::R13
}

#[cfg(windows)]
fn entry_ctx_arg_reg() -> Gpr {
    Gpr::Rcx
}

#[cfg(windows)]
fn entry_vars_arg_reg() -> Gpr {
    Gpr::Rdx
}

#[cfg(not(windows))]
fn entry_ctx_arg_reg() -> Gpr {
    Gpr::Rdi
}

#[cfg(not(windows))]
fn entry_vars_arg_reg() -> Gpr {
    Gpr::Rsi
}

#[cfg(windows)]
fn table_ptr_arg_reg() -> Gpr {
    Gpr::Rdx
}

#[cfg(windows)]
fn table_len_arg_reg() -> Gpr {
    Gpr::R8
}

#[cfg(windows)]
fn table_id_arg_reg() -> Gpr {
    Gpr::R9
}

#[cfg(not(windows))]
fn table_ptr_arg_reg() -> Gpr {
    Gpr::Rdi
}

#[cfg(not(windows))]
fn table_len_arg_reg() -> Gpr {
    Gpr::Rsi
}

#[cfg(not(windows))]
fn table_id_arg_reg() -> Gpr {
    Gpr::Rdx
}

#[cfg(windows)]
fn context_filter_ctx_arg_reg() -> Gpr {
    Gpr::Rdx
}

#[cfg(windows)]
fn context_filter_id_arg_reg() -> Gpr {
    Gpr::R8
}

#[cfg(not(windows))]
fn context_filter_ctx_arg_reg() -> Gpr {
    Gpr::Rdi
}

#[cfg(not(windows))]
fn context_filter_id_arg_reg() -> Gpr {
    Gpr::Rsi
}

#[cfg(windows)]
fn timer_ctx_arg_reg() -> Gpr {
    Gpr::R8
}

#[cfg(not(windows))]
fn timer_ctx_arg_reg() -> Gpr {
    Gpr::Rdi
}

#[cfg(windows)]
fn operand_filter_operands_arg_reg() -> Gpr {
    Gpr::Rcx
}

#[cfg(windows)]
fn operand_filter_ctx_arg_reg() -> Gpr {
    Gpr::Rdx
}

#[cfg(windows)]
fn operand_filter_id_arg_reg() -> Gpr {
    Gpr::R8
}

#[cfg(not(windows))]
fn operand_filter_operands_arg_reg() -> Gpr {
    Gpr::Rdi
}

#[cfg(not(windows))]
fn operand_filter_ctx_arg_reg() -> Gpr {
    Gpr::Rsi
}

#[cfg(not(windows))]
fn operand_filter_id_arg_reg() -> Gpr {
    Gpr::Rdx
}

#[cfg(windows)]
fn dynamic_variable_base_arg_reg() -> Gpr {
    Gpr::Rdx
}

#[cfg(windows)]
fn dynamic_variable_len_arg_reg() -> Gpr {
    Gpr::R8
}

#[cfg(windows)]
fn dynamic_variable_lower_arg_reg() -> Gpr {
    Gpr::R9
}

#[cfg(not(windows))]
fn dynamic_variable_base_arg_reg() -> Gpr {
    Gpr::Rdi
}

#[cfg(not(windows))]
fn dynamic_variable_len_arg_reg() -> Gpr {
    Gpr::Rsi
}

#[cfg(not(windows))]
fn dynamic_variable_lower_arg_reg() -> Gpr {
    Gpr::Rdx
}

#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
mod tests {
    use super::{
        ABS_VALUE_MASK_HIGH, ABS_VALUE_MASK_LOW, BOOLEAN_EPSILON, BRANCH_CURRENTS_OFFSET,
        ConditionCode, ContextFilterHelper, DYNAMIC_READ_FRAME_BYTES, FunctionCompiler, Gpr,
        I64_MAX_EXCLUSIVE_AS_F64, I64_MIN_AS_F64, INTERNAL_VOLTAGES_OFFSET, K_BOLTZMANN,
        LITERAL_POOL_ALIGNMENT, NativeAssignment, OperandContextFilterHelper, PARAMS_OFFSET,
        Q_ELECTRON, ROUND_TEMP_FRAME_BYTES, STATEFUL_SCRATCH_FRAME_BYTES, THERMAL_VOLTAGE_PER_K,
        TableHelper, TimerHelper, VECTOR_LITERAL_ALIGNMENT, VOLTAGES_OFFSET, WORD_BYTES,
        X64Encoder, XMM_STACK, Xmm, assignment_uses_helper_calls, call_result_disp,
        compile_assignment_function, compile_assignment_pass_function, compile_value_function,
        entry_ctx_arg_reg, entry_vars_arg_reg, native_op_reads_entry_args,
        native_op_uses_helper_call, program_uses_helper_calls, rspice_exp,
        value_program_needs_saved_entry_args,
    };
    use crate::codegen::{BytecodeProgram, Instruction, LookupTable};
    use crate::laplace::StateSpaceFilter;
    use crate::native::expr::{
        BinaryMathOp, CompareOp, EntryKind, ExtremumOp, IntegerBinaryOp, LogicalOp,
        NativeLoweringLimits, NativeOp, NativeProgram, UnaryMathOp,
    };
    use crate::native::runtime::ExecutableMemory;
    use crate::native::{
        EvalContext, clear_native_runtime_error, rspice_limexp, rspice_limited_exp,
        take_native_runtime_error,
    };
    use crate::vm::{CrossDetector, DelayBuffer, SlewFilter, TransitionFilter};
    use crate::zfilter::ZiFilter;

    #[test]
    fn literal_pool_reuses_identical_bit_patterns() {
        let mut compiler = FunctionCompiler::new(false, false, 0, None, None, 0);
        compiler.encoder.ret();
        compiler.emit_literal_load(Xmm::Xmm0, 2.0);
        compiler.emit_literal_load(Xmm::Xmm1, 2.0);
        compiler.emit_literal_load(Xmm::Xmm2, 3.0);

        let bytes = compiler
            .finish_with_literals()
            .expect("literal pool relocation succeeds");

        assert_eq!(
            count_bytes(&bytes, &2.0_f64.to_le_bytes()),
            1,
            "duplicate exact literals should share one pool slot"
        );
        assert_eq!(
            count_bytes(&bytes, &3.0_f64.to_le_bytes()),
            1,
            "distinct literals should keep distinct pool slots"
        );
        let two_offset =
            find_bytes(&bytes, &2.0_f64.to_le_bytes()).expect("2.0 literal is present");
        let three_offset =
            find_bytes(&bytes, &3.0_f64.to_le_bytes()).expect("3.0 literal is present");
        assert_eq!(
            two_offset % LITERAL_POOL_ALIGNMENT,
            0,
            "first literal should be naturally aligned"
        );
        assert_eq!(
            three_offset % LITERAL_POOL_ALIGNMENT,
            0,
            "subsequent literal should stay naturally aligned"
        );
    }

    #[test]
    fn vector_literal_pool_reuses_abs_masks_and_aligns_to_16_bytes() {
        let mut compiler = FunctionCompiler::new(false, false, 0, None, None, 0);
        compiler.encoder.ret();
        compiler.emit_abs_register(Xmm::Xmm0);
        compiler.emit_abs_register(Xmm::Xmm1);

        let bytes = compiler
            .finish_with_literals()
            .expect("vector literal pool relocation succeeds");
        let mask = abs_value_mask_bytes();
        let mask_offset = find_bytes(&bytes, &mask).expect("abs mask literal is present");

        assert_eq!(
            count_bytes(&bytes, &mask),
            1,
            "duplicate vector masks should share one pool slot"
        );
        assert_eq!(
            mask_offset % VECTOR_LITERAL_ALIGNMENT,
            0,
            "vector literal should be naturally aligned"
        );
    }

    #[test]
    fn generated_value_leaf_evaluates_native_expression() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushParam(0),
                Instruction::PushVoltage(0, 1),
                Instruction::Mul,
                Instruction::PushVariable(1),
                Instruction::PushConst(4.0),
                Instruction::Div,
                Instruction::Add,
            ],
            2,
        );
        let bytes = compile_value_function(&program).expect("compile value function");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate value leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let params = [2.0_f64];
        let voltages = [5.0_f64, 1.0_f64];
        let vars = [0.0_f64, 8.0_f64];
        let ctx = eval_context(&params, &voltages, &[], &[]);

        assert_eq!(f(&ctx, vars.as_ptr()), 10.0);
    }

    #[test]
    fn generated_value_leaf_uses_compact_base_displacements_for_param_load() {
        let program = native_program(EntryKind::StampValue, vec![Instruction::PushParam(0)], 0);

        let bytes = compile_value_function(&program).expect("compile param value function");

        assert!(
            contains_bytes(&bytes, &context_pointer_load_bytes(PARAMS_OFFSET)),
            "param base pointer should use the compact context field load"
        );
        assert!(
            contains_bytes(&bytes, &[0xF2, 0x0F, 0x10, 0x00]),
            "param value load at index 0 should use the no-displacement memory form"
        );
        assert!(
            !contains_bytes(
                &bytes,
                &old_disp32_context_pointer_load_bytes(PARAMS_OFFSET)
            ),
            "param base pointer should not use the old forced disp32 context field load"
        );
        assert!(
            !contains_bytes(&bytes, &[0xF2, 0x0F, 0x10, 0x80, 0, 0, 0, 0]),
            "param value load at index 0 should not use the old forced disp32 memory form"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate param value leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let params = [7.25_f64];
        let ctx = eval_context(&params, &[], &[], &[]);

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 7.25_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_reuses_param_base_for_adjacent_param_loads() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushParam(0),
                Instruction::PushParam(1),
                Instruction::Add,
            ],
            0,
        );

        let bytes = compile_value_function(&program).expect("compile adjacent param value leaf");
        assert_eq!(
            count_bytes(&bytes, &context_pointer_load_bytes(PARAMS_OFFSET)),
            1,
            "adjacent param loads should materialize the params base pointer once"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate adjacent param leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let params = [1.25_f64, 3.5_f64];
        let ctx = eval_context(&params, &[], &[], &[]);

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 4.75_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_reuses_param_base_across_pure_arithmetic() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushParam(0),
                Instruction::PushVariable(0),
                Instruction::Mul,
                Instruction::PushParam(1),
                Instruction::PushVariable(1),
                Instruction::Mul,
                Instruction::Add,
            ],
            0,
        );

        let bytes = compile_value_function(&program).expect("compile fused param product leaf");
        assert_eq!(
            count_bytes(&bytes, &context_pointer_load_bytes(PARAMS_OFFSET)),
            1,
            "param base should remain cached across direct variable loads and pure arithmetic"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate fused param product leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let params = [1.25_f64, 3.5_f64];
        let vars = [7.0_f64, 10.0_f64];
        let ctx = eval_context(&params, &[], &[], &[]);

        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 43.75_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_reuses_param_base_across_context_scalar_ops() {
        let program = NativeProgram::from_ops_for_test(
            vec![
                NativeOp::LoadParam(0),
                NativeOp::LoadTemperature,
                NativeOp::Add,
                NativeOp::LoadThermalVoltage,
                NativeOp::Add,
                NativeOp::LoadTime,
                NativeOp::Add,
                NativeOp::Analysis(1),
                NativeOp::Add,
                NativeOp::LoadMfactor,
                NativeOp::Add,
                NativeOp::LoadParam(1),
                NativeOp::Add,
            ],
            2,
            Vec::new(),
            Vec::new(),
        );

        let bytes = compile_value_function(&program).expect("compile scalar context param leaf");
        assert_eq!(
            count_bytes(&bytes, &context_pointer_load_bytes(PARAMS_OFFSET)),
            1,
            "param base should remain cached across scalar context loads"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate scalar context leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let params = [1.25_f64, 3.5_f64];
        let mut ctx = eval_context(&params, &[], &[], &[]);
        ctx.temperature = 300.0;
        ctx.time = 2.0;
        ctx.analysis_type = 1;
        ctx.multiplicity = 4.0;

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            (1.25_f64 + 300.0 + thermal_voltage(300.0) + 2.0 + 1.0 + 4.0 + 3.5).to_bits()
        );
    }

    #[test]
    fn generated_value_leaf_reuses_param_base_across_large_signal_noise_ops() {
        let program = NativeProgram::from_ops_for_test(
            vec![
                NativeOp::LoadParam(0),
                NativeOp::Const(1.0),
                NativeOp::WhiteNoise,
                NativeOp::Add,
                NativeOp::Const(2.0),
                NativeOp::Const(3.0),
                NativeOp::FlickerNoise,
                NativeOp::Add,
                NativeOp::LoadParam(1),
                NativeOp::Add,
            ],
            3,
            Vec::new(),
            Vec::new(),
        );

        let bytes = compile_value_function(&program).expect("compile noise param leaf");
        assert_eq!(
            count_bytes(&bytes, &context_pointer_load_bytes(PARAMS_OFFSET)),
            1,
            "param base should remain cached across large-signal noise ops"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate noise param leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let params = [1.25_f64, 3.5_f64];
        let ctx = eval_context(&params, &[], &[], &[]);

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 4.75_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_reuses_param_base_across_inline_dynamic_variable_load() {
        let program = NativeProgram::from_ops_for_test(
            vec![
                NativeOp::LoadParam(0),
                NativeOp::LoadVariableDyn {
                    base: 0,
                    len: 3,
                    lower: 1,
                },
                NativeOp::LoadParam(1),
                NativeOp::Add,
            ],
            2,
            Vec::new(),
            Vec::new(),
        );

        let bytes = compile_value_function(&program).expect("compile dynamic variable param leaf");
        assert_eq!(
            count_bytes(&bytes, &context_pointer_load_bytes(PARAMS_OFFSET)),
            1,
            "param base should remain cached across inline dynamic variable fast path"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate dynamic variable leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let params = [2.0_f64, 3.5_f64];
        let vars = [5.0_f64, 7.0_f64, 9.0_f64];
        let ctx = eval_context(&params, &[], &[], &[]);

        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 10.5_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_reuses_param_base_across_boolean_select_ops() {
        let program = NativeProgram::from_ops_for_test(
            vec![
                NativeOp::LoadParam(0),
                NativeOp::LoadParam(1),
                NativeOp::Compare(CompareOp::Eq),
                NativeOp::LoadParam(2),
                NativeOp::Logical(LogicalOp::And),
                NativeOp::LoadParam(3),
                NativeOp::LoadParam(4),
                NativeOp::IfElse,
                NativeOp::LoadParam(5),
                NativeOp::Add,
            ],
            3,
            Vec::new(),
            Vec::new(),
        );

        let bytes = compile_value_function(&program).expect("compile boolean select param leaf");
        assert_eq!(
            count_bytes(&bytes, &context_pointer_load_bytes(PARAMS_OFFSET)),
            1,
            "param base should remain cached across compare/logical/ifelse ops"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate boolean select leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let params = [2.0_f64, 2.0_f64, 7.0_f64, 10.0_f64, 20.0_f64, 5.0_f64];
        let ctx = eval_context(&params, &[], &[], &[]);

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 15.0_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_reuses_param_base_across_misc_pure_ops() {
        let program = NativeProgram::from_ops_for_test(
            vec![
                NativeOp::LoadParam(0),
                NativeOp::Neg,
                NativeOp::Abs,
                NativeOp::UnaryMath(UnaryMathOp::Floor),
                NativeOp::LoadParam(1),
                NativeOp::Extremum(ExtremumOp::Max),
                NativeOp::LoadParam(2),
                NativeOp::IntegerBinary(IntegerBinaryOp::BitAnd),
                NativeOp::ExtremumConst(ExtremumOp::Max, 2.0),
                NativeOp::LoadParam(3),
                NativeOp::Add,
                NativeOp::LoadParam(4),
                NativeOp::IntegerBinary(IntegerBinaryOp::Shl),
            ],
            2,
            Vec::new(),
            Vec::new(),
        );

        let bytes = compile_value_function(&program).expect("compile misc pure param leaf");
        assert_eq!(
            count_bytes(&bytes, &context_pointer_load_bytes(PARAMS_OFFSET)),
            1,
            "param base should remain cached across branchless pure ops"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate misc pure leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let params = [6.75_f64, 4.0_f64, 3.0_f64, 5.0_f64, 1.0_f64];
        let ctx = eval_context(&params, &[], &[], &[]);

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 14.0_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_without_helper_call_omits_saved_arg_prologue() {
        let program = native_program(EntryKind::StampValue, vec![Instruction::PushConst(1.0)], 0);

        let bytes = compile_value_function(&program).expect("compile literal value function");

        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "helper-free native leaves should not pay callee-saved prologue cost"
        );
    }

    #[test]
    fn generated_value_leaf_uses_register_zero_for_positive_zero_constant() {
        let program = native_program(EntryKind::StampValue, vec![Instruction::PushConst(0.0)], 0);

        let bytes = compile_value_function(&program).expect("compile zero value function");

        assert!(
            contains_bytes(&bytes, &xorpd_xmm_bytes(Xmm::Xmm0, Xmm::Xmm0)),
            "positive zero constants should use a register zero idiom"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate zero value leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[], &[], &[]);

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_preserves_negative_zero_constant_bits() {
        let program = native_program(EntryKind::StampValue, vec![Instruction::PushConst(-0.0)], 0);

        let bytes = compile_value_function(&program).expect("compile negative zero value function");

        assert!(
            !contains_bytes(&bytes, &xorpd_xmm_bytes(Xmm::Xmm0, Xmm::Xmm0)),
            "negative zero constants must remain literal loads to preserve sign bits"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate negative zero leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[], &[], &[]);

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), (-0.0_f64).to_bits());
    }

    #[test]
    fn generated_value_leaf_terminal_helper_call_omits_saved_arg_prologue() {
        let program = native_program(
            EntryKind::StampValue,
            vec![Instruction::PushTemperature, Instruction::Exp],
            0,
        );

        let bytes = compile_value_function(&program).expect("compile helper-call value function");

        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "terminal pure helper-call leaves should not save unused context and vars pointers"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate terminal helper leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.temperature = 0.5;

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            runtime_exp(0.5).to_bits()
        );
    }

    #[test]
    fn generated_value_leaf_terminal_idtmod_preserves_entry_args() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushVariable(0),
                Instruction::PushConst(0.5),
                Instruction::PushConst(1.0),
                Instruction::PushConst(0.25),
                Instruction::IdtModState(1),
            ],
            0,
        );

        let bytes =
            compile_value_function(&program).expect("compile terminal idtmod value function");

        assert!(
            bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "terminal idtmod must preserve context and vars pointers across its internal helper call"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate terminal idtmod leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let vars = [2.0_f64];
        let previous_state = [0.0_f64, 0.9_f64];
        let mut state_values = [0.0_f64, 0.0_f64];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.timestep = 0.25;
        ctx.state_prev = previous_state.as_ptr();
        ctx.state_prev_len = previous_state.len();
        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_values_len = state_values.len();

        let value = f(&ctx, vars.as_ptr());
        assert!((value - 0.4).abs() < 1.0e-12, "value: {value}");
        assert!(
            (state_values[1] - 0.4).abs() < 1.0e-12,
            "state: {state_values:?}"
        );
    }

    #[test]
    fn generated_value_leaf_helper_before_context_load_preserves_entry_args() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushTemperature,
                Instruction::Exp,
                Instruction::PushParam(0),
                Instruction::Add,
            ],
            0,
        );

        let bytes = compile_value_function(&program).expect("compile helper-call value function");

        assert!(
            bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "helper calls before later context loads must preserve context and vars pointers"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate preserved helper leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let mut ctx = eval_context(&[2.0], &[], &[], &[]);
        ctx.temperature = 0.5;

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            (runtime_exp(0.5) + 2.0).to_bits()
        );
    }

    #[test]
    fn helper_call_metadata_preserves_entry_args_for_all_continuing_helpers() {
        let helper_ops = [
            ("unary-math", NativeOp::UnaryMath(UnaryMathOp::Exp), false),
            (
                "binary-math",
                NativeOp::BinaryMath(BinaryMathOp::Pow),
                false,
            ),
            ("table-lookup", NativeOp::TableLookup(0), false),
            ("table-derivative", NativeOp::TableDerivative(0), false),
            ("laplace", NativeOp::LaplaceState(0), false),
            ("zi", NativeOp::ZiState(0), false),
            ("timer", NativeOp::TimerState(0), false),
            ("transition", NativeOp::TransitionState(0), false),
            ("slew", NativeOp::SlewState(0), false),
            ("absdelay", NativeOp::AbsDelayState(0), false),
            ("cross", NativeOp::CrossState(0), false),
            ("idtmod", NativeOp::IdtModState(0), true),
            (
                "dynamic-variable-slow-path",
                NativeOp::LoadVariableDyn {
                    base: 0,
                    len: 1,
                    lower: super::INLINE_DYNAMIC_LOWER_ABS_LIMIT + 1,
                },
                false,
            ),
        ];

        for (name, op, terminal_needs_saved_entry_args) in helper_ops {
            assert!(
                native_op_uses_helper_call(&op),
                "{name} must be classified as a continuing helper call"
            );

            let terminal_program =
                NativeProgram::from_ops_for_test(vec![op], 1, Vec::new(), Vec::new());
            assert_eq!(
                value_program_needs_saved_entry_args(&terminal_program),
                terminal_needs_saved_entry_args,
                "{name}: terminal helper entry-arg preservation mismatch"
            );

            let later_entry_read = NativeProgram::from_ops_for_test(
                vec![op, NativeOp::LoadParam(0)],
                1,
                Vec::new(),
                Vec::new(),
            );
            assert!(
                value_program_needs_saved_entry_args(&later_entry_read),
                "{name} must preserve entry args when a later op reads EvalContext"
            );
        }
    }

    #[test]
    fn helper_call_metadata_preserves_entry_args_for_context_helpers_after_prior_helper() {
        let pure_helper_prefixes = [
            (
                "unary-math",
                vec![NativeOp::Const(0.25), NativeOp::UnaryMath(UnaryMathOp::Exp)],
            ),
            (
                "binary-math",
                vec![
                    NativeOp::Const(2.0),
                    NativeOp::Const(3.0),
                    NativeOp::BinaryMath(BinaryMathOp::Pow),
                ],
            ),
        ];
        let entry_arg_ops = [
            ("load-param", NativeOp::LoadParam(0)),
            ("load-var", NativeOp::LoadVariable(0)),
            ("table-lookup", NativeOp::TableLookup(0)),
            ("table-derivative", NativeOp::TableDerivative(0)),
            ("laplace", NativeOp::LaplaceState(0)),
            ("zi", NativeOp::ZiState(0)),
            ("timer", NativeOp::TimerState(0)),
            ("transition", NativeOp::TransitionState(0)),
            ("slew", NativeOp::SlewState(0)),
            ("absdelay", NativeOp::AbsDelayState(0)),
            ("cross", NativeOp::CrossState(0)),
            ("idtmod", NativeOp::IdtModState(0)),
            (
                "dynamic-variable-slow-path",
                NativeOp::LoadVariableDyn {
                    base: 0,
                    len: 1,
                    lower: super::INLINE_DYNAMIC_LOWER_ABS_LIMIT + 1,
                },
            ),
        ];

        for (prefix_name, prefix_ops) in pure_helper_prefixes {
            for (op_name, op) in entry_arg_ops {
                let mut ops = prefix_ops.clone();
                ops.push(op);
                let program = NativeProgram::from_ops_for_test(ops, 2, Vec::new(), Vec::new());
                assert!(
                    value_program_needs_saved_entry_args(&program),
                    "{prefix_name} before {op_name} must preserve ctx/vars across the first helper"
                );
            }
        }
    }

    #[test]
    fn generated_value_leaf_helper_before_table_helper_preserves_entry_args() {
        let program = NativeProgram::from_ops_for_test(
            vec![
                NativeOp::LoadTemperature,
                NativeOp::UnaryMath(UnaryMathOp::Exp),
                NativeOp::TableLookup(0),
            ],
            1,
            Vec::new(),
            Vec::new(),
        );

        let bytes =
            compile_value_function(&program).expect("compile pure-helper before table-helper leaf");
        assert!(
            bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "table helper after an earlier helper must use saved ctx/vars registers"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate helper table leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let table = [LookupTable::from_data(
            vec![0.0, 1.0, 2.0],
            vec![10.0, 20.0, 40.0],
        )];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.temperature = 0.0;
        ctx.lookup_tables = table.as_ptr();
        ctx.lookup_tables_len = table.len();

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 20.0_f64.to_bits());
    }

    #[test]
    fn helper_call_metadata_keeps_error_return_loads_out_of_helper_class() {
        let entry_arg_loads = [
            ("param-given", NativeOp::LoadParamGiven(0)),
            ("port-connected", NativeOp::LoadPortConnected(0)),
            ("current", NativeOp::LoadCurrent(0)),
            ("prior-current", NativeOp::LoadPriorCurrent(0)),
            (
                "dynamic-variable-inline",
                NativeOp::LoadVariableDyn {
                    base: 0,
                    len: 1,
                    lower: 0,
                },
            ),
        ];

        for (name, op) in entry_arg_loads {
            assert!(
                !native_op_uses_helper_call(&op),
                "{name} may hard-return on error but must not be classified as a continuing helper call"
            );
            assert!(
                native_op_reads_entry_args(op),
                "{name} must still force saved entry args after an earlier continuing helper"
            );
        }
    }

    #[test]
    fn generated_helper_call_abi_sentinels_receive_mixed_arguments() {
        let table = [
            LookupTable::from_data(vec![0.0, 1.0], vec![2.0, 3.0]),
            LookupTable::from_data(vec![0.0, 1.0], vec![4.0, 5.0]),
        ];
        ABI_EXPECTED_TABLE_PTR.store(table.as_ptr() as usize, std::sync::atomic::Ordering::SeqCst);
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.lookup_tables = table.as_ptr();
        ctx.lookup_tables_len = table.len();
        ctx.time = 12.0;
        ctx.temperature = 315.0;
        ctx.multiplicity = 3.0;

        assert_f64_matches(
            run_table_helper_sentinel(abi_sentinel_table_helper, &ctx),
            101.0,
            "table helper ABI sentinel",
        );
        assert_f64_matches(
            run_context_filter_helper_sentinel(abi_sentinel_context_filter_helper, &ctx),
            202.0,
            "context filter helper ABI sentinel",
        );
        assert_f64_matches(
            run_timer_helper_sentinel(abi_sentinel_timer_helper, &ctx),
            303.0,
            "timer helper ABI sentinel",
        );
        assert_f64_matches(
            run_operand_context_filter_helper_sentinel(
                abi_sentinel_operand_context_filter_helper,
                &ctx,
            ),
            404.0,
            "operand context filter helper ABI sentinel",
        );
    }

    #[test]
    fn generated_helper_call_abi_sentinels_enter_helpers_with_aligned_stack() {
        let alignment_helper_memory = stack_alignment_helper_memory();
        let helper_ptr = alignment_helper_memory
            .ptr_at(0)
            .expect("alignment helper entry point");
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.time = 12.0;
        ctx.temperature = 315.0;
        ctx.multiplicity = 3.0;

        let table_helper: TableHelper = unsafe { std::mem::transmute(helper_ptr) };
        assert_f64_matches(
            run_table_helper_sentinel(table_helper, &ctx),
            1.0,
            "table helper stack alignment",
        );

        let context_filter_helper: ContextFilterHelper = unsafe { std::mem::transmute(helper_ptr) };
        assert_f64_matches(
            run_context_filter_helper_sentinel(context_filter_helper, &ctx),
            1.0,
            "context filter helper stack alignment",
        );

        let timer_helper: TimerHelper = unsafe { std::mem::transmute(helper_ptr) };
        assert_f64_matches(
            run_timer_helper_sentinel(timer_helper, &ctx),
            1.0,
            "timer helper stack alignment",
        );

        let operand_context_filter_helper: OperandContextFilterHelper =
            unsafe { std::mem::transmute(helper_ptr) };
        assert_f64_matches(
            run_operand_context_filter_helper_sentinel(operand_context_filter_helper, &ctx),
            1.0,
            "operand context filter helper stack alignment",
        );
    }

    #[test]
    fn generated_value_leaf_no_spill_unary_helper_omits_call_frame_base_register() {
        let program = native_program(
            EntryKind::StampValue,
            vec![Instruction::PushTemperature, Instruction::Exp],
            0,
        );

        let bytes = compile_value_function(&program).expect("compile unary helper value function");
        let frame_bytes = call_frame_bytes(0);
        let old_fixed_frame_bytes = old_fixed_call_frame_bytes();

        assert!(
            contains_bytes(&bytes, &sub_rsp_bytes(frame_bytes)),
            "no-spill helper calls must reserve the minimum ABI call frame"
        );
        assert!(
            contains_bytes(&bytes, &add_rsp_bytes(frame_bytes)),
            "no-spill helper calls must release the minimum ABI call frame"
        );
        assert!(
            !contains_bytes(&bytes, &sub_rsp_bytes(old_fixed_frame_bytes)),
            "no-spill helper calls should not reserve the old maximum spill frame"
        );
        assert!(
            !contains_bytes(&bytes, &add_rsp_bytes(old_fixed_frame_bytes)),
            "no-spill helper calls should not release the old maximum spill frame"
        );
        assert_eq!(
            count_bytes(&bytes, &mov_r11_rsp_bytes()),
            0,
            "unary helper calls without preserved XMM values should not materialize a call-frame base"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate unary helper leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.temperature = 0.5;

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            runtime_exp(0.5).to_bits()
        );
    }

    #[test]
    fn generated_value_leaf_no_spill_binary_helper_omits_call_frame_base_register() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushTemperature,
                Instruction::PushTime,
                Instruction::Pow,
            ],
            0,
        );

        let bytes = compile_value_function(&program).expect("compile binary helper value function");
        let frame_bytes = call_frame_bytes(0);
        let old_fixed_frame_bytes = old_fixed_call_frame_bytes();

        assert!(
            contains_bytes(&bytes, &sub_rsp_bytes(frame_bytes)),
            "no-spill helper calls must reserve the minimum ABI call frame"
        );
        assert!(
            contains_bytes(&bytes, &add_rsp_bytes(frame_bytes)),
            "no-spill helper calls must release the minimum ABI call frame"
        );
        assert!(
            !contains_bytes(&bytes, &sub_rsp_bytes(old_fixed_frame_bytes)),
            "no-spill helper calls should not reserve the old maximum spill frame"
        );
        assert!(
            !contains_bytes(&bytes, &add_rsp_bytes(old_fixed_frame_bytes)),
            "no-spill helper calls should not release the old maximum spill frame"
        );
        assert_eq!(
            count_bytes(&bytes, &mov_r11_rsp_bytes()),
            0,
            "binary helper calls without preserved XMM values should not materialize a call-frame base"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate binary helper leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.temperature = 2.0;
        ctx.time = 3.0;

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 8.0_f64.to_bits());
    }

    #[cfg(windows)]
    #[test]
    fn generated_value_leaf_preserves_win64_callee_saved_xmm_stack_slots() {
        let prefix = variable_prefix(0, XMM_STACK.len());
        let mut instructions = prefix.clone();
        instructions.extend(add_reductions(prefix.len() - 1));
        let program = native_program(EntryKind::StampValue, instructions, 0);
        assert_eq!(program.max_stack_depth(), XMM_STACK.len());

        let bytes = compile_value_function(&program).expect("compile full-depth value function");
        let callee_saved_count = super::callee_saved_xmm_count_for_depth(XMM_STACK.len());
        let frame_bytes =
            super::align_local_frame(super::callee_saved_xmm_frame_bytes(callee_saved_count));
        assert!(
            contains_bytes(&bytes, &sub_rsp_bytes(frame_bytes)),
            "full-depth Win64 value leaf should reserve callee-saved XMM spill space"
        );
        assert!(
            contains_bytes(&bytes, &add_rsp_bytes(frame_bytes)),
            "full-depth Win64 value leaf should release callee-saved XMM spill space"
        );

        for index in 0..callee_saved_count {
            let register = super::callee_saved_xmm_register(index);
            let disp = frame_bytes - super::callee_saved_xmm_frame_bytes(callee_saved_count)
                + (index as i32 * super::CALLEE_SAVED_XMM_BYTES);
            assert_eq!(
                count_bytes(&bytes, &callee_saved_xmm_store_bytes(register, disp)),
                1,
                "{register:?} should be saved exactly once in the prologue"
            );
            assert_eq!(
                count_bytes(&bytes, &callee_saved_xmm_load_bytes(register, disp)),
                1,
                "{register:?} should be restored exactly once in the epilogue"
            );
        }

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate full-depth value leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[], &[], &[]);
        let vars = (0..prefix.len())
            .map(|index| (index + 1) as f64)
            .collect::<Vec<_>>();

        assert_eq!(
            f(&ctx, vars.as_ptr()).to_bits(),
            constant_prefix_sum(prefix.len()).to_bits()
        );
    }

    #[test]
    fn generated_value_leaf_helper_result_stays_in_target_register() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushConst(1.0),
                Instruction::PushParam(0),
                Instruction::Exp,
                Instruction::Add,
            ],
            0,
        );

        assert_eq!(
            program.ops(),
            &[
                NativeOp::Const(1.0),
                NativeOp::LoadParam(0),
                NativeOp::UnaryMath(crate::native::expr::UnaryMathOp::Exp),
                NativeOp::Add,
            ],
            "test fixture must force a unary helper with xmm0 live and xmm1 as target"
        );

        let bytes = compile_value_function(&program).expect("compile helper result leaf");
        let frame_bytes = call_frame_bytes(1);
        let old_fixed_frame_bytes = old_fixed_call_frame_bytes();

        assert!(
            contains_bytes(&bytes, &sub_rsp_bytes(frame_bytes)),
            "single-prefix helper call should reserve only one XMM spill slot"
        );
        assert!(
            contains_bytes(&bytes, &add_rsp_bytes(frame_bytes)),
            "single-prefix helper call should release only one XMM spill slot"
        );
        assert!(
            !contains_bytes(&bytes, &sub_rsp_bytes(old_fixed_frame_bytes)),
            "single-prefix helper call should not reserve the old maximum spill frame"
        );
        assert!(
            !contains_bytes(&bytes, &add_rsp_bytes(old_fixed_frame_bytes)),
            "single-prefix helper call should not release the old maximum spill frame"
        );
        assert_eq!(
            count_bytes(&bytes, &mov_r11_rsp_bytes()),
            0,
            "helper calls should address spills directly from rsp instead of materializing a call-frame base"
        );
        assert!(
            contains_bytes(&bytes, &call_frame_spill_bytes(0, Xmm::Xmm0)),
            "live prefix should spill directly to the call frame"
        );
        assert!(
            contains_bytes(&bytes, &call_frame_load_bytes(Xmm::Xmm0, 0)),
            "live prefix should reload directly from the call frame"
        );

        let mut old_result_store = X64Encoder::new();
        old_result_store.movsd_m64_base_disp32_xmm(Gpr::R11, call_result_disp(), Xmm::Xmm0);
        assert!(
            !contains_bytes(&bytes, &old_result_store.into_bytes()),
            "helper result should not be spilled to the call frame"
        );

        let mut old_result_reload = X64Encoder::new();
        old_result_reload.movsd_xmm_m64_base_disp32(Xmm::Xmm1, Gpr::R11, call_result_disp());
        assert!(
            !contains_bytes(&bytes, &old_result_reload.into_bytes()),
            "helper result should move directly into the target register"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate helper result leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[2.0], &[], &[], &[]);

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            (1.0_f64 + runtime_exp(2.0)).to_bits()
        );
    }

    #[test]
    fn generated_value_leaf_applies_constant_rhs_arithmetic_without_extra_stack_slot() {
        let cases = [
            (Instruction::Add, 12.0_f64),
            (Instruction::Sub, 4.0_f64),
            (Instruction::Mul, 32.0_f64),
            (Instruction::Div, 2.0_f64),
        ];

        for (instruction, expected) in cases {
            let instruction_name = format!("{instruction:?}");
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(4.0),
                    instruction,
                ],
                0,
            );

            assert_eq!(
                program.max_stack_depth(),
                1,
                "{instruction_name} should use a literal RHS instruction, not a second stack slot"
            );

            let bytes =
                compile_value_function(&program).expect("compile literal RHS arithmetic leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant RHS arithmetic should stay helper-free"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate literal RHS arithmetic leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = 8.0;

            assert_eq!(f(&ctx, std::ptr::null()).to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn generated_value_leaf_doubles_constant_rhs_mul_without_literal_load() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushParam(0),
                Instruction::PushConst(2.0),
                Instruction::Mul,
            ],
            0,
        );
        assert_eq!(
            program.ops(),
            &[NativeOp::LoadParam(0), NativeOp::MulConst(2.0)],
            "x * 2.0 should stay a literal RHS multiply in native IR"
        );
        assert_eq!(
            program.max_stack_depth(),
            1,
            "x * 2.0 should not allocate an RHS stack slot"
        );

        let bytes = compile_value_function(&program).expect("compile constant double leaf");
        assert!(
            contains_bytes(&bytes, &addsd_xmm_xmm_bytes(Xmm::Xmm0, Xmm::Xmm0)),
            "x * 2.0 should emit an in-place addsd"
        );
        assert!(
            !contains_bytes(&bytes, &mulsd_xmm_m64_rip_prefix_bytes(Xmm::Xmm0)),
            "x * 2.0 should not emit a literal-pool mulsd"
        );
        assert!(
            !contains_bytes(&bytes, &2.0_f64.to_le_bytes()),
            "x * 2.0 should not append a 2.0 literal"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate constant double leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let cases = [
            ("finite", 3.25_f64),
            ("negative-zero", -0.0_f64),
            ("positive-infinity", f64::INFINITY),
            ("overflow", f64::MAX),
            ("unordered", f64::from_bits(0x7ff0_0000_0000_1111)),
        ];

        for (name, input) in cases {
            let params = [input];
            let ctx = eval_context(&params, &[], &[], &[]);
            assert_f64_matches(f(&ctx, std::ptr::null()), input * 2.0, name);
        }
    }

    #[test]
    fn generated_value_leaf_applies_constant_lhs_commutative_arithmetic_without_extra_stack_slot() {
        let cases = [
            (
                "add-finite",
                Instruction::Add,
                10.0_f64,
                3.25_f64,
                10.0_f64 + 3.25_f64,
            ),
            (
                "mul-finite",
                Instruction::Mul,
                -3.5_f64,
                4.0_f64,
                -3.5_f64 * 4.0_f64,
            ),
            (
                "add-snan-rhs",
                Instruction::Add,
                10.0_f64,
                f64::from_bits(0x7ff0_0000_0000_1111),
                10.0_f64 + f64::from_bits(0x7ff0_0000_0000_1111),
            ),
        ];

        for (name, instruction, lhs, input, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(lhs),
                    Instruction::PushTemperature,
                    instruction,
                ],
                0,
            );

            assert_eq!(
                program.max_stack_depth(),
                1,
                "{name} should use a literal LHS commutative op, not a second stack slot"
            );

            let bytes = compile_value_function(&program)
                .expect("compile literal LHS commutative arithmetic leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant LHS commutative arithmetic should stay helper-free"
            );

            let memory = ExecutableMemory::allocate(&bytes)
                .expect("allocate literal LHS commutative arithmetic leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = input;

            assert_f64_matches(f(&ctx, std::ptr::null()), expected, name);
        }
    }

    #[test]
    fn generated_value_leaf_folds_constant_binary_arithmetic_to_literal() {
        let cases = [
            (
                "add finite",
                Instruction::Add,
                3.25_f64,
                4.5_f64,
                3.25_f64 + 4.5_f64,
            ),
            (
                "sub finite",
                Instruction::Sub,
                3.25_f64,
                4.5_f64,
                3.25_f64 - 4.5_f64,
            ),
            (
                "mul signed zero",
                Instruction::Mul,
                -0.0_f64,
                4.5_f64,
                -0.0_f64 * 4.5_f64,
            ),
            (
                "div negative zero",
                Instruction::Div,
                10.0_f64,
                -0.0_f64,
                10.0_f64 / -0.0_f64,
            ),
            (
                "mul unordered",
                Instruction::Mul,
                f64::from_bits(0x7ff8_0000_0000_0003),
                4.5_f64,
                f64::from_bits(0x7ff8_0000_0000_0003) * 4.5_f64,
            ),
        ];

        for (case, instruction, left, right, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    instruction,
                ],
                0,
            );

            match program.ops() {
                [NativeOp::Const(value)] => {
                    assert_f64_matches(*value, expected, case);
                }
                ops => panic!("{case} lowered to unexpected ops: {ops:?}"),
            }

            let bytes =
                compile_value_function(&program).expect("compile folded arithmetic literal leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "folded arithmetic literal should stay helper-free"
            );

            let memory = ExecutableMemory::allocate(&bytes)
                .expect("allocate folded arithmetic literal leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_f64_matches(f(&ctx, std::ptr::null()), expected, case);
        }
    }

    #[test]
    fn generated_value_leaf_applies_constant_lhs_sub_div_without_extra_stack_slot() {
        let temp_sub_rsp = sub_rsp_bytes(ROUND_TEMP_FRAME_BYTES);
        let temp_add_rsp = add_rsp_bytes(ROUND_TEMP_FRAME_BYTES);
        let cases = [
            (
                "sub-finite",
                Instruction::Sub,
                10.0_f64,
                3.0_f64,
                10.0_f64 - 3.0_f64,
            ),
            (
                "sub-signed-zero",
                Instruction::Sub,
                -0.0_f64,
                0.0_f64,
                -0.0_f64 - 0.0_f64,
            ),
            (
                "sub-unordered",
                Instruction::Sub,
                10.0_f64,
                f64::from_bits(0x7ff8_0000_0000_0001),
                10.0_f64 - f64::from_bits(0x7ff8_0000_0000_0001),
            ),
            (
                "div-finite",
                Instruction::Div,
                10.0_f64,
                4.0_f64,
                10.0_f64 / 4.0_f64,
            ),
            (
                "div-negative-zero",
                Instruction::Div,
                10.0_f64,
                -0.0_f64,
                10.0_f64 / -0.0_f64,
            ),
            (
                "div-unordered",
                Instruction::Div,
                10.0_f64,
                f64::from_bits(0x7ff8_0000_0000_0002),
                10.0_f64 / f64::from_bits(0x7ff8_0000_0000_0002),
            ),
        ];

        for (name, instruction, lhs, input, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(lhs),
                    Instruction::PushTemperature,
                    instruction,
                ],
                0,
            );

            assert_eq!(
                program.max_stack_depth(),
                1,
                "{name} should use a literal LHS arithmetic op, not a second stack slot"
            );

            let bytes =
                compile_value_function(&program).expect("compile literal LHS arithmetic leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant LHS arithmetic should stay helper-free"
            );
            assert!(
                !contains_bytes(&bytes, &temp_sub_rsp),
                "{name} should use an XMM scratch instead of an RSP temp slot"
            );
            assert!(
                !contains_bytes(&bytes, &temp_add_rsp),
                "{name} should not restore an unused RSP temp slot"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate literal LHS arithmetic leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = input;

            assert_f64_matches(f(&ctx, std::ptr::null()), expected, name);
        }
    }

    #[test]
    fn generated_value_leaf_applies_constant_lhs_arithmetic_at_full_xmm_stack_depth() {
        let temp_sub_rsp = sub_rsp_bytes(ROUND_TEMP_FRAME_BYTES);
        let temp_add_rsp = add_rsp_bytes(ROUND_TEMP_FRAME_BYTES);
        let cases = [
            ("sub", Instruction::Sub, 4.0_f64, 10.0_f64 - 4.0_f64),
            ("div", Instruction::Div, 4.0_f64, 10.0_f64 / 4.0_f64),
        ];

        for (name, instruction, input, folded_value) in cases {
            let prefix = constant_prefix(XMM_STACK.len() - 1);
            let mut instructions = prefix.clone();
            instructions.extend([
                Instruction::PushConst(10.0),
                Instruction::PushTemperature,
                instruction,
            ]);
            instructions.extend(add_reductions(prefix.len()));
            let program = native_program(EntryKind::StampValue, instructions, 0);

            assert_eq!(program.max_stack_depth(), XMM_STACK.len(), "{name}");

            let bytes = compile_value_function(&program)
                .expect("compile full-stack literal LHS arithmetic leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant LHS arithmetic should stay helper-free at full XMM stack depth"
            );
            assert!(
                contains_bytes(&bytes, &temp_sub_rsp),
                "{name} should fall back to an RSP temp slot at full XMM stack depth"
            );
            assert!(
                contains_bytes(&bytes, &temp_add_rsp),
                "{name} should restore the RSP temp slot at full XMM stack depth"
            );

            let memory = ExecutableMemory::allocate(&bytes)
                .expect("allocate full-stack literal LHS arithmetic leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = input;

            let expected = constant_prefix_sum(prefix.len()) + folded_value;
            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_applies_constant_rhs_comparisons_without_extra_stack_slot() {
        let cases = [
            (Instruction::Gt, 8.0_f64, 1.0_f64),
            (Instruction::Ge, 4.0_f64, 1.0_f64),
            (Instruction::Lt, 2.0_f64, 1.0_f64),
            (Instruction::Le, 4.0_f64, 1.0_f64),
            (Instruction::Eq, 4.0_f64, 1.0_f64),
            (Instruction::Ne, 8.0_f64, 1.0_f64),
            (Instruction::Lt, f64::NAN, 0.0_f64),
            (Instruction::Le, f64::NAN, 0.0_f64),
            (Instruction::Eq, f64::NAN, 0.0_f64),
            (Instruction::Ne, f64::NAN, 0.0_f64),
        ];

        for (instruction, input, expected) in cases {
            let instruction_name = format!("{instruction:?}");
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(4.0),
                    instruction,
                ],
                0,
            );

            assert_eq!(
                program.max_stack_depth(),
                1,
                "{instruction_name} should use a literal RHS compare, not a second stack slot"
            );

            let bytes =
                compile_value_function(&program).expect("compile literal RHS comparison leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant RHS comparison should stay helper-free"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate literal RHS comparison leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = input;

            assert_eq!(f(&ctx, std::ptr::null()).to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn generated_value_leaf_compares_zero_rhs_without_literal_subtract() {
        let cases = [
            ("eq-positive-zero", Instruction::Eq, CompareOp::Eq, 0.0),
            ("eq-negative-zero", Instruction::Eq, CompareOp::Eq, -0.0),
            ("ne-positive-zero", Instruction::Ne, CompareOp::Ne, 0.0),
            ("ne-negative-zero", Instruction::Ne, CompareOp::Ne, -0.0),
        ];
        let inputs = [
            ("positive-zero", 0.0_f64),
            ("negative-zero", -0.0_f64),
            ("within-epsilon", 0.5e-15_f64),
            ("at-epsilon", 1.0e-15_f64),
            ("infinity", f64::INFINITY),
            ("unordered", f64::NAN),
        ];

        for (name, instruction, expected_op, literal) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushParam(0),
                    Instruction::PushConst(literal),
                    instruction,
                ],
                0,
            );

            assert_eq!(
                program.ops(),
                &[
                    NativeOp::LoadParam(0),
                    NativeOp::CompareConst(expected_op, literal)
                ],
                "{name} should lower to a literal RHS comparison"
            );
            assert_eq!(
                program.max_stack_depth(),
                1,
                "{name} should use a literal RHS compare, not a second stack slot"
            );

            let bytes =
                compile_value_function(&program).expect("compile zero literal comparison leaf");
            assert!(
                !contains_bytes(&bytes, &subsd_xmm_m64_rip_prefix_bytes(Xmm::Xmm0)),
                "{name} should not subtract a zero literal before the epsilon compare"
            );
            assert_eq!(
                count_bytes(&bytes, &BOOLEAN_EPSILON.to_le_bytes()),
                1,
                "{name} should keep exactly one epsilon comparison literal"
            );

            let memory = ExecutableMemory::allocate(&bytes).expect("allocate zero comparison leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };

            for (input_name, input) in inputs {
                let params = [input];
                let ctx = eval_context(&params, &[], &[], &[]);
                let expected = match expected_op {
                    CompareOp::Eq => {
                        if input.is_nan() || input.abs() >= BOOLEAN_EPSILON {
                            0.0_f64
                        } else {
                            1.0_f64
                        }
                    }
                    CompareOp::Ne => {
                        if input.is_nan() || input.abs() < BOOLEAN_EPSILON {
                            0.0_f64
                        } else {
                            1.0_f64
                        }
                    }
                    CompareOp::Gt | CompareOp::Ge | CompareOp::Lt | CompareOp::Le => {
                        unreachable!("zero comparison test only covers equality operators")
                    }
                };

                assert_eq!(
                    f(&ctx, std::ptr::null()).to_bits(),
                    expected.to_bits(),
                    "{name}:{input_name}"
                );
            }
        }
    }

    #[test]
    fn generated_value_leaf_applies_constant_lhs_comparisons_without_extra_stack_slot() {
        let cases = [
            (Instruction::Gt, CompareOp::Lt, 2.0_f64, 1.0_f64),
            (Instruction::Ge, CompareOp::Le, 4.0_f64, 1.0_f64),
            (Instruction::Lt, CompareOp::Gt, 8.0_f64, 1.0_f64),
            (Instruction::Le, CompareOp::Ge, 4.0_f64, 1.0_f64),
            (Instruction::Eq, CompareOp::Eq, 4.0_f64, 1.0_f64),
            (Instruction::Ne, CompareOp::Ne, 8.0_f64, 1.0_f64),
            (Instruction::Gt, CompareOp::Lt, f64::NAN, 0.0_f64),
            (Instruction::Ge, CompareOp::Le, f64::NAN, 0.0_f64),
            (Instruction::Eq, CompareOp::Eq, f64::NAN, 0.0_f64),
            (Instruction::Ne, CompareOp::Ne, f64::NAN, 0.0_f64),
        ];

        for (instruction, expected_op, input, expected) in cases {
            let instruction_name = format!("{instruction:?}");
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(4.0),
                    Instruction::PushTemperature,
                    instruction,
                ],
                0,
            );

            assert_eq!(
                program.max_stack_depth(),
                1,
                "{instruction_name} should use a literal LHS compare, not a second stack slot"
            );
            assert_eq!(
                program.ops(),
                &[
                    NativeOp::LoadTemperature,
                    NativeOp::CompareConst(expected_op, 4.0)
                ],
                "{instruction_name} should flip the comparison around the literal LHS"
            );

            let bytes =
                compile_value_function(&program).expect("compile literal LHS comparison leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant LHS comparison should stay helper-free"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate literal LHS comparison leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = input;

            assert_eq!(f(&ctx, std::ptr::null()).to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn generated_value_leaf_folds_constant_comparisons_to_literals() {
        let cases = [
            ("gt-true", Instruction::Gt, 5.0, 4.0, 1.0),
            ("gt-unordered", Instruction::Gt, f64::NAN, 4.0, 0.0),
            ("ge-equal", Instruction::Ge, 4.0, 4.0, 1.0),
            ("lt-true", Instruction::Lt, 3.0, 4.0, 1.0),
            ("le-equal", Instruction::Le, 4.0, 4.0, 1.0),
            ("le-unordered", Instruction::Le, 4.0, f64::NAN, 0.0),
            ("eq-within-epsilon", Instruction::Eq, 0.0, 0.5e-15, 1.0),
            ("eq-at-epsilon", Instruction::Eq, 0.0, 1.0e-15, 0.0),
            ("eq-unordered", Instruction::Eq, f64::NAN, 0.0, 0.0),
            ("ne-within-epsilon", Instruction::Ne, 0.0, 0.5e-15, 0.0),
            ("ne-at-epsilon", Instruction::Ne, 0.0, 1.0e-15, 1.0),
            ("ne-unordered", Instruction::Ne, 0.0, f64::NAN, 0.0),
        ];

        for (name, instruction, left, right, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    instruction,
                ],
                0,
            );

            assert_eq!(program.max_stack_depth(), 1, "{name}");
            assert_eq!(
                program.ops(),
                &[NativeOp::Const(expected)],
                "{name} should compile as a folded literal"
            );

            let bytes = compile_value_function(&program).expect("compile folded comparison leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "folded comparison should stay helper-free"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate folded comparison leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_squares_constant_power_without_helper_call() {
        for instruction in [Instruction::Pow, Instruction::FnPow] {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(2.0),
                    instruction,
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile square power leaf");

            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant-square power should not pay helper-call prologue cost"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate square power native leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = -3.0;

            assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 9.0_f64.to_bits());
        }
    }

    #[test]
    fn generated_value_leaf_elides_identity_power_helper_call() {
        for instruction in [Instruction::Pow, Instruction::FnPow] {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(1.0),
                    instruction,
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile identity power leaf");

            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant-one power should not pay helper-call prologue cost"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate identity power native leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = -0.0;

            assert_eq!(f(&ctx, std::ptr::null()).to_bits(), (-0.0_f64).to_bits());
        }
    }

    #[test]
    fn generated_value_leaf_elides_reciprocal_power_helper_call() {
        for instruction in [Instruction::Pow, Instruction::FnPow] {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(-1.0),
                    instruction,
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile reciprocal power leaf");

            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant-minus-one power should not pay helper-call prologue cost"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate reciprocal power native leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = -0.0;

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                f64::NEG_INFINITY.to_bits()
            );
        }
    }

    #[test]
    fn generated_value_leaf_loads_dynamic_variable_and_preserves_stack() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushConst(1.0),
                Instruction::PushParam(0),
                Instruction::PushVariableDyn {
                    base: 1,
                    len: 3,
                    lower: 1,
                },
                Instruction::Add,
            ],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile dynamic variable leaf");
        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "dynamic read fast path should not pay helper-call prologue cost"
        );
        assert!(
            !contains_bytes(&bytes, &sub_rsp_bytes(DYNAMIC_READ_FRAME_BYTES)),
            "dynamic read with a spare XMM register should not spill the index before the fast path"
        );
        assert!(
            !contains_bytes(&bytes, &add_rsp_bytes(DYNAMIC_READ_FRAME_BYTES)),
            "dynamic read with a spare XMM register should not restore a fast-path spill frame"
        );
        assert!(
            contains_bytes(&bytes, &dynamic_variable_scaled_address_bytes(1)),
            "dynamic read fast path should use one scaled indexed LEA for variable address formation"
        );
        assert!(
            !contains_bytes(&bytes, &dynamic_variable_shift_add_address_bytes(1)),
            "dynamic read fast path should not use the old shift/add address sequence"
        );
        assert!(
            contains_bytes(&bytes, &sub_r64_imm32_bytes(Gpr::R10, 1)),
            "dynamic read fast path should subtract small lower bounds as an imm32"
        );
        assert!(
            contains_bytes(&bytes, &cmp_r64_imm32_bytes(Gpr::R10, 3)),
            "dynamic read fast path should compare small lengths as an imm32"
        );
        assert!(
            !contains_bytes(&bytes, &dynamic_variable_movabs_sub_lower_bytes(1)),
            "dynamic read fast path should not materialize small lower bounds in a GPR"
        );
        assert!(
            !contains_bytes(&bytes, &dynamic_variable_movabs_cmp_len_bytes(3)),
            "dynamic read fast path should not materialize small lengths in a GPR"
        );
        assert!(
            contains_bytes(
                &bytes,
                &mov_r32_imm32_bytes(super::dynamic_variable_len_arg_reg(), 3)
            ),
            "dynamic read helper slow path should materialize small lengths with a compact imm32 move"
        );
        assert!(
            contains_bytes(
                &bytes,
                &mov_r32_imm32_bytes(super::dynamic_variable_lower_arg_reg(), 1)
            ),
            "dynamic read helper slow path should materialize small positive lower bounds with a compact imm32 move"
        );
        assert!(
            !contains_bytes(
                &bytes,
                &movabs_imm64_bytes(super::dynamic_variable_len_arg_reg(), 3)
            ),
            "dynamic read helper slow path should not materialize small lengths with movabs"
        );
        assert!(
            !contains_bytes(
                &bytes,
                &movabs_imm64_bytes(super::dynamic_variable_lower_arg_reg(), 1)
            ),
            "dynamic read helper slow path should not materialize small positive lower bounds with movabs"
        );
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate dynamic variable leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let vars = [99.0_f64, 2.0, 4.0, 8.0];
        let ctx = eval_context(&[2.49], &[], &[], &[]);
        clear_native_runtime_error();

        let loaded = f(&ctx, vars.as_ptr());

        assert_eq!(loaded.to_bits(), 5.0_f64.to_bits());
        assert!(take_native_runtime_error().is_none());
    }

    #[test]
    fn generated_value_leaf_helper_free_dynamic_variable_read_hard_fails_bounds() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushParam(0),
                Instruction::PushVariableDyn {
                    base: 1,
                    len: 3,
                    lower: 1,
                },
            ],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile dynamic variable leaf");
        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "dynamic read slow path should not require the helper-call prologue"
        );
        assert!(
            !contains_bytes(&bytes, &sub_rsp_bytes(DYNAMIC_READ_FRAME_BYTES)),
            "out-of-range dynamic read with a spare XMM register should still avoid the old spill frame"
        );
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate dynamic variable leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let vars = [99.0_f64, 2.0, 4.0, 8.0];
        let ctx = eval_context(&[4.0], &[], &[], &[]);
        clear_native_runtime_error();

        let loaded = f(&ctx, vars.as_ptr());

        assert_eq!(loaded.to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error().expect("out-of-range dynamic read must hard-fail");
        assert!(
            error.contains("array index 4 outside declared bounds [1:3]"),
            "error must preserve array bounds diagnostic, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    #[test]
    fn generated_value_leaf_zero_based_dynamic_variable_read_skips_lower_subtract() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushParam(0),
                Instruction::PushVariableDyn {
                    base: 0,
                    len: 4,
                    lower: 0,
                },
            ],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile zero-based dynamic read");
        assert!(
            !contains_bytes(&bytes, &sub_r64_imm32_bytes(Gpr::R10, 0)),
            "zero-based dynamic read fast path should not emit a no-op lower-bound subtract"
        );
        assert!(
            contains_bytes(&bytes, &cmp_r64_imm32_bytes(Gpr::R10, 4)),
            "zero-based dynamic read fast path should still bounds-check the normalized index"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate zero-based dynamic read");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let vars = [2.0_f64, 4.0, 8.0, 16.0];
        let ctx = eval_context(&[2.49], &[], &[], &[]);
        clear_native_runtime_error();

        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 8.0_f64.to_bits());
        assert!(take_native_runtime_error().is_none());
    }

    #[test]
    fn generated_value_leaf_keeps_full_stack_dynamic_variable_read_on_spill_path() {
        let prefix = constant_prefix(XMM_STACK.len() - 1);
        let mut instructions = prefix.clone();
        instructions.extend([
            Instruction::PushParam(0),
            Instruction::PushVariableDyn {
                base: 1,
                len: 3,
                lower: 1,
            },
        ]);
        instructions.extend(add_reductions(prefix.len()));
        let program = native_program(EntryKind::StampValue, instructions, 0);
        assert_eq!(program.max_stack_depth(), XMM_STACK.len());
        let bytes =
            compile_value_function(&program).expect("compile full-stack dynamic variable leaf");
        assert!(
            contains_bytes(&bytes, &sub_rsp_bytes(DYNAMIC_READ_FRAME_BYTES)),
            "full-stack dynamic read must keep the stack spill fallback"
        );
        assert!(
            contains_bytes(&bytes, &add_rsp_bytes(DYNAMIC_READ_FRAME_BYTES)),
            "full-stack dynamic read must restore the stack spill fallback"
        );

        let memory =
            ExecutableMemory::allocate(&bytes).expect("allocate full-stack dynamic variable leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let vars = [99.0_f64, 2.0, 4.0, 8.0];
        let ctx = eval_context(&[2.49], &[], &[], &[]);
        clear_native_runtime_error();

        let loaded = f(&ctx, vars.as_ptr());

        assert_eq!(
            loaded.to_bits(),
            (constant_prefix_sum(prefix.len()) + 4.0).to_bits()
        );
        assert!(take_native_runtime_error().is_none());
    }

    #[test]
    fn generated_value_leaf_keeps_huge_dynamic_variable_ranges_on_helper_path() {
        let huge_len = (1_usize << 52) + 1;
        let program = NativeProgram::from_bytecode(
            "x64-codegen-test",
            EntryKind::StampValue,
            &BytecodeProgram {
                instructions: vec![
                    Instruction::PushParam(0),
                    Instruction::PushVariableDyn {
                        base: 0,
                        len: huge_len,
                        lower: 0,
                    },
                ],
            },
            NativeLoweringLimits::new(0, 0, 1, huge_len, 0),
        )
        .expect("huge dynamic range is valid IR before x64 lowering");
        let bytes = compile_value_function(&program).expect("compile huge dynamic range leaf");

        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "terminal huge dynamic helper leaves should not save unused context and vars pointers"
        );
        assert!(
            contains_bytes(
                &bytes,
                &movabs_imm64_bytes(super::dynamic_variable_len_arg_reg(), huge_len as u64)
            ),
            "huge dynamic ranges must still use the helper-backed continuation path"
        );
    }

    #[test]
    fn generated_value_leaf_folds_constant_dynamic_variable_read_to_direct_load() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushConst(1.0),
                Instruction::PushConst(2.49),
                Instruction::PushVariableDyn {
                    base: 1,
                    len: 3,
                    lower: 1,
                },
                Instruction::Add,
            ],
            0,
        );

        assert_eq!(
            program.ops(),
            &[NativeOp::LoadVariable(2), NativeOp::AddConst(1.0)],
            "finite in-range literal dynamic index should lower to direct variable load"
        );

        let bytes = compile_value_function(&program).expect("compile folded dynamic variable leaf");
        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "folded dynamic read should be helper-free"
        );
        let memory =
            ExecutableMemory::allocate(&bytes).expect("allocate folded dynamic variable leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let vars = [99.0_f64, 2.0, 4.0, 8.0];
        let ctx = eval_context(&[], &[], &[], &[]);
        clear_native_runtime_error();

        let loaded = f(&ctx, vars.as_ptr());

        assert_eq!(loaded.to_bits(), 5.0_f64.to_bits());
        assert!(take_native_runtime_error().is_none());
    }

    #[test]
    fn generated_value_leaf_hard_fails_dynamic_variable_bounds_errors() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushConst(4.0),
                Instruction::PushVariableDyn {
                    base: 1,
                    len: 3,
                    lower: 1,
                },
            ],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile dynamic variable leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate dynamic variable leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let vars = [99.0_f64, 2.0, 4.0, 8.0];
        let ctx = eval_context(&[], &[], &[], &[]);
        clear_native_runtime_error();

        let loaded = f(&ctx, vars.as_ptr());

        assert_eq!(loaded.to_bits(), 0.0_f64.to_bits());
        let error =
            take_native_runtime_error().expect("out-of-range native array read must hard-fail");
        assert!(
            error.contains("array index 4 outside declared bounds [1:3]"),
            "error must preserve bounds diagnostic, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    #[test]
    fn generated_assignment_leaf_stores_native_expression_result() {
        let program = native_program(
            EntryKind::Assignment,
            vec![
                Instruction::PushParam(0),
                Instruction::PushVoltage(1, 0),
                Instruction::Add,
            ],
            2,
        );
        let bytes = compile_assignment_function(2, &program).expect("compile assignment function");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate assignment leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *mut f64) = unsafe { std::mem::transmute(entry) };

        let params = [3.0_f64];
        let voltages = [1.0_f64, 6.0_f64];
        let mut vars = [0.0_f64; 3];
        let ctx = eval_context(&params, &voltages, &[], &[]);

        f(&ctx, vars.as_mut_ptr());

        assert_eq!(vars[2], 8.0);
    }

    #[test]
    fn generated_assignment_pass_stores_indexed_value_and_preserves_slot_across_helper_call() {
        let assignments = [
            NativeAssignment::Indexed {
                base: 1,
                len: 3,
                lower: 1,
                index: native_program(EntryKind::Assignment, vec![Instruction::PushConst(2.49)], 0),
                value: native_program(
                    EntryKind::Assignment,
                    vec![Instruction::PushConst(2.0), Instruction::Exp],
                    0,
                ),
            },
            NativeAssignment::Direct {
                var_index: 0,
                program: native_program(
                    EntryKind::Assignment,
                    vec![
                        Instruction::PushVariable(2),
                        Instruction::PushConst(1.0),
                        Instruction::Add,
                    ],
                    0,
                ),
            },
        ];
        let bytes = compile_assignment_pass_function(&assignments)
            .expect("compile indexed assignment pass");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate indexed assignment pass");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *mut f64) = unsafe { std::mem::transmute(entry) };

        let mut vars = [0.0_f64, 2.0, 4.0, 8.0];
        let ctx = eval_context(&[], &[], &[], &[]);
        clear_native_runtime_error();

        f(&ctx, vars.as_mut_ptr());

        let expected = runtime_exp(2.0);
        assert_eq!(vars[2].to_bits(), expected.to_bits());
        assert_eq!(vars[0].to_bits(), (expected + 1.0).to_bits());
        assert!(take_native_runtime_error().is_none());
    }

    #[test]
    fn generated_assignment_pass_helper_free_indexed_assignment_stores_runtime_slot() {
        let assignments = [
            NativeAssignment::Indexed {
                base: 1,
                len: 3,
                lower: 1,
                index: native_program(EntryKind::Assignment, vec![Instruction::PushParam(0)], 0),
                value: native_program(EntryKind::Assignment, vec![Instruction::PushParam(1)], 0),
            },
            NativeAssignment::Direct {
                var_index: 0,
                program: native_program(
                    EntryKind::Assignment,
                    vec![Instruction::PushConst(123.0)],
                    0,
                ),
            },
        ];
        assert!(
            !assignment_uses_helper_calls(&assignments[0]),
            "supported runtime indexed writes should not force the helper-call prologue"
        );
        let bytes = compile_assignment_pass_function(&assignments)
            .expect("compile helper-free indexed assignment pass");
        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "helper-free indexed writes should not pay the saved-argument prologue"
        );
        assert!(
            contains_bytes(&bytes, &sub_r64_imm32_bytes(Gpr::R10, 1)),
            "indexed write fast path should subtract small lower bounds as an imm32"
        );
        assert!(
            contains_bytes(&bytes, &cmp_r64_imm32_bytes(Gpr::R10, 3)),
            "indexed write fast path should compare small lengths as an imm32"
        );
        assert!(
            !contains_bytes(&bytes, &dynamic_variable_movabs_sub_lower_bytes(1)),
            "indexed write fast path should not materialize small lower bounds in a GPR"
        );
        assert!(
            !contains_bytes(&bytes, &dynamic_variable_movabs_cmp_len_bytes(3)),
            "indexed write fast path should not materialize small lengths in a GPR"
        );
        let memory =
            ExecutableMemory::allocate(&bytes).expect("allocate helper-free indexed assignment");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *mut f64) = unsafe { std::mem::transmute(entry) };

        let params = [2.49_f64, 11.0];
        let mut vars = [0.0_f64, 2.0, 4.0, 8.0];
        let ctx = eval_context(&params, &[], &[], &[]);
        clear_native_runtime_error();

        f(&ctx, vars.as_mut_ptr());

        assert_eq!(vars, [123.0, 2.0, 11.0, 8.0]);
        assert!(take_native_runtime_error().is_none());
    }

    #[test]
    fn generated_assignment_pass_helper_free_indexed_assignment_handles_negative_lower_bound() {
        let assignments = [NativeAssignment::Indexed {
            base: 1,
            len: 3,
            lower: -2,
            index: native_program(EntryKind::Assignment, vec![Instruction::PushParam(0)], 0),
            value: native_program(EntryKind::Assignment, vec![Instruction::PushParam(1)], 0),
        }];
        assert!(
            !assignment_uses_helper_calls(&assignments[0]),
            "supported negative lower-bound indexed writes should not require the helper"
        );
        let bytes = compile_assignment_pass_function(&assignments)
            .expect("compile negative lower-bound indexed assignment pass");
        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "helper-free indexed writes should not pay the saved-argument prologue"
        );
        assert!(
            contains_bytes(&bytes, &sub_r64_imm32_bytes(Gpr::R10, -2)),
            "indexed write fast path should subtract negative lower bounds with a sign-extended imm32"
        );
        assert!(
            contains_bytes(
                &bytes,
                &mov_r64_imm32_bytes(super::dynamic_variable_lower_arg_reg(), -2)
            ),
            "indexed write helper slow path should materialize small negative lower bounds with a sign-extended imm32 move"
        );
        assert!(
            !contains_bytes(&bytes, &dynamic_variable_movabs_sub_lower_bytes(-2)),
            "indexed write fast path should not materialize small negative lower bounds in a GPR"
        );
        assert!(
            !contains_bytes(
                &bytes,
                &movabs_imm64_bytes(super::dynamic_variable_lower_arg_reg(), (-2_i64) as u64)
            ),
            "indexed write helper slow path should not materialize small negative lower bounds with movabs"
        );
        let memory = ExecutableMemory::allocate(&bytes)
            .expect("allocate negative lower-bound indexed assignment");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *mut f64) = unsafe { std::mem::transmute(entry) };

        let params = [-1.5_f64, 12.0];
        let mut vars = [0.0_f64, 2.0, 4.0, 8.0];
        let ctx = eval_context(&params, &[], &[], &[]);
        clear_native_runtime_error();

        f(&ctx, vars.as_mut_ptr());

        assert_eq!(vars, [0.0, 12.0, 4.0, 8.0]);
        assert!(take_native_runtime_error().is_none());
    }

    #[test]
    fn generated_assignment_pass_zero_based_indexed_assignment_skips_lower_subtract() {
        let assignments = [NativeAssignment::Indexed {
            base: 0,
            len: 4,
            lower: 0,
            index: native_program(EntryKind::Assignment, vec![Instruction::PushParam(0)], 0),
            value: native_program(EntryKind::Assignment, vec![Instruction::PushParam(1)], 0),
        }];
        let bytes = compile_assignment_pass_function(&assignments)
            .expect("compile zero-based indexed assignment pass");
        assert!(
            !contains_bytes(&bytes, &sub_r64_imm32_bytes(Gpr::R10, 0)),
            "zero-based indexed write fast path should not emit a no-op lower-bound subtract"
        );
        assert!(
            contains_bytes(&bytes, &cmp_r64_imm32_bytes(Gpr::R10, 4)),
            "zero-based indexed write fast path should still bounds-check the normalized index"
        );

        let memory =
            ExecutableMemory::allocate(&bytes).expect("allocate zero-based indexed assignment");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *mut f64) = unsafe { std::mem::transmute(entry) };

        let params = [2.49_f64, 13.0];
        let mut vars = [2.0_f64, 4.0, 8.0, 16.0];
        let ctx = eval_context(&params, &[], &[], &[]);
        clear_native_runtime_error();

        f(&ctx, vars.as_mut_ptr());

        assert_eq!(vars, [2.0, 4.0, 13.0, 16.0]);
        assert!(take_native_runtime_error().is_none());
    }

    #[test]
    fn generated_assignment_pass_keeps_huge_indexed_variable_ranges_on_helper_path() {
        let huge_len = (1_usize << 52) + 1;
        let assignments = [NativeAssignment::Indexed {
            base: 0,
            len: huge_len,
            lower: 0,
            index: native_program(EntryKind::Assignment, vec![Instruction::PushParam(0)], 0),
            value: native_program(EntryKind::Assignment, vec![Instruction::PushConst(11.0)], 0),
        }];
        assert!(
            assignment_uses_helper_calls(&assignments[0]),
            "huge indexed ranges must keep helper-backed continuation semantics"
        );
        let bytes = compile_assignment_pass_function(&assignments)
            .expect("compile huge indexed range assignment pass");

        assert!(
            bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "huge indexed ranges must keep the helper-call prologue"
        );
    }

    #[test]
    fn generated_assignment_pass_hard_fails_indexed_assignment_bounds_errors() {
        let assignments = [
            NativeAssignment::Indexed {
                base: 1,
                len: 3,
                lower: 1,
                index: native_program(EntryKind::Assignment, vec![Instruction::PushConst(4.0)], 0),
                value: native_program(EntryKind::Assignment, vec![Instruction::PushConst(11.0)], 0),
            },
            NativeAssignment::Direct {
                var_index: 0,
                program: native_program(
                    EntryKind::Assignment,
                    vec![Instruction::PushConst(123.0)],
                    0,
                ),
            },
        ];
        let bytes = compile_assignment_pass_function(&assignments)
            .expect("compile indexed assignment pass");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate indexed assignment pass");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *mut f64) = unsafe { std::mem::transmute(entry) };

        let mut vars = [0.0_f64, 2.0, 4.0, 8.0];
        let ctx = eval_context(&[], &[], &[], &[]);
        clear_native_runtime_error();

        f(&ctx, vars.as_mut_ptr());

        assert_eq!(vars, [0.0, 2.0, 4.0, 8.0]);
        let error =
            take_native_runtime_error().expect("out-of-range native indexed write must hard-fail");
        assert!(
            error.contains("array index 4 outside declared bounds [1:3]"),
            "error must preserve array bounds diagnostic, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    #[test]
    fn generated_assignment_pass_executes_nested_loops_with_indexed_body() {
        let assignments = [
            NativeAssignment::Direct {
                var_index: 0,
                program: native_program(
                    EntryKind::Assignment,
                    vec![Instruction::PushConst(0.0)],
                    0,
                ),
            },
            NativeAssignment::Direct {
                var_index: 2,
                program: native_program(
                    EntryKind::Assignment,
                    vec![Instruction::PushConst(0.0)],
                    0,
                ),
            },
            NativeAssignment::Loop {
                condition: native_program(
                    EntryKind::Assignment,
                    vec![
                        Instruction::PushVariable(0),
                        Instruction::PushConst(2.0),
                        Instruction::Lt,
                    ],
                    0,
                ),
                body: vec![
                    NativeAssignment::Direct {
                        var_index: 1,
                        program: native_program(
                            EntryKind::Assignment,
                            vec![Instruction::PushConst(0.0)],
                            0,
                        ),
                    },
                    NativeAssignment::Loop {
                        condition: native_program(
                            EntryKind::Assignment,
                            vec![
                                Instruction::PushVariable(1),
                                Instruction::PushConst(2.0),
                                Instruction::Lt,
                            ],
                            0,
                        ),
                        body: vec![
                            NativeAssignment::Indexed {
                                base: 3,
                                len: 2,
                                lower: 1,
                                index: native_program(
                                    EntryKind::Assignment,
                                    vec![
                                        Instruction::PushVariable(1),
                                        Instruction::PushConst(1.0),
                                        Instruction::Add,
                                    ],
                                    0,
                                ),
                                value: native_program(
                                    EntryKind::Assignment,
                                    vec![
                                        Instruction::PushVariable(0),
                                        Instruction::PushConst(10.0),
                                        Instruction::Mul,
                                        Instruction::PushVariable(1),
                                        Instruction::Add,
                                    ],
                                    0,
                                ),
                            },
                            NativeAssignment::Direct {
                                var_index: 2,
                                program: native_program(
                                    EntryKind::Assignment,
                                    vec![
                                        Instruction::PushVariable(2),
                                        Instruction::PushVariable(1),
                                        Instruction::PushConst(1.0),
                                        Instruction::Add,
                                        Instruction::PushVariableDyn {
                                            base: 3,
                                            len: 2,
                                            lower: 1,
                                        },
                                        Instruction::Add,
                                    ],
                                    0,
                                ),
                            },
                            NativeAssignment::Direct {
                                var_index: 1,
                                program: native_program(
                                    EntryKind::Assignment,
                                    vec![
                                        Instruction::PushVariable(1),
                                        Instruction::PushConst(1.0),
                                        Instruction::Add,
                                    ],
                                    0,
                                ),
                            },
                        ],
                    },
                    NativeAssignment::Direct {
                        var_index: 0,
                        program: native_program(
                            EntryKind::Assignment,
                            vec![
                                Instruction::PushVariable(0),
                                Instruction::PushConst(1.0),
                                Instruction::Add,
                            ],
                            0,
                        ),
                    },
                ],
            },
        ];
        let bytes =
            compile_assignment_pass_function(&assignments).expect("compile loop assignment pass");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate loop assignment pass");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *mut f64) = unsafe { std::mem::transmute(entry) };

        let mut vars = [0.0_f64; 5];
        let ctx = eval_context(&[], &[], &[], &[]);
        clear_native_runtime_error();

        f(&ctx, vars.as_mut_ptr());

        assert_eq!(vars, [2.0, 2.0, 22.0, 10.0, 11.0]);
        assert!(take_native_runtime_error().is_none());
    }

    #[test]
    fn generated_assignment_pass_helper_free_loop_omits_saved_arg_prologue_and_hard_fails_limit() {
        let assignments = [
            NativeAssignment::Loop {
                condition: native_program(
                    EntryKind::Assignment,
                    vec![Instruction::PushConst(1.0)],
                    0,
                ),
                body: Vec::new(),
            },
            NativeAssignment::Direct {
                var_index: 0,
                program: native_program(
                    EntryKind::Assignment,
                    vec![Instruction::PushConst(99.0)],
                    0,
                ),
            },
        ];
        assert!(
            !assignment_uses_helper_calls(&assignments[0]),
            "helper-free runtime loops should not force the helper-call prologue"
        );
        let bytes = compile_assignment_pass_function(&assignments)
            .expect("compile helper-free infinite loop assignment pass");
        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "helper-free runtime loops should not pay the saved-argument prologue"
        );
        assert!(
            contains_bytes(&bytes, &xor_r64_bytes(Gpr::R10, Gpr::R10)),
            "runtime loop counter should be zeroed with a zero idiom"
        );
        assert!(
            !contains_bytes(&bytes, &movabs_imm64_bytes(Gpr::R10, 0)),
            "runtime loop counter should not materialize zero as a 64-bit immediate"
        );
        let memory =
            ExecutableMemory::allocate(&bytes).expect("allocate infinite loop assignment pass");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *mut f64) = unsafe { std::mem::transmute(entry) };

        let mut vars = [0.0_f64];
        let ctx = eval_context(&[], &[], &[], &[]);
        clear_native_runtime_error();

        f(&ctx, vars.as_mut_ptr());

        assert_eq!(vars, [0.0]);
        let error = take_native_runtime_error().expect("loop limit must hard-fail");
        assert!(
            error.contains("native runtime loop iteration limit exceeded"),
            "error must preserve loop-limit diagnostic, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    #[test]
    fn generated_value_leaf_handles_ground_voltage_without_memory_load() {
        let program = native_program(
            EntryKind::StampValue,
            vec![Instruction::PushVoltage(usize::MAX, 0)],
            1,
        );
        let bytes = compile_value_function(&program).expect("compile ground voltage function");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate ground voltage leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let voltages = [6.25_f64];
        let ctx = eval_context(&[], &voltages, &[], &[]);

        assert_eq!(f(&ctx, std::ptr::null()), -6.25);
    }

    #[test]
    fn generated_value_leaf_handles_unified_internal_voltage_pairs() {
        let terminals = [10.0_f64, 4.0_f64];
        let internals = [7.0_f64, 3.0_f64];
        let cases = [
            ("terminal-internal", Instruction::PushVoltage(0, 2), 3.0_f64),
            (
                "ground-internal",
                Instruction::PushVoltage(usize::MAX, 2),
                -7.0_f64,
            ),
            (
                "internal-ground",
                Instruction::PushVoltage(3, usize::MAX),
                3.0_f64,
            ),
            ("internal-internal", Instruction::PushVoltage(2, 3), 4.0_f64),
        ];

        for (name, instruction, expected) in cases {
            let program =
                native_program_with_internals(EntryKind::StampValue, vec![instruction], 2, 2);
            let bytes =
                compile_value_function(&program).expect("compile unified internal voltage leaf");
            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate unified internal voltage leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &terminals, &internals, &[]);

            assert_eq!(f(&ctx, std::ptr::null()), expected, "{name}");
        }
    }

    #[test]
    fn generated_value_leaf_reuses_voltage_base_for_same_storage_pairs() {
        let cases = [
            (
                "terminal-terminal",
                native_program(
                    EntryKind::StampValue,
                    vec![Instruction::PushVoltage(0, 1)],
                    2,
                ),
                eval_context(&[], &[9.0, 4.0], &[], &[]),
                VOLTAGES_OFFSET,
                INTERNAL_VOLTAGES_OFFSET,
                5.0_f64,
            ),
            (
                "internal-internal",
                native_program_with_internals(
                    EntryKind::StampValue,
                    vec![Instruction::PushVoltage(2, 3)],
                    2,
                    2,
                ),
                eval_context(&[], &[0.0, 0.0], &[8.0, 3.0], &[]),
                INTERNAL_VOLTAGES_OFFSET,
                VOLTAGES_OFFSET,
                5.0_f64,
            ),
        ];

        for (name, program, ctx, reused_offset, unused_offset, expected) in cases {
            let bytes =
                compile_value_function(&program).expect("compile same-storage voltage leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "{name} should stay helper-free"
            );
            assert_eq!(
                count_bytes(&bytes, &context_pointer_load_bytes(reused_offset)),
                1,
                "{name} should load its voltage base pointer once"
            );
            assert_eq!(
                count_bytes(&bytes, &context_pointer_load_bytes(unused_offset)),
                0,
                "{name} should not touch the other voltage storage"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate same-storage voltage leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };

            assert_eq!(f(&ctx, std::ptr::null()).to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn generated_value_leaf_reuses_voltage_bases_across_pure_arithmetic() {
        let cases = [
            (
                "terminal",
                native_program(
                    EntryKind::StampValue,
                    vec![
                        Instruction::PushVoltage(0, 1),
                        Instruction::PushVoltage(2, 3),
                        Instruction::Add,
                    ],
                    4,
                ),
                eval_context(&[], &[9.0, 4.0, 12.0, 2.0], &[], &[]),
                VOLTAGES_OFFSET,
                INTERNAL_VOLTAGES_OFFSET,
                15.0_f64,
            ),
            (
                "internal",
                native_program_with_internals(
                    EntryKind::StampValue,
                    vec![
                        Instruction::PushVoltage(4, 5),
                        Instruction::PushVoltage(6, 7),
                        Instruction::Add,
                    ],
                    4,
                    4,
                ),
                eval_context(&[], &[0.0, 0.0, 0.0, 0.0], &[8.0, 3.0, 11.0, 2.0], &[]),
                INTERNAL_VOLTAGES_OFFSET,
                VOLTAGES_OFFSET,
                14.0_f64,
            ),
        ];

        for (name, program, ctx, reused_offset, unused_offset, expected) in cases {
            let bytes =
                compile_value_function(&program).expect("compile repeated voltage expression leaf");
            assert_eq!(
                count_bytes(&bytes, &context_pointer_load_bytes(reused_offset)),
                1,
                "{name} repeated voltage loads should materialize their base pointer once"
            );
            assert_eq!(
                count_bytes(&bytes, &context_pointer_load_bytes(unused_offset)),
                0,
                "{name} repeated voltage loads should not touch the other voltage storage"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate repeated voltage leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };

            assert_eq!(f(&ctx, std::ptr::null()).to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn generated_value_leaf_keeps_mixed_storage_voltage_base_loads_separate() {
        let program = native_program_with_internals(
            EntryKind::StampValue,
            vec![Instruction::PushVoltage(0, 2)],
            2,
            1,
        );

        let bytes = compile_value_function(&program).expect("compile mixed-storage voltage leaf");
        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "mixed-storage voltage load should stay helper-free"
        );
        assert_eq!(
            count_bytes(&bytes, &context_pointer_load_bytes(VOLTAGES_OFFSET)),
            1,
            "mixed-storage voltage load should read terminal voltage storage once"
        );
        assert_eq!(
            count_bytes(
                &bytes,
                &context_pointer_load_bytes(INTERNAL_VOLTAGES_OFFSET)
            ),
            1,
            "mixed-storage voltage load should read internal voltage storage once"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate mixed voltage leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[11.0, 0.0], &[2.5], &[]);

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 8.5_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_same_node_voltage_subtracts_in_register() {
        let cases = [
            (
                "terminal",
                native_program(
                    EntryKind::StampValue,
                    vec![Instruction::PushVoltage(0, 0)],
                    1,
                ),
                VOLTAGES_OFFSET,
                0,
                vec![
                    (6.25_f64, 0.0_f64),
                    (-0.0_f64, 0.0_f64),
                    (f64::INFINITY, f64::NAN),
                    (f64::from_bits(0x7ff8_0000_0000_0001), f64::NAN),
                ],
                false,
            ),
            (
                "internal",
                native_program_with_internals(
                    EntryKind::StampValue,
                    vec![Instruction::PushVoltage(2, 2)],
                    2,
                    1,
                ),
                INTERNAL_VOLTAGES_OFFSET,
                0,
                vec![
                    (-3.0_f64, 0.0_f64),
                    (f64::NEG_INFINITY, f64::NAN),
                    (f64::from_bits(0xfff8_0000_0000_0002), f64::NAN),
                ],
                true,
            ),
        ];

        for (name, program, reused_offset, index, values, uses_internal) in cases {
            let bytes = compile_value_function(&program).expect("compile same-node voltage leaf");
            assert_eq!(
                count_bytes(&bytes, &context_pointer_load_bytes(reused_offset)),
                1,
                "{name} same-node voltage should still load its base pointer once"
            );
            assert!(
                !contains_bytes(&bytes, &same_storage_voltage_memory_subtract_bytes(index)),
                "{name} same-node voltage should not reread the same slot for subtraction"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate same-node voltage leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };

            for (input, expected) in values {
                let got = if uses_internal {
                    let voltages = [0.0_f64, 0.0_f64];
                    let internal_voltages = [input];
                    let ctx = eval_context(&[], &voltages, &internal_voltages, &[]);
                    f(&ctx, std::ptr::null())
                } else {
                    let voltages = [input];
                    let ctx = eval_context(&[], &voltages, &[], &[]);
                    f(&ctx, std::ptr::null())
                };
                if expected.is_nan() {
                    assert!(got.is_nan(), "{name} {input:?}");
                } else {
                    assert_eq!(got.to_bits(), expected.to_bits(), "{name} {input:?}");
                }
            }
        }
    }

    #[test]
    fn generated_value_leaf_handles_internal_branch_sub_and_neg() {
        let program = native_program_with_internals(
            EntryKind::StampValue,
            vec![
                Instruction::PushInternalVoltage(1),
                Instruction::PushBranchCurrent(0),
                Instruction::Sub,
                Instruction::Neg,
            ],
            0,
            2,
        );
        let bytes = compile_value_function(&program).expect("compile negated internal expression");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate negated internal leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let internal_voltages = [0.0_f64, 9.0_f64];
        let branch_unknowns = [4.0_f64];
        let ctx = eval_context(&[], &[], &internal_voltages, &branch_unknowns);

        assert_eq!(f(&ctx, std::ptr::null()), -5.0);
    }

    #[test]
    fn generated_value_leaf_loads_terminal_pair_current_probe() {
        let available_current_pairs = [3];
        let program = native_program_with_available_current_pairs(
            EntryKind::StampValue,
            vec![
                Instruction::PushCurrent(1, 0),
                Instruction::PushConst(0.25),
                Instruction::Mul,
            ],
            2,
            &available_current_pairs,
        );
        let bytes = compile_value_function(&program).expect("compile current probe leaf");
        assert!(
            contains_bytes(&bytes, &guarded_slice_index_cmp_imm32_bytes(3)),
            "current probe load should compare storage length against an imm32 index"
        );
        assert!(
            !contains_bytes(&bytes, &guarded_slice_index_cmp_register_bytes(3)),
            "current probe load should not materialize the constant index in a GPR"
        );
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate current probe leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let branch_currents = [
            f64::NAN,
            4.0_f64,
            f64::NAN,
            -4.0_f64,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            f64::NAN,
        ];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.branch_currents = branch_currents.as_ptr();
        ctx.branch_currents_len = branch_currents.len();
        ctx.num_terminals = 2;

        assert_eq!(f(&ctx, std::ptr::null()), -1.0);

        ctx.branch_currents = std::ptr::null();
        clear_native_runtime_error();
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error().expect("missing current probe must hard-fail");
        assert_current_probe_error(&error);

        ctx.branch_currents = branch_currents.as_ptr();
        ctx.branch_currents_len = 3;
        clear_native_runtime_error();
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error().expect("out-of-range current probe must hard-fail");
        assert_current_probe_error(&error);
    }

    #[test]
    fn generated_value_leaf_loads_terminal_to_ground_current_probe() {
        let available_current_pairs = [2];
        let program = native_program_with_available_current_pairs(
            EntryKind::StampValue,
            vec![
                Instruction::PushCurrent(0, usize::MAX),
                Instruction::PushConst(0.25),
                Instruction::Mul,
            ],
            2,
            &available_current_pairs,
        );
        let bytes = compile_value_function(&program).expect("compile current probe leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate current probe leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let branch_currents = [
            f64::NAN,
            f64::NAN,
            8.0_f64,
            f64::NAN,
            f64::NAN,
            f64::NAN,
            -8.0_f64,
            f64::NAN,
            f64::NAN,
        ];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.branch_currents = branch_currents.as_ptr();
        ctx.branch_currents_len = branch_currents.len();
        ctx.num_terminals = 2;

        assert_eq!(f(&ctx, std::ptr::null()), 2.0);
    }

    #[test]
    fn generated_value_leaf_reuses_current_base_for_adjacent_current_loads() {
        let program = NativeProgram::from_ops_for_test(
            vec![
                NativeOp::LoadCurrent(1),
                NativeOp::LoadCurrent(2),
                NativeOp::Add,
            ],
            2,
            vec![1, 2],
            Vec::new(),
        );
        let bytes = compile_value_function(&program).expect("compile adjacent current leaf");
        assert_eq!(
            count_bytes(&bytes, &context_pointer_load_bytes(BRANCH_CURRENTS_OFFSET)),
            1,
            "adjacent guarded current loads should materialize the current base pointer once"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate adjacent current leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let branch_currents = [f64::NAN, 4.0_f64, 6.5_f64];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.branch_currents = branch_currents.as_ptr();
        ctx.branch_currents_len = branch_currents.len();

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 10.5_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_loads_prior_current_probe() {
        let program = NativeProgram::from_ops_for_test(
            vec![NativeOp::LoadPriorCurrent(1)],
            1,
            Vec::new(),
            vec![1],
        );
        let bytes = compile_value_function(&program).expect("compile prior current leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate prior current leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let currents = [2.0_f64, 7.0_f64];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.currents = currents.as_ptr();
        ctx.currents_len = currents.len();

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 7.0_f64.to_bits());

        ctx.currents = std::ptr::null();
        clear_native_runtime_error();
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error().expect("missing prior current must hard-fail");
        assert_prior_current_error(&error);

        ctx.currents = currents.as_ptr();
        ctx.currents_len = 1;
        clear_native_runtime_error();
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error().expect("short prior current must hard-fail");
        assert_prior_current_error(&error);
    }

    #[test]
    fn generated_value_leaf_loads_param_given_flag() {
        let program = native_program(
            EntryKind::StampValue,
            vec![Instruction::PushParamGiven(1)],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile param_given leaf");
        assert!(
            contains_bytes(&bytes, &guarded_slice_index_cmp_imm32_bytes(1)),
            "param_given load should compare storage length against an imm32 index"
        );
        assert!(
            !contains_bytes(&bytes, &guarded_slice_index_cmp_register_bytes(1)),
            "param_given load should not materialize the constant index in a GPR"
        );
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate param_given leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let param_given = [0_u8, 1_u8];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.param_given = param_given.as_ptr();
        ctx.param_given_len = param_given.len();

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 1.0_f64.to_bits());

        ctx.param_given = std::ptr::null();
        clear_native_runtime_error();
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error().expect("missing param_given must hard-fail");
        assert_param_given_error(&error);

        ctx.param_given = param_given.as_ptr();
        ctx.param_given_len = 1;
        clear_native_runtime_error();
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error().expect("short param_given must hard-fail");
        assert_param_given_error(&error);
    }

    #[test]
    fn generated_value_leaf_loads_port_connected_flag() {
        let program = native_program(
            EntryKind::StampValue,
            vec![Instruction::PushPortConnected(1)],
            2,
        );
        let bytes = compile_value_function(&program).expect("compile port_connected leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate port_connected leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let port_connected = [0_u8, 1_u8];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.port_connected = port_connected.as_ptr();
        ctx.port_connected_len = port_connected.len();

        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 1.0_f64.to_bits());

        ctx.port_connected = std::ptr::null();
        clear_native_runtime_error();
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error().expect("missing port_connected must hard-fail");
        assert_port_connected_error(&error);

        ctx.port_connected = port_connected.as_ptr();
        ctx.port_connected_len = 1;
        clear_native_runtime_error();
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error().expect("short port_connected must hard-fail");
        assert_port_connected_error(&error);
    }

    #[test]
    fn generated_value_leaf_computes_thermal_voltage_from_context_temperature() {
        let program = native_program(EntryKind::StampValue, vec![Instruction::PushVt], 0);
        let bytes = compile_value_function(&program).expect("compile thermal voltage leaf");
        assert!(
            contains_bytes(&bytes, &THERMAL_VOLTAGE_PER_K.to_le_bytes()),
            "thermal voltage should use one precomputed k/q literal"
        );
        assert!(
            !contains_bytes(&bytes, &K_BOLTZMANN.to_le_bytes()),
            "thermal voltage should not emit a separate Boltzmann literal"
        );
        assert!(
            !contains_bytes(&bytes, &Q_ELECTRON.to_le_bytes()),
            "thermal voltage should not emit a separate electron-charge literal"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate thermal voltage leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.temperature = 315.0;

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            thermal_voltage(315.0).to_bits()
        );
    }

    #[test]
    fn generated_value_leaf_computes_ddt_state_and_records_operand() {
        let program = native_program(
            EntryKind::StampValue,
            vec![Instruction::PushVariable(0), Instruction::DdtState(1)],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile ddt state leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate ddt state leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let vars = [2.0_f64];

        let previous_state = [0.0_f64, 1.5_f64];
        let mut state_values = [0.0_f64, 0.0_f64];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.timestep = 0.25;
        ctx.state_prev = previous_state.as_ptr();
        ctx.state_prev_len = previous_state.len();
        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_values_len = state_values.len();

        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 2.0_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 2.0_f64.to_bits());

        state_values[1] = f64::NAN;
        ctx.timestep = 0.0;
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 2.0_f64.to_bits());

        state_values[1] = f64::NAN;
        ctx.timestep = 0.25;
        ctx.state_prev = std::ptr::null();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 2.0_f64.to_bits());

        ctx.state_values = std::ptr::null_mut();
        clear_native_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error().expect("missing ddt state must hard-fail");
        assert_missing_state_storage_error(&error);

        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_values_len = 1;
        clear_native_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error().expect("short ddt state must hard-fail");
        assert_state_storage_bounds_error(&error);

        ctx.state_values_len = state_values.len();
        ctx.state_prev = previous_state.as_ptr();
        ctx.state_prev_len = 1;
        clear_native_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error().expect("short ddt prior state must hard-fail");
        assert_prior_state_storage_bounds_error(&error);
    }

    #[test]
    fn generated_value_leaf_computes_ddt_jacobian_from_timestep() {
        let program = native_program(
            EntryKind::Jacobian,
            vec![Instruction::PushVariable(0), Instruction::DdtJacobian],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile ddt jacobian leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate ddt jacobian leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let vars = [2.0_f64];
        let mut ctx = eval_context(&[], &[], &[], &[]);

        ctx.timestep = 0.25;
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 8.0_f64.to_bits());

        ctx.timestep = 0.0;
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_computes_idt_state_and_records_integral() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushVariable(0),
                Instruction::PushConst(0.5),
                Instruction::IdtState(1),
            ],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile idt state leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate idt state leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let vars = [2.0_f64];

        let previous_state = [0.0_f64, 1.5_f64];
        let mut state_values = [0.0_f64, 0.0_f64];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.timestep = 0.25;
        ctx.state_prev = previous_state.as_ptr();
        ctx.state_prev_len = previous_state.len();
        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_values_len = state_values.len();

        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 2.0_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 2.0_f64.to_bits());

        state_values[1] = f64::NAN;
        ctx.timestep = 0.0;
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.5_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 0.5_f64.to_bits());

        state_values[1] = f64::NAN;
        ctx.timestep = 1.0e-20;
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.5_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 0.5_f64.to_bits());

        state_values[1] = f64::NAN;
        ctx.timestep = 0.25;
        ctx.state_prev = std::ptr::null();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 1.0_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 1.0_f64.to_bits());

        state_values[1] = f64::NAN;
        ctx.timestep = -0.25;
        ctx.state_prev = previous_state.as_ptr();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 1.0_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 1.0_f64.to_bits());

        state_values[1] = f64::NAN;
        ctx.timestep = f64::NAN;
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.5_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 0.5_f64.to_bits());

        ctx.state_values = std::ptr::null_mut();
        ctx.timestep = 0.0;
        clear_native_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error().expect("missing idt state must hard-fail");
        assert_missing_state_storage_error(&error);

        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_values_len = 1;
        clear_native_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error().expect("short idt state must hard-fail");
        assert_state_storage_bounds_error(&error);

        ctx.state_values_len = state_values.len();
        ctx.timestep = 0.25;
        ctx.state_prev = previous_state.as_ptr();
        ctx.state_prev_len = 1;
        clear_native_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error().expect("short idt prior state must hard-fail");
        assert_prior_state_storage_bounds_error(&error);
    }

    #[test]
    fn generated_value_leaf_computes_idt_jacobian_from_timestep() {
        let program = native_program(
            EntryKind::Jacobian,
            vec![Instruction::PushVariable(0), Instruction::IdtJacobian],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile idt jacobian leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate idt jacobian leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let vars = [2.0_f64];
        let mut ctx = eval_context(&[], &[], &[], &[]);

        ctx.timestep = 0.25;
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.5_f64.to_bits());

        ctx.timestep = 0.0;
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());

        ctx.timestep = 1.0e-20;
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());

        ctx.timestep = -0.25;
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), (-0.5_f64).to_bits());

        ctx.timestep = f64::NAN;
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_computes_idtmod_state_and_records_wrapped_integral() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushVariable(0),
                Instruction::PushConst(0.5),
                Instruction::PushConst(1.0),
                Instruction::PushConst(0.25),
                Instruction::IdtModState(1),
            ],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile idtmod state leaf");
        for (slot, register) in [
            (0, Xmm::Xmm0),
            (1, Xmm::Xmm1),
            (2, Xmm::Xmm2),
            (3, Xmm::Xmm3),
        ] {
            assert!(
                !contains_bytes(&bytes, &call_frame_spill_bytes(slot, register)),
                "idtmod helper should not spill consumed operand slot {slot}"
            );
        }
        assert!(
            !contains_bytes(&bytes, &call_frame_load_bytes(Xmm::Xmm0, 0)),
            "idtmod helper should pass value directly instead of reloading it from the spill frame"
        );
        assert!(
            !contains_bytes(&bytes, &call_frame_load_bytes(Xmm::Xmm1, 2)),
            "idtmod helper should pass modulus directly instead of reloading it from the spill frame"
        );
        assert!(
            !contains_bytes(&bytes, &call_frame_load_bytes(Xmm::Xmm2, 3)),
            "idtmod helper should pass offset directly instead of reloading it from the spill frame"
        );
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate idtmod state leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let vars = [2.0_f64];

        let previous_state = [0.0_f64, 0.9_f64];
        let mut state_values = [0.0_f64, 0.0_f64];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.timestep = 0.25;
        ctx.state_prev = previous_state.as_ptr();
        ctx.state_prev_len = previous_state.len();
        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_values_len = state_values.len();

        let value = f(&ctx, vars.as_ptr());
        assert!((value - 0.4).abs() < 1.0e-12, "value: {value}");
        assert!(
            (state_values[1] - 0.4).abs() < 1.0e-12,
            "state: {state_values:?}"
        );

        state_values[1] = f64::NAN;
        ctx.timestep = 0.0;
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.5_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 0.5_f64.to_bits());

        state_values[1] = f64::NAN;
        ctx.timestep = 1.0e-20;
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.5_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 0.5_f64.to_bits());

        state_values[1] = f64::NAN;
        ctx.timestep = f64::NAN;
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.5_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 0.5_f64.to_bits());

        state_values[1] = f64::NAN;
        ctx.timestep = 0.25;
        ctx.state_prev = std::ptr::null();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 1.0_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 1.0_f64.to_bits());

        let unwrapped_program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushVariable(0),
                Instruction::PushConst(0.5),
                Instruction::PushConst(0.0),
                Instruction::PushConst(0.25),
                Instruction::IdtModState(1),
            ],
            0,
        );
        let unwrapped_bytes =
            compile_value_function(&unwrapped_program).expect("compile idtmod unwrapped leaf");
        let unwrapped_memory =
            ExecutableMemory::allocate(&unwrapped_bytes).expect("allocate idtmod unwrapped leaf");
        let unwrapped_entry = unwrapped_memory
            .ptr_at(0)
            .expect("entry point inside unwrapped image");
        let unwrapped: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(unwrapped_entry) };

        state_values[1] = f64::NAN;
        ctx.timestep = 0.25;
        ctx.state_prev = previous_state.as_ptr();
        let unwrapped_value = unwrapped(&ctx, vars.as_ptr());
        assert!(
            (unwrapped_value - 1.4).abs() < 1.0e-12,
            "unwrapped value: {unwrapped_value}"
        );
        assert!(
            (state_values[1] - 1.4).abs() < 1.0e-12,
            "unwrapped state: {state_values:?}"
        );

        ctx.state_values = std::ptr::null_mut();
        ctx.timestep = 0.0;
        clear_native_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error().expect("missing idtmod state must hard-fail");
        assert_missing_state_storage_error(&error);

        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_values_len = 1;
        clear_native_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error().expect("short idtmod state must hard-fail");
        assert_state_storage_bounds_error(&error);

        ctx.state_values_len = state_values.len();
        ctx.timestep = 0.25;
        ctx.state_prev = previous_state.as_ptr();
        ctx.state_prev_len = 1;
        clear_native_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error().expect("short idtmod prior state must hard-fail");
        assert_prior_state_storage_bounds_error(&error);
    }

    #[test]
    fn generated_value_leaf_omits_stateful_scratch_when_full_stack_occurs_after_stateful_op() {
        let prefix = variable_prefix(1, XMM_STACK.len() - 1);
        let mut instructions = vec![Instruction::PushVariable(0), Instruction::DdtState(1)];
        instructions.extend(prefix.clone());
        instructions.extend(add_reductions(prefix.len()));
        let program = native_program(EntryKind::StampValue, instructions, 0);
        assert_eq!(program.max_stack_depth(), XMM_STACK.len());
        let bytes = compile_value_function(&program).expect("compile spare-depth stateful leaf");
        assert!(
            !contains_bytes(&bytes, &sub_rsp_bytes(STATEFUL_SCRATCH_FRAME_BYTES)),
            "stateful op with spare XMM capacity should not reserve a scratch frame just because a later expression reaches full depth"
        );

        let memory =
            ExecutableMemory::allocate(&bytes).expect("allocate spare-depth stateful leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let vars = vars_with_prefix(2.0, prefix.len());
        let previous_state = [0.0_f64, 1.5_f64];
        let mut state_values = [0.0_f64, 0.0_f64];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.timestep = 0.25;
        ctx.state_prev = previous_state.as_ptr();
        ctx.state_prev_len = previous_state.len();
        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_values_len = state_values.len();

        assert_eq!(
            f(&ctx, vars.as_ptr()).to_bits(),
            (2.0 + constant_prefix_sum(prefix.len())).to_bits()
        );
        assert_eq!(state_values[1].to_bits(), 2.0_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_keeps_full_stack_stateful_ops_on_scratch_frame() {
        let run = |program: NativeProgram, vars: &[f64], expected: f64, state_expected: f64| {
            assert_eq!(program.max_stack_depth(), XMM_STACK.len());
            let bytes = compile_value_function(&program).expect("compile full-stack stateful leaf");
            let frame_bytes = super::align_local_frame(
                STATEFUL_SCRATCH_FRAME_BYTES
                    + super::callee_saved_xmm_frame_bytes(super::callee_saved_xmm_count_for_depth(
                        XMM_STACK.len(),
                    )),
            );
            assert!(
                contains_bytes(&bytes, &sub_rsp_bytes(frame_bytes)),
                "full-stack stateful leaf should reserve a local scratch frame"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate full-stack stateful leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };

            let previous_state = [0.0_f64, 1.5_f64];
            let mut state_values = [0.0_f64, 0.0_f64];
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.timestep = 0.25;
            ctx.state_prev = previous_state.as_ptr();
            ctx.state_prev_len = previous_state.len();
            ctx.state_values = state_values.as_mut_ptr();
            ctx.state_values_len = state_values.len();

            assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), expected.to_bits());
            assert_eq!(state_values[1].to_bits(), state_expected.to_bits());
        };

        let ddt_prefix = variable_prefix(1, XMM_STACK.len() - 1);
        let mut ddt_instructions = ddt_prefix.clone();
        ddt_instructions.extend([Instruction::PushVariable(0), Instruction::DdtState(1)]);
        ddt_instructions.extend(add_reductions(ddt_prefix.len()));
        let ddt_vars = vars_with_prefix(2.0, ddt_prefix.len());
        run(
            native_program(EntryKind::StampValue, ddt_instructions, 0),
            &ddt_vars,
            constant_prefix_sum(ddt_prefix.len()) + 2.0,
            2.0,
        );

        let idt_prefix = variable_prefix(1, XMM_STACK.len() - 2);
        let mut idt_instructions = idt_prefix.clone();
        idt_instructions.extend([
            Instruction::PushVariable(0),
            Instruction::PushConst(0.5),
            Instruction::IdtState(1),
        ]);
        idt_instructions.extend(add_reductions(idt_prefix.len()));
        let idt_vars = vars_with_prefix(2.0, idt_prefix.len());
        run(
            native_program(EntryKind::StampValue, idt_instructions, 0),
            &idt_vars,
            constant_prefix_sum(idt_prefix.len()) + 2.0,
            2.0,
        );

        let idtmod_prefix = variable_prefix(1, XMM_STACK.len() - 4);
        let mut idtmod_instructions = idtmod_prefix.clone();
        idtmod_instructions.extend([
            Instruction::PushVariable(0),
            Instruction::PushConst(0.5),
            Instruction::PushConst(1.0),
            Instruction::PushConst(0.25),
            Instruction::IdtModState(1),
        ]);
        idtmod_instructions.extend(add_reductions(idtmod_prefix.len()));
        let idtmod_vars = vars_with_prefix(2.0, idtmod_prefix.len());
        run(
            native_program(EntryKind::StampValue, idtmod_instructions, 0),
            &idtmod_vars,
            constant_prefix_sum(idtmod_prefix.len()) + 1.0,
            1.0,
        );

        let jacobian_prefix = variable_prefix(1, XMM_STACK.len() - 1);
        let mut ddt_jacobian = jacobian_prefix.clone();
        ddt_jacobian.extend([Instruction::PushVariable(0), Instruction::DdtJacobian]);
        ddt_jacobian.extend(add_reductions(jacobian_prefix.len()));
        let mut idt_jacobian = jacobian_prefix.clone();
        idt_jacobian.extend([Instruction::PushVariable(0), Instruction::IdtJacobian]);
        idt_jacobian.extend(add_reductions(jacobian_prefix.len()));
        let jacobian_cases = [
            (
                constant_prefix_sum(jacobian_prefix.len()) + 8.0,
                ddt_jacobian,
            ),
            (
                constant_prefix_sum(jacobian_prefix.len()) + 0.5,
                idt_jacobian,
            ),
        ];
        for (expected, instructions) in jacobian_cases {
            let program = native_program(EntryKind::Jacobian, instructions, 0);
            assert_eq!(program.max_stack_depth(), XMM_STACK.len());
            let bytes =
                compile_value_function(&program).expect("compile full-stack stateful jacobian");
            let frame_bytes = super::align_local_frame(
                STATEFUL_SCRATCH_FRAME_BYTES
                    + super::callee_saved_xmm_frame_bytes(super::callee_saved_xmm_count_for_depth(
                        XMM_STACK.len(),
                    )),
            );
            assert!(
                contains_bytes(&bytes, &sub_rsp_bytes(frame_bytes)),
                "full-stack stateful jacobian should reserve a local scratch frame"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate full-stack jacobian leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.timestep = 0.25;

            let vars = vars_with_prefix(2.0, jacobian_prefix.len());
            assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn generated_value_leaf_preserves_prefix_across_idtmod_helper_call() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushConst(10.0),
                Instruction::PushVariable(0),
                Instruction::PushConst(0.5),
                Instruction::PushConst(1.0),
                Instruction::PushConst(0.25),
                Instruction::IdtModState(1),
                Instruction::Add,
            ],
            0,
        );
        assert_eq!(program.max_stack_depth(), 5);
        let bytes = compile_value_function(&program).expect("compile prefixed idtmod state leaf");
        let frame_bytes = call_frame_bytes(1);
        let old_fixed_frame_bytes = old_fixed_call_frame_bytes();
        assert!(
            contains_bytes(&bytes, &sub_rsp_bytes(frame_bytes)),
            "prefixed idtmod helper should reserve only the lower live prefix slot"
        );
        assert!(
            contains_bytes(&bytes, &add_rsp_bytes(frame_bytes)),
            "prefixed idtmod helper should release only the lower live prefix slot"
        );
        assert!(
            !contains_bytes(&bytes, &sub_rsp_bytes(old_fixed_frame_bytes)),
            "prefixed idtmod helper should not reserve the old maximum spill frame"
        );
        assert!(
            !contains_bytes(&bytes, &add_rsp_bytes(old_fixed_frame_bytes)),
            "prefixed idtmod helper should not release the old maximum spill frame"
        );
        assert_eq!(
            count_bytes(&bytes, &call_frame_spill_bytes(0, Xmm::Xmm0)),
            2,
            "idtmod should spill the lower live prefix once per helper-call path"
        );
        for (slot, register) in [
            (1, Xmm::Xmm1),
            (2, Xmm::Xmm2),
            (3, Xmm::Xmm3),
            (4, Xmm::Xmm4),
        ] {
            assert!(
                !contains_bytes(&bytes, &call_frame_spill_bytes(slot, register)),
                "idtmod helper should not spill consumed operand slot {slot}"
            );
        }
        assert!(
            !contains_bytes(&bytes, &call_frame_load_bytes(Xmm::Xmm0, 1)),
            "idtmod helper should pass value directly instead of reloading it from the spill frame"
        );
        assert!(
            !contains_bytes(&bytes, &call_frame_load_bytes(Xmm::Xmm1, 3)),
            "idtmod helper should pass modulus directly instead of reloading it from the spill frame"
        );
        assert!(
            !contains_bytes(&bytes, &call_frame_load_bytes(Xmm::Xmm2, 4)),
            "idtmod helper should pass offset directly instead of reloading it from the spill frame"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate prefixed idtmod leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let vars = [2.0_f64];

        let previous_state = [0.0_f64, 0.9_f64];
        let mut state_values = [0.0_f64, 0.0_f64];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.timestep = 0.25;
        ctx.state_prev = previous_state.as_ptr();
        ctx.state_prev_len = previous_state.len();
        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_values_len = state_values.len();

        let value = f(&ctx, vars.as_ptr());
        assert!((value - 10.4).abs() < 1.0e-12, "value: {value}");
        assert!(
            (state_values[1] - 0.4).abs() < 1.0e-12,
            "state: {state_values:?}"
        );

        state_values[1] = f64::NAN;
        ctx.timestep = 0.0;
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 10.5_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 0.5_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_computes_limit_state_and_records_iteration_value() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushVariable(0),
                Instruction::PushConst(0.5),
                Instruction::LimitState(1),
            ],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile limit state leaf");
        assert!(
            !contains_bytes(&bytes, &stack_spill_store_bytes(Xmm::Xmm1)),
            "limit state with a spare XMM register should not spill the positive step to the stack"
        );
        assert!(
            !contains_bytes(&bytes, &stack_spill_minsd_bytes(Xmm::Xmm0)),
            "limit state with a spare XMM register should clamp from the XMM scratch copy"
        );
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate limit state leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let mut state_values = [0.0_f64, 0.0_f64];
        let mut state_initialized = [0_u8, 0_u8];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_values_len = state_values.len();
        ctx.state_initialized = state_initialized.as_mut_ptr();
        ctx.state_initialized_len = state_initialized.len();

        let vars = [10.0_f64];
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 10.0_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 10.0_f64.to_bits());
        assert_eq!(state_initialized[1], 1);

        let vars = [11.0_f64];
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 10.5_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 10.5_f64.to_bits());
        assert_eq!(state_initialized[1], 1);

        let vars = [0.0_f64];
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 10.0_f64.to_bits());
        assert_eq!(state_values[1].to_bits(), 10.0_f64.to_bits());

        state_values[1] = 0.0;
        state_initialized[1] = 1;
        assert_eq!(state_initialized[1], 1);
        let vars = [10.0_f64];
        assert_eq!(
            f(&ctx, vars.as_ptr()).to_bits(),
            0.5_f64.to_bits(),
            "initialized zero state must clamp instead of behaving like first evaluation"
        );
        assert_eq!(state_values[1].to_bits(), 0.5_f64.to_bits());

        ctx.state_initialized_len = 1;
        state_values[1] = 10.0;
        state_initialized[1] = 1;
        assert_eq!(state_initialized[1], 1);
        let vars = [20.0_f64];
        clear_native_runtime_error();
        assert_eq!(
            f(&ctx, vars.as_ptr()).to_bits(),
            0.0_f64.to_bits(),
            "native limit must return through the hard-fail path"
        );
        let error =
            take_native_runtime_error().expect("out-of-range limit metadata must hard-fail");
        assert!(
            error.contains("state index outside initialization flag storage"),
            "error must identify invalid limit metadata, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
        assert_eq!(
            state_values[1].to_bits(),
            10.0_f64.to_bits(),
            "out-of-range state flag metadata must leave native state untouched"
        );

        ctx.state_initialized_len = state_initialized.len();
        ctx.state_values_len = 1;
        clear_native_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error().expect("short limit state must hard-fail");
        assert_limit_state_storage_bounds_error(&error);

        ctx.state_values_len = state_values.len();
        ctx.state_values = std::ptr::null_mut();
        clear_native_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error = take_native_runtime_error().expect("missing limit state must hard-fail");
        assert!(
            error.contains("missing state storage"),
            "error must identify missing limit state storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );

        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_initialized = std::ptr::null_mut();
        clear_native_runtime_error();
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
        let error =
            take_native_runtime_error().expect("missing limit initialization flags must hard-fail");
        assert!(
            error.contains("missing initialization flag storage"),
            "error must identify missing limit initialization storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );

        ctx.state_initialized = state_initialized.as_mut_ptr();
        state_values[1] = 1.0;
        state_initialized[1] = 1;
        assert_eq!(state_initialized[1], 1);
        let vars = [f64::NAN];
        let result = f(&ctx, vars.as_ptr());
        assert!(result.is_nan(), "initialized limit must propagate NaN");
        assert!(
            state_values[1].is_nan(),
            "native state should record propagated NaN"
        );
    }

    #[test]
    fn generated_value_leaf_keeps_full_stack_limit_state_on_spill_path() {
        let prefix = constant_prefix(XMM_STACK.len() - 2);
        let mut instructions = prefix.clone();
        instructions.extend([
            Instruction::PushVariable(0),
            Instruction::PushConst(0.5),
            Instruction::LimitState(1),
        ]);
        instructions.extend(add_reductions(prefix.len()));
        let program = native_program(EntryKind::StampValue, instructions, 0);
        assert_eq!(program.max_stack_depth(), XMM_STACK.len());
        let bytes = compile_value_function(&program).expect("compile full-stack limit state leaf");
        assert!(
            contains_bytes(&bytes, &sub_rsp_bytes(WORD_BYTES as i32)),
            "full-stack limit state must keep the positive-step spill fallback"
        );
        assert!(
            contains_bytes(&bytes, &add_rsp_bytes(WORD_BYTES as i32)),
            "full-stack limit state must restore the positive-step spill fallback"
        );
        assert!(
            contains_bytes(
                &bytes,
                &stack_spill_store_bytes(XMM_STACK[XMM_STACK.len() - 1])
            ),
            "full-stack limit state must spill the positive step to the stack"
        );
        assert!(
            contains_bytes(
                &bytes,
                &stack_spill_minsd_bytes(XMM_STACK[XMM_STACK.len() - 2])
            ),
            "full-stack limit state must clamp from the spilled positive step"
        );

        let memory =
            ExecutableMemory::allocate(&bytes).expect("allocate full-stack limit state leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let mut state_values = [0.0_f64, 10.0_f64];
        let mut state_initialized = [0_u8, 1_u8];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.state_values = state_values.as_mut_ptr();
        ctx.state_values_len = state_values.len();
        ctx.state_initialized = state_initialized.as_mut_ptr();
        ctx.state_initialized_len = state_initialized.len();

        let vars = [11.0_f64];
        assert_eq!(
            f(&ctx, vars.as_ptr()).to_bits(),
            (constant_prefix_sum(prefix.len()) + 10.5).to_bits()
        );
        assert_eq!(state_values[1].to_bits(), 10.5_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_computes_sqrt_in_place() {
        let program = native_program(
            EntryKind::StampValue,
            vec![Instruction::PushTemperature, Instruction::Sqrt],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile sqrt leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate sqrt leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.temperature = 49.0;

        assert_eq!(f(&ctx, std::ptr::null()), 7.0);
    }

    #[test]
    fn generated_value_leaf_computes_abs_in_place() {
        let program = native_program(
            EntryKind::StampValue,
            vec![Instruction::PushTemperature, Instruction::Abs],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile abs leaf");
        assert!(
            contains_bytes(&bytes, &andpd_rip_prefix_bytes(Xmm::Xmm0)),
            "abs should clear the sign bit with a RIP-relative vector mask"
        );
        assert!(
            !contains_bytes(
                &bytes,
                &scalar_abs_integer_bit_clear_bytes(Xmm::Xmm0, Gpr::Rax)
            ),
            "abs should not roundtrip through a GPR"
        );
        let mask_offset =
            find_bytes(&bytes, &abs_value_mask_bytes()).expect("abs mask literal is present");
        assert_eq!(
            mask_offset % VECTOR_LITERAL_ALIGNMENT,
            0,
            "abs mask literal should be 16-byte aligned"
        );
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate abs leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let mut ctx = eval_context(&[], &[], &[], &[]);

        ctx.temperature = -7.5;
        assert_eq!(f(&ctx, std::ptr::null()), 7.5);
        ctx.temperature = -0.0;
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 0.0_f64.to_bits());
        ctx.temperature = f64::from_bits(0xfff8_0000_0000_0001);
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 0x7ff8_0000_0000_0001);
    }

    #[test]
    fn generated_value_leaf_folds_constant_unary_ops_to_literals() {
        let cases = [
            (
                "neg negative literal",
                Instruction::Neg,
                (-7.5_f64).to_bits(),
                7.5_f64.to_bits(),
            ),
            (
                "neg negative zero",
                Instruction::Neg,
                (-0.0_f64).to_bits(),
                0.0_f64.to_bits(),
            ),
            (
                "neg positive nan",
                Instruction::Neg,
                0x7ff8_0000_0000_0001,
                0xfff8_0000_0000_0001,
            ),
            (
                "abs negative literal",
                Instruction::Abs,
                (-7.5_f64).to_bits(),
                7.5_f64.to_bits(),
            ),
            (
                "abs negative zero",
                Instruction::Abs,
                (-0.0_f64).to_bits(),
                0.0_f64.to_bits(),
            ),
            (
                "abs negative nan",
                Instruction::Abs,
                0xfff8_0000_0000_0001,
                0x7ff8_0000_0000_0001,
            ),
        ];

        for (case, instruction, input_bits, expected_bits) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(f64::from_bits(input_bits)),
                    instruction,
                ],
                0,
            );

            match program.ops() {
                [NativeOp::Const(value)] => {
                    assert_eq!(value.to_bits(), expected_bits, "{case}");
                }
                ops => panic!("{case} lowered to unexpected ops: {ops:?}"),
            }

            let bytes = compile_value_function(&program).expect("compile folded unary leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "folded constant unary op should stay helper-free"
            );

            let memory = ExecutableMemory::allocate(&bytes).expect("allocate folded unary leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_eq!(f(&ctx, std::ptr::null()).to_bits(), expected_bits, "{case}");
        }
    }

    #[test]
    fn generated_value_leaf_computes_large_signal_noise_as_zero() {
        let cases = [
            (
                "white-standalone",
                vec![Instruction::PushConst(5.0), Instruction::WhiteNoise],
                0.0_f64,
            ),
            (
                "flicker-standalone",
                vec![
                    Instruction::PushConst(5.0),
                    Instruction::PushConst(1.0),
                    Instruction::FlickerNoise,
                ],
                0.0_f64,
            ),
            (
                "white-composed",
                vec![
                    Instruction::PushConst(2.0),
                    Instruction::PushConst(5.0),
                    Instruction::WhiteNoise,
                    Instruction::Add,
                ],
                2.0_f64,
            ),
            (
                "flicker-composed",
                vec![
                    Instruction::PushConst(2.0),
                    Instruction::PushConst(5.0),
                    Instruction::PushConst(1.0),
                    Instruction::FlickerNoise,
                    Instruction::Add,
                ],
                2.0_f64,
            ),
            (
                "flicker-overwrite-dead-register",
                vec![
                    Instruction::PushConst(7.0),
                    Instruction::PushConst(5.0),
                    Instruction::PushConst(1.0),
                    Instruction::FlickerNoise,
                    Instruction::PushConst(3.0),
                    Instruction::Add,
                    Instruction::Add,
                ],
                10.0_f64,
            ),
        ];

        for (name, instructions, expected) in cases {
            let program = native_program(EntryKind::StampValue, instructions, 0);
            let bytes = compile_value_function(&program).expect("compile noise leaf");
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate noise leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_calls_laplace_helper_and_preserves_stack() {
        let program = NativeProgram::from_bytecode(
            "x64-codegen-test",
            EntryKind::StampValue,
            &BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(2.0),
                    Instruction::PushConst(4.0),
                    Instruction::LaplaceState(0),
                    Instruction::Add,
                ],
            },
            NativeLoweringLimits::new(0, 0, 0, 0, 0).with_laplace_filter_count(1),
        )
        .expect("lower Laplace helper program");
        let bytes = compile_value_function(&program).expect("compile Laplace helper leaf");
        assert!(
            contains_bytes(
                &bytes,
                &xor_r64_bytes(
                    super::context_filter_id_arg_reg(),
                    super::context_filter_id_arg_reg()
                )
            ),
            "Laplace helper should zero filter ID 0 with a compact dependency-breaking xor"
        );
        assert!(
            !contains_bytes(
                &bytes,
                &mov_r32_imm32_bytes(super::context_filter_id_arg_reg(), 0)
            ),
            "Laplace helper should not materialize filter ID 0 with a wider imm32 move"
        );
        assert!(
            !contains_bytes(
                &bytes,
                &movabs_imm64_bytes(super::context_filter_id_arg_reg(), 0)
            ),
            "Laplace helper should not use movabs for small filter IDs"
        );
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate Laplace helper leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let mut filters = [StateSpaceFilter::from_transfer_function(
            &[1.0],
            &[1.0, 1.0],
        )];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.laplace_filters = filters.as_mut_ptr();
        ctx.laplace_filters_len = filters.len();

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            6.0_f64.to_bits(),
            "non-transient Laplace evaluation uses DC output"
        );

        ctx.analysis_type = 2;
        ctx.timestep = 0.5;
        let transient = f(&ctx, std::ptr::null());
        assert!(
            (transient - (2.0 + 4.0 / 3.0)).abs() < 1.0e-12,
            "transient Laplace value: {transient}"
        );
    }

    #[test]
    fn generated_value_leaf_calls_zi_helper_and_preserves_stack() {
        let program = NativeProgram::from_bytecode(
            "x64-codegen-test",
            EntryKind::StampValue,
            &BytecodeProgram {
                instructions: vec![
                    Instruction::PushConst(2.0),
                    Instruction::PushConst(1.0),
                    Instruction::ZiState(0),
                    Instruction::Add,
                ],
            },
            NativeLoweringLimits::new(0, 0, 0, 0, 0).with_zi_filter_count(1),
        )
        .expect("lower zi helper program");
        let bytes = compile_value_function(&program).expect("compile zi helper leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate zi helper leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let mut filters = [ZiFilter::new(vec![0.25], vec![1.0, -0.75], 1.0e-6)];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.zi_filters = filters.as_mut_ptr();
        ctx.zi_filters_len = filters.len();

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            3.0_f64.to_bits(),
            "non-transient zi evaluation uses DC steady state"
        );

        ctx.analysis_type = 2;
        ctx.time = 0.0;
        let first = f(&ctx, std::ptr::null());
        let repeated = f(&ctx, std::ptr::null());
        assert_eq!(
            first.to_bits(),
            repeated.to_bits(),
            "native zi helper must preserve Newton re-evaluation idempotence"
        );
        assert!((first - 2.25).abs() < 1.0e-12, "first zi sample: {first}");
        filters[0].commit(ctx.time);

        ctx.time = 0.5e-6;
        let held = f(&ctx, std::ptr::null());
        assert!((held - 2.25).abs() < 1.0e-12, "held zi output: {held}");
        filters[0].commit(ctx.time);

        ctx.time = 1.0e-6;
        let next = f(&ctx, std::ptr::null());
        assert!((next - 2.4375).abs() < 1.0e-12, "second zi sample: {next}");
    }

    #[test]
    fn generated_value_leaf_calls_transition_helper_and_preserves_stack() {
        let program = NativeProgram::from_bytecode(
            "x64-codegen-test",
            EntryKind::StampValue,
            &BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(1.0),
                    Instruction::PushConst(0.2),
                    Instruction::PushConst(0.4),
                    Instruction::PushConst(0.4),
                    Instruction::TransitionState(0),
                    Instruction::Add,
                ],
            },
            NativeLoweringLimits::new(0, 0, 0, 0, 0),
        )
        .expect("lower transition helper program");
        let bytes = compile_value_function(&program).expect("compile transition helper leaf");
        assert!(
            contains_bytes(
                &bytes,
                &xor_r64_bytes(
                    super::operand_filter_id_arg_reg(),
                    super::operand_filter_id_arg_reg()
                )
            ),
            "transition helper should zero filter ID 0 with a compact dependency-breaking xor"
        );
        assert!(
            !contains_bytes(
                &bytes,
                &mov_r32_imm32_bytes(super::operand_filter_id_arg_reg(), 0)
            ),
            "transition helper should not materialize filter ID 0 with a wider imm32 move"
        );
        assert!(
            !contains_bytes(
                &bytes,
                &movabs_imm64_bytes(super::operand_filter_id_arg_reg(), 0)
            ),
            "transition helper should not use movabs for small filter IDs"
        );
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate transition helper leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let mut filters = [TransitionFilter::default()];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.temperature = 310.0;
        ctx.transition_filters = filters.as_mut_ptr();
        ctx.transition_filters_len = filters.len();

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            311.0_f64.to_bits(),
            "non-transient transition evaluation passes input through"
        );

        ctx.analysis_type = 2;

        ctx.time = 1.0;
        let first = f(&ctx, std::ptr::null());
        assert_eq!(first.to_bits(), 310.0_f64.to_bits());

        ctx.time = 1.4;
        let mid = f(&ctx, std::ptr::null());
        assert!((mid - 310.5).abs() < 1.0e-12, "mid transition: {mid}");

        ctx.time = 1.6;
        let done = f(&ctx, std::ptr::null());
        assert!((done - 311.0).abs() < 1.0e-12, "done transition: {done}");
    }

    #[test]
    fn generated_value_leaf_calls_slew_helper_and_preserves_stack() {
        let program = NativeProgram::from_bytecode(
            "x64-codegen-test",
            EntryKind::StampValue,
            &BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(10.0),
                    Instruction::PushConst(2.0),
                    Instruction::PushConst(2.0),
                    Instruction::SlewState(0),
                    Instruction::Add,
                ],
            },
            NativeLoweringLimits::new(0, 0, 0, 0, 0),
        )
        .expect("lower slew helper program");
        let bytes = compile_value_function(&program).expect("compile slew helper leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate slew helper leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let mut filters = [SlewFilter::default()];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.temperature = 310.0;
        ctx.slew_filters = filters.as_mut_ptr();
        ctx.slew_filters_len = filters.len();

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            320.0_f64.to_bits(),
            "non-transient slew evaluation passes input through"
        );

        ctx.analysis_type = 2;

        ctx.time = 0.0;
        let first = f(&ctx, std::ptr::null());
        assert_eq!(first.to_bits(), 310.0_f64.to_bits());

        ctx.time = 0.5;
        let mid = f(&ctx, std::ptr::null());
        assert!((mid - 311.0).abs() < 1.0e-12, "mid slew: {mid}");

        ctx.time = 1.0;
        let done = f(&ctx, std::ptr::null());
        assert!((done - 312.0).abs() < 1.0e-12, "done slew: {done}");
    }

    #[test]
    fn generated_value_leaf_calls_absdelay_helper_and_preserves_stack() {
        let program = NativeProgram::from_bytecode(
            "x64-codegen-test",
            EntryKind::StampValue,
            &BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushVoltage(0, usize::MAX),
                    Instruction::PushConst(0.5),
                    Instruction::AbsDelayState(0),
                    Instruction::Add,
                ],
            },
            NativeLoweringLimits::new(1, 0, 0, 0, 0),
        )
        .expect("lower absdelay helper program");
        let bytes = compile_value_function(&program).expect("compile absdelay helper leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate absdelay helper leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let mut voltages = [7.0_f64];
        let mut buffers = [DelayBuffer::default()];
        let mut ctx = eval_context(&[], &voltages, &[], &[]);
        ctx.temperature = 310.0;
        ctx.delay_buffers = buffers.as_mut_ptr();
        ctx.delay_buffers_len = buffers.len();

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            317.0_f64.to_bits(),
            "non-transient absdelay evaluation passes input through"
        );

        ctx.analysis_type = 2;

        ctx.time = 0.0;
        voltages[0] = 0.0;
        std::hint::black_box(voltages[0]);
        let first = f(&ctx, std::ptr::null());
        assert_eq!(first.to_bits(), 310.0_f64.to_bits());

        ctx.time = 0.5;
        voltages[0] = 1.0;
        std::hint::black_box(voltages[0]);
        let delayed_start = f(&ctx, std::ptr::null());
        assert_eq!(delayed_start.to_bits(), 310.0_f64.to_bits());

        ctx.time = 1.0;
        voltages[0] = 3.0;
        std::hint::black_box(voltages[0]);
        let delayed = f(&ctx, std::ptr::null());
        assert!((delayed - 311.0).abs() < 1.0e-12, "delayed: {delayed}");

        ctx.time = 1.25;
        voltages[0] = 5.0;
        std::hint::black_box(voltages[0]);
        let interpolated = f(&ctx, std::ptr::null());
        assert!(
            (interpolated - 312.0).abs() < 1.0e-12,
            "interpolated: {interpolated}"
        );
    }

    #[test]
    fn generated_value_leaf_calls_cross_helper_and_preserves_stack() {
        let program = NativeProgram::from_bytecode(
            "x64-codegen-test",
            EntryKind::StampValue,
            &BytecodeProgram {
                instructions: vec![
                    Instruction::PushTemperature,
                    Instruction::PushVoltage(0, usize::MAX),
                    Instruction::PushConst(1.0),
                    Instruction::CrossState(0),
                    Instruction::Add,
                ],
            },
            NativeLoweringLimits::new(1, 0, 0, 0, 0),
        )
        .expect("lower cross helper program");
        let bytes = compile_value_function(&program).expect("compile cross helper leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate cross helper leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let mut voltages = [7.0_f64];
        let mut detectors = [CrossDetector::default()];
        let mut ctx = eval_context(&[], &voltages, &[], &[]);
        ctx.temperature = 310.0;
        ctx.cross_detectors = detectors.as_mut_ptr();
        ctx.cross_detectors_len = detectors.len();

        ctx.time = -0.5;
        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            310.0_f64.to_bits(),
            "non-transient cross evaluation reports zero while preserving the stack"
        );

        ctx.analysis_type = 2;

        ctx.time = 0.0;
        voltages[0] = -1.0;
        std::hint::black_box(voltages[0]);
        let first = f(&ctx, std::ptr::null());
        assert_eq!(first.to_bits(), 310.0_f64.to_bits());

        ctx.time = 0.5;
        voltages[0] = 1.0;
        std::hint::black_box(voltages[0]);
        let crossing = f(&ctx, std::ptr::null());
        assert_eq!(crossing.to_bits(), 311.0_f64.to_bits());

        ctx.time = 1.0;
        voltages[0] = 2.0;
        std::hint::black_box(voltages[0]);
        let steady = f(&ctx, std::ptr::null());
        assert_eq!(steady.to_bits(), 310.0_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_computes_ordered_comparisons() {
        let cases = [
            ("gt-true", Instruction::Gt, 5.0, 3.0, 1.0),
            ("gt-false", Instruction::Gt, 3.0, 5.0, 0.0),
            ("lt-true", Instruction::Lt, 3.0, 5.0, 1.0),
            ("lt-false", Instruction::Lt, 5.0, 3.0, 0.0),
            ("ge-true", Instruction::Ge, 5.0, 3.0, 1.0),
            ("ge-false", Instruction::Ge, 3.0, 5.0, 0.0),
            ("ge-equal", Instruction::Ge, 3.0, 3.0, 1.0),
            ("le-true", Instruction::Le, 3.0, 5.0, 1.0),
            ("le-false", Instruction::Le, 5.0, 3.0, 0.0),
            ("le-equal", Instruction::Le, 3.0, 3.0, 1.0),
            ("gt-left-unordered", Instruction::Gt, f64::NAN, 3.0, 0.0),
            ("gt-right-unordered", Instruction::Gt, 3.0, f64::NAN, 0.0),
            ("lt-left-unordered", Instruction::Lt, f64::NAN, 3.0, 0.0),
            ("lt-right-unordered", Instruction::Lt, 3.0, f64::NAN, 0.0),
            ("ge-left-unordered", Instruction::Ge, f64::NAN, 3.0, 0.0),
            ("ge-right-unordered", Instruction::Ge, 3.0, f64::NAN, 0.0),
            ("le-left-unordered", Instruction::Le, f64::NAN, 3.0, 0.0),
            ("le-right-unordered", Instruction::Le, 3.0, f64::NAN, 0.0),
        ];

        for (name, op, left, right, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    op,
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile comparison leaf");
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate comparison leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_eq!(f(&ctx, std::ptr::null()), expected, "{name}");
        }
    }

    #[test]
    fn generated_value_leaf_computes_equality_comparisons() {
        let cases = [
            ("eq-exact", Instruction::Eq, 1.0, 1.0, 1.0),
            ("eq-within-epsilon", Instruction::Eq, 0.0, 0.5e-15, 1.0),
            ("eq-at-epsilon", Instruction::Eq, 0.0, 1.0e-15, 0.0),
            ("eq-outside-epsilon", Instruction::Eq, 0.0, 1.5e-15, 0.0),
            ("ne-exact", Instruction::Ne, 1.0, 1.0, 0.0),
            ("ne-within-epsilon", Instruction::Ne, 0.0, 0.5e-15, 0.0),
            ("ne-at-epsilon", Instruction::Ne, 0.0, 1.0e-15, 1.0),
            ("ne-outside-epsilon", Instruction::Ne, 0.0, 1.5e-15, 1.0),
            ("eq-left-unordered", Instruction::Eq, f64::NAN, 1.0, 0.0),
            ("eq-right-unordered", Instruction::Eq, 1.0, f64::NAN, 0.0),
            ("ne-left-unordered", Instruction::Ne, f64::NAN, 1.0, 0.0),
            ("ne-right-unordered", Instruction::Ne, 1.0, f64::NAN, 0.0),
        ];

        for (name, op, left, right, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    op,
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile equality leaf");
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate equality leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_eq!(f(&ctx, std::ptr::null()), expected, "{name}");
        }
    }

    #[test]
    fn generated_value_leaf_computes_logical_ops() {
        let cases = [
            ("and-true", Instruction::And, 2.0e-15, -2.0e-15, 1.0),
            (
                "and-left-at-epsilon",
                Instruction::And,
                1.0e-15,
                2.0e-15,
                0.0,
            ),
            (
                "and-left-unordered",
                Instruction::And,
                f64::NAN,
                2.0e-15,
                0.0,
            ),
            (
                "and-right-unordered",
                Instruction::And,
                2.0e-15,
                f64::NAN,
                0.0,
            ),
            ("or-right-true", Instruction::Or, 0.5e-15, -2.0e-15, 1.0),
            ("or-both-false", Instruction::Or, 1.0e-15, 0.5e-15, 0.0),
            ("or-left-unordered", Instruction::Or, f64::NAN, 0.5e-15, 0.0),
            (
                "or-right-unordered",
                Instruction::Or,
                0.5e-15,
                f64::NAN,
                0.0,
            ),
        ];

        for (name, op, left, right, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    op,
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile logical leaf");
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate logical leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_eq!(f(&ctx, std::ptr::null()), expected, "{name}");
        }

        let const_rhs_cases = [
            (
                "and-rhs-true-left-true",
                Instruction::And,
                2.0e-15,
                -2.0e-15,
                1.0,
            ),
            (
                "and-rhs-true-left-false",
                Instruction::And,
                0.5e-15,
                -2.0e-15,
                0.0,
            ),
            ("and-rhs-false", Instruction::And, 2.0e-15, 1.0e-15, 0.0),
            (
                "and-rhs-unordered",
                Instruction::And,
                2.0e-15,
                f64::NAN,
                0.0,
            ),
            ("or-rhs-true", Instruction::Or, 0.5e-15, -2.0e-15, 1.0),
            (
                "or-rhs-false-left-true",
                Instruction::Or,
                2.0e-15,
                1.0e-15,
                1.0,
            ),
            (
                "or-rhs-false-left-false",
                Instruction::Or,
                0.5e-15,
                1.0e-15,
                0.0,
            ),
            ("or-rhs-unordered", Instruction::Or, 0.5e-15, f64::NAN, 0.0),
        ];

        for (name, op, input, rhs, expected) in const_rhs_cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(rhs),
                    op,
                ],
                0,
            );

            assert_eq!(
                program.max_stack_depth(),
                1,
                "{name} should use a literal RHS logical op, not a second stack slot"
            );

            let bytes = compile_value_function(&program).expect("compile literal RHS logical leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant RHS logical op should stay helper-free"
            );
            if name == "or-rhs-true" {
                assert!(
                    contains_bytes(&bytes, &mov_r32_imm32_bytes(Gpr::R10, 1)),
                    "literal true logical result should use compact imm32 materialization"
                );
                assert!(
                    !contains_bytes(&bytes, &movabs_imm64_bytes(Gpr::R10, 1)),
                    "literal true logical result should not use a 64-bit immediate"
                );
            }

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate literal RHS logical leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = input;

            assert_eq!(f(&ctx, std::ptr::null()), expected, "{name}");
        }

        let const_lhs_cases = [
            (
                "and-lhs-true-right-true",
                Instruction::And,
                -2.0e-15,
                2.0e-15,
                1.0,
            ),
            (
                "and-lhs-true-right-false",
                Instruction::And,
                -2.0e-15,
                0.5e-15,
                0.0,
            ),
            ("and-lhs-false", Instruction::And, 1.0e-15, 2.0e-15, 0.0),
            (
                "and-lhs-unordered",
                Instruction::And,
                f64::NAN,
                2.0e-15,
                0.0,
            ),
            ("or-lhs-true", Instruction::Or, -2.0e-15, 0.5e-15, 1.0),
            (
                "or-lhs-false-right-true",
                Instruction::Or,
                1.0e-15,
                2.0e-15,
                1.0,
            ),
            (
                "or-lhs-false-right-false",
                Instruction::Or,
                1.0e-15,
                0.5e-15,
                0.0,
            ),
            ("or-lhs-unordered", Instruction::Or, f64::NAN, 0.5e-15, 0.0),
        ];

        for (name, op, lhs, input, expected) in const_lhs_cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(lhs),
                    Instruction::PushTemperature,
                    op,
                ],
                0,
            );

            assert_eq!(
                program.max_stack_depth(),
                1,
                "{name} should use a literal LHS logical op, not a second stack slot"
            );

            let bytes = compile_value_function(&program).expect("compile literal LHS logical leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant LHS logical op should stay helper-free"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate literal LHS logical leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = input;

            assert_eq!(f(&ctx, std::ptr::null()), expected, "{name}");
        }

        let not_cases = [
            ("not-within-epsilon", 0.5e-15, 1.0),
            ("not-at-epsilon", 1.0e-15, 0.0),
            ("not-outside-epsilon", 2.0e-15, 0.0),
            ("not-unordered", f64::NAN, 0.0),
        ];

        for (name, value, expected) in not_cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![Instruction::PushConst(value), Instruction::Not],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile logical-not leaf");
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate logical-not leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_eq!(f(&ctx, std::ptr::null()), expected, "{name}");
        }
    }

    #[test]
    fn generated_value_leaf_folds_constant_logical_ops_to_literals() {
        let cases = [
            ("and-both-true", Instruction::And, 2.0e-15, -2.0e-15, 1.0),
            (
                "and-left-at-epsilon",
                Instruction::And,
                1.0e-15,
                2.0e-15,
                0.0,
            ),
            (
                "and-right-unordered",
                Instruction::And,
                2.0e-15,
                f64::NAN,
                0.0,
            ),
            ("or-right-true", Instruction::Or, 0.5e-15, -2.0e-15, 1.0),
            ("or-both-false", Instruction::Or, 1.0e-15, 0.5e-15, 0.0),
            ("or-left-unordered", Instruction::Or, f64::NAN, 0.5e-15, 0.0),
        ];

        for (name, instruction, left, right, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    instruction,
                ],
                0,
            );

            assert_eq!(program.max_stack_depth(), 1, "{name}");
            assert_eq!(
                program.ops(),
                &[NativeOp::Const(expected)],
                "{name} should compile as a folded literal"
            );

            let bytes = compile_value_function(&program).expect("compile folded logical leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "folded logical op should stay helper-free"
            );

            let memory = ExecutableMemory::allocate(&bytes).expect("allocate folded logical leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_computes_constant_rhs_shifts_without_count_checks() {
        let cases = [
            (
                "shl",
                Instruction::Shl,
                IntegerBinaryOp::Shl,
                3.0,
                2.0,
                runtime_shl(3.0, 2.0),
                shift_imm_bytes(IntegerBinaryOp::Shl, Gpr::R11, 2),
            ),
            (
                "shr-negative",
                Instruction::Shr,
                IntegerBinaryOp::Shr,
                -16.0,
                2.75,
                runtime_shr(-16.0, 2.75),
                shift_imm_bytes(IntegerBinaryOp::Shr, Gpr::R11, 2),
            ),
        ];

        for (name, instruction, expected_op, left, right, integer_expected, shift_bytes) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushParam(0),
                    Instruction::PushConst(right),
                    instruction,
                ],
                0,
            );
            assert_eq!(
                program.ops(),
                &[
                    NativeOp::LoadParam(0),
                    NativeOp::IntegerShiftConst(expected_op, right as i64 as u8),
                ],
                "{name}: valid constant count should lower to an immediate shift"
            );
            assert_eq!(
                program.max_stack_depth(),
                1,
                "{name}: immediate shift should not allocate an RHS stack slot"
            );

            let bytes = compile_value_function(&program).expect("compile constant shift leaf");
            assert!(
                contains_bytes(&bytes, &shift_bytes),
                "{name}: generated code should use an imm8 shift"
            );
            assert!(
                !contains_bytes(&bytes, &cmp_r64_imm32_bytes(Gpr::Rcx, 64)),
                "{name}: constant count shifts should not emit runtime count bounds checks"
            );
            assert!(
                !contains_bytes(&bytes, &xor_r64_bytes(Gpr::Rcx, Gpr::Rcx)),
                "{name}: constant count shifts should not convert an RHS count through RCX"
            );

            let memory = ExecutableMemory::allocate(&bytes).expect("allocate constant shift leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let params = [left];
            let ctx = eval_context(&params, &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                integer_expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_elides_zero_count_constant_shift_opcode() {
        let cases = [
            (
                "shl-zero-fractional",
                Instruction::Shl,
                IntegerBinaryOp::Shl,
                13.75,
                runtime_shl(13.75, 0.0),
            ),
            (
                "shr-zero-nan",
                Instruction::Shr,
                IntegerBinaryOp::Shr,
                f64::NAN,
                runtime_shr(f64::NAN, 0.0),
            ),
        ];

        for (name, instruction, expected_op, left, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushParam(0),
                    Instruction::PushConst(0.0),
                    instruction,
                ],
                0,
            );
            assert_eq!(
                program.ops(),
                &[
                    NativeOp::LoadParam(0),
                    NativeOp::IntegerShiftConst(expected_op, 0)
                ],
                "{name}: zero count should still lower to the integer-conversion shift op"
            );

            let bytes = compile_value_function(&program).expect("compile zero-count shift leaf");
            assert!(
                !contains_bytes(&bytes, &shift_imm_bytes(expected_op, Gpr::R11, 0)),
                "{name}: zero-count constant shifts should skip the redundant shift instruction"
            );
            assert!(
                !contains_bytes(&bytes, &cmp_r64_imm32_bytes(Gpr::Rcx, 64)),
                "{name}: constant zero-count shifts should not emit runtime count bounds checks"
            );
            assert!(
                !contains_bytes(&bytes, &xor_r64_bytes(Gpr::Rcx, Gpr::Rcx)),
                "{name}: constant zero-count shifts should not convert an RHS count through RCX"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate zero-count shift leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let params = [left];
            let ctx = eval_context(&params, &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}: zero-count shift must still perform Rust-style f64-to-i64 conversion"
            );
        }
    }

    #[test]
    fn generated_value_leaf_computes_runtime_shifts_without_helper_call() {
        let cases = [
            ("shl", Instruction::Shl, 3.0, 2.0, runtime_shl(3.0, 2.0)),
            (
                "shr-negative",
                Instruction::Shr,
                -16.0,
                2.0,
                runtime_shr(-16.0, 2.0),
            ),
        ];

        for (name, op, left, right, integer_expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushVariable(0),
                    Instruction::PushParam(0),
                    Instruction::PushParam(1),
                    op,
                    Instruction::Add,
                    Instruction::PushTime,
                    Instruction::Add,
                    Instruction::Add,
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile integer shift leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "{name}: runtime shifts should not pay helper-call prologue"
            );
            assert!(
                contains_bytes(&bytes, &xor_r64_bytes(Gpr::R11, Gpr::R11)),
                "{name}: f64-to-i64 NaN path should zero the left operand with a zero idiom"
            );
            assert!(
                contains_bytes(&bytes, &xor_r64_bytes(Gpr::Rcx, Gpr::Rcx)),
                "{name}: f64-to-i64 NaN path should zero the shift count with a zero idiom"
            );
            assert!(
                !contains_bytes(&bytes, &movabs_imm64_bytes(Gpr::R11, 0)),
                "{name}: f64-to-i64 NaN path should not materialize zero as a 64-bit immediate"
            );
            assert!(
                !contains_bytes(&bytes, &movabs_imm64_bytes(Gpr::Rcx, 0)),
                "{name}: f64-to-i64 NaN path should not materialize zero as a 64-bit immediate"
            );
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate integer shift leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let params = [left, right];
            let mut ctx = eval_context(&params, &[], &[], &[]);
            ctx.temperature = 310.0;
            ctx.time = 2.0;
            let vars = [7.0_f64];

            clear_native_runtime_error();
            assert_eq!(
                f(&ctx, vars.as_ptr()).to_bits(),
                (310.0 + ((7.0 + integer_expected) + 2.0)).to_bits(),
                "{name}"
            );
            assert!(
                take_native_runtime_error().is_none(),
                "{name}: valid runtime shift count should not report a native runtime error"
            );
        }
    }

    #[test]
    fn generated_value_leaf_runtime_shifts_hard_fail_invalid_counts_without_helper_call() {
        let cases = [
            ("shl-negative-count", Instruction::Shl, 3.0, -1.0),
            ("shr-too-large-count", Instruction::Shr, -16.0, 64.0),
        ];

        for (name, op, left, right) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![Instruction::PushParam(0), Instruction::PushParam(1), op],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile integer shift leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "{name}: invalid runtime shifts should not require the helper-call prologue"
            );
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate integer shift leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let params = [left, right];
            let ctx = eval_context(&params, &[], &[], &[]);

            clear_native_runtime_error();
            let result = f(&ctx, std::ptr::null());

            assert_eq!(result.to_bits(), 0.0_f64.to_bits(), "{name}");
            let error = take_native_runtime_error().expect("invalid shift count must hard-fail");
            assert!(
                error.contains("integer shift count"),
                "{name}: error must identify shift-count failure, got: {error}"
            );
            assert!(
                error.contains("no interpreter fallback"),
                "{name}: error must preserve the native hard-fail contract, got: {error}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_computes_runtime_bitwise_integer_ops_without_helper_call() {
        let cases = [
            (
                "bitand-truncates",
                Instruction::BitAnd,
                13.75,
                6.25,
                runtime_bitand(13.75, 6.25),
            ),
            (
                "bitor-negative",
                Instruction::BitOr,
                -16.0,
                3.0,
                runtime_bitor(-16.0, 3.0),
            ),
            (
                "bitxor-nan",
                Instruction::BitXor,
                f64::NAN,
                7.0,
                runtime_bitxor(f64::NAN, 7.0),
            ),
            (
                "bitand-positive-infinity",
                Instruction::BitAnd,
                f64::INFINITY,
                -1.0,
                runtime_bitand(f64::INFINITY, -1.0),
            ),
            (
                "bitand-positive-saturation",
                Instruction::BitAnd,
                I64_MAX_EXCLUSIVE_AS_F64,
                -1.0,
                runtime_bitand(I64_MAX_EXCLUSIVE_AS_F64, -1.0),
            ),
            (
                "bitor-negative-saturation",
                Instruction::BitOr,
                I64_MIN_AS_F64,
                7.0,
                runtime_bitor(I64_MIN_AS_F64, 7.0),
            ),
            (
                "bitxor-negative-infinity",
                Instruction::BitXor,
                f64::NEG_INFINITY,
                7.0,
                runtime_bitxor(f64::NEG_INFINITY, 7.0),
            ),
        ];

        for (name, op, left, right, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![Instruction::PushParam(0), Instruction::PushParam(1), op],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile bitwise leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "{name}: runtime bitwise integer ops should not pay helper-call prologue"
            );

            let memory = ExecutableMemory::allocate(&bytes).expect("allocate bitwise leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let params = [left, right];
            let ctx = eval_context(&params, &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_computes_constant_rhs_bitwise_without_rhs_conversion() {
        let cases = [
            (
                "bitand-imm",
                Instruction::BitAnd,
                IntegerBinaryOp::BitAnd,
                13.75,
                6.25,
                6,
                runtime_bitand(13.75, 6.25),
            ),
            (
                "bitor-negative-imm",
                Instruction::BitOr,
                IntegerBinaryOp::BitOr,
                -16.0,
                -1.0,
                -1,
                runtime_bitor(-16.0, -1.0),
            ),
            (
                "bitxor-nonzero-imm",
                Instruction::BitXor,
                IntegerBinaryOp::BitXor,
                f64::NAN,
                7.0,
                7,
                runtime_bitxor(f64::NAN, 7.0),
            ),
        ];

        for (name, instruction, expected_op, left, right, rhs_i64, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushParam(0),
                    Instruction::PushConst(right),
                    instruction,
                ],
                0,
            );
            assert_eq!(
                program.ops(),
                &[
                    NativeOp::LoadParam(0),
                    NativeOp::IntegerBinaryConst(expected_op, rhs_i64),
                ],
                "{name}: constant RHS should lower to an immediate bitwise op"
            );
            assert_eq!(
                program.max_stack_depth(),
                1,
                "{name}: constant RHS bitwise should not allocate an RHS stack slot"
            );

            let bytes = compile_value_function(&program).expect("compile constant bitwise leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "{name}: constant bitwise ops should not pay helper-call prologue"
            );
            assert!(
                contains_bytes(
                    &bytes,
                    &bitwise_imm32_bytes(expected_op, Gpr::R10, rhs_i64 as i32)
                ),
                "{name}: generated code should apply the RHS as an imm32"
            );
            assert!(
                !contains_bytes(&bytes, &xor_r64_bytes(Gpr::R11, Gpr::R11)),
                "{name}: constant RHS bitwise should not convert the RHS through R11"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate constant bitwise leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let params = [left];
            let ctx = eval_context(&params, &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }

        let wide_program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushParam(0),
                Instruction::PushConst(f64::INFINITY),
                Instruction::BitAnd,
            ],
            0,
        );
        let wide_bytes =
            compile_value_function(&wide_program).expect("compile wide constant bitwise leaf");
        assert!(
            contains_bytes(&wide_bytes, &movabs_imm64_bytes(Gpr::R11, i64::MAX as u64)),
            "wide constant RHS bitwise should materialize the full i64 value"
        );

        let memory = ExecutableMemory::allocate(&wide_bytes).expect("allocate wide bitwise leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let params = [13.75];
        let ctx = eval_context(&params, &[], &[], &[]);
        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            runtime_bitand(13.75, f64::INFINITY).to_bits()
        );
    }

    #[test]
    fn generated_value_leaf_computes_constant_lhs_bitwise_without_lhs_conversion() {
        let cases = [
            (
                "bitand-lhs-imm",
                Instruction::BitAnd,
                IntegerBinaryOp::BitAnd,
                6.25,
                13.75,
                6,
                runtime_bitand(6.25, 13.75),
            ),
            (
                "bitor-negative-lhs-imm",
                Instruction::BitOr,
                IntegerBinaryOp::BitOr,
                -1.0,
                -16.0,
                -1,
                runtime_bitor(-1.0, -16.0),
            ),
            (
                "bitxor-lhs-imm",
                Instruction::BitXor,
                IntegerBinaryOp::BitXor,
                7.0,
                f64::NAN,
                7,
                runtime_bitxor(7.0, f64::NAN),
            ),
        ];

        for (name, instruction, expected_op, left, right, lhs_i64, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(left),
                    Instruction::PushParam(0),
                    instruction,
                ],
                0,
            );
            assert_eq!(
                program.ops(),
                &[
                    NativeOp::LoadParam(0),
                    NativeOp::IntegerBinaryConst(expected_op, lhs_i64),
                ],
                "{name}: constant LHS should commute to an immediate bitwise op"
            );
            assert_eq!(
                program.max_stack_depth(),
                1,
                "{name}: constant LHS bitwise should not allocate an LHS stack slot"
            );

            let bytes =
                compile_value_function(&program).expect("compile constant-LHS bitwise leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "{name}: constant-LHS bitwise ops should not pay helper-call prologue"
            );
            assert!(
                contains_bytes(
                    &bytes,
                    &bitwise_imm32_bytes(expected_op, Gpr::R10, lhs_i64 as i32)
                ),
                "{name}: generated code should apply the LHS as an imm32"
            );
            assert!(
                !contains_bytes(&bytes, &xor_r64_bytes(Gpr::R11, Gpr::R11)),
                "{name}: constant LHS bitwise should not convert the LHS through R11"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate constant-LHS bitwise leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let params = [right];
            let ctx = eval_context(&params, &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }

        let wide_program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushConst(f64::INFINITY),
                Instruction::PushParam(0),
                Instruction::BitAnd,
            ],
            0,
        );
        let wide_bytes =
            compile_value_function(&wide_program).expect("compile wide constant-LHS bitwise leaf");
        assert!(
            contains_bytes(&wide_bytes, &movabs_imm64_bytes(Gpr::R11, i64::MAX as u64)),
            "wide constant LHS bitwise should materialize the full i64 value"
        );

        let memory = ExecutableMemory::allocate(&wide_bytes)
            .expect("allocate wide constant-LHS bitwise leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let params = [13.75];
        let ctx = eval_context(&params, &[], &[], &[]);
        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            runtime_bitand(f64::INFINITY, 13.75).to_bits()
        );
    }

    #[test]
    fn generated_value_leaf_integerizes_constant_rhs_bitwise_identities() {
        let cases = [
            (
                "bitor-zero-fraction",
                Instruction::BitOr,
                IntegerBinaryOp::BitOr,
                13.75,
                0.0,
                runtime_bitor(13.75, 0.0),
            ),
            (
                "bitxor-negative-zero-fraction",
                Instruction::BitXor,
                IntegerBinaryOp::BitXor,
                -16.75,
                -0.0,
                runtime_bitxor(-16.75, -0.0),
            ),
            (
                "bitand-all-ones-nan",
                Instruction::BitAnd,
                IntegerBinaryOp::BitAnd,
                f64::NAN,
                -1.0,
                runtime_bitand(f64::NAN, -1.0),
            ),
            (
                "bitand-all-ones-positive-infinity",
                Instruction::BitAnd,
                IntegerBinaryOp::BitAnd,
                f64::INFINITY,
                -1.0,
                runtime_bitand(f64::INFINITY, -1.0),
            ),
        ];

        for (name, instruction, elided_op, left, right, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushParam(0),
                    Instruction::PushConst(right),
                    instruction,
                ],
                0,
            );
            assert_eq!(
                program.ops(),
                &[NativeOp::LoadParam(0), NativeOp::IntegerCast],
                "{name}: identity bitwise op should lower to integer conversion"
            );
            assert_eq!(
                program.max_stack_depth(),
                1,
                "{name}: identity bitwise op should not allocate an RHS stack slot"
            );

            let bytes = compile_value_function(&program).expect("compile integer-cast leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "{name}: integer cast should not pay helper-call prologue"
            );
            assert!(
                !contains_bytes(
                    &bytes,
                    &bitwise_imm32_bytes(elided_op, Gpr::R10, right as i64 as i32)
                ),
                "{name}: generated code should elide the no-op bitwise immediate"
            );
            assert!(
                !contains_bytes(&bytes, &movabs_imm64_bytes(Gpr::R11, right as i64 as u64)),
                "{name}: generated code should not materialize the RHS"
            );

            let memory = ExecutableMemory::allocate(&bytes).expect("allocate integer-cast leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let params = [left];
            let ctx = eval_context(&params, &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_integerizes_constant_lhs_bitwise_identities() {
        let cases = [
            (
                "bitor-zero-fraction",
                Instruction::BitOr,
                IntegerBinaryOp::BitOr,
                0.0,
                13.75,
                runtime_bitor(0.0, 13.75),
            ),
            (
                "bitxor-negative-zero-fraction",
                Instruction::BitXor,
                IntegerBinaryOp::BitXor,
                -0.0,
                -16.75,
                runtime_bitxor(-0.0, -16.75),
            ),
            (
                "bitand-all-ones-nan",
                Instruction::BitAnd,
                IntegerBinaryOp::BitAnd,
                -1.0,
                f64::NAN,
                runtime_bitand(-1.0, f64::NAN),
            ),
            (
                "bitand-all-ones-positive-infinity",
                Instruction::BitAnd,
                IntegerBinaryOp::BitAnd,
                -1.0,
                f64::INFINITY,
                runtime_bitand(-1.0, f64::INFINITY),
            ),
        ];

        for (name, instruction, elided_op, left, right, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(left),
                    Instruction::PushParam(0),
                    instruction,
                ],
                0,
            );
            assert_eq!(
                program.ops(),
                &[NativeOp::LoadParam(0), NativeOp::IntegerCast],
                "{name}: identity bitwise op should lower to integer conversion"
            );
            assert_eq!(
                program.max_stack_depth(),
                1,
                "{name}: identity bitwise op should not allocate an LHS stack slot"
            );

            let bytes = compile_value_function(&program).expect("compile integer-cast leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "{name}: integer cast should not pay helper-call prologue"
            );
            assert!(
                !contains_bytes(
                    &bytes,
                    &bitwise_imm32_bytes(elided_op, Gpr::R10, left as i64 as i32)
                ),
                "{name}: generated code should elide the no-op bitwise immediate"
            );
            assert!(
                !contains_bytes(&bytes, &movabs_imm64_bytes(Gpr::R11, left as i64 as u64)),
                "{name}: generated code should not materialize the LHS"
            );

            let memory = ExecutableMemory::allocate(&bytes).expect("allocate integer-cast leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let params = [right];
            let ctx = eval_context(&params, &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_folds_safe_constant_integer_binary_to_literals() {
        let cases = [
            ("shl", Instruction::Shl, 3.0, 2.0, runtime_shl(3.0, 2.0)),
            (
                "shr-negative",
                Instruction::Shr,
                -16.0,
                2.0,
                runtime_shr(-16.0, 2.0),
            ),
            (
                "bitand",
                Instruction::BitAnd,
                13.0,
                6.0,
                runtime_bitand(13.0, 6.0),
            ),
            (
                "bitor",
                Instruction::BitOr,
                8.0,
                3.0,
                runtime_bitor(8.0, 3.0),
            ),
            (
                "bitxor",
                Instruction::BitXor,
                15.0,
                6.0,
                runtime_bitxor(15.0, 6.0),
            ),
            (
                "truncates-operands",
                Instruction::BitAnd,
                13.75,
                6.25,
                runtime_bitand(13.75, 6.25),
            ),
        ];

        for (name, instruction, left, right, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    instruction,
                ],
                0,
            );

            assert_eq!(program.max_stack_depth(), 1, "{name}");
            assert_eq!(
                program.ops(),
                &[NativeOp::Const(expected)],
                "{name} should compile as a folded helper-equivalent literal"
            );

            let bytes = compile_value_function(&program).expect("compile folded integer leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "folded integer binary op should stay helper-free"
            );

            let memory = ExecutableMemory::allocate(&bytes).expect("allocate folded integer leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_calls_table_helpers_and_preserves_state() {
        let cases = [
            (
                "lookup-interpolate",
                Instruction::TableLookup(0),
                1.5,
                5.0_f64,
            ),
            (
                "lookup-extrapolate",
                Instruction::TableLookup(0),
                -0.5,
                -1.0_f64,
            ),
            (
                "lookup-second-table",
                Instruction::TableLookup(1),
                1.5,
                13.0_f64,
            ),
            (
                "derivative-interpolate",
                Instruction::TableDerivative(0),
                1.5,
                6.0_f64,
            ),
            (
                "derivative-extrapolate",
                Instruction::TableDerivative(0),
                -0.5,
                2.0_f64,
            ),
            (
                "derivative-second-table",
                Instruction::TableDerivative(1),
                1.5,
                4.0_f64,
            ),
        ];

        for (name, op, input, table_expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushVariable(0),
                    Instruction::PushConst(input),
                    op,
                    Instruction::Add,
                    Instruction::PushTime,
                    Instruction::Add,
                    Instruction::Add,
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile table helper leaf");
            if name.ends_with("second-table") {
                assert!(
                    contains_bytes(&bytes, &mov_r32_imm32_bytes(super::table_id_arg_reg(), 1)),
                    "table helper should materialize small table IDs with a compact imm32 move"
                );
                assert!(
                    !contains_bytes(&bytes, &movabs_imm64_bytes(super::table_id_arg_reg(), 1)),
                    "table helper should not use movabs for small table IDs"
                );
            }
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate table helper leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let table = [
                LookupTable::from_data(vec![0.0, 1.0, 2.0], vec![0.0, 2.0, 8.0]),
                LookupTable::from_data(vec![0.0, 1.0, 2.0], vec![10.0, 11.0, 15.0]),
            ];
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = 310.0;
            ctx.time = 2.0;
            ctx.lookup_tables = table.as_ptr();
            ctx.lookup_tables_len = table.len();
            let vars = [7.0_f64];

            assert_eq!(
                f(&ctx, vars.as_ptr()).to_bits(),
                (310.0 + ((7.0 + table_expected) + 2.0)).to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_computes_ifelse() {
        let then_nan = f64::from_bits(0x7ff8_0000_0000_0001);
        let else_neg_zero = -0.0_f64;
        let cases = [
            ("true", 2.0e-15, 7.0, 3.0, 7.0_f64.to_bits()),
            ("within-epsilon", 0.5e-15, 7.0, 3.0, 3.0_f64.to_bits()),
            ("at-epsilon", 1.0e-15, 7.0, 3.0, 3.0_f64.to_bits()),
            ("unordered", f64::NAN, 7.0, 3.0, 3.0_f64.to_bits()),
            (
                "selected-then-bits",
                2.0e-15,
                then_nan,
                3.0,
                then_nan.to_bits(),
            ),
            (
                "selected-else-bits",
                0.0,
                7.0,
                else_neg_zero,
                else_neg_zero.to_bits(),
            ),
        ];

        for (name, cond, then_value, else_value, expected_bits) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(cond),
                    Instruction::PushConst(then_value),
                    Instruction::PushConst(else_value),
                    Instruction::IfElse,
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile ifelse leaf");
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate ifelse leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_eq!(f(&ctx, std::ptr::null()).to_bits(), expected_bits, "{name}");
        }
    }

    #[test]
    fn generated_value_leaf_computes_variable_condition_ifelse_with_literal_else() {
        let program = NativeProgram::from_bytecode(
            "x64-codegen-test",
            EntryKind::StampValue,
            &BytecodeProgram {
                instructions: vec![
                    Instruction::PushVariable(9),
                    Instruction::PushVariable(3),
                    Instruction::PushVariable(0),
                    Instruction::Mul,
                    Instruction::PushConst(0.0),
                    Instruction::IfElse,
                ],
            },
            NativeLoweringLimits::new(0, 0, 0, 34, 0),
        )
        .expect("lower variable ifelse leaf");
        let bytes = compile_value_function(&program).expect("compile variable ifelse leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate variable ifelse leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[], &[], &[]);
        let mut vars = vec![0.0; 34];

        vars[0] = 2.0;
        vars[3] = 4.0;
        vars[9] = 1.0;
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 8.0_f64.to_bits());

        vars[9] = 0.0;
        assert_eq!(f(&ctx, vars.as_ptr()).to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_computes_analysis_checks() {
        let program = native_program(EntryKind::StampValue, vec![Instruction::Analysis(2)], 0);
        let bytes = compile_value_function(&program).expect("compile analysis leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate analysis leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let mut ctx = eval_context(&[], &[], &[], &[]);

        ctx.analysis_type = 2;
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 1.0_f64.to_bits());

        ctx.analysis_type = 0;
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 0.0_f64.to_bits());

        let static_program =
            native_program(EntryKind::StampValue, vec![Instruction::Analysis(5)], 0);
        let static_bytes =
            compile_value_function(&static_program).expect("compile static analysis leaf");
        let static_memory =
            ExecutableMemory::allocate(&static_bytes).expect("allocate static analysis leaf");
        let static_entry = static_memory.ptr_at(0).expect("entry point inside image");
        let static_check: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(static_entry) };

        for analysis_type in [0, 4] {
            ctx.analysis_type = analysis_type;
            assert_eq!(
                static_check(&ctx, std::ptr::null()).to_bits(),
                1.0_f64.to_bits(),
                "analysis_type: {analysis_type}"
            );
        }
        for analysis_type in [1, 2, 3, 5] {
            ctx.analysis_type = analysis_type;
            assert_eq!(
                static_check(&ctx, std::ptr::null()).to_bits(),
                0.0_f64.to_bits(),
                "analysis_type: {analysis_type}"
            );
        }

        let smallsig_program =
            native_program(EntryKind::StampValue, vec![Instruction::Analysis(6)], 0);
        let smallsig_bytes =
            compile_value_function(&smallsig_program).expect("compile smallsig analysis leaf");
        let smallsig_memory =
            ExecutableMemory::allocate(&smallsig_bytes).expect("allocate smallsig analysis leaf");
        let smallsig_entry = smallsig_memory.ptr_at(0).expect("entry point inside image");
        let smallsig_check: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(smallsig_entry) };

        for analysis_type in [1, 3] {
            ctx.analysis_type = analysis_type;
            assert_eq!(
                smallsig_check(&ctx, std::ptr::null()).to_bits(),
                1.0_f64.to_bits(),
                "analysis_type: {analysis_type}"
            );
        }
        for analysis_type in [0, 2, 4, 5] {
            ctx.analysis_type = analysis_type;
            assert_eq!(
                smallsig_check(&ctx, std::ptr::null()).to_bits(),
                0.0_f64.to_bits(),
                "analysis_type: {analysis_type}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_executes_timer_state_and_preserves_stack() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushTemperature,
                Instruction::PushConst(1.0),
                Instruction::PushConst(0.5),
                Instruction::TimerState(0),
                Instruction::Add,
            ],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile timer leaf");
        let frame_bytes = call_frame_bytes(1);
        let old_fixed_frame_bytes = old_fixed_call_frame_bytes();
        assert!(
            contains_bytes(&bytes, &sub_rsp_bytes(frame_bytes)),
            "timer helper should reserve only the lower live prefix slot"
        );
        assert!(
            contains_bytes(&bytes, &add_rsp_bytes(frame_bytes)),
            "timer helper should release only the lower live prefix slot"
        );
        assert!(
            !contains_bytes(&bytes, &sub_rsp_bytes(old_fixed_frame_bytes)),
            "timer helper should not reserve the old maximum spill frame"
        );
        assert!(
            !contains_bytes(&bytes, &add_rsp_bytes(old_fixed_frame_bytes)),
            "timer helper should not release the old maximum spill frame"
        );
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate timer leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.temperature = 310.0;
        ctx.timestep = 0.01;

        ctx.time = 1.25;
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 310.0_f64.to_bits());

        ctx.time = 1.5;
        assert_eq!(f(&ctx, std::ptr::null()).to_bits(), 311.0_f64.to_bits());
    }

    #[test]
    fn generated_value_leaf_computes_min_max() {
        let left_nan = f64::from_bits(0x7ff8_0000_0000_0001);
        let right_nan = f64::from_bits(0x7ff8_0000_0000_0002);
        let cases = [
            ("min-left-smaller", Instruction::Min, -2.0, 5.0),
            ("min-right-smaller", Instruction::Min, 5.0, -2.0),
            ("min-left-nan", Instruction::Min, left_nan, 5.0),
            ("min-right-nan", Instruction::Min, 5.0, right_nan),
            ("min-both-nan", Instruction::Min, left_nan, right_nan),
            ("min-left-neg-zero", Instruction::Min, -0.0, 0.0),
            ("min-right-neg-zero", Instruction::Min, 0.0, -0.0),
            ("max-left-larger", Instruction::Max, 5.0, -2.0),
            ("max-right-larger", Instruction::Max, -2.0, 5.0),
            ("max-left-nan", Instruction::Max, left_nan, 5.0),
            ("max-right-nan", Instruction::Max, 5.0, right_nan),
            ("max-both-nan", Instruction::Max, left_nan, right_nan),
            ("max-left-pos-zero", Instruction::Max, 0.0, -0.0),
            ("max-right-pos-zero", Instruction::Max, -0.0, 0.0),
        ];

        for (name, op, left, right) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushVariable(0),
                    op.clone(),
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile min/max leaf");
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate min/max leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = left;
            let vars = [right];
            let expected = match op {
                Instruction::Min => runtime_min(left, right),
                Instruction::Max => runtime_max(left, right),
                _ => unreachable!("min/max test cases only use min/max opcodes"),
            };

            assert_eq!(
                f(&ctx, vars.as_ptr()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_applies_constant_rhs_min_max_without_extra_stack_slot() {
        let left_nan = f64::from_bits(0x7ff8_0000_0000_0001);
        let right_nan = f64::from_bits(0x7ff8_0000_0000_0002);
        let cases = [
            ("min-left-smaller", Instruction::Min, -2.0, 5.0),
            ("min-right-smaller", Instruction::Min, 5.0, -2.0),
            ("min-left-nan", Instruction::Min, left_nan, 5.0),
            ("min-right-nan", Instruction::Min, 5.0, right_nan),
            ("min-both-nan", Instruction::Min, left_nan, right_nan),
            ("min-left-neg-zero", Instruction::Min, -0.0, 0.0),
            ("min-right-neg-zero", Instruction::Min, 0.0, -0.0),
            ("max-left-larger", Instruction::Max, 5.0, -2.0),
            ("max-right-larger", Instruction::Max, -2.0, 5.0),
            ("max-left-nan", Instruction::Max, left_nan, 5.0),
            ("max-right-nan", Instruction::Max, 5.0, right_nan),
            ("max-both-nan", Instruction::Max, left_nan, right_nan),
            ("max-left-pos-zero", Instruction::Max, 0.0, -0.0),
            ("max-right-pos-zero", Instruction::Max, -0.0, 0.0),
        ];

        for (name, instruction, input, rhs) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(rhs),
                    instruction.clone(),
                ],
                0,
            );

            assert_eq!(
                program.max_stack_depth(),
                1,
                "{name} should use a literal RHS min/max, not a second stack slot"
            );

            let bytes = compile_value_function(&program).expect("compile literal RHS min/max leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant RHS min/max should stay helper-free"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate literal RHS min/max leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = input;
            let expected = match instruction {
                Instruction::Min => runtime_min(input, rhs),
                Instruction::Max => runtime_max(input, rhs),
                _ => unreachable!("min/max test cases only use min/max opcodes"),
            };

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_applies_constant_lhs_min_max_without_extra_stack_slot() {
        let left_nan = f64::from_bits(0x7ff8_0000_0000_0001);
        let right_nan = f64::from_bits(0x7ff8_0000_0000_0002);
        let cases = [
            ("min-left-smaller", Instruction::Min, -2.0, 5.0),
            ("min-right-smaller", Instruction::Min, 5.0, -2.0),
            ("min-left-nan", Instruction::Min, left_nan, 5.0),
            ("min-right-nan", Instruction::Min, 5.0, right_nan),
            ("min-both-nan", Instruction::Min, left_nan, right_nan),
            ("min-left-neg-zero", Instruction::Min, -0.0, 0.0),
            ("min-right-neg-zero", Instruction::Min, 0.0, -0.0),
            ("max-left-larger", Instruction::Max, 5.0, -2.0),
            ("max-right-larger", Instruction::Max, -2.0, 5.0),
            ("max-left-nan", Instruction::Max, left_nan, 5.0),
            ("max-right-nan", Instruction::Max, 5.0, right_nan),
            ("max-both-nan", Instruction::Max, left_nan, right_nan),
            ("max-left-pos-zero", Instruction::Max, 0.0, -0.0),
            ("max-right-pos-zero", Instruction::Max, -0.0, 0.0),
        ];

        for (name, instruction, lhs, input) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(lhs),
                    Instruction::PushTemperature,
                    instruction.clone(),
                ],
                0,
            );

            assert_eq!(
                program.max_stack_depth(),
                1,
                "{name} should use a literal LHS min/max, not a second stack slot"
            );

            let bytes = compile_value_function(&program).expect("compile literal LHS min/max leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "constant LHS min/max should stay helper-free"
            );
            assert!(
                !contains_bytes(&bytes, &sub_rsp_bytes(ROUND_TEMP_FRAME_BYTES)),
                "{name}: constant LHS min/max with a spare XMM register should not spill the RHS"
            );
            assert!(
                !contains_bytes(&bytes, &add_rsp_bytes(ROUND_TEMP_FRAME_BYTES)),
                "{name}: constant LHS min/max with a spare XMM register should not restore a spill frame"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate literal LHS min/max leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = input;
            let expected = match instruction {
                Instruction::Min => runtime_min(lhs, input),
                Instruction::Max => runtime_max(lhs, input),
                _ => unreachable!("min/max test cases only use min/max opcodes"),
            };

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_applies_constant_lhs_min_max_at_full_xmm_stack_depth() {
        let prefix = constant_prefix(XMM_STACK.len() - 1);
        let mut instructions = prefix.clone();
        instructions.extend([
            Instruction::PushConst(-2.0),
            Instruction::PushTemperature,
            Instruction::Min,
        ]);
        instructions.extend(add_reductions(prefix.len()));
        let program = native_program(EntryKind::StampValue, instructions, 0);

        assert_eq!(program.max_stack_depth(), XMM_STACK.len());

        let bytes =
            compile_value_function(&program).expect("compile full-stack literal LHS min/max leaf");
        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "full-stack constant LHS min/max should remain helper-free"
        );
        assert!(
            contains_bytes(&bytes, &sub_rsp_bytes(ROUND_TEMP_FRAME_BYTES)),
            "full-stack constant LHS min/max must save the dynamic RHS"
        );
        assert!(
            contains_bytes(&bytes, &add_rsp_bytes(ROUND_TEMP_FRAME_BYTES)),
            "full-stack constant LHS min/max must restore the temp frame"
        );

        let memory =
            ExecutableMemory::allocate(&bytes).expect("allocate full-stack literal LHS min/max");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.temperature = 5.0;

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            (constant_prefix_sum(prefix.len()) + runtime_min(-2.0, 5.0)).to_bits()
        );
    }

    #[test]
    fn generated_value_leaf_calls_unary_math_helpers_and_preserves_state() {
        let cases = [
            ("exp", Instruction::Exp, 0.5, runtime_exp(0.5)),
            ("log", Instruction::Log, 2.5, runtime_log(2.5)),
            ("log10", Instruction::Log10, 100.0, runtime_log10(100.0)),
            ("sin", Instruction::Sin, 0.5, runtime_sin(0.5)),
            ("cos", Instruction::Cos, 0.5, runtime_cos(0.5)),
            ("tan", Instruction::Tan, 0.25, runtime_tan(0.25)),
            ("sinh", Instruction::Sinh, 0.25, runtime_sinh(0.25)),
            ("cosh", Instruction::Cosh, 0.25, runtime_cosh(0.25)),
            ("tanh", Instruction::Tanh, 0.25, runtime_tanh(0.25)),
            (
                "limexp-linear",
                Instruction::Limexp,
                45.0,
                runtime_limexp(45.0),
            ),
            (
                "limexp-negative",
                Instruction::Limexp,
                -50.0,
                runtime_limexp(-50.0),
            ),
            (
                "limited-exp-linear",
                Instruction::LimitedExp,
                85.0,
                runtime_limited_exp(85.0),
            ),
            (
                "limited-exp-negative",
                Instruction::LimitedExp,
                -85.0,
                runtime_limited_exp(-85.0),
            ),
            ("asin", Instruction::Asin, 0.25, runtime_asin(0.25)),
            ("acos", Instruction::Acos, 0.25, runtime_acos(0.25)),
            ("atan", Instruction::Atan, 0.25, runtime_atan(0.25)),
            ("floor", Instruction::Floor, 3.75, runtime_floor(3.75)),
            (
                "floor-negative",
                Instruction::Floor,
                -3.25,
                runtime_floor(-3.25),
            ),
            ("ceil", Instruction::Ceil, 3.25, runtime_ceil(3.25)),
            (
                "ceil-negative",
                Instruction::Ceil,
                -3.75,
                runtime_ceil(-3.75),
            ),
        ];

        for (name, op, input, unary_expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushConst(input),
                    op,
                    Instruction::PushVariable(0),
                    Instruction::Add,
                    Instruction::PushTime,
                    Instruction::Add,
                    Instruction::Add,
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile helper-call leaf");
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate helper-call leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = 310.0;
            ctx.time = 2.0;
            let vars = [7.0_f64];

            assert_eq!(
                f(&ctx, vars.as_ptr()).to_bits(),
                (310.0 + ((unary_expected + 7.0) + 2.0)).to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_computes_floor_ceil_without_helper_call() {
        let cases = [
            ("floor-positive", Instruction::Floor, 3.75, 3.0_f64),
            ("floor-negative", Instruction::Floor, -3.25, -4.0_f64),
            ("floor-integral", Instruction::Floor, -4.0, -4.0_f64),
            ("floor-negative-zero", Instruction::Floor, -0.0, -0.0_f64),
            (
                "floor-huge",
                Instruction::Floor,
                4_503_599_627_370_496.0,
                4_503_599_627_370_496.0,
            ),
            ("ceil-positive", Instruction::Ceil, 3.25, 4.0_f64),
            ("ceil-negative", Instruction::Ceil, -3.75, -3.0_f64),
            ("ceil-integral", Instruction::Ceil, 4.0, 4.0_f64),
            ("ceil-negative-zero", Instruction::Ceil, -0.0, -0.0_f64),
            (
                "ceil-huge",
                Instruction::Ceil,
                -4_503_599_627_370_496.0,
                -4_503_599_627_370_496.0,
            ),
        ];

        for (name, instruction, input, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![Instruction::PushParam(0), instruction],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile helper-free floor/ceil");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "{name}: floor/ceil should not pay helper-call prologue cost"
            );
            assert!(
                !contains_bytes(&bytes, &sub_rsp_bytes(ROUND_TEMP_FRAME_BYTES)),
                "{name}: floor/ceil with a spare XMM register should not spill the original value"
            );
            assert!(
                !contains_bytes(&bytes, &add_rsp_bytes(ROUND_TEMP_FRAME_BYTES)),
                "{name}: floor/ceil with a spare XMM register should not restore a spill frame"
            );

            let memory = ExecutableMemory::allocate(&bytes).expect("allocate floor/ceil leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let params = [input];
            let ctx = eval_context(&params, &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_keeps_full_stack_floor_on_spill_path() {
        let prefix = constant_prefix(XMM_STACK.len() - 1);
        let mut instructions = prefix.clone();
        instructions.extend([Instruction::PushParam(0), Instruction::Floor]);
        instructions.extend(add_reductions(prefix.len()));
        let program = native_program(EntryKind::StampValue, instructions, 0);
        assert_eq!(program.max_stack_depth(), XMM_STACK.len());
        let bytes = compile_value_function(&program).expect("compile full-stack floor leaf");
        assert!(
            contains_bytes(&bytes, &sub_rsp_bytes(ROUND_TEMP_FRAME_BYTES)),
            "full-stack floor must keep the original-value spill fallback"
        );
        assert!(
            contains_bytes(&bytes, &add_rsp_bytes(ROUND_TEMP_FRAME_BYTES)),
            "full-stack floor must restore the original-value spill fallback"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate full-stack floor leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[3.75], &[], &[], &[]);

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            (constant_prefix_sum(prefix.len()) + 3.0).to_bits()
        );
    }

    #[test]
    fn generated_value_leaf_inlines_limexp_clamped_regions() {
        let program = native_program(
            EntryKind::StampValue,
            vec![Instruction::PushParam(0), Instruction::Limexp],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile inline limexp leaf");
        assert!(
            !contains_bytes(
                &bytes,
                &(rspice_limexp as *const () as usize as u64).to_le_bytes()
            ),
            "inline limexp should not call the limexp helper"
        );
        assert!(
            contains_bytes(
                &bytes,
                &(rspice_exp as *const () as usize as u64).to_le_bytes()
            ),
            "inline limexp should only call exp for the middle region"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate inline limexp leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        for (name, input) in [
            ("middle", 0.5),
            ("upper-threshold", 40.0),
            ("lower-threshold", -40.0),
            ("high-linear", 45.0),
            ("low-clamped", -50.0),
            ("nan", f64::NAN),
        ] {
            let params = [input];
            let ctx = eval_context(&params, &[], &[], &[]);
            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                runtime_limexp(input).to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_inlines_limited_exp_clamped_regions() {
        let program = native_program(
            EntryKind::StampValue,
            vec![Instruction::PushParam(0), Instruction::LimitedExp],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile inline limited-exp leaf");
        assert!(
            !contains_bytes(
                &bytes,
                &(rspice_limited_exp as *const () as usize as u64).to_le_bytes()
            ),
            "inline limited exp should not call the limited-exp helper"
        );
        assert!(
            contains_bytes(
                &bytes,
                &(rspice_exp as *const () as usize as u64).to_le_bytes()
            ),
            "inline limited exp should only call exp for the middle region"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate inline limited-exp leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        for (name, input) in [
            ("middle", 0.5),
            ("upper-threshold", 80.0),
            ("lower-threshold", -80.0),
            ("high-linear", 85.0),
            ("low-clamped", -85.0),
            ("nan", f64::NAN),
        ] {
            let params = [input];
            let ctx = eval_context(&params, &[], &[], &[]);
            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                runtime_limited_exp(input).to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn native_x64_hot_path_codegen_stays_compact_and_avoids_unneeded_calls() {
        let cases = [
            (
                "constant_leaf",
                native_program(EntryKind::StampValue, vec![Instruction::PushConst(1.0)], 0),
                32,
                false,
            ),
            (
                "arithmetic_context",
                native_program(
                    EntryKind::StampValue,
                    vec![
                        Instruction::PushParam(0),
                        Instruction::PushParam(1),
                        Instruction::Mul,
                        Instruction::PushVariable(0),
                        Instruction::Add,
                        Instruction::PushVoltage(0, 1),
                        Instruction::Add,
                        Instruction::PushTemperature,
                        Instruction::PushConst(0.01),
                        Instruction::Mul,
                        Instruction::Add,
                    ],
                    2,
                ),
                192,
                false,
            ),
            (
                "param_variable_dot8",
                native_program(
                    EntryKind::StampValue,
                    vec![
                        Instruction::PushParam(0),
                        Instruction::PushVariable(0),
                        Instruction::Mul,
                        Instruction::PushParam(1),
                        Instruction::PushVariable(1),
                        Instruction::Mul,
                        Instruction::Add,
                        Instruction::PushParam(2),
                        Instruction::PushVariable(2),
                        Instruction::Mul,
                        Instruction::Add,
                        Instruction::PushParam(3),
                        Instruction::PushVariable(3),
                        Instruction::Mul,
                        Instruction::Add,
                        Instruction::PushParam(4),
                        Instruction::PushVariable(4),
                        Instruction::Mul,
                        Instruction::Add,
                        Instruction::PushParam(5),
                        Instruction::PushVariable(5),
                        Instruction::Mul,
                        Instruction::Add,
                        Instruction::PushParam(6),
                        Instruction::PushVariable(6),
                        Instruction::Mul,
                        Instruction::Add,
                        Instruction::PushParam(7),
                        Instruction::PushVariable(7),
                        Instruction::Mul,
                        Instruction::Add,
                    ],
                    0,
                ),
                384,
                false,
            ),
            (
                "same_storage_voltage_pair",
                native_program(
                    EntryKind::StampValue,
                    vec![
                        Instruction::PushVoltage(0, 1),
                        Instruction::PushVoltage(2, 3),
                        Instruction::Add,
                    ],
                    4,
                ),
                96,
                false,
            ),
            (
                "dynamic_index_inline",
                native_program(
                    EntryKind::StampValue,
                    vec![
                        Instruction::PushVariable(0),
                        Instruction::PushVariableDyn {
                            base: 1,
                            len: 4,
                            lower: 0,
                        },
                        Instruction::PushParam(0),
                        Instruction::Add,
                    ],
                    0,
                ),
                320,
                true,
            ),
            (
                "integer_bitwise",
                native_program(
                    EntryKind::StampValue,
                    vec![
                        Instruction::PushParam(2),
                        Instruction::PushParam(3),
                        Instruction::BitAnd,
                        Instruction::PushVariable(0),
                        Instruction::Add,
                    ],
                    0,
                ),
                256,
                false,
            ),
        ];

        for (name, program, max_bytes, allow_cold_error_call) in cases {
            assert!(
                !program_uses_helper_calls(&program),
                "{name}: hot path should remain helper-free at lowering"
            );

            let bytes = compile_value_function(&program).expect("compile hot path");
            if !allow_cold_error_call {
                assert!(
                    !contains_bytes(&bytes, &call_rax_bytes()),
                    "{name}: helper-free hot path emitted an indirect helper call"
                );
            }
            assert!(
                bytes.len() <= max_bytes,
                "{name}: generated {} bytes, budget {max_bytes}",
                bytes.len()
            );
        }
    }

    #[test]
    fn generated_value_leaf_folds_constant_unary_math_to_literals() {
        let cases = [
            ("exp", Instruction::Exp, 0.5, runtime_exp(0.5)),
            ("log", Instruction::Log, 2.5, runtime_log(2.5)),
            ("log10", Instruction::Log10, 100.0, runtime_log10(100.0)),
            ("sin", Instruction::Sin, 0.5, runtime_sin(0.5)),
            ("cos", Instruction::Cos, 0.5, runtime_cos(0.5)),
            ("tan", Instruction::Tan, 0.25, runtime_tan(0.25)),
            ("sinh", Instruction::Sinh, 0.25, runtime_sinh(0.25)),
            ("cosh", Instruction::Cosh, 0.25, runtime_cosh(0.25)),
            ("tanh", Instruction::Tanh, 0.25, runtime_tanh(0.25)),
            (
                "limexp-linear",
                Instruction::Limexp,
                45.0,
                runtime_limexp(45.0),
            ),
            (
                "limexp-negative",
                Instruction::Limexp,
                -50.0,
                runtime_limexp(-50.0),
            ),
            (
                "limited-exp-linear",
                Instruction::LimitedExp,
                85.0,
                runtime_limited_exp(85.0),
            ),
            (
                "limited-exp-negative",
                Instruction::LimitedExp,
                -85.0,
                runtime_limited_exp(-85.0),
            ),
            ("asin", Instruction::Asin, 0.25, runtime_asin(0.25)),
            ("asin-domain-nan", Instruction::Asin, 2.0, runtime_asin(2.0)),
            ("acos", Instruction::Acos, 0.25, runtime_acos(0.25)),
            ("atan", Instruction::Atan, 0.25, runtime_atan(0.25)),
            ("floor", Instruction::Floor, 3.75, runtime_floor(3.75)),
            (
                "floor-negative",
                Instruction::Floor,
                -3.25,
                runtime_floor(-3.25),
            ),
            ("ceil", Instruction::Ceil, 3.25, runtime_ceil(3.25)),
            (
                "ceil-negative",
                Instruction::Ceil,
                -3.75,
                runtime_ceil(-3.75),
            ),
        ];

        for (name, instruction, input, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![Instruction::PushConst(input), instruction],
                0,
            );

            assert_eq!(program.max_stack_depth(), 1, "{name}");
            match program.ops() {
                [NativeOp::Const(value)] => assert_eq!(
                    value.to_bits(),
                    expected.to_bits(),
                    "{name} should compile as the helper-equivalent folded literal"
                ),
                other => panic!("{name} should compile as one folded literal, got {other:?}"),
            }

            let bytes = compile_value_function(&program).expect("compile folded unary math leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "folded unary math should stay helper-free"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate folded unary math leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_calls_binary_math_helpers_and_preserves_state() {
        let cases = [
            (
                "pow-operator",
                Instruction::Pow,
                2.0,
                3.0,
                runtime_pow(2.0, 3.0),
            ),
            (
                "fn-pow",
                Instruction::FnPow,
                4.0,
                0.5,
                runtime_pow(4.0, 0.5),
            ),
            (
                "atan2",
                Instruction::Atan2,
                0.5,
                0.25,
                runtime_atan2(0.5, 0.25),
            ),
            ("mod", Instruction::Mod, 5.25, 2.0, runtime_mod(5.25, 2.0)),
            (
                "mod-negative-dividend",
                Instruction::Mod,
                -5.25,
                2.0,
                runtime_mod(-5.25, 2.0),
            ),
            (
                "mod-negative-divisor",
                Instruction::Mod,
                5.25,
                -2.0,
                runtime_mod(5.25, -2.0),
            ),
        ];

        for (name, op, left, right, binary_expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushTemperature,
                    Instruction::PushVariable(0),
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    op,
                    Instruction::Add,
                    Instruction::PushTime,
                    Instruction::Add,
                    Instruction::Add,
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile binary helper-call leaf");
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate binary helper leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = 310.0;
            ctx.time = 2.0;
            let vars = [7.0_f64];

            assert_eq!(
                f(&ctx, vars.as_ptr()).to_bits(),
                (310.0 + ((7.0 + binary_expected) + 2.0)).to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_folds_constant_binary_math_to_literals() {
        let cases = [
            (
                "pow-operator",
                Instruction::Pow,
                2.0,
                3.0,
                runtime_pow(2.0, 3.0),
            ),
            (
                "fn-pow",
                Instruction::FnPow,
                4.0,
                0.5,
                runtime_pow(4.0, 0.5),
            ),
            (
                "pow-domain-nan",
                Instruction::Pow,
                -4.0,
                0.5,
                runtime_pow(-4.0, 0.5),
            ),
            (
                "atan2",
                Instruction::Atan2,
                0.5,
                0.25,
                runtime_atan2(0.5, 0.25),
            ),
            (
                "atan2-signed-zero",
                Instruction::Atan2,
                -0.0,
                -0.0,
                runtime_atan2(-0.0, -0.0),
            ),
            ("mod", Instruction::Mod, 5.25, 2.0, runtime_mod(5.25, 2.0)),
            (
                "mod-negative-dividend",
                Instruction::Mod,
                -5.25,
                2.0,
                runtime_mod(-5.25, 2.0),
            ),
            (
                "mod-zero-divisor",
                Instruction::Mod,
                5.25,
                0.0,
                runtime_mod(5.25, 0.0),
            ),
        ];

        for (name, instruction, left, right, expected) in cases {
            let program = native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    instruction,
                ],
                0,
            );

            assert_eq!(program.max_stack_depth(), 1, "{name}");
            match program.ops() {
                [NativeOp::Const(value)] => assert_eq!(
                    value.to_bits(),
                    expected.to_bits(),
                    "{name} should compile as the helper-equivalent folded literal"
                ),
                other => panic!("{name} should compile as one folded literal, got {other:?}"),
            }

            let bytes = compile_value_function(&program).expect("compile folded binary math leaf");
            assert!(
                !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
                "folded binary math should stay helper-free"
            );

            let memory =
                ExecutableMemory::allocate(&bytes).expect("allocate folded binary math leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);

            assert_eq!(
                f(&ctx, std::ptr::null()).to_bits(),
                expected.to_bits(),
                "{name}"
            );
        }
    }

    #[test]
    fn generated_value_leaf_spills_max_depth_across_helper_call() {
        let input = 0.25;
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushConst(1.0),
                Instruction::PushConst(2.0),
                Instruction::PushConst(3.0),
                Instruction::PushConst(4.0),
                Instruction::PushConst(5.0),
                Instruction::PushConst(input),
                Instruction::Exp,
                Instruction::Add,
                Instruction::Add,
                Instruction::Add,
                Instruction::Add,
                Instruction::Add,
            ],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile max-depth helper-call leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate max-depth helper leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[], &[], &[]);

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            (1.0 + 2.0 + 3.0 + 4.0 + 5.0 + runtime_exp(input)).to_bits()
        );
    }

    #[test]
    fn generated_value_leaf_spills_max_depth_across_binary_helper_call() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushConst(1.0),
                Instruction::PushConst(2.0),
                Instruction::PushConst(3.0),
                Instruction::PushConst(4.0),
                Instruction::PushConst(5.0),
                Instruction::PushConst(2.0),
                Instruction::Pow,
                Instruction::Add,
                Instruction::Add,
                Instruction::Add,
                Instruction::Add,
            ],
            0,
        );
        let bytes =
            compile_value_function(&program).expect("compile max-depth binary helper-call leaf");
        let memory =
            ExecutableMemory::allocate(&bytes).expect("allocate max-depth binary helper leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[], &[], &[]);

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            (1.0 + (2.0 + (3.0 + (4.0 + runtime_pow(5.0, 2.0))))).to_bits()
        );
    }

    #[test]
    fn generated_value_leaf_runs_from_nonzero_concatenated_image_offset() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushConst(12.0),
                Instruction::PushConst(0.5),
                Instruction::Add,
            ],
            0,
        );
        let function = compile_value_function(&program).expect("compile literal value function");
        let prefix = [0xC3_u8];
        let mut image = prefix.to_vec();
        image.extend_from_slice(&function);

        let memory = ExecutableMemory::allocate(&image).expect("allocate concatenated image");
        let entry = memory
            .ptr_at(prefix.len())
            .expect("entry point inside concatenated image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[], &[], &[]);

        assert_eq!(f(&ctx, std::ptr::null()), 12.5);
    }

    #[test]
    fn rejects_variable_index_that_exceeds_disp32_range() {
        let too_large_index = (i32::MAX as usize / std::mem::size_of::<f64>()) + 1;
        let program = NativeProgram::from_bytecode(
            "x64-codegen-test",
            EntryKind::StampValue,
            &BytecodeProgram {
                instructions: vec![Instruction::PushVariable(too_large_index)],
            },
            NativeLoweringLimits::new(0, 0, 0, too_large_index + 1, 0),
        )
        .expect("large variable index is valid IR before x64 disp32 lowering");

        let error = compile_value_function(&program)
            .expect_err("large variable index must not truncate displacement");

        assert!(matches!(error, crate::native::JitError::Encoding { .. }));
        assert!(error.to_string().contains("disp32"));
    }

    #[test]
    fn generated_value_leaf_negates_at_full_xmm_stack_depth() {
        let prefix = constant_prefix(XMM_STACK.len() - 1);
        let mut instructions = prefix.clone();
        instructions.extend([Instruction::PushTemperature, Instruction::Neg]);
        instructions.extend(add_reductions(prefix.len()));
        let program = native_program(EntryKind::StampValue, instructions, 0);

        assert_eq!(program.max_stack_depth(), XMM_STACK.len());

        let bytes = compile_value_function(&program).expect("compile full-stack dynamic neg leaf");
        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "dynamic neg should remain helper-free at full XMM stack depth"
        );

        let memory =
            ExecutableMemory::allocate(&bytes).expect("allocate full-stack dynamic neg leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.temperature = 6.0;

        assert_eq!(
            f(&ctx, std::ptr::null()).to_bits(),
            (constant_prefix_sum(prefix.len()) - 6.0).to_bits()
        );
    }

    #[test]
    fn generated_value_leaf_loads_differential_voltage_at_full_xmm_stack_depth_without_scratch() {
        let prefix = variable_prefix(0, XMM_STACK.len() - 1);
        let mut instructions = prefix.clone();
        instructions.push(Instruction::PushVoltage(0, 1));
        instructions.extend(add_reductions(prefix.len()));
        let program = native_program(EntryKind::StampValue, instructions, 2);

        assert_eq!(program.max_stack_depth(), XMM_STACK.len());

        let bytes =
            compile_value_function(&program).expect("compile full-stack differential voltage leaf");
        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "differential voltage load should stay helper-free at full XMM stack depth"
        );
        assert_eq!(
            count_bytes(&bytes, &context_pointer_load_bytes(VOLTAGES_OFFSET)),
            1,
            "full-stack differential voltage load should reuse one terminal voltage base pointer"
        );

        let memory = ExecutableMemory::allocate(&bytes).expect("allocate full-stack voltage leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[9.0, 4.0], &[], &[]);
        let vars: Vec<f64> = (0..prefix.len()).map(|index| (index + 1) as f64).collect();

        assert_eq!(
            f(&ctx, vars.as_ptr()).to_bits(),
            (constant_prefix_sum(prefix.len()) + 5.0).to_bits()
        );
    }

    #[test]
    #[ignore = "release-only direct native x64 throughput probe; run with --release --features native -- --ignored --nocapture"]
    fn native_x64_microbench_reports_direct_call_throughput() {
        assert!(
            !cfg!(debug_assertions),
            "native x64 microbench is release-only; rerun with --release"
        );
        let iterations = native_microbench_iterations();
        let samples = native_microbench_samples();
        eprintln!("native-x64-microbench iterations={iterations} samples={samples}");

        let params = [1.25_f64, 3.5, 13.75, 6.25, 0.5, 1.5, 2.5, 3.5];
        let vars = [7.0_f64, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0];
        let dyn_vars = [2.0_f64, 10.0, 20.0, 30.0, 40.0];
        let voltages = [9.0_f64, 4.0, 12.0, 2.0];
        let branch_currents = [f64::NAN, 4.0_f64, 6.5_f64];
        let mut ctx = eval_context(&params, &voltages, &[], &[]);
        ctx.temperature = 300.0;
        ctx.branch_currents = branch_currents.as_ptr();
        ctx.branch_currents_len = branch_currents.len();

        run_native_value_microbench(
            "constant_leaf",
            native_program(EntryKind::StampValue, vec![Instruction::PushConst(1.0)], 0),
            &ctx,
            std::ptr::null(),
            iterations,
            samples,
            1.0,
        );
        run_native_value_microbench(
            "arithmetic_context",
            native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushParam(0),
                    Instruction::PushParam(1),
                    Instruction::Mul,
                    Instruction::PushVariable(0),
                    Instruction::Add,
                    Instruction::PushVoltage(0, 1),
                    Instruction::Add,
                    Instruction::PushTemperature,
                    Instruction::PushConst(0.01),
                    Instruction::Mul,
                    Instruction::Add,
                ],
                2,
            ),
            &ctx,
            vars.as_ptr(),
            iterations,
            samples,
            19.375,
        );
        run_native_value_microbench(
            "param_variable_dot8",
            native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushParam(0),
                    Instruction::PushVariable(0),
                    Instruction::Mul,
                    Instruction::PushParam(1),
                    Instruction::PushVariable(1),
                    Instruction::Mul,
                    Instruction::Add,
                    Instruction::PushParam(2),
                    Instruction::PushVariable(2),
                    Instruction::Mul,
                    Instruction::Add,
                    Instruction::PushParam(3),
                    Instruction::PushVariable(3),
                    Instruction::Mul,
                    Instruction::Add,
                    Instruction::PushParam(4),
                    Instruction::PushVariable(4),
                    Instruction::Mul,
                    Instruction::Add,
                    Instruction::PushParam(5),
                    Instruction::PushVariable(5),
                    Instruction::Mul,
                    Instruction::Add,
                    Instruction::PushParam(6),
                    Instruction::PushVariable(6),
                    Instruction::Mul,
                    Instruction::Add,
                    Instruction::PushParam(7),
                    Instruction::PushVariable(7),
                    Instruction::Mul,
                    Instruction::Add,
                ],
                0,
            ),
            &ctx,
            vars.as_ptr(),
            iterations,
            samples,
            996.25,
        );
        run_native_value_microbench(
            "same_storage_voltage_pair",
            native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushVoltage(0, 1),
                    Instruction::PushVoltage(2, 3),
                    Instruction::Add,
                ],
                4,
            ),
            &ctx,
            std::ptr::null(),
            iterations,
            samples,
            15.0,
        );
        run_native_value_microbench(
            "dynamic_index_inline",
            native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushVariable(0),
                    Instruction::PushVariableDyn {
                        base: 1,
                        len: 4,
                        lower: 0,
                    },
                    Instruction::PushParam(0),
                    Instruction::Add,
                ],
                0,
            ),
            &ctx,
            dyn_vars.as_ptr(),
            iterations,
            samples,
            31.25,
        );
        run_native_value_microbench(
            "integer_bitwise",
            native_program(
                EntryKind::StampValue,
                vec![
                    Instruction::PushParam(2),
                    Instruction::PushParam(3),
                    Instruction::BitAnd,
                    Instruction::PushVariable(0),
                    Instruction::Add,
                ],
                0,
            ),
            &ctx,
            vars.as_ptr(),
            iterations,
            samples,
            11.0,
        );
        run_native_value_microbench(
            "guarded_current_pair",
            NativeProgram::from_ops_for_test(
                vec![
                    NativeOp::LoadCurrent(1),
                    NativeOp::LoadCurrent(2),
                    NativeOp::Add,
                ],
                2,
                vec![1, 2],
                Vec::new(),
            ),
            &ctx,
            std::ptr::null(),
            iterations,
            samples,
            10.5,
        );

        let state_vars = [2.0_f64];
        let previous_state = [0.0_f64, 1.5_f64];
        let mut state_values = [0.0_f64, 0.0_f64];
        let mut state_ctx = eval_context(&[], &[], &[], &[]);
        state_ctx.timestep = 0.25;
        state_ctx.state_prev = previous_state.as_ptr();
        state_ctx.state_prev_len = previous_state.len();
        state_ctx.state_values = state_values.as_mut_ptr();
        state_ctx.state_values_len = state_values.len();
        run_native_value_microbench(
            "stateful_ddt",
            native_program(
                EntryKind::StampValue,
                vec![Instruction::PushVariable(0), Instruction::DdtState(1)],
                0,
            ),
            &state_ctx,
            state_vars.as_ptr(),
            iterations,
            samples,
            2.0,
        );
    }

    fn run_native_value_microbench(
        name: &str,
        program: NativeProgram,
        ctx: &EvalContext,
        vars: *const f64,
        iterations: usize,
        samples: usize,
        expected: f64,
    ) {
        let bytes = compile_value_function(&program).expect("compile native microbench leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate native microbench leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        assert_f64_matches(f(ctx, vars), expected, name);

        let warmup_iterations = iterations / 10;
        let warmup_checksum = run_native_microbench_sample(f, ctx, vars, warmup_iterations.max(1));
        assert!(
            warmup_checksum.is_finite(),
            "{name}: native microbench warmup checksum must stay finite"
        );

        let mut sample_ns_per_eval = Vec::with_capacity(samples);
        let mut checksum = 0.0_f64;
        for _ in 0..samples {
            let start = std::time::Instant::now();
            checksum += run_native_microbench_sample(f, ctx, vars, iterations);
            let elapsed = start.elapsed();
            sample_ns_per_eval.push(elapsed.as_nanos() as f64 / iterations as f64);
        }
        sample_ns_per_eval.sort_by(|left, right| left.total_cmp(right));
        let min_ns_per_eval = sample_ns_per_eval[0];
        let median_ns_per_eval = sample_ns_per_eval[sample_ns_per_eval.len() / 2];
        let checksum = std::hint::black_box(checksum);
        assert!(
            checksum.is_finite(),
            "{name}: native microbench checksum must stay finite"
        );
        eprintln!(
            "native-x64-microbench case={name} code_bytes={} min_ns_per_eval={min_ns_per_eval:.3} median_ns_per_eval={median_ns_per_eval:.3} checksum={checksum:.17e}",
            bytes.len(),
        );

        std::hint::black_box(memory);
    }

    fn run_native_microbench_sample(
        f: extern "C" fn(*const EvalContext, *const f64) -> f64,
        ctx: &EvalContext,
        vars: *const f64,
        iterations: usize,
    ) -> f64 {
        let mut checksum = 0.0_f64;
        for _ in 0..iterations {
            checksum += std::hint::black_box(f(
                std::hint::black_box(ctx as *const EvalContext),
                std::hint::black_box(vars),
            ));
        }
        std::hint::black_box(checksum)
    }

    fn native_microbench_iterations() -> usize {
        5_000_000
    }

    fn native_microbench_samples() -> usize {
        5
    }

    fn native_program(
        entry_kind: EntryKind,
        instructions: Vec<Instruction>,
        terminal_count: usize,
    ) -> NativeProgram {
        native_program_with_internals(entry_kind, instructions, terminal_count, 0)
    }

    fn native_program_with_internals(
        entry_kind: EntryKind,
        instructions: Vec<Instruction>,
        terminal_count: usize,
        internal_node_count: usize,
    ) -> NativeProgram {
        let storage_limit = test_storage_limit();
        NativeProgram::from_bytecode(
            "x64-codegen-test",
            entry_kind,
            &BytecodeProgram { instructions },
            NativeLoweringLimits::new(
                terminal_count,
                internal_node_count,
                storage_limit,
                storage_limit,
                storage_limit,
            )
            .with_lookup_table_count(8),
        )
        .expect("lower bytecode to native program")
    }

    fn native_program_with_available_current_pairs(
        entry_kind: EntryKind,
        instructions: Vec<Instruction>,
        terminal_count: usize,
        available_current_pairs: &[usize],
    ) -> NativeProgram {
        let storage_limit = test_storage_limit();
        NativeProgram::from_bytecode(
            "x64-codegen-test",
            entry_kind,
            &BytecodeProgram { instructions },
            NativeLoweringLimits::new(
                terminal_count,
                0,
                storage_limit,
                storage_limit,
                storage_limit,
            )
            .with_lookup_table_count(8)
            .with_available_current_pairs(available_current_pairs),
        )
        .expect("lower bytecode to native program")
    }

    fn test_storage_limit() -> usize {
        XMM_STACK.len().max(8) + 1
    }

    static ABI_EXPECTED_TABLE_PTR: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    fn run_table_helper_sentinel(helper: TableHelper, ctx: &EvalContext) -> f64 {
        let mut compiler = abi_sentinel_compiler();
        push_abi_sentinel_const(&mut compiler, 3.25);
        compiler
            .emit_table_helper_call(7, helper)
            .expect("emit table helper ABI sentinel");
        run_abi_sentinel_value(compiler, ctx)
    }

    fn run_context_filter_helper_sentinel(helper: ContextFilterHelper, ctx: &EvalContext) -> f64 {
        let mut compiler = abi_sentinel_compiler();
        let target = push_abi_sentinel_const(&mut compiler, 4.5);
        compiler
            .emit_context_filter_helper_call(target, 9, helper)
            .expect("emit context filter helper ABI sentinel");
        run_abi_sentinel_value(compiler, ctx)
    }

    fn run_timer_helper_sentinel(helper: TimerHelper, ctx: &EvalContext) -> f64 {
        let mut compiler = abi_sentinel_compiler();
        let start = push_abi_sentinel_const(&mut compiler, 1.25);
        let period = push_abi_sentinel_const(&mut compiler, 0.75);
        compiler.emit_timer_helper_call(start, period, helper);
        run_abi_sentinel_value(compiler, ctx)
    }

    fn run_operand_context_filter_helper_sentinel(
        helper: OperandContextFilterHelper,
        ctx: &EvalContext,
    ) -> f64 {
        let mut compiler = abi_sentinel_compiler();
        push_abi_sentinel_const(&mut compiler, 99.0);
        let input = push_abi_sentinel_const(&mut compiler, 2.0);
        push_abi_sentinel_const(&mut compiler, 3.0);
        push_abi_sentinel_const(&mut compiler, 4.0);
        compiler.emit_operand_context_filter_helper_call(input, 3, 11, helper);
        compiler.encoder.movsd_xmm_xmm(Xmm::Xmm0, input);
        run_abi_sentinel_value(compiler, ctx)
    }

    fn abi_sentinel_compiler() -> FunctionCompiler {
        FunctionCompiler::new(true, false, 0, None, None, 0)
    }

    fn push_abi_sentinel_const(compiler: &mut FunctionCompiler, value: f64) -> Xmm {
        let register = compiler.push_register().expect("ABI sentinel stack slot");
        compiler.emit_constant_load(register, value);
        register
    }

    fn run_abi_sentinel_value(compiler: FunctionCompiler, ctx: &EvalContext) -> f64 {
        let bytes = compiler
            .finish_value_function()
            .expect("finish ABI sentinel value function");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate ABI sentinel function");
        let entry = memory.ptr_at(0).expect("ABI sentinel entry point");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        f(ctx, std::ptr::null())
    }

    unsafe extern "C" fn abi_sentinel_table_helper(
        input: f64,
        tables: *const LookupTable,
        table_count: usize,
        table_id: usize,
    ) -> f64 {
        if input.to_bits() != 3.25_f64.to_bits() {
            return -1.0;
        }
        let expected = ABI_EXPECTED_TABLE_PTR.load(std::sync::atomic::Ordering::SeqCst);
        if tables as usize != expected {
            return -2.0;
        }
        if table_count != 2 {
            return -3.0;
        }
        if table_id != 7 {
            return -4.0;
        }
        101.0
    }

    unsafe extern "C" fn abi_sentinel_context_filter_helper(
        input: f64,
        ctx: *const EvalContext,
        filter_id: usize,
    ) -> f64 {
        if input.to_bits() != 4.5_f64.to_bits() {
            return -10.0;
        }
        if filter_id != 9 {
            return -11.0;
        }
        let Some(ctx) = (unsafe { ctx.as_ref() }) else {
            return -12.0;
        };
        if ctx.time.to_bits() != 12.0_f64.to_bits()
            || ctx.temperature.to_bits() != 315.0_f64.to_bits()
            || ctx.multiplicity.to_bits() != 3.0_f64.to_bits()
        {
            return -13.0;
        }
        202.0
    }

    unsafe extern "C" fn abi_sentinel_timer_helper(
        start: f64,
        period: f64,
        ctx: *const EvalContext,
    ) -> f64 {
        if start.to_bits() != 1.25_f64.to_bits() {
            return -20.0;
        }
        if period.to_bits() != 0.75_f64.to_bits() {
            return -21.0;
        }
        let Some(ctx) = (unsafe { ctx.as_ref() }) else {
            return -22.0;
        };
        if ctx.time.to_bits() != 12.0_f64.to_bits()
            || ctx.multiplicity.to_bits() != 3.0_f64.to_bits()
        {
            return -23.0;
        }
        303.0
    }

    unsafe extern "C" fn abi_sentinel_operand_context_filter_helper(
        operands: *const f64,
        ctx: *const EvalContext,
        filter_id: usize,
    ) -> f64 {
        if filter_id != 11 {
            return -30.0;
        }
        let Some(ctx) = (unsafe { ctx.as_ref() }) else {
            return -31.0;
        };
        if ctx.temperature.to_bits() != 315.0_f64.to_bits()
            || ctx.multiplicity.to_bits() != 3.0_f64.to_bits()
        {
            return -32.0;
        }
        if operands.is_null() {
            return -33.0;
        }
        let operands = unsafe { std::slice::from_raw_parts(operands, 3) };
        if operands[0].to_bits() != 2.0_f64.to_bits()
            || operands[1].to_bits() != 3.0_f64.to_bits()
            || operands[2].to_bits() != 4.0_f64.to_bits()
        {
            return -34.0;
        }
        404.0
    }

    fn stack_alignment_helper_memory() -> ExecutableMemory {
        let mut encoder = X64Encoder::new();
        encoder.mov_r64_r64(Gpr::Rax, Gpr::Rsp);
        encoder.mov_r64_imm32(Gpr::R11, 15);
        encoder.and_r64_r64(Gpr::Rax, Gpr::R11);
        encoder.cmp_r64_imm32(Gpr::Rax, 8);
        encoder.setcc_r8(ConditionCode::Equal, Gpr::R10);
        encoder.movzx_r32_r8(Gpr::R10, Gpr::R10);
        encoder.cvtsi2sd_xmm_r32(Xmm::Xmm0, Gpr::R10);
        encoder.ret();
        ExecutableMemory::allocate(&encoder.into_bytes()).expect("allocate stack helper sentinel")
    }

    fn eval_context(
        params: &[f64],
        voltages: &[f64],
        internal_voltages: &[f64],
        branch_unknowns: &[f64],
    ) -> EvalContext {
        EvalContext {
            voltages: voltages.as_ptr(),
            internal_voltages: internal_voltages.as_ptr(),
            params: params.as_ptr(),
            branch_currents: std::ptr::null(),
            branch_currents_len: 0,
            currents: std::ptr::null(),
            currents_len: 0,
            num_terminals: 0,
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
            branch_unknowns: branch_unknowns.as_ptr(),
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
        }
    }

    fn contains_bytes(bytes: &[u8], needle: &[u8]) -> bool {
        bytes.windows(needle.len()).any(|window| window == needle)
    }

    fn find_bytes(bytes: &[u8], needle: &[u8]) -> Option<usize> {
        bytes
            .windows(needle.len())
            .position(|window| window == needle)
    }

    fn assert_f64_matches(actual: f64, expected: f64, context: &str) {
        if expected.is_nan() {
            assert!(
                actual.is_nan(),
                "{context}: expected NaN, got {actual:?} ({:#x})",
                actual.to_bits()
            );
        } else {
            assert_eq!(actual.to_bits(), expected.to_bits(), "{context}");
        }
    }

    fn sub_rsp_bytes(value: i32) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.sub_rsp_imm32(value);
        encoder.into_bytes()
    }

    fn add_rsp_bytes(value: i32) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.add_rsp_imm32(value);
        encoder.into_bytes()
    }

    fn addsd_xmm_xmm_bytes(dst: Xmm, src: Xmm) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.addsd_xmm_xmm(dst, src);
        encoder.into_bytes()
    }

    fn mulsd_xmm_m64_rip_prefix_bytes(dst: Xmm) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.mulsd_xmm_m64_rip_disp32(dst, 0);
        let mut bytes = encoder.into_bytes();
        bytes.truncate(bytes.len() - std::mem::size_of::<i32>());
        bytes
    }

    fn subsd_xmm_m64_rip_prefix_bytes(dst: Xmm) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.subsd_xmm_m64_rip_disp32(dst, 0);
        let mut bytes = encoder.into_bytes();
        bytes.truncate(bytes.len() - std::mem::size_of::<i32>());
        bytes
    }

    fn stack_spill_store_bytes(register: Xmm) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.movsd_m64_base_disp32_xmm(Gpr::Rsp, 0, register);
        encoder.into_bytes()
    }

    fn stack_spill_minsd_bytes(register: Xmm) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.minsd_xmm_m64_base_disp32(register, Gpr::Rsp, 0);
        encoder.into_bytes()
    }

    fn call_rax_bytes() -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.call_r64(Gpr::Rax);
        encoder.into_bytes()
    }

    fn call_frame_bytes(slot_count: usize) -> i32 {
        super::call_frame_bytes_for_slots(slot_count)
    }

    fn old_fixed_call_frame_bytes() -> i32 {
        call_frame_bytes(XMM_STACK.len() + 1)
    }

    fn constant_prefix(count: usize) -> Vec<Instruction> {
        (0..count)
            .map(|index| Instruction::PushConst((index + 1) as f64))
            .collect()
    }

    fn variable_prefix(start: usize, count: usize) -> Vec<Instruction> {
        (0..count)
            .map(|index| Instruction::PushVariable(start + index))
            .collect()
    }

    fn vars_with_prefix(first: f64, count: usize) -> Vec<f64> {
        let mut vars = Vec::with_capacity(count + 1);
        vars.push(first);
        vars.extend((0..count).map(|index| (index + 1) as f64));
        vars
    }

    fn add_reductions(count: usize) -> Vec<Instruction> {
        (0..count).map(|_| Instruction::Add).collect()
    }

    fn constant_prefix_sum(count: usize) -> f64 {
        (count * (count + 1) / 2) as f64
    }

    fn count_bytes(bytes: &[u8], needle: &[u8]) -> usize {
        bytes
            .windows(needle.len())
            .filter(|window| *window == needle)
            .count()
    }

    fn context_pointer_load_bytes(ctx_field_offset: i32) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.mov_r64_m64_base_disp32(Gpr::Rax, entry_ctx_arg_reg(), ctx_field_offset);
        encoder.into_bytes()
    }

    fn old_disp32_context_pointer_load_bytes(ctx_field_offset: i32) -> Vec<u8> {
        let modrm = match entry_ctx_arg_reg() {
            Gpr::Rcx => 0x81,
            Gpr::Rdi => 0x87,
            _ => unreachable!("native entry context arg register is fixed by the host ABI"),
        };
        let mut bytes = vec![0x48, 0x8B, modrm];
        bytes.extend_from_slice(&ctx_field_offset.to_le_bytes());
        bytes
    }

    fn dynamic_variable_scaled_address_bytes(base: usize) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.lea_r64_base_index_scale8_disp32(
            Gpr::R11,
            entry_vars_arg_reg(),
            Gpr::R10,
            super::byte_disp(base).expect("dynamic address test index fits disp32"),
        );
        encoder.into_bytes()
    }

    fn dynamic_variable_shift_add_address_bytes(base: usize) -> Vec<u8> {
        let base_disp = super::byte_disp(base).expect("dynamic address test index fits disp32");
        let mut encoder = X64Encoder::new();
        encoder.mov_r64_r64(Gpr::Rax, Gpr::R10);
        encoder.shl_r64_imm8(Gpr::Rax, 3);
        encoder.mov_r64_r64(Gpr::R11, entry_vars_arg_reg());
        if base_disp != 0 {
            encoder.add_r64_imm32(Gpr::R11, base_disp);
        }
        encoder.add_r64_r64(Gpr::Rax, Gpr::R11);
        encoder.into_bytes()
    }

    fn sub_r64_imm32_bytes(register: Gpr, value: i32) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.sub_r64_imm32(register, value);
        encoder.into_bytes()
    }

    fn cmp_r64_imm32_bytes(register: Gpr, value: i32) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.cmp_r64_imm32(register, value);
        encoder.into_bytes()
    }

    fn shift_imm_bytes(op: IntegerBinaryOp, register: Gpr, count: u8) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        match op {
            IntegerBinaryOp::Shl => encoder.shl_r64_imm8(register, count),
            IntegerBinaryOp::Shr => encoder.sar_r64_imm8(register, count),
            IntegerBinaryOp::BitAnd | IntegerBinaryOp::BitOr | IntegerBinaryOp::BitXor => {
                unreachable!("shift byte helper only accepts shl/shr")
            }
        }
        encoder.into_bytes()
    }

    fn bitwise_imm32_bytes(op: IntegerBinaryOp, register: Gpr, value: i32) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        match op {
            IntegerBinaryOp::BitAnd => encoder.and_r64_imm32(register, value),
            IntegerBinaryOp::BitOr => encoder.or_r64_imm32(register, value),
            IntegerBinaryOp::BitXor => encoder.xor_r64_imm32(register, value),
            IntegerBinaryOp::Shl | IntegerBinaryOp::Shr => {
                unreachable!("bitwise byte helper only accepts bitwise ops")
            }
        }
        encoder.into_bytes()
    }

    fn dynamic_variable_movabs_sub_lower_bytes(lower: i64) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.movabs_r64_imm64(Gpr::R11, lower as u64);
        encoder.sub_r64_r64(Gpr::R10, Gpr::R11);
        encoder.into_bytes()
    }

    fn dynamic_variable_movabs_cmp_len_bytes(len: usize) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.movabs_r64_imm64(Gpr::R11, len as u64);
        encoder.cmp_r64_r64(Gpr::R10, Gpr::R11);
        encoder.into_bytes()
    }

    fn guarded_slice_index_cmp_imm32_bytes(index: usize) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.cmp_r64_imm32(
            Gpr::R10,
            super::slice_index_imm32(index).expect("guarded slice test index fits imm32"),
        );
        encoder.into_bytes()
    }

    fn guarded_slice_index_cmp_register_bytes(index: usize) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.movabs_r64_imm64(Gpr::R11, index as u64);
        encoder.cmp_r64_r64(Gpr::R10, Gpr::R11);
        encoder.into_bytes()
    }

    fn mov_r32_imm32_bytes(register: Gpr, value: u32) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.mov_r32_imm32(register, value);
        encoder.into_bytes()
    }

    fn mov_r64_imm32_bytes(register: Gpr, value: i32) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.mov_r64_imm32(register, value);
        encoder.into_bytes()
    }

    fn movabs_imm64_bytes(register: Gpr, value: u64) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.movabs_r64_imm64(register, value);
        encoder.into_bytes()
    }

    fn xor_r64_bytes(dst: Gpr, src: Gpr) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.xor_r64_r64(dst, src);
        encoder.into_bytes()
    }

    fn mov_r11_rsp_bytes() -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.mov_r64_r64(Gpr::R11, Gpr::Rsp);
        encoder.into_bytes()
    }

    fn xorpd_xmm_bytes(dst: Xmm, src: Xmm) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.xorpd_xmm_xmm(dst, src);
        encoder.into_bytes()
    }

    fn andpd_rip_prefix_bytes(target: Xmm) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.andpd_xmm_m128_rip_disp32(target, 0);
        encoder.into_bytes()[..4].to_vec()
    }

    fn abs_value_mask_bytes() -> Vec<u8> {
        let mut bytes = Vec::with_capacity(16);
        bytes.extend_from_slice(&ABS_VALUE_MASK_LOW.to_le_bytes());
        bytes.extend_from_slice(&ABS_VALUE_MASK_HIGH.to_le_bytes());
        bytes
    }

    fn scalar_abs_integer_bit_clear_bytes(target: Xmm, scratch: Gpr) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.movq_r64_xmm(scratch, target);
        encoder.btr_r64_imm8(scratch, 63);
        encoder.movq_xmm_r64(target, scratch);
        encoder.into_bytes()
    }

    fn call_frame_spill_bytes(index: usize, register: Xmm) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.movsd_m64_base_disp32_xmm(Gpr::Rsp, super::call_spill_disp(index), register);
        encoder.into_bytes()
    }

    fn call_frame_load_bytes(register: Xmm, index: usize) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.movsd_xmm_m64_base_disp32(register, Gpr::Rsp, super::call_spill_disp(index));
        encoder.into_bytes()
    }

    fn callee_saved_xmm_store_bytes(register: Xmm, disp: i32) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.movdqu_m128_base_disp32_xmm(Gpr::Rsp, disp, register);
        encoder.into_bytes()
    }

    fn callee_saved_xmm_load_bytes(register: Xmm, disp: i32) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.movdqu_xmm_m128_base_disp32(register, Gpr::Rsp, disp);
        encoder.into_bytes()
    }

    fn same_storage_voltage_memory_subtract_bytes(index: usize) -> Vec<u8> {
        let mut encoder = X64Encoder::new();
        encoder.subsd_xmm_m64_base_disp32(
            Xmm::Xmm0,
            Gpr::Rax,
            super::byte_disp(index).expect("same-node test index fits disp32"),
        );
        encoder.into_bytes()
    }

    fn assert_missing_state_storage_error(error: &str) {
        assert!(
            error.contains("missing state storage"),
            "error must identify missing native state storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    fn assert_state_storage_bounds_error(error: &str) {
        assert!(
            error.contains("index outside state storage"),
            "error must identify out-of-range native state storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    fn assert_limit_state_storage_bounds_error(error: &str) {
        assert!(
            error.contains("limit state index outside state storage"),
            "error must identify out-of-range limit state storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    fn assert_prior_state_storage_bounds_error(error: &str) {
        assert!(
            error.contains("index outside prior-state storage"),
            "error must identify out-of-range native prior-state storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    fn assert_current_probe_error(error: &str) {
        assert!(
            error.contains("missing terminal-pair current storage"),
            "error must identify missing native current-probe storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    fn assert_prior_current_error(error: &str) {
        assert!(
            error.contains("missing contribution current storage"),
            "error must identify missing native prior-current storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    fn assert_param_given_error(error: &str) {
        assert!(
            error.contains("missing parameter-given storage"),
            "error must identify missing native param_given storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    fn assert_port_connected_error(error: &str) {
        assert!(
            error.contains("missing connection-flag storage"),
            "error must identify missing native port_connected storage, got: {error}"
        );
        assert!(
            error.contains("no interpreter fallback"),
            "error must preserve the native hard-fail contract, got: {error}"
        );
    }

    fn runtime_min(left: f64, right: f64) -> f64 {
        std::hint::black_box(left).min(std::hint::black_box(right))
    }

    fn runtime_max(left: f64, right: f64) -> f64 {
        std::hint::black_box(left).max(std::hint::black_box(right))
    }

    fn runtime_exp(value: f64) -> f64 {
        std::hint::black_box(value).exp()
    }

    fn runtime_log(value: f64) -> f64 {
        std::hint::black_box(value).ln()
    }

    fn runtime_log10(value: f64) -> f64 {
        std::hint::black_box(value).log10()
    }

    fn runtime_sin(value: f64) -> f64 {
        std::hint::black_box(value).sin()
    }

    fn runtime_cos(value: f64) -> f64 {
        std::hint::black_box(value).cos()
    }

    fn runtime_tan(value: f64) -> f64 {
        std::hint::black_box(value).tan()
    }

    fn runtime_sinh(value: f64) -> f64 {
        std::hint::black_box(value).sinh()
    }

    fn runtime_cosh(value: f64) -> f64 {
        std::hint::black_box(value).cosh()
    }

    fn runtime_tanh(value: f64) -> f64 {
        std::hint::black_box(value).tanh()
    }

    fn runtime_limexp(value: f64) -> f64 {
        const LIMIT: f64 = 40.0;
        let value = std::hint::black_box(value);
        if value > LIMIT {
            let exp_limit = LIMIT.exp();
            exp_limit * (1.0 + value - LIMIT)
        } else if value < -LIMIT {
            (-LIMIT).exp()
        } else {
            value.exp()
        }
    }

    fn runtime_limited_exp(value: f64) -> f64 {
        const LIMIT: f64 = 80.0;
        const LOW_VALUE: f64 = 1.804851387e-35;
        let value = std::hint::black_box(value);
        if value > LIMIT {
            LIMIT.exp() * (1.0 + value - LIMIT)
        } else if value < -LIMIT {
            LOW_VALUE
        } else {
            value.exp()
        }
    }

    fn runtime_asin(value: f64) -> f64 {
        std::hint::black_box(value).asin()
    }

    fn runtime_acos(value: f64) -> f64 {
        std::hint::black_box(value).acos()
    }

    fn runtime_atan(value: f64) -> f64 {
        std::hint::black_box(value).atan()
    }

    fn runtime_floor(value: f64) -> f64 {
        std::hint::black_box(value).floor()
    }

    fn runtime_ceil(value: f64) -> f64 {
        std::hint::black_box(value).ceil()
    }

    fn runtime_pow(left: f64, right: f64) -> f64 {
        std::hint::black_box(left).powf(std::hint::black_box(right))
    }

    fn runtime_atan2(left: f64, right: f64) -> f64 {
        std::hint::black_box(left).atan2(std::hint::black_box(right))
    }

    fn runtime_mod(left: f64, right: f64) -> f64 {
        std::hint::black_box(left) % std::hint::black_box(right)
    }

    fn runtime_shl(left: f64, right: f64) -> f64 {
        ((std::hint::black_box(left) as i64) << (std::hint::black_box(right) as i64)) as f64
    }

    fn runtime_shr(left: f64, right: f64) -> f64 {
        ((std::hint::black_box(left) as i64) >> (std::hint::black_box(right) as i64)) as f64
    }

    fn runtime_bitand(left: f64, right: f64) -> f64 {
        ((std::hint::black_box(left) as i64) & (std::hint::black_box(right) as i64)) as f64
    }

    fn runtime_bitor(left: f64, right: f64) -> f64 {
        ((std::hint::black_box(left) as i64) | (std::hint::black_box(right) as i64)) as f64
    }

    fn runtime_bitxor(left: f64, right: f64) -> f64 {
        ((std::hint::black_box(left) as i64) ^ (std::hint::black_box(right) as i64)) as f64
    }

    fn thermal_voltage(temperature: f64) -> f64 {
        temperature * THERMAL_VOLTAGE_PER_K
    }
}
