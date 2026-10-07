//! Iterative operator typing and lowering, shared by constants and processes.
//!
//! Shape is computed bottom-up once, then width/sign context travels top-down.
//! The borrowed AST remains fixed throughout each walk; pointer keys identify
//! its nodes only within that walk and are never dereferenced or persisted.

use super::*;
use crate::ast::{DigitalExpr, PackedSelect};

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
        Expression::ArrayAccess(value) => vec![&value.index],
        Expression::Digital(DigitalExpr::PartSelect(value)) => vec![&value.msb, &value.lsb],
        Expression::Digital(DigitalExpr::ArraySelect(value)) => value.children(),
        Expression::ArrayLiteral(value) => {
            let mut children = Vec::new();
            let mut pending: Vec<_> = value.elements.iter().rev().collect();
            while let Some(element) = pending.pop() {
                match element {
                    ArrayLiteralElement::Value(value) => children.push(value),
                    ArrayLiteralElement::Replication(value) => {
                        children.push(&value.count);
                        pending.extend(value.elements.iter().rev());
                    }
                }
            }
            children
        }
        Expression::Call(value) => value.args.iter().collect(),
        Expression::SystemFunction(value) => value.args.iter().collect(),
        _ => Vec::new(),
    }
}

fn key(expression: &Expression) -> *const Expression {
    expression
}

/// Prepared information belongs to one immutable AST walk. Constant selectors
/// use already-prepared descendants, so a replication count containing another
/// replication cannot recursively re-enter type inference.
#[derive(Default)]
pub(super) struct Shapes {
    values: HashMap<*const Expression, Shape>,
    constants: HashMap<*const Expression, Option<i64>>,
    elements: HashMap<*const ArrayLiteralElement, Option<u32>>,
}

impl std::ops::Index<&*const Expression> for Shapes {
    type Output = Shape;
    fn index(&self, key: &*const Expression) -> &Shape {
        &self.values[key]
    }
}

impl Shapes {
    pub(super) fn get(&self, expression: &Expression) -> Shape {
        self[&key(expression)]
    }

    fn prepare_constant(&mut self, lowerer: &ProcessLowerer<'_>, expression: &Expression) {
        if self.constants.contains_key(&key(expression)) {
            return;
        }
        let mut reads = BTreeSet::new();
        collect_expression_reads(expression, &mut reads);
        let value = if reads.iter().any(|name| {
            lowerer.lookup_local(name).is_some() || lowerer.index.contains_key(name.as_str())
        }) {
            None
        } else {
            constants::scalar_prepared(expression, lowerer.constants, lowerer.time_scale, self)
                .and_then(|value| match value {
                    crate::numeric_literal::NumericLiteralValue::Integer(value) => Some(value),
                    crate::numeric_literal::NumericLiteralValue::Real(value) => {
                        crate::semantic::SemanticAnalyzer::exact_const_i64(value)
                    }
                })
        };
        self.constants.insert(key(expression), value);
    }

    fn constant(&self, expression: &Expression) -> Option<i64> {
        self.constants[&key(expression)]
    }

    fn count(&self, expression: &Expression) -> Option<u32> {
        self.constant(expression)
            .and_then(|count| u32::try_from(count).ok())
    }

    fn part_width(&self, msb: &Expression, lsb: &Expression) -> u32 {
        self.constant(msb)
            .zip(self.constant(lsb))
            .and_then(|(msb, lsb)| ProcessLowerer::bounded_part_width(msb, lsb))
            .unwrap_or(1)
    }

    fn group_width(&self, elements: &[ArrayLiteralElement]) -> Option<u32> {
        elements.iter().try_fold(0u32, |sum, element| {
            sum.checked_add(self.elements[&(element as *const _)]?)
                .filter(|width| *width <= crate::semantic::MAX_DIGITAL_VECTOR_WIDTH)
        })
    }

    fn prepare_elements(&mut self, lowerer: &ProcessLowerer<'_>, elements: &[ArrayLiteralElement]) {
        let mut pending: Vec<_> = elements
            .iter()
            .rev()
            .map(|element| (element, false))
            .collect();
        while let Some((element, ready)) = pending.pop() {
            let width = match element {
                ArrayLiteralElement::Value(value) => Some(self.get(value).width),
                ArrayLiteralElement::Replication(value) => {
                    if !ready {
                        self.prepare_constant(lowerer, &value.count);
                        pending.push((element, true));
                        pending.extend(value.elements.iter().rev().map(|element| (element, false)));
                        continue;
                    }
                    self.group_width(&value.elements)
                        .filter(|width| *width > 0)
                        .zip(self.count(&value.count))
                        .and_then(|(width, count)| width.checked_mul(count))
                        .filter(|width| *width <= crate::semantic::MAX_DIGITAL_VECTOR_WIDTH)
                }
            };
            self.elements.insert(element, width);
        }
    }
}

fn shapes(lowerer: &ProcessLowerer<'_>, root: &Expression) -> Shapes {
    let mut result = Shapes::default();
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
        match expression {
            Expression::ArrayLiteral(value) => result.prepare_elements(lowerer, &value.elements),
            Expression::Digital(DigitalExpr::PartSelect(value)) => {
                result.prepare_constant(lowerer, &value.msb);
                result.prepare_constant(lowerer, &value.lsb);
            }
            Expression::Digital(DigitalExpr::ArraySelect(value)) => {
                if let PackedSelect::Part { msb, lsb } = &value.select {
                    result.prepare_constant(lowerer, msb);
                    result.prepare_constant(lowerer, lsb);
                }
            }
            _ => {}
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
            Expression::ArrayLiteral(value) => Shape {
                width: result
                    .group_width(&value.elements)
                    .filter(|width| {
                        *width > 0
                            || matches!(
                                value.elements.as_slice(),
                                [ArrayLiteralElement::Replication(_)]
                            )
                    })
                    .unwrap_or(1),
                signed: false,
                real: false,
            },
            Expression::Digital(DigitalExpr::PartSelect(value)) => Shape {
                width: result.part_width(&value.msb, &value.lsb),
                signed: false,
                real: false,
            },
            Expression::Digital(DigitalExpr::ArraySelect(value)) => Shape {
                width: match &value.select {
                    PackedSelect::Bit(_) => 1,
                    PackedSelect::Part { msb, lsb } => result.part_width(msb, lsb),
                },
                signed: false,
                real: false,
            },
            _ => Shape {
                width: lowerer.self_width_leaf(expression),
                signed: lowerer.self_signed_leaf(expression),
                real: lowerer.is_real_expression_leaf(expression),
            },
        };
        result.values.insert(key(expression), shape);
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
    Elements(BlockId, &'a [ArrayLiteralElement], u32),
    Element(BlockId, &'a ArrayLiteralElement),
    Concat(BlockId, usize, u32),
    ArrayRead(BlockId, &'a SmolStr, Span, bool),
    BitSelect(BlockId, ValueId, VectorBounds, bool),
    ArraySelect(BlockId, &'a crate::ast::ArraySelectExpr, VectorBounds),
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
    lower_prepared(lowerer, block, root, mode, &shapes)
}

pub(super) fn lower_prepared(
    lowerer: &mut ProcessLowerer<'_>,
    block: BlockId,
    root: &Expression,
    mode: Mode,
    shapes: &Shapes,
) -> ValueId {
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
            Work::Elements(block, elements, count) => {
                pending.push(Work::Concat(block, values.len(), count));
                pending.extend(
                    elements
                        .iter()
                        .rev()
                        .map(|element| Work::Element(block, element)),
                );
                continue;
            }
            Work::Element(block, element) => {
                match element {
                    ArrayLiteralElement::Value(value) => {
                        if shapes.get(value).width == 0 {
                            // The parser represents {0{...}} as an expression
                            // inside its surrounding concatenation. It has no
                            // result bits, but its operands still execute.
                            let Expression::ArrayLiteral(literal) = value else {
                                unreachable!()
                            };
                            let [ArrayLiteralElement::Replication(replication)] =
                                literal.elements.as_slice()
                            else {
                                unreachable!()
                            };
                            pending.push(Work::Elements(block, &replication.elements, 0));
                        } else {
                            pending.push(Work::Expression(
                                block,
                                value,
                                Mode::Bits(Context::SELF_DETERMINED),
                            ));
                        }
                    }
                    ArrayLiteralElement::Replication(value) => pending.push(Work::Elements(
                        block,
                        &value.elements,
                        shapes.count(&value.count).expect("validated count"),
                    )),
                }
                continue;
            }
            Work::Concat(block, start, count) => {
                let parts = values.split_off(start);
                // VAMS 4.2.13 evaluates the operands once, even for a zero
                // replication. Only the resulting bit pattern is repeated.
                if count != 0 {
                    let width = parts
                        .iter()
                        .map(|part| lowerer.value_width(*part))
                        .sum::<u32>()
                        * count;
                    let repeated = (0..count).flat_map(|_| parts.iter().copied()).collect();
                    values.push(lowerer.builder.push(
                        block,
                        CfgValueType::FourState { width },
                        CfgValueKind::DigitalConcat { parts: repeated },
                    ));
                }
                continue;
            }
            Work::ArrayRead(block, name, span, signed) => {
                let index = values.pop().expect("array index");
                values.push(if lowerer.digital_array(name).is_some() {
                    lowerer.digital_array_read_value(block, name, index, signed)
                } else {
                    lowerer.analog_array_read_value(block, name, span, index, signed)
                });
                continue;
            }
            Work::BitSelect(block, input, range, signed) => {
                let index = values.pop().expect("bit index");
                values.push(lowerer.builder.push(
                    block,
                    CfgValueType::FourState { width: 1 },
                    CfgValueKind::DigitalBitSelect {
                        input,
                        index,
                        signed,
                        bounds: (range.msb, range.lsb),
                    },
                ));
                continue;
            }
            Work::ArraySelect(block, access, range) => {
                let input = values.pop().expect("array word");
                match &access.select {
                    PackedSelect::Bit(bit) => {
                        pending.push(Work::BitSelect(block, input, range, shapes.get(bit).signed));
                        pending.push(Work::Expression(block, bit, value_mode(shapes.get(bit))));
                    }
                    PackedSelect::Part { msb, lsb } => values.push(part_read(
                        lowerer,
                        block,
                        input,
                        range,
                        msb,
                        lsb,
                        access.span,
                        shapes,
                    )),
                }
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
            Expression::ArrayLiteral(value) => {
                if shapes
                    .group_width(&value.elements)
                    .is_none_or(|width| width == 0)
                {
                    lowerer.error(
                        "a concatenation requires a positive bounded width and non-negative constant replication counts",
                        value.span);
                    values.push(lowerer.unknown(1));
                } else {
                    pending.push(Work::Elements(block, &value.elements, 1));
                }
            }
            Expression::ArrayAccess(access) => {
                if lowerer.digital_array(&access.array).is_some()
                    || lowerer.analog_array(&access.array).is_some()
                {
                    pending.push(Work::ArrayRead(
                        block,
                        &access.array,
                        access.span,
                        shapes.get(&access.index).signed,
                    ));
                } else if let Some(input) =
                    lowerer.packed_named_value(block, &access.array, access.span)
                {
                    pending.push(Work::BitSelect(
                        block,
                        input,
                        lowerer.declared_range_of(&access.array),
                        shapes.get(&access.index).signed,
                    ));
                } else {
                    values.push(lowerer.unknown(1));
                    continue;
                }
                pending.push(Work::Expression(
                    block,
                    &access.index,
                    value_mode(shapes.get(&access.index)),
                ));
            }
            Expression::Digital(DigitalExpr::PartSelect(select)) => {
                let value = if let Some(input) =
                    lowerer.packed_named_value(block, &select.name, select.span)
                {
                    part_read(
                        lowerer,
                        block,
                        input,
                        lowerer.declared_range_of(&select.name),
                        &select.msb,
                        &select.lsb,
                        select.span,
                        shapes,
                    )
                } else {
                    lowerer.unknown(1)
                };
                values.push(value);
            }
            Expression::Digital(DigitalExpr::ArraySelect(access)) => {
                let range = if let Some(array) = lowerer.digital_array(&access.name) {
                    if lowerer.real_signal(array.base) {
                        lowerer.error(
                            "packed selection requires integral array elements",
                            access.span,
                        );
                        values.push(lowerer.unknown(1));
                        continue;
                    }
                    lowerer.signals[usize::from(array.base)].declared_range()
                } else if lowerer
                    .analog_array(&access.name)
                    .is_some_and(|(quantity, _, _)| {
                        quantity == super::super::digital::DigitalAnalogQuantity::IntegerVariable
                    })
                {
                    INTEGER_BOUNDS
                } else {
                    lowerer.error("packed selection requires an unpacked array of four-state or integer elements", access.span);
                    values.push(lowerer.unknown(1));
                    continue;
                };
                pending.push(Work::ArraySelect(block, access, range));
                pending.push(Work::ArrayRead(
                    block,
                    &access.name,
                    access.span,
                    shapes.get(&access.index).signed,
                ));
                pending.push(Work::Expression(
                    block,
                    &access.index,
                    value_mode(shapes.get(&access.index)),
                ));
            }
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

fn value_mode(shape: Shape) -> Mode {
    if shape.real {
        Mode::Real
    } else {
        Mode::Bits(Context::SELF_DETERMINED)
    }
}

#[allow(clippy::too_many_arguments)]
fn part_read(
    lowerer: &mut ProcessLowerer<'_>,
    block: BlockId,
    input: ValueId,
    range: VectorBounds,
    msb: &Expression,
    lsb: &Expression,
    span: Span,
    shapes: &Shapes,
) -> ValueId {
    let Some((msb, lsb)) = shapes.constant(msb).zip(shapes.constant(lsb)) else {
        lowerer.error("a bit or part select must have constant bounds representable as signed 64-bit integers", span);
        return lowerer.unknown(1);
    };
    let Some(selected) = lowerer.validate_part_bounds(VectorBounds { msb, lsb }, range, span)
    else {
        return lowerer.unknown(1);
    };
    let width = selected.width();
    let (Some(msb), Some(lsb)) = (
        range.checked_position_of(selected.msb),
        range.checked_position_of(selected.lsb),
    ) else {
        return lowerer.unknown(width);
    };
    lowerer.builder.push(
        block,
        CfgValueType::FourState { width },
        CfgValueKind::DigitalPartSelect { input, msb, lsb },
    )
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
