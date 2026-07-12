use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

use super::{
    BranchId, BranchUnknownId, CanonicalValueType, CompilerPhase, EquationId, ExprId,
    HirAnalogOperator, HirAssignment, HirExprKind, HirLoop, HirModel, HirParamRange, HirStatement,
    IrDiagnostic, IrValidationResult, MirEquation, MirEquationKind, MirModel, NodeId, ParamId,
    ScheduleId, ValueId, VariableId,
};

const MAX_SCALAR_LOOP_UNROLL_ITERATIONS: usize = 1024;
const MAX_SCALAR_BOUNDED_LOOP_UNROLL_ITERATIONS: usize = 128;
const MAX_SCALAR_BOUNDED_LOOP_ASSIGNMENT_EXPANSIONS: usize = 2048;
const MAX_SCALAR_GUARD_HISTORY_RECONSTRUCTION_ENTRIES: usize = 32;
const MAX_SCALAR_CURRENT_PATH_HISTORY_SCAN_ENTRIES: usize = 512;
const MAX_SCALAR_CURRENT_PATH_HISTORY_DEPTH: usize = 5;
const MAX_SCALAR_CHEAP_CURRENT_PATH_HISTORY_DEPTH: usize = 1;
const MAX_SCALAR_CHEAP_CURRENT_PATH_HISTORY_ENTRIES: usize = 4;
const MAX_SCALAR_EXHAUSTED_HISTORY_REPLAY_ENTRIES: usize = 2;
const MAX_SCALAR_CURRENT_PATH_SELF_UPDATE_CASCADE_SCAN_ENTRIES: usize = 128;
const MAX_SCALAR_CURRENT_PATH_SELF_UPDATE_CASCADE_TERMS: usize = 12;
const MAX_SCALAR_CURRENT_PATH_SELF_UPDATE_CASCADE_ATOMS: usize = 12;
const MAX_SCALAR_RECENT_HISTORY_RECONSTRUCTION_ENTRIES: usize = 512;
const MAX_SCALAR_HISTORY_RECONSTRUCTION_STEPS: usize = 64_000;
const MAX_LARGE_SCALAR_HISTORY_RECONSTRUCTION_STEPS: usize = 4_096;
const MAX_HUGE_SCALAR_HISTORY_RECONSTRUCTION_STEPS: usize = 2_048;
const MAX_SCALAR_EXPRESSION_LOWERING_DEPTH: usize = 2048;
const MAX_SCALAR_ASSIGNMENT_ALIAS_SNAPSHOT_EXPR_NODES: usize = 32;
const MAX_SELECTIVE_ASSIGNMENT_ALIAS_SNAPSHOT_EXPR_NODES: usize = 256;
const MAX_SCALAR_ASSIGNMENT_ALIAS_SNAPSHOT_VARIABLES: usize = 8;
const MAX_SCALAR_LOCAL_ASSIGNMENT_HISTORY_ENTRIES: usize = 5;
const MAX_SCALAR_LOCAL_ASSIGNMENT_SNAPSHOT_VARIABLES: usize = 16;
const MAX_SCALAR_LOCAL_ASSIGNMENT_SNAPSHOT_TOTAL_ENTRIES: usize = 64;
const REQUIRED_SCALAR_LOCAL_ASSIGNMENT_SNAPSHOT_DEPENDENCY_DEPTH: usize = 1;
const MAX_SCALAR_LOCAL_ASSIGNMENT_SNAPSHOT_DEPENDENCY_DEPTH: usize = 2;
const MAX_SELECTIVE_ASSIGNMENT_DEPENDENCY_EXPR_NODES: usize = 256;
const MAX_SELECTIVE_ASSIGNMENT_DEPENDENCY_VARIABLES: usize = 64;
const MAX_SELECTIVE_ASSIGNMENT_BROAD_DEPENDENCY_VARIABLES: usize = 256;
const MAX_SELECTIVE_SCRATCH_HISTORY_ASSIGNMENTS: usize = 4;
const MAX_SELECTIVE_ASSIGNMENT_HISTORY_SNAPSHOT_EXPRESSIONS: usize = 100_000;
const MAX_SELECTIVE_ASSIGNMENT_HISTORY_SNAPSHOT_STATEMENTS: usize = 7_000;
const MAX_SELECTIVE_ASSIGNMENT_HISTORY_TARGETS: usize = 2_048;
const MAX_SELECTIVE_ASSIGNMENT_ARITHMETIC_DEPENDENCY_DEPTH: usize = 2;
const MAX_SELECTIVE_ASSIGNMENT_SIMPLE_DEPENDENCY_DEPTH: usize = 4;
const MAX_LARGE_SELECTIVE_ASSIGNMENT_HISTORY_SNAPSHOT_EXPRESSIONS: usize = 500_000;
const MAX_LARGE_SELECTIVE_ASSIGNMENT_HISTORY_SNAPSHOT_STATEMENTS: usize = 25_000;
const MAX_LARGE_SELECTIVE_ASSIGNMENT_HISTORY_TARGETS: usize = 512;
const MAX_LARGE_SELECTIVE_ASSIGNMENT_ARITHMETIC_DEPENDENCY_DEPTH: usize = 0;
const MAX_LARGE_SELECTIVE_ASSIGNMENT_SIMPLE_DEPENDENCY_DEPTH: usize = 1;
const MAX_EXPANDED_ASSIGNMENT_HISTORY_SNAPSHOT_EXPRESSIONS: usize = 20_000;
const MAX_EXPANDED_ASSIGNMENT_HISTORY_SNAPSHOT_STATEMENTS: usize = 5_000;
const MAX_SCALAR_CONDITION_REASONING_DEPTH: usize = 128;
const MAX_SCALAR_STRUCTURAL_EQUALITY_DEPTH: usize = 192;
pub(crate) const LIMEXP_MAX: f64 = 5.54062238439351e34;
pub(crate) const THERMAL_VOLTAGE_PER_K: f64 = 1.380649e-23 / 1.602176634e-19;

fn opt_phase_trace_enabled() -> bool {
    std::env::var_os("RSPICE_VERILOGA_PHASE_TRACE").is_some()
        || std::env::var_os("RSPICE_VERILOGA_CANONICAL_IR_PHASE_TRACE").is_some()
}

fn opt_statement_trace_enabled() -> bool {
    std::env::var_os("RSPICE_VERILOGA_OPT_STATEMENT_TRACE").is_some()
}

fn opt_assignment_value_trace_filter() -> Option<String> {
    std::env::var("RSPICE_VERILOGA_OPT_ASSIGNMENT_VALUE_TRACE")
        .ok()
        .filter(|filter| !filter.trim().is_empty())
}

fn opt_assignment_snapshot_trace_filter() -> Option<String> {
    std::env::var("RSPICE_VERILOGA_OPT_ASSIGNMENT_SNAPSHOT_TRACE")
        .ok()
        .filter(|filter| !filter.trim().is_empty())
}

fn opt_runtime_loop_trace_enabled() -> bool {
    std::env::var_os("RSPICE_VERILOGA_OPT_RUNTIME_LOOP_TRACE").is_some()
}

fn opt_lower_failure_trace_enabled() -> bool {
    std::env::var_os("RSPICE_VERILOGA_OPT_LOWER_FAILURE_TRACE").is_some()
}

fn opt_lower_failure_trace_equation_filter() -> Option<usize> {
    std::env::var("RSPICE_VERILOGA_OPT_LOWER_FAILURE_TRACE_EQUATION")
        .ok()
        .and_then(|value| value.parse().ok())
}

fn opt_lower_failure_trace_assignment_filter() -> Option<String> {
    std::env::var("RSPICE_VERILOGA_OPT_LOWER_FAILURE_TRACE_ASSIGNMENT")
        .ok()
        .filter(|filter| !filter.trim().is_empty())
}

fn opt_selective_history_target_trace_filter() -> Option<String> {
    std::env::var("RSPICE_VERILOGA_OPT_SELECTIVE_HISTORY_TARGET_TRACE")
        .ok()
        .filter(|filter| !filter.trim().is_empty())
}

fn opt_equation_trace_enabled() -> bool {
    std::env::var_os("RSPICE_VERILOGA_OPT_EQUATION_TRACE").is_some()
}

fn history_trace_enabled() -> bool {
    std::env::var_os("RSPICE_VERILOGA_HISTORY_TRACE").is_some()
}

fn history_trace_variable_filter() -> Option<String> {
    std::env::var("RSPICE_VERILOGA_HISTORY_TRACE_VARIABLE")
        .ok()
        .filter(|filter| !filter.trim().is_empty())
}

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

fn mir_equation_trace_label(equation: &MirEquation) -> String {
    format!(
        "lower equation {} {:?} branch={} expr={} span={}:{}..{}",
        equation.id.index(),
        equation.kind,
        equation.branch.label,
        equation.expression.id.index(),
        equation.span.source_file_id,
        equation.span.start,
        equation.span.end
    )
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
    Mod,
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
        let trace = opt_phase_trace_enabled();
        let trace_equations = opt_equation_trace_enabled();
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
            builder.reset_history_reconstruction_budget_for_equations();
        }
        let mut equation_values = Vec::with_capacity(mir.equations.len());
        trace_opt_phase(trace, &mir.module_name, "lower equations", None, None);
        let phase_started = std::time::Instant::now();
        for equation in &mir.equations {
            let equation_phase = trace_equations.then(|| mir_equation_trace_label(equation));
            if let Some(phase) = equation_phase.as_deref() {
                trace_opt_phase(
                    true,
                    &mir.module_name,
                    phase,
                    None,
                    Some(builder.values.len()),
                );
            }
            let equation_started = std::time::Instant::now();
            let value = builder.lower_equation_expression(equation);
            builder.cache_declared_branch_current(equation, value);
            equation_values.push(value);
            if let Some(phase) = equation_phase.as_deref() {
                trace_opt_phase(
                    true,
                    &mir.module_name,
                    phase,
                    Some(equation_started.elapsed()),
                    Some(builder.values.len()),
                );
            }
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
    variable_ids_by_name: HashMap<SmolStr, VariableId>,
    parameter_ids_by_name: HashMap<SmolStr, ParamId>,
    expr_variable_reference_cache: HashMap<(ExprId, VariableId), bool>,
    variable_values: HashMap<VariableId, Option<ValueId>>,
    variable_assignment_exprs: HashMap<VariableId, ExprId>,
    foldable_initial_step_guards: HashMap<VariableId, SmolStr>,
    assignments_by_variable: HashMap<VariableId, Vec<&'a HirAssignment>>,
    initial_step_group_proofs: HashMap<VariableId, bool>,
    variable_assignment_history: HashMap<VariableId, Vec<AssignmentHistoryEntry>>,
    local_assignment_history: HashMap<VariableId, LocalAssignmentHistory>,
    guarded_path_assignment_exprs: HashMap<VariableId, GuardedPathAssignmentExpr>,
    assignment_history_prefix_cache: HashMap<AssignmentHistoryPrefixCacheKey, Option<ValueId>>,
    current_path_history_cache: HashMap<CurrentPathHistoryCacheKey, Option<ValueId>>,
    recent_assignment_history_cache: HashMap<RecentAssignmentHistoryCacheKey, Option<ValueId>>,
    variable_lowering_stack: HashSet<VariableId>,
    guard_history_lowering_stack: HashSet<(VariableId, usize)>,
    expression_lowering_stack: HashSet<ExprId>,
    expression_lowering_stack_cycle_rejected: bool,
    expression_lowering_depth: usize,
    assignment_history_value_snapshot_stack: Vec<(VariableId, ValueId)>,
    assignment_history_snapshot_stack: Vec<(VariableId, usize)>,
    assignment_history_local_snapshot_stack: Vec<(VariableId, LocalAssignmentHistory)>,
    assignment_history_previous_value_stack: Vec<(VariableId, ValueId)>,
    history_reconstruction_steps: usize,
    history_reconstruction_step_limit: usize,
    history_reconstruction_budget_exhausted: bool,
    potential_equation_state_operator_depth: usize,
    guard_alias_lowering_depth: usize,
    branch_current_values: HashMap<String, ValueId>,
    branch_flow_context: Option<BranchUnknownId>,
    lower_failure_trace_equation: Option<EquationId>,
    lower_failure_trace_assignment: Option<VariableId>,
    lowering_equation: bool,
    conditional_path_stack: Vec<ConditionalPathPredicate>,
    track_assignment_history: bool,
    selective_assignment_history_targets: HashSet<VariableId>,
    local_assignment_history_targets: HashSet<VariableId>,
    expanded_assignment_history_snapshots: bool,
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
    local_dependencies: Vec<VariableId>,
    local_snapshots: Vec<(VariableId, LocalAssignmentHistory)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LocalAssignmentHistory {
    base_value: Option<ValueId>,
    entries: Vec<AssignmentHistoryEntry>,
}

impl LocalAssignmentHistory {
    fn len(&self) -> usize {
        self.entries.len()
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ConditionLiteral {
    condition: ExprId,
    truth: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct IndexedConditionLiteral {
    atom: usize,
    truth: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CurrentPathSelfUpdateCascadeTerm {
    entry: AssignmentHistoryEntry,
    value_expr: ExprId,
    active_literals: Vec<ConditionLiteral>,
    indexed_literals: Vec<IndexedConditionLiteral>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CurrentPathSelfUpdateCascadeCandidate {
    value_expr: ExprId,
    active_literals: Vec<ConditionLiteral>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CurrentPathAssignmentEffect {
    Assign(ValueId),
    KeepPrevious,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CurrentPathAssignmentScan {
    value: Option<ValueId>,
    crossed_unknown_guard: bool,
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
        let expanded_assignment_history_snapshots =
            expanded_assignment_history_snapshots_enabled(hir, mir);
        let track_assignment_history = expanded_assignment_history_snapshots;
        let history_reconstruction_step_limit =
            history_reconstruction_step_limit(hir, mir, expanded_assignment_history_snapshots);
        let selective_assignment_history_targets =
            selective_assignment_history_targets(hir, mir, track_assignment_history);
        let local_assignment_history_targets =
            local_assignment_history_targets(hir, expanded_assignment_history_snapshots);
        let variable_ids_by_name = hir
            .map(|hir| {
                hir.variables
                    .iter()
                    .map(|variable| (variable.name.clone(), variable.id))
                    .collect()
            })
            .unwrap_or_default();
        let parameter_ids_by_name = mir
            .parameters
            .iter()
            .map(|parameter| (parameter.name.clone(), parameter.id))
            .collect();
        let mut assignments_by_variable = HashMap::new();
        if let Some(hir) = hir {
            collect_hir_assignments_by_variable(&hir.statements, &mut assignments_by_variable);
        }
        let foldable_initial_step_guards = assignments_by_variable
            .iter()
            .filter_map(|(variable, assignments)| {
                let guard = assignments.first()?.unfiltered_initial_step_guard.clone()?;
                assignments
                    .iter()
                    .all(|assignment| {
                        assignment.index.is_none()
                            && assignment.unfiltered_initial_step_guard.as_ref() == Some(&guard)
                    })
                    .then_some((*variable, guard))
            })
            .collect();
        Self {
            hir,
            mir,
            values: Vec::new(),
            value_keys: HashMap::new(),
            expression_values: HashMap::new(),
            variable_ids_by_name,
            parameter_ids_by_name,
            expr_variable_reference_cache: HashMap::new(),
            variable_values: HashMap::new(),
            variable_assignment_exprs: HashMap::new(),
            foldable_initial_step_guards,
            assignments_by_variable,
            initial_step_group_proofs: HashMap::new(),
            variable_assignment_history: HashMap::new(),
            local_assignment_history: HashMap::new(),
            guarded_path_assignment_exprs: HashMap::new(),
            assignment_history_prefix_cache: HashMap::new(),
            current_path_history_cache: HashMap::new(),
            recent_assignment_history_cache: HashMap::new(),
            variable_lowering_stack: HashSet::new(),
            guard_history_lowering_stack: HashSet::new(),
            expression_lowering_stack: HashSet::new(),
            expression_lowering_stack_cycle_rejected: false,
            expression_lowering_depth: 0,
            assignment_history_value_snapshot_stack: Vec::new(),
            assignment_history_snapshot_stack: Vec::new(),
            assignment_history_local_snapshot_stack: Vec::new(),
            assignment_history_previous_value_stack: Vec::new(),
            history_reconstruction_steps: 0,
            history_reconstruction_step_limit,
            history_reconstruction_budget_exhausted: false,
            potential_equation_state_operator_depth: 0,
            guard_alias_lowering_depth: 0,
            branch_current_values: HashMap::new(),
            branch_flow_context: None,
            lower_failure_trace_equation: None,
            lower_failure_trace_assignment: None,
            lowering_equation: false,
            conditional_path_stack: Vec::new(),
            track_assignment_history,
            selective_assignment_history_targets,
            local_assignment_history_targets,
            expanded_assignment_history_snapshots,
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
        observable_roots.extend(self.runtime_loop_derivative_observable_roots(&live));

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
            for derivative in &self.values[usize::from(variable.initial)].derivatives {
                stack.push(derivative.value);
            }
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

    fn runtime_loop_derivative_observable_roots(&self, live: &[bool]) -> HashSet<ValueId> {
        let derivative_runtime_loops: HashSet<_> = self
            .values
            .iter()
            .filter(|value| live.get(usize::from(value.id)).copied().unwrap_or(false))
            .filter_map(|value| match value.kind {
                OptValueKind::RuntimeLoopResultDerivative { loop_id, .. } => Some(loop_id),
                _ => None,
            })
            .collect();
        if derivative_runtime_loops.is_empty() {
            return HashSet::new();
        }

        let mut roots = HashSet::new();
        for runtime_loop in &self.runtime_loops {
            if !derivative_runtime_loops.contains(&runtime_loop.loop_id) {
                continue;
            }
            for variable in &runtime_loop.variables {
                roots.insert(variable.initial);
            }
            for assignment in &runtime_loop.assignments {
                roots.insert(assignment.value);
            }
        }
        roots
    }

    fn lower_statements(&mut self, statements: &[HirStatement]) {
        let trace = opt_statement_trace_enabled();
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
        if let Some(value) = self.lower_static_unfiltered_initial_step_assignment(assignment) {
            self.variable_values.insert(assignment.target, Some(value));
            self.variable_assignment_exprs.remove(&assignment.target);
            self.variable_assignment_history.remove(&assignment.target);
            self.local_assignment_history.remove(&assignment.target);
            self.expression_values.clear();
            return;
        }
        let records_history = self.should_record_assignment_history(assignment)
            && assignment.index.is_none()
            && supported_assignment_value_type(assignment.expr_type);
        if records_history {
            let entry = self.assignment_history_entry(assignment.target, assignment.expr.id);
            self.variable_assignment_exprs
                .insert(assignment.target, assignment.expr.id);
            self.variable_assignment_history
                .entry(assignment.target)
                .or_default()
                .push(entry);
        } else {
            if self.should_retain_current_path_assignment_expr(assignment) {
                self.variable_assignment_exprs
                    .insert(assignment.target, assignment.expr.id);
            } else {
                self.variable_assignment_exprs.remove(&assignment.target);
            }
            self.variable_assignment_history.remove(&assignment.target);
        }
        self.update_local_assignment_history(assignment);
        let value = if assignment.index.is_none()
            && supported_assignment_value_type(assignment.expr_type)
        {
            self.lower_assignment_value_with_trace(assignment)
        } else {
            None
        };
        self.variable_values.insert(assignment.target, value);
        self.absorb_local_assignment_history_value(assignment.target, value);
        self.trace_assignment_value(assignment, value, records_history);
        self.expression_values.clear();
    }

    /// Fold assignments only when every write to the variable belongs to one
    /// unfiltered `initial_step` event and each sequential result is entirely
    /// instance- or temperature-static. Exact semantic provenance ensures this
    /// removes no nested user condition and fails closed for other event shapes.
    fn lower_static_unfiltered_initial_step_assignment(
        &mut self,
        assignment: &HirAssignment,
    ) -> Option<ValueId> {
        let guard_name = self
            .foldable_initial_step_guards
            .get(&assignment.target)?
            .clone();
        if !self.prove_initial_step_group_static(assignment.target, &guard_name) {
            return None;
        }
        let expression = self.mir.expressions.get(usize::from(assignment.expr.id))?;
        let HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } = expression.kind
        else {
            return None;
        };
        if self.variable_identifier(else_expr) != Some(assignment.target)
            || self.expr_references_variable(then_expr, assignment.target)
        {
            return None;
        }

        let remaining_conditions = self.strip_initial_step_guard(condition, &guard_name)?;
        let active = self.lower_expression(then_expr)?;
        let active = self.coerce_value_to_variable_type(assignment.target, active)?;
        let value =
            if let Some(condition) = self.lower_initial_step_conditions(&remaining_conditions)? {
                let fallback = self.current_variable_value(assignment.target)?;
                self.push_typed_select(
                    self.variable_opt_value_type(assignment.target)?,
                    condition,
                    active,
                    fallback,
                )
            } else {
                active
            };

        let mut memo = vec![None; self.values.len()];
        match value_invalidation(&self.values, value, &mut memo) {
            InvalidationClass::InstanceStatic | InvalidationClass::TemperatureStatic => Some(value),
            _ => None,
        }
    }

    fn prove_initial_step_group_static(&mut self, variable: VariableId, guard_name: &str) -> bool {
        if let Some(proven) = self.initial_step_group_proofs.get(&variable).copied() {
            return proven;
        }
        let assignments = self
            .assignments_by_variable
            .get(&variable)
            .cloned()
            .unwrap_or_default();
        self.materialize_static_guard_aliases();
        let unavailable_variables: HashSet<_> = self
            .variable_ids_by_name
            .values()
            .copied()
            .filter(|candidate| {
                *candidate != variable
                    && !matches!(self.variable_values.get(candidate), Some(Some(_)))
            })
            .collect();
        let Some(value_type) = self.variable_value_type(variable) else {
            self.initial_step_group_proofs.insert(variable, false);
            return false;
        };
        let Some(mut fallback) = self.default_variable_value(value_type) else {
            self.initial_step_group_proofs.insert(variable, false);
            return false;
        };

        let proven = assignments.into_iter().all(|assignment| {
            let Some(expression) = self.mir.expressions.get(usize::from(assignment.expr.id)) else {
                return false;
            };
            let HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } = expression.kind
            else {
                return false;
            };
            if self.variable_identifier(else_expr) != Some(variable)
                || self.expr_references_variable(then_expr, variable)
                || self.expr_references_any_variable(then_expr, &unavailable_variables)
            {
                return false;
            }
            let Some(remaining_conditions) = self.strip_initial_step_guard(condition, guard_name)
            else {
                return false;
            };
            if remaining_conditions.iter().any(|condition| {
                self.expr_references_any_variable(*condition, &unavailable_variables)
            }) {
                return false;
            }
            let Some(active) = self.lower_expression(then_expr) else {
                return false;
            };
            let Some(active) = self.coerce_value_to_variable_type(variable, active) else {
                return false;
            };
            let Some(condition) = self.lower_initial_step_conditions(&remaining_conditions) else {
                return false;
            };
            let value = if let Some(condition) = condition {
                self.push_typed_select(
                    self.variable_opt_value_type(variable)
                        .expect("checked variable type"),
                    condition,
                    active,
                    fallback,
                )
            } else {
                active
            };
            let mut memo = vec![None; self.values.len()];
            if !matches!(
                value_invalidation(&self.values, value, &mut memo),
                InvalidationClass::InstanceStatic | InvalidationClass::TemperatureStatic
            ) {
                return false;
            }
            fallback = value;
            true
        });
        self.initial_step_group_proofs.insert(variable, proven);
        proven
    }

    /// Guard snapshots are emitted before their guarded branch, but an
    /// if/else-if chain emits later sibling snapshots between assignments to
    /// the initialized variable. Materialize only single-assignment, pure
    /// static guard aliases so the whole initialization group can be proven
    /// before any member is folded.
    fn materialize_static_guard_aliases(&mut self) {
        loop {
            let unavailable: HashSet<_> = self
                .variable_ids_by_name
                .values()
                .copied()
                .filter(|variable| !matches!(self.variable_values.get(variable), Some(Some(_))))
                .collect();
            let candidates: Vec<_> = self
                .assignments_by_variable
                .iter()
                .filter_map(|(variable, assignments)| {
                    let name = self.variable_name(*variable);
                    (name.starts_with("__guard") && assignments.len() == 1)
                        .then_some((*variable, assignments[0].expr.id))
                })
                .collect();
            let mut progress = false;
            for (variable, expr) in candidates {
                if matches!(self.variable_values.get(&variable), Some(Some(_)))
                    || self.expr_references_any_variable(expr, &unavailable)
                {
                    continue;
                }
                let Some(value) = self.lower_expression(expr) else {
                    continue;
                };
                let Some(value) = self.coerce_value_to_variable_type(variable, value) else {
                    continue;
                };
                let mut memo = vec![None; self.values.len()];
                if matches!(
                    value_invalidation(&self.values, value, &mut memo),
                    InvalidationClass::InstanceStatic | InvalidationClass::TemperatureStatic
                ) {
                    self.variable_values.insert(variable, Some(value));
                    progress = true;
                }
            }
            if !progress {
                break;
            }
        }
    }

    /// Return the conjunction terms left after removing the exact
    /// snapshotted event guard from the left spine of an `And` tree.
    fn strip_initial_step_guard(&self, condition: ExprId, guard_name: &str) -> Option<Vec<ExprId>> {
        let expression = self.mir.expressions.get(usize::from(condition))?;
        match &expression.kind {
            HirExprKind::Identifier { name } if name == guard_name => Some(Vec::new()),
            HirExprKind::Binary { op, left, right } if op.as_str() == "And" => {
                let mut remaining = self.strip_initial_step_guard(*left, guard_name)?;
                remaining.push(*right);
                Some(remaining)
            }
            _ => None,
        }
    }

    fn lower_initial_step_conditions(&mut self, conditions: &[ExprId]) -> Option<Option<ValueId>> {
        let mut lowered = conditions.iter().copied();
        let Some(first) = lowered.next() else {
            return Some(None);
        };
        let mut condition = self.lower_expression(first)?;
        for next in lowered {
            let next = self.lower_expression(next)?;
            condition = self.push_value(
                OptValueType::Boolean,
                OptValueKind::Binary {
                    op: OptBinaryOp::And,
                    left: condition,
                    right: next,
                },
            );
        }
        Some(Some(condition))
    }

    fn lower_assignment_value_with_trace(&mut self, assignment: &HirAssignment) -> Option<ValueId> {
        let previous_assignment = self.lower_failure_trace_assignment;
        self.lower_failure_trace_assignment = Some(assignment.target);
        let lowered = self
            .lower_expression(assignment.expr.id)
            .and_then(|value| self.coerce_value_to_variable_type(assignment.target, value));
        self.lower_failure_trace_assignment = previous_assignment;
        lowered
    }

    fn trace_assignment_value(
        &self,
        assignment: &HirAssignment,
        value: Option<ValueId>,
        records_history: bool,
    ) {
        let Some(filter) = opt_assignment_value_trace_filter() else {
            return;
        };
        let matched = filter
            .split(',')
            .map(str::trim)
            .any(|filter| !filter.is_empty() && assignment.target_name.contains(filter));
        if !matched {
            return;
        }
        let history_entries = self
            .variable_assignment_history
            .get(&assignment.target)
            .map(Vec::len)
            .unwrap_or(0);
        eprintln!(
            "OptIR {}: assignment value target={} expr={} value={:?} records_history={} history_entries={} steps={} exhausted={}",
            self.mir.module_name,
            assignment.target_name,
            assignment.expr.id.index(),
            value,
            records_history,
            history_entries,
            self.history_reconstruction_steps,
            self.history_reconstruction_budget_exhausted
        );
    }

    fn trace_assignment_snapshot(
        &self,
        target: VariableId,
        expr: ExprId,
        variables: &BTreeSet<VariableId>,
        entry: &AssignmentHistoryEntry,
    ) {
        let Some(filter) = opt_assignment_snapshot_trace_filter() else {
            return;
        };
        let matched = filter
            .split(',')
            .map(str::trim)
            .any(|filter| !filter.is_empty() && self.variable_name(target).contains(filter));
        if !matched {
            return;
        }
        let variables: Vec<_> = variables
            .iter()
            .map(|variable| self.trace_variable_label(*variable))
            .collect();
        let value_snapshots: Vec<_> = entry
            .value_snapshots
            .iter()
            .map(|(variable, value)| format!("{}={value:?}", self.trace_variable_label(*variable)))
            .collect();
        let history_snapshots: Vec<_> = entry
            .snapshots
            .iter()
            .map(|(variable, limit)| format!("{}@{limit}", self.trace_variable_label(*variable)))
            .collect();
        let local_dependencies: Vec<_> = entry
            .local_dependencies
            .iter()
            .map(|variable| self.trace_variable_label(*variable))
            .collect();
        let local_snapshots: Vec<_> = entry
            .local_snapshots
            .iter()
            .map(|(variable, history)| {
                let exprs: Vec<_> = history
                    .entries
                    .iter()
                    .map(|entry| entry.expr.index().to_string())
                    .collect();
                format!(
                    "{}:base={:?}:{}[{}]",
                    self.trace_variable_label(*variable),
                    history.base_value,
                    history.entries.len(),
                    exprs.join("|")
                )
            })
            .collect();
        eprintln!(
            "OptIR {}: assignment snapshot target={} expr={} variables=[{}] value_snapshots=[{}] history_snapshots=[{}] local_dependencies=[{}] local_snapshots=[{}]",
            self.mir.module_name,
            self.trace_variable_label(target),
            expr.index(),
            variables.join(","),
            value_snapshots.join(","),
            history_snapshots.join(","),
            local_dependencies.join(","),
            local_snapshots.join(",")
        );
    }

    fn trace_local_assignment_history_update(
        &self,
        assignment: &HirAssignment,
        action: &str,
        entries: usize,
    ) {
        let Some(filter) = opt_assignment_snapshot_trace_filter() else {
            return;
        };
        let target_name = self.variable_name(assignment.target);
        let matched = filter
            .split(',')
            .map(str::trim)
            .any(|filter| !filter.is_empty() && target_name.contains(filter));
        if !matched {
            return;
        }
        eprintln!(
            "OptIR {}: local assignment history {action} target={} expr={} entries={} track_history={} selected={}",
            self.mir.module_name,
            self.trace_variable_label(assignment.target),
            assignment.expr.id.index(),
            entries,
            self.track_assignment_history,
            self.local_assignment_history_targets
                .contains(&assignment.target)
        );
    }

    fn trace_local_assignment_history_snapshot_reject(
        &self,
        variables: impl IntoIterator<Item = VariableId>,
        reason: &str,
    ) {
        let Some(filter) = opt_assignment_snapshot_trace_filter() else {
            return;
        };
        let variables: Vec<_> = variables.into_iter().collect();
        let matched = variables.iter().any(|variable| {
            let name = self.variable_name(*variable);
            filter
                .split(',')
                .map(str::trim)
                .any(|filter| !filter.is_empty() && name.contains(filter))
        });
        if !matched {
            return;
        }
        let labels: Vec<_> = variables
            .iter()
            .map(|variable| self.trace_variable_label(*variable))
            .collect();
        eprintln!(
            "OptIR {}: local assignment history snapshot rejected variables=[{}] reason={reason}",
            self.mir.module_name,
            labels.join(",")
        );
    }

    fn should_record_assignment_history(&mut self, assignment: &HirAssignment) -> bool {
        if assignment.index.is_some() || !supported_assignment_value_type(assignment.expr_type) {
            return false;
        }
        if self.track_assignment_history {
            return true;
        }
        if !self
            .selective_assignment_history_targets
            .contains(&assignment.target)
        {
            return false;
        }
        if self
            .conditional_self_update(assignment.target, assignment.expr.id)
            .is_some_and(|update| {
                !self.expr_references_variable(update.value_expr, assignment.target)
            })
        {
            return true;
        }
        if !self.expr_references_variable(assignment.expr.id, assignment.target) {
            return true;
        }
        self.self_assignment_alias_snapshot_variables(assignment.target, assignment.expr.id)
            .is_some()
    }

    fn should_retain_current_path_assignment_expr(&mut self, assignment: &HirAssignment) -> bool {
        if assignment.index.is_some() || !supported_assignment_value_type(assignment.expr_type) {
            return false;
        }
        if assignment.target_name.starts_with("__guard") {
            return true;
        }
        self.conditional_self_update(assignment.target, assignment.expr.id)
            .is_some_and(|update| {
                !self.expr_references_variable(update.value_expr, assignment.target)
            })
    }

    fn should_record_local_assignment_history(&self, assignment: &HirAssignment) -> bool {
        assignment.index.is_none()
            && supported_assignment_value_type(assignment.expr_type)
            && !self.track_assignment_history
            && self
                .local_assignment_history_targets
                .contains(&assignment.target)
    }

    fn update_local_assignment_history(&mut self, assignment: &HirAssignment) {
        if !self.should_record_local_assignment_history(assignment) {
            let removed = self.local_assignment_history.remove(&assignment.target);
            self.trace_local_assignment_history_update(
                assignment,
                "remove",
                removed
                    .as_ref()
                    .map(LocalAssignmentHistory::len)
                    .unwrap_or_default(),
            );
            return;
        }
        let entry = self.local_assignment_history_entry(assignment.target, assignment.expr.id);
        let mut history = self
            .local_assignment_history
            .remove(&assignment.target)
            .unwrap_or_else(|| LocalAssignmentHistory {
                base_value: self.current_variable_value(assignment.target),
                entries: Vec::new(),
            });
        history.entries.push(entry);
        while history.entries.len() > MAX_SCALAR_LOCAL_ASSIGNMENT_HISTORY_ENTRIES {
            self.advance_local_assignment_history_base(assignment.target, &mut history);
        }
        let entries = history.entries.len();
        self.local_assignment_history
            .insert(assignment.target, history);
        self.trace_local_assignment_history_update(assignment, "record", entries);
    }

    fn advance_local_assignment_history_base(
        &mut self,
        variable: VariableId,
        history: &mut LocalAssignmentHistory,
    ) {
        if history.entries.is_empty() {
            return;
        }
        let entry = history.entries.remove(0);
        let Some(base_value) = history.base_value else {
            return;
        };
        let original_expression_values = std::mem::take(&mut self.expression_values);
        let advanced = self
            .lower_assignment_history_expr_from_previous_with_budget(
                variable, &entry, base_value, false,
            )
            .and_then(|value| self.coerce_value_to_variable_type(variable, value));
        self.expression_values = original_expression_values;
        if advanced.is_none() && self.trace_history_variable(variable) {
            eprintln!(
                "OptIR {}: local assignment history base advance failed variable={} id={} expr={}",
                self.mir.module_name,
                self.variable_name(variable),
                variable.index(),
                entry.expr.index()
            );
        }
        history.base_value = advanced;
    }

    fn absorb_local_assignment_history_value(
        &mut self,
        variable: VariableId,
        value: Option<ValueId>,
    ) {
        let Some(value) = value else {
            return;
        };
        let Some(history) = self.local_assignment_history.get_mut(&variable) else {
            return;
        };
        history.base_value = Some(value);
        history.entries.clear();
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
        let original_local_assignment_history = self.local_assignment_history.clone();
        let original_guarded_path_assignment_exprs = self.guarded_path_assignment_exprs.clone();
        let original_assignment_history_prefix_cache = self.assignment_history_prefix_cache.clone();
        let original_current_path_history_cache = self.current_path_history_cache.clone();
        let original_recent_assignment_history_cache = self.recent_assignment_history_cache.clone();
        let original_conditional_path_stack = self.conditional_path_stack.clone();
        let original_history_reconstruction_steps = self.history_reconstruction_steps;
        let original_history_reconstruction_budget_exhausted =
            self.history_reconstruction_budget_exhausted;

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
            self.local_assignment_history = original_local_assignment_history;
            self.guarded_path_assignment_exprs = original_guarded_path_assignment_exprs;
            self.assignment_history_prefix_cache = original_assignment_history_prefix_cache;
            self.current_path_history_cache = original_current_path_history_cache;
            self.recent_assignment_history_cache = original_recent_assignment_history_cache;
            self.conditional_path_stack = original_conditional_path_stack;
            self.history_reconstruction_steps = original_history_reconstruction_steps;
            self.history_reconstruction_budget_exhausted =
                original_history_reconstruction_budget_exhausted;
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
        let original_local_assignment_history = self.local_assignment_history.clone();
        let original_guarded_path_assignment_exprs = self.guarded_path_assignment_exprs.clone();
        let original_assignment_history_prefix_cache = self.assignment_history_prefix_cache.clone();
        let original_current_path_history_cache = self.current_path_history_cache.clone();
        let original_recent_assignment_history_cache = self.recent_assignment_history_cache.clone();
        let original_conditional_path_stack = self.conditional_path_stack.clone();
        let original_runtime_loops = self.runtime_loops.clone();
        let original_next_loop_id = self.next_loop_id;
        let original_history_reconstruction_steps = self.history_reconstruction_steps;
        let original_history_reconstruction_budget_exhausted =
            self.history_reconstruction_budget_exhausted;

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
            self.local_assignment_history = original_local_assignment_history;
            self.guarded_path_assignment_exprs = original_guarded_path_assignment_exprs;
            self.assignment_history_prefix_cache = original_assignment_history_prefix_cache;
            self.current_path_history_cache = original_current_path_history_cache;
            self.recent_assignment_history_cache = original_recent_assignment_history_cache;
            self.conditional_path_stack = original_conditional_path_stack;
            self.runtime_loops = original_runtime_loops;
            self.next_loop_id = original_next_loop_id;
            self.history_reconstruction_steps = original_history_reconstruction_steps;
            self.history_reconstruction_budget_exhausted =
                original_history_reconstruction_budget_exhausted;
            false
        }
    }

    fn try_lower_runtime_bounded_guarded_loop(&mut self, loop_statement: &HirLoop) -> bool {
        let trace = opt_runtime_loop_trace_enabled();
        let Some(counter) = self.runtime_loop_counter(loop_statement.condition.id) else {
            if trace {
                eprintln!(
                    "OptIR {}: runtime loop rejected: no scalar loop counter for condition {}",
                    self.mir.module_name,
                    loop_statement.condition.id.index()
                );
            }
            return false;
        };
        if loop_statement
            .body
            .iter()
            .any(|statement| !matches!(statement, HirStatement::Assignment(_)))
        {
            if trace {
                eprintln!(
                    "OptIR {}: runtime loop rejected: non-assignment body statement",
                    self.mir.module_name
                );
            }
            return false;
        }
        if !loop_statement.body.iter().any(|statement| {
            matches!(
                statement,
                HirStatement::Assignment(assignment)
                    if self.is_counter_increment_assignment(assignment, counter)
            )
        }) {
            if trace {
                eprintln!(
                    "OptIR {}: runtime loop rejected: no increment for counter {} ({})",
                    self.mir.module_name,
                    counter.index(),
                    self.variable_name(counter)
                );
            }
            return false;
        }

        let Some(assigned_variables) = self.ordered_loop_assignment_targets(&loop_statement.body)
        else {
            if trace {
                eprintln!(
                    "OptIR {}: runtime loop rejected: unsupported assignment target",
                    self.mir.module_name
                );
            }
            return false;
        };
        let Some(required_initials) = self.runtime_loop_initial_value_required_targets(
            loop_statement.condition.id,
            &loop_statement.body,
            &assigned_variables,
        ) else {
            if trace {
                eprintln!(
                    "OptIR {}: runtime loop rejected: unsupported initial-value dependency scan",
                    self.mir.module_name
                );
            }
            return false;
        };
        let Some(loop_id) = self.allocate_loop_id() else {
            if trace {
                eprintln!(
                    "OptIR {}: runtime loop rejected: loop id overflow",
                    self.mir.module_name
                );
            }
            return false;
        };

        let mut runtime_variable_inputs = Vec::with_capacity(assigned_variables.len());
        for variable in &assigned_variables {
            let Some(value_type) = self.variable_opt_value_type(*variable) else {
                if trace {
                    eprintln!(
                        "OptIR {}: runtime loop rejected: unsupported variable type for {} ({})",
                        self.mir.module_name,
                        variable.index(),
                        self.variable_name(*variable)
                    );
                }
                return false;
            };
            let current = self
                .current_variable_value(*variable)
                .map(|value| self.coerce_value_to_type(value, value_type));
            let initial = if required_initials.contains(variable) {
                let initial = current.or_else(|| {
                    self.runtime_loop_current_path_initial(
                        *variable,
                        value_type,
                        loop_statement.condition.id,
                    )
                });
                let Some(initial) = initial else {
                    if trace {
                        eprintln!(
                            "OptIR {}: runtime loop rejected: missing carried initial for {} ({})",
                            self.mir.module_name,
                            variable.index(),
                            self.variable_name(*variable)
                        );
                    }
                    return false;
                };
                initial
            } else if let Some(initial) = current {
                initial
            } else {
                let Some(canonical_type) = self.variable_value_type(*variable) else {
                    if trace {
                        eprintln!(
                            "OptIR {}: runtime loop rejected: missing local type for {} ({})",
                            self.mir.module_name,
                            variable.index(),
                            self.variable_name(*variable)
                        );
                    }
                    return false;
                };
                let Some(initial) = self
                    .default_variable_value(canonical_type)
                    .map(|value| self.coerce_value_to_type(value, value_type))
                else {
                    if trace {
                        eprintln!(
                            "OptIR {}: runtime loop rejected: missing local default for {} ({})",
                            self.mir.module_name,
                            variable.index(),
                            self.variable_name(*variable)
                        );
                    }
                    return false;
                };
                initial
            };
            runtime_variable_inputs.push((*variable, value_type, initial));
        }

        let mut slot_by_variable = HashMap::new();
        let mut variables = Vec::with_capacity(runtime_variable_inputs.len());
        for (variable, value_type, initial) in runtime_variable_inputs {
            let slot = u32::try_from(variables.len()).ok();
            let Some(slot) = slot else {
                if trace {
                    eprintln!(
                        "OptIR {}: runtime loop rejected: slot overflow",
                        self.mir.module_name
                    );
                }
                return false;
            };
            let variable_value = self.push_value(
                value_type,
                OptValueKind::RuntimeLoopVariable { loop_id, slot },
            );
            slot_by_variable.insert(variable, slot);
            self.variable_values.insert(variable, Some(variable_value));
            variables.push(OptRuntimeLoopVariable {
                source: variable,
                value_type,
                initial,
                variable: variable_value,
                result: ValueId::from(0),
            });
        }

        self.expression_values.clear();
        let Some(condition) = self.lower_expression(loop_statement.condition.id) else {
            if trace {
                eprintln!(
                    "OptIR {}: runtime loop rejected: failed to lower condition {}",
                    self.mir.module_name,
                    loop_statement.condition.id.index()
                );
            }
            return false;
        };

        let mut assignments = Vec::new();
        self.clear_assignment_replay_caches();
        self.conditional_path_stack.push(ConditionalPathPredicate {
            condition: loop_statement.condition.id,
            truth: true,
        });
        for statement in &loop_statement.body {
            let HirStatement::Assignment(assignment) = statement else {
                self.conditional_path_stack.pop();
                return false;
            };
            if assignment.index.is_some() || !supported_assignment_value_type(assignment.expr_type)
            {
                if trace {
                    eprintln!(
                        "OptIR {}: runtime loop rejected: unsupported assignment {} ({})",
                        self.mir.module_name,
                        assignment.target.index(),
                        assignment.target_name
                    );
                }
                self.conditional_path_stack.pop();
                return false;
            }
            let Some(slot) = slot_by_variable.get(&assignment.target).copied() else {
                if trace {
                    eprintln!(
                        "OptIR {}: runtime loop rejected: missing slot for {} ({})",
                        self.mir.module_name,
                        assignment.target.index(),
                        assignment.target_name
                    );
                }
                self.conditional_path_stack.pop();
                return false;
            };
            if self.should_record_assignment_history(assignment) {
                let entry = self.assignment_history_entry(assignment.target, assignment.expr.id);
                self.variable_assignment_exprs
                    .insert(assignment.target, assignment.expr.id);
                self.variable_assignment_history
                    .entry(assignment.target)
                    .or_default()
                    .push(entry);
            }
            self.update_local_assignment_history(assignment);
            self.expression_values.clear();
            let Some(value) = self.lower_assignment_value_with_trace(assignment) else {
                if trace {
                    eprintln!(
                        "OptIR {}: runtime loop rejected: failed to lower assignment {} ({}) expr={}",
                        self.mir.module_name,
                        assignment.target.index(),
                        assignment.target_name,
                        assignment.expr.id.index()
                    );
                }
                self.conditional_path_stack.pop();
                return false;
            };
            self.variable_values.insert(assignment.target, Some(value));
            self.absorb_local_assignment_history_value(assignment.target, Some(value));
            assignments.push(OptRuntimeLoopAssignment { slot, value });
        }
        self.conditional_path_stack.pop();

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
        let runtime_variable_results: Vec<_> = self
            .runtime_loops
            .last()
            .expect("runtime loop was just pushed")
            .variables
            .iter()
            .map(|runtime_variable| (runtime_variable.source, runtime_variable.result))
            .collect();
        for (source, result) in runtime_variable_results {
            self.variable_values.insert(source, Some(result));
            self.variable_assignment_exprs.remove(&source);
            self.variable_assignment_history.remove(&source);
            self.local_assignment_history.remove(&source);
            self.guarded_path_assignment_exprs.remove(&source);
        }
        self.expression_values.clear();
        if trace {
            eprintln!(
                "OptIR {}: runtime loop accepted: loop_id={} variables={} assignments={} carried_initials={}",
                self.mir.module_name,
                loop_id,
                self.runtime_loops
                    .last()
                    .map(|runtime_loop| runtime_loop.variables.len())
                    .unwrap_or_default(),
                self.runtime_loops
                    .last()
                    .map(|runtime_loop| runtime_loop.assignments.len())
                    .unwrap_or_default(),
                required_initials.len()
            );
        }
        true
    }

    fn runtime_loop_counter(&mut self, condition: ExprId) -> Option<VariableId> {
        if let Some((counter, _iteration_count)) =
            self.bounded_guarded_loop_iteration_count(condition)
        {
            return Some(counter);
        }

        let counter = self.runtime_loop_bound_counter(condition)?;
        matches!(
            self.variable_value_type(counter),
            Some(CanonicalValueType::Integer | CanonicalValueType::Real)
        )
        .then_some(counter)
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
        if self.should_record_assignment_history(assignment) {
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
        } else {
            self.guarded_path_assignment_exprs
                .remove(&assignment.target);
            self.variable_assignment_exprs.remove(&assignment.target);
            self.variable_assignment_history.remove(&assignment.target);
        }
        self.update_local_assignment_history(assignment);
        self.expression_values.clear();
        let Some(next) = self.lower_assignment_value_with_trace(assignment) else {
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
        let value = self.push_typed_select(value_type, guard, next, previous);
        self.variable_values.insert(assignment.target, Some(value));
        self.absorb_local_assignment_history_value(assignment.target, Some(value));
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
        let original_local_assignment_history = self.local_assignment_history.clone();
        let original_guarded_path_assignment_exprs = self.guarded_path_assignment_exprs.clone();
        let original_assignment_history_prefix_cache = self.assignment_history_prefix_cache.clone();
        let original_current_path_history_cache = self.current_path_history_cache.clone();
        let original_recent_assignment_history_cache = self.recent_assignment_history_cache.clone();
        let original_next_loop_id = self.next_loop_id;
        let original_history_reconstruction_steps = self.history_reconstruction_steps;
        let original_history_reconstruction_budget_exhausted =
            self.history_reconstruction_budget_exhausted;

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
            self.local_assignment_history = original_local_assignment_history;
            self.guarded_path_assignment_exprs = original_guarded_path_assignment_exprs;
            self.assignment_history_prefix_cache = original_assignment_history_prefix_cache;
            self.current_path_history_cache = original_current_path_history_cache;
            self.recent_assignment_history_cache = original_recent_assignment_history_cache;
            self.next_loop_id = original_next_loop_id;
            self.history_reconstruction_steps = original_history_reconstruction_steps;
            self.history_reconstruction_budget_exhausted =
                original_history_reconstruction_budget_exhausted;
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
            let Some(value) = self
                .lower_expression(expr)
                .and_then(|value| self.coerce_value_to_variable_type(assignment.target, value))
            else {
                return false;
            };
            self.variable_values.insert(assignment.target, Some(value));
        }

        self.variable_values = original_variable_values;
        self.clear_assignment_replay_caches();
        for assigned in assigned_variables {
            self.variable_assignment_exprs.remove(&assigned);
            self.variable_assignment_history.remove(&assigned);
            self.local_assignment_history.remove(&assigned);
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

    fn runtime_loop_initial_value_required_targets(
        &mut self,
        condition: ExprId,
        statements: &[HirStatement],
        assigned_variables: &[VariableId],
    ) -> Option<HashSet<VariableId>> {
        let assigned: HashSet<_> = assigned_variables.iter().copied().collect();
        let mut unassigned = assigned.clone();
        let mut required = self.expr_referenced_loop_targets(condition, &assigned)?;

        for statement in statements {
            let HirStatement::Assignment(assignment) = statement else {
                return None;
            };
            if assignment.index.is_some() || !supported_assignment_value_type(assignment.expr_type)
            {
                return None;
            }
            match self.runtime_loop_initial_dependency_expr(assignment, condition)? {
                Some(expr) => {
                    required.extend(self.expr_referenced_loop_targets(expr, &unassigned)?);
                    unassigned.remove(&assignment.target);
                }
                None => {
                    if unassigned.contains(&assignment.target) {
                        required.insert(assignment.target);
                    }
                }
            }
        }

        Some(required)
    }

    fn runtime_loop_initial_dependency_expr(
        &mut self,
        assignment: &HirAssignment,
        loop_condition: ExprId,
    ) -> Option<Option<ExprId>> {
        let Some(update) = self.conditional_self_update(assignment.target, assignment.expr.id)
        else {
            return Some(Some(assignment.expr.id));
        };
        self.conditional_path_stack.push(ConditionalPathPredicate {
            condition: loop_condition,
            truth: true,
        });
        let condition_truth = self.condition_truth_in_current_path(update.condition);
        self.conditional_path_stack.pop();
        match condition_truth {
            Some(truth) if truth == update.active_truth => Some(Some(update.value_expr)),
            Some(_) => Some(None),
            None => Some(Some(assignment.expr.id)),
        }
    }

    fn expr_referenced_loop_targets(
        &self,
        expr: ExprId,
        targets: &HashSet<VariableId>,
    ) -> Option<HashSet<VariableId>> {
        let mut referenced = HashSet::new();
        let mut visited = HashSet::new();
        let mut stack = vec![expr];
        while let Some(expr) = stack.pop() {
            if !visited.insert(expr) {
                continue;
            }
            let expression = self.mir.expressions.get(usize::from(expr))?;
            match &expression.kind {
                HirExprKind::Identifier { .. } => {
                    if let Some(variable) = self.variable_identifier(expr)
                        && targets.contains(&variable)
                    {
                        referenced.insert(variable);
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
                | HirExprKind::NoiseSource { .. } => return None,
            }
        }
        Some(referenced)
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

    fn runtime_loop_current_path_initial(
        &mut self,
        variable: VariableId,
        value_type: OptValueType,
        condition: ExprId,
    ) -> Option<ValueId> {
        if self.conditional_path_stack.len() >= MAX_SCALAR_CURRENT_PATH_HISTORY_DEPTH {
            return None;
        }
        self.conditional_path_stack.push(ConditionalPathPredicate {
            condition,
            truth: true,
        });
        let initial = self
            .lower_conditional_path_variable_identifier(variable)
            .map(|value| self.coerce_value_to_type(value, value_type));
        self.conditional_path_stack.pop();
        self.expression_values.clear();
        initial
    }

    fn variable_value_type(&self, variable: VariableId) -> Option<CanonicalValueType> {
        self.hir?
            .variables
            .get(usize::from(variable))
            .map(|variable| variable.value_type)
    }

    fn variable_name(&self, variable: VariableId) -> &str {
        self.hir
            .and_then(|hir| hir.variables.get(usize::from(variable)))
            .map(|variable| variable.name.as_str())
            .unwrap_or("<unknown>")
    }

    fn trace_variable_label(&self, variable: VariableId) -> String {
        format!("{}#{}", self.variable_name(variable), variable.index())
    }

    fn trace_history_variable(&self, variable: VariableId) -> bool {
        if !history_trace_enabled() {
            return false;
        }
        let Some(filter) = history_trace_variable_filter() else {
            return true;
        };
        let name = self.variable_name(variable);
        filter
            .split(',')
            .map(str::trim)
            .any(|filter| !filter.is_empty() && name.contains(filter))
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

    fn coerce_value_to_variable_type(
        &mut self,
        variable: VariableId,
        value: ValueId,
    ) -> Option<ValueId> {
        let value_type = self.variable_opt_value_type(variable)?;
        Some(self.coerce_value_to_type(value, value_type))
    }

    fn coerce_value_to_type(&mut self, value: ValueId, target_type: OptValueType) -> ValueId {
        let source_type = self.values[usize::from(value)].value_type;
        if source_type == target_type {
            return value;
        }

        match (source_type, target_type) {
            (OptValueType::Boolean, OptValueType::Real) => {
                let one = self.push_value(OptValueType::Real, OptValueKind::RealConstant(1.0));
                let zero = self.push_value(OptValueType::Real, OptValueKind::RealConstant(0.0));
                self.push_value(
                    OptValueType::Real,
                    OptValueKind::Select {
                        condition: value,
                        then_value: one,
                        else_value: zero,
                    },
                )
            }
            (OptValueType::Real, OptValueType::Boolean) => {
                let zero = self.zero_real();
                self.push_value(
                    OptValueType::Boolean,
                    OptValueKind::Binary {
                        op: OptBinaryOp::Ne,
                        left: value,
                        right: zero,
                    },
                )
            }
            _ => value,
        }
    }

    fn push_typed_select(
        &mut self,
        value_type: OptValueType,
        condition: ValueId,
        then_value: ValueId,
        else_value: ValueId,
    ) -> ValueId {
        let then_value = self.coerce_value_to_type(then_value, value_type);
        let else_value = self.coerce_value_to_type(else_value, value_type);
        self.push_value(
            value_type,
            OptValueKind::Select {
                condition,
                then_value,
                else_value,
            },
        )
    }

    fn variable_identifier(&self, expr: ExprId) -> Option<VariableId> {
        let expression = self.mir.expressions.get(usize::from(expr))?;
        let HirExprKind::Identifier { name } = &expression.kind else {
            return None;
        };
        self.variable_ids_by_name.get(name).copied()
    }

    fn number_constant_expr(&self, expr: ExprId) -> Option<f64> {
        let expression = self.mir.expressions.get(usize::from(expr))?;
        let HirExprKind::Number { value, .. } = expression.kind else {
            return None;
        };
        Some(value)
    }

    fn expr_references_any_variable(
        &mut self,
        expr: ExprId,
        variables: &HashSet<VariableId>,
    ) -> bool {
        if variables.len() == 1
            && let Some(variable) = variables.iter().next().copied()
        {
            return self.expr_references_variable(expr, variable);
        }
        self.expr_references_any_variable_uncached(expr, variables)
    }

    fn expr_references_any_variable_uncached(
        &self,
        expr: ExprId,
        variables: &HashSet<VariableId>,
    ) -> bool {
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
                    self.local_assignment_history.remove(&assignment.target);
                    self.guarded_path_assignment_exprs
                        .remove(&assignment.target);
                }
                HirStatement::Loop(loop_statement) => {
                    self.mark_statement_assignments_unknown(&loop_statement.body);
                }
            }
        }
    }

    fn trace_lower_failure(&self) -> bool {
        if !opt_lower_failure_trace_enabled() {
            return false;
        }
        let equation_filter = opt_lower_failure_trace_equation_filter();
        let assignment_filter = opt_lower_failure_trace_assignment_filter();
        if equation_filter.is_none() && assignment_filter.is_none() {
            return true;
        };
        if let Some(filter) = equation_filter
            && self
                .lower_failure_trace_equation
                .is_some_and(|equation| equation.index() as usize == filter)
        {
            return true;
        }
        if let Some(filter) = assignment_filter
            && let Some(variable) = self.lower_failure_trace_assignment
        {
            let name = self.variable_name(variable);
            return filter
                .split(',')
                .map(str::trim)
                .any(|filter| !filter.is_empty() && name.contains(filter));
        }
        false
    }

    fn lower_expression(&mut self, expr_id: ExprId) -> Option<ValueId> {
        let use_cache = self.conditional_path_stack.is_empty()
            && self.assignment_history_value_snapshot_stack.is_empty()
            && self.assignment_history_snapshot_stack.is_empty()
            && self.assignment_history_local_snapshot_stack.is_empty()
            && self.assignment_history_previous_value_stack.is_empty();
        if use_cache && let Some(value) = self.expression_values.get(&expr_id) {
            return *value;
        }

        if self.expression_lowering_depth >= MAX_SCALAR_EXPRESSION_LOWERING_DEPTH {
            if self.trace_lower_failure() {
                eprintln!(
                    "OptIR {}: lower expression rejected before lowering expr={} depth={} stack_cycle=false",
                    self.mir.module_name,
                    expr_id.index(),
                    self.expression_lowering_depth,
                );
            }
            return None;
        }
        if !self.expression_lowering_stack.insert(expr_id) {
            self.expression_lowering_stack_cycle_rejected = true;
            if self.trace_lower_failure() {
                eprintln!(
                    "OptIR {}: lower expression rejected before lowering expr={} depth={} stack_cycle=true",
                    self.mir.module_name,
                    expr_id.index(),
                    self.expression_lowering_depth,
                );
            }
            return None;
        }

        self.expression_lowering_depth += 1;
        let lowered = self.lower_expression_inner(expr_id);
        self.expression_lowering_depth -= 1;
        self.expression_lowering_stack.remove(&expr_id);
        if lowered.is_none()
            && self.trace_lower_failure()
            && let Some(expression) = self.mir.expressions.get(usize::from(expr_id))
        {
            let assignment = self
                .lower_failure_trace_assignment
                .map(|variable| self.trace_variable_label(variable));
            eprintln!(
                "OptIR {}: lower expression failed equation={:?} assignment={:?} expr={} depth={} path_depth={} branch_context={:?} kind={:?}",
                self.mir.module_name,
                self.lower_failure_trace_equation,
                assignment,
                expr_id.index(),
                self.expression_lowering_depth,
                self.conditional_path_stack.len(),
                self.branch_flow_context,
                expression.kind
            );
        }

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
        let previous_lower_failure_trace_equation = self.lower_failure_trace_equation;
        let previous_lowering_equation = self.lowering_equation;
        self.branch_flow_context = self.equation_branch_unknown(equation);
        self.lower_failure_trace_equation = Some(equation.id);
        self.lowering_equation = true;
        if equation.kind == MirEquationKind::Potential {
            self.potential_equation_state_operator_depth += 1;
        }
        let lowered = self.lower_equation_expression_inner(equation.expression.id);
        self.potential_equation_state_operator_depth = previous_state_operator_depth;
        self.branch_flow_context = previous_context;
        self.lower_failure_trace_equation = previous_lower_failure_trace_equation;
        self.lowering_equation = previous_lowering_equation;
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

    fn lower_with_stack_cycle_probe<T>(
        &mut self,
        lower: impl FnOnce(&mut Self) -> Option<T>,
    ) -> (Option<T>, bool) {
        let previous_stack_cycle_rejected = self.expression_lowering_stack_cycle_rejected;
        self.expression_lowering_stack_cycle_rejected = false;
        let lowered = lower(self);
        let stack_cycle_rejected = self.expression_lowering_stack_cycle_rejected;
        self.expression_lowering_stack_cycle_rejected =
            previous_stack_cycle_rejected || stack_cycle_rejected;
        (lowered, stack_cycle_rejected)
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
                OptBinaryOp::Mod => {
                    let left = left.trunc();
                    let right = right.trunc();
                    (right != 0.0)
                        .then(|| (OptValueType::Real, OptValueKind::RealConstant(left % right)))
                }
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
            OptValueKind::Binary {
                op: OptBinaryOp::Pow,
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
            OptValueKind::Binary {
                op: OptBinaryOp::Pow,
                right,
                ..
            } if self.is_real_constant(*right, 0.0) => {
                Some((OptValueType::Real, OptValueKind::RealConstant(1.0)))
            }
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
            OptBinaryOp::Mod
            | OptBinaryOp::Eq
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

        let parameter = self.parameter_ids_by_name.get(name).copied()?;

        Some(self.push_value(OptValueType::Real, OptValueKind::Parameter { parameter }))
    }

    fn lower_variable_identifier(&mut self, name: &SmolStr) -> Option<Option<ValueId>> {
        let variable = *self.variable_ids_by_name.get(name)?;
        if let Some(previous_value) = self.assignment_history_previous_value(variable) {
            return Some(Some(previous_value));
        }
        if let Some(value) = self.assignment_history_value_snapshot(variable) {
            return Some(Some(value));
        }
        if let Some(limit) = self.assignment_history_snapshot_limit(variable) {
            if let Some(value) = self.lower_recent_assignment_history_prefix(variable, limit) {
                return Some(Some(value));
            }
            return Some(self.lower_assignment_history_prefix(variable, limit));
        }
        if let Some(history) = self.assignment_history_local_snapshot(variable) {
            return Some(self.lower_local_assignment_history_snapshot(variable, &history));
        }
        if let Some(value) = self.variable_values.get(&variable).copied() {
            let has_assignment_history = self.variable_assignment_history.contains_key(&variable);
            let has_complete_assignment_history =
                self.track_assignment_history && has_assignment_history;
            let has_bounded_assignment_history = has_assignment_history
                && (self.track_assignment_history
                    || self
                        .selective_assignment_history_targets
                        .contains(&variable));
            let has_guarded_path_assignment =
                self.guarded_path_assignment_exprs.contains_key(&variable);
            if value.is_none()
                && name.starts_with("__guard")
                && let Some(guard) = self.lower_guard_alias_variable_identifier(variable)
            {
                return Some(Some(guard));
            }
            if value.is_none()
                && !self.conditional_path_stack.is_empty()
                && self.variable_assignment_exprs.contains_key(&variable)
                && let Some(contextual) =
                    self.lower_current_assignment_expr_for_current_path(variable)
            {
                return Some(Some(contextual));
            }
            if value.is_none()
                && has_bounded_assignment_history
                && !has_guarded_path_assignment
                && self.conditional_path_stack.len() >= MAX_SCALAR_CURRENT_PATH_HISTORY_DEPTH
                && let Some(history_value) =
                    self.lower_recent_assignment_history_without_current_path(variable)
            {
                return Some(Some(history_value));
            }
            if value.is_none()
                && (has_assignment_history || has_guarded_path_assignment)
                && let Some(contextual) = self.lower_conditional_path_variable_identifier(variable)
            {
                return Some(Some(contextual));
            }
            if value.is_none()
                && has_bounded_assignment_history
                && let Some(history_value) = self.lower_recent_assignment_history_value(variable)
            {
                return Some(Some(history_value));
            }
            if value.is_none()
                && let Some(contextual) = self.lower_contextual_variable_identifier(variable)
            {
                return Some(Some(contextual));
            }
            if value.is_none()
                && self.guard_alias_lowering_depth > 0
                && has_complete_assignment_history
                && let Some(guard_history) = self.lower_assignment_history_value(variable)
            {
                return Some(Some(guard_history));
            }
            if value.is_none()
                && has_bounded_assignment_history
                && let Some(history_value) =
                    self.lower_complementary_assignment_history_value(variable)
            {
                return Some(Some(history_value));
            }
            if value.is_none()
                && has_bounded_assignment_history
                && let Some(history_value) = self.lower_assignment_history_value(variable)
            {
                return Some(Some(history_value));
            }
            return Some(value);
        }

        Some(Some(self.default_variable_value(
            self.variable_value_type(variable)?,
        )?))
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
        if self.history_reconstruction_budget_exhausted {
            if !self.assignment_history_local_snapshot_stack.is_empty() {
                return None;
            }
            let cache_key = self.assignment_history_prefix_cache_key(variable, limit);
            return self
                .assignment_history_prefix_cache
                .get(&cache_key)
                .copied()
                .flatten();
        }
        if self.trace_history_variable(variable) {
            eprintln!(
                "OptIR {}: replay full history variable={} id={} entries={} steps={}",
                self.mir.module_name,
                self.variable_name(variable),
                variable.index(),
                limit,
                self.history_reconstruction_steps
            );
        }
        self.lower_assignment_history_prefix(variable, limit)
    }

    fn lower_assignment_history_prefix(
        &mut self,
        variable: VariableId,
        limit: usize,
    ) -> Option<ValueId> {
        let cache_key = self.assignment_history_prefix_cache_key(variable, limit);
        let cache_enabled = self.assignment_history_local_snapshot_stack.is_empty();
        if cache_enabled {
            if let Some(cached) = self
                .assignment_history_prefix_cache
                .get(&cache_key)
                .copied()
            {
                return cached;
            }
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
        if cache_enabled && (lowered.is_some() || !self.history_reconstruction_budget_exhausted) {
            self.assignment_history_prefix_cache
                .insert(cache_key, lowered);
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
            return self
                .lower_expression_with_assignment_snapshot(entry, entry.expr)
                .and_then(|value| self.coerce_value_to_variable_type(variable, value));
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

    fn assignment_history_local_snapshot(
        &self,
        variable: VariableId,
    ) -> Option<LocalAssignmentHistory> {
        self.assignment_history_local_snapshot_stack
            .iter()
            .rev()
            .find_map(|(candidate, history)| (*candidate == variable).then(|| history.clone()))
    }

    fn lower_local_assignment_history_snapshot(
        &mut self,
        variable: VariableId,
        history: &LocalAssignmentHistory,
    ) -> Option<ValueId> {
        if !self.variable_lowering_stack.insert(variable) {
            return None;
        }
        let lowered = self.lower_assignment_history_window_from_base(variable, history);
        self.variable_lowering_stack.remove(&variable);
        lowered
    }

    fn lower_assignment_history_window_from_base(
        &mut self,
        variable: VariableId,
        history: &LocalAssignmentHistory,
    ) -> Option<ValueId> {
        let Some(mut value) = history.base_value else {
            return self.lower_assignment_history_window_without_base(variable, history);
        };
        if history.entries.is_empty() {
            return self.coerce_value_to_variable_type(variable, value);
        }
        let cheap_replay = history.entries.len() <= MAX_SCALAR_CHEAP_CURRENT_PATH_HISTORY_ENTRIES;
        if !cheap_replay {
            self.consume_history_reconstruction_step()?;
        }
        for entry in &history.entries {
            let next = if self.conditional_path_stack.is_empty() {
                self.lower_assignment_history_expr_from_previous_with_budget(
                    variable,
                    entry,
                    value,
                    !cheap_replay,
                )
            } else {
                self.lower_assignment_history_expr_from_previous_for_current_path(
                    variable,
                    entry,
                    value,
                    history.entries.len(),
                )
            }?;
            value = self.coerce_value_to_variable_type(variable, next)?;
        }
        Some(value)
    }

    fn lower_assignment_history_window_without_base(
        &mut self,
        variable: VariableId,
        history: &LocalAssignmentHistory,
    ) -> Option<ValueId> {
        if history.entries.is_empty() {
            return None;
        }
        if !self.conditional_path_stack.is_empty()
            && let Some(value) =
                self.lower_assignment_history_entries_for_current_path(variable, &history.entries)
        {
            return self.coerce_value_to_variable_type(variable, value);
        }
        self.lower_complementary_assignment_history_entries_value(variable, &history.entries)
            .or_else(|| {
                self.lower_recent_assignment_history_entries_value(variable, &history.entries)
            })
            .and_then(|value| self.coerce_value_to_variable_type(variable, value))
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
        self.assignment_history_local_snapshot_stack
            .extend_from_slice(&entry.local_snapshots);
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
        let retained_local = self
            .assignment_history_local_snapshot_stack
            .len()
            .saturating_sub(entry.local_snapshots.len());
        self.assignment_history_local_snapshot_stack
            .truncate(retained_local);
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
        self.assignment_history_local_snapshot_stack
            .extend_from_slice(&entry.local_snapshots);
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
        let retained_local = self
            .assignment_history_local_snapshot_stack
            .len()
            .saturating_sub(entry.local_snapshots.len());
        self.assignment_history_local_snapshot_stack
            .truncate(retained_local);
        lowered
    }

    fn lower_expression_with_assignment_previous_value_snapshot_and_path(
        &mut self,
        variable: VariableId,
        previous_value: ValueId,
        entry: &AssignmentHistoryEntry,
        condition: ExprId,
        truth: bool,
        expr: ExprId,
    ) -> Option<ValueId> {
        if self
            .condition_truth_in_current_path(condition)
            .is_some_and(|known| known == truth)
        {
            return self.lower_expression_with_assignment_previous_value_and_snapshot(
                variable,
                previous_value,
                entry,
                expr,
            );
        }
        if self.conditional_path_stack.len() >= MAX_SCALAR_CURRENT_PATH_HISTORY_DEPTH {
            return None;
        }
        self.conditional_path_stack
            .push(ConditionalPathPredicate { condition, truth });
        let lowered = self.lower_expression_with_assignment_previous_value_and_snapshot(
            variable,
            previous_value,
            entry,
            expr,
        );
        self.conditional_path_stack.pop();
        lowered
    }

    fn assignment_history_entry(
        &mut self,
        target: VariableId,
        expr: ExprId,
    ) -> AssignmentHistoryEntry {
        self.assignment_history_entry_inner(target, expr, true)
    }

    fn local_assignment_history_entry(
        &mut self,
        target: VariableId,
        expr: ExprId,
    ) -> AssignmentHistoryEntry {
        self.assignment_history_entry_inner(target, expr, false)
    }

    fn assignment_history_entry_inner(
        &mut self,
        target: VariableId,
        expr: ExprId,
        include_local_snapshots: bool,
    ) -> AssignmentHistoryEntry {
        let variables = self.assignment_history_snapshot_variables(target, expr);
        let trace_variables = variables.clone();
        let mut value_snapshots = Vec::new();
        let mut snapshots = Vec::new();
        let mut unresolved_local_variables = BTreeSet::new();
        for variable in variables {
            if let Some(Some(value)) = self.variable_values.get(&variable).copied() {
                value_snapshots.push((variable, value));
            } else if let Some(history) = self.variable_assignment_history.get(&variable) {
                snapshots.push((variable, history.len()));
            } else if self.local_assignment_history.contains_key(&variable) {
                if include_local_snapshots
                    && let Some(value) =
                        self.lower_local_assignment_history_snapshot_for_entry(variable)
                {
                    value_snapshots.push((variable, value));
                } else {
                    unresolved_local_variables.insert(variable);
                }
            }
        }
        let local_dependencies: Vec<_> = unresolved_local_variables.iter().copied().collect();
        let local_snapshots = if include_local_snapshots {
            self.local_assignment_history_snapshots(unresolved_local_variables)
                .unwrap_or_default()
        } else {
            Vec::new()
        };
        value_snapshots.sort_by_key(|(variable, _)| usize::from(*variable));
        snapshots.sort_by_key(|(variable, _)| usize::from(*variable));
        let entry = AssignmentHistoryEntry {
            expr,
            value_snapshots,
            snapshots,
            local_dependencies,
            local_snapshots,
        };
        self.trace_assignment_snapshot(target, expr, &trace_variables, &entry);
        entry
    }

    fn lower_local_assignment_history_snapshot_for_entry(
        &mut self,
        variable: VariableId,
    ) -> Option<ValueId> {
        let history = self.local_assignment_history.get(&variable)?.clone();
        let original_expression_values = std::mem::take(&mut self.expression_values);
        let lowered = self.lower_local_assignment_history_snapshot(variable, &history);
        self.expression_values = original_expression_values;
        lowered
    }

    fn local_assignment_history_snapshots(
        &mut self,
        variables: BTreeSet<VariableId>,
    ) -> Option<Vec<(VariableId, LocalAssignmentHistory)>> {
        if variables.is_empty() {
            return Some(Vec::new());
        }

        let mut snapshots = Vec::new();
        let mut seen = HashSet::new();
        let mut pending: VecDeque<_> = variables
            .into_iter()
            .map(|variable| (variable, 0usize))
            .collect();
        let mut total_entries = 0usize;
        while let Some((variable, depth)) = pending.pop_front() {
            if !seen.insert(variable) {
                continue;
            }
            let required = depth <= REQUIRED_SCALAR_LOCAL_ASSIGNMENT_SNAPSHOT_DEPENDENCY_DEPTH;
            if snapshots.len() >= MAX_SCALAR_LOCAL_ASSIGNMENT_SNAPSHOT_VARIABLES {
                if !required {
                    continue;
                }
                self.trace_local_assignment_history_snapshot_reject(
                    seen.iter().copied(),
                    "variable limit",
                );
                return None;
            }
            let Some(history) = self.local_assignment_history.get(&variable).cloned() else {
                continue;
            };
            if history.base_value.is_none() && history.entries.is_empty() {
                continue;
            }
            let Some(next_total_entries) = total_entries.checked_add(history.len()) else {
                if !required {
                    continue;
                }
                self.trace_local_assignment_history_snapshot_reject(
                    seen.iter().copied(),
                    "entry count overflow",
                );
                return None;
            };
            if next_total_entries > MAX_SCALAR_LOCAL_ASSIGNMENT_SNAPSHOT_TOTAL_ENTRIES {
                if !required {
                    continue;
                }
                self.trace_local_assignment_history_snapshot_reject(
                    seen.iter().copied(),
                    "entry limit",
                );
                return None;
            }
            total_entries = next_total_entries;
            if depth < MAX_SCALAR_LOCAL_ASSIGNMENT_SNAPSHOT_DEPENDENCY_DEPTH {
                for entry in &history.entries {
                    for dependency in &entry.local_dependencies {
                        if *dependency != variable
                            && self.local_assignment_history.contains_key(dependency)
                        {
                            pending.push_back((*dependency, depth + 1));
                        }
                    }
                }
            }
            snapshots.push((variable, history));
        }
        snapshots.sort_by_key(|(variable, _)| usize::from(*variable));
        Some(snapshots)
    }

    fn assignment_history_snapshot_variables(
        &mut self,
        target: VariableId,
        expr: ExprId,
    ) -> BTreeSet<VariableId> {
        if let Some(update) = self.conditional_self_update(target, expr)
            && !self.expr_references_variable(update.value_expr, target)
            && let Some(mut variables) =
                self.assignment_history_alias_variables(target, update.value_expr)
        {
            if (self.expanded_assignment_history_snapshots
                || self.selective_assignment_history_targets.contains(&target)
                || self.local_assignment_history_targets.contains(&target))
                && let Some(condition_variables) =
                    self.assignment_history_alias_variables(target, update.condition)
            {
                variables.extend(condition_variables);
            }
            return self
                .simple_assignment_alias_snapshot_variables_from_set(variables)
                .unwrap_or_default();
        }
        if let Some(variable) = self.variable_identifier(expr) {
            return if variable != target {
                BTreeSet::from([variable])
            } else {
                Default::default()
            };
        }
        if self.expr_references_variable(expr, target)
            && let Some(variables) = self.self_assignment_alias_snapshot_variables(target, expr)
        {
            return variables;
        }
        self.assignment_history_alias_variables(target, expr)
            .and_then(|variables| {
                self.simple_assignment_alias_snapshot_variables_from_set(variables)
            })
            .unwrap_or_default()
    }

    fn self_assignment_alias_snapshot_variables(
        &mut self,
        target: VariableId,
        expr: ExprId,
    ) -> Option<BTreeSet<VariableId>> {
        let mut variables = self.assignment_history_alias_variables(target, expr)?;
        variables.remove(&target);
        self.simple_assignment_alias_snapshot_variables_from_set(variables)
    }

    fn assignment_history_alias_variables(
        &self,
        target: VariableId,
        expr: ExprId,
    ) -> Option<BTreeSet<VariableId>> {
        let allow_calls = self.expanded_assignment_history_snapshots
            || self.selective_assignment_history_targets.contains(&target);
        let max_nodes = if self.selective_assignment_history_targets.contains(&target)
            && !self.expanded_assignment_history_snapshots
        {
            MAX_SELECTIVE_ASSIGNMENT_ALIAS_SNAPSHOT_EXPR_NODES
        } else {
            MAX_SCALAR_ASSIGNMENT_ALIAS_SNAPSHOT_EXPR_NODES
        };
        self.simple_assignment_alias_variables(expr, allow_calls, max_nodes)
    }

    fn simple_assignment_alias_snapshot_variables_from_set(
        &self,
        variables: BTreeSet<VariableId>,
    ) -> Option<BTreeSet<VariableId>> {
        if variables.len() > MAX_SCALAR_ASSIGNMENT_ALIAS_SNAPSHOT_VARIABLES {
            return None;
        }
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
        let max_history_backed_unresolved = if self.expanded_assignment_history_snapshots {
            MAX_SCALAR_ASSIGNMENT_ALIAS_SNAPSHOT_VARIABLES
        } else {
            1
        };
        (history_backed_unresolved <= max_history_backed_unresolved).then_some(variables)
    }

    fn simple_assignment_alias_variables(
        &self,
        expr: ExprId,
        allow_calls: bool,
        max_nodes: usize,
    ) -> Option<BTreeSet<VariableId>> {
        let mut variables = BTreeSet::new();
        let mut visited = HashSet::new();
        let mut stack = vec![expr];
        let mut nodes = 0;
        while let Some(expr) = stack.pop() {
            if !visited.insert(expr) {
                continue;
            }
            nodes += 1;
            if nodes > max_nodes {
                return None;
            }
            let expression = self.mir.expressions.get(usize::from(expr))?;
            match &expression.kind {
                HirExprKind::Identifier { .. } => {
                    if let Some(variable) = self.variable_identifier(expr)
                        && (self.variable_values.contains_key(&variable)
                            || self.variable_assignment_history.contains_key(&variable)
                            || self.local_assignment_history.contains_key(&variable))
                    {
                        variables.insert(variable);
                    }
                }
                HirExprKind::Number { .. } => {}
                HirExprKind::BranchAccess { .. } | HirExprKind::NamedBranchAccess { .. } => {}
                HirExprKind::Unary { op, operand }
                    if simple_assignment_alias_snapshot_unary_op(op, true) =>
                {
                    stack.push(*operand);
                }
                HirExprKind::Binary { op, left, right }
                    if simple_assignment_alias_snapshot_binary_op(op, true) =>
                {
                    stack.extend([*left, *right]);
                }
                HirExprKind::Conditional {
                    condition,
                    then_expr,
                    else_expr,
                } => {
                    stack.extend([*condition, *then_expr, *else_expr]);
                }
                HirExprKind::Call { name, args }
                    if allow_calls && simple_assignment_alias_snapshot_call(name, args.len()) =>
                {
                    stack.extend(args.iter().copied());
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
        if self.trace_history_variable(variable) {
            eprintln!(
                "OptIR {}: replay complementary history variable={} id={} entries={} steps={}",
                self.mir.module_name,
                self.variable_name(variable),
                variable.index(),
                limit,
                self.history_reconstruction_steps
            );
        }
        let entry = self
            .variable_assignment_history
            .get(&variable)?
            .get(limit.checked_sub(1)?)?
            .clone();
        let update = self.conditional_self_update(variable, entry.expr)?;
        self.lower_complementary_self_assignment_pair(variable, limit, &entry, update)
    }

    fn lower_complementary_assignment_history_entries_value(
        &mut self,
        variable: VariableId,
        history: &[AssignmentHistoryEntry],
    ) -> Option<ValueId> {
        let limit = history.len();
        let current_entry = history.get(limit.checked_sub(1)?)?.clone();
        let update = self.conditional_self_update(variable, current_entry.expr)?;
        self.lower_complementary_self_assignment_pair_from_history(
            variable,
            history,
            limit,
            &current_entry,
            update,
        )
    }

    fn lower_complementary_self_assignment_pair(
        &mut self,
        variable: VariableId,
        limit: usize,
        current_entry: &AssignmentHistoryEntry,
        current: ConditionalSelfUpdate,
    ) -> Option<ValueId> {
        let history = self.variable_assignment_history.get(&variable)?.clone();
        self.lower_complementary_self_assignment_pair_from_history(
            variable,
            &history,
            limit,
            current_entry,
            current,
        )
    }

    fn lower_complementary_self_assignment_pair_from_history(
        &mut self,
        variable: VariableId,
        history: &[AssignmentHistoryEntry],
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

        let previous_entry = history.get(limit - 2)?.clone();
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
        Some(self.push_typed_select(value_type, condition, then_value, else_value))
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
        let cache_enabled = self.assignment_history_local_snapshot_stack.is_empty();
        if cache_enabled {
            if let Some(cached) = self
                .recent_assignment_history_cache
                .get(&cache_key)
                .copied()
            {
                return cached;
            }
        }
        if self.history_reconstruction_budget_exhausted && !self.allow_exhausted_history_replay() {
            return None;
        }
        let entries = self
            .variable_assignment_history
            .get(&variable)
            .map(Vec::len)?;
        if self.history_reconstruction_budget_exhausted
            && entries > MAX_SCALAR_EXHAUSTED_HISTORY_REPLAY_ENTRIES
        {
            return None;
        }
        if self.trace_history_variable(variable) {
            eprintln!(
                "OptIR {}: replay recent history variable={} id={} entries={} steps={}",
                self.mir.module_name,
                self.variable_name(variable),
                variable.index(),
                entries,
                self.history_reconstruction_steps
            );
        }
        let lowered = self.lower_recent_assignment_history_value_uncached(variable);
        if cache_enabled
            && (lowered.is_some()
                || !self.history_reconstruction_budget_exhausted
                || (self.allow_exhausted_history_replay()
                    && entries <= MAX_SCALAR_EXHAUSTED_HISTORY_REPLAY_ENTRIES))
        {
            self.recent_assignment_history_cache
                .insert(cache_key, lowered);
        }
        lowered
    }

    fn lower_recent_assignment_history_without_current_path(
        &mut self,
        variable: VariableId,
    ) -> Option<ValueId> {
        if self.conditional_path_stack.is_empty() {
            return None;
        }
        let original_path = std::mem::take(&mut self.conditional_path_stack);
        let original_expression_values = std::mem::take(&mut self.expression_values);
        let lowered = self.lower_recent_assignment_history_value(variable);
        self.expression_values = original_expression_values;
        self.conditional_path_stack = original_path;
        lowered
    }

    fn lower_recent_assignment_history_value_uncached(
        &mut self,
        variable: VariableId,
    ) -> Option<ValueId> {
        let history = self.variable_assignment_history.get(&variable)?.clone();
        self.lower_recent_assignment_history_entries_value(variable, &history)
    }

    fn lower_recent_assignment_history_entries_value(
        &mut self,
        variable: VariableId,
        history: &[AssignmentHistoryEntry],
    ) -> Option<ValueId> {
        let cheap_replay = history.len() <= MAX_SCALAR_CHEAP_CURRENT_PATH_HISTORY_ENTRIES;
        if !cheap_replay {
            self.consume_history_reconstruction_step()?;
        }
        let scan_start = history
            .len()
            .saturating_sub(MAX_SCALAR_RECENT_HISTORY_RECONSTRUCTION_ENTRIES);
        for base_index in (scan_start..history.len()).rev() {
            let base_entry = &history[base_index];
            if self.expr_references_variable(base_entry.expr, variable) {
                continue;
            }
            let Some(mut value) = self
                .lower_expression_with_assignment_snapshot(base_entry, base_entry.expr)
                .and_then(|value| self.coerce_value_to_variable_type(variable, value))
            else {
                continue;
            };
            for entry in history.iter().skip(base_index + 1) {
                let Some(next) = self.lower_assignment_history_expr_from_previous_with_budget(
                    variable,
                    entry,
                    value,
                    !cheap_replay,
                ) else {
                    return None;
                };
                value = next;
            }
            return Some(value);
        }
        None
    }

    fn lower_recent_assignment_history_prefix(
        &mut self,
        variable: VariableId,
        limit: usize,
    ) -> Option<ValueId> {
        let cache_key = self.assignment_history_prefix_cache_key(variable, limit);
        let cache_enabled = self.assignment_history_local_snapshot_stack.is_empty();
        if cache_enabled {
            if let Some(cached) = self
                .assignment_history_prefix_cache
                .get(&cache_key)
                .copied()
            {
                return cached;
            }
        }
        let (lowered, stack_cycle_rejected) = self.lower_with_stack_cycle_probe(|this| {
            this.lower_recent_assignment_history_prefix_uncached(variable, limit)
        });
        if cache_enabled
            && (lowered.is_some()
                || (!stack_cycle_rejected && !self.history_reconstruction_budget_exhausted))
        {
            self.assignment_history_prefix_cache
                .insert(cache_key, lowered);
        }
        lowered
    }

    fn lower_recent_assignment_history_prefix_uncached(
        &mut self,
        variable: VariableId,
        limit: usize,
    ) -> Option<ValueId> {
        let history = self.variable_assignment_history.get(&variable)?.clone();
        let limit = limit.min(history.len());
        if limit == 0 {
            return self.default_variable_value(self.variable_value_type(variable)?);
        }
        if self.trace_history_variable(variable) {
            eprintln!(
                "OptIR {}: replay recent prefix variable={} id={} limit={} entries={} steps={}",
                self.mir.module_name,
                self.variable_name(variable),
                variable.index(),
                limit,
                history.len(),
                self.history_reconstruction_steps
            );
        }
        let scan_start = limit.saturating_sub(MAX_SCALAR_RECENT_HISTORY_RECONSTRUCTION_ENTRIES);
        for base_index in (scan_start..limit).rev() {
            let base_entry = &history[base_index];
            if self.expr_references_variable(base_entry.expr, variable) {
                continue;
            }
            let replay_len = limit - base_index;
            let cheap_replay = replay_len <= MAX_SCALAR_CHEAP_CURRENT_PATH_HISTORY_ENTRIES;
            if !cheap_replay {
                self.consume_history_reconstruction_step()?;
            }
            let Some(mut value) = self
                .lower_expression_with_assignment_snapshot(base_entry, base_entry.expr)
                .and_then(|value| self.coerce_value_to_variable_type(variable, value))
            else {
                continue;
            };
            let mut complete = true;
            for entry in history.iter().take(limit).skip(base_index + 1) {
                let Some(next) = self.lower_assignment_history_expr_from_previous_with_budget(
                    variable,
                    entry,
                    value,
                    !cheap_replay,
                ) else {
                    complete = false;
                    break;
                };
                value = next;
            }
            if complete {
                return Some(value);
            }
        }
        None
    }

    fn lower_assignment_history_expr_from_previous(
        &mut self,
        variable: VariableId,
        entry: &AssignmentHistoryEntry,
        previous_value: ValueId,
    ) -> Option<ValueId> {
        self.lower_assignment_history_expr_from_previous_with_budget(
            variable,
            entry,
            previous_value,
            true,
        )
    }

    fn lower_assignment_history_expr_from_previous_with_budget(
        &mut self,
        variable: VariableId,
        entry: &AssignmentHistoryEntry,
        previous_value: ValueId,
        charge_budget: bool,
    ) -> Option<ValueId> {
        if charge_budget {
            self.consume_history_reconstruction_step()?;
        }
        let expression = self.mir.expressions.get(usize::from(entry.expr))?;
        let HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } = &expression.kind
        else {
            return self
                .lower_expression_with_assignment_previous_value_and_snapshot(
                    variable,
                    previous_value,
                    entry,
                    entry.expr,
                )
                .and_then(|value| self.coerce_value_to_variable_type(variable, value));
        };

        let then_lowered = if self.variable_identifier(*then_expr) == Some(variable) {
            Some(previous_value)
        } else {
            self.lower_expression_with_assignment_previous_value_snapshot_and_path(
                variable,
                previous_value,
                entry,
                *condition,
                true,
                *then_expr,
            )
        };
        let then_value = then_lowered?;

        let else_lowered = if self.variable_identifier(*else_expr) == Some(variable) {
            Some(previous_value)
        } else {
            self.lower_expression_with_assignment_previous_value_snapshot_and_path(
                variable,
                previous_value,
                entry,
                *condition,
                false,
                *else_expr,
            )
        };
        let else_value = else_lowered?;

        if then_value == else_value {
            return self.coerce_value_to_variable_type(variable, then_value);
        }
        let condition = self.lower_expression_with_assignment_previous_value_and_snapshot(
            variable,
            previous_value,
            entry,
            *condition,
        )?;
        let value_type = self.variable_opt_value_type(variable)?;
        Some(self.push_typed_select(value_type, condition, then_value, else_value))
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

    fn expr_references_variable(&mut self, expr: ExprId, variable: VariableId) -> bool {
        if let Some(references) = self
            .expr_variable_reference_cache
            .get(&(expr, variable))
            .copied()
        {
            return references;
        }
        let references =
            self.expr_references_any_variable_uncached(expr, &HashSet::from([variable]));
        self.expr_variable_reference_cache
            .insert((expr, variable), references);
        references
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

    fn lower_current_assignment_expr_for_current_path(
        &mut self,
        variable: VariableId,
    ) -> Option<ValueId> {
        if self.conditional_path_stack.is_empty() || !self.variable_lowering_stack.insert(variable)
        {
            return None;
        }
        let expr = self.variable_assignment_exprs.get(&variable).copied();
        let lowered = expr
            .and_then(|expr| {
                self.lower_current_assignment_expr_for_current_path_inner(variable, expr)
            })
            .and_then(|value| self.coerce_value_to_variable_type(variable, value));
        self.variable_lowering_stack.remove(&variable);
        lowered
    }

    fn lower_current_assignment_expr_for_current_path_inner(
        &mut self,
        variable: VariableId,
        expr: ExprId,
    ) -> Option<ValueId> {
        let expression = self.mir.expressions.get(usize::from(expr))?;
        let HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } = &expression.kind
        else {
            if self.expr_references_variable(expr, variable) {
                return None;
            }
            return self.lower_expression(expr);
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
        self.lower_expression(selected)
    }

    fn lower_assignment_history_for_current_path(
        &mut self,
        variable: VariableId,
    ) -> Option<ValueId> {
        if self.conditional_path_stack.len() > MAX_SCALAR_CURRENT_PATH_HISTORY_DEPTH {
            return None;
        }
        let cache_key = self.current_path_history_cache_key(variable);
        let cache_enabled = self.assignment_history_local_snapshot_stack.is_empty();
        if cache_enabled {
            if let Some(cached) = self.current_path_history_cache.get(&cache_key).copied() {
                return cached;
            }
        }
        if self.history_reconstruction_budget_exhausted && !self.allow_exhausted_history_replay() {
            return None;
        }
        let history = self.variable_assignment_history.get(&variable)?.clone();
        if self.history_reconstruction_budget_exhausted
            && !self.exhausted_budget_current_path_history_replay(history.len())
        {
            return None;
        }
        if self.trace_history_variable(variable) {
            eprintln!(
                "OptIR {}: replay current-path history variable={} id={} entries={} path={} steps={}",
                self.mir.module_name,
                self.variable_name(variable),
                variable.index(),
                history.len(),
                self.conditional_path_stack.len(),
                self.history_reconstruction_steps
            );
        }
        let lowered = self.lower_assignment_history_entries_for_current_path(variable, &history);
        if cache_enabled
            && (lowered.is_some()
                || !self.history_reconstruction_budget_exhausted
                || (self.allow_exhausted_history_replay()
                    && self.exhausted_budget_current_path_history_replay(history.len())))
        {
            self.current_path_history_cache.insert(cache_key, lowered);
        }
        lowered
    }

    fn lower_assignment_history_entries_for_current_path(
        &mut self,
        variable: VariableId,
        history: &[AssignmentHistoryEntry],
    ) -> Option<ValueId> {
        if history.is_empty()
            || self.conditional_path_stack.len() > MAX_SCALAR_CURRENT_PATH_HISTORY_DEPTH
        {
            return None;
        }
        if self.history_reconstruction_budget_exhausted
            && !self.exhausted_budget_current_path_history_replay(history.len())
        {
            return None;
        }
        self.consume_current_path_history_reconstruction_step(history.len())?;
        let known = self.lower_known_assignment_history_for_current_path(variable, history);
        if known.crossed_unknown_guard {
            self.lower_covering_self_update_cascade_for_current_path(variable, history)
                .or_else(|| {
                    self.lower_complementary_assignment_history_pair_for_current_path(
                        variable, history,
                    )
                })
                .or_else(|| {
                    self.lower_recent_assignment_history_for_current_path(variable, history)
                })
                .or(known.value)
        } else {
            known
                .value
                .or_else(|| {
                    self.lower_covering_self_update_cascade_for_current_path(variable, history)
                })
                .or_else(|| {
                    self.lower_complementary_assignment_history_pair_for_current_path(
                        variable, history,
                    )
                })
                .or_else(|| {
                    self.lower_recent_assignment_history_for_current_path(variable, history)
                })
        }
    }

    fn consume_history_reconstruction_step(&mut self) -> Option<()> {
        if self.history_reconstruction_steps >= self.history_reconstruction_step_limit {
            self.history_reconstruction_budget_exhausted = true;
            if history_trace_enabled() && history_trace_variable_filter().is_none() {
                eprintln!(
                    "OptIR {}: history replay budget exhausted at {} steps (limit {})",
                    self.mir.module_name,
                    self.history_reconstruction_steps,
                    self.history_reconstruction_step_limit
                );
            }
            return None;
        }
        self.history_reconstruction_steps += 1;
        Some(())
    }

    fn consume_current_path_history_reconstruction_step(&mut self, entries: usize) -> Option<()> {
        if self.cheap_current_path_history_replay(entries) {
            Some(())
        } else {
            self.consume_history_reconstruction_step()
        }
    }

    fn consume_current_path_assignment_history_replay_step(
        &mut self,
        entries: usize,
    ) -> Option<()> {
        if self.history_reconstruction_budget_exhausted
            && self.exhausted_budget_current_path_history_replay(entries)
        {
            Some(())
        } else {
            self.consume_history_reconstruction_step()
        }
    }

    fn cheap_current_path_history_replay(&self, entries: usize) -> bool {
        entries <= MAX_SCALAR_CHEAP_CURRENT_PATH_HISTORY_ENTRIES
            && self.conditional_path_stack.len() <= MAX_SCALAR_CHEAP_CURRENT_PATH_HISTORY_DEPTH
    }

    fn exhausted_budget_current_path_history_replay(&self, entries: usize) -> bool {
        entries <= MAX_SCALAR_EXHAUSTED_HISTORY_REPLAY_ENTRIES
            && self.conditional_path_stack.len() <= MAX_SCALAR_CHEAP_CURRENT_PATH_HISTORY_DEPTH
    }

    fn allow_exhausted_history_replay(&self) -> bool {
        self.lowering_equation
    }

    fn reset_history_reconstruction_budget(&mut self) {
        self.history_reconstruction_steps = 0;
        self.history_reconstruction_budget_exhausted = false;
    }

    fn reset_history_reconstruction_budget_for_equations(&mut self) {
        if self.history_reconstruction_budget_exhausted
            && self.history_reconstruction_step_limit < MAX_SCALAR_HISTORY_RECONSTRUCTION_STEPS
        {
            return;
        }
        self.reset_history_reconstruction_budget();
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
        match self.lower_assignment_expr_for_current_path(variable, &assignment.value)? {
            CurrentPathAssignmentEffect::Assign(value) => Some(value),
            CurrentPathAssignmentEffect::KeepPrevious => None,
        }
    }

    fn lower_known_assignment_history_for_current_path(
        &mut self,
        variable: VariableId,
        history: &[AssignmentHistoryEntry],
    ) -> CurrentPathAssignmentScan {
        let mut crossed_unknown_guard = false;
        for entry in history
            .iter()
            .rev()
            .take(MAX_SCALAR_CURRENT_PATH_HISTORY_SCAN_ENTRIES)
        {
            match self.lower_assignment_expr_for_current_path(variable, entry) {
                Some(CurrentPathAssignmentEffect::Assign(value)) => {
                    return CurrentPathAssignmentScan {
                        value: Some(value),
                        crossed_unknown_guard,
                    };
                }
                Some(CurrentPathAssignmentEffect::KeepPrevious) => continue,
                None => crossed_unknown_guard = true,
            }
        }
        CurrentPathAssignmentScan {
            value: None,
            crossed_unknown_guard,
        }
    }

    fn lower_assignment_expr_for_current_path(
        &mut self,
        variable: VariableId,
        entry: &AssignmentHistoryEntry,
    ) -> Option<CurrentPathAssignmentEffect> {
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
            return self
                .lower_expression_with_assignment_snapshot(entry, entry.expr)
                .and_then(|value| self.coerce_value_to_variable_type(variable, value))
                .map(CurrentPathAssignmentEffect::Assign);
        };
        let condition_truth = self.condition_truth_in_current_path(*condition)?;
        let selected = if condition_truth {
            *then_expr
        } else {
            *else_expr
        };
        if self.variable_identifier(selected) == Some(variable) {
            return Some(CurrentPathAssignmentEffect::KeepPrevious);
        }
        self.lower_expression_with_assignment_snapshot_and_path(
            entry,
            *condition,
            condition_truth,
            selected,
        )
        .and_then(|value| self.coerce_value_to_variable_type(variable, value))
        .map(CurrentPathAssignmentEffect::Assign)
    }

    fn lower_expression_with_assignment_snapshot_and_path(
        &mut self,
        entry: &AssignmentHistoryEntry,
        condition: ExprId,
        truth: bool,
        expr: ExprId,
    ) -> Option<ValueId> {
        if self
            .condition_truth_in_current_path(condition)
            .is_some_and(|known| known == truth)
        {
            return self.lower_expression_with_assignment_snapshot(entry, expr);
        }
        if self.conditional_path_stack.len() >= MAX_SCALAR_CURRENT_PATH_HISTORY_DEPTH {
            return None;
        }
        self.conditional_path_stack
            .push(ConditionalPathPredicate { condition, truth });
        let lowered = self.lower_expression_with_assignment_snapshot(entry, expr);
        self.conditional_path_stack.pop();
        lowered
    }

    fn lower_recent_assignment_history_for_current_path(
        &mut self,
        variable: VariableId,
        history: &[AssignmentHistoryEntry],
    ) -> Option<ValueId> {
        if self.conditional_path_stack.is_empty()
            || self.conditional_path_stack.len() > MAX_SCALAR_CURRENT_PATH_HISTORY_DEPTH
            || (self.history_reconstruction_budget_exhausted
                && !self.exhausted_budget_current_path_history_replay(history.len()))
        {
            return None;
        }
        self.consume_current_path_history_reconstruction_step(history.len())?;
        let scan_start = history
            .len()
            .saturating_sub(MAX_SCALAR_RECENT_HISTORY_RECONSTRUCTION_ENTRIES);
        for base_index in (scan_start..history.len()).rev() {
            let base_entry = &history[base_index];
            if self.expr_references_variable(base_entry.expr, variable) {
                continue;
            }
            let Some(mut value) = self
                .lower_expression_with_assignment_snapshot(base_entry, base_entry.expr)
                .and_then(|value| self.coerce_value_to_variable_type(variable, value))
            else {
                continue;
            };
            let mut complete = true;
            for entry in history.iter().skip(base_index + 1) {
                let Some(next) = self.lower_assignment_history_expr_from_previous_for_current_path(
                    variable,
                    entry,
                    value,
                    history.len(),
                ) else {
                    complete = false;
                    break;
                };
                value = next;
            }
            if complete {
                return Some(value);
            }
        }
        None
    }

    fn lower_assignment_history_expr_from_previous_for_current_path(
        &mut self,
        variable: VariableId,
        entry: &AssignmentHistoryEntry,
        previous_value: ValueId,
        history_len: usize,
    ) -> Option<ValueId> {
        self.consume_current_path_assignment_history_replay_step(history_len)?;
        let expression = self.mir.expressions.get(usize::from(entry.expr))?;
        let HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } = &expression.kind
        else {
            return self
                .lower_expression_with_assignment_previous_value_and_snapshot(
                    variable,
                    previous_value,
                    entry,
                    entry.expr,
                )
                .and_then(|value| self.coerce_value_to_variable_type(variable, value));
        };

        let Some(condition_truth) = self.condition_truth_in_current_path(*condition) else {
            return self.lower_assignment_history_expr_from_previous(
                variable,
                entry,
                previous_value,
            );
        };
        let selected = if condition_truth {
            *then_expr
        } else {
            *else_expr
        };
        if self.variable_identifier(selected) == Some(variable) {
            return Some(previous_value);
        }
        self.lower_expression_with_assignment_previous_value_snapshot_and_path(
            variable,
            previous_value,
            entry,
            *condition,
            condition_truth,
            selected,
        )
        .and_then(|value| self.coerce_value_to_variable_type(variable, value))
    }

    fn lower_covering_self_update_cascade_for_current_path(
        &mut self,
        variable: VariableId,
        history: &[AssignmentHistoryEntry],
    ) -> Option<ValueId> {
        if self.history_reconstruction_budget_exhausted
            && !self.exhausted_budget_current_path_history_replay(history.len())
        {
            return None;
        }
        if self.conditional_path_stack.len() > MAX_SCALAR_CURRENT_PATH_HISTORY_DEPTH
            || history.len() < 2
        {
            return None;
        }

        let scan_start = history
            .len()
            .saturating_sub(MAX_SCALAR_CURRENT_PATH_SELF_UPDATE_CASCADE_SCAN_ENTRIES);
        let trace = self.trace_history_variable(variable);
        if trace {
            eprintln!(
                "OptIR {}: replay self-update cascade variable={} id={} entries={} path={} steps={}",
                self.mir.module_name,
                self.variable_name(variable),
                variable.index(),
                history.len(),
                self.conditional_path_stack.len(),
                self.history_reconstruction_steps
            );
        }
        for current_index in (scan_start..history.len()).rev() {
            let mut terms = Vec::new();
            let mut atoms = Vec::new();
            let term_start = current_index
                .saturating_add(1)
                .saturating_sub(MAX_SCALAR_CURRENT_PATH_SELF_UPDATE_CASCADE_TERMS)
                .max(scan_start);
            for entry_index in (term_start..=current_index).rev() {
                let entry = history[entry_index].clone();
                let Some(candidate) =
                    self.self_update_cascade_candidate_for_current_path(variable, entry.expr)
                else {
                    if trace {
                        eprintln!(
                            "OptIR {}: self-update cascade stop index={} expr={} no cascade update",
                            self.mir.module_name,
                            entry_index,
                            entry.expr.index()
                        );
                    }
                    break;
                };
                if self.expr_references_variable(candidate.value_expr, variable) {
                    if trace {
                        eprintln!(
                            "OptIR {}: self-update cascade stop index={} expr={} value references target",
                            self.mir.module_name,
                            entry_index,
                            candidate.value_expr.index()
                        );
                    }
                    break;
                };
                let Some(indexed) =
                    self.index_condition_literals(&candidate.active_literals, &mut atoms)
                else {
                    if trace {
                        eprintln!(
                            "OptIR {}: self-update cascade stop index={} value={} atom limit/contradiction",
                            self.mir.module_name,
                            entry_index,
                            candidate.value_expr.index()
                        );
                    }
                    break;
                };
                terms.push(CurrentPathSelfUpdateCascadeTerm {
                    entry,
                    value_expr: candidate.value_expr,
                    active_literals: candidate.active_literals,
                    indexed_literals: indexed,
                });
                if trace {
                    eprintln!(
                        "OptIR {}: self-update cascade term index={} terms={} atoms={}",
                        self.mir.module_name,
                        entry_index,
                        terms.len(),
                        atoms.len()
                    );
                }
                if terms.len() >= 2
                    && self.indexed_condition_terms_cover_current_path(&terms, atoms.len())
                {
                    if trace {
                        eprintln!(
                            "OptIR {}: self-update cascade covered terms={} atoms={}",
                            self.mir.module_name,
                            terms.len(),
                            atoms.len()
                        );
                    }
                    return self.lower_self_update_cascade_terms_for_current_path(variable, &terms);
                }
            }
        }
        None
    }

    fn self_update_cascade_candidate_for_current_path(
        &mut self,
        variable: VariableId,
        expr: ExprId,
    ) -> Option<CurrentPathSelfUpdateCascadeCandidate> {
        self.self_update_cascade_candidate_inner(variable, expr, 0, &mut HashSet::new())
    }

    fn self_update_cascade_candidate_inner(
        &mut self,
        variable: VariableId,
        expr: ExprId,
        depth: usize,
        active: &mut HashSet<ExprId>,
    ) -> Option<CurrentPathSelfUpdateCascadeCandidate> {
        if depth > MAX_SCALAR_CONDITION_REASONING_DEPTH || !active.insert(expr) {
            return None;
        }

        let result = (|| {
            let expression = self.mir.expressions.get(usize::from(expr))?;
            let HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } = &expression.kind
            else {
                return None;
            };

            if let Some(condition_truth) = self.condition_truth_in_current_path(*condition) {
                let selected = if condition_truth {
                    *then_expr
                } else {
                    *else_expr
                };
                if self.variable_identifier(selected) == Some(variable) {
                    return None;
                }
                return self.self_update_cascade_candidate_inner(
                    variable,
                    selected,
                    depth + 1,
                    active,
                );
            }

            let then_self = self.variable_identifier(*then_expr) == Some(variable);
            let else_self = self.variable_identifier(*else_expr) == Some(variable);
            match (then_self, else_self) {
                (false, true) => self.self_update_cascade_branch_candidate(
                    variable,
                    *then_expr,
                    *condition,
                    true,
                    depth + 1,
                    active,
                ),
                (true, false) => self.self_update_cascade_branch_candidate(
                    variable,
                    *else_expr,
                    *condition,
                    false,
                    depth + 1,
                    active,
                ),
                _ => None,
            }
        })();
        active.remove(&expr);
        result
    }

    fn self_update_cascade_branch_candidate(
        &mut self,
        variable: VariableId,
        value_expr: ExprId,
        condition: ExprId,
        truth: bool,
        depth: usize,
        active: &mut HashSet<ExprId>,
    ) -> Option<CurrentPathSelfUpdateCascadeCandidate> {
        let mut active_literals =
            self.conjunctive_condition_literals_for_current_path(condition, truth)?;
        if self.expr_references_variable(value_expr, variable) {
            let candidate =
                self.self_update_cascade_candidate_inner(variable, value_expr, depth + 1, active)?;
            for literal in candidate.active_literals {
                self.push_conjunctive_condition_literal(&mut active_literals, literal)?;
            }
            return Some(CurrentPathSelfUpdateCascadeCandidate {
                value_expr: candidate.value_expr,
                active_literals,
            });
        }
        Some(CurrentPathSelfUpdateCascadeCandidate {
            value_expr,
            active_literals,
        })
    }

    fn conjunctive_condition_literals_for_current_path(
        &self,
        condition: ExprId,
        truth: bool,
    ) -> Option<Vec<ConditionLiteral>> {
        let mut literals = Vec::new();
        self.collect_conjunctive_condition_literals_for_current_path(
            condition,
            truth,
            0,
            &mut HashSet::new(),
            &mut literals,
        )?;
        Some(literals)
    }

    fn collect_conjunctive_condition_literals_for_current_path(
        &self,
        condition: ExprId,
        truth: bool,
        depth: usize,
        active: &mut HashSet<ExprId>,
        literals: &mut Vec<ConditionLiteral>,
    ) -> Option<()> {
        if depth > MAX_SCALAR_CONDITION_REASONING_DEPTH || !active.insert(condition) {
            return None;
        }

        let result = (|| {
            if let Some(condition_truth) = self.condition_truth_in_current_path(condition) {
                return (condition_truth == truth).then_some(());
            }
            if let Some(alias) = self.guard_alias_expr(condition) {
                return self.collect_conjunctive_condition_literals_for_current_path(
                    alias,
                    truth,
                    depth + 1,
                    active,
                    literals,
                );
            }
            let expression = self.mir.expressions.get(usize::from(condition))?;
            match &expression.kind {
                HirExprKind::Unary { op, operand } if op.as_str() == "Not" => self
                    .collect_conjunctive_condition_literals_for_current_path(
                        *operand,
                        !truth,
                        depth + 1,
                        active,
                        literals,
                    ),
                HirExprKind::Binary { op, left, right } if op.as_str() == "And" && truth => {
                    self.collect_conjunctive_condition_literals_for_current_path(
                        *left,
                        true,
                        depth + 1,
                        active,
                        literals,
                    )?;
                    self.collect_conjunctive_condition_literals_for_current_path(
                        *right,
                        true,
                        depth + 1,
                        active,
                        literals,
                    )
                }
                HirExprKind::Binary { op, left, right } if op.as_str() == "Or" && !truth => {
                    self.collect_conjunctive_condition_literals_for_current_path(
                        *left,
                        false,
                        depth + 1,
                        active,
                        literals,
                    )?;
                    self.collect_conjunctive_condition_literals_for_current_path(
                        *right,
                        false,
                        depth + 1,
                        active,
                        literals,
                    )
                }
                _ => self.push_conjunctive_condition_literal(
                    literals,
                    ConditionLiteral { condition, truth },
                ),
            }
        })();
        active.remove(&condition);
        result
    }

    fn push_conjunctive_condition_literal(
        &self,
        literals: &mut Vec<ConditionLiteral>,
        literal: ConditionLiteral,
    ) -> Option<()> {
        for existing in literals.iter() {
            if self.expr_structurally_equal(existing.condition, literal.condition) {
                return (existing.truth == literal.truth).then_some(());
            }
            if self.expr_is_logical_complement(existing.condition, literal.condition) {
                return (existing.truth != literal.truth).then_some(());
            }
        }
        literals.push(literal);
        Some(())
    }

    fn index_condition_literals(
        &self,
        literals: &[ConditionLiteral],
        atoms: &mut Vec<ExprId>,
    ) -> Option<Vec<IndexedConditionLiteral>> {
        let mut indexed = Vec::with_capacity(literals.len());
        for literal in literals {
            let mut matched = None;
            for (atom, condition) in atoms.iter().enumerate() {
                if self.expr_structurally_equal(*condition, literal.condition) {
                    matched = Some(IndexedConditionLiteral {
                        atom,
                        truth: literal.truth,
                    });
                    break;
                }
                if self.expr_is_logical_complement(*condition, literal.condition) {
                    matched = Some(IndexedConditionLiteral {
                        atom,
                        truth: !literal.truth,
                    });
                    break;
                }
            }
            let literal = if let Some(literal) = matched {
                literal
            } else {
                if atoms.len() >= MAX_SCALAR_CURRENT_PATH_SELF_UPDATE_CASCADE_ATOMS {
                    return None;
                }
                let atom = atoms.len();
                atoms.push(literal.condition);
                IndexedConditionLiteral {
                    atom,
                    truth: literal.truth,
                }
            };
            if indexed.iter().any(|existing: &IndexedConditionLiteral| {
                existing.atom == literal.atom && existing.truth != literal.truth
            }) {
                return None;
            }
            if !indexed.contains(&literal) {
                indexed.push(literal);
            }
        }
        Some(indexed)
    }

    fn indexed_condition_terms_cover_current_path(
        &self,
        terms: &[CurrentPathSelfUpdateCascadeTerm],
        atom_count: usize,
    ) -> bool {
        if atom_count > MAX_SCALAR_CURRENT_PATH_SELF_UPDATE_CASCADE_ATOMS {
            return false;
        }
        let assignments = 1usize << atom_count;
        (0..assignments).all(|assignment| {
            terms.iter().any(|term| {
                term.indexed_literals
                    .iter()
                    .all(|literal| (((assignment >> literal.atom) & 1) != 0) == literal.truth)
            })
        })
    }

    fn lower_self_update_cascade_terms_for_current_path(
        &mut self,
        variable: VariableId,
        terms: &[CurrentPathSelfUpdateCascadeTerm],
    ) -> Option<ValueId> {
        let value_type = self.variable_opt_value_type(variable)?;
        let mut value = None;
        for term in terms.iter().rev() {
            let term_value = self
                .lower_self_update_cascade_value_for_current_path(term)
                .and_then(|value| self.coerce_value_to_variable_type(variable, value))?;
            value = Some(if let Some(previous) = value {
                let condition =
                    self.lower_condition_literals_with_assignment_snapshot(&term.entry, term)?;
                self.push_typed_select(value_type, condition, term_value, previous)
            } else {
                term_value
            });
        }
        value
    }

    fn lower_condition_literals_with_assignment_snapshot(
        &mut self,
        entry: &AssignmentHistoryEntry,
        term: &CurrentPathSelfUpdateCascadeTerm,
    ) -> Option<ValueId> {
        let mut condition = None;
        for literal in &term.active_literals {
            let mut value =
                self.lower_expression_with_assignment_snapshot(entry, literal.condition)?;
            if !literal.truth {
                value = self.push_value(
                    OptValueType::Boolean,
                    OptValueKind::Unary {
                        op: OptUnaryOp::Not,
                        input: value,
                    },
                );
            }
            condition = Some(if let Some(previous) = condition {
                self.push_value(
                    OptValueType::Boolean,
                    OptValueKind::Binary {
                        op: OptBinaryOp::And,
                        left: previous,
                        right: value,
                    },
                )
            } else {
                value
            });
        }
        condition.or_else(|| {
            Some(self.push_value(OptValueType::Boolean, OptValueKind::BooleanConstant(true)))
        })
    }

    fn lower_self_update_cascade_value_for_current_path(
        &mut self,
        term: &CurrentPathSelfUpdateCascadeTerm,
    ) -> Option<ValueId> {
        let mut pushed = 0usize;
        for literal in &term.active_literals {
            if self
                .condition_truth_in_current_path(literal.condition)
                .is_some_and(|truth| truth == literal.truth)
            {
                continue;
            }
            if self.conditional_path_stack.len() >= MAX_SCALAR_CURRENT_PATH_HISTORY_DEPTH {
                break;
            }
            self.conditional_path_stack.push(ConditionalPathPredicate {
                condition: literal.condition,
                truth: literal.truth,
            });
            pushed += 1;
        }
        let lowered = self.lower_expression_with_assignment_snapshot(&term.entry, term.value_expr);
        for _ in 0..pushed {
            self.conditional_path_stack.pop();
        }
        lowered
    }

    fn lower_complementary_assignment_history_pair_for_current_path(
        &mut self,
        variable: VariableId,
        history: &[AssignmentHistoryEntry],
    ) -> Option<ValueId> {
        if self.conditional_path_stack.len() > MAX_SCALAR_CURRENT_PATH_HISTORY_DEPTH
            || (self.history_reconstruction_budget_exhausted
                && !self.exhausted_budget_current_path_history_replay(history.len()))
        {
            return None;
        }
        let scan_start = history
            .len()
            .saturating_sub(MAX_SCALAR_CURRENT_PATH_HISTORY_SCAN_ENTRIES);
        for current_index in (scan_start.saturating_add(1)..history.len()).rev() {
            let previous_entry = &history[current_index - 1];
            let current_entry = &history[current_index];
            let Some(previous) =
                self.conditional_self_update_for_current_path(variable, previous_entry.expr)
            else {
                continue;
            };
            let Some(current) =
                self.conditional_self_update_for_current_path(variable, current_entry.expr)
            else {
                continue;
            };
            if self.expr_references_variable(previous.value_expr, variable)
                || self.expr_references_variable(current.value_expr, variable)
            {
                continue;
            }
            let Some(previous_condition) = self.self_update_relative_to_current_path(previous)
            else {
                continue;
            };
            let Some(current_condition) = self.self_update_relative_to_current_path(current) else {
                continue;
            };
            if !self.relative_conditions_are_complements(previous_condition, current_condition) {
                continue;
            }

            if self.conditional_path_stack.len() >= MAX_SCALAR_CURRENT_PATH_HISTORY_DEPTH {
                return None;
            }
            self.conditional_path_stack.push(ConditionalPathPredicate {
                condition: previous_condition.condition,
                truth: previous_condition.truth,
            });
            let previous_value =
                self.lower_expression_with_assignment_snapshot(previous_entry, previous.value_expr);
            self.conditional_path_stack.pop();
            let previous_value = previous_value?;

            if self.conditional_path_stack.len() >= MAX_SCALAR_CURRENT_PATH_HISTORY_DEPTH {
                return None;
            }
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
            return Some(self.push_typed_select(value_type, condition, then_value, else_value));
        }
        None
    }

    fn conditional_self_update_for_current_path(
        &self,
        variable: VariableId,
        expr: ExprId,
    ) -> Option<ConditionalSelfUpdate> {
        let expression = self.mir.expressions.get(usize::from(expr))?;
        if let HirExprKind::Conditional {
            condition,
            then_expr,
            else_expr,
        } = &expression.kind
            && let Some(condition_truth) = self.condition_truth_in_current_path(*condition)
        {
            let selected = if condition_truth {
                *then_expr
            } else {
                *else_expr
            };
            if self.variable_identifier(selected) == Some(variable) {
                return None;
            }
            return self.conditional_self_update_for_current_path(variable, selected);
        }
        self.conditional_self_update(variable, expr)
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
            if let Some(value) = self.lower_static_mod(left, right) {
                return Some(value);
            }
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
            | OptBinaryOp::Mod
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
        if let Some(truth) = self.condition_truth_in_current_path(condition_expr) {
            let selected = if truth { then_expr } else { else_expr };
            return self.lower_expression(selected);
        }

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
        if args.is_empty() {
            return None;
        }
        let mut result = None;
        for argument in args {
            let expression = self.mir.expressions.get(usize::from(*argument))?;
            let HirExprKind::StringLiteral { value } = &expression.kind else {
                return None;
            };
            let query = normalize_analysis_query(value.as_str()).map(SmolStr::new);
            let value = match query {
                Some(query) => {
                    self.push_value(OptValueType::Real, OptValueKind::Analysis { query })
                }
                None => self.zero_real(),
            };
            result = Some(match result {
                Some(previous) => self.push_value(
                    OptValueType::Boolean,
                    OptValueKind::Binary {
                        op: OptBinaryOp::Or,
                        left: previous,
                        right: value,
                    },
                ),
                None => value,
            });
        }
        let result = result?;
        if args.len() == 1 {
            Some(result)
        } else {
            let one = self.push_value(OptValueType::Real, OptValueKind::RealConstant(1.0));
            let zero = self.zero_real();
            Some(self.push_value(
                OptValueType::Real,
                OptValueKind::Select {
                    condition: result,
                    then_value: one,
                    else_value: zero,
                },
            ))
        }
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
        "__rspice_initial_step" => Some("__rspice_initial_step"),
        "__rspice_final_step" => Some("__rspice_final_step"),
        _ => None,
    }
}

fn expanded_assignment_history_snapshots_enabled(hir: Option<&HirModel>, mir: &MirModel) -> bool {
    let expression_count = hir
        .map(|hir| hir.expressions.len())
        .unwrap_or(mir.expressions.len());
    let statement_count = hir.map(|hir| hir.statements.len()).unwrap_or(0);
    expression_count <= MAX_EXPANDED_ASSIGNMENT_HISTORY_SNAPSHOT_EXPRESSIONS
        && statement_count <= MAX_EXPANDED_ASSIGNMENT_HISTORY_SNAPSHOT_STATEMENTS
}

fn history_reconstruction_step_limit(
    hir: Option<&HirModel>,
    mir: &MirModel,
    expanded_assignment_history_snapshots: bool,
) -> usize {
    if expanded_assignment_history_snapshots {
        return MAX_SCALAR_HISTORY_RECONSTRUCTION_STEPS;
    }

    let expression_count = hir
        .map(|hir| hir.expressions.len())
        .unwrap_or(mir.expressions.len());
    let statement_count = hir.map(|hir| hir.statements.len()).unwrap_or(0);

    if expression_count > MAX_LARGE_SELECTIVE_ASSIGNMENT_HISTORY_SNAPSHOT_EXPRESSIONS
        || statement_count > MAX_LARGE_SELECTIVE_ASSIGNMENT_HISTORY_SNAPSHOT_STATEMENTS
    {
        return MAX_HUGE_SCALAR_HISTORY_RECONSTRUCTION_STEPS;
    }

    if expression_count > MAX_SELECTIVE_ASSIGNMENT_HISTORY_SNAPSHOT_EXPRESSIONS
        || statement_count > MAX_SELECTIVE_ASSIGNMENT_HISTORY_SNAPSHOT_STATEMENTS
    {
        return MAX_LARGE_SCALAR_HISTORY_RECONSTRUCTION_STEPS;
    }

    MAX_SCALAR_HISTORY_RECONSTRUCTION_STEPS
}

#[derive(Clone, Copy)]
struct SelectiveAssignmentHistoryBudget {
    max_targets: usize,
    arithmetic_dependency_depth: usize,
    simple_dependency_depth: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SelectiveAssignmentDependencyDepth {
    arithmetic: usize,
    simple: usize,
}

impl SelectiveAssignmentDependencyDepth {
    fn from_budget(budget: SelectiveAssignmentHistoryBudget) -> Self {
        Self {
            arithmetic: budget.arithmetic_dependency_depth,
            simple: budget.simple_dependency_depth,
        }
    }

    fn arithmetic_dependency(self) -> Option<Self> {
        Some(Self {
            arithmetic: self.arithmetic.checked_sub(1)?,
            simple: self.simple,
        })
    }

    fn simple_dependency(self) -> Option<Self> {
        Some(Self {
            arithmetic: 0,
            simple: self.simple.checked_sub(1)?,
        })
    }

    fn dominates(self, other: Self) -> bool {
        self.arithmetic >= other.arithmetic && self.simple >= other.simple
    }
}

fn selective_assignment_history_budget(
    hir: Option<&HirModel>,
    mir: &MirModel,
) -> Option<SelectiveAssignmentHistoryBudget> {
    let expression_count = hir
        .map(|hir| hir.expressions.len())
        .unwrap_or(mir.expressions.len());
    let statement_count = hir.map(|hir| hir.statements.len()).unwrap_or(0);
    if expression_count <= MAX_SELECTIVE_ASSIGNMENT_HISTORY_SNAPSHOT_EXPRESSIONS
        && statement_count <= MAX_SELECTIVE_ASSIGNMENT_HISTORY_SNAPSHOT_STATEMENTS
    {
        return Some(SelectiveAssignmentHistoryBudget {
            max_targets: MAX_SELECTIVE_ASSIGNMENT_HISTORY_TARGETS,
            arithmetic_dependency_depth: MAX_SELECTIVE_ASSIGNMENT_ARITHMETIC_DEPENDENCY_DEPTH,
            simple_dependency_depth: MAX_SELECTIVE_ASSIGNMENT_SIMPLE_DEPENDENCY_DEPTH,
        });
    }
    if expression_count <= MAX_LARGE_SELECTIVE_ASSIGNMENT_HISTORY_SNAPSHOT_EXPRESSIONS
        && statement_count <= MAX_LARGE_SELECTIVE_ASSIGNMENT_HISTORY_SNAPSHOT_STATEMENTS
    {
        return Some(SelectiveAssignmentHistoryBudget {
            max_targets: MAX_LARGE_SELECTIVE_ASSIGNMENT_HISTORY_TARGETS,
            arithmetic_dependency_depth: MAX_LARGE_SELECTIVE_ASSIGNMENT_ARITHMETIC_DEPENDENCY_DEPTH,
            simple_dependency_depth: MAX_LARGE_SELECTIVE_ASSIGNMENT_SIMPLE_DEPENDENCY_DEPTH,
        });
    }
    None
}

fn selective_assignment_history_targets(
    hir: Option<&HirModel>,
    mir: &MirModel,
    track_complete_history: bool,
) -> HashSet<VariableId> {
    if track_complete_history {
        return HashSet::new();
    }
    let Some(hir) = hir else {
        return HashSet::new();
    };
    let Some(budget) = selective_assignment_history_budget(Some(hir), mir) else {
        return HashSet::new();
    };
    let variables_by_name: HashMap<_, _> = hir
        .variables
        .iter()
        .map(|variable| (variable.name.as_str(), variable.id))
        .collect();
    let variable_names_by_id: HashMap<_, _> = hir
        .variables
        .iter()
        .map(|variable| (variable.id, variable.name.as_str()))
        .collect();
    let direct_target_groups =
        collect_selective_assignment_direct_target_groups(mir, &variables_by_name);
    let mut assignments = Vec::new();
    collect_hir_assignments(&hir.statements, &mut assignments);
    let mut assignments_by_target: HashMap<_, Vec<_>> = HashMap::new();
    for assignment in assignments {
        assignments_by_target
            .entry(assignment.target)
            .or_default()
            .push(assignment);
    }

    let mut targets = HashSet::new();
    let mut processed_depths: HashMap<VariableId, SelectiveAssignmentDependencyDepth> =
        HashMap::new();
    select_selective_assignment_history_targets(
        &direct_target_groups,
        budget,
        &mut targets,
        &mut processed_depths,
        |target, dependency_depth| {
            let Some(assignments) = assignments_by_target.get(&target) else {
                return Vec::new();
            };
            let mut dependencies = HashMap::new();
            for assignment in assignments {
                collect_selective_assignment_dependencies(
                    mir,
                    &variables_by_name,
                    assignment,
                    dependency_depth,
                    &mut dependencies,
                );
            }
            let mut dependencies: Vec<_> = dependencies.into_iter().collect();
            dependencies.retain(|(dependency, _)| {
                selective_assignment_history_dependency_variable(
                    &variable_names_by_id,
                    &assignments_by_target,
                    *dependency,
                )
            });
            sort_selective_assignment_dependencies(&mut dependencies);
            dependencies
        },
    );
    trace_selective_assignment_history_targets(hir, &targets);
    targets
}

fn local_assignment_history_targets(
    hir: Option<&HirModel>,
    expanded_assignment_history_snapshots: bool,
) -> HashSet<VariableId> {
    if expanded_assignment_history_snapshots {
        return HashSet::new();
    }
    let Some(hir) = hir else {
        return HashSet::new();
    };
    let mut assignments = Vec::new();
    collect_hir_assignments(&hir.statements, &mut assignments);
    let mut assignment_counts: HashMap<VariableId, usize> = HashMap::new();
    for assignment in assignments {
        *assignment_counts.entry(assignment.target).or_default() += 1;
    }
    hir.variables
        .iter()
        .filter(|variable| {
            let assignment_count = assignment_counts
                .get(&variable.id)
                .copied()
                .unwrap_or_default();
            repeated_scratch_history_variable(variable.name.as_str(), assignment_count)
        })
        .map(|variable| variable.id)
        .collect()
}

fn collect_selective_assignment_direct_target_groups(
    mir: &MirModel,
    variables_by_name: &HashMap<&str, VariableId>,
) -> Vec<Vec<VariableId>> {
    let mut target_groups = Vec::new();
    let mut seen = HashSet::new();
    for equation in &mir.equations {
        let mut targets = Vec::new();
        collect_expression_direct_assignment_targets(
            mir,
            variables_by_name,
            equation.expression.id,
            &mut seen,
            &mut targets,
        );
        if !targets.is_empty() {
            target_groups.push(targets);
        }
    }
    target_groups
}

fn collect_expression_direct_assignment_targets(
    mir: &MirModel,
    variables_by_name: &HashMap<&str, VariableId>,
    root: ExprId,
    seen: &mut HashSet<VariableId>,
    targets: &mut Vec<VariableId>,
) {
    let mut visited = HashSet::new();
    let mut stack = vec![root];
    while let Some(expr) = stack.pop() {
        if !visited.insert(expr) {
            continue;
        }
        let Some(expression) = mir.expressions.get(usize::from(expr)) else {
            continue;
        };
        match &expression.kind {
            HirExprKind::Identifier { name } => {
                if !selective_assignment_history_target_name(name) {
                    continue;
                }
                if let Some(variable) = variables_by_name.get(name.as_str()).copied()
                    && seen.insert(variable)
                {
                    targets.push(variable);
                }
            }
            HirExprKind::Binary { left, right, .. } => stack.extend([*right, *left]),
            HirExprKind::Unary { operand, .. } => stack.push(*operand),
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => stack.extend([*else_expr, *then_expr, *condition]),
            HirExprKind::Call { args, .. }
            | HirExprKind::SystemFunction { args, .. }
            | HirExprKind::ArrayLiteral { elements: args } => {
                stack.extend(args.iter().rev().copied())
            }
            HirExprKind::ArrayAccess { index, .. } => stack.push(*index),
            HirExprKind::AnalogOperator { op } => {
                push_analog_operator_expr_children(op, &mut stack);
            }
            HirExprKind::Laplace { expr, .. } | HirExprKind::Zi { expr, .. } => stack.push(*expr),
            HirExprKind::NoiseSource { operands, .. } => {
                stack.extend(operands.iter().rev().copied())
            }
            HirExprKind::Number { .. }
            | HirExprKind::StringLiteral { .. }
            | HirExprKind::BranchAccess { .. }
            | HirExprKind::NamedBranchAccess { .. } => {}
        }
    }
}

fn selective_assignment_history_target_variable(
    variable_names_by_id: &HashMap<VariableId, &str>,
    variable: VariableId,
) -> bool {
    variable_names_by_id
        .get(&variable)
        .is_none_or(|name| selective_assignment_history_target_name(name))
}

fn selective_assignment_history_dependency_variable(
    variable_names_by_id: &HashMap<VariableId, &str>,
    assignments_by_target: &HashMap<VariableId, Vec<&HirAssignment>>,
    variable: VariableId,
) -> bool {
    if !selective_assignment_history_target_variable(variable_names_by_id, variable) {
        return false;
    }
    let Some(name) = variable_names_by_id.get(&variable).copied() else {
        return true;
    };
    let assignment_count = assignments_by_target
        .get(&variable)
        .map(Vec::len)
        .unwrap_or(0);
    !high_churn_scratch_history_variable(name, assignment_count)
}

fn selective_assignment_history_target_name(name: &str) -> bool {
    // Generated guards are normally available as current scalar values and can be
    // snapshotted by history entries without spending selective history targets.
    !name.starts_with("__guard")
}

fn high_churn_scratch_history_variable(name: &str, assignment_count: usize) -> bool {
    assignment_count > MAX_SELECTIVE_SCRATCH_HISTORY_ASSIGNMENTS
        && (numbered_temp_name(name, "T") || numbered_temp_name(name, "TMF"))
}

fn repeated_scratch_history_variable(name: &str, assignment_count: usize) -> bool {
    assignment_count > 1 && (numbered_temp_name(name, "T") || numbered_temp_name(name, "TMF"))
}

fn numbered_temp_name(name: &str, prefix: &str) -> bool {
    let Some(suffix) = name.strip_prefix(prefix) else {
        return false;
    };
    !suffix.is_empty() && suffix.bytes().all(|byte| byte.is_ascii_digit())
}

fn select_selective_assignment_history_targets<F>(
    direct_target_groups: &[Vec<VariableId>],
    budget: SelectiveAssignmentHistoryBudget,
    targets: &mut HashSet<VariableId>,
    processed_depths: &mut HashMap<VariableId, SelectiveAssignmentDependencyDepth>,
    mut dependencies_for: F,
) where
    F: FnMut(
        VariableId,
        SelectiveAssignmentDependencyDepth,
    ) -> Vec<(VariableId, SelectiveAssignmentDependencyDepth)>,
{
    for direct_targets in direct_target_groups {
        let root_depth = SelectiveAssignmentDependencyDepth::from_budget(budget);
        for &root in direct_targets {
            if !targets.contains(&root) {
                if targets.len() >= budget.max_targets {
                    return;
                }
                targets.insert(root);
            }
        }
        for &root in direct_targets {
            if processed_depths
                .get(&root)
                .is_some_and(|processed| processed.dominates(root_depth))
            {
                continue;
            }
            let mut pending = VecDeque::from([(root, root_depth)]);
            while let Some((target, dependency_depth)) = pending.pop_front() {
                if !targets.contains(&target) {
                    if targets.len() >= budget.max_targets {
                        return;
                    }
                    targets.insert(target);
                }
                if processed_depths
                    .get(&target)
                    .is_some_and(|processed| processed.dominates(dependency_depth))
                {
                    continue;
                }
                insert_selective_assignment_dependency(processed_depths, target, dependency_depth);
                for (dependency, dependency_depth) in dependencies_for(target, dependency_depth) {
                    if !targets.contains(&dependency) && targets.len() >= budget.max_targets {
                        return;
                    }
                    if processed_depths
                        .get(&dependency)
                        .is_none_or(|processed| !processed.dominates(dependency_depth))
                    {
                        pending.push_back((dependency, dependency_depth));
                    }
                }
            }
        }
    }
}

fn sort_selective_assignment_dependencies(
    dependencies: &mut [(VariableId, SelectiveAssignmentDependencyDepth)],
) {
    dependencies.sort_by(
        |(left_variable, left_depth), (right_variable, right_depth)| {
            right_depth
                .arithmetic
                .cmp(&left_depth.arithmetic)
                .then_with(|| right_depth.simple.cmp(&left_depth.simple))
                .then_with(|| usize::from(*left_variable).cmp(&usize::from(*right_variable)))
        },
    );
}

fn trace_selective_assignment_history_targets(hir: &HirModel, targets: &HashSet<VariableId>) {
    let Some(filter) = opt_selective_history_target_trace_filter() else {
        return;
    };
    let filters: Vec<_> = filter
        .split(',')
        .map(str::trim)
        .filter(|filter| !filter.is_empty())
        .collect();
    if filters.is_empty() {
        return;
    }
    for variable in &hir.variables {
        if filters
            .iter()
            .any(|filter| variable.name.as_str().contains(filter))
        {
            eprintln!(
                "OptIR {}: selective history target variable={} id={} selected={} total={}",
                hir.module_name,
                variable.name,
                variable.id.index(),
                targets.contains(&variable.id),
                targets.len()
            );
        }
    }
}

fn collect_selective_assignment_dependencies(
    mir: &MirModel,
    variables_by_name: &HashMap<&str, VariableId>,
    assignment: &HirAssignment,
    dependency_depth: SelectiveAssignmentDependencyDepth,
    variables: &mut HashMap<VariableId, SelectiveAssignmentDependencyDepth>,
) {
    if let Some(update) = conditional_self_update_expr(
        mir,
        variables_by_name,
        assignment.target,
        assignment.expr.id,
    ) {
        let mut condition_variables = HashSet::new();
        collect_expression_variables(
            mir,
            variables_by_name,
            update.condition,
            None,
            &mut condition_variables,
        );
        for variable in condition_variables {
            insert_selective_assignment_dependency(variables, variable, dependency_depth);
        }
        if let Some(variable) = variable_identifier_expr(mir, variables_by_name, update.value_expr)
            && variable != assignment.target
        {
            insert_selective_assignment_dependency(variables, variable, dependency_depth);
        } else if let Some(next_depth) = dependency_depth
            .arithmetic_dependency()
            .or_else(|| dependency_depth.simple_dependency())
            && let Some(dependencies) = collect_selective_assignment_replay_dependencies(
                mir,
                variables_by_name,
                update.value_expr,
                Some(assignment.target),
            )
            .or_else(|| {
                collect_selective_assignment_broad_dependencies(
                    mir,
                    variables_by_name,
                    update.value_expr,
                    Some(assignment.target),
                )
            })
        {
            for variable in dependencies {
                insert_selective_assignment_dependency(variables, variable, next_depth);
            }
        }
        return;
    }

    if assignment.target_name.starts_with("__guard") {
        let mut guard_variables = HashSet::new();
        collect_expression_variables(
            mir,
            variables_by_name,
            assignment.expr.id,
            Some(assignment.target),
            &mut guard_variables,
        );
        for variable in guard_variables {
            insert_selective_assignment_dependency(variables, variable, dependency_depth);
        }
        return;
    }

    if let Some(variable) = variable_identifier_expr(mir, variables_by_name, assignment.expr.id)
        && variable != assignment.target
    {
        insert_selective_assignment_dependency(variables, variable, dependency_depth);
    } else if let Some(next_depth) = dependency_depth
        .arithmetic_dependency()
        .or_else(|| dependency_depth.simple_dependency())
        && let Some(dependencies) = collect_selective_assignment_replay_dependencies(
            mir,
            variables_by_name,
            assignment.expr.id,
            Some(assignment.target),
        )
        .or_else(|| {
            collect_selective_assignment_broad_dependencies(
                mir,
                variables_by_name,
                assignment.expr.id,
                Some(assignment.target),
            )
        })
    {
        for variable in dependencies {
            insert_selective_assignment_dependency(variables, variable, next_depth);
        }
    }
}

fn insert_selective_assignment_dependency(
    variables: &mut HashMap<VariableId, SelectiveAssignmentDependencyDepth>,
    variable: VariableId,
    dependency_depth: SelectiveAssignmentDependencyDepth,
) {
    let entry = variables.entry(variable).or_insert(dependency_depth);
    entry.arithmetic = entry.arithmetic.max(dependency_depth.arithmetic);
    entry.simple = entry.simple.max(dependency_depth.simple);
}

fn collect_selective_assignment_replay_dependencies(
    mir: &MirModel,
    variables_by_name: &HashMap<&str, VariableId>,
    expr: ExprId,
    excluded: Option<VariableId>,
) -> Option<HashSet<VariableId>> {
    let mut variables = HashSet::new();
    let mut visited = HashSet::new();
    let mut stack = vec![expr];
    let mut nodes = 0;
    while let Some(expr) = stack.pop() {
        if !visited.insert(expr) {
            continue;
        }
        nodes += 1;
        if nodes > MAX_SELECTIVE_ASSIGNMENT_DEPENDENCY_EXPR_NODES {
            return None;
        }
        let expression = mir.expressions.get(usize::from(expr))?;
        match &expression.kind {
            HirExprKind::Identifier { name } => {
                if let Some(variable) = variables_by_name.get(name.as_str()).copied()
                    && Some(variable) != excluded
                {
                    variables.insert(variable);
                    if variables.len() > MAX_SELECTIVE_ASSIGNMENT_DEPENDENCY_VARIABLES {
                        return None;
                    }
                }
            }
            HirExprKind::Number { .. }
            | HirExprKind::StringLiteral { .. }
            | HirExprKind::BranchAccess { .. }
            | HirExprKind::NamedBranchAccess { .. } => {}
            HirExprKind::Unary { op, operand }
                if simple_assignment_alias_snapshot_unary_op(op, true) =>
            {
                stack.push(*operand);
            }
            HirExprKind::Binary { op, left, right }
                if simple_assignment_alias_snapshot_binary_op(op, true) =>
            {
                stack.extend([*left, *right]);
            }
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => {
                stack.extend([*condition, *then_expr, *else_expr]);
            }
            HirExprKind::Call { name, args }
                if simple_assignment_alias_snapshot_call(name, args.len()) =>
            {
                stack.extend(args.iter().copied());
            }
            _ => return None,
        }
    }
    Some(variables)
}

fn collect_selective_assignment_broad_dependencies(
    mir: &MirModel,
    variables_by_name: &HashMap<&str, VariableId>,
    expr: ExprId,
    excluded: Option<VariableId>,
) -> Option<HashSet<VariableId>> {
    let mut variables = HashSet::new();
    collect_expression_variables(mir, variables_by_name, expr, excluded, &mut variables);
    (variables.len() <= MAX_SELECTIVE_ASSIGNMENT_BROAD_DEPENDENCY_VARIABLES).then_some(variables)
}

fn conditional_self_update_expr(
    mir: &MirModel,
    variables_by_name: &HashMap<&str, VariableId>,
    variable: VariableId,
    expr: ExprId,
) -> Option<ConditionalSelfUpdate> {
    let expression = mir.expressions.get(usize::from(expr))?;
    let HirExprKind::Conditional {
        condition,
        then_expr,
        else_expr,
    } = &expression.kind
    else {
        return None;
    };
    let then_self = variable_identifier_expr(mir, variables_by_name, *then_expr) == Some(variable);
    let else_self = variable_identifier_expr(mir, variables_by_name, *else_expr) == Some(variable);
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

fn variable_identifier_expr(
    mir: &MirModel,
    variables_by_name: &HashMap<&str, VariableId>,
    expr: ExprId,
) -> Option<VariableId> {
    let expression = mir.expressions.get(usize::from(expr))?;
    let HirExprKind::Identifier { name } = &expression.kind else {
        return None;
    };
    variables_by_name.get(name.as_str()).copied()
}

fn collect_hir_assignments<'a>(
    statements: &'a [HirStatement],
    assignments: &mut Vec<&'a HirAssignment>,
) {
    for statement in statements {
        match statement {
            HirStatement::Assignment(assignment) => assignments.push(assignment),
            HirStatement::Loop(loop_statement) => {
                collect_hir_assignments(&loop_statement.body, assignments);
            }
        }
    }
}

fn collect_hir_assignments_by_variable<'a>(
    statements: &'a [HirStatement],
    assignments: &mut HashMap<VariableId, Vec<&'a HirAssignment>>,
) {
    for statement in statements {
        match statement {
            HirStatement::Assignment(assignment) => {
                assignments
                    .entry(assignment.target)
                    .or_default()
                    .push(assignment);
            }
            HirStatement::Loop(loop_statement) => {
                collect_hir_assignments_by_variable(&loop_statement.body, assignments);
            }
        }
    }
}

fn collect_expression_variables(
    mir: &MirModel,
    variables_by_name: &HashMap<&str, VariableId>,
    expr: ExprId,
    excluded: Option<VariableId>,
    variables: &mut HashSet<VariableId>,
) {
    let mut visited = HashSet::new();
    let mut stack = vec![expr];
    while let Some(expr) = stack.pop() {
        if !visited.insert(expr) {
            continue;
        }
        let Some(expression) = mir.expressions.get(usize::from(expr)) else {
            continue;
        };
        match &expression.kind {
            HirExprKind::Identifier { name } => {
                if let Some(variable) = variables_by_name.get(name.as_str()).copied()
                    && Some(variable) != excluded
                {
                    variables.insert(variable);
                }
            }
            HirExprKind::Binary { left, right, .. } => stack.extend([*left, *right]),
            HirExprKind::Unary { operand, .. } => stack.push(*operand),
            HirExprKind::Conditional {
                condition,
                then_expr,
                else_expr,
            } => stack.extend([*condition, *then_expr, *else_expr]),
            HirExprKind::Call { args, .. }
            | HirExprKind::SystemFunction { args, .. }
            | HirExprKind::ArrayLiteral { elements: args } => stack.extend(args.iter().copied()),
            HirExprKind::ArrayAccess { index, .. } => stack.push(*index),
            HirExprKind::AnalogOperator { op } => {
                push_analog_operator_expr_children(op, &mut stack)
            }
            HirExprKind::Laplace { expr, .. } | HirExprKind::Zi { expr, .. } => stack.push(*expr),
            HirExprKind::NoiseSource { operands, .. } => stack.extend(operands.iter().copied()),
            HirExprKind::Number { .. }
            | HirExprKind::StringLiteral { .. }
            | HirExprKind::BranchAccess { .. }
            | HirExprKind::NamedBranchAccess { .. } => {}
        }
    }
}

fn push_analog_operator_expr_children(op: &HirAnalogOperator, stack: &mut Vec<ExprId>) {
    match op {
        HirAnalogOperator::Ddt { expr, abstol } => {
            stack.push(*expr);
            stack.extend(abstol.iter().copied());
        }
        HirAnalogOperator::Idt {
            expr,
            ic,
            assert,
            abstol,
        } => {
            stack.push(*expr);
            stack.extend(ic.iter().copied());
            stack.extend(assert.iter().copied());
            stack.extend(abstol.iter().copied());
        }
        HirAnalogOperator::IdtMod {
            expr,
            ic,
            modulus,
            offset,
            abstol,
        } => {
            stack.push(*expr);
            stack.extend(ic.iter().copied());
            stack.extend(modulus.iter().copied());
            stack.extend(offset.iter().copied());
            stack.extend(abstol.iter().copied());
        }
        HirAnalogOperator::Ddx { expr, probe } => {
            stack.extend([*expr, *probe]);
        }
        HirAnalogOperator::Limexp { expr } => stack.push(*expr),
        HirAnalogOperator::Absdelay {
            expr,
            delay,
            max_delay,
        } => {
            stack.extend([*expr, *delay]);
            stack.extend(max_delay.iter().copied());
        }
        HirAnalogOperator::Transition {
            expr,
            delay,
            rise,
            fall,
            tolerance,
        } => {
            stack.push(*expr);
            stack.extend(delay.iter().copied());
            stack.extend(rise.iter().copied());
            stack.extend(fall.iter().copied());
            stack.extend(tolerance.iter().copied());
        }
        HirAnalogOperator::Slew {
            expr,
            max_rise,
            max_fall,
        } => {
            stack.push(*expr);
            stack.extend(max_rise.iter().copied());
            stack.extend(max_fall.iter().copied());
        }
        HirAnalogOperator::LastCrossing { expr, .. } => stack.push(*expr),
    }
}

fn binary_op(op: &str) -> Option<OptBinaryOp> {
    match op {
        "Add" => Some(OptBinaryOp::Add),
        "Sub" => Some(OptBinaryOp::Sub),
        "Mul" => Some(OptBinaryOp::Mul),
        "Div" => Some(OptBinaryOp::Div),
        "Mod" => Some(OptBinaryOp::Mod),
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

fn simple_assignment_alias_snapshot_binary_op(op: &str, expanded: bool) -> bool {
    matches!(op, "Add" | "Sub" | "Mul" | "Div" | "Mod" | "Pow")
        || (expanded && matches!(op, "Eq" | "Ne" | "Lt" | "Le" | "Gt" | "Ge" | "And" | "Or"))
}

fn unary_op(op: &str) -> Option<OptUnaryOp> {
    match op {
        "Pos" => Some(OptUnaryOp::Pos),
        "Neg" => Some(OptUnaryOp::Neg),
        "Not" => Some(OptUnaryOp::Not),
        _ => None,
    }
}

fn simple_assignment_alias_snapshot_unary_op(op: &str, expanded: bool) -> bool {
    matches!(op, "Pos" | "Neg") || (expanded && op == "Not")
}

fn simple_assignment_alias_snapshot_call(name: &str, arity: usize) -> bool {
    match arity {
        1 => matches!(
            name,
            "exp"
                | "expm1"
                | "limexp"
                | "__rspice_limited_exp"
                | "ln"
                | "log"
                | "log1p"
                | "sqrt"
                | "abs"
                | "sin"
                | "cos"
                | "tan"
                | "sinh"
                | "cosh"
                | "tanh"
                | "atan"
                | "asinh"
                | "floor"
                | "ceil"
        ),
        2 => matches!(name, "fpow" | "min" | "max" | "pow"),
        _ => false,
    }
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
        && range.min_parameter.is_none()
        && range.max_parameter.is_none()
        && range.min_expression.is_none()
        && range.max_expression.is_none()
        && range.exclude.is_empty()
        && range.exclude_parameters.is_empty()
        && range.exclude_expressions.is_empty()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical_ir::{
        ContributionId, HirExprRef, HirExpression, HirVariable, MirBranchRef, ModuleId,
        SourceSpanRef,
    };

    fn test_span() -> SourceSpanRef {
        SourceSpanRef {
            source_file_id: 0,
            start: 0,
            end: 0,
        }
    }

    fn test_expr_ref(id: ExprId) -> HirExprRef {
        HirExprRef {
            id,
            kind: "test".into(),
            span: test_span(),
        }
    }

    fn test_mir(expressions: Vec<HirExpression>) -> MirModel {
        MirModel {
            module_name: "test".into(),
            nodes: Vec::new(),
            parameters: Vec::new(),
            branches: Vec::new(),
            branch_unknowns: Vec::new(),
            state_slots: Vec::new(),
            equations: vec![MirEquation {
                id: EquationId::from(0),
                contribution: ContributionId::from(0),
                branch: MirBranchRef {
                    label: "p,n".into(),
                    declared_name: None,
                    pos_node: None,
                    neg_node: None,
                },
                kind: MirEquationKind::Current,
                expression: test_expr_ref(ExprId::from(0)),
                active_domains: Vec::new(),
                span: test_span(),
            }],
            expressions,
            value_symbols: Vec::new(),
            ground_nodes: Vec::new(),
        }
    }

    fn test_assignment_history_entry(expr: ExprId) -> AssignmentHistoryEntry {
        AssignmentHistoryEntry {
            expr,
            value_snapshots: Vec::new(),
            snapshots: Vec::new(),
            local_dependencies: Vec::new(),
            local_snapshots: Vec::new(),
        }
    }

    fn test_hir_variables(variables: Vec<(&str, CanonicalValueType)>) -> HirModel {
        HirModel {
            module_id: ModuleId::new(0),
            module_name: "test".into(),
            schema_version: 0,
            source_package: SmolStr::new("test"),
            source_digest: SmolStr::new("test"),
            compiler_version: SmolStr::new("test"),
            feature_flags: Vec::new(),
            ports: Vec::new(),
            parameters: Vec::new(),
            variables: variables
                .into_iter()
                .enumerate()
                .map(|(id, (name, value_type))| HirVariable {
                    id: VariableId::from(id),
                    name: name.into(),
                    value_type,
                    is_state: false,
                })
                .collect(),
            arrays: Vec::new(),
            branches: Vec::new(),
            contributions: Vec::new(),
            statements: Vec::new(),
            expressions: Vec::new(),
            internal_nodes: Vec::new(),
            ground_nodes: Vec::new(),
        }
    }

    fn test_assignment(target: VariableId, target_name: &str, expr: ExprId) -> HirAssignment {
        HirAssignment {
            target,
            target_name: target_name.into(),
            index: None,
            expr: test_expr_ref(expr),
            expr_type: CanonicalValueType::Boolean,
            span: test_span(),
            unfiltered_initial_step_guard: None,
        }
    }

    #[test]
    fn selective_assignment_targets_prioritize_selected_root_dependencies() {
        let budget = SelectiveAssignmentHistoryBudget {
            max_targets: 3,
            arithmetic_dependency_depth: 0,
            simple_dependency_depth: 2,
        };
        let root = VariableId::from(0);
        let first_dependency = VariableId::from(1);
        let second_dependency = VariableId::from(2);
        let unrelated_root = VariableId::from(10);
        let direct_target_groups = vec![vec![root], vec![unrelated_root]];
        let mut targets = HashSet::new();
        let mut processed_depths = HashMap::new();

        select_selective_assignment_history_targets(
            &direct_target_groups,
            budget,
            &mut targets,
            &mut processed_depths,
            |target, depth| match target {
                target if target == root => depth
                    .simple_dependency()
                    .map(|depth| vec![(first_dependency, depth)])
                    .unwrap_or_default(),
                target if target == first_dependency => depth
                    .simple_dependency()
                    .map(|depth| vec![(second_dependency, depth)])
                    .unwrap_or_default(),
                _ => Vec::new(),
            },
        );

        assert_eq!(targets.len(), budget.max_targets);
        assert!(targets.contains(&root));
        assert!(targets.contains(&first_dependency));
        assert!(targets.contains(&second_dependency));
        assert!(!targets.contains(&unrelated_root));
    }

    #[test]
    fn selective_assignment_targets_admit_same_equation_roots_before_dependencies() {
        let budget = SelectiveAssignmentHistoryBudget {
            max_targets: 3,
            arithmetic_dependency_depth: 0,
            simple_dependency_depth: 2,
        };
        let root = VariableId::from(0);
        let sibling_root = VariableId::from(1);
        let first_dependency = VariableId::from(2);
        let second_dependency = VariableId::from(3);
        let direct_target_groups = vec![vec![root, sibling_root]];
        let mut targets = HashSet::new();
        let mut processed_depths = HashMap::new();

        select_selective_assignment_history_targets(
            &direct_target_groups,
            budget,
            &mut targets,
            &mut processed_depths,
            |target, depth| match target {
                target if target == root => depth
                    .simple_dependency()
                    .map(|depth| vec![(first_dependency, depth)])
                    .unwrap_or_default(),
                target if target == first_dependency => depth
                    .simple_dependency()
                    .map(|depth| vec![(second_dependency, depth)])
                    .unwrap_or_default(),
                _ => Vec::new(),
            },
        );

        assert_eq!(targets.len(), budget.max_targets);
        assert!(targets.contains(&root));
        assert!(targets.contains(&sibling_root));
        assert!(targets.contains(&first_dependency));
        assert!(!targets.contains(&second_dependency));
    }

    #[test]
    fn high_churn_numbered_temporaries_are_not_selective_history_dependencies() {
        assert!(high_churn_scratch_history_variable(
            "T1",
            MAX_SELECTIVE_SCRATCH_HISTORY_ASSIGNMENTS + 1
        ));
        assert!(high_churn_scratch_history_variable(
            "T12",
            MAX_SELECTIVE_SCRATCH_HISTORY_ASSIGNMENTS + 1
        ));
        assert!(high_churn_scratch_history_variable(
            "TMF1",
            MAX_SELECTIVE_SCRATCH_HISTORY_ASSIGNMENTS + 1
        ));
        assert!(!high_churn_scratch_history_variable(
            "T1",
            MAX_SELECTIVE_SCRATCH_HISTORY_ASSIGNMENTS
        ));
        assert!(!high_churn_scratch_history_variable(
            "T1w",
            MAX_SELECTIVE_SCRATCH_HISTORY_ASSIGNMENTS + 1
        ));
        assert!(!high_churn_scratch_history_variable(
            "Ids",
            MAX_SELECTIVE_SCRATCH_HISTORY_ASSIGNMENTS + 1
        ));
    }

    #[test]
    fn repeated_numbered_temporaries_get_local_history_windows() {
        assert!(repeated_scratch_history_variable("T1", 2));
        assert!(repeated_scratch_history_variable("TMF1", 2));
        assert!(!repeated_scratch_history_variable("T1", 1));
        assert!(!repeated_scratch_history_variable("T1w", 2));
        assert!(!repeated_scratch_history_variable("Ids", 2));
    }

    #[test]
    fn selected_assignment_snapshots_local_scratch_history() {
        let condition_expr = ExprId::from(0);
        let outer_condition_expr = ExprId::from(1);
        let temp_expr = ExprId::from(2);
        let target_expr = ExprId::from(3);
        let first_value_expr = ExprId::from(4);
        let second_value_expr = ExprId::from(5);
        let first_temp_update_expr = ExprId::from(6);
        let second_temp_update_expr = ExprId::from(7);
        let target_update_expr = ExprId::from(8);
        let mir = test_mir(vec![
            HirExpression {
                id: condition_expr,
                kind: HirExprKind::Identifier {
                    name: "cond".into(),
                },
                span: test_span(),
            },
            HirExpression {
                id: outer_condition_expr,
                kind: HirExprKind::Identifier {
                    name: "outer".into(),
                },
                span: test_span(),
            },
            HirExpression {
                id: temp_expr,
                kind: HirExprKind::Identifier { name: "T2".into() },
                span: test_span(),
            },
            HirExpression {
                id: target_expr,
                kind: HirExprKind::Identifier { name: "out".into() },
                span: test_span(),
            },
            HirExpression {
                id: first_value_expr,
                kind: HirExprKind::Number {
                    value: 1.0,
                    raw: "1.0".into(),
                },
                span: test_span(),
            },
            HirExpression {
                id: second_value_expr,
                kind: HirExprKind::Number {
                    value: 2.0,
                    raw: "2.0".into(),
                },
                span: test_span(),
            },
            HirExpression {
                id: first_temp_update_expr,
                kind: HirExprKind::Conditional {
                    condition: condition_expr,
                    then_expr: first_value_expr,
                    else_expr: temp_expr,
                },
                span: test_span(),
            },
            HirExpression {
                id: second_temp_update_expr,
                kind: HirExprKind::Conditional {
                    condition: condition_expr,
                    then_expr: temp_expr,
                    else_expr: second_value_expr,
                },
                span: test_span(),
            },
            HirExpression {
                id: target_update_expr,
                kind: HirExprKind::Conditional {
                    condition: outer_condition_expr,
                    then_expr: temp_expr,
                    else_expr: target_expr,
                },
                span: test_span(),
            },
        ]);
        let hir = test_hir_variables(vec![
            ("out", CanonicalValueType::Real),
            ("T2", CanonicalValueType::Real),
            ("cond", CanonicalValueType::Boolean),
            ("outer", CanonicalValueType::Boolean),
        ]);
        let target = VariableId::from(0);
        let temp = VariableId::from(1);
        let condition = VariableId::from(2);
        let outer_condition = VariableId::from(3);
        let mut builder = ScalarGraphBuilder::new(Some(&hir), &mir);
        builder.expanded_assignment_history_snapshots = false;
        builder.track_assignment_history = false;
        builder.selective_assignment_history_targets.insert(target);
        builder.local_assignment_history_targets.insert(temp);
        builder.variable_values.insert(target, None);
        let condition_true =
            builder.push_value(OptValueType::Boolean, OptValueKind::BooleanConstant(true));
        let condition_false =
            builder.push_value(OptValueType::Boolean, OptValueKind::BooleanConstant(false));
        let outer_true =
            builder.push_value(OptValueType::Boolean, OptValueKind::BooleanConstant(true));
        let outer_false =
            builder.push_value(OptValueType::Boolean, OptValueKind::BooleanConstant(false));
        builder
            .variable_values
            .insert(condition, Some(condition_true));
        let temp_base = builder.push_value(OptValueType::Real, OptValueKind::RealConstant(0.0));
        let first_entry = builder.local_assignment_history_entry(temp, first_temp_update_expr);
        let second_entry = builder.local_assignment_history_entry(temp, second_temp_update_expr);
        assert!(
            first_entry
                .value_snapshots
                .iter()
                .any(|(variable, _)| *variable == condition)
        );
        builder.local_assignment_history.insert(
            temp,
            LocalAssignmentHistory {
                base_value: Some(temp_base),
                entries: vec![first_entry, second_entry],
            },
        );
        builder
            .variable_values
            .insert(outer_condition, Some(outer_true));
        let target_entry = builder.assignment_history_entry(target, target_update_expr);
        assert!(
            target_entry
                .value_snapshots
                .iter()
                .any(|(variable, _)| *variable == temp)
                || target_entry
                    .local_snapshots
                    .iter()
                    .any(|(variable, history)| *variable == temp && history.len() == 2)
        );
        builder
            .variable_assignment_history
            .insert(target, vec![target_entry]);

        builder
            .variable_values
            .insert(condition, Some(condition_false));
        builder
            .variable_values
            .insert(outer_condition, Some(outer_false));

        let value = builder
            .lower_assignment_history_value(target)
            .expect("selected target should replay local scratch snapshot");
        assert!(matches!(
            &builder.values[usize::from(value)].kind,
            OptValueKind::RealConstant(value) if *value == 1.0
        ));
    }

    #[test]
    fn local_assignment_snapshot_replays_complementary_window_without_base() {
        let condition_expr = ExprId::from(0);
        let inverted_condition_expr = ExprId::from(1);
        let temp_expr = ExprId::from(2);
        let first_value_expr = ExprId::from(3);
        let second_value_expr = ExprId::from(4);
        let first_update_expr = ExprId::from(5);
        let second_update_expr = ExprId::from(6);
        let mir = test_mir(vec![
            HirExpression {
                id: condition_expr,
                kind: HirExprKind::Identifier {
                    name: "cond".into(),
                },
                span: test_span(),
            },
            HirExpression {
                id: inverted_condition_expr,
                kind: HirExprKind::Unary {
                    op: "Not".into(),
                    operand: condition_expr,
                },
                span: test_span(),
            },
            HirExpression {
                id: temp_expr,
                kind: HirExprKind::Identifier { name: "T2".into() },
                span: test_span(),
            },
            HirExpression {
                id: first_value_expr,
                kind: HirExprKind::Number {
                    value: 1.0,
                    raw: "1.0".into(),
                },
                span: test_span(),
            },
            HirExpression {
                id: second_value_expr,
                kind: HirExprKind::Number {
                    value: 2.0,
                    raw: "2.0".into(),
                },
                span: test_span(),
            },
            HirExpression {
                id: first_update_expr,
                kind: HirExprKind::Conditional {
                    condition: condition_expr,
                    then_expr: first_value_expr,
                    else_expr: temp_expr,
                },
                span: test_span(),
            },
            HirExpression {
                id: second_update_expr,
                kind: HirExprKind::Conditional {
                    condition: inverted_condition_expr,
                    then_expr: second_value_expr,
                    else_expr: temp_expr,
                },
                span: test_span(),
            },
        ]);
        let hir = test_hir_variables(vec![
            ("T2", CanonicalValueType::Real),
            ("cond", CanonicalValueType::Boolean),
        ]);
        let temp = VariableId::from(0);
        let condition = VariableId::from(1);
        let mut builder = ScalarGraphBuilder::new(Some(&hir), &mir);
        let condition_value =
            builder.push_value(OptValueType::Boolean, OptValueKind::BooleanConstant(true));
        builder
            .variable_values
            .insert(condition, Some(condition_value));
        let history = LocalAssignmentHistory {
            base_value: None,
            entries: vec![
                test_assignment_history_entry(first_update_expr),
                test_assignment_history_entry(second_update_expr),
            ],
        };
        builder
            .local_assignment_history
            .insert(temp, history.clone());

        let snapshots = builder
            .local_assignment_history_snapshots(BTreeSet::from([temp]))
            .expect("base-free local history should be snapshotted");
        assert_eq!(snapshots, vec![(temp, history.clone())]);

        let value = builder
            .lower_local_assignment_history_snapshot(temp, &history)
            .expect("complementary local history window should replay without a base value");
        assert!(matches!(
            builder.values[usize::from(value)].kind,
            OptValueKind::RealConstant(value) if value == 1.0
        ));
    }

    #[test]
    fn local_assignment_snapshots_include_bounded_local_dependencies() {
        let value_expr = ExprId::from(0);
        let mir = test_mir(vec![HirExpression {
            id: value_expr,
            kind: HirExprKind::Number {
                value: 4.0,
                raw: "4.0".into(),
            },
            span: test_span(),
        }]);
        let hir = test_hir_variables(vec![
            ("T2", CanonicalValueType::Real),
            ("TMF0", CanonicalValueType::Real),
        ]);
        let target = VariableId::from(0);
        let dependency = VariableId::from(1);
        let mut builder = ScalarGraphBuilder::new(Some(&hir), &mir);
        let dependency_base =
            builder.push_value(OptValueType::Real, OptValueKind::RealConstant(1.0));
        let dependency_history = LocalAssignmentHistory {
            base_value: Some(dependency_base),
            entries: vec![test_assignment_history_entry(value_expr)],
        };
        let mut target_entry = test_assignment_history_entry(value_expr);
        target_entry.local_dependencies.push(dependency);
        let target_history = LocalAssignmentHistory {
            base_value: None,
            entries: vec![target_entry],
        };
        builder
            .local_assignment_history
            .insert(target, target_history.clone());
        builder
            .local_assignment_history
            .insert(dependency, dependency_history.clone());

        let snapshots = builder
            .local_assignment_history_snapshots(BTreeSet::from([target]))
            .expect("bounded local dependency should be snapshotted");

        assert_eq!(snapshots.len(), 2);
        assert!(snapshots.contains(&(target, target_history)));
        assert!(snapshots.contains(&(dependency, dependency_history)));
    }

    #[test]
    fn local_assignment_snapshots_include_small_nested_local_dependencies() {
        let value_expr = ExprId::from(0);
        let mir = test_mir(vec![HirExpression {
            id: value_expr,
            kind: HirExprKind::Number {
                value: 4.0,
                raw: "4.0".into(),
            },
            span: test_span(),
        }]);
        let hir = test_hir_variables(vec![
            ("T2", CanonicalValueType::Real),
            ("TMF0", CanonicalValueType::Real),
            ("TMF1", CanonicalValueType::Real),
        ]);
        let target = VariableId::from(0);
        let dependency = VariableId::from(1);
        let nested_dependency = VariableId::from(2);
        let mut builder = ScalarGraphBuilder::new(Some(&hir), &mir);
        let nested_dependency_history = LocalAssignmentHistory {
            base_value: None,
            entries: vec![test_assignment_history_entry(value_expr)],
        };
        let mut dependency_entry = test_assignment_history_entry(value_expr);
        dependency_entry.local_dependencies.push(nested_dependency);
        let dependency_history = LocalAssignmentHistory {
            base_value: None,
            entries: vec![dependency_entry],
        };
        let mut target_entry = test_assignment_history_entry(value_expr);
        target_entry.local_dependencies.push(dependency);
        let target_history = LocalAssignmentHistory {
            base_value: None,
            entries: vec![target_entry],
        };
        builder
            .local_assignment_history
            .insert(target, target_history.clone());
        builder
            .local_assignment_history
            .insert(dependency, dependency_history.clone());
        builder
            .local_assignment_history
            .insert(nested_dependency, nested_dependency_history.clone());

        let snapshots = builder
            .local_assignment_history_snapshots(BTreeSet::from([target]))
            .expect("small nested local dependency should be snapshotted");

        assert_eq!(snapshots.len(), 3);
        assert!(snapshots.contains(&(target, target_history)));
        assert!(snapshots.contains(&(dependency, dependency_history)));
        assert!(snapshots.contains(&(nested_dependency, nested_dependency_history)));
    }

    #[test]
    fn local_assignment_snapshots_skip_oversized_optional_nested_dependencies() {
        let value_expr = ExprId::from(0);
        let mir = test_mir(vec![HirExpression {
            id: value_expr,
            kind: HirExprKind::Number {
                value: 4.0,
                raw: "4.0".into(),
            },
            span: test_span(),
        }]);
        let hir = test_hir_variables(vec![
            ("T2", CanonicalValueType::Real),
            ("TMF0", CanonicalValueType::Real),
            ("TMF1", CanonicalValueType::Real),
        ]);
        let target = VariableId::from(0);
        let dependency = VariableId::from(1);
        let nested_dependency = VariableId::from(2);
        let mut builder = ScalarGraphBuilder::new(Some(&hir), &mir);
        let nested_dependency_history = LocalAssignmentHistory {
            base_value: None,
            entries: vec![
                test_assignment_history_entry(value_expr);
                MAX_SCALAR_LOCAL_ASSIGNMENT_SNAPSHOT_TOTAL_ENTRIES
            ],
        };
        let mut dependency_entry = test_assignment_history_entry(value_expr);
        dependency_entry.local_dependencies.push(nested_dependency);
        let dependency_history = LocalAssignmentHistory {
            base_value: None,
            entries: vec![dependency_entry],
        };
        let mut target_entry = test_assignment_history_entry(value_expr);
        target_entry.local_dependencies.push(dependency);
        let target_history = LocalAssignmentHistory {
            base_value: None,
            entries: vec![target_entry],
        };
        builder
            .local_assignment_history
            .insert(target, target_history.clone());
        builder
            .local_assignment_history
            .insert(dependency, dependency_history.clone());
        builder
            .local_assignment_history
            .insert(nested_dependency, nested_dependency_history);

        let snapshots = builder
            .local_assignment_history_snapshots(BTreeSet::from([target]))
            .expect("oversized optional nested dependency should not reject the snapshot");

        assert_eq!(snapshots.len(), 2);
        assert!(snapshots.contains(&(target, target_history)));
        assert!(snapshots.contains(&(dependency, dependency_history)));
        assert!(
            !snapshots
                .iter()
                .any(|(variable, _)| *variable == nested_dependency)
        );
    }

    #[test]
    fn recent_assignment_history_prefix_caches_stable_failure() {
        let expr = ExprId::from(0);
        let mir = test_mir(vec![HirExpression {
            id: expr,
            kind: HirExprKind::StringLiteral {
                value: "unsupported".into(),
            },
            span: test_span(),
        }]);
        let variable = VariableId::from(0);
        let mut builder = ScalarGraphBuilder::new(None, &mir);
        builder
            .variable_assignment_history
            .insert(variable, vec![test_assignment_history_entry(expr)]);

        assert_eq!(
            builder.lower_recent_assignment_history_prefix(variable, 1),
            None
        );
        let cache_key = builder.assignment_history_prefix_cache_key(variable, 1);
        assert_eq!(
            builder.assignment_history_prefix_cache.get(&cache_key),
            Some(&None)
        );
    }

    #[test]
    fn recent_assignment_history_prefix_does_not_cache_stack_cycle_failure() {
        let expr = ExprId::from(0);
        let mir = test_mir(vec![HirExpression {
            id: expr,
            kind: HirExprKind::Number {
                value: 1.0,
                raw: "1.0".into(),
            },
            span: test_span(),
        }]);
        let variable = VariableId::from(0);
        let mut builder = ScalarGraphBuilder::new(None, &mir);
        builder
            .variable_assignment_history
            .insert(variable, vec![test_assignment_history_entry(expr)]);
        builder.expression_lowering_stack.insert(expr);

        assert_eq!(
            builder.lower_recent_assignment_history_prefix(variable, 1),
            None
        );
        let cache_key = builder.assignment_history_prefix_cache_key(variable, 1);
        assert!(
            !builder
                .assignment_history_prefix_cache
                .contains_key(&cache_key)
        );
        assert!(builder.expression_lowering_stack_cycle_rejected);
    }

    #[test]
    fn full_assignment_history_replay_stops_after_budget_exhaustion() {
        let expr = ExprId::from(0);
        let mir = test_mir(vec![HirExpression {
            id: expr,
            kind: HirExprKind::Number {
                value: 1.0,
                raw: "1.0".into(),
            },
            span: test_span(),
        }]);
        let variable = VariableId::from(0);
        let mut builder = ScalarGraphBuilder::new(None, &mir);
        builder
            .variable_assignment_history
            .insert(variable, vec![test_assignment_history_entry(expr)]);
        builder.history_reconstruction_steps = builder.history_reconstruction_step_limit;
        builder.history_reconstruction_budget_exhausted = true;

        assert_eq!(builder.lower_assignment_history_value(variable), None);
        assert_eq!(
            builder.history_reconstruction_steps,
            builder.history_reconstruction_step_limit
        );

        let cached = builder.push_value(OptValueType::Real, OptValueKind::RealConstant(1.0));
        let cache_key = builder.assignment_history_prefix_cache_key(variable, 1);
        builder
            .assignment_history_prefix_cache
            .insert(cache_key, Some(cached));

        assert_eq!(
            builder.lower_assignment_history_value(variable),
            Some(cached)
        );
    }

    #[test]
    fn complementary_assignment_history_replays_after_budget_exhaustion() {
        let cond_expr = ExprId::from(0);
        let target_expr = ExprId::from(1);
        let first_value_expr = ExprId::from(2);
        let first_update_expr = ExprId::from(3);
        let inverted_cond_expr = ExprId::from(4);
        let second_value_expr = ExprId::from(5);
        let second_update_expr = ExprId::from(6);
        let mir = test_mir(vec![
            HirExpression {
                id: cond_expr,
                kind: HirExprKind::Identifier {
                    name: "cond".into(),
                },
                span: test_span(),
            },
            HirExpression {
                id: target_expr,
                kind: HirExprKind::Identifier { name: "x".into() },
                span: test_span(),
            },
            HirExpression {
                id: first_value_expr,
                kind: HirExprKind::Number {
                    value: 1.0,
                    raw: "1.0".into(),
                },
                span: test_span(),
            },
            HirExpression {
                id: first_update_expr,
                kind: HirExprKind::Conditional {
                    condition: cond_expr,
                    then_expr: first_value_expr,
                    else_expr: target_expr,
                },
                span: test_span(),
            },
            HirExpression {
                id: inverted_cond_expr,
                kind: HirExprKind::Unary {
                    op: "Not".into(),
                    operand: cond_expr,
                },
                span: test_span(),
            },
            HirExpression {
                id: second_value_expr,
                kind: HirExprKind::Number {
                    value: 2.0,
                    raw: "2.0".into(),
                },
                span: test_span(),
            },
            HirExpression {
                id: second_update_expr,
                kind: HirExprKind::Conditional {
                    condition: inverted_cond_expr,
                    then_expr: second_value_expr,
                    else_expr: target_expr,
                },
                span: test_span(),
            },
        ]);
        let hir = test_hir_variables(vec![
            ("x", CanonicalValueType::Real),
            ("cond", CanonicalValueType::Boolean),
        ]);
        let target = VariableId::from(0);
        let condition = VariableId::from(1);
        let mut builder = ScalarGraphBuilder::new(Some(&hir), &mir);
        let condition_value =
            builder.push_value(OptValueType::Boolean, OptValueKind::BooleanConstant(true));
        builder
            .variable_values
            .insert(condition, Some(condition_value));
        builder.variable_values.insert(target, None);
        builder.variable_assignment_history.insert(
            target,
            vec![
                test_assignment_history_entry(first_update_expr),
                test_assignment_history_entry(second_update_expr),
            ],
        );
        builder.history_reconstruction_steps = builder.history_reconstruction_step_limit;
        builder.history_reconstruction_budget_exhausted = true;

        let value = builder
            .lower_complementary_assignment_history_value(target)
            .expect("complementary pair should replay from current snapshots");

        assert_eq!(
            builder.history_reconstruction_steps,
            builder.history_reconstruction_step_limit
        );
        assert!(matches!(
            &builder.values[usize::from(value)].kind,
            OptValueKind::RealConstant(value) if *value == 1.0
        ));
    }

    #[test]
    fn generated_guard_alias_assignment_retains_replay_expression() {
        let left = ExprId::from(0);
        let right = ExprId::from(1);
        let guard = ExprId::from(2);
        let mir = test_mir(vec![
            HirExpression {
                id: left,
                kind: HirExprKind::Number {
                    value: 1.0,
                    raw: "1.0".into(),
                },
                span: test_span(),
            },
            HirExpression {
                id: right,
                kind: HirExprKind::Number {
                    value: 0.0,
                    raw: "0.0".into(),
                },
                span: test_span(),
            },
            HirExpression {
                id: guard,
                kind: HirExprKind::Binary {
                    op: "Gt".into(),
                    left,
                    right,
                },
                span: test_span(),
            },
        ]);
        let variable = VariableId::from(0);
        let assignment = test_assignment(variable, "__guard0", guard);
        let mut builder = ScalarGraphBuilder::new(None, &mir);
        builder.track_assignment_history = false;

        builder.lower_assignment_statement(&assignment);

        assert_eq!(
            builder.variable_assignment_exprs.get(&variable),
            Some(&guard)
        );
        assert!(!builder.variable_assignment_history.contains_key(&variable));
    }
}
