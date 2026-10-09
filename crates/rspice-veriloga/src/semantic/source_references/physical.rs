//! Borrow declaration shapes for type checking. Elaboration redirects these
//! aliases to the existing occurrence before allocating executable storage.
use super::*;

pub(super) fn literal(value: i64, span: Span) -> Expression {
    Expression::Number(NumberLit {
        value: value as f64,
        raw: value.to_string().into(),
        span,
    })
}

fn range(bounds: VectorBounds, span: Span) -> VectorRange {
    VectorRange {
        msb: literal(bounds.msb, span),
        lsb: literal(bounds.lsb, span),
        span,
    }
}

impl Resolver {
    pub(super) fn import_physical(
        &mut self,
        owner: usize,
        target: usize,
        name: &SmolStr,
        symbol: &SmolStr,
        span: Span,
    ) -> CompileResult<()> {
        if self.frames[target].physical.is_none() {
            let (expanded, nodes) = super::super::node_vectors::declarations(
                &self.frames[target].source,
                &self.sources.disciplines,
            )?;
            self.frames[target].physical = Some((expanded.into_owned(), nodes));
        }
        let (expanded, nodes) = self.frames[target].physical.as_ref().unwrap();
        let source = &self.frames[target].source;
        let path = self.reference_path(owner, target);
        if let Some(mut branch) = source.branches.iter().find(|b| b.name == *name).cloned() {
            let lanes = nodes.reference_lanes(name, true);
            let constants = DigitalConstants::from_module(source);
            let mut expressions = branch
                .pos_prefix
                .iter_mut()
                .chain(&mut branch.neg_prefix)
                .collect::<Vec<_>>();
            for select in [&mut branch.pos_select, &mut branch.neg_select]
                .into_iter()
                .flatten()
            {
                match select {
                    PackedSelect::Bit(index) => expressions.push(index),
                    PackedSelect::Part { msb, lsb } => {
                        expressions.extend([msb.as_mut(), lsb.as_mut()])
                    }
                }
            }
            if let Some(range) = &mut branch.range {
                expressions.extend([&mut range.msb, &mut range.lsb]);
            }
            let dependencies =
                SourceParameters::new(source).dependencies(expressions.iter().map(|e| &**e))?;
            for expression in expressions {
                let value =
                    super::super::node_vectors::integer(expression, &constants, source.time_scale)?;
                *expression = literal(value, expression.span());
            }
            self.retain_dependencies(owner, target, dependencies);
            branch.name = symbol.clone();
            for endpoint in [&mut branch.pos, &mut branch.neg] {
                if !endpoint.is_empty() && *endpoint != "0" {
                    *endpoint = self.import_symbol(owner, target, endpoint, None, span)?;
                }
            }
            if !branch.is_port {
                self.frames[owner].source.foreign_physical.insert(
                    symbol.clone(),
                    ForeignPhysicalReference {
                        path,
                        lanes,
                        kind: ForeignPhysicalKind::Branch,
                        span,
                    },
                );
            }
            self.frames[owner].source.branches.push(branch);
        } else {
            let lanes = nodes.reference_lanes(name, false);
            let first = &lanes[0];
            let discipline = expanded
                .nets
                .iter()
                .filter(|n| n.names.contains(first))
                .find_map(|n| n.discipline.clone())
                .or_else(|| {
                    expanded
                        .port_declarations
                        .iter()
                        .filter(|p| p.names.contains(first))
                        .find_map(|p| p.discipline.clone())
                });
            let ground = expanded
                .nets
                .iter()
                .any(|n| n.is_ground && n.names.contains(first));
            if !ground
                && !discipline.as_ref().is_some_and(|d| {
                    self.sources
                        .disciplines
                        .get_discipline(d)
                        .is_some_and(|d| d.domain == Domain::Continuous)
                })
            {
                return Err(error(
                    format!(
                        "`{}.{name}` is not a continuous net, branch, parameter or analog function",
                        self.frames[target].path
                    ),
                    span,
                ));
            }
            let mut dependencies = Vec::new();
            for net in &source.nets {
                if !net.names.contains(name) {
                    continue;
                }
                if let Some(range) = &net.range {
                    dependencies.extend([&range.msb, &range.lsb]);
                }
                for (_, dimensions) in net.dimensions.iter().filter(|(n, _)| n == name) {
                    for dimension in dimensions {
                        dependencies.extend([&dimension.start, &dimension.end]);
                    }
                }
            }
            for port in &source.port_declarations {
                if port.names.contains(name)
                    && let Some(range) = &port.range
                {
                    dependencies.extend([&range.msb, &range.lsb]);
                }
            }
            for net in &source.digital_nets {
                for item in net.items.iter().filter(|item| item.name == *name) {
                    if let Some(range) = &net.range {
                        dependencies.extend([&range.msb, &range.lsb]);
                    }
                    for dimension in &item.dimensions {
                        dependencies.extend([&dimension.start, &dimension.end]);
                    }
                }
            }
            let dependencies = SourceParameters::new(source).dependencies(dependencies)?;
            let array = nodes.arrays.get(name);
            let bounds = array
                .and_then(|a| a.bus)
                .or_else(|| nodes.vectors.get(name).map(|v| v.bounds));
            let dimensions = array
                .map(|a| {
                    let rank = a.dimensions.len() - usize::from(a.bus.is_some());
                    vec![(
                        symbol.clone(),
                        a.dimensions[..rank]
                            .iter()
                            .map(|b| ArrayDimension {
                                start: literal(b.msb, span),
                                end: literal(b.lsb, span),
                                span,
                            })
                            .collect(),
                    )]
                })
                .unwrap_or_default();
            let is_port = source.ports.iter().any(|p| p.name == *name);
            let declaration = NetDecl {
                dimensions,
                range: bounds.map(|b| range(b, span)),
                discipline,
                names: vec![symbol.clone()],
                is_ground: ground,
                is_internal: true,
                span,
            };
            self.retain_dependencies(owner, target, dependencies);
            let source = &mut self.frames[owner].source;
            source.nets.push(declaration);
            source.foreign_physical.insert(
                symbol.clone(),
                ForeignPhysicalReference {
                    path,
                    lanes,
                    kind: ForeignPhysicalKind::Node { is_port },
                    span,
                },
            );
        }
        self.frames[owner].physical = None;
        Ok(())
    }
}
