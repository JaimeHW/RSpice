//! Identify a concrete discrete lane without confusing array axes with bits.
use super::*;
use crate::ast::{
    ArrayAccessExpr, ArraySelectExpr, DigitalExpr, NumberLit, PackedSelect, PartSelectExpr,
};
use crate::numeric_literal::NumericLiteralValue;

fn index(
    value: &Expression,
    constants: &super::super::DigitalConstants,
    source: &Module,
) -> CompileResult<i64> {
    match crate::canonical_ir::digital_lower::elaboration_constant(
        value,
        constants,
        source.time_scale,
    ) {
        Some(NumericLiteralValue::Integer(value)) => Ok(value),
        _ => Err(error(
            "a mixed-domain connection selector must resolve to an integer at elaboration",
            value.span(),
        )),
    }
}

fn literal(value: i64, span: Span) -> Expression {
    Expression::Number(NumberLit {
        value: value as f64,
        raw: value.to_string().into(),
        span,
    })
}

pub(super) fn endpoint(
    source: &Module,
    module: &AnalyzedModule,
    actual: &Expression,
    constants: &super::super::DigitalConstants,
) -> CompileResult<Option<(Endpoint, Expression)>> {
    let (name, indices, part) = match actual {
        Expression::Identifier(identifier) => {
            return Ok(super::endpoint(source, module, &identifier.name)
                .map(|endpoint| (endpoint, actual.clone())));
        }
        Expression::Number(number) if number.value == 0.0 => {
            return Ok(
                super::endpoint(source, module, "0").map(|endpoint| (endpoint, actual.clone()))
            );
        }
        Expression::ArrayAccess(access) => {
            (access.array.as_str(), vec![access.index.as_ref()], None)
        }
        Expression::Digital(DigitalExpr::PartSelect(select)) => (
            select.name.as_str(),
            vec![],
            Some((select.msb.as_ref(), select.lsb.as_ref())),
        ),
        Expression::Digital(DigitalExpr::ArraySelect(select)) => {
            let mut indices = vec![select.index.as_ref()];
            indices.extend(&select.additional_indices);
            let part = match &select.select {
                PackedSelect::Bit(bit) => {
                    indices.push(bit.as_ref());
                    None
                }
                PackedSelect::Part { msb, lsb } => Some((msb.as_ref(), lsb.as_ref())),
            };
            (select.name.as_str(), indices, part)
        }
        _ => return Ok(None),
    };
    let Some(signal) = module
        .digital
        .signals
        .iter()
        .find(|signal| signal.name == name)
    else {
        return Ok(None);
    };
    let mut endpoint = super::endpoint(source, module, name).expect("declared digital signal");
    let rank = signal.dimensions.len();
    let packed_bit = indices.len() == rank + 1 && part.is_none();
    if indices.len() != rank && !packed_bit {
        return Err(error(
            format!(
                "mixed connection '{name}' requires {rank} unpacked coordinates before a packed selection"
            ),
            actual.span(),
        ));
    }
    let mut indices = indices
        .into_iter()
        .map(|value| index(value, constants, source))
        .collect::<CompileResult<Vec<_>>>()?;
    let packed = if packed_bit {
        let bit = indices.pop().expect("one packed coordinate");
        Some((bit, bit))
    } else {
        match part {
            Some((msb, lsb)) => Some((
                index(msb, constants, source)?,
                index(lsb, constants, source)?,
            )),
            None => None,
        }
    };
    if packed.is_some() && signal.class.is_real() {
        return Err(error(
            format!("real-valued connection '{name}' has no packed bits"),
            actual.span(),
        ));
    }
    endpoint.width = match packed {
        Some((msb, lsb)) => u32::try_from(msb.abs_diff(lsb).saturating_add(1)).map_err(|_| {
            error(
                "mixed connection selection width exceeds the supported range",
                actual.span(),
            )
        })?,
        None => signal.width,
    };
    endpoint.unpacked = false;
    // Selecting the only packed bit is the same physical lane as naming the
    // whole one-bit signal/array element. Group those sites together. Keep a
    // structured identity so escaped scalar names cannot alias array syntax.
    let identity_bits = packed.filter(|&(msb, lsb)| {
        !(signal.width == 1
            && msb == lsb
            && signal
                .range
                .unwrap_or(super::super::VectorBounds::SCALAR)
                .lsb
                == lsb)
    });
    endpoint.identity.elements = indices.clone();
    endpoint.identity.bits = identity_bits;
    let mut identity = name.to_owned();
    for value in &indices {
        identity.push_str(&format!("[{value}]"));
    }
    if let Some((msb, lsb)) = identity_bits {
        if msb == lsb {
            identity.push_str(&format!("[{msb}]"));
        } else {
            identity.push_str(&format!("[{msb}:{lsb}]"));
        }
    }
    endpoint.segment.name = identity.into();
    let span = actual.span();
    let packed = packed.map(|(msb, lsb)| {
        if msb == lsb {
            PackedSelect::Bit(Box::new(literal(msb, span)))
        } else {
            PackedSelect::Part {
                msb: Box::new(literal(msb, span)),
                lsb: Box::new(literal(lsb, span)),
            }
        }
    });
    let expression = if indices.is_empty() {
        match packed.expect("a selection of a non-array signal") {
            PackedSelect::Bit(index) => Expression::ArrayAccess(ArrayAccessExpr {
                normalized: false,
                packed: None,
                discrete_validity: None,
                array: name.into(),
                index,
                span,
            }),
            PackedSelect::Part { msb, lsb } => {
                Expression::Digital(DigitalExpr::PartSelect(PartSelectExpr {
                    name: name.into(),
                    msb,
                    lsb,
                    span,
                }))
            }
        }
    } else if indices.len() == 1 && packed.is_none() {
        Expression::ArrayAccess(ArrayAccessExpr {
            normalized: false,
            packed: None,
            discrete_validity: None,
            array: name.into(),
            index: Box::new(literal(indices[0], span)),
            span,
        })
    } else {
        let select = packed.unwrap_or_else(|| {
            PackedSelect::Bit(Box::new(literal(
                indices.pop().expect("last array axis"),
                span,
            )))
        });
        Expression::Digital(DigitalExpr::ArraySelect(ArraySelectExpr {
            name: name.into(),
            index: Box::new(literal(indices[0], span)),
            additional_indices: indices[1..]
                .iter()
                .map(|&value| literal(value, span))
                .collect(),
            select,
            span,
        }))
    };
    Ok(Some((endpoint, expression)))
}

/// An implicit port assignment uses the connecting module's declared coordinate
/// system. Normal semantic lowering handles its type, range and driver identity.
pub(super) fn lvalue(actual: &Expression) -> crate::ast::DigitalLValue {
    use crate::ast::DigitalLValue;
    match actual {
        Expression::ArrayAccess(access) => DigitalLValue::BitSelect {
            name: access.array.clone(),
            index: access.index.clone(),
            span: access.span,
        },
        Expression::Digital(DigitalExpr::PartSelect(select)) => DigitalLValue::PartSelect {
            name: select.name.clone(),
            msb: select.msb.clone(),
            lsb: select.lsb.clone(),
            span: select.span,
        },
        Expression::Digital(DigitalExpr::ArraySelect(select)) => {
            DigitalLValue::ArraySelect(select.clone())
        }
        _ => unreachable!("selected mixed connection"),
    }
}
