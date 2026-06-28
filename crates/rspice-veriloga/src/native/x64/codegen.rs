use super::encoder::{ConditionCode, Gpr, X64Encoder, Xmm};
use crate::native::abi::{
    rspice_acos, rspice_asin, rspice_atan, rspice_cos, rspice_cosh, rspice_exp, rspice_limexp,
    rspice_log, rspice_log10, rspice_sin, rspice_sinh, rspice_tan, rspice_tanh,
};
use crate::native::expr::UnaryMathOp;
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
const PARAM_GIVEN_OFFSET: i32 = 152;
const BRANCH_UNKNOWNS_OFFSET: i32 = 160;
const MFACTOR_OFFSET: i32 = 176;
const WORD_BYTES: usize = std::mem::size_of::<f64>();
const K_BOLTZMANN: f64 = 1.380649e-23;
const Q_ELECTRON: f64 = 1.602176634e-19;
const BOOLEAN_EPSILON: f64 = 1.0e-15;
const CALL_SPILL_SLOT_COUNT: usize = 7;
const CALL_RESULT_SLOT: usize = 6;
const CALL_FRAME_BYTES: i32 = CALL_SHADOW_BYTES + (CALL_SPILL_SLOT_COUNT * WORD_BYTES) as i32;
#[cfg(windows)]
const CALL_SHADOW_BYTES: i32 = 32;
#[cfg(not(windows))]
const CALL_SHADOW_BYTES: i32 = 0;
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
    let mut compiler = FunctionCompiler::new(program_uses_helper_calls(program));
    compiler.emit_program(program)?;
    compiler.finish_value_function()
}

#[allow(dead_code)]
pub(crate) fn compile_assignment_function(
    var_index: usize,
    program: &NativeProgram,
) -> JitResult<Vec<u8>> {
    let mut compiler = FunctionCompiler::new(program_uses_helper_calls(program));
    compiler.emit_program(program)?;
    compiler.finish_assignment_function(var_index)
}

pub(crate) fn compile_assignment_pass_function(
    assignments: &[(usize, NativeProgram)],
) -> JitResult<Vec<u8>> {
    let mut compiler = FunctionCompiler::new(
        assignments
            .iter()
            .any(|(_, program)| program_uses_helper_calls(program)),
    );
    for (var_index, program) in assignments {
        compiler.emit_program(program)?;
        compiler.emit_assignment_store(*var_index)?;
    }
    compiler.finish_assignment_pass_function()
}

#[derive(Debug)]
struct FunctionCompiler {
    encoder: X64Encoder,
    depth: usize,
    literals: Vec<LiteralPatch>,
    uses_helper_calls: bool,
}

#[derive(Debug)]
struct LiteralPatch {
    displacement_offset: usize,
    value: f64,
}

impl FunctionCompiler {
    fn new(uses_helper_calls: bool) -> Self {
        let mut compiler = Self {
            encoder: X64Encoder::new(),
            depth: 0,
            literals: Vec::new(),
            uses_helper_calls,
        };
        if compiler.uses_helper_calls {
            compiler.emit_prologue();
        }
        compiler
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
                NativeOp::LoadMfactor => {
                    self.emit_context_f64_load(MFACTOR_OFFSET)?;
                }
                NativeOp::Add => self.emit_binary_op(BinaryOp::Add)?,
                NativeOp::Sub => self.emit_binary_op(BinaryOp::Sub)?,
                NativeOp::Mul => self.emit_binary_op(BinaryOp::Mul)?,
                NativeOp::Div => self.emit_binary_op(BinaryOp::Div)?,
                NativeOp::Neg => self.emit_neg()?,
                NativeOp::Abs => self.emit_abs()?,
                NativeOp::Sqrt => self.emit_sqrt()?,
                NativeOp::Compare(op) => self.emit_compare(op)?,
                NativeOp::Logical(op) => self.emit_logical(op)?,
                NativeOp::IfElse => self.emit_ifelse()?,
                NativeOp::Extremum(op) => self.emit_extremum(op)?,
                NativeOp::UnaryMath(op) => self.emit_unary_math(op)?,
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
        self.emit_return();
        self.finish_with_literals()
    }

    fn finish_assignment_function(mut self, var_index: usize) -> JitResult<Vec<u8>> {
        self.emit_assignment_store(var_index)?;
        self.emit_return();
        self.finish_with_literals()
    }

    fn finish_assignment_pass_function(mut self) -> JitResult<Vec<u8>> {
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
    }

    fn emit_return(&mut self) {
        if self.uses_helper_calls {
            self.encoder.pop_r64(Gpr::R13);
            self.encoder.pop_r64(Gpr::R12);
        }
        self.encoder.ret();
    }

    fn ctx_arg_reg(&self) -> Gpr {
        if self.uses_helper_calls {
            saved_ctx_arg_reg()
        } else {
            entry_ctx_arg_reg()
        }
    }

    fn vars_arg_reg(&self) -> Gpr {
        if self.uses_helper_calls {
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
}

#[derive(Debug, Clone, Copy)]
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

type UnaryHelper = extern "C" fn(f64) -> f64;

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
    }
}

fn call_spill_disp(index: usize) -> i32 {
    CALL_SHADOW_BYTES + (index * WORD_BYTES) as i32
}

fn call_result_disp() -> i32 {
    call_spill_disp(CALL_RESULT_SLOT)
}

fn program_uses_helper_calls(program: &NativeProgram) -> bool {
    program
        .ops()
        .iter()
        .any(|op| matches!(op, NativeOp::UnaryMath(_)))
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

#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
mod tests {
    use super::{K_BOLTZMANN, Q_ELECTRON, compile_assignment_function, compile_value_function};
    use crate::codegen::{BytecodeProgram, Instruction};
    use crate::native::EvalContext;
    use crate::native::expr::{EntryKind, NativeLoweringLimits, NativeProgram};
    use crate::native::runtime::ExecutableMemory;

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
            NativeLoweringLimits::new(terminal_count, internal_node_count, 8, 8, 8),
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
            lookup_tables: std::ptr::null(),
            lookup_tables_len: 0,
            laplace_filters: std::ptr::null_mut(),
            laplace_filters_len: 0,
            param_given: std::ptr::null(),
            branch_unknowns: branch_unknowns.as_ptr(),
            analysis_type: 0,
            multiplicity: 1.0,
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

    fn thermal_voltage(temperature: f64) -> f64 {
        K_BOLTZMANN * temperature / Q_ELECTRON
    }
}
