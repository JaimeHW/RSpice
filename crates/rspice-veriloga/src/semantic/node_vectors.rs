//! Physical vector declarations and the scalar lanes shared by both domains.
mod arrays;
mod branches;
mod connections;
mod continuous_nets;
mod real_buses;
use super::*;
pub(super) use connections::{ConnectionScope, bind as bind_connections};
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub(crate) struct NodeVector {
    pub bounds: VectorBounds,
    /// Source left to right, independent of ascending or descending coordinates.
    pub lanes: Vec<SmolStr>,
}

impl NodeVector {
    pub fn lane(&self, coordinate: i64, span: Span) -> CompileResult<SmolStr> {
        if !self.bounds.contains(coordinate) {
            return Err(error(
                format!(
                    "physical node index {coordinate} is outside [{}:{}]",
                    self.bounds.msb, self.bounds.lsb
                ),
                span,
            ));
        }
        Ok(self.lanes[self.bounds.msb.abs_diff(coordinate) as usize].clone())
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct PhysicalNodes {
    pub arrays: HashMap<SmolStr, arrays::NodeArray>,
    pub real_buses: HashMap<SmolStr, real_buses::RealBus>,
    pub real_aliases: HashMap<SmolStr, DigitalElementAlias>,
    pub real_input_buses: HashSet<SmolStr>,
    pub vectors: HashMap<SmolStr, NodeVector>,
    pub branches: HashMap<SmolStr, NodeVector>,
    /// Scalar aliases resolve to the original local port before hierarchy collapse.
    pub port_branches: HashMap<SmolStr, SmolStr>,
    /// One group per authored formal port, including unchanged discrete ports.
    pub ports: Vec<(SmolStr, Vec<SmolStr>)>,
}

pub(super) fn error(message: impl Into<String>, span: Span) -> CompileError {
    SemanticError::new(SemanticErrorKind::InvalidExpression(message.into()), span).into()
}

pub(super) fn integer(
    expr: &Expression,
    constants: &DigitalConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
) -> CompileResult<i64> {
    match crate::canonical_ir::digital_lower::elaboration_constant(expr, constants, time_scale) {
        Some(crate::numeric_literal::NumericLiteralValue::Integer(value)) => Ok(value),
        _ => Err(error(
            "a physical vector bound or selector must resolve to an integer at elaboration",
            expr.span(),
        )),
    }
}

fn bounds(
    range: &VectorRange,
    constants: &DigitalConstants,
    time_scale: crate::time_scale::ModuleTimeScale,
) -> CompileResult<VectorBounds> {
    let result = VectorBounds {
        msb: integer(&range.msb, constants, time_scale)?,
        lsb: integer(&range.lsb, constants, time_scale)?,
    };
    if result.width() > MAX_DIGITAL_VECTOR_WIDTH {
        return Err(error(
            format!("physical vector width exceeds {MAX_DIGITAL_VECTOR_WIDTH} lanes"),
            range.span,
        ));
    }
    Ok(result)
}

/// Expand declarations only. The retained source still owns authored parameter
/// expressions, so every specialization recomputes its complete physical shape.
pub(super) fn declarations<'a>(
    module: &'a Module,
    db: &DisciplineDb,
) -> CompileResult<(Cow<'a, Module>, PhysicalNodes)> {
    let normalized = continuous_nets::normalize(module, db)?;
    let module = normalized.as_ref();
    let constants = DigitalConstants::from_module(module);
    let mut physical = HashSet::new();
    let continuous = |name: &SmolStr| {
        db.get_discipline(name)
            .is_some_and(|discipline| discipline.domain == Domain::Continuous)
    };
    for net in &module.nets {
        if net.discipline.as_ref().is_some_and(continuous)
            || (net.is_ground && (net.range.is_some() || !net.dimensions.is_empty()))
        {
            physical.extend(net.names.iter().cloned());
        }
    }
    for port in &module.port_declarations {
        if port.discipline.as_ref().is_some_and(continuous) {
            physical.extend(port.names.iter().cloned());
        }
    }
    let array_names: HashSet<_> = module
        .nets
        .iter()
        .flat_map(|net| &net.dimensions)
        .map(|(name, _)| name)
        .filter(|name| physical.contains(*name))
        .cloned()
        .collect();
    let mut ranges: BTreeMap<SmolStr, Option<VectorBounds>> = BTreeMap::new();
    for (names, range, span) in module
        .port_declarations
        .iter()
        .map(|decl| (&decl.names, decl.range.as_ref(), decl.span))
        .chain(
            module
                .nets
                .iter()
                .filter(|net| !net.is_ground || net.range.is_some())
                .map(|decl| (&decl.names, decl.range.as_ref(), decl.span)),
        )
    {
        for name in names
            .iter()
            .filter(|name| physical.contains(*name) && !array_names.contains(*name))
        {
            let resolved = range
                .map(|range| bounds(range, &constants, module.time_scale))
                .transpose()?;
            if let Some(previous) = ranges.insert(name.clone(), resolved) {
                if previous != resolved {
                    return Err(error(
                        format!(
                            "physical port/net '{name}' has inconsistent vector ranges; direction and discipline declarations must have identical bounds"
                        ),
                        span,
                    ));
                }
            }
        }
    }
    let mut nodes = PhysicalNodes::default();
    let mut used = super::hierarchy_connections::declared_names(module);
    let mut count = 0_u64;
    for (name, range) in ranges {
        let Some(range) = range else { continue };
        count += u64::from(range.width());
        if count > MAX_PARAMETER_ARRAY_ELEMENTS {
            return Err(error(
                "physical vector declarations exceed the module lane limit",
                module.span,
            ));
        }
        let mut lanes = Vec::with_capacity(range.width() as usize);
        for index in range.indices_msb_first() {
            let mut lane: SmolStr = format!("{name}[{index}]").into();
            // Escaped authored names and other generated declarations remain
            // distinct from a vector coordinate with the same printed spelling.
            while !used.insert(lane.clone()) {
                lane = format!("{lane}_").into();
            }
            lanes.push(lane);
        }
        nodes.vectors.insert(
            name,
            NodeVector {
                bounds: range,
                lanes,
            },
        );
    }
    let names = |name: &SmolStr| {
        nodes
            .vectors
            .get(name)
            .map_or_else(|| vec![name.clone()], |vector| vector.lanes.clone())
    };
    nodes.ports = module
        .ports
        .iter()
        .map(|port| (port.name.clone(), names(&port.name)))
        .collect();
    if nodes.vectors.is_empty()
        && array_names.is_empty()
        && module.branches.is_empty()
        && !module
            .digital_nets
            .iter()
            .any(|net| net.kind.is_real() && net.range.is_some())
    {
        return Ok((normalized, nodes));
    }
    let mut expanded = module.clone();
    expanded.ports = module
        .ports
        .iter()
        .flat_map(|port| {
            names(&port.name).into_iter().map(|name| Port {
                name,
                span: port.span,
            })
        })
        .collect();
    expanded.port_declarations = module
        .port_declarations
        .iter()
        .flat_map(|declaration| {
            declaration.names.iter().map(|name| {
                let mut expanded = declaration.clone();
                expanded.names = names(name);
                expanded.initializers.retain(|(target, _)| target == name);
                if nodes.vectors.contains_key(name) {
                    expanded.range = None;
                }
                expanded
            })
        })
        .collect();
    for declaration in &mut expanded.nets {
        if declaration
            .names
            .iter()
            .any(|name| nodes.vectors.contains_key(name))
        {
            declaration.names = declaration.names.iter().flat_map(names).collect();
            declaration.range = None;
        }
    }
    count = arrays::expand(
        module,
        &mut expanded,
        &mut nodes,
        &constants,
        &mut used,
        db,
        count,
    )?;
    branches::expand(
        module,
        &mut expanded,
        &mut nodes,
        &constants,
        &mut used,
        count,
    )?;
    real_buses::expand(module, &mut expanded, &mut nodes, &constants, &mut used)?;
    Ok((Cow::Owned(expanded), nodes))
}

impl SemanticAnalyzer {
    /// Resolve after analog loop substitutions, before any backend or branch
    /// inventory sees the probe. A dynamic coordinate cannot select topology.
    pub(super) fn resolve_vector_access(
        &self,
        access: &BranchAccess,
    ) -> CompileResult<BranchAccess> {
        let mut resolved = access.clone();
        match &mut resolved {
            BranchAccess::Nodes {
                pos,
                neg,
                pos_indices,
                neg_indices,
                span,
                ..
            } => {
                self.resolve_physical_operand(pos, pos_indices, neg.is_none(), *span)?;
                if let Some(neg) = neg {
                    self.resolve_physical_operand(neg, neg_indices, false, *span)?;
                }
            }
            BranchAccess::Branch {
                name, index, span, ..
            } => {
                let mut indices: Vec<_> = index.take().into_iter().map(|value| *value).collect();
                self.resolve_physical_operand(name, &mut indices, true, *span)?;
            }
        }
        let alias = match &resolved {
            BranchAccess::Nodes {
                access,
                pos,
                neg: None,
                span,
                ..
            } => Some((access, pos, span)),
            BranchAccess::Branch {
                access, name, span, ..
            } => Some((access, name, span)),
            _ => None,
        };
        if let Some((access, name, span)) = alias
            && let Some(port) = self.physical_nodes.port_branches.get(name)
        {
            return Ok(BranchAccess::Branch {
                access: access.clone(),
                kind: None,
                name: port.clone(),
                index: None,
                span: *span,
            });
        }
        Ok(resolved)
    }

    fn resolve_physical_operand(
        &self,
        name: &mut SmolStr,
        indices: &mut Vec<Expression>,
        allow_branch: bool,
        span: Span,
    ) -> CompileResult<()> {
        // Check the authored base name before scalar normalization can hide it.
        if self
            .digital_scopes
            .iter()
            .any(|scope| scope.iter().any(|local| &local.name == name))
        {
            return Err(error(
                format!(
                    "analog access names process-local storage '{name}', not a continuous net or branch"
                ),
                span,
            ));
        }
        let vector = self.physical_nodes.vectors.get(name).or_else(|| {
            allow_branch
                .then(|| self.physical_nodes.branches.get(name))
                .flatten()
        });
        let array = self.physical_nodes.arrays.get(name);
        if (vector.is_some()
            || array.is_some()
            || self.physical_nodes.port_branches.contains_key(name))
            && self.symbols.lookup(name).is_some_and(|symbol| {
                !matches!(
                    symbol.kind,
                    SymbolKind::Port | SymbolKind::Node | SymbolKind::Branch
                )
            })
        {
            return Err(error(
                format!("analog access '{name}' is shadowed by a nonphysical declaration"),
                span,
            ));
        }
        if !indices.is_empty() {
            let mut coordinates = Vec::with_capacity(indices.len());
            for index in std::mem::take(indices) {
                let index = self.substitute_physical_selector(&index);
                self.check_physical_selector_scope(&index)?;
                coordinates.push(integer(
                    &index,
                    &self.digital_selector_constants,
                    self.current_time_scale,
                )?);
                self.physical_selectors.borrow_mut().push(index);
            }
            *name = if let Some(array) = array {
                array.lane(&coordinates, span)?
            } else if let Some(vector) = vector {
                if coordinates.len() != 1 {
                    return Err(error(
                        "physical vector access requires exactly one coordinate",
                        span,
                    ));
                }
                vector.lane(coordinates[0], span)?
            } else {
                return Err(error(
                    format!("'{name}' is not a physical vector node or branch"),
                    span,
                ));
            };
        } else if vector.is_some() || array.is_some() {
            return Err(error(
                format!(
                    "physical vector or array '{name}' requires a scalar coordinate for every dimension in an access function"
                ),
                span,
            ));
        }
        Ok(())
    }

    pub(super) fn physical_access_call(&self, call: &CallExpr) -> Option<BranchAccess> {
        if self.user_functions.contains_key(&call.name)
            || self.disciplines.resolve_access(&call.name).is_none()
            || !matches!(call.args.len(), 1 | 2)
        {
            return None;
        }
        let operand = |expression: &Expression| match expression {
            Expression::Identifier(id) => Some((id.name.clone(), Vec::new())),
            Expression::ArrayAccess(access) => {
                Some((access.array.clone(), vec![*access.index.clone()]))
            }
            Expression::Digital(DigitalExpr::ArraySelect(access)) => {
                let PackedSelect::Bit(last) = &access.select else {
                    return None;
                };
                let mut indices = vec![*access.index.clone()];
                indices.extend(access.additional_indices.clone());
                indices.push(*last.clone());
                Some((access.name.clone(), indices))
            }
            Expression::Number(number) if number.value == 0.0 => Some(("0".into(), Vec::new())),
            _ => None,
        };
        let (pos, pos_indices) = operand(&call.args[0])?;
        let (neg, neg_indices) = if let Some(value) = call.args.get(1) {
            let (name, index) = operand(value)?;
            (Some(name), index)
        } else {
            (None, Vec::new())
        };
        Some(BranchAccess::Nodes {
            access: call.name.clone(),
            kind: None,
            pos,
            neg,
            pos_indices,
            neg_indices,
            span: call.span,
        })
    }

    /// Scalar updates cannot change node topology or a statically bound probe.
    /// Retain only actual parameter dependencies, including localparam chains
    /// and $param_given, so unrelated model parameters remain adjustable.
    pub(super) fn protect_physical_parameters(
        &self,
        source: &Module,
        module: &mut AnalyzedModule,
    ) -> CompileResult<()> {
        let selectors = self.physical_selectors.borrow();
        let mut expressions: Vec<_> = source
            .nets
            .iter()
            .filter_map(|net| net.range.as_ref())
            .chain(
                source
                    .port_declarations
                    .iter()
                    .filter_map(|port| port.range.as_ref()),
            )
            .flat_map(|range| [&range.msb, &range.lsb])
            .chain(selectors.iter())
            .collect();
        for net in &source.nets {
            for (_, dimensions) in &net.dimensions {
                for axis in dimensions {
                    expressions.extend([&axis.start, &axis.end]);
                }
            }
        }
        for branch in &source.branches {
            expressions.extend(branch.pos_prefix.iter().chain(&branch.neg_prefix));
            if let Some(range) = &branch.range {
                expressions.extend([&range.msb, &range.lsb]);
            }
            for select in [&branch.pos_select, &branch.neg_select]
                .into_iter()
                .flatten()
            {
                match select {
                    PackedSelect::Bit(index) => expressions.push(index),
                    PackedSelect::Part { msb, lsb } => {
                        expressions.extend([msb.as_ref(), lsb.as_ref()])
                    }
                }
            }
        }
        expressions.extend(
            source
                .instances
                .iter()
                .flat_map(|instance| &instance.connections)
                .filter_map(|connection| match connection {
                    Connection::Named { signal, .. } | Connection::Ordered { signal, .. } => {
                        signal.as_ref()
                    }
                }),
        );
        if expressions.is_empty() {
            return Ok(());
        }
        let dependencies = crate::canonical_ir::digital_lower::expression_dependencies(
            &self.digital_selector_constants,
            source.time_scale,
            expressions,
        )
        .map_err(|message| error(message, source.span))?;
        constant_dependencies::protect(module, &dependencies, source.span)?;
        Ok(())
    }

    fn check_physical_selector_scope(&self, expression: &Expression) -> CompileResult<()> {
        let mut expression = expression.clone();
        let mut pending = vec![&mut expression];
        while let Some(expression) = pending.pop() {
            if matches!(expression, Expression::SystemFunction(function)
                if function.name.eq_ignore_ascii_case("$param_given") || function.name.eq_ignore_ascii_case("param_given"))
            {
                continue;
            }
            let name = match &*expression {
                Expression::Identifier(id) => Some(&id.name),
                Expression::ArrayAccess(access) => Some(&access.array),
                Expression::Digital(digital) => digital.base_name(),
                _ => None,
            };
            if let Some(name) = name {
                if self
                    .digital_scopes
                    .iter()
                    .any(|scope| scope.iter().any(|local| &local.name == name))
                {
                    return Err(error(
                        format!(
                            "physical node selector '{name}' names process-local storage, not an elaboration constant"
                        ),
                        expression.span(),
                    ));
                }
            }
            flow_probes::for_child_mut(expression, &mut |child| pending.push(child));
        }
        Ok(())
    }

    fn substitute_physical_selector(&self, expression: &Expression) -> Expression {
        let mut expression = expression.clone();
        let mut pending = vec![&mut expression];
        while let Some(expression) = pending.pop() {
            if let Expression::Identifier(id) = expression {
                if let Some(substitution) = self.lookup_substitution(&id.name) {
                    *expression = substitution;
                }
            }
            flow_probes::for_child_mut(expression, &mut |child| pending.push(child));
        }
        expression
    }

    /// An analog genvar loop creates topology. Its controls use the instance's
    /// exact parameter values and never allocate a runtime counter variable.
    pub(super) fn unroll_analog_genvar(
        &mut self,
        statement: &ForStmt,
        module: &mut AnalyzedModule,
        sink: &mut Vec<AnalyzedStatement>,
    ) -> CompileResult<()> {
        if *statement.update.target_name() != statement.var {
            return Err(error(
                "analog genvar update must assign its own loop variable",
                statement.span,
            ));
        }
        if self.lookup_substitution(&statement.var).is_some() {
            return Err(error(
                "an analog genvar cannot be reused by an active nested loop",
                statement.span,
            ));
        }
        if function_effects::may_write_variable(
            &statement.body,
            &statement.var,
            &self.user_functions,
        ) {
            return Err(error(
                "an analog genvar cannot be written in the loop body",
                statement.span,
            ));
        }
        let evaluate = |analyzer: &Self, expression: &Expression| -> CompileResult<i64> {
            let expression = analyzer.substitute_physical_selector(expression);
            let value = integer(
                &expression,
                &analyzer.digital_selector_constants,
                analyzer.current_time_scale,
            )?;
            analyzer.physical_selectors.borrow_mut().push(expression);
            Ok(value)
        };
        let mut value = evaluate(self, &statement.init)?;
        for iteration in 0..=Self::MAX_UNROLL_ITERATIONS {
            let value32 = i32::try_from(value).map_err(|_| {
                error(
                    "analog genvar value must fit a signed 32-bit integer",
                    statement.span,
                )
            })?;
            self.subst_stack.push(HashMap::from([(
                statement.var.clone(),
                exact_integer_expression(i64::from(value32), statement.span),
            )]));
            let step = (|| -> CompileResult<Option<i64>> {
                if evaluate(self, &statement.condition)? == 0 {
                    return Ok(None);
                }
                if iteration == Self::MAX_UNROLL_ITERATIONS {
                    return Err(error(
                        "analog genvar loop exceeds the unroll limit",
                        statement.span,
                    ));
                }
                self.analog_genvar_iterations += 1;
                if self.analog_genvar_iterations > Self::MAX_UNROLL_ITERATIONS {
                    return Err(error(
                        "analog genvar expansion exceeds the module iteration limit",
                        statement.span,
                    ));
                }
                self.analyze_statement(&statement.body, module, sink)?;
                Ok(Some(evaluate(self, &statement.update.value)?))
            })();
            self.subst_stack.pop();
            match step? {
                None => return Ok(()),
                Some(next) => value = next,
            }
        }
        unreachable!("bounded loop returns on exhaustion")
    }
}
