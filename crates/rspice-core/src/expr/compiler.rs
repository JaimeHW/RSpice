//! AST to bytecode compiler
//!
//! Converts expression AST into efficient bytecode for the VM.

use super::ast::{BinaryOp, Expr, Function, UnaryOp};
use super::vm::{CompiledExpr, Context, Instruction, Vm};
use crate::Value;

/// Compile an expression AST to bytecode
pub fn compile(expr: &Expr) -> CompiledExpr {
    let mut program = CompiledExpr::new();
    compile_expr(expr, &mut program, None);
    program
}

/// Specialize time-independent subexpressions to one explicitly fixed
/// environment. Use the ordinary VM for constants, preserving dialect and
/// evaluation order; no floating-point algebraic reassociation is performed.
pub(crate) fn compile_time_expression(expr: &Expr, context: &Context<'_>) -> CompiledExpr {
    let mut program = CompiledExpr::new();
    compile_expr(expr, &mut program, Some(context));
    program
}

pub(crate) fn function_uses_implicit_time(function: Function) -> bool {
    matches!(
        function,
        Function::Sdt
            | Function::SpiceSin
            | Function::SpicePulse
            | Function::SpiceExp
            | Function::SpiceSffm
    )
}

pub(crate) fn constant_over_time(expr: &Expr) -> bool {
    match expr {
        Expr::Time | Expr::NodeVoltage(_) | Expr::BranchCurrent(_) | Expr::StringLiteral(_) => {
            false
        }
        Expr::Unary { operand, .. } => constant_over_time(operand),
        Expr::Binary { left, right, .. } => constant_over_time(left) && constant_over_time(right),
        Expr::Function { func, args } => {
            !function_uses_implicit_time(*func) && args.iter().all(constant_over_time)
        }
        Expr::LookupTable { input, .. } => constant_over_time(input),
        _ => true,
    }
}

pub(crate) fn constant_value(expr: &Expr, context: &Context<'_>) -> Option<Value> {
    if !constant_over_time(expr) {
        return None;
    }
    // This result may be an operand inside a larger expression. Xyce's
    // nonfinite replacement belongs only at the completed source boundary;
    // applying it here would turn, for example, time/exp(1000) into time/1e50.
    let value = Vm::new().execute_raw(&compile(expr), context);
    value.is_finite().then_some(value)
}

/// Reuse an already evaluated sibling without changing arithmetic order or
/// adding a cache. Callers exclude siblings with independent SDT state.
fn reuse_top(expr: &Expr, previous: &Expr, program: &mut CompiledExpr) -> bool {
    if expr == previous {
        program.instructions.push(Instruction::Dup);
        return true;
    }
    if let Expr::Unary { op, operand } = expr
        && reuse_top(operand, previous, program)
    {
        program.instructions.push(match op {
            UnaryOp::Neg => Instruction::Neg,
            UnaryOp::Not => Instruction::Not,
        });
        return true;
    }
    false
}

fn compile_expr(expr: &Expr, program: &mut CompiledExpr, context: Option<&Context<'_>>) {
    if let Some(value) = context.and_then(|context| constant_value(expr, context)) {
        program.instructions.push(Instruction::PushConst(value));
        return;
    }
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

        Expr::ThermalVoltage => {
            program.instructions.push(Instruction::PushThermalVoltage);
        }

        Expr::Gmin => {
            program.instructions.push(Instruction::PushGmin);
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

        Expr::LookupTable { input, table } => {
            compile_expr(input, program, context);
            let index = program.add_lookup_table(table.clone());
            program.instructions.push(Instruction::LookupTable(index));
        }

        Expr::Binary { op, left, right } => {
            // Compile operands first (left-to-right)
            let states = program.sdt_count;
            compile_expr(left, program, context);
            if context.is_none() || states != program.sdt_count || !reuse_top(right, left, program)
            {
                compile_expr(right, program, context);
            }

            // Then the operation
            let instr = match op {
                BinaryOp::Add => Instruction::Add,
                BinaryOp::Sub => Instruction::Sub,
                BinaryOp::Mul => Instruction::Mul,
                BinaryOp::Div => Instruction::Div,
                BinaryOp::Mod => Instruction::Mod,
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
            compile_expr(operand, program, context);

            let instr = match op {
                UnaryOp::Neg => Instruction::Neg,
                UnaryOp::Not => Instruction::Not,
            };
            program.instructions.push(instr);
        }

        Expr::Function { func, args } => {
            // Compile all arguments
            let mut previous = None;
            for arg in args {
                let states = program.sdt_count;
                if previous.is_none_or(|previous| !reuse_top(arg, previous, program)) {
                    compile_expr(arg, program, context);
                }
                previous = (context.is_some() && states == program.sdt_count).then_some(arg);
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
                Function::Trunc => Instruction::Trunc,
                Function::Floor => Instruction::Floor,
                Function::Ceil => Instruction::Ceil,
                Function::Round => Instruction::Round,
                Function::Sqr => Instruction::Sqr,
                Function::Min => Instruction::Min(args.len()),
                Function::Max => Instruction::Max(args.len()),
                Function::Pwr => Instruction::Pwr,
                Function::Pwrs => Instruction::Pwrs,
                Function::Limit => Instruction::Limit(args.len()),
                Function::Sign => Instruction::Sign,
                Function::HspiceSign => Instruction::HspiceSign,
                Function::Uramp => Instruction::Uramp,
                Function::Stp => Instruction::Stp,
                Function::Ustep => Instruction::Ustep,
                Function::U2 => Instruction::U2,
                Function::Eq0 => Instruction::Eq0,
                Function::Ne0 => Instruction::Ne0,
                Function::Gt0 => Instruction::Gt0,
                Function::Lt0 => Instruction::Lt0,
                Function::Ge0 => Instruction::Ge0,
                Function::Le0 => Instruction::Le0,
                Function::Pow => Instruction::FunctionPow,
                Function::Table => Instruction::Table(args.len()),
                Function::Pwl => Instruction::Pwl(args.len()),
                Function::TableFile
                | Function::FastTable
                | Function::FastTableFile
                | Function::Cubic
                | Function::CubicFile
                | Function::Akima
                | Function::AkimaFile
                | Function::Wodicka
                | Function::WodickaFile
                | Function::Barycentric
                | Function::BarycentricFile => Instruction::PushConst(0.0),
                Function::Sdt => Instruction::Sdt(program.add_sdt()),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ExpressionDialect;
    use crate::expr::{TimeEnclosure, parse_expression_strict};

    #[test]
    fn fixed_environment_keeps_nonfinite_intermediates_until_the_final_boundary() {
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            let context = Context::transient(&[], &[], 0.0).with_expression_dialect(dialect);
            for expression in [
                "time/exp(1000)",
                "exp(1000)/(exp(1000)+time)",
                "0*exp(1000)+time",
                "exp(1000)>time",
                "if(exp(1000)>time,time,-time)",
                "time/(0*exp(1000))",
            ] {
                let ast = parse_expression_strict(expression).unwrap();
                let original = compile(&ast);
                let specialized = compile_time_expression(&ast, &context);
                for time in [-1e100, -1.0, 0.0, 1.0, 1e100] {
                    let point = Context { time, ..context };
                    let expected = Vm::new().execute(&original, &point);
                    let actual = Vm::new().execute(&specialized, &point);
                    if expected.is_nan() {
                        assert!(actual.is_nan(), "{dialect:?}: {expression} at {time}");
                    } else {
                        assert_eq!(
                            actual.to_bits(),
                            expected.to_bits(),
                            "{dialect:?}: {expression} at {time}"
                        );
                    }
                }
            }
            for expression in ["exp(1000)", "0*exp(1000)"] {
                assert!(
                    constant_value(&parse_expression_strict(expression).unwrap(), &context)
                        .is_none()
                );
            }
        }
    }

    #[test]
    fn fixed_environment_reuses_stateless_siblings_bit_exactly() {
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            let context = Context::transient(&[], &[], 0.0).with_expression_dialect(dialect);
            for expression in [
                "max(cos(time),-cos(time))",
                "min(cos(time),-cos(time),1)",
                "cos(time)-cos(time)",
                "time*(-time)",
                "max(exp(time),-exp(time))",
                "max(0*exp(time),-(0*exp(time)))",
                "pow(time,-time)",
            ] {
                let expression = parse_expression_strict(expression).unwrap();
                let original = compile(&expression);
                let specialized = compile_time_expression(&expression, &context);
                assert!(
                    specialized
                        .instructions
                        .iter()
                        .any(|i| matches!(i, Instruction::Dup))
                );
                for time in [-1000.0, -1.0, -0.0, 0.0, 0.123, 1.0, 1000.0] {
                    let point = Context { time, ..context };
                    let expected = Vm::new().execute(&original, &point);
                    let actual = Vm::new().execute(&specialized, &point);
                    if expected.is_nan() {
                        assert!(actual.is_nan());
                    } else {
                        assert_eq!(actual.to_bits(), expected.to_bits(), "{expression:?}");
                    }
                }
            }
            for expression in ["sdt(time)+sdt(time)", "max(sdt(time),-sdt(time))"] {
                let expression = parse_expression_strict(expression).unwrap();
                let specialized = compile_time_expression(&expression, &context);
                assert_eq!(specialized.sdt_count, 2);
            }
        }
    }

    #[test]
    fn fixed_environment_specialization_preserves_vm_values_and_stateful_sites() {
        let expression =
            parse_expression_strict("cos(6*pi*time)+sqrt(2)/7+pow(-2,0.5)+temper*1e-3").unwrap();
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            let context = Context::transient(&[], &[], 0.0)
                .with_temperature(57.0)
                .with_expression_dialect(dialect);
            let original = compile(&expression);
            let specialized = compile_time_expression(&expression, &context);
            assert!(specialized.instructions.len() < original.instructions.len());
            assert!(TimeEnclosure::new(&specialized, 1.0).is_some());
            for time in [0.0, 0.123, 0.99, 1.0] {
                let context = Context { time, ..context };
                assert_eq!(
                    Vm::new().execute(&original, &context).to_bits(),
                    Vm::new().execute(&specialized, &context).to_bits()
                );
            }
        }
        let context = Context::transient(&[], &[], 0.0);
        let invalid = parse_expression_strict("0*exp(1000)+time").unwrap();
        assert!(
            Vm::new()
                .execute(&compile_time_expression(&invalid, &context), &context)
                .is_nan()
        );
        let integral = parse_expression_strict("sdt(1)+sdt(2)").unwrap();
        assert_eq!(compile_time_expression(&integral, &context).sdt_count, 2);
        let sine = parse_expression_strict("spice_sin(0,1,3)").unwrap();
        assert!(constant_value(&sine, &context).is_none());
        assert!(
            compile_time_expression(&sine, &context)
                .instructions
                .iter()
                .any(|instruction| matches!(instruction, Instruction::SpiceSin(3)))
        );
    }
}
