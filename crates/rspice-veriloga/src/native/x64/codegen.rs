use super::encoder::{Gpr, X64Encoder, Xmm};
use crate::native::expr::{NativeOp, NativeProgram, VoltageNode};
use crate::native::{JitError, JitResult};

const MODEL: &str = "native-x64";
const VOLTAGES_OFFSET: i32 = 0;
const INTERNAL_VOLTAGES_OFFSET: i32 = 8;
const PARAMS_OFFSET: i32 = 16;
const BRANCH_UNKNOWNS_OFFSET: i32 = 160;
const WORD_BYTES: usize = std::mem::size_of::<f64>();
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
    let mut compiler = FunctionCompiler::new(program)?;
    compiler.emit_program()?;
    compiler.finish_value_function()
}

#[allow(dead_code)]
pub(crate) fn compile_assignment_function(
    var_index: usize,
    program: &NativeProgram,
) -> JitResult<Vec<u8>> {
    let mut compiler = FunctionCompiler::new(program)?;
    compiler.emit_program()?;
    compiler.finish_assignment_function(var_index)
}

#[derive(Debug)]
struct FunctionCompiler<'a> {
    program: &'a NativeProgram,
    encoder: X64Encoder,
    depth: usize,
    literals: Vec<LiteralPatch>,
}

#[derive(Debug)]
struct LiteralPatch {
    displacement_offset: usize,
    value: f64,
}

impl<'a> FunctionCompiler<'a> {
    fn new(program: &'a NativeProgram) -> JitResult<Self> {
        if program.max_stack_depth() > XMM_STACK.len() {
            return Err(register_allocation_error(format!(
                "expression stack depth {} exceeds {} XMM registers",
                program.max_stack_depth(),
                XMM_STACK.len()
            )));
        }

        Ok(Self {
            program,
            encoder: X64Encoder::new(),
            depth: 0,
            literals: Vec::new(),
        })
    }

    fn emit_program(&mut self) -> JitResult<()> {
        for op in self.program.ops() {
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
                NativeOp::LoadVoltage { pos, neg } => {
                    self.emit_voltage_load(pos, neg)?;
                }
                NativeOp::LoadInternalVoltage(index) => {
                    let dst = self.push_register()?;
                    self.emit_context_pointer_load(INTERNAL_VOLTAGES_OFFSET);
                    self.encoder
                        .movsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(index)?);
                }
                NativeOp::LoadVariable(index) => {
                    let dst = self.push_register()?;
                    self.encoder.movsd_xmm_m64_base_disp32(
                        dst,
                        host_vars_arg_reg(),
                        byte_disp(index)?,
                    );
                }
                NativeOp::LoadBranchUnknown(index) => {
                    let dst = self.push_register()?;
                    self.emit_context_pointer_load(BRANCH_UNKNOWNS_OFFSET);
                    self.encoder
                        .movsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(index)?);
                }
                NativeOp::Add => self.emit_binary_op(BinaryOp::Add)?,
                NativeOp::Sub => self.emit_binary_op(BinaryOp::Sub)?,
                NativeOp::Mul => self.emit_binary_op(BinaryOp::Mul)?,
                NativeOp::Div => self.emit_binary_op(BinaryOp::Div)?,
                NativeOp::Neg => self.emit_neg()?,
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
        self.encoder.ret();
        self.finish_with_literals()
    }

    fn finish_assignment_function(mut self, var_index: usize) -> JitResult<Vec<u8>> {
        self.encoder.movsd_m64_base_disp32_xmm(
            host_vars_arg_reg(),
            byte_disp(var_index)?,
            Xmm::Xmm0,
        );
        self.encoder.ret();
        self.finish_with_literals()
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

    fn emit_voltage_load(&mut self, pos: VoltageNode, neg: VoltageNode) -> JitResult<()> {
        let dst = self.push_register()?;

        match (pos, neg) {
            (VoltageNode::Ground, VoltageNode::Ground) => {
                self.encoder.xorpd_xmm_xmm(dst, dst);
            }
            (VoltageNode::Terminal(pos), VoltageNode::Ground) => {
                self.emit_terminal_voltage_load(dst, pos)?;
            }
            (VoltageNode::Ground, VoltageNode::Terminal(neg)) => {
                let scratch = self.scratch_register()?;
                self.emit_terminal_voltage_load(dst, neg)?;
                self.encoder.xorpd_xmm_xmm(scratch, scratch);
                self.encoder.subsd_xmm_xmm(scratch, dst);
                self.encoder.movsd_xmm_xmm(dst, scratch);
            }
            (VoltageNode::Terminal(pos), VoltageNode::Terminal(neg)) => {
                let scratch = self.scratch_register()?;
                self.emit_terminal_voltage_load(dst, pos)?;
                self.emit_terminal_voltage_load(scratch, neg)?;
                self.encoder.subsd_xmm_xmm(dst, scratch);
            }
        }

        Ok(())
    }

    fn emit_terminal_voltage_load(&mut self, dst: Xmm, index: usize) -> JitResult<()> {
        self.emit_context_pointer_load(VOLTAGES_OFFSET);
        self.encoder
            .movsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(index)?);
        Ok(())
    }

    fn emit_context_pointer_load(&mut self, ctx_field_offset: i32) {
        self.encoder
            .mov_r64_m64_base_disp32(Gpr::Rax, host_ctx_arg_reg(), ctx_field_offset);
    }

    fn emit_literal_load(&mut self, dst: Xmm, value: f64) {
        let displacement_offset = self.encoder.movsd_xmm_m64_rip_disp32(dst, 0);
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

fn register_allocation_error(detail: String) -> JitError {
    JitError::RegisterAllocation {
        model: MODEL.into(),
        detail: detail.into(),
    }
}

#[cfg(windows)]
fn host_ctx_arg_reg() -> Gpr {
    Gpr::Rcx
}

#[cfg(windows)]
fn host_vars_arg_reg() -> Gpr {
    Gpr::Rdx
}

#[cfg(not(windows))]
fn host_ctx_arg_reg() -> Gpr {
    Gpr::Rdi
}

#[cfg(not(windows))]
fn host_vars_arg_reg() -> Gpr {
    Gpr::Rsi
}

#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
mod tests {
    use super::{compile_assignment_function, compile_value_function};
    use crate::codegen::{BytecodeProgram, Instruction};
    use crate::native::EvalContext;
    use crate::native::expr::{EntryKind, NativeProgram};
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
    fn generated_value_leaf_handles_internal_branch_sub_and_neg() {
        let program = native_program(
            EntryKind::StampValue,
            vec![
                Instruction::PushInternalVoltage(1),
                Instruction::PushBranchCurrent(0),
                Instruction::Sub,
                Instruction::Neg,
            ],
            0,
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
        let program = native_program(
            EntryKind::StampValue,
            vec![Instruction::PushVariable(too_large_index)],
            0,
        );

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
        NativeProgram::from_bytecode(
            "x64-codegen-test",
            entry_kind,
            &BytecodeProgram { instructions },
            terminal_count,
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
}
