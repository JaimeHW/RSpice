//! Explicit branch() references name an existing branch owned by the selected
//! occurrence. Endpoint names are relative to it; selectors remain lexical.
use super::*;

struct Terminal {
    name: SmolStr,
    prefix: Vec<Expression>,
    select: Option<PackedSelect>,
    nodes: Vec<ForeignPhysicalNode>,
    vector: bool,
}

impl Resolver {
    pub(super) fn import_unnamed(
        &mut self,
        owner: usize,
        target: usize,
        branch: &HierarchicalBranch,
        symbol: &SmolStr,
        span: Span,
    ) -> CompileResult<()> {
        let pos = self.unnamed_terminal(owner, target, &branch.pos, branch.is_port, span)?;
        let neg = if let Some(neg) = &branch.neg {
            self.unnamed_terminal(owner, target, neg, false, span)?
        } else {
            Terminal {
                name: "0".into(),
                prefix: Vec::new(),
                select: None,
                nodes: vec![ForeignPhysicalNode {
                    path: "".into(),
                    name: "0".into(),
                }],
                vector: false,
            }
        };
        if pos.vector && neg.vector && pos.nodes.len() != neg.nodes.len() {
            return Err(error("branch() terminal widths differ", span));
        }
        let width = pos.nodes.len().max(neg.nodes.len());
        if !branch.is_port {
            let pairs = (0..width)
                .map(|i| {
                    (
                        pos.nodes[if pos.vector { i } else { 0 }].clone(),
                        neg.nodes[if neg.vector { i } else { 0 }].clone(),
                    )
                })
                .collect();
            let path = self.frames[target]
                .path
                .strip_prefix(self.frames[owner].path.as_str())
                .expect("descendant branch owner")
                .trim_start_matches('.')
                .into();
            self.frames[owner].source.foreign_physical.insert(
                symbol.clone(),
                ForeignPhysicalReference {
                    path,
                    lanes: Vec::new(),
                    kind: ForeignPhysicalKind::Unnamed { pairs },
                    span,
                },
            );
        }
        self.frames[owner].source.branches.push(BranchDecl {
            name: symbol.clone(),
            pos: pos.name,
            neg: neg.name,
            pos_prefix: pos.prefix,
            neg_prefix: neg.prefix,
            pos_select: pos.select,
            neg_select: neg.select,
            range: None,
            is_port: branch.is_port,
            span,
        });
        self.frames[owner].physical = None;
        Ok(())
    }

    fn unnamed_terminal(
        &mut self,
        owner: usize,
        branch_owner: usize,
        terminal: &HierarchicalBranchTerminal,
        port: bool,
        span: Span,
    ) -> CompileResult<Terminal> {
        let source = &self.frames[owner].source;
        let constants = DigitalConstants::from_module(source);
        let scale = source.time_scale;
        let close = |expression: &mut Expression| -> CompileResult<()> {
            let value = super::super::node_vectors::integer(expression, &constants, scale)?;
            *expression = super::physical::literal(value, expression.span());
            Ok(())
        };
        let terminal_name = &terminal.name.segments.last().expect("parsed terminal").name;
        if terminal_name == "0" && terminal.name.segments.len() == 1 && !port {
            if !terminal.prefix.is_empty() || terminal.select.is_some() {
                return Err(error("ground cannot have a selector", span));
            }
            return Ok(Terminal {
                name: "0".into(),
                prefix: Vec::new(),
                select: None,
                nodes: vec![ForeignPhysicalNode {
                    path: "".into(),
                    name: "0".into(),
                }],
                vector: false,
            });
        }
        let (target, name) = if terminal.name.segments.len() == 1 && !terminal.name.absolute {
            let name = self.frames[branch_owner].member(&[], terminal_name).ok_or_else(||
                error(format!("branch terminal `{terminal_name}` is not declared in the selected instance"), span))?;
            (branch_owner, name)
        } else {
            let scopes = terminal.name.segments[..terminal.name.segments.len() - 1]
                .iter()
                .map(|segment| {
                    Ok(HierarchicalScopeKey {
                        name: segment.name.clone(),
                        index: segment
                            .index
                            .as_ref()
                            .map(|e| super::super::node_vectors::integer(e, &constants, scale))
                            .transpose()?,
                    })
                })
                .collect::<CompileResult<Vec<_>>>()?;
            self.target(
                branch_owner,
                &ScopedHierarchicalReference {
                    source: terminal.name.clone(),
                    origin: Vec::new(),
                    scopes,
                    index_dependencies: Vec::new(),
                },
            )?
        };
        if self.frames[target].physical.is_none() {
            let (expanded, nodes) = super::super::node_vectors::declarations(
                &self.frames[target].source,
                &self.sources.disciplines,
            )?;
            self.frames[target].physical = Some((expanded.into_owned(), nodes));
        }
        let (expanded, physical) = self.frames[target].physical.as_ref().unwrap();
        if self.frames[target]
            .source
            .branches
            .iter()
            .any(|b| b.name == name)
        {
            return Err(error(
                "branch() terminals must be nets, not named branches",
                span,
            ));
        }
        if port
            && !self.frames[target]
                .source
                .ports
                .iter()
                .any(|p| p.name == name)
        {
            return Err(error(
                "branch(<port>) must name a port of the selected instance",
                span,
            ));
        }
        let mut prefix = terminal.prefix.clone();
        for expression in &mut prefix {
            close(expression)?;
        }
        let mut select = terminal.select.clone();
        if let Some(select) = &mut select {
            match select {
                PackedSelect::Bit(index) => close(index)?,
                PackedSelect::Part { msb, lsb } => {
                    close(msb)?;
                    close(lsb)?;
                }
            }
        }
        let (lanes, vector) = super::super::node_vectors::reference_terminal(
            &name,
            &prefix,
            select.as_ref(),
            physical,
            &constants,
            scale,
            span,
        )?;
        let path: SmolStr = self.frames[target]
            .path
            .strip_prefix(self.frames[branch_owner].path.as_str())
            .expect("descendant endpoint")
            .trim_start_matches('.')
            .into();
        let nodes = lanes
            .into_iter()
            .map(|name| {
                if expanded
                    .nets
                    .iter()
                    .any(|net| net.is_ground && net.names.contains(&name))
                {
                    ForeignPhysicalNode {
                        path: "".into(),
                        name: "0".into(),
                    }
                } else {
                    ForeignPhysicalNode {
                        path: path.clone(),
                        name,
                    }
                }
            })
            .collect();
        // Imports retain all target shape dependencies; selector dependencies
        // were retained in the caller by the generated-reference binder.
        let name = self.import_symbol(owner, target, &name, None, span)?;
        Ok(Terminal {
            name,
            prefix,
            select,
            nodes,
            vector,
        })
    }
}
