//! The value-returning system functions of analog code: `$random`,
//! `$arandom`, the `$dist_*` and `$rdist_*` distributions, `$realtobits`
//! (and `$bitstoreal` of it), `$test$plusargs`, `$value$plusargs`, `$fopen`
//! and `$fscanf` (LRM 2.4 sections 9.5, 9.10 and 9.13).
//!
//! A call returns a value and may assign variables: a seed variable, the
//! targets of `$fscanf` or `$value$plusargs`. An expression cannot assign, so
//! each call is materialized at its statement boundary, the way a call of an
//! analog function with output arguments is: the call is assigned to a
//! private temporary, each variable it assigns is assigned from a read of the
//! call's output, and the expression reads the temporary. Every route then
//! evaluates the call, with its source order and its guard, as a statement.
//!
//! The call's site holds what is not a number: the function, where its seed
//! lives, and its string literals. The module keeps one site per call.

use rspice_veriloga_runtime::{AnalogSeed, AnalogSystemFunction, AnalogSystemSite};
use smol_str::SmolStr;

use super::{AnalyzedModule, AnalyzedStatement, SemanticAnalyzer, SymbolKind};
use crate::analog_tasks::{AnalogSystemCall, SYSTEM_CALL, SYSTEM_OUTPUT};
use crate::ast::{
    AssignmentStmt, Expression, Identifier, LValue, NumberLit, SystemFunction, UnaryExpr, UnaryOp,
    VarType,
};
use crate::error::{CompileError, CompileResult, SemanticError, SemanticErrorKind};
use crate::source::Span;
use crate::types::ValueType;

/// Whether `name` is one of the internal names a materialized call takes.
pub(crate) fn is_internal_system_name(name: &str) -> bool {
    name == SYSTEM_CALL || name == SYSTEM_OUTPUT
}

fn refuse(name: &str, detail: impl std::fmt::Display, span: Span) -> CompileError {
    CompileError::Semantic(SemanticError::new(
        SemanticErrorKind::InvalidAnalogOperator(format!("`{name}`: {detail}")),
        span,
    ))
}

fn arity(name: &str, expected: &str, got: usize, span: Span) -> CompileError {
    CompileError::Semantic(SemanticError::new(
        SemanticErrorKind::ArgumentCountMismatch {
            name: name.to_string(),
            expected: expected.to_string(),
            got,
        },
        span,
    ))
}

/// `$fopen` modes (LRM 2.4 table 9-24).
const FILE_MODES: [&str; 15] = [
    "r", "rb", "w", "wb", "a", "ab", "r+", "r+b", "rb+", "w+", "w+b", "wb+", "a+", "a+b", "ab+",
];

/// A seed argument, as LRM 2.4 section 9.13 reads it.
enum Seed {
    /// An integer variable, which the call updates.
    Variable(SmolStr),
    /// An integer parameter or a constant, which starts an internal seed.
    Constant(Expression),
}

/// A variable a call assigns.
struct Target {
    name: SmolStr,
    integer: bool,
    span: Span,
}

impl SemanticAnalyzer {
    /// Whether materialization turns `function` into statements. A constant
    /// declaration cannot run one, so it reports the call as unsupported.
    pub(super) fn is_system_value_call(&self, function: &SystemFunction) -> bool {
        !self.lowering_declaration_constant
            && (AnalogSystemFunction::from_name(&function.name).is_some()
                || function.name == "$bitstoreal")
    }

    /// Materialize `function` into `sink` and return the expression that reads
    /// its value, or `None` when it is not a value-returning system function.
    pub(super) fn materialize_system_value_call(
        &mut self,
        function: &SystemFunction,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<Option<Expression>> {
        if !self.is_system_value_call(function) {
            return Ok(None);
        }
        let name = function.name.as_str();
        let span = function.span;
        if name == "$bitstoreal" {
            // IEEE 1364-2005 section 17.8: the argument is a 64-bit pattern,
            // which no analog integer holds; Icarus refuses a 32-bit one. The
            // pattern `$realtobits` gives is the one analog code can pass.
            return match function.args.as_slice() {
                [Expression::SystemFunction(inner)]
                    if inner.name == "$realtobits" && inner.args.len() == 1 =>
                {
                    let value =
                        self.materialize_output_function_calls(&inner.args[0], module, sink)?;
                    Ok(Some(Expression::Unary(UnaryExpr {
                        op: UnaryOp::ToReal,
                        operand: Box::new(value),
                        span,
                    })))
                }
                [_] => Err(refuse(
                    name,
                    "the argument must be the 64-bit pattern `$realtobits` returns; an analog \
                     integer holds only 32 bits",
                    span,
                )),
                args => Err(arity(name, "1", args.len(), span)),
            };
        }
        let kind = AnalogSystemFunction::from_name(name).expect("checked above");
        let args = function.args.as_slice();
        let mut operands = Vec::new();
        let mut outputs = Vec::new();
        let mut text = Vec::new();
        let mut seed = AnalogSeed::Stream;
        match kind {
            _ if kind.is_random() => {
                let parameters = kind.distribution_parameters();
                let takes_type =
                    kind == AnalogSystemFunction::Arandom || kind.is_real_distribution();
                let mut args = args;
                if takes_type && let Some(Expression::StringLit(literal)) = args.last() {
                    if !matches!(literal.value.as_str(), "global" | "instance") {
                        return Err(refuse(
                            name,
                            format!(
                                "the type string is \"global\" or \"instance\", not \"{}\"",
                                literal.value
                            ),
                            literal.span,
                        ));
                    }
                    args = &args[..args.len() - 1];
                }
                let optional_seed = matches!(
                    kind,
                    AnalogSystemFunction::Random | AnalogSystemFunction::Arandom
                );
                let expected = 1 + parameters;
                if args.len() != expected && !(optional_seed && args.is_empty()) {
                    let expected = if optional_seed {
                        "0..1".to_string()
                    } else {
                        expected.to_string()
                    };
                    return Err(arity(name, &expected, function.args.len(), span));
                }
                if let Some(argument) = args.first() {
                    let constant_allowed =
                        kind == AnalogSystemFunction::Arandom || kind.is_real_distribution();
                    match self.seed_argument(argument) {
                        Some(Seed::Variable(variable)) => {
                            seed = AnalogSeed::Variable;
                            operands.push(Expression::Identifier(Identifier {
                                name: variable.clone(),
                                span: argument.span(),
                            }));
                            outputs.push(Target {
                                name: variable,
                                integer: true,
                                span: argument.span(),
                            });
                        }
                        Some(Seed::Constant(value)) if constant_allowed => {
                            seed = AnalogSeed::Internal;
                            operands.push(value);
                        }
                        _ => {
                            let accepted = if constant_allowed {
                                "an integer variable, an integer parameter or a constant"
                            } else {
                                "an integer variable"
                            };
                            return Err(refuse(
                                name,
                                format!("the seed must be {accepted} (LRM 2.4 section 9.13)"),
                                argument.span(),
                            ));
                        }
                    }
                    for argument in &args[1..] {
                        operands
                            .push(self.materialize_output_function_calls(argument, module, sink)?);
                    }
                }
            }
            AnalogSystemFunction::RealToBits => {
                let [argument] = args else {
                    return Err(arity(name, "1", args.len(), span));
                };
                operands.push(self.materialize_output_function_calls(argument, module, sink)?);
            }
            AnalogSystemFunction::TestPlusargs => {
                let [argument] = args else {
                    return Err(arity(name, "1", args.len(), span));
                };
                text.push(literal(name, argument, "the plusarg name")?);
            }
            AnalogSystemFunction::ValuePlusargs => {
                let [format, target] = args else {
                    return Err(arity(name, "2", args.len(), span));
                };
                let format_text = literal(name, format, "the format")?;
                let target = self.assigned_variable(name, target)?;
                let conversion = format_text
                    .split_once('%')
                    .and_then(|(_, rest)| rest.chars().find(|c| !c.is_ascii_digit()))
                    .map(|c| c.to_ascii_lowercase());
                if !matches!(
                    conversion,
                    Some('d' | 'o' | 'h' | 'x' | 'b' | 'e' | 'f' | 'g')
                ) {
                    return Err(refuse(
                        name,
                        format!(
                            "format \"{format_text}\" needs one of %d %o %h %b %e %f %g for an \
                             analog variable (IEEE 1364-2005 section 17.10.2)"
                        ),
                        format.span(),
                    ));
                }
                text.push(format_text);
                outputs.push(target);
            }
            AnalogSystemFunction::Fopen => {
                let (file, mode) = match args {
                    [file] => (file, None),
                    [file, mode] => (file, Some(mode)),
                    _ => return Err(arity(name, "1..2", args.len(), span)),
                };
                text.push(literal(name, file, "the file name")?);
                if let Some(mode) = mode {
                    let mode_text = literal(name, mode, "the mode")?;
                    if !FILE_MODES.contains(&mode_text.as_str()) {
                        return Err(refuse(
                            name,
                            format!(
                                "mode \"{mode_text}\" is not one of LRM 2.4 table 9-24's: {}",
                                FILE_MODES.join(" ")
                            ),
                            mode.span(),
                        ));
                    }
                    text.push(mode_text);
                }
            }
            AnalogSystemFunction::Fscanf => {
                let [descriptor, format, targets @ ..] = args else {
                    return Err(arity(name, "2+", args.len(), span));
                };
                operands.push(self.materialize_output_function_calls(descriptor, module, sink)?);
                text.push(literal(name, format, "the format")?);
                for target in targets {
                    outputs.push(self.assigned_variable(name, target)?);
                }
            }
            _ => unreachable!("every analog system function is matched"),
        }
        let targets = match kind {
            AnalogSystemFunction::Fscanf | AnalogSystemFunction::ValuePlusargs => {
                outputs.iter().map(|target| target.integer).collect()
            }
            _ => Vec::new(),
        };
        let site = u32::try_from(module.analog_system_calls.len())
            .expect("system-function sites are fewer than u32::MAX");
        module.analog_system_calls.push(AnalogSystemCall {
            site: AnalogSystemSite {
                function: kind,
                seed,
                text,
                targets,
                file: String::new(),
                line: 0,
            },
            span,
        });

        self.local_counter += 1;
        let result: SmolStr = format!("__sys{}@result", self.local_counter).into();
        let var_type = if kind.returns_integer() {
            VarType::Integer
        } else {
            VarType::Real
        };
        self.register_function_temp(module, result.clone(), var_type, span)?;
        let number = |value: f64| {
            Expression::Number(NumberLit {
                value,
                raw: value.to_string().into(),
                span,
            })
        };
        let mut call_args = vec![number(f64::from(site))];
        call_args.extend(operands);
        let call = AssignmentStmt {
            target: LValue::Variable {
                name: result.clone(),
                span,
            },
            value: Expression::SystemFunction(SystemFunction {
                name: SYSTEM_CALL.into(),
                args: call_args,
                span,
            }),
            span,
        };
        self.analyze_assignment(&call, module, sink)?;
        for (index, target) in outputs.into_iter().enumerate() {
            let read = AssignmentStmt {
                target: LValue::Variable {
                    name: target.name.clone(),
                    span: target.span,
                },
                value: Expression::SystemFunction(SystemFunction {
                    name: SYSTEM_OUTPUT.into(),
                    args: vec![
                        number(f64::from(site)),
                        number(index as f64),
                        Expression::Identifier(Identifier {
                            name: target.name,
                            span: target.span,
                        }),
                    ],
                    span,
                }),
                span: target.span,
            };
            self.analyze_assignment(&read, module, sink)?;
        }
        Ok(Some(Expression::Identifier(Identifier {
            name: result,
            span,
        })))
    }

    /// What a seed argument names, after the enclosing function's formals are
    /// substituted.
    fn seed_argument(&self, argument: &Expression) -> Option<Seed> {
        let substituted = match argument {
            Expression::Identifier(identifier) => self.lookup_substitution(&identifier.name),
            _ => None,
        };
        let argument = substituted.as_ref().unwrap_or(argument);
        match argument {
            Expression::Identifier(identifier) => {
                let symbol = self.symbols.lookup(&identifier.name)?;
                match symbol.kind {
                    SymbolKind::Variable if symbol.value_type == ValueType::Integer => {
                        Some(Seed::Variable(identifier.name.clone()))
                    }
                    SymbolKind::Parameter if symbol.value_type == ValueType::Integer => {
                        Some(Seed::Constant(argument.clone()))
                    }
                    SymbolKind::LoopVar => Some(Seed::Constant(argument.clone())),
                    _ => None,
                }
            }
            Expression::Number(number) if number.value.fract() == 0.0 => {
                Some(Seed::Constant(argument.clone()))
            }
            Expression::Unary(UnaryExpr {
                op: UnaryOp::Neg | UnaryOp::Pos,
                operand,
                ..
            }) if matches!(&**operand, Expression::Number(number) if number.value.fract() == 0.0) => {
                Some(Seed::Constant(argument.clone()))
            }
            _ => None,
        }
    }

    /// A variable a call assigns: an integer or real variable, named.
    fn assigned_variable(&self, name: &str, argument: &Expression) -> CompileResult<Target> {
        if let Expression::Identifier(identifier) = argument {
            let resolved = self.resolve_substituted_name(&identifier.name);
            if let Some(symbol) = self.symbols.lookup(&resolved)
                && symbol.kind == SymbolKind::Variable
                && matches!(symbol.value_type, ValueType::Integer | ValueType::Real)
            {
                return Ok(Target {
                    name: resolved,
                    integer: symbol.value_type == ValueType::Integer,
                    span: identifier.span,
                });
            }
        }
        Err(refuse(
            name,
            "the argument it assigns must be an integer or real variable",
            argument.span(),
        ))
    }
}

/// A string literal argument's text.
fn literal(name: &str, argument: &Expression, what: &str) -> CompileResult<String> {
    match argument {
        Expression::StringLit(literal) => Ok(literal.value.to_string()),
        _ => Err(refuse(
            name,
            format!("{what} must be a string literal"),
            argument.span(),
        )),
    }
}
