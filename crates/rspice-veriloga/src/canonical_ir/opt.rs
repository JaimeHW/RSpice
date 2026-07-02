use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use super::{
    BranchId, BranchUnknownId, CanonicalValueType, CompilerPhase, EquationId, ExprId,
    HirAnalogOperator, HirAssignment, HirExprKind, HirLoop, HirModel, HirParamRange, HirStatement,
    IrDiagnostic, IrValidationResult, MirEquation, MirEquationKind, MirModel, NodeId, ParamId,
    ScheduleId, ValueId, VariableId,
};

const MAX_SCALAR_LOOP_UNROLL_ITERATIONS: usize = 1024;
const MAX_SCALAR_BOUNDED_LOOP_UNROLL_ITERATIONS: usize = 128;
const MAX_SCALAR_BOUNDED_LOOP_ASSIGNMENT_EXPANSIONS: usize = 2048;
const MAX_SCALAR_GUARD_HISTORY_RECONSTRUCTION_ENTRIES: usize = 16;
const MAX_SCALAR_CURRENT_PATH_HISTORY_SCAN_ENTRIES: usize = 512;
const MAX_SCALAR_RECENT_HISTORY_RECONSTRUCTION_ENTRIES: usize = 512;
const MAX_SCALAR_HISTORY_RECONSTRUCTION_STEPS: usize = 64_000;
const MAX_SCALAR_EXPRESSION_LOWERING_DEPTH: usize = 2048;
const MAX_SCALAR_ASSIGNMENT_ALIAS_SNAPSHOT_EXPR_NODES: usize = 12;
const MAX_SCALAR_ASSIGNMENT_ALIAS_SNAPSHOT_VARIABLES: usize = 4;
const MAX_SCALAR_CONDITION_REASONING_DEPTH: usize = 128;
const MAX_SCALAR_STRUCTURAL_EQUALITY_DEPTH: usize = 192;
pub(crate) const LIMEXP_MAX: f64 = 5.54062238439351e34;
pub(crate) const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

fn trace_opt_phase(
    enabled: bool,
    module_name: &str,
    phase: &str,
    elapsed: Option<std::time::Duration>,
    value_count: Option<usize>,
) {
    if !enabled {
        return;
    }
    let values = value_count
        .map(|count| format!(", values={count}"))
        .unwrap_or_default();
    if let Some(elapsed) = elapsed {
        eprintln!("OptIR {module_name}: finished {phase} in {elapsed:.2?}{values}");
    } else {
        eprintln!("OptIR {module_name}: starting {phase}{values}");
    }
    use std::io::Write as _;
    let _ = std::io::stderr().flush();
}

fn hir_statement_trace_label(statement: &HirStatement) -> String {
    match statement {
        HirStatement::Assignment(assignment) => format!(
            "assign {} expr={} type={:?} span={}:{}..{}",
            assignment.target_name,
            assignment.expr.id.index(),
            assignment.expr_type,
            assignment.span.source_file_id,
            assignment.span.start,
            assignment.span.end
        ),
        HirStatement::Loop(loop_statement) => format!(
            "loop cond={} body={} span={}:{}..{}",
            loop_statement.condition.id.index(),
            loop_statement.body.len(),
            loop_statement.span.source_file_id,
            loop_statement.span.start,
            loop_statement.span.end
        ),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InvalidationClass {
    InstanceStatic,
    TemperatureStatic,
    TimestepStatic,
    OperatingPointStatic,
    NewtonIteration,
    AcFrequency,
    NoiseFrequency,
    OperatingPointReport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptValueType {
    Real,
    Boolean,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DerivativeLaneKind {
    Node,
    BranchUnknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DerivativeLane {
    pub kind: DerivativeLaneKind,
    pub index: u32,
}

impl DerivativeLane {
    pub const fn node(node: NodeId) -> Self {
        Self {
            kind: DerivativeLaneKind::Node,
            index: node.index(),
        }
    }

    pub const fn branch_unknown(branch_unknown: BranchUnknownId) -> Self {
        Self {
            kind: DerivativeLaneKind::BranchUnknown,
            index: branch_unknown.index(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptUnaryOp {
    Pos,
    Neg,
    Not,
    Exp,
    LimExp,
    LimExpDerivative,
    LimitedExp,
    LimitedExpDerivative,
    Ln,
    Sqrt,
    Abs,
    Sin,
    Cos,
    Tan,
    Sinh,
    Cosh,
    Tanh,
    Atan,
    Asinh,
    Floor,
    Ceil,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OptValueKind {
    RealConstant(f64),
    BooleanConstant(bool),
    Parameter {
        parameter: ParamId,
    },
    ParamGiven {
        parameter: ParamId,
    },
    Temperature,
    ThermalVoltage,
    Multiplicity,
    Time,
    Analysis {
        query: SmolStr,
    },
    Ddx {
        value: ValueId,
        pos_node: Option<NodeId>,
        neg_node: Option<NodeId>,
    },
    Ddt {
        operator: ExprId,
        input: ValueId,
    },
    DdtScale,
    NodePotential {
        node: NodeId,
    },
    BranchFlow {
        branch: BranchId,
    },
    BranchUnknownFlow {
        branch_unknown: BranchUnknownId,
    },
    LoopIndex {
        loop_id: u32,
    },
    CountedSum {
        loop_id: u32,
        count: ValueId,
        initial: ValueId,
        term: ValueId,
    },
    RuntimeLoopVariable {
        loop_id: u32,
        slot: u32,
    },
    RuntimeLoopVariableDerivative {
        loop_id: u32,
        slot: u32,
        lane: DerivativeLane,
    },
    RuntimeLoopResult {
        loop_id: u32,
        slot: u32,
    },
    RuntimeLoopResultDerivative {
        loop_id: u32,
        slot: u32,
        lane: DerivativeLane,
    },
    Unary {
        op: OptUnaryOp,
        input: ValueId,
    },
    Binary {
        op: OptBinaryOp,
        left: ValueId,
        right: ValueId,
    },
    Select {
        condition: ValueId,
        then_value: ValueId,
        else_value: ValueId,
    },
    EquationValue {
        equation: EquationId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptDerivative {
    pub lane: DerivativeLane,
    pub value: ValueId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptValue {
    pub id: ValueId,
    pub value_type: OptValueType,
    pub kind: OptValueKind,
    pub derivatives: Vec<OptDerivative>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptRuntimeLoop {
    pub loop_id: u32,
    pub variables: Vec<OptRuntimeLoopVariable>,
    pub condition: ValueId,
    pub assignments: Vec<OptRuntimeLoopAssignment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptRuntimeLoopVariable {
    pub source: VariableId,
    pub value_type: OptValueType,
    pub initial: ValueId,
    pub variable: ValueId,
    pub result: ValueId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptRuntimeLoopAssignment {
    pub slot: u32,
    pub value: ValueId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptOp {
    ComputeValue { value: ValueId },
    EvaluateEquation { equation: EquationId },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptSchedule {
    pub id: ScheduleId,
    pub invalidation: InvalidationClass,
    pub ops: Vec<OptOp>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptModel {
    pub module_name: SmolStr,
    pub node_count: u32,
    pub parameter_count: u32,
    pub branch_count: u32,
    pub branch_unknown_count: u32,
    pub equation_count: u32,
    pub values: Vec<OptValue>,
    pub runtime_loops: Vec<OptRuntimeLoop>,
    pub schedules: Vec<OptSchedule>,
}

impl OptModel {
    pub fn from_mir(mir: &MirModel) -> Result<Self, Vec<IrDiagnostic>> {
        mir.validate()?;

        Self::from_validated_inputs(None, mir)
    }

    pub fn from_hir_and_mir(hir: &HirModel, mir: &MirModel) -> Result<Self, Vec<IrDiagnostic>> {
        hir.validate()?;
        mir.validate()?;

        Self::from_validated_inputs(Some(hir), mir)
    }

    fn from_validated_inputs(
        hir: Option<&HirModel>,
        mir: &MirModel,
    ) -> Result<Self, Vec<IrDiagnostic>> {
        let trace = std::env::var_os("RSPICE_VERILOGA_CANONICAL_IR_PHASE_TRACE").is_some();
        let mut builder = ScalarGraphBuilder::new(hir, mir);
        if let Some(hir) = hir {
            trace_opt_phase(trace, &mir.module_name, "lower statements", None, None);
            let phase_started = std::time::Instant::now();
            builder.lower_statements(&hir.statements);
            trace_opt_phase(
                trace,
                &mir.module_name,
                "lower statements",
                Some(phase_started.elapsed()),
                Some(builder.values.len()),
            );
            builder.reset_history_reconstruction_budget();
        }
        let mut equation_values = Vec::with_capacity(mir.equations.len());
        trace_opt_phase(trace, &mir.module_name, "lower equations", None, None);
        let phase_started = std::time::Instant::now();
        for equation in &mir.equations {
            let value = builder.lower_equation_expression(equation);
            builder.cache_declared_branch_current(equation, value);
            equation_values.push(value);
        }
        trace_opt_phase(
            trace,
            &mir.module_name,
            "lower equations",
            Some(phase_started.elapsed()),
            Some(builder.values.len()),
        );
        trace_opt_phase(trace, &mir.module_name, "add derivatives", None, None);
        let phase_started = std::time::Instant::now();
        builder.add_sparse_derivatives();
        trace_opt_phase(
            trace,
            &mir.module_name,
            "add derivatives",
            Some(phase_started.elapsed()),
            Some(builder.values.len()),
        );
        trace_opt_phase(trace, &mir.module_name, "finish", None, None);
        let phase_started = std::time::Instant::now();
        let (values, runtime_loops) = builder.finish(&mut equation_values);
        trace_opt_phase(
            trace,
            &mir.module_name,
            "finish",
            Some(phase_started.elapsed()),
            Some(values.len()),
        );

        trace_opt_phase(trace, &mir.module_name, "build schedules", None, None);
        let phase_started = std::time::Instant::now();
        let mut schedules = Vec::new();
        for invalidation in [
            InvalidationClass::InstanceStatic,
            InvalidationClass::TemperatureStatic,
        ] {
            let ops = collect_static_ops(&values, invalidation);
            if !ops.is_empty() {
                schedules.push(OptSchedule {
                    id: ScheduleId::from(schedules.len()),
                    invalidation,
                    ops,
                });
            }
        }

        let mut newton_ops = Vec::new();
        for (equation, value) in mir.equations.iter().zip(equation_values) {
            if let Some(value) = value {
                newton_ops.push(OptOp::ComputeValue { value });
            }
            newton_ops.push(OptOp::EvaluateEquation {
                equation: equation.id,
            });
        }
        schedules.push(OptSchedule {
            id: ScheduleId::from(schedules.len()),
            invalidation: InvalidationClass::NewtonIteration,
            ops: newton_ops,
        });
        trace_opt_phase(
            trace,
            &mir.module_name,
            "build schedules",
            Some(phase_started.elapsed()),
            Some(values.len()),
        );

        let opt = Self {
            module_name: mir.module_name.clone(),
            node_count: u32::try_from(mir.nodes.len()).expect("MIR node count exceeds u32::MAX"),
            parameter_count: u32::try_from(mir.parameters.len())
                .expect("MIR parameter count exceeds u32::MAX"),
            branch_count: u32::try_from(mir.branches.len())
                .expect("MIR branch count exceeds u32::MAX"),
            branch_unknown_count: u32::try_from(mir.branch_unknowns.len())
                .expect("MIR branch unknown count exceeds u32::MAX"),
            equation_count: u32::try_from(mir.equations.len())
                .expect("MIR equation count exceeds u32::MAX"),
            values,
            runtime_loops,
            schedules,
        };

        trace_opt_phase(
            trace,
            &mir.module_name,
            "validate",
            None,
            Some(opt.values.len()),
        );
        let phase_started = std::time::Instant::now();
        opt.validate().map(|()| {
            trace_opt_phase(
                trace,
                &mir.module_name,
                "validate",
                Some(phase_started.elapsed()),
                Some(opt.values.len()),
            );
            opt
        })
    }

    pub fn validate(&self) -> IrValidationResult {
        let mut diagnostics = Vec::new();

        if self.module_name.is_empty() {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::OptValidation,
                "OptIR module name must not be empty",
            ));
        }

        validate_dense_value_ids(&mut diagnostics, &self.values);
        validate_values(&mut diagnostics, self);
        validate_runtime_loops(&mut diagnostics, self);
        validate_dense_schedule_ids(&mut diagnostics, &self.schedules);
        validate_schedules(
            &mut diagnostics,
            &self.schedules,
            self.values.len(),
            self.equation_count,
        );

        if diagnostics.is_empty() {
            Ok(())
        } else {
            Err(diagnostics)
        }
    }
}

struct ScalarGraphBuilder<'a> {
    hir: Option<&'a HirModel>,
    mir: &'a MirModel,
    values: Vec<OptValue>,
    value_keys: HashMap<OptValueKey, ValueId>,
    expression_values: HashMap<ExprId, Option<ValueId>>,
    variable_values: HashMap<VariableId, Option<ValueId>>,
    variable_assignment_exprs: HashMap<VariableId, ExprId>,
    variable_assignment_history: HashMap<VariableId, Vec<AssignmentHistoryEntry>>,
    guarded_path_assignment_exprs: HashMap<VariableId, GuardedPathAssignmentExpr>,
    assignment_history_prefix_cache: HashMap<AssignmentHistoryPrefixCacheKey, ValueId>,
    current_path_history_cache: HashMap<CurrentPathHistoryCacheKey, ValueId>,
    recent_assignment_history_cache: HashMap<RecentAssignmentHistoryCacheKey, ValueId>,
    variable_lowering_stack: HashSet<VariableId>,
    guard_history_lowering_stack: HashSet<(VariableId, usize)>,
    expression_lowering_stack: HashSet<ExprId>,
    expression_lowering_depth: usize,
    assignment_history_value_snapshot_stack: Vec<(VariableId, ValueId)>,
    assignment_history_snapshot_stack: Vec<(VariableId, usize)>,
    assignment_history_previous_value_stack: Vec<(VariableId, ValueId)>,
    history_reconstruction_steps: usize,
    potential_equation_state_operator_depth: usize,
    guard_alias_lowering_depth: usize,
    branch_current_values: HashMap<String, ValueId>,
    branch_flow_context: Option<BranchUnknownId>,
    conditional_path_stack: Vec<ConditionalPathPredicate>,
    runtime_loops: Vec<OptRuntimeLoop>,
    next_loop_id: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ConditionalPathPredicate {
    condition: ExprId,
    truth: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AssignmentHistoryEntry {
    expr: ExprId,
    value_snapshots: Vec<(VariableId, ValueId)>,
    snapshots: Vec<(VariableId, usize)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GuardedPathAssignmentExpr {
    condition: ExprId,
    value: AssignmentHistoryEntry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ConditionalSelfUpdate {
    condition: ExprId,
    active_truth: bool,
    value_expr: ExprId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RelativeCondition {
    condition: ExprId,
    truth: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct AssignmentHistoryPrefixCacheKey {
    variable: VariableId,
    limit: usize,
    branch_flow_context: Option<BranchUnknownId>,
    guard_alias_lowering_depth: usize,
    path: Vec<ConditionalPathPredicate>,
    previous_values: Vec<(VariableId, ValueId)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CurrentPathHistoryCacheKey {
    variable: VariableId,
    branch_flow_context: Option<BranchUnknownId>,
    guard_alias_lowering_depth: usize,
    path: Vec<ConditionalPathPredicate>,
    previous_values: Vec<(VariableId, ValueId)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RecentAssignmentHistoryCacheKey {
    variable: VariableId,
    branch_flow_context: Option<BranchUnknownId>,
    guard_alias_lowering_depth: usize,
    path: Vec<ConditionalPathPredicate>,
    previous_values: Vec<(VariableId, ValueId)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum OptValueKey {
    RealConstant(u64),
    BooleanConstant(bool),
    Parameter(ParamId),
    ParamGiven(ParamId),
    Temperature,
    ThermalVoltage,
    Multiplicity,
    Time,
    Analysis(SmolStr),
    Ddx {
        value: ValueId,
        pos_node: Option<NodeId>,
        neg_node: Option<NodeId>,
    },
    Ddt {
        operator: ExprId,
        input: ValueId,
    },
    DdtScale,
    NodePotential(NodeId),
    BranchFlow(BranchId),
    BranchUnknownFlow(BranchUnknownId),
    LoopIndex(u32),
    CountedSum {
        loop_id: u32,
        count: ValueId,
        initial: ValueId,
        term: ValueId,
    },
    RuntimeLoopVariable {
        loop_id: u32,
        slot: u32,
    },
    RuntimeLoopVariableDerivative {
        loop_id: u32,
        slot: u32,
        lane: DerivativeLane,
    },
    RuntimeLoopResult {
        loop_id: u32,
        slot: u32,
    },
    RuntimeLoopResultDerivative {
        loop_id: u32,
        slot: u32,
        lane: DerivativeLane,
    },
    Unary {
        op: OptUnaryOp,
        input: ValueId,
    },
    Binary {
        op: OptBinaryOp,
        left: ValueId,
        right: ValueId,
    },
    Select {
        condition: ValueId,
        then_value: ValueId,
        else_value: ValueId,
    },
    EquationValue(EquationId),
}

impl OptValueKey {
    fn from_kind(kind: &OptValueKind) -> Self {
        match kind {
            OptValueKind::RealConstant(value) => Self::RealConstant(value.to_bits()),
            OptValueKind::BooleanConstant(value) => Self::BooleanConstant(*value),
            OptValueKind::Parameter { parameter } => Self::Parameter(*parameter),
            OptValueKind::ParamGiven { parameter } => Self::ParamGiven(*parameter),
            OptValueKind::Temperature => Self::Temperature,
            OptValueKind::ThermalVoltage => Self::ThermalVoltage,
            OptValueKind::Multiplicity => Self::Multiplicity,
            OptValueKind::Time => Self::Time,
            OptValueKind::Analysis { query } => Self::Analysis(query.clone()),
            OptValueKind::Ddx {
                value,
                pos_node,
                neg_node,
            } => Self::Ddx {
                value: *value,
                pos_node: *pos_node,
                neg_node: *neg_node,
            },
            OptValueKind::Ddt { operator, input } => Self::Ddt {
                operator: *operator,
                input: *input,
            },
            OptValueKind::DdtScale => Self::DdtScale,
            OptValueKind::NodePotential { node } => Self::NodePotential(*node),
            OptValueKind::BranchFlow { branch } => Self::BranchFlow(*branch),
            OptValueKind::BranchUnknownFlow { branch_unknown } => {
                Self::BranchUnknownFlow(*branch_unknown)
            }
            OptValueKind::LoopIndex { loop_id } => Self::LoopIndex(*loop_id),
            OptValueKind::CountedSum {
                loop_id,
                count,
                initial,
                term,
            } => Self::CountedSum {
                loop_id: *loop_id,
                count: *count,
                initial: *initial,
                term: *term,
            },
            OptValueKind::RuntimeLoopVariable { loop_id, slot } => Self::RuntimeLoopVariable {
                loop_id: *loop_id,
                slot: *slot,
            },
            OptValueKind::RuntimeLoopVariableDerivative {
                loop_id,
                slot,
                lane,
            } => Self::RuntimeLoopVariableDerivative {
                loop_id: *loop_id,
                slot: *slot,
                lane: *lane,
            },
            OptValueKind::RuntimeLoopResult { loop_id, slot } => Self::RuntimeLoopResult {
                loop_id: *loop_id,
                slot: *slot,
            },
            OptValueKind::RuntimeLoopResultDerivative {
                loop_id,
                slot,
                lane,
            } => Self::RuntimeLoopResultDerivative {
                loop_id: *loop_id,
                slot: *slot,
                lane: *lane,
            },
            OptValueKind::Unary { op, input } => Self::Unary {
                op: *op,
                input: *input,
            },
            OptValueKind::Binary { op, left, right } => Self::Binary {
                op: *op,
                left: *left,
                right: *right,
            },
            OptValueKind::Select {
                condition,
                then_value,
                else_value,
            } => Self::Select {
                condition: *condition,
                then_value: *then_value,
                else_value: *else_value,
            },
            OptValueKind::EquationValue { equation } => Self::EquationValue(*equation),
        }
    }
}

impl<'a> ScalarGraphBuilder<'a> {
    fn new(hir: Option<&'a HirModel>, mir: &'a MirModel) -> Self {
        Self {
            hir,
            mir,
            values: Vec::new(),
            value_keys: HashMap::new(),
            expression_values: HashMap::new(),
            variable_values: HashMap::new(),
            variable_assignment_exprs: HashMap::new(),
            variable_assignment_history: HashMap::new(),
            guarded_path_assignment_exprs: HashMap::new(),
            assignment_history_prefix_cache: HashMap::new(),
            current_path_history_cache: HashMap::new(),
            recent_assignment_history_cache: HashMap::new(),
            variable_lowering_stack: HashSet::new(),
            guard_history_lowering_stack: HashSet::new(),
            expression_lowering_stack: HashSet::new(),
            expression_lowering_depth: 0,
            assignment_history_value_snapshot_stack: Vec::new(),
            assignment_history_snapshot_stack: Vec::new(),
            assignment_history_previous_value_stack: Vec::new(),
            history_reconstruction_steps: 0,
            potential_equation_state_operator_depth: 0,
            guard_alias_lowering_depth: 0,
            branch_current_values: HashMap::new(),
            branch_flow_context: None,
            conditional_path_stack: Vec::new(),
            runtime_loops: Vec::new(),
            next_loop_id: 0,
        }
    }

    fn finish(
        mut self,
        equation_values: &mut [Option<ValueId>],
    ) -> (Vec<OptValue>, Vec<OptRuntimeLoop>) {
        self.eliminate_dead_values(equation_values);
        (self.values, self.runtime_loops)
    }

    fn eliminate_dead_values(&mut self, equation_values: &mut [Option<ValueId>]) {
        if self.values.is_empty() {
            return;
        }

        let mut observable_roots = HashSet::new();
        let mut live = vec![false; self.values.len()];
        for root in equation_values.iter().flatten().copied() {
            observable_roots.insert(root);
            self.mark_live_value(root, &mut live);
            for derivative in self.values[usize::from(root)].derivatives.clone() {
                self.mark_live_value(derivative.value, &mut live);
            }
        }
        observable_roots.extend(self.ddx_derivative_observable_roots(&live));

        if live.iter().all(|is_live| *is_live) {
            return;
        }

        let mut remap = vec![None; self.values.len()];
        let mut compacted = Vec::with_capacity(live.iter().filter(|is_live| **is_live).count());
        for (old_index, value) in self.values.iter().enumerate() {
            if live[old_index] {
                let new_id = ValueId::from(compacted.len());
                remap[old_index] = Some(new_id);
                let mut value = value.clone();
                value.id = new_id;
                compacted.push(value);
            }
        }

        let observable_new_roots: HashSet<_> = observable_roots
            .iter()
            .map(|root| remap_value_id(*root, &remap))
            .collect();

        for value in &mut compacted {
            value.kind = remap_value_kind(&value.kind, &remap);
            if observable_new_roots.contains(&value.id) {
                value.derivatives = value
                    .derivatives
                    .iter()
                    .map(|derivative| OptDerivative {
                        lane: derivative.lane,
                        value: remap_value_id(derivative.value, &remap),
                    })
                    .collect();
            } else {
                value.derivatives.clear();
            }
        }

        for root in equation_values.iter_mut().flatten() {
            *root = remap_value_id(*root, &remap);
        }

        let live_runtime_loops: HashSet<_> = self
            .values
            .iter()
            .enumerate()
            .filter(|(index, _)| live[*index])
            .filter_map(|(_, value)| match value.kind {
                OptValueKind::RuntimeLoopResult { loop_id, .. }
                | OptValueKind::RuntimeLoopResultDerivative { loop_id, .. } => Some(loop_id),
                _ => None,
            })
            .collect();
        self.runtime_loops = self
            .runtime_loops
            .iter()
            .filter(|runtime_loop| live_runtime_loops.contains(&runtime_loop.loop_id))
            .map(|runtime_loop| remap_runtime_loop(runtime_loop, &remap))
            .collect();
        self.values = compacted;
    }

    fn mark_live_value(&self, value: ValueId, live: &mut [bool]) {
        let mut stack = vec![value];
        while let Some(value) = stack.pop() {
            let index = usize::from(value);
            if live.get(index).copied().unwrap_or(false) {
                continue;
            }
            let Some(slot) = live.get_mut(index) else {
                continue;
            };
            *slot = true;
            self.push_live_value_dependencies(index, &mut stack);
        }
    }

    fn push_live_value_dependencies(&self, index: usize, stack: &mut Vec<ValueId>) {
        match self.values[index].kind {
            OptValueKind::RealConstant(_)
            | OptValueKind::BooleanConstant(_)
            | OptValueKind::Parameter { .. }
            | OptValueKind::ParamGiven { .. }
            | OptValueKind::Temperature
            | OptValueKind::ThermalVoltage
            | OptValueKind::Multiplicity
            | OptValueKind::Time
            | OptValueKind::Analysis { .. }
            | OptValueKind::NodePotential { .. }
            | OptValueKind::BranchFlow { .. }
            | OptValueKind::BranchUnknownFlow { .. }
            | OptValueKind::LoopIndex { .. }
            | OptValueKind::RuntimeLoopVariable { .. }
            | OptValueKind::RuntimeLoopVariableDerivative { .. }
            | OptValueKind::EquationValue { .. }
            | OptValueKind::DdtScale => {}
            OptValueKind::RuntimeLoopResult { loop_id, .. }
            | OptValueKind::RuntimeLoopResultDerivative { loop_id, .. } => {
                self.push_runtime_loop_live_dependencies(loop_id, stack);
            }
            OptValueKind::CountedSum {
                count,
                initial,
                term,
                ..
            } => {
                stack.extend([count, initial, term]);
            }
            OptValueKind::Ddx {
                value,
                pos_node,
                neg_node,
            } => {
                stack.push(value);
                self.push_ddx_projection_values(value, pos_node, neg_node, stack);
            }
            OptValueKind::Ddt { input, .. } | OptValueKind::Unary { input, .. } => {
                stack.push(input);
            }
            OptValueKind::Binary { left, right, .. } => {
                stack.extend([left, right]);
            }
            OptValueKind::Select {
                condition,
                then_value,
                else_value,
            } => {
                stack.extend([condition, then_value, else_value]);
            }
        }
    }

    fn push_runtime_loop_live_dependencies(&self, loop_id: u32, stack: &mut Vec<ValueId>) {
        let Some(runtime_loop) = self
            .runtime_loops
            .iter()
            .find(|runtime_loop| runtime_loop.loop_id == loop_id)
        else {
            return;
        };
        for variable in &runtime_loop.variables {
            stack.extend([variable.initial, variable.variable, variable.result]);
        }
        stack.push(runtime_loop.condition);
        for assignment in &runtime_loop.assignments {
            stack.push(assignment.value);
            for derivative in &self.values[usize::from(assignment.value)].derivatives {
                stack.push(derivative.value);
            }
        }
    }

    fn push_ddx_projection_values(
        &self,
        value: ValueId,
        pos_node: Option<NodeId>,
        neg_node: Option<NodeId>,
        stack: &mut Vec<ValueId>,
    ) {
        for node in [pos_node, neg_node].into_iter().flatten() {
            if let Some(derivative) = self.derivative_value(value, DerivativeLane::node(node)) {
                stack.push(derivative);
            }
        }
    }

    fn ddx_derivative_observable_roots(&self, live: &[bool]) -> HashSet<ValueId> {
        self.values
            .iter()
            .filter(|value| live.get(usize::from(value.id)).copied().unwrap_or(false))
            .filter_map(|value| match value.kind {
                OptValueKind::Ddx { value, .. } => Some(value),
                _ => None,
            })
            .collect()
    }

    fn lower_statements(&mut self, statements: &[HirStatement]) {
        let trace = std::env::var_os("RSPICE_VERILOGA_CANONICAL_IR_PHASE_TRACE").is_some();
        for (index, statement) in statements.iter().enumerate() {
            let phase = format!(
                "lower statement {}/{} {}",
                index + 1,
                statements.len(),
                hir_statement_trace_label(statement)
            );
            trace_opt_phase(
                trace,
                &self.mir.module_name,
                &phase,
                None,
                Some(self.values.len()),
            );
            let phase_started = std::time::Instant::now();
            self.lower_statement(statement);
            trace_opt_phase(
                trace,
                &self.mir.module_name,
                &phase,
                Some(phase_started.elapsed()),
                Some(self.values.len()),
            );
        }
    }

    fn lower_statement(&mut self, statement: &HirStatement) {
        match statement {
            HirStatement::Assignment(assignment) => self.lower_assignment_statement(assignment),
            HirStatement::Loop(loop_statement) => self.lower_loop_statement(loop_statement),
        }
    }

    fn lower_assignment_statement(&mut self, assignment: &HirAssignment) {
        self.clear_assignment_replay_caches();
        self.guarded_path_assignment_exprs
            .remove(&assignment.target);
        if assignment.index.is_none() && supported_assignment_value_type(assignment.expr_type) {
            let entry = self.assignment_history_entry(assignment.target, assignment.expr.id);
            self.variable_assignment_exprs
                .insert(assignment.target, assignment.expr.id);
            self.variable_assignment_history
                .entry(assignment.target)
                .or_default()
                .push(entry);
        } else {
            self.variable_assignment_exprs.remove(&assignment.target);
            self.variable_assignment_history.remove(&assignment.target);
        }
        let value = if assignment.index.is_none()
            && supported_assignment_value_type(assignment.expr_type)
        {
            self.lower_expression(assignment.expr.id)
        } else {
            None
        };
        self.variable_values.insert(assignment.target, value);
        self.expression_values.clear();
    }

    fn lower_loop_statement(&mut self, loop_statement: &HirLoop) {
        if self.lower_counted_accumulator_loop(loop_statement) {
            return;
        }
        if self.lower_bounded_guarded_loop(loop_statement) {
            return;
        }
        if self.lower_runtime_bounded_guarded_loop(loop_statement) {
            return;
        }

        let mut iterations = 0;
        loop {
            self.expression_values.clear();
            let Some(condition) = self.lower_expression(loop_statement.condition.id) else {
                self.mark_loop_assignments_unknown(loop_statement);
                return;
            };
            match self.boolean_constant(condition) {
                Some(false) => {
                    self.expression_values.clear();
                    return;
                }
                Some(true) => {}
                None => {
                    self.mark_loop_assignments_unknown(loop_statement);
                    return;
                }
            }

            if iterations == MAX_SCALAR_LOOP_UNROLL_ITERATIONS {
                self.mark_loop_assignments_unknown(loop_statement);
                return;
            }
            iterations += 1;
            self.expression_values.clear();
            self.lower_statements(&loop_statement.body);
        }
    }

    fn lower_bounded_guarded_loop(&mut self, loop_statement: &HirLoop) -> bool {
        let original_values = self.values.clone();
        let original_value_keys = self.value_keys.clone();
        let original_expression_values = self.expression_values.clone();
        let original_variable_values = self.variable_values.clone();
        let original_variable_assignment_exprs = self.variable_assignment_exprs.clone();
        let original_variable_assignment_history = self.variable_assignment_history.clone();
        let original_guarded_path_assignment_exprs = self.guarded_path_assignment_exprs.clone();
        let original_assignment_history_prefix_cache = self.assignment_history_prefix_cache.clone();
        let original_current_path_history_cache = self.current_path_history_cache.clone();
        let original_recent_assignment_history_cache = self.recent_assignment_history_cache.clone();
        let original_conditional_path_stack = self.conditional_path_stack.clone();
        let original_history_reconstruction_steps = self.history_reconstruction_steps;

        let matched = self.try_lower_bounded_guarded_loop(loop_statement);
        if matched {
            self.expression_values.clear();
            true
        } else {
            self.values = original_values;
            self.value_keys = original_value_keys;
            self.expression_values = original_expression_values;
            self.variable_values = original_variable_values;
            self.variable_assignment_exprs = original_variable_assignment_exprs;
            self.variable_assignment_history = original_variable_assignment_history;
            self.guarded_path_assignment_exprs = original_guarded_path_assignment_exprs;
            self.assignment_history_prefix_cache = original_assignment_history_prefix_cache;
            self.current_path_history_cache = original_current_path_history_cache;
            self.recent_assignment_history_cache = original_recent_assignment_history_cache;
            self.conditional_path_stack = original_conditional_path_stack;
            self.history_reconstruction_steps = original_history_reconstruction_steps;
            false
        }
    }

    fn lower_runtime_bounded_guarded_loop(&mut self, loop_statement: &HirLoop) -> bool {
        let original_values = self.values.clone();
        let original_value_keys = self.value_keys.clone();
        let original_expression_values = self.expression_values.clone();
        let original_variable_values = self.variable_values.clone();
        let original_variable_assignment_exprs = self.variable_assignment_exprs.clone();
        let original_variable_assignment_history = self.variable_assignment_history.clone();
        let original_guarded_path_assignment_exprs = self.guarded_path_assignment_exprs.clone();
        let original_assignment_history_prefix_cache = self.assignment_history_prefix_cache.clone();
        let original_current_path_history_cache = self.current_path_history_cache.clone();
        let original_recent_assignment_history_cache = self.recent_assignment_history_cache.clone();
        let original_conditional_path_stack = self.conditional_path_stack.clone();
        let original_runtime_loops = self.runtime_loops.clone();
        let original_next_loop_id = self.next_loop_id;
        let original_history_reconstruction_steps = self.history_reconstruction_steps;

        let matched = self.try_lower_runtime_bounded_guarded_loop(loop_statement);
        if matched {
            self.expression_values.clear();
            true
        } else {
            self.values = original_values;
            self.value_keys = original_value_keys;
            self.expression_values = original_expression_values;
            self.variable_values = original_variable_values;
            self.variable_assignment_exprs = original_variable_assignment_exprs;
            self.variable_assignment_history = original_variable_assignment_history;
            self.guarded_path_assignment_exprs = original_guarded_path_assignment_exprs;
            self.assignment_history_prefix_cache = original_assignment_history_prefix_cache;
            self.current_path_history_cache = original_current_path_history_cache;
            self.recent_assignment_history_cache = original_recent_assignment_history_cache;
            self.conditional_path_stack = original_conditional_path_stack;
            self.runtime_loops = original_runtime_loops;
            self.next_loop_id = original_next_loop_id;
            self.history_reconstruction_steps = original_history_reconstruction_steps;
            false
        }
    }

    fn try_lower_runtime_bounded_guarded_loop(&mut self, loop_statement: &HirLoop) -> bool {
        let Some(counter) = self.runtime_loop_counter(loop_statement.condition.id) else {
            return false;
        };
        if loop_statement
            .body
            .iter()
            .any(|statement| !matches!(statement, HirStatement::Assignment(_)))
        {
            return false;
        }
        if !loop_statement.body.iter().any(|statement| {
            matches!(
                statement,
                HirStatement::Assignment(assignment)
                    if self.is_counter_increment_assignment(assignment, counter)
            )
        }) {
            return false;
        }

        let Some(assigned_variables) = self.ordered_loop_assignment_targets(&loop_statement.body)
        else {
            return false;
        };
        let Some(loop_id) = self.allocate_loop_id() else {
            return false;
        };

        let mut slot_by_variable = HashMap::new();
        let mut variables = Vec::with_capacity(assigned_variables.len());
        for variable in &assigned_variables {
            let Some(initial) = self.current_variable_value(*variable) else {
                return false;
            };
            let Some(value_type) = self.variable_opt_value_type(*variable) else {
                return false;
            };
            let slot = u32::try_from(variables.len()).ok();
            let Some(slot) = slot else {
                return false;
            };
            let variable_value = self.push_value(
                value_type,
                OptValueKind::RuntimeLoopVariable { loop_id, slot },
            );
            slot_by_variable.insert(*variable, slot);
            self.variable_values.insert(*variable, Some(variable_value));
            variables.push(OptRuntimeLoopVariable {
                source: *variable,
                value_type,
                initial,
                variable: variable_value,
                result: ValueId::from(0),
            });
        }

        self.expression_values.clear();
        let Some(condition) = self.lower_expression(loop_statement.condition.id) else {
            return false;
        };

        let mut assignments = Vec::new();
        self.clear_assignment_replay_caches();
        for statement in &loop_statement.body {
            let HirStatement::Assignment(assignment) = statement else {
                return false;
            };
            if assignment.index.is_some() || !supported_assignment_value_type(assignment.expr_type)
            {
                return false;
            }
            let Some(slot) = slot_by_variable.get(&assignment.target).copied() else {
                return false;
            };
            let entry = self.assignment_history_entry(assignment.target, assignment.expr.id);
            self.variable_assignment_exprs
                .insert(assignment.target, assignment.expr.id);
            self.variable_assignment_history
                .entry(assignment.target)
                .or_default()
                .push(entry);
            self.expression_values.clear();
            let Some(value) = self.lower_expression(assignment.expr.id) else {
                return false;
            };
            self.variable_values.insert(assignment.target, Some(value));
            assignments.push(OptRuntimeLoopAssignment { slot, value });
        }

        for (slot, variable) in variables.iter_mut().enumerate() {
            let slot = u32::try_from(slot).expect("runtime loop slot exceeds u32::MAX");
            variable.result = self.push_value(
                variable.value_type,
                OptValueKind::RuntimeLoopResult { loop_id, slot },
            );
        }
        self.runtime_loops.push(OptRuntimeLoop {
            loop_id,
            variables,
            condition,
            assignments,
        });
        self.clear_assignment_replay_caches();
        for runtime_variable in self
            .runtime_loops
            .last()
            .expect("runtime loop was just pushed")
            .variables
            .iter()
        {
            self.variable_values
                .insert(runtime_variable.source, Some(runtime_variable.result));
            self.variable_assignment_exprs
                .remove(&runtime_variable.source);
            self.variable_assignment_history
                .remove(&runtime_variable.source);
            self.guarded_path_assignment_exprs
                .remove(&runtime_variable.source);
        }
        self.expression_values.clear();
        true
    }

    fn runtime_loop_counter(&mut self, condition: ExprId) -> Option<VariableId> {
        if let Some((counter, _iteration_count)) =
            self.bounded_guarded_loop_iteration_count(condition)
        {
            return Some(counter);
        }

        let counter = self.runtime_loop_bound_counter(condition)?;
        (self.variable_value_type(counter) == Some(CanonicalValueType::Integer)).then_some(counter)
    }

    fn runtime_loop_bound_counter(&mut self, condition: ExprId) -> Option<VariableId> {
        if let Some(counter) = self.runtime_loop_simple_bound_counter(condition) {
            return Some(counter);
        }

        let expression = self.mir.expressions.get(usize::from(condition))?;
        let HirExprKind::Binary { op, left, right } = &expression.kind else {
            return None;
        };
        if op.as_str() != "And" {
            return None;
        }
        self.runtime_loop_bound_counter(*left)
            .or_else(|| self.runtime_loop_bound_counter(*right))
    }

    fn runtime_loop_simple_bound_counter(&mut self, condition: ExprId) -> Option<VariableId> {
        let expression = self.mir.expressions.get(usize::from(condition))?;
        let HirExprKind::Binary { op, left, right } = &expression.kind else {
            return None;
        };
        if !matches!(op.as_str(), "Lt" | "Le") {
            return None;
        }
        let counter = self.variable_identifier(*left)?;
        self.lower_expression(*right)?;
        Some(counter)
    }

    fn try_lower_bounded_guarded_loop(&mut self, loop_statement: &HirLoop) -> bool {
        let Some((counter, iteration_count)) =
            self.bounded_guarded_loop_iteration_count(loop_statement.condition.id)
        else {
            return false;
        };
        if iteration_count > MAX_SCALAR_BOUNDED_LOOP_UNROLL_ITERATIONS {
            return false;
        }
        if iteration_count.saturating_mul(loop_statement.body.len())
            > MAX_SCALAR_BOUNDED_LOOP_ASSIGNMENT_EXPANSIONS
        {
            return false;
        }
        if !loop_statement.body.iter().any(|statement| {
            matches!(
                statement,
                HirStatement::Assignment(assignment)
                    if self.is_counter_increment_assignment(assignment, counter)
            )
        }) {
            return false;
        }
        if loop_statement
            .body
            .iter()
            .any(|statement| !matches!(statement, HirStatement::Assignment(_)))
        {
            return false;
        }

        for _ in 0..iteration_count {
            self.expression_values.clear();
            let Some(guard) = self.lower_expression(loop_statement.condition.id) else {
                return false;
            };
            self.conditional_path_stack.push(ConditionalPathPredicate {
                condition: loop_statement.condition.id,
                truth: true,
            });
            for statement in &loop_statement.body {
                let HirStatement::Assignment(assignment) = statement else {
                    return false;
                };
                if !self.lower_guarded_assignment_statement(assignment, guard) {
                    self.conditional_path_stack.pop();
                    return false;
                }
            }
            self.conditional_path_stack.pop();
        }
        self.expression_values.clear();
        true
    }

    fn lower_guarded_assignment_statement(
        &mut self,
        assignment: &HirAssignment,
        guard: ValueId,
    ) -> bool {
        if assignment.index.is_some() || !supported_assignment_value_type(assignment.expr_type) {
            return false;
        }

        let previous = self.current_variable_value(assignment.target);
        self.clear_assignment_replay_caches();
        let entry = self.assignment_history_entry(assignment.target, assignment.expr.id);
        self.guarded_path_assignment_exprs.insert(
            assignment.target,
            GuardedPathAssignmentExpr {
                condition: self
                    .conditional_path_stack
                    .last()
                    .expect("guarded assignment must have an active path")
                    .condition,
                value: entry.clone(),
            },
        );
        self.variable_assignment_exprs
            .insert(assignment.target, assignment.expr.id);
        self.variable_assignment_history
            .entry(assignment.target)
            .or_default()
            .push(entry);
        self.expression_values.clear();
        let Some(next) = self.lower_expression(assignment.expr.id) else {
            self.variable_values.insert(assignment.target, None);
            self.expression_values.clear();
            return true;
        };
        let Some(previous) = previous else {
            self.variable_values.insert(assignment.target, None);
            self.expression_values.clear();
            return true;
        };
        let Some(value_type) = self.variable_opt_value_type(assignment.target) else {
            self.variable_values.insert(assignment.target, None);
            self.expression_values.clear();
            return true;
        };
        let value = self.push_value(
            value_type,
            OptValueKind::Select {
                condition: guard,
                then_value: next,
                else_value: previous,
            },
        );
        self.variable_values.insert(assignment.target, Some(value));
        self.expression_values.clear();
        true
    }

    fn bounded_guarded_loop_iteration_count(
        &mut self,
        condition: ExprId,
    ) -> Option<(VariableId, usize)> {
        let (counter, inclusive, upper) = self.bounded_guarded_loop_counter_bound(condition)?;
        let start = self.current_variable_value(counter)?;
        let start = self.real_constant(start)?;
        if start.fract() != 0.0 || upper.fract() != 0.0 || upper < start {
            return None;
        }
        let count = if inclusive {
            upper - start + 1.0
        } else {
            upper - start
        };
        (count >= 0.0 && count <= usize::MAX as f64).then_some((counter, count as usize))
    }

    fn bounded_guarded_loop_counter_bound(
        &self,
        condition: ExprId,
    ) -> Option<(VariableId, bool, f64)> {
        if let Some(bound) = self.counter_upper_bound_condition(condition) {
            return Some(bound);
        }
        let expression = self.mir.expressions.get(usize::from(condition))?;
        let HirExprKind::Binary { op, left, right } = &expression.kind else {
            return None;
        };
        if op.as_str() != "And" {
            return None;
        }
        self.bounded_guarded_loop_counter_bound(*left)
            .or_else(|| self.bounded_guarded_loop_counter_bound(*right))
    }

    fn counter_upper_bound_condition(&self, condition: ExprId) -> Option<(VariableId, bool, f64)> {
        let expression = self.mir.expressions.get(usize::from(condition))?;
        let HirExprKind::Binary { op, left, right } = &expression.kind else {
            return None;
        };
        let inclusive = match op.as_str() {
            "Le" => true,
            "Lt" => false,
            _ => return None,
        };
        let counter = self.variable_identifier(*left)?;
        let upper = self.number_constant_expr(*right)?;
        Some((counter, inclusive, upper))
    }

    fn lower_counted_accumulator_loop(&mut self, loop_statement: &HirLoop) -> bool {
        let original_values = self.values.clone();
        let original_value_keys = self.value_keys.clone();
        let original_expression_values = self.expression_values.clone();
        let original_variable_values = self.variable_values.clone();
        let original_variable_assignment_exprs = self.variable_assignment_exprs.clone();
        let original_variable_assignment_history = self.variable_assignment_history.clone();
        let original_guarded_path_assignment_exprs = self.guarded_path_assignment_exprs.clone();
        let original_assignment_history_prefix_cache = self.assignment_history_prefix_cache.clone();
        let original_current_path_history_cache = self.current_path_history_cache.clone();
        let original_recent_assignment_history_cache = self.recent_assignment_history_cache.clone();
        let original_next_loop_id = self.next_loop_id;
        let original_history_reconstruction_steps = self.history_reconstruction_steps;

        let matched = self.try_lower_counted_accumulator_loop(loop_statement);
        if matched {
            self.expression_values.clear();
            true
        } else {
            self.values = original_values;
            self.value_keys = original_value_keys;
            self.expression_values = original_expression_values;
            self.variable_values = original_variable_values;
            self.variable_assignment_exprs = original_variable_assignment_exprs;
            self.variable_assignment_history = original_variable_assignment_history;
            self.guarded_path_assignment_exprs = original_guarded_path_assignment_exprs;
            self.assignment_history_prefix_cache = original_assignment_history_prefix_cache;
            self.current_path_history_cache = original_current_path_history_cache;
            self.recent_assignment_history_cache = original_recent_assignment_history_cache;
            self.next_loop_id = original_next_loop_id;
            self.history_reconstruction_steps = original_history_reconstruction_steps;
            false
        }
    }

    fn try_lower_counted_accumulator_loop(&mut self, loop_statement: &HirLoop) -> bool {
        self.expression_values.clear();
        let Some((counter, bound, _guarded_loop)) =
            self.counted_loop_condition(loop_statement.condition.id)
        else {
            return false;
        };
        if self.variable_value_type(counter) != Some(CanonicalValueType::Integer) {
            return false;
        }
        let Some(counter_start) = self.current_variable_value(counter) else {
            return false;
        };
        if !self.is_real_constant(counter_start, 0.0) {
            return false;
        }

        let Some(assigned_variables) = self.loop_assignment_targets(&loop_statement.body) else {
            return false;
        };
        let mut accumulator_targets = HashSet::new();
        let mut product_targets = HashSet::new();
        let mut saw_counter_increment = false;

        for statement in &loop_statement.body {
            let HirStatement::Assignment(assignment) = statement else {
                return false;
            };
            if self.is_counter_increment_assignment(assignment, counter) {
                saw_counter_increment = true;
                continue;
            }
            let expr = self.counted_loop_assignment_expr(assignment);
            if let Some(_term) = self.accumulator_update_term(assignment, counter, expr) {
                accumulator_targets.insert(assignment.target);
                continue;
            }
            if let Some(_factor) = self.product_update_factor(assignment, counter, expr) {
                product_targets.insert(assignment.target);
                continue;
            }
            if assignment.index.is_some() || !supported_assignment_value_type(assignment.expr_type)
            {
                return false;
            }
            if assignment.target == counter {
                return false;
            }
        }

        if !saw_counter_increment || (accumulator_targets.is_empty() && product_targets.is_empty())
        {
            return false;
        }

        let loop_carried_candidates: HashSet<_> = assigned_variables
            .iter()
            .copied()
            .filter(|variable| *variable != counter)
            .collect();
        let mut safe_loop_local_targets = HashSet::new();
        for statement in &loop_statement.body {
            let HirStatement::Assignment(assignment) = statement else {
                return false;
            };
            if self.is_counter_increment_assignment(assignment, counter)
                || accumulator_targets.contains(&assignment.target)
                || product_targets.contains(&assignment.target)
            {
                continue;
            }
            let expr = self.counted_loop_assignment_expr(assignment);
            if assignment.index.is_some()
                || !supported_assignment_value_type(assignment.expr_type)
                || self.expr_references_any_variable(expr, &loop_carried_candidates)
            {
                return false;
            }
            safe_loop_local_targets.insert(assignment.target);
        }
        let forbidden_accumulator_term_targets: HashSet<_> = assigned_variables
            .iter()
            .copied()
            .filter(|variable| *variable != counter)
            .filter(|variable| !accumulator_targets.contains(variable))
            .filter(|variable| !product_targets.contains(variable))
            .filter(|variable| !safe_loop_local_targets.contains(variable))
            .collect();
        let mut forbidden_product_factor_targets: HashSet<_> = assigned_variables
            .iter()
            .copied()
            .filter(|variable| *variable != counter)
            .collect();
        forbidden_product_factor_targets.insert(counter);

        let original_variable_values = self.variable_values.clone();
        let Some(loop_id) = self.allocate_loop_id() else {
            return false;
        };
        let loop_index = self.push_value(OptValueType::Real, OptValueKind::LoopIndex { loop_id });
        self.variable_values.insert(counter, Some(loop_index));
        self.expression_values.clear();

        let mut accumulator_sums = Vec::new();
        let mut accumulator_products = Vec::new();
        for statement in &loop_statement.body {
            let HirStatement::Assignment(assignment) = statement else {
                return false;
            };
            if self.is_counter_increment_assignment(assignment, counter) {
                continue;
            }

            let expr = self.counted_loop_assignment_expr(assignment);
            if let Some(term_expr) = self.accumulator_update_term(assignment, counter, expr) {
                if self.expr_references_any_variable(term_expr, &accumulator_targets) {
                    return false;
                }
                if self.expr_references_any_variable(term_expr, &forbidden_accumulator_term_targets)
                {
                    return false;
                }
                let Some(previous) =
                    self.variable_value_from_snapshot(assignment.target, &original_variable_values)
                else {
                    return false;
                };
                self.expression_values.clear();
                let Some(term) = self.lower_expression(term_expr) else {
                    return false;
                };
                accumulator_sums.push((assignment.target, previous, term));
                continue;
            }
            if let Some(factor_expr) = self.product_update_factor(assignment, counter, expr) {
                if self.expr_references_any_variable(factor_expr, &forbidden_product_factor_targets)
                {
                    return false;
                }
                let Some(initial) =
                    self.variable_value_from_snapshot(assignment.target, &original_variable_values)
                else {
                    return false;
                };
                self.expression_values.clear();
                let Some(factor) = self.lower_expression(factor_expr) else {
                    return false;
                };
                accumulator_products.push((assignment.target, initial, factor));
                continue;
            }

            if !safe_loop_local_targets.contains(&assignment.target) {
                return false;
            }
            self.expression_values.clear();
            let Some(value) = self.lower_expression(expr) else {
                return false;
            };
            self.variable_values.insert(assignment.target, Some(value));
        }

        self.variable_values = original_variable_values;
        self.clear_assignment_replay_caches();
        for assigned in assigned_variables {
            self.variable_assignment_exprs.remove(&assigned);
            self.variable_assignment_history.remove(&assigned);
            self.guarded_path_assignment_exprs.remove(&assigned);
            self.variable_values.insert(assigned, None);
        }
        for (target, initial, term) in accumulator_sums {
            let sum = self.push_value(
                OptValueType::Real,
                OptValueKind::CountedSum {
                    loop_id,
                    count: bound,
                    initial,
                    term,
                },
            );
            self.variable_values.insert(target, Some(sum));
        }
        for (target, initial, factor) in accumulator_products {
            let power = self.push_binary_value(OptBinaryOp::Pow, factor, bound);
            let product = self.push_binary_value(OptBinaryOp::Mul, initial, power);
            self.variable_values.insert(target, Some(product));
        }
        self.variable_values.insert(counter, Some(bound));
        true
    }

    fn counted_loop_condition(&mut self, condition: ExprId) -> Option<(VariableId, ValueId, bool)> {
        if let Some(counted) = self.counted_loop_bound_condition(condition) {
            let (counter, bound) = counted;
            return Some((counter, bound, false));
        }

        let expression = self.mir.expressions.get(usize::from(condition))?;
        let HirExprKind::Binary { op, left, right } = &expression.kind else {
            return None;
        };
        if op.as_str() != "And" {
            return None;
        }

        if let Some((counter, bound)) = self.counted_loop_bound_condition(*left) {
            let count = self.guarded_loop_count(*right, bound)?;
            return Some((counter, count, true));
        }
        if let Some((counter, bound)) = self.counted_loop_bound_condition(*right) {
            let count = self.guarded_loop_count(*left, bound)?;
            return Some((counter, count, true));
        }
        None
    }

    fn counted_loop_bound_condition(&mut self, condition: ExprId) -> Option<(VariableId, ValueId)> {
        let expression = self.mir.expressions.get(usize::from(condition))?;
        let HirExprKind::Binary { op, left, right } = &expression.kind else {
            return None;
        };
        if op.as_str() != "Lt" {
            return None;
        }
        let counter = self.variable_identifier(*left)?;
        let bound = self.counted_loop_bound(*right)?;
        Some((counter, bound))
    }

    fn guarded_loop_count(&mut self, guard: ExprId, bound: ValueId) -> Option<ValueId> {
        let guard = self.lower_expression(guard)?;
        let zero = self.zero_real();
        Some(self.push_value(
            OptValueType::Real,
            OptValueKind::Select {
                condition: guard,
                then_value: bound,
                else_value: zero,
            },
        ))
    }

    fn counted_loop_bound(&mut self, expr: ExprId) -> Option<ValueId> {
        let bound = self.lower_expression(expr)?;
        let mut memo = vec![None; self.values.len()];
        (value_invalidation(&self.values, bound, &mut memo) == InvalidationClass::InstanceStatic)
            .then_some(bound)
    }

    fn loop_assignment_targets(&self, statements: &[HirStatement]) -> Option<HashSet<VariableId>> {
        let mut targets = HashSet::new();
        for statement in statements {
            let HirStatement::Assignment(assignment) = statement else {
                return None;
            };
            if assignment.index.is_some() {
                return None;
            }
            targets.insert(assignment.target);
        }
        Some(targets)
    }

    fn ordered_loop_assignment_targets(
        &self,
        statements: &[HirStatement],
    ) -> Option<Vec<VariableId>> {
        let mut seen = HashSet::new();
        let mut targets = Vec::new();
        for statement in statements {
            let HirStatement::Assignment(assignment) = statement else {
                return None;
            };
            if assignment.index.is_some() || !supported_assignment_value_type(assignment.expr_type)
            {
                return None;
            }
            if seen.insert(assignment.target) {
                targets.push(assignment.target);
            }
        }
        Some(targets)
    }

    fn is_counter_increment_assignment(
        &self,
        assignment: &HirAssignment,
        counter: VariableId,
    ) -> bool {
        if assignment.target != counter || assignment.index.is_some() {
            return false;
        }
        let expr = self.counted_loop_assignment_expr(assignment);
        let Some((left, right)) = self.add_operands(expr) else {
            return false;
        };
        (self.variable_identifier(left) == Some(counter)
            && self.number_constant_expr(right) == Some(1.0))
            || (self.variable_identifier(right) == Some(counter)
                && self.number_constant_expr(left) == Some(1.0))
    }

    fn accumulator_update_term(
        &self,
        assignment: &HirAssignment,
        counter: VariableId,
        expr: ExprId,
    ) -> Option<ExprId> {
        if assignment.target == counter
            || assignment.index.is_some()
            || !supported_assignment_value_type(assignment.expr_type)
        {
            return None;
        }
        let (left, right) = self.add_operands(expr)?;
        if self.variable_identifier(left) == Some(assignment.target) {
            return Some(right);
        }
        if self.variable_identifier(right) == Some(assignment.target) {
            return Some(left);
        }
        None
    }

    fn product_update_factor(
        &self,
        assignment: &HirAssignment,
        counter: VariableId,
        expr: ExprId,
    ) -> Option<ExprId> {
        if assignment.target == counter
            || assignment.index.is_some()
            || !supported_assignment_value_type(assignment.expr_type)
        {
            return None;
        }
        let (left, right) = self.mul_operands(expr)?;
        if self.variable_identifier(left) == Some(assignment.target) {
            return Some(right);
        }
        if self.variable_identifier(right) == Some(assignment.target) {
            return Some(left);
        }
        None
    }

    fn counted_loop_assignment_expr(&self, assignment: &HirAssignment) -> ExprId {
        let Some(expression) = self.mir.expressions.get(usize::from(assignment.expr.id)) else {
            return assignment.expr.id;
        };
        let HirExprKind::Conditional {
            then_expr,
            else_expr,
            ..
        } = &expression.kind
        else {
            return assignment.expr.id;
        };
        if self.variable_identifier(*else_expr) == Some(assignment.target) {
            *then_expr
        } else {
            assignment.expr.id
        }
    }

    fn variable_value_from_snapshot(
        &mut self,
        variable: VariableId,
        snapshot: &HashMap<VariableId, Option<ValueId>>,
    ) -> Option<ValueId> {
        match snapshot.get(&variable).copied() {
            Some(value) => value,
            None => self.default_variable_value(self.variable_value_type(variable)?),
        }
    }

    fn allocate_loop_id(&mut self) -> Option<u32> {
        let loop_id = self.next_loop_id;
        self.next_loop_id = self.next_loop_id.checked_add(1)?;
        Some(loop_id)
    }

    fn add_operands(&self, expr: ExprId) -> Option<(ExprId, ExprId)> {
        let expression = self.mir.expressions.get(usize::from(expr))?;
        let HirExprKind::Binary { op, left, right } = &expression.kind else {
            return None;
        };
        (op.as_str() == "Add").then_some((*left, *right))
    }

    fn mul_operands(&self, expr: ExprId) -> Option<(ExprId, ExprId)> {
        let expression = self.mir.expressions.get(usize::from(expr))?;
        let HirExprKind::Binary { op, left, right } = &expression.kind else {
            return None;
        };
        (op.as_str() == "Mul").then_some((*left, *right))
    }

    fn current_variable_value(&mut self, variable: VariableId) -> Option<ValueId> {
        match self.variable_values.get(&variable).copied() {
            Some(value) => value,
            None => self.default_variable_value(self.variable_value_type(variable)?),
        }
    }

    fn variable_value_type(&self, variable: VariableId) -> Option<CanonicalValueType> {
        self.hir?
            .variables
            .get(usize::from(variable))
            .map(|variable| variable.value_type)
    }

    fn variable_opt_value_type(&self, variable: VariableId) -> Option<OptValueType> {
        match self.variable_value_type(variable)? {
            CanonicalValueType::Boolean => Some(OptValueType::Boolean),
            CanonicalValueType::Real
            | CanonicalValueType::Integer
            | CanonicalValueType::NatureAccess => Some(OptValueType::Real),
            CanonicalValueType::String
            | CanonicalValueType::Void
            | CanonicalValueType::Unknown
            | CanonicalValueType::Error => None,
        }
    }

    fn variable_identifier(&self, expr: ExprId) -> Option<VariableId> {
        let expression = self.mir.expressions.get(usize::from(expr))?;
        let HirExprKind::Identifier { name } = &expression.kind else {
            return None;
        };
        self.hir?
            .variables
            .iter()
            .find(|variable| variable.name == *name)
            .map(|variable| variable.id)
    }

    fn number_constant_expr(&self, expr: ExprId) -> Option<f64> {
        let expression = self.mir.expressions.get(usize::from(expr))?;
        let HirExprKind::Number { value, .. } = expression.kind else {
            return None;
        };
        Some(value)
    }

    fn expr_references_any_variable(&self, expr: ExprId, variables: &HashSet<VariableId>) -> bool {
        let mut visited = HashSet::new();
        let mut stack = vec![expr];
        while let Some(expr) = stack.pop() {
            if !visited.insert(expr) {
                continue;
            }
            let Some(expression) = self.mir.expressions.get(usize::from(expr)) else {
                return true;
            };
            match &expression.kind {
                HirExprKind::Identifier { .. } => {
                    if self
                        .variable_identifier(expr)
                        .is_some_and(|variable| variables.contains(&variable))
                    {
                        return true;
                    }
                }
                HirExprKind::Binary { left, right, .. } => {
                    stack.extend([*left, *right]);
                }
                HirExprKind::Unary { operand, .. } => {
                    stack.push(*operand);
                }
                HirExprKind::Conditional {
                    condition,
                    then_expr,
                    else_expr,
                } => {
                    stack.extend([*condition, *then_expr, *else_expr]);
                }
                HirExprKind::Call { args, .. } | HirExprKind::SystemFunction { args, .. } => {
                    stack.extend(args.iter().copied());
                }
                HirExprKind::AnalogOperator {
                    op: HirAnalogOperator::Limexp { expr },
                } => stack.push(*expr),
                HirExprKind::Number { .. }
                | HirExprKind::StringLiteral { .. }
                | HirExprKind::BranchAccess { .. }
                | HirExprKind::NamedBranchAccess { .. } => {}
                HirExprKind::ArrayAccess { .. }
                | HirExprKind::ArrayLiteral { .. }
                | HirExprKind::AnalogOperator { .. }
                | HirExprKind::Laplace { .. }
                | HirExprKind::Zi { .. }
                | HirExprKind::NoiseSource { .. } => return true,
            }
        }
        false
    }

    fn mark_loop_assignments_unknown(&mut self, loop_statement: &HirLoop) {
        self.mark_statement_assignments_unknown(&loop_statement.body);
        self.expression_values.clear();
    }

    fn mark_statement_assignments_unknown(&mut self, statements: &[HirStatement]) {
        self.clear_assignment_replay_caches();
        for statement in statements {
            match statement {
                HirStatement::Assignment(assignment) => {
                    self.variable_values.insert(assignment.target, None);
                    self.variable_assignment_exprs.remove(&assignment.target);
                    self.variable_assignment_history.remove(&assignment.target);
                    self.guarded_path_assignment_exprs
                        .remove(&assignment.target);
                }
                HirStatement::Loop(loop_statement) => {
                    self.mark_statement_assignments_unknown(&loop_statement.body);
                }
            }
        }
    }

    fn lower_expression(&mut self, expr_id: ExprId) -> Option<ValueId> {
        let use_cache = self.conditional_path_stack.is_empty()
            && self.assignment_history_value_snapshot_stack.is_empty()
            && self.assignment_history_snapshot_stack.is_empty()
            && self.assignment_history_previous_value_stack.is_empty();
        if use_cache && let Some(value) = self.expression_values.get(&expr_id) {
            return *value;
        }

        if self.expression_lowering_depth >= MAX_SCALAR_EXPRESSION_LOWERING_DEPTH
            || !self.expression_lowering_stack.insert(expr_id)
        {
            return None;
        }

        self.expression_lowering_depth += 1;
        let lowered = self.lower_expression_inner(expr_id);
        self.expression_lowering_depth -= 1;
        self.expression_lowering_stack.remove(&expr_id);

        if use_cache {
            self.expression_values.insert(expr_id, lowered);
        }
        lowered
    }

    fn lower_expression_inner(&mut self, expr_id: ExprId) -> Option<ValueId> {
        let expression = self.mir.expressions.get(usize::from(expr_id))?;
        match &expression.kind {
            HirExprKind::Number { value, .. } => {
                Some(self.push_value(OptValueType::Real, OptValueKind::RealConstant(*value)))
            }
            HirExprKind::Identifier { name } => self.lower_identifier(name),
            HirExprKind::SystemFunction { name, args } => self.lower_system_function(name, args),
            HirExprKind::BranchAccess { access, pos, neg } => {
                self.lower_branch_access(access, pos, neg.as_deref())
            }
            HirExprKind::NamedBranchAccess { access, name } => {
                self.lower_named_branch_access(access, name)
            }
            HirExprKind::Binary { op, left, right } => self.lower_binary(op, *left, *right),
            HirExprKind::Unary { op, operand } => self.lower_unary(op, *operand),
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => self.lower_conditional(*condition, *then_expr, *else_expr),
            HirExprKind::Call { name, args } => self.lower_call(expr_id, name, args),
            HirExprKind::NoiseSource { .. } => Some(self.zero_real()),
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Ddt { expr, abstol: None },
            } => self.lower_ddt_expression(*expr, expr_id),
            HirExprKind::AnalogOperator {
                op:
                    HirAnalogOperator::Idt {
                        expr,
                        assert: None,
                        abstol: None,
                        ..
                    },
            } if self.in_potential_equation_state_operator_context() => {
                self.lower_expression(*expr)
            }
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Ddx { expr, probe },
            } => self.lower_ddx_expression(*expr, *probe),
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Limexp { expr },
            } => self.lower_intrinsic_unary(OptUnaryOp::LimExp, *expr),
            _ => None,
        }
    }

    fn lower_equation_expression(&mut self, equation: &MirEquation) -> Option<ValueId> {
        self.expression_values.clear();
        let previous_context = self.branch_flow_context;
        let previous_state_operator_depth = self.potential_equation_state_operator_depth;
        self.branch_flow_context = self.equation_branch_unknown(equation);
        if equation.kind == MirEquationKind::Potential {
            self.potential_equation_state_operator_depth += 1;
        }
        let lowered = self.lower_equation_expression_inner(equation.expression.id);
        self.potential_equation_state_operator_depth = previous_state_operator_depth;
        self.branch_flow_context = previous_context;
        self.expression_values.clear();
        lowered
    }

    fn lower_equation_expression_inner(&mut self, expr: ExprId) -> Option<ValueId> {
        let expression = self.mir.expressions.get(usize::from(expr))?;
        match &expression.kind {
            HirExprKind::AnalogOperator {
                op: HirAnalogOperator::Ddt { expr, abstol: None },
            } => self.lower_expression(*expr),
            HirExprKind::AnalogOperator {
                op:
                    HirAnalogOperator::Idt {
                        expr,
                        assert: None,
                        abstol: None,
                        ..
                    },
            } => self.lower_expression(*expr),
            HirExprKind::Call { name, args }
                if name.eq_ignore_ascii_case("ddt") && args.len() == 1 =>
            {
                self.lower_expression(args[0])
            }
            HirExprKind::Call { name, args }
                if name.eq_ignore_ascii_case("idt") && (1..=2).contains(&args.len()) =>
            {
                self.lower_expression(args[0])
            }
            _ => self.lower_expression(expr),
        }
    }

    fn equation_branch_unknown(&self, equation: &MirEquation) -> Option<BranchUnknownId> {
        if equation.kind != MirEquationKind::Potential {
            return None;
        }
        self.mir
            .branch_unknowns
            .iter()
            .find(|unknown| unknown.equation == equation.id)
            .map(|unknown| unknown.id)
    }

    fn push_value(&mut self, value_type: OptValueType, kind: OptValueKind) -> ValueId {
        let (value_type, kind) = self.simplify_value(value_type, kind);

        let key = OptValueKey::from_kind(&kind);
        if let Some(value) = self.value_keys.get(&key) {
            return *value;
        }

        let id = ValueId::from(self.values.len());
        self.values.push(OptValue {
            id,
            value_type,
            kind,
            derivatives: Vec::new(),
        });
        self.value_keys.insert(key, id);
        id
    }

    fn simplify_value(
        &self,
        mut value_type: OptValueType,
        mut kind: OptValueKind,
    ) -> (OptValueType, OptValueKind) {
        loop {
            let folded = self.fold_constant_value(value_type, kind);
            value_type = folded.0;
            kind = self.normalize_value_kind(folded.1);

            if let Some(existing) = self.simplified_existing_value(&kind) {
                return (
                    self.values[usize::from(existing)].value_type,
                    self.values[usize::from(existing)].kind.clone(),
                );
            }

            if let Some(next) = self.simplified_replacement_kind(&kind) {
                value_type = next.0;
                kind = next.1;
                continue;
            }

            return (value_type, kind);
        }
    }

    fn fold_constant_value(
        &self,
        value_type: OptValueType,
        kind: OptValueKind,
    ) -> (OptValueType, OptValueKind) {
        match kind {
            OptValueKind::Unary { op, input } => {
                if let Some(value) = self.real_constant(input) {
                    let folded = match op {
                        OptUnaryOp::Pos => Some(value),
                        OptUnaryOp::Neg => Some(-value),
                        OptUnaryOp::Exp => Some(value.exp()),
                        OptUnaryOp::LimExp => Some(limexp_value(value)),
                        OptUnaryOp::LimExpDerivative => Some(limexp_derivative(value)),
                        OptUnaryOp::LimitedExp => Some(limited_exp_value(value)),
                        OptUnaryOp::LimitedExpDerivative => Some(limited_exp_derivative(value)),
                        OptUnaryOp::Ln => Some(value.ln()),
                        OptUnaryOp::Sqrt => Some(value.sqrt()),
                        OptUnaryOp::Abs => Some(value.abs()),
                        OptUnaryOp::Sin => Some(value.sin()),
                        OptUnaryOp::Cos => Some(value.cos()),
                        OptUnaryOp::Tan => Some(value.tan()),
                        OptUnaryOp::Sinh => Some(value.sinh()),
                        OptUnaryOp::Cosh => Some(value.cosh()),
                        OptUnaryOp::Tanh => Some(value.tanh()),
                        OptUnaryOp::Atan => Some(value.atan()),
                        OptUnaryOp::Asinh => Some(value.asinh()),
                        OptUnaryOp::Floor => Some(value.floor()),
                        OptUnaryOp::Ceil => Some(value.ceil()),
                        OptUnaryOp::Not => None,
                    };
                    if let Some(folded) = folded {
                        return (OptValueType::Real, OptValueKind::RealConstant(folded));
                    }
                }
                if let (OptUnaryOp::Not, Some(value)) = (op, self.constant_truth(input)) {
                    return (OptValueType::Boolean, OptValueKind::BooleanConstant(!value));
                }
                (value_type, OptValueKind::Unary { op, input })
            }
            OptValueKind::Binary { op, left, right } => {
                if let Some(folded) = self.fold_constant_binary(op, left, right) {
                    return folded;
                }
                (value_type, OptValueKind::Binary { op, left, right })
            }
            OptValueKind::Select {
                condition,
                then_value,
                else_value,
            } => {
                if let Some(condition) = self.boolean_constant(condition) {
                    return if condition {
                        (
                            self.values[usize::from(then_value)].value_type,
                            self.values[usize::from(then_value)].kind.clone(),
                        )
                    } else {
                        (
                            self.values[usize::from(else_value)].value_type,
                            self.values[usize::from(else_value)].kind.clone(),
                        )
                    };
                }
                (
                    value_type,
                    OptValueKind::Select {
                        condition,
                        then_value,
                        else_value,
                    },
                )
            }
            other => (value_type, other),
        }
    }

    fn fold_constant_binary(
        &self,
        op: OptBinaryOp,
        left: ValueId,
        right: ValueId,
    ) -> Option<(OptValueType, OptValueKind)> {
        if matches!(op, OptBinaryOp::And | OptBinaryOp::Or)
            && let (Some(left), Some(right)) =
                (self.constant_truth(left), self.constant_truth(right))
        {
            let value = match op {
                OptBinaryOp::And => left && right,
                OptBinaryOp::Or => left || right,
                _ => unreachable!(),
            };
            return Some((OptValueType::Boolean, OptValueKind::BooleanConstant(value)));
        }

        if let (Some(left), Some(right)) = (self.real_constant(left), self.real_constant(right)) {
            let real = |value| Some((OptValueType::Real, OptValueKind::RealConstant(value)));
            let boolean =
                |value| Some((OptValueType::Boolean, OptValueKind::BooleanConstant(value)));
            return match op {
                OptBinaryOp::Add => real(left + right),
                OptBinaryOp::Sub => real(left - right),
                OptBinaryOp::Mul => real(left * right),
                OptBinaryOp::Div => real(left / right),
                OptBinaryOp::Pow => real(left.powf(right)),
                OptBinaryOp::Eq => boolean(left == right),
                OptBinaryOp::Ne => boolean(left != right),
                OptBinaryOp::Lt => boolean(left < right),
                OptBinaryOp::Le => boolean(left <= right),
                OptBinaryOp::Gt => boolean(left > right),
                OptBinaryOp::Ge => boolean(left >= right),
                OptBinaryOp::And | OptBinaryOp::Or => None,
            };
        }

        if let (Some(left), Some(right)) =
            (self.boolean_constant(left), self.boolean_constant(right))
        {
            let boolean =
                |value| Some((OptValueType::Boolean, OptValueKind::BooleanConstant(value)));
            return match op {
                OptBinaryOp::Eq => boolean(left == right),
                OptBinaryOp::Ne => boolean(left != right),
                OptBinaryOp::And => boolean(left && right),
                OptBinaryOp::Or => boolean(left || right),
                _ => None,
            };
        }

        None
    }

    fn normalize_value_kind(&self, kind: OptValueKind) -> OptValueKind {
        match kind {
            OptValueKind::Binary { op, left, right }
                if Self::commutative_binary_op(op) && usize::from(right) < usize::from(left) =>
            {
                OptValueKind::Binary {
                    op,
                    left: right,
                    right: left,
                }
            }
            other => other,
        }
    }

    fn commutative_binary_op(op: OptBinaryOp) -> bool {
        matches!(
            op,
            OptBinaryOp::Add
                | OptBinaryOp::Mul
                | OptBinaryOp::Eq
                | OptBinaryOp::Ne
                | OptBinaryOp::And
                | OptBinaryOp::Or
        )
    }

    fn simplified_existing_value(&self, kind: &OptValueKind) -> Option<ValueId> {
        match kind {
            OptValueKind::Unary {
                op: OptUnaryOp::Pos,
                input,
            } => Some(*input),
            OptValueKind::Unary {
                op: OptUnaryOp::Neg,
                input,
            } => match self
                .values
                .get(usize::from(*input))
                .map(|value| &value.kind)
            {
                Some(OptValueKind::Unary {
                    op: OptUnaryOp::Neg,
                    input,
                }) => Some(*input),
                _ => None,
            },
            OptValueKind::Binary {
                op: OptBinaryOp::Add,
                left,
                right,
            } if self.is_real_constant(*left, 0.0) => Some(*right),
            OptValueKind::Binary {
                op: OptBinaryOp::Add,
                left,
                right,
            } if self.is_real_constant(*right, 0.0) => Some(*left),
            OptValueKind::Binary {
                op: OptBinaryOp::Sub,
                left,
                right,
            } if self.is_real_constant(*right, 0.0) => Some(*left),
            OptValueKind::Binary {
                op: OptBinaryOp::Mul,
                left,
                right,
            } if self.is_real_constant(*left, 1.0) => Some(*right),
            OptValueKind::Binary {
                op: OptBinaryOp::Mul,
                left,
                right,
            } if self.is_real_constant(*right, 1.0) => Some(*left),
            OptValueKind::Binary {
                op: OptBinaryOp::Div,
                left,
                right,
            } if self.is_real_constant(*right, 1.0) => Some(*left),
            OptValueKind::Select {
                then_value,
                else_value,
                ..
            } if then_value == else_value => Some(*then_value),
            _ => None,
        }
    }

    fn simplified_replacement_kind(
        &self,
        kind: &OptValueKind,
    ) -> Option<(OptValueType, OptValueKind)> {
        match kind {
            OptValueKind::Binary {
                op: OptBinaryOp::Mul,
                left,
                right,
            } if self.is_real_constant(*left, -1.0) => Some((
                OptValueType::Real,
                OptValueKind::Unary {
                    op: OptUnaryOp::Neg,
                    input: *right,
                },
            )),
            OptValueKind::Binary {
                op: OptBinaryOp::Mul,
                left,
                right,
            } if self.is_real_constant(*right, -1.0) => Some((
                OptValueType::Real,
                OptValueKind::Unary {
                    op: OptUnaryOp::Neg,
                    input: *left,
                },
            )),
            OptValueKind::Binary {
                op: OptBinaryOp::Div,
                left,
                right,
            } if self.is_real_constant(*right, -1.0) => Some((
                OptValueType::Real,
                OptValueKind::Unary {
                    op: OptUnaryOp::Neg,
                    input: *left,
                },
            )),
            _ => None,
        }
    }

    fn is_real_constant(&self, value: ValueId, expected: f64) -> bool {
        matches!(
            self.values.get(usize::from(value)).map(|value| &value.kind),
            Some(OptValueKind::RealConstant(actual)) if actual.to_bits() == expected.to_bits()
        )
    }

    fn real_constant(&self, value: ValueId) -> Option<f64> {
        match self.values.get(usize::from(value)).map(|value| &value.kind) {
            Some(OptValueKind::RealConstant(value)) => Some(*value),
            _ => None,
        }
    }

    fn boolean_constant(&self, value: ValueId) -> Option<bool> {
        match self.values.get(usize::from(value)).map(|value| &value.kind) {
            Some(OptValueKind::BooleanConstant(value)) => Some(*value),
            _ => None,
        }
    }

    fn constant_truth(&self, value: ValueId) -> Option<bool> {
        self.boolean_constant(value)
            .or_else(|| self.real_constant(value).map(real_truth_value))
    }

    fn add_sparse_derivatives(&mut self) {
        let primal_count = self.values.len();
        for index in 0..primal_count {
            let value = ValueId::from(index);
            let derivatives = self.lower_value_derivatives(value);
            self.values[index].derivatives = derivatives
                .into_iter()
                .filter(|(_, value)| !self.is_real_constant(*value, 0.0))
                .map(|(lane, value)| OptDerivative { lane, value })
                .collect();
        }
    }

    fn lower_value_derivatives(&mut self, value: ValueId) -> BTreeMap<DerivativeLane, ValueId> {
        match self.values[usize::from(value)].kind.clone() {
            OptValueKind::RealConstant(_)
            | OptValueKind::BooleanConstant(_)
            | OptValueKind::Parameter { .. }
            | OptValueKind::ParamGiven { .. }
            | OptValueKind::Temperature
            | OptValueKind::ThermalVoltage
            | OptValueKind::Multiplicity
            | OptValueKind::Time
            | OptValueKind::Analysis { .. }
            | OptValueKind::Ddx { .. }
            | OptValueKind::DdtScale
            | OptValueKind::LoopIndex { .. }
            | OptValueKind::RuntimeLoopVariableDerivative { .. }
            | OptValueKind::RuntimeLoopResultDerivative { .. }
            | OptValueKind::EquationValue { .. } => BTreeMap::new(),
            OptValueKind::RuntimeLoopVariable { loop_id, slot } => self
                .all_derivative_lanes()
                .into_iter()
                .map(|lane| {
                    (
                        lane,
                        self.push_value(
                            OptValueType::Real,
                            OptValueKind::RuntimeLoopVariableDerivative {
                                loop_id,
                                slot,
                                lane,
                            },
                        ),
                    )
                })
                .collect(),
            OptValueKind::RuntimeLoopResult { loop_id, slot } => self
                .all_derivative_lanes()
                .into_iter()
                .map(|lane| {
                    (
                        lane,
                        self.push_value(
                            OptValueType::Real,
                            OptValueKind::RuntimeLoopResultDerivative {
                                loop_id,
                                slot,
                                lane,
                            },
                        ),
                    )
                })
                .collect(),
            OptValueKind::CountedSum {
                loop_id,
                count,
                initial,
                term,
            } => self.lower_counted_sum_derivatives(loop_id, count, initial, term),
            OptValueKind::Ddt { input, .. } => self.lower_ddt_derivatives(input),
            OptValueKind::NodePotential { node } => {
                let derivative =
                    self.push_value(OptValueType::Real, OptValueKind::RealConstant(1.0));
                BTreeMap::from([(DerivativeLane::node(node), derivative)])
            }
            OptValueKind::BranchFlow { branch } => {
                if let Some(branch_unknown) = self.branch_unknown_for_branch(branch) {
                    let derivative =
                        self.push_value(OptValueType::Real, OptValueKind::RealConstant(1.0));
                    BTreeMap::from([(DerivativeLane::branch_unknown(branch_unknown), derivative)])
                } else {
                    BTreeMap::new()
                }
            }
            OptValueKind::BranchUnknownFlow { branch_unknown } => {
                let derivative =
                    self.push_value(OptValueType::Real, OptValueKind::RealConstant(1.0));
                BTreeMap::from([(DerivativeLane::branch_unknown(branch_unknown), derivative)])
            }
            OptValueKind::Unary { op, input } => self.lower_unary_derivatives(value, op, input),
            OptValueKind::Binary { op, left, right } => {
                self.lower_binary_derivatives(value, op, left, right)
            }
            OptValueKind::Select {
                condition,
                then_value,
                else_value,
            } => self.lower_select_derivatives(condition, then_value, else_value),
        }
    }

    fn all_derivative_lanes(&self) -> Vec<DerivativeLane> {
        let mut lanes = Vec::with_capacity(
            self.mir
                .nodes
                .len()
                .saturating_add(self.mir.branch_unknowns.len()),
        );
        lanes.extend(
            (0..self.mir.nodes.len()).map(|index| DerivativeLane::node(NodeId::from(index))),
        );
        lanes.extend(
            (0..self.mir.branch_unknowns.len())
                .map(|index| DerivativeLane::branch_unknown(BranchUnknownId::from(index))),
        );
        lanes
    }

    fn lower_unary_derivatives(
        &mut self,
        value: ValueId,
        op: OptUnaryOp,
        input: ValueId,
    ) -> BTreeMap<DerivativeLane, ValueId> {
        let input_derivatives = self.derivative_map(input);
        let mut derivatives = BTreeMap::new();

        for (lane, input_derivative) in input_derivatives {
            let derivative = match op {
                OptUnaryOp::Pos => input_derivative,
                OptUnaryOp::Neg => self.push_value(
                    OptValueType::Real,
                    OptValueKind::Unary {
                        op: OptUnaryOp::Neg,
                        input: input_derivative,
                    },
                ),
                OptUnaryOp::Exp => {
                    self.push_binary_value(OptBinaryOp::Mul, value, input_derivative)
                }
                OptUnaryOp::LimExp => {
                    let scale = self.push_value(
                        OptValueType::Real,
                        OptValueKind::Unary {
                            op: OptUnaryOp::LimExpDerivative,
                            input,
                        },
                    );
                    self.push_binary_value(OptBinaryOp::Mul, scale, input_derivative)
                }
                OptUnaryOp::LimitedExp => {
                    let scale = self.push_value(
                        OptValueType::Real,
                        OptValueKind::Unary {
                            op: OptUnaryOp::LimitedExpDerivative,
                            input,
                        },
                    );
                    self.push_binary_value(OptBinaryOp::Mul, scale, input_derivative)
                }
                OptUnaryOp::Ln => self.push_binary_value(OptBinaryOp::Div, input_derivative, input),
                OptUnaryOp::Sqrt => {
                    let two = self.push_value(OptValueType::Real, OptValueKind::RealConstant(2.0));
                    let denominator = self.push_binary_value(OptBinaryOp::Mul, two, value);
                    self.push_binary_value(OptBinaryOp::Div, input_derivative, denominator)
                }
                OptUnaryOp::Sin => {
                    let cos = self.push_value(
                        OptValueType::Real,
                        OptValueKind::Unary {
                            op: OptUnaryOp::Cos,
                            input,
                        },
                    );
                    self.push_binary_value(OptBinaryOp::Mul, cos, input_derivative)
                }
                OptUnaryOp::Cos => {
                    let sin = self.push_value(
                        OptValueType::Real,
                        OptValueKind::Unary {
                            op: OptUnaryOp::Sin,
                            input,
                        },
                    );
                    let scaled = self.push_binary_value(OptBinaryOp::Mul, sin, input_derivative);
                    self.push_value(
                        OptValueType::Real,
                        OptValueKind::Unary {
                            op: OptUnaryOp::Neg,
                            input: scaled,
                        },
                    )
                }
                OptUnaryOp::Tan => {
                    let cos = self.push_value(
                        OptValueType::Real,
                        OptValueKind::Unary {
                            op: OptUnaryOp::Cos,
                            input,
                        },
                    );
                    let denominator = self.push_binary_value(OptBinaryOp::Mul, cos, cos);
                    self.push_binary_value(OptBinaryOp::Div, input_derivative, denominator)
                }
                OptUnaryOp::Sinh => {
                    let cosh = self.push_value(
                        OptValueType::Real,
                        OptValueKind::Unary {
                            op: OptUnaryOp::Cosh,
                            input,
                        },
                    );
                    self.push_binary_value(OptBinaryOp::Mul, cosh, input_derivative)
                }
                OptUnaryOp::Cosh => {
                    let sinh = self.push_value(
                        OptValueType::Real,
                        OptValueKind::Unary {
                            op: OptUnaryOp::Sinh,
                            input,
                        },
                    );
                    self.push_binary_value(OptBinaryOp::Mul, sinh, input_derivative)
                }
                OptUnaryOp::Tanh => {
                    let one = self.push_value(OptValueType::Real, OptValueKind::RealConstant(1.0));
                    let square = self.push_binary_value(OptBinaryOp::Mul, value, value);
                    let scale = self.push_binary_value(OptBinaryOp::Sub, one, square);
                    self.push_binary_value(OptBinaryOp::Mul, scale, input_derivative)
                }
                OptUnaryOp::Atan => {
                    let one = self.push_value(OptValueType::Real, OptValueKind::RealConstant(1.0));
                    let square = self.push_binary_value(OptBinaryOp::Mul, input, input);
                    let denominator = self.push_binary_value(OptBinaryOp::Add, one, square);
                    self.push_binary_value(OptBinaryOp::Div, input_derivative, denominator)
                }
                OptUnaryOp::Asinh => {
                    let one = self.push_value(OptValueType::Real, OptValueKind::RealConstant(1.0));
                    let square = self.push_binary_value(OptBinaryOp::Mul, input, input);
                    let sum = self.push_binary_value(OptBinaryOp::Add, one, square);
                    let denominator = self.push_value(
                        OptValueType::Real,
                        OptValueKind::Unary {
                            op: OptUnaryOp::Sqrt,
                            input: sum,
                        },
                    );
                    self.push_binary_value(OptBinaryOp::Div, input_derivative, denominator)
                }
                OptUnaryOp::Abs
                | OptUnaryOp::Floor
                | OptUnaryOp::Ceil
                | OptUnaryOp::Not
                | OptUnaryOp::LimExpDerivative
                | OptUnaryOp::LimitedExpDerivative => continue,
            };
            derivatives.insert(lane, derivative);
        }

        derivatives
    }

    fn lower_binary_derivatives(
        &mut self,
        value: ValueId,
        op: OptBinaryOp,
        left: ValueId,
        right: ValueId,
    ) -> BTreeMap<DerivativeLane, ValueId> {
        match op {
            OptBinaryOp::Add => self.combine_binary_derivatives(
                left,
                right,
                |builder, left, right| builder.push_binary_value(OptBinaryOp::Add, left, right),
                |_, value| value,
                |_, value| value,
            ),
            OptBinaryOp::Sub => self.combine_binary_derivatives(
                left,
                right,
                |builder, left, right| builder.push_binary_value(OptBinaryOp::Sub, left, right),
                |_, value| value,
                |builder, value| {
                    builder.push_value(
                        OptValueType::Real,
                        OptValueKind::Unary {
                            op: OptUnaryOp::Neg,
                            input: value,
                        },
                    )
                },
            ),
            OptBinaryOp::Mul => self.product_derivatives(left, right),
            OptBinaryOp::Div => self.quotient_derivatives(left, right),
            OptBinaryOp::Pow => self.pow_derivatives(value, left, right),
            OptBinaryOp::Eq
            | OptBinaryOp::Ne
            | OptBinaryOp::Lt
            | OptBinaryOp::Le
            | OptBinaryOp::Gt
            | OptBinaryOp::Ge
            | OptBinaryOp::And
            | OptBinaryOp::Or => BTreeMap::new(),
        }
    }

    fn combine_binary_derivatives(
        &mut self,
        left: ValueId,
        right: ValueId,
        both: impl Fn(&mut Self, ValueId, ValueId) -> ValueId,
        only_left: impl Fn(&mut Self, ValueId) -> ValueId,
        only_right: impl Fn(&mut Self, ValueId) -> ValueId,
    ) -> BTreeMap<DerivativeLane, ValueId> {
        let left_derivatives = self.derivative_map(left);
        let right_derivatives = self.derivative_map(right);
        let mut lanes: BTreeSet<_> = left_derivatives.keys().copied().collect();
        lanes.extend(right_derivatives.keys().copied());

        let mut derivatives = BTreeMap::new();
        for lane in lanes {
            let derivative = match (
                left_derivatives.get(&lane).copied(),
                right_derivatives.get(&lane).copied(),
            ) {
                (Some(left), Some(right)) => both(self, left, right),
                (Some(left), None) => only_left(self, left),
                (None, Some(right)) => only_right(self, right),
                (None, None) => continue,
            };
            derivatives.insert(lane, derivative);
        }

        derivatives
    }

    fn product_derivatives(
        &mut self,
        left: ValueId,
        right: ValueId,
    ) -> BTreeMap<DerivativeLane, ValueId> {
        let left_derivatives = self.derivative_map(left);
        let right_derivatives = self.derivative_map(right);
        let mut lanes: BTreeSet<_> = left_derivatives.keys().copied().collect();
        lanes.extend(right_derivatives.keys().copied());

        let mut derivatives = BTreeMap::new();
        for lane in lanes {
            let left_term = left_derivatives
                .get(&lane)
                .copied()
                .map(|derivative| self.push_binary_value(OptBinaryOp::Mul, derivative, right));
            let right_term = right_derivatives
                .get(&lane)
                .copied()
                .map(|derivative| self.push_binary_value(OptBinaryOp::Mul, left, derivative));

            let derivative = match (left_term, right_term) {
                (Some(left_term), Some(right_term)) => {
                    self.push_binary_value(OptBinaryOp::Add, left_term, right_term)
                }
                (Some(term), None) | (None, Some(term)) => term,
                (None, None) => continue,
            };
            derivatives.insert(lane, derivative);
        }

        derivatives
    }

    fn pow_derivatives(
        &mut self,
        value: ValueId,
        left: ValueId,
        right: ValueId,
    ) -> BTreeMap<DerivativeLane, ValueId> {
        let left_derivatives = self.derivative_map(left);
        let right_derivatives = self.derivative_map(right);
        let mut lanes: BTreeSet<_> = left_derivatives.keys().copied().collect();
        lanes.extend(right_derivatives.keys().copied());

        let mut derivatives = BTreeMap::new();
        for lane in lanes {
            let left_term = left_derivatives
                .get(&lane)
                .copied()
                .map(|derivative| self.pow_base_derivative(left, right, derivative));
            let right_term = right_derivatives
                .get(&lane)
                .copied()
                .map(|derivative| self.pow_exponent_derivative(value, left, derivative));

            let derivative = match (left_term, right_term) {
                (Some(left_term), Some(right_term)) => {
                    self.push_binary_value(OptBinaryOp::Add, left_term, right_term)
                }
                (Some(term), None) | (None, Some(term)) => term,
                (None, None) => continue,
            };
            derivatives.insert(lane, derivative);
        }

        derivatives
    }

    fn lower_counted_sum_derivatives(
        &mut self,
        loop_id: u32,
        count: ValueId,
        initial: ValueId,
        term: ValueId,
    ) -> BTreeMap<DerivativeLane, ValueId> {
        let initial_derivatives = self.derivative_map(initial);
        let term_derivatives = self.derivative_map(term);
        let mut lanes: BTreeSet<_> = initial_derivatives.keys().copied().collect();
        lanes.extend(term_derivatives.keys().copied());

        let mut derivatives = BTreeMap::new();
        for lane in lanes {
            let initial = initial_derivatives
                .get(&lane)
                .copied()
                .unwrap_or_else(|| self.zero_real());
            let Some(term) = term_derivatives.get(&lane).copied() else {
                derivatives.insert(lane, initial);
                continue;
            };
            let derivative = self.push_value(
                OptValueType::Real,
                OptValueKind::CountedSum {
                    loop_id,
                    count,
                    initial,
                    term,
                },
            );
            derivatives.insert(lane, derivative);
        }

        derivatives
    }

    fn lower_ddt_derivatives(&mut self, input: ValueId) -> BTreeMap<DerivativeLane, ValueId> {
        let input_derivatives = self.derivative_map(input);
        let scale = self.ddt_scale();
        input_derivatives
            .into_iter()
            .map(|(lane, derivative)| {
                (
                    lane,
                    self.push_binary_value(OptBinaryOp::Mul, derivative, scale),
                )
            })
            .collect()
    }

    fn pow_base_derivative(
        &mut self,
        left: ValueId,
        right: ValueId,
        derivative: ValueId,
    ) -> ValueId {
        if self.is_real_constant(right, 0.0) {
            return self.push_value(OptValueType::Real, OptValueKind::RealConstant(0.0));
        }

        let one = self.push_value(OptValueType::Real, OptValueKind::RealConstant(1.0));
        let exponent_minus_one = self.push_binary_value(OptBinaryOp::Sub, right, one);
        let reduced_power = self.push_binary_value(OptBinaryOp::Pow, left, exponent_minus_one);
        let scaled_power = self.push_binary_value(OptBinaryOp::Mul, right, reduced_power);
        self.push_binary_value(OptBinaryOp::Mul, scaled_power, derivative)
    }

    fn pow_exponent_derivative(
        &mut self,
        value: ValueId,
        left: ValueId,
        derivative: ValueId,
    ) -> ValueId {
        let ln_base = self.push_value(
            OptValueType::Real,
            OptValueKind::Unary {
                op: OptUnaryOp::Ln,
                input: left,
            },
        );
        let scaled = self.push_binary_value(OptBinaryOp::Mul, value, ln_base);
        self.push_binary_value(OptBinaryOp::Mul, scaled, derivative)
    }

    fn quotient_derivatives(
        &mut self,
        left: ValueId,
        right: ValueId,
    ) -> BTreeMap<DerivativeLane, ValueId> {
        let left_derivatives = self.derivative_map(left);
        let right_derivatives = self.derivative_map(right);
        let mut lanes: BTreeSet<_> = left_derivatives.keys().copied().collect();
        lanes.extend(right_derivatives.keys().copied());

        let mut derivatives = BTreeMap::new();
        for lane in lanes {
            let numerator = match (
                left_derivatives.get(&lane).copied(),
                right_derivatives.get(&lane).copied(),
            ) {
                (Some(left_derivative), Some(right_derivative)) => {
                    let left_term =
                        self.push_binary_value(OptBinaryOp::Mul, left_derivative, right);
                    let right_term =
                        self.push_binary_value(OptBinaryOp::Mul, left, right_derivative);
                    self.push_binary_value(OptBinaryOp::Sub, left_term, right_term)
                }
                (Some(left_derivative), None) => {
                    derivatives.insert(
                        lane,
                        self.push_binary_value(OptBinaryOp::Div, left_derivative, right),
                    );
                    continue;
                }
                (None, Some(right_derivative)) => {
                    let right_term =
                        self.push_binary_value(OptBinaryOp::Mul, left, right_derivative);
                    self.push_value(
                        OptValueType::Real,
                        OptValueKind::Unary {
                            op: OptUnaryOp::Neg,
                            input: right_term,
                        },
                    )
                }
                (None, None) => continue,
            };
            let denominator = self.push_binary_value(OptBinaryOp::Mul, right, right);
            let derivative = self.push_binary_value(OptBinaryOp::Div, numerator, denominator);
            derivatives.insert(lane, derivative);
        }

        derivatives
    }

    fn lower_select_derivatives(
        &mut self,
        condition: ValueId,
        then_value: ValueId,
        else_value: ValueId,
    ) -> BTreeMap<DerivativeLane, ValueId> {
        let then_derivatives = self.derivative_map(then_value);
        let else_derivatives = self.derivative_map(else_value);
        let mut lanes: BTreeSet<_> = then_derivatives.keys().copied().collect();
        lanes.extend(else_derivatives.keys().copied());

        let mut derivatives = BTreeMap::new();
        for lane in lanes {
            let then_derivative = then_derivatives.get(&lane).copied().unwrap_or_else(|| {
                self.push_value(OptValueType::Real, OptValueKind::RealConstant(0.0))
            });
            let else_derivative = else_derivatives.get(&lane).copied().unwrap_or_else(|| {
                self.push_value(OptValueType::Real, OptValueKind::RealConstant(0.0))
            });
            let derivative = self.push_value(
                OptValueType::Real,
                OptValueKind::Select {
                    condition,
                    then_value: then_derivative,
                    else_value: else_derivative,
                },
            );
            derivatives.insert(lane, derivative);
        }

        derivatives
    }

    fn derivative_map(&self, value: ValueId) -> BTreeMap<DerivativeLane, ValueId> {
        self.values[usize::from(value)]
            .derivatives
            .iter()
            .map(|derivative| (derivative.lane, derivative.value))
            .collect()
    }

    fn derivative_value(&self, value: ValueId, lane: DerivativeLane) -> Option<ValueId> {
        self.values
            .get(usize::from(value))?
            .derivatives
            .iter()
            .find(|derivative| derivative.lane == lane)
            .map(|derivative| derivative.value)
    }

    fn push_binary_value(&mut self, op: OptBinaryOp, left: ValueId, right: ValueId) -> ValueId {
        self.push_value(OptValueType::Real, OptValueKind::Binary { op, left, right })
    }

    fn lower_identifier(&mut self, name: &SmolStr) -> Option<ValueId> {
        if let Some(value) = self.lower_variable_identifier(name) {
            return value;
        }

        let parameter = self
            .mir
            .parameters
            .iter()
            .find(|parameter| parameter.name == *name)
            .map(|parameter| parameter.id)?;

        Some(self.push_value(OptValueType::Real, OptValueKind::Parameter { parameter }))
    }

    fn lower_variable_identifier(&mut self, name: &SmolStr) -> Option<Option<ValueId>> {
        let variable = self
            .hir?
            .variables
            .iter()
            .find(|variable| variable.name == *name)?;
        if let Some(previous_value) = self.assignment_history_previous_value(variable.id) {
            return Some(Some(previous_value));
        }
        if let Some(value) = self.assignment_history_value_snapshot(variable.id) {
            return Some(Some(value));
        }
        if let Some(limit) = self.assignment_history_snapshot_limit(variable.id) {
            return Some(self.lower_assignment_history_prefix(variable.id, limit));
        }
        if let Some(value) = self.variable_values.get(&variable.id).copied() {
            let has_assignment_history =
                self.variable_assignment_history.contains_key(&variable.id);
            let has_guarded_path_assignment = self
                .guarded_path_assignment_exprs
                .contains_key(&variable.id);
            if value.is_none()
                && variable.name.starts_with("__guard")
                && let Some(guard) = self.lower_guard_alias_variable_identifier(variable.id)
            {
                return Some(Some(guard));
            }
            if value.is_none()
                && (has_assignment_history || has_guarded_path_assignment)
                && let Some(contextual) =
                    self.lower_conditional_path_variable_identifier(variable.id)
            {
                return Some(Some(contextual));
            }
            if value.is_none()
                && has_assignment_history
                && let Some(history_value) = self.lower_recent_assignment_history_value(variable.id)
            {
                return Some(Some(history_value));
            }
            if value.is_none()
                && let Some(contextual) = self.lower_contextual_variable_identifier(variable.id)
            {
                return Some(Some(contextual));
            }
            if value.is_none()
                && self.guard_alias_lowering_depth > 0
                && has_assignment_history
                && let Some(guard_history) = self.lower_assignment_history_value(variable.id)
            {
                return Some(Some(guard_history));
            }
            if value.is_none()
                && has_assignment_history
                && let Some(history_value) =
                    self.lower_complementary_assignment_history_value(variable.id)
            {
                return Some(Some(history_value));
            }
            if value.is_none()
                && has_assignment_history
                && let Some(history_value) = self.lower_assignment_history_value(variable.id)
            {
                return Some(Some(history_value));
            }
            return Some(value);
        }

        Some(Some(self.default_variable_value(variable.value_type)?))
    }

    fn lower_guard_alias_variable_identifier(&mut self, variable: VariableId) -> Option<ValueId> {
        if !self.variable_lowering_stack.insert(variable) {
            return None;
        }
        let expr = self.variable_assignment_exprs.get(&variable).copied();
        self.guard_alias_lowering_depth += 1;
        let lowered = expr.and_then(|expr| self.lower_expression(expr));
        self.guard_alias_lowering_depth -= 1;
        self.variable_lowering_stack.remove(&variable);
        lowered
    }

    fn lower_assignment_history_value(&mut self, variable: VariableId) -> Option<ValueId> {
        let limit = self.variable_assignment_history.get(&variable)?.len();
        if limit > MAX_SCALAR_GUARD_HISTORY_RECONSTRUCTION_ENTRIES {
            return None;
        }
        self.lower_assignment_history_prefix(variable, limit)
    }

    fn lower_assignment_history_prefix(
        &mut self,
        variable: VariableId,
        limit: usize,
    ) -> Option<ValueId> {
        let cache_key = self.assignment_history_prefix_cache_key(variable, limit);
        if let Some(value) = self
            .assignment_history_prefix_cache
            .get(&cache_key)
            .copied()
        {
            return Some(value);
        }
        let lowered = if limit == 0 {
            self.default_variable_value(self.variable_value_type(variable)?)
        } else {
            let entry = self
                .variable_assignment_history
                .get(&variable)?
                .get(limit - 1)?
                .clone();
            self.consume_history_reconstruction_step()?;
            if !self.guard_history_lowering_stack.insert((variable, limit)) {
                return None;
            }
            let lowered = self.lower_assignment_history_expr(variable, limit, &entry);
            self.guard_history_lowering_stack.remove(&(variable, limit));
            lowered
        };
        if let Some(value) = lowered {
            self.assignment_history_prefix_cache
                .insert(cache_key, value);
        }
        lowered
    }

    fn lower_assignment_history_expr(
        &mut self,
        variable: VariableId,
        limit: usize,
        entry: &AssignmentHistoryEntry,
    ) -> Option<ValueId> {
        if !self.expr_references_variable(entry.expr, variable) {
            return self.lower_expression_with_assignment_snapshot(entry, entry.expr);
        }

        let expression = self.mir.expressions.get(usize::from(entry.expr))?;
        if !matches!(expression.kind, HirExprKind::Conditional { .. }) {
            let previous_value = self.lower_assignment_history_prefix(variable, limit - 1)?;
            return self.lower_assignment_history_expr_from_previous(
                variable,
                entry,
                previous_value,
            );
        }

        if let Some(update) = self.conditional_self_update(variable, entry.expr)
            && let Some(value) =
                self.lower_complementary_self_assignment_pair(variable, limit, entry, update)
        {
            return Some(value);
        }

        let previous_value = self.lower_assignment_history_prefix(variable, limit - 1)?;
        self.lower_assignment_history_expr_from_previous(variable, entry, previous_value)
    }

    fn lower_expression_with_assignment_previous_value(
        &mut self,
        variable: VariableId,
        previous_value: ValueId,
        expr: ExprId,
    ) -> Option<ValueId> {
        self.assignment_history_previous_value_stack
            .push((variable, previous_value));
        let lowered = self.lower_expression(expr);
        self.assignment_history_previous_value_stack.pop();
        lowered
    }

    fn assignment_history_previous_value(&self, variable: VariableId) -> Option<ValueId> {
        self.assignment_history_previous_value_stack
            .iter()
            .rev()
            .find_map(|(candidate, value)| (*candidate == variable).then_some(*value))
    }

    fn assignment_history_value_snapshot(&self, variable: VariableId) -> Option<ValueId> {
        self.assignment_history_value_snapshot_stack
            .iter()
            .rev()
            .find_map(|(candidate, value)| (*candidate == variable).then_some(*value))
    }

    fn assignment_history_snapshot_limit(&self, variable: VariableId) -> Option<usize> {
        self.assignment_history_snapshot_stack
            .iter()
            .rev()
            .find_map(|(candidate, limit)| (*candidate == variable).then_some(*limit))
    }

    fn lower_expression_with_assignment_snapshot(
        &mut self,
        entry: &AssignmentHistoryEntry,
        expr: ExprId,
    ) -> Option<ValueId> {
        self.assignment_history_value_snapshot_stack
            .extend_from_slice(&entry.value_snapshots);
        self.assignment_history_snapshot_stack
            .extend_from_slice(&entry.snapshots);
        let lowered = self.lower_expression(expr);
        let retained_value_snapshots = self
            .assignment_history_value_snapshot_stack
            .len()
            .saturating_sub(entry.value_snapshots.len());
        self.assignment_history_value_snapshot_stack
            .truncate(retained_value_snapshots);
        let retained = self
            .assignment_history_snapshot_stack
            .len()
            .saturating_sub(entry.snapshots.len());
        self.assignment_history_snapshot_stack.truncate(retained);
        lowered
    }

    fn lower_expression_with_assignment_previous_value_and_snapshot(
        &mut self,
        variable: VariableId,
        previous_value: ValueId,
        entry: &AssignmentHistoryEntry,
        expr: ExprId,
    ) -> Option<ValueId> {
        self.assignment_history_value_snapshot_stack
            .extend_from_slice(&entry.value_snapshots);
        self.assignment_history_snapshot_stack
            .extend_from_slice(&entry.snapshots);
        let lowered =
            self.lower_expression_with_assignment_previous_value(variable, previous_value, expr);
        let retained_value_snapshots = self
            .assignment_history_value_snapshot_stack
            .len()
            .saturating_sub(entry.value_snapshots.len());
        self.assignment_history_value_snapshot_stack
            .truncate(retained_value_snapshots);
        let retained = self
            .assignment_history_snapshot_stack
            .len()
            .saturating_sub(entry.snapshots.len());
        self.assignment_history_snapshot_stack.truncate(retained);
        lowered
    }

    fn assignment_history_entry(&self, target: VariableId, expr: ExprId) -> AssignmentHistoryEntry {
        let variables = self.assignment_history_snapshot_variables(target, expr);
        let mut value_snapshots = Vec::new();
        let mut snapshots = Vec::new();
        for variable in variables {
            if let Some(Some(value)) = self.variable_values.get(&variable).copied() {
                value_snapshots.push((variable, value));
            } else if let Some(history) = self.variable_assignment_history.get(&variable) {
                snapshots.push((variable, history.len()));
            }
        }
        value_snapshots.sort_by_key(|(variable, _)| usize::from(*variable));
        snapshots.sort_by_key(|(variable, _)| usize::from(*variable));
        AssignmentHistoryEntry {
            expr,
            value_snapshots,
            snapshots,
        }
    }

    fn assignment_history_snapshot_variables(
        &self,
        target: VariableId,
        expr: ExprId,
    ) -> BTreeSet<VariableId> {
        if let Some(update) = self.conditional_self_update(target, expr)
            && !self.expr_references_variable(update.value_expr, target)
            && let Some(variables) = self.simple_assignment_alias_variables(update.value_expr)
        {
            return variables;
        }
        if let Some(variable) = self.variable_identifier(expr) {
            return (variable != target)
                .then(|| BTreeSet::from([variable]))
                .unwrap_or_default();
        }
        if self.expr_references_variable(expr, target)
            && let Some(variables) = self.self_assignment_alias_snapshot_variables(target, expr)
        {
            return variables;
        }
        self.simple_assignment_alias_snapshot_variables(expr)
            .unwrap_or_default()
    }

    fn self_assignment_alias_snapshot_variables(
        &self,
        target: VariableId,
        expr: ExprId,
    ) -> Option<BTreeSet<VariableId>> {
        let mut variables = self.simple_assignment_alias_variables(expr)?;
        variables.remove(&target);
        self.simple_assignment_alias_snapshot_variables_from_set(variables)
    }

    fn simple_assignment_alias_snapshot_variables(
        &self,
        expr: ExprId,
    ) -> Option<BTreeSet<VariableId>> {
        let variables = self.simple_assignment_alias_variables(expr)?;
        self.simple_assignment_alias_snapshot_variables_from_set(variables)
    }

    fn simple_assignment_alias_snapshot_variables_from_set(
        &self,
        variables: BTreeSet<VariableId>,
    ) -> Option<BTreeSet<VariableId>> {
        let history_backed_unresolved = variables
            .iter()
            .filter(|variable| {
                self.variable_assignment_history.contains_key(variable)
                    && self
                        .variable_values
                        .get(variable)
                        .copied()
                        .flatten()
                        .is_none()
            })
            .count();
        (history_backed_unresolved <= 1).then_some(variables)
    }

    fn simple_assignment_alias_variables(&self, expr: ExprId) -> Option<BTreeSet<VariableId>> {
        let mut variables = BTreeSet::new();
        let mut visited = HashSet::new();
        let mut stack = vec![expr];
        let mut nodes = 0;
        while let Some(expr) = stack.pop() {
            if !visited.insert(expr) {
                continue;
            }
            nodes += 1;
            if nodes > MAX_SCALAR_ASSIGNMENT_ALIAS_SNAPSHOT_EXPR_NODES {
                return None;
            }
            let expression = self.mir.expressions.get(usize::from(expr))?;
            match &expression.kind {
                HirExprKind::Identifier { .. } => {
                    if let Some(variable) = self.variable_identifier(expr)
                        && (self.variable_values.contains_key(&variable)
                            || self.variable_assignment_history.contains_key(&variable))
                    {
                        variables.insert(variable);
                    }
                }
                HirExprKind::Number { .. } => {}
                HirExprKind::Unary { op, operand }
                    if simple_assignment_alias_snapshot_unary_op(op) =>
                {
                    stack.push(*operand);
                }
                HirExprKind::Binary { op, left, right }
                    if simple_assignment_alias_snapshot_binary_op(op) =>
                {
                    stack.extend([*left, *right]);
                }
                _ => return None,
            }
        }
        if variables.len() > MAX_SCALAR_ASSIGNMENT_ALIAS_SNAPSHOT_VARIABLES {
            return None;
        }
        Some(variables)
    }

    fn lower_complementary_assignment_history_value(
        &mut self,
        variable: VariableId,
    ) -> Option<ValueId> {
        let limit = self.variable_assignment_history.get(&variable)?.len();
        let entry = self
            .variable_assignment_history
            .get(&variable)?
            .get(limit.checked_sub(1)?)?
            .clone();
        let update = self.conditional_self_update(variable, entry.expr)?;
        self.lower_complementary_self_assignment_pair(variable, limit, &entry, update)
    }

    fn lower_complementary_self_assignment_pair(
        &mut self,
        variable: VariableId,
        limit: usize,
        current_entry: &AssignmentHistoryEntry,
        current: ConditionalSelfUpdate,
    ) -> Option<ValueId> {
        if limit < 2 {
            return None;
        }
        if self.expr_references_variable(current.value_expr, variable) {
            return None;
        }

        let previous_entry = self
            .variable_assignment_history
            .get(&variable)?
            .get(limit - 2)?
            .clone();
        let previous = self.conditional_self_update(variable, previous_entry.expr)?;
        if self.expr_references_variable(previous.value_expr, variable) {
            return None;
        }

        let previous_truth =
            self.self_update_active_truth_relative_to(previous, previous.condition)?;
        let current_truth =
            self.self_update_active_truth_relative_to(current, previous.condition)?;
        if previous_truth == current_truth {
            return None;
        }

        let previous_value =
            self.lower_expression_with_assignment_snapshot(&previous_entry, previous.value_expr)?;
        let current_value =
            self.lower_expression_with_assignment_snapshot(current_entry, current.value_expr)?;
        let condition =
            self.lower_expression_with_assignment_snapshot(&previous_entry, previous.condition)?;
        let (then_value, else_value) = if previous_truth {
            (previous_value, current_value)
        } else {
            (current_value, previous_value)
        };
        let value_type = self.variable_opt_value_type(variable)?;
        Some(self.push_value(
            value_type,
            OptValueKind::Select {
                condition,
                then_value,
                else_value,
            },
        ))
    }

    fn conditional_self_update(
        &self,
        variable: VariableId,
        expr: ExprId,
    ) -> Option<ConditionalSelfUpdate> {
        let expression = self.mir.expressions.get(usize::from(expr))?;
        let HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } = &expression.kind
        else {
            return None;
        };
        let then_self = self.variable_identifier(*then_expr) == Some(variable);
        let else_self = self.variable_identifier(*else_expr) == Some(variable);
        match (then_self, else_self) {
            (false, true) => Some(ConditionalSelfUpdate {
                condition: *condition,
                active_truth: true,
                value_expr: *then_expr,
            }),
            (true, false) => Some(ConditionalSelfUpdate {
                condition: *condition,
                active_truth: false,
                value_expr: *else_expr,
            }),
            _ => None,
        }
    }

    fn lower_recent_assignment_history_value(&mut self, variable: VariableId) -> Option<ValueId> {
        let cache_key = self.recent_assignment_history_cache_key(variable);
        if let Some(value) = self
            .recent_assignment_history_cache
            .get(&cache_key)
            .copied()
        {
            return Some(value);
        }
        let lowered = self.lower_recent_assignment_history_value_uncached(variable);
        if let Some(value) = lowered {
            self.recent_assignment_history_cache
                .insert(cache_key, value);
        }
        lowered
    }

    fn lower_recent_assignment_history_value_uncached(
        &mut self,
        variable: VariableId,
    ) -> Option<ValueId> {
        let history = self.variable_assignment_history.get(&variable)?.clone();
        self.consume_history_reconstruction_step()?;
        let scan_start = history
            .len()
            .saturating_sub(MAX_SCALAR_RECENT_HISTORY_RECONSTRUCTION_ENTRIES);
        for base_index in (scan_start..history.len()).rev() {
            let base_entry = &history[base_index];
            if self.expr_references_variable(base_entry.expr, variable) {
                continue;
            }
            let Some(mut value) =
                self.lower_expression_with_assignment_snapshot(base_entry, base_entry.expr)
            else {
                continue;
            };
            for entry in history.iter().skip(base_index + 1) {
                let Some(next) =
                    self.lower_assignment_history_expr_from_previous(variable, entry, value)
                else {
                    return None;
                };
                value = next;
            }
            return Some(value);
        }
        None
    }

    fn lower_assignment_history_expr_from_previous(
        &mut self,
        variable: VariableId,
        entry: &AssignmentHistoryEntry,
        previous_value: ValueId,
    ) -> Option<ValueId> {
        self.consume_history_reconstruction_step()?;
        let expression = self.mir.expressions.get(usize::from(entry.expr))?;
        let HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } = &expression.kind
        else {
            return self.lower_expression_with_assignment_previous_value_and_snapshot(
                variable,
                previous_value,
                entry,
                entry.expr,
            );
        };

        let then_value = if self.variable_identifier(*then_expr) == Some(variable) {
            previous_value
        } else {
            self.lower_expression_with_assignment_previous_value_and_snapshot(
                variable,
                previous_value,
                entry,
                *then_expr,
            )?
        };
        let else_value = if self.variable_identifier(*else_expr) == Some(variable) {
            previous_value
        } else {
            self.lower_expression_with_assignment_previous_value_and_snapshot(
                variable,
                previous_value,
                entry,
                *else_expr,
            )?
        };
        if then_value == else_value {
            return Some(then_value);
        }
        let condition = self.lower_expression_with_assignment_previous_value_and_snapshot(
            variable,
            previous_value,
            entry,
            *condition,
        )?;
        let value_type = self.variable_opt_value_type(variable)?;
        Some(self.push_value(
            value_type,
            OptValueKind::Select {
                condition,
                then_value,
                else_value,
            },
        ))
    }

    fn self_update_active_truth_relative_to(
        &self,
        update: ConditionalSelfUpdate,
        reference_condition: ExprId,
    ) -> Option<bool> {
        if self.expr_structurally_equal(update.condition, reference_condition) {
            Some(update.active_truth)
        } else if self.expr_is_logical_complement(update.condition, reference_condition) {
            Some(!update.active_truth)
        } else {
            None
        }
    }

    fn expr_references_variable(&self, expr: ExprId, variable: VariableId) -> bool {
        self.expr_references_any_variable(expr, &HashSet::from([variable]))
    }

    fn lower_conditional_path_variable_identifier(
        &mut self,
        variable: VariableId,
    ) -> Option<ValueId> {
        if self.conditional_path_stack.is_empty() || !self.variable_lowering_stack.insert(variable)
        {
            return None;
        }
        let guarded_expr = self.guarded_path_assignment_exprs.get(&variable).cloned();
        let lowered = guarded_expr
            .and_then(|expr| self.lower_guarded_path_assignment_for_current_path(variable, expr))
            .or_else(|| self.lower_assignment_history_for_current_path(variable));
        self.variable_lowering_stack.remove(&variable);
        lowered
    }

    fn lower_assignment_history_for_current_path(
        &mut self,
        variable: VariableId,
    ) -> Option<ValueId> {
        let cache_key = self.current_path_history_cache_key(variable);
        if let Some(value) = self.current_path_history_cache.get(&cache_key).copied() {
            return Some(value);
        }
        let history = self.variable_assignment_history.get(&variable)?.clone();
        self.consume_history_reconstruction_step()?;
        let direct = history
            .iter()
            .rev()
            .take(MAX_SCALAR_CURRENT_PATH_HISTORY_SCAN_ENTRIES)
            .find_map(|entry| self.lower_assignment_expr_for_current_path(variable, entry));
        let lowered = direct.or_else(|| {
            self.lower_complementary_assignment_history_pair_for_current_path(variable, &history)
        });
        if let Some(value) = lowered {
            self.current_path_history_cache.insert(cache_key, value);
        }
        lowered
    }

    fn consume_history_reconstruction_step(&mut self) -> Option<()> {
        if self.history_reconstruction_steps >= MAX_SCALAR_HISTORY_RECONSTRUCTION_STEPS {
            return None;
        }
        self.history_reconstruction_steps += 1;
        Some(())
    }

    fn reset_history_reconstruction_budget(&mut self) {
        self.history_reconstruction_steps = 0;
    }

    fn assignment_history_prefix_cache_key(
        &self,
        variable: VariableId,
        limit: usize,
    ) -> AssignmentHistoryPrefixCacheKey {
        AssignmentHistoryPrefixCacheKey {
            variable,
            limit,
            branch_flow_context: self.branch_flow_context,
            guard_alias_lowering_depth: self.guard_alias_lowering_depth,
            path: self.conditional_path_stack.clone(),
            previous_values: self.assignment_history_previous_value_stack.clone(),
        }
    }

    fn current_path_history_cache_key(&self, variable: VariableId) -> CurrentPathHistoryCacheKey {
        CurrentPathHistoryCacheKey {
            variable,
            branch_flow_context: self.branch_flow_context,
            guard_alias_lowering_depth: self.guard_alias_lowering_depth,
            path: self.conditional_path_stack.clone(),
            previous_values: self.assignment_history_previous_value_stack.clone(),
        }
    }

    fn recent_assignment_history_cache_key(
        &self,
        variable: VariableId,
    ) -> RecentAssignmentHistoryCacheKey {
        RecentAssignmentHistoryCacheKey {
            variable,
            branch_flow_context: self.branch_flow_context,
            guard_alias_lowering_depth: self.guard_alias_lowering_depth,
            path: self.conditional_path_stack.clone(),
            previous_values: self.assignment_history_previous_value_stack.clone(),
        }
    }

    fn clear_assignment_replay_caches(&mut self) {
        self.assignment_history_prefix_cache.clear();
        self.current_path_history_cache.clear();
        self.recent_assignment_history_cache.clear();
    }

    fn lower_guarded_path_assignment_for_current_path(
        &mut self,
        variable: VariableId,
        assignment: GuardedPathAssignmentExpr,
    ) -> Option<ValueId> {
        self.condition_truth_in_current_path(assignment.condition)
            .is_some_and(|truth| truth)
            .then_some(())?;
        self.lower_assignment_expr_for_current_path(variable, &assignment.value)
    }

    fn lower_assignment_expr_for_current_path(
        &mut self,
        variable: VariableId,
        entry: &AssignmentHistoryEntry,
    ) -> Option<ValueId> {
        let expression = self.mir.expressions.get(usize::from(entry.expr))?;
        let HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } = &expression.kind
        else {
            if self.expr_references_variable(entry.expr, variable) {
                return None;
            }
            return self.lower_expression_with_assignment_snapshot(entry, entry.expr);
        };
        let condition_truth = self.condition_truth_in_current_path(*condition)?;
        let selected = if condition_truth {
            *then_expr
        } else {
            *else_expr
        };
        if self.variable_identifier(selected) == Some(variable) {
            return None;
        }
        self.lower_expression_with_assignment_snapshot(entry, selected)
    }

    fn lower_complementary_assignment_history_pair_for_current_path(
        &mut self,
        variable: VariableId,
        history: &[AssignmentHistoryEntry],
    ) -> Option<ValueId> {
        let scan_start = history
            .len()
            .saturating_sub(MAX_SCALAR_CURRENT_PATH_HISTORY_SCAN_ENTRIES);
        for current_index in (scan_start.saturating_add(1)..history.len()).rev() {
            let previous_entry = &history[current_index - 1];
            let current_entry = &history[current_index];
            let Some(previous) = self.conditional_self_update(variable, previous_entry.expr) else {
                continue;
            };
            let Some(current) = self.conditional_self_update(variable, current_entry.expr) else {
                continue;
            };
            if self.expr_references_variable(previous.value_expr, variable)
                || self.expr_references_variable(current.value_expr, variable)
            {
                continue;
            }
            let previous_condition = self.self_update_relative_to_current_path(previous)?;
            let current_condition = self.self_update_relative_to_current_path(current)?;
            if !self.relative_conditions_are_complements(previous_condition, current_condition) {
                continue;
            }

            self.conditional_path_stack.push(ConditionalPathPredicate {
                condition: previous_condition.condition,
                truth: previous_condition.truth,
            });
            let previous_value =
                self.lower_expression_with_assignment_snapshot(previous_entry, previous.value_expr);
            self.conditional_path_stack.pop();
            let previous_value = previous_value?;

            self.conditional_path_stack.push(ConditionalPathPredicate {
                condition: current_condition.condition,
                truth: current_condition.truth,
            });
            let current_value =
                self.lower_expression_with_assignment_snapshot(current_entry, current.value_expr);
            self.conditional_path_stack.pop();
            let current_value = current_value?;

            let condition = self.lower_expression_with_assignment_snapshot(
                previous_entry,
                previous_condition.condition,
            )?;
            let (then_value, else_value) = if previous_condition.truth {
                (previous_value, current_value)
            } else {
                (current_value, previous_value)
            };
            let value_type = self.variable_opt_value_type(variable)?;
            return Some(self.push_value(
                value_type,
                OptValueKind::Select {
                    condition,
                    then_value,
                    else_value,
                },
            ));
        }
        None
    }

    fn self_update_relative_to_current_path(
        &self,
        update: ConditionalSelfUpdate,
    ) -> Option<RelativeCondition> {
        let mut condition = self.relative_condition_in_current_path(update.condition)?;
        if !update.active_truth {
            condition.truth = !condition.truth;
        }
        Some(condition)
    }

    fn relative_conditions_are_complements(
        &self,
        left: RelativeCondition,
        right: RelativeCondition,
    ) -> bool {
        if self.expr_structurally_equal(left.condition, right.condition) {
            return left.truth != right.truth;
        }
        if self.expr_is_logical_complement(left.condition, right.condition) {
            return left.truth == right.truth;
        }
        false
    }

    fn relative_condition_in_current_path(&self, condition: ExprId) -> Option<RelativeCondition> {
        let mut active = HashSet::new();
        self.relative_condition_in_current_path_inner(condition, 0, &mut active)
    }

    fn relative_condition_in_current_path_inner(
        &self,
        condition: ExprId,
        depth: usize,
        active: &mut HashSet<ExprId>,
    ) -> Option<RelativeCondition> {
        if depth > MAX_SCALAR_CONDITION_REASONING_DEPTH || !active.insert(condition) {
            return None;
        }

        let result = (|| {
            if let Some(true) = self.condition_truth_in_current_path(condition) {
                return None;
            }
            if let Some(alias) = self.guard_alias_expr(condition) {
                return self.relative_condition_in_current_path_inner(alias, depth + 1, active);
            }
            let expression = self.mir.expressions.get(usize::from(condition))?;
            match &expression.kind {
                HirExprKind::Unary { op, operand } if op.as_str() == "Not" => {
                    let mut relative =
                        self.relative_condition_in_current_path_inner(*operand, depth + 1, active)?;
                    relative.truth = !relative.truth;
                    Some(relative)
                }
                HirExprKind::Binary { op, left, right } if op.as_str() == "And" => {
                    match (
                        self.condition_truth_in_current_path(*left),
                        self.condition_truth_in_current_path(*right),
                    ) {
                        (Some(false), _) | (_, Some(false)) => None,
                        (Some(true), Some(true)) => None,
                        (Some(true), _) => {
                            self.relative_condition_in_current_path_inner(*right, depth + 1, active)
                        }
                        (_, Some(true)) => {
                            self.relative_condition_in_current_path_inner(*left, depth + 1, active)
                        }
                        _ => Some(RelativeCondition {
                            condition,
                            truth: true,
                        }),
                    }
                }
                _ => Some(RelativeCondition {
                    condition,
                    truth: true,
                }),
            }
        })();
        active.remove(&condition);
        result
    }

    fn condition_truth_in_current_path(&self, condition: ExprId) -> Option<bool> {
        let mut active = HashSet::new();
        self.condition_truth_in_current_path_inner(condition, 0, &mut active)
    }

    fn condition_truth_in_current_path_inner(
        &self,
        condition: ExprId,
        depth: usize,
        active: &mut HashSet<ExprId>,
    ) -> Option<bool> {
        if depth > MAX_SCALAR_CONDITION_REASONING_DEPTH || !active.insert(condition) {
            return None;
        }

        let result = (|| {
            for predicate in self.conditional_path_stack.iter().rev() {
                if let Some(truth) = self.condition_truth_from_predicate(condition, *predicate) {
                    return Some(truth);
                }
            }

            if let Some(alias) = self.guard_alias_expr(condition) {
                return self.condition_truth_in_current_path_inner(alias, depth + 1, active);
            }

            let expression = self.mir.expressions.get(usize::from(condition))?;
            match &expression.kind {
                HirExprKind::Unary { op, operand } if op.as_str() == "Not" => self
                    .condition_truth_in_current_path_inner(*operand, depth + 1, active)
                    .map(|truth| !truth),
                HirExprKind::Binary { op, left, right } if op.as_str() == "And" => {
                    let left_truth =
                        self.condition_truth_in_current_path_inner(*left, depth + 1, active);
                    let right_truth =
                        self.condition_truth_in_current_path_inner(*right, depth + 1, active);
                    match (left_truth, right_truth) {
                        (Some(false), _) | (_, Some(false)) => Some(false),
                        (Some(true), Some(true)) => Some(true),
                        _ => None,
                    }
                }
                HirExprKind::Binary { op, left, right } if op.as_str() == "Or" => {
                    let left_truth =
                        self.condition_truth_in_current_path_inner(*left, depth + 1, active);
                    let right_truth =
                        self.condition_truth_in_current_path_inner(*right, depth + 1, active);
                    match (left_truth, right_truth) {
                        (Some(true), _) | (_, Some(true)) => Some(true),
                        (Some(false), Some(false)) => Some(false),
                        _ => None,
                    }
                }
                _ => None,
            }
        })();
        active.remove(&condition);
        result
    }

    fn condition_truth_from_predicate(
        &self,
        condition: ExprId,
        predicate: ConditionalPathPredicate,
    ) -> Option<bool> {
        if self.expr_structurally_equal(condition, predicate.condition) {
            return Some(predicate.truth);
        }
        if self.expr_is_logical_not(condition, predicate.condition)
            || self.expr_is_logical_not(predicate.condition, condition)
        {
            return Some(!predicate.truth);
        }
        if self.expr_is_logical_complement(condition, predicate.condition) {
            return Some(!predicate.truth);
        }
        if predicate.truth && self.expr_conjunctively_contains(predicate.condition, condition) {
            return Some(true);
        }
        None
    }

    fn expr_conjunctively_contains(&self, container: ExprId, target: ExprId) -> bool {
        let mut active = HashSet::new();
        self.expr_conjunctively_contains_inner(container, target, 0, &mut active)
    }

    fn expr_conjunctively_contains_inner(
        &self,
        container: ExprId,
        target: ExprId,
        depth: usize,
        active: &mut HashSet<ExprId>,
    ) -> bool {
        if depth > MAX_SCALAR_CONDITION_REASONING_DEPTH || !active.insert(container) {
            return false;
        }

        let result = (|| {
            if self.expr_structurally_equal(container, target) {
                return true;
            }
            if let Some(alias) = self.guard_alias_expr(container) {
                return self.expr_conjunctively_contains_inner(alias, target, depth + 1, active);
            }
            let Some(expression) = self.mir.expressions.get(usize::from(container)) else {
                return false;
            };
            match &expression.kind {
                HirExprKind::Binary { op, left, right } if op.as_str() == "And" => {
                    self.expr_conjunctively_contains_inner(*left, target, depth + 1, active)
                        || self.expr_conjunctively_contains_inner(*right, target, depth + 1, active)
                }
                _ => false,
            }
        })();
        active.remove(&container);
        result
    }

    fn expr_is_logical_not(&self, expr: ExprId, inner: ExprId) -> bool {
        let mut active = HashSet::new();
        self.expr_is_logical_not_inner(expr, inner, 0, &mut active)
    }

    fn expr_is_logical_not_inner(
        &self,
        expr: ExprId,
        inner: ExprId,
        depth: usize,
        active: &mut HashSet<ExprId>,
    ) -> bool {
        if depth > MAX_SCALAR_CONDITION_REASONING_DEPTH || !active.insert(expr) {
            return false;
        }

        let result = (|| {
            if let Some(alias) = self.guard_alias_expr(expr) {
                return self.expr_is_logical_not_inner(alias, inner, depth + 1, active);
            }
            let Some(expression) = self.mir.expressions.get(usize::from(expr)) else {
                return false;
            };
            matches!(
                &expression.kind,
                HirExprKind::Unary { op, operand }
                    if op.as_str() == "Not" && self.expr_structurally_equal(*operand, inner)
            )
        })();
        active.remove(&expr);
        result
    }

    fn expr_is_logical_complement(&self, left: ExprId, right: ExprId) -> bool {
        let mut active = HashSet::new();
        self.expr_is_logical_complement_inner(left, right, 0, &mut active)
    }

    fn expr_is_logical_complement_inner(
        &self,
        left: ExprId,
        right: ExprId,
        depth: usize,
        active: &mut HashSet<(ExprId, ExprId)>,
    ) -> bool {
        if depth > MAX_SCALAR_CONDITION_REASONING_DEPTH || !active.insert((left, right)) {
            return false;
        }

        let result = (|| {
            if let Some(alias) = self.guard_alias_expr(left) {
                return self.expr_is_logical_complement_inner(alias, right, depth + 1, active);
            }
            if let Some(alias) = self.guard_alias_expr(right) {
                return self.expr_is_logical_complement_inner(left, alias, depth + 1, active);
            }
            self.expr_is_logical_not(left, right)
                || self.expr_is_logical_not(right, left)
                || self.binary_conditions_are_complements(left, right)
        })();
        active.remove(&(left, right));
        result
    }

    fn binary_conditions_are_complements(&self, left: ExprId, right: ExprId) -> bool {
        let Some(left_expr) = self.mir.expressions.get(usize::from(left)) else {
            return false;
        };
        let Some(right_expr) = self.mir.expressions.get(usize::from(right)) else {
            return false;
        };
        let HirExprKind::Binary {
            op: left_op,
            left: left_left,
            right: left_right,
        } = &left_expr.kind
        else {
            return false;
        };
        let HirExprKind::Binary {
            op: right_op,
            left: right_left,
            right: right_right,
        } = &right_expr.kind
        else {
            return false;
        };

        let complements = matches!(
            (left_op.as_str(), right_op.as_str()),
            ("Eq", "Ne") | ("Ne", "Eq") | ("Lt", "Ge") | ("Ge", "Lt") | ("Le", "Gt") | ("Gt", "Le")
        );
        if left_op.as_str() == "Eq"
            && right_op.as_str() == "Eq"
            && self.binary_switch_equalities_are_complements(
                *left_left,
                *left_right,
                *right_left,
                *right_right,
            )
        {
            return true;
        }
        if !complements {
            return false;
        }

        if self.expr_structurally_equal(*left_left, *right_left)
            && self.expr_structurally_equal(*left_right, *right_right)
        {
            return true;
        }

        matches!(
            (left_op.as_str(), right_op.as_str()),
            ("Eq", "Ne") | ("Ne", "Eq")
        ) && self.expr_structurally_equal(*left_left, *right_right)
            && self.expr_structurally_equal(*left_right, *right_left)
    }

    fn binary_switch_equalities_are_complements(
        &self,
        left_left: ExprId,
        left_right: ExprId,
        right_left: ExprId,
        right_right: ExprId,
    ) -> bool {
        let Some((left_parameter, left_value)) =
            self.binary_switch_equality_operand(left_left, left_right)
        else {
            return false;
        };
        let Some((right_parameter, right_value)) =
            self.binary_switch_equality_operand(right_left, right_right)
        else {
            return false;
        };
        left_parameter == right_parameter
            && ((left_value == 0.0 && right_value == 1.0)
                || (left_value == 1.0 && right_value == 0.0))
    }

    fn binary_switch_equality_operand(
        &self,
        left: ExprId,
        right: ExprId,
    ) -> Option<(ParamId, f64)> {
        if let Some(parameter) = self.binary_switch_parameter_expr(left)
            && let Some(value) = self.number_constant_expr(right)
        {
            return Some((parameter, value));
        }
        if let Some(parameter) = self.binary_switch_parameter_expr(right)
            && let Some(value) = self.number_constant_expr(left)
        {
            return Some((parameter, value));
        }
        None
    }

    fn binary_switch_parameter_expr(&self, expr: ExprId) -> Option<ParamId> {
        let expression = self.mir.expressions.get(usize::from(expr))?;
        let HirExprKind::Identifier { name } = &expression.kind else {
            return None;
        };
        self.mir
            .parameters
            .iter()
            .find(|parameter| {
                parameter.name == *name || parameter.aliases.iter().any(|alias| alias == name)
            })
            .filter(|parameter| {
                parameter.value_type == CanonicalValueType::Integer
                    && parameter.range.as_ref().is_some_and(binary_switch_range)
            })
            .map(|parameter| parameter.id)
    }

    fn expr_structurally_equal(&self, left: ExprId, right: ExprId) -> bool {
        let mut active = HashSet::new();
        self.expr_structurally_equal_inner(left, right, 0, &mut active)
    }

    fn expr_structurally_equal_inner(
        &self,
        left: ExprId,
        right: ExprId,
        depth: usize,
        active: &mut HashSet<(ExprId, ExprId)>,
    ) -> bool {
        if left == right {
            return true;
        }
        if depth > MAX_SCALAR_STRUCTURAL_EQUALITY_DEPTH || !active.insert((left, right)) {
            return false;
        }

        let result = (|| {
            if let Some(alias) = self.guard_alias_expr(left)
                && self.expr_structurally_equal_inner(alias, right, depth + 1, active)
            {
                return true;
            }
            if let Some(alias) = self.guard_alias_expr(right)
                && self.expr_structurally_equal_inner(left, alias, depth + 1, active)
            {
                return true;
            }

            let Some(left_expr) = self.mir.expressions.get(usize::from(left)) else {
                return false;
            };
            let Some(right_expr) = self.mir.expressions.get(usize::from(right)) else {
                return false;
            };
            match (&left_expr.kind, &right_expr.kind) {
                (
                    HirExprKind::Number { value: left, .. },
                    HirExprKind::Number { value: right, .. },
                ) => left.to_bits() == right.to_bits(),
                (
                    HirExprKind::StringLiteral { value: left },
                    HirExprKind::StringLiteral { value: right },
                )
                | (
                    HirExprKind::Identifier { name: left },
                    HirExprKind::Identifier { name: right },
                ) => left == right,
                (
                    HirExprKind::SystemFunction {
                        name: left_name,
                        args: left_args,
                    },
                    HirExprKind::SystemFunction {
                        name: right_name,
                        args: right_args,
                    },
                )
                | (
                    HirExprKind::Call {
                        name: left_name,
                        args: left_args,
                    },
                    HirExprKind::Call {
                        name: right_name,
                        args: right_args,
                    },
                ) => {
                    left_name == right_name
                        && left_args.len() == right_args.len()
                        && left_args.iter().zip(right_args).all(|(left, right)| {
                            self.expr_structurally_equal_inner(*left, *right, depth + 1, active)
                        })
                }
                (
                    HirExprKind::Binary {
                        op: left_op,
                        left: left_left,
                        right: left_right,
                    },
                    HirExprKind::Binary {
                        op: right_op,
                        left: right_left,
                        right: right_right,
                    },
                ) => {
                    left_op == right_op
                        && self.expr_structurally_equal_inner(
                            *left_left,
                            *right_left,
                            depth + 1,
                            active,
                        )
                        && self.expr_structurally_equal_inner(
                            *left_right,
                            *right_right,
                            depth + 1,
                            active,
                        )
                }
                (
                    HirExprKind::Unary {
                        op: left_op,
                        operand: left_operand,
                    },
                    HirExprKind::Unary {
                        op: right_op,
                        operand: right_operand,
                    },
                ) => {
                    left_op == right_op
                        && self.expr_structurally_equal_inner(
                            *left_operand,
                            *right_operand,
                            depth + 1,
                            active,
                        )
                }
                (
                    HirExprKind::Conditional {
                        condition: left_condition,
                        then_expr: left_then,
                        else_expr: left_else,
                    },
                    HirExprKind::Conditional {
                        condition: right_condition,
                        then_expr: right_then,
                        else_expr: right_else,
                    },
                ) => {
                    self.expr_structurally_equal_inner(
                        *left_condition,
                        *right_condition,
                        depth + 1,
                        active,
                    ) && self.expr_structurally_equal_inner(
                        *left_then,
                        *right_then,
                        depth + 1,
                        active,
                    ) && self.expr_structurally_equal_inner(
                        *left_else,
                        *right_else,
                        depth + 1,
                        active,
                    )
                }
                (
                    HirExprKind::BranchAccess {
                        access: left_access,
                        pos: left_pos,
                        neg: left_neg,
                    },
                    HirExprKind::BranchAccess {
                        access: right_access,
                        pos: right_pos,
                        neg: right_neg,
                    },
                ) => left_access == right_access && left_pos == right_pos && left_neg == right_neg,
                (
                    HirExprKind::NamedBranchAccess {
                        access: left_access,
                        name: left_name,
                    },
                    HirExprKind::NamedBranchAccess {
                        access: right_access,
                        name: right_name,
                    },
                ) => left_access == right_access && left_name == right_name,
                (
                    HirExprKind::ArrayAccess {
                        array: left_array,
                        index: left_index,
                    },
                    HirExprKind::ArrayAccess {
                        array: right_array,
                        index: right_index,
                    },
                ) => {
                    left_array == right_array
                        && self.expr_structurally_equal_inner(
                            *left_index,
                            *right_index,
                            depth + 1,
                            active,
                        )
                }
                (
                    HirExprKind::ArrayLiteral {
                        elements: left_elements,
                    },
                    HirExprKind::ArrayLiteral {
                        elements: right_elements,
                    },
                ) => {
                    left_elements.len() == right_elements.len()
                        && left_elements
                            .iter()
                            .zip(right_elements)
                            .all(|(left, right)| {
                                self.expr_structurally_equal_inner(*left, *right, depth + 1, active)
                            })
                }
                (
                    HirExprKind::AnalogOperator { op: left_op },
                    HirExprKind::AnalogOperator { op: right_op },
                ) => left_op == right_op,
                (
                    HirExprKind::Laplace {
                        expr: left_expr,
                        kind: left_kind,
                    },
                    HirExprKind::Laplace {
                        expr: right_expr,
                        kind: right_kind,
                    },
                ) => {
                    left_kind == right_kind
                        && self.expr_structurally_equal_inner(
                            *left_expr,
                            *right_expr,
                            depth + 1,
                            active,
                        )
                }
                (
                    HirExprKind::Zi {
                        expr: left_expr,
                        kind: left_kind,
                    },
                    HirExprKind::Zi {
                        expr: right_expr,
                        kind: right_kind,
                    },
                ) => {
                    left_kind == right_kind
                        && self.expr_structurally_equal_inner(
                            *left_expr,
                            *right_expr,
                            depth + 1,
                            active,
                        )
                }
                (
                    HirExprKind::NoiseSource {
                        source: left_source,
                        operands: left_operands,
                        name: left_name,
                    },
                    HirExprKind::NoiseSource {
                        source: right_source,
                        operands: right_operands,
                        name: right_name,
                    },
                ) => {
                    left_source == right_source
                        && left_name == right_name
                        && left_operands.len() == right_operands.len()
                        && left_operands
                            .iter()
                            .zip(right_operands)
                            .all(|(left, right)| {
                                self.expr_structurally_equal_inner(*left, *right, depth + 1, active)
                            })
                }
                _ => false,
            }
        })();
        active.remove(&(left, right));
        result
    }

    fn guard_alias_expr(&self, expr: ExprId) -> Option<ExprId> {
        let expression = self.mir.expressions.get(usize::from(expr))?;
        let HirExprKind::Identifier { name } = &expression.kind else {
            return None;
        };
        if !name.starts_with("__guard") {
            return None;
        }
        let variable = self
            .hir?
            .variables
            .iter()
            .find(|variable| variable.name == *name)?;
        self.variable_assignment_exprs
            .get(&variable.id)
            .copied()
            .filter(|alias| *alias != expr)
    }

    fn lower_contextual_variable_identifier(&mut self, variable: VariableId) -> Option<ValueId> {
        self.branch_flow_context?;
        if !self.variable_lowering_stack.insert(variable) {
            return None;
        }
        let expr = self.variable_assignment_exprs.get(&variable).copied();
        let lowered = expr.and_then(|expr| self.lower_expression(expr));
        self.variable_lowering_stack.remove(&variable);
        lowered
    }

    fn default_variable_value(&mut self, value_type: CanonicalValueType) -> Option<ValueId> {
        match value_type {
            CanonicalValueType::Boolean => {
                Some(self.push_value(OptValueType::Boolean, OptValueKind::BooleanConstant(false)))
            }
            CanonicalValueType::Real
            | CanonicalValueType::Integer
            | CanonicalValueType::NatureAccess => {
                Some(self.push_value(OptValueType::Real, OptValueKind::RealConstant(0.0)))
            }
            _ => None,
        }
    }

    fn lower_branch_access(
        &mut self,
        access: &SmolStr,
        pos: &SmolStr,
        neg: Option<&str>,
    ) -> Option<ValueId> {
        if is_flow_access_name(access.as_str()) {
            return self.lower_direct_branch_flow(pos, neg);
        }

        let pos = self.resolve_endpoint(pos)?;
        let neg = match neg {
            Some(neg) => self.resolve_endpoint(neg)?,
            None => None,
        };

        Some(self.lower_voltage(pos, neg))
    }

    fn lower_direct_branch_flow(&mut self, pos: &SmolStr, neg: Option<&str>) -> Option<ValueId> {
        let pos = self.resolve_endpoint(pos)?;
        let neg = match neg {
            Some(neg) => self.resolve_endpoint(neg)?,
            None => None,
        };
        let (branch_unknown, reversed) = self
            .context_branch_unknown_by_nodes(pos, neg)
            .or_else(|| self.branch_unknown_by_nodes(pos, neg))?;
        let flow = self.push_value(
            OptValueType::Real,
            OptValueKind::BranchUnknownFlow { branch_unknown },
        );
        if reversed {
            Some(self.push_value(
                OptValueType::Real,
                OptValueKind::Unary {
                    op: OptUnaryOp::Neg,
                    input: flow,
                },
            ))
        } else {
            Some(flow)
        }
    }

    fn context_branch_unknown_by_nodes(
        &self,
        pos_node: Option<NodeId>,
        neg_node: Option<NodeId>,
    ) -> Option<(BranchUnknownId, bool)> {
        let branch_unknown = self.branch_flow_context?;
        let unknown = self.mir.branch_unknowns.get(usize::from(branch_unknown))?;
        if unknown.pos_node == pos_node && unknown.neg_node == neg_node {
            return Some((unknown.id, false));
        }
        if unknown.pos_node == neg_node && unknown.neg_node == pos_node {
            return Some((unknown.id, true));
        }
        None
    }

    fn lower_named_branch_access(&mut self, access: &SmolStr, name: &SmolStr) -> Option<ValueId> {
        if is_flow_access_name(access.as_str()) {
            if let Some(current) = self.branch_current_values.get(name.as_str()).copied() {
                return Some(current);
            }
            let branch = self.branch_id_by_name(name)?;
            self.branch_unknown_for_branch(branch)?;
            Some(self.push_value(OptValueType::Real, OptValueKind::BranchFlow { branch }))
        } else {
            let (pos, neg) = self
                .mir
                .branches
                .iter()
                .find(|branch| branch.name.as_str() == name)
                .map(|branch| (branch.pos_node, branch.neg_node))?;
            Some(self.lower_voltage(pos, neg))
        }
    }

    fn lower_voltage(&mut self, pos: Option<NodeId>, neg: Option<NodeId>) -> ValueId {
        match (pos, neg) {
            (None, None) => self.push_value(OptValueType::Real, OptValueKind::RealConstant(0.0)),
            (Some(pos), None) => self.push_node_potential(pos),
            (None, Some(neg)) => {
                let zero = self.push_value(OptValueType::Real, OptValueKind::RealConstant(0.0));
                let neg = self.push_node_potential(neg);
                self.push_value(
                    OptValueType::Real,
                    OptValueKind::Binary {
                        op: OptBinaryOp::Sub,
                        left: zero,
                        right: neg,
                    },
                )
            }
            (Some(pos), Some(neg)) => {
                let pos = self.push_node_potential(pos);
                let neg = self.push_node_potential(neg);
                self.push_value(
                    OptValueType::Real,
                    OptValueKind::Binary {
                        op: OptBinaryOp::Sub,
                        left: pos,
                        right: neg,
                    },
                )
            }
        }
    }

    fn push_node_potential(&mut self, node: NodeId) -> ValueId {
        self.push_value(OptValueType::Real, OptValueKind::NodePotential { node })
    }

    fn cache_declared_branch_current(&mut self, equation: &MirEquation, value: Option<ValueId>) {
        if equation.kind != MirEquationKind::Current {
            return;
        }
        let Some(value) = value else {
            return;
        };
        let Some(branch_name) = self.declared_contribution_branch_name(equation) else {
            return;
        };

        let value = if let Some(previous) = self.branch_current_values.get(branch_name.as_str()) {
            self.push_value(
                OptValueType::Real,
                OptValueKind::Binary {
                    op: OptBinaryOp::Add,
                    left: *previous,
                    right: value,
                },
            )
        } else {
            value
        };
        self.branch_current_values.insert(branch_name, value);
    }

    fn declared_contribution_branch_name(&self, equation: &MirEquation) -> Option<String> {
        if let Some(name) = equation.branch.declared_name.as_deref() {
            return Some(name.to_string());
        }
        if self
            .mir
            .branches
            .iter()
            .any(|branch| branch.name.as_str() == equation.branch.label.as_str())
        {
            return Some(equation.branch.label.to_string());
        }

        let mut matches = self.mir.branches.iter().filter(|branch| {
            branch.pos_node == equation.branch.pos_node
                && branch.neg_node == equation.branch.neg_node
        });
        let first = matches.next()?;
        if matches.next().is_none() {
            Some(first.name.to_string())
        } else {
            None
        }
    }

    fn branch_id_by_name(&self, name: &str) -> Option<BranchId> {
        self.mir
            .branches
            .iter()
            .find(|branch| branch.name.as_str() == name)
            .map(|branch| branch.id)
    }

    fn branch_unknown_by_nodes(
        &self,
        pos_node: Option<NodeId>,
        neg_node: Option<NodeId>,
    ) -> Option<(BranchUnknownId, bool)> {
        let mut matches = self
            .mir
            .branch_unknowns
            .iter()
            .filter(|unknown| unknown.pos_node == pos_node && unknown.neg_node == neg_node);
        if let Some(unknown) = matches.next()
            && matches.next().is_none()
        {
            return Some((unknown.id, false));
        }

        let mut reversed_matches = self
            .mir
            .branch_unknowns
            .iter()
            .filter(|unknown| unknown.pos_node == neg_node && unknown.neg_node == pos_node);
        let unknown = reversed_matches.next()?;
        if reversed_matches.next().is_some() {
            return None;
        }
        Some((unknown.id, true))
    }

    fn branch_unknown_for_branch(&self, branch_id: BranchId) -> Option<BranchUnknownId> {
        let branch = self.mir.branches.get(usize::from(branch_id))?;
        if let Some(unknown) = self.mir.branch_unknowns.iter().find(|unknown| {
            unknown
                .declared_name
                .as_deref()
                .is_some_and(|name| name == branch.name.as_str())
        }) {
            return Some(unknown.id);
        }

        let mut matches = self.mir.branch_unknowns.iter().filter(|unknown| {
            unknown.pos_node == branch.pos_node && unknown.neg_node == branch.neg_node
        });
        let first = matches.next()?;
        if matches.next().is_none() {
            Some(first.id)
        } else {
            None
        }
    }

    fn resolve_endpoint(&self, name: &str) -> Option<Option<NodeId>> {
        if name == "0"
            || self
                .mir
                .ground_nodes
                .iter()
                .any(|ground| ground.as_str() == name)
        {
            return Some(None);
        }

        self.mir
            .nodes
            .iter()
            .find(|node| node.name.as_str() == name)
            .map(|node| Some(node.id))
    }

    fn lower_binary(&mut self, op: &SmolStr, left: ExprId, right: ExprId) -> Option<ValueId> {
        if op.as_str() == "Mod" {
            return self.lower_static_mod(left, right);
        }
        let op = binary_op(op)?;
        let left = self.lower_expression(left)?;
        let right = self.lower_expression(right)?;
        let value_type = match op {
            OptBinaryOp::Eq
            | OptBinaryOp::Ne
            | OptBinaryOp::Lt
            | OptBinaryOp::Le
            | OptBinaryOp::Gt
            | OptBinaryOp::Ge
            | OptBinaryOp::And
            | OptBinaryOp::Or => OptValueType::Boolean,
            OptBinaryOp::Add
            | OptBinaryOp::Sub
            | OptBinaryOp::Mul
            | OptBinaryOp::Div
            | OptBinaryOp::Pow => OptValueType::Real,
        };

        Some(self.push_value(value_type, OptValueKind::Binary { op, left, right }))
    }

    fn lower_static_mod(&mut self, left: ExprId, right: ExprId) -> Option<ValueId> {
        let left = self.lower_expression(left)?;
        let right = self.lower_expression(right)?;
        let left = self.real_constant(left)?.trunc();
        let right = self.real_constant(right)?.trunc();
        if right == 0.0 {
            return None;
        }
        Some(self.push_value(OptValueType::Real, OptValueKind::RealConstant(left % right)))
    }

    fn lower_unary(&mut self, op: &SmolStr, operand: ExprId) -> Option<ValueId> {
        let op = unary_op(op)?;
        self.lower_intrinsic_unary(op, operand)
    }

    fn lower_intrinsic_unary(&mut self, op: OptUnaryOp, input: ExprId) -> Option<ValueId> {
        let input = self.lower_expression(input)?;
        let value_type = if op == OptUnaryOp::Not {
            OptValueType::Boolean
        } else {
            OptValueType::Real
        };

        Some(self.push_value(value_type, OptValueKind::Unary { op, input }))
    }

    fn lower_conditional(
        &mut self,
        condition: ExprId,
        then_expr: ExprId,
        else_expr: ExprId,
    ) -> Option<ValueId> {
        let condition_expr = condition;
        let condition = self.lower_expression(condition_expr)?;
        self.conditional_path_stack.push(ConditionalPathPredicate {
            condition: condition_expr,
            truth: true,
        });
        let then_value = self.lower_expression(then_expr);
        self.conditional_path_stack.pop();
        let then_value = then_value?;

        self.conditional_path_stack.push(ConditionalPathPredicate {
            condition: condition_expr,
            truth: false,
        });
        let else_value = self.lower_expression(else_expr);
        self.conditional_path_stack.pop();
        let else_value = else_value?;

        let value_type = self.values[usize::from(then_value)].value_type;

        Some(self.push_value(
            value_type,
            OptValueKind::Select {
                condition,
                then_value,
                else_value,
            },
        ))
    }

    fn lower_call(&mut self, operator: ExprId, name: &SmolStr, args: &[ExprId]) -> Option<ValueId> {
        if is_noise_name(name.as_str()) {
            return Some(self.zero_real());
        }
        if name.eq_ignore_ascii_case("ddt") {
            let [expr] = args else {
                return None;
            };
            return self.lower_ddt_expression(*expr, operator);
        }
        if self.in_potential_equation_state_operator_context()
            && name.eq_ignore_ascii_case("idt")
            && (1..=2).contains(&args.len())
        {
            return self.lower_expression(args[0]);
        }
        if name.eq_ignore_ascii_case("analysis") {
            return self.lower_analysis_call(args);
        }
        if name.eq_ignore_ascii_case("ddx") {
            return self.lower_ddx_call(args);
        }
        if name.eq_ignore_ascii_case("expm1") {
            let [arg] = args else {
                return None;
            };
            return self.lower_expm1_call(*arg);
        }
        if name.eq_ignore_ascii_case("log1p") {
            let [arg] = args else {
                return None;
            };
            return self.lower_log1p_call(*arg);
        }
        if args.len() == 2 {
            if name.eq_ignore_ascii_case("fpow") {
                return self.lower_fpow_call(args[0], args[1]);
            }
            return self.lower_binary_intrinsic_call(name, args[0], args[1]);
        }
        if args.len() == 1 {
            let op = match name.as_str() {
                "exp" => OptUnaryOp::Exp,
                "limexp" => OptUnaryOp::LimExp,
                "__rspice_limited_exp" => OptUnaryOp::LimitedExp,
                "ln" | "log" => OptUnaryOp::Ln,
                "sqrt" => OptUnaryOp::Sqrt,
                "abs" => OptUnaryOp::Abs,
                "sin" => OptUnaryOp::Sin,
                "cos" => OptUnaryOp::Cos,
                "tan" => OptUnaryOp::Tan,
                "sinh" => OptUnaryOp::Sinh,
                "cosh" => OptUnaryOp::Cosh,
                "tanh" => OptUnaryOp::Tanh,
                "atan" => OptUnaryOp::Atan,
                "asinh" => OptUnaryOp::Asinh,
                "floor" => OptUnaryOp::Floor,
                "ceil" => OptUnaryOp::Ceil,
                _ => return None,
            };
            return self.lower_intrinsic_unary(op, args[0]);
        }

        None
    }

    fn lower_expm1_call(&mut self, arg: ExprId) -> Option<ValueId> {
        let input = self.lower_expression(arg)?;
        let exp = self.push_value(
            OptValueType::Real,
            OptValueKind::Unary {
                op: OptUnaryOp::Exp,
                input,
            },
        );
        let one = self.push_value(OptValueType::Real, OptValueKind::RealConstant(1.0));
        Some(self.push_binary_value(OptBinaryOp::Sub, exp, one))
    }

    fn lower_log1p_call(&mut self, arg: ExprId) -> Option<ValueId> {
        let input = self.lower_expression(arg)?;
        let one = self.push_value(OptValueType::Real, OptValueKind::RealConstant(1.0));
        let sum = self.push_binary_value(OptBinaryOp::Add, one, input);
        Some(self.push_value(
            OptValueType::Real,
            OptValueKind::Unary {
                op: OptUnaryOp::Ln,
                input: sum,
            },
        ))
    }

    fn lower_fpow_call(&mut self, left: ExprId, right: ExprId) -> Option<ValueId> {
        let left = self.lower_expression(left)?;
        let right = self.lower_expression(right)?;
        let pow = self.push_binary_value(OptBinaryOp::Pow, left, right);
        let zero = self.zero_real();
        let one = self.push_value(OptValueType::Real, OptValueKind::RealConstant(1.0));
        let left_is_zero = self.push_binary_value(OptBinaryOp::Eq, left, zero);
        let right_is_zero = self.push_binary_value(OptBinaryOp::Eq, right, zero);
        let both_zero = self.push_binary_value(OptBinaryOp::And, left_is_zero, right_is_zero);
        Some(self.push_value(
            OptValueType::Real,
            OptValueKind::Select {
                condition: both_zero,
                then_value: one,
                else_value: pow,
            },
        ))
    }

    fn zero_real(&mut self) -> ValueId {
        self.push_value(OptValueType::Real, OptValueKind::RealConstant(0.0))
    }

    fn in_potential_equation_state_operator_context(&self) -> bool {
        self.potential_equation_state_operator_depth > 0
    }

    fn ddt_scale(&mut self) -> ValueId {
        self.push_value(OptValueType::Real, OptValueKind::DdtScale)
    }

    fn lower_analysis_call(&mut self, args: &[ExprId]) -> Option<ValueId> {
        let [query] = args else {
            return None;
        };
        let query = self.analysis_query(*query)?;
        Some(self.push_value(OptValueType::Real, OptValueKind::Analysis { query }))
    }

    fn lower_ddx_call(&mut self, args: &[ExprId]) -> Option<ValueId> {
        let [expr, probe] = args else {
            return None;
        };
        self.lower_ddx_expression(*expr, *probe)
    }

    fn lower_ddx_expression(&mut self, expr: ExprId, probe: ExprId) -> Option<ValueId> {
        let value = self.lower_expression(expr)?;
        let (pos_node, neg_node) = self.ddx_probe_nodes(probe)?;
        Some(self.push_value(
            OptValueType::Real,
            OptValueKind::Ddx {
                value,
                pos_node,
                neg_node,
            },
        ))
    }

    fn lower_ddt_expression(&mut self, expr: ExprId, operator: ExprId) -> Option<ValueId> {
        let input = self.lower_expression(expr)?;
        Some(self.push_value(OptValueType::Real, OptValueKind::Ddt { operator, input }))
    }

    fn ddx_probe_nodes(&self, probe: ExprId) -> Option<(Option<NodeId>, Option<NodeId>)> {
        let expression = self.mir.expressions.get(usize::from(probe))?;
        match &expression.kind {
            HirExprKind::BranchAccess { access, pos, neg }
                if !is_flow_access_name(access.as_str()) =>
            {
                let pos = self.resolve_endpoint(pos)?;
                let neg = match neg {
                    Some(neg) => self.resolve_endpoint(neg)?,
                    None => None,
                };
                Some((pos, neg))
            }
            HirExprKind::NamedBranchAccess { access, name }
                if !is_flow_access_name(access.as_str()) =>
            {
                self.mir
                    .branches
                    .iter()
                    .find(|branch| branch.name.as_str() == name.as_str())
                    .map(|branch| (branch.pos_node, branch.neg_node))
            }
            _ => None,
        }
    }

    fn lower_binary_intrinsic_call(
        &mut self,
        name: &SmolStr,
        left: ExprId,
        right: ExprId,
    ) -> Option<ValueId> {
        let op = match name.as_str() {
            "min" => OptBinaryOp::Lt,
            "max" => OptBinaryOp::Gt,
            "pow" => {
                let left = self.lower_expression(left)?;
                let right = self.lower_expression(right)?;
                return Some(self.push_binary_value(OptBinaryOp::Pow, left, right));
            }
            _ => return None,
        };
        let left = self.lower_expression(left)?;
        let right = self.lower_expression(right)?;
        let condition = self.push_value(
            OptValueType::Boolean,
            OptValueKind::Binary { op, left, right },
        );
        Some(self.push_value(
            OptValueType::Real,
            OptValueKind::Select {
                condition,
                then_value: left,
                else_value: right,
            },
        ))
    }

    fn lower_system_function(&mut self, name: &SmolStr, args: &[ExprId]) -> Option<ValueId> {
        match name.to_ascii_lowercase().as_str() {
            "$temperature" if args.is_empty() => {
                Some(self.push_value(OptValueType::Real, OptValueKind::Temperature))
            }
            "$abstime" | "$realtime" if args.is_empty() => {
                Some(self.push_value(OptValueType::Real, OptValueKind::Time))
            }
            "$mfactor" if args.is_empty() => {
                Some(self.push_value(OptValueType::Real, OptValueKind::Multiplicity))
            }
            "$vt" | "$thermal_vt" if args.is_empty() => {
                Some(self.push_value(OptValueType::Real, OptValueKind::ThermalVoltage))
            }
            "$vt" | "$thermal_vt" if args.len() == 1 => {
                let temperature = self.lower_expression(args[0])?;
                let scale = self.push_value(
                    OptValueType::Real,
                    OptValueKind::RealConstant(THERMAL_VOLTAGE_PER_K),
                );
                Some(self.push_binary_value(OptBinaryOp::Mul, temperature, scale))
            }
            "$simparam" if args.len() == 1 => {
                let default = self.simparam_default(args[0])?;
                Some(self.push_value(OptValueType::Real, OptValueKind::RealConstant(default)))
            }
            "$simparam" if args.len() == 2 => self.lower_expression(args[1]),
            "$param_given" if args.len() == 1 => {
                let parameter = self.parameter_arg(args[0])?;
                Some(self.push_value(OptValueType::Real, OptValueKind::ParamGiven { parameter }))
            }
            "$port_connected" if args.len() == 1 => {
                Some(self.push_value(OptValueType::Real, OptValueKind::RealConstant(1.0)))
            }
            _ => None,
        }
    }

    fn parameter_arg(&self, expr: ExprId) -> Option<ParamId> {
        let expression = self.mir.expressions.get(usize::from(expr))?;
        let HirExprKind::Identifier { name } = &expression.kind else {
            return None;
        };
        self.mir
            .parameters
            .iter()
            .find(|parameter| parameter.name == *name)
            .map(|parameter| parameter.id)
    }

    fn simparam_default(&self, expr: ExprId) -> Option<f64> {
        let expression = self.mir.expressions.get(usize::from(expr))?;
        let HirExprKind::StringLiteral { value } = &expression.kind else {
            return None;
        };
        match value.to_ascii_lowercase().as_str() {
            "gmin" => Some(1.0e-12),
            "tnom" => Some(27.0),
            "scale" => Some(1.0),
            "temp" | "temperature" => Some(27.0),
            _ => None,
        }
    }

    fn analysis_query(&self, expr: ExprId) -> Option<SmolStr> {
        let expression = self.mir.expressions.get(usize::from(expr))?;
        let HirExprKind::StringLiteral { value } = &expression.kind else {
            return None;
        };
        normalize_analysis_query(value.as_str()).map(SmolStr::new)
    }
}

fn normalize_analysis_query(name: &str) -> Option<&'static str> {
    match name.to_ascii_lowercase().as_str() {
        "dc" | "op" => Some("dc"),
        "ac" => Some("ac"),
        "tran" | "transient" => Some("tran"),
        "noise" => Some("noise"),
        "ic" => Some("ic"),
        "static" => Some("static"),
        "smallsig" | "smallsignal" | "small_signal" => Some("smallsig"),
        _ => None,
    }
}

fn binary_op(op: &str) -> Option<OptBinaryOp> {
    match op {
        "Add" => Some(OptBinaryOp::Add),
        "Sub" => Some(OptBinaryOp::Sub),
        "Mul" => Some(OptBinaryOp::Mul),
        "Div" => Some(OptBinaryOp::Div),
        "Pow" => Some(OptBinaryOp::Pow),
        "Eq" => Some(OptBinaryOp::Eq),
        "Ne" => Some(OptBinaryOp::Ne),
        "Lt" => Some(OptBinaryOp::Lt),
        "Le" => Some(OptBinaryOp::Le),
        "Gt" => Some(OptBinaryOp::Gt),
        "Ge" => Some(OptBinaryOp::Ge),
        "And" => Some(OptBinaryOp::And),
        "Or" => Some(OptBinaryOp::Or),
        _ => None,
    }
}

fn simple_assignment_alias_snapshot_binary_op(op: &str) -> bool {
    matches!(op, "Add" | "Sub" | "Mul" | "Div" | "Pow")
}

fn unary_op(op: &str) -> Option<OptUnaryOp> {
    match op {
        "Pos" => Some(OptUnaryOp::Pos),
        "Neg" => Some(OptUnaryOp::Neg),
        "Not" => Some(OptUnaryOp::Not),
        _ => None,
    }
}

fn simple_assignment_alias_snapshot_unary_op(op: &str) -> bool {
    matches!(op, "Pos" | "Neg")
}

fn is_flow_access_name(access: &str) -> bool {
    matches!(access, "I" | "Pwr" | "F" | "Tau" | "Phi" | "Flow")
}

pub(crate) fn limexp_value(value: f64) -> f64 {
    if value < 80.0 {
        value.exp()
    } else {
        LIMEXP_MAX * (1.0 + (value - 80.0))
    }
}

pub(crate) fn limexp_derivative(value: f64) -> f64 {
    if value < 80.0 {
        value.exp()
    } else {
        LIMEXP_MAX
    }
}

pub(crate) fn limited_exp_value(value: f64) -> f64 {
    if value > 80.0 {
        LIMEXP_MAX * (1.0 + value - 80.0)
    } else if value < -80.0 {
        1.804851387e-35
    } else {
        value.exp()
    }
}

pub(crate) fn limited_exp_derivative(value: f64) -> f64 {
    if value > 80.0 {
        LIMEXP_MAX
    } else if value < -80.0 {
        0.0
    } else {
        value.exp()
    }
}

pub(crate) fn real_truth_value(value: f64) -> bool {
    value != 0.0
}

fn binary_switch_range(range: &HirParamRange) -> bool {
    range.min == Some(0.0)
        && range.max == Some(1.0)
        && !range.min_exclusive
        && !range.max_exclusive
        && range.exclude.is_empty()
}

fn supported_assignment_value_type(value_type: CanonicalValueType) -> bool {
    matches!(
        value_type,
        CanonicalValueType::Real
            | CanonicalValueType::Integer
            | CanonicalValueType::Boolean
            | CanonicalValueType::NatureAccess
    )
}

fn is_noise_name(name: &str) -> bool {
    matches!(
        name.trim_start_matches('$'),
        "white_noise" | "flicker_noise" | "noise_table" | "noise_table_log"
    )
}

fn remap_value_id(value: ValueId, remap: &[Option<ValueId>]) -> ValueId {
    remap
        .get(usize::from(value))
        .and_then(|value| *value)
        .expect("live OptIR value must have a compacted id")
}

fn remap_value_kind(kind: &OptValueKind, remap: &[Option<ValueId>]) -> OptValueKind {
    match kind {
        OptValueKind::RealConstant(value) => OptValueKind::RealConstant(*value),
        OptValueKind::BooleanConstant(value) => OptValueKind::BooleanConstant(*value),
        OptValueKind::Parameter { parameter } => OptValueKind::Parameter {
            parameter: *parameter,
        },
        OptValueKind::ParamGiven { parameter } => OptValueKind::ParamGiven {
            parameter: *parameter,
        },
        OptValueKind::Temperature => OptValueKind::Temperature,
        OptValueKind::ThermalVoltage => OptValueKind::ThermalVoltage,
        OptValueKind::Multiplicity => OptValueKind::Multiplicity,
        OptValueKind::Time => OptValueKind::Time,
        OptValueKind::Analysis { query } => OptValueKind::Analysis {
            query: query.clone(),
        },
        OptValueKind::Ddx {
            value,
            pos_node,
            neg_node,
        } => OptValueKind::Ddx {
            value: remap_value_id(*value, remap),
            pos_node: *pos_node,
            neg_node: *neg_node,
        },
        OptValueKind::Ddt { operator, input } => OptValueKind::Ddt {
            operator: *operator,
            input: remap_value_id(*input, remap),
        },
        OptValueKind::DdtScale => OptValueKind::DdtScale,
        OptValueKind::NodePotential { node } => OptValueKind::NodePotential { node: *node },
        OptValueKind::BranchFlow { branch } => OptValueKind::BranchFlow { branch: *branch },
        OptValueKind::BranchUnknownFlow { branch_unknown } => OptValueKind::BranchUnknownFlow {
            branch_unknown: *branch_unknown,
        },
        OptValueKind::LoopIndex { loop_id } => OptValueKind::LoopIndex { loop_id: *loop_id },
        OptValueKind::CountedSum {
            loop_id,
            count,
            initial,
            term,
        } => OptValueKind::CountedSum {
            loop_id: *loop_id,
            count: remap_value_id(*count, remap),
            initial: remap_value_id(*initial, remap),
            term: remap_value_id(*term, remap),
        },
        OptValueKind::RuntimeLoopVariable { loop_id, slot } => OptValueKind::RuntimeLoopVariable {
            loop_id: *loop_id,
            slot: *slot,
        },
        OptValueKind::RuntimeLoopVariableDerivative {
            loop_id,
            slot,
            lane,
        } => OptValueKind::RuntimeLoopVariableDerivative {
            loop_id: *loop_id,
            slot: *slot,
            lane: *lane,
        },
        OptValueKind::RuntimeLoopResult { loop_id, slot } => OptValueKind::RuntimeLoopResult {
            loop_id: *loop_id,
            slot: *slot,
        },
        OptValueKind::RuntimeLoopResultDerivative {
            loop_id,
            slot,
            lane,
        } => OptValueKind::RuntimeLoopResultDerivative {
            loop_id: *loop_id,
            slot: *slot,
            lane: *lane,
        },
        OptValueKind::Unary { op, input } => OptValueKind::Unary {
            op: *op,
            input: remap_value_id(*input, remap),
        },
        OptValueKind::Binary { op, left, right } => OptValueKind::Binary {
            op: *op,
            left: remap_value_id(*left, remap),
            right: remap_value_id(*right, remap),
        },
        OptValueKind::Select {
            condition,
            then_value,
            else_value,
        } => OptValueKind::Select {
            condition: remap_value_id(*condition, remap),
            then_value: remap_value_id(*then_value, remap),
            else_value: remap_value_id(*else_value, remap),
        },
        OptValueKind::EquationValue { equation } => OptValueKind::EquationValue {
            equation: *equation,
        },
    }
}

fn remap_runtime_loop(runtime_loop: &OptRuntimeLoop, remap: &[Option<ValueId>]) -> OptRuntimeLoop {
    OptRuntimeLoop {
        loop_id: runtime_loop.loop_id,
        variables: runtime_loop
            .variables
            .iter()
            .map(|variable| OptRuntimeLoopVariable {
                source: variable.source,
                value_type: variable.value_type,
                initial: remap_value_id(variable.initial, remap),
                variable: remap_value_id(variable.variable, remap),
                result: remap_value_id(variable.result, remap),
            })
            .collect(),
        condition: remap_value_id(runtime_loop.condition, remap),
        assignments: runtime_loop
            .assignments
            .iter()
            .map(|assignment| OptRuntimeLoopAssignment {
                slot: assignment.slot,
                value: remap_value_id(assignment.value, remap),
            })
            .collect(),
    }
}

fn validate_values(diagnostics: &mut Vec<IrDiagnostic>, opt: &OptModel) {
    for value in &opt.values {
        validate_value_kind(diagnostics, opt, value);
        validate_derivatives(diagnostics, opt, value);
    }
}

fn validate_value_kind(diagnostics: &mut Vec<IrDiagnostic>, opt: &OptModel, value: &OptValue) {
    match &value.kind {
        OptValueKind::RealConstant(_) | OptValueKind::BooleanConstant(_) => {}
        OptValueKind::Parameter { parameter } | OptValueKind::ParamGiven { parameter } => {
            if parameter.index() >= opt.parameter_count {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::OptValidation,
                    format!(
                        "OptIR value {} parameter {} is out of range for {} parameters",
                        value.id, parameter, opt.parameter_count
                    ),
                ));
            }
        }
        OptValueKind::Temperature
        | OptValueKind::ThermalVoltage
        | OptValueKind::Multiplicity
        | OptValueKind::Time
        | OptValueKind::Analysis { .. }
        | OptValueKind::LoopIndex { .. }
        | OptValueKind::RuntimeLoopVariable { .. }
        | OptValueKind::RuntimeLoopVariableDerivative { .. }
        | OptValueKind::RuntimeLoopResult { .. }
        | OptValueKind::RuntimeLoopResultDerivative { .. } => {}
        OptValueKind::Ddx {
            value: input,
            pos_node,
            neg_node,
        } => {
            validate_value_operand(
                diagnostics,
                opt.values.len(),
                value.id,
                *input,
                "ddx operand",
            );
            for node in [*pos_node, *neg_node].into_iter().flatten() {
                if node.index() >= opt.node_count {
                    diagnostics.push(IrDiagnostic::global_error(
                        CompilerPhase::OptValidation,
                        format!(
                            "OptIR value {} ddx node {} is out of range for {} nodes",
                            value.id, node, opt.node_count
                        ),
                    ));
                }
            }
        }
        OptValueKind::Ddt { input, .. } => {
            validate_value_operand(
                diagnostics,
                opt.values.len(),
                value.id,
                *input,
                "ddt operand",
            );
        }
        OptValueKind::DdtScale => {}
        OptValueKind::NodePotential { node } => {
            if node.index() >= opt.node_count {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::OptValidation,
                    format!(
                        "OptIR value {} node {} is out of range for {} nodes",
                        value.id, node, opt.node_count
                    ),
                ));
            }
        }
        OptValueKind::BranchFlow { branch } => {
            if branch.index() >= opt.branch_count {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::OptValidation,
                    format!(
                        "OptIR value {} branch {} is out of range for {} branches",
                        value.id, branch, opt.branch_count
                    ),
                ));
            }
        }
        OptValueKind::BranchUnknownFlow { branch_unknown } => {
            if branch_unknown.index() >= opt.branch_unknown_count {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::OptValidation,
                    format!(
                        "OptIR value {} branch unknown {} is out of range for {} branch unknowns",
                        value.id, branch_unknown, opt.branch_unknown_count
                    ),
                ));
            }
        }
        OptValueKind::Unary { input, .. } => {
            validate_value_operand(diagnostics, opt.values.len(), value.id, *input, "operand");
        }
        OptValueKind::Binary { left, right, .. } => {
            validate_value_operand(
                diagnostics,
                opt.values.len(),
                value.id,
                *left,
                "left operand",
            );
            validate_value_operand(
                diagnostics,
                opt.values.len(),
                value.id,
                *right,
                "right operand",
            );
        }
        OptValueKind::Select {
            condition,
            then_value,
            else_value,
        } => {
            validate_value_operand(
                diagnostics,
                opt.values.len(),
                value.id,
                *condition,
                "condition operand",
            );
            validate_value_operand(
                diagnostics,
                opt.values.len(),
                value.id,
                *then_value,
                "then operand",
            );
            validate_value_operand(
                diagnostics,
                opt.values.len(),
                value.id,
                *else_value,
                "else operand",
            );
        }
        OptValueKind::CountedSum {
            count,
            initial,
            term,
            ..
        } => {
            validate_value_operand(
                diagnostics,
                opt.values.len(),
                value.id,
                *count,
                "count operand",
            );
            validate_value_operand(
                diagnostics,
                opt.values.len(),
                value.id,
                *initial,
                "initial operand",
            );
            validate_value_operand(
                diagnostics,
                opt.values.len(),
                value.id,
                *term,
                "term operand",
            );
        }
        OptValueKind::EquationValue { equation } => {
            if equation.index() >= opt.equation_count {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::OptValidation,
                    format!(
                        "OptIR value {} equation {} is out of range for {} equations",
                        value.id, equation, opt.equation_count
                    ),
                ));
            }
        }
    }
}

fn validate_runtime_loops(diagnostics: &mut Vec<IrDiagnostic>, opt: &OptModel) {
    let mut loop_ids = HashSet::new();
    for runtime_loop in &opt.runtime_loops {
        if !loop_ids.insert(runtime_loop.loop_id) {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::OptValidation,
                format!("OptIR duplicate runtime loop id {}", runtime_loop.loop_id),
            ));
        }
        if runtime_loop.variables.is_empty() {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::OptValidation,
                format!(
                    "OptIR runtime loop {} must have at least one variable",
                    runtime_loop.loop_id
                ),
            ));
        }
        validate_runtime_loop_value_reference(
            diagnostics,
            opt.values.len(),
            runtime_loop.loop_id,
            runtime_loop.condition,
            "condition",
        );
        for (slot, variable) in runtime_loop.variables.iter().enumerate() {
            let expected_slot =
                u32::try_from(slot).expect("OptIR runtime loop slot exceeds u32::MAX");
            validate_runtime_loop_value_reference(
                diagnostics,
                opt.values.len(),
                runtime_loop.loop_id,
                variable.initial,
                "initial",
            );
            validate_runtime_loop_value_reference(
                diagnostics,
                opt.values.len(),
                runtime_loop.loop_id,
                variable.variable,
                "variable",
            );
            validate_runtime_loop_value_reference(
                diagnostics,
                opt.values.len(),
                runtime_loop.loop_id,
                variable.result,
                "result",
            );
            validate_runtime_loop_slot_value(
                diagnostics,
                opt,
                runtime_loop.loop_id,
                expected_slot,
                variable.variable,
                "variable",
            );
            validate_runtime_loop_slot_value(
                diagnostics,
                opt,
                runtime_loop.loop_id,
                expected_slot,
                variable.result,
                "result",
            );
        }
        for assignment in &runtime_loop.assignments {
            if usize::try_from(assignment.slot)
                .ok()
                .is_none_or(|slot| slot >= runtime_loop.variables.len())
            {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::OptValidation,
                    format!(
                        "OptIR runtime loop {} assignment slot {} is out of range for {} variables",
                        runtime_loop.loop_id,
                        assignment.slot,
                        runtime_loop.variables.len()
                    ),
                ));
            }
            validate_runtime_loop_value_reference(
                diagnostics,
                opt.values.len(),
                runtime_loop.loop_id,
                assignment.value,
                "assignment value",
            );
        }
    }
}

fn validate_runtime_loop_value_reference(
    diagnostics: &mut Vec<IrDiagnostic>,
    value_count: usize,
    loop_id: u32,
    value: ValueId,
    label: &str,
) {
    if usize::from(value) >= value_count {
        diagnostics.push(IrDiagnostic::global_error(
            CompilerPhase::OptValidation,
            format!(
                "OptIR runtime loop {loop_id} {label} value {value} is out of range for {value_count} values"
            ),
        ));
    }
}

fn validate_runtime_loop_slot_value(
    diagnostics: &mut Vec<IrDiagnostic>,
    opt: &OptModel,
    loop_id: u32,
    expected_slot: u32,
    value: ValueId,
    label: &str,
) {
    let Some(value) = opt.values.get(usize::from(value)) else {
        return;
    };
    let valid = match value.kind {
        OptValueKind::RuntimeLoopVariable {
            loop_id: actual_loop,
            slot,
        }
        | OptValueKind::RuntimeLoopResult {
            loop_id: actual_loop,
            slot,
        } => actual_loop == loop_id && slot == expected_slot,
        _ => false,
    };
    if !valid {
        diagnostics.push(IrDiagnostic::global_error(
            CompilerPhase::OptValidation,
            format!(
                "OptIR runtime loop {loop_id} {label} value {} must reference slot {expected_slot}",
                value.id
            ),
        ));
    }
}

fn validate_value_operand(
    diagnostics: &mut Vec<IrDiagnostic>,
    value_count: usize,
    owner: ValueId,
    operand: ValueId,
    label: &str,
) {
    if usize::from(operand) >= value_count {
        diagnostics.push(IrDiagnostic::global_error(
            CompilerPhase::OptValidation,
            format!(
                "OptIR value {} {} {} is out of range for {} values",
                owner, label, operand, value_count
            ),
        ));
        return;
    }

    if operand.index() >= owner.index() {
        diagnostics.push(IrDiagnostic::global_error(
            CompilerPhase::OptValidation,
            format!(
                "OptIR value {} {} {} violates scalar value topological order",
                owner, label, operand
            ),
        ));
    }
}

fn validate_derivatives(diagnostics: &mut Vec<IrDiagnostic>, opt: &OptModel, value: &OptValue) {
    let mut previous_lane = None;
    let mut lanes = HashSet::new();

    for derivative in &value.derivatives {
        validate_derivative_lane(diagnostics, opt, value.id, derivative.lane);
        validate_value_reference(
            diagnostics,
            opt.values.len(),
            value.id,
            derivative.value,
            "derivative value",
        );

        if let Some(previous_lane) = previous_lane
            && previous_lane > derivative.lane
        {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::OptValidation,
                format!(
                    "OptIR value {} derivative lanes must be sorted by lane",
                    value.id
                ),
            ));
        }
        previous_lane = Some(derivative.lane);

        if !lanes.insert(derivative.lane) {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::OptValidation,
                format!(
                    "OptIR value {} has duplicate derivative lane {:?}",
                    value.id, derivative.lane
                ),
            ));
        }
    }
}

fn validate_value_reference(
    diagnostics: &mut Vec<IrDiagnostic>,
    value_count: usize,
    owner: ValueId,
    reference: ValueId,
    label: &str,
) {
    if usize::from(reference) >= value_count {
        diagnostics.push(IrDiagnostic::global_error(
            CompilerPhase::OptValidation,
            format!(
                "OptIR value {} {} {} is out of range for {} values",
                owner, label, reference, value_count
            ),
        ));
    }
}

fn validate_derivative_lane(
    diagnostics: &mut Vec<IrDiagnostic>,
    opt: &OptModel,
    owner: ValueId,
    lane: DerivativeLane,
) {
    let limit = match lane.kind {
        DerivativeLaneKind::Node => opt.node_count,
        DerivativeLaneKind::BranchUnknown => opt.branch_unknown_count,
    };

    if lane.index >= limit {
        diagnostics.push(IrDiagnostic::global_error(
            CompilerPhase::OptValidation,
            format!(
                "OptIR value {} derivative lane {:?} is out of range for limit {}",
                owner, lane, limit
            ),
        ));
    }
}

fn validate_dense_value_ids(diagnostics: &mut Vec<IrDiagnostic>, values: &[OptValue]) {
    for (expected, value) in values.iter().enumerate() {
        let expected = u32::try_from(expected).expect("OptIR value count exceeds u32::MAX");
        if value.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::OptValidation,
                format!(
                    "OptIR value IDs must be dense: expected ValueId({}) at index {}, found {}",
                    expected, expected, value.id
                ),
            ));
        }
    }
}

fn collect_static_ops(values: &[OptValue], target: InvalidationClass) -> Vec<OptOp> {
    let mut invalidation_memo = vec![None; values.len()];
    let mut parameter_memo = vec![None; values.len()];
    let mut temperature_memo = vec![None; values.len()];
    let mut loop_index_memo = vec![None; values.len()];
    values
        .iter()
        .filter(|value| {
            if value_invalidation(values, value.id, &mut invalidation_memo) != target {
                return false;
            }
            if !matches!(value.kind, OptValueKind::CountedSum { .. })
                && value_depends_on_loop_index(values, value.id, &mut loop_index_memo)
            {
                return false;
            }
            match target {
                InvalidationClass::InstanceStatic => {
                    value_depends_on_parameter(values, value.id, &mut parameter_memo)
                }
                InvalidationClass::TemperatureStatic => {
                    !matches!(
                        value.kind,
                        OptValueKind::Temperature | OptValueKind::ThermalVoltage
                    ) && value_depends_on_temperature(values, value.id, &mut temperature_memo)
                }
                _ => false,
            }
        })
        .map(|value| OptOp::ComputeValue { value: value.id })
        .collect()
}

fn value_invalidation(
    values: &[OptValue],
    value: ValueId,
    memo: &mut [Option<InvalidationClass>],
) -> InvalidationClass {
    let index = usize::from(value);
    if let Some(invalidation) = memo[index] {
        return invalidation;
    }

    let invalidation = match values[index].kind {
        OptValueKind::RealConstant(_)
        | OptValueKind::BooleanConstant(_)
        | OptValueKind::Parameter { .. }
        | OptValueKind::ParamGiven { .. }
        | OptValueKind::LoopIndex { .. } => InvalidationClass::InstanceStatic,
        OptValueKind::Temperature | OptValueKind::ThermalVoltage => {
            InvalidationClass::TemperatureStatic
        }
        OptValueKind::Multiplicity
        | OptValueKind::Time
        | OptValueKind::Analysis { .. }
        | OptValueKind::Ddx { .. }
        | OptValueKind::Ddt { .. }
        | OptValueKind::DdtScale
        | OptValueKind::RuntimeLoopVariable { .. }
        | OptValueKind::RuntimeLoopVariableDerivative { .. }
        | OptValueKind::RuntimeLoopResult { .. }
        | OptValueKind::RuntimeLoopResultDerivative { .. } => InvalidationClass::NewtonIteration,
        OptValueKind::NodePotential { .. }
        | OptValueKind::BranchFlow { .. }
        | OptValueKind::BranchUnknownFlow { .. }
        | OptValueKind::EquationValue { .. } => InvalidationClass::NewtonIteration,
        OptValueKind::Unary { input, .. } => value_invalidation(values, input, memo),
        OptValueKind::Binary { left, right, .. } => max_invalidation(
            value_invalidation(values, left, memo),
            value_invalidation(values, right, memo),
        ),
        OptValueKind::Select {
            condition,
            then_value,
            else_value,
        } => max_invalidation(
            value_invalidation(values, condition, memo),
            max_invalidation(
                value_invalidation(values, then_value, memo),
                value_invalidation(values, else_value, memo),
            ),
        ),
        OptValueKind::CountedSum {
            count,
            initial,
            term,
            ..
        } => max_invalidation(
            value_invalidation(values, count, memo),
            max_invalidation(
                value_invalidation(values, initial, memo),
                value_invalidation(values, term, memo),
            ),
        ),
    };
    memo[index] = Some(invalidation);
    invalidation
}

fn value_depends_on_parameter(
    values: &[OptValue],
    value: ValueId,
    memo: &mut [Option<bool>],
) -> bool {
    let index = usize::from(value);
    if let Some(depends) = memo[index] {
        return depends;
    }

    let depends = match values[index].kind {
        OptValueKind::Parameter { .. } | OptValueKind::ParamGiven { .. } => true,
        OptValueKind::RealConstant(_)
        | OptValueKind::BooleanConstant(_)
        | OptValueKind::Temperature
        | OptValueKind::ThermalVoltage
        | OptValueKind::Multiplicity
        | OptValueKind::Time
        | OptValueKind::Analysis { .. }
        | OptValueKind::Ddx { .. }
        | OptValueKind::Ddt { .. }
        | OptValueKind::DdtScale
        | OptValueKind::NodePotential { .. }
        | OptValueKind::BranchFlow { .. }
        | OptValueKind::BranchUnknownFlow { .. }
        | OptValueKind::LoopIndex { .. }
        | OptValueKind::RuntimeLoopVariable { .. }
        | OptValueKind::RuntimeLoopVariableDerivative { .. }
        | OptValueKind::RuntimeLoopResult { .. }
        | OptValueKind::RuntimeLoopResultDerivative { .. }
        | OptValueKind::EquationValue { .. } => false,
        OptValueKind::Unary { input, .. } => value_depends_on_parameter(values, input, memo),
        OptValueKind::Binary { left, right, .. } => {
            value_depends_on_parameter(values, left, memo)
                || value_depends_on_parameter(values, right, memo)
        }
        OptValueKind::Select {
            condition,
            then_value,
            else_value,
        } => {
            value_depends_on_parameter(values, condition, memo)
                || value_depends_on_parameter(values, then_value, memo)
                || value_depends_on_parameter(values, else_value, memo)
        }
        OptValueKind::CountedSum {
            count,
            initial,
            term,
            ..
        } => {
            value_depends_on_parameter(values, count, memo)
                || value_depends_on_parameter(values, initial, memo)
                || value_depends_on_parameter(values, term, memo)
        }
    };
    memo[index] = Some(depends);
    depends
}

fn value_depends_on_temperature(
    values: &[OptValue],
    value: ValueId,
    memo: &mut [Option<bool>],
) -> bool {
    let index = usize::from(value);
    if let Some(depends) = memo[index] {
        return depends;
    }

    let depends = match values[index].kind {
        OptValueKind::Temperature | OptValueKind::ThermalVoltage => true,
        OptValueKind::RealConstant(_)
        | OptValueKind::BooleanConstant(_)
        | OptValueKind::Parameter { .. }
        | OptValueKind::ParamGiven { .. }
        | OptValueKind::Multiplicity
        | OptValueKind::Time
        | OptValueKind::Analysis { .. }
        | OptValueKind::Ddx { .. }
        | OptValueKind::Ddt { .. }
        | OptValueKind::DdtScale
        | OptValueKind::NodePotential { .. }
        | OptValueKind::BranchFlow { .. }
        | OptValueKind::BranchUnknownFlow { .. }
        | OptValueKind::LoopIndex { .. }
        | OptValueKind::RuntimeLoopVariable { .. }
        | OptValueKind::RuntimeLoopVariableDerivative { .. }
        | OptValueKind::RuntimeLoopResult { .. }
        | OptValueKind::RuntimeLoopResultDerivative { .. }
        | OptValueKind::EquationValue { .. } => false,
        OptValueKind::Unary { input, .. } => value_depends_on_temperature(values, input, memo),
        OptValueKind::Binary { left, right, .. } => {
            value_depends_on_temperature(values, left, memo)
                || value_depends_on_temperature(values, right, memo)
        }
        OptValueKind::Select {
            condition,
            then_value,
            else_value,
        } => {
            value_depends_on_temperature(values, condition, memo)
                || value_depends_on_temperature(values, then_value, memo)
                || value_depends_on_temperature(values, else_value, memo)
        }
        OptValueKind::CountedSum {
            count,
            initial,
            term,
            ..
        } => {
            value_depends_on_temperature(values, count, memo)
                || value_depends_on_temperature(values, initial, memo)
                || value_depends_on_temperature(values, term, memo)
        }
    };
    memo[index] = Some(depends);
    depends
}

fn value_depends_on_loop_index(
    values: &[OptValue],
    value: ValueId,
    memo: &mut [Option<bool>],
) -> bool {
    let index = usize::from(value);
    if let Some(depends) = memo[index] {
        return depends;
    }

    let depends = match values[index].kind {
        OptValueKind::LoopIndex { .. }
        | OptValueKind::RuntimeLoopVariable { .. }
        | OptValueKind::RuntimeLoopVariableDerivative { .. } => true,
        OptValueKind::RealConstant(_)
        | OptValueKind::BooleanConstant(_)
        | OptValueKind::Parameter { .. }
        | OptValueKind::ParamGiven { .. }
        | OptValueKind::Temperature
        | OptValueKind::ThermalVoltage
        | OptValueKind::Multiplicity
        | OptValueKind::Time
        | OptValueKind::Analysis { .. }
        | OptValueKind::Ddx { .. }
        | OptValueKind::Ddt { .. }
        | OptValueKind::DdtScale
        | OptValueKind::NodePotential { .. }
        | OptValueKind::BranchFlow { .. }
        | OptValueKind::BranchUnknownFlow { .. }
        | OptValueKind::RuntimeLoopResult { .. }
        | OptValueKind::RuntimeLoopResultDerivative { .. }
        | OptValueKind::EquationValue { .. } => false,
        OptValueKind::Unary { input, .. } => value_depends_on_loop_index(values, input, memo),
        OptValueKind::Binary { left, right, .. } => {
            value_depends_on_loop_index(values, left, memo)
                || value_depends_on_loop_index(values, right, memo)
        }
        OptValueKind::Select {
            condition,
            then_value,
            else_value,
        } => {
            value_depends_on_loop_index(values, condition, memo)
                || value_depends_on_loop_index(values, then_value, memo)
                || value_depends_on_loop_index(values, else_value, memo)
        }
        OptValueKind::CountedSum {
            count,
            initial,
            term,
            ..
        } => {
            value_depends_on_loop_index(values, count, memo)
                || value_depends_on_loop_index(values, initial, memo)
                || value_depends_on_loop_index(values, term, memo)
        }
    };
    memo[index] = Some(depends);
    depends
}

fn max_invalidation(left: InvalidationClass, right: InvalidationClass) -> InvalidationClass {
    if invalidation_rank(left) >= invalidation_rank(right) {
        left
    } else {
        right
    }
}

fn validate_dense_schedule_ids(diagnostics: &mut Vec<IrDiagnostic>, schedules: &[OptSchedule]) {
    for (expected, schedule) in schedules.iter().enumerate() {
        let expected = u32::try_from(expected).expect("OptIR schedule count exceeds u32::MAX");
        if schedule.id.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::OptValidation,
                format!(
                    "OptIR schedule IDs must be dense: expected ScheduleId({}) at index {}, found {}",
                    expected, expected, schedule.id
                ),
            ));
        }
    }
}

fn validate_schedules(
    diagnostics: &mut Vec<IrDiagnostic>,
    schedules: &[OptSchedule],
    value_count: usize,
    equation_count: u32,
) {
    let mut invalidations = HashSet::new();
    let mut newton_count = 0;
    let mut previous_invalidation = None;

    for schedule in schedules {
        if let Some(previous) = previous_invalidation {
            if invalidation_rank(previous) > invalidation_rank(schedule.invalidation) {
                diagnostics.push(IrDiagnostic::global_error(
                    CompilerPhase::OptValidation,
                    format!(
                        "OptIR schedule order must follow invalidation order: {:?} appears before {:?}",
                        previous, schedule.invalidation
                    ),
                ));
            }
        }
        previous_invalidation = Some(schedule.invalidation);

        if !invalidations.insert(schedule.invalidation) {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::OptValidation,
                format!(
                    "OptIR duplicate schedule for invalidation {:?}",
                    schedule.invalidation
                ),
            ));
        }

        if schedule.invalidation == InvalidationClass::NewtonIteration {
            newton_count += 1;
            validate_newton_schedule(diagnostics, schedule, value_count, equation_count);
        } else {
            validate_schedule_ops(diagnostics, schedule, value_count, equation_count);
        }
    }

    if newton_count != 1 {
        diagnostics.push(IrDiagnostic::global_error(
            CompilerPhase::OptValidation,
            format!(
                "OptIR must contain exactly one NewtonIteration schedule, found {}",
                newton_count
            ),
        ));
    }
}

fn invalidation_rank(invalidation: InvalidationClass) -> u8 {
    match invalidation {
        InvalidationClass::InstanceStatic => 0,
        InvalidationClass::TemperatureStatic => 1,
        InvalidationClass::TimestepStatic => 2,
        InvalidationClass::OperatingPointStatic => 3,
        InvalidationClass::NewtonIteration => 4,
        InvalidationClass::AcFrequency => 5,
        InvalidationClass::NoiseFrequency => 6,
        InvalidationClass::OperatingPointReport => 7,
    }
}

fn validate_newton_schedule(
    diagnostics: &mut Vec<IrDiagnostic>,
    schedule: &OptSchedule,
    value_count: usize,
    equation_count: u32,
) {
    validate_schedule_ops(diagnostics, schedule, value_count, equation_count);

    let equation_ops: Vec<_> = schedule
        .ops
        .iter()
        .filter_map(|op| match op {
            OptOp::EvaluateEquation { equation } => Some(*equation),
            OptOp::ComputeValue { .. } => None,
        })
        .collect();

    if equation_ops.len() != equation_count as usize {
        diagnostics.push(IrDiagnostic::global_error(
            CompilerPhase::OptValidation,
            format!(
                "OptIR NewtonIteration schedule must contain one op per equation: expected {}, found {}",
                equation_count,
                equation_ops.len()
            ),
        ));
    }

    for (expected, equation) in equation_ops.iter().copied().enumerate() {
        let expected = u32::try_from(expected).expect("OptIR op count exceeds u32::MAX");
        if equation.index() != expected {
            diagnostics.push(IrDiagnostic::global_error(
                CompilerPhase::OptValidation,
                format!(
                    "OptIR NewtonIteration op at index {} must evaluate EquationId({}), found {}",
                    expected, expected, equation
                ),
            ));
        }
    }
}

fn validate_schedule_ops(
    diagnostics: &mut Vec<IrDiagnostic>,
    schedule: &OptSchedule,
    value_count: usize,
    equation_count: u32,
) {
    let mut equations = HashSet::new();

    for op in &schedule.ops {
        match op {
            OptOp::ComputeValue { value } => {
                if usize::from(*value) >= value_count {
                    diagnostics.push(IrDiagnostic::global_error(
                        CompilerPhase::OptValidation,
                        format!(
                            "OptIR schedule {} ComputeValue {} is out of range for {} values",
                            schedule.id, value, value_count
                        ),
                    ));
                }
            }
            OptOp::EvaluateEquation { equation } => {
                if equation.index() >= equation_count {
                    diagnostics.push(IrDiagnostic::global_error(
                        CompilerPhase::OptValidation,
                        format!(
                            "OptIR schedule {} EvaluateEquation {} is out of range for {} equations",
                            schedule.id, equation, equation_count
                        ),
                    ));
                }

                if !equations.insert(*equation) {
                    diagnostics.push(IrDiagnostic::global_error(
                        CompilerPhase::OptValidation,
                        format!(
                            "OptIR schedule {} has duplicate equation {}",
                            schedule.id, equation
                        ),
                    ));
                }
            }
        }
    }
}
