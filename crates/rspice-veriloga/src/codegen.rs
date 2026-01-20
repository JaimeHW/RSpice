//! Code Generator for Verilog-A
//!
//! Generates simulator-ready device models from IR.
//! Produces bytecode programs for efficient simulation.

use crate::CompilerOptions;
use crate::ast::BinaryOp;
use crate::error::{CodeGenError, CodeGenErrorKind, CompileError, CompileResult};
use crate::ir::{BranchEquation, DerivativeWrt, DeviceIR, IrExpr, IrFunction};
use crate::semantic::AnalyzedFile;
use smol_str::SmolStr;

/// Code generator
pub struct CodeGenerator<'a> {
    #[allow(dead_code)]
    options: &'a CompilerOptions,
}

/// Compiled device model ready for simulation
#[derive(Debug, Clone)]
pub struct CompiledModel {
    /// Model name
    pub name: SmolStr,
    /// Number of terminals
    pub num_terminals: usize,
    /// Terminal names
    pub terminal_names: Vec<SmolStr>,
    /// Parameter definitions
    pub parameters: Vec<CompiledParameter>,
    /// Number of variables
    pub num_variables: usize,
    /// Variable assignment programs (executed in order before contributions)
    pub assignment_programs: Vec<AssignmentProgram>,
    /// Compiled stamp programs for each contribution
    pub stamp_programs: Vec<StampProgram>,
    /// Number of internal nodes (if any)
    pub internal_nodes: usize,
    /// Number of branch currents to track
    pub branch_currents: usize,
}

/// Compiled parameter
#[derive(Debug, Clone)]
pub struct CompiledParameter {
    pub name: SmolStr,
    pub default: f64,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

/// Stamp program for a contribution
#[derive(Debug, Clone)]
pub struct StampProgram {
    /// Which row/col this stamps to
    pub stamp_locations: Vec<StampLocation>,
    /// The bytecode program to evaluate the value
    pub value_program: BytecodeProgram,
    /// Jacobian programs (one per derivative)
    pub jacobian_programs: Vec<JacobianEntry>,
}

/// Assignment program for a variable
#[derive(Debug, Clone)]
pub struct AssignmentProgram {
    /// Index of variable being assigned
    pub var_index: usize,
    /// The bytecode program to compute the value
    pub program: BytecodeProgram,
}

/// Location to stamp in matrix
#[derive(Debug, Clone)]
pub struct StampLocation {
    pub row: StampIndex,
    pub col: StampIndex,
    pub sign: f64,
}

/// Index for stamping (terminal or internal)
#[derive(Debug, Clone)]
pub enum StampIndex {
    Terminal(usize),
    Internal(usize),
    Ground,
}

/// Jacobian entry
#[derive(Debug, Clone)]
pub struct JacobianEntry {
    pub row: StampIndex,
    pub col: StampIndex,
    pub program: BytecodeProgram,
}

/// Bytecode program for expression evaluation
#[derive(Debug, Clone, Default)]
pub struct BytecodeProgram {
    pub instructions: Vec<Instruction>,
}

/// VM Instructions
#[derive(Debug, Clone)]
pub enum Instruction {
    /// Push constant
    PushConst(f64),
    /// Push parameter value
    PushParam(usize),
    /// Push voltage V(i, j)
    PushVoltage(usize, usize),
    /// Push current I(i, j)
    PushCurrent(usize, usize),
    /// Push internal node voltage (for internal nodes not in port list)
    PushInternalVoltage(usize),
    /// Push variable value
    PushVariable(usize),
    /// Push temperature
    PushTemperature,
    /// Push thermal voltage
    PushVt,
    /// Push time
    PushTime,
    /// Binary operations
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    /// Unary operations
    Neg,
    /// Functions
    Abs,
    Sqrt,
    Exp,
    Log,
    Log10,
    Sin,
    Cos,
    Tan,
    Sinh,
    Cosh,
    Tanh,
    Min,
    Max,
    /// Limited exponential (for convergence)
    Limexp,
    /// Inverse trigonometric functions
    Asin,
    Acos,
    Atan,
    Atan2, // 2-argument arctangent(y, x)
    /// Rounding functions
    Floor,
    Ceil,
    /// Power function (2-argument)
    FnPow,
    /// Comparison operations (return 1.0 for true, 0.0 for false)
    Gt, // Greater than
    Lt, // Less than
    Ge, // Greater than or equal
    Le, // Less than or equal
    Eq, // Equal
    Ne, // Not equal
    /// Logical operations
    And, // Logical and
    Or, // Logical or
    Not, // Logical not
    /// State-based time derivative: ddt(expr) using state index
    /// Uses backward Euler: (current - prev) / dt
    DdtState(usize),
    /// State-based integration: idt(expr, ic) using state index
    /// Uses forward Euler: prev + expr * dt
    IdtState(usize),
    /// Conditional: if top is nonzero, use second, else third
    IfElse,
}

impl<'a> CodeGenerator<'a> {
    /// Create a new code generator
    pub fn new(options: &'a CompilerOptions) -> Self {
        Self { options }
    }

    /// Generate compiled model from analyzed file
    pub fn generate(&self, analyzed: &AnalyzedFile) -> CompileResult<CompiledModel> {
        // Get the first module (for now, single module per file)
        let module = analyzed.modules.values().next().ok_or_else(|| {
            CodeGenError::new(CodeGenErrorKind::Internal("No modules found".into()))
        })?;

        // Build IR
        let ir = DeviceIR::from_analyzed(module);

        // Generate code from IR
        self.generate_from_ir(&ir)
    }

    /// Generate from IR
    fn generate_from_ir(&self, ir: &DeviceIR) -> CompileResult<CompiledModel> {
        let mut model = CompiledModel {
            name: ir.name.clone(),
            num_terminals: ir.terminals.len(),
            terminal_names: ir.terminals.iter().map(|t| t.name.clone()).collect(),
            parameters: ir
                .parameters
                .iter()
                .map(|p| CompiledParameter {
                    name: p.name.clone(),
                    default: p.default,
                    min: p.min,
                    max: p.max,
                })
                .collect(),
            num_variables: ir.variables.len(),
            assignment_programs: Vec::new(),
            stamp_programs: Vec::new(),
            internal_nodes: ir.internal_nodes.len(),
            branch_currents: 0,
        };

        // Generate assignment programs (executed in order before contributions)
        for assign in &ir.assignments {
            let program = self.compile_expr(&assign.expr, ir)?;
            model.assignment_programs.push(AssignmentProgram {
                var_index: assign.var_index,
                program,
            });
        }

        // Generate stamp programs for each equation
        for eq in &ir.equations {
            let program = self.compile_equation(eq, ir)?;
            model.stamp_programs.push(program);
        }

        Ok(model)
    }

    /// Compile a branch equation to a stamp program
    fn compile_equation(&self, eq: &BranchEquation, ir: &DeviceIR) -> CompileResult<StampProgram> {
        let value_program = self.compile_expr(&eq.expr, ir)?;

        let mut jacobian_programs = Vec::new();
        for deriv in &eq.derivatives {
            let program = self.compile_expr(&deriv.expr, ir)?;
            let (row, col) = self.derivative_indices(&eq.branch, &deriv.wrt, eq.is_current);
            jacobian_programs.push(JacobianEntry { row, col, program });
        }

        // Build stamp locations for the contribution
        let pos = eq.branch.pos_terminal;
        let neg = eq.branch.neg_terminal;

        let stamp_locations = if eq.is_current {
            // Current contribution: stamps to RHS at pos and neg
            vec![
                StampLocation {
                    row: StampIndex::Terminal(pos),
                    col: StampIndex::Ground,
                    sign: -1.0,
                },
                StampLocation {
                    row: StampIndex::Terminal(neg),
                    col: StampIndex::Ground,
                    sign: 1.0,
                },
            ]
        } else {
            // Voltage contribution would need branch equation
            vec![]
        };

        Ok(StampProgram {
            stamp_locations,
            value_program,
            jacobian_programs,
        })
    }

    /// Get row/col for a derivative
    fn derivative_indices(
        &self,
        branch: &crate::ir::BranchRef,
        wrt: &DerivativeWrt,
        is_current: bool,
    ) -> (StampIndex, StampIndex) {
        match wrt {
            DerivativeWrt::Voltage(node) => {
                if is_current {
                    (
                        StampIndex::Terminal(branch.pos_terminal),
                        StampIndex::Terminal(*node),
                    )
                } else {
                    (StampIndex::Internal(0), StampIndex::Terminal(*node))
                }
            }
            DerivativeWrt::Current(p, _n) => (StampIndex::Terminal(*p), StampIndex::Internal(0)),
            DerivativeWrt::Time => (StampIndex::Ground, StampIndex::Ground),
        }
    }

    /// Compile an IR expression to bytecode
    fn compile_expr(&self, expr: &IrExpr, ir: &DeviceIR) -> CompileResult<BytecodeProgram> {
        let mut program = BytecodeProgram::default();
        self.emit_expr(expr, ir, &mut program)?;
        Ok(program)
    }

    /// Emit bytecode for an expression
    fn emit_expr(
        &self,
        expr: &IrExpr,
        ir: &DeviceIR,
        program: &mut BytecodeProgram,
    ) -> CompileResult<()> {
        match expr {
            IrExpr::Const(v) => {
                program.instructions.push(Instruction::PushConst(*v));
            }
            IrExpr::Param(name) => {
                let idx = ir
                    .parameters
                    .iter()
                    .position(|p| &p.name == name)
                    .ok_or_else(|| {
                        CodeGenError::new(CodeGenErrorKind::Internal(format!(
                            "Unknown parameter: {}",
                            name
                        )))
                    })?;
                program.instructions.push(Instruction::PushParam(idx));
            }
            IrExpr::Var(name) => {
                let idx = ir
                    .variables
                    .iter()
                    .position(|v| &v.name == name)
                    .ok_or_else(|| {
                        CodeGenError::new(CodeGenErrorKind::Internal(format!(
                            "Unknown variable: {}",
                            name
                        )))
                    })?;
                program.instructions.push(Instruction::PushVariable(idx));
            }
            IrExpr::Voltage(p, n) => {
                program.instructions.push(Instruction::PushVoltage(*p, *n));
            }
            IrExpr::Current(p, n) => {
                program.instructions.push(Instruction::PushCurrent(*p, *n));
            }
            IrExpr::Temperature => {
                program.instructions.push(Instruction::PushTemperature);
            }
            IrExpr::Vt => {
                program.instructions.push(Instruction::PushVt);
            }
            IrExpr::Time => {
                program.instructions.push(Instruction::PushTime);
            }
            IrExpr::Binary(op, left, right) => {
                self.emit_expr(left, ir, program)?;
                self.emit_expr(right, ir, program)?;
                program.instructions.push(match op {
                    // Arithmetic
                    BinaryOp::Add => Instruction::Add,
                    BinaryOp::Sub => Instruction::Sub,
                    BinaryOp::Mul => Instruction::Mul,
                    BinaryOp::Div => Instruction::Div,
                    BinaryOp::Pow => Instruction::Pow,
                    // Comparisons
                    BinaryOp::Gt => Instruction::Gt,
                    BinaryOp::Lt => Instruction::Lt,
                    BinaryOp::Ge => Instruction::Ge,
                    BinaryOp::Le => Instruction::Le,
                    BinaryOp::Eq => Instruction::Eq,
                    BinaryOp::Ne => Instruction::Ne,
                    // Logical
                    BinaryOp::And => Instruction::And,
                    BinaryOp::Or => Instruction::Or,
                    _ => {
                        return Err(CompileError::CodeGen(CodeGenError::new(
                            CodeGenErrorKind::UnsupportedFeature(format!("Binary op {:?}", op)),
                        )));
                    }
                });
            }
            IrExpr::Unary(crate::ast::UnaryOp::Neg, inner) => {
                self.emit_expr(inner, ir, program)?;
                program.instructions.push(Instruction::Neg);
            }
            IrExpr::Call(func, args) => {
                for arg in args {
                    self.emit_expr(arg, ir, program)?;
                }
                program.instructions.push(match func {
                    IrFunction::Abs => Instruction::Abs,
                    IrFunction::Sqrt => Instruction::Sqrt,
                    IrFunction::Exp => Instruction::Exp,
                    IrFunction::Log => Instruction::Log,
                    IrFunction::Log10 => Instruction::Log10,
                    IrFunction::Sin => Instruction::Sin,
                    IrFunction::Cos => Instruction::Cos,
                    IrFunction::Tan => Instruction::Tan,
                    IrFunction::Sinh => Instruction::Sinh,
                    IrFunction::Cosh => Instruction::Cosh,
                    IrFunction::Tanh => Instruction::Tanh,
                    IrFunction::Min => Instruction::Min,
                    IrFunction::Max => Instruction::Max,
                    // Inverse trig
                    IrFunction::Asin => Instruction::Asin,
                    IrFunction::Acos => Instruction::Acos,
                    IrFunction::Atan => Instruction::Atan,
                    IrFunction::Atan2 => Instruction::Atan2,
                    // Rounding
                    IrFunction::Floor => Instruction::Floor,
                    IrFunction::Ceil => Instruction::Ceil,
                    // Power
                    IrFunction::Pow => Instruction::FnPow,
                });
            }
            IrExpr::Limexp(inner) => {
                self.emit_expr(inner, ir, program)?;
                program.instructions.push(Instruction::Limexp);
            }
            IrExpr::Conditional(cond, then_expr, else_expr) => {
                self.emit_expr(cond, ir, program)?;
                self.emit_expr(then_expr, ir, program)?;
                self.emit_expr(else_expr, ir, program)?;
                program.instructions.push(Instruction::IfElse);
            }
            IrExpr::Unary(crate::ast::UnaryOp::Not, inner) => {
                self.emit_expr(inner, ir, program)?;
                program.instructions.push(Instruction::Not);
            }
            IrExpr::Ddt(inner) => {
                // For DC analysis, ddt = 0. For transient, would need state tracking.
                // For now, emit 0 for DC compatibility
                let _ = inner; // Mark as intentionally unused for now
                program.instructions.push(Instruction::PushConst(0.0));
            }
            IrExpr::Idt(inner, ic) => {
                // For DC analysis, idt behavior depends on context
                // For now, use initial condition if provided, else 0
                if let Some(ic_expr) = ic {
                    self.emit_expr(ic_expr, ir, program)?;
                } else {
                    let _ = inner; // Mark as intentionally unused for now
                    program.instructions.push(Instruction::PushConst(0.0));
                }
            }
            IrExpr::Unary(op, _) => {
                return Err(CompileError::CodeGen(CodeGenError::new(
                    CodeGenErrorKind::UnsupportedFeature(format!("Unary op {:?}", op)),
                )));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::IrExpr;

    // ========================================================================
    // Bytecode Program Tests
    // ========================================================================

    #[test]
    fn test_bytecode_program_creation() {
        let mut program = BytecodeProgram::default();
        program.instructions.push(Instruction::PushConst(1.0));
        program.instructions.push(Instruction::PushConst(2.0));
        program.instructions.push(Instruction::Add);
        assert_eq!(program.instructions.len(), 3);
    }

    #[test]
    fn test_instruction_variants() {
        // Test all instruction variants can be created
        let instructions = vec![
            Instruction::PushConst(3.14),
            Instruction::PushParam(0),
            Instruction::PushVoltage(0, 1),
            Instruction::PushCurrent(0, 1),
            Instruction::PushTime,
            Instruction::PushTemperature,
            Instruction::PushVt,
            Instruction::Add,
            Instruction::Sub,
            Instruction::Mul,
            Instruction::Div,
            Instruction::Neg,
            Instruction::Pow,
            Instruction::Abs,
            Instruction::Sqrt,
            Instruction::Exp,
            Instruction::Log,
            Instruction::Log10,
            Instruction::Sin,
            Instruction::Cos,
            Instruction::Tan,
            Instruction::Sinh,
            Instruction::Cosh,
            Instruction::Tanh,
            Instruction::Min,
            Instruction::Max,
            Instruction::Limexp,
            Instruction::IfElse,
        ];
        assert_eq!(instructions.len(), 28);
    }

    // ========================================================================
    // Code Generation Tests
    // ========================================================================

    #[test]
    fn test_compile_const_expr() {
        let options = CompilerOptions::default();
        let codegen = CodeGenerator::new(&options);

        let ir = create_test_ir();
        let expr = IrExpr::Const(42.0);
        let program = codegen.compile_expr(&expr, &ir).unwrap();

        assert_eq!(program.instructions.len(), 1);
        assert!(
            matches!(program.instructions[0], Instruction::PushConst(v) if (v - 42.0).abs() < 1e-10)
        );
    }

    #[test]
    fn test_compile_voltage_expr() {
        let options = CompilerOptions::default();
        let codegen = CodeGenerator::new(&options);

        let ir = create_test_ir();
        let expr = IrExpr::Voltage(0, 1);
        let program = codegen.compile_expr(&expr, &ir).unwrap();

        assert_eq!(program.instructions.len(), 1);
        assert!(matches!(
            program.instructions[0],
            Instruction::PushVoltage(0, 1)
        ));
    }

    #[test]
    fn test_compile_param_expr() {
        let options = CompilerOptions::default();
        let codegen = CodeGenerator::new(&options);

        let ir = create_test_ir();
        let expr = IrExpr::Param("g".into());
        let program = codegen.compile_expr(&expr, &ir).unwrap();

        assert_eq!(program.instructions.len(), 1);
        assert!(matches!(program.instructions[0], Instruction::PushParam(0)));
    }

    #[test]
    fn test_compile_binary_expr() {
        let options = CompilerOptions::default();
        let codegen = CodeGenerator::new(&options);

        let ir = create_test_ir();
        // g * V(0,1)
        let expr = IrExpr::Binary(
            BinaryOp::Mul,
            Box::new(IrExpr::Param("g".into())),
            Box::new(IrExpr::Voltage(0, 1)),
        );
        let program = codegen.compile_expr(&expr, &ir).unwrap();

        assert_eq!(program.instructions.len(), 3);
        assert!(matches!(program.instructions[0], Instruction::PushParam(0)));
        assert!(matches!(
            program.instructions[1],
            Instruction::PushVoltage(0, 1)
        ));
        assert!(matches!(program.instructions[2], Instruction::Mul));
    }

    #[test]
    fn test_compile_function_expr() {
        let options = CompilerOptions::default();
        let codegen = CodeGenerator::new(&options);

        let ir = create_test_ir();
        // exp(V(0,1))
        let expr = IrExpr::Call(crate::ir::IrFunction::Exp, vec![IrExpr::Voltage(0, 1)]);
        let program = codegen.compile_expr(&expr, &ir).unwrap();

        assert_eq!(program.instructions.len(), 2);
        assert!(matches!(
            program.instructions[0],
            Instruction::PushVoltage(0, 1)
        ));
        assert!(matches!(program.instructions[1], Instruction::Exp));
    }

    #[test]
    fn test_compile_conditional_expr() {
        let options = CompilerOptions::default();
        let codegen = CodeGenerator::new(&options);

        let ir = create_test_ir();
        // cond ? 1.0 : 0.0
        let expr = IrExpr::Conditional(
            Box::new(IrExpr::Voltage(0, 1)),
            Box::new(IrExpr::Const(1.0)),
            Box::new(IrExpr::Const(0.0)),
        );
        let program = codegen.compile_expr(&expr, &ir).unwrap();

        assert_eq!(program.instructions.len(), 4);
        assert!(matches!(program.instructions[3], Instruction::IfElse));
    }

    #[test]
    fn test_compile_limexp() {
        let options = CompilerOptions::default();
        let codegen = CodeGenerator::new(&options);

        let ir = create_test_ir();
        let expr = IrExpr::Limexp(Box::new(IrExpr::Const(20.0)));
        let program = codegen.compile_expr(&expr, &ir).unwrap();

        assert_eq!(program.instructions.len(), 2);
        assert!(matches!(program.instructions[1], Instruction::Limexp));
    }

    #[test]
    fn test_compile_negation() {
        let options = CompilerOptions::default();
        let codegen = CodeGenerator::new(&options);

        let ir = create_test_ir();
        let expr = IrExpr::Unary(crate::ast::UnaryOp::Neg, Box::new(IrExpr::Const(5.0)));
        let program = codegen.compile_expr(&expr, &ir).unwrap();

        assert_eq!(program.instructions.len(), 2);
        assert!(matches!(program.instructions[0], Instruction::PushConst(_)));
        assert!(matches!(program.instructions[1], Instruction::Neg));
    }

    #[test]
    fn test_compile_system_vars() {
        let options = CompilerOptions::default();
        let codegen = CodeGenerator::new(&options);

        let ir = create_test_ir();

        // Temperature
        let temp = codegen.compile_expr(&IrExpr::Temperature, &ir).unwrap();
        assert!(matches!(temp.instructions[0], Instruction::PushTemperature));

        // Vt
        let vt = codegen.compile_expr(&IrExpr::Vt, &ir).unwrap();
        assert!(matches!(vt.instructions[0], Instruction::PushVt));

        // Time
        let time = codegen.compile_expr(&IrExpr::Time, &ir).unwrap();
        assert!(matches!(time.instructions[0], Instruction::PushTime));
    }

    #[test]
    fn test_compile_complex_expr() {
        let options = CompilerOptions::default();
        let codegen = CodeGenerator::new(&options);

        let ir = create_test_ir();
        // is * (exp(V(0,1) / vt) - 1)
        // Simplified: exp(V) - 1
        let expr = IrExpr::Binary(
            BinaryOp::Sub,
            Box::new(IrExpr::Call(
                crate::ir::IrFunction::Exp,
                vec![IrExpr::Voltage(0, 1)],
            )),
            Box::new(IrExpr::Const(1.0)),
        );
        let program = codegen.compile_expr(&expr, &ir).unwrap();

        // PushVoltage, Exp, PushConst(1), Sub
        assert_eq!(program.instructions.len(), 4);
    }

    #[test]
    fn test_stamp_location() {
        let loc = StampLocation {
            row: StampIndex::Terminal(0),
            col: StampIndex::Terminal(1),
            sign: 1.0,
        };
        assert!(matches!(loc.row, StampIndex::Terminal(0)));
        assert!(matches!(loc.col, StampIndex::Terminal(1)));
        assert!((loc.sign - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_stamp_index_variants() {
        let term = StampIndex::Terminal(0);
        let internal = StampIndex::Internal(1);
        let ground = StampIndex::Ground;

        assert!(matches!(term, StampIndex::Terminal(0)));
        assert!(matches!(internal, StampIndex::Internal(1)));
        assert!(matches!(ground, StampIndex::Ground));
    }

    // ========================================================================
    // Helper Functions
    // ========================================================================

    fn create_test_ir() -> crate::ir::DeviceIR {
        crate::ir::DeviceIR {
            name: "test".into(),
            terminals: vec![
                crate::ir::Terminal {
                    name: "p".into(),
                    index: 0,
                },
                crate::ir::Terminal {
                    name: "n".into(),
                    index: 1,
                },
            ],
            internal_nodes: vec![],
            parameters: vec![crate::ir::ParamDef {
                name: "g".into(),
                default: 0.001,
                min: Some(0.0),
                max: None,
            }],
            variables: vec![],
            assignments: vec![],
            equations: vec![],
            noise_sources: vec![],
        }
    }
}
