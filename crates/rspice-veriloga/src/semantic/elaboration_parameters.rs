//! Parameter provenance across flattened analog instance scopes.
use super::*;
use crate::ast::{DigitalExpr, GenerateConstruct};
use crate::semantic::parameter_given;

pub(super) type SpecializationKey = (SmolStr, Vec<(usize, String)>);

pub(super) struct SpecializedModule {
    pub source: Module,
    pub analyzed: AnalyzedModule,
}

/// Reanalyze only subtrees that need final values for typing or structure.
/// Trees without elaboration-bound inputs keep their existing symbolic path.
pub(super) fn specialization_modules(
    sources: &HashMap<SmolStr, &Module>,
    analyzed: &AnalyzedFile,
) -> HashSet<SmolStr> {
    let mut parents: HashMap<SmolStr, Vec<SmolStr>> = HashMap::new();
    let mut pending = Vec::new();
    for source in sources.values() {
        if analyzed.modules.get(&source.name).is_some_and(|module| {
            module.parameters.iter().any(|parameter| {
                parameter.elaboration_value.is_some()
                    || parameter.elaboration_given.is_some()
                    || !parameter.dimensions.is_empty()
            })
        }) || source.generate_template.is_some()
            || source
                .parameters
                .iter()
                .chain(&source.localparams)
                .any(|parameter| !parameter.type_is_explicit)
        {
            pending.push(source.name.clone());
        }
        for instance in &source.instances {
            parents
                .entry(instance.module.clone())
                .or_default()
                .push(source.name.clone());
        }
    }
    let mut result = HashSet::new();
    while let Some(name) = pending.pop() {
        if result.insert(name.clone())
            && let Some(ancestors) = parents.get(&name)
        {
            pending.extend(ancestors.iter().cloned());
        }
    }
    result
}

impl HierarchyElaborator<'_> {
    pub(super) fn specialize_parameters(
        &mut self,
        source: &Module,
        child: &AnalyzedModule,
        parent: &Module,
        overrides: &HashMap<usize, Expression>,
        path: &str,
    ) -> CompileResult<Option<std::sync::Arc<SpecializedModule>>> {
        if overrides.is_empty() || !self.specialization_modules.contains(&source.name) {
            return Ok(None);
        }
        let constants = crate::semantic::instance_parameters::constants(parent);
        let mut indices: Vec<_> = overrides.keys().copied().collect();
        indices.sort_unstable();
        let mut values = Vec::new();
        let mut key = Vec::new();
        for index in indices {
            let declaration = &source.parameters[index];
            let expression = &overrides[&index];
            let value = crate::semantic::instance_parameters::close_override(
                declaration,
                expression.clone(),
                &constants,
                parent.time_scale,
            )
            .map_err(|message| {
                semantic_error(
                    SemanticErrorKind::UnsupportedFeature(format!(
                        "parameter '{}' of instance '{path}': {message}",
                        declaration.name
                    )),
                    expression.span(),
                )
            })?;
            let identity = super::super::instance_parameters::override_identity(&value)
                .map_err(internal_error)?;
            key.push((index, identity));
            values.push((index, value));
        }
        let key = (source.name.clone(), key);
        if let Some(specialized) = self.specializations.get(&key) {
            return Ok(Some(specialized.clone()));
        }
        let mut source = source.clone();
        for (index, value) in values {
            source.parameters[index].default = Some(value);
            source.parameters[index].is_given = true;
        }
        crate::parser::expand_specialized_generates(&mut source)?;
        let mut analyzer = SemanticAnalyzer::new();
        analyzer.disciplines = self.analyzed.disciplines.clone();
        analyzer.current_default_transition = child.default_transition;
        let mut analyzed = analyzer.analyze_module(&source, child.default_transition)?;
        analyzed.default_discipline = child.default_discipline.clone();
        let specialized = std::sync::Arc::new(SpecializedModule { source, analyzed });
        self.specializations.insert(key, specialized.clone());
        Ok(Some(specialized))
    }
}

#[derive(Default)]
pub(super) struct ParameterHierarchy {
    edges: HashMap<SmolStr, HashSet<SmolStr>>,
    values: HashMap<SmolStr, f64>,
    roots: HashSet<SmolStr>,
    given_edges: HashMap<SmolStr, HashSet<SmolStr>>,
    given_roots: HashSet<SmolStr>,
}

pub(super) fn is_packed(parameter: &AnalyzedParameter) -> bool {
    matches!(
        parameter.default_expr,
        Some(Expression::Digital(DigitalExpr::FourState(_)))
    )
}

impl ParameterHierarchy {
    pub(super) fn register(
        &mut self,
        source: &Module,
        analyzed: &AnalyzedModule,
        scope: &ScopeMap,
        parent: Option<(&Module, &ScopeMap, &HashMap<usize, Expression>)>,
    ) -> CompileResult<()> {
        let order = crate::semantic::parameter_defaults::declaration_order(source);
        let declarations: Vec<_> = order
            .iter()
            .map(|&(local, index)| {
                if local {
                    &source.localparams[index]
                } else {
                    &source.parameters[index]
                }
            })
            .collect();
        let assignments = crate::canonical_ir::digital_lower::parameter_assignments(
            &declarations,
            &parameter_given::GivenParameters::new(source),
            source.time_scale,
        )
        .map_err(|errors| {
            semantic_error(
                SemanticErrorKind::UnsupportedFeature(errors[0].diagnostic.message.clone()),
                source.span,
            )
        })?;
        for (declaration, assignment) in declarations.iter().zip(assignments) {
            let Some(name) = scope.parameters.get(&declaration.name) else {
                continue;
            };
            let value = assignment
                .value
                .as_ref()
                .and_then(|expression| match expression {
                    Expression::Number(number) => Some(number.value),
                    Expression::Digital(DigitalExpr::FourState(literal)) => {
                        crate::numeric_literal::parse_numeric_literal(&literal.value.raw)
                            .ok()
                            .and_then(|value| {
                                value.as_exact_f64("hierarchy elaboration dependency").ok()
                            })
                    }
                    _ => None,
                });
            if let Some(value) = value.filter(|value| value.is_finite()) {
                self.values.insert(name.clone(), value);
            }
        }
        let local_dependencies = SourceParameters::new(source);
        let parent_dependencies = parent.map(|(source, _, _)| SourceParameters::new(source));
        for (index, parameter) in analyzed.parameters.iter().enumerate() {
            let name = &scope.parameters[&parameter.name];
            let declaration = &source.parameters[index];
            let (dependencies, dependency_scope) = if let Some((_, parent_scope, overrides)) =
                parent
                && let Some(expression) = overrides.get(&index)
            {
                (
                    parent_dependencies
                        .as_ref()
                        .expect("parent scope")
                        .dependencies([expression])?,
                    parent_scope,
                )
            } else {
                let mut expressions: Vec<_> = declaration.default.iter().collect();
                if let Some(range) = &declaration.packed_range {
                    expressions.extend([&range.msb, &range.lsb]);
                }
                (local_dependencies.dependencies(expressions)?, scope)
            };
            self.edges.insert(
                name.clone(),
                dependencies
                    .values
                    .into_iter()
                    .filter_map(|name| dependency_scope.parameters.get(&name).cloned())
                    .collect(),
            );
            self.given_edges.insert(
                name.clone(),
                dependencies
                    .given
                    .into_iter()
                    .filter_map(|name| dependency_scope.parameters.get(&name).cloned())
                    .collect(),
            );
            if is_packed(parameter)
                || parameter.elaboration_value.is_some()
                || (!parameter.dimensions.is_empty()
                    && parent.is_some_and(|(_, _, overrides)| overrides.contains_key(&index)))
            {
                self.roots.insert(name.clone());
            }
        }
        // Exact locals have no numeric slot, but their public dependencies do.
        for local in &analyzed.digital.elaboration_parameters {
            let expression = Expression::Identifier(Identifier {
                name: local.name.clone(),
                span: source.span,
            });
            let dependencies = local_dependencies.dependencies([&expression])?;
            self.roots.extend(
                dependencies
                    .values
                    .into_iter()
                    .filter_map(|name| scope.parameters.get(&name).cloned()),
            );
            self.given_roots.extend(
                dependencies
                    .given
                    .into_iter()
                    .filter_map(|name| scope.parameters.get(&name).cloned()),
            );
        }
        // Re-expanded generate structure is also immutable in a compiled device.
        if let Some(template) = &source.generate_template {
            let dependencies =
                local_dependencies.dependencies(generate_controls(&template.module.generates))?;
            self.roots.extend(
                dependencies
                    .values
                    .into_iter()
                    .filter_map(|name| scope.parameters.get(&name).cloned()),
            );
            self.given_roots.extend(
                dependencies
                    .given
                    .into_iter()
                    .filter_map(|name| scope.parameters.get(&name).cloned()),
            );
        }
        Ok(())
    }

    pub(super) fn protect(&self, module: &mut AnalyzedModule, span: Span) -> CompileResult<()> {
        let mut pending: Vec<_> = self.roots.iter().cloned().collect();
        let mut required = HashSet::new();
        let mut given_required = self.given_roots.clone();
        while let Some(name) = pending.pop() {
            if !required.insert(name.clone()) {
                continue;
            }
            if let Some(reads) = self.edges.get(&name) {
                pending.extend(reads.iter().cloned());
            }
            if let Some(reads) = self.given_edges.get(&name) {
                given_required.extend(reads.iter().cloned());
            }
        }
        for parameter in &mut module.parameters {
            if parameter.is_public && given_required.contains(&parameter.name) {
                parameter.elaboration_given = Some(parameter.is_given);
            }
            if !required.contains(&parameter.name)
                || is_packed(parameter)
                || !parameter.dimensions.is_empty()
            {
                continue;
            }
            let value = self.values.get(&parameter.name).copied()
                .or(parameter.elaboration_value)
                .ok_or_else(|| semantic_error(SemanticErrorKind::UnsupportedFeature(format!(
                    "hierarchical elaboration dependency '{}' requires an exact numeric value", parameter.name
                )), span))?;
            parameter.elaboration_value = Some(value);
        }
        Ok(())
    }
}

/// Public inputs read by an expression, expanding local dependencies only.
/// Public-to-public edges remain separate so instance overrides replace them.
#[derive(Default)]
struct ParameterDependencies {
    values: HashSet<SmolStr>,
    given: HashSet<SmolStr>,
}

struct SourceParameters<'a> {
    given: parameter_given::GivenParameters,
    public: HashSet<&'a SmolStr>,
    locals: HashMap<&'a SmolStr, &'a crate::ast::ParameterDecl>,
}

impl<'a> SourceParameters<'a> {
    fn new(source: &'a Module) -> Self {
        Self {
            given: parameter_given::GivenParameters::new(source),
            public: source.parameters.iter().map(|value| &value.name).collect(),
            locals: source
                .localparams
                .iter()
                .map(|value| (&value.name, value))
                .collect(),
        }
    }

    fn dependencies<'b>(
        &'b self,
        expressions: impl IntoIterator<Item = &'b Expression>,
    ) -> CompileResult<ParameterDependencies> {
        let mut pending: Vec<_> = expressions.into_iter().collect();
        let mut visited = HashSet::new();
        let mut result = ParameterDependencies::default();
        while let Some(expression) = pending.pop() {
            let (folded, given) = self.given.fold(expression).map_err(|message| {
                SemanticError::new(
                    SemanticErrorKind::InvalidExpression(message),
                    expression.span(),
                )
            })?;
            result.given.extend(given);
            crate::semantic::flow_probes::visit_expression(&folded, &mut |expression| {
                let name = match expression {
                    Expression::Identifier(value) => Some(&value.name),
                    Expression::ArrayAccess(value) => Some(&value.array),
                    Expression::Digital(value) => value.base_name(),
                    _ => None,
                };
                let Some(name) = name else { return };
                if self.public.contains(name) {
                    result.values.insert(name.clone());
                } else if visited.insert(name.clone())
                    && let Some(local) = self.locals.get(name)
                {
                    pending.extend(local.default.iter());
                    if let Some(range) = &local.packed_range {
                        pending.extend([&range.msb, &range.lsb]);
                    }
                }
            });
        }
        Ok(result)
    }
}

fn generate_controls(constructs: &[GenerateConstruct]) -> Vec<&Expression> {
    let mut pending: Vec<_> = constructs.iter().collect();
    let mut result = Vec::new();
    while let Some(construct) = pending.pop() {
        let mut blocks = Vec::new();
        match construct {
            GenerateConstruct::Loop(value) => {
                result.extend([&value.init, &value.condition, &value.update]);
                blocks.push(&value.body);
            }
            GenerateConstruct::Conditional(value) => {
                result.push(&value.condition);
                blocks.push(&value.then_block);
                blocks.extend(value.else_block.iter());
            }
            GenerateConstruct::Case(value) => {
                result.push(&value.selector);
                for item in &value.items {
                    result.extend(&item.labels);
                    blocks.push(&item.block);
                }
                blocks.extend(value.default.iter());
            }
            GenerateConstruct::Block(value) => blocks.push(value),
        }
        for block in blocks {
            pending.extend(&block.nested);
            pending.extend(&block.items.generates);
        }
    }
    result
}
