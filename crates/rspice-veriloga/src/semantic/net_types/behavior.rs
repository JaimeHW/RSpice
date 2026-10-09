//! Find uses that require a discrete value rather than structural connectivity.
use super::*;
use std::collections::{BTreeSet, HashSet};

fn reads(expression: &Expression, locals: &BTreeSet<SmolStr>, used: &mut HashSet<SmolStr>) {
    super::super::flow_probes::visit_expression(expression, &mut |expression| {
        let name = match expression {
            Expression::Identifier(id) => &id.name,
            Expression::ArrayAccess(access) => &access.array,
            Expression::Digital(DigitalExpr::PartSelect(select)) => &select.name,
            Expression::Digital(DigitalExpr::ArraySelect(select)) => &select.name,
            _ => return,
        };
        if !locals.contains(name) {
            used.insert(name.clone());
        }
    });
}

pub(super) fn used(module: &SpecializedModule) -> HashSet<SmolStr> {
    let mut used = HashSet::new();
    let empty = BTreeSet::new();
    for process in &module.analyzed.digital.processes {
        super::super::digital_walk::collect_module_writes(&process.body, &mut used);
        super::super::digital_walk::visit_scoped_roots(&process.body, &mut |root, locals| {
            reads(root, locals, &mut used);
        });
    }
    for assignment in &module.analyzed.digital.continuous_assigns {
        let assignment = &assignment.assignment;
        reads(&assignment.value, &empty, &mut used);
        if let Some(delay) = &assignment.delay {
            reads(delay, &empty, &mut used);
        }
        let mut targets = vec![&assignment.target];
        while let Some(target) = targets.pop() {
            match target {
                DigitalLValue::Identifier { name, .. } => {
                    used.insert(name.clone());
                }
                DigitalLValue::BitSelect { name, index, .. } => {
                    used.insert(name.clone());
                    reads(index, &empty, &mut used);
                }
                DigitalLValue::PartSelect { name, msb, lsb, .. } => {
                    used.insert(name.clone());
                    reads(msb, &empty, &mut used);
                    reads(lsb, &empty, &mut used);
                }
                DigitalLValue::ArraySelect(select) => {
                    used.insert(select.name.clone());
                    for value in select.children() {
                        reads(value, &empty, &mut used);
                    }
                }
                DigitalLValue::Concat { elements, .. } => targets.extend(elements),
            }
        }
    }
    for signal in &module.analyzed.digital.signals {
        if let Some(value) = &signal.initializer {
            reads(value, &empty, &mut used);
        }
    }
    for declaration in &module.source.digital_nets {
        for item in &declaration.items {
            if let Some(value) = &item.init {
                used.insert(item.name.clone());
                reads(value, &empty, &mut used);
            }
        }
    }
    // The analyzer already resolves lexical scopes for numeric analog reads.
    // Reuse those bindings, including packed projections and unpacked arrays.
    let analog_values: HashSet<_> = module
        .analyzed
        .variables
        .iter()
        .filter(|variable| variable.is_event_controlled)
        .map(|variable| &variable.name)
        .collect();
    for signal in &module.analyzed.digital.signals {
        if analog_values.contains(&signal.name)
            || module
                .analyzed
                .arrays
                .get(&signal.name)
                .is_some_and(|array| {
                    module
                        .analyzed
                        .variables
                        .get(array.base)
                        .is_some_and(|variable| variable.is_event_controlled)
                })
        {
            used.insert(signal.name.clone());
        }
    }
    used.extend(
        module
            .analyzed
            .discrete_selections
            .iter()
            .map(|selection| selection.signal.clone()),
    );
    for instance in &module.source.instances {
        for connection in &instance.connections {
            let actual = match connection {
                Connection::Named { signal, .. } | Connection::Ordered { signal, .. } => {
                    signal.as_ref()
                }
            };
            if let Some(actual) = actual {
                connection_reads(module, actual, &mut used);
            }
        }
    }
    used
}

fn connection_reads(module: &SpecializedModule, actual: &Expression, used: &mut HashSet<SmolStr>) {
    let empty = BTreeSet::new();
    let constant = |expression: &Expression| {
        crate::canonical_ir::digital_lower::elaboration_constant(
            expression,
            &module.analyzed.digital.constants,
            module.source.time_scale,
        )
    };
    let integer = |expression: &Expression| {
        matches!(
            constant(expression),
            Some(crate::numeric_literal::NumericLiteralValue::Integer(_))
        )
    };
    let mut expressions = vec![actual];
    let mut elements = Vec::new();
    loop {
        if let Some(expression) = expressions.pop() {
            match expression {
                Expression::Identifier(_) => {}
                Expression::ArrayAccess(access) if integer(&access.index) => {}
                Expression::Digital(DigitalExpr::PartSelect(select))
                    if integer(&select.msb) && integer(&select.lsb) => {}
                Expression::Digital(DigitalExpr::ArraySelect(select))
                    if select.children().into_iter().all(integer) => {}
                Expression::ArrayLiteral(concat) if !concat.assignment_pattern => {
                    elements.extend(concat.elements.iter())
                }
                _ => reads(expression, &empty, used),
            }
        } else if let Some(element) = elements.pop() {
            match element {
                ArrayLiteralElement::Value(value) => expressions.push(value),
                ArrayLiteralElement::Replication(repeated) => {
                    if !matches!(
                        constant(&repeated.count),
                        Some(crate::numeric_literal::NumericLiteralValue::Integer(0))
                    ) {
                        reads(&repeated.count, &empty, used);
                        elements.extend(&repeated.elements);
                    }
                }
            }
        } else {
            break;
        }
    }
}
