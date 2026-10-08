//! Shape-checked unpacked values, expanded in declaration order before writes.
use crate::{array_index::UnpackedArrayLayout, ast::*, source::Span};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ElementType {
    Real,
    Integral { width: u32, signed: bool },
    String,
}

#[derive(Clone)]
pub(crate) struct ArrayType {
    pub layout: UnpackedArrayLayout,
    pub element: ElementType,
}

/// An array copy requires equivalent element types and equal extents on every
/// axis. Bounds and direction may differ. Patterns inherit their target type.
pub(crate) fn assignment_elements(
    expression: &Expression,
    target: &ArrayType,
    resolve: impl Fn(&str) -> Option<ArrayType>,
) -> Result<Vec<Expression>, String> {
    if let Expression::Identifier(id) = expression {
        let source = resolve(&id.name)
            .ok_or_else(|| format!("`{}` is not an unpacked array value", id.name))?;
        if source.element != target.element {
            return Err(
                "array assignment requires equivalent source and target element types".into(),
            );
        }
        if source.layout.axes().len() != target.layout.axes().len()
            || source
                .layout
                .axes()
                .iter()
                .zip(target.layout.axes())
                .any(|(a, b)| a.len() != b.len())
        {
            return Err(
                "array assignment requires equal extents on every unpacked dimension".into(),
            );
        }
        return Ok((0..source.layout.len())
            .map(|ordinal| element_read(&id.name, &source.layout, ordinal, id.span))
            .collect());
    }
    initializer_elements(expression, &target.layout)
        .map(|elements| elements.into_iter().cloned().collect())
}

pub(crate) fn coordinates(
    layout: &UnpackedArrayLayout,
    ordinal: usize,
    span: Span,
) -> Vec<Expression> {
    let slot = layout
        .declaration_slot(ordinal)
        .expect("validated array ordinal");
    layout
        .indices(slot)
        .expect("validated array slot")
        .into_iter()
        .map(|value| {
            Expression::Number(NumberLit {
                value: value as f64,
                raw: value.to_string().into(),
                span,
            })
        })
        .collect()
}

pub(crate) fn element_read(
    name: &str,
    layout: &UnpackedArrayLayout,
    ordinal: usize,
    span: Span,
) -> Expression {
    let mut indices = coordinates(layout, ordinal, span);
    if indices.len() == 1 {
        return Expression::ArrayAccess(ArrayAccessExpr {
            normalized: false,
            array: name.into(),
            index: Box::new(indices.remove(0)),
            packed: None,
            discrete_validity: None,
            span,
        });
    }
    let last = indices.pop().unwrap();
    Expression::Digital(DigitalExpr::ArraySelect(ArraySelectExpr {
        name: name.into(),
        index: Box::new(indices.remove(0)),
        additional_indices: indices,
        select: PackedSelect::Bit(Box::new(last)),
        span,
    }))
}

pub(crate) fn digital_target(
    name: &str,
    layout: &UnpackedArrayLayout,
    ordinal: usize,
    span: Span,
) -> DigitalLValue {
    match element_read(name, layout, ordinal, span) {
        Expression::ArrayAccess(access) => DigitalLValue::BitSelect {
            name: access.array,
            index: access.index,
            span,
        },
        Expression::Digital(DigitalExpr::ArraySelect(access)) => DigitalLValue::ArraySelect(access),
        _ => unreachable!("array element read"),
    }
}

/// Read nested patterns in authored dimension order. Leaf expressions may themselves
/// be packed concatenations; only the unpacked rank determines the pattern depth.
pub(crate) fn initializer_elements<'a>(
    expression: &'a Expression,
    layout: &crate::array_index::UnpackedArrayLayout,
) -> Result<Vec<&'a Expression>, String> {
    let mut elements = Vec::with_capacity(layout.len());
    let mut pending = vec![(expression, 0)];
    while let Some((expression, depth)) = pending.pop() {
        if depth == layout.axes().len() {
            elements.push(expression);
            continue;
        }
        let Expression::ArrayLiteral(literal) = expression else {
            return Err(format!(
                "array initializer dimension {} requires an array literal",
                depth + 1
            ));
        };
        if !literal.assignment_pattern {
            return Err(
                "unpacked array values require an assignment pattern opened with `'{`".into(),
            );
        }
        let expected = layout.axes()[depth].len();
        if literal.first_replication().is_some() {
            return Err(
                "replicated array initialization requires element-pattern expansion".into(),
            );
        }
        if literal.elements.len() != expected {
            return Err(format!(
                "array initializer dimension {} requires {expected} elements, found {}",
                depth + 1,
                literal.elements.len()
            ));
        }
        for element in literal.elements.iter().rev() {
            let ArrayLiteralElement::Value(value) = element else {
                unreachable!("replication rejected")
            };
            pending.push((value, depth + 1));
        }
    }
    Ok(elements)
}
