//! Arrays of gate instances (IEEE 1364-2005 sections 7.1.5 and 7.1.6).
//!
//! Expanded once the module is parsed, because a terminal is shared out by its
//! width: one bit reaches every instance, and a terminal as wide as the array
//! gives each instance one bit, the leftmost instance taking the leftmost bit.

use super::Parser;
use super::digital::GatePrimitive;
use crate::ast::*;
use crate::error::{ParseError, ParseErrorKind};
use crate::numeric_literal::NumericLiteralValue;
use crate::semantic::DigitalConstants;
use crate::source::Span;
use smol_str::SmolStr;
use std::collections::HashMap;

/// Replace each array of gate instances with one instance per element.
pub(super) fn expand(module: &mut Module, constants: &DigitalConstants) -> Result<(), ParseError> {
    let arrays = std::mem::take(&mut module.gate_arrays);
    if arrays.is_empty() {
        return Ok(());
    }
    let shapes = Shapes::of(module, constants);
    for array in arrays {
        let Some(gate) = GatePrimitive::from_name(&array.primitive) else {
            continue;
        };
        let context = format!("`{}` gate array", array.primitive);
        let left = shapes.constant(&array.left, &context)?;
        let right = shapes.constant(&array.right, &context)?;
        let Some(count) = u32::try_from(left.abs_diff(right) + 1)
            .ok()
            .filter(|count| *count <= 1 << 16)
        else {
            return Err(refusal(
                &context,
                "more instances than this compiler elaborates".to_string(),
                array.span,
            ));
        };
        for position in 0..count {
            let bit = count - 1 - position;
            let terminals = array
                .terminals
                .iter()
                .map(|terminal| shapes.lane(terminal, bit, count, &context))
                .collect::<Result<Vec<_>, _>>()?;
            let mut assignments =
                Parser::gate_assignments(gate, terminals, array.delay.clone(), array.span)?;
            for assignment in &mut assignments {
                assignment.strength = array.strength;
            }
            module.continuous_assigns.extend(assignments);
        }
    }
    Ok(())
}

fn refusal(context: &str, found: String, span: Span) -> ParseError {
    ParseError::new(
        ParseErrorKind::UnsupportedConstruct {
            context: context.to_string(),
            found,
        },
        span,
    )
}

/// The declared range of every vector the module names, `(msb, lsb)`.
struct Shapes<'a> {
    ranges: HashMap<SmolStr, (i64, i64)>,
    constants: &'a DigitalConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
}

impl<'a> Shapes<'a> {
    fn of(module: &Module, constants: &'a DigitalConstants) -> Self {
        let mut shapes = Self {
            ranges: HashMap::new(),
            constants,
            time_scale: module.time_scale,
        };
        let declare = |shapes: &mut Self, name: &SmolStr, range: Option<&VectorRange>| {
            let bounds = match range {
                Some(range) => {
                    let msb = shapes.integer(&range.msb);
                    let lsb = shapes.integer(&range.lsb);
                    let (Some(msb), Some(lsb)) = (msb, lsb) else {
                        return;
                    };
                    (msb, lsb)
                }
                None => (0, 0),
            };
            // A port declared twice (`output [3:0] y; wire [3:0] y;`) keeps
            // its ranged declaration.
            let entry = shapes.ranges.entry(name.clone()).or_insert(bounds);
            if range.is_some() {
                *entry = bounds;
            }
        };
        for net in &module.digital_nets {
            for item in net.items.iter().filter(|item| item.dimensions.is_empty()) {
                declare(&mut shapes, &item.name, net.range.as_ref());
            }
        }
        for variable in &module.digital_variables {
            for item in variable
                .items
                .iter()
                .filter(|item| item.dimensions.is_empty())
            {
                match variable.kind {
                    DigitalVariableKind::Reg => {
                        declare(&mut shapes, &item.name, variable.range.as_ref());
                    }
                    DigitalVariableKind::Integer => {
                        shapes.ranges.insert(item.name.clone(), (31, 0));
                    }
                    DigitalVariableKind::Real | DigitalVariableKind::Event => {}
                }
            }
        }
        for port in &module.port_declarations {
            for name in &port.names {
                declare(&mut shapes, name, port.range.as_ref());
            }
        }
        shapes
    }

    fn integer(&self, expression: &Expression) -> Option<i64> {
        match crate::canonical_ir::digital_lower::elaboration_constant(
            expression,
            self.constants,
            self.time_scale,
        ) {
            Some(NumericLiteralValue::Integer(value)) => Some(value),
            _ => None,
        }
    }

    fn constant(&self, expression: &Expression, context: &str) -> Result<i64, ParseError> {
        self.integer(expression).ok_or_else(|| {
            refusal(
                context,
                "an expression this compiler cannot fold to a constant; IEEE 1364-2005 \
                 section 7.1.5 bounds an array of instances by constant expressions"
                    .to_string(),
                expression.span(),
            )
        })
    }

    /// One instance's share of a terminal: the terminal itself when it is one
    /// bit wide, and its bit `bit` (counted from the least significant) when
    /// it is as wide as the array.
    fn lane(
        &self,
        terminal: &Expression,
        bit: u32,
        count: u32,
        context: &str,
    ) -> Result<Expression, ParseError> {
        let unsized_terminal = || {
            refusal(
                context,
                "a terminal this compiler cannot size; an array of gate instances shares out \
                 a net, a constant select of one, a literal or a concatenation of these \
                 (IEEE 1364-2005 section 7.1.6)"
                    .to_string(),
                terminal.span(),
            )
        };
        let width = self.width(terminal).ok_or_else(unsized_terminal)?;
        if width == 1 {
            return Ok(terminal.clone());
        }
        if width != u64::from(count) {
            return Err(refusal(
                context,
                format!(
                    "a {width}-bit terminal on an array of {count} instances; IEEE 1364-2005 \
                     section 7.1.6 requires one bit or one bit per instance"
                ),
                terminal.span(),
            ));
        }
        self.slice(terminal, u64::from(bit))
            .ok_or_else(unsized_terminal)
    }

    fn width(&self, expression: &Expression) -> Option<u64> {
        let span_of = |(msb, lsb): (i64, i64)| Some(msb.abs_diff(lsb) + 1);
        match expression {
            Expression::Identifier(identifier) => span_of(*self.ranges.get(&identifier.name)?),
            Expression::ArrayAccess(access) if access.packed.is_none() => {
                self.ranges.get(&access.array).map(|_| 1)
            }
            Expression::Digital(DigitalExpr::PartSelect(select)) => {
                self.ranges.get(&select.name)?;
                span_of((self.integer(&select.msb)?, self.integer(&select.lsb)?))
            }
            Expression::Digital(DigitalExpr::FourState(literal)) => {
                Some(u64::from(literal.value.width()))
            }
            // A based literal without `x` or `z` digits parses as a number.
            Expression::Number(number) if number.raw.contains('\'') => Some(u64::from(
                crate::four_state::decode(&number.raw).ok()?.width(),
            )),
            Expression::Number(number) if number.value.fract() == 0.0 => Some(32),
            Expression::ArrayLiteral(concat) if !concat.assignment_pattern => {
                self.elements_width(&concat.elements)
            }
            _ => None,
        }
    }

    fn elements_width(&self, elements: &[ArrayLiteralElement]) -> Option<u64> {
        elements.iter().try_fold(0u64, |total, element| {
            let width = match element {
                ArrayLiteralElement::Value(value) => self.width(value)?,
                ArrayLiteralElement::Replication(replication) => {
                    let count = u64::try_from(self.integer(&replication.count)?).ok()?;
                    count.checked_mul(self.elements_width(&replication.elements)?)?
                }
            };
            total.checked_add(width)
        })
    }

    /// Bit `bit` of `expression`, counted from its least significant end.
    fn slice(&self, expression: &Expression, bit: u64) -> Option<Expression> {
        let span = expression.span();
        let select = |name: &SmolStr, (msb, lsb): (i64, i64)| {
            let offset = i64::try_from(bit).ok()?;
            let index = if msb >= lsb {
                lsb.checked_add(offset)?
            } else {
                lsb.checked_sub(offset)?
            };
            Some(Expression::ArrayAccess(ArrayAccessExpr {
                normalized: false,
                packed: None,
                array: name.clone(),
                discrete_validity: None,
                index: Box::new(Expression::Number(NumberLit {
                    value: index as f64,
                    raw: index.to_string().into(),
                    span,
                })),
                span,
            }))
        };
        let literal_bit = |value: crate::four_state::FourStateBit| {
            let spelling = format!("1'b{}", value.as_char());
            let value = crate::four_state::decode(&spelling).ok()?;
            Some(Expression::Digital(DigitalExpr::FourState(FourStateLit {
                value,
                span,
            })))
        };
        match expression {
            Expression::Identifier(identifier) => {
                select(&identifier.name, *self.ranges.get(&identifier.name)?)
            }
            Expression::Digital(DigitalExpr::PartSelect(part)) => select(
                &part.name,
                (self.integer(&part.msb)?, self.integer(&part.lsb)?),
            ),
            Expression::Digital(DigitalExpr::FourState(literal)) => {
                let bits = &literal.value.bits;
                let position = bits.len().checked_sub(1 + usize::try_from(bit).ok()?)?;
                literal_bit(bits[position])
            }
            Expression::Number(number) if number.raw.contains('\'') => {
                let literal = crate::four_state::decode(&number.raw).ok()?;
                let position = literal
                    .bits
                    .len()
                    .checked_sub(1 + usize::try_from(bit).ok()?)?;
                literal_bit(literal.bits[position])
            }
            Expression::Number(number) => {
                let value = number.value as i64;
                let one = bit < 64 && (value >> bit) & 1 == 1;
                literal_bit(if one {
                    crate::four_state::FourStateBit::One
                } else {
                    crate::four_state::FourStateBit::Zero
                })
            }
            Expression::ArrayLiteral(concat) if !concat.assignment_pattern => {
                self.slice_elements(&concat.elements, bit)
            }
            _ if bit == 0 && self.width(expression) == Some(1) => Some(expression.clone()),
            _ => None,
        }
    }

    /// Bit `bit` of a concatenation: its last element holds the low bits.
    fn slice_elements(&self, elements: &[ArrayLiteralElement], mut bit: u64) -> Option<Expression> {
        for element in elements.iter().rev() {
            match element {
                ArrayLiteralElement::Value(value) => {
                    let width = self.width(value)?;
                    if bit < width {
                        return if width == 1 {
                            Some(value.clone())
                        } else {
                            self.slice(value, bit)
                        };
                    }
                    bit -= width;
                }
                ArrayLiteralElement::Replication(replication) => {
                    let inner = self.elements_width(&replication.elements)?;
                    let count = u64::try_from(self.integer(&replication.count)?).ok()?;
                    let width = inner.checked_mul(count)?;
                    if bit < width {
                        return self.slice_elements(&replication.elements, bit % inner);
                    }
                    bit -= width;
                }
            }
        }
        None
    }
}
