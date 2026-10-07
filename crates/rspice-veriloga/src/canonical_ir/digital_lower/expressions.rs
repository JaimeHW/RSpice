//! Iterative operator typing and lowering, shared by constants and processes.
//!
//! Shape is computed bottom-up once, then width/sign context travels top-down.
//! The borrowed AST remains fixed throughout each walk; pointer keys identify
//! its nodes only within that walk and are never dereferenced or persisted.

use super::*;
use crate::ast::DigitalExpr;

#[derive(Clone, Copy)]
pub(super) struct Shape {
    pub width: u32,
    pub signed: bool,
    pub real: bool,
}

fn children(expression: &Expression) -> Vec<&Expression> {
    match expression {
        Expression::Binary(value) => vec![&value.left, &value.right],
        Expression::Unary(value) => vec![&value.operand],
        Expression::Conditional(value) => {
            vec![&value.condition, &value.then_expr, &value.else_expr]
        }
        Expression::Digital(DigitalExpr::Xnor(value)) => vec![&value.left, &value.right],
        Expression::Digital(DigitalExpr::ArithmeticShiftRight(value)) => {
            vec![&value.left, &value.right]
        }
        Expression::Digital(DigitalExpr::CaseEquality(value)) => vec![&value.left, &value.right],
        Expression::Digital(DigitalExpr::Reduction(value)) => vec![&value.operand],
        Expression::Call(value) => value.args.iter().collect(),
        Expression::SystemFunction(value) => value.args.iter().collect(),
        _ => Vec::new(),
    }
}

fn key(expression: &Expression) -> *const Expression {
    expression
}

fn shapes(lowerer: &ProcessLowerer<'_>, root: &Expression) -> HashMap<*const Expression, Shape> {
    let mut result: HashMap<*const Expression, Shape> = HashMap::new();
    let mut pending = vec![(root, false)];
    while let Some((expression, ready)) = pending.pop() {
        if !ready {
            pending.push((expression, true));
            pending.extend(
                children(expression)
                    .into_iter()
                    .rev()
                    .map(|child| (child, false)),
            );
            continue;
        }
        let get = |expression: &Expression| result[&key(expression)];
        let shape = match expression {
            Expression::Binary(value) => {
                let left: Shape = get(&value.left);
                let right: Shape = get(&value.right);
                let arithmetic = matches!(
                    value.op,
                    BinaryOp::Add
                        | BinaryOp::Sub
                        | BinaryOp::Mul
                        | BinaryOp::Div
                        | BinaryOp::Mod
                        | BinaryOp::Pow
                );
                let (width, signed) = match value.op {
                    BinaryOp::CheckedValue
                    | BinaryOp::DiscreteValue
                    | BinaryOp::IntAdd
                    | BinaryOp::IntSub
                    | BinaryOp::IntMul
                    | BinaryOp::IntDiv
                    | BinaryOp::IntMod
                    | BinaryOp::IntPow => (32, true),
                    BinaryOp::And
                    | BinaryOp::Or
                    | BinaryOp::Eq
                    | BinaryOp::Ne
                    | BinaryOp::Lt
                    | BinaryOp::Le
                    | BinaryOp::Gt
                    | BinaryOp::Ge => (1, false),
                    BinaryOp::Shl | BinaryOp::Shr | BinaryOp::Pow => (left.width, left.signed),
                    _ => (left.width.max(right.width), left.signed && right.signed),
                };
                Shape {
                    width,
                    signed,
                    real: arithmetic && (left.real || right.real),
                }
            }
            Expression::Unary(value) => {
                let input: Shape = get(&value.operand);
                match value.op {
                    UnaryOp::ToInteger => Shape {
                        width: 32,
                        signed: true,
                        real: false,
                    },
                    UnaryOp::Not => Shape {
                        width: 1,
                        signed: false,
                        real: false,
                    },
                    UnaryOp::BitNot => Shape {
                        real: false,
                        ..input
                    },
                    UnaryOp::Pos | UnaryOp::Neg => input,
                }
            }
            Expression::Conditional(value) => {
                let left: Shape = get(&value.then_expr);
                let right: Shape = get(&value.else_expr);
                Shape {
                    width: left.width.max(right.width),
                    signed: left.signed && right.signed,
                    real: left.real || right.real,
                }
            }
            Expression::Digital(DigitalExpr::Xnor(value)) => {
                let left: Shape = get(&value.left);
                let right: Shape = get(&value.right);
                Shape {
                    width: left.width.max(right.width),
                    signed: left.signed && right.signed,
                    real: false,
                }
            }
            Expression::Digital(DigitalExpr::ArithmeticShiftRight(value)) => Shape {
                real: false,
                ..get(&value.left)
            },
            Expression::Digital(DigitalExpr::CaseEquality(_) | DigitalExpr::Reduction(_)) => {
                Shape {
                    width: 1,
                    signed: false,
                    real: false,
                }
            }
            _ => Shape {
                width: lowerer.self_width_leaf(expression),
                signed: lowerer.self_signed_leaf(expression),
                real: lowerer.is_real_expression_leaf(expression),
            },
        };
        result.insert(key(expression), shape);
    }
    result
}

pub(super) fn shape(lowerer: &ProcessLowerer<'_>, expression: &Expression) -> Shape {
    shapes(lowerer, expression)[&key(expression)]
}

#[derive(Clone, Copy)]
pub(super) enum Mode {
    Bits(Context),
    /// Propagate width and enclosing signedness before evaluating an operand,
    /// then extend self-determined leaves to that common context.
    Operand(Context),
    Real,
    Condition,
}

enum Operation {
    Resize(Context),
    ToReal(bool),
    Truth,
    RealTruth,
    Unary(UnaryOp, Context, bool),
    Binary(BinaryOp, Context, bool, bool, bool),
    Xnor(Context),
    ArithmeticShift(Context),
    CaseEquality(bool, bool),
    Reduction(crate::ast::ReductionOp),
    MathUnary(super::super::CfgUnaryOp),
    MathBinary(super::super::CfgBinaryOp),
    BitsToReal,
    RealToBits,
}

enum Work<'a> {
    Expression(BlockId, &'a Expression, Mode),
    Apply(BlockId, Operation),
    LogicalLeft(BlockId, LogicalOp, &'a Expression),
    LogicalRight(LogicalFrame),
    Conditional(BlockId, &'a crate::ast::ConditionalExpr, ConditionalDomain),
    Then(ConditionalFrame, &'a Expression),
    Else(ConditionalFrame, ValueId, Option<ValueId>),
}

fn push_unary<'a>(
    pending: &mut Vec<Work<'a>>,
    block: BlockId,
    operation: Operation,
    child: &'a Expression,
    mode: Mode,
) {
    pending.push(Work::Apply(block, operation));
    pending.push(Work::Expression(block, child, mode));
}

pub(super) fn lower(
    lowerer: &mut ProcessLowerer<'_>,
    block: BlockId,
    root: &Expression,
    mode: Mode,
) -> ValueId {
    let shapes = shapes(lowerer, root);
    let mut pending = vec![Work::Expression(block, root, mode)];
    let mut values = Vec::new();
    while let Some(work) = pending.pop() {
        let (block, expression, mode) = match work {
            Work::Expression(block, expression, mode) => (block, expression, mode),
            Work::Apply(block, operation) => {
                let value = apply(lowerer, block, operation, &mut values);
                values.push(value);
                continue;
            }
            Work::LogicalLeft(block, op, right) => {
                let left = values.pop().expect("logical left operand");
                let frame = LogicalFrame::start(lowerer, block, op, left);
                let right_block = frame.right_block;
                pending.push(Work::LogicalRight(frame));
                pending.push(Work::Expression(right_block, right, Mode::Condition));
                continue;
            }
            Work::LogicalRight(frame) => {
                let right = values.pop().expect("logical right operand");
                values.push(frame.finish(lowerer, right));
                continue;
            }
            Work::Conditional(block, conditional, domain) => {
                let condition = values.pop().expect("conditional condition");
                let frame = ConditionalFrame::start(lowerer, block, condition, domain);
                let then_block = frame.then_block;
                pending.push(Work::Then(frame, &conditional.else_expr));
                pending.push(Work::Expression(
                    then_block,
                    &conditional.then_expr,
                    domain_mode(domain),
                ));
                continue;
            }
            Work::Then(frame, else_expression) => {
                let then_value = values.pop().expect("conditional first arm");
                let then_at_else = frame.finish_then(lowerer, then_value);
                let block = frame.else_block;
                let mode = domain_mode(frame.domain);
                pending.push(Work::Else(frame, then_value, then_at_else));
                pending.push(Work::Expression(block, else_expression, mode));
                continue;
            }
            Work::Else(frame, then_value, then_at_else) => {
                let else_value = values.pop().expect("conditional second arm");
                values.push(frame.finish(lowerer, then_value, then_at_else, else_value));
                continue;
            }
        };
        let shape = shapes[&key(expression)];
        match mode {
            Mode::Operand(context) => {
                pending.push(Work::Apply(block, Operation::Resize(context)));
                pending.push(Work::Expression(block, expression, Mode::Bits(context)));
                continue;
            }
            Mode::Condition => {
                pending.push(Work::Apply(
                    block,
                    if shape.real {
                        Operation::RealTruth
                    } else {
                        Operation::Truth
                    },
                ));
                pending.push(Work::Expression(
                    block,
                    expression,
                    if shape.real {
                        Mode::Real
                    } else {
                        Mode::Bits(Context::SELF_DETERMINED)
                    },
                ));
                continue;
            }
            Mode::Real if !shape.real => {
                pending.push(Work::Apply(block, Operation::ToReal(shape.signed)));
                pending.push(Work::Expression(
                    block,
                    expression,
                    Mode::Bits(Context::SELF_DETERMINED),
                ));
                continue;
            }
            Mode::Bits(context) if shape.real => {
                lowerer.error(
                    "a real value has no four-state form here: Verilog-AMS LRM 2.4 section 3.7 \
                     converts one to bits with the explicit `$realtobits`, and this position needs \
                     bits",
                    expression.span(),
                );
                values.push(lowerer.unknown(context.width.max(1)));
                continue;
            }
            _ => {}
        }
        let real = matches!(mode, Mode::Real);
        let context = match mode {
            Mode::Bits(context) => Context {
                width: shape.width.max(context.width),
                signed: shape.signed && context.signed,
            },
            Mode::Real => Context::SELF_DETERMINED,
            _ => unreachable!(),
        };
        // Push right before left: stack execution preserves source order.
        match expression {
            Expression::Conditional(value) => {
                let domain = if real {
                    ConditionalDomain::Real
                } else {
                    ConditionalDomain::FourState(context)
                };
                pending.push(Work::Conditional(block, value, domain));
                pending.push(Work::Expression(block, &value.condition, Mode::Condition));
            }
            Expression::Call(call) if real && lowerer.constant_expression => {
                match (constants::math_call(&call.name), call.args.as_slice()) {
                    (Some(constants::MathCall::Unary(op)), [input]) => {
                        push_unary(
                            &mut pending,
                            block,
                            Operation::MathUnary(op),
                            input,
                            Mode::Real,
                        );
                    }
                    (Some(constants::MathCall::Binary(op)), [left, right]) => {
                        pending.push(Work::Apply(block, Operation::MathBinary(op)));
                        pending.push(Work::Expression(block, right, Mode::Real));
                        pending.push(Work::Expression(block, left, Mode::Real));
                    }
                    _ => {
                        lowerer.error(
                            format!("`{}` has no supported constant math signature", call.name),
                            call.span,
                        );
                        values.push(lowerer.real_constant(0.0));
                    }
                }
            }
            Expression::SystemFunction(function) if function.name == "$bitstoreal" && real => {
                if let Some(input) = function.args.first() {
                    push_unary(
                        &mut pending,
                        block,
                        Operation::BitsToReal,
                        input,
                        Mode::Operand(Context {
                            width: REAL_BIT_PATTERN_WIDTH,
                            signed: false,
                        }),
                    );
                } else {
                    values.push(lowerer.real_constant(0.0));
                }
            }
            Expression::SystemFunction(function) if function.name == "$realtobits" && !real => {
                if let Some(input) = function.args.first() {
                    push_unary(
                        &mut pending,
                        block,
                        Operation::RealToBits,
                        input,
                        Mode::Real,
                    );
                } else {
                    values.push(lowerer.unknown(REAL_BIT_PATTERN_WIDTH));
                }
            }
            Expression::Unary(value) => {
                if value.op == UnaryOp::ToInteger {
                    lowerer.invariant(
                        "analog integer conversion reached four-state lowering",
                        value.span,
                    );
                    values.push(lowerer.unknown(context.width.max(1)));
                    continue;
                }
                let child_shape = shapes[&key(&value.operand)];
                let child_mode = if real {
                    Mode::Real
                } else if value.op == UnaryOp::Not {
                    if child_shape.real {
                        Mode::Condition
                    } else {
                        Mode::Bits(Context::SELF_DETERMINED)
                    }
                } else {
                    Mode::Operand(context)
                };
                push_unary(
                    &mut pending,
                    block,
                    Operation::Unary(value.op, context, real),
                    &value.operand,
                    child_mode,
                );
            }
            Expression::Binary(value) if matches!(value.op, BinaryOp::And | BinaryOp::Or) => {
                // VAMS-2023 4.2.3: only a definite controlling left operand
                // skips the right side. An X/Z truth value still needs it.
                // Stateful analog operators are rejected by digital analysis;
                // continuous probes are reads and can be skipped.
                let op = if value.op == BinaryOp::And {
                    LogicalOp::And
                } else {
                    LogicalOp::Or
                };
                pending.push(Work::LogicalLeft(block, op, &value.right));
                pending.push(Work::Expression(block, &value.left, Mode::Condition));
            }
            Expression::Binary(value) => {
                if matches!(
                    value.op,
                    BinaryOp::CheckedValue
                        | BinaryOp::DiscreteValue
                        | BinaryOp::IntAdd
                        | BinaryOp::IntSub
                        | BinaryOp::IntMul
                        | BinaryOp::IntDiv
                        | BinaryOp::IntMod
                        | BinaryOp::IntPow
                ) {
                    lowerer.invariant(
                        "an internal analog integer operator cannot appear in a digital process",
                        value.span,
                    );
                    values.push(lowerer.unknown(context.width.max(1)));
                    continue;
                }
                let left = shapes[&key(&value.left)];
                let right = shapes[&key(&value.right)];
                let real_compare = real_compare_op(value.op).is_some() && (left.real || right.real);
                let (left_mode, right_mode, operation_context) = if real || real_compare {
                    (Mode::Real, Mode::Real, context)
                } else {
                    match value.op {
                        BinaryOp::And | BinaryOp::Or => {
                            unreachable!("logical control-flow lowering")
                        }
                        BinaryOp::Eq
                        | BinaryOp::Ne
                        | BinaryOp::Lt
                        | BinaryOp::Le
                        | BinaryOp::Gt
                        | BinaryOp::Ge => {
                            // The comparison supplies its own shared operand context.
                            let comparison = Context {
                                width: left.width.max(right.width),
                                signed: left.signed && right.signed,
                            };
                            (
                                Mode::Operand(comparison),
                                Mode::Operand(comparison),
                                comparison,
                            )
                        }
                        BinaryOp::Shl | BinaryOp::Shr | BinaryOp::Pow => (
                            Mode::Operand(context),
                            Mode::Bits(Context::SELF_DETERMINED),
                            context,
                        ),
                        _ => (Mode::Operand(context), Mode::Operand(context), context),
                    }
                };
                pending.push(Work::Apply(
                    block,
                    Operation::Binary(
                        value.op,
                        operation_context,
                        right.signed,
                        real,
                        real_compare,
                    ),
                ));
                pending.push(Work::Expression(block, &value.right, right_mode));
                pending.push(Work::Expression(block, &value.left, left_mode));
            }
            Expression::Digital(DigitalExpr::Xnor(value)) => {
                pending.push(Work::Apply(block, Operation::Xnor(context)));
                pending.push(Work::Expression(
                    block,
                    &value.right,
                    Mode::Operand(context),
                ));
                pending.push(Work::Expression(block, &value.left, Mode::Operand(context)));
            }
            Expression::Digital(DigitalExpr::ArithmeticShiftRight(value)) => {
                pending.push(Work::Apply(block, Operation::ArithmeticShift(context)));
                pending.push(Work::Expression(
                    block,
                    &value.right,
                    Mode::Bits(Context::SELF_DETERMINED),
                ));
                pending.push(Work::Expression(block, &value.left, Mode::Operand(context)));
            }
            Expression::Digital(DigitalExpr::CaseEquality(value)) => {
                let left = shapes[&key(&value.left)];
                let right = shapes[&key(&value.right)];
                let context = Context {
                    width: left.width.max(right.width),
                    signed: left.signed && right.signed,
                };
                pending.push(Work::Apply(
                    block,
                    Operation::CaseEquality(context.signed, value.negate),
                ));
                pending.push(Work::Expression(
                    block,
                    &value.right,
                    Mode::Operand(context),
                ));
                pending.push(Work::Expression(block, &value.left, Mode::Operand(context)));
            }
            Expression::Digital(DigitalExpr::Reduction(value)) => {
                push_unary(
                    &mut pending,
                    block,
                    Operation::Reduction(value.op),
                    &value.operand,
                    Mode::Bits(Context::SELF_DETERMINED),
                );
            }
            _ => values.push(if real {
                lowerer.real_expression_leaf(block, expression)
            } else {
                lowerer.sized_leaf(block, expression, context)
            }),
        }
    }
    debug_assert_eq!(values.len(), 1);
    values.pop().expect("expression result")
}

fn apply(
    lowerer: &mut ProcessLowerer<'_>,
    block: BlockId,
    operation: Operation,
    values: &mut Vec<ValueId>,
) -> ValueId {
    let right = values.pop().expect("operand result");
    let bits = |width| CfgValueType::FourState { width };
    let (ty, kind) = match operation {
        Operation::Resize(context) => {
            return lowerer.resize(block, right, context.width, context.signed);
        }
        Operation::ToReal(signed) => (
            CfgValueType::Real,
            CfgValueKind::DigitalIntegerToReal {
                input: right,
                signed,
            },
        ),
        Operation::Truth => return lowerer.truth_value(block, right),
        Operation::RealTruth => {
            let zero = lowerer.real_constant(0.0);
            (
                bits(1),
                CfgValueKind::DigitalRealCompare {
                    op: RealCompareOp::Ne,
                    left: right,
                    right: zero,
                },
            )
        }
        Operation::Unary(op, context, real) => match op {
            UnaryOp::Pos => return right,
            UnaryOp::Not => (bits(1), CfgValueKind::DigitalLogicalNot { input: right }),
            UnaryOp::BitNot => (
                bits(context.width),
                CfgValueKind::DigitalBitwiseNot { input: right },
            ),
            UnaryOp::Neg if real => (
                CfgValueType::Real,
                CfgValueKind::DigitalRealNegate { input: right },
            ),
            UnaryOp::Neg => {
                let zero = lowerer.builder.push_leaf(
                    bits(context.width),
                    CfgValueKind::FourStateConstant(FourStateValue::zero(context.width)),
                );
                (
                    bits(context.width),
                    CfgValueKind::DigitalArithmetic {
                        op: ArithmeticOp::Sub,
                        left: zero,
                        right,
                        signed: context.signed,
                    },
                )
            }
            UnaryOp::ToInteger => unreachable!("refused before operand lowering"),
        },
        Operation::Binary(op, context, right_signed, real, real_compare) => {
            let left = values.pop().expect("left operand");
            if real_compare {
                (
                    bits(1),
                    CfgValueKind::DigitalRealCompare {
                        op: real_compare_op(op).expect("comparison"),
                        left,
                        right,
                    },
                )
            } else if real {
                let op = match op {
                    BinaryOp::Add => RealArithmeticOp::Add,
                    BinaryOp::Sub => RealArithmeticOp::Sub,
                    BinaryOp::Mul => RealArithmeticOp::Mul,
                    BinaryOp::Div => RealArithmeticOp::Div,
                    BinaryOp::Mod => RealArithmeticOp::Mod,
                    BinaryOp::Pow => RealArithmeticOp::Pow,
                    _ => unreachable!("real arithmetic classified"),
                };
                (
                    CfgValueType::Real,
                    CfgValueKind::DigitalRealArithmetic { op, left, right },
                )
            } else {
                let kind = match op {
                    BinaryOp::Add
                    | BinaryOp::Sub
                    | BinaryOp::Mul
                    | BinaryOp::Div
                    | BinaryOp::Mod => CfgValueKind::DigitalArithmetic {
                        op: match op {
                            BinaryOp::Add => ArithmeticOp::Add,
                            BinaryOp::Sub => ArithmeticOp::Sub,
                            BinaryOp::Mul => ArithmeticOp::Mul,
                            BinaryOp::Div => ArithmeticOp::Div,
                            _ => ArithmeticOp::Mod,
                        },
                        left,
                        right,
                        signed: context.signed,
                    },
                    BinaryOp::BitAnd | BinaryOp::BitOr | BinaryOp::BitXor => {
                        CfgValueKind::DigitalBitwise {
                            op: match op {
                                BinaryOp::BitAnd => BitwiseOp::And,
                                BinaryOp::BitOr => BitwiseOp::Or,
                                _ => BitwiseOp::Xor,
                            },
                            left,
                            right,
                        }
                    }
                    BinaryOp::And | BinaryOp::Or => unreachable!("logical control-flow lowering"),
                    BinaryOp::Eq | BinaryOp::Ne => {
                        return lowerer.builder.push(
                            block,
                            bits(1),
                            CfgValueKind::DigitalEquality {
                                left,
                                right,
                                negate: op == BinaryOp::Ne,
                                signed: context.signed,
                            },
                        );
                    }
                    BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => {
                        return lowerer.builder.push(
                            block,
                            bits(1),
                            CfgValueKind::DigitalRelational {
                                op: match op {
                                    BinaryOp::Lt => RelationalOp::Lt,
                                    BinaryOp::Le => RelationalOp::Le,
                                    BinaryOp::Gt => RelationalOp::Gt,
                                    _ => RelationalOp::Ge,
                                },
                                left,
                                right,
                                signed: context.signed,
                            },
                        );
                    }
                    BinaryOp::Shl | BinaryOp::Shr => CfgValueKind::DigitalShift {
                        op: if op == BinaryOp::Shl {
                            ShiftOp::Left
                        } else {
                            ShiftOp::Right
                        },
                        value: left,
                        count: right,
                    },
                    BinaryOp::Pow => CfgValueKind::DigitalPower {
                        base: left,
                        exponent: right,
                        base_signed: context.signed,
                        exponent_signed: right_signed,
                    },
                    _ => unreachable!("internal analog operator refused"),
                };
                (bits(context.width), kind)
            }
        }
        Operation::Xnor(context) => {
            let left = values.pop().expect("left operand");
            (
                bits(context.width),
                CfgValueKind::DigitalBitwise {
                    op: BitwiseOp::Xnor,
                    left,
                    right,
                },
            )
        }
        Operation::ArithmeticShift(context) => {
            let value = values.pop().expect("shift input");
            (
                bits(context.width),
                CfgValueKind::DigitalShift {
                    op: if context.signed {
                        ShiftOp::ArithmeticRight
                    } else {
                        ShiftOp::Right
                    },
                    value,
                    count: right,
                },
            )
        }
        Operation::CaseEquality(signed, negate) => {
            let selector = values.pop().expect("case selector");
            let matched = lowerer.builder.push(
                block,
                bits(1),
                CfgValueKind::DigitalCaseMatch {
                    selector,
                    label: right,
                    kind: DigitalCaseMatch::Exact,
                    signed,
                },
            );
            if !negate {
                return matched;
            }
            (bits(1), CfgValueKind::DigitalLogicalNot { input: matched })
        }
        Operation::Reduction(op) => return lowerer.reduce_value(block, op, right),
        Operation::MathUnary(op) => (CfgValueType::Real, CfgValueKind::Unary { op, input: right }),
        Operation::MathBinary(op) => {
            let left = values.pop().expect("left math argument");
            (CfgValueType::Real, CfgValueKind::Binary { op, left, right })
        }
        Operation::BitsToReal => (
            CfgValueType::Real,
            CfgValueKind::DigitalBitsToReal { input: right },
        ),
        Operation::RealToBits => (
            bits(REAL_BIT_PATTERN_WIDTH),
            CfgValueKind::DigitalRealToBits { input: right },
        ),
    };
    lowerer.builder.push(block, ty, kind)
}

// Conditional continuations keep arm evaluation lazy without recursive lowering.
// ProcessBuilder follows continue_at redirects when an arm creates nested CFG.
struct ConditionalFrame {
    block: BlockId,
    condition: ValueId,
    is_false: ValueId,
    is_true: ValueId,
    then_block: BlockId,
    else_block: BlockId,
    join: BlockId,
    domain: ConditionalDomain,
}

fn domain_mode(domain: ConditionalDomain) -> Mode {
    match domain {
        ConditionalDomain::Real => Mode::Real,
        ConditionalDomain::FourState(context) => Mode::Operand(context),
    }
}

impl ConditionalFrame {
    fn start(
        lowerer: &mut ProcessLowerer<'_>,
        block: BlockId,
        condition: ValueId,
        domain: ConditionalDomain,
    ) -> Self {
        let bits = CfgValueType::FourState { width: 1 };
        let zero = lowerer.builder.push_leaf(
            bits,
            CfgValueKind::FourStateConstant(FourStateValue::from_u64(1, 0)),
        );
        let one = lowerer.builder.push_leaf(
            bits,
            CfgValueKind::FourStateConstant(FourStateValue::from_u64(1, 1)),
        );
        let is_false = lowerer.builder.push(
            block,
            bits,
            CfgValueKind::DigitalCaseMatch {
                selector: condition,
                label: zero,
                kind: DigitalCaseMatch::Exact,
                signed: false,
            },
        );
        let is_true = lowerer.builder.push(
            block,
            bits,
            CfgValueKind::DigitalCaseMatch {
                selector: condition,
                label: one,
                kind: DigitalCaseMatch::Exact,
                signed: false,
            },
        );
        let then_block = lowerer.builder.create_block();
        let else_block = lowerer.builder.create_block();
        let join = lowerer.builder.create_block();
        lowerer.builder.set_terminator(
            block,
            CfgTerminator::Branch {
                condition: is_false,
                then_target: else_block,
                then_args: Vec::new(),
                else_target: then_block,
                else_args: Vec::new(),
            },
        );
        lowerer.builder.seal_block(then_block);
        Self {
            block,
            condition,
            is_false,
            is_true,
            then_block,
            else_block,
            join,
            domain,
        }
    }

    fn finish_then(
        &self,
        lowerer: &mut ProcessLowerer<'_>,
        then_value: ValueId,
    ) -> Option<ValueId> {
        lowerer.builder.set_terminator(
            self.then_block,
            CfgTerminator::Branch {
                condition: self.is_true,
                then_target: self.join,
                then_args: Vec::new(),
                else_target: self.else_block,
                else_args: Vec::new(),
            },
        );
        // A false condition skips the first arm. Only an ambiguous condition
        // carries its actual value to the second arm for the bitwise merge.
        let then_at_else = match self.domain {
            ConditionalDomain::Real => None,
            ConditionalDomain::FourState(context) => {
                let placeholder = lowerer.unknown(context.width);
                Some(lowerer.builder.merge_values(
                    self.else_block,
                    &[(self.block, placeholder), (self.then_block, then_value)],
                ))
            }
        };
        lowerer.builder.seal_block(self.else_block);
        then_at_else
    }

    fn finish(
        self,
        lowerer: &mut ProcessLowerer<'_>,
        then_value: ValueId,
        then_at_else: Option<ValueId>,
        else_value: ValueId,
    ) -> ValueId {
        let (ty, kind) = match self.domain {
            ConditionalDomain::Real => {
                let zero = lowerer.real_constant(0.0);
                (
                    CfgValueType::Real,
                    CfgValueKind::DigitalRealSelect {
                        condition: self.is_false,
                        then_value: else_value,
                        else_value: zero,
                    },
                )
            }
            ConditionalDomain::FourState(context) => (
                CfgValueType::FourState {
                    width: context.width,
                },
                CfgValueKind::DigitalSelect {
                    condition: self.condition,
                    then_value: then_at_else.expect("four-state merge"),
                    else_value,
                },
            ),
        };
        let else_result = lowerer.builder.push(self.else_block, ty, kind);
        lowerer.builder.set_terminator(
            self.else_block,
            CfgTerminator::Jump {
                target: self.join,
                args: Vec::new(),
            },
        );
        let result = lowerer.builder.merge_values(
            self.join,
            &[
                (self.then_block, then_value),
                (self.else_block, else_result),
            ],
        );
        lowerer.builder.seal_block(self.join);
        lowerer.builder.continue_at(self.block, self.join);
        result
    }
}

// A logical RHS is evaluated only if the left truth value is not the known
// controlling value. Its own nested CFG and sample barriers may redirect
// right_block; ProcessBuilder resolves that continuation for the join.
struct LogicalFrame {
    block: BlockId,
    op: LogicalOp,
    left: ValueId,
    controlling: ValueId,
    right_block: BlockId,
    join: BlockId,
}

impl LogicalFrame {
    fn start(
        lowerer: &mut ProcessLowerer<'_>,
        block: BlockId,
        op: LogicalOp,
        left: ValueId,
    ) -> Self {
        let bits = CfgValueType::FourState { width: 1 };
        let controlling = lowerer.builder.push_leaf(
            bits,
            CfgValueKind::FourStateConstant(FourStateValue::from_u64(
                1,
                u64::from(op == LogicalOp::Or),
            )),
        );
        let skip = lowerer.builder.push(
            block,
            bits,
            CfgValueKind::DigitalCaseMatch {
                selector: left,
                label: controlling,
                kind: DigitalCaseMatch::Exact,
                signed: false,
            },
        );
        let right_block = lowerer.builder.create_block();
        let join = lowerer.builder.create_block();
        lowerer.builder.set_terminator(
            block,
            CfgTerminator::Branch {
                condition: skip,
                then_target: join,
                then_args: Vec::new(),
                else_target: right_block,
                else_args: Vec::new(),
            },
        );
        lowerer.builder.seal_block(right_block);
        Self {
            block,
            op,
            left,
            controlling,
            right_block,
            join,
        }
    }

    fn finish(self, lowerer: &mut ProcessLowerer<'_>, right: ValueId) -> ValueId {
        let evaluated = lowerer.builder.push(
            self.right_block,
            CfgValueType::FourState { width: 1 },
            CfgValueKind::DigitalLogical {
                op: self.op,
                left: self.left,
                right,
            },
        );
        lowerer.builder.set_terminator(
            self.right_block,
            CfgTerminator::Jump {
                target: self.join,
                args: Vec::new(),
            },
        );
        let result = lowerer.builder.merge_values(
            self.join,
            &[
                (self.block, self.controlling),
                (self.right_block, evaluated),
            ],
        );
        lowerer.builder.seal_block(self.join);
        lowerer.builder.continue_at(self.block, self.join);
        result
    }
}
