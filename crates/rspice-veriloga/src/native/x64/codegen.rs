use super::encoder::{ConditionCode, Gpr, X64Encoder, Xmm};
use crate::native::abi::{
    rspice_absdelay_state_native, rspice_acos, rspice_asin, rspice_atan, rspice_atan2,
    rspice_bitand, rspice_bitor, rspice_bitxor, rspice_ceil, rspice_cos, rspice_cosh,
    rspice_cross_state_native, rspice_dynamic_variable_load_native,
    rspice_dynamic_variable_slot_native, rspice_exp, rspice_floor, rspice_idtmod_wrap,
    rspice_laplace_step_native, rspice_limexp, rspice_log, rspice_log10, rspice_mod,
    rspice_native_loop_limit_error, rspice_pow, rspice_shl, rspice_shr, rspice_sin, rspice_sinh,
    rspice_slew_state_native, rspice_table_derivative_native, rspice_table_lookup_native,
    rspice_tan, rspice_tanh, rspice_timer_state_native, rspice_transition_state_native,
    rspice_zi_step_native,
};
use crate::native::expr::{BinaryMathOp, IntegerBinaryOp, UnaryMathOp};
use crate::native::expr::{CompareOp, ExtremumOp, LogicalOp, NativeOp, NativeProgram, VoltageNode};
use crate::native::{JitError, JitResult};

const MODEL: &str = "native-x64";
const VOLTAGES_OFFSET: i32 = 0;
const INTERNAL_VOLTAGES_OFFSET: i32 = 8;
const PARAMS_OFFSET: i32 = 16;
const BRANCH_CURRENTS_OFFSET: i32 = 24;
const PORT_CONNECTED_OFFSET: i32 = 64;
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
const BRANCH_UNKNOWNS_OFFSET: i32 = 176;
const ANALYSIS_TYPE_OFFSET: i32 = 184;
const MFACTOR_OFFSET: i32 = 192;
const WORD_BYTES: usize = std::mem::size_of::<f64>();
const K_BOLTZMANN: f64 = 1.380649e-23;
const Q_ELECTRON: f64 = 1.602176634e-19;
const BOOLEAN_EPSILON: f64 = 1.0e-15;
const TIMESTEP_DC_EPSILON: f64 = 1.0e-20;
const CALL_SPILL_SLOT_COUNT: usize = 7;
const CALL_RESULT_SLOT: usize = 6;
const CALL_FRAME_BYTES: i32 = CALL_SHADOW_BYTES + (CALL_SPILL_SLOT_COUNT * WORD_BYTES) as i32;
#[cfg(windows)]
const CALL_SHADOW_BYTES: i32 = 32;
#[cfg(not(windows))]
const CALL_SHADOW_BYTES: i32 = 0;
const LOCAL_SLOT_BYTES: i32 = 8;
const LOCAL_FRAME_ALIGN_BYTES: i32 = 16;
const INDEXED_ASSIGNMENT_SLOT_PTR_DISP: i32 = 0;
const MAX_RUNTIME_LOOP_ITERATIONS: i32 = 100_000;
const XMM_STACK: [Xmm; 6] = [
    Xmm::Xmm0,
    Xmm::Xmm1,
    Xmm::Xmm2,
    Xmm::Xmm3,
    Xmm::Xmm4,
    Xmm::Xmm5,
];
const SIGN_MASK: f64 = f64::from_bits(0x8000_0000_0000_0000);

#[allow(dead_code)]
pub(crate) fn compile_value_function(program: &NativeProgram) -> JitResult<Vec<u8>> {
    let mut compiler = FunctionCompiler::new(program_uses_helper_calls(program), 0, None);
    compiler.emit_program(program)?;
    compiler.finish_value_function()
}

#[allow(dead_code)]
pub(crate) fn compile_assignment_function(
    var_index: usize,
    program: &NativeProgram,
) -> JitResult<Vec<u8>> {
    let mut compiler = FunctionCompiler::new(program_uses_helper_calls(program), 0, None);
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
    let uses_helper_calls = assignments.iter().any(assignment_uses_helper_calls);
    let indexed_slot_bytes = if has_indexed_assignment {
        LOCAL_SLOT_BYTES
    } else {
        0
    };
    let loop_counter_base_disp = (loop_depth > 0).then_some(indexed_slot_bytes);
    let local_frame_bytes = align_local_frame(indexed_slot_bytes + loop_depth * LOCAL_SLOT_BYTES);

    let mut compiler =
        FunctionCompiler::new(uses_helper_calls, local_frame_bytes, loop_counter_base_disp);
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
    uses_helper_calls: bool,
    local_frame_bytes: i32,
    loop_counter_base_disp: Option<i32>,
    early_return_jumps: Vec<usize>,
}

#[derive(Debug)]
struct LiteralPatch {
    displacement_offset: usize,
    value: f64,
}

impl FunctionCompiler {
    fn new(
        uses_helper_calls: bool,
        local_frame_bytes: i32,
        loop_counter_base_disp: Option<i32>,
    ) -> Self {
        debug_assert_eq!(local_frame_bytes % 16, 0);
        let mut compiler = Self {
            encoder: X64Encoder::new(),
            depth: 0,
            literals: Vec::new(),
            uses_helper_calls,
            local_frame_bytes,
            loop_counter_base_disp,
            early_return_jumps: Vec::new(),
        };
        if compiler.has_prologue() {
            compiler.emit_prologue();
        }
        compiler
    }

    fn has_prologue(&self) -> bool {
        self.uses_helper_calls || self.local_frame_bytes > 0
    }

    fn emit_program(&mut self, program: &NativeProgram) -> JitResult<()> {
        if program.max_stack_depth() > XMM_STACK.len() {
            return Err(register_allocation_error(format!(
                "expression stack depth {} exceeds {} XMM registers",
                program.max_stack_depth(),
                XMM_STACK.len()
            )));
        }

        self.depth = 0;
        for op in program.ops() {
            match *op {
                NativeOp::Const(value) => {
                    let dst = self.push_register()?;
                    self.emit_literal_load(dst, value);
                }
                NativeOp::LoadParam(index) => {
                    let dst = self.push_register()?;
                    self.emit_context_pointer_load(PARAMS_OFFSET);
                    self.encoder
                        .movsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(index)?);
                }
                NativeOp::LoadParamGiven(index) => {
                    self.emit_context_u8_flag_load(PARAM_GIVEN_OFFSET, index)?;
                }
                NativeOp::LoadPortConnected(index) => {
                    self.emit_context_u8_flag_load(PORT_CONNECTED_OFFSET, index)?;
                }
                NativeOp::LoadVoltage { pos, neg } => {
                    self.emit_voltage_load(pos, neg)?;
                }
                NativeOp::LoadCurrent(pair_index) => {
                    let dst = self.push_register()?;
                    self.emit_context_pointer_load(BRANCH_CURRENTS_OFFSET);
                    self.encoder
                        .movsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(pair_index)?);
                }
                NativeOp::LoadInternalVoltage(index) => {
                    let dst = self.push_register()?;
                    self.emit_internal_voltage_load(dst, index)?;
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
                    self.emit_context_pointer_load(BRANCH_UNKNOWNS_OFFSET);
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
                NativeOp::Neg => self.emit_neg()?,
                NativeOp::Abs => self.emit_abs()?,
                NativeOp::Square => self.emit_square()?,
                NativeOp::Sqrt => self.emit_sqrt()?,
                NativeOp::Compare(op) => self.emit_compare(op)?,
                NativeOp::Logical(op) => self.emit_logical(op)?,
                NativeOp::IfElse => self.emit_ifelse()?,
                NativeOp::Extremum(op) => self.emit_extremum(op)?,
                NativeOp::UnaryMath(op) => self.emit_unary_math(op)?,
                NativeOp::BinaryMath(op) => self.emit_binary_math(op)?,
                NativeOp::IntegerBinary(op) => self.emit_integer_binary(op)?,
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
        self.encoder.push_r64(Gpr::R12);
        self.encoder.push_r64(Gpr::R13);
        self.encoder
            .mov_r64_r64(saved_ctx_arg_reg(), entry_ctx_arg_reg());
        self.encoder
            .mov_r64_r64(saved_vars_arg_reg(), entry_vars_arg_reg());
        if self.local_frame_bytes > 0 {
            self.encoder.sub_rsp_imm32(self.local_frame_bytes);
        }
    }

    fn emit_return(&mut self) {
        if self.local_frame_bytes > 0 {
            self.encoder.add_rsp_imm32(self.local_frame_bytes);
        }
        if self.has_prologue() {
            self.encoder.pop_r64(Gpr::R13);
            self.encoder.pop_r64(Gpr::R12);
        }
        self.encoder.ret();
    }

    fn ctx_arg_reg(&self) -> Gpr {
        if self.has_prologue() {
            saved_ctx_arg_reg()
        } else {
            entry_ctx_arg_reg()
        }
    }

    fn vars_arg_reg(&self) -> Gpr {
        if self.has_prologue() {
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
        self.encoder.movabs_r64_imm64(Gpr::R10, 0);
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
        self.emit_dynamic_variable_slot_call(base, len, lower)?;
        self.encoder.test_r64_r64(Gpr::Rax, Gpr::Rax);
        let null_slot = self.encoder.jcc_rel32_placeholder(ConditionCode::Equal);
        self.early_return_jumps.push(null_slot);
        self.encoder
            .mov_m64_base_disp32_r64(Gpr::Rsp, INDEXED_ASSIGNMENT_SLOT_PTR_DISP, Gpr::Rax);

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

    fn emit_neg(&mut self) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "neg requires stack depth 1, found 0".into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        let scratch = self.scratch_register()?;
        self.emit_literal_load(scratch, SIGN_MASK);
        self.encoder.xorpd_xmm_xmm(target, scratch);
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
        self.encoder.movq_r64_xmm(Gpr::Rax, target);
        self.encoder.btr_r64_imm8(Gpr::Rax, 63);
        self.encoder.movq_xmm_r64(target, Gpr::Rax);
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
        self.emit_unary_helper_call(target, unary_math_helper(op));
        Ok(())
    }

    fn emit_unary_helper_call(&mut self, target: Xmm, helper: UnaryHelper) {
        debug_assert!(self.uses_helper_calls);
        self.encoder.sub_rsp_imm32(CALL_FRAME_BYTES);
        self.encoder.mov_r64_r64(Gpr::R11, Gpr::Rsp);
        for (index, register) in XMM_STACK.iter().copied().take(self.depth).enumerate() {
            if register != target {
                self.encoder
                    .movsd_m64_base_disp32_xmm(Gpr::R11, call_spill_disp(index), register);
            }
        }

        if target != Xmm::Xmm0 {
            self.encoder.movsd_xmm_xmm(Xmm::Xmm0, target);
        }
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);

        self.encoder.mov_r64_r64(Gpr::R11, Gpr::Rsp);
        self.encoder
            .movsd_m64_base_disp32_xmm(Gpr::R11, call_result_disp(), Xmm::Xmm0);
        for (index, register) in XMM_STACK.iter().copied().take(self.depth).enumerate() {
            if register != target {
                self.encoder
                    .movsd_xmm_m64_base_disp32(register, Gpr::R11, call_spill_disp(index));
            }
        }
        self.encoder
            .movsd_xmm_m64_base_disp32(target, Gpr::R11, call_result_disp());
        self.encoder.add_rsp_imm32(CALL_FRAME_BYTES);
    }

    fn emit_dynamic_variable_load(&mut self, base: usize, len: usize, lower: i64) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "dynamic variable load requires stack depth 1, found 0".into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        self.emit_dynamic_variable_helper_call(
            target,
            base,
            len,
            lower,
            rspice_dynamic_variable_load_native,
        )
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

        self.encoder.sub_rsp_imm32(CALL_FRAME_BYTES);
        self.encoder.mov_r64_r64(Gpr::R11, Gpr::Rsp);
        for (index, register) in XMM_STACK.iter().copied().take(self.depth).enumerate() {
            if register != target {
                self.encoder
                    .movsd_m64_base_disp32_xmm(Gpr::R11, call_spill_disp(index), register);
            }
        }

        if target != Xmm::Xmm0 {
            self.encoder.movsd_xmm_xmm(Xmm::Xmm0, target);
        }
        self.encoder
            .mov_r64_r64(dynamic_variable_base_arg_reg(), self.vars_arg_reg());
        if base_disp != 0 {
            self.encoder
                .add_r64_imm32(dynamic_variable_base_arg_reg(), base_disp);
        }
        self.encoder
            .movabs_r64_imm64(dynamic_variable_len_arg_reg(), len as u64);
        self.encoder
            .movabs_r64_imm64(dynamic_variable_lower_arg_reg(), lower as u64);
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);

        self.encoder.mov_r64_r64(Gpr::R11, Gpr::Rsp);
        self.encoder
            .movsd_m64_base_disp32_xmm(Gpr::R11, call_result_disp(), Xmm::Xmm0);
        for (index, register) in XMM_STACK.iter().copied().take(self.depth).enumerate() {
            if register != target {
                self.encoder
                    .movsd_xmm_m64_base_disp32(register, Gpr::R11, call_spill_disp(index));
            }
        }
        self.encoder
            .movsd_xmm_m64_base_disp32(target, Gpr::R11, call_result_disp());
        self.encoder.add_rsp_imm32(CALL_FRAME_BYTES);
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

        self.encoder.sub_rsp_imm32(CALL_FRAME_BYTES);
        self.encoder
            .mov_r64_r64(dynamic_variable_base_arg_reg(), self.vars_arg_reg());
        if base_disp != 0 {
            self.encoder
                .add_r64_imm32(dynamic_variable_base_arg_reg(), base_disp);
        }
        self.encoder
            .movabs_r64_imm64(dynamic_variable_len_arg_reg(), len as u64);
        self.encoder
            .movabs_r64_imm64(dynamic_variable_lower_arg_reg(), lower as u64);
        let helper: DynamicVariableSlotHelper = rspice_dynamic_variable_slot_native;
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);
        self.encoder.add_rsp_imm32(CALL_FRAME_BYTES);
        self.depth = 0;
        Ok(())
    }

    fn emit_runtime_loop_limit_error_call(&mut self) {
        debug_assert!(self.uses_helper_calls);
        self.encoder.sub_rsp_imm32(CALL_FRAME_BYTES);
        let helper: VoidHelper = rspice_native_loop_limit_error;
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);
        self.encoder.add_rsp_imm32(CALL_FRAME_BYTES);
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
        self.emit_binary_helper_call(left, right, integer_binary_helper(op));
        self.depth -= 1;
        Ok(())
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
        self.encoder.sub_rsp_imm32(CALL_FRAME_BYTES);
        self.encoder.mov_r64_r64(Gpr::R11, Gpr::Rsp);
        for (index, register) in XMM_STACK.iter().copied().take(self.depth).enumerate() {
            if register != target {
                self.encoder
                    .movsd_m64_base_disp32_xmm(Gpr::R11, call_spill_disp(index), register);
            }
        }

        if target != Xmm::Xmm0 {
            self.encoder.movsd_xmm_xmm(Xmm::Xmm0, target);
        }
        self.encoder.mov_r64_m64_base_disp32(
            table_ptr_arg_reg(),
            self.ctx_arg_reg(),
            LOOKUP_TABLES_OFFSET,
        );
        self.encoder.mov_r64_m64_base_disp32(
            table_len_arg_reg(),
            self.ctx_arg_reg(),
            LOOKUP_TABLES_LEN_OFFSET,
        );
        self.encoder
            .movabs_r64_imm64(table_id_arg_reg(), table_id as u64);
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);

        self.encoder.mov_r64_r64(Gpr::R11, Gpr::Rsp);
        self.encoder
            .movsd_m64_base_disp32_xmm(Gpr::R11, call_result_disp(), Xmm::Xmm0);
        for (index, register) in XMM_STACK.iter().copied().take(self.depth).enumerate() {
            if register != target {
                self.encoder
                    .movsd_xmm_m64_base_disp32(register, Gpr::R11, call_spill_disp(index));
            }
        }
        self.encoder
            .movsd_xmm_m64_base_disp32(target, Gpr::R11, call_result_disp());
        self.encoder.add_rsp_imm32(CALL_FRAME_BYTES);
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
        self.encoder.sub_rsp_imm32(CALL_FRAME_BYTES);
        self.encoder.mov_r64_r64(Gpr::R11, Gpr::Rsp);
        for (index, register) in XMM_STACK.iter().copied().take(self.depth).enumerate() {
            if register != target {
                self.encoder
                    .movsd_m64_base_disp32_xmm(Gpr::R11, call_spill_disp(index), register);
            }
        }

        if target != Xmm::Xmm0 {
            self.encoder.movsd_xmm_xmm(Xmm::Xmm0, target);
        }
        self.encoder
            .mov_r64_r64(context_filter_ctx_arg_reg(), self.ctx_arg_reg());
        self.encoder
            .movabs_r64_imm64(context_filter_id_arg_reg(), filter_id as u64);
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);

        self.encoder.mov_r64_r64(Gpr::R11, Gpr::Rsp);
        self.encoder
            .movsd_m64_base_disp32_xmm(Gpr::R11, call_result_disp(), Xmm::Xmm0);
        for (index, register) in XMM_STACK.iter().copied().take(self.depth).enumerate() {
            if register != target {
                self.encoder
                    .movsd_xmm_m64_base_disp32(register, Gpr::R11, call_spill_disp(index));
            }
        }
        self.encoder
            .movsd_xmm_m64_base_disp32(target, Gpr::R11, call_result_disp());
        self.encoder.add_rsp_imm32(CALL_FRAME_BYTES);
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

        self.encoder.sub_rsp_imm32(WORD_BYTES as i32);
        self.encoder.movsd_m64_base_disp32_xmm(Gpr::Rsp, 0, step);
        self.encoder
            .subsd_xmm_m64_base_disp32(value, Gpr::Rax, state_disp);
        self.encoder.ucomisd_xmm_xmm(value, value);
        let unordered_delta = self.encoder.jcc_rel32_placeholder(ConditionCode::Parity);
        self.encoder.movq_r64_xmm(Gpr::R11, step);
        self.encoder.btc_r64_imm8(Gpr::R11, 63);
        self.encoder.movq_xmm_r64(step, Gpr::R11);
        self.encoder.maxsd_xmm_xmm(value, step);
        self.encoder.minsd_xmm_m64_base_disp32(value, Gpr::Rsp, 0);
        self.encoder
            .addsd_xmm_m64_base_disp32(value, Gpr::Rax, state_disp);
        self.patch_rel32_to_current(unordered_delta)?;
        self.encoder.add_rsp_imm32(WORD_BYTES as i32);

        self.patch_rel32_to_current(first_evaluation)?;
        self.encoder
            .movsd_m64_base_disp32_xmm(Gpr::Rax, state_disp, value);
        self.encoder
            .mov_m8_base_disp32_imm8(Gpr::R10, initialized_disp, 1);
        let done_after_initialized_store = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(no_initialized_flags)?;
        self.patch_rel32_to_current(initialized_flags_out_of_range)?;
        self.patch_rel32_to_current(no_state)?;
        self.patch_rel32_to_current(done_after_initialized_store)?;
        self.depth -= 1;
        Ok(())
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
        let scratch = self.scratch_register()?;
        let state_disp = byte_disp(state_index)?;

        self.emit_state_value_store(state_disp, target)?;

        self.encoder.movsd_xmm_xmm(scratch, target);
        self.emit_context_pointer_load(STATE_PREV_OFFSET);
        self.encoder.test_r64_r64(Gpr::Rax, Gpr::Rax);
        let skip_previous_load = self.encoder.jcc_rel32_placeholder(ConditionCode::Equal);
        self.encoder
            .movsd_xmm_m64_base_disp32(scratch, Gpr::Rax, state_disp);
        self.patch_rel32_to_current(skip_previous_load)?;

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
        let scratch = self.scratch_register()?;
        self.emit_timestep_guarded_scale(target, scratch, BinaryOp::Div)
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
        let scratch = self.scratch_register()?;
        let state_disp = byte_disp(state_index)?;

        self.encoder
            .movsd_xmm_m64_base_disp32(scratch, self.ctx_arg_reg(), TIMESTEP_OFFSET);
        self.encoder.movq_r64_xmm(Gpr::R11, scratch);
        self.emit_abs_register(scratch);
        self.emit_literal_compare(scratch, TIMESTEP_DC_EPSILON);

        let non_dc_path = self.encoder.jcc_rel32_placeholder(ConditionCode::Above);
        self.encoder.movsd_xmm_xmm(value, ic);
        self.emit_state_value_store(state_disp, value)?;
        let done = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(non_dc_path)?;
        self.emit_context_pointer_load(STATE_PREV_OFFSET);
        self.encoder.test_r64_r64(Gpr::Rax, Gpr::Rax);
        let skip_previous_load = self.encoder.jcc_rel32_placeholder(ConditionCode::Equal);
        self.encoder
            .movsd_xmm_m64_base_disp32(ic, Gpr::Rax, state_disp);
        self.patch_rel32_to_current(skip_previous_load)?;

        self.encoder.movq_xmm_r64(scratch, Gpr::R11);
        self.encoder.mulsd_xmm_xmm(value, scratch);
        self.encoder.addsd_xmm_xmm(value, ic);
        self.emit_state_value_store(state_disp, value)?;

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
        let scratch = self.scratch_register()?;
        self.emit_timestep_guarded_scale(target, scratch, BinaryOp::Mul)
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
        let scratch = self.scratch_register()?;
        let state_disp = byte_disp(state_index)?;

        self.encoder
            .movsd_xmm_m64_base_disp32(scratch, self.ctx_arg_reg(), TIMESTEP_OFFSET);
        self.encoder.movq_r64_xmm(Gpr::R11, scratch);
        self.emit_abs_register(scratch);
        self.emit_literal_compare(scratch, TIMESTEP_DC_EPSILON);

        let non_dc_path = self.encoder.jcc_rel32_placeholder(ConditionCode::Above);
        self.encoder.movsd_xmm_xmm(value, ic);
        self.emit_ternary_helper_call(value, modulus, offset, rspice_idtmod_wrap);
        self.emit_state_value_store(state_disp, value)?;
        let done = self.encoder.jmp_rel32_placeholder();

        self.patch_rel32_to_current(non_dc_path)?;
        self.emit_context_pointer_load(STATE_PREV_OFFSET);
        self.encoder.test_r64_r64(Gpr::Rax, Gpr::Rax);
        let skip_previous_load = self.encoder.jcc_rel32_placeholder(ConditionCode::Equal);
        self.encoder
            .movsd_xmm_m64_base_disp32(ic, Gpr::Rax, state_disp);
        self.patch_rel32_to_current(skip_previous_load)?;

        self.encoder.movq_xmm_r64(scratch, Gpr::R11);
        self.encoder.mulsd_xmm_xmm(value, scratch);
        self.encoder.addsd_xmm_xmm(value, ic);
        self.emit_ternary_helper_call(value, modulus, offset, rspice_idtmod_wrap);
        self.emit_state_value_store(state_disp, value)?;

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

    fn emit_state_value_store(&mut self, state_disp: i32, src: Xmm) -> JitResult<()> {
        self.emit_context_pointer_load(STATE_VALUES_OFFSET);
        self.encoder.test_r64_r64(Gpr::Rax, Gpr::Rax);
        let skip_store = self.encoder.jcc_rel32_placeholder(ConditionCode::Equal);
        self.encoder
            .movsd_m64_base_disp32_xmm(Gpr::Rax, state_disp, src);
        self.patch_rel32_to_current(skip_store)
    }

    fn emit_binary_helper_call(&mut self, left: Xmm, right: Xmm, helper: BinaryHelper) {
        debug_assert!(self.uses_helper_calls);
        self.encoder.sub_rsp_imm32(CALL_FRAME_BYTES);
        self.encoder.mov_r64_r64(Gpr::R11, Gpr::Rsp);
        for (index, register) in XMM_STACK.iter().copied().take(self.depth).enumerate() {
            if register != left && register != right {
                self.encoder
                    .movsd_m64_base_disp32_xmm(Gpr::R11, call_spill_disp(index), register);
            }
        }

        if left != Xmm::Xmm0 {
            self.encoder.movsd_xmm_xmm(Xmm::Xmm0, left);
        }
        if right != Xmm::Xmm1 {
            self.encoder.movsd_xmm_xmm(Xmm::Xmm1, right);
        }
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);

        self.encoder.mov_r64_r64(Gpr::R11, Gpr::Rsp);
        self.encoder
            .movsd_m64_base_disp32_xmm(Gpr::R11, call_result_disp(), Xmm::Xmm0);
        for (index, register) in XMM_STACK.iter().copied().take(self.depth).enumerate() {
            if register != left && register != right {
                self.encoder
                    .movsd_xmm_m64_base_disp32(register, Gpr::R11, call_spill_disp(index));
            }
        }
        self.encoder
            .movsd_xmm_m64_base_disp32(left, Gpr::R11, call_result_disp());
        self.encoder.add_rsp_imm32(CALL_FRAME_BYTES);
    }

    fn emit_ternary_helper_call(
        &mut self,
        target: Xmm,
        arg1: Xmm,
        arg2: Xmm,
        helper: TernaryHelper,
    ) {
        debug_assert!(self.uses_helper_calls);
        debug_assert!(xmm_stack_slot(target) < self.depth);
        debug_assert!(xmm_stack_slot(arg1) < self.depth);
        debug_assert!(xmm_stack_slot(arg2) < self.depth);

        self.encoder.sub_rsp_imm32(CALL_FRAME_BYTES);
        self.encoder.mov_r64_r64(Gpr::R11, Gpr::Rsp);
        for (index, register) in XMM_STACK.iter().copied().take(self.depth).enumerate() {
            self.encoder
                .movsd_m64_base_disp32_xmm(Gpr::R11, call_spill_disp(index), register);
        }

        self.encoder.movsd_xmm_m64_base_disp32(
            Xmm::Xmm0,
            Gpr::R11,
            call_spill_disp(xmm_stack_slot(target)),
        );
        self.encoder.movsd_xmm_m64_base_disp32(
            Xmm::Xmm1,
            Gpr::R11,
            call_spill_disp(xmm_stack_slot(arg1)),
        );
        self.encoder.movsd_xmm_m64_base_disp32(
            Xmm::Xmm2,
            Gpr::R11,
            call_spill_disp(xmm_stack_slot(arg2)),
        );
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);

        self.encoder.mov_r64_r64(Gpr::R11, Gpr::Rsp);
        self.encoder
            .movsd_m64_base_disp32_xmm(Gpr::R11, call_result_disp(), Xmm::Xmm0);
        for (index, register) in XMM_STACK.iter().copied().take(self.depth).enumerate() {
            if register != target {
                self.encoder
                    .movsd_xmm_m64_base_disp32(register, Gpr::R11, call_spill_disp(index));
            }
        }
        self.encoder
            .movsd_xmm_m64_base_disp32(target, Gpr::R11, call_result_disp());
        self.encoder.add_rsp_imm32(CALL_FRAME_BYTES);
    }

    fn emit_timer_helper_call(&mut self, start: Xmm, period: Xmm, helper: TimerHelper) {
        debug_assert!(self.uses_helper_calls);
        self.encoder.sub_rsp_imm32(CALL_FRAME_BYTES);
        self.encoder.mov_r64_r64(Gpr::R11, Gpr::Rsp);
        for (index, register) in XMM_STACK.iter().copied().take(self.depth).enumerate() {
            if register != start && register != period {
                self.encoder
                    .movsd_m64_base_disp32_xmm(Gpr::R11, call_spill_disp(index), register);
            }
        }

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

        self.encoder.mov_r64_r64(Gpr::R11, Gpr::Rsp);
        self.encoder
            .movsd_m64_base_disp32_xmm(Gpr::R11, call_result_disp(), Xmm::Xmm0);
        for (index, register) in XMM_STACK.iter().copied().take(self.depth).enumerate() {
            if register != start && register != period {
                self.encoder
                    .movsd_xmm_m64_base_disp32(register, Gpr::R11, call_spill_disp(index));
            }
        }
        self.encoder
            .movsd_xmm_m64_base_disp32(start, Gpr::R11, call_result_disp());
        self.encoder.add_rsp_imm32(CALL_FRAME_BYTES);
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

        self.encoder.sub_rsp_imm32(CALL_FRAME_BYTES);
        self.encoder.mov_r64_r64(Gpr::R11, Gpr::Rsp);
        for (index, register) in XMM_STACK.iter().copied().take(self.depth).enumerate() {
            self.encoder
                .movsd_m64_base_disp32_xmm(Gpr::R11, call_spill_disp(index), register);
        }

        self.encoder
            .mov_r64_r64(operand_filter_operands_arg_reg(), Gpr::R11);
        let operands_disp = call_spill_disp(input_slot);
        if operands_disp != 0 {
            self.encoder
                .add_r64_imm32(operand_filter_operands_arg_reg(), operands_disp);
        }
        self.encoder
            .mov_r64_r64(operand_filter_ctx_arg_reg(), self.ctx_arg_reg());
        self.encoder
            .movabs_r64_imm64(operand_filter_id_arg_reg(), filter_id as u64);
        self.encoder
            .movabs_r64_imm64(Gpr::Rax, helper as usize as u64);
        self.encoder.call_r64(Gpr::Rax);

        self.encoder.mov_r64_r64(Gpr::R11, Gpr::Rsp);
        self.encoder
            .movsd_m64_base_disp32_xmm(Gpr::R11, call_result_disp(), Xmm::Xmm0);
        for (index, register) in XMM_STACK.iter().copied().take(input_slot).enumerate() {
            self.encoder
                .movsd_xmm_m64_base_disp32(register, Gpr::R11, call_spill_disp(index));
        }
        self.encoder
            .movsd_xmm_m64_base_disp32(input, Gpr::R11, call_result_disp());
        self.encoder.add_rsp_imm32(CALL_FRAME_BYTES);
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

    fn emit_logical_not(&mut self) -> JitResult<()> {
        if self.depth == 0 {
            return Err(JitError::Encoding {
                model: MODEL.into(),
                detail: "logical not requires stack depth 1, found 0".into(),
            });
        }

        let target = XMM_STACK[self.depth - 1];
        self.emit_falsy_to_gpr(target, Gpr::R10, Gpr::R11);
        self.encoder.movzx_r32_r8(Gpr::R10, Gpr::R10);
        self.encoder.cvtsi2sd_xmm_r32(target, Gpr::R10);
        Ok(())
    }

    fn emit_truthy_to_gpr(&mut self, value: Xmm, dst: Gpr) {
        self.emit_abs_register(value);
        self.emit_literal_compare(value, BOOLEAN_EPSILON);
        self.encoder.setcc_r8(ConditionCode::Above, dst);
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
        self.encoder.movq_r64_xmm(Gpr::Rax, else_value);
        self.encoder.movq_r64_xmm(Gpr::R11, then_value);
        self.encoder.test_r8_r8(Gpr::R10, Gpr::R10);
        self.encoder.cmovne_r64_r64(Gpr::Rax, Gpr::R11);
        self.encoder.movq_xmm_r64(cond, Gpr::Rax);
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
        self.encoder.movq_r64_xmm(Gpr::Rax, left);
        self.encoder.ucomisd_xmm_xmm(left, left);
        self.encoder.setcc_r8(ConditionCode::NotParity, Gpr::R10);
        self.emit_abs_zero_to_gpr(left, Gpr::R8);
        match op {
            ExtremumOp::Min => self.encoder.minsd_xmm_xmm(left, right),
            ExtremumOp::Max => self.encoder.maxsd_xmm_xmm(left, right),
        }
        self.emit_extremum_select_left_fixup(left, right);
        self.depth -= 1;
        Ok(())
    }

    fn emit_extremum_select_left_fixup(&mut self, result: Xmm, right: Xmm) {
        self.encoder.ucomisd_xmm_xmm(right, right);
        self.encoder.setcc_r8(ConditionCode::Parity, Gpr::R11);
        self.encoder.and_r8_r8(Gpr::R10, Gpr::R11);
        self.emit_abs_zero_to_gpr(result, Gpr::R9);
        self.encoder.and_r8_r8(Gpr::R8, Gpr::R9);
        self.encoder.or_r8_r8(Gpr::R10, Gpr::R8);
        self.encoder.movq_r64_xmm(Gpr::R11, result);
        self.encoder.test_r8_r8(Gpr::R10, Gpr::R10);
        self.encoder.cmovne_r64_r64(Gpr::R11, Gpr::Rax);
        self.encoder.movq_xmm_r64(result, Gpr::R11);
    }

    fn emit_abs_zero_to_gpr(&mut self, value: Xmm, dst: Gpr) {
        self.encoder.movq_r64_xmm(dst, value);
        self.encoder.btr_r64_imm8(dst, 63);
        self.encoder.test_r64_r64(dst, dst);
        self.encoder.setcc_r8(ConditionCode::Equal, dst);
    }

    fn emit_voltage_load(&mut self, pos: VoltageNode, neg: VoltageNode) -> JitResult<()> {
        let dst = self.push_register()?;

        match (pos, neg) {
            (VoltageNode::Ground, VoltageNode::Ground) => {
                self.encoder.xorpd_xmm_xmm(dst, dst);
            }
            (pos, VoltageNode::Ground) => {
                self.emit_node_voltage_load(dst, pos)?;
            }
            (VoltageNode::Ground, neg) => {
                let scratch = self.scratch_register()?;
                self.encoder.xorpd_xmm_xmm(dst, dst);
                self.emit_node_voltage_load(scratch, neg)?;
                self.encoder.subsd_xmm_xmm(dst, scratch);
            }
            (pos, neg) => {
                let scratch = self.scratch_register()?;
                self.emit_node_voltage_load(dst, pos)?;
                self.emit_node_voltage_load(scratch, neg)?;
                self.encoder.subsd_xmm_xmm(dst, scratch);
            }
        }

        Ok(())
    }

    fn emit_thermal_voltage_load(&mut self) -> JitResult<()> {
        let dst = self.push_register()?;
        self.encoder
            .movsd_xmm_m64_base_disp32(dst, self.ctx_arg_reg(), TEMPERATURE_OFFSET);
        self.emit_literal_binary_op(dst, K_BOLTZMANN, BinaryOp::Mul);
        self.emit_literal_binary_op(dst, Q_ELECTRON, BinaryOp::Div);
        Ok(())
    }

    fn emit_node_voltage_load(&mut self, dst: Xmm, node: VoltageNode) -> JitResult<()> {
        match node {
            VoltageNode::Terminal(index) => self.emit_terminal_voltage_load(dst, index),
            VoltageNode::Internal(index) => self.emit_internal_voltage_load(dst, index),
            VoltageNode::Ground => {
                self.encoder.xorpd_xmm_xmm(dst, dst);
                Ok(())
            }
        }
    }

    fn emit_terminal_voltage_load(&mut self, dst: Xmm, index: usize) -> JitResult<()> {
        self.emit_context_pointer_load(VOLTAGES_OFFSET);
        self.encoder
            .movsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(index)?);
        Ok(())
    }

    fn emit_internal_voltage_load(&mut self, dst: Xmm, index: usize) -> JitResult<()> {
        self.emit_context_pointer_load(INTERNAL_VOLTAGES_OFFSET);
        self.encoder
            .movsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(index)?);
        Ok(())
    }

    fn emit_context_pointer_load(&mut self, ctx_field_offset: i32) {
        self.encoder
            .mov_r64_m64_base_disp32(Gpr::Rax, self.ctx_arg_reg(), ctx_field_offset);
    }

    fn emit_context_f64_load(&mut self, ctx_field_offset: i32) -> JitResult<()> {
        let dst = self.push_register()?;
        self.encoder
            .movsd_xmm_m64_base_disp32(dst, self.ctx_arg_reg(), ctx_field_offset);
        Ok(())
    }

    fn emit_context_u8_flag_load(
        &mut self,
        ctx_pointer_field_offset: i32,
        index: usize,
    ) -> JitResult<()> {
        let dst = self.push_register()?;
        self.emit_context_pointer_load(ctx_pointer_field_offset);
        self.encoder
            .movzx_r32_m8_base_disp32(Gpr::R10, Gpr::Rax, byte_disp_u8(index)?);
        self.encoder.cvtsi2sd_xmm_r32(dst, Gpr::R10);
        Ok(())
    }

    fn emit_analysis_check(&mut self, analysis_id: u8) -> JitResult<()> {
        let dst = self.push_register()?;
        if analysis_id > 5 {
            self.encoder.xorpd_xmm_xmm(dst, dst);
            return Ok(());
        }

        self.encoder
            .movzx_r32_m8_base_disp32(Gpr::R10, self.ctx_arg_reg(), ANALYSIS_TYPE_OFFSET);
        if analysis_id == 5 {
            self.encoder.cmp_r32_imm8(Gpr::R10, 0);
            self.encoder.setcc_r8(ConditionCode::Equal, Gpr::R11);
            self.encoder.cmp_r32_imm8(Gpr::R10, 4);
            self.encoder.setcc_r8(ConditionCode::Equal, Gpr::R10);
            self.encoder.or_r8_r8(Gpr::R10, Gpr::R11);
        } else {
            self.encoder.cmp_r32_imm8(Gpr::R10, analysis_id);
            self.encoder.setcc_r8(ConditionCode::Equal, Gpr::R10);
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

    fn emit_literal_binary_op(&mut self, dst: Xmm, value: f64, op: BinaryOp) {
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

    fn finish_with_literals(self) -> JitResult<Vec<u8>> {
        let mut bytes = self.encoder.into_bytes();
        for literal in &self.literals {
            let literal_offset = bytes.len();
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
            bytes.extend_from_slice(&literal.value.to_le_bytes());
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
        NativeAssignment::Indexed { .. } => true,
        NativeAssignment::Loop { .. } => true,
    }
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
        UnaryMathOp::Limexp => rspice_limexp,
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
        BinaryMathOp::Mod => rspice_mod,
    }
}

fn integer_binary_helper(op: IntegerBinaryOp) -> BinaryHelper {
    match op {
        IntegerBinaryOp::Shl => rspice_shl,
        IntegerBinaryOp::Shr => rspice_shr,
        IntegerBinaryOp::BitAnd => rspice_bitand,
        IntegerBinaryOp::BitOr => rspice_bitor,
        IntegerBinaryOp::BitXor => rspice_bitxor,
    }
}

fn call_spill_disp(index: usize) -> i32 {
    CALL_SHADOW_BYTES + (index * WORD_BYTES) as i32
}

fn call_result_disp() -> i32 {
    call_spill_disp(CALL_RESULT_SLOT)
}

fn program_uses_helper_calls(program: &NativeProgram) -> bool {
    program.ops().iter().any(|op| {
        matches!(
            op,
            NativeOp::UnaryMath(_)
                | NativeOp::BinaryMath(_)
                | NativeOp::IntegerBinary(_)
                | NativeOp::TableLookup(_)
                | NativeOp::TableDerivative(_)
                | NativeOp::LoadVariableDyn { .. }
                | NativeOp::LaplaceState(_)
                | NativeOp::ZiState(_)
                | NativeOp::TimerState(_)
                | NativeOp::TransitionState(_)
                | NativeOp::SlewState(_)
                | NativeOp::AbsDelayState(_)
                | NativeOp::CrossState(_)
                | NativeOp::IdtModState(_)
        )
    })
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
        K_BOLTZMANN, NativeAssignment, Q_ELECTRON, compile_assignment_function,
        compile_assignment_pass_function, compile_value_function,
    };
    use crate::codegen::{BytecodeProgram, Instruction, LookupTable};
    use crate::laplace::StateSpaceFilter;
    use crate::native::expr::{EntryKind, NativeLoweringLimits, NativeProgram};
    use crate::native::runtime::ExecutableMemory;
    use crate::native::{EvalContext, clear_native_runtime_error, take_native_runtime_error};
    use crate::vm::{CrossDetector, DelayBuffer, SlewFilter, TransitionFilter};
    use crate::zfilter::ZiFilter;

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
    fn generated_value_leaf_without_helper_call_omits_saved_arg_prologue() {
        let program = native_program(EntryKind::StampValue, vec![Instruction::PushConst(1.0)], 0);

        let bytes = compile_value_function(&program).expect("compile literal value function");

        assert!(
            !bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "helper-free native leaves should not pay callee-saved prologue cost"
        );
    }

    #[test]
    fn generated_value_leaf_with_helper_call_emits_saved_arg_prologue() {
        let program = native_program(
            EntryKind::StampValue,
            vec![Instruction::PushConst(1.0), Instruction::Exp],
            0,
        );

        let bytes = compile_value_function(&program).expect("compile helper-call value function");

        assert!(
            bytes.starts_with(&[0x41, 0x54, 0x41, 0x55]),
            "helper-call native leaves must preserve context and vars pointers"
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
    fn generated_value_leaf_loads_dynamic_variable_and_preserves_stack() {
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
        let bytes = compile_value_function(&program).expect("compile dynamic variable leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate dynamic variable leaf");
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
        let available_current_pairs = [2];
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
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate current probe leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let branch_currents = [f64::NAN, 4.0_f64, -4.0_f64, f64::NAN];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.branch_currents = branch_currents.as_ptr();
        ctx.branch_currents_len = branch_currents.len();
        ctx.num_terminals = 2;

        assert_eq!(f(&ctx, std::ptr::null()), -1.0);
    }

    #[test]
    fn generated_value_leaf_computes_thermal_voltage_from_context_temperature() {
        let program = native_program(EntryKind::StampValue, vec![Instruction::PushVt], 0);
        let bytes = compile_value_function(&program).expect("compile thermal voltage leaf");
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
        ctx.state_values = state_values.as_mut_ptr();

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
        ctx.state_values = state_values.as_mut_ptr();

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
        ctx.state_values = state_values.as_mut_ptr();

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
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate limit state leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };

        let mut state_values = [0.0_f64, 0.0_f64];
        let mut state_initialized = [0_u8, 0_u8];
        let mut ctx = eval_context(&[], &[], &[], &[]);
        ctx.state_values = state_values.as_mut_ptr();
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
        assert_eq!(
            f(&ctx, vars.as_ptr()).to_bits(),
            20.0_f64.to_bits(),
            "native limit must not index past state_initialized_len"
        );
        assert_eq!(
            state_values[1].to_bits(),
            10.0_f64.to_bits(),
            "out-of-range state flag metadata must leave native state untouched"
        );

        ctx.state_initialized_len = state_initialized.len();
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
    fn generated_value_leaf_computes_sqrt_in_place() {
        let program = native_program(
            EntryKind::StampValue,
            vec![Instruction::PushConst(49.0), Instruction::Sqrt],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile sqrt leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate sqrt leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[], &[], &[]);

        assert_eq!(f(&ctx, std::ptr::null()), 7.0);
    }

    #[test]
    fn generated_value_leaf_computes_abs_in_place() {
        let program = native_program(
            EntryKind::StampValue,
            vec![Instruction::PushConst(-7.5), Instruction::Abs],
            0,
        );
        let bytes = compile_value_function(&program).expect("compile abs leaf");
        let memory = ExecutableMemory::allocate(&bytes).expect("allocate abs leaf");
        let entry = memory.ptr_at(0).expect("entry point inside image");
        let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
            unsafe { std::mem::transmute(entry) };
        let ctx = eval_context(&[], &[], &[], &[]);

        assert_eq!(f(&ctx, std::ptr::null()), 7.5);
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
    fn generated_value_leaf_calls_integer_binary_helpers_and_preserves_state() {
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

        for (name, op, left, right, integer_expected) in cases {
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
            let bytes = compile_value_function(&program).expect("compile integer helper leaf");
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate integer helper leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let mut ctx = eval_context(&[], &[], &[], &[]);
            ctx.temperature = 310.0;
            ctx.time = 2.0;
            let vars = [7.0_f64];

            assert_eq!(
                f(&ctx, vars.as_ptr()).to_bits(),
                (310.0 + ((7.0 + integer_expected) + 2.0)).to_bits(),
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
                    Instruction::PushConst(left),
                    Instruction::PushConst(right),
                    op.clone(),
                ],
                0,
            );
            let bytes = compile_value_function(&program).expect("compile min/max leaf");
            let memory = ExecutableMemory::allocate(&bytes).expect("allocate min/max leaf");
            let entry = memory.ptr_at(0).expect("entry point inside image");
            let f: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                unsafe { std::mem::transmute(entry) };
            let ctx = eval_context(&[], &[], &[], &[]);
            let expected = match op {
                Instruction::Min => runtime_min(left, right),
                Instruction::Max => runtime_max(left, right),
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
    fn rejects_neg_when_no_scratch_xmm_register_is_available() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushConst(1.0),
                Instruction::PushConst(2.0),
                Instruction::PushConst(3.0),
                Instruction::PushConst(4.0),
                Instruction::PushConst(5.0),
                Instruction::PushConst(6.0),
                Instruction::Neg,
                Instruction::Add,
                Instruction::Add,
                Instruction::Add,
                Instruction::Add,
                Instruction::Add,
            ],
            0,
        );

        let error = compile_value_function(&program)
            .expect_err("neg at full stack depth must require scratch register");

        assert!(matches!(
            error,
            crate::native::JitError::RegisterAllocation { .. }
        ));
        assert!(error.to_string().contains("scratch"));
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
        NativeProgram::from_bytecode(
            "x64-codegen-test",
            entry_kind,
            &BytecodeProgram { instructions },
            NativeLoweringLimits::new(terminal_count, internal_node_count, 8, 8, 8)
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
        NativeProgram::from_bytecode(
            "x64-codegen-test",
            entry_kind,
            &BytecodeProgram { instructions },
            NativeLoweringLimits::new(terminal_count, 0, 8, 8, 8)
                .with_lookup_table_count(8)
                .with_available_current_pairs(available_current_pairs),
        )
        .expect("lower bytecode to native program")
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
        }
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
        K_BOLTZMANN * temperature / Q_ELECTRON
    }
}
