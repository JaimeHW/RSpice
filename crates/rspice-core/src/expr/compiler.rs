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

        Expr::Temperature => {
            program.instructions.push(Instruction::PushTemperature);
        }

        Expr::NodeVoltage(node) => {
            let idx = program.get_or_create_node(node);
            program.instructions.push(Instruction::LoadVoltage(idx));
        }

        Expr::BranchCurrent(branch) => {
            let idx = program.get_or_create_branch(branch);
            program.instructions.push(Instruction::LoadCurrent(idx));
        }

        Expr::StringLiteral(_) => {
            program.instructions.push(Instruction::PushConst(0.0));
        }

        Expr::LookupTable(table) => {
            let index = program.add_lookup_table(table.clone());
            program.instructions.push(Instruction::LookupTable(index));
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
                Function::Ln => Instruction::Ln,
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
                Function::Asinh => Instruction::Asinh,
                Function::Acosh => Instruction::Acosh,
                Function::Atanh => Instruction::Atanh,
                Function::Floor => Instruction::Floor,
                Function::Ceil => Instruction::Ceil,
                Function::Round => Instruction::Round,
                Function::Sqr => Instruction::Sqr,
                Function::Min => Instruction::Min,
                Function::Max => Instruction::Max,
                Function::Pwr => Instruction::Pwr,
                Function::Pwrs => Instruction::Pwrs,
                Function::Limit => Instruction::Limit,
                Function::Sign => Instruction::Sign,
                Function::Uramp => Instruction::Uramp,
                Function::Stp => Instruction::Stp,
                Function::U2 => Instruction::U2,
                Function::Eq0 => Instruction::Eq0,
                Function::Ne0 => Instruction::Ne0,
                Function::Gt0 => Instruction::Gt0,
                Function::Lt0 => Instruction::Lt0,
                Function::Ge0 => Instruction::Ge0,
                Function::Le0 => Instruction::Le0,
                Function::Pow => Instruction::Pow,
                Function::Table => Instruction::Table(args.len()),
                Function::Pwl => Instruction::Pwl(args.len()),
                Function::TableFile | Function::FastTable | Function::FastTableFile => {
                    Instruction::PushConst(0.0)
                }
                Function::Mod => Instruction::Mod,
                Function::SpicePulse => Instruction::SpicePulse(args.len()),
                Function::SpiceSin => Instruction::SpiceSin(args.len()),
                Function::SpiceExp => Instruction::SpiceExp(args.len()),
                Function::SpiceSffm => Instruction::SpiceSffm(args.len()),
                Function::If => Instruction::IfElse,
            };
            program.instructions.push(instr);
        }
    }
}
