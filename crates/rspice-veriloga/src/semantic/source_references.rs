//! Source-level occurrence discovery for foreign parameter references. No solver
//! storage or hidden ports are created; both domains analyze the same bound value.
use super::elaboration::parameters::{ParameterDependencies, SourceParameters};

mod functions;
use super::*;
use std::borrow::Cow;
use std::sync::Arc;

type Sources = Arc<HashMap<SmolStr, Module>>;

fn error(message: impl Into<String>, span: Span) -> CompileError {
    SemanticError::new(
        SemanticErrorKind::UnsupportedFeature(format!(
            "hierarchical reference: {}",
            message.into()
        )),
        span,
    )
    .into()
}

/// Refresh the source closure before body checks, including configuration replay.
/// Catalog templates have no back-reference, so sharing cannot form an Arc cycle.
pub(crate) fn prepare(source: &SourceFile) -> CompileResult<Cow<'_, SourceFile>> {
    let modules = source
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Module(module) | Item::ConnectModule(module) => Some(module),
            _ => None,
        })
        .collect::<Vec<_>>();
    if !modules
        .iter()
        .any(|module| !module.pending_hierarchical_references.is_empty())
    {
        return Ok(Cow::Borrowed(source));
    }
    let mut catalog = HashMap::new();
    for module in modules {
        let mut raw = module.clone();
        raw.reference_sources = None;
        crate::parser::expand_specialized_generates(&mut raw)?;
        if let Some(previous) = catalog.insert(raw.name.clone(), raw) {
            return Err(SemanticError::new(
                SemanticErrorKind::DuplicateSymbol {
                    name: module.name.clone(),
                    first_defined: previous.span,
                },
                module.span,
            )
            .into());
        }
    }
    let catalog = Arc::new(catalog);
    let mut prepared = source.clone();
    for item in &mut prepared.items {
        let (Item::Module(module) | Item::ConnectModule(module)) = item else {
            continue;
        };
        *module = catalog[&module.name].clone();
        module.reference_sources = Some(catalog.clone());
        bind(module)?;
    }
    Ok(Cow::Owned(prepared))
}

pub(crate) fn bind(module: &mut Module) -> CompileResult<()> {
    if module.pending_hierarchical_references.is_empty() {
        return Ok(());
    }
    let Some(sources) = module.reference_sources.clone() else {
        return Ok(());
    };
    let mut resolver = Resolver {
        sources,
        frames: vec![Frame::new(module.clone(), "".into(), 0)],
        children: HashMap::new(),
        active: HashSet::new(),
        resolved: HashSet::new(),
        imports: HashMap::new(),
        active_imports: HashSet::new(),
        next_import: 0,
        builtins: crate::types::FunctionRegistry::new(),
    };
    let references = module
        .pending_hierarchical_references
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    for symbol in references {
        resolver.reference(0, &symbol)?;
    }
    *module = resolver.frames.remove(0).source;
    Ok(())
}

struct Frame {
    source: Module,
    path: SmolStr,
    depth: usize,
    instances: HashMap<SmolStr, usize>,
    root_members: HashSet<SmolStr>,
}
impl Frame {
    fn new(source: Module, path: SmolStr, depth: usize) -> Self {
        let instances = source
            .instances
            .iter()
            .enumerate()
            .map(|(index, item)| (item.name.clone(), index))
            .collect();
        let root_members = source.declared_names();
        Self {
            source,
            path,
            depth,
            instances,
            root_members,
        }
    }
    fn member(&self, scope: &[HierarchicalScopeKey], name: &str) -> Option<SmolStr> {
        if let Some(scope) = self.source.hierarchical_scopes.get(scope) {
            return scope.members.get(name).cloned();
        }
        if scope.is_empty() && self.root_members.contains(name) {
            Some(name.into())
        } else {
            None
        }
    }
    fn has_scope(&self, scope: &[HierarchicalScopeKey], first: &str) -> bool {
        self.source
            .hierarchical_scopes
            .get(scope)
            .is_some_and(|scope| scope.children.contains(first))
    }
}

struct Resolver {
    sources: Sources,
    frames: Vec<Frame>,
    children: HashMap<(usize, SmolStr), usize>,
    active: HashSet<(usize, SmolStr)>,
    resolved: HashSet<(usize, SmolStr)>,
    imports: HashMap<(usize, usize, SmolStr), SmolStr>,
    active_imports: HashSet<(usize, usize, SmolStr)>,
    next_import: usize,
    builtins: crate::types::FunctionRegistry,
}

impl Resolver {
    fn reference(&mut self, owner: usize, symbol: &SmolStr) -> CompileResult<()> {
        let key = (owner, symbol.clone());
        if self.resolved.contains(&key) {
            return Ok(());
        }
        let reference = self.frames[owner].source.pending_hierarchical_references[symbol].clone();
        if !self.active.insert(key.clone()) {
            return Err(error(
                "cyclic parameter/instance binding",
                reference.source.span,
            ));
        }
        if self.active.len() > 256 {
            return Err(error(
                "parameter binding exceeds the dependency depth limit of 256",
                reference.source.span,
            ));
        }
        let declarations = self.frames[owner]
            .source
            .parameters
            .iter()
            .chain(&self.frames[owner].source.localparams);
        for declaration in declarations {
            let mut forbidden = false;
            let mut expressions: Vec<_> = declaration.default.iter().collect();
            if let Some(range) = &declaration.packed_range {
                expressions.extend([&range.msb, &range.lsb]);
            }
            for dimension in &declaration.dimensions {
                expressions.extend([&dimension.start, &dimension.end]);
            }
            if let Some(range) = &declaration.range {
                for bound in &range.bounds {
                    expressions.extend(bound.lower.iter().chain(&bound.upper));
                }
                expressions.extend(&range.exclude);
            }
            expressions.extend(
                declaration
                    .attributes
                    .iter()
                    .filter_map(|attribute| attribute.value.as_ref()),
            );
            for expression in expressions {
                crate::ast::visit_expression(expression, &mut |value| {
                    forbidden |= matches!(value, Expression::Identifier(id) if id.name == *symbol)
                        || matches!(value, Expression::ArrayAccess(access) if access.array == *symbol)
                        || matches!(value, Expression::Digital(expr) if expr.base_name() == Some(symbol))
                        || matches!(value, Expression::Call(call) if call.name == *symbol);
                });
            }
            if forbidden {
                return Err(error(
                    "a parameter declaration cannot reference outside its module (VAMS-2023 6.7.1)",
                    reference.source.span,
                ));
            }
        }
        let (target, name) = self.target(owner, &reference)?;
        self.import_symbol(
            owner,
            target,
            &name,
            Some(symbol.clone()),
            reference.source.span,
        )?;
        let dependencies = SourceParameters::new(&self.frames[owner].source)
            .dependencies(reference.index_dependencies.iter())?;
        self.retain_dependencies(owner, owner, dependencies);
        self.active.remove(&key);
        self.resolved.insert(key);
        Ok(())
    }

    fn fresh_import(&mut self, owner: usize) -> SmolStr {
        loop {
            let name: SmolStr = format!("$rspice_import_{}", self.next_import).into();
            self.next_import += 1;
            let source = &self.frames[owner].source;
            if !source.reserved_identifiers.contains(&name)
                && !source.declared_names().contains(&name)
                && !source.pending_hierarchical_references.contains_key(&name)
                && !self.imports.values().any(|value| value == &name)
            {
                return name;
            }
        }
    }

    fn import_symbol(
        &mut self,
        owner: usize,
        target: usize,
        name: &SmolStr,
        preferred: Option<SmolStr>,
        span: Span,
    ) -> CompileResult<SmolStr> {
        let key = (owner, target, name.clone());
        if self.active_imports.contains(&key) {
            return Err(error("recursive analog function binding", span));
        }
        if preferred.is_none()
            && let Some(symbol) = self.imports.get(&key)
        {
            return Ok(symbol.clone());
        }
        if self.active_imports.len() >= 256 {
            return Err(error(
                "function binding exceeds the dependency depth limit of 256",
                span,
            ));
        }
        let symbol = preferred.unwrap_or_else(|| self.fresh_import(owner));
        self.active_imports.insert(key.clone());
        // Reserve before following callees so imported names cannot collide.
        self.imports.entry(key.clone()).or_insert(symbol.clone());
        let source = &self.frames[target].source;
        if let Some(declaration) = source
            .parameters
            .iter()
            .chain(&source.localparams)
            .find(|declaration| declaration.name == *name)
            .cloned()
        {
            self.import_parameter(owner, target, declaration, &symbol, span)?;
        } else if let Some(function) = source
            .functions
            .iter()
            .find(|function| function.name == *name)
            .cloned()
        {
            self.import_function(owner, target, function, &symbol)?;
        } else {
            return Err(error(
                format!(
                    "`{}.{name}` is not a parameter or analog function; foreign storage binding remains unimplemented",
                    self.frames[target].path
                ),
                span,
            ));
        }
        self.active_imports.remove(&key);
        Ok(symbol)
    }

    fn import_parameter(
        &mut self,
        owner: usize,
        target: usize,
        declaration: ParameterDecl,
        symbol: &SmolStr,
        span: Span,
    ) -> CompileResult<()> {
        let target_source = &self.frames[target].source;
        if !declaration.dimensions.is_empty() {
            return Err(error(
                "foreign parameter arrays require aggregate reference binding",
                span,
            ));
        }
        let query = Expression::Identifier(Identifier {
            name: declaration.name.clone(),
            span,
        });
        let mut typed = declaration.clone();
        typed.default = Some(query.clone());
        let constants = DigitalConstants::from_module(target_source);
        let value = crate::canonical_ir::digital_lower::parameter_override_literal(
            &typed,
            &constants,
            target_source.time_scale,
        )
        .map_err(|detail| error(detail, span))?;
        let mut dependencies = vec![&query];
        if let Some(range) = &declaration.packed_range {
            dependencies.extend([&range.msb, &range.lsb]);
        }
        let target_dependencies =
            SourceParameters::new(target_source).dependencies(dependencies)?;
        let mut imported = declaration;
        imported.name = symbol.clone();
        imported.default = Some(value);
        imported.is_given = false;
        imported.range = None;
        imported.attributes.clear();
        imported.span = span;
        if let Some(range) = &mut imported.packed_range {
            for expression in [&mut range.msb, &mut range.lsb] {
                let value = crate::canonical_ir::digital_lower::elaboration_constant(
                    expression,
                    &constants,
                    target_source.time_scale,
                )
                .and_then(|value| match value {
                    crate::numeric_literal::NumericLiteralValue::Integer(value) => Some(value),
                    _ => None,
                })
                .ok_or_else(|| {
                    error(
                        "parameter packed bounds require integer constants",
                        expression.span(),
                    )
                })?;
                *expression = Expression::Number(NumberLit {
                    value: value as f64,
                    raw: value.to_string().into(),
                    span: expression.span(),
                });
            }
        }
        self.retain_dependencies(owner, target, target_dependencies);
        self.frames[owner].source.localparams.push(imported);
        Ok(())
    }

    fn retain_dependencies(
        &mut self,
        owner: usize,
        target: usize,
        dependencies: ParameterDependencies,
    ) {
        let target_path = &self.frames[target].path;
        let owner_path = &self.frames[owner].path;
        let relative = target_path
            .strip_prefix(owner_path.as_str())
            .expect("reference target is a descendant")
            .trim_start_matches('.');
        let qualify = |name: SmolStr| -> SmolStr {
            if relative.is_empty() {
                name
            } else {
                format!("{relative}.{name}").into()
            }
        };
        let values = dependencies
            .values
            .into_iter()
            .map(qualify)
            .collect::<Vec<_>>();
        let given = dependencies
            .given
            .into_iter()
            .map(qualify)
            .collect::<Vec<_>>();
        let source = &mut self.frames[owner].source;
        source.hierarchical_parameter_values.extend(values);
        source.hierarchical_parameter_given.extend(given);
    }

    fn target(
        &mut self,
        owner: usize,
        reference: &ScopedHierarchicalReference,
    ) -> CompileResult<(usize, SmolStr)> {
        let span = reference.source.span;
        if reference.source.absolute {
            return Err(error(
                "absolute $root paths require selected-design occurrence binding",
                span,
            ));
        }
        let first = &reference.scopes[0];
        let mut scope = (0..=reference.origin.len())
            .rev()
            .map(|depth| reference.origin[..depth].to_vec())
            .find(|scope| {
                self.frames[owner].member(scope, &first.name).is_some()
                    || self.frames[owner].has_scope(scope, &first.name)
            })
            .ok_or_else(|| {
                error(
                    format!(
                        "scope `{}` is not declared in this module or its generated ancestors",
                        first.name
                    ),
                    span,
                )
            })?;
        let mut frame = owner;
        for key in &reference.scopes {
            if let Some(member) = self.frames[frame].member(&scope, &key.name) {
                let instance = self.frames[frame]
                    .instances
                    .get(&member)
                    .copied()
                    .ok_or_else(|| {
                        error(
                            format!("`{}` is not a module or generated scope", key.name),
                            span,
                        )
                    })?;
                if key.index.is_some() {
                    return Err(error(
                        "module-instance array selectors are not implemented",
                        span,
                    ));
                }
                frame = self.child(frame, instance, span)?;
                scope.clear();
            } else {
                scope.push(key.clone());
                let allowed = self.frames[frame]
                    .source
                    .hierarchical_scopes
                    .get(&scope)
                    .is_some_and(|declared| {
                        declared.explicit
                            || (frame == owner && reference.origin.starts_with(&scope))
                    });
                if !allowed {
                    return Err(error(
                        format!(
                            "generated scope `{}` with index {:?} is missing or not source-visible",
                            key.name, key.index
                        ),
                        span,
                    ));
                }
            }
        }
        let terminal = &reference
            .source
            .segments
            .last()
            .expect("parsed terminal")
            .name;
        let name = self.frames[frame].member(&scope, terminal).ok_or_else(|| {
            error(
                format!("`{terminal}` is not declared in the selected scope"),
                span,
            )
        })?;
        Ok((frame, name))
    }

    fn child(&mut self, parent: usize, ordinal: usize, span: Span) -> CompileResult<usize> {
        let instance = self.frames[parent].source.instances[ordinal].clone();
        let identity = (parent, instance.name.clone());
        if let Some(index) = self.children.get(&identity) {
            return Ok(*index);
        }
        // Overrides can read another occurrence's parameter. Resolve those reads
        // first and detect cycles rather than depending on declaration order.
        let mut references = std::collections::BTreeSet::new();
        for parameter in &instance.parameters {
            crate::ast::visit_expression(&parameter.value, &mut |expression| {
                let name = match expression {
                    Expression::Identifier(id) => Some(&id.name),
                    Expression::ArrayAccess(value) => Some(&value.array),
                    Expression::Digital(value) => value.base_name(),
                    _ => None,
                };
                if let Some(name) = name.filter(|name| {
                    self.frames[parent]
                        .source
                        .pending_hierarchical_references
                        .contains_key(*name)
                }) {
                    references.insert(name.clone());
                }
            });
        }
        for reference in references {
            self.reference(parent, &reference)?;
        }
        let source = self.sources.get(&instance.module).ok_or_else(|| {
            error(
                format!("module `{}` is not in the source closure", instance.module),
                span,
            )
        })?;
        let path: SmolStr = if self.frames[parent].path.is_empty() {
            instance.name.clone()
        } else {
            format!("{}.{}", self.frames[parent].path, instance.name).into()
        };
        super::digital_elaborate::check_hierarchy_capacity(
            self.frames[parent].depth + 1,
            self.frames.len(),
            &path,
            span,
        )?;
        let names = source
            .parameters
            .iter()
            .map(|parameter| parameter.name.clone())
            .collect::<Vec<_>>();
        let aliases = source
            .aliasparams
            .iter()
            .filter_map(|alias| {
                source
                    .parameters
                    .iter()
                    .position(|parameter| parameter.name == alias.target)
                    .map(|index| (alias.alias.clone(), index))
            })
            .collect();
        let mut overrides: Vec<_> =
            super::instance_parameters::bind_overrides(&instance, &names, &aliases, &path)?
                .into_iter()
                .collect();
        overrides.sort_by_key(|(index, _)| *index);
        let parent_source = &self.frames[parent].source;
        let constants = DigitalConstants::from_module(parent_source);
        let mut child = source.clone();
        for (index, expression) in overrides {
            let value = super::instance_parameters::close_override(
                &source.parameters[index],
                expression,
                &constants,
                parent_source.time_scale,
            )
            .map_err(|detail| error(format!("instance `{path}`: {detail}"), span))?;
            child.parameters[index].default = Some(value);
            child.parameters[index].is_given = true;
        }
        child.reference_sources = None;
        crate::parser::expand_specialized_generates(&mut child)?;
        let index = self.frames.len();
        self.frames
            .push(Frame::new(child, path, self.frames[parent].depth + 1));
        self.children.insert(identity, index);
        Ok(index)
    }
}
