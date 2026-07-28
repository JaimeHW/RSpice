//! Lowering the structured HIR body into a [`CfgFunction`].
//!
//! This is the pass that replaces guard flattening. Where the OptIR lowering
//! reconstructs "the value a variable had before this `if`" by searching an
//! assignment history, this one asks the SSA builder — which knows, exactly,
//! because it never threw the block structure away.
//!
//! ## Contributions are accumulators
//!
//! `I(a,b) <+ x` becomes `residual += x` against a per-contribution SSA
//! variable, so a contribution inside an `if` is an update on one edge of a
//! diamond and the join merges it with the untouched running total. That is
//! both what a hand-written model does and what makes the "contributes zero
//! when the branch is not taken" rule fall out of ordinary SSA rather than
//! needing a rule of its own.
//!
//! ## Conditional expressions become diamonds
//!
//! `c ? a : b` lowers to a branch, because [`CfgValueKind`] has no select and
//! because evaluating only the taken side is the entire point. `min`/`max` are
//! the deliberate exception: they stay single operations, since a diamond per
//! `min` in a BSIM-class model would swamp the block count for no gain.
//!
//! ## Reporting rather than guessing
//!
//! A construct this pass cannot yet lower produces a diagnostic and a zero
//! placeholder, and the walk continues. That yields the complete inventory of
//! what a model still needs in one run instead of one construct per run, and it
//! is the reason [`CfgModel::from_hir`] returns every diagnostic it found rather
//! than the first.

use smol_str::SmolStr;
use std::collections::{HashMap, HashSet};

use super::cfg::{
    CfgBinaryOp, CfgFunction, CfgTerminator, CfgUnaryOp, CfgValueKind, CfgValueType, CfgVariable,
    SsaBuilder,
};
use super::hir::{
    HirAnalogOperator, HirContribution, HirExprKind, HirExpression, HirLimiterArgument, HirModel,
    HirRegion,
};
use super::mir::{MirEquationKind, MirModel};
use super::{
    BlockId, BranchId, BranchUnknownId, CompilerPhase, ContributionId, DiagnosticSeverity, ExprId,
    IrDiagnostic, NodeId, ParamId, SourceSpanRef, ValueId, VariableId,
};

/// A module lowered to a single control-flow graph.
#[derive(Debug, Clone, PartialEq)]
pub struct CfgModel {
    pub module_name: SmolStr,
    pub function: CfgFunction,
    /// The accumulated residual of each contribution at the exit block, indexed
    /// by [`ContributionId`]. Parallel to `MirModel::equations`.
    pub residuals: Vec<ValueId>,
    /// Everything the lowering wanted to say that did not stop it. Carried on
    /// the model rather than dropped, because a model that lowered *and* warned
    /// is exactly the case worth surfacing.
    pub warnings: Vec<IrDiagnostic>,
}

impl CfgModel {
    /// Lower `hir`'s structured body, using `mir` for name-to-id resolution.
    ///
    /// Both are needed: the body lives in HIR because that is where control
    /// flow survives, and node, branch, and branch-unknown identity is settled
    /// in MIR. Nothing is recomputed here that MIR already decided.
    pub fn from_hir(hir: &HirModel, mir: &MirModel) -> Result<Self, Vec<IrDiagnostic>> {
        let mut lowerer = CfgLowerer::new(hir, mir);
        let (function, residuals) = lowerer.lower()?;
        // Errors only. A warning that failed the lowering would be an error
        // wearing a different word.
        if lowerer
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
        {
            return Err(lowerer.diagnostics);
        }
        Ok(Self {
            module_name: hir.module_name.clone(),
            function,
            residuals,
            warnings: lowerer.diagnostics,
        })
    }
}

/// A leaf value's identity, for interning.
///
/// Constants and unknowns are read thousands of times in a BSIM-class model and
/// there is no reason for each read to define a new value; the derivative pass
/// would then carry thousands of identical lanes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum LeafKey {
    RealConstant(u64),
    Parameter(ParamId),
    ParameterGiven(ParamId),
    Temperature,
    ThermalVoltage,
    Multiplicity,
    Time,
    Analysis(SmolStr),
    NodePotential(NodeId),
    BranchFlow(BranchId),
    BranchUnknownFlow(BranchUnknownId),
}

struct CfgLowerer<'a> {
    hir: &'a HirModel,
    mir: &'a MirModel,
    builder: SsaBuilder,
    /// Where instructions are appended right now. Moves as blocks are created,
    /// which is why nothing may assume it still equals the block it started in.
    block: BlockId,
    variables_by_name: HashMap<SmolStr, VariableId>,
    parameters_by_name: HashMap<SmolStr, ParamId>,
    nodes_by_name: HashMap<SmolStr, NodeId>,
    ground_names: HashSet<SmolStr>,
    leaves: HashMap<LeafKey, ValueId>,
    /// How many conditional-expression results have been given SSA keys.
    temporary_count: usize,
    /// `$limit` calls whose inlined body is currently being walked.
    limiters: Vec<Limiter>,
    diagnostics: Vec<IrDiagnostic>,
}

/// One `$limit` call, while its body is in scope.
#[derive(Debug, Clone, Copy)]
struct Limiter {
    operator: ExprId,
    proposed: ValueId,
}

impl<'a> CfgLowerer<'a> {
    fn new(hir: &'a HirModel, mir: &'a MirModel) -> Self {
        let mut ground_names: HashSet<SmolStr> = mir.ground_nodes.iter().cloned().collect();
        ground_names.insert(SmolStr::new("0"));

        Self {
            hir,
            mir,
            builder: SsaBuilder::new(),
            block: BlockId::from(0usize),
            variables_by_name: hir
                .variables
                .iter()
                .map(|variable| (variable.name.clone(), variable.id))
                .collect(),
            parameters_by_name: hir
                .parameters
                .iter()
                .map(|parameter| (parameter.name.clone(), parameter.id))
                .collect(),
            nodes_by_name: mir
                .nodes
                .iter()
                .map(|node| (node.name.clone(), node.id))
                .collect(),
            ground_names,
            leaves: HashMap::new(),
            temporary_count: 0,
            limiters: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    fn lower(&mut self) -> Result<(CfgFunction, Vec<ValueId>), Vec<IrDiagnostic>> {
        let entry = self.builder.create_block();
        self.builder.seal_block(entry);
        self.block = entry;

        // Every residual starts at zero so an untaken branch needs no special
        // case: the join simply merges the value that was never updated.
        let zero = self.real_constant(0.0);
        for index in 0..self.hir.contributions.len() {
            self.builder.write_variable(
                CfgVariable::Residual(ContributionId::from(index)),
                entry,
                zero,
            );
        }

        let body = self.hir.body.clone();
        self.regions(&body);

        let exit = self.block;
        let residuals: Vec<_> = (0..self.hir.contributions.len())
            .map(|index| {
                let variable = CfgVariable::Residual(ContributionId::from(index));
                self.builder.read_variable(variable, exit).unwrap_or(zero)
            })
            .collect();
        self.builder.set_terminator(exit, CfgTerminator::Return);

        // Through `finish_with_outputs`, because finishing renumbers values and
        // a residual id read out beforehand names something else afterwards.
        let builder = std::mem::take(&mut self.builder);
        match builder.finish_with_outputs(entry, &residuals) {
            Ok((function, residuals)) => Ok((function, residuals)),
            Err(error) => Err(vec![IrDiagnostic::global_error(
                CompilerPhase::CfgLowering,
                format!("CFG construction produced an invalid function: {error}"),
            )]),
        }
    }

    fn regions(&mut self, regions: &[HirRegion]) {
        for region in regions {
            self.region(region);
        }
    }

    fn region(&mut self, region: &HirRegion) {
        match region {
            HirRegion::Assignment(assignment) => {
                if assignment.index.is_some() {
                    self.unsupported(
                        assignment.span,
                        format!(
                            "assignment to '{}' at a run-time array index",
                            assignment.target_name
                        ),
                    );
                    return;
                }
                let value = self.expr(assignment.expr.id);
                self.builder.write_variable(
                    CfgVariable::Local(assignment.target),
                    self.block,
                    value,
                );
            }
            HirRegion::Contribution(contribution) => self.contribution(contribution),
            HirRegion::Conditional {
                condition,
                then_body,
                else_body,
                ..
            } => self.conditional(condition.id, then_body, else_body),
            HirRegion::Loop {
                condition, body, ..
            } => self.runtime_loop(condition.id, body),
        }
    }

    fn contribution(&mut self, contribution: &HirContribution) {
        let value = self.expr(contribution.expression.id);
        let variable = CfgVariable::Residual(contribution.id);
        let accumulated = match self.builder.read_variable(variable, self.block) {
            Some(accumulated) => accumulated,
            None => self.real_constant(0.0),
        };
        let sum = self.binary(CfgBinaryOp::Add, accumulated, value);
        self.builder.write_variable(variable, self.block, sum);
    }

    fn conditional(&mut self, condition: ExprId, then_body: &[HirRegion], else_body: &[HirRegion]) {
        let condition = self.expr(condition);
        let then_block = self.builder.create_block();
        let else_block = self.builder.create_block();
        let join = self.builder.create_block();

        // The condition may itself have been a diamond, so the branch belongs
        // on wherever lowering it ended up, not on the block it started in.
        self.builder.set_terminator(
            self.block,
            CfgTerminator::Branch {
                condition,
                then_target: then_block,
                then_args: Vec::new(),
                else_target: else_block,
                else_args: Vec::new(),
            },
        );
        self.builder.seal_block(then_block);
        self.builder.seal_block(else_block);

        self.block = then_block;
        self.regions(then_body);
        self.builder.set_terminator(
            self.block,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );

        self.block = else_block;
        self.regions(else_body);
        self.builder.set_terminator(
            self.block,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );

        self.builder.seal_block(join);
        self.block = join;
    }

    /// `while (condition) body`, as a header that both the entry edge and the
    /// back edge reach.
    ///
    /// The header stays unsealed until the back edge exists. That is the one
    /// place incremental SSA construction needs the delay, and getting it wrong
    /// shows up as a loop-carried variable reading its initial value forever.
    fn runtime_loop(&mut self, condition: ExprId, body: &[HirRegion]) {
        let header = self.builder.create_block();
        self.builder.set_terminator(
            self.block,
            CfgTerminator::Jump {
                target: header,
                args: Vec::new(),
            },
        );

        self.block = header;
        let condition = self.expr(condition);
        let test = self.block;

        let body_block = self.builder.create_block();
        let exit = self.builder.create_block();
        self.builder.set_terminator(
            test,
            CfgTerminator::Branch {
                condition,
                then_target: body_block,
                then_args: Vec::new(),
                else_target: exit,
                else_args: Vec::new(),
            },
        );
        self.builder.seal_block(body_block);

        self.block = body_block;
        self.regions(body);
        self.builder.set_terminator(
            self.block,
            CfgTerminator::Jump {
                target: header,
                args: Vec::new(),
            },
        );

        self.builder.seal_block(header);
        self.builder.seal_block(exit);
        self.block = exit;
    }

    fn expr(&mut self, id: ExprId) -> ValueId {
        let Some(expression) = self.hir.expressions.get(usize::from(id)).cloned() else {
            self.diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::CfgLowering,
                format!("CFG lowering found expression id {id} outside the arena"),
            ));
            return self.real_constant(0.0);
        };
        self.expr_kind(&expression)
    }

    fn expr_kind(&mut self, expression: &HirExpression) -> ValueId {
        let span = expression.span;
        match &expression.kind {
            HirExprKind::Number { value, .. } => self.real_constant(*value),
            HirExprKind::Identifier { name } => self.identifier(name, span),
            HirExprKind::Binary { op, left, right } => self.binary_expr(op, *left, *right, span),
            HirExprKind::Unary { op, operand } => self.unary_expr(op, *operand, span),
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => self.conditional_expr(*condition, *then_expr, *else_expr),
            HirExprKind::BranchAccess { access, pos, neg } => {
                self.branch_access(access, pos, neg.as_deref(), span)
            }
            HirExprKind::NamedBranchAccess { access, name } => {
                self.named_branch_access(access, name, span)
            }
            HirExprKind::SystemFunction { name, args } => self.system_function(name, args, span),
            HirExprKind::Call { name, args } => self.call(expression.id, name, args, span),
            HirExprKind::AnalogOperator { op } => self.analog_operator(op, expression, span),
            // Noise sources are lifted into their own plan before codegen and
            // contribute nothing to the time-domain residual, exactly as in the
            // level this replaces.
            HirExprKind::NoiseSource { .. } => self.real_constant(0.0),
            other => {
                self.unsupported(span, format!("{} expression", kind_label(other)));
                self.real_constant(0.0)
            }
        }
    }

    fn identifier(&mut self, name: &SmolStr, span: SourceSpanRef) -> ValueId {
        if let Some(variable) = self.variables_by_name.get(name).copied() {
            // The whole reason this level exists: no history search, no
            // heuristic. The builder either has a reaching definition or the
            // variable is genuinely read before assignment.
            if let Some(value) = self
                .builder
                .read_variable(CfgVariable::Local(variable), self.block)
            {
                return value;
            }
            // Verilog-AMS initialises an analog variable to zero, and released
            // compact models rely on it: `r3_cmc` reads one before assigning it
            // on any path, and every production compiler runs `r3_cmc`. So this
            // is a warning and a zero, not a refusal.
            //
            // It stays a warning rather than becoming silent because the same
            // shape is also how a name-resolution bug shows up — a block local
            // shadowing a parameter of the same name reads as undefined, and
            // zero is the wrong answer to that one.
            self.warn(
                span,
                format!(
                    "'{name}' is read before it is assigned on any path; \
                     Verilog-AMS initialises it to zero"
                ),
            );
            return self.real_constant(0.0);
        }

        if let Some(parameter) = self.parameters_by_name.get(name).copied() {
            return self.leaf(
                LeafKey::Parameter(parameter),
                CfgValueType::Real,
                CfgValueKind::Parameter(parameter),
            );
        }

        self.unsupported(span, format!("identifier '{name}'"));
        self.real_constant(0.0)
    }

    fn binary_expr(
        &mut self,
        op: &SmolStr,
        left: ExprId,
        right: ExprId,
        span: SourceSpanRef,
    ) -> ValueId {
        let Some(op) = binary_op(op.as_str()) else {
            self.unsupported(span, format!("binary operator '{op}'"));
            return self.real_constant(0.0);
        };
        let left = self.expr(left);
        let right = self.expr(right);
        self.binary(op, left, right)
    }

    fn unary_expr(&mut self, op: &SmolStr, operand: ExprId, span: SourceSpanRef) -> ValueId {
        match op.as_str() {
            "Pos" => self.expr(operand),
            "Neg" => {
                let input = self.expr(operand);
                self.unary(CfgUnaryOp::Neg, input)
            }
            "Not" => {
                let input = self.expr(operand);
                self.unary_typed(CfgUnaryOp::Not, input, CfgValueType::Boolean)
            }
            _ => {
                self.unsupported(span, format!("unary operator '{op}'"));
                self.real_constant(0.0)
            }
        }
    }

    /// `c ? a : b` as a diamond, so the untaken side costs nothing.
    fn conditional_expr(
        &mut self,
        condition: ExprId,
        then_expr: ExprId,
        else_expr: ExprId,
    ) -> ValueId {
        let condition = self.expr(condition);
        let then_block = self.builder.create_block();
        let else_block = self.builder.create_block();
        let join = self.builder.create_block();

        self.builder.set_terminator(
            self.block,
            CfgTerminator::Branch {
                condition,
                then_target: then_block,
                then_args: Vec::new(),
                else_target: else_block,
                else_args: Vec::new(),
            },
        );
        self.builder.seal_block(then_block);
        self.builder.seal_block(else_block);

        // The result is carried through an SSA variable of its own, which is
        // what turns the two arms into one block parameter at the join — and
        // into no parameter at all when both arms produce the same value.
        let result = CfgVariable::Local(self.result_variable());

        self.block = then_block;
        let then_value = self.expr(then_expr);
        self.builder.write_variable(result, self.block, then_value);
        self.builder.set_terminator(
            self.block,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );

        self.block = else_block;
        let else_value = self.expr(else_expr);
        self.builder.write_variable(result, self.block, else_value);
        self.builder.set_terminator(
            self.block,
            CfgTerminator::Jump {
                target: join,
                args: Vec::new(),
            },
        );

        self.builder.seal_block(join);
        self.block = join;
        self.builder
            .read_variable(result, join)
            .unwrap_or(then_value)
    }

    /// A fresh SSA key for one conditional expression's result.
    ///
    /// Numbered above every declared variable so it cannot collide with one;
    /// nothing else ever reads it, so it needs no name.
    fn result_variable(&mut self) -> VariableId {
        let id = VariableId::from(self.hir.variables.len() + self.temporary_count);
        self.temporary_count += 1;
        id
    }

    fn branch_access(
        &mut self,
        access: &SmolStr,
        pos: &SmolStr,
        neg: Option<&str>,
        span: SourceSpanRef,
    ) -> ValueId {
        let pos_node = self.endpoint(pos, span);
        let neg_node = match neg {
            Some(neg) => self.endpoint(&SmolStr::new(neg), span),
            None => Ok(None),
        };
        let (Ok(pos_node), Ok(neg_node)) = (pos_node, neg_node) else {
            return self.real_constant(0.0);
        };

        if is_flow_access(access.as_str()) {
            let Some((unknown, reversed)) = self.branch_unknown_by_nodes(pos_node, neg_node) else {
                if let Some(contributed) = self.contributed_flow(pos_node, neg_node) {
                    return contributed;
                }
                // `I(<p>)` reaches here as a single-ended access on a port,
                // because a port branch is not a branch between two nets.
                if let (Some(node), None) = (pos_node, neg_node)
                    && let Some(terminal) = self.port_flow(node)
                {
                    return terminal;
                }
                self.unsupported(
                    span,
                    format!("flow access '{access}' on a branch with nothing driving it"),
                );
                return self.real_constant(0.0);
            };
            let flow = self.leaf(
                LeafKey::BranchUnknownFlow(unknown),
                CfgValueType::Real,
                CfgValueKind::BranchUnknownFlow(unknown),
            );
            return if reversed {
                self.unary(CfgUnaryOp::Neg, flow)
            } else {
                flow
            };
        }

        self.potential_difference(pos_node, neg_node)
    }

    /// The current already contributed to a branch, at this point in the walk.
    ///
    /// A branch with no unknown is one nothing solves for: it is driven by flow
    /// contributions, so its current is not a variable but the running sum of
    /// what the model has put into it. That sum is already in hand — it is the
    /// residual accumulator each `<+` writes — so a probe reads the accumulator
    /// rather than needing a mechanism of its own, and it reads it *here*, which
    /// is what makes `I(di, si)` after the contributions see them and the same
    /// probe before them see zero. Both are what the sequential semantics of an
    /// analog block say.
    ///
    /// This is how the models that read back their own terminal currents work:
    /// ASM-HEMT's `idisi = mult_i*I(di,si)` under "output info variables",
    /// HICUM's and BJT505's operating-point sections. Seven of the nine models
    /// that did not lower were this one construct.
    fn contributed_flow(&mut self, pos: Option<NodeId>, neg: Option<NodeId>) -> Option<ValueId> {
        let contributions: Vec<(ContributionId, bool)> = self
            .mir
            .equations
            .iter()
            .filter(|equation| equation.kind == MirEquationKind::Current)
            .filter_map(|equation| {
                let branch = &equation.branch;
                if branch.pos_node == pos && branch.neg_node == neg {
                    return Some((equation.contribution, false));
                }
                if branch.pos_node == neg && branch.neg_node == pos {
                    return Some((equation.contribution, true));
                }
                None
            })
            .collect();
        if contributions.is_empty() {
            return None;
        }

        let mut total: Option<ValueId> = None;
        for (contribution, reversed) in contributions {
            let variable = CfgVariable::Residual(contribution);
            let accumulated = self.builder.read_variable(variable, self.block)?;
            let term = if reversed {
                self.unary(CfgUnaryOp::Neg, accumulated)
            } else {
                accumulated
            };
            total = Some(match total {
                Some(total) => self.binary(CfgBinaryOp::Add, total, term),
                None => term,
            });
        }
        total
    }

    /// The current entering the module at a terminal — `I(<p>)`.
    ///
    /// A port branch is not a branch between two nets, so nothing above finds
    /// it. What it names is Kirchhoff's law at the terminal: whatever comes in
    /// from outside is what the model's own network takes away, so the answer is
    /// the signed sum of everything incident on that node — flow contributions
    /// through their accumulators, and potential contributions through the
    /// unknown that carries their current.
    ///
    /// Restricted to external nodes. `I(<n>)` for an internal net is not a port
    /// access, and answering it with a KCL sum would turn a mistyped net into a
    /// plausible number.
    ///
    /// A branch driven by potential contributions from both arms of an `if` has
    /// two unknowns for one physical current — see [`Self::branch_unknown_by_nodes`]
    /// — so the unknowns are taken one per distinct branch, or the terminal
    /// current would double-count.
    fn port_flow(&mut self, node: NodeId) -> Option<ValueId> {
        if !self
            .mir
            .nodes
            .get(usize::from(node))
            .is_some_and(|entry| entry.is_external)
        {
            return None;
        }

        // A local copy of the shared reference, so walking MIR does not borrow
        // `self` while the builder is being written.
        let mir = self.mir;
        let mut terms: Vec<(ValueId, bool)> = Vec::new();
        for equation in &mir.equations {
            if equation.kind != MirEquationKind::Current {
                continue;
            }
            let reversed = if equation.branch.pos_node == Some(node) {
                false
            } else if equation.branch.neg_node == Some(node) {
                true
            } else {
                continue;
            };
            let variable = CfgVariable::Residual(equation.contribution);
            let accumulated = self.builder.read_variable(variable, self.block)?;
            terms.push((accumulated, reversed));
        }

        let mut seen: HashSet<(Option<NodeId>, Option<NodeId>)> = HashSet::new();
        for unknown in &mir.branch_unknowns {
            let reversed = if unknown.pos_node == Some(node) {
                false
            } else if unknown.neg_node == Some(node) {
                true
            } else {
                continue;
            };
            if !seen.insert((unknown.pos_node, unknown.neg_node)) {
                continue;
            }
            let flow = self.leaf(
                LeafKey::BranchUnknownFlow(unknown.id),
                CfgValueType::Real,
                CfgValueKind::BranchUnknownFlow(unknown.id),
            );
            terms.push((flow, reversed));
        }

        let mut total: Option<ValueId> = None;
        for (term, reversed) in terms {
            let term = if reversed {
                self.unary(CfgUnaryOp::Neg, term)
            } else {
                term
            };
            total = Some(match total {
                Some(total) => self.binary(CfgBinaryOp::Add, total, term),
                None => term,
            });
        }
        total
    }

    fn named_branch_access(
        &mut self,
        access: &SmolStr,
        name: &SmolStr,
        span: SourceSpanRef,
    ) -> ValueId {
        let Some(branch) = self
            .mir
            .branches
            .iter()
            .find(|branch| branch.name.as_str() == name.as_str())
        else {
            self.unsupported(span, format!("access to undeclared branch '{name}'"));
            return self.real_constant(0.0);
        };
        let (pos_node, neg_node, id) = (branch.pos_node, branch.neg_node, branch.id);

        if is_flow_access(access.as_str()) {
            if let Some((unknown, reversed)) = self.branch_unknown_by_nodes(pos_node, neg_node) {
                let flow = self.leaf(
                    LeafKey::BranchUnknownFlow(unknown),
                    CfgValueType::Real,
                    CfgValueKind::BranchUnknownFlow(unknown),
                );
                return if reversed {
                    self.unary(CfgUnaryOp::Neg, flow)
                } else {
                    flow
                };
            }
            // Same situation as the unnamed form, and the same answer: what the
            // model has contributed. The runtime-supplied flow stays as the
            // fallback for a declared branch nothing contributes to.
            if let Some(contributed) = self.contributed_flow(pos_node, neg_node) {
                return contributed;
            }
            return self.leaf(
                LeafKey::BranchFlow(id),
                CfgValueType::Real,
                CfgValueKind::BranchFlow(id),
            );
        }

        self.potential_difference(pos_node, neg_node)
    }

    fn potential_difference(&mut self, pos: Option<NodeId>, neg: Option<NodeId>) -> ValueId {
        let pos = pos.map(|node| {
            self.leaf(
                LeafKey::NodePotential(node),
                CfgValueType::Real,
                CfgValueKind::NodePotential(node),
            )
        });
        let neg = neg.map(|node| {
            self.leaf(
                LeafKey::NodePotential(node),
                CfgValueType::Real,
                CfgValueKind::NodePotential(node),
            )
        });
        match (pos, neg) {
            (Some(pos), Some(neg)) => self.binary(CfgBinaryOp::Sub, pos, neg),
            (Some(pos), None) => pos,
            (None, Some(neg)) => self.unary(CfgUnaryOp::Neg, neg),
            (None, None) => self.real_constant(0.0),
        }
    }

    /// `Ok(None)` is ground; `Err(())` is an endpoint that does not resolve.
    fn endpoint(&mut self, name: &SmolStr, span: SourceSpanRef) -> Result<Option<NodeId>, ()> {
        if self.ground_names.contains(name) {
            return Ok(None);
        }
        match self.nodes_by_name.get(name).copied() {
            Some(node) => Ok(Some(node)),
            None => {
                self.unsupported(span, format!("net '{name}'"));
                Err(())
            }
        }
    }

    /// The branch unknown carrying the flow between two nets, and whether the
    /// probe reads it backwards.
    ///
    /// MIR mints one unknown per potential or indirect *contribution statement*,
    /// so a branch driven from both arms of an `if` — `V(rf,si) <+ ...` in one
    /// and `V(rf,si) <+ 0.0` in the other, which is how Angelov writes an
    /// optional series resistance — produces two unknowns for one branch. A
    /// branch has one flow, so the first is the answer; the duplication is
    /// MIR's modelling of contributions, not two physical quantities.
    fn branch_unknown_by_nodes(
        &self,
        pos: Option<NodeId>,
        neg: Option<NodeId>,
    ) -> Option<(BranchUnknownId, bool)> {
        if let Some(unknown) = self
            .mir
            .branch_unknowns
            .iter()
            .find(|unknown| unknown.pos_node == pos && unknown.neg_node == neg)
        {
            return Some((unknown.id, false));
        }

        let unknown = self
            .mir
            .branch_unknowns
            .iter()
            .find(|unknown| unknown.pos_node == neg && unknown.neg_node == pos)?;
        Some((unknown.id, true))
    }

    fn system_function(&mut self, name: &SmolStr, args: &[ExprId], span: SourceSpanRef) -> ValueId {
        match (name.to_ascii_lowercase().as_str(), args.len()) {
            ("$temperature", 0) => self.leaf(
                LeafKey::Temperature,
                CfgValueType::Real,
                CfgValueKind::Temperature,
            ),
            ("$abstime" | "$realtime", 0) => {
                self.leaf(LeafKey::Time, CfgValueType::Real, CfgValueKind::Time)
            }
            ("$mfactor", 0) => self.leaf(
                LeafKey::Multiplicity,
                CfgValueType::Real,
                CfgValueKind::Multiplicity,
            ),
            ("$vt" | "$thermal_vt", 0) => self.leaf(
                LeafKey::ThermalVoltage,
                CfgValueType::Real,
                CfgValueKind::ThermalVoltage,
            ),
            ("$vt" | "$thermal_vt", 1) => {
                let temperature = self.expr(args[0]);
                let scale = self.real_constant(THERMAL_VOLTAGE_PER_KELVIN);
                self.binary(CfgBinaryOp::Mul, temperature, scale)
            }
            ("$simparam", 1 | 2) => self.simparam(args, span),
            ("$param_given", 1) => match self.parameter_argument(args[0]) {
                Some(parameter) => self.leaf(
                    LeafKey::ParameterGiven(parameter),
                    CfgValueType::Real,
                    CfgValueKind::ParameterGiven(parameter),
                ),
                None => {
                    self.unsupported(span, "$param_given of a non-parameter".to_string());
                    self.real_constant(0.0)
                }
            },
            ("$port_connected", 1) => self.real_constant(1.0),
            _ => {
                self.unsupported(span, format!("system function '{name}'"));
                self.real_constant(0.0)
            }
        }
    }

    fn simparam(&mut self, args: &[ExprId], span: SourceSpanRef) -> ValueId {
        let Some(HirExprKind::StringLiteral { value }) = self
            .hir
            .expressions
            .get(usize::from(args[0]))
            .map(|expression| expression.kind.clone())
        else {
            self.unsupported(span, "$simparam with a non-literal name".to_string());
            return self.real_constant(0.0);
        };
        let fallback = match args.get(1) {
            Some(fallback) => self.expr(*fallback),
            None => self.real_constant(0.0),
        };
        self.builder.push(
            self.block,
            CfgValueType::Real,
            CfgValueKind::SimParam {
                name: SmolStr::new(value.to_ascii_lowercase()),
                fallback,
            },
        )
    }

    fn parameter_argument(&self, expr: ExprId) -> Option<ParamId> {
        let expression = self.hir.expressions.get(usize::from(expr))?;
        let HirExprKind::Identifier { name } = &expression.kind else {
            return None;
        };
        self.parameters_by_name.get(name).copied()
    }

    fn call(
        &mut self,
        expression: ExprId,
        name: &SmolStr,
        args: &[ExprId],
        span: SourceSpanRef,
    ) -> ValueId {
        let lowered = name.to_ascii_lowercase();
        if is_noise_name(lowered.as_str()) {
            return self.real_constant(0.0);
        }
        match (lowered.as_str(), args.len()) {
            // Keyed by the *call*, not by its argument. The other `ddt` path —
            // `HirAnalogOperator::Ddt`, below — always did, and so does the
            // state-slot allocation every backend reads, so keying this one by
            // the operand named a slot that does not exist.
            ("ddt", 1) => {
                let input = self.expr(args[0]);
                self.builder.push(
                    self.block,
                    CfgValueType::Real,
                    CfgValueKind::Ddt {
                        operator: expression,
                        input,
                    },
                )
            }
            ("analysis", 1) => self.analysis_call(args[0], span),
            ("ddx", 2) => self.ddx(args[0], args[1], span),
            ("expm1", 1) => {
                let input = self.expr(args[0]);
                let exp = self.unary(CfgUnaryOp::Exp, input);
                let one = self.real_constant(1.0);
                self.binary(CfgBinaryOp::Sub, exp, one)
            }
            ("log1p", 1) => {
                let input = self.expr(args[0]);
                let one = self.real_constant(1.0);
                let sum = self.binary(CfgBinaryOp::Add, one, input);
                self.unary(CfgUnaryOp::Ln, sum)
            }
            ("min" | "max" | "pow" | "fpow" | "hypot" | "atan2", 2) => {
                let op = match lowered.as_str() {
                    "min" => CfgBinaryOp::Min,
                    "max" => CfgBinaryOp::Max,
                    "pow" | "fpow" => CfgBinaryOp::Pow,
                    _ => {
                        self.unsupported(span, format!("function '{name}'"));
                        return self.real_constant(0.0);
                    }
                };
                let left = self.expr(args[0]);
                let right = self.expr(args[1]);
                self.binary(op, left, right)
            }
            (_, 1) => {
                let Some(op) = unary_intrinsic(lowered.as_str()) else {
                    self.unsupported(span, format!("function '{name}'"));
                    return self.real_constant(0.0);
                };
                let input = self.expr(args[0]);
                self.unary(op, input)
            }
            _ => {
                self.unsupported(span, format!("function '{name}'"));
                self.real_constant(0.0)
            }
        }
    }

    /// `ddx(expr, V(pos, neg))`, left symbolic for the derivative pass.
    fn ddx(&mut self, expr: ExprId, probe: ExprId, span: SourceSpanRef) -> ValueId {
        let Some((pos_node, neg_node)) = self.ddx_probe(probe, span) else {
            return self.real_constant(0.0);
        };
        let value = self.expr(expr);
        self.builder.push(
            self.block,
            CfgValueType::Real,
            CfgValueKind::Ddx {
                value,
                pos_node,
                neg_node,
            },
        )
    }

    /// The node pair a `ddx` probe names. Only a potential probe is meaningful:
    /// a flow probe would be a derivative with respect to a value the solver
    /// does not own as an unknown.
    fn ddx_probe(
        &mut self,
        probe: ExprId,
        span: SourceSpanRef,
    ) -> Option<(Option<NodeId>, Option<NodeId>)> {
        let kind = self
            .hir
            .expressions
            .get(usize::from(probe))
            .map(|expression| expression.kind.clone());
        match kind {
            Some(HirExprKind::BranchAccess { access, pos, neg }) if !is_flow_access(&access) => {
                let pos_node = self.endpoint(&pos, span).ok()?;
                let neg_node = match neg {
                    Some(neg) => self.endpoint(&neg, span).ok()?,
                    None => None,
                };
                Some((pos_node, neg_node))
            }
            Some(HirExprKind::NamedBranchAccess { access, name }) if !is_flow_access(&access) => {
                let branch = self
                    .mir
                    .branches
                    .iter()
                    .find(|branch| branch.name == name)?;
                Some((branch.pos_node, branch.neg_node))
            }
            _ => {
                self.unsupported(span, "a ddx probe that is not a potential".to_string());
                None
            }
        }
    }

    fn analysis_call(&mut self, argument: ExprId, span: SourceSpanRef) -> ValueId {
        let Some(HirExprKind::StringLiteral { value }) = self
            .hir
            .expressions
            .get(usize::from(argument))
            .map(|expression| expression.kind.clone())
        else {
            self.unsupported(span, "analysis() with a non-literal name".to_string());
            return self.real_constant(0.0);
        };
        let name = SmolStr::new(value.to_ascii_lowercase());
        self.leaf(
            LeafKey::Analysis(name.clone()),
            CfgValueType::Boolean,
            CfgValueKind::Analysis(name),
        )
    }

    fn analog_operator(
        &mut self,
        op: &HirAnalogOperator,
        expression: &HirExpression,
        span: SourceSpanRef,
    ) -> ValueId {
        match op {
            HirAnalogOperator::Limexp { expr } => {
                let input = self.expr(*expr);
                self.unary(CfgUnaryOp::LimExp, input)
            }
            HirAnalogOperator::Ddx { expr, probe } => self.ddx(*expr, *probe, span),
            HirAnalogOperator::Ddt { expr, .. } => {
                let input = self.expr(*expr);
                self.builder.push(
                    self.block,
                    CfgValueType::Real,
                    CfgValueKind::Ddt {
                        operator: expression.id,
                        input,
                    },
                )
            }
            HirAnalogOperator::Limit {
                proposed,
                candidate,
                selector,
                ..
            } => {
                let proposed = self.expr(*proposed);
                // The candidate is the limiter's inlined body; its implicit
                // arguments are resolved against this call, so the enclosing
                // operator has to be on the stack while it is walked.
                self.limiters.push(Limiter {
                    operator: expression.id,
                    proposed,
                });
                let candidate = self.expr(*candidate);
                self.limiters.pop();
                self.builder.push(
                    self.block,
                    CfgValueType::Real,
                    CfgValueKind::Limit {
                        operator: expression.id,
                        proposed,
                        candidate,
                        selector: selector.clone(),
                    },
                )
            }
            HirAnalogOperator::LimiterArgument { argument } => {
                let Some(limiter) = self.limiters.last().copied() else {
                    self.unsupported(span, "a limiter argument outside a $limit".to_string());
                    return self.real_constant(0.0);
                };
                match argument {
                    HirLimiterArgument::Proposed => limiter.proposed,
                    HirLimiterArgument::Previous => self.builder.push(
                        self.block,
                        CfgValueType::Real,
                        CfgValueKind::LimitPrevious {
                            operator: limiter.operator,
                            proposed: limiter.proposed,
                        },
                    ),
                }
            }
            other => {
                self.unsupported(span, format!("{} operator", analog_operator_label(other)));
                self.real_constant(0.0)
            }
        }
    }

    fn real_constant(&mut self, value: f64) -> ValueId {
        self.leaf(
            LeafKey::RealConstant(value.to_bits()),
            CfgValueType::Real,
            CfgValueKind::RealConstant(value),
        )
    }

    fn leaf(&mut self, key: LeafKey, value_type: CfgValueType, kind: CfgValueKind) -> ValueId {
        if let Some(value) = self.leaves.get(&key) {
            return *value;
        }
        let value = self.builder.push_leaf(value_type, kind);
        self.leaves.insert(key, value);
        value
    }

    fn unary(&mut self, op: CfgUnaryOp, input: ValueId) -> ValueId {
        self.unary_typed(op, input, CfgValueType::Real)
    }

    fn unary_typed(&mut self, op: CfgUnaryOp, input: ValueId, value_type: CfgValueType) -> ValueId {
        self.builder
            .push(self.block, value_type, CfgValueKind::Unary { op, input })
    }

    fn binary(&mut self, op: CfgBinaryOp, left: ValueId, right: ValueId) -> ValueId {
        let value_type = if is_predicate(op) {
            CfgValueType::Boolean
        } else {
            CfgValueType::Real
        };
        self.builder.push(
            self.block,
            value_type,
            CfgValueKind::Binary { op, left, right },
        )
    }

    fn unsupported(&mut self, span: SourceSpanRef, what: String) {
        self.diagnostics.push(IrDiagnostic::error(
            CompilerPhase::CfgLowering,
            format!("CFG lowering does not support {what}"),
            span,
        ));
    }

    fn warn(&mut self, span: SourceSpanRef, what: String) {
        self.diagnostics
            .push(IrDiagnostic::warning(CompilerPhase::CfgLowering, what, span));
    }
}

const THERMAL_VOLTAGE_PER_KELVIN: f64 = 8.617_333_262e-5;

fn is_noise_name(name: &str) -> bool {
    matches!(
        name.trim_start_matches('$'),
        "white_noise" | "flicker_noise" | "noise_table" | "noise_table_log"
    )
}

fn is_flow_access(access: &str) -> bool {
    matches!(access, "I" | "Pwr" | "F" | "Tau" | "Phi" | "Flow")
}

fn is_predicate(op: CfgBinaryOp) -> bool {
    matches!(
        op,
        CfgBinaryOp::Eq
            | CfgBinaryOp::Ne
            | CfgBinaryOp::Lt
            | CfgBinaryOp::Le
            | CfgBinaryOp::Gt
            | CfgBinaryOp::Ge
            | CfgBinaryOp::And
            | CfgBinaryOp::Or
    )
}

/// HIR spells operators with the AST enum's variant name, not the source token.
fn binary_op(op: &str) -> Option<CfgBinaryOp> {
    Some(match op {
        "Add" => CfgBinaryOp::Add,
        "Sub" => CfgBinaryOp::Sub,
        "Mul" => CfgBinaryOp::Mul,
        "Div" => CfgBinaryOp::Div,
        "Mod" => CfgBinaryOp::Mod,
        "Pow" => CfgBinaryOp::Pow,
        "Eq" => CfgBinaryOp::Eq,
        "Ne" => CfgBinaryOp::Ne,
        "Lt" => CfgBinaryOp::Lt,
        "Le" => CfgBinaryOp::Le,
        "Gt" => CfgBinaryOp::Gt,
        "Ge" => CfgBinaryOp::Ge,
        "And" => CfgBinaryOp::And,
        "Or" => CfgBinaryOp::Or,
        _ => return None,
    })
}

fn unary_intrinsic(name: &str) -> Option<CfgUnaryOp> {
    Some(match name {
        "exp" => CfgUnaryOp::Exp,
        "limexp" => CfgUnaryOp::LimExp,
        "__rspice_limited_exp" => CfgUnaryOp::LimitedExp,
        "ln" | "log" => CfgUnaryOp::Ln,
        "sqrt" => CfgUnaryOp::Sqrt,
        "abs" | "fabs" => CfgUnaryOp::Abs,
        "sin" => CfgUnaryOp::Sin,
        "cos" => CfgUnaryOp::Cos,
        "tan" => CfgUnaryOp::Tan,
        "sinh" => CfgUnaryOp::Sinh,
        "cosh" => CfgUnaryOp::Cosh,
        "tanh" => CfgUnaryOp::Tanh,
        "atan" => CfgUnaryOp::Atan,
        "asinh" => CfgUnaryOp::Asinh,
        "floor" => CfgUnaryOp::Floor,
        "ceil" => CfgUnaryOp::Ceil,
        _ => return None,
    })
}

fn kind_label(kind: &HirExprKind) -> &'static str {
    match kind {
        HirExprKind::Number { .. } => "number",
        HirExprKind::StringLiteral { .. } => "string literal",
        HirExprKind::Identifier { .. } => "identifier",
        HirExprKind::SystemFunction { .. } => "system function",
        HirExprKind::Binary { .. } => "binary",
        HirExprKind::Unary { .. } => "unary",
        HirExprKind::Conditional { .. } => "conditional",
        HirExprKind::Call { .. } => "call",
        HirExprKind::BranchAccess { .. } => "branch access",
        HirExprKind::NamedBranchAccess { .. } => "named branch access",
        HirExprKind::ArrayAccess { .. } => "array access",
        HirExprKind::ArrayLiteral { .. } => "array literal",
        HirExprKind::AnalogOperator { .. } => "analog operator",
        HirExprKind::Laplace { .. } => "laplace",
        HirExprKind::Zi { .. } => "z-transform",
        HirExprKind::NoiseSource { .. } => "noise source",
    }
}

fn analog_operator_label(op: &HirAnalogOperator) -> &'static str {
    match op {
        HirAnalogOperator::Limit { .. } => "$limit",
        HirAnalogOperator::LimiterArgument { .. } => "limiter argument",
        HirAnalogOperator::Ddt { .. } => "ddt",
        HirAnalogOperator::Idt { .. } => "idt",
        HirAnalogOperator::IdtMod { .. } => "idtmod",
        HirAnalogOperator::Ddx { .. } => "ddx",
        HirAnalogOperator::Limexp { .. } => "limexp",
        HirAnalogOperator::Absdelay { .. } => "absdelay",
        HirAnalogOperator::Transition { .. } => "transition",
        HirAnalogOperator::Slew { .. } => "slew",
        HirAnalogOperator::LastCrossing { .. } => "last_crossing",
    }
}
