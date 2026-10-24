//! Module-scope rules of IEEE 1364-2005 the parser checks while the module is
//! as written: a named block and a generate block do not share a name
//! (section 12.7), and an attribute's value is a constant expression
//! (section 3.8).

use crate::ast::*;
use crate::error::{ParseError, ParseErrorKind};
use smol_str::SmolStr;
use std::collections::HashSet;

/// Section 12.7: "An identifier shall be used to declare only one item within
/// a scope", and "for generate blocks, this rule applies regardless of whether
/// the generate block is instantiated". A named block of a process that no
/// other named block encloses is declared in the module's scope, where a
/// generate block of the same name already is.
pub(super) fn reject_block_name_collisions(module: &Module) -> Result<(), ParseError> {
    let mut generated = HashSet::new();
    for construct in &module.generates {
        generate_block_names(construct, &mut generated);
    }
    if generated.is_empty() {
        return Ok(());
    }
    let mut pending: Vec<&DigitalStatement> = module
        .digital_processes
        .iter()
        .map(|process| &process.body)
        .collect();
    while let Some(statement) = pending.pop() {
        match statement {
            DigitalStatement::Block(block) | DigitalStatement::Fork(block) => match &block.name {
                Some(name) if generated.contains(name) => {
                    return Err(ParseError::new(
                        ParseErrorKind::Expected {
                            expected: "a block name the module's scope does not already declare \
                                       (IEEE 1364-2005 section 12.7)"
                                .to_string(),
                            found: format!("`{name}`, the name of a generate block of the module"),
                        },
                        block.span,
                    ));
                }
                // A named block is a scope of its own.
                Some(_) => {}
                None => pending.extend(&block.statements),
            },
            DigitalStatement::Conditional(conditional) => {
                pending.push(&conditional.then_branch);
                pending.extend(conditional.else_branch.as_deref());
            }
            DigitalStatement::Case(case) => {
                pending.extend(case.items.iter().map(|item| &*item.statement));
                pending.extend(case.default.as_deref());
            }
            DigitalStatement::For(loop_) => pending.push(&loop_.body),
            DigitalStatement::While(loop_) => pending.push(&loop_.body),
            DigitalStatement::Repeat(loop_) => pending.push(&loop_.body),
            DigitalStatement::Forever(loop_) => pending.push(&loop_.body),
            DigitalStatement::Timing(timing) => pending.extend(timing.statement.as_deref()),
            DigitalStatement::Wait(wait) => pending.extend(wait.statement.as_deref()),
            _ => {}
        }
    }
    Ok(())
}

/// The explicit names of the generate blocks a construct declares in the
/// scope that holds it. A conditional written directly in another's branch
/// shares that scope (section 12.4.2).
fn generate_block_names(construct: &GenerateConstruct, names: &mut HashSet<SmolStr>) {
    let visit = |block: &GenerateBlock, names: &mut HashSet<SmolStr>| {
        if block.explicit_name
            && let Some(name) = &block.name
        {
            names.insert(name.clone());
        }
        if block.directly_nested {
            for nested in &block.nested {
                if matches!(
                    nested,
                    GenerateConstruct::Conditional(_) | GenerateConstruct::Case(_)
                ) {
                    generate_block_names(nested, names);
                }
            }
        }
    };
    match construct {
        GenerateConstruct::Loop(loop_) => visit(&loop_.body, names),
        GenerateConstruct::Conditional(conditional) => {
            visit(&conditional.then_block, names);
            if let Some(other) = &conditional.else_block {
                visit(other, names);
            }
        }
        GenerateConstruct::Case(case) => {
            for item in &case.items {
                visit(&item.block, names);
            }
            if let Some(other) = &case.default {
                visit(other, names);
            }
        }
        GenerateConstruct::Block(inner) => visit(inner, names),
    }
}

/// Section 3.8 gives an attribute the value `constant_expression`, so one
/// that reads a variable or a net of the module is not an attribute value.
/// Names the module has not declared by this point are left alone: an
/// attribute may name a tool's own value, and a port of a continuous
/// discipline (`supplySensitivity = vdd`) is a node, not a variable.
pub(super) fn reject_variable_attribute_values(
    module: &Module,
    attributes: &[Attribute],
) -> Result<(), ParseError> {
    for attribute in attributes {
        let Some(value) = &attribute.value else {
            continue;
        };
        let mut variable = None;
        crate::ast::visit_expression(value, &mut |expression| {
            if let Expression::Identifier(identifier) = expression
                && variable.is_none()
                && declares_variable_or_net(module, &identifier.name)
            {
                variable = Some(identifier.name.clone());
            }
        });
        if let Some(name) = variable {
            return Err(ParseError::new(
                ParseErrorKind::Expected {
                    expected: format!(
                        "a constant expression as the value of attribute `{}` (IEEE 1364-2005 \
                         section 3.8)",
                        attribute.name
                    ),
                    found: format!("`{name}`, a variable or net of the module"),
                },
                attribute.span,
            ));
        }
    }
    Ok(())
}

fn declares_variable_or_net(module: &Module, name: &SmolStr) -> bool {
    module
        .digital_nets
        .iter()
        .any(|net| net.items.iter().any(|item| item.name == *name))
        || module
            .digital_variables
            .iter()
            .any(|variable| variable.items.iter().any(|item| item.name == *name))
        || module
            .variables
            .iter()
            .any(|variable| variable.items.iter().any(|item| item.name == *name))
}
