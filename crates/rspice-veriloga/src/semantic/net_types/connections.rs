//! Collect type constraints from a structural connection without allocating lanes.
use super::*;
use crate::array_values::MAX_REPLICATION_NESTING;
use crate::semantic::MAX_DIGITAL_VECTOR_WIDTH;

pub(super) enum Segment {
    Net {
        node: usize,
        complete: bool,
        span: Span,
    },
    Real(DigitalNetKind),
}

pub(super) struct Group {
    pub width: u32,
    pub segments: Vec<Segment>,
}

pub(super) fn collect(
    occurrence: &Occurrence,
    nets: &[Net],
    expression: &Expression,
) -> Option<Group> {
    Collector { occurrence, nets }.expression(expression, 0)
}

struct Collector<'a> {
    occurrence: &'a Occurrence,
    nets: &'a [Net],
}

impl Collector<'_> {
    fn expression(&self, expression: &Expression, depth: usize) -> Option<Group> {
        if depth > MAX_REPLICATION_NESTING {
            return None;
        }
        if let Expression::ArrayLiteral(concat) = expression {
            if concat.assignment_pattern {
                return None;
            }
            return self.elements(&concat.elements, depth + 1);
        }
        if let Expression::Identifier(id) = expression {
            let &node = self.occurrence.nets.get(&id.name)?;
            return Some(Group {
                width: self.nets[node].width,
                segments: vec![Segment::Net {
                    node,
                    complete: true,
                    span: expression.span(),
                }],
            });
        }
        if let Some((kind, width)) = selected_real_type(&self.occurrence.module, expression) {
            return Some(Group {
                width,
                segments: vec![Segment::Real(kind)],
            });
        }
        let (name, width, complete) = selected_wire(&self.occurrence.module, expression)?;
        let &node = self.occurrence.nets.get(name)?;
        Some(Group {
            width,
            segments: vec![Segment::Net {
                node,
                complete,
                span: expression.span(),
            }],
        })
    }

    fn elements(&self, elements: &[ArrayLiteralElement], depth: usize) -> Option<Group> {
        if depth > MAX_REPLICATION_NESTING {
            return None;
        }
        let mut group = Group {
            width: 0,
            segments: Vec::new(),
        };
        for element in elements {
            let part = match element {
                ArrayLiteralElement::Value(value) => self.expression(value, depth)?,
                ArrayLiteralElement::Replication(replication) => {
                    if depth >= MAX_REPLICATION_NESTING {
                        return None;
                    }
                    let constants = super::super::instance_parameters::constants(
                        &self.occurrence.module.source,
                    );
                    let count = match crate::canonical_ir::digital_lower::elaboration_constant(
                        &replication.count,
                        &constants,
                        self.occurrence.module.source.time_scale,
                    ) {
                        Some(crate::numeric_literal::NumericLiteralValue::Integer(count)) => {
                            u32::try_from(count).ok()?
                        }
                        _ => return None,
                    };
                    if count == 0 || count > MAX_DIGITAL_VECTOR_WIDTH {
                        return None;
                    }
                    let mut part = self.elements(&replication.elements, depth + 1)?;
                    part.width = part.width.checked_mul(count)?;
                    // Repeating a connection repeats its lane count, not its
                    // type constraint or original driver contribution.
                    part
                }
            };
            group.width = group.width.checked_add(part.width)?;
            if group.width > MAX_DIGITAL_VECTOR_WIDTH {
                return None;
            }
            group.segments.extend(part.segments);
        }
        (group.width != 0).then_some(group)
    }
}
