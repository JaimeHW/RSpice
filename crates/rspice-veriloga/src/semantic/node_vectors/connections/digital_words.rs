//! Expand packed values without confusing array coordinates with packed bits.
use super::*;
use crate::ast::{ArraySelectExpr, PackedSelect};

impl ConnectionScope {
    pub(super) fn append_digital(
        &self,
        actual: &Expression,
        output: &mut Vec<Expression>,
    ) -> CompileResult<bool> {
        let name = match actual {
            Expression::Identifier(id) => &id.name,
            Expression::ArrayAccess(access) => &access.array,
            Expression::Digital(DigitalExpr::PartSelect(select)) => &select.name,
            Expression::Digital(DigitalExpr::ArraySelect(select)) => &select.name,
            _ => return Ok(false),
        };
        if self.append_real_bus(actual, output)? {
            return Ok(true);
        }
        let Some(shape) = self.digital.get(name) else {
            return Ok(false);
        };
        let span = actual.span();
        let rank = shape.dimensions.len();
        let missing_coordinates = || {
            error(
                format!(
                    "mixed connection '{name}' requires {rank} unpacked coordinates before a packed selection",
                ),
                span,
            )
        };
        let (indices, packed) = match actual {
            Expression::Identifier(_) => (Vec::new(), None),
            Expression::ArrayAccess(access) if rank == 0 => (
                Vec::new(),
                Some((access.index.as_ref(), access.index.as_ref())),
            ),
            Expression::ArrayAccess(access) => (vec![access.index.as_ref()], None),
            Expression::Digital(DigitalExpr::PartSelect(select)) => {
                (Vec::new(), Some((select.msb.as_ref(), select.lsb.as_ref())))
            }
            Expression::Digital(DigitalExpr::ArraySelect(select)) => {
                let (indices, packed) = select.split(rank).ok_or_else(missing_coordinates)?;
                let packed = packed.map(|select| match select {
                    PackedSelect::Bit(bit) => (bit.as_ref(), bit.as_ref()),
                    PackedSelect::Part { msb, lsb } => (msb.as_ref(), lsb.as_ref()),
                });
                (indices, packed)
            }
            _ => unreachable!("recognized discrete connection syntax"),
        };
        if indices.len() != rank {
            return Err(missing_coordinates());
        }
        // Array coordinates select values, not physical nodes. Out-of-range
        // reads retain the digital evaluator's X/non-finite semantics.
        let indices = indices
            .into_iter()
            .map(|index| self.coordinate(index))
            .collect::<CompileResult<Vec<_>>>()?;
        if shape.real {
            if packed.is_some() {
                return Err(error(
                    format!("real-valued connection '{name}' has no packed bits"),
                    span,
                ));
            }
            output.push(actual.clone());
            return Ok(true);
        }
        let bounds = shape.range.unwrap_or(VectorBounds {
            msb: i64::from(shape.width) - 1,
            lsb: 0,
        });
        let selected = match packed {
            Some((msb, lsb)) => {
                let selected = VectorBounds {
                    msb: self.coordinate(msb)?,
                    lsb: self.coordinate(lsb)?,
                };
                if selected.msb != selected.lsb
                    && (selected.msb < selected.lsb) != (bounds.msb < bounds.lsb)
                {
                    return Err(error(
                        "packed part-select direction disagrees with its declaration",
                        span,
                    ));
                }
                selected
            }
            None if shape.width == 1 => {
                output.push(actual.clone());
                return Ok(true);
            }
            None => bounds,
        };
        if selected.width() > MAX_DIGITAL_VECTOR_WIDTH {
            return Err(error(
                "physical port connection exceeds the lane limit",
                span,
            ));
        }
        for bit in selected.indices_msb_first() {
            output.push(if indices.is_empty() {
                self.selected(name, bit, span)?
            } else {
                Expression::Digital(DigitalExpr::ArraySelect(ArraySelectExpr {
                    name: name.clone(),
                    index: Box::new(exact_integer_expression(indices[0], span)),
                    additional_indices: indices[1..]
                        .iter()
                        .map(|&index| exact_integer_expression(index, span))
                        .collect(),
                    select: PackedSelect::Bit(Box::new(exact_integer_expression(bit, span))),
                    span,
                }))
            });
        }
        Ok(true)
    }
}
