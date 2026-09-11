//! Resolve flow probes through simultaneous branch equations.
//!
//! A probed flow source owns one private mathematical unknown. Its authored
//! contributions form the equation `i - sum(f) = 0`; a single physical source
//! injects `i`. This preserves source sites and stateful operators while AD,
//! AC and noise use the same solver dependency as ordinary voltage probes.

use super::*;
use crate::branch_identity::BranchIdentity;
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};

type BranchKey = BranchIdentity<SmolStr>;

struct BranchResolver<'a> {
    declared: &'a [AnalyzedBranch],
    grounds: &'a [SmolStr],
}

impl BranchResolver<'_> {
    fn ground(&self, name: &str) -> SmolStr {
        if name.is_empty()
            || is_global_ground_name(name)
            || self.grounds.iter().any(|ground| ground == name)
        {
            "0".into()
        } else {
            name.into()
        }
    }

    fn nodes(&self, pos: &str, neg: &str) -> (BranchKey, SmolStr, SmolStr, f64) {
        let pos = self.ground(pos);
        let neg = self.ground(neg);
        if pos <= neg {
            (BranchKey::Nodes(pos.clone(), neg.clone()), pos, neg, 1.0)
        } else {
            (BranchKey::Nodes(neg.clone(), pos.clone()), neg, pos, -1.0)
        }
    }

    fn named(&self, name: &SmolStr) -> Option<(BranchKey, SmolStr, SmolStr, f64)> {
        self.declared
            .iter()
            .find(|branch| branch.name == *name)
            .map(|branch| {
                (
                    BranchKey::Named(name.clone()),
                    self.ground(&branch.pos_node),
                    self.ground(&branch.neg_node),
                    1.0,
                )
            })
    }

    fn access(&self, access: &BranchAccess) -> Option<(BranchKey, SmolStr, SmolStr, f64)> {
        if access.kind() != Some(AccessKind::Flow) {
            return None;
        }
        self.resolve(access)
    }

    fn resolve(&self, access: &BranchAccess) -> Option<(BranchKey, SmolStr, SmolStr, f64)> {
        match access {
            BranchAccess::Nodes { pos, neg: None, .. } => {
                self.named(pos).or_else(|| Some(self.nodes(pos, "0")))
            }
            BranchAccess::Nodes {
                pos,
                neg: Some(neg),
                ..
            } => Some(self.nodes(pos, neg)),
            BranchAccess::Branch { name, .. } => self.named(name),
        }
    }

    fn contribution(
        &self,
        contribution: &AnalyzedContribution,
    ) -> (BranchKey, SmolStr, SmolStr, f64) {
        if let Some(name) = &contribution.declared_branch
            && let Some(branch) = self.named(name)
        {
            return branch;
        }
        let (pos, neg) = contribution
            .branch
            .split_once(',')
            .unwrap_or((&contribution.branch, "0"));
        self.nodes(pos.trim(), neg.trim())
    }
}

/// Local branch keys must be resolved before hierarchy node bindings merge
/// otherwise distinct module ports onto the same parent node.
pub(super) fn canonical_node_pair(
    pos: &str,
    neg: &str,
    grounds: &[SmolStr],
) -> (SmolStr, SmolStr, f64) {
    let (_, pos, neg, sign) = BranchResolver {
        declared: &[],
        grounds,
    }
    .nodes(pos, neg);
    (pos, neg, sign)
}

#[derive(Default)]
pub(super) struct HierarchyBranches {
    pub unnamed: BTreeMap<(SmolStr, SmolStr), Span>,
    pub port_flows: BTreeMap<SmolStr, Span>,
    pub conducting_named: BTreeSet<SmolStr>,
    pub conducting_unnamed: BTreeSet<(SmolStr, SmolStr)>,
}

pub(super) fn hierarchy_branches(module: &AnalyzedModule) -> HierarchyBranches {
    let resolver = BranchResolver {
        declared: &module.branches,
        grounds: &module.ground_nodes,
    };
    let mut inventory = HierarchyBranches::default();
    let mut inspect = |expression: &Expression| {
        visit_expression(expression, &mut |expression| {
            let Expression::BranchAccess(access) = expression else {
                return;
            };
            if let BranchAccess::Branch {
                name,
                kind: Some(AccessKind::Flow),
                span,
                ..
            } = access
                && resolver.named(name).is_none()
                && module.ports.iter().any(|port| port.name == *name)
            {
                inventory.port_flows.entry(name.clone()).or_insert(*span);
            }
            if let Some((key, ..)) = resolver.resolve(access) {
                if let BranchKey::Nodes(pos, neg) = &key {
                    inventory
                        .unnamed
                        .entry((pos.clone(), neg.clone()))
                        .or_insert(access.span());
                }
                if access.kind() == Some(AccessKind::Flow) {
                    match key {
                        BranchKey::Named(name) => {
                            inventory.conducting_named.insert(name);
                        }
                        BranchKey::Nodes(pos, neg) => {
                            inventory.conducting_unnamed.insert((pos, neg));
                        }
                    }
                }
            }
        });
    };
    for contribution in &module.contributions {
        inspect(&contribution.expression);
    }
    visit_statements(&module.statements, &mut inspect);
    for process in &module.digital.processes {
        super::digital_walk::visit_roots(&process.body, &mut inspect);
    }
    for assign in &module.digital.continuous_assigns {
        inspect(&assign.assignment.value);
        if let Some(delay) = &assign.assignment.delay {
            inspect(delay);
        }
    }
    for contribution in &module.contributions {
        match resolver.contribution(contribution).0 {
            BranchKey::Named(name) => {
                inventory.conducting_named.insert(name);
            }
            BranchKey::Nodes(pos, neg) => {
                inventory
                    .unnamed
                    .entry((pos.clone(), neg.clone()))
                    .or_insert(contribution.span);
                inventory.conducting_unnamed.insert((pos, neg));
            }
        }
    }
    inventory
}

pub(super) fn sum_expressions(mut terms: Vec<Expression>, span: Span) -> Expression {
    // Keep port sums logarithmic in depth even for a large instance tree.
    while terms.len() > 1 {
        let mut inputs = terms.into_iter();
        let mut next = Vec::with_capacity(inputs.len().div_ceil(2));
        while let Some(left) = inputs.next() {
            next.push(if let Some(right) = inputs.next() {
                SemanticAnalyzer::binary_expr(BinaryOp::Add, left, right)
            } else {
                left
            });
        }
        terms = next;
    }
    terms
        .pop()
        .unwrap_or_else(|| SemanticAnalyzer::number_expr(0.0, span))
}

pub(super) fn expand_port_flows(
    module: &mut AnalyzedModule,
    ports: &BTreeMap<SmolStr, Expression>,
) {
    if ports.is_empty() {
        return;
    }
    let branches = BTreeMap::new();
    let resolver = BranchResolver {
        declared: &[],
        grounds: &[],
    };
    let rewrite =
        |expression: &mut Expression| rewrite_expression(expression, &branches, &resolver, ports);
    rewrite_statements(&mut module.statements, &rewrite);
    for process in &mut module.digital.processes {
        super::digital_walk::rewrite_roots(&mut process.body, &mut |expression| {
            rewrite(expression)
        });
    }
    for assign in &mut module.digital.continuous_assigns {
        rewrite(&mut assign.assignment.value);
        if let Some(delay) = &mut assign.assignment.delay {
            rewrite(delay);
        }
    }
    rewrite_regions(&mut module.body, &rewrite, &branches, &HashMap::new());
    for contribution in &mut module.contributions {
        rewrite(&mut contribution.expression);
    }
}

#[derive(Clone)]
struct FlowBranch {
    pos: SmolStr,
    neg: SmolStr,
    state: SmolStr,
    span: Span,
    source: bool,
    switch: Option<super::switch_branches::SwitchState>,
}

fn validate_indirect_source_pairs(
    module: &AnalyzedModule,
    resolver: &BranchResolver<'_>,
) -> CompileResult<()> {
    if !module.contributions.iter().any(|source| source.indirect) {
        return Ok(());
    }
    // VAMS 5.6.7.2 applies to the physical pair, including separately named
    // parallel branches. Check before flow lowering introduces private nodes.
    let mut kinds = HashMap::new();
    let mut indirect_branches = BTreeSet::new();
    for source in &module.contributions {
        let (branch, pos, neg, _) = resolver.contribution(source);
        if source.indirect && !indirect_branches.insert(branch) {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidContribution(
                    "over-determined branch: multiple indirect constraints target the same source"
                        .into(),
                ),
                source.span,
            )));
        }
        let pair = if pos <= neg { (pos, neg) } else { (neg, pos) };
        if kinds
            .insert(pair.clone(), source.indirect)
            .is_some_and(|previous| previous != source.indirect)
        {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidContribution(format!(
                    "direct and indirect contributions cannot share analog nets '{}' and '{}' (including parallel branches)",
                    pair.0, pair.1
                )),
                source.span,
            )));
        }
    }
    Ok(())
}

pub(crate) fn lower<'a>(
    mut module: Cow<'a, AnalyzedModule>,
) -> CompileResult<Cow<'a, AnalyzedModule>> {
    normalize_potential_directions(&mut module);
    let resolver = BranchResolver {
        declared: &module.branches,
        grounds: &module.ground_nodes,
    };
    validate_indirect_source_pairs(&module, &resolver)?;
    let mut branches = BTreeMap::new();
    let mut port_reads = std::collections::BTreeSet::new();
    let mut potential_reads = HashSet::new();
    let mut inspect = |expression: &Expression| {
        visit_expression(expression, &mut |expression| {
            if let Expression::BranchAccess(access) = expression
                && access.kind() == Some(AccessKind::Potential)
                && let Some((key, ..)) = resolver.resolve(access)
            {
                potential_reads.insert(key);
            }
            if let Expression::BranchAccess(BranchAccess::Branch {
                name,
                kind: Some(AccessKind::Flow),
                ..
            }) = expression
                && resolver.named(name).is_none()
                && module.ports.iter().any(|port| port.name == *name)
            {
                port_reads.insert(name.clone());
            }
            if let Expression::BranchAccess(access) = expression
                && let Some((key, pos, neg, _)) = resolver.access(access)
            {
                branches.entry(key).or_insert(FlowBranch {
                    pos,
                    neg,
                    state: SmolStr::default(),
                    span: access.span(),
                    source: false,
                    switch: None,
                });
            }
        });
    };
    for contribution in &module.contributions {
        inspect(&contribution.expression);
    }
    visit_statements(&module.statements, &mut inspect);
    for process in &module.digital.processes {
        super::digital_walk::visit_roots(&process.body, &mut inspect);
    }
    for assign in &module.digital.continuous_assigns {
        inspect(&assign.assignment.value);
        if let Some(delay) = &assign.assignment.delay {
            inspect(delay);
        }
    }
    let mut existing_unknowns = HashSet::new();
    let mut source_kinds = HashMap::<_, u8>::new();
    let mut incident = branches.clone();
    for contribution in &module.contributions {
        let (key, pos, neg, _) = resolver.contribution(contribution);
        let branch = incident.entry(key.clone()).or_insert(FlowBranch {
            pos,
            neg,
            state: SmolStr::default(),
            span: contribution.span,
            source: false,
            switch: None,
        });
        *source_kinds.entry(key.clone()).or_default() |= if contribution.indirect {
            4
        } else if contribution.is_current {
            1
        } else {
            2
        };
        branch.source |= contribution.is_current;
        if !contribution.is_current || contribution.indirect {
            existing_unknowns.insert(key);
        } else if branch.pos == branch.neg {
            // Coincident bound ports have no physical injection. Retain
            // evaluation (including state and validation) in a private
            // equation even when nobody probes the cancelled source.
            branches.entry(key).or_insert_with(|| branch.clone()).source = true;
        } else if let Some(branch) = branches.get_mut(&key) {
            branch.source = true;
        }
    }
    for (key, branch) in &incident {
        if branch.pos != branch.neg
            && (port_reads.contains(&branch.pos) || port_reads.contains(&branch.neg))
        {
            branches
                .entry(key.clone())
                .or_insert_with(|| branch.clone());
        }
    }
    // Potential and indirect branches already carry solver-owned currents.
    // A switched source needs its current even while the potential arm is off.
    let switches: BTreeSet<_> = source_kinds
        .into_iter()
        .filter_map(|(key, kinds)| (kinds == 3).then_some(key))
        .collect();
    for key in &switches {
        existing_unknowns.remove(key);
        branches
            .entry(key.clone())
            .or_insert_with(|| incident[key].clone());
    }
    branches.retain(|key, _| !existing_unknowns.contains(key));
    for (key, branch) in &branches {
        if !branch.source && potential_reads.contains(key) {
            return Err(CompileError::Semantic(SemanticError::new(
                SemanticErrorKind::InvalidAnalogOperator(
                    "cannot probe both flow and potential on a branch without a source contribution".into(),
                ),
                branch.span,
            )));
        }
    }
    if branches.is_empty() && port_reads.is_empty() {
        return Ok(module);
    }

    let rewrites: HashMap<_, _> = module
        .contributions
        .iter()
        .filter_map(|contribution| {
            let (key, _, _, sign) = resolver.contribution(contribution);
            branches
                .contains_key(&key)
                .then_some((contribution.site, (key, sign)))
        })
        .collect();
    // Preserve the resolver's authored declarations while the owned module
    // gains its private nodes. No expression forest is cloned for this map.
    let declared = module.branches.clone();
    let ground_nodes = module.ground_nodes.clone();
    let target = module.to_mut();
    for (ordinal, (key, branch)) in branches.iter_mut().enumerate() {
        let mut suffix = ordinal;
        loop {
            let name: SmolStr = format!("__flow_state{suffix}").into();
            if target.symbol_table.lookup(&name).is_none()
                && !target.internal_nodes.iter().any(|node| node.name == name)
                && !target.ports.iter().any(|port| port.name == name)
                && !target
                    .variables
                    .iter()
                    .any(|variable| variable.name == name)
                && !target
                    .parameters
                    .iter()
                    .any(|parameter| parameter.name == name)
                && !target.branches.iter().any(|branch| branch.name == name)
            {
                branch.state = name;
                break;
            }
            suffix += 1;
        }
        let index = target.internal_nodes.len();
        target.internal_nodes.push(AnalyzedInternalNode {
            is_state: true,
            name: branch.state.clone(),
            discipline: "electrical".into(),
            index,
        });
        if switches.contains(key) {
            branch.switch = Some(super::switch_branches::SwitchState::allocate(
                target,
                &branch.state,
                branch.span,
            ));
        }
    }

    let port_values: BTreeMap<_, _> = port_reads
        .into_iter()
        .map(|port| {
            let mut terms = Vec::new();
            for (key, branch) in &incident {
                let sign = i8::from(branch.pos == port) - i8::from(branch.neg == port);
                if sign == 0 {
                    continue;
                }
                let term = if let Some(lowered) = branches.get(key) {
                    potential(&lowered.state, "0", branch.span)
                } else {
                    Expression::BranchAccess(match key {
                        BranchKey::Named(name) => BranchAccess::Branch {
                            access: "I".into(),
                            kind: Some(AccessKind::Flow),
                            name: name.clone(),
                            span: branch.span,
                        },
                        BranchKey::Nodes(pos, neg) => BranchAccess::Nodes {
                            access: "I".into(),
                            kind: Some(AccessKind::Flow),
                            pos: pos.clone(),
                            neg: Some(neg.clone()),
                            span: branch.span,
                        },
                    })
                };
                terms.push(signed(term, f64::from(sign)));
            }
            (port, sum_expressions(terms, Span::dummy()))
        })
        .collect();
    let resolver = BranchResolver {
        declared: &declared,
        grounds: &ground_nodes,
    };
    let rewrite = |expression: &mut Expression| {
        rewrite_expression(expression, &branches, &resolver, &port_values);
    };
    rewrite_statements(&mut target.statements, &rewrite);
    for process in &mut target.digital.processes {
        super::digital_walk::rewrite_roots(&mut process.body, &mut |expression| {
            rewrite(expression)
        });
    }
    for assign in &mut target.digital.continuous_assigns {
        rewrite(&mut assign.assignment.value);
        if let Some(delay) = &mut assign.assignment.delay {
            rewrite(delay);
        }
    }
    rewrite_regions(&mut target.body, &rewrite, &branches, &rewrites);
    for contribution in &mut target.contributions {
        rewrite(&mut contribution.expression);
        redirect_contribution(contribution, &branches, &rewrites);
    }
    let switch_sites = rewrites
        .iter()
        .filter_map(|(&site, (key, sign))| {
            branches[key]
                .switch
                .as_ref()
                .map(|state| (site, (state, *sign)))
        })
        .collect();
    super::switch_branches::lower(target, &switch_sites)?;
    for (key, branch) in &branches {
        let state = potential(&branch.state, "0", branch.span);
        let balance = if let Some(switch) = &branch.switch {
            switch.balance(
                state.clone(),
                potential(&branch.pos, &branch.neg, branch.span),
            )
        } else if branch.source {
            state.clone()
        } else {
            potential(&branch.pos, &branch.neg, branch.span)
        };
        for (index, (label, declared_branch, expression)) in [
            (
                format!("{},{}", branch.pos, branch.neg).into(),
                match key {
                    BranchKey::Named(name) => Some(name.clone()),
                    BranchKey::Nodes(..) => None,
                },
                state,
            ),
            (branch.state.clone(), None, balance),
        ]
        .into_iter()
        .enumerate()
        {
            if index == 0 && branch.pos == branch.neg {
                // Bound ports can coincide. The source's internal equation
                // remains observable, but its physical injection is zero.
                continue;
            }
            let site = AnalogSiteId(target.analog_site_count);
            target.analog_site_count =
                target.analog_site_count.checked_add(1).ok_or_else(|| {
                    CompileError::Semantic(SemanticError::new(
                        SemanticErrorKind::UnsupportedFeature(
                            "too many analog equation sites".into(),
                        ),
                        branch.span,
                    ))
                })?;
            let contribution = AnalyzedContribution {
                branch: label,
                declared_branch,
                is_current: true,
                indirect: false,
                equation_abstol: None,
                expression,
                site,
                expression_guard: AnalogSiteGuard::None,
                expr_type: ValueType::Real,
                span: branch.span,
            };
            target
                .body
                .push(AnalyzedRegion::Contribution(contribution.clone()));
            target.contributions.push(contribution);
        }
    }
    Ok(module)
}

fn potential(pos: &str, neg: &str, span: Span) -> Expression {
    Expression::BranchAccess(BranchAccess::Nodes {
        access: "V".into(),
        kind: Some(AccessKind::Potential),
        pos: pos.into(),
        neg: Some(neg.into()),
        span,
    })
}

/// Every equation of one potential branch uses its first source's direction.
/// Normalize before the flat and structured IRs diverge so residuals, AD and
/// noise agree with the solver's single structural branch row.
fn normalize_potential_directions(module: &mut Cow<'_, AnalyzedModule>) {
    let resolver = BranchResolver {
        declared: &module.branches,
        grounds: &module.ground_nodes,
    };
    let mut directions = HashMap::new();
    let mut rewrites = HashMap::new();
    for contribution in &module.contributions {
        if contribution.is_current || contribution.indirect {
            continue;
        }
        let (key, _, _, sign) = resolver.contribution(contribution);
        let (first_sign, label) = directions
            .entry(key)
            .or_insert_with(|| (sign, contribution.branch.clone()));
        if sign != *first_sign {
            rewrites.insert(contribution.site, label.clone());
        }
    }
    if rewrites.is_empty() {
        return;
    }
    let target = module.to_mut();
    let redirect = |contribution: &mut AnalyzedContribution, flat| {
        if let Some(label) = rewrites.get(&contribution.site) {
            contribution.branch = label.clone();
            negate_contribution(contribution, flat);
        }
    };
    for contribution in &mut target.contributions {
        redirect(contribution, true);
    }
    let mut pending = vec![target.body.as_mut_slice()];
    while let Some(body) = pending.pop() {
        for region in body {
            match region {
                AnalyzedRegion::Contribution(contribution) => redirect(contribution, false),
                AnalyzedRegion::Conditional {
                    then_body,
                    else_body,
                    ..
                } => {
                    pending.push(then_body);
                    pending.push(else_body);
                }
                AnalyzedRegion::Loop { body, .. } | AnalyzedRegion::Initialization { body, .. } => {
                    pending.push(body)
                }
                AnalyzedRegion::Assignment(_) | AnalyzedRegion::Task(_) => {}
            }
        }
    }
}

pub(super) fn signed(expression: Expression, sign: f64) -> Expression {
    if sign > 0.0 {
        expression
    } else {
        let span = expression.span();
        Expression::Unary(UnaryExpr {
            op: UnaryOp::Neg,
            operand: Box::new(expression),
            span,
        })
    }
}

fn redirect_contribution(
    contribution: &mut AnalyzedContribution,
    branches: &BTreeMap<BranchKey, FlowBranch>,
    rewrites: &HashMap<AnalogSiteId, (BranchKey, f64)>,
) {
    let Some((key, sign)) = rewrites.get(&contribution.site) else {
        return;
    };
    if branches[key].switch.is_some() {
        return;
    }
    contribution.branch = branches[key].state.clone();
    contribution.declared_branch = None;
    if *sign > 0.0 {
        negate_contribution(contribution, true);
    }
}

pub(super) fn negate_contribution(contribution: &mut AnalyzedContribution, flat: bool) {
    // Flat expressions carry the recorded select wrapper; structured ones
    // are already inside their guard region and must be negated as a whole.
    let negate = |expression: &mut Expression| {
        *expression = signed(
            std::mem::replace(
                expression,
                SemanticAnalyzer::number_expr(0.0, contribution.span),
            ),
            -1.0,
        );
    };
    match (
        flat,
        contribution.expression_guard,
        &mut contribution.expression,
    ) {
        (true, AnalogSiteGuard::Select, Expression::Conditional(conditional)) => {
            negate(&mut conditional.then_expr);
            negate(&mut conditional.else_expr);
        }
        (_, _, expression) => negate(expression),
    }
}

fn rewrite_expression(
    expression: &mut Expression,
    branches: &BTreeMap<BranchKey, FlowBranch>,
    resolver: &BranchResolver<'_>,
    ports: &BTreeMap<SmolStr, Expression>,
) {
    let mut pending = vec![expression];
    while let Some(expression) = pending.pop() {
        if let Expression::BranchAccess(BranchAccess::Branch {
            name,
            kind: Some(AccessKind::Flow),
            ..
        }) = expression
            && let Some(value) = ports.get(name)
        {
            *expression = value.clone();
            continue;
        }
        if let Expression::BranchAccess(access) = expression
            && let Some((key, _, _, sign)) = resolver.access(access)
            && let Some(branch) = branches.get(&key)
        {
            // Retain a branch probe for ddx's second argument. A unary
            // negation has the same value but is not a valid derivative axis.
            *expression = if sign > 0.0 {
                potential(&branch.state, "0", access.span())
            } else {
                potential("0", &branch.state, access.span())
            };
            continue;
        }
        for_child_mut(expression, &mut |child| pending.push(child));
    }
}

fn visit_statements(body: &[AnalyzedStatement], visit: &mut impl FnMut(&Expression)) {
    for statement in body {
        match statement {
            AnalyzedStatement::Assignment(assignment) => {
                visit(&assignment.expression);
                if let Some(index) = &assignment.index {
                    visit(index);
                }
            }
            AnalyzedStatement::Loop(loop_) => {
                visit(&loop_.condition);
                visit_statements(&loop_.body, visit);
            }
            AnalyzedStatement::Initialization { body, .. } => visit_statements(body, visit),
            AnalyzedStatement::Task(task) => task.expressions().for_each(&mut *visit),
        }
    }
}

fn rewrite_statements(body: &mut [AnalyzedStatement], rewrite: &impl Fn(&mut Expression)) {
    for statement in body {
        match statement {
            AnalyzedStatement::Assignment(assignment) => {
                rewrite(&mut assignment.expression);
                if let Some(index) = &mut assignment.index {
                    rewrite(index);
                }
            }
            AnalyzedStatement::Loop(loop_) => {
                rewrite(&mut loop_.condition);
                rewrite_statements(&mut loop_.body, rewrite);
            }
            AnalyzedStatement::Initialization { body, .. } => rewrite_statements(body, rewrite),
            AnalyzedStatement::Task(task) => task.expressions_mut().for_each(rewrite),
        }
    }
}

fn rewrite_regions(
    body: &mut [AnalyzedRegion],
    rewrite: &impl Fn(&mut Expression),
    branches: &BTreeMap<BranchKey, FlowBranch>,
    rewrites: &HashMap<AnalogSiteId, (BranchKey, f64)>,
) {
    for region in body {
        match region {
            AnalyzedRegion::Assignment(assignment) => {
                rewrite(&mut assignment.expression);
                if let Some(index) = &mut assignment.index {
                    rewrite(index);
                }
            }
            AnalyzedRegion::Contribution(contribution) => {
                rewrite(&mut contribution.expression);
                // Structured expressions are unguarded, even when their flat
                // counterpart records a select wrapper.
                let guard = contribution.expression_guard;
                contribution.expression_guard = AnalogSiteGuard::None;
                redirect_contribution(contribution, branches, rewrites);
                contribution.expression_guard = guard;
            }
            AnalyzedRegion::Conditional {
                condition,
                then_body,
                else_body,
                ..
            } => {
                rewrite(condition);
                rewrite_regions(then_body, rewrite, branches, rewrites);
                rewrite_regions(else_body, rewrite, branches, rewrites);
            }
            AnalyzedRegion::Loop {
                condition, body, ..
            } => {
                rewrite(condition);
                rewrite_regions(body, rewrite, branches, rewrites);
            }
            AnalyzedRegion::Initialization { body, .. } => {
                rewrite_regions(body, rewrite, branches, rewrites)
            }
            AnalyzedRegion::Task(task) => task.expressions_mut().for_each(rewrite),
        }
    }
}

pub(super) fn visit_expression(expression: &Expression, visit: &mut impl FnMut(&Expression)) {
    enum Pending<'a> {
        Expression(&'a Expression),
        Element(&'a ArrayLiteralElement),
    }
    let mut pending = vec![Pending::Expression(expression)];
    while let Some(next) = pending.pop() {
        let expression = match next {
            Pending::Expression(expression) => expression,
            Pending::Element(ArrayLiteralElement::Value(expression)) => expression,
            Pending::Element(ArrayLiteralElement::Replication(replication)) => {
                pending.extend(replication.elements.iter().map(Pending::Element));
                &replication.count
            }
        };
        visit(expression);
        match expression {
            Expression::Binary(expr) => {
                pending.push(Pending::Expression(&expr.left));
                pending.push(Pending::Expression(&expr.right));
            }
            Expression::Unary(expr) => pending.push(Pending::Expression(&expr.operand)),
            Expression::Conditional(expr) => {
                pending.push(Pending::Expression(&expr.condition));
                pending.push(Pending::Expression(&expr.then_expr));
                pending.push(Pending::Expression(&expr.else_expr));
            }
            Expression::Call(expr) => pending.extend(expr.args.iter().map(Pending::Expression)),
            Expression::SystemFunction(expr) => {
                pending.extend(expr.args.iter().map(Pending::Expression))
            }
            Expression::ArrayAccess(expr) => pending.push(Pending::Expression(&expr.index)),
            Expression::ArrayLiteral(expr) => {
                pending.extend(expr.elements.iter().map(Pending::Element))
            }
            Expression::AnalogOperator(AnalogOperator::Limit {
                proposed,
                candidate,
                type_metadata,
                ..
            }) => {
                pending.push(Pending::Expression(proposed));
                pending.push(Pending::Expression(candidate));
                if let Some(value) = type_metadata {
                    pending.push(Pending::Expression(value));
                }
            }
            Expression::NoiseSource(NoiseSource::White { power, .. }) => {
                pending.push(Pending::Expression(power))
            }
            Expression::NoiseSource(NoiseSource::Flicker {
                power, exponent, ..
            }) => {
                pending.push(Pending::Expression(power));
                pending.push(Pending::Expression(exponent));
            }
            Expression::NoiseSource(NoiseSource::Table { data, .. }) => {
                pending.extend(data.iter().map(Pending::Expression))
            }
            Expression::Digital(expr) => {
                pending.extend(expr.children().into_iter().map(Pending::Expression))
            }
            Expression::Number(_)
            | Expression::StringLit(_)
            | Expression::Identifier(_)
            | Expression::NullArgument(_)
            | Expression::BranchAccess(_)
            | Expression::AnalogOperator(AnalogOperator::LimiterArgument { .. }) => {}
        }
    }
}

pub(super) fn for_child_mut<'a>(
    expression: &'a mut Expression,
    visit: &mut impl FnMut(&'a mut Expression),
) {
    match expression {
        Expression::Binary(expr) => {
            visit(&mut expr.left);
            visit(&mut expr.right);
        }
        Expression::Unary(expr) => visit(&mut expr.operand),
        Expression::Conditional(expr) => {
            visit(&mut expr.condition);
            visit(&mut expr.then_expr);
            visit(&mut expr.else_expr);
        }
        Expression::Call(expr) => expr.args.iter_mut().for_each(visit),
        Expression::SystemFunction(expr) => expr.args.iter_mut().for_each(visit),
        Expression::ArrayAccess(expr) => visit(&mut expr.index),
        Expression::ArrayLiteral(expr) => rewrite_elements(&mut expr.elements, visit),
        Expression::AnalogOperator(AnalogOperator::Limit {
            proposed,
            candidate,
            type_metadata,
            ..
        }) => {
            visit(proposed);
            visit(candidate);
            if let Some(value) = type_metadata {
                visit(value);
            }
        }
        Expression::NoiseSource(NoiseSource::White { power, .. }) => visit(power),
        Expression::NoiseSource(NoiseSource::Flicker {
            power, exponent, ..
        }) => {
            visit(power);
            visit(exponent);
        }
        Expression::NoiseSource(NoiseSource::Table { data, .. }) => data.iter_mut().for_each(visit),
        Expression::Digital(digital) => match digital {
            DigitalExpr::FourState(_) => {}
            DigitalExpr::PartSelect(expr) => {
                visit(&mut expr.msb);
                visit(&mut expr.lsb);
            }
            DigitalExpr::Xnor(expr) => {
                visit(&mut expr.left);
                visit(&mut expr.right);
            }
            DigitalExpr::CaseEquality(expr) => {
                visit(&mut expr.left);
                visit(&mut expr.right);
            }
            DigitalExpr::Reduction(expr) => visit(&mut expr.operand),
            DigitalExpr::ArithmeticShiftRight(expr) => {
                visit(&mut expr.left);
                visit(&mut expr.right);
            }
        },
        Expression::Number(_)
        | Expression::StringLit(_)
        | Expression::Identifier(_)
        | Expression::NullArgument(_)
        | Expression::BranchAccess(_)
        | Expression::AnalogOperator(AnalogOperator::LimiterArgument { .. }) => {}
    }
}

fn rewrite_elements<'a>(
    elements: &'a mut [ArrayLiteralElement],
    visit: &mut impl FnMut(&'a mut Expression),
) {
    for element in elements {
        match element {
            ArrayLiteralElement::Value(value) => visit(value),
            ArrayLiteralElement::Replication(replication) => {
                visit(&mut replication.count);
                rewrite_elements(&mut replication.elements, visit);
            }
        }
    }
}
