//! Keep computed operands intact while locating physical bits of a mixed input.
use super::*;
use crate::four_state::{FourStateBit, FourStateLiteral, LiteralBase};

impl ConnectionScope {
    /// The value releases physical positions with Z; converters drive those bits.
    /// None positions belong to the expression's single ordinary digital driver.
    pub(crate) fn mixed_input(
        &self,
        actual: &Expression,
        types: &mut crate::canonical_ir::digital_lower::ConnectionShapes<'_>,
    ) -> CompileResult<(Expression, Vec<Option<Expression>>)> {
        self.input_value(actual, types, 0)
    }

    fn input_value(
        &self,
        actual: &Expression,
        types: &mut crate::canonical_ir::digital_lower::ConnectionShapes<'_>,
        depth: usize,
    ) -> CompileResult<(Expression, Vec<Option<Expression>>)> {
        if depth > MAX_REPLICATION_NESTING {
            return Err(error(
                "port concatenation nesting exceeds the supported limit",
                actual.span(),
            ));
        }
        if !self.contains_physical(actual) {
            let width = types.four_state_width(actual).ok_or_else(|| error(
                "a mixed bus connection requires scalar physical or four-state digital lanes; real values cannot be concatenated",
                actual.span(),
            ))?;
            if width > MAX_DIGITAL_VECTOR_WIDTH {
                return Err(error(
                    "physical port connection exceeds the lane limit",
                    actual.span(),
                ));
            }
            return Ok((actual.clone(), vec![None; width as usize]));
        }
        if let Expression::ArrayLiteral(concat) = actual {
            if concat.assignment_pattern {
                return Err(error(
                    "a physical port connection requires a concatenation, not an assignment pattern",
                    actual.span(),
                ));
            }
            let (elements, lanes) =
                self.input_elements(&concat.elements, types, depth + 1, concat.span)?;
            return Ok((
                Expression::ArrayLiteral(crate::ast::ArrayLiteralExpr {
                    elements,
                    assignment_pattern: false,
                    span: concat.span,
                }),
                lanes,
            ));
        }
        let lanes = self
            .physical_selection(actual)?
            .expect("a physical input leaf");
        let width = lanes.len() as u32;
        let value = Expression::Digital(DigitalExpr::FourState(crate::ast::FourStateLit {
            value: FourStateLiteral {
                raw: format!("{width}'bz").into(),
                declared_width: Some(width),
                base: LiteralBase::Binary,
                signed: false,
                bits: vec![FourStateBit::HighImpedance; lanes.len()],
            },
            span: actual.span(),
        }));
        Ok((value, lanes.into_iter().map(Some).collect()))
    }

    fn input_elements(
        &self,
        elements: &[ArrayLiteralElement],
        types: &mut crate::canonical_ir::digital_lower::ConnectionShapes<'_>,
        depth: usize,
        span: Span,
    ) -> CompileResult<(Vec<ArrayLiteralElement>, Vec<Option<Expression>>)> {
        if depth > MAX_REPLICATION_NESTING {
            return Err(error(
                "port concatenation nesting exceeds the supported limit",
                span,
            ));
        }
        let mut values = Vec::new();
        let mut lanes = Vec::new();
        for element in elements {
            let (value, added) = match element {
                ArrayLiteralElement::Value(value) => {
                    let (value, lanes) = self.input_value(value, types, depth)?;
                    (ArrayLiteralElement::Value(value), lanes)
                }
                ArrayLiteralElement::Replication(replication) => {
                    let count = self.coordinate(&replication.count)?;
                    if count < 0 || count > i64::from(MAX_DIGITAL_VECTOR_WIDTH) {
                        return Err(error(
                            "port concatenation repetition must be nonnegative and bounded",
                            replication.span,
                        ));
                    }
                    let (values, repeated) = self.input_elements(
                        &replication.elements,
                        types,
                        depth + 1,
                        replication.span,
                    )?;
                    let count = count as usize;
                    let length = repeated.len().checked_mul(count);
                    if length.is_none_or(|length| length > MAX_DIGITAL_VECTOR_WIDTH as usize) {
                        return Err(error(
                            "physical port concatenation exceeds the lane limit",
                            replication.span,
                        ));
                    }
                    let mut lanes = Vec::with_capacity(length.unwrap());
                    for _ in 0..count {
                        lanes.extend(repeated.iter().cloned());
                    }
                    // Retain replication in the value tree: its computed operands
                    // are evaluated once, even though physical sites are per bit.
                    (
                        ArrayLiteralElement::Replication(crate::ast::ReplicationExpr {
                            count: replication.count.clone(),
                            elements: values,
                            span: replication.span,
                        }),
                        lanes,
                    )
                }
            };
            if lanes
                .len()
                .checked_add(added.len())
                .is_none_or(|length| length > MAX_DIGITAL_VECTOR_WIDTH as usize)
            {
                return Err(error(
                    "physical port concatenation exceeds the lane limit",
                    element.span(),
                ));
            }
            values.push(value);
            lanes.extend(added);
        }
        Ok((values, lanes))
    }
}
