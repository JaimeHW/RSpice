//! Check authored net operands before connection preparation creates assignments.
use super::*;
use crate::ast::{ArrayLiteralElement, DigitalExpr};

pub(super) fn check_actual(
    file: &AnalyzedFile,
    source: &Module,
    module: &AnalyzedModule,
    actual: &Expression,
    lower: &Endpoint,
    path: &str,
    port: &str,
) -> CompileResult<()> {
    let Some(lower_discipline) = lower.segment.declared.as_deref() else {
        return Ok(());
    };
    let mut expressions = vec![actual];
    let mut elements = Vec::new();
    loop {
        if let Some(expression) = expressions.pop() {
            let name = match expression {
                Expression::Identifier(id) => &id.name,
                Expression::ArrayAccess(access) => &access.array,
                Expression::Digital(DigitalExpr::PartSelect(select)) => &select.name,
                Expression::Digital(DigitalExpr::ArraySelect(select)) => &select.name,
                Expression::ArrayLiteral(concat) if !concat.assignment_pattern => {
                    elements.extend(concat.elements.iter().rev());
                    continue;
                }
                // Computed values are assignments, not connections to each net
                // read by the expression. Selectors also do not connect nets.
                _ => continue,
            };
            let Some(upper) = endpoint(source, module, name) else {
                continue;
            };
            if upper.net_kind.is_some() != lower.net_kind.is_some() {
                continue;
            }
            if let Some(upper_discipline) = upper.segment.declared.as_deref() {
                file.connect_rules
                    .check_net_compatibility(
                        &file.disciplines,
                        upper_discipline,
                        lower_discipline,
                        name,
                    )
                    .map_err(|cause| {
                        error(
                            format!("instance '{path}' port '{port}': {cause}"),
                            expression.span(),
                        )
                    })?;
            }
        } else if let Some(element) = elements.pop() {
            match element {
                ArrayLiteralElement::Value(value) => expressions.push(value),
                ArrayLiteralElement::Replication(repeated) => {
                    let count = crate::canonical_ir::digital_lower::elaboration_constant(
                        &repeated.count,
                        &module.digital.constants,
                        source.time_scale,
                    );
                    if !matches!(
                        count,
                        Some(crate::numeric_literal::NumericLiteralValue::Integer(0))
                    ) {
                        // One copy is sufficient for checking discipline compatibility.
                        elements.extend(repeated.elements.iter().rev());
                    }
                }
            }
        } else {
            break;
        }
    }
    Ok(())
}
