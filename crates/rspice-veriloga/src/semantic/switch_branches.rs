//! Ordered source retention for branches with both potential and flow writes.
//!
//! Keep one mathematical current unknown. Its constraint is V - retained_V
//! in potential mode and I - retained_I in flow mode. Each authored RHS still
//! executes at its own site; changing source kind resets the retained value.

use super::*;
use std::collections::BTreeMap;

#[derive(Clone)]
pub(super) struct SwitchState {
    value: (SmolStr, usize),
    kind: (SmolStr, usize),
    span: Span,
}

impl SwitchState {
    pub(super) fn allocate(module: &mut AnalyzedModule, node: &str, span: Span) -> Self {
        let mut suffix = 0;
        let names = loop {
            let names =
                ["value", "kind"].map(|role| SmolStr::from(format!("{node}_{role}{suffix}")));
            if names.iter().all(|name| {
                module.symbol_table.lookup(name).is_none()
                    && !module.variables.iter().any(|v| v.name == *name)
                    && !module.internal_nodes.iter().any(|v| v.name == *name)
                    && !module.ports.iter().any(|v| v.name == *name)
                    && !module.parameters.iter().any(|v| v.name == *name)
                    && !module.branches.iter().any(|v| v.name == *name)
            }) {
                break names;
            }
            suffix += 1;
        };
        let [value, kind] = names.map(|name| {
            let index = module.variables.len();
            module.variables.push(AnalyzedVariable {
                name: name.clone(),
                var_type: VarType::Real,
                value_type: ValueType::Real,
                is_state: false,
            });
            (name, index)
        });
        module.variables[kind.1].is_state = true;
        module.event_state_variables.push(kind.1);
        module.switch_branch_variables.push(kind.1);
        Self { value, kind, span }
    }

    pub(super) fn balance(&self, current: Expression, voltage: Expression) -> Expression {
        SemanticAnalyzer::binary_expr(
            BinaryOp::Sub,
            select(read(&self.kind, self.span), voltage, current, self.span),
            read(&self.value, self.span),
        )
    }

    fn steps(
        &self,
        contribution: &AnalyzedContribution,
        sign: f64,
        flat: bool,
    ) -> CompileResult<Vec<AnalyzedAssignment>> {
        let span = contribution.span;
        let kind = SemanticAnalyzer::number_expr(f64::from(!contribution.is_current), span);
        let same_kind =
            SemanticAnalyzer::binary_expr(BinaryOp::Eq, read(&self.kind, span), kind.clone());
        // A kind change discards the old numerical contribution, but never
        // the evaluation and domain checks of an executed source statement.
        let retained = SemanticAnalyzer::binary_expr(
            BinaryOp::CheckedValue,
            read(&self.value, span),
            select(
                same_kind,
                read(&self.value, span),
                SemanticAnalyzer::number_expr(0.0, span),
                span,
            ),
        );
        let (rhs, guard) = match (
            flat,
            contribution.expression_guard,
            &contribution.expression,
        ) {
            (true, AnalogSiteGuard::Select, Expression::Conditional(expression)) => (
                (*expression.then_expr).clone(),
                Some((*expression.condition).clone()),
            ),
            (true, AnalogSiteGuard::Select | AnalogSiteGuard::Conjunction, _) => {
                return Err(CompileError::Semantic(SemanticError::new(
                    SemanticErrorKind::InvalidAnalogOperator(
                        "invalid switch-branch contribution guard".into(),
                    ),
                    span,
                )));
            }
            _ => (contribution.expression.clone(), None),
        };
        let value =
            SemanticAnalyzer::binary_expr(BinaryOp::Add, retained, flow_probes::signed(rhs, sign));
        Ok([&self.value, &self.kind]
            .into_iter()
            .zip([value, kind])
            .enumerate()
            .map(|(offset, (variable, value))| {
                let expression = if let Some(guard) = &guard {
                    select(guard.clone(), value, read(variable, span), span)
                } else {
                    value
                };
                assignment(
                    variable,
                    expression,
                    AnalogSiteId(contribution.site.0 + offset as u32),
                    contribution.expression_guard,
                    span,
                )
            })
            .collect())
    }
}

fn read(variable: &(SmolStr, usize), span: Span) -> Expression {
    Expression::Identifier(Identifier {
        name: variable.0.clone(),
        span,
    })
}

fn select(condition: Expression, yes: Expression, no: Expression, span: Span) -> Expression {
    Expression::Conditional(ConditionalExpr {
        condition: Box::new(condition),
        then_expr: Box::new(yes),
        else_expr: Box::new(no),
        span,
    })
}

fn assignment(
    variable: &(SmolStr, usize),
    expression: Expression,
    site: AnalogSiteId,
    expression_guard: AnalogSiteGuard,
    span: Span,
) -> AnalyzedAssignment {
    AnalyzedAssignment {
        target: variable.0.clone(),
        var_index: variable.1,
        index: None,
        expression,
        site,
        expression_guard,
        expr_type: ValueType::Real,
        span,
        unfiltered_initial_step_guard: None,
    }
}

pub(super) fn lower(
    module: &mut AnalyzedModule,
    sites: &BTreeMap<AnalogSiteId, (&SwitchState, f64)>,
) -> CompileResult<()> {
    if sites.is_empty() {
        return Ok(());
    }
    let states: BTreeMap<_, _> = sites
        .values()
        .map(|(state, _)| (state.value.1, *state))
        .collect();
    // Reserve two IDs at every old site so paired flat/structured expressions
    // retain execution order after a contribution becomes two assignments.
    let prefix = u32::try_from(states.len())
        .ok()
        .and_then(|n| n.checked_mul(2));
    let count =
        prefix.and_then(|prefix| module.analog_site_count.checked_mul(2)?.checked_add(prefix));
    let (Some(prefix), Some(count)) = (prefix, count) else {
        return Err(CompileError::Semantic(SemanticError::new(
            SemanticErrorKind::UnsupportedFeature("too many analog equation sites".into()),
            Span::dummy(),
        )));
    };
    let map = |id: AnalogSiteId| AnalogSiteId(prefix + 2 * id.0);
    let mut pending = BTreeMap::new();
    let old_contributions = std::mem::take(&mut module.contributions);
    for mut contribution in old_contributions {
        let switch = sites.get(&contribution.site);
        contribution.site = map(contribution.site);
        if let Some((state, sign)) = switch {
            pending.insert(contribution.site, state.steps(&contribution, *sign, true)?);
        } else {
            module.contributions.push(contribution);
        }
    }
    remap_statements(&mut module.statements, &map);
    let mut statements = Vec::new();
    let mut prologue = Vec::new();
    for (index, statement) in std::mem::take(&mut module.statements)
        .into_iter()
        .enumerate()
    {
        let site = statement_site(&statement);
        while pending
            .first_key_value()
            .is_some_and(|(next, _)| *next < site)
        {
            statements.extend(
                pending
                    .pop_first()
                    .unwrap()
                    .1
                    .into_iter()
                    .map(AnalyzedStatement::Assignment),
            );
        }
        if module.prologue_statements.contains(&index) {
            prologue.push(statements.len() + prefix as usize);
        }
        statements.push(statement);
    }
    for assignments in pending.into_values() {
        statements.extend(assignments.into_iter().map(AnalyzedStatement::Assignment));
    }
    module.statements = statements;
    module.prologue_statements = prologue;
    rewrite_regions(&mut module.body, sites, &map)?;
    let mut resets = Vec::new();
    for state in states.into_values() {
        for variable in [&state.value, &state.kind] {
            resets.push(assignment(
                variable,
                SemanticAnalyzer::number_expr(0.0, state.span),
                AnalogSiteId(resets.len() as u32),
                AnalogSiteGuard::None,
                state.span,
            ));
        }
    }
    module.statements.splice(
        0..0,
        resets.iter().cloned().map(AnalyzedStatement::Assignment),
    );
    module
        .body
        .splice(0..0, resets.into_iter().map(AnalyzedRegion::Assignment));
    module.analog_site_count = count;
    Ok(())
}

fn statement_site(statement: &AnalyzedStatement) -> AnalogSiteId {
    match statement {
        AnalyzedStatement::Assignment(value) => value.site,
        AnalyzedStatement::Loop(value) => value.site,
        AnalyzedStatement::Task(value) => AnalogSiteId(value.site),
        AnalyzedStatement::Initialization { site, .. } => *site,
    }
}

fn remap_statements(
    statements: &mut [AnalyzedStatement],
    map: &impl Fn(AnalogSiteId) -> AnalogSiteId,
) {
    for statement in statements {
        match statement {
            AnalyzedStatement::Assignment(value) => value.site = map(value.site),
            AnalyzedStatement::Task(value) => value.site = map(AnalogSiteId(value.site)).0,
            AnalyzedStatement::Loop(value) => {
                value.site = map(value.site);
                remap_statements(&mut value.body, map);
            }
            AnalyzedStatement::Initialization { site, body, .. } => {
                *site = map(*site);
                remap_statements(body, map);
            }
        }
    }
}

fn rewrite_regions(
    body: &mut Vec<AnalyzedRegion>,
    sites: &BTreeMap<AnalogSiteId, (&SwitchState, f64)>,
    map: &impl Fn(AnalogSiteId) -> AnalogSiteId,
) -> CompileResult<()> {
    let mut rewritten = Vec::new();
    for mut region in std::mem::take(body) {
        match &mut region {
            AnalyzedRegion::Contribution(value) => {
                let switch = sites.get(&value.site);
                value.site = map(value.site);
                if let Some((state, sign)) = switch {
                    rewritten.extend(
                        state
                            .steps(value, *sign, false)?
                            .into_iter()
                            .map(AnalyzedRegion::Assignment),
                    );
                    continue;
                }
            }
            AnalyzedRegion::Assignment(value) => value.site = map(value.site),
            AnalyzedRegion::Task(value) => value.site = map(AnalogSiteId(value.site)).0,
            AnalyzedRegion::Conditional {
                condition_site,
                then_body,
                else_body,
                ..
            } => {
                *condition_site = condition_site.map(map);
                rewrite_regions(then_body, sites, map)?;
                rewrite_regions(else_body, sites, map)?;
            }
            AnalyzedRegion::Loop { site, body, .. } => {
                *site = map(*site);
                rewrite_regions(body, sites, map)?;
            }
            AnalyzedRegion::Initialization { body, .. } => rewrite_regions(body, sites, map)?,
        }
        rewritten.push(region);
    }
    *body = rewritten;
    Ok(())
}
