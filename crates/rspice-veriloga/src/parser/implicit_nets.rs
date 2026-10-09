//! Materialize structural implicit nets before generate expansion and analysis.
use crate::ast::*;
use crate::error::{ParseError, ParseErrorKind};
use crate::source::Span;
use smol_str::SmolStr;
use std::collections::HashSet;

pub(super) fn default_type(name: &str, span: Span) -> Result<bool, ParseError> {
    match name {
        "wire" | "tri" => Ok(true),
        "none" => Ok(false),
        _ => Err(ParseError::new(
            ParseErrorKind::UnsupportedConstruct {
                context: "default_nettype".into(),
                found: format!("`{name}`; supported net defaults are wire, tri and none"),
            },
            span,
        )),
    }
}

fn allowed(defaults: &[(u32, bool)], span: Span) -> bool {
    let index = defaults.partition_point(|(offset, _)| *offset <= span.start);
    index == 0 || defaults[index - 1].1
}

fn refused(name: &str, span: Span) -> ParseError {
    ParseError::new(
        ParseErrorKind::UnsupportedConstruct {
            context: "implicit net declaration".into(),
            found: format!("`{name}` requires a net declaration under `default_nettype none`"),
        },
        span,
    )
}

pub(super) fn declare(module: &mut Module, defaults: &[(u32, bool)]) -> Result<(), ParseError> {
    scope(module, &HashSet::new(), defaults)?;
    Ok(())
}

fn scope(
    module: &mut Module,
    parents: &HashSet<SmolStr>,
    defaults: &[(u32, bool)],
) -> Result<HashSet<SmolStr>, ParseError> {
    let mut known = parents.clone();
    known.extend(module.declared_names());
    known.insert("inf".into());
    // Direction and range alone ordinarily imply a wire port. `none` requires
    // an explicit type or discipline, including a separate body declaration.
    for port in &module.port_declarations {
        if port.net_type.is_some() || port.discipline.is_some() || allowed(defaults, port.span) {
            continue;
        }
        for name in &port.names {
            let declared = module.nets.iter().any(|net| net.names.contains(name))
                || module
                    .digital_nets
                    .iter()
                    .any(|net| net.items.iter().any(|item| item.name == *name))
                || module
                    .digital_variables
                    .iter()
                    .any(|net| net.items.iter().any(|item| item.name == *name))
                || module
                    .variables
                    .iter()
                    .any(|net| net.items.iter().any(|item| item.name == *name));
            if !declared {
                return Err(refused(name, port.span));
            }
        }
    }
    let mut candidates = Vec::new();
    for instance in &module.instances {
        for connection in &instance.connections {
            let actual = match connection {
                Connection::Named { signal, .. } | Connection::Ordered { signal, .. } => {
                    signal.as_ref()
                }
            };
            if let Some(actual) = actual {
                visit_expression(actual, &mut |expression| {
                    let (name, span) = match expression {
                        Expression::Identifier(id) => (&id.name, id.span),
                        Expression::ArrayAccess(access) => (&access.array, access.span),
                        Expression::Digital(DigitalExpr::PartSelect(select)) => {
                            (&select.name, select.span)
                        }
                        Expression::Digital(DigitalExpr::ArraySelect(select)) => {
                            (&select.name, select.span)
                        }
                        _ => return,
                    };
                    candidates.push((name.clone(), span));
                });
            }
        }
    }
    // Continuous-assignment targets can declare implicit nets. Reads in an
    // arbitrary behavioral expression do not create declarations.
    for assignment in &module.continuous_assigns {
        candidates.extend(
            assignment
                .target
                .written_names()
                .into_iter()
                .map(|(name, span)| (name.clone(), span)),
        );
    }
    candidates.sort_by_key(|(_, span)| span.start);
    for (name, span) in candidates {
        if known.contains(&name) {
            continue;
        }
        if !allowed(defaults, span) {
            return Err(refused(&name, span));
        }
        known.insert(name.clone());
        module.digital_nets.push(DigitalNetDecl {
            kind: DigitalNetKind::Wire,
            signedness: Signedness::Unsigned,
            range: None,
            items: vec![DigitalDeclItem {
                name,
                dimensions: Vec::new(),
                init: None,
                span,
            }],
            span,
        });
    }
    let reserved = module.declared_names();
    constructs(&mut module.generates, &known, &reserved, defaults)?;
    Ok(known)
}

fn constructs(
    constructs: &mut [GenerateConstruct],
    known: &HashSet<SmolStr>,
    reserved: &HashSet<SmolStr>,
    defaults: &[(u32, bool)],
) -> Result<(), ParseError> {
    let mut reserved = reserved.clone();
    for construct in constructs.iter() {
        let mut declared = Vec::new();
        block_names(construct, &mut declared);
        // Alternatives of one conditional may have the same name. Names from
        // separate constructs conflict even when their branches are inactive.
        let mut alternatives = HashSet::new();
        for (name, span) in declared {
            if !alternatives.insert(name.clone()) {
                continue;
            }
            if !reserved.insert(name.clone()) {
                return Err(ParseError::new(
                    ParseErrorKind::UnsupportedConstruct {
                        context: "generate scope".into(),
                        found: format!(
                            "block name '{name}' conflicts with another declaration in this scope (VAMS-2023 6.6.2, 6.8)"
                        ),
                    },
                    span,
                ));
            }
        }
    }
    for (index, construct) in constructs.iter_mut().enumerate() {
        name_construct(construct, index + 1, known, &reserved, defaults)?;
    }
    Ok(())
}

fn block_names<'a>(construct: &'a GenerateConstruct, names: &mut Vec<(&'a SmolStr, Span)>) {
    let blocks: Vec<_> = match construct {
        GenerateConstruct::Block(block) => vec![block],
        GenerateConstruct::Loop(value) => vec![&value.body],
        GenerateConstruct::Conditional(value) => std::iter::once(&value.then_block)
            .chain(value.else_block.as_ref())
            .collect(),
        GenerateConstruct::Case(value) => value
            .items
            .iter()
            .map(|item| &item.block)
            .chain(value.default.as_ref())
            .collect(),
    };
    for block in blocks {
        if block.directly_nested {
            for nested in &block.nested {
                block_names(nested, names);
            }
        } else if let Some(name) = &block.name {
            names.push((name, block.span));
        }
    }
}

fn name_construct(
    construct: &mut GenerateConstruct,
    ordinal: usize,
    known: &HashSet<SmolStr>,
    reserved: &HashSet<SmolStr>,
    defaults: &[(u32, bool)],
) -> Result<(), ParseError> {
    let blocks: Vec<_> = match construct {
        GenerateConstruct::Block(block) => vec![block],
        GenerateConstruct::Loop(value) => vec![&mut value.body],
        GenerateConstruct::Conditional(value) => std::iter::once(&mut value.then_block)
            .chain(value.else_block.as_mut())
            .collect(),
        GenerateConstruct::Case(value) => value
            .items
            .iter_mut()
            .map(|item| &mut item.block)
            .chain(value.default.as_mut())
            .collect(),
    };
    for block in blocks {
        if block.directly_nested {
            // Direct nesting adds neither a scope nor another construct number.
            for nested in &mut block.nested {
                name_construct(nested, ordinal, known, reserved, defaults)?;
            }
            continue;
        }
        if block.name.is_none() {
            let mut number = ordinal.to_string();
            while reserved.contains(format!("genblk{number}").as_str()) {
                number.insert(0, '0');
            }
            block.name = Some(format!("genblk{number}").into());
        }
        let locals = scope(&mut block.items, known, defaults)?;
        let reserved = block.items.declared_names();
        constructs(&mut block.nested, &locals, &reserved, defaults)?;
    }
    Ok(())
}
