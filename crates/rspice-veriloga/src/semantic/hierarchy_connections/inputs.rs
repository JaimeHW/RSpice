//! Input expressions execute as ordinary assignments in their parent's scope.
use super::*;

pub(super) fn requires_assignment(
    actual: &Expression,
    parent: &AnalyzedModule,
    formal: &super::super::AnalyzedDigitalSignal,
) -> bool {
    let signal = |name: &SmolStr| {
        parent
            .digital
            .signals
            .iter()
            .find(|signal| &signal.name == name)
    };
    let coordinate =
        |expression: &Expression| match crate::canonical_ir::digital_lower::elaboration_constant(
            expression,
            &parent.digital.constants,
            parent.digital.time_scale,
        ) {
            Some(crate::numeric_literal::NumericLiteralValue::Integer(value)) => Some(value),
            _ => None,
        };
    // Keep direct bindings and already-supported constant selections intact.
    // General expressions need their parent's full value/array/time context.
    match actual {
        Expression::Identifier(identifier) => !signal(&identifier.name).is_some_and(|signal| {
            signal.unpacked.is_none()
                && signal.width == formal.width
                && signal.range == formal.range
                && signal.signedness == formal.signedness
                && signal.class.is_real() == formal.class.is_real()
                && !(formal.class.is_real() && signal.class.is_variable())
        }),
        Expression::ArrayAccess(access) => {
            !(formal.width == 1
                && signal(&access.array).is_some_and(|signal| signal.unpacked.is_none())
                && coordinate(&access.index).is_some())
        }
        Expression::Digital(crate::ast::DigitalExpr::PartSelect(select)) => {
            !(signal(&select.name).is_some_and(|signal| signal.unpacked.is_none())
                && coordinate(&select.msb)
                    .zip(coordinate(&select.lsb))
                    .is_some_and(|(msb, lsb)| {
                        msb.abs_diff(lsb).checked_add(1) == Some(u64::from(formal.width))
                    }))
        }
        _ => true,
    }
}

pub(super) fn insert(
    prepared: &mut Module,
    used: &mut HashSet<SmolStr>,
    child: &AnalyzedModule,
    formal: &Endpoint,
    site: (usize, usize),
    actual: &Expression,
) {
    let (instance, port) = site;
    let declared = child
        .digital
        .signals
        .iter()
        .find(|signal| signal.name == child.ports[port].name)
        .expect("a discrete input has a declaration");
    let span = actual.span();
    let name = temporary_net(
        prepared,
        used,
        format!("__rspice_input_{instance}_{port}").into(),
        formal,
        declared,
        span,
    );
    prepared.continuous_assigns.push(ContinuousAssign {
        target: DigitalLValue::Identifier {
            name: name.clone(),
            span,
        },
        value: actual.clone(),
        delay: None,
        span,
    });
    match &mut prepared.instances[instance].connections[port] {
        Connection::Named { signal, .. } | Connection::Ordered { signal, .. } => {
            *signal = Some(Expression::Identifier(Identifier { name, span }));
        }
    }
}

/// Parent-side storage has the formal's assignment width/type. Both expression
/// inputs and mixed packed ports use this declaration path.
pub(super) fn temporary_net(
    prepared: &mut Module,
    used: &mut HashSet<SmolStr>,
    mut name: SmolStr,
    formal: &Endpoint,
    declared: &super::super::AnalyzedDigitalSignal,
    span: Span,
) -> SmolStr {
    while !used.insert(name.clone()) {
        name = format!("{name}_").into();
    }
    let range = declared.range.map(|bounds| crate::ast::VectorRange {
        msb: super::super::exact_integer_expression(bounds.msb, span),
        lsb: super::super::exact_integer_expression(bounds.lsb, span),
        span,
    });
    prepared.nets.push(NetDecl {
        range: range.clone(),
        discipline: formal.segment.declared.clone(),
        names: vec![name.clone()],
        is_ground: false,
        is_internal: true,
        span,
    });
    prepared.digital_nets.push(DigitalNetDecl {
        kind: formal.net_kind.expect("a discrete formal"),
        signedness: declared.signedness,
        range,
        items: vec![DigitalDeclItem {
            name: name.clone(),
            dimensions: Vec::new(),
            init: None,
            span,
        }],
        span,
    });
    name
}
