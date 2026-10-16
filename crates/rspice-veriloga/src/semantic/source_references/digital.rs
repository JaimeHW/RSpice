//! Borrow a discrete-domain declaration for a hierarchical reference to it.
//!
//! The referring module declares a copy of the target's net or variable under
//! the reference's symbol, with its bounds closed in the target's scope, so the
//! referring bodies are checked against the target's type. Digital lowering
//! binds the symbol to the target's own signal; the copy never takes storage.
use super::physical::literal;
use super::*;

/// The declaration a discrete-domain target is borrowed as.
enum Borrowed {
    Net(DigitalNetDecl),
    Variable(DigitalVariableDecl),
}

impl Resolver {
    /// Whether `name` is a discrete-domain net or variable of `target`.
    pub(super) fn declares_digital(&self, target: usize, name: &str) -> bool {
        let source = &self.frames[target].source;
        let item = |items: &[DigitalDeclItem]| items.iter().any(|item| item.name == name);
        source.digital_nets.iter().any(|net| item(&net.items))
            || source
                .digital_variables
                .iter()
                .any(|variable| item(&variable.items))
            || source.variables.iter().any(|declaration| {
                declaration.var_type != VarType::String
                    && declaration.items.iter().any(|item| item.name == name)
            })
            || self.implicit_digital_port(target, name).is_some()
    }

    /// A port of `target` whose only declaration is the port itself, which
    /// section 12.3.3 makes an implicit net, and which no continuous
    /// discipline claims.
    fn implicit_digital_port(&self, target: usize, name: &str) -> Option<&PortDeclaration> {
        let source = &self.frames[target].source;
        if source
            .nets
            .iter()
            .any(|net| net.names.iter().any(|n| n == name))
        {
            return None;
        }
        source.port_declarations.iter().find(|port| {
            port.net_type.is_none()
                && port.names.iter().any(|n| n == name)
                && port.discipline.as_ref().is_none_or(|discipline| {
                    self.sources
                        .disciplines
                        .get_discipline(discipline)
                        .is_some_and(|discipline| discipline.domain == Domain::Discrete)
                })
        })
    }

    pub(super) fn import_digital(
        &mut self,
        owner: usize,
        target: usize,
        name: &SmolStr,
        symbol: &SmolStr,
        span: Span,
    ) -> CompileResult<()> {
        let source = &self.frames[target].source;
        let constants = DigitalConstants::from_module(source);
        let time_scale = source.time_scale;
        let mut dependencies: Vec<&Expression> = Vec::new();
        let close = |expression: &Expression| -> CompileResult<Expression> {
            let value = super::super::node_vectors::integer(expression, &constants, time_scale)?;
            Ok(literal(value, span))
        };
        let close_range = |range: &Option<VectorRange>| -> CompileResult<Option<VectorRange>> {
            range
                .as_ref()
                .map(|range| {
                    Ok(VectorRange {
                        msb: close(&range.msb)?,
                        lsb: close(&range.lsb)?,
                        span,
                    })
                })
                .transpose()
        };
        let close_item = |item: &DigitalDeclItem| -> CompileResult<DigitalDeclItem> {
            Ok(DigitalDeclItem {
                name: symbol.clone(),
                dimensions: item
                    .dimensions
                    .iter()
                    .map(|dimension| {
                        Ok(ArrayDimension {
                            start: close(&dimension.start)?,
                            end: close(&dimension.end)?,
                            span,
                        })
                    })
                    .collect::<CompileResult<_>>()?,
                init: None,
                span,
            })
        };
        let mut borrowed = None;
        for net in &source.digital_nets {
            if let Some(item) = net.items.iter().find(|item| item.name == *name) {
                dependencies.extend(net.range.iter().flat_map(|r| [&r.msb, &r.lsb]));
                dependencies.extend(item.dimensions.iter().flat_map(|d| [&d.start, &d.end]));
                borrowed = Some(Borrowed::Net(DigitalNetDecl {
                    kind: net.kind,
                    signedness: net.signedness,
                    range: close_range(&net.range)?,
                    items: vec![close_item(item)?],
                    span,
                }));
                break;
            }
        }
        if borrowed.is_none() {
            for variable in &source.digital_variables {
                if let Some(item) = variable.items.iter().find(|item| item.name == *name) {
                    dependencies.extend(variable.range.iter().flat_map(|r| [&r.msb, &r.lsb]));
                    dependencies.extend(item.dimensions.iter().flat_map(|d| [&d.start, &d.end]));
                    borrowed = Some(Borrowed::Variable(DigitalVariableDecl {
                        kind: variable.kind,
                        signedness: variable.signedness,
                        range: close_range(&variable.range)?,
                        items: vec![close_item(item)?],
                        span,
                    }));
                    break;
                }
            }
        }
        if borrowed.is_none() {
            for declaration in &source.variables {
                let kind = match declaration.var_type {
                    VarType::Integer => DigitalVariableKind::Integer,
                    VarType::Real => DigitalVariableKind::Real,
                    VarType::String => continue,
                };
                if let Some(item) = declaration.items.iter().find(|item| item.name == *name) {
                    dependencies.extend(item.dimensions.iter().flat_map(|d| [&d.start, &d.end]));
                    let item = DigitalDeclItem {
                        name: item.name.clone(),
                        dimensions: item.dimensions.clone(),
                        init: None,
                        span: item.span,
                    };
                    borrowed = Some(Borrowed::Variable(DigitalVariableDecl {
                        kind,
                        signedness: if kind == DigitalVariableKind::Integer {
                            Signedness::Signed
                        } else {
                            Signedness::Unsigned
                        },
                        range: None,
                        items: vec![close_item(&item)?],
                        span,
                    }));
                    break;
                }
            }
        }
        if borrowed.is_none()
            && let Some(port) = self.implicit_digital_port(target, name)
        {
            dependencies.extend(port.range.iter().flat_map(|r| [&r.msb, &r.lsb]));
            borrowed = Some(Borrowed::Net(DigitalNetDecl {
                kind: DigitalNetKind::Wire,
                signedness: port.signedness,
                range: close_range(&port.range)?,
                items: vec![DigitalDeclItem {
                    name: symbol.clone(),
                    dimensions: Vec::new(),
                    init: None,
                    span,
                }],
                span,
            }));
        }
        let Some(borrowed) = borrowed else {
            return Err(error(
                format!("`{name}` is not a discrete-domain net or variable"),
                span,
            ));
        };
        let dependencies = SourceParameters::new(source).dependencies(dependencies)?;
        let path = self.reference_path(owner, target);
        self.retain_dependencies(owner, target, dependencies);
        let owner_source = &mut self.frames[owner].source;
        match borrowed {
            Borrowed::Net(net) => owner_source.digital_nets.push(net),
            Borrowed::Variable(variable) => owner_source.digital_variables.push(variable),
        }
        owner_source.foreign_digital.insert(
            symbol.clone(),
            ForeignDigitalReference {
                path,
                name: name.clone(),
                span,
            },
        );
        self.frames[owner].physical = None;
        Ok(())
    }
}
