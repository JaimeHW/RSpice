//! Whether a module's contributions connect one of its electrical ports to
//! node 0, read from its canonical IR.
//!
//! One function answers this for every route: the build calls it for a module
//! it compiles at run time, and the generator calls it for a module it
//! compiles ahead of time and records the answer as the generated model's
//! [`GroundReference`] metadata. The rules, and what a guard may read, are
//! documented on [`rspice_veriloga_runtime::GroundReference`]'s module.
//!
//! Only a branch the module stamps joins nets: a contribution's branch, and
//! the plumbing the compiler lowers a probed flow onto. A read does not: a
//! potential or flow read in an expression, a condition, an assignment or a
//! tolerance, a Verilog-AMS process's analog read, and a task's arguments
//! (`$strobe`, `$bound_step`, ...) observe a net without stamping its row or
//! column, so none of them joins.
//!
//! The walk follows the structured analog block, [`HirModel::body`], because
//! that is where a contribution keeps the `if` it sits in:
//!
//! * a contribution in a branch of an `if` or `case` joins under that
//!   branch's condition, and under every condition around it;
//! * a variable assigned from literals and public parameters alone carries
//!   that expression forward until it is assigned again, and an `if` that
//!   assigns it in its branches leaves it `condition ? then : else`;
//! * a contribution in a loop or an analog initial block joins under the
//!   conditions around the statement, since whether it runs is not something
//!   the instance can know before it solves;
//! * a branch the compiler lowers onto a private state, because the module
//!   probes its flow or switches it between potential and flow, joins under
//!   the conditions of its authored sources, not the unconditional plumbing
//!   the lowering adds (see `Walk::finish`); a probed flow on a branch nothing
//!   drives is an ideal short, and joins as the plumbing is written.
//!
//! A port is electrical when its discipline's potential is a voltage or its
//! flow a current; a thermal port's ground is ambient, not node 0.

use std::collections::HashMap;

use rspice_veriloga_runtime::{GroundGuardOp, GroundJoin, GroundReference, GroundReferenceGraph};

use super::hir::HirRegion;
use super::{CanonicalIrArtifact, CanonicalValueType, ExprId, HirExprKind, HirModel};

/// A guard program longer than this is unknown. A chain of `if`s that each
/// reassign one variable from the last could otherwise grow without bound.
const MAX_GUARD_OPS: usize = 256;

/// [`GroundReference`] with owned storage, borrowing names from the artifact.
#[derive(Debug, Clone, PartialEq)]
pub struct GroundReferenceSummary<'a> {
    kind: SummaryKind,
    group_count: u32,
    port_groups: Vec<u32>,
    joins: Vec<GroundJoin>,
    guards: Vec<GroundGuardOp<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SummaryKind {
    Never,
    Always,
    Conditional,
}

impl GroundReferenceSummary<'_> {
    pub fn view(&self) -> GroundReference<'_> {
        match self.kind {
            SummaryKind::Never => GroundReference::Never,
            SummaryKind::Always => GroundReference::Always,
            SummaryKind::Conditional => GroundReference::Conditional(GroundReferenceGraph {
                group_count: self.group_count,
                port_groups: &self.port_groups,
                joins: &self.joins,
                guards: &self.guards,
            }),
        }
    }
}

/// Whether the module `artifact` holds connects an electrical port to node 0,
/// and under which conditions.
pub fn ground_reference(artifact: &CanonicalIrArtifact) -> GroundReferenceSummary<'_> {
    let hir = &artifact.hir;
    let mut walk = Walk::new(hir);
    let mut values = Values::new();
    for statement in &hir.prologue_statements {
        if let Some(super::HirStatement::Assignment(assignment)) =
            hir.statements.get(*statement as usize)
        {
            walk.assign(
                &mut values,
                assignment.target_name.as_str(),
                assignment.index.is_some(),
                assignment.expr.id,
            );
        }
    }
    if hir.body.is_empty() && !hir.contributions.is_empty() {
        // No structured block to follow: every contribution counts.
        for contribution in &hir.contributions {
            walk.contribution_branch(contribution.branch.as_str(), &[]);
        }
    } else {
        walk.regions(&hir.body, &[], &mut values);
    }
    summarize(hir, walk.finish())
}

type Program<'a> = Vec<GroundGuardOp<'a>>;

/// The variables whose values are known expressions at this point.
type Values<'a> = HashMap<&'a str, Program<'a>>;

/// One branch: the nets it connects, `None` for ground, under the conjunction
/// of its guards.
struct RawJoin<'a> {
    ends: [Option<&'a str>; 2],
    guards: Vec<Program<'a>>,
}

/// The guards a branch sits under, one conjunct each.
type Guards<'a> = Vec<Program<'a>>;

struct Walk<'a> {
    hir: &'a HirModel,
    joins: Vec<RawJoin<'a>>,
    parameters: HashMap<&'a str, &'a super::HirParameter>,
    variables: HashMap<&'a str, CanonicalValueType>,
    /// The compiler's private unknowns, among them the current of a branch
    /// whose flow the module probes, or whose sources switch between
    /// potential and flow.
    states: Vec<&'a str>,
    /// While set, joins belong to this state's plumbing; see [`Self::finish`].
    plumbing: Option<&'a str>,
    deferred: Vec<(&'a str, RawJoin<'a>)>,
    /// States whose physical injection the walk has passed.
    injected: Vec<&'a str>,
    /// Per state, the guards of the authored contributions redirected to it.
    sources: HashMap<&'a str, Vec<Guards<'a>>>,
    /// Per variable, the guards of every assignment to it of more than a
    /// literal. A switch branch's authored sources assign its retained value
    /// an expression; the lowering's reset at the top of the block, and the
    /// source kind, assign literals.
    assignments: HashMap<&'a str, Vec<Guards<'a>>>,
    /// Per state, the variables its balance reads: a switch branch's retained
    /// value and source kind.
    balance_reads: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Walk<'a> {
    fn new(hir: &'a HirModel) -> Self {
        Self {
            hir,
            joins: Vec::new(),
            parameters: hir
                .parameters
                .iter()
                .map(|parameter| (parameter.name.as_str(), parameter))
                .collect(),
            variables: hir
                .variables
                .iter()
                .map(|variable| (variable.name.as_str(), variable.value_type))
                .collect(),
            states: hir
                .internal_nodes
                .iter()
                .filter(|node| node.is_state)
                .map(|node| node.name.as_str())
                .collect(),
            plumbing: None,
            deferred: Vec::new(),
            injected: Vec::new(),
            sources: HashMap::new(),
            assignments: HashMap::new(),
            balance_reads: HashMap::new(),
        }
    }

    fn net(&self, name: Option<&'a str>) -> Option<&'a str> {
        name.filter(|name| {
            !name.is_empty() && *name != "0" && !self.hir.ground_nodes.iter().any(|g| g == name)
        })
    }

    fn join(&mut self, pos: &'a str, neg: Option<&'a str>, guards: &[Program<'a>]) {
        let ends = [self.net(Some(pos)), self.net(neg)];
        if ends[0].is_some() && ends[0] == ends[1] {
            return;
        }
        let join = RawJoin {
            ends,
            guards: guards.to_vec(),
        };
        match self.plumbing {
            Some(state) => self.deferred.push((state, join)),
            None => self.joins.push(join),
        }
    }

    /// The state a contribution's branch names, when it names one.
    fn state(&self, branch: &str) -> Option<&'a str> {
        self.states.iter().copied().find(|state| *state == branch)
    }

    /// The state whose current an expression injects, when it is exactly that
    /// state's potential: the compiler's physical source for a lowered branch.
    fn injected_state(&self, id: ExprId) -> Option<&'a str> {
        let hir = self.hir;
        match &hir.expressions.get(usize::from(id))?.kind {
            HirExprKind::BranchAccess {
                kind: crate::ast::AccessKind::Potential,
                pos,
                neg: Some(neg),
                ..
            } if neg.as_str() == "0" => self.state(pos),
            _ => None,
        }
    }

    /// The identifiers an expression reads.
    fn identifiers(&self, id: ExprId, out: &mut Vec<&'a str>) {
        let hir = self.hir;
        let Some(expression) = hir.expressions.get(usize::from(id)) else {
            return;
        };
        if let HirExprKind::Identifier { name } = &expression.kind {
            out.push(name.as_str());
        }
        for child in children(&expression.kind) {
            self.identifiers(child, out);
        }
    }

    /// The joins, with each lowered branch's plumbing put back under its
    /// sources.
    ///
    /// The compiler lowers a branch whose flow the module probes, or whose
    /// sources switch between potential and flow, onto a private state: each
    /// authored source, under its own conditions, becomes a contribution to
    /// the state or an assignment to the switch's retained value and kind,
    /// and an unconditional injection and balance at the end of the block tie
    /// the branch's nets through the state. The branch exists when one of its
    /// authored sources runs, so the plumbing joins under the disjunction of
    /// their guards. A state with no authored source is a probe of a branch
    /// that has none, an ideal short, and its plumbing joins as written.
    fn finish(mut self) -> Vec<RawJoin<'a>> {
        for (state, mut join) in std::mem::take(&mut self.deferred) {
            let mut sources: Vec<&Guards<'a>> =
                self.sources.get(state).into_iter().flatten().collect();
            for variable in self.balance_reads.get(state).into_iter().flatten() {
                sources.extend(self.assignments.get(variable).into_iter().flatten());
            }
            if !sources.is_empty() && sources.iter().all(|guards| !guards.is_empty()) {
                let mut program = Vec::new();
                for (index, guards) in sources.into_iter().enumerate() {
                    program.extend(conjunction(guards));
                    if index > 0 {
                        program.push(GroundGuardOp::Or);
                    }
                }
                join.guards.push(bounded(program));
            }
            self.joins.push(join);
        }
        self.joins
    }

    /// The branch a contribution stamps, `pos,neg` or a single net.
    fn contribution_branch(&mut self, branch: &'a str, guards: &[Program<'a>]) {
        let mut ends = branch.splitn(2, ',');
        let pos = ends.next().unwrap_or_default();
        self.join(pos, ends.next(), guards);
    }

    /// A contribution joins the nets of its branch. What its expression reads
    /// is not stamped there, and joins nothing.
    fn contribution(&mut self, contribution: &'a super::HirContribution, guards: &[Program<'a>]) {
        let branch = contribution.branch.as_str();
        let expression = contribution.expression.id;
        match (self.state(branch), self.injected_state(expression)) {
            // The balance of a lowered branch.
            (Some(state), _) if self.injected.contains(&state) => {
                let mut reads = Vec::new();
                self.identifiers(expression, &mut reads);
                self.balance_reads.entry(state).or_default().extend(reads);
                self.plumbing = Some(state);
                self.contribution_branch(branch, guards);
                self.plumbing = None;
            }
            // An authored source redirected to its branch's state.
            (Some(state), _) => {
                self.sources.entry(state).or_default().push(guards.to_vec());
                self.contribution_branch(branch, guards);
            }
            // The physical injection of a lowered branch.
            (None, Some(state)) => {
                self.injected.push(state);
                self.plumbing = Some(state);
                self.contribution_branch(branch, guards);
                self.plumbing = None;
            }
            (None, None) => self.contribution_branch(branch, guards),
        }
    }

    fn regions(
        &mut self,
        regions: &'a [HirRegion],
        guards: &[Program<'a>],
        values: &mut Values<'a>,
    ) {
        for region in regions {
            match region {
                HirRegion::Assignment(assignment) => {
                    let literal = matches!(
                        self.hir.expressions.get(usize::from(assignment.expr.id)),
                        Some(expression) if matches!(expression.kind, HirExprKind::Number { .. })
                    );
                    if !literal {
                        self.assignments
                            .entry(assignment.target_name.as_str())
                            .or_default()
                            .push(guards.to_vec());
                    }
                    self.assign(
                        values,
                        assignment.target_name.as_str(),
                        assignment.index.is_some(),
                        assignment.expr.id,
                    );
                }
                HirRegion::Contribution(contribution) => self.contribution(contribution, guards),
                HirRegion::Conditional {
                    condition,
                    then_body,
                    else_body,
                    ..
                } => {
                    let test = self.value(condition.id, values);
                    let mut negated = test.clone();
                    negated.push(GroundGuardOp::Not);
                    let mut then_values = values.clone();
                    let mut else_values = values.clone();
                    let mut inner = guards.to_vec();
                    inner.push(test.clone());
                    self.regions(then_body, &inner, &mut then_values);
                    *inner.last_mut().expect("pushed above") = negated;
                    self.regions(else_body, &inner, &mut else_values);
                    let mut assigned = Vec::new();
                    assigned_in(then_body, &mut assigned);
                    assigned_in(else_body, &mut assigned);
                    for name in assigned {
                        match (then_values.get(name), else_values.get(name)) {
                            (Some(then), Some(otherwise)) if then == otherwise => {
                                let value = then.clone();
                                values.insert(name, value);
                            }
                            (Some(then), Some(otherwise)) => {
                                let mut value = test.clone();
                                value.extend_from_slice(then);
                                value.extend_from_slice(otherwise);
                                value.push(GroundGuardOp::Select);
                                values.insert(name, bounded(value));
                            }
                            _ => {
                                values.remove(name);
                            }
                        }
                    }
                }
                HirRegion::Loop { body, .. } => {
                    self.forget(body, values);
                    let mut inner = values.clone();
                    self.regions(body, guards, &mut inner);
                    self.forget(body, values);
                }
                // A task's arguments (`$strobe`, `$bound_step`, ...) are
                // observations, not equations: nothing they read is stamped.
                HirRegion::Task(_) => {}
                HirRegion::Initialization { body, .. } => {
                    let mut inner = values.clone();
                    self.regions(body, guards, &mut inner);
                    self.forget(body, values);
                }
            }
        }
    }

    fn forget(&self, body: &'a [HirRegion], values: &mut Values<'a>) {
        let mut assigned = Vec::new();
        assigned_in(body, &mut assigned);
        for name in assigned {
            values.remove(name);
        }
    }

    /// Record what an assignment leaves its target holding.
    fn assign(&self, values: &mut Values<'a>, target: &'a str, indexed: bool, expr: ExprId) {
        if indexed {
            values.remove(target);
            return;
        }
        let mut value = self.value(expr, values);
        if value.contains(&GroundGuardOp::Unknown) {
            values.remove(target);
            return;
        }
        if self.variables.get(target) == Some(&CanonicalValueType::Integer) {
            value.push(GroundGuardOp::Integer);
        }
        values.insert(target, bounded(value));
    }

    /// The expression `id` as a guard program over what the instance knows.
    fn value(&self, id: ExprId, values: &Values<'a>) -> Program<'a> {
        let mut program = Vec::new();
        self.push_value(id, values, &mut program);
        bounded(program)
    }

    fn push_value(&self, id: ExprId, values: &Values<'a>, out: &mut Program<'a>) {
        use GroundGuardOp as Op;
        let hir = self.hir;
        let Some(expression) = hir.expressions.get(usize::from(id)) else {
            out.push(Op::Unknown);
            return;
        };
        let identifier = |id: ExprId| match hir.expressions.get(usize::from(id)).map(|e| &e.kind) {
            Some(HirExprKind::Identifier { name }) => Some(name.as_str()),
            _ => None,
        };
        match &expression.kind {
            HirExprKind::Number { value, .. } => out.push(Op::Number(*value)),
            HirExprKind::Identifier { name } => {
                let name = name.as_str();
                if let Some(value) = values.get(name) {
                    out.extend_from_slice(value);
                } else if self.variables.contains_key(name) {
                    out.push(Op::Unknown);
                } else {
                    out.push(match self.parameters.get(name) {
                        Some(parameter) => match parameter.elaboration_value {
                            Some(value) => Op::Number(value),
                            None if parameter.is_public => Op::Parameter(parameter.name.as_str()),
                            None => Op::Unknown,
                        },
                        None => Op::Unknown,
                    });
                }
            }
            HirExprKind::SystemFunction { name, args }
                if args.len() == 1 && name.eq_ignore_ascii_case("$param_given") =>
            {
                out.push(
                    match identifier(args[0]).and_then(|name| self.parameters.get(name)) {
                        Some(parameter) => match parameter.elaboration_given {
                            Some(given) => Op::Number(if given { 1.0 } else { 0.0 }),
                            None if parameter.is_public => {
                                Op::ParameterGiven(parameter.name.as_str())
                            }
                            None => Op::Unknown,
                        },
                        None => Op::Unknown,
                    },
                );
            }
            HirExprKind::SystemFunction { name, args }
                if args.len() == 1 && name.eq_ignore_ascii_case("$port_connected") =>
            {
                out.push(
                    identifier(args[0])
                        .and_then(|name| hir.ports.iter().position(|port| port.name == name))
                        .and_then(|index| u32::try_from(index).ok())
                        .map_or(Op::Unknown, Op::PortConnected),
                );
            }
            HirExprKind::Binary { op, left, right } => {
                let op = match op.as_str() {
                    "Add" => Op::Add,
                    "Sub" => Op::Subtract,
                    "Mul" => Op::Multiply,
                    "Eq" => Op::Equal,
                    "Ne" => Op::NotEqual,
                    "Lt" => Op::Less,
                    "Le" => Op::LessEqual,
                    "Gt" => Op::Greater,
                    "Ge" => Op::GreaterEqual,
                    "And" => Op::And,
                    "Or" => Op::Or,
                    // Division and the rest depend on operand types the
                    // program does not carry.
                    _ => {
                        out.push(Op::Unknown);
                        return;
                    }
                };
                self.push_value(*left, values, out);
                self.push_value(*right, values, out);
                out.push(op);
            }
            HirExprKind::Unary { op, operand } => {
                let op = match op.as_str() {
                    "Pos" => None,
                    "Neg" => Some(Op::Negate),
                    "Not" => Some(Op::Not),
                    "ToInteger" => Some(Op::Integer),
                    _ => {
                        out.push(Op::Unknown);
                        return;
                    }
                };
                self.push_value(*operand, values, out);
                out.extend(op);
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => {
                self.push_value(*condition, values, out);
                self.push_value(*then_expr, values, out);
                self.push_value(*else_expr, values, out);
                out.push(Op::Select);
            }
            _ => out.push(Op::Unknown),
        }
    }
}

/// The expressions directly under one.
fn children(kind: &HirExprKind) -> Vec<ExprId> {
    match kind {
        HirExprKind::Call { args, .. } | HirExprKind::SystemFunction { args, .. } => args.clone(),
        HirExprKind::Binary { left, right, .. } => vec![*left, *right],
        HirExprKind::Unary { operand, .. } | HirExprKind::ArrayIndex { operand, .. } => {
            vec![*operand]
        }
        HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } => vec![*condition, *then_expr, *else_expr],
        HirExprKind::ArrayAccess { index, .. } => vec![*index],
        HirExprKind::ArrayLiteral { elements, .. } => elements.clone(),
        HirExprKind::NoiseSource { operands, .. } => operands.clone(),
        HirExprKind::AnalogOperator {
            op:
                super::HirAnalogOperator::Limit {
                    proposed,
                    candidate,
                    type_metadata,
                    ..
                },
        } => [Some(*proposed), Some(*candidate), *type_metadata]
            .into_iter()
            .flatten()
            .collect(),
        HirExprKind::AnalogOperator { .. }
        | HirExprKind::BranchAccess { .. }
        | HirExprKind::NamedBranchAccess { .. }
        | HirExprKind::NullArgument
        | HirExprKind::Number { .. }
        | HirExprKind::StringLiteral { .. }
        | HirExprKind::Identifier { .. } => Vec::new(),
    }
}

/// One program for the conjunction of an access's guards; empty when it has
/// none.
fn conjunction<'a>(guards: &[Program<'a>]) -> Program<'a> {
    let mut program = Vec::new();
    for (index, conjunct) in guards.iter().enumerate() {
        program.extend_from_slice(conjunct);
        if index > 0 {
            program.push(GroundGuardOp::And);
        }
    }
    bounded(program)
}

fn bounded(program: Program<'_>) -> Program<'_> {
    if program.len() > MAX_GUARD_OPS {
        vec![GroundGuardOp::Unknown]
    } else {
        program
    }
}

/// The variables a region list assigns, at any depth, in first-seen order.
fn assigned_in<'a>(regions: &'a [HirRegion], out: &mut Vec<&'a str>) {
    for region in regions {
        match region {
            HirRegion::Assignment(assignment) => {
                let name = assignment.target_name.as_str();
                if !out.contains(&name) {
                    out.push(name);
                }
            }
            HirRegion::Conditional {
                then_body,
                else_body,
                ..
            } => {
                assigned_in(then_body, out);
                assigned_in(else_body, out);
            }
            HirRegion::Loop { body, .. } | HirRegion::Initialization { body, .. } => {
                assigned_in(body, out);
            }
            HirRegion::Contribution(_) | HirRegion::Task(_) => {}
        }
    }
}

/// Whether a guard reads anything of the instance's.
fn reads_instance(program: &[GroundGuardOp<'_>]) -> bool {
    program.iter().any(|op| {
        matches!(
            op,
            GroundGuardOp::Parameter(_)
                | GroundGuardOp::ParameterGiven(_)
                | GroundGuardOp::PortConnected(_)
        )
    })
}

/// The groups a module's nets fall into, with ground as group 0.
struct Groups<'a> {
    index: HashMap<&'a str, u32>,
    parent: Vec<u32>,
}

impl<'a> Groups<'a> {
    fn of(&mut self, net: Option<&'a str>) -> u32 {
        let Some(net) = net else { return 0 };
        let next = u32::try_from(self.parent.len()).expect("a module has fewer than 2^32 nets");
        let group = *self.index.entry(net).or_insert(next);
        if group == next {
            self.parent.push(next);
        }
        group
    }

    fn root(&mut self, mut group: u32) -> u32 {
        while self.parent[group as usize] != group {
            let next = self.parent[self.parent[group as usize] as usize];
            self.parent[group as usize] = next;
            group = next;
        }
        group
    }

    fn join(&mut self, left: u32, right: u32) {
        let (left, right) = (self.root(left), self.root(right));
        let (low, high) = (left.min(right), left.max(right));
        self.parent[high as usize] = low;
    }
}

/// Make every join that always happens, and keep the ones an instance has to
/// decide.
fn summarize<'a>(hir: &'a HirModel, joins: Vec<RawJoin<'a>>) -> GroundReferenceSummary<'a> {
    struct NoInstance;
    impl rspice_veriloga_runtime::GroundReferenceInputs for NoInstance {
        fn parameter(&self, _: &str) -> Option<f64> {
            None
        }
        fn parameter_given(&self, _: &str) -> Option<bool> {
            None
        }
        fn port_connected(&self, _: u32) -> Option<bool> {
            None
        }
    }

    let mut groups = Groups {
        index: HashMap::new(),
        parent: vec![0],
    };
    let mut decided = Vec::new();
    for join in joins {
        let ends = [groups.of(join.ends[0]), groups.of(join.ends[1])];
        let guard = conjunction(&join.guards);
        if guard.is_empty() {
            groups.join(ends[0], ends[1]);
            continue;
        }
        if !reads_instance(&guard) {
            // Fixed by the module alone.
            match rspice_veriloga_runtime::evaluate_ground_guard(&guard, &NoInstance) {
                Some(false) => {}
                Some(true) | None => groups.join(ends[0], ends[1]),
            }
            continue;
        }
        decided.push((ends, guard));
    }
    let ports: Vec<u32> = hir
        .ports
        .iter()
        .filter(|port| {
            port.nature_potential.as_deref() == Some("Voltage")
                || port.nature_flow.as_deref() == Some("Current")
        })
        .filter_map(|port| groups.index.get(port.name.as_str()).copied())
        .collect();
    let ground = groups.root(0);
    let mut port_roots: Vec<u32> = ports.iter().map(|&port| groups.root(port)).collect();
    let fixed = |kind| GroundReferenceSummary {
        kind,
        group_count: 0,
        port_groups: Vec::new(),
        joins: Vec::new(),
        guards: Vec::new(),
    };
    if port_roots.contains(&ground) {
        return fixed(SummaryKind::Always);
    }
    let decided: Vec<_> = decided
        .into_iter()
        .map(|([left, right], guard)| ([groups.root(left), groups.root(right)], guard))
        .filter(|([left, right], _)| left != right)
        .collect();
    // Whether every remaining join together reaches ground.
    let mut all = Groups {
        index: HashMap::new(),
        parent: groups.parent.clone(),
    };
    for ([left, right], _) in &decided {
        all.join(*left, *right);
    }
    let ground_all = all.root(0);
    if !port_roots.iter().any(|&port| all.root(port) == ground_all) {
        return fixed(SummaryKind::Never);
    }
    // Number the groups the remaining joins and ports use, ground first.
    let mut numbering: Vec<u32> = vec![ground];
    let mut number = |root: u32| match numbering.iter().position(|&seen| seen == root) {
        Some(index) => index as u32,
        None => {
            numbering.push(root);
            (numbering.len() - 1) as u32
        }
    };
    let mut summary_joins = Vec::with_capacity(decided.len());
    let mut guards = Vec::new();
    for ([left, right], guard) in decided {
        let start = u32::try_from(guards.len()).expect("guard storage fits u32");
        let len = u32::try_from(guard.len()).expect("guard fits u32");
        guards.extend(guard);
        summary_joins.push(GroundJoin {
            groups: [number(left), number(right)],
            guard: [start, len],
        });
    }
    port_roots.dedup();
    let mut port_groups: Vec<u32> = port_roots.into_iter().map(&mut number).collect();
    port_groups.sort_unstable();
    port_groups.dedup();
    GroundReferenceSummary {
        kind: SummaryKind::Conditional,
        group_count: u32::try_from(numbering.len()).expect("group count fits u32"),
        port_groups,
        joins: summary_joins,
        guards,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_veriloga_runtime::GroundReferenceInputs;

    struct Instance<'a> {
        parameters: &'a [(&'a str, f64)],
        connected: bool,
    }

    /// The card's values over the module's defaults, `mode = 0` and `r = 1k`.
    impl GroundReferenceInputs for Instance<'_> {
        fn parameter(&self, name: &str) -> Option<f64> {
            self.parameters
                .iter()
                .chain(&[("mode", 0.0), ("r", 1.0e3)])
                .find(|(key, _)| *key == name)
                .map(|(_, value)| *value)
        }
        fn parameter_given(&self, name: &str) -> Option<bool> {
            Some(self.parameters.iter().any(|(key, _)| *key == name))
        }
        fn port_connected(&self, _: u32) -> Option<bool> {
            Some(self.connected)
        }
    }

    fn artifact(body: &str) -> CanonicalIrArtifact {
        let source = format!(
            "module m(p, n, t);\ninout p, n, t; electrical p, n; thermal t;\n\
             parameter integer mode = 0; parameter real r = 1k;\n{body}\nendmodule\n"
        );
        crate::VerilogACompiler::default()
            .compile_canonical_ir(&source)
            .unwrap_or_else(|error| panic!("{source}: {error:?}"))
    }

    fn references_ground(body: &str, parameters: &[(&str, f64)], connected: bool) -> bool {
        let artifact = artifact(body);
        ground_reference(&artifact)
            .view()
            .references_ground(&Instance {
                parameters,
                connected,
            })
    }

    /// A contribution that reaches a port single-ended, or through a `ground`
    /// net, connects node 0; a differential one does not, and a read of a
    /// port single-ended, which stamps nothing at node 0, does not either.
    #[test]
    fn a_module_connects_node_zero_through_the_branches_it_stamps_only() {
        for (body, expected) in [
            ("analog I(p, n) <+ V(p, n) / r;", false),
            ("analog V(p) <+ 1.0;", true),
            ("analog I(n) <+ 1e-3;", true),
            // Reads: in a contribution, an assignment, a condition and a
            // derivative's axis.
            ("analog I(p, n) <+ V(p) / r;", false),
            (
                "real v; analog begin I(p, n) <+ V(p, n) / r; v = V(p); end",
                false,
            ),
            (
                "analog begin I(p, n) <+ V(p, n) / r; if (V(n) > 0.5) I(p, n) <+ 1m; end",
                false,
            ),
            (
                "real g; analog begin I(p, n) <+ V(p, n) / r; g = ddx(V(p, n) * V(p, n), V(p)); end",
                false,
            ),
            (
                "electrical g; ground g; analog I(p, g) <+ V(p, g) / r;",
                true,
            ),
            // A chain of differential branches through an internal node
            // still ties the port to ground.
            (
                "electrical x; analog begin I(p, x) <+ V(p, x) / r; V(x) <+ 0; end",
                true,
            ),
            // A private state node: its own single-ended equation, read
            // differentially from the ports.
            (
                "electrical q; analog begin I(q) <+ V(q) - V(p, n); I(p, n) <+ V(q) / r; end",
                false,
            ),
            // Thermal ground is ambient, not node 0.
            ("analog Pwr(t) <+ V(p, n) * V(p, n) / r;", false),
            // A port's own flow is the current through it, not to ground.
            (
                "real i; analog begin I(p, n) <+ V(p, n) / r; i = I(<p>); end",
                false,
            ),
        ] {
            assert_eq!(references_ground(body, &[], true), expected, "{body}");
        }
    }

    /// A contribution under an `if` the instance's parameters decide counts
    /// only when they select it; one the solve decides always counts.
    #[test]
    fn a_guarded_contribution_counts_when_the_instances_values_select_it() {
        let parameter = "analog begin I(p, n) <+ V(p, n) / r; if (mode == 2) V(n) <+ 0; end";
        assert!(!references_ground(parameter, &[], true));
        assert!(!references_ground(parameter, &[("mode", 1.0)], true));
        assert!(references_ground(parameter, &[("mode", 2.0)], true));
        // Through a variable set from a parameter, and in the else branch.
        let variable = "integer m; analog begin m = mode; I(p, n) <+ V(p, n) / r; \
                        if (m != 2) I(p, n) <+ 0; else V(n) <+ 0; end";
        assert!(!references_ground(variable, &[("mode", 1.0)], true));
        assert!(references_ground(variable, &[("mode", 2.0)], true));
        // A real rounded into an integer variable.
        let rounded = "integer m; analog begin m = r; I(p, n) <+ V(p, n) / r; \
                       if (m == 3) V(n) <+ 0; end";
        assert!(references_ground(rounded, &[("r", 2.6)], true));
        assert!(!references_ground(rounded, &[("r", 2.4)], true));
        // `$port_connected` and `$param_given`.
        let connected = "analog begin I(p, n) <+ V(p, n) / r; \
                         if ($port_connected(n) && !$param_given(mode)) V(n) <+ 0; end";
        assert!(references_ground(connected, &[], true));
        assert!(!references_ground(connected, &[], false));
        assert!(!references_ground(connected, &[("mode", 0.0)], true));
        // The solve decides this one, so it counts.
        let solved = "analog begin I(p, n) <+ V(p, n) / r; if (V(p, n) > 1) V(n) <+ 0; end";
        assert!(references_ground(solved, &[("mode", 0.0)], true));
        // A variable an `if` assigns in both branches is the selection of
        // the two.
        let merged = "integer m; analog begin if (r > 5) m = 1; else m = 2; \
                      I(p, n) <+ V(p, n) / r; if (m == 2) V(n) <+ 0; end";
        assert!(references_ground(merged, &[("r", 1.0)], true));
        assert!(!references_ground(merged, &[("r", 10.0)], true));
    }

    /// A branch the compiler lowers onto a private state, because its flow is
    /// probed or its sources switch between potential and flow, connects node
    /// 0 under its authored sources' conditions, not the plumbing's.
    #[test]
    fn a_lowered_branch_counts_under_its_authored_sources_conditions() {
        let probed = "real i; analog begin I(p, n) <+ V(p, n) / r; \
                      if (mode == 2) I(n) <+ 1m; i = I(n); end";
        assert!(!references_ground(probed, &[], true));
        assert!(references_ground(probed, &[("mode", 2.0)], true));
        let switched = "analog begin I(p, n) <+ V(p, n) / r; \
                        if (mode == 2) V(n) <+ 0; if (mode == 3) I(n) <+ 1m; end";
        assert!(!references_ground(switched, &[], true));
        assert!(references_ground(switched, &[("mode", 2.0)], true));
        assert!(references_ground(switched, &[("mode", 3.0)], true));
        // A probe of a branch with no source is an ideal short.
        let ammeter = "real i; analog begin I(p, n) <+ V(p, n) / r; i = I(n); end";
        assert!(references_ground(ammeter, &[], true));
        // A task observes; it is not an equation.
        let strobe = "analog begin I(p, n) <+ V(p, n) / r; $strobe(\"%g\", V(p)); end";
        assert!(!references_ground(strobe, &[], true));
    }

    /// A Verilog-AMS process that samples a port, single-ended or against
    /// another port, observes it and stamps nothing: it does not connect node
    /// 0. A contribution beside it does.
    #[test]
    fn a_process_sampling_a_port_does_not_connect_node_zero() {
        for (sample, analog, expected) in [
            ("V(p)", "", false),
            ("V(p, n)", "", false),
            ("V(p)", "analog V(p) <+ 0.5;", true),
        ] {
            let source = format!(
                "`timescale 1ns/1ps\nmodule m(p, n, q);\n inout p, n; electrical p, n;\n \
                 output q; reg q; real v;\n initial q = 1'b0;\n \
                 always #5 begin v = {sample}; q = v > 0.5; end\n {analog}\nendmodule\n"
            );
            let artifact = crate::VerilogACompiler::default()
                .compile_canonical_ir(&source)
                .unwrap_or_else(|error| panic!("{source}: {error:?}"));
            let reference = ground_reference(&artifact);
            assert_eq!(
                reference.view().references_ground(&Instance {
                    parameters: &[],
                    connected: true
                }),
                expected,
                "{source}"
            );
        }
    }
}
