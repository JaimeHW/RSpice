//! AST to bytecode compiler
//!
//! Converts expression AST into efficient bytecode for the VM.

use super::ast::{BinaryOp, Expr, Function, UnaryOp};
use super::vm::{CompiledExpr, Instruction};

/// Compile an expression AST to bytecode
pub fn compile(expr: &Expr) -> CompiledExpr {
    let mut program = CompiledExpr::new();
    compile_expr(expr, &mut program);
    program
}

fn compile_expr(expr: &Expr, program: &mut CompiledExpr) {
    match expr {
        Expr::Const(value) => {
            program.instructions.push(Instruction::PushConst(*value));
        }

        Expr::Time => {
            program.instructions.push(Instruction::PushTime);
        }

        Expr::Frequency => {
            program.instructions.push(Instruction::PushFreq);
        }

        Expr::NodeVoltage(node) => {
            let idx = program.get_or_create_node(node);
            program.instructions.push(Instruction::LoadVoltage(idx));
        }

        Expr::BranchCurrent(branch) => {
            let idx = program.get_or_create_branch(branch);
            program.instructions.push(Instruction::LoadCurrent(idx));
        }

        Expr::Binary { op, left, right } => {
            // Compile operands first (left-to-right)
            compile_expr(left, program);
            compile_expr(right, program);

            // Then the operation
            let instr = match op {
                BinaryOp::Add => Instruction::Add,
                BinaryOp::Sub => Instruction::Sub,
                BinaryOp::Mul => Instruction::Mul,
                BinaryOp::Div => Instruction::Div,
                BinaryOp::Pow => Instruction::Pow,
                BinaryOp::Lt => Instruction::Lt,
                BinaryOp::Le => Instruction::Le,
                BinaryOp::Gt => Instruction::Gt,
                BinaryOp::Ge => Instruction::Ge,
                BinaryOp::Eq => Instruction::Eq,
                BinaryOp::Ne => Instruction::Ne,
                BinaryOp::And => Instruction::And,
                BinaryOp::Or => Instruction::Or,
            };
            program.instructions.push(instr);
        }

        Expr::Unary { op, operand } => {
            compile_expr(operand, program);

            let instr = match op {
                UnaryOp::Neg => Instruction::Neg,
                UnaryOp::Not => Instruction::Not,
            };
            program.instructions.push(instr);
        }

        Expr::Function { func, args } => {
            // Compile all arguments
            for arg in args {
                compile_expr(arg, program);
            }

            // Push the function instruction
            let instr = match func {
                Function::Abs => Instruction::Abs,
                Function::Sqrt => Instruction::Sqrt,
                Function::Exp => Instruction::Exp,
                Function::Log => Instruction::Log,
                Function::Log10 => Instruction::Log10,
                Function::Sin => Instruction::Sin,
                Function::Cos => Instruction::Cos,
                Function::Tan => Instruction::Tan,
                Function::Asin => Instruction::Asin,
                Function::Acos => Instruction::Acos,
                Function::Atan => Instruction::Atan,
                Function::Atan2 => Instruction::Atan2,
                Function::Sinh => Instruction::Sinh,
                Function::Cosh => Instruction::Cosh,
                Function::Tanh => Instruction::Tanh,
                Function::Floor => Instruction::Floor,
                Function::Ceil => Instruction::Ceil,
                Function::Round => Instruction::Round,
                Function::Min => Instruction::Min,
                Function::Max => Instruction::Max,
                Function::Pwr => Instruction::Pwr,
                Function::Pwrs => Instruction::Pwr, // Simplified - same as pwr for now
                Function::Limit => Instruction::Limit,
                Function::Sign => Instruction::Sign,
                Function::Uramp => Instruction::Uramp,
                Function::Stp => Instruction::Stp,
                Function::Mod => Instruction::Mod,
                Function::If => Instruction::IfElse,
            };
            program.instructions.push(instr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::vm::{Context, Vm};

    #[test]
    fn test_compile_simple() {
        // V(2) * 2
        let expr = Expr::mul(Expr::voltage("2"), Expr::constant(2.0));

        let program = compile(&expr);

        assert_eq!(program.instructions.len(), 3);
        assert!(program.node_map.contains_key("2"));
    }

    #[test]
    fn test_compile_and_execute() {
        // V(1) + V(2)
        let expr = Expr::add(Expr::voltage("1"), Expr::voltage("2"));

        let program = compile(&expr);

        // Map nodes to voltages
        let mut voltages = vec![0.0; program.node_map.len()];
        if let Some(&idx) = program.node_map.get("1") {
            voltages[idx] = 3.0;
        }
        if let Some(&idx) = program.node_map.get("2") {
            voltages[idx] = 5.0;
        }

        let mut vm = Vm::new();
        let ctx = Context::dc(&voltages, &[]);
        let result = vm.execute(&program, &ctx);

        assert!((result - 8.0).abs() < 1e-10);
    }

    #[test]
    fn test_compile_function() {
        // sin(V(1))
        let expr = Expr::Function {
            func: Function::Sin,
            args: vec![Expr::voltage("1")],
        };

        let program = compile(&expr);

        let voltages = [std::f64::consts::PI / 2.0];
        let mut vm = Vm::new();
        let ctx = Context::dc(&voltages, &[]);
        let result = vm.execute(&program, &ctx);

        assert!((result - 1.0).abs() < 1e-10);
    }
}
